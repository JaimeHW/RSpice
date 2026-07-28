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
            let slot = match operator { 73861 => 0usize, 73865 => 1usize, 73869 => 2usize, 73942 => 3usize, 73946 => 4usize, 74007 => 5usize, 74027 => 6usize, 74033 => 7usize, 74064 => 8usize, 74070 => 9usize, 74091 => 10usize, 74114 => 11usize, 74134 => 12usize, 74140 => 13usize, 74146 => 14usize, _ => usize::MAX };
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
            let v1 = 1e0f64;
            let v2 = 1.0f64;
            let v3 = 0e0f64;
            let v4 = parameters[43];
            let v6 = 0e0f64;
            let v7 = 0.0f64;
            let v10 = 0e0f64;
            let v11 = 1e-12f64;
            let v12 = parameters[237];
            let v13 = 5e-1f64;
            let v14 = parameters[51];
            let v15 = 1e1f64;
            let v18 = 2e2f64;
            let v19 = parameters[52];
            let v20 = 1e-2f64;
            let v22 = parameters[73];
            let v23 = 1e-6f64;
            let v25 = parameters[104];
            let v27 = parameters[201];
            let v29 = 1e-4f64;
            let v30 = parameters[240];
            let v32 = parameters[241];
            let v34 = parameters[242];
            let v36 = parameters[243];
            let v38 = parameters[59];
            let v40 = parameters[284];
            let v42 = parameters[148];
            let v44 = parameters[198];
            let v46 = parameters[70];
            let v48 = parameters[83];
            let v50 = parameters[84];
            let v52 = parameters[85];
            let v54 = parameters[80];
            let v56 = parameters[81];
            let v58 = parameters[82];
            let v60 = parameters[250];
            let v61 = 1e6f64;
            let v63 = parameters[232];
            let v64 = 2.7315e2f64;
            let v66 = parameters[58];
            let v67 = parameters[15];
            let v68 = 1e2f64;
            let v70 = parameters[46];
            let v71 = parameters[34];
            let v72 = if parameter_given[190] { 1.0 } else { 0.0 };
            let v73 = parameters[190];
            let v74 = 5e9f64;
            let v78 = 2e0f64;
            let v79 = 1e-1f64;
            let v80 = 2.1e0f64;
            let v82 = 1.0f64;
            let v84 = 2.1e0f64;
            let v88 = 1.0000000000000005e-4f64;
            let v90 = 4e0f64;
            let v91 = 8e0f64;
            let v92 = 1.0f64;
            let v93 = 0.0f64;
            let v94 = 1.0f64;
            let v95 = 0.0f64;
            let v96 = 3e0f64;
            let v97 = 0.0f64;
            let v107 = 2.5e-1f64;
            let v113 = 2.1e0f64;
            let v115 = parameters[55];
            let v116 = 9.025e-5f64;
            let v117 = 1e-7f64;
            let v122 = parameters[236];
            let v123 = 1.034943e-10f64;
            let v126 = 3.453133e-11f64;
            let v129 = parameters[239];
            let v133 = parameters[0];
            let v134 = parameters[56];
            let v137 = parameters[57];
            let v140 = parameters[40];
            let v144 = parameters[1];
            let v145 = parameters[9];
            let v147 = parameters[60];
            let v149 = parameters[295];
            let v151 = parameters[61];
            let v157 = parameters[18];
            let v171 = parameters[107];
            let v172 = parameters[108];
            let v173 = parameters[111];
            let v178 = parameters[109];
            let v179 = parameters[110];
            let v187 = parameters[72];
            let v191 = parameters[74];
            let v192 = parameters[75];
            let v197 = parameters[62];
            let v201 = parameters[63];
            let v206 = 1.6021918e-19f64;
            let v207 = 1.3806226e-23f64;
            let v212 = parameters[244];
            let v213 = parameters[247];
            let v217 = parameters[251];
            let v218 = parameters[252];
            let v222 = parameters[248];
            let v224 = parameters[249];
            let v228 = 3.2043836e-19f64;
            let v236 = parameters[91];
            let v238 = parameters[89];
            let v240 = parameters[68];
            let v241 = parameters[76];
            let v242 = parameters[77];
            let v246 = parameters[78];
            let v247 = parameters[79];
            let v250 = parameters[149];
            let v251 = parameters[150];
            let v253 = parameters[151];
            let v258 = parameters[152];
            let v259 = parameters[153];
            let v263 = parameters[192];
            let v265 = parameters[193];
            let v268 = parameters[67];
            let v269 = parameters[7];
            let v270 = parameters[6];
            let v275 = parameters[8];
            let v280 = parameters[44];
            let v282 = parameters[130];
            let v283 = parameters[131];
            let v287 = parameters[124];
            let v288 = parameters[125];
            let v289 = parameters[126];
            let v294 = parameters[123];
            let v297 = parameters[117];
            let v298 = parameters[119];
            let v299 = parameters[120];
            let v304 = parameters[118];
            let v305 = parameters[121];
            let v310 = parameters[127];
            let v311 = parameters[128];
            let v312 = parameters[129];
            let v324 = parameters[132];
            let v325 = parameters[133];
            let v338 = parameters[65];
            let v340 = parameters[66];
            let v343 = parameters[134];
            let v344 = parameters[135];
            let v345 = parameters[136];
            let v354 = parameters[115];
            let v356 = parameters[114];
            let v360 = parameters[116];
            let v362 = 1e-50f64;
            let v365 = parameters[50];
            let v366 = parameters[253];
            let v368 = if parameter_given[168] { 1.0 } else { 0.0 };
            let v369 = if parameter_given[169] { 1.0 } else { 0.0 };
            let v370 = if parameter_given[170] { 1.0 } else { 0.0 };
            let v371 = if parameter_given[294] { 1.0 } else { 0.0 };
            let v372 = if parameter_given[293] { 1.0 } else { 0.0 };
            let v373 = if parameter_given[13] { 1.0 } else { 0.0 };
            let v374 = if parameter_given[14] { 1.0 } else { 0.0 };
            let v375 = if parameter_given[23] { 1.0 } else { 0.0 };
            let v376 = if parameter_given[22] { 1.0 } else { 0.0 };
            let v377 = if parameter_given[16] { 1.0 } else { 0.0 };
            let v378 = parameters[17];
            let v382 = parameters[13];
            let v383 = parameters[14];
            let v384 = parameters[16];
            let v388 = parameters[10];
            let v390 = parameters[11];
            let v395 = parameters[12];
            let v418 = parameters[162];
            let v421 = parameters[161];
            let v423 = parameters[163];
            let v433 = parameters[199];
            let v434 = parameters[200];
            let v438 = parameters[202];
            let v439 = parameters[203];
            let v459 = parameters[165];
            let v462 = parameters[164];
            let v464 = parameters[166];
            let v504 = 5.1702525384001115e-2f64;
            let v505 = 1.04e16f64;
            let v509 = 5.1702525384001115e-2f64;
            let v510 = 1.04e16f64;
            let v514 = 1.2919089961638799e9f64;
            let v517 = parameters[194];
            let v518 = parameters[195];
            let v522 = parameters[196];
            let v523 = parameters[197];
            let v529 = 1e-3f64;
            let v530 = 4e-6f64;
            let v535 = 1e-10f64;
            let v536 = 1e-13f64;
            let v539 = parameters[35];
            let v543 = 1e3f64;
            let v544 = 1e3f64;
            let v545 = parameters[261];
            let v547 = parameters[289];
            let v549 = parameters[288];
            let v552 = parameters[262];
            let v554 = parameters[290];
            let v556 = 1e4f64;
            let v557 = 1e4f64;
            let v560 = parameters[291];
            let v562 = 1e4f64;
            let v566 = parameters[24];
            let v567 = parameters[23];
            let v568 = parameters[20];
            let v570 = parameters[19];
            let v573 = parameters[22];
            let v574 = parameters[21];
            let v581 = parameters[294];
            let v586 = parameters[293];
            let v602 = node_potentials[6];
            let v603 = node_potentials[7];
            let v606 = node_potentials[11];
            let v609 = node_potentials[12];
            let v612 = node_potentials[0];
            let v613 = node_potentials[2];
            let v616 = 1e-9f64;
            let v617 = 1e-5f64;
            let v618 = node_potentials[18];
            let v620 = 1e-5f64;
            let v621 = node_potentials[13];
            let v623 = 1e-5f64;
            let v624 = node_potentials[15];
            let v626 = 1e-5f64;
            let v627 = node_potentials[16];
            let v629 = 1e-5f64;
            let v631 = parameters[38];
            let v635 = node_potentials[10];
            let v640 = -1e0f64;
            let v644 = 5e0f64;
            let v646 = 6e0f64;
            let v648 = temperature;
            let v656 = parameters[53];
            let v659 = parameters[54];
            let v666 = parameters[254];
            let v667 = parameters[98];
            let v668 = parameters[99];
            let v673 = parameters[100];
            let v674 = parameters[101];
            let v679 = parameters[102];
            let v680 = parameters[103];
            let v685 = parameters[159];
            let v688 = parameters[158];
            let v691 = parameters[160];
            let v700 = parameters[112];
            let v707 = 1.8e0f64;
            let v708 = 4e-1f64;
            let v720 = 1.04e16f64;
            let v721 = 1.5e0f64;
            let v748 = 1.414213562373095e0f64;
            let v763 = 1.2919089961638799e9f64;
            let v765 = 1.2919089961638799e9f64;
            let v776 = 8e-1f64;
            let v777 = 1.2e0f64;
            let v796 = 1.0f64;
            let v797 = 0.0f64;
            let v798 = 0.0f64;
            let v799 = 1.0f64;
            let v800 = 0.0f64;
            let v810 = 1.25e-1f64;
            let v821 = 2e1f64;
            let v827 = -2e1f64;
            let v829 = -2e1f64;
            let v832 = -2e1f64;
            let v834 = -2e1f64;
            let v840 = parameters[226];
            let v842 = 5e-1f64;
            let v843 = 1.6666666666666666e-1f64;
            let v844 = 4.1666666666666664e-2f64;
            let v845 = 8.333333333333333e-3f64;
            let v846 = 1.388888888888889e-3f64;
            let v847 = 1.984126984126984e-4f64;
            let v861 = 5e-12f64;
            let v883 = 4e-6f64;
            let v888 = 1e-13f64;
            let v899 = 5e-2f64;
            let v901 = 2.0000000000000004e-2f64;
            let v902 = 1.0f64;
            let v903 = -2.0000000000000004e-2f64;
            let v922 = parameters[204];
            let v924 = parameters[206];
            let v927 = parameters[205];
            let v944 = 4e-8f64;
            let v949 = 1.0000000000000002e-14f64;
            let v976 = 1e12f64;
            let v991 = 2e-3f64;
            let v992 = 1.0f64;
            let v993 = -2e-3f64;
            let v1004 = 2.069886e-10f64;
            let v1035 = 2.069886e-10f64;
            let v1052 = 9.5e-1f64;
            let v1057 = 3.8e0f64;
            let v1068 = 3.2043836e-19f64;
            let v1087 = parameters[69];
            let v1102 = parameters[71];
            let v1114 = parameters[86];
            let v1117 = parameters[88];
            let v1120 = parameters[87];
            let v1134 = parameters[105];
            let v1147 = parameters[90];
            let v1149 = -3e0f64;
            let v1152 = 3.333333333333333e-1f64;
            let v1153 = 2.7e1f64;
            let v1154 = 3.7037037037037035e-2f64;
            let v1161 = 3.333333333333333e-1f64;
            let v1162 = 4.02052934513951e-2f64;
            let v1163 = 1.48148111111111e-1f64;
            let v1176 = 4.000000000000001e-2f64;
            let v1181 = 1.0000000000000001e-11f64;
            let v1188 = 2e-1f64;
            let v1189 = 1.0f64;
            let v1190 = -2e-1f64;
            let v1208 = 7e0f64;
            let v1223 = -1.6021918e-19f64;
            let v1226 = -1.6021918e-19f64;
            let v1231 = 1e-5f64;
            let v1233 = parameters[39];
            let v1254 = 2.220446049250313e-15f64;
            let v1256 = 2.220446049250313e-15f64;
            let v1270 = 8e-4f64;
            let v1305 = -1e-9f64;
            let v1373 = -1e0f64;
            let v1386 = 1.2919089961638799e9f64;
            let v1390 = 9.9e-1f64;
            let v1410 = 5e-1f64;
            let v1411 = 1.6666666666666666e-1f64;
            let v1412 = 4.1666666666666664e-2f64;
            let v1413 = 8.333333333333333e-3f64;
            let v1414 = 1.388888888888889e-3f64;
            let v1415 = 1.984126984126984e-4f64;
            let v1448 = 1.0f64;
            let v1449 = 0.0f64;
            let v1450 = 1.0f64;
            let v1451 = 0.0f64;
            let v1452 = 0.0f64;
            let v1462 = 2.5e-1f64;
            let v1481 = 1.0f64;
            let v1482 = 0.0f64;
            let v1483 = 1.0f64;
            let v1484 = 0.0f64;
            let v1485 = 0.0f64;
            let v1495 = 2.5e-1f64;
            let v1513 = 0.0f64;
            let v1522 = 2.220446049250313e-15f64;
            let v1524 = 2.220446049250313e-15f64;
            let v1536 = 1.3094570021973102e-2f64;
            let v1540 = 8.1e1f64;
            let v1543 = -2.916e3f64;
            let v1549 = 1.458e3f64;
            let v1550 = 5.4e1f64;
            let v1562 = 3.333333333333333e-1f64;
            let v1564 = 1.259921049894873e0f64;
            let v1569 = 2.6456684199469993e-1f64;
            let v1615 = 1.2919089961638799e9f64;
            let v1661 = 9.8e-1f64;
            let v1665 = 1.0f64;
            let v1671 = 2.560000000000001e-2f64;
            let v1673 = 1.0f64;
            let v1674 = 0.0f64;
            let v1675 = 1.0f64;
            let v1676 = 0.0f64;
            let v1677 = 0.0f64;
            let v1687 = 2.5e-1f64;
            let v1705 = -1.6e0f64;
            let v1707 = 6e-1f64;
            let v1743 = 2.220446049250313e-15f64;
            let v1745 = 2.220446049250313e-15f64;
            let v1792 = -1e-9f64;
            let v1865 = -1e0f64;
            let v1886 = parameters[25];
            let v1889 = 2e-1f64;
            let v1896 = parameters[137];
            let v1897 = 3.2043836e-19f64;
            let v1952 = 3.0000000000000002e-2f64;
            let v1969 = 2.220446049250313e-15f64;
            let v1971 = 2.220446049250313e-15f64;
            let v1981 = 1.3e0f64;
            let v1985 = 3e-2f64;
            let v2000 = parameters[36];
            let v2002 = 4.12e0f64;
            let v2003 = parameters[142];
            let v2008 = parameters[145];
            let v2013 = parameters[144];
            let v2018 = 9.9e1f64;
            let v2031 = 4e-6f64;
            let v2036 = 1e-13f64;
            let v2039 = parameters[143];
            let v2047 = -3.4e1f64;
            let v2050 = 2.5e-1f64;
            let v2054 = 7.38905609893065e0f64;
            let v2086 = 4e-6f64;
            let v2091 = 1e-13f64;
            let v2098 = 0e0f64;
            let v2103 = parameters[122];
            let v2108 = 0e0f64;
            let v2113 = 4e-4f64;
            let v2118 = 1e-12f64;
            let v2122 = 0e0f64;
            let v2149 = 1.0f64;
            let v2150 = 0.0f64;
            let v2151 = 0.0f64;
            let v2152 = 1.0f64;
            let v2153 = 0.0f64;
            let v2163 = 1.25e-1f64;
            let v2184 = 4e-6f64;
            let v2189 = 1e-13f64;
            let v2204 = parameters[26];
            let v2208 = parameters[141];
            let v2212 = 4.1046315303568966e26f64;
            let v2213 = 2.4665765749313358e0f64;
            let v2216 = 2.1633307652783932e-2f64;
            let v2223 = parameters[140];
            let v2228 = 3.3163543761348e-29f64;
            let v2247 = parameters[37];
            let v2248 = parameters[138];
            let v2249 = parameters[139];
            let v2253 = 1e-5f64;
            let v2254 = node_potentials[17];
            let v2268 = -1e-9f64;
            let v2326 = 5e2f64;
            let v2328 = 1.403592217853e217f64;
            let v2330 = 6e1f64;
            let v2333 = 1.14200738981568e26f64;
            let v2342 = -1e-9f64;
            let v2382 = 1.0f64;
            let v2383 = 0.0f64;
            let v2384 = 1.0f64;
            let v2385 = 0.0f64;
            let v2386 = 0.0f64;
            let v2396 = 2.5e-1f64;
            let v2426 = 1.0f64;
            let v2427 = 0.0f64;
            let v2428 = 1.0f64;
            let v2429 = 0.0f64;
            let v2430 = 0.0f64;
            let v2440 = 2.5e-1f64;
            let v2480 = -1e0f64;
            let v2485 = -1e0f64;
            let v2535 = 8e1f64;
            let v2537 = 1.25e2f64;
            let v2538 = 4e1f64;
            let v2541 = 2.5e1f64;
            let v2591 = -5e-1f64;
            let v2597 = 5e-1f64;
            let v2625 = 1.0f64;
            let v2626 = 0.0f64;
            let v2627 = 0.0f64;
            let v2628 = 1.0f64;
            let v2629 = 0.0f64;
            let v2639 = 1.25e-1f64;
            let v2652 = 4e-4f64;
            let v2657 = 1e-12f64;
            let v2673 = 0.0f64;
            let v2682 = 1.3e0f64;
            let v2686 = 1.3e0f64;
            let v2696 = 1.3e0f64;
            let v2709 = 2.220446049250313e-15f64;
            let v2711 = 2.220446049250313e-15f64;
            let v2743 = 2.220446049250313e-15f64;
            let v2745 = 2.220446049250313e-15f64;
            let v2770 = 1.2919089961638799e9f64;
            let v2774 = 1.2919089961638799e9f64;
            let v2801 = -1e-9f64;
            let v2869 = -1e0f64;
            let v2909 = -1e-9f64;
            let v2982 = -1e0f64;
            let v3025 = -1e-9f64;
            let v3099 = -1e-9f64;
            let v3139 = 1.0f64;
            let v3140 = 0.0f64;
            let v3141 = 1.0f64;
            let v3142 = 0.0f64;
            let v3143 = 0.0f64;
            let v3153 = 2.5e-1f64;
            let v3183 = 1.0f64;
            let v3184 = 0.0f64;
            let v3185 = 1.0f64;
            let v3186 = 0.0f64;
            let v3187 = 0.0f64;
            let v3197 = 2.5e-1f64;
            let v3239 = -1e0f64;
            let v3244 = -1e0f64;
            let v3345 = -5e-1f64;
            let v3366 = 1.0f64;
            let v3367 = 0.0f64;
            let v3368 = 1.0f64;
            let v3369 = 0.0f64;
            let v3370 = 0.0f64;
            let v3390 = 1.0f64;
            let v3391 = 0.0f64;
            let v3392 = 1.0f64;
            let v3393 = 0.0f64;
            let v3394 = 0.0f64;
            let v3404 = 2.5e-1f64;
            let v3422 = 1e-5f64;
            let v3424 = 1.0f64;
            let v3426 = 1e-5f64;
            let v3430 = 1.0000000000000004e-20f64;
            let v3432 = 1.0f64;
            let v3433 = 0.0f64;
            let v3434 = 1.0f64;
            let v3435 = 0.0f64;
            let v3436 = 0.0f64;
            let v3446 = 2.5e-1f64;
            let v3452 = 1e-5f64;
            let v3458 = 2.220446049250313e-15f64;
            let v3460 = 2.220446049250313e-15f64;
            let v3462 = -5e-1f64;
            let v3482 = -1e0f64;
            let v3493 = 4.242640687119285e0f64;
            let v3500 = 9e0f64;
            let v3503 = 9.899494936611664e0f64;
            let v3506 = 1e-8f64;
            let v3509 = -9.899494936611664e0f64;
            let v3517 = -9.899494936611664e0f64;
            let v3522 = -5.65685424949238e0f64;
            let v3523 = 1.2e1f64;
            let v3542 = 0.0f64;
            let v3550 = 2.220446049250313e-15f64;
            let v3552 = 2.220446049250313e-15f64;
            let v3563 = 1.3094570021973102e-2f64;
            let v3569 = -2.916e3f64;
            let v3591 = 2.6456684199469993e-1f64;
            let v3618 = 2.5e-12f64;
            let v3630 = 1e-5f64;
            let v3652 = 2.01e2f64;
            let v3672 = 1e-16f64;
            let v3684 = 5e-3f64;
            let v3748 = -1e0f64;
            let v3751 = -1e0f64;
            let v3758 = 1.01e0f64;
            let v3807 = 2.01e2f64;
            let v3810 = 5e-2f64;
            let v3819 = -1e0f64;
            let v3838 = 2.220446049250313e-15f64;
            let v3840 = 2.220446049250313e-15f64;
            let v3852 = -1e0f64;
            let v3890 = 1.0f64;
            let v3891 = 0.0f64;
            let v3892 = 0.0f64;
            let v3893 = 1.0f64;
            let v3894 = 0.0f64;
            let v3904 = 1.25e-1f64;
            let v3917 = 4e-4f64;
            let v3922 = 1e-12f64;
            let v3940 = 0.0f64;
            let v3942 = 1.0f64;
            let v3947 = 1.3e0f64;
            let v3951 = 1.3e0f64;
            let v3961 = 1.3e0f64;
            let v3977 = 2.01e2f64;
            let v4067 = -1e0f64;
            let v4116 = 2.01e2f64;
            let v4119 = 5e-2f64;
            let v4128 = -1e0f64;
            let v4146 = 2.220446049250313e-15f64;
            let v4245 = 1e0f64;
            let v4247 = 1.0f64;
            let v4248 = 0.0f64;
            let v4249 = 0.0f64;
            let v4250 = 1.0f64;
            let v4251 = 0.0f64;
            let v4261 = 1.25e-1f64;
            let v4270 = 2.220446049250313e-15f64;
            let v4272 = 2.220446049250313e-15f64;
            let v4274 = 6.666666666666667e-1f64;
            let v4299 = -5e-1f64;
            let v4321 = 5.0000001e-1f64;
            let v4330 = 2.220446049250313e-15f64;
            let v4332 = parameters[191];
            let v4333 = 2.220446049250313e-15f64;
            let v4342 = 2.220446049250313e-15f64;
            let v4345 = 2.220446049250313e-15f64;
            let v4356 = parameters[189];
            let v4363 = 2.220446049250313e-15f64;
            let v4366 = 2.220446049250313e-15f64;
            let v4371 = 4e-6f64;
            let v4376 = 1e-13f64;
            let v4388 = 1e5f64;
            let v4389 = 1e9f64;
            let v4436 = 5e-1f64;
            let v4451 = parameters[227];
            let v4453 = 5e-1f64;
            let v4454 = 1.6666666666666666e-1f64;
            let v4455 = 4.1666666666666664e-2f64;
            let v4456 = 8.333333333333333e-3f64;
            let v4457 = 1.388888888888889e-3f64;
            let v4458 = 1.984126984126984e-4f64;
            let v4472 = 2.220446049250313e-15f64;
            let v4474 = 2.220446049250313e-15f64;
            let v4477 = 1.034943e-12f64;
            let v4480 = parameters[92];
            let v4482 = parameters[93];
            let v4484 = parameters[94];
            let v4493 = 3.6e7f64;
            let v4498 = 3e-7f64;
            let v4502 = parameters[97];
            let v4510 = parameters[95];
            let v4511 = parameters[96];
            let v4513 = 1e11f64;
            let v4519 = parameters[106];
            let v4528 = 4e-100f64;
            let v4533 = 1.0000000000000001e-60f64;
            let v4547 = 9.999999999999978e-1f64;
            let v4548 = parameters[113];
            let v4550 = 1.0000000000000022e0f64;
            let v4553 = 1.9999999999999978e0f64;
            let v4555 = 2.000000000000002e0f64;
            let v4564 = 9.999999999999978e-1f64;
            let v4566 = 1.0000000000000022e0f64;
            let v4570 = 1.9999999999999978e0f64;
            let v4572 = 2.000000000000002e0f64;
            let v4577 = -1e0f64;
            let v4589 = parameters[281];
            let v4596 = 5e-1f64;
            let v4597 = 1.6666666666666666e-1f64;
            let v4598 = 4.1666666666666664e-2f64;
            let v4599 = 8.333333333333333e-3f64;
            let v4600 = 1.388888888888889e-3f64;
            let v4601 = 1.984126984126984e-4f64;
            let v4615 = 1.1e0f64;
            let v4619 = 1.0000000000000002e-2f64;
            let v4624 = 5.0000000000000005e-12f64;
            let v4630 = parameters[245];
            let v4633 = parameters[246];
            let v4657 = parameters[33];
            let v4668 = parameters[154];
            let v4669 = parameters[155];
            let v4673 = parameters[156];
            let v4674 = parameters[157];
            let v4696 = -1e0f64;
            let v4717 = 4e-4f64;
            let v4722 = 1e-12f64;
            let v4744 = 2e-3f64;
            let v4747 = 8e-3f64;
            let v4762 = 4e-4f64;
            let v4767 = 1e-12f64;
            let v4771 = 2.220446049250313e-15f64;
            let v4775 = 4e-4f64;
            let v4780 = 1e-12f64;
            let v4784 = 2.220446049250313e-15f64;
            let v4793 = 4.000000000000001e-2f64;
            let v4798 = 1.0000000000000001e-11f64;
            let v4802 = 2.220446049250313e-15f64;
            let v4809 = 1e0f64;
            let v4811 = 1.0f64;
            let v4812 = 0.0f64;
            let v4813 = 0.0f64;
            let v4814 = 1.0f64;
            let v4815 = 0.0f64;
            let v4825 = 1.25e-1f64;
            let v4838 = parameters[30];
            let v4840 = parameters[32];
            let v4851 = 4e-6f64;
            let v4856 = 1e-13f64;
            let v4860 = 4e-6f64;
            let v4865 = 1e-13f64;
            let v4871 = 2.220446049250313e-15f64;
            let v4873 = 2.220446049250313e-15f64;
            let v4879 = parameters[285];
            let v4882 = parameters[286];
            let v4885 = parameters[283];
            let v4892 = 3.2043836e-19f64;
            let v4902 = -2.5e-1f64;
            let v4914 = 2.220446049250313e-15f64;
            let v4916 = 2.220446049250313e-15f64;
            let v4927 = 1.0f64;
            let v4931 = 1.3094570021973102e-2f64;
            let v4937 = -2.916e3f64;
            let v4959 = 2.6456684199469993e-1f64;
            let v4994 = parameters[287];
            let v5055 = 1.0f64;
            let v5061 = 2.560000000000001e-2f64;
            let v5063 = 1.0f64;
            let v5064 = 0.0f64;
            let v5065 = 1.0f64;
            let v5066 = 0.0f64;
            let v5067 = 0.0f64;
            let v5077 = 2.5e-1f64;
            let v5084 = 2.5e-12f64;
            let v5106 = 1.3e0f64;
            let v5110 = 1.3e0f64;
            let v5120 = 1.3e0f64;
            let v5129 = parameters[282];
            let v5142 = 4.242640687119285e0f64;
            let v5151 = 9.899494936611664e0f64;
            let v5156 = -9.899494936611664e0f64;
            let v5164 = -9.899494936611664e0f64;
            let v5169 = -5.65685424949238e0f64;
            let v5206 = 2.01e2f64;
            let v5337 = 2.01e2f64;
            let v5340 = 5e-2f64;
            let v5349 = -1e0f64;
            let v5370 = -1e0f64;
            let v5385 = 7.071067811865475e-1f64;
            let v5397 = 4e-12f64;
            let v5402 = 1e-16f64;
            let v5431 = 3.2043836e-19f64;
            let v5446 = 1.0f64;
            let v5447 = 1.0f64;
            let v5448 = 0.0f64;
            let v5449 = 0.0f64;
            let v5450 = 0.0f64;
            let v5460 = 5e-1f64;
            let v5468 = 2.220446049250313e-15f64;
            let v5479 = parameters[45];
            let v5491 = parameters[48];
            let v5500 = parameters[49];
            let v5509 = 4e-6f64;
            let v5514 = 1e-13f64;
            let v5531 = 4e-4f64;
            let v5536 = 1e-12f64;
            let v5569 = 1.0f64;
            let v5570 = 0.0f64;
            let v5571 = 0.0f64;
            let v5572 = 1.0f64;
            let v5573 = 0.0f64;
            let v5583 = 1.25e-1f64;
            let v5604 = 4e-6f64;
            let v5609 = 1e-13f64;
            let v5633 = 4.1046315303568966e26f64;
            let v5634 = 2.4665765749313358e0f64;
            let v5637 = 2.1633307652783932e-2f64;
            let v5665 = 3.3163543761348e-29f64;
            let v5690 = parameters[47];
            let v5710 = 1e-5f64;
            let v5717 = parameters[146];
            let v5725 = parameters[147];
            let v5735 = 4.000000000000001e-2f64;
            let v5740 = 1.0000000000000001e-11f64;
            let v5751 = 4.000000000000001e-2f64;
            let v5756 = 1.0000000000000001e-11f64;
            let v5793 = parameters[27];
            let v5796 = 2.220446049250313e-15f64;
            let v5799 = parameters[216];
            let v5804 = parameters[215];
            let v5809 = parameters[217];
            let v5815 = 4e-4f64;
            let v5820 = 1e-12f64;
            let v5824 = 4e-6f64;
            let v5829 = 1e-13f64;
            let v5842 = parameters[219];
            let v5845 = parameters[218];
            let v5850 = parameters[214];
            let v5854 = -3.4e1f64;
            let v5857 = parameters[213];
            let v5872 = parameters[221];
            let v5875 = parameters[222];
            let v5882 = parameters[220];
            let v5888 = -1e0f64;
            let v5901 = -1e0f64;
            let v5906 = parameters[225];
            let v5910 = 4e-4f64;
            let v5915 = 1e-12f64;
            let v5920 = parameters[224];
            let v5923 = -3.4e1f64;
            let v5926 = parameters[223];
            let v5932 = parameters[28];
            let v5934 = parameters[209];
            let v5935 = parameters[210];
            let v5939 = parameters[211];
            let v5945 = 4e-4f64;
            let v5950 = 1e-12f64;
            let v5956 = parameters[208];
            let v5960 = -3.4e1f64;
            let v5963 = parameters[207];
            let v5974 = parameters[212];
            let v5989 = 4e-4f64;
            let v5994 = 1e-12f64;
            let v6003 = -3.4e1f64;
            let v6031 = 1.0f64;
            let v6035 = parameters[292];
            let v6036 = 0.0f64;
            let v6044 = 1e0f64;
            let v6045 = 0e0f64;
            let v6075 = 2.220446049250313e-15f64;
            let v6110 = 4.242640687119285e0f64;
            let v6119 = 9.899494936611664e0f64;
            let v6127 = -9.899494936611664e0f64;
            let v6135 = -9.899494936611664e0f64;
            let v6140 = -5.65685424949238e0f64;
            let v6160 = 4.9787068367863944e-2f64;
            let v6169 = 2.220446049250313e-15f64;
            let v6171 = 2.220446049250313e-15f64;
            let v6187 = 2.220446049250313e-15f64;
            let v6189 = 2.220446049250313e-15f64;
            let v6198 = -1.047839336957922e-1f64;
            let v6199 = 7.071067811865476e-1f64;
            let v6205 = -5.151950988020902e1f64;
            let v6207 = 5.286687693921294e-4f64;
            let v6210 = 1.8773541122053122e-2f64;
            let v6213 = 2.8160311683079683e-2f64;
            let v6215 = 1.0979672760764175e-2f64;
            let v6217 = 7.930031540881942e-4f64;
            let v6231 = -3.7209791878387604e0f64;
            let v6276 = 6.0000000000000005e-2f64;
            let v6279 = 6.0000000000000005e-2f64;
            let v6296 = 2.220446049250313e-15f64;
            let v6298 = 2.220446049250313e-15f64;
            let v6304 = parameters[42];
            let v6308 = 4.1e1f64;
            let v6316 = 2.9693154855771e-1f64;
            let v6317 = -7.053654284009761e-2f64;
            let v6318 = 6.115288895133179e-3f64;
            let v6324 = 8.907946456731299e-1f64;
            let v6325 = -2.8214617136039044e-1f64;
            let v6338 = 7.07106781186548e-1f64;
            let v6339 = -1.17851130197758e-1f64;
            let v6340 = 1.78800506338833e-2f64;
            let v6341 = -1.63730162779191e-3f64;
            let v6342 = 6.36964918866352e-5f64;
            let v6352 = -2.35702260395516e-1f64;
            let v6353 = 5.3640151901649905e-2f64;
            let v6354 = -6.54920651116764e-3f64;
            let v6397 = -1e0f64;
            let v6403 = 4.1e1f64;
            let v6406 = 5e-2f64;
            let v6415 = -1e0f64;
            let v6436 = 2.220446049250313e-15f64;
            let v6455 = 1.0f64;
            let v6464 = 0.0f64;
            let v6471 = 0e0f64;
            let v6472 = 1e0f64;
            let v6483 = 2.220446049250313e-15f64;
            let v6510 = 4.242640687119285e0f64;
            let v6519 = 9.899494936611664e0f64;
            let v6527 = -9.899494936611664e0f64;
            let v6535 = -9.899494936611664e0f64;
            let v6540 = -5.65685424949238e0f64;
            let v6560 = 4.9787068367863944e-2f64;
            let v6569 = 2.220446049250313e-15f64;
            let v6571 = 2.220446049250313e-15f64;
            let v6587 = 2.220446049250313e-15f64;
            let v6589 = 2.220446049250313e-15f64;
            let v6598 = -1.047839336957922e-1f64;
            let v6599 = 7.071067811865476e-1f64;
            let v6605 = -5.151950988020902e1f64;
            let v6607 = 5.286687693921294e-4f64;
            let v6610 = 1.8773541122053122e-2f64;
            let v6613 = 2.8160311683079683e-2f64;
            let v6615 = 1.0979672760764175e-2f64;
            let v6617 = 7.930031540881942e-4f64;
            let v6631 = -3.7209791878387604e0f64;
            let v6676 = 6.0000000000000005e-2f64;
            let v6679 = 6.0000000000000005e-2f64;
            let v6696 = 2.220446049250313e-15f64;
            let v6698 = 2.220446049250313e-15f64;
            let v6707 = 4.1e1f64;
            let v6715 = -7.053654284009761e-2f64;
            let v6721 = 8.907946456731299e-1f64;
            let v6722 = -2.8214617136039044e-1f64;
            let v6735 = -1.17851130197758e-1f64;
            let v6736 = -1.63730162779191e-3f64;
            let v6746 = -2.35702260395516e-1f64;
            let v6747 = 5.3640151901649905e-2f64;
            let v6748 = -6.54920651116764e-3f64;
            let v6791 = -1e0f64;
            let v6797 = 4.1e1f64;
            let v6800 = 5e-2f64;
            let v6809 = -1e0f64;
            let v6832 = 2.220446049250313e-15f64;
            let v6855 = 1.0f64;
            let v6862 = 0.0f64;
            let v6875 = parameters[64];
            let v6877 = 2.220446049250313e-15f64;
            let v6880 = 2.220446049250313e-15f64;
            let v6883 = 1e-15f64;
            let v6890 = parameters[29];
            let v6892 = parameters[188];
            let v6895 = parameters[171];
            let v6896 = parameters[172];
            let v6922 = 1e0f64;
            let v6923 = 0e0f64;
            let v6946 = 2.220446049250313e-15f64;
            let v6996 = 4.242640687119285e0f64;
            let v7005 = 9.899494936611664e0f64;
            let v7013 = -9.899494936611664e0f64;
            let v7021 = -9.899494936611664e0f64;
            let v7026 = -5.65685424949238e0f64;
            let v7046 = 4.9787068367863944e-2f64;
            let v7055 = 2.220446049250313e-15f64;
            let v7057 = 2.220446049250313e-15f64;
            let v7073 = 2.220446049250313e-15f64;
            let v7075 = 2.220446049250313e-15f64;
            let v7084 = -1.047839336957922e-1f64;
            let v7085 = 7.071067811865476e-1f64;
            let v7091 = -5.151950988020902e1f64;
            let v7093 = 5.286687693921294e-4f64;
            let v7096 = 1.8773541122053122e-2f64;
            let v7099 = 2.8160311683079683e-2f64;
            let v7101 = 1.0979672760764175e-2f64;
            let v7103 = 7.930031540881942e-4f64;
            let v7117 = -3.7209791878387604e0f64;
            let v7123 = parameters[41];
            let v7164 = 6.0000000000000005e-2f64;
            let v7167 = 6.0000000000000005e-2f64;
            let v7185 = 2.220446049250313e-15f64;
            let v7187 = 2.220446049250313e-15f64;
            let v7200 = 4.1e1f64;
            let v7208 = -7.053654284009761e-2f64;
            let v7214 = 8.907946456731299e-1f64;
            let v7215 = -2.8214617136039044e-1f64;
            let v7228 = -1.17851130197758e-1f64;
            let v7229 = -1.63730162779191e-3f64;
            let v7239 = -2.35702260395516e-1f64;
            let v7240 = 5.3640151901649905e-2f64;
            let v7241 = -6.54920651116764e-3f64;
            let v7284 = -1e0f64;
            let v7290 = 4.1e1f64;
            let v7293 = 5e-2f64;
            let v7302 = -1e0f64;
            let v7323 = 2.220446049250313e-15f64;
            let v7356 = 0e0f64;
            let v7357 = 1e0f64;
            let v7380 = 2.220446049250313e-15f64;
            let v7424 = 4.242640687119285e0f64;
            let v7433 = 9.899494936611664e0f64;
            let v7441 = -9.899494936611664e0f64;
            let v7449 = -9.899494936611664e0f64;
            let v7454 = -5.65685424949238e0f64;
            let v7474 = 4.9787068367863944e-2f64;
            let v7483 = 2.220446049250313e-15f64;
            let v7485 = 2.220446049250313e-15f64;
            let v7501 = 2.220446049250313e-15f64;
            let v7503 = 2.220446049250313e-15f64;
            let v7512 = -1.047839336957922e-1f64;
            let v7513 = 7.071067811865476e-1f64;
            let v7519 = -5.151950988020902e1f64;
            let v7521 = 5.286687693921294e-4f64;
            let v7524 = 1.8773541122053122e-2f64;
            let v7527 = 2.8160311683079683e-2f64;
            let v7529 = 1.0979672760764175e-2f64;
            let v7531 = 7.930031540881942e-4f64;
            let v7545 = -3.7209791878387604e0f64;
            let v7591 = 6.0000000000000005e-2f64;
            let v7594 = 6.0000000000000005e-2f64;
            let v7612 = 2.220446049250313e-15f64;
            let v7614 = 2.220446049250313e-15f64;
            let v7627 = 4.1e1f64;
            let v7635 = -7.053654284009761e-2f64;
            let v7641 = 8.907946456731299e-1f64;
            let v7642 = -2.8214617136039044e-1f64;
            let v7655 = -1.17851130197758e-1f64;
            let v7656 = -1.63730162779191e-3f64;
            let v7666 = -2.35702260395516e-1f64;
            let v7667 = 5.3640151901649905e-2f64;
            let v7668 = -6.54920651116764e-3f64;
            let v7711 = -1e0f64;
            let v7717 = 4.1e1f64;
            let v7720 = 5e-2f64;
            let v7729 = -1e0f64;
            let v7752 = 2.220446049250313e-15f64;
            let v7788 = parameters[170];
            let v7790 = parameters[169];
            let v7881 = parameters[173];
            let v7885 = parameters[175];
            let v7889 = parameters[174];
            let v7893 = parameters[176];
            let v7911 = parameters[177];
            let v7937 = parameters[178];
            let v7963 = parameters[179];
            let v7964 = parameters[2];
            let v7966 = parameters[3];
            let v7968 = parameters[238];
            let v7971 = parameters[5];
            let v7973 = parameters[180];
            let v7976 = parameters[181];
            let v7981 = parameters[185];
            let v7984 = parameters[182];
            let v7998 = parameters[186];
            let v8001 = parameters[183];
            let v8017 = parameters[187];
            let v8020 = parameters[184];
            let v8093 = parameters[4];
            let v8208 = -1.6021918e-19f64;
            let v8233 = -1e0f64;
            let v8236 = -1.6021918e-19f64;
            let v8261 = -1e0f64;
            let v8263 = parameters[233];
            let v8264 = parameters[234];
            let v8277 = parameters[235];
            let v8279 = parameters[31];
            let v8284 = -2e0f64;
            let v8294 = 2.220446049250313e-15f64;
            let v8312 = 9.999999999999978e-1f64;
            let v8314 = 1.0000000000000022e0f64;
            let v8317 = 1.9999999999999978e0f64;
            let v8319 = 2.000000000000002e0f64;
            let v8328 = -1e0f64;
            let v8359 = 1.5e1f64;
            let v8382 = 4.2e1f64;
            let v8407 = 3.872983346207417e0f64;
            let v8428 = parameters[168];
            let v8435 = 2.1983327444149834e-11f64;
            let v8436 = parameters[167];
            let v8468 = 2.1983327444149834e-11f64;
            let v8523 = 2.069886e-10f64;
            let v8526 = 1.3e0f64;
            let v8717 = 1.898893985185185e-20f64;
            let v8723 = 2.220446049250313e-15f64;
            let v8725 = 2.220446049250313e-15f64;
            let v8754 = parameters[259];
            let v8756 = 1.0f64;
            let v8757 = parameters[264];
            let v8759 = parameters[266];
            let v8760 = parameters[268];
            let v8761 = parameters[273];
            let v8762 = parameters[263];
            let v8764 = parameters[255];
            let v8767 = parameters[258];
            let v8770 = parameters[265];
            let v8771 = parameters[267];
            let v8772 = parameters[272];
            let v8774 = parameters[256];
            let v8777 = parameters[257];
            let v8780 = parameters[271];
            let v8789 = parameters[269];
            let v8792 = parameters[270];
            let v8797 = parameters[274];
            let v8800 = parameters[279];
            let v8801 = parameters[280];
            let v8805 = parameters[277];
            let v8806 = parameters[278];
            let v8810 = parameters[275];
            let v8811 = parameters[276];
            let v8827 = 9.999999999999978e-1f64;
            let v8829 = 1.0000000000000022e0f64;
            let v8832 = 1.9999999999999978e0f64;
            let v8834 = 2.000000000000002e0f64;
            let v8844 = 9.999999999999978e-1f64;
            let v8846 = 1.0000000000000022e0f64;
            let v8850 = 1.9999999999999978e0f64;
            let v8852 = 2.000000000000002e0f64;
            let v8857 = -1e0f64;
            let v8880 = parameters[260];
            let v8882 = 0.0f64;
            let v8931 = 9.999999999999978e-1f64;
            let v8933 = 1.0000000000000022e0f64;
            let v8936 = 1.9999999999999978e0f64;
            let v8938 = 2.000000000000002e0f64;
            let v8948 = 9.999999999999978e-1f64;
            let v8950 = 1.0000000000000022e0f64;
            let v8954 = 1.9999999999999978e0f64;
            let v8956 = 2.000000000000002e0f64;
            let v8961 = -1e0f64;
            let v8986 = 1.0000000000000001e-11f64;
            let v8988 = 1.0000000000000001e-11f64;
            let v8990 = 1.0000000000000001e-11f64;
            let v8992 = 1.0000000000000001e-11f64;
            let v9024 = 1.0000000000000001e-11f64;
            let v9026 = 1.0000000000000001e-11f64;
            let v9027 = 1.0000000000000001e-11f64;
            let v9099 = 5.5224904e-23f64;
            let v9108 = 0e0f64;
            let v9111 = 0e0f64;
            let v9119 = 0e0f64;
            let v9129 = node_potentials[14];
            let v9130 = 0e0f64;
            let v9131 = 0e0f64;
            let v9145 = 0e0f64;
            let v9146 = 0e0f64;
            let v9147 = 0e0f64;
            let v9148 = 0e0f64;
            let v9149 = 0e0f64;
            let v9152 = node_potentials[1];
            let v9155 = 0e0f64;
            let v9177 = node_potentials[4];
            let v9182 = 0e0f64;
            let v9185 = node_potentials[9];
            let v9190 = node_potentials[8];
            let v9193 = 0e0f64;
            let v9194 = 0e0f64;
            let v9201 = 1e-5f64;
            let v9204 = 1e-5f64;
            let v9207 = 0e0f64;
            let v9208 = 0e0f64;
            let v9217 = 1e-5f64;
            let v9220 = 0e0f64;
            let v9225 = 0e0f64;
            let v9227 = 1e-5f64;
            let v9230 = 0e0f64;
            let v9238 = 1e-5f64;
            let v9241 = 1e-5f64;
            let v9244 = 1e-5f64;
            let v9247 = 0e0f64;
            let v9248 = 0e0f64;
            let v9249 = 0e0f64;
            let v9250 = 0e0f64;
            let v9251 = 0e0f64;
            let v9252 = 0e0f64;
            let v9368 = 1e0f64;
            let v9369 = Lanes([1e0f64; 1]);
            let v9370 = Lanes([1e0f64; 1]);
            let v9371 = Lanes([1e0f64; 1]);
            let v9372 = Lanes([1e0f64; 1]);
            let v9373 = Lanes([1e0f64; 1]);
            let v9374 = Lanes([1e0f64; 1]);
            let v9375 = Lanes([1e0f64; 1]);
            let v9376 = Lanes([1e0f64; 1]);
            let v9377 = Lanes([1e0f64; 1]);
            let v9378 = Lanes([1e0f64; 1]);
            let v9379 = Lanes([1e0f64; 1]);
            let v9380 = Lanes([1e0f64; 1]);
            let v9381 = Lanes([1e0f64; 1]);
            let v9382 = Lanes([1e0f64; 1]);
            let v9383 = Lanes([1e0f64; 1]);
            let v9384 = Lanes([1e0f64; 1]);
            let v9385 = Lanes([1e0f64; 1]);
            let v10374 = Lanes([0e0f64; 1]);
            let v10375 = Lanes([0e0f64; 1]);
            let v10376 = Lanes([0e0f64; 1]);
            let v10380 = Lanes([0e0f64; 2]);
            let v10381 = Lanes([0e0f64; 2]);
            let v10382 = Lanes([0e0f64; 1]);
            let v10389 = Lanes([0e0f64; 1]);
            let v10390 = -1e0f64;
            let v10435 = 2e0f64;
            let v10504 = Lanes([0e0f64; 3]);
            let v10515 = -8.75e-1f64;
            let v10530 = Lanes([0e0f64; 2]);
            let v10531 = Lanes([0e0f64; 3]);
            let v10579 = Lanes([0e0f64; 5]);
            let v10625 = Lanes([0e0f64; 4]);
            let v10660 = Lanes([0e0f64; 4]);
            let v10930 = -6.666666666666667e-1f64;
            let v10999 = -6.666666666666667e-1f64;
            let v11036 = Lanes([0e0f64; 1]);
            let v11062 = Lanes([0e0f64; 6]);
            let v11131 = -8.75e-1f64;
            let v11304 = 0e0f64;
            let v11387 = -8.75e-1f64;
            let v12048 = -7.5e-1f64;
            let v12065 = -7.5e-1f64;
            let v12122 = -7.5e-1f64;
            let v12637 = -8.75e-1f64;
            let v12843 = -8.75e-1f64;
            let v13271 = -7.5e-1f64;
            let v13312 = -7.5e-1f64;
            let v13515 = -7.5e-1f64;
            let v13562 = -7.5e-1f64;
            let v14250 = -8.75e-1f64;
            let v14461 = -6.666666666666667e-1f64;
            let v14529 = -7.5e-1f64;
            let v14840 = -6.666666666666667e-1f64;
            let v14979 = -5e-1f64;
            let v15063 = -8.75e-1f64;
            let v15765 = -6.666666666666667e-1f64;
            let v15770 = -6.666666666666667e-1f64;
            let v16095 = -6.666666666666667e-1f64;
            let v16265 = -6.666666666666667e-1f64;
            let v16270 = -6.666666666666667e-1f64;
            let v16595 = -6.666666666666667e-1f64;
            let v16829 = -6.666666666666667e-1f64;
            let v16834 = -6.666666666666667e-1f64;
            let v17168 = -6.666666666666667e-1f64;
            let v17357 = -6.666666666666667e-1f64;
            let v17362 = -6.666666666666667e-1f64;
            let v17696 = -6.666666666666667e-1f64;
            let v17770 = Lanes([0e0f64; 3]);
            let v17771 = Lanes([0e0f64; 3]);
            let v18459 = Lanes([0e0f64; 5]);
            let v18634 = Lanes([0e0f64; 3]);
            let v18635 = Lanes([0e0f64; 7]);
            let v18636 = Lanes([0e0f64; 7]);
            let v18661 = Lanes([0e0f64; 7]);
            let v18662 = Lanes([0e0f64; 7]);
            let v18663 = Lanes([0e0f64; 8]);
            let v18741 = ddt_scale();
            let v18789 = Lanes([0e0f64; 2]);
            let v18818 = Lanes([0e0f64; 2]);
            let v18819 = Lanes([0e0f64; 2]);
            let v18820 = Lanes([0e0f64; 2]);
            let v19041 = -7.5e-1f64;
            let v19088 = -7.5e-1f64;
            let v9253: f64;
            let v9254: f64;
            if v2 != 0.0 {
                let v5 = if v4 == v0 { 1.0 } else { 0.0 };
                if v5 != 0.0 {
                } else {
                }
                v9253 = v3;
                v9254 = v0;
            } else {
                if v7 != 0.0 {
                    let v8 = if v4 == v1 { 1.0 } else { 0.0 };
                    if v8 != 0.0 {
                    } else {
                    }
                } else {
                }
                v9253 = v0;
                v9254 = v6;
            }
            let v9 = if v4 == v0 { 1.0 } else { 0.0 };
            let v9255: f64;
            if v9 != 0.0 {
                v9255 = v10;
            } else {
                v9255 = v0;
            }
            let v17 = (v14 * v15) % v15;
            let v21 = v19 * v20;
            let v24 = v22 / v23;
            let v26 = v25 * v20;
            let v28 = v27 / v23;
            let v31 = v30 / v23;
            let v33 = v32 / v23;
            let v35 = v34 * v20;
            let v37 = v36 / v20;
            let v39 = v38 / v23;
            let v41 = v40 / v23;
            let v43 = v42 / v23;
            let v45 = v44 / v29;
            let v47 = v46 * v20;
            let v49 = if v48 == v0 { 1.0 } else { 0.0 };
            let v51: f64;
            if v49 != 0.0 {
                v51 = v0;
            } else {
                v51 = v50;
            }
            let v53: f64;
            if v49 != 0.0 {
                v53 = v0;
            } else {
                v53 = v52;
            }
            let v55 = if v54 == v0 { 1.0 } else { 0.0 };
            let v57: f64;
            if v55 != 0.0 {
                v57 = v0;
            } else {
                v57 = v56;
            }
            let v59: f64;
            if v49 != 0.0 {
                v59 = v0;
            } else {
                v59 = v58;
            }
            let v62 = v60 * v61;
            let v65 = v63 + v64;
            let v69 = v67 * v68;
            let v77: f64;
            if v72 != 0.0 {
                v77 = v73;
            } else {
                let v76 = v74 / (v12 * v30);
                v77 = v76;
            }
            let v83 = if (if v77 < v80 { 1.0 } else { 0.0 }) != 0.0 && v82 != 0.0 { 1.0 } else { 0.0 };
            let v4351: f64;
            if v83 != 0.0 {
                let v85 = v84 - v77;
                let v86 = v85 * v85;
                let v89 = (v86 * v86) + v88;
                let v109: f64;
                if v92 != 0.0 {
                    let v103: f64;
                    if v93 != 0.0 {
                        v103 = v1;
                    } else {
                        let v104: f64;
                        if v94 != 0.0 {
                            v104 = v78;
                        } else {
                            let v105: f64;
                            if v95 != 0.0 {
                                v105 = v96;
                            } else {
                                let v106: f64;
                                if v97 != 0.0 {
                                    v106 = v90;
                                } else {
                                    v106 = v0;
                                }
                                v105 = v106;
                            }
                            v104 = v105;
                        }
                        v103 = v104;
                    }
                    let mut v98: f64 = 0.0;
                    let mut v100: f64 = 0.0;
                    v98 = v0;
                    v100 = v89;
                    loop {
                        let v99 = if v98 < v103 { 1.0 } else { 0.0 };
                        if v99 == 0.0 {
                            break;
                        }
                        let v101 = v100.sqrt();
                        let v102 = v98 + v1;
                        v98 = v102;
                        v100 = v101;
                    }
                    v109 = v100;
                } else {
                    let v108 = v89.powf(v107);
                    v109 = v108;
                }
                let v114 = v113 - ((v85 * v79) * (v1 / v109));
                v4351 = v114;
            } else {
                v4351 = v77;
            }
            let v121 = v115 - (v65 * (v116 + (v65 * v117)));
            let v124 = v123 / v12;
            let v125 = v1 / v124;
            let v127 = v126 / v122;
            let v128 = v122 / v126;
            let v130 = v126 / v129;
            let v131 = v129 / v126;
            let v132 = v131 + v125;
            let v136 = v133 - (v78 * v134);
            let v139 = v133 - (v78 * v137);
            let v141 = if v140 == v0 { 1.0 } else { 0.0 };
            let v142: f64;
            if v141 != 0.0 {
                v142 = v133;
            } else {
                v142 = v136;
            }
            let v143 = v142 * v61;
            let v146 = v144 / v145;
            let v148 = if v17 < v1 { 1.0 } else { 0.0 };
            let v150: f64;
            if v148 != 0.0 {
                v150 = v0;
            } else {
                v150 = v149;
            }
            let v152: f64;
            if v148 != 0.0 {
                v152 = v147;
            } else {
                v152 = v151;
            }
            let v165: f64;
            let v167: f64;
            if v9 != 0.0 {
                let v154 = v146 - (v78 * v147);
                let v156 = v146 - (v78 * v152);
                v165 = v154;
                v167 = v156;
            } else {
                let v159 = v146 - (v157 * v150);
                let v160 = v78 - v157;
                let v162 = v159 - (v160 * v147);
                let v164 = v159 - (v160 * v152);
                v165 = v162;
                v167 = v164;
            }
            let v166 = v165 * v145;
            let v168 = v167 * v145;
            let v169 = v146 * v61;
            let v170 = v169 * v143;
            let v183 = (v171 * (v1 + (v172 / (v143.powf(v173))))) * (v1 + (v178 / (v169.powf(v179))));
            let v184 = if v17 > v96 { 1.0 } else { 0.0 };
            let v188 = if v187 > v0 { 1.0 } else { 0.0 };
            let v189 = if (if v184 != 0.0 && (if v24 < v31 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v188 != 0.0 { 1.0 } else { 0.0 };
            let v190: f64;
            if v189 != 0.0 {
                v190 = v31;
            } else {
                v190 = v24;
            }
            let v196 = v190 * (v1 + (v191 / (v169.powf(v192))));
            let v198 = v13 * v133;
            let v205 = v78 / ((v1 / (v197 + v198)) + (v1 / (v201 + v198)));
            let v209 = v206 / (v207 * v65);
            let v211 = (v206 * v33) * v123;
            let v216 = v212 * (v143.powf((-v213)));
            let v221 = v217 * (v143.powf((-v218)));
            let v227 = v222 * ((v143 + v62).powf((-v224)));
            let v231 = ((v228 * v43) * v123).sqrt();
            let v233 = v1 / (v43 * v43);
            let v239 = ((v1 + (v1 / v143)).powf(v236)) * v238;
            let v245 = v142 + (v241 / (v170.powf(v242)));
            let v249 = v246 / (v170.powf(v247));
            let v262 = (v250 * (v1 + (v251 / ((v245 * v61).powf(v253))))) + (v258 / (v169.powf(v259)));
            let v267 = v1 + ((v143.powf(v263)) * v265);
            let v279 = (v268 * (v269 + (v165 / (v96 * v270)))) / ((v270 * (v133 - v275)) * v145);
            let v281 = if v280 <= v0 { 1.0 } else { 0.0 };
            let v2080: f64;
            let v2106: f64;
            let v2107: f64;
            let v2121: f64;
            let v2196: f64;
            let v2200: f64;
            if v281 != 0.0 {
                let v286 = v1 + (v282 / (v169.powf(v283)));
                let v293 = v287 * (v1 + (v288 / (v143.powf(v289))));
                let v296 = v143 / (v143 + v294);
                let v303 = v297 * (v1 + (v298 / (v143.powf(v299))));
                let v308 = v304 * (v1 + (v305 / v143));
                v2080 = v293;
                v2106 = v296;
                v2107 = v286;
                v2121 = v2122;
                v2196 = v308;
                v2200 = v303;
            } else {
                let v309 = v169.powf(v283);
                let v319 = (v310 * (v1 + (v311 / (v143.powf(v312))))) * (v309 / (v309 + v282));
                let v323 = v287 * (v1 + (v288 / (v143.powf(v289))));
                let v329 = v294 * (v1 + (v324 / (v143.powf(v325))));
                let v333 = v297 * (v1 + (v298 / (v143.powf(v299))));
                let v336 = v304 * (v1 + (v305 / v143));
                v2080 = v323;
                v2106 = v329;
                v2107 = v2108;
                v2121 = v319;
                v2196 = v336;
                v2200 = v333;
            }
            let v342 = ((v61 * v168) * v338) / (v143.powf(v340));
            let v349 = v343 * (v1 + (v344 / (v143.powf(v345))));
            let v2097: f64;
            if v281 != 0.0 {
                let v353 = v310 * (v1 + (v311 / (v143.powf(v312))));
                v2097 = v353;
            } else {
                v2097 = v2098;
            }
            let v355 = v354 * v143;
            let v363 = (((v355 * v356) / (v355 + v356)) + v360) + v362;
            let v364 = if v363 < v96 { 1.0 } else { 0.0 };
            let v2662: f64;
            if v364 != 0.0 {
                v2662 = v96;
            } else {
                v2662 = v363;
            }
            let v367 = v365 * v366;
            let v379 = if v378 == v0 { 1.0 } else { 0.0 };
            let v380: f64;
            if v379 != 0.0 {
                v380 = v0;
            } else {
                v380 = v1;
            }
            let v381 = ctx.simparam_or("gmin", v0);
            let v385 = v384 + v64;
            let v386 = v35 / v166;
            let v387 = v37 * v168;
            let v399 = if (if (if v388 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v390 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v145 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if (if v145 > v1 { 1.0 } else { 0.0 }) != 0.0 && (if v395 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v416: f64;
            if v399 != 0.0 {
                let mut v400: f64 = 0.0;
                let mut v402: f64 = 0.0;
                v400 = v0;
                v402 = v0;
                loop {
                    let v401 = if v400 < v145 { 1.0 } else { 0.0 };
                    if v401 == 0.0 {
                        break;
                    }
                    let v405 = v400 * (v395 + v133);
                    let v412 = (v402 + (v1 / ((v388 + v198) + v405))) + (v1 / ((v390 + v198) + v405));
                    let v413 = v400 + v1;
                    v400 = v413;
                    v402 = v412;
                }
                let v415 = (v78 * v145) / v402;
                v416 = v415;
            } else {
                v416 = v0;
            }
            let v417 = if v416 > v0 { 1.0 } else { 0.0 };
            let v480: f64;
            if v417 != 0.0 {
                let v420 = v1 / (v1 + v418);
                let v432 = (v196 * (v1 + (v420 * ((v421 / v416).powf(v423))))) / (v1 + (v420 * ((v421 / v205).powf(v423))));
                v480 = v432;
            } else {
                v480 = v196;
            }
            let v444 = v28 / v31;
            let v446 = (v444 - ((v1 + (v433 / (v169.powf(v434)))) * (v1 + (v438 / (v143.powf(v439)))))) - v20;
            let v448 = (v90 * v444) * v20;
            let v449 = if v448 > v0 { 1.0 } else { 0.0 };
            let v451: f64;
            if v449 != 0.0 {
                v451 = v448;
            } else {
                let v450 = -v448;
                v451 = v450;
            }
            let v458 = v31 * (v444 - (v13 * (v446 + (((v446 * v446) + v451).sqrt()))));
            let v477: f64;
            if v417 != 0.0 {
                let v461 = v1 / (v1 + v459);
                let v473 = (v458 * (v1 + (v461 * ((v462 / v416).powf(v464))))) / (v1 + (v461 * ((v462 / v205).powf(v464))));
                v477 = v473;
            } else {
                v477 = v458;
            }
            let v476 = if (if v142 > v187 { 1.0 } else { 0.0 }) != 0.0 || (if v187 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v489: f64;
            if v476 != 0.0 {
                let v483 = ((v477 * (v142 - v187)) + (v480 * v187)) / v142;
                v489 = v483;
            } else {
                let v488 = v480 + (((v480 - v477) * (v187 - v142)) / v187);
                v489 = v488;
            }
            let v490 = v206 * v489;
            let v491 = v490 * v123;
            let v492 = v78 * v491;
            let v495 = if (if v142 <= (v78 * v187) { 1.0 } else { 0.0 }) != 0.0 && v188 != 0.0 { 1.0 } else { 0.0 };
            let v703: f64;
            if v495 != 0.0 {
                let v503 = ((((v78 * v480) - (((v480 - v477) * v142) / v187)) - v477) / v477).ln();
                v703 = v503;
            } else {
                v703 = v0;
            }
            let v508 = v504 * ((v489 / v505).ln());
            let v513 = v509 * ((v477 / v510).ln());
            let v516 = (v514 / v489).sqrt();
            let v527 = (v1 + (v517 / (v143.powf(v518)))) * (v1 + (v522 / (v170.powf(v523))));
            let v537 = (v13 * (v527 + (((v527 * v527) + v530).sqrt()))) + v536;
            let v538 = if v537 < v0 { 1.0 } else { 0.0 };
            let v705: f64;
            if v538 != 0.0 {
                v705 = v0;
            } else {
                v705 = v537;
            }
            let v540 = if v539 == v1 { 1.0 } else { 0.0 };
            let v9150: f64;
            if v540 != 0.0 {
                let v541 = if v279 > v529 { 1.0 } else { 0.0 };
                let v9151: f64;
                if v541 != 0.0 {
                    let v542 = v1 / v279;
                    v9151 = v542;
                } else {
                    v9151 = v543;
                }
                v9150 = v9151;
            } else {
                v9150 = v544;
            }
            let v546 = if v545 == v1 { 1.0 } else { 0.0 };
            let v9179: f64;
            if v546 != 0.0 {
                let v550 = (v547 * v166) + v549;
                let v551 = if v550 < v29 { 1.0 } else { 0.0 };
                let v9180: f64;
                if v551 != 0.0 {
                    v9180 = v29;
                } else {
                    v9180 = v550;
                }
                v9179 = v9180;
            } else {
                v9179 = v29;
            }
            let v553 = if v552 == v1 { 1.0 } else { 0.0 };
            let v9183: f64;
            let v9188: f64;
            if v553 != 0.0 {
                let v555 = if v554 < v29 { 1.0 } else { 0.0 };
                let v9189: f64;
                if v555 != 0.0 {
                    v9189 = v557;
                } else {
                    let v559 = v23 + (v1 / v554);
                    v9189 = v559;
                }
                let v561 = if v560 < v29 { 1.0 } else { 0.0 };
                let v9184: f64;
                if v561 != 0.0 {
                    v9184 = v562;
                } else {
                    let v564 = v23 + (v1 / v560);
                    v9184 = v564;
                }
                v9183 = v9184;
                v9188 = v9189;
            } else {
                v9183 = v0;
                v9188 = v0;
            }
            let v565 = if v4 == v1 { 1.0 } else { 0.0 };
            let v3861: f64;
            let v6032: f64;
            let v6899: f64;
            let v7794: f64;
            let v7899: f64;
            let v7903: f64;
            let v8421: f64;
            let v8424: f64;
            let v8442: f64;
            let v8445: f64;
            if v565 != 0.0 {
                let v3862: f64;
                let v6033: f64;
                let v8422: f64;
                let v8425: f64;
                if v566 != 0.0 {
                    let v572: f64;
                    if v375 != 0.0 {
                        v572 = v567;
                    } else {
                        let v571 = (v568 * v145) * v570;
                        v572 = v571;
                    }
                    let v577: f64;
                    if v376 != 0.0 {
                        v577 = v573;
                    } else {
                        let v576 = (v574 * v145) * v570;
                        v577 = v576;
                    }
                    let v579 = if (if v572 > v0 { 1.0 } else { 0.0 }) != 0.0 && v371 != 0.0 { 1.0 } else { 0.0 };
                    let v8423: f64;
                    if v579 != 0.0 {
                        let v582 = (-v572) * v581;
                        v8423 = v582;
                    } else {
                        v8423 = v0;
                    }
                    let v584 = if (if v577 > v0 { 1.0 } else { 0.0 }) != 0.0 && v372 != 0.0 { 1.0 } else { 0.0 };
                    let v3863: f64;
                    let v8426: f64;
                    if v584 != 0.0 {
                        let v587 = (-v577) * v586;
                        v3863 = v0;
                        v8426 = v587;
                    } else {
                        v3863 = v577;
                        v8426 = v0;
                    }
                    v3862 = v3863;
                    v6033 = v572;
                    v8422 = v8423;
                    v8425 = v8426;
                } else {
                    v3862 = v0;
                    v6033 = v0;
                    v8422 = v0;
                    v8425 = v0;
                }
                let v588 = if v570 > v133 { 1.0 } else { 0.0 };
                let v591: f64;
                if v588 != 0.0 {
                    let v590 = v13 * (v570 - v133);
                    v591 = v590;
                } else {
                    v591 = v0;
                }
                let v592 = if v373 == v0 { 1.0 } else { 0.0 };
                let v594: f64;
                if v592 != 0.0 {
                    v594 = v591;
                } else {
                    v594 = v382;
                }
                let v593 = if v374 == v0 { 1.0 } else { 0.0 };
                let v597: f64;
                if v593 != 0.0 {
                    v597 = v591;
                } else {
                    v597 = v383;
                }
                let v595 = v145 * v594;
                let v596 = v166 + v595;
                let v598 = v145 * v597;
                let v599 = v166 + v598;
                let v600 = v168 + v595;
                let v601 = v168 + v598;
                v3861 = v3862;
                v6032 = v6033;
                v6899 = v601;
                v7794 = v600;
                v7899 = v596;
                v7903 = v599;
                v8421 = v8422;
                v8424 = v8425;
                v8442 = v594;
                v8445 = v597;
            } else {
                v3861 = v0;
                v6032 = v0;
                v6899 = v0;
                v7794 = v0;
                v7899 = v0;
                v7903 = v0;
                v8421 = v0;
                v8424 = v0;
                v8442 = v382;
                v8445 = v383;
            }
            let v605 = v365 * (v602 - v603);
            let v10365 = ((Lanes([v9369[0], 0.0])) - (Lanes([0.0, v9370[0]]))) * v365;
            let v608 = v365 * (v606 - v603);
            let v10369 = ((Lanes([0.0, v9371[0]])) - (Lanes([v9370[0], 0.0]))) * v365;
            let v611 = v365 * (v609 - v603);
            let v10373 = ((Lanes([0.0, v9372[0]])) - (Lanes([v9370[0], 0.0]))) * v365;
            let v7879: f64;
            let v7880: f64;
            let v8997: f64;
            let v9004: f64;
            let v9029: f64;
            let v9036: f64;
            let v9386: Lanes<2>;
            let v9387: Lanes<2>;
            let v9388: Lanes<1>;
            let v9389: Lanes<1>;
            let v9390: Lanes<1>;
            let v9391: Lanes<1>;
            if v565 != 0.0 {
                let v615 = v365 * (v609 - v602);
                let v10386 = ((Lanes([0.0, v9372[0]])) - (Lanes([v9369[0], 0.0]))) * v365;
                let v8998: f64;
                let v9005: f64;
                let v9392: Lanes<1>;
                let v9393: Lanes<1>;
                if v71 != 0.0 {
                    let v619 = v617 * v618;
                    let v10387 = v9375 * v617;
                    let v622 = v620 * v621;
                    let v10388 = v9376 * v620;
                    v8998 = v619;
                    v9005 = v622;
                    v9392 = v10387;
                    v9393 = v10388;
                } else {
                    v8998 = v0;
                    v9005 = v0;
                    v9392 = v10382;
                    v9393 = v10374;
                }
                v7879 = v615;
                v7880 = v611;
                v8997 = v8998;
                v9004 = v9005;
                v9029 = v0;
                v9036 = v0;
                v9386 = v10386;
                v9387 = v10373;
                v9388 = v9392;
                v9389 = v9393;
                v9390 = v10375;
                v9391 = v10376;
            } else {
                let v9006: f64;
                let v9030: f64;
                let v9037: f64;
                let v9394: Lanes<1>;
                let v9395: Lanes<1>;
                let v9396: Lanes<1>;
                if v71 != 0.0 {
                    let v625 = v623 * v624;
                    let v10377 = v9377 * v623;
                    let v628 = v626 * v627;
                    let v10378 = v9378 * v626;
                    let v630 = v629 * v621;
                    let v10379 = v9376 * v629;
                    v9006 = v630;
                    v9030 = v625;
                    v9037 = v628;
                    v9394 = v10379;
                    v9395 = v10377;
                    v9396 = v10378;
                } else {
                    v9006 = v0;
                    v9030 = v0;
                    v9037 = v0;
                    v9394 = v10374;
                    v9395 = v10375;
                    v9396 = v10376;
                }
                v7879 = v0;
                v7880 = v0;
                v8997 = v0;
                v9004 = v9006;
                v9029 = v9030;
                v9036 = v9037;
                v9386 = v10380;
                v9387 = v10381;
                v9388 = v10382;
                v9389 = v9394;
                v9390 = v9395;
                v9391 = v9396;
            }
            let v632 = if v631 > v0 { 1.0 } else { 0.0 };
            let v633 = if v35 > v0 { 1.0 } else { 0.0 };
            let v634 = if v632 != 0.0 && v633 != 0.0 { 1.0 } else { 0.0 };
            let v638: f64;
            let v9397: Lanes<1>;
            if v634 != 0.0 {
                let v636 = if v635 > v0 { 1.0 } else { 0.0 };
                let v637: f64;
                let v9398: Lanes<1>;
                if v636 != 0.0 {
                    v637 = v635;
                    v9398 = v9379;
                } else {
                    v637 = v0;
                    v9398 = v10389;
                }
                v638 = v637;
                v9397 = v9398;
            } else {
                v638 = v0;
                v9397 = v10389;
            }
            let v639 = if v605 >= v0 { 1.0 } else { 0.0 };
            let v782: f64;
            let v820: f64;
            let v824: f64;
            let v6046: f64;
            let v6048: f64;
            let v7825: f64;
            let v9399: Lanes<3>;
            let v9400: Lanes<2>;
            let v9401: Lanes<3>;
            if v639 != 0.0 {
                let v10398 = Lanes([0.0, v10373[0], v10373[1]]);
                let v10399 = Lanes([0.0, v10369[0], v10369[1]]);
                v782 = v611;
                v820 = v605;
                v824 = v608;
                v6046 = v1;
                v6048 = v0;
                v7825 = v1;
                v9399 = v10398;
                v9400 = v10365;
                v9401 = v10399;
            } else {
                let v641 = -v605;
                let v10391 = v10365 * v10390;
                let v642 = v608 - v605;
                let v10394 = (Lanes([0.0, v10369[0], v10369[1]])) - (Lanes([v10365[0], v10365[1], 0.0]));
                let v643 = v611 - v605;
                let v10397 = (Lanes([0.0, v10373[0], v10373[1]])) - (Lanes([v10365[0], v10365[1], 0.0]));
                v782 = v643;
                v820 = v641;
                v824 = v642;
                v6046 = v0;
                v6048 = v1;
                v7825 = v640;
                v9399 = v10397;
                v9400 = v10391;
                v9401 = v10394;
            }
            let v645 = if v70 >= v644 { 1.0 } else { 0.0 };
            if v645 != 0.0 {
            } else {
            }
            let v647 = if v70 >= v646 { 1.0 } else { 0.0 };
            if v647 != 0.0 {
            } else {
            }
            let v649: f64;
            if v377 != 0.0 {
                v649 = v385;
            } else {
                v649 = v648;
            }
            let v651: f64;
            if v380 != 0.0 {
                let v650 = v649 + v378;
                v651 = v650;
            } else {
                v651 = v649;
            }
            let v652 = v651 + v638;
            let v653 = v652 - v65;
            let v654 = v652 + v65;
            let v661 = (v121 - (v656 * v653)) - (v659 * (v653 * v654));
            let v10406 = ((v9397 * v656) * v10390) - (((v9397 * v654) + (v9397 * v653)) * v659);
            let v662 = v207 * v652;
            let v663 = v206 / v662;
            let v10410 = (((v9397 * v207) * v663) * v10390) / v662;
            let v664 = v663 * v663;
            let v10411 = v10410 * v663;
            let v10412 = v10411 + v10411;
            let v665 = v1 / v663;
            let v10415 = ((v10410 * v665) * v10390) / v663;
            let v684 = ((v666 * (v1 + (v667 / (v169.powf(v668))))) * (v1 + (v673 / (v143.powf(v674))))) * (v1 + (v679 / (v170.powf(v680))));
            let v687 = v1 / (v1 + v685);
            let v689 = v688 / v69;
            let v693 = if (if v689 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v691 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v695: f64;
            if v693 != 0.0 {
                v695 = v1;
            } else {
                let v694 = v689.powf(v691);
                v695 = v694;
            }
            let v698 = v684 * (v1 + (v687 * v695));
            let v699 = v652 / v65;
            let v10416 = v9397 / v65;
            let v702 = (v699.powf(v700)) / v698;
            let v10421 = (v10416 * (v700 * (v699.powf((v700 - v9368))))) / v698;
            let v704 = v703 * v665;
            let v10422 = v10415 * v703;
            let v711 = v79 * v699;
            let v713 = (v707 + (v708 * v699)) + (v711 * v699);
            let v10428 = (v10416 * v708) + (((v10416 * v79) * v699) + (v10416 * v711));
            let v714 = v1 - v699;
            let v10429 = v10416 * v10390;
            let v716 = v713 - (v26 * v714);
            let v717 = (v705 * v21) / v716;
            let v10434 = (((v10428 - (v10429 * v26)) * v717) * v10390) / v716;
            let v718 = v661.sqrt();
            let v10438 = v10406 * (v9368 / (v10435 * v718));
            let v719 = v661 * v718;
            let v10441 = (v10406 * v718) + (v10438 * v661);
            let v19483 = v699.sqrt();
            let v723 = v720 * (v699 * v19483);
            let v725 = (-v661) / v78;
            let v730 = ((v725 * v663) + ((v121 / v78) * v209)).exp();
            let v731 = v723 * v730;
            let v10453 = (((v10416 * (v721 * v19483)) * v720) * v730) + ((((((v10406 * v10390) / v78) * v663) + (v10410 * v725)) * v730) * v723);
            let v732 = v665.sqrt();
            let v10456 = v10415 * (v9368 / (v10435 * v732));
            let v733 = v231 * v732;
            let v10457 = v10456 * v231;
            let v734 = v733 * v733;
            let v10458 = v10457 * v733;
            let v10459 = v10458 + v10458;
            let v735 = v731 * v731;
            let v10460 = v10453 * v731;
            let v10461 = v10460 + v10460;
            let v736 = v735 * v233;
            let v10462 = v10461 * v233;
            let v766: f64;
            let v9402: Lanes<1>;
            if v184 != 0.0 {
                let v737 = v78 * v665;
                let v738 = v489 / v731;
                let v739 = v738.ln();
                let v740 = v737 * v739;
                let v10480 = ((v10415 * v78) * v739) + (((((v10453 * v738) * v10390) / v731) * (v9368 / v738)) * v737);
                v766 = v740;
                v9402 = v10480;
            } else {
                let v741 = v78 * v665;
                let v742 = v477 / v731;
                let v743 = v742.ln();
                let v744 = v741 * v743;
                let v10471 = ((v10415 * v78) * v743) + (((((v10453 * v742) * v10390) / v731) * (v9368 / v742)) * v741);
                v766 = v744;
                v9402 = v10471;
            }
            let v745 = v123 / v490;
            let v747 = (v745 * v665).sqrt();
            let v749 = v490 * v748;
            let v750 = v749 * v747;
            let v10485 = ((v10415 * v745) * (v9368 / (v10435 * v747))) * v749;
            let v758: f64;
            let v1240: f64;
            let v1262: f64;
            let v9403: Lanes<1>;
            let v9404: Lanes<1>;
            let v9405: Lanes<1>;
            if v565 != 0.0 {
                let v751 = v731 / v489;
                let v10494 = v10453 / v489;
                v758 = v751;
                v1240 = v0;
                v1262 = v0;
                v9403 = v10494;
                v9404 = v10389;
                v9405 = v10389;
            } else {
                let v752 = v78 * v211;
                let v754 = (v752 * v665).sqrt();
                let v10489 = (v10415 * v752) * (v9368 / (v10435 * v754));
                let v755 = v731 / v33;
                let v756 = v755 * v755;
                let v10491 = (v10453 / v33) * v755;
                let v10492 = v10491 + v10491;
                let v757 = v731 / v477;
                let v10493 = v10453 / v477;
                v758 = v757;
                v1240 = v754;
                v1262 = v756;
                v9403 = v10493;
                v9404 = v10489;
                v9405 = v10492;
            }
            let v759 = v758 * v758;
            let v10495 = v9403 * v758;
            let v10496 = v10495 + v10495;
            let v760 = v745 / v663;
            let v762 = (v78 * v760).sqrt();
            let v10503 = ((((v10410 * v760) * v10390) / v663) * v78) * (v9368 / (v10435 * v762));
            let v764 = v763 / v477;
            let v769 = ((v765 * v766) / v477).sqrt();
            let v770 = if v165 < v616 { 1.0 } else { 0.0 };
            let v775: f64;
            if v770 != 0.0 {
                v775 = v1;
            } else {
                v775 = v0;
            }
            let v771 = if v167 < v616 { 1.0 } else { 0.0 };
            let v774: f64;
            if v771 != 0.0 {
                v774 = v1;
            } else {
                v774 = v775;
            }
            let v772 = if v136 < v616 { 1.0 } else { 0.0 };
            let v773: f64;
            if v772 != 0.0 {
                v773 = v1;
            } else {
                v773 = v774;
            }
            if v773 != 0.0 {
            } else {
            }
            let v778: f64;
            let v779: f64;
            if v565 != 0.0 {
                v778 = v708;
                v779 = v776;
            } else {
                v778 = v776;
                v779 = v777;
            }
            let v780 = v779 * v13;
            let v781 = if v778 > v780 { 1.0 } else { 0.0 };
            let v783: f64;
            if v781 != 0.0 {
                v783 = v780;
            } else {
                v783 = v778;
            }
            let v784 = if v782 > v783 { 1.0 } else { 0.0 };
            let v831: f64;
            let v836: f64;
            let v9406: Lanes<3>;
            let v9407: Lanes<3>;
            if v784 != 0.0 {
                let v785 = v782 - v783;
                let v786 = v779 - v783;
                let v787 = v785 * v785;
                let v10505 = v9399 * v785;
                let v10506 = v10505 + v10505;
                let v788 = v786 * v786;
                let v789 = v787 * v787;
                let v10507 = v10506 * v787;
                let v791 = v789 * v787;
                let v10514 = ((((v10507 + v10507) * v787) + (v10506 * v789)) * v787) + (v10506 * v791);
                let v794 = ((v788 * v788) * v788) * v788;
                let v795 = (v791 * v787) + v794;
                let v812: f64;
                let v9408: Lanes<3>;
                if v796 != 0.0 {
                    let v806: f64;
                    if v797 != 0.0 {
                        v806 = v1;
                    } else {
                        let v807: f64;
                        if v798 != 0.0 {
                            v807 = v78;
                        } else {
                            let v808: f64;
                            if v799 != 0.0 {
                                v808 = v96;
                            } else {
                                let v809: f64;
                                if v800 != 0.0 {
                                    v809 = v90;
                                } else {
                                    v809 = v0;
                                }
                                v808 = v809;
                            }
                            v807 = v808;
                        }
                        v806 = v807;
                    }
                    let mut v801: f64 = 0.0;
                    let mut v803: f64 = 0.0;
                    let mut v9409: Lanes<3> = Lanes([0.0; 3]);
                    v801 = v0;
                    v803 = v795;
                    v9409 = v10514;
                    loop {
                        let v802 = if v801 < v806 { 1.0 } else { 0.0 };
                        if v802 == 0.0 {
                            break;
                        }
                        let v804 = v803.sqrt();
                        let v19271 = v9409 * (v9368 / (v10435 * v804));
                        let v805 = v801 + v1;
                        v801 = v805;
                        v803 = v804;
                        v9409 = v19271;
                    }
                    v812 = v803;
                    v9408 = v9409;
                } else {
                    let v811 = v795.powf(v810);
                    let v10518 = v10514 * (v810 * (v795.powf(v10515)));
                    v812 = v811;
                    v9408 = v10518;
                }
                let v813 = v1 / v812;
                let v10521 = ((v9408 * v813) * v10390) / v812;
                let v814 = v785 * v786;
                let v10525 = ((v9399 * v786) * v813) + (v10521 * v814);
                let v816 = v786 * v794;
                let v818 = (v816 * v813) / v795;
                let v10529 = ((v10521 * v816) - (v10514 * v818)) / v795;
                let v819 = v783 + (v814 * v813);
                v831 = v819;
                v836 = v818;
                v9406 = v10525;
                v9407 = v10529;
            } else {
                v831 = v782;
                v836 = v1;
                v9406 = v9399;
                v9407 = v10504;
            }
            let v822 = if v820 > v821 { 1.0 } else { 0.0 };
            let v823: f64;
            let v9410: Lanes<2>;
            if v822 != 0.0 {
                v823 = v821;
                v9410 = v10530;
            } else {
                v823 = v820;
                v9410 = v9400;
            }
            let v825 = if v824 > v821 { 1.0 } else { 0.0 };
            let v826: f64;
            let v9411: Lanes<3>;
            if v825 != 0.0 {
                v826 = v821;
                v9411 = v10531;
            } else {
                v826 = v824;
                v9411 = v9401;
            }
            let v828 = if v824 < v827 { 1.0 } else { 0.0 };
            let v830: f64;
            let v9412: Lanes<3>;
            if v828 != 0.0 {
                v830 = v829;
                v9412 = v10531;
            } else {
                v830 = v826;
                v9412 = v9411;
            }
            let v833 = if v831 < v832 { 1.0 } else { 0.0 };
            let v835: f64;
            let v9413: Lanes<3>;
            if v833 != 0.0 {
                v835 = v834;
                v9413 = v10504;
            } else {
                v835 = v831;
                v9413 = v9406;
            }
            let v10533 = v9410 * v836;
            let v839 = v78 * ((v836 * v823) / v78);
            let v10537 = (((v9407 * v823) + (Lanes([v10533[0], v10533[1], 0.0]))) / v78) * v78;
            let v841 = v839 / v840;
            let v10538 = v10537 / v840;
            let v849 = v846 + (v841 * v847);
            let v851 = v845 + (v841 * v849);
            let v853 = v844 + (v841 * v851);
            let v855 = v843 + (v841 * v853);
            let v857 = v842 + (v841 * v855);
            let v859 = v1 + (v841 * v857);
            let v860 = v840 / v859;
            let v10557 = ((((v10538 * v857) + (((v10538 * v855) + (((v10538 * v853) + (((v10538 * v851) + (((v10538 * v849) + ((v10538 * v847) * v841)) * v841)) * v841)) * v841)) * v841)) * v860) * v10390) / v859;
            let v862 = if v860 < v861 { 1.0 } else { 0.0 };
            let v863: f64;
            let v9414: Lanes<3>;
            if v862 != 0.0 {
                v863 = v861;
                v9414 = v10504;
            } else {
                v863 = v860;
                v9414 = v10557;
            }
            let v864 = v835 + v863;
            let v10558 = v9413 + v9414;
            let v866 = v823 + (v78 * v863);
            let v10560 = Lanes([v9410[0], v9410[1], 0.0]);
            let v10561 = v10560 + (v9414 * v78);
            let v867 = v830 + v863;
            let v10562 = Lanes([v9412[0], v9412[1], v9412[2], 0.0]);
            let v10564 = v10562 + (Lanes([v9414[0], v9414[1], 0.0, v9414[2]]));
            let v878: f64;
            let v988: f64;
            let v9415: Lanes<3>;
            let v9416: Lanes<3>;
            if v565 != 0.0 {
                v878 = v835;
                v988 = v864;
                v9415 = v9413;
                v9416 = v10558;
            } else {
                let v868 = if v17 < v96 { 1.0 } else { 0.0 };
                let v869: f64;
                let v9417: Lanes<3>;
                if v868 != 0.0 {
                    v869 = v835;
                    v9417 = v9413;
                } else {
                    v869 = v0;
                    v9417 = v10504;
                }
                let v870: f64;
                let v9418: Lanes<3>;
                if v868 != 0.0 {
                    v870 = v864;
                    v9418 = v10558;
                } else {
                    v870 = v0;
                    v9418 = v10504;
                }
                v878 = v869;
                v988 = v870;
                v9415 = v9417;
                v9416 = v9418;
            }
            let v872 = (v78 * v490) * v123;
            let v874 = (v872 * v128) * v128;
            let v875 = v830 - v240;
            let v876 = v78 / v874;
            let v10567 = (Lanes([v9412[0], v9412[1], 0.0, v9412[2]])) - (Lanes([0.0, 0.0, v10415[0], 0.0]));
            let v10571 = ((Lanes([v10567[0], v10567[1], v10567[2], v10567[3], 0.0])) - (Lanes([v9415[0], v9415[1], 0.0, 0.0, v9415[2]]))) * v876;
            let v881 = v1 + (v876 * ((v875 - v665) - v878));
            let v10572 = v10571 * v881;
            let v885 = ((v881 * v881) + v883).sqrt();
            let v10578 = (v10571 + ((v10572 + v10572) * (v9368 / (v10435 * v885)))) * v13;
            let v889 = (v13 * (v881 + v885)) + v888;
            let v890 = if v889 < v0 { 1.0 } else { 0.0 };
            let v891: f64;
            let v9419: Lanes<5>;
            if v890 != 0.0 {
                v891 = v0;
                v9419 = v10579;
            } else {
                v891 = v889;
                v9419 = v10578;
            }
            let v893 = (v891 + v362).sqrt();
            let v10585 = Lanes([v9412[0], v9412[1], 0.0, v9412[2], 0.0]);
            let v10588 = (v10585 + (((v9419 * (v9368 / (v10435 * v893))) * v10390) * v874)) - (Lanes([0.0, 0.0, v9402[0], 0.0, 0.0]));
            let v900 = (((v875 + (v874 * (v1 - v893))) - v766) - v79) - v899;
            let v904: f64;
            if v902 != 0.0 {
                v904 = v901;
            } else {
                v904 = v903;
            }
            let v10589 = v10588 * v900;
            let v907 = ((v900 * v900) + v904).sqrt();
            let v910 = v79 + (v13 * (v900 + v907));
            let v911 = v823 / v910;
            let v10597 = Lanes([v9410[0], v9410[1], 0.0, 0.0, 0.0]);
            let v10599 = (v10597 - (((v10588 + ((v10589 + v10589) * (v9368 / (v10435 * v907)))) * v13) * v911)) / v910;
            let v912 = v911 * v911;
            let v10600 = v10599 * v911;
            let v10601 = v10600 + v10600;
            let v10605 = v10601 * v912;
            let v918 = (((v1 + v911) + v912) + (v912 * v911)) + (v912 * v912);
            let v919 = v1 / v918;
            let v920 = v1 - v919;
            let v921 = v920 * v920;
            let v10614 = (((((((v10599 + v10601) + ((v10601 * v911) + (v10599 * v912))) + (v10605 + v10605)) * v919) * v10390) / v918) * v10390) * v920;
            let v10615 = v10614 + v10614;
            let v929 = if (if (if v922 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v924 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v927 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v935: f64;
            if v929 != 0.0 {
                v935 = v0;
            } else {
                v935 = v1;
            }
            let v932 = v508 + v240;
            let v934 = v932 + (((v872 * v508).sqrt()) / v127);
            let v936 = if v935 == v0 { 1.0 } else { 0.0 };
            let v1048: f64;
            let v1128: f64;
            let v1211: f64;
            let v9420: Lanes<4>;
            let v9421: Lanes<4>;
            let v9422: Lanes<5>;
            if v936 != 0.0 {
                let v938 = (v750 * v128) * v128;
                let v939 = v938 * v750;
                let v10658 = (((v10485 * v128) * v128) * v750) + (v10485 * v938);
                let v10659 = Lanes([0.0, 0.0, v10658[0], 0.0, 0.0]);
                v1048 = v128;
                v1128 = v127;
                v1211 = v939;
                v9420 = v10625;
                v9421 = v10625;
                v9422 = v10659;
            } else {
                let v10617 = v10562 - (Lanes([v9415[0], v9415[1], 0.0, v9415[2]]));
                let v942 = ((v830 - v878) - v934) + v927;
                let v10618 = v10617 * v942;
                let v946 = ((v942 * v942) + v944).sqrt();
                let v10624 = (v10617 + ((v10618 + v10618) * (v9368 / (v10435 * v946)))) * v13;
                let v950 = (v13 * (v942 + v946)) + v949;
                let v951 = if v950 < v0 { 1.0 } else { 0.0 };
                let v952: f64;
                let v9423: Lanes<4>;
                if v951 != 0.0 {
                    v952 = v0;
                    v9423 = v10625;
                } else {
                    v952 = v950;
                    v9423 = v10624;
                }
                let v953 = v1 / v952;
                let v10628 = ((v9423 * v953) * v10390) / v952;
                let v955 = v78 * (v934.abs());
                let v957 = (v240 - v934) + v927;
                let v958 = if v957 > v955 { 1.0 } else { 0.0 };
                let v959: f64;
                if v958 != 0.0 {
                    v959 = v957;
                } else {
                    v959 = v955;
                }
                let v960 = v1 / v959;
                let v10629 = v10628 * v10390;
                let v962 = (v960 - v953) - v29;
                let v964 = (v90 * v960) * v29;
                let v965 = if v964 > v0 { 1.0 } else { 0.0 };
                let v967: f64;
                if v965 != 0.0 {
                    v967 = v964;
                } else {
                    let v966 = -v964;
                    v967 = v966;
                }
                let v10630 = v10629 * v962;
                let v970 = ((v962 * v962) + v967).sqrt();
                let v10638 = (((v10629 + ((v10630 + v10630) * (v9368 / (v10435 * v970)))) * v13) * v10390) * v922;
                let v975 = (v922 * (v960 - (v13 * (v962 + v970)))) + v924;
                let v978 = if (v975 * v976) < v122 { 1.0 } else { 0.0 };
                let v979: f64;
                let v9424: Lanes<4>;
                if v978 != 0.0 {
                    v979 = v0;
                    v9424 = v10625;
                } else {
                    v979 = v975;
                    v9424 = v10638;
                }
                let v980 = v122 + v979;
                let v981 = v126 / v980;
                let v10641 = ((v9424 * v981) * v10390) / v980;
                let v982 = v980 / v126;
                let v10642 = v9424 / v126;
                let v983 = v750 * v750;
                let v10643 = v10485 * v750;
                let v984 = v983 * v982;
                let v10645 = (v10643 + v10643) * v982;
                let v10646 = v10642 * v983;
                let v985 = v984 * v982;
                let v10651 = v10642 * v984;
                let v10653 = (((Lanes([0.0, 0.0, v10645[0], 0.0, 0.0])) + (Lanes([v10646[0], v10646[1], 0.0, v10646[2], v10646[3]]))) * v982) + (Lanes([v10651[0], v10651[1], 0.0, v10651[2], v10651[3]]));
                v1048 = v982;
                v1128 = v981;
                v1211 = v985;
                v9420 = v10642;
                v9421 = v10641;
                v9422 = v10653;
            }
            let v986 = if v17 < v96 { 1.0 } else { 0.0 };
            let v987 = if v565 != 0.0 || v986 != 0.0 { 1.0 } else { 0.0 };
            let v1037: f64;
            let v9425: Lanes<4>;
            if v987 != 0.0 {
                let v10661 = v9416 * v10390;
                let v990 = (v13 - v988) - v529;
                let v994: f64;
                if v992 != 0.0 {
                    v994 = v991;
                } else {
                    v994 = v993;
                }
                let v10662 = v10661 * v990;
                let v997 = ((v990 * v990) + v994).sqrt();
                let v10669 = ((v10661 + ((v10662 + v10662) * (v9368 / (v10435 * v997)))) * v13) * v10390;
                let v1007 = (((((-v12) * v12) * v490) / v1004) + v766) - v665;
                let v10670 = v9402 - v10415;
                let v10672 = Lanes([0.0, 0.0, v10670[0], 0.0]);
                let v10673 = (Lanes([v10669[0], v10669[1], 0.0, v10669[2]])) - v10672;
                let v1009 = ((v13 - (v13 * (v990 + v997))) - v1007) - v529;
                let v1011 = (v90 * v1007) * v529;
                let v10675 = (v10670 * v90) * v529;
                let v1012 = if v1011 > v0 { 1.0 } else { 0.0 };
                let v1014: f64;
                let v9426: Lanes<1>;
                if v1012 != 0.0 {
                    v1014 = v1011;
                    v9426 = v10675;
                } else {
                    let v1013 = -v1011;
                    let v10676 = v10675 * v10390;
                    v1014 = v1013;
                    v9426 = v10676;
                }
                let v10677 = v10673 * v1009;
                let v1017 = ((v1009 * v1009) + v1014).sqrt();
                let v1020 = v1007 + (v13 * (v1009 + v1017));
                let v10686 = v10672 + ((v10673 + (((v10677 + v10677) + (Lanes([0.0, 0.0, v9426[0], 0.0]))) * (v9368 / (v10435 * v1017)))) * v13);
                let v1021 = if v17 > v78 { 1.0 } else { 0.0 };
                let v1038: f64;
                let v9427: Lanes<4>;
                if v1021 != 0.0 {
                    let v10687 = v10686 * v10390;
                    let v1023 = (v508 - v1020) - v529;
                    let v1025 = (v90 * v508) * v529;
                    let v1026 = if v1025 > v0 { 1.0 } else { 0.0 };
                    let v1028: f64;
                    if v1026 != 0.0 {
                        v1028 = v1025;
                    } else {
                        let v1027 = -v1025;
                        v1028 = v1027;
                    }
                    let v10688 = v10687 * v1023;
                    let v1031 = ((v1023 * v1023) + v1028).sqrt();
                    let v1034 = v508 - (v13 * (v1023 + v1031));
                    let v10695 = ((v10687 + ((v10688 + v10688) * (v9368 / (v10435 * v1031)))) * v13) * v10390;
                    v1038 = v1034;
                    v9427 = v10695;
                } else {
                    v1038 = v1020;
                    v9427 = v10686;
                }
                v1037 = v1038;
                v9425 = v9427;
            } else {
                v1037 = v0;
                v9425 = v10660;
            }
            let v1083: f64;
            let v9428: Lanes<4>;
            if v986 != 0.0 {
                v1083 = v12;
                v9428 = v10660;
            } else {
                let v1036 = v1035 / v490;
                let v1041 = (v1036 * (v508 - v1037)).sqrt();
                let v10700 = ((v9425 * v10390) * v1036) * (v9368 / (v10435 * v1041));
                v1083 = v1041;
                v9428 = v10700;
            }
            let v1047: f64;
            let v9429: Lanes<4>;
            if v986 != 0.0 {
                let v1043 = (v492 * v508).sqrt();
                v1047 = v1043;
                v9429 = v10660;
            } else {
                let v1046 = (v492 * (v508 - v1037)).sqrt();
                let v10705 = ((v9425 * v10390) * v492) * (v9368 / (v10435 * v1046));
                v1047 = v1046;
                v9429 = v10705;
            }
            let v10706 = v9429 * v1048;
            let v10707 = v9420 * v1047;
            let v1051 = (v932 + (v1047 * v1048)) + v704;
            let v10712 = ((Lanes([v10706[0], v10706[1], v10706[2], 0.0, v10706[3]])) + (Lanes([v10707[0], v10707[1], 0.0, v10707[2], v10707[3]]))) + (Lanes([0.0, 0.0, v10422[0], 0.0, 0.0]));
            let v1053 = v1052 * v508;
            let v10713 = v9425 * v10390;
            let v1055 = (v1053 - v1037) - v529;
            let v10714 = v10713 * v1055;
            let v1061 = ((v1055 * v1055) + ((v1057 * v508) * v529)).sqrt();
            let v1065 = v508 - (v1053 - (v13 * (v1055 + v1061)));
            let v10722 = (((v10713 + ((v10714 + v10714) * (v9368 / (v10435 * v1061)))) * v13) * v10390) * v10390;
            let v1066 = v1065.sqrt();
            let v10725 = v10722 * (v9368 / (v10435 * v1066));
            let v1067 = if v187 != v0 { 1.0 } else { 0.0 };
            let v1137: f64;
            let v9430: Lanes<5>;
            if v1067 != 0.0 {
                let v1070 = (v1068 * v477) * v123;
                let v1076: f64;
                let v9431: Lanes<4>;
                if v986 != 0.0 {
                    let v1072 = (v1070 * v513).sqrt();
                    v1076 = v1072;
                    v9431 = v10660;
                } else {
                    let v1075 = (v1070 * (v513 - v1037)).sqrt();
                    let v10729 = (v10713 * v1070) * (v9368 / (v10435 * v1075));
                    v1076 = v1075;
                    v9431 = v10729;
                }
                let v10730 = v9431 * v1048;
                let v10731 = v9420 * v1076;
                let v1080 = v123 * v1048;
                let v1082 = v1 / (v187 * v187);
                let v1085 = (v78 * v1083) * v1082;
                let v10738 = (v9420 * v123) * v1085;
                let v10739 = ((v9428 * v78) * v1082) * v1080;
                let v1088 = v1087 - v508;
                let v1089 = (v1080 * v1085) * v1088;
                let v1090 = v1051 - ((v513 + v240) + (v1076 * v1048));
                let v1091 = v59 / v187;
                let v10746 = v10561 * v57;
                let v1095 = (v54 + (v1091 * v1065)) + (v57 * v866);
                let v1096 = v1090 * v1089;
                let v1097 = v1096 * v1095;
                let v10753 = ((v10722 * v1091) + (Lanes([v10746[0], v10746[1], 0.0, v10746[2]]))) * v1096;
                let v10755 = ((((v10712 - ((Lanes([v10730[0], v10730[1], v10730[2], 0.0, v10730[3]])) + (Lanes([v10731[0], v10731[1], 0.0, v10731[2], v10731[3]])))) * v1089) + ((((Lanes([v10738[0], v10738[1], 0.0, v10738[2], v10738[3]])) + (Lanes([v10739[0], v10739[1], v10739[2], 0.0, v10739[3]]))) * v1088) * v1090)) * v1095) + (Lanes([v10753[0], v10753[1], v10753[2], 0.0, v10753[3]]));
                v1137 = v1097;
                v9430 = v10755;
            } else {
                v1137 = v0;
                v9430 = v10579;
            }
            let v1099 = (v123 * v1083) * v78;
            let v10758 = v9420 * v1099;
            let v10759 = ((v9428 * v123) * v78) * v1048;
            let v1101 = v1087 - v508;
            let v1103 = v142 - v1102;
            let v1105 = v1 / (v1103 * v1103);
            let v1107 = ((v1048 * v1099) * v1101) * v1105;
            let v1108 = v53 / v142;
            let v10766 = v10561 * v51;
            let v1112 = (v48 + (v1108 * v1065)) + (v51 * v866);
            let v1113 = v1107 * v1112;
            let v10770 = ((v10722 * v1108) + (Lanes([v10766[0], v10766[1], 0.0, v10766[2]]))) * v1107;
            let v10772 = (((((Lanes([v10758[0], v10758[1], 0.0, v10758[2], v10758[3]])) + (Lanes([v10759[0], v10759[1], v10759[2], 0.0, v10759[3]]))) * v1101) * v1105) * v1112) + (Lanes([v10770[0], v10770[1], v10770[2], 0.0, v10770[3]]));
            let v1115 = if v1114 > v0 { 1.0 } else { 0.0 };
            let v1140: f64;
            let v9432: Lanes<4>;
            if v1115 != 0.0 {
                let v10773 = v10406 + v9402;
                let v10774 = v10561 * v1120;
                let v1126 = (v1114 * v12) / ((v142 * v13) + v47);
                let v1127 = (((v661 + v766) - (v78 * v1117)) + (v1120 * v866)) * v1126;
                let v10778 = ((Lanes([0.0, 0.0, v10773[0], 0.0])) + (Lanes([v10774[0], v10774[1], 0.0, v10774[2]]))) * v1126;
                v1140 = v1127;
                v9432 = v10778;
            } else {
                v1140 = v0;
                v9432 = v10660;
            }
            let v1130 = v1128 + (v45 / v165);
            let v1131 = v1 / v1130;
            let v1132 = v1048 - v1131;
            let v10783 = v9429 * v1132;
            let v10784 = (v9420 - (((v9421 * v1131) * v10390) / v1130)) * v1047;
            let v1138 = v1113 + v1137;
            let v10788 = v10772 + v9430;
            let v10791 = (v10788 + ((Lanes([v10783[0], v10783[1], v10783[2], 0.0, v10783[3]])) + (Lanes([v10784[0], v10784[1], 0.0, v10784[2], v10784[3]])))) + (Lanes([v9432[0], v9432[1], v9432[2], 0.0, v9432[3]]));
            let v1142 = ((v1138 + ((v1047 * v1132) + (v1134 / v169))) + v1140) + v249;
            let v1143 = v1051 - v1142;
            let v1144 = if v238 == v0 { 1.0 } else { 0.0 };
            let v1145: f64;
            if v1144 != 0.0 {
                v1145 = v0;
            } else {
                v1145 = v1;
            }
            let v1146 = if v1145 == v0 { 1.0 } else { 0.0 };
            let v1199: f64;
            let v9433: Lanes<4>;
            if v1146 != 0.0 {
                v1199 = v0;
                v9433 = v10625;
            } else {
                let v1148 = v867 - v1147;
                let v1150 = if v1148 < v1149 { 1.0 } else { 0.0 };
                let v1172: f64;
                let v9434: Lanes<4>;
                if v1150 != 0.0 {
                    v1172 = v0;
                    v9434 = v10625;
                } else {
                    let v1151 = if v1148 < v0 { 1.0 } else { 0.0 };
                    let v1173: f64;
                    let v9435: Lanes<4>;
                    if v1151 != 0.0 {
                        let v1156 = v1152 + (v1148 * v1154);
                        let v1158 = v1 + (v1148 * v1156);
                        let v10808 = (v10564 * v1158) + (((v10564 * v1156) + ((v10564 * v1154) * v1148)) * v1148);
                        let v1160 = v1 + (v1148 * v1158);
                        v1173 = v1160;
                        v9435 = v10808;
                    } else {
                        let v1165 = v1162 + (v1148 * v1163);
                        let v1167 = v1161 + (v1148 * v1165);
                        let v1169 = v1 + (v1148 * v1167);
                        let v10801 = (v10564 * v1169) + (((v10564 * v1167) + (((v10564 * v1165) + ((v10564 * v1163) * v1148)) * v1148)) * v1148);
                        let v1171 = v1 + (v1148 * v1169);
                        v1173 = v1171;
                        v9435 = v10801;
                    }
                    v1172 = v1173;
                    v9434 = v9435;
                }
                let v1174 = v1172 - v1;
                let v10809 = v9434 * v1174;
                let v1178 = ((v1174 * v1174) + v1176).sqrt();
                let v10815 = (v9434 + ((v10809 + v10809) * (v9368 / (v10435 * v1178)))) * v13;
                let v1182 = (v13 * (v1174 + v1178)) + v1181;
                let v1183 = if v1182 < v0 { 1.0 } else { 0.0 };
                let v1184: f64;
                let v9436: Lanes<4>;
                if v1183 != 0.0 {
                    v1184 = v0;
                    v9436 = v10625;
                } else {
                    v1184 = v1182;
                    v9436 = v10815;
                }
                let v10817 = (v9436 * v239) * v10390;
                let v1187 = (v1 - (v1184 * v239)) - v899;
                let v1191: f64;
                if v1189 != 0.0 {
                    v1191 = v1188;
                } else {
                    v1191 = v1190;
                }
                let v10818 = v10817 * v1187;
                let v1194 = ((v1187 * v1187) + v1191).sqrt();
                let v1197 = v1 - (v13 * (v1187 + v1194));
                let v10825 = ((v10817 + ((v10818 + v10818) * (v9368 / (v10435 * v1194)))) * v13) * v10390;
                v1199 = v1197;
                v9433 = v10825;
            }
            let v1200 = (v875 + v1142) - v1199;
            let v10827 = Lanes([v9433[0], v9433[1], 0.0, v9433[2], v9433[3]]);
            let v10828 = (v10585 + v10791) - v10827;
            let v1202 = (v477 / v33).ln();
            let v1203 = v665 * v1202;
            let v10829 = v10415 * v1202;
            let v1205 = (v240 - v1142) + v1199;
            let v1206 = v750 * v1048;
            let v10830 = v10485 * v1048;
            let v10831 = v9420 * v750;
            let v10834 = (Lanes([0.0, 0.0, v10830[0], 0.0, 0.0])) + (Lanes([v10831[0], v10831[1], 0.0, v10831[2], v10831[3]]));
            let v1207 = v1206 * v1206;
            let v10835 = v10834 * v1206;
            let v10836 = v10835 + v10835;
            let v4305: f64;
            let v4307: f64;
            let v4311: f64;
            let v4314: f64;
            let v4325: f64;
            let v4336: f64;
            let v4340: f64;
            let v4348: f64;
            let v4381: f64;
            let v4421: f64;
            let v4428: f64;
            let v4438: f64;
            let v4439: f64;
            let v4445: f64;
            let v4637: f64;
            let v4735: f64;
            let v4787: f64;
            let v4843: f64;
            let v4964: f64;
            let v4973: f64;
            let v4977: f64;
            let v5093: f64;
            let v5501: f64;
            let v5643: f64;
            let v5721: f64;
            let v5781: f64;
            let v8304: f64;
            let v8481: f64;
            let v8486: f64;
            let v8491: f64;
            let v8497: f64;
            let v8564: f64;
            let v8576: f64;
            let v9211: f64;
            let v9437: Lanes<6>;
            let v9438: Lanes<6>;
            let v9439: Lanes<6>;
            let v9440: Lanes<6>;
            let v9441: Lanes<6>;
            let v9442: Lanes<6>;
            let v9443: Lanes<6>;
            let v9444: Lanes<6>;
            let v9445: Lanes<6>;
            let v9446: Lanes<6>;
            let v9447: Lanes<6>;
            let v9448: Lanes<6>;
            let v9449: Lanes<6>;
            let v9450: Lanes<1>;
            let v9451: Lanes<1>;
            let v9452: Lanes<6>;
            let v9453: Lanes<5>;
            let v9454: Lanes<4>;
            let v9455: Lanes<5>;
            let v9456: Lanes<5>;
            let v9457: Lanes<6>;
            let v9458: Lanes<5>;
            let v9459: Lanes<6>;
            let v9460: Lanes<6>;
            let v9461: Lanes<6>;
            let v9462: Lanes<6>;
            let v9463: Lanes<6>;
            let v9464: Lanes<6>;
            let v9465: Lanes<6>;
            let v9466: Lanes<6>;
            let v9467: Lanes<6>;
            if v9 != 0.0 {
                let v1209 = v766 + v1;
                let v1210 = v1 / v759;
                let v11925 = ((v10496 * v1210) * v10390) / v759;
                let v1212 = v1210 / v1211;
                let v11929 = ((Lanes([0.0, 0.0, v11925[0], 0.0, 0.0])) - (v9422 * v1212)) / v1211;
                let v1213 = v1212 * v1209;
                let v11931 = v9402 * v1212;
                let v1214 = v1213 * v1209;
                let v11935 = v9402 * v1213;
                let v1215 = v78 / v1209;
                let v1216 = v663 + v1215;
                let v1218 = (v1214.ln()) / v1216;
                let v11944 = (v10410 + (((v9402 * v1215) * v10390) / v1209)) * v1218;
                let v1220 = (v764 * v1218).sqrt();
                let v11951 = ((((((((v11929 * v1209) + (Lanes([0.0, 0.0, v11931[0], 0.0, 0.0]))) * v1209) + (Lanes([0.0, 0.0, v11935[0], 0.0, 0.0]))) * (v9368 / v1214)) - (Lanes([0.0, 0.0, v11944[0], 0.0, 0.0]))) / v1216) * v764) * (v9368 / (v10435 * v1220));
                let v1221 = if v1220 > v12 { 1.0 } else { 0.0 };
                let v1222: f64;
                let v9468: Lanes<5>;
                if v1221 != 0.0 {
                    v1222 = v12;
                    v9468 = v10579;
                } else {
                    v1222 = v1220;
                    v9468 = v11951;
                }
                let v1224 = v1223 * v477;
                let v1225 = v1224 * v1222;
                let v11952 = v9468 * v1224;
                let v1228 = (v1226 * v477) * v12;
                let v1229 = -v1228;
                let v1230 = v1229 * v529;
                let v1232 = v1229 * v1231;
                let v1244: f64;
                let v9469: Lanes<4>;
                if v1233 != 0.0 {
                    let v1234 = v864 + v1203;
                    let v11958 = (Lanes([v10558[0], v10558[1], 0.0, v10558[2]])) + (Lanes([0.0, 0.0, v10829[0], 0.0]));
                    v1244 = v1234;
                    v9469 = v11958;
                } else {
                    let v1235 = v835 + v1203;
                    let v11955 = (Lanes([v9413[0], v9413[1], 0.0, v9413[2]])) + (Lanes([0.0, 0.0, v10829[0], 0.0]));
                    v1244 = v1235;
                    v9469 = v11955;
                }
                let v1239 = (v78 / v663) * ((v33 / v731).ln());
                let v11959 = v9404 * v1240;
                let v1243 = ((v1240 * v1240) * v132) * v132;
                let v11962 = ((v11959 + v11959) * v132) * v132;
                let v1245 = -v1244;
                let v11963 = v9469 * v10390;
                let v1247 = v1243 * v663;
                let v11967 = (v11962 * v663) + (v10410 * v1243);
                let v1248 = (v78 * v1245) + v1247;
                let v11969 = (v11963 * v78) + (Lanes([0.0, 0.0, v11967[0], 0.0]));
                let v1250 = v1245 * v1245;
                let v11970 = v11963 * v1245;
                let v11971 = v11970 + v11970;
                let v11974 = (v11971 + (Lanes([0.0, 0.0, v11962[0], 0.0]))) * v90;
                let v1253 = (v1248 * v1248) - (v90 * (v1250 + v1243));
                let v1255 = if v1253 >= v1254 { 1.0 } else { 0.0 };
                let v1257: f64;
                if v1255 != 0.0 {
                    v1257 = v1253;
                } else {
                    v1257 = v1256;
                }
                let v1260 = (v1248 - (v1257.sqrt())) / v78;
                let v1261 = v1250 / v1243;
                let v11975 = v11962 * v1261;
                let v11978 = (v11971 - (Lanes([0.0, 0.0, v11975[0], 0.0]))) / v1243;
                let v1263 = v1261 / v1262;
                let v11979 = v9405 * v1263;
                let v11980 = Lanes([0.0, 0.0, v11979[0], 0.0]);
                let v11981 = v9368 / v1263;
                let v1265 = v78 / v1245;
                let v1266 = v663 + v1265;
                let v1267 = (v1263.ln()) / v1266;
                let v11987 = ((Lanes([0.0, 0.0, v10410[0], 0.0])) + (((v11963 * v1265) * v10390) / v1245)) * v1267;
                let v1268 = if v1260 < v1239 { 1.0 } else { 0.0 };
                let v1384: f64;
                if v1268 != 0.0 {
                    v1384 = v1260;
                } else {
                    let v1271 = (v1267 - v1260) - v1270;
                    let v1273 = (v90 * v1267) * v1270;
                    let v1274 = if v1273 > v0 { 1.0 } else { 0.0 };
                    let v1276: f64;
                    if v1274 != 0.0 {
                        v1276 = v1273;
                    } else {
                        let v1275 = -v1273;
                        v1276 = v1275;
                    }
                    let v1282 = v1267 - (v13 * (v1271 + (((v1271 * v1271) + v1276).sqrt())));
                    v1384 = v1282;
                }
                let mut v1283: f64 = 0.0;
                let mut v1285: f64 = 0.0;
                let mut v1385: f64 = 0.0;
                let mut v1509: f64 = 0.0;
                v1283 = v0;
                v1285 = v1384;
                v1385 = v0;
                v1509 = v0;
                loop {
                    let v1284 = if v1283 < v18 { 1.0 } else { 0.0 };
                    if v1284 == 0.0 {
                        break;
                    }
                    let v1286 = v663 * v1285;
                    let v1288 = (-v1286).exp();
                    let v1289 = if v1285 > v616 { 1.0 } else { 0.0 };
                    let v1323: f64;
                    let v1356: f64;
                    if v1289 != 0.0 {
                        let v1290 = v1286.exp();
                        let v1298 = (-v1240) * ((((v1288 + v1286) - v1) + (v1262 * (v1290 - v1))).sqrt());
                        let v1304 = (v211 / v1298) * (((-v1288) + v1) + (v1262 * v1290));
                        v1323 = v1298;
                        v1356 = v1304;
                    } else {
                        let v1306 = if v1285 < v1305 { 1.0 } else { 0.0 };
                        let v1324: f64;
                        let v1357: f64;
                        if v1306 != 0.0 {
                            let v1310 = v1240 * (((v1288 + v1286) - v1).sqrt());
                            let v1314 = (v211 / v1310) * ((-v1288) + v1);
                            v1324 = v1310;
                            v1357 = v1314;
                        } else {
                            let v1319 = ((-((v211 / v663).sqrt())) * v663) * v1285;
                            let v1322 = -((v211 * v663).sqrt());
                            v1324 = v1319;
                            v1357 = v1322;
                        }
                        v1323 = v1324;
                        v1356 = v1357;
                    }
                    let v1329 = ((v1323 * v1323) + ((v90 * v1230) * v1230)).sqrt();
                    let v1332 = v13 * (v1 + (v1323 / v1329));
                    let v1336 = (v13 * (v1323 + v1329)) + (v535 * v1230);
                    let v1337 = if v1336 < v0 { 1.0 } else { 0.0 };
                    let v1338: f64;
                    let v1355: f64;
                    if v1337 != 0.0 {
                        v1338 = v0;
                        v1355 = v0;
                    } else {
                        v1338 = v1336;
                        v1355 = v1332;
                    }
                    let v1340 = (v1229 - v1338) - v1232;
                    let v1342 = (v90 * v1229) * v1232;
                    let v1343 = if v1342 > v0 { 1.0 } else { 0.0 };
                    let v1345: f64;
                    if v1343 != 0.0 {
                        v1345 = v1342;
                    } else {
                        let v1344 = -v1342;
                        v1345 = v1344;
                    }
                    let v1348 = ((v1340 * v1340) + v1345).sqrt();
                    let v1354 = v1229 - (v13 * (v1340 + v1348));
                    let v1364 = ((((v1354 * v1354) / v78) / v123) / v206) / v477;
                    let v1378 = v1285 - (((((-v1285) + (v1323 / v130)) - v1244) + v1364) / ((v1373 + (v1356 / v130)) + (((v78 * v1364) * (v1355 * (v1356 * (v13 * (v1 + (v1340 / v1348)))))) / v1354)));
                    let v1381 = if ((v1378 - v1285).abs()) < v861 { 1.0 } else { 0.0 };
                    let v1382: f64;
                    if v1381 != 0.0 {
                        v1382 = v18;
                    } else {
                        v1382 = v1283;
                    }
                    let v1383 = v1382 + v1;
                    v1283 = v1383;
                    v1285 = v1378;
                    v1385 = v1364;
                    v1509 = v1323;
                }
                let v1392 = if (((v1386 * v1385) / v477).sqrt()) > (v1390 * v12) { 1.0 } else { 0.0 };
                let v1574: f64;
                let v1888: f64;
                let v9470: Lanes<5>;
                if v1392 != 0.0 {
                    let v1393 = v1 / v1128;
                    let v11990 = ((v9421 * v1393) * v10390) / v1128;
                    let v1394 = v12 / v123;
                    let v1395 = v1 / v130;
                    let v1397 = (v1393 + v1394) + v1395;
                    let v1398 = v1 / v1397;
                    let v11991 = v11990 * v1398;
                    let v11993 = (v11991 * v10390) / v1397;
                    let v1400 = v1 - (v1398 * v1393);
                    let v1404 = v1245 + ((v1395 + (v13 * v1394)) * v1229);
                    let v1405 = v1398 * v1404;
                    let v11997 = v11993 * v1404;
                    let v11998 = v11963 * v1398;
                    let v12002 = v11990 * v1405;
                    let v1407 = (v1393 * v1405) / v1400;
                    let v12006 = (((v11993 * v1393) + v11991) * v10390) * v1407;
                    let v12009 = (((Lanes([v12002[0], v12002[1], 0.0, v12002[2], v12002[3]])) + (((Lanes([v11997[0], v11997[1], 0.0, v11997[2], v11997[3]])) + (Lanes([v11998[0], v11998[1], v11998[2], 0.0, v11998[3]]))) * v1393)) - (Lanes([v12006[0], v12006[1], 0.0, v12006[2], v12006[3]]))) / v1400;
                    let v1408 = v1205 + v1407;
                    v1574 = v1407;
                    v1888 = v1408;
                    v9470 = v12009;
                } else {
                    v1574 = v0;
                    v1888 = v1205;
                    v9470 = v10579;
                }
                let v1409 = v839 / v79;
                let v12010 = v10537 / v79;
                let v1417 = v1414 + (v1409 * v1415);
                let v1419 = v1413 + (v1409 * v1417);
                let v1421 = v1412 + (v1409 * v1419);
                let v1423 = v1411 + (v1409 * v1421);
                let v1425 = v1410 + (v1409 * v1423);
                let v1427 = v1 + (v1409 * v1425);
                let v1428 = v79 / v1427;
                let v12029 = ((((v12010 * v1425) + (((v12010 * v1423) + (((v12010 * v1421) + (((v12010 * v1419) + (((v12010 * v1417) + ((v12010 * v1415) * v1409)) * v1409)) * v1409)) * v1409)) * v1409)) * v1428) * v10390) / v1427;
                let v1429 = if v1428 < v861 { 1.0 } else { 0.0 };
                let v1430: f64;
                let v9471: Lanes<3>;
                if v1429 != 0.0 {
                    v1430 = v861;
                    v9471 = v10504;
                } else {
                    v1430 = v1428;
                    v9471 = v12029;
                }
                let v12031 = v10562 + (Lanes([v9471[0], v9471[1], 0.0, v9471[2]]));
                let v1434 = (((v830 + v1430) - v240) + v1142) - v1199;
                let v1435 = v721 * v766;
                let v1436 = v1222 / v1435;
                let v12036 = (v9402 * v721) * v1436;
                let v1437 = v1436 * v1434;
                let v12042 = (((v9468 - (Lanes([0.0, 0.0, v12036[0], 0.0, 0.0]))) / v1435) * v1434) + ((((Lanes([v12031[0], v12031[1], 0.0, v12031[2], v12031[3]])) + v10791) - v10827) * v1436);
                let v1438 = v12 * v1208;
                let v1441 = if (if v1437 < v1438 { 1.0 } else { 0.0 }) != 0.0 && (if v1438 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1469: f64;
                let v9472: Lanes<5>;
                if v1441 != 0.0 {
                    let v1442 = v1438 - v1437;
                    let v12043 = v12042 * v10390;
                    let v1443 = v1442 * v1442;
                    let v12044 = v12043 * v1442;
                    let v1444 = v1438 * v1438;
                    let v12046 = (v12044 + v12044) * v1443;
                    let v12047 = v12046 + v12046;
                    let v1447 = (v1443 * v1443) + (v1444 * v1444);
                    let v1464: f64;
                    let v9473: Lanes<5>;
                    if v1448 != 0.0 {
                        let v1458: f64;
                        if v1449 != 0.0 {
                            v1458 = v1;
                        } else {
                            let v1459: f64;
                            if v1450 != 0.0 {
                                v1459 = v78;
                            } else {
                                let v1460: f64;
                                if v1451 != 0.0 {
                                    v1460 = v96;
                                } else {
                                    let v1461: f64;
                                    if v1452 != 0.0 {
                                        v1461 = v90;
                                    } else {
                                        v1461 = v0;
                                    }
                                    v1460 = v1461;
                                }
                                v1459 = v1460;
                            }
                            v1458 = v1459;
                        }
                        let mut v1453: f64 = 0.0;
                        let mut v1455: f64 = 0.0;
                        let mut v9474: Lanes<5> = Lanes([0.0; 5]);
                        v1453 = v0;
                        v1455 = v1447;
                        v9474 = v12047;
                        loop {
                            let v1454 = if v1453 < v1458 { 1.0 } else { 0.0 };
                            if v1454 == 0.0 {
                                break;
                            }
                            let v1456 = v1455.sqrt();
                            let v19268 = v9474 * (v9368 / (v10435 * v1456));
                            let v1457 = v1453 + v1;
                            v1453 = v1457;
                            v1455 = v1456;
                            v9474 = v19268;
                        }
                        v1464 = v1455;
                        v9473 = v9474;
                    } else {
                        let v1463 = v1447.powf(v1462);
                        let v12051 = v12047 * (v1462 * (v1447.powf(v12048)));
                        v1464 = v1463;
                        v9473 = v12051;
                    }
                    let v1465 = v1 / v1464;
                    let v1466 = v1442 * v1438;
                    let v1468 = v1438 - (v1466 * v1465);
                    let v12059 = (((v12043 * v1438) * v1465) + ((((v9473 * v1465) * v10390) / v1464) * v1466)) * v10390;
                    v1469 = v1468;
                    v9472 = v12059;
                } else {
                    v1469 = v1437;
                    v9472 = v12042;
                }
                let v1470 = v1222 - v12;
                let v1473 = if (if v1469 > v1470 { 1.0 } else { 0.0 }) != 0.0 && (if v12 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1502: f64;
                let v9475: Lanes<5>;
                if v1473 != 0.0 {
                    let v12060 = v9472 - v9468;
                    let v1475 = (v1469 - v1222) + v12;
                    let v1476 = v1475 * v1475;
                    let v12061 = v12060 * v1475;
                    let v1477 = v12 * v12;
                    let v12063 = (v12061 + v12061) * v1476;
                    let v12064 = v12063 + v12063;
                    let v1480 = (v1476 * v1476) + (v1477 * v1477);
                    let v1497: f64;
                    let v9476: Lanes<5>;
                    if v1481 != 0.0 {
                        let v1491: f64;
                        if v1482 != 0.0 {
                            v1491 = v1;
                        } else {
                            let v1492: f64;
                            if v1483 != 0.0 {
                                v1492 = v78;
                            } else {
                                let v1493: f64;
                                if v1484 != 0.0 {
                                    v1493 = v96;
                                } else {
                                    let v1494: f64;
                                    if v1485 != 0.0 {
                                        v1494 = v90;
                                    } else {
                                        v1494 = v0;
                                    }
                                    v1493 = v1494;
                                }
                                v1492 = v1493;
                            }
                            v1491 = v1492;
                        }
                        let mut v1486: f64 = 0.0;
                        let mut v1488: f64 = 0.0;
                        let mut v9477: Lanes<5> = Lanes([0.0; 5]);
                        v1486 = v0;
                        v1488 = v1480;
                        v9477 = v12064;
                        loop {
                            let v1487 = if v1486 < v1491 { 1.0 } else { 0.0 };
                            if v1487 == 0.0 {
                                break;
                            }
                            let v1489 = v1488.sqrt();
                            let v19265 = v9477 * (v9368 / (v10435 * v1489));
                            let v1490 = v1486 + v1;
                            v1486 = v1490;
                            v1488 = v1489;
                            v9477 = v19265;
                        }
                        v1497 = v1488;
                        v9476 = v9477;
                    } else {
                        let v1496 = v1480.powf(v1495);
                        let v12068 = v12064 * (v1495 * (v1480.powf(v12065)));
                        v1497 = v1496;
                        v9476 = v12068;
                    }
                    let v1498 = v1 / v1497;
                    let v1499 = v1475 * v12;
                    let v1501 = v1470 + (v1499 * v1498);
                    let v12076 = v9468 + (((v12060 * v12) * v1498) + ((((v9476 * v1498) * v10390) / v1497) * v1499));
                    v1502 = v1501;
                    v9475 = v12076;
                } else {
                    v1502 = v1469;
                    v9475 = v9472;
                }
                let v1504 = (-v1502) * v490;
                let v12078 = (v9475 * v10390) * v490;
                let v1512 = ((((v1229 * v12) / v78) / v123) + v665) - ((v1509 * v12) / v123);
                let v2258: f64;
                let v2259: f64;
                let v2260: f64;
                let v2585: f64;
                let v2600: f64;
                let v2678: f64;
                let v3331: f64;
                let v5094: f64;
                let v9478: Lanes<5>;
                let v9479: Lanes<5>;
                let v9480: Lanes<5>;
                let v9481: Lanes<5>;
                let v9482: Lanes<5>;
                let v9483: Lanes<5>;
                if v1513 != 0.0 {
                    let v1514 = if v0 < v1512 { 1.0 } else { 0.0 };
                    let v1515: f64;
                    if v1514 != 0.0 {
                        v1515 = v1;
                    } else {
                        v1515 = v78;
                    }
                    v2258 = v0;
                    v2259 = v0;
                    v2260 = v0;
                    v2585 = v1515;
                    v2600 = v0;
                    v2678 = v0;
                    v3331 = v0;
                    v5094 = v0;
                    v9478 = v10579;
                    v9479 = v10579;
                    v9480 = v10579;
                    v9481 = v10579;
                    v9482 = v10579;
                    v9483 = v10579;
                } else {
                    let v1521 = v1 + ((v90 * ((v663 * v1200) - v1)) / (v1207 * v664));
                    let v1523 = if v1521 >= v1522 { 1.0 } else { 0.0 };
                    let v1525: f64;
                    if v1523 != 0.0 {
                        v1525 = v1521;
                    } else {
                        v1525 = v1524;
                    }
                    let v1531 = v1200 + (((v1207 * v663) * v13) * (v1 - (v1525.sqrt())));
                    let v1533 = if (v663 * v1531) < v96 { 1.0 } else { 0.0 };
                    let v1612: f64;
                    if v1533 != 0.0 {
                        let v1539 = v1 / ((v1536 * v663) * v1206);
                        let v1542 = v1540 + (v96 * v1539);
                        let v1547 = (v1153 * v1539) * (v663 * (v1200 - v835));
                        let v1554 = (v1549 - (v1540 * (v1550 + v1539))) + v1547;
                        let v1563 = (((v1543 - (v1540 * v1539)) + v1547) + (((((v90 * v1542) * v1542) * v1542) + (v1554 * v1554)).sqrt())).powf(v1562);
                        let v1573 = (((v96 - ((v1564 * v1542) / (v96 * v1563))) + (v1569 * v1563)) * v665) + v835;
                        v1612 = v1573;
                    } else {
                        let v1576 = if (v830 - v1574) <= v1143 { 1.0 } else { 0.0 };
                        let v1613: f64;
                        if v1576 != 0.0 {
                            let v1578 = v12 / v123;
                            let v1579 = v1 / v130;
                            let v1591 = v1200 - (((v1 / (((v1 / v1128) + v1578) + v1579)) * ((v1200 - v1244) + ((v1579 + (v13 * v1578)) * (-v1504)))) / v1128);
                            v1613 = v1591;
                        } else {
                            let v1592 = v1200 - v1574;
                            let v1598 = (((v1212 * v1592) * v1592).ln()) / (v663 + (v78 / v1592));
                            let v1600 = (v1598 - v1531) - v1270;
                            let v1602 = (v90 * v1598) * v1270;
                            let v1603 = if v1602 > v0 { 1.0 } else { 0.0 };
                            let v1605: f64;
                            if v1603 != 0.0 {
                                v1605 = v1602;
                            } else {
                                let v1604 = -v1602;
                                v1605 = v1604;
                            }
                            let v1611 = v1598 - (v13 * (v1600 + (((v1600 * v1600) + v1605).sqrt())));
                            v1613 = v1611;
                        }
                        v1612 = v1613;
                    }
                    let v1614 = if v1612 > v0 { 1.0 } else { 0.0 };
                    let v1619: f64;
                    if v1614 != 0.0 {
                        let v1618 = ((v1615 * v1612) / v477).sqrt();
                        v1619 = v1618;
                    } else {
                        v1619 = v0;
                    }
                    let v1620 = if v1619 < v12 { 1.0 } else { 0.0 };
                    let v2586: f64;
                    if v1620 != 0.0 {
                        v2586 = v1;
                    } else {
                        v2586 = v78;
                    }
                    let v1622 = if (v830 - v1574) <= v1143 { 1.0 } else { 0.0 };
                    let v1694: f64;
                    let v1697: f64;
                    let v9484: Lanes<5>;
                    let v9485: Lanes<5>;
                    if v1622 != 0.0 {
                        let v1623 = v1 / v1128;
                        let v1624 = v12 / v123;
                        let v1625 = v1 / v130;
                        let v1627 = (v1623 + v1624) + v1625;
                        let v1628 = v1 / v1627;
                        let v1631 = v1625 + (v13 * v1624);
                        let v1634 = (v1200 - v1244) + (v1631 * (-v1504));
                        let v12148 = ((((((v9421 * v1623) * v10390) / v1128) * v1628) * v10390) / v1627) * v1634;
                        let v1636 = (v1628 * v1634) / v1128;
                        let v12152 = v9421 * v1636;
                        let v1637 = v1200 - v1636;
                        let v12156 = v10828 - ((((Lanes([v12148[0], v12148[1], 0.0, v12148[2], v12148[3]])) + (((v10828 - (Lanes([v9469[0], v9469[1], v9469[2], 0.0, v9469[3]]))) + ((v12078 * v10390) * v1631)) * v1628)) - (Lanes([v12152[0], v12152[1], 0.0, v12152[2], v12152[3]]))) / v1128);
                        v1694 = v1637;
                        v1697 = v1637;
                        v9484 = v12156;
                        v9485 = v12156;
                    } else {
                        let v1638 = v1 / v1128;
                        let v1639 = v12 / v123;
                        let v1640 = v1 / v130;
                        let v1642 = (v1638 + v1639) + v1640;
                        let v1643 = v1 / v1642;
                        let v1646 = v1640 + (v13 * v1639);
                        let v1649 = (v1200 - v1244) + (v1646 * (-v1504));
                        let v12090 = ((((((v9421 * v1638) * v10390) / v1128) * v1643) * v10390) / v1642) * v1649;
                        let v1651 = (v1643 * v1649) / v1128;
                        let v12094 = v9421 * v1651;
                        let v1652 = v1200 - v1651;
                        let v12098 = v10828 - ((((Lanes([v12090[0], v12090[1], 0.0, v12090[2], v12090[3]])) + (((v10828 - (Lanes([v9469[0], v9469[1], v9469[2], 0.0, v9469[3]]))) + ((v12078 * v10390) * v1646)) * v1643)) - (Lanes([v12094[0], v12094[1], 0.0, v12094[2], v12094[3]]))) / v1128);
                        let v1653 = v1200 - v1574;
                        let v12099 = v10828 - v9470;
                        let v1654 = if v1653 > v0 { 1.0 } else { 0.0 };
                        let v1695: f64;
                        let v9486: Lanes<5>;
                        if v1654 != 0.0 {
                            let v1655 = v1212 * v1653;
                            let v1656 = v1655 * v1653;
                            let v1657 = v78 / v1653;
                            let v1658 = v663 + v1657;
                            let v1660 = (v1656.ln()) / v1658;
                            let v1662 = v1660 * v1661;
                            let v12116 = (((((((v11929 * v1653) + (v12099 * v1212)) * v1653) + (v12099 * v1655)) * (v9368 / v1656)) - (((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + (((v12099 * v1657) * v10390) / v1653)) * v1660)) / v1658) * v1661;
                            let v1663 = v1662 - v708;
                            let v1666 = if (if v1652 > v1663 { 1.0 } else { 0.0 }) != 0.0 && v1665 != 0.0 { 1.0 } else { 0.0 };
                            let v1696: f64;
                            let v9487: Lanes<5>;
                            if v1666 != 0.0 {
                                let v12117 = v12098 - v12116;
                                let v1668 = (v1652 - v1662) + v708;
                                let v1669 = v1668 * v1668;
                                let v12118 = v12117 * v1668;
                                let v12120 = (v12118 + v12118) * v1669;
                                let v12121 = v12120 + v12120;
                                let v1672 = (v1669 * v1669) + v1671;
                                let v1689: f64;
                                let v9488: Lanes<5>;
                                if v1673 != 0.0 {
                                    let v1683: f64;
                                    if v1674 != 0.0 {
                                        v1683 = v1;
                                    } else {
                                        let v1684: f64;
                                        if v1675 != 0.0 {
                                            v1684 = v78;
                                        } else {
                                            let v1685: f64;
                                            if v1676 != 0.0 {
                                                v1685 = v96;
                                            } else {
                                                let v1686: f64;
                                                if v1677 != 0.0 {
                                                    v1686 = v90;
                                                } else {
                                                    v1686 = v0;
                                                }
                                                v1685 = v1686;
                                            }
                                            v1684 = v1685;
                                        }
                                        v1683 = v1684;
                                    }
                                    let mut v1678: f64 = 0.0;
                                    let mut v1680: f64 = 0.0;
                                    let mut v9489: Lanes<5> = Lanes([0.0; 5]);
                                    v1678 = v0;
                                    v1680 = v1672;
                                    v9489 = v12121;
                                    loop {
                                        let v1679 = if v1678 < v1683 { 1.0 } else { 0.0 };
                                        if v1679 == 0.0 {
                                            break;
                                        }
                                        let v1681 = v1680.sqrt();
                                        let v12136 = v9489 * (v9368 / (v10435 * v1681));
                                        let v1682 = v1678 + v1;
                                        v1678 = v1682;
                                        v1680 = v1681;
                                        v9489 = v12136;
                                    }
                                    v1689 = v1680;
                                    v9488 = v9489;
                                } else {
                                    let v1688 = v1672.powf(v1687);
                                    let v12125 = v12121 * (v1687 * (v1672.powf(v12122)));
                                    v1689 = v1688;
                                    v9488 = v12125;
                                }
                                let v1690 = v1 / v1689;
                                let v1691 = v1668 * v708;
                                let v1693 = v1663 + (v1691 * v1690);
                                let v12133 = v12116 + (((v12117 * v708) * v1690) + ((((v9488 * v1690) * v10390) / v1689) * v1691));
                                v1696 = v1693;
                                v9487 = v12133;
                            } else {
                                v1696 = v1652;
                                v9487 = v12098;
                            }
                            v1695 = v1696;
                            v9486 = v9487;
                        } else {
                            v1695 = v1652;
                            v9486 = v12098;
                        }
                        v1694 = v1695;
                        v1697 = v1652;
                        v9484 = v9486;
                        v9485 = v12098;
                    }
                    let v1698 = v13 * v1228;
                    let v1701 = (v1694 + (v1698 * v125)) - v1244;
                    let v12157 = Lanes([v9469[0], v9469[1], v9469[2], 0.0, v9469[3]]);
                    let v12158 = v9484 - v12157;
                    let v1702 = if v1701 < v0 { 1.0 } else { 0.0 };
                    let v1879: f64;
                    let v9490: Lanes<5>;
                    if v1702 != 0.0 {
                        let v1703 = v1240 * v132;
                        let v1704 = v1703 * v1703;
                        let v12209 = (v9404 * v132) * v1703;
                        let v12210 = v12209 + v12209;
                        let v12211 = v12158 * v1705;
                        let v1708 = (v1705 * v1701) + v1707;
                        let v1710 = v1708 * v529;
                        let v12212 = v12211 * v529;
                        let v1711 = (v1708 - v13) - v1710;
                        let v12213 = v12211 - v12212;
                        let v1712 = v90 * v1708;
                        let v1713 = v1712 * v1710;
                        let v12217 = ((v12211 * v90) * v1710) + (v12212 * v1712);
                        let v1714 = if v1713 > v0 { 1.0 } else { 0.0 };
                        let v1716: f64;
                        let v9491: Lanes<5>;
                        if v1714 != 0.0 {
                            v1716 = v1713;
                            v9491 = v12217;
                        } else {
                            let v1715 = -v1713;
                            let v12218 = v12217 * v10390;
                            v1716 = v1715;
                            v9491 = v12218;
                        }
                        let v12219 = v12213 * v1711;
                        let v1719 = ((v1711 * v1711) + v1716).sqrt();
                        let v1722 = v1708 - (v13 * (v1711 + v1719));
                        let v1723 = v1704 * v1722;
                        let v12228 = v12210 * v1722;
                        let v1724 = v1723 * v664;
                        let v12233 = v10412 * v1723;
                        let v12235 = (((Lanes([0.0, 0.0, v12228[0], 0.0, 0.0])) + ((v12211 - ((v12213 + (((v12219 + v12219) + v9491) * (v9368 / (v10435 * v1719)))) * v13)) * v1704)) * v664) + (Lanes([0.0, 0.0, v12233[0], 0.0, 0.0]));
                        let v1725 = v1724.sqrt();
                        let v1726 = v1 - v1725;
                        let v1728 = v1 - v1724;
                        let v1729 = (v1701 * v1726) / v1728;
                        let v12246 = (((v12158 * v1726) + (((v12235 * (v9368 / (v10435 * v1725))) * v10390) * v1701)) - ((v12235 * v10390) * v1729)) / v1728;
                        v1879 = v1729;
                        v9490 = v12246;
                    } else {
                        let v1735 = -((v1244 - v1694) - (((v1228 / v78) * v12) / v123));
                        let v12160 = (v12157 - v9484) * v10390;
                        let v1737 = (v78 * v1735) + v1247;
                        let v12163 = (v12160 * v78) + (Lanes([0.0, 0.0, v11967[0], 0.0, 0.0]));
                        let v12164 = v12163 * v1737;
                        let v1739 = v1735 * v1735;
                        let v12166 = v12160 * v1735;
                        let v12167 = v12166 + v12166;
                        let v1742 = (v1737 * v1737) - (v90 * (v1739 + v1243));
                        let v12171 = (v12164 + v12164) - ((v12167 + (Lanes([0.0, 0.0, v11962[0], 0.0, 0.0]))) * v90);
                        let v1744 = if v1742 >= v1743 { 1.0 } else { 0.0 };
                        let v1746: f64;
                        let v9492: Lanes<5>;
                        if v1744 != 0.0 {
                            v1746 = v1742;
                            v9492 = v12171;
                        } else {
                            v1746 = v1745;
                            v9492 = v10579;
                        }
                        let v1747 = v1746.sqrt();
                        let v1749 = (v1737 - v1747) / v78;
                        let v12176 = (v12163 - (v9492 * (v9368 / (v10435 * v1747)))) / v78;
                        let v1750 = v1739 / v1243;
                        let v12177 = v11962 * v1750;
                        let v1751 = v1750 / v1262;
                        let v12181 = v9405 * v1751;
                        let v1753 = v78 / v1735;
                        let v1754 = v663 + v1753;
                        let v1755 = (v1751.ln()) / v1754;
                        let v12194 = ((((((v12167 - (Lanes([0.0, 0.0, v12177[0], 0.0, 0.0]))) / v1243) - (Lanes([0.0, 0.0, v12181[0], 0.0, 0.0]))) / v1262) * (v9368 / v1751)) - (((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + (((v12160 * v1753) * v10390) / v1735)) * v1755)) / v1754;
                        let v1756 = if v1749 < v1239 { 1.0 } else { 0.0 };
                        let v1880: f64;
                        let v9493: Lanes<5>;
                        if v1756 != 0.0 {
                            v1880 = v1749;
                            v9493 = v12176;
                        } else {
                            let v12195 = v12194 - v12176;
                            let v1758 = (v1755 - v1749) - v1270;
                            let v1760 = (v90 * v1755) * v1270;
                            let v12197 = (v12194 * v90) * v1270;
                            let v1761 = if v1760 > v0 { 1.0 } else { 0.0 };
                            let v1763: f64;
                            let v9494: Lanes<5>;
                            if v1761 != 0.0 {
                                v1763 = v1760;
                                v9494 = v12197;
                            } else {
                                let v1762 = -v1760;
                                let v12198 = v12197 * v10390;
                                v1763 = v1762;
                                v9494 = v12198;
                            }
                            let v12199 = v12195 * v1758;
                            let v1766 = ((v1758 * v1758) + v1763).sqrt();
                            let v1769 = v1755 - (v13 * (v1758 + v1766));
                            let v12207 = v12194 - ((v12195 + (((v12199 + v12199) + v9494) * (v9368 / (v10435 * v1766)))) * v13);
                            v1880 = v1769;
                            v9493 = v12207;
                        }
                        v1879 = v1880;
                        v9490 = v9493;
                    }
                    let mut v1770: f64 = 0.0;
                    let mut v1772: f64 = 0.0;
                    let mut v1882: f64 = 0.0;
                    let mut v9495: Lanes<5> = Lanes([0.0; 5]);
                    let mut v9496: Lanes<5> = Lanes([0.0; 5]);
                    v1770 = v0;
                    v1772 = v1879;
                    v1882 = v0;
                    v9495 = v9490;
                    v9496 = v10579;
                    loop {
                        let v1771 = if v1770 < v18 { 1.0 } else { 0.0 };
                        if v1771 == 0.0 {
                            break;
                        }
                        let v1773 = v663 * v1772;
                        let v12250 = v10410 * v1772;
                        let v12253 = (Lanes([0.0, 0.0, v12250[0], 0.0, 0.0])) + (v9495 * v663);
                        let v1775 = (-v1773).exp();
                        let v12255 = (v12253 * v10390) * v1775;
                        let v1776 = if v1772 > v616 { 1.0 } else { 0.0 };
                        let v1810: f64;
                        let v1843: f64;
                        let v9497: Lanes<5>;
                        let v9498: Lanes<5>;
                        if v1776 != 0.0 {
                            let v1777 = v1773.exp();
                            let v1778 = -v1240;
                            let v1781 = v1777 - v1;
                            let v12294 = v9405 * v1781;
                            let v12295 = (v12253 * v1777) * v1262;
                            let v1784 = (((v1775 + v1773) - v1) + (v1262 * v1781)).sqrt();
                            let v1785 = v1778 * v1784;
                            let v12302 = (v9404 * v10390) * v1784;
                            let v12305 = (Lanes([0.0, 0.0, v12302[0], 0.0, 0.0])) + ((((v12255 + v12253) + ((Lanes([0.0, 0.0, v12294[0], 0.0, 0.0])) + v12295)) * (v9368 / (v10435 * v1784))) * v1778);
                            let v1786 = v211 / v1785;
                            let v12310 = v9405 * v1777;
                            let v1790 = ((-v1775) + v1) + (v1262 * v1777);
                            let v1791 = v1786 * v1790;
                            let v12316 = ((((v12305 * v1786) * v10390) / v1785) * v1790) + (((v12255 * v10390) + ((Lanes([0.0, 0.0, v12310[0], 0.0, 0.0])) + v12295)) * v1786);
                            v1810 = v1785;
                            v1843 = v1791;
                            v9497 = v12305;
                            v9498 = v12316;
                        } else {
                            let v1793 = if v1772 < v1792 { 1.0 } else { 0.0 };
                            let v1811: f64;
                            let v1844: f64;
                            let v9499: Lanes<5>;
                            let v9500: Lanes<5>;
                            if v1793 != 0.0 {
                                let v1796 = ((v1775 + v1773) - v1).sqrt();
                                let v1797 = v1240 * v1796;
                                let v12280 = v9404 * v1796;
                                let v12283 = (Lanes([0.0, 0.0, v12280[0], 0.0, 0.0])) + (((v12255 + v12253) * (v9368 / (v10435 * v1796))) * v1240);
                                let v1798 = v211 / v1797;
                                let v1800 = (-v1775) + v1;
                                let v1801 = v1798 * v1800;
                                let v12290 = ((((v12283 * v1798) * v10390) / v1797) * v1800) + ((v12255 * v10390) * v1798);
                                v1811 = v1797;
                                v1844 = v1801;
                                v9499 = v12283;
                                v9500 = v12290;
                            } else {
                                let v1802 = v211 / v663;
                                let v1803 = v1802.sqrt();
                                let v1804 = -v1803;
                                let v1805 = v1804 * v663;
                                let v1806 = v1805 * v1772;
                                let v12266 = (((((((v10410 * v1802) * v10390) / v663) * (v9368 / (v10435 * v1803))) * v10390) * v663) + (v10410 * v1804)) * v1772;
                                let v12269 = (Lanes([0.0, 0.0, v12266[0], 0.0, 0.0])) + (v9495 * v1805);
                                let v1808 = (v211 * v663).sqrt();
                                let v1809 = -v1808;
                                let v12274 = ((v10410 * v211) * (v9368 / (v10435 * v1808))) * v10390;
                                let v12275 = Lanes([0.0, 0.0, v12274[0], 0.0, 0.0]);
                                v1811 = v1806;
                                v1844 = v1809;
                                v9499 = v12269;
                                v9500 = v12275;
                            }
                            v1810 = v1811;
                            v1843 = v1844;
                            v9497 = v9499;
                            v9498 = v9500;
                        }
                        let v12317 = v9497 * v1810;
                        let v1816 = ((v1810 * v1810) + ((v90 * v1230) * v1230)).sqrt();
                        let v12321 = (v12317 + v12317) * (v9368 / (v10435 * v1816));
                        let v1817 = v1810 / v1816;
                        let v1819 = v13 * (v1 + v1817);
                        let v12325 = ((v9497 - (v12321 * v1817)) / v1816) * v13;
                        let v12327 = (v9497 + v12321) * v13;
                        let v1823 = (v13 * (v1810 + v1816)) + (v535 * v1230);
                        let v1824 = if v1823 < v0 { 1.0 } else { 0.0 };
                        let v1825: f64;
                        let v1842: f64;
                        let v9501: Lanes<5>;
                        let v9502: Lanes<5>;
                        if v1824 != 0.0 {
                            v1825 = v0;
                            v1842 = v0;
                            v9501 = v10579;
                            v9502 = v10579;
                        } else {
                            v1825 = v1823;
                            v1842 = v1819;
                            v9501 = v12327;
                            v9502 = v12325;
                        }
                        let v12328 = v9501 * v10390;
                        let v1827 = (v1229 - v1825) - v1232;
                        let v1829 = (v90 * v1229) * v1232;
                        let v1830 = if v1829 > v0 { 1.0 } else { 0.0 };
                        let v1832: f64;
                        if v1830 != 0.0 {
                            v1832 = v1829;
                        } else {
                            let v1831 = -v1829;
                            v1832 = v1831;
                        }
                        let v12329 = v12328 * v1827;
                        let v1835 = ((v1827 * v1827) + v1832).sqrt();
                        let v12333 = (v12329 + v12329) * (v9368 / (v10435 * v1835));
                        let v1836 = v1827 / v1835;
                        let v1838 = v13 * (v1 + v1836);
                        let v1841 = v1229 - (v13 * (v1827 + v1835));
                        let v12340 = ((v12328 + v12333) * v13) * v10390;
                        let v1845 = v1843 * v1838;
                        let v1846 = v1842 * v1845;
                        let v12347 = v12340 * v1841;
                        let v1851 = ((((v1841 * v1841) / v78) / v123) / v206) / v477;
                        let v12352 = ((((v12347 + v12347) / v78) / v123) / v206) / v477;
                        let v1852 = v78 * v1851;
                        let v1854 = (v1852 * v1846) / v1841;
                        let v1871 = ((v1865 + (v1843 / v130)) + ((v1843 * v12) / v123)) + v1854;
                        let v1872 = (((((v1694 - v1772) + (v1810 / v130)) + (((v1810 + (v1228 / v78)) * v12) / v123)) - v1244) + v1851) / v1871;
                        let v1873 = v1772 - v1872;
                        let v12376 = v9495 - (((((((v9484 - v9495) + (v9497 / v130)) + ((v9497 * v12) / v123)) - v12157) + v12352) - ((((v9498 / v130) + ((v9498 * v12) / v123)) + (((((v12352 * v78) * v1846) + (((v9502 * v1845) + (((v9498 * v1838) + ((((v12328 - (v12333 * v1836)) / v1835) * v13) * v1843)) * v1842)) * v1852)) - (v12340 * v1854)) / v1841)) * v1872)) / v1871);
                        let v1876 = if ((v1873 - v1772).abs()) < v529 { 1.0 } else { 0.0 };
                        let v1877: f64;
                        if v1876 != 0.0 {
                            v1877 = v18;
                        } else {
                            v1877 = v1770;
                        }
                        let v1878 = v1877 + v1;
                        v1770 = v1878;
                        v1772 = v1873;
                        v1882 = v1810;
                        v9495 = v12376;
                        v9496 = v9497;
                    }
                    let v1881 = v1244 + v1772;
                    let v12247 = v12157 + v9495;
                    let v1885 = v1694 + (v125 * (v1698 + v1882));
                    let v12249 = v9484 + (v9496 * v125);
                    v2258 = v1694;
                    v2259 = v1885;
                    v2260 = v1881;
                    v2585 = v2586;
                    v2600 = v1882;
                    v2678 = v1697;
                    v3331 = v1619;
                    v5094 = v1694;
                    v9478 = v9484;
                    v9479 = v12249;
                    v9480 = v12247;
                    v9481 = v9496;
                    v9482 = v9485;
                    v9483 = v9484;
                }
                let v1892 = if (if v1886 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v830 > (v1888 + v1889) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v2569: f64;
                let v2676: f64;
                let v4736: f64;
                let v4788: f64;
                let v5644: f64;
                let v5782: f64;
                let v9212: f64;
                let v9503: Lanes<6>;
                let v9504: Lanes<5>;
                let v9505: Lanes<1>;
                let v9506: Lanes<1>;
                let v9507: Lanes<5>;
                let v9508: Lanes<6>;
                if v1892 != 0.0 {
                    let v1895 = ((v867 - v349) + v1142) - v1199;
                    let v12379 = ((Lanes([v10564[0], v10564[1], 0.0, v10564[2], v10564[3]])) + v10791) - v10827;
                    let v1900 = ((v1897 * v477) * v123) / v663;
                    let v1901 = v1900.sqrt();
                    let v12385 = (((v10410 * v1900) * v10390) / v663) * (v9368 / (v10435 * v1901));
                    let v1903 = (v735 / v477) / v477;
                    let v12387 = (v10461 / v477) / v477;
                    let v12388 = v12385 * v1901;
                    let v12389 = v12388 + v12388;
                    let v1905 = (v1901 * v1901) / v1128;
                    let v12390 = v9421 * v1905;
                    let v1906 = v1905 / v1128;
                    let v12395 = v9421 * v1906;
                    let v12398 = ((((Lanes([0.0, 0.0, v12389[0], 0.0, 0.0])) - (Lanes([v12390[0], v12390[1], 0.0, v12390[2], v12390[3]]))) / v1128) - (Lanes([v12395[0], v12395[1], 0.0, v12395[2], v12395[3]]))) / v1128;
                    let v12400 = v10410 * v1906;
                    let v1908 = (v1906 * v663) / v78;
                    let v12403 = ((v12398 * v663) + (Lanes([0.0, 0.0, v12400[0], 0.0, 0.0]))) / v78;
                    let v12405 = v10410 * v1908;
                    let v1910 = (v1908 * v663) * v78;
                    let v12409 = v10410 * v1895;
                    let v1914 = (v90 * ((v663 * v1895) - v1)) / v1910;
                    let v1916 = (v1 + v1914).sqrt();
                    let v1917 = v1 - v1916;
                    let v1920 = v1 / v1903;
                    let v12427 = ((v12387 * v1920) * v10390) / v1903;
                    let v1921 = v1920 / v1906;
                    let v1922 = v1895 * v1895;
                    let v12432 = v12379 * v1895;
                    let v1923 = v1921 * v1922;
                    let v1925 = v78 / v1895;
                    let v1926 = v663 + v1925;
                    let v1927 = (v1923.ln()) / v1926;
                    let v12446 = (((((((Lanes([0.0, 0.0, v12427[0], 0.0, 0.0])) - (v12398 * v1921)) / v1906) * v1922) + ((v12432 + v12432) * v1921)) * (v9368 / v1923)) - (((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + (((v12379 * v1925) * v10390) / v1895)) * v1927)) / v1926;
                    let v12447 = v12446 - (v12379 + ((v12403 * v1917) + ((((((((Lanes([0.0, 0.0, v12409[0], 0.0, 0.0])) + (v12379 * v663)) * v90) - ((((v12403 * v663) + (Lanes([0.0, 0.0, v12405[0], 0.0, 0.0]))) * v78) * v1914)) / v1910) * (v9368 / (v10435 * v1916))) * v10390) * v1908)));
                    let v1929 = (v1927 - (v1895 + (v1908 * v1917))) - v1896;
                    let v12448 = v12447 * v1929;
                    let v1931 = v90 * v1896;
                    let v1934 = ((v1929 * v1929) + (v1931 * v1927)).sqrt();
                    let v1937 = v1927 - (v13 * (v1929 + v1934));
                    let v12457 = v12446 - ((v12447 + (((v12448 + v12448) + (v12446 * v1931)) * (v9368 / (v10435 * v1934)))) * v13);
                    let v1938 = v663 * v1937;
                    let v12458 = v10410 * v1937;
                    let v12461 = (Lanes([0.0, 0.0, v12458[0], 0.0, 0.0])) + (v12457 * v663);
                    let v1939 = v1938.exp();
                    let v1940 = v1938 - v1;
                    let v12463 = v12387 * v1939;
                    let v1942 = v1940 + (v1903 * v1939);
                    let v12467 = v12461 + ((Lanes([0.0, 0.0, v12463[0], 0.0, 0.0])) + ((v12461 * v1939) * v1903));
                    let v1945 = if (if v1942 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1940 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2570: f64;
                    let v2677: f64;
                    let v5645: f64;
                    let v5783: f64;
                    let v9213: f64;
                    let v9509: Lanes<6>;
                    let v9510: Lanes<5>;
                    let v9511: Lanes<5>;
                    let v9512: Lanes<6>;
                    if v1945 != 0.0 {
                        let v1946 = v1942.sqrt();
                        let v1947 = v1940.sqrt();
                        let v1948 = v1946 - v1947;
                        let v1949 = v1901 * v1948;
                        let v12475 = v12385 * v1948;
                        let v1951 = (v78 * v165) / v663;
                        let v1953 = -v663;
                        let v12482 = v10410 * v10390;
                        let v12483 = v12482 * v866;
                        let v12484 = v10561 * v1953;
                        let v1955 = (v1953 * v866).exp();
                        let v1957 = -(v1955 - v1);
                        let v1958 = v1 / v136;
                        let v1959 = v1951 * v1952;
                        let v1960 = v1959 * v1949;
                        let v12491 = ((((v10410 * v1951) * v10390) / v663) * v1952) * v1949;
                        let v12496 = ((((Lanes([0.0, 0.0, v12483[0], 0.0])) + (Lanes([v12484[0], v12484[1], 0.0, v12484[2]]))) * v1955) * v10390) * v1960;
                        let v1962 = (v1960 * v1957) * v1958;
                        let v12499 = ((((Lanes([0.0, 0.0, v12491[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v12475[0], 0.0, 0.0])) + (((v12467 * (v9368 / (v10435 * v1946))) - (v12461 * (v9368 / (v10435 * v1947)))) * v1901)) * v1959)) * v1957) + (Lanes([v12496[0], v12496[1], v12496[2], 0.0, v12496[3]]))) * v1958;
                        let v12500 = v10410 * v1200;
                        let v1966 = v1207 * v664;
                        let v12506 = v10412 * v1207;
                        let v1967 = (v90 * ((v663 * v1200) - v1)) / v1966;
                        let v12511 = ((((Lanes([0.0, 0.0, v12500[0], 0.0, 0.0])) + (v10828 * v663)) * v90) - (((v10836 * v664) + (Lanes([0.0, 0.0, v12506[0], 0.0, 0.0]))) * v1967)) / v1966;
                        let v1968 = v1 + v1967;
                        let v1970 = if v1968 < v1969 { 1.0 } else { 0.0 };
                        let v1974: f64;
                        let v9513: Lanes<5>;
                        if v1970 != 0.0 {
                            v1974 = v1971;
                            v9513 = v10579;
                        } else {
                            v1974 = v1968;
                            v9513 = v12511;
                        }
                        let v12513 = v10410 * v1207;
                        let v1973 = (v1207 * v663) * v13;
                        let v1975 = v1974.sqrt();
                        let v1976 = v1 - v1975;
                        let v1978 = v1200 + (v1973 * v1976);
                        let v12524 = v10828 + (((((v10836 * v663) + (Lanes([0.0, 0.0, v12513[0], 0.0, 0.0]))) * v13) * v1976) + (((v9513 * (v9368 / (v10435 * v1975))) * v10390) * v1973));
                        let v1979 = v1978 - v1937;
                        let v12525 = v12524 - v12457;
                        let v1980 = if v1979 < v0 { 1.0 } else { 0.0 };
                        let v1982: f64;
                        let v9514: Lanes<5>;
                        if v1980 != 0.0 {
                            v1982 = v0;
                            v9514 = v10579;
                        } else {
                            v1982 = v1979;
                            v9514 = v12525;
                        }
                        let v1983 = v1981 * v1982;
                        let v12526 = v9514 * v1981;
                        let v12528 = v12526 - (Lanes([v10561[0], v10561[1], 0.0, 0.0, v10561[2]]));
                        let v1986 = (v1983 - v866) - v1985;
                        let v12529 = v12528 * v1986;
                        let v1991 = ((v1986 * v1986) + ((v90 * v1983) * v1985)).sqrt();
                        let v1994 = v1983 - (v13 * (v1986 + v1991));
                        let v12539 = v12526 - ((v12528 + (((v12529 + v12529) + ((v12526 * v90) * v1985)) * (v9368 / (v10435 * v1991)))) * v13);
                        let v1995 = if v1994 > v1982 { 1.0 } else { 0.0 };
                        let v1996: f64;
                        let v9515: Lanes<5>;
                        if v1995 != 0.0 {
                            v1996 = v1982;
                            v9515 = v9514;
                        } else {
                            v1996 = v1994;
                            v9515 = v12539;
                        }
                        let v1997 = v122 * v68;
                        let v1998 = v166 * v68;
                        let v1999 = v136 * v68;
                        let v2001 = if v2000 == v0 { 1.0 } else { 0.0 };
                        let v2220: f64;
                        let v9516: Lanes<5>;
                        if v2001 != 0.0 {
                            v2220 = v0;
                            v9516 = v10579;
                        } else {
                            let v2006 = ((v2003 * v206) * v1998) * v1999;
                            let v2007 = v2006 / v718;
                            let v12542 = ((v10438 * v2007) * v10390) / v718;
                            let v12543 = v9416 * v2008;
                            let v2016 = (-(((((v2008 * v988) + v1113) + v1137) + v661) + v2013)) / v1997;
                            let v12550 = (((((Lanes([v12543[0], v12543[1], 0.0, 0.0, v12543[2]])) + v10772) + v9430) + (Lanes([0.0, 0.0, v10406[0], 0.0, 0.0]))) * v10390) / v1997;
                            let mut v2017: f64 = 0.0;
                            let mut v2065: f64 = 0.0;
                            let mut v9517: Lanes<5> = Lanes([0.0; 5]);
                            v2017 = v0;
                            v2065 = v0;
                            v9517 = v10579;
                            loop {
                                let v2019 = if v2017 <= v2018 { 1.0 } else { 0.0 };
                                if v2019 == 0.0 {
                                    break;
                                }
                                let v2020 = v2017 / v68;
                                let v2024 = (v1200 + v863) - ((v1996 * v2020) + v1937);
                                let v12555 = (v10828 + (Lanes([v9414[0], v9414[1], 0.0, 0.0, v9414[2]]))) - ((v9515 * v2020) + v12457);
                                let v2026 = v1 - (v2024 / v2002);
                                let v12557 = (v12555 / v2002) * v10390;
                                let v2028 = v2016 + (v2024 / v1997);
                                let v12559 = v12550 + (v12555 / v1997);
                                let v2029 = v2028 * v2028;
                                let v12560 = v12559 * v2028;
                                let v12561 = v12560 + v12560;
                                let v12562 = v12557 * v2026;
                                let v2033 = ((v2026 * v2026) + v2031).sqrt();
                                let v12568 = (v12557 + ((v12562 + v12562) * (v9368 / (v10435 * v2033)))) * v13;
                                let v2037 = (v13 * (v2026 + v2033)) + v2036;
                                let v2038 = if v2037 < v0 { 1.0 } else { 0.0 };
                                let v2040: f64;
                                let v9518: Lanes<5>;
                                if v2038 != 0.0 {
                                    v2040 = v0;
                                    v9518 = v10579;
                                } else {
                                    v2040 = v2037;
                                    v9518 = v12568;
                                }
                                let v2041 = v2040.sqrt();
                                let v2044 = v2039 * (v1 - (v2041 * v2040));
                                let v12576 = ((((v9518 * (v9368 / (v10435 * v2041))) * v2040) + (v9518 * v2041)) * v10390) * v2039;
                                let v2046 = (-v2044) / v2028;
                                let v12580 = ((v12576 * v10390) - (v12559 * v2046)) / v2028;
                                let v2048 = if v2046 < v2047 { 1.0 } else { 0.0 };
                                let v2060: f64;
                                let v9519: Lanes<5>;
                                if v2048 != 0.0 {
                                    v2060 = v0;
                                    v9519 = v10579;
                                } else {
                                    let v2049 = v2046.exp();
                                    let v12581 = v12580 * v2049;
                                    v2060 = v2049;
                                    v9519 = v12581;
                                }
                                let v2051 = v2050 * v2007;
                                let v2052 = v2051 * v2044;
                                let v12583 = (v12542 * v2050) * v2044;
                                let v2055 = (v2052 * v2044) * v2054;
                                let v12590 = ((((Lanes([0.0, 0.0, v12583[0], 0.0, 0.0])) + (v12576 * v2051)) * v2044) + (v12576 * v2052)) * v2054;
                                let v2058 = if ((v78 * v2028) + v2044) < v0 { 1.0 } else { 0.0 };
                                let v2066: f64;
                                let v9520: Lanes<5>;
                                if v2058 != 0.0 {
                                    v2066 = v2055;
                                    v9520 = v12590;
                                } else {
                                    let v2059 = v2006 * v2029;
                                    let v2061 = v2059 * v2060;
                                    let v12594 = ((v12561 * v2006) * v2060) + (v9519 * v2059);
                                    let v2064 = if (if v2061 < v2055 { 1.0 } else { 0.0 }) != 0.0 || (if v2028 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v2067: f64;
                                    let v9521: Lanes<5>;
                                    if v2064 != 0.0 {
                                        v2067 = v2055;
                                        v9521 = v12590;
                                    } else {
                                        v2067 = v2061;
                                        v9521 = v12594;
                                    }
                                    v2066 = v2067;
                                    v9520 = v9521;
                                }
                                let v2068 = v2065 + v2066;
                                let v12595 = v9517 + v9520;
                                let v2069 = if v2066 < v616 { 1.0 } else { 0.0 };
                                let v2070: f64;
                                if v2069 != 0.0 {
                                    v2070 = v68;
                                } else {
                                    v2070 = v2017;
                                }
                                let v2071 = v2070 + v1;
                                v2017 = v2071;
                                v2065 = v2068;
                                v9517 = v12595;
                            }
                            v2220 = v2065;
                            v9516 = v9517;
                        }
                        let v2074 = if (if v297 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v21 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v2219: f64;
                        let v9522: Lanes<5>;
                        if v2074 != 0.0 {
                            v2219 = v0;
                            v9522 = v10579;
                        } else {
                            let v2192: f64;
                            let v9523: Lanes<5>;
                            if v281 != 0.0 {
                                let v2075 = v1128 * v1128;
                                let v12675 = v9421 * v1128;
                                let v12676 = v12675 + v12675;
                                let v2076 = v491 / v2075;
                                let v12679 = ((v12676 * v2076) * v10390) / v2075;
                                let v2077 = v78 / v491;
                                let v2078 = v2077 * v2075;
                                let v12683 = v9416 * v2080;
                                let v2082 = (v1895 - v665) - (v2080 * v988);
                                let v12686 = (v12676 * v2077) * v2082;
                                let v12689 = (Lanes([v12686[0], v12686[1], 0.0, v12686[2], v12686[3]])) + (((v12379 - (Lanes([0.0, 0.0, v10415[0], 0.0, 0.0]))) - (Lanes([v12683[0], v12683[1], 0.0, 0.0, v12683[2]]))) * v2078);
                                let v2084 = v1 + (v2078 * v2082);
                                let v12690 = v12689 * v2084;
                                let v2088 = ((v2084 * v2084) + v2086).sqrt();
                                let v12696 = (v12689 + ((v12690 + v12690) * (v9368 / (v10435 * v2088)))) * v13;
                                let v2092 = (v13 * (v2084 + v2088)) + v2091;
                                let v2093 = if v2092 < v0 { 1.0 } else { 0.0 };
                                let v2094: f64;
                                let v9524: Lanes<5>;
                                if v2093 != 0.0 {
                                    v2094 = v0;
                                    v9524 = v10579;
                                } else {
                                    v2094 = v2092;
                                    v9524 = v12696;
                                }
                                let v2096 = (v2094 + v362).sqrt();
                                let v2100 = v1 - v2096;
                                let v12702 = v12679 * v2100;
                                let v12707 = v10561 * v2103;
                                let v2109 = v2106 * v2107;
                                let v2111 = ((v2103 * v866) + v1937) - (v2109 * ((v1895 * v2097) + (v2076 * v2100)));
                                let v12711 = ((Lanes([v12707[0], v12707[1], 0.0, 0.0, v12707[2]])) + v12457) - (((v12379 * v2097) + ((Lanes([v12702[0], v12702[1], 0.0, v12702[2], v12702[3]])) + (((v9524 * (v9368 / (v10435 * v2096))) * v10390) * v2076))) * v2109);
                                let v12712 = v12711 * v2111;
                                let v2115 = ((v2111 * v2111) + v2113).sqrt();
                                let v12718 = (v12711 + ((v12712 + v12712) * (v9368 / (v10435 * v2115)))) * v13;
                                let v2119 = (v13 * (v2111 + v2115)) + v2118;
                                let v2120 = if v2119 < v0 { 1.0 } else { 0.0 };
                                let v2193: f64;
                                let v9525: Lanes<5>;
                                if v2120 != 0.0 {
                                    v2193 = v0;
                                    v9525 = v10579;
                                } else {
                                    v2193 = v2119;
                                    v9525 = v12718;
                                }
                                v2192 = v2193;
                                v9523 = v9525;
                            } else {
                                let v2123 = v2121 * v1895;
                                let v12596 = v12379 * v2121;
                                let v2124 = v1128 * v1128;
                                let v12597 = v9421 * v1128;
                                let v12598 = v12597 + v12597;
                                let v2125 = v491 / v2124;
                                let v12601 = ((v12598 * v2125) * v10390) / v2124;
                                let v2126 = v78 / v491;
                                let v2127 = v2126 * v2124;
                                let v12602 = v12598 * v2126;
                                let v12605 = v9416 * v2080;
                                let v2130 = (v2123 - v665) - (v2080 * v988);
                                let v12608 = v12602 * v2130;
                                let v12611 = (Lanes([v12608[0], v12608[1], 0.0, v12608[2], v12608[3]])) + (((v12596 - (Lanes([0.0, 0.0, v10415[0], 0.0, 0.0]))) - (Lanes([v12605[0], v12605[1], 0.0, 0.0, v12605[2]]))) * v2127);
                                let v2132 = v1 + (v2127 * v2130);
                                let v2134 = v78 * (v1 + v2127);
                                let v12612 = v12602 * v78;
                                let v2135 = v362 + v2134;
                                let v2138 = if (if v2132 < v2135 { 1.0 } else { 0.0 }) != 0.0 && (if v2134 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v2170: f64;
                                let v9526: Lanes<5>;
                                if v2138 != 0.0 {
                                    let v2139 = v2135 - v2132;
                                    let v12613 = Lanes([v12612[0], v12612[1], 0.0, v12612[2], v12612[3]]);
                                    let v12614 = v12613 - v12611;
                                    let v2140 = v2139 * v2139;
                                    let v12615 = v12614 * v2139;
                                    let v12616 = v12615 + v12615;
                                    let v2141 = v2134 * v2134;
                                    let v12617 = v12612 * v2134;
                                    let v12618 = v12617 + v12617;
                                    let v2142 = v2140 * v2140;
                                    let v12619 = v12616 * v2140;
                                    let v2143 = v2141 * v2141;
                                    let v12621 = v12618 * v2141;
                                    let v2144 = v2142 * v2140;
                                    let v2145 = v2143 * v2141;
                                    let v12634 = ((((v12621 + v12621) * v2141) + (v12618 * v2143)) * v2141) + (v12618 * v2145);
                                    let v2148 = (v2144 * v2140) + (v2145 * v2141);
                                    let v12636 = (((((v12619 + v12619) * v2140) + (v12616 * v2142)) * v2140) + (v12616 * v2144)) + (Lanes([v12634[0], v12634[1], 0.0, v12634[2], v12634[3]]));
                                    let v2165: f64;
                                    let v9527: Lanes<5>;
                                    if v2149 != 0.0 {
                                        let v2159: f64;
                                        if v2150 != 0.0 {
                                            v2159 = v1;
                                        } else {
                                            let v2160: f64;
                                            if v2151 != 0.0 {
                                                v2160 = v78;
                                            } else {
                                                let v2161: f64;
                                                if v2152 != 0.0 {
                                                    v2161 = v96;
                                                } else {
                                                    let v2162: f64;
                                                    if v2153 != 0.0 {
                                                        v2162 = v90;
                                                    } else {
                                                        v2162 = v0;
                                                    }
                                                    v2161 = v2162;
                                                }
                                                v2160 = v2161;
                                            }
                                            v2159 = v2160;
                                        }
                                        let mut v2154: f64 = 0.0;
                                        let mut v2156: f64 = 0.0;
                                        let mut v9528: Lanes<5> = Lanes([0.0; 5]);
                                        v2154 = v0;
                                        v2156 = v2148;
                                        v9528 = v12636;
                                        loop {
                                            let v2155 = if v2154 < v2159 { 1.0 } else { 0.0 };
                                            if v2155 == 0.0 {
                                                break;
                                            }
                                            let v2157 = v2156.sqrt();
                                            let v12674 = v9528 * (v9368 / (v10435 * v2157));
                                            let v2158 = v2154 + v1;
                                            v2154 = v2158;
                                            v2156 = v2157;
                                            v9528 = v12674;
                                        }
                                        v2165 = v2156;
                                        v9527 = v9528;
                                    } else {
                                        let v2164 = v2148.powf(v2163);
                                        let v12640 = v12636 * (v2163 * (v2148.powf(v12637)));
                                        v2165 = v2164;
                                        v9527 = v12640;
                                    }
                                    let v2166 = v1 / v2165;
                                    let v2167 = v2139 * v2134;
                                    let v12645 = v12612 * v2139;
                                    let v2169 = v2135 - (v2167 * v2166);
                                    let v12651 = v12613 - ((((v12614 * v2134) + (Lanes([v12645[0], v12645[1], 0.0, v12645[2], v12645[3]]))) * v2166) + ((((v9527 * v2166) * v10390) / v2165) * v2167));
                                    v2170 = v2169;
                                    v9526 = v12651;
                                } else {
                                    v2170 = v2132;
                                    v9526 = v12611;
                                }
                                let v2171 = if v2170 <= v0 { 1.0 } else { 0.0 };
                                let v2173: f64;
                                let v9529: Lanes<5>;
                                if v2171 != 0.0 {
                                    v2173 = v0;
                                    v9529 = v10579;
                                } else {
                                    let v2172 = v2170.sqrt();
                                    let v12654 = v9526 * (v9368 / (v10435 * v2172));
                                    v2173 = v2172;
                                    v9529 = v12654;
                                }
                                let v2174 = v1 - v2173;
                                let v12656 = v12601 * v2174;
                                let v2178 = v143 / (v2106 + v143);
                                let v12661 = v10561 * v2103;
                                let v2182 = ((v2103 * v866) + v1) - (v2178 * (v2123 + (v2125 * v2174)));
                                let v12664 = (Lanes([v12661[0], v12661[1], 0.0, 0.0, v12661[2]])) - ((v12596 + ((Lanes([v12656[0], v12656[1], 0.0, v12656[2], v12656[3]])) + ((v9529 * v10390) * v2125))) * v2178);
                                let v12665 = v12664 * v2182;
                                let v2186 = ((v2182 * v2182) + v2184).sqrt();
                                let v12671 = (v12664 + ((v12665 + v12665) * (v9368 / (v10435 * v2186)))) * v13;
                                let v2190 = (v13 * (v2182 + v2186)) + v2189;
                                let v2191 = if v2190 < v0 { 1.0 } else { 0.0 };
                                let v2194: f64;
                                let v9530: Lanes<5>;
                                if v2191 != 0.0 {
                                    v2194 = v0;
                                    v9530 = v10579;
                                } else {
                                    v2194 = v2190;
                                    v9530 = v12671;
                                }
                                v2192 = v2194;
                                v9523 = v9530;
                            }
                            let v2195 = v2192 + v362;
                            let v2198 = (-v2196) / v2195;
                            let v2199 = v2198.exp();
                            let v2201 = v2200 * v2195;
                            let v2202 = v2201 * v1962;
                            let v2203 = v2202 * v2199;
                            let v12729 = ((((v9523 * v2200) * v1962) + (v12499 * v2201)) * v2199) + (((((v9523 * v2198) * v10390) / v2195) * v2199) * v2202);
                            v2219 = v2203;
                            v9522 = v12729;
                        }
                        let v2205 = if v2204 == v1 { 1.0 } else { 0.0 };
                        let v2571: f64;
                        let v9214: f64;
                        let v9531: Lanes<6>;
                        let v9532: Lanes<6>;
                        if v2205 != 0.0 {
                            let v2207 = (v206 * v12) * v166;
                            let v2210 = (v1953 * v2208).exp();
                            let v2215 = v2212 + (v2213 * v477);
                            let v2217 = (v2207 * v2210) * v2215;
                            let v2218 = v2216 / v2217;
                            let v2221 = v2219 + v2220;
                            let v12739 = (((((((v12482 * v2208) * v2210) * v2207) * v2215) * v2218) * v10390) / v2217) * v2221;
                            let v2224 = v2223 * v665;
                            let v2225 = v1 + (v2221 * v2218);
                            let v2226 = v2225.ln();
                            let v12745 = (v10415 * v2223) * v2226;
                            let v2229 = v2228 * v477;
                            let v2231 = (v2229 * v665).sqrt();
                            let v2232 = v1937 - (v2224 * v2226);
                            let v12753 = v12457 - ((Lanes([0.0, 0.0, v12745[0], 0.0, 0.0])) + (((((v9522 + v9516) * v2218) + (Lanes([0.0, 0.0, v12739[0], 0.0, 0.0]))) * (v9368 / v2225)) * v2224));
                            let v12754 = v12482 * v2232;
                            let v2234 = (v1953 * v2232).exp();
                            let v12759 = v10410 * v2232;
                            let v2238 = ((v2234 - v1) + (v663 * v2232)).sqrt();
                            let v12767 = v12482 * v1937;
                            let v2240 = (v1953 * v1937).exp();
                            let v2243 = ((v2240 - v1) + v1938).sqrt();
                            let v2244 = -v2231;
                            let v2245 = v2238 - v2243;
                            let v2246 = v2244 * v2245;
                            let v12778 = (((v10415 * v2229) * (v9368 / (v10435 * v2231))) * v10390) * v2245;
                            let v12781 = (Lanes([0.0, 0.0, v12778[0], 0.0, 0.0])) + (((((((Lanes([0.0, 0.0, v12754[0], 0.0, 0.0])) + (v12753 * v1953)) * v2234) + ((Lanes([0.0, 0.0, v12759[0], 0.0, 0.0])) + (v12753 * v663))) * (v9368 / (v10435 * v2238))) - (((((Lanes([0.0, 0.0, v12767[0], 0.0, 0.0])) + (v12457 * v1953)) * v2240) + v12461) * (v9368 / (v10435 * v2243)))) * v2244);
                            let v2572: f64;
                            let v9215: f64;
                            let v9533: Lanes<6>;
                            let v9534: Lanes<6>;
                            if v2247 != 0.0 {
                                let v2250 = v2219 + v2249;
                                let v2251 = v2248 / v2250;
                                let v2252 = v2251 * v1128;
                                let v12787 = v9421 * v2251;
                                let v2255 = v2253 * v2254;
                                let v12790 = v9380 * v2253;
                                let v12791 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v12790[0]]);
                                let v2257 = (v2255 - v2246) / v2252;
                                let v12794 = (((((v9522 * v2251) * v10390) / v2250) * v1128) + (Lanes([v12787[0], v12787[1], 0.0, v12787[2], v12787[3]]))) * v2257;
                                let v12797 = ((v12791 - (Lanes([v12781[0], v12781[1], v12781[2], v12781[3], v12781[4], 0.0]))) - (Lanes([v12794[0], v12794[1], v12794[2], v12794[3], v12794[4], 0.0]))) / v2252;
                                v2572 = v2255;
                                v9215 = v2257;
                                v9533 = v12791;
                                v9534 = v12797;
                            } else {
                                let v12782 = Lanes([v12781[0], v12781[1], v12781[2], v12781[3], v12781[4], 0.0]);
                                v2572 = v2246;
                                v9215 = v0;
                                v9533 = v12782;
                                v9534 = v11062;
                            }
                            v2571 = v2572;
                            v9214 = v9215;
                            v9531 = v9533;
                            v9532 = v9534;
                        } else {
                            v2571 = v0;
                            v9214 = v0;
                            v9531 = v11062;
                            v9532 = v11062;
                        }
                        v2570 = v2571;
                        v2677 = v1978;
                        v5645 = v2219;
                        v5783 = v1952;
                        v9213 = v9214;
                        v9509 = v9531;
                        v9510 = v12524;
                        v9511 = v9522;
                        v9512 = v9532;
                    } else {
                        v2570 = v0;
                        v2677 = v2678;
                        v5645 = v0;
                        v5783 = v0;
                        v9213 = v0;
                        v9509 = v11062;
                        v9510 = v9482;
                        v9511 = v10579;
                        v9512 = v11062;
                    }
                    v2569 = v2570;
                    v2676 = v2677;
                    v4736 = v1903;
                    v4788 = v1901;
                    v5644 = v5645;
                    v5782 = v5783;
                    v9212 = v9213;
                    v9503 = v9509;
                    v9504 = v9510;
                    v9505 = v12387;
                    v9506 = v12385;
                    v9507 = v9511;
                    v9508 = v9512;
                } else {
                    v2569 = v0;
                    v2676 = v2678;
                    v4736 = v736;
                    v4788 = v733;
                    v5644 = v0;
                    v5782 = v0;
                    v9212 = v0;
                    v9503 = v11062;
                    v9504 = v9482;
                    v9505 = v10462;
                    v9506 = v10457;
                    v9507 = v10579;
                    v9508 = v11062;
                }
                let v12798 = Lanes([v9480[0], v9480[1], v9480[2], v9480[3], v9480[4], 0.0]);
                let v12799 = Lanes([v9478[0], v9478[1], v9478[2], v9478[3], v9478[4], 0.0]);
                let v12800 = Lanes([v9479[0], v9479[1], v9479[2], v9479[3], v9479[4], 0.0]);
                let v12801 = Lanes([v9481[0], v9481[1], v9481[2], v9481[3], v9481[4], 0.0]);
                let mut v2261: f64 = 0.0;
                let mut v2263: f64 = 0.0;
                let mut v2299: f64 = 0.0;
                let mut v2321: f64 = 0.0;
                let mut v2455: f64 = 0.0;
                let mut v2573: f64 = 0.0;
                let mut v2578: f64 = 0.0;
                let mut v2589: f64 = 0.0;
                let mut v2592: f64 = 0.0;
                let mut v2599: f64 = 0.0;
                let mut v9535: Lanes<6> = Lanes([0.0; 6]);
                let mut v9536: Lanes<6> = Lanes([0.0; 6]);
                let mut v9537: Lanes<6> = Lanes([0.0; 6]);
                let mut v9538: Lanes<6> = Lanes([0.0; 6]);
                let mut v9539: Lanes<6> = Lanes([0.0; 6]);
                let mut v9540: Lanes<6> = Lanes([0.0; 6]);
                let mut v9541: Lanes<6> = Lanes([0.0; 6]);
                v2261 = v1;
                v2263 = v2260;
                v2299 = v2258;
                v2321 = v2259;
                v2455 = v0;
                v2573 = v0;
                v2578 = v0;
                v2589 = v0;
                v2592 = v0;
                v2599 = v2600;
                v9535 = v12798;
                v9536 = v12799;
                v9537 = v12800;
                v9538 = v11062;
                v9539 = v11062;
                v9540 = v11062;
                v9541 = v12801;
                loop {
                    let v2262 = if v2261 <= v18 { 1.0 } else { 0.0 };
                    if v2262 == 0.0 {
                        break;
                    }
                    let v2264 = v2263 - v1244;
                    let v2265 = v663 * v2264;
                    let v18864 = v10410 * v2264;
                    let v18867 = (Lanes([0.0, 0.0, v18864[0], 0.0, 0.0, 0.0])) + ((v9535 - (Lanes([v9469[0], v9469[1], v9469[2], 0.0, v9469[3], 0.0]))) * v663);
                    let v2267 = (-v2265).exp();
                    let v18869 = (v18867 * v10390) * v2267;
                    let v2269 = if v2264 < v2268 { 1.0 } else { 0.0 };
                    let v2458: f64;
                    let v2471: f64;
                    let v9542: Lanes<6>;
                    let v9543: Lanes<6>;
                    if v2269 != 0.0 {
                        let v2272 = ((v2267 + v2265) - v1).sqrt();
                        let v2273 = v1240 * v2272;
                        let v18909 = v9404 * v2272;
                        let v18912 = (Lanes([0.0, 0.0, v18909[0], 0.0, 0.0, 0.0])) + (((v18869 + v18867) * (v9368 / (v10435 * v2272))) * v1240);
                        let v2277 = (v211 * ((-v2267) + v1)) / v2273;
                        let v18917 = (((v18869 * v10390) * v211) - (v18912 * v2277)) / v2273;
                        v2458 = v2273;
                        v2471 = v2277;
                        v9542 = v18912;
                        v9543 = v18917;
                    } else {
                        let v2278 = if v2264 > v616 { 1.0 } else { 0.0 };
                        let v2459: f64;
                        let v2472: f64;
                        let v9544: Lanes<6>;
                        let v9545: Lanes<6>;
                        if v2278 != 0.0 {
                            let v2279 = v2265.exp();
                            let v18879 = v18867 * v2279;
                            let v2280 = -v1240;
                            let v2284 = (v2279 + v2265) - v1;
                            let v18883 = v9405 * v2284;
                            let v2287 = (((v2267 + v2265) - v1) + (v1262 * v2284)).sqrt();
                            let v2288 = v2280 * v2287;
                            let v18891 = (v9404 * v10390) * v2287;
                            let v18894 = (Lanes([0.0, 0.0, v18891[0], 0.0, 0.0, 0.0])) + ((((v18869 + v18867) + ((Lanes([0.0, 0.0, v18883[0], 0.0, 0.0, 0.0])) + ((v18879 + v18867) * v1262))) * (v9368 / (v10435 * v2287))) * v2280);
                            let v2291 = v2279 + v1;
                            let v18896 = v9405 * v2291;
                            let v2295 = (v211 * (((-v2267) + v1) + (v1262 * v2291))) / v2288;
                            let v18904 = ((((v18869 * v10390) + ((Lanes([0.0, 0.0, v18896[0], 0.0, 0.0, 0.0])) + (v18879 * v1262))) * v211) - (v18894 * v2295)) / v2288;
                            v2459 = v2288;
                            v2472 = v2295;
                            v9544 = v18894;
                            v9545 = v18904;
                        } else {
                            let v2296 = -v1240;
                            let v18870 = v9404 * v10390;
                            let v2297 = v2296 * v2265;
                            let v18871 = v18870 * v2265;
                            let v18874 = (Lanes([0.0, 0.0, v18871[0], 0.0, 0.0, 0.0])) + (v18867 * v2296);
                            let v2298 = v2296 * v663;
                            let v18877 = (v18870 * v663) + (v10410 * v2296);
                            let v18878 = Lanes([0.0, 0.0, v18877[0], 0.0, 0.0, 0.0]);
                            v2459 = v2297;
                            v2472 = v2298;
                            v9544 = v18874;
                            v9545 = v18878;
                        }
                        v2458 = v2459;
                        v2471 = v2472;
                        v9542 = v9544;
                        v9543 = v9545;
                    }
                    let v2300 = v663 * v2299;
                    let v18918 = v10410 * v2299;
                    let v18921 = (Lanes([0.0, 0.0, v18918[0], 0.0, 0.0, 0.0])) + (v9536 * v663);
                    let v2301 = v2300.exp();
                    let v18922 = v18921 * v2301;
                    let v18923 = v12078 * v1504;
                    let v2303 = v750 * v750;
                    let v18925 = v10485 * v750;
                    let v2304 = (v1504 * v1504) / v2303;
                    let v18927 = (v18925 + v18925) * v2304;
                    let v18930 = ((v18923 + v18923) - (Lanes([0.0, 0.0, v18927[0], 0.0, 0.0]))) / v2303;
                    let v2305 = v78 * v759;
                    let v2307 = (v2301 + v2300) - v1;
                    let v18933 = (v10496 * v78) * v2307;
                    let v2310 = (v2304 + (v2305 * v2307)).sqrt();
                    let v18941 = ((Lanes([v18930[0], v18930[1], v18930[2], v18930[3], v18930[4], 0.0])) + ((Lanes([0.0, 0.0, v18933[0], 0.0, 0.0, 0.0])) + ((v18922 + v18921) * v2305))) * (v9368 / (v10435 * v2310));
                    let v2311 = v78 * v663;
                    let v2312 = v2311 * v759;
                    let v2313 = v2301 + v1;
                    let v18946 = (((v10410 * v78) * v759) + (v10496 * v2311)) * v2313;
                    let v2315 = v78 * v2310;
                    let v2316 = (v2312 * v2313) / v2315;
                    let v2317 = -v750;
                    let v18954 = v10485 * v10390;
                    let v18955 = v18954 * v2310;
                    let v2319 = (v2317 * v2310) - v1504;
                    let v18959 = Lanes([v12078[0], v12078[1], v12078[2], v12078[3], v12078[4], 0.0]);
                    let v18960 = ((Lanes([0.0, 0.0, v18955[0], 0.0, 0.0, 0.0])) + (v18941 * v2317)) - v18959;
                    let v2320 = v2317 * v2316;
                    let v18961 = v18954 * v2316;
                    let v18964 = (Lanes([0.0, 0.0, v18961[0], 0.0, 0.0, 0.0])) + (((((Lanes([0.0, 0.0, v18946[0], 0.0, 0.0, 0.0])) + (v18922 * v2312)) - ((v18941 * v78) * v2316)) / v2315) * v2317);
                    let v2323 = (v2321 - v2299) / v1208;
                    let v2324 = v663 * v2323;
                    let v18967 = v10410 * v2323;
                    let v18970 = (Lanes([0.0, 0.0, v18967[0], 0.0, 0.0, 0.0])) + (((v9537 - v9536) / v1208) * v663);
                    let v2325 = -v2324;
                    let v18971 = v18970 * v10390;
                    let v2327 = if v2325 >= v2326 { 1.0 } else { 0.0 };
                    let v2346: f64;
                    let v9546: Lanes<6>;
                    if v2327 != 0.0 {
                        v2346 = v2328;
                        v9546 = v11062;
                    } else {
                        let mut v2329: f64 = 0.0;
                        let mut v2332: f64 = 0.0;
                        let mut v9547: Lanes<6> = Lanes([0.0; 6]);
                        v2329 = v2325;
                        v2332 = v1;
                        v9547 = v18971;
                        loop {
                            let v2331 = if v2329 >= v2330 { 1.0 } else { 0.0 };
                            if v2331 == 0.0 {
                                break;
                            }
                            let v2334 = v2332 * v2333;
                            let v2335 = v2329 - v2330;
                            let edge0 = v2335;
                            let edge1 = v2334;
                            let edge2 = v9547;
                            v2329 = edge0;
                            v2332 = edge1;
                            v9547 = edge2;
                        }
                        let v2336 = v2329.exp();
                        let v2337 = v2332 * v2336;
                        let v18973 = (v9547 * v2336) * v2332;
                        v2346 = v2337;
                        v9546 = v18973;
                    }
                    let v2338 = v2325.exp();
                    let v2341 = ((v2338 + v2324) - v1).sqrt();
                    let v18978 = ((v18971 * v2338) + v18970) * (v9368 / (v10435 * v2341));
                    let v2343 = if v2323 < v2342 { 1.0 } else { 0.0 };
                    let v2369: f64;
                    let v2406: f64;
                    let v2410: f64;
                    let v9548: Lanes<6>;
                    let v9549: Lanes<6>;
                    let v9550: Lanes<6>;
                    if v2343 != 0.0 {
                        let v2344 = v750 * v2341;
                        let v19009 = v10485 * v2341;
                        let v19012 = (Lanes([0.0, 0.0, v19009[0], 0.0, 0.0, 0.0])) + (v18978 * v750);
                        let v2345 = v750 * v663;
                        let v2348 = (-v2346) + v1;
                        let v19017 = ((v10485 * v663) + (v10410 * v750)) * v2348;
                        let v2350 = v78 * v2341;
                        let v2351 = (v2345 * v2348) / v2350;
                        let v2352 = v2351 / v1208;
                        let v19025 = ((((Lanes([0.0, 0.0, v19017[0], 0.0, 0.0, 0.0])) + ((v9546 * v10390) * v2345)) - ((v18978 * v78) * v2351)) / v2350) / v1208;
                        let v2353 = -v2352;
                        let v19026 = v19025 * v10390;
                        v2369 = v2344;
                        v2406 = v2352;
                        v2410 = v2353;
                        v9548 = v19012;
                        v9549 = v19025;
                        v9550 = v19026;
                    } else {
                        let v2354 = if v2323 > v616 { 1.0 } else { 0.0 };
                        let v2370: f64;
                        let v2407: f64;
                        let v2411: f64;
                        let v9551: Lanes<6>;
                        let v9552: Lanes<6>;
                        let v9553: Lanes<6>;
                        if v2354 != 0.0 {
                            let v2355 = v2317 * v2341;
                            let v18991 = v18954 * v2341;
                            let v18994 = (Lanes([0.0, 0.0, v18991[0], 0.0, 0.0, 0.0])) + (v18978 * v2317);
                            let v2356 = v2317 * v663;
                            let v2358 = (-v2346) + v1;
                            let v18999 = ((v18954 * v663) + (v10410 * v2317)) * v2358;
                            let v2360 = v78 * v2341;
                            let v2361 = (v2356 * v2358) / v2360;
                            let v2362 = v2361 / v1208;
                            let v19007 = ((((Lanes([0.0, 0.0, v18999[0], 0.0, 0.0, 0.0])) + ((v9546 * v10390) * v2356)) - ((v18978 * v78) * v2361)) / v2360) / v1208;
                            let v2363 = -v2362;
                            let v19008 = v19007 * v10390;
                            v2370 = v2355;
                            v2407 = v2362;
                            v2411 = v2363;
                            v9551 = v18994;
                            v9552 = v19007;
                            v9553 = v19008;
                        } else {
                            let v18979 = v18954 * v2324;
                            let v2365 = (v2317 * v2324) / v748;
                            let v18983 = ((Lanes([0.0, 0.0, v18979[0], 0.0, 0.0, 0.0])) + (v18970 * v2317)) / v748;
                            let v2367 = (v2317 * v663) / v748;
                            let v18987 = ((v18954 * v663) + (v10410 * v2317)) / v748;
                            let v2368 = -v2367;
                            let v18988 = v18987 * v10390;
                            let v18989 = Lanes([0.0, 0.0, v18987[0], 0.0, 0.0, 0.0]);
                            let v18990 = Lanes([0.0, 0.0, v18988[0], 0.0, 0.0, 0.0]);
                            v2370 = v2365;
                            v2407 = v2367;
                            v2411 = v2368;
                            v9551 = v18983;
                            v9552 = v18989;
                            v9553 = v18990;
                        }
                        v2369 = v2370;
                        v2406 = v2407;
                        v2410 = v2411;
                        v9548 = v9551;
                        v9549 = v9552;
                        v9550 = v9553;
                    }
                    let v2371 = -v1225;
                    let v19027 = v11952 * v10390;
                    let v2372 = v0 - v2371;
                    let v19028 = v19027 * v10390;
                    let v2375 = if (if v2369 > v2372 { 1.0 } else { 0.0 }) != 0.0 && (if v2371 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2408: f64;
                    let v2413: f64;
                    let v9554: Lanes<6>;
                    let v9555: Lanes<6>;
                    if v2375 != 0.0 {
                        let v2376 = v2369 + v2371;
                        let v19030 = v9548 + (Lanes([v19027[0], v19027[1], v19027[2], v19027[3], v19027[4], 0.0]));
                        let v2377 = v2376 * v2376;
                        let v19031 = v19030 * v2376;
                        let v2378 = v2371 * v2371;
                        let v19033 = v19027 * v2371;
                        let v19035 = (v19031 + v19031) * v2377;
                        let v2380 = v2378 * v2378;
                        let v19037 = (v19033 + v19033) * v2378;
                        let v19038 = v19037 + v19037;
                        let v2381 = (v2377 * v2377) + v2380;
                        let v19040 = (v19035 + v19035) + (Lanes([v19038[0], v19038[1], v19038[2], v19038[3], v19038[4], 0.0]));
                        let v2398: f64;
                        let v9556: Lanes<6>;
                        if v2382 != 0.0 {
                            let v2392: f64;
                            if v2383 != 0.0 {
                                v2392 = v1;
                            } else {
                                let v2393: f64;
                                if v2384 != 0.0 {
                                    v2393 = v78;
                                } else {
                                    let v2394: f64;
                                    if v2385 != 0.0 {
                                        v2394 = v96;
                                    } else {
                                        let v2395: f64;
                                        if v2386 != 0.0 {
                                            v2395 = v90;
                                        } else {
                                            v2395 = v0;
                                        }
                                        v2394 = v2395;
                                    }
                                    v2393 = v2394;
                                }
                                v2392 = v2393;
                            }
                            let mut v2387: f64 = 0.0;
                            let mut v2389: f64 = 0.0;
                            let mut v9557: Lanes<6> = Lanes([0.0; 6]);
                            v2387 = v0;
                            v2389 = v2381;
                            v9557 = v19040;
                            loop {
                                let v2388 = if v2387 < v2392 { 1.0 } else { 0.0 };
                                if v2388 == 0.0 {
                                    break;
                                }
                                let v2390 = v2389.sqrt();
                                let v19262 = v9557 * (v9368 / (v10435 * v2390));
                                let v2391 = v2387 + v1;
                                v2387 = v2391;
                                v2389 = v2390;
                                v9557 = v19262;
                            }
                            v2398 = v2389;
                            v9556 = v9557;
                        } else {
                            let v2397 = v2381.powf(v2396);
                            let v19044 = v19040 * (v2396 * (v2381.powf(v19041)));
                            v2398 = v2397;
                            v9556 = v19044;
                        }
                        let v2399 = v1 / v2398;
                        let v19047 = ((v9556 * v2399) * v10390) / v2398;
                        let v2400 = v2376 * v2371;
                        let v19049 = v19027 * v2376;
                        let v2402 = v2371 * v2380;
                        let v19058 = ((v19027 * v2380) + (v19038 * v2371)) * v2399;
                        let v2404 = (v2402 * v2399) / v2381;
                        let v19064 = (((Lanes([v19058[0], v19058[1], v19058[2], v19058[3], v19058[4], 0.0])) + (v19047 * v2402)) - (v19040 * v2404)) / v2381;
                        let v2405 = v2372 + (v2400 * v2399);
                        let v19066 = (Lanes([v19028[0], v19028[1], v19028[2], v19028[3], v19028[4], 0.0])) + ((((v19030 * v2371) + (Lanes([v19049[0], v19049[1], v19049[2], v19049[3], v19049[4], 0.0]))) * v2399) + (v19047 * v2400));
                        v2408 = v2404;
                        v2413 = v2405;
                        v9554 = v19064;
                        v9555 = v19066;
                    } else {
                        v2408 = v1;
                        v2413 = v2369;
                        v9554 = v11062;
                        v9555 = v9548;
                    }
                    let v2409 = v2406 * v2408;
                    let v19069 = (v9549 * v2408) + (v9554 * v2406);
                    let v2412 = v2410 * v2408;
                    let v19072 = (v9550 * v2408) + (v9554 * v2410);
                    let v2414 = v1228 - v1504;
                    let v19073 = v12078 * v10390;
                    let v2415 = -v2414;
                    let v19074 = v19073 * v10390;
                    let v2416 = v2414 + v2415;
                    let v19075 = v19073 + v19074;
                    let v2419 = if (if v2413 < v2416 { 1.0 } else { 0.0 }) != 0.0 && (if v2415 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2450: f64;
                    let v2453: f64;
                    let v9558: Lanes<6>;
                    let v9559: Lanes<6>;
                    if v2419 != 0.0 {
                        let v2420 = v2416 - v2413;
                        let v19076 = Lanes([v19075[0], v19075[1], v19075[2], v19075[3], v19075[4], 0.0]);
                        let v19077 = v19076 - v9555;
                        let v2421 = v2420 * v2420;
                        let v19078 = v19077 * v2420;
                        let v2422 = v2415 * v2415;
                        let v19080 = v19074 * v2415;
                        let v19082 = (v19078 + v19078) * v2421;
                        let v2424 = v2422 * v2422;
                        let v19084 = (v19080 + v19080) * v2422;
                        let v19085 = v19084 + v19084;
                        let v2425 = (v2421 * v2421) + v2424;
                        let v19087 = (v19082 + v19082) + (Lanes([v19085[0], v19085[1], v19085[2], v19085[3], v19085[4], 0.0]));
                        let v2442: f64;
                        let v9560: Lanes<6>;
                        if v2426 != 0.0 {
                            let v2436: f64;
                            if v2427 != 0.0 {
                                v2436 = v1;
                            } else {
                                let v2437: f64;
                                if v2428 != 0.0 {
                                    v2437 = v78;
                                } else {
                                    let v2438: f64;
                                    if v2429 != 0.0 {
                                        v2438 = v96;
                                    } else {
                                        let v2439: f64;
                                        if v2430 != 0.0 {
                                            v2439 = v90;
                                        } else {
                                            v2439 = v0;
                                        }
                                        v2438 = v2439;
                                    }
                                    v2437 = v2438;
                                }
                                v2436 = v2437;
                            }
                            let mut v2431: f64 = 0.0;
                            let mut v2433: f64 = 0.0;
                            let mut v9561: Lanes<6> = Lanes([0.0; 6]);
                            v2431 = v0;
                            v2433 = v2425;
                            v9561 = v19087;
                            loop {
                                let v2432 = if v2431 < v2436 { 1.0 } else { 0.0 };
                                if v2432 == 0.0 {
                                    break;
                                }
                                let v2434 = v2433.sqrt();
                                let v19259 = v9561 * (v9368 / (v10435 * v2434));
                                let v2435 = v2431 + v1;
                                v2431 = v2435;
                                v2433 = v2434;
                                v9561 = v19259;
                            }
                            v2442 = v2433;
                            v9560 = v9561;
                        } else {
                            let v2441 = v2425.powf(v2440);
                            let v19091 = v19087 * (v2440 * (v2425.powf(v19088)));
                            v2442 = v2441;
                            v9560 = v19091;
                        }
                        let v2443 = v1 / v2442;
                        let v19094 = ((v9560 * v2443) * v10390) / v2442;
                        let v2444 = v2420 * v2415;
                        let v19096 = v19074 * v2420;
                        let v2446 = v2415 * v2424;
                        let v19105 = ((v19074 * v2424) + (v19085 * v2415)) * v2443;
                        let v2448 = (v2446 * v2443) / v2425;
                        let v19111 = (((Lanes([v19105[0], v19105[1], v19105[2], v19105[3], v19105[4], 0.0])) + (v19094 * v2446)) - (v19087 * v2448)) / v2425;
                        let v2449 = v2416 - (v2444 * v2443);
                        let v19112 = v19076 - ((((v19077 * v2415) + (Lanes([v19096[0], v19096[1], v19096[2], v19096[3], v19096[4], 0.0]))) * v2443) + (v19094 * v2444));
                        v2450 = v2448;
                        v2453 = v2449;
                        v9558 = v19111;
                        v9559 = v19112;
                    } else {
                        v2450 = v1;
                        v2453 = v2413;
                        v9558 = v11062;
                        v9559 = v9555;
                    }
                    let v2451 = v2412 * v2450;
                    let v19115 = (v19072 * v2450) + (v9558 * v2412);
                    let v2452 = v2409 * v2450;
                    let v19118 = (v19069 * v2450) + (v9558 * v2409);
                    let v2454 = v1504 + v2453;
                    let v19119 = v18959 + v9559;
                    let v2456 = if v2455 == v1 { 1.0 } else { 0.0 };
                    let v2562: f64;
                    let v2564: f64;
                    let v2565: f64;
                    let v2566: f64;
                    let v2567: f64;
                    let v2574: f64;
                    let v9562: Lanes<6>;
                    let v9563: Lanes<6>;
                    let v9564: Lanes<6>;
                    if v2456 != 0.0 {
                        v2562 = v18;
                        v2564 = v2263;
                        v2565 = v2299;
                        v2566 = v2321;
                        v2567 = v2455;
                        v2574 = v2261;
                        v9562 = v9535;
                        v9563 = v9536;
                        v9564 = v9537;
                    } else {
                        let v2463 = (((v2458 + v1504) + v2319) + v2453) + v2569;
                        let v19126 = v9420 * v2463;
                        let v2465 = (v2299 - v1200) - (v1048 * v2463);
                        let v19130 = (v9536 - (Lanes([v10828[0], v10828[1], v10828[2], v10828[3], v10828[4], 0.0]))) - ((Lanes([v19126[0], v19126[1], 0.0, v19126[2], v19126[3], 0.0])) + (((((v9542 + v18959) + v18960) + v9559) + v9503) * v1048));
                        let v2466 = v2320 + v2451;
                        let v19132 = v9420 * v2466;
                        let v2468 = v1 - (v1048 * v2466);
                        let v19136 = ((Lanes([v19132[0], v19132[1], 0.0, v19132[2], v19132[3], 0.0])) + ((v18964 + v19115) * v1048)) * v10390;
                        let v2469 = -v1048;
                        let v19137 = v9420 * v10390;
                        let v2470 = v2469 * v2452;
                        let v19138 = v19137 * v2452;
                        let v19141 = (Lanes([v19138[0], v19138[1], 0.0, v19138[2], v19138[3], 0.0])) + (v19118 * v2469);
                        let v2473 = v2469 * v2471;
                        let v19142 = v19137 * v2471;
                        let v19145 = (Lanes([v19142[0], v19142[1], 0.0, v19142[2], v19142[3], 0.0])) + (v9543 * v2469);
                        let v2479 = v2321 - (v2299 + (v125 * ((v13 * v1228) + v2458)));
                        let v19149 = v9537 - (v9536 + (v9542 * v125));
                        let v2481 = -(v125 * v2471);
                        let v19150 = (v9543 * v125) * v10390;
                        let v2484 = (v2263 - v2321) - (v131 * v2458);
                        let v19153 = (v9535 - v9537) - (v9542 * v131);
                        let v2487 = v1 - (v131 * v2471);
                        let v19155 = (v9543 * v131) * v10390;
                        let v2488 = v2468 * v2487;
                        let v19158 = (v19136 * v2487) + (v19155 * v2468);
                        let v2489 = v2468 * v2481;
                        let v19161 = (v19136 * v2481) + (v19150 * v2468);
                        let v2492 = v2470 * v2480;
                        let v19164 = v19141 * v2480;
                        let v2495 = v2473 * v2480;
                        let v19169 = v19145 * v2480;
                        let v2498 = (((v2488 - (v2489 * v2485)) - (v2492 * v2487)) + (v2495 * v2485)) + v362;
                        let v2499 = v1 / v2498;
                        let v2501 = v2487 - (v2481 * v2485);
                        let v2504 = (v2473 * v2485) - (v2470 * v2487);
                        let v2506 = (v2470 * v2481) - v2473;
                        let v2507 = v2495 - v2489;
                        let v2509 = (-v2468) * v2485;
                        let v2510 = v2468 - v2492;
                        let v2511 = -v2499;
                        let v19190 = ((((((v19158 - (v19161 * v2485)) - ((v19164 * v2487) + (v19155 * v2492))) + (v19169 * v2485)) * v2499) * v10390) / v2498) * v10390;
                        let v2516 = ((v2501 * v2465) + (v2504 * v2479)) + (v2506 * v2484);
                        let v2517 = v2511 * v2516;
                        let v19204 = (v19190 * v2516) + ((((((v19155 - (v19150 * v2485)) * v2465) + (v19130 * v2501)) + ((((v19145 * v2485) - ((v19141 * v2487) + (v19155 * v2470))) * v2479) + (v19149 * v2504))) + (((((v19141 * v2481) + (v19150 * v2470)) - v19145) * v2484) + (v19153 * v2506))) * v2511);
                        let v2522 = ((v2487 * v2465) + (v2488 * v2479)) + (v2507 * v2484);
                        let v2523 = v2511 * v2522;
                        let v19218 = (v19190 * v2522) + (((((v19155 * v2465) + (v19130 * v2487)) + ((v19158 * v2479) + (v19149 * v2488))) + (((v19169 - v19161) * v2484) + (v19153 * v2507))) * v2511);
                        let v2527 = (v2465 + (v2509 * v2479)) + (v2510 * v2484);
                        let v2528 = v2511 * v2527;
                        let v19229 = (v19190 * v2527) + (((v19130 + ((((v19136 * v10390) * v2485) * v2479) + (v19149 * v2509))) + (((v19136 - v19164) * v2484) + (v19153 * v2510))) * v2511);
                        let v2529 = v2517.abs();
                        let v19233 = v19204 * ((v10435 * (if v2517 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                        let v2530 = v2523.abs();
                        let v19237 = v19218 * ((v10435 * (if v2523 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                        let v2531 = if v2529 < v2530 { 1.0 } else { 0.0 };
                        let v2532: f64;
                        let v9565: Lanes<6>;
                        if v2531 != 0.0 {
                            v2532 = v2530;
                            v9565 = v19237;
                        } else {
                            v2532 = v2529;
                            v9565 = v19233;
                        }
                        let v2533 = v2528.abs();
                        let v19241 = v19229 * ((v10435 * (if v2528 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                        let v2534 = if v2532 < v2533 { 1.0 } else { 0.0 };
                        let v2543: f64;
                        let v9566: Lanes<6>;
                        if v2534 != 0.0 {
                            v2543 = v2533;
                            v9566 = v19241;
                        } else {
                            v2543 = v2532;
                            v9566 = v9565;
                        }
                        let v2536 = if v2261 > v2535 { 1.0 } else { 0.0 };
                        let v2544: f64;
                        if v2536 != 0.0 {
                            v2544 = v2537;
                        } else {
                            let v2539 = if v2261 > v2538 { 1.0 } else { 0.0 };
                            let v2545: f64;
                            if v2539 != 0.0 {
                                v2545 = v2537;
                            } else {
                                let v2540 = if v2261 > v821 { 1.0 } else { 0.0 };
                                let v2546: f64;
                                if v2540 != 0.0 {
                                    v2546 = v2541;
                                } else {
                                    let v2542 = if v2261 > v15 { 1.0 } else { 0.0 };
                                    let v2547: f64;
                                    if v2542 != 0.0 {
                                        v2547 = v644;
                                    } else {
                                        v2547 = v1;
                                    }
                                    v2546 = v2547;
                                }
                                v2545 = v2546;
                            }
                            v2544 = v2545;
                        }
                        let v2548 = v79 / v2544;
                        let v2549 = if v2543 > v2548 { 1.0 } else { 0.0 };
                        let v2554: f64;
                        let v2556: f64;
                        let v2558: f64;
                        let v9567: Lanes<6>;
                        let v9568: Lanes<6>;
                        let v9569: Lanes<6>;
                        if v2549 != 0.0 {
                            let v2550 = v2548 / v2543;
                            let v19244 = ((v9566 * v2550) * v10390) / v2543;
                            let v2551 = v2517 * v2550;
                            let v19247 = (v19204 * v2550) + (v19244 * v2517);
                            let v2552 = v2523 * v2550;
                            let v19250 = (v19218 * v2550) + (v19244 * v2523);
                            let v2553 = v2528 * v2550;
                            let v19253 = (v19229 * v2550) + (v19244 * v2528);
                            v2554 = v2551;
                            v2556 = v2552;
                            v2558 = v2553;
                            v9567 = v19247;
                            v9568 = v19250;
                            v9569 = v19253;
                        } else {
                            v2554 = v2517;
                            v2556 = v2523;
                            v2558 = v2528;
                            v9567 = v19204;
                            v9568 = v19218;
                            v9569 = v19229;
                        }
                        let v2555 = v2299 + v2554;
                        let v19254 = v9536 + v9567;
                        let v2557 = v2321 + v2556;
                        let v19255 = v9537 + v9568;
                        let v2559 = v2263 + v2558;
                        let v19256 = v9535 + v9569;
                        let v2561 = if v2543 < (v861 * v2544) { 1.0 } else { 0.0 };
                        let v2568: f64;
                        if v2561 != 0.0 {
                            v2568 = v1;
                        } else {
                            v2568 = v2455;
                        }
                        v2562 = v2261;
                        v2564 = v2559;
                        v2565 = v2555;
                        v2566 = v2557;
                        v2567 = v2568;
                        v2574 = v2573;
                        v9562 = v19256;
                        v9563 = v19254;
                        v9564 = v19255;
                    }
                    let v2563 = v2562 + v1;
                    v2261 = v2563;
                    v2263 = v2564;
                    v2299 = v2565;
                    v2321 = v2566;
                    v2455 = v2567;
                    v2573 = v2574;
                    v2578 = v2319;
                    v2589 = v2453;
                    v2592 = v2454;
                    v2599 = v2458;
                    v9535 = v9562;
                    v9536 = v9563;
                    v9537 = v9564;
                    v9538 = v18960;
                    v9539 = v9559;
                    v9540 = v19119;
                    v9541 = v9542;
                }
                let v2575 = if v2573 > v0 { 1.0 } else { 0.0 };
                if v2575 != 0.0 {
                } else {
                }
                let v2576 = if v2455 == v0 { 1.0 } else { 0.0 };
                let v2577: f64;
                let v2603: f64;
                let v2604: f64;
                let v9570: Lanes<6>;
                let v9571: Lanes<6>;
                let v9572: Lanes<6>;
                if v2576 != 0.0 {
                    v2577 = v2258;
                    v2603 = v2259;
                    v2604 = v2260;
                    v9570 = v12799;
                    v9571 = v12800;
                    v9572 = v12798;
                } else {
                    v2577 = v2299;
                    v2603 = v2321;
                    v2604 = v2263;
                    v9570 = v9536;
                    v9571 = v9537;
                    v9572 = v9535;
                }
                let v2579 = -v2578;
                let v12802 = v9538 * v10390;
                let v2580 = if v2579 <= v362 { 1.0 } else { 0.0 };
                let v2581: f64;
                let v9573: Lanes<6>;
                if v2580 != 0.0 {
                    v2581 = v362;
                    v9573 = v11062;
                } else {
                    v2581 = v2579;
                    v9573 = v12802;
                }
                let v2582 = v2581 * v1048;
                let v12804 = v9420 * v2581;
                let v12806 = (v9573 * v1048) + (Lanes([v12804[0], v12804[1], 0.0, v12804[2], v12804[3], 0.0]));
                let v2584 = if (if v2577 <= v0 { 1.0 } else { 0.0 }) != 0.0 && v0 != 0.0 { 1.0 } else { 0.0 };
                let v3466: f64;
                let v3475: f64;
                let v4308: f64;
                let v4312: f64;
                let v4315: f64;
                let v4326: f64;
                let v4337: f64;
                let v4382: f64;
                let v4422: f64;
                let v4429: f64;
                let v4440: f64;
                let v4446: f64;
                let v4844: f64;
                let v5722: f64;
                let v8305: f64;
                let v8482: f64;
                let v8487: f64;
                let v8492: f64;
                let v8498: f64;
                let v9574: Lanes<6>;
                let v9575: Lanes<6>;
                let v9576: Lanes<6>;
                let v9577: Lanes<6>;
                let v9578: Lanes<6>;
                let v9579: Lanes<6>;
                let v9580: Lanes<6>;
                let v9581: Lanes<6>;
                let v9582: Lanes<6>;
                let v9583: Lanes<6>;
                let v9584: Lanes<6>;
                let v9585: Lanes<6>;
                let v9586: Lanes<6>;
                let v9587: Lanes<6>;
                let v9588: Lanes<6>;
                let v9589: Lanes<6>;
                if v2584 != 0.0 {
                    let v2588 = (-v168) * v139;
                    let v2594 = v2591 * ((v1504 + v2589) + v2592);
                    let v13743 = (((Lanes([v12078[0], v12078[1], v12078[2], v12078[3], v12078[4], 0.0])) + v9539) + v9540) * v2591;
                    let v2595 = v2588 * v2594;
                    let v13744 = v13743 * v2588;
                    let v2596 = v2595 * v13;
                    let v13745 = v13744 * v13;
                    let v2598 = v2595 * v2597;
                    let v13746 = v13744 * v2597;
                    let v2602 = (v2599 * v139) * v168;
                    let v13748 = (v9541 * v139) * v168;
                    v3466 = v2585;
                    v3475 = v0;
                    v4308 = v0;
                    v4312 = v0;
                    v4315 = v0;
                    v4326 = v1;
                    v4337 = v2577;
                    v4382 = v0;
                    v4422 = v2594;
                    v4429 = v0;
                    v4440 = v2599;
                    v4446 = v0;
                    v4844 = v0;
                    v5722 = v2603;
                    v8305 = v2577;
                    v8482 = v2595;
                    v8487 = v2602;
                    v8492 = v2596;
                    v8498 = v2598;
                    v9574 = v11062;
                    v9575 = v11062;
                    v9576 = v11062;
                    v9577 = v9570;
                    v9578 = v11062;
                    v9579 = v13743;
                    v9580 = v11062;
                    v9581 = v9541;
                    v9582 = v11062;
                    v9583 = v11062;
                    v9584 = v9571;
                    v9585 = v9570;
                    v9586 = v13744;
                    v9587 = v13748;
                    v9588 = v13745;
                    v9589 = v13746;
                } else {
                    let v2605 = v1128 * v1128;
                    let v12807 = v9421 * v1128;
                    let v2606 = v491 / v2605;
                    let v12811 = (((v12807 + v12807) * v2606) * v10390) / v2605;
                    let v2607 = v78 / v2606;
                    let v12814 = ((v12811 * v2607) * v10390) / v2606;
                    let v2608 = v1200 - v362;
                    let v12815 = v12814 * v2608;
                    let v12818 = (Lanes([v12815[0], v12815[1], 0.0, v12815[2], v12815[3]])) + (v10828 * v2607);
                    let v2610 = v1 + (v2607 * v2608);
                    let v2611 = v1 + v2607;
                    let v2614 = if (if v2610 < v2611 { 1.0 } else { 0.0 }) != 0.0 && (if v2611 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2646: f64;
                    let v9590: Lanes<5>;
                    if v2614 != 0.0 {
                        let v2615 = v2611 - v2610;
                        let v12819 = Lanes([v12814[0], v12814[1], 0.0, v12814[2], v12814[3]]);
                        let v12820 = v12819 - v12818;
                        let v2616 = v2615 * v2615;
                        let v12821 = v12820 * v2615;
                        let v12822 = v12821 + v12821;
                        let v2617 = v2611 * v2611;
                        let v12823 = v12814 * v2611;
                        let v12824 = v12823 + v12823;
                        let v2618 = v2616 * v2616;
                        let v12825 = v12822 * v2616;
                        let v2619 = v2617 * v2617;
                        let v12827 = v12824 * v2617;
                        let v2620 = v2618 * v2616;
                        let v2621 = v2619 * v2617;
                        let v12840 = ((((v12827 + v12827) * v2617) + (v12824 * v2619)) * v2617) + (v12824 * v2621);
                        let v2624 = (v2620 * v2616) + (v2621 * v2617);
                        let v12842 = (((((v12825 + v12825) * v2616) + (v12822 * v2618)) * v2616) + (v12822 * v2620)) + (Lanes([v12840[0], v12840[1], 0.0, v12840[2], v12840[3]]));
                        let v2641: f64;
                        let v9591: Lanes<5>;
                        if v2625 != 0.0 {
                            let v2635: f64;
                            if v2626 != 0.0 {
                                v2635 = v1;
                            } else {
                                let v2636: f64;
                                if v2627 != 0.0 {
                                    v2636 = v78;
                                } else {
                                    let v2637: f64;
                                    if v2628 != 0.0 {
                                        v2637 = v96;
                                    } else {
                                        let v2638: f64;
                                        if v2629 != 0.0 {
                                            v2638 = v90;
                                        } else {
                                            v2638 = v0;
                                        }
                                        v2637 = v2638;
                                    }
                                    v2636 = v2637;
                                }
                                v2635 = v2636;
                            }
                            let mut v2630: f64 = 0.0;
                            let mut v2632: f64 = 0.0;
                            let mut v9592: Lanes<5> = Lanes([0.0; 5]);
                            v2630 = v0;
                            v2632 = v2624;
                            v9592 = v12842;
                            loop {
                                let v2631 = if v2630 < v2635 { 1.0 } else { 0.0 };
                                if v2631 == 0.0 {
                                    break;
                                }
                                let v2633 = v2632.sqrt();
                                let v13739 = v9592 * (v9368 / (v10435 * v2633));
                                let v2634 = v2630 + v1;
                                v2630 = v2634;
                                v2632 = v2633;
                                v9592 = v13739;
                            }
                            v2641 = v2632;
                            v9591 = v9592;
                        } else {
                            let v2640 = v2624.powf(v2639);
                            let v12846 = v12842 * (v2639 * (v2624.powf(v12843)));
                            v2641 = v2640;
                            v9591 = v12846;
                        }
                        let v2642 = v1 / v2641;
                        let v2643 = v2615 * v2611;
                        let v12851 = v12814 * v2615;
                        let v2645 = v2611 - (v2643 * v2642);
                        let v12857 = v12819 - ((((v12820 * v2611) + (Lanes([v12851[0], v12851[1], 0.0, v12851[2], v12851[3]]))) * v2642) + ((((v9591 * v2642) * v10390) / v2641) * v2643));
                        v2646 = v2645;
                        v9590 = v12857;
                    } else {
                        v2646 = v2610;
                        v9590 = v12818;
                    }
                    let v2647 = v2646.sqrt();
                    let v2648 = v1 - v2647;
                    let v12862 = v12811 * v2648;
                    let v2650 = v1200 + (v2606 * v2648);
                    let v12866 = v10828 + ((Lanes([v12862[0], v12862[1], 0.0, v12862[2], v12862[3]])) + (((v9590 * (v9368 / (v10435 * v2647))) * v10390) * v2606));
                    let v12867 = v12866 * v2650;
                    let v2654 = ((v2650 * v2650) + v2652).sqrt();
                    let v12873 = (v12866 + ((v12867 + v12867) * (v9368 / (v10435 * v2654)))) * v13;
                    let v2658 = (v13 * (v2650 + v2654)) + v2657;
                    let v2659 = if v2658 < v0 { 1.0 } else { 0.0 };
                    let v2660: f64;
                    let v9593: Lanes<5>;
                    if v2659 != 0.0 {
                        v2660 = v0;
                        v9593 = v10579;
                    } else {
                        v2660 = v2658;
                        v9593 = v12873;
                    }
                    let v2661 = v823 / v2660;
                    let v12876 = (v10597 - (v9593 * v2661)) / v2660;
                    let v2663 = v2662 - v1;
                    let v2664 = v2661.powf(v2663);
                    let v12883 = ((v12876 * (v2663 * (v2661.powf((v2663 - v9368))))) * v2661) + (v12876 * v2664);
                    let v2666 = v1 + (v2664 * v2661);
                    let v2668 = (v1 / v2662) - v1;
                    let v2669 = v2666.powf(v2668);
                    let v2670 = v2669 * v2666;
                    let v2671 = v823 / v2670;
                    let v12893 = (v10597 - ((((v12883 * (v2668 * (v2666.powf((v2668 - v9368))))) * v2666) + (v12883 * v2669)) * v2671)) / v2670;
                    let v2672 = if v2671 < v0 { 1.0 } else { 0.0 };
                    let v3003: f64;
                    let v3008: f64;
                    let v3015: f64;
                    let v3330: f64;
                    let v3354: f64;
                    let v3467: f64;
                    let v9594: Lanes<6>;
                    let v9595: Lanes<6>;
                    let v9596: Lanes<6>;
                    let v9597: Lanes<6>;
                    if v2672 != 0.0 {
                        v3003 = v2603;
                        v3008 = v2577;
                        v3015 = v2604;
                        v3330 = v3331;
                        v3354 = v0;
                        v3467 = v2585;
                        v9594 = v9571;
                        v9595 = v9570;
                        v9596 = v9572;
                        v9597 = v11062;
                    } else {
                        let v3004: f64;
                        let v3009: f64;
                        let v3016: f64;
                        let v3332: f64;
                        let v3355: f64;
                        let v3468: f64;
                        let v9598: Lanes<6>;
                        let v9599: Lanes<6>;
                        let v9600: Lanes<6>;
                        let v9601: Lanes<6>;
                        if v2673 != 0.0 {
                            let v2674 = if v0 < v1512 { 1.0 } else { 0.0 };
                            let v2675: f64;
                            if v2674 != 0.0 {
                                v2675 = v1;
                            } else {
                                v2675 = v78;
                            }
                            v3004 = v0;
                            v3009 = v0;
                            v3016 = v0;
                            v3332 = v3331;
                            v3355 = v0;
                            v3468 = v2675;
                            v9598 = v11062;
                            v9599 = v11062;
                            v9600 = v11062;
                            v9601 = v11062;
                        } else {
                            let v2679 = v2676 - v2577;
                            let v12895 = (Lanes([v9504[0], v9504[1], v9504[2], v9504[3], v9504[4], 0.0])) - v9570;
                            let v2680 = if v2679 >= v0 { 1.0 } else { 0.0 };
                            let v2681: f64;
                            let v9602: Lanes<6>;
                            if v2680 != 0.0 {
                                v2681 = v2679;
                                v9602 = v12895;
                            } else {
                                v2681 = v0;
                                v9602 = v11062;
                            }
                            let v12897 = Lanes([v12893[0], v12893[1], v12893[2], v12893[3], v12893[4], 0.0]);
                            let v12898 = (v9602 * v2682) - v12897;
                            let v2685 = ((v2682 * v2681) - v2671) - v1985;
                            let v2689 = (v90 * (v2686 * v2681)) * v1985;
                            let v12901 = ((v9602 * v2686) * v90) * v1985;
                            let v2690 = if v2689 > v0 { 1.0 } else { 0.0 };
                            let v2692: f64;
                            let v9603: Lanes<6>;
                            if v2690 != 0.0 {
                                v2692 = v2689;
                                v9603 = v12901;
                            } else {
                                let v2691 = -v2689;
                                let v12902 = v12901 * v10390;
                                v2692 = v2691;
                                v9603 = v12902;
                            }
                            let v12903 = v12898 * v2685;
                            let v2695 = ((v2685 * v2685) + v2692).sqrt();
                            let v2700 = (v2696 * v2681) - (v13 * (v2685 + v2695));
                            let v12912 = (v9602 * v2696) - ((v12898 + (((v12903 + v12903) + v9603) * (v9368 / (v10435 * v2695)))) * v13);
                            let v2701 = if v2700 <= v2681 { 1.0 } else { 0.0 };
                            let v2702: f64;
                            let v9604: Lanes<6>;
                            if v2701 != 0.0 {
                                v2702 = v2700;
                                v9604 = v12912;
                            } else {
                                v2702 = v2681;
                                v9604 = v9602;
                            }
                            let v2703 = if v2702 < v0 { 1.0 } else { 0.0 };
                            let v2705: f64;
                            let v9605: Lanes<6>;
                            if v2703 != 0.0 {
                                v2705 = v0;
                                v9605 = v11062;
                            } else {
                                let v2704 = if v2702 > v2671 { 1.0 } else { 0.0 };
                                let v2706: f64;
                                let v9606: Lanes<6>;
                                if v2704 != 0.0 {
                                    v2706 = v2671;
                                    v9606 = v12897;
                                } else {
                                    v2706 = v2702;
                                    v9606 = v9604;
                                }
                                v2705 = v2706;
                                v9605 = v9606;
                            }
                            let v2707 = v2577 + v2705;
                            let v12913 = v9570 + v9605;
                            let v2708 = if v2707 < v1512 { 1.0 } else { 0.0 };
                            let v2880: f64;
                            let v9607: Lanes<6>;
                            if v2708 != 0.0 {
                                let v12964 = v11969 * v1248;
                                let v12966 = (v12964 + v12964) - v11974;
                                let v2710 = if v1253 >= v2709 { 1.0 } else { 0.0 };
                                let v2712: f64;
                                let v9608: Lanes<4>;
                                if v2710 != 0.0 {
                                    v2712 = v1253;
                                    v9608 = v12966;
                                } else {
                                    v2712 = v2711;
                                    v9608 = v10660;
                                }
                                let v2713 = v2712.sqrt();
                                let v2715 = (v1248 - v2713) / v78;
                                let v12971 = (v11969 - (v9608 * (v9368 / (v10435 * v2713)))) / v78;
                                let v12976 = ((((v11978 - v11980) / v1262) * v11981) - v11987) / v1266;
                                let v2716 = if v2715 < v1239 { 1.0 } else { 0.0 };
                                let v2881: f64;
                                let v9609: Lanes<4>;
                                if v2716 != 0.0 {
                                    v2881 = v2715;
                                    v9609 = v12971;
                                } else {
                                    let v12977 = v12976 - v12971;
                                    let v2718 = (v1267 - v2715) - v1270;
                                    let v2720 = (v90 * v1267) * v1270;
                                    let v12979 = (v12976 * v90) * v1270;
                                    let v2721 = if v2720 > v0 { 1.0 } else { 0.0 };
                                    let v2723: f64;
                                    let v9610: Lanes<4>;
                                    if v2721 != 0.0 {
                                        v2723 = v2720;
                                        v9610 = v12979;
                                    } else {
                                        let v2722 = -v2720;
                                        let v12980 = v12979 * v10390;
                                        v2723 = v2722;
                                        v9610 = v12980;
                                    }
                                    let v12981 = v12977 * v2718;
                                    let v2726 = ((v2718 * v2718) + v2723).sqrt();
                                    let v2729 = v1267 - (v13 * (v2718 + v2726));
                                    let v12989 = v12976 - ((v12977 + (((v12981 + v12981) + v9610) * (v9368 / (v10435 * v2726)))) * v13);
                                    v2881 = v2729;
                                    v9609 = v12989;
                                }
                                let v12990 = Lanes([v9609[0], v9609[1], v9609[2], 0.0, v9609[3], 0.0]);
                                v2880 = v2881;
                                v9607 = v12990;
                            } else {
                                let v2735 = -((v1244 - v2707) - (((v1228 / v78) * v12) / v123));
                                let v12916 = ((Lanes([v9469[0], v9469[1], v9469[2], 0.0, v9469[3], 0.0])) - v12913) * v10390;
                                let v2737 = (v78 * v2735) + v1247;
                                let v12919 = (v12916 * v78) + (Lanes([0.0, 0.0, v11967[0], 0.0, 0.0, 0.0]));
                                let v12920 = v12919 * v2737;
                                let v2739 = v2735 * v2735;
                                let v12922 = v12916 * v2735;
                                let v12923 = v12922 + v12922;
                                let v2742 = (v2737 * v2737) - (v90 * (v2739 + v1243));
                                let v12927 = (v12920 + v12920) - ((v12923 + (Lanes([0.0, 0.0, v11962[0], 0.0, 0.0, 0.0]))) * v90);
                                let v2744 = if v2742 >= v2743 { 1.0 } else { 0.0 };
                                let v2746: f64;
                                let v9611: Lanes<6>;
                                if v2744 != 0.0 {
                                    v2746 = v2742;
                                    v9611 = v12927;
                                } else {
                                    v2746 = v2745;
                                    v9611 = v11062;
                                }
                                let v2747 = v2746.sqrt();
                                let v2749 = (v2737 - v2747) / v78;
                                let v12932 = (v12919 - (v9611 * (v9368 / (v10435 * v2747)))) / v78;
                                let v2750 = v2739 / v1243;
                                let v12933 = v11962 * v2750;
                                let v2751 = v2750 / v1262;
                                let v12937 = v9405 * v2751;
                                let v2753 = v78 / v2735;
                                let v2754 = v663 + v2753;
                                let v2755 = (v2751.ln()) / v2754;
                                let v12950 = ((((((v12923 - (Lanes([0.0, 0.0, v12933[0], 0.0, 0.0, 0.0]))) / v1243) - (Lanes([0.0, 0.0, v12937[0], 0.0, 0.0, 0.0]))) / v1262) * (v9368 / v2751)) - (((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0, 0.0])) + (((v12916 * v2753) * v10390) / v2735)) * v2755)) / v2754;
                                let v2756 = if v2749 < v1239 { 1.0 } else { 0.0 };
                                let v2882: f64;
                                let v9612: Lanes<6>;
                                if v2756 != 0.0 {
                                    v2882 = v2749;
                                    v9612 = v12932;
                                } else {
                                    let v12951 = v12950 - v12932;
                                    let v2758 = (v2755 - v2749) - v1270;
                                    let v2760 = (v90 * v2755) * v1270;
                                    let v12953 = (v12950 * v90) * v1270;
                                    let v2761 = if v2760 > v0 { 1.0 } else { 0.0 };
                                    let v2763: f64;
                                    let v9613: Lanes<6>;
                                    if v2761 != 0.0 {
                                        v2763 = v2760;
                                        v9613 = v12953;
                                    } else {
                                        let v2762 = -v2760;
                                        let v12954 = v12953 * v10390;
                                        v2763 = v2762;
                                        v9613 = v12954;
                                    }
                                    let v12955 = v12951 * v2758;
                                    let v2766 = ((v2758 * v2758) + v2763).sqrt();
                                    let v2769 = v2755 - (v13 * (v2758 + v2766));
                                    let v12963 = v12950 - ((v12951 + (((v12955 + v12955) + v9613) * (v9368 / (v10435 * v2766)))) * v13);
                                    v2882 = v2769;
                                    v9612 = v12963;
                                }
                                v2880 = v2882;
                                v9607 = v9612;
                            }
                            let v2773 = if ((v2770 * v2707) / v477) > v0 { 1.0 } else { 0.0 };
                            let v3333: f64;
                            if v2773 != 0.0 {
                                let v2777 = ((v2774 * v2707) / v477).sqrt();
                                v3333 = v2777;
                            } else {
                                v3333 = v0;
                            }
                            let v2778 = if v2708 != 0.0 && v0 != 0.0 { 1.0 } else { 0.0 };
                            let v3000: f64;
                            let v3017: f64;
                            let v3356: f64;
                            let v3469: f64;
                            let v9614: Lanes<6>;
                            let v9615: Lanes<6>;
                            let v9616: Lanes<6>;
                            if v2778 != 0.0 {
                                let mut v2779: f64 = 0.0;
                                let mut v2781: f64 = 0.0;
                                let mut v2884: f64 = 0.0;
                                let mut v9617: Lanes<6> = Lanes([0.0; 6]);
                                let mut v9618: Lanes<6> = Lanes([0.0; 6]);
                                v2779 = v0;
                                v2781 = v2880;
                                v2884 = v0;
                                v9617 = v9607;
                                v9618 = v11062;
                                loop {
                                    let v2780 = if v2779 < v18 { 1.0 } else { 0.0 };
                                    if v2780 == 0.0 {
                                        break;
                                    }
                                    let v2782 = v663 * v2781;
                                    let v13127 = v10410 * v2781;
                                    let v13130 = (Lanes([0.0, 0.0, v13127[0], 0.0, 0.0, 0.0])) + (v9617 * v663);
                                    let v2784 = (-v2782).exp();
                                    let v13132 = (v13130 * v10390) * v2784;
                                    let v2785 = if v2781 > v616 { 1.0 } else { 0.0 };
                                    let v2819: f64;
                                    let v2852: f64;
                                    let v9619: Lanes<6>;
                                    let v9620: Lanes<6>;
                                    if v2785 != 0.0 {
                                        let v2786 = v2782.exp();
                                        let v2787 = -v1240;
                                        let v2790 = v2786 - v1;
                                        let v13171 = v9405 * v2790;
                                        let v13172 = (v13130 * v2786) * v1262;
                                        let v2793 = (((v2784 + v2782) - v1) + (v1262 * v2790)).sqrt();
                                        let v2794 = v2787 * v2793;
                                        let v13179 = (v9404 * v10390) * v2793;
                                        let v13182 = (Lanes([0.0, 0.0, v13179[0], 0.0, 0.0, 0.0])) + ((((v13132 + v13130) + ((Lanes([0.0, 0.0, v13171[0], 0.0, 0.0, 0.0])) + v13172)) * (v9368 / (v10435 * v2793))) * v2787);
                                        let v2795 = v211 / v2794;
                                        let v13187 = v9405 * v2786;
                                        let v2799 = ((-v2784) + v1) + (v1262 * v2786);
                                        let v2800 = v2795 * v2799;
                                        let v13193 = ((((v13182 * v2795) * v10390) / v2794) * v2799) + (((v13132 * v10390) + ((Lanes([0.0, 0.0, v13187[0], 0.0, 0.0, 0.0])) + v13172)) * v2795);
                                        v2819 = v2794;
                                        v2852 = v2800;
                                        v9619 = v13182;
                                        v9620 = v13193;
                                    } else {
                                        let v2802 = if v2781 < v2801 { 1.0 } else { 0.0 };
                                        let v2820: f64;
                                        let v2853: f64;
                                        let v9621: Lanes<6>;
                                        let v9622: Lanes<6>;
                                        if v2802 != 0.0 {
                                            let v2805 = ((v2784 + v2782) - v1).sqrt();
                                            let v2806 = v1240 * v2805;
                                            let v13157 = v9404 * v2805;
                                            let v13160 = (Lanes([0.0, 0.0, v13157[0], 0.0, 0.0, 0.0])) + (((v13132 + v13130) * (v9368 / (v10435 * v2805))) * v1240);
                                            let v2807 = v211 / v2806;
                                            let v2809 = (-v2784) + v1;
                                            let v2810 = v2807 * v2809;
                                            let v13167 = ((((v13160 * v2807) * v10390) / v2806) * v2809) + ((v13132 * v10390) * v2807);
                                            v2820 = v2806;
                                            v2853 = v2810;
                                            v9621 = v13160;
                                            v9622 = v13167;
                                        } else {
                                            let v2811 = v211 / v663;
                                            let v2812 = v2811.sqrt();
                                            let v2813 = -v2812;
                                            let v2814 = v2813 * v663;
                                            let v2815 = v2814 * v2781;
                                            let v13143 = (((((((v10410 * v2811) * v10390) / v663) * (v9368 / (v10435 * v2812))) * v10390) * v663) + (v10410 * v2813)) * v2781;
                                            let v13146 = (Lanes([0.0, 0.0, v13143[0], 0.0, 0.0, 0.0])) + (v9617 * v2814);
                                            let v2817 = (v211 * v663).sqrt();
                                            let v2818 = -v2817;
                                            let v13151 = ((v10410 * v211) * (v9368 / (v10435 * v2817))) * v10390;
                                            let v13152 = Lanes([0.0, 0.0, v13151[0], 0.0, 0.0, 0.0]);
                                            v2820 = v2815;
                                            v2853 = v2818;
                                            v9621 = v13146;
                                            v9622 = v13152;
                                        }
                                        v2819 = v2820;
                                        v2852 = v2853;
                                        v9619 = v9621;
                                        v9620 = v9622;
                                    }
                                    let v13194 = v9619 * v2819;
                                    let v2825 = ((v2819 * v2819) + ((v90 * v1230) * v1230)).sqrt();
                                    let v13198 = (v13194 + v13194) * (v9368 / (v10435 * v2825));
                                    let v2826 = v2819 / v2825;
                                    let v2828 = v13 * (v1 + v2826);
                                    let v13202 = ((v9619 - (v13198 * v2826)) / v2825) * v13;
                                    let v13204 = (v9619 + v13198) * v13;
                                    let v2832 = (v13 * (v2819 + v2825)) + (v535 * v1230);
                                    let v2833 = if v2832 < v0 { 1.0 } else { 0.0 };
                                    let v2834: f64;
                                    let v2851: f64;
                                    let v9623: Lanes<6>;
                                    let v9624: Lanes<6>;
                                    if v2833 != 0.0 {
                                        v2834 = v0;
                                        v2851 = v0;
                                        v9623 = v11062;
                                        v9624 = v11062;
                                    } else {
                                        v2834 = v2832;
                                        v2851 = v2828;
                                        v9623 = v13204;
                                        v9624 = v13202;
                                    }
                                    let v13205 = v9623 * v10390;
                                    let v2836 = (v1229 - v2834) - v1232;
                                    let v2838 = (v90 * v1229) * v1232;
                                    let v2839 = if v2838 > v0 { 1.0 } else { 0.0 };
                                    let v2841: f64;
                                    if v2839 != 0.0 {
                                        v2841 = v2838;
                                    } else {
                                        let v2840 = -v2838;
                                        v2841 = v2840;
                                    }
                                    let v13206 = v13205 * v2836;
                                    let v2844 = ((v2836 * v2836) + v2841).sqrt();
                                    let v13210 = (v13206 + v13206) * (v9368 / (v10435 * v2844));
                                    let v2845 = v2836 / v2844;
                                    let v2847 = v13 * (v1 + v2845);
                                    let v2850 = v1229 - (v13 * (v2836 + v2844));
                                    let v13217 = ((v13205 + v13210) * v13) * v10390;
                                    let v2854 = v2852 * v2847;
                                    let v2855 = v2851 * v2854;
                                    let v13224 = v13217 * v2850;
                                    let v2860 = ((((v2850 * v2850) / v78) / v123) / v206) / v477;
                                    let v13229 = ((((v13224 + v13224) / v78) / v123) / v206) / v477;
                                    let v2861 = v78 * v2860;
                                    let v2863 = (v2861 * v2855) / v2850;
                                    let v2872 = (v2869 + (v2852 / v130)) + v2863;
                                    let v2873 = ((((-v2781) + (v2819 / v130)) - v1244) + v2860) / v2872;
                                    let v2874 = v2781 - v2873;
                                    let v13248 = v9617 - ((((((v9617 * v10390) + (v9619 / v130)) - (Lanes([v9469[0], v9469[1], v9469[2], 0.0, v9469[3], 0.0]))) + v13229) - (((v9620 / v130) + (((((v13229 * v78) * v2855) + (((v9624 * v2854) + (((v9620 * v2847) + ((((v13205 - (v13210 * v2845)) / v2844) * v13) * v2852)) * v2851)) * v2861)) - (v13217 * v2863)) / v2850)) * v2873)) / v2872);
                                    let v2877 = if ((v2874 - v2781).abs()) < v861 { 1.0 } else { 0.0 };
                                    let v2878: f64;
                                    if v2877 != 0.0 {
                                        v2878 = v18;
                                    } else {
                                        v2878 = v2779;
                                    }
                                    let v2879 = v2878 + v1;
                                    v2779 = v2879;
                                    v2781 = v2874;
                                    v2884 = v2819;
                                    v9617 = v13248;
                                    v9618 = v9619;
                                }
                                let v2883 = v1244 + v2781;
                                let v13124 = (Lanes([v9469[0], v9469[1], v9469[2], 0.0, v9469[3], 0.0])) + v9617;
                                let v2886 = v2883 - (v2884 / v130);
                                let v13126 = v13124 - (v9618 / v130);
                                v3000 = v2886;
                                v3017 = v2883;
                                v3356 = v2884;
                                v3469 = v1;
                                v9614 = v13126;
                                v9615 = v13124;
                                v9616 = v9618;
                            } else {
                                let mut v2887: f64 = 0.0;
                                let mut v2889: f64 = 0.0;
                                let mut v2997: f64 = 0.0;
                                let mut v9625: Lanes<6> = Lanes([0.0; 6]);
                                let mut v9626: Lanes<6> = Lanes([0.0; 6]);
                                v2887 = v0;
                                v2889 = v2880;
                                v2997 = v0;
                                v9625 = v9607;
                                v9626 = v11062;
                                loop {
                                    let v2888 = if v2887 < v18 { 1.0 } else { 0.0 };
                                    if v2888 == 0.0 {
                                        break;
                                    }
                                    let v2890 = v663 * v2889;
                                    let v12995 = v10410 * v2889;
                                    let v12998 = (Lanes([0.0, 0.0, v12995[0], 0.0, 0.0, 0.0])) + (v9625 * v663);
                                    let v2892 = (-v2890).exp();
                                    let v13000 = (v12998 * v10390) * v2892;
                                    let v2893 = if v2889 > v616 { 1.0 } else { 0.0 };
                                    let v2927: f64;
                                    let v2960: f64;
                                    let v9627: Lanes<6>;
                                    let v9628: Lanes<6>;
                                    if v2893 != 0.0 {
                                        let v2894 = v2890.exp();
                                        let v2895 = -v1240;
                                        let v2898 = v2894 - v1;
                                        let v13039 = v9405 * v2898;
                                        let v13040 = (v12998 * v2894) * v1262;
                                        let v2901 = (((v2892 + v2890) - v1) + (v1262 * v2898)).sqrt();
                                        let v2902 = v2895 * v2901;
                                        let v13047 = (v9404 * v10390) * v2901;
                                        let v13050 = (Lanes([0.0, 0.0, v13047[0], 0.0, 0.0, 0.0])) + ((((v13000 + v12998) + ((Lanes([0.0, 0.0, v13039[0], 0.0, 0.0, 0.0])) + v13040)) * (v9368 / (v10435 * v2901))) * v2895);
                                        let v2903 = v211 / v2902;
                                        let v13055 = v9405 * v2894;
                                        let v2907 = ((-v2892) + v1) + (v1262 * v2894);
                                        let v2908 = v2903 * v2907;
                                        let v13061 = ((((v13050 * v2903) * v10390) / v2902) * v2907) + (((v13000 * v10390) + ((Lanes([0.0, 0.0, v13055[0], 0.0, 0.0, 0.0])) + v13040)) * v2903);
                                        v2927 = v2902;
                                        v2960 = v2908;
                                        v9627 = v13050;
                                        v9628 = v13061;
                                    } else {
                                        let v2910 = if v2889 < v2909 { 1.0 } else { 0.0 };
                                        let v2928: f64;
                                        let v2961: f64;
                                        let v9629: Lanes<6>;
                                        let v9630: Lanes<6>;
                                        if v2910 != 0.0 {
                                            let v2913 = ((v2892 + v2890) - v1).sqrt();
                                            let v2914 = v1240 * v2913;
                                            let v13025 = v9404 * v2913;
                                            let v13028 = (Lanes([0.0, 0.0, v13025[0], 0.0, 0.0, 0.0])) + (((v13000 + v12998) * (v9368 / (v10435 * v2913))) * v1240);
                                            let v2915 = v211 / v2914;
                                            let v2917 = (-v2892) + v1;
                                            let v2918 = v2915 * v2917;
                                            let v13035 = ((((v13028 * v2915) * v10390) / v2914) * v2917) + ((v13000 * v10390) * v2915);
                                            v2928 = v2914;
                                            v2961 = v2918;
                                            v9629 = v13028;
                                            v9630 = v13035;
                                        } else {
                                            let v2919 = v211 / v663;
                                            let v2920 = v2919.sqrt();
                                            let v2921 = -v2920;
                                            let v2922 = v2921 * v663;
                                            let v2923 = v2922 * v2889;
                                            let v13011 = (((((((v10410 * v2919) * v10390) / v663) * (v9368 / (v10435 * v2920))) * v10390) * v663) + (v10410 * v2921)) * v2889;
                                            let v13014 = (Lanes([0.0, 0.0, v13011[0], 0.0, 0.0, 0.0])) + (v9625 * v2922);
                                            let v2925 = (v211 * v663).sqrt();
                                            let v2926 = -v2925;
                                            let v13019 = ((v10410 * v211) * (v9368 / (v10435 * v2925))) * v10390;
                                            let v13020 = Lanes([0.0, 0.0, v13019[0], 0.0, 0.0, 0.0]);
                                            v2928 = v2923;
                                            v2961 = v2926;
                                            v9629 = v13014;
                                            v9630 = v13020;
                                        }
                                        v2927 = v2928;
                                        v2960 = v2961;
                                        v9627 = v9629;
                                        v9628 = v9630;
                                    }
                                    let v13062 = v9627 * v2927;
                                    let v2933 = ((v2927 * v2927) + ((v90 * v1230) * v1230)).sqrt();
                                    let v13066 = (v13062 + v13062) * (v9368 / (v10435 * v2933));
                                    let v2934 = v2927 / v2933;
                                    let v2936 = v13 * (v1 + v2934);
                                    let v13070 = ((v9627 - (v13066 * v2934)) / v2933) * v13;
                                    let v13072 = (v9627 + v13066) * v13;
                                    let v2940 = (v13 * (v2927 + v2933)) + (v535 * v1230);
                                    let v2941 = if v2940 < v0 { 1.0 } else { 0.0 };
                                    let v2942: f64;
                                    let v2959: f64;
                                    let v9631: Lanes<6>;
                                    let v9632: Lanes<6>;
                                    if v2941 != 0.0 {
                                        v2942 = v0;
                                        v2959 = v0;
                                        v9631 = v11062;
                                        v9632 = v11062;
                                    } else {
                                        v2942 = v2940;
                                        v2959 = v2936;
                                        v9631 = v13072;
                                        v9632 = v13070;
                                    }
                                    let v13073 = v9631 * v10390;
                                    let v2944 = (v1229 - v2942) - v1232;
                                    let v2946 = (v90 * v1229) * v1232;
                                    let v2947 = if v2946 > v0 { 1.0 } else { 0.0 };
                                    let v2949: f64;
                                    if v2947 != 0.0 {
                                        v2949 = v2946;
                                    } else {
                                        let v2948 = -v2946;
                                        v2949 = v2948;
                                    }
                                    let v13074 = v13073 * v2944;
                                    let v2952 = ((v2944 * v2944) + v2949).sqrt();
                                    let v13078 = (v13074 + v13074) * (v9368 / (v10435 * v2952));
                                    let v2953 = v2944 / v2952;
                                    let v2955 = v13 * (v1 + v2953);
                                    let v2958 = v1229 - (v13 * (v2944 + v2952));
                                    let v13085 = ((v13073 + v13078) * v13) * v10390;
                                    let v2962 = v2960 * v2955;
                                    let v2963 = v2959 * v2962;
                                    let v13092 = v13085 * v2958;
                                    let v2968 = ((((v2958 * v2958) / v78) / v123) / v206) / v477;
                                    let v13097 = ((((v13092 + v13092) / v78) / v123) / v206) / v477;
                                    let v2969 = v78 * v2968;
                                    let v2971 = (v2969 * v2963) / v2958;
                                    let v2988 = ((v2982 + (v2960 / v130)) + ((v2960 * v12) / v123)) + v2971;
                                    let v2989 = (((((v2707 - v2889) + (v2927 / v130)) + (((v2927 + (v1228 / v78)) * v12) / v123)) - v1244) + v2968) / v2988;
                                    let v2990 = v2889 - v2989;
                                    let v13122 = v9625 - (((((((v12913 - v9625) + (v9627 / v130)) + ((v9627 * v12) / v123)) - (Lanes([v9469[0], v9469[1], v9469[2], 0.0, v9469[3], 0.0]))) + v13097) - ((((v9628 / v130) + ((v9628 * v12) / v123)) + (((((v13097 * v78) * v2963) + (((v9632 * v2962) + (((v9628 * v2955) + ((((v13073 - (v13078 * v2953)) / v2952) * v13) * v2960)) * v2959)) * v2969)) - (v13085 * v2971)) / v2958)) * v2989)) / v2988);
                                    let v2993 = if ((v2990 - v2889).abs()) < v861 { 1.0 } else { 0.0 };
                                    let v2994: f64;
                                    if v2993 != 0.0 {
                                        v2994 = v18;
                                    } else {
                                        v2994 = v2887;
                                    }
                                    let v2995 = v2994 + v1;
                                    v2887 = v2995;
                                    v2889 = v2990;
                                    v2997 = v2927;
                                    v9625 = v13122;
                                    v9626 = v9627;
                                }
                                let v2996 = v1244 + v2889;
                                let v12992 = (Lanes([v9469[0], v9469[1], v9469[2], 0.0, v9469[3], 0.0])) + v9625;
                                let v2999 = v2996 - (v2997 / v130);
                                let v12994 = v12992 - (v9626 / v130);
                                v3000 = v2999;
                                v3017 = v2996;
                                v3356 = v2997;
                                v3469 = v78;
                                v9614 = v12994;
                                v9615 = v12992;
                                v9616 = v9626;
                            }
                            let v3001 = if v3000 < v0 { 1.0 } else { 0.0 };
                            let v3005: f64;
                            let v9633: Lanes<6>;
                            if v3001 != 0.0 {
                                v3005 = v0;
                                v9633 = v11062;
                            } else {
                                v3005 = v3000;
                                v9633 = v9614;
                            }
                            v3004 = v3005;
                            v3009 = v2707;
                            v3016 = v3017;
                            v3332 = v3333;
                            v3355 = v3356;
                            v3468 = v3469;
                            v9598 = v9633;
                            v9599 = v12913;
                            v9600 = v9615;
                            v9601 = v9616;
                        }
                        v3003 = v3004;
                        v3008 = v3009;
                        v3015 = v3016;
                        v3330 = v3332;
                        v3354 = v3355;
                        v3467 = v3468;
                        v9594 = v9598;
                        v9595 = v9599;
                        v9596 = v9600;
                        v9597 = v9601;
                    }
                    let v3002 = if v2577 < v0 { 1.0 } else { 0.0 };
                    let v3007: f64;
                    let v9634: Lanes<6>;
                    if v3002 != 0.0 {
                        v3007 = v2577;
                        v9634 = v9570;
                    } else {
                        v3007 = v3008;
                        v9634 = v9595;
                    }
                    let v3006 = if v3003 < v20 { 1.0 } else { 0.0 };
                    let v3014: f64;
                    let v9635: Lanes<6>;
                    if v3006 != 0.0 {
                        let v3013 = v3007 + (v125 * ((v13 * v1228) + v2599));
                        let v13250 = v9634 + (v9541 * v125);
                        v3014 = v3013;
                        v9635 = v13250;
                    } else {
                        v3014 = v3003;
                        v9635 = v9594;
                    }
                    let mut v3018: f64 = 0.0;
                    let mut v3020: f64 = 0.0;
                    let mut v3056: f64 = 0.0;
                    let mut v3079: f64 = 0.0;
                    let mut v3212: f64 = 0.0;
                    let mut v3324: f64 = 0.0;
                    let mut v3335: f64 = 0.0;
                    let mut v3346: f64 = 0.0;
                    let mut v3353: f64 = 0.0;
                    let mut v9636: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9637: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9638: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9639: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9640: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9641: Lanes<6> = Lanes([0.0; 6]);
                    v3018 = v1;
                    v3020 = v3015;
                    v3056 = v3007;
                    v3079 = v3014;
                    v3212 = v0;
                    v3324 = v0;
                    v3335 = v0;
                    v3346 = v0;
                    v3353 = v3354;
                    v9636 = v9596;
                    v9637 = v9634;
                    v9638 = v9635;
                    v9639 = v11062;
                    v9640 = v11062;
                    v9641 = v9597;
                    loop {
                        let v3019 = if v3018 <= v18 { 1.0 } else { 0.0 };
                        if v3019 == 0.0 {
                            break;
                        }
                        let v3021 = v3020 - v1244;
                        let v3022 = v663 * v3021;
                        let v13336 = v10410 * v3021;
                        let v13339 = (Lanes([0.0, 0.0, v13336[0], 0.0, 0.0, 0.0])) + ((v9636 - (Lanes([v9469[0], v9469[1], v9469[2], 0.0, v9469[3], 0.0]))) * v663);
                        let v3024 = (-v3022).exp();
                        let v13341 = (v13339 * v10390) * v3024;
                        let v3026 = if v3021 < v3025 { 1.0 } else { 0.0 };
                        let v3217: f64;
                        let v3230: f64;
                        let v9642: Lanes<6>;
                        let v9643: Lanes<6>;
                        if v3026 != 0.0 {
                            let v3029 = ((v3024 + v3022) - v1).sqrt();
                            let v3030 = v1240 * v3029;
                            let v13381 = v9404 * v3029;
                            let v13384 = (Lanes([0.0, 0.0, v13381[0], 0.0, 0.0, 0.0])) + (((v13341 + v13339) * (v9368 / (v10435 * v3029))) * v1240);
                            let v3034 = (v211 * ((-v3024) + v1)) / v3030;
                            let v13389 = (((v13341 * v10390) * v211) - (v13384 * v3034)) / v3030;
                            v3217 = v3030;
                            v3230 = v3034;
                            v9642 = v13384;
                            v9643 = v13389;
                        } else {
                            let v3035 = if v3021 > v616 { 1.0 } else { 0.0 };
                            let v3218: f64;
                            let v3231: f64;
                            let v9644: Lanes<6>;
                            let v9645: Lanes<6>;
                            if v3035 != 0.0 {
                                let v3036 = v3022.exp();
                                let v13351 = v13339 * v3036;
                                let v3037 = -v1240;
                                let v3041 = (v3036 + v3022) - v1;
                                let v13355 = v9405 * v3041;
                                let v3044 = (((v3024 + v3022) - v1) + (v1262 * v3041)).sqrt();
                                let v3045 = v3037 * v3044;
                                let v13363 = (v9404 * v10390) * v3044;
                                let v13366 = (Lanes([0.0, 0.0, v13363[0], 0.0, 0.0, 0.0])) + ((((v13341 + v13339) + ((Lanes([0.0, 0.0, v13355[0], 0.0, 0.0, 0.0])) + ((v13351 + v13339) * v1262))) * (v9368 / (v10435 * v3044))) * v3037);
                                let v3048 = v3036 + v1;
                                let v13368 = v9405 * v3048;
                                let v3052 = (v211 * (((-v3024) + v1) + (v1262 * v3048))) / v3045;
                                let v13376 = ((((v13341 * v10390) + ((Lanes([0.0, 0.0, v13368[0], 0.0, 0.0, 0.0])) + (v13351 * v1262))) * v211) - (v13366 * v3052)) / v3045;
                                v3218 = v3045;
                                v3231 = v3052;
                                v9644 = v13366;
                                v9645 = v13376;
                            } else {
                                let v3053 = -v1240;
                                let v13342 = v9404 * v10390;
                                let v3054 = v3053 * v3022;
                                let v13343 = v13342 * v3022;
                                let v13346 = (Lanes([0.0, 0.0, v13343[0], 0.0, 0.0, 0.0])) + (v13339 * v3053);
                                let v3055 = v3053 * v663;
                                let v13349 = (v13342 * v663) + (v10410 * v3053);
                                let v13350 = Lanes([0.0, 0.0, v13349[0], 0.0, 0.0, 0.0]);
                                v3218 = v3054;
                                v3231 = v3055;
                                v9644 = v13346;
                                v9645 = v13350;
                            }
                            v3217 = v3218;
                            v3230 = v3231;
                            v9642 = v9644;
                            v9643 = v9645;
                        }
                        let v3057 = v3056 - v2671;
                        let v13392 = v10410 * v3057;
                        let v3059 = (v663 * v3057).exp();
                        let v13396 = ((Lanes([0.0, 0.0, v13392[0], 0.0, 0.0, 0.0])) + ((v9637 - (Lanes([v12893[0], v12893[1], v12893[2], v12893[3], v12893[4], 0.0]))) * v663)) * v3059;
                        let v13397 = v12078 * v1504;
                        let v3061 = v750 * v750;
                        let v13399 = v10485 * v750;
                        let v3062 = (v1504 * v1504) / v3061;
                        let v13401 = (v13399 + v13399) * v3062;
                        let v13404 = ((v13397 + v13397) - (Lanes([0.0, 0.0, v13401[0], 0.0, 0.0]))) / v3061;
                        let v3063 = v78 * v759;
                        let v3065 = (v3059 + v3022) - v1;
                        let v13407 = (v10496 * v78) * v3065;
                        let v3068 = (v3062 + (v3063 * v3065)).sqrt();
                        let v13415 = ((Lanes([v13404[0], v13404[1], v13404[2], v13404[3], v13404[4], 0.0])) + ((Lanes([0.0, 0.0, v13407[0], 0.0, 0.0, 0.0])) + ((v13396 + v13339) * v3063))) * (v9368 / (v10435 * v3068));
                        let v3069 = v78 * v663;
                        let v3070 = v3069 * v759;
                        let v3071 = v3059 + v1;
                        let v13420 = (((v10410 * v78) * v759) + (v10496 * v3069)) * v3071;
                        let v3073 = v78 * v3068;
                        let v3074 = (v3070 * v3071) / v3073;
                        let v3075 = -v750;
                        let v13428 = v10485 * v10390;
                        let v13429 = v13428 * v3068;
                        let v3077 = (v3075 * v3068) - v1504;
                        let v13433 = Lanes([v12078[0], v12078[1], v12078[2], v12078[3], v12078[4], 0.0]);
                        let v13434 = ((Lanes([0.0, 0.0, v13429[0], 0.0, 0.0, 0.0])) + (v13415 * v3075)) - v13433;
                        let v3078 = v3075 * v3074;
                        let v13435 = v13428 * v3074;
                        let v13438 = (Lanes([0.0, 0.0, v13435[0], 0.0, 0.0, 0.0])) + (((((Lanes([0.0, 0.0, v13420[0], 0.0, 0.0, 0.0])) + (v13396 * v3070)) - ((v13415 * v78) * v3074)) / v3073) * v3075);
                        let v3081 = (v3079 - v3056) / v1208;
                        let v3082 = v663 * v3081;
                        let v13441 = v10410 * v3081;
                        let v13444 = (Lanes([0.0, 0.0, v13441[0], 0.0, 0.0, 0.0])) + (((v9638 - v9637) / v1208) * v663);
                        let v3083 = -v3082;
                        let v13445 = v13444 * v10390;
                        let v3084 = if v3083 >= v2326 { 1.0 } else { 0.0 };
                        let v3095: f64;
                        let v3103: f64;
                        let v9646: Lanes<6>;
                        let v9647: Lanes<6>;
                        if v3084 != 0.0 {
                            let v3087 = v2328 * ((v1 + v3083) - v2326);
                            let v13448 = v13445 * v2328;
                            v3095 = v3087;
                            v3103 = v2328;
                            v9646 = v13448;
                            v9647 = v11062;
                        } else {
                            let mut v3088: f64 = 0.0;
                            let mut v3090: f64 = 0.0;
                            let mut v9648: Lanes<6> = Lanes([0.0; 6]);
                            v3088 = v3083;
                            v3090 = v1;
                            v9648 = v13445;
                            loop {
                                let v3089 = if v3088 >= v2330 { 1.0 } else { 0.0 };
                                if v3089 == 0.0 {
                                    break;
                                }
                                let v3091 = v3090 * v2333;
                                let v3092 = v3088 - v2330;
                                let edge0 = v3092;
                                let edge1 = v3091;
                                let edge2 = v9648;
                                v3088 = edge0;
                                v3090 = edge1;
                                v9648 = edge2;
                            }
                            let v3093 = v3088.exp();
                            let v3094 = v3090 * v3093;
                            let v13447 = (v9648 * v3093) * v3090;
                            v3095 = v3094;
                            v3103 = v3094;
                            v9646 = v13447;
                            v9647 = v13447;
                        }
                        let v3098 = ((v3095 + v3082) - v1).sqrt();
                        let v13452 = (v9646 + v13444) * (v9368 / (v10435 * v3098));
                        let v3100 = if v3081 < v3099 { 1.0 } else { 0.0 };
                        let v3126: f64;
                        let v3163: f64;
                        let v3167: f64;
                        let v9649: Lanes<6>;
                        let v9650: Lanes<6>;
                        let v9651: Lanes<6>;
                        if v3100 != 0.0 {
                            let v3101 = v750 * v3098;
                            let v13483 = v10485 * v3098;
                            let v13486 = (Lanes([0.0, 0.0, v13483[0], 0.0, 0.0, 0.0])) + (v13452 * v750);
                            let v3102 = v750 * v663;
                            let v3105 = (-v3103) + v1;
                            let v13491 = ((v10485 * v663) + (v10410 * v750)) * v3105;
                            let v3107 = v78 * v3098;
                            let v3108 = (v3102 * v3105) / v3107;
                            let v3109 = v3108 / v1208;
                            let v13499 = ((((Lanes([0.0, 0.0, v13491[0], 0.0, 0.0, 0.0])) + ((v9647 * v10390) * v3102)) - ((v13452 * v78) * v3108)) / v3107) / v1208;
                            let v3110 = -v3109;
                            let v13500 = v13499 * v10390;
                            v3126 = v3101;
                            v3163 = v3109;
                            v3167 = v3110;
                            v9649 = v13486;
                            v9650 = v13499;
                            v9651 = v13500;
                        } else {
                            let v3111 = if v3081 > v616 { 1.0 } else { 0.0 };
                            let v3127: f64;
                            let v3164: f64;
                            let v3168: f64;
                            let v9652: Lanes<6>;
                            let v9653: Lanes<6>;
                            let v9654: Lanes<6>;
                            if v3111 != 0.0 {
                                let v3112 = v3075 * v3098;
                                let v13465 = v13428 * v3098;
                                let v13468 = (Lanes([0.0, 0.0, v13465[0], 0.0, 0.0, 0.0])) + (v13452 * v3075);
                                let v3113 = v3075 * v663;
                                let v3115 = (-v3103) + v1;
                                let v13473 = ((v13428 * v663) + (v10410 * v3075)) * v3115;
                                let v3117 = v78 * v3098;
                                let v3118 = (v3113 * v3115) / v3117;
                                let v3119 = v3118 / v1208;
                                let v13481 = ((((Lanes([0.0, 0.0, v13473[0], 0.0, 0.0, 0.0])) + ((v9647 * v10390) * v3113)) - ((v13452 * v78) * v3118)) / v3117) / v1208;
                                let v3120 = -v3119;
                                let v13482 = v13481 * v10390;
                                v3127 = v3112;
                                v3164 = v3119;
                                v3168 = v3120;
                                v9652 = v13468;
                                v9653 = v13481;
                                v9654 = v13482;
                            } else {
                                let v13453 = v13428 * v3082;
                                let v3122 = (v3075 * v3082) / v748;
                                let v13457 = ((Lanes([0.0, 0.0, v13453[0], 0.0, 0.0, 0.0])) + (v13444 * v3075)) / v748;
                                let v3124 = (v3075 * v663) / v748;
                                let v13461 = ((v13428 * v663) + (v10410 * v3075)) / v748;
                                let v3125 = -v3124;
                                let v13462 = v13461 * v10390;
                                let v13463 = Lanes([0.0, 0.0, v13461[0], 0.0, 0.0, 0.0]);
                                let v13464 = Lanes([0.0, 0.0, v13462[0], 0.0, 0.0, 0.0]);
                                v3127 = v3122;
                                v3164 = v3124;
                                v3168 = v3125;
                                v9652 = v13457;
                                v9653 = v13463;
                                v9654 = v13464;
                            }
                            v3126 = v3127;
                            v3163 = v3164;
                            v3167 = v3168;
                            v9649 = v9652;
                            v9650 = v9653;
                            v9651 = v9654;
                        }
                        let v3128 = -v1225;
                        let v13501 = v11952 * v10390;
                        let v3129 = v0 - v3128;
                        let v13502 = v13501 * v10390;
                        let v3132 = if (if v3126 > v3129 { 1.0 } else { 0.0 }) != 0.0 && (if v3128 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3165: f64;
                        let v3170: f64;
                        let v9655: Lanes<6>;
                        let v9656: Lanes<6>;
                        if v3132 != 0.0 {
                            let v3133 = v3126 + v3128;
                            let v13504 = v9649 + (Lanes([v13501[0], v13501[1], v13501[2], v13501[3], v13501[4], 0.0]));
                            let v3134 = v3133 * v3133;
                            let v13505 = v13504 * v3133;
                            let v3135 = v3128 * v3128;
                            let v13507 = v13501 * v3128;
                            let v13509 = (v13505 + v13505) * v3134;
                            let v3137 = v3135 * v3135;
                            let v13511 = (v13507 + v13507) * v3135;
                            let v13512 = v13511 + v13511;
                            let v3138 = (v3134 * v3134) + v3137;
                            let v13514 = (v13509 + v13509) + (Lanes([v13512[0], v13512[1], v13512[2], v13512[3], v13512[4], 0.0]));
                            let v3155: f64;
                            let v9657: Lanes<6>;
                            if v3139 != 0.0 {
                                let v3149: f64;
                                if v3140 != 0.0 {
                                    v3149 = v1;
                                } else {
                                    let v3150: f64;
                                    if v3141 != 0.0 {
                                        v3150 = v78;
                                    } else {
                                        let v3151: f64;
                                        if v3142 != 0.0 {
                                            v3151 = v96;
                                        } else {
                                            let v3152: f64;
                                            if v3143 != 0.0 {
                                                v3152 = v90;
                                            } else {
                                                v3152 = v0;
                                            }
                                            v3151 = v3152;
                                        }
                                        v3150 = v3151;
                                    }
                                    v3149 = v3150;
                                }
                                let mut v3144: f64 = 0.0;
                                let mut v3146: f64 = 0.0;
                                let mut v9658: Lanes<6> = Lanes([0.0; 6]);
                                v3144 = v0;
                                v3146 = v3138;
                                v9658 = v13514;
                                loop {
                                    let v3145 = if v3144 < v3149 { 1.0 } else { 0.0 };
                                    if v3145 == 0.0 {
                                        break;
                                    }
                                    let v3147 = v3146.sqrt();
                                    let v13736 = v9658 * (v9368 / (v10435 * v3147));
                                    let v3148 = v3144 + v1;
                                    v3144 = v3148;
                                    v3146 = v3147;
                                    v9658 = v13736;
                                }
                                v3155 = v3146;
                                v9657 = v9658;
                            } else {
                                let v3154 = v3138.powf(v3153);
                                let v13518 = v13514 * (v3153 * (v3138.powf(v13515)));
                                v3155 = v3154;
                                v9657 = v13518;
                            }
                            let v3156 = v1 / v3155;
                            let v13521 = ((v9657 * v3156) * v10390) / v3155;
                            let v3157 = v3133 * v3128;
                            let v13523 = v13501 * v3133;
                            let v3159 = v3128 * v3137;
                            let v13532 = ((v13501 * v3137) + (v13512 * v3128)) * v3156;
                            let v3161 = (v3159 * v3156) / v3138;
                            let v13538 = (((Lanes([v13532[0], v13532[1], v13532[2], v13532[3], v13532[4], 0.0])) + (v13521 * v3159)) - (v13514 * v3161)) / v3138;
                            let v3162 = v3129 + (v3157 * v3156);
                            let v13540 = (Lanes([v13502[0], v13502[1], v13502[2], v13502[3], v13502[4], 0.0])) + ((((v13504 * v3128) + (Lanes([v13523[0], v13523[1], v13523[2], v13523[3], v13523[4], 0.0]))) * v3156) + (v13521 * v3157));
                            v3165 = v3161;
                            v3170 = v3162;
                            v9655 = v13538;
                            v9656 = v13540;
                        } else {
                            v3165 = v1;
                            v3170 = v3126;
                            v9655 = v11062;
                            v9656 = v9649;
                        }
                        let v3166 = v3163 * v3165;
                        let v13543 = (v9650 * v3165) + (v9655 * v3163);
                        let v3169 = v3167 * v3165;
                        let v13546 = (v9651 * v3165) + (v9655 * v3167);
                        let v3171 = v1228 - v1504;
                        let v13547 = v12078 * v10390;
                        let v3172 = -v3171;
                        let v13548 = v13547 * v10390;
                        let v3173 = v3171 + v3172;
                        let v13549 = v13547 + v13548;
                        let v3176 = if (if v3170 < v3173 { 1.0 } else { 0.0 }) != 0.0 && (if v3172 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3207: f64;
                        let v3210: f64;
                        let v9659: Lanes<6>;
                        let v9660: Lanes<6>;
                        if v3176 != 0.0 {
                            let v3177 = v3173 - v3170;
                            let v13550 = Lanes([v13549[0], v13549[1], v13549[2], v13549[3], v13549[4], 0.0]);
                            let v13551 = v13550 - v9656;
                            let v3178 = v3177 * v3177;
                            let v13552 = v13551 * v3177;
                            let v3179 = v3172 * v3172;
                            let v13554 = v13548 * v3172;
                            let v13556 = (v13552 + v13552) * v3178;
                            let v3181 = v3179 * v3179;
                            let v13558 = (v13554 + v13554) * v3179;
                            let v13559 = v13558 + v13558;
                            let v3182 = (v3178 * v3178) + v3181;
                            let v13561 = (v13556 + v13556) + (Lanes([v13559[0], v13559[1], v13559[2], v13559[3], v13559[4], 0.0]));
                            let v3199: f64;
                            let v9661: Lanes<6>;
                            if v3183 != 0.0 {
                                let v3193: f64;
                                if v3184 != 0.0 {
                                    v3193 = v1;
                                } else {
                                    let v3194: f64;
                                    if v3185 != 0.0 {
                                        v3194 = v78;
                                    } else {
                                        let v3195: f64;
                                        if v3186 != 0.0 {
                                            v3195 = v96;
                                        } else {
                                            let v3196: f64;
                                            if v3187 != 0.0 {
                                                v3196 = v90;
                                            } else {
                                                v3196 = v0;
                                            }
                                            v3195 = v3196;
                                        }
                                        v3194 = v3195;
                                    }
                                    v3193 = v3194;
                                }
                                let mut v3188: f64 = 0.0;
                                let mut v3190: f64 = 0.0;
                                let mut v9662: Lanes<6> = Lanes([0.0; 6]);
                                v3188 = v0;
                                v3190 = v3182;
                                v9662 = v13561;
                                loop {
                                    let v3189 = if v3188 < v3193 { 1.0 } else { 0.0 };
                                    if v3189 == 0.0 {
                                        break;
                                    }
                                    let v3191 = v3190.sqrt();
                                    let v13733 = v9662 * (v9368 / (v10435 * v3191));
                                    let v3192 = v3188 + v1;
                                    v3188 = v3192;
                                    v3190 = v3191;
                                    v9662 = v13733;
                                }
                                v3199 = v3190;
                                v9661 = v9662;
                            } else {
                                let v3198 = v3182.powf(v3197);
                                let v13565 = v13561 * (v3197 * (v3182.powf(v13562)));
                                v3199 = v3198;
                                v9661 = v13565;
                            }
                            let v3200 = v1 / v3199;
                            let v13568 = ((v9661 * v3200) * v10390) / v3199;
                            let v3201 = v3177 * v3172;
                            let v13570 = v13548 * v3177;
                            let v3203 = v3172 * v3181;
                            let v13579 = ((v13548 * v3181) + (v13559 * v3172)) * v3200;
                            let v3205 = (v3203 * v3200) / v3182;
                            let v13585 = (((Lanes([v13579[0], v13579[1], v13579[2], v13579[3], v13579[4], 0.0])) + (v13568 * v3203)) - (v13561 * v3205)) / v3182;
                            let v3206 = v3173 - (v3201 * v3200);
                            let v13586 = v13550 - ((((v13551 * v3172) + (Lanes([v13570[0], v13570[1], v13570[2], v13570[3], v13570[4], 0.0]))) * v3200) + (v13568 * v3201));
                            v3207 = v3205;
                            v3210 = v3206;
                            v9659 = v13585;
                            v9660 = v13586;
                        } else {
                            v3207 = v1;
                            v3210 = v3170;
                            v9659 = v11062;
                            v9660 = v9656;
                        }
                        let v3208 = v3169 * v3207;
                        let v13589 = (v13546 * v3207) + (v9659 * v3169);
                        let v3209 = v3166 * v3207;
                        let v13592 = (v13543 * v3207) + (v9659 * v3166);
                        let v3211 = v1504 + v3210;
                        let v13593 = v13433 + v9660;
                        let v3215 = if (if v3212 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v3018 > v96 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3317: f64;
                        let v3319: f64;
                        let v3320: f64;
                        let v3321: f64;
                        let v3322: f64;
                        let v3325: f64;
                        let v9663: Lanes<6>;
                        let v9664: Lanes<6>;
                        let v9665: Lanes<6>;
                        if v3215 != 0.0 {
                            v3317 = v18;
                            v3319 = v3020;
                            v3320 = v3056;
                            v3321 = v3079;
                            v3322 = v3212;
                            v3325 = v3018;
                            v9663 = v9636;
                            v9664 = v9637;
                            v9665 = v9638;
                        } else {
                            let v3222 = (((v3217 + v1504) + v3077) + v3210) + v2569;
                            let v13600 = v9420 * v3222;
                            let v3224 = (v3056 - v1200) - (v1048 * v3222);
                            let v13604 = (v9637 - (Lanes([v10828[0], v10828[1], v10828[2], v10828[3], v10828[4], 0.0]))) - ((Lanes([v13600[0], v13600[1], 0.0, v13600[2], v13600[3], 0.0])) + (((((v9642 + v13433) + v13434) + v9660) + v9503) * v1048));
                            let v3225 = v3078 + v3208;
                            let v13606 = v9420 * v3225;
                            let v3227 = v1 - (v1048 * v3225);
                            let v13610 = ((Lanes([v13606[0], v13606[1], 0.0, v13606[2], v13606[3], 0.0])) + ((v13438 + v13589) * v1048)) * v10390;
                            let v3228 = -v1048;
                            let v13611 = v9420 * v10390;
                            let v3229 = v3228 * v3209;
                            let v13612 = v13611 * v3209;
                            let v13615 = (Lanes([v13612[0], v13612[1], 0.0, v13612[2], v13612[3], 0.0])) + (v13592 * v3228);
                            let v3232 = v3228 * v3230;
                            let v13616 = v13611 * v3230;
                            let v13619 = (Lanes([v13616[0], v13616[1], 0.0, v13616[2], v13616[3], 0.0])) + (v9643 * v3228);
                            let v3238 = v3079 - (v3056 + (v125 * ((v13 * v1228) + v3217)));
                            let v13623 = v9638 - (v9637 + (v9642 * v125));
                            let v3240 = -(v125 * v3230);
                            let v13624 = (v9643 * v125) * v10390;
                            let v3243 = (v3020 - v3079) - (v131 * v3217);
                            let v13627 = (v9636 - v9638) - (v9642 * v131);
                            let v3246 = v1 - (v131 * v3230);
                            let v13629 = (v9643 * v131) * v10390;
                            let v3247 = v3227 * v3246;
                            let v13632 = (v13610 * v3246) + (v13629 * v3227);
                            let v3248 = v3227 * v3240;
                            let v13635 = (v13610 * v3240) + (v13624 * v3227);
                            let v3251 = v3229 * v3239;
                            let v13638 = v13615 * v3239;
                            let v3254 = v3232 * v3239;
                            let v13643 = v13619 * v3239;
                            let v3257 = (((v3247 - (v3248 * v3244)) - (v3251 * v3246)) + (v3254 * v3244)) + v362;
                            let v3258 = v1 / v3257;
                            let v3260 = v3246 - (v3240 * v3244);
                            let v3263 = (v3232 * v3244) - (v3229 * v3246);
                            let v3265 = (v3229 * v3240) - v3232;
                            let v3266 = v3254 - v3248;
                            let v3268 = (-v3227) * v3244;
                            let v3269 = v3227 - v3251;
                            let v3270 = -v3258;
                            let v13664 = ((((((v13632 - (v13635 * v3244)) - ((v13638 * v3246) + (v13629 * v3251))) + (v13643 * v3244)) * v3258) * v10390) / v3257) * v10390;
                            let v3275 = ((v3260 * v3224) + (v3263 * v3238)) + (v3265 * v3243);
                            let v3276 = v3270 * v3275;
                            let v13678 = (v13664 * v3275) + ((((((v13629 - (v13624 * v3244)) * v3224) + (v13604 * v3260)) + ((((v13619 * v3244) - ((v13615 * v3246) + (v13629 * v3229))) * v3238) + (v13623 * v3263))) + (((((v13615 * v3240) + (v13624 * v3229)) - v13619) * v3243) + (v13627 * v3265))) * v3270);
                            let v3281 = ((v3246 * v3224) + (v3247 * v3238)) + (v3266 * v3243);
                            let v3282 = v3270 * v3281;
                            let v13692 = (v13664 * v3281) + (((((v13629 * v3224) + (v13604 * v3246)) + ((v13632 * v3238) + (v13623 * v3247))) + (((v13643 - v13635) * v3243) + (v13627 * v3266))) * v3270);
                            let v3286 = (v3224 + (v3268 * v3238)) + (v3269 * v3243);
                            let v3287 = v3270 * v3286;
                            let v13703 = (v13664 * v3286) + (((v13604 + ((((v13610 * v10390) * v3244) * v3238) + (v13623 * v3268))) + (((v13610 - v13638) * v3243) + (v13627 * v3269))) * v3270);
                            let v3288 = v3276.abs();
                            let v13707 = v13678 * ((v10435 * (if v3276 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                            let v3289 = v3282.abs();
                            let v13711 = v13692 * ((v10435 * (if v3282 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                            let v3290 = if v3288 < v3289 { 1.0 } else { 0.0 };
                            let v3291: f64;
                            let v9666: Lanes<6>;
                            if v3290 != 0.0 {
                                v3291 = v3289;
                                v9666 = v13711;
                            } else {
                                v3291 = v3288;
                                v9666 = v13707;
                            }
                            let v3292 = v3287.abs();
                            let v13715 = v13703 * ((v10435 * (if v3287 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                            let v3293 = if v3291 < v3292 { 1.0 } else { 0.0 };
                            let v3298: f64;
                            let v9667: Lanes<6>;
                            if v3293 != 0.0 {
                                v3298 = v3292;
                                v9667 = v13715;
                            } else {
                                v3298 = v3291;
                                v9667 = v9666;
                            }
                            let v3294 = if v3018 > v2535 { 1.0 } else { 0.0 };
                            let v3299: f64;
                            if v3294 != 0.0 {
                                v3299 = v2537;
                            } else {
                                let v3295 = if v3018 > v2538 { 1.0 } else { 0.0 };
                                let v3300: f64;
                                if v3295 != 0.0 {
                                    v3300 = v2537;
                                } else {
                                    let v3296 = if v3018 > v821 { 1.0 } else { 0.0 };
                                    let v3301: f64;
                                    if v3296 != 0.0 {
                                        v3301 = v2541;
                                    } else {
                                        let v3297 = if v3018 > v15 { 1.0 } else { 0.0 };
                                        let v3302: f64;
                                        if v3297 != 0.0 {
                                            v3302 = v644;
                                        } else {
                                            v3302 = v1;
                                        }
                                        v3301 = v3302;
                                    }
                                    v3300 = v3301;
                                }
                                v3299 = v3300;
                            }
                            let v3303 = v79 / v3299;
                            let v3304 = if v3298 > v3303 { 1.0 } else { 0.0 };
                            let v3309: f64;
                            let v3311: f64;
                            let v3313: f64;
                            let v9668: Lanes<6>;
                            let v9669: Lanes<6>;
                            let v9670: Lanes<6>;
                            if v3304 != 0.0 {
                                let v3305 = v3303 / v3298;
                                let v13718 = ((v9667 * v3305) * v10390) / v3298;
                                let v3306 = v3276 * v3305;
                                let v13721 = (v13678 * v3305) + (v13718 * v3276);
                                let v3307 = v3282 * v3305;
                                let v13724 = (v13692 * v3305) + (v13718 * v3282);
                                let v3308 = v3287 * v3305;
                                let v13727 = (v13703 * v3305) + (v13718 * v3287);
                                v3309 = v3306;
                                v3311 = v3307;
                                v3313 = v3308;
                                v9668 = v13721;
                                v9669 = v13724;
                                v9670 = v13727;
                            } else {
                                v3309 = v3276;
                                v3311 = v3282;
                                v3313 = v3287;
                                v9668 = v13678;
                                v9669 = v13692;
                                v9670 = v13703;
                            }
                            let v3310 = v3056 + v3309;
                            let v13728 = v9637 + v9668;
                            let v3312 = v3079 + v3311;
                            let v13729 = v9638 + v9669;
                            let v3314 = v3020 + v3313;
                            let v13730 = v9636 + v9670;
                            let v3316 = if v3298 < (v861 * v3299) { 1.0 } else { 0.0 };
                            let v3323: f64;
                            if v3316 != 0.0 {
                                v3323 = v1;
                            } else {
                                v3323 = v3212;
                            }
                            v3317 = v3018;
                            v3319 = v3314;
                            v3320 = v3310;
                            v3321 = v3312;
                            v3322 = v3323;
                            v3325 = v3324;
                            v9663 = v13730;
                            v9664 = v13728;
                            v9665 = v13729;
                        }
                        let v3318 = v3317 + v1;
                        v3018 = v3318;
                        v3020 = v3319;
                        v3056 = v3320;
                        v3079 = v3321;
                        v3212 = v3322;
                        v3324 = v3325;
                        v3335 = v3077;
                        v3346 = v3211;
                        v3353 = v3217;
                        v9636 = v9663;
                        v9637 = v9664;
                        v9638 = v9665;
                        v9639 = v13434;
                        v9640 = v13593;
                        v9641 = v9642;
                    }
                    let v3326 = if v3324 > v0 { 1.0 } else { 0.0 };
                    if v3326 != 0.0 {
                    } else {
                    }
                    let v3327 = if v3212 == v0 { 1.0 } else { 0.0 };
                    let v3328: f64;
                    let v5723: f64;
                    let v9671: Lanes<6>;
                    let v9672: Lanes<6>;
                    if v3327 != 0.0 {
                        v3328 = v3007;
                        v5723 = v3014;
                        v9671 = v9634;
                        v9672 = v9635;
                    } else {
                        v3328 = v3056;
                        v5723 = v3079;
                        v9671 = v9637;
                        v9672 = v9638;
                    }
                    let v4327: f64;
                    if v3002 != 0.0 {
                        v4327 = v1;
                    } else {
                        v4327 = v0;
                    }
                    let v3329 = v3328 - v2577;
                    let v13251 = v9671 - v9570;
                    let v3334 = v3330 / v123;
                    let v3336 = v3335 - v2578;
                    let v13252 = v9639 - v9538;
                    let v3337 = v3335 + v2578;
                    let v13253 = v9639 + v9538;
                    let v3338 = v663 * v3337;
                    let v13254 = v10410 * v3337;
                    let v3341 = v3336 - ((v3338 * v3329) * v13);
                    let v13262 = v13252 - (((((Lanes([0.0, 0.0, v13254[0], 0.0, 0.0, 0.0])) + (v13253 * v663)) * v3329) + (v13251 * v3338)) * v13);
                    let v3344 = if (if v3341 < v0 { 1.0 } else { 0.0 }) != 0.0 || (if v823 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4383: f64;
                    let v9673: Lanes<6>;
                    if v3344 != 0.0 {
                        v4383 = v0;
                        v9673 = v11062;
                    } else {
                        v4383 = v3341;
                        v9673 = v13262;
                    }
                    let v3348 = v3345 * (v3346 + v2592);
                    let v13264 = (v9640 + v9540) * v3345;
                    let v3349 = v3329 + v861;
                    let v3362 = v1228 * v1231;
                    let v3364 = if v3362 >= v0 { 1.0 } else { 0.0 };
                    let v3365 = if (if (-(((v3353 * v3353) - (v2599 * v2599)) / (v130 / ((v130 * v3334) + v1)))) < v3362 { 1.0 } else { 0.0 }) != 0.0 && v3364 != 0.0 { 1.0 } else { 0.0 };
                    if v3365 != 0.0 {
                        if v3366 != 0.0 {
                            let v3374: f64;
                            if v3367 != 0.0 {
                                v3374 = v1;
                            } else {
                                let v3375: f64;
                                if v3368 != 0.0 {
                                    v3375 = v78;
                                } else {
                                    let v3376: f64;
                                    if v3369 != 0.0 {
                                        v3376 = v96;
                                    } else {
                                        let v3377: f64;
                                        if v3370 != 0.0 {
                                            v3377 = v90;
                                        } else {
                                            v3377 = v0;
                                        }
                                        v3376 = v3377;
                                    }
                                    v3375 = v3376;
                                }
                                v3374 = v3375;
                            }
                            let mut v3371: f64 = 0.0;
                            v3371 = v0;
                            loop {
                                let v3372 = if v3371 < v3374 { 1.0 } else { 0.0 };
                                if v3372 == 0.0 {
                                    break;
                                }
                                let v3373 = v3371 + v1;
                                v3371 = v3373;
                            }
                        } else {
                        }
                    } else {
                    }
                    let v3380 = if ((v663 * v2604) - v1) > v0 { 1.0 } else { 0.0 };
                    if v3380 != 0.0 {
                    } else {
                    }
                    let v3381 = -v3336;
                    let v13265 = v13252 * v10390;
                    let v3383 = if (if v3381 < v3362 { 1.0 } else { 0.0 }) != 0.0 && v3364 != 0.0 { 1.0 } else { 0.0 };
                    let v3411: f64;
                    let v9674: Lanes<6>;
                    if v3383 != 0.0 {
                        let v3384 = v3362 - v3381;
                        let v13266 = v13265 * v10390;
                        let v3385 = v3384 * v3384;
                        let v13267 = v13266 * v3384;
                        let v3386 = v3362 * v3362;
                        let v13269 = (v13267 + v13267) * v3385;
                        let v13270 = v13269 + v13269;
                        let v3389 = (v3385 * v3385) + (v3386 * v3386);
                        let v3406: f64;
                        let v9675: Lanes<6>;
                        if v3390 != 0.0 {
                            let v3400: f64;
                            if v3391 != 0.0 {
                                v3400 = v1;
                            } else {
                                let v3401: f64;
                                if v3392 != 0.0 {
                                    v3401 = v78;
                                } else {
                                    let v3402: f64;
                                    if v3393 != 0.0 {
                                        v3402 = v96;
                                    } else {
                                        let v3403: f64;
                                        if v3394 != 0.0 {
                                            v3403 = v90;
                                        } else {
                                            v3403 = v0;
                                        }
                                        v3402 = v3403;
                                    }
                                    v3401 = v3402;
                                }
                                v3400 = v3401;
                            }
                            let mut v3395: f64 = 0.0;
                            let mut v3397: f64 = 0.0;
                            let mut v9676: Lanes<6> = Lanes([0.0; 6]);
                            v3395 = v0;
                            v3397 = v3389;
                            v9676 = v13270;
                            loop {
                                let v3396 = if v3395 < v3400 { 1.0 } else { 0.0 };
                                if v3396 == 0.0 {
                                    break;
                                }
                                let v3398 = v3397.sqrt();
                                let v13333 = v9676 * (v9368 / (v10435 * v3398));
                                let v3399 = v3395 + v1;
                                v3395 = v3399;
                                v3397 = v3398;
                                v9676 = v13333;
                            }
                            v3406 = v3397;
                            v9675 = v9676;
                        } else {
                            let v3405 = v3389.powf(v3404);
                            let v13274 = v13270 * (v3404 * (v3389.powf(v13271)));
                            v3406 = v3405;
                            v9675 = v13274;
                        }
                        let v3407 = v1 / v3406;
                        let v3408 = v3384 * v3362;
                        let v3410 = v3362 - (v3408 * v3407);
                        let v13282 = (((v13266 * v3362) * v3407) + ((((v9675 * v3407) * v10390) / v3406) * v3408)) * v10390;
                        v3411 = v3410;
                        v9674 = v13282;
                    } else {
                        v3411 = v3381;
                        v9674 = v13265;
                    }
                    let v3414 = v663 * v1128;
                    let v13285 = v10410 * v1128;
                    let v13286 = v9421 * v663;
                    let v3415 = v3414 * v3349;
                    let v13290 = ((Lanes([0.0, 0.0, v13285[0], 0.0, 0.0])) + (Lanes([v13286[0], v13286[1], 0.0, v13286[2], v13286[3]]))) * v3349;
                    let v3416 = v3415 * v3349;
                    let v3417 = (v78 * (-v3411)) / v3416;
                    let v3418 = v1 + v3417;
                    let v3420 = (v3418 * v3349) / v2582;
                    let v3421 = v1 - v3420;
                    let v13306 = ((((((((v9674 * v10390) * v78) - (((((Lanes([v13290[0], v13290[1], v13290[2], v13290[3], v13290[4], 0.0])) + (v13251 * v3414)) * v3349) + (v13251 * v3415)) * v3417)) / v3416) * v3349) + (v13251 * v3418)) - (v12806 * v3420)) / v2582) * v10390;
                    let v3425 = if (if v3421 < v3422 { 1.0 } else { 0.0 }) != 0.0 && v3424 != 0.0 { 1.0 } else { 0.0 };
                    let v3454: f64;
                    let v9677: Lanes<6>;
                    if v3425 != 0.0 {
                        let v3427 = v3426 - v3421;
                        let v13307 = v13306 * v10390;
                        let v3428 = v3427 * v3427;
                        let v13308 = v13307 * v3427;
                        let v13310 = (v13308 + v13308) * v3428;
                        let v13311 = v13310 + v13310;
                        let v3431 = (v3428 * v3428) + v3430;
                        let v3448: f64;
                        let v9678: Lanes<6>;
                        if v3432 != 0.0 {
                            let v3442: f64;
                            if v3433 != 0.0 {
                                v3442 = v1;
                            } else {
                                let v3443: f64;
                                if v3434 != 0.0 {
                                    v3443 = v78;
                                } else {
                                    let v3444: f64;
                                    if v3435 != 0.0 {
                                        v3444 = v96;
                                    } else {
                                        let v3445: f64;
                                        if v3436 != 0.0 {
                                            v3445 = v90;
                                        } else {
                                            v3445 = v0;
                                        }
                                        v3444 = v3445;
                                    }
                                    v3443 = v3444;
                                }
                                v3442 = v3443;
                            }
                            let mut v3437: f64 = 0.0;
                            let mut v3439: f64 = 0.0;
                            let mut v9679: Lanes<6> = Lanes([0.0; 6]);
                            v3437 = v0;
                            v3439 = v3431;
                            v9679 = v13311;
                            loop {
                                let v3438 = if v3437 < v3442 { 1.0 } else { 0.0 };
                                if v3438 == 0.0 {
                                    break;
                                }
                                let v3440 = v3439.sqrt();
                                let v13330 = v9679 * (v9368 / (v10435 * v3440));
                                let v3441 = v3437 + v1;
                                v3437 = v3441;
                                v3439 = v3440;
                                v9679 = v13330;
                            }
                            v3448 = v3439;
                            v9678 = v9679;
                        } else {
                            let v3447 = v3431.powf(v3446);
                            let v13315 = v13311 * (v3446 * (v3431.powf(v13312)));
                            v3448 = v3447;
                            v9678 = v13315;
                        }
                        let v3449 = v1 / v3448;
                        let v3450 = v3427 * v1231;
                        let v3453 = v3452 - (v3450 * v3449);
                        let v13323 = (((v13307 * v1231) * v3449) + ((((v9678 * v3449) * v10390) / v3448) * v3450)) * v10390;
                        v3454 = v3453;
                        v9677 = v13323;
                    } else {
                        v3454 = v3421;
                        v9677 = v13306;
                    }
                    let v3455 = v1 + v3454;
                    let v13326 = (v9677 * v3455) + (v9677 * v3454);
                    let v3457 = v1 + (v3454 * v3455);
                    let v3459 = if v3455 >= v3458 { 1.0 } else { 0.0 };
                    let v3461: f64;
                    let v9680: Lanes<6>;
                    if v3459 != 0.0 {
                        v3461 = v3455;
                        v9680 = v9677;
                    } else {
                        v3461 = v3460;
                        v9680 = v11062;
                    }
                    let v3463 = v3462 * v3337;
                    let v13327 = v13253 * v3462;
                    v3466 = v3467;
                    v3475 = v3212;
                    v4308 = v3454;
                    v4312 = v3461;
                    v4315 = v3457;
                    v4326 = v4327;
                    v4337 = v3328;
                    v4382 = v4383;
                    v4422 = v3348;
                    v4429 = v3463;
                    v4440 = v3353;
                    v4446 = v3329;
                    v4844 = v2582;
                    v5722 = v5723;
                    v8305 = v0;
                    v8482 = v0;
                    v8487 = v0;
                    v8492 = v0;
                    v8498 = v0;
                    v9574 = v9677;
                    v9575 = v9680;
                    v9576 = v13326;
                    v9577 = v9671;
                    v9578 = v9673;
                    v9579 = v13264;
                    v9580 = v13327;
                    v9581 = v9641;
                    v9582 = v13251;
                    v9583 = v12806;
                    v9584 = v9672;
                    v9585 = v11062;
                    v9586 = v11062;
                    v9587 = v11062;
                    v9588 = v11062;
                    v9589 = v11062;
                }
                let v3464 = if v70 >= v1 { 1.0 } else { 0.0 };
                if v3464 != 0.0 {
                    let v3471 = if (if v2585 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v3466 == v78 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v3471 != 0.0 {
                    } else {
                    }
                    let v3474 = if (if v2585 == v78 { 1.0 } else { 0.0 }) != 0.0 && (if v3466 == v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v3474 != 0.0 {
                    } else {
                    }
                } else {
                }
                if v2576 != 0.0 {
                } else {
                }
                let v3476 = if v3475 == v0 { 1.0 } else { 0.0 };
                if v3476 != 0.0 {
                } else {
                }
                let v3478 = if (v2455 + v3475) < v1 { 1.0 } else { 0.0 };
                if v3478 != 0.0 {
                } else {
                }
                v4305 = v0;
                v4307 = v4308;
                v4311 = v4312;
                v4314 = v4315;
                v4325 = v4326;
                v4336 = v4337;
                v4340 = v2577;
                v4348 = v2581;
                v4381 = v4382;
                v4421 = v4422;
                v4428 = v4429;
                v4438 = v2599;
                v4439 = v4440;
                v4445 = v4446;
                v4637 = v2603;
                v4735 = v4736;
                v4787 = v4788;
                v4843 = v4844;
                v4964 = v1574;
                v4973 = v1244;
                v4977 = v1504;
                v5093 = v5094;
                v5501 = v2569;
                v5643 = v5644;
                v5721 = v5722;
                v5781 = v5782;
                v8304 = v8305;
                v8481 = v8482;
                v8486 = v8487;
                v8491 = v8492;
                v8497 = v8498;
                v8564 = v0;
                v8576 = v0;
                v9211 = v9212;
                v9437 = v9574;
                v9438 = v9575;
                v9439 = v9576;
                v9440 = v9577;
                v9441 = v9570;
                v9442 = v9573;
                v9443 = v9578;
                v9444 = v9579;
                v9445 = v9580;
                v9446 = v9541;
                v9447 = v9581;
                v9448 = v9582;
                v9449 = v9571;
                v9450 = v9505;
                v9451 = v9506;
                v9452 = v9583;
                v9453 = v9470;
                v9454 = v9469;
                v9455 = v12078;
                v9456 = v9483;
                v9457 = v9503;
                v9458 = v9507;
                v9459 = v9584;
                v9460 = v9585;
                v9461 = v9586;
                v9462 = v9587;
                v9463 = v9588;
                v9464 = v9589;
                v9465 = v11062;
                v9466 = v11062;
                v9467 = v9508;
            } else {
                let v3479 = if v769 < v12 { 1.0 } else { 0.0 };
                let v4189: f64;
                if v3479 != 0.0 {
                    v4189 = v1;
                } else {
                    v4189 = v78;
                }
                let v10837 = Lanes([v9413[0], v9413[1], 0.0, 0.0, v9413[2]]);
                let v3481 = if v830 < (v1205 + v835) { 1.0 } else { 0.0 };
                let v3636: f64;
                let v3834: f64;
                let v3943: f64;
                let v5095: f64;
                let v9681: Lanes<5>;
                let v9682: Lanes<5>;
                let v9683: Lanes<5>;
                if v3481 != 0.0 {
                    let v3483 = v78 * v665;
                    let v3485 = (-v367) / v1206;
                    let v3486 = v3485.ln();
                    let v3487 = v3483 * v3486;
                    let v10953 = (v10415 * v78) * v3486;
                    let v10956 = (Lanes([0.0, 0.0, v10953[0], 0.0, 0.0])) + (((((v10834 * v3485) * v10390) / v1206) * (v9368 / v3485)) * v3483);
                    let v3488 = v1200 - v835;
                    let v10958 = v10410 * v3488;
                    let v3490 = v663 * v750;
                    let v3491 = v1 / v3490;
                    let v3492 = v3491 * v1128;
                    let v10968 = (((((v10410 * v750) + (v10485 * v663)) * v3491) * v10390) / v3490) * v1128;
                    let v10969 = v9421 * v3491;
                    let v10972 = (Lanes([0.0, 0.0, v10968[0], 0.0, 0.0])) + (Lanes([v10969[0], v10969[1], 0.0, v10969[2], v10969[3]]));
                    let v10973 = v10972 * v3493;
                    let v3495 = v78 + (v3493 * v3492);
                    let v3496 = v91 * v3495;
                    let v3497 = v3496 * v3495;
                    let v3498 = v3497 * v3495;
                    let v10980 = ((((v10973 * v91) * v3495) + (v10973 * v3496)) * v3495) + (v10973 * v3497);
                    let v3499 = (v663 * v3488) - v78;
                    let v3501 = v3500 * v3492;
                    let v3502 = v3501 * v3499;
                    let v10984 = ((v10972 * v3500) * v3499) + (((Lanes([0.0, 0.0, v10958[0], 0.0, 0.0])) + ((v10828 - v10837) * v663)) * v3501);
                    let v3504 = v3503 - v3502;
                    let v10985 = v10984 * v10390;
                    let v3505 = v3504 * v3504;
                    let v10986 = v10985 * v3504;
                    let v10987 = v10986 + v10986;
                    let v3508 = if v3498 < (v3505 * v3506) { 1.0 } else { 0.0 };
                    let v3520: f64;
                    let v9684: Lanes<5>;
                    if v3508 != 0.0 {
                        let v3512 = (v13 * v3498) / v3504;
                        let v3514 = ((v3509 + v3504) + v3512) + v3502;
                        let v10998 = (v10985 + (((v10980 * v13) - (v10985 * v3512)) / v3504)) + v10984;
                        v3520 = v3514;
                        v9684 = v10998;
                    } else {
                        let v3516 = (v3498 + v3505).sqrt();
                        let v3519 = (v3517 + v3516) + v3502;
                        let v10992 = ((v10980 + v10987) * (v9368 / (v10435 * v3516))) + v10984;
                        v3520 = v3519;
                        v9684 = v10992;
                    }
                    let v3521 = v3520.powf(v1562);
                    let v11002 = v9684 * (v1562 * (v3520.powf(v10999)));
                    let v3528 = v748 * v3521;
                    let v3530 = ((v3522 - (v3523 * v3492)) + (v78 * v3521)) + (v3528 * v3521);
                    let v3531 = v1 / v3521;
                    let v3532 = v3530 * v3531;
                    let v11019 = v10415 * v3532;
                    let v3535 = ((v3532 * v665) + v835) - v835;
                    let v11023 = (((((((((v10972 * v3523) * v10390) + (v11002 * v78)) + (((v11002 * v748) * v3521) + (v11002 * v3528))) * v3531) + ((((v11002 * v3531) * v10390) / v3521) * v3530)) * v665) + (Lanes([0.0, 0.0, v11019[0], 0.0, 0.0]))) + v10837) - v10837;
                    let v3536 = v3535 / v3487;
                    let v11027 = ((v11023 - (v10956 * v3536)) / v3487) * v3536;
                    let v3539 = (v1 + (v3536 * v3536)).sqrt();
                    let v3540 = v3535 / v3539;
                    let v3541 = v3540 + v835;
                    let v11035 = ((v11023 - (((v11027 + v11027) * (v9368 / (v10435 * v3539))) * v3540)) / v3539) + v10837;
                    v3636 = v3541;
                    v3834 = v3482;
                    v3943 = v0;
                    v5095 = v0;
                    v9681 = v11035;
                    v9682 = v10579;
                    v9683 = v10579;
                } else {
                    let v3623: f64;
                    let v3625: f64;
                    let v9685: Lanes<5>;
                    let v9686: Lanes<5>;
                    if v3542 != 0.0 {
                        v3623 = v0;
                        v3625 = v0;
                        v9685 = v10579;
                        v9686 = v10579;
                    } else {
                        let v3543 = v1200 - v835;
                        let v3544 = v663 * v3543;
                        let v10839 = v10410 * v3543;
                        let v10842 = (Lanes([0.0, 0.0, v10839[0], 0.0, 0.0])) + ((v10828 - v10837) * v663);
                        let v3547 = v1207 * v664;
                        let v10845 = v10412 * v1207;
                        let v3548 = (v90 * (v3544 - v1)) / v3547;
                        let v10850 = ((v10842 * v90) - (((v10836 * v664) + (Lanes([0.0, 0.0, v10845[0], 0.0, 0.0]))) * v3548)) / v3547;
                        let v3549 = v1 + v3548;
                        let v3551 = if v3549 >= v3550 { 1.0 } else { 0.0 };
                        let v3553: f64;
                        let v9687: Lanes<5>;
                        if v3551 != 0.0 {
                            v3553 = v3549;
                            v9687 = v10850;
                        } else {
                            v3553 = v3552;
                            v9687 = v10579;
                        }
                        let v10852 = v10410 * v1207;
                        let v3555 = (v1207 * v663) * v13;
                        let v3556 = v3553.sqrt();
                        let v3557 = v1 - v3556;
                        let v3559 = v1200 + (v3555 * v3557);
                        let v10863 = v10828 + (((((v10836 * v663) + (Lanes([0.0, 0.0, v10852[0], 0.0, 0.0]))) * v13) * v3557) + (((v9687 * (v9368 / (v10435 * v3556))) * v10390) * v3555));
                        let v3562 = if (v663 * (v3559 - v835)) < v96 { 1.0 } else { 0.0 };
                        let v3620: f64;
                        let v3626: f64;
                        let v9688: Lanes<5>;
                        let v9689: Lanes<5>;
                        if v3562 != 0.0 {
                            let v3564 = v3563 * v663;
                            let v3565 = v3564 * v1206;
                            let v10901 = (v10410 * v3563) * v1206;
                            let v3566 = v1 / v3565;
                            let v10907 = ((((Lanes([0.0, 0.0, v10901[0], 0.0, 0.0])) + (v10834 * v3564)) * v3566) * v10390) / v3565;
                            let v10908 = v10907 * v96;
                            let v3568 = v1540 + (v96 * v3566);
                            let v3572 = v1153 * v3566;
                            let v3573 = v3572 * v3544;
                            let v10915 = ((v10907 * v1540) * v10390) + (((v10907 * v1153) * v3544) + (v10842 * v3572));
                            let v3578 = (v1549 - (v1540 * (v1550 + v3566))) + v3573;
                            let v10916 = v10915 * v3578;
                            let v3580 = v90 * v3568;
                            let v3581 = v3580 * v3568;
                            let v3584 = ((v3581 * v3568) + (v3578 * v3578)).sqrt();
                            let v3585 = ((v3569 - (v1540 * v3566)) + v3573) + v3584;
                            let v3586 = v3585.powf(v1562);
                            let v10933 = (v10915 + (((((((v10908 * v90) * v3568) + (v10908 * v3580)) * v3568) + (v10908 * v3581)) + (v10916 + v10916)) * (v9368 / (v10435 * v3584)))) * (v1562 * (v3585.powf(v10930)));
                            let v3588 = v96 * v3586;
                            let v3589 = (v1564 * v3568) / v3588;
                            let v3593 = (v96 - v3589) + (v3591 * v3586);
                            let v10943 = v10415 * v3593;
                            let v3595 = (v3593 * v665) + v835;
                            let v10946 = (((((((v10908 * v1564) - ((v10933 * v96) * v3589)) / v3588) * v10390) + (v10933 * v3591)) * v665) + (Lanes([0.0, 0.0, v10943[0], 0.0, 0.0]))) + v10837;
                            v3620 = v3595;
                            v3626 = v3595;
                            v9688 = v10946;
                            v9689 = v10946;
                        } else {
                            let v3596 = if v830 <= v1143 { 1.0 } else { 0.0 };
                            let v3621: f64;
                            let v9690: Lanes<5>;
                            if v3596 != 0.0 {
                                v3621 = v3559;
                                v9690 = v10863;
                            } else {
                                let v3597 = v1 / v759;
                                let v10866 = ((v10496 * v3597) * v10390) / v759;
                                let v3598 = v3597 / v1211;
                                let v3599 = v3598 * v1200;
                                let v3600 = v3599 * v1200;
                                let v3601 = v78 / v1200;
                                let v3602 = v663 + v3601;
                                let v3604 = (v3600.ln()) / v3602;
                                let v10886 = (((((((((Lanes([0.0, 0.0, v10866[0], 0.0, 0.0])) - (v9422 * v3598)) / v1211) * v1200) + (v10828 * v3598)) * v1200) + (v10828 * v3599)) * (v9368 / v3600)) - (((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + (((v10828 * v3601) * v10390) / v1200)) * v3604)) / v3602;
                                let v10887 = v10886 - v10863;
                                let v3606 = (v3604 - v3559) - v1270;
                                let v3608 = (v90 * v3604) * v1270;
                                let v10889 = (v10886 * v90) * v1270;
                                let v3609 = if v3608 > v0 { 1.0 } else { 0.0 };
                                let v3611: f64;
                                let v9691: Lanes<5>;
                                if v3609 != 0.0 {
                                    v3611 = v3608;
                                    v9691 = v10889;
                                } else {
                                    let v3610 = -v3608;
                                    let v10890 = v10889 * v10390;
                                    v3611 = v3610;
                                    v9691 = v10890;
                                }
                                let v10891 = v10887 * v3606;
                                let v3614 = ((v3606 * v3606) + v3611).sqrt();
                                let v3617 = v3604 - (v13 * (v3606 + v3614));
                                let v10899 = v10886 - ((v10887 + (((v10891 + v10891) + v9691) * (v9368 / (v10435 * v3614)))) * v13);
                                v3621 = v3617;
                                v9690 = v10899;
                            }
                            v3620 = v3621;
                            v3626 = v3559;
                            v9688 = v9690;
                            v9689 = v10863;
                        }
                        let v3619 = v835 + v3618;
                        let v3622 = if v3620 < v3619 { 1.0 } else { 0.0 };
                        let v3624: f64;
                        let v9692: Lanes<5>;
                        if v3622 != 0.0 {
                            v3624 = v3619;
                            v9692 = v10837;
                        } else {
                            v3624 = v3620;
                            v9692 = v9688;
                        }
                        v3623 = v3624;
                        v3625 = v3626;
                        v9685 = v9692;
                        v9686 = v9689;
                    }
                    v3636 = v3623;
                    v3834 = v0;
                    v3943 = v3625;
                    v5095 = v3623;
                    v9681 = v9685;
                    v9682 = v9686;
                    v9683 = v9685;
                }
                let v3629 = if (if v1886 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v2204 == v78 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v3632: f64;
                let v9693: Lanes<1>;
                if v3629 != 0.0 {
                    let v3631 = v3630 * v2254;
                    let v11037 = v9380 * v3630;
                    v3632 = v3631;
                    v9693 = v11037;
                } else {
                    v3632 = v0;
                    v9693 = v11036;
                }
                let v11038 = v10410 * v835;
                let v11039 = v9413 * v663;
                let v3634 = (v663 * v835).exp();
                let v11043 = ((Lanes([0.0, 0.0, v11038[0], 0.0])) + (Lanes([v11039[0], v11039[1], 0.0, v11039[2]]))) * v3634;
                let v3635 = v759 * v3634;
                let v11044 = v10496 * v3634;
                let v11047 = (Lanes([0.0, 0.0, v11044[0], 0.0])) + (v11043 * v759);
                let v3640 = (((v490 * v12) * v12) / v78) / v123;
                let v3643 = ((v78 * v663) * v3640).sqrt();
                let v11052 = ((v10410 * v78) * v3640) * (v9368 / (v10435 * v3643));
                let v3644 = v3643.exp();
                let v3646 = (-v3643).exp();
                let v3648 = (v3644 + v3646) / v78;
                let v3650 = (v3648.ln()) / v3640;
                let v11060 = ((((v11052 * v3644) + ((v11052 * v10390) * v3646)) / v78) * (v9368 / v3648)) / v3640;
                let v11061 = Lanes([v9681[0], v9681[1], v9681[2], v9681[3], v9681[4], 0.0]);
                let mut v3651: f64 = 0.0;
                let mut v3654: f64 = 0.0;
                let mut v3744: f64 = 0.0;
                let mut v3750: f64 = 0.0;
                let mut v3835: f64 = 0.0;
                let mut v3842: f64 = 0.0;
                let mut v3845: f64 = 0.0;
                let mut v4188: f64 = 0.0;
                let mut v9694: Lanes<6> = Lanes([0.0; 6]);
                let mut v9695: Lanes<6> = Lanes([0.0; 6]);
                let mut v9696: Lanes<6> = Lanes([0.0; 6]);
                let mut v9697: Lanes<6> = Lanes([0.0; 6]);
                v3651 = v1;
                v3654 = v3636;
                v3744 = v0;
                v3750 = v3834;
                v3835 = v0;
                v3842 = v0;
                v3845 = v0;
                v4188 = v4189;
                v9694 = v11061;
                v9695 = v11062;
                v9696 = v11062;
                v9697 = v11062;
                loop {
                    let v3653 = if v3651 <= v3652 { 1.0 } else { 0.0 };
                    if v3653 == 0.0 {
                        break;
                    }
                    let v3655 = v3654 - v835;
                    let v11688 = v9694 - (Lanes([v9413[0], v9413[1], 0.0, 0.0, v9413[2], 0.0]));
                    let v3656 = v663 * v3655;
                    let v11689 = v10410 * v3655;
                    let v11692 = (Lanes([0.0, 0.0, v11689[0], 0.0, 0.0, 0.0])) + (v11688 * v663);
                    let v3657 = v3655 - v3640;
                    let v3658 = v3650 * v3657;
                    let v11693 = v11060 * v3657;
                    let v11696 = (Lanes([0.0, 0.0, v11693[0], 0.0, 0.0, 0.0])) + (v11688 * v3650);
                    let v3659 = if v3658 < v2535 { 1.0 } else { 0.0 };
                    let v3669: f64;
                    let v3674: f64;
                    let v9698: Lanes<6>;
                    let v9699: Lanes<6>;
                    if v3659 != 0.0 {
                        let v3660 = v3658.exp();
                        let v11697 = v11696 * v3660;
                        let v3663 = ((-v3650) * v3640).exp();
                        let v11700 = ((v11060 * v10390) * v3640) * v3663;
                        let v11702 = v11697 - (Lanes([0.0, 0.0, v11700[0], 0.0, 0.0, 0.0]));
                        let v3665 = v1 + (v3660 - v3663);
                        let v3667 = (v3665.ln()) / v3650;
                        let v11705 = v11060 * v3667;
                        let v11708 = ((v11702 * (v9368 / v3665)) - (Lanes([0.0, 0.0, v11705[0], 0.0, 0.0, 0.0]))) / v3650;
                        let v3668 = v3660 / v3665;
                        let v11711 = (v11697 - (v11702 * v3668)) / v3665;
                        v3669 = v3667;
                        v3674 = v3668;
                        v9698 = v11708;
                        v9699 = v11711;
                    } else {
                        v3669 = v3657;
                        v3674 = v1;
                        v9698 = v11688;
                        v9699 = v11062;
                    }
                    let v3670 = v663 * v3669;
                    let v11712 = v10410 * v3669;
                    let v11715 = (Lanes([0.0, 0.0, v11712[0], 0.0, 0.0, 0.0])) + (v9698 * v663);
                    let v3671 = v3656.abs();
                    let v3673 = if v3671 < v3672 { 1.0 } else { 0.0 };
                    let v3753: f64;
                    let v3763: f64;
                    let v9700: Lanes<6>;
                    let v9701: Lanes<6>;
                    if v3673 != 0.0 {
                        let v11818 = v9699 * v3674;
                        let v3678 = ((v1 - (v3674 * v3674)) / v78).sqrt();
                        let v11824 = (((v11818 + v11818) * v10390) / v78) * (v9368 / (v10435 * v3678));
                        let v3679 = v3656 * v3678;
                        let v11827 = (v11692 * v3678) + (v11824 * v3656);
                        let v3680 = v663 * v3678;
                        let v11828 = v10410 * v3678;
                        let v11831 = (Lanes([0.0, 0.0, v11828[0], 0.0, 0.0, 0.0])) + (v11824 * v663);
                        let v3681 = if v3656 < v0 { 1.0 } else { 0.0 };
                        let v3754: f64;
                        let v3764: f64;
                        let v9702: Lanes<6>;
                        let v9703: Lanes<6>;
                        if v3681 != 0.0 {
                            let v3682 = -v3679;
                            let v11832 = v11827 * v10390;
                            let v3683 = -v3680;
                            let v11833 = v11831 * v10390;
                            v3754 = v3682;
                            v3764 = v3683;
                            v9702 = v11832;
                            v9703 = v11833;
                        } else {
                            v3754 = v3679;
                            v3764 = v3680;
                            v9702 = v11827;
                            v9703 = v11831;
                        }
                        v3753 = v3754;
                        v3763 = v3764;
                        v9700 = v9702;
                        v9701 = v9703;
                    } else {
                        let v3685 = if v3671 < v3684 { 1.0 } else { 0.0 };
                        let v3755: f64;
                        let v3765: f64;
                        let v9704: Lanes<6>;
                        let v9705: Lanes<6>;
                        if v3685 != 0.0 {
                            let v11740 = v11692 * v3656;
                            let v3687 = (v3656 * v3656) / v78;
                            let v3688 = v3656 / v96;
                            let v11743 = v11692 / v96;
                            let v3689 = v3656 / v90;
                            let v11744 = v11692 / v90;
                            let v3691 = v1 - (v3656 / v644);
                            let v3693 = v1 - (v3689 * v3691);
                            let v3695 = v1 - (v3688 * v3693);
                            let v3697 = v3656 / v78;
                            let v3698 = v1 - v3689;
                            let v3700 = v1 - (v3688 * v3698);
                            let v3702 = v1 - (v3697 * v3700);
                            let v11771 = v11715 * v3670;
                            let v3705 = (v3670 * v3670) / v78;
                            let v3706 = v3670 / v96;
                            let v11774 = v11715 / v96;
                            let v3707 = v3670 / v90;
                            let v11775 = v11715 / v90;
                            let v3709 = v1 - (v3670 / v644);
                            let v3711 = v1 - (v3707 * v3709);
                            let v3713 = v1 - (v3706 * v3711);
                            let v3715 = v3670 / v78;
                            let v3716 = v1 - v3707;
                            let v3718 = v1 - (v3706 * v3716);
                            let v3720 = v1 - (v3715 * v3718);
                            let v3721 = v3670 * v3720;
                            let v3723 = ((v3687 * v3695) - (v3705 * v3713)).sqrt();
                            let v11805 = (((((v11740 + v11740) / v78) * v3695) + ((((v11743 * v3693) + ((((v11744 * v3691) + (((v11692 / v644) * v10390) * v3689)) * v10390) * v3688)) * v10390) * v3687)) - ((((v11771 + v11771) / v78) * v3713) + ((((v11774 * v3711) + ((((v11775 * v3709) + (((v11715 / v644) * v10390) * v3707)) * v10390) * v3706)) * v10390) * v3705))) * (v9368 / (v10435 * v3723));
                            let v3724 = v663 * v13;
                            let v3726 = (v3656 * v3702) - (v3674 * v3721);
                            let v11811 = (v10410 * v13) * v3726;
                            let v3728 = (v3724 * v3726) / v3723;
                            let v11817 = (((Lanes([0.0, 0.0, v11811[0], 0.0, 0.0, 0.0])) + ((((v11692 * v3702) + (((((v11692 / v78) * v3700) + ((((v11743 * v3698) + ((v11744 * v10390) * v3688)) * v10390) * v3697)) * v10390) * v3656)) - ((v9699 * v3721) + (((v11715 * v3720) + (((((v11715 / v78) * v3718) + ((((v11774 * v3716) + ((v11775 * v10390) * v3706)) * v10390) * v3715)) * v10390) * v3670)) * v3674))) * v3724)) - (v11805 * v3728)) / v3723;
                            v3755 = v3723;
                            v3765 = v3728;
                            v9704 = v11805;
                            v9705 = v11817;
                        } else {
                            let v3730 = (-v3656).exp();
                            let v11717 = (v11692 * v10390) * v3730;
                            let v3732 = (-v3670).exp();
                            let v11719 = (v11715 * v10390) * v3732;
                            let v3736 = ((v3656 - v3670) + (v3730 - v3732)).sqrt();
                            let v11725 = ((v11692 - v11715) + (v11717 - v11719)) * (v9368 / (v10435 * v3736));
                            let v3737 = v663 * v13;
                            let v3739 = v1 - v3732;
                            let v3741 = (v1 - v3730) - (v3674 * v3739);
                            let v11733 = (v10410 * v13) * v3741;
                            let v3743 = (v3737 * v3741) / v3736;
                            let v11739 = (((Lanes([0.0, 0.0, v11733[0], 0.0, 0.0, 0.0])) + (((v11717 * v10390) - ((v9699 * v3739) + ((v11719 * v10390) * v3674))) * v3737)) - (v11725 * v3743)) / v3736;
                            v3755 = v3736;
                            v3765 = v3743;
                            v9704 = v11725;
                            v9705 = v11739;
                        }
                        v3753 = v3755;
                        v3763 = v3765;
                        v9700 = v9704;
                        v9701 = v9705;
                    }
                    let v3745 = if v3744 == v1 { 1.0 } else { 0.0 };
                    let v3746 = if v3656 < v0 { 1.0 } else { 0.0 };
                    let v3747 = if v3745 != 0.0 && v3746 != 0.0 { 1.0 } else { 0.0 };
                    let v3749: f64;
                    if v3747 != 0.0 {
                        v3749 = v3748;
                    } else {
                        v3749 = v3750;
                    }
                    let v3752 = if v3749 == v3751 { 1.0 } else { 0.0 };
                    let v3757: f64;
                    let v9706: Lanes<6>;
                    if v3752 != 0.0 {
                        v3757 = v0;
                        v9706 = v11062;
                    } else {
                        let v3756 = v762 * v3753;
                        let v11834 = v10503 * v3753;
                        let v11837 = (Lanes([0.0, 0.0, v11834[0], 0.0, 0.0, 0.0])) + (v9700 * v762);
                        v3757 = v3756;
                        v9706 = v11837;
                    }
                    let v3760 = if v3757 < (v12 * v3758) { 1.0 } else { 0.0 };
                    let v4190: f64;
                    if v3760 != 0.0 {
                        v4190 = v1;
                    } else {
                        v4190 = v78;
                    }
                    let v3761 = v490 * v3757;
                    let v11838 = v9706 * v490;
                    let v3797: f64;
                    let v3803: f64;
                    let v3846: f64;
                    let v9707: Lanes<6>;
                    let v9708: Lanes<6>;
                    let v9709: Lanes<6>;
                    if v3746 != 0.0 {
                        let v3762 = -v3753;
                        let v11891 = v9700 * v10390;
                        let v3766 = -v3763;
                        let v11892 = v9701 * v10390;
                        v3797 = v3762;
                        v3803 = v3766;
                        v3846 = v3845;
                        v9707 = v11891;
                        v9708 = v11892;
                        v9709 = v9697;
                    } else {
                        let v3767 = if v3656 < v117 { 1.0 } else { 0.0 };
                        let v3798: f64;
                        let v3804: f64;
                        let v3847: f64;
                        let v9710: Lanes<6>;
                        let v9711: Lanes<6>;
                        let v9712: Lanes<6>;
                        if v3767 != 0.0 {
                            v3798 = v3753;
                            v3804 = v3763;
                            v3847 = v3845;
                            v9710 = v9700;
                            v9711 = v9701;
                            v9712 = v9697;
                        } else {
                            let v3768 = if v3656 < v2535 { 1.0 } else { 0.0 };
                            let v3786: f64;
                            let v3791: f64;
                            let v9713: Lanes<6>;
                            let v9714: Lanes<6>;
                            if v3768 != 0.0 {
                                let v3769 = v3656.exp();
                                let v11862 = v11692 * v3769;
                                let v3771 = v3769 - (v3656 + v1);
                                let v3772 = v3635 * v3771;
                                let v11864 = v11047 * v3771;
                                let v11867 = (Lanes([v11864[0], v11864[1], v11864[2], 0.0, v11864[3], 0.0])) + ((v11862 - v11692) * v3635);
                                let v3773 = v3635 * v663;
                                let v11869 = v10410 * v3635;
                                let v3774 = v3769 - v1;
                                let v3775 = v3773 * v3774;
                                let v11872 = ((v11047 * v663) + (Lanes([0.0, 0.0, v11869[0], 0.0]))) * v3774;
                                let v11875 = (Lanes([v11872[0], v11872[1], v11872[2], 0.0, v11872[3], 0.0])) + (v11862 * v3773);
                                v3786 = v3772;
                                v3791 = v3775;
                                v9713 = v11867;
                                v9714 = v11875;
                            } else {
                                let v11839 = v10410 * v3654;
                                let v3777 = (v663 * v3654).exp();
                                let v11843 = ((Lanes([0.0, 0.0, v11839[0], 0.0, 0.0, 0.0])) + (v9694 * v663)) * v3777;
                                let v3778 = v3656 + v1;
                                let v11844 = v11043 * v3778;
                                let v3780 = v3777 - (v3634 * v3778);
                                let v3781 = v759 * v3780;
                                let v11849 = v10496 * v3780;
                                let v11852 = (Lanes([0.0, 0.0, v11849[0], 0.0, 0.0, 0.0])) + ((v11843 - ((Lanes([v11844[0], v11844[1], v11844[2], 0.0, v11844[3], 0.0])) + (v11692 * v3634))) * v759);
                                let v3782 = v759 * v663;
                                let v3783 = v3777 - v3634;
                                let v3784 = v3782 * v3783;
                                let v11858 = ((v10496 * v663) + (v10410 * v759)) * v3783;
                                let v11861 = (Lanes([0.0, 0.0, v11858[0], 0.0, 0.0, 0.0])) + ((v11843 - (Lanes([v11043[0], v11043[1], v11043[2], 0.0, v11043[3], 0.0]))) * v3782);
                                v3786 = v3781;
                                v3791 = v3784;
                                v9713 = v11852;
                                v9714 = v11861;
                            }
                            let v11876 = v9700 * v3753;
                            let v3788 = ((v3753 * v3753) + v3786).sqrt();
                            let v11881 = ((v11876 + v11876) + v9713) * (v9368 / (v10435 * v3788));
                            let v3789 = v78 * v3763;
                            let v3794 = (v13 * ((v3789 * v3753) + v3791)) / v3788;
                            let v11890 = ((((((v9701 * v78) * v3753) + (v9700 * v3789)) + v9714) * v13) - (v11881 * v3794)) / v3788;
                            v3798 = v3788;
                            v3804 = v3794;
                            v3847 = v3786;
                            v9710 = v11881;
                            v9711 = v11890;
                            v9712 = v9713;
                        }
                        v3797 = v3798;
                        v3803 = v3804;
                        v3846 = v3847;
                        v9707 = v9710;
                        v9708 = v9711;
                        v9709 = v9712;
                    }
                    let v11893 = v10828 * v10390;
                    let v11896 = v10834 * v3797;
                    let v11901 = v9420 * v3632;
                    let v11902 = v9693 * v1048;
                    let v11905 = (Lanes([v11901[0], v11901[1], v11901[2], v11901[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v11902[0]]));
                    let v3802 = (((-v1200) + v3654) + (v1206 * v3797)) - (v1048 * v3632);
                    let v11907 = (((Lanes([v11893[0], v11893[1], v11893[2], v11893[3], v11893[4], 0.0])) + v9694) + ((Lanes([v11896[0], v11896[1], v11896[2], v11896[3], v11896[4], 0.0])) + (v9707 * v1206))) - (Lanes([v11905[0], v11905[1], 0.0, v11905[2], v11905[3], v11905[4]]));
                    let v11908 = v10834 * v3803;
                    let v11911 = (Lanes([v11908[0], v11908[1], v11908[2], v11908[3], v11908[4], 0.0])) + (v9708 * v1206);
                    let v3806 = v1 + (v1206 * v3803);
                    let v3829: f64;
                    let v3831: f64;
                    let v3832: f64;
                    let v9715: Lanes<6>;
                    if v3745 != 0.0 {
                        v3829 = v3807;
                        v3831 = v3654;
                        v3832 = v3744;
                        v9715 = v9694;
                    } else {
                        let v3809 = (-v3802) / v3806;
                        let v11915 = ((v11907 * v10390) - (v11911 * v3809)) / v3806;
                        let v3811 = v3654.abs();
                        let v11919 = v9694 * ((v10435 * (if v3654 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                        let v3812 = if v1 >= v3811 { 1.0 } else { 0.0 };
                        let v3813: f64;
                        let v9716: Lanes<6>;
                        if v3812 != 0.0 {
                            v3813 = v1;
                            v9716 = v11062;
                        } else {
                            v3813 = v3811;
                            v9716 = v11919;
                        }
                        let v3815 = v3810 * (v1 + v3813);
                        let v11920 = v9716 * v3810;
                        let v3817 = if (v3809.abs()) > v3815 { 1.0 } else { 0.0 };
                        let v3822: f64;
                        let v9717: Lanes<6>;
                        if v3817 != 0.0 {
                            let v3818 = if v3809 >= v0 { 1.0 } else { 0.0 };
                            let v3820: f64;
                            if v3818 != 0.0 {
                                v3820 = v1;
                            } else {
                                v3820 = v3819;
                            }
                            let v3821 = v3815 * v3820;
                            let v11921 = v11920 * v3820;
                            v3822 = v3821;
                            v9717 = v11921;
                        } else {
                            v3822 = v3809;
                            v9717 = v11915;
                        }
                        let v3823 = v3654 + v3822;
                        let v11922 = v9694 + v9717;
                        let v3828 = if (if (v3822.abs()) <= v861 { 1.0 } else { 0.0 }) != 0.0 && (if (v3802.abs()) <= v3506 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3833: f64;
                        if v3828 != 0.0 {
                            v3833 = v1;
                        } else {
                            v3833 = v3744;
                        }
                        v3829 = v3651;
                        v3831 = v3823;
                        v3832 = v3833;
                        v9715 = v11922;
                    }
                    let v3830 = v3829 + v1;
                    v3651 = v3830;
                    v3654 = v3831;
                    v3744 = v3832;
                    v3750 = v3749;
                    v3835 = v3761;
                    v3842 = v3797;
                    v3845 = v3846;
                    v4188 = v4190;
                    v9694 = v9715;
                    v9695 = v11838;
                    v9696 = v9707;
                    v9697 = v9709;
                }
                let v3836 = v3835 / v750;
                let v11063 = v10485 * v3836;
                let v11066 = (v9695 - (Lanes([0.0, 0.0, v11063[0], 0.0, 0.0, 0.0]))) / v750;
                let v11067 = v11066 * v3836;
                let v11068 = v11067 + v11067;
                let v3839 = (v3836 * v3836) + v3838;
                let v3841 = v3836 + v3840;
                let v3843 = v3842 + v3841;
                let v3844 = v1 / v3843;
                let v3848 = v750 * v3845;
                let v11073 = v10485 * v3845;
                let v3849 = v3848 * v3844;
                let v11079 = (((Lanes([0.0, 0.0, v11073[0], 0.0, 0.0, 0.0])) + (v9697 * v750)) * v3844) + (((((v9696 + v11066) * v3844) * v10390) / v3843) * v3848);
                let v3850 = -v3849;
                let v11080 = v11079 * v10390;
                let v3851 = v3849 * v1048;
                let v11082 = v9420 * v3849;
                let v11084 = (v11079 * v1048) + (Lanes([v11082[0], v11082[1], 0.0, v11082[2], v11082[3], 0.0]));
                let v3855 = if (if v3750 == v3852 { 1.0 } else { 0.0 }) != 0.0 || (if v3851 <= v11 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v3868: f64;
                let v4141: f64;
                let v4236: f64;
                let v4328: f64;
                let v4339: f64;
                let v4426: f64;
                let v8306: f64;
                let v8483: f64;
                let v8565: f64;
                let v8577: f64;
                let v9718: Lanes<6>;
                let v9719: Lanes<6>;
                let v9720: Lanes<6>;
                let v9721: Lanes<6>;
                let v9722: Lanes<6>;
                let v9723: Lanes<6>;
                let v9724: Lanes<6>;
                if v3855 != 0.0 {
                    let v3856 = v1200 - v3654;
                    let v3857 = v1128 * v3856;
                    let v11087 = v9421 * v3856;
                    let v11090 = (Lanes([v11087[0], v11087[1], 0.0, v11087[2], v11087[3], 0.0])) + (((Lanes([v10828[0], v10828[1], v10828[2], v10828[3], v10828[4], 0.0])) - v9694) * v1128);
                    let v3859 = (-v168) * v139;
                    let v3860 = v3859 * v3857;
                    let v11091 = v11090 * v3859;
                    let v3864 = -v3861;
                    let v3865 = v3864 * v3857;
                    let v11092 = v11090 * v3864;
                    let v3866 = v3865 * v13;
                    let v11093 = v11092 * v13;
                    let v3867 = v3865 - v3866;
                    let v11094 = v11092 - v11093;
                    v3868 = v1;
                    v4141 = v90;
                    v4236 = v0;
                    v4328 = v1;
                    v4339 = v3654;
                    v4426 = v3857;
                    v8306 = v3654;
                    v8483 = v3860;
                    v8565 = v3867;
                    v8577 = v3866;
                    v9718 = v11062;
                    v9719 = v9694;
                    v9720 = v11090;
                    v9721 = v9694;
                    v9722 = v11091;
                    v9723 = v11094;
                    v9724 = v11093;
                } else {
                    v3868 = v0;
                    v4141 = v3750;
                    v4236 = v3851;
                    v4328 = v0;
                    v4339 = v0;
                    v4426 = v0;
                    v8306 = v0;
                    v8483 = v0;
                    v8565 = v0;
                    v8577 = v0;
                    v9718 = v11084;
                    v9719 = v11062;
                    v9720 = v11062;
                    v9721 = v11062;
                    v9722 = v11062;
                    v9723 = v11062;
                    v9724 = v11062;
                }
                let v3869 = if v3868 == v0 { 1.0 } else { 0.0 };
                let v4309: f64;
                let v4313: f64;
                let v4316: f64;
                let v4338: f64;
                let v4384: f64;
                let v4423: f64;
                let v4430: f64;
                let v4447: f64;
                let v9725: Lanes<6>;
                let v9726: Lanes<6>;
                let v9727: Lanes<6>;
                let v9728: Lanes<6>;
                let v9729: Lanes<6>;
                let v9730: Lanes<6>;
                let v9731: Lanes<6>;
                let v9732: Lanes<6>;
                if v3869 != 0.0 {
                    let v3870 = v1128 * v1128;
                    let v11095 = v9421 * v1128;
                    let v3871 = v491 / v3870;
                    let v11099 = (((v11095 + v11095) * v3871) * v10390) / v3870;
                    let v3872 = v78 / v3871;
                    let v11102 = ((v11099 * v3872) * v10390) / v3871;
                    let v3873 = v1200 - v362;
                    let v11103 = v11102 * v3873;
                    let v11106 = (Lanes([v11103[0], v11103[1], 0.0, v11103[2], v11103[3]])) + (v10828 * v3872);
                    let v3875 = v1 + (v3872 * v3873);
                    let v3876 = v1 + v3872;
                    let v3879 = if (if v3875 < v3876 { 1.0 } else { 0.0 }) != 0.0 && (if v3876 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v3911: f64;
                    let v9733: Lanes<5>;
                    if v3879 != 0.0 {
                        let v3880 = v3876 - v3875;
                        let v11107 = Lanes([v11102[0], v11102[1], 0.0, v11102[2], v11102[3]]);
                        let v11108 = v11107 - v11106;
                        let v3881 = v3880 * v3880;
                        let v11109 = v11108 * v3880;
                        let v11110 = v11109 + v11109;
                        let v3882 = v3876 * v3876;
                        let v11111 = v11102 * v3876;
                        let v11112 = v11111 + v11111;
                        let v3883 = v3881 * v3881;
                        let v11113 = v11110 * v3881;
                        let v3884 = v3882 * v3882;
                        let v11115 = v11112 * v3882;
                        let v3885 = v3883 * v3881;
                        let v3886 = v3884 * v3882;
                        let v11128 = ((((v11115 + v11115) * v3882) + (v11112 * v3884)) * v3882) + (v11112 * v3886);
                        let v3889 = (v3885 * v3881) + (v3886 * v3882);
                        let v11130 = (((((v11113 + v11113) * v3881) + (v11110 * v3883)) * v3881) + (v11110 * v3885)) + (Lanes([v11128[0], v11128[1], 0.0, v11128[2], v11128[3]]));
                        let v3906: f64;
                        let v9734: Lanes<5>;
                        if v3890 != 0.0 {
                            let v3900: f64;
                            if v3891 != 0.0 {
                                v3900 = v1;
                            } else {
                                let v3901: f64;
                                if v3892 != 0.0 {
                                    v3901 = v78;
                                } else {
                                    let v3902: f64;
                                    if v3893 != 0.0 {
                                        v3902 = v96;
                                    } else {
                                        let v3903: f64;
                                        if v3894 != 0.0 {
                                            v3903 = v90;
                                        } else {
                                            v3903 = v0;
                                        }
                                        v3902 = v3903;
                                    }
                                    v3901 = v3902;
                                }
                                v3900 = v3901;
                            }
                            let mut v3895: f64 = 0.0;
                            let mut v3897: f64 = 0.0;
                            let mut v9735: Lanes<5> = Lanes([0.0; 5]);
                            v3895 = v0;
                            v3897 = v3889;
                            v9735 = v11130;
                            loop {
                                let v3896 = if v3895 < v3900 { 1.0 } else { 0.0 };
                                if v3896 == 0.0 {
                                    break;
                                }
                                let v3898 = v3897.sqrt();
                                let v11686 = v9735 * (v9368 / (v10435 * v3898));
                                let v3899 = v3895 + v1;
                                v3895 = v3899;
                                v3897 = v3898;
                                v9735 = v11686;
                            }
                            v3906 = v3897;
                            v9734 = v9735;
                        } else {
                            let v3905 = v3889.powf(v3904);
                            let v11134 = v11130 * (v3904 * (v3889.powf(v11131)));
                            v3906 = v3905;
                            v9734 = v11134;
                        }
                        let v3907 = v1 / v3906;
                        let v3908 = v3880 * v3876;
                        let v11139 = v11102 * v3880;
                        let v3910 = v3876 - (v3908 * v3907);
                        let v11145 = v11107 - ((((v11108 * v3876) + (Lanes([v11139[0], v11139[1], 0.0, v11139[2], v11139[3]]))) * v3907) + ((((v9734 * v3907) * v10390) / v3906) * v3908));
                        v3911 = v3910;
                        v9733 = v11145;
                    } else {
                        v3911 = v3875;
                        v9733 = v11106;
                    }
                    let v3912 = v3911.sqrt();
                    let v3913 = v1 - v3912;
                    let v11150 = v11099 * v3913;
                    let v3915 = v1200 + (v3871 * v3913);
                    let v11154 = v10828 + ((Lanes([v11150[0], v11150[1], 0.0, v11150[2], v11150[3]])) + (((v9733 * (v9368 / (v10435 * v3912))) * v10390) * v3871));
                    let v11155 = v11154 * v3915;
                    let v3919 = ((v3915 * v3915) + v3917).sqrt();
                    let v11161 = (v11154 + ((v11155 + v11155) * (v9368 / (v10435 * v3919)))) * v13;
                    let v3923 = (v13 * (v3915 + v3919)) + v3922;
                    let v3924 = if v3923 < v0 { 1.0 } else { 0.0 };
                    let v3925: f64;
                    let v9736: Lanes<5>;
                    if v3924 != 0.0 {
                        v3925 = v0;
                        v9736 = v10579;
                    } else {
                        v3925 = v3923;
                        v9736 = v11161;
                    }
                    let v3926 = v823 / v3925;
                    let v11164 = (v10597 - (v9736 * v3926)) / v3925;
                    let v3927 = v2662 - v1;
                    let v3928 = v3926.powf(v3927);
                    let v11171 = ((v11164 * (v3927 * (v3926.powf((v3927 - v9368))))) * v3926) + (v11164 * v3928);
                    let v3930 = v1 + (v3928 * v3926);
                    let v3932 = (v1 / v2662) - v1;
                    let v3933 = v3930.powf(v3932);
                    let v3934 = v3933 * v3930;
                    let v3935 = v823 / v3934;
                    let v11181 = (v10597 - ((((v11171 * (v3932 * (v3930.powf((v3932 - v9368))))) * v3930) + (v11171 * v3933)) * v3935)) / v3934;
                    let v3936 = v835 - v3935;
                    let v11183 = v10410 * v3936;
                    let v3938 = (v663 * v3936).exp();
                    let v11187 = ((Lanes([0.0, 0.0, v11183[0], 0.0, 0.0])) + ((v10837 - v11181) * v663)) * v3938;
                    let v3939 = if v3935 <= v0 { 1.0 } else { 0.0 };
                    let v3975: f64;
                    let v9737: Lanes<6>;
                    if v3939 != 0.0 {
                        v3975 = v3654;
                        v9737 = v9694;
                    } else {
                        let v3969: f64;
                        let v9738: Lanes<6>;
                        if v3940 != 0.0 {
                            let v3941 = v0 - v3654;
                            let v11188 = v9694 * v10390;
                            v3969 = v3941;
                            v9738 = v11188;
                        } else {
                            v3969 = v0;
                            v9738 = v11062;
                        }
                        let v3968: f64;
                        let v9739: Lanes<6>;
                        if v3942 != 0.0 {
                            let v3944 = v3943 - v3654;
                            let v11190 = (Lanes([v9682[0], v9682[1], v9682[2], v9682[3], v9682[4], 0.0])) - v9694;
                            let v3945 = if v3944 >= v0 { 1.0 } else { 0.0 };
                            let v3946: f64;
                            let v9740: Lanes<6>;
                            if v3945 != 0.0 {
                                v3946 = v3944;
                                v9740 = v11190;
                            } else {
                                v3946 = v0;
                                v9740 = v11062;
                            }
                            let v11193 = (v9740 * v3947) - (Lanes([v11181[0], v11181[1], v11181[2], v11181[3], v11181[4], 0.0]));
                            let v3950 = ((v3947 * v3946) - v3935) - v1985;
                            let v3954 = (v90 * (v3951 * v3946)) * v1985;
                            let v11196 = ((v9740 * v3951) * v90) * v1985;
                            let v3955 = if v3954 > v0 { 1.0 } else { 0.0 };
                            let v3957: f64;
                            let v9741: Lanes<6>;
                            if v3955 != 0.0 {
                                v3957 = v3954;
                                v9741 = v11196;
                            } else {
                                let v3956 = -v3954;
                                let v11197 = v11196 * v10390;
                                v3957 = v3956;
                                v9741 = v11197;
                            }
                            let v11198 = v11193 * v3950;
                            let v3960 = ((v3950 * v3950) + v3957).sqrt();
                            let v3965 = (v3961 * v3946) - (v13 * (v3950 + v3960));
                            let v11207 = (v9740 * v3961) - ((v11193 + (((v11198 + v11198) + v9741) * (v9368 / (v10435 * v3960)))) * v13);
                            let v3966 = if v3965 <= v3946 { 1.0 } else { 0.0 };
                            let v3967: f64;
                            let v9742: Lanes<6>;
                            if v3966 != 0.0 {
                                v3967 = v3965;
                                v9742 = v11207;
                            } else {
                                v3967 = v3946;
                                v9742 = v9740;
                            }
                            v3968 = v3967;
                            v9739 = v9742;
                        } else {
                            v3968 = v3969;
                            v9739 = v9738;
                        }
                        let v3970 = if v3968 < v0 { 1.0 } else { 0.0 };
                        let v3972: f64;
                        let v9743: Lanes<6>;
                        if v3970 != 0.0 {
                            v3972 = v0;
                            v9743 = v11062;
                        } else {
                            let v3971 = if v3968 > v3935 { 1.0 } else { 0.0 };
                            let v3973: f64;
                            let v9744: Lanes<6>;
                            if v3971 != 0.0 {
                                let v11208 = Lanes([v11181[0], v11181[1], v11181[2], v11181[3], v11181[4], 0.0]);
                                v3973 = v3935;
                                v9744 = v11208;
                            } else {
                                v3973 = v3968;
                                v9744 = v9739;
                            }
                            v3972 = v3973;
                            v9743 = v9744;
                        }
                        let v3974 = v3654 + v3972;
                        let v11209 = v9694 + v9743;
                        v3975 = v3974;
                        v9737 = v11209;
                    }
                    let mut v3976: f64 = 0.0;
                    let mut v3979: f64 = 0.0;
                    let mut v4112: f64 = 0.0;
                    let mut v4144: f64 = 0.0;
                    let mut v4148: f64 = 0.0;
                    let mut v4151: f64 = 0.0;
                    let mut v9745: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9746: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9747: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9748: Lanes<6> = Lanes([0.0; 6]);
                    v3976 = v1;
                    v3979 = v3975;
                    v4112 = v0;
                    v4144 = v3835;
                    v4148 = v0;
                    v4151 = v0;
                    v9745 = v9737;
                    v9746 = v9695;
                    v9747 = v11062;
                    v9748 = v11062;
                    loop {
                        let v3978 = if v3976 <= v3977 { 1.0 } else { 0.0 };
                        if v3978 == 0.0 {
                            break;
                        }
                        let v3980 = v3979 - v835;
                        let v11461 = v9745 - (Lanes([v9413[0], v9413[1], 0.0, 0.0, v9413[2], 0.0]));
                        let v3981 = v663 * v3980;
                        let v11462 = v10410 * v3980;
                        let v11465 = (Lanes([0.0, 0.0, v11462[0], 0.0, 0.0, 0.0])) + (v11461 * v663);
                        let v3982 = v3980 - v3640;
                        let v3983 = v3650 * v3982;
                        let v11466 = v11060 * v3982;
                        let v11469 = (Lanes([0.0, 0.0, v11466[0], 0.0, 0.0, 0.0])) + (v11461 * v3650);
                        let v3984 = if v3983 < v2535 { 1.0 } else { 0.0 };
                        let v3994: f64;
                        let v3998: f64;
                        let v9749: Lanes<6>;
                        let v9750: Lanes<6>;
                        if v3984 != 0.0 {
                            let v3985 = v3983.exp();
                            let v11470 = v11469 * v3985;
                            let v3988 = ((-v3650) * v3640).exp();
                            let v11473 = ((v11060 * v10390) * v3640) * v3988;
                            let v11475 = v11470 - (Lanes([0.0, 0.0, v11473[0], 0.0, 0.0, 0.0]));
                            let v3990 = v1 + (v3985 - v3988);
                            let v3992 = (v3990.ln()) / v3650;
                            let v11478 = v11060 * v3992;
                            let v11481 = ((v11475 * (v9368 / v3990)) - (Lanes([0.0, 0.0, v11478[0], 0.0, 0.0, 0.0]))) / v3650;
                            let v3993 = v3985 / v3990;
                            let v11484 = (v11470 - (v11475 * v3993)) / v3990;
                            v3994 = v3992;
                            v3998 = v3993;
                            v9749 = v11481;
                            v9750 = v11484;
                        } else {
                            v3994 = v3982;
                            v3998 = v1;
                            v9749 = v11461;
                            v9750 = v11062;
                        }
                        let v3995 = v663 * v3994;
                        let v11485 = v10410 * v3994;
                        let v11488 = (Lanes([0.0, 0.0, v11485[0], 0.0, 0.0, 0.0])) + (v9749 * v663);
                        let v3996 = v3981.abs();
                        let v3997 = if v3996 < v3672 { 1.0 } else { 0.0 };
                        let v4069: f64;
                        let v4077: f64;
                        let v9751: Lanes<6>;
                        let v9752: Lanes<6>;
                        if v3997 != 0.0 {
                            let v11591 = v9750 * v3998;
                            let v4002 = ((v1 - (v3998 * v3998)) / v78).sqrt();
                            let v11597 = (((v11591 + v11591) * v10390) / v78) * (v9368 / (v10435 * v4002));
                            let v4003 = v3981 * v4002;
                            let v11600 = (v11465 * v4002) + (v11597 * v3981);
                            let v4004 = v663 * v4002;
                            let v11601 = v10410 * v4002;
                            let v11604 = (Lanes([0.0, 0.0, v11601[0], 0.0, 0.0, 0.0])) + (v11597 * v663);
                            let v4005 = if v3981 < v0 { 1.0 } else { 0.0 };
                            let v4070: f64;
                            let v4078: f64;
                            let v9753: Lanes<6>;
                            let v9754: Lanes<6>;
                            if v4005 != 0.0 {
                                let v4006 = -v4003;
                                let v11605 = v11600 * v10390;
                                let v4007 = -v4004;
                                let v11606 = v11604 * v10390;
                                v4070 = v4006;
                                v4078 = v4007;
                                v9753 = v11605;
                                v9754 = v11606;
                            } else {
                                v4070 = v4003;
                                v4078 = v4004;
                                v9753 = v11600;
                                v9754 = v11604;
                            }
                            v4069 = v4070;
                            v4077 = v4078;
                            v9751 = v9753;
                            v9752 = v9754;
                        } else {
                            let v4008 = if v3996 < v3684 { 1.0 } else { 0.0 };
                            let v4071: f64;
                            let v4079: f64;
                            let v9755: Lanes<6>;
                            let v9756: Lanes<6>;
                            if v4008 != 0.0 {
                                let v11513 = v11465 * v3981;
                                let v4010 = (v3981 * v3981) / v78;
                                let v4011 = v3981 / v96;
                                let v11516 = v11465 / v96;
                                let v4012 = v3981 / v90;
                                let v11517 = v11465 / v90;
                                let v4014 = v1 - (v3981 / v644);
                                let v4016 = v1 - (v4012 * v4014);
                                let v4018 = v1 - (v4011 * v4016);
                                let v4020 = v3981 / v78;
                                let v4021 = v1 - v4012;
                                let v4023 = v1 - (v4011 * v4021);
                                let v4025 = v1 - (v4020 * v4023);
                                let v11544 = v11488 * v3995;
                                let v4028 = (v3995 * v3995) / v78;
                                let v4029 = v3995 / v96;
                                let v11547 = v11488 / v96;
                                let v4030 = v3995 / v90;
                                let v11548 = v11488 / v90;
                                let v4032 = v1 - (v3995 / v644);
                                let v4034 = v1 - (v4030 * v4032);
                                let v4036 = v1 - (v4029 * v4034);
                                let v4038 = v3995 / v78;
                                let v4039 = v1 - v4030;
                                let v4041 = v1 - (v4029 * v4039);
                                let v4043 = v1 - (v4038 * v4041);
                                let v4044 = v3995 * v4043;
                                let v4046 = ((v4010 * v4018) - (v4028 * v4036)).sqrt();
                                let v11578 = (((((v11513 + v11513) / v78) * v4018) + ((((v11516 * v4016) + ((((v11517 * v4014) + (((v11465 / v644) * v10390) * v4012)) * v10390) * v4011)) * v10390) * v4010)) - ((((v11544 + v11544) / v78) * v4036) + ((((v11547 * v4034) + ((((v11548 * v4032) + (((v11488 / v644) * v10390) * v4030)) * v10390) * v4029)) * v10390) * v4028))) * (v9368 / (v10435 * v4046));
                                let v4047 = v663 * v13;
                                let v4049 = (v3981 * v4025) - (v3998 * v4044);
                                let v11584 = (v10410 * v13) * v4049;
                                let v4051 = (v4047 * v4049) / v4046;
                                let v11590 = (((Lanes([0.0, 0.0, v11584[0], 0.0, 0.0, 0.0])) + ((((v11465 * v4025) + (((((v11465 / v78) * v4023) + ((((v11516 * v4021) + ((v11517 * v10390) * v4011)) * v10390) * v4020)) * v10390) * v3981)) - ((v9750 * v4044) + (((v11488 * v4043) + (((((v11488 / v78) * v4041) + ((((v11547 * v4039) + ((v11548 * v10390) * v4029)) * v10390) * v4038)) * v10390) * v3995)) * v3998))) * v4047)) - (v11578 * v4051)) / v4046;
                                v4071 = v4046;
                                v4079 = v4051;
                                v9755 = v11578;
                                v9756 = v11590;
                            } else {
                                let v4053 = (-v3981).exp();
                                let v11490 = (v11465 * v10390) * v4053;
                                let v4055 = (-v3995).exp();
                                let v11492 = (v11488 * v10390) * v4055;
                                let v4059 = ((v3981 - v3995) + (v4053 - v4055)).sqrt();
                                let v11498 = ((v11465 - v11488) + (v11490 - v11492)) * (v9368 / (v10435 * v4059));
                                let v4060 = v663 * v13;
                                let v4062 = v1 - v4055;
                                let v4064 = (v1 - v4053) - (v3998 * v4062);
                                let v11506 = (v10410 * v13) * v4064;
                                let v4066 = (v4060 * v4064) / v4059;
                                let v11512 = (((Lanes([0.0, 0.0, v11506[0], 0.0, 0.0, 0.0])) + (((v11490 * v10390) - ((v9750 * v4062) + ((v11492 * v10390) * v3998))) * v4060)) - (v11498 * v4066)) / v4059;
                                v4071 = v4059;
                                v4079 = v4066;
                                v9755 = v11498;
                                v9756 = v11512;
                            }
                            v4069 = v4071;
                            v4077 = v4079;
                            v9751 = v9755;
                            v9752 = v9756;
                        }
                        let v4068 = if v4141 == v4067 { 1.0 } else { 0.0 };
                        let v4073: f64;
                        let v9757: Lanes<6>;
                        if v4068 != 0.0 {
                            v4073 = v0;
                            v9757 = v11062;
                        } else {
                            let v4072 = v762 * v4069;
                            let v11607 = v10503 * v4069;
                            let v11610 = (Lanes([0.0, 0.0, v11607[0], 0.0, 0.0, 0.0])) + (v9751 * v762);
                            v4073 = v4072;
                            v9757 = v11610;
                        }
                        let v4074 = v490 * v4073;
                        let v11611 = v9757 * v490;
                        let v4075 = if v3981 < v0 { 1.0 } else { 0.0 };
                        let v4102: f64;
                        let v4108: f64;
                        let v4152: f64;
                        let v9758: Lanes<6>;
                        let v9759: Lanes<6>;
                        let v9760: Lanes<6>;
                        if v4075 != 0.0 {
                            let v4076 = -v4069;
                            let v11652 = v9751 * v10390;
                            let v4080 = -v4077;
                            let v11653 = v9752 * v10390;
                            v4102 = v4076;
                            v4108 = v4080;
                            v4152 = v4151;
                            v9758 = v11652;
                            v9759 = v11653;
                            v9760 = v9748;
                        } else {
                            let v4081 = if v3981 < v117 { 1.0 } else { 0.0 };
                            let v4103: f64;
                            let v4109: f64;
                            let v4153: f64;
                            let v9761: Lanes<6>;
                            let v9762: Lanes<6>;
                            let v9763: Lanes<6>;
                            if v4081 != 0.0 {
                                v4103 = v4069;
                                v4109 = v4077;
                                v4153 = v4151;
                                v9761 = v9751;
                                v9762 = v9752;
                                v9763 = v9748;
                            } else {
                                let v4082 = v3979 - v3935;
                                let v11614 = v10410 * v4082;
                                let v4084 = (v663 * v4082).exp();
                                let v11618 = ((Lanes([0.0, 0.0, v11614[0], 0.0, 0.0, 0.0])) + ((v9745 - (Lanes([v11181[0], v11181[1], v11181[2], v11181[3], v11181[4], 0.0]))) * v663)) * v4084;
                                let v4085 = v3981 + v1;
                                let v11619 = v11187 * v4085;
                                let v4087 = v4084 - (v3938 * v4085);
                                let v4088 = v759 * v4087;
                                let v11624 = v10496 * v4087;
                                let v11627 = (Lanes([0.0, 0.0, v11624[0], 0.0, 0.0, 0.0])) + ((v11618 - ((Lanes([v11619[0], v11619[1], v11619[2], v11619[3], v11619[4], 0.0])) + (v11465 * v3938))) * v759);
                                let v4089 = v759 * v663;
                                let v4090 = v4084 - v3938;
                                let v11633 = ((v10496 * v663) + (v10410 * v759)) * v4090;
                                let v11637 = v9751 * v4069;
                                let v4094 = ((v4069 * v4069) + v4088).sqrt();
                                let v11642 = ((v11637 + v11637) + v11627) * (v9368 / (v10435 * v4094));
                                let v4095 = v78 * v4077;
                                let v4099 = (v13 * ((v4095 * v4069) + (v4089 * v4090))) / v4094;
                                let v11651 = ((((((v9752 * v78) * v4069) + (v9751 * v4095)) + ((Lanes([0.0, 0.0, v11633[0], 0.0, 0.0, 0.0])) + ((v11618 - (Lanes([v11187[0], v11187[1], v11187[2], v11187[3], v11187[4], 0.0]))) * v4089))) * v13) - (v11642 * v4099)) / v4094;
                                v4103 = v4094;
                                v4109 = v4099;
                                v4153 = v4088;
                                v9761 = v11642;
                                v9762 = v11651;
                                v9763 = v11627;
                            }
                            v4102 = v4103;
                            v4108 = v4109;
                            v4152 = v4153;
                            v9758 = v9761;
                            v9759 = v9762;
                            v9760 = v9763;
                        }
                        let v11654 = v10828 * v10390;
                        let v11657 = v10834 * v4102;
                        let v11662 = v9420 * v3632;
                        let v11663 = v9693 * v1048;
                        let v11666 = (Lanes([v11662[0], v11662[1], v11662[2], v11662[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v11663[0]]));
                        let v4107 = (((-v1200) + v3979) + (v1206 * v4102)) - (v1048 * v3632);
                        let v11668 = (((Lanes([v11654[0], v11654[1], v11654[2], v11654[3], v11654[4], 0.0])) + v9745) + ((Lanes([v11657[0], v11657[1], v11657[2], v11657[3], v11657[4], 0.0])) + (v9758 * v1206))) - (Lanes([v11666[0], v11666[1], 0.0, v11666[2], v11666[3], v11666[4]]));
                        let v11669 = v10834 * v4108;
                        let v11672 = (Lanes([v11669[0], v11669[1], v11669[2], v11669[3], v11669[4], 0.0])) + (v9759 * v1206);
                        let v4111 = v1 + (v1206 * v4108);
                        let v4115 = if (if v4112 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v3976 > v96 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v4138: f64;
                        let v4140: f64;
                        let v4142: f64;
                        let v9764: Lanes<6>;
                        if v4115 != 0.0 {
                            v4138 = v4116;
                            v4140 = v3979;
                            v4142 = v4112;
                            v9764 = v9745;
                        } else {
                            let v4118 = (-v4107) / v4111;
                            let v11676 = ((v11668 * v10390) - (v11672 * v4118)) / v4111;
                            let v4120 = v3979.abs();
                            let v11680 = v9745 * ((v10435 * (if v3979 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                            let v4121 = if v1 >= v4120 { 1.0 } else { 0.0 };
                            let v4122: f64;
                            let v9765: Lanes<6>;
                            if v4121 != 0.0 {
                                v4122 = v1;
                                v9765 = v11062;
                            } else {
                                v4122 = v4120;
                                v9765 = v11680;
                            }
                            let v4124 = v4119 * (v1 + v4122);
                            let v11681 = v9765 * v4119;
                            let v4126 = if (v4118.abs()) > v4124 { 1.0 } else { 0.0 };
                            let v4131: f64;
                            let v9766: Lanes<6>;
                            if v4126 != 0.0 {
                                let v4127 = if v4118 >= v0 { 1.0 } else { 0.0 };
                                let v4129: f64;
                                if v4127 != 0.0 {
                                    v4129 = v1;
                                } else {
                                    v4129 = v4128;
                                }
                                let v4130 = v4124 * v4129;
                                let v11682 = v11681 * v4129;
                                v4131 = v4130;
                                v9766 = v11682;
                            } else {
                                v4131 = v4118;
                                v9766 = v11676;
                            }
                            let v4132 = v3979 + v4131;
                            let v11683 = v9745 + v9766;
                            let v4137 = if (if (v4131.abs()) <= v861 { 1.0 } else { 0.0 }) != 0.0 && (if (v4107.abs()) <= v3506 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let v4143: f64;
                            if v4137 != 0.0 {
                                v4143 = v1;
                            } else {
                                v4143 = v4112;
                            }
                            v4138 = v3976;
                            v4140 = v4132;
                            v4142 = v4143;
                            v9764 = v11683;
                        }
                        let v4139 = v4138 + v1;
                        v3976 = v4139;
                        v3979 = v4140;
                        v4112 = v4142;
                        v4144 = v4074;
                        v4148 = v4102;
                        v4151 = v4152;
                        v9745 = v9764;
                        v9746 = v11611;
                        v9747 = v9758;
                        v9748 = v9760;
                    }
                    let v4145 = v4144 / v750;
                    let v11210 = v10485 * v4145;
                    let v11213 = (v9746 - (Lanes([0.0, 0.0, v11210[0], 0.0, 0.0, 0.0]))) / v750;
                    let v4149 = v4148 + (v4145 + v4146);
                    let v4150 = v1 / v4149;
                    let v4154 = v750 * v4151;
                    let v11218 = v10485 * v4151;
                    let v4156 = -(v4154 * v4150);
                    let v11225 = ((((Lanes([0.0, 0.0, v11218[0], 0.0, 0.0, 0.0])) + (v9748 * v750)) * v4150) + (((((v9747 + v11213) * v4150) * v10390) / v4149) * v4154)) * v10390;
                    let v4157 = v3979 - v3654;
                    let v11226 = v9745 - v9694;
                    let v4158 = v663 / v3839;
                    let v4161 = ((v4158 * v4157) + v1).sqrt();
                    let v4162 = v4161 + v1;
                    let v4163 = v1 / v4162;
                    let v4164 = v4163 / v3841;
                    let v4166 = v13 * (v3836 + v4145);
                    let v11244 = (v11066 + v11213) * v13;
                    let v11246 = v10828 + (Lanes([0.0, 0.0, v10415[0], 0.0, 0.0]));
                    let v4171 = (v1200 + v665) - (v13 * ((v78 * v3654) + v4157));
                    let v4173 = (-v4166) + v4164;
                    let v4174 = v663 * v1128;
                    let v11254 = v10410 * v1128;
                    let v11255 = v9421 * v663;
                    let v4175 = v663 * v750;
                    let v11262 = ((Lanes([0.0, 0.0, v11254[0], 0.0, 0.0])) + (Lanes([v11255[0], v11255[1], 0.0, v11255[2], v11255[3]]))) * v4171;
                    let v11266 = ((v10410 * v750) + (v10485 * v663)) * v4173;
                    let v4178 = (v4174 * v4171) + (v4175 * v4173);
                    let v11270 = ((Lanes([v11262[0], v11262[1], v11262[2], v11262[3], v11262[4], 0.0])) + (((Lanes([v11246[0], v11246[1], v11246[2], v11246[3], v11246[4], 0.0])) - (((v9694 * v78) + v11226) * v13)) * v4174)) + ((Lanes([0.0, 0.0, v11266[0], 0.0, 0.0, 0.0])) + (((v11244 * v10390) + (((((((((((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0, 0.0])) - (v11068 * v4158)) / v3839) * v4157) + (v11226 * v4158)) * (v9368 / (v10435 * v4161))) * v4163) * v10390) / v4162) - (v11066 * v4164)) / v3841)) * v4175));
                    let v4179 = v4144 + v3835;
                    let v11271 = v9746 + v9695;
                    let v4180 = v4179 / v78;
                    let v11272 = v11271 / v78;
                    let v4181 = v4156 + v3850;
                    let v11273 = v11225 + v11080;
                    let v4183 = (-v4181) / v78;
                    let v11275 = (v11273 * v10390) / v78;
                    let v4184 = v4144 - v3835;
                    let v11276 = v9746 - v9695;
                    let v4186 = -(v4156 - v3850);
                    let v11278 = (v11225 - v11080) * v10390;
                    let v4187 = v750 * v750;
                    let v11279 = v10485 * v750;
                    let v11280 = v11279 + v11279;
                    let v4191 = if v4188 <= v1 { 1.0 } else { 0.0 };
                    let v4202: f64;
                    let v9767: Lanes<6>;
                    if v4191 != 0.0 {
                        let v4192 = v4183 * v663;
                        let v11285 = v10410 * v4183;
                        let v4195 = v4184 * v4184;
                        let v11292 = v11276 * v4184;
                        let v4197 = (v4195 * v4184) / v4187;
                        let v11297 = v11280 * v4197;
                        let v4199 = ((v4192 * v4157) - v4186) - (v4197 / v646);
                        let v11302 = (((((v11275 * v663) + (Lanes([0.0, 0.0, v11285[0], 0.0, 0.0, 0.0]))) * v4157) + (v11226 * v4192)) - v11278) - ((((((v11292 + v11292) * v4184) + (v11276 * v4195)) - (Lanes([0.0, 0.0, v11297[0], 0.0, 0.0, 0.0]))) / v4187) / v646);
                        v4202 = v4199;
                        v9767 = v11302;
                    } else {
                        let v4200 = v4157 * v4178;
                        let v11283 = (v11226 * v4178) + (v11270 * v4157);
                        v4202 = v4200;
                        v9767 = v11283;
                    }
                    let v4204 = if (if v70 >= v1 { 1.0 } else { 0.0 }) != 0.0 && (if v4202 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4229: f64;
                    let v9768: Lanes<6>;
                    if v4204 != 0.0 {
                        v4229 = v0;
                        v9768 = v11062;
                    } else {
                        v4229 = v4202;
                        v9768 = v9767;
                    }
                    let v4424: f64;
                    let v9769: Lanes<6>;
                    if v4191 != 0.0 {
                        let v4206 = if (v4157.abs()) > v23 { 1.0 } else { 0.0 };
                        let v4425: f64;
                        let v9770: Lanes<6>;
                        if v4206 != 0.0 {
                            let v4207 = v4183 * v663;
                            let v11306 = v10410 * v4183;
                            let v4209 = (v4207 * v4157) - v4186;
                            let v4211 = v78 * v4180;
                            let v11316 = v11272 * v78;
                            let v4213 = v1128 / v663;
                            let v11318 = v10410 * v4213;
                            let v4215 = (v4211 * v4180) / v4187;
                            let v11326 = v11280 * v4215;
                            let v11331 = v11276 * v4184;
                            let v4218 = (v4184 * v4184) / v4187;
                            let v11333 = v11280 * v4218;
                            let v4220 = (v1 - v4215) + (v4218 / v15);
                            let v11339 = (((Lanes([v9421[0], v9421[1], 0.0, v9421[2], v9421[3]])) - (Lanes([0.0, 0.0, v11318[0], 0.0, 0.0]))) / v663) * v4220;
                            let v4222 = (v4183 - v4211) + (v4213 * v4220);
                            let v4223 = v4222 * v4184;
                            let v4224 = v4223 * v4184;
                            let v4226 = (v4224 * v4184) / v4187;
                            let v11353 = v11280 * v4226;
                            let v4230 = ((v4180 * v4209) + (v4226 / v646)) / v4229;
                            let v11361 = ((((v11272 * v4209) + ((((((v11275 * v663) + (Lanes([0.0, 0.0, v11306[0], 0.0, 0.0, 0.0]))) * v4157) + (v11226 * v4207)) - v11278) * v4180)) + (((((((((((v11275 - v11316) + ((Lanes([v11339[0], v11339[1], v11339[2], v11339[3], v11339[4], 0.0])) + (((((((v11316 * v4180) + (v11272 * v4211)) - (Lanes([0.0, 0.0, v11326[0], 0.0, 0.0, 0.0]))) / v4187) * v10390) + ((((v11331 + v11331) - (Lanes([0.0, 0.0, v11333[0], 0.0, 0.0, 0.0]))) / v4187) / v15)) * v4213))) * v4184) + (v11276 * v4222)) * v4184) + (v11276 * v4223)) * v4184) + (v11276 * v4224)) - (Lanes([0.0, 0.0, v11353[0], 0.0, 0.0, 0.0]))) / v4187) / v646)) - (v9768 * v4230)) / v4229;
                            v4425 = v4230;
                            v9770 = v11361;
                        } else {
                            v4425 = v4180;
                            v9770 = v11272;
                        }
                        v4424 = v4425;
                        v9769 = v9770;
                    } else {
                        let v4231 = v13 * v4179;
                        let v11303 = v11271 * v13;
                        v4424 = v4231;
                        v9769 = v11303;
                    }
                    let v4232 = v78 * v1206;
                    let v4233 = v4166 - v3841;
                    let v11364 = (v10834 * v78) * v4233;
                    let v4235 = v4157 + (v4232 * v4233);
                    let v4237 = v1 / v4236;
                    let v4240 = v1 - (v1 - (v4235 * v4237));
                    let v11376 = ((((v11226 + ((Lanes([v11364[0], v11364[1], v11364[2], v11364[3], v11364[4], 0.0])) + ((v11244 - v11066) * v4232))) * v4237) + ((((v9718 * v4237) * v10390) / v4236) * v4235)) * v10390) * v10390;
                    let v4241 = v4240 * v4240;
                    let v11377 = v11376 * v4240;
                    let v11378 = v11377 + v11377;
                    let v4242 = v4241 * v4241;
                    let v11379 = v11378 * v4241;
                    let v4243 = v4242 * v4241;
                    let v11386 = ((((v11379 + v11379) * v4241) + (v11378 * v4242)) * v4241) + (v11378 * v4243);
                    let v4246 = (v4243 * v4241) + v4245;
                    let v4263: f64;
                    let v9771: Lanes<6>;
                    if v4247 != 0.0 {
                        let v4257: f64;
                        if v4248 != 0.0 {
                            v4257 = v1;
                        } else {
                            let v4258: f64;
                            if v4249 != 0.0 {
                                v4258 = v78;
                            } else {
                                let v4259: f64;
                                if v4250 != 0.0 {
                                    v4259 = v96;
                                } else {
                                    let v4260: f64;
                                    if v4251 != 0.0 {
                                        v4260 = v90;
                                    } else {
                                        v4260 = v0;
                                    }
                                    v4259 = v4260;
                                }
                                v4258 = v4259;
                            }
                            v4257 = v4258;
                        }
                        let mut v4252: f64 = 0.0;
                        let mut v4254: f64 = 0.0;
                        let mut v9772: Lanes<6> = Lanes([0.0; 6]);
                        v4252 = v0;
                        v4254 = v4246;
                        v9772 = v11386;
                        loop {
                            let v4253 = if v4252 < v4257 { 1.0 } else { 0.0 };
                            if v4253 == 0.0 {
                                break;
                            }
                            let v4255 = v4254.sqrt();
                            let v11459 = v9772 * (v9368 / (v10435 * v4255));
                            let v4256 = v4252 + v1;
                            v4252 = v4256;
                            v4254 = v4255;
                            v9772 = v11459;
                        }
                        v4263 = v4254;
                        v9771 = v9772;
                    } else {
                        let v4262 = v4246.powf(v4261);
                        let v11390 = v11386 * (v4261 * (v4246.powf(v11387)));
                        v4263 = v4262;
                        v9771 = v11390;
                    }
                    let v4264 = v1 / v4263;
                    let v4266 = v1 - (v4240 * v4264);
                    let v11397 = ((v11376 * v4264) + ((((v9771 * v4264) * v10390) / v4263) * v4240)) * v10390;
                    let v4267 = v1 + v4266;
                    let v11400 = (v11397 * v4267) + (v11397 * v4266);
                    let v4269 = v1 + (v4266 * v4267);
                    let v4271 = if v4267 >= v4270 { 1.0 } else { 0.0 };
                    let v4273: f64;
                    let v9773: Lanes<6>;
                    if v4271 != 0.0 {
                        v4273 = v4267;
                        v9773 = v11397;
                    } else {
                        v4273 = v4272;
                        v9773 = v11062;
                    }
                    let v4431: f64;
                    let v9774: Lanes<6>;
                    if v4191 != 0.0 {
                        let v4276 = if (v4157.abs()) > v23 { 1.0 } else { 0.0 };
                        let v4432: f64;
                        let v9775: Lanes<6>;
                        if v4276 != 0.0 {
                            let v11402 = v11275 * v4183;
                            let v11404 = v11278 * v4186;
                            let v4280 = (v4183 * v4183) + ((v4186 * v4186) / v3523);
                            let v4281 = v4280 * v663;
                            let v11409 = v10410 * v4280;
                            let v4286 = v1128 / v663;
                            let v11420 = v10410 * v4286;
                            let v4287 = v4286 * v4184;
                            let v11425 = (((Lanes([v9421[0], v9421[1], 0.0, v9421[2], v9421[3]])) - (Lanes([0.0, 0.0, v11420[0], 0.0, 0.0]))) / v663) * v4184;
                            let v4289 = (v4287 * v4184) / v4187;
                            let v11432 = v11280 * v4289;
                            let v4291 = (v78 * v4183) + (v4289 / v644);
                            let v4292 = v4291 * v4184;
                            let v4293 = v4292 * v4184;
                            let v4295 = (v4293 * v4184) / v4187;
                            let v11447 = v11280 * v4295;
                            let v4298 = (((v4281 * v4157) - (v4183 * v4186)) - (v4295 / v646)) / v4229;
                            let v11455 = (((((((((v11402 + v11402) + ((v11404 + v11404) / v3523)) * v663) + (Lanes([0.0, 0.0, v11409[0], 0.0, 0.0, 0.0]))) * v4157) + (v11226 * v4281)) - ((v11275 * v4186) + (v11278 * v4183))) - (((((((((((v11275 * v78) + (((((((Lanes([v11425[0], v11425[1], v11425[2], v11425[3], v11425[4], 0.0])) + (v11276 * v4286)) * v4184) + (v11276 * v4287)) - (Lanes([0.0, 0.0, v11432[0], 0.0, 0.0, 0.0]))) / v4187) / v644)) * v4184) + (v11276 * v4291)) * v4184) + (v11276 * v4292)) * v4184) + (v11276 * v4293)) - (Lanes([0.0, 0.0, v11447[0], 0.0, 0.0, 0.0]))) / v4187) / v646)) - (v9768 * v4298)) / v4229;
                            v4432 = v4298;
                            v9775 = v11455;
                        } else {
                            v4432 = v4183;
                            v9775 = v11275;
                        }
                        v4431 = v4432;
                        v9774 = v9775;
                    } else {
                        let v4300 = v4299 * v4181;
                        let v11401 = v11273 * v4299;
                        v4431 = v4300;
                        v9774 = v11401;
                    }
                    let v4301 = if v3744 == v0 { 1.0 } else { 0.0 };
                    if v4301 != 0.0 {
                    } else {
                    }
                    let v4302 = if v4112 == v0 { 1.0 } else { 0.0 };
                    if v4302 != 0.0 {
                    } else {
                    }
                    let v4304 = if (v3744 + v4112) < v1 { 1.0 } else { 0.0 };
                    if v4304 != 0.0 {
                    } else {
                    }
                    v4309 = v4266;
                    v4313 = v4273;
                    v4316 = v4269;
                    v4338 = v3979;
                    v4384 = v4229;
                    v4423 = v4424;
                    v4430 = v4431;
                    v4447 = v4157;
                    v9725 = v11397;
                    v9726 = v9773;
                    v9727 = v11400;
                    v9728 = v9745;
                    v9729 = v9768;
                    v9730 = v9769;
                    v9731 = v9774;
                    v9732 = v11226;
                } else {
                    v4309 = v0;
                    v4313 = v0;
                    v4316 = v0;
                    v4338 = v4339;
                    v4384 = v0;
                    v4423 = v4426;
                    v4430 = v0;
                    v4447 = v0;
                    v9725 = v11062;
                    v9726 = v11062;
                    v9727 = v11062;
                    v9728 = v9719;
                    v9729 = v11062;
                    v9730 = v9720;
                    v9731 = v11062;
                    v9732 = v11062;
                }
                let v11456 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9693[0]]);
                v4305 = v3868;
                v4307 = v4309;
                v4311 = v4313;
                v4314 = v4316;
                v4325 = v4328;
                v4336 = v4338;
                v4340 = v3654;
                v4348 = v3849;
                v4381 = v4384;
                v4421 = v4423;
                v4428 = v4430;
                v4438 = v0;
                v4439 = v0;
                v4445 = v4447;
                v4637 = v0;
                v4735 = v736;
                v4787 = v733;
                v4843 = v4236;
                v4964 = v0;
                v4973 = v0;
                v4977 = v0;
                v5093 = v5095;
                v5501 = v3632;
                v5643 = v0;
                v5721 = v0;
                v5781 = v0;
                v8304 = v8306;
                v8481 = v8483;
                v8486 = v0;
                v8491 = v0;
                v8497 = v0;
                v8564 = v8565;
                v8576 = v8577;
                v9211 = v0;
                v9437 = v9725;
                v9438 = v9726;
                v9439 = v9727;
                v9440 = v9728;
                v9441 = v9694;
                v9442 = v11079;
                v9443 = v9729;
                v9444 = v9730;
                v9445 = v9731;
                v9446 = v11062;
                v9447 = v11062;
                v9448 = v9732;
                v9449 = v11062;
                v9450 = v10462;
                v9451 = v10457;
                v9452 = v9718;
                v9453 = v10579;
                v9454 = v10660;
                v9455 = v10579;
                v9456 = v9683;
                v9457 = v11456;
                v9458 = v10579;
                v9459 = v11062;
                v9460 = v9721;
                v9461 = v9722;
                v9462 = v11062;
                v9463 = v11062;
                v9464 = v11062;
                v9465 = v9723;
                v9466 = v9724;
                v9467 = v11062;
            }
            let v4306 = if v4305 == v0 { 1.0 } else { 0.0 };
            let v4876: f64;
            let v5525: f64;
            let v5778: f64;
            let v5780: f64;
            let v5789: f64;
            let v8265: f64;
            let v8285: f64;
            let v8288: f64;
            let v8300: f64;
            let v8309: f64;
            let v8368: f64;
            let v8374: f64;
            let v8378: f64;
            let v8408: f64;
            let v8480: f64;
            let v8484: f64;
            let v8488: f64;
            let v8489: f64;
            let v8495: f64;
            let v9118: f64;
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
            let v9786: Lanes<6>;
            let v9787: Lanes<6>;
            let v9788: Lanes<6>;
            let v9789: Lanes<6>;
            let v9790: Lanes<6>;
            let v9791: Lanes<6>;
            let v9792: Lanes<6>;
            let v9793: Lanes<6>;
            if v4306 != 0.0 {
                let v4317 = v4311 * v4314;
                let v4319 = (v708 * (v13 + v4307)) / v4317;
                let v4320 = v1707 - v4319;
                let v13756 = (((v9437 * v708) - (((v9438 * v4314) + (v9439 * v4311)) * v4319)) / v4317) * v10390;
                let v4322 = if v4320 > v4321 { 1.0 } else { 0.0 };
                let v4324: f64;
                let v9794: Lanes<6>;
                if v4322 != 0.0 {
                    let v4323 = if v70 >= v1 { 1.0 } else { 0.0 };
                    if v4323 != 0.0 {
                    } else {
                    }
                    v4324 = v13;
                    v9794 = v11062;
                } else {
                    v4324 = v4320;
                    v9794 = v13756;
                }
                let v4329 = if v4325 == v0 { 1.0 } else { 0.0 };
                let v4415: f64;
                let v8301: f64;
                let v9795: Lanes<6>;
                let v9796: Lanes<6>;
                if v4329 != 0.0 {
                    let v4335 = if (if v73 < v4330 { 1.0 } else { 0.0 }) != 0.0 && (if v4332 < v4333 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4413: f64;
                    let v8302: f64;
                    let v9797: Lanes<6>;
                    let v9798: Lanes<6>;
                    if v4335 != 0.0 {
                        let v4341 = v4340 + v866;
                        let v13822 = v9441 + (Lanes([v10561[0], v10561[1], 0.0, 0.0, v10561[2], 0.0]));
                        let v4344 = if v4336 > (v4341 - v4342) { 1.0 } else { 0.0 };
                        let v8303: f64;
                        let v9799: Lanes<6>;
                        if v4344 != 0.0 {
                            let v4346 = v4341 - v4345;
                            v8303 = v4346;
                            v9799 = v13822;
                        } else {
                            v8303 = v4336;
                            v9799 = v9440;
                        }
                        v4413 = v0;
                        v8302 = v8303;
                        v9797 = v11062;
                        v9798 = v9799;
                    } else {
                        if v565 != 0.0 {
                        } else {
                        }
                        let v4347 = v1 / v12;
                        let v4353 = (v4351 * v490) + (v4332 * (v4348 * v4347));
                        let v4354 = v1 / v4353;
                        let v4355 = v123 * v4354;
                        let v13762 = (((((v9442 * v4347) * v4332) * v4354) * v10390) / v4353) * v123;
                        let v4357 = v1 - v4356;
                        let v4361 = (v4356 * (v823 + v4340)) + (v4357 * v4336);
                        let v13767 = (((Lanes([v9410[0], v9410[1], 0.0, 0.0, 0.0, 0.0])) + v9441) * v4356) + (v9440 * v4357);
                        let v4362 = v4340 + v866;
                        let v13769 = v9441 + (Lanes([v10561[0], v10561[1], 0.0, 0.0, v10561[2], 0.0]));
                        let v4365 = if v4361 > (v4362 - v4363) { 1.0 } else { 0.0 };
                        let v4368: f64;
                        let v9800: Lanes<6>;
                        if v4365 != 0.0 {
                            let v4367 = v4362 - v4366;
                            v4368 = v4367;
                            v9800 = v13769;
                        } else {
                            v4368 = v4361;
                            v9800 = v13767;
                        }
                        let v4369 = v4368 - v4336;
                        let v13770 = v9800 - v9440;
                        let v13771 = v13770 * v4369;
                        let v4373 = ((v4369 * v4369) + v4371).sqrt();
                        let v13777 = (v13770 + ((v13771 + v13771) * (v9368 / (v10435 * v4373)))) * v13;
                        let v4377 = (v13 * (v4369 + v4373)) + v4376;
                        let v4378 = if v4377 < v0 { 1.0 } else { 0.0 };
                        let v4394: f64;
                        let v9801: Lanes<6>;
                        if v4378 != 0.0 {
                            v4394 = v0;
                            v9801 = v11062;
                        } else {
                            v4394 = v4377;
                            v9801 = v13777;
                        }
                        let v4379 = v663 * v4348;
                        let v13778 = v10410 * v4348;
                        let v4380 = v1 / v4379;
                        let v4385 = v4381 * v4380;
                        let v13787 = (v9443 * v4380) + ((((((Lanes([0.0, 0.0, v13778[0], 0.0, 0.0, 0.0])) + (v9442 * v663)) * v4380) * v10390) / v4379) * v4381);
                        let v4386 = if v4385 < v665 { 1.0 } else { 0.0 };
                        let v4391: f64;
                        let v9802: Lanes<6>;
                        if v4386 != 0.0 {
                            let v13788 = Lanes([0.0, 0.0, v10415[0], 0.0, 0.0, 0.0]);
                            v4391 = v665;
                            v9802 = v13788;
                        } else {
                            v4391 = v4385;
                            v9802 = v13787;
                        }
                        let v4390 = v1 / v136;
                        let v4393 = v78 * (v490 / v123);
                        let v4395 = v4393 * v4394;
                        let v13790 = v9801 * v4393;
                        let v4400 = (((v78 * v4391) + (v4395 * v4355)) + (v4389 * v4355)) * v4390;
                        let v4401 = v4400 * v4355;
                        let v13800 = (((((v9802 * v78) + ((v13790 * v4355) + (v13762 * v4395))) + (v13762 * v4389)) * v4390) * v4355) + (v13762 * v4400);
                        let v4403 = v90 * (v4395 + v4389);
                        let v4404 = v4403 * v4355;
                        let v13808 = v13800 * v4401;
                        let v4408 = ((v4401 * v4401) + (v4404 * v4355)).sqrt();
                        let v4411 = v13 * ((-v4401) + v4408);
                        let v4412 = v921 * v4411;
                        let v13817 = v10615 * v4411;
                        let v13820 = (Lanes([v13817[0], v13817[1], v13817[2], v13817[3], v13817[4], 0.0])) + ((((v13800 * v10390) + (((v13808 + v13808) + (((((v13790 * v90) * v4355) + (v13762 * v4403)) * v4355) + (v13762 * v4404))) * (v9368 / (v10435 * v4408)))) * v13) * v921);
                        v4413 = v4412;
                        v8302 = v4368;
                        v9797 = v13820;
                        v9798 = v9800;
                    }
                    let v4414 = v4413 * v267;
                    let v13823 = v9797 * v267;
                    v4415 = v4414;
                    v8301 = v8302;
                    v9795 = v13823;
                    v9796 = v9798;
                } else {
                    v4415 = v0;
                    v8301 = v8304;
                    v9795 = v11062;
                    v9796 = v9460;
                }
                let v4416 = v136 - v4415;
                let v13824 = v9795 * v10390;
                let v4417 = v139 - v4415;
                let v4418 = if v4416 < v616 { 1.0 } else { 0.0 };
                let v4525: f64;
                let v9803: Lanes<6>;
                if v4418 != 0.0 {
                    v4525 = v616;
                    v9803 = v11062;
                } else {
                    v4525 = v4416;
                    v9803 = v13824;
                }
                let v4420 = (-v168) * v139;
                let v4427 = v4420 * v4421;
                let v13825 = v9444 * v4420;
                let v4433 = v4420 * v4428;
                let v13826 = v9445 * v4420;
                let v4434 = v4433 * v13;
                let v13827 = v13826 * v13;
                let v8485: f64;
                let v8490: f64;
                let v8496: f64;
                let v9804: Lanes<6>;
                let v9805: Lanes<6>;
                let v9806: Lanes<6>;
                if v9 != 0.0 {
                    let v4435 = v4427 * v13;
                    let v13828 = v13825 * v13;
                    let v4437 = v4427 * v4436;
                    let v13829 = v13825 * v4436;
                    let v4444 = ((v13 * (v4438 + v4439)) * v139) * v168;
                    let v13833 = (((v9446 + v9447) * v13) * v139) * v168;
                    v8485 = v4444;
                    v8490 = v4435;
                    v8496 = v4437;
                    v9804 = v13833;
                    v9805 = v13828;
                    v9806 = v13829;
                } else {
                    v8485 = v8486;
                    v8490 = v8491;
                    v8496 = v8497;
                    v9804 = v9462;
                    v9805 = v9463;
                    v9806 = v9464;
                }
                let v4448 = v823 - v4445;
                let v13835 = (Lanes([v9410[0], v9410[1], 0.0, 0.0, 0.0, 0.0])) - v9448;
                let v4452 = (v78 * (v4448 / v78)) / v4451;
                let v13838 = ((v13835 / v78) * v78) / v4451;
                let v4460 = v4457 + (v4452 * v4458);
                let v4462 = v4456 + (v4452 * v4460);
                let v4464 = v4455 + (v4452 * v4462);
                let v4466 = v4454 + (v4452 * v4464);
                let v4468 = v4453 + (v4452 * v4466);
                let v4470 = v1 + (v4452 * v4468);
                let v4471 = v4451 / v4470;
                let v13857 = ((((v13838 * v4468) + (((v13838 * v4466) + (((v13838 * v4464) + (((v13838 * v4462) + (((v13838 * v4460) + ((v13838 * v4458) * v4452)) * v4452)) * v4452)) * v4452)) * v4452)) * v4471) * v10390) / v4470;
                let v4473 = if v4471 < v4472 { 1.0 } else { 0.0 };
                let v4475: f64;
                let v9807: Lanes<6>;
                if v4473 != 0.0 {
                    v4475 = v4474;
                    v9807 = v11062;
                } else {
                    v4475 = v4471;
                    v9807 = v13857;
                }
                let v4476 = v4340 + v4475;
                let v13858 = v9441 + v9807;
                let v4479 = v4428 / v556;
                let v13860 = v9445 / v556;
                let v4481 = v4480 / v4477;
                let v4483 = v4482 / v4477;
                let v4487 = v1 + ((v4336 - v4340) * v4484);
                let v4491 = ((v4481 * (v4421 / v556)) + (v4483 * v4479)) / v4487;
                let v13868 = ((((v9444 / v556) * v4481) + (v13860 * v4483)) - (((v9440 - v9441) * v4484) * v4491)) / v4487;
                let v13869 = v13868 * v4491;
                let v4495 = ((v4491 * v4491) + v4493).sqrt();
                let v13875 = (v13868 + ((v13869 + v13869) * (v9368 / (v10435 * v4495)))) * v13;
                let v4499 = (v13 * (v4491 + v4495)) + v4498;
                let v4500 = if v4499 < v0 { 1.0 } else { 0.0 };
                let v4501: f64;
                let v9808: Lanes<6>;
                if v4500 != 0.0 {
                    v4501 = v0;
                    v9808 = v11062;
                } else {
                    v4501 = v4499;
                    v9808 = v13875;
                }
                let v4503 = v4502 - v1;
                let v4504 = v4501.powf(v4503);
                let v4505 = v4504 * v4501;
                let v4506 = v183 - v1;
                let v4507 = v4501.powf(v4506);
                let v4515 = v4510 + ((v4511 * (v4479 / v206)) / v4513);
                let v4516 = v1 / v4515;
                let v13896 = v10421 * v4505;
                let v4521 = (v4516 + (v702 * v4505)) + ((v4507 * v4501) / v4519);
                let v4522 = v1 / v4521;
                let v4523 = v4522 * v29;
                let v13906 = (((((((((((v13860 / v206) * v4511) / v4513) * v4516) * v10390) / v4515) + ((Lanes([0.0, 0.0, v13896[0], 0.0, 0.0, 0.0])) + ((((v9808 * (v4503 * (v4501.powf((v4503 - v9368))))) * v4501) + (v9808 * v4504)) * v702))) + ((((v9808 * (v4506 * (v4501.powf((v4506 - v9368))))) * v4501) + (v9808 * v4507)) / v4519)) * v4522) * v10390) / v4521) * v29;
                let v4524 = v663 * v4348;
                let v13907 = v10410 * v4348;
                let v4526 = v4524 * v4525;
                let v13913 = (((Lanes([0.0, 0.0, v13907[0], 0.0, 0.0, 0.0])) + (v9442 * v663)) * v4525) + (v9803 * v4524);
                let v13914 = v13913 * v4526;
                let v4530 = ((v4526 * v4526) + v4528).sqrt();
                let v13920 = (v13913 + ((v13914 + v13914) * (v9368 / (v10435 * v4530)))) * v13;
                let v4534 = (v13 * (v4526 + v4530)) + v4533;
                let v4535 = if v4534 < v0 { 1.0 } else { 0.0 };
                let v4536: f64;
                let v9809: Lanes<6>;
                if v4535 != 0.0 {
                    v4536 = v0;
                    v9809 = v11062;
                } else {
                    v4536 = v4534;
                    v9809 = v13920;
                }
                let v4537 = v1 / v4536;
                let v4538 = v4381 * v4537;
                let v13927 = v10434 * v1889;
                let v4540 = (v1889 * v717) / v4523;
                let v13932 = ((v9443 * v4537) + ((((v9809 * v4537) * v10390) / v4536) * v4381)) * v4538;
                let v13934 = (((Lanes([0.0, 0.0, v13927[0], 0.0, 0.0, 0.0])) - (v13906 * v4540)) / v4523) * v4540;
                let v4544 = ((v4538 * v4538) + (v4540 * v4540)).sqrt();
                let v13939 = ((v13932 + v13932) + (v13934 + v13934)) * (v9368 / (v10435 * v4544));
                let v4546 = (v4523 * v4544) / v717;
                let v13943 = v10434 * v4546;
                let v13946 = (((v13906 * v4544) + (v13939 * v4523)) - (Lanes([0.0, 0.0, v13943[0], 0.0, 0.0, 0.0]))) / v717;
                let v4552 = if (if v4547 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v4550 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4560: f64;
                let v9810: Lanes<6>;
                if v4552 != 0.0 {
                    v4560 = v1;
                    v9810 = v11062;
                } else {
                    let v4557 = if (if v4553 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v4555 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4561: f64;
                    let v9811: Lanes<6>;
                    if v4557 != 0.0 {
                        v4561 = v4546;
                        v9811 = v13946;
                    } else {
                        let v4558 = v4548 - v1;
                        let v4559 = v4546.powf(v4558);
                        let v13950 = v13946 * (v4558 * (v4546.powf((v4558 - v9368))));
                        v4561 = v4559;
                        v9811 = v13950;
                    }
                    v4560 = v4561;
                    v9810 = v9811;
                }
                let v13953 = (v13946 * v4560) + (v9810 * v4546);
                let v4563 = v1 + (v4546 * v4560);
                let v4568 = if (if v4564 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v4566 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4582: f64;
                let v9812: Lanes<6>;
                if v4568 != 0.0 {
                    let v4569 = v1 / v4563;
                    let v13969 = ((v13953 * v4569) * v10390) / v4563;
                    v4582 = v4569;
                    v9812 = v13969;
                } else {
                    let v4574 = if (if v4570 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v4572 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4583: f64;
                    let v9813: Lanes<6>;
                    if v4574 != 0.0 {
                        let v4575 = v4563.sqrt();
                        let v4576 = v1 / v4575;
                        let v13966 = (((v13953 * (v9368 / (v10435 * v4575))) * v4576) * v10390) / v4575;
                        v4583 = v4576;
                        v9813 = v13966;
                    } else {
                        let v4579 = (v4577 / v4548) - v1;
                        let v4580 = v4563.powf(v4579);
                        let v4581 = v4563 * v4580;
                        let v13960 = (v13953 * v4580) + ((v13953 * (v4579 * (v4563.powf((v4579 - v9368))))) * v4563);
                        v4583 = v4581;
                        v9813 = v13960;
                    }
                    v4582 = v4583;
                    v9812 = v9813;
                }
                let v4584 = v4523 * v4582;
                let v13972 = (v13906 * v4582) + (v9812 * v4523);
                let v13973 = v10415 * v166;
                let v4586 = (v166 * v665) / v4416;
                let v13977 = ((Lanes([0.0, 0.0, v13973[0], 0.0, 0.0, 0.0])) - (v13824 * v4586)) / v4416;
                let v4587 = v4586 * v4381;
                let v4588 = v4587 * v4584;
                let v13983 = (((v13977 * v4381) + (v9443 * v4586)) * v4584) + (v13972 * v4587);
                let v4592 = if (if v4589 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v212 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4648: f64;
                let v9814: Lanes<6>;
                if v4592 != 0.0 {
                    let v4595 = (v78 * (v13 * v4448)) / v20;
                    let v13986 = ((v13835 * v13) * v78) / v20;
                    let v4603 = v4600 + (v4595 * v4601);
                    let v4605 = v4599 + (v4595 * v4603);
                    let v4607 = v4598 + (v4595 * v4605);
                    let v4609 = v4597 + (v4595 * v4607);
                    let v4611 = v4596 + (v4595 * v4609);
                    let v4613 = v1 + (v4595 * v4611);
                    let v4614 = v20 / v4613;
                    let v4616 = v4340 + v4614;
                    let v14006 = v9441 + (((((v13986 * v4611) + (((v13986 * v4609) + (((v13986 * v4607) + (((v13986 * v4605) + (((v13986 * v4603) + ((v13986 * v4601) * v4595)) * v4595)) * v4595)) * v4595)) * v4595)) * v4614) * v10390) / v4613);
                    let v4617 = v4615 - v4616;
                    let v14007 = v14006 * v10390;
                    let v14008 = v14007 * v4617;
                    let v4621 = ((v4617 * v4617) + v4619).sqrt();
                    let v14014 = (v14007 + ((v14008 + v14008) * (v9368 / (v10435 * v4621)))) * v13;
                    let v4625 = (v13 * (v4617 + v4621)) + v4624;
                    let v4626 = if v4625 < v0 { 1.0 } else { 0.0 };
                    let v4629: f64;
                    let v9815: Lanes<6>;
                    if v4626 != 0.0 {
                        v4629 = v0;
                        v9815 = v11062;
                    } else {
                        v4629 = v4625;
                        v9815 = v14014;
                    }
                    let v4627 = v663 * v216;
                    let v4628 = v1128 * v4627;
                    let v14016 = v9421 * v4627;
                    let v14017 = (v10410 * v216) * v1128;
                    let v4631 = v4629.powf(v4630);
                    let v4632 = v4628 * v4631;
                    let v14025 = ((Lanes([v14016[0], v14016[1], 0.0, v14016[2], v14016[3]])) + (Lanes([0.0, 0.0, v14017[0], 0.0, 0.0]))) * v4631;
                    let v14028 = (Lanes([v14025[0], v14025[1], v14025[2], v14025[3], v14025[4], 0.0])) + ((v9815 * (v4630 * (v4629.powf((v4630 - v9368))))) * v4628);
                    let v14029 = v10561 * v4633;
                    let v4635 = v1 + (v866 * v4633);
                    let v4640: f64;
                    let v9816: Lanes<6>;
                    if v987 != 0.0 {
                        let v4636 = v4616 - v864;
                        let v14032 = v14006 - (Lanes([v10558[0], v10558[1], 0.0, 0.0, v10558[2], 0.0]));
                        v4640 = v4636;
                        v9816 = v14032;
                    } else {
                        let v4638 = v4616 - v4637;
                        let v14030 = v14006 - v9449;
                        v4640 = v4638;
                        v9816 = v14030;
                    }
                    let v4639 = v866 * v221;
                    let v14034 = (v10561 * v221) * v4640;
                    let v4642 = v4635 + (v4639 * v4640);
                    let v4643 = v4632 * v4642;
                    let v14042 = (v14028 * v4642) + (((Lanes([v14029[0], v14029[1], 0.0, 0.0, v14029[2], 0.0])) + ((Lanes([v14034[0], v14034[1], 0.0, 0.0, v14034[2], 0.0])) + (v9816 * v4639))) * v4632);
                    v4648 = v4643;
                    v9814 = v14042;
                } else {
                    v4648 = v0;
                    v9814 = v11062;
                }
                let v4644 = if v222 != v0 { 1.0 } else { 0.0 };
                let v4649: f64;
                let v9817: Lanes<5>;
                if v4644 != 0.0 {
                    let v4645 = v663 * v227;
                    let v4646 = v1128 * v4645;
                    let v14044 = v9421 * v4645;
                    let v14045 = (v10410 * v227) * v1128;
                    let v4647 = v4646 * v866;
                    let v14050 = v10561 * v4646;
                    let v14052 = (((Lanes([v14044[0], v14044[1], 0.0, v14044[2], v14044[3]])) + (Lanes([0.0, 0.0, v14045[0], 0.0, 0.0]))) * v866) + (Lanes([v14050[0], v14050[1], 0.0, 0.0, v14050[2]]));
                    v4649 = v4647;
                    v9817 = v14052;
                } else {
                    v4649 = v0;
                    v9817 = v10579;
                }
                let v4650 = v4648 + v4649;
                let v14054 = v9814 + (Lanes([v9817[0], v9817[1], v9817[2], v9817[3], v9817[4], 0.0]));
                let v4651 = if v4650 > v0 { 1.0 } else { 0.0 };
                let v4655: f64;
                let v9818: Lanes<6>;
                if v4651 != 0.0 {
                    let v4652 = v4445 * v4650;
                    let v4653 = v4586 * v4652;
                    let v4654 = v4653 * v4584;
                    let v14063 = (((v13977 * v4652) + (((v9448 * v4650) + (v14054 * v4445)) * v4586)) * v4584) + (v13972 * v4653);
                    v4655 = v4654;
                    v9818 = v14063;
                } else {
                    v4655 = v0;
                    v9818 = v11062;
                }
                let v4656 = v4588 + v4655;
                let v14064 = v13983 + v9818;
                let v4658 = if v4657 != v0 { 1.0 } else { 0.0 };
                let v4877: f64;
                let v9819: Lanes<6>;
                if v4658 != 0.0 {
                    let v4659 = v245 - v1102;
                    let v4661 = v1 / (v4659 * v4659);
                    let v4662 = v78 * v1101;
                    let v4666 = ((v4662 * (v123 * v1048)) * v516) * v4661;
                    let v4667 = v4666 * v1066;
                    let v14069 = ((((v9420 * v123) * v4662) * v516) * v4661) * v1066;
                    let v14070 = v10725 * v4666;
                    let v4671 = v4668 + (v4669 * v866);
                    let v4672 = v4667 * v4671;
                    let v14076 = (v10561 * v4669) * v4667;
                    let v14078 = (((Lanes([v14069[0], v14069[1], 0.0, v14069[2], v14069[3]])) + (Lanes([v14070[0], v14070[1], v14070[2], 0.0, v14070[3]]))) * v4671) + (Lanes([v14076[0], v14076[1], 0.0, 0.0, v14076[2]]));
                    let v14080 = (v9410 * v4674) * v10390;
                    let v14082 = v10564 + (Lanes([v14080[0], v14080[1], 0.0, 0.0]));
                    let v4679 = ((v867 - v240) + (v4673 - (v4674 * v823))) + v4672;
                    let v14084 = (Lanes([v14082[0], v14082[1], 0.0, v14082[2], v14082[3]])) + v14078;
                    let v4680 = v734 * v1048;
                    let v14085 = v10459 * v1048;
                    let v14086 = v9420 * v734;
                    let v4681 = v4680 * v1048;
                    let v14091 = v9420 * v4680;
                    let v14093 = (((Lanes([0.0, 0.0, v14085[0], 0.0, 0.0])) + (Lanes([v14086[0], v14086[1], 0.0, v14086[2], v14086[3]]))) * v1048) + (Lanes([v14091[0], v14091[1], 0.0, v14091[2], v14091[3]]));
                    let v14095 = v10410 * v4681;
                    let v4683 = (v4681 * v663) * v13;
                    let v14098 = ((v14093 * v663) + (Lanes([0.0, 0.0, v14095[0], 0.0, 0.0]))) * v13;
                    let v14100 = v10410 * v4683;
                    let v4685 = (v4683 * v663) * v78;
                    let v14103 = ((v14098 * v663) + (Lanes([0.0, 0.0, v14100[0], 0.0, 0.0]))) * v78;
                    let v4686 = v663 * v2050;
                    let v14106 = (v10410 * v2050) * v4681;
                    let v14111 = ((Lanes([0.0, 0.0, v10415[0], 0.0, 0.0])) - ((v14093 * v4686) + (Lanes([0.0, 0.0, v14106[0], 0.0, 0.0])))) - v14078;
                    let v4692 = ((((v665 - (v4681 * v4686)) + v240) - v4673) - v4672) + v362;
                    let v14113 = (Lanes([v10564[0], v10564[1], 0.0, v10564[2], v10564[3]])) - v14111;
                    let v4694 = (v867 - v4692) - v3684;
                    let v4695 = if v4692 >= v0 { 1.0 } else { 0.0 };
                    let v4697: f64;
                    if v4695 != 0.0 {
                        v4697 = v1;
                    } else {
                        v4697 = v4696;
                    }
                    let v14114 = v14113 * v4694;
                    let v4699 = v4697 * v90;
                    let v4703 = ((v4694 * v4694) + ((v4699 * v4692) * v3684)).sqrt();
                    let v4710 = ((((v4692 + (v13 * (v4694 + v4703))) - v240) + v4673) + v4672) - v988;
                    let v14126 = Lanes([v9416[0], v9416[1], 0.0, 0.0, v9416[2]]);
                    let v14128 = v10410 * v4710;
                    let v4712 = (v663 * v4710) - v1;
                    let v4713 = v90 / v4685;
                    let v14137 = (((Lanes([0.0, 0.0, v14128[0], 0.0, 0.0])) + ((((v14111 + ((v14113 + (((v14114 + v14114) + ((v14111 * v4699) * v3684)) * (v9368 / (v10435 * v4703)))) * v13)) + v14078) - v14126) * v663)) * v4713) + ((((v14103 * v4713) * v10390) / v4685) * v4712);
                    let v4715 = v1 + (v4712 * v4713);
                    let v14138 = v14137 * v4715;
                    let v4719 = ((v4715 * v4715) + v4717).sqrt();
                    let v14144 = (v14137 + ((v14138 + v14138) * (v9368 / (v10435 * v4719)))) * v13;
                    let v4723 = (v13 * (v4715 + v4719)) + v4722;
                    let v4724 = if v4723 < v0 { 1.0 } else { 0.0 };
                    let v4725: f64;
                    let v9820: Lanes<5>;
                    if v4724 != 0.0 {
                        v4725 = v0;
                        v9820 = v10579;
                    } else {
                        v4725 = v4723;
                        v9820 = v14144;
                    }
                    let v4727 = (v4725 + v362).sqrt();
                    let v4728 = v1 - v4727;
                    let v4730 = v4679 + (v4683 * v4728);
                    let v14152 = v14084 + ((v14098 * v4728) + (((v9820 * (v9368 / (v10435 * v4727))) * v10390) * v4683));
                    let v4731 = v4679 + v362;
                    let v4732 = v78 / v4731;
                    let v4733 = v663 + v4732;
                    let v4734 = v1 / v4733;
                    let v4737 = v1 / v4735;
                    let v14163 = ((v9450 * v4737) * v10390) / v4735;
                    let v4738 = v4737 / v4681;
                    let v4739 = v4679 * v4679;
                    let v14168 = v14084 * v4679;
                    let v4740 = v4738 * v4739;
                    let v4741 = v4740.ln();
                    let v4742 = v4741 * v4734;
                    let v14177 = (((((((Lanes([0.0, 0.0, v14163[0], 0.0, 0.0])) - (v14093 * v4738)) / v4681) * v4739) + ((v14168 + v14168) * v4738)) * (v9368 / v4740)) * v4734) + ((((((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + (((v14084 * v4732) * v10390) / v4731)) * v4734) * v10390) / v4733) * v4741);
                    let v14178 = v14177 - v14152;
                    let v4745 = (v4742 - v4730) - v4744;
                    let v14179 = v14178 * v4745;
                    let v4750 = ((v4745 * v4745) + (v4747 * v4742)).sqrt();
                    let v4753 = v4742 - (v13 * (v4745 + v4750));
                    let v14188 = v14177 - ((v14178 + (((v14179 + v14179) + (v14177 * v4747)) * (v9368 / (v10435 * v4750)))) * v13);
                    let v14189 = v10410 * v4753;
                    let v4755 = (v663 * v4753).exp();
                    let v14194 = v9450 * v4755;
                    let v4757 = v4753 - v988;
                    let v14199 = v10410 * v4757;
                    let v14202 = (Lanes([0.0, 0.0, v14199[0], 0.0, 0.0])) + ((v14188 - v14126) * v663);
                    let v4759 = (v663 * v4757) - v1;
                    let v4760 = v4759 + (v4735 * v4755);
                    let v14203 = v14202 + ((Lanes([0.0, 0.0, v14194[0], 0.0, 0.0])) + ((((Lanes([0.0, 0.0, v14189[0], 0.0, 0.0])) + (v14188 * v663)) * v4755) * v4735));
                    let v14204 = v14203 * v4760;
                    let v4764 = ((v4760 * v4760) + v4762).sqrt();
                    let v14210 = (v14203 + ((v14204 + v14204) * (v9368 / (v10435 * v4764)))) * v13;
                    let v4768 = (v13 * (v4760 + v4764)) + v4767;
                    let v4769 = if v4768 < v0 { 1.0 } else { 0.0 };
                    let v4770: f64;
                    let v9821: Lanes<5>;
                    if v4769 != 0.0 {
                        v4770 = v0;
                        v9821 = v10579;
                    } else {
                        v4770 = v4768;
                        v9821 = v14210;
                    }
                    let v4773 = (v4770 + v4771).sqrt();
                    let v14213 = v9821 * (v9368 / (v10435 * v4773));
                    let v14214 = v14202 * v4759;
                    let v4777 = ((v4759 * v4759) + v4775).sqrt();
                    let v14220 = (v14202 + ((v14214 + v14214) * (v9368 / (v10435 * v4777)))) * v13;
                    let v4781 = (v13 * (v4759 + v4777)) + v4780;
                    let v4782 = if v4781 < v0 { 1.0 } else { 0.0 };
                    let v4783: f64;
                    let v9822: Lanes<5>;
                    if v4782 != 0.0 {
                        v4783 = v0;
                        v9822 = v10579;
                    } else {
                        v4783 = v4781;
                        v9822 = v14220;
                    }
                    let v4786 = (v4783 + v4784).sqrt();
                    let v4789 = v4773 - v4786;
                    let v4790 = v4787 * v4789;
                    let v14225 = v9451 * v4789;
                    let v14228 = (Lanes([0.0, 0.0, v14225[0], 0.0, 0.0])) + ((v14213 - (v9822 * (v9368 / (v10435 * v4786)))) * v4787);
                    let v4791 = v4730 - v4753;
                    let v14229 = v14152 - v14188;
                    let v14230 = v14229 * v4791;
                    let v4795 = ((v4791 * v4791) + v4793).sqrt();
                    let v14236 = (v14229 + ((v14230 + v14230) * (v9368 / (v10435 * v4795)))) * v13;
                    let v4799 = (v13 * (v4791 + v4795)) + v4798;
                    let v4800 = if v4799 < v0 { 1.0 } else { 0.0 };
                    let v4801: f64;
                    let v9823: Lanes<5>;
                    if v4800 != 0.0 {
                        v4801 = v0;
                        v9823 = v10579;
                    } else {
                        v4801 = v4799;
                        v9823 = v14236;
                    }
                    let v4803 = v4801 + v4802;
                    let v4804 = v823 / v4803;
                    let v14239 = (v10597 - (v9823 * v4804)) / v4803;
                    let v4805 = v4804 * v4804;
                    let v14240 = v14239 * v4804;
                    let v14241 = v14240 + v14240;
                    let v4806 = v4805 * v4805;
                    let v14242 = v14241 * v4805;
                    let v4807 = v4806 * v4805;
                    let v14249 = ((((v14242 + v14242) * v4805) + (v14241 * v4806)) * v4805) + (v14241 * v4807);
                    let v4810 = (v4807 * v4805) + v4809;
                    let v4827: f64;
                    let v9824: Lanes<5>;
                    if v4811 != 0.0 {
                        let v4821: f64;
                        if v4812 != 0.0 {
                            v4821 = v1;
                        } else {
                            let v4822: f64;
                            if v4813 != 0.0 {
                                v4822 = v78;
                            } else {
                                let v4823: f64;
                                if v4814 != 0.0 {
                                    v4823 = v96;
                                } else {
                                    let v4824: f64;
                                    if v4815 != 0.0 {
                                        v4824 = v90;
                                    } else {
                                        v4824 = v0;
                                    }
                                    v4823 = v4824;
                                }
                                v4822 = v4823;
                            }
                            v4821 = v4822;
                        }
                        let mut v4816: f64 = 0.0;
                        let mut v4818: f64 = 0.0;
                        let mut v9825: Lanes<5> = Lanes([0.0; 5]);
                        v4816 = v0;
                        v4818 = v4810;
                        v9825 = v14249;
                        loop {
                            let v4817 = if v4816 < v4821 { 1.0 } else { 0.0 };
                            if v4817 == 0.0 {
                                break;
                            }
                            let v4819 = v4818.sqrt();
                            let v18861 = v9825 * (v9368 / (v10435 * v4819));
                            let v4820 = v4816 + v1;
                            v4816 = v4820;
                            v4818 = v4819;
                            v9825 = v18861;
                        }
                        v4827 = v4818;
                        v9824 = v9825;
                    } else {
                        let v4826 = v4810.powf(v4825);
                        let v14253 = v14249 * (v4825 * (v4810.powf(v14250)));
                        v4827 = v4826;
                        v9824 = v14253;
                    }
                    let v4828 = v1 / v4827;
                    let v4829 = v4804 * v4828;
                    let v4831 = (v78 * v262) * v145;
                    let v4832 = v4831 * v665;
                    let v4833 = v4832 * v4584;
                    let v14261 = (v10415 * v4831) * v4584;
                    let v4834 = v4833 * v4790;
                    let v14266 = v14228 * v4833;
                    let v14270 = ((v14239 * v4828) + ((((v9824 * v4828) * v10390) / v4827) * v4804)) * v4834;
                    let v4836 = (v4834 * v4829) / v4525;
                    let v4837 = v4656 + v4836;
                    let v14276 = v14064 + ((((((((Lanes([0.0, 0.0, v14261[0], 0.0, 0.0, 0.0])) + (v13972 * v4832)) * v4790) + (Lanes([v14266[0], v14266[1], v14266[2], v14266[3], v14266[4], 0.0]))) * v4829) + (Lanes([v14270[0], v14270[1], v14270[2], v14270[3], v14270[4], 0.0]))) - (v9803 * v4836)) / v4525);
                    v4877 = v4837;
                    v9819 = v14276;
                } else {
                    v4877 = v4656;
                    v9819 = v14064;
                }
                let v4842 = if (if v4838 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v4840 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8369: f64;
                let v8375: f64;
                let v8379: f64;
                let v8409: f64;
                let v9826: Lanes<6>;
                let v9827: Lanes<6>;
                let v9828: Lanes<6>;
                if v4842 != 0.0 {
                    let v4845 = v4843 * v4843;
                    let v14277 = v9452 * v4843;
                    let v14278 = v14277 + v14277;
                    let v4846 = v78 * v665;
                    let v4847 = v4846 * v1048;
                    let v14280 = (v10415 * v78) * v1048;
                    let v14281 = v9420 * v4846;
                    let v14285 = ((Lanes([0.0, 0.0, v14280[0], 0.0, 0.0])) + (Lanes([v14281[0], v14281[1], 0.0, v14281[2], v14281[3]]))) * v4381;
                    let v4849 = v4845 - (v4847 * v4381);
                    let v14289 = v14278 - ((Lanes([v14285[0], v14285[1], v14285[2], v14285[3], v14285[4], 0.0])) + (v9443 * v4847));
                    let v14290 = v14278 * v4845;
                    let v4853 = ((v4845 * v4845) + v4851).sqrt();
                    let v14296 = (v14278 + ((v14290 + v14290) * (v9368 / (v10435 * v4853)))) * v13;
                    let v4857 = (v13 * (v4845 + v4853)) + v4856;
                    let v4858 = if v4857 < v0 { 1.0 } else { 0.0 };
                    let v4868: f64;
                    let v9829: Lanes<6>;
                    if v4858 != 0.0 {
                        v4868 = v0;
                        v9829 = v11062;
                    } else {
                        v4868 = v4857;
                        v9829 = v14296;
                    }
                    let v14297 = v14289 * v4849;
                    let v4862 = ((v4849 * v4849) + v4860).sqrt();
                    let v14303 = (v14289 + ((v14297 + v14297) * (v9368 / (v10435 * v4862)))) * v13;
                    let v4866 = (v13 * (v4849 + v4862)) + v4865;
                    let v4867 = if v4866 < v0 { 1.0 } else { 0.0 };
                    let v4869: f64;
                    let v9830: Lanes<6>;
                    if v4867 != 0.0 {
                        v4869 = v0;
                        v9830 = v11062;
                    } else {
                        v4869 = v4866;
                        v9830 = v14303;
                    }
                    let v4870 = v4868 - v4869;
                    let v14304 = v9829 - v9830;
                    let v4875 = if (if v4348 < v4871 { 1.0 } else { 0.0 }) != 0.0 || (if v4870 < v4873 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8370: f64;
                    if v4875 != 0.0 {
                        v8370 = v0;
                    } else {
                        v8370 = v1;
                    }
                    v8369 = v8370;
                    v8375 = v4869;
                    v8379 = v4868;
                    v8409 = v4870;
                    v9826 = v9830;
                    v9827 = v9829;
                    v9828 = v14304;
                } else {
                    v8369 = v0;
                    v8375 = v0;
                    v8379 = v0;
                    v8409 = v0;
                    v9826 = v11062;
                    v9827 = v11062;
                    v9828 = v11062;
                }
                v4876 = v4877;
                v5525 = v4476;
                v5778 = v4586;
                v5780 = v4584;
                v5789 = v4544;
                v8265 = v4525;
                v8285 = v4433;
                v8288 = v4417;
                v8300 = v8301;
                v8309 = v4523;
                v8368 = v8369;
                v8374 = v8375;
                v8378 = v8379;
                v8408 = v8409;
                v8480 = v4427;
                v8484 = v8485;
                v8488 = v4434;
                v8489 = v8490;
                v8495 = v8496;
                v9118 = v4324;
                v9776 = v9819;
                v9777 = v13858;
                v9778 = v13977;
                v9779 = v13972;
                v9780 = v13939;
                v9781 = v9803;
                v9782 = v13826;
                v9783 = v9796;
                v9784 = v13906;
                v9785 = v9826;
                v9786 = v9827;
                v9787 = v9828;
                v9788 = v13825;
                v9789 = v9804;
                v9790 = v13827;
                v9791 = v9805;
                v9792 = v9806;
                v9793 = v9794;
            } else {
                v4876 = v0;
                v5525 = v1;
                v5778 = v1;
                v5780 = v5781;
                v5789 = v0;
                v8265 = v136;
                v8285 = v0;
                v8288 = v0;
                v8300 = v8304;
                v8309 = v0;
                v8368 = v0;
                v8374 = v0;
                v8378 = v0;
                v8408 = v0;
                v8480 = v8481;
                v8484 = v8486;
                v8488 = v0;
                v8489 = v8491;
                v8495 = v8497;
                v9118 = v13;
                v9776 = v11062;
                v9777 = v11062;
                v9778 = v11062;
                v9779 = v11062;
                v9780 = v11062;
                v9781 = v11062;
                v9782 = v11062;
                v9783 = v9460;
                v9784 = v11062;
                v9785 = v11062;
                v9786 = v11062;
                v9787 = v11062;
                v9788 = v9461;
                v9789 = v9462;
                v9790 = v11062;
                v9791 = v9463;
                v9792 = v9464;
                v9793 = v11062;
            }
            let v4881 = if (if v4589 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v4879 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5620: f64;
            let v6024: f64;
            let v9831: Lanes<6>;
            let v9832: Lanes<6>;
            if v4881 != 0.0 {
                let v4883 = v1200 - v4882;
                let v4884 = v1143 + v4882;
                let v4886 = v41 / v731;
                let v4888 = (v4886 * v489) / v731;
                let v4889 = v4888.ln();
                let v4890 = v665 * v4889;
                let v14316 = (v10415 * v4889) + ((((((((v10453 * v4886) * v10390) / v731) * v489) - (v10453 * v4888)) / v731) * (v9368 / v4888)) * v665);
                let v4891: f64;
                let v9833: Lanes<6>;
                if v565 != 0.0 {
                    let v14317 = Lanes([v9425[0], v9425[1], v9425[2], 0.0, v9425[3], 0.0]);
                    v4891 = v1037;
                    v9833 = v14317;
                } else {
                    v4891 = v4637;
                    v9833 = v9449;
                }
                let v4898 = v489 + v41;
                let v4900 = (((((v4892 * (v4890 - v4891)) / v123) * v489) * v41) / v4898).sqrt();
                let v4901 = v4900 * v142;
                let v14328 = ((((((((Lanes([0.0, 0.0, v14316[0], 0.0, 0.0, 0.0])) - v9833) * v4892) / v123) * v489) * v41) / v4898) * (v9368 / (v10435 * v4900))) * v142;
                let v4903 = v4902 * v4901;
                let v4905 = v823 + v4901;
                let v14333 = Lanes([v9410[0], v9410[1], 0.0, 0.0, 0.0, 0.0]);
                let v4906 = (v4903 * v4901) / v4905;
                let v14337 = ((((v14328 * v4902) * v4901) + (v14328 * v4903)) - ((v14333 + v14328) * v4906)) / v4905;
                let v4907 = v4883 - v4906;
                let v14338 = Lanes([v10828[0], v10828[1], v10828[2], v10828[3], v10828[4], 0.0]);
                let v4908 = v663 * v4907;
                let v14340 = v10410 * v4907;
                let v14343 = (Lanes([0.0, 0.0, v14340[0], 0.0, 0.0, 0.0])) + ((v14338 - v14337) * v663);
                let v4911 = v1207 * v664;
                let v14346 = v10412 * v1207;
                let v4912 = (v90 * (v4908 - v1)) / v4911;
                let v14349 = ((v10836 * v664) + (Lanes([0.0, 0.0, v14346[0], 0.0, 0.0]))) * v4912;
                let v14352 = ((v14343 * v90) - (Lanes([v14349[0], v14349[1], v14349[2], v14349[3], v14349[4], 0.0]))) / v4911;
                let v4913 = v1 + v4912;
                let v4915 = if v4913 >= v4914 { 1.0 } else { 0.0 };
                let v4917: f64;
                let v9834: Lanes<6>;
                if v4915 != 0.0 {
                    v4917 = v4913;
                    v9834 = v14352;
                } else {
                    v4917 = v4916;
                    v9834 = v11062;
                }
                let v14354 = v10410 * v1207;
                let v4919 = (v1207 * v663) * v13;
                let v4920 = v4917.sqrt();
                let v4921 = v1 - v4920;
                let v14362 = (((v10836 * v663) + (Lanes([0.0, 0.0, v14354[0], 0.0, 0.0]))) * v13) * v4921;
                let v4923 = v4883 + (v4919 * v4921);
                let v14366 = v14338 + ((Lanes([v14362[0], v14362[1], v14362[2], v14362[3], v14362[4], 0.0])) + (((v9834 * (v9368 / (v10435 * v4920))) * v10390) * v4919));
                let v4926 = if v830 < ((v240 + v4884) * v13) { 1.0 } else { 0.0 };
                if v4926 != 0.0 {
                } else {
                }
                let v5086: f64;
                let v5098: f64;
                let v9835: Lanes<6>;
                if v4927 != 0.0 {
                    let v4930 = if (v663 * (v4923 - v4906)) < v96 { 1.0 } else { 0.0 };
                    let v5091: f64;
                    let v5101: f64;
                    let v9836: Lanes<6>;
                    if v4930 != 0.0 {
                        let v4932 = v4931 * v663;
                        let v4933 = v4932 * v1206;
                        let v14429 = (v10410 * v4931) * v1206;
                        let v4934 = v1 / v4933;
                        let v14435 = ((((Lanes([0.0, 0.0, v14429[0], 0.0, 0.0])) + (v10834 * v4932)) * v4934) * v10390) / v4933;
                        let v14436 = v14435 * v96;
                        let v4936 = v1540 + (v96 * v4934);
                        let v14438 = (v14435 * v1540) * v10390;
                        let v4940 = v1153 * v4934;
                        let v4941 = v4940 * v4908;
                        let v14440 = (v14435 * v1153) * v4908;
                        let v14445 = (Lanes([v14438[0], v14438[1], v14438[2], v14438[3], v14438[4], 0.0])) + ((Lanes([v14440[0], v14440[1], v14440[2], v14440[3], v14440[4], 0.0])) + (v14343 * v4940));
                        let v4946 = (v1549 - (v1540 * (v1550 + v4934))) + v4941;
                        let v14446 = v14445 * v4946;
                        let v4948 = v90 * v4936;
                        let v4949 = v4948 * v4936;
                        let v14454 = ((((v14436 * v90) * v4936) + (v14436 * v4948)) * v4936) + (v14436 * v4949);
                        let v4952 = ((v4949 * v4936) + (v4946 * v4946)).sqrt();
                        let v4953 = ((v4937 - (v1540 * v4934)) + v4941) + v4952;
                        let v4954 = v4953.powf(v1562);
                        let v14464 = (v14445 + (((Lanes([v14454[0], v14454[1], v14454[2], v14454[3], v14454[4], 0.0])) + (v14446 + v14446)) * (v9368 / (v10435 * v4952)))) * (v1562 * (v4953.powf(v14461)));
                        let v14465 = v14436 * v1564;
                        let v4956 = v96 * v4954;
                        let v4957 = (v1564 * v4936) / v4956;
                        let v4961 = (v96 - v4957) + (v4959 * v4954);
                        let v14475 = v10415 * v4961;
                        let v4963 = (v4961 * v665) + v4906;
                        let v14478 = (((((((Lanes([v14465[0], v14465[1], v14465[2], v14465[3], v14465[4], 0.0])) - ((v14464 * v96) * v4957)) / v4956) * v10390) + (v14464 * v4959)) * v665) + (Lanes([0.0, 0.0, v14475[0], 0.0, 0.0, 0.0]))) + v14337;
                        v5091 = v4963;
                        v5101 = v4963;
                        v9836 = v14478;
                    } else {
                        let v4966 = if (v830 - v4964) <= v4884 { 1.0 } else { 0.0 };
                        let v5092: f64;
                        let v5102: f64;
                        let v9837: Lanes<6>;
                        if v4966 != 0.0 {
                            let v4984: f64;
                            let v9838: Lanes<6>;
                            if v9 != 0.0 {
                                let v4967 = v1 / v1128;
                                let v4968 = v12 / v123;
                                let v4969 = v1 / v130;
                                let v4971 = (v4967 + v4968) + v4969;
                                let v4972 = v1 / v4971;
                                let v4976 = v4969 + (v13 * v4968);
                                let v4980 = (v4883 - v4973) + (v4976 * (-v4977));
                                let v14418 = ((((((v9421 * v4967) * v10390) / v1128) * v4972) * v10390) / v4971) * v4980;
                                let v4982 = (v4972 * v4980) / v1128;
                                let v14422 = v9421 * v4982;
                                let v4983 = v4883 - v4982;
                                let v14426 = v10828 - ((((Lanes([v14418[0], v14418[1], 0.0, v14418[2], v14418[3]])) + (((v10828 - (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3]]))) + ((v9455 * v10390) * v4976)) * v4972)) - (Lanes([v14422[0], v14422[1], 0.0, v14422[2], v14422[3]]))) / v1128);
                                let v14427 = Lanes([v14426[0], v14426[1], v14426[2], v14426[3], v14426[4], 0.0]);
                                v4984 = v4983;
                                v9838 = v14427;
                            } else {
                                v4984 = v4923;
                                v9838 = v14366;
                            }
                            v5092 = v4984;
                            v5102 = v4984;
                            v9837 = v9838;
                        } else {
                            let v4985 = v1 / v759;
                            let v14370 = ((v10496 * v4985) * v10390) / v759;
                            let v4986 = v4985 / v1211;
                            let v4987 = v4883 - v4964;
                            let v14375 = v10828 - v9453;
                            let v4988 = v4986 * v4987;
                            let v4989 = v4988 * v4987;
                            let v4990 = v78 / v4987;
                            let v4991 = v663 + v4990;
                            let v4993 = (v4989.ln()) / v4991;
                            let v14391 = (((((((((Lanes([0.0, 0.0, v14370[0], 0.0, 0.0])) - (v9422 * v4986)) / v1211) * v4987) + (v14375 * v4986)) * v4987) + (v14375 * v4988)) * (v9368 / v4989)) - (((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + (((v14375 * v4990) * v10390) / v4987)) * v4993)) / v4991;
                            let v4995 = v4993 + v4994;
                            let v14392 = Lanes([v14391[0], v14391[1], v14391[2], v14391[3], v14391[4], 0.0]);
                            let v14393 = v14392 - v14366;
                            let v4997 = (v4995 - v4923) - v1270;
                            let v4999 = (v90 * v4995) * v1270;
                            let v14395 = (v14391 * v90) * v1270;
                            let v5000 = if v4999 > v0 { 1.0 } else { 0.0 };
                            let v5002: f64;
                            let v9839: Lanes<5>;
                            if v5000 != 0.0 {
                                v5002 = v4999;
                                v9839 = v14395;
                            } else {
                                let v5001 = -v4999;
                                let v14396 = v14395 * v10390;
                                v5002 = v5001;
                                v9839 = v14396;
                            }
                            let v14397 = v14393 * v4997;
                            let v5005 = ((v4997 * v4997) + v5002).sqrt();
                            let v5008 = v4995 - (v13 * (v4997 + v5005));
                            let v14406 = v14392 - ((v14393 + (((v14397 + v14397) + (Lanes([v9839[0], v9839[1], v9839[2], v9839[3], v9839[4], 0.0]))) * (v9368 / (v10435 * v5005)))) * v13);
                            v5092 = v5008;
                            v5102 = v4923;
                            v9837 = v14406;
                        }
                        v5091 = v5092;
                        v5101 = v5102;
                        v9836 = v9837;
                    }
                    let v5087: f64;
                    let v5099: f64;
                    let v9840: Lanes<6>;
                    if v9 != 0.0 {
                        let v5010 = if (v830 - v4964) <= v4884 { 1.0 } else { 0.0 };
                        let v5088: f64;
                        let v5100: f64;
                        let v9841: Lanes<5>;
                        if v5010 != 0.0 {
                            let v5011 = v1 / v1128;
                            let v5012 = v12 / v123;
                            let v5013 = v1 / v130;
                            let v5015 = (v5011 + v5012) + v5013;
                            let v5016 = v1 / v5015;
                            let v5019 = v5013 + (v13 * v5012);
                            let v5022 = (v4883 - v4973) + (v5019 * (-v4977));
                            let v14555 = ((((((v9421 * v5011) * v10390) / v1128) * v5016) * v10390) / v5015) * v5022;
                            let v5024 = (v5016 * v5022) / v1128;
                            let v14559 = v9421 * v5024;
                            let v5025 = v4883 - v5024;
                            let v14563 = v10828 - ((((Lanes([v14555[0], v14555[1], 0.0, v14555[2], v14555[3]])) + (((v10828 - (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3]]))) + ((v9455 * v10390) * v5019)) * v5016)) - (Lanes([v14559[0], v14559[1], 0.0, v14559[2], v14559[3]]))) / v1128);
                            v5088 = v5025;
                            v5100 = v5025;
                            v9841 = v14563;
                        } else {
                            let v5026 = v1 / v1128;
                            let v5027 = v12 / v123;
                            let v5028 = v1 / v130;
                            let v5030 = (v5026 + v5027) + v5028;
                            let v5031 = v1 / v5030;
                            let v5034 = v5028 + (v13 * v5027);
                            let v5037 = (v4883 - v4973) + (v5034 * (-v4977));
                            let v14490 = ((((((v9421 * v5026) * v10390) / v1128) * v5031) * v10390) / v5030) * v5037;
                            let v5039 = (v5031 * v5037) / v1128;
                            let v14494 = v9421 * v5039;
                            let v5040 = v4883 - v5039;
                            let v14498 = v10828 - ((((Lanes([v14490[0], v14490[1], 0.0, v14490[2], v14490[3]])) + (((v10828 - (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3]]))) + ((v9455 * v10390) * v5034)) * v5031)) - (Lanes([v14494[0], v14494[1], 0.0, v14494[2], v14494[3]]))) / v1128);
                            let v5041 = v4883 - v4964;
                            let v14499 = v10828 - v9453;
                            let v5042 = if v5041 > v0 { 1.0 } else { 0.0 };
                            let v5089: f64;
                            let v9842: Lanes<5>;
                            if v5042 != 0.0 {
                                let v5043 = v1 / v759;
                                let v14502 = ((v10496 * v5043) * v10390) / v759;
                                let v5044 = v5043 / v1211;
                                let v5045 = v5044 * v5041;
                                let v5046 = v5045 * v5041;
                                let v5047 = v78 / v5041;
                                let v5048 = v663 + v5047;
                                let v5050 = (v5046.ln()) / v5048;
                                let v5052 = (v5050 + v4994) * v1661;
                                let v14523 = ((((((((((Lanes([0.0, 0.0, v14502[0], 0.0, 0.0])) - (v9422 * v5044)) / v1211) * v5041) + (v14499 * v5044)) * v5041) + (v14499 * v5045)) * (v9368 / v5046)) - (((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + (((v14499 * v5047) * v10390) / v5041)) * v5050)) / v5048) * v1661;
                                let v5053 = v5052 - v708;
                                let v5056 = if (if v5040 > v5053 { 1.0 } else { 0.0 }) != 0.0 && v5055 != 0.0 { 1.0 } else { 0.0 };
                                let v5090: f64;
                                let v9843: Lanes<5>;
                                if v5056 != 0.0 {
                                    let v14524 = v14498 - v14523;
                                    let v5058 = (v5040 - v5052) + v708;
                                    let v5059 = v5058 * v5058;
                                    let v14525 = v14524 * v5058;
                                    let v14527 = (v14525 + v14525) * v5059;
                                    let v14528 = v14527 + v14527;
                                    let v5062 = (v5059 * v5059) + v5061;
                                    let v5079: f64;
                                    let v9844: Lanes<5>;
                                    if v5063 != 0.0 {
                                        let v5073: f64;
                                        if v5064 != 0.0 {
                                            v5073 = v1;
                                        } else {
                                            let v5074: f64;
                                            if v5065 != 0.0 {
                                                v5074 = v78;
                                            } else {
                                                let v5075: f64;
                                                if v5066 != 0.0 {
                                                    v5075 = v96;
                                                } else {
                                                    let v5076: f64;
                                                    if v5067 != 0.0 {
                                                        v5076 = v90;
                                                    } else {
                                                        v5076 = v0;
                                                    }
                                                    v5075 = v5076;
                                                }
                                                v5074 = v5075;
                                            }
                                            v5073 = v5074;
                                        }
                                        let mut v5068: f64 = 0.0;
                                        let mut v5070: f64 = 0.0;
                                        let mut v9845: Lanes<5> = Lanes([0.0; 5]);
                                        v5068 = v0;
                                        v5070 = v5062;
                                        v9845 = v14528;
                                        loop {
                                            let v5069 = if v5068 < v5073 { 1.0 } else { 0.0 };
                                            if v5069 == 0.0 {
                                                break;
                                            }
                                            let v5071 = v5070.sqrt();
                                            let v14543 = v9845 * (v9368 / (v10435 * v5071));
                                            let v5072 = v5068 + v1;
                                            v5068 = v5072;
                                            v5070 = v5071;
                                            v9845 = v14543;
                                        }
                                        v5079 = v5070;
                                        v9844 = v9845;
                                    } else {
                                        let v5078 = v5062.powf(v5077);
                                        let v14532 = v14528 * (v5077 * (v5062.powf(v14529)));
                                        v5079 = v5078;
                                        v9844 = v14532;
                                    }
                                    let v5080 = v1 / v5079;
                                    let v5081 = v5058 * v708;
                                    let v5083 = v5053 + (v5081 * v5080);
                                    let v14540 = v14523 + (((v14524 * v708) * v5080) + ((((v9844 * v5080) * v10390) / v5079) * v5081));
                                    v5090 = v5083;
                                    v9843 = v14540;
                                } else {
                                    v5090 = v5040;
                                    v9843 = v14498;
                                }
                                v5089 = v5090;
                                v9842 = v9843;
                            } else {
                                v5089 = v5040;
                                v9842 = v14498;
                            }
                            v5088 = v5089;
                            v5100 = v5040;
                            v9841 = v9842;
                        }
                        let v14564 = Lanes([v9841[0], v9841[1], v9841[2], v9841[3], v9841[4], 0.0]);
                        v5087 = v5088;
                        v5099 = v5100;
                        v9840 = v14564;
                    } else {
                        v5087 = v5091;
                        v5099 = v5101;
                        v9840 = v9836;
                    }
                    v5086 = v5087;
                    v5098 = v5099;
                    v9835 = v9840;
                } else {
                    let v14367 = Lanes([v9456[0], v9456[1], v9456[2], v9456[3], v9456[4], 0.0]);
                    v5086 = v5093;
                    v5098 = v4923;
                    v9835 = v14367;
                }
                let v5085 = v4906 + v5084;
                let v5096 = if v5086 < v5085 { 1.0 } else { 0.0 };
                let v5097: f64;
                let v9846: Lanes<6>;
                if v5096 != 0.0 {
                    v5097 = v5085;
                    v9846 = v14337;
                } else {
                    v5097 = v5086;
                    v9846 = v9835;
                }
                if v0 != 0.0 {
                    let v5103 = v5098 - v5097;
                    let v5104 = if v5103 >= v0 { 1.0 } else { 0.0 };
                    let v5105: f64;
                    if v5104 != 0.0 {
                        v5105 = v5103;
                    } else {
                        v5105 = v0;
                    }
                    let v5109 = ((v5106 * v5105) - v4994) - v1985;
                    let v5113 = (v90 * (v5110 * v5105)) * v1985;
                    let v5114 = if v5113 > v0 { 1.0 } else { 0.0 };
                    let v5116: f64;
                    if v5114 != 0.0 {
                        v5116 = v5113;
                    } else {
                        let v5115 = -v5113;
                        v5116 = v5115;
                    }
                    let v5124 = (v5120 * v5105) - (v13 * (v5109 + (((v5109 * v5109) + v5116).sqrt())));
                    let v5125 = if v5124 <= v5105 { 1.0 } else { 0.0 };
                    let v5126: f64;
                    if v5125 != 0.0 {
                        v5126 = v5124;
                    } else {
                        v5126 = v5105;
                    }
                    let v5127 = if v5126 < v0 { 1.0 } else { 0.0 };
                    if v5127 != 0.0 {
                    } else {
                        let v5128 = if v5126 > v823 { 1.0 } else { 0.0 };
                        if v5128 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v5130 = if v5129 == v1 { 1.0 } else { 0.0 };
                let v5365: f64;
                let v9847: Lanes<6>;
                if v5130 != 0.0 {
                    let v5133 = if v830 < ((v1205 + v4906) + v4882) { 1.0 } else { 0.0 };
                    let v5366: f64;
                    let v9848: Lanes<6>;
                    if v5133 != 0.0 {
                        let v5134 = v78 * v665;
                        let v5136 = (-v367) / v1206;
                        let v5137 = v5136.ln();
                        let v5138 = v5134 * v5137;
                        let v14796 = (v10415 * v78) * v5137;
                        let v14799 = (Lanes([0.0, 0.0, v14796[0], 0.0, 0.0])) + (((((v10834 * v5136) * v10390) / v1206) * (v9368 / v5136)) * v5134);
                        let v5139 = v663 * v750;
                        let v5140 = v1 / v5139;
                        let v5141 = v5140 * v1128;
                        let v14806 = (((((v10410 * v750) + (v10485 * v663)) * v5140) * v10390) / v5139) * v1128;
                        let v14807 = v9421 * v5140;
                        let v14810 = (Lanes([0.0, 0.0, v14806[0], 0.0, 0.0])) + (Lanes([v14807[0], v14807[1], 0.0, v14807[2], v14807[3]]));
                        let v14811 = v14810 * v5142;
                        let v5144 = v78 + (v5142 * v5141);
                        let v5145 = v91 * v5144;
                        let v5146 = v5145 * v5144;
                        let v5147 = v5146 * v5144;
                        let v14818 = ((((v14811 * v91) * v5144) + (v14811 * v5145)) * v5144) + (v14811 * v5146);
                        let v5148 = v4908 - v78;
                        let v5149 = v3500 * v5141;
                        let v5150 = v5149 * v5148;
                        let v14820 = (v14810 * v3500) * v5148;
                        let v14823 = (Lanes([v14820[0], v14820[1], v14820[2], v14820[3], v14820[4], 0.0])) + (v14343 * v5149);
                        let v5152 = v5151 - v5150;
                        let v14824 = v14823 * v10390;
                        let v5153 = v5152 * v5152;
                        let v14825 = v14824 * v5152;
                        let v14826 = v14825 + v14825;
                        let v5155 = if v5147 < (v5153 * v3506) { 1.0 } else { 0.0 };
                        let v5167: f64;
                        let v9849: Lanes<6>;
                        if v5155 != 0.0 {
                            let v14833 = v14818 * v13;
                            let v5159 = (v13 * v5147) / v5152;
                            let v5161 = ((v5156 + v5152) + v5159) + v5150;
                            let v14839 = (v14824 + (((Lanes([v14833[0], v14833[1], v14833[2], v14833[3], v14833[4], 0.0])) - (v14824 * v5159)) / v5152)) + v14823;
                            v5167 = v5161;
                            v9849 = v14839;
                        } else {
                            let v5163 = (v5147 + v5153).sqrt();
                            let v5166 = (v5164 + v5163) + v5150;
                            let v14832 = (((Lanes([v14818[0], v14818[1], v14818[2], v14818[3], v14818[4], 0.0])) + v14826) * (v9368 / (v10435 * v5163))) + v14823;
                            v5167 = v5166;
                            v9849 = v14832;
                        }
                        let v5168 = v5167.powf(v1562);
                        let v14843 = v9849 * (v1562 * (v5167.powf(v14840)));
                        let v14845 = (v14810 * v3523) * v10390;
                        let v5174 = v748 * v5168;
                        let v5176 = ((v5169 - (v3523 * v5141)) + (v78 * v5168)) + (v5174 * v5168);
                        let v5177 = v1 / v5168;
                        let v5178 = v5176 * v5177;
                        let v14861 = v10415 * v5178;
                        let v5181 = ((v5178 * v665) + v4906) - v4906;
                        let v14865 = ((((((((Lanes([v14845[0], v14845[1], v14845[2], v14845[3], v14845[4], 0.0])) + (v14843 * v78)) + (((v14843 * v748) * v5168) + (v14843 * v5174))) * v5177) + ((((v14843 * v5177) * v10390) / v5168) * v5176)) * v665) + (Lanes([0.0, 0.0, v14861[0], 0.0, 0.0, 0.0]))) + v14337) - v14337;
                        let v5182 = v5181 / v5138;
                        let v14866 = v14799 * v5182;
                        let v14870 = ((v14865 - (Lanes([v14866[0], v14866[1], v14866[2], v14866[3], v14866[4], 0.0]))) / v5138) * v5182;
                        let v5185 = (v1 + (v5182 * v5182)).sqrt();
                        let v5186 = v5181 / v5185;
                        let v5187 = v5186 + v4906;
                        let v14878 = ((v14865 - (((v14870 + v14870) * (v9368 / (v10435 * v5185))) * v5186)) / v5185) + v14337;
                        v5366 = v5187;
                        v9848 = v14878;
                    } else {
                        let v5188 = v4906 - v4994;
                        let v14565 = v10410 * v5188;
                        let v5190 = (v663 * v5188).exp();
                        let v14569 = ((Lanes([0.0, 0.0, v14565[0], 0.0, 0.0, 0.0])) + (v14337 * v663)) * v5190;
                        let v5194 = (((v490 * v12) * v12) / v78) / v123;
                        let v5197 = ((v78 * v663) * v5194).sqrt();
                        let v14574 = ((v10410 * v78) * v5194) * (v9368 / (v10435 * v5197));
                        let v5198 = v5197.exp();
                        let v5200 = (-v5197).exp();
                        let v5202 = (v5198 + v5200) / v78;
                        let v5204 = (v5202.ln()) / v5194;
                        let v14582 = ((((v14574 * v5198) + ((v14574 * v10390) * v5200)) / v78) * (v9368 / v5202)) / v5194;
                        let mut v5205: f64 = 0.0;
                        let mut v5208: f64 = 0.0;
                        let mut v5296: f64 = 0.0;
                        let mut v9850: Lanes<6> = Lanes([0.0; 6]);
                        v5205 = v1;
                        v5208 = v5097;
                        v5296 = v0;
                        v9850 = v9846;
                        loop {
                            let v5207 = if v5205 <= v5206 { 1.0 } else { 0.0 };
                            if v5207 == 0.0 {
                                break;
                            }
                            let v5209 = v5208 - v4906;
                            let v14583 = v9850 - v14337;
                            let v5210 = v663 * v5209;
                            let v14584 = v10410 * v5209;
                            let v14587 = (Lanes([0.0, 0.0, v14584[0], 0.0, 0.0, 0.0])) + (v14583 * v663);
                            let v5211 = v5209 - v5194;
                            let v5212 = v5204 * v5211;
                            let v14588 = v14582 * v5211;
                            let v14591 = (Lanes([0.0, 0.0, v14588[0], 0.0, 0.0, 0.0])) + (v14583 * v5204);
                            let v5213 = if v5212 < v2535 { 1.0 } else { 0.0 };
                            let v5223: f64;
                            let v5227: f64;
                            let v9851: Lanes<6>;
                            let v9852: Lanes<6>;
                            if v5213 != 0.0 {
                                let v5214 = v5212.exp();
                                let v14592 = v14591 * v5214;
                                let v5217 = ((-v5204) * v5194).exp();
                                let v14595 = ((v14582 * v10390) * v5194) * v5217;
                                let v14597 = v14592 - (Lanes([0.0, 0.0, v14595[0], 0.0, 0.0, 0.0]));
                                let v5219 = v1 + (v5214 - v5217);
                                let v5221 = (v5219.ln()) / v5204;
                                let v14600 = v14582 * v5221;
                                let v14603 = ((v14597 * (v9368 / v5219)) - (Lanes([0.0, 0.0, v14600[0], 0.0, 0.0, 0.0]))) / v5204;
                                let v5222 = v5214 / v5219;
                                let v14606 = (v14592 - (v14597 * v5222)) / v5219;
                                v5223 = v5221;
                                v5227 = v5222;
                                v9851 = v14603;
                                v9852 = v14606;
                            } else {
                                v5223 = v5211;
                                v5227 = v1;
                                v9851 = v14583;
                                v9852 = v11062;
                            }
                            let v5224 = v663 * v5223;
                            let v14607 = v10410 * v5223;
                            let v14610 = (Lanes([0.0, 0.0, v14607[0], 0.0, 0.0, 0.0])) + (v9851 * v663);
                            let v5225 = v5210.abs();
                            let v5226 = if v5225 < v3672 { 1.0 } else { 0.0 };
                            let v5300: f64;
                            let v5304: f64;
                            let v9853: Lanes<6>;
                            let v9854: Lanes<6>;
                            if v5226 != 0.0 {
                                let v14713 = v9852 * v5227;
                                let v5231 = ((v1 - (v5227 * v5227)) / v78).sqrt();
                                let v14719 = (((v14713 + v14713) * v10390) / v78) * (v9368 / (v10435 * v5231));
                                let v5232 = v5210 * v5231;
                                let v14722 = (v14587 * v5231) + (v14719 * v5210);
                                let v5233 = v663 * v5231;
                                let v14723 = v10410 * v5231;
                                let v14726 = (Lanes([0.0, 0.0, v14723[0], 0.0, 0.0, 0.0])) + (v14719 * v663);
                                let v5234 = if v5210 < v0 { 1.0 } else { 0.0 };
                                let v5301: f64;
                                let v5305: f64;
                                let v9855: Lanes<6>;
                                let v9856: Lanes<6>;
                                if v5234 != 0.0 {
                                    let v5235 = -v5232;
                                    let v14727 = v14722 * v10390;
                                    let v5236 = -v5233;
                                    let v14728 = v14726 * v10390;
                                    v5301 = v5235;
                                    v5305 = v5236;
                                    v9855 = v14727;
                                    v9856 = v14728;
                                } else {
                                    v5301 = v5232;
                                    v5305 = v5233;
                                    v9855 = v14722;
                                    v9856 = v14726;
                                }
                                v5300 = v5301;
                                v5304 = v5305;
                                v9853 = v9855;
                                v9854 = v9856;
                            } else {
                                let v5237 = if v5225 < v3684 { 1.0 } else { 0.0 };
                                let v5302: f64;
                                let v5306: f64;
                                let v9857: Lanes<6>;
                                let v9858: Lanes<6>;
                                if v5237 != 0.0 {
                                    let v14635 = v14587 * v5210;
                                    let v5239 = (v5210 * v5210) / v78;
                                    let v5240 = v5210 / v96;
                                    let v14638 = v14587 / v96;
                                    let v5241 = v5210 / v90;
                                    let v14639 = v14587 / v90;
                                    let v5243 = v1 - (v5210 / v644);
                                    let v5245 = v1 - (v5241 * v5243);
                                    let v5247 = v1 - (v5240 * v5245);
                                    let v5249 = v5210 / v78;
                                    let v5250 = v1 - v5241;
                                    let v5252 = v1 - (v5240 * v5250);
                                    let v5254 = v1 - (v5249 * v5252);
                                    let v14666 = v14610 * v5224;
                                    let v5257 = (v5224 * v5224) / v78;
                                    let v5258 = v5224 / v96;
                                    let v14669 = v14610 / v96;
                                    let v5259 = v5224 / v90;
                                    let v14670 = v14610 / v90;
                                    let v5261 = v1 - (v5224 / v644);
                                    let v5263 = v1 - (v5259 * v5261);
                                    let v5265 = v1 - (v5258 * v5263);
                                    let v5267 = v5224 / v78;
                                    let v5268 = v1 - v5259;
                                    let v5270 = v1 - (v5258 * v5268);
                                    let v5272 = v1 - (v5267 * v5270);
                                    let v5273 = v5224 * v5272;
                                    let v5275 = ((v5239 * v5247) - (v5257 * v5265)).sqrt();
                                    let v14700 = (((((v14635 + v14635) / v78) * v5247) + ((((v14638 * v5245) + ((((v14639 * v5243) + (((v14587 / v644) * v10390) * v5241)) * v10390) * v5240)) * v10390) * v5239)) - ((((v14666 + v14666) / v78) * v5265) + ((((v14669 * v5263) + ((((v14670 * v5261) + (((v14610 / v644) * v10390) * v5259)) * v10390) * v5258)) * v10390) * v5257))) * (v9368 / (v10435 * v5275));
                                    let v5276 = v663 * v13;
                                    let v5278 = (v5210 * v5254) - (v5227 * v5273);
                                    let v14706 = (v10410 * v13) * v5278;
                                    let v5280 = (v5276 * v5278) / v5275;
                                    let v14712 = (((Lanes([0.0, 0.0, v14706[0], 0.0, 0.0, 0.0])) + ((((v14587 * v5254) + (((((v14587 / v78) * v5252) + ((((v14638 * v5250) + ((v14639 * v10390) * v5240)) * v10390) * v5249)) * v10390) * v5210)) - ((v9852 * v5273) + (((v14610 * v5272) + (((((v14610 / v78) * v5270) + ((((v14669 * v5268) + ((v14670 * v10390) * v5258)) * v10390) * v5267)) * v10390) * v5224)) * v5227))) * v5276)) - (v14700 * v5280)) / v5275;
                                    v5302 = v5275;
                                    v5306 = v5280;
                                    v9857 = v14700;
                                    v9858 = v14712;
                                } else {
                                    let v5282 = (-v5210).exp();
                                    let v14612 = (v14587 * v10390) * v5282;
                                    let v5284 = (-v5224).exp();
                                    let v14614 = (v14610 * v10390) * v5284;
                                    let v5288 = ((v5210 - v5224) + (v5282 - v5284)).sqrt();
                                    let v14620 = ((v14587 - v14610) + (v14612 - v14614)) * (v9368 / (v10435 * v5288));
                                    let v5289 = v663 * v13;
                                    let v5291 = v1 - v5284;
                                    let v5293 = (v1 - v5282) - (v5227 * v5291);
                                    let v14628 = (v10410 * v13) * v5293;
                                    let v5295 = (v5289 * v5293) / v5288;
                                    let v14634 = (((Lanes([0.0, 0.0, v14628[0], 0.0, 0.0, 0.0])) + (((v14612 * v10390) - ((v9852 * v5291) + ((v14614 * v10390) * v5227))) * v5289)) - (v14620 * v5295)) / v5288;
                                    v5302 = v5288;
                                    v5306 = v5295;
                                    v9857 = v14620;
                                    v9858 = v14634;
                                }
                                v5300 = v5302;
                                v5304 = v5306;
                                v9853 = v9857;
                                v9854 = v9858;
                            }
                            let v5297 = if v5296 == v1 { 1.0 } else { 0.0 };
                            let v5298 = if v5210 < v0 { 1.0 } else { 0.0 };
                            let v5299 = if v5297 != 0.0 && v5298 != 0.0 { 1.0 } else { 0.0 };
                            if v5299 != 0.0 {
                            } else {
                            }
                            let v5329: f64;
                            let v5333: f64;
                            let v9859: Lanes<6>;
                            let v9860: Lanes<6>;
                            if v5298 != 0.0 {
                                let v5303 = -v5300;
                                let v14765 = v9853 * v10390;
                                let v5307 = -v5304;
                                let v14766 = v9854 * v10390;
                                v5329 = v5303;
                                v5333 = v5307;
                                v9859 = v14765;
                                v9860 = v14766;
                            } else {
                                let v5308 = if v5210 < v117 { 1.0 } else { 0.0 };
                                let v5330: f64;
                                let v5334: f64;
                                let v9861: Lanes<6>;
                                let v9862: Lanes<6>;
                                if v5308 != 0.0 {
                                    v5330 = v5300;
                                    v5334 = v5304;
                                    v9861 = v9853;
                                    v9862 = v9854;
                                } else {
                                    let v5309 = v5208 - v4994;
                                    let v14729 = v10410 * v5309;
                                    let v5311 = (v663 * v5309).exp();
                                    let v14733 = ((Lanes([0.0, 0.0, v14729[0], 0.0, 0.0, 0.0])) + (v9850 * v663)) * v5311;
                                    let v5312 = v5210 + v1;
                                    let v5314 = v5311 - (v5190 * v5312);
                                    let v14738 = v10496 * v5314;
                                    let v5316 = v759 * v663;
                                    let v5317 = v5311 - v5190;
                                    let v14746 = ((v10496 * v663) + (v10410 * v759)) * v5317;
                                    let v14750 = v9853 * v5300;
                                    let v5321 = ((v5300 * v5300) + (v759 * v5314)).sqrt();
                                    let v14755 = ((v14750 + v14750) + ((Lanes([0.0, 0.0, v14738[0], 0.0, 0.0, 0.0])) + ((v14733 - ((v14569 * v5312) + (v14587 * v5190))) * v759))) * (v9368 / (v10435 * v5321));
                                    let v5322 = v78 * v5304;
                                    let v5326 = (v13 * ((v5322 * v5300) + (v5316 * v5317))) / v5321;
                                    let v14764 = ((((((v9854 * v78) * v5300) + (v9853 * v5322)) + ((Lanes([0.0, 0.0, v14746[0], 0.0, 0.0, 0.0])) + ((v14733 - v14569) * v5316))) * v13) - (v14755 * v5326)) / v5321;
                                    v5330 = v5321;
                                    v5334 = v5326;
                                    v9861 = v14755;
                                    v9862 = v14764;
                                }
                                v5329 = v5330;
                                v5333 = v5334;
                                v9859 = v9861;
                                v9860 = v9862;
                            }
                            let v14767 = v10828 * v10390;
                            let v14770 = v10834 * v5329;
                            let v5332 = ((-v4883) + v5208) + (v1206 * v5329);
                            let v14774 = ((Lanes([v14767[0], v14767[1], v14767[2], v14767[3], v14767[4], 0.0])) + v9850) + ((Lanes([v14770[0], v14770[1], v14770[2], v14770[3], v14770[4], 0.0])) + (v9859 * v1206));
                            let v14775 = v10834 * v5333;
                            let v14778 = (Lanes([v14775[0], v14775[1], v14775[2], v14775[3], v14775[4], 0.0])) + (v9860 * v1206);
                            let v5336 = v1 + (v1206 * v5333);
                            let v5359: f64;
                            let v5361: f64;
                            let v5362: f64;
                            let v9863: Lanes<6>;
                            if v5297 != 0.0 {
                                v5359 = v5337;
                                v5361 = v5208;
                                v5362 = v5296;
                                v9863 = v9850;
                            } else {
                                let v5339 = (-v5332) / v5336;
                                let v14782 = ((v14774 * v10390) - (v14778 * v5339)) / v5336;
                                let v5341 = v5208.abs();
                                let v14786 = v9850 * ((v10435 * (if v5208 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                                let v5342 = if v1 >= v5341 { 1.0 } else { 0.0 };
                                let v5343: f64;
                                let v9864: Lanes<6>;
                                if v5342 != 0.0 {
                                    v5343 = v1;
                                    v9864 = v11062;
                                } else {
                                    v5343 = v5341;
                                    v9864 = v14786;
                                }
                                let v5345 = v5340 * (v1 + v5343);
                                let v14787 = v9864 * v5340;
                                let v5347 = if (v5339.abs()) > v5345 { 1.0 } else { 0.0 };
                                let v5352: f64;
                                let v9865: Lanes<6>;
                                if v5347 != 0.0 {
                                    let v5348 = if v5339 >= v0 { 1.0 } else { 0.0 };
                                    let v5350: f64;
                                    if v5348 != 0.0 {
                                        v5350 = v1;
                                    } else {
                                        v5350 = v5349;
                                    }
                                    let v5351 = v5345 * v5350;
                                    let v14788 = v14787 * v5350;
                                    v5352 = v5351;
                                    v9865 = v14788;
                                } else {
                                    v5352 = v5339;
                                    v9865 = v14782;
                                }
                                let v5353 = v5208 + v5352;
                                let v14789 = v9850 + v9865;
                                let v5358 = if (if (v5352.abs()) <= v861 { 1.0 } else { 0.0 }) != 0.0 && (if (v5332.abs()) <= v3506 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v5363: f64;
                                if v5358 != 0.0 {
                                    v5363 = v1;
                                } else {
                                    v5363 = v5296;
                                }
                                v5359 = v5205;
                                v5361 = v5353;
                                v5362 = v5363;
                                v9863 = v14789;
                            }
                            let v5360 = v5359 + v1;
                            v5205 = v5360;
                            v5208 = v5361;
                            v5296 = v5362;
                            v9850 = v9863;
                        }
                        v5366 = v5208;
                        v9848 = v9850;
                    }
                    v5365 = v5366;
                    v9847 = v9848;
                } else {
                    v5365 = v5097;
                    v9847 = v9846;
                }
                let v5364 = -v663;
                let v5367 = v5365 - v4906;
                let v14880 = v9847 - v14337;
                let v5368 = v5364 * v5367;
                let v14881 = (v10410 * v10390) * v5367;
                let v14884 = (Lanes([0.0, 0.0, v14881[0], 0.0, 0.0, 0.0])) + (v14880 * v5364);
                let v5369 = if v5368 >= v0 { 1.0 } else { 0.0 };
                let v5371: f64;
                if v5369 != 0.0 {
                    v5371 = v1;
                } else {
                    v5371 = v5370;
                }
                let v5372 = v5371 * v5368;
                let v14885 = v14884 * v5371;
                let v5373 = v5368.exp();
                let v5375 = (v5373 - v1) - v5368;
                let v14887 = (v14884 * v5373) - v14884;
                let v5376 = if v5368 > v117 { 1.0 } else { 0.0 };
                let v5394: f64;
                let v9866: Lanes<6>;
                if v5376 != 0.0 {
                    let v5377 = -v750;
                    let v5378 = v5375.sqrt();
                    let v5379 = v5377 * v5378;
                    let v14912 = (v10485 * v10390) * v5378;
                    let v14915 = (Lanes([0.0, 0.0, v14912[0], 0.0, 0.0, 0.0])) + ((v14887 * (v9368 / (v10435 * v5378))) * v5377);
                    v5394 = v5379;
                    v9866 = v14915;
                } else {
                    let v5380 = if v5372 > v117 { 1.0 } else { 0.0 };
                    let v5395: f64;
                    let v9867: Lanes<6>;
                    if v5380 != 0.0 {
                        let v5381 = v5375.sqrt();
                        let v5382 = v750 * v5381;
                        let v14904 = v10485 * v5381;
                        let v14907 = (Lanes([0.0, 0.0, v14904[0], 0.0, 0.0, 0.0])) + ((v14887 * (v9368 / (v10435 * v5381))) * v750);
                        v5395 = v5382;
                        v9867 = v14907;
                    } else {
                        let v5383 = -v5371;
                        let v5386 = (v5383 * v5372) * v5385;
                        let v5387 = v5372 * v1562;
                        let v5389 = v1 + (v2050 * v5372);
                        let v5392 = (v1 + (v5387 * v5389)).sqrt();
                        let v5393 = v5386 * v5392;
                        let v14900 = (((v14885 * v5383) * v5385) * v5392) + (((((v14885 * v1562) * v5389) + ((v14885 * v2050) * v5387)) * (v9368 / (v10435 * v5392))) * v5386);
                        v5395 = v5393;
                        v9867 = v14900;
                    }
                    v5394 = v5395;
                    v9866 = v9867;
                }
                let v14916 = v9866 * v5394;
                let v5399 = ((v5394 * v5394) + v5397).sqrt();
                let v14922 = (v9866 + ((v14916 + v14916) * (v9368 / (v10435 * v5399)))) * v13;
                let v5403 = (v13 * (v5394 + v5399)) + v5402;
                let v5404 = if v5403 < v0 { 1.0 } else { 0.0 };
                let v5405: f64;
                let v9868: Lanes<6>;
                if v5404 != 0.0 {
                    v5405 = v0;
                    v9868 = v11062;
                } else {
                    v5405 = v5403;
                    v9868 = v14922;
                }
                let v5406 = v5405 / v490;
                let v14923 = v9868 / v490;
                let v5407 = v5406 - v4885;
                let v5408 = v5406 * v20;
                let v14924 = v14923 * v20;
                let v14925 = v14923 * v5407;
                let v5410 = v90 * v5408;
                let v5413 = ((v5407 * v5407) + (v5410 * v5408)).sqrt();
                let v5417 = (v13 * (v5407 + v5413)) + (v535 * v5408);
                let v14938 = ((v14923 + (((v14925 + v14925) + (((v14924 * v90) * v5408) + (v14924 * v5410))) * (v9368 / (v10435 * v5413)))) * v13) + (v14924 * v535);
                let v5418 = if v5417 < v0 { 1.0 } else { 0.0 };
                let v5419: f64;
                let v9869: Lanes<6>;
                if v5418 != 0.0 {
                    v5419 = v0;
                    v9869 = v11062;
                } else {
                    v5419 = v5417;
                    v9869 = v14938;
                }
                let v5420 = v5419 / v5406;
                let v5422 = (v5420 * v5419) / v5406;
                let v5424 = (v5367 * v5422) + v4906;
                let v14951 = ((v14880 * v5422) + (((((((v9869 - (v14923 * v5420)) / v5406) * v5419) + (v9869 * v5420)) - (v14923 * v5422)) / v5406) * v5367)) + v14337;
                let v14952 = v10410 * v5424;
                let v5426 = (v663 * v5424).exp();
                let v5427 = v5424 - v823;
                let v14958 = v10410 * v5427;
                let v5429 = (v663 * v5427).exp();
                let v5430 = v5426 - v5429;
                let v14963 = (((Lanes([0.0, 0.0, v14952[0], 0.0, 0.0, 0.0])) + (v14951 * v663)) * v5426) - (((Lanes([0.0, 0.0, v14958[0], 0.0, 0.0, 0.0])) + ((v14951 - v14333) * v663)) * v5429);
                let v5434 = ((v5431 * v41) * v123).sqrt();
                let v5435 = v5434 * v732;
                let v14964 = v10456 * v5434;
                let v5436 = v5424 - v4906;
                let v5437 = v663 * v5436;
                let v14966 = v10410 * v5436;
                let v14969 = (Lanes([0.0, 0.0, v14966[0], 0.0, 0.0, 0.0])) + ((v14951 - v14337) * v663);
                let v5438 = v1889 * v663;
                let v14970 = v10410 * v1889;
                let v5441 = if (if v5437 < v5438 { 1.0 } else { 0.0 }) != 0.0 && (if v5438 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5467: f64;
                let v9870: Lanes<6>;
                if v5441 != 0.0 {
                    let v5442 = v5438 - v5437;
                    let v14971 = Lanes([0.0, 0.0, v14970[0], 0.0, 0.0, 0.0]);
                    let v14972 = v14971 - v14969;
                    let v14973 = v14972 * v5442;
                    let v14975 = v14970 * v5438;
                    let v14976 = v14975 + v14975;
                    let v5445 = (v5442 * v5442) + (v5438 * v5438);
                    let v14978 = (v14973 + v14973) + (Lanes([0.0, 0.0, v14976[0], 0.0, 0.0, 0.0]));
                    let v5462: f64;
                    let v9871: Lanes<6>;
                    if v5446 != 0.0 {
                        let v5456: f64;
                        if v5447 != 0.0 {
                            v5456 = v1;
                        } else {
                            let v5457: f64;
                            if v5448 != 0.0 {
                                v5457 = v78;
                            } else {
                                let v5458: f64;
                                if v5449 != 0.0 {
                                    v5458 = v96;
                                } else {
                                    let v5459: f64;
                                    if v5450 != 0.0 {
                                        v5459 = v90;
                                    } else {
                                        v5459 = v0;
                                    }
                                    v5458 = v5459;
                                }
                                v5457 = v5458;
                            }
                            v5456 = v5457;
                        }
                        let mut v5451: f64 = 0.0;
                        let mut v5453: f64 = 0.0;
                        let mut v9872: Lanes<6> = Lanes([0.0; 6]);
                        v5451 = v0;
                        v5453 = v5445;
                        v9872 = v14978;
                        loop {
                            let v5452 = if v5451 < v5456 { 1.0 } else { 0.0 };
                            if v5452 == 0.0 {
                                break;
                            }
                            let v5454 = v5453.sqrt();
                            let v18858 = v9872 * (v9368 / (v10435 * v5454));
                            let v5455 = v5451 + v1;
                            v5451 = v5455;
                            v5453 = v5454;
                            v9872 = v18858;
                        }
                        v5462 = v5453;
                        v9871 = v9872;
                    } else {
                        let v5461 = v5445.sqrt();
                        let v14982 = v14978 * (v5460 * (v5445.powf(v14979)));
                        v5462 = v5461;
                        v9871 = v14982;
                    }
                    let v5463 = v1 / v5462;
                    let v5464 = v5442 * v5438;
                    let v14987 = v14970 * v5442;
                    let v5466 = v5438 - (v5464 * v5463);
                    let v14993 = v14971 - ((((v14972 * v5438) + (Lanes([0.0, 0.0, v14987[0], 0.0, 0.0, 0.0]))) * v5463) + ((((v9871 * v5463) * v10390) / v5462) * v5464));
                    v5467 = v5466;
                    v9870 = v14993;
                } else {
                    v5467 = v5437;
                    v9870 = v14969;
                }
                let v5470 = (v5467 + v5468).sqrt();
                let v5471 = v5435 * v5470;
                let v14997 = v14964 * v5470;
                let v5473 = (v78 * v665) / v142;
                let v15003 = ((v10415 * v78) / v142) * v5471;
                let v5476 = ((v5473 * v5471) * v4879) * v166;
                let v5478 = v4876 + (v5476 * v5430);
                let v15012 = v9776 + ((((((Lanes([0.0, 0.0, v15003[0], 0.0, 0.0, 0.0])) + (((Lanes([0.0, 0.0, v14997[0], 0.0, 0.0, 0.0])) + ((v9870 * (v9368 / (v10435 * v5470))) * v5435)) * v5473)) * v4879) * v166) * v5430) + (v14963 * v5476));
                v5620 = v5478;
                v6024 = v5394;
                v9831 = v15012;
                v9832 = v9866;
            } else {
                v5620 = v4876;
                v6024 = v4421;
                v9831 = v9776;
                v9832 = v9444;
            }
            let v5481 = if v565 != 0.0 || (if v5479 == v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5640: f64;
            let v9873: Lanes<6>;
            if v5481 != 0.0 {
                let v5484 = if (if v4325 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v1886 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5641: f64;
                let v9874: Lanes<6>;
                if v5484 != 0.0 {
                    v5641 = v0;
                    v9874 = v11062;
                } else {
                    let v5487 = if (if v297 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v21 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v5642: f64;
                    let v9875: Lanes<6>;
                    if v5487 != 0.0 {
                        v5642 = v0;
                        v9875 = v11062;
                    } else {
                        let v15016 = ((Lanes([v10564[0], v10564[1], 0.0, v10564[2], v10564[3]])) + v10791) - v10827;
                        let v5492 = (((v867 - v349) + v1142) - v1199) + v5491;
                        let v5612: f64;
                        let v9876: Lanes<6>;
                        if v281 != 0.0 {
                            let v5493 = v1128 * v1128;
                            let v15103 = v9421 * v1128;
                            let v15104 = v15103 + v15103;
                            let v5494 = v491 / v5493;
                            let v15107 = ((v15104 * v5494) * v10390) / v5493;
                            let v5495 = v78 / v491;
                            let v5496 = v5495 * v5493;
                            let v15111 = v9416 * v2080;
                            let v15113 = (v15016 - (Lanes([0.0, 0.0, v10415[0], 0.0, 0.0]))) - (Lanes([v15111[0], v15111[1], 0.0, 0.0, v15111[2]]));
                            let v5505 = ((v5492 - v665) - (v2080 * v988)) - (v2080 * ((v5500 * v5501) / v124));
                            let v15119 = (v15104 * v5495) * v5505;
                            let v15122 = (Lanes([v15119[0], v15119[1], 0.0, v15119[2], v15119[3], 0.0])) + (((Lanes([v15113[0], v15113[1], v15113[2], v15113[3], v15113[4], 0.0])) - (((v9457 * v5500) / v124) * v2080)) * v5496);
                            let v5507 = v1 + (v5496 * v5505);
                            let v15123 = v15122 * v5507;
                            let v5511 = ((v5507 * v5507) + v5509).sqrt();
                            let v15129 = (v15122 + ((v15123 + v15123) * (v9368 / (v10435 * v5511)))) * v13;
                            let v5515 = (v13 * (v5507 + v5511)) + v5514;
                            let v5516 = if v5515 < v0 { 1.0 } else { 0.0 };
                            let v5517: f64;
                            let v9877: Lanes<6>;
                            if v5516 != 0.0 {
                                v5517 = v0;
                                v9877 = v11062;
                            } else {
                                v5517 = v5515;
                                v9877 = v15129;
                            }
                            let v5519 = (v5517 + v362).sqrt();
                            let v15133 = v15016 * v2097;
                            let v5521 = v1 - v5519;
                            let v15135 = v15107 * v5521;
                            let v15141 = v10561 * v2103;
                            let v5527 = v2106 * v2107;
                            let v5529 = ((v2103 * v866) + v5525) - (v5527 * ((v5492 * v2097) + (v5494 * v5521)));
                            let v15145 = ((Lanes([v15141[0], v15141[1], 0.0, 0.0, v15141[2], 0.0])) + v9777) - (((Lanes([v15133[0], v15133[1], v15133[2], v15133[3], v15133[4], 0.0])) + ((Lanes([v15135[0], v15135[1], 0.0, v15135[2], v15135[3], 0.0])) + (((v9877 * (v9368 / (v10435 * v5519))) * v10390) * v5494))) * v5527);
                            let v15146 = v15145 * v5529;
                            let v5533 = ((v5529 * v5529) + v5531).sqrt();
                            let v15152 = (v15145 + ((v15146 + v15146) * (v9368 / (v10435 * v5533)))) * v13;
                            let v5537 = (v13 * (v5529 + v5533)) + v5536;
                            let v5538 = if v5537 < v0 { 1.0 } else { 0.0 };
                            let v5613: f64;
                            let v9878: Lanes<6>;
                            if v5538 != 0.0 {
                                v5613 = v0;
                                v9878 = v11062;
                            } else {
                                v5613 = v5537;
                                v9878 = v15152;
                            }
                            v5612 = v5613;
                            v9876 = v9878;
                        } else {
                            let v5539 = v2121 * v5492;
                            let v15017 = v15016 * v2121;
                            let v5540 = v1128 * v1128;
                            let v15018 = v9421 * v1128;
                            let v15019 = v15018 + v15018;
                            let v5541 = v491 / v5540;
                            let v15022 = ((v15019 * v5541) * v10390) / v5540;
                            let v5542 = v78 / v491;
                            let v5543 = v5542 * v5540;
                            let v15023 = v15019 * v5542;
                            let v15026 = v9416 * v2080;
                            let v15028 = (v15017 - (Lanes([0.0, 0.0, v10415[0], 0.0, 0.0]))) - (Lanes([v15026[0], v15026[1], 0.0, 0.0, v15026[2]]));
                            let v5550 = ((v5539 - v665) - (v2080 * v988)) - (v2080 * ((v5500 * v5501) / v124));
                            let v15034 = v15023 * v5550;
                            let v15037 = (Lanes([v15034[0], v15034[1], 0.0, v15034[2], v15034[3], 0.0])) + (((Lanes([v15028[0], v15028[1], v15028[2], v15028[3], v15028[4], 0.0])) - (((v9457 * v5500) / v124) * v2080)) * v5543);
                            let v5552 = v1 + (v5543 * v5550);
                            let v5554 = v78 * (v1 + v5543);
                            let v15038 = v15023 * v78;
                            let v5555 = v362 + v5554;
                            let v5558 = if (if v5552 < v5555 { 1.0 } else { 0.0 }) != 0.0 && (if v5554 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let v5590: f64;
                            let v9879: Lanes<6>;
                            if v5558 != 0.0 {
                                let v5559 = v5555 - v5552;
                                let v15039 = Lanes([v15038[0], v15038[1], 0.0, v15038[2], v15038[3], 0.0]);
                                let v15040 = v15039 - v15037;
                                let v5560 = v5559 * v5559;
                                let v15041 = v15040 * v5559;
                                let v15042 = v15041 + v15041;
                                let v5561 = v5554 * v5554;
                                let v15043 = v15038 * v5554;
                                let v15044 = v15043 + v15043;
                                let v5562 = v5560 * v5560;
                                let v15045 = v15042 * v5560;
                                let v5563 = v5561 * v5561;
                                let v15047 = v15044 * v5561;
                                let v5564 = v5562 * v5560;
                                let v5565 = v5563 * v5561;
                                let v15060 = ((((v15047 + v15047) * v5561) + (v15044 * v5563)) * v5561) + (v15044 * v5565);
                                let v5568 = (v5564 * v5560) + (v5565 * v5561);
                                let v15062 = (((((v15045 + v15045) * v5560) + (v15042 * v5562)) * v5560) + (v15042 * v5564)) + (Lanes([v15060[0], v15060[1], 0.0, v15060[2], v15060[3], 0.0]));
                                let v5585: f64;
                                let v9880: Lanes<6>;
                                if v5569 != 0.0 {
                                    let v5579: f64;
                                    if v5570 != 0.0 {
                                        v5579 = v1;
                                    } else {
                                        let v5580: f64;
                                        if v5571 != 0.0 {
                                            v5580 = v78;
                                        } else {
                                            let v5581: f64;
                                            if v5572 != 0.0 {
                                                v5581 = v96;
                                            } else {
                                                let v5582: f64;
                                                if v5573 != 0.0 {
                                                    v5582 = v90;
                                                } else {
                                                    v5582 = v0;
                                                }
                                                v5581 = v5582;
                                            }
                                            v5580 = v5581;
                                        }
                                        v5579 = v5580;
                                    }
                                    let mut v5574: f64 = 0.0;
                                    let mut v5576: f64 = 0.0;
                                    let mut v9881: Lanes<6> = Lanes([0.0; 6]);
                                    v5574 = v0;
                                    v5576 = v5568;
                                    v9881 = v15062;
                                    loop {
                                        let v5575 = if v5574 < v5579 { 1.0 } else { 0.0 };
                                        if v5575 == 0.0 {
                                            break;
                                        }
                                        let v5577 = v5576.sqrt();
                                        let v15102 = v9881 * (v9368 / (v10435 * v5577));
                                        let v5578 = v5574 + v1;
                                        v5574 = v5578;
                                        v5576 = v5577;
                                        v9881 = v15102;
                                    }
                                    v5585 = v5576;
                                    v9880 = v9881;
                                } else {
                                    let v5584 = v5568.powf(v5583);
                                    let v15066 = v15062 * (v5583 * (v5568.powf(v15063)));
                                    v5585 = v5584;
                                    v9880 = v15066;
                                }
                                let v5586 = v1 / v5585;
                                let v5587 = v5559 * v5554;
                                let v15071 = v15038 * v5559;
                                let v5589 = v5555 - (v5587 * v5586);
                                let v15077 = v15039 - ((((v15040 * v5554) + (Lanes([v15071[0], v15071[1], 0.0, v15071[2], v15071[3], 0.0]))) * v5586) + ((((v9880 * v5586) * v10390) / v5585) * v5587));
                                v5590 = v5589;
                                v9879 = v15077;
                            } else {
                                v5590 = v5552;
                                v9879 = v15037;
                            }
                            let v5591 = if v5590 <= v0 { 1.0 } else { 0.0 };
                            let v5593: f64;
                            let v9882: Lanes<6>;
                            if v5591 != 0.0 {
                                v5593 = v0;
                                v9882 = v11062;
                            } else {
                                let v5592 = v5590.sqrt();
                                let v15080 = v9879 * (v9368 / (v10435 * v5592));
                                v5593 = v5592;
                                v9882 = v15080;
                            }
                            let v5594 = v1 - v5593;
                            let v15082 = v15022 * v5594;
                            let v5598 = v143 / (v2106 + v143);
                            let v15088 = v10561 * v2103;
                            let v5602 = ((v2103 * v866) + v5525) - (v5598 * (v5539 + (v5541 * v5594)));
                            let v15092 = ((Lanes([v15088[0], v15088[1], 0.0, 0.0, v15088[2], 0.0])) + v9777) - (((Lanes([v15017[0], v15017[1], v15017[2], v15017[3], v15017[4], 0.0])) + ((Lanes([v15082[0], v15082[1], 0.0, v15082[2], v15082[3], 0.0])) + ((v9882 * v10390) * v5541))) * v5598);
                            let v15093 = v15092 * v5602;
                            let v5606 = ((v5602 * v5602) + v5604).sqrt();
                            let v15099 = (v15092 + ((v15093 + v15093) * (v9368 / (v10435 * v5606)))) * v13;
                            let v5610 = (v13 * (v5602 + v5606)) + v5609;
                            let v5611 = if v5610 < v0 { 1.0 } else { 0.0 };
                            let v5614: f64;
                            let v9883: Lanes<6>;
                            if v5611 != 0.0 {
                                v5614 = v0;
                                v9883 = v11062;
                            } else {
                                v5614 = v5610;
                                v9883 = v15099;
                            }
                            v5612 = v5614;
                            v9876 = v9883;
                        }
                        let v5615 = v5612 + v362;
                        let v5617 = (-v2196) / v5615;
                        let v5618 = v5617.exp();
                        let v5619 = v2200 * v5615;
                        let v5621 = v5619 * v5620;
                        let v5622 = v5621 * v5618;
                        let v15163 = ((((v9876 * v2200) * v5620) + (v9831 * v5619)) * v5618) + (((((v9876 * v5617) * v10390) / v5615) * v5618) * v5621);
                        v5642 = v5622;
                        v9875 = v15163;
                    }
                    v5641 = v5642;
                    v9874 = v9875;
                }
                v5640 = v5641;
                v9873 = v9874;
            } else {
                let v15013 = Lanes([v9458[0], v9458[1], v9458[2], v9458[3], v9458[4], 0.0]);
                v5640 = v5643;
                v9873 = v15013;
            }
            let v5625 = if (if v1886 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v2204 == v78 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5626 = if v5625 != 0.0 && v565 != 0.0 { 1.0 } else { 0.0 };
            let v9210: f64;
            let v9884: Lanes<6>;
            if v5626 != 0.0 {
                let v5628 = (v206 * v12) * v166;
                let v5629 = -v663;
                let v15164 = v10410 * v10390;
                let v5631 = (v5629 * v2208).exp();
                let v5636 = v5633 + (v5634 * v477);
                let v5638 = (v5628 * v5631) * v5636;
                let v5639 = v5637 / v5638;
                let v15173 = (((((((v15164 * v2208) * v5631) * v5628) * v5636) * v5639) * v10390) / v5638) * v5640;
                let v5647 = v2223 * v665;
                let v5648 = v1 + (v5640 * v5639);
                let v5649 = v5648.ln();
                let v15179 = (v10415 * v2223) * v5649;
                let v15183 = Lanes([0.0, 0.0, v9402[0], 0.0, 0.0, 0.0]);
                let v5652 = v766 * v20;
                let v15185 = v9402 * v20;
                let v5653 = (v766 - (v5647 * v5649)) - v5652;
                let v15187 = (v15183 - ((Lanes([0.0, 0.0, v15179[0], 0.0, 0.0, 0.0])) + ((((v9873 * v5639) + (Lanes([0.0, 0.0, v15173[0], 0.0, 0.0, 0.0]))) * (v9368 / v5648)) * v5647))) - (Lanes([0.0, 0.0, v15185[0], 0.0, 0.0, 0.0]));
                let v5654 = v90 * v766;
                let v5655 = v5654 * v5652;
                let v15191 = ((v9402 * v90) * v5652) + (v15185 * v5654);
                let v5656 = if v5655 > v0 { 1.0 } else { 0.0 };
                let v5658: f64;
                let v9885: Lanes<1>;
                if v5656 != 0.0 {
                    v5658 = v5655;
                    v9885 = v15191;
                } else {
                    let v5657 = -v5655;
                    let v15192 = v15191 * v10390;
                    v5658 = v5657;
                    v9885 = v15192;
                }
                let v15193 = v15187 * v5653;
                let v5661 = ((v5653 * v5653) + v5658).sqrt();
                let v5666 = v5665 * v477;
                let v5668 = (v5666 * v665).sqrt();
                let v15206 = (v10415 * v5666) * (v9368 / (v10435 * v5668));
                let v5669 = v5525 - (v766 - (v13 * (v5653 + v5661)));
                let v15207 = v9777 - (v15183 - ((v15187 + (((v15193 + v15193) + (Lanes([0.0, 0.0, v9885[0], 0.0, 0.0, 0.0]))) * (v9368 / (v10435 * v5661)))) * v13));
                let v15208 = v15164 * v5669;
                let v5671 = (v5629 * v5669).exp();
                let v15213 = v10410 * v5669;
                let v5674 = (v5671 - v1) + (v663 * v5669);
                let v15217 = (((Lanes([0.0, 0.0, v15208[0], 0.0, 0.0, 0.0])) + (v15207 * v5629)) * v5671) + ((Lanes([0.0, 0.0, v15213[0], 0.0, 0.0, 0.0])) + (v15207 * v663));
                let v5675 = if v5674 > v0 { 1.0 } else { 0.0 };
                let v5680: f64;
                let v9886: Lanes<6>;
                if v5675 != 0.0 {
                    let v5676 = v5674.sqrt();
                    let v15225 = v15217 * (v9368 / (v10435 * v5676));
                    v5680 = v5676;
                    v9886 = v15225;
                } else {
                    let v5678 = (-v5674).sqrt();
                    let v5679 = -v5678;
                    let v15222 = ((v15217 * v10390) * (v9368 / (v10435 * v5678))) * v10390;
                    v5680 = v5679;
                    v9886 = v15222;
                }
                let v15226 = v15164 * v5525;
                let v5682 = (v5629 * v5525).exp();
                let v15231 = v10410 * v5525;
                let v5686 = ((v5682 - v1) + (v663 * v5525)).sqrt();
                let v5687 = -v5668;
                let v5688 = v5680 - v5686;
                let v15241 = (v15206 * v10390) * v5688;
                let v15245 = ((Lanes([0.0, 0.0, v15241[0], 0.0, 0.0, 0.0])) + ((v9886 - (((((Lanes([0.0, 0.0, v15226[0], 0.0, 0.0, 0.0])) + (v9777 * v5629)) * v5682) + ((Lanes([0.0, 0.0, v15231[0], 0.0, 0.0, 0.0])) + (v9777 * v663))) * (v9368 / (v10435 * v5686)))) * v5687)) * v10390;
                let v5692 = v5690 * v20;
                let v5693 = (v5690 - (v5687 * v5688)) - v5692;
                let v5695 = (v90 * v5690) * v5692;
                let v5696 = if v5695 > v0 { 1.0 } else { 0.0 };
                let v5698: f64;
                if v5696 != 0.0 {
                    v5698 = v5695;
                } else {
                    let v5697 = -v5695;
                    v5698 = v5697;
                }
                let v15246 = v15245 * v5693;
                let v5701 = ((v5693 * v5693) + v5698).sqrt();
                let v5704 = v5690 - (v13 * (v5693 + v5701));
                let v15253 = ((v15245 + ((v15246 + v15246) * (v9368 / (v10435 * v5701)))) * v13) * v10390;
                let v5705 = if v2248 > v0 { 1.0 } else { 0.0 };
                let v5706: f64;
                if v5705 != 0.0 {
                    v5706 = v2248;
                } else {
                    v5706 = v1;
                }
                let v5707 = v5640 + v2249;
                let v5708 = v5706 / v5707;
                let v5709 = v5708 * v1128;
                let v15258 = v9421 * v5708;
                let v15261 = v9380 * v5710;
                let v5713 = ((v5710 * v2254) - v5704) / v5709;
                let v15266 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v15261[0]])) - v15253) - ((((((v9873 * v5708) * v10390) / v5707) * v1128) + (Lanes([v15258[0], v15258[1], 0.0, v15258[2], v15258[3], 0.0]))) * v5713)) / v5709;
                v9210 = v5713;
                v9884 = v15266;
            } else {
                v9210 = v9211;
                v9884 = v9467;
            }
            let v5714 = if v4325 == v0 { 1.0 } else { 0.0 };
            let v5719 = if (if v5714 != 0.0 && (if v5640 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v5717 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v8419: f64;
            let v9887: Lanes<6>;
            if v5719 != 0.0 {
                let v5730: f64;
                let v5746: f64;
                let v9888: Lanes<6>;
                let v9889: Lanes<6>;
                if v986 != 0.0 {
                    v5730 = v0;
                    v5746 = v0;
                    v9888 = v11062;
                    v9889 = v11062;
                } else {
                    let v5720: f64;
                    let v9890: Lanes<6>;
                    if v565 != 0.0 {
                        let v15267 = Lanes([v9413[0], v9413[1], 0.0, 0.0, v9413[2], 0.0]);
                        v5720 = v835;
                        v9890 = v15267;
                    } else {
                        v5720 = v4637;
                        v9890 = v9449;
                    }
                    let v5724: f64;
                    let v9891: Lanes<6>;
                    if v565 != 0.0 {
                        let v15268 = Lanes([v9413[0], v9413[1], 0.0, 0.0, v9413[2], 0.0]);
                        v5724 = v835;
                        v9891 = v15268;
                    } else {
                        v5724 = v5721;
                        v9891 = v9459;
                    }
                    v5730 = v5720;
                    v5746 = v5724;
                    v9888 = v9890;
                    v9889 = v9891;
                }
                let v5728 = v5717 * (v1 + (v5725 * v1142));
                let v5729 = v5728 * v5640;
                let v15271 = ((v10791 * v5725) * v5717) * v5640;
                let v15274 = (Lanes([v15271[0], v15271[1], v15271[2], v15271[3], v15271[4], 0.0])) + (v9873 * v5728);
                let v5731 = v4340 - v5730;
                let v15276 = v10410 * v5731;
                let v15279 = (Lanes([0.0, 0.0, v15276[0], 0.0, 0.0, 0.0])) + ((v9441 - v9888) * v663);
                let v5733 = (v663 * v5731) - v1;
                let v15280 = v15279 * v5733;
                let v5737 = ((v5733 * v5733) + v5735).sqrt();
                let v15286 = (v15279 + ((v15280 + v15280) * (v9368 / (v10435 * v5737)))) * v13;
                let v5741 = (v13 * (v5733 + v5737)) + v5740;
                let v5742 = if v5741 < v0 { 1.0 } else { 0.0 };
                let v5743: f64;
                let v9892: Lanes<6>;
                if v5742 != 0.0 {
                    v5743 = v0;
                    v9892 = v11062;
                } else {
                    v5743 = v5741;
                    v9892 = v15286;
                }
                let v5744 = v5743.sqrt();
                let v15289 = v9892 * (v9368 / (v10435 * v5744));
                let v5745 = v5743 * v5744;
                let v15292 = (v9892 * v5744) + (v15289 * v5743);
                let v5747 = v4336 - v5746;
                let v15294 = v10410 * v5747;
                let v15297 = (Lanes([0.0, 0.0, v15294[0], 0.0, 0.0, 0.0])) + ((v9440 - v9889) * v663);
                let v5749 = (v663 * v5747) - v1;
                let v15298 = v15297 * v5749;
                let v5753 = ((v5749 * v5749) + v5751).sqrt();
                let v15304 = (v15297 + ((v15298 + v15298) * (v9368 / (v10435 * v5753)))) * v13;
                let v5757 = (v13 * (v5749 + v5753)) + v5756;
                let v5758 = if v5757 < v0 { 1.0 } else { 0.0 };
                let v5759: f64;
                let v9893: Lanes<6>;
                if v5758 != 0.0 {
                    v5759 = v0;
                    v9893 = v11062;
                } else {
                    v5759 = v5757;
                    v9893 = v15304;
                }
                let v5760 = v5759.sqrt();
                let v15307 = v9893 * (v9368 / (v10435 * v5760));
                let v5761 = v5759 * v5760;
                let v5762 = v1 / v5743;
                let v5763 = v663 * v5729;
                let v15314 = v10410 * v5729;
                let v15317 = (Lanes([0.0, 0.0, v15314[0], 0.0, 0.0, 0.0])) + (v15274 * v663);
                let v5764 = v5763 * v5762;
                let v15320 = (v15317 * v5762) + ((((v9892 * v5762) * v10390) / v5743) * v5763);
                let v5765 = v1 / v5759;
                let v5766 = v5763 * v5765;
                let v15326 = (v15317 * v5765) + ((((v9893 * v5765) * v10390) / v5759) * v5763);
                let v5769 = (v5761 * v5766) - (v5745 * v5764);
                let v15334 = v10485 * v5769;
                let v5771 = v750 * v13;
                let v5772 = -v5760;
                let v5775 = (v5772 * v5766) + (v5744 * v5764);
                let v15347 = (v10485 * v13) * v5775;
                let v5777 = (v750 * v5769) + (v5771 * v5775);
                let v5779 = v5778 * v5777;
                let v5784 = v5779 * v5780;
                let v15357 = (((v9778 * v5777) + ((((Lanes([0.0, 0.0, v15334[0], 0.0, 0.0, 0.0])) + ((((((v9893 * v5760) + (v15307 * v5759)) * v5766) + (v15326 * v5761)) - ((v15292 * v5764) + (v15320 * v5745))) * v750)) + ((Lanes([0.0, 0.0, v15347[0], 0.0, 0.0, 0.0])) + (((((v15307 * v10390) * v5766) + (v15326 * v5772)) + ((v15289 * v5764) + (v15320 * v5744))) * v5771))) * v5778)) * v5780) + (v9779 * v5779);
                v8419 = v5784;
                v9887 = v15357;
            } else {
                v8419 = v0;
                v9887 = v11062;
            }
            let v5785 = v122 * v68;
            let v5786 = v1128 / v556;
            let v15358 = v9421 / v556;
            let v5787 = v136 * v68;
            let v5788 = v166 * v68;
            let v5790 = v5789 / v68;
            let v15359 = v9780 / v68;
            let v5791 = v4428 / v556;
            let v15360 = v9445 / v556;
            let v5792 = v750 / v556;
            let v15361 = v10485 / v556;
            let v5794 = if v5793 == v0 { 1.0 } else { 0.0 };
            let v8680: f64;
            let v8684: f64;
            let v8685: f64;
            let v8689: f64;
            let v8694: f64;
            let v9894: Lanes<4>;
            let v9895: Lanes<6>;
            let v9896: Lanes<3>;
            let v9897: Lanes<3>;
            if v5794 != 0.0 {
                v8680 = v0;
                v8684 = v0;
                v8685 = v0;
                v8689 = v0;
                v8694 = v0;
                v9894 = v10625;
                v9895 = v11062;
                v9896 = v10531;
                v9897 = v10531;
            } else {
                let v8686: f64;
                let v9898: Lanes<6>;
                if v5714 != 0.0 {
                    let v15368 = (Lanes([v10564[0], v10564[1], 0.0, v10564[2], v10564[3]])) + (((v10791 - v10827) * v5799) * v5787);
                    let v5807 = v1 / v5785;
                    let v5808 = (((v867 - v240) + ((v5799 * (v1142 - v1199)) * v5787)) - (((v5525 + v866) - v5796) * v5804)) * v5807;
                    let v5810 = v1 / v5809;
                    let v5812 = v1 + (v5790 * v5810);
                    let v5813 = v5808 * v5812;
                    let v15376 = ((((Lanes([v15368[0], v15368[1], v15368[2], v15368[3], v15368[4], 0.0])) - ((v9777 + (Lanes([v10561[0], v10561[1], 0.0, 0.0, v10561[2], 0.0]))) * v5804)) * v5807) * v5812) + ((v15359 * v5810) * v5808);
                    let v15377 = v15376 * v5813;
                    let v5817 = ((v5813 * v5813) + v5815).sqrt();
                    let v15383 = (v15376 + ((v15377 + v15377) * (v9368 / (v10435 * v5817)))) * v13;
                    let v5821 = (v13 * (v5813 + v5817)) + v5820;
                    let v5822 = if v5821 < v0 { 1.0 } else { 0.0 };
                    let v5839: f64;
                    let v9899: Lanes<6>;
                    if v5822 != 0.0 {
                        v5839 = v0;
                        v9899 = v11062;
                    } else {
                        v5839 = v5821;
                        v9899 = v15383;
                    }
                    let v15384 = v10564 * v867;
                    let v5826 = ((v867 * v867) + v5824).sqrt();
                    let v15390 = (v10564 + ((v15384 + v15384) * (v9368 / (v10435 * v5826)))) * v13;
                    let v5830 = (v13 * (v867 + v5826)) + v5829;
                    let v5831 = if v5830 < v0 { 1.0 } else { 0.0 };
                    let v5832: f64;
                    let v9900: Lanes<4>;
                    if v5831 != 0.0 {
                        v5832 = v0;
                        v9900 = v10625;
                    } else {
                        v5832 = v5830;
                        v9900 = v15390;
                    }
                    let v5834 = (v5832 - v840) / v79;
                    let v15392 = (v9900 / v79) * v5834;
                    let v5836 = v1 + (v5834 * v5834);
                    let v5837 = v1 / v5836;
                    let v5838 = v1 - v5837;
                    let v5840 = v5839 * v5838;
                    let v15399 = (((((v15392 + v15392) * v5837) * v10390) / v5836) * v10390) * v5839;
                    let v15401 = (v9899 * v5838) + (Lanes([v15399[0], v15399[1], 0.0, v15399[2], v15399[3], 0.0]));
                    let v5841 = v5787 * v5788;
                    let v5844 = v5842 / (v5842 + v5841);
                    let v5846 = v5845 + v866;
                    let v5847 = v5845 / v5846;
                    let v15404 = ((v10561 * v5847) * v10390) / v5846;
                    let v5848 = v5840 + v362;
                    let v5849 = v1 / v5848;
                    let v5851 = -v5850;
                    let v5852 = v5851 * v719;
                    let v5853 = v5852 * v5849;
                    let v15409 = (v10441 * v5851) * v5849;
                    let v15412 = (Lanes([0.0, 0.0, v15409[0], 0.0, 0.0, 0.0])) + ((((v15401 * v5849) * v10390) / v5848) * v5852);
                    let v5855 = if v5853 < v5854 { 1.0 } else { 0.0 };
                    let v8687: f64;
                    let v9901: Lanes<6>;
                    if v5855 != 0.0 {
                        v8687 = v0;
                        v9901 = v11062;
                    } else {
                        let v5856 = v5853.exp();
                        let v5858 = v5857 / v718;
                        let v5860 = (v5858 * v206) * v5841;
                        let v5861 = v1 / v5792;
                        let v15422 = v15358 * v11;
                        let v5863 = v5791 + (v5786 * v11);
                        let v15426 = (((v15361 * v5861) * v10390) / v5792) * v5863;
                        let v5865 = (v5863 * v5861).sqrt();
                        let v5866 = v5856 * v5860;
                        let v15433 = (((((v10438 * v5858) * v10390) / v718) * v206) * v5841) * v5856;
                        let v5867 = v5866 * v5865;
                        let v5868 = v5867 * v5840;
                        let v5869 = v5868 * v5840;
                        let v5870 = v5844 * v5847;
                        let v5871 = v5870 * v5869;
                        let v15446 = (v15404 * v5844) * v5869;
                        let v15449 = (Lanes([v15446[0], v15446[1], 0.0, 0.0, v15446[2], 0.0])) + ((((((((((v15412 * v5856) * v5860) + (Lanes([0.0, 0.0, v15433[0], 0.0, 0.0, 0.0]))) * v5865) + (((((v15360 + (Lanes([v15422[0], v15422[1], 0.0, v15422[2], v15422[3], 0.0]))) * v5861) + (Lanes([0.0, 0.0, v15426[0], 0.0, 0.0, 0.0]))) * (v9368 / (v10435 * v5865))) * v5866)) * v5840) + (v15401 * v5867)) * v5840) + (v15401 * v5868)) * v5870);
                        v8687 = v5871;
                        v9901 = v15449;
                    }
                    v8686 = v8687;
                    v9898 = v9901;
                } else {
                    v8686 = v0;
                    v9898 = v11062;
                }
                let v5873 = -v5872;
                let v5878 = (v5785 * ((v5873 * v830) + v5875)).exp();
                let v5880 = (v830 / v5785) / v5785;
                let v5881 = v830 * v5880;
                let v5884 = (v5882 / v61) * v5788;
                let v5885 = v5884 * v5878;
                let v5886 = v5885 * v5881;
                let v15461 = (((((v9412 * v5873) * v5785) * v5878) * v5884) * v5881) + (((v9412 * v5880) + (((v9412 / v5785) / v5785) * v830)) * v5885);
                let v5887 = if v830 >= v0 { 1.0 } else { 0.0 };
                let v8695: f64;
                let v9902: Lanes<3>;
                if v5887 != 0.0 {
                    let v5889 = v5886 * v5888;
                    let v15462 = v15461 * v5888;
                    v8695 = v5889;
                    v9902 = v15462;
                } else {
                    v8695 = v5886;
                    v9902 = v15461;
                }
                let v5890 = v830 - v823;
                let v15464 = v9412 - (Lanes([v9410[0], v9410[1], 0.0]));
                let v5894 = (v5785 * ((v5873 * v5890) + v5875)).exp();
                let v5896 = (v5890 / v5785) / v5785;
                let v5897 = v5890 * v5896;
                let v5898 = v5884 * v5894;
                let v5899 = v5898 * v5897;
                let v15476 = (((((v15464 * v5873) * v5785) * v5894) * v5884) * v5897) + (((v15464 * v5896) + (((v15464 / v5785) / v5785) * v5890)) * v5898);
                let v5900 = if v5890 >= v0 { 1.0 } else { 0.0 };
                let v8690: f64;
                let v9903: Lanes<3>;
                if v5900 != 0.0 {
                    let v5902 = v5899 * v5901;
                    let v15477 = v15476 * v5901;
                    v8690 = v5902;
                    v9903 = v15477;
                } else {
                    v8690 = v5899;
                    v9903 = v15476;
                }
                let v15478 = v9412 * v10390;
                let v5908 = ((((-v830) + v878) + v240) + v5906) / v5785;
                let v15482 = ((Lanes([v15478[0], v15478[1], v15478[2], 0.0])) + (Lanes([v9415[0], v9415[1], 0.0, v9415[2]]))) / v5785;
                let v15483 = v15482 * v5908;
                let v5912 = ((v5908 * v5908) + v5910).sqrt();
                let v15489 = (v15482 + ((v15483 + v15483) * (v9368 / (v10435 * v5912)))) * v13;
                let v5916 = (v13 * (v5908 + v5912)) + v5915;
                let v5917 = if v5916 < v0 { 1.0 } else { 0.0 };
                let v5918: f64;
                let v9904: Lanes<4>;
                if v5917 != 0.0 {
                    v5918 = v0;
                    v9904 = v10625;
                } else {
                    v5918 = v5916;
                    v9904 = v15489;
                }
                let v5919 = v5918 + v362;
                let v5922 = (-v5920) / v5919;
                let v15492 = ((v9904 * v5922) * v10390) / v5919;
                let v5924 = if v5922 < v5923 { 1.0 } else { 0.0 };
                let v8681: f64;
                let v9905: Lanes<4>;
                if v5924 != 0.0 {
                    v8681 = v0;
                    v9905 = v10625;
                } else {
                    let v5925 = v5922.exp();
                    let v5928 = (v5926 * v5788) * v5787;
                    let v5929 = v5928 * v5919;
                    let v5930 = v5929 * v5919;
                    let v5931 = v5930 * v5925;
                    let v15500 = ((((v9904 * v5928) * v5919) + (v9904 * v5929)) * v5925) + ((v15492 * v5925) * v5930);
                    v8681 = v5931;
                    v9905 = v15500;
                }
                v8680 = v8681;
                v8684 = v13;
                v8685 = v8686;
                v8689 = v8690;
                v8694 = v8695;
                v9894 = v9905;
                v9895 = v9898;
                v9896 = v9903;
                v9897 = v9902;
            }
            let v5933 = if v5932 == v0 { 1.0 } else { 0.0 };
            let v8702: f64;
            let v9906: Lanes<5>;
            if v5933 != 0.0 {
                v8702 = v0;
                v9906 = v10579;
            } else {
                let v15501 = v9410 * v5934;
                let v15503 = (Lanes([v15501[0], v15501[1], 0.0])) - v9412;
                let v5942 = v1 / v122;
                let v5943 = (((v5934 * (v823 + v5935)) - v830) + (v1138 * v5939)) * v5942;
                let v15507 = ((Lanes([v15503[0], v15503[1], 0.0, v15503[2], 0.0])) + (v10788 * v5939)) * v5942;
                let v15508 = v15507 * v5943;
                let v5947 = ((v5943 * v5943) + v5945).sqrt();
                let v15514 = (v15507 + ((v15508 + v15508) * (v9368 / (v10435 * v5947)))) * v13;
                let v5951 = (v13 * (v5943 + v5947)) + v5950;
                let v5952 = if v5951 < v0 { 1.0 } else { 0.0 };
                let v5953: f64;
                let v9907: Lanes<5>;
                if v5952 != 0.0 {
                    v5953 = v0;
                    v9907 = v10579;
                } else {
                    v5953 = v5951;
                    v9907 = v15514;
                }
                let v5954 = v5953 + v362;
                let v5955 = v1 / v5954;
                let v5957 = -v5956;
                let v5958 = v5957 * v719;
                let v5959 = v5958 * v5955;
                let v15519 = (v10441 * v5957) * v5955;
                let v15522 = (Lanes([0.0, 0.0, v15519[0], 0.0, 0.0])) + ((((v9907 * v5955) * v10390) / v5954) * v5958);
                let v5961 = if v5959 < v5960 { 1.0 } else { 0.0 };
                let v5977: f64;
                let v9908: Lanes<5>;
                if v5961 != 0.0 {
                    v5977 = v0;
                    v9908 = v10579;
                } else {
                    let v5962 = v5959.exp();
                    let v5964 = v5963 / v718;
                    let v5966 = (v5964 * v206) * v166;
                    let v5967 = v5966 * v5953;
                    let v15529 = (((((v10438 * v5964) * v10390) / v718) * v206) * v166) * v5953;
                    let v5968 = v5967 * v5953;
                    let v5969 = v5968 * v5962;
                    let v15538 = (((((Lanes([0.0, 0.0, v15529[0], 0.0, 0.0])) + (v9907 * v5966)) * v5953) + (v9907 * v5967)) * v5962) + ((v15522 * v5962) * v5968);
                    v5977 = v5969;
                    v9908 = v15538;
                }
                let v5970 = v823 - v878;
                let v15539 = v10560 - v9415;
                let v5971 = if v5970 > v0 { 1.0 } else { 0.0 };
                let v8703: f64;
                let v9909: Lanes<5>;
                if v5971 != 0.0 {
                    let v5972 = v5970 * v5970;
                    let v15540 = v15539 * v5970;
                    let v5973 = v5972 * v5970;
                    let v15544 = ((v15540 + v15540) * v5970) + (v15539 * v5972);
                    let v5975 = v5973 + v5974;
                    let v5976 = v5973 / v5975;
                    let v5978 = v5977 * v5976;
                    let v15549 = ((v15544 - (v15544 * v5976)) / v5975) * v5977;
                    let v15551 = (v9908 * v5976) + (Lanes([v15549[0], v15549[1], 0.0, 0.0, v15549[2]]));
                    v8703 = v5978;
                    v9909 = v15551;
                } else {
                    v8703 = v0;
                    v9909 = v10579;
                }
                v8702 = v8703;
                v9906 = v9909;
            }
            let v8704: f64;
            let v9910: Lanes<5>;
            if v5933 != 0.0 {
                v8704 = v0;
                v9910 = v10579;
            } else {
                let v15553 = (v9410 * v10390) * v5934;
                let v15557 = (Lanes([v15553[0], v15553[1], 0.0])) - (v9412 - (Lanes([v9410[0], v9410[1], 0.0])));
                let v5986 = v1 / v122;
                let v5987 = (((v5934 * ((-v823) + v5935)) - (v830 - v823)) + (v1138 * v5939)) * v5986;
                let v15561 = ((Lanes([v15557[0], v15557[1], 0.0, v15557[2], 0.0])) + (v10788 * v5939)) * v5986;
                let v15562 = v15561 * v5987;
                let v5991 = ((v5987 * v5987) + v5989).sqrt();
                let v15568 = (v15561 + ((v15562 + v15562) * (v9368 / (v10435 * v5991)))) * v13;
                let v5995 = (v13 * (v5987 + v5991)) + v5994;
                let v5996 = if v5995 < v0 { 1.0 } else { 0.0 };
                let v5997: f64;
                let v9911: Lanes<5>;
                if v5996 != 0.0 {
                    v5997 = v0;
                    v9911 = v10579;
                } else {
                    v5997 = v5995;
                    v9911 = v15568;
                }
                let v5998 = v5997 + v362;
                let v5999 = v1 / v5998;
                let v6000 = -v5956;
                let v6001 = v6000 * v719;
                let v6002 = v6001 * v5999;
                let v15573 = (v10441 * v6000) * v5999;
                let v15576 = (Lanes([0.0, 0.0, v15573[0], 0.0, 0.0])) + ((((v9911 * v5999) * v10390) / v5998) * v6001);
                let v6004 = if v6002 < v6003 { 1.0 } else { 0.0 };
                let v6019: f64;
                let v9912: Lanes<5>;
                if v6004 != 0.0 {
                    v6019 = v0;
                    v9912 = v10579;
                } else {
                    let v6005 = v6002.exp();
                    let v6006 = v1 / v718;
                    let v6009 = ((v5963 * v6006) * v206) * v166;
                    let v6010 = v6009 * v5997;
                    let v15584 = ((((((v10438 * v6006) * v10390) / v718) * v5963) * v206) * v166) * v5997;
                    let v6011 = v6010 * v5997;
                    let v6012 = v6011 * v6005;
                    let v15593 = (((((Lanes([0.0, 0.0, v15584[0], 0.0, 0.0])) + (v9911 * v6009)) * v5997) + (v9911 * v6010)) * v6005) + ((v15576 * v6005) * v6011);
                    v6019 = v6012;
                    v9912 = v15593;
                }
                let v6013 = -v878;
                let v15594 = v9415 * v10390;
                let v6014 = if v6013 > v0 { 1.0 } else { 0.0 };
                let v8705: f64;
                let v9913: Lanes<5>;
                if v6014 != 0.0 {
                    let v6015 = v6013 * v6013;
                    let v15595 = v15594 * v6013;
                    let v6016 = v6015 * v6013;
                    let v15599 = ((v15595 + v15595) * v6013) + (v15594 * v6015);
                    let v6017 = v6016 + v5974;
                    let v6018 = v6016 / v6017;
                    let v6020 = v6019 * v6018;
                    let v15604 = ((v15599 - (v15599 * v6018)) / v6017) * v6019;
                    let v15606 = (v9912 * v6018) + (Lanes([v15604[0], v15604[1], 0.0, 0.0, v15604[2]]));
                    v8705 = v6020;
                    v9913 = v15606;
                } else {
                    v8705 = v0;
                    v9913 = v10579;
                }
                v8704 = v8705;
                v9910 = v9913;
            }
            let v8539: f64;
            let v8547: f64;
            let v8555: f64;
            let v8567: f64;
            let v8579: f64;
            let v8586: f64;
            let v8596: f64;
            let v8603: f64;
            let v9914: Lanes<5>;
            let v9915: Lanes<5>;
            let v9916: Lanes<6>;
            let v9917: Lanes<6>;
            let v9918: Lanes<5>;
            let v9919: Lanes<6>;
            let v9920: Lanes<5>;
            let v9921: Lanes<6>;
            if v565 != 0.0 {
                let v6021 = v1 / v127;
                let v6022 = -v3861;
                let v6023 = v6022 * v4428;
                let v15607 = v9445 * v6022;
                let v6026 = v6023 + (v6022 * v6024);
                let v15609 = v15607 + (v9832 * v6022);
                let v6027 = v6023 * v13;
                let v15610 = v15607 * v13;
                let v6028 = v6023 - v6027;
                let v15611 = v15607 - v15610;
                let v6029 = v6026 * v13;
                let v15612 = v15609 * v13;
                let v6030 = v6026 - v6029;
                let v15613 = v15609 - v15612;
                let v8540: f64;
                let v8548: f64;
                let v8556: f64;
                let v8568: f64;
                let v8580: f64;
                let v8587: f64;
                let v8597: f64;
                let v8604: f64;
                let v9922: Lanes<5>;
                let v9923: Lanes<5>;
                let v9924: Lanes<6>;
                let v9925: Lanes<6>;
                let v9926: Lanes<5>;
                let v9927: Lanes<6>;
                let v9928: Lanes<5>;
                let v9929: Lanes<6>;
                if v566 != 0.0 {
                    let v6038: f64;
                    let v6098: f64;
                    let v6456: f64;
                    if v6031 != 0.0 {
                        let v6034 = v6032 * v13;
                        v6038 = v371;
                        v6098 = v6035;
                        v6456 = v6034;
                    } else {
                        let v6039: f64;
                        let v6099: f64;
                        let v6457: f64;
                        if v6036 != 0.0 {
                            let v6037 = v3861 * v13;
                            v6039 = v1;
                            v6099 = v240;
                            v6457 = v6037;
                        } else {
                            v6039 = v0;
                            v6099 = v0;
                            v6457 = v0;
                        }
                        v6038 = v6039;
                        v6098 = v6099;
                        v6456 = v6457;
                    }
                    let v6040 = if v6038 == v0 { 1.0 } else { 0.0 };
                    let v8541: f64;
                    let v8549: f64;
                    let v8557: f64;
                    let v8569: f64;
                    let v8581: f64;
                    let v8588: f64;
                    let v8598: f64;
                    let v8605: f64;
                    let v9930: Lanes<5>;
                    let v9931: Lanes<5>;
                    let v9932: Lanes<6>;
                    let v9933: Lanes<6>;
                    let v9934: Lanes<5>;
                    let v9935: Lanes<6>;
                    let v9936: Lanes<5>;
                    let v9937: Lanes<6>;
                    if v6040 != 0.0 {
                        let v6042 = (v489 / v489).sqrt();
                        let v6043 = v750 * v6042;
                        let v15614 = v10485 * v6042;
                        let v6051 = (v6046 * v835) + (v6048 * (v835 - v823));
                        let v15618 = (v9413 * v6046) + ((v9413 - v10560) * v6048);
                        let v15622 = (v9410 * v6046) + ((v9410 * v10390) * v6048);
                        let v6057 = v830 - v823;
                        let v15625 = v9412 - (Lanes([v9410[0], v9410[1], 0.0]));
                        let v6059 = (v6046 * v830) + (v6048 * v6057);
                        let v15627 = (v9412 * v6046) + (v15625 * v6048);
                        let v6062 = (v6048 * v830) + (v6046 * v6057);
                        let v15630 = (v9412 * v6048) + (v15625 * v6046);
                        let v6063 = ((v6046 * v823) + (v6048 * (-v823))) - v6051;
                        let v15632 = (Lanes([v15622[0], v15622[1], 0.0])) - v15618;
                        let v6064 = -v6051;
                        let v15633 = v15618 * v10390;
                        let v6066 = v6046 + (v6045 * v6048);
                        let v6068 = v6048 + (v6045 * v6046);
                        let v6071 = (v6066 * v6059) + (v6068 * v6062);
                        let v15636 = (v15627 * v6066) + (v15630 * v6068);
                        let v6077 = -(((v6066 * v6064) + (v6068 * v6063)) + v6075);
                        let v15640 = ((v15633 * v6066) + (v15632 * v6068)) * v10390;
                        let v6078 = if v6077 > v783 { 1.0 } else { 0.0 };
                        let v6093: f64;
                        let v9938: Lanes<3>;
                        if v6078 != 0.0 {
                            let v6080 = v779 - v783;
                            let v6081 = (v6077 - v783) / v6080;
                            let v15641 = v15640 / v6080;
                            let v6082 = v6081 * v6081;
                            let v15642 = v15641 * v6081;
                            let v15643 = v15642 + v15642;
                            let v15647 = v15643 * v6082;
                            let v6088 = (((v1 + v6081) + v6082) + (v6082 * v6081)) + (v6082 * v6082);
                            let v6089 = v1 / v6088;
                            let v15656 = (((((((v15641 + v15643) + ((v15643 * v6081) + (v15641 * v6082))) + (v15647 + v15647)) * v6089) * v10390) / v6088) * v10390) * v6080;
                            let v6092 = v783 + (v6080 * (v1 - v6089));
                            v6093 = v6092;
                            v9938 = v15656;
                        } else {
                            v6093 = v6077;
                            v9938 = v15640;
                        }
                        let v15657 = v9938 * v10390;
                        let v6095 = (-v6093) - v11;
                        let v6096 = v6043 * v6021;
                        let v15658 = v15614 * v6021;
                        let v6097 = v6096 * v6096;
                        let v15659 = v15658 * v6096;
                        let v15660 = v15659 + v15659;
                        let v6100 = v6071 - v6098;
                        let v6101 = v489 / v731;
                        let v6102 = v78 / v663;
                        let v6103 = v6101.ln();
                        let v6104 = v6102 * v6103;
                        let v15671 = ((((v10410 * v6102) * v10390) / v663) * v6103) + (((((v10453 * v6101) * v10390) / v731) * (v9368 / v6101)) * v6102);
                        let v6105 = -v6095;
                        let v15672 = v15657 * v10390;
                        let v6106 = if v6100 < v6105 { 1.0 } else { 0.0 };
                        let v6450: f64;
                        let v6452: f64;
                        let v6829: f64;
                        let v6839: f64;
                        let v6844: f64;
                        let v9939: Lanes<5>;
                        let v9940: Lanes<5>;
                        let v9941: Lanes<5>;
                        let v9942: Lanes<5>;
                        let v9943: Lanes<5>;
                        if v6106 != 0.0 {
                            let v6107 = v663 * v6043;
                            let v6108 = v1 / v6107;
                            let v6109 = v6108 * v127;
                            let v16056 = (((((v10410 * v6043) + (v15614 * v663)) * v6108) * v10390) / v6107) * v127;
                            let v16057 = v16056 * v6110;
                            let v6112 = v78 + (v6110 * v6109);
                            let v6113 = v91 * v6112;
                            let v6114 = v6113 * v6112;
                            let v6115 = v6114 * v6112;
                            let v16064 = ((((v16057 * v91) * v6112) + (v16057 * v6113)) * v6112) + (v16057 * v6114);
                            let v6116 = v661 - v6104;
                            let v16065 = v10406 - v15671;
                            let v6117 = v6100 + v6095;
                            let v16069 = v10410 * v6117;
                            let v16070 = ((Lanes([v15636[0], v15636[1], v15636[2], 0.0])) + (Lanes([v15657[0], v15657[1], 0.0, v15657[2]]))) * v663;
                            let v6120 = v3500 * v6109;
                            let v6121 = (v663 * v6117) - v78;
                            let v6122 = v6120 * v6121;
                            let v16075 = (v16056 * v3500) * v6121;
                            let v16078 = (Lanes([0.0, 0.0, v16075[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v16069[0], 0.0, 0.0])) + (Lanes([v16070[0], v16070[1], 0.0, v16070[2], v16070[3]]))) * v6120);
                            let v6123 = v6119 - v6122;
                            let v16079 = v16078 * v10390;
                            let v6124 = v6123 * v6123;
                            let v16080 = v16079 * v6123;
                            let v16081 = v16080 + v16080;
                            let v6126 = if v6115 < (v6124 * v3506) { 1.0 } else { 0.0 };
                            let v6138: f64;
                            let v9944: Lanes<5>;
                            if v6126 != 0.0 {
                                let v16088 = v16064 * v13;
                                let v6130 = (v13 * v6115) / v6123;
                                let v6132 = ((v6127 + v6123) + v6130) + v6122;
                                let v16094 = (v16079 + (((Lanes([0.0, 0.0, v16088[0], 0.0, 0.0])) - (v16079 * v6130)) / v6123)) + v16078;
                                v6138 = v6132;
                                v9944 = v16094;
                            } else {
                                let v6134 = (v6115 + v6124).sqrt();
                                let v6137 = (v6135 + v6134) + v6122;
                                let v16087 = (((Lanes([0.0, 0.0, v16064[0], 0.0, 0.0])) + v16081) * (v9368 / (v10435 * v6134))) + v16078;
                                v6138 = v6137;
                                v9944 = v16087;
                            }
                            let v6139 = v6138.powf(v1562);
                            let v16098 = v9944 * (v1562 * (v6138.powf(v16095)));
                            let v16100 = (v16056 * v3523) * v10390;
                            let v6145 = v748 * v6139;
                            let v6148 = (((v6140 - (v3523 * v6109)) + (v78 * v6139)) + (v6145 * v6139)) / v6139;
                            let v16113 = v10415 * v6148;
                            let v16116 = Lanes([v15657[0], v15657[1], 0.0, 0.0, v15657[2]]);
                            let v6151 = ((v6148 * v665) - v6095) + v6095;
                            let v16118 = ((((((((Lanes([0.0, 0.0, v16100[0], 0.0, 0.0])) + (v16098 * v78)) + (((v16098 * v748) * v6139) + (v16098 * v6145))) - (v16098 * v6148)) / v6139) * v665) + (Lanes([0.0, 0.0, v16113[0], 0.0, 0.0]))) - v16116) + v16116;
                            let v6152 = v6151 / v6116;
                            let v16119 = v16065 * v6152;
                            let v16123 = ((v16118 - (Lanes([0.0, 0.0, v16119[0], 0.0, 0.0]))) / v6116) * v6152;
                            let v6155 = (v1 + (v6152 * v6152)).sqrt();
                            let v6156 = v6151 / v6155;
                            let v6159 = v127 * (v6100 - (v6156 - v6095));
                            let v16134 = ((Lanes([v15636[0], v15636[1], 0.0, v15636[2], 0.0])) - (((v16118 - (((v16123 + v16123) * (v9368 / (v10435 * v6155))) * v6156)) / v6155) - v16116)) * v127;
                            v6450 = v6159;
                            v6452 = v6159;
                            v6829 = v0;
                            v6839 = v0;
                            v6844 = v0;
                            v9939 = v16134;
                            v9940 = v16134;
                            v9941 = v10579;
                            v9942 = v10579;
                            v9943 = v10579;
                        } else {
                            let v6161 = v6100 + v6095;
                            let v15675 = (Lanes([v15636[0], v15636[1], v15636[2], 0.0])) + (Lanes([v15657[0], v15657[1], 0.0, v15657[2]]));
                            let v15676 = v10410 * v6161;
                            let v15677 = v15675 * v663;
                            let v15679 = Lanes([v15677[0], v15677[1], 0.0, v15677[2], v15677[3]]);
                            let v15680 = (Lanes([0.0, 0.0, v15676[0], 0.0, 0.0])) + v15679;
                            let v6163 = (v663 * v6161) - v1;
                            let v6166 = v6097 * v664;
                            let v15684 = (v15660 * v664) + (v10412 * v6097);
                            let v6167 = (v90 * (v6163 + v6160)) / v6166;
                            let v15685 = v15684 * v6167;
                            let v15688 = ((v15680 * v90) - (Lanes([0.0, 0.0, v15685[0], 0.0, 0.0]))) / v6166;
                            let v6168 = v1 + v6167;
                            let v6170 = if v6168 < v6169 { 1.0 } else { 0.0 };
                            let v6174: f64;
                            let v9945: Lanes<5>;
                            if v6170 != 0.0 {
                                v6174 = v6171;
                                v9945 = v10579;
                            } else {
                                v6174 = v6168;
                                v9945 = v15688;
                            }
                            let v6173 = (v6097 * v663) / v78;
                            let v15692 = ((v15660 * v663) + (v10410 * v6097)) / v78;
                            let v6175 = v6174.sqrt();
                            let v6176 = v1 - v6175;
                            let v15697 = v15692 * v6176;
                            let v15701 = Lanes([v15636[0], v15636[1], 0.0, v15636[2], 0.0]);
                            let v6179 = (v6100 + (v6173 * v6176)) + v6095;
                            let v15703 = Lanes([v15657[0], v15657[1], 0.0, 0.0, v15657[2]]);
                            let v15705 = v10410 * v6179;
                            let v6182 = (-(v663 * v6179)).exp();
                            let v6185 = (v90 * (v6163 + v6182)) / v6166;
                            let v15713 = v15684 * v6185;
                            let v15716 = (((v15680 + ((((Lanes([0.0, 0.0, v15705[0], 0.0, 0.0])) + (((v15701 + ((Lanes([0.0, 0.0, v15697[0], 0.0, 0.0])) + (((v9945 * (v9368 / (v10435 * v6175))) * v10390) * v6173))) + v15703) * v663)) * v10390) * v6182)) * v90) - (Lanes([0.0, 0.0, v15713[0], 0.0, 0.0]))) / v6166;
                            let v6186 = v1 + v6185;
                            let v6188 = if v6186 < v6187 { 1.0 } else { 0.0 };
                            let v6190: f64;
                            let v9946: Lanes<5>;
                            if v6188 != 0.0 {
                                v6190 = v6189;
                                v9946 = v10579;
                            } else {
                                v6190 = v6186;
                                v9946 = v15716;
                            }
                            let v6191 = v6190.sqrt();
                            let v6192 = v1 - v6191;
                            let v15721 = v15692 * v6192;
                            let v6195 = (v6100 + (v6173 * v6192)) + v6095;
                            let v6196 = v663 * v6195;
                            let v15727 = v10410 * v6195;
                            let v15730 = (Lanes([0.0, 0.0, v15727[0], 0.0, 0.0])) + (((v15701 + ((Lanes([0.0, 0.0, v15721[0], 0.0, 0.0])) + (((v9946 * (v9368 / (v10435 * v6191))) * v10390) * v6173))) + v15703) * v663);
                            let v6197 = if v6196 < v96 { 1.0 } else { 0.0 };
                            let v6274: f64;
                            let v9947: Lanes<5>;
                            if v6197 != 0.0 {
                                let v6200 = v663 * v6096;
                                let v6201 = v1 / v6200;
                                let v15736 = ((((v10410 * v6096) + (v15658 * v663)) * v6201) * v10390) / v6200;
                                let v6202 = v6199 + v6201;
                                let v15737 = v15675 * v10390;
                                let v6204 = (-v6161) / v6096;
                                let v15738 = v15658 * v6204;
                                let v15745 = ((v15736 * v6198) / v6207) * v10390;
                                let v6212 = (v6205 - ((v6198 * v6202) / v6207)) + (v6204 / v6210);
                                let v15748 = (Lanes([0.0, 0.0, v15745[0], 0.0, 0.0])) + ((((Lanes([v15737[0], v15737[1], 0.0, v15737[2], v15737[3]])) - (Lanes([0.0, 0.0, v15738[0], 0.0, 0.0]))) / v6096) / v6210);
                                let v6218 = ((v6213 * v6202) - v6215) / v6217;
                                let v15750 = (v15736 * v6213) / v6217;
                                let v15751 = v15748 * v6212;
                                let v6220 = v6218 * v6218;
                                let v15753 = v15750 * v6218;
                                let v15757 = ((v15753 + v15753) * v6218) + (v15750 * v6220);
                                let v6223 = ((v6212 * v6212) + (v6220 * v6218)).sqrt();
                                let v15762 = ((v15751 + v15751) + (Lanes([0.0, 0.0, v15757[0], 0.0, 0.0]))) * (v9368 / (v10435 * v6223));
                                let v6225 = (-v6212) + v6223;
                                let v6227 = v6212 + v6223;
                                let v6232 = ((v6225.powf(v1562)) + (-(v6227.powf(v1562)))) - v6231;
                                let v15777 = v10415 * v6232;
                                let v6235 = ((v6232 * v665) - v6095) + v6095;
                                let v6236 = v663 * v6235;
                                let v15782 = v10410 * v6235;
                                let v15785 = (Lanes([0.0, 0.0, v15782[0], 0.0, 0.0])) + (((((((((v15748 * v10390) + v15762) * (v1562 * (v6225.powf(v15765)))) + (((v15748 + v15762) * (v1562 * (v6227.powf(v15770)))) * v10390)) * v665) + (Lanes([0.0, 0.0, v15777[0], 0.0, 0.0]))) - v15703) + v15703) * v663);
                                v6274 = v6236;
                                v9947 = v15785;
                            } else {
                                v6274 = v6196;
                                v9947 = v15730;
                            }
                            let v6237 = v6161 + v79;
                            let v15786 = v10410 * v6105;
                            let v15787 = v15672 * v663;
                            let v6239 = (v663 * v6105).exp();
                            let v15791 = ((Lanes([0.0, 0.0, v15786[0], 0.0])) + (Lanes([v15787[0], v15787[1], 0.0, v15787[2]]))) * v6239;
                            let v6240 = v6239 + v362;
                            let v6241 = v731 / v489;
                            let v6242 = v6241 * v6241;
                            let v15793 = (v10453 / v489) * v6241;
                            let v15794 = v15793 + v15793;
                            let v6243 = v6242 * v6240;
                            let v15795 = v15794 * v6240;
                            let v15796 = v15791 * v6242;
                            let v6244 = v663 * v6237;
                            let v15799 = v10410 * v6237;
                            let v15801 = (Lanes([0.0, 0.0, v15799[0], 0.0, 0.0])) + v15679;
                            let v6245 = v6243 * v6166;
                            let v15803 = v15684 * v6243;
                            let v15805 = (((Lanes([0.0, 0.0, v15795[0], 0.0])) + v15796) * v6166) + (Lanes([0.0, 0.0, v15803[0], 0.0]));
                            let v15806 = v15801 * v6244;
                            let v6247 = v6245 + (v6244 * v6244);
                            let v15808 = Lanes([v15805[0], v15805[1], v15805[2], 0.0, v15805[3]]);
                            let v6249 = v6242 * v6166;
                            let v6250 = v6249.ln();
                            let v15816 = ((v15794 * v6166) + (v15684 * v6242)) * (v9368 / v6249);
                            let v15817 = Lanes([0.0, 0.0, v15816[0], 0.0, 0.0]);
                            let v6252 = v663 * v6095;
                            let v15819 = v10410 * v6095;
                            let v15820 = v15657 * v663;
                            let v15823 = (Lanes([0.0, 0.0, v15819[0], 0.0])) + (Lanes([v15820[0], v15820[1], 0.0, v15820[2]]));
                            let v15824 = Lanes([v15823[0], v15823[1], v15823[2], 0.0, v15823[3]]);
                            let v15826 = v15801 - ((((v15808 + (v15806 + v15806)) * (v9368 / v6247)) - v15817) + v15824);
                            let v6255 = (v6244 - (((v6247.ln()) - v6250) + v6252)) - v1;
                            let v6256 = v90 * v6244;
                            let v15827 = v15801 * v90;
                            let v6257 = if v6256 > v0 { 1.0 } else { 0.0 };
                            let v6259: f64;
                            let v9948: Lanes<5>;
                            if v6257 != 0.0 {
                                v6259 = v6256;
                                v9948 = v15827;
                            } else {
                                let v6258 = -v6256;
                                let v15828 = v15827 * v10390;
                                v6259 = v6258;
                                v9948 = v15828;
                            }
                            let v15829 = v15826 * v6255;
                            let v6262 = ((v6255 * v6255) + v6259).sqrt();
                            let v15839 = v10410 * v79;
                            let v6268 = (v6244 - (v6244 - (v13 * (v6255 + v6262)))) + (v663 * v79);
                            let v15842 = ((v15801 - (v15801 - ((v15826 + (((v15829 + v15829) + v9948) * (v9368 / (v10435 * v6262)))) * v13))) + (Lanes([0.0, 0.0, v15839[0], 0.0, 0.0]))) * v6268;
                            let v6270 = v6245 + (v6268 * v6268);
                            let v6273 = ((v6270.ln()) - v6250) + v6252;
                            let v15848 = (((v15808 + (v15842 + v15842)) * (v9368 / v6270)) - v15817) + v15824;
                            let v15849 = v15848 - v9947;
                            let v6277 = (v6273 - v6274) - v6276;
                            let v6280 = (v90 * v6273) * v6279;
                            let v15851 = (v15848 * v90) * v6279;
                            let v6281 = if v6280 > v0 { 1.0 } else { 0.0 };
                            let v6283: f64;
                            let v9949: Lanes<5>;
                            if v6281 != 0.0 {
                                v6283 = v6280;
                                v9949 = v15851;
                            } else {
                                let v6282 = -v6280;
                                let v15852 = v15851 * v10390;
                                v6283 = v6282;
                                v9949 = v15852;
                            }
                            let v15853 = v15849 * v6277;
                            let v6286 = ((v6277 * v6277) + v6283).sqrt();
                            let v6289 = v6273 - (v13 * (v6277 + v6286));
                            let v15861 = v15848 - ((v15849 + (((v15853 + v15853) + v9949) * (v9368 / (v10435 * v6286)))) * v13);
                            let v6290 = v6289 / v663;
                            let v15862 = v10410 * v6290;
                            let v6291 = v6290 - v6095;
                            let v15866 = ((v15861 - (Lanes([0.0, 0.0, v15862[0], 0.0, 0.0]))) / v663) - v15703;
                            let v6294 = (-v6289).exp();
                            let v6295 = (v6289 - v1) + v6294;
                            let v15869 = v15861 + ((v15861 * v10390) * v6294);
                            let v6297 = if v6295 < v6296 { 1.0 } else { 0.0 };
                            let v6299: f64;
                            let v9950: Lanes<5>;
                            if v6297 != 0.0 {
                                v6299 = v6298;
                                v9950 = v10579;
                            } else {
                                v6299 = v6295;
                                v9950 = v15869;
                            }
                            let v6300 = v6299.sqrt();
                            let v6301 = v6043 * v6300;
                            let v15873 = v15614 * v6300;
                            let v15876 = (Lanes([0.0, 0.0, v15873[0], 0.0, 0.0])) + ((v9950 * (v9368 / (v10435 * v6300))) * v6043);
                            let v6303 = v127 * (v6100 - v6291);
                            let v15878 = (v15701 - v15866) * v127;
                            let v6305 = if v6304 == v1 { 1.0 } else { 0.0 };
                            let v6451: f64;
                            let v6453: f64;
                            let v6830: f64;
                            let v6840: f64;
                            let v6845: f64;
                            let v9951: Lanes<5>;
                            let v9952: Lanes<5>;
                            let v9953: Lanes<5>;
                            let v9954: Lanes<5>;
                            let v9955: Lanes<5>;
                            if v6305 != 0.0 {
                                let v6306 = v6242 * v6239;
                                let v15879 = v15794 * v6239;
                                let v15881 = (Lanes([0.0, 0.0, v15879[0], 0.0])) + v15796;
                                let mut v6307: f64 = 0.0;
                                let mut v6310: f64 = 0.0;
                                let mut v6401: f64 = 0.0;
                                let mut v6431: f64 = 0.0;
                                let mut v6434: f64 = 0.0;
                                let mut v6442: f64 = 0.0;
                                let mut v6445: f64 = 0.0;
                                let mut v9956: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9957: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9958: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9959: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9960: Lanes<5> = Lanes([0.0; 5]);
                                v6307 = v1;
                                v6310 = v6291;
                                v6401 = v0;
                                v6431 = v6289;
                                v6434 = v0;
                                v6442 = v0;
                                v6445 = v0;
                                v9956 = v15866;
                                v9957 = v15861;
                                v9958 = v10579;
                                v9959 = v10579;
                                v9960 = v10579;
                                loop {
                                    let v6309 = if v6307 <= v6308 { 1.0 } else { 0.0 };
                                    if v6309 == 0.0 {
                                        break;
                                    }
                                    let v6311 = v6310 + v6095;
                                    let v6312 = v663 * v6311;
                                    let v15902 = v10410 * v6311;
                                    let v15905 = (Lanes([0.0, 0.0, v15902[0], 0.0, 0.0])) + ((v9956 + v15703) * v663);
                                    let v6313 = if v6312 < v644 { 1.0 } else { 0.0 };
                                    let v6394: f64;
                                    let v6398: f64;
                                    let v6435: f64;
                                    let v6446: f64;
                                    let v9961: Lanes<5>;
                                    let v9962: Lanes<5>;
                                    let v9963: Lanes<5>;
                                    let v9964: Lanes<5>;
                                    if v6313 != 0.0 {
                                        let v6314 = v6312 * v6312;
                                        let v15947 = v15905 * v6312;
                                        let v15948 = v15947 + v15947;
                                        let v6315 = v6314 * v6312;
                                        let v6320 = v6317 + (v6312 * v6318);
                                        let v6322 = v6316 + (v6312 * v6320);
                                        let v6323 = v6315 * v6322;
                                        let v15958 = (((v15948 * v6312) + (v15905 * v6314)) * v6322) + (((v15905 * v6320) + ((v15905 * v6318) * v6312)) * v6315);
                                        let v6326 = v6312 * v644;
                                        let v15959 = v15905 * v644;
                                        let v6328 = v6325 + (v6326 * v6318);
                                        let v6330 = v6324 + (v6312 * v6328);
                                        let v6331 = v6314 * v6330;
                                        let v6332 = v6306 * v6323;
                                        let v15967 = v15881 * v6323;
                                        let v6333 = v6332 * v6323;
                                        let v15973 = (((Lanes([v15967[0], v15967[1], v15967[2], 0.0, v15967[3]])) + (v15958 * v6306)) * v6323) + (v15958 * v6332);
                                        let v15975 = v10410 * v6306;
                                        let v6335 = (v6306 * v663) * v78;
                                        let v6336 = v6335 * v6323;
                                        let v15979 = (((v15881 * v663) + (Lanes([0.0, 0.0, v15975[0], 0.0]))) * v78) * v6323;
                                        let v6344 = v6341 + (v6312 * v6342);
                                        let v6346 = v6340 + (v6312 * v6344);
                                        let v6348 = v6339 + (v6312 * v6346);
                                        let v6350 = v6338 + (v6312 * v6348);
                                        let v6351 = v6312 * v6350;
                                        let v15998 = (v15905 * v6350) + (((v15905 * v6348) + (((v15905 * v6346) + (((v15905 * v6344) + ((v15905 * v6342) * v6312)) * v6312)) * v6312)) * v6312);
                                        let v6356 = v6354 + (v6326 * v6342);
                                        let v6358 = v6353 + (v6312 * v6356);
                                        let v6360 = v6352 + (v6312 * v6358);
                                        let v6362 = v6338 + (v6312 * v6360);
                                        let v16009 = v15998 * v6351;
                                        let v6366 = (((v6351 * v6351) + v6333) + v362).sqrt();
                                        let v16014 = ((v16009 + v16009) + v15973) * (v9368 / (v10435 * v6366));
                                        let v16015 = v10410 * v6362;
                                        let v6368 = (v663 * v6362) * v78;
                                        let v6371 = v6366 + v6366;
                                        let v6372 = ((v6368 * v6351) + (v6336 * v6331)) / v6371;
                                        let v16027 = (((((((Lanes([0.0, 0.0, v16015[0], 0.0, 0.0])) + (((v15905 * v6360) + (((v15905 * v6358) + (((v15905 * v6356) + ((v15959 * v6342) * v6312)) * v6312)) * v6312)) * v663)) * v78) * v6351) + (v15998 * v6368)) + ((((Lanes([v15979[0], v15979[1], v15979[2], 0.0, v15979[3]])) + (v15958 * v6335)) * v6331) + (((v15948 * v6330) + (((v15905 * v6328) + ((v15959 * v6318) * v6312)) * v6314)) * v6336))) - ((v16014 + v16014) * v6372)) / v6371;
                                        v6394 = v6366;
                                        v6398 = v6372;
                                        v6435 = v6351;
                                        v6446 = v6333;
                                        v9961 = v16014;
                                        v9962 = v16027;
                                        v9963 = v15998;
                                        v9964 = v15973;
                                    } else {
                                        let v6373 = if v6312 < v2535 { 1.0 } else { 0.0 };
                                        let v6386: f64;
                                        let v6389: f64;
                                        let v9965: Lanes<5>;
                                        let v9966: Lanes<5>;
                                        if v6373 != 0.0 {
                                            let v6374 = v6312.exp();
                                            let v15924 = v15905 * v6374;
                                            let v6375 = v6374 - v1;
                                            let v6376 = v6306 * v6375;
                                            let v15925 = v15881 * v6375;
                                            let v15928 = (Lanes([v15925[0], v15925[1], v15925[2], 0.0, v15925[3]])) + (v15924 * v6306);
                                            let v6377 = v6306 * v663;
                                            let v15930 = v10410 * v6306;
                                            let v6378 = v6377 * v6374;
                                            let v15933 = ((v15881 * v663) + (Lanes([0.0, 0.0, v15930[0], 0.0]))) * v6374;
                                            let v15936 = (Lanes([v15933[0], v15933[1], v15933[2], 0.0, v15933[3]])) + (v15924 * v6377);
                                            v6386 = v6376;
                                            v6389 = v6378;
                                            v9965 = v15928;
                                            v9966 = v15936;
                                        } else {
                                            let v15906 = v10410 * v6310;
                                            let v6380 = (v663 * v6310).exp();
                                            let v15910 = ((Lanes([0.0, 0.0, v15906[0], 0.0, 0.0])) + (v9956 * v663)) * v6380;
                                            let v6381 = v6380 - v6239;
                                            let v6382 = v6242 * v6381;
                                            let v15913 = v15794 * v6381;
                                            let v15916 = (Lanes([0.0, 0.0, v15913[0], 0.0, 0.0])) + ((v15910 - (Lanes([v15791[0], v15791[1], v15791[2], 0.0, v15791[3]]))) * v6242);
                                            let v6383 = v6242 * v663;
                                            let v6384 = v6383 * v6380;
                                            let v15920 = ((v15794 * v663) + (v10410 * v6242)) * v6380;
                                            let v15923 = (Lanes([0.0, 0.0, v15920[0], 0.0, 0.0])) + (v15910 * v6383);
                                            v6386 = v6382;
                                            v6389 = v6384;
                                            v9965 = v15916;
                                            v9966 = v15923;
                                        }
                                        let v6388 = ((v6312 - v1) + v6386).sqrt();
                                        let v15940 = (v15905 + v9965) * (v9368 / (v10435 * v6388));
                                        let v6391 = (v663 + v6389) / v6388;
                                        let v6392 = v6391 * v13;
                                        let v15946 = ((((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + v9966) - (v15940 * v6391)) / v6388) * v13;
                                        v6394 = v6388;
                                        v6398 = v6392;
                                        v6435 = v0;
                                        v6446 = v6386;
                                        v9961 = v15940;
                                        v9962 = v15946;
                                        v9963 = v10579;
                                        v9964 = v9965;
                                    }
                                    let v16029 = v15658 * v6394;
                                    let v6396 = (v6100 - v6310) - (v6096 * v6394);
                                    let v16033 = (v15701 - v9956) - ((Lanes([0.0, 0.0, v16029[0], 0.0, 0.0])) + (v9961 * v6096));
                                    let v16034 = v15658 * v6398;
                                    let v6400 = v6397 - (v6096 * v6398);
                                    let v16038 = ((Lanes([0.0, 0.0, v16034[0], 0.0, 0.0])) + (v9962 * v6096)) * v10390;
                                    let v6402 = if v6401 == v1 { 1.0 } else { 0.0 };
                                    let v6425: f64;
                                    let v6427: f64;
                                    let v6428: f64;
                                    let v9967: Lanes<5>;
                                    if v6402 != 0.0 {
                                        v6425 = v6403;
                                        v6427 = v6310;
                                        v6428 = v6401;
                                        v9967 = v9956;
                                    } else {
                                        let v6405 = (-v6396) / v6400;
                                        let v16042 = ((v16033 * v10390) - (v16038 * v6405)) / v6400;
                                        let v6407 = v6310.abs();
                                        let v16046 = v9956 * ((v10435 * (if v6310 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                                        let v6408 = if v1 >= v6407 { 1.0 } else { 0.0 };
                                        let v6409: f64;
                                        let v9968: Lanes<5>;
                                        if v6408 != 0.0 {
                                            v6409 = v1;
                                            v9968 = v10579;
                                        } else {
                                            v6409 = v6407;
                                            v9968 = v16046;
                                        }
                                        let v6411 = v6406 * (v1 + v6409);
                                        let v16047 = v9968 * v6406;
                                        let v6413 = if (v6405.abs()) > v6411 { 1.0 } else { 0.0 };
                                        let v6418: f64;
                                        let v9969: Lanes<5>;
                                        if v6413 != 0.0 {
                                            let v6414 = if v6405 >= v0 { 1.0 } else { 0.0 };
                                            let v6416: f64;
                                            if v6414 != 0.0 {
                                                v6416 = v1;
                                            } else {
                                                v6416 = v6415;
                                            }
                                            let v6417 = v6411 * v6416;
                                            let v16048 = v16047 * v6416;
                                            v6418 = v6417;
                                            v9969 = v16048;
                                        } else {
                                            v6418 = v6405;
                                            v9969 = v16042;
                                        }
                                        let v6419 = v6310 + v6418;
                                        let v16049 = v9956 + v9969;
                                        let v6424 = if (if (v6418.abs()) <= v861 { 1.0 } else { 0.0 }) != 0.0 && (if (v6396.abs()) <= v3506 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v6429: f64;
                                        if v6424 != 0.0 {
                                            v6429 = v1;
                                        } else {
                                            v6429 = v6401;
                                        }
                                        v6425 = v6307;
                                        v6427 = v6419;
                                        v6428 = v6429;
                                        v9967 = v16049;
                                    }
                                    let v6426 = v6425 + v1;
                                    v6307 = v6426;
                                    v6310 = v6427;
                                    v6401 = v6428;
                                    v6431 = v6312;
                                    v6434 = v6435;
                                    v6442 = v6394;
                                    v6445 = v6446;
                                    v9956 = v9967;
                                    v9957 = v15905;
                                    v9958 = v9963;
                                    v9959 = v9961;
                                    v9960 = v9964;
                                }
                                let v6430 = if v6401 == v0 { 1.0 } else { 0.0 };
                                if v6430 != 0.0 {
                                } else {
                                }
                                let v6432 = if v6431 < v644 { 1.0 } else { 0.0 };
                                let v6440: f64;
                                let v9970: Lanes<5>;
                                if v6432 != 0.0 {
                                    let v6433 = if v6431 < v96 { 1.0 } else { 0.0 };
                                    if v6433 != 0.0 {
                                    } else {
                                    }
                                    let v6437 = v6434 + v6436;
                                    v6440 = v6437;
                                    v9970 = v9958;
                                } else {
                                    let v6439 = (v6431 - v1).sqrt();
                                    let v15884 = v9957 * (v9368 / (v10435 * v6439));
                                    v6440 = v6439;
                                    v9970 = v15884;
                                }
                                let v6441 = v6043 * v6440;
                                let v15885 = v15614 * v6440;
                                let v15888 = (Lanes([0.0, 0.0, v15885[0], 0.0, 0.0])) + (v9970 * v6043);
                                let v6443 = v6442 + v6440;
                                let v6444 = v1 / v6443;
                                let v6447 = v6043 * v6445;
                                let v15893 = v15614 * v6445;
                                let v6449 = v6441 + (v6447 * v6444);
                                let v15900 = v15888 + ((((Lanes([0.0, 0.0, v15893[0], 0.0, 0.0])) + (v9960 * v6043)) * v6444) + (((((v9959 + v9970) * v6444) * v10390) / v6443) * v6447));
                                v6451 = v6449;
                                v6453 = v6441;
                                v6830 = v6434;
                                v6840 = v6442;
                                v6845 = v6445;
                                v9951 = v15900;
                                v9952 = v15888;
                                v9953 = v9958;
                                v9954 = v9959;
                                v9955 = v9960;
                            } else {
                                v6451 = v6303;
                                v6453 = v6301;
                                v6830 = v0;
                                v6840 = v0;
                                v6845 = v0;
                                v9951 = v15878;
                                v9952 = v15876;
                                v9953 = v10579;
                                v9954 = v10579;
                                v9955 = v10579;
                            }
                            v6450 = v6451;
                            v6452 = v6453;
                            v6829 = v6830;
                            v6839 = v6840;
                            v6844 = v6845;
                            v9939 = v9951;
                            v9940 = v9952;
                            v9941 = v9953;
                            v9942 = v9954;
                            v9943 = v9955;
                        }
                        let v6454 = v6450 - v6452;
                        let v16135 = v9939 - v9940;
                        let v8544: f64;
                        let v8552: f64;
                        let v8559: f64;
                        let v8571: f64;
                        let v8584: f64;
                        let v8590: f64;
                        let v8601: f64;
                        let v8607: f64;
                        let v9971: Lanes<5>;
                        let v9972: Lanes<5>;
                        let v9973: Lanes<6>;
                        let v9974: Lanes<6>;
                        let v9975: Lanes<5>;
                        let v9976: Lanes<6>;
                        let v9977: Lanes<5>;
                        let v9978: Lanes<6>;
                        if v6455 != 0.0 {
                            let v8545: f64;
                            let v8602: f64;
                            let v9979: Lanes<5>;
                            let v9980: Lanes<5>;
                            if v6044 != 0.0 {
                                let v6458 = -v6456;
                                let v6459 = v6458 * v6450;
                                let v16144 = v9939 * v6458;
                                let v6460 = v6458 * v6454;
                                let v16145 = v16135 * v6458;
                                v8545 = v6459;
                                v8602 = v6460;
                                v9979 = v16144;
                                v9980 = v16145;
                            } else {
                                v8545 = v0;
                                v8602 = v0;
                                v9979 = v10579;
                                v9980 = v10579;
                            }
                            let v8553: f64;
                            let v8585: f64;
                            let v9981: Lanes<5>;
                            let v9982: Lanes<5>;
                            if v6045 != 0.0 {
                                let v6461 = -v6456;
                                let v6462 = v6461 * v6450;
                                let v16146 = v9939 * v6461;
                                let v6463 = v6461 * v6454;
                                let v16147 = v16135 * v6461;
                                v8553 = v6462;
                                v8585 = v6463;
                                v9981 = v16146;
                                v9982 = v16147;
                            } else {
                                v8553 = v0;
                                v8585 = v0;
                                v9981 = v10579;
                                v9982 = v10579;
                            }
                            v8544 = v8545;
                            v8552 = v8553;
                            v8559 = v6030;
                            v8571 = v6029;
                            v8584 = v8585;
                            v8590 = v6027;
                            v8601 = v8602;
                            v8607 = v6028;
                            v9971 = v9979;
                            v9972 = v9981;
                            v9973 = v15613;
                            v9974 = v15612;
                            v9975 = v9982;
                            v9976 = v15610;
                            v9977 = v9980;
                            v9978 = v15611;
                        } else {
                            let v8560: f64;
                            let v8572: f64;
                            let v8591: f64;
                            let v8608: f64;
                            let v9983: Lanes<6>;
                            let v9984: Lanes<6>;
                            let v9985: Lanes<6>;
                            let v9986: Lanes<6>;
                            if v6464 != 0.0 {
                                let v8561: f64;
                                let v8609: f64;
                                let v9987: Lanes<6>;
                                let v9988: Lanes<6>;
                                if v6044 != 0.0 {
                                    let v6465 = -v6456;
                                    let v6466 = v6465 * v6450;
                                    let v16136 = v9939 * v6465;
                                    let v6467 = v6465 * v6454;
                                    let v16137 = v16135 * v6465;
                                    let v16138 = Lanes([v16136[0], v16136[1], v16136[2], v16136[3], v16136[4], 0.0]);
                                    let v16139 = Lanes([v16137[0], v16137[1], v16137[2], v16137[3], v16137[4], 0.0]);
                                    v8561 = v6466;
                                    v8609 = v6467;
                                    v9987 = v16138;
                                    v9988 = v16139;
                                } else {
                                    v8561 = v6030;
                                    v8609 = v6028;
                                    v9987 = v15613;
                                    v9988 = v15611;
                                }
                                let v8573: f64;
                                let v8592: f64;
                                let v9989: Lanes<6>;
                                let v9990: Lanes<6>;
                                if v6045 != 0.0 {
                                    let v6468 = -v6456;
                                    let v6469 = v6468 * v6450;
                                    let v16140 = v9939 * v6468;
                                    let v6470 = v6468 * v6454;
                                    let v16141 = v16135 * v6468;
                                    let v16142 = Lanes([v16140[0], v16140[1], v16140[2], v16140[3], v16140[4], 0.0]);
                                    let v16143 = Lanes([v16141[0], v16141[1], v16141[2], v16141[3], v16141[4], 0.0]);
                                    v8573 = v6469;
                                    v8592 = v6470;
                                    v9989 = v16142;
                                    v9990 = v16143;
                                } else {
                                    v8573 = v6029;
                                    v8592 = v6027;
                                    v9989 = v15612;
                                    v9990 = v15610;
                                }
                                v8560 = v8561;
                                v8572 = v8573;
                                v8591 = v8592;
                                v8608 = v8609;
                                v9983 = v9987;
                                v9984 = v9989;
                                v9985 = v9990;
                                v9986 = v9988;
                            } else {
                                v8560 = v6030;
                                v8572 = v6029;
                                v8591 = v6027;
                                v8608 = v6028;
                                v9983 = v15613;
                                v9984 = v15612;
                                v9985 = v15610;
                                v9986 = v15611;
                            }
                            v8544 = v0;
                            v8552 = v0;
                            v8559 = v8560;
                            v8571 = v8572;
                            v8584 = v0;
                            v8590 = v8591;
                            v8601 = v0;
                            v8607 = v8608;
                            v9971 = v10579;
                            v9972 = v10579;
                            v9973 = v9983;
                            v9974 = v9984;
                            v9975 = v10579;
                            v9976 = v9985;
                            v9977 = v10579;
                            v9978 = v9986;
                        }
                        let v6474 = (v6471 * v6046) + v6048;
                        let v6476 = (v6471 * v6048) + v6046;
                        let v6479 = (v6474 * v6059) + (v6476 * v6062);
                        let v16150 = (v15627 * v6474) + (v15630 * v6476);
                        let v6485 = -(((v6474 * v6064) + (v6476 * v6063)) + v6483);
                        let v16154 = ((v15633 * v6474) + (v15632 * v6476)) * v10390;
                        let v6486 = if v6485 > v783 { 1.0 } else { 0.0 };
                        let v6501: f64;
                        let v9991: Lanes<3>;
                        if v6486 != 0.0 {
                            let v6488 = v779 - v783;
                            let v6489 = (v6485 - v783) / v6488;
                            let v16155 = v16154 / v6488;
                            let v6490 = v6489 * v6489;
                            let v16156 = v16155 * v6489;
                            let v16157 = v16156 + v16156;
                            let v16161 = v16157 * v6490;
                            let v6496 = (((v1 + v6489) + v6490) + (v6490 * v6489)) + (v6490 * v6490);
                            let v6497 = v1 / v6496;
                            let v16170 = (((((((v16155 + v16157) + ((v16157 * v6489) + (v16155 * v6490))) + (v16161 + v16161)) * v6497) * v10390) / v6496) * v10390) * v6488;
                            let v6500 = v783 + (v6488 * (v1 - v6497));
                            v6501 = v6500;
                            v9991 = v16170;
                        } else {
                            v6501 = v6485;
                            v9991 = v16154;
                        }
                        let v16171 = v9991 * v10390;
                        let v6503 = (-v6501) - v11;
                        let v6504 = v6479 - v6098;
                        let v6505 = -v6503;
                        let v16172 = v16171 * v10390;
                        let v6506 = if v6504 < v6505 { 1.0 } else { 0.0 };
                        let v6850: f64;
                        let v6852: f64;
                        let v9992: Lanes<5>;
                        let v9993: Lanes<5>;
                        if v6506 != 0.0 {
                            let v6507 = v663 * v6043;
                            let v6508 = v1 / v6507;
                            let v6509 = v6508 * v127;
                            let v16556 = (((((v10410 * v6043) + (v15614 * v663)) * v6508) * v10390) / v6507) * v127;
                            let v16557 = v16556 * v6510;
                            let v6512 = v78 + (v6510 * v6509);
                            let v6513 = v91 * v6512;
                            let v6514 = v6513 * v6512;
                            let v6515 = v6514 * v6512;
                            let v16564 = ((((v16557 * v91) * v6512) + (v16557 * v6513)) * v6512) + (v16557 * v6514);
                            let v6516 = v661 - v6104;
                            let v16565 = v10406 - v15671;
                            let v6517 = v6504 + v6503;
                            let v16569 = v10410 * v6517;
                            let v16570 = ((Lanes([v16150[0], v16150[1], v16150[2], 0.0])) + (Lanes([v16171[0], v16171[1], 0.0, v16171[2]]))) * v663;
                            let v6520 = v3500 * v6509;
                            let v6521 = (v663 * v6517) - v78;
                            let v6522 = v6520 * v6521;
                            let v16575 = (v16556 * v3500) * v6521;
                            let v16578 = (Lanes([0.0, 0.0, v16575[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v16569[0], 0.0, 0.0])) + (Lanes([v16570[0], v16570[1], 0.0, v16570[2], v16570[3]]))) * v6520);
                            let v6523 = v6519 - v6522;
                            let v16579 = v16578 * v10390;
                            let v6524 = v6523 * v6523;
                            let v16580 = v16579 * v6523;
                            let v16581 = v16580 + v16580;
                            let v6526 = if v6515 < (v6524 * v3506) { 1.0 } else { 0.0 };
                            let v6538: f64;
                            let v9994: Lanes<5>;
                            if v6526 != 0.0 {
                                let v16588 = v16564 * v13;
                                let v6530 = (v13 * v6515) / v6523;
                                let v6532 = ((v6527 + v6523) + v6530) + v6522;
                                let v16594 = (v16579 + (((Lanes([0.0, 0.0, v16588[0], 0.0, 0.0])) - (v16579 * v6530)) / v6523)) + v16578;
                                v6538 = v6532;
                                v9994 = v16594;
                            } else {
                                let v6534 = (v6515 + v6524).sqrt();
                                let v6537 = (v6535 + v6534) + v6522;
                                let v16587 = (((Lanes([0.0, 0.0, v16564[0], 0.0, 0.0])) + v16581) * (v9368 / (v10435 * v6534))) + v16578;
                                v6538 = v6537;
                                v9994 = v16587;
                            }
                            let v6539 = v6538.powf(v1562);
                            let v16598 = v9994 * (v1562 * (v6538.powf(v16595)));
                            let v16600 = (v16556 * v3523) * v10390;
                            let v6545 = v748 * v6539;
                            let v6548 = (((v6540 - (v3523 * v6509)) + (v78 * v6539)) + (v6545 * v6539)) / v6539;
                            let v16613 = v10415 * v6548;
                            let v16616 = Lanes([v16171[0], v16171[1], 0.0, 0.0, v16171[2]]);
                            let v6551 = ((v6548 * v665) - v6503) + v6503;
                            let v16618 = ((((((((Lanes([0.0, 0.0, v16600[0], 0.0, 0.0])) + (v16598 * v78)) + (((v16598 * v748) * v6539) + (v16598 * v6545))) - (v16598 * v6548)) / v6539) * v665) + (Lanes([0.0, 0.0, v16613[0], 0.0, 0.0]))) - v16616) + v16616;
                            let v6552 = v6551 / v6516;
                            let v16619 = v16565 * v6552;
                            let v16623 = ((v16618 - (Lanes([0.0, 0.0, v16619[0], 0.0, 0.0]))) / v6516) * v6552;
                            let v6555 = (v1 + (v6552 * v6552)).sqrt();
                            let v6556 = v6551 / v6555;
                            let v6559 = v127 * (v6504 - (v6556 - v6503));
                            let v16634 = ((Lanes([v16150[0], v16150[1], 0.0, v16150[2], 0.0])) - (((v16618 - (((v16623 + v16623) * (v9368 / (v10435 * v6555))) * v6556)) / v6555) - v16616)) * v127;
                            v6850 = v6559;
                            v6852 = v6559;
                            v9992 = v16634;
                            v9993 = v16634;
                        } else {
                            let v6561 = v6504 + v6503;
                            let v16175 = (Lanes([v16150[0], v16150[1], v16150[2], 0.0])) + (Lanes([v16171[0], v16171[1], 0.0, v16171[2]]));
                            let v16176 = v10410 * v6561;
                            let v16177 = v16175 * v663;
                            let v16179 = Lanes([v16177[0], v16177[1], 0.0, v16177[2], v16177[3]]);
                            let v16180 = (Lanes([0.0, 0.0, v16176[0], 0.0, 0.0])) + v16179;
                            let v6563 = (v663 * v6561) - v1;
                            let v6566 = v6097 * v664;
                            let v16184 = (v15660 * v664) + (v10412 * v6097);
                            let v6567 = (v90 * (v6563 + v6560)) / v6566;
                            let v16185 = v16184 * v6567;
                            let v16188 = ((v16180 * v90) - (Lanes([0.0, 0.0, v16185[0], 0.0, 0.0]))) / v6566;
                            let v6568 = v1 + v6567;
                            let v6570 = if v6568 < v6569 { 1.0 } else { 0.0 };
                            let v6574: f64;
                            let v9995: Lanes<5>;
                            if v6570 != 0.0 {
                                v6574 = v6571;
                                v9995 = v10579;
                            } else {
                                v6574 = v6568;
                                v9995 = v16188;
                            }
                            let v6573 = (v6097 * v663) / v78;
                            let v16192 = ((v15660 * v663) + (v10410 * v6097)) / v78;
                            let v6575 = v6574.sqrt();
                            let v6576 = v1 - v6575;
                            let v16197 = v16192 * v6576;
                            let v16201 = Lanes([v16150[0], v16150[1], 0.0, v16150[2], 0.0]);
                            let v6579 = (v6504 + (v6573 * v6576)) + v6503;
                            let v16203 = Lanes([v16171[0], v16171[1], 0.0, 0.0, v16171[2]]);
                            let v16205 = v10410 * v6579;
                            let v6582 = (-(v663 * v6579)).exp();
                            let v6585 = (v90 * (v6563 + v6582)) / v6566;
                            let v16213 = v16184 * v6585;
                            let v16216 = (((v16180 + ((((Lanes([0.0, 0.0, v16205[0], 0.0, 0.0])) + (((v16201 + ((Lanes([0.0, 0.0, v16197[0], 0.0, 0.0])) + (((v9995 * (v9368 / (v10435 * v6575))) * v10390) * v6573))) + v16203) * v663)) * v10390) * v6582)) * v90) - (Lanes([0.0, 0.0, v16213[0], 0.0, 0.0]))) / v6566;
                            let v6586 = v1 + v6585;
                            let v6588 = if v6586 < v6587 { 1.0 } else { 0.0 };
                            let v6590: f64;
                            let v9996: Lanes<5>;
                            if v6588 != 0.0 {
                                v6590 = v6589;
                                v9996 = v10579;
                            } else {
                                v6590 = v6586;
                                v9996 = v16216;
                            }
                            let v6591 = v6590.sqrt();
                            let v6592 = v1 - v6591;
                            let v16221 = v16192 * v6592;
                            let v6595 = (v6504 + (v6573 * v6592)) + v6503;
                            let v6596 = v663 * v6595;
                            let v16227 = v10410 * v6595;
                            let v16230 = (Lanes([0.0, 0.0, v16227[0], 0.0, 0.0])) + (((v16201 + ((Lanes([0.0, 0.0, v16221[0], 0.0, 0.0])) + (((v9996 * (v9368 / (v10435 * v6591))) * v10390) * v6573))) + v16203) * v663);
                            let v6597 = if v6596 < v96 { 1.0 } else { 0.0 };
                            let v6674: f64;
                            let v9997: Lanes<5>;
                            if v6597 != 0.0 {
                                let v6600 = v663 * v6096;
                                let v6601 = v1 / v6600;
                                let v16236 = ((((v10410 * v6096) + (v15658 * v663)) * v6601) * v10390) / v6600;
                                let v6602 = v6599 + v6601;
                                let v16237 = v16175 * v10390;
                                let v6604 = (-v6561) / v6096;
                                let v16238 = v15658 * v6604;
                                let v16245 = ((v16236 * v6598) / v6607) * v10390;
                                let v6612 = (v6605 - ((v6598 * v6602) / v6607)) + (v6604 / v6610);
                                let v16248 = (Lanes([0.0, 0.0, v16245[0], 0.0, 0.0])) + ((((Lanes([v16237[0], v16237[1], 0.0, v16237[2], v16237[3]])) - (Lanes([0.0, 0.0, v16238[0], 0.0, 0.0]))) / v6096) / v6610);
                                let v6618 = ((v6613 * v6602) - v6615) / v6617;
                                let v16250 = (v16236 * v6613) / v6617;
                                let v16251 = v16248 * v6612;
                                let v6620 = v6618 * v6618;
                                let v16253 = v16250 * v6618;
                                let v16257 = ((v16253 + v16253) * v6618) + (v16250 * v6620);
                                let v6623 = ((v6612 * v6612) + (v6620 * v6618)).sqrt();
                                let v16262 = ((v16251 + v16251) + (Lanes([0.0, 0.0, v16257[0], 0.0, 0.0]))) * (v9368 / (v10435 * v6623));
                                let v6625 = (-v6612) + v6623;
                                let v6627 = v6612 + v6623;
                                let v6632 = ((v6625.powf(v1562)) + (-(v6627.powf(v1562)))) - v6631;
                                let v16277 = v10415 * v6632;
                                let v6635 = ((v6632 * v665) - v6503) + v6503;
                                let v6636 = v663 * v6635;
                                let v16282 = v10410 * v6635;
                                let v16285 = (Lanes([0.0, 0.0, v16282[0], 0.0, 0.0])) + (((((((((v16248 * v10390) + v16262) * (v1562 * (v6625.powf(v16265)))) + (((v16248 + v16262) * (v1562 * (v6627.powf(v16270)))) * v10390)) * v665) + (Lanes([0.0, 0.0, v16277[0], 0.0, 0.0]))) - v16203) + v16203) * v663);
                                v6674 = v6636;
                                v9997 = v16285;
                            } else {
                                v6674 = v6596;
                                v9997 = v16230;
                            }
                            let v6637 = v6561 + v79;
                            let v16286 = v10410 * v6505;
                            let v16287 = v16172 * v663;
                            let v6639 = (v663 * v6505).exp();
                            let v16291 = ((Lanes([0.0, 0.0, v16286[0], 0.0])) + (Lanes([v16287[0], v16287[1], 0.0, v16287[2]]))) * v6639;
                            let v6640 = v6639 + v362;
                            let v6641 = v731 / v489;
                            let v6642 = v6641 * v6641;
                            let v16293 = (v10453 / v489) * v6641;
                            let v16294 = v16293 + v16293;
                            let v6643 = v6642 * v6640;
                            let v16295 = v16294 * v6640;
                            let v16296 = v16291 * v6642;
                            let v6644 = v663 * v6637;
                            let v16299 = v10410 * v6637;
                            let v16301 = (Lanes([0.0, 0.0, v16299[0], 0.0, 0.0])) + v16179;
                            let v6645 = v6643 * v6566;
                            let v16303 = v16184 * v6643;
                            let v16305 = (((Lanes([0.0, 0.0, v16295[0], 0.0])) + v16296) * v6566) + (Lanes([0.0, 0.0, v16303[0], 0.0]));
                            let v16306 = v16301 * v6644;
                            let v6647 = v6645 + (v6644 * v6644);
                            let v16308 = Lanes([v16305[0], v16305[1], v16305[2], 0.0, v16305[3]]);
                            let v6649 = v6642 * v6566;
                            let v6650 = v6649.ln();
                            let v16316 = ((v16294 * v6566) + (v16184 * v6642)) * (v9368 / v6649);
                            let v16317 = Lanes([0.0, 0.0, v16316[0], 0.0, 0.0]);
                            let v6652 = v663 * v6503;
                            let v16319 = v10410 * v6503;
                            let v16320 = v16171 * v663;
                            let v16323 = (Lanes([0.0, 0.0, v16319[0], 0.0])) + (Lanes([v16320[0], v16320[1], 0.0, v16320[2]]));
                            let v16324 = Lanes([v16323[0], v16323[1], v16323[2], 0.0, v16323[3]]);
                            let v16326 = v16301 - ((((v16308 + (v16306 + v16306)) * (v9368 / v6647)) - v16317) + v16324);
                            let v6655 = (v6644 - (((v6647.ln()) - v6650) + v6652)) - v1;
                            let v6656 = v90 * v6644;
                            let v16327 = v16301 * v90;
                            let v6657 = if v6656 > v0 { 1.0 } else { 0.0 };
                            let v6659: f64;
                            let v9998: Lanes<5>;
                            if v6657 != 0.0 {
                                v6659 = v6656;
                                v9998 = v16327;
                            } else {
                                let v6658 = -v6656;
                                let v16328 = v16327 * v10390;
                                v6659 = v6658;
                                v9998 = v16328;
                            }
                            let v16329 = v16326 * v6655;
                            let v6662 = ((v6655 * v6655) + v6659).sqrt();
                            let v16339 = v10410 * v79;
                            let v6668 = (v6644 - (v6644 - (v13 * (v6655 + v6662)))) + (v663 * v79);
                            let v16342 = ((v16301 - (v16301 - ((v16326 + (((v16329 + v16329) + v9998) * (v9368 / (v10435 * v6662)))) * v13))) + (Lanes([0.0, 0.0, v16339[0], 0.0, 0.0]))) * v6668;
                            let v6670 = v6645 + (v6668 * v6668);
                            let v6673 = ((v6670.ln()) - v6650) + v6652;
                            let v16348 = (((v16308 + (v16342 + v16342)) * (v9368 / v6670)) - v16317) + v16324;
                            let v16349 = v16348 - v9997;
                            let v6677 = (v6673 - v6674) - v6676;
                            let v6680 = (v90 * v6673) * v6679;
                            let v16351 = (v16348 * v90) * v6679;
                            let v6681 = if v6680 > v0 { 1.0 } else { 0.0 };
                            let v6683: f64;
                            let v9999: Lanes<5>;
                            if v6681 != 0.0 {
                                v6683 = v6680;
                                v9999 = v16351;
                            } else {
                                let v6682 = -v6680;
                                let v16352 = v16351 * v10390;
                                v6683 = v6682;
                                v9999 = v16352;
                            }
                            let v16353 = v16349 * v6677;
                            let v6686 = ((v6677 * v6677) + v6683).sqrt();
                            let v6689 = v6673 - (v13 * (v6677 + v6686));
                            let v16361 = v16348 - ((v16349 + (((v16353 + v16353) + v9999) * (v9368 / (v10435 * v6686)))) * v13);
                            let v6690 = v6689 / v663;
                            let v16362 = v10410 * v6690;
                            let v6691 = v6690 - v6503;
                            let v16366 = ((v16361 - (Lanes([0.0, 0.0, v16362[0], 0.0, 0.0]))) / v663) - v16203;
                            let v6694 = (-v6689).exp();
                            let v6695 = (v6689 - v1) + v6694;
                            let v16369 = v16361 + ((v16361 * v10390) * v6694);
                            let v6697 = if v6695 < v6696 { 1.0 } else { 0.0 };
                            let v6699: f64;
                            let v10000: Lanes<5>;
                            if v6697 != 0.0 {
                                v6699 = v6698;
                                v10000 = v10579;
                            } else {
                                v6699 = v6695;
                                v10000 = v16369;
                            }
                            let v6700 = v6699.sqrt();
                            let v6701 = v6043 * v6700;
                            let v16373 = v15614 * v6700;
                            let v16376 = (Lanes([0.0, 0.0, v16373[0], 0.0, 0.0])) + ((v10000 * (v9368 / (v10435 * v6700))) * v6043);
                            let v6703 = v127 * (v6504 - v6691);
                            let v16378 = (v16201 - v16366) * v127;
                            let v6704 = if v6304 == v1 { 1.0 } else { 0.0 };
                            let v6851: f64;
                            let v6853: f64;
                            let v10001: Lanes<5>;
                            let v10002: Lanes<5>;
                            if v6704 != 0.0 {
                                let v6705 = v6642 * v6639;
                                let v16379 = v16294 * v6639;
                                let v16381 = (Lanes([0.0, 0.0, v16379[0], 0.0])) + v16296;
                                let mut v6706: f64 = 0.0;
                                let mut v6709: f64 = 0.0;
                                let mut v6795: f64 = 0.0;
                                let mut v6825: f64 = 0.0;
                                let mut v6828: f64 = 0.0;
                                let mut v6838: f64 = 0.0;
                                let mut v6843: f64 = 0.0;
                                let mut v10003: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10004: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10005: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10006: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10007: Lanes<5> = Lanes([0.0; 5]);
                                v6706 = v1;
                                v6709 = v6691;
                                v6795 = v0;
                                v6825 = v6689;
                                v6828 = v6829;
                                v6838 = v6839;
                                v6843 = v6844;
                                v10003 = v16366;
                                v10004 = v16361;
                                v10005 = v9941;
                                v10006 = v9942;
                                v10007 = v9943;
                                loop {
                                    let v6708 = if v6706 <= v6707 { 1.0 } else { 0.0 };
                                    if v6708 == 0.0 {
                                        break;
                                    }
                                    let v6710 = v6709 + v6503;
                                    let v6711 = v663 * v6710;
                                    let v16402 = v10410 * v6710;
                                    let v16405 = (Lanes([0.0, 0.0, v16402[0], 0.0, 0.0])) + ((v10003 + v16203) * v663);
                                    let v6712 = if v6711 < v644 { 1.0 } else { 0.0 };
                                    let v6788: f64;
                                    let v6792: f64;
                                    let v6831: f64;
                                    let v6846: f64;
                                    let v10008: Lanes<5>;
                                    let v10009: Lanes<5>;
                                    let v10010: Lanes<5>;
                                    let v10011: Lanes<5>;
                                    if v6712 != 0.0 {
                                        let v6713 = v6711 * v6711;
                                        let v16447 = v16405 * v6711;
                                        let v16448 = v16447 + v16447;
                                        let v6714 = v6713 * v6711;
                                        let v6717 = v6715 + (v6711 * v6318);
                                        let v6719 = v6316 + (v6711 * v6717);
                                        let v6720 = v6714 * v6719;
                                        let v16458 = (((v16448 * v6711) + (v16405 * v6713)) * v6719) + (((v16405 * v6717) + ((v16405 * v6318) * v6711)) * v6714);
                                        let v6723 = v6711 * v644;
                                        let v16459 = v16405 * v644;
                                        let v6725 = v6722 + (v6723 * v6318);
                                        let v6727 = v6721 + (v6711 * v6725);
                                        let v6728 = v6713 * v6727;
                                        let v6729 = v6705 * v6720;
                                        let v16467 = v16381 * v6720;
                                        let v6730 = v6729 * v6720;
                                        let v16473 = (((Lanes([v16467[0], v16467[1], v16467[2], 0.0, v16467[3]])) + (v16458 * v6705)) * v6720) + (v16458 * v6729);
                                        let v16475 = v10410 * v6705;
                                        let v6732 = (v6705 * v663) * v78;
                                        let v6733 = v6732 * v6720;
                                        let v16479 = (((v16381 * v663) + (Lanes([0.0, 0.0, v16475[0], 0.0]))) * v78) * v6720;
                                        let v6738 = v6736 + (v6711 * v6342);
                                        let v6740 = v6340 + (v6711 * v6738);
                                        let v6742 = v6735 + (v6711 * v6740);
                                        let v6744 = v6338 + (v6711 * v6742);
                                        let v6745 = v6711 * v6744;
                                        let v16498 = (v16405 * v6744) + (((v16405 * v6742) + (((v16405 * v6740) + (((v16405 * v6738) + ((v16405 * v6342) * v6711)) * v6711)) * v6711)) * v6711);
                                        let v6750 = v6748 + (v6723 * v6342);
                                        let v6752 = v6747 + (v6711 * v6750);
                                        let v6754 = v6746 + (v6711 * v6752);
                                        let v6756 = v6338 + (v6711 * v6754);
                                        let v16509 = v16498 * v6745;
                                        let v6760 = (((v6745 * v6745) + v6730) + v362).sqrt();
                                        let v16514 = ((v16509 + v16509) + v16473) * (v9368 / (v10435 * v6760));
                                        let v16515 = v10410 * v6756;
                                        let v6762 = (v663 * v6756) * v78;
                                        let v6765 = v6760 + v6760;
                                        let v6766 = ((v6762 * v6745) + (v6733 * v6728)) / v6765;
                                        let v16527 = (((((((Lanes([0.0, 0.0, v16515[0], 0.0, 0.0])) + (((v16405 * v6754) + (((v16405 * v6752) + (((v16405 * v6750) + ((v16459 * v6342) * v6711)) * v6711)) * v6711)) * v663)) * v78) * v6745) + (v16498 * v6762)) + ((((Lanes([v16479[0], v16479[1], v16479[2], 0.0, v16479[3]])) + (v16458 * v6732)) * v6728) + (((v16448 * v6727) + (((v16405 * v6725) + ((v16459 * v6318) * v6711)) * v6713)) * v6733))) - ((v16514 + v16514) * v6766)) / v6765;
                                        v6788 = v6760;
                                        v6792 = v6766;
                                        v6831 = v6745;
                                        v6846 = v6730;
                                        v10008 = v16514;
                                        v10009 = v16527;
                                        v10010 = v16498;
                                        v10011 = v16473;
                                    } else {
                                        let v6767 = if v6711 < v2535 { 1.0 } else { 0.0 };
                                        let v6780: f64;
                                        let v6783: f64;
                                        let v10012: Lanes<5>;
                                        let v10013: Lanes<5>;
                                        if v6767 != 0.0 {
                                            let v6768 = v6711.exp();
                                            let v16424 = v16405 * v6768;
                                            let v6769 = v6768 - v1;
                                            let v6770 = v6705 * v6769;
                                            let v16425 = v16381 * v6769;
                                            let v16428 = (Lanes([v16425[0], v16425[1], v16425[2], 0.0, v16425[3]])) + (v16424 * v6705);
                                            let v6771 = v6705 * v663;
                                            let v16430 = v10410 * v6705;
                                            let v6772 = v6771 * v6768;
                                            let v16433 = ((v16381 * v663) + (Lanes([0.0, 0.0, v16430[0], 0.0]))) * v6768;
                                            let v16436 = (Lanes([v16433[0], v16433[1], v16433[2], 0.0, v16433[3]])) + (v16424 * v6771);
                                            v6780 = v6770;
                                            v6783 = v6772;
                                            v10012 = v16428;
                                            v10013 = v16436;
                                        } else {
                                            let v16406 = v10410 * v6709;
                                            let v6774 = (v663 * v6709).exp();
                                            let v16410 = ((Lanes([0.0, 0.0, v16406[0], 0.0, 0.0])) + (v10003 * v663)) * v6774;
                                            let v6775 = v6774 - v6639;
                                            let v6776 = v6642 * v6775;
                                            let v16413 = v16294 * v6775;
                                            let v16416 = (Lanes([0.0, 0.0, v16413[0], 0.0, 0.0])) + ((v16410 - (Lanes([v16291[0], v16291[1], v16291[2], 0.0, v16291[3]]))) * v6642);
                                            let v6777 = v6642 * v663;
                                            let v6778 = v6777 * v6774;
                                            let v16420 = ((v16294 * v663) + (v10410 * v6642)) * v6774;
                                            let v16423 = (Lanes([0.0, 0.0, v16420[0], 0.0, 0.0])) + (v16410 * v6777);
                                            v6780 = v6776;
                                            v6783 = v6778;
                                            v10012 = v16416;
                                            v10013 = v16423;
                                        }
                                        let v6782 = ((v6711 - v1) + v6780).sqrt();
                                        let v16440 = (v16405 + v10012) * (v9368 / (v10435 * v6782));
                                        let v6785 = (v663 + v6783) / v6782;
                                        let v6786 = v6785 * v13;
                                        let v16446 = ((((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + v10013) - (v16440 * v6785)) / v6782) * v13;
                                        v6788 = v6782;
                                        v6792 = v6786;
                                        v6831 = v0;
                                        v6846 = v6780;
                                        v10008 = v16440;
                                        v10009 = v16446;
                                        v10010 = v10579;
                                        v10011 = v10012;
                                    }
                                    let v16529 = v15658 * v6788;
                                    let v6790 = (v6504 - v6709) - (v6096 * v6788);
                                    let v16533 = (v16201 - v10003) - ((Lanes([0.0, 0.0, v16529[0], 0.0, 0.0])) + (v10008 * v6096));
                                    let v16534 = v15658 * v6792;
                                    let v6794 = v6791 - (v6096 * v6792);
                                    let v16538 = ((Lanes([0.0, 0.0, v16534[0], 0.0, 0.0])) + (v10009 * v6096)) * v10390;
                                    let v6796 = if v6795 == v1 { 1.0 } else { 0.0 };
                                    let v6819: f64;
                                    let v6821: f64;
                                    let v6822: f64;
                                    let v10014: Lanes<5>;
                                    if v6796 != 0.0 {
                                        v6819 = v6797;
                                        v6821 = v6709;
                                        v6822 = v6795;
                                        v10014 = v10003;
                                    } else {
                                        let v6799 = (-v6790) / v6794;
                                        let v16542 = ((v16533 * v10390) - (v16538 * v6799)) / v6794;
                                        let v6801 = v6709.abs();
                                        let v16546 = v10003 * ((v10435 * (if v6709 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                                        let v6802 = if v1 >= v6801 { 1.0 } else { 0.0 };
                                        let v6803: f64;
                                        let v10015: Lanes<5>;
                                        if v6802 != 0.0 {
                                            v6803 = v1;
                                            v10015 = v10579;
                                        } else {
                                            v6803 = v6801;
                                            v10015 = v16546;
                                        }
                                        let v6805 = v6800 * (v1 + v6803);
                                        let v16547 = v10015 * v6800;
                                        let v6807 = if (v6799.abs()) > v6805 { 1.0 } else { 0.0 };
                                        let v6812: f64;
                                        let v10016: Lanes<5>;
                                        if v6807 != 0.0 {
                                            let v6808 = if v6799 >= v0 { 1.0 } else { 0.0 };
                                            let v6810: f64;
                                            if v6808 != 0.0 {
                                                v6810 = v1;
                                            } else {
                                                v6810 = v6809;
                                            }
                                            let v6811 = v6805 * v6810;
                                            let v16548 = v16547 * v6810;
                                            v6812 = v6811;
                                            v10016 = v16548;
                                        } else {
                                            v6812 = v6799;
                                            v10016 = v16542;
                                        }
                                        let v6813 = v6709 + v6812;
                                        let v16549 = v10003 + v10016;
                                        let v6818 = if (if (v6812.abs()) <= v861 { 1.0 } else { 0.0 }) != 0.0 && (if (v6790.abs()) <= v3506 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v6823: f64;
                                        if v6818 != 0.0 {
                                            v6823 = v1;
                                        } else {
                                            v6823 = v6795;
                                        }
                                        v6819 = v6706;
                                        v6821 = v6813;
                                        v6822 = v6823;
                                        v10014 = v16549;
                                    }
                                    let v6820 = v6819 + v1;
                                    v6706 = v6820;
                                    v6709 = v6821;
                                    v6795 = v6822;
                                    v6825 = v6711;
                                    v6828 = v6831;
                                    v6838 = v6788;
                                    v6843 = v6846;
                                    v10003 = v10014;
                                    v10004 = v16405;
                                    v10005 = v10010;
                                    v10006 = v10008;
                                    v10007 = v10011;
                                }
                                let v6824 = if v6795 == v0 { 1.0 } else { 0.0 };
                                if v6824 != 0.0 {
                                } else {
                                }
                                let v6826 = if v6825 < v644 { 1.0 } else { 0.0 };
                                let v6836: f64;
                                let v10017: Lanes<5>;
                                if v6826 != 0.0 {
                                    let v6827 = if v6825 < v96 { 1.0 } else { 0.0 };
                                    if v6827 != 0.0 {
                                    } else {
                                    }
                                    let v6833 = v6828 + v6832;
                                    v6836 = v6833;
                                    v10017 = v10005;
                                } else {
                                    let v6835 = (v6825 - v1).sqrt();
                                    let v16384 = v10004 * (v9368 / (v10435 * v6835));
                                    v6836 = v6835;
                                    v10017 = v16384;
                                }
                                let v6837 = v6043 * v6836;
                                let v16385 = v15614 * v6836;
                                let v16388 = (Lanes([0.0, 0.0, v16385[0], 0.0, 0.0])) + (v10017 * v6043);
                                let v6841 = v6838 + v6836;
                                let v6842 = v1 / v6841;
                                let v6847 = v6043 * v6843;
                                let v16393 = v15614 * v6843;
                                let v6849 = v6837 + (v6847 * v6842);
                                let v16400 = v16388 + ((((Lanes([0.0, 0.0, v16393[0], 0.0, 0.0])) + (v10007 * v6043)) * v6842) + (((((v10006 + v10017) * v6842) * v10390) / v6841) * v6847));
                                v6851 = v6849;
                                v6853 = v6837;
                                v10001 = v16400;
                                v10002 = v16388;
                            } else {
                                v6851 = v6703;
                                v6853 = v6701;
                                v10001 = v16378;
                                v10002 = v16376;
                            }
                            v6850 = v6851;
                            v6852 = v6853;
                            v9992 = v10001;
                            v9993 = v10002;
                        }
                        let v6854 = v6850 - v6852;
                        let v16635 = v9992 - v9993;
                        let v8542: f64;
                        let v8550: f64;
                        let v8558: f64;
                        let v8570: f64;
                        let v8582: f64;
                        let v8589: f64;
                        let v8599: f64;
                        let v8606: f64;
                        let v10018: Lanes<5>;
                        let v10019: Lanes<5>;
                        let v10020: Lanes<6>;
                        let v10021: Lanes<6>;
                        let v10022: Lanes<5>;
                        let v10023: Lanes<6>;
                        let v10024: Lanes<5>;
                        let v10025: Lanes<6>;
                        if v6855 != 0.0 {
                            let v8543: f64;
                            let v8600: f64;
                            let v10026: Lanes<5>;
                            let v10027: Lanes<5>;
                            if v6471 != 0.0 {
                                let v6856 = -v6456;
                                let v6857 = v6856 * v6850;
                                let v16644 = v9992 * v6856;
                                let v6858 = v6856 * v6854;
                                let v16645 = v16635 * v6856;
                                v8543 = v6857;
                                v8600 = v6858;
                                v10026 = v16644;
                                v10027 = v16645;
                            } else {
                                v8543 = v8544;
                                v8600 = v8601;
                                v10026 = v9971;
                                v10027 = v9977;
                            }
                            let v8551: f64;
                            let v8583: f64;
                            let v10028: Lanes<5>;
                            let v10029: Lanes<5>;
                            if v6472 != 0.0 {
                                let v6859 = -v6456;
                                let v6860 = v6859 * v6850;
                                let v16646 = v9992 * v6859;
                                let v6861 = v6859 * v6854;
                                let v16647 = v16635 * v6859;
                                v8551 = v6860;
                                v8583 = v6861;
                                v10028 = v16646;
                                v10029 = v16647;
                            } else {
                                v8551 = v8552;
                                v8583 = v8584;
                                v10028 = v9972;
                                v10029 = v9975;
                            }
                            v8542 = v8543;
                            v8550 = v8551;
                            v8558 = v8559;
                            v8570 = v8571;
                            v8582 = v8583;
                            v8589 = v8590;
                            v8599 = v8600;
                            v8606 = v8607;
                            v10018 = v10026;
                            v10019 = v10028;
                            v10020 = v9973;
                            v10021 = v9974;
                            v10022 = v10029;
                            v10023 = v9976;
                            v10024 = v10027;
                            v10025 = v9978;
                        } else {
                            let v8562: f64;
                            let v8574: f64;
                            let v8593: f64;
                            let v8610: f64;
                            let v10030: Lanes<6>;
                            let v10031: Lanes<6>;
                            let v10032: Lanes<6>;
                            let v10033: Lanes<6>;
                            if v6862 != 0.0 {
                                let v8563: f64;
                                let v8611: f64;
                                let v10034: Lanes<6>;
                                let v10035: Lanes<6>;
                                if v6471 != 0.0 {
                                    let v6863 = -v6456;
                                    let v6864 = v6863 * v6850;
                                    let v16636 = v9992 * v6863;
                                    let v6865 = v6863 * v6854;
                                    let v16637 = v16635 * v6863;
                                    let v16638 = Lanes([v16636[0], v16636[1], v16636[2], v16636[3], v16636[4], 0.0]);
                                    let v16639 = Lanes([v16637[0], v16637[1], v16637[2], v16637[3], v16637[4], 0.0]);
                                    v8563 = v6864;
                                    v8611 = v6865;
                                    v10034 = v16638;
                                    v10035 = v16639;
                                } else {
                                    v8563 = v8559;
                                    v8611 = v8607;
                                    v10034 = v9973;
                                    v10035 = v9978;
                                }
                                let v8575: f64;
                                let v8594: f64;
                                let v10036: Lanes<6>;
                                let v10037: Lanes<6>;
                                if v6472 != 0.0 {
                                    let v6866 = -v6456;
                                    let v6867 = v6866 * v6850;
                                    let v16640 = v9992 * v6866;
                                    let v6868 = v6866 * v6854;
                                    let v16641 = v16635 * v6866;
                                    let v16642 = Lanes([v16640[0], v16640[1], v16640[2], v16640[3], v16640[4], 0.0]);
                                    let v16643 = Lanes([v16641[0], v16641[1], v16641[2], v16641[3], v16641[4], 0.0]);
                                    v8575 = v6867;
                                    v8594 = v6868;
                                    v10036 = v16642;
                                    v10037 = v16643;
                                } else {
                                    v8575 = v8571;
                                    v8594 = v8590;
                                    v10036 = v9974;
                                    v10037 = v9976;
                                }
                                v8562 = v8563;
                                v8574 = v8575;
                                v8593 = v8594;
                                v8610 = v8611;
                                v10030 = v10034;
                                v10031 = v10036;
                                v10032 = v10037;
                                v10033 = v10035;
                            } else {
                                v8562 = v8559;
                                v8574 = v8571;
                                v8593 = v8590;
                                v8610 = v8607;
                                v10030 = v9973;
                                v10031 = v9974;
                                v10032 = v9976;
                                v10033 = v9978;
                            }
                            v8542 = v8544;
                            v8550 = v8552;
                            v8558 = v8562;
                            v8570 = v8574;
                            v8582 = v8584;
                            v8589 = v8593;
                            v8599 = v8601;
                            v8606 = v8610;
                            v10018 = v9971;
                            v10019 = v9972;
                            v10020 = v10030;
                            v10021 = v10031;
                            v10022 = v9975;
                            v10023 = v10032;
                            v10024 = v9977;
                            v10025 = v10033;
                        }
                        v8541 = v8542;
                        v8549 = v8550;
                        v8557 = v8558;
                        v8569 = v8570;
                        v8581 = v8582;
                        v8588 = v8589;
                        v8598 = v8599;
                        v8605 = v8606;
                        v9930 = v10018;
                        v9931 = v10019;
                        v9932 = v10020;
                        v9933 = v10021;
                        v9934 = v10022;
                        v9935 = v10023;
                        v9936 = v10024;
                        v9937 = v10025;
                    } else {
                        v8541 = v0;
                        v8549 = v0;
                        v8557 = v6030;
                        v8569 = v6029;
                        v8581 = v0;
                        v8588 = v6027;
                        v8598 = v0;
                        v8605 = v6028;
                        v9930 = v10579;
                        v9931 = v10579;
                        v9932 = v15613;
                        v9933 = v15612;
                        v9934 = v10579;
                        v9935 = v15610;
                        v9936 = v10579;
                        v9937 = v15611;
                    }
                    v8540 = v8541;
                    v8548 = v8549;
                    v8556 = v8557;
                    v8568 = v8569;
                    v8580 = v8581;
                    v8587 = v8588;
                    v8597 = v8598;
                    v8604 = v8605;
                    v9922 = v9930;
                    v9923 = v9931;
                    v9924 = v9932;
                    v9925 = v9933;
                    v9926 = v9934;
                    v9927 = v9935;
                    v9928 = v9936;
                    v9929 = v9937;
                } else {
                    v8540 = v0;
                    v8548 = v0;
                    v8556 = v6030;
                    v8568 = v6029;
                    v8580 = v0;
                    v8587 = v6027;
                    v8597 = v0;
                    v8604 = v6028;
                    v9922 = v10579;
                    v9923 = v10579;
                    v9924 = v15613;
                    v9925 = v15612;
                    v9926 = v10579;
                    v9927 = v15610;
                    v9928 = v10579;
                    v9929 = v15611;
                }
                v8539 = v8540;
                v8547 = v8548;
                v8555 = v8556;
                v8567 = v8568;
                v8579 = v8580;
                v8586 = v8587;
                v8596 = v8597;
                v8603 = v8604;
                v9914 = v9922;
                v9915 = v9923;
                v9916 = v9924;
                v9917 = v9925;
                v9918 = v9926;
                v9919 = v9927;
                v9920 = v9928;
                v9921 = v9929;
            } else {
                v8539 = v0;
                v8547 = v0;
                v8555 = v8564;
                v8567 = v8576;
                v8579 = v0;
                v8586 = v0;
                v8596 = v0;
                v8603 = v0;
                v9914 = v10579;
                v9915 = v10579;
                v9916 = v9465;
                v9917 = v9466;
                v9918 = v10579;
                v9919 = v11062;
                v9920 = v10579;
                v9921 = v11062;
            }
            let v6869 = if v4325 != v0 { 1.0 } else { 0.0 };
            let v8298: f64;
            let v8511: f64;
            let v10038: Lanes<6>;
            let v10039: Lanes<6>;
            if v6869 != 0.0 {
                let v6870 = v823 + v4340;
                let v16660 = (Lanes([v9410[0], v9410[1], 0.0, 0.0, 0.0, 0.0])) + v9441;
                let v6872 = v1 - v4356;
                let v6874 = (v4356 * v6870) + (v6872 * v4336);
                let v16663 = (v16660 * v4356) + (v9440 * v6872);
                let v6876 = if v6875 != v0 { 1.0 } else { 0.0 };
                if v6876 != 0.0 {
                } else {
                }
                let v6879 = if v6874 > (v6870 - v6877) { 1.0 } else { 0.0 };
                let v8299: f64;
                let v10040: Lanes<6>;
                if v6879 != 0.0 {
                    let v6881 = v6870 - v6880;
                    v8299 = v6881;
                    v10040 = v16660;
                } else {
                    v8299 = v6874;
                    v10040 = v16663;
                }
                v8298 = v8299;
                v8511 = v0;
                v10038 = v10040;
                v10039 = v11062;
            } else {
                let v6882 = if v6875 != v0 { 1.0 } else { 0.0 };
                let v8512: f64;
                let v10041: Lanes<6>;
                if v6882 != 0.0 {
                    let v6884 = if v4381 < v6883 { 1.0 } else { 0.0 };
                    let v8513: f64;
                    let v10042: Lanes<6>;
                    if v6884 != 0.0 {
                        v8513 = v0;
                        v10042 = v11062;
                    } else {
                        let v6885 = v665 / v136;
                        let v6886 = v1 / v4348;
                        let v6887 = v4381 * v6885;
                        let v16653 = (v10415 / v136) * v4381;
                        let v6888 = v6887 * v6886;
                        let v16658 = (((v9443 * v6885) + (Lanes([0.0, 0.0, v16653[0], 0.0, 0.0, 0.0]))) * v6886) + ((((v9442 * v6886) * v10390) / v4348) * v6887);
                        v8513 = v6888;
                        v10042 = v16658;
                    }
                    v8512 = v8513;
                    v10041 = v10042;
                } else {
                    v8512 = v0;
                    v10041 = v11062;
                }
                v8298 = v8300;
                v8511 = v8512;
                v10038 = v9783;
                v10039 = v10041;
            }
            let v6889 = v1 / v127;
            let v8454: f64;
            let v8458: f64;
            let v8623: f64;
            let v8629: f64;
            let v8641: f64;
            let v8652: f64;
            let v10043: Lanes<6>;
            let v10044: Lanes<6>;
            let v10045: Lanes<5>;
            let v10046: Lanes<5>;
            let v10047: Lanes<5>;
            let v10048: Lanes<5>;
            if v566 != 0.0 {
                let v6893 = if v6892 > v0 { 1.0 } else { 0.0 };
                let v6894 = if (if v6890 >= v1 { 1.0 } else { 0.0 }) != 0.0 && v6893 != 0.0 { 1.0 } else { 0.0 };
                let v8455: f64;
                let v8459: f64;
                let v8624: f64;
                let v8630: f64;
                let v8642: f64;
                let v8653: f64;
                let v10049: Lanes<6>;
                let v10050: Lanes<6>;
                let v10051: Lanes<5>;
                let v10052: Lanes<5>;
                let v10053: Lanes<5>;
                let v10054: Lanes<5>;
                if v6894 != 0.0 {
                    let v6898 = if (if v39 == v0 { 1.0 } else { 0.0 }) != 0.0 && v6893 != 0.0 { 1.0 } else { 0.0 };
                    let v7801: f64;
                    let v7820: f64;
                    let v8625: f64;
                    let v8631: f64;
                    let v8643: f64;
                    let v8654: f64;
                    let v10055: Lanes<6>;
                    let v10056: Lanes<6>;
                    let v10057: Lanes<5>;
                    let v10058: Lanes<5>;
                    let v10059: Lanes<5>;
                    let v10060: Lanes<5>;
                    if v6898 != 0.0 {
                        let v6902: f64;
                        if v565 != 0.0 {
                            let v6900 = v6899 * v127;
                            v6902 = v6900;
                        } else {
                            let v6901 = v168 * v127;
                            v6902 = v6901;
                        }
                        let v6903 = v6895 * v6902;
                        let v6904 = v6896 + v830;
                        let v6905 = v6903 * v6904;
                        let v6906 = v6892 * v6902;
                        let v6907 = v777 - v4340;
                        let v17742 = v9412 * v6906;
                        let v17744 = (v9412 * v6903) * v6907;
                        let v6910 = (v830 * v6906) - (v6907 * v6905);
                        let v17748 = (Lanes([v17742[0], v17742[1], 0.0, v17742[2], 0.0, 0.0])) - (((v9441 * v10390) * v6905) + (Lanes([v17744[0], v17744[1], 0.0, v17744[2], 0.0, 0.0])));
                        let v17750 = v9412 - (Lanes([v9410[0], v9410[1], 0.0]));
                        let v6912 = v6903 * (v6904 - v823);
                        let v6914 = v777 - (v4336 - v823);
                        let v17755 = v17750 * v6906;
                        let v17756 = (v17750 * v6903) * v6914;
                        let v6918 = ((v830 - v823) * v6906) - (v6912 * v6914);
                        let v17761 = (Lanes([v17755[0], v17755[1], 0.0, v17755[2], 0.0, 0.0])) - ((Lanes([v17756[0], v17756[1], 0.0, v17756[2], 0.0, 0.0])) + (((v9440 - (Lanes([v9410[0], v9410[1], 0.0, 0.0, 0.0, 0.0]))) * v10390) * v6912));
                        v7801 = v6918;
                        v7820 = v6910;
                        v8625 = v0;
                        v8631 = v0;
                        v8643 = v0;
                        v8654 = v0;
                        v10055 = v17761;
                        v10056 = v17748;
                        v10057 = v10579;
                        v10058 = v10579;
                        v10059 = v10579;
                        v10060 = v10579;
                    } else {
                        let v6920 = (v39 / v489).sqrt();
                        let v6921 = v750 * v6920;
                        let v16670 = v10485 * v6920;
                        let v6960: f64;
                        let v6982: f64;
                        let v7344: f64;
                        let v7350: f64;
                        let v10061: Lanes<3>;
                        let v10062: Lanes<4>;
                        if v565 != 0.0 {
                            let v6927 = (v6046 * v835) + (v6048 * (v835 - v823));
                            let v16685 = (v9413 * v6046) + ((v9413 - v10560) * v6048);
                            let v16689 = (v9410 * v6046) + ((v9410 * v10390) * v6048);
                            let v16694 = (v9412 * v6046) + ((v9412 - (Lanes([v9410[0], v9410[1], 0.0]))) * v6048);
                            let v6937 = ((v6046 * v830) + (v6048 * (v830 - v823))) - v6927;
                            let v16699 = (Lanes([v16694[0], v16694[1], v16694[2], 0.0])) - (Lanes([v16685[0], v16685[1], 0.0, v16685[2]]));
                            let v6940 = v6046 + (v6923 * v6048);
                            let v6942 = v6048 + (v6923 * v6046);
                            let v16703 = ((v16685 * v10390) * v6940) + (((Lanes([v16689[0], v16689[1], 0.0])) - v16685) * v6942);
                            let v6947 = ((v6940 * (-v6927)) + (v6942 * (((v6046 * v823) + (v6048 * (-v823))) - v6927))) + v6946;
                            v6960 = v6947;
                            v6982 = v6937;
                            v7344 = v6940;
                            v7350 = v6942;
                            v10061 = v16703;
                            v10062 = v16699;
                        } else {
                            let v6949 = v6046 + (v6923 * v6048);
                            let v6951 = v6048 + (v6923 * v6046);
                            let v6984: f64;
                            let v10063: Lanes<3>;
                            if v6922 != 0.0 {
                                let v6955 = (v6046 * v830) + (v6048 * (v830 - v823));
                                let v16675 = (v9412 * v6046) + ((v9412 - (Lanes([v9410[0], v9410[1], 0.0]))) * v6048);
                                v6984 = v6955;
                                v10063 = v16675;
                            } else {
                                v6984 = v0;
                                v10063 = v10531;
                            }
                            let v6983: f64;
                            let v10064: Lanes<3>;
                            if v6923 != 0.0 {
                                let v6959 = (v6048 * v830) + (v6046 * (v830 - v823));
                                let v16680 = (v9412 * v6048) + ((v9412 - (Lanes([v9410[0], v9410[1], 0.0]))) * v6046);
                                v6983 = v6959;
                                v10064 = v16680;
                            } else {
                                v6983 = v6984;
                                v10064 = v10063;
                            }
                            let v16681 = Lanes([v10064[0], v10064[1], v10064[2], 0.0]);
                            v6960 = v0;
                            v6982 = v6983;
                            v7344 = v6949;
                            v7350 = v6951;
                            v10061 = v10504;
                            v10062 = v16681;
                        }
                        let v6961 = -v6960;
                        let v16704 = v10061 * v10390;
                        let v6962 = if v6961 > v783 { 1.0 } else { 0.0 };
                        let v6977: f64;
                        let v10065: Lanes<3>;
                        if v6962 != 0.0 {
                            let v6964 = v779 - v783;
                            let v6965 = (v6961 - v783) / v6964;
                            let v16705 = v16704 / v6964;
                            let v6966 = v6965 * v6965;
                            let v16706 = v16705 * v6965;
                            let v16707 = v16706 + v16706;
                            let v16711 = v16707 * v6966;
                            let v6972 = (((v1 + v6965) + v6966) + (v6966 * v6965)) + (v6966 * v6966);
                            let v6973 = v1 / v6972;
                            let v16720 = (((((((v16705 + v16707) + ((v16707 * v6965) + (v16705 * v6966))) + (v16711 + v16711)) * v6973) * v10390) / v6972) * v10390) * v6964;
                            let v6976 = v783 + (v6964 * (v1 - v6973));
                            v6977 = v6976;
                            v10065 = v16720;
                        } else {
                            v6977 = v6961;
                            v10065 = v16704;
                        }
                        let v16721 = v10065 * v10390;
                        let v6979 = (-v6977) - v11;
                        let v6980 = v6921 * v6889;
                        let v16722 = v16670 * v6889;
                        let v6981 = v6980 * v6980;
                        let v16723 = v16722 * v6980;
                        let v16724 = v16723 + v16723;
                        let v16725 = v10062 * v10390;
                        let v6986 = (-v6982) + v66;
                        let v6987 = v39 / v731;
                        let v6988 = v78 / v663;
                        let v6989 = v6987.ln();
                        let v6990 = v6988 * v6989;
                        let v16736 = ((((v10410 * v6988) * v10390) / v663) * v6989) + (((((v10453 * v6987) * v10390) / v731) * (v9368 / v6987)) * v6988);
                        let v6991 = -v6979;
                        let v16737 = v16721 * v10390;
                        let v6992 = if v6986 < v6991 { 1.0 } else { 0.0 };
                        let v7337: f64;
                        let v7339: f64;
                        let v7749: f64;
                        let v10066: Lanes<5>;
                        let v10067: Lanes<5>;
                        let v10068: Lanes<5>;
                        if v6992 != 0.0 {
                            let v6993 = v663 * v6921;
                            let v6994 = v1 / v6993;
                            let v6995 = v6994 * v127;
                            let v17130 = (((((v10410 * v6921) + (v16670 * v663)) * v6994) * v10390) / v6993) * v127;
                            let v17131 = v17130 * v6996;
                            let v6998 = v78 + (v6996 * v6995);
                            let v6999 = v91 * v6998;
                            let v7000 = v6999 * v6998;
                            let v7001 = v7000 * v6998;
                            let v17138 = ((((v17131 * v91) * v6998) + (v17131 * v6999)) * v6998) + (v17131 * v7000);
                            let v7002 = v661 - v6990;
                            let v17139 = v10406 - v16736;
                            let v7003 = v6986 + v6979;
                            let v17142 = v10410 * v7003;
                            let v17143 = (v16725 + (Lanes([v16721[0], v16721[1], 0.0, v16721[2]]))) * v663;
                            let v7006 = v3500 * v6995;
                            let v7007 = (v663 * v7003) - v78;
                            let v7008 = v7006 * v7007;
                            let v17148 = (v17130 * v3500) * v7007;
                            let v17151 = (Lanes([0.0, 0.0, v17148[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v17142[0], 0.0, 0.0])) + (Lanes([v17143[0], v17143[1], 0.0, v17143[2], v17143[3]]))) * v7006);
                            let v7009 = v7005 - v7008;
                            let v17152 = v17151 * v10390;
                            let v7010 = v7009 * v7009;
                            let v17153 = v17152 * v7009;
                            let v17154 = v17153 + v17153;
                            let v7012 = if v7001 < (v7010 * v3506) { 1.0 } else { 0.0 };
                            let v7024: f64;
                            let v10069: Lanes<5>;
                            if v7012 != 0.0 {
                                let v17161 = v17138 * v13;
                                let v7016 = (v13 * v7001) / v7009;
                                let v7018 = ((v7013 + v7009) + v7016) + v7008;
                                let v17167 = (v17152 + (((Lanes([0.0, 0.0, v17161[0], 0.0, 0.0])) - (v17152 * v7016)) / v7009)) + v17151;
                                v7024 = v7018;
                                v10069 = v17167;
                            } else {
                                let v7020 = (v7001 + v7010).sqrt();
                                let v7023 = (v7021 + v7020) + v7008;
                                let v17160 = (((Lanes([0.0, 0.0, v17138[0], 0.0, 0.0])) + v17154) * (v9368 / (v10435 * v7020))) + v17151;
                                v7024 = v7023;
                                v10069 = v17160;
                            }
                            let v7025 = v7024.powf(v1562);
                            let v17171 = v10069 * (v1562 * (v7024.powf(v17168)));
                            let v17173 = (v17130 * v3523) * v10390;
                            let v7031 = v748 * v7025;
                            let v7034 = (((v7026 - (v3523 * v6995)) + (v78 * v7025)) + (v7031 * v7025)) / v7025;
                            let v17186 = v10415 * v7034;
                            let v17189 = Lanes([v16721[0], v16721[1], 0.0, 0.0, v16721[2]]);
                            let v7037 = ((v7034 * v665) - v6979) + v6979;
                            let v17191 = ((((((((Lanes([0.0, 0.0, v17173[0], 0.0, 0.0])) + (v17171 * v78)) + (((v17171 * v748) * v7025) + (v17171 * v7031))) - (v17171 * v7034)) / v7025) * v665) + (Lanes([0.0, 0.0, v17186[0], 0.0, 0.0]))) - v17189) + v17189;
                            let v7038 = v7037 / v7002;
                            let v17192 = v17139 * v7038;
                            let v17196 = ((v17191 - (Lanes([0.0, 0.0, v17192[0], 0.0, 0.0]))) / v7002) * v7038;
                            let v7041 = (v1 + (v7038 * v7038)).sqrt();
                            let v7042 = v7037 / v7041;
                            let v7045 = v127 * (v6986 - (v7042 - v6979));
                            let v17207 = ((Lanes([v16725[0], v16725[1], 0.0, v16725[2], v16725[3]])) - (((v17191 - (((v17196 + v17196) * (v9368 / (v10435 * v7041))) * v7042)) / v7041) - v17189)) * v127;
                            v7337 = v7045;
                            v7339 = v7045;
                            v7749 = v0;
                            v10066 = v17207;
                            v10067 = v17207;
                            v10068 = v10579;
                        } else {
                            let v7047 = v6986 + v6979;
                            let v16739 = v16725 + (Lanes([v16721[0], v16721[1], 0.0, v16721[2]]));
                            let v16740 = v10410 * v7047;
                            let v16741 = v16739 * v663;
                            let v16743 = Lanes([v16741[0], v16741[1], 0.0, v16741[2], v16741[3]]);
                            let v16744 = (Lanes([0.0, 0.0, v16740[0], 0.0, 0.0])) + v16743;
                            let v7049 = (v663 * v7047) - v1;
                            let v7052 = v6981 * v664;
                            let v16748 = (v16724 * v664) + (v10412 * v6981);
                            let v7053 = (v90 * (v7049 + v7046)) / v7052;
                            let v16749 = v16748 * v7053;
                            let v16752 = ((v16744 * v90) - (Lanes([0.0, 0.0, v16749[0], 0.0, 0.0]))) / v7052;
                            let v7054 = v1 + v7053;
                            let v7056 = if v7054 < v7055 { 1.0 } else { 0.0 };
                            let v7060: f64;
                            let v10070: Lanes<5>;
                            if v7056 != 0.0 {
                                v7060 = v7057;
                                v10070 = v10579;
                            } else {
                                v7060 = v7054;
                                v10070 = v16752;
                            }
                            let v7059 = (v6981 * v663) / v78;
                            let v16756 = ((v16724 * v663) + (v10410 * v6981)) / v78;
                            let v7061 = v7060.sqrt();
                            let v7062 = v1 - v7061;
                            let v16761 = v16756 * v7062;
                            let v16765 = Lanes([v16725[0], v16725[1], 0.0, v16725[2], v16725[3]]);
                            let v7065 = (v6986 + (v7059 * v7062)) + v6979;
                            let v16767 = Lanes([v16721[0], v16721[1], 0.0, 0.0, v16721[2]]);
                            let v16769 = v10410 * v7065;
                            let v7068 = (-(v663 * v7065)).exp();
                            let v7071 = (v90 * (v7049 + v7068)) / v7052;
                            let v16777 = v16748 * v7071;
                            let v16780 = (((v16744 + ((((Lanes([0.0, 0.0, v16769[0], 0.0, 0.0])) + (((v16765 + ((Lanes([0.0, 0.0, v16761[0], 0.0, 0.0])) + (((v10070 * (v9368 / (v10435 * v7061))) * v10390) * v7059))) + v16767) * v663)) * v10390) * v7068)) * v90) - (Lanes([0.0, 0.0, v16777[0], 0.0, 0.0]))) / v7052;
                            let v7072 = v1 + v7071;
                            let v7074 = if v7072 < v7073 { 1.0 } else { 0.0 };
                            let v7076: f64;
                            let v10071: Lanes<5>;
                            if v7074 != 0.0 {
                                v7076 = v7075;
                                v10071 = v10579;
                            } else {
                                v7076 = v7072;
                                v10071 = v16780;
                            }
                            let v7077 = v7076.sqrt();
                            let v7078 = v1 - v7077;
                            let v16785 = v16756 * v7078;
                            let v7081 = (v6986 + (v7059 * v7078)) + v6979;
                            let v7082 = v663 * v7081;
                            let v16791 = v10410 * v7081;
                            let v16794 = (Lanes([0.0, 0.0, v16791[0], 0.0, 0.0])) + (((v16765 + ((Lanes([0.0, 0.0, v16785[0], 0.0, 0.0])) + (((v10071 * (v9368 / (v10435 * v7077))) * v10390) * v7059))) + v16767) * v663);
                            let v7083 = if v7082 < v96 { 1.0 } else { 0.0 };
                            let v7162: f64;
                            let v10072: Lanes<5>;
                            if v7083 != 0.0 {
                                let v7086 = v663 * v6980;
                                let v7087 = v1 / v7086;
                                let v16800 = ((((v10410 * v6980) + (v16722 * v663)) * v7087) * v10390) / v7086;
                                let v7088 = v7085 + v7087;
                                let v16801 = v16739 * v10390;
                                let v7090 = (-v7047) / v6980;
                                let v16802 = v16722 * v7090;
                                let v16809 = ((v16800 * v7084) / v7093) * v10390;
                                let v7098 = (v7091 - ((v7084 * v7088) / v7093)) + (v7090 / v7096);
                                let v16812 = (Lanes([0.0, 0.0, v16809[0], 0.0, 0.0])) + ((((Lanes([v16801[0], v16801[1], 0.0, v16801[2], v16801[3]])) - (Lanes([0.0, 0.0, v16802[0], 0.0, 0.0]))) / v6980) / v7096);
                                let v7104 = ((v7099 * v7088) - v7101) / v7103;
                                let v16814 = (v16800 * v7099) / v7103;
                                let v16815 = v16812 * v7098;
                                let v7106 = v7104 * v7104;
                                let v16817 = v16814 * v7104;
                                let v16821 = ((v16817 + v16817) * v7104) + (v16814 * v7106);
                                let v7109 = ((v7098 * v7098) + (v7106 * v7104)).sqrt();
                                let v16826 = ((v16815 + v16815) + (Lanes([0.0, 0.0, v16821[0], 0.0, 0.0]))) * (v9368 / (v10435 * v7109));
                                let v7111 = (-v7098) + v7109;
                                let v7113 = v7098 + v7109;
                                let v7118 = ((v7111.powf(v1562)) + (-(v7113.powf(v1562)))) - v7117;
                                let v16841 = v10415 * v7118;
                                let v7121 = ((v7118 * v665) - v6979) + v6979;
                                let v7122 = v663 * v7121;
                                let v16846 = v10410 * v7121;
                                let v16849 = (Lanes([0.0, 0.0, v16846[0], 0.0, 0.0])) + (((((((((v16812 * v10390) + v16826) * (v1562 * (v7111.powf(v16829)))) + (((v16812 + v16826) * (v1562 * (v7113.powf(v16834)))) * v10390)) * v665) + (Lanes([0.0, 0.0, v16841[0], 0.0, 0.0]))) - v16767) + v16767) * v663);
                                v7162 = v7122;
                                v10072 = v16849;
                            } else {
                                v7162 = v7082;
                                v10072 = v16794;
                            }
                            let v7124 = if v7123 > v0 { 1.0 } else { 0.0 };
                            let v7178: f64;
                            let v10073: Lanes<5>;
                            if v7124 != 0.0 {
                                let v7125 = v7047 + v79;
                                let v16850 = v10410 * v6991;
                                let v16851 = v16737 * v663;
                                let v7127 = (v663 * v6991).exp();
                                let v7128 = v7127 + v362;
                                let v7129 = v731 / v39;
                                let v7130 = v7129 * v7129;
                                let v16857 = (v10453 / v39) * v7129;
                                let v16858 = v16857 + v16857;
                                let v7131 = v7130 * v7128;
                                let v16859 = v16858 * v7128;
                                let v7132 = v663 * v7125;
                                let v16863 = v10410 * v7125;
                                let v16865 = (Lanes([0.0, 0.0, v16863[0], 0.0, 0.0])) + v16743;
                                let v7133 = v7131 * v7052;
                                let v16867 = v16748 * v7131;
                                let v16869 = (((Lanes([0.0, 0.0, v16859[0], 0.0])) + ((((Lanes([0.0, 0.0, v16850[0], 0.0])) + (Lanes([v16851[0], v16851[1], 0.0, v16851[2]]))) * v7127) * v7130)) * v7052) + (Lanes([0.0, 0.0, v16867[0], 0.0]));
                                let v16870 = v16865 * v7132;
                                let v7135 = v7133 + (v7132 * v7132);
                                let v16872 = Lanes([v16869[0], v16869[1], v16869[2], 0.0, v16869[3]]);
                                let v7137 = v7130 * v7052;
                                let v7138 = v7137.ln();
                                let v16880 = ((v16858 * v7052) + (v16748 * v7130)) * (v9368 / v7137);
                                let v16881 = Lanes([0.0, 0.0, v16880[0], 0.0, 0.0]);
                                let v7140 = v663 * v6979;
                                let v16883 = v10410 * v6979;
                                let v16884 = v16721 * v663;
                                let v16887 = (Lanes([0.0, 0.0, v16883[0], 0.0])) + (Lanes([v16884[0], v16884[1], 0.0, v16884[2]]));
                                let v16888 = Lanes([v16887[0], v16887[1], v16887[2], 0.0, v16887[3]]);
                                let v16890 = v16865 - ((((v16872 + (v16870 + v16870)) * (v9368 / v7135)) - v16881) + v16888);
                                let v7143 = (v7132 - (((v7135.ln()) - v7138) + v7140)) - v1;
                                let v7144 = v90 * v7132;
                                let v16891 = v16865 * v90;
                                let v7145 = if v7144 > v0 { 1.0 } else { 0.0 };
                                let v7147: f64;
                                let v10074: Lanes<5>;
                                if v7145 != 0.0 {
                                    v7147 = v7144;
                                    v10074 = v16891;
                                } else {
                                    let v7146 = -v7144;
                                    let v16892 = v16891 * v10390;
                                    v7147 = v7146;
                                    v10074 = v16892;
                                }
                                let v16893 = v16890 * v7143;
                                let v7150 = ((v7143 * v7143) + v7147).sqrt();
                                let v16903 = v10410 * v79;
                                let v7156 = (v7132 - (v7132 - (v13 * (v7143 + v7150)))) + (v663 * v79);
                                let v16906 = ((v16865 - (v16865 - ((v16890 + (((v16893 + v16893) + v10074) * (v9368 / (v10435 * v7150)))) * v13))) + (Lanes([0.0, 0.0, v16903[0], 0.0, 0.0]))) * v7156;
                                let v7158 = v7133 + (v7156 * v7156);
                                let v7161 = ((v7158.ln()) - v7138) + v7140;
                                let v16912 = (((v16872 + (v16906 + v16906)) * (v9368 / v7158)) - v16881) + v16888;
                                let v16913 = v16912 - v10072;
                                let v7165 = (v7161 - v7162) - v7164;
                                let v7168 = (v90 * v7161) * v7167;
                                let v16915 = (v16912 * v90) * v7167;
                                let v7169 = if v7168 > v0 { 1.0 } else { 0.0 };
                                let v7171: f64;
                                let v10075: Lanes<5>;
                                if v7169 != 0.0 {
                                    v7171 = v7168;
                                    v10075 = v16915;
                                } else {
                                    let v7170 = -v7168;
                                    let v16916 = v16915 * v10390;
                                    v7171 = v7170;
                                    v10075 = v16916;
                                }
                                let v16917 = v16913 * v7165;
                                let v7174 = ((v7165 * v7165) + v7171).sqrt();
                                let v7177 = v7161 - (v13 * (v7165 + v7174));
                                let v16925 = v16912 - ((v16913 + (((v16917 + v16917) + v10075) * (v9368 / (v10435 * v7174)))) * v13);
                                v7178 = v7177;
                                v10073 = v16925;
                            } else {
                                v7178 = v7162;
                                v10073 = v10072;
                            }
                            let v7179 = v7178 / v663;
                            let v16926 = v10410 * v7179;
                            let v7180 = v7179 - v6979;
                            let v16930 = ((v10073 - (Lanes([0.0, 0.0, v16926[0], 0.0, 0.0]))) / v663) - v16767;
                            let v7183 = (-v7178).exp();
                            let v7184 = (v7178 - v1) + v7183;
                            let v16933 = v10073 + ((v10073 * v10390) * v7183);
                            let v7186 = if v7184 < v7185 { 1.0 } else { 0.0 };
                            let v7188: f64;
                            let v10076: Lanes<5>;
                            if v7186 != 0.0 {
                                v7188 = v7187;
                                v10076 = v10579;
                            } else {
                                v7188 = v7184;
                                v10076 = v16933;
                            }
                            let v7189 = v7188.sqrt();
                            let v7190 = v6921 * v7189;
                            let v16937 = v16670 * v7189;
                            let v16940 = (Lanes([0.0, 0.0, v16937[0], 0.0, 0.0])) + ((v10076 * (v9368 / (v10435 * v7189))) * v6921);
                            let v7192 = v127 * (v6986 - v7180);
                            let v16942 = (v16765 - v16930) * v127;
                            let v7193 = if v7123 == v1 { 1.0 } else { 0.0 };
                            let v7338: f64;
                            let v7340: f64;
                            let v7750: f64;
                            let v10077: Lanes<5>;
                            let v10078: Lanes<5>;
                            let v10079: Lanes<5>;
                            if v7193 != 0.0 {
                                let v16943 = v10410 * v6991;
                                let v16944 = v16737 * v663;
                                let v7195 = (v663 * v6991).exp();
                                let v16948 = ((Lanes([0.0, 0.0, v16943[0], 0.0])) + (Lanes([v16944[0], v16944[1], 0.0, v16944[2]]))) * v7195;
                                let v7196 = v731 / v39;
                                let v7197 = v7196 * v7196;
                                let v16950 = (v10453 / v39) * v7196;
                                let v16951 = v16950 + v16950;
                                let v7198 = v7197 * v7195;
                                let v16952 = v16951 * v7195;
                                let v16955 = (Lanes([0.0, 0.0, v16952[0], 0.0])) + (v16948 * v7197);
                                let mut v7199: f64 = 0.0;
                                let mut v7202: f64 = 0.0;
                                let mut v7288: f64 = 0.0;
                                let mut v7318: f64 = 0.0;
                                let mut v7321: f64 = 0.0;
                                let mut v7329: f64 = 0.0;
                                let mut v7332: f64 = 0.0;
                                let mut v10080: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10081: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10082: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10083: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10084: Lanes<5> = Lanes([0.0; 5]);
                                v7199 = v1;
                                v7202 = v7180;
                                v7288 = v0;
                                v7318 = v7178;
                                v7321 = v0;
                                v7329 = v0;
                                v7332 = v0;
                                v10080 = v16930;
                                v10081 = v10073;
                                v10082 = v10579;
                                v10083 = v10579;
                                v10084 = v10579;
                                loop {
                                    let v7201 = if v7199 <= v7200 { 1.0 } else { 0.0 };
                                    if v7201 == 0.0 {
                                        break;
                                    }
                                    let v7203 = v7202 + v6979;
                                    let v7204 = v663 * v7203;
                                    let v16976 = v10410 * v7203;
                                    let v16979 = (Lanes([0.0, 0.0, v16976[0], 0.0, 0.0])) + ((v10080 + v16767) * v663);
                                    let v7205 = if v7204 < v644 { 1.0 } else { 0.0 };
                                    let v7281: f64;
                                    let v7285: f64;
                                    let v7322: f64;
                                    let v7333: f64;
                                    let v10085: Lanes<5>;
                                    let v10086: Lanes<5>;
                                    let v10087: Lanes<5>;
                                    let v10088: Lanes<5>;
                                    if v7205 != 0.0 {
                                        let v7206 = v7204 * v7204;
                                        let v17021 = v16979 * v7204;
                                        let v17022 = v17021 + v17021;
                                        let v7207 = v7206 * v7204;
                                        let v7210 = v7208 + (v7204 * v6318);
                                        let v7212 = v6316 + (v7204 * v7210);
                                        let v7213 = v7207 * v7212;
                                        let v17032 = (((v17022 * v7204) + (v16979 * v7206)) * v7212) + (((v16979 * v7210) + ((v16979 * v6318) * v7204)) * v7207);
                                        let v7216 = v7204 * v644;
                                        let v17033 = v16979 * v644;
                                        let v7218 = v7215 + (v7216 * v6318);
                                        let v7220 = v7214 + (v7204 * v7218);
                                        let v7221 = v7206 * v7220;
                                        let v7222 = v7198 * v7213;
                                        let v17041 = v16955 * v7213;
                                        let v7223 = v7222 * v7213;
                                        let v17047 = (((Lanes([v17041[0], v17041[1], v17041[2], 0.0, v17041[3]])) + (v17032 * v7198)) * v7213) + (v17032 * v7222);
                                        let v17049 = v10410 * v7198;
                                        let v7225 = (v7198 * v663) * v78;
                                        let v7226 = v7225 * v7213;
                                        let v17053 = (((v16955 * v663) + (Lanes([0.0, 0.0, v17049[0], 0.0]))) * v78) * v7213;
                                        let v7231 = v7229 + (v7204 * v6342);
                                        let v7233 = v6340 + (v7204 * v7231);
                                        let v7235 = v7228 + (v7204 * v7233);
                                        let v7237 = v6338 + (v7204 * v7235);
                                        let v7238 = v7204 * v7237;
                                        let v17072 = (v16979 * v7237) + (((v16979 * v7235) + (((v16979 * v7233) + (((v16979 * v7231) + ((v16979 * v6342) * v7204)) * v7204)) * v7204)) * v7204);
                                        let v7243 = v7241 + (v7216 * v6342);
                                        let v7245 = v7240 + (v7204 * v7243);
                                        let v7247 = v7239 + (v7204 * v7245);
                                        let v7249 = v6338 + (v7204 * v7247);
                                        let v17083 = v17072 * v7238;
                                        let v7253 = (((v7238 * v7238) + v7223) + v362).sqrt();
                                        let v17088 = ((v17083 + v17083) + v17047) * (v9368 / (v10435 * v7253));
                                        let v17089 = v10410 * v7249;
                                        let v7255 = (v663 * v7249) * v78;
                                        let v7258 = v7253 + v7253;
                                        let v7259 = ((v7255 * v7238) + (v7226 * v7221)) / v7258;
                                        let v17101 = (((((((Lanes([0.0, 0.0, v17089[0], 0.0, 0.0])) + (((v16979 * v7247) + (((v16979 * v7245) + (((v16979 * v7243) + ((v17033 * v6342) * v7204)) * v7204)) * v7204)) * v663)) * v78) * v7238) + (v17072 * v7255)) + ((((Lanes([v17053[0], v17053[1], v17053[2], 0.0, v17053[3]])) + (v17032 * v7225)) * v7221) + (((v17022 * v7220) + (((v16979 * v7218) + ((v17033 * v6318) * v7204)) * v7206)) * v7226))) - ((v17088 + v17088) * v7259)) / v7258;
                                        v7281 = v7253;
                                        v7285 = v7259;
                                        v7322 = v7238;
                                        v7333 = v7223;
                                        v10085 = v17088;
                                        v10086 = v17101;
                                        v10087 = v17072;
                                        v10088 = v17047;
                                    } else {
                                        let v7260 = if v7204 < v2535 { 1.0 } else { 0.0 };
                                        let v7273: f64;
                                        let v7276: f64;
                                        let v10089: Lanes<5>;
                                        let v10090: Lanes<5>;
                                        if v7260 != 0.0 {
                                            let v7261 = v7204.exp();
                                            let v16998 = v16979 * v7261;
                                            let v7262 = v7261 - v1;
                                            let v7263 = v7198 * v7262;
                                            let v16999 = v16955 * v7262;
                                            let v17002 = (Lanes([v16999[0], v16999[1], v16999[2], 0.0, v16999[3]])) + (v16998 * v7198);
                                            let v7264 = v7198 * v663;
                                            let v17004 = v10410 * v7198;
                                            let v7265 = v7264 * v7261;
                                            let v17007 = ((v16955 * v663) + (Lanes([0.0, 0.0, v17004[0], 0.0]))) * v7261;
                                            let v17010 = (Lanes([v17007[0], v17007[1], v17007[2], 0.0, v17007[3]])) + (v16998 * v7264);
                                            v7273 = v7263;
                                            v7276 = v7265;
                                            v10089 = v17002;
                                            v10090 = v17010;
                                        } else {
                                            let v16980 = v10410 * v7202;
                                            let v7267 = (v663 * v7202).exp();
                                            let v16984 = ((Lanes([0.0, 0.0, v16980[0], 0.0, 0.0])) + (v10080 * v663)) * v7267;
                                            let v7268 = v7267 - v7195;
                                            let v7269 = v7197 * v7268;
                                            let v16987 = v16951 * v7268;
                                            let v16990 = (Lanes([0.0, 0.0, v16987[0], 0.0, 0.0])) + ((v16984 - (Lanes([v16948[0], v16948[1], v16948[2], 0.0, v16948[3]]))) * v7197);
                                            let v7270 = v7197 * v663;
                                            let v7271 = v7270 * v7267;
                                            let v16994 = ((v16951 * v663) + (v10410 * v7197)) * v7267;
                                            let v16997 = (Lanes([0.0, 0.0, v16994[0], 0.0, 0.0])) + (v16984 * v7270);
                                            v7273 = v7269;
                                            v7276 = v7271;
                                            v10089 = v16990;
                                            v10090 = v16997;
                                        }
                                        let v7275 = ((v7204 - v1) + v7273).sqrt();
                                        let v17014 = (v16979 + v10089) * (v9368 / (v10435 * v7275));
                                        let v7278 = (v663 + v7276) / v7275;
                                        let v7279 = v7278 * v13;
                                        let v17020 = ((((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + v10090) - (v17014 * v7278)) / v7275) * v13;
                                        v7281 = v7275;
                                        v7285 = v7279;
                                        v7322 = v0;
                                        v7333 = v7273;
                                        v10085 = v17014;
                                        v10086 = v17020;
                                        v10087 = v10579;
                                        v10088 = v10089;
                                    }
                                    let v17103 = v16722 * v7281;
                                    let v7283 = (v6986 - v7202) - (v6980 * v7281);
                                    let v17107 = (v16765 - v10080) - ((Lanes([0.0, 0.0, v17103[0], 0.0, 0.0])) + (v10085 * v6980));
                                    let v17108 = v16722 * v7285;
                                    let v7287 = v7284 - (v6980 * v7285);
                                    let v17112 = ((Lanes([0.0, 0.0, v17108[0], 0.0, 0.0])) + (v10086 * v6980)) * v10390;
                                    let v7289 = if v7288 == v1 { 1.0 } else { 0.0 };
                                    let v7312: f64;
                                    let v7314: f64;
                                    let v7315: f64;
                                    let v10091: Lanes<5>;
                                    if v7289 != 0.0 {
                                        v7312 = v7290;
                                        v7314 = v7202;
                                        v7315 = v7288;
                                        v10091 = v10080;
                                    } else {
                                        let v7292 = (-v7283) / v7287;
                                        let v17116 = ((v17107 * v10390) - (v17112 * v7292)) / v7287;
                                        let v7294 = v7202.abs();
                                        let v17120 = v10080 * ((v10435 * (if v7202 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                                        let v7295 = if v1 >= v7294 { 1.0 } else { 0.0 };
                                        let v7296: f64;
                                        let v10092: Lanes<5>;
                                        if v7295 != 0.0 {
                                            v7296 = v1;
                                            v10092 = v10579;
                                        } else {
                                            v7296 = v7294;
                                            v10092 = v17120;
                                        }
                                        let v7298 = v7293 * (v1 + v7296);
                                        let v17121 = v10092 * v7293;
                                        let v7300 = if (v7292.abs()) > v7298 { 1.0 } else { 0.0 };
                                        let v7305: f64;
                                        let v10093: Lanes<5>;
                                        if v7300 != 0.0 {
                                            let v7301 = if v7292 >= v0 { 1.0 } else { 0.0 };
                                            let v7303: f64;
                                            if v7301 != 0.0 {
                                                v7303 = v1;
                                            } else {
                                                v7303 = v7302;
                                            }
                                            let v7304 = v7298 * v7303;
                                            let v17122 = v17121 * v7303;
                                            v7305 = v7304;
                                            v10093 = v17122;
                                        } else {
                                            v7305 = v7292;
                                            v10093 = v17116;
                                        }
                                        let v7306 = v7202 + v7305;
                                        let v17123 = v10080 + v10093;
                                        let v7311 = if (if (v7305.abs()) <= v861 { 1.0 } else { 0.0 }) != 0.0 && (if (v7283.abs()) <= v3506 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v7316: f64;
                                        if v7311 != 0.0 {
                                            v7316 = v1;
                                        } else {
                                            v7316 = v7288;
                                        }
                                        v7312 = v7199;
                                        v7314 = v7306;
                                        v7315 = v7316;
                                        v10091 = v17123;
                                    }
                                    let v7313 = v7312 + v1;
                                    v7199 = v7313;
                                    v7202 = v7314;
                                    v7288 = v7315;
                                    v7318 = v7204;
                                    v7321 = v7322;
                                    v7329 = v7281;
                                    v7332 = v7333;
                                    v10080 = v10091;
                                    v10081 = v16979;
                                    v10082 = v10087;
                                    v10083 = v10085;
                                    v10084 = v10088;
                                }
                                let v7317 = if v7288 == v0 { 1.0 } else { 0.0 };
                                if v7317 != 0.0 {
                                } else {
                                }
                                let v7319 = if v7318 < v644 { 1.0 } else { 0.0 };
                                let v7327: f64;
                                let v10094: Lanes<5>;
                                if v7319 != 0.0 {
                                    let v7320 = if v7318 < v96 { 1.0 } else { 0.0 };
                                    if v7320 != 0.0 {
                                    } else {
                                    }
                                    let v7324 = v7321 + v7323;
                                    v7327 = v7324;
                                    v10094 = v10082;
                                } else {
                                    let v7326 = (v7318 - v1).sqrt();
                                    let v16958 = v10081 * (v9368 / (v10435 * v7326));
                                    v7327 = v7326;
                                    v10094 = v16958;
                                }
                                let v7328 = v6921 * v7327;
                                let v16959 = v16670 * v7327;
                                let v16962 = (Lanes([0.0, 0.0, v16959[0], 0.0, 0.0])) + (v10094 * v6921);
                                let v7330 = v7329 + v7327;
                                let v7331 = v1 / v7330;
                                let v7334 = v6921 * v7332;
                                let v16967 = v16670 * v7332;
                                let v7336 = v7328 + (v7334 * v7331);
                                let v16974 = v16962 + ((((Lanes([0.0, 0.0, v16967[0], 0.0, 0.0])) + (v10084 * v6921)) * v7331) + (((((v10083 + v10094) * v7331) * v10390) / v7330) * v7334));
                                v7338 = v7336;
                                v7340 = v7328;
                                v7750 = v7321;
                                v10077 = v16974;
                                v10078 = v16962;
                                v10079 = v10082;
                            } else {
                                v7338 = v7192;
                                v7340 = v7190;
                                v7750 = v0;
                                v10077 = v16942;
                                v10078 = v16940;
                                v10079 = v10579;
                            }
                            v7337 = v7338;
                            v7339 = v7340;
                            v7749 = v7750;
                            v10066 = v10077;
                            v10067 = v10078;
                            v10068 = v10079;
                        }
                        let v7343: f64;
                        if v565 != 0.0 {
                            let v7341 = v6899 * v6892;
                            v7343 = v7341;
                        } else {
                            let v7342 = v168 * v6892;
                            v7343 = v7342;
                        }
                        let v7347 = if (if v7344 != 0.0 && v9 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v6922 != 0.0 && v565 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v8627: f64;
                        let v8656: f64;
                        let v10095: Lanes<5>;
                        let v10096: Lanes<5>;
                        if v7347 != 0.0 {
                            let v7348 = v7343 * v7337;
                            let v17208 = v10066 * v7343;
                            let v7349 = v7343 * v7339;
                            let v17209 = v10067 * v7343;
                            v8627 = v7348;
                            v8656 = v7349;
                            v10095 = v17208;
                            v10096 = v17209;
                        } else {
                            v8627 = v0;
                            v8656 = v0;
                            v10095 = v10579;
                            v10096 = v10579;
                        }
                        let v7353 = if (if v7350 != 0.0 && v9 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v6923 != 0.0 && v565 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v8633: f64;
                        let v8645: f64;
                        let v10097: Lanes<5>;
                        let v10098: Lanes<5>;
                        if v7353 != 0.0 {
                            let v7354 = v7343 * v7337;
                            let v17210 = v10066 * v7343;
                            let v7355 = v7343 * v7339;
                            let v17211 = v10067 * v7343;
                            v8633 = v7354;
                            v8645 = v7355;
                            v10097 = v17210;
                            v10098 = v17211;
                        } else {
                            v8633 = v0;
                            v8645 = v0;
                            v10097 = v10579;
                            v10098 = v10579;
                        }
                        let v7394: f64;
                        let v7414: f64;
                        let v7773: f64;
                        let v7779: f64;
                        let v10099: Lanes<3>;
                        let v10100: Lanes<4>;
                        if v565 != 0.0 {
                            let v7361 = (v6046 * v835) + (v6048 * (v835 - v823));
                            let v17227 = (v9413 * v6046) + ((v9413 - v10560) * v6048);
                            let v17231 = (v9410 * v6046) + ((v9410 * v10390) * v6048);
                            let v17236 = (v9412 * v6046) + ((v9412 - (Lanes([v9410[0], v9410[1], 0.0]))) * v6048);
                            let v7371 = ((v6046 * v830) + (v6048 * (v830 - v823))) - v7361;
                            let v17241 = (Lanes([v17236[0], v17236[1], v17236[2], 0.0])) - (Lanes([v17227[0], v17227[1], 0.0, v17227[2]]));
                            let v7374 = (v7356 * v6046) + v6048;
                            let v7376 = (v7356 * v6048) + v6046;
                            let v17245 = ((v17227 * v10390) * v7374) + (((Lanes([v17231[0], v17231[1], 0.0])) - v17227) * v7376);
                            let v7381 = ((v7374 * (-v7361)) + (v7376 * (((v6046 * v823) + (v6048 * (-v823))) - v7361))) + v7380;
                            v7394 = v7381;
                            v7414 = v7371;
                            v7773 = v7374;
                            v7779 = v7376;
                            v10099 = v17245;
                            v10100 = v17241;
                        } else {
                            let v7383 = (v7356 * v6046) + v6048;
                            let v7385 = (v7356 * v6048) + v6046;
                            let v7416: f64;
                            let v10101: Lanes<4>;
                            if v7356 != 0.0 {
                                let v7389 = (v6046 * v830) + (v6048 * (v830 - v823));
                                let v17216 = (v9412 * v6046) + ((v9412 - (Lanes([v9410[0], v9410[1], 0.0]))) * v6048);
                                let v17217 = Lanes([v17216[0], v17216[1], v17216[2], 0.0]);
                                v7416 = v7389;
                                v10101 = v17217;
                            } else {
                                v7416 = v6982;
                                v10101 = v10062;
                            }
                            let v7415: f64;
                            let v10102: Lanes<4>;
                            if v7357 != 0.0 {
                                let v7393 = (v6048 * v830) + (v6046 * (v830 - v823));
                                let v17222 = (v9412 * v6048) + ((v9412 - (Lanes([v9410[0], v9410[1], 0.0]))) * v6046);
                                let v17223 = Lanes([v17222[0], v17222[1], v17222[2], 0.0]);
                                v7415 = v7393;
                                v10102 = v17223;
                            } else {
                                v7415 = v7416;
                                v10102 = v10101;
                            }
                            v7394 = v0;
                            v7414 = v7415;
                            v7773 = v7383;
                            v7779 = v7385;
                            v10099 = v10504;
                            v10100 = v10102;
                        }
                        let v7395 = -v7394;
                        let v17246 = v10099 * v10390;
                        let v7396 = if v7395 > v783 { 1.0 } else { 0.0 };
                        let v7411: f64;
                        let v10103: Lanes<3>;
                        if v7396 != 0.0 {
                            let v7398 = v779 - v783;
                            let v7399 = (v7395 - v783) / v7398;
                            let v17247 = v17246 / v7398;
                            let v7400 = v7399 * v7399;
                            let v17248 = v17247 * v7399;
                            let v17249 = v17248 + v17248;
                            let v17253 = v17249 * v7400;
                            let v7406 = (((v1 + v7399) + v7400) + (v7400 * v7399)) + (v7400 * v7400);
                            let v7407 = v1 / v7406;
                            let v17262 = (((((((v17247 + v17249) + ((v17249 * v7399) + (v17247 * v7400))) + (v17253 + v17253)) * v7407) * v10390) / v7406) * v10390) * v7398;
                            let v7410 = v783 + (v7398 * (v1 - v7407));
                            v7411 = v7410;
                            v10103 = v17262;
                        } else {
                            v7411 = v7395;
                            v10103 = v17246;
                        }
                        let v17263 = v10103 * v10390;
                        let v7413 = (-v7411) - v11;
                        let v17264 = v10100 * v10390;
                        let v7418 = (-v7414) + v66;
                        let v7419 = -v7413;
                        let v17265 = v17263 * v10390;
                        let v7420 = if v7418 < v7419 { 1.0 } else { 0.0 };
                        let v7766: f64;
                        let v7768: f64;
                        let v10104: Lanes<5>;
                        let v10105: Lanes<5>;
                        if v7420 != 0.0 {
                            let v7421 = v663 * v6921;
                            let v7422 = v1 / v7421;
                            let v7423 = v7422 * v127;
                            let v17658 = (((((v10410 * v6921) + (v16670 * v663)) * v7422) * v10390) / v7421) * v127;
                            let v17659 = v17658 * v7424;
                            let v7426 = v78 + (v7424 * v7423);
                            let v7427 = v91 * v7426;
                            let v7428 = v7427 * v7426;
                            let v7429 = v7428 * v7426;
                            let v17666 = ((((v17659 * v91) * v7426) + (v17659 * v7427)) * v7426) + (v17659 * v7428);
                            let v7430 = v661 - v6990;
                            let v17667 = v10406 - v16736;
                            let v7431 = v7418 + v7413;
                            let v17670 = v10410 * v7431;
                            let v17671 = (v17264 + (Lanes([v17263[0], v17263[1], 0.0, v17263[2]]))) * v663;
                            let v7434 = v3500 * v7423;
                            let v7435 = (v663 * v7431) - v78;
                            let v7436 = v7434 * v7435;
                            let v17676 = (v17658 * v3500) * v7435;
                            let v17679 = (Lanes([0.0, 0.0, v17676[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v17670[0], 0.0, 0.0])) + (Lanes([v17671[0], v17671[1], 0.0, v17671[2], v17671[3]]))) * v7434);
                            let v7437 = v7433 - v7436;
                            let v17680 = v17679 * v10390;
                            let v7438 = v7437 * v7437;
                            let v17681 = v17680 * v7437;
                            let v17682 = v17681 + v17681;
                            let v7440 = if v7429 < (v7438 * v3506) { 1.0 } else { 0.0 };
                            let v7452: f64;
                            let v10106: Lanes<5>;
                            if v7440 != 0.0 {
                                let v17689 = v17666 * v13;
                                let v7444 = (v13 * v7429) / v7437;
                                let v7446 = ((v7441 + v7437) + v7444) + v7436;
                                let v17695 = (v17680 + (((Lanes([0.0, 0.0, v17689[0], 0.0, 0.0])) - (v17680 * v7444)) / v7437)) + v17679;
                                v7452 = v7446;
                                v10106 = v17695;
                            } else {
                                let v7448 = (v7429 + v7438).sqrt();
                                let v7451 = (v7449 + v7448) + v7436;
                                let v17688 = (((Lanes([0.0, 0.0, v17666[0], 0.0, 0.0])) + v17682) * (v9368 / (v10435 * v7448))) + v17679;
                                v7452 = v7451;
                                v10106 = v17688;
                            }
                            let v7453 = v7452.powf(v1562);
                            let v17699 = v10106 * (v1562 * (v7452.powf(v17696)));
                            let v17701 = (v17658 * v3523) * v10390;
                            let v7459 = v748 * v7453;
                            let v7462 = (((v7454 - (v3523 * v7423)) + (v78 * v7453)) + (v7459 * v7453)) / v7453;
                            let v17714 = v10415 * v7462;
                            let v17717 = Lanes([v17263[0], v17263[1], 0.0, 0.0, v17263[2]]);
                            let v7465 = ((v7462 * v665) - v7413) + v7413;
                            let v17719 = ((((((((Lanes([0.0, 0.0, v17701[0], 0.0, 0.0])) + (v17699 * v78)) + (((v17699 * v748) * v7453) + (v17699 * v7459))) - (v17699 * v7462)) / v7453) * v665) + (Lanes([0.0, 0.0, v17714[0], 0.0, 0.0]))) - v17717) + v17717;
                            let v7466 = v7465 / v7430;
                            let v17720 = v17667 * v7466;
                            let v17724 = ((v17719 - (Lanes([0.0, 0.0, v17720[0], 0.0, 0.0]))) / v7430) * v7466;
                            let v7469 = (v1 + (v7466 * v7466)).sqrt();
                            let v7470 = v7465 / v7469;
                            let v7473 = v127 * (v7418 - (v7470 - v7413));
                            let v17735 = ((Lanes([v17264[0], v17264[1], 0.0, v17264[2], v17264[3]])) - (((v17719 - (((v17724 + v17724) * (v9368 / (v10435 * v7469))) * v7470)) / v7469) - v17717)) * v127;
                            v7766 = v7473;
                            v7768 = v7473;
                            v10104 = v17735;
                            v10105 = v17735;
                        } else {
                            let v7475 = v7418 + v7413;
                            let v17267 = v17264 + (Lanes([v17263[0], v17263[1], 0.0, v17263[2]]));
                            let v17268 = v10410 * v7475;
                            let v17269 = v17267 * v663;
                            let v17271 = Lanes([v17269[0], v17269[1], 0.0, v17269[2], v17269[3]]);
                            let v17272 = (Lanes([0.0, 0.0, v17268[0], 0.0, 0.0])) + v17271;
                            let v7477 = (v663 * v7475) - v1;
                            let v7480 = v6981 * v664;
                            let v17276 = (v16724 * v664) + (v10412 * v6981);
                            let v7481 = (v90 * (v7477 + v7474)) / v7480;
                            let v17277 = v17276 * v7481;
                            let v17280 = ((v17272 * v90) - (Lanes([0.0, 0.0, v17277[0], 0.0, 0.0]))) / v7480;
                            let v7482 = v1 + v7481;
                            let v7484 = if v7482 < v7483 { 1.0 } else { 0.0 };
                            let v7488: f64;
                            let v10107: Lanes<5>;
                            if v7484 != 0.0 {
                                v7488 = v7485;
                                v10107 = v10579;
                            } else {
                                v7488 = v7482;
                                v10107 = v17280;
                            }
                            let v7487 = (v6981 * v663) / v78;
                            let v17284 = ((v16724 * v663) + (v10410 * v6981)) / v78;
                            let v7489 = v7488.sqrt();
                            let v7490 = v1 - v7489;
                            let v17289 = v17284 * v7490;
                            let v17293 = Lanes([v17264[0], v17264[1], 0.0, v17264[2], v17264[3]]);
                            let v7493 = (v7418 + (v7487 * v7490)) + v7413;
                            let v17295 = Lanes([v17263[0], v17263[1], 0.0, 0.0, v17263[2]]);
                            let v17297 = v10410 * v7493;
                            let v7496 = (-(v663 * v7493)).exp();
                            let v7499 = (v90 * (v7477 + v7496)) / v7480;
                            let v17305 = v17276 * v7499;
                            let v17308 = (((v17272 + ((((Lanes([0.0, 0.0, v17297[0], 0.0, 0.0])) + (((v17293 + ((Lanes([0.0, 0.0, v17289[0], 0.0, 0.0])) + (((v10107 * (v9368 / (v10435 * v7489))) * v10390) * v7487))) + v17295) * v663)) * v10390) * v7496)) * v90) - (Lanes([0.0, 0.0, v17305[0], 0.0, 0.0]))) / v7480;
                            let v7500 = v1 + v7499;
                            let v7502 = if v7500 < v7501 { 1.0 } else { 0.0 };
                            let v7504: f64;
                            let v10108: Lanes<5>;
                            if v7502 != 0.0 {
                                v7504 = v7503;
                                v10108 = v10579;
                            } else {
                                v7504 = v7500;
                                v10108 = v17308;
                            }
                            let v7505 = v7504.sqrt();
                            let v7506 = v1 - v7505;
                            let v17313 = v17284 * v7506;
                            let v7509 = (v7418 + (v7487 * v7506)) + v7413;
                            let v7510 = v663 * v7509;
                            let v17319 = v10410 * v7509;
                            let v17322 = (Lanes([0.0, 0.0, v17319[0], 0.0, 0.0])) + (((v17293 + ((Lanes([0.0, 0.0, v17313[0], 0.0, 0.0])) + (((v10108 * (v9368 / (v10435 * v7505))) * v10390) * v7487))) + v17295) * v663);
                            let v7511 = if v7510 < v96 { 1.0 } else { 0.0 };
                            let v7589: f64;
                            let v10109: Lanes<5>;
                            if v7511 != 0.0 {
                                let v7514 = v663 * v6980;
                                let v7515 = v1 / v7514;
                                let v17328 = ((((v10410 * v6980) + (v16722 * v663)) * v7515) * v10390) / v7514;
                                let v7516 = v7513 + v7515;
                                let v17329 = v17267 * v10390;
                                let v7518 = (-v7475) / v6980;
                                let v17330 = v16722 * v7518;
                                let v17337 = ((v17328 * v7512) / v7521) * v10390;
                                let v7526 = (v7519 - ((v7512 * v7516) / v7521)) + (v7518 / v7524);
                                let v17340 = (Lanes([0.0, 0.0, v17337[0], 0.0, 0.0])) + ((((Lanes([v17329[0], v17329[1], 0.0, v17329[2], v17329[3]])) - (Lanes([0.0, 0.0, v17330[0], 0.0, 0.0]))) / v6980) / v7524);
                                let v7532 = ((v7527 * v7516) - v7529) / v7531;
                                let v17342 = (v17328 * v7527) / v7531;
                                let v17343 = v17340 * v7526;
                                let v7534 = v7532 * v7532;
                                let v17345 = v17342 * v7532;
                                let v17349 = ((v17345 + v17345) * v7532) + (v17342 * v7534);
                                let v7537 = ((v7526 * v7526) + (v7534 * v7532)).sqrt();
                                let v17354 = ((v17343 + v17343) + (Lanes([0.0, 0.0, v17349[0], 0.0, 0.0]))) * (v9368 / (v10435 * v7537));
                                let v7539 = (-v7526) + v7537;
                                let v7541 = v7526 + v7537;
                                let v7546 = ((v7539.powf(v1562)) + (-(v7541.powf(v1562)))) - v7545;
                                let v17369 = v10415 * v7546;
                                let v7549 = ((v7546 * v665) - v7413) + v7413;
                                let v7550 = v663 * v7549;
                                let v17374 = v10410 * v7549;
                                let v17377 = (Lanes([0.0, 0.0, v17374[0], 0.0, 0.0])) + (((((((((v17340 * v10390) + v17354) * (v1562 * (v7539.powf(v17357)))) + (((v17340 + v17354) * (v1562 * (v7541.powf(v17362)))) * v10390)) * v665) + (Lanes([0.0, 0.0, v17369[0], 0.0, 0.0]))) - v17295) + v17295) * v663);
                                v7589 = v7550;
                                v10109 = v17377;
                            } else {
                                v7589 = v7510;
                                v10109 = v17322;
                            }
                            let v7551 = if v7123 > v0 { 1.0 } else { 0.0 };
                            let v7605: f64;
                            let v10110: Lanes<5>;
                            if v7551 != 0.0 {
                                let v7552 = v7475 + v79;
                                let v17378 = v10410 * v7419;
                                let v17379 = v17265 * v663;
                                let v7554 = (v663 * v7419).exp();
                                let v7555 = v7554 + v362;
                                let v7556 = v731 / v39;
                                let v7557 = v7556 * v7556;
                                let v17385 = (v10453 / v39) * v7556;
                                let v17386 = v17385 + v17385;
                                let v7558 = v7557 * v7555;
                                let v17387 = v17386 * v7555;
                                let v7559 = v663 * v7552;
                                let v17391 = v10410 * v7552;
                                let v17393 = (Lanes([0.0, 0.0, v17391[0], 0.0, 0.0])) + v17271;
                                let v7560 = v7558 * v7480;
                                let v17395 = v17276 * v7558;
                                let v17397 = (((Lanes([0.0, 0.0, v17387[0], 0.0])) + ((((Lanes([0.0, 0.0, v17378[0], 0.0])) + (Lanes([v17379[0], v17379[1], 0.0, v17379[2]]))) * v7554) * v7557)) * v7480) + (Lanes([0.0, 0.0, v17395[0], 0.0]));
                                let v17398 = v17393 * v7559;
                                let v7562 = v7560 + (v7559 * v7559);
                                let v17400 = Lanes([v17397[0], v17397[1], v17397[2], 0.0, v17397[3]]);
                                let v7564 = v7557 * v7480;
                                let v7565 = v7564.ln();
                                let v17408 = ((v17386 * v7480) + (v17276 * v7557)) * (v9368 / v7564);
                                let v17409 = Lanes([0.0, 0.0, v17408[0], 0.0, 0.0]);
                                let v7567 = v663 * v7413;
                                let v17411 = v10410 * v7413;
                                let v17412 = v17263 * v663;
                                let v17415 = (Lanes([0.0, 0.0, v17411[0], 0.0])) + (Lanes([v17412[0], v17412[1], 0.0, v17412[2]]));
                                let v17416 = Lanes([v17415[0], v17415[1], v17415[2], 0.0, v17415[3]]);
                                let v17418 = v17393 - ((((v17400 + (v17398 + v17398)) * (v9368 / v7562)) - v17409) + v17416);
                                let v7570 = (v7559 - (((v7562.ln()) - v7565) + v7567)) - v1;
                                let v7571 = v90 * v7559;
                                let v17419 = v17393 * v90;
                                let v7572 = if v7571 > v0 { 1.0 } else { 0.0 };
                                let v7574: f64;
                                let v10111: Lanes<5>;
                                if v7572 != 0.0 {
                                    v7574 = v7571;
                                    v10111 = v17419;
                                } else {
                                    let v7573 = -v7571;
                                    let v17420 = v17419 * v10390;
                                    v7574 = v7573;
                                    v10111 = v17420;
                                }
                                let v17421 = v17418 * v7570;
                                let v7577 = ((v7570 * v7570) + v7574).sqrt();
                                let v17431 = v10410 * v79;
                                let v7583 = (v7559 - (v7559 - (v13 * (v7570 + v7577)))) + (v663 * v79);
                                let v17434 = ((v17393 - (v17393 - ((v17418 + (((v17421 + v17421) + v10111) * (v9368 / (v10435 * v7577)))) * v13))) + (Lanes([0.0, 0.0, v17431[0], 0.0, 0.0]))) * v7583;
                                let v7585 = v7560 + (v7583 * v7583);
                                let v7588 = ((v7585.ln()) - v7565) + v7567;
                                let v17440 = (((v17400 + (v17434 + v17434)) * (v9368 / v7585)) - v17409) + v17416;
                                let v17441 = v17440 - v10109;
                                let v7592 = (v7588 - v7589) - v7591;
                                let v7595 = (v90 * v7588) * v7594;
                                let v17443 = (v17440 * v90) * v7594;
                                let v7596 = if v7595 > v0 { 1.0 } else { 0.0 };
                                let v7598: f64;
                                let v10112: Lanes<5>;
                                if v7596 != 0.0 {
                                    v7598 = v7595;
                                    v10112 = v17443;
                                } else {
                                    let v7597 = -v7595;
                                    let v17444 = v17443 * v10390;
                                    v7598 = v7597;
                                    v10112 = v17444;
                                }
                                let v17445 = v17441 * v7592;
                                let v7601 = ((v7592 * v7592) + v7598).sqrt();
                                let v7604 = v7588 - (v13 * (v7592 + v7601));
                                let v17453 = v17440 - ((v17441 + (((v17445 + v17445) + v10112) * (v9368 / (v10435 * v7601)))) * v13);
                                v7605 = v7604;
                                v10110 = v17453;
                            } else {
                                v7605 = v7589;
                                v10110 = v10109;
                            }
                            let v7606 = v7605 / v663;
                            let v17454 = v10410 * v7606;
                            let v7607 = v7606 - v7413;
                            let v17458 = ((v10110 - (Lanes([0.0, 0.0, v17454[0], 0.0, 0.0]))) / v663) - v17295;
                            let v7610 = (-v7605).exp();
                            let v7611 = (v7605 - v1) + v7610;
                            let v17461 = v10110 + ((v10110 * v10390) * v7610);
                            let v7613 = if v7611 < v7612 { 1.0 } else { 0.0 };
                            let v7615: f64;
                            let v10113: Lanes<5>;
                            if v7613 != 0.0 {
                                v7615 = v7614;
                                v10113 = v10579;
                            } else {
                                v7615 = v7611;
                                v10113 = v17461;
                            }
                            let v7616 = v7615.sqrt();
                            let v7617 = v6921 * v7616;
                            let v17465 = v16670 * v7616;
                            let v17468 = (Lanes([0.0, 0.0, v17465[0], 0.0, 0.0])) + ((v10113 * (v9368 / (v10435 * v7616))) * v6921);
                            let v7619 = v127 * (v7418 - v7607);
                            let v17470 = (v17293 - v17458) * v127;
                            let v7620 = if v7123 == v1 { 1.0 } else { 0.0 };
                            let v7767: f64;
                            let v7769: f64;
                            let v10114: Lanes<5>;
                            let v10115: Lanes<5>;
                            if v7620 != 0.0 {
                                let v17471 = v10410 * v7419;
                                let v17472 = v17265 * v663;
                                let v7622 = (v663 * v7419).exp();
                                let v17476 = ((Lanes([0.0, 0.0, v17471[0], 0.0])) + (Lanes([v17472[0], v17472[1], 0.0, v17472[2]]))) * v7622;
                                let v7623 = v731 / v39;
                                let v7624 = v7623 * v7623;
                                let v17478 = (v10453 / v39) * v7623;
                                let v17479 = v17478 + v17478;
                                let v7625 = v7624 * v7622;
                                let v17480 = v17479 * v7622;
                                let v17483 = (Lanes([0.0, 0.0, v17480[0], 0.0])) + (v17476 * v7624);
                                let mut v7626: f64 = 0.0;
                                let mut v7629: f64 = 0.0;
                                let mut v7715: f64 = 0.0;
                                let mut v7745: f64 = 0.0;
                                let mut v7748: f64 = 0.0;
                                let mut v7758: f64 = 0.0;
                                let mut v7761: f64 = 0.0;
                                let mut v10116: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10117: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10118: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10119: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10120: Lanes<5> = Lanes([0.0; 5]);
                                v7626 = v1;
                                v7629 = v7607;
                                v7715 = v0;
                                v7745 = v7605;
                                v7748 = v7749;
                                v7758 = v0;
                                v7761 = v0;
                                v10116 = v17458;
                                v10117 = v10110;
                                v10118 = v10068;
                                v10119 = v10579;
                                v10120 = v10579;
                                loop {
                                    let v7628 = if v7626 <= v7627 { 1.0 } else { 0.0 };
                                    if v7628 == 0.0 {
                                        break;
                                    }
                                    let v7630 = v7629 + v7413;
                                    let v7631 = v663 * v7630;
                                    let v17504 = v10410 * v7630;
                                    let v17507 = (Lanes([0.0, 0.0, v17504[0], 0.0, 0.0])) + ((v10116 + v17295) * v663);
                                    let v7632 = if v7631 < v644 { 1.0 } else { 0.0 };
                                    let v7708: f64;
                                    let v7712: f64;
                                    let v7751: f64;
                                    let v7762: f64;
                                    let v10121: Lanes<5>;
                                    let v10122: Lanes<5>;
                                    let v10123: Lanes<5>;
                                    let v10124: Lanes<5>;
                                    if v7632 != 0.0 {
                                        let v7633 = v7631 * v7631;
                                        let v17549 = v17507 * v7631;
                                        let v17550 = v17549 + v17549;
                                        let v7634 = v7633 * v7631;
                                        let v7637 = v7635 + (v7631 * v6318);
                                        let v7639 = v6316 + (v7631 * v7637);
                                        let v7640 = v7634 * v7639;
                                        let v17560 = (((v17550 * v7631) + (v17507 * v7633)) * v7639) + (((v17507 * v7637) + ((v17507 * v6318) * v7631)) * v7634);
                                        let v7643 = v7631 * v644;
                                        let v17561 = v17507 * v644;
                                        let v7645 = v7642 + (v7643 * v6318);
                                        let v7647 = v7641 + (v7631 * v7645);
                                        let v7648 = v7633 * v7647;
                                        let v7649 = v7625 * v7640;
                                        let v17569 = v17483 * v7640;
                                        let v7650 = v7649 * v7640;
                                        let v17575 = (((Lanes([v17569[0], v17569[1], v17569[2], 0.0, v17569[3]])) + (v17560 * v7625)) * v7640) + (v17560 * v7649);
                                        let v17577 = v10410 * v7625;
                                        let v7652 = (v7625 * v663) * v78;
                                        let v7653 = v7652 * v7640;
                                        let v17581 = (((v17483 * v663) + (Lanes([0.0, 0.0, v17577[0], 0.0]))) * v78) * v7640;
                                        let v7658 = v7656 + (v7631 * v6342);
                                        let v7660 = v6340 + (v7631 * v7658);
                                        let v7662 = v7655 + (v7631 * v7660);
                                        let v7664 = v6338 + (v7631 * v7662);
                                        let v7665 = v7631 * v7664;
                                        let v17600 = (v17507 * v7664) + (((v17507 * v7662) + (((v17507 * v7660) + (((v17507 * v7658) + ((v17507 * v6342) * v7631)) * v7631)) * v7631)) * v7631);
                                        let v7670 = v7668 + (v7643 * v6342);
                                        let v7672 = v7667 + (v7631 * v7670);
                                        let v7674 = v7666 + (v7631 * v7672);
                                        let v7676 = v6338 + (v7631 * v7674);
                                        let v17611 = v17600 * v7665;
                                        let v7680 = (((v7665 * v7665) + v7650) + v362).sqrt();
                                        let v17616 = ((v17611 + v17611) + v17575) * (v9368 / (v10435 * v7680));
                                        let v17617 = v10410 * v7676;
                                        let v7682 = (v663 * v7676) * v78;
                                        let v7685 = v7680 + v7680;
                                        let v7686 = ((v7682 * v7665) + (v7653 * v7648)) / v7685;
                                        let v17629 = (((((((Lanes([0.0, 0.0, v17617[0], 0.0, 0.0])) + (((v17507 * v7674) + (((v17507 * v7672) + (((v17507 * v7670) + ((v17561 * v6342) * v7631)) * v7631)) * v7631)) * v663)) * v78) * v7665) + (v17600 * v7682)) + ((((Lanes([v17581[0], v17581[1], v17581[2], 0.0, v17581[3]])) + (v17560 * v7652)) * v7648) + (((v17550 * v7647) + (((v17507 * v7645) + ((v17561 * v6318) * v7631)) * v7633)) * v7653))) - ((v17616 + v17616) * v7686)) / v7685;
                                        v7708 = v7680;
                                        v7712 = v7686;
                                        v7751 = v7665;
                                        v7762 = v7650;
                                        v10121 = v17616;
                                        v10122 = v17629;
                                        v10123 = v17600;
                                        v10124 = v17575;
                                    } else {
                                        let v7687 = if v7631 < v2535 { 1.0 } else { 0.0 };
                                        let v7700: f64;
                                        let v7703: f64;
                                        let v10125: Lanes<5>;
                                        let v10126: Lanes<5>;
                                        if v7687 != 0.0 {
                                            let v7688 = v7631.exp();
                                            let v17526 = v17507 * v7688;
                                            let v7689 = v7688 - v1;
                                            let v7690 = v7625 * v7689;
                                            let v17527 = v17483 * v7689;
                                            let v17530 = (Lanes([v17527[0], v17527[1], v17527[2], 0.0, v17527[3]])) + (v17526 * v7625);
                                            let v7691 = v7625 * v663;
                                            let v17532 = v10410 * v7625;
                                            let v7692 = v7691 * v7688;
                                            let v17535 = ((v17483 * v663) + (Lanes([0.0, 0.0, v17532[0], 0.0]))) * v7688;
                                            let v17538 = (Lanes([v17535[0], v17535[1], v17535[2], 0.0, v17535[3]])) + (v17526 * v7691);
                                            v7700 = v7690;
                                            v7703 = v7692;
                                            v10125 = v17530;
                                            v10126 = v17538;
                                        } else {
                                            let v17508 = v10410 * v7629;
                                            let v7694 = (v663 * v7629).exp();
                                            let v17512 = ((Lanes([0.0, 0.0, v17508[0], 0.0, 0.0])) + (v10116 * v663)) * v7694;
                                            let v7695 = v7694 - v7622;
                                            let v7696 = v7624 * v7695;
                                            let v17515 = v17479 * v7695;
                                            let v17518 = (Lanes([0.0, 0.0, v17515[0], 0.0, 0.0])) + ((v17512 - (Lanes([v17476[0], v17476[1], v17476[2], 0.0, v17476[3]]))) * v7624);
                                            let v7697 = v7624 * v663;
                                            let v7698 = v7697 * v7694;
                                            let v17522 = ((v17479 * v663) + (v10410 * v7624)) * v7694;
                                            let v17525 = (Lanes([0.0, 0.0, v17522[0], 0.0, 0.0])) + (v17512 * v7697);
                                            v7700 = v7696;
                                            v7703 = v7698;
                                            v10125 = v17518;
                                            v10126 = v17525;
                                        }
                                        let v7702 = ((v7631 - v1) + v7700).sqrt();
                                        let v17542 = (v17507 + v10125) * (v9368 / (v10435 * v7702));
                                        let v7705 = (v663 + v7703) / v7702;
                                        let v7706 = v7705 * v13;
                                        let v17548 = ((((Lanes([0.0, 0.0, v10410[0], 0.0, 0.0])) + v10126) - (v17542 * v7705)) / v7702) * v13;
                                        v7708 = v7702;
                                        v7712 = v7706;
                                        v7751 = v0;
                                        v7762 = v7700;
                                        v10121 = v17542;
                                        v10122 = v17548;
                                        v10123 = v10579;
                                        v10124 = v10125;
                                    }
                                    let v17631 = v16722 * v7708;
                                    let v7710 = (v7418 - v7629) - (v6980 * v7708);
                                    let v17635 = (v17293 - v10116) - ((Lanes([0.0, 0.0, v17631[0], 0.0, 0.0])) + (v10121 * v6980));
                                    let v17636 = v16722 * v7712;
                                    let v7714 = v7711 - (v6980 * v7712);
                                    let v17640 = ((Lanes([0.0, 0.0, v17636[0], 0.0, 0.0])) + (v10122 * v6980)) * v10390;
                                    let v7716 = if v7715 == v1 { 1.0 } else { 0.0 };
                                    let v7739: f64;
                                    let v7741: f64;
                                    let v7742: f64;
                                    let v10127: Lanes<5>;
                                    if v7716 != 0.0 {
                                        v7739 = v7717;
                                        v7741 = v7629;
                                        v7742 = v7715;
                                        v10127 = v10116;
                                    } else {
                                        let v7719 = (-v7710) / v7714;
                                        let v17644 = ((v17635 * v10390) - (v17640 * v7719)) / v7714;
                                        let v7721 = v7629.abs();
                                        let v17648 = v10116 * ((v10435 * (if v7629 >= v11304 { 1.0 } else { 0.0 })) - v9368);
                                        let v7722 = if v1 >= v7721 { 1.0 } else { 0.0 };
                                        let v7723: f64;
                                        let v10128: Lanes<5>;
                                        if v7722 != 0.0 {
                                            v7723 = v1;
                                            v10128 = v10579;
                                        } else {
                                            v7723 = v7721;
                                            v10128 = v17648;
                                        }
                                        let v7725 = v7720 * (v1 + v7723);
                                        let v17649 = v10128 * v7720;
                                        let v7727 = if (v7719.abs()) > v7725 { 1.0 } else { 0.0 };
                                        let v7732: f64;
                                        let v10129: Lanes<5>;
                                        if v7727 != 0.0 {
                                            let v7728 = if v7719 >= v0 { 1.0 } else { 0.0 };
                                            let v7730: f64;
                                            if v7728 != 0.0 {
                                                v7730 = v1;
                                            } else {
                                                v7730 = v7729;
                                            }
                                            let v7731 = v7725 * v7730;
                                            let v17650 = v17649 * v7730;
                                            v7732 = v7731;
                                            v10129 = v17650;
                                        } else {
                                            v7732 = v7719;
                                            v10129 = v17644;
                                        }
                                        let v7733 = v7629 + v7732;
                                        let v17651 = v10116 + v10129;
                                        let v7738 = if (if (v7732.abs()) <= v861 { 1.0 } else { 0.0 }) != 0.0 && (if (v7710.abs()) <= v3506 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v7743: f64;
                                        if v7738 != 0.0 {
                                            v7743 = v1;
                                        } else {
                                            v7743 = v7715;
                                        }
                                        v7739 = v7626;
                                        v7741 = v7733;
                                        v7742 = v7743;
                                        v10127 = v17651;
                                    }
                                    let v7740 = v7739 + v1;
                                    v7626 = v7740;
                                    v7629 = v7741;
                                    v7715 = v7742;
                                    v7745 = v7631;
                                    v7748 = v7751;
                                    v7758 = v7708;
                                    v7761 = v7762;
                                    v10116 = v10127;
                                    v10117 = v17507;
                                    v10118 = v10123;
                                    v10119 = v10121;
                                    v10120 = v10124;
                                }
                                let v7744 = if v7715 == v0 { 1.0 } else { 0.0 };
                                if v7744 != 0.0 {
                                } else {
                                }
                                let v7746 = if v7745 < v644 { 1.0 } else { 0.0 };
                                let v7756: f64;
                                let v10130: Lanes<5>;
                                if v7746 != 0.0 {
                                    let v7747 = if v7745 < v96 { 1.0 } else { 0.0 };
                                    if v7747 != 0.0 {
                                    } else {
                                    }
                                    let v7753 = v7748 + v7752;
                                    v7756 = v7753;
                                    v10130 = v10118;
                                } else {
                                    let v7755 = (v7745 - v1).sqrt();
                                    let v17486 = v10117 * (v9368 / (v10435 * v7755));
                                    v7756 = v7755;
                                    v10130 = v17486;
                                }
                                let v7757 = v6921 * v7756;
                                let v17487 = v16670 * v7756;
                                let v17490 = (Lanes([0.0, 0.0, v17487[0], 0.0, 0.0])) + (v10130 * v6921);
                                let v7759 = v7758 + v7756;
                                let v7760 = v1 / v7759;
                                let v7763 = v6921 * v7761;
                                let v17495 = v16670 * v7761;
                                let v7765 = v7757 + (v7763 * v7760);
                                let v17502 = v17490 + ((((Lanes([0.0, 0.0, v17495[0], 0.0, 0.0])) + (v10120 * v6921)) * v7760) + (((((v10119 + v10130) * v7760) * v10390) / v7759) * v7763));
                                v7767 = v7765;
                                v7769 = v7757;
                                v10114 = v17502;
                                v10115 = v17490;
                            } else {
                                v7767 = v7619;
                                v7769 = v7617;
                                v10114 = v17470;
                                v10115 = v17468;
                            }
                            v7766 = v7767;
                            v7768 = v7769;
                            v10104 = v10114;
                            v10105 = v10115;
                        }
                        let v7772: f64;
                        if v565 != 0.0 {
                            let v7770 = v6899 * v6892;
                            v7772 = v7770;
                        } else {
                            let v7771 = v168 * v6892;
                            v7772 = v7771;
                        }
                        let v7776 = if (if v7773 != 0.0 && v9 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v7356 != 0.0 && v565 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v8626: f64;
                        let v8655: f64;
                        let v10131: Lanes<5>;
                        let v10132: Lanes<5>;
                        if v7776 != 0.0 {
                            let v7777 = v7772 * v7766;
                            let v17736 = v10104 * v7772;
                            let v7778 = v7772 * v7768;
                            let v17737 = v10105 * v7772;
                            v8626 = v7777;
                            v8655 = v7778;
                            v10131 = v17736;
                            v10132 = v17737;
                        } else {
                            v8626 = v8627;
                            v8655 = v8656;
                            v10131 = v10095;
                            v10132 = v10096;
                        }
                        let v7782 = if (if v7779 != 0.0 && v9 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v7357 != 0.0 && v565 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v8632: f64;
                        let v8644: f64;
                        let v10133: Lanes<5>;
                        let v10134: Lanes<5>;
                        if v7782 != 0.0 {
                            let v7783 = v7772 * v7766;
                            let v17738 = v10104 * v7772;
                            let v7784 = v7772 * v7768;
                            let v17739 = v10105 * v7772;
                            v8632 = v7783;
                            v8644 = v7784;
                            v10133 = v17738;
                            v10134 = v17739;
                        } else {
                            v8632 = v8633;
                            v8644 = v8645;
                            v10133 = v10097;
                            v10134 = v10098;
                        }
                        v7801 = v0;
                        v7820 = v0;
                        v8625 = v8626;
                        v8631 = v8632;
                        v8643 = v8644;
                        v8654 = v8655;
                        v10055 = v11062;
                        v10056 = v11062;
                        v10057 = v10131;
                        v10058 = v10133;
                        v10059 = v10134;
                        v10060 = v10132;
                    }
                    let v7787 = (v6048 * v370) + (v6046 * v369);
                    let v8456: f64;
                    let v10135: Lanes<6>;
                    if v7787 != 0.0 {
                        let v7792 = (v6048 * v7788) + (v6046 * v7790);
                        let v7802: f64;
                        if v565 != 0.0 {
                            let v7798 = v7792 * (-((v6048 * v6899) + (v6046 * v7794)));
                            v7802 = v7798;
                        } else {
                            let v7800 = v7792 * (-v168);
                            v7802 = v7800;
                        }
                        let v7803 = -v7802;
                        let v17764 = (v9412 - (Lanes([v9410[0], v9410[1], 0.0]))) * v7803;
                        let v7806 = v7801 + (v7803 * (v830 - v823));
                        let v17766 = v10055 + (Lanes([v17764[0], v17764[1], 0.0, v17764[2], 0.0, 0.0]));
                        v8456 = v7806;
                        v10135 = v17766;
                    } else {
                        v8456 = v7801;
                        v10135 = v10055;
                    }
                    let v7809 = (v6046 * v370) + (v6048 * v369);
                    let v8460: f64;
                    let v10136: Lanes<6>;
                    if v7809 != 0.0 {
                        let v7812 = (v6046 * v7788) + (v6048 * v7790);
                        let v7821: f64;
                        if v565 != 0.0 {
                            let v7817 = v7812 * (-((v6046 * v6899) + (v6048 * v7794)));
                            v7821 = v7817;
                        } else {
                            let v7819 = v7812 * (-v168);
                            v7821 = v7819;
                        }
                        let v7822 = -v7821;
                        let v17767 = v9412 * v7822;
                        let v7824 = v7820 + (v7822 * v830);
                        let v17769 = v10056 + (Lanes([v17767[0], v17767[1], 0.0, v17767[2], 0.0, 0.0]));
                        v8460 = v7824;
                        v10136 = v17769;
                    } else {
                        v8460 = v7820;
                        v10136 = v10056;
                    }
                    v8455 = v8456;
                    v8459 = v8460;
                    v8624 = v8625;
                    v8630 = v8631;
                    v8642 = v8643;
                    v8653 = v8654;
                    v10049 = v10135;
                    v10050 = v10136;
                    v10051 = v10057;
                    v10052 = v10058;
                    v10053 = v10059;
                    v10054 = v10060;
                } else {
                    let v7826 = if v7825 == v1 { 1.0 } else { 0.0 };
                    let v7827 = if v369 == 0.0 { 1.0 } else { 0.0 };
                    let v7829 = if v7825 != v1 { 1.0 } else { 0.0 };
                    let v7830 = if v370 == 0.0 { 1.0 } else { 0.0 };
                    let v7832 = if (if v7826 != 0.0 && v7827 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v7829 != 0.0 && v7830 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7849: f64;
                    if v7832 != 0.0 {
                        let v7850: f64;
                        if v565 != 0.0 {
                            let v7835 = ((-v127) * v6892) * v7794;
                            v7850 = v7835;
                        } else {
                            let v7838 = ((-v127) * v6892) * v168;
                            v7850 = v7838;
                        }
                        v7849 = v7850;
                    } else {
                        let v7841 = (v6048 * v7788) + (v6046 * v7790);
                        let v7851: f64;
                        if v565 != 0.0 {
                            let v7846 = v7841 * (-((v6048 * v6899) + (v6046 * v7794)));
                            v7851 = v7846;
                        } else {
                            let v7848 = v7841 * (-v168);
                            v7851 = v7848;
                        }
                        v7849 = v7851;
                    }
                    let v7852 = -v7849;
                    let v7854 = v7852 * (v830 - v823);
                    let v16666 = (v9412 - (Lanes([v9410[0], v9410[1], 0.0]))) * v7852;
                    let v7857 = if (if v7826 != 0.0 && v7830 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v7829 != 0.0 && v7827 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7874: f64;
                    if v7857 != 0.0 {
                        let v7875: f64;
                        if v565 != 0.0 {
                            let v7860 = ((-v127) * v6892) * v6899;
                            v7875 = v7860;
                        } else {
                            let v7863 = ((-v127) * v6892) * v168;
                            v7875 = v7863;
                        }
                        v7874 = v7875;
                    } else {
                        let v7866 = (v6046 * v7788) + (v6048 * v7790);
                        let v7876: f64;
                        if v565 != 0.0 {
                            let v7871 = v7866 * (-((v6046 * v6899) + (v6048 * v7794)));
                            v7876 = v7871;
                        } else {
                            let v7873 = v7866 * (-v168);
                            v7876 = v7873;
                        }
                        v7874 = v7876;
                    }
                    let v7877 = -v7874;
                    let v7878 = v7877 * v830;
                    let v16667 = v9412 * v7877;
                    let v16668 = Lanes([v16666[0], v16666[1], 0.0, v16666[2], 0.0, 0.0]);
                    let v16669 = Lanes([v16667[0], v16667[1], 0.0, v16667[2], 0.0, 0.0]);
                    v8455 = v7854;
                    v8459 = v7878;
                    v8624 = v0;
                    v8630 = v0;
                    v8642 = v0;
                    v8653 = v0;
                    v10049 = v16668;
                    v10050 = v16669;
                    v10051 = v10579;
                    v10052 = v10579;
                    v10053 = v10579;
                    v10054 = v10579;
                }
                v8454 = v8455;
                v8458 = v8459;
                v8623 = v8624;
                v8629 = v8630;
                v8641 = v8642;
                v8652 = v8653;
                v10043 = v10049;
                v10044 = v10050;
                v10045 = v10051;
                v10046 = v10052;
                v10047 = v10053;
                v10048 = v10054;
            } else {
                v8454 = v0;
                v8458 = v0;
                v8623 = v0;
                v8629 = v0;
                v8641 = v0;
                v8652 = v0;
                v10043 = v11062;
                v10044 = v11062;
                v10045 = v10579;
                v10046 = v10579;
                v10047 = v10579;
                v10048 = v10579;
            }
            let v8673: f64;
            let v8674: f64;
            let v8675: f64;
            let v8677: f64;
            let v10137: Lanes<3>;
            let v10138: Lanes<3>;
            let v10139: Lanes<2>;
            let v10140: Lanes<2>;
            if v565 != 0.0 {
                let v7884 = (v121 * v209) - (v661 * v663);
                let v17775 = ((v10406 * v663) + (v10410 * v661)) * v10390;
                let v7886 = v699.ln();
                let v17777 = v10416 * (v9368 / v699);
                let v7891 = ((v7884 + (v7885 * v7886)) / v7889).exp();
                let v7892 = v7881 * v7891;
                let v17782 = (((v17775 + (v17777 * v7885)) / v7889) * v7891) * v7881;
                let v7897 = ((v7884 + (v7893 * v7886)) / v7889).exp();
                let v7898 = v7881 * v7897;
                let v17787 = (((v17775 + (v17777 * v7893)) / v7889) * v7897) * v7881;
                let v7900 = v7899 * v12;
                let v7901 = v7900 * v7892;
                let v17788 = v17782 * v7900;
                let v7902 = v7900 * v7898;
                let v17789 = v17787 * v7900;
                let v7904 = v7903 * v12;
                let v7905 = v7904 * v7892;
                let v17790 = v17782 * v7904;
                let v7906 = v7904 * v7898;
                let v17791 = v17787 * v7904;
                let v17792 = v10416 * v699;
                let v7908 = v7901 + v362;
                let v7909 = v7905 + v362;
                let v7910 = v7889 / v663;
                let v17796 = ((v10410 * v7910) * v10390) / v663;
                let v7912 = v7911 * (v699 * v699);
                let v17797 = (v17792 + v17792) * v7911;
                let v7913 = v7912 / v7908;
                let v7914 = v1 + v7913;
                let v7915 = v7914.ln();
                let v7916 = v7910 * v7915;
                let v17805 = (v17796 * v7915) + ((((v17797 - (v17788 * v7913)) / v7908) * (v9368 / v7914)) * v7910);
                let v7917 = v7912 / v7909;
                let v7918 = v1 + v7917;
                let v7919 = v7918.ln();
                let v7920 = v7910 * v7919;
                let v17813 = (v17796 * v7919) + ((((v17797 - (v17790 * v7917)) / v7909) * (v9368 / v7918)) * v7910);
                let v7921 = v7889 * v665;
                let v17814 = v10415 * v7889;
                let v7922 = if v7879 < v7916 { 1.0 } else { 0.0 };
                let v7936: f64;
                let v10141: Lanes<3>;
                if v7922 != 0.0 {
                    let v7923 = v7879 / v7921;
                    let v17837 = v17814 * v7923;
                    let v7924 = v7923.exp();
                    let v7925 = v7924 - v1;
                    let v7926 = v7901 * v7925;
                    let v17843 = v17788 * v7925;
                    let v17846 = (Lanes([0.0, v17843[0], 0.0])) + (((((Lanes([v9386[0], 0.0, v9386[1]])) - (Lanes([0.0, v17837[0], 0.0]))) / v7921) * v7924) * v7901);
                    v7936 = v7926;
                    v10141 = v17846;
                } else {
                    let v7927 = v7916 / v7921;
                    let v7928 = v7927.exp();
                    let v17818 = ((v17805 - (v17814 * v7927)) / v7921) * v7928;
                    let v7929 = v7928 - v1;
                    let v17821 = (v17788 * v7929) + (v17818 * v7901);
                    let v7931 = v7901 / v7921;
                    let v7932 = v7931 * v7928;
                    let v7933 = v7879 - v7916;
                    let v17831 = ((((v17788 - (v17814 * v7931)) / v7921) * v7928) + (v17818 * v7931)) * v7933;
                    let v7935 = (v7901 * v7929) + (v7932 * v7933);
                    let v17836 = (Lanes([0.0, v17821[0], 0.0])) + ((Lanes([0.0, v17831[0], 0.0])) + (((Lanes([v9386[0], 0.0, v9386[1]])) - (Lanes([0.0, v17805[0], 0.0]))) * v7932));
                    v7936 = v7935;
                    v10141 = v17836;
                }
                let v7938 = v7937 * v7879;
                let v17848 = (v9386 * v7937) * v7902;
                let v17849 = v17789 * v7938;
                let v7940 = v7936 + (v7938 * v7902);
                let v17853 = v10141 + ((Lanes([v17848[0], 0.0, v17848[1]])) + (Lanes([0.0, v17849[0], 0.0])));
                let v7941 = if v7880 < v7920 { 1.0 } else { 0.0 };
                let v7955: f64;
                let v10142: Lanes<3>;
                if v7941 != 0.0 {
                    let v7942 = v7880 / v7921;
                    let v17876 = v17814 * v7942;
                    let v7943 = v7942.exp();
                    let v7944 = v7943 - v1;
                    let v7945 = v7905 * v7944;
                    let v17882 = v17790 * v7944;
                    let v17885 = (Lanes([0.0, v17882[0], 0.0])) + (((((Lanes([v9387[0], 0.0, v9387[1]])) - (Lanes([0.0, v17876[0], 0.0]))) / v7921) * v7943) * v7905);
                    v7955 = v7945;
                    v10142 = v17885;
                } else {
                    let v7946 = v7920 / v7921;
                    let v7947 = v7946.exp();
                    let v17857 = ((v17813 - (v17814 * v7946)) / v7921) * v7947;
                    let v7948 = v7947 - v1;
                    let v17860 = (v17790 * v7948) + (v17857 * v7905);
                    let v7950 = v7905 / v7921;
                    let v7951 = v7950 * v7947;
                    let v7952 = v7880 - v7920;
                    let v17870 = ((((v17790 - (v17814 * v7950)) / v7921) * v7947) + (v17857 * v7950)) * v7952;
                    let v7954 = (v7905 * v7948) + (v7951 * v7952);
                    let v17875 = (Lanes([0.0, v17860[0], 0.0])) + ((Lanes([0.0, v17870[0], 0.0])) + (((Lanes([v9387[0], 0.0, v9387[1]])) - (Lanes([0.0, v17813[0], 0.0]))) * v7951));
                    v7955 = v7954;
                    v10142 = v17875;
                }
                let v7956 = v7937 * v7880;
                let v17887 = (v9387 * v7937) * v7906;
                let v17888 = v17791 * v7956;
                let v17893 = v9386 * v381;
                let v7960 = v7940 + (v381 * v7879);
                let v17895 = v17853 + (Lanes([v17893[0], 0.0, v17893[1]]));
                let v17896 = v9387 * v381;
                let v7962 = (v7955 + (v7956 * v7906)) + (v381 * v7880);
                let v17898 = (v10142 + ((Lanes([v17887[0], 0.0, v17887[1]])) + (Lanes([0.0, v17888[0], 0.0])))) + (Lanes([v17896[0], 0.0, v17896[1]]));
                let v7965 = v7963 * v7964;
                let v7967 = v7963 * v7966;
                let v7969 = v12 - v7968;
                let v7970 = if v7969 <= v0 { 1.0 } else { 0.0 };
                let v7979: f64;
                let v8099: f64;
                if v7970 != 0.0 {
                    v7979 = v0;
                    v8099 = v0;
                } else {
                    v7979 = v7967;
                    v8099 = v7965;
                }
                let v7972 = if v7971 > v6899 { 1.0 } else { 0.0 };
                let v8214: f64;
                let v10143: Lanes<2>;
                if v7972 != 0.0 {
                    let v7975 = v7973 * (v7971 - v6899);
                    let v7977 = v7976 * v6899;
                    let v7978 = if v7880 < v0 { 1.0 } else { 0.0 };
                    let v8215: f64;
                    let v10144: Lanes<2>;
                    if v7978 != 0.0 {
                        let v7980 = if v7979 > v0 { 1.0 } else { 0.0 };
                        let v8007: f64;
                        let v10145: Lanes<2>;
                        if v7980 != 0.0 {
                            let v7983 = v1 - (v7880 / v7981);
                            let v17947 = (v9387 / v7981) * v10390;
                            let v7985 = if v7984 == v13 { 1.0 } else { 0.0 };
                            let v7991: f64;
                            let v10146: Lanes<2>;
                            if v7985 != 0.0 {
                                let v7986 = v7983.sqrt();
                                let v7987 = v1 / v7986;
                                let v17957 = (((v17947 * (v9368 / (v10435 * v7986))) * v7987) * v10390) / v7986;
                                v7991 = v7987;
                                v10146 = v17957;
                            } else {
                                let v7988 = -v7984;
                                let v7989 = v7983.powf(v7988);
                                let v17951 = v17947 * (v7988 * (v7983.powf((v7988 - v9368))));
                                v7991 = v7989;
                                v10146 = v17951;
                            }
                            let v7990 = v7981 * v7979;
                            let v7995 = v1 - v7984;
                            let v7996 = (v7990 * (v1 - (v7983 * v7991))) / v7995;
                            let v17963 = ((((v17947 * v7991) + (v10146 * v7983)) * v10390) * v7990) / v7995;
                            v8007 = v7996;
                            v10145 = v17963;
                        } else {
                            v8007 = v0;
                            v10145 = v10381;
                        }
                        let v7997 = if v7975 > v0 { 1.0 } else { 0.0 };
                        let v8026: f64;
                        let v10147: Lanes<2>;
                        if v7997 != 0.0 {
                            let v8000 = v1 - (v7880 / v7998);
                            let v17965 = (v9387 / v7998) * v10390;
                            let v8002 = if v8001 == v13 { 1.0 } else { 0.0 };
                            let v8009: f64;
                            let v10148: Lanes<2>;
                            if v8002 != 0.0 {
                                let v8003 = v8000.sqrt();
                                let v8004 = v1 / v8003;
                                let v17975 = (((v17965 * (v9368 / (v10435 * v8003))) * v8004) * v10390) / v8003;
                                v8009 = v8004;
                                v10148 = v17975;
                            } else {
                                let v8005 = -v8001;
                                let v8006 = v8000.powf(v8005);
                                let v17969 = v17965 * (v8005 * (v8000.powf((v8005 - v9368))));
                                v8009 = v8006;
                                v10148 = v17969;
                            }
                            let v8008 = v7998 * v7975;
                            let v8013 = v1 - v8001;
                            let v8015 = v8007 + ((v8008 * (v1 - (v8000 * v8009))) / v8013);
                            let v17982 = v10145 + (((((v17965 * v8009) + (v10148 * v8000)) * v10390) * v8008) / v8013);
                            v8026 = v8015;
                            v10147 = v17982;
                        } else {
                            v8026 = v8007;
                            v10147 = v10145;
                        }
                        let v8016 = if v7977 > v0 { 1.0 } else { 0.0 };
                        let v8216: f64;
                        let v10149: Lanes<2>;
                        if v8016 != 0.0 {
                            let v8019 = v1 - (v7880 / v8017);
                            let v17984 = (v9387 / v8017) * v10390;
                            let v8021 = if v8020 == v13 { 1.0 } else { 0.0 };
                            let v8028: f64;
                            let v10150: Lanes<2>;
                            if v8021 != 0.0 {
                                let v8022 = v8019.sqrt();
                                let v8023 = v1 / v8022;
                                let v17994 = (((v17984 * (v9368 / (v10435 * v8022))) * v8023) * v10390) / v8022;
                                v8028 = v8023;
                                v10150 = v17994;
                            } else {
                                let v8024 = -v8020;
                                let v8025 = v8019.powf(v8024);
                                let v17988 = v17984 * (v8024 * (v8019.powf((v8024 - v9368))));
                                v8028 = v8025;
                                v10150 = v17988;
                            }
                            let v8027 = v8017 * v7977;
                            let v8032 = v1 - v8020;
                            let v8034 = v8026 + ((v8027 * (v1 - (v8019 * v8028))) / v8032);
                            let v18001 = v10147 + (((((v17984 * v8028) + (v10150 * v8019)) * v10390) * v8027) / v8032);
                            v8216 = v8034;
                            v10149 = v18001;
                        } else {
                            v8216 = v8026;
                            v10149 = v10147;
                        }
                        v8215 = v8216;
                        v10144 = v10149;
                    } else {
                        let v8044 = (((v7979 * v7984) / v7981) + ((v7975 * v8001) / v7998)) + ((v7977 * v8020) / v8017);
                        let v8047 = ((v7979 + v7975) + v7977) + ((v7880 * v13) * v8044);
                        let v8048 = v7880 * v8047;
                        let v17945 = (v9387 * v8047) + (((v9387 * v13) * v8044) * v7880);
                        v8215 = v8048;
                        v10144 = v17945;
                    }
                    v8214 = v8215;
                    v10143 = v10144;
                } else {
                    let v8049 = v7976 * v7971;
                    let v8050 = if v7880 < v0 { 1.0 } else { 0.0 };
                    let v8217: f64;
                    let v10151: Lanes<2>;
                    if v8050 != 0.0 {
                        let v8051 = if v7979 > v0 { 1.0 } else { 0.0 };
                        let v8074: f64;
                        let v10152: Lanes<2>;
                        if v8051 != 0.0 {
                            let v8053 = v1 - (v7880 / v7981);
                            let v17905 = (v9387 / v7981) * v10390;
                            let v8054 = if v7984 == v13 { 1.0 } else { 0.0 };
                            let v8060: f64;
                            let v10153: Lanes<2>;
                            if v8054 != 0.0 {
                                let v8055 = v8053.sqrt();
                                let v8056 = v1 / v8055;
                                let v17915 = (((v17905 * (v9368 / (v10435 * v8055))) * v8056) * v10390) / v8055;
                                v8060 = v8056;
                                v10153 = v17915;
                            } else {
                                let v8057 = -v7984;
                                let v8058 = v8053.powf(v8057);
                                let v17909 = v17905 * (v8057 * (v8053.powf((v8057 - v9368))));
                                v8060 = v8058;
                                v10153 = v17909;
                            }
                            let v8059 = v7981 * v7979;
                            let v8064 = v1 - v7984;
                            let v8065 = (v8059 * (v1 - (v8053 * v8060))) / v8064;
                            let v17921 = ((((v17905 * v8060) + (v10153 * v8053)) * v10390) * v8059) / v8064;
                            v8074 = v8065;
                            v10152 = v17921;
                        } else {
                            v8074 = v0;
                            v10152 = v10381;
                        }
                        let v8066 = if v8049 > v0 { 1.0 } else { 0.0 };
                        let v8218: f64;
                        let v10154: Lanes<2>;
                        if v8066 != 0.0 {
                            let v8068 = v1 - (v7880 / v8017);
                            let v17923 = (v9387 / v8017) * v10390;
                            let v8069 = if v8020 == v13 { 1.0 } else { 0.0 };
                            let v8076: f64;
                            let v10155: Lanes<2>;
                            if v8069 != 0.0 {
                                let v8070 = v8068.sqrt();
                                let v8071 = v1 / v8070;
                                let v17933 = (((v17923 * (v9368 / (v10435 * v8070))) * v8071) * v10390) / v8070;
                                v8076 = v8071;
                                v10155 = v17933;
                            } else {
                                let v8072 = -v8020;
                                let v8073 = v8068.powf(v8072);
                                let v17927 = v17923 * (v8072 * (v8068.powf((v8072 - v9368))));
                                v8076 = v8073;
                                v10155 = v17927;
                            }
                            let v8075 = v8017 * v8049;
                            let v8080 = v1 - v8020;
                            let v8082 = v8074 + ((v8075 * (v1 - (v8068 * v8076))) / v8080);
                            let v17940 = v10152 + (((((v17923 * v8076) + (v10155 * v8068)) * v10390) * v8075) / v8080);
                            v8218 = v8082;
                            v10154 = v17940;
                        } else {
                            v8218 = v8074;
                            v10154 = v10152;
                        }
                        v8217 = v8218;
                        v10151 = v10154;
                    } else {
                        let v8088 = ((v7979 * v7984) / v7981) + ((v8049 * v8020) / v8017);
                        let v8091 = (v7979 + v8049) + ((v7880 * v13) * v8088);
                        let v8092 = v7880 * v8091;
                        let v17903 = (v9387 * v8091) + (((v9387 * v13) * v8088) * v7880);
                        v8217 = v8092;
                        v10151 = v17903;
                    }
                    v8214 = v8217;
                    v10143 = v10151;
                }
                let v8094 = if v8093 > v7794 { 1.0 } else { 0.0 };
                let v8242: f64;
                let v10156: Lanes<2>;
                if v8094 != 0.0 {
                    let v8096 = v7973 * (v8093 - v7794);
                    let v8097 = v7976 * v7794;
                    let v8098 = if v7879 < v0 { 1.0 } else { 0.0 };
                    let v8243: f64;
                    let v10157: Lanes<2>;
                    if v8098 != 0.0 {
                        let v8100 = if v8099 > v0 { 1.0 } else { 0.0 };
                        let v8123: f64;
                        let v10158: Lanes<2>;
                        if v8100 != 0.0 {
                            let v8102 = v1 - (v7879 / v7981);
                            let v18050 = (v9386 / v7981) * v10390;
                            let v8103 = if v7984 == v13 { 1.0 } else { 0.0 };
                            let v8109: f64;
                            let v10159: Lanes<2>;
                            if v8103 != 0.0 {
                                let v8104 = v8102.sqrt();
                                let v8105 = v1 / v8104;
                                let v18060 = (((v18050 * (v9368 / (v10435 * v8104))) * v8105) * v10390) / v8104;
                                v8109 = v8105;
                                v10159 = v18060;
                            } else {
                                let v8106 = -v7984;
                                let v8107 = v8102.powf(v8106);
                                let v18054 = v18050 * (v8106 * (v8102.powf((v8106 - v9368))));
                                v8109 = v8107;
                                v10159 = v18054;
                            }
                            let v8108 = v7981 * v8099;
                            let v8113 = v1 - v7984;
                            let v8114 = (v8108 * (v1 - (v8102 * v8109))) / v8113;
                            let v18066 = ((((v18050 * v8109) + (v10159 * v8102)) * v10390) * v8108) / v8113;
                            v8123 = v8114;
                            v10158 = v18066;
                        } else {
                            v8123 = v0;
                            v10158 = v10380;
                        }
                        let v8115 = if v8096 > v0 { 1.0 } else { 0.0 };
                        let v8140: f64;
                        let v10160: Lanes<2>;
                        if v8115 != 0.0 {
                            let v8117 = v1 - (v7879 / v7998);
                            let v18068 = (v9386 / v7998) * v10390;
                            let v8118 = if v8001 == v13 { 1.0 } else { 0.0 };
                            let v8125: f64;
                            let v10161: Lanes<2>;
                            if v8118 != 0.0 {
                                let v8119 = v8117.sqrt();
                                let v8120 = v1 / v8119;
                                let v18078 = (((v18068 * (v9368 / (v10435 * v8119))) * v8120) * v10390) / v8119;
                                v8125 = v8120;
                                v10161 = v18078;
                            } else {
                                let v8121 = -v8001;
                                let v8122 = v8117.powf(v8121);
                                let v18072 = v18068 * (v8121 * (v8117.powf((v8121 - v9368))));
                                v8125 = v8122;
                                v10161 = v18072;
                            }
                            let v8124 = v7998 * v8096;
                            let v8129 = v1 - v8001;
                            let v8131 = v8123 + ((v8124 * (v1 - (v8117 * v8125))) / v8129);
                            let v18085 = v10158 + (((((v18068 * v8125) + (v10161 * v8117)) * v10390) * v8124) / v8129);
                            v8140 = v8131;
                            v10160 = v18085;
                        } else {
                            v8140 = v8123;
                            v10160 = v10158;
                        }
                        let v8132 = if v8097 > v0 { 1.0 } else { 0.0 };
                        let v8244: f64;
                        let v10162: Lanes<2>;
                        if v8132 != 0.0 {
                            let v8134 = v1 - (v7879 / v8017);
                            let v18087 = (v9386 / v8017) * v10390;
                            let v8135 = if v8020 == v13 { 1.0 } else { 0.0 };
                            let v8142: f64;
                            let v10163: Lanes<2>;
                            if v8135 != 0.0 {
                                let v8136 = v8134.sqrt();
                                let v8137 = v1 / v8136;
                                let v18097 = (((v18087 * (v9368 / (v10435 * v8136))) * v8137) * v10390) / v8136;
                                v8142 = v8137;
                                v10163 = v18097;
                            } else {
                                let v8138 = -v8020;
                                let v8139 = v8134.powf(v8138);
                                let v18091 = v18087 * (v8138 * (v8134.powf((v8138 - v9368))));
                                v8142 = v8139;
                                v10163 = v18091;
                            }
                            let v8141 = v8017 * v8097;
                            let v8146 = v1 - v8020;
                            let v8148 = v8140 + ((v8141 * (v1 - (v8134 * v8142))) / v8146);
                            let v18104 = v10160 + (((((v18087 * v8142) + (v10163 * v8134)) * v10390) * v8141) / v8146);
                            v8244 = v8148;
                            v10162 = v18104;
                        } else {
                            v8244 = v8140;
                            v10162 = v10160;
                        }
                        v8243 = v8244;
                        v10157 = v10162;
                    } else {
                        let v8158 = (((v8099 * v7984) / v7981) + ((v8096 * v8001) / v7998)) + ((v8097 * v8020) / v8017);
                        let v8161 = ((v8099 + v8096) + v8097) + ((v7879 * v13) * v8158);
                        let v8162 = v7879 * v8161;
                        let v18048 = (v9386 * v8161) + (((v9386 * v13) * v8158) * v7879);
                        v8243 = v8162;
                        v10157 = v18048;
                    }
                    v8242 = v8243;
                    v10156 = v10157;
                } else {
                    let v8163 = v7976 * v8093;
                    let v8164 = if v7879 < v0 { 1.0 } else { 0.0 };
                    let v8245: f64;
                    let v10164: Lanes<2>;
                    if v8164 != 0.0 {
                        let v8165 = if v8099 > v0 { 1.0 } else { 0.0 };
                        let v8188: f64;
                        let v10165: Lanes<2>;
                        if v8165 != 0.0 {
                            let v8167 = v1 - (v7879 / v7981);
                            let v18008 = (v9386 / v7981) * v10390;
                            let v8168 = if v7984 == v13 { 1.0 } else { 0.0 };
                            let v8174: f64;
                            let v10166: Lanes<2>;
                            if v8168 != 0.0 {
                                let v8169 = v8167.sqrt();
                                let v8170 = v1 / v8169;
                                let v18018 = (((v18008 * (v9368 / (v10435 * v8169))) * v8170) * v10390) / v8169;
                                v8174 = v8170;
                                v10166 = v18018;
                            } else {
                                let v8171 = -v7984;
                                let v8172 = v8167.powf(v8171);
                                let v18012 = v18008 * (v8171 * (v8167.powf((v8171 - v9368))));
                                v8174 = v8172;
                                v10166 = v18012;
                            }
                            let v8173 = v7981 * v8099;
                            let v8178 = v1 - v7984;
                            let v8179 = (v8173 * (v1 - (v8167 * v8174))) / v8178;
                            let v18024 = ((((v18008 * v8174) + (v10166 * v8167)) * v10390) * v8173) / v8178;
                            v8188 = v8179;
                            v10165 = v18024;
                        } else {
                            v8188 = v0;
                            v10165 = v10380;
                        }
                        let v8180 = if v8163 > v0 { 1.0 } else { 0.0 };
                        let v8246: f64;
                        let v10167: Lanes<2>;
                        if v8180 != 0.0 {
                            let v8182 = v1 - (v7879 / v8017);
                            let v18026 = (v9386 / v8017) * v10390;
                            let v8183 = if v8020 == v13 { 1.0 } else { 0.0 };
                            let v8190: f64;
                            let v10168: Lanes<2>;
                            if v8183 != 0.0 {
                                let v8184 = v8182.sqrt();
                                let v8185 = v1 / v8184;
                                let v18036 = (((v18026 * (v9368 / (v10435 * v8184))) * v8185) * v10390) / v8184;
                                v8190 = v8185;
                                v10168 = v18036;
                            } else {
                                let v8186 = -v8020;
                                let v8187 = v8182.powf(v8186);
                                let v18030 = v18026 * (v8186 * (v8182.powf((v8186 - v9368))));
                                v8190 = v8187;
                                v10168 = v18030;
                            }
                            let v8189 = v8017 * v8163;
                            let v8194 = v1 - v8020;
                            let v8196 = v8188 + ((v8189 * (v1 - (v8182 * v8190))) / v8194);
                            let v18043 = v10165 + (((((v18026 * v8190) + (v10168 * v8182)) * v10390) * v8189) / v8194);
                            v8246 = v8196;
                            v10167 = v18043;
                        } else {
                            v8246 = v8188;
                            v10167 = v10165;
                        }
                        v8245 = v8246;
                        v10164 = v10167;
                    } else {
                        let v8202 = ((v8099 * v7984) / v7981) + ((v8163 * v8020) / v8017);
                        let v8205 = (v8099 + v8163) + ((v7879 * v13) * v8202);
                        let v8206 = v7879 * v8205;
                        let v18006 = (v9386 * v8205) + (((v9386 * v13) * v8202) * v7879);
                        v8245 = v8206;
                        v10164 = v18006;
                    }
                    v8242 = v8245;
                    v10156 = v10164;
                }
                let v8207 = if v7979 > v0 { 1.0 } else { 0.0 };
                let v8678: f64;
                let v10169: Lanes<2>;
                if v8207 != 0.0 {
                    let v8212 = -(((v8208 * v477) * v7969) * v7966);
                    let v8213 = v529 * v8212;
                    let v18106 = (v10143 * v10390) * v10390;
                    let v8221 = (v8212 - (-v8214)) - v8213;
                    let v8223 = (v90 * v8212) * v8213;
                    let v8224 = if v8223 > v0 { 1.0 } else { 0.0 };
                    let v8226: f64;
                    if v8224 != 0.0 {
                        v8226 = v8223;
                    } else {
                        let v8225 = -v8223;
                        v8226 = v8225;
                    }
                    let v18107 = v18106 * v8221;
                    let v8229 = ((v8221 * v8221) + v8226).sqrt();
                    let v8234 = (v8212 - (v13 * (v8221 + v8229))) * v8233;
                    let v18115 = (((v18106 + ((v18107 + v18107) * (v9368 / (v10435 * v8229)))) * v13) * v10390) * v8233;
                    v8678 = v8234;
                    v10169 = v18115;
                } else {
                    v8678 = v8214;
                    v10169 = v10143;
                }
                let v8235 = if v8099 > v0 { 1.0 } else { 0.0 };
                let v8676: f64;
                let v10170: Lanes<2>;
                if v8235 != 0.0 {
                    let v8240 = -(((v8236 * v477) * v7969) * v7964);
                    let v8241 = v529 * v8240;
                    let v18117 = (v10156 * v10390) * v10390;
                    let v8249 = (v8240 - (-v8242)) - v8241;
                    let v8251 = (v90 * v8240) * v8241;
                    let v8252 = if v8251 > v0 { 1.0 } else { 0.0 };
                    let v8254: f64;
                    if v8252 != 0.0 {
                        v8254 = v8251;
                    } else {
                        let v8253 = -v8251;
                        v8254 = v8253;
                    }
                    let v18118 = v18117 * v8249;
                    let v8257 = ((v8249 * v8249) + v8254).sqrt();
                    let v8262 = (v8240 - (v13 * (v8249 + v8257))) * v8261;
                    let v18126 = (((v18117 + ((v18118 + v18118) * (v9368 / (v10435 * v8257)))) * v13) * v10390) * v8261;
                    v8676 = v8262;
                    v10170 = v18126;
                } else {
                    v8676 = v8242;
                    v10170 = v10156;
                }
                v8673 = v7962;
                v8674 = v7960;
                v8675 = v8676;
                v8677 = v8678;
                v10137 = v17898;
                v10138 = v17895;
                v10139 = v10170;
                v10140 = v10169;
            } else {
                v8673 = v0;
                v8674 = v0;
                v8675 = v0;
                v8677 = v0;
                v10137 = v17770;
                v10138 = v17771;
                v10139 = v10380;
                v10140 = v10381;
            }
            let v8984: f64;
            let v8989: f64;
            let v10171: Lanes<6>;
            let v10172: Lanes<4>;
            if v71 != 0.0 {
                let v8985: f64;
                let v10173: Lanes<6>;
                if v5714 != 0.0 {
                    let v8266 = v8263 * v8264;
                    let v8267 = v8266 * v8265;
                    let v8271 = v8264 * v8265;
                    let v8274 = (((v5780 * v4843) * v8263) + (v8271 * v8265)) + v362;
                    let v8275 = (v8267 * v8265) / v8274;
                    let v18142 = ((((v9781 * v8266) * v8265) + (v9781 * v8267)) - (((((v9779 * v4843) + (v9452 * v5780)) * v8263) + (((v9781 * v8264) * v8265) + (v9781 * v8271))) * v8275)) / v8274;
                    v8985 = v8275;
                    v10173 = v18142;
                } else {
                    let v8276 = v8263 + v362;
                    v8985 = v8276;
                    v10173 = v11062;
                }
                let v8278 = v8277 * v1128;
                let v18143 = v9421 * v8277;
                v8984 = v8985;
                v8989 = v8278;
                v10171 = v10173;
                v10172 = v18143;
            } else {
                v8984 = v0;
                v8989 = v0;
                v10171 = v11062;
                v10172 = v10625;
            }
            let v8281 = if v4325 == 0.0 { 1.0 } else { 0.0 };
            let v8282 = if (if v8279 != v0 { 1.0 } else { 0.0 }) != 0.0 && v8281 != 0.0 { 1.0 } else { 0.0 };
            if v8282 != 0.0 {
                let v8283 = v4348 / v206;
                let v8295 = if (((((((v8284 * v8285) / v206) / v8288) / v168) - v8283) - v8283).abs()) > v8294 { 1.0 } else { 0.0 };
                if v8295 != 0.0 {
                } else {
                }
            } else {
            }
            let v8296 = if v4840 != v0 { 1.0 } else { 0.0 };
            let v8297 = if v8296 != 0.0 && v8281 != 0.0 { 1.0 } else { 0.0 };
            let v8401: f64;
            let v8729: f64;
            let v10174: Lanes<6>;
            let v10175: Lanes<6>;
            if v8297 != 0.0 {
                let v8308 = (v8298 - v4340) / v8265;
                let v8311 = (v8309 * v8308) / v4388;
                let v18151 = ((v9784 * v8308) + ((((v10038 - v9441) - (v9781 * v8308)) / v8265) * v8309)) / v4388;
                let v8316 = if (if v8312 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v8314 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8324: f64;
                let v10176: Lanes<6>;
                if v8316 != 0.0 {
                    v8324 = v1;
                    v10176 = v11062;
                } else {
                    let v8321 = if (if v8317 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v8319 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8325: f64;
                    let v10177: Lanes<6>;
                    if v8321 != 0.0 {
                        v8325 = v8311;
                        v10177 = v18151;
                    } else {
                        let v8322 = v4548 - v1;
                        let v8323 = v8311.powf(v8322);
                        let v18155 = v18151 * (v8322 * (v8311.powf((v8322 - v9368))));
                        v8325 = v8323;
                        v10177 = v18155;
                    }
                    v8324 = v8325;
                    v10176 = v10177;
                }
                let v18158 = (v18151 * v8324) + (v10176 * v8311);
                let v8327 = v1 + (v8311 * v8324);
                let v8330 = (v8328 / v4548) - v1;
                let v8331 = v8327.powf(v8330);
                let v8332 = v8327 * v8331;
                let v8333 = v8309 * v8332;
                let v18168 = (v9784 * v8332) + (((v18158 * v8331) + ((v18158 * (v8330 * (v8327.powf((v8330 - v9368))))) * v8327)) * v8309);
                let v8335 = (v5780 + v8333) / v78;
                let v18170 = (v9779 + v18168) / v78;
                let v8336 = v4307 * v4307;
                let v18171 = v9437 * v4307;
                let v18172 = v18171 + v18171;
                let v8337 = v166 * v1128;
                let v8338 = v8337 * v4843;
                let v18174 = (v9421 * v166) * v4843;
                let v8339 = v8338 * v5780;
                let v8340 = v96 * v4307;
                let v18181 = v9437 * v96;
                let v8343 = (v1 + v8340) + (v646 * v8336);
                let v8344 = v8343 * v8333;
                let v8349 = (v96 + (v90 * v4307)) + (v96 * v8336);
                let v8350 = v8349 * v8333;
                let v8354 = (v646 + v8340) + v8336;
                let v8355 = v8354 * v5780;
                let v8357 = ((v8344 * v8333) + (v8350 * v5780)) + (v8355 * v5780);
                let v8360 = v8359 * v8265;
                let v8361 = v1 + v4307;
                let v8362 = v8360 * v8361;
                let v8363 = v8362 * v8335;
                let v8364 = v8363 * v8335;
                let v8365 = (v8339 * v8357) / v8364;
                let v18223 = (((((((Lanes([v18174[0], v18174[1], 0.0, v18174[2], v18174[3], 0.0])) + (v9452 * v8337)) * v5780) + (v9779 * v8338)) * v8357) + ((((((((v18181 + (v18172 * v646)) * v8333) + (v18168 * v8343)) * v8333) + (v18168 * v8344)) + ((((((v9437 * v90) + (v18172 * v96)) * v8333) + (v18168 * v8349)) * v5780) + (v9779 * v8350))) + (((((v18181 + v18172) * v5780) + (v9779 * v8354)) * v5780) + (v9779 * v8355))) * v8339)) - ((((((((v9781 * v8359) * v8361) + (v9437 * v8360)) * v8335) + (v18170 * v8362)) * v8335) + (v18170 * v8363)) * v8365)) / v8364;
                v8401 = v8365;
                v8729 = v8333;
                v10174 = v18223;
                v10175 = v18168;
            } else {
                v8401 = v0;
                v8729 = v0;
                v10174 = v11062;
                v10175 = v11062;
            }
            let v8373 = if (if (if (if v4838 != v0 { 1.0 } else { 0.0 }) != 0.0 && v8296 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v8368 == v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v8281 != 0.0 { 1.0 } else { 0.0 };
            let v8721: f64;
            let v8734: f64;
            let v8743: f64;
            let v8747: f64;
            let v10178: Lanes<6>;
            let v10179: Lanes<6>;
            let v10180: Lanes<6>;
            let v10181: Lanes<6>;
            if v8373 != 0.0 {
                let v8376 = v8374.sqrt();
                let v18226 = v9785 * (v9368 / (v10435 * v8376));
                let v8377 = v4843 + v8376;
                let v18227 = v9452 + v18226;
                let v18228 = v9786 * v8378;
                let v18230 = v9785 * v8374;
                let v8383 = v8382 * v8378;
                let v8388 = v821 * v8376;
                let v8389 = v8388 * v4843;
                let v8390 = v8378 + v8374;
                let v8392 = ((v8383 * v8374) + (v90 * ((v8378 * v8378) + (v8374 * v8374)))) + (v8389 * v8390);
                let v18247 = ((((v9786 * v8382) * v8374) + (v9785 * v8383)) + (((v18228 + v18228) + (v18230 + v18230)) * v90)) + (((((v18226 * v821) * v4843) + (v9452 * v8388)) * v8390) + ((v9786 + v9785) * v8389));
                let v8393 = v8377 * v8377;
                let v18248 = v18227 * v8377;
                let v8394 = v8393 * v8393;
                let v18250 = (v18248 + v18248) * v8393;
                let v8395 = v8394 * v8377;
                let v8396 = v8392 / v8395;
                let v18257 = (v18247 - ((((v18250 + v18250) * v8377) + (v18227 * v8394)) * v8396)) / v8395;
                let v8397 = v166 / v8265;
                let v8398 = v8397 * v5780;
                let v8399 = v8398 * v1128;
                let v18265 = v9421 * v8398;
                let v18267 = ((((((v9781 * v8397) * v10390) / v8265) * v5780) + (v9779 * v8397)) * v1128) + (Lanes([v18265[0], v18265[1], 0.0, v18265[2], v18265[3], 0.0]));
                let v8400 = v8399 * v4843;
                let v8402 = v8401 / v8400;
                let v8403 = v90 * v4843;
                let v8406 = (v8378 + (v8403 * v8376)) + v8374;
                let v8410 = v8407 * v8408;
                let v8412 = v646 * v8377;
                let v8413 = v8402 * v8377;
                let v8414 = v8413 * v4843;
                let v8416 = (v8414 * v8392).sqrt();
                let v8417 = v8412 * v8416;
                let v8418 = (v8410 * v8406) / v8417;
                let v18302 = ((((v9787 * v8407) * v8406) + (((v9786 + (((v9452 * v90) * v8376) + (v18226 * v8403))) + v9785) * v8410)) - ((((v18227 * v646) * v8416) + ((((((((((v10174 - (((v18267 * v4843) + (v9452 * v8399)) * v8402)) / v8400) * v8377) + (v18227 * v8402)) * v4843) + (v9452 * v8413)) * v8392) + (v18247 * v8414)) * (v9368 / (v10435 * v8416))) * v8412)) * v8418)) / v8417;
                v8721 = v8399;
                v8734 = v8376;
                v8743 = v8396;
                v8747 = v8418;
                v10178 = v18267;
                v10179 = v18226;
                v10180 = v18257;
                v10181 = v18302;
            } else {
                v8721 = v11;
                v8734 = v0;
                v8743 = v0;
                v8747 = v0;
                v10178 = v11062;
                v10179 = v11062;
                v10180 = v11062;
                v10181 = v11062;
            }
            let v8420 = v5620 + v8419;
            let v18303 = v9831 + v9887;
            let v8615: f64;
            let v8616: f64;
            let v8618: f64;
            let v10182: Lanes<6>;
            let v10183: Lanes<6>;
            let v10184: Lanes<4>;
            if v565 != 0.0 {
                let v8427 = v8421 + v8424;
                let v8431: f64;
                if v368 != 0.0 {
                    let v8430 = v8427 - (v8428 * v142);
                    v8431 = v8430;
                } else {
                    v8431 = v8427;
                }
                let v8432 = -v8431;
                let v8433 = v830 - v878;
                let v18316 = v10562 - (Lanes([v9415[0], v9415[1], 0.0, v9415[2]]));
                let v8440 = v8435 * ((v1 + (v8436 / v122)).ln());
                let v8441 = v8440 * v145;
                let v8444 = v8441 * (v146 + v8442);
                let v8447 = v8441 * (v146 + v8445);
                let v18320 = (v9412 - (Lanes([v9410[0], v9410[1], 0.0]))) * v8444;
                let v18321 = v9412 * v8447;
                let v8452 = (v8440 * v570) * v145;
                let v8457 = v8454 + (v8444 * (v830 - v823));
                let v18324 = v10043 + (Lanes([v18320[0], v18320[1], 0.0, v18320[2], 0.0, 0.0]));
                let v8461 = v8458 + (v8447 * v830);
                let v18326 = v10044 + (Lanes([v18321[0], v18321[1], 0.0, v18321[2], 0.0, 0.0]));
                let v8462 = (v8432 * v8433) + (v8452 * v8433);
                let v18327 = (v18316 * v8432) + (v18316 * v8452);
                v8615 = v8457;
                v8616 = v8461;
                v8618 = v8462;
                v10182 = v18324;
                v10183 = v18326;
                v10184 = v18327;
            } else {
                let v8619: f64;
                let v10185: Lanes<4>;
                if v368 != 0.0 {
                    let v8465 = -((-v8428) * v142);
                    let v8467 = v8465 * (v830 - v878);
                    let v18306 = (v10562 - (Lanes([v9415[0], v9415[1], 0.0, v9415[2]]))) * v8465;
                    v8619 = v8467;
                    v10185 = v18306;
                } else {
                    v8619 = v0;
                    v10185 = v10625;
                }
                let v8474 = ((v8468 * v146) * v145) * ((v1 + (v8436 / v122)).ln());
                let v18309 = (v9412 - (Lanes([v9410[0], v9410[1], 0.0]))) * v8474;
                let v18310 = v9412 * v8474;
                let v8478 = v8454 + (v8474 * (v830 - v823));
                let v18312 = v10043 + (Lanes([v18309[0], v18309[1], 0.0, v18309[2], 0.0, 0.0]));
                let v8479 = v8458 + (v8474 * v830);
                let v18314 = v10044 + (Lanes([v18310[0], v18310[1], 0.0, v18310[2], 0.0, 0.0]));
                v8615 = v8478;
                v8616 = v8479;
                v8618 = v8619;
                v10182 = v18312;
                v10183 = v18314;
                v10184 = v10185;
            }
            let v8613: f64;
            let v8637: f64;
            let v8649: f64;
            let v8993: f64;
            let v8999: f64;
            let v9007: f64;
            let v9031: f64;
            let v9038: f64;
            let v10186: Lanes<6>;
            let v10187: Lanes<6>;
            let v10188: Lanes<6>;
            let v10189: Lanes<6>;
            let v10190: Lanes<6>;
            let v10191: Lanes<6>;
            let v10192: Lanes<6>;
            if v71 != 0.0 {
                let v8994: f64;
                let v9000: f64;
                let v9008: f64;
                let v9032: f64;
                let v9039: f64;
                let v10193: Lanes<6>;
                let v10194: Lanes<6>;
                let v10195: Lanes<6>;
                let v10196: Lanes<6>;
                if v565 != 0.0 {
                    v8994 = v13;
                    v9000 = v8285;
                    v9008 = v8480;
                    v9032 = v0;
                    v9039 = v0;
                    v10193 = v9782;
                    v10194 = v9788;
                    v10195 = v11062;
                    v10196 = v11062;
                } else {
                    let v8493 = v8488 + v8489;
                    let v18338 = v9790 + v9791;
                    let v8499 = (v8285 - v8488) + v8495;
                    let v18340 = (v9782 - v9790) + v9792;
                    v8994 = v0;
                    v9000 = v0;
                    v9008 = v8484;
                    v9032 = v8493;
                    v9039 = v8499;
                    v10193 = v11062;
                    v10194 = v9789;
                    v10195 = v18338;
                    v10196 = v18340;
                }
                v8613 = v0;
                v8637 = v0;
                v8649 = v0;
                v8993 = v8994;
                v8999 = v9000;
                v9007 = v9008;
                v9031 = v9032;
                v9038 = v9039;
                v10186 = v11062;
                v10187 = v11062;
                v10188 = v11062;
                v10189 = v10193;
                v10190 = v10194;
                v10191 = v10195;
                v10192 = v10196;
            } else {
                let v8614: f64;
                let v8638: f64;
                let v8650: f64;
                let v10197: Lanes<6>;
                let v10198: Lanes<6>;
                let v10199: Lanes<6>;
                if v565 != 0.0 {
                    let v8501 = (-v8480) - v8285;
                    let v18336 = (v9788 * v10390) - v9782;
                    let v8502 = v8285 - v8488;
                    let v18337 = v9782 - v9790;
                    v8614 = v8501;
                    v8638 = v8488;
                    v8650 = v8502;
                    v10197 = v18336;
                    v10198 = v9790;
                    v10199 = v18337;
                } else {
                    let v8506 = (((-v8484) - v8285) - v8495) - v8489;
                    let v18331 = (((v9789 * v10390) - v9782) - v9792) - v9791;
                    let v8507 = v8488 + v8489;
                    let v18332 = v9790 + v9791;
                    let v8509 = (v8285 - v8488) + v8495;
                    let v18334 = (v9782 - v9790) + v9792;
                    v8614 = v8506;
                    v8638 = v8507;
                    v8650 = v8509;
                    v10197 = v18331;
                    v10198 = v18332;
                    v10199 = v18334;
                }
                v8613 = v8614;
                v8637 = v8638;
                v8649 = v8650;
                v8993 = v0;
                v8999 = v0;
                v9007 = v0;
                v9031 = v0;
                v9038 = v0;
                v10186 = v10197;
                v10187 = v10198;
                v10188 = v10199;
                v10189 = v11062;
                v10190 = v11062;
                v10191 = v11062;
                v10192 = v11062;
            }
            let v8510 = if v6875 == v0 { 1.0 } else { 0.0 };
            let v8535: f64;
            let v10200: Lanes<6>;
            if v8510 != 0.0 {
                v8535 = v0;
                v10200 = v11062;
            } else {
                let v8515 = (v8511 * v136) + v4340;
                let v18342 = (v10039 * v136) + v9441;
                let v8516 = if v8515 > v8298 { 1.0 } else { 0.0 };
                let v8520: f64;
                let v10201: Lanes<6>;
                if v8516 != 0.0 {
                    v8520 = v8298;
                    v10201 = v10038;
                } else {
                    v8520 = v8515;
                    v10201 = v18342;
                }
                let v8517 = v823 + v4340;
                let v18344 = (Lanes([v9410[0], v9410[1], 0.0, 0.0, 0.0, 0.0])) + v9441;
                let v8519 = v1 - v4356;
                let v8529 = (v123 * v168) * (((v8523 / v490).sqrt()) * v8526);
                let v8533 = (((v8517 - ((v4356 * v8517) + (v8519 * v8520))) / v6875) - v8511) * v8529;
                let v18351 = (((v18344 - ((v18344 * v4356) + (v10201 * v8519))) / v6875) - v10039) * v8529;
                v8535 = v8533;
                v10200 = v18351;
            }
            let v8534 = if v338 != v0 { 1.0 } else { 0.0 };
            let v8621: f64;
            let v10202: Lanes<6>;
            if v8534 != 0.0 {
                let v18352 = v9415 * v342;
                let v8537 = v8535 + (v342 * v878);
                let v18354 = v10200 + (Lanes([v18352[0], v18352[1], 0.0, 0.0, v18352[2], 0.0]));
                v8621 = v8537;
                v10202 = v18354;
            } else {
                v8621 = v8535;
                v10202 = v10200;
            }
            let v8538 = if v566 == v1 { 1.0 } else { 0.0 };
            let v8708: f64;
            let v9013: f64;
            let v9021: f64;
            let v9052: f64;
            let v9058: f64;
            let v10203: Lanes<6>;
            let v10204: Lanes<6>;
            let v10205: Lanes<6>;
            let v10206: Lanes<6>;
            let v10207: Lanes<6>;
            if v8538 != 0.0 {
                let v8709: f64;
                let v9014: f64;
                let v9022: f64;
                let v9053: f64;
                let v9059: f64;
                let v10208: Lanes<6>;
                let v10209: Lanes<6>;
                let v10210: Lanes<6>;
                let v10211: Lanes<6>;
                let v10212: Lanes<6>;
                if v565 != 0.0 {
                    let v18374 = (v9914 * v10390) - v9915;
                    let v8578 = (((-v8539) - v8547) - v8555) - v8567;
                    let v18377 = ((Lanes([v18374[0], v18374[1], v18374[2], v18374[3], v18374[4], 0.0])) - v9916) - v9917;
                    let v8612 = v8596 + v8603;
                    let v18381 = (Lanes([v9920[0], v9920[1], v9920[2], v9920[3], v9920[4], 0.0])) + v9921;
                    let v8636 = v8613 + ((((((v8615 + v8616) + v8618) - v8621) - v8623) - v8629) + v8578);
                    let v18391 = v10186 + ((((((v10182 + v10183) + (Lanes([v10184[0], v10184[1], 0.0, v10184[2], v10184[3], 0.0]))) - v10202) - (Lanes([v10045[0], v10045[1], v10045[2], v10045[3], v10045[4], 0.0]))) - (Lanes([v10046[0], v10046[1], v10046[2], v10046[3], v10046[4], 0.0]))) + v18377);
                    let v8648 = v8637 + ((((-v8615) + v8621) + v8641) + (v8579 + v8586));
                    let v18397 = v10187 + ((((v10182 * v10390) + v10202) + (Lanes([v10047[0], v10047[1], v10047[2], v10047[3], v10047[4], 0.0]))) + ((Lanes([v9918[0], v9918[1], v9918[2], v9918[3], v9918[4], 0.0])) + v9919));
                    let v8659 = v8649 + (((-v8616) + v8652) + v8612);
                    let v18402 = v10188 + (((v10183 * v10390) + (Lanes([v10048[0], v10048[1], v10048[2], v10048[3], v10048[4], 0.0]))) + v18381);
                    v8709 = v8636;
                    v9014 = v8612;
                    v9022 = v8578;
                    v9053 = v8648;
                    v9059 = v8659;
                    v10208 = v18391;
                    v10209 = v18381;
                    v10210 = v18377;
                    v10211 = v18397;
                    v10212 = v18402;
                } else {
                    let v8665 = v8613 + (((((v8615 + v8616) + v8618) - v8621) - v8623) - v8629);
                    let v18363 = v10186 + (((((v10182 + v10183) + (Lanes([v10184[0], v10184[1], 0.0, v10184[2], v10184[3], 0.0]))) - v10202) - (Lanes([v10045[0], v10045[1], v10045[2], v10045[3], v10045[4], 0.0]))) - (Lanes([v10046[0], v10046[1], v10046[2], v10046[3], v10046[4], 0.0])));
                    let v8669 = v8637 + (((-v8615) + v8621) + v8641);
                    let v18368 = v10187 + (((v10182 * v10390) + v10202) + (Lanes([v10047[0], v10047[1], v10047[2], v10047[3], v10047[4], 0.0])));
                    let v8672 = v8649 + ((-v8616) + v8652);
                    let v18372 = v10188 + ((v10183 * v10390) + (Lanes([v10048[0], v10048[1], v10048[2], v10048[3], v10048[4], 0.0])));
                    v8709 = v8665;
                    v9014 = v0;
                    v9022 = v0;
                    v9053 = v8669;
                    v9059 = v8672;
                    v10208 = v18363;
                    v10209 = v11062;
                    v10210 = v11062;
                    v10211 = v18368;
                    v10212 = v18372;
                }
                v8708 = v8709;
                v9013 = v9014;
                v9021 = v9022;
                v9052 = v9053;
                v9058 = v9059;
                v10203 = v10208;
                v10204 = v10209;
                v10205 = v10210;
                v10206 = v10211;
                v10207 = v10212;
            } else {
                v8708 = v8613;
                v9013 = v0;
                v9021 = v0;
                v9052 = v8637;
                v9058 = v8649;
                v10203 = v10186;
                v10204 = v11062;
                v10205 = v11062;
                v10206 = v10187;
                v10207 = v10188;
            }
            let v9079: f64;
            let v9080: f64;
            let v9081: f64;
            let v9082: f64;
            let v10213: Lanes<3>;
            let v10214: Lanes<2>;
            let v10215: Lanes<3>;
            let v10216: Lanes<2>;
            if v565 != 0.0 {
                v9079 = v8674;
                v9080 = v8675;
                v9081 = v8673;
                v9082 = v8677;
                v10213 = v10138;
                v10214 = v10139;
                v10215 = v10137;
                v10216 = v10140;
            } else {
                v9079 = v0;
                v9080 = v0;
                v9081 = v0;
                v9082 = v0;
                v10213 = v17771;
                v10214 = v10380;
                v10215 = v17770;
                v10216 = v10381;
            }
            let v8679 = if v1886 != v1 { 1.0 } else { 0.0 };
            let v9047: f64;
            let v10217: Lanes<6>;
            if v8679 != 0.0 {
                v9047 = v0;
                v10217 = v11062;
            } else {
                v9047 = v5640;
                v10217 = v9873;
            }
            let v8682 = -v8680;
            let v18403 = v9894 * v10390;
            let v8683 = if v7825 == v1 { 1.0 } else { 0.0 };
            let v9077: f64;
            let v10218: Lanes<6>;
            if v8683 != 0.0 {
                let v8691 = (v8684 * v8685) - v8689;
                let v18409 = (v9895 * v8684) - (Lanes([v9896[0], v9896[1], 0.0, v9896[2], 0.0, 0.0]));
                v9077 = v8691;
                v10218 = v18409;
            } else {
                let v8692 = v1 - v8684;
                let v8696 = (v8692 * v8685) - v8694;
                let v18406 = (v9895 * v8692) - (Lanes([v9897[0], v9897[1], 0.0, v9897[2], 0.0, 0.0]));
                v9077 = v8696;
                v10218 = v18406;
            }
            let v9078: f64;
            let v10219: Lanes<6>;
            if v8683 != 0.0 {
                let v8697 = v1 - v8684;
                let v8699 = (v8697 * v8685) - v8694;
                let v18415 = (v9895 * v8697) - (Lanes([v9897[0], v9897[1], 0.0, v9897[2], 0.0, 0.0]));
                v9078 = v8699;
                v10219 = v18415;
            } else {
                let v8701 = (v8684 * v8685) - v8689;
                let v18412 = (v9895 * v8684) - (Lanes([v9896[0], v9896[1], 0.0, v9896[2], 0.0, 0.0]));
                v9078 = v8701;
                v10219 = v18412;
            }
            let v8706: f64;
            let v10220: Lanes<5>;
            if v8683 != 0.0 {
                v8706 = v8702;
                v10220 = v9906;
            } else {
                v8706 = v8704;
                v10220 = v9910;
            }
            let v8707: f64;
            let v10221: Lanes<5>;
            if v8683 != 0.0 {
                v8707 = v8704;
                v10221 = v9910;
            } else {
                v8707 = v8702;
                v10221 = v9906;
            }
            let v8710 = v365 * (v10203[0]);
            let v8711 = v365 * (v10203[1]);
            let v8712 = if v7825 > v0 { 1.0 } else { 0.0 };
            let v8713: f64;
            if v8712 != 0.0 {
                v8713 = v8711;
            } else {
                v8713 = v8710;
            }
            let v9121: f64;
            let v9123: f64;
            let v10222: Lanes<6>;
            let v10223: Lanes<6>;
            if v8373 != 0.0 {
                let v8716 = ((v23 * v1128) * v168) * v139;
                let v18420 = ((v10415 * v8717) * v8713) * v8713;
                let v8722 = (((v8717 * v665) * v8713) * v8713) / v8721;
                let v18424 = ((Lanes([0.0, 0.0, v18420[0], 0.0, 0.0, 0.0])) - (v10178 * v8722)) / v8721;
                let v8727 = if (if v8408 > v8723 { 1.0 } else { 0.0 }) != 0.0 && (if v823 > v8725 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8745: f64;
                let v10224: Lanes<6>;
                if v8727 != 0.0 {
                    let v8728 = v8309 / v5780;
                    let v18430 = (v9784 - (v9779 * v8728)) / v5780;
                    let v8730 = v8309 / v8729;
                    let v8732 = (v8730 - v8728) / v823;
                    let v18435 = v9410 * v8732;
                    let v8733 = v4274 * v8732;
                    let v8737 = (v8378 + (v4843 * v8734)) + v8374;
                    let v8739 = v4843 + v8734;
                    let v8740 = (v8733 * v8737) / v8739;
                    let v8741 = v8728 + v8740;
                    let v18452 = v18430 + ((((((((((v9784 - (v10175 * v8730)) / v8729) - v18430) - (Lanes([v18435[0], v18435[1], 0.0, 0.0, 0.0, 0.0]))) / v823) * v4274) * v8737) + (((v9786 + ((v9452 * v8734) + (v10179 * v4843))) + v9785) * v8733)) - ((v9452 + v10179) * v8740)) / v8739);
                    v8745 = v8741;
                    v10224 = v18452;
                } else {
                    let v8742 = v8309 / v8729;
                    let v18427 = (v9784 - (v10175 * v8742)) / v8729;
                    v8745 = v8742;
                    v10224 = v18427;
                }
                let v8744 = v8722 * v8743;
                let v8746 = v8744 * v8745;
                let v18458 = (((v18424 * v8743) + (v10180 * v8722)) * v8745) + (v10224 * v8744);
                let v8749 = if (-v8713) > v8716 { 1.0 } else { 0.0 };
                let v8751 = if v8749 != 0.0 && (if v8746 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8752: f64;
                let v10225: Lanes<6>;
                if v8751 != 0.0 {
                    v8752 = v8746;
                    v10225 = v18458;
                } else {
                    v8752 = v0;
                    v10225 = v11062;
                }
                let v8753: f64;
                let v10226: Lanes<6>;
                if v8749 != 0.0 {
                    v8753 = v8747;
                    v10226 = v10181;
                } else {
                    v8753 = v0;
                    v10226 = v11062;
                }
                v9121 = v8753;
                v9123 = v8752;
                v10222 = v10226;
                v10223 = v10225;
            } else {
                v9121 = v0;
                v9123 = v0;
                v10222 = v11062;
                v10223 = v11062;
            }
            let v8755 = if v8754 == v1 { 1.0 } else { 0.0 };
            let v9046: f64;
            let v10227: Lanes<5>;
            if v8755 != 0.0 {
                let v8785: f64;
                let v8787: f64;
                let v8796: f64;
                let v8819: f64;
                let v8820: f64;
                let v8868: f64;
                let v8874: f64;
                let v10228: Lanes<4>;
                if v8756 != 0.0 {
                    let v8758 = v8757 / v23;
                    let v8763 = if v8762 > v0 { 1.0 } else { 0.0 };
                    let v8766: f64;
                    if v8763 != 0.0 {
                        let v8765 = v8762 * v8764;
                        v8766 = v8765;
                    } else {
                        v8766 = v0;
                    }
                    let v8769 = v365 * (v603 - v613);
                    let v18468 = ((Lanes([0.0, v9370[0]])) - (Lanes([v9374[0], 0.0]))) * v365;
                    let v18469 = Lanes([0.0, v18468[0], 0.0, v18468[1]]);
                    v8785 = v8759;
                    v8787 = v8760;
                    v8796 = v8761;
                    v8819 = v8769;
                    v8820 = v8767;
                    v8868 = v8758;
                    v8874 = v8766;
                    v10228 = v18469;
                } else {
                    let v8773 = if v8762 > v0 { 1.0 } else { 0.0 };
                    let v8776: f64;
                    if v8773 != 0.0 {
                        let v8775 = v8762 * v8774;
                        v8776 = v8775;
                    } else {
                        v8776 = v0;
                    }
                    let v8779 = v365 * (v612 - v602);
                    let v18463 = ((Lanes([v9373[0], 0.0])) - (Lanes([0.0, v9369[0]]))) * v365;
                    let v18464 = Lanes([v18463[0], 0.0, v18463[1], 0.0]);
                    v8785 = v8770;
                    v8787 = v8771;
                    v8796 = v8772;
                    v8819 = v8779;
                    v8820 = v8777;
                    v8868 = v39;
                    v8874 = v8776;
                    v10228 = v18464;
                }
                let v8784 = ((v8780 * v8780) + (v134 * v134)).sqrt();
                let v8790 = v699.powf(v8789);
                let v8791 = (v8785 / v556) / v8790;
                let v8794 = v713 - (v8792 * v714);
                let v8795 = (v8787 / v68) / v8794;
                let v18482 = v9397 * v8797;
                let v8799 = v8796 + (v8797 * v653);
                let v8804 = v1 + (v8800 / (v143.powf(v8801)));
                let v8809 = v1 + (v8805 / (v143.powf(v8806)));
                let v8814 = v1 + (v8810 / (v169.powf(v8811)));
                let v8815 = v8791 * v8804;
                let v18483 = ((((v10416 * (v8789 * (v699.powf((v8789 - v9368))))) * v8791) * v10390) / v8790) * v8804;
                let v18485 = (((((v10428 - (v10429 * v8792)) * v8795) * v10390) / v8794) * v8814) * v8809;
                let v8818 = ((v8795 * v8814) * v8809) + v362;
                let v8821 = v8819 / v8820;
                let v8822 = v8815 * v8821;
                let v18487 = v18483 * v8821;
                let v18488 = (v10228 / v8820) * v8815;
                let v18491 = (Lanes([0.0, 0.0, 0.0, 0.0, v18487[0]])) + (Lanes([v18488[0], v18488[1], v18488[2], v18488[3], 0.0]));
                let v8823 = if v8819 >= v0 { 1.0 } else { 0.0 };
                let v8837: f64;
                let v10229: Lanes<5>;
                if v8823 != 0.0 {
                    let v8824 = v8822 / v8818;
                    let v18497 = v18485 * v8824;
                    let v18500 = (v18491 - (Lanes([0.0, 0.0, 0.0, 0.0, v18497[0]]))) / v8818;
                    v8837 = v8824;
                    v10229 = v18500;
                } else {
                    let v8826 = (-v8822) / v8818;
                    let v18493 = v18485 * v8826;
                    let v18496 = ((v18491 * v10390) - (Lanes([0.0, 0.0, 0.0, 0.0, v18493[0]]))) / v8818;
                    v8837 = v8826;
                    v10229 = v18496;
                }
                let v8831 = if (if v8827 <= v8799 { 1.0 } else { 0.0 }) != 0.0 && (if v8799 <= v8829 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8840: f64;
                let v10230: Lanes<5>;
                if v8831 != 0.0 {
                    v8840 = v1;
                    v10230 = v18459;
                } else {
                    let v8836 = if (if v8832 <= v8799 { 1.0 } else { 0.0 }) != 0.0 && (if v8799 <= v8834 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8841: f64;
                    let v10231: Lanes<5>;
                    if v8836 != 0.0 {
                        v8841 = v8837;
                        v10231 = v10229;
                    } else {
                        let v8838 = v8799 - v1;
                        let v8839 = v8837.powf(v8838);
                        let v18507 = v18482 * (v8839 * (v8837.ln()));
                        let v18509 = (v10229 * (v8838 * (v8837.powf((v8838 - v9368))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18507[0]]));
                        v8841 = v8839;
                        v10231 = v18509;
                    }
                    v8840 = v8841;
                    v10230 = v10231;
                }
                let v18512 = (v10229 * v8840) + (v10230 * v8837);
                let v8843 = v1 + (v8837 * v8840);
                let v8848 = if (if v8844 <= v8799 { 1.0 } else { 0.0 }) != 0.0 && (if v8799 <= v8846 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8862: f64;
                let v10232: Lanes<5>;
                if v8848 != 0.0 {
                    let v8849 = v1 / v8843;
                    let v18536 = ((v18512 * v8849) * v10390) / v8843;
                    v8862 = v8849;
                    v10232 = v18536;
                } else {
                    let v8854 = if (if v8850 <= v8799 { 1.0 } else { 0.0 }) != 0.0 && (if v8799 <= v8852 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8863: f64;
                    let v10233: Lanes<5>;
                    if v8854 != 0.0 {
                        let v8855 = v8843.sqrt();
                        let v8856 = v1 / v8855;
                        let v18533 = (((v18512 * (v9368 / (v10435 * v8855))) * v8856) * v10390) / v8855;
                        v8863 = v8856;
                        v10233 = v18533;
                    } else {
                        let v8858 = v8857 / v8799;
                        let v8859 = v8858 - v1;
                        let v8860 = v8843.powf(v8859);
                        let v18522 = (((v18482 * v8858) * v10390) / v8799) * (v8860 * (v8843.ln()));
                        let v8861 = v8843 * v8860;
                        let v18527 = (v18512 * v8860) + (((v18512 * (v8859 * (v8843.powf((v8859 - v9368))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18522[0]]))) * v8843);
                        v8863 = v8861;
                        v10233 = v18527;
                    }
                    v8862 = v8863;
                    v10232 = v10233;
                }
                let v18537 = v18483 * v8862;
                let v8866 = (v206 / v8820) * v8784;
                let v8869 = (v8866 * (v8815 * v8862)) * v8868;
                let v18542 = (((Lanes([0.0, 0.0, 0.0, 0.0, v18537[0]])) + (v10232 * v8815)) * v8866) * v8868;
                let v8870 = if v8869 <= v0 { 1.0 } else { 0.0 };
                let v8871: f64;
                let v10234: Lanes<5>;
                if v8870 != 0.0 {
                    v8871 = v362;
                    v10234 = v18459;
                } else {
                    v8871 = v8869;
                    v10234 = v18542;
                }
                let v8872 = v1 / v8871;
                let v18546 = (((v10234 * v8872) * v10390) / v8871) / v166;
                let v8875 = (v8872 / v166) + v8874;
                let v8877 = if (if v8875 > v29 { 1.0 } else { 0.0 }) != 0.0 && v8296 != 0.0 { 1.0 } else { 0.0 };
                if v8877 != 0.0 {
                } else {
                }
                let v8878 = if v8875 < v29 { 1.0 } else { 0.0 };
                let v8879: f64;
                let v10235: Lanes<5>;
                if v8878 != 0.0 {
                    v8879 = v29;
                    v10235 = v18459;
                } else {
                    v8879 = v8875;
                    v10235 = v18546;
                }
                v9046 = v8879;
                v10227 = v10235;
            } else {
                v9046 = v0;
                v10227 = v18459;
            }
            let v8881 = if v8880 == v1 { 1.0 } else { 0.0 };
            let v9045: f64;
            let v10236: Lanes<5>;
            if v8881 != 0.0 {
                let v8898: f64;
                let v8900: f64;
                let v8907: f64;
                let v8923: f64;
                let v8924: f64;
                let v8972: f64;
                let v8978: f64;
                let v10237: Lanes<4>;
                if v8882 != 0.0 {
                    let v8883 = v8757 / v23;
                    let v8884 = if v8762 > v0 { 1.0 } else { 0.0 };
                    let v8886: f64;
                    if v8884 != 0.0 {
                        let v8885 = v8762 * v8764;
                        v8886 = v8885;
                    } else {
                        v8886 = v0;
                    }
                    let v8888 = v365 * (v603 - v613);
                    let v18555 = ((Lanes([0.0, v9370[0]])) - (Lanes([v9374[0], 0.0]))) * v365;
                    let v18556 = Lanes([0.0, v18555[0], 0.0, v18555[1]]);
                    v8898 = v8759;
                    v8900 = v8760;
                    v8907 = v8761;
                    v8923 = v8888;
                    v8924 = v8767;
                    v8972 = v8883;
                    v8978 = v8886;
                    v10237 = v18556;
                } else {
                    let v8889 = if v8762 > v0 { 1.0 } else { 0.0 };
                    let v8891: f64;
                    if v8889 != 0.0 {
                        let v8890 = v8762 * v8774;
                        v8891 = v8890;
                    } else {
                        v8891 = v0;
                    }
                    let v8893 = v365 * (v612 - v602);
                    let v18550 = ((Lanes([v9373[0], 0.0])) - (Lanes([0.0, v9369[0]]))) * v365;
                    let v18551 = Lanes([v18550[0], 0.0, v18550[1], 0.0]);
                    v8898 = v8770;
                    v8900 = v8771;
                    v8907 = v8772;
                    v8923 = v8893;
                    v8924 = v8777;
                    v8972 = v39;
                    v8978 = v8891;
                    v10237 = v18551;
                }
                let v8897 = ((v8780 * v8780) + (v134 * v134)).sqrt();
                let v8902 = v699.powf(v8789);
                let v8903 = (v8898 / v556) / v8902;
                let v8905 = v713 - (v8792 * v714);
                let v8906 = (v8900 / v68) / v8905;
                let v18569 = v9397 * v8797;
                let v8909 = v8907 + (v8797 * v653);
                let v8912 = v1 + (v8800 / (v143.powf(v8801)));
                let v8915 = v1 + (v8805 / (v143.powf(v8806)));
                let v8918 = v1 + (v8810 / (v169.powf(v8811)));
                let v8919 = v8903 * v8912;
                let v18570 = ((((v10416 * (v8789 * (v699.powf((v8789 - v9368))))) * v8903) * v10390) / v8902) * v8912;
                let v18572 = (((((v10428 - (v10429 * v8792)) * v8906) * v10390) / v8905) * v8918) * v8915;
                let v8922 = ((v8906 * v8918) * v8915) + v362;
                let v8925 = v8923 / v8924;
                let v8926 = v8919 * v8925;
                let v18574 = v18570 * v8925;
                let v18575 = (v10237 / v8924) * v8919;
                let v18578 = (Lanes([0.0, 0.0, 0.0, 0.0, v18574[0]])) + (Lanes([v18575[0], v18575[1], v18575[2], v18575[3], 0.0]));
                let v8927 = if v8923 >= v0 { 1.0 } else { 0.0 };
                let v8941: f64;
                let v10238: Lanes<5>;
                if v8927 != 0.0 {
                    let v8928 = v8926 / v8922;
                    let v18584 = v18572 * v8928;
                    let v18587 = (v18578 - (Lanes([0.0, 0.0, 0.0, 0.0, v18584[0]]))) / v8922;
                    v8941 = v8928;
                    v10238 = v18587;
                } else {
                    let v8930 = (-v8926) / v8922;
                    let v18580 = v18572 * v8930;
                    let v18583 = ((v18578 * v10390) - (Lanes([0.0, 0.0, 0.0, 0.0, v18580[0]]))) / v8922;
                    v8941 = v8930;
                    v10238 = v18583;
                }
                let v8935 = if (if v8931 <= v8909 { 1.0 } else { 0.0 }) != 0.0 && (if v8909 <= v8933 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8944: f64;
                let v10239: Lanes<5>;
                if v8935 != 0.0 {
                    v8944 = v1;
                    v10239 = v18459;
                } else {
                    let v8940 = if (if v8936 <= v8909 { 1.0 } else { 0.0 }) != 0.0 && (if v8909 <= v8938 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8945: f64;
                    let v10240: Lanes<5>;
                    if v8940 != 0.0 {
                        v8945 = v8941;
                        v10240 = v10238;
                    } else {
                        let v8942 = v8909 - v1;
                        let v8943 = v8941.powf(v8942);
                        let v18594 = v18569 * (v8943 * (v8941.ln()));
                        let v18596 = (v10238 * (v8942 * (v8941.powf((v8942 - v9368))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18594[0]]));
                        v8945 = v8943;
                        v10240 = v18596;
                    }
                    v8944 = v8945;
                    v10239 = v10240;
                }
                let v18599 = (v10238 * v8944) + (v10239 * v8941);
                let v8947 = v1 + (v8941 * v8944);
                let v8952 = if (if v8948 <= v8909 { 1.0 } else { 0.0 }) != 0.0 && (if v8909 <= v8950 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8966: f64;
                let v10241: Lanes<5>;
                if v8952 != 0.0 {
                    let v8953 = v1 / v8947;
                    let v18623 = ((v18599 * v8953) * v10390) / v8947;
                    v8966 = v8953;
                    v10241 = v18623;
                } else {
                    let v8958 = if (if v8954 <= v8909 { 1.0 } else { 0.0 }) != 0.0 && (if v8909 <= v8956 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8967: f64;
                    let v10242: Lanes<5>;
                    if v8958 != 0.0 {
                        let v8959 = v8947.sqrt();
                        let v8960 = v1 / v8959;
                        let v18620 = (((v18599 * (v9368 / (v10435 * v8959))) * v8960) * v10390) / v8959;
                        v8967 = v8960;
                        v10242 = v18620;
                    } else {
                        let v8962 = v8961 / v8909;
                        let v8963 = v8962 - v1;
                        let v8964 = v8947.powf(v8963);
                        let v18609 = (((v18569 * v8962) * v10390) / v8909) * (v8964 * (v8947.ln()));
                        let v8965 = v8947 * v8964;
                        let v18614 = (v18599 * v8964) + (((v18599 * (v8963 * (v8947.powf((v8963 - v9368))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18609[0]]))) * v8947);
                        v8967 = v8965;
                        v10242 = v18614;
                    }
                    v8966 = v8967;
                    v10241 = v10242;
                }
                let v18624 = v18570 * v8966;
                let v8970 = (v206 / v8924) * v8897;
                let v8973 = (v8970 * (v8919 * v8966)) * v8972;
                let v18629 = (((Lanes([0.0, 0.0, 0.0, 0.0, v18624[0]])) + (v10241 * v8919)) * v8970) * v8972;
                let v8974 = if v8973 <= v0 { 1.0 } else { 0.0 };
                let v8975: f64;
                let v10243: Lanes<5>;
                if v8974 != 0.0 {
                    v8975 = v362;
                    v10243 = v18459;
                } else {
                    v8975 = v8973;
                    v10243 = v18629;
                }
                let v8976 = v1 / v8975;
                let v18633 = (((v10243 * v8976) * v10390) / v8975) / v166;
                let v8979 = (v8976 / v166) + v8978;
                let v8981 = if (if v8979 > v29 { 1.0 } else { 0.0 }) != 0.0 && v8296 != 0.0 { 1.0 } else { 0.0 };
                if v8981 != 0.0 {
                } else {
                }
                let v8982 = if v8979 < v29 { 1.0 } else { 0.0 };
                let v8983: f64;
                let v10244: Lanes<5>;
                if v8982 != 0.0 {
                    v8983 = v29;
                    v10244 = v18459;
                } else {
                    v8983 = v8979;
                    v10244 = v18633;
                }
                v9045 = v8983;
                v10236 = v10244;
            } else {
                v9045 = v0;
                v10236 = v18459;
            }
            let v9048: f64;
            let v9054: f64;
            let v9060: f64;
            let v9066: f64;
            let v9195: f64;
            let v9197: f64;
            let v9231: f64;
            let v9233: f64;
            let v10245: Lanes<10>;
            let v10246: Lanes<8>;
            let v10247: Lanes<8>;
            let v10248: Lanes<1>;
            let v10249: Lanes<7>;
            let v10250: Lanes<7>;
            let v10251: Lanes<7>;
            let v10252: Lanes<7>;
            if v565 != 0.0 {
                let v9049: f64;
                let v9055: f64;
                let v9061: f64;
                let v9067: f64;
                let v9196: f64;
                let v9198: f64;
                let v10253: Lanes<8>;
                let v10254: Lanes<7>;
                let v10255: Lanes<7>;
                let v10256: Lanes<1>;
                let v10257: Lanes<7>;
                let v10258: Lanes<7>;
                if v71 != 0.0 {
                    let v8987 = if v8984 < v8986 { 1.0 } else { 0.0 };
                    let v9002: f64;
                    let v10259: Lanes<6>;
                    if v8987 != 0.0 {
                        v9002 = v8988;
                        v10259 = v11062;
                    } else {
                        v9002 = v8984;
                        v10259 = v10171;
                    }
                    let v8991 = if v8989 < v8990 { 1.0 } else { 0.0 };
                    let v9010: f64;
                    let v10260: Lanes<4>;
                    if v8991 != 0.0 {
                        v9010 = v8992;
                        v10260 = v10625;
                    } else {
                        v9010 = v8989;
                        v10260 = v10172;
                    }
                    let v8996: f64;
                    if v8683 != 0.0 {
                        v8996 = v8993;
                    } else {
                        let v8995 = v1 - v8993;
                        v8996 = v8995;
                    }
                    let v9003 = (v8997 - v8999) / v9002;
                    let v18667 = v10259 * v9003;
                    let v18670 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v9388[0]])) - (Lanes([v10189[0], v10189[1], v10189[2], v10189[3], v10189[4], v10189[5], 0.0]))) - (Lanes([v18667[0], v18667[1], v18667[2], v18667[3], v18667[4], v18667[5], 0.0]))) / v9002;
                    let v9011 = (v9004 - v9007) / v9010;
                    let v18674 = v10260 * v9011;
                    let v18677 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9389[0], 0.0])) - (Lanes([v10190[0], v10190[1], v10190[2], v10190[3], v10190[4], 0.0, v10190[5]]))) - (Lanes([v18674[0], v18674[1], 0.0, v18674[2], v18674[3], 0.0, 0.0]))) / v9010;
                    let v18678 = v9388 * v8996;
                    let v9015 = (v8997 * v8996) + v9013;
                    let v18680 = Lanes([v10204[0], v10204[1], v10204[2], v10204[3], v10204[4], v10204[5], 0.0]);
                    let v18681 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v18678[0]])) + v18680;
                    let v9016 = v1 - v8996;
                    let v18682 = v9388 * v9016;
                    let v9018 = (v8997 * v9016) + v9013;
                    let v18684 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v18682[0]])) + v18680;
                    let v18685 = v9388 * v10390;
                    let v18688 = (Lanes([0.0, v18685[0]])) - (Lanes([v9389[0], 0.0]));
                    let v9023 = ((-v8997) - v9004) + v9021;
                    let v18691 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18688[0], 0.0, v18688[1]])) + (Lanes([v10205[0], v10205[1], v10205[2], v10205[3], v10205[4], 0.0, v10205[5], 0.0]));
                    v9049 = v9023;
                    v9055 = v9015;
                    v9061 = v9018;
                    v9067 = v9004;
                    v9196 = v9003;
                    v9198 = v9011;
                    v10253 = v18691;
                    v10254 = v18681;
                    v10255 = v18684;
                    v10256 = v9389;
                    v10257 = v18670;
                    v10258 = v18677;
                } else {
                    v9049 = v0;
                    v9055 = v0;
                    v9061 = v0;
                    v9067 = v0;
                    v9196 = v0;
                    v9198 = v0;
                    v10253 = v18663;
                    v10254 = v18661;
                    v10255 = v18661;
                    v10256 = v10374;
                    v10257 = v18661;
                    v10258 = v18662;
                }
                let v18692 = Lanes([v10253[0], v10253[1], v10253[2], v10253[3], v10253[4], v10253[5], 0.0, 0.0, v10253[6], v10253[7]]);
                let v18693 = Lanes([v10254[0], v10254[1], v10254[2], v10254[3], v10254[4], 0.0, v10254[5], v10254[6]]);
                let v18694 = Lanes([v10255[0], v10255[1], v10255[2], v10255[3], v10255[4], 0.0, v10255[5], v10255[6]]);
                v9048 = v9049;
                v9054 = v9055;
                v9060 = v9061;
                v9066 = v9067;
                v9195 = v9196;
                v9197 = v9198;
                v9231 = v0;
                v9233 = v0;
                v10245 = v18692;
                v10246 = v18693;
                v10247 = v18694;
                v10248 = v10256;
                v10249 = v10257;
                v10250 = v10258;
                v10251 = v18635;
                v10252 = v18636;
            } else {
                let v9050: f64;
                let v9056: f64;
                let v9062: f64;
                let v9068: f64;
                let v9232: f64;
                let v9234: f64;
                let v10261: Lanes<3>;
                let v10262: Lanes<1>;
                let v10263: Lanes<1>;
                let v10264: Lanes<1>;
                let v10265: Lanes<7>;
                let v10266: Lanes<7>;
                if v71 != 0.0 {
                    let v9025 = if v8984 < v9024 { 1.0 } else { 0.0 };
                    let v9034: f64;
                    let v10267: Lanes<6>;
                    if v9025 != 0.0 {
                        v9034 = v9026;
                        v10267 = v11062;
                    } else {
                        v9034 = v8984;
                        v10267 = v10171;
                    }
                    let v9028 = if v8989 < v9027 { 1.0 } else { 0.0 };
                    if v9028 != 0.0 {
                    } else {
                    }
                    let v9035 = (v9029 - v9031) / v9034;
                    let v18640 = v10267 * v9035;
                    let v18643 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9390[0], 0.0])) - (Lanes([v10191[0], v10191[1], v10191[2], v10191[3], v10191[4], 0.0, v10191[5]]))) - (Lanes([v18640[0], v18640[1], v18640[2], v18640[3], v18640[4], 0.0, v18640[5]]))) / v9034;
                    let v9041 = (v9036 - v9038) / v9034;
                    let v18647 = v10267 * v9041;
                    let v18650 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9391[0], 0.0])) - (Lanes([v10192[0], v10192[1], v10192[2], v10192[3], v10192[4], 0.0, v10192[5]]))) - (Lanes([v18647[0], v18647[1], v18647[2], v18647[3], v18647[4], 0.0, v18647[5]]))) / v9034;
                    let v18651 = v9390 * v10390;
                    let v18654 = (Lanes([v18651[0], 0.0])) - (Lanes([0.0, v9391[0]]));
                    let v9044 = ((-v9029) - v9036) - v9004;
                    let v18657 = (Lanes([0.0, v18654[0], v18654[1]])) - (Lanes([v9389[0], 0.0, 0.0]));
                    v9050 = v9044;
                    v9056 = v9029;
                    v9062 = v9036;
                    v9068 = v9004;
                    v9232 = v9035;
                    v9234 = v9041;
                    v10261 = v18657;
                    v10262 = v9390;
                    v10263 = v9391;
                    v10264 = v9389;
                    v10265 = v18643;
                    v10266 = v18650;
                } else {
                    v9050 = v0;
                    v9056 = v0;
                    v9062 = v0;
                    v9068 = v0;
                    v9232 = v0;
                    v9234 = v0;
                    v10261 = v18634;
                    v10262 = v10375;
                    v10263 = v10376;
                    v10264 = v10374;
                    v10265 = v18635;
                    v10266 = v18636;
                }
                let v18658 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10261[0], v10261[1], v10261[2], 0.0, 0.0]);
                let v18659 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10262[0], 0.0, 0.0]);
                let v18660 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10263[0], 0.0, 0.0]);
                v9048 = v9050;
                v9054 = v9056;
                v9060 = v9062;
                v9066 = v9068;
                v9195 = v0;
                v9197 = v0;
                v9231 = v9232;
                v9233 = v9234;
                v10245 = v18658;
                v10246 = v18659;
                v10247 = v18660;
                v10248 = v10264;
                v10249 = v18661;
                v10250 = v18662;
                v10251 = v10265;
                v10252 = v10266;
            }
            let v9085: f64;
            let v9088: f64;
            let v9089: f64;
            let v9091: f64;
            let v9092: f64;
            let v9093: f64;
            let v10268: Lanes<6>;
            let v10269: Lanes<6>;
            let v10270: Lanes<6>;
            let v10271: Lanes<10>;
            let v10272: Lanes<9>;
            let v10273: Lanes<7>;
            if v8683 != 0.0 {
                let v9051 = v8708 + v9048;
                let v18708 = (Lanes([v10203[0], v10203[1], v10203[2], v10203[3], v10203[4], 0.0, 0.0, 0.0, v10203[5], 0.0])) + v10245;
                let v9057 = v9052 + v9054;
                let v18710 = (Lanes([v10206[0], v10206[1], v10206[2], v10206[3], v10206[4], 0.0, v10206[5], 0.0])) + v10246;
                let v18713 = ((v10203 + v10206) + v10207) * v10390;
                let v9069 = (-((v8708 + v9052) + v9058)) + v9066;
                let v18716 = (Lanes([v18713[0], v18713[1], v18713[2], v18713[3], v18713[4], 0.0, v18713[5]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10248[0], 0.0]));
                let v18717 = Lanes([v18710[0], v18710[1], v18710[2], v18710[3], v18710[4], v18710[5], 0.0, v18710[6], v18710[7]]);
                v9085 = v8420;
                v9088 = v9047;
                v9089 = v0;
                v9091 = v9051;
                v9092 = v9057;
                v9093 = v9069;
                v10268 = v18303;
                v10269 = v10217;
                v10270 = v11062;
                v10271 = v18708;
                v10272 = v18717;
                v10273 = v18716;
            } else {
                let v9070 = -v8420;
                let v18695 = v18303 * v10390;
                let v9071 = v8708 + v9048;
                let v18697 = (Lanes([v10203[0], v10203[1], v10203[2], v10203[3], v10203[4], 0.0, 0.0, 0.0, v10203[5], 0.0])) + v10245;
                let v9072 = v9058 + v9060;
                let v18699 = (Lanes([v10207[0], v10207[1], v10207[2], v10207[3], v10207[4], 0.0, v10207[5], 0.0])) + v10247;
                let v18702 = ((v10203 + v10206) + v10207) * v10390;
                let v9076 = (-((v8708 + v9052) + v9058)) + v9066;
                let v18705 = (Lanes([v18702[0], v18702[1], v18702[2], v18702[3], v18702[4], 0.0, v18702[5]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10248[0], 0.0]));
                let v18706 = Lanes([v18699[0], v18699[1], v18699[2], v18699[3], v18699[4], 0.0, v18699[5], v18699[6], v18699[7]]);
                v9085 = v9070;
                v9088 = v0;
                v9089 = v9047;
                v9091 = v9071;
                v9092 = v9072;
                v9093 = v9076;
                v10268 = v18695;
                v10269 = v11062;
                v10270 = v10217;
                v10271 = v18697;
                v10272 = v18706;
                v10273 = v18705;
            }
            let v9094: f64;
            let v9095: f64;
            let v9096: f64;
            let v9097: f64;
            let v10274: Lanes<3>;
            let v10275: Lanes<3>;
            let v10276: Lanes<2>;
            let v10277: Lanes<2>;
            if v565 != 0.0 {
                v9094 = v9079;
                v9095 = v9081;
                v9096 = v9080;
                v9097 = v9082;
                v10274 = v10213;
                v10275 = v10215;
                v10276 = v10214;
                v10277 = v10216;
            } else {
                v9094 = v8674;
                v9095 = v8673;
                v9096 = v8675;
                v9097 = v8677;
                v10274 = v10138;
                v10275 = v10137;
                v10276 = v10139;
                v10277 = v10140;
            }
            let v9084 = if (if v631 == v1 { 1.0 } else { 0.0 }) != 0.0 && v633 != 0.0 { 1.0 } else { 0.0 };
            let v9158: f64;
            let v9159: f64;
            let v9163: f64;
            let v10278: Lanes<6>;
            if v9084 != 0.0 {
                let v9086 = v9085 * v823;
                let v18719 = v9410 * v9085;
                let v18721 = (v10268 * v823) + (Lanes([v18719[0], v18719[1], 0.0, 0.0, 0.0, 0.0]));
                let v9087 = v1 / v386;
                v9158 = v9086;
                v9159 = v9087;
                v9163 = v387;
                v10278 = v18721;
            } else {
                v9158 = v0;
                v9159 = v0;
                v9163 = v0;
                v10278 = v11062;
            }
            let v9090 = if v7825 != v1 { 1.0 } else { 0.0 };
            if v9090 != 0.0 {
            } else {
            }
            if v565 != 0.0 {
            } else {
            }
            let v9098 = if v70 >= v91 { 1.0 } else { 0.0 };
            if v9098 != 0.0 {
                if v565 != 0.0 {
                } else {
                }
            } else {
            }
            let v9100 = v9099 * v652;
            let v18722 = v9397 * v9099;
            let v9101 = v365 * v9085;
            let v18723 = v10268 * v365;
            let v9102 = if v5793 == v1 { 1.0 } else { 0.0 };
            let v9256: f64;
            let v9257: f64;
            let v9258: f64;
            let v10279: Lanes<6>;
            let v10280: Lanes<6>;
            let v10281: Lanes<4>;
            if v9102 != 0.0 {
                let v9103 = v365 * v9078;
                let v18724 = v10219 * v365;
                let v9104 = v365 * v9077;
                let v18725 = v10218 * v365;
                let v9105 = v365 * v8682;
                let v18726 = v18403 * v365;
                v9256 = v9103;
                v9257 = v9104;
                v9258 = v9105;
                v10279 = v18724;
                v10280 = v18725;
                v10281 = v18726;
            } else {
                v9256 = v0;
                v9257 = v0;
                v9258 = v0;
                v10279 = v11062;
                v10280 = v11062;
                v10281 = v10625;
            }
            let v9259: f64;
            let v9260: f64;
            let v10282: Lanes<5>;
            if v8754 != 0.0 {
                let v18729 = (Lanes([0.0, v9370[0]])) - (Lanes([v9374[0], 0.0]));
                let v9107 = (v603 - v613) / v9046;
                let v18733 = ((Lanes([0.0, v18729[0], 0.0, v18729[1], 0.0])) - (v10227 * v9107)) / v9046;
                v9259 = v9107;
                v9260 = v0;
                v10282 = v18733;
            } else {
                v9259 = v0;
                v9260 = v9108;
                v10282 = v18459;
            }
            let v9261: f64;
            let v9262: f64;
            let v10283: Lanes<5>;
            if v8880 != 0.0 {
                let v18736 = (Lanes([v9373[0], 0.0])) - (Lanes([0.0, v9369[0]]));
                let v9110 = (v612 - v602) / v9045;
                let v18740 = ((Lanes([v18736[0], 0.0, v18736[1], 0.0, 0.0])) - (v10236 * v9110)) / v9045;
                v9261 = v9110;
                v9262 = v0;
                v10283 = v18740;
            } else {
                v9261 = v0;
                v9262 = v9111;
                v10283 = v18459;
            }
            let v9113 = v365 * (ddt(73861, v9091));
            let v18743 = (v10271 * v18741) * v365;
            let v9115 = v365 * (ddt(73865, v9092));
            let v18745 = (v10272 * v18741) * v365;
            let v9117 = v365 * (ddt(73869, v9093));
            let v18747 = (v10273 * v18741) * v365;
            let v9120 = v9100 * v8401;
            let v18748 = v18722 * v8401;
            let v18751 = (Lanes([0.0, 0.0, v18748[0], 0.0, 0.0, 0.0])) + (v10174 * v9100);
            let v9125 = if (if v9120 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v9123 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v9128: f64;
            let v10284: Lanes<6>;
            if v9125 != 0.0 {
                let v9126 = v9123 / v9120;
                let v9127 = v9126.sqrt();
                let v18757 = ((v10223 - (v18751 * v9126)) / v9120) * (v9368 / (v10435 * v9127));
                v9128 = v9127;
                v10284 = v18757;
            } else {
                v9128 = v0;
                v10284 = v11062;
            }
            let v9132 = v9121 * v9129;
            let v18758 = v10222 * v9129;
            let v18759 = v9381 * v9121;
            let v18762 = (Lanes([v18758[0], v18758[1], v18758[2], v18758[3], v18758[4], 0.0, v18758[5]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18759[0], 0.0]));
            let v9136: f64;
            let v10285: Lanes<6>;
            if v8712 != 0.0 {
                let v9133 = v1 - v9118;
                let v9134 = v9128 * v9133;
                let v18769 = (v10284 * v9133) + ((v9793 * v10390) * v9128);
                v9136 = v9134;
                v10285 = v18769;
            } else {
                let v9135 = v9128 * v9118;
                let v18765 = (v10284 * v9118) + (v9793 * v9128);
                v9136 = v9135;
                v10285 = v18765;
            }
            let v9140: f64;
            let v10286: Lanes<6>;
            if v8712 != 0.0 {
                let v9137 = v9128 * v9118;
                let v18776 = (v10284 * v9118) + (v9793 * v9128);
                v9140 = v9137;
                v10286 = v18776;
            } else {
                let v9138 = v1 - v9118;
                let v9139 = v9128 * v9138;
                let v18773 = (v10284 * v9138) + ((v9793 * v10390) * v9128);
                v9140 = v9139;
                v10286 = v18773;
            }
            let v9141 = v9129 * v9136;
            let v18777 = v9381 * v9136;
            let v18778 = v10285 * v9129;
            let v18781 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18777[0], 0.0])) + (Lanes([v18778[0], v18778[1], v18778[2], v18778[3], v18778[4], 0.0, v18778[5]]));
            let v9142 = ddt(73942, v9141);
            let v18782 = v18781 * v18741;
            let v9143 = v9129 * v9140;
            let v18783 = v9381 * v9140;
            let v18784 = v10286 * v9129;
            let v18787 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18783[0], 0.0])) + (Lanes([v18784[0], v18784[1], v18784[2], v18784[3], v18784[4], 0.0, v18784[5]]));
            let v9144 = ddt(73946, v9143);
            let v18788 = v18787 * v18741;
            let v9263: f64;
            if v8754 != 0.0 {
                v9263 = v9145;
            } else {
                v9263 = v0;
            }
            let v9264: f64;
            if v8880 != 0.0 {
                v9264 = v9146;
            } else {
                v9264 = v0;
            }
            let v9265: f64;
            let v9266: f64;
            let v9267: f64;
            if v9102 != 0.0 {
                v9265 = v9147;
                v9266 = v9148;
                v9267 = v9149;
            } else {
                v9265 = v0;
                v9266 = v0;
                v9267 = v0;
            }
            let v9268: f64;
            let v9269: f64;
            let v10287: Lanes<2>;
            if v539 != 0.0 {
                let v9154 = v9150 * (v9152 - v606);
                let v18793 = ((Lanes([v9382[0], 0.0])) - (Lanes([0.0, v9371[0]]))) * v9150;
                v9268 = v9154;
                v9269 = v0;
                v10287 = v18793;
            } else {
                v9268 = v0;
                v9269 = v9155;
                v10287 = v18789;
            }
            let v9157 = if v632 != 0.0 && (if v34 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v9270: f64;
            let v9271: f64;
            let v9272: f64;
            let v9273: f64;
            let v9274: f64;
            let v9353: f64;
            let v10288: Lanes<1>;
            let v10289: Lanes<6>;
            let v10290: Lanes<1>;
            let v10291: Lanes<1>;
            let v10292: Lanes<1>;
            let v10293: Lanes<1>;
            if v9157 != 0.0 {
                let v9160 = v635 * v9159;
                let v18795 = v9379 * v9159;
                let v9161 = -v9158;
                let v18796 = v10278 * v10390;
                let v9162 = v635 * v11;
                let v18797 = v9379 * v11;
                let v9164 = v9163 * v635;
                let v18798 = v9379 * v9163;
                let v9165 = ddt(74007, v9164);
                let v18799 = v18798 * v18741;
                v9270 = v9160;
                v9271 = v9161;
                v9272 = v9162;
                v9273 = v9165;
                v9274 = v0;
                v9353 = v9164;
                v10288 = v18795;
                v10289 = v18796;
                v10290 = v18797;
                v10291 = v18799;
                v10292 = v10389;
                v10293 = v18798;
            } else {
                let v9166 = v635 * v556;
                let v18794 = v9379 * v556;
                v9270 = v0;
                v9271 = v0;
                v9272 = v0;
                v9273 = v0;
                v9274 = v9166;
                v9353 = v0;
                v10288 = v10389;
                v10289 = v11062;
                v10290 = v10389;
                v10291 = v10389;
                v10292 = v18794;
                v10293 = v10389;
            }
            let v9275: f64;
            let v9276: f64;
            let v9277: f64;
            let v9278: f64;
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
            let v9309: f64;
            let v9311: f64;
            let v9313: f64;
            let v9315: f64;
            let v9316: f64;
            let v9317: f64;
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
            let v9342: f64;
            let v9344: f64;
            let v9346: f64;
            let v9348: f64;
            let v9355: f64;
            let v9357: f64;
            let v9359: f64;
            let v9361: f64;
            let v9363: f64;
            let v9365: f64;
            let v9367: f64;
            let v10294: Lanes<6>;
            let v10295: Lanes<6>;
            let v10296: Lanes<3>;
            let v10297: Lanes<3>;
            let v10298: Lanes<2>;
            let v10299: Lanes<2>;
            let v10300: Lanes<2>;
            let v10301: Lanes<7>;
            let v10302: Lanes<7>;
            let v10303: Lanes<1>;
            let v10304: Lanes<1>;
            let v10305: Lanes<1>;
            let v10306: Lanes<1>;
            let v10307: Lanes<6>;
            let v10308: Lanes<1>;
            let v10309: Lanes<1>;
            let v10310: Lanes<6>;
            let v10311: Lanes<6>;
            let v10312: Lanes<6>;
            let v10313: Lanes<1>;
            let v10314: Lanes<1>;
            let v10315: Lanes<7>;
            let v10316: Lanes<7>;
            let v10317: Lanes<7>;
            let v10318: Lanes<1>;
            let v10319: Lanes<1>;
            let v10320: Lanes<1>;
            let v10321: Lanes<1>;
            let v10322: Lanes<1>;
            let v10323: Lanes<1>;
            let v10324: Lanes<1>;
            let v10325: Lanes<1>;
            let v10326: Lanes<1>;
            let v10327: Lanes<1>;
            let v10328: Lanes<1>;
            let v10329: Lanes<1>;
            let v10330: Lanes<1>;
            if v565 != 0.0 {
                let v9168 = v365 * (v8706 + v9088);
                let v18823 = ((Lanes([v10220[0], v10220[1], v10220[2], v10220[3], v10220[4], 0.0])) + v10269) * v365;
                let v9170 = v365 * (v8707 + v9089);
                let v18826 = ((Lanes([v10221[0], v10221[1], v10221[2], v10221[3], v10221[4], 0.0])) + v10270) * v365;
                let v18827 = v10277 * v18741;
                let v9173 = v365 * (v9095 + (ddt(74027, v9097)));
                let v18830 = (v10275 + (Lanes([v18827[0], 0.0, v18827[1]]))) * v365;
                let v18831 = v10276 * v18741;
                let v9176 = v365 * (v9094 + (ddt(74033, v9096)));
                let v18834 = (v10274 + (Lanes([v18831[0], 0.0, v18831[1]]))) * v365;
                let v9280: f64;
                let v9282: f64;
                let v10331: Lanes<2>;
                if v545 != 0.0 {
                    let v9181 = (v9177 - v609) / v9179;
                    let v18838 = ((Lanes([v9383[0], 0.0])) - (Lanes([0.0, v9372[0]]))) / v9179;
                    v9280 = v9181;
                    v9282 = v0;
                    v10331 = v18838;
                } else {
                    v9280 = v0;
                    v9282 = v9182;
                    v10331 = v18818;
                }
                let v9284: f64;
                let v9286: f64;
                let v9288: f64;
                let v9290: f64;
                let v10332: Lanes<2>;
                let v10333: Lanes<2>;
                if v552 != 0.0 {
                    let v9187 = v9183 * (v9185 - v609);
                    let v18842 = ((Lanes([v9384[0], 0.0])) - (Lanes([0.0, v9372[0]]))) * v9183;
                    let v9192 = v9188 * (v9190 - v609);
                    let v18846 = ((Lanes([v9385[0], 0.0])) - (Lanes([0.0, v9372[0]]))) * v9188;
                    v9284 = v9187;
                    v9286 = v9192;
                    v9288 = v0;
                    v9290 = v0;
                    v10332 = v18842;
                    v10333 = v18846;
                } else {
                    v9284 = v0;
                    v9286 = v0;
                    v9288 = v9193;
                    v9290 = v9194;
                    v10332 = v18819;
                    v10333 = v18820;
                }
                let v9292: f64;
                let v9294: f64;
                let v9296: f64;
                let v9298: f64;
                let v9300: f64;
                let v9302: f64;
                let v9304: f64;
                let v9306: f64;
                let v9354: f64;
                let v9356: f64;
                let v10334: Lanes<7>;
                let v10335: Lanes<7>;
                let v10336: Lanes<1>;
                let v10337: Lanes<1>;
                let v10338: Lanes<1>;
                let v10339: Lanes<1>;
                let v10340: Lanes<1>;
                let v10341: Lanes<1>;
                if v71 != 0.0 {
                    let v9199 = v618 * v11;
                    let v18847 = v9375 * v11;
                    let v9200 = v621 * v11;
                    let v18848 = v9376 * v11;
                    let v9202 = v9201 * v618;
                    let v18849 = v9375 * v9201;
                    let v9203 = ddt(74064, v9202);
                    let v18850 = v18849 * v18741;
                    let v9205 = v9204 * v621;
                    let v18851 = v9376 * v9204;
                    let v9206 = ddt(74070, v9205);
                    let v18852 = v18851 * v18741;
                    v9292 = v9195;
                    v9294 = v9197;
                    v9296 = v9199;
                    v9298 = v9200;
                    v9300 = v9203;
                    v9302 = v9206;
                    v9304 = v0;
                    v9306 = v0;
                    v9354 = v9202;
                    v9356 = v9205;
                    v10334 = v10249;
                    v10335 = v10250;
                    v10336 = v18847;
                    v10337 = v18848;
                    v10338 = v18850;
                    v10339 = v18852;
                    v10340 = v18849;
                    v10341 = v18851;
                } else {
                    v9292 = v0;
                    v9294 = v0;
                    v9296 = v0;
                    v9298 = v0;
                    v9300 = v0;
                    v9302 = v0;
                    v9304 = v9207;
                    v9306 = v9208;
                    v9354 = v0;
                    v9356 = v0;
                    v10334 = v18661;
                    v10335 = v18662;
                    v10336 = v10382;
                    v10337 = v10374;
                    v10338 = v10382;
                    v10339 = v10374;
                    v10340 = v10382;
                    v10341 = v10374;
                }
                let v9209 = if v2247 != 0.0 || v5625 != 0.0 { 1.0 } else { 0.0 };
                let v9308: f64;
                let v9310: f64;
                let v9312: f64;
                let v9314: f64;
                let v9358: f64;
                let v10342: Lanes<6>;
                let v10343: Lanes<1>;
                let v10344: Lanes<1>;
                let v10345: Lanes<1>;
                if v9209 != 0.0 {
                    let v9216 = v2254 * v11;
                    let v18853 = v9380 * v11;
                    let v9218 = v9217 * v2254;
                    let v18854 = v9380 * v9217;
                    let v9219 = ddt(74091, v9218);
                    let v18855 = v18854 * v18741;
                    v9308 = v9210;
                    v9310 = v9216;
                    v9312 = v9219;
                    v9314 = v0;
                    v9358 = v9218;
                    v10342 = v9884;
                    v10343 = v18853;
                    v10344 = v18855;
                    v10345 = v18854;
                } else {
                    v9308 = v0;
                    v9310 = v0;
                    v9312 = v0;
                    v9314 = v9220;
                    v9358 = v0;
                    v10342 = v11062;
                    v10343 = v11036;
                    v10344 = v11036;
                    v10345 = v11036;
                }
                v9275 = v9168;
                v9276 = v9170;
                v9277 = v9173;
                v9278 = v9176;
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
                v9307 = v9308;
                v9309 = v9310;
                v9311 = v9312;
                v9313 = v9314;
                v9315 = v0;
                v9316 = v0;
                v9317 = v0;
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
                v9342 = v0;
                v9344 = v0;
                v9346 = v0;
                v9348 = v0;
                v9355 = v9354;
                v9357 = v9356;
                v9359 = v9358;
                v9361 = v0;
                v9363 = v0;
                v9365 = v0;
                v9367 = v0;
                v10294 = v18823;
                v10295 = v18826;
                v10296 = v18830;
                v10297 = v18834;
                v10298 = v10331;
                v10299 = v10332;
                v10300 = v10333;
                v10301 = v10334;
                v10302 = v10335;
                v10303 = v10336;
                v10304 = v10337;
                v10305 = v10338;
                v10306 = v10339;
                v10307 = v10342;
                v10308 = v10343;
                v10309 = v10344;
                v10310 = v11062;
                v10311 = v11062;
                v10312 = v11062;
                v10313 = v11036;
                v10314 = v11036;
                v10315 = v18635;
                v10316 = v18636;
                v10317 = v18662;
                v10318 = v10375;
                v10319 = v10376;
                v10320 = v10374;
                v10321 = v10375;
                v10322 = v10376;
                v10323 = v10374;
                v10324 = v10340;
                v10325 = v10341;
                v10326 = v10345;
                v10327 = v11036;
                v10328 = v10375;
                v10329 = v10376;
                v10330 = v10374;
            } else {
                let v9222 = v365 * (v8706 + v9088);
                let v18802 = ((Lanes([v10220[0], v10220[1], v10220[2], v10220[3], v10220[4], 0.0])) + v10269) * v365;
                let v9224 = v365 * (v8707 + v9089);
                let v18805 = ((Lanes([v10221[0], v10221[1], v10221[2], v10221[3], v10221[4], 0.0])) + v10270) * v365;
                let v9319: f64;
                let v9321: f64;
                let v9323: f64;
                let v9325: f64;
                let v9360: f64;
                let v10346: Lanes<6>;
                let v10347: Lanes<1>;
                let v10348: Lanes<1>;
                let v10349: Lanes<1>;
                if v2247 != 0.0 {
                    let v9226 = v2254 * v11;
                    let v18806 = v9380 * v11;
                    let v9228 = v9227 * v2254;
                    let v18807 = v9380 * v9227;
                    let v9229 = ddt(74114, v9228);
                    let v18808 = v18807 * v18741;
                    v9319 = v9210;
                    v9321 = v9226;
                    v9323 = v9229;
                    v9325 = v0;
                    v9360 = v9228;
                    v10346 = v9884;
                    v10347 = v18806;
                    v10348 = v18808;
                    v10349 = v18807;
                } else {
                    v9319 = v0;
                    v9321 = v0;
                    v9323 = v0;
                    v9325 = v9230;
                    v9360 = v0;
                    v10346 = v11062;
                    v10347 = v11036;
                    v10348 = v11036;
                    v10349 = v11036;
                }
                let v9327: f64;
                let v9329: f64;
                let v9331: f64;
                let v9333: f64;
                let v9335: f64;
                let v9337: f64;
                let v9339: f64;
                let v9341: f64;
                let v9343: f64;
                let v9345: f64;
                let v9347: f64;
                let v9349: f64;
                let v9362: f64;
                let v9364: f64;
                let v9366: f64;
                let v10350: Lanes<7>;
                let v10351: Lanes<7>;
                let v10352: Lanes<7>;
                let v10353: Lanes<1>;
                let v10354: Lanes<1>;
                let v10355: Lanes<1>;
                let v10356: Lanes<1>;
                let v10357: Lanes<1>;
                let v10358: Lanes<1>;
                let v10359: Lanes<1>;
                let v10360: Lanes<1>;
                let v10361: Lanes<1>;
                if v71 != 0.0 {
                    let v9235 = v624 * v11;
                    let v18809 = v9377 * v11;
                    let v9236 = v627 * v11;
                    let v18810 = v9378 * v11;
                    let v9237 = v621 * v11;
                    let v18811 = v9376 * v11;
                    let v9239 = v9238 * v624;
                    let v18812 = v9377 * v9238;
                    let v9240 = ddt(74134, v9239);
                    let v18813 = v18812 * v18741;
                    let v9242 = v9241 * v627;
                    let v18814 = v9378 * v9241;
                    let v9243 = ddt(74140, v9242);
                    let v18815 = v18814 * v18741;
                    let v9245 = v9244 * v621;
                    let v18816 = v9376 * v9244;
                    let v9246 = ddt(74146, v9245);
                    let v18817 = v18816 * v18741;
                    v9327 = v9231;
                    v9329 = v9233;
                    v9331 = v9197;
                    v9333 = v9235;
                    v9335 = v9236;
                    v9337 = v9237;
                    v9339 = v9240;
                    v9341 = v9243;
                    v9343 = v9246;
                    v9345 = v0;
                    v9347 = v0;
                    v9349 = v0;
                    v9362 = v9239;
                    v9364 = v9242;
                    v9366 = v9245;
                    v10350 = v10251;
                    v10351 = v10252;
                    v10352 = v10250;
                    v10353 = v18809;
                    v10354 = v18810;
                    v10355 = v18811;
                    v10356 = v18813;
                    v10357 = v18815;
                    v10358 = v18817;
                    v10359 = v18812;
                    v10360 = v18814;
                    v10361 = v18816;
                } else {
                    v9327 = v0;
                    v9329 = v0;
                    v9331 = v0;
                    v9333 = v0;
                    v9335 = v0;
                    v9337 = v0;
                    v9339 = v0;
                    v9341 = v0;
                    v9343 = v0;
                    v9345 = v9247;
                    v9347 = v9248;
                    v9349 = v9249;
                    v9362 = v0;
                    v9364 = v0;
                    v9366 = v0;
                    v10350 = v18635;
                    v10351 = v18636;
                    v10352 = v18662;
                    v10353 = v10375;
                    v10354 = v10376;
                    v10355 = v10374;
                    v10356 = v10375;
                    v10357 = v10376;
                    v10358 = v10374;
                    v10359 = v10375;
                    v10360 = v10376;
                    v10361 = v10374;
                }
                v9275 = v0;
                v9276 = v0;
                v9277 = v0;
                v9278 = v0;
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
                v9307 = v0;
                v9309 = v0;
                v9311 = v0;
                v9313 = v0;
                v9315 = v9222;
                v9316 = v9224;
                v9317 = v9225;
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
                v9342 = v9343;
                v9344 = v9345;
                v9346 = v9347;
                v9348 = v9349;
                v9355 = v0;
                v9357 = v0;
                v9359 = v0;
                v9361 = v9360;
                v9363 = v9362;
                v9365 = v9364;
                v9367 = v9366;
                v10294 = v11062;
                v10295 = v11062;
                v10296 = v17770;
                v10297 = v17771;
                v10298 = v18818;
                v10299 = v18819;
                v10300 = v18820;
                v10301 = v18661;
                v10302 = v18662;
                v10303 = v10382;
                v10304 = v10374;
                v10305 = v10382;
                v10306 = v10374;
                v10307 = v11062;
                v10308 = v11036;
                v10309 = v11036;
                v10310 = v18802;
                v10311 = v18805;
                v10312 = v10346;
                v10313 = v10347;
                v10314 = v10348;
                v10315 = v10350;
                v10316 = v10351;
                v10317 = v10352;
                v10318 = v10353;
                v10319 = v10354;
                v10320 = v10355;
                v10321 = v10356;
                v10322 = v10357;
                v10323 = v10358;
                v10324 = v10382;
                v10325 = v10374;
                v10326 = v11036;
                v10327 = v10349;
                v10328 = v10359;
                v10329 = v10360;
                v10330 = v10361;
            }
            let v9350: f64;
            let v9351: f64;
            let v9352: f64;
            if v9 != 0.0 {
                v9350 = v9250;
                v9351 = v0;
                v9352 = v0;
            } else {
                v9350 = v0;
                v9351 = v9251;
                v9352 = v9252;
            }
            let v19272 = v18723[0];
            let v19273 = v18723[1];
            let v19274 = v18723[2];
            let v19275 = v18723[3];
            let v19276 = v18723[4];
            let v19277 = v18723[5];
            let v19278 = v10279[0];
            let v19279 = v10279[1];
            let v19280 = v10279[2];
            let v19281 = v10279[3];
            let v19282 = v10279[4];
            let v19283 = v10279[5];
            let v19284 = v10280[0];
            let v19285 = v10280[1];
            let v19286 = v10280[2];
            let v19287 = v10280[3];
            let v19288 = v10280[4];
            let v19289 = v10280[5];
            let v19290 = v10281[0];
            let v19291 = v10281[1];
            let v19292 = v10281[2];
            let v19293 = v10281[3];
            let v19294 = v10282[0];
            let v19295 = v10282[1];
            let v19296 = v10282[2];
            let v19297 = v10282[3];
            let v19298 = v10282[4];
            let v19299 = v10283[0];
            let v19300 = v10283[1];
            let v19301 = v10283[2];
            let v19302 = v10283[3];
            let v19303 = v10283[4];
            let v19304 = v18743[0];
            let v19305 = v18743[1];
            let v19306 = v18743[2];
            let v19307 = v18743[3];
            let v19308 = v18743[4];
            let v19309 = v18743[5];
            let v19310 = v18743[6];
            let v19311 = v18743[7];
            let v19312 = v18743[8];
            let v19313 = v18743[9];
            let v19314 = v18745[0];
            let v19315 = v18745[1];
            let v19316 = v18745[2];
            let v19317 = v18745[3];
            let v19318 = v18745[4];
            let v19319 = v18745[5];
            let v19320 = v18745[6];
            let v19321 = v18745[7];
            let v19322 = v18745[8];
            let v19323 = v18747[0];
            let v19324 = v18747[1];
            let v19325 = v18747[2];
            let v19326 = v18747[3];
            let v19327 = v18747[4];
            let v19328 = v18747[5];
            let v19329 = v18747[6];
            let v19330 = v9381[0];
            let v19331 = v18762[0];
            let v19332 = v18762[1];
            let v19333 = v18762[2];
            let v19334 = v18762[3];
            let v19335 = v18762[4];
            let v19336 = v18762[5];
            let v19337 = v18762[6];
            let v19338 = v18782[0];
            let v19339 = v18782[1];
            let v19340 = v18782[2];
            let v19341 = v18782[3];
            let v19342 = v18782[4];
            let v19343 = v18782[5];
            let v19344 = v18782[6];
            let v19345 = v18788[0];
            let v19346 = v18788[1];
            let v19347 = v18788[2];
            let v19348 = v18788[3];
            let v19349 = v18788[4];
            let v19350 = v18788[5];
            let v19351 = v18788[6];
            let v19352 = v10287[0];
            let v19353 = v10287[1];
            let v19354 = v10288[0];
            let v19355 = v10289[0];
            let v19356 = v10289[1];
            let v19357 = v10289[2];
            let v19358 = v10289[3];
            let v19359 = v10289[4];
            let v19360 = v10289[5];
            let v19361 = v10290[0];
            let v19362 = v10291[0];
            let v19363 = v10292[0];
            let v19364 = v10294[0];
            let v19365 = v10294[1];
            let v19366 = v10294[2];
            let v19367 = v10294[3];
            let v19368 = v10294[4];
            let v19369 = v10294[5];
            let v19370 = v10295[0];
            let v19371 = v10295[1];
            let v19372 = v10295[2];
            let v19373 = v10295[3];
            let v19374 = v10295[4];
            let v19375 = v10295[5];
            let v19376 = v10296[0];
            let v19377 = v10296[1];
            let v19378 = v10296[2];
            let v19379 = v10297[0];
            let v19380 = v10297[1];
            let v19381 = v10297[2];
            let v19382 = v10298[0];
            let v19383 = v10298[1];
            let v19384 = v10299[0];
            let v19385 = v10299[1];
            let v19386 = v10300[0];
            let v19387 = v10300[1];
            let v19388 = v10301[0];
            let v19389 = v10301[1];
            let v19390 = v10301[2];
            let v19391 = v10301[3];
            let v19392 = v10301[4];
            let v19393 = v10301[5];
            let v19394 = v10301[6];
            let v19395 = v10302[0];
            let v19396 = v10302[1];
            let v19397 = v10302[2];
            let v19398 = v10302[3];
            let v19399 = v10302[4];
            let v19400 = v10302[5];
            let v19401 = v10302[6];
            let v19402 = v10303[0];
            let v19403 = v10304[0];
            let v19404 = v10305[0];
            let v19405 = v10306[0];
            let v19406 = v10307[0];
            let v19407 = v10307[1];
            let v19408 = v10307[2];
            let v19409 = v10307[3];
            let v19410 = v10307[4];
            let v19411 = v10307[5];
            let v19412 = v10308[0];
            let v19413 = v10309[0];
            let v19414 = v10310[0];
            let v19415 = v10310[1];
            let v19416 = v10310[2];
            let v19417 = v10310[3];
            let v19418 = v10310[4];
            let v19419 = v10310[5];
            let v19420 = v10311[0];
            let v19421 = v10311[1];
            let v19422 = v10311[2];
            let v19423 = v10311[3];
            let v19424 = v10311[4];
            let v19425 = v10311[5];
            let v19426 = v10312[0];
            let v19427 = v10312[1];
            let v19428 = v10312[2];
            let v19429 = v10312[3];
            let v19430 = v10312[4];
            let v19431 = v10312[5];
            let v19432 = v10313[0];
            let v19433 = v10314[0];
            let v19434 = v10315[0];
            let v19435 = v10315[1];
            let v19436 = v10315[2];
            let v19437 = v10315[3];
            let v19438 = v10315[4];
            let v19439 = v10315[5];
            let v19440 = v10315[6];
            let v19441 = v10316[0];
            let v19442 = v10316[1];
            let v19443 = v10316[2];
            let v19444 = v10316[3];
            let v19445 = v10316[4];
            let v19446 = v10316[5];
            let v19447 = v10316[6];
            let v19448 = v10317[0];
            let v19449 = v10317[1];
            let v19450 = v10317[2];
            let v19451 = v10317[3];
            let v19452 = v10317[4];
            let v19453 = v10317[5];
            let v19454 = v10317[6];
            let v19455 = v10318[0];
            let v19456 = v10319[0];
            let v19457 = v10320[0];
            let v19458 = v10321[0];
            let v19459 = v10322[0];
            let v19460 = v10323[0];
            let v19461 = v18781[0];
            let v19462 = v18781[1];
            let v19463 = v18781[2];
            let v19464 = v18781[3];
            let v19465 = v18781[4];
            let v19466 = v18781[5];
            let v19467 = v18781[6];
            let v19468 = v18787[0];
            let v19469 = v18787[1];
            let v19470 = v18787[2];
            let v19471 = v18787[3];
            let v19472 = v18787[4];
            let v19473 = v18787[5];
            let v19474 = v18787[6];
            let v19475 = v10293[0];
            let v19476 = v10324[0];
            let v19477 = v10325[0];
            let v19478 = v10326[0];
            let v19479 = v10327[0];
            let v19480 = v10328[0];
            let v19481 = v10329[0];
            let v19482 = v10330[0];
        stamper.stamp_potential_branch_local(Some(5), Some(10), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            v9253,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(5), None, 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            v9254,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(4), Some(10), 2, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            2,
            v9255,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9101),
            [6, 7, 10, 11, 12, 17],
            [v19272, v19273, v19274, v19275, v19276, v19277],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (v9256),
            [6, 7, 10, 11, 12, 17],
            [v19278, v19279, v19280, v19281, v19282, v19283],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(6),
            multiplicity * (v9257),
            [6, 7, 10, 11, 12, 17],
            [v19284, v19285, v19286, v19287, v19288, v19289],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(12),
            multiplicity * (v9258),
            [6, 7, 11, 12],
            [v19290, v19291, v19292, v19293],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(2),
            multiplicity * (v9259),
            [0, 2, 6, 7, 10],
            [v19294, v19295, v19296, v19297, v19298],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(7), Some(2), 3, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            3,
            v9260,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(0),
            Some(6),
            multiplicity * (v9261),
            [0, 2, 6, 7, 10],
            [v19299, v19300, v19301, v19302, v19303],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(0), Some(6), 4, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            4,
            v9262,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(7),
            multiplicity * (v9113),
            [6, 7, 10, 11, 12, 13, 15, 16, 17, 18],
            [v19304, v19305, v19306, v19307, v19308, v19309, v19310, v19311, v19312, v19313],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9115),
            [6, 7, 10, 11, 12, 15, 16, 17, 18],
            [v19314, v19315, v19316, v19317, v19318, v19319, v19320, v19321, v19322],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(12),
            Some(7),
            multiplicity * (v9117),
            [6, 7, 10, 11, 12, 13, 17],
            [v19323, v19324, v19325, v19326, v19327, v19328, v19329],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9119),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(14),
            None,
            multiplicity * (v9129),
            [14],
            [v19330],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(14),
            None,
            multiplicity * (v9130),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9131),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9132),
            [6, 7, 10, 11, 12, 14, 17],
            [v19331, v19332, v19333, v19334, v19335, v19336, v19337],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(7),
            multiplicity * (v9142),
            [6, 7, 10, 11, 12, 14, 17],
            [v19338, v19339, v19340, v19341, v19342, v19343, v19344],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(6),
            multiplicity * (v9144),
            [6, 7, 10, 11, 12, 14, 17],
            [v19345, v19346, v19347, v19348, v19349, v19350, v19351],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(2),
            multiplicity * (v9263),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(6),
            multiplicity * (v9264),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(6),
            multiplicity * (v9265),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(7),
            multiplicity * (v9266),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(12),
            multiplicity * (v9267),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(11),
            multiplicity * (v9268),
            [1, 11],
            [v19352, v19353],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(1), Some(11), 5, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            5,
            v9269,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9270),
            [10],
            [v19354],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            None,
            multiplicity * (v9271),
            [6, 7, 10, 11, 12, 17],
            [v19355, v19356, v19357, v19358, v19359, v19360],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9272),
            [10],
            [v19361],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9273),
            [10],
            [v19362],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9274),
            [10],
            [v19363],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(12),
            multiplicity * (v9275),
            [6, 7, 10, 11, 12, 17],
            [v19364, v19365, v19366, v19367, v19368, v19369],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(12),
            multiplicity * (v9276),
            [6, 7, 10, 11, 12, 17],
            [v19370, v19371, v19372, v19373, v19374, v19375],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(7),
            multiplicity * (v9277),
            [7, 10, 12],
            [v19376, v19377, v19378],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(6),
            multiplicity * (v9278),
            [6, 10, 12],
            [v19379, v19380, v19381],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(12),
            multiplicity * (v9279),
            [4, 12],
            [v19382, v19383],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(4), Some(12), 6, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            6,
            v9281,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(9),
            Some(12),
            multiplicity * (v9283),
            [9, 12],
            [v19384, v19385],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(8),
            Some(12),
            multiplicity * (v9285),
            [8, 12],
            [v19386, v19387],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(9), Some(12), 7, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            7,
            v9287,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(8), Some(12), 8, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            8,
            v9289,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(18),
            None,
            multiplicity * (v9291),
            [6, 7, 10, 11, 12, 17, 18],
            [v19388, v19389, v19390, v19391, v19392, v19393, v19394],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(13),
            None,
            multiplicity * (v9293),
            [6, 7, 10, 11, 12, 13, 17],
            [v19395, v19396, v19397, v19398, v19399, v19400, v19401],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(18),
            None,
            multiplicity * (v9295),
            [18],
            [v19402],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9297),
            [13],
            [v19403],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(18),
            None,
            multiplicity * (v9299),
            [18],
            [v19404],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9301),
            [13],
            [v19405],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(18), None, 9, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            9,
            v9303,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(13), None, 10, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            10,
            v9305,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(17),
            None,
            multiplicity * (v9307),
            [6, 7, 10, 11, 12, 17],
            [v19406, v19407, v19408, v19409, v19410, v19411],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9309),
            [17],
            [v19412],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9311),
            [17],
            [v19413],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(17), None, 11, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            11,
            v9313,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9315),
            [6, 7, 10, 11, 12, 17],
            [v19414, v19415, v19416, v19417, v19418, v19419],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(6),
            multiplicity * (v9316),
            [6, 7, 10, 11, 12, 17],
            [v19420, v19421, v19422, v19423, v19424, v19425],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(3), Some(12), 12, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            12,
            v9317,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(17),
            None,
            multiplicity * (v9318),
            [6, 7, 10, 11, 12, 17],
            [v19426, v19427, v19428, v19429, v19430, v19431],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9320),
            [17],
            [v19432],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9322),
            [17],
            [v19433],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(17), None, 13, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            13,
            v9324,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(15),
            None,
            multiplicity * (v9326),
            [6, 7, 10, 11, 12, 15, 17],
            [v19434, v19435, v19436, v19437, v19438, v19439, v19440],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(16),
            None,
            multiplicity * (v9328),
            [6, 7, 10, 11, 12, 16, 17],
            [v19441, v19442, v19443, v19444, v19445, v19446, v19447],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(13),
            None,
            multiplicity * (v9330),
            [6, 7, 10, 11, 12, 13, 17],
            [v19448, v19449, v19450, v19451, v19452, v19453, v19454],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (v9332),
            [15],
            [v19455],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(16),
            None,
            multiplicity * (v9334),
            [16],
            [v19456],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9336),
            [13],
            [v19457],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (v9338),
            [15],
            [v19458],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(16),
            None,
            multiplicity * (v9340),
            [16],
            [v19459],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9342),
            [13],
            [v19460],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(15), None, 14, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            14,
            v9344,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(16), None, 15, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            15,
            v9346,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(13), None, 16, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            16,
            v9348,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(18), None, 17, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            17,
            v9350,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(15), None, 18, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            18,
            v9351,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_potential_branch_local(Some(16), None, 19, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            19,
            v9352,
            [],
            [],
            [],
            [],
        );
        self.canonical_reactive[0] = v9253;
        self.canonical_reactive[1] = v9254;
        self.canonical_reactive[2] = v9255;
        self.canonical_reactive[3] = v9101;
        self.canonical_reactive[4] = v9256;
        self.canonical_reactive[5] = v9257;
        self.canonical_reactive[6] = v9258;
        self.canonical_reactive[7] = v9259;
        self.canonical_reactive[8] = v9260;
        self.canonical_reactive[9] = v9261;
        self.canonical_reactive[10] = v9262;
        self.canonical_reactive[11] = v9113;
        self.canonical_reactive[12] = v9115;
        self.canonical_reactive[13] = v9117;
        self.canonical_reactive[14] = v9119;
        self.canonical_reactive[15] = v9129;
        self.canonical_reactive[16] = v9130;
        self.canonical_reactive[17] = v9131;
        self.canonical_reactive[18] = v9132;
        self.canonical_reactive[19] = v9141;
        self.canonical_reactive[20] = v19461;
        self.canonical_reactive[21] = v19462;
        self.canonical_reactive[22] = v19463;
        self.canonical_reactive[23] = v19464;
        self.canonical_reactive[24] = v19465;
        self.canonical_reactive[25] = v19466;
        self.canonical_reactive[26] = v19467;
        self.canonical_reactive[27] = v9143;
        self.canonical_reactive[28] = v19468;
        self.canonical_reactive[29] = v19469;
        self.canonical_reactive[30] = v19470;
        self.canonical_reactive[31] = v19471;
        self.canonical_reactive[32] = v19472;
        self.canonical_reactive[33] = v19473;
        self.canonical_reactive[34] = v19474;
        self.canonical_reactive[35] = v9263;
        self.canonical_reactive[36] = v9264;
        self.canonical_reactive[37] = v9265;
        self.canonical_reactive[38] = v9266;
        self.canonical_reactive[39] = v9267;
        self.canonical_reactive[40] = v9268;
        self.canonical_reactive[41] = v9269;
        self.canonical_reactive[42] = v9270;
        self.canonical_reactive[43] = v9271;
        self.canonical_reactive[44] = v9272;
        self.canonical_reactive[45] = v9353;
        self.canonical_reactive[46] = v19475;
        self.canonical_reactive[47] = v9274;
        self.canonical_reactive[48] = v9275;
        self.canonical_reactive[49] = v9276;
        self.canonical_reactive[50] = v9277;
        self.canonical_reactive[51] = v9278;
        self.canonical_reactive[52] = v9279;
        self.canonical_reactive[53] = v9281;
        self.canonical_reactive[54] = v9283;
        self.canonical_reactive[55] = v9285;
        self.canonical_reactive[56] = v9287;
        self.canonical_reactive[57] = v9289;
        self.canonical_reactive[58] = v9291;
        self.canonical_reactive[59] = v9293;
        self.canonical_reactive[60] = v9295;
        self.canonical_reactive[61] = v9297;
        self.canonical_reactive[62] = v9355;
        self.canonical_reactive[63] = v19476;
        self.canonical_reactive[64] = v9357;
        self.canonical_reactive[65] = v19477;
        self.canonical_reactive[66] = v9303;
        self.canonical_reactive[67] = v9305;
        self.canonical_reactive[68] = v9307;
        self.canonical_reactive[69] = v9309;
        self.canonical_reactive[70] = v9359;
        self.canonical_reactive[71] = v19478;
        self.canonical_reactive[72] = v9313;
        self.canonical_reactive[73] = v9315;
        self.canonical_reactive[74] = v9316;
        self.canonical_reactive[75] = v9317;
        self.canonical_reactive[76] = v9318;
        self.canonical_reactive[77] = v9320;
        self.canonical_reactive[78] = v9361;
        self.canonical_reactive[79] = v19479;
        self.canonical_reactive[80] = v9324;
        self.canonical_reactive[81] = v9326;
        self.canonical_reactive[82] = v9328;
        self.canonical_reactive[83] = v9330;
        self.canonical_reactive[84] = v9332;
        self.canonical_reactive[85] = v9334;
        self.canonical_reactive[86] = v9336;
        self.canonical_reactive[87] = v9363;
        self.canonical_reactive[88] = v19480;
        self.canonical_reactive[89] = v9365;
        self.canonical_reactive[90] = v19481;
        self.canonical_reactive[91] = v9367;
        self.canonical_reactive[92] = v19482;
        self.canonical_reactive[93] = v9344;
        self.canonical_reactive[94] = v9346;
        self.canonical_reactive[95] = v9348;
        self.canonical_reactive[96] = v9350;
        self.canonical_reactive[97] = v9351;
        self.canonical_reactive[98] = v9352;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(7),
            &[6, 7, 10, 11, 12, 14, 17],
            &[cached[20], cached[21], cached[22], cached[23], cached[24], cached[25], cached[26]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(6),
            &[6, 7, 10, 11, 12, 14, 17],
            &[cached[28], cached[29], cached[30], cached[31], cached[32], cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            None,
            &[10],
            &[cached[46]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(18),
            None,
            &[18],
            &[cached[63]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(13),
            None,
            &[13],
            &[cached[65]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(17),
            None,
            &[17],
            &[cached[71]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(17),
            None,
            &[17],
            &[cached[79]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(15),
            None,
            &[15],
            &[cached[88]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(16),
            None,
            &[16],
            &[cached[90]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(13),
            None,
            &[13],
            &[cached[92]],
            &[],
            &[],
            multiplicity,
        );
    }

}

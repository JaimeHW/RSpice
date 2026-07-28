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
            let v9353 = 1e0f64;
            let v9354 = Lanes([1e0f64; 1]);
            let v9355 = Lanes([1e0f64; 1]);
            let v9356 = Lanes([1e0f64; 1]);
            let v9357 = Lanes([1e0f64; 1]);
            let v9358 = Lanes([1e0f64; 1]);
            let v9359 = Lanes([1e0f64; 1]);
            let v9360 = Lanes([1e0f64; 1]);
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
            let v10344 = Lanes([0e0f64; 1]);
            let v10345 = Lanes([0e0f64; 1]);
            let v10346 = Lanes([0e0f64; 1]);
            let v10350 = Lanes([0e0f64; 2]);
            let v10351 = Lanes([0e0f64; 2]);
            let v10352 = Lanes([0e0f64; 1]);
            let v10359 = Lanes([0e0f64; 1]);
            let v10360 = -1e0f64;
            let v10405 = 2e0f64;
            let v10474 = Lanes([0e0f64; 3]);
            let v10485 = -8.75e-1f64;
            let v10500 = Lanes([0e0f64; 2]);
            let v10501 = Lanes([0e0f64; 3]);
            let v10549 = Lanes([0e0f64; 5]);
            let v10595 = Lanes([0e0f64; 4]);
            let v10630 = Lanes([0e0f64; 4]);
            let v10900 = -6.666666666666667e-1f64;
            let v10969 = -6.666666666666667e-1f64;
            let v11006 = Lanes([0e0f64; 1]);
            let v11032 = Lanes([0e0f64; 6]);
            let v11101 = -8.75e-1f64;
            let v11274 = 0e0f64;
            let v11357 = -8.75e-1f64;
            let v12018 = -7.5e-1f64;
            let v12035 = -7.5e-1f64;
            let v12092 = -7.5e-1f64;
            let v12607 = -8.75e-1f64;
            let v12813 = -8.75e-1f64;
            let v13241 = -7.5e-1f64;
            let v13282 = -7.5e-1f64;
            let v13485 = -7.5e-1f64;
            let v13532 = -7.5e-1f64;
            let v14220 = -8.75e-1f64;
            let v14431 = -6.666666666666667e-1f64;
            let v14499 = -7.5e-1f64;
            let v14810 = -6.666666666666667e-1f64;
            let v14949 = -5e-1f64;
            let v15033 = -8.75e-1f64;
            let v15735 = -6.666666666666667e-1f64;
            let v15740 = -6.666666666666667e-1f64;
            let v16065 = -6.666666666666667e-1f64;
            let v16235 = -6.666666666666667e-1f64;
            let v16240 = -6.666666666666667e-1f64;
            let v16565 = -6.666666666666667e-1f64;
            let v16799 = -6.666666666666667e-1f64;
            let v16804 = -6.666666666666667e-1f64;
            let v17138 = -6.666666666666667e-1f64;
            let v17327 = -6.666666666666667e-1f64;
            let v17332 = -6.666666666666667e-1f64;
            let v17666 = -6.666666666666667e-1f64;
            let v17740 = Lanes([0e0f64; 3]);
            let v17741 = Lanes([0e0f64; 3]);
            let v18429 = Lanes([0e0f64; 5]);
            let v18604 = Lanes([0e0f64; 3]);
            let v18605 = Lanes([0e0f64; 7]);
            let v18606 = Lanes([0e0f64; 7]);
            let v18631 = Lanes([0e0f64; 7]);
            let v18632 = Lanes([0e0f64; 7]);
            let v18633 = Lanes([0e0f64; 8]);
            let v18711 = ddt_scale();
            let v18759 = Lanes([0e0f64; 2]);
            let v18788 = Lanes([0e0f64; 2]);
            let v18789 = Lanes([0e0f64; 2]);
            let v18790 = Lanes([0e0f64; 2]);
            let v19011 = -7.5e-1f64;
            let v19058 = -7.5e-1f64;
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
            let v10335 = ((Lanes([v9354[0], 0.0])) - (Lanes([0.0, v9355[0]]))) * v365;
            let v608 = v365 * (v606 - v603);
            let v10339 = ((Lanes([0.0, v9356[0]])) - (Lanes([v9355[0], 0.0]))) * v365;
            let v611 = v365 * (v609 - v603);
            let v10343 = ((Lanes([0.0, v9357[0]])) - (Lanes([v9355[0], 0.0]))) * v365;
            let v7879: f64;
            let v7880: f64;
            let v8997: f64;
            let v9004: f64;
            let v9029: f64;
            let v9036: f64;
            let v9371: Lanes<2>;
            let v9372: Lanes<2>;
            let v9373: Lanes<1>;
            let v9374: Lanes<1>;
            let v9375: Lanes<1>;
            let v9376: Lanes<1>;
            if v565 != 0.0 {
                let v615 = v365 * (v609 - v602);
                let v10356 = ((Lanes([0.0, v9357[0]])) - (Lanes([v9354[0], 0.0]))) * v365;
                let v8998: f64;
                let v9005: f64;
                let v9377: Lanes<1>;
                let v9378: Lanes<1>;
                if v71 != 0.0 {
                    let v619 = v617 * v618;
                    let v10357 = v9360 * v617;
                    let v622 = v620 * v621;
                    let v10358 = v9361 * v620;
                    v8998 = v619;
                    v9005 = v622;
                    v9377 = v10357;
                    v9378 = v10358;
                } else {
                    v8998 = v0;
                    v9005 = v0;
                    v9377 = v10352;
                    v9378 = v10344;
                }
                v7879 = v615;
                v7880 = v611;
                v8997 = v8998;
                v9004 = v9005;
                v9029 = v0;
                v9036 = v0;
                v9371 = v10356;
                v9372 = v10343;
                v9373 = v9377;
                v9374 = v9378;
                v9375 = v10345;
                v9376 = v10346;
            } else {
                let v9006: f64;
                let v9030: f64;
                let v9037: f64;
                let v9379: Lanes<1>;
                let v9380: Lanes<1>;
                let v9381: Lanes<1>;
                if v71 != 0.0 {
                    let v625 = v623 * v624;
                    let v10347 = v9362 * v623;
                    let v628 = v626 * v627;
                    let v10348 = v9363 * v626;
                    let v630 = v629 * v621;
                    let v10349 = v9361 * v629;
                    v9006 = v630;
                    v9030 = v625;
                    v9037 = v628;
                    v9379 = v10349;
                    v9380 = v10347;
                    v9381 = v10348;
                } else {
                    v9006 = v0;
                    v9030 = v0;
                    v9037 = v0;
                    v9379 = v10344;
                    v9380 = v10345;
                    v9381 = v10346;
                }
                v7879 = v0;
                v7880 = v0;
                v8997 = v0;
                v9004 = v9006;
                v9029 = v9030;
                v9036 = v9037;
                v9371 = v10350;
                v9372 = v10351;
                v9373 = v10352;
                v9374 = v9379;
                v9375 = v9380;
                v9376 = v9381;
            }
            let v632 = if v631 > v0 { 1.0 } else { 0.0 };
            let v633 = if v35 > v0 { 1.0 } else { 0.0 };
            let v634 = if v632 != 0.0 && v633 != 0.0 { 1.0 } else { 0.0 };
            let v638: f64;
            let v9382: Lanes<1>;
            if v634 != 0.0 {
                let v636 = if v635 > v0 { 1.0 } else { 0.0 };
                let v637: f64;
                let v9383: Lanes<1>;
                if v636 != 0.0 {
                    v637 = v635;
                    v9383 = v9364;
                } else {
                    v637 = v0;
                    v9383 = v10359;
                }
                v638 = v637;
                v9382 = v9383;
            } else {
                v638 = v0;
                v9382 = v10359;
            }
            let v639 = if v605 >= v0 { 1.0 } else { 0.0 };
            let v782: f64;
            let v820: f64;
            let v824: f64;
            let v6046: f64;
            let v6048: f64;
            let v7825: f64;
            let v9384: Lanes<3>;
            let v9385: Lanes<2>;
            let v9386: Lanes<3>;
            if v639 != 0.0 {
                let v10368 = Lanes([0.0, v10343[0], v10343[1]]);
                let v10369 = Lanes([0.0, v10339[0], v10339[1]]);
                v782 = v611;
                v820 = v605;
                v824 = v608;
                v6046 = v1;
                v6048 = v0;
                v7825 = v1;
                v9384 = v10368;
                v9385 = v10335;
                v9386 = v10369;
            } else {
                let v641 = -v605;
                let v10361 = v10335 * v10360;
                let v642 = v608 - v605;
                let v10364 = (Lanes([0.0, v10339[0], v10339[1]])) - (Lanes([v10335[0], v10335[1], 0.0]));
                let v643 = v611 - v605;
                let v10367 = (Lanes([0.0, v10343[0], v10343[1]])) - (Lanes([v10335[0], v10335[1], 0.0]));
                v782 = v643;
                v820 = v641;
                v824 = v642;
                v6046 = v0;
                v6048 = v1;
                v7825 = v640;
                v9384 = v10367;
                v9385 = v10361;
                v9386 = v10364;
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
            let v10376 = ((v9382 * v656) * v10360) - (((v9382 * v654) + (v9382 * v653)) * v659);
            let v662 = v207 * v652;
            let v663 = v206 / v662;
            let v10380 = (((v9382 * v207) * v663) * v10360) / v662;
            let v664 = v663 * v663;
            let v10381 = v10380 * v663;
            let v10382 = v10381 + v10381;
            let v665 = v1 / v663;
            let v10385 = ((v10380 * v665) * v10360) / v663;
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
            let v10386 = v9382 / v65;
            let v702 = (v699.powf(v700)) / v698;
            let v10391 = (v10386 * (v700 * (v699.powf((v700 - v9353))))) / v698;
            let v704 = v703 * v665;
            let v10392 = v10385 * v703;
            let v711 = v79 * v699;
            let v713 = (v707 + (v708 * v699)) + (v711 * v699);
            let v10398 = (v10386 * v708) + (((v10386 * v79) * v699) + (v10386 * v711));
            let v714 = v1 - v699;
            let v10399 = v10386 * v10360;
            let v716 = v713 - (v26 * v714);
            let v717 = (v705 * v21) / v716;
            let v10404 = (((v10398 - (v10399 * v26)) * v717) * v10360) / v716;
            let v718 = v661.sqrt();
            let v10408 = v10376 * (v9353 / (v10405 * v718));
            let v719 = v661 * v718;
            let v10411 = (v10376 * v718) + (v10408 * v661);
            let v19445 = v699.sqrt();
            let v723 = v720 * (v699 * v19445);
            let v725 = (-v661) / v78;
            let v730 = ((v725 * v663) + ((v121 / v78) * v209)).exp();
            let v731 = v723 * v730;
            let v10423 = (((v10386 * (v721 * v19445)) * v720) * v730) + ((((((v10376 * v10360) / v78) * v663) + (v10380 * v725)) * v730) * v723);
            let v732 = v665.sqrt();
            let v10426 = v10385 * (v9353 / (v10405 * v732));
            let v733 = v231 * v732;
            let v10427 = v10426 * v231;
            let v734 = v733 * v733;
            let v10428 = v10427 * v733;
            let v10429 = v10428 + v10428;
            let v735 = v731 * v731;
            let v10430 = v10423 * v731;
            let v10431 = v10430 + v10430;
            let v736 = v735 * v233;
            let v10432 = v10431 * v233;
            let v766: f64;
            let v9387: Lanes<1>;
            if v184 != 0.0 {
                let v737 = v78 * v665;
                let v738 = v489 / v731;
                let v739 = v738.ln();
                let v740 = v737 * v739;
                let v10450 = ((v10385 * v78) * v739) + (((((v10423 * v738) * v10360) / v731) * (v9353 / v738)) * v737);
                v766 = v740;
                v9387 = v10450;
            } else {
                let v741 = v78 * v665;
                let v742 = v477 / v731;
                let v743 = v742.ln();
                let v744 = v741 * v743;
                let v10441 = ((v10385 * v78) * v743) + (((((v10423 * v742) * v10360) / v731) * (v9353 / v742)) * v741);
                v766 = v744;
                v9387 = v10441;
            }
            let v745 = v123 / v490;
            let v747 = (v745 * v665).sqrt();
            let v749 = v490 * v748;
            let v750 = v749 * v747;
            let v10455 = ((v10385 * v745) * (v9353 / (v10405 * v747))) * v749;
            let v758: f64;
            let v1240: f64;
            let v1262: f64;
            let v9388: Lanes<1>;
            let v9389: Lanes<1>;
            let v9390: Lanes<1>;
            if v565 != 0.0 {
                let v751 = v731 / v489;
                let v10464 = v10423 / v489;
                v758 = v751;
                v1240 = v0;
                v1262 = v0;
                v9388 = v10464;
                v9389 = v10359;
                v9390 = v10359;
            } else {
                let v752 = v78 * v211;
                let v754 = (v752 * v665).sqrt();
                let v10459 = (v10385 * v752) * (v9353 / (v10405 * v754));
                let v755 = v731 / v33;
                let v756 = v755 * v755;
                let v10461 = (v10423 / v33) * v755;
                let v10462 = v10461 + v10461;
                let v757 = v731 / v477;
                let v10463 = v10423 / v477;
                v758 = v757;
                v1240 = v754;
                v1262 = v756;
                v9388 = v10463;
                v9389 = v10459;
                v9390 = v10462;
            }
            let v759 = v758 * v758;
            let v10465 = v9388 * v758;
            let v10466 = v10465 + v10465;
            let v760 = v745 / v663;
            let v762 = (v78 * v760).sqrt();
            let v10473 = ((((v10380 * v760) * v10360) / v663) * v78) * (v9353 / (v10405 * v762));
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
            let v9391: Lanes<3>;
            let v9392: Lanes<3>;
            if v784 != 0.0 {
                let v785 = v782 - v783;
                let v786 = v779 - v783;
                let v787 = v785 * v785;
                let v10475 = v9384 * v785;
                let v10476 = v10475 + v10475;
                let v788 = v786 * v786;
                let v789 = v787 * v787;
                let v10477 = v10476 * v787;
                let v791 = v789 * v787;
                let v10484 = ((((v10477 + v10477) * v787) + (v10476 * v789)) * v787) + (v10476 * v791);
                let v794 = ((v788 * v788) * v788) * v788;
                let v795 = (v791 * v787) + v794;
                let v812: f64;
                let v9393: Lanes<3>;
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
                    let mut v9394: Lanes<3> = Lanes([0.0; 3]);
                    v801 = v0;
                    v803 = v795;
                    v9394 = v10484;
                    loop {
                        let v802 = if v801 < v806 { 1.0 } else { 0.0 };
                        if v802 == 0.0 {
                            break;
                        }
                        let v804 = v803.sqrt();
                        let v19241 = v9394 * (v9353 / (v10405 * v804));
                        let v805 = v801 + v1;
                        v801 = v805;
                        v803 = v804;
                        v9394 = v19241;
                    }
                    v812 = v803;
                    v9393 = v9394;
                } else {
                    let v811 = v795.powf(v810);
                    let v10488 = v10484 * (v810 * (v795.powf(v10485)));
                    v812 = v811;
                    v9393 = v10488;
                }
                let v813 = v1 / v812;
                let v10491 = ((v9393 * v813) * v10360) / v812;
                let v814 = v785 * v786;
                let v10495 = ((v9384 * v786) * v813) + (v10491 * v814);
                let v816 = v786 * v794;
                let v818 = (v816 * v813) / v795;
                let v10499 = ((v10491 * v816) - (v10484 * v818)) / v795;
                let v819 = v783 + (v814 * v813);
                v831 = v819;
                v836 = v818;
                v9391 = v10495;
                v9392 = v10499;
            } else {
                v831 = v782;
                v836 = v1;
                v9391 = v9384;
                v9392 = v10474;
            }
            let v822 = if v820 > v821 { 1.0 } else { 0.0 };
            let v823: f64;
            let v9395: Lanes<2>;
            if v822 != 0.0 {
                v823 = v821;
                v9395 = v10500;
            } else {
                v823 = v820;
                v9395 = v9385;
            }
            let v825 = if v824 > v821 { 1.0 } else { 0.0 };
            let v826: f64;
            let v9396: Lanes<3>;
            if v825 != 0.0 {
                v826 = v821;
                v9396 = v10501;
            } else {
                v826 = v824;
                v9396 = v9386;
            }
            let v828 = if v824 < v827 { 1.0 } else { 0.0 };
            let v830: f64;
            let v9397: Lanes<3>;
            if v828 != 0.0 {
                v830 = v829;
                v9397 = v10501;
            } else {
                v830 = v826;
                v9397 = v9396;
            }
            let v833 = if v831 < v832 { 1.0 } else { 0.0 };
            let v835: f64;
            let v9398: Lanes<3>;
            if v833 != 0.0 {
                v835 = v834;
                v9398 = v10474;
            } else {
                v835 = v831;
                v9398 = v9391;
            }
            let v10503 = v9395 * v836;
            let v839 = v78 * ((v836 * v823) / v78);
            let v10507 = (((v9392 * v823) + (Lanes([v10503[0], v10503[1], 0.0]))) / v78) * v78;
            let v841 = v839 / v840;
            let v10508 = v10507 / v840;
            let v849 = v846 + (v841 * v847);
            let v851 = v845 + (v841 * v849);
            let v853 = v844 + (v841 * v851);
            let v855 = v843 + (v841 * v853);
            let v857 = v842 + (v841 * v855);
            let v859 = v1 + (v841 * v857);
            let v860 = v840 / v859;
            let v10527 = ((((v10508 * v857) + (((v10508 * v855) + (((v10508 * v853) + (((v10508 * v851) + (((v10508 * v849) + ((v10508 * v847) * v841)) * v841)) * v841)) * v841)) * v841)) * v860) * v10360) / v859;
            let v862 = if v860 < v861 { 1.0 } else { 0.0 };
            let v863: f64;
            let v9399: Lanes<3>;
            if v862 != 0.0 {
                v863 = v861;
                v9399 = v10474;
            } else {
                v863 = v860;
                v9399 = v10527;
            }
            let v864 = v835 + v863;
            let v10528 = v9398 + v9399;
            let v866 = v823 + (v78 * v863);
            let v10530 = Lanes([v9395[0], v9395[1], 0.0]);
            let v10531 = v10530 + (v9399 * v78);
            let v867 = v830 + v863;
            let v10532 = Lanes([v9397[0], v9397[1], v9397[2], 0.0]);
            let v10534 = v10532 + (Lanes([v9399[0], v9399[1], 0.0, v9399[2]]));
            let v878: f64;
            let v988: f64;
            let v9400: Lanes<3>;
            let v9401: Lanes<3>;
            if v565 != 0.0 {
                v878 = v835;
                v988 = v864;
                v9400 = v9398;
                v9401 = v10528;
            } else {
                let v868 = if v17 < v96 { 1.0 } else { 0.0 };
                let v869: f64;
                let v9402: Lanes<3>;
                if v868 != 0.0 {
                    v869 = v835;
                    v9402 = v9398;
                } else {
                    v869 = v0;
                    v9402 = v10474;
                }
                let v870: f64;
                let v9403: Lanes<3>;
                if v868 != 0.0 {
                    v870 = v864;
                    v9403 = v10528;
                } else {
                    v870 = v0;
                    v9403 = v10474;
                }
                v878 = v869;
                v988 = v870;
                v9400 = v9402;
                v9401 = v9403;
            }
            let v872 = (v78 * v490) * v123;
            let v874 = (v872 * v128) * v128;
            let v875 = v830 - v240;
            let v876 = v78 / v874;
            let v10537 = (Lanes([v9397[0], v9397[1], 0.0, v9397[2]])) - (Lanes([0.0, 0.0, v10385[0], 0.0]));
            let v10541 = ((Lanes([v10537[0], v10537[1], v10537[2], v10537[3], 0.0])) - (Lanes([v9400[0], v9400[1], 0.0, 0.0, v9400[2]]))) * v876;
            let v881 = v1 + (v876 * ((v875 - v665) - v878));
            let v10542 = v10541 * v881;
            let v885 = ((v881 * v881) + v883).sqrt();
            let v10548 = (v10541 + ((v10542 + v10542) * (v9353 / (v10405 * v885)))) * v13;
            let v889 = (v13 * (v881 + v885)) + v888;
            let v890 = if v889 < v0 { 1.0 } else { 0.0 };
            let v891: f64;
            let v9404: Lanes<5>;
            if v890 != 0.0 {
                v891 = v0;
                v9404 = v10549;
            } else {
                v891 = v889;
                v9404 = v10548;
            }
            let v893 = (v891 + v362).sqrt();
            let v10555 = Lanes([v9397[0], v9397[1], 0.0, v9397[2], 0.0]);
            let v10558 = (v10555 + (((v9404 * (v9353 / (v10405 * v893))) * v10360) * v874)) - (Lanes([0.0, 0.0, v9387[0], 0.0, 0.0]));
            let v900 = (((v875 + (v874 * (v1 - v893))) - v766) - v79) - v899;
            let v904: f64;
            if v902 != 0.0 {
                v904 = v901;
            } else {
                v904 = v903;
            }
            let v10559 = v10558 * v900;
            let v907 = ((v900 * v900) + v904).sqrt();
            let v910 = v79 + (v13 * (v900 + v907));
            let v911 = v823 / v910;
            let v10567 = Lanes([v9395[0], v9395[1], 0.0, 0.0, 0.0]);
            let v10569 = (v10567 - (((v10558 + ((v10559 + v10559) * (v9353 / (v10405 * v907)))) * v13) * v911)) / v910;
            let v912 = v911 * v911;
            let v10570 = v10569 * v911;
            let v10571 = v10570 + v10570;
            let v10575 = v10571 * v912;
            let v918 = (((v1 + v911) + v912) + (v912 * v911)) + (v912 * v912);
            let v919 = v1 / v918;
            let v920 = v1 - v919;
            let v921 = v920 * v920;
            let v10584 = (((((((v10569 + v10571) + ((v10571 * v911) + (v10569 * v912))) + (v10575 + v10575)) * v919) * v10360) / v918) * v10360) * v920;
            let v10585 = v10584 + v10584;
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
            let v9405: Lanes<4>;
            let v9406: Lanes<4>;
            let v9407: Lanes<5>;
            if v936 != 0.0 {
                let v938 = (v750 * v128) * v128;
                let v939 = v938 * v750;
                let v10628 = (((v10455 * v128) * v128) * v750) + (v10455 * v938);
                let v10629 = Lanes([0.0, 0.0, v10628[0], 0.0, 0.0]);
                v1048 = v128;
                v1128 = v127;
                v1211 = v939;
                v9405 = v10595;
                v9406 = v10595;
                v9407 = v10629;
            } else {
                let v10587 = v10532 - (Lanes([v9400[0], v9400[1], 0.0, v9400[2]]));
                let v942 = ((v830 - v878) - v934) + v927;
                let v10588 = v10587 * v942;
                let v946 = ((v942 * v942) + v944).sqrt();
                let v10594 = (v10587 + ((v10588 + v10588) * (v9353 / (v10405 * v946)))) * v13;
                let v950 = (v13 * (v942 + v946)) + v949;
                let v951 = if v950 < v0 { 1.0 } else { 0.0 };
                let v952: f64;
                let v9408: Lanes<4>;
                if v951 != 0.0 {
                    v952 = v0;
                    v9408 = v10595;
                } else {
                    v952 = v950;
                    v9408 = v10594;
                }
                let v953 = v1 / v952;
                let v10598 = ((v9408 * v953) * v10360) / v952;
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
                let v10599 = v10598 * v10360;
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
                let v10600 = v10599 * v962;
                let v970 = ((v962 * v962) + v967).sqrt();
                let v10608 = (((v10599 + ((v10600 + v10600) * (v9353 / (v10405 * v970)))) * v13) * v10360) * v922;
                let v975 = (v922 * (v960 - (v13 * (v962 + v970)))) + v924;
                let v978 = if (v975 * v976) < v122 { 1.0 } else { 0.0 };
                let v979: f64;
                let v9409: Lanes<4>;
                if v978 != 0.0 {
                    v979 = v0;
                    v9409 = v10595;
                } else {
                    v979 = v975;
                    v9409 = v10608;
                }
                let v980 = v122 + v979;
                let v981 = v126 / v980;
                let v10611 = ((v9409 * v981) * v10360) / v980;
                let v982 = v980 / v126;
                let v10612 = v9409 / v126;
                let v983 = v750 * v750;
                let v10613 = v10455 * v750;
                let v984 = v983 * v982;
                let v10615 = (v10613 + v10613) * v982;
                let v10616 = v10612 * v983;
                let v985 = v984 * v982;
                let v10621 = v10612 * v984;
                let v10623 = (((Lanes([0.0, 0.0, v10615[0], 0.0, 0.0])) + (Lanes([v10616[0], v10616[1], 0.0, v10616[2], v10616[3]]))) * v982) + (Lanes([v10621[0], v10621[1], 0.0, v10621[2], v10621[3]]));
                v1048 = v982;
                v1128 = v981;
                v1211 = v985;
                v9405 = v10612;
                v9406 = v10611;
                v9407 = v10623;
            }
            let v986 = if v17 < v96 { 1.0 } else { 0.0 };
            let v987 = if v565 != 0.0 || v986 != 0.0 { 1.0 } else { 0.0 };
            let v1037: f64;
            let v9410: Lanes<4>;
            if v987 != 0.0 {
                let v10631 = v9401 * v10360;
                let v990 = (v13 - v988) - v529;
                let v994: f64;
                if v992 != 0.0 {
                    v994 = v991;
                } else {
                    v994 = v993;
                }
                let v10632 = v10631 * v990;
                let v997 = ((v990 * v990) + v994).sqrt();
                let v10639 = ((v10631 + ((v10632 + v10632) * (v9353 / (v10405 * v997)))) * v13) * v10360;
                let v1007 = (((((-v12) * v12) * v490) / v1004) + v766) - v665;
                let v10640 = v9387 - v10385;
                let v10642 = Lanes([0.0, 0.0, v10640[0], 0.0]);
                let v10643 = (Lanes([v10639[0], v10639[1], 0.0, v10639[2]])) - v10642;
                let v1009 = ((v13 - (v13 * (v990 + v997))) - v1007) - v529;
                let v1011 = (v90 * v1007) * v529;
                let v10645 = (v10640 * v90) * v529;
                let v1012 = if v1011 > v0 { 1.0 } else { 0.0 };
                let v1014: f64;
                let v9411: Lanes<1>;
                if v1012 != 0.0 {
                    v1014 = v1011;
                    v9411 = v10645;
                } else {
                    let v1013 = -v1011;
                    let v10646 = v10645 * v10360;
                    v1014 = v1013;
                    v9411 = v10646;
                }
                let v10647 = v10643 * v1009;
                let v1017 = ((v1009 * v1009) + v1014).sqrt();
                let v1020 = v1007 + (v13 * (v1009 + v1017));
                let v10656 = v10642 + ((v10643 + (((v10647 + v10647) + (Lanes([0.0, 0.0, v9411[0], 0.0]))) * (v9353 / (v10405 * v1017)))) * v13);
                let v1021 = if v17 > v78 { 1.0 } else { 0.0 };
                let v1038: f64;
                let v9412: Lanes<4>;
                if v1021 != 0.0 {
                    let v10657 = v10656 * v10360;
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
                    let v10658 = v10657 * v1023;
                    let v1031 = ((v1023 * v1023) + v1028).sqrt();
                    let v1034 = v508 - (v13 * (v1023 + v1031));
                    let v10665 = ((v10657 + ((v10658 + v10658) * (v9353 / (v10405 * v1031)))) * v13) * v10360;
                    v1038 = v1034;
                    v9412 = v10665;
                } else {
                    v1038 = v1020;
                    v9412 = v10656;
                }
                v1037 = v1038;
                v9410 = v9412;
            } else {
                v1037 = v0;
                v9410 = v10630;
            }
            let v1083: f64;
            let v9413: Lanes<4>;
            if v986 != 0.0 {
                v1083 = v12;
                v9413 = v10630;
            } else {
                let v1036 = v1035 / v490;
                let v1041 = (v1036 * (v508 - v1037)).sqrt();
                let v10670 = ((v9410 * v10360) * v1036) * (v9353 / (v10405 * v1041));
                v1083 = v1041;
                v9413 = v10670;
            }
            let v1047: f64;
            let v9414: Lanes<4>;
            if v986 != 0.0 {
                let v1043 = (v492 * v508).sqrt();
                v1047 = v1043;
                v9414 = v10630;
            } else {
                let v1046 = (v492 * (v508 - v1037)).sqrt();
                let v10675 = ((v9410 * v10360) * v492) * (v9353 / (v10405 * v1046));
                v1047 = v1046;
                v9414 = v10675;
            }
            let v10676 = v9414 * v1048;
            let v10677 = v9405 * v1047;
            let v1051 = (v932 + (v1047 * v1048)) + v704;
            let v10682 = ((Lanes([v10676[0], v10676[1], v10676[2], 0.0, v10676[3]])) + (Lanes([v10677[0], v10677[1], 0.0, v10677[2], v10677[3]]))) + (Lanes([0.0, 0.0, v10392[0], 0.0, 0.0]));
            let v1053 = v1052 * v508;
            let v10683 = v9410 * v10360;
            let v1055 = (v1053 - v1037) - v529;
            let v10684 = v10683 * v1055;
            let v1061 = ((v1055 * v1055) + ((v1057 * v508) * v529)).sqrt();
            let v1065 = v508 - (v1053 - (v13 * (v1055 + v1061)));
            let v10692 = (((v10683 + ((v10684 + v10684) * (v9353 / (v10405 * v1061)))) * v13) * v10360) * v10360;
            let v1066 = v1065.sqrt();
            let v10695 = v10692 * (v9353 / (v10405 * v1066));
            let v1067 = if v187 != v0 { 1.0 } else { 0.0 };
            let v1137: f64;
            let v9415: Lanes<5>;
            if v1067 != 0.0 {
                let v1070 = (v1068 * v477) * v123;
                let v1076: f64;
                let v9416: Lanes<4>;
                if v986 != 0.0 {
                    let v1072 = (v1070 * v513).sqrt();
                    v1076 = v1072;
                    v9416 = v10630;
                } else {
                    let v1075 = (v1070 * (v513 - v1037)).sqrt();
                    let v10699 = (v10683 * v1070) * (v9353 / (v10405 * v1075));
                    v1076 = v1075;
                    v9416 = v10699;
                }
                let v10700 = v9416 * v1048;
                let v10701 = v9405 * v1076;
                let v1080 = v123 * v1048;
                let v1082 = v1 / (v187 * v187);
                let v1085 = (v78 * v1083) * v1082;
                let v10708 = (v9405 * v123) * v1085;
                let v10709 = ((v9413 * v78) * v1082) * v1080;
                let v1088 = v1087 - v508;
                let v1089 = (v1080 * v1085) * v1088;
                let v1090 = v1051 - ((v513 + v240) + (v1076 * v1048));
                let v1091 = v59 / v187;
                let v10716 = v10531 * v57;
                let v1095 = (v54 + (v1091 * v1065)) + (v57 * v866);
                let v1096 = v1090 * v1089;
                let v1097 = v1096 * v1095;
                let v10723 = ((v10692 * v1091) + (Lanes([v10716[0], v10716[1], 0.0, v10716[2]]))) * v1096;
                let v10725 = ((((v10682 - ((Lanes([v10700[0], v10700[1], v10700[2], 0.0, v10700[3]])) + (Lanes([v10701[0], v10701[1], 0.0, v10701[2], v10701[3]])))) * v1089) + ((((Lanes([v10708[0], v10708[1], 0.0, v10708[2], v10708[3]])) + (Lanes([v10709[0], v10709[1], v10709[2], 0.0, v10709[3]]))) * v1088) * v1090)) * v1095) + (Lanes([v10723[0], v10723[1], v10723[2], 0.0, v10723[3]]));
                v1137 = v1097;
                v9415 = v10725;
            } else {
                v1137 = v0;
                v9415 = v10549;
            }
            let v1099 = (v123 * v1083) * v78;
            let v10728 = v9405 * v1099;
            let v10729 = ((v9413 * v123) * v78) * v1048;
            let v1101 = v1087 - v508;
            let v1103 = v142 - v1102;
            let v1105 = v1 / (v1103 * v1103);
            let v1107 = ((v1048 * v1099) * v1101) * v1105;
            let v1108 = v53 / v142;
            let v10736 = v10531 * v51;
            let v1112 = (v48 + (v1108 * v1065)) + (v51 * v866);
            let v1113 = v1107 * v1112;
            let v10740 = ((v10692 * v1108) + (Lanes([v10736[0], v10736[1], 0.0, v10736[2]]))) * v1107;
            let v10742 = (((((Lanes([v10728[0], v10728[1], 0.0, v10728[2], v10728[3]])) + (Lanes([v10729[0], v10729[1], v10729[2], 0.0, v10729[3]]))) * v1101) * v1105) * v1112) + (Lanes([v10740[0], v10740[1], v10740[2], 0.0, v10740[3]]));
            let v1115 = if v1114 > v0 { 1.0 } else { 0.0 };
            let v1140: f64;
            let v9417: Lanes<4>;
            if v1115 != 0.0 {
                let v10743 = v10376 + v9387;
                let v10744 = v10531 * v1120;
                let v1126 = (v1114 * v12) / ((v142 * v13) + v47);
                let v1127 = (((v661 + v766) - (v78 * v1117)) + (v1120 * v866)) * v1126;
                let v10748 = ((Lanes([0.0, 0.0, v10743[0], 0.0])) + (Lanes([v10744[0], v10744[1], 0.0, v10744[2]]))) * v1126;
                v1140 = v1127;
                v9417 = v10748;
            } else {
                v1140 = v0;
                v9417 = v10630;
            }
            let v1130 = v1128 + (v45 / v165);
            let v1131 = v1 / v1130;
            let v1132 = v1048 - v1131;
            let v10753 = v9414 * v1132;
            let v10754 = (v9405 - (((v9406 * v1131) * v10360) / v1130)) * v1047;
            let v1138 = v1113 + v1137;
            let v10758 = v10742 + v9415;
            let v10761 = (v10758 + ((Lanes([v10753[0], v10753[1], v10753[2], 0.0, v10753[3]])) + (Lanes([v10754[0], v10754[1], 0.0, v10754[2], v10754[3]])))) + (Lanes([v9417[0], v9417[1], v9417[2], 0.0, v9417[3]]));
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
            let v9418: Lanes<4>;
            if v1146 != 0.0 {
                v1199 = v0;
                v9418 = v10595;
            } else {
                let v1148 = v867 - v1147;
                let v1150 = if v1148 < v1149 { 1.0 } else { 0.0 };
                let v1172: f64;
                let v9419: Lanes<4>;
                if v1150 != 0.0 {
                    v1172 = v0;
                    v9419 = v10595;
                } else {
                    let v1151 = if v1148 < v0 { 1.0 } else { 0.0 };
                    let v1173: f64;
                    let v9420: Lanes<4>;
                    if v1151 != 0.0 {
                        let v1156 = v1152 + (v1148 * v1154);
                        let v1158 = v1 + (v1148 * v1156);
                        let v10778 = (v10534 * v1158) + (((v10534 * v1156) + ((v10534 * v1154) * v1148)) * v1148);
                        let v1160 = v1 + (v1148 * v1158);
                        v1173 = v1160;
                        v9420 = v10778;
                    } else {
                        let v1165 = v1162 + (v1148 * v1163);
                        let v1167 = v1161 + (v1148 * v1165);
                        let v1169 = v1 + (v1148 * v1167);
                        let v10771 = (v10534 * v1169) + (((v10534 * v1167) + (((v10534 * v1165) + ((v10534 * v1163) * v1148)) * v1148)) * v1148);
                        let v1171 = v1 + (v1148 * v1169);
                        v1173 = v1171;
                        v9420 = v10771;
                    }
                    v1172 = v1173;
                    v9419 = v9420;
                }
                let v1174 = v1172 - v1;
                let v10779 = v9419 * v1174;
                let v1178 = ((v1174 * v1174) + v1176).sqrt();
                let v10785 = (v9419 + ((v10779 + v10779) * (v9353 / (v10405 * v1178)))) * v13;
                let v1182 = (v13 * (v1174 + v1178)) + v1181;
                let v1183 = if v1182 < v0 { 1.0 } else { 0.0 };
                let v1184: f64;
                let v9421: Lanes<4>;
                if v1183 != 0.0 {
                    v1184 = v0;
                    v9421 = v10595;
                } else {
                    v1184 = v1182;
                    v9421 = v10785;
                }
                let v10787 = (v9421 * v239) * v10360;
                let v1187 = (v1 - (v1184 * v239)) - v899;
                let v1191: f64;
                if v1189 != 0.0 {
                    v1191 = v1188;
                } else {
                    v1191 = v1190;
                }
                let v10788 = v10787 * v1187;
                let v1194 = ((v1187 * v1187) + v1191).sqrt();
                let v1197 = v1 - (v13 * (v1187 + v1194));
                let v10795 = ((v10787 + ((v10788 + v10788) * (v9353 / (v10405 * v1194)))) * v13) * v10360;
                v1199 = v1197;
                v9418 = v10795;
            }
            let v1200 = (v875 + v1142) - v1199;
            let v10797 = Lanes([v9418[0], v9418[1], 0.0, v9418[2], v9418[3]]);
            let v10798 = (v10555 + v10761) - v10797;
            let v1202 = (v477 / v33).ln();
            let v1203 = v665 * v1202;
            let v10799 = v10385 * v1202;
            let v1205 = (v240 - v1142) + v1199;
            let v1206 = v750 * v1048;
            let v10800 = v10455 * v1048;
            let v10801 = v9405 * v750;
            let v10804 = (Lanes([0.0, 0.0, v10800[0], 0.0, 0.0])) + (Lanes([v10801[0], v10801[1], 0.0, v10801[2], v10801[3]]));
            let v1207 = v1206 * v1206;
            let v10805 = v10804 * v1206;
            let v10806 = v10805 + v10805;
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
            let v9422: Lanes<6>;
            let v9423: Lanes<6>;
            let v9424: Lanes<6>;
            let v9425: Lanes<6>;
            let v9426: Lanes<6>;
            let v9427: Lanes<6>;
            let v9428: Lanes<6>;
            let v9429: Lanes<6>;
            let v9430: Lanes<6>;
            let v9431: Lanes<6>;
            let v9432: Lanes<6>;
            let v9433: Lanes<6>;
            let v9434: Lanes<6>;
            let v9435: Lanes<1>;
            let v9436: Lanes<1>;
            let v9437: Lanes<6>;
            let v9438: Lanes<5>;
            let v9439: Lanes<4>;
            let v9440: Lanes<5>;
            let v9441: Lanes<5>;
            let v9442: Lanes<6>;
            let v9443: Lanes<5>;
            let v9444: Lanes<6>;
            let v9445: Lanes<6>;
            let v9446: Lanes<6>;
            let v9447: Lanes<6>;
            let v9448: Lanes<6>;
            let v9449: Lanes<6>;
            let v9450: Lanes<6>;
            let v9451: Lanes<6>;
            let v9452: Lanes<6>;
            if v9 != 0.0 {
                let v1209 = v766 + v1;
                let v1210 = v1 / v759;
                let v11895 = ((v10466 * v1210) * v10360) / v759;
                let v1212 = v1210 / v1211;
                let v11899 = ((Lanes([0.0, 0.0, v11895[0], 0.0, 0.0])) - (v9407 * v1212)) / v1211;
                let v1213 = v1212 * v1209;
                let v11901 = v9387 * v1212;
                let v1214 = v1213 * v1209;
                let v11905 = v9387 * v1213;
                let v1215 = v78 / v1209;
                let v1216 = v663 + v1215;
                let v1218 = (v1214.ln()) / v1216;
                let v11914 = (v10380 + (((v9387 * v1215) * v10360) / v1209)) * v1218;
                let v1220 = (v764 * v1218).sqrt();
                let v11921 = ((((((((v11899 * v1209) + (Lanes([0.0, 0.0, v11901[0], 0.0, 0.0]))) * v1209) + (Lanes([0.0, 0.0, v11905[0], 0.0, 0.0]))) * (v9353 / v1214)) - (Lanes([0.0, 0.0, v11914[0], 0.0, 0.0]))) / v1216) * v764) * (v9353 / (v10405 * v1220));
                let v1221 = if v1220 > v12 { 1.0 } else { 0.0 };
                let v1222: f64;
                let v9453: Lanes<5>;
                if v1221 != 0.0 {
                    v1222 = v12;
                    v9453 = v10549;
                } else {
                    v1222 = v1220;
                    v9453 = v11921;
                }
                let v1224 = v1223 * v477;
                let v1225 = v1224 * v1222;
                let v11922 = v9453 * v1224;
                let v1228 = (v1226 * v477) * v12;
                let v1229 = -v1228;
                let v1230 = v1229 * v529;
                let v1232 = v1229 * v1231;
                let v1244: f64;
                let v9454: Lanes<4>;
                if v1233 != 0.0 {
                    let v1234 = v864 + v1203;
                    let v11928 = (Lanes([v10528[0], v10528[1], 0.0, v10528[2]])) + (Lanes([0.0, 0.0, v10799[0], 0.0]));
                    v1244 = v1234;
                    v9454 = v11928;
                } else {
                    let v1235 = v835 + v1203;
                    let v11925 = (Lanes([v9398[0], v9398[1], 0.0, v9398[2]])) + (Lanes([0.0, 0.0, v10799[0], 0.0]));
                    v1244 = v1235;
                    v9454 = v11925;
                }
                let v1239 = (v78 / v663) * ((v33 / v731).ln());
                let v11929 = v9389 * v1240;
                let v1243 = ((v1240 * v1240) * v132) * v132;
                let v11932 = ((v11929 + v11929) * v132) * v132;
                let v1245 = -v1244;
                let v11933 = v9454 * v10360;
                let v1247 = v1243 * v663;
                let v11937 = (v11932 * v663) + (v10380 * v1243);
                let v1248 = (v78 * v1245) + v1247;
                let v11939 = (v11933 * v78) + (Lanes([0.0, 0.0, v11937[0], 0.0]));
                let v1250 = v1245 * v1245;
                let v11940 = v11933 * v1245;
                let v11941 = v11940 + v11940;
                let v11944 = (v11941 + (Lanes([0.0, 0.0, v11932[0], 0.0]))) * v90;
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
                let v11945 = v11932 * v1261;
                let v11948 = (v11941 - (Lanes([0.0, 0.0, v11945[0], 0.0]))) / v1243;
                let v1263 = v1261 / v1262;
                let v11949 = v9390 * v1263;
                let v11950 = Lanes([0.0, 0.0, v11949[0], 0.0]);
                let v11951 = v9353 / v1263;
                let v1265 = v78 / v1245;
                let v1266 = v663 + v1265;
                let v1267 = (v1263.ln()) / v1266;
                let v11957 = ((Lanes([0.0, 0.0, v10380[0], 0.0])) + (((v11933 * v1265) * v10360) / v1245)) * v1267;
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
                let v9455: Lanes<5>;
                if v1392 != 0.0 {
                    let v1393 = v1 / v1128;
                    let v11960 = ((v9406 * v1393) * v10360) / v1128;
                    let v1394 = v12 / v123;
                    let v1395 = v1 / v130;
                    let v1397 = (v1393 + v1394) + v1395;
                    let v1398 = v1 / v1397;
                    let v11961 = v11960 * v1398;
                    let v11963 = (v11961 * v10360) / v1397;
                    let v1400 = v1 - (v1398 * v1393);
                    let v1404 = v1245 + ((v1395 + (v13 * v1394)) * v1229);
                    let v1405 = v1398 * v1404;
                    let v11967 = v11963 * v1404;
                    let v11968 = v11933 * v1398;
                    let v11972 = v11960 * v1405;
                    let v1407 = (v1393 * v1405) / v1400;
                    let v11976 = (((v11963 * v1393) + v11961) * v10360) * v1407;
                    let v11979 = (((Lanes([v11972[0], v11972[1], 0.0, v11972[2], v11972[3]])) + (((Lanes([v11967[0], v11967[1], 0.0, v11967[2], v11967[3]])) + (Lanes([v11968[0], v11968[1], v11968[2], 0.0, v11968[3]]))) * v1393)) - (Lanes([v11976[0], v11976[1], 0.0, v11976[2], v11976[3]]))) / v1400;
                    let v1408 = v1205 + v1407;
                    v1574 = v1407;
                    v1888 = v1408;
                    v9455 = v11979;
                } else {
                    v1574 = v0;
                    v1888 = v1205;
                    v9455 = v10549;
                }
                let v1409 = v839 / v79;
                let v11980 = v10507 / v79;
                let v1417 = v1414 + (v1409 * v1415);
                let v1419 = v1413 + (v1409 * v1417);
                let v1421 = v1412 + (v1409 * v1419);
                let v1423 = v1411 + (v1409 * v1421);
                let v1425 = v1410 + (v1409 * v1423);
                let v1427 = v1 + (v1409 * v1425);
                let v1428 = v79 / v1427;
                let v11999 = ((((v11980 * v1425) + (((v11980 * v1423) + (((v11980 * v1421) + (((v11980 * v1419) + (((v11980 * v1417) + ((v11980 * v1415) * v1409)) * v1409)) * v1409)) * v1409)) * v1409)) * v1428) * v10360) / v1427;
                let v1429 = if v1428 < v861 { 1.0 } else { 0.0 };
                let v1430: f64;
                let v9456: Lanes<3>;
                if v1429 != 0.0 {
                    v1430 = v861;
                    v9456 = v10474;
                } else {
                    v1430 = v1428;
                    v9456 = v11999;
                }
                let v12001 = v10532 + (Lanes([v9456[0], v9456[1], 0.0, v9456[2]]));
                let v1434 = (((v830 + v1430) - v240) + v1142) - v1199;
                let v1435 = v721 * v766;
                let v1436 = v1222 / v1435;
                let v12006 = (v9387 * v721) * v1436;
                let v1437 = v1436 * v1434;
                let v12012 = (((v9453 - (Lanes([0.0, 0.0, v12006[0], 0.0, 0.0]))) / v1435) * v1434) + ((((Lanes([v12001[0], v12001[1], 0.0, v12001[2], v12001[3]])) + v10761) - v10797) * v1436);
                let v1438 = v12 * v1208;
                let v1441 = if (if v1437 < v1438 { 1.0 } else { 0.0 }) != 0.0 && (if v1438 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1469: f64;
                let v9457: Lanes<5>;
                if v1441 != 0.0 {
                    let v1442 = v1438 - v1437;
                    let v12013 = v12012 * v10360;
                    let v1443 = v1442 * v1442;
                    let v12014 = v12013 * v1442;
                    let v1444 = v1438 * v1438;
                    let v12016 = (v12014 + v12014) * v1443;
                    let v12017 = v12016 + v12016;
                    let v1447 = (v1443 * v1443) + (v1444 * v1444);
                    let v1464: f64;
                    let v9458: Lanes<5>;
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
                        let mut v9459: Lanes<5> = Lanes([0.0; 5]);
                        v1453 = v0;
                        v1455 = v1447;
                        v9459 = v12017;
                        loop {
                            let v1454 = if v1453 < v1458 { 1.0 } else { 0.0 };
                            if v1454 == 0.0 {
                                break;
                            }
                            let v1456 = v1455.sqrt();
                            let v19238 = v9459 * (v9353 / (v10405 * v1456));
                            let v1457 = v1453 + v1;
                            v1453 = v1457;
                            v1455 = v1456;
                            v9459 = v19238;
                        }
                        v1464 = v1455;
                        v9458 = v9459;
                    } else {
                        let v1463 = v1447.powf(v1462);
                        let v12021 = v12017 * (v1462 * (v1447.powf(v12018)));
                        v1464 = v1463;
                        v9458 = v12021;
                    }
                    let v1465 = v1 / v1464;
                    let v1466 = v1442 * v1438;
                    let v1468 = v1438 - (v1466 * v1465);
                    let v12029 = (((v12013 * v1438) * v1465) + ((((v9458 * v1465) * v10360) / v1464) * v1466)) * v10360;
                    v1469 = v1468;
                    v9457 = v12029;
                } else {
                    v1469 = v1437;
                    v9457 = v12012;
                }
                let v1470 = v1222 - v12;
                let v1473 = if (if v1469 > v1470 { 1.0 } else { 0.0 }) != 0.0 && (if v12 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1502: f64;
                let v9460: Lanes<5>;
                if v1473 != 0.0 {
                    let v12030 = v9457 - v9453;
                    let v1475 = (v1469 - v1222) + v12;
                    let v1476 = v1475 * v1475;
                    let v12031 = v12030 * v1475;
                    let v1477 = v12 * v12;
                    let v12033 = (v12031 + v12031) * v1476;
                    let v12034 = v12033 + v12033;
                    let v1480 = (v1476 * v1476) + (v1477 * v1477);
                    let v1497: f64;
                    let v9461: Lanes<5>;
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
                        let mut v9462: Lanes<5> = Lanes([0.0; 5]);
                        v1486 = v0;
                        v1488 = v1480;
                        v9462 = v12034;
                        loop {
                            let v1487 = if v1486 < v1491 { 1.0 } else { 0.0 };
                            if v1487 == 0.0 {
                                break;
                            }
                            let v1489 = v1488.sqrt();
                            let v19235 = v9462 * (v9353 / (v10405 * v1489));
                            let v1490 = v1486 + v1;
                            v1486 = v1490;
                            v1488 = v1489;
                            v9462 = v19235;
                        }
                        v1497 = v1488;
                        v9461 = v9462;
                    } else {
                        let v1496 = v1480.powf(v1495);
                        let v12038 = v12034 * (v1495 * (v1480.powf(v12035)));
                        v1497 = v1496;
                        v9461 = v12038;
                    }
                    let v1498 = v1 / v1497;
                    let v1499 = v1475 * v12;
                    let v1501 = v1470 + (v1499 * v1498);
                    let v12046 = v9453 + (((v12030 * v12) * v1498) + ((((v9461 * v1498) * v10360) / v1497) * v1499));
                    v1502 = v1501;
                    v9460 = v12046;
                } else {
                    v1502 = v1469;
                    v9460 = v9457;
                }
                let v1504 = (-v1502) * v490;
                let v12048 = (v9460 * v10360) * v490;
                let v1512 = ((((v1229 * v12) / v78) / v123) + v665) - ((v1509 * v12) / v123);
                let v2258: f64;
                let v2259: f64;
                let v2260: f64;
                let v2585: f64;
                let v2600: f64;
                let v2678: f64;
                let v3331: f64;
                let v5094: f64;
                let v9463: Lanes<5>;
                let v9464: Lanes<5>;
                let v9465: Lanes<5>;
                let v9466: Lanes<5>;
                let v9467: Lanes<5>;
                let v9468: Lanes<5>;
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
                    v9463 = v10549;
                    v9464 = v10549;
                    v9465 = v10549;
                    v9466 = v10549;
                    v9467 = v10549;
                    v9468 = v10549;
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
                    let v9469: Lanes<5>;
                    let v9470: Lanes<5>;
                    if v1622 != 0.0 {
                        let v1623 = v1 / v1128;
                        let v1624 = v12 / v123;
                        let v1625 = v1 / v130;
                        let v1627 = (v1623 + v1624) + v1625;
                        let v1628 = v1 / v1627;
                        let v1631 = v1625 + (v13 * v1624);
                        let v1634 = (v1200 - v1244) + (v1631 * (-v1504));
                        let v12118 = ((((((v9406 * v1623) * v10360) / v1128) * v1628) * v10360) / v1627) * v1634;
                        let v1636 = (v1628 * v1634) / v1128;
                        let v12122 = v9406 * v1636;
                        let v1637 = v1200 - v1636;
                        let v12126 = v10798 - ((((Lanes([v12118[0], v12118[1], 0.0, v12118[2], v12118[3]])) + (((v10798 - (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3]]))) + ((v12048 * v10360) * v1631)) * v1628)) - (Lanes([v12122[0], v12122[1], 0.0, v12122[2], v12122[3]]))) / v1128);
                        v1694 = v1637;
                        v1697 = v1637;
                        v9469 = v12126;
                        v9470 = v12126;
                    } else {
                        let v1638 = v1 / v1128;
                        let v1639 = v12 / v123;
                        let v1640 = v1 / v130;
                        let v1642 = (v1638 + v1639) + v1640;
                        let v1643 = v1 / v1642;
                        let v1646 = v1640 + (v13 * v1639);
                        let v1649 = (v1200 - v1244) + (v1646 * (-v1504));
                        let v12060 = ((((((v9406 * v1638) * v10360) / v1128) * v1643) * v10360) / v1642) * v1649;
                        let v1651 = (v1643 * v1649) / v1128;
                        let v12064 = v9406 * v1651;
                        let v1652 = v1200 - v1651;
                        let v12068 = v10798 - ((((Lanes([v12060[0], v12060[1], 0.0, v12060[2], v12060[3]])) + (((v10798 - (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3]]))) + ((v12048 * v10360) * v1646)) * v1643)) - (Lanes([v12064[0], v12064[1], 0.0, v12064[2], v12064[3]]))) / v1128);
                        let v1653 = v1200 - v1574;
                        let v12069 = v10798 - v9455;
                        let v1654 = if v1653 > v0 { 1.0 } else { 0.0 };
                        let v1695: f64;
                        let v9471: Lanes<5>;
                        if v1654 != 0.0 {
                            let v1655 = v1212 * v1653;
                            let v1656 = v1655 * v1653;
                            let v1657 = v78 / v1653;
                            let v1658 = v663 + v1657;
                            let v1660 = (v1656.ln()) / v1658;
                            let v1662 = v1660 * v1661;
                            let v12086 = (((((((v11899 * v1653) + (v12069 * v1212)) * v1653) + (v12069 * v1655)) * (v9353 / v1656)) - (((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + (((v12069 * v1657) * v10360) / v1653)) * v1660)) / v1658) * v1661;
                            let v1663 = v1662 - v708;
                            let v1666 = if (if v1652 > v1663 { 1.0 } else { 0.0 }) != 0.0 && v1665 != 0.0 { 1.0 } else { 0.0 };
                            let v1696: f64;
                            let v9472: Lanes<5>;
                            if v1666 != 0.0 {
                                let v12087 = v12068 - v12086;
                                let v1668 = (v1652 - v1662) + v708;
                                let v1669 = v1668 * v1668;
                                let v12088 = v12087 * v1668;
                                let v12090 = (v12088 + v12088) * v1669;
                                let v12091 = v12090 + v12090;
                                let v1672 = (v1669 * v1669) + v1671;
                                let v1689: f64;
                                let v9473: Lanes<5>;
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
                                    let mut v9474: Lanes<5> = Lanes([0.0; 5]);
                                    v1678 = v0;
                                    v1680 = v1672;
                                    v9474 = v12091;
                                    loop {
                                        let v1679 = if v1678 < v1683 { 1.0 } else { 0.0 };
                                        if v1679 == 0.0 {
                                            break;
                                        }
                                        let v1681 = v1680.sqrt();
                                        let v12106 = v9474 * (v9353 / (v10405 * v1681));
                                        let v1682 = v1678 + v1;
                                        v1678 = v1682;
                                        v1680 = v1681;
                                        v9474 = v12106;
                                    }
                                    v1689 = v1680;
                                    v9473 = v9474;
                                } else {
                                    let v1688 = v1672.powf(v1687);
                                    let v12095 = v12091 * (v1687 * (v1672.powf(v12092)));
                                    v1689 = v1688;
                                    v9473 = v12095;
                                }
                                let v1690 = v1 / v1689;
                                let v1691 = v1668 * v708;
                                let v1693 = v1663 + (v1691 * v1690);
                                let v12103 = v12086 + (((v12087 * v708) * v1690) + ((((v9473 * v1690) * v10360) / v1689) * v1691));
                                v1696 = v1693;
                                v9472 = v12103;
                            } else {
                                v1696 = v1652;
                                v9472 = v12068;
                            }
                            v1695 = v1696;
                            v9471 = v9472;
                        } else {
                            v1695 = v1652;
                            v9471 = v12068;
                        }
                        v1694 = v1695;
                        v1697 = v1652;
                        v9469 = v9471;
                        v9470 = v12068;
                    }
                    let v1698 = v13 * v1228;
                    let v1701 = (v1694 + (v1698 * v125)) - v1244;
                    let v12127 = Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3]]);
                    let v12128 = v9469 - v12127;
                    let v1702 = if v1701 < v0 { 1.0 } else { 0.0 };
                    let v1879: f64;
                    let v9475: Lanes<5>;
                    if v1702 != 0.0 {
                        let v1703 = v1240 * v132;
                        let v1704 = v1703 * v1703;
                        let v12179 = (v9389 * v132) * v1703;
                        let v12180 = v12179 + v12179;
                        let v12181 = v12128 * v1705;
                        let v1708 = (v1705 * v1701) + v1707;
                        let v1710 = v1708 * v529;
                        let v12182 = v12181 * v529;
                        let v1711 = (v1708 - v13) - v1710;
                        let v12183 = v12181 - v12182;
                        let v1712 = v90 * v1708;
                        let v1713 = v1712 * v1710;
                        let v12187 = ((v12181 * v90) * v1710) + (v12182 * v1712);
                        let v1714 = if v1713 > v0 { 1.0 } else { 0.0 };
                        let v1716: f64;
                        let v9476: Lanes<5>;
                        if v1714 != 0.0 {
                            v1716 = v1713;
                            v9476 = v12187;
                        } else {
                            let v1715 = -v1713;
                            let v12188 = v12187 * v10360;
                            v1716 = v1715;
                            v9476 = v12188;
                        }
                        let v12189 = v12183 * v1711;
                        let v1719 = ((v1711 * v1711) + v1716).sqrt();
                        let v1722 = v1708 - (v13 * (v1711 + v1719));
                        let v1723 = v1704 * v1722;
                        let v12198 = v12180 * v1722;
                        let v1724 = v1723 * v664;
                        let v12203 = v10382 * v1723;
                        let v12205 = (((Lanes([0.0, 0.0, v12198[0], 0.0, 0.0])) + ((v12181 - ((v12183 + (((v12189 + v12189) + v9476) * (v9353 / (v10405 * v1719)))) * v13)) * v1704)) * v664) + (Lanes([0.0, 0.0, v12203[0], 0.0, 0.0]));
                        let v1725 = v1724.sqrt();
                        let v1726 = v1 - v1725;
                        let v1728 = v1 - v1724;
                        let v1729 = (v1701 * v1726) / v1728;
                        let v12216 = (((v12128 * v1726) + (((v12205 * (v9353 / (v10405 * v1725))) * v10360) * v1701)) - ((v12205 * v10360) * v1729)) / v1728;
                        v1879 = v1729;
                        v9475 = v12216;
                    } else {
                        let v1735 = -((v1244 - v1694) - (((v1228 / v78) * v12) / v123));
                        let v12130 = (v12127 - v9469) * v10360;
                        let v1737 = (v78 * v1735) + v1247;
                        let v12133 = (v12130 * v78) + (Lanes([0.0, 0.0, v11937[0], 0.0, 0.0]));
                        let v12134 = v12133 * v1737;
                        let v1739 = v1735 * v1735;
                        let v12136 = v12130 * v1735;
                        let v12137 = v12136 + v12136;
                        let v1742 = (v1737 * v1737) - (v90 * (v1739 + v1243));
                        let v12141 = (v12134 + v12134) - ((v12137 + (Lanes([0.0, 0.0, v11932[0], 0.0, 0.0]))) * v90);
                        let v1744 = if v1742 >= v1743 { 1.0 } else { 0.0 };
                        let v1746: f64;
                        let v9477: Lanes<5>;
                        if v1744 != 0.0 {
                            v1746 = v1742;
                            v9477 = v12141;
                        } else {
                            v1746 = v1745;
                            v9477 = v10549;
                        }
                        let v1747 = v1746.sqrt();
                        let v1749 = (v1737 - v1747) / v78;
                        let v12146 = (v12133 - (v9477 * (v9353 / (v10405 * v1747)))) / v78;
                        let v1750 = v1739 / v1243;
                        let v12147 = v11932 * v1750;
                        let v1751 = v1750 / v1262;
                        let v12151 = v9390 * v1751;
                        let v1753 = v78 / v1735;
                        let v1754 = v663 + v1753;
                        let v1755 = (v1751.ln()) / v1754;
                        let v12164 = ((((((v12137 - (Lanes([0.0, 0.0, v12147[0], 0.0, 0.0]))) / v1243) - (Lanes([0.0, 0.0, v12151[0], 0.0, 0.0]))) / v1262) * (v9353 / v1751)) - (((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + (((v12130 * v1753) * v10360) / v1735)) * v1755)) / v1754;
                        let v1756 = if v1749 < v1239 { 1.0 } else { 0.0 };
                        let v1880: f64;
                        let v9478: Lanes<5>;
                        if v1756 != 0.0 {
                            v1880 = v1749;
                            v9478 = v12146;
                        } else {
                            let v12165 = v12164 - v12146;
                            let v1758 = (v1755 - v1749) - v1270;
                            let v1760 = (v90 * v1755) * v1270;
                            let v12167 = (v12164 * v90) * v1270;
                            let v1761 = if v1760 > v0 { 1.0 } else { 0.0 };
                            let v1763: f64;
                            let v9479: Lanes<5>;
                            if v1761 != 0.0 {
                                v1763 = v1760;
                                v9479 = v12167;
                            } else {
                                let v1762 = -v1760;
                                let v12168 = v12167 * v10360;
                                v1763 = v1762;
                                v9479 = v12168;
                            }
                            let v12169 = v12165 * v1758;
                            let v1766 = ((v1758 * v1758) + v1763).sqrt();
                            let v1769 = v1755 - (v13 * (v1758 + v1766));
                            let v12177 = v12164 - ((v12165 + (((v12169 + v12169) + v9479) * (v9353 / (v10405 * v1766)))) * v13);
                            v1880 = v1769;
                            v9478 = v12177;
                        }
                        v1879 = v1880;
                        v9475 = v9478;
                    }
                    let mut v1770: f64 = 0.0;
                    let mut v1772: f64 = 0.0;
                    let mut v1882: f64 = 0.0;
                    let mut v9480: Lanes<5> = Lanes([0.0; 5]);
                    let mut v9481: Lanes<5> = Lanes([0.0; 5]);
                    v1770 = v0;
                    v1772 = v1879;
                    v1882 = v0;
                    v9480 = v9475;
                    v9481 = v10549;
                    loop {
                        let v1771 = if v1770 < v18 { 1.0 } else { 0.0 };
                        if v1771 == 0.0 {
                            break;
                        }
                        let v1773 = v663 * v1772;
                        let v12220 = v10380 * v1772;
                        let v12223 = (Lanes([0.0, 0.0, v12220[0], 0.0, 0.0])) + (v9480 * v663);
                        let v1775 = (-v1773).exp();
                        let v12225 = (v12223 * v10360) * v1775;
                        let v1776 = if v1772 > v616 { 1.0 } else { 0.0 };
                        let v1810: f64;
                        let v1843: f64;
                        let v9482: Lanes<5>;
                        let v9483: Lanes<5>;
                        if v1776 != 0.0 {
                            let v1777 = v1773.exp();
                            let v1778 = -v1240;
                            let v1781 = v1777 - v1;
                            let v12264 = v9390 * v1781;
                            let v12265 = (v12223 * v1777) * v1262;
                            let v1784 = (((v1775 + v1773) - v1) + (v1262 * v1781)).sqrt();
                            let v1785 = v1778 * v1784;
                            let v12272 = (v9389 * v10360) * v1784;
                            let v12275 = (Lanes([0.0, 0.0, v12272[0], 0.0, 0.0])) + ((((v12225 + v12223) + ((Lanes([0.0, 0.0, v12264[0], 0.0, 0.0])) + v12265)) * (v9353 / (v10405 * v1784))) * v1778);
                            let v1786 = v211 / v1785;
                            let v12280 = v9390 * v1777;
                            let v1790 = ((-v1775) + v1) + (v1262 * v1777);
                            let v1791 = v1786 * v1790;
                            let v12286 = ((((v12275 * v1786) * v10360) / v1785) * v1790) + (((v12225 * v10360) + ((Lanes([0.0, 0.0, v12280[0], 0.0, 0.0])) + v12265)) * v1786);
                            v1810 = v1785;
                            v1843 = v1791;
                            v9482 = v12275;
                            v9483 = v12286;
                        } else {
                            let v1793 = if v1772 < v1792 { 1.0 } else { 0.0 };
                            let v1811: f64;
                            let v1844: f64;
                            let v9484: Lanes<5>;
                            let v9485: Lanes<5>;
                            if v1793 != 0.0 {
                                let v1796 = ((v1775 + v1773) - v1).sqrt();
                                let v1797 = v1240 * v1796;
                                let v12250 = v9389 * v1796;
                                let v12253 = (Lanes([0.0, 0.0, v12250[0], 0.0, 0.0])) + (((v12225 + v12223) * (v9353 / (v10405 * v1796))) * v1240);
                                let v1798 = v211 / v1797;
                                let v1800 = (-v1775) + v1;
                                let v1801 = v1798 * v1800;
                                let v12260 = ((((v12253 * v1798) * v10360) / v1797) * v1800) + ((v12225 * v10360) * v1798);
                                v1811 = v1797;
                                v1844 = v1801;
                                v9484 = v12253;
                                v9485 = v12260;
                            } else {
                                let v1802 = v211 / v663;
                                let v1803 = v1802.sqrt();
                                let v1804 = -v1803;
                                let v1805 = v1804 * v663;
                                let v1806 = v1805 * v1772;
                                let v12236 = (((((((v10380 * v1802) * v10360) / v663) * (v9353 / (v10405 * v1803))) * v10360) * v663) + (v10380 * v1804)) * v1772;
                                let v12239 = (Lanes([0.0, 0.0, v12236[0], 0.0, 0.0])) + (v9480 * v1805);
                                let v1808 = (v211 * v663).sqrt();
                                let v1809 = -v1808;
                                let v12244 = ((v10380 * v211) * (v9353 / (v10405 * v1808))) * v10360;
                                let v12245 = Lanes([0.0, 0.0, v12244[0], 0.0, 0.0]);
                                v1811 = v1806;
                                v1844 = v1809;
                                v9484 = v12239;
                                v9485 = v12245;
                            }
                            v1810 = v1811;
                            v1843 = v1844;
                            v9482 = v9484;
                            v9483 = v9485;
                        }
                        let v12287 = v9482 * v1810;
                        let v1816 = ((v1810 * v1810) + ((v90 * v1230) * v1230)).sqrt();
                        let v12291 = (v12287 + v12287) * (v9353 / (v10405 * v1816));
                        let v1817 = v1810 / v1816;
                        let v1819 = v13 * (v1 + v1817);
                        let v12295 = ((v9482 - (v12291 * v1817)) / v1816) * v13;
                        let v12297 = (v9482 + v12291) * v13;
                        let v1823 = (v13 * (v1810 + v1816)) + (v535 * v1230);
                        let v1824 = if v1823 < v0 { 1.0 } else { 0.0 };
                        let v1825: f64;
                        let v1842: f64;
                        let v9486: Lanes<5>;
                        let v9487: Lanes<5>;
                        if v1824 != 0.0 {
                            v1825 = v0;
                            v1842 = v0;
                            v9486 = v10549;
                            v9487 = v10549;
                        } else {
                            v1825 = v1823;
                            v1842 = v1819;
                            v9486 = v12297;
                            v9487 = v12295;
                        }
                        let v12298 = v9486 * v10360;
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
                        let v12299 = v12298 * v1827;
                        let v1835 = ((v1827 * v1827) + v1832).sqrt();
                        let v12303 = (v12299 + v12299) * (v9353 / (v10405 * v1835));
                        let v1836 = v1827 / v1835;
                        let v1838 = v13 * (v1 + v1836);
                        let v1841 = v1229 - (v13 * (v1827 + v1835));
                        let v12310 = ((v12298 + v12303) * v13) * v10360;
                        let v1845 = v1843 * v1838;
                        let v1846 = v1842 * v1845;
                        let v12317 = v12310 * v1841;
                        let v1851 = ((((v1841 * v1841) / v78) / v123) / v206) / v477;
                        let v12322 = ((((v12317 + v12317) / v78) / v123) / v206) / v477;
                        let v1852 = v78 * v1851;
                        let v1854 = (v1852 * v1846) / v1841;
                        let v1871 = ((v1865 + (v1843 / v130)) + ((v1843 * v12) / v123)) + v1854;
                        let v1872 = (((((v1694 - v1772) + (v1810 / v130)) + (((v1810 + (v1228 / v78)) * v12) / v123)) - v1244) + v1851) / v1871;
                        let v1873 = v1772 - v1872;
                        let v12346 = v9480 - (((((((v9469 - v9480) + (v9482 / v130)) + ((v9482 * v12) / v123)) - v12127) + v12322) - ((((v9483 / v130) + ((v9483 * v12) / v123)) + (((((v12322 * v78) * v1846) + (((v9487 * v1845) + (((v9483 * v1838) + ((((v12298 - (v12303 * v1836)) / v1835) * v13) * v1843)) * v1842)) * v1852)) - (v12310 * v1854)) / v1841)) * v1872)) / v1871);
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
                        v9480 = v12346;
                        v9481 = v9482;
                    }
                    let v1881 = v1244 + v1772;
                    let v12217 = v12127 + v9480;
                    let v1885 = v1694 + (v125 * (v1698 + v1882));
                    let v12219 = v9469 + (v9481 * v125);
                    v2258 = v1694;
                    v2259 = v1885;
                    v2260 = v1881;
                    v2585 = v2586;
                    v2600 = v1882;
                    v2678 = v1697;
                    v3331 = v1619;
                    v5094 = v1694;
                    v9463 = v9469;
                    v9464 = v12219;
                    v9465 = v12217;
                    v9466 = v9481;
                    v9467 = v9470;
                    v9468 = v9469;
                }
                let v1892 = if (if v1886 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v830 > (v1888 + v1889) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v2569: f64;
                let v2676: f64;
                let v4736: f64;
                let v4788: f64;
                let v5644: f64;
                let v5782: f64;
                let v9212: f64;
                let v9488: Lanes<6>;
                let v9489: Lanes<5>;
                let v9490: Lanes<1>;
                let v9491: Lanes<1>;
                let v9492: Lanes<5>;
                let v9493: Lanes<6>;
                if v1892 != 0.0 {
                    let v1895 = ((v867 - v349) + v1142) - v1199;
                    let v12349 = ((Lanes([v10534[0], v10534[1], 0.0, v10534[2], v10534[3]])) + v10761) - v10797;
                    let v1900 = ((v1897 * v477) * v123) / v663;
                    let v1901 = v1900.sqrt();
                    let v12355 = (((v10380 * v1900) * v10360) / v663) * (v9353 / (v10405 * v1901));
                    let v1903 = (v735 / v477) / v477;
                    let v12357 = (v10431 / v477) / v477;
                    let v12358 = v12355 * v1901;
                    let v12359 = v12358 + v12358;
                    let v1905 = (v1901 * v1901) / v1128;
                    let v12360 = v9406 * v1905;
                    let v1906 = v1905 / v1128;
                    let v12365 = v9406 * v1906;
                    let v12368 = ((((Lanes([0.0, 0.0, v12359[0], 0.0, 0.0])) - (Lanes([v12360[0], v12360[1], 0.0, v12360[2], v12360[3]]))) / v1128) - (Lanes([v12365[0], v12365[1], 0.0, v12365[2], v12365[3]]))) / v1128;
                    let v12370 = v10380 * v1906;
                    let v1908 = (v1906 * v663) / v78;
                    let v12373 = ((v12368 * v663) + (Lanes([0.0, 0.0, v12370[0], 0.0, 0.0]))) / v78;
                    let v12375 = v10380 * v1908;
                    let v1910 = (v1908 * v663) * v78;
                    let v12379 = v10380 * v1895;
                    let v1914 = (v90 * ((v663 * v1895) - v1)) / v1910;
                    let v1916 = (v1 + v1914).sqrt();
                    let v1917 = v1 - v1916;
                    let v1920 = v1 / v1903;
                    let v12397 = ((v12357 * v1920) * v10360) / v1903;
                    let v1921 = v1920 / v1906;
                    let v1922 = v1895 * v1895;
                    let v12402 = v12349 * v1895;
                    let v1923 = v1921 * v1922;
                    let v1925 = v78 / v1895;
                    let v1926 = v663 + v1925;
                    let v1927 = (v1923.ln()) / v1926;
                    let v12416 = (((((((Lanes([0.0, 0.0, v12397[0], 0.0, 0.0])) - (v12368 * v1921)) / v1906) * v1922) + ((v12402 + v12402) * v1921)) * (v9353 / v1923)) - (((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + (((v12349 * v1925) * v10360) / v1895)) * v1927)) / v1926;
                    let v12417 = v12416 - (v12349 + ((v12373 * v1917) + ((((((((Lanes([0.0, 0.0, v12379[0], 0.0, 0.0])) + (v12349 * v663)) * v90) - ((((v12373 * v663) + (Lanes([0.0, 0.0, v12375[0], 0.0, 0.0]))) * v78) * v1914)) / v1910) * (v9353 / (v10405 * v1916))) * v10360) * v1908)));
                    let v1929 = (v1927 - (v1895 + (v1908 * v1917))) - v1896;
                    let v12418 = v12417 * v1929;
                    let v1931 = v90 * v1896;
                    let v1934 = ((v1929 * v1929) + (v1931 * v1927)).sqrt();
                    let v1937 = v1927 - (v13 * (v1929 + v1934));
                    let v12427 = v12416 - ((v12417 + (((v12418 + v12418) + (v12416 * v1931)) * (v9353 / (v10405 * v1934)))) * v13);
                    let v1938 = v663 * v1937;
                    let v12428 = v10380 * v1937;
                    let v12431 = (Lanes([0.0, 0.0, v12428[0], 0.0, 0.0])) + (v12427 * v663);
                    let v1939 = v1938.exp();
                    let v1940 = v1938 - v1;
                    let v12433 = v12357 * v1939;
                    let v1942 = v1940 + (v1903 * v1939);
                    let v12437 = v12431 + ((Lanes([0.0, 0.0, v12433[0], 0.0, 0.0])) + ((v12431 * v1939) * v1903));
                    let v1945 = if (if v1942 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1940 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2570: f64;
                    let v2677: f64;
                    let v5645: f64;
                    let v5783: f64;
                    let v9213: f64;
                    let v9494: Lanes<6>;
                    let v9495: Lanes<5>;
                    let v9496: Lanes<5>;
                    let v9497: Lanes<6>;
                    if v1945 != 0.0 {
                        let v1946 = v1942.sqrt();
                        let v1947 = v1940.sqrt();
                        let v1948 = v1946 - v1947;
                        let v1949 = v1901 * v1948;
                        let v12445 = v12355 * v1948;
                        let v1951 = (v78 * v165) / v663;
                        let v1953 = -v663;
                        let v12452 = v10380 * v10360;
                        let v12453 = v12452 * v866;
                        let v12454 = v10531 * v1953;
                        let v1955 = (v1953 * v866).exp();
                        let v1957 = -(v1955 - v1);
                        let v1958 = v1 / v136;
                        let v1959 = v1951 * v1952;
                        let v1960 = v1959 * v1949;
                        let v12461 = ((((v10380 * v1951) * v10360) / v663) * v1952) * v1949;
                        let v12466 = ((((Lanes([0.0, 0.0, v12453[0], 0.0])) + (Lanes([v12454[0], v12454[1], 0.0, v12454[2]]))) * v1955) * v10360) * v1960;
                        let v1962 = (v1960 * v1957) * v1958;
                        let v12469 = ((((Lanes([0.0, 0.0, v12461[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v12445[0], 0.0, 0.0])) + (((v12437 * (v9353 / (v10405 * v1946))) - (v12431 * (v9353 / (v10405 * v1947)))) * v1901)) * v1959)) * v1957) + (Lanes([v12466[0], v12466[1], v12466[2], 0.0, v12466[3]]))) * v1958;
                        let v12470 = v10380 * v1200;
                        let v1966 = v1207 * v664;
                        let v12476 = v10382 * v1207;
                        let v1967 = (v90 * ((v663 * v1200) - v1)) / v1966;
                        let v12481 = ((((Lanes([0.0, 0.0, v12470[0], 0.0, 0.0])) + (v10798 * v663)) * v90) - (((v10806 * v664) + (Lanes([0.0, 0.0, v12476[0], 0.0, 0.0]))) * v1967)) / v1966;
                        let v1968 = v1 + v1967;
                        let v1970 = if v1968 < v1969 { 1.0 } else { 0.0 };
                        let v1974: f64;
                        let v9498: Lanes<5>;
                        if v1970 != 0.0 {
                            v1974 = v1971;
                            v9498 = v10549;
                        } else {
                            v1974 = v1968;
                            v9498 = v12481;
                        }
                        let v12483 = v10380 * v1207;
                        let v1973 = (v1207 * v663) * v13;
                        let v1975 = v1974.sqrt();
                        let v1976 = v1 - v1975;
                        let v1978 = v1200 + (v1973 * v1976);
                        let v12494 = v10798 + (((((v10806 * v663) + (Lanes([0.0, 0.0, v12483[0], 0.0, 0.0]))) * v13) * v1976) + (((v9498 * (v9353 / (v10405 * v1975))) * v10360) * v1973));
                        let v1979 = v1978 - v1937;
                        let v12495 = v12494 - v12427;
                        let v1980 = if v1979 < v0 { 1.0 } else { 0.0 };
                        let v1982: f64;
                        let v9499: Lanes<5>;
                        if v1980 != 0.0 {
                            v1982 = v0;
                            v9499 = v10549;
                        } else {
                            v1982 = v1979;
                            v9499 = v12495;
                        }
                        let v1983 = v1981 * v1982;
                        let v12496 = v9499 * v1981;
                        let v12498 = v12496 - (Lanes([v10531[0], v10531[1], 0.0, 0.0, v10531[2]]));
                        let v1986 = (v1983 - v866) - v1985;
                        let v12499 = v12498 * v1986;
                        let v1991 = ((v1986 * v1986) + ((v90 * v1983) * v1985)).sqrt();
                        let v1994 = v1983 - (v13 * (v1986 + v1991));
                        let v12509 = v12496 - ((v12498 + (((v12499 + v12499) + ((v12496 * v90) * v1985)) * (v9353 / (v10405 * v1991)))) * v13);
                        let v1995 = if v1994 > v1982 { 1.0 } else { 0.0 };
                        let v1996: f64;
                        let v9500: Lanes<5>;
                        if v1995 != 0.0 {
                            v1996 = v1982;
                            v9500 = v9499;
                        } else {
                            v1996 = v1994;
                            v9500 = v12509;
                        }
                        let v1997 = v122 * v68;
                        let v1998 = v166 * v68;
                        let v1999 = v136 * v68;
                        let v2001 = if v2000 == v0 { 1.0 } else { 0.0 };
                        let v2220: f64;
                        let v9501: Lanes<5>;
                        if v2001 != 0.0 {
                            v2220 = v0;
                            v9501 = v10549;
                        } else {
                            let v2006 = ((v2003 * v206) * v1998) * v1999;
                            let v2007 = v2006 / v718;
                            let v12512 = ((v10408 * v2007) * v10360) / v718;
                            let v12513 = v9401 * v2008;
                            let v2016 = (-(((((v2008 * v988) + v1113) + v1137) + v661) + v2013)) / v1997;
                            let v12520 = (((((Lanes([v12513[0], v12513[1], 0.0, 0.0, v12513[2]])) + v10742) + v9415) + (Lanes([0.0, 0.0, v10376[0], 0.0, 0.0]))) * v10360) / v1997;
                            let mut v2017: f64 = 0.0;
                            let mut v2065: f64 = 0.0;
                            let mut v9502: Lanes<5> = Lanes([0.0; 5]);
                            v2017 = v0;
                            v2065 = v0;
                            v9502 = v10549;
                            loop {
                                let v2019 = if v2017 <= v2018 { 1.0 } else { 0.0 };
                                if v2019 == 0.0 {
                                    break;
                                }
                                let v2020 = v2017 / v68;
                                let v2024 = (v1200 + v863) - ((v1996 * v2020) + v1937);
                                let v12525 = (v10798 + (Lanes([v9399[0], v9399[1], 0.0, 0.0, v9399[2]]))) - ((v9500 * v2020) + v12427);
                                let v2026 = v1 - (v2024 / v2002);
                                let v12527 = (v12525 / v2002) * v10360;
                                let v2028 = v2016 + (v2024 / v1997);
                                let v12529 = v12520 + (v12525 / v1997);
                                let v2029 = v2028 * v2028;
                                let v12530 = v12529 * v2028;
                                let v12531 = v12530 + v12530;
                                let v12532 = v12527 * v2026;
                                let v2033 = ((v2026 * v2026) + v2031).sqrt();
                                let v12538 = (v12527 + ((v12532 + v12532) * (v9353 / (v10405 * v2033)))) * v13;
                                let v2037 = (v13 * (v2026 + v2033)) + v2036;
                                let v2038 = if v2037 < v0 { 1.0 } else { 0.0 };
                                let v2040: f64;
                                let v9503: Lanes<5>;
                                if v2038 != 0.0 {
                                    v2040 = v0;
                                    v9503 = v10549;
                                } else {
                                    v2040 = v2037;
                                    v9503 = v12538;
                                }
                                let v2041 = v2040.sqrt();
                                let v2044 = v2039 * (v1 - (v2041 * v2040));
                                let v12546 = ((((v9503 * (v9353 / (v10405 * v2041))) * v2040) + (v9503 * v2041)) * v10360) * v2039;
                                let v2046 = (-v2044) / v2028;
                                let v12550 = ((v12546 * v10360) - (v12529 * v2046)) / v2028;
                                let v2048 = if v2046 < v2047 { 1.0 } else { 0.0 };
                                let v2060: f64;
                                let v9504: Lanes<5>;
                                if v2048 != 0.0 {
                                    v2060 = v0;
                                    v9504 = v10549;
                                } else {
                                    let v2049 = v2046.exp();
                                    let v12551 = v12550 * v2049;
                                    v2060 = v2049;
                                    v9504 = v12551;
                                }
                                let v2051 = v2050 * v2007;
                                let v2052 = v2051 * v2044;
                                let v12553 = (v12512 * v2050) * v2044;
                                let v2055 = (v2052 * v2044) * v2054;
                                let v12560 = ((((Lanes([0.0, 0.0, v12553[0], 0.0, 0.0])) + (v12546 * v2051)) * v2044) + (v12546 * v2052)) * v2054;
                                let v2058 = if ((v78 * v2028) + v2044) < v0 { 1.0 } else { 0.0 };
                                let v2066: f64;
                                let v9505: Lanes<5>;
                                if v2058 != 0.0 {
                                    v2066 = v2055;
                                    v9505 = v12560;
                                } else {
                                    let v2059 = v2006 * v2029;
                                    let v2061 = v2059 * v2060;
                                    let v12564 = ((v12531 * v2006) * v2060) + (v9504 * v2059);
                                    let v2064 = if (if v2061 < v2055 { 1.0 } else { 0.0 }) != 0.0 || (if v2028 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v2067: f64;
                                    let v9506: Lanes<5>;
                                    if v2064 != 0.0 {
                                        v2067 = v2055;
                                        v9506 = v12560;
                                    } else {
                                        v2067 = v2061;
                                        v9506 = v12564;
                                    }
                                    v2066 = v2067;
                                    v9505 = v9506;
                                }
                                let v2068 = v2065 + v2066;
                                let v12565 = v9502 + v9505;
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
                                v9502 = v12565;
                            }
                            v2220 = v2065;
                            v9501 = v9502;
                        }
                        let v2074 = if (if v297 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v21 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v2219: f64;
                        let v9507: Lanes<5>;
                        if v2074 != 0.0 {
                            v2219 = v0;
                            v9507 = v10549;
                        } else {
                            let v2192: f64;
                            let v9508: Lanes<5>;
                            if v281 != 0.0 {
                                let v2075 = v1128 * v1128;
                                let v12645 = v9406 * v1128;
                                let v12646 = v12645 + v12645;
                                let v2076 = v491 / v2075;
                                let v12649 = ((v12646 * v2076) * v10360) / v2075;
                                let v2077 = v78 / v491;
                                let v2078 = v2077 * v2075;
                                let v12653 = v9401 * v2080;
                                let v2082 = (v1895 - v665) - (v2080 * v988);
                                let v12656 = (v12646 * v2077) * v2082;
                                let v12659 = (Lanes([v12656[0], v12656[1], 0.0, v12656[2], v12656[3]])) + (((v12349 - (Lanes([0.0, 0.0, v10385[0], 0.0, 0.0]))) - (Lanes([v12653[0], v12653[1], 0.0, 0.0, v12653[2]]))) * v2078);
                                let v2084 = v1 + (v2078 * v2082);
                                let v12660 = v12659 * v2084;
                                let v2088 = ((v2084 * v2084) + v2086).sqrt();
                                let v12666 = (v12659 + ((v12660 + v12660) * (v9353 / (v10405 * v2088)))) * v13;
                                let v2092 = (v13 * (v2084 + v2088)) + v2091;
                                let v2093 = if v2092 < v0 { 1.0 } else { 0.0 };
                                let v2094: f64;
                                let v9509: Lanes<5>;
                                if v2093 != 0.0 {
                                    v2094 = v0;
                                    v9509 = v10549;
                                } else {
                                    v2094 = v2092;
                                    v9509 = v12666;
                                }
                                let v2096 = (v2094 + v362).sqrt();
                                let v2100 = v1 - v2096;
                                let v12672 = v12649 * v2100;
                                let v12677 = v10531 * v2103;
                                let v2109 = v2106 * v2107;
                                let v2111 = ((v2103 * v866) + v1937) - (v2109 * ((v1895 * v2097) + (v2076 * v2100)));
                                let v12681 = ((Lanes([v12677[0], v12677[1], 0.0, 0.0, v12677[2]])) + v12427) - (((v12349 * v2097) + ((Lanes([v12672[0], v12672[1], 0.0, v12672[2], v12672[3]])) + (((v9509 * (v9353 / (v10405 * v2096))) * v10360) * v2076))) * v2109);
                                let v12682 = v12681 * v2111;
                                let v2115 = ((v2111 * v2111) + v2113).sqrt();
                                let v12688 = (v12681 + ((v12682 + v12682) * (v9353 / (v10405 * v2115)))) * v13;
                                let v2119 = (v13 * (v2111 + v2115)) + v2118;
                                let v2120 = if v2119 < v0 { 1.0 } else { 0.0 };
                                let v2193: f64;
                                let v9510: Lanes<5>;
                                if v2120 != 0.0 {
                                    v2193 = v0;
                                    v9510 = v10549;
                                } else {
                                    v2193 = v2119;
                                    v9510 = v12688;
                                }
                                v2192 = v2193;
                                v9508 = v9510;
                            } else {
                                let v2123 = v2121 * v1895;
                                let v12566 = v12349 * v2121;
                                let v2124 = v1128 * v1128;
                                let v12567 = v9406 * v1128;
                                let v12568 = v12567 + v12567;
                                let v2125 = v491 / v2124;
                                let v12571 = ((v12568 * v2125) * v10360) / v2124;
                                let v2126 = v78 / v491;
                                let v2127 = v2126 * v2124;
                                let v12572 = v12568 * v2126;
                                let v12575 = v9401 * v2080;
                                let v2130 = (v2123 - v665) - (v2080 * v988);
                                let v12578 = v12572 * v2130;
                                let v12581 = (Lanes([v12578[0], v12578[1], 0.0, v12578[2], v12578[3]])) + (((v12566 - (Lanes([0.0, 0.0, v10385[0], 0.0, 0.0]))) - (Lanes([v12575[0], v12575[1], 0.0, 0.0, v12575[2]]))) * v2127);
                                let v2132 = v1 + (v2127 * v2130);
                                let v2134 = v78 * (v1 + v2127);
                                let v12582 = v12572 * v78;
                                let v2135 = v362 + v2134;
                                let v2138 = if (if v2132 < v2135 { 1.0 } else { 0.0 }) != 0.0 && (if v2134 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v2170: f64;
                                let v9511: Lanes<5>;
                                if v2138 != 0.0 {
                                    let v2139 = v2135 - v2132;
                                    let v12583 = Lanes([v12582[0], v12582[1], 0.0, v12582[2], v12582[3]]);
                                    let v12584 = v12583 - v12581;
                                    let v2140 = v2139 * v2139;
                                    let v12585 = v12584 * v2139;
                                    let v12586 = v12585 + v12585;
                                    let v2141 = v2134 * v2134;
                                    let v12587 = v12582 * v2134;
                                    let v12588 = v12587 + v12587;
                                    let v2142 = v2140 * v2140;
                                    let v12589 = v12586 * v2140;
                                    let v2143 = v2141 * v2141;
                                    let v12591 = v12588 * v2141;
                                    let v2144 = v2142 * v2140;
                                    let v2145 = v2143 * v2141;
                                    let v12604 = ((((v12591 + v12591) * v2141) + (v12588 * v2143)) * v2141) + (v12588 * v2145);
                                    let v2148 = (v2144 * v2140) + (v2145 * v2141);
                                    let v12606 = (((((v12589 + v12589) * v2140) + (v12586 * v2142)) * v2140) + (v12586 * v2144)) + (Lanes([v12604[0], v12604[1], 0.0, v12604[2], v12604[3]]));
                                    let v2165: f64;
                                    let v9512: Lanes<5>;
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
                                        let mut v9513: Lanes<5> = Lanes([0.0; 5]);
                                        v2154 = v0;
                                        v2156 = v2148;
                                        v9513 = v12606;
                                        loop {
                                            let v2155 = if v2154 < v2159 { 1.0 } else { 0.0 };
                                            if v2155 == 0.0 {
                                                break;
                                            }
                                            let v2157 = v2156.sqrt();
                                            let v12644 = v9513 * (v9353 / (v10405 * v2157));
                                            let v2158 = v2154 + v1;
                                            v2154 = v2158;
                                            v2156 = v2157;
                                            v9513 = v12644;
                                        }
                                        v2165 = v2156;
                                        v9512 = v9513;
                                    } else {
                                        let v2164 = v2148.powf(v2163);
                                        let v12610 = v12606 * (v2163 * (v2148.powf(v12607)));
                                        v2165 = v2164;
                                        v9512 = v12610;
                                    }
                                    let v2166 = v1 / v2165;
                                    let v2167 = v2139 * v2134;
                                    let v12615 = v12582 * v2139;
                                    let v2169 = v2135 - (v2167 * v2166);
                                    let v12621 = v12583 - ((((v12584 * v2134) + (Lanes([v12615[0], v12615[1], 0.0, v12615[2], v12615[3]]))) * v2166) + ((((v9512 * v2166) * v10360) / v2165) * v2167));
                                    v2170 = v2169;
                                    v9511 = v12621;
                                } else {
                                    v2170 = v2132;
                                    v9511 = v12581;
                                }
                                let v2171 = if v2170 <= v0 { 1.0 } else { 0.0 };
                                let v2173: f64;
                                let v9514: Lanes<5>;
                                if v2171 != 0.0 {
                                    v2173 = v0;
                                    v9514 = v10549;
                                } else {
                                    let v2172 = v2170.sqrt();
                                    let v12624 = v9511 * (v9353 / (v10405 * v2172));
                                    v2173 = v2172;
                                    v9514 = v12624;
                                }
                                let v2174 = v1 - v2173;
                                let v12626 = v12571 * v2174;
                                let v2178 = v143 / (v2106 + v143);
                                let v12631 = v10531 * v2103;
                                let v2182 = ((v2103 * v866) + v1) - (v2178 * (v2123 + (v2125 * v2174)));
                                let v12634 = (Lanes([v12631[0], v12631[1], 0.0, 0.0, v12631[2]])) - ((v12566 + ((Lanes([v12626[0], v12626[1], 0.0, v12626[2], v12626[3]])) + ((v9514 * v10360) * v2125))) * v2178);
                                let v12635 = v12634 * v2182;
                                let v2186 = ((v2182 * v2182) + v2184).sqrt();
                                let v12641 = (v12634 + ((v12635 + v12635) * (v9353 / (v10405 * v2186)))) * v13;
                                let v2190 = (v13 * (v2182 + v2186)) + v2189;
                                let v2191 = if v2190 < v0 { 1.0 } else { 0.0 };
                                let v2194: f64;
                                let v9515: Lanes<5>;
                                if v2191 != 0.0 {
                                    v2194 = v0;
                                    v9515 = v10549;
                                } else {
                                    v2194 = v2190;
                                    v9515 = v12641;
                                }
                                v2192 = v2194;
                                v9508 = v9515;
                            }
                            let v2195 = v2192 + v362;
                            let v2198 = (-v2196) / v2195;
                            let v2199 = v2198.exp();
                            let v2201 = v2200 * v2195;
                            let v2202 = v2201 * v1962;
                            let v2203 = v2202 * v2199;
                            let v12699 = ((((v9508 * v2200) * v1962) + (v12469 * v2201)) * v2199) + (((((v9508 * v2198) * v10360) / v2195) * v2199) * v2202);
                            v2219 = v2203;
                            v9507 = v12699;
                        }
                        let v2205 = if v2204 == v1 { 1.0 } else { 0.0 };
                        let v2571: f64;
                        let v9214: f64;
                        let v9516: Lanes<6>;
                        let v9517: Lanes<6>;
                        if v2205 != 0.0 {
                            let v2207 = (v206 * v12) * v166;
                            let v2210 = (v1953 * v2208).exp();
                            let v2215 = v2212 + (v2213 * v477);
                            let v2217 = (v2207 * v2210) * v2215;
                            let v2218 = v2216 / v2217;
                            let v2221 = v2219 + v2220;
                            let v12709 = (((((((v12452 * v2208) * v2210) * v2207) * v2215) * v2218) * v10360) / v2217) * v2221;
                            let v2224 = v2223 * v665;
                            let v2225 = v1 + (v2221 * v2218);
                            let v2226 = v2225.ln();
                            let v12715 = (v10385 * v2223) * v2226;
                            let v2229 = v2228 * v477;
                            let v2231 = (v2229 * v665).sqrt();
                            let v2232 = v1937 - (v2224 * v2226);
                            let v12723 = v12427 - ((Lanes([0.0, 0.0, v12715[0], 0.0, 0.0])) + (((((v9507 + v9501) * v2218) + (Lanes([0.0, 0.0, v12709[0], 0.0, 0.0]))) * (v9353 / v2225)) * v2224));
                            let v12724 = v12452 * v2232;
                            let v2234 = (v1953 * v2232).exp();
                            let v12729 = v10380 * v2232;
                            let v2238 = ((v2234 - v1) + (v663 * v2232)).sqrt();
                            let v12737 = v12452 * v1937;
                            let v2240 = (v1953 * v1937).exp();
                            let v2243 = ((v2240 - v1) + v1938).sqrt();
                            let v2244 = -v2231;
                            let v2245 = v2238 - v2243;
                            let v2246 = v2244 * v2245;
                            let v12748 = (((v10385 * v2229) * (v9353 / (v10405 * v2231))) * v10360) * v2245;
                            let v12751 = (Lanes([0.0, 0.0, v12748[0], 0.0, 0.0])) + (((((((Lanes([0.0, 0.0, v12724[0], 0.0, 0.0])) + (v12723 * v1953)) * v2234) + ((Lanes([0.0, 0.0, v12729[0], 0.0, 0.0])) + (v12723 * v663))) * (v9353 / (v10405 * v2238))) - (((((Lanes([0.0, 0.0, v12737[0], 0.0, 0.0])) + (v12427 * v1953)) * v2240) + v12431) * (v9353 / (v10405 * v2243)))) * v2244);
                            let v2572: f64;
                            let v9215: f64;
                            let v9518: Lanes<6>;
                            let v9519: Lanes<6>;
                            if v2247 != 0.0 {
                                let v2250 = v2219 + v2249;
                                let v2251 = v2248 / v2250;
                                let v2252 = v2251 * v1128;
                                let v12757 = v9406 * v2251;
                                let v2255 = v2253 * v2254;
                                let v12760 = v9365 * v2253;
                                let v12761 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v12760[0]]);
                                let v2257 = (v2255 - v2246) / v2252;
                                let v12764 = (((((v9507 * v2251) * v10360) / v2250) * v1128) + (Lanes([v12757[0], v12757[1], 0.0, v12757[2], v12757[3]]))) * v2257;
                                let v12767 = ((v12761 - (Lanes([v12751[0], v12751[1], v12751[2], v12751[3], v12751[4], 0.0]))) - (Lanes([v12764[0], v12764[1], v12764[2], v12764[3], v12764[4], 0.0]))) / v2252;
                                v2572 = v2255;
                                v9215 = v2257;
                                v9518 = v12761;
                                v9519 = v12767;
                            } else {
                                let v12752 = Lanes([v12751[0], v12751[1], v12751[2], v12751[3], v12751[4], 0.0]);
                                v2572 = v2246;
                                v9215 = v0;
                                v9518 = v12752;
                                v9519 = v11032;
                            }
                            v2571 = v2572;
                            v9214 = v9215;
                            v9516 = v9518;
                            v9517 = v9519;
                        } else {
                            v2571 = v0;
                            v9214 = v0;
                            v9516 = v11032;
                            v9517 = v11032;
                        }
                        v2570 = v2571;
                        v2677 = v1978;
                        v5645 = v2219;
                        v5783 = v1952;
                        v9213 = v9214;
                        v9494 = v9516;
                        v9495 = v12494;
                        v9496 = v9507;
                        v9497 = v9517;
                    } else {
                        v2570 = v0;
                        v2677 = v2678;
                        v5645 = v0;
                        v5783 = v0;
                        v9213 = v0;
                        v9494 = v11032;
                        v9495 = v9467;
                        v9496 = v10549;
                        v9497 = v11032;
                    }
                    v2569 = v2570;
                    v2676 = v2677;
                    v4736 = v1903;
                    v4788 = v1901;
                    v5644 = v5645;
                    v5782 = v5783;
                    v9212 = v9213;
                    v9488 = v9494;
                    v9489 = v9495;
                    v9490 = v12357;
                    v9491 = v12355;
                    v9492 = v9496;
                    v9493 = v9497;
                } else {
                    v2569 = v0;
                    v2676 = v2678;
                    v4736 = v736;
                    v4788 = v733;
                    v5644 = v0;
                    v5782 = v0;
                    v9212 = v0;
                    v9488 = v11032;
                    v9489 = v9467;
                    v9490 = v10432;
                    v9491 = v10427;
                    v9492 = v10549;
                    v9493 = v11032;
                }
                let v12768 = Lanes([v9465[0], v9465[1], v9465[2], v9465[3], v9465[4], 0.0]);
                let v12769 = Lanes([v9463[0], v9463[1], v9463[2], v9463[3], v9463[4], 0.0]);
                let v12770 = Lanes([v9464[0], v9464[1], v9464[2], v9464[3], v9464[4], 0.0]);
                let v12771 = Lanes([v9466[0], v9466[1], v9466[2], v9466[3], v9466[4], 0.0]);
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
                let mut v9520: Lanes<6> = Lanes([0.0; 6]);
                let mut v9521: Lanes<6> = Lanes([0.0; 6]);
                let mut v9522: Lanes<6> = Lanes([0.0; 6]);
                let mut v9523: Lanes<6> = Lanes([0.0; 6]);
                let mut v9524: Lanes<6> = Lanes([0.0; 6]);
                let mut v9525: Lanes<6> = Lanes([0.0; 6]);
                let mut v9526: Lanes<6> = Lanes([0.0; 6]);
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
                v9520 = v12768;
                v9521 = v12769;
                v9522 = v12770;
                v9523 = v11032;
                v9524 = v11032;
                v9525 = v11032;
                v9526 = v12771;
                loop {
                    let v2262 = if v2261 <= v18 { 1.0 } else { 0.0 };
                    if v2262 == 0.0 {
                        break;
                    }
                    let v2264 = v2263 - v1244;
                    let v2265 = v663 * v2264;
                    let v18834 = v10380 * v2264;
                    let v18837 = (Lanes([0.0, 0.0, v18834[0], 0.0, 0.0, 0.0])) + ((v9520 - (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3], 0.0]))) * v663);
                    let v2267 = (-v2265).exp();
                    let v18839 = (v18837 * v10360) * v2267;
                    let v2269 = if v2264 < v2268 { 1.0 } else { 0.0 };
                    let v2458: f64;
                    let v2471: f64;
                    let v9527: Lanes<6>;
                    let v9528: Lanes<6>;
                    if v2269 != 0.0 {
                        let v2272 = ((v2267 + v2265) - v1).sqrt();
                        let v2273 = v1240 * v2272;
                        let v18879 = v9389 * v2272;
                        let v18882 = (Lanes([0.0, 0.0, v18879[0], 0.0, 0.0, 0.0])) + (((v18839 + v18837) * (v9353 / (v10405 * v2272))) * v1240);
                        let v2277 = (v211 * ((-v2267) + v1)) / v2273;
                        let v18887 = (((v18839 * v10360) * v211) - (v18882 * v2277)) / v2273;
                        v2458 = v2273;
                        v2471 = v2277;
                        v9527 = v18882;
                        v9528 = v18887;
                    } else {
                        let v2278 = if v2264 > v616 { 1.0 } else { 0.0 };
                        let v2459: f64;
                        let v2472: f64;
                        let v9529: Lanes<6>;
                        let v9530: Lanes<6>;
                        if v2278 != 0.0 {
                            let v2279 = v2265.exp();
                            let v18849 = v18837 * v2279;
                            let v2280 = -v1240;
                            let v2284 = (v2279 + v2265) - v1;
                            let v18853 = v9390 * v2284;
                            let v2287 = (((v2267 + v2265) - v1) + (v1262 * v2284)).sqrt();
                            let v2288 = v2280 * v2287;
                            let v18861 = (v9389 * v10360) * v2287;
                            let v18864 = (Lanes([0.0, 0.0, v18861[0], 0.0, 0.0, 0.0])) + ((((v18839 + v18837) + ((Lanes([0.0, 0.0, v18853[0], 0.0, 0.0, 0.0])) + ((v18849 + v18837) * v1262))) * (v9353 / (v10405 * v2287))) * v2280);
                            let v2291 = v2279 + v1;
                            let v18866 = v9390 * v2291;
                            let v2295 = (v211 * (((-v2267) + v1) + (v1262 * v2291))) / v2288;
                            let v18874 = ((((v18839 * v10360) + ((Lanes([0.0, 0.0, v18866[0], 0.0, 0.0, 0.0])) + (v18849 * v1262))) * v211) - (v18864 * v2295)) / v2288;
                            v2459 = v2288;
                            v2472 = v2295;
                            v9529 = v18864;
                            v9530 = v18874;
                        } else {
                            let v2296 = -v1240;
                            let v18840 = v9389 * v10360;
                            let v2297 = v2296 * v2265;
                            let v18841 = v18840 * v2265;
                            let v18844 = (Lanes([0.0, 0.0, v18841[0], 0.0, 0.0, 0.0])) + (v18837 * v2296);
                            let v2298 = v2296 * v663;
                            let v18847 = (v18840 * v663) + (v10380 * v2296);
                            let v18848 = Lanes([0.0, 0.0, v18847[0], 0.0, 0.0, 0.0]);
                            v2459 = v2297;
                            v2472 = v2298;
                            v9529 = v18844;
                            v9530 = v18848;
                        }
                        v2458 = v2459;
                        v2471 = v2472;
                        v9527 = v9529;
                        v9528 = v9530;
                    }
                    let v2300 = v663 * v2299;
                    let v18888 = v10380 * v2299;
                    let v18891 = (Lanes([0.0, 0.0, v18888[0], 0.0, 0.0, 0.0])) + (v9521 * v663);
                    let v2301 = v2300.exp();
                    let v18892 = v18891 * v2301;
                    let v18893 = v12048 * v1504;
                    let v2303 = v750 * v750;
                    let v18895 = v10455 * v750;
                    let v2304 = (v1504 * v1504) / v2303;
                    let v18897 = (v18895 + v18895) * v2304;
                    let v18900 = ((v18893 + v18893) - (Lanes([0.0, 0.0, v18897[0], 0.0, 0.0]))) / v2303;
                    let v2305 = v78 * v759;
                    let v2307 = (v2301 + v2300) - v1;
                    let v18903 = (v10466 * v78) * v2307;
                    let v2310 = (v2304 + (v2305 * v2307)).sqrt();
                    let v18911 = ((Lanes([v18900[0], v18900[1], v18900[2], v18900[3], v18900[4], 0.0])) + ((Lanes([0.0, 0.0, v18903[0], 0.0, 0.0, 0.0])) + ((v18892 + v18891) * v2305))) * (v9353 / (v10405 * v2310));
                    let v2311 = v78 * v663;
                    let v2312 = v2311 * v759;
                    let v2313 = v2301 + v1;
                    let v18916 = (((v10380 * v78) * v759) + (v10466 * v2311)) * v2313;
                    let v2315 = v78 * v2310;
                    let v2316 = (v2312 * v2313) / v2315;
                    let v2317 = -v750;
                    let v18924 = v10455 * v10360;
                    let v18925 = v18924 * v2310;
                    let v2319 = (v2317 * v2310) - v1504;
                    let v18929 = Lanes([v12048[0], v12048[1], v12048[2], v12048[3], v12048[4], 0.0]);
                    let v18930 = ((Lanes([0.0, 0.0, v18925[0], 0.0, 0.0, 0.0])) + (v18911 * v2317)) - v18929;
                    let v2320 = v2317 * v2316;
                    let v18931 = v18924 * v2316;
                    let v18934 = (Lanes([0.0, 0.0, v18931[0], 0.0, 0.0, 0.0])) + (((((Lanes([0.0, 0.0, v18916[0], 0.0, 0.0, 0.0])) + (v18892 * v2312)) - ((v18911 * v78) * v2316)) / v2315) * v2317);
                    let v2323 = (v2321 - v2299) / v1208;
                    let v2324 = v663 * v2323;
                    let v18937 = v10380 * v2323;
                    let v18940 = (Lanes([0.0, 0.0, v18937[0], 0.0, 0.0, 0.0])) + (((v9522 - v9521) / v1208) * v663);
                    let v2325 = -v2324;
                    let v18941 = v18940 * v10360;
                    let v2327 = if v2325 >= v2326 { 1.0 } else { 0.0 };
                    let v2346: f64;
                    let v9531: Lanes<6>;
                    if v2327 != 0.0 {
                        v2346 = v2328;
                        v9531 = v11032;
                    } else {
                        let mut v2329: f64 = 0.0;
                        let mut v2332: f64 = 0.0;
                        let mut v9532: Lanes<6> = Lanes([0.0; 6]);
                        v2329 = v2325;
                        v2332 = v1;
                        v9532 = v18941;
                        loop {
                            let v2331 = if v2329 >= v2330 { 1.0 } else { 0.0 };
                            if v2331 == 0.0 {
                                break;
                            }
                            let v2334 = v2332 * v2333;
                            let v2335 = v2329 - v2330;
                            let edge0 = v2335;
                            let edge1 = v2334;
                            let edge2 = v9532;
                            v2329 = edge0;
                            v2332 = edge1;
                            v9532 = edge2;
                        }
                        let v2336 = v2329.exp();
                        let v2337 = v2332 * v2336;
                        let v18943 = (v9532 * v2336) * v2332;
                        v2346 = v2337;
                        v9531 = v18943;
                    }
                    let v2338 = v2325.exp();
                    let v2341 = ((v2338 + v2324) - v1).sqrt();
                    let v18948 = ((v18941 * v2338) + v18940) * (v9353 / (v10405 * v2341));
                    let v2343 = if v2323 < v2342 { 1.0 } else { 0.0 };
                    let v2369: f64;
                    let v2406: f64;
                    let v2410: f64;
                    let v9533: Lanes<6>;
                    let v9534: Lanes<6>;
                    let v9535: Lanes<6>;
                    if v2343 != 0.0 {
                        let v2344 = v750 * v2341;
                        let v18979 = v10455 * v2341;
                        let v18982 = (Lanes([0.0, 0.0, v18979[0], 0.0, 0.0, 0.0])) + (v18948 * v750);
                        let v2345 = v750 * v663;
                        let v2348 = (-v2346) + v1;
                        let v18987 = ((v10455 * v663) + (v10380 * v750)) * v2348;
                        let v2350 = v78 * v2341;
                        let v2351 = (v2345 * v2348) / v2350;
                        let v2352 = v2351 / v1208;
                        let v18995 = ((((Lanes([0.0, 0.0, v18987[0], 0.0, 0.0, 0.0])) + ((v9531 * v10360) * v2345)) - ((v18948 * v78) * v2351)) / v2350) / v1208;
                        let v2353 = -v2352;
                        let v18996 = v18995 * v10360;
                        v2369 = v2344;
                        v2406 = v2352;
                        v2410 = v2353;
                        v9533 = v18982;
                        v9534 = v18995;
                        v9535 = v18996;
                    } else {
                        let v2354 = if v2323 > v616 { 1.0 } else { 0.0 };
                        let v2370: f64;
                        let v2407: f64;
                        let v2411: f64;
                        let v9536: Lanes<6>;
                        let v9537: Lanes<6>;
                        let v9538: Lanes<6>;
                        if v2354 != 0.0 {
                            let v2355 = v2317 * v2341;
                            let v18961 = v18924 * v2341;
                            let v18964 = (Lanes([0.0, 0.0, v18961[0], 0.0, 0.0, 0.0])) + (v18948 * v2317);
                            let v2356 = v2317 * v663;
                            let v2358 = (-v2346) + v1;
                            let v18969 = ((v18924 * v663) + (v10380 * v2317)) * v2358;
                            let v2360 = v78 * v2341;
                            let v2361 = (v2356 * v2358) / v2360;
                            let v2362 = v2361 / v1208;
                            let v18977 = ((((Lanes([0.0, 0.0, v18969[0], 0.0, 0.0, 0.0])) + ((v9531 * v10360) * v2356)) - ((v18948 * v78) * v2361)) / v2360) / v1208;
                            let v2363 = -v2362;
                            let v18978 = v18977 * v10360;
                            v2370 = v2355;
                            v2407 = v2362;
                            v2411 = v2363;
                            v9536 = v18964;
                            v9537 = v18977;
                            v9538 = v18978;
                        } else {
                            let v18949 = v18924 * v2324;
                            let v2365 = (v2317 * v2324) / v748;
                            let v18953 = ((Lanes([0.0, 0.0, v18949[0], 0.0, 0.0, 0.0])) + (v18940 * v2317)) / v748;
                            let v2367 = (v2317 * v663) / v748;
                            let v18957 = ((v18924 * v663) + (v10380 * v2317)) / v748;
                            let v2368 = -v2367;
                            let v18958 = v18957 * v10360;
                            let v18959 = Lanes([0.0, 0.0, v18957[0], 0.0, 0.0, 0.0]);
                            let v18960 = Lanes([0.0, 0.0, v18958[0], 0.0, 0.0, 0.0]);
                            v2370 = v2365;
                            v2407 = v2367;
                            v2411 = v2368;
                            v9536 = v18953;
                            v9537 = v18959;
                            v9538 = v18960;
                        }
                        v2369 = v2370;
                        v2406 = v2407;
                        v2410 = v2411;
                        v9533 = v9536;
                        v9534 = v9537;
                        v9535 = v9538;
                    }
                    let v2371 = -v1225;
                    let v18997 = v11922 * v10360;
                    let v2372 = v0 - v2371;
                    let v18998 = v18997 * v10360;
                    let v2375 = if (if v2369 > v2372 { 1.0 } else { 0.0 }) != 0.0 && (if v2371 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2408: f64;
                    let v2413: f64;
                    let v9539: Lanes<6>;
                    let v9540: Lanes<6>;
                    if v2375 != 0.0 {
                        let v2376 = v2369 + v2371;
                        let v19000 = v9533 + (Lanes([v18997[0], v18997[1], v18997[2], v18997[3], v18997[4], 0.0]));
                        let v2377 = v2376 * v2376;
                        let v19001 = v19000 * v2376;
                        let v2378 = v2371 * v2371;
                        let v19003 = v18997 * v2371;
                        let v19005 = (v19001 + v19001) * v2377;
                        let v2380 = v2378 * v2378;
                        let v19007 = (v19003 + v19003) * v2378;
                        let v19008 = v19007 + v19007;
                        let v2381 = (v2377 * v2377) + v2380;
                        let v19010 = (v19005 + v19005) + (Lanes([v19008[0], v19008[1], v19008[2], v19008[3], v19008[4], 0.0]));
                        let v2398: f64;
                        let v9541: Lanes<6>;
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
                            let mut v9542: Lanes<6> = Lanes([0.0; 6]);
                            v2387 = v0;
                            v2389 = v2381;
                            v9542 = v19010;
                            loop {
                                let v2388 = if v2387 < v2392 { 1.0 } else { 0.0 };
                                if v2388 == 0.0 {
                                    break;
                                }
                                let v2390 = v2389.sqrt();
                                let v19232 = v9542 * (v9353 / (v10405 * v2390));
                                let v2391 = v2387 + v1;
                                v2387 = v2391;
                                v2389 = v2390;
                                v9542 = v19232;
                            }
                            v2398 = v2389;
                            v9541 = v9542;
                        } else {
                            let v2397 = v2381.powf(v2396);
                            let v19014 = v19010 * (v2396 * (v2381.powf(v19011)));
                            v2398 = v2397;
                            v9541 = v19014;
                        }
                        let v2399 = v1 / v2398;
                        let v19017 = ((v9541 * v2399) * v10360) / v2398;
                        let v2400 = v2376 * v2371;
                        let v19019 = v18997 * v2376;
                        let v2402 = v2371 * v2380;
                        let v19028 = ((v18997 * v2380) + (v19008 * v2371)) * v2399;
                        let v2404 = (v2402 * v2399) / v2381;
                        let v19034 = (((Lanes([v19028[0], v19028[1], v19028[2], v19028[3], v19028[4], 0.0])) + (v19017 * v2402)) - (v19010 * v2404)) / v2381;
                        let v2405 = v2372 + (v2400 * v2399);
                        let v19036 = (Lanes([v18998[0], v18998[1], v18998[2], v18998[3], v18998[4], 0.0])) + ((((v19000 * v2371) + (Lanes([v19019[0], v19019[1], v19019[2], v19019[3], v19019[4], 0.0]))) * v2399) + (v19017 * v2400));
                        v2408 = v2404;
                        v2413 = v2405;
                        v9539 = v19034;
                        v9540 = v19036;
                    } else {
                        v2408 = v1;
                        v2413 = v2369;
                        v9539 = v11032;
                        v9540 = v9533;
                    }
                    let v2409 = v2406 * v2408;
                    let v19039 = (v9534 * v2408) + (v9539 * v2406);
                    let v2412 = v2410 * v2408;
                    let v19042 = (v9535 * v2408) + (v9539 * v2410);
                    let v2414 = v1228 - v1504;
                    let v19043 = v12048 * v10360;
                    let v2415 = -v2414;
                    let v19044 = v19043 * v10360;
                    let v2416 = v2414 + v2415;
                    let v19045 = v19043 + v19044;
                    let v2419 = if (if v2413 < v2416 { 1.0 } else { 0.0 }) != 0.0 && (if v2415 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2450: f64;
                    let v2453: f64;
                    let v9543: Lanes<6>;
                    let v9544: Lanes<6>;
                    if v2419 != 0.0 {
                        let v2420 = v2416 - v2413;
                        let v19046 = Lanes([v19045[0], v19045[1], v19045[2], v19045[3], v19045[4], 0.0]);
                        let v19047 = v19046 - v9540;
                        let v2421 = v2420 * v2420;
                        let v19048 = v19047 * v2420;
                        let v2422 = v2415 * v2415;
                        let v19050 = v19044 * v2415;
                        let v19052 = (v19048 + v19048) * v2421;
                        let v2424 = v2422 * v2422;
                        let v19054 = (v19050 + v19050) * v2422;
                        let v19055 = v19054 + v19054;
                        let v2425 = (v2421 * v2421) + v2424;
                        let v19057 = (v19052 + v19052) + (Lanes([v19055[0], v19055[1], v19055[2], v19055[3], v19055[4], 0.0]));
                        let v2442: f64;
                        let v9545: Lanes<6>;
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
                            let mut v9546: Lanes<6> = Lanes([0.0; 6]);
                            v2431 = v0;
                            v2433 = v2425;
                            v9546 = v19057;
                            loop {
                                let v2432 = if v2431 < v2436 { 1.0 } else { 0.0 };
                                if v2432 == 0.0 {
                                    break;
                                }
                                let v2434 = v2433.sqrt();
                                let v19229 = v9546 * (v9353 / (v10405 * v2434));
                                let v2435 = v2431 + v1;
                                v2431 = v2435;
                                v2433 = v2434;
                                v9546 = v19229;
                            }
                            v2442 = v2433;
                            v9545 = v9546;
                        } else {
                            let v2441 = v2425.powf(v2440);
                            let v19061 = v19057 * (v2440 * (v2425.powf(v19058)));
                            v2442 = v2441;
                            v9545 = v19061;
                        }
                        let v2443 = v1 / v2442;
                        let v19064 = ((v9545 * v2443) * v10360) / v2442;
                        let v2444 = v2420 * v2415;
                        let v19066 = v19044 * v2420;
                        let v2446 = v2415 * v2424;
                        let v19075 = ((v19044 * v2424) + (v19055 * v2415)) * v2443;
                        let v2448 = (v2446 * v2443) / v2425;
                        let v19081 = (((Lanes([v19075[0], v19075[1], v19075[2], v19075[3], v19075[4], 0.0])) + (v19064 * v2446)) - (v19057 * v2448)) / v2425;
                        let v2449 = v2416 - (v2444 * v2443);
                        let v19082 = v19046 - ((((v19047 * v2415) + (Lanes([v19066[0], v19066[1], v19066[2], v19066[3], v19066[4], 0.0]))) * v2443) + (v19064 * v2444));
                        v2450 = v2448;
                        v2453 = v2449;
                        v9543 = v19081;
                        v9544 = v19082;
                    } else {
                        v2450 = v1;
                        v2453 = v2413;
                        v9543 = v11032;
                        v9544 = v9540;
                    }
                    let v2451 = v2412 * v2450;
                    let v19085 = (v19042 * v2450) + (v9543 * v2412);
                    let v2452 = v2409 * v2450;
                    let v19088 = (v19039 * v2450) + (v9543 * v2409);
                    let v2454 = v1504 + v2453;
                    let v19089 = v18929 + v9544;
                    let v2456 = if v2455 == v1 { 1.0 } else { 0.0 };
                    let v2562: f64;
                    let v2564: f64;
                    let v2565: f64;
                    let v2566: f64;
                    let v2567: f64;
                    let v2574: f64;
                    let v9547: Lanes<6>;
                    let v9548: Lanes<6>;
                    let v9549: Lanes<6>;
                    if v2456 != 0.0 {
                        v2562 = v18;
                        v2564 = v2263;
                        v2565 = v2299;
                        v2566 = v2321;
                        v2567 = v2455;
                        v2574 = v2261;
                        v9547 = v9520;
                        v9548 = v9521;
                        v9549 = v9522;
                    } else {
                        let v2463 = (((v2458 + v1504) + v2319) + v2453) + v2569;
                        let v19096 = v9405 * v2463;
                        let v2465 = (v2299 - v1200) - (v1048 * v2463);
                        let v19100 = (v9521 - (Lanes([v10798[0], v10798[1], v10798[2], v10798[3], v10798[4], 0.0]))) - ((Lanes([v19096[0], v19096[1], 0.0, v19096[2], v19096[3], 0.0])) + (((((v9527 + v18929) + v18930) + v9544) + v9488) * v1048));
                        let v2466 = v2320 + v2451;
                        let v19102 = v9405 * v2466;
                        let v2468 = v1 - (v1048 * v2466);
                        let v19106 = ((Lanes([v19102[0], v19102[1], 0.0, v19102[2], v19102[3], 0.0])) + ((v18934 + v19085) * v1048)) * v10360;
                        let v2469 = -v1048;
                        let v19107 = v9405 * v10360;
                        let v2470 = v2469 * v2452;
                        let v19108 = v19107 * v2452;
                        let v19111 = (Lanes([v19108[0], v19108[1], 0.0, v19108[2], v19108[3], 0.0])) + (v19088 * v2469);
                        let v2473 = v2469 * v2471;
                        let v19112 = v19107 * v2471;
                        let v19115 = (Lanes([v19112[0], v19112[1], 0.0, v19112[2], v19112[3], 0.0])) + (v9528 * v2469);
                        let v2479 = v2321 - (v2299 + (v125 * ((v13 * v1228) + v2458)));
                        let v19119 = v9522 - (v9521 + (v9527 * v125));
                        let v2481 = -(v125 * v2471);
                        let v19120 = (v9528 * v125) * v10360;
                        let v2484 = (v2263 - v2321) - (v131 * v2458);
                        let v19123 = (v9520 - v9522) - (v9527 * v131);
                        let v2487 = v1 - (v131 * v2471);
                        let v19125 = (v9528 * v131) * v10360;
                        let v2488 = v2468 * v2487;
                        let v19128 = (v19106 * v2487) + (v19125 * v2468);
                        let v2489 = v2468 * v2481;
                        let v19131 = (v19106 * v2481) + (v19120 * v2468);
                        let v2492 = v2470 * v2480;
                        let v19134 = v19111 * v2480;
                        let v2495 = v2473 * v2480;
                        let v19139 = v19115 * v2480;
                        let v2498 = (((v2488 - (v2489 * v2485)) - (v2492 * v2487)) + (v2495 * v2485)) + v362;
                        let v2499 = v1 / v2498;
                        let v2501 = v2487 - (v2481 * v2485);
                        let v2504 = (v2473 * v2485) - (v2470 * v2487);
                        let v2506 = (v2470 * v2481) - v2473;
                        let v2507 = v2495 - v2489;
                        let v2509 = (-v2468) * v2485;
                        let v2510 = v2468 - v2492;
                        let v2511 = -v2499;
                        let v19160 = ((((((v19128 - (v19131 * v2485)) - ((v19134 * v2487) + (v19125 * v2492))) + (v19139 * v2485)) * v2499) * v10360) / v2498) * v10360;
                        let v2516 = ((v2501 * v2465) + (v2504 * v2479)) + (v2506 * v2484);
                        let v2517 = v2511 * v2516;
                        let v19174 = (v19160 * v2516) + ((((((v19125 - (v19120 * v2485)) * v2465) + (v19100 * v2501)) + ((((v19115 * v2485) - ((v19111 * v2487) + (v19125 * v2470))) * v2479) + (v19119 * v2504))) + (((((v19111 * v2481) + (v19120 * v2470)) - v19115) * v2484) + (v19123 * v2506))) * v2511);
                        let v2522 = ((v2487 * v2465) + (v2488 * v2479)) + (v2507 * v2484);
                        let v2523 = v2511 * v2522;
                        let v19188 = (v19160 * v2522) + (((((v19125 * v2465) + (v19100 * v2487)) + ((v19128 * v2479) + (v19119 * v2488))) + (((v19139 - v19131) * v2484) + (v19123 * v2507))) * v2511);
                        let v2527 = (v2465 + (v2509 * v2479)) + (v2510 * v2484);
                        let v2528 = v2511 * v2527;
                        let v19199 = (v19160 * v2527) + (((v19100 + ((((v19106 * v10360) * v2485) * v2479) + (v19119 * v2509))) + (((v19106 - v19134) * v2484) + (v19123 * v2510))) * v2511);
                        let v2529 = v2517.abs();
                        let v19203 = v19174 * ((v10405 * (if v2517 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                        let v2530 = v2523.abs();
                        let v19207 = v19188 * ((v10405 * (if v2523 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                        let v2531 = if v2529 < v2530 { 1.0 } else { 0.0 };
                        let v2532: f64;
                        let v9550: Lanes<6>;
                        if v2531 != 0.0 {
                            v2532 = v2530;
                            v9550 = v19207;
                        } else {
                            v2532 = v2529;
                            v9550 = v19203;
                        }
                        let v2533 = v2528.abs();
                        let v19211 = v19199 * ((v10405 * (if v2528 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                        let v2534 = if v2532 < v2533 { 1.0 } else { 0.0 };
                        let v2543: f64;
                        let v9551: Lanes<6>;
                        if v2534 != 0.0 {
                            v2543 = v2533;
                            v9551 = v19211;
                        } else {
                            v2543 = v2532;
                            v9551 = v9550;
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
                        let v9552: Lanes<6>;
                        let v9553: Lanes<6>;
                        let v9554: Lanes<6>;
                        if v2549 != 0.0 {
                            let v2550 = v2548 / v2543;
                            let v19214 = ((v9551 * v2550) * v10360) / v2543;
                            let v2551 = v2517 * v2550;
                            let v19217 = (v19174 * v2550) + (v19214 * v2517);
                            let v2552 = v2523 * v2550;
                            let v19220 = (v19188 * v2550) + (v19214 * v2523);
                            let v2553 = v2528 * v2550;
                            let v19223 = (v19199 * v2550) + (v19214 * v2528);
                            v2554 = v2551;
                            v2556 = v2552;
                            v2558 = v2553;
                            v9552 = v19217;
                            v9553 = v19220;
                            v9554 = v19223;
                        } else {
                            v2554 = v2517;
                            v2556 = v2523;
                            v2558 = v2528;
                            v9552 = v19174;
                            v9553 = v19188;
                            v9554 = v19199;
                        }
                        let v2555 = v2299 + v2554;
                        let v19224 = v9521 + v9552;
                        let v2557 = v2321 + v2556;
                        let v19225 = v9522 + v9553;
                        let v2559 = v2263 + v2558;
                        let v19226 = v9520 + v9554;
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
                        v9547 = v19226;
                        v9548 = v19224;
                        v9549 = v19225;
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
                    v9520 = v9547;
                    v9521 = v9548;
                    v9522 = v9549;
                    v9523 = v18930;
                    v9524 = v9544;
                    v9525 = v19089;
                    v9526 = v9527;
                }
                let v2575 = if v2573 > v0 { 1.0 } else { 0.0 };
                if v2575 != 0.0 {
                } else {
                }
                let v2576 = if v2455 == v0 { 1.0 } else { 0.0 };
                let v2577: f64;
                let v2603: f64;
                let v2604: f64;
                let v9555: Lanes<6>;
                let v9556: Lanes<6>;
                let v9557: Lanes<6>;
                if v2576 != 0.0 {
                    v2577 = v2258;
                    v2603 = v2259;
                    v2604 = v2260;
                    v9555 = v12769;
                    v9556 = v12770;
                    v9557 = v12768;
                } else {
                    v2577 = v2299;
                    v2603 = v2321;
                    v2604 = v2263;
                    v9555 = v9521;
                    v9556 = v9522;
                    v9557 = v9520;
                }
                let v2579 = -v2578;
                let v12772 = v9523 * v10360;
                let v2580 = if v2579 <= v362 { 1.0 } else { 0.0 };
                let v2581: f64;
                let v9558: Lanes<6>;
                if v2580 != 0.0 {
                    v2581 = v362;
                    v9558 = v11032;
                } else {
                    v2581 = v2579;
                    v9558 = v12772;
                }
                let v2582 = v2581 * v1048;
                let v12774 = v9405 * v2581;
                let v12776 = (v9558 * v1048) + (Lanes([v12774[0], v12774[1], 0.0, v12774[2], v12774[3], 0.0]));
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
                let v9559: Lanes<6>;
                let v9560: Lanes<6>;
                let v9561: Lanes<6>;
                let v9562: Lanes<6>;
                let v9563: Lanes<6>;
                let v9564: Lanes<6>;
                let v9565: Lanes<6>;
                let v9566: Lanes<6>;
                let v9567: Lanes<6>;
                let v9568: Lanes<6>;
                let v9569: Lanes<6>;
                let v9570: Lanes<6>;
                let v9571: Lanes<6>;
                let v9572: Lanes<6>;
                let v9573: Lanes<6>;
                let v9574: Lanes<6>;
                if v2584 != 0.0 {
                    let v2588 = (-v168) * v139;
                    let v2594 = v2591 * ((v1504 + v2589) + v2592);
                    let v13713 = (((Lanes([v12048[0], v12048[1], v12048[2], v12048[3], v12048[4], 0.0])) + v9524) + v9525) * v2591;
                    let v2595 = v2588 * v2594;
                    let v13714 = v13713 * v2588;
                    let v2596 = v2595 * v13;
                    let v13715 = v13714 * v13;
                    let v2598 = v2595 * v2597;
                    let v13716 = v13714 * v2597;
                    let v2602 = (v2599 * v139) * v168;
                    let v13718 = (v9526 * v139) * v168;
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
                    v9559 = v11032;
                    v9560 = v11032;
                    v9561 = v11032;
                    v9562 = v9555;
                    v9563 = v11032;
                    v9564 = v13713;
                    v9565 = v11032;
                    v9566 = v9526;
                    v9567 = v11032;
                    v9568 = v11032;
                    v9569 = v9556;
                    v9570 = v9555;
                    v9571 = v13714;
                    v9572 = v13718;
                    v9573 = v13715;
                    v9574 = v13716;
                } else {
                    let v2605 = v1128 * v1128;
                    let v12777 = v9406 * v1128;
                    let v2606 = v491 / v2605;
                    let v12781 = (((v12777 + v12777) * v2606) * v10360) / v2605;
                    let v2607 = v78 / v2606;
                    let v12784 = ((v12781 * v2607) * v10360) / v2606;
                    let v2608 = v1200 - v362;
                    let v12785 = v12784 * v2608;
                    let v12788 = (Lanes([v12785[0], v12785[1], 0.0, v12785[2], v12785[3]])) + (v10798 * v2607);
                    let v2610 = v1 + (v2607 * v2608);
                    let v2611 = v1 + v2607;
                    let v2614 = if (if v2610 < v2611 { 1.0 } else { 0.0 }) != 0.0 && (if v2611 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2646: f64;
                    let v9575: Lanes<5>;
                    if v2614 != 0.0 {
                        let v2615 = v2611 - v2610;
                        let v12789 = Lanes([v12784[0], v12784[1], 0.0, v12784[2], v12784[3]]);
                        let v12790 = v12789 - v12788;
                        let v2616 = v2615 * v2615;
                        let v12791 = v12790 * v2615;
                        let v12792 = v12791 + v12791;
                        let v2617 = v2611 * v2611;
                        let v12793 = v12784 * v2611;
                        let v12794 = v12793 + v12793;
                        let v2618 = v2616 * v2616;
                        let v12795 = v12792 * v2616;
                        let v2619 = v2617 * v2617;
                        let v12797 = v12794 * v2617;
                        let v2620 = v2618 * v2616;
                        let v2621 = v2619 * v2617;
                        let v12810 = ((((v12797 + v12797) * v2617) + (v12794 * v2619)) * v2617) + (v12794 * v2621);
                        let v2624 = (v2620 * v2616) + (v2621 * v2617);
                        let v12812 = (((((v12795 + v12795) * v2616) + (v12792 * v2618)) * v2616) + (v12792 * v2620)) + (Lanes([v12810[0], v12810[1], 0.0, v12810[2], v12810[3]]));
                        let v2641: f64;
                        let v9576: Lanes<5>;
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
                            let mut v9577: Lanes<5> = Lanes([0.0; 5]);
                            v2630 = v0;
                            v2632 = v2624;
                            v9577 = v12812;
                            loop {
                                let v2631 = if v2630 < v2635 { 1.0 } else { 0.0 };
                                if v2631 == 0.0 {
                                    break;
                                }
                                let v2633 = v2632.sqrt();
                                let v13709 = v9577 * (v9353 / (v10405 * v2633));
                                let v2634 = v2630 + v1;
                                v2630 = v2634;
                                v2632 = v2633;
                                v9577 = v13709;
                            }
                            v2641 = v2632;
                            v9576 = v9577;
                        } else {
                            let v2640 = v2624.powf(v2639);
                            let v12816 = v12812 * (v2639 * (v2624.powf(v12813)));
                            v2641 = v2640;
                            v9576 = v12816;
                        }
                        let v2642 = v1 / v2641;
                        let v2643 = v2615 * v2611;
                        let v12821 = v12784 * v2615;
                        let v2645 = v2611 - (v2643 * v2642);
                        let v12827 = v12789 - ((((v12790 * v2611) + (Lanes([v12821[0], v12821[1], 0.0, v12821[2], v12821[3]]))) * v2642) + ((((v9576 * v2642) * v10360) / v2641) * v2643));
                        v2646 = v2645;
                        v9575 = v12827;
                    } else {
                        v2646 = v2610;
                        v9575 = v12788;
                    }
                    let v2647 = v2646.sqrt();
                    let v2648 = v1 - v2647;
                    let v12832 = v12781 * v2648;
                    let v2650 = v1200 + (v2606 * v2648);
                    let v12836 = v10798 + ((Lanes([v12832[0], v12832[1], 0.0, v12832[2], v12832[3]])) + (((v9575 * (v9353 / (v10405 * v2647))) * v10360) * v2606));
                    let v12837 = v12836 * v2650;
                    let v2654 = ((v2650 * v2650) + v2652).sqrt();
                    let v12843 = (v12836 + ((v12837 + v12837) * (v9353 / (v10405 * v2654)))) * v13;
                    let v2658 = (v13 * (v2650 + v2654)) + v2657;
                    let v2659 = if v2658 < v0 { 1.0 } else { 0.0 };
                    let v2660: f64;
                    let v9578: Lanes<5>;
                    if v2659 != 0.0 {
                        v2660 = v0;
                        v9578 = v10549;
                    } else {
                        v2660 = v2658;
                        v9578 = v12843;
                    }
                    let v2661 = v823 / v2660;
                    let v12846 = (v10567 - (v9578 * v2661)) / v2660;
                    let v2663 = v2662 - v1;
                    let v2664 = v2661.powf(v2663);
                    let v12853 = ((v12846 * (v2663 * (v2661.powf((v2663 - v9353))))) * v2661) + (v12846 * v2664);
                    let v2666 = v1 + (v2664 * v2661);
                    let v2668 = (v1 / v2662) - v1;
                    let v2669 = v2666.powf(v2668);
                    let v2670 = v2669 * v2666;
                    let v2671 = v823 / v2670;
                    let v12863 = (v10567 - ((((v12853 * (v2668 * (v2666.powf((v2668 - v9353))))) * v2666) + (v12853 * v2669)) * v2671)) / v2670;
                    let v2672 = if v2671 < v0 { 1.0 } else { 0.0 };
                    let v3003: f64;
                    let v3008: f64;
                    let v3015: f64;
                    let v3330: f64;
                    let v3354: f64;
                    let v3467: f64;
                    let v9579: Lanes<6>;
                    let v9580: Lanes<6>;
                    let v9581: Lanes<6>;
                    let v9582: Lanes<6>;
                    if v2672 != 0.0 {
                        v3003 = v2603;
                        v3008 = v2577;
                        v3015 = v2604;
                        v3330 = v3331;
                        v3354 = v0;
                        v3467 = v2585;
                        v9579 = v9556;
                        v9580 = v9555;
                        v9581 = v9557;
                        v9582 = v11032;
                    } else {
                        let v3004: f64;
                        let v3009: f64;
                        let v3016: f64;
                        let v3332: f64;
                        let v3355: f64;
                        let v3468: f64;
                        let v9583: Lanes<6>;
                        let v9584: Lanes<6>;
                        let v9585: Lanes<6>;
                        let v9586: Lanes<6>;
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
                            v9583 = v11032;
                            v9584 = v11032;
                            v9585 = v11032;
                            v9586 = v11032;
                        } else {
                            let v2679 = v2676 - v2577;
                            let v12865 = (Lanes([v9489[0], v9489[1], v9489[2], v9489[3], v9489[4], 0.0])) - v9555;
                            let v2680 = if v2679 >= v0 { 1.0 } else { 0.0 };
                            let v2681: f64;
                            let v9587: Lanes<6>;
                            if v2680 != 0.0 {
                                v2681 = v2679;
                                v9587 = v12865;
                            } else {
                                v2681 = v0;
                                v9587 = v11032;
                            }
                            let v12867 = Lanes([v12863[0], v12863[1], v12863[2], v12863[3], v12863[4], 0.0]);
                            let v12868 = (v9587 * v2682) - v12867;
                            let v2685 = ((v2682 * v2681) - v2671) - v1985;
                            let v2689 = (v90 * (v2686 * v2681)) * v1985;
                            let v12871 = ((v9587 * v2686) * v90) * v1985;
                            let v2690 = if v2689 > v0 { 1.0 } else { 0.0 };
                            let v2692: f64;
                            let v9588: Lanes<6>;
                            if v2690 != 0.0 {
                                v2692 = v2689;
                                v9588 = v12871;
                            } else {
                                let v2691 = -v2689;
                                let v12872 = v12871 * v10360;
                                v2692 = v2691;
                                v9588 = v12872;
                            }
                            let v12873 = v12868 * v2685;
                            let v2695 = ((v2685 * v2685) + v2692).sqrt();
                            let v2700 = (v2696 * v2681) - (v13 * (v2685 + v2695));
                            let v12882 = (v9587 * v2696) - ((v12868 + (((v12873 + v12873) + v9588) * (v9353 / (v10405 * v2695)))) * v13);
                            let v2701 = if v2700 <= v2681 { 1.0 } else { 0.0 };
                            let v2702: f64;
                            let v9589: Lanes<6>;
                            if v2701 != 0.0 {
                                v2702 = v2700;
                                v9589 = v12882;
                            } else {
                                v2702 = v2681;
                                v9589 = v9587;
                            }
                            let v2703 = if v2702 < v0 { 1.0 } else { 0.0 };
                            let v2705: f64;
                            let v9590: Lanes<6>;
                            if v2703 != 0.0 {
                                v2705 = v0;
                                v9590 = v11032;
                            } else {
                                let v2704 = if v2702 > v2671 { 1.0 } else { 0.0 };
                                let v2706: f64;
                                let v9591: Lanes<6>;
                                if v2704 != 0.0 {
                                    v2706 = v2671;
                                    v9591 = v12867;
                                } else {
                                    v2706 = v2702;
                                    v9591 = v9589;
                                }
                                v2705 = v2706;
                                v9590 = v9591;
                            }
                            let v2707 = v2577 + v2705;
                            let v12883 = v9555 + v9590;
                            let v2708 = if v2707 < v1512 { 1.0 } else { 0.0 };
                            let v2880: f64;
                            let v9592: Lanes<6>;
                            if v2708 != 0.0 {
                                let v12934 = v11939 * v1248;
                                let v12936 = (v12934 + v12934) - v11944;
                                let v2710 = if v1253 >= v2709 { 1.0 } else { 0.0 };
                                let v2712: f64;
                                let v9593: Lanes<4>;
                                if v2710 != 0.0 {
                                    v2712 = v1253;
                                    v9593 = v12936;
                                } else {
                                    v2712 = v2711;
                                    v9593 = v10630;
                                }
                                let v2713 = v2712.sqrt();
                                let v2715 = (v1248 - v2713) / v78;
                                let v12941 = (v11939 - (v9593 * (v9353 / (v10405 * v2713)))) / v78;
                                let v12946 = ((((v11948 - v11950) / v1262) * v11951) - v11957) / v1266;
                                let v2716 = if v2715 < v1239 { 1.0 } else { 0.0 };
                                let v2881: f64;
                                let v9594: Lanes<4>;
                                if v2716 != 0.0 {
                                    v2881 = v2715;
                                    v9594 = v12941;
                                } else {
                                    let v12947 = v12946 - v12941;
                                    let v2718 = (v1267 - v2715) - v1270;
                                    let v2720 = (v90 * v1267) * v1270;
                                    let v12949 = (v12946 * v90) * v1270;
                                    let v2721 = if v2720 > v0 { 1.0 } else { 0.0 };
                                    let v2723: f64;
                                    let v9595: Lanes<4>;
                                    if v2721 != 0.0 {
                                        v2723 = v2720;
                                        v9595 = v12949;
                                    } else {
                                        let v2722 = -v2720;
                                        let v12950 = v12949 * v10360;
                                        v2723 = v2722;
                                        v9595 = v12950;
                                    }
                                    let v12951 = v12947 * v2718;
                                    let v2726 = ((v2718 * v2718) + v2723).sqrt();
                                    let v2729 = v1267 - (v13 * (v2718 + v2726));
                                    let v12959 = v12946 - ((v12947 + (((v12951 + v12951) + v9595) * (v9353 / (v10405 * v2726)))) * v13);
                                    v2881 = v2729;
                                    v9594 = v12959;
                                }
                                let v12960 = Lanes([v9594[0], v9594[1], v9594[2], 0.0, v9594[3], 0.0]);
                                v2880 = v2881;
                                v9592 = v12960;
                            } else {
                                let v2735 = -((v1244 - v2707) - (((v1228 / v78) * v12) / v123));
                                let v12886 = ((Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3], 0.0])) - v12883) * v10360;
                                let v2737 = (v78 * v2735) + v1247;
                                let v12889 = (v12886 * v78) + (Lanes([0.0, 0.0, v11937[0], 0.0, 0.0, 0.0]));
                                let v12890 = v12889 * v2737;
                                let v2739 = v2735 * v2735;
                                let v12892 = v12886 * v2735;
                                let v12893 = v12892 + v12892;
                                let v2742 = (v2737 * v2737) - (v90 * (v2739 + v1243));
                                let v12897 = (v12890 + v12890) - ((v12893 + (Lanes([0.0, 0.0, v11932[0], 0.0, 0.0, 0.0]))) * v90);
                                let v2744 = if v2742 >= v2743 { 1.0 } else { 0.0 };
                                let v2746: f64;
                                let v9596: Lanes<6>;
                                if v2744 != 0.0 {
                                    v2746 = v2742;
                                    v9596 = v12897;
                                } else {
                                    v2746 = v2745;
                                    v9596 = v11032;
                                }
                                let v2747 = v2746.sqrt();
                                let v2749 = (v2737 - v2747) / v78;
                                let v12902 = (v12889 - (v9596 * (v9353 / (v10405 * v2747)))) / v78;
                                let v2750 = v2739 / v1243;
                                let v12903 = v11932 * v2750;
                                let v2751 = v2750 / v1262;
                                let v12907 = v9390 * v2751;
                                let v2753 = v78 / v2735;
                                let v2754 = v663 + v2753;
                                let v2755 = (v2751.ln()) / v2754;
                                let v12920 = ((((((v12893 - (Lanes([0.0, 0.0, v12903[0], 0.0, 0.0, 0.0]))) / v1243) - (Lanes([0.0, 0.0, v12907[0], 0.0, 0.0, 0.0]))) / v1262) * (v9353 / v2751)) - (((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0, 0.0])) + (((v12886 * v2753) * v10360) / v2735)) * v2755)) / v2754;
                                let v2756 = if v2749 < v1239 { 1.0 } else { 0.0 };
                                let v2882: f64;
                                let v9597: Lanes<6>;
                                if v2756 != 0.0 {
                                    v2882 = v2749;
                                    v9597 = v12902;
                                } else {
                                    let v12921 = v12920 - v12902;
                                    let v2758 = (v2755 - v2749) - v1270;
                                    let v2760 = (v90 * v2755) * v1270;
                                    let v12923 = (v12920 * v90) * v1270;
                                    let v2761 = if v2760 > v0 { 1.0 } else { 0.0 };
                                    let v2763: f64;
                                    let v9598: Lanes<6>;
                                    if v2761 != 0.0 {
                                        v2763 = v2760;
                                        v9598 = v12923;
                                    } else {
                                        let v2762 = -v2760;
                                        let v12924 = v12923 * v10360;
                                        v2763 = v2762;
                                        v9598 = v12924;
                                    }
                                    let v12925 = v12921 * v2758;
                                    let v2766 = ((v2758 * v2758) + v2763).sqrt();
                                    let v2769 = v2755 - (v13 * (v2758 + v2766));
                                    let v12933 = v12920 - ((v12921 + (((v12925 + v12925) + v9598) * (v9353 / (v10405 * v2766)))) * v13);
                                    v2882 = v2769;
                                    v9597 = v12933;
                                }
                                v2880 = v2882;
                                v9592 = v9597;
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
                            let v9599: Lanes<6>;
                            let v9600: Lanes<6>;
                            let v9601: Lanes<6>;
                            if v2778 != 0.0 {
                                let mut v2779: f64 = 0.0;
                                let mut v2781: f64 = 0.0;
                                let mut v2884: f64 = 0.0;
                                let mut v9602: Lanes<6> = Lanes([0.0; 6]);
                                let mut v9603: Lanes<6> = Lanes([0.0; 6]);
                                v2779 = v0;
                                v2781 = v2880;
                                v2884 = v0;
                                v9602 = v9592;
                                v9603 = v11032;
                                loop {
                                    let v2780 = if v2779 < v18 { 1.0 } else { 0.0 };
                                    if v2780 == 0.0 {
                                        break;
                                    }
                                    let v2782 = v663 * v2781;
                                    let v13097 = v10380 * v2781;
                                    let v13100 = (Lanes([0.0, 0.0, v13097[0], 0.0, 0.0, 0.0])) + (v9602 * v663);
                                    let v2784 = (-v2782).exp();
                                    let v13102 = (v13100 * v10360) * v2784;
                                    let v2785 = if v2781 > v616 { 1.0 } else { 0.0 };
                                    let v2819: f64;
                                    let v2852: f64;
                                    let v9604: Lanes<6>;
                                    let v9605: Lanes<6>;
                                    if v2785 != 0.0 {
                                        let v2786 = v2782.exp();
                                        let v2787 = -v1240;
                                        let v2790 = v2786 - v1;
                                        let v13141 = v9390 * v2790;
                                        let v13142 = (v13100 * v2786) * v1262;
                                        let v2793 = (((v2784 + v2782) - v1) + (v1262 * v2790)).sqrt();
                                        let v2794 = v2787 * v2793;
                                        let v13149 = (v9389 * v10360) * v2793;
                                        let v13152 = (Lanes([0.0, 0.0, v13149[0], 0.0, 0.0, 0.0])) + ((((v13102 + v13100) + ((Lanes([0.0, 0.0, v13141[0], 0.0, 0.0, 0.0])) + v13142)) * (v9353 / (v10405 * v2793))) * v2787);
                                        let v2795 = v211 / v2794;
                                        let v13157 = v9390 * v2786;
                                        let v2799 = ((-v2784) + v1) + (v1262 * v2786);
                                        let v2800 = v2795 * v2799;
                                        let v13163 = ((((v13152 * v2795) * v10360) / v2794) * v2799) + (((v13102 * v10360) + ((Lanes([0.0, 0.0, v13157[0], 0.0, 0.0, 0.0])) + v13142)) * v2795);
                                        v2819 = v2794;
                                        v2852 = v2800;
                                        v9604 = v13152;
                                        v9605 = v13163;
                                    } else {
                                        let v2802 = if v2781 < v2801 { 1.0 } else { 0.0 };
                                        let v2820: f64;
                                        let v2853: f64;
                                        let v9606: Lanes<6>;
                                        let v9607: Lanes<6>;
                                        if v2802 != 0.0 {
                                            let v2805 = ((v2784 + v2782) - v1).sqrt();
                                            let v2806 = v1240 * v2805;
                                            let v13127 = v9389 * v2805;
                                            let v13130 = (Lanes([0.0, 0.0, v13127[0], 0.0, 0.0, 0.0])) + (((v13102 + v13100) * (v9353 / (v10405 * v2805))) * v1240);
                                            let v2807 = v211 / v2806;
                                            let v2809 = (-v2784) + v1;
                                            let v2810 = v2807 * v2809;
                                            let v13137 = ((((v13130 * v2807) * v10360) / v2806) * v2809) + ((v13102 * v10360) * v2807);
                                            v2820 = v2806;
                                            v2853 = v2810;
                                            v9606 = v13130;
                                            v9607 = v13137;
                                        } else {
                                            let v2811 = v211 / v663;
                                            let v2812 = v2811.sqrt();
                                            let v2813 = -v2812;
                                            let v2814 = v2813 * v663;
                                            let v2815 = v2814 * v2781;
                                            let v13113 = (((((((v10380 * v2811) * v10360) / v663) * (v9353 / (v10405 * v2812))) * v10360) * v663) + (v10380 * v2813)) * v2781;
                                            let v13116 = (Lanes([0.0, 0.0, v13113[0], 0.0, 0.0, 0.0])) + (v9602 * v2814);
                                            let v2817 = (v211 * v663).sqrt();
                                            let v2818 = -v2817;
                                            let v13121 = ((v10380 * v211) * (v9353 / (v10405 * v2817))) * v10360;
                                            let v13122 = Lanes([0.0, 0.0, v13121[0], 0.0, 0.0, 0.0]);
                                            v2820 = v2815;
                                            v2853 = v2818;
                                            v9606 = v13116;
                                            v9607 = v13122;
                                        }
                                        v2819 = v2820;
                                        v2852 = v2853;
                                        v9604 = v9606;
                                        v9605 = v9607;
                                    }
                                    let v13164 = v9604 * v2819;
                                    let v2825 = ((v2819 * v2819) + ((v90 * v1230) * v1230)).sqrt();
                                    let v13168 = (v13164 + v13164) * (v9353 / (v10405 * v2825));
                                    let v2826 = v2819 / v2825;
                                    let v2828 = v13 * (v1 + v2826);
                                    let v13172 = ((v9604 - (v13168 * v2826)) / v2825) * v13;
                                    let v13174 = (v9604 + v13168) * v13;
                                    let v2832 = (v13 * (v2819 + v2825)) + (v535 * v1230);
                                    let v2833 = if v2832 < v0 { 1.0 } else { 0.0 };
                                    let v2834: f64;
                                    let v2851: f64;
                                    let v9608: Lanes<6>;
                                    let v9609: Lanes<6>;
                                    if v2833 != 0.0 {
                                        v2834 = v0;
                                        v2851 = v0;
                                        v9608 = v11032;
                                        v9609 = v11032;
                                    } else {
                                        v2834 = v2832;
                                        v2851 = v2828;
                                        v9608 = v13174;
                                        v9609 = v13172;
                                    }
                                    let v13175 = v9608 * v10360;
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
                                    let v13176 = v13175 * v2836;
                                    let v2844 = ((v2836 * v2836) + v2841).sqrt();
                                    let v13180 = (v13176 + v13176) * (v9353 / (v10405 * v2844));
                                    let v2845 = v2836 / v2844;
                                    let v2847 = v13 * (v1 + v2845);
                                    let v2850 = v1229 - (v13 * (v2836 + v2844));
                                    let v13187 = ((v13175 + v13180) * v13) * v10360;
                                    let v2854 = v2852 * v2847;
                                    let v2855 = v2851 * v2854;
                                    let v13194 = v13187 * v2850;
                                    let v2860 = ((((v2850 * v2850) / v78) / v123) / v206) / v477;
                                    let v13199 = ((((v13194 + v13194) / v78) / v123) / v206) / v477;
                                    let v2861 = v78 * v2860;
                                    let v2863 = (v2861 * v2855) / v2850;
                                    let v2872 = (v2869 + (v2852 / v130)) + v2863;
                                    let v2873 = ((((-v2781) + (v2819 / v130)) - v1244) + v2860) / v2872;
                                    let v2874 = v2781 - v2873;
                                    let v13218 = v9602 - ((((((v9602 * v10360) + (v9604 / v130)) - (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3], 0.0]))) + v13199) - (((v9605 / v130) + (((((v13199 * v78) * v2855) + (((v9609 * v2854) + (((v9605 * v2847) + ((((v13175 - (v13180 * v2845)) / v2844) * v13) * v2852)) * v2851)) * v2861)) - (v13187 * v2863)) / v2850)) * v2873)) / v2872);
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
                                    v9602 = v13218;
                                    v9603 = v9604;
                                }
                                let v2883 = v1244 + v2781;
                                let v13094 = (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3], 0.0])) + v9602;
                                let v2886 = v2883 - (v2884 / v130);
                                let v13096 = v13094 - (v9603 / v130);
                                v3000 = v2886;
                                v3017 = v2883;
                                v3356 = v2884;
                                v3469 = v1;
                                v9599 = v13096;
                                v9600 = v13094;
                                v9601 = v9603;
                            } else {
                                let mut v2887: f64 = 0.0;
                                let mut v2889: f64 = 0.0;
                                let mut v2997: f64 = 0.0;
                                let mut v9610: Lanes<6> = Lanes([0.0; 6]);
                                let mut v9611: Lanes<6> = Lanes([0.0; 6]);
                                v2887 = v0;
                                v2889 = v2880;
                                v2997 = v0;
                                v9610 = v9592;
                                v9611 = v11032;
                                loop {
                                    let v2888 = if v2887 < v18 { 1.0 } else { 0.0 };
                                    if v2888 == 0.0 {
                                        break;
                                    }
                                    let v2890 = v663 * v2889;
                                    let v12965 = v10380 * v2889;
                                    let v12968 = (Lanes([0.0, 0.0, v12965[0], 0.0, 0.0, 0.0])) + (v9610 * v663);
                                    let v2892 = (-v2890).exp();
                                    let v12970 = (v12968 * v10360) * v2892;
                                    let v2893 = if v2889 > v616 { 1.0 } else { 0.0 };
                                    let v2927: f64;
                                    let v2960: f64;
                                    let v9612: Lanes<6>;
                                    let v9613: Lanes<6>;
                                    if v2893 != 0.0 {
                                        let v2894 = v2890.exp();
                                        let v2895 = -v1240;
                                        let v2898 = v2894 - v1;
                                        let v13009 = v9390 * v2898;
                                        let v13010 = (v12968 * v2894) * v1262;
                                        let v2901 = (((v2892 + v2890) - v1) + (v1262 * v2898)).sqrt();
                                        let v2902 = v2895 * v2901;
                                        let v13017 = (v9389 * v10360) * v2901;
                                        let v13020 = (Lanes([0.0, 0.0, v13017[0], 0.0, 0.0, 0.0])) + ((((v12970 + v12968) + ((Lanes([0.0, 0.0, v13009[0], 0.0, 0.0, 0.0])) + v13010)) * (v9353 / (v10405 * v2901))) * v2895);
                                        let v2903 = v211 / v2902;
                                        let v13025 = v9390 * v2894;
                                        let v2907 = ((-v2892) + v1) + (v1262 * v2894);
                                        let v2908 = v2903 * v2907;
                                        let v13031 = ((((v13020 * v2903) * v10360) / v2902) * v2907) + (((v12970 * v10360) + ((Lanes([0.0, 0.0, v13025[0], 0.0, 0.0, 0.0])) + v13010)) * v2903);
                                        v2927 = v2902;
                                        v2960 = v2908;
                                        v9612 = v13020;
                                        v9613 = v13031;
                                    } else {
                                        let v2910 = if v2889 < v2909 { 1.0 } else { 0.0 };
                                        let v2928: f64;
                                        let v2961: f64;
                                        let v9614: Lanes<6>;
                                        let v9615: Lanes<6>;
                                        if v2910 != 0.0 {
                                            let v2913 = ((v2892 + v2890) - v1).sqrt();
                                            let v2914 = v1240 * v2913;
                                            let v12995 = v9389 * v2913;
                                            let v12998 = (Lanes([0.0, 0.0, v12995[0], 0.0, 0.0, 0.0])) + (((v12970 + v12968) * (v9353 / (v10405 * v2913))) * v1240);
                                            let v2915 = v211 / v2914;
                                            let v2917 = (-v2892) + v1;
                                            let v2918 = v2915 * v2917;
                                            let v13005 = ((((v12998 * v2915) * v10360) / v2914) * v2917) + ((v12970 * v10360) * v2915);
                                            v2928 = v2914;
                                            v2961 = v2918;
                                            v9614 = v12998;
                                            v9615 = v13005;
                                        } else {
                                            let v2919 = v211 / v663;
                                            let v2920 = v2919.sqrt();
                                            let v2921 = -v2920;
                                            let v2922 = v2921 * v663;
                                            let v2923 = v2922 * v2889;
                                            let v12981 = (((((((v10380 * v2919) * v10360) / v663) * (v9353 / (v10405 * v2920))) * v10360) * v663) + (v10380 * v2921)) * v2889;
                                            let v12984 = (Lanes([0.0, 0.0, v12981[0], 0.0, 0.0, 0.0])) + (v9610 * v2922);
                                            let v2925 = (v211 * v663).sqrt();
                                            let v2926 = -v2925;
                                            let v12989 = ((v10380 * v211) * (v9353 / (v10405 * v2925))) * v10360;
                                            let v12990 = Lanes([0.0, 0.0, v12989[0], 0.0, 0.0, 0.0]);
                                            v2928 = v2923;
                                            v2961 = v2926;
                                            v9614 = v12984;
                                            v9615 = v12990;
                                        }
                                        v2927 = v2928;
                                        v2960 = v2961;
                                        v9612 = v9614;
                                        v9613 = v9615;
                                    }
                                    let v13032 = v9612 * v2927;
                                    let v2933 = ((v2927 * v2927) + ((v90 * v1230) * v1230)).sqrt();
                                    let v13036 = (v13032 + v13032) * (v9353 / (v10405 * v2933));
                                    let v2934 = v2927 / v2933;
                                    let v2936 = v13 * (v1 + v2934);
                                    let v13040 = ((v9612 - (v13036 * v2934)) / v2933) * v13;
                                    let v13042 = (v9612 + v13036) * v13;
                                    let v2940 = (v13 * (v2927 + v2933)) + (v535 * v1230);
                                    let v2941 = if v2940 < v0 { 1.0 } else { 0.0 };
                                    let v2942: f64;
                                    let v2959: f64;
                                    let v9616: Lanes<6>;
                                    let v9617: Lanes<6>;
                                    if v2941 != 0.0 {
                                        v2942 = v0;
                                        v2959 = v0;
                                        v9616 = v11032;
                                        v9617 = v11032;
                                    } else {
                                        v2942 = v2940;
                                        v2959 = v2936;
                                        v9616 = v13042;
                                        v9617 = v13040;
                                    }
                                    let v13043 = v9616 * v10360;
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
                                    let v13044 = v13043 * v2944;
                                    let v2952 = ((v2944 * v2944) + v2949).sqrt();
                                    let v13048 = (v13044 + v13044) * (v9353 / (v10405 * v2952));
                                    let v2953 = v2944 / v2952;
                                    let v2955 = v13 * (v1 + v2953);
                                    let v2958 = v1229 - (v13 * (v2944 + v2952));
                                    let v13055 = ((v13043 + v13048) * v13) * v10360;
                                    let v2962 = v2960 * v2955;
                                    let v2963 = v2959 * v2962;
                                    let v13062 = v13055 * v2958;
                                    let v2968 = ((((v2958 * v2958) / v78) / v123) / v206) / v477;
                                    let v13067 = ((((v13062 + v13062) / v78) / v123) / v206) / v477;
                                    let v2969 = v78 * v2968;
                                    let v2971 = (v2969 * v2963) / v2958;
                                    let v2988 = ((v2982 + (v2960 / v130)) + ((v2960 * v12) / v123)) + v2971;
                                    let v2989 = (((((v2707 - v2889) + (v2927 / v130)) + (((v2927 + (v1228 / v78)) * v12) / v123)) - v1244) + v2968) / v2988;
                                    let v2990 = v2889 - v2989;
                                    let v13092 = v9610 - (((((((v12883 - v9610) + (v9612 / v130)) + ((v9612 * v12) / v123)) - (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3], 0.0]))) + v13067) - ((((v9613 / v130) + ((v9613 * v12) / v123)) + (((((v13067 * v78) * v2963) + (((v9617 * v2962) + (((v9613 * v2955) + ((((v13043 - (v13048 * v2953)) / v2952) * v13) * v2960)) * v2959)) * v2969)) - (v13055 * v2971)) / v2958)) * v2989)) / v2988);
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
                                    v9610 = v13092;
                                    v9611 = v9612;
                                }
                                let v2996 = v1244 + v2889;
                                let v12962 = (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3], 0.0])) + v9610;
                                let v2999 = v2996 - (v2997 / v130);
                                let v12964 = v12962 - (v9611 / v130);
                                v3000 = v2999;
                                v3017 = v2996;
                                v3356 = v2997;
                                v3469 = v78;
                                v9599 = v12964;
                                v9600 = v12962;
                                v9601 = v9611;
                            }
                            let v3001 = if v3000 < v0 { 1.0 } else { 0.0 };
                            let v3005: f64;
                            let v9618: Lanes<6>;
                            if v3001 != 0.0 {
                                v3005 = v0;
                                v9618 = v11032;
                            } else {
                                v3005 = v3000;
                                v9618 = v9599;
                            }
                            v3004 = v3005;
                            v3009 = v2707;
                            v3016 = v3017;
                            v3332 = v3333;
                            v3355 = v3356;
                            v3468 = v3469;
                            v9583 = v9618;
                            v9584 = v12883;
                            v9585 = v9600;
                            v9586 = v9601;
                        }
                        v3003 = v3004;
                        v3008 = v3009;
                        v3015 = v3016;
                        v3330 = v3332;
                        v3354 = v3355;
                        v3467 = v3468;
                        v9579 = v9583;
                        v9580 = v9584;
                        v9581 = v9585;
                        v9582 = v9586;
                    }
                    let v3002 = if v2577 < v0 { 1.0 } else { 0.0 };
                    let v3007: f64;
                    let v9619: Lanes<6>;
                    if v3002 != 0.0 {
                        v3007 = v2577;
                        v9619 = v9555;
                    } else {
                        v3007 = v3008;
                        v9619 = v9580;
                    }
                    let v3006 = if v3003 < v20 { 1.0 } else { 0.0 };
                    let v3014: f64;
                    let v9620: Lanes<6>;
                    if v3006 != 0.0 {
                        let v3013 = v3007 + (v125 * ((v13 * v1228) + v2599));
                        let v13220 = v9619 + (v9526 * v125);
                        v3014 = v3013;
                        v9620 = v13220;
                    } else {
                        v3014 = v3003;
                        v9620 = v9579;
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
                    let mut v9621: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9622: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9623: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9624: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9625: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9626: Lanes<6> = Lanes([0.0; 6]);
                    v3018 = v1;
                    v3020 = v3015;
                    v3056 = v3007;
                    v3079 = v3014;
                    v3212 = v0;
                    v3324 = v0;
                    v3335 = v0;
                    v3346 = v0;
                    v3353 = v3354;
                    v9621 = v9581;
                    v9622 = v9619;
                    v9623 = v9620;
                    v9624 = v11032;
                    v9625 = v11032;
                    v9626 = v9582;
                    loop {
                        let v3019 = if v3018 <= v18 { 1.0 } else { 0.0 };
                        if v3019 == 0.0 {
                            break;
                        }
                        let v3021 = v3020 - v1244;
                        let v3022 = v663 * v3021;
                        let v13306 = v10380 * v3021;
                        let v13309 = (Lanes([0.0, 0.0, v13306[0], 0.0, 0.0, 0.0])) + ((v9621 - (Lanes([v9454[0], v9454[1], v9454[2], 0.0, v9454[3], 0.0]))) * v663);
                        let v3024 = (-v3022).exp();
                        let v13311 = (v13309 * v10360) * v3024;
                        let v3026 = if v3021 < v3025 { 1.0 } else { 0.0 };
                        let v3217: f64;
                        let v3230: f64;
                        let v9627: Lanes<6>;
                        let v9628: Lanes<6>;
                        if v3026 != 0.0 {
                            let v3029 = ((v3024 + v3022) - v1).sqrt();
                            let v3030 = v1240 * v3029;
                            let v13351 = v9389 * v3029;
                            let v13354 = (Lanes([0.0, 0.0, v13351[0], 0.0, 0.0, 0.0])) + (((v13311 + v13309) * (v9353 / (v10405 * v3029))) * v1240);
                            let v3034 = (v211 * ((-v3024) + v1)) / v3030;
                            let v13359 = (((v13311 * v10360) * v211) - (v13354 * v3034)) / v3030;
                            v3217 = v3030;
                            v3230 = v3034;
                            v9627 = v13354;
                            v9628 = v13359;
                        } else {
                            let v3035 = if v3021 > v616 { 1.0 } else { 0.0 };
                            let v3218: f64;
                            let v3231: f64;
                            let v9629: Lanes<6>;
                            let v9630: Lanes<6>;
                            if v3035 != 0.0 {
                                let v3036 = v3022.exp();
                                let v13321 = v13309 * v3036;
                                let v3037 = -v1240;
                                let v3041 = (v3036 + v3022) - v1;
                                let v13325 = v9390 * v3041;
                                let v3044 = (((v3024 + v3022) - v1) + (v1262 * v3041)).sqrt();
                                let v3045 = v3037 * v3044;
                                let v13333 = (v9389 * v10360) * v3044;
                                let v13336 = (Lanes([0.0, 0.0, v13333[0], 0.0, 0.0, 0.0])) + ((((v13311 + v13309) + ((Lanes([0.0, 0.0, v13325[0], 0.0, 0.0, 0.0])) + ((v13321 + v13309) * v1262))) * (v9353 / (v10405 * v3044))) * v3037);
                                let v3048 = v3036 + v1;
                                let v13338 = v9390 * v3048;
                                let v3052 = (v211 * (((-v3024) + v1) + (v1262 * v3048))) / v3045;
                                let v13346 = ((((v13311 * v10360) + ((Lanes([0.0, 0.0, v13338[0], 0.0, 0.0, 0.0])) + (v13321 * v1262))) * v211) - (v13336 * v3052)) / v3045;
                                v3218 = v3045;
                                v3231 = v3052;
                                v9629 = v13336;
                                v9630 = v13346;
                            } else {
                                let v3053 = -v1240;
                                let v13312 = v9389 * v10360;
                                let v3054 = v3053 * v3022;
                                let v13313 = v13312 * v3022;
                                let v13316 = (Lanes([0.0, 0.0, v13313[0], 0.0, 0.0, 0.0])) + (v13309 * v3053);
                                let v3055 = v3053 * v663;
                                let v13319 = (v13312 * v663) + (v10380 * v3053);
                                let v13320 = Lanes([0.0, 0.0, v13319[0], 0.0, 0.0, 0.0]);
                                v3218 = v3054;
                                v3231 = v3055;
                                v9629 = v13316;
                                v9630 = v13320;
                            }
                            v3217 = v3218;
                            v3230 = v3231;
                            v9627 = v9629;
                            v9628 = v9630;
                        }
                        let v3057 = v3056 - v2671;
                        let v13362 = v10380 * v3057;
                        let v3059 = (v663 * v3057).exp();
                        let v13366 = ((Lanes([0.0, 0.0, v13362[0], 0.0, 0.0, 0.0])) + ((v9622 - (Lanes([v12863[0], v12863[1], v12863[2], v12863[3], v12863[4], 0.0]))) * v663)) * v3059;
                        let v13367 = v12048 * v1504;
                        let v3061 = v750 * v750;
                        let v13369 = v10455 * v750;
                        let v3062 = (v1504 * v1504) / v3061;
                        let v13371 = (v13369 + v13369) * v3062;
                        let v13374 = ((v13367 + v13367) - (Lanes([0.0, 0.0, v13371[0], 0.0, 0.0]))) / v3061;
                        let v3063 = v78 * v759;
                        let v3065 = (v3059 + v3022) - v1;
                        let v13377 = (v10466 * v78) * v3065;
                        let v3068 = (v3062 + (v3063 * v3065)).sqrt();
                        let v13385 = ((Lanes([v13374[0], v13374[1], v13374[2], v13374[3], v13374[4], 0.0])) + ((Lanes([0.0, 0.0, v13377[0], 0.0, 0.0, 0.0])) + ((v13366 + v13309) * v3063))) * (v9353 / (v10405 * v3068));
                        let v3069 = v78 * v663;
                        let v3070 = v3069 * v759;
                        let v3071 = v3059 + v1;
                        let v13390 = (((v10380 * v78) * v759) + (v10466 * v3069)) * v3071;
                        let v3073 = v78 * v3068;
                        let v3074 = (v3070 * v3071) / v3073;
                        let v3075 = -v750;
                        let v13398 = v10455 * v10360;
                        let v13399 = v13398 * v3068;
                        let v3077 = (v3075 * v3068) - v1504;
                        let v13403 = Lanes([v12048[0], v12048[1], v12048[2], v12048[3], v12048[4], 0.0]);
                        let v13404 = ((Lanes([0.0, 0.0, v13399[0], 0.0, 0.0, 0.0])) + (v13385 * v3075)) - v13403;
                        let v3078 = v3075 * v3074;
                        let v13405 = v13398 * v3074;
                        let v13408 = (Lanes([0.0, 0.0, v13405[0], 0.0, 0.0, 0.0])) + (((((Lanes([0.0, 0.0, v13390[0], 0.0, 0.0, 0.0])) + (v13366 * v3070)) - ((v13385 * v78) * v3074)) / v3073) * v3075);
                        let v3081 = (v3079 - v3056) / v1208;
                        let v3082 = v663 * v3081;
                        let v13411 = v10380 * v3081;
                        let v13414 = (Lanes([0.0, 0.0, v13411[0], 0.0, 0.0, 0.0])) + (((v9623 - v9622) / v1208) * v663);
                        let v3083 = -v3082;
                        let v13415 = v13414 * v10360;
                        let v3084 = if v3083 >= v2326 { 1.0 } else { 0.0 };
                        let v3095: f64;
                        let v3103: f64;
                        let v9631: Lanes<6>;
                        let v9632: Lanes<6>;
                        if v3084 != 0.0 {
                            let v3087 = v2328 * ((v1 + v3083) - v2326);
                            let v13418 = v13415 * v2328;
                            v3095 = v3087;
                            v3103 = v2328;
                            v9631 = v13418;
                            v9632 = v11032;
                        } else {
                            let mut v3088: f64 = 0.0;
                            let mut v3090: f64 = 0.0;
                            let mut v9633: Lanes<6> = Lanes([0.0; 6]);
                            v3088 = v3083;
                            v3090 = v1;
                            v9633 = v13415;
                            loop {
                                let v3089 = if v3088 >= v2330 { 1.0 } else { 0.0 };
                                if v3089 == 0.0 {
                                    break;
                                }
                                let v3091 = v3090 * v2333;
                                let v3092 = v3088 - v2330;
                                let edge0 = v3092;
                                let edge1 = v3091;
                                let edge2 = v9633;
                                v3088 = edge0;
                                v3090 = edge1;
                                v9633 = edge2;
                            }
                            let v3093 = v3088.exp();
                            let v3094 = v3090 * v3093;
                            let v13417 = (v9633 * v3093) * v3090;
                            v3095 = v3094;
                            v3103 = v3094;
                            v9631 = v13417;
                            v9632 = v13417;
                        }
                        let v3098 = ((v3095 + v3082) - v1).sqrt();
                        let v13422 = (v9631 + v13414) * (v9353 / (v10405 * v3098));
                        let v3100 = if v3081 < v3099 { 1.0 } else { 0.0 };
                        let v3126: f64;
                        let v3163: f64;
                        let v3167: f64;
                        let v9634: Lanes<6>;
                        let v9635: Lanes<6>;
                        let v9636: Lanes<6>;
                        if v3100 != 0.0 {
                            let v3101 = v750 * v3098;
                            let v13453 = v10455 * v3098;
                            let v13456 = (Lanes([0.0, 0.0, v13453[0], 0.0, 0.0, 0.0])) + (v13422 * v750);
                            let v3102 = v750 * v663;
                            let v3105 = (-v3103) + v1;
                            let v13461 = ((v10455 * v663) + (v10380 * v750)) * v3105;
                            let v3107 = v78 * v3098;
                            let v3108 = (v3102 * v3105) / v3107;
                            let v3109 = v3108 / v1208;
                            let v13469 = ((((Lanes([0.0, 0.0, v13461[0], 0.0, 0.0, 0.0])) + ((v9632 * v10360) * v3102)) - ((v13422 * v78) * v3108)) / v3107) / v1208;
                            let v3110 = -v3109;
                            let v13470 = v13469 * v10360;
                            v3126 = v3101;
                            v3163 = v3109;
                            v3167 = v3110;
                            v9634 = v13456;
                            v9635 = v13469;
                            v9636 = v13470;
                        } else {
                            let v3111 = if v3081 > v616 { 1.0 } else { 0.0 };
                            let v3127: f64;
                            let v3164: f64;
                            let v3168: f64;
                            let v9637: Lanes<6>;
                            let v9638: Lanes<6>;
                            let v9639: Lanes<6>;
                            if v3111 != 0.0 {
                                let v3112 = v3075 * v3098;
                                let v13435 = v13398 * v3098;
                                let v13438 = (Lanes([0.0, 0.0, v13435[0], 0.0, 0.0, 0.0])) + (v13422 * v3075);
                                let v3113 = v3075 * v663;
                                let v3115 = (-v3103) + v1;
                                let v13443 = ((v13398 * v663) + (v10380 * v3075)) * v3115;
                                let v3117 = v78 * v3098;
                                let v3118 = (v3113 * v3115) / v3117;
                                let v3119 = v3118 / v1208;
                                let v13451 = ((((Lanes([0.0, 0.0, v13443[0], 0.0, 0.0, 0.0])) + ((v9632 * v10360) * v3113)) - ((v13422 * v78) * v3118)) / v3117) / v1208;
                                let v3120 = -v3119;
                                let v13452 = v13451 * v10360;
                                v3127 = v3112;
                                v3164 = v3119;
                                v3168 = v3120;
                                v9637 = v13438;
                                v9638 = v13451;
                                v9639 = v13452;
                            } else {
                                let v13423 = v13398 * v3082;
                                let v3122 = (v3075 * v3082) / v748;
                                let v13427 = ((Lanes([0.0, 0.0, v13423[0], 0.0, 0.0, 0.0])) + (v13414 * v3075)) / v748;
                                let v3124 = (v3075 * v663) / v748;
                                let v13431 = ((v13398 * v663) + (v10380 * v3075)) / v748;
                                let v3125 = -v3124;
                                let v13432 = v13431 * v10360;
                                let v13433 = Lanes([0.0, 0.0, v13431[0], 0.0, 0.0, 0.0]);
                                let v13434 = Lanes([0.0, 0.0, v13432[0], 0.0, 0.0, 0.0]);
                                v3127 = v3122;
                                v3164 = v3124;
                                v3168 = v3125;
                                v9637 = v13427;
                                v9638 = v13433;
                                v9639 = v13434;
                            }
                            v3126 = v3127;
                            v3163 = v3164;
                            v3167 = v3168;
                            v9634 = v9637;
                            v9635 = v9638;
                            v9636 = v9639;
                        }
                        let v3128 = -v1225;
                        let v13471 = v11922 * v10360;
                        let v3129 = v0 - v3128;
                        let v13472 = v13471 * v10360;
                        let v3132 = if (if v3126 > v3129 { 1.0 } else { 0.0 }) != 0.0 && (if v3128 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3165: f64;
                        let v3170: f64;
                        let v9640: Lanes<6>;
                        let v9641: Lanes<6>;
                        if v3132 != 0.0 {
                            let v3133 = v3126 + v3128;
                            let v13474 = v9634 + (Lanes([v13471[0], v13471[1], v13471[2], v13471[3], v13471[4], 0.0]));
                            let v3134 = v3133 * v3133;
                            let v13475 = v13474 * v3133;
                            let v3135 = v3128 * v3128;
                            let v13477 = v13471 * v3128;
                            let v13479 = (v13475 + v13475) * v3134;
                            let v3137 = v3135 * v3135;
                            let v13481 = (v13477 + v13477) * v3135;
                            let v13482 = v13481 + v13481;
                            let v3138 = (v3134 * v3134) + v3137;
                            let v13484 = (v13479 + v13479) + (Lanes([v13482[0], v13482[1], v13482[2], v13482[3], v13482[4], 0.0]));
                            let v3155: f64;
                            let v9642: Lanes<6>;
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
                                let mut v9643: Lanes<6> = Lanes([0.0; 6]);
                                v3144 = v0;
                                v3146 = v3138;
                                v9643 = v13484;
                                loop {
                                    let v3145 = if v3144 < v3149 { 1.0 } else { 0.0 };
                                    if v3145 == 0.0 {
                                        break;
                                    }
                                    let v3147 = v3146.sqrt();
                                    let v13706 = v9643 * (v9353 / (v10405 * v3147));
                                    let v3148 = v3144 + v1;
                                    v3144 = v3148;
                                    v3146 = v3147;
                                    v9643 = v13706;
                                }
                                v3155 = v3146;
                                v9642 = v9643;
                            } else {
                                let v3154 = v3138.powf(v3153);
                                let v13488 = v13484 * (v3153 * (v3138.powf(v13485)));
                                v3155 = v3154;
                                v9642 = v13488;
                            }
                            let v3156 = v1 / v3155;
                            let v13491 = ((v9642 * v3156) * v10360) / v3155;
                            let v3157 = v3133 * v3128;
                            let v13493 = v13471 * v3133;
                            let v3159 = v3128 * v3137;
                            let v13502 = ((v13471 * v3137) + (v13482 * v3128)) * v3156;
                            let v3161 = (v3159 * v3156) / v3138;
                            let v13508 = (((Lanes([v13502[0], v13502[1], v13502[2], v13502[3], v13502[4], 0.0])) + (v13491 * v3159)) - (v13484 * v3161)) / v3138;
                            let v3162 = v3129 + (v3157 * v3156);
                            let v13510 = (Lanes([v13472[0], v13472[1], v13472[2], v13472[3], v13472[4], 0.0])) + ((((v13474 * v3128) + (Lanes([v13493[0], v13493[1], v13493[2], v13493[3], v13493[4], 0.0]))) * v3156) + (v13491 * v3157));
                            v3165 = v3161;
                            v3170 = v3162;
                            v9640 = v13508;
                            v9641 = v13510;
                        } else {
                            v3165 = v1;
                            v3170 = v3126;
                            v9640 = v11032;
                            v9641 = v9634;
                        }
                        let v3166 = v3163 * v3165;
                        let v13513 = (v9635 * v3165) + (v9640 * v3163);
                        let v3169 = v3167 * v3165;
                        let v13516 = (v9636 * v3165) + (v9640 * v3167);
                        let v3171 = v1228 - v1504;
                        let v13517 = v12048 * v10360;
                        let v3172 = -v3171;
                        let v13518 = v13517 * v10360;
                        let v3173 = v3171 + v3172;
                        let v13519 = v13517 + v13518;
                        let v3176 = if (if v3170 < v3173 { 1.0 } else { 0.0 }) != 0.0 && (if v3172 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3207: f64;
                        let v3210: f64;
                        let v9644: Lanes<6>;
                        let v9645: Lanes<6>;
                        if v3176 != 0.0 {
                            let v3177 = v3173 - v3170;
                            let v13520 = Lanes([v13519[0], v13519[1], v13519[2], v13519[3], v13519[4], 0.0]);
                            let v13521 = v13520 - v9641;
                            let v3178 = v3177 * v3177;
                            let v13522 = v13521 * v3177;
                            let v3179 = v3172 * v3172;
                            let v13524 = v13518 * v3172;
                            let v13526 = (v13522 + v13522) * v3178;
                            let v3181 = v3179 * v3179;
                            let v13528 = (v13524 + v13524) * v3179;
                            let v13529 = v13528 + v13528;
                            let v3182 = (v3178 * v3178) + v3181;
                            let v13531 = (v13526 + v13526) + (Lanes([v13529[0], v13529[1], v13529[2], v13529[3], v13529[4], 0.0]));
                            let v3199: f64;
                            let v9646: Lanes<6>;
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
                                let mut v9647: Lanes<6> = Lanes([0.0; 6]);
                                v3188 = v0;
                                v3190 = v3182;
                                v9647 = v13531;
                                loop {
                                    let v3189 = if v3188 < v3193 { 1.0 } else { 0.0 };
                                    if v3189 == 0.0 {
                                        break;
                                    }
                                    let v3191 = v3190.sqrt();
                                    let v13703 = v9647 * (v9353 / (v10405 * v3191));
                                    let v3192 = v3188 + v1;
                                    v3188 = v3192;
                                    v3190 = v3191;
                                    v9647 = v13703;
                                }
                                v3199 = v3190;
                                v9646 = v9647;
                            } else {
                                let v3198 = v3182.powf(v3197);
                                let v13535 = v13531 * (v3197 * (v3182.powf(v13532)));
                                v3199 = v3198;
                                v9646 = v13535;
                            }
                            let v3200 = v1 / v3199;
                            let v13538 = ((v9646 * v3200) * v10360) / v3199;
                            let v3201 = v3177 * v3172;
                            let v13540 = v13518 * v3177;
                            let v3203 = v3172 * v3181;
                            let v13549 = ((v13518 * v3181) + (v13529 * v3172)) * v3200;
                            let v3205 = (v3203 * v3200) / v3182;
                            let v13555 = (((Lanes([v13549[0], v13549[1], v13549[2], v13549[3], v13549[4], 0.0])) + (v13538 * v3203)) - (v13531 * v3205)) / v3182;
                            let v3206 = v3173 - (v3201 * v3200);
                            let v13556 = v13520 - ((((v13521 * v3172) + (Lanes([v13540[0], v13540[1], v13540[2], v13540[3], v13540[4], 0.0]))) * v3200) + (v13538 * v3201));
                            v3207 = v3205;
                            v3210 = v3206;
                            v9644 = v13555;
                            v9645 = v13556;
                        } else {
                            v3207 = v1;
                            v3210 = v3170;
                            v9644 = v11032;
                            v9645 = v9641;
                        }
                        let v3208 = v3169 * v3207;
                        let v13559 = (v13516 * v3207) + (v9644 * v3169);
                        let v3209 = v3166 * v3207;
                        let v13562 = (v13513 * v3207) + (v9644 * v3166);
                        let v3211 = v1504 + v3210;
                        let v13563 = v13403 + v9645;
                        let v3215 = if (if v3212 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v3018 > v96 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v3317: f64;
                        let v3319: f64;
                        let v3320: f64;
                        let v3321: f64;
                        let v3322: f64;
                        let v3325: f64;
                        let v9648: Lanes<6>;
                        let v9649: Lanes<6>;
                        let v9650: Lanes<6>;
                        if v3215 != 0.0 {
                            v3317 = v18;
                            v3319 = v3020;
                            v3320 = v3056;
                            v3321 = v3079;
                            v3322 = v3212;
                            v3325 = v3018;
                            v9648 = v9621;
                            v9649 = v9622;
                            v9650 = v9623;
                        } else {
                            let v3222 = (((v3217 + v1504) + v3077) + v3210) + v2569;
                            let v13570 = v9405 * v3222;
                            let v3224 = (v3056 - v1200) - (v1048 * v3222);
                            let v13574 = (v9622 - (Lanes([v10798[0], v10798[1], v10798[2], v10798[3], v10798[4], 0.0]))) - ((Lanes([v13570[0], v13570[1], 0.0, v13570[2], v13570[3], 0.0])) + (((((v9627 + v13403) + v13404) + v9645) + v9488) * v1048));
                            let v3225 = v3078 + v3208;
                            let v13576 = v9405 * v3225;
                            let v3227 = v1 - (v1048 * v3225);
                            let v13580 = ((Lanes([v13576[0], v13576[1], 0.0, v13576[2], v13576[3], 0.0])) + ((v13408 + v13559) * v1048)) * v10360;
                            let v3228 = -v1048;
                            let v13581 = v9405 * v10360;
                            let v3229 = v3228 * v3209;
                            let v13582 = v13581 * v3209;
                            let v13585 = (Lanes([v13582[0], v13582[1], 0.0, v13582[2], v13582[3], 0.0])) + (v13562 * v3228);
                            let v3232 = v3228 * v3230;
                            let v13586 = v13581 * v3230;
                            let v13589 = (Lanes([v13586[0], v13586[1], 0.0, v13586[2], v13586[3], 0.0])) + (v9628 * v3228);
                            let v3238 = v3079 - (v3056 + (v125 * ((v13 * v1228) + v3217)));
                            let v13593 = v9623 - (v9622 + (v9627 * v125));
                            let v3240 = -(v125 * v3230);
                            let v13594 = (v9628 * v125) * v10360;
                            let v3243 = (v3020 - v3079) - (v131 * v3217);
                            let v13597 = (v9621 - v9623) - (v9627 * v131);
                            let v3246 = v1 - (v131 * v3230);
                            let v13599 = (v9628 * v131) * v10360;
                            let v3247 = v3227 * v3246;
                            let v13602 = (v13580 * v3246) + (v13599 * v3227);
                            let v3248 = v3227 * v3240;
                            let v13605 = (v13580 * v3240) + (v13594 * v3227);
                            let v3251 = v3229 * v3239;
                            let v13608 = v13585 * v3239;
                            let v3254 = v3232 * v3239;
                            let v13613 = v13589 * v3239;
                            let v3257 = (((v3247 - (v3248 * v3244)) - (v3251 * v3246)) + (v3254 * v3244)) + v362;
                            let v3258 = v1 / v3257;
                            let v3260 = v3246 - (v3240 * v3244);
                            let v3263 = (v3232 * v3244) - (v3229 * v3246);
                            let v3265 = (v3229 * v3240) - v3232;
                            let v3266 = v3254 - v3248;
                            let v3268 = (-v3227) * v3244;
                            let v3269 = v3227 - v3251;
                            let v3270 = -v3258;
                            let v13634 = ((((((v13602 - (v13605 * v3244)) - ((v13608 * v3246) + (v13599 * v3251))) + (v13613 * v3244)) * v3258) * v10360) / v3257) * v10360;
                            let v3275 = ((v3260 * v3224) + (v3263 * v3238)) + (v3265 * v3243);
                            let v3276 = v3270 * v3275;
                            let v13648 = (v13634 * v3275) + ((((((v13599 - (v13594 * v3244)) * v3224) + (v13574 * v3260)) + ((((v13589 * v3244) - ((v13585 * v3246) + (v13599 * v3229))) * v3238) + (v13593 * v3263))) + (((((v13585 * v3240) + (v13594 * v3229)) - v13589) * v3243) + (v13597 * v3265))) * v3270);
                            let v3281 = ((v3246 * v3224) + (v3247 * v3238)) + (v3266 * v3243);
                            let v3282 = v3270 * v3281;
                            let v13662 = (v13634 * v3281) + (((((v13599 * v3224) + (v13574 * v3246)) + ((v13602 * v3238) + (v13593 * v3247))) + (((v13613 - v13605) * v3243) + (v13597 * v3266))) * v3270);
                            let v3286 = (v3224 + (v3268 * v3238)) + (v3269 * v3243);
                            let v3287 = v3270 * v3286;
                            let v13673 = (v13634 * v3286) + (((v13574 + ((((v13580 * v10360) * v3244) * v3238) + (v13593 * v3268))) + (((v13580 - v13608) * v3243) + (v13597 * v3269))) * v3270);
                            let v3288 = v3276.abs();
                            let v13677 = v13648 * ((v10405 * (if v3276 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                            let v3289 = v3282.abs();
                            let v13681 = v13662 * ((v10405 * (if v3282 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                            let v3290 = if v3288 < v3289 { 1.0 } else { 0.0 };
                            let v3291: f64;
                            let v9651: Lanes<6>;
                            if v3290 != 0.0 {
                                v3291 = v3289;
                                v9651 = v13681;
                            } else {
                                v3291 = v3288;
                                v9651 = v13677;
                            }
                            let v3292 = v3287.abs();
                            let v13685 = v13673 * ((v10405 * (if v3287 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                            let v3293 = if v3291 < v3292 { 1.0 } else { 0.0 };
                            let v3298: f64;
                            let v9652: Lanes<6>;
                            if v3293 != 0.0 {
                                v3298 = v3292;
                                v9652 = v13685;
                            } else {
                                v3298 = v3291;
                                v9652 = v9651;
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
                            let v9653: Lanes<6>;
                            let v9654: Lanes<6>;
                            let v9655: Lanes<6>;
                            if v3304 != 0.0 {
                                let v3305 = v3303 / v3298;
                                let v13688 = ((v9652 * v3305) * v10360) / v3298;
                                let v3306 = v3276 * v3305;
                                let v13691 = (v13648 * v3305) + (v13688 * v3276);
                                let v3307 = v3282 * v3305;
                                let v13694 = (v13662 * v3305) + (v13688 * v3282);
                                let v3308 = v3287 * v3305;
                                let v13697 = (v13673 * v3305) + (v13688 * v3287);
                                v3309 = v3306;
                                v3311 = v3307;
                                v3313 = v3308;
                                v9653 = v13691;
                                v9654 = v13694;
                                v9655 = v13697;
                            } else {
                                v3309 = v3276;
                                v3311 = v3282;
                                v3313 = v3287;
                                v9653 = v13648;
                                v9654 = v13662;
                                v9655 = v13673;
                            }
                            let v3310 = v3056 + v3309;
                            let v13698 = v9622 + v9653;
                            let v3312 = v3079 + v3311;
                            let v13699 = v9623 + v9654;
                            let v3314 = v3020 + v3313;
                            let v13700 = v9621 + v9655;
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
                            v9648 = v13700;
                            v9649 = v13698;
                            v9650 = v13699;
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
                        v9621 = v9648;
                        v9622 = v9649;
                        v9623 = v9650;
                        v9624 = v13404;
                        v9625 = v13563;
                        v9626 = v9627;
                    }
                    let v3326 = if v3324 > v0 { 1.0 } else { 0.0 };
                    if v3326 != 0.0 {
                    } else {
                    }
                    let v3327 = if v3212 == v0 { 1.0 } else { 0.0 };
                    let v3328: f64;
                    let v5723: f64;
                    let v9656: Lanes<6>;
                    let v9657: Lanes<6>;
                    if v3327 != 0.0 {
                        v3328 = v3007;
                        v5723 = v3014;
                        v9656 = v9619;
                        v9657 = v9620;
                    } else {
                        v3328 = v3056;
                        v5723 = v3079;
                        v9656 = v9622;
                        v9657 = v9623;
                    }
                    let v4327: f64;
                    if v3002 != 0.0 {
                        v4327 = v1;
                    } else {
                        v4327 = v0;
                    }
                    let v3329 = v3328 - v2577;
                    let v13221 = v9656 - v9555;
                    let v3334 = v3330 / v123;
                    let v3336 = v3335 - v2578;
                    let v13222 = v9624 - v9523;
                    let v3337 = v3335 + v2578;
                    let v13223 = v9624 + v9523;
                    let v3338 = v663 * v3337;
                    let v13224 = v10380 * v3337;
                    let v3341 = v3336 - ((v3338 * v3329) * v13);
                    let v13232 = v13222 - (((((Lanes([0.0, 0.0, v13224[0], 0.0, 0.0, 0.0])) + (v13223 * v663)) * v3329) + (v13221 * v3338)) * v13);
                    let v3344 = if (if v3341 < v0 { 1.0 } else { 0.0 }) != 0.0 || (if v823 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4383: f64;
                    let v9658: Lanes<6>;
                    if v3344 != 0.0 {
                        v4383 = v0;
                        v9658 = v11032;
                    } else {
                        v4383 = v3341;
                        v9658 = v13232;
                    }
                    let v3348 = v3345 * (v3346 + v2592);
                    let v13234 = (v9625 + v9525) * v3345;
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
                    let v13235 = v13222 * v10360;
                    let v3383 = if (if v3381 < v3362 { 1.0 } else { 0.0 }) != 0.0 && v3364 != 0.0 { 1.0 } else { 0.0 };
                    let v3411: f64;
                    let v9659: Lanes<6>;
                    if v3383 != 0.0 {
                        let v3384 = v3362 - v3381;
                        let v13236 = v13235 * v10360;
                        let v3385 = v3384 * v3384;
                        let v13237 = v13236 * v3384;
                        let v3386 = v3362 * v3362;
                        let v13239 = (v13237 + v13237) * v3385;
                        let v13240 = v13239 + v13239;
                        let v3389 = (v3385 * v3385) + (v3386 * v3386);
                        let v3406: f64;
                        let v9660: Lanes<6>;
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
                            let mut v9661: Lanes<6> = Lanes([0.0; 6]);
                            v3395 = v0;
                            v3397 = v3389;
                            v9661 = v13240;
                            loop {
                                let v3396 = if v3395 < v3400 { 1.0 } else { 0.0 };
                                if v3396 == 0.0 {
                                    break;
                                }
                                let v3398 = v3397.sqrt();
                                let v13303 = v9661 * (v9353 / (v10405 * v3398));
                                let v3399 = v3395 + v1;
                                v3395 = v3399;
                                v3397 = v3398;
                                v9661 = v13303;
                            }
                            v3406 = v3397;
                            v9660 = v9661;
                        } else {
                            let v3405 = v3389.powf(v3404);
                            let v13244 = v13240 * (v3404 * (v3389.powf(v13241)));
                            v3406 = v3405;
                            v9660 = v13244;
                        }
                        let v3407 = v1 / v3406;
                        let v3408 = v3384 * v3362;
                        let v3410 = v3362 - (v3408 * v3407);
                        let v13252 = (((v13236 * v3362) * v3407) + ((((v9660 * v3407) * v10360) / v3406) * v3408)) * v10360;
                        v3411 = v3410;
                        v9659 = v13252;
                    } else {
                        v3411 = v3381;
                        v9659 = v13235;
                    }
                    let v3414 = v663 * v1128;
                    let v13255 = v10380 * v1128;
                    let v13256 = v9406 * v663;
                    let v3415 = v3414 * v3349;
                    let v13260 = ((Lanes([0.0, 0.0, v13255[0], 0.0, 0.0])) + (Lanes([v13256[0], v13256[1], 0.0, v13256[2], v13256[3]]))) * v3349;
                    let v3416 = v3415 * v3349;
                    let v3417 = (v78 * (-v3411)) / v3416;
                    let v3418 = v1 + v3417;
                    let v3420 = (v3418 * v3349) / v2582;
                    let v3421 = v1 - v3420;
                    let v13276 = ((((((((v9659 * v10360) * v78) - (((((Lanes([v13260[0], v13260[1], v13260[2], v13260[3], v13260[4], 0.0])) + (v13221 * v3414)) * v3349) + (v13221 * v3415)) * v3417)) / v3416) * v3349) + (v13221 * v3418)) - (v12776 * v3420)) / v2582) * v10360;
                    let v3425 = if (if v3421 < v3422 { 1.0 } else { 0.0 }) != 0.0 && v3424 != 0.0 { 1.0 } else { 0.0 };
                    let v3454: f64;
                    let v9662: Lanes<6>;
                    if v3425 != 0.0 {
                        let v3427 = v3426 - v3421;
                        let v13277 = v13276 * v10360;
                        let v3428 = v3427 * v3427;
                        let v13278 = v13277 * v3427;
                        let v13280 = (v13278 + v13278) * v3428;
                        let v13281 = v13280 + v13280;
                        let v3431 = (v3428 * v3428) + v3430;
                        let v3448: f64;
                        let v9663: Lanes<6>;
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
                            let mut v9664: Lanes<6> = Lanes([0.0; 6]);
                            v3437 = v0;
                            v3439 = v3431;
                            v9664 = v13281;
                            loop {
                                let v3438 = if v3437 < v3442 { 1.0 } else { 0.0 };
                                if v3438 == 0.0 {
                                    break;
                                }
                                let v3440 = v3439.sqrt();
                                let v13300 = v9664 * (v9353 / (v10405 * v3440));
                                let v3441 = v3437 + v1;
                                v3437 = v3441;
                                v3439 = v3440;
                                v9664 = v13300;
                            }
                            v3448 = v3439;
                            v9663 = v9664;
                        } else {
                            let v3447 = v3431.powf(v3446);
                            let v13285 = v13281 * (v3446 * (v3431.powf(v13282)));
                            v3448 = v3447;
                            v9663 = v13285;
                        }
                        let v3449 = v1 / v3448;
                        let v3450 = v3427 * v1231;
                        let v3453 = v3452 - (v3450 * v3449);
                        let v13293 = (((v13277 * v1231) * v3449) + ((((v9663 * v3449) * v10360) / v3448) * v3450)) * v10360;
                        v3454 = v3453;
                        v9662 = v13293;
                    } else {
                        v3454 = v3421;
                        v9662 = v13276;
                    }
                    let v3455 = v1 + v3454;
                    let v13296 = (v9662 * v3455) + (v9662 * v3454);
                    let v3457 = v1 + (v3454 * v3455);
                    let v3459 = if v3455 >= v3458 { 1.0 } else { 0.0 };
                    let v3461: f64;
                    let v9665: Lanes<6>;
                    if v3459 != 0.0 {
                        v3461 = v3455;
                        v9665 = v9662;
                    } else {
                        v3461 = v3460;
                        v9665 = v11032;
                    }
                    let v3463 = v3462 * v3337;
                    let v13297 = v13223 * v3462;
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
                    v9559 = v9662;
                    v9560 = v9665;
                    v9561 = v13296;
                    v9562 = v9656;
                    v9563 = v9658;
                    v9564 = v13234;
                    v9565 = v13297;
                    v9566 = v9626;
                    v9567 = v13221;
                    v9568 = v12776;
                    v9569 = v9657;
                    v9570 = v11032;
                    v9571 = v11032;
                    v9572 = v11032;
                    v9573 = v11032;
                    v9574 = v11032;
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
                v9422 = v9559;
                v9423 = v9560;
                v9424 = v9561;
                v9425 = v9562;
                v9426 = v9555;
                v9427 = v9558;
                v9428 = v9563;
                v9429 = v9564;
                v9430 = v9565;
                v9431 = v9526;
                v9432 = v9566;
                v9433 = v9567;
                v9434 = v9556;
                v9435 = v9490;
                v9436 = v9491;
                v9437 = v9568;
                v9438 = v9455;
                v9439 = v9454;
                v9440 = v12048;
                v9441 = v9468;
                v9442 = v9488;
                v9443 = v9492;
                v9444 = v9569;
                v9445 = v9570;
                v9446 = v9571;
                v9447 = v9572;
                v9448 = v9573;
                v9449 = v9574;
                v9450 = v11032;
                v9451 = v11032;
                v9452 = v9493;
            } else {
                let v3479 = if v769 < v12 { 1.0 } else { 0.0 };
                let v4189: f64;
                if v3479 != 0.0 {
                    v4189 = v1;
                } else {
                    v4189 = v78;
                }
                let v10807 = Lanes([v9398[0], v9398[1], 0.0, 0.0, v9398[2]]);
                let v3481 = if v830 < (v1205 + v835) { 1.0 } else { 0.0 };
                let v3636: f64;
                let v3834: f64;
                let v3943: f64;
                let v5095: f64;
                let v9666: Lanes<5>;
                let v9667: Lanes<5>;
                let v9668: Lanes<5>;
                if v3481 != 0.0 {
                    let v3483 = v78 * v665;
                    let v3485 = (-v367) / v1206;
                    let v3486 = v3485.ln();
                    let v3487 = v3483 * v3486;
                    let v10923 = (v10385 * v78) * v3486;
                    let v10926 = (Lanes([0.0, 0.0, v10923[0], 0.0, 0.0])) + (((((v10804 * v3485) * v10360) / v1206) * (v9353 / v3485)) * v3483);
                    let v3488 = v1200 - v835;
                    let v10928 = v10380 * v3488;
                    let v3490 = v663 * v750;
                    let v3491 = v1 / v3490;
                    let v3492 = v3491 * v1128;
                    let v10938 = (((((v10380 * v750) + (v10455 * v663)) * v3491) * v10360) / v3490) * v1128;
                    let v10939 = v9406 * v3491;
                    let v10942 = (Lanes([0.0, 0.0, v10938[0], 0.0, 0.0])) + (Lanes([v10939[0], v10939[1], 0.0, v10939[2], v10939[3]]));
                    let v10943 = v10942 * v3493;
                    let v3495 = v78 + (v3493 * v3492);
                    let v3496 = v91 * v3495;
                    let v3497 = v3496 * v3495;
                    let v3498 = v3497 * v3495;
                    let v10950 = ((((v10943 * v91) * v3495) + (v10943 * v3496)) * v3495) + (v10943 * v3497);
                    let v3499 = (v663 * v3488) - v78;
                    let v3501 = v3500 * v3492;
                    let v3502 = v3501 * v3499;
                    let v10954 = ((v10942 * v3500) * v3499) + (((Lanes([0.0, 0.0, v10928[0], 0.0, 0.0])) + ((v10798 - v10807) * v663)) * v3501);
                    let v3504 = v3503 - v3502;
                    let v10955 = v10954 * v10360;
                    let v3505 = v3504 * v3504;
                    let v10956 = v10955 * v3504;
                    let v10957 = v10956 + v10956;
                    let v3508 = if v3498 < (v3505 * v3506) { 1.0 } else { 0.0 };
                    let v3520: f64;
                    let v9669: Lanes<5>;
                    if v3508 != 0.0 {
                        let v3512 = (v13 * v3498) / v3504;
                        let v3514 = ((v3509 + v3504) + v3512) + v3502;
                        let v10968 = (v10955 + (((v10950 * v13) - (v10955 * v3512)) / v3504)) + v10954;
                        v3520 = v3514;
                        v9669 = v10968;
                    } else {
                        let v3516 = (v3498 + v3505).sqrt();
                        let v3519 = (v3517 + v3516) + v3502;
                        let v10962 = ((v10950 + v10957) * (v9353 / (v10405 * v3516))) + v10954;
                        v3520 = v3519;
                        v9669 = v10962;
                    }
                    let v3521 = v3520.powf(v1562);
                    let v10972 = v9669 * (v1562 * (v3520.powf(v10969)));
                    let v3528 = v748 * v3521;
                    let v3530 = ((v3522 - (v3523 * v3492)) + (v78 * v3521)) + (v3528 * v3521);
                    let v3531 = v1 / v3521;
                    let v3532 = v3530 * v3531;
                    let v10989 = v10385 * v3532;
                    let v3535 = ((v3532 * v665) + v835) - v835;
                    let v10993 = (((((((((v10942 * v3523) * v10360) + (v10972 * v78)) + (((v10972 * v748) * v3521) + (v10972 * v3528))) * v3531) + ((((v10972 * v3531) * v10360) / v3521) * v3530)) * v665) + (Lanes([0.0, 0.0, v10989[0], 0.0, 0.0]))) + v10807) - v10807;
                    let v3536 = v3535 / v3487;
                    let v10997 = ((v10993 - (v10926 * v3536)) / v3487) * v3536;
                    let v3539 = (v1 + (v3536 * v3536)).sqrt();
                    let v3540 = v3535 / v3539;
                    let v3541 = v3540 + v835;
                    let v11005 = ((v10993 - (((v10997 + v10997) * (v9353 / (v10405 * v3539))) * v3540)) / v3539) + v10807;
                    v3636 = v3541;
                    v3834 = v3482;
                    v3943 = v0;
                    v5095 = v0;
                    v9666 = v11005;
                    v9667 = v10549;
                    v9668 = v10549;
                } else {
                    let v3623: f64;
                    let v3625: f64;
                    let v9670: Lanes<5>;
                    let v9671: Lanes<5>;
                    if v3542 != 0.0 {
                        v3623 = v0;
                        v3625 = v0;
                        v9670 = v10549;
                        v9671 = v10549;
                    } else {
                        let v3543 = v1200 - v835;
                        let v3544 = v663 * v3543;
                        let v10809 = v10380 * v3543;
                        let v10812 = (Lanes([0.0, 0.0, v10809[0], 0.0, 0.0])) + ((v10798 - v10807) * v663);
                        let v3547 = v1207 * v664;
                        let v10815 = v10382 * v1207;
                        let v3548 = (v90 * (v3544 - v1)) / v3547;
                        let v10820 = ((v10812 * v90) - (((v10806 * v664) + (Lanes([0.0, 0.0, v10815[0], 0.0, 0.0]))) * v3548)) / v3547;
                        let v3549 = v1 + v3548;
                        let v3551 = if v3549 >= v3550 { 1.0 } else { 0.0 };
                        let v3553: f64;
                        let v9672: Lanes<5>;
                        if v3551 != 0.0 {
                            v3553 = v3549;
                            v9672 = v10820;
                        } else {
                            v3553 = v3552;
                            v9672 = v10549;
                        }
                        let v10822 = v10380 * v1207;
                        let v3555 = (v1207 * v663) * v13;
                        let v3556 = v3553.sqrt();
                        let v3557 = v1 - v3556;
                        let v3559 = v1200 + (v3555 * v3557);
                        let v10833 = v10798 + (((((v10806 * v663) + (Lanes([0.0, 0.0, v10822[0], 0.0, 0.0]))) * v13) * v3557) + (((v9672 * (v9353 / (v10405 * v3556))) * v10360) * v3555));
                        let v3562 = if (v663 * (v3559 - v835)) < v96 { 1.0 } else { 0.0 };
                        let v3620: f64;
                        let v3626: f64;
                        let v9673: Lanes<5>;
                        let v9674: Lanes<5>;
                        if v3562 != 0.0 {
                            let v3564 = v3563 * v663;
                            let v3565 = v3564 * v1206;
                            let v10871 = (v10380 * v3563) * v1206;
                            let v3566 = v1 / v3565;
                            let v10877 = ((((Lanes([0.0, 0.0, v10871[0], 0.0, 0.0])) + (v10804 * v3564)) * v3566) * v10360) / v3565;
                            let v10878 = v10877 * v96;
                            let v3568 = v1540 + (v96 * v3566);
                            let v3572 = v1153 * v3566;
                            let v3573 = v3572 * v3544;
                            let v10885 = ((v10877 * v1540) * v10360) + (((v10877 * v1153) * v3544) + (v10812 * v3572));
                            let v3578 = (v1549 - (v1540 * (v1550 + v3566))) + v3573;
                            let v10886 = v10885 * v3578;
                            let v3580 = v90 * v3568;
                            let v3581 = v3580 * v3568;
                            let v3584 = ((v3581 * v3568) + (v3578 * v3578)).sqrt();
                            let v3585 = ((v3569 - (v1540 * v3566)) + v3573) + v3584;
                            let v3586 = v3585.powf(v1562);
                            let v10903 = (v10885 + (((((((v10878 * v90) * v3568) + (v10878 * v3580)) * v3568) + (v10878 * v3581)) + (v10886 + v10886)) * (v9353 / (v10405 * v3584)))) * (v1562 * (v3585.powf(v10900)));
                            let v3588 = v96 * v3586;
                            let v3589 = (v1564 * v3568) / v3588;
                            let v3593 = (v96 - v3589) + (v3591 * v3586);
                            let v10913 = v10385 * v3593;
                            let v3595 = (v3593 * v665) + v835;
                            let v10916 = (((((((v10878 * v1564) - ((v10903 * v96) * v3589)) / v3588) * v10360) + (v10903 * v3591)) * v665) + (Lanes([0.0, 0.0, v10913[0], 0.0, 0.0]))) + v10807;
                            v3620 = v3595;
                            v3626 = v3595;
                            v9673 = v10916;
                            v9674 = v10916;
                        } else {
                            let v3596 = if v830 <= v1143 { 1.0 } else { 0.0 };
                            let v3621: f64;
                            let v9675: Lanes<5>;
                            if v3596 != 0.0 {
                                v3621 = v3559;
                                v9675 = v10833;
                            } else {
                                let v3597 = v1 / v759;
                                let v10836 = ((v10466 * v3597) * v10360) / v759;
                                let v3598 = v3597 / v1211;
                                let v3599 = v3598 * v1200;
                                let v3600 = v3599 * v1200;
                                let v3601 = v78 / v1200;
                                let v3602 = v663 + v3601;
                                let v3604 = (v3600.ln()) / v3602;
                                let v10856 = (((((((((Lanes([0.0, 0.0, v10836[0], 0.0, 0.0])) - (v9407 * v3598)) / v1211) * v1200) + (v10798 * v3598)) * v1200) + (v10798 * v3599)) * (v9353 / v3600)) - (((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + (((v10798 * v3601) * v10360) / v1200)) * v3604)) / v3602;
                                let v10857 = v10856 - v10833;
                                let v3606 = (v3604 - v3559) - v1270;
                                let v3608 = (v90 * v3604) * v1270;
                                let v10859 = (v10856 * v90) * v1270;
                                let v3609 = if v3608 > v0 { 1.0 } else { 0.0 };
                                let v3611: f64;
                                let v9676: Lanes<5>;
                                if v3609 != 0.0 {
                                    v3611 = v3608;
                                    v9676 = v10859;
                                } else {
                                    let v3610 = -v3608;
                                    let v10860 = v10859 * v10360;
                                    v3611 = v3610;
                                    v9676 = v10860;
                                }
                                let v10861 = v10857 * v3606;
                                let v3614 = ((v3606 * v3606) + v3611).sqrt();
                                let v3617 = v3604 - (v13 * (v3606 + v3614));
                                let v10869 = v10856 - ((v10857 + (((v10861 + v10861) + v9676) * (v9353 / (v10405 * v3614)))) * v13);
                                v3621 = v3617;
                                v9675 = v10869;
                            }
                            v3620 = v3621;
                            v3626 = v3559;
                            v9673 = v9675;
                            v9674 = v10833;
                        }
                        let v3619 = v835 + v3618;
                        let v3622 = if v3620 < v3619 { 1.0 } else { 0.0 };
                        let v3624: f64;
                        let v9677: Lanes<5>;
                        if v3622 != 0.0 {
                            v3624 = v3619;
                            v9677 = v10807;
                        } else {
                            v3624 = v3620;
                            v9677 = v9673;
                        }
                        v3623 = v3624;
                        v3625 = v3626;
                        v9670 = v9677;
                        v9671 = v9674;
                    }
                    v3636 = v3623;
                    v3834 = v0;
                    v3943 = v3625;
                    v5095 = v3623;
                    v9666 = v9670;
                    v9667 = v9671;
                    v9668 = v9670;
                }
                let v3629 = if (if v1886 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v2204 == v78 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v3632: f64;
                let v9678: Lanes<1>;
                if v3629 != 0.0 {
                    let v3631 = v3630 * v2254;
                    let v11007 = v9365 * v3630;
                    v3632 = v3631;
                    v9678 = v11007;
                } else {
                    v3632 = v0;
                    v9678 = v11006;
                }
                let v11008 = v10380 * v835;
                let v11009 = v9398 * v663;
                let v3634 = (v663 * v835).exp();
                let v11013 = ((Lanes([0.0, 0.0, v11008[0], 0.0])) + (Lanes([v11009[0], v11009[1], 0.0, v11009[2]]))) * v3634;
                let v3635 = v759 * v3634;
                let v11014 = v10466 * v3634;
                let v11017 = (Lanes([0.0, 0.0, v11014[0], 0.0])) + (v11013 * v759);
                let v3640 = (((v490 * v12) * v12) / v78) / v123;
                let v3643 = ((v78 * v663) * v3640).sqrt();
                let v11022 = ((v10380 * v78) * v3640) * (v9353 / (v10405 * v3643));
                let v3644 = v3643.exp();
                let v3646 = (-v3643).exp();
                let v3648 = (v3644 + v3646) / v78;
                let v3650 = (v3648.ln()) / v3640;
                let v11030 = ((((v11022 * v3644) + ((v11022 * v10360) * v3646)) / v78) * (v9353 / v3648)) / v3640;
                let v11031 = Lanes([v9666[0], v9666[1], v9666[2], v9666[3], v9666[4], 0.0]);
                let mut v3651: f64 = 0.0;
                let mut v3654: f64 = 0.0;
                let mut v3744: f64 = 0.0;
                let mut v3750: f64 = 0.0;
                let mut v3835: f64 = 0.0;
                let mut v3842: f64 = 0.0;
                let mut v3845: f64 = 0.0;
                let mut v4188: f64 = 0.0;
                let mut v9679: Lanes<6> = Lanes([0.0; 6]);
                let mut v9680: Lanes<6> = Lanes([0.0; 6]);
                let mut v9681: Lanes<6> = Lanes([0.0; 6]);
                let mut v9682: Lanes<6> = Lanes([0.0; 6]);
                v3651 = v1;
                v3654 = v3636;
                v3744 = v0;
                v3750 = v3834;
                v3835 = v0;
                v3842 = v0;
                v3845 = v0;
                v4188 = v4189;
                v9679 = v11031;
                v9680 = v11032;
                v9681 = v11032;
                v9682 = v11032;
                loop {
                    let v3653 = if v3651 <= v3652 { 1.0 } else { 0.0 };
                    if v3653 == 0.0 {
                        break;
                    }
                    let v3655 = v3654 - v835;
                    let v11658 = v9679 - (Lanes([v9398[0], v9398[1], 0.0, 0.0, v9398[2], 0.0]));
                    let v3656 = v663 * v3655;
                    let v11659 = v10380 * v3655;
                    let v11662 = (Lanes([0.0, 0.0, v11659[0], 0.0, 0.0, 0.0])) + (v11658 * v663);
                    let v3657 = v3655 - v3640;
                    let v3658 = v3650 * v3657;
                    let v11663 = v11030 * v3657;
                    let v11666 = (Lanes([0.0, 0.0, v11663[0], 0.0, 0.0, 0.0])) + (v11658 * v3650);
                    let v3659 = if v3658 < v2535 { 1.0 } else { 0.0 };
                    let v3669: f64;
                    let v3674: f64;
                    let v9683: Lanes<6>;
                    let v9684: Lanes<6>;
                    if v3659 != 0.0 {
                        let v3660 = v3658.exp();
                        let v11667 = v11666 * v3660;
                        let v3663 = ((-v3650) * v3640).exp();
                        let v11670 = ((v11030 * v10360) * v3640) * v3663;
                        let v11672 = v11667 - (Lanes([0.0, 0.0, v11670[0], 0.0, 0.0, 0.0]));
                        let v3665 = v1 + (v3660 - v3663);
                        let v3667 = (v3665.ln()) / v3650;
                        let v11675 = v11030 * v3667;
                        let v11678 = ((v11672 * (v9353 / v3665)) - (Lanes([0.0, 0.0, v11675[0], 0.0, 0.0, 0.0]))) / v3650;
                        let v3668 = v3660 / v3665;
                        let v11681 = (v11667 - (v11672 * v3668)) / v3665;
                        v3669 = v3667;
                        v3674 = v3668;
                        v9683 = v11678;
                        v9684 = v11681;
                    } else {
                        v3669 = v3657;
                        v3674 = v1;
                        v9683 = v11658;
                        v9684 = v11032;
                    }
                    let v3670 = v663 * v3669;
                    let v11682 = v10380 * v3669;
                    let v11685 = (Lanes([0.0, 0.0, v11682[0], 0.0, 0.0, 0.0])) + (v9683 * v663);
                    let v3671 = v3656.abs();
                    let v3673 = if v3671 < v3672 { 1.0 } else { 0.0 };
                    let v3753: f64;
                    let v3763: f64;
                    let v9685: Lanes<6>;
                    let v9686: Lanes<6>;
                    if v3673 != 0.0 {
                        let v11788 = v9684 * v3674;
                        let v3678 = ((v1 - (v3674 * v3674)) / v78).sqrt();
                        let v11794 = (((v11788 + v11788) * v10360) / v78) * (v9353 / (v10405 * v3678));
                        let v3679 = v3656 * v3678;
                        let v11797 = (v11662 * v3678) + (v11794 * v3656);
                        let v3680 = v663 * v3678;
                        let v11798 = v10380 * v3678;
                        let v11801 = (Lanes([0.0, 0.0, v11798[0], 0.0, 0.0, 0.0])) + (v11794 * v663);
                        let v3681 = if v3656 < v0 { 1.0 } else { 0.0 };
                        let v3754: f64;
                        let v3764: f64;
                        let v9687: Lanes<6>;
                        let v9688: Lanes<6>;
                        if v3681 != 0.0 {
                            let v3682 = -v3679;
                            let v11802 = v11797 * v10360;
                            let v3683 = -v3680;
                            let v11803 = v11801 * v10360;
                            v3754 = v3682;
                            v3764 = v3683;
                            v9687 = v11802;
                            v9688 = v11803;
                        } else {
                            v3754 = v3679;
                            v3764 = v3680;
                            v9687 = v11797;
                            v9688 = v11801;
                        }
                        v3753 = v3754;
                        v3763 = v3764;
                        v9685 = v9687;
                        v9686 = v9688;
                    } else {
                        let v3685 = if v3671 < v3684 { 1.0 } else { 0.0 };
                        let v3755: f64;
                        let v3765: f64;
                        let v9689: Lanes<6>;
                        let v9690: Lanes<6>;
                        if v3685 != 0.0 {
                            let v11710 = v11662 * v3656;
                            let v3687 = (v3656 * v3656) / v78;
                            let v3688 = v3656 / v96;
                            let v11713 = v11662 / v96;
                            let v3689 = v3656 / v90;
                            let v11714 = v11662 / v90;
                            let v3691 = v1 - (v3656 / v644);
                            let v3693 = v1 - (v3689 * v3691);
                            let v3695 = v1 - (v3688 * v3693);
                            let v3697 = v3656 / v78;
                            let v3698 = v1 - v3689;
                            let v3700 = v1 - (v3688 * v3698);
                            let v3702 = v1 - (v3697 * v3700);
                            let v11741 = v11685 * v3670;
                            let v3705 = (v3670 * v3670) / v78;
                            let v3706 = v3670 / v96;
                            let v11744 = v11685 / v96;
                            let v3707 = v3670 / v90;
                            let v11745 = v11685 / v90;
                            let v3709 = v1 - (v3670 / v644);
                            let v3711 = v1 - (v3707 * v3709);
                            let v3713 = v1 - (v3706 * v3711);
                            let v3715 = v3670 / v78;
                            let v3716 = v1 - v3707;
                            let v3718 = v1 - (v3706 * v3716);
                            let v3720 = v1 - (v3715 * v3718);
                            let v3721 = v3670 * v3720;
                            let v3723 = ((v3687 * v3695) - (v3705 * v3713)).sqrt();
                            let v11775 = (((((v11710 + v11710) / v78) * v3695) + ((((v11713 * v3693) + ((((v11714 * v3691) + (((v11662 / v644) * v10360) * v3689)) * v10360) * v3688)) * v10360) * v3687)) - ((((v11741 + v11741) / v78) * v3713) + ((((v11744 * v3711) + ((((v11745 * v3709) + (((v11685 / v644) * v10360) * v3707)) * v10360) * v3706)) * v10360) * v3705))) * (v9353 / (v10405 * v3723));
                            let v3724 = v663 * v13;
                            let v3726 = (v3656 * v3702) - (v3674 * v3721);
                            let v11781 = (v10380 * v13) * v3726;
                            let v3728 = (v3724 * v3726) / v3723;
                            let v11787 = (((Lanes([0.0, 0.0, v11781[0], 0.0, 0.0, 0.0])) + ((((v11662 * v3702) + (((((v11662 / v78) * v3700) + ((((v11713 * v3698) + ((v11714 * v10360) * v3688)) * v10360) * v3697)) * v10360) * v3656)) - ((v9684 * v3721) + (((v11685 * v3720) + (((((v11685 / v78) * v3718) + ((((v11744 * v3716) + ((v11745 * v10360) * v3706)) * v10360) * v3715)) * v10360) * v3670)) * v3674))) * v3724)) - (v11775 * v3728)) / v3723;
                            v3755 = v3723;
                            v3765 = v3728;
                            v9689 = v11775;
                            v9690 = v11787;
                        } else {
                            let v3730 = (-v3656).exp();
                            let v11687 = (v11662 * v10360) * v3730;
                            let v3732 = (-v3670).exp();
                            let v11689 = (v11685 * v10360) * v3732;
                            let v3736 = ((v3656 - v3670) + (v3730 - v3732)).sqrt();
                            let v11695 = ((v11662 - v11685) + (v11687 - v11689)) * (v9353 / (v10405 * v3736));
                            let v3737 = v663 * v13;
                            let v3739 = v1 - v3732;
                            let v3741 = (v1 - v3730) - (v3674 * v3739);
                            let v11703 = (v10380 * v13) * v3741;
                            let v3743 = (v3737 * v3741) / v3736;
                            let v11709 = (((Lanes([0.0, 0.0, v11703[0], 0.0, 0.0, 0.0])) + (((v11687 * v10360) - ((v9684 * v3739) + ((v11689 * v10360) * v3674))) * v3737)) - (v11695 * v3743)) / v3736;
                            v3755 = v3736;
                            v3765 = v3743;
                            v9689 = v11695;
                            v9690 = v11709;
                        }
                        v3753 = v3755;
                        v3763 = v3765;
                        v9685 = v9689;
                        v9686 = v9690;
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
                    let v9691: Lanes<6>;
                    if v3752 != 0.0 {
                        v3757 = v0;
                        v9691 = v11032;
                    } else {
                        let v3756 = v762 * v3753;
                        let v11804 = v10473 * v3753;
                        let v11807 = (Lanes([0.0, 0.0, v11804[0], 0.0, 0.0, 0.0])) + (v9685 * v762);
                        v3757 = v3756;
                        v9691 = v11807;
                    }
                    let v3760 = if v3757 < (v12 * v3758) { 1.0 } else { 0.0 };
                    let v4190: f64;
                    if v3760 != 0.0 {
                        v4190 = v1;
                    } else {
                        v4190 = v78;
                    }
                    let v3761 = v490 * v3757;
                    let v11808 = v9691 * v490;
                    let v3797: f64;
                    let v3803: f64;
                    let v3846: f64;
                    let v9692: Lanes<6>;
                    let v9693: Lanes<6>;
                    let v9694: Lanes<6>;
                    if v3746 != 0.0 {
                        let v3762 = -v3753;
                        let v11861 = v9685 * v10360;
                        let v3766 = -v3763;
                        let v11862 = v9686 * v10360;
                        v3797 = v3762;
                        v3803 = v3766;
                        v3846 = v3845;
                        v9692 = v11861;
                        v9693 = v11862;
                        v9694 = v9682;
                    } else {
                        let v3767 = if v3656 < v117 { 1.0 } else { 0.0 };
                        let v3798: f64;
                        let v3804: f64;
                        let v3847: f64;
                        let v9695: Lanes<6>;
                        let v9696: Lanes<6>;
                        let v9697: Lanes<6>;
                        if v3767 != 0.0 {
                            v3798 = v3753;
                            v3804 = v3763;
                            v3847 = v3845;
                            v9695 = v9685;
                            v9696 = v9686;
                            v9697 = v9682;
                        } else {
                            let v3768 = if v3656 < v2535 { 1.0 } else { 0.0 };
                            let v3786: f64;
                            let v3791: f64;
                            let v9698: Lanes<6>;
                            let v9699: Lanes<6>;
                            if v3768 != 0.0 {
                                let v3769 = v3656.exp();
                                let v11832 = v11662 * v3769;
                                let v3771 = v3769 - (v3656 + v1);
                                let v3772 = v3635 * v3771;
                                let v11834 = v11017 * v3771;
                                let v11837 = (Lanes([v11834[0], v11834[1], v11834[2], 0.0, v11834[3], 0.0])) + ((v11832 - v11662) * v3635);
                                let v3773 = v3635 * v663;
                                let v11839 = v10380 * v3635;
                                let v3774 = v3769 - v1;
                                let v3775 = v3773 * v3774;
                                let v11842 = ((v11017 * v663) + (Lanes([0.0, 0.0, v11839[0], 0.0]))) * v3774;
                                let v11845 = (Lanes([v11842[0], v11842[1], v11842[2], 0.0, v11842[3], 0.0])) + (v11832 * v3773);
                                v3786 = v3772;
                                v3791 = v3775;
                                v9698 = v11837;
                                v9699 = v11845;
                            } else {
                                let v11809 = v10380 * v3654;
                                let v3777 = (v663 * v3654).exp();
                                let v11813 = ((Lanes([0.0, 0.0, v11809[0], 0.0, 0.0, 0.0])) + (v9679 * v663)) * v3777;
                                let v3778 = v3656 + v1;
                                let v11814 = v11013 * v3778;
                                let v3780 = v3777 - (v3634 * v3778);
                                let v3781 = v759 * v3780;
                                let v11819 = v10466 * v3780;
                                let v11822 = (Lanes([0.0, 0.0, v11819[0], 0.0, 0.0, 0.0])) + ((v11813 - ((Lanes([v11814[0], v11814[1], v11814[2], 0.0, v11814[3], 0.0])) + (v11662 * v3634))) * v759);
                                let v3782 = v759 * v663;
                                let v3783 = v3777 - v3634;
                                let v3784 = v3782 * v3783;
                                let v11828 = ((v10466 * v663) + (v10380 * v759)) * v3783;
                                let v11831 = (Lanes([0.0, 0.0, v11828[0], 0.0, 0.0, 0.0])) + ((v11813 - (Lanes([v11013[0], v11013[1], v11013[2], 0.0, v11013[3], 0.0]))) * v3782);
                                v3786 = v3781;
                                v3791 = v3784;
                                v9698 = v11822;
                                v9699 = v11831;
                            }
                            let v11846 = v9685 * v3753;
                            let v3788 = ((v3753 * v3753) + v3786).sqrt();
                            let v11851 = ((v11846 + v11846) + v9698) * (v9353 / (v10405 * v3788));
                            let v3789 = v78 * v3763;
                            let v3794 = (v13 * ((v3789 * v3753) + v3791)) / v3788;
                            let v11860 = ((((((v9686 * v78) * v3753) + (v9685 * v3789)) + v9699) * v13) - (v11851 * v3794)) / v3788;
                            v3798 = v3788;
                            v3804 = v3794;
                            v3847 = v3786;
                            v9695 = v11851;
                            v9696 = v11860;
                            v9697 = v9698;
                        }
                        v3797 = v3798;
                        v3803 = v3804;
                        v3846 = v3847;
                        v9692 = v9695;
                        v9693 = v9696;
                        v9694 = v9697;
                    }
                    let v11863 = v10798 * v10360;
                    let v11866 = v10804 * v3797;
                    let v11871 = v9405 * v3632;
                    let v11872 = v9678 * v1048;
                    let v11875 = (Lanes([v11871[0], v11871[1], v11871[2], v11871[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v11872[0]]));
                    let v3802 = (((-v1200) + v3654) + (v1206 * v3797)) - (v1048 * v3632);
                    let v11877 = (((Lanes([v11863[0], v11863[1], v11863[2], v11863[3], v11863[4], 0.0])) + v9679) + ((Lanes([v11866[0], v11866[1], v11866[2], v11866[3], v11866[4], 0.0])) + (v9692 * v1206))) - (Lanes([v11875[0], v11875[1], 0.0, v11875[2], v11875[3], v11875[4]]));
                    let v11878 = v10804 * v3803;
                    let v11881 = (Lanes([v11878[0], v11878[1], v11878[2], v11878[3], v11878[4], 0.0])) + (v9693 * v1206);
                    let v3806 = v1 + (v1206 * v3803);
                    let v3829: f64;
                    let v3831: f64;
                    let v3832: f64;
                    let v9700: Lanes<6>;
                    if v3745 != 0.0 {
                        v3829 = v3807;
                        v3831 = v3654;
                        v3832 = v3744;
                        v9700 = v9679;
                    } else {
                        let v3809 = (-v3802) / v3806;
                        let v11885 = ((v11877 * v10360) - (v11881 * v3809)) / v3806;
                        let v3811 = v3654.abs();
                        let v11889 = v9679 * ((v10405 * (if v3654 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                        let v3812 = if v1 >= v3811 { 1.0 } else { 0.0 };
                        let v3813: f64;
                        let v9701: Lanes<6>;
                        if v3812 != 0.0 {
                            v3813 = v1;
                            v9701 = v11032;
                        } else {
                            v3813 = v3811;
                            v9701 = v11889;
                        }
                        let v3815 = v3810 * (v1 + v3813);
                        let v11890 = v9701 * v3810;
                        let v3817 = if (v3809.abs()) > v3815 { 1.0 } else { 0.0 };
                        let v3822: f64;
                        let v9702: Lanes<6>;
                        if v3817 != 0.0 {
                            let v3818 = if v3809 >= v0 { 1.0 } else { 0.0 };
                            let v3820: f64;
                            if v3818 != 0.0 {
                                v3820 = v1;
                            } else {
                                v3820 = v3819;
                            }
                            let v3821 = v3815 * v3820;
                            let v11891 = v11890 * v3820;
                            v3822 = v3821;
                            v9702 = v11891;
                        } else {
                            v3822 = v3809;
                            v9702 = v11885;
                        }
                        let v3823 = v3654 + v3822;
                        let v11892 = v9679 + v9702;
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
                        v9700 = v11892;
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
                    v9679 = v9700;
                    v9680 = v11808;
                    v9681 = v9692;
                    v9682 = v9694;
                }
                let v3836 = v3835 / v750;
                let v11033 = v10455 * v3836;
                let v11036 = (v9680 - (Lanes([0.0, 0.0, v11033[0], 0.0, 0.0, 0.0]))) / v750;
                let v11037 = v11036 * v3836;
                let v11038 = v11037 + v11037;
                let v3839 = (v3836 * v3836) + v3838;
                let v3841 = v3836 + v3840;
                let v3843 = v3842 + v3841;
                let v3844 = v1 / v3843;
                let v3848 = v750 * v3845;
                let v11043 = v10455 * v3845;
                let v3849 = v3848 * v3844;
                let v11049 = (((Lanes([0.0, 0.0, v11043[0], 0.0, 0.0, 0.0])) + (v9682 * v750)) * v3844) + (((((v9681 + v11036) * v3844) * v10360) / v3843) * v3848);
                let v3850 = -v3849;
                let v11050 = v11049 * v10360;
                let v3851 = v3849 * v1048;
                let v11052 = v9405 * v3849;
                let v11054 = (v11049 * v1048) + (Lanes([v11052[0], v11052[1], 0.0, v11052[2], v11052[3], 0.0]));
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
                let v9703: Lanes<6>;
                let v9704: Lanes<6>;
                let v9705: Lanes<6>;
                let v9706: Lanes<6>;
                let v9707: Lanes<6>;
                let v9708: Lanes<6>;
                let v9709: Lanes<6>;
                if v3855 != 0.0 {
                    let v3856 = v1200 - v3654;
                    let v3857 = v1128 * v3856;
                    let v11057 = v9406 * v3856;
                    let v11060 = (Lanes([v11057[0], v11057[1], 0.0, v11057[2], v11057[3], 0.0])) + (((Lanes([v10798[0], v10798[1], v10798[2], v10798[3], v10798[4], 0.0])) - v9679) * v1128);
                    let v3859 = (-v168) * v139;
                    let v3860 = v3859 * v3857;
                    let v11061 = v11060 * v3859;
                    let v3864 = -v3861;
                    let v3865 = v3864 * v3857;
                    let v11062 = v11060 * v3864;
                    let v3866 = v3865 * v13;
                    let v11063 = v11062 * v13;
                    let v3867 = v3865 - v3866;
                    let v11064 = v11062 - v11063;
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
                    v9703 = v11032;
                    v9704 = v9679;
                    v9705 = v11060;
                    v9706 = v9679;
                    v9707 = v11061;
                    v9708 = v11064;
                    v9709 = v11063;
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
                    v9703 = v11054;
                    v9704 = v11032;
                    v9705 = v11032;
                    v9706 = v11032;
                    v9707 = v11032;
                    v9708 = v11032;
                    v9709 = v11032;
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
                let v9710: Lanes<6>;
                let v9711: Lanes<6>;
                let v9712: Lanes<6>;
                let v9713: Lanes<6>;
                let v9714: Lanes<6>;
                let v9715: Lanes<6>;
                let v9716: Lanes<6>;
                let v9717: Lanes<6>;
                if v3869 != 0.0 {
                    let v3870 = v1128 * v1128;
                    let v11065 = v9406 * v1128;
                    let v3871 = v491 / v3870;
                    let v11069 = (((v11065 + v11065) * v3871) * v10360) / v3870;
                    let v3872 = v78 / v3871;
                    let v11072 = ((v11069 * v3872) * v10360) / v3871;
                    let v3873 = v1200 - v362;
                    let v11073 = v11072 * v3873;
                    let v11076 = (Lanes([v11073[0], v11073[1], 0.0, v11073[2], v11073[3]])) + (v10798 * v3872);
                    let v3875 = v1 + (v3872 * v3873);
                    let v3876 = v1 + v3872;
                    let v3879 = if (if v3875 < v3876 { 1.0 } else { 0.0 }) != 0.0 && (if v3876 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v3911: f64;
                    let v9718: Lanes<5>;
                    if v3879 != 0.0 {
                        let v3880 = v3876 - v3875;
                        let v11077 = Lanes([v11072[0], v11072[1], 0.0, v11072[2], v11072[3]]);
                        let v11078 = v11077 - v11076;
                        let v3881 = v3880 * v3880;
                        let v11079 = v11078 * v3880;
                        let v11080 = v11079 + v11079;
                        let v3882 = v3876 * v3876;
                        let v11081 = v11072 * v3876;
                        let v11082 = v11081 + v11081;
                        let v3883 = v3881 * v3881;
                        let v11083 = v11080 * v3881;
                        let v3884 = v3882 * v3882;
                        let v11085 = v11082 * v3882;
                        let v3885 = v3883 * v3881;
                        let v3886 = v3884 * v3882;
                        let v11098 = ((((v11085 + v11085) * v3882) + (v11082 * v3884)) * v3882) + (v11082 * v3886);
                        let v3889 = (v3885 * v3881) + (v3886 * v3882);
                        let v11100 = (((((v11083 + v11083) * v3881) + (v11080 * v3883)) * v3881) + (v11080 * v3885)) + (Lanes([v11098[0], v11098[1], 0.0, v11098[2], v11098[3]]));
                        let v3906: f64;
                        let v9719: Lanes<5>;
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
                            let mut v9720: Lanes<5> = Lanes([0.0; 5]);
                            v3895 = v0;
                            v3897 = v3889;
                            v9720 = v11100;
                            loop {
                                let v3896 = if v3895 < v3900 { 1.0 } else { 0.0 };
                                if v3896 == 0.0 {
                                    break;
                                }
                                let v3898 = v3897.sqrt();
                                let v11656 = v9720 * (v9353 / (v10405 * v3898));
                                let v3899 = v3895 + v1;
                                v3895 = v3899;
                                v3897 = v3898;
                                v9720 = v11656;
                            }
                            v3906 = v3897;
                            v9719 = v9720;
                        } else {
                            let v3905 = v3889.powf(v3904);
                            let v11104 = v11100 * (v3904 * (v3889.powf(v11101)));
                            v3906 = v3905;
                            v9719 = v11104;
                        }
                        let v3907 = v1 / v3906;
                        let v3908 = v3880 * v3876;
                        let v11109 = v11072 * v3880;
                        let v3910 = v3876 - (v3908 * v3907);
                        let v11115 = v11077 - ((((v11078 * v3876) + (Lanes([v11109[0], v11109[1], 0.0, v11109[2], v11109[3]]))) * v3907) + ((((v9719 * v3907) * v10360) / v3906) * v3908));
                        v3911 = v3910;
                        v9718 = v11115;
                    } else {
                        v3911 = v3875;
                        v9718 = v11076;
                    }
                    let v3912 = v3911.sqrt();
                    let v3913 = v1 - v3912;
                    let v11120 = v11069 * v3913;
                    let v3915 = v1200 + (v3871 * v3913);
                    let v11124 = v10798 + ((Lanes([v11120[0], v11120[1], 0.0, v11120[2], v11120[3]])) + (((v9718 * (v9353 / (v10405 * v3912))) * v10360) * v3871));
                    let v11125 = v11124 * v3915;
                    let v3919 = ((v3915 * v3915) + v3917).sqrt();
                    let v11131 = (v11124 + ((v11125 + v11125) * (v9353 / (v10405 * v3919)))) * v13;
                    let v3923 = (v13 * (v3915 + v3919)) + v3922;
                    let v3924 = if v3923 < v0 { 1.0 } else { 0.0 };
                    let v3925: f64;
                    let v9721: Lanes<5>;
                    if v3924 != 0.0 {
                        v3925 = v0;
                        v9721 = v10549;
                    } else {
                        v3925 = v3923;
                        v9721 = v11131;
                    }
                    let v3926 = v823 / v3925;
                    let v11134 = (v10567 - (v9721 * v3926)) / v3925;
                    let v3927 = v2662 - v1;
                    let v3928 = v3926.powf(v3927);
                    let v11141 = ((v11134 * (v3927 * (v3926.powf((v3927 - v9353))))) * v3926) + (v11134 * v3928);
                    let v3930 = v1 + (v3928 * v3926);
                    let v3932 = (v1 / v2662) - v1;
                    let v3933 = v3930.powf(v3932);
                    let v3934 = v3933 * v3930;
                    let v3935 = v823 / v3934;
                    let v11151 = (v10567 - ((((v11141 * (v3932 * (v3930.powf((v3932 - v9353))))) * v3930) + (v11141 * v3933)) * v3935)) / v3934;
                    let v3936 = v835 - v3935;
                    let v11153 = v10380 * v3936;
                    let v3938 = (v663 * v3936).exp();
                    let v11157 = ((Lanes([0.0, 0.0, v11153[0], 0.0, 0.0])) + ((v10807 - v11151) * v663)) * v3938;
                    let v3939 = if v3935 <= v0 { 1.0 } else { 0.0 };
                    let v3975: f64;
                    let v9722: Lanes<6>;
                    if v3939 != 0.0 {
                        v3975 = v3654;
                        v9722 = v9679;
                    } else {
                        let v3969: f64;
                        let v9723: Lanes<6>;
                        if v3940 != 0.0 {
                            let v3941 = v0 - v3654;
                            let v11158 = v9679 * v10360;
                            v3969 = v3941;
                            v9723 = v11158;
                        } else {
                            v3969 = v0;
                            v9723 = v11032;
                        }
                        let v3968: f64;
                        let v9724: Lanes<6>;
                        if v3942 != 0.0 {
                            let v3944 = v3943 - v3654;
                            let v11160 = (Lanes([v9667[0], v9667[1], v9667[2], v9667[3], v9667[4], 0.0])) - v9679;
                            let v3945 = if v3944 >= v0 { 1.0 } else { 0.0 };
                            let v3946: f64;
                            let v9725: Lanes<6>;
                            if v3945 != 0.0 {
                                v3946 = v3944;
                                v9725 = v11160;
                            } else {
                                v3946 = v0;
                                v9725 = v11032;
                            }
                            let v11163 = (v9725 * v3947) - (Lanes([v11151[0], v11151[1], v11151[2], v11151[3], v11151[4], 0.0]));
                            let v3950 = ((v3947 * v3946) - v3935) - v1985;
                            let v3954 = (v90 * (v3951 * v3946)) * v1985;
                            let v11166 = ((v9725 * v3951) * v90) * v1985;
                            let v3955 = if v3954 > v0 { 1.0 } else { 0.0 };
                            let v3957: f64;
                            let v9726: Lanes<6>;
                            if v3955 != 0.0 {
                                v3957 = v3954;
                                v9726 = v11166;
                            } else {
                                let v3956 = -v3954;
                                let v11167 = v11166 * v10360;
                                v3957 = v3956;
                                v9726 = v11167;
                            }
                            let v11168 = v11163 * v3950;
                            let v3960 = ((v3950 * v3950) + v3957).sqrt();
                            let v3965 = (v3961 * v3946) - (v13 * (v3950 + v3960));
                            let v11177 = (v9725 * v3961) - ((v11163 + (((v11168 + v11168) + v9726) * (v9353 / (v10405 * v3960)))) * v13);
                            let v3966 = if v3965 <= v3946 { 1.0 } else { 0.0 };
                            let v3967: f64;
                            let v9727: Lanes<6>;
                            if v3966 != 0.0 {
                                v3967 = v3965;
                                v9727 = v11177;
                            } else {
                                v3967 = v3946;
                                v9727 = v9725;
                            }
                            v3968 = v3967;
                            v9724 = v9727;
                        } else {
                            v3968 = v3969;
                            v9724 = v9723;
                        }
                        let v3970 = if v3968 < v0 { 1.0 } else { 0.0 };
                        let v3972: f64;
                        let v9728: Lanes<6>;
                        if v3970 != 0.0 {
                            v3972 = v0;
                            v9728 = v11032;
                        } else {
                            let v3971 = if v3968 > v3935 { 1.0 } else { 0.0 };
                            let v3973: f64;
                            let v9729: Lanes<6>;
                            if v3971 != 0.0 {
                                let v11178 = Lanes([v11151[0], v11151[1], v11151[2], v11151[3], v11151[4], 0.0]);
                                v3973 = v3935;
                                v9729 = v11178;
                            } else {
                                v3973 = v3968;
                                v9729 = v9724;
                            }
                            v3972 = v3973;
                            v9728 = v9729;
                        }
                        let v3974 = v3654 + v3972;
                        let v11179 = v9679 + v9728;
                        v3975 = v3974;
                        v9722 = v11179;
                    }
                    let mut v3976: f64 = 0.0;
                    let mut v3979: f64 = 0.0;
                    let mut v4112: f64 = 0.0;
                    let mut v4144: f64 = 0.0;
                    let mut v4148: f64 = 0.0;
                    let mut v4151: f64 = 0.0;
                    let mut v9730: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9731: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9732: Lanes<6> = Lanes([0.0; 6]);
                    let mut v9733: Lanes<6> = Lanes([0.0; 6]);
                    v3976 = v1;
                    v3979 = v3975;
                    v4112 = v0;
                    v4144 = v3835;
                    v4148 = v0;
                    v4151 = v0;
                    v9730 = v9722;
                    v9731 = v9680;
                    v9732 = v11032;
                    v9733 = v11032;
                    loop {
                        let v3978 = if v3976 <= v3977 { 1.0 } else { 0.0 };
                        if v3978 == 0.0 {
                            break;
                        }
                        let v3980 = v3979 - v835;
                        let v11431 = v9730 - (Lanes([v9398[0], v9398[1], 0.0, 0.0, v9398[2], 0.0]));
                        let v3981 = v663 * v3980;
                        let v11432 = v10380 * v3980;
                        let v11435 = (Lanes([0.0, 0.0, v11432[0], 0.0, 0.0, 0.0])) + (v11431 * v663);
                        let v3982 = v3980 - v3640;
                        let v3983 = v3650 * v3982;
                        let v11436 = v11030 * v3982;
                        let v11439 = (Lanes([0.0, 0.0, v11436[0], 0.0, 0.0, 0.0])) + (v11431 * v3650);
                        let v3984 = if v3983 < v2535 { 1.0 } else { 0.0 };
                        let v3994: f64;
                        let v3998: f64;
                        let v9734: Lanes<6>;
                        let v9735: Lanes<6>;
                        if v3984 != 0.0 {
                            let v3985 = v3983.exp();
                            let v11440 = v11439 * v3985;
                            let v3988 = ((-v3650) * v3640).exp();
                            let v11443 = ((v11030 * v10360) * v3640) * v3988;
                            let v11445 = v11440 - (Lanes([0.0, 0.0, v11443[0], 0.0, 0.0, 0.0]));
                            let v3990 = v1 + (v3985 - v3988);
                            let v3992 = (v3990.ln()) / v3650;
                            let v11448 = v11030 * v3992;
                            let v11451 = ((v11445 * (v9353 / v3990)) - (Lanes([0.0, 0.0, v11448[0], 0.0, 0.0, 0.0]))) / v3650;
                            let v3993 = v3985 / v3990;
                            let v11454 = (v11440 - (v11445 * v3993)) / v3990;
                            v3994 = v3992;
                            v3998 = v3993;
                            v9734 = v11451;
                            v9735 = v11454;
                        } else {
                            v3994 = v3982;
                            v3998 = v1;
                            v9734 = v11431;
                            v9735 = v11032;
                        }
                        let v3995 = v663 * v3994;
                        let v11455 = v10380 * v3994;
                        let v11458 = (Lanes([0.0, 0.0, v11455[0], 0.0, 0.0, 0.0])) + (v9734 * v663);
                        let v3996 = v3981.abs();
                        let v3997 = if v3996 < v3672 { 1.0 } else { 0.0 };
                        let v4069: f64;
                        let v4077: f64;
                        let v9736: Lanes<6>;
                        let v9737: Lanes<6>;
                        if v3997 != 0.0 {
                            let v11561 = v9735 * v3998;
                            let v4002 = ((v1 - (v3998 * v3998)) / v78).sqrt();
                            let v11567 = (((v11561 + v11561) * v10360) / v78) * (v9353 / (v10405 * v4002));
                            let v4003 = v3981 * v4002;
                            let v11570 = (v11435 * v4002) + (v11567 * v3981);
                            let v4004 = v663 * v4002;
                            let v11571 = v10380 * v4002;
                            let v11574 = (Lanes([0.0, 0.0, v11571[0], 0.0, 0.0, 0.0])) + (v11567 * v663);
                            let v4005 = if v3981 < v0 { 1.0 } else { 0.0 };
                            let v4070: f64;
                            let v4078: f64;
                            let v9738: Lanes<6>;
                            let v9739: Lanes<6>;
                            if v4005 != 0.0 {
                                let v4006 = -v4003;
                                let v11575 = v11570 * v10360;
                                let v4007 = -v4004;
                                let v11576 = v11574 * v10360;
                                v4070 = v4006;
                                v4078 = v4007;
                                v9738 = v11575;
                                v9739 = v11576;
                            } else {
                                v4070 = v4003;
                                v4078 = v4004;
                                v9738 = v11570;
                                v9739 = v11574;
                            }
                            v4069 = v4070;
                            v4077 = v4078;
                            v9736 = v9738;
                            v9737 = v9739;
                        } else {
                            let v4008 = if v3996 < v3684 { 1.0 } else { 0.0 };
                            let v4071: f64;
                            let v4079: f64;
                            let v9740: Lanes<6>;
                            let v9741: Lanes<6>;
                            if v4008 != 0.0 {
                                let v11483 = v11435 * v3981;
                                let v4010 = (v3981 * v3981) / v78;
                                let v4011 = v3981 / v96;
                                let v11486 = v11435 / v96;
                                let v4012 = v3981 / v90;
                                let v11487 = v11435 / v90;
                                let v4014 = v1 - (v3981 / v644);
                                let v4016 = v1 - (v4012 * v4014);
                                let v4018 = v1 - (v4011 * v4016);
                                let v4020 = v3981 / v78;
                                let v4021 = v1 - v4012;
                                let v4023 = v1 - (v4011 * v4021);
                                let v4025 = v1 - (v4020 * v4023);
                                let v11514 = v11458 * v3995;
                                let v4028 = (v3995 * v3995) / v78;
                                let v4029 = v3995 / v96;
                                let v11517 = v11458 / v96;
                                let v4030 = v3995 / v90;
                                let v11518 = v11458 / v90;
                                let v4032 = v1 - (v3995 / v644);
                                let v4034 = v1 - (v4030 * v4032);
                                let v4036 = v1 - (v4029 * v4034);
                                let v4038 = v3995 / v78;
                                let v4039 = v1 - v4030;
                                let v4041 = v1 - (v4029 * v4039);
                                let v4043 = v1 - (v4038 * v4041);
                                let v4044 = v3995 * v4043;
                                let v4046 = ((v4010 * v4018) - (v4028 * v4036)).sqrt();
                                let v11548 = (((((v11483 + v11483) / v78) * v4018) + ((((v11486 * v4016) + ((((v11487 * v4014) + (((v11435 / v644) * v10360) * v4012)) * v10360) * v4011)) * v10360) * v4010)) - ((((v11514 + v11514) / v78) * v4036) + ((((v11517 * v4034) + ((((v11518 * v4032) + (((v11458 / v644) * v10360) * v4030)) * v10360) * v4029)) * v10360) * v4028))) * (v9353 / (v10405 * v4046));
                                let v4047 = v663 * v13;
                                let v4049 = (v3981 * v4025) - (v3998 * v4044);
                                let v11554 = (v10380 * v13) * v4049;
                                let v4051 = (v4047 * v4049) / v4046;
                                let v11560 = (((Lanes([0.0, 0.0, v11554[0], 0.0, 0.0, 0.0])) + ((((v11435 * v4025) + (((((v11435 / v78) * v4023) + ((((v11486 * v4021) + ((v11487 * v10360) * v4011)) * v10360) * v4020)) * v10360) * v3981)) - ((v9735 * v4044) + (((v11458 * v4043) + (((((v11458 / v78) * v4041) + ((((v11517 * v4039) + ((v11518 * v10360) * v4029)) * v10360) * v4038)) * v10360) * v3995)) * v3998))) * v4047)) - (v11548 * v4051)) / v4046;
                                v4071 = v4046;
                                v4079 = v4051;
                                v9740 = v11548;
                                v9741 = v11560;
                            } else {
                                let v4053 = (-v3981).exp();
                                let v11460 = (v11435 * v10360) * v4053;
                                let v4055 = (-v3995).exp();
                                let v11462 = (v11458 * v10360) * v4055;
                                let v4059 = ((v3981 - v3995) + (v4053 - v4055)).sqrt();
                                let v11468 = ((v11435 - v11458) + (v11460 - v11462)) * (v9353 / (v10405 * v4059));
                                let v4060 = v663 * v13;
                                let v4062 = v1 - v4055;
                                let v4064 = (v1 - v4053) - (v3998 * v4062);
                                let v11476 = (v10380 * v13) * v4064;
                                let v4066 = (v4060 * v4064) / v4059;
                                let v11482 = (((Lanes([0.0, 0.0, v11476[0], 0.0, 0.0, 0.0])) + (((v11460 * v10360) - ((v9735 * v4062) + ((v11462 * v10360) * v3998))) * v4060)) - (v11468 * v4066)) / v4059;
                                v4071 = v4059;
                                v4079 = v4066;
                                v9740 = v11468;
                                v9741 = v11482;
                            }
                            v4069 = v4071;
                            v4077 = v4079;
                            v9736 = v9740;
                            v9737 = v9741;
                        }
                        let v4068 = if v4141 == v4067 { 1.0 } else { 0.0 };
                        let v4073: f64;
                        let v9742: Lanes<6>;
                        if v4068 != 0.0 {
                            v4073 = v0;
                            v9742 = v11032;
                        } else {
                            let v4072 = v762 * v4069;
                            let v11577 = v10473 * v4069;
                            let v11580 = (Lanes([0.0, 0.0, v11577[0], 0.0, 0.0, 0.0])) + (v9736 * v762);
                            v4073 = v4072;
                            v9742 = v11580;
                        }
                        let v4074 = v490 * v4073;
                        let v11581 = v9742 * v490;
                        let v4075 = if v3981 < v0 { 1.0 } else { 0.0 };
                        let v4102: f64;
                        let v4108: f64;
                        let v4152: f64;
                        let v9743: Lanes<6>;
                        let v9744: Lanes<6>;
                        let v9745: Lanes<6>;
                        if v4075 != 0.0 {
                            let v4076 = -v4069;
                            let v11622 = v9736 * v10360;
                            let v4080 = -v4077;
                            let v11623 = v9737 * v10360;
                            v4102 = v4076;
                            v4108 = v4080;
                            v4152 = v4151;
                            v9743 = v11622;
                            v9744 = v11623;
                            v9745 = v9733;
                        } else {
                            let v4081 = if v3981 < v117 { 1.0 } else { 0.0 };
                            let v4103: f64;
                            let v4109: f64;
                            let v4153: f64;
                            let v9746: Lanes<6>;
                            let v9747: Lanes<6>;
                            let v9748: Lanes<6>;
                            if v4081 != 0.0 {
                                v4103 = v4069;
                                v4109 = v4077;
                                v4153 = v4151;
                                v9746 = v9736;
                                v9747 = v9737;
                                v9748 = v9733;
                            } else {
                                let v4082 = v3979 - v3935;
                                let v11584 = v10380 * v4082;
                                let v4084 = (v663 * v4082).exp();
                                let v11588 = ((Lanes([0.0, 0.0, v11584[0], 0.0, 0.0, 0.0])) + ((v9730 - (Lanes([v11151[0], v11151[1], v11151[2], v11151[3], v11151[4], 0.0]))) * v663)) * v4084;
                                let v4085 = v3981 + v1;
                                let v11589 = v11157 * v4085;
                                let v4087 = v4084 - (v3938 * v4085);
                                let v4088 = v759 * v4087;
                                let v11594 = v10466 * v4087;
                                let v11597 = (Lanes([0.0, 0.0, v11594[0], 0.0, 0.0, 0.0])) + ((v11588 - ((Lanes([v11589[0], v11589[1], v11589[2], v11589[3], v11589[4], 0.0])) + (v11435 * v3938))) * v759);
                                let v4089 = v759 * v663;
                                let v4090 = v4084 - v3938;
                                let v11603 = ((v10466 * v663) + (v10380 * v759)) * v4090;
                                let v11607 = v9736 * v4069;
                                let v4094 = ((v4069 * v4069) + v4088).sqrt();
                                let v11612 = ((v11607 + v11607) + v11597) * (v9353 / (v10405 * v4094));
                                let v4095 = v78 * v4077;
                                let v4099 = (v13 * ((v4095 * v4069) + (v4089 * v4090))) / v4094;
                                let v11621 = ((((((v9737 * v78) * v4069) + (v9736 * v4095)) + ((Lanes([0.0, 0.0, v11603[0], 0.0, 0.0, 0.0])) + ((v11588 - (Lanes([v11157[0], v11157[1], v11157[2], v11157[3], v11157[4], 0.0]))) * v4089))) * v13) - (v11612 * v4099)) / v4094;
                                v4103 = v4094;
                                v4109 = v4099;
                                v4153 = v4088;
                                v9746 = v11612;
                                v9747 = v11621;
                                v9748 = v11597;
                            }
                            v4102 = v4103;
                            v4108 = v4109;
                            v4152 = v4153;
                            v9743 = v9746;
                            v9744 = v9747;
                            v9745 = v9748;
                        }
                        let v11624 = v10798 * v10360;
                        let v11627 = v10804 * v4102;
                        let v11632 = v9405 * v3632;
                        let v11633 = v9678 * v1048;
                        let v11636 = (Lanes([v11632[0], v11632[1], v11632[2], v11632[3], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v11633[0]]));
                        let v4107 = (((-v1200) + v3979) + (v1206 * v4102)) - (v1048 * v3632);
                        let v11638 = (((Lanes([v11624[0], v11624[1], v11624[2], v11624[3], v11624[4], 0.0])) + v9730) + ((Lanes([v11627[0], v11627[1], v11627[2], v11627[3], v11627[4], 0.0])) + (v9743 * v1206))) - (Lanes([v11636[0], v11636[1], 0.0, v11636[2], v11636[3], v11636[4]]));
                        let v11639 = v10804 * v4108;
                        let v11642 = (Lanes([v11639[0], v11639[1], v11639[2], v11639[3], v11639[4], 0.0])) + (v9744 * v1206);
                        let v4111 = v1 + (v1206 * v4108);
                        let v4115 = if (if v4112 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v3976 > v96 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v4138: f64;
                        let v4140: f64;
                        let v4142: f64;
                        let v9749: Lanes<6>;
                        if v4115 != 0.0 {
                            v4138 = v4116;
                            v4140 = v3979;
                            v4142 = v4112;
                            v9749 = v9730;
                        } else {
                            let v4118 = (-v4107) / v4111;
                            let v11646 = ((v11638 * v10360) - (v11642 * v4118)) / v4111;
                            let v4120 = v3979.abs();
                            let v11650 = v9730 * ((v10405 * (if v3979 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                            let v4121 = if v1 >= v4120 { 1.0 } else { 0.0 };
                            let v4122: f64;
                            let v9750: Lanes<6>;
                            if v4121 != 0.0 {
                                v4122 = v1;
                                v9750 = v11032;
                            } else {
                                v4122 = v4120;
                                v9750 = v11650;
                            }
                            let v4124 = v4119 * (v1 + v4122);
                            let v11651 = v9750 * v4119;
                            let v4126 = if (v4118.abs()) > v4124 { 1.0 } else { 0.0 };
                            let v4131: f64;
                            let v9751: Lanes<6>;
                            if v4126 != 0.0 {
                                let v4127 = if v4118 >= v0 { 1.0 } else { 0.0 };
                                let v4129: f64;
                                if v4127 != 0.0 {
                                    v4129 = v1;
                                } else {
                                    v4129 = v4128;
                                }
                                let v4130 = v4124 * v4129;
                                let v11652 = v11651 * v4129;
                                v4131 = v4130;
                                v9751 = v11652;
                            } else {
                                v4131 = v4118;
                                v9751 = v11646;
                            }
                            let v4132 = v3979 + v4131;
                            let v11653 = v9730 + v9751;
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
                            v9749 = v11653;
                        }
                        let v4139 = v4138 + v1;
                        v3976 = v4139;
                        v3979 = v4140;
                        v4112 = v4142;
                        v4144 = v4074;
                        v4148 = v4102;
                        v4151 = v4152;
                        v9730 = v9749;
                        v9731 = v11581;
                        v9732 = v9743;
                        v9733 = v9745;
                    }
                    let v4145 = v4144 / v750;
                    let v11180 = v10455 * v4145;
                    let v11183 = (v9731 - (Lanes([0.0, 0.0, v11180[0], 0.0, 0.0, 0.0]))) / v750;
                    let v4149 = v4148 + (v4145 + v4146);
                    let v4150 = v1 / v4149;
                    let v4154 = v750 * v4151;
                    let v11188 = v10455 * v4151;
                    let v4156 = -(v4154 * v4150);
                    let v11195 = ((((Lanes([0.0, 0.0, v11188[0], 0.0, 0.0, 0.0])) + (v9733 * v750)) * v4150) + (((((v9732 + v11183) * v4150) * v10360) / v4149) * v4154)) * v10360;
                    let v4157 = v3979 - v3654;
                    let v11196 = v9730 - v9679;
                    let v4158 = v663 / v3839;
                    let v4161 = ((v4158 * v4157) + v1).sqrt();
                    let v4162 = v4161 + v1;
                    let v4163 = v1 / v4162;
                    let v4164 = v4163 / v3841;
                    let v4166 = v13 * (v3836 + v4145);
                    let v11214 = (v11036 + v11183) * v13;
                    let v11216 = v10798 + (Lanes([0.0, 0.0, v10385[0], 0.0, 0.0]));
                    let v4171 = (v1200 + v665) - (v13 * ((v78 * v3654) + v4157));
                    let v4173 = (-v4166) + v4164;
                    let v4174 = v663 * v1128;
                    let v11224 = v10380 * v1128;
                    let v11225 = v9406 * v663;
                    let v4175 = v663 * v750;
                    let v11232 = ((Lanes([0.0, 0.0, v11224[0], 0.0, 0.0])) + (Lanes([v11225[0], v11225[1], 0.0, v11225[2], v11225[3]]))) * v4171;
                    let v11236 = ((v10380 * v750) + (v10455 * v663)) * v4173;
                    let v4178 = (v4174 * v4171) + (v4175 * v4173);
                    let v11240 = ((Lanes([v11232[0], v11232[1], v11232[2], v11232[3], v11232[4], 0.0])) + (((Lanes([v11216[0], v11216[1], v11216[2], v11216[3], v11216[4], 0.0])) - (((v9679 * v78) + v11196) * v13)) * v4174)) + ((Lanes([0.0, 0.0, v11236[0], 0.0, 0.0, 0.0])) + (((v11214 * v10360) + (((((((((((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0, 0.0])) - (v11038 * v4158)) / v3839) * v4157) + (v11196 * v4158)) * (v9353 / (v10405 * v4161))) * v4163) * v10360) / v4162) - (v11036 * v4164)) / v3841)) * v4175));
                    let v4179 = v4144 + v3835;
                    let v11241 = v9731 + v9680;
                    let v4180 = v4179 / v78;
                    let v11242 = v11241 / v78;
                    let v4181 = v4156 + v3850;
                    let v11243 = v11195 + v11050;
                    let v4183 = (-v4181) / v78;
                    let v11245 = (v11243 * v10360) / v78;
                    let v4184 = v4144 - v3835;
                    let v11246 = v9731 - v9680;
                    let v4186 = -(v4156 - v3850);
                    let v11248 = (v11195 - v11050) * v10360;
                    let v4187 = v750 * v750;
                    let v11249 = v10455 * v750;
                    let v11250 = v11249 + v11249;
                    let v4191 = if v4188 <= v1 { 1.0 } else { 0.0 };
                    let v4202: f64;
                    let v9752: Lanes<6>;
                    if v4191 != 0.0 {
                        let v4192 = v4183 * v663;
                        let v11255 = v10380 * v4183;
                        let v4195 = v4184 * v4184;
                        let v11262 = v11246 * v4184;
                        let v4197 = (v4195 * v4184) / v4187;
                        let v11267 = v11250 * v4197;
                        let v4199 = ((v4192 * v4157) - v4186) - (v4197 / v646);
                        let v11272 = (((((v11245 * v663) + (Lanes([0.0, 0.0, v11255[0], 0.0, 0.0, 0.0]))) * v4157) + (v11196 * v4192)) - v11248) - ((((((v11262 + v11262) * v4184) + (v11246 * v4195)) - (Lanes([0.0, 0.0, v11267[0], 0.0, 0.0, 0.0]))) / v4187) / v646);
                        v4202 = v4199;
                        v9752 = v11272;
                    } else {
                        let v4200 = v4157 * v4178;
                        let v11253 = (v11196 * v4178) + (v11240 * v4157);
                        v4202 = v4200;
                        v9752 = v11253;
                    }
                    let v4204 = if (if v70 >= v1 { 1.0 } else { 0.0 }) != 0.0 && (if v4202 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4229: f64;
                    let v9753: Lanes<6>;
                    if v4204 != 0.0 {
                        v4229 = v0;
                        v9753 = v11032;
                    } else {
                        v4229 = v4202;
                        v9753 = v9752;
                    }
                    let v4424: f64;
                    let v9754: Lanes<6>;
                    if v4191 != 0.0 {
                        let v4206 = if (v4157.abs()) > v23 { 1.0 } else { 0.0 };
                        let v4425: f64;
                        let v9755: Lanes<6>;
                        if v4206 != 0.0 {
                            let v4207 = v4183 * v663;
                            let v11276 = v10380 * v4183;
                            let v4209 = (v4207 * v4157) - v4186;
                            let v4211 = v78 * v4180;
                            let v11286 = v11242 * v78;
                            let v4213 = v1128 / v663;
                            let v11288 = v10380 * v4213;
                            let v4215 = (v4211 * v4180) / v4187;
                            let v11296 = v11250 * v4215;
                            let v11301 = v11246 * v4184;
                            let v4218 = (v4184 * v4184) / v4187;
                            let v11303 = v11250 * v4218;
                            let v4220 = (v1 - v4215) + (v4218 / v15);
                            let v11309 = (((Lanes([v9406[0], v9406[1], 0.0, v9406[2], v9406[3]])) - (Lanes([0.0, 0.0, v11288[0], 0.0, 0.0]))) / v663) * v4220;
                            let v4222 = (v4183 - v4211) + (v4213 * v4220);
                            let v4223 = v4222 * v4184;
                            let v4224 = v4223 * v4184;
                            let v4226 = (v4224 * v4184) / v4187;
                            let v11323 = v11250 * v4226;
                            let v4230 = ((v4180 * v4209) + (v4226 / v646)) / v4229;
                            let v11331 = ((((v11242 * v4209) + ((((((v11245 * v663) + (Lanes([0.0, 0.0, v11276[0], 0.0, 0.0, 0.0]))) * v4157) + (v11196 * v4207)) - v11248) * v4180)) + (((((((((((v11245 - v11286) + ((Lanes([v11309[0], v11309[1], v11309[2], v11309[3], v11309[4], 0.0])) + (((((((v11286 * v4180) + (v11242 * v4211)) - (Lanes([0.0, 0.0, v11296[0], 0.0, 0.0, 0.0]))) / v4187) * v10360) + ((((v11301 + v11301) - (Lanes([0.0, 0.0, v11303[0], 0.0, 0.0, 0.0]))) / v4187) / v15)) * v4213))) * v4184) + (v11246 * v4222)) * v4184) + (v11246 * v4223)) * v4184) + (v11246 * v4224)) - (Lanes([0.0, 0.0, v11323[0], 0.0, 0.0, 0.0]))) / v4187) / v646)) - (v9753 * v4230)) / v4229;
                            v4425 = v4230;
                            v9755 = v11331;
                        } else {
                            v4425 = v4180;
                            v9755 = v11242;
                        }
                        v4424 = v4425;
                        v9754 = v9755;
                    } else {
                        let v4231 = v13 * v4179;
                        let v11273 = v11241 * v13;
                        v4424 = v4231;
                        v9754 = v11273;
                    }
                    let v4232 = v78 * v1206;
                    let v4233 = v4166 - v3841;
                    let v11334 = (v10804 * v78) * v4233;
                    let v4235 = v4157 + (v4232 * v4233);
                    let v4237 = v1 / v4236;
                    let v4240 = v1 - (v1 - (v4235 * v4237));
                    let v11346 = ((((v11196 + ((Lanes([v11334[0], v11334[1], v11334[2], v11334[3], v11334[4], 0.0])) + ((v11214 - v11036) * v4232))) * v4237) + ((((v9703 * v4237) * v10360) / v4236) * v4235)) * v10360) * v10360;
                    let v4241 = v4240 * v4240;
                    let v11347 = v11346 * v4240;
                    let v11348 = v11347 + v11347;
                    let v4242 = v4241 * v4241;
                    let v11349 = v11348 * v4241;
                    let v4243 = v4242 * v4241;
                    let v11356 = ((((v11349 + v11349) * v4241) + (v11348 * v4242)) * v4241) + (v11348 * v4243);
                    let v4246 = (v4243 * v4241) + v4245;
                    let v4263: f64;
                    let v9756: Lanes<6>;
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
                        let mut v9757: Lanes<6> = Lanes([0.0; 6]);
                        v4252 = v0;
                        v4254 = v4246;
                        v9757 = v11356;
                        loop {
                            let v4253 = if v4252 < v4257 { 1.0 } else { 0.0 };
                            if v4253 == 0.0 {
                                break;
                            }
                            let v4255 = v4254.sqrt();
                            let v11429 = v9757 * (v9353 / (v10405 * v4255));
                            let v4256 = v4252 + v1;
                            v4252 = v4256;
                            v4254 = v4255;
                            v9757 = v11429;
                        }
                        v4263 = v4254;
                        v9756 = v9757;
                    } else {
                        let v4262 = v4246.powf(v4261);
                        let v11360 = v11356 * (v4261 * (v4246.powf(v11357)));
                        v4263 = v4262;
                        v9756 = v11360;
                    }
                    let v4264 = v1 / v4263;
                    let v4266 = v1 - (v4240 * v4264);
                    let v11367 = ((v11346 * v4264) + ((((v9756 * v4264) * v10360) / v4263) * v4240)) * v10360;
                    let v4267 = v1 + v4266;
                    let v11370 = (v11367 * v4267) + (v11367 * v4266);
                    let v4269 = v1 + (v4266 * v4267);
                    let v4271 = if v4267 >= v4270 { 1.0 } else { 0.0 };
                    let v4273: f64;
                    let v9758: Lanes<6>;
                    if v4271 != 0.0 {
                        v4273 = v4267;
                        v9758 = v11367;
                    } else {
                        v4273 = v4272;
                        v9758 = v11032;
                    }
                    let v4431: f64;
                    let v9759: Lanes<6>;
                    if v4191 != 0.0 {
                        let v4276 = if (v4157.abs()) > v23 { 1.0 } else { 0.0 };
                        let v4432: f64;
                        let v9760: Lanes<6>;
                        if v4276 != 0.0 {
                            let v11372 = v11245 * v4183;
                            let v11374 = v11248 * v4186;
                            let v4280 = (v4183 * v4183) + ((v4186 * v4186) / v3523);
                            let v4281 = v4280 * v663;
                            let v11379 = v10380 * v4280;
                            let v4286 = v1128 / v663;
                            let v11390 = v10380 * v4286;
                            let v4287 = v4286 * v4184;
                            let v11395 = (((Lanes([v9406[0], v9406[1], 0.0, v9406[2], v9406[3]])) - (Lanes([0.0, 0.0, v11390[0], 0.0, 0.0]))) / v663) * v4184;
                            let v4289 = (v4287 * v4184) / v4187;
                            let v11402 = v11250 * v4289;
                            let v4291 = (v78 * v4183) + (v4289 / v644);
                            let v4292 = v4291 * v4184;
                            let v4293 = v4292 * v4184;
                            let v4295 = (v4293 * v4184) / v4187;
                            let v11417 = v11250 * v4295;
                            let v4298 = (((v4281 * v4157) - (v4183 * v4186)) - (v4295 / v646)) / v4229;
                            let v11425 = (((((((((v11372 + v11372) + ((v11374 + v11374) / v3523)) * v663) + (Lanes([0.0, 0.0, v11379[0], 0.0, 0.0, 0.0]))) * v4157) + (v11196 * v4281)) - ((v11245 * v4186) + (v11248 * v4183))) - (((((((((((v11245 * v78) + (((((((Lanes([v11395[0], v11395[1], v11395[2], v11395[3], v11395[4], 0.0])) + (v11246 * v4286)) * v4184) + (v11246 * v4287)) - (Lanes([0.0, 0.0, v11402[0], 0.0, 0.0, 0.0]))) / v4187) / v644)) * v4184) + (v11246 * v4291)) * v4184) + (v11246 * v4292)) * v4184) + (v11246 * v4293)) - (Lanes([0.0, 0.0, v11417[0], 0.0, 0.0, 0.0]))) / v4187) / v646)) - (v9753 * v4298)) / v4229;
                            v4432 = v4298;
                            v9760 = v11425;
                        } else {
                            v4432 = v4183;
                            v9760 = v11245;
                        }
                        v4431 = v4432;
                        v9759 = v9760;
                    } else {
                        let v4300 = v4299 * v4181;
                        let v11371 = v11243 * v4299;
                        v4431 = v4300;
                        v9759 = v11371;
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
                    v9710 = v11367;
                    v9711 = v9758;
                    v9712 = v11370;
                    v9713 = v9730;
                    v9714 = v9753;
                    v9715 = v9754;
                    v9716 = v9759;
                    v9717 = v11196;
                } else {
                    v4309 = v0;
                    v4313 = v0;
                    v4316 = v0;
                    v4338 = v4339;
                    v4384 = v0;
                    v4423 = v4426;
                    v4430 = v0;
                    v4447 = v0;
                    v9710 = v11032;
                    v9711 = v11032;
                    v9712 = v11032;
                    v9713 = v9704;
                    v9714 = v11032;
                    v9715 = v9705;
                    v9716 = v11032;
                    v9717 = v11032;
                }
                let v11426 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9678[0]]);
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
                v9422 = v9710;
                v9423 = v9711;
                v9424 = v9712;
                v9425 = v9713;
                v9426 = v9679;
                v9427 = v11049;
                v9428 = v9714;
                v9429 = v9715;
                v9430 = v9716;
                v9431 = v11032;
                v9432 = v11032;
                v9433 = v9717;
                v9434 = v11032;
                v9435 = v10432;
                v9436 = v10427;
                v9437 = v9703;
                v9438 = v10549;
                v9439 = v10630;
                v9440 = v10549;
                v9441 = v9668;
                v9442 = v11426;
                v9443 = v10549;
                v9444 = v11032;
                v9445 = v9706;
                v9446 = v9707;
                v9447 = v11032;
                v9448 = v11032;
                v9449 = v11032;
                v9450 = v9708;
                v9451 = v9709;
                v9452 = v11032;
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
            let v9761: Lanes<6>;
            let v9762: Lanes<6>;
            let v9763: Lanes<6>;
            let v9764: Lanes<6>;
            let v9765: Lanes<6>;
            let v9766: Lanes<6>;
            let v9767: Lanes<6>;
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
            if v4306 != 0.0 {
                let v4317 = v4311 * v4314;
                let v4319 = (v708 * (v13 + v4307)) / v4317;
                let v4320 = v1707 - v4319;
                let v13726 = (((v9422 * v708) - (((v9423 * v4314) + (v9424 * v4311)) * v4319)) / v4317) * v10360;
                let v4322 = if v4320 > v4321 { 1.0 } else { 0.0 };
                let v4324: f64;
                let v9779: Lanes<6>;
                if v4322 != 0.0 {
                    let v4323 = if v70 >= v1 { 1.0 } else { 0.0 };
                    if v4323 != 0.0 {
                    } else {
                    }
                    v4324 = v13;
                    v9779 = v11032;
                } else {
                    v4324 = v4320;
                    v9779 = v13726;
                }
                let v4329 = if v4325 == v0 { 1.0 } else { 0.0 };
                let v4415: f64;
                let v8301: f64;
                let v9780: Lanes<6>;
                let v9781: Lanes<6>;
                if v4329 != 0.0 {
                    let v4335 = if (if v73 < v4330 { 1.0 } else { 0.0 }) != 0.0 && (if v4332 < v4333 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4413: f64;
                    let v8302: f64;
                    let v9782: Lanes<6>;
                    let v9783: Lanes<6>;
                    if v4335 != 0.0 {
                        let v4341 = v4340 + v866;
                        let v13792 = v9426 + (Lanes([v10531[0], v10531[1], 0.0, 0.0, v10531[2], 0.0]));
                        let v4344 = if v4336 > (v4341 - v4342) { 1.0 } else { 0.0 };
                        let v8303: f64;
                        let v9784: Lanes<6>;
                        if v4344 != 0.0 {
                            let v4346 = v4341 - v4345;
                            v8303 = v4346;
                            v9784 = v13792;
                        } else {
                            v8303 = v4336;
                            v9784 = v9425;
                        }
                        v4413 = v0;
                        v8302 = v8303;
                        v9782 = v11032;
                        v9783 = v9784;
                    } else {
                        if v565 != 0.0 {
                        } else {
                        }
                        let v4347 = v1 / v12;
                        let v4353 = (v4351 * v490) + (v4332 * (v4348 * v4347));
                        let v4354 = v1 / v4353;
                        let v4355 = v123 * v4354;
                        let v13732 = (((((v9427 * v4347) * v4332) * v4354) * v10360) / v4353) * v123;
                        let v4357 = v1 - v4356;
                        let v4361 = (v4356 * (v823 + v4340)) + (v4357 * v4336);
                        let v13737 = (((Lanes([v9395[0], v9395[1], 0.0, 0.0, 0.0, 0.0])) + v9426) * v4356) + (v9425 * v4357);
                        let v4362 = v4340 + v866;
                        let v13739 = v9426 + (Lanes([v10531[0], v10531[1], 0.0, 0.0, v10531[2], 0.0]));
                        let v4365 = if v4361 > (v4362 - v4363) { 1.0 } else { 0.0 };
                        let v4368: f64;
                        let v9785: Lanes<6>;
                        if v4365 != 0.0 {
                            let v4367 = v4362 - v4366;
                            v4368 = v4367;
                            v9785 = v13739;
                        } else {
                            v4368 = v4361;
                            v9785 = v13737;
                        }
                        let v4369 = v4368 - v4336;
                        let v13740 = v9785 - v9425;
                        let v13741 = v13740 * v4369;
                        let v4373 = ((v4369 * v4369) + v4371).sqrt();
                        let v13747 = (v13740 + ((v13741 + v13741) * (v9353 / (v10405 * v4373)))) * v13;
                        let v4377 = (v13 * (v4369 + v4373)) + v4376;
                        let v4378 = if v4377 < v0 { 1.0 } else { 0.0 };
                        let v4394: f64;
                        let v9786: Lanes<6>;
                        if v4378 != 0.0 {
                            v4394 = v0;
                            v9786 = v11032;
                        } else {
                            v4394 = v4377;
                            v9786 = v13747;
                        }
                        let v4379 = v663 * v4348;
                        let v13748 = v10380 * v4348;
                        let v4380 = v1 / v4379;
                        let v4385 = v4381 * v4380;
                        let v13757 = (v9428 * v4380) + ((((((Lanes([0.0, 0.0, v13748[0], 0.0, 0.0, 0.0])) + (v9427 * v663)) * v4380) * v10360) / v4379) * v4381);
                        let v4386 = if v4385 < v665 { 1.0 } else { 0.0 };
                        let v4391: f64;
                        let v9787: Lanes<6>;
                        if v4386 != 0.0 {
                            let v13758 = Lanes([0.0, 0.0, v10385[0], 0.0, 0.0, 0.0]);
                            v4391 = v665;
                            v9787 = v13758;
                        } else {
                            v4391 = v4385;
                            v9787 = v13757;
                        }
                        let v4390 = v1 / v136;
                        let v4393 = v78 * (v490 / v123);
                        let v4395 = v4393 * v4394;
                        let v13760 = v9786 * v4393;
                        let v4400 = (((v78 * v4391) + (v4395 * v4355)) + (v4389 * v4355)) * v4390;
                        let v4401 = v4400 * v4355;
                        let v13770 = (((((v9787 * v78) + ((v13760 * v4355) + (v13732 * v4395))) + (v13732 * v4389)) * v4390) * v4355) + (v13732 * v4400);
                        let v4403 = v90 * (v4395 + v4389);
                        let v4404 = v4403 * v4355;
                        let v13778 = v13770 * v4401;
                        let v4408 = ((v4401 * v4401) + (v4404 * v4355)).sqrt();
                        let v4411 = v13 * ((-v4401) + v4408);
                        let v4412 = v921 * v4411;
                        let v13787 = v10585 * v4411;
                        let v13790 = (Lanes([v13787[0], v13787[1], v13787[2], v13787[3], v13787[4], 0.0])) + ((((v13770 * v10360) + (((v13778 + v13778) + (((((v13760 * v90) * v4355) + (v13732 * v4403)) * v4355) + (v13732 * v4404))) * (v9353 / (v10405 * v4408)))) * v13) * v921);
                        v4413 = v4412;
                        v8302 = v4368;
                        v9782 = v13790;
                        v9783 = v9785;
                    }
                    let v4414 = v4413 * v267;
                    let v13793 = v9782 * v267;
                    v4415 = v4414;
                    v8301 = v8302;
                    v9780 = v13793;
                    v9781 = v9783;
                } else {
                    v4415 = v0;
                    v8301 = v8304;
                    v9780 = v11032;
                    v9781 = v9445;
                }
                let v4416 = v136 - v4415;
                let v13794 = v9780 * v10360;
                let v4417 = v139 - v4415;
                let v4418 = if v4416 < v616 { 1.0 } else { 0.0 };
                let v4525: f64;
                let v9788: Lanes<6>;
                if v4418 != 0.0 {
                    v4525 = v616;
                    v9788 = v11032;
                } else {
                    v4525 = v4416;
                    v9788 = v13794;
                }
                let v4420 = (-v168) * v139;
                let v4427 = v4420 * v4421;
                let v13795 = v9429 * v4420;
                let v4433 = v4420 * v4428;
                let v13796 = v9430 * v4420;
                let v4434 = v4433 * v13;
                let v13797 = v13796 * v13;
                let v8485: f64;
                let v8490: f64;
                let v8496: f64;
                let v9789: Lanes<6>;
                let v9790: Lanes<6>;
                let v9791: Lanes<6>;
                if v9 != 0.0 {
                    let v4435 = v4427 * v13;
                    let v13798 = v13795 * v13;
                    let v4437 = v4427 * v4436;
                    let v13799 = v13795 * v4436;
                    let v4444 = ((v13 * (v4438 + v4439)) * v139) * v168;
                    let v13803 = (((v9431 + v9432) * v13) * v139) * v168;
                    v8485 = v4444;
                    v8490 = v4435;
                    v8496 = v4437;
                    v9789 = v13803;
                    v9790 = v13798;
                    v9791 = v13799;
                } else {
                    v8485 = v8486;
                    v8490 = v8491;
                    v8496 = v8497;
                    v9789 = v9447;
                    v9790 = v9448;
                    v9791 = v9449;
                }
                let v4448 = v823 - v4445;
                let v13805 = (Lanes([v9395[0], v9395[1], 0.0, 0.0, 0.0, 0.0])) - v9433;
                let v4452 = (v78 * (v4448 / v78)) / v4451;
                let v13808 = ((v13805 / v78) * v78) / v4451;
                let v4460 = v4457 + (v4452 * v4458);
                let v4462 = v4456 + (v4452 * v4460);
                let v4464 = v4455 + (v4452 * v4462);
                let v4466 = v4454 + (v4452 * v4464);
                let v4468 = v4453 + (v4452 * v4466);
                let v4470 = v1 + (v4452 * v4468);
                let v4471 = v4451 / v4470;
                let v13827 = ((((v13808 * v4468) + (((v13808 * v4466) + (((v13808 * v4464) + (((v13808 * v4462) + (((v13808 * v4460) + ((v13808 * v4458) * v4452)) * v4452)) * v4452)) * v4452)) * v4452)) * v4471) * v10360) / v4470;
                let v4473 = if v4471 < v4472 { 1.0 } else { 0.0 };
                let v4475: f64;
                let v9792: Lanes<6>;
                if v4473 != 0.0 {
                    v4475 = v4474;
                    v9792 = v11032;
                } else {
                    v4475 = v4471;
                    v9792 = v13827;
                }
                let v4476 = v4340 + v4475;
                let v13828 = v9426 + v9792;
                let v4479 = v4428 / v556;
                let v13830 = v9430 / v556;
                let v4481 = v4480 / v4477;
                let v4483 = v4482 / v4477;
                let v4487 = v1 + ((v4336 - v4340) * v4484);
                let v4491 = ((v4481 * (v4421 / v556)) + (v4483 * v4479)) / v4487;
                let v13838 = ((((v9429 / v556) * v4481) + (v13830 * v4483)) - (((v9425 - v9426) * v4484) * v4491)) / v4487;
                let v13839 = v13838 * v4491;
                let v4495 = ((v4491 * v4491) + v4493).sqrt();
                let v13845 = (v13838 + ((v13839 + v13839) * (v9353 / (v10405 * v4495)))) * v13;
                let v4499 = (v13 * (v4491 + v4495)) + v4498;
                let v4500 = if v4499 < v0 { 1.0 } else { 0.0 };
                let v4501: f64;
                let v9793: Lanes<6>;
                if v4500 != 0.0 {
                    v4501 = v0;
                    v9793 = v11032;
                } else {
                    v4501 = v4499;
                    v9793 = v13845;
                }
                let v4503 = v4502 - v1;
                let v4504 = v4501.powf(v4503);
                let v4505 = v4504 * v4501;
                let v4506 = v183 - v1;
                let v4507 = v4501.powf(v4506);
                let v4515 = v4510 + ((v4511 * (v4479 / v206)) / v4513);
                let v4516 = v1 / v4515;
                let v13866 = v10391 * v4505;
                let v4521 = (v4516 + (v702 * v4505)) + ((v4507 * v4501) / v4519);
                let v4522 = v1 / v4521;
                let v4523 = v4522 * v29;
                let v13876 = (((((((((((v13830 / v206) * v4511) / v4513) * v4516) * v10360) / v4515) + ((Lanes([0.0, 0.0, v13866[0], 0.0, 0.0, 0.0])) + ((((v9793 * (v4503 * (v4501.powf((v4503 - v9353))))) * v4501) + (v9793 * v4504)) * v702))) + ((((v9793 * (v4506 * (v4501.powf((v4506 - v9353))))) * v4501) + (v9793 * v4507)) / v4519)) * v4522) * v10360) / v4521) * v29;
                let v4524 = v663 * v4348;
                let v13877 = v10380 * v4348;
                let v4526 = v4524 * v4525;
                let v13883 = (((Lanes([0.0, 0.0, v13877[0], 0.0, 0.0, 0.0])) + (v9427 * v663)) * v4525) + (v9788 * v4524);
                let v13884 = v13883 * v4526;
                let v4530 = ((v4526 * v4526) + v4528).sqrt();
                let v13890 = (v13883 + ((v13884 + v13884) * (v9353 / (v10405 * v4530)))) * v13;
                let v4534 = (v13 * (v4526 + v4530)) + v4533;
                let v4535 = if v4534 < v0 { 1.0 } else { 0.0 };
                let v4536: f64;
                let v9794: Lanes<6>;
                if v4535 != 0.0 {
                    v4536 = v0;
                    v9794 = v11032;
                } else {
                    v4536 = v4534;
                    v9794 = v13890;
                }
                let v4537 = v1 / v4536;
                let v4538 = v4381 * v4537;
                let v13897 = v10404 * v1889;
                let v4540 = (v1889 * v717) / v4523;
                let v13902 = ((v9428 * v4537) + ((((v9794 * v4537) * v10360) / v4536) * v4381)) * v4538;
                let v13904 = (((Lanes([0.0, 0.0, v13897[0], 0.0, 0.0, 0.0])) - (v13876 * v4540)) / v4523) * v4540;
                let v4544 = ((v4538 * v4538) + (v4540 * v4540)).sqrt();
                let v13909 = ((v13902 + v13902) + (v13904 + v13904)) * (v9353 / (v10405 * v4544));
                let v4546 = (v4523 * v4544) / v717;
                let v13913 = v10404 * v4546;
                let v13916 = (((v13876 * v4544) + (v13909 * v4523)) - (Lanes([0.0, 0.0, v13913[0], 0.0, 0.0, 0.0]))) / v717;
                let v4552 = if (if v4547 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v4550 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4560: f64;
                let v9795: Lanes<6>;
                if v4552 != 0.0 {
                    v4560 = v1;
                    v9795 = v11032;
                } else {
                    let v4557 = if (if v4553 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v4555 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4561: f64;
                    let v9796: Lanes<6>;
                    if v4557 != 0.0 {
                        v4561 = v4546;
                        v9796 = v13916;
                    } else {
                        let v4558 = v4548 - v1;
                        let v4559 = v4546.powf(v4558);
                        let v13920 = v13916 * (v4558 * (v4546.powf((v4558 - v9353))));
                        v4561 = v4559;
                        v9796 = v13920;
                    }
                    v4560 = v4561;
                    v9795 = v9796;
                }
                let v13923 = (v13916 * v4560) + (v9795 * v4546);
                let v4563 = v1 + (v4546 * v4560);
                let v4568 = if (if v4564 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v4566 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4582: f64;
                let v9797: Lanes<6>;
                if v4568 != 0.0 {
                    let v4569 = v1 / v4563;
                    let v13939 = ((v13923 * v4569) * v10360) / v4563;
                    v4582 = v4569;
                    v9797 = v13939;
                } else {
                    let v4574 = if (if v4570 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v4572 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4583: f64;
                    let v9798: Lanes<6>;
                    if v4574 != 0.0 {
                        let v4575 = v4563.sqrt();
                        let v4576 = v1 / v4575;
                        let v13936 = (((v13923 * (v9353 / (v10405 * v4575))) * v4576) * v10360) / v4575;
                        v4583 = v4576;
                        v9798 = v13936;
                    } else {
                        let v4579 = (v4577 / v4548) - v1;
                        let v4580 = v4563.powf(v4579);
                        let v4581 = v4563 * v4580;
                        let v13930 = (v13923 * v4580) + ((v13923 * (v4579 * (v4563.powf((v4579 - v9353))))) * v4563);
                        v4583 = v4581;
                        v9798 = v13930;
                    }
                    v4582 = v4583;
                    v9797 = v9798;
                }
                let v4584 = v4523 * v4582;
                let v13942 = (v13876 * v4582) + (v9797 * v4523);
                let v13943 = v10385 * v166;
                let v4586 = (v166 * v665) / v4416;
                let v13947 = ((Lanes([0.0, 0.0, v13943[0], 0.0, 0.0, 0.0])) - (v13794 * v4586)) / v4416;
                let v4587 = v4586 * v4381;
                let v4588 = v4587 * v4584;
                let v13953 = (((v13947 * v4381) + (v9428 * v4586)) * v4584) + (v13942 * v4587);
                let v4592 = if (if v4589 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v212 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4648: f64;
                let v9799: Lanes<6>;
                if v4592 != 0.0 {
                    let v4595 = (v78 * (v13 * v4448)) / v20;
                    let v13956 = ((v13805 * v13) * v78) / v20;
                    let v4603 = v4600 + (v4595 * v4601);
                    let v4605 = v4599 + (v4595 * v4603);
                    let v4607 = v4598 + (v4595 * v4605);
                    let v4609 = v4597 + (v4595 * v4607);
                    let v4611 = v4596 + (v4595 * v4609);
                    let v4613 = v1 + (v4595 * v4611);
                    let v4614 = v20 / v4613;
                    let v4616 = v4340 + v4614;
                    let v13976 = v9426 + (((((v13956 * v4611) + (((v13956 * v4609) + (((v13956 * v4607) + (((v13956 * v4605) + (((v13956 * v4603) + ((v13956 * v4601) * v4595)) * v4595)) * v4595)) * v4595)) * v4595)) * v4614) * v10360) / v4613);
                    let v4617 = v4615 - v4616;
                    let v13977 = v13976 * v10360;
                    let v13978 = v13977 * v4617;
                    let v4621 = ((v4617 * v4617) + v4619).sqrt();
                    let v13984 = (v13977 + ((v13978 + v13978) * (v9353 / (v10405 * v4621)))) * v13;
                    let v4625 = (v13 * (v4617 + v4621)) + v4624;
                    let v4626 = if v4625 < v0 { 1.0 } else { 0.0 };
                    let v4629: f64;
                    let v9800: Lanes<6>;
                    if v4626 != 0.0 {
                        v4629 = v0;
                        v9800 = v11032;
                    } else {
                        v4629 = v4625;
                        v9800 = v13984;
                    }
                    let v4627 = v663 * v216;
                    let v4628 = v1128 * v4627;
                    let v13986 = v9406 * v4627;
                    let v13987 = (v10380 * v216) * v1128;
                    let v4631 = v4629.powf(v4630);
                    let v4632 = v4628 * v4631;
                    let v13995 = ((Lanes([v13986[0], v13986[1], 0.0, v13986[2], v13986[3]])) + (Lanes([0.0, 0.0, v13987[0], 0.0, 0.0]))) * v4631;
                    let v13998 = (Lanes([v13995[0], v13995[1], v13995[2], v13995[3], v13995[4], 0.0])) + ((v9800 * (v4630 * (v4629.powf((v4630 - v9353))))) * v4628);
                    let v13999 = v10531 * v4633;
                    let v4635 = v1 + (v866 * v4633);
                    let v4640: f64;
                    let v9801: Lanes<6>;
                    if v987 != 0.0 {
                        let v4636 = v4616 - v864;
                        let v14002 = v13976 - (Lanes([v10528[0], v10528[1], 0.0, 0.0, v10528[2], 0.0]));
                        v4640 = v4636;
                        v9801 = v14002;
                    } else {
                        let v4638 = v4616 - v4637;
                        let v14000 = v13976 - v9434;
                        v4640 = v4638;
                        v9801 = v14000;
                    }
                    let v4639 = v866 * v221;
                    let v14004 = (v10531 * v221) * v4640;
                    let v4642 = v4635 + (v4639 * v4640);
                    let v4643 = v4632 * v4642;
                    let v14012 = (v13998 * v4642) + (((Lanes([v13999[0], v13999[1], 0.0, 0.0, v13999[2], 0.0])) + ((Lanes([v14004[0], v14004[1], 0.0, 0.0, v14004[2], 0.0])) + (v9801 * v4639))) * v4632);
                    v4648 = v4643;
                    v9799 = v14012;
                } else {
                    v4648 = v0;
                    v9799 = v11032;
                }
                let v4644 = if v222 != v0 { 1.0 } else { 0.0 };
                let v4649: f64;
                let v9802: Lanes<5>;
                if v4644 != 0.0 {
                    let v4645 = v663 * v227;
                    let v4646 = v1128 * v4645;
                    let v14014 = v9406 * v4645;
                    let v14015 = (v10380 * v227) * v1128;
                    let v4647 = v4646 * v866;
                    let v14020 = v10531 * v4646;
                    let v14022 = (((Lanes([v14014[0], v14014[1], 0.0, v14014[2], v14014[3]])) + (Lanes([0.0, 0.0, v14015[0], 0.0, 0.0]))) * v866) + (Lanes([v14020[0], v14020[1], 0.0, 0.0, v14020[2]]));
                    v4649 = v4647;
                    v9802 = v14022;
                } else {
                    v4649 = v0;
                    v9802 = v10549;
                }
                let v4650 = v4648 + v4649;
                let v14024 = v9799 + (Lanes([v9802[0], v9802[1], v9802[2], v9802[3], v9802[4], 0.0]));
                let v4651 = if v4650 > v0 { 1.0 } else { 0.0 };
                let v4655: f64;
                let v9803: Lanes<6>;
                if v4651 != 0.0 {
                    let v4652 = v4445 * v4650;
                    let v4653 = v4586 * v4652;
                    let v4654 = v4653 * v4584;
                    let v14033 = (((v13947 * v4652) + (((v9433 * v4650) + (v14024 * v4445)) * v4586)) * v4584) + (v13942 * v4653);
                    v4655 = v4654;
                    v9803 = v14033;
                } else {
                    v4655 = v0;
                    v9803 = v11032;
                }
                let v4656 = v4588 + v4655;
                let v14034 = v13953 + v9803;
                let v4658 = if v4657 != v0 { 1.0 } else { 0.0 };
                let v4877: f64;
                let v9804: Lanes<6>;
                if v4658 != 0.0 {
                    let v4659 = v245 - v1102;
                    let v4661 = v1 / (v4659 * v4659);
                    let v4662 = v78 * v1101;
                    let v4666 = ((v4662 * (v123 * v1048)) * v516) * v4661;
                    let v4667 = v4666 * v1066;
                    let v14039 = ((((v9405 * v123) * v4662) * v516) * v4661) * v1066;
                    let v14040 = v10695 * v4666;
                    let v4671 = v4668 + (v4669 * v866);
                    let v4672 = v4667 * v4671;
                    let v14046 = (v10531 * v4669) * v4667;
                    let v14048 = (((Lanes([v14039[0], v14039[1], 0.0, v14039[2], v14039[3]])) + (Lanes([v14040[0], v14040[1], v14040[2], 0.0, v14040[3]]))) * v4671) + (Lanes([v14046[0], v14046[1], 0.0, 0.0, v14046[2]]));
                    let v14050 = (v9395 * v4674) * v10360;
                    let v14052 = v10534 + (Lanes([v14050[0], v14050[1], 0.0, 0.0]));
                    let v4679 = ((v867 - v240) + (v4673 - (v4674 * v823))) + v4672;
                    let v14054 = (Lanes([v14052[0], v14052[1], 0.0, v14052[2], v14052[3]])) + v14048;
                    let v4680 = v734 * v1048;
                    let v14055 = v10429 * v1048;
                    let v14056 = v9405 * v734;
                    let v4681 = v4680 * v1048;
                    let v14061 = v9405 * v4680;
                    let v14063 = (((Lanes([0.0, 0.0, v14055[0], 0.0, 0.0])) + (Lanes([v14056[0], v14056[1], 0.0, v14056[2], v14056[3]]))) * v1048) + (Lanes([v14061[0], v14061[1], 0.0, v14061[2], v14061[3]]));
                    let v14065 = v10380 * v4681;
                    let v4683 = (v4681 * v663) * v13;
                    let v14068 = ((v14063 * v663) + (Lanes([0.0, 0.0, v14065[0], 0.0, 0.0]))) * v13;
                    let v14070 = v10380 * v4683;
                    let v4685 = (v4683 * v663) * v78;
                    let v14073 = ((v14068 * v663) + (Lanes([0.0, 0.0, v14070[0], 0.0, 0.0]))) * v78;
                    let v4686 = v663 * v2050;
                    let v14076 = (v10380 * v2050) * v4681;
                    let v14081 = ((Lanes([0.0, 0.0, v10385[0], 0.0, 0.0])) - ((v14063 * v4686) + (Lanes([0.0, 0.0, v14076[0], 0.0, 0.0])))) - v14048;
                    let v4692 = ((((v665 - (v4681 * v4686)) + v240) - v4673) - v4672) + v362;
                    let v14083 = (Lanes([v10534[0], v10534[1], 0.0, v10534[2], v10534[3]])) - v14081;
                    let v4694 = (v867 - v4692) - v3684;
                    let v4695 = if v4692 >= v0 { 1.0 } else { 0.0 };
                    let v4697: f64;
                    if v4695 != 0.0 {
                        v4697 = v1;
                    } else {
                        v4697 = v4696;
                    }
                    let v14084 = v14083 * v4694;
                    let v4699 = v4697 * v90;
                    let v4703 = ((v4694 * v4694) + ((v4699 * v4692) * v3684)).sqrt();
                    let v4710 = ((((v4692 + (v13 * (v4694 + v4703))) - v240) + v4673) + v4672) - v988;
                    let v14096 = Lanes([v9401[0], v9401[1], 0.0, 0.0, v9401[2]]);
                    let v14098 = v10380 * v4710;
                    let v4712 = (v663 * v4710) - v1;
                    let v4713 = v90 / v4685;
                    let v14107 = (((Lanes([0.0, 0.0, v14098[0], 0.0, 0.0])) + ((((v14081 + ((v14083 + (((v14084 + v14084) + ((v14081 * v4699) * v3684)) * (v9353 / (v10405 * v4703)))) * v13)) + v14048) - v14096) * v663)) * v4713) + ((((v14073 * v4713) * v10360) / v4685) * v4712);
                    let v4715 = v1 + (v4712 * v4713);
                    let v14108 = v14107 * v4715;
                    let v4719 = ((v4715 * v4715) + v4717).sqrt();
                    let v14114 = (v14107 + ((v14108 + v14108) * (v9353 / (v10405 * v4719)))) * v13;
                    let v4723 = (v13 * (v4715 + v4719)) + v4722;
                    let v4724 = if v4723 < v0 { 1.0 } else { 0.0 };
                    let v4725: f64;
                    let v9805: Lanes<5>;
                    if v4724 != 0.0 {
                        v4725 = v0;
                        v9805 = v10549;
                    } else {
                        v4725 = v4723;
                        v9805 = v14114;
                    }
                    let v4727 = (v4725 + v362).sqrt();
                    let v4728 = v1 - v4727;
                    let v4730 = v4679 + (v4683 * v4728);
                    let v14122 = v14054 + ((v14068 * v4728) + (((v9805 * (v9353 / (v10405 * v4727))) * v10360) * v4683));
                    let v4731 = v4679 + v362;
                    let v4732 = v78 / v4731;
                    let v4733 = v663 + v4732;
                    let v4734 = v1 / v4733;
                    let v4737 = v1 / v4735;
                    let v14133 = ((v9435 * v4737) * v10360) / v4735;
                    let v4738 = v4737 / v4681;
                    let v4739 = v4679 * v4679;
                    let v14138 = v14054 * v4679;
                    let v4740 = v4738 * v4739;
                    let v4741 = v4740.ln();
                    let v4742 = v4741 * v4734;
                    let v14147 = (((((((Lanes([0.0, 0.0, v14133[0], 0.0, 0.0])) - (v14063 * v4738)) / v4681) * v4739) + ((v14138 + v14138) * v4738)) * (v9353 / v4740)) * v4734) + ((((((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + (((v14054 * v4732) * v10360) / v4731)) * v4734) * v10360) / v4733) * v4741);
                    let v14148 = v14147 - v14122;
                    let v4745 = (v4742 - v4730) - v4744;
                    let v14149 = v14148 * v4745;
                    let v4750 = ((v4745 * v4745) + (v4747 * v4742)).sqrt();
                    let v4753 = v4742 - (v13 * (v4745 + v4750));
                    let v14158 = v14147 - ((v14148 + (((v14149 + v14149) + (v14147 * v4747)) * (v9353 / (v10405 * v4750)))) * v13);
                    let v14159 = v10380 * v4753;
                    let v4755 = (v663 * v4753).exp();
                    let v14164 = v9435 * v4755;
                    let v4757 = v4753 - v988;
                    let v14169 = v10380 * v4757;
                    let v14172 = (Lanes([0.0, 0.0, v14169[0], 0.0, 0.0])) + ((v14158 - v14096) * v663);
                    let v4759 = (v663 * v4757) - v1;
                    let v4760 = v4759 + (v4735 * v4755);
                    let v14173 = v14172 + ((Lanes([0.0, 0.0, v14164[0], 0.0, 0.0])) + ((((Lanes([0.0, 0.0, v14159[0], 0.0, 0.0])) + (v14158 * v663)) * v4755) * v4735));
                    let v14174 = v14173 * v4760;
                    let v4764 = ((v4760 * v4760) + v4762).sqrt();
                    let v14180 = (v14173 + ((v14174 + v14174) * (v9353 / (v10405 * v4764)))) * v13;
                    let v4768 = (v13 * (v4760 + v4764)) + v4767;
                    let v4769 = if v4768 < v0 { 1.0 } else { 0.0 };
                    let v4770: f64;
                    let v9806: Lanes<5>;
                    if v4769 != 0.0 {
                        v4770 = v0;
                        v9806 = v10549;
                    } else {
                        v4770 = v4768;
                        v9806 = v14180;
                    }
                    let v4773 = (v4770 + v4771).sqrt();
                    let v14183 = v9806 * (v9353 / (v10405 * v4773));
                    let v14184 = v14172 * v4759;
                    let v4777 = ((v4759 * v4759) + v4775).sqrt();
                    let v14190 = (v14172 + ((v14184 + v14184) * (v9353 / (v10405 * v4777)))) * v13;
                    let v4781 = (v13 * (v4759 + v4777)) + v4780;
                    let v4782 = if v4781 < v0 { 1.0 } else { 0.0 };
                    let v4783: f64;
                    let v9807: Lanes<5>;
                    if v4782 != 0.0 {
                        v4783 = v0;
                        v9807 = v10549;
                    } else {
                        v4783 = v4781;
                        v9807 = v14190;
                    }
                    let v4786 = (v4783 + v4784).sqrt();
                    let v4789 = v4773 - v4786;
                    let v4790 = v4787 * v4789;
                    let v14195 = v9436 * v4789;
                    let v14198 = (Lanes([0.0, 0.0, v14195[0], 0.0, 0.0])) + ((v14183 - (v9807 * (v9353 / (v10405 * v4786)))) * v4787);
                    let v4791 = v4730 - v4753;
                    let v14199 = v14122 - v14158;
                    let v14200 = v14199 * v4791;
                    let v4795 = ((v4791 * v4791) + v4793).sqrt();
                    let v14206 = (v14199 + ((v14200 + v14200) * (v9353 / (v10405 * v4795)))) * v13;
                    let v4799 = (v13 * (v4791 + v4795)) + v4798;
                    let v4800 = if v4799 < v0 { 1.0 } else { 0.0 };
                    let v4801: f64;
                    let v9808: Lanes<5>;
                    if v4800 != 0.0 {
                        v4801 = v0;
                        v9808 = v10549;
                    } else {
                        v4801 = v4799;
                        v9808 = v14206;
                    }
                    let v4803 = v4801 + v4802;
                    let v4804 = v823 / v4803;
                    let v14209 = (v10567 - (v9808 * v4804)) / v4803;
                    let v4805 = v4804 * v4804;
                    let v14210 = v14209 * v4804;
                    let v14211 = v14210 + v14210;
                    let v4806 = v4805 * v4805;
                    let v14212 = v14211 * v4805;
                    let v4807 = v4806 * v4805;
                    let v14219 = ((((v14212 + v14212) * v4805) + (v14211 * v4806)) * v4805) + (v14211 * v4807);
                    let v4810 = (v4807 * v4805) + v4809;
                    let v4827: f64;
                    let v9809: Lanes<5>;
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
                        let mut v9810: Lanes<5> = Lanes([0.0; 5]);
                        v4816 = v0;
                        v4818 = v4810;
                        v9810 = v14219;
                        loop {
                            let v4817 = if v4816 < v4821 { 1.0 } else { 0.0 };
                            if v4817 == 0.0 {
                                break;
                            }
                            let v4819 = v4818.sqrt();
                            let v18831 = v9810 * (v9353 / (v10405 * v4819));
                            let v4820 = v4816 + v1;
                            v4816 = v4820;
                            v4818 = v4819;
                            v9810 = v18831;
                        }
                        v4827 = v4818;
                        v9809 = v9810;
                    } else {
                        let v4826 = v4810.powf(v4825);
                        let v14223 = v14219 * (v4825 * (v4810.powf(v14220)));
                        v4827 = v4826;
                        v9809 = v14223;
                    }
                    let v4828 = v1 / v4827;
                    let v4829 = v4804 * v4828;
                    let v4831 = (v78 * v262) * v145;
                    let v4832 = v4831 * v665;
                    let v4833 = v4832 * v4584;
                    let v14231 = (v10385 * v4831) * v4584;
                    let v4834 = v4833 * v4790;
                    let v14236 = v14198 * v4833;
                    let v14240 = ((v14209 * v4828) + ((((v9809 * v4828) * v10360) / v4827) * v4804)) * v4834;
                    let v4836 = (v4834 * v4829) / v4525;
                    let v4837 = v4656 + v4836;
                    let v14246 = v14034 + ((((((((Lanes([0.0, 0.0, v14231[0], 0.0, 0.0, 0.0])) + (v13942 * v4832)) * v4790) + (Lanes([v14236[0], v14236[1], v14236[2], v14236[3], v14236[4], 0.0]))) * v4829) + (Lanes([v14240[0], v14240[1], v14240[2], v14240[3], v14240[4], 0.0]))) - (v9788 * v4836)) / v4525);
                    v4877 = v4837;
                    v9804 = v14246;
                } else {
                    v4877 = v4656;
                    v9804 = v14034;
                }
                let v4842 = if (if v4838 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v4840 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8369: f64;
                let v8375: f64;
                let v8379: f64;
                let v8409: f64;
                let v9811: Lanes<6>;
                let v9812: Lanes<6>;
                let v9813: Lanes<6>;
                if v4842 != 0.0 {
                    let v4845 = v4843 * v4843;
                    let v14247 = v9437 * v4843;
                    let v14248 = v14247 + v14247;
                    let v4846 = v78 * v665;
                    let v4847 = v4846 * v1048;
                    let v14250 = (v10385 * v78) * v1048;
                    let v14251 = v9405 * v4846;
                    let v14255 = ((Lanes([0.0, 0.0, v14250[0], 0.0, 0.0])) + (Lanes([v14251[0], v14251[1], 0.0, v14251[2], v14251[3]]))) * v4381;
                    let v4849 = v4845 - (v4847 * v4381);
                    let v14259 = v14248 - ((Lanes([v14255[0], v14255[1], v14255[2], v14255[3], v14255[4], 0.0])) + (v9428 * v4847));
                    let v14260 = v14248 * v4845;
                    let v4853 = ((v4845 * v4845) + v4851).sqrt();
                    let v14266 = (v14248 + ((v14260 + v14260) * (v9353 / (v10405 * v4853)))) * v13;
                    let v4857 = (v13 * (v4845 + v4853)) + v4856;
                    let v4858 = if v4857 < v0 { 1.0 } else { 0.0 };
                    let v4868: f64;
                    let v9814: Lanes<6>;
                    if v4858 != 0.0 {
                        v4868 = v0;
                        v9814 = v11032;
                    } else {
                        v4868 = v4857;
                        v9814 = v14266;
                    }
                    let v14267 = v14259 * v4849;
                    let v4862 = ((v4849 * v4849) + v4860).sqrt();
                    let v14273 = (v14259 + ((v14267 + v14267) * (v9353 / (v10405 * v4862)))) * v13;
                    let v4866 = (v13 * (v4849 + v4862)) + v4865;
                    let v4867 = if v4866 < v0 { 1.0 } else { 0.0 };
                    let v4869: f64;
                    let v9815: Lanes<6>;
                    if v4867 != 0.0 {
                        v4869 = v0;
                        v9815 = v11032;
                    } else {
                        v4869 = v4866;
                        v9815 = v14273;
                    }
                    let v4870 = v4868 - v4869;
                    let v14274 = v9814 - v9815;
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
                    v9811 = v9815;
                    v9812 = v9814;
                    v9813 = v14274;
                } else {
                    v8369 = v0;
                    v8375 = v0;
                    v8379 = v0;
                    v8409 = v0;
                    v9811 = v11032;
                    v9812 = v11032;
                    v9813 = v11032;
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
                v9761 = v9804;
                v9762 = v13828;
                v9763 = v13947;
                v9764 = v13942;
                v9765 = v13909;
                v9766 = v9788;
                v9767 = v13796;
                v9768 = v9781;
                v9769 = v13876;
                v9770 = v9811;
                v9771 = v9812;
                v9772 = v9813;
                v9773 = v13795;
                v9774 = v9789;
                v9775 = v13797;
                v9776 = v9790;
                v9777 = v9791;
                v9778 = v9779;
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
                v9761 = v11032;
                v9762 = v11032;
                v9763 = v11032;
                v9764 = v11032;
                v9765 = v11032;
                v9766 = v11032;
                v9767 = v11032;
                v9768 = v9445;
                v9769 = v11032;
                v9770 = v11032;
                v9771 = v11032;
                v9772 = v11032;
                v9773 = v9446;
                v9774 = v9447;
                v9775 = v11032;
                v9776 = v9448;
                v9777 = v9449;
                v9778 = v11032;
            }
            let v4881 = if (if v4589 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v4879 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5620: f64;
            let v6024: f64;
            let v9816: Lanes<6>;
            let v9817: Lanes<6>;
            if v4881 != 0.0 {
                let v4883 = v1200 - v4882;
                let v4884 = v1143 + v4882;
                let v4886 = v41 / v731;
                let v4888 = (v4886 * v489) / v731;
                let v4889 = v4888.ln();
                let v4890 = v665 * v4889;
                let v14286 = (v10385 * v4889) + ((((((((v10423 * v4886) * v10360) / v731) * v489) - (v10423 * v4888)) / v731) * (v9353 / v4888)) * v665);
                let v4891: f64;
                let v9818: Lanes<6>;
                if v565 != 0.0 {
                    let v14287 = Lanes([v9410[0], v9410[1], v9410[2], 0.0, v9410[3], 0.0]);
                    v4891 = v1037;
                    v9818 = v14287;
                } else {
                    v4891 = v4637;
                    v9818 = v9434;
                }
                let v4898 = v489 + v41;
                let v4900 = (((((v4892 * (v4890 - v4891)) / v123) * v489) * v41) / v4898).sqrt();
                let v4901 = v4900 * v142;
                let v14298 = ((((((((Lanes([0.0, 0.0, v14286[0], 0.0, 0.0, 0.0])) - v9818) * v4892) / v123) * v489) * v41) / v4898) * (v9353 / (v10405 * v4900))) * v142;
                let v4903 = v4902 * v4901;
                let v4905 = v823 + v4901;
                let v14303 = Lanes([v9395[0], v9395[1], 0.0, 0.0, 0.0, 0.0]);
                let v4906 = (v4903 * v4901) / v4905;
                let v14307 = ((((v14298 * v4902) * v4901) + (v14298 * v4903)) - ((v14303 + v14298) * v4906)) / v4905;
                let v4907 = v4883 - v4906;
                let v14308 = Lanes([v10798[0], v10798[1], v10798[2], v10798[3], v10798[4], 0.0]);
                let v4908 = v663 * v4907;
                let v14310 = v10380 * v4907;
                let v14313 = (Lanes([0.0, 0.0, v14310[0], 0.0, 0.0, 0.0])) + ((v14308 - v14307) * v663);
                let v4911 = v1207 * v664;
                let v14316 = v10382 * v1207;
                let v4912 = (v90 * (v4908 - v1)) / v4911;
                let v14319 = ((v10806 * v664) + (Lanes([0.0, 0.0, v14316[0], 0.0, 0.0]))) * v4912;
                let v14322 = ((v14313 * v90) - (Lanes([v14319[0], v14319[1], v14319[2], v14319[3], v14319[4], 0.0]))) / v4911;
                let v4913 = v1 + v4912;
                let v4915 = if v4913 >= v4914 { 1.0 } else { 0.0 };
                let v4917: f64;
                let v9819: Lanes<6>;
                if v4915 != 0.0 {
                    v4917 = v4913;
                    v9819 = v14322;
                } else {
                    v4917 = v4916;
                    v9819 = v11032;
                }
                let v14324 = v10380 * v1207;
                let v4919 = (v1207 * v663) * v13;
                let v4920 = v4917.sqrt();
                let v4921 = v1 - v4920;
                let v14332 = (((v10806 * v663) + (Lanes([0.0, 0.0, v14324[0], 0.0, 0.0]))) * v13) * v4921;
                let v4923 = v4883 + (v4919 * v4921);
                let v14336 = v14308 + ((Lanes([v14332[0], v14332[1], v14332[2], v14332[3], v14332[4], 0.0])) + (((v9819 * (v9353 / (v10405 * v4920))) * v10360) * v4919));
                let v4926 = if v830 < ((v240 + v4884) * v13) { 1.0 } else { 0.0 };
                if v4926 != 0.0 {
                } else {
                }
                let v5086: f64;
                let v5098: f64;
                let v9820: Lanes<6>;
                if v4927 != 0.0 {
                    let v4930 = if (v663 * (v4923 - v4906)) < v96 { 1.0 } else { 0.0 };
                    let v5091: f64;
                    let v5101: f64;
                    let v9821: Lanes<6>;
                    if v4930 != 0.0 {
                        let v4932 = v4931 * v663;
                        let v4933 = v4932 * v1206;
                        let v14399 = (v10380 * v4931) * v1206;
                        let v4934 = v1 / v4933;
                        let v14405 = ((((Lanes([0.0, 0.0, v14399[0], 0.0, 0.0])) + (v10804 * v4932)) * v4934) * v10360) / v4933;
                        let v14406 = v14405 * v96;
                        let v4936 = v1540 + (v96 * v4934);
                        let v14408 = (v14405 * v1540) * v10360;
                        let v4940 = v1153 * v4934;
                        let v4941 = v4940 * v4908;
                        let v14410 = (v14405 * v1153) * v4908;
                        let v14415 = (Lanes([v14408[0], v14408[1], v14408[2], v14408[3], v14408[4], 0.0])) + ((Lanes([v14410[0], v14410[1], v14410[2], v14410[3], v14410[4], 0.0])) + (v14313 * v4940));
                        let v4946 = (v1549 - (v1540 * (v1550 + v4934))) + v4941;
                        let v14416 = v14415 * v4946;
                        let v4948 = v90 * v4936;
                        let v4949 = v4948 * v4936;
                        let v14424 = ((((v14406 * v90) * v4936) + (v14406 * v4948)) * v4936) + (v14406 * v4949);
                        let v4952 = ((v4949 * v4936) + (v4946 * v4946)).sqrt();
                        let v4953 = ((v4937 - (v1540 * v4934)) + v4941) + v4952;
                        let v4954 = v4953.powf(v1562);
                        let v14434 = (v14415 + (((Lanes([v14424[0], v14424[1], v14424[2], v14424[3], v14424[4], 0.0])) + (v14416 + v14416)) * (v9353 / (v10405 * v4952)))) * (v1562 * (v4953.powf(v14431)));
                        let v14435 = v14406 * v1564;
                        let v4956 = v96 * v4954;
                        let v4957 = (v1564 * v4936) / v4956;
                        let v4961 = (v96 - v4957) + (v4959 * v4954);
                        let v14445 = v10385 * v4961;
                        let v4963 = (v4961 * v665) + v4906;
                        let v14448 = (((((((Lanes([v14435[0], v14435[1], v14435[2], v14435[3], v14435[4], 0.0])) - ((v14434 * v96) * v4957)) / v4956) * v10360) + (v14434 * v4959)) * v665) + (Lanes([0.0, 0.0, v14445[0], 0.0, 0.0, 0.0]))) + v14307;
                        v5091 = v4963;
                        v5101 = v4963;
                        v9821 = v14448;
                    } else {
                        let v4966 = if (v830 - v4964) <= v4884 { 1.0 } else { 0.0 };
                        let v5092: f64;
                        let v5102: f64;
                        let v9822: Lanes<6>;
                        if v4966 != 0.0 {
                            let v4984: f64;
                            let v9823: Lanes<6>;
                            if v9 != 0.0 {
                                let v4967 = v1 / v1128;
                                let v4968 = v12 / v123;
                                let v4969 = v1 / v130;
                                let v4971 = (v4967 + v4968) + v4969;
                                let v4972 = v1 / v4971;
                                let v4976 = v4969 + (v13 * v4968);
                                let v4980 = (v4883 - v4973) + (v4976 * (-v4977));
                                let v14388 = ((((((v9406 * v4967) * v10360) / v1128) * v4972) * v10360) / v4971) * v4980;
                                let v4982 = (v4972 * v4980) / v1128;
                                let v14392 = v9406 * v4982;
                                let v4983 = v4883 - v4982;
                                let v14396 = v10798 - ((((Lanes([v14388[0], v14388[1], 0.0, v14388[2], v14388[3]])) + (((v10798 - (Lanes([v9439[0], v9439[1], v9439[2], 0.0, v9439[3]]))) + ((v9440 * v10360) * v4976)) * v4972)) - (Lanes([v14392[0], v14392[1], 0.0, v14392[2], v14392[3]]))) / v1128);
                                let v14397 = Lanes([v14396[0], v14396[1], v14396[2], v14396[3], v14396[4], 0.0]);
                                v4984 = v4983;
                                v9823 = v14397;
                            } else {
                                v4984 = v4923;
                                v9823 = v14336;
                            }
                            v5092 = v4984;
                            v5102 = v4984;
                            v9822 = v9823;
                        } else {
                            let v4985 = v1 / v759;
                            let v14340 = ((v10466 * v4985) * v10360) / v759;
                            let v4986 = v4985 / v1211;
                            let v4987 = v4883 - v4964;
                            let v14345 = v10798 - v9438;
                            let v4988 = v4986 * v4987;
                            let v4989 = v4988 * v4987;
                            let v4990 = v78 / v4987;
                            let v4991 = v663 + v4990;
                            let v4993 = (v4989.ln()) / v4991;
                            let v14361 = (((((((((Lanes([0.0, 0.0, v14340[0], 0.0, 0.0])) - (v9407 * v4986)) / v1211) * v4987) + (v14345 * v4986)) * v4987) + (v14345 * v4988)) * (v9353 / v4989)) - (((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + (((v14345 * v4990) * v10360) / v4987)) * v4993)) / v4991;
                            let v4995 = v4993 + v4994;
                            let v14362 = Lanes([v14361[0], v14361[1], v14361[2], v14361[3], v14361[4], 0.0]);
                            let v14363 = v14362 - v14336;
                            let v4997 = (v4995 - v4923) - v1270;
                            let v4999 = (v90 * v4995) * v1270;
                            let v14365 = (v14361 * v90) * v1270;
                            let v5000 = if v4999 > v0 { 1.0 } else { 0.0 };
                            let v5002: f64;
                            let v9824: Lanes<5>;
                            if v5000 != 0.0 {
                                v5002 = v4999;
                                v9824 = v14365;
                            } else {
                                let v5001 = -v4999;
                                let v14366 = v14365 * v10360;
                                v5002 = v5001;
                                v9824 = v14366;
                            }
                            let v14367 = v14363 * v4997;
                            let v5005 = ((v4997 * v4997) + v5002).sqrt();
                            let v5008 = v4995 - (v13 * (v4997 + v5005));
                            let v14376 = v14362 - ((v14363 + (((v14367 + v14367) + (Lanes([v9824[0], v9824[1], v9824[2], v9824[3], v9824[4], 0.0]))) * (v9353 / (v10405 * v5005)))) * v13);
                            v5092 = v5008;
                            v5102 = v4923;
                            v9822 = v14376;
                        }
                        v5091 = v5092;
                        v5101 = v5102;
                        v9821 = v9822;
                    }
                    let v5087: f64;
                    let v5099: f64;
                    let v9825: Lanes<6>;
                    if v9 != 0.0 {
                        let v5010 = if (v830 - v4964) <= v4884 { 1.0 } else { 0.0 };
                        let v5088: f64;
                        let v5100: f64;
                        let v9826: Lanes<5>;
                        if v5010 != 0.0 {
                            let v5011 = v1 / v1128;
                            let v5012 = v12 / v123;
                            let v5013 = v1 / v130;
                            let v5015 = (v5011 + v5012) + v5013;
                            let v5016 = v1 / v5015;
                            let v5019 = v5013 + (v13 * v5012);
                            let v5022 = (v4883 - v4973) + (v5019 * (-v4977));
                            let v14525 = ((((((v9406 * v5011) * v10360) / v1128) * v5016) * v10360) / v5015) * v5022;
                            let v5024 = (v5016 * v5022) / v1128;
                            let v14529 = v9406 * v5024;
                            let v5025 = v4883 - v5024;
                            let v14533 = v10798 - ((((Lanes([v14525[0], v14525[1], 0.0, v14525[2], v14525[3]])) + (((v10798 - (Lanes([v9439[0], v9439[1], v9439[2], 0.0, v9439[3]]))) + ((v9440 * v10360) * v5019)) * v5016)) - (Lanes([v14529[0], v14529[1], 0.0, v14529[2], v14529[3]]))) / v1128);
                            v5088 = v5025;
                            v5100 = v5025;
                            v9826 = v14533;
                        } else {
                            let v5026 = v1 / v1128;
                            let v5027 = v12 / v123;
                            let v5028 = v1 / v130;
                            let v5030 = (v5026 + v5027) + v5028;
                            let v5031 = v1 / v5030;
                            let v5034 = v5028 + (v13 * v5027);
                            let v5037 = (v4883 - v4973) + (v5034 * (-v4977));
                            let v14460 = ((((((v9406 * v5026) * v10360) / v1128) * v5031) * v10360) / v5030) * v5037;
                            let v5039 = (v5031 * v5037) / v1128;
                            let v14464 = v9406 * v5039;
                            let v5040 = v4883 - v5039;
                            let v14468 = v10798 - ((((Lanes([v14460[0], v14460[1], 0.0, v14460[2], v14460[3]])) + (((v10798 - (Lanes([v9439[0], v9439[1], v9439[2], 0.0, v9439[3]]))) + ((v9440 * v10360) * v5034)) * v5031)) - (Lanes([v14464[0], v14464[1], 0.0, v14464[2], v14464[3]]))) / v1128);
                            let v5041 = v4883 - v4964;
                            let v14469 = v10798 - v9438;
                            let v5042 = if v5041 > v0 { 1.0 } else { 0.0 };
                            let v5089: f64;
                            let v9827: Lanes<5>;
                            if v5042 != 0.0 {
                                let v5043 = v1 / v759;
                                let v14472 = ((v10466 * v5043) * v10360) / v759;
                                let v5044 = v5043 / v1211;
                                let v5045 = v5044 * v5041;
                                let v5046 = v5045 * v5041;
                                let v5047 = v78 / v5041;
                                let v5048 = v663 + v5047;
                                let v5050 = (v5046.ln()) / v5048;
                                let v5052 = (v5050 + v4994) * v1661;
                                let v14493 = ((((((((((Lanes([0.0, 0.0, v14472[0], 0.0, 0.0])) - (v9407 * v5044)) / v1211) * v5041) + (v14469 * v5044)) * v5041) + (v14469 * v5045)) * (v9353 / v5046)) - (((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + (((v14469 * v5047) * v10360) / v5041)) * v5050)) / v5048) * v1661;
                                let v5053 = v5052 - v708;
                                let v5056 = if (if v5040 > v5053 { 1.0 } else { 0.0 }) != 0.0 && v5055 != 0.0 { 1.0 } else { 0.0 };
                                let v5090: f64;
                                let v9828: Lanes<5>;
                                if v5056 != 0.0 {
                                    let v14494 = v14468 - v14493;
                                    let v5058 = (v5040 - v5052) + v708;
                                    let v5059 = v5058 * v5058;
                                    let v14495 = v14494 * v5058;
                                    let v14497 = (v14495 + v14495) * v5059;
                                    let v14498 = v14497 + v14497;
                                    let v5062 = (v5059 * v5059) + v5061;
                                    let v5079: f64;
                                    let v9829: Lanes<5>;
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
                                        let mut v9830: Lanes<5> = Lanes([0.0; 5]);
                                        v5068 = v0;
                                        v5070 = v5062;
                                        v9830 = v14498;
                                        loop {
                                            let v5069 = if v5068 < v5073 { 1.0 } else { 0.0 };
                                            if v5069 == 0.0 {
                                                break;
                                            }
                                            let v5071 = v5070.sqrt();
                                            let v14513 = v9830 * (v9353 / (v10405 * v5071));
                                            let v5072 = v5068 + v1;
                                            v5068 = v5072;
                                            v5070 = v5071;
                                            v9830 = v14513;
                                        }
                                        v5079 = v5070;
                                        v9829 = v9830;
                                    } else {
                                        let v5078 = v5062.powf(v5077);
                                        let v14502 = v14498 * (v5077 * (v5062.powf(v14499)));
                                        v5079 = v5078;
                                        v9829 = v14502;
                                    }
                                    let v5080 = v1 / v5079;
                                    let v5081 = v5058 * v708;
                                    let v5083 = v5053 + (v5081 * v5080);
                                    let v14510 = v14493 + (((v14494 * v708) * v5080) + ((((v9829 * v5080) * v10360) / v5079) * v5081));
                                    v5090 = v5083;
                                    v9828 = v14510;
                                } else {
                                    v5090 = v5040;
                                    v9828 = v14468;
                                }
                                v5089 = v5090;
                                v9827 = v9828;
                            } else {
                                v5089 = v5040;
                                v9827 = v14468;
                            }
                            v5088 = v5089;
                            v5100 = v5040;
                            v9826 = v9827;
                        }
                        let v14534 = Lanes([v9826[0], v9826[1], v9826[2], v9826[3], v9826[4], 0.0]);
                        v5087 = v5088;
                        v5099 = v5100;
                        v9825 = v14534;
                    } else {
                        v5087 = v5091;
                        v5099 = v5101;
                        v9825 = v9821;
                    }
                    v5086 = v5087;
                    v5098 = v5099;
                    v9820 = v9825;
                } else {
                    let v14337 = Lanes([v9441[0], v9441[1], v9441[2], v9441[3], v9441[4], 0.0]);
                    v5086 = v5093;
                    v5098 = v4923;
                    v9820 = v14337;
                }
                let v5085 = v4906 + v5084;
                let v5096 = if v5086 < v5085 { 1.0 } else { 0.0 };
                let v5097: f64;
                let v9831: Lanes<6>;
                if v5096 != 0.0 {
                    v5097 = v5085;
                    v9831 = v14307;
                } else {
                    v5097 = v5086;
                    v9831 = v9820;
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
                let v9832: Lanes<6>;
                if v5130 != 0.0 {
                    let v5133 = if v830 < ((v1205 + v4906) + v4882) { 1.0 } else { 0.0 };
                    let v5366: f64;
                    let v9833: Lanes<6>;
                    if v5133 != 0.0 {
                        let v5134 = v78 * v665;
                        let v5136 = (-v367) / v1206;
                        let v5137 = v5136.ln();
                        let v5138 = v5134 * v5137;
                        let v14766 = (v10385 * v78) * v5137;
                        let v14769 = (Lanes([0.0, 0.0, v14766[0], 0.0, 0.0])) + (((((v10804 * v5136) * v10360) / v1206) * (v9353 / v5136)) * v5134);
                        let v5139 = v663 * v750;
                        let v5140 = v1 / v5139;
                        let v5141 = v5140 * v1128;
                        let v14776 = (((((v10380 * v750) + (v10455 * v663)) * v5140) * v10360) / v5139) * v1128;
                        let v14777 = v9406 * v5140;
                        let v14780 = (Lanes([0.0, 0.0, v14776[0], 0.0, 0.0])) + (Lanes([v14777[0], v14777[1], 0.0, v14777[2], v14777[3]]));
                        let v14781 = v14780 * v5142;
                        let v5144 = v78 + (v5142 * v5141);
                        let v5145 = v91 * v5144;
                        let v5146 = v5145 * v5144;
                        let v5147 = v5146 * v5144;
                        let v14788 = ((((v14781 * v91) * v5144) + (v14781 * v5145)) * v5144) + (v14781 * v5146);
                        let v5148 = v4908 - v78;
                        let v5149 = v3500 * v5141;
                        let v5150 = v5149 * v5148;
                        let v14790 = (v14780 * v3500) * v5148;
                        let v14793 = (Lanes([v14790[0], v14790[1], v14790[2], v14790[3], v14790[4], 0.0])) + (v14313 * v5149);
                        let v5152 = v5151 - v5150;
                        let v14794 = v14793 * v10360;
                        let v5153 = v5152 * v5152;
                        let v14795 = v14794 * v5152;
                        let v14796 = v14795 + v14795;
                        let v5155 = if v5147 < (v5153 * v3506) { 1.0 } else { 0.0 };
                        let v5167: f64;
                        let v9834: Lanes<6>;
                        if v5155 != 0.0 {
                            let v14803 = v14788 * v13;
                            let v5159 = (v13 * v5147) / v5152;
                            let v5161 = ((v5156 + v5152) + v5159) + v5150;
                            let v14809 = (v14794 + (((Lanes([v14803[0], v14803[1], v14803[2], v14803[3], v14803[4], 0.0])) - (v14794 * v5159)) / v5152)) + v14793;
                            v5167 = v5161;
                            v9834 = v14809;
                        } else {
                            let v5163 = (v5147 + v5153).sqrt();
                            let v5166 = (v5164 + v5163) + v5150;
                            let v14802 = (((Lanes([v14788[0], v14788[1], v14788[2], v14788[3], v14788[4], 0.0])) + v14796) * (v9353 / (v10405 * v5163))) + v14793;
                            v5167 = v5166;
                            v9834 = v14802;
                        }
                        let v5168 = v5167.powf(v1562);
                        let v14813 = v9834 * (v1562 * (v5167.powf(v14810)));
                        let v14815 = (v14780 * v3523) * v10360;
                        let v5174 = v748 * v5168;
                        let v5176 = ((v5169 - (v3523 * v5141)) + (v78 * v5168)) + (v5174 * v5168);
                        let v5177 = v1 / v5168;
                        let v5178 = v5176 * v5177;
                        let v14831 = v10385 * v5178;
                        let v5181 = ((v5178 * v665) + v4906) - v4906;
                        let v14835 = ((((((((Lanes([v14815[0], v14815[1], v14815[2], v14815[3], v14815[4], 0.0])) + (v14813 * v78)) + (((v14813 * v748) * v5168) + (v14813 * v5174))) * v5177) + ((((v14813 * v5177) * v10360) / v5168) * v5176)) * v665) + (Lanes([0.0, 0.0, v14831[0], 0.0, 0.0, 0.0]))) + v14307) - v14307;
                        let v5182 = v5181 / v5138;
                        let v14836 = v14769 * v5182;
                        let v14840 = ((v14835 - (Lanes([v14836[0], v14836[1], v14836[2], v14836[3], v14836[4], 0.0]))) / v5138) * v5182;
                        let v5185 = (v1 + (v5182 * v5182)).sqrt();
                        let v5186 = v5181 / v5185;
                        let v5187 = v5186 + v4906;
                        let v14848 = ((v14835 - (((v14840 + v14840) * (v9353 / (v10405 * v5185))) * v5186)) / v5185) + v14307;
                        v5366 = v5187;
                        v9833 = v14848;
                    } else {
                        let v5188 = v4906 - v4994;
                        let v14535 = v10380 * v5188;
                        let v5190 = (v663 * v5188).exp();
                        let v14539 = ((Lanes([0.0, 0.0, v14535[0], 0.0, 0.0, 0.0])) + (v14307 * v663)) * v5190;
                        let v5194 = (((v490 * v12) * v12) / v78) / v123;
                        let v5197 = ((v78 * v663) * v5194).sqrt();
                        let v14544 = ((v10380 * v78) * v5194) * (v9353 / (v10405 * v5197));
                        let v5198 = v5197.exp();
                        let v5200 = (-v5197).exp();
                        let v5202 = (v5198 + v5200) / v78;
                        let v5204 = (v5202.ln()) / v5194;
                        let v14552 = ((((v14544 * v5198) + ((v14544 * v10360) * v5200)) / v78) * (v9353 / v5202)) / v5194;
                        let mut v5205: f64 = 0.0;
                        let mut v5208: f64 = 0.0;
                        let mut v5296: f64 = 0.0;
                        let mut v9835: Lanes<6> = Lanes([0.0; 6]);
                        v5205 = v1;
                        v5208 = v5097;
                        v5296 = v0;
                        v9835 = v9831;
                        loop {
                            let v5207 = if v5205 <= v5206 { 1.0 } else { 0.0 };
                            if v5207 == 0.0 {
                                break;
                            }
                            let v5209 = v5208 - v4906;
                            let v14553 = v9835 - v14307;
                            let v5210 = v663 * v5209;
                            let v14554 = v10380 * v5209;
                            let v14557 = (Lanes([0.0, 0.0, v14554[0], 0.0, 0.0, 0.0])) + (v14553 * v663);
                            let v5211 = v5209 - v5194;
                            let v5212 = v5204 * v5211;
                            let v14558 = v14552 * v5211;
                            let v14561 = (Lanes([0.0, 0.0, v14558[0], 0.0, 0.0, 0.0])) + (v14553 * v5204);
                            let v5213 = if v5212 < v2535 { 1.0 } else { 0.0 };
                            let v5223: f64;
                            let v5227: f64;
                            let v9836: Lanes<6>;
                            let v9837: Lanes<6>;
                            if v5213 != 0.0 {
                                let v5214 = v5212.exp();
                                let v14562 = v14561 * v5214;
                                let v5217 = ((-v5204) * v5194).exp();
                                let v14565 = ((v14552 * v10360) * v5194) * v5217;
                                let v14567 = v14562 - (Lanes([0.0, 0.0, v14565[0], 0.0, 0.0, 0.0]));
                                let v5219 = v1 + (v5214 - v5217);
                                let v5221 = (v5219.ln()) / v5204;
                                let v14570 = v14552 * v5221;
                                let v14573 = ((v14567 * (v9353 / v5219)) - (Lanes([0.0, 0.0, v14570[0], 0.0, 0.0, 0.0]))) / v5204;
                                let v5222 = v5214 / v5219;
                                let v14576 = (v14562 - (v14567 * v5222)) / v5219;
                                v5223 = v5221;
                                v5227 = v5222;
                                v9836 = v14573;
                                v9837 = v14576;
                            } else {
                                v5223 = v5211;
                                v5227 = v1;
                                v9836 = v14553;
                                v9837 = v11032;
                            }
                            let v5224 = v663 * v5223;
                            let v14577 = v10380 * v5223;
                            let v14580 = (Lanes([0.0, 0.0, v14577[0], 0.0, 0.0, 0.0])) + (v9836 * v663);
                            let v5225 = v5210.abs();
                            let v5226 = if v5225 < v3672 { 1.0 } else { 0.0 };
                            let v5300: f64;
                            let v5304: f64;
                            let v9838: Lanes<6>;
                            let v9839: Lanes<6>;
                            if v5226 != 0.0 {
                                let v14683 = v9837 * v5227;
                                let v5231 = ((v1 - (v5227 * v5227)) / v78).sqrt();
                                let v14689 = (((v14683 + v14683) * v10360) / v78) * (v9353 / (v10405 * v5231));
                                let v5232 = v5210 * v5231;
                                let v14692 = (v14557 * v5231) + (v14689 * v5210);
                                let v5233 = v663 * v5231;
                                let v14693 = v10380 * v5231;
                                let v14696 = (Lanes([0.0, 0.0, v14693[0], 0.0, 0.0, 0.0])) + (v14689 * v663);
                                let v5234 = if v5210 < v0 { 1.0 } else { 0.0 };
                                let v5301: f64;
                                let v5305: f64;
                                let v9840: Lanes<6>;
                                let v9841: Lanes<6>;
                                if v5234 != 0.0 {
                                    let v5235 = -v5232;
                                    let v14697 = v14692 * v10360;
                                    let v5236 = -v5233;
                                    let v14698 = v14696 * v10360;
                                    v5301 = v5235;
                                    v5305 = v5236;
                                    v9840 = v14697;
                                    v9841 = v14698;
                                } else {
                                    v5301 = v5232;
                                    v5305 = v5233;
                                    v9840 = v14692;
                                    v9841 = v14696;
                                }
                                v5300 = v5301;
                                v5304 = v5305;
                                v9838 = v9840;
                                v9839 = v9841;
                            } else {
                                let v5237 = if v5225 < v3684 { 1.0 } else { 0.0 };
                                let v5302: f64;
                                let v5306: f64;
                                let v9842: Lanes<6>;
                                let v9843: Lanes<6>;
                                if v5237 != 0.0 {
                                    let v14605 = v14557 * v5210;
                                    let v5239 = (v5210 * v5210) / v78;
                                    let v5240 = v5210 / v96;
                                    let v14608 = v14557 / v96;
                                    let v5241 = v5210 / v90;
                                    let v14609 = v14557 / v90;
                                    let v5243 = v1 - (v5210 / v644);
                                    let v5245 = v1 - (v5241 * v5243);
                                    let v5247 = v1 - (v5240 * v5245);
                                    let v5249 = v5210 / v78;
                                    let v5250 = v1 - v5241;
                                    let v5252 = v1 - (v5240 * v5250);
                                    let v5254 = v1 - (v5249 * v5252);
                                    let v14636 = v14580 * v5224;
                                    let v5257 = (v5224 * v5224) / v78;
                                    let v5258 = v5224 / v96;
                                    let v14639 = v14580 / v96;
                                    let v5259 = v5224 / v90;
                                    let v14640 = v14580 / v90;
                                    let v5261 = v1 - (v5224 / v644);
                                    let v5263 = v1 - (v5259 * v5261);
                                    let v5265 = v1 - (v5258 * v5263);
                                    let v5267 = v5224 / v78;
                                    let v5268 = v1 - v5259;
                                    let v5270 = v1 - (v5258 * v5268);
                                    let v5272 = v1 - (v5267 * v5270);
                                    let v5273 = v5224 * v5272;
                                    let v5275 = ((v5239 * v5247) - (v5257 * v5265)).sqrt();
                                    let v14670 = (((((v14605 + v14605) / v78) * v5247) + ((((v14608 * v5245) + ((((v14609 * v5243) + (((v14557 / v644) * v10360) * v5241)) * v10360) * v5240)) * v10360) * v5239)) - ((((v14636 + v14636) / v78) * v5265) + ((((v14639 * v5263) + ((((v14640 * v5261) + (((v14580 / v644) * v10360) * v5259)) * v10360) * v5258)) * v10360) * v5257))) * (v9353 / (v10405 * v5275));
                                    let v5276 = v663 * v13;
                                    let v5278 = (v5210 * v5254) - (v5227 * v5273);
                                    let v14676 = (v10380 * v13) * v5278;
                                    let v5280 = (v5276 * v5278) / v5275;
                                    let v14682 = (((Lanes([0.0, 0.0, v14676[0], 0.0, 0.0, 0.0])) + ((((v14557 * v5254) + (((((v14557 / v78) * v5252) + ((((v14608 * v5250) + ((v14609 * v10360) * v5240)) * v10360) * v5249)) * v10360) * v5210)) - ((v9837 * v5273) + (((v14580 * v5272) + (((((v14580 / v78) * v5270) + ((((v14639 * v5268) + ((v14640 * v10360) * v5258)) * v10360) * v5267)) * v10360) * v5224)) * v5227))) * v5276)) - (v14670 * v5280)) / v5275;
                                    v5302 = v5275;
                                    v5306 = v5280;
                                    v9842 = v14670;
                                    v9843 = v14682;
                                } else {
                                    let v5282 = (-v5210).exp();
                                    let v14582 = (v14557 * v10360) * v5282;
                                    let v5284 = (-v5224).exp();
                                    let v14584 = (v14580 * v10360) * v5284;
                                    let v5288 = ((v5210 - v5224) + (v5282 - v5284)).sqrt();
                                    let v14590 = ((v14557 - v14580) + (v14582 - v14584)) * (v9353 / (v10405 * v5288));
                                    let v5289 = v663 * v13;
                                    let v5291 = v1 - v5284;
                                    let v5293 = (v1 - v5282) - (v5227 * v5291);
                                    let v14598 = (v10380 * v13) * v5293;
                                    let v5295 = (v5289 * v5293) / v5288;
                                    let v14604 = (((Lanes([0.0, 0.0, v14598[0], 0.0, 0.0, 0.0])) + (((v14582 * v10360) - ((v9837 * v5291) + ((v14584 * v10360) * v5227))) * v5289)) - (v14590 * v5295)) / v5288;
                                    v5302 = v5288;
                                    v5306 = v5295;
                                    v9842 = v14590;
                                    v9843 = v14604;
                                }
                                v5300 = v5302;
                                v5304 = v5306;
                                v9838 = v9842;
                                v9839 = v9843;
                            }
                            let v5297 = if v5296 == v1 { 1.0 } else { 0.0 };
                            let v5298 = if v5210 < v0 { 1.0 } else { 0.0 };
                            let v5299 = if v5297 != 0.0 && v5298 != 0.0 { 1.0 } else { 0.0 };
                            if v5299 != 0.0 {
                            } else {
                            }
                            let v5329: f64;
                            let v5333: f64;
                            let v9844: Lanes<6>;
                            let v9845: Lanes<6>;
                            if v5298 != 0.0 {
                                let v5303 = -v5300;
                                let v14735 = v9838 * v10360;
                                let v5307 = -v5304;
                                let v14736 = v9839 * v10360;
                                v5329 = v5303;
                                v5333 = v5307;
                                v9844 = v14735;
                                v9845 = v14736;
                            } else {
                                let v5308 = if v5210 < v117 { 1.0 } else { 0.0 };
                                let v5330: f64;
                                let v5334: f64;
                                let v9846: Lanes<6>;
                                let v9847: Lanes<6>;
                                if v5308 != 0.0 {
                                    v5330 = v5300;
                                    v5334 = v5304;
                                    v9846 = v9838;
                                    v9847 = v9839;
                                } else {
                                    let v5309 = v5208 - v4994;
                                    let v14699 = v10380 * v5309;
                                    let v5311 = (v663 * v5309).exp();
                                    let v14703 = ((Lanes([0.0, 0.0, v14699[0], 0.0, 0.0, 0.0])) + (v9835 * v663)) * v5311;
                                    let v5312 = v5210 + v1;
                                    let v5314 = v5311 - (v5190 * v5312);
                                    let v14708 = v10466 * v5314;
                                    let v5316 = v759 * v663;
                                    let v5317 = v5311 - v5190;
                                    let v14716 = ((v10466 * v663) + (v10380 * v759)) * v5317;
                                    let v14720 = v9838 * v5300;
                                    let v5321 = ((v5300 * v5300) + (v759 * v5314)).sqrt();
                                    let v14725 = ((v14720 + v14720) + ((Lanes([0.0, 0.0, v14708[0], 0.0, 0.0, 0.0])) + ((v14703 - ((v14539 * v5312) + (v14557 * v5190))) * v759))) * (v9353 / (v10405 * v5321));
                                    let v5322 = v78 * v5304;
                                    let v5326 = (v13 * ((v5322 * v5300) + (v5316 * v5317))) / v5321;
                                    let v14734 = ((((((v9839 * v78) * v5300) + (v9838 * v5322)) + ((Lanes([0.0, 0.0, v14716[0], 0.0, 0.0, 0.0])) + ((v14703 - v14539) * v5316))) * v13) - (v14725 * v5326)) / v5321;
                                    v5330 = v5321;
                                    v5334 = v5326;
                                    v9846 = v14725;
                                    v9847 = v14734;
                                }
                                v5329 = v5330;
                                v5333 = v5334;
                                v9844 = v9846;
                                v9845 = v9847;
                            }
                            let v14737 = v10798 * v10360;
                            let v14740 = v10804 * v5329;
                            let v5332 = ((-v4883) + v5208) + (v1206 * v5329);
                            let v14744 = ((Lanes([v14737[0], v14737[1], v14737[2], v14737[3], v14737[4], 0.0])) + v9835) + ((Lanes([v14740[0], v14740[1], v14740[2], v14740[3], v14740[4], 0.0])) + (v9844 * v1206));
                            let v14745 = v10804 * v5333;
                            let v14748 = (Lanes([v14745[0], v14745[1], v14745[2], v14745[3], v14745[4], 0.0])) + (v9845 * v1206);
                            let v5336 = v1 + (v1206 * v5333);
                            let v5359: f64;
                            let v5361: f64;
                            let v5362: f64;
                            let v9848: Lanes<6>;
                            if v5297 != 0.0 {
                                v5359 = v5337;
                                v5361 = v5208;
                                v5362 = v5296;
                                v9848 = v9835;
                            } else {
                                let v5339 = (-v5332) / v5336;
                                let v14752 = ((v14744 * v10360) - (v14748 * v5339)) / v5336;
                                let v5341 = v5208.abs();
                                let v14756 = v9835 * ((v10405 * (if v5208 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                                let v5342 = if v1 >= v5341 { 1.0 } else { 0.0 };
                                let v5343: f64;
                                let v9849: Lanes<6>;
                                if v5342 != 0.0 {
                                    v5343 = v1;
                                    v9849 = v11032;
                                } else {
                                    v5343 = v5341;
                                    v9849 = v14756;
                                }
                                let v5345 = v5340 * (v1 + v5343);
                                let v14757 = v9849 * v5340;
                                let v5347 = if (v5339.abs()) > v5345 { 1.0 } else { 0.0 };
                                let v5352: f64;
                                let v9850: Lanes<6>;
                                if v5347 != 0.0 {
                                    let v5348 = if v5339 >= v0 { 1.0 } else { 0.0 };
                                    let v5350: f64;
                                    if v5348 != 0.0 {
                                        v5350 = v1;
                                    } else {
                                        v5350 = v5349;
                                    }
                                    let v5351 = v5345 * v5350;
                                    let v14758 = v14757 * v5350;
                                    v5352 = v5351;
                                    v9850 = v14758;
                                } else {
                                    v5352 = v5339;
                                    v9850 = v14752;
                                }
                                let v5353 = v5208 + v5352;
                                let v14759 = v9835 + v9850;
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
                                v9848 = v14759;
                            }
                            let v5360 = v5359 + v1;
                            v5205 = v5360;
                            v5208 = v5361;
                            v5296 = v5362;
                            v9835 = v9848;
                        }
                        v5366 = v5208;
                        v9833 = v9835;
                    }
                    v5365 = v5366;
                    v9832 = v9833;
                } else {
                    v5365 = v5097;
                    v9832 = v9831;
                }
                let v5364 = -v663;
                let v5367 = v5365 - v4906;
                let v14850 = v9832 - v14307;
                let v5368 = v5364 * v5367;
                let v14851 = (v10380 * v10360) * v5367;
                let v14854 = (Lanes([0.0, 0.0, v14851[0], 0.0, 0.0, 0.0])) + (v14850 * v5364);
                let v5369 = if v5368 >= v0 { 1.0 } else { 0.0 };
                let v5371: f64;
                if v5369 != 0.0 {
                    v5371 = v1;
                } else {
                    v5371 = v5370;
                }
                let v5372 = v5371 * v5368;
                let v14855 = v14854 * v5371;
                let v5373 = v5368.exp();
                let v5375 = (v5373 - v1) - v5368;
                let v14857 = (v14854 * v5373) - v14854;
                let v5376 = if v5368 > v117 { 1.0 } else { 0.0 };
                let v5394: f64;
                let v9851: Lanes<6>;
                if v5376 != 0.0 {
                    let v5377 = -v750;
                    let v5378 = v5375.sqrt();
                    let v5379 = v5377 * v5378;
                    let v14882 = (v10455 * v10360) * v5378;
                    let v14885 = (Lanes([0.0, 0.0, v14882[0], 0.0, 0.0, 0.0])) + ((v14857 * (v9353 / (v10405 * v5378))) * v5377);
                    v5394 = v5379;
                    v9851 = v14885;
                } else {
                    let v5380 = if v5372 > v117 { 1.0 } else { 0.0 };
                    let v5395: f64;
                    let v9852: Lanes<6>;
                    if v5380 != 0.0 {
                        let v5381 = v5375.sqrt();
                        let v5382 = v750 * v5381;
                        let v14874 = v10455 * v5381;
                        let v14877 = (Lanes([0.0, 0.0, v14874[0], 0.0, 0.0, 0.0])) + ((v14857 * (v9353 / (v10405 * v5381))) * v750);
                        v5395 = v5382;
                        v9852 = v14877;
                    } else {
                        let v5383 = -v5371;
                        let v5386 = (v5383 * v5372) * v5385;
                        let v5387 = v5372 * v1562;
                        let v5389 = v1 + (v2050 * v5372);
                        let v5392 = (v1 + (v5387 * v5389)).sqrt();
                        let v5393 = v5386 * v5392;
                        let v14870 = (((v14855 * v5383) * v5385) * v5392) + (((((v14855 * v1562) * v5389) + ((v14855 * v2050) * v5387)) * (v9353 / (v10405 * v5392))) * v5386);
                        v5395 = v5393;
                        v9852 = v14870;
                    }
                    v5394 = v5395;
                    v9851 = v9852;
                }
                let v14886 = v9851 * v5394;
                let v5399 = ((v5394 * v5394) + v5397).sqrt();
                let v14892 = (v9851 + ((v14886 + v14886) * (v9353 / (v10405 * v5399)))) * v13;
                let v5403 = (v13 * (v5394 + v5399)) + v5402;
                let v5404 = if v5403 < v0 { 1.0 } else { 0.0 };
                let v5405: f64;
                let v9853: Lanes<6>;
                if v5404 != 0.0 {
                    v5405 = v0;
                    v9853 = v11032;
                } else {
                    v5405 = v5403;
                    v9853 = v14892;
                }
                let v5406 = v5405 / v490;
                let v14893 = v9853 / v490;
                let v5407 = v5406 - v4885;
                let v5408 = v5406 * v20;
                let v14894 = v14893 * v20;
                let v14895 = v14893 * v5407;
                let v5410 = v90 * v5408;
                let v5413 = ((v5407 * v5407) + (v5410 * v5408)).sqrt();
                let v5417 = (v13 * (v5407 + v5413)) + (v535 * v5408);
                let v14908 = ((v14893 + (((v14895 + v14895) + (((v14894 * v90) * v5408) + (v14894 * v5410))) * (v9353 / (v10405 * v5413)))) * v13) + (v14894 * v535);
                let v5418 = if v5417 < v0 { 1.0 } else { 0.0 };
                let v5419: f64;
                let v9854: Lanes<6>;
                if v5418 != 0.0 {
                    v5419 = v0;
                    v9854 = v11032;
                } else {
                    v5419 = v5417;
                    v9854 = v14908;
                }
                let v5420 = v5419 / v5406;
                let v5422 = (v5420 * v5419) / v5406;
                let v5424 = (v5367 * v5422) + v4906;
                let v14921 = ((v14850 * v5422) + (((((((v9854 - (v14893 * v5420)) / v5406) * v5419) + (v9854 * v5420)) - (v14893 * v5422)) / v5406) * v5367)) + v14307;
                let v14922 = v10380 * v5424;
                let v5426 = (v663 * v5424).exp();
                let v5427 = v5424 - v823;
                let v14928 = v10380 * v5427;
                let v5429 = (v663 * v5427).exp();
                let v5430 = v5426 - v5429;
                let v14933 = (((Lanes([0.0, 0.0, v14922[0], 0.0, 0.0, 0.0])) + (v14921 * v663)) * v5426) - (((Lanes([0.0, 0.0, v14928[0], 0.0, 0.0, 0.0])) + ((v14921 - v14303) * v663)) * v5429);
                let v5434 = ((v5431 * v41) * v123).sqrt();
                let v5435 = v5434 * v732;
                let v14934 = v10426 * v5434;
                let v5436 = v5424 - v4906;
                let v5437 = v663 * v5436;
                let v14936 = v10380 * v5436;
                let v14939 = (Lanes([0.0, 0.0, v14936[0], 0.0, 0.0, 0.0])) + ((v14921 - v14307) * v663);
                let v5438 = v1889 * v663;
                let v14940 = v10380 * v1889;
                let v5441 = if (if v5437 < v5438 { 1.0 } else { 0.0 }) != 0.0 && (if v5438 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5467: f64;
                let v9855: Lanes<6>;
                if v5441 != 0.0 {
                    let v5442 = v5438 - v5437;
                    let v14941 = Lanes([0.0, 0.0, v14940[0], 0.0, 0.0, 0.0]);
                    let v14942 = v14941 - v14939;
                    let v14943 = v14942 * v5442;
                    let v14945 = v14940 * v5438;
                    let v14946 = v14945 + v14945;
                    let v5445 = (v5442 * v5442) + (v5438 * v5438);
                    let v14948 = (v14943 + v14943) + (Lanes([0.0, 0.0, v14946[0], 0.0, 0.0, 0.0]));
                    let v5462: f64;
                    let v9856: Lanes<6>;
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
                        let mut v9857: Lanes<6> = Lanes([0.0; 6]);
                        v5451 = v0;
                        v5453 = v5445;
                        v9857 = v14948;
                        loop {
                            let v5452 = if v5451 < v5456 { 1.0 } else { 0.0 };
                            if v5452 == 0.0 {
                                break;
                            }
                            let v5454 = v5453.sqrt();
                            let v18828 = v9857 * (v9353 / (v10405 * v5454));
                            let v5455 = v5451 + v1;
                            v5451 = v5455;
                            v5453 = v5454;
                            v9857 = v18828;
                        }
                        v5462 = v5453;
                        v9856 = v9857;
                    } else {
                        let v5461 = v5445.sqrt();
                        let v14952 = v14948 * (v5460 * (v5445.powf(v14949)));
                        v5462 = v5461;
                        v9856 = v14952;
                    }
                    let v5463 = v1 / v5462;
                    let v5464 = v5442 * v5438;
                    let v14957 = v14940 * v5442;
                    let v5466 = v5438 - (v5464 * v5463);
                    let v14963 = v14941 - ((((v14942 * v5438) + (Lanes([0.0, 0.0, v14957[0], 0.0, 0.0, 0.0]))) * v5463) + ((((v9856 * v5463) * v10360) / v5462) * v5464));
                    v5467 = v5466;
                    v9855 = v14963;
                } else {
                    v5467 = v5437;
                    v9855 = v14939;
                }
                let v5470 = (v5467 + v5468).sqrt();
                let v5471 = v5435 * v5470;
                let v14967 = v14934 * v5470;
                let v5473 = (v78 * v665) / v142;
                let v14973 = ((v10385 * v78) / v142) * v5471;
                let v5476 = ((v5473 * v5471) * v4879) * v166;
                let v5478 = v4876 + (v5476 * v5430);
                let v14982 = v9761 + ((((((Lanes([0.0, 0.0, v14973[0], 0.0, 0.0, 0.0])) + (((Lanes([0.0, 0.0, v14967[0], 0.0, 0.0, 0.0])) + ((v9855 * (v9353 / (v10405 * v5470))) * v5435)) * v5473)) * v4879) * v166) * v5430) + (v14933 * v5476));
                v5620 = v5478;
                v6024 = v5394;
                v9816 = v14982;
                v9817 = v9851;
            } else {
                v5620 = v4876;
                v6024 = v4421;
                v9816 = v9761;
                v9817 = v9429;
            }
            let v5481 = if v565 != 0.0 || (if v5479 == v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5640: f64;
            let v9858: Lanes<6>;
            if v5481 != 0.0 {
                let v5484 = if (if v4325 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v1886 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5641: f64;
                let v9859: Lanes<6>;
                if v5484 != 0.0 {
                    v5641 = v0;
                    v9859 = v11032;
                } else {
                    let v5487 = if (if v297 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v21 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v5642: f64;
                    let v9860: Lanes<6>;
                    if v5487 != 0.0 {
                        v5642 = v0;
                        v9860 = v11032;
                    } else {
                        let v14986 = ((Lanes([v10534[0], v10534[1], 0.0, v10534[2], v10534[3]])) + v10761) - v10797;
                        let v5492 = (((v867 - v349) + v1142) - v1199) + v5491;
                        let v5612: f64;
                        let v9861: Lanes<6>;
                        if v281 != 0.0 {
                            let v5493 = v1128 * v1128;
                            let v15073 = v9406 * v1128;
                            let v15074 = v15073 + v15073;
                            let v5494 = v491 / v5493;
                            let v15077 = ((v15074 * v5494) * v10360) / v5493;
                            let v5495 = v78 / v491;
                            let v5496 = v5495 * v5493;
                            let v15081 = v9401 * v2080;
                            let v15083 = (v14986 - (Lanes([0.0, 0.0, v10385[0], 0.0, 0.0]))) - (Lanes([v15081[0], v15081[1], 0.0, 0.0, v15081[2]]));
                            let v5505 = ((v5492 - v665) - (v2080 * v988)) - (v2080 * ((v5500 * v5501) / v124));
                            let v15089 = (v15074 * v5495) * v5505;
                            let v15092 = (Lanes([v15089[0], v15089[1], 0.0, v15089[2], v15089[3], 0.0])) + (((Lanes([v15083[0], v15083[1], v15083[2], v15083[3], v15083[4], 0.0])) - (((v9442 * v5500) / v124) * v2080)) * v5496);
                            let v5507 = v1 + (v5496 * v5505);
                            let v15093 = v15092 * v5507;
                            let v5511 = ((v5507 * v5507) + v5509).sqrt();
                            let v15099 = (v15092 + ((v15093 + v15093) * (v9353 / (v10405 * v5511)))) * v13;
                            let v5515 = (v13 * (v5507 + v5511)) + v5514;
                            let v5516 = if v5515 < v0 { 1.0 } else { 0.0 };
                            let v5517: f64;
                            let v9862: Lanes<6>;
                            if v5516 != 0.0 {
                                v5517 = v0;
                                v9862 = v11032;
                            } else {
                                v5517 = v5515;
                                v9862 = v15099;
                            }
                            let v5519 = (v5517 + v362).sqrt();
                            let v15103 = v14986 * v2097;
                            let v5521 = v1 - v5519;
                            let v15105 = v15077 * v5521;
                            let v15111 = v10531 * v2103;
                            let v5527 = v2106 * v2107;
                            let v5529 = ((v2103 * v866) + v5525) - (v5527 * ((v5492 * v2097) + (v5494 * v5521)));
                            let v15115 = ((Lanes([v15111[0], v15111[1], 0.0, 0.0, v15111[2], 0.0])) + v9762) - (((Lanes([v15103[0], v15103[1], v15103[2], v15103[3], v15103[4], 0.0])) + ((Lanes([v15105[0], v15105[1], 0.0, v15105[2], v15105[3], 0.0])) + (((v9862 * (v9353 / (v10405 * v5519))) * v10360) * v5494))) * v5527);
                            let v15116 = v15115 * v5529;
                            let v5533 = ((v5529 * v5529) + v5531).sqrt();
                            let v15122 = (v15115 + ((v15116 + v15116) * (v9353 / (v10405 * v5533)))) * v13;
                            let v5537 = (v13 * (v5529 + v5533)) + v5536;
                            let v5538 = if v5537 < v0 { 1.0 } else { 0.0 };
                            let v5613: f64;
                            let v9863: Lanes<6>;
                            if v5538 != 0.0 {
                                v5613 = v0;
                                v9863 = v11032;
                            } else {
                                v5613 = v5537;
                                v9863 = v15122;
                            }
                            v5612 = v5613;
                            v9861 = v9863;
                        } else {
                            let v5539 = v2121 * v5492;
                            let v14987 = v14986 * v2121;
                            let v5540 = v1128 * v1128;
                            let v14988 = v9406 * v1128;
                            let v14989 = v14988 + v14988;
                            let v5541 = v491 / v5540;
                            let v14992 = ((v14989 * v5541) * v10360) / v5540;
                            let v5542 = v78 / v491;
                            let v5543 = v5542 * v5540;
                            let v14993 = v14989 * v5542;
                            let v14996 = v9401 * v2080;
                            let v14998 = (v14987 - (Lanes([0.0, 0.0, v10385[0], 0.0, 0.0]))) - (Lanes([v14996[0], v14996[1], 0.0, 0.0, v14996[2]]));
                            let v5550 = ((v5539 - v665) - (v2080 * v988)) - (v2080 * ((v5500 * v5501) / v124));
                            let v15004 = v14993 * v5550;
                            let v15007 = (Lanes([v15004[0], v15004[1], 0.0, v15004[2], v15004[3], 0.0])) + (((Lanes([v14998[0], v14998[1], v14998[2], v14998[3], v14998[4], 0.0])) - (((v9442 * v5500) / v124) * v2080)) * v5543);
                            let v5552 = v1 + (v5543 * v5550);
                            let v5554 = v78 * (v1 + v5543);
                            let v15008 = v14993 * v78;
                            let v5555 = v362 + v5554;
                            let v5558 = if (if v5552 < v5555 { 1.0 } else { 0.0 }) != 0.0 && (if v5554 >= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                            let v5590: f64;
                            let v9864: Lanes<6>;
                            if v5558 != 0.0 {
                                let v5559 = v5555 - v5552;
                                let v15009 = Lanes([v15008[0], v15008[1], 0.0, v15008[2], v15008[3], 0.0]);
                                let v15010 = v15009 - v15007;
                                let v5560 = v5559 * v5559;
                                let v15011 = v15010 * v5559;
                                let v15012 = v15011 + v15011;
                                let v5561 = v5554 * v5554;
                                let v15013 = v15008 * v5554;
                                let v15014 = v15013 + v15013;
                                let v5562 = v5560 * v5560;
                                let v15015 = v15012 * v5560;
                                let v5563 = v5561 * v5561;
                                let v15017 = v15014 * v5561;
                                let v5564 = v5562 * v5560;
                                let v5565 = v5563 * v5561;
                                let v15030 = ((((v15017 + v15017) * v5561) + (v15014 * v5563)) * v5561) + (v15014 * v5565);
                                let v5568 = (v5564 * v5560) + (v5565 * v5561);
                                let v15032 = (((((v15015 + v15015) * v5560) + (v15012 * v5562)) * v5560) + (v15012 * v5564)) + (Lanes([v15030[0], v15030[1], 0.0, v15030[2], v15030[3], 0.0]));
                                let v5585: f64;
                                let v9865: Lanes<6>;
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
                                    let mut v9866: Lanes<6> = Lanes([0.0; 6]);
                                    v5574 = v0;
                                    v5576 = v5568;
                                    v9866 = v15032;
                                    loop {
                                        let v5575 = if v5574 < v5579 { 1.0 } else { 0.0 };
                                        if v5575 == 0.0 {
                                            break;
                                        }
                                        let v5577 = v5576.sqrt();
                                        let v15072 = v9866 * (v9353 / (v10405 * v5577));
                                        let v5578 = v5574 + v1;
                                        v5574 = v5578;
                                        v5576 = v5577;
                                        v9866 = v15072;
                                    }
                                    v5585 = v5576;
                                    v9865 = v9866;
                                } else {
                                    let v5584 = v5568.powf(v5583);
                                    let v15036 = v15032 * (v5583 * (v5568.powf(v15033)));
                                    v5585 = v5584;
                                    v9865 = v15036;
                                }
                                let v5586 = v1 / v5585;
                                let v5587 = v5559 * v5554;
                                let v15041 = v15008 * v5559;
                                let v5589 = v5555 - (v5587 * v5586);
                                let v15047 = v15009 - ((((v15010 * v5554) + (Lanes([v15041[0], v15041[1], 0.0, v15041[2], v15041[3], 0.0]))) * v5586) + ((((v9865 * v5586) * v10360) / v5585) * v5587));
                                v5590 = v5589;
                                v9864 = v15047;
                            } else {
                                v5590 = v5552;
                                v9864 = v15007;
                            }
                            let v5591 = if v5590 <= v0 { 1.0 } else { 0.0 };
                            let v5593: f64;
                            let v9867: Lanes<6>;
                            if v5591 != 0.0 {
                                v5593 = v0;
                                v9867 = v11032;
                            } else {
                                let v5592 = v5590.sqrt();
                                let v15050 = v9864 * (v9353 / (v10405 * v5592));
                                v5593 = v5592;
                                v9867 = v15050;
                            }
                            let v5594 = v1 - v5593;
                            let v15052 = v14992 * v5594;
                            let v5598 = v143 / (v2106 + v143);
                            let v15058 = v10531 * v2103;
                            let v5602 = ((v2103 * v866) + v5525) - (v5598 * (v5539 + (v5541 * v5594)));
                            let v15062 = ((Lanes([v15058[0], v15058[1], 0.0, 0.0, v15058[2], 0.0])) + v9762) - (((Lanes([v14987[0], v14987[1], v14987[2], v14987[3], v14987[4], 0.0])) + ((Lanes([v15052[0], v15052[1], 0.0, v15052[2], v15052[3], 0.0])) + ((v9867 * v10360) * v5541))) * v5598);
                            let v15063 = v15062 * v5602;
                            let v5606 = ((v5602 * v5602) + v5604).sqrt();
                            let v15069 = (v15062 + ((v15063 + v15063) * (v9353 / (v10405 * v5606)))) * v13;
                            let v5610 = (v13 * (v5602 + v5606)) + v5609;
                            let v5611 = if v5610 < v0 { 1.0 } else { 0.0 };
                            let v5614: f64;
                            let v9868: Lanes<6>;
                            if v5611 != 0.0 {
                                v5614 = v0;
                                v9868 = v11032;
                            } else {
                                v5614 = v5610;
                                v9868 = v15069;
                            }
                            v5612 = v5614;
                            v9861 = v9868;
                        }
                        let v5615 = v5612 + v362;
                        let v5617 = (-v2196) / v5615;
                        let v5618 = v5617.exp();
                        let v5619 = v2200 * v5615;
                        let v5621 = v5619 * v5620;
                        let v5622 = v5621 * v5618;
                        let v15133 = ((((v9861 * v2200) * v5620) + (v9816 * v5619)) * v5618) + (((((v9861 * v5617) * v10360) / v5615) * v5618) * v5621);
                        v5642 = v5622;
                        v9860 = v15133;
                    }
                    v5641 = v5642;
                    v9859 = v9860;
                }
                v5640 = v5641;
                v9858 = v9859;
            } else {
                let v14983 = Lanes([v9443[0], v9443[1], v9443[2], v9443[3], v9443[4], 0.0]);
                v5640 = v5643;
                v9858 = v14983;
            }
            let v5625 = if (if v1886 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v2204 == v78 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5626 = if v5625 != 0.0 && v565 != 0.0 { 1.0 } else { 0.0 };
            let v9210: f64;
            let v9869: Lanes<6>;
            if v5626 != 0.0 {
                let v5628 = (v206 * v12) * v166;
                let v5629 = -v663;
                let v15134 = v10380 * v10360;
                let v5631 = (v5629 * v2208).exp();
                let v5636 = v5633 + (v5634 * v477);
                let v5638 = (v5628 * v5631) * v5636;
                let v5639 = v5637 / v5638;
                let v15143 = (((((((v15134 * v2208) * v5631) * v5628) * v5636) * v5639) * v10360) / v5638) * v5640;
                let v5647 = v2223 * v665;
                let v5648 = v1 + (v5640 * v5639);
                let v5649 = v5648.ln();
                let v15149 = (v10385 * v2223) * v5649;
                let v15153 = Lanes([0.0, 0.0, v9387[0], 0.0, 0.0, 0.0]);
                let v5652 = v766 * v20;
                let v15155 = v9387 * v20;
                let v5653 = (v766 - (v5647 * v5649)) - v5652;
                let v15157 = (v15153 - ((Lanes([0.0, 0.0, v15149[0], 0.0, 0.0, 0.0])) + ((((v9858 * v5639) + (Lanes([0.0, 0.0, v15143[0], 0.0, 0.0, 0.0]))) * (v9353 / v5648)) * v5647))) - (Lanes([0.0, 0.0, v15155[0], 0.0, 0.0, 0.0]));
                let v5654 = v90 * v766;
                let v5655 = v5654 * v5652;
                let v15161 = ((v9387 * v90) * v5652) + (v15155 * v5654);
                let v5656 = if v5655 > v0 { 1.0 } else { 0.0 };
                let v5658: f64;
                let v9870: Lanes<1>;
                if v5656 != 0.0 {
                    v5658 = v5655;
                    v9870 = v15161;
                } else {
                    let v5657 = -v5655;
                    let v15162 = v15161 * v10360;
                    v5658 = v5657;
                    v9870 = v15162;
                }
                let v15163 = v15157 * v5653;
                let v5661 = ((v5653 * v5653) + v5658).sqrt();
                let v5666 = v5665 * v477;
                let v5668 = (v5666 * v665).sqrt();
                let v15176 = (v10385 * v5666) * (v9353 / (v10405 * v5668));
                let v5669 = v5525 - (v766 - (v13 * (v5653 + v5661)));
                let v15177 = v9762 - (v15153 - ((v15157 + (((v15163 + v15163) + (Lanes([0.0, 0.0, v9870[0], 0.0, 0.0, 0.0]))) * (v9353 / (v10405 * v5661)))) * v13));
                let v15178 = v15134 * v5669;
                let v5671 = (v5629 * v5669).exp();
                let v15183 = v10380 * v5669;
                let v5674 = (v5671 - v1) + (v663 * v5669);
                let v15187 = (((Lanes([0.0, 0.0, v15178[0], 0.0, 0.0, 0.0])) + (v15177 * v5629)) * v5671) + ((Lanes([0.0, 0.0, v15183[0], 0.0, 0.0, 0.0])) + (v15177 * v663));
                let v5675 = if v5674 > v0 { 1.0 } else { 0.0 };
                let v5680: f64;
                let v9871: Lanes<6>;
                if v5675 != 0.0 {
                    let v5676 = v5674.sqrt();
                    let v15195 = v15187 * (v9353 / (v10405 * v5676));
                    v5680 = v5676;
                    v9871 = v15195;
                } else {
                    let v5678 = (-v5674).sqrt();
                    let v5679 = -v5678;
                    let v15192 = ((v15187 * v10360) * (v9353 / (v10405 * v5678))) * v10360;
                    v5680 = v5679;
                    v9871 = v15192;
                }
                let v15196 = v15134 * v5525;
                let v5682 = (v5629 * v5525).exp();
                let v15201 = v10380 * v5525;
                let v5686 = ((v5682 - v1) + (v663 * v5525)).sqrt();
                let v5687 = -v5668;
                let v5688 = v5680 - v5686;
                let v15211 = (v15176 * v10360) * v5688;
                let v15215 = ((Lanes([0.0, 0.0, v15211[0], 0.0, 0.0, 0.0])) + ((v9871 - (((((Lanes([0.0, 0.0, v15196[0], 0.0, 0.0, 0.0])) + (v9762 * v5629)) * v5682) + ((Lanes([0.0, 0.0, v15201[0], 0.0, 0.0, 0.0])) + (v9762 * v663))) * (v9353 / (v10405 * v5686)))) * v5687)) * v10360;
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
                let v15216 = v15215 * v5693;
                let v5701 = ((v5693 * v5693) + v5698).sqrt();
                let v5704 = v5690 - (v13 * (v5693 + v5701));
                let v15223 = ((v15215 + ((v15216 + v15216) * (v9353 / (v10405 * v5701)))) * v13) * v10360;
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
                let v15228 = v9406 * v5708;
                let v15231 = v9365 * v5710;
                let v5713 = ((v5710 * v2254) - v5704) / v5709;
                let v15236 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v15231[0]])) - v15223) - ((((((v9858 * v5708) * v10360) / v5707) * v1128) + (Lanes([v15228[0], v15228[1], 0.0, v15228[2], v15228[3], 0.0]))) * v5713)) / v5709;
                v9210 = v5713;
                v9869 = v15236;
            } else {
                v9210 = v9211;
                v9869 = v9452;
            }
            let v5714 = if v4325 == v0 { 1.0 } else { 0.0 };
            let v5719 = if (if v5714 != 0.0 && (if v5640 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v5717 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v8419: f64;
            let v9872: Lanes<6>;
            if v5719 != 0.0 {
                let v5730: f64;
                let v5746: f64;
                let v9873: Lanes<6>;
                let v9874: Lanes<6>;
                if v986 != 0.0 {
                    v5730 = v0;
                    v5746 = v0;
                    v9873 = v11032;
                    v9874 = v11032;
                } else {
                    let v5720: f64;
                    let v9875: Lanes<6>;
                    if v565 != 0.0 {
                        let v15237 = Lanes([v9398[0], v9398[1], 0.0, 0.0, v9398[2], 0.0]);
                        v5720 = v835;
                        v9875 = v15237;
                    } else {
                        v5720 = v4637;
                        v9875 = v9434;
                    }
                    let v5724: f64;
                    let v9876: Lanes<6>;
                    if v565 != 0.0 {
                        let v15238 = Lanes([v9398[0], v9398[1], 0.0, 0.0, v9398[2], 0.0]);
                        v5724 = v835;
                        v9876 = v15238;
                    } else {
                        v5724 = v5721;
                        v9876 = v9444;
                    }
                    v5730 = v5720;
                    v5746 = v5724;
                    v9873 = v9875;
                    v9874 = v9876;
                }
                let v5728 = v5717 * (v1 + (v5725 * v1142));
                let v5729 = v5728 * v5640;
                let v15241 = ((v10761 * v5725) * v5717) * v5640;
                let v15244 = (Lanes([v15241[0], v15241[1], v15241[2], v15241[3], v15241[4], 0.0])) + (v9858 * v5728);
                let v5731 = v4340 - v5730;
                let v15246 = v10380 * v5731;
                let v15249 = (Lanes([0.0, 0.0, v15246[0], 0.0, 0.0, 0.0])) + ((v9426 - v9873) * v663);
                let v5733 = (v663 * v5731) - v1;
                let v15250 = v15249 * v5733;
                let v5737 = ((v5733 * v5733) + v5735).sqrt();
                let v15256 = (v15249 + ((v15250 + v15250) * (v9353 / (v10405 * v5737)))) * v13;
                let v5741 = (v13 * (v5733 + v5737)) + v5740;
                let v5742 = if v5741 < v0 { 1.0 } else { 0.0 };
                let v5743: f64;
                let v9877: Lanes<6>;
                if v5742 != 0.0 {
                    v5743 = v0;
                    v9877 = v11032;
                } else {
                    v5743 = v5741;
                    v9877 = v15256;
                }
                let v5744 = v5743.sqrt();
                let v15259 = v9877 * (v9353 / (v10405 * v5744));
                let v5745 = v5743 * v5744;
                let v15262 = (v9877 * v5744) + (v15259 * v5743);
                let v5747 = v4336 - v5746;
                let v15264 = v10380 * v5747;
                let v15267 = (Lanes([0.0, 0.0, v15264[0], 0.0, 0.0, 0.0])) + ((v9425 - v9874) * v663);
                let v5749 = (v663 * v5747) - v1;
                let v15268 = v15267 * v5749;
                let v5753 = ((v5749 * v5749) + v5751).sqrt();
                let v15274 = (v15267 + ((v15268 + v15268) * (v9353 / (v10405 * v5753)))) * v13;
                let v5757 = (v13 * (v5749 + v5753)) + v5756;
                let v5758 = if v5757 < v0 { 1.0 } else { 0.0 };
                let v5759: f64;
                let v9878: Lanes<6>;
                if v5758 != 0.0 {
                    v5759 = v0;
                    v9878 = v11032;
                } else {
                    v5759 = v5757;
                    v9878 = v15274;
                }
                let v5760 = v5759.sqrt();
                let v15277 = v9878 * (v9353 / (v10405 * v5760));
                let v5761 = v5759 * v5760;
                let v5762 = v1 / v5743;
                let v5763 = v663 * v5729;
                let v15284 = v10380 * v5729;
                let v15287 = (Lanes([0.0, 0.0, v15284[0], 0.0, 0.0, 0.0])) + (v15244 * v663);
                let v5764 = v5763 * v5762;
                let v15290 = (v15287 * v5762) + ((((v9877 * v5762) * v10360) / v5743) * v5763);
                let v5765 = v1 / v5759;
                let v5766 = v5763 * v5765;
                let v15296 = (v15287 * v5765) + ((((v9878 * v5765) * v10360) / v5759) * v5763);
                let v5769 = (v5761 * v5766) - (v5745 * v5764);
                let v15304 = v10455 * v5769;
                let v5771 = v750 * v13;
                let v5772 = -v5760;
                let v5775 = (v5772 * v5766) + (v5744 * v5764);
                let v15317 = (v10455 * v13) * v5775;
                let v5777 = (v750 * v5769) + (v5771 * v5775);
                let v5779 = v5778 * v5777;
                let v5784 = v5779 * v5780;
                let v15327 = (((v9763 * v5777) + ((((Lanes([0.0, 0.0, v15304[0], 0.0, 0.0, 0.0])) + ((((((v9878 * v5760) + (v15277 * v5759)) * v5766) + (v15296 * v5761)) - ((v15262 * v5764) + (v15290 * v5745))) * v750)) + ((Lanes([0.0, 0.0, v15317[0], 0.0, 0.0, 0.0])) + (((((v15277 * v10360) * v5766) + (v15296 * v5772)) + ((v15259 * v5764) + (v15290 * v5744))) * v5771))) * v5778)) * v5780) + (v9764 * v5779);
                v8419 = v5784;
                v9872 = v15327;
            } else {
                v8419 = v0;
                v9872 = v11032;
            }
            let v5785 = v122 * v68;
            let v5786 = v1128 / v556;
            let v15328 = v9406 / v556;
            let v5787 = v136 * v68;
            let v5788 = v166 * v68;
            let v5790 = v5789 / v68;
            let v15329 = v9765 / v68;
            let v5791 = v4428 / v556;
            let v15330 = v9430 / v556;
            let v5792 = v750 / v556;
            let v15331 = v10455 / v556;
            let v5794 = if v5793 == v0 { 1.0 } else { 0.0 };
            let v8680: f64;
            let v8684: f64;
            let v8685: f64;
            let v8689: f64;
            let v8694: f64;
            let v9879: Lanes<4>;
            let v9880: Lanes<6>;
            let v9881: Lanes<3>;
            let v9882: Lanes<3>;
            if v5794 != 0.0 {
                v8680 = v0;
                v8684 = v0;
                v8685 = v0;
                v8689 = v0;
                v8694 = v0;
                v9879 = v10595;
                v9880 = v11032;
                v9881 = v10501;
                v9882 = v10501;
            } else {
                let v8686: f64;
                let v9883: Lanes<6>;
                if v5714 != 0.0 {
                    let v15338 = (Lanes([v10534[0], v10534[1], 0.0, v10534[2], v10534[3]])) + (((v10761 - v10797) * v5799) * v5787);
                    let v5807 = v1 / v5785;
                    let v5808 = (((v867 - v240) + ((v5799 * (v1142 - v1199)) * v5787)) - (((v5525 + v866) - v5796) * v5804)) * v5807;
                    let v5810 = v1 / v5809;
                    let v5812 = v1 + (v5790 * v5810);
                    let v5813 = v5808 * v5812;
                    let v15346 = ((((Lanes([v15338[0], v15338[1], v15338[2], v15338[3], v15338[4], 0.0])) - ((v9762 + (Lanes([v10531[0], v10531[1], 0.0, 0.0, v10531[2], 0.0]))) * v5804)) * v5807) * v5812) + ((v15329 * v5810) * v5808);
                    let v15347 = v15346 * v5813;
                    let v5817 = ((v5813 * v5813) + v5815).sqrt();
                    let v15353 = (v15346 + ((v15347 + v15347) * (v9353 / (v10405 * v5817)))) * v13;
                    let v5821 = (v13 * (v5813 + v5817)) + v5820;
                    let v5822 = if v5821 < v0 { 1.0 } else { 0.0 };
                    let v5839: f64;
                    let v9884: Lanes<6>;
                    if v5822 != 0.0 {
                        v5839 = v0;
                        v9884 = v11032;
                    } else {
                        v5839 = v5821;
                        v9884 = v15353;
                    }
                    let v15354 = v10534 * v867;
                    let v5826 = ((v867 * v867) + v5824).sqrt();
                    let v15360 = (v10534 + ((v15354 + v15354) * (v9353 / (v10405 * v5826)))) * v13;
                    let v5830 = (v13 * (v867 + v5826)) + v5829;
                    let v5831 = if v5830 < v0 { 1.0 } else { 0.0 };
                    let v5832: f64;
                    let v9885: Lanes<4>;
                    if v5831 != 0.0 {
                        v5832 = v0;
                        v9885 = v10595;
                    } else {
                        v5832 = v5830;
                        v9885 = v15360;
                    }
                    let v5834 = (v5832 - v840) / v79;
                    let v15362 = (v9885 / v79) * v5834;
                    let v5836 = v1 + (v5834 * v5834);
                    let v5837 = v1 / v5836;
                    let v5838 = v1 - v5837;
                    let v5840 = v5839 * v5838;
                    let v15369 = (((((v15362 + v15362) * v5837) * v10360) / v5836) * v10360) * v5839;
                    let v15371 = (v9884 * v5838) + (Lanes([v15369[0], v15369[1], 0.0, v15369[2], v15369[3], 0.0]));
                    let v5841 = v5787 * v5788;
                    let v5844 = v5842 / (v5842 + v5841);
                    let v5846 = v5845 + v866;
                    let v5847 = v5845 / v5846;
                    let v15374 = ((v10531 * v5847) * v10360) / v5846;
                    let v5848 = v5840 + v362;
                    let v5849 = v1 / v5848;
                    let v5851 = -v5850;
                    let v5852 = v5851 * v719;
                    let v5853 = v5852 * v5849;
                    let v15379 = (v10411 * v5851) * v5849;
                    let v15382 = (Lanes([0.0, 0.0, v15379[0], 0.0, 0.0, 0.0])) + ((((v15371 * v5849) * v10360) / v5848) * v5852);
                    let v5855 = if v5853 < v5854 { 1.0 } else { 0.0 };
                    let v8687: f64;
                    let v9886: Lanes<6>;
                    if v5855 != 0.0 {
                        v8687 = v0;
                        v9886 = v11032;
                    } else {
                        let v5856 = v5853.exp();
                        let v5858 = v5857 / v718;
                        let v5860 = (v5858 * v206) * v5841;
                        let v5861 = v1 / v5792;
                        let v15392 = v15328 * v11;
                        let v5863 = v5791 + (v5786 * v11);
                        let v15396 = (((v15331 * v5861) * v10360) / v5792) * v5863;
                        let v5865 = (v5863 * v5861).sqrt();
                        let v5866 = v5856 * v5860;
                        let v15403 = (((((v10408 * v5858) * v10360) / v718) * v206) * v5841) * v5856;
                        let v5867 = v5866 * v5865;
                        let v5868 = v5867 * v5840;
                        let v5869 = v5868 * v5840;
                        let v5870 = v5844 * v5847;
                        let v5871 = v5870 * v5869;
                        let v15416 = (v15374 * v5844) * v5869;
                        let v15419 = (Lanes([v15416[0], v15416[1], 0.0, 0.0, v15416[2], 0.0])) + ((((((((((v15382 * v5856) * v5860) + (Lanes([0.0, 0.0, v15403[0], 0.0, 0.0, 0.0]))) * v5865) + (((((v15330 + (Lanes([v15392[0], v15392[1], 0.0, v15392[2], v15392[3], 0.0]))) * v5861) + (Lanes([0.0, 0.0, v15396[0], 0.0, 0.0, 0.0]))) * (v9353 / (v10405 * v5865))) * v5866)) * v5840) + (v15371 * v5867)) * v5840) + (v15371 * v5868)) * v5870);
                        v8687 = v5871;
                        v9886 = v15419;
                    }
                    v8686 = v8687;
                    v9883 = v9886;
                } else {
                    v8686 = v0;
                    v9883 = v11032;
                }
                let v5873 = -v5872;
                let v5878 = (v5785 * ((v5873 * v830) + v5875)).exp();
                let v5880 = (v830 / v5785) / v5785;
                let v5881 = v830 * v5880;
                let v5884 = (v5882 / v61) * v5788;
                let v5885 = v5884 * v5878;
                let v5886 = v5885 * v5881;
                let v15431 = (((((v9397 * v5873) * v5785) * v5878) * v5884) * v5881) + (((v9397 * v5880) + (((v9397 / v5785) / v5785) * v830)) * v5885);
                let v5887 = if v830 >= v0 { 1.0 } else { 0.0 };
                let v8695: f64;
                let v9887: Lanes<3>;
                if v5887 != 0.0 {
                    let v5889 = v5886 * v5888;
                    let v15432 = v15431 * v5888;
                    v8695 = v5889;
                    v9887 = v15432;
                } else {
                    v8695 = v5886;
                    v9887 = v15431;
                }
                let v5890 = v830 - v823;
                let v15434 = v9397 - (Lanes([v9395[0], v9395[1], 0.0]));
                let v5894 = (v5785 * ((v5873 * v5890) + v5875)).exp();
                let v5896 = (v5890 / v5785) / v5785;
                let v5897 = v5890 * v5896;
                let v5898 = v5884 * v5894;
                let v5899 = v5898 * v5897;
                let v15446 = (((((v15434 * v5873) * v5785) * v5894) * v5884) * v5897) + (((v15434 * v5896) + (((v15434 / v5785) / v5785) * v5890)) * v5898);
                let v5900 = if v5890 >= v0 { 1.0 } else { 0.0 };
                let v8690: f64;
                let v9888: Lanes<3>;
                if v5900 != 0.0 {
                    let v5902 = v5899 * v5901;
                    let v15447 = v15446 * v5901;
                    v8690 = v5902;
                    v9888 = v15447;
                } else {
                    v8690 = v5899;
                    v9888 = v15446;
                }
                let v15448 = v9397 * v10360;
                let v5908 = ((((-v830) + v878) + v240) + v5906) / v5785;
                let v15452 = ((Lanes([v15448[0], v15448[1], v15448[2], 0.0])) + (Lanes([v9400[0], v9400[1], 0.0, v9400[2]]))) / v5785;
                let v15453 = v15452 * v5908;
                let v5912 = ((v5908 * v5908) + v5910).sqrt();
                let v15459 = (v15452 + ((v15453 + v15453) * (v9353 / (v10405 * v5912)))) * v13;
                let v5916 = (v13 * (v5908 + v5912)) + v5915;
                let v5917 = if v5916 < v0 { 1.0 } else { 0.0 };
                let v5918: f64;
                let v9889: Lanes<4>;
                if v5917 != 0.0 {
                    v5918 = v0;
                    v9889 = v10595;
                } else {
                    v5918 = v5916;
                    v9889 = v15459;
                }
                let v5919 = v5918 + v362;
                let v5922 = (-v5920) / v5919;
                let v15462 = ((v9889 * v5922) * v10360) / v5919;
                let v5924 = if v5922 < v5923 { 1.0 } else { 0.0 };
                let v8681: f64;
                let v9890: Lanes<4>;
                if v5924 != 0.0 {
                    v8681 = v0;
                    v9890 = v10595;
                } else {
                    let v5925 = v5922.exp();
                    let v5928 = (v5926 * v5788) * v5787;
                    let v5929 = v5928 * v5919;
                    let v5930 = v5929 * v5919;
                    let v5931 = v5930 * v5925;
                    let v15470 = ((((v9889 * v5928) * v5919) + (v9889 * v5929)) * v5925) + ((v15462 * v5925) * v5930);
                    v8681 = v5931;
                    v9890 = v15470;
                }
                v8680 = v8681;
                v8684 = v13;
                v8685 = v8686;
                v8689 = v8690;
                v8694 = v8695;
                v9879 = v9890;
                v9880 = v9883;
                v9881 = v9888;
                v9882 = v9887;
            }
            let v5933 = if v5932 == v0 { 1.0 } else { 0.0 };
            let v8702: f64;
            let v9891: Lanes<5>;
            if v5933 != 0.0 {
                v8702 = v0;
                v9891 = v10549;
            } else {
                let v15471 = v9395 * v5934;
                let v15473 = (Lanes([v15471[0], v15471[1], 0.0])) - v9397;
                let v5942 = v1 / v122;
                let v5943 = (((v5934 * (v823 + v5935)) - v830) + (v1138 * v5939)) * v5942;
                let v15477 = ((Lanes([v15473[0], v15473[1], 0.0, v15473[2], 0.0])) + (v10758 * v5939)) * v5942;
                let v15478 = v15477 * v5943;
                let v5947 = ((v5943 * v5943) + v5945).sqrt();
                let v15484 = (v15477 + ((v15478 + v15478) * (v9353 / (v10405 * v5947)))) * v13;
                let v5951 = (v13 * (v5943 + v5947)) + v5950;
                let v5952 = if v5951 < v0 { 1.0 } else { 0.0 };
                let v5953: f64;
                let v9892: Lanes<5>;
                if v5952 != 0.0 {
                    v5953 = v0;
                    v9892 = v10549;
                } else {
                    v5953 = v5951;
                    v9892 = v15484;
                }
                let v5954 = v5953 + v362;
                let v5955 = v1 / v5954;
                let v5957 = -v5956;
                let v5958 = v5957 * v719;
                let v5959 = v5958 * v5955;
                let v15489 = (v10411 * v5957) * v5955;
                let v15492 = (Lanes([0.0, 0.0, v15489[0], 0.0, 0.0])) + ((((v9892 * v5955) * v10360) / v5954) * v5958);
                let v5961 = if v5959 < v5960 { 1.0 } else { 0.0 };
                let v5977: f64;
                let v9893: Lanes<5>;
                if v5961 != 0.0 {
                    v5977 = v0;
                    v9893 = v10549;
                } else {
                    let v5962 = v5959.exp();
                    let v5964 = v5963 / v718;
                    let v5966 = (v5964 * v206) * v166;
                    let v5967 = v5966 * v5953;
                    let v15499 = (((((v10408 * v5964) * v10360) / v718) * v206) * v166) * v5953;
                    let v5968 = v5967 * v5953;
                    let v5969 = v5968 * v5962;
                    let v15508 = (((((Lanes([0.0, 0.0, v15499[0], 0.0, 0.0])) + (v9892 * v5966)) * v5953) + (v9892 * v5967)) * v5962) + ((v15492 * v5962) * v5968);
                    v5977 = v5969;
                    v9893 = v15508;
                }
                let v5970 = v823 - v878;
                let v15509 = v10530 - v9400;
                let v5971 = if v5970 > v0 { 1.0 } else { 0.0 };
                let v8703: f64;
                let v9894: Lanes<5>;
                if v5971 != 0.0 {
                    let v5972 = v5970 * v5970;
                    let v15510 = v15509 * v5970;
                    let v5973 = v5972 * v5970;
                    let v15514 = ((v15510 + v15510) * v5970) + (v15509 * v5972);
                    let v5975 = v5973 + v5974;
                    let v5976 = v5973 / v5975;
                    let v5978 = v5977 * v5976;
                    let v15519 = ((v15514 - (v15514 * v5976)) / v5975) * v5977;
                    let v15521 = (v9893 * v5976) + (Lanes([v15519[0], v15519[1], 0.0, 0.0, v15519[2]]));
                    v8703 = v5978;
                    v9894 = v15521;
                } else {
                    v8703 = v0;
                    v9894 = v10549;
                }
                v8702 = v8703;
                v9891 = v9894;
            }
            let v8704: f64;
            let v9895: Lanes<5>;
            if v5933 != 0.0 {
                v8704 = v0;
                v9895 = v10549;
            } else {
                let v15523 = (v9395 * v10360) * v5934;
                let v15527 = (Lanes([v15523[0], v15523[1], 0.0])) - (v9397 - (Lanes([v9395[0], v9395[1], 0.0])));
                let v5986 = v1 / v122;
                let v5987 = (((v5934 * ((-v823) + v5935)) - (v830 - v823)) + (v1138 * v5939)) * v5986;
                let v15531 = ((Lanes([v15527[0], v15527[1], 0.0, v15527[2], 0.0])) + (v10758 * v5939)) * v5986;
                let v15532 = v15531 * v5987;
                let v5991 = ((v5987 * v5987) + v5989).sqrt();
                let v15538 = (v15531 + ((v15532 + v15532) * (v9353 / (v10405 * v5991)))) * v13;
                let v5995 = (v13 * (v5987 + v5991)) + v5994;
                let v5996 = if v5995 < v0 { 1.0 } else { 0.0 };
                let v5997: f64;
                let v9896: Lanes<5>;
                if v5996 != 0.0 {
                    v5997 = v0;
                    v9896 = v10549;
                } else {
                    v5997 = v5995;
                    v9896 = v15538;
                }
                let v5998 = v5997 + v362;
                let v5999 = v1 / v5998;
                let v6000 = -v5956;
                let v6001 = v6000 * v719;
                let v6002 = v6001 * v5999;
                let v15543 = (v10411 * v6000) * v5999;
                let v15546 = (Lanes([0.0, 0.0, v15543[0], 0.0, 0.0])) + ((((v9896 * v5999) * v10360) / v5998) * v6001);
                let v6004 = if v6002 < v6003 { 1.0 } else { 0.0 };
                let v6019: f64;
                let v9897: Lanes<5>;
                if v6004 != 0.0 {
                    v6019 = v0;
                    v9897 = v10549;
                } else {
                    let v6005 = v6002.exp();
                    let v6006 = v1 / v718;
                    let v6009 = ((v5963 * v6006) * v206) * v166;
                    let v6010 = v6009 * v5997;
                    let v15554 = ((((((v10408 * v6006) * v10360) / v718) * v5963) * v206) * v166) * v5997;
                    let v6011 = v6010 * v5997;
                    let v6012 = v6011 * v6005;
                    let v15563 = (((((Lanes([0.0, 0.0, v15554[0], 0.0, 0.0])) + (v9896 * v6009)) * v5997) + (v9896 * v6010)) * v6005) + ((v15546 * v6005) * v6011);
                    v6019 = v6012;
                    v9897 = v15563;
                }
                let v6013 = -v878;
                let v15564 = v9400 * v10360;
                let v6014 = if v6013 > v0 { 1.0 } else { 0.0 };
                let v8705: f64;
                let v9898: Lanes<5>;
                if v6014 != 0.0 {
                    let v6015 = v6013 * v6013;
                    let v15565 = v15564 * v6013;
                    let v6016 = v6015 * v6013;
                    let v15569 = ((v15565 + v15565) * v6013) + (v15564 * v6015);
                    let v6017 = v6016 + v5974;
                    let v6018 = v6016 / v6017;
                    let v6020 = v6019 * v6018;
                    let v15574 = ((v15569 - (v15569 * v6018)) / v6017) * v6019;
                    let v15576 = (v9897 * v6018) + (Lanes([v15574[0], v15574[1], 0.0, 0.0, v15574[2]]));
                    v8705 = v6020;
                    v9898 = v15576;
                } else {
                    v8705 = v0;
                    v9898 = v10549;
                }
                v8704 = v8705;
                v9895 = v9898;
            }
            let v8539: f64;
            let v8547: f64;
            let v8555: f64;
            let v8567: f64;
            let v8579: f64;
            let v8586: f64;
            let v8596: f64;
            let v8603: f64;
            let v9899: Lanes<5>;
            let v9900: Lanes<5>;
            let v9901: Lanes<6>;
            let v9902: Lanes<6>;
            let v9903: Lanes<5>;
            let v9904: Lanes<6>;
            let v9905: Lanes<5>;
            let v9906: Lanes<6>;
            if v565 != 0.0 {
                let v6021 = v1 / v127;
                let v6022 = -v3861;
                let v6023 = v6022 * v4428;
                let v15577 = v9430 * v6022;
                let v6026 = v6023 + (v6022 * v6024);
                let v15579 = v15577 + (v9817 * v6022);
                let v6027 = v6023 * v13;
                let v15580 = v15577 * v13;
                let v6028 = v6023 - v6027;
                let v15581 = v15577 - v15580;
                let v6029 = v6026 * v13;
                let v15582 = v15579 * v13;
                let v6030 = v6026 - v6029;
                let v15583 = v15579 - v15582;
                let v8540: f64;
                let v8548: f64;
                let v8556: f64;
                let v8568: f64;
                let v8580: f64;
                let v8587: f64;
                let v8597: f64;
                let v8604: f64;
                let v9907: Lanes<5>;
                let v9908: Lanes<5>;
                let v9909: Lanes<6>;
                let v9910: Lanes<6>;
                let v9911: Lanes<5>;
                let v9912: Lanes<6>;
                let v9913: Lanes<5>;
                let v9914: Lanes<6>;
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
                    let v9915: Lanes<5>;
                    let v9916: Lanes<5>;
                    let v9917: Lanes<6>;
                    let v9918: Lanes<6>;
                    let v9919: Lanes<5>;
                    let v9920: Lanes<6>;
                    let v9921: Lanes<5>;
                    let v9922: Lanes<6>;
                    if v6040 != 0.0 {
                        let v6042 = (v489 / v489).sqrt();
                        let v6043 = v750 * v6042;
                        let v15584 = v10455 * v6042;
                        let v6051 = (v6046 * v835) + (v6048 * (v835 - v823));
                        let v15588 = (v9398 * v6046) + ((v9398 - v10530) * v6048);
                        let v15592 = (v9395 * v6046) + ((v9395 * v10360) * v6048);
                        let v6057 = v830 - v823;
                        let v15595 = v9397 - (Lanes([v9395[0], v9395[1], 0.0]));
                        let v6059 = (v6046 * v830) + (v6048 * v6057);
                        let v15597 = (v9397 * v6046) + (v15595 * v6048);
                        let v6062 = (v6048 * v830) + (v6046 * v6057);
                        let v15600 = (v9397 * v6048) + (v15595 * v6046);
                        let v6063 = ((v6046 * v823) + (v6048 * (-v823))) - v6051;
                        let v15602 = (Lanes([v15592[0], v15592[1], 0.0])) - v15588;
                        let v6064 = -v6051;
                        let v15603 = v15588 * v10360;
                        let v6066 = v6046 + (v6045 * v6048);
                        let v6068 = v6048 + (v6045 * v6046);
                        let v6071 = (v6066 * v6059) + (v6068 * v6062);
                        let v15606 = (v15597 * v6066) + (v15600 * v6068);
                        let v6077 = -(((v6066 * v6064) + (v6068 * v6063)) + v6075);
                        let v15610 = ((v15603 * v6066) + (v15602 * v6068)) * v10360;
                        let v6078 = if v6077 > v783 { 1.0 } else { 0.0 };
                        let v6093: f64;
                        let v9923: Lanes<3>;
                        if v6078 != 0.0 {
                            let v6080 = v779 - v783;
                            let v6081 = (v6077 - v783) / v6080;
                            let v15611 = v15610 / v6080;
                            let v6082 = v6081 * v6081;
                            let v15612 = v15611 * v6081;
                            let v15613 = v15612 + v15612;
                            let v15617 = v15613 * v6082;
                            let v6088 = (((v1 + v6081) + v6082) + (v6082 * v6081)) + (v6082 * v6082);
                            let v6089 = v1 / v6088;
                            let v15626 = (((((((v15611 + v15613) + ((v15613 * v6081) + (v15611 * v6082))) + (v15617 + v15617)) * v6089) * v10360) / v6088) * v10360) * v6080;
                            let v6092 = v783 + (v6080 * (v1 - v6089));
                            v6093 = v6092;
                            v9923 = v15626;
                        } else {
                            v6093 = v6077;
                            v9923 = v15610;
                        }
                        let v15627 = v9923 * v10360;
                        let v6095 = (-v6093) - v11;
                        let v6096 = v6043 * v6021;
                        let v15628 = v15584 * v6021;
                        let v6097 = v6096 * v6096;
                        let v15629 = v15628 * v6096;
                        let v15630 = v15629 + v15629;
                        let v6100 = v6071 - v6098;
                        let v6101 = v489 / v731;
                        let v6102 = v78 / v663;
                        let v6103 = v6101.ln();
                        let v6104 = v6102 * v6103;
                        let v15641 = ((((v10380 * v6102) * v10360) / v663) * v6103) + (((((v10423 * v6101) * v10360) / v731) * (v9353 / v6101)) * v6102);
                        let v6105 = -v6095;
                        let v15642 = v15627 * v10360;
                        let v6106 = if v6100 < v6105 { 1.0 } else { 0.0 };
                        let v6450: f64;
                        let v6452: f64;
                        let v6829: f64;
                        let v6839: f64;
                        let v6844: f64;
                        let v9924: Lanes<5>;
                        let v9925: Lanes<5>;
                        let v9926: Lanes<5>;
                        let v9927: Lanes<5>;
                        let v9928: Lanes<5>;
                        if v6106 != 0.0 {
                            let v6107 = v663 * v6043;
                            let v6108 = v1 / v6107;
                            let v6109 = v6108 * v127;
                            let v16026 = (((((v10380 * v6043) + (v15584 * v663)) * v6108) * v10360) / v6107) * v127;
                            let v16027 = v16026 * v6110;
                            let v6112 = v78 + (v6110 * v6109);
                            let v6113 = v91 * v6112;
                            let v6114 = v6113 * v6112;
                            let v6115 = v6114 * v6112;
                            let v16034 = ((((v16027 * v91) * v6112) + (v16027 * v6113)) * v6112) + (v16027 * v6114);
                            let v6116 = v661 - v6104;
                            let v16035 = v10376 - v15641;
                            let v6117 = v6100 + v6095;
                            let v16039 = v10380 * v6117;
                            let v16040 = ((Lanes([v15606[0], v15606[1], v15606[2], 0.0])) + (Lanes([v15627[0], v15627[1], 0.0, v15627[2]]))) * v663;
                            let v6120 = v3500 * v6109;
                            let v6121 = (v663 * v6117) - v78;
                            let v6122 = v6120 * v6121;
                            let v16045 = (v16026 * v3500) * v6121;
                            let v16048 = (Lanes([0.0, 0.0, v16045[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v16039[0], 0.0, 0.0])) + (Lanes([v16040[0], v16040[1], 0.0, v16040[2], v16040[3]]))) * v6120);
                            let v6123 = v6119 - v6122;
                            let v16049 = v16048 * v10360;
                            let v6124 = v6123 * v6123;
                            let v16050 = v16049 * v6123;
                            let v16051 = v16050 + v16050;
                            let v6126 = if v6115 < (v6124 * v3506) { 1.0 } else { 0.0 };
                            let v6138: f64;
                            let v9929: Lanes<5>;
                            if v6126 != 0.0 {
                                let v16058 = v16034 * v13;
                                let v6130 = (v13 * v6115) / v6123;
                                let v6132 = ((v6127 + v6123) + v6130) + v6122;
                                let v16064 = (v16049 + (((Lanes([0.0, 0.0, v16058[0], 0.0, 0.0])) - (v16049 * v6130)) / v6123)) + v16048;
                                v6138 = v6132;
                                v9929 = v16064;
                            } else {
                                let v6134 = (v6115 + v6124).sqrt();
                                let v6137 = (v6135 + v6134) + v6122;
                                let v16057 = (((Lanes([0.0, 0.0, v16034[0], 0.0, 0.0])) + v16051) * (v9353 / (v10405 * v6134))) + v16048;
                                v6138 = v6137;
                                v9929 = v16057;
                            }
                            let v6139 = v6138.powf(v1562);
                            let v16068 = v9929 * (v1562 * (v6138.powf(v16065)));
                            let v16070 = (v16026 * v3523) * v10360;
                            let v6145 = v748 * v6139;
                            let v6148 = (((v6140 - (v3523 * v6109)) + (v78 * v6139)) + (v6145 * v6139)) / v6139;
                            let v16083 = v10385 * v6148;
                            let v16086 = Lanes([v15627[0], v15627[1], 0.0, 0.0, v15627[2]]);
                            let v6151 = ((v6148 * v665) - v6095) + v6095;
                            let v16088 = ((((((((Lanes([0.0, 0.0, v16070[0], 0.0, 0.0])) + (v16068 * v78)) + (((v16068 * v748) * v6139) + (v16068 * v6145))) - (v16068 * v6148)) / v6139) * v665) + (Lanes([0.0, 0.0, v16083[0], 0.0, 0.0]))) - v16086) + v16086;
                            let v6152 = v6151 / v6116;
                            let v16089 = v16035 * v6152;
                            let v16093 = ((v16088 - (Lanes([0.0, 0.0, v16089[0], 0.0, 0.0]))) / v6116) * v6152;
                            let v6155 = (v1 + (v6152 * v6152)).sqrt();
                            let v6156 = v6151 / v6155;
                            let v6159 = v127 * (v6100 - (v6156 - v6095));
                            let v16104 = ((Lanes([v15606[0], v15606[1], 0.0, v15606[2], 0.0])) - (((v16088 - (((v16093 + v16093) * (v9353 / (v10405 * v6155))) * v6156)) / v6155) - v16086)) * v127;
                            v6450 = v6159;
                            v6452 = v6159;
                            v6829 = v0;
                            v6839 = v0;
                            v6844 = v0;
                            v9924 = v16104;
                            v9925 = v16104;
                            v9926 = v10549;
                            v9927 = v10549;
                            v9928 = v10549;
                        } else {
                            let v6161 = v6100 + v6095;
                            let v15645 = (Lanes([v15606[0], v15606[1], v15606[2], 0.0])) + (Lanes([v15627[0], v15627[1], 0.0, v15627[2]]));
                            let v15646 = v10380 * v6161;
                            let v15647 = v15645 * v663;
                            let v15649 = Lanes([v15647[0], v15647[1], 0.0, v15647[2], v15647[3]]);
                            let v15650 = (Lanes([0.0, 0.0, v15646[0], 0.0, 0.0])) + v15649;
                            let v6163 = (v663 * v6161) - v1;
                            let v6166 = v6097 * v664;
                            let v15654 = (v15630 * v664) + (v10382 * v6097);
                            let v6167 = (v90 * (v6163 + v6160)) / v6166;
                            let v15655 = v15654 * v6167;
                            let v15658 = ((v15650 * v90) - (Lanes([0.0, 0.0, v15655[0], 0.0, 0.0]))) / v6166;
                            let v6168 = v1 + v6167;
                            let v6170 = if v6168 < v6169 { 1.0 } else { 0.0 };
                            let v6174: f64;
                            let v9930: Lanes<5>;
                            if v6170 != 0.0 {
                                v6174 = v6171;
                                v9930 = v10549;
                            } else {
                                v6174 = v6168;
                                v9930 = v15658;
                            }
                            let v6173 = (v6097 * v663) / v78;
                            let v15662 = ((v15630 * v663) + (v10380 * v6097)) / v78;
                            let v6175 = v6174.sqrt();
                            let v6176 = v1 - v6175;
                            let v15667 = v15662 * v6176;
                            let v15671 = Lanes([v15606[0], v15606[1], 0.0, v15606[2], 0.0]);
                            let v6179 = (v6100 + (v6173 * v6176)) + v6095;
                            let v15673 = Lanes([v15627[0], v15627[1], 0.0, 0.0, v15627[2]]);
                            let v15675 = v10380 * v6179;
                            let v6182 = (-(v663 * v6179)).exp();
                            let v6185 = (v90 * (v6163 + v6182)) / v6166;
                            let v15683 = v15654 * v6185;
                            let v15686 = (((v15650 + ((((Lanes([0.0, 0.0, v15675[0], 0.0, 0.0])) + (((v15671 + ((Lanes([0.0, 0.0, v15667[0], 0.0, 0.0])) + (((v9930 * (v9353 / (v10405 * v6175))) * v10360) * v6173))) + v15673) * v663)) * v10360) * v6182)) * v90) - (Lanes([0.0, 0.0, v15683[0], 0.0, 0.0]))) / v6166;
                            let v6186 = v1 + v6185;
                            let v6188 = if v6186 < v6187 { 1.0 } else { 0.0 };
                            let v6190: f64;
                            let v9931: Lanes<5>;
                            if v6188 != 0.0 {
                                v6190 = v6189;
                                v9931 = v10549;
                            } else {
                                v6190 = v6186;
                                v9931 = v15686;
                            }
                            let v6191 = v6190.sqrt();
                            let v6192 = v1 - v6191;
                            let v15691 = v15662 * v6192;
                            let v6195 = (v6100 + (v6173 * v6192)) + v6095;
                            let v6196 = v663 * v6195;
                            let v15697 = v10380 * v6195;
                            let v15700 = (Lanes([0.0, 0.0, v15697[0], 0.0, 0.0])) + (((v15671 + ((Lanes([0.0, 0.0, v15691[0], 0.0, 0.0])) + (((v9931 * (v9353 / (v10405 * v6191))) * v10360) * v6173))) + v15673) * v663);
                            let v6197 = if v6196 < v96 { 1.0 } else { 0.0 };
                            let v6274: f64;
                            let v9932: Lanes<5>;
                            if v6197 != 0.0 {
                                let v6200 = v663 * v6096;
                                let v6201 = v1 / v6200;
                                let v15706 = ((((v10380 * v6096) + (v15628 * v663)) * v6201) * v10360) / v6200;
                                let v6202 = v6199 + v6201;
                                let v15707 = v15645 * v10360;
                                let v6204 = (-v6161) / v6096;
                                let v15708 = v15628 * v6204;
                                let v15715 = ((v15706 * v6198) / v6207) * v10360;
                                let v6212 = (v6205 - ((v6198 * v6202) / v6207)) + (v6204 / v6210);
                                let v15718 = (Lanes([0.0, 0.0, v15715[0], 0.0, 0.0])) + ((((Lanes([v15707[0], v15707[1], 0.0, v15707[2], v15707[3]])) - (Lanes([0.0, 0.0, v15708[0], 0.0, 0.0]))) / v6096) / v6210);
                                let v6218 = ((v6213 * v6202) - v6215) / v6217;
                                let v15720 = (v15706 * v6213) / v6217;
                                let v15721 = v15718 * v6212;
                                let v6220 = v6218 * v6218;
                                let v15723 = v15720 * v6218;
                                let v15727 = ((v15723 + v15723) * v6218) + (v15720 * v6220);
                                let v6223 = ((v6212 * v6212) + (v6220 * v6218)).sqrt();
                                let v15732 = ((v15721 + v15721) + (Lanes([0.0, 0.0, v15727[0], 0.0, 0.0]))) * (v9353 / (v10405 * v6223));
                                let v6225 = (-v6212) + v6223;
                                let v6227 = v6212 + v6223;
                                let v6232 = ((v6225.powf(v1562)) + (-(v6227.powf(v1562)))) - v6231;
                                let v15747 = v10385 * v6232;
                                let v6235 = ((v6232 * v665) - v6095) + v6095;
                                let v6236 = v663 * v6235;
                                let v15752 = v10380 * v6235;
                                let v15755 = (Lanes([0.0, 0.0, v15752[0], 0.0, 0.0])) + (((((((((v15718 * v10360) + v15732) * (v1562 * (v6225.powf(v15735)))) + (((v15718 + v15732) * (v1562 * (v6227.powf(v15740)))) * v10360)) * v665) + (Lanes([0.0, 0.0, v15747[0], 0.0, 0.0]))) - v15673) + v15673) * v663);
                                v6274 = v6236;
                                v9932 = v15755;
                            } else {
                                v6274 = v6196;
                                v9932 = v15700;
                            }
                            let v6237 = v6161 + v79;
                            let v15756 = v10380 * v6105;
                            let v15757 = v15642 * v663;
                            let v6239 = (v663 * v6105).exp();
                            let v15761 = ((Lanes([0.0, 0.0, v15756[0], 0.0])) + (Lanes([v15757[0], v15757[1], 0.0, v15757[2]]))) * v6239;
                            let v6240 = v6239 + v362;
                            let v6241 = v731 / v489;
                            let v6242 = v6241 * v6241;
                            let v15763 = (v10423 / v489) * v6241;
                            let v15764 = v15763 + v15763;
                            let v6243 = v6242 * v6240;
                            let v15765 = v15764 * v6240;
                            let v15766 = v15761 * v6242;
                            let v6244 = v663 * v6237;
                            let v15769 = v10380 * v6237;
                            let v15771 = (Lanes([0.0, 0.0, v15769[0], 0.0, 0.0])) + v15649;
                            let v6245 = v6243 * v6166;
                            let v15773 = v15654 * v6243;
                            let v15775 = (((Lanes([0.0, 0.0, v15765[0], 0.0])) + v15766) * v6166) + (Lanes([0.0, 0.0, v15773[0], 0.0]));
                            let v15776 = v15771 * v6244;
                            let v6247 = v6245 + (v6244 * v6244);
                            let v15778 = Lanes([v15775[0], v15775[1], v15775[2], 0.0, v15775[3]]);
                            let v6249 = v6242 * v6166;
                            let v6250 = v6249.ln();
                            let v15786 = ((v15764 * v6166) + (v15654 * v6242)) * (v9353 / v6249);
                            let v15787 = Lanes([0.0, 0.0, v15786[0], 0.0, 0.0]);
                            let v6252 = v663 * v6095;
                            let v15789 = v10380 * v6095;
                            let v15790 = v15627 * v663;
                            let v15793 = (Lanes([0.0, 0.0, v15789[0], 0.0])) + (Lanes([v15790[0], v15790[1], 0.0, v15790[2]]));
                            let v15794 = Lanes([v15793[0], v15793[1], v15793[2], 0.0, v15793[3]]);
                            let v15796 = v15771 - ((((v15778 + (v15776 + v15776)) * (v9353 / v6247)) - v15787) + v15794);
                            let v6255 = (v6244 - (((v6247.ln()) - v6250) + v6252)) - v1;
                            let v6256 = v90 * v6244;
                            let v15797 = v15771 * v90;
                            let v6257 = if v6256 > v0 { 1.0 } else { 0.0 };
                            let v6259: f64;
                            let v9933: Lanes<5>;
                            if v6257 != 0.0 {
                                v6259 = v6256;
                                v9933 = v15797;
                            } else {
                                let v6258 = -v6256;
                                let v15798 = v15797 * v10360;
                                v6259 = v6258;
                                v9933 = v15798;
                            }
                            let v15799 = v15796 * v6255;
                            let v6262 = ((v6255 * v6255) + v6259).sqrt();
                            let v15809 = v10380 * v79;
                            let v6268 = (v6244 - (v6244 - (v13 * (v6255 + v6262)))) + (v663 * v79);
                            let v15812 = ((v15771 - (v15771 - ((v15796 + (((v15799 + v15799) + v9933) * (v9353 / (v10405 * v6262)))) * v13))) + (Lanes([0.0, 0.0, v15809[0], 0.0, 0.0]))) * v6268;
                            let v6270 = v6245 + (v6268 * v6268);
                            let v6273 = ((v6270.ln()) - v6250) + v6252;
                            let v15818 = (((v15778 + (v15812 + v15812)) * (v9353 / v6270)) - v15787) + v15794;
                            let v15819 = v15818 - v9932;
                            let v6277 = (v6273 - v6274) - v6276;
                            let v6280 = (v90 * v6273) * v6279;
                            let v15821 = (v15818 * v90) * v6279;
                            let v6281 = if v6280 > v0 { 1.0 } else { 0.0 };
                            let v6283: f64;
                            let v9934: Lanes<5>;
                            if v6281 != 0.0 {
                                v6283 = v6280;
                                v9934 = v15821;
                            } else {
                                let v6282 = -v6280;
                                let v15822 = v15821 * v10360;
                                v6283 = v6282;
                                v9934 = v15822;
                            }
                            let v15823 = v15819 * v6277;
                            let v6286 = ((v6277 * v6277) + v6283).sqrt();
                            let v6289 = v6273 - (v13 * (v6277 + v6286));
                            let v15831 = v15818 - ((v15819 + (((v15823 + v15823) + v9934) * (v9353 / (v10405 * v6286)))) * v13);
                            let v6290 = v6289 / v663;
                            let v15832 = v10380 * v6290;
                            let v6291 = v6290 - v6095;
                            let v15836 = ((v15831 - (Lanes([0.0, 0.0, v15832[0], 0.0, 0.0]))) / v663) - v15673;
                            let v6294 = (-v6289).exp();
                            let v6295 = (v6289 - v1) + v6294;
                            let v15839 = v15831 + ((v15831 * v10360) * v6294);
                            let v6297 = if v6295 < v6296 { 1.0 } else { 0.0 };
                            let v6299: f64;
                            let v9935: Lanes<5>;
                            if v6297 != 0.0 {
                                v6299 = v6298;
                                v9935 = v10549;
                            } else {
                                v6299 = v6295;
                                v9935 = v15839;
                            }
                            let v6300 = v6299.sqrt();
                            let v6301 = v6043 * v6300;
                            let v15843 = v15584 * v6300;
                            let v15846 = (Lanes([0.0, 0.0, v15843[0], 0.0, 0.0])) + ((v9935 * (v9353 / (v10405 * v6300))) * v6043);
                            let v6303 = v127 * (v6100 - v6291);
                            let v15848 = (v15671 - v15836) * v127;
                            let v6305 = if v6304 == v1 { 1.0 } else { 0.0 };
                            let v6451: f64;
                            let v6453: f64;
                            let v6830: f64;
                            let v6840: f64;
                            let v6845: f64;
                            let v9936: Lanes<5>;
                            let v9937: Lanes<5>;
                            let v9938: Lanes<5>;
                            let v9939: Lanes<5>;
                            let v9940: Lanes<5>;
                            if v6305 != 0.0 {
                                let v6306 = v6242 * v6239;
                                let v15849 = v15764 * v6239;
                                let v15851 = (Lanes([0.0, 0.0, v15849[0], 0.0])) + v15766;
                                let mut v6307: f64 = 0.0;
                                let mut v6310: f64 = 0.0;
                                let mut v6401: f64 = 0.0;
                                let mut v6431: f64 = 0.0;
                                let mut v6434: f64 = 0.0;
                                let mut v6442: f64 = 0.0;
                                let mut v6445: f64 = 0.0;
                                let mut v9941: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9942: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9943: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9944: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9945: Lanes<5> = Lanes([0.0; 5]);
                                v6307 = v1;
                                v6310 = v6291;
                                v6401 = v0;
                                v6431 = v6289;
                                v6434 = v0;
                                v6442 = v0;
                                v6445 = v0;
                                v9941 = v15836;
                                v9942 = v15831;
                                v9943 = v10549;
                                v9944 = v10549;
                                v9945 = v10549;
                                loop {
                                    let v6309 = if v6307 <= v6308 { 1.0 } else { 0.0 };
                                    if v6309 == 0.0 {
                                        break;
                                    }
                                    let v6311 = v6310 + v6095;
                                    let v6312 = v663 * v6311;
                                    let v15872 = v10380 * v6311;
                                    let v15875 = (Lanes([0.0, 0.0, v15872[0], 0.0, 0.0])) + ((v9941 + v15673) * v663);
                                    let v6313 = if v6312 < v644 { 1.0 } else { 0.0 };
                                    let v6394: f64;
                                    let v6398: f64;
                                    let v6435: f64;
                                    let v6446: f64;
                                    let v9946: Lanes<5>;
                                    let v9947: Lanes<5>;
                                    let v9948: Lanes<5>;
                                    let v9949: Lanes<5>;
                                    if v6313 != 0.0 {
                                        let v6314 = v6312 * v6312;
                                        let v15917 = v15875 * v6312;
                                        let v15918 = v15917 + v15917;
                                        let v6315 = v6314 * v6312;
                                        let v6320 = v6317 + (v6312 * v6318);
                                        let v6322 = v6316 + (v6312 * v6320);
                                        let v6323 = v6315 * v6322;
                                        let v15928 = (((v15918 * v6312) + (v15875 * v6314)) * v6322) + (((v15875 * v6320) + ((v15875 * v6318) * v6312)) * v6315);
                                        let v6326 = v6312 * v644;
                                        let v15929 = v15875 * v644;
                                        let v6328 = v6325 + (v6326 * v6318);
                                        let v6330 = v6324 + (v6312 * v6328);
                                        let v6331 = v6314 * v6330;
                                        let v6332 = v6306 * v6323;
                                        let v15937 = v15851 * v6323;
                                        let v6333 = v6332 * v6323;
                                        let v15943 = (((Lanes([v15937[0], v15937[1], v15937[2], 0.0, v15937[3]])) + (v15928 * v6306)) * v6323) + (v15928 * v6332);
                                        let v15945 = v10380 * v6306;
                                        let v6335 = (v6306 * v663) * v78;
                                        let v6336 = v6335 * v6323;
                                        let v15949 = (((v15851 * v663) + (Lanes([0.0, 0.0, v15945[0], 0.0]))) * v78) * v6323;
                                        let v6344 = v6341 + (v6312 * v6342);
                                        let v6346 = v6340 + (v6312 * v6344);
                                        let v6348 = v6339 + (v6312 * v6346);
                                        let v6350 = v6338 + (v6312 * v6348);
                                        let v6351 = v6312 * v6350;
                                        let v15968 = (v15875 * v6350) + (((v15875 * v6348) + (((v15875 * v6346) + (((v15875 * v6344) + ((v15875 * v6342) * v6312)) * v6312)) * v6312)) * v6312);
                                        let v6356 = v6354 + (v6326 * v6342);
                                        let v6358 = v6353 + (v6312 * v6356);
                                        let v6360 = v6352 + (v6312 * v6358);
                                        let v6362 = v6338 + (v6312 * v6360);
                                        let v15979 = v15968 * v6351;
                                        let v6366 = (((v6351 * v6351) + v6333) + v362).sqrt();
                                        let v15984 = ((v15979 + v15979) + v15943) * (v9353 / (v10405 * v6366));
                                        let v15985 = v10380 * v6362;
                                        let v6368 = (v663 * v6362) * v78;
                                        let v6371 = v6366 + v6366;
                                        let v6372 = ((v6368 * v6351) + (v6336 * v6331)) / v6371;
                                        let v15997 = (((((((Lanes([0.0, 0.0, v15985[0], 0.0, 0.0])) + (((v15875 * v6360) + (((v15875 * v6358) + (((v15875 * v6356) + ((v15929 * v6342) * v6312)) * v6312)) * v6312)) * v663)) * v78) * v6351) + (v15968 * v6368)) + ((((Lanes([v15949[0], v15949[1], v15949[2], 0.0, v15949[3]])) + (v15928 * v6335)) * v6331) + (((v15918 * v6330) + (((v15875 * v6328) + ((v15929 * v6318) * v6312)) * v6314)) * v6336))) - ((v15984 + v15984) * v6372)) / v6371;
                                        v6394 = v6366;
                                        v6398 = v6372;
                                        v6435 = v6351;
                                        v6446 = v6333;
                                        v9946 = v15984;
                                        v9947 = v15997;
                                        v9948 = v15968;
                                        v9949 = v15943;
                                    } else {
                                        let v6373 = if v6312 < v2535 { 1.0 } else { 0.0 };
                                        let v6386: f64;
                                        let v6389: f64;
                                        let v9950: Lanes<5>;
                                        let v9951: Lanes<5>;
                                        if v6373 != 0.0 {
                                            let v6374 = v6312.exp();
                                            let v15894 = v15875 * v6374;
                                            let v6375 = v6374 - v1;
                                            let v6376 = v6306 * v6375;
                                            let v15895 = v15851 * v6375;
                                            let v15898 = (Lanes([v15895[0], v15895[1], v15895[2], 0.0, v15895[3]])) + (v15894 * v6306);
                                            let v6377 = v6306 * v663;
                                            let v15900 = v10380 * v6306;
                                            let v6378 = v6377 * v6374;
                                            let v15903 = ((v15851 * v663) + (Lanes([0.0, 0.0, v15900[0], 0.0]))) * v6374;
                                            let v15906 = (Lanes([v15903[0], v15903[1], v15903[2], 0.0, v15903[3]])) + (v15894 * v6377);
                                            v6386 = v6376;
                                            v6389 = v6378;
                                            v9950 = v15898;
                                            v9951 = v15906;
                                        } else {
                                            let v15876 = v10380 * v6310;
                                            let v6380 = (v663 * v6310).exp();
                                            let v15880 = ((Lanes([0.0, 0.0, v15876[0], 0.0, 0.0])) + (v9941 * v663)) * v6380;
                                            let v6381 = v6380 - v6239;
                                            let v6382 = v6242 * v6381;
                                            let v15883 = v15764 * v6381;
                                            let v15886 = (Lanes([0.0, 0.0, v15883[0], 0.0, 0.0])) + ((v15880 - (Lanes([v15761[0], v15761[1], v15761[2], 0.0, v15761[3]]))) * v6242);
                                            let v6383 = v6242 * v663;
                                            let v6384 = v6383 * v6380;
                                            let v15890 = ((v15764 * v663) + (v10380 * v6242)) * v6380;
                                            let v15893 = (Lanes([0.0, 0.0, v15890[0], 0.0, 0.0])) + (v15880 * v6383);
                                            v6386 = v6382;
                                            v6389 = v6384;
                                            v9950 = v15886;
                                            v9951 = v15893;
                                        }
                                        let v6388 = ((v6312 - v1) + v6386).sqrt();
                                        let v15910 = (v15875 + v9950) * (v9353 / (v10405 * v6388));
                                        let v6391 = (v663 + v6389) / v6388;
                                        let v6392 = v6391 * v13;
                                        let v15916 = ((((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + v9951) - (v15910 * v6391)) / v6388) * v13;
                                        v6394 = v6388;
                                        v6398 = v6392;
                                        v6435 = v0;
                                        v6446 = v6386;
                                        v9946 = v15910;
                                        v9947 = v15916;
                                        v9948 = v10549;
                                        v9949 = v9950;
                                    }
                                    let v15999 = v15628 * v6394;
                                    let v6396 = (v6100 - v6310) - (v6096 * v6394);
                                    let v16003 = (v15671 - v9941) - ((Lanes([0.0, 0.0, v15999[0], 0.0, 0.0])) + (v9946 * v6096));
                                    let v16004 = v15628 * v6398;
                                    let v6400 = v6397 - (v6096 * v6398);
                                    let v16008 = ((Lanes([0.0, 0.0, v16004[0], 0.0, 0.0])) + (v9947 * v6096)) * v10360;
                                    let v6402 = if v6401 == v1 { 1.0 } else { 0.0 };
                                    let v6425: f64;
                                    let v6427: f64;
                                    let v6428: f64;
                                    let v9952: Lanes<5>;
                                    if v6402 != 0.0 {
                                        v6425 = v6403;
                                        v6427 = v6310;
                                        v6428 = v6401;
                                        v9952 = v9941;
                                    } else {
                                        let v6405 = (-v6396) / v6400;
                                        let v16012 = ((v16003 * v10360) - (v16008 * v6405)) / v6400;
                                        let v6407 = v6310.abs();
                                        let v16016 = v9941 * ((v10405 * (if v6310 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                                        let v6408 = if v1 >= v6407 { 1.0 } else { 0.0 };
                                        let v6409: f64;
                                        let v9953: Lanes<5>;
                                        if v6408 != 0.0 {
                                            v6409 = v1;
                                            v9953 = v10549;
                                        } else {
                                            v6409 = v6407;
                                            v9953 = v16016;
                                        }
                                        let v6411 = v6406 * (v1 + v6409);
                                        let v16017 = v9953 * v6406;
                                        let v6413 = if (v6405.abs()) > v6411 { 1.0 } else { 0.0 };
                                        let v6418: f64;
                                        let v9954: Lanes<5>;
                                        if v6413 != 0.0 {
                                            let v6414 = if v6405 >= v0 { 1.0 } else { 0.0 };
                                            let v6416: f64;
                                            if v6414 != 0.0 {
                                                v6416 = v1;
                                            } else {
                                                v6416 = v6415;
                                            }
                                            let v6417 = v6411 * v6416;
                                            let v16018 = v16017 * v6416;
                                            v6418 = v6417;
                                            v9954 = v16018;
                                        } else {
                                            v6418 = v6405;
                                            v9954 = v16012;
                                        }
                                        let v6419 = v6310 + v6418;
                                        let v16019 = v9941 + v9954;
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
                                        v9952 = v16019;
                                    }
                                    let v6426 = v6425 + v1;
                                    v6307 = v6426;
                                    v6310 = v6427;
                                    v6401 = v6428;
                                    v6431 = v6312;
                                    v6434 = v6435;
                                    v6442 = v6394;
                                    v6445 = v6446;
                                    v9941 = v9952;
                                    v9942 = v15875;
                                    v9943 = v9948;
                                    v9944 = v9946;
                                    v9945 = v9949;
                                }
                                let v6430 = if v6401 == v0 { 1.0 } else { 0.0 };
                                if v6430 != 0.0 {
                                } else {
                                }
                                let v6432 = if v6431 < v644 { 1.0 } else { 0.0 };
                                let v6440: f64;
                                let v9955: Lanes<5>;
                                if v6432 != 0.0 {
                                    let v6433 = if v6431 < v96 { 1.0 } else { 0.0 };
                                    if v6433 != 0.0 {
                                    } else {
                                    }
                                    let v6437 = v6434 + v6436;
                                    v6440 = v6437;
                                    v9955 = v9943;
                                } else {
                                    let v6439 = (v6431 - v1).sqrt();
                                    let v15854 = v9942 * (v9353 / (v10405 * v6439));
                                    v6440 = v6439;
                                    v9955 = v15854;
                                }
                                let v6441 = v6043 * v6440;
                                let v15855 = v15584 * v6440;
                                let v15858 = (Lanes([0.0, 0.0, v15855[0], 0.0, 0.0])) + (v9955 * v6043);
                                let v6443 = v6442 + v6440;
                                let v6444 = v1 / v6443;
                                let v6447 = v6043 * v6445;
                                let v15863 = v15584 * v6445;
                                let v6449 = v6441 + (v6447 * v6444);
                                let v15870 = v15858 + ((((Lanes([0.0, 0.0, v15863[0], 0.0, 0.0])) + (v9945 * v6043)) * v6444) + (((((v9944 + v9955) * v6444) * v10360) / v6443) * v6447));
                                v6451 = v6449;
                                v6453 = v6441;
                                v6830 = v6434;
                                v6840 = v6442;
                                v6845 = v6445;
                                v9936 = v15870;
                                v9937 = v15858;
                                v9938 = v9943;
                                v9939 = v9944;
                                v9940 = v9945;
                            } else {
                                v6451 = v6303;
                                v6453 = v6301;
                                v6830 = v0;
                                v6840 = v0;
                                v6845 = v0;
                                v9936 = v15848;
                                v9937 = v15846;
                                v9938 = v10549;
                                v9939 = v10549;
                                v9940 = v10549;
                            }
                            v6450 = v6451;
                            v6452 = v6453;
                            v6829 = v6830;
                            v6839 = v6840;
                            v6844 = v6845;
                            v9924 = v9936;
                            v9925 = v9937;
                            v9926 = v9938;
                            v9927 = v9939;
                            v9928 = v9940;
                        }
                        let v6454 = v6450 - v6452;
                        let v16105 = v9924 - v9925;
                        let v8544: f64;
                        let v8552: f64;
                        let v8559: f64;
                        let v8571: f64;
                        let v8584: f64;
                        let v8590: f64;
                        let v8601: f64;
                        let v8607: f64;
                        let v9956: Lanes<5>;
                        let v9957: Lanes<5>;
                        let v9958: Lanes<6>;
                        let v9959: Lanes<6>;
                        let v9960: Lanes<5>;
                        let v9961: Lanes<6>;
                        let v9962: Lanes<5>;
                        let v9963: Lanes<6>;
                        if v6455 != 0.0 {
                            let v8545: f64;
                            let v8602: f64;
                            let v9964: Lanes<5>;
                            let v9965: Lanes<5>;
                            if v6044 != 0.0 {
                                let v6458 = -v6456;
                                let v6459 = v6458 * v6450;
                                let v16114 = v9924 * v6458;
                                let v6460 = v6458 * v6454;
                                let v16115 = v16105 * v6458;
                                v8545 = v6459;
                                v8602 = v6460;
                                v9964 = v16114;
                                v9965 = v16115;
                            } else {
                                v8545 = v0;
                                v8602 = v0;
                                v9964 = v10549;
                                v9965 = v10549;
                            }
                            let v8553: f64;
                            let v8585: f64;
                            let v9966: Lanes<5>;
                            let v9967: Lanes<5>;
                            if v6045 != 0.0 {
                                let v6461 = -v6456;
                                let v6462 = v6461 * v6450;
                                let v16116 = v9924 * v6461;
                                let v6463 = v6461 * v6454;
                                let v16117 = v16105 * v6461;
                                v8553 = v6462;
                                v8585 = v6463;
                                v9966 = v16116;
                                v9967 = v16117;
                            } else {
                                v8553 = v0;
                                v8585 = v0;
                                v9966 = v10549;
                                v9967 = v10549;
                            }
                            v8544 = v8545;
                            v8552 = v8553;
                            v8559 = v6030;
                            v8571 = v6029;
                            v8584 = v8585;
                            v8590 = v6027;
                            v8601 = v8602;
                            v8607 = v6028;
                            v9956 = v9964;
                            v9957 = v9966;
                            v9958 = v15583;
                            v9959 = v15582;
                            v9960 = v9967;
                            v9961 = v15580;
                            v9962 = v9965;
                            v9963 = v15581;
                        } else {
                            let v8560: f64;
                            let v8572: f64;
                            let v8591: f64;
                            let v8608: f64;
                            let v9968: Lanes<6>;
                            let v9969: Lanes<6>;
                            let v9970: Lanes<6>;
                            let v9971: Lanes<6>;
                            if v6464 != 0.0 {
                                let v8561: f64;
                                let v8609: f64;
                                let v9972: Lanes<6>;
                                let v9973: Lanes<6>;
                                if v6044 != 0.0 {
                                    let v6465 = -v6456;
                                    let v6466 = v6465 * v6450;
                                    let v16106 = v9924 * v6465;
                                    let v6467 = v6465 * v6454;
                                    let v16107 = v16105 * v6465;
                                    let v16108 = Lanes([v16106[0], v16106[1], v16106[2], v16106[3], v16106[4], 0.0]);
                                    let v16109 = Lanes([v16107[0], v16107[1], v16107[2], v16107[3], v16107[4], 0.0]);
                                    v8561 = v6466;
                                    v8609 = v6467;
                                    v9972 = v16108;
                                    v9973 = v16109;
                                } else {
                                    v8561 = v6030;
                                    v8609 = v6028;
                                    v9972 = v15583;
                                    v9973 = v15581;
                                }
                                let v8573: f64;
                                let v8592: f64;
                                let v9974: Lanes<6>;
                                let v9975: Lanes<6>;
                                if v6045 != 0.0 {
                                    let v6468 = -v6456;
                                    let v6469 = v6468 * v6450;
                                    let v16110 = v9924 * v6468;
                                    let v6470 = v6468 * v6454;
                                    let v16111 = v16105 * v6468;
                                    let v16112 = Lanes([v16110[0], v16110[1], v16110[2], v16110[3], v16110[4], 0.0]);
                                    let v16113 = Lanes([v16111[0], v16111[1], v16111[2], v16111[3], v16111[4], 0.0]);
                                    v8573 = v6469;
                                    v8592 = v6470;
                                    v9974 = v16112;
                                    v9975 = v16113;
                                } else {
                                    v8573 = v6029;
                                    v8592 = v6027;
                                    v9974 = v15582;
                                    v9975 = v15580;
                                }
                                v8560 = v8561;
                                v8572 = v8573;
                                v8591 = v8592;
                                v8608 = v8609;
                                v9968 = v9972;
                                v9969 = v9974;
                                v9970 = v9975;
                                v9971 = v9973;
                            } else {
                                v8560 = v6030;
                                v8572 = v6029;
                                v8591 = v6027;
                                v8608 = v6028;
                                v9968 = v15583;
                                v9969 = v15582;
                                v9970 = v15580;
                                v9971 = v15581;
                            }
                            v8544 = v0;
                            v8552 = v0;
                            v8559 = v8560;
                            v8571 = v8572;
                            v8584 = v0;
                            v8590 = v8591;
                            v8601 = v0;
                            v8607 = v8608;
                            v9956 = v10549;
                            v9957 = v10549;
                            v9958 = v9968;
                            v9959 = v9969;
                            v9960 = v10549;
                            v9961 = v9970;
                            v9962 = v10549;
                            v9963 = v9971;
                        }
                        let v6474 = (v6471 * v6046) + v6048;
                        let v6476 = (v6471 * v6048) + v6046;
                        let v6479 = (v6474 * v6059) + (v6476 * v6062);
                        let v16120 = (v15597 * v6474) + (v15600 * v6476);
                        let v6485 = -(((v6474 * v6064) + (v6476 * v6063)) + v6483);
                        let v16124 = ((v15603 * v6474) + (v15602 * v6476)) * v10360;
                        let v6486 = if v6485 > v783 { 1.0 } else { 0.0 };
                        let v6501: f64;
                        let v9976: Lanes<3>;
                        if v6486 != 0.0 {
                            let v6488 = v779 - v783;
                            let v6489 = (v6485 - v783) / v6488;
                            let v16125 = v16124 / v6488;
                            let v6490 = v6489 * v6489;
                            let v16126 = v16125 * v6489;
                            let v16127 = v16126 + v16126;
                            let v16131 = v16127 * v6490;
                            let v6496 = (((v1 + v6489) + v6490) + (v6490 * v6489)) + (v6490 * v6490);
                            let v6497 = v1 / v6496;
                            let v16140 = (((((((v16125 + v16127) + ((v16127 * v6489) + (v16125 * v6490))) + (v16131 + v16131)) * v6497) * v10360) / v6496) * v10360) * v6488;
                            let v6500 = v783 + (v6488 * (v1 - v6497));
                            v6501 = v6500;
                            v9976 = v16140;
                        } else {
                            v6501 = v6485;
                            v9976 = v16124;
                        }
                        let v16141 = v9976 * v10360;
                        let v6503 = (-v6501) - v11;
                        let v6504 = v6479 - v6098;
                        let v6505 = -v6503;
                        let v16142 = v16141 * v10360;
                        let v6506 = if v6504 < v6505 { 1.0 } else { 0.0 };
                        let v6850: f64;
                        let v6852: f64;
                        let v9977: Lanes<5>;
                        let v9978: Lanes<5>;
                        if v6506 != 0.0 {
                            let v6507 = v663 * v6043;
                            let v6508 = v1 / v6507;
                            let v6509 = v6508 * v127;
                            let v16526 = (((((v10380 * v6043) + (v15584 * v663)) * v6508) * v10360) / v6507) * v127;
                            let v16527 = v16526 * v6510;
                            let v6512 = v78 + (v6510 * v6509);
                            let v6513 = v91 * v6512;
                            let v6514 = v6513 * v6512;
                            let v6515 = v6514 * v6512;
                            let v16534 = ((((v16527 * v91) * v6512) + (v16527 * v6513)) * v6512) + (v16527 * v6514);
                            let v6516 = v661 - v6104;
                            let v16535 = v10376 - v15641;
                            let v6517 = v6504 + v6503;
                            let v16539 = v10380 * v6517;
                            let v16540 = ((Lanes([v16120[0], v16120[1], v16120[2], 0.0])) + (Lanes([v16141[0], v16141[1], 0.0, v16141[2]]))) * v663;
                            let v6520 = v3500 * v6509;
                            let v6521 = (v663 * v6517) - v78;
                            let v6522 = v6520 * v6521;
                            let v16545 = (v16526 * v3500) * v6521;
                            let v16548 = (Lanes([0.0, 0.0, v16545[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v16539[0], 0.0, 0.0])) + (Lanes([v16540[0], v16540[1], 0.0, v16540[2], v16540[3]]))) * v6520);
                            let v6523 = v6519 - v6522;
                            let v16549 = v16548 * v10360;
                            let v6524 = v6523 * v6523;
                            let v16550 = v16549 * v6523;
                            let v16551 = v16550 + v16550;
                            let v6526 = if v6515 < (v6524 * v3506) { 1.0 } else { 0.0 };
                            let v6538: f64;
                            let v9979: Lanes<5>;
                            if v6526 != 0.0 {
                                let v16558 = v16534 * v13;
                                let v6530 = (v13 * v6515) / v6523;
                                let v6532 = ((v6527 + v6523) + v6530) + v6522;
                                let v16564 = (v16549 + (((Lanes([0.0, 0.0, v16558[0], 0.0, 0.0])) - (v16549 * v6530)) / v6523)) + v16548;
                                v6538 = v6532;
                                v9979 = v16564;
                            } else {
                                let v6534 = (v6515 + v6524).sqrt();
                                let v6537 = (v6535 + v6534) + v6522;
                                let v16557 = (((Lanes([0.0, 0.0, v16534[0], 0.0, 0.0])) + v16551) * (v9353 / (v10405 * v6534))) + v16548;
                                v6538 = v6537;
                                v9979 = v16557;
                            }
                            let v6539 = v6538.powf(v1562);
                            let v16568 = v9979 * (v1562 * (v6538.powf(v16565)));
                            let v16570 = (v16526 * v3523) * v10360;
                            let v6545 = v748 * v6539;
                            let v6548 = (((v6540 - (v3523 * v6509)) + (v78 * v6539)) + (v6545 * v6539)) / v6539;
                            let v16583 = v10385 * v6548;
                            let v16586 = Lanes([v16141[0], v16141[1], 0.0, 0.0, v16141[2]]);
                            let v6551 = ((v6548 * v665) - v6503) + v6503;
                            let v16588 = ((((((((Lanes([0.0, 0.0, v16570[0], 0.0, 0.0])) + (v16568 * v78)) + (((v16568 * v748) * v6539) + (v16568 * v6545))) - (v16568 * v6548)) / v6539) * v665) + (Lanes([0.0, 0.0, v16583[0], 0.0, 0.0]))) - v16586) + v16586;
                            let v6552 = v6551 / v6516;
                            let v16589 = v16535 * v6552;
                            let v16593 = ((v16588 - (Lanes([0.0, 0.0, v16589[0], 0.0, 0.0]))) / v6516) * v6552;
                            let v6555 = (v1 + (v6552 * v6552)).sqrt();
                            let v6556 = v6551 / v6555;
                            let v6559 = v127 * (v6504 - (v6556 - v6503));
                            let v16604 = ((Lanes([v16120[0], v16120[1], 0.0, v16120[2], 0.0])) - (((v16588 - (((v16593 + v16593) * (v9353 / (v10405 * v6555))) * v6556)) / v6555) - v16586)) * v127;
                            v6850 = v6559;
                            v6852 = v6559;
                            v9977 = v16604;
                            v9978 = v16604;
                        } else {
                            let v6561 = v6504 + v6503;
                            let v16145 = (Lanes([v16120[0], v16120[1], v16120[2], 0.0])) + (Lanes([v16141[0], v16141[1], 0.0, v16141[2]]));
                            let v16146 = v10380 * v6561;
                            let v16147 = v16145 * v663;
                            let v16149 = Lanes([v16147[0], v16147[1], 0.0, v16147[2], v16147[3]]);
                            let v16150 = (Lanes([0.0, 0.0, v16146[0], 0.0, 0.0])) + v16149;
                            let v6563 = (v663 * v6561) - v1;
                            let v6566 = v6097 * v664;
                            let v16154 = (v15630 * v664) + (v10382 * v6097);
                            let v6567 = (v90 * (v6563 + v6560)) / v6566;
                            let v16155 = v16154 * v6567;
                            let v16158 = ((v16150 * v90) - (Lanes([0.0, 0.0, v16155[0], 0.0, 0.0]))) / v6566;
                            let v6568 = v1 + v6567;
                            let v6570 = if v6568 < v6569 { 1.0 } else { 0.0 };
                            let v6574: f64;
                            let v9980: Lanes<5>;
                            if v6570 != 0.0 {
                                v6574 = v6571;
                                v9980 = v10549;
                            } else {
                                v6574 = v6568;
                                v9980 = v16158;
                            }
                            let v6573 = (v6097 * v663) / v78;
                            let v16162 = ((v15630 * v663) + (v10380 * v6097)) / v78;
                            let v6575 = v6574.sqrt();
                            let v6576 = v1 - v6575;
                            let v16167 = v16162 * v6576;
                            let v16171 = Lanes([v16120[0], v16120[1], 0.0, v16120[2], 0.0]);
                            let v6579 = (v6504 + (v6573 * v6576)) + v6503;
                            let v16173 = Lanes([v16141[0], v16141[1], 0.0, 0.0, v16141[2]]);
                            let v16175 = v10380 * v6579;
                            let v6582 = (-(v663 * v6579)).exp();
                            let v6585 = (v90 * (v6563 + v6582)) / v6566;
                            let v16183 = v16154 * v6585;
                            let v16186 = (((v16150 + ((((Lanes([0.0, 0.0, v16175[0], 0.0, 0.0])) + (((v16171 + ((Lanes([0.0, 0.0, v16167[0], 0.0, 0.0])) + (((v9980 * (v9353 / (v10405 * v6575))) * v10360) * v6573))) + v16173) * v663)) * v10360) * v6582)) * v90) - (Lanes([0.0, 0.0, v16183[0], 0.0, 0.0]))) / v6566;
                            let v6586 = v1 + v6585;
                            let v6588 = if v6586 < v6587 { 1.0 } else { 0.0 };
                            let v6590: f64;
                            let v9981: Lanes<5>;
                            if v6588 != 0.0 {
                                v6590 = v6589;
                                v9981 = v10549;
                            } else {
                                v6590 = v6586;
                                v9981 = v16186;
                            }
                            let v6591 = v6590.sqrt();
                            let v6592 = v1 - v6591;
                            let v16191 = v16162 * v6592;
                            let v6595 = (v6504 + (v6573 * v6592)) + v6503;
                            let v6596 = v663 * v6595;
                            let v16197 = v10380 * v6595;
                            let v16200 = (Lanes([0.0, 0.0, v16197[0], 0.0, 0.0])) + (((v16171 + ((Lanes([0.0, 0.0, v16191[0], 0.0, 0.0])) + (((v9981 * (v9353 / (v10405 * v6591))) * v10360) * v6573))) + v16173) * v663);
                            let v6597 = if v6596 < v96 { 1.0 } else { 0.0 };
                            let v6674: f64;
                            let v9982: Lanes<5>;
                            if v6597 != 0.0 {
                                let v6600 = v663 * v6096;
                                let v6601 = v1 / v6600;
                                let v16206 = ((((v10380 * v6096) + (v15628 * v663)) * v6601) * v10360) / v6600;
                                let v6602 = v6599 + v6601;
                                let v16207 = v16145 * v10360;
                                let v6604 = (-v6561) / v6096;
                                let v16208 = v15628 * v6604;
                                let v16215 = ((v16206 * v6598) / v6607) * v10360;
                                let v6612 = (v6605 - ((v6598 * v6602) / v6607)) + (v6604 / v6610);
                                let v16218 = (Lanes([0.0, 0.0, v16215[0], 0.0, 0.0])) + ((((Lanes([v16207[0], v16207[1], 0.0, v16207[2], v16207[3]])) - (Lanes([0.0, 0.0, v16208[0], 0.0, 0.0]))) / v6096) / v6610);
                                let v6618 = ((v6613 * v6602) - v6615) / v6617;
                                let v16220 = (v16206 * v6613) / v6617;
                                let v16221 = v16218 * v6612;
                                let v6620 = v6618 * v6618;
                                let v16223 = v16220 * v6618;
                                let v16227 = ((v16223 + v16223) * v6618) + (v16220 * v6620);
                                let v6623 = ((v6612 * v6612) + (v6620 * v6618)).sqrt();
                                let v16232 = ((v16221 + v16221) + (Lanes([0.0, 0.0, v16227[0], 0.0, 0.0]))) * (v9353 / (v10405 * v6623));
                                let v6625 = (-v6612) + v6623;
                                let v6627 = v6612 + v6623;
                                let v6632 = ((v6625.powf(v1562)) + (-(v6627.powf(v1562)))) - v6631;
                                let v16247 = v10385 * v6632;
                                let v6635 = ((v6632 * v665) - v6503) + v6503;
                                let v6636 = v663 * v6635;
                                let v16252 = v10380 * v6635;
                                let v16255 = (Lanes([0.0, 0.0, v16252[0], 0.0, 0.0])) + (((((((((v16218 * v10360) + v16232) * (v1562 * (v6625.powf(v16235)))) + (((v16218 + v16232) * (v1562 * (v6627.powf(v16240)))) * v10360)) * v665) + (Lanes([0.0, 0.0, v16247[0], 0.0, 0.0]))) - v16173) + v16173) * v663);
                                v6674 = v6636;
                                v9982 = v16255;
                            } else {
                                v6674 = v6596;
                                v9982 = v16200;
                            }
                            let v6637 = v6561 + v79;
                            let v16256 = v10380 * v6505;
                            let v16257 = v16142 * v663;
                            let v6639 = (v663 * v6505).exp();
                            let v16261 = ((Lanes([0.0, 0.0, v16256[0], 0.0])) + (Lanes([v16257[0], v16257[1], 0.0, v16257[2]]))) * v6639;
                            let v6640 = v6639 + v362;
                            let v6641 = v731 / v489;
                            let v6642 = v6641 * v6641;
                            let v16263 = (v10423 / v489) * v6641;
                            let v16264 = v16263 + v16263;
                            let v6643 = v6642 * v6640;
                            let v16265 = v16264 * v6640;
                            let v16266 = v16261 * v6642;
                            let v6644 = v663 * v6637;
                            let v16269 = v10380 * v6637;
                            let v16271 = (Lanes([0.0, 0.0, v16269[0], 0.0, 0.0])) + v16149;
                            let v6645 = v6643 * v6566;
                            let v16273 = v16154 * v6643;
                            let v16275 = (((Lanes([0.0, 0.0, v16265[0], 0.0])) + v16266) * v6566) + (Lanes([0.0, 0.0, v16273[0], 0.0]));
                            let v16276 = v16271 * v6644;
                            let v6647 = v6645 + (v6644 * v6644);
                            let v16278 = Lanes([v16275[0], v16275[1], v16275[2], 0.0, v16275[3]]);
                            let v6649 = v6642 * v6566;
                            let v6650 = v6649.ln();
                            let v16286 = ((v16264 * v6566) + (v16154 * v6642)) * (v9353 / v6649);
                            let v16287 = Lanes([0.0, 0.0, v16286[0], 0.0, 0.0]);
                            let v6652 = v663 * v6503;
                            let v16289 = v10380 * v6503;
                            let v16290 = v16141 * v663;
                            let v16293 = (Lanes([0.0, 0.0, v16289[0], 0.0])) + (Lanes([v16290[0], v16290[1], 0.0, v16290[2]]));
                            let v16294 = Lanes([v16293[0], v16293[1], v16293[2], 0.0, v16293[3]]);
                            let v16296 = v16271 - ((((v16278 + (v16276 + v16276)) * (v9353 / v6647)) - v16287) + v16294);
                            let v6655 = (v6644 - (((v6647.ln()) - v6650) + v6652)) - v1;
                            let v6656 = v90 * v6644;
                            let v16297 = v16271 * v90;
                            let v6657 = if v6656 > v0 { 1.0 } else { 0.0 };
                            let v6659: f64;
                            let v9983: Lanes<5>;
                            if v6657 != 0.0 {
                                v6659 = v6656;
                                v9983 = v16297;
                            } else {
                                let v6658 = -v6656;
                                let v16298 = v16297 * v10360;
                                v6659 = v6658;
                                v9983 = v16298;
                            }
                            let v16299 = v16296 * v6655;
                            let v6662 = ((v6655 * v6655) + v6659).sqrt();
                            let v16309 = v10380 * v79;
                            let v6668 = (v6644 - (v6644 - (v13 * (v6655 + v6662)))) + (v663 * v79);
                            let v16312 = ((v16271 - (v16271 - ((v16296 + (((v16299 + v16299) + v9983) * (v9353 / (v10405 * v6662)))) * v13))) + (Lanes([0.0, 0.0, v16309[0], 0.0, 0.0]))) * v6668;
                            let v6670 = v6645 + (v6668 * v6668);
                            let v6673 = ((v6670.ln()) - v6650) + v6652;
                            let v16318 = (((v16278 + (v16312 + v16312)) * (v9353 / v6670)) - v16287) + v16294;
                            let v16319 = v16318 - v9982;
                            let v6677 = (v6673 - v6674) - v6676;
                            let v6680 = (v90 * v6673) * v6679;
                            let v16321 = (v16318 * v90) * v6679;
                            let v6681 = if v6680 > v0 { 1.0 } else { 0.0 };
                            let v6683: f64;
                            let v9984: Lanes<5>;
                            if v6681 != 0.0 {
                                v6683 = v6680;
                                v9984 = v16321;
                            } else {
                                let v6682 = -v6680;
                                let v16322 = v16321 * v10360;
                                v6683 = v6682;
                                v9984 = v16322;
                            }
                            let v16323 = v16319 * v6677;
                            let v6686 = ((v6677 * v6677) + v6683).sqrt();
                            let v6689 = v6673 - (v13 * (v6677 + v6686));
                            let v16331 = v16318 - ((v16319 + (((v16323 + v16323) + v9984) * (v9353 / (v10405 * v6686)))) * v13);
                            let v6690 = v6689 / v663;
                            let v16332 = v10380 * v6690;
                            let v6691 = v6690 - v6503;
                            let v16336 = ((v16331 - (Lanes([0.0, 0.0, v16332[0], 0.0, 0.0]))) / v663) - v16173;
                            let v6694 = (-v6689).exp();
                            let v6695 = (v6689 - v1) + v6694;
                            let v16339 = v16331 + ((v16331 * v10360) * v6694);
                            let v6697 = if v6695 < v6696 { 1.0 } else { 0.0 };
                            let v6699: f64;
                            let v9985: Lanes<5>;
                            if v6697 != 0.0 {
                                v6699 = v6698;
                                v9985 = v10549;
                            } else {
                                v6699 = v6695;
                                v9985 = v16339;
                            }
                            let v6700 = v6699.sqrt();
                            let v6701 = v6043 * v6700;
                            let v16343 = v15584 * v6700;
                            let v16346 = (Lanes([0.0, 0.0, v16343[0], 0.0, 0.0])) + ((v9985 * (v9353 / (v10405 * v6700))) * v6043);
                            let v6703 = v127 * (v6504 - v6691);
                            let v16348 = (v16171 - v16336) * v127;
                            let v6704 = if v6304 == v1 { 1.0 } else { 0.0 };
                            let v6851: f64;
                            let v6853: f64;
                            let v9986: Lanes<5>;
                            let v9987: Lanes<5>;
                            if v6704 != 0.0 {
                                let v6705 = v6642 * v6639;
                                let v16349 = v16264 * v6639;
                                let v16351 = (Lanes([0.0, 0.0, v16349[0], 0.0])) + v16266;
                                let mut v6706: f64 = 0.0;
                                let mut v6709: f64 = 0.0;
                                let mut v6795: f64 = 0.0;
                                let mut v6825: f64 = 0.0;
                                let mut v6828: f64 = 0.0;
                                let mut v6838: f64 = 0.0;
                                let mut v6843: f64 = 0.0;
                                let mut v9988: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9989: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9990: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9991: Lanes<5> = Lanes([0.0; 5]);
                                let mut v9992: Lanes<5> = Lanes([0.0; 5]);
                                v6706 = v1;
                                v6709 = v6691;
                                v6795 = v0;
                                v6825 = v6689;
                                v6828 = v6829;
                                v6838 = v6839;
                                v6843 = v6844;
                                v9988 = v16336;
                                v9989 = v16331;
                                v9990 = v9926;
                                v9991 = v9927;
                                v9992 = v9928;
                                loop {
                                    let v6708 = if v6706 <= v6707 { 1.0 } else { 0.0 };
                                    if v6708 == 0.0 {
                                        break;
                                    }
                                    let v6710 = v6709 + v6503;
                                    let v6711 = v663 * v6710;
                                    let v16372 = v10380 * v6710;
                                    let v16375 = (Lanes([0.0, 0.0, v16372[0], 0.0, 0.0])) + ((v9988 + v16173) * v663);
                                    let v6712 = if v6711 < v644 { 1.0 } else { 0.0 };
                                    let v6788: f64;
                                    let v6792: f64;
                                    let v6831: f64;
                                    let v6846: f64;
                                    let v9993: Lanes<5>;
                                    let v9994: Lanes<5>;
                                    let v9995: Lanes<5>;
                                    let v9996: Lanes<5>;
                                    if v6712 != 0.0 {
                                        let v6713 = v6711 * v6711;
                                        let v16417 = v16375 * v6711;
                                        let v16418 = v16417 + v16417;
                                        let v6714 = v6713 * v6711;
                                        let v6717 = v6715 + (v6711 * v6318);
                                        let v6719 = v6316 + (v6711 * v6717);
                                        let v6720 = v6714 * v6719;
                                        let v16428 = (((v16418 * v6711) + (v16375 * v6713)) * v6719) + (((v16375 * v6717) + ((v16375 * v6318) * v6711)) * v6714);
                                        let v6723 = v6711 * v644;
                                        let v16429 = v16375 * v644;
                                        let v6725 = v6722 + (v6723 * v6318);
                                        let v6727 = v6721 + (v6711 * v6725);
                                        let v6728 = v6713 * v6727;
                                        let v6729 = v6705 * v6720;
                                        let v16437 = v16351 * v6720;
                                        let v6730 = v6729 * v6720;
                                        let v16443 = (((Lanes([v16437[0], v16437[1], v16437[2], 0.0, v16437[3]])) + (v16428 * v6705)) * v6720) + (v16428 * v6729);
                                        let v16445 = v10380 * v6705;
                                        let v6732 = (v6705 * v663) * v78;
                                        let v6733 = v6732 * v6720;
                                        let v16449 = (((v16351 * v663) + (Lanes([0.0, 0.0, v16445[0], 0.0]))) * v78) * v6720;
                                        let v6738 = v6736 + (v6711 * v6342);
                                        let v6740 = v6340 + (v6711 * v6738);
                                        let v6742 = v6735 + (v6711 * v6740);
                                        let v6744 = v6338 + (v6711 * v6742);
                                        let v6745 = v6711 * v6744;
                                        let v16468 = (v16375 * v6744) + (((v16375 * v6742) + (((v16375 * v6740) + (((v16375 * v6738) + ((v16375 * v6342) * v6711)) * v6711)) * v6711)) * v6711);
                                        let v6750 = v6748 + (v6723 * v6342);
                                        let v6752 = v6747 + (v6711 * v6750);
                                        let v6754 = v6746 + (v6711 * v6752);
                                        let v6756 = v6338 + (v6711 * v6754);
                                        let v16479 = v16468 * v6745;
                                        let v6760 = (((v6745 * v6745) + v6730) + v362).sqrt();
                                        let v16484 = ((v16479 + v16479) + v16443) * (v9353 / (v10405 * v6760));
                                        let v16485 = v10380 * v6756;
                                        let v6762 = (v663 * v6756) * v78;
                                        let v6765 = v6760 + v6760;
                                        let v6766 = ((v6762 * v6745) + (v6733 * v6728)) / v6765;
                                        let v16497 = (((((((Lanes([0.0, 0.0, v16485[0], 0.0, 0.0])) + (((v16375 * v6754) + (((v16375 * v6752) + (((v16375 * v6750) + ((v16429 * v6342) * v6711)) * v6711)) * v6711)) * v663)) * v78) * v6745) + (v16468 * v6762)) + ((((Lanes([v16449[0], v16449[1], v16449[2], 0.0, v16449[3]])) + (v16428 * v6732)) * v6728) + (((v16418 * v6727) + (((v16375 * v6725) + ((v16429 * v6318) * v6711)) * v6713)) * v6733))) - ((v16484 + v16484) * v6766)) / v6765;
                                        v6788 = v6760;
                                        v6792 = v6766;
                                        v6831 = v6745;
                                        v6846 = v6730;
                                        v9993 = v16484;
                                        v9994 = v16497;
                                        v9995 = v16468;
                                        v9996 = v16443;
                                    } else {
                                        let v6767 = if v6711 < v2535 { 1.0 } else { 0.0 };
                                        let v6780: f64;
                                        let v6783: f64;
                                        let v9997: Lanes<5>;
                                        let v9998: Lanes<5>;
                                        if v6767 != 0.0 {
                                            let v6768 = v6711.exp();
                                            let v16394 = v16375 * v6768;
                                            let v6769 = v6768 - v1;
                                            let v6770 = v6705 * v6769;
                                            let v16395 = v16351 * v6769;
                                            let v16398 = (Lanes([v16395[0], v16395[1], v16395[2], 0.0, v16395[3]])) + (v16394 * v6705);
                                            let v6771 = v6705 * v663;
                                            let v16400 = v10380 * v6705;
                                            let v6772 = v6771 * v6768;
                                            let v16403 = ((v16351 * v663) + (Lanes([0.0, 0.0, v16400[0], 0.0]))) * v6768;
                                            let v16406 = (Lanes([v16403[0], v16403[1], v16403[2], 0.0, v16403[3]])) + (v16394 * v6771);
                                            v6780 = v6770;
                                            v6783 = v6772;
                                            v9997 = v16398;
                                            v9998 = v16406;
                                        } else {
                                            let v16376 = v10380 * v6709;
                                            let v6774 = (v663 * v6709).exp();
                                            let v16380 = ((Lanes([0.0, 0.0, v16376[0], 0.0, 0.0])) + (v9988 * v663)) * v6774;
                                            let v6775 = v6774 - v6639;
                                            let v6776 = v6642 * v6775;
                                            let v16383 = v16264 * v6775;
                                            let v16386 = (Lanes([0.0, 0.0, v16383[0], 0.0, 0.0])) + ((v16380 - (Lanes([v16261[0], v16261[1], v16261[2], 0.0, v16261[3]]))) * v6642);
                                            let v6777 = v6642 * v663;
                                            let v6778 = v6777 * v6774;
                                            let v16390 = ((v16264 * v663) + (v10380 * v6642)) * v6774;
                                            let v16393 = (Lanes([0.0, 0.0, v16390[0], 0.0, 0.0])) + (v16380 * v6777);
                                            v6780 = v6776;
                                            v6783 = v6778;
                                            v9997 = v16386;
                                            v9998 = v16393;
                                        }
                                        let v6782 = ((v6711 - v1) + v6780).sqrt();
                                        let v16410 = (v16375 + v9997) * (v9353 / (v10405 * v6782));
                                        let v6785 = (v663 + v6783) / v6782;
                                        let v6786 = v6785 * v13;
                                        let v16416 = ((((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + v9998) - (v16410 * v6785)) / v6782) * v13;
                                        v6788 = v6782;
                                        v6792 = v6786;
                                        v6831 = v0;
                                        v6846 = v6780;
                                        v9993 = v16410;
                                        v9994 = v16416;
                                        v9995 = v10549;
                                        v9996 = v9997;
                                    }
                                    let v16499 = v15628 * v6788;
                                    let v6790 = (v6504 - v6709) - (v6096 * v6788);
                                    let v16503 = (v16171 - v9988) - ((Lanes([0.0, 0.0, v16499[0], 0.0, 0.0])) + (v9993 * v6096));
                                    let v16504 = v15628 * v6792;
                                    let v6794 = v6791 - (v6096 * v6792);
                                    let v16508 = ((Lanes([0.0, 0.0, v16504[0], 0.0, 0.0])) + (v9994 * v6096)) * v10360;
                                    let v6796 = if v6795 == v1 { 1.0 } else { 0.0 };
                                    let v6819: f64;
                                    let v6821: f64;
                                    let v6822: f64;
                                    let v9999: Lanes<5>;
                                    if v6796 != 0.0 {
                                        v6819 = v6797;
                                        v6821 = v6709;
                                        v6822 = v6795;
                                        v9999 = v9988;
                                    } else {
                                        let v6799 = (-v6790) / v6794;
                                        let v16512 = ((v16503 * v10360) - (v16508 * v6799)) / v6794;
                                        let v6801 = v6709.abs();
                                        let v16516 = v9988 * ((v10405 * (if v6709 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                                        let v6802 = if v1 >= v6801 { 1.0 } else { 0.0 };
                                        let v6803: f64;
                                        let v10000: Lanes<5>;
                                        if v6802 != 0.0 {
                                            v6803 = v1;
                                            v10000 = v10549;
                                        } else {
                                            v6803 = v6801;
                                            v10000 = v16516;
                                        }
                                        let v6805 = v6800 * (v1 + v6803);
                                        let v16517 = v10000 * v6800;
                                        let v6807 = if (v6799.abs()) > v6805 { 1.0 } else { 0.0 };
                                        let v6812: f64;
                                        let v10001: Lanes<5>;
                                        if v6807 != 0.0 {
                                            let v6808 = if v6799 >= v0 { 1.0 } else { 0.0 };
                                            let v6810: f64;
                                            if v6808 != 0.0 {
                                                v6810 = v1;
                                            } else {
                                                v6810 = v6809;
                                            }
                                            let v6811 = v6805 * v6810;
                                            let v16518 = v16517 * v6810;
                                            v6812 = v6811;
                                            v10001 = v16518;
                                        } else {
                                            v6812 = v6799;
                                            v10001 = v16512;
                                        }
                                        let v6813 = v6709 + v6812;
                                        let v16519 = v9988 + v10001;
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
                                        v9999 = v16519;
                                    }
                                    let v6820 = v6819 + v1;
                                    v6706 = v6820;
                                    v6709 = v6821;
                                    v6795 = v6822;
                                    v6825 = v6711;
                                    v6828 = v6831;
                                    v6838 = v6788;
                                    v6843 = v6846;
                                    v9988 = v9999;
                                    v9989 = v16375;
                                    v9990 = v9995;
                                    v9991 = v9993;
                                    v9992 = v9996;
                                }
                                let v6824 = if v6795 == v0 { 1.0 } else { 0.0 };
                                if v6824 != 0.0 {
                                } else {
                                }
                                let v6826 = if v6825 < v644 { 1.0 } else { 0.0 };
                                let v6836: f64;
                                let v10002: Lanes<5>;
                                if v6826 != 0.0 {
                                    let v6827 = if v6825 < v96 { 1.0 } else { 0.0 };
                                    if v6827 != 0.0 {
                                    } else {
                                    }
                                    let v6833 = v6828 + v6832;
                                    v6836 = v6833;
                                    v10002 = v9990;
                                } else {
                                    let v6835 = (v6825 - v1).sqrt();
                                    let v16354 = v9989 * (v9353 / (v10405 * v6835));
                                    v6836 = v6835;
                                    v10002 = v16354;
                                }
                                let v6837 = v6043 * v6836;
                                let v16355 = v15584 * v6836;
                                let v16358 = (Lanes([0.0, 0.0, v16355[0], 0.0, 0.0])) + (v10002 * v6043);
                                let v6841 = v6838 + v6836;
                                let v6842 = v1 / v6841;
                                let v6847 = v6043 * v6843;
                                let v16363 = v15584 * v6843;
                                let v6849 = v6837 + (v6847 * v6842);
                                let v16370 = v16358 + ((((Lanes([0.0, 0.0, v16363[0], 0.0, 0.0])) + (v9992 * v6043)) * v6842) + (((((v9991 + v10002) * v6842) * v10360) / v6841) * v6847));
                                v6851 = v6849;
                                v6853 = v6837;
                                v9986 = v16370;
                                v9987 = v16358;
                            } else {
                                v6851 = v6703;
                                v6853 = v6701;
                                v9986 = v16348;
                                v9987 = v16346;
                            }
                            v6850 = v6851;
                            v6852 = v6853;
                            v9977 = v9986;
                            v9978 = v9987;
                        }
                        let v6854 = v6850 - v6852;
                        let v16605 = v9977 - v9978;
                        let v8542: f64;
                        let v8550: f64;
                        let v8558: f64;
                        let v8570: f64;
                        let v8582: f64;
                        let v8589: f64;
                        let v8599: f64;
                        let v8606: f64;
                        let v10003: Lanes<5>;
                        let v10004: Lanes<5>;
                        let v10005: Lanes<6>;
                        let v10006: Lanes<6>;
                        let v10007: Lanes<5>;
                        let v10008: Lanes<6>;
                        let v10009: Lanes<5>;
                        let v10010: Lanes<6>;
                        if v6855 != 0.0 {
                            let v8543: f64;
                            let v8600: f64;
                            let v10011: Lanes<5>;
                            let v10012: Lanes<5>;
                            if v6471 != 0.0 {
                                let v6856 = -v6456;
                                let v6857 = v6856 * v6850;
                                let v16614 = v9977 * v6856;
                                let v6858 = v6856 * v6854;
                                let v16615 = v16605 * v6856;
                                v8543 = v6857;
                                v8600 = v6858;
                                v10011 = v16614;
                                v10012 = v16615;
                            } else {
                                v8543 = v8544;
                                v8600 = v8601;
                                v10011 = v9956;
                                v10012 = v9962;
                            }
                            let v8551: f64;
                            let v8583: f64;
                            let v10013: Lanes<5>;
                            let v10014: Lanes<5>;
                            if v6472 != 0.0 {
                                let v6859 = -v6456;
                                let v6860 = v6859 * v6850;
                                let v16616 = v9977 * v6859;
                                let v6861 = v6859 * v6854;
                                let v16617 = v16605 * v6859;
                                v8551 = v6860;
                                v8583 = v6861;
                                v10013 = v16616;
                                v10014 = v16617;
                            } else {
                                v8551 = v8552;
                                v8583 = v8584;
                                v10013 = v9957;
                                v10014 = v9960;
                            }
                            v8542 = v8543;
                            v8550 = v8551;
                            v8558 = v8559;
                            v8570 = v8571;
                            v8582 = v8583;
                            v8589 = v8590;
                            v8599 = v8600;
                            v8606 = v8607;
                            v10003 = v10011;
                            v10004 = v10013;
                            v10005 = v9958;
                            v10006 = v9959;
                            v10007 = v10014;
                            v10008 = v9961;
                            v10009 = v10012;
                            v10010 = v9963;
                        } else {
                            let v8562: f64;
                            let v8574: f64;
                            let v8593: f64;
                            let v8610: f64;
                            let v10015: Lanes<6>;
                            let v10016: Lanes<6>;
                            let v10017: Lanes<6>;
                            let v10018: Lanes<6>;
                            if v6862 != 0.0 {
                                let v8563: f64;
                                let v8611: f64;
                                let v10019: Lanes<6>;
                                let v10020: Lanes<6>;
                                if v6471 != 0.0 {
                                    let v6863 = -v6456;
                                    let v6864 = v6863 * v6850;
                                    let v16606 = v9977 * v6863;
                                    let v6865 = v6863 * v6854;
                                    let v16607 = v16605 * v6863;
                                    let v16608 = Lanes([v16606[0], v16606[1], v16606[2], v16606[3], v16606[4], 0.0]);
                                    let v16609 = Lanes([v16607[0], v16607[1], v16607[2], v16607[3], v16607[4], 0.0]);
                                    v8563 = v6864;
                                    v8611 = v6865;
                                    v10019 = v16608;
                                    v10020 = v16609;
                                } else {
                                    v8563 = v8559;
                                    v8611 = v8607;
                                    v10019 = v9958;
                                    v10020 = v9963;
                                }
                                let v8575: f64;
                                let v8594: f64;
                                let v10021: Lanes<6>;
                                let v10022: Lanes<6>;
                                if v6472 != 0.0 {
                                    let v6866 = -v6456;
                                    let v6867 = v6866 * v6850;
                                    let v16610 = v9977 * v6866;
                                    let v6868 = v6866 * v6854;
                                    let v16611 = v16605 * v6866;
                                    let v16612 = Lanes([v16610[0], v16610[1], v16610[2], v16610[3], v16610[4], 0.0]);
                                    let v16613 = Lanes([v16611[0], v16611[1], v16611[2], v16611[3], v16611[4], 0.0]);
                                    v8575 = v6867;
                                    v8594 = v6868;
                                    v10021 = v16612;
                                    v10022 = v16613;
                                } else {
                                    v8575 = v8571;
                                    v8594 = v8590;
                                    v10021 = v9959;
                                    v10022 = v9961;
                                }
                                v8562 = v8563;
                                v8574 = v8575;
                                v8593 = v8594;
                                v8610 = v8611;
                                v10015 = v10019;
                                v10016 = v10021;
                                v10017 = v10022;
                                v10018 = v10020;
                            } else {
                                v8562 = v8559;
                                v8574 = v8571;
                                v8593 = v8590;
                                v8610 = v8607;
                                v10015 = v9958;
                                v10016 = v9959;
                                v10017 = v9961;
                                v10018 = v9963;
                            }
                            v8542 = v8544;
                            v8550 = v8552;
                            v8558 = v8562;
                            v8570 = v8574;
                            v8582 = v8584;
                            v8589 = v8593;
                            v8599 = v8601;
                            v8606 = v8610;
                            v10003 = v9956;
                            v10004 = v9957;
                            v10005 = v10015;
                            v10006 = v10016;
                            v10007 = v9960;
                            v10008 = v10017;
                            v10009 = v9962;
                            v10010 = v10018;
                        }
                        v8541 = v8542;
                        v8549 = v8550;
                        v8557 = v8558;
                        v8569 = v8570;
                        v8581 = v8582;
                        v8588 = v8589;
                        v8598 = v8599;
                        v8605 = v8606;
                        v9915 = v10003;
                        v9916 = v10004;
                        v9917 = v10005;
                        v9918 = v10006;
                        v9919 = v10007;
                        v9920 = v10008;
                        v9921 = v10009;
                        v9922 = v10010;
                    } else {
                        v8541 = v0;
                        v8549 = v0;
                        v8557 = v6030;
                        v8569 = v6029;
                        v8581 = v0;
                        v8588 = v6027;
                        v8598 = v0;
                        v8605 = v6028;
                        v9915 = v10549;
                        v9916 = v10549;
                        v9917 = v15583;
                        v9918 = v15582;
                        v9919 = v10549;
                        v9920 = v15580;
                        v9921 = v10549;
                        v9922 = v15581;
                    }
                    v8540 = v8541;
                    v8548 = v8549;
                    v8556 = v8557;
                    v8568 = v8569;
                    v8580 = v8581;
                    v8587 = v8588;
                    v8597 = v8598;
                    v8604 = v8605;
                    v9907 = v9915;
                    v9908 = v9916;
                    v9909 = v9917;
                    v9910 = v9918;
                    v9911 = v9919;
                    v9912 = v9920;
                    v9913 = v9921;
                    v9914 = v9922;
                } else {
                    v8540 = v0;
                    v8548 = v0;
                    v8556 = v6030;
                    v8568 = v6029;
                    v8580 = v0;
                    v8587 = v6027;
                    v8597 = v0;
                    v8604 = v6028;
                    v9907 = v10549;
                    v9908 = v10549;
                    v9909 = v15583;
                    v9910 = v15582;
                    v9911 = v10549;
                    v9912 = v15580;
                    v9913 = v10549;
                    v9914 = v15581;
                }
                v8539 = v8540;
                v8547 = v8548;
                v8555 = v8556;
                v8567 = v8568;
                v8579 = v8580;
                v8586 = v8587;
                v8596 = v8597;
                v8603 = v8604;
                v9899 = v9907;
                v9900 = v9908;
                v9901 = v9909;
                v9902 = v9910;
                v9903 = v9911;
                v9904 = v9912;
                v9905 = v9913;
                v9906 = v9914;
            } else {
                v8539 = v0;
                v8547 = v0;
                v8555 = v8564;
                v8567 = v8576;
                v8579 = v0;
                v8586 = v0;
                v8596 = v0;
                v8603 = v0;
                v9899 = v10549;
                v9900 = v10549;
                v9901 = v9450;
                v9902 = v9451;
                v9903 = v10549;
                v9904 = v11032;
                v9905 = v10549;
                v9906 = v11032;
            }
            let v6869 = if v4325 != v0 { 1.0 } else { 0.0 };
            let v8298: f64;
            let v8511: f64;
            let v10023: Lanes<6>;
            let v10024: Lanes<6>;
            if v6869 != 0.0 {
                let v6870 = v823 + v4340;
                let v16630 = (Lanes([v9395[0], v9395[1], 0.0, 0.0, 0.0, 0.0])) + v9426;
                let v6872 = v1 - v4356;
                let v6874 = (v4356 * v6870) + (v6872 * v4336);
                let v16633 = (v16630 * v4356) + (v9425 * v6872);
                let v6876 = if v6875 != v0 { 1.0 } else { 0.0 };
                if v6876 != 0.0 {
                } else {
                }
                let v6879 = if v6874 > (v6870 - v6877) { 1.0 } else { 0.0 };
                let v8299: f64;
                let v10025: Lanes<6>;
                if v6879 != 0.0 {
                    let v6881 = v6870 - v6880;
                    v8299 = v6881;
                    v10025 = v16630;
                } else {
                    v8299 = v6874;
                    v10025 = v16633;
                }
                v8298 = v8299;
                v8511 = v0;
                v10023 = v10025;
                v10024 = v11032;
            } else {
                let v6882 = if v6875 != v0 { 1.0 } else { 0.0 };
                let v8512: f64;
                let v10026: Lanes<6>;
                if v6882 != 0.0 {
                    let v6884 = if v4381 < v6883 { 1.0 } else { 0.0 };
                    let v8513: f64;
                    let v10027: Lanes<6>;
                    if v6884 != 0.0 {
                        v8513 = v0;
                        v10027 = v11032;
                    } else {
                        let v6885 = v665 / v136;
                        let v6886 = v1 / v4348;
                        let v6887 = v4381 * v6885;
                        let v16623 = (v10385 / v136) * v4381;
                        let v6888 = v6887 * v6886;
                        let v16628 = (((v9428 * v6885) + (Lanes([0.0, 0.0, v16623[0], 0.0, 0.0, 0.0]))) * v6886) + ((((v9427 * v6886) * v10360) / v4348) * v6887);
                        v8513 = v6888;
                        v10027 = v16628;
                    }
                    v8512 = v8513;
                    v10026 = v10027;
                } else {
                    v8512 = v0;
                    v10026 = v11032;
                }
                v8298 = v8300;
                v8511 = v8512;
                v10023 = v9768;
                v10024 = v10026;
            }
            let v6889 = v1 / v127;
            let v8454: f64;
            let v8458: f64;
            let v8623: f64;
            let v8629: f64;
            let v8641: f64;
            let v8652: f64;
            let v10028: Lanes<6>;
            let v10029: Lanes<6>;
            let v10030: Lanes<5>;
            let v10031: Lanes<5>;
            let v10032: Lanes<5>;
            let v10033: Lanes<5>;
            if v566 != 0.0 {
                let v6893 = if v6892 > v0 { 1.0 } else { 0.0 };
                let v6894 = if (if v6890 >= v1 { 1.0 } else { 0.0 }) != 0.0 && v6893 != 0.0 { 1.0 } else { 0.0 };
                let v8455: f64;
                let v8459: f64;
                let v8624: f64;
                let v8630: f64;
                let v8642: f64;
                let v8653: f64;
                let v10034: Lanes<6>;
                let v10035: Lanes<6>;
                let v10036: Lanes<5>;
                let v10037: Lanes<5>;
                let v10038: Lanes<5>;
                let v10039: Lanes<5>;
                if v6894 != 0.0 {
                    let v6898 = if (if v39 == v0 { 1.0 } else { 0.0 }) != 0.0 && v6893 != 0.0 { 1.0 } else { 0.0 };
                    let v7801: f64;
                    let v7820: f64;
                    let v8625: f64;
                    let v8631: f64;
                    let v8643: f64;
                    let v8654: f64;
                    let v10040: Lanes<6>;
                    let v10041: Lanes<6>;
                    let v10042: Lanes<5>;
                    let v10043: Lanes<5>;
                    let v10044: Lanes<5>;
                    let v10045: Lanes<5>;
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
                        let v17712 = v9397 * v6906;
                        let v17714 = (v9397 * v6903) * v6907;
                        let v6910 = (v830 * v6906) - (v6907 * v6905);
                        let v17718 = (Lanes([v17712[0], v17712[1], 0.0, v17712[2], 0.0, 0.0])) - (((v9426 * v10360) * v6905) + (Lanes([v17714[0], v17714[1], 0.0, v17714[2], 0.0, 0.0])));
                        let v17720 = v9397 - (Lanes([v9395[0], v9395[1], 0.0]));
                        let v6912 = v6903 * (v6904 - v823);
                        let v6914 = v777 - (v4336 - v823);
                        let v17725 = v17720 * v6906;
                        let v17726 = (v17720 * v6903) * v6914;
                        let v6918 = ((v830 - v823) * v6906) - (v6912 * v6914);
                        let v17731 = (Lanes([v17725[0], v17725[1], 0.0, v17725[2], 0.0, 0.0])) - ((Lanes([v17726[0], v17726[1], 0.0, v17726[2], 0.0, 0.0])) + (((v9425 - (Lanes([v9395[0], v9395[1], 0.0, 0.0, 0.0, 0.0]))) * v10360) * v6912));
                        v7801 = v6918;
                        v7820 = v6910;
                        v8625 = v0;
                        v8631 = v0;
                        v8643 = v0;
                        v8654 = v0;
                        v10040 = v17731;
                        v10041 = v17718;
                        v10042 = v10549;
                        v10043 = v10549;
                        v10044 = v10549;
                        v10045 = v10549;
                    } else {
                        let v6920 = (v39 / v489).sqrt();
                        let v6921 = v750 * v6920;
                        let v16640 = v10455 * v6920;
                        let v6960: f64;
                        let v6982: f64;
                        let v7344: f64;
                        let v7350: f64;
                        let v10046: Lanes<3>;
                        let v10047: Lanes<4>;
                        if v565 != 0.0 {
                            let v6927 = (v6046 * v835) + (v6048 * (v835 - v823));
                            let v16655 = (v9398 * v6046) + ((v9398 - v10530) * v6048);
                            let v16659 = (v9395 * v6046) + ((v9395 * v10360) * v6048);
                            let v16664 = (v9397 * v6046) + ((v9397 - (Lanes([v9395[0], v9395[1], 0.0]))) * v6048);
                            let v6937 = ((v6046 * v830) + (v6048 * (v830 - v823))) - v6927;
                            let v16669 = (Lanes([v16664[0], v16664[1], v16664[2], 0.0])) - (Lanes([v16655[0], v16655[1], 0.0, v16655[2]]));
                            let v6940 = v6046 + (v6923 * v6048);
                            let v6942 = v6048 + (v6923 * v6046);
                            let v16673 = ((v16655 * v10360) * v6940) + (((Lanes([v16659[0], v16659[1], 0.0])) - v16655) * v6942);
                            let v6947 = ((v6940 * (-v6927)) + (v6942 * (((v6046 * v823) + (v6048 * (-v823))) - v6927))) + v6946;
                            v6960 = v6947;
                            v6982 = v6937;
                            v7344 = v6940;
                            v7350 = v6942;
                            v10046 = v16673;
                            v10047 = v16669;
                        } else {
                            let v6949 = v6046 + (v6923 * v6048);
                            let v6951 = v6048 + (v6923 * v6046);
                            let v6984: f64;
                            let v10048: Lanes<3>;
                            if v6922 != 0.0 {
                                let v6955 = (v6046 * v830) + (v6048 * (v830 - v823));
                                let v16645 = (v9397 * v6046) + ((v9397 - (Lanes([v9395[0], v9395[1], 0.0]))) * v6048);
                                v6984 = v6955;
                                v10048 = v16645;
                            } else {
                                v6984 = v0;
                                v10048 = v10501;
                            }
                            let v6983: f64;
                            let v10049: Lanes<3>;
                            if v6923 != 0.0 {
                                let v6959 = (v6048 * v830) + (v6046 * (v830 - v823));
                                let v16650 = (v9397 * v6048) + ((v9397 - (Lanes([v9395[0], v9395[1], 0.0]))) * v6046);
                                v6983 = v6959;
                                v10049 = v16650;
                            } else {
                                v6983 = v6984;
                                v10049 = v10048;
                            }
                            let v16651 = Lanes([v10049[0], v10049[1], v10049[2], 0.0]);
                            v6960 = v0;
                            v6982 = v6983;
                            v7344 = v6949;
                            v7350 = v6951;
                            v10046 = v10474;
                            v10047 = v16651;
                        }
                        let v6961 = -v6960;
                        let v16674 = v10046 * v10360;
                        let v6962 = if v6961 > v783 { 1.0 } else { 0.0 };
                        let v6977: f64;
                        let v10050: Lanes<3>;
                        if v6962 != 0.0 {
                            let v6964 = v779 - v783;
                            let v6965 = (v6961 - v783) / v6964;
                            let v16675 = v16674 / v6964;
                            let v6966 = v6965 * v6965;
                            let v16676 = v16675 * v6965;
                            let v16677 = v16676 + v16676;
                            let v16681 = v16677 * v6966;
                            let v6972 = (((v1 + v6965) + v6966) + (v6966 * v6965)) + (v6966 * v6966);
                            let v6973 = v1 / v6972;
                            let v16690 = (((((((v16675 + v16677) + ((v16677 * v6965) + (v16675 * v6966))) + (v16681 + v16681)) * v6973) * v10360) / v6972) * v10360) * v6964;
                            let v6976 = v783 + (v6964 * (v1 - v6973));
                            v6977 = v6976;
                            v10050 = v16690;
                        } else {
                            v6977 = v6961;
                            v10050 = v16674;
                        }
                        let v16691 = v10050 * v10360;
                        let v6979 = (-v6977) - v11;
                        let v6980 = v6921 * v6889;
                        let v16692 = v16640 * v6889;
                        let v6981 = v6980 * v6980;
                        let v16693 = v16692 * v6980;
                        let v16694 = v16693 + v16693;
                        let v16695 = v10047 * v10360;
                        let v6986 = (-v6982) + v66;
                        let v6987 = v39 / v731;
                        let v6988 = v78 / v663;
                        let v6989 = v6987.ln();
                        let v6990 = v6988 * v6989;
                        let v16706 = ((((v10380 * v6988) * v10360) / v663) * v6989) + (((((v10423 * v6987) * v10360) / v731) * (v9353 / v6987)) * v6988);
                        let v6991 = -v6979;
                        let v16707 = v16691 * v10360;
                        let v6992 = if v6986 < v6991 { 1.0 } else { 0.0 };
                        let v7337: f64;
                        let v7339: f64;
                        let v7749: f64;
                        let v10051: Lanes<5>;
                        let v10052: Lanes<5>;
                        let v10053: Lanes<5>;
                        if v6992 != 0.0 {
                            let v6993 = v663 * v6921;
                            let v6994 = v1 / v6993;
                            let v6995 = v6994 * v127;
                            let v17100 = (((((v10380 * v6921) + (v16640 * v663)) * v6994) * v10360) / v6993) * v127;
                            let v17101 = v17100 * v6996;
                            let v6998 = v78 + (v6996 * v6995);
                            let v6999 = v91 * v6998;
                            let v7000 = v6999 * v6998;
                            let v7001 = v7000 * v6998;
                            let v17108 = ((((v17101 * v91) * v6998) + (v17101 * v6999)) * v6998) + (v17101 * v7000);
                            let v7002 = v661 - v6990;
                            let v17109 = v10376 - v16706;
                            let v7003 = v6986 + v6979;
                            let v17112 = v10380 * v7003;
                            let v17113 = (v16695 + (Lanes([v16691[0], v16691[1], 0.0, v16691[2]]))) * v663;
                            let v7006 = v3500 * v6995;
                            let v7007 = (v663 * v7003) - v78;
                            let v7008 = v7006 * v7007;
                            let v17118 = (v17100 * v3500) * v7007;
                            let v17121 = (Lanes([0.0, 0.0, v17118[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v17112[0], 0.0, 0.0])) + (Lanes([v17113[0], v17113[1], 0.0, v17113[2], v17113[3]]))) * v7006);
                            let v7009 = v7005 - v7008;
                            let v17122 = v17121 * v10360;
                            let v7010 = v7009 * v7009;
                            let v17123 = v17122 * v7009;
                            let v17124 = v17123 + v17123;
                            let v7012 = if v7001 < (v7010 * v3506) { 1.0 } else { 0.0 };
                            let v7024: f64;
                            let v10054: Lanes<5>;
                            if v7012 != 0.0 {
                                let v17131 = v17108 * v13;
                                let v7016 = (v13 * v7001) / v7009;
                                let v7018 = ((v7013 + v7009) + v7016) + v7008;
                                let v17137 = (v17122 + (((Lanes([0.0, 0.0, v17131[0], 0.0, 0.0])) - (v17122 * v7016)) / v7009)) + v17121;
                                v7024 = v7018;
                                v10054 = v17137;
                            } else {
                                let v7020 = (v7001 + v7010).sqrt();
                                let v7023 = (v7021 + v7020) + v7008;
                                let v17130 = (((Lanes([0.0, 0.0, v17108[0], 0.0, 0.0])) + v17124) * (v9353 / (v10405 * v7020))) + v17121;
                                v7024 = v7023;
                                v10054 = v17130;
                            }
                            let v7025 = v7024.powf(v1562);
                            let v17141 = v10054 * (v1562 * (v7024.powf(v17138)));
                            let v17143 = (v17100 * v3523) * v10360;
                            let v7031 = v748 * v7025;
                            let v7034 = (((v7026 - (v3523 * v6995)) + (v78 * v7025)) + (v7031 * v7025)) / v7025;
                            let v17156 = v10385 * v7034;
                            let v17159 = Lanes([v16691[0], v16691[1], 0.0, 0.0, v16691[2]]);
                            let v7037 = ((v7034 * v665) - v6979) + v6979;
                            let v17161 = ((((((((Lanes([0.0, 0.0, v17143[0], 0.0, 0.0])) + (v17141 * v78)) + (((v17141 * v748) * v7025) + (v17141 * v7031))) - (v17141 * v7034)) / v7025) * v665) + (Lanes([0.0, 0.0, v17156[0], 0.0, 0.0]))) - v17159) + v17159;
                            let v7038 = v7037 / v7002;
                            let v17162 = v17109 * v7038;
                            let v17166 = ((v17161 - (Lanes([0.0, 0.0, v17162[0], 0.0, 0.0]))) / v7002) * v7038;
                            let v7041 = (v1 + (v7038 * v7038)).sqrt();
                            let v7042 = v7037 / v7041;
                            let v7045 = v127 * (v6986 - (v7042 - v6979));
                            let v17177 = ((Lanes([v16695[0], v16695[1], 0.0, v16695[2], v16695[3]])) - (((v17161 - (((v17166 + v17166) * (v9353 / (v10405 * v7041))) * v7042)) / v7041) - v17159)) * v127;
                            v7337 = v7045;
                            v7339 = v7045;
                            v7749 = v0;
                            v10051 = v17177;
                            v10052 = v17177;
                            v10053 = v10549;
                        } else {
                            let v7047 = v6986 + v6979;
                            let v16709 = v16695 + (Lanes([v16691[0], v16691[1], 0.0, v16691[2]]));
                            let v16710 = v10380 * v7047;
                            let v16711 = v16709 * v663;
                            let v16713 = Lanes([v16711[0], v16711[1], 0.0, v16711[2], v16711[3]]);
                            let v16714 = (Lanes([0.0, 0.0, v16710[0], 0.0, 0.0])) + v16713;
                            let v7049 = (v663 * v7047) - v1;
                            let v7052 = v6981 * v664;
                            let v16718 = (v16694 * v664) + (v10382 * v6981);
                            let v7053 = (v90 * (v7049 + v7046)) / v7052;
                            let v16719 = v16718 * v7053;
                            let v16722 = ((v16714 * v90) - (Lanes([0.0, 0.0, v16719[0], 0.0, 0.0]))) / v7052;
                            let v7054 = v1 + v7053;
                            let v7056 = if v7054 < v7055 { 1.0 } else { 0.0 };
                            let v7060: f64;
                            let v10055: Lanes<5>;
                            if v7056 != 0.0 {
                                v7060 = v7057;
                                v10055 = v10549;
                            } else {
                                v7060 = v7054;
                                v10055 = v16722;
                            }
                            let v7059 = (v6981 * v663) / v78;
                            let v16726 = ((v16694 * v663) + (v10380 * v6981)) / v78;
                            let v7061 = v7060.sqrt();
                            let v7062 = v1 - v7061;
                            let v16731 = v16726 * v7062;
                            let v16735 = Lanes([v16695[0], v16695[1], 0.0, v16695[2], v16695[3]]);
                            let v7065 = (v6986 + (v7059 * v7062)) + v6979;
                            let v16737 = Lanes([v16691[0], v16691[1], 0.0, 0.0, v16691[2]]);
                            let v16739 = v10380 * v7065;
                            let v7068 = (-(v663 * v7065)).exp();
                            let v7071 = (v90 * (v7049 + v7068)) / v7052;
                            let v16747 = v16718 * v7071;
                            let v16750 = (((v16714 + ((((Lanes([0.0, 0.0, v16739[0], 0.0, 0.0])) + (((v16735 + ((Lanes([0.0, 0.0, v16731[0], 0.0, 0.0])) + (((v10055 * (v9353 / (v10405 * v7061))) * v10360) * v7059))) + v16737) * v663)) * v10360) * v7068)) * v90) - (Lanes([0.0, 0.0, v16747[0], 0.0, 0.0]))) / v7052;
                            let v7072 = v1 + v7071;
                            let v7074 = if v7072 < v7073 { 1.0 } else { 0.0 };
                            let v7076: f64;
                            let v10056: Lanes<5>;
                            if v7074 != 0.0 {
                                v7076 = v7075;
                                v10056 = v10549;
                            } else {
                                v7076 = v7072;
                                v10056 = v16750;
                            }
                            let v7077 = v7076.sqrt();
                            let v7078 = v1 - v7077;
                            let v16755 = v16726 * v7078;
                            let v7081 = (v6986 + (v7059 * v7078)) + v6979;
                            let v7082 = v663 * v7081;
                            let v16761 = v10380 * v7081;
                            let v16764 = (Lanes([0.0, 0.0, v16761[0], 0.0, 0.0])) + (((v16735 + ((Lanes([0.0, 0.0, v16755[0], 0.0, 0.0])) + (((v10056 * (v9353 / (v10405 * v7077))) * v10360) * v7059))) + v16737) * v663);
                            let v7083 = if v7082 < v96 { 1.0 } else { 0.0 };
                            let v7162: f64;
                            let v10057: Lanes<5>;
                            if v7083 != 0.0 {
                                let v7086 = v663 * v6980;
                                let v7087 = v1 / v7086;
                                let v16770 = ((((v10380 * v6980) + (v16692 * v663)) * v7087) * v10360) / v7086;
                                let v7088 = v7085 + v7087;
                                let v16771 = v16709 * v10360;
                                let v7090 = (-v7047) / v6980;
                                let v16772 = v16692 * v7090;
                                let v16779 = ((v16770 * v7084) / v7093) * v10360;
                                let v7098 = (v7091 - ((v7084 * v7088) / v7093)) + (v7090 / v7096);
                                let v16782 = (Lanes([0.0, 0.0, v16779[0], 0.0, 0.0])) + ((((Lanes([v16771[0], v16771[1], 0.0, v16771[2], v16771[3]])) - (Lanes([0.0, 0.0, v16772[0], 0.0, 0.0]))) / v6980) / v7096);
                                let v7104 = ((v7099 * v7088) - v7101) / v7103;
                                let v16784 = (v16770 * v7099) / v7103;
                                let v16785 = v16782 * v7098;
                                let v7106 = v7104 * v7104;
                                let v16787 = v16784 * v7104;
                                let v16791 = ((v16787 + v16787) * v7104) + (v16784 * v7106);
                                let v7109 = ((v7098 * v7098) + (v7106 * v7104)).sqrt();
                                let v16796 = ((v16785 + v16785) + (Lanes([0.0, 0.0, v16791[0], 0.0, 0.0]))) * (v9353 / (v10405 * v7109));
                                let v7111 = (-v7098) + v7109;
                                let v7113 = v7098 + v7109;
                                let v7118 = ((v7111.powf(v1562)) + (-(v7113.powf(v1562)))) - v7117;
                                let v16811 = v10385 * v7118;
                                let v7121 = ((v7118 * v665) - v6979) + v6979;
                                let v7122 = v663 * v7121;
                                let v16816 = v10380 * v7121;
                                let v16819 = (Lanes([0.0, 0.0, v16816[0], 0.0, 0.0])) + (((((((((v16782 * v10360) + v16796) * (v1562 * (v7111.powf(v16799)))) + (((v16782 + v16796) * (v1562 * (v7113.powf(v16804)))) * v10360)) * v665) + (Lanes([0.0, 0.0, v16811[0], 0.0, 0.0]))) - v16737) + v16737) * v663);
                                v7162 = v7122;
                                v10057 = v16819;
                            } else {
                                v7162 = v7082;
                                v10057 = v16764;
                            }
                            let v7124 = if v7123 > v0 { 1.0 } else { 0.0 };
                            let v7178: f64;
                            let v10058: Lanes<5>;
                            if v7124 != 0.0 {
                                let v7125 = v7047 + v79;
                                let v16820 = v10380 * v6991;
                                let v16821 = v16707 * v663;
                                let v7127 = (v663 * v6991).exp();
                                let v7128 = v7127 + v362;
                                let v7129 = v731 / v39;
                                let v7130 = v7129 * v7129;
                                let v16827 = (v10423 / v39) * v7129;
                                let v16828 = v16827 + v16827;
                                let v7131 = v7130 * v7128;
                                let v16829 = v16828 * v7128;
                                let v7132 = v663 * v7125;
                                let v16833 = v10380 * v7125;
                                let v16835 = (Lanes([0.0, 0.0, v16833[0], 0.0, 0.0])) + v16713;
                                let v7133 = v7131 * v7052;
                                let v16837 = v16718 * v7131;
                                let v16839 = (((Lanes([0.0, 0.0, v16829[0], 0.0])) + ((((Lanes([0.0, 0.0, v16820[0], 0.0])) + (Lanes([v16821[0], v16821[1], 0.0, v16821[2]]))) * v7127) * v7130)) * v7052) + (Lanes([0.0, 0.0, v16837[0], 0.0]));
                                let v16840 = v16835 * v7132;
                                let v7135 = v7133 + (v7132 * v7132);
                                let v16842 = Lanes([v16839[0], v16839[1], v16839[2], 0.0, v16839[3]]);
                                let v7137 = v7130 * v7052;
                                let v7138 = v7137.ln();
                                let v16850 = ((v16828 * v7052) + (v16718 * v7130)) * (v9353 / v7137);
                                let v16851 = Lanes([0.0, 0.0, v16850[0], 0.0, 0.0]);
                                let v7140 = v663 * v6979;
                                let v16853 = v10380 * v6979;
                                let v16854 = v16691 * v663;
                                let v16857 = (Lanes([0.0, 0.0, v16853[0], 0.0])) + (Lanes([v16854[0], v16854[1], 0.0, v16854[2]]));
                                let v16858 = Lanes([v16857[0], v16857[1], v16857[2], 0.0, v16857[3]]);
                                let v16860 = v16835 - ((((v16842 + (v16840 + v16840)) * (v9353 / v7135)) - v16851) + v16858);
                                let v7143 = (v7132 - (((v7135.ln()) - v7138) + v7140)) - v1;
                                let v7144 = v90 * v7132;
                                let v16861 = v16835 * v90;
                                let v7145 = if v7144 > v0 { 1.0 } else { 0.0 };
                                let v7147: f64;
                                let v10059: Lanes<5>;
                                if v7145 != 0.0 {
                                    v7147 = v7144;
                                    v10059 = v16861;
                                } else {
                                    let v7146 = -v7144;
                                    let v16862 = v16861 * v10360;
                                    v7147 = v7146;
                                    v10059 = v16862;
                                }
                                let v16863 = v16860 * v7143;
                                let v7150 = ((v7143 * v7143) + v7147).sqrt();
                                let v16873 = v10380 * v79;
                                let v7156 = (v7132 - (v7132 - (v13 * (v7143 + v7150)))) + (v663 * v79);
                                let v16876 = ((v16835 - (v16835 - ((v16860 + (((v16863 + v16863) + v10059) * (v9353 / (v10405 * v7150)))) * v13))) + (Lanes([0.0, 0.0, v16873[0], 0.0, 0.0]))) * v7156;
                                let v7158 = v7133 + (v7156 * v7156);
                                let v7161 = ((v7158.ln()) - v7138) + v7140;
                                let v16882 = (((v16842 + (v16876 + v16876)) * (v9353 / v7158)) - v16851) + v16858;
                                let v16883 = v16882 - v10057;
                                let v7165 = (v7161 - v7162) - v7164;
                                let v7168 = (v90 * v7161) * v7167;
                                let v16885 = (v16882 * v90) * v7167;
                                let v7169 = if v7168 > v0 { 1.0 } else { 0.0 };
                                let v7171: f64;
                                let v10060: Lanes<5>;
                                if v7169 != 0.0 {
                                    v7171 = v7168;
                                    v10060 = v16885;
                                } else {
                                    let v7170 = -v7168;
                                    let v16886 = v16885 * v10360;
                                    v7171 = v7170;
                                    v10060 = v16886;
                                }
                                let v16887 = v16883 * v7165;
                                let v7174 = ((v7165 * v7165) + v7171).sqrt();
                                let v7177 = v7161 - (v13 * (v7165 + v7174));
                                let v16895 = v16882 - ((v16883 + (((v16887 + v16887) + v10060) * (v9353 / (v10405 * v7174)))) * v13);
                                v7178 = v7177;
                                v10058 = v16895;
                            } else {
                                v7178 = v7162;
                                v10058 = v10057;
                            }
                            let v7179 = v7178 / v663;
                            let v16896 = v10380 * v7179;
                            let v7180 = v7179 - v6979;
                            let v16900 = ((v10058 - (Lanes([0.0, 0.0, v16896[0], 0.0, 0.0]))) / v663) - v16737;
                            let v7183 = (-v7178).exp();
                            let v7184 = (v7178 - v1) + v7183;
                            let v16903 = v10058 + ((v10058 * v10360) * v7183);
                            let v7186 = if v7184 < v7185 { 1.0 } else { 0.0 };
                            let v7188: f64;
                            let v10061: Lanes<5>;
                            if v7186 != 0.0 {
                                v7188 = v7187;
                                v10061 = v10549;
                            } else {
                                v7188 = v7184;
                                v10061 = v16903;
                            }
                            let v7189 = v7188.sqrt();
                            let v7190 = v6921 * v7189;
                            let v16907 = v16640 * v7189;
                            let v16910 = (Lanes([0.0, 0.0, v16907[0], 0.0, 0.0])) + ((v10061 * (v9353 / (v10405 * v7189))) * v6921);
                            let v7192 = v127 * (v6986 - v7180);
                            let v16912 = (v16735 - v16900) * v127;
                            let v7193 = if v7123 == v1 { 1.0 } else { 0.0 };
                            let v7338: f64;
                            let v7340: f64;
                            let v7750: f64;
                            let v10062: Lanes<5>;
                            let v10063: Lanes<5>;
                            let v10064: Lanes<5>;
                            if v7193 != 0.0 {
                                let v16913 = v10380 * v6991;
                                let v16914 = v16707 * v663;
                                let v7195 = (v663 * v6991).exp();
                                let v16918 = ((Lanes([0.0, 0.0, v16913[0], 0.0])) + (Lanes([v16914[0], v16914[1], 0.0, v16914[2]]))) * v7195;
                                let v7196 = v731 / v39;
                                let v7197 = v7196 * v7196;
                                let v16920 = (v10423 / v39) * v7196;
                                let v16921 = v16920 + v16920;
                                let v7198 = v7197 * v7195;
                                let v16922 = v16921 * v7195;
                                let v16925 = (Lanes([0.0, 0.0, v16922[0], 0.0])) + (v16918 * v7197);
                                let mut v7199: f64 = 0.0;
                                let mut v7202: f64 = 0.0;
                                let mut v7288: f64 = 0.0;
                                let mut v7318: f64 = 0.0;
                                let mut v7321: f64 = 0.0;
                                let mut v7329: f64 = 0.0;
                                let mut v7332: f64 = 0.0;
                                let mut v10065: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10066: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10067: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10068: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10069: Lanes<5> = Lanes([0.0; 5]);
                                v7199 = v1;
                                v7202 = v7180;
                                v7288 = v0;
                                v7318 = v7178;
                                v7321 = v0;
                                v7329 = v0;
                                v7332 = v0;
                                v10065 = v16900;
                                v10066 = v10058;
                                v10067 = v10549;
                                v10068 = v10549;
                                v10069 = v10549;
                                loop {
                                    let v7201 = if v7199 <= v7200 { 1.0 } else { 0.0 };
                                    if v7201 == 0.0 {
                                        break;
                                    }
                                    let v7203 = v7202 + v6979;
                                    let v7204 = v663 * v7203;
                                    let v16946 = v10380 * v7203;
                                    let v16949 = (Lanes([0.0, 0.0, v16946[0], 0.0, 0.0])) + ((v10065 + v16737) * v663);
                                    let v7205 = if v7204 < v644 { 1.0 } else { 0.0 };
                                    let v7281: f64;
                                    let v7285: f64;
                                    let v7322: f64;
                                    let v7333: f64;
                                    let v10070: Lanes<5>;
                                    let v10071: Lanes<5>;
                                    let v10072: Lanes<5>;
                                    let v10073: Lanes<5>;
                                    if v7205 != 0.0 {
                                        let v7206 = v7204 * v7204;
                                        let v16991 = v16949 * v7204;
                                        let v16992 = v16991 + v16991;
                                        let v7207 = v7206 * v7204;
                                        let v7210 = v7208 + (v7204 * v6318);
                                        let v7212 = v6316 + (v7204 * v7210);
                                        let v7213 = v7207 * v7212;
                                        let v17002 = (((v16992 * v7204) + (v16949 * v7206)) * v7212) + (((v16949 * v7210) + ((v16949 * v6318) * v7204)) * v7207);
                                        let v7216 = v7204 * v644;
                                        let v17003 = v16949 * v644;
                                        let v7218 = v7215 + (v7216 * v6318);
                                        let v7220 = v7214 + (v7204 * v7218);
                                        let v7221 = v7206 * v7220;
                                        let v7222 = v7198 * v7213;
                                        let v17011 = v16925 * v7213;
                                        let v7223 = v7222 * v7213;
                                        let v17017 = (((Lanes([v17011[0], v17011[1], v17011[2], 0.0, v17011[3]])) + (v17002 * v7198)) * v7213) + (v17002 * v7222);
                                        let v17019 = v10380 * v7198;
                                        let v7225 = (v7198 * v663) * v78;
                                        let v7226 = v7225 * v7213;
                                        let v17023 = (((v16925 * v663) + (Lanes([0.0, 0.0, v17019[0], 0.0]))) * v78) * v7213;
                                        let v7231 = v7229 + (v7204 * v6342);
                                        let v7233 = v6340 + (v7204 * v7231);
                                        let v7235 = v7228 + (v7204 * v7233);
                                        let v7237 = v6338 + (v7204 * v7235);
                                        let v7238 = v7204 * v7237;
                                        let v17042 = (v16949 * v7237) + (((v16949 * v7235) + (((v16949 * v7233) + (((v16949 * v7231) + ((v16949 * v6342) * v7204)) * v7204)) * v7204)) * v7204);
                                        let v7243 = v7241 + (v7216 * v6342);
                                        let v7245 = v7240 + (v7204 * v7243);
                                        let v7247 = v7239 + (v7204 * v7245);
                                        let v7249 = v6338 + (v7204 * v7247);
                                        let v17053 = v17042 * v7238;
                                        let v7253 = (((v7238 * v7238) + v7223) + v362).sqrt();
                                        let v17058 = ((v17053 + v17053) + v17017) * (v9353 / (v10405 * v7253));
                                        let v17059 = v10380 * v7249;
                                        let v7255 = (v663 * v7249) * v78;
                                        let v7258 = v7253 + v7253;
                                        let v7259 = ((v7255 * v7238) + (v7226 * v7221)) / v7258;
                                        let v17071 = (((((((Lanes([0.0, 0.0, v17059[0], 0.0, 0.0])) + (((v16949 * v7247) + (((v16949 * v7245) + (((v16949 * v7243) + ((v17003 * v6342) * v7204)) * v7204)) * v7204)) * v663)) * v78) * v7238) + (v17042 * v7255)) + ((((Lanes([v17023[0], v17023[1], v17023[2], 0.0, v17023[3]])) + (v17002 * v7225)) * v7221) + (((v16992 * v7220) + (((v16949 * v7218) + ((v17003 * v6318) * v7204)) * v7206)) * v7226))) - ((v17058 + v17058) * v7259)) / v7258;
                                        v7281 = v7253;
                                        v7285 = v7259;
                                        v7322 = v7238;
                                        v7333 = v7223;
                                        v10070 = v17058;
                                        v10071 = v17071;
                                        v10072 = v17042;
                                        v10073 = v17017;
                                    } else {
                                        let v7260 = if v7204 < v2535 { 1.0 } else { 0.0 };
                                        let v7273: f64;
                                        let v7276: f64;
                                        let v10074: Lanes<5>;
                                        let v10075: Lanes<5>;
                                        if v7260 != 0.0 {
                                            let v7261 = v7204.exp();
                                            let v16968 = v16949 * v7261;
                                            let v7262 = v7261 - v1;
                                            let v7263 = v7198 * v7262;
                                            let v16969 = v16925 * v7262;
                                            let v16972 = (Lanes([v16969[0], v16969[1], v16969[2], 0.0, v16969[3]])) + (v16968 * v7198);
                                            let v7264 = v7198 * v663;
                                            let v16974 = v10380 * v7198;
                                            let v7265 = v7264 * v7261;
                                            let v16977 = ((v16925 * v663) + (Lanes([0.0, 0.0, v16974[0], 0.0]))) * v7261;
                                            let v16980 = (Lanes([v16977[0], v16977[1], v16977[2], 0.0, v16977[3]])) + (v16968 * v7264);
                                            v7273 = v7263;
                                            v7276 = v7265;
                                            v10074 = v16972;
                                            v10075 = v16980;
                                        } else {
                                            let v16950 = v10380 * v7202;
                                            let v7267 = (v663 * v7202).exp();
                                            let v16954 = ((Lanes([0.0, 0.0, v16950[0], 0.0, 0.0])) + (v10065 * v663)) * v7267;
                                            let v7268 = v7267 - v7195;
                                            let v7269 = v7197 * v7268;
                                            let v16957 = v16921 * v7268;
                                            let v16960 = (Lanes([0.0, 0.0, v16957[0], 0.0, 0.0])) + ((v16954 - (Lanes([v16918[0], v16918[1], v16918[2], 0.0, v16918[3]]))) * v7197);
                                            let v7270 = v7197 * v663;
                                            let v7271 = v7270 * v7267;
                                            let v16964 = ((v16921 * v663) + (v10380 * v7197)) * v7267;
                                            let v16967 = (Lanes([0.0, 0.0, v16964[0], 0.0, 0.0])) + (v16954 * v7270);
                                            v7273 = v7269;
                                            v7276 = v7271;
                                            v10074 = v16960;
                                            v10075 = v16967;
                                        }
                                        let v7275 = ((v7204 - v1) + v7273).sqrt();
                                        let v16984 = (v16949 + v10074) * (v9353 / (v10405 * v7275));
                                        let v7278 = (v663 + v7276) / v7275;
                                        let v7279 = v7278 * v13;
                                        let v16990 = ((((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + v10075) - (v16984 * v7278)) / v7275) * v13;
                                        v7281 = v7275;
                                        v7285 = v7279;
                                        v7322 = v0;
                                        v7333 = v7273;
                                        v10070 = v16984;
                                        v10071 = v16990;
                                        v10072 = v10549;
                                        v10073 = v10074;
                                    }
                                    let v17073 = v16692 * v7281;
                                    let v7283 = (v6986 - v7202) - (v6980 * v7281);
                                    let v17077 = (v16735 - v10065) - ((Lanes([0.0, 0.0, v17073[0], 0.0, 0.0])) + (v10070 * v6980));
                                    let v17078 = v16692 * v7285;
                                    let v7287 = v7284 - (v6980 * v7285);
                                    let v17082 = ((Lanes([0.0, 0.0, v17078[0], 0.0, 0.0])) + (v10071 * v6980)) * v10360;
                                    let v7289 = if v7288 == v1 { 1.0 } else { 0.0 };
                                    let v7312: f64;
                                    let v7314: f64;
                                    let v7315: f64;
                                    let v10076: Lanes<5>;
                                    if v7289 != 0.0 {
                                        v7312 = v7290;
                                        v7314 = v7202;
                                        v7315 = v7288;
                                        v10076 = v10065;
                                    } else {
                                        let v7292 = (-v7283) / v7287;
                                        let v17086 = ((v17077 * v10360) - (v17082 * v7292)) / v7287;
                                        let v7294 = v7202.abs();
                                        let v17090 = v10065 * ((v10405 * (if v7202 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                                        let v7295 = if v1 >= v7294 { 1.0 } else { 0.0 };
                                        let v7296: f64;
                                        let v10077: Lanes<5>;
                                        if v7295 != 0.0 {
                                            v7296 = v1;
                                            v10077 = v10549;
                                        } else {
                                            v7296 = v7294;
                                            v10077 = v17090;
                                        }
                                        let v7298 = v7293 * (v1 + v7296);
                                        let v17091 = v10077 * v7293;
                                        let v7300 = if (v7292.abs()) > v7298 { 1.0 } else { 0.0 };
                                        let v7305: f64;
                                        let v10078: Lanes<5>;
                                        if v7300 != 0.0 {
                                            let v7301 = if v7292 >= v0 { 1.0 } else { 0.0 };
                                            let v7303: f64;
                                            if v7301 != 0.0 {
                                                v7303 = v1;
                                            } else {
                                                v7303 = v7302;
                                            }
                                            let v7304 = v7298 * v7303;
                                            let v17092 = v17091 * v7303;
                                            v7305 = v7304;
                                            v10078 = v17092;
                                        } else {
                                            v7305 = v7292;
                                            v10078 = v17086;
                                        }
                                        let v7306 = v7202 + v7305;
                                        let v17093 = v10065 + v10078;
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
                                        v10076 = v17093;
                                    }
                                    let v7313 = v7312 + v1;
                                    v7199 = v7313;
                                    v7202 = v7314;
                                    v7288 = v7315;
                                    v7318 = v7204;
                                    v7321 = v7322;
                                    v7329 = v7281;
                                    v7332 = v7333;
                                    v10065 = v10076;
                                    v10066 = v16949;
                                    v10067 = v10072;
                                    v10068 = v10070;
                                    v10069 = v10073;
                                }
                                let v7317 = if v7288 == v0 { 1.0 } else { 0.0 };
                                if v7317 != 0.0 {
                                } else {
                                }
                                let v7319 = if v7318 < v644 { 1.0 } else { 0.0 };
                                let v7327: f64;
                                let v10079: Lanes<5>;
                                if v7319 != 0.0 {
                                    let v7320 = if v7318 < v96 { 1.0 } else { 0.0 };
                                    if v7320 != 0.0 {
                                    } else {
                                    }
                                    let v7324 = v7321 + v7323;
                                    v7327 = v7324;
                                    v10079 = v10067;
                                } else {
                                    let v7326 = (v7318 - v1).sqrt();
                                    let v16928 = v10066 * (v9353 / (v10405 * v7326));
                                    v7327 = v7326;
                                    v10079 = v16928;
                                }
                                let v7328 = v6921 * v7327;
                                let v16929 = v16640 * v7327;
                                let v16932 = (Lanes([0.0, 0.0, v16929[0], 0.0, 0.0])) + (v10079 * v6921);
                                let v7330 = v7329 + v7327;
                                let v7331 = v1 / v7330;
                                let v7334 = v6921 * v7332;
                                let v16937 = v16640 * v7332;
                                let v7336 = v7328 + (v7334 * v7331);
                                let v16944 = v16932 + ((((Lanes([0.0, 0.0, v16937[0], 0.0, 0.0])) + (v10069 * v6921)) * v7331) + (((((v10068 + v10079) * v7331) * v10360) / v7330) * v7334));
                                v7338 = v7336;
                                v7340 = v7328;
                                v7750 = v7321;
                                v10062 = v16944;
                                v10063 = v16932;
                                v10064 = v10067;
                            } else {
                                v7338 = v7192;
                                v7340 = v7190;
                                v7750 = v0;
                                v10062 = v16912;
                                v10063 = v16910;
                                v10064 = v10549;
                            }
                            v7337 = v7338;
                            v7339 = v7340;
                            v7749 = v7750;
                            v10051 = v10062;
                            v10052 = v10063;
                            v10053 = v10064;
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
                        let v10080: Lanes<5>;
                        let v10081: Lanes<5>;
                        if v7347 != 0.0 {
                            let v7348 = v7343 * v7337;
                            let v17178 = v10051 * v7343;
                            let v7349 = v7343 * v7339;
                            let v17179 = v10052 * v7343;
                            v8627 = v7348;
                            v8656 = v7349;
                            v10080 = v17178;
                            v10081 = v17179;
                        } else {
                            v8627 = v0;
                            v8656 = v0;
                            v10080 = v10549;
                            v10081 = v10549;
                        }
                        let v7353 = if (if v7350 != 0.0 && v9 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v6923 != 0.0 && v565 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v8633: f64;
                        let v8645: f64;
                        let v10082: Lanes<5>;
                        let v10083: Lanes<5>;
                        if v7353 != 0.0 {
                            let v7354 = v7343 * v7337;
                            let v17180 = v10051 * v7343;
                            let v7355 = v7343 * v7339;
                            let v17181 = v10052 * v7343;
                            v8633 = v7354;
                            v8645 = v7355;
                            v10082 = v17180;
                            v10083 = v17181;
                        } else {
                            v8633 = v0;
                            v8645 = v0;
                            v10082 = v10549;
                            v10083 = v10549;
                        }
                        let v7394: f64;
                        let v7414: f64;
                        let v7773: f64;
                        let v7779: f64;
                        let v10084: Lanes<3>;
                        let v10085: Lanes<4>;
                        if v565 != 0.0 {
                            let v7361 = (v6046 * v835) + (v6048 * (v835 - v823));
                            let v17197 = (v9398 * v6046) + ((v9398 - v10530) * v6048);
                            let v17201 = (v9395 * v6046) + ((v9395 * v10360) * v6048);
                            let v17206 = (v9397 * v6046) + ((v9397 - (Lanes([v9395[0], v9395[1], 0.0]))) * v6048);
                            let v7371 = ((v6046 * v830) + (v6048 * (v830 - v823))) - v7361;
                            let v17211 = (Lanes([v17206[0], v17206[1], v17206[2], 0.0])) - (Lanes([v17197[0], v17197[1], 0.0, v17197[2]]));
                            let v7374 = (v7356 * v6046) + v6048;
                            let v7376 = (v7356 * v6048) + v6046;
                            let v17215 = ((v17197 * v10360) * v7374) + (((Lanes([v17201[0], v17201[1], 0.0])) - v17197) * v7376);
                            let v7381 = ((v7374 * (-v7361)) + (v7376 * (((v6046 * v823) + (v6048 * (-v823))) - v7361))) + v7380;
                            v7394 = v7381;
                            v7414 = v7371;
                            v7773 = v7374;
                            v7779 = v7376;
                            v10084 = v17215;
                            v10085 = v17211;
                        } else {
                            let v7383 = (v7356 * v6046) + v6048;
                            let v7385 = (v7356 * v6048) + v6046;
                            let v7416: f64;
                            let v10086: Lanes<4>;
                            if v7356 != 0.0 {
                                let v7389 = (v6046 * v830) + (v6048 * (v830 - v823));
                                let v17186 = (v9397 * v6046) + ((v9397 - (Lanes([v9395[0], v9395[1], 0.0]))) * v6048);
                                let v17187 = Lanes([v17186[0], v17186[1], v17186[2], 0.0]);
                                v7416 = v7389;
                                v10086 = v17187;
                            } else {
                                v7416 = v6982;
                                v10086 = v10047;
                            }
                            let v7415: f64;
                            let v10087: Lanes<4>;
                            if v7357 != 0.0 {
                                let v7393 = (v6048 * v830) + (v6046 * (v830 - v823));
                                let v17192 = (v9397 * v6048) + ((v9397 - (Lanes([v9395[0], v9395[1], 0.0]))) * v6046);
                                let v17193 = Lanes([v17192[0], v17192[1], v17192[2], 0.0]);
                                v7415 = v7393;
                                v10087 = v17193;
                            } else {
                                v7415 = v7416;
                                v10087 = v10086;
                            }
                            v7394 = v0;
                            v7414 = v7415;
                            v7773 = v7383;
                            v7779 = v7385;
                            v10084 = v10474;
                            v10085 = v10087;
                        }
                        let v7395 = -v7394;
                        let v17216 = v10084 * v10360;
                        let v7396 = if v7395 > v783 { 1.0 } else { 0.0 };
                        let v7411: f64;
                        let v10088: Lanes<3>;
                        if v7396 != 0.0 {
                            let v7398 = v779 - v783;
                            let v7399 = (v7395 - v783) / v7398;
                            let v17217 = v17216 / v7398;
                            let v7400 = v7399 * v7399;
                            let v17218 = v17217 * v7399;
                            let v17219 = v17218 + v17218;
                            let v17223 = v17219 * v7400;
                            let v7406 = (((v1 + v7399) + v7400) + (v7400 * v7399)) + (v7400 * v7400);
                            let v7407 = v1 / v7406;
                            let v17232 = (((((((v17217 + v17219) + ((v17219 * v7399) + (v17217 * v7400))) + (v17223 + v17223)) * v7407) * v10360) / v7406) * v10360) * v7398;
                            let v7410 = v783 + (v7398 * (v1 - v7407));
                            v7411 = v7410;
                            v10088 = v17232;
                        } else {
                            v7411 = v7395;
                            v10088 = v17216;
                        }
                        let v17233 = v10088 * v10360;
                        let v7413 = (-v7411) - v11;
                        let v17234 = v10085 * v10360;
                        let v7418 = (-v7414) + v66;
                        let v7419 = -v7413;
                        let v17235 = v17233 * v10360;
                        let v7420 = if v7418 < v7419 { 1.0 } else { 0.0 };
                        let v7766: f64;
                        let v7768: f64;
                        let v10089: Lanes<5>;
                        let v10090: Lanes<5>;
                        if v7420 != 0.0 {
                            let v7421 = v663 * v6921;
                            let v7422 = v1 / v7421;
                            let v7423 = v7422 * v127;
                            let v17628 = (((((v10380 * v6921) + (v16640 * v663)) * v7422) * v10360) / v7421) * v127;
                            let v17629 = v17628 * v7424;
                            let v7426 = v78 + (v7424 * v7423);
                            let v7427 = v91 * v7426;
                            let v7428 = v7427 * v7426;
                            let v7429 = v7428 * v7426;
                            let v17636 = ((((v17629 * v91) * v7426) + (v17629 * v7427)) * v7426) + (v17629 * v7428);
                            let v7430 = v661 - v6990;
                            let v17637 = v10376 - v16706;
                            let v7431 = v7418 + v7413;
                            let v17640 = v10380 * v7431;
                            let v17641 = (v17234 + (Lanes([v17233[0], v17233[1], 0.0, v17233[2]]))) * v663;
                            let v7434 = v3500 * v7423;
                            let v7435 = (v663 * v7431) - v78;
                            let v7436 = v7434 * v7435;
                            let v17646 = (v17628 * v3500) * v7435;
                            let v17649 = (Lanes([0.0, 0.0, v17646[0], 0.0, 0.0])) + (((Lanes([0.0, 0.0, v17640[0], 0.0, 0.0])) + (Lanes([v17641[0], v17641[1], 0.0, v17641[2], v17641[3]]))) * v7434);
                            let v7437 = v7433 - v7436;
                            let v17650 = v17649 * v10360;
                            let v7438 = v7437 * v7437;
                            let v17651 = v17650 * v7437;
                            let v17652 = v17651 + v17651;
                            let v7440 = if v7429 < (v7438 * v3506) { 1.0 } else { 0.0 };
                            let v7452: f64;
                            let v10091: Lanes<5>;
                            if v7440 != 0.0 {
                                let v17659 = v17636 * v13;
                                let v7444 = (v13 * v7429) / v7437;
                                let v7446 = ((v7441 + v7437) + v7444) + v7436;
                                let v17665 = (v17650 + (((Lanes([0.0, 0.0, v17659[0], 0.0, 0.0])) - (v17650 * v7444)) / v7437)) + v17649;
                                v7452 = v7446;
                                v10091 = v17665;
                            } else {
                                let v7448 = (v7429 + v7438).sqrt();
                                let v7451 = (v7449 + v7448) + v7436;
                                let v17658 = (((Lanes([0.0, 0.0, v17636[0], 0.0, 0.0])) + v17652) * (v9353 / (v10405 * v7448))) + v17649;
                                v7452 = v7451;
                                v10091 = v17658;
                            }
                            let v7453 = v7452.powf(v1562);
                            let v17669 = v10091 * (v1562 * (v7452.powf(v17666)));
                            let v17671 = (v17628 * v3523) * v10360;
                            let v7459 = v748 * v7453;
                            let v7462 = (((v7454 - (v3523 * v7423)) + (v78 * v7453)) + (v7459 * v7453)) / v7453;
                            let v17684 = v10385 * v7462;
                            let v17687 = Lanes([v17233[0], v17233[1], 0.0, 0.0, v17233[2]]);
                            let v7465 = ((v7462 * v665) - v7413) + v7413;
                            let v17689 = ((((((((Lanes([0.0, 0.0, v17671[0], 0.0, 0.0])) + (v17669 * v78)) + (((v17669 * v748) * v7453) + (v17669 * v7459))) - (v17669 * v7462)) / v7453) * v665) + (Lanes([0.0, 0.0, v17684[0], 0.0, 0.0]))) - v17687) + v17687;
                            let v7466 = v7465 / v7430;
                            let v17690 = v17637 * v7466;
                            let v17694 = ((v17689 - (Lanes([0.0, 0.0, v17690[0], 0.0, 0.0]))) / v7430) * v7466;
                            let v7469 = (v1 + (v7466 * v7466)).sqrt();
                            let v7470 = v7465 / v7469;
                            let v7473 = v127 * (v7418 - (v7470 - v7413));
                            let v17705 = ((Lanes([v17234[0], v17234[1], 0.0, v17234[2], v17234[3]])) - (((v17689 - (((v17694 + v17694) * (v9353 / (v10405 * v7469))) * v7470)) / v7469) - v17687)) * v127;
                            v7766 = v7473;
                            v7768 = v7473;
                            v10089 = v17705;
                            v10090 = v17705;
                        } else {
                            let v7475 = v7418 + v7413;
                            let v17237 = v17234 + (Lanes([v17233[0], v17233[1], 0.0, v17233[2]]));
                            let v17238 = v10380 * v7475;
                            let v17239 = v17237 * v663;
                            let v17241 = Lanes([v17239[0], v17239[1], 0.0, v17239[2], v17239[3]]);
                            let v17242 = (Lanes([0.0, 0.0, v17238[0], 0.0, 0.0])) + v17241;
                            let v7477 = (v663 * v7475) - v1;
                            let v7480 = v6981 * v664;
                            let v17246 = (v16694 * v664) + (v10382 * v6981);
                            let v7481 = (v90 * (v7477 + v7474)) / v7480;
                            let v17247 = v17246 * v7481;
                            let v17250 = ((v17242 * v90) - (Lanes([0.0, 0.0, v17247[0], 0.0, 0.0]))) / v7480;
                            let v7482 = v1 + v7481;
                            let v7484 = if v7482 < v7483 { 1.0 } else { 0.0 };
                            let v7488: f64;
                            let v10092: Lanes<5>;
                            if v7484 != 0.0 {
                                v7488 = v7485;
                                v10092 = v10549;
                            } else {
                                v7488 = v7482;
                                v10092 = v17250;
                            }
                            let v7487 = (v6981 * v663) / v78;
                            let v17254 = ((v16694 * v663) + (v10380 * v6981)) / v78;
                            let v7489 = v7488.sqrt();
                            let v7490 = v1 - v7489;
                            let v17259 = v17254 * v7490;
                            let v17263 = Lanes([v17234[0], v17234[1], 0.0, v17234[2], v17234[3]]);
                            let v7493 = (v7418 + (v7487 * v7490)) + v7413;
                            let v17265 = Lanes([v17233[0], v17233[1], 0.0, 0.0, v17233[2]]);
                            let v17267 = v10380 * v7493;
                            let v7496 = (-(v663 * v7493)).exp();
                            let v7499 = (v90 * (v7477 + v7496)) / v7480;
                            let v17275 = v17246 * v7499;
                            let v17278 = (((v17242 + ((((Lanes([0.0, 0.0, v17267[0], 0.0, 0.0])) + (((v17263 + ((Lanes([0.0, 0.0, v17259[0], 0.0, 0.0])) + (((v10092 * (v9353 / (v10405 * v7489))) * v10360) * v7487))) + v17265) * v663)) * v10360) * v7496)) * v90) - (Lanes([0.0, 0.0, v17275[0], 0.0, 0.0]))) / v7480;
                            let v7500 = v1 + v7499;
                            let v7502 = if v7500 < v7501 { 1.0 } else { 0.0 };
                            let v7504: f64;
                            let v10093: Lanes<5>;
                            if v7502 != 0.0 {
                                v7504 = v7503;
                                v10093 = v10549;
                            } else {
                                v7504 = v7500;
                                v10093 = v17278;
                            }
                            let v7505 = v7504.sqrt();
                            let v7506 = v1 - v7505;
                            let v17283 = v17254 * v7506;
                            let v7509 = (v7418 + (v7487 * v7506)) + v7413;
                            let v7510 = v663 * v7509;
                            let v17289 = v10380 * v7509;
                            let v17292 = (Lanes([0.0, 0.0, v17289[0], 0.0, 0.0])) + (((v17263 + ((Lanes([0.0, 0.0, v17283[0], 0.0, 0.0])) + (((v10093 * (v9353 / (v10405 * v7505))) * v10360) * v7487))) + v17265) * v663);
                            let v7511 = if v7510 < v96 { 1.0 } else { 0.0 };
                            let v7589: f64;
                            let v10094: Lanes<5>;
                            if v7511 != 0.0 {
                                let v7514 = v663 * v6980;
                                let v7515 = v1 / v7514;
                                let v17298 = ((((v10380 * v6980) + (v16692 * v663)) * v7515) * v10360) / v7514;
                                let v7516 = v7513 + v7515;
                                let v17299 = v17237 * v10360;
                                let v7518 = (-v7475) / v6980;
                                let v17300 = v16692 * v7518;
                                let v17307 = ((v17298 * v7512) / v7521) * v10360;
                                let v7526 = (v7519 - ((v7512 * v7516) / v7521)) + (v7518 / v7524);
                                let v17310 = (Lanes([0.0, 0.0, v17307[0], 0.0, 0.0])) + ((((Lanes([v17299[0], v17299[1], 0.0, v17299[2], v17299[3]])) - (Lanes([0.0, 0.0, v17300[0], 0.0, 0.0]))) / v6980) / v7524);
                                let v7532 = ((v7527 * v7516) - v7529) / v7531;
                                let v17312 = (v17298 * v7527) / v7531;
                                let v17313 = v17310 * v7526;
                                let v7534 = v7532 * v7532;
                                let v17315 = v17312 * v7532;
                                let v17319 = ((v17315 + v17315) * v7532) + (v17312 * v7534);
                                let v7537 = ((v7526 * v7526) + (v7534 * v7532)).sqrt();
                                let v17324 = ((v17313 + v17313) + (Lanes([0.0, 0.0, v17319[0], 0.0, 0.0]))) * (v9353 / (v10405 * v7537));
                                let v7539 = (-v7526) + v7537;
                                let v7541 = v7526 + v7537;
                                let v7546 = ((v7539.powf(v1562)) + (-(v7541.powf(v1562)))) - v7545;
                                let v17339 = v10385 * v7546;
                                let v7549 = ((v7546 * v665) - v7413) + v7413;
                                let v7550 = v663 * v7549;
                                let v17344 = v10380 * v7549;
                                let v17347 = (Lanes([0.0, 0.0, v17344[0], 0.0, 0.0])) + (((((((((v17310 * v10360) + v17324) * (v1562 * (v7539.powf(v17327)))) + (((v17310 + v17324) * (v1562 * (v7541.powf(v17332)))) * v10360)) * v665) + (Lanes([0.0, 0.0, v17339[0], 0.0, 0.0]))) - v17265) + v17265) * v663);
                                v7589 = v7550;
                                v10094 = v17347;
                            } else {
                                v7589 = v7510;
                                v10094 = v17292;
                            }
                            let v7551 = if v7123 > v0 { 1.0 } else { 0.0 };
                            let v7605: f64;
                            let v10095: Lanes<5>;
                            if v7551 != 0.0 {
                                let v7552 = v7475 + v79;
                                let v17348 = v10380 * v7419;
                                let v17349 = v17235 * v663;
                                let v7554 = (v663 * v7419).exp();
                                let v7555 = v7554 + v362;
                                let v7556 = v731 / v39;
                                let v7557 = v7556 * v7556;
                                let v17355 = (v10423 / v39) * v7556;
                                let v17356 = v17355 + v17355;
                                let v7558 = v7557 * v7555;
                                let v17357 = v17356 * v7555;
                                let v7559 = v663 * v7552;
                                let v17361 = v10380 * v7552;
                                let v17363 = (Lanes([0.0, 0.0, v17361[0], 0.0, 0.0])) + v17241;
                                let v7560 = v7558 * v7480;
                                let v17365 = v17246 * v7558;
                                let v17367 = (((Lanes([0.0, 0.0, v17357[0], 0.0])) + ((((Lanes([0.0, 0.0, v17348[0], 0.0])) + (Lanes([v17349[0], v17349[1], 0.0, v17349[2]]))) * v7554) * v7557)) * v7480) + (Lanes([0.0, 0.0, v17365[0], 0.0]));
                                let v17368 = v17363 * v7559;
                                let v7562 = v7560 + (v7559 * v7559);
                                let v17370 = Lanes([v17367[0], v17367[1], v17367[2], 0.0, v17367[3]]);
                                let v7564 = v7557 * v7480;
                                let v7565 = v7564.ln();
                                let v17378 = ((v17356 * v7480) + (v17246 * v7557)) * (v9353 / v7564);
                                let v17379 = Lanes([0.0, 0.0, v17378[0], 0.0, 0.0]);
                                let v7567 = v663 * v7413;
                                let v17381 = v10380 * v7413;
                                let v17382 = v17233 * v663;
                                let v17385 = (Lanes([0.0, 0.0, v17381[0], 0.0])) + (Lanes([v17382[0], v17382[1], 0.0, v17382[2]]));
                                let v17386 = Lanes([v17385[0], v17385[1], v17385[2], 0.0, v17385[3]]);
                                let v17388 = v17363 - ((((v17370 + (v17368 + v17368)) * (v9353 / v7562)) - v17379) + v17386);
                                let v7570 = (v7559 - (((v7562.ln()) - v7565) + v7567)) - v1;
                                let v7571 = v90 * v7559;
                                let v17389 = v17363 * v90;
                                let v7572 = if v7571 > v0 { 1.0 } else { 0.0 };
                                let v7574: f64;
                                let v10096: Lanes<5>;
                                if v7572 != 0.0 {
                                    v7574 = v7571;
                                    v10096 = v17389;
                                } else {
                                    let v7573 = -v7571;
                                    let v17390 = v17389 * v10360;
                                    v7574 = v7573;
                                    v10096 = v17390;
                                }
                                let v17391 = v17388 * v7570;
                                let v7577 = ((v7570 * v7570) + v7574).sqrt();
                                let v17401 = v10380 * v79;
                                let v7583 = (v7559 - (v7559 - (v13 * (v7570 + v7577)))) + (v663 * v79);
                                let v17404 = ((v17363 - (v17363 - ((v17388 + (((v17391 + v17391) + v10096) * (v9353 / (v10405 * v7577)))) * v13))) + (Lanes([0.0, 0.0, v17401[0], 0.0, 0.0]))) * v7583;
                                let v7585 = v7560 + (v7583 * v7583);
                                let v7588 = ((v7585.ln()) - v7565) + v7567;
                                let v17410 = (((v17370 + (v17404 + v17404)) * (v9353 / v7585)) - v17379) + v17386;
                                let v17411 = v17410 - v10094;
                                let v7592 = (v7588 - v7589) - v7591;
                                let v7595 = (v90 * v7588) * v7594;
                                let v17413 = (v17410 * v90) * v7594;
                                let v7596 = if v7595 > v0 { 1.0 } else { 0.0 };
                                let v7598: f64;
                                let v10097: Lanes<5>;
                                if v7596 != 0.0 {
                                    v7598 = v7595;
                                    v10097 = v17413;
                                } else {
                                    let v7597 = -v7595;
                                    let v17414 = v17413 * v10360;
                                    v7598 = v7597;
                                    v10097 = v17414;
                                }
                                let v17415 = v17411 * v7592;
                                let v7601 = ((v7592 * v7592) + v7598).sqrt();
                                let v7604 = v7588 - (v13 * (v7592 + v7601));
                                let v17423 = v17410 - ((v17411 + (((v17415 + v17415) + v10097) * (v9353 / (v10405 * v7601)))) * v13);
                                v7605 = v7604;
                                v10095 = v17423;
                            } else {
                                v7605 = v7589;
                                v10095 = v10094;
                            }
                            let v7606 = v7605 / v663;
                            let v17424 = v10380 * v7606;
                            let v7607 = v7606 - v7413;
                            let v17428 = ((v10095 - (Lanes([0.0, 0.0, v17424[0], 0.0, 0.0]))) / v663) - v17265;
                            let v7610 = (-v7605).exp();
                            let v7611 = (v7605 - v1) + v7610;
                            let v17431 = v10095 + ((v10095 * v10360) * v7610);
                            let v7613 = if v7611 < v7612 { 1.0 } else { 0.0 };
                            let v7615: f64;
                            let v10098: Lanes<5>;
                            if v7613 != 0.0 {
                                v7615 = v7614;
                                v10098 = v10549;
                            } else {
                                v7615 = v7611;
                                v10098 = v17431;
                            }
                            let v7616 = v7615.sqrt();
                            let v7617 = v6921 * v7616;
                            let v17435 = v16640 * v7616;
                            let v17438 = (Lanes([0.0, 0.0, v17435[0], 0.0, 0.0])) + ((v10098 * (v9353 / (v10405 * v7616))) * v6921);
                            let v7619 = v127 * (v7418 - v7607);
                            let v17440 = (v17263 - v17428) * v127;
                            let v7620 = if v7123 == v1 { 1.0 } else { 0.0 };
                            let v7767: f64;
                            let v7769: f64;
                            let v10099: Lanes<5>;
                            let v10100: Lanes<5>;
                            if v7620 != 0.0 {
                                let v17441 = v10380 * v7419;
                                let v17442 = v17235 * v663;
                                let v7622 = (v663 * v7419).exp();
                                let v17446 = ((Lanes([0.0, 0.0, v17441[0], 0.0])) + (Lanes([v17442[0], v17442[1], 0.0, v17442[2]]))) * v7622;
                                let v7623 = v731 / v39;
                                let v7624 = v7623 * v7623;
                                let v17448 = (v10423 / v39) * v7623;
                                let v17449 = v17448 + v17448;
                                let v7625 = v7624 * v7622;
                                let v17450 = v17449 * v7622;
                                let v17453 = (Lanes([0.0, 0.0, v17450[0], 0.0])) + (v17446 * v7624);
                                let mut v7626: f64 = 0.0;
                                let mut v7629: f64 = 0.0;
                                let mut v7715: f64 = 0.0;
                                let mut v7745: f64 = 0.0;
                                let mut v7748: f64 = 0.0;
                                let mut v7758: f64 = 0.0;
                                let mut v7761: f64 = 0.0;
                                let mut v10101: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10102: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10103: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10104: Lanes<5> = Lanes([0.0; 5]);
                                let mut v10105: Lanes<5> = Lanes([0.0; 5]);
                                v7626 = v1;
                                v7629 = v7607;
                                v7715 = v0;
                                v7745 = v7605;
                                v7748 = v7749;
                                v7758 = v0;
                                v7761 = v0;
                                v10101 = v17428;
                                v10102 = v10095;
                                v10103 = v10053;
                                v10104 = v10549;
                                v10105 = v10549;
                                loop {
                                    let v7628 = if v7626 <= v7627 { 1.0 } else { 0.0 };
                                    if v7628 == 0.0 {
                                        break;
                                    }
                                    let v7630 = v7629 + v7413;
                                    let v7631 = v663 * v7630;
                                    let v17474 = v10380 * v7630;
                                    let v17477 = (Lanes([0.0, 0.0, v17474[0], 0.0, 0.0])) + ((v10101 + v17265) * v663);
                                    let v7632 = if v7631 < v644 { 1.0 } else { 0.0 };
                                    let v7708: f64;
                                    let v7712: f64;
                                    let v7751: f64;
                                    let v7762: f64;
                                    let v10106: Lanes<5>;
                                    let v10107: Lanes<5>;
                                    let v10108: Lanes<5>;
                                    let v10109: Lanes<5>;
                                    if v7632 != 0.0 {
                                        let v7633 = v7631 * v7631;
                                        let v17519 = v17477 * v7631;
                                        let v17520 = v17519 + v17519;
                                        let v7634 = v7633 * v7631;
                                        let v7637 = v7635 + (v7631 * v6318);
                                        let v7639 = v6316 + (v7631 * v7637);
                                        let v7640 = v7634 * v7639;
                                        let v17530 = (((v17520 * v7631) + (v17477 * v7633)) * v7639) + (((v17477 * v7637) + ((v17477 * v6318) * v7631)) * v7634);
                                        let v7643 = v7631 * v644;
                                        let v17531 = v17477 * v644;
                                        let v7645 = v7642 + (v7643 * v6318);
                                        let v7647 = v7641 + (v7631 * v7645);
                                        let v7648 = v7633 * v7647;
                                        let v7649 = v7625 * v7640;
                                        let v17539 = v17453 * v7640;
                                        let v7650 = v7649 * v7640;
                                        let v17545 = (((Lanes([v17539[0], v17539[1], v17539[2], 0.0, v17539[3]])) + (v17530 * v7625)) * v7640) + (v17530 * v7649);
                                        let v17547 = v10380 * v7625;
                                        let v7652 = (v7625 * v663) * v78;
                                        let v7653 = v7652 * v7640;
                                        let v17551 = (((v17453 * v663) + (Lanes([0.0, 0.0, v17547[0], 0.0]))) * v78) * v7640;
                                        let v7658 = v7656 + (v7631 * v6342);
                                        let v7660 = v6340 + (v7631 * v7658);
                                        let v7662 = v7655 + (v7631 * v7660);
                                        let v7664 = v6338 + (v7631 * v7662);
                                        let v7665 = v7631 * v7664;
                                        let v17570 = (v17477 * v7664) + (((v17477 * v7662) + (((v17477 * v7660) + (((v17477 * v7658) + ((v17477 * v6342) * v7631)) * v7631)) * v7631)) * v7631);
                                        let v7670 = v7668 + (v7643 * v6342);
                                        let v7672 = v7667 + (v7631 * v7670);
                                        let v7674 = v7666 + (v7631 * v7672);
                                        let v7676 = v6338 + (v7631 * v7674);
                                        let v17581 = v17570 * v7665;
                                        let v7680 = (((v7665 * v7665) + v7650) + v362).sqrt();
                                        let v17586 = ((v17581 + v17581) + v17545) * (v9353 / (v10405 * v7680));
                                        let v17587 = v10380 * v7676;
                                        let v7682 = (v663 * v7676) * v78;
                                        let v7685 = v7680 + v7680;
                                        let v7686 = ((v7682 * v7665) + (v7653 * v7648)) / v7685;
                                        let v17599 = (((((((Lanes([0.0, 0.0, v17587[0], 0.0, 0.0])) + (((v17477 * v7674) + (((v17477 * v7672) + (((v17477 * v7670) + ((v17531 * v6342) * v7631)) * v7631)) * v7631)) * v663)) * v78) * v7665) + (v17570 * v7682)) + ((((Lanes([v17551[0], v17551[1], v17551[2], 0.0, v17551[3]])) + (v17530 * v7652)) * v7648) + (((v17520 * v7647) + (((v17477 * v7645) + ((v17531 * v6318) * v7631)) * v7633)) * v7653))) - ((v17586 + v17586) * v7686)) / v7685;
                                        v7708 = v7680;
                                        v7712 = v7686;
                                        v7751 = v7665;
                                        v7762 = v7650;
                                        v10106 = v17586;
                                        v10107 = v17599;
                                        v10108 = v17570;
                                        v10109 = v17545;
                                    } else {
                                        let v7687 = if v7631 < v2535 { 1.0 } else { 0.0 };
                                        let v7700: f64;
                                        let v7703: f64;
                                        let v10110: Lanes<5>;
                                        let v10111: Lanes<5>;
                                        if v7687 != 0.0 {
                                            let v7688 = v7631.exp();
                                            let v17496 = v17477 * v7688;
                                            let v7689 = v7688 - v1;
                                            let v7690 = v7625 * v7689;
                                            let v17497 = v17453 * v7689;
                                            let v17500 = (Lanes([v17497[0], v17497[1], v17497[2], 0.0, v17497[3]])) + (v17496 * v7625);
                                            let v7691 = v7625 * v663;
                                            let v17502 = v10380 * v7625;
                                            let v7692 = v7691 * v7688;
                                            let v17505 = ((v17453 * v663) + (Lanes([0.0, 0.0, v17502[0], 0.0]))) * v7688;
                                            let v17508 = (Lanes([v17505[0], v17505[1], v17505[2], 0.0, v17505[3]])) + (v17496 * v7691);
                                            v7700 = v7690;
                                            v7703 = v7692;
                                            v10110 = v17500;
                                            v10111 = v17508;
                                        } else {
                                            let v17478 = v10380 * v7629;
                                            let v7694 = (v663 * v7629).exp();
                                            let v17482 = ((Lanes([0.0, 0.0, v17478[0], 0.0, 0.0])) + (v10101 * v663)) * v7694;
                                            let v7695 = v7694 - v7622;
                                            let v7696 = v7624 * v7695;
                                            let v17485 = v17449 * v7695;
                                            let v17488 = (Lanes([0.0, 0.0, v17485[0], 0.0, 0.0])) + ((v17482 - (Lanes([v17446[0], v17446[1], v17446[2], 0.0, v17446[3]]))) * v7624);
                                            let v7697 = v7624 * v663;
                                            let v7698 = v7697 * v7694;
                                            let v17492 = ((v17449 * v663) + (v10380 * v7624)) * v7694;
                                            let v17495 = (Lanes([0.0, 0.0, v17492[0], 0.0, 0.0])) + (v17482 * v7697);
                                            v7700 = v7696;
                                            v7703 = v7698;
                                            v10110 = v17488;
                                            v10111 = v17495;
                                        }
                                        let v7702 = ((v7631 - v1) + v7700).sqrt();
                                        let v17512 = (v17477 + v10110) * (v9353 / (v10405 * v7702));
                                        let v7705 = (v663 + v7703) / v7702;
                                        let v7706 = v7705 * v13;
                                        let v17518 = ((((Lanes([0.0, 0.0, v10380[0], 0.0, 0.0])) + v10111) - (v17512 * v7705)) / v7702) * v13;
                                        v7708 = v7702;
                                        v7712 = v7706;
                                        v7751 = v0;
                                        v7762 = v7700;
                                        v10106 = v17512;
                                        v10107 = v17518;
                                        v10108 = v10549;
                                        v10109 = v10110;
                                    }
                                    let v17601 = v16692 * v7708;
                                    let v7710 = (v7418 - v7629) - (v6980 * v7708);
                                    let v17605 = (v17263 - v10101) - ((Lanes([0.0, 0.0, v17601[0], 0.0, 0.0])) + (v10106 * v6980));
                                    let v17606 = v16692 * v7712;
                                    let v7714 = v7711 - (v6980 * v7712);
                                    let v17610 = ((Lanes([0.0, 0.0, v17606[0], 0.0, 0.0])) + (v10107 * v6980)) * v10360;
                                    let v7716 = if v7715 == v1 { 1.0 } else { 0.0 };
                                    let v7739: f64;
                                    let v7741: f64;
                                    let v7742: f64;
                                    let v10112: Lanes<5>;
                                    if v7716 != 0.0 {
                                        v7739 = v7717;
                                        v7741 = v7629;
                                        v7742 = v7715;
                                        v10112 = v10101;
                                    } else {
                                        let v7719 = (-v7710) / v7714;
                                        let v17614 = ((v17605 * v10360) - (v17610 * v7719)) / v7714;
                                        let v7721 = v7629.abs();
                                        let v17618 = v10101 * ((v10405 * (if v7629 >= v11274 { 1.0 } else { 0.0 })) - v9353);
                                        let v7722 = if v1 >= v7721 { 1.0 } else { 0.0 };
                                        let v7723: f64;
                                        let v10113: Lanes<5>;
                                        if v7722 != 0.0 {
                                            v7723 = v1;
                                            v10113 = v10549;
                                        } else {
                                            v7723 = v7721;
                                            v10113 = v17618;
                                        }
                                        let v7725 = v7720 * (v1 + v7723);
                                        let v17619 = v10113 * v7720;
                                        let v7727 = if (v7719.abs()) > v7725 { 1.0 } else { 0.0 };
                                        let v7732: f64;
                                        let v10114: Lanes<5>;
                                        if v7727 != 0.0 {
                                            let v7728 = if v7719 >= v0 { 1.0 } else { 0.0 };
                                            let v7730: f64;
                                            if v7728 != 0.0 {
                                                v7730 = v1;
                                            } else {
                                                v7730 = v7729;
                                            }
                                            let v7731 = v7725 * v7730;
                                            let v17620 = v17619 * v7730;
                                            v7732 = v7731;
                                            v10114 = v17620;
                                        } else {
                                            v7732 = v7719;
                                            v10114 = v17614;
                                        }
                                        let v7733 = v7629 + v7732;
                                        let v17621 = v10101 + v10114;
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
                                        v10112 = v17621;
                                    }
                                    let v7740 = v7739 + v1;
                                    v7626 = v7740;
                                    v7629 = v7741;
                                    v7715 = v7742;
                                    v7745 = v7631;
                                    v7748 = v7751;
                                    v7758 = v7708;
                                    v7761 = v7762;
                                    v10101 = v10112;
                                    v10102 = v17477;
                                    v10103 = v10108;
                                    v10104 = v10106;
                                    v10105 = v10109;
                                }
                                let v7744 = if v7715 == v0 { 1.0 } else { 0.0 };
                                if v7744 != 0.0 {
                                } else {
                                }
                                let v7746 = if v7745 < v644 { 1.0 } else { 0.0 };
                                let v7756: f64;
                                let v10115: Lanes<5>;
                                if v7746 != 0.0 {
                                    let v7747 = if v7745 < v96 { 1.0 } else { 0.0 };
                                    if v7747 != 0.0 {
                                    } else {
                                    }
                                    let v7753 = v7748 + v7752;
                                    v7756 = v7753;
                                    v10115 = v10103;
                                } else {
                                    let v7755 = (v7745 - v1).sqrt();
                                    let v17456 = v10102 * (v9353 / (v10405 * v7755));
                                    v7756 = v7755;
                                    v10115 = v17456;
                                }
                                let v7757 = v6921 * v7756;
                                let v17457 = v16640 * v7756;
                                let v17460 = (Lanes([0.0, 0.0, v17457[0], 0.0, 0.0])) + (v10115 * v6921);
                                let v7759 = v7758 + v7756;
                                let v7760 = v1 / v7759;
                                let v7763 = v6921 * v7761;
                                let v17465 = v16640 * v7761;
                                let v7765 = v7757 + (v7763 * v7760);
                                let v17472 = v17460 + ((((Lanes([0.0, 0.0, v17465[0], 0.0, 0.0])) + (v10105 * v6921)) * v7760) + (((((v10104 + v10115) * v7760) * v10360) / v7759) * v7763));
                                v7767 = v7765;
                                v7769 = v7757;
                                v10099 = v17472;
                                v10100 = v17460;
                            } else {
                                v7767 = v7619;
                                v7769 = v7617;
                                v10099 = v17440;
                                v10100 = v17438;
                            }
                            v7766 = v7767;
                            v7768 = v7769;
                            v10089 = v10099;
                            v10090 = v10100;
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
                        let v10116: Lanes<5>;
                        let v10117: Lanes<5>;
                        if v7776 != 0.0 {
                            let v7777 = v7772 * v7766;
                            let v17706 = v10089 * v7772;
                            let v7778 = v7772 * v7768;
                            let v17707 = v10090 * v7772;
                            v8626 = v7777;
                            v8655 = v7778;
                            v10116 = v17706;
                            v10117 = v17707;
                        } else {
                            v8626 = v8627;
                            v8655 = v8656;
                            v10116 = v10080;
                            v10117 = v10081;
                        }
                        let v7782 = if (if v7779 != 0.0 && v9 != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v7357 != 0.0 && v565 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v8632: f64;
                        let v8644: f64;
                        let v10118: Lanes<5>;
                        let v10119: Lanes<5>;
                        if v7782 != 0.0 {
                            let v7783 = v7772 * v7766;
                            let v17708 = v10089 * v7772;
                            let v7784 = v7772 * v7768;
                            let v17709 = v10090 * v7772;
                            v8632 = v7783;
                            v8644 = v7784;
                            v10118 = v17708;
                            v10119 = v17709;
                        } else {
                            v8632 = v8633;
                            v8644 = v8645;
                            v10118 = v10082;
                            v10119 = v10083;
                        }
                        v7801 = v0;
                        v7820 = v0;
                        v8625 = v8626;
                        v8631 = v8632;
                        v8643 = v8644;
                        v8654 = v8655;
                        v10040 = v11032;
                        v10041 = v11032;
                        v10042 = v10116;
                        v10043 = v10118;
                        v10044 = v10119;
                        v10045 = v10117;
                    }
                    let v7787 = (v6048 * v370) + (v6046 * v369);
                    let v8456: f64;
                    let v10120: Lanes<6>;
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
                        let v17734 = (v9397 - (Lanes([v9395[0], v9395[1], 0.0]))) * v7803;
                        let v7806 = v7801 + (v7803 * (v830 - v823));
                        let v17736 = v10040 + (Lanes([v17734[0], v17734[1], 0.0, v17734[2], 0.0, 0.0]));
                        v8456 = v7806;
                        v10120 = v17736;
                    } else {
                        v8456 = v7801;
                        v10120 = v10040;
                    }
                    let v7809 = (v6046 * v370) + (v6048 * v369);
                    let v8460: f64;
                    let v10121: Lanes<6>;
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
                        let v17737 = v9397 * v7822;
                        let v7824 = v7820 + (v7822 * v830);
                        let v17739 = v10041 + (Lanes([v17737[0], v17737[1], 0.0, v17737[2], 0.0, 0.0]));
                        v8460 = v7824;
                        v10121 = v17739;
                    } else {
                        v8460 = v7820;
                        v10121 = v10041;
                    }
                    v8455 = v8456;
                    v8459 = v8460;
                    v8624 = v8625;
                    v8630 = v8631;
                    v8642 = v8643;
                    v8653 = v8654;
                    v10034 = v10120;
                    v10035 = v10121;
                    v10036 = v10042;
                    v10037 = v10043;
                    v10038 = v10044;
                    v10039 = v10045;
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
                    let v16636 = (v9397 - (Lanes([v9395[0], v9395[1], 0.0]))) * v7852;
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
                    let v16637 = v9397 * v7877;
                    let v16638 = Lanes([v16636[0], v16636[1], 0.0, v16636[2], 0.0, 0.0]);
                    let v16639 = Lanes([v16637[0], v16637[1], 0.0, v16637[2], 0.0, 0.0]);
                    v8455 = v7854;
                    v8459 = v7878;
                    v8624 = v0;
                    v8630 = v0;
                    v8642 = v0;
                    v8653 = v0;
                    v10034 = v16638;
                    v10035 = v16639;
                    v10036 = v10549;
                    v10037 = v10549;
                    v10038 = v10549;
                    v10039 = v10549;
                }
                v8454 = v8455;
                v8458 = v8459;
                v8623 = v8624;
                v8629 = v8630;
                v8641 = v8642;
                v8652 = v8653;
                v10028 = v10034;
                v10029 = v10035;
                v10030 = v10036;
                v10031 = v10037;
                v10032 = v10038;
                v10033 = v10039;
            } else {
                v8454 = v0;
                v8458 = v0;
                v8623 = v0;
                v8629 = v0;
                v8641 = v0;
                v8652 = v0;
                v10028 = v11032;
                v10029 = v11032;
                v10030 = v10549;
                v10031 = v10549;
                v10032 = v10549;
                v10033 = v10549;
            }
            let v8673: f64;
            let v8674: f64;
            let v8675: f64;
            let v8677: f64;
            let v10122: Lanes<3>;
            let v10123: Lanes<3>;
            let v10124: Lanes<2>;
            let v10125: Lanes<2>;
            if v565 != 0.0 {
                let v7884 = (v121 * v209) - (v661 * v663);
                let v17745 = ((v10376 * v663) + (v10380 * v661)) * v10360;
                let v7886 = v699.ln();
                let v17747 = v10386 * (v9353 / v699);
                let v7891 = ((v7884 + (v7885 * v7886)) / v7889).exp();
                let v7892 = v7881 * v7891;
                let v17752 = (((v17745 + (v17747 * v7885)) / v7889) * v7891) * v7881;
                let v7897 = ((v7884 + (v7893 * v7886)) / v7889).exp();
                let v7898 = v7881 * v7897;
                let v17757 = (((v17745 + (v17747 * v7893)) / v7889) * v7897) * v7881;
                let v7900 = v7899 * v12;
                let v7901 = v7900 * v7892;
                let v17758 = v17752 * v7900;
                let v7902 = v7900 * v7898;
                let v17759 = v17757 * v7900;
                let v7904 = v7903 * v12;
                let v7905 = v7904 * v7892;
                let v17760 = v17752 * v7904;
                let v7906 = v7904 * v7898;
                let v17761 = v17757 * v7904;
                let v17762 = v10386 * v699;
                let v7908 = v7901 + v362;
                let v7909 = v7905 + v362;
                let v7910 = v7889 / v663;
                let v17766 = ((v10380 * v7910) * v10360) / v663;
                let v7912 = v7911 * (v699 * v699);
                let v17767 = (v17762 + v17762) * v7911;
                let v7913 = v7912 / v7908;
                let v7914 = v1 + v7913;
                let v7915 = v7914.ln();
                let v7916 = v7910 * v7915;
                let v17775 = (v17766 * v7915) + ((((v17767 - (v17758 * v7913)) / v7908) * (v9353 / v7914)) * v7910);
                let v7917 = v7912 / v7909;
                let v7918 = v1 + v7917;
                let v7919 = v7918.ln();
                let v7920 = v7910 * v7919;
                let v17783 = (v17766 * v7919) + ((((v17767 - (v17760 * v7917)) / v7909) * (v9353 / v7918)) * v7910);
                let v7921 = v7889 * v665;
                let v17784 = v10385 * v7889;
                let v7922 = if v7879 < v7916 { 1.0 } else { 0.0 };
                let v7936: f64;
                let v10126: Lanes<3>;
                if v7922 != 0.0 {
                    let v7923 = v7879 / v7921;
                    let v17807 = v17784 * v7923;
                    let v7924 = v7923.exp();
                    let v7925 = v7924 - v1;
                    let v7926 = v7901 * v7925;
                    let v17813 = v17758 * v7925;
                    let v17816 = (Lanes([0.0, v17813[0], 0.0])) + (((((Lanes([v9371[0], 0.0, v9371[1]])) - (Lanes([0.0, v17807[0], 0.0]))) / v7921) * v7924) * v7901);
                    v7936 = v7926;
                    v10126 = v17816;
                } else {
                    let v7927 = v7916 / v7921;
                    let v7928 = v7927.exp();
                    let v17788 = ((v17775 - (v17784 * v7927)) / v7921) * v7928;
                    let v7929 = v7928 - v1;
                    let v17791 = (v17758 * v7929) + (v17788 * v7901);
                    let v7931 = v7901 / v7921;
                    let v7932 = v7931 * v7928;
                    let v7933 = v7879 - v7916;
                    let v17801 = ((((v17758 - (v17784 * v7931)) / v7921) * v7928) + (v17788 * v7931)) * v7933;
                    let v7935 = (v7901 * v7929) + (v7932 * v7933);
                    let v17806 = (Lanes([0.0, v17791[0], 0.0])) + ((Lanes([0.0, v17801[0], 0.0])) + (((Lanes([v9371[0], 0.0, v9371[1]])) - (Lanes([0.0, v17775[0], 0.0]))) * v7932));
                    v7936 = v7935;
                    v10126 = v17806;
                }
                let v7938 = v7937 * v7879;
                let v17818 = (v9371 * v7937) * v7902;
                let v17819 = v17759 * v7938;
                let v7940 = v7936 + (v7938 * v7902);
                let v17823 = v10126 + ((Lanes([v17818[0], 0.0, v17818[1]])) + (Lanes([0.0, v17819[0], 0.0])));
                let v7941 = if v7880 < v7920 { 1.0 } else { 0.0 };
                let v7955: f64;
                let v10127: Lanes<3>;
                if v7941 != 0.0 {
                    let v7942 = v7880 / v7921;
                    let v17846 = v17784 * v7942;
                    let v7943 = v7942.exp();
                    let v7944 = v7943 - v1;
                    let v7945 = v7905 * v7944;
                    let v17852 = v17760 * v7944;
                    let v17855 = (Lanes([0.0, v17852[0], 0.0])) + (((((Lanes([v9372[0], 0.0, v9372[1]])) - (Lanes([0.0, v17846[0], 0.0]))) / v7921) * v7943) * v7905);
                    v7955 = v7945;
                    v10127 = v17855;
                } else {
                    let v7946 = v7920 / v7921;
                    let v7947 = v7946.exp();
                    let v17827 = ((v17783 - (v17784 * v7946)) / v7921) * v7947;
                    let v7948 = v7947 - v1;
                    let v17830 = (v17760 * v7948) + (v17827 * v7905);
                    let v7950 = v7905 / v7921;
                    let v7951 = v7950 * v7947;
                    let v7952 = v7880 - v7920;
                    let v17840 = ((((v17760 - (v17784 * v7950)) / v7921) * v7947) + (v17827 * v7950)) * v7952;
                    let v7954 = (v7905 * v7948) + (v7951 * v7952);
                    let v17845 = (Lanes([0.0, v17830[0], 0.0])) + ((Lanes([0.0, v17840[0], 0.0])) + (((Lanes([v9372[0], 0.0, v9372[1]])) - (Lanes([0.0, v17783[0], 0.0]))) * v7951));
                    v7955 = v7954;
                    v10127 = v17845;
                }
                let v7956 = v7937 * v7880;
                let v17857 = (v9372 * v7937) * v7906;
                let v17858 = v17761 * v7956;
                let v17863 = v9371 * v381;
                let v7960 = v7940 + (v381 * v7879);
                let v17865 = v17823 + (Lanes([v17863[0], 0.0, v17863[1]]));
                let v17866 = v9372 * v381;
                let v7962 = (v7955 + (v7956 * v7906)) + (v381 * v7880);
                let v17868 = (v10127 + ((Lanes([v17857[0], 0.0, v17857[1]])) + (Lanes([0.0, v17858[0], 0.0])))) + (Lanes([v17866[0], 0.0, v17866[1]]));
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
                let v10128: Lanes<2>;
                if v7972 != 0.0 {
                    let v7975 = v7973 * (v7971 - v6899);
                    let v7977 = v7976 * v6899;
                    let v7978 = if v7880 < v0 { 1.0 } else { 0.0 };
                    let v8215: f64;
                    let v10129: Lanes<2>;
                    if v7978 != 0.0 {
                        let v7980 = if v7979 > v0 { 1.0 } else { 0.0 };
                        let v8007: f64;
                        let v10130: Lanes<2>;
                        if v7980 != 0.0 {
                            let v7983 = v1 - (v7880 / v7981);
                            let v17917 = (v9372 / v7981) * v10360;
                            let v7985 = if v7984 == v13 { 1.0 } else { 0.0 };
                            let v7991: f64;
                            let v10131: Lanes<2>;
                            if v7985 != 0.0 {
                                let v7986 = v7983.sqrt();
                                let v7987 = v1 / v7986;
                                let v17927 = (((v17917 * (v9353 / (v10405 * v7986))) * v7987) * v10360) / v7986;
                                v7991 = v7987;
                                v10131 = v17927;
                            } else {
                                let v7988 = -v7984;
                                let v7989 = v7983.powf(v7988);
                                let v17921 = v17917 * (v7988 * (v7983.powf((v7988 - v9353))));
                                v7991 = v7989;
                                v10131 = v17921;
                            }
                            let v7990 = v7981 * v7979;
                            let v7995 = v1 - v7984;
                            let v7996 = (v7990 * (v1 - (v7983 * v7991))) / v7995;
                            let v17933 = ((((v17917 * v7991) + (v10131 * v7983)) * v10360) * v7990) / v7995;
                            v8007 = v7996;
                            v10130 = v17933;
                        } else {
                            v8007 = v0;
                            v10130 = v10351;
                        }
                        let v7997 = if v7975 > v0 { 1.0 } else { 0.0 };
                        let v8026: f64;
                        let v10132: Lanes<2>;
                        if v7997 != 0.0 {
                            let v8000 = v1 - (v7880 / v7998);
                            let v17935 = (v9372 / v7998) * v10360;
                            let v8002 = if v8001 == v13 { 1.0 } else { 0.0 };
                            let v8009: f64;
                            let v10133: Lanes<2>;
                            if v8002 != 0.0 {
                                let v8003 = v8000.sqrt();
                                let v8004 = v1 / v8003;
                                let v17945 = (((v17935 * (v9353 / (v10405 * v8003))) * v8004) * v10360) / v8003;
                                v8009 = v8004;
                                v10133 = v17945;
                            } else {
                                let v8005 = -v8001;
                                let v8006 = v8000.powf(v8005);
                                let v17939 = v17935 * (v8005 * (v8000.powf((v8005 - v9353))));
                                v8009 = v8006;
                                v10133 = v17939;
                            }
                            let v8008 = v7998 * v7975;
                            let v8013 = v1 - v8001;
                            let v8015 = v8007 + ((v8008 * (v1 - (v8000 * v8009))) / v8013);
                            let v17952 = v10130 + (((((v17935 * v8009) + (v10133 * v8000)) * v10360) * v8008) / v8013);
                            v8026 = v8015;
                            v10132 = v17952;
                        } else {
                            v8026 = v8007;
                            v10132 = v10130;
                        }
                        let v8016 = if v7977 > v0 { 1.0 } else { 0.0 };
                        let v8216: f64;
                        let v10134: Lanes<2>;
                        if v8016 != 0.0 {
                            let v8019 = v1 - (v7880 / v8017);
                            let v17954 = (v9372 / v8017) * v10360;
                            let v8021 = if v8020 == v13 { 1.0 } else { 0.0 };
                            let v8028: f64;
                            let v10135: Lanes<2>;
                            if v8021 != 0.0 {
                                let v8022 = v8019.sqrt();
                                let v8023 = v1 / v8022;
                                let v17964 = (((v17954 * (v9353 / (v10405 * v8022))) * v8023) * v10360) / v8022;
                                v8028 = v8023;
                                v10135 = v17964;
                            } else {
                                let v8024 = -v8020;
                                let v8025 = v8019.powf(v8024);
                                let v17958 = v17954 * (v8024 * (v8019.powf((v8024 - v9353))));
                                v8028 = v8025;
                                v10135 = v17958;
                            }
                            let v8027 = v8017 * v7977;
                            let v8032 = v1 - v8020;
                            let v8034 = v8026 + ((v8027 * (v1 - (v8019 * v8028))) / v8032);
                            let v17971 = v10132 + (((((v17954 * v8028) + (v10135 * v8019)) * v10360) * v8027) / v8032);
                            v8216 = v8034;
                            v10134 = v17971;
                        } else {
                            v8216 = v8026;
                            v10134 = v10132;
                        }
                        v8215 = v8216;
                        v10129 = v10134;
                    } else {
                        let v8044 = (((v7979 * v7984) / v7981) + ((v7975 * v8001) / v7998)) + ((v7977 * v8020) / v8017);
                        let v8047 = ((v7979 + v7975) + v7977) + ((v7880 * v13) * v8044);
                        let v8048 = v7880 * v8047;
                        let v17915 = (v9372 * v8047) + (((v9372 * v13) * v8044) * v7880);
                        v8215 = v8048;
                        v10129 = v17915;
                    }
                    v8214 = v8215;
                    v10128 = v10129;
                } else {
                    let v8049 = v7976 * v7971;
                    let v8050 = if v7880 < v0 { 1.0 } else { 0.0 };
                    let v8217: f64;
                    let v10136: Lanes<2>;
                    if v8050 != 0.0 {
                        let v8051 = if v7979 > v0 { 1.0 } else { 0.0 };
                        let v8074: f64;
                        let v10137: Lanes<2>;
                        if v8051 != 0.0 {
                            let v8053 = v1 - (v7880 / v7981);
                            let v17875 = (v9372 / v7981) * v10360;
                            let v8054 = if v7984 == v13 { 1.0 } else { 0.0 };
                            let v8060: f64;
                            let v10138: Lanes<2>;
                            if v8054 != 0.0 {
                                let v8055 = v8053.sqrt();
                                let v8056 = v1 / v8055;
                                let v17885 = (((v17875 * (v9353 / (v10405 * v8055))) * v8056) * v10360) / v8055;
                                v8060 = v8056;
                                v10138 = v17885;
                            } else {
                                let v8057 = -v7984;
                                let v8058 = v8053.powf(v8057);
                                let v17879 = v17875 * (v8057 * (v8053.powf((v8057 - v9353))));
                                v8060 = v8058;
                                v10138 = v17879;
                            }
                            let v8059 = v7981 * v7979;
                            let v8064 = v1 - v7984;
                            let v8065 = (v8059 * (v1 - (v8053 * v8060))) / v8064;
                            let v17891 = ((((v17875 * v8060) + (v10138 * v8053)) * v10360) * v8059) / v8064;
                            v8074 = v8065;
                            v10137 = v17891;
                        } else {
                            v8074 = v0;
                            v10137 = v10351;
                        }
                        let v8066 = if v8049 > v0 { 1.0 } else { 0.0 };
                        let v8218: f64;
                        let v10139: Lanes<2>;
                        if v8066 != 0.0 {
                            let v8068 = v1 - (v7880 / v8017);
                            let v17893 = (v9372 / v8017) * v10360;
                            let v8069 = if v8020 == v13 { 1.0 } else { 0.0 };
                            let v8076: f64;
                            let v10140: Lanes<2>;
                            if v8069 != 0.0 {
                                let v8070 = v8068.sqrt();
                                let v8071 = v1 / v8070;
                                let v17903 = (((v17893 * (v9353 / (v10405 * v8070))) * v8071) * v10360) / v8070;
                                v8076 = v8071;
                                v10140 = v17903;
                            } else {
                                let v8072 = -v8020;
                                let v8073 = v8068.powf(v8072);
                                let v17897 = v17893 * (v8072 * (v8068.powf((v8072 - v9353))));
                                v8076 = v8073;
                                v10140 = v17897;
                            }
                            let v8075 = v8017 * v8049;
                            let v8080 = v1 - v8020;
                            let v8082 = v8074 + ((v8075 * (v1 - (v8068 * v8076))) / v8080);
                            let v17910 = v10137 + (((((v17893 * v8076) + (v10140 * v8068)) * v10360) * v8075) / v8080);
                            v8218 = v8082;
                            v10139 = v17910;
                        } else {
                            v8218 = v8074;
                            v10139 = v10137;
                        }
                        v8217 = v8218;
                        v10136 = v10139;
                    } else {
                        let v8088 = ((v7979 * v7984) / v7981) + ((v8049 * v8020) / v8017);
                        let v8091 = (v7979 + v8049) + ((v7880 * v13) * v8088);
                        let v8092 = v7880 * v8091;
                        let v17873 = (v9372 * v8091) + (((v9372 * v13) * v8088) * v7880);
                        v8217 = v8092;
                        v10136 = v17873;
                    }
                    v8214 = v8217;
                    v10128 = v10136;
                }
                let v8094 = if v8093 > v7794 { 1.0 } else { 0.0 };
                let v8242: f64;
                let v10141: Lanes<2>;
                if v8094 != 0.0 {
                    let v8096 = v7973 * (v8093 - v7794);
                    let v8097 = v7976 * v7794;
                    let v8098 = if v7879 < v0 { 1.0 } else { 0.0 };
                    let v8243: f64;
                    let v10142: Lanes<2>;
                    if v8098 != 0.0 {
                        let v8100 = if v8099 > v0 { 1.0 } else { 0.0 };
                        let v8123: f64;
                        let v10143: Lanes<2>;
                        if v8100 != 0.0 {
                            let v8102 = v1 - (v7879 / v7981);
                            let v18020 = (v9371 / v7981) * v10360;
                            let v8103 = if v7984 == v13 { 1.0 } else { 0.0 };
                            let v8109: f64;
                            let v10144: Lanes<2>;
                            if v8103 != 0.0 {
                                let v8104 = v8102.sqrt();
                                let v8105 = v1 / v8104;
                                let v18030 = (((v18020 * (v9353 / (v10405 * v8104))) * v8105) * v10360) / v8104;
                                v8109 = v8105;
                                v10144 = v18030;
                            } else {
                                let v8106 = -v7984;
                                let v8107 = v8102.powf(v8106);
                                let v18024 = v18020 * (v8106 * (v8102.powf((v8106 - v9353))));
                                v8109 = v8107;
                                v10144 = v18024;
                            }
                            let v8108 = v7981 * v8099;
                            let v8113 = v1 - v7984;
                            let v8114 = (v8108 * (v1 - (v8102 * v8109))) / v8113;
                            let v18036 = ((((v18020 * v8109) + (v10144 * v8102)) * v10360) * v8108) / v8113;
                            v8123 = v8114;
                            v10143 = v18036;
                        } else {
                            v8123 = v0;
                            v10143 = v10350;
                        }
                        let v8115 = if v8096 > v0 { 1.0 } else { 0.0 };
                        let v8140: f64;
                        let v10145: Lanes<2>;
                        if v8115 != 0.0 {
                            let v8117 = v1 - (v7879 / v7998);
                            let v18038 = (v9371 / v7998) * v10360;
                            let v8118 = if v8001 == v13 { 1.0 } else { 0.0 };
                            let v8125: f64;
                            let v10146: Lanes<2>;
                            if v8118 != 0.0 {
                                let v8119 = v8117.sqrt();
                                let v8120 = v1 / v8119;
                                let v18048 = (((v18038 * (v9353 / (v10405 * v8119))) * v8120) * v10360) / v8119;
                                v8125 = v8120;
                                v10146 = v18048;
                            } else {
                                let v8121 = -v8001;
                                let v8122 = v8117.powf(v8121);
                                let v18042 = v18038 * (v8121 * (v8117.powf((v8121 - v9353))));
                                v8125 = v8122;
                                v10146 = v18042;
                            }
                            let v8124 = v7998 * v8096;
                            let v8129 = v1 - v8001;
                            let v8131 = v8123 + ((v8124 * (v1 - (v8117 * v8125))) / v8129);
                            let v18055 = v10143 + (((((v18038 * v8125) + (v10146 * v8117)) * v10360) * v8124) / v8129);
                            v8140 = v8131;
                            v10145 = v18055;
                        } else {
                            v8140 = v8123;
                            v10145 = v10143;
                        }
                        let v8132 = if v8097 > v0 { 1.0 } else { 0.0 };
                        let v8244: f64;
                        let v10147: Lanes<2>;
                        if v8132 != 0.0 {
                            let v8134 = v1 - (v7879 / v8017);
                            let v18057 = (v9371 / v8017) * v10360;
                            let v8135 = if v8020 == v13 { 1.0 } else { 0.0 };
                            let v8142: f64;
                            let v10148: Lanes<2>;
                            if v8135 != 0.0 {
                                let v8136 = v8134.sqrt();
                                let v8137 = v1 / v8136;
                                let v18067 = (((v18057 * (v9353 / (v10405 * v8136))) * v8137) * v10360) / v8136;
                                v8142 = v8137;
                                v10148 = v18067;
                            } else {
                                let v8138 = -v8020;
                                let v8139 = v8134.powf(v8138);
                                let v18061 = v18057 * (v8138 * (v8134.powf((v8138 - v9353))));
                                v8142 = v8139;
                                v10148 = v18061;
                            }
                            let v8141 = v8017 * v8097;
                            let v8146 = v1 - v8020;
                            let v8148 = v8140 + ((v8141 * (v1 - (v8134 * v8142))) / v8146);
                            let v18074 = v10145 + (((((v18057 * v8142) + (v10148 * v8134)) * v10360) * v8141) / v8146);
                            v8244 = v8148;
                            v10147 = v18074;
                        } else {
                            v8244 = v8140;
                            v10147 = v10145;
                        }
                        v8243 = v8244;
                        v10142 = v10147;
                    } else {
                        let v8158 = (((v8099 * v7984) / v7981) + ((v8096 * v8001) / v7998)) + ((v8097 * v8020) / v8017);
                        let v8161 = ((v8099 + v8096) + v8097) + ((v7879 * v13) * v8158);
                        let v8162 = v7879 * v8161;
                        let v18018 = (v9371 * v8161) + (((v9371 * v13) * v8158) * v7879);
                        v8243 = v8162;
                        v10142 = v18018;
                    }
                    v8242 = v8243;
                    v10141 = v10142;
                } else {
                    let v8163 = v7976 * v8093;
                    let v8164 = if v7879 < v0 { 1.0 } else { 0.0 };
                    let v8245: f64;
                    let v10149: Lanes<2>;
                    if v8164 != 0.0 {
                        let v8165 = if v8099 > v0 { 1.0 } else { 0.0 };
                        let v8188: f64;
                        let v10150: Lanes<2>;
                        if v8165 != 0.0 {
                            let v8167 = v1 - (v7879 / v7981);
                            let v17978 = (v9371 / v7981) * v10360;
                            let v8168 = if v7984 == v13 { 1.0 } else { 0.0 };
                            let v8174: f64;
                            let v10151: Lanes<2>;
                            if v8168 != 0.0 {
                                let v8169 = v8167.sqrt();
                                let v8170 = v1 / v8169;
                                let v17988 = (((v17978 * (v9353 / (v10405 * v8169))) * v8170) * v10360) / v8169;
                                v8174 = v8170;
                                v10151 = v17988;
                            } else {
                                let v8171 = -v7984;
                                let v8172 = v8167.powf(v8171);
                                let v17982 = v17978 * (v8171 * (v8167.powf((v8171 - v9353))));
                                v8174 = v8172;
                                v10151 = v17982;
                            }
                            let v8173 = v7981 * v8099;
                            let v8178 = v1 - v7984;
                            let v8179 = (v8173 * (v1 - (v8167 * v8174))) / v8178;
                            let v17994 = ((((v17978 * v8174) + (v10151 * v8167)) * v10360) * v8173) / v8178;
                            v8188 = v8179;
                            v10150 = v17994;
                        } else {
                            v8188 = v0;
                            v10150 = v10350;
                        }
                        let v8180 = if v8163 > v0 { 1.0 } else { 0.0 };
                        let v8246: f64;
                        let v10152: Lanes<2>;
                        if v8180 != 0.0 {
                            let v8182 = v1 - (v7879 / v8017);
                            let v17996 = (v9371 / v8017) * v10360;
                            let v8183 = if v8020 == v13 { 1.0 } else { 0.0 };
                            let v8190: f64;
                            let v10153: Lanes<2>;
                            if v8183 != 0.0 {
                                let v8184 = v8182.sqrt();
                                let v8185 = v1 / v8184;
                                let v18006 = (((v17996 * (v9353 / (v10405 * v8184))) * v8185) * v10360) / v8184;
                                v8190 = v8185;
                                v10153 = v18006;
                            } else {
                                let v8186 = -v8020;
                                let v8187 = v8182.powf(v8186);
                                let v18000 = v17996 * (v8186 * (v8182.powf((v8186 - v9353))));
                                v8190 = v8187;
                                v10153 = v18000;
                            }
                            let v8189 = v8017 * v8163;
                            let v8194 = v1 - v8020;
                            let v8196 = v8188 + ((v8189 * (v1 - (v8182 * v8190))) / v8194);
                            let v18013 = v10150 + (((((v17996 * v8190) + (v10153 * v8182)) * v10360) * v8189) / v8194);
                            v8246 = v8196;
                            v10152 = v18013;
                        } else {
                            v8246 = v8188;
                            v10152 = v10150;
                        }
                        v8245 = v8246;
                        v10149 = v10152;
                    } else {
                        let v8202 = ((v8099 * v7984) / v7981) + ((v8163 * v8020) / v8017);
                        let v8205 = (v8099 + v8163) + ((v7879 * v13) * v8202);
                        let v8206 = v7879 * v8205;
                        let v17976 = (v9371 * v8205) + (((v9371 * v13) * v8202) * v7879);
                        v8245 = v8206;
                        v10149 = v17976;
                    }
                    v8242 = v8245;
                    v10141 = v10149;
                }
                let v8207 = if v7979 > v0 { 1.0 } else { 0.0 };
                let v8678: f64;
                let v10154: Lanes<2>;
                if v8207 != 0.0 {
                    let v8212 = -(((v8208 * v477) * v7969) * v7966);
                    let v8213 = v529 * v8212;
                    let v18076 = (v10128 * v10360) * v10360;
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
                    let v18077 = v18076 * v8221;
                    let v8229 = ((v8221 * v8221) + v8226).sqrt();
                    let v8234 = (v8212 - (v13 * (v8221 + v8229))) * v8233;
                    let v18085 = (((v18076 + ((v18077 + v18077) * (v9353 / (v10405 * v8229)))) * v13) * v10360) * v8233;
                    v8678 = v8234;
                    v10154 = v18085;
                } else {
                    v8678 = v8214;
                    v10154 = v10128;
                }
                let v8235 = if v8099 > v0 { 1.0 } else { 0.0 };
                let v8676: f64;
                let v10155: Lanes<2>;
                if v8235 != 0.0 {
                    let v8240 = -(((v8236 * v477) * v7969) * v7964);
                    let v8241 = v529 * v8240;
                    let v18087 = (v10141 * v10360) * v10360;
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
                    let v18088 = v18087 * v8249;
                    let v8257 = ((v8249 * v8249) + v8254).sqrt();
                    let v8262 = (v8240 - (v13 * (v8249 + v8257))) * v8261;
                    let v18096 = (((v18087 + ((v18088 + v18088) * (v9353 / (v10405 * v8257)))) * v13) * v10360) * v8261;
                    v8676 = v8262;
                    v10155 = v18096;
                } else {
                    v8676 = v8242;
                    v10155 = v10141;
                }
                v8673 = v7962;
                v8674 = v7960;
                v8675 = v8676;
                v8677 = v8678;
                v10122 = v17868;
                v10123 = v17865;
                v10124 = v10155;
                v10125 = v10154;
            } else {
                v8673 = v0;
                v8674 = v0;
                v8675 = v0;
                v8677 = v0;
                v10122 = v17740;
                v10123 = v17741;
                v10124 = v10350;
                v10125 = v10351;
            }
            let v8984: f64;
            let v8989: f64;
            let v10156: Lanes<6>;
            let v10157: Lanes<4>;
            if v71 != 0.0 {
                let v8985: f64;
                let v10158: Lanes<6>;
                if v5714 != 0.0 {
                    let v8266 = v8263 * v8264;
                    let v8267 = v8266 * v8265;
                    let v8271 = v8264 * v8265;
                    let v8274 = (((v5780 * v4843) * v8263) + (v8271 * v8265)) + v362;
                    let v8275 = (v8267 * v8265) / v8274;
                    let v18112 = ((((v9766 * v8266) * v8265) + (v9766 * v8267)) - (((((v9764 * v4843) + (v9437 * v5780)) * v8263) + (((v9766 * v8264) * v8265) + (v9766 * v8271))) * v8275)) / v8274;
                    v8985 = v8275;
                    v10158 = v18112;
                } else {
                    let v8276 = v8263 + v362;
                    v8985 = v8276;
                    v10158 = v11032;
                }
                let v8278 = v8277 * v1128;
                let v18113 = v9406 * v8277;
                v8984 = v8985;
                v8989 = v8278;
                v10156 = v10158;
                v10157 = v18113;
            } else {
                v8984 = v0;
                v8989 = v0;
                v10156 = v11032;
                v10157 = v10595;
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
            let v10159: Lanes<6>;
            let v10160: Lanes<6>;
            if v8297 != 0.0 {
                let v8308 = (v8298 - v4340) / v8265;
                let v8311 = (v8309 * v8308) / v4388;
                let v18121 = ((v9769 * v8308) + ((((v10023 - v9426) - (v9766 * v8308)) / v8265) * v8309)) / v4388;
                let v8316 = if (if v8312 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v8314 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8324: f64;
                let v10161: Lanes<6>;
                if v8316 != 0.0 {
                    v8324 = v1;
                    v10161 = v11032;
                } else {
                    let v8321 = if (if v8317 <= v4548 { 1.0 } else { 0.0 }) != 0.0 && (if v4548 <= v8319 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8325: f64;
                    let v10162: Lanes<6>;
                    if v8321 != 0.0 {
                        v8325 = v8311;
                        v10162 = v18121;
                    } else {
                        let v8322 = v4548 - v1;
                        let v8323 = v8311.powf(v8322);
                        let v18125 = v18121 * (v8322 * (v8311.powf((v8322 - v9353))));
                        v8325 = v8323;
                        v10162 = v18125;
                    }
                    v8324 = v8325;
                    v10161 = v10162;
                }
                let v18128 = (v18121 * v8324) + (v10161 * v8311);
                let v8327 = v1 + (v8311 * v8324);
                let v8330 = (v8328 / v4548) - v1;
                let v8331 = v8327.powf(v8330);
                let v8332 = v8327 * v8331;
                let v8333 = v8309 * v8332;
                let v18138 = (v9769 * v8332) + (((v18128 * v8331) + ((v18128 * (v8330 * (v8327.powf((v8330 - v9353))))) * v8327)) * v8309);
                let v8335 = (v5780 + v8333) / v78;
                let v18140 = (v9764 + v18138) / v78;
                let v8336 = v4307 * v4307;
                let v18141 = v9422 * v4307;
                let v18142 = v18141 + v18141;
                let v8337 = v166 * v1128;
                let v8338 = v8337 * v4843;
                let v18144 = (v9406 * v166) * v4843;
                let v8339 = v8338 * v5780;
                let v8340 = v96 * v4307;
                let v18151 = v9422 * v96;
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
                let v18193 = (((((((Lanes([v18144[0], v18144[1], 0.0, v18144[2], v18144[3], 0.0])) + (v9437 * v8337)) * v5780) + (v9764 * v8338)) * v8357) + ((((((((v18151 + (v18142 * v646)) * v8333) + (v18138 * v8343)) * v8333) + (v18138 * v8344)) + ((((((v9422 * v90) + (v18142 * v96)) * v8333) + (v18138 * v8349)) * v5780) + (v9764 * v8350))) + (((((v18151 + v18142) * v5780) + (v9764 * v8354)) * v5780) + (v9764 * v8355))) * v8339)) - ((((((((v9766 * v8359) * v8361) + (v9422 * v8360)) * v8335) + (v18140 * v8362)) * v8335) + (v18140 * v8363)) * v8365)) / v8364;
                v8401 = v8365;
                v8729 = v8333;
                v10159 = v18193;
                v10160 = v18138;
            } else {
                v8401 = v0;
                v8729 = v0;
                v10159 = v11032;
                v10160 = v11032;
            }
            let v8373 = if (if (if (if v4838 != v0 { 1.0 } else { 0.0 }) != 0.0 && v8296 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v8368 == v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v8281 != 0.0 { 1.0 } else { 0.0 };
            let v8721: f64;
            let v8734: f64;
            let v8743: f64;
            let v8747: f64;
            let v10163: Lanes<6>;
            let v10164: Lanes<6>;
            let v10165: Lanes<6>;
            let v10166: Lanes<6>;
            if v8373 != 0.0 {
                let v8376 = v8374.sqrt();
                let v18196 = v9770 * (v9353 / (v10405 * v8376));
                let v8377 = v4843 + v8376;
                let v18197 = v9437 + v18196;
                let v18198 = v9771 * v8378;
                let v18200 = v9770 * v8374;
                let v8383 = v8382 * v8378;
                let v8388 = v821 * v8376;
                let v8389 = v8388 * v4843;
                let v8390 = v8378 + v8374;
                let v8392 = ((v8383 * v8374) + (v90 * ((v8378 * v8378) + (v8374 * v8374)))) + (v8389 * v8390);
                let v18217 = ((((v9771 * v8382) * v8374) + (v9770 * v8383)) + (((v18198 + v18198) + (v18200 + v18200)) * v90)) + (((((v18196 * v821) * v4843) + (v9437 * v8388)) * v8390) + ((v9771 + v9770) * v8389));
                let v8393 = v8377 * v8377;
                let v18218 = v18197 * v8377;
                let v8394 = v8393 * v8393;
                let v18220 = (v18218 + v18218) * v8393;
                let v8395 = v8394 * v8377;
                let v8396 = v8392 / v8395;
                let v18227 = (v18217 - ((((v18220 + v18220) * v8377) + (v18197 * v8394)) * v8396)) / v8395;
                let v8397 = v166 / v8265;
                let v8398 = v8397 * v5780;
                let v8399 = v8398 * v1128;
                let v18235 = v9406 * v8398;
                let v18237 = ((((((v9766 * v8397) * v10360) / v8265) * v5780) + (v9764 * v8397)) * v1128) + (Lanes([v18235[0], v18235[1], 0.0, v18235[2], v18235[3], 0.0]));
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
                let v18272 = ((((v9772 * v8407) * v8406) + (((v9771 + (((v9437 * v90) * v8376) + (v18196 * v8403))) + v9770) * v8410)) - ((((v18197 * v646) * v8416) + ((((((((((v10159 - (((v18237 * v4843) + (v9437 * v8399)) * v8402)) / v8400) * v8377) + (v18197 * v8402)) * v4843) + (v9437 * v8413)) * v8392) + (v18217 * v8414)) * (v9353 / (v10405 * v8416))) * v8412)) * v8418)) / v8417;
                v8721 = v8399;
                v8734 = v8376;
                v8743 = v8396;
                v8747 = v8418;
                v10163 = v18237;
                v10164 = v18196;
                v10165 = v18227;
                v10166 = v18272;
            } else {
                v8721 = v11;
                v8734 = v0;
                v8743 = v0;
                v8747 = v0;
                v10163 = v11032;
                v10164 = v11032;
                v10165 = v11032;
                v10166 = v11032;
            }
            let v8420 = v5620 + v8419;
            let v18273 = v9816 + v9872;
            let v8615: f64;
            let v8616: f64;
            let v8618: f64;
            let v10167: Lanes<6>;
            let v10168: Lanes<6>;
            let v10169: Lanes<4>;
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
                let v18286 = v10532 - (Lanes([v9400[0], v9400[1], 0.0, v9400[2]]));
                let v8440 = v8435 * ((v1 + (v8436 / v122)).ln());
                let v8441 = v8440 * v145;
                let v8444 = v8441 * (v146 + v8442);
                let v8447 = v8441 * (v146 + v8445);
                let v18290 = (v9397 - (Lanes([v9395[0], v9395[1], 0.0]))) * v8444;
                let v18291 = v9397 * v8447;
                let v8452 = (v8440 * v570) * v145;
                let v8457 = v8454 + (v8444 * (v830 - v823));
                let v18294 = v10028 + (Lanes([v18290[0], v18290[1], 0.0, v18290[2], 0.0, 0.0]));
                let v8461 = v8458 + (v8447 * v830);
                let v18296 = v10029 + (Lanes([v18291[0], v18291[1], 0.0, v18291[2], 0.0, 0.0]));
                let v8462 = (v8432 * v8433) + (v8452 * v8433);
                let v18297 = (v18286 * v8432) + (v18286 * v8452);
                v8615 = v8457;
                v8616 = v8461;
                v8618 = v8462;
                v10167 = v18294;
                v10168 = v18296;
                v10169 = v18297;
            } else {
                let v8619: f64;
                let v10170: Lanes<4>;
                if v368 != 0.0 {
                    let v8465 = -((-v8428) * v142);
                    let v8467 = v8465 * (v830 - v878);
                    let v18276 = (v10532 - (Lanes([v9400[0], v9400[1], 0.0, v9400[2]]))) * v8465;
                    v8619 = v8467;
                    v10170 = v18276;
                } else {
                    v8619 = v0;
                    v10170 = v10595;
                }
                let v8474 = ((v8468 * v146) * v145) * ((v1 + (v8436 / v122)).ln());
                let v18279 = (v9397 - (Lanes([v9395[0], v9395[1], 0.0]))) * v8474;
                let v18280 = v9397 * v8474;
                let v8478 = v8454 + (v8474 * (v830 - v823));
                let v18282 = v10028 + (Lanes([v18279[0], v18279[1], 0.0, v18279[2], 0.0, 0.0]));
                let v8479 = v8458 + (v8474 * v830);
                let v18284 = v10029 + (Lanes([v18280[0], v18280[1], 0.0, v18280[2], 0.0, 0.0]));
                v8615 = v8478;
                v8616 = v8479;
                v8618 = v8619;
                v10167 = v18282;
                v10168 = v18284;
                v10169 = v10170;
            }
            let v8613: f64;
            let v8637: f64;
            let v8649: f64;
            let v8993: f64;
            let v8999: f64;
            let v9007: f64;
            let v9031: f64;
            let v9038: f64;
            let v10171: Lanes<6>;
            let v10172: Lanes<6>;
            let v10173: Lanes<6>;
            let v10174: Lanes<6>;
            let v10175: Lanes<6>;
            let v10176: Lanes<6>;
            let v10177: Lanes<6>;
            if v71 != 0.0 {
                let v8994: f64;
                let v9000: f64;
                let v9008: f64;
                let v9032: f64;
                let v9039: f64;
                let v10178: Lanes<6>;
                let v10179: Lanes<6>;
                let v10180: Lanes<6>;
                let v10181: Lanes<6>;
                if v565 != 0.0 {
                    v8994 = v13;
                    v9000 = v8285;
                    v9008 = v8480;
                    v9032 = v0;
                    v9039 = v0;
                    v10178 = v9767;
                    v10179 = v9773;
                    v10180 = v11032;
                    v10181 = v11032;
                } else {
                    let v8493 = v8488 + v8489;
                    let v18308 = v9775 + v9776;
                    let v8499 = (v8285 - v8488) + v8495;
                    let v18310 = (v9767 - v9775) + v9777;
                    v8994 = v0;
                    v9000 = v0;
                    v9008 = v8484;
                    v9032 = v8493;
                    v9039 = v8499;
                    v10178 = v11032;
                    v10179 = v9774;
                    v10180 = v18308;
                    v10181 = v18310;
                }
                v8613 = v0;
                v8637 = v0;
                v8649 = v0;
                v8993 = v8994;
                v8999 = v9000;
                v9007 = v9008;
                v9031 = v9032;
                v9038 = v9039;
                v10171 = v11032;
                v10172 = v11032;
                v10173 = v11032;
                v10174 = v10178;
                v10175 = v10179;
                v10176 = v10180;
                v10177 = v10181;
            } else {
                let v8614: f64;
                let v8638: f64;
                let v8650: f64;
                let v10182: Lanes<6>;
                let v10183: Lanes<6>;
                let v10184: Lanes<6>;
                if v565 != 0.0 {
                    let v8501 = (-v8480) - v8285;
                    let v18306 = (v9773 * v10360) - v9767;
                    let v8502 = v8285 - v8488;
                    let v18307 = v9767 - v9775;
                    v8614 = v8501;
                    v8638 = v8488;
                    v8650 = v8502;
                    v10182 = v18306;
                    v10183 = v9775;
                    v10184 = v18307;
                } else {
                    let v8506 = (((-v8484) - v8285) - v8495) - v8489;
                    let v18301 = (((v9774 * v10360) - v9767) - v9777) - v9776;
                    let v8507 = v8488 + v8489;
                    let v18302 = v9775 + v9776;
                    let v8509 = (v8285 - v8488) + v8495;
                    let v18304 = (v9767 - v9775) + v9777;
                    v8614 = v8506;
                    v8638 = v8507;
                    v8650 = v8509;
                    v10182 = v18301;
                    v10183 = v18302;
                    v10184 = v18304;
                }
                v8613 = v8614;
                v8637 = v8638;
                v8649 = v8650;
                v8993 = v0;
                v8999 = v0;
                v9007 = v0;
                v9031 = v0;
                v9038 = v0;
                v10171 = v10182;
                v10172 = v10183;
                v10173 = v10184;
                v10174 = v11032;
                v10175 = v11032;
                v10176 = v11032;
                v10177 = v11032;
            }
            let v8510 = if v6875 == v0 { 1.0 } else { 0.0 };
            let v8535: f64;
            let v10185: Lanes<6>;
            if v8510 != 0.0 {
                v8535 = v0;
                v10185 = v11032;
            } else {
                let v8515 = (v8511 * v136) + v4340;
                let v18312 = (v10024 * v136) + v9426;
                let v8516 = if v8515 > v8298 { 1.0 } else { 0.0 };
                let v8520: f64;
                let v10186: Lanes<6>;
                if v8516 != 0.0 {
                    v8520 = v8298;
                    v10186 = v10023;
                } else {
                    v8520 = v8515;
                    v10186 = v18312;
                }
                let v8517 = v823 + v4340;
                let v18314 = (Lanes([v9395[0], v9395[1], 0.0, 0.0, 0.0, 0.0])) + v9426;
                let v8519 = v1 - v4356;
                let v8529 = (v123 * v168) * (((v8523 / v490).sqrt()) * v8526);
                let v8533 = (((v8517 - ((v4356 * v8517) + (v8519 * v8520))) / v6875) - v8511) * v8529;
                let v18321 = (((v18314 - ((v18314 * v4356) + (v10186 * v8519))) / v6875) - v10024) * v8529;
                v8535 = v8533;
                v10185 = v18321;
            }
            let v8534 = if v338 != v0 { 1.0 } else { 0.0 };
            let v8621: f64;
            let v10187: Lanes<6>;
            if v8534 != 0.0 {
                let v18322 = v9400 * v342;
                let v8537 = v8535 + (v342 * v878);
                let v18324 = v10185 + (Lanes([v18322[0], v18322[1], 0.0, 0.0, v18322[2], 0.0]));
                v8621 = v8537;
                v10187 = v18324;
            } else {
                v8621 = v8535;
                v10187 = v10185;
            }
            let v8538 = if v566 == v1 { 1.0 } else { 0.0 };
            let v8708: f64;
            let v9013: f64;
            let v9021: f64;
            let v9052: f64;
            let v9058: f64;
            let v10188: Lanes<6>;
            let v10189: Lanes<6>;
            let v10190: Lanes<6>;
            let v10191: Lanes<6>;
            let v10192: Lanes<6>;
            if v8538 != 0.0 {
                let v8709: f64;
                let v9014: f64;
                let v9022: f64;
                let v9053: f64;
                let v9059: f64;
                let v10193: Lanes<6>;
                let v10194: Lanes<6>;
                let v10195: Lanes<6>;
                let v10196: Lanes<6>;
                let v10197: Lanes<6>;
                if v565 != 0.0 {
                    let v18344 = (v9899 * v10360) - v9900;
                    let v8578 = (((-v8539) - v8547) - v8555) - v8567;
                    let v18347 = ((Lanes([v18344[0], v18344[1], v18344[2], v18344[3], v18344[4], 0.0])) - v9901) - v9902;
                    let v8612 = v8596 + v8603;
                    let v18351 = (Lanes([v9905[0], v9905[1], v9905[2], v9905[3], v9905[4], 0.0])) + v9906;
                    let v8636 = v8613 + ((((((v8615 + v8616) + v8618) - v8621) - v8623) - v8629) + v8578);
                    let v18361 = v10171 + ((((((v10167 + v10168) + (Lanes([v10169[0], v10169[1], 0.0, v10169[2], v10169[3], 0.0]))) - v10187) - (Lanes([v10030[0], v10030[1], v10030[2], v10030[3], v10030[4], 0.0]))) - (Lanes([v10031[0], v10031[1], v10031[2], v10031[3], v10031[4], 0.0]))) + v18347);
                    let v8648 = v8637 + ((((-v8615) + v8621) + v8641) + (v8579 + v8586));
                    let v18367 = v10172 + ((((v10167 * v10360) + v10187) + (Lanes([v10032[0], v10032[1], v10032[2], v10032[3], v10032[4], 0.0]))) + ((Lanes([v9903[0], v9903[1], v9903[2], v9903[3], v9903[4], 0.0])) + v9904));
                    let v8659 = v8649 + (((-v8616) + v8652) + v8612);
                    let v18372 = v10173 + (((v10168 * v10360) + (Lanes([v10033[0], v10033[1], v10033[2], v10033[3], v10033[4], 0.0]))) + v18351);
                    v8709 = v8636;
                    v9014 = v8612;
                    v9022 = v8578;
                    v9053 = v8648;
                    v9059 = v8659;
                    v10193 = v18361;
                    v10194 = v18351;
                    v10195 = v18347;
                    v10196 = v18367;
                    v10197 = v18372;
                } else {
                    let v8665 = v8613 + (((((v8615 + v8616) + v8618) - v8621) - v8623) - v8629);
                    let v18333 = v10171 + (((((v10167 + v10168) + (Lanes([v10169[0], v10169[1], 0.0, v10169[2], v10169[3], 0.0]))) - v10187) - (Lanes([v10030[0], v10030[1], v10030[2], v10030[3], v10030[4], 0.0]))) - (Lanes([v10031[0], v10031[1], v10031[2], v10031[3], v10031[4], 0.0])));
                    let v8669 = v8637 + (((-v8615) + v8621) + v8641);
                    let v18338 = v10172 + (((v10167 * v10360) + v10187) + (Lanes([v10032[0], v10032[1], v10032[2], v10032[3], v10032[4], 0.0])));
                    let v8672 = v8649 + ((-v8616) + v8652);
                    let v18342 = v10173 + ((v10168 * v10360) + (Lanes([v10033[0], v10033[1], v10033[2], v10033[3], v10033[4], 0.0])));
                    v8709 = v8665;
                    v9014 = v0;
                    v9022 = v0;
                    v9053 = v8669;
                    v9059 = v8672;
                    v10193 = v18333;
                    v10194 = v11032;
                    v10195 = v11032;
                    v10196 = v18338;
                    v10197 = v18342;
                }
                v8708 = v8709;
                v9013 = v9014;
                v9021 = v9022;
                v9052 = v9053;
                v9058 = v9059;
                v10188 = v10193;
                v10189 = v10194;
                v10190 = v10195;
                v10191 = v10196;
                v10192 = v10197;
            } else {
                v8708 = v8613;
                v9013 = v0;
                v9021 = v0;
                v9052 = v8637;
                v9058 = v8649;
                v10188 = v10171;
                v10189 = v11032;
                v10190 = v11032;
                v10191 = v10172;
                v10192 = v10173;
            }
            let v9079: f64;
            let v9080: f64;
            let v9081: f64;
            let v9082: f64;
            let v10198: Lanes<3>;
            let v10199: Lanes<2>;
            let v10200: Lanes<3>;
            let v10201: Lanes<2>;
            if v565 != 0.0 {
                v9079 = v8674;
                v9080 = v8675;
                v9081 = v8673;
                v9082 = v8677;
                v10198 = v10123;
                v10199 = v10124;
                v10200 = v10122;
                v10201 = v10125;
            } else {
                v9079 = v0;
                v9080 = v0;
                v9081 = v0;
                v9082 = v0;
                v10198 = v17741;
                v10199 = v10350;
                v10200 = v17740;
                v10201 = v10351;
            }
            let v8679 = if v1886 != v1 { 1.0 } else { 0.0 };
            let v9047: f64;
            let v10202: Lanes<6>;
            if v8679 != 0.0 {
                v9047 = v0;
                v10202 = v11032;
            } else {
                v9047 = v5640;
                v10202 = v9858;
            }
            let v8682 = -v8680;
            let v18373 = v9879 * v10360;
            let v8683 = if v7825 == v1 { 1.0 } else { 0.0 };
            let v9077: f64;
            let v10203: Lanes<6>;
            if v8683 != 0.0 {
                let v8691 = (v8684 * v8685) - v8689;
                let v18379 = (v9880 * v8684) - (Lanes([v9881[0], v9881[1], 0.0, v9881[2], 0.0, 0.0]));
                v9077 = v8691;
                v10203 = v18379;
            } else {
                let v8692 = v1 - v8684;
                let v8696 = (v8692 * v8685) - v8694;
                let v18376 = (v9880 * v8692) - (Lanes([v9882[0], v9882[1], 0.0, v9882[2], 0.0, 0.0]));
                v9077 = v8696;
                v10203 = v18376;
            }
            let v9078: f64;
            let v10204: Lanes<6>;
            if v8683 != 0.0 {
                let v8697 = v1 - v8684;
                let v8699 = (v8697 * v8685) - v8694;
                let v18385 = (v9880 * v8697) - (Lanes([v9882[0], v9882[1], 0.0, v9882[2], 0.0, 0.0]));
                v9078 = v8699;
                v10204 = v18385;
            } else {
                let v8701 = (v8684 * v8685) - v8689;
                let v18382 = (v9880 * v8684) - (Lanes([v9881[0], v9881[1], 0.0, v9881[2], 0.0, 0.0]));
                v9078 = v8701;
                v10204 = v18382;
            }
            let v8706: f64;
            let v10205: Lanes<5>;
            if v8683 != 0.0 {
                v8706 = v8702;
                v10205 = v9891;
            } else {
                v8706 = v8704;
                v10205 = v9895;
            }
            let v8707: f64;
            let v10206: Lanes<5>;
            if v8683 != 0.0 {
                v8707 = v8704;
                v10206 = v9895;
            } else {
                v8707 = v8702;
                v10206 = v9891;
            }
            let v8710 = v365 * (v10188[0]);
            let v8711 = v365 * (v10188[1]);
            let v8712 = if v7825 > v0 { 1.0 } else { 0.0 };
            let v8713: f64;
            if v8712 != 0.0 {
                v8713 = v8711;
            } else {
                v8713 = v8710;
            }
            let v9121: f64;
            let v9123: f64;
            let v10207: Lanes<6>;
            let v10208: Lanes<6>;
            if v8373 != 0.0 {
                let v8716 = ((v23 * v1128) * v168) * v139;
                let v18390 = ((v10385 * v8717) * v8713) * v8713;
                let v8722 = (((v8717 * v665) * v8713) * v8713) / v8721;
                let v18394 = ((Lanes([0.0, 0.0, v18390[0], 0.0, 0.0, 0.0])) - (v10163 * v8722)) / v8721;
                let v8727 = if (if v8408 > v8723 { 1.0 } else { 0.0 }) != 0.0 && (if v823 > v8725 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8745: f64;
                let v10209: Lanes<6>;
                if v8727 != 0.0 {
                    let v8728 = v8309 / v5780;
                    let v18400 = (v9769 - (v9764 * v8728)) / v5780;
                    let v8730 = v8309 / v8729;
                    let v8732 = (v8730 - v8728) / v823;
                    let v18405 = v9395 * v8732;
                    let v8733 = v4274 * v8732;
                    let v8737 = (v8378 + (v4843 * v8734)) + v8374;
                    let v8739 = v4843 + v8734;
                    let v8740 = (v8733 * v8737) / v8739;
                    let v8741 = v8728 + v8740;
                    let v18422 = v18400 + ((((((((((v9769 - (v10160 * v8730)) / v8729) - v18400) - (Lanes([v18405[0], v18405[1], 0.0, 0.0, 0.0, 0.0]))) / v823) * v4274) * v8737) + (((v9771 + ((v9437 * v8734) + (v10164 * v4843))) + v9770) * v8733)) - ((v9437 + v10164) * v8740)) / v8739);
                    v8745 = v8741;
                    v10209 = v18422;
                } else {
                    let v8742 = v8309 / v8729;
                    let v18397 = (v9769 - (v10160 * v8742)) / v8729;
                    v8745 = v8742;
                    v10209 = v18397;
                }
                let v8744 = v8722 * v8743;
                let v8746 = v8744 * v8745;
                let v18428 = (((v18394 * v8743) + (v10165 * v8722)) * v8745) + (v10209 * v8744);
                let v8749 = if (-v8713) > v8716 { 1.0 } else { 0.0 };
                let v8751 = if v8749 != 0.0 && (if v8746 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8752: f64;
                let v10210: Lanes<6>;
                if v8751 != 0.0 {
                    v8752 = v8746;
                    v10210 = v18428;
                } else {
                    v8752 = v0;
                    v10210 = v11032;
                }
                let v8753: f64;
                let v10211: Lanes<6>;
                if v8749 != 0.0 {
                    v8753 = v8747;
                    v10211 = v10166;
                } else {
                    v8753 = v0;
                    v10211 = v11032;
                }
                v9121 = v8753;
                v9123 = v8752;
                v10207 = v10211;
                v10208 = v10210;
            } else {
                v9121 = v0;
                v9123 = v0;
                v10207 = v11032;
                v10208 = v11032;
            }
            let v8755 = if v8754 == v1 { 1.0 } else { 0.0 };
            let v9046: f64;
            let v10212: Lanes<5>;
            if v8755 != 0.0 {
                let v8785: f64;
                let v8787: f64;
                let v8796: f64;
                let v8819: f64;
                let v8820: f64;
                let v8868: f64;
                let v8874: f64;
                let v10213: Lanes<4>;
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
                    let v18438 = ((Lanes([0.0, v9355[0]])) - (Lanes([v9359[0], 0.0]))) * v365;
                    let v18439 = Lanes([0.0, v18438[0], 0.0, v18438[1]]);
                    v8785 = v8759;
                    v8787 = v8760;
                    v8796 = v8761;
                    v8819 = v8769;
                    v8820 = v8767;
                    v8868 = v8758;
                    v8874 = v8766;
                    v10213 = v18439;
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
                    let v18433 = ((Lanes([v9358[0], 0.0])) - (Lanes([0.0, v9354[0]]))) * v365;
                    let v18434 = Lanes([v18433[0], 0.0, v18433[1], 0.0]);
                    v8785 = v8770;
                    v8787 = v8771;
                    v8796 = v8772;
                    v8819 = v8779;
                    v8820 = v8777;
                    v8868 = v39;
                    v8874 = v8776;
                    v10213 = v18434;
                }
                let v8784 = ((v8780 * v8780) + (v134 * v134)).sqrt();
                let v8790 = v699.powf(v8789);
                let v8791 = (v8785 / v556) / v8790;
                let v8794 = v713 - (v8792 * v714);
                let v8795 = (v8787 / v68) / v8794;
                let v18452 = v9382 * v8797;
                let v8799 = v8796 + (v8797 * v653);
                let v8804 = v1 + (v8800 / (v143.powf(v8801)));
                let v8809 = v1 + (v8805 / (v143.powf(v8806)));
                let v8814 = v1 + (v8810 / (v169.powf(v8811)));
                let v8815 = v8791 * v8804;
                let v18453 = ((((v10386 * (v8789 * (v699.powf((v8789 - v9353))))) * v8791) * v10360) / v8790) * v8804;
                let v18455 = (((((v10398 - (v10399 * v8792)) * v8795) * v10360) / v8794) * v8814) * v8809;
                let v8818 = ((v8795 * v8814) * v8809) + v362;
                let v8821 = v8819 / v8820;
                let v8822 = v8815 * v8821;
                let v18457 = v18453 * v8821;
                let v18458 = (v10213 / v8820) * v8815;
                let v18461 = (Lanes([0.0, 0.0, 0.0, 0.0, v18457[0]])) + (Lanes([v18458[0], v18458[1], v18458[2], v18458[3], 0.0]));
                let v8823 = if v8819 >= v0 { 1.0 } else { 0.0 };
                let v8837: f64;
                let v10214: Lanes<5>;
                if v8823 != 0.0 {
                    let v8824 = v8822 / v8818;
                    let v18467 = v18455 * v8824;
                    let v18470 = (v18461 - (Lanes([0.0, 0.0, 0.0, 0.0, v18467[0]]))) / v8818;
                    v8837 = v8824;
                    v10214 = v18470;
                } else {
                    let v8826 = (-v8822) / v8818;
                    let v18463 = v18455 * v8826;
                    let v18466 = ((v18461 * v10360) - (Lanes([0.0, 0.0, 0.0, 0.0, v18463[0]]))) / v8818;
                    v8837 = v8826;
                    v10214 = v18466;
                }
                let v8831 = if (if v8827 <= v8799 { 1.0 } else { 0.0 }) != 0.0 && (if v8799 <= v8829 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8840: f64;
                let v10215: Lanes<5>;
                if v8831 != 0.0 {
                    v8840 = v1;
                    v10215 = v18429;
                } else {
                    let v8836 = if (if v8832 <= v8799 { 1.0 } else { 0.0 }) != 0.0 && (if v8799 <= v8834 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8841: f64;
                    let v10216: Lanes<5>;
                    if v8836 != 0.0 {
                        v8841 = v8837;
                        v10216 = v10214;
                    } else {
                        let v8838 = v8799 - v1;
                        let v8839 = v8837.powf(v8838);
                        let v18477 = v18452 * (v8839 * (v8837.ln()));
                        let v18479 = (v10214 * (v8838 * (v8837.powf((v8838 - v9353))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18477[0]]));
                        v8841 = v8839;
                        v10216 = v18479;
                    }
                    v8840 = v8841;
                    v10215 = v10216;
                }
                let v18482 = (v10214 * v8840) + (v10215 * v8837);
                let v8843 = v1 + (v8837 * v8840);
                let v8848 = if (if v8844 <= v8799 { 1.0 } else { 0.0 }) != 0.0 && (if v8799 <= v8846 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8862: f64;
                let v10217: Lanes<5>;
                if v8848 != 0.0 {
                    let v8849 = v1 / v8843;
                    let v18506 = ((v18482 * v8849) * v10360) / v8843;
                    v8862 = v8849;
                    v10217 = v18506;
                } else {
                    let v8854 = if (if v8850 <= v8799 { 1.0 } else { 0.0 }) != 0.0 && (if v8799 <= v8852 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8863: f64;
                    let v10218: Lanes<5>;
                    if v8854 != 0.0 {
                        let v8855 = v8843.sqrt();
                        let v8856 = v1 / v8855;
                        let v18503 = (((v18482 * (v9353 / (v10405 * v8855))) * v8856) * v10360) / v8855;
                        v8863 = v8856;
                        v10218 = v18503;
                    } else {
                        let v8858 = v8857 / v8799;
                        let v8859 = v8858 - v1;
                        let v8860 = v8843.powf(v8859);
                        let v18492 = (((v18452 * v8858) * v10360) / v8799) * (v8860 * (v8843.ln()));
                        let v8861 = v8843 * v8860;
                        let v18497 = (v18482 * v8860) + (((v18482 * (v8859 * (v8843.powf((v8859 - v9353))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18492[0]]))) * v8843);
                        v8863 = v8861;
                        v10218 = v18497;
                    }
                    v8862 = v8863;
                    v10217 = v10218;
                }
                let v18507 = v18453 * v8862;
                let v8866 = (v206 / v8820) * v8784;
                let v8869 = (v8866 * (v8815 * v8862)) * v8868;
                let v18512 = (((Lanes([0.0, 0.0, 0.0, 0.0, v18507[0]])) + (v10217 * v8815)) * v8866) * v8868;
                let v8870 = if v8869 <= v0 { 1.0 } else { 0.0 };
                let v8871: f64;
                let v10219: Lanes<5>;
                if v8870 != 0.0 {
                    v8871 = v362;
                    v10219 = v18429;
                } else {
                    v8871 = v8869;
                    v10219 = v18512;
                }
                let v8872 = v1 / v8871;
                let v18516 = (((v10219 * v8872) * v10360) / v8871) / v166;
                let v8875 = (v8872 / v166) + v8874;
                let v8877 = if (if v8875 > v29 { 1.0 } else { 0.0 }) != 0.0 && v8296 != 0.0 { 1.0 } else { 0.0 };
                if v8877 != 0.0 {
                } else {
                }
                let v8878 = if v8875 < v29 { 1.0 } else { 0.0 };
                let v8879: f64;
                let v10220: Lanes<5>;
                if v8878 != 0.0 {
                    v8879 = v29;
                    v10220 = v18429;
                } else {
                    v8879 = v8875;
                    v10220 = v18516;
                }
                v9046 = v8879;
                v10212 = v10220;
            } else {
                v9046 = v0;
                v10212 = v18429;
            }
            let v8881 = if v8880 == v1 { 1.0 } else { 0.0 };
            let v9045: f64;
            let v10221: Lanes<5>;
            if v8881 != 0.0 {
                let v8898: f64;
                let v8900: f64;
                let v8907: f64;
                let v8923: f64;
                let v8924: f64;
                let v8972: f64;
                let v8978: f64;
                let v10222: Lanes<4>;
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
                    let v18525 = ((Lanes([0.0, v9355[0]])) - (Lanes([v9359[0], 0.0]))) * v365;
                    let v18526 = Lanes([0.0, v18525[0], 0.0, v18525[1]]);
                    v8898 = v8759;
                    v8900 = v8760;
                    v8907 = v8761;
                    v8923 = v8888;
                    v8924 = v8767;
                    v8972 = v8883;
                    v8978 = v8886;
                    v10222 = v18526;
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
                    let v18520 = ((Lanes([v9358[0], 0.0])) - (Lanes([0.0, v9354[0]]))) * v365;
                    let v18521 = Lanes([v18520[0], 0.0, v18520[1], 0.0]);
                    v8898 = v8770;
                    v8900 = v8771;
                    v8907 = v8772;
                    v8923 = v8893;
                    v8924 = v8777;
                    v8972 = v39;
                    v8978 = v8891;
                    v10222 = v18521;
                }
                let v8897 = ((v8780 * v8780) + (v134 * v134)).sqrt();
                let v8902 = v699.powf(v8789);
                let v8903 = (v8898 / v556) / v8902;
                let v8905 = v713 - (v8792 * v714);
                let v8906 = (v8900 / v68) / v8905;
                let v18539 = v9382 * v8797;
                let v8909 = v8907 + (v8797 * v653);
                let v8912 = v1 + (v8800 / (v143.powf(v8801)));
                let v8915 = v1 + (v8805 / (v143.powf(v8806)));
                let v8918 = v1 + (v8810 / (v169.powf(v8811)));
                let v8919 = v8903 * v8912;
                let v18540 = ((((v10386 * (v8789 * (v699.powf((v8789 - v9353))))) * v8903) * v10360) / v8902) * v8912;
                let v18542 = (((((v10398 - (v10399 * v8792)) * v8906) * v10360) / v8905) * v8918) * v8915;
                let v8922 = ((v8906 * v8918) * v8915) + v362;
                let v8925 = v8923 / v8924;
                let v8926 = v8919 * v8925;
                let v18544 = v18540 * v8925;
                let v18545 = (v10222 / v8924) * v8919;
                let v18548 = (Lanes([0.0, 0.0, 0.0, 0.0, v18544[0]])) + (Lanes([v18545[0], v18545[1], v18545[2], v18545[3], 0.0]));
                let v8927 = if v8923 >= v0 { 1.0 } else { 0.0 };
                let v8941: f64;
                let v10223: Lanes<5>;
                if v8927 != 0.0 {
                    let v8928 = v8926 / v8922;
                    let v18554 = v18542 * v8928;
                    let v18557 = (v18548 - (Lanes([0.0, 0.0, 0.0, 0.0, v18554[0]]))) / v8922;
                    v8941 = v8928;
                    v10223 = v18557;
                } else {
                    let v8930 = (-v8926) / v8922;
                    let v18550 = v18542 * v8930;
                    let v18553 = ((v18548 * v10360) - (Lanes([0.0, 0.0, 0.0, 0.0, v18550[0]]))) / v8922;
                    v8941 = v8930;
                    v10223 = v18553;
                }
                let v8935 = if (if v8931 <= v8909 { 1.0 } else { 0.0 }) != 0.0 && (if v8909 <= v8933 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8944: f64;
                let v10224: Lanes<5>;
                if v8935 != 0.0 {
                    v8944 = v1;
                    v10224 = v18429;
                } else {
                    let v8940 = if (if v8936 <= v8909 { 1.0 } else { 0.0 }) != 0.0 && (if v8909 <= v8938 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8945: f64;
                    let v10225: Lanes<5>;
                    if v8940 != 0.0 {
                        v8945 = v8941;
                        v10225 = v10223;
                    } else {
                        let v8942 = v8909 - v1;
                        let v8943 = v8941.powf(v8942);
                        let v18564 = v18539 * (v8943 * (v8941.ln()));
                        let v18566 = (v10223 * (v8942 * (v8941.powf((v8942 - v9353))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18564[0]]));
                        v8945 = v8943;
                        v10225 = v18566;
                    }
                    v8944 = v8945;
                    v10224 = v10225;
                }
                let v18569 = (v10223 * v8944) + (v10224 * v8941);
                let v8947 = v1 + (v8941 * v8944);
                let v8952 = if (if v8948 <= v8909 { 1.0 } else { 0.0 }) != 0.0 && (if v8909 <= v8950 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8966: f64;
                let v10226: Lanes<5>;
                if v8952 != 0.0 {
                    let v8953 = v1 / v8947;
                    let v18593 = ((v18569 * v8953) * v10360) / v8947;
                    v8966 = v8953;
                    v10226 = v18593;
                } else {
                    let v8958 = if (if v8954 <= v8909 { 1.0 } else { 0.0 }) != 0.0 && (if v8909 <= v8956 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8967: f64;
                    let v10227: Lanes<5>;
                    if v8958 != 0.0 {
                        let v8959 = v8947.sqrt();
                        let v8960 = v1 / v8959;
                        let v18590 = (((v18569 * (v9353 / (v10405 * v8959))) * v8960) * v10360) / v8959;
                        v8967 = v8960;
                        v10227 = v18590;
                    } else {
                        let v8962 = v8961 / v8909;
                        let v8963 = v8962 - v1;
                        let v8964 = v8947.powf(v8963);
                        let v18579 = (((v18539 * v8962) * v10360) / v8909) * (v8964 * (v8947.ln()));
                        let v8965 = v8947 * v8964;
                        let v18584 = (v18569 * v8964) + (((v18569 * (v8963 * (v8947.powf((v8963 - v9353))))) + (Lanes([0.0, 0.0, 0.0, 0.0, v18579[0]]))) * v8947);
                        v8967 = v8965;
                        v10227 = v18584;
                    }
                    v8966 = v8967;
                    v10226 = v10227;
                }
                let v18594 = v18540 * v8966;
                let v8970 = (v206 / v8924) * v8897;
                let v8973 = (v8970 * (v8919 * v8966)) * v8972;
                let v18599 = (((Lanes([0.0, 0.0, 0.0, 0.0, v18594[0]])) + (v10226 * v8919)) * v8970) * v8972;
                let v8974 = if v8973 <= v0 { 1.0 } else { 0.0 };
                let v8975: f64;
                let v10228: Lanes<5>;
                if v8974 != 0.0 {
                    v8975 = v362;
                    v10228 = v18429;
                } else {
                    v8975 = v8973;
                    v10228 = v18599;
                }
                let v8976 = v1 / v8975;
                let v18603 = (((v10228 * v8976) * v10360) / v8975) / v166;
                let v8979 = (v8976 / v166) + v8978;
                let v8981 = if (if v8979 > v29 { 1.0 } else { 0.0 }) != 0.0 && v8296 != 0.0 { 1.0 } else { 0.0 };
                if v8981 != 0.0 {
                } else {
                }
                let v8982 = if v8979 < v29 { 1.0 } else { 0.0 };
                let v8983: f64;
                let v10229: Lanes<5>;
                if v8982 != 0.0 {
                    v8983 = v29;
                    v10229 = v18429;
                } else {
                    v8983 = v8979;
                    v10229 = v18603;
                }
                v9045 = v8983;
                v10221 = v10229;
            } else {
                v9045 = v0;
                v10221 = v18429;
            }
            let v9048: f64;
            let v9054: f64;
            let v9060: f64;
            let v9066: f64;
            let v9195: f64;
            let v9197: f64;
            let v9231: f64;
            let v9233: f64;
            let v10230: Lanes<10>;
            let v10231: Lanes<8>;
            let v10232: Lanes<8>;
            let v10233: Lanes<1>;
            let v10234: Lanes<7>;
            let v10235: Lanes<7>;
            let v10236: Lanes<7>;
            let v10237: Lanes<7>;
            if v565 != 0.0 {
                let v9049: f64;
                let v9055: f64;
                let v9061: f64;
                let v9067: f64;
                let v9196: f64;
                let v9198: f64;
                let v10238: Lanes<8>;
                let v10239: Lanes<7>;
                let v10240: Lanes<7>;
                let v10241: Lanes<1>;
                let v10242: Lanes<7>;
                let v10243: Lanes<7>;
                if v71 != 0.0 {
                    let v8987 = if v8984 < v8986 { 1.0 } else { 0.0 };
                    let v9002: f64;
                    let v10244: Lanes<6>;
                    if v8987 != 0.0 {
                        v9002 = v8988;
                        v10244 = v11032;
                    } else {
                        v9002 = v8984;
                        v10244 = v10156;
                    }
                    let v8991 = if v8989 < v8990 { 1.0 } else { 0.0 };
                    let v9010: f64;
                    let v10245: Lanes<4>;
                    if v8991 != 0.0 {
                        v9010 = v8992;
                        v10245 = v10595;
                    } else {
                        v9010 = v8989;
                        v10245 = v10157;
                    }
                    let v8996: f64;
                    if v8683 != 0.0 {
                        v8996 = v8993;
                    } else {
                        let v8995 = v1 - v8993;
                        v8996 = v8995;
                    }
                    let v9003 = (v8997 - v8999) / v9002;
                    let v18637 = v10244 * v9003;
                    let v18640 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v9373[0]])) - (Lanes([v10174[0], v10174[1], v10174[2], v10174[3], v10174[4], v10174[5], 0.0]))) - (Lanes([v18637[0], v18637[1], v18637[2], v18637[3], v18637[4], v18637[5], 0.0]))) / v9002;
                    let v9011 = (v9004 - v9007) / v9010;
                    let v18644 = v10245 * v9011;
                    let v18647 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9374[0], 0.0])) - (Lanes([v10175[0], v10175[1], v10175[2], v10175[3], v10175[4], 0.0, v10175[5]]))) - (Lanes([v18644[0], v18644[1], 0.0, v18644[2], v18644[3], 0.0, 0.0]))) / v9010;
                    let v18648 = v9373 * v8996;
                    let v9015 = (v8997 * v8996) + v9013;
                    let v18650 = Lanes([v10189[0], v10189[1], v10189[2], v10189[3], v10189[4], v10189[5], 0.0]);
                    let v18651 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v18648[0]])) + v18650;
                    let v9016 = v1 - v8996;
                    let v18652 = v9373 * v9016;
                    let v9018 = (v8997 * v9016) + v9013;
                    let v18654 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v18652[0]])) + v18650;
                    let v18655 = v9373 * v10360;
                    let v18658 = (Lanes([0.0, v18655[0]])) - (Lanes([v9374[0], 0.0]));
                    let v9023 = ((-v8997) - v9004) + v9021;
                    let v18661 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18658[0], 0.0, v18658[1]])) + (Lanes([v10190[0], v10190[1], v10190[2], v10190[3], v10190[4], 0.0, v10190[5], 0.0]));
                    v9049 = v9023;
                    v9055 = v9015;
                    v9061 = v9018;
                    v9067 = v9004;
                    v9196 = v9003;
                    v9198 = v9011;
                    v10238 = v18661;
                    v10239 = v18651;
                    v10240 = v18654;
                    v10241 = v9374;
                    v10242 = v18640;
                    v10243 = v18647;
                } else {
                    v9049 = v0;
                    v9055 = v0;
                    v9061 = v0;
                    v9067 = v0;
                    v9196 = v0;
                    v9198 = v0;
                    v10238 = v18633;
                    v10239 = v18631;
                    v10240 = v18631;
                    v10241 = v10344;
                    v10242 = v18631;
                    v10243 = v18632;
                }
                let v18662 = Lanes([v10238[0], v10238[1], v10238[2], v10238[3], v10238[4], v10238[5], 0.0, 0.0, v10238[6], v10238[7]]);
                let v18663 = Lanes([v10239[0], v10239[1], v10239[2], v10239[3], v10239[4], 0.0, v10239[5], v10239[6]]);
                let v18664 = Lanes([v10240[0], v10240[1], v10240[2], v10240[3], v10240[4], 0.0, v10240[5], v10240[6]]);
                v9048 = v9049;
                v9054 = v9055;
                v9060 = v9061;
                v9066 = v9067;
                v9195 = v9196;
                v9197 = v9198;
                v9231 = v0;
                v9233 = v0;
                v10230 = v18662;
                v10231 = v18663;
                v10232 = v18664;
                v10233 = v10241;
                v10234 = v10242;
                v10235 = v10243;
                v10236 = v18605;
                v10237 = v18606;
            } else {
                let v9050: f64;
                let v9056: f64;
                let v9062: f64;
                let v9068: f64;
                let v9232: f64;
                let v9234: f64;
                let v10246: Lanes<3>;
                let v10247: Lanes<1>;
                let v10248: Lanes<1>;
                let v10249: Lanes<1>;
                let v10250: Lanes<7>;
                let v10251: Lanes<7>;
                if v71 != 0.0 {
                    let v9025 = if v8984 < v9024 { 1.0 } else { 0.0 };
                    let v9034: f64;
                    let v10252: Lanes<6>;
                    if v9025 != 0.0 {
                        v9034 = v9026;
                        v10252 = v11032;
                    } else {
                        v9034 = v8984;
                        v10252 = v10156;
                    }
                    let v9028 = if v8989 < v9027 { 1.0 } else { 0.0 };
                    if v9028 != 0.0 {
                    } else {
                    }
                    let v9035 = (v9029 - v9031) / v9034;
                    let v18610 = v10252 * v9035;
                    let v18613 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9375[0], 0.0])) - (Lanes([v10176[0], v10176[1], v10176[2], v10176[3], v10176[4], 0.0, v10176[5]]))) - (Lanes([v18610[0], v18610[1], v18610[2], v18610[3], v18610[4], 0.0, v18610[5]]))) / v9034;
                    let v9041 = (v9036 - v9038) / v9034;
                    let v18617 = v10252 * v9041;
                    let v18620 = (((Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v9376[0], 0.0])) - (Lanes([v10177[0], v10177[1], v10177[2], v10177[3], v10177[4], 0.0, v10177[5]]))) - (Lanes([v18617[0], v18617[1], v18617[2], v18617[3], v18617[4], 0.0, v18617[5]]))) / v9034;
                    let v18621 = v9375 * v10360;
                    let v18624 = (Lanes([v18621[0], 0.0])) - (Lanes([0.0, v9376[0]]));
                    let v9044 = ((-v9029) - v9036) - v9004;
                    let v18627 = (Lanes([0.0, v18624[0], v18624[1]])) - (Lanes([v9374[0], 0.0, 0.0]));
                    v9050 = v9044;
                    v9056 = v9029;
                    v9062 = v9036;
                    v9068 = v9004;
                    v9232 = v9035;
                    v9234 = v9041;
                    v10246 = v18627;
                    v10247 = v9375;
                    v10248 = v9376;
                    v10249 = v9374;
                    v10250 = v18613;
                    v10251 = v18620;
                } else {
                    v9050 = v0;
                    v9056 = v0;
                    v9062 = v0;
                    v9068 = v0;
                    v9232 = v0;
                    v9234 = v0;
                    v10246 = v18604;
                    v10247 = v10345;
                    v10248 = v10346;
                    v10249 = v10344;
                    v10250 = v18605;
                    v10251 = v18606;
                }
                let v18628 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10246[0], v10246[1], v10246[2], 0.0, 0.0]);
                let v18629 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10247[0], 0.0, 0.0]);
                let v18630 = Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10248[0], 0.0, 0.0]);
                v9048 = v9050;
                v9054 = v9056;
                v9060 = v9062;
                v9066 = v9068;
                v9195 = v0;
                v9197 = v0;
                v9231 = v9232;
                v9233 = v9234;
                v10230 = v18628;
                v10231 = v18629;
                v10232 = v18630;
                v10233 = v10249;
                v10234 = v18631;
                v10235 = v18632;
                v10236 = v10250;
                v10237 = v10251;
            }
            let v9085: f64;
            let v9088: f64;
            let v9089: f64;
            let v9091: f64;
            let v9092: f64;
            let v9093: f64;
            let v10253: Lanes<6>;
            let v10254: Lanes<6>;
            let v10255: Lanes<6>;
            let v10256: Lanes<10>;
            let v10257: Lanes<9>;
            let v10258: Lanes<7>;
            if v8683 != 0.0 {
                let v9051 = v8708 + v9048;
                let v18678 = (Lanes([v10188[0], v10188[1], v10188[2], v10188[3], v10188[4], 0.0, 0.0, 0.0, v10188[5], 0.0])) + v10230;
                let v9057 = v9052 + v9054;
                let v18680 = (Lanes([v10191[0], v10191[1], v10191[2], v10191[3], v10191[4], 0.0, v10191[5], 0.0])) + v10231;
                let v18683 = ((v10188 + v10191) + v10192) * v10360;
                let v9069 = (-((v8708 + v9052) + v9058)) + v9066;
                let v18686 = (Lanes([v18683[0], v18683[1], v18683[2], v18683[3], v18683[4], 0.0, v18683[5]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10233[0], 0.0]));
                let v18687 = Lanes([v18680[0], v18680[1], v18680[2], v18680[3], v18680[4], v18680[5], 0.0, v18680[6], v18680[7]]);
                v9085 = v8420;
                v9088 = v9047;
                v9089 = v0;
                v9091 = v9051;
                v9092 = v9057;
                v9093 = v9069;
                v10253 = v18273;
                v10254 = v10202;
                v10255 = v11032;
                v10256 = v18678;
                v10257 = v18687;
                v10258 = v18686;
            } else {
                let v9070 = -v8420;
                let v18665 = v18273 * v10360;
                let v9071 = v8708 + v9048;
                let v18667 = (Lanes([v10188[0], v10188[1], v10188[2], v10188[3], v10188[4], 0.0, 0.0, 0.0, v10188[5], 0.0])) + v10230;
                let v9072 = v9058 + v9060;
                let v18669 = (Lanes([v10192[0], v10192[1], v10192[2], v10192[3], v10192[4], 0.0, v10192[5], 0.0])) + v10232;
                let v18672 = ((v10188 + v10191) + v10192) * v10360;
                let v9076 = (-((v8708 + v9052) + v9058)) + v9066;
                let v18675 = (Lanes([v18672[0], v18672[1], v18672[2], v18672[3], v18672[4], 0.0, v18672[5]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v10233[0], 0.0]));
                let v18676 = Lanes([v18669[0], v18669[1], v18669[2], v18669[3], v18669[4], 0.0, v18669[5], v18669[6], v18669[7]]);
                v9085 = v9070;
                v9088 = v0;
                v9089 = v9047;
                v9091 = v9071;
                v9092 = v9072;
                v9093 = v9076;
                v10253 = v18665;
                v10254 = v11032;
                v10255 = v10202;
                v10256 = v18667;
                v10257 = v18676;
                v10258 = v18675;
            }
            let v9094: f64;
            let v9095: f64;
            let v9096: f64;
            let v9097: f64;
            let v10259: Lanes<3>;
            let v10260: Lanes<3>;
            let v10261: Lanes<2>;
            let v10262: Lanes<2>;
            if v565 != 0.0 {
                v9094 = v9079;
                v9095 = v9081;
                v9096 = v9080;
                v9097 = v9082;
                v10259 = v10198;
                v10260 = v10200;
                v10261 = v10199;
                v10262 = v10201;
            } else {
                v9094 = v8674;
                v9095 = v8673;
                v9096 = v8675;
                v9097 = v8677;
                v10259 = v10123;
                v10260 = v10122;
                v10261 = v10124;
                v10262 = v10125;
            }
            let v9084 = if (if v631 == v1 { 1.0 } else { 0.0 }) != 0.0 && v633 != 0.0 { 1.0 } else { 0.0 };
            let v9158: f64;
            let v9159: f64;
            let v9163: f64;
            let v10263: Lanes<6>;
            if v9084 != 0.0 {
                let v9086 = v9085 * v823;
                let v18689 = v9395 * v9085;
                let v18691 = (v10253 * v823) + (Lanes([v18689[0], v18689[1], 0.0, 0.0, 0.0, 0.0]));
                let v9087 = v1 / v386;
                v9158 = v9086;
                v9159 = v9087;
                v9163 = v387;
                v10263 = v18691;
            } else {
                v9158 = v0;
                v9159 = v0;
                v9163 = v0;
                v10263 = v11032;
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
            let v18692 = v9382 * v9099;
            let v9101 = v365 * v9085;
            let v18693 = v10253 * v365;
            let v9102 = if v5793 == v1 { 1.0 } else { 0.0 };
            let v9256: f64;
            let v9257: f64;
            let v9258: f64;
            let v10264: Lanes<6>;
            let v10265: Lanes<6>;
            let v10266: Lanes<4>;
            if v9102 != 0.0 {
                let v9103 = v365 * v9078;
                let v18694 = v10204 * v365;
                let v9104 = v365 * v9077;
                let v18695 = v10203 * v365;
                let v9105 = v365 * v8682;
                let v18696 = v18373 * v365;
                v9256 = v9103;
                v9257 = v9104;
                v9258 = v9105;
                v10264 = v18694;
                v10265 = v18695;
                v10266 = v18696;
            } else {
                v9256 = v0;
                v9257 = v0;
                v9258 = v0;
                v10264 = v11032;
                v10265 = v11032;
                v10266 = v10595;
            }
            let v9259: f64;
            let v9260: f64;
            let v10267: Lanes<5>;
            if v8754 != 0.0 {
                let v18699 = (Lanes([0.0, v9355[0]])) - (Lanes([v9359[0], 0.0]));
                let v9107 = (v603 - v613) / v9046;
                let v18703 = ((Lanes([0.0, v18699[0], 0.0, v18699[1], 0.0])) - (v10212 * v9107)) / v9046;
                v9259 = v9107;
                v9260 = v0;
                v10267 = v18703;
            } else {
                v9259 = v0;
                v9260 = v9108;
                v10267 = v18429;
            }
            let v9261: f64;
            let v9262: f64;
            let v10268: Lanes<5>;
            if v8880 != 0.0 {
                let v18706 = (Lanes([v9358[0], 0.0])) - (Lanes([0.0, v9354[0]]));
                let v9110 = (v612 - v602) / v9045;
                let v18710 = ((Lanes([v18706[0], 0.0, v18706[1], 0.0, 0.0])) - (v10221 * v9110)) / v9045;
                v9261 = v9110;
                v9262 = v0;
                v10268 = v18710;
            } else {
                v9261 = v0;
                v9262 = v9111;
                v10268 = v18429;
            }
            let v9113 = v365 * (ddt(73861, v9091));
            let v18713 = (v10256 * v18711) * v365;
            let v9115 = v365 * (ddt(73865, v9092));
            let v18715 = (v10257 * v18711) * v365;
            let v9117 = v365 * (ddt(73869, v9093));
            let v18717 = (v10258 * v18711) * v365;
            let v9120 = v9100 * v8401;
            let v18718 = v18692 * v8401;
            let v18721 = (Lanes([0.0, 0.0, v18718[0], 0.0, 0.0, 0.0])) + (v10159 * v9100);
            let v9125 = if (if v9120 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v9123 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v9128: f64;
            let v10269: Lanes<6>;
            if v9125 != 0.0 {
                let v9126 = v9123 / v9120;
                let v9127 = v9126.sqrt();
                let v18727 = ((v10208 - (v18721 * v9126)) / v9120) * (v9353 / (v10405 * v9127));
                v9128 = v9127;
                v10269 = v18727;
            } else {
                v9128 = v0;
                v10269 = v11032;
            }
            let v9132 = v9121 * v9129;
            let v18728 = v10207 * v9129;
            let v18729 = v9366 * v9121;
            let v18732 = (Lanes([v18728[0], v18728[1], v18728[2], v18728[3], v18728[4], 0.0, v18728[5]])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18729[0], 0.0]));
            let v9136: f64;
            let v10270: Lanes<6>;
            if v8712 != 0.0 {
                let v9133 = v1 - v9118;
                let v9134 = v9128 * v9133;
                let v18739 = (v10269 * v9133) + ((v9778 * v10360) * v9128);
                v9136 = v9134;
                v10270 = v18739;
            } else {
                let v9135 = v9128 * v9118;
                let v18735 = (v10269 * v9118) + (v9778 * v9128);
                v9136 = v9135;
                v10270 = v18735;
            }
            let v9140: f64;
            let v10271: Lanes<6>;
            if v8712 != 0.0 {
                let v9137 = v9128 * v9118;
                let v18746 = (v10269 * v9118) + (v9778 * v9128);
                v9140 = v9137;
                v10271 = v18746;
            } else {
                let v9138 = v1 - v9118;
                let v9139 = v9128 * v9138;
                let v18743 = (v10269 * v9138) + ((v9778 * v10360) * v9128);
                v9140 = v9139;
                v10271 = v18743;
            }
            let v9141 = v9129 * v9136;
            let v18747 = v9366 * v9136;
            let v18748 = v10270 * v9129;
            let v18751 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18747[0], 0.0])) + (Lanes([v18748[0], v18748[1], v18748[2], v18748[3], v18748[4], 0.0, v18748[5]]));
            let v9142 = ddt(73942, v9141);
            let v18752 = v18751 * v18711;
            let v9143 = v9129 * v9140;
            let v18753 = v9366 * v9140;
            let v18754 = v10271 * v9129;
            let v18757 = (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v18753[0], 0.0])) + (Lanes([v18754[0], v18754[1], v18754[2], v18754[3], v18754[4], 0.0, v18754[5]]));
            let v9144 = ddt(73946, v9143);
            let v18758 = v18757 * v18711;
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
            let v10272: Lanes<2>;
            if v539 != 0.0 {
                let v9154 = v9150 * (v9152 - v606);
                let v18763 = ((Lanes([v9367[0], 0.0])) - (Lanes([0.0, v9356[0]]))) * v9150;
                v9268 = v9154;
                v9269 = v0;
                v10272 = v18763;
            } else {
                v9268 = v0;
                v9269 = v9155;
                v10272 = v18759;
            }
            let v9157 = if v632 != 0.0 && (if v34 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v9270: f64;
            let v9271: f64;
            let v9272: f64;
            let v9273: f64;
            let v9274: f64;
            let v10273: Lanes<1>;
            let v10274: Lanes<6>;
            let v10275: Lanes<1>;
            let v10276: Lanes<1>;
            let v10277: Lanes<1>;
            if v9157 != 0.0 {
                let v9160 = v635 * v9159;
                let v18765 = v9364 * v9159;
                let v9161 = -v9158;
                let v18766 = v10263 * v10360;
                let v9162 = v635 * v11;
                let v18767 = v9364 * v11;
                let v9165 = ddt(74007, (v9163 * v635));
                let v18769 = (v9364 * v9163) * v18711;
                v9270 = v9160;
                v9271 = v9161;
                v9272 = v9162;
                v9273 = v9165;
                v9274 = v0;
                v10273 = v18765;
                v10274 = v18766;
                v10275 = v18767;
                v10276 = v18769;
                v10277 = v10359;
            } else {
                let v9166 = v635 * v556;
                let v18764 = v9364 * v556;
                v9270 = v0;
                v9271 = v0;
                v9272 = v0;
                v9273 = v0;
                v9274 = v9166;
                v10273 = v10359;
                v10274 = v11032;
                v10275 = v10359;
                v10276 = v10359;
                v10277 = v18764;
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
            let v10278: Lanes<6>;
            let v10279: Lanes<6>;
            let v10280: Lanes<3>;
            let v10281: Lanes<3>;
            let v10282: Lanes<2>;
            let v10283: Lanes<2>;
            let v10284: Lanes<2>;
            let v10285: Lanes<7>;
            let v10286: Lanes<7>;
            let v10287: Lanes<1>;
            let v10288: Lanes<1>;
            let v10289: Lanes<1>;
            let v10290: Lanes<1>;
            let v10291: Lanes<6>;
            let v10292: Lanes<1>;
            let v10293: Lanes<1>;
            let v10294: Lanes<6>;
            let v10295: Lanes<6>;
            let v10296: Lanes<6>;
            let v10297: Lanes<1>;
            let v10298: Lanes<1>;
            let v10299: Lanes<7>;
            let v10300: Lanes<7>;
            let v10301: Lanes<7>;
            let v10302: Lanes<1>;
            let v10303: Lanes<1>;
            let v10304: Lanes<1>;
            let v10305: Lanes<1>;
            let v10306: Lanes<1>;
            let v10307: Lanes<1>;
            if v565 != 0.0 {
                let v9168 = v365 * (v8706 + v9088);
                let v18793 = ((Lanes([v10205[0], v10205[1], v10205[2], v10205[3], v10205[4], 0.0])) + v10254) * v365;
                let v9170 = v365 * (v8707 + v9089);
                let v18796 = ((Lanes([v10206[0], v10206[1], v10206[2], v10206[3], v10206[4], 0.0])) + v10255) * v365;
                let v18797 = v10262 * v18711;
                let v9173 = v365 * (v9095 + (ddt(74027, v9097)));
                let v18800 = (v10260 + (Lanes([v18797[0], 0.0, v18797[1]]))) * v365;
                let v18801 = v10261 * v18711;
                let v9176 = v365 * (v9094 + (ddt(74033, v9096)));
                let v18804 = (v10259 + (Lanes([v18801[0], 0.0, v18801[1]]))) * v365;
                let v9280: f64;
                let v9282: f64;
                let v10308: Lanes<2>;
                if v545 != 0.0 {
                    let v9181 = (v9177 - v609) / v9179;
                    let v18808 = ((Lanes([v9368[0], 0.0])) - (Lanes([0.0, v9357[0]]))) / v9179;
                    v9280 = v9181;
                    v9282 = v0;
                    v10308 = v18808;
                } else {
                    v9280 = v0;
                    v9282 = v9182;
                    v10308 = v18788;
                }
                let v9284: f64;
                let v9286: f64;
                let v9288: f64;
                let v9290: f64;
                let v10309: Lanes<2>;
                let v10310: Lanes<2>;
                if v552 != 0.0 {
                    let v9187 = v9183 * (v9185 - v609);
                    let v18812 = ((Lanes([v9369[0], 0.0])) - (Lanes([0.0, v9357[0]]))) * v9183;
                    let v9192 = v9188 * (v9190 - v609);
                    let v18816 = ((Lanes([v9370[0], 0.0])) - (Lanes([0.0, v9357[0]]))) * v9188;
                    v9284 = v9187;
                    v9286 = v9192;
                    v9288 = v0;
                    v9290 = v0;
                    v10309 = v18812;
                    v10310 = v18816;
                } else {
                    v9284 = v0;
                    v9286 = v0;
                    v9288 = v9193;
                    v9290 = v9194;
                    v10309 = v18789;
                    v10310 = v18790;
                }
                let v9292: f64;
                let v9294: f64;
                let v9296: f64;
                let v9298: f64;
                let v9300: f64;
                let v9302: f64;
                let v9304: f64;
                let v9306: f64;
                let v10311: Lanes<7>;
                let v10312: Lanes<7>;
                let v10313: Lanes<1>;
                let v10314: Lanes<1>;
                let v10315: Lanes<1>;
                let v10316: Lanes<1>;
                if v71 != 0.0 {
                    let v9199 = v618 * v11;
                    let v18817 = v9360 * v11;
                    let v9200 = v621 * v11;
                    let v18818 = v9361 * v11;
                    let v9203 = ddt(74064, (v9201 * v618));
                    let v18820 = (v9360 * v9201) * v18711;
                    let v9206 = ddt(74070, (v9204 * v621));
                    let v18822 = (v9361 * v9204) * v18711;
                    v9292 = v9195;
                    v9294 = v9197;
                    v9296 = v9199;
                    v9298 = v9200;
                    v9300 = v9203;
                    v9302 = v9206;
                    v9304 = v0;
                    v9306 = v0;
                    v10311 = v10234;
                    v10312 = v10235;
                    v10313 = v18817;
                    v10314 = v18818;
                    v10315 = v18820;
                    v10316 = v18822;
                } else {
                    v9292 = v0;
                    v9294 = v0;
                    v9296 = v0;
                    v9298 = v0;
                    v9300 = v0;
                    v9302 = v0;
                    v9304 = v9207;
                    v9306 = v9208;
                    v10311 = v18631;
                    v10312 = v18632;
                    v10313 = v10352;
                    v10314 = v10344;
                    v10315 = v10352;
                    v10316 = v10344;
                }
                let v9209 = if v2247 != 0.0 || v5625 != 0.0 { 1.0 } else { 0.0 };
                let v9308: f64;
                let v9310: f64;
                let v9312: f64;
                let v9314: f64;
                let v10317: Lanes<6>;
                let v10318: Lanes<1>;
                let v10319: Lanes<1>;
                if v9209 != 0.0 {
                    let v9216 = v2254 * v11;
                    let v18823 = v9365 * v11;
                    let v9219 = ddt(74091, (v9217 * v2254));
                    let v18825 = (v9365 * v9217) * v18711;
                    v9308 = v9210;
                    v9310 = v9216;
                    v9312 = v9219;
                    v9314 = v0;
                    v10317 = v9869;
                    v10318 = v18823;
                    v10319 = v18825;
                } else {
                    v9308 = v0;
                    v9310 = v0;
                    v9312 = v0;
                    v9314 = v9220;
                    v10317 = v11032;
                    v10318 = v11006;
                    v10319 = v11006;
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
                v10278 = v18793;
                v10279 = v18796;
                v10280 = v18800;
                v10281 = v18804;
                v10282 = v10308;
                v10283 = v10309;
                v10284 = v10310;
                v10285 = v10311;
                v10286 = v10312;
                v10287 = v10313;
                v10288 = v10314;
                v10289 = v10315;
                v10290 = v10316;
                v10291 = v10317;
                v10292 = v10318;
                v10293 = v10319;
                v10294 = v11032;
                v10295 = v11032;
                v10296 = v11032;
                v10297 = v11006;
                v10298 = v11006;
                v10299 = v18605;
                v10300 = v18606;
                v10301 = v18632;
                v10302 = v10345;
                v10303 = v10346;
                v10304 = v10344;
                v10305 = v10345;
                v10306 = v10346;
                v10307 = v10344;
            } else {
                let v9222 = v365 * (v8706 + v9088);
                let v18772 = ((Lanes([v10205[0], v10205[1], v10205[2], v10205[3], v10205[4], 0.0])) + v10254) * v365;
                let v9224 = v365 * (v8707 + v9089);
                let v18775 = ((Lanes([v10206[0], v10206[1], v10206[2], v10206[3], v10206[4], 0.0])) + v10255) * v365;
                let v9319: f64;
                let v9321: f64;
                let v9323: f64;
                let v9325: f64;
                let v10320: Lanes<6>;
                let v10321: Lanes<1>;
                let v10322: Lanes<1>;
                if v2247 != 0.0 {
                    let v9226 = v2254 * v11;
                    let v18776 = v9365 * v11;
                    let v9229 = ddt(74114, (v9227 * v2254));
                    let v18778 = (v9365 * v9227) * v18711;
                    v9319 = v9210;
                    v9321 = v9226;
                    v9323 = v9229;
                    v9325 = v0;
                    v10320 = v9869;
                    v10321 = v18776;
                    v10322 = v18778;
                } else {
                    v9319 = v0;
                    v9321 = v0;
                    v9323 = v0;
                    v9325 = v9230;
                    v10320 = v11032;
                    v10321 = v11006;
                    v10322 = v11006;
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
                let v10323: Lanes<7>;
                let v10324: Lanes<7>;
                let v10325: Lanes<7>;
                let v10326: Lanes<1>;
                let v10327: Lanes<1>;
                let v10328: Lanes<1>;
                let v10329: Lanes<1>;
                let v10330: Lanes<1>;
                let v10331: Lanes<1>;
                if v71 != 0.0 {
                    let v9235 = v624 * v11;
                    let v18779 = v9362 * v11;
                    let v9236 = v627 * v11;
                    let v18780 = v9363 * v11;
                    let v9237 = v621 * v11;
                    let v18781 = v9361 * v11;
                    let v9240 = ddt(74134, (v9238 * v624));
                    let v18783 = (v9362 * v9238) * v18711;
                    let v9243 = ddt(74140, (v9241 * v627));
                    let v18785 = (v9363 * v9241) * v18711;
                    let v9246 = ddt(74146, (v9244 * v621));
                    let v18787 = (v9361 * v9244) * v18711;
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
                    v10323 = v10236;
                    v10324 = v10237;
                    v10325 = v10235;
                    v10326 = v18779;
                    v10327 = v18780;
                    v10328 = v18781;
                    v10329 = v18783;
                    v10330 = v18785;
                    v10331 = v18787;
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
                    v10323 = v18605;
                    v10324 = v18606;
                    v10325 = v18632;
                    v10326 = v10345;
                    v10327 = v10346;
                    v10328 = v10344;
                    v10329 = v10345;
                    v10330 = v10346;
                    v10331 = v10344;
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
                v10278 = v11032;
                v10279 = v11032;
                v10280 = v17740;
                v10281 = v17741;
                v10282 = v18788;
                v10283 = v18789;
                v10284 = v18790;
                v10285 = v18631;
                v10286 = v18632;
                v10287 = v10352;
                v10288 = v10344;
                v10289 = v10352;
                v10290 = v10344;
                v10291 = v11032;
                v10292 = v11006;
                v10293 = v11006;
                v10294 = v18772;
                v10295 = v18775;
                v10296 = v10320;
                v10297 = v10321;
                v10298 = v10322;
                v10299 = v10323;
                v10300 = v10324;
                v10301 = v10325;
                v10302 = v10326;
                v10303 = v10327;
                v10304 = v10328;
                v10305 = v10329;
                v10306 = v10330;
                v10307 = v10331;
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
            let v19242 = v18693[0];
            let v19243 = v18693[1];
            let v19244 = v18693[2];
            let v19245 = v18693[3];
            let v19246 = v18693[4];
            let v19247 = v18693[5];
            let v19248 = v10264[0];
            let v19249 = v10264[1];
            let v19250 = v10264[2];
            let v19251 = v10264[3];
            let v19252 = v10264[4];
            let v19253 = v10264[5];
            let v19254 = v10265[0];
            let v19255 = v10265[1];
            let v19256 = v10265[2];
            let v19257 = v10265[3];
            let v19258 = v10265[4];
            let v19259 = v10265[5];
            let v19260 = v10266[0];
            let v19261 = v10266[1];
            let v19262 = v10266[2];
            let v19263 = v10266[3];
            let v19264 = v10267[0];
            let v19265 = v10267[1];
            let v19266 = v10267[2];
            let v19267 = v10267[3];
            let v19268 = v10267[4];
            let v19269 = v10268[0];
            let v19270 = v10268[1];
            let v19271 = v10268[2];
            let v19272 = v10268[3];
            let v19273 = v10268[4];
            let v19274 = v18713[0];
            let v19275 = v18713[1];
            let v19276 = v18713[2];
            let v19277 = v18713[3];
            let v19278 = v18713[4];
            let v19279 = v18713[5];
            let v19280 = v18713[6];
            let v19281 = v18713[7];
            let v19282 = v18713[8];
            let v19283 = v18713[9];
            let v19284 = v18715[0];
            let v19285 = v18715[1];
            let v19286 = v18715[2];
            let v19287 = v18715[3];
            let v19288 = v18715[4];
            let v19289 = v18715[5];
            let v19290 = v18715[6];
            let v19291 = v18715[7];
            let v19292 = v18715[8];
            let v19293 = v18717[0];
            let v19294 = v18717[1];
            let v19295 = v18717[2];
            let v19296 = v18717[3];
            let v19297 = v18717[4];
            let v19298 = v18717[5];
            let v19299 = v18717[6];
            let v19300 = v9366[0];
            let v19301 = v18732[0];
            let v19302 = v18732[1];
            let v19303 = v18732[2];
            let v19304 = v18732[3];
            let v19305 = v18732[4];
            let v19306 = v18732[5];
            let v19307 = v18732[6];
            let v19308 = v18752[0];
            let v19309 = v18752[1];
            let v19310 = v18752[2];
            let v19311 = v18752[3];
            let v19312 = v18752[4];
            let v19313 = v18752[5];
            let v19314 = v18752[6];
            let v19315 = v18758[0];
            let v19316 = v18758[1];
            let v19317 = v18758[2];
            let v19318 = v18758[3];
            let v19319 = v18758[4];
            let v19320 = v18758[5];
            let v19321 = v18758[6];
            let v19322 = v10272[0];
            let v19323 = v10272[1];
            let v19324 = v10273[0];
            let v19325 = v10274[0];
            let v19326 = v10274[1];
            let v19327 = v10274[2];
            let v19328 = v10274[3];
            let v19329 = v10274[4];
            let v19330 = v10274[5];
            let v19331 = v10275[0];
            let v19332 = v10276[0];
            let v19333 = v10277[0];
            let v19334 = v10278[0];
            let v19335 = v10278[1];
            let v19336 = v10278[2];
            let v19337 = v10278[3];
            let v19338 = v10278[4];
            let v19339 = v10278[5];
            let v19340 = v10279[0];
            let v19341 = v10279[1];
            let v19342 = v10279[2];
            let v19343 = v10279[3];
            let v19344 = v10279[4];
            let v19345 = v10279[5];
            let v19346 = v10280[0];
            let v19347 = v10280[1];
            let v19348 = v10280[2];
            let v19349 = v10281[0];
            let v19350 = v10281[1];
            let v19351 = v10281[2];
            let v19352 = v10282[0];
            let v19353 = v10282[1];
            let v19354 = v10283[0];
            let v19355 = v10283[1];
            let v19356 = v10284[0];
            let v19357 = v10284[1];
            let v19358 = v10285[0];
            let v19359 = v10285[1];
            let v19360 = v10285[2];
            let v19361 = v10285[3];
            let v19362 = v10285[4];
            let v19363 = v10285[5];
            let v19364 = v10285[6];
            let v19365 = v10286[0];
            let v19366 = v10286[1];
            let v19367 = v10286[2];
            let v19368 = v10286[3];
            let v19369 = v10286[4];
            let v19370 = v10286[5];
            let v19371 = v10286[6];
            let v19372 = v10287[0];
            let v19373 = v10288[0];
            let v19374 = v10289[0];
            let v19375 = v10290[0];
            let v19376 = v10291[0];
            let v19377 = v10291[1];
            let v19378 = v10291[2];
            let v19379 = v10291[3];
            let v19380 = v10291[4];
            let v19381 = v10291[5];
            let v19382 = v10292[0];
            let v19383 = v10293[0];
            let v19384 = v10294[0];
            let v19385 = v10294[1];
            let v19386 = v10294[2];
            let v19387 = v10294[3];
            let v19388 = v10294[4];
            let v19389 = v10294[5];
            let v19390 = v10295[0];
            let v19391 = v10295[1];
            let v19392 = v10295[2];
            let v19393 = v10295[3];
            let v19394 = v10295[4];
            let v19395 = v10295[5];
            let v19396 = v10296[0];
            let v19397 = v10296[1];
            let v19398 = v10296[2];
            let v19399 = v10296[3];
            let v19400 = v10296[4];
            let v19401 = v10296[5];
            let v19402 = v10297[0];
            let v19403 = v10298[0];
            let v19404 = v10299[0];
            let v19405 = v10299[1];
            let v19406 = v10299[2];
            let v19407 = v10299[3];
            let v19408 = v10299[4];
            let v19409 = v10299[5];
            let v19410 = v10299[6];
            let v19411 = v10300[0];
            let v19412 = v10300[1];
            let v19413 = v10300[2];
            let v19414 = v10300[3];
            let v19415 = v10300[4];
            let v19416 = v10300[5];
            let v19417 = v10300[6];
            let v19418 = v10301[0];
            let v19419 = v10301[1];
            let v19420 = v10301[2];
            let v19421 = v10301[3];
            let v19422 = v10301[4];
            let v19423 = v10301[5];
            let v19424 = v10301[6];
            let v19425 = v10302[0];
            let v19426 = v10303[0];
            let v19427 = v10304[0];
            let v19428 = v10305[0];
            let v19429 = v10306[0];
            let v19430 = v10307[0];
            let v19431 = v18751[0];
            let v19432 = v18751[1];
            let v19433 = v18751[2];
            let v19434 = v18751[3];
            let v19435 = v18751[4];
            let v19436 = v18751[5];
            let v19437 = v18751[6];
            let v19438 = v18757[0];
            let v19439 = v18757[1];
            let v19440 = v18757[2];
            let v19441 = v18757[3];
            let v19442 = v18757[4];
            let v19443 = v18757[5];
            let v19444 = v18757[6];
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
            [v19242, v19243, v19244, v19245, v19246, v19247],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (v9256),
            [6, 7, 10, 11, 12, 17],
            [v19248, v19249, v19250, v19251, v19252, v19253],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(6),
            multiplicity * (v9257),
            [6, 7, 10, 11, 12, 17],
            [v19254, v19255, v19256, v19257, v19258, v19259],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(11),
            Some(12),
            multiplicity * (v9258),
            [6, 7, 11, 12],
            [v19260, v19261, v19262, v19263],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(2),
            multiplicity * (v9259),
            [0, 2, 6, 7, 10],
            [v19264, v19265, v19266, v19267, v19268],
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
            [v19269, v19270, v19271, v19272, v19273],
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
            [v19274, v19275, v19276, v19277, v19278, v19279, v19280, v19281, v19282, v19283],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (v9115),
            [6, 7, 10, 11, 12, 15, 16, 17, 18],
            [v19284, v19285, v19286, v19287, v19288, v19289, v19290, v19291, v19292],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(12),
            Some(7),
            multiplicity * (v9117),
            [6, 7, 10, 11, 12, 13, 17],
            [v19293, v19294, v19295, v19296, v19297, v19298, v19299],
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
            [v19300],
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
            [v19301, v19302, v19303, v19304, v19305, v19306, v19307],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(7),
            multiplicity * (v9142),
            [6, 7, 10, 11, 12, 14, 17],
            [v19308, v19309, v19310, v19311, v19312, v19313, v19314],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(6),
            multiplicity * (v9144),
            [6, 7, 10, 11, 12, 14, 17],
            [v19315, v19316, v19317, v19318, v19319, v19320, v19321],
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
            [v19322, v19323],
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
            [v19324],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            None,
            multiplicity * (v9271),
            [6, 7, 10, 11, 12, 17],
            [v19325, v19326, v19327, v19328, v19329, v19330],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9272),
            [10],
            [v19331],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9273),
            [10],
            [v19332],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v9274),
            [10],
            [v19333],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(12),
            multiplicity * (v9275),
            [6, 7, 10, 11, 12, 17],
            [v19334, v19335, v19336, v19337, v19338, v19339],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(12),
            multiplicity * (v9276),
            [6, 7, 10, 11, 12, 17],
            [v19340, v19341, v19342, v19343, v19344, v19345],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(9),
            Some(7),
            multiplicity * (v9277),
            [7, 10, 12],
            [v19346, v19347, v19348],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(6),
            multiplicity * (v9278),
            [6, 10, 12],
            [v19349, v19350, v19351],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(4),
            Some(12),
            multiplicity * (v9279),
            [4, 12],
            [v19352, v19353],
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
            [v19354, v19355],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(8),
            Some(12),
            multiplicity * (v9285),
            [8, 12],
            [v19356, v19357],
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
            [v19358, v19359, v19360, v19361, v19362, v19363, v19364],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(13),
            None,
            multiplicity * (v9293),
            [6, 7, 10, 11, 12, 13, 17],
            [v19365, v19366, v19367, v19368, v19369, v19370, v19371],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(18),
            None,
            multiplicity * (v9295),
            [18],
            [v19372],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9297),
            [13],
            [v19373],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(18),
            None,
            multiplicity * (v9299),
            [18],
            [v19374],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9301),
            [13],
            [v19375],
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
            [v19376, v19377, v19378, v19379, v19380, v19381],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9309),
            [17],
            [v19382],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9311),
            [17],
            [v19383],
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
            [v19384, v19385, v19386, v19387, v19388, v19389],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(6),
            multiplicity * (v9316),
            [6, 7, 10, 11, 12, 17],
            [v19390, v19391, v19392, v19393, v19394, v19395],
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
            [v19396, v19397, v19398, v19399, v19400, v19401],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9320),
            [17],
            [v19402],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(17),
            None,
            multiplicity * (v9322),
            [17],
            [v19403],
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
            [v19404, v19405, v19406, v19407, v19408, v19409, v19410],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(16),
            None,
            multiplicity * (v9328),
            [6, 7, 10, 11, 12, 16, 17],
            [v19411, v19412, v19413, v19414, v19415, v19416, v19417],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(13),
            None,
            multiplicity * (v9330),
            [6, 7, 10, 11, 12, 13, 17],
            [v19418, v19419, v19420, v19421, v19422, v19423, v19424],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (v9332),
            [15],
            [v19425],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(16),
            None,
            multiplicity * (v9334),
            [16],
            [v19426],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9336),
            [13],
            [v19427],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(15),
            None,
            multiplicity * (v9338),
            [15],
            [v19428],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(16),
            None,
            multiplicity * (v9340),
            [16],
            [v19429],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v9342),
            [13],
            [v19430],
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
        self.canonical_reactive[20] = v19431;
        self.canonical_reactive[21] = v19432;
        self.canonical_reactive[22] = v19433;
        self.canonical_reactive[23] = v19434;
        self.canonical_reactive[24] = v19435;
        self.canonical_reactive[25] = v19436;
        self.canonical_reactive[26] = v19437;
        self.canonical_reactive[27] = v9143;
        self.canonical_reactive[28] = v19438;
        self.canonical_reactive[29] = v19439;
        self.canonical_reactive[30] = v19440;
        self.canonical_reactive[31] = v19441;
        self.canonical_reactive[32] = v19442;
        self.canonical_reactive[33] = v19443;
        self.canonical_reactive[34] = v19444;
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
        self.canonical_reactive[45] = v9273;
        self.canonical_reactive[46] = v9274;
        self.canonical_reactive[47] = v9275;
        self.canonical_reactive[48] = v9276;
        self.canonical_reactive[49] = v9277;
        self.canonical_reactive[50] = v9278;
        self.canonical_reactive[51] = v9279;
        self.canonical_reactive[52] = v9281;
        self.canonical_reactive[53] = v9283;
        self.canonical_reactive[54] = v9285;
        self.canonical_reactive[55] = v9287;
        self.canonical_reactive[56] = v9289;
        self.canonical_reactive[57] = v9291;
        self.canonical_reactive[58] = v9293;
        self.canonical_reactive[59] = v9295;
        self.canonical_reactive[60] = v9297;
        self.canonical_reactive[61] = v9299;
        self.canonical_reactive[62] = v9301;
        self.canonical_reactive[63] = v9303;
        self.canonical_reactive[64] = v9305;
        self.canonical_reactive[65] = v9307;
        self.canonical_reactive[66] = v9309;
        self.canonical_reactive[67] = v9311;
        self.canonical_reactive[68] = v9313;
        self.canonical_reactive[69] = v9315;
        self.canonical_reactive[70] = v9316;
        self.canonical_reactive[71] = v9317;
        self.canonical_reactive[72] = v9318;
        self.canonical_reactive[73] = v9320;
        self.canonical_reactive[74] = v9322;
        self.canonical_reactive[75] = v9324;
        self.canonical_reactive[76] = v9326;
        self.canonical_reactive[77] = v9328;
        self.canonical_reactive[78] = v9330;
        self.canonical_reactive[79] = v9332;
        self.canonical_reactive[80] = v9334;
        self.canonical_reactive[81] = v9336;
        self.canonical_reactive[82] = v9338;
        self.canonical_reactive[83] = v9340;
        self.canonical_reactive[84] = v9342;
        self.canonical_reactive[85] = v9344;
        self.canonical_reactive[86] = v9346;
        self.canonical_reactive[87] = v9348;
        self.canonical_reactive[88] = v9350;
        self.canonical_reactive[89] = v9351;
        self.canonical_reactive[90] = v9352;
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
    }

}

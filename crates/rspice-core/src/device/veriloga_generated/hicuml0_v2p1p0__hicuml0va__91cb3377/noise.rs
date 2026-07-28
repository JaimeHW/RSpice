#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::GeneratedEvalContext;
pub use crate::device::veriloga_generated::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};


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
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 6] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RB", label: Some("rb"), kind: GeneratedNoiseKind::White, equation: 26, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_C_RCX", label: Some("rcx"), kind: GeneratedNoiseKind::White, equation: 27, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_EI_E_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 28, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 29, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBE", label: Some("ibe"), kind: GeneratedNoiseKind::White, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("it"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3])];
            let v0 = 0e0f64;
            let v1 = parameters[110];
            let v2 = node_potentials[1];
            let v3 = node_potentials[5];
            let v6 = node_potentials[6];
            let v9 = node_potentials[7];
            let v13 = node_potentials[3];
            let v16 = node_potentials[2];
            let v20 = parameters[108];
            let v21 = 2.7315e2f64;
            let v23 = temperature;
            let v24 = 1.3806226e-23f64;
            let v26 = 1.602176462e-19f64;
            let v28 = parameters[88];
            let v30 = 5e-1f64;
            let v31 = parameters[76];
            let v32 = parameters[77];
            let v35 = parameters[78];
            let v38 = parameters[79];
            let v41 = 3e0f64;
            let v42 = parameters[80];
            let v46 = 1e0f64;
            let v48 = parameters[87];
            let v50 = 1.5e0f64;
            let v52 = parameters[82];
            let v53 = parameters[81];
            let v55 = parameters[34];
            let v56 = parameters[21];
            let v58 = parameters[41];
            let v61 = parameters[109];
            let v63 = 1e2f64;
            let v64 = 1.7314999999999998e2f64;
            let v66 = 1.7314999999999998e2f64;
            let v67 = 6e2f64;
            let v69 = 6e2f64;
            let v80 = parameters[35];
            let v83 = 2e0f64;
            let v99 = 4e0f64;
            let v111 = parameters[36];
            let v117 = parameters[37];
            let v120 = parameters[38];
            let v143 = parameters[39];
            let v149 = parameters[40];
            let v152 = parameters[15];
            let v158 = parameters[17];
            let v166 = parameters[42];
            let v190 = parameters[43];
            let v196 = parameters[19];
            let v202 = parameters[1];
            let v208 = parameters[9];
            let v209 = parameters[95];
            let v211 = parameters[83];
            let v216 = parameters[62];
            let v221 = parameters[61];
            let v226 = parameters[64];
            let v227 = parameters[89];
            let v231 = parameters[65];
            let v233 = parameters[90];
            let v237 = parameters[54];
            let v238 = parameters[85];
            let v241 = parameters[86];
            let v246 = parameters[96];
            let v250 = parameters[99];
            let v254 = parameters[22];
            let v255 = parameters[100];
            let v259 = parameters[23];
            let v260 = parameters[91];
            let v264 = parameters[46];
            let v287 = parameters[45];
            let v288 = parameters[47];
            let v294 = parameters[51];
            let v318 = parameters[50];
            let v319 = parameters[52];
            let v325 = parameters[32];
            let v327 = parameters[30];
            let v331 = parameters[7];
            let v332 = parameters[97];
            let v336 = parameters[6];
            let v338 = parameters[84];
            let v345 = parameters[0];
            let v346 = 2e2f64;
            let v348 = parameters[101];
            let v349 = parameters[102];
            let v354 = parameters[98];
            let v357 = parameters[12];
            let v360 = parameters[13];
            let v365 = parameters[14];
            let v366 = parameters[29];
            let v367 = parameters[93];
            let v371 = parameters[26];
            let v372 = parameters[92];
            let v376 = parameters[28];
            let v377 = parameters[94];
            let v381 = parameters[104];
            let v382 = parameters[103];
            let v384 = parameters[111];
            let v387 = node_potentials[4];
            let v389 = 1.7314999999999998e2f64;
            let v391 = 1.7314999999999998e2f64;
            let v392 = 6e2f64;
            let v394 = 6e2f64;
            let v600 = 1e-30f64;
            let v603 = parameters[49];
            let v607 = parameters[44];
            let v613 = 2.4e0f64;
            let v614 = -8.754687373538999e-1f64;
            let v629 = 8e1f64;
            let v637 = 1e-1f64;
            let v687 = -8.754687373538999e-1f64;
            let v695 = 1.921812e0f64;
            let v716 = parameters[48];
            let v721 = -8.754687373538999e-1f64;
            let v747 = -8.754687373538999e-1f64;
            let v816 = -8.754687373538999e-1f64;
            let v848 = -8.754687373538999e-1f64;
            let v917 = -8.754687373538999e-1f64;
            let v948 = -8.754687373538999e-1f64;
            let v997 = parameters[67];
            let v1007 = parameters[63];
            let v1010 = parameters[66];
            let v1091 = parameters[8];
            let v1113 = 1e-3f64;
            let v1128 = parameters[5];
            let v1131 = 2e1f64;
            let v1134 = 2.5e-2f64;
            let v1143 = parameters[55];
            let v1148 = parameters[56];
            let v1153 = parameters[10];
            let v1160 = parameters[11];
            let v1161 = parameters[3];
            let v1173 = parameters[4];
            let v1189 = 6.666e-1f64;
            let v1219 = 1e-8f64;
            let v1231 = 1e-2f64;
            let v1235 = 2.004987562112089e0f64;
            let v1237 = parameters[2];
            let v1266 = 3.333333333333333e-1f64;
            let v1267 = -2e0f64;
            let v1269 = 1e6f64;
            let v1292 = 2.7e1f64;
            let v1299 = 2.5e-1f64;
            let v1306 = 1e-10f64;
            let v1343 = -2.7e1f64;
            let v1349 = 1.5707963267948966e0f64;
            let v1355 = 1.5707963267948966e0f64;
            let v1361 = -4e0f64;
            let v1374 = 1e-20f64;
            let v1383 = parameters[16];
            let v1397 = parameters[18];
            let v1414 = parameters[20];
            let v1432 = -8.754687373538999e-1f64;
            let v1498 = -8.754687373538999e-1f64;
            let v1557 = parameters[24];
            let v1560 = parameters[25];
            let v1574 = parameters[27];
            let v1578 = 1e-6f64;
            let v1593 = parameters[31];
            let v1603 = parameters[33];
            let v1607 = parameters[53];
            let v1613 = -8.754687373538999e-1f64;
            let v1638 = parameters[73];
            let v1642 = node_potentials[9];
            let v1643 = parameters[70];
            let v1645 = parameters[69];
            let v1677 = parameters[107];
            let v1682 = 5.5224904e-23f64;
            let v1685 = 0e0f64;
            let v1689 = parameters[74];
            let v1691 = parameters[75];
            let v1694 = 3.204352924e-19f64;
            let v1700 = branch_unknown_flows[2];
            let v5 = v1 * (v2 - v3);
            let v8 = v1 * (v6 - v3);
            let v11 = v1 * (v6 - v9);
            let v12 = v11 - v8;
            let v15 = v1 * (v13 - v3);
            let v18 = v1 * (v2 - v16);
            let v19 = v2 - v6;
            let v22 = v20 + v21;
            let v27 = (v24 * v22) / v26;
            let v29 = v28 * v22;
            let v34 = v30 * (v31 + v32);
            let v37 = v30 * (v31 + v35);
            let v40 = v30 * (v38 + v35);
            let v45 = v41 - ((v26 * v42) / v24);
            let v49 = (v45 + v46) - v48;
            let v51 = v45 - v50;
            let v54 = v31 - v32;
            let v60 = if (if v56 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v58 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v248: f64;
            if v60 != 0.0 {
                v248 = v46;
            } else {
                v248 = v0;
            }
            let v62 = v23 + v61;
            let v65 = if v62 < v64 { 1.0 } else { 0.0 };
            let v70: f64;
            if v65 != 0.0 {
                v70 = v66;
            } else {
                let v68 = if v62 > v67 { 1.0 } else { 0.0 };
                let v71: f64;
                if v68 != 0.0 {
                    v71 = v69;
                } else {
                    v71 = v62;
                }
                v70 = v71;
            }
            let v73 = (v24 * v70) / v26;
            let v74 = v46 / v73;
            let v75 = v70 - v22;
            let v76 = v70 / v22;
            let v77 = v76.ln();
            let v79 = v74 * (v76 - v46);
            let v82 = (v30 * v80) / v27;
            let v84 = v83 * v27;
            let v90 = v84 * (((v82.exp()) - ((-v82).exp())).ln());
            let v92 = v46 - v76;
            let v93 = v34 * v92;
            let v96 = (v45 * v73) * v77;
            let v97 = ((v90 * v76) + v93) - v96;
            let v98 = v83 * v73;
            let v110 = v97 + (v98 * ((v30 * (v46 + ((v46 + (v99 * (((-v97) * v74).exp()))).sqrt()))).ln()));
            let v116 = v55 * ((v111 * ((v80 / v110).ln())).exp());
            let v119 = (v117 * v110) / v80;
            let v122 = (v30 * v120) / v27;
            let v128 = v84 * (((v122.exp()) - ((-v122).exp())).ln());
            let v131 = ((v128 * v76) + v93) - v96;
            let v142 = v131 + (v98 * ((v30 * (v46 + ((v46 + (v99 * (((-v131) * v74).exp()))).sqrt()))).ln()));
            let v148 = v55 * ((v143 * ((v120 / v142).ln())).exp());
            let v151 = (v149 * v142) / v120;
            let v157 = v152 * (((v52 * v77) + (v32 * v79)).exp());
            let v159 = v30 * v45;
            let v161 = v30 * v34;
            let v165 = v158 * (((v159 * v77) + (v161 * v79)).exp());
            let v168 = (v30 * v166) / v27;
            let v174 = v84 * (((v168.exp()) - ((-v168).exp())).ln());
            let v176 = v37 * v92;
            let v178 = ((v174 * v76) + v176) - v96;
            let v189 = v178 + (v98 * ((v30 * (v46 + ((v46 + (v99 * (((-v178) * v74).exp()))).sqrt()))).ln()));
            let v195 = v58 * ((v190 * ((v166 / v189).ln())).exp());
            let v198 = v35 * v79;
            let v201 = v196 * (((v49 * v77) + v198).exp());
            let v207 = v202 * (((v53 * v77) + (v31 * v79)).exp());
            let v215 = v208 * (((v209 * v77) - (v211 * v79)).exp());
            let v217 = v48 - v29;
            let v220 = v216 * ((v217 * v77).exp());
            let v225 = v46 / (v221 * ((v48 * v77).exp()));
            let v230 = v226 * (v46 + (v227 * v75));
            let v232 = if v231 > v0 { 1.0 } else { 0.0 };
            let v977: f64;
            let v981: f64;
            if v232 != 0.0 {
                let v236 = v231 * (v46 - (v233 * v75));
                v977 = v236;
                v981 = v226;
            } else {
                v977 = v231;
                v981 = v230;
            }
            let v245 = v237 * ((v46 + (v238 * v75)) + ((v241 * v75) * v75));
            let v247 = if v246 == v46 { 1.0 } else { 0.0 };
            if v247 != 0.0 {
            } else {
            }
            let v249 = if v248 == v46 { 1.0 } else { 0.0 };
            let v1528: f64;
            let v1537: f64;
            if v249 != 0.0 {
                let v253 = v56 * ((v250 * v75).exp());
                let v258 = v254 * ((v255 * v75).exp());
                v1528 = v258;
                v1537 = v253;
            } else {
                v1528 = v254;
                v1537 = v56;
            }
            let v263 = v259 * ((v260 * v77).exp());
            let v266 = (v30 * v264) / v27;
            let v272 = v84 * (((v266.exp()) - ((-v266).exp())).ln());
            let v275 = ((v272 * v76) + v176) - v96;
            let v286 = v275 + (v98 * ((v30 * (v46 + ((v46 + (v99 * (((-v275) * v74).exp()))).sqrt()))).ln()));
            let v293 = v287 * ((v288 * ((v264 / v286).ln())).exp());
            let v296 = (v30 * v294) / v27;
            let v302 = v84 * (((v296.exp()) - ((-v296).exp())).ln());
            let v306 = ((v302 * v76) + (v40 * v92)) - v96;
            let v317 = v306 + (v98 * ((v30 * (v46 + ((v46 + (v99 * (((-v306) * v74).exp()))).sqrt()))).ln()));
            let v324 = v318 * ((v319 * ((v294 / v317).ln())).exp());
            let v330 = v327 * (((v51 * v77) + v198).exp());
            let v335 = v331 * ((v332 * v77).exp());
            let v344 = v336 / (((v211 * v74) * (((v338 * v77).exp()) - v46)).exp());
            let v347 = if v345 <= v346 { 1.0 } else { 0.0 };
            let v358: f64;
            if v347 != 0.0 {
                let v353 = v46 + (v75 * (v348 + (v349 * v75)));
                v358 = v353;
            } else {
                let v356 = (v354 * v77).exp();
                v358 = v356;
            }
            let v359 = v357 * v358;
            let v364 = (v360 * v358) * ((v54 * v79).exp());
            let v370 = v366 * ((v367 * v77).exp());
            let v375 = v371 * ((v372 * v77).exp());
            let v380 = v376 * ((v377 * v77).exp());
            let v385 = if v381 >= v384 { 1.0 } else { 0.0 };
            let v386 = if (if v382 != v0 { 1.0 } else { 0.0 }) != 0.0 && v385 != 0.0 { 1.0 } else { 0.0 };
            let v599: f64;
            let v602: f64;
            let v611: f64;
            let v627: f64;
            let v633: f64;
            let v719: f64;
            let v975: f64;
            let v979: f64;
            let v993: f64;
            let v995: f64;
            let v1022: f64;
            let v1024: f64;
            let v1025: f64;
            let v1056: f64;
            let v1058: f64;
            let v1059: f64;
            let v1103: f64;
            let v1124: f64;
            let v1142: f64;
            let v1157: f64;
            let v1171: f64;
            let v1192: f64;
            let v1193: f64;
            let v1389: f64;
            let v1403: f64;
            let v1420: f64;
            let v1526: f64;
            let v1535: f64;
            let v1555: f64;
            let v1590: f64;
            let v1599: f64;
            let v1609: f64;
            let v1611: f64;
            let v1669: f64;
            let v1671: f64;
            let v1683: f64;
            if v386 != 0.0 {
                let v388 = v62 + v387;
                let v390 = if v388 < v389 { 1.0 } else { 0.0 };
                let v395: f64;
                if v390 != 0.0 {
                    v395 = v391;
                } else {
                    let v393 = if v388 > v392 { 1.0 } else { 0.0 };
                    let v396: f64;
                    if v393 != 0.0 {
                        v396 = v394;
                    } else {
                        v396 = v388;
                    }
                    v395 = v396;
                }
                let v398 = (v24 * v395) / v26;
                let v399 = v46 / v398;
                let v400 = v395 - v22;
                let v401 = v395 / v22;
                let v402 = v401.ln();
                let v404 = v399 * (v401 - v46);
                let v406 = v46 - v401;
                let v407 = v34 * v406;
                let v410 = (v45 * v398) * v402;
                let v411 = ((v90 * v401) + v407) - v410;
                let v412 = v83 * v398;
                let v423 = v411 + (v412 * ((v30 * (v46 + ((v46 + (v99 * (((-v411) * v399).exp()))).sqrt()))).ln()));
                let v428 = v55 * ((v111 * ((v80 / v423).ln())).exp());
                let v430 = (v117 * v423) / v80;
                let v433 = ((v128 * v401) + v407) - v410;
                let v444 = v433 + (v412 * ((v30 * (v46 + ((v46 + (v99 * (((-v433) * v399).exp()))).sqrt()))).ln()));
                let v449 = v55 * ((v143 * ((v120 / v444).ln())).exp());
                let v451 = (v149 * v444) / v120;
                let v456 = v152 * (((v52 * v402) + (v32 * v404)).exp());
                let v461 = v158 * (((v159 * v402) + (v161 * v404)).exp());
                let v463 = v37 * v406;
                let v465 = ((v174 * v401) + v463) - v410;
                let v476 = v465 + (v412 * ((v30 * (v46 + ((v46 + (v99 * (((-v465) * v399).exp()))).sqrt()))).ln()));
                let v481 = v58 * ((v190 * ((v166 / v476).ln())).exp());
                let v483 = v35 * v404;
                let v486 = v196 * (((v49 * v402) + v483).exp());
                let v491 = v202 * (((v53 * v402) + (v31 * v404)).exp());
                let v496 = v208 * (((v209 * v402) - (v211 * v404)).exp());
                let v499 = v216 * ((v217 * v402).exp());
                let v503 = v46 / (v221 * ((v48 * v402).exp()));
                let v506 = v226 * (v46 + (v227 * v400));
                let v976: f64;
                let v980: f64;
                if v232 != 0.0 {
                    let v509 = v231 * (v46 - (v233 * v400));
                    v976 = v509;
                    v980 = v226;
                } else {
                    v976 = v231;
                    v980 = v506;
                }
                let v515 = v237 * ((v46 + (v238 * v400)) + ((v241 * v400) * v400));
                if v247 != 0.0 {
                } else {
                }
                let v1527: f64;
                let v1536: f64;
                if v249 != 0.0 {
                    let v518 = v56 * ((v250 * v400).exp());
                    let v521 = v254 * ((v255 * v400).exp());
                    v1527 = v521;
                    v1536 = v518;
                } else {
                    v1527 = v254;
                    v1536 = v56;
                }
                let v524 = v259 * ((v260 * v402).exp());
                let v527 = ((v272 * v401) + v463) - v410;
                let v538 = v527 + (v412 * ((v30 * (v46 + ((v46 + (v99 * (((-v527) * v399).exp()))).sqrt()))).ln()));
                let v543 = v287 * ((v288 * ((v264 / v538).ln())).exp());
                let v547 = ((v302 * v401) + (v40 * v406)) - v410;
                let v558 = v547 + (v412 * ((v30 * (v46 + ((v46 + (v99 * (((-v547) * v399).exp()))).sqrt()))).ln()));
                let v563 = v318 * ((v319 * ((v294 / v558).ln())).exp());
                let v567 = v327 * (((v51 * v402) + v483).exp());
                let v570 = v331 * ((v332 * v402).exp());
                let v577 = v336 / (((v211 * v399) * (((v338 * v402).exp()) - v46)).exp());
                let v584: f64;
                if v347 != 0.0 {
                    let v581 = v46 + (v400 * (v348 + (v349 * v400)));
                    v584 = v581;
                } else {
                    let v583 = (v354 * v402).exp();
                    v584 = v583;
                }
                let v585 = v357 * v584;
                let v589 = (v360 * v584) * ((v54 * v404).exp());
                let v592 = v366 * ((v367 * v402).exp());
                let v595 = v371 * ((v372 * v402).exp());
                let v598 = v376 * ((v377 * v402).exp());
                v599 = v543;
                v602 = v481;
                v611 = v476;
                v627 = v399;
                v633 = v398;
                v719 = v538;
                v975 = v976;
                v979 = v980;
                v993 = v499;
                v995 = v503;
                v1022 = v428;
                v1024 = v423;
                v1025 = v430;
                v1056 = v449;
                v1058 = v444;
                v1059 = v451;
                v1103 = v570;
                v1124 = v577;
                v1142 = v515;
                v1157 = v496;
                v1171 = v491;
                v1192 = v589;
                v1193 = v585;
                v1389 = v456;
                v1403 = v461;
                v1420 = v486;
                v1526 = v1527;
                v1535 = v1536;
                v1555 = v524;
                v1590 = v595;
                v1599 = v567;
                v1609 = v563;
                v1611 = v558;
                v1669 = v598;
                v1671 = v592;
                v1683 = v395;
            } else {
                v599 = v293;
                v602 = v195;
                v611 = v189;
                v627 = v74;
                v633 = v73;
                v719 = v286;
                v975 = v977;
                v979 = v981;
                v993 = v220;
                v995 = v225;
                v1022 = v116;
                v1024 = v110;
                v1025 = v119;
                v1056 = v148;
                v1058 = v142;
                v1059 = v151;
                v1103 = v335;
                v1124 = v344;
                v1142 = v245;
                v1157 = v215;
                v1171 = v207;
                v1192 = v364;
                v1193 = v359;
                v1389 = v157;
                v1403 = v165;
                v1420 = v201;
                v1526 = v1528;
                v1535 = v1537;
                v1555 = v263;
                v1590 = v375;
                v1599 = v330;
                v1609 = v324;
                v1611 = v317;
                v1669 = v380;
                v1671 = v370;
                v1683 = v70;
            }
            let v601 = if v599 <= v600 { 1.0 } else { 0.0 };
            let v844: f64;
            let v1647: f64;
            if v601 != 0.0 {
                let v604 = v602 * v603;
                let v606 = v602 * (v46 - v603);
                let v608 = if v607 < v63 { 1.0 } else { 0.0 };
                let v1648: f64;
                if v608 != 0.0 {
                    let v609 = if v606 > v0 { 1.0 } else { 0.0 };
                    let v1649: f64;
                    if v609 != 0.0 {
                        let v610 = v190 / v99;
                        let v612 = v607 - v611;
                        let v618 = v611 * (v46 - ((v614 / v190).exp()));
                        let v619 = v613 * v606;
                        let v625 = v606 * (((v610 - v190) * ((v607 / v611).ln())).exp());
                        let v628 = (v618 - v5) * v627;
                        let v630 = if v628 < v629 { 1.0 } else { 0.0 };
                        let v641: f64;
                        if v630 != 0.0 {
                            let v636 = v618 - (v633 * ((v46 + (v628.exp())).ln()));
                            v641 = v636;
                        } else {
                            v641 = v5;
                        }
                        let v640 = (v637 * v612) + (v99 * v633);
                        let v643 = (v612 + v641) / v640;
                        let v644 = if v643 < v629 { 1.0 } else { 0.0 };
                        let v660: f64;
                        if v644 != 0.0 {
                            let v655 = (-v612) + (v640 * (((v46 + (v643.exp())).ln()) - (((-(v612 + v618)) / v640).exp())));
                            v660 = v655;
                        } else {
                            v660 = v641;
                        }
                        let v663 = (v46 - (v660 / v611)).ln();
                        let v664 = v46 - v190;
                        let v665 = v46 - v610;
                        let v685 = (((((v606 * (v46 - ((v663 * v664).exp()))) / v664) + ((v625 * (v46 - ((((v46 - (v641 / v611)).ln()) * v665).exp()))) / v665)) - ((v625 * (v46 - ((v663 * v665).exp()))) / v665)) * v611) + (v619 * (v5 - v641));
                        v1649 = v685;
                    } else {
                        v1649 = v0;
                    }
                    v1648 = v1649;
                } else {
                    let v686 = if v606 > v0 { 1.0 } else { 0.0 };
                    let v1650: f64;
                    if v686 != 0.0 {
                        let v691 = v611 * (v46 - ((v687 / v190).exp()));
                        let v693 = (v691 - v5) * v627;
                        let v701 = v691 - (v633 * ((v693 + (((v693 * v693) + v695).sqrt())) * v30));
                        let v705 = v46 - v190;
                        let v714 = v606 * (((v611 * (v46 - ((((v46 - (v701 / v611)).ln()) * v705).exp()))) / v705) + (v613 * (v5 - v701)));
                        v1650 = v714;
                    } else {
                        v1650 = v0;
                    }
                    v1648 = v1650;
                }
                v844 = v604;
                v1647 = v1648;
            } else {
                let v715 = v599 * v603;
                let v717 = if v716 < v63 { 1.0 } else { 0.0 };
                if v717 != 0.0 {
                    let v718 = if v715 > v0 { 1.0 } else { 0.0 };
                    if v718 != 0.0 {
                        let v720 = v716 - v719;
                        let v725 = v719 * (v46 - ((v721 / v288).exp()));
                        let v727 = (v725 - v8) * v627;
                        let v728 = if v727 < v629 { 1.0 } else { 0.0 };
                        let v737: f64;
                        if v728 != 0.0 {
                            let v733 = v725 - (v633 * ((v46 + (v727.exp())).ln()));
                            v737 = v733;
                        } else {
                            v737 = v8;
                        }
                        let v740 = if ((v720 + v737) / ((v637 * v720) + (v99 * v633))) < v629 { 1.0 } else { 0.0 };
                        if v740 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                    let v741 = if v715 > v0 { 1.0 } else { 0.0 };
                    if v741 != 0.0 {
                    } else {
                    }
                }
                let v743 = v599 * (v46 - v603);
                let v1651: f64;
                if v717 != 0.0 {
                    let v744 = if v743 > v0 { 1.0 } else { 0.0 };
                    let v1652: f64;
                    if v744 != 0.0 {
                        let v745 = v288 / v99;
                        let v746 = v716 - v719;
                        let v751 = v719 * (v46 - ((v747 / v288).exp()));
                        let v752 = v613 * v743;
                        let v758 = v743 * (((v745 - v288) * ((v716 / v719).ln())).exp());
                        let v760 = (v751 - v5) * v627;
                        let v761 = if v760 < v629 { 1.0 } else { 0.0 };
                        let v770: f64;
                        if v761 != 0.0 {
                            let v766 = v751 - (v633 * ((v46 + (v760.exp())).ln()));
                            v770 = v766;
                        } else {
                            v770 = v5;
                        }
                        let v769 = (v637 * v746) + (v99 * v633);
                        let v772 = (v746 + v770) / v769;
                        let v773 = if v772 < v629 { 1.0 } else { 0.0 };
                        let v789: f64;
                        if v773 != 0.0 {
                            let v784 = (-v746) + (v769 * (((v46 + (v772.exp())).ln()) - (((-(v746 + v751)) / v769).exp())));
                            v789 = v784;
                        } else {
                            v789 = v770;
                        }
                        let v792 = (v46 - (v789 / v719)).ln();
                        let v793 = v46 - v288;
                        let v794 = v46 - v745;
                        let v814 = (((((v743 * (v46 - ((v792 * v793).exp()))) / v793) + ((v758 * (v46 - ((((v46 - (v770 / v719)).ln()) * v794).exp()))) / v794)) - ((v758 * (v46 - ((v792 * v794).exp()))) / v794)) * v719) + (v752 * (v5 - v770));
                        v1652 = v814;
                    } else {
                        v1652 = v0;
                    }
                    v1651 = v1652;
                } else {
                    let v815 = if v743 > v0 { 1.0 } else { 0.0 };
                    let v1653: f64;
                    if v815 != 0.0 {
                        let v820 = v719 * (v46 - ((v816 / v288).exp()));
                        let v822 = (v820 - v5) * v627;
                        let v829 = v820 - (v633 * ((v822 + (((v822 * v822) + v695).sqrt())) * v30));
                        let v833 = v46 - v288;
                        let v842 = v743 * (((v719 * (v46 - ((((v46 - (v829 / v719)).ln()) * v833).exp()))) / v833) + (v613 * (v5 - v829)));
                        v1653 = v842;
                    } else {
                        v1653 = v0;
                    }
                    v1651 = v1653;
                }
                v844 = v602;
                v1647 = v1651;
            }
            let v843 = if v607 < v63 { 1.0 } else { 0.0 };
            let v944: f64;
            if v843 != 0.0 {
                let v845 = if v844 > v0 { 1.0 } else { 0.0 };
                let v945: f64;
                if v845 != 0.0 {
                    let v846 = v190 / v99;
                    let v847 = v607 - v611;
                    let v852 = v611 * (v46 - ((v848 / v190).exp()));
                    let v853 = v613 * v844;
                    let v859 = v844 * (((v846 - v190) * ((v607 / v611).ln())).exp());
                    let v861 = (v852 - v8) * v627;
                    let v862 = if v861 < v629 { 1.0 } else { 0.0 };
                    let v871: f64;
                    if v862 != 0.0 {
                        let v867 = v852 - (v633 * ((v46 + (v861.exp())).ln()));
                        v871 = v867;
                    } else {
                        v871 = v8;
                    }
                    let v870 = (v637 * v847) + (v99 * v633);
                    let v873 = (v847 + v871) / v870;
                    let v874 = if v873 < v629 { 1.0 } else { 0.0 };
                    let v890: f64;
                    if v874 != 0.0 {
                        let v885 = (-v847) + (v870 * (((v46 + (v873.exp())).ln()) - (((-(v847 + v852)) / v870).exp())));
                        v890 = v885;
                    } else {
                        v890 = v871;
                    }
                    let v893 = (v46 - (v890 / v611)).ln();
                    let v894 = v46 - v190;
                    let v895 = v46 - v846;
                    let v915 = (((((v844 * (v46 - ((v893 * v894).exp()))) / v894) + ((v859 * (v46 - ((((v46 - (v871 / v611)).ln()) * v895).exp()))) / v895)) - ((v859 * (v46 - ((v893 * v895).exp()))) / v895)) * v611) + (v853 * (v8 - v871));
                    v945 = v915;
                } else {
                    v945 = v0;
                }
                v944 = v945;
            } else {
                let v916 = if v844 > v0 { 1.0 } else { 0.0 };
                let v946: f64;
                if v916 != 0.0 {
                    let v921 = v611 * (v46 - ((v917 / v190).exp()));
                    let v923 = (v921 - v8) * v627;
                    let v930 = v921 - (v633 * ((v923 + (((v923 * v923) + v695).sqrt())) * v30));
                    let v934 = v46 - v190;
                    let v943 = v844 * (((v611 * (v46 - ((((v46 - (v930 / v611)).ln()) * v934).exp()))) / v934) + (v613 * (v8 - v930)));
                    v946 = v943;
                } else {
                    v946 = v0;
                }
                v944 = v946;
            }
            let v947 = if v844 > v0 { 1.0 } else { 0.0 };
            let v1017: f64;
            if v947 != 0.0 {
                let v952 = v611 * (v46 - ((v948 / v190).exp()));
                let v954 = (v952 - v8) * v627;
                let v957 = ((v954 * v954) + v695).sqrt();
                let v959 = (v954 + v957) * v30;
                let v962 = v959 / v957;
                let v974 = ((v844 * (((-v190) * ((v46 - ((v952 - (v633 * v959)) / v611)).ln())).exp())) * v962) + ((v613 * v844) * (v46 - v962));
                v1017 = v974;
            } else {
                v1017 = v0;
            }
            let v983: f64;
            if v232 != 0.0 {
                let v978 = v975 - v8;
                v983 = v978;
            } else {
                let v982 = v12 - v979;
                v983 = v982;
            }
            let v985 = (v983 * v627) - v46;
            let v992 = (v46 + ((v985 + (((v985 * v985) + v695).sqrt())) / v83)) * v633;
            let v1008 = (v992 - v993) / v1007;
            let v1016 = ((v992 * v995) / ((((v46 + ((v997 * ((v992 / v993).ln())).exp())).ln()) / v997).exp())) * (v46 + (v30 * (v1008 + (((v1008 * v1008) + v1010).sqrt()))));
            let v1019 = if (if v1017 > v0 { 1.0 } else { 0.0 }) != 0.0 && v947 != 0.0 { 1.0 } else { 0.0 };
            let v1127: f64;
            let v1144: f64;
            if v1019 != 0.0 {
                let v1020 = v844 / v1017;
                let v1021 = v944 / v844;
                v1127 = v1021;
                v1144 = v1020;
            } else {
                v1127 = v0;
                v1144 = v46;
            }
            let v1023 = if v1022 > v0 { 1.0 } else { 0.0 };
            let v1054: f64;
            if v1023 != 0.0 {
                let v1031 = v1024 * (v46 - (((-(v1025.ln())) / v111).exp()));
                let v1033 = (v1031 - v11) * v627;
                let v1040 = v1031 - (v633 * ((v1033 + (((v1033 * v1033) + v695).sqrt())) * v30));
                let v1044 = v46 - v111;
                let v1053 = v1022 * (((v1024 * (v46 - ((((v46 - (v1040 / v1024)).ln()) * v1044).exp()))) / v1044) + (v1025 * (v11 - v1040)));
                v1054 = v1053;
            } else {
                v1054 = v0;
            }
            let v1055 = v1054 / v1022;
            let v1093: f64;
            let v1104: f64;
            let v1122: f64;
            if v347 != 0.0 {
                let v1057 = if v1056 > v0 { 1.0 } else { 0.0 };
                let v1088: f64;
                if v1057 != 0.0 {
                    let v1065 = v1058 * (v46 - (((-(v1059.ln())) / v143).exp()));
                    let v1067 = (v1065 - v11) * v627;
                    let v1074 = v1065 - (v633 * ((v1067 + (((v1067 * v1067) + v695).sqrt())) * v30));
                    let v1078 = v46 - v143;
                    let v1087 = v1056 * (((v1058 * (v46 - ((((v46 - (v1074 / v1058)).ln()) * v1078).exp()))) / v1078) + (v1059 * (v11 - v1074)));
                    v1088 = v1087;
                } else {
                    v1088 = v0;
                }
                let v1089 = v1088 / v1056;
                v1093 = v1058;
                v1104 = v143;
                v1122 = v1089;
            } else {
                v1093 = v1024;
                v1104 = v111;
                v1122 = v1055;
            }
            let v1090 = if v331 == v0 { 1.0 } else { 0.0 };
            let v1120: f64;
            if v1090 != 0.0 {
                v1120 = v46;
            } else {
                let v1092 = v1091 * v633;
                let v1095 = (v1093 - v11) / v1092;
                let v1111 = v1103 * (v46 - ((v1104 * ((v46 - ((v1093 - ((v1092 * (v1095 + (((v1095 * v1095) + v695).sqrt()))) * v30)) / v1093)).ln())).exp()));
                let v1114 = if (v1111.abs()) >= v1113 { 1.0 } else { 0.0 };
                let v1121: f64;
                if v1114 != 0.0 {
                    let v1117 = ((v1111.exp()) - v46) / v1111;
                    v1121 = v1117;
                } else {
                    let v1119 = v46 + (v1111 * v30);
                    v1121 = v1119;
                }
                v1120 = v1121;
            }
            let v1133 = (v1131 * ((v46 + ((v1120 * v1122) / v1124)) + (v1127 / v1128))) - v46;
            let v1141 = v1134 * (v46 + ((v1133 + (((v1133 * v1133) + v695).sqrt())) / v83));
            let v1152 = (v1142 + (v1143 * (v1144 - v46))) + (v1148 * ((v46 / v1144) - v46));
            let v1154 = if v1153 == v46 { 1.0 } else { 0.0 };
            let v1185: f64;
            if v1154 != 0.0 {
                let v1159 = v1157 / (v46 + ((v1152 / v1142) - v46));
                v1185 = v1159;
            } else {
                v1185 = v1157;
            }
            let v1163 = v11 / (v1161 * v633);
            let v1164 = if v1163 > v629 { 1.0 } else { 0.0 };
            let v1167: f64;
            let v1168: f64;
            if v1164 != 0.0 {
                let v1166 = v46 + (v1163 - v629);
                v1167 = v1166;
                v1168 = v629;
            } else {
                v1167 = v46;
                v1168 = v1163;
            }
            let v1172 = v1171 * (v1167 * (rspice_limexp(v1168)));
            let v1175 = v8 / (v1173 * v633);
            let v1176 = if v1175 > v629 { 1.0 } else { 0.0 };
            let v1179: f64;
            let v1180: f64;
            if v1176 != 0.0 {
                let v1178 = v46 + (v1175 - v629);
                v1179 = v1178;
                v1180 = v629;
            } else {
                v1179 = v46;
                v1180 = v1175;
            }
            let v1183 = v1171 * (v1179 * (rspice_limexp(v1180)));
            let v1184 = if v360 != v0 { 1.0 } else { 0.0 };
            let v1209: f64;
            let v1213: f64;
            if v1184 != 0.0 {
                let v1188 = (v1172 / v1185) + (v1183 / v1160);
                let v1198 = (v1189 * (((v1172 * (v1172 / v1016)) * (v1192 / v1193)).ln())).exp();
                let v1199 = v1188 + v1198;
                let v1202 = (v1188 + (v1172 / v1193)) + v1198;
                v1209 = v1199;
                v1213 = v1202;
            } else {
                let v1205 = (v1172 / v1185) + (v1183 / v1160);
                let v1207 = v1205 + (v1172 / v1193);
                v1209 = v1205;
                v1213 = v1207;
            }
            let v1208 = v1141 * v1141;
            let v1212 = v1141 + ((v1208 + v1209).sqrt());
            let v1216 = v1141 + ((v1208 + v1213).sqrt());
            let v1220 = if ((v1213 - v1209).abs()) > v1219 { 1.0 } else { 0.0 };
            let v1243: f64;
            if v1220 != 0.0 {
                let v1223 = (v1016 / (v46 + v365)) / v1172;
                let v1229 = (v46 - (v1223 * v1212)) / (v46 + (v1223 * (v1216 - v1212)));
                let v1236 = ((((v1229 * v1229) + v1231).sqrt()) + v1229) / v1235;
                v1243 = v1236;
            } else {
                v1243 = v0;
            }
            let v1238 = if v1237 == v0 { 1.0 } else { 0.0 };
            let v1373: f64;
            if v1238 != 0.0 {
                let v1262: f64;
                if v1184 != 0.0 {
                    let v1254 = (((v1172 / v1185) + (v1183 / v1160)) + (((v1172 / v1193) * v1243) * v1243)) + ((v1189 * (((v1172 * (v1172 / v1016)) * (v1192 / v1193)).ln())).exp());
                    v1262 = v1254;
                } else {
                    let v1261 = ((v1172 / v1185) + (v1183 / v1160)) + (((v1172 / v1193) * v1243) * v1243);
                    v1262 = v1261;
                }
                let v1265 = v1141 + ((v1208 + v1262).sqrt());
                v1373 = v1265;
            } else {
                let v1268 = v1267 * v1141;
                let v1272 = if (if v208 == v1269 { 1.0 } else { 0.0 }) != 0.0 && (if v357 == v1269 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1287: f64;
                if v1272 != 0.0 {
                    v1287 = v0;
                } else {
                    let v1280 = -(((v1172 / v1185) + (v1183 / v1160)) + (((v1172 / v1193) * v1243) * v1243));
                    v1287 = v1280;
                }
                let v1286 = v1268 * v1268;
                let v1289 = v1287 - (v1286 * v1266);
                let v1297 = ((((v83 * v1268) * v1286) / v1292) - ((v1268 * v1287) * v1266)) + (((((-v1172) * v1172) / v1016) * v1192) / v1193);
                let v1302 = (v1289 * v1289) * v1289;
                let v1304 = ((v1297 * v1297) * v1299) + (v1302 / v1292);
                let v1307 = if (v1304.abs()) < v1306 { 1.0 } else { 0.0 };
                let v1371: f64;
                if v1307 != 0.0 {
                    let v1311 = ((v41 * v1297) / v1289) - (v1268 * v1266);
                    v1371 = v1311;
                } else {
                    let v1312 = if v1304 > v0 { 1.0 } else { 0.0 };
                    let v1372: f64;
                    if v1312 != 0.0 {
                        let v1314 = (-v1297) * v30;
                        let v1315 = v1304.sqrt();
                        let v1316 = v1314 + v1315;
                        let v1317 = if v1316 > v0 { 1.0 } else { 0.0 };
                        let v1336: f64;
                        if v1317 != 0.0 {
                            let v1320 = (v1266 * (v1316.ln())).exp();
                            v1336 = v1320;
                        } else {
                            let v1325 = -((v1266 * ((-v1316).ln())).exp());
                            v1336 = v1325;
                        }
                        let v1326 = v1314 - v1315;
                        let v1327 = if v1326 > v0 { 1.0 } else { 0.0 };
                        let v1337: f64;
                        if v1327 != 0.0 {
                            let v1330 = (v1266 * (v1326.ln())).exp();
                            v1337 = v1330;
                        } else {
                            let v1335 = -((v1266 * ((-v1326).ln())).exp());
                            v1337 = v1335;
                        }
                        let v1340 = (v1336 + v1337) - (v1268 * v1266);
                        v1372 = v1340;
                    } else {
                        let v1346 = ((-v1297) * v30) * ((v1343 / v1302).sqrt());
                        let v1347 = v1346 * v1346;
                        let v1348 = if v1346 >= v0 { 1.0 } else { 0.0 };
                        let v1365: f64;
                        if v1348 != 0.0 {
                            let v1354 = v1349 - (((v1347 / (v46 - v1347)).sqrt()).atan());
                            v1365 = v1354;
                        } else {
                            let v1360 = v1355 + (((v1347 / (v46 - v1347)).sqrt()).atan());
                            v1365 = v1360;
                        }
                        let v1370 = ((((v1361 * v1289) * v1266).sqrt()) * ((v1266 * v1365).cos())) - (v1268 * v1266);
                        v1372 = v1370;
                    }
                    v1371 = v1372;
                }
                v1373 = v1371;
            }
            let v1375 = if v1373 < v1374 { 1.0 } else { 0.0 };
            let v1376: f64;
            if v1375 != 0.0 {
                v1376 = v1374;
            } else {
                v1376 = v1373;
            }
            let v1377 = v1172 / v1376;
            let v1378 = v1183 / v1376;
            let v1379 = if v1377 < v1374 { 1.0 } else { 0.0 };
            let v1380: f64;
            if v1379 != 0.0 {
                v1380 = v1374;
            } else {
                v1380 = v1377;
            }
            let v1381 = v1380 - v1378;
            let v1382 = if v152 > v0 { 1.0 } else { 0.0 };
            let v1410: f64;
            if v1382 != 0.0 {
                let v1385 = v11 / (v1383 * v633);
                let v1386 = if v1385 > v629 { 1.0 } else { 0.0 };
                let v1390: f64;
                let v1391: f64;
                if v1386 != 0.0 {
                    let v1388 = v46 + (v1385 - v629);
                    v1390 = v1388;
                    v1391 = v629;
                } else {
                    v1390 = v46;
                    v1391 = v1385;
                }
                let v1395 = v1389 * ((v1390 * (rspice_limexp(v1391))) - v46);
                v1410 = v1395;
            } else {
                v1410 = v0;
            }
            let v1396 = if v158 > v0 { 1.0 } else { 0.0 };
            let v1411: f64;
            if v1396 != 0.0 {
                let v1399 = v11 / (v1397 * v633);
                let v1400 = if v1399 > v629 { 1.0 } else { 0.0 };
                let v1404: f64;
                let v1405: f64;
                if v1400 != 0.0 {
                    let v1402 = v46 + (v1399 - v629);
                    v1404 = v1402;
                    v1405 = v629;
                } else {
                    v1404 = v46;
                    v1405 = v1399;
                }
                let v1409 = v1403 * ((v1404 * (rspice_limexp(v1405))) - v46);
                v1411 = v1409;
            } else {
                v1411 = v0;
            }
            let v1412 = v1410 + v1411;
            let v1413 = if v196 > v0 { 1.0 } else { 0.0 };
            let v1427: f64;
            if v1413 != 0.0 {
                let v1416 = v8 / (v1414 * v633);
                let v1417 = if v1416 > v629 { 1.0 } else { 0.0 };
                let v1421: f64;
                let v1422: f64;
                if v1417 != 0.0 {
                    let v1419 = v46 + (v1416 - v629);
                    v1421 = v1419;
                    v1422 = v629;
                } else {
                    v1421 = v46;
                    v1422 = v1416;
                }
                let v1426 = v1420 * ((v1421 * (rspice_limexp(v1422))) - v46);
                v1427 = v1426;
            } else {
                v1427 = v0;
            }
            let v1428 = v1412 + v1427;
            let v1529: f64;
            if v843 != 0.0 {
                let v1429 = if v602 > v0 { 1.0 } else { 0.0 };
                let v1530: f64;
                if v1429 != 0.0 {
                    let v1430 = v190 / v99;
                    let v1431 = v607 - v611;
                    let v1436 = v611 * (v46 - ((v1432 / v190).exp()));
                    let v1437 = v613 * v602;
                    let v1443 = v602 * (((v1430 - v190) * ((v607 / v611).ln())).exp());
                    let v1445 = (v1436 - v8) * v627;
                    let v1446 = if v1445 < v629 { 1.0 } else { 0.0 };
                    let v1456: f64;
                    let v1483: f64;
                    if v1446 != 0.0 {
                        let v1447 = v1445.exp();
                        let v1448 = v46 + v1447;
                        let v1449 = v1447 / v1448;
                        let v1452 = v1436 - (v633 * (v1448.ln()));
                        v1456 = v1452;
                        v1483 = v1449;
                    } else {
                        v1456 = v8;
                        v1483 = v46;
                    }
                    let v1455 = (v637 * v1431) + (v99 * v633);
                    let v1458 = (v1431 + v1456) / v1455;
                    let v1459 = if v1458 < v629 { 1.0 } else { 0.0 };
                    let v1475: f64;
                    let v1485: f64;
                    if v1459 != 0.0 {
                        let v1460 = v1458.exp();
                        let v1461 = v46 + v1460;
                        let v1462 = v1460 / v1461;
                        let v1471 = (-v1431) + (v1455 * ((v1461.ln()) - (((-(v1431 + v1436)) / v1455).exp())));
                        v1475 = v1471;
                        v1485 = v1462;
                    } else {
                        v1475 = v1456;
                        v1485 = v46;
                    }
                    let v1496 = ((((v602 * ((((v46 - (v1475 / v611)).ln()) * (-v190)).exp())) * v1483) * v1485) + ((v1443 * ((((v46 - (v1456 / v611)).ln()) * (-v1430)).exp())) * (v46 - v1485))) + (v1437 * (v46 - v1483));
                    v1530 = v1496;
                } else {
                    v1530 = v0;
                }
                v1529 = v1530;
            } else {
                let v1497 = if v602 > v0 { 1.0 } else { 0.0 };
                let v1531: f64;
                if v1497 != 0.0 {
                    let v1502 = v611 * (v46 - ((v1498 / v190).exp()));
                    let v1504 = (v1502 - v8) * v627;
                    let v1507 = ((v1504 * v1504) + v695).sqrt();
                    let v1509 = (v1504 + v1507) * v30;
                    let v1512 = v1509 / v1507;
                    let v1523 = v602 * (((((-v190) * ((v46 - ((v1502 - (v633 * v1509)) / v611)).ln())).exp()) * v1512) + (v613 * (v46 - v1512)));
                    v1531 = v1523;
                } else {
                    v1531 = v0;
                }
                v1529 = v1531;
            }
            let v1636: f64;
            if v249 != 0.0 {
                let v1524 = v611 - v8;
                let v1525 = if v1524 > v0 { 1.0 } else { 0.0 };
                let v1637: f64;
                if v1525 != 0.0 {
                    let v1532 = v1526 / v1529;
                    let v1533 = v1526 / v602;
                    let v1534 = if v1524 > v1533 { 1.0 } else { 0.0 };
                    let v1553: f64;
                    if v1534 != 0.0 {
                        let v1547 = (v1535 * (((-v1532) / v1533).exp())) * (v1533 + ((v46 + (v1532 / v1533)) * (v1524 - v1533)));
                        v1553 = v1547;
                    } else {
                        let v1552 = (v1535 * v1524) * (((-v1532) / v1524).exp());
                        v1553 = v1552;
                    }
                    let v1554 = v1380 * v1553;
                    v1637 = v1554;
                } else {
                    v1637 = v0;
                }
                v1636 = v1637;
            } else {
                v1636 = v0;
            }
            let v1556 = if v1555 > v0 { 1.0 } else { 0.0 };
            let v1587: f64;
            if v1556 != 0.0 {
                let v1566 = (((v46 + (v1055 / v1557)) + (v1127 / v1560)) + (v1380 / v1185)) + (v1378 / v1160);
                let v1572 = v1555 / (v30 * (v1566 + (((v1566 * v1566) + v1231).sqrt())));
                let v1573 = if v1428 > v0 { 1.0 } else { 0.0 };
                let v1588: f64;
                if v1573 != 0.0 {
                    let v1577 = ((v1574 * v1572) * v1428) * v627;
                    let v1579 = if v1577 < v1578 { 1.0 } else { 0.0 };
                    let v1589: f64;
                    if v1579 != 0.0 {
                        let v1582 = v1572 * (v46 - (v30 * v1577));
                        v1589 = v1582;
                    } else {
                        let v1586 = (v1572 * ((v1577 + v46).ln())) / v1577;
                        v1589 = v1586;
                    }
                    v1588 = v1589;
                } else {
                    v1588 = v1572;
                }
                v1587 = v1588;
            } else {
                v1587 = v0;
            }
            let v1591 = v1587 + v1590;
            let v1592 = if v327 > v0 { 1.0 } else { 0.0 };
            let v1663: f64;
            if v1592 != 0.0 {
                let v1594 = v1593 * v633;
                let v1601 = v1599 * ((rspice_limexp((v5 / v1594))) - (rspice_limexp((v15 / v1594))));
                v1663 = v1601;
            } else {
                v1663 = v0;
            }
            let v1602 = if v325 > v0 { 1.0 } else { 0.0 };
            if v1602 != 0.0 {
                let v1606 = if (v15 / (v1603 * v633)) > v629 { 1.0 } else { 0.0 };
                if v1606 != 0.0 {
                } else {
                }
            } else {
            }
            let v1608 = if v1607 < v63 { 1.0 } else { 0.0 };
            if v1608 != 0.0 {
                let v1610 = if v1609 > v0 { 1.0 } else { 0.0 };
                if v1610 != 0.0 {
                    let v1612 = v1607 - v1611;
                    let v1617 = v1611 * (v46 - ((v1613 / v319).exp()));
                    let v1619 = (v1617 - v15) * v627;
                    let v1620 = if v1619 < v629 { 1.0 } else { 0.0 };
                    let v1629: f64;
                    if v1620 != 0.0 {
                        let v1625 = v1617 - (v633 * ((v46 + (v1619.exp())).ln()));
                        v1629 = v1625;
                    } else {
                        v1629 = v15;
                    }
                    let v1632 = if ((v1612 + v1629) / ((v637 * v1612) + (v99 * v633))) < v629 { 1.0 } else { 0.0 };
                    if v1632 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v1633 = if v1609 > v0 { 1.0 } else { 0.0 };
                if v1633 != 0.0 {
                } else {
                }
            }
            let v1635 = if (if v382 == v46 { 1.0 } else { 0.0 }) != 0.0 && v385 != 0.0 { 1.0 } else { 0.0 };
            if v1635 != 0.0 {
            } else {
            }
            let v1641 = if (if v1638 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v237 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1659: f64;
            if v1641 != 0.0 {
                v1659 = v1642;
            } else {
                v1659 = v1380;
            }
            let v1654 = v1 * v1647;
            let v1655 = v1 * (v1643 * v5);
            let v1656 = v1 * (v1645 * v18);
            let v1657 = v1 * v1412;
            let v1658 = v1 * v1381;
            let v1661 = v1 * (v1659 - v1378);
            let v1662 = v1 * v1636;
            let v1664 = v1 * v1663;
            let v1665 = 0e0f64;
            let v1666 = 0e0f64;
            let v1667 = 0e0f64;
            let v1668 = if v376 >= v384 { 1.0 } else { 0.0 };
            if v1668 != 0.0 {
            } else {
            }
            let v1670 = if v366 >= v384 { 1.0 } else { 0.0 };
            if v1670 != 0.0 {
            } else {
            }
            let v1674 = if (if v259 >= v384 { 1.0 } else { 0.0 }) != 0.0 || (if v371 >= v384 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1698: f64;
            if v1674 != 0.0 {
                let v1675 = v19 / v1591;
                v1698 = v1675;
            } else {
                v1698 = v0;
            }
            let v1676 = if v382 == v0 { 1.0 } else { 0.0 };
            let v1679 = if v1676 != 0.0 || (if v1677 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v1679 != 0.0 {
            } else {
            }
            let v1681 = if v1676 != 0.0 || (if v381 < v384 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v1681 != 0.0 {
            } else {
            }
            let v1684 = v1682 * v1683;
            let v1699: f64;
            let v1731: f64;
            let v1732: f64;
            if v1674 != 0.0 {
                let v1686 = v1684 / v1591;
                v1699 = v1685;
                v1731 = v46;
                v1732 = v1686;
            } else {
                v1699 = v0;
                v1731 = v0;
                v1732 = v0;
            }
            let v1733: f64;
            let v1734: f64;
            if v1670 != 0.0 {
                let v1687 = v1684 / v1671;
                v1733 = v46;
                v1734 = v1687;
            } else {
                v1733 = v0;
                v1734 = v0;
            }
            let v1735: f64;
            let v1736: f64;
            if v1668 != 0.0 {
                let v1688 = v1684 / v1669;
                v1735 = v46;
                v1736 = v1688;
            } else {
                v1735 = v0;
                v1736 = v0;
            }
            let v1690 = v1657.abs();
            let v1693 = v1689 * (v1690.powf(v1691));
            let v1695 = v1694 * v1690;
            let v1697 = v1694 * (v1661.abs());
            let v1707 = if ((((((v1664 + v1665) + v1666) + v1667) + v1698) + v1699) + v1700) != v0 { 1.0 } else { 0.0 };
            if v1707 != 0.0 {
            } else {
            }
            let v1712 = (-(0e0f64)) - (-(0e0f64));
            let v1715 = if (v1712.abs()) > (ctx.simparam_or("gmin", v0)) { 1.0 } else { 0.0 };
            if v1715 != 0.0 {
            } else {
                let v1716 = if v1712 >= v0 { 1.0 } else { 0.0 };
                if v1716 != 0.0 {
                } else {
                }
            }
            let v1721 = (-(0e0f64)) - (-(0e0f64));
            let v1724 = if (v1721.abs()) > (ctx.simparam_or("gmin", v0)) { 1.0 } else { 0.0 };
            if v1724 != 0.0 {
            } else {
                let v1725 = if v1721 >= v0 { 1.0 } else { 0.0 };
                if v1725 != 0.0 {
                } else {
                }
            }
            let v1726 = 0e0f64;
            let v1729 = if (v1726.abs()) > (ctx.simparam_or("gmin", v0)) { 1.0 } else { 0.0 };
            if v1729 != 0.0 {
            } else {
                let v1730 = if v1726 >= v0 { 1.0 } else { 0.0 };
                if v1730 != 0.0 {
                } else {
                }
            }
        if v1731 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1732;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1733 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1734;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v1735 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v1736;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1693;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v46);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1695;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v1697;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

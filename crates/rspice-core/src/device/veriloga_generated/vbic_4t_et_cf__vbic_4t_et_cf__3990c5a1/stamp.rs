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
            let slot = match operator { 6568 => 0usize, 6570 => 1usize, 6572 => 2usize, 6574 => 3usize, 6576 => 4usize, 6578 => 5usize, 6580 => 6usize, 6585 => 7usize, 6589 => 8usize, _ => usize::MAX };
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
            let v1 = 2.7315e2f64;
            let v2 = parameters[0];
            let v4 = temperature;
            let v5 = parameters[105];
            let v7 = node_potentials[4];
            let v9 = 1.3806503e-23f64;
            let v11 = 1.602176462e-19f64;
            let v15 = parameters[53];
            let v16 = parameters[90];
            let v19 = parameters[1];
            let v20 = parameters[91];
            let v23 = parameters[2];
            let v24 = parameters[68];
            let v27 = parameters[6];
            let v28 = parameters[92];
            let v31 = parameters[7];
            let v32 = parameters[67];
            let v35 = parameters[8];
            let v36 = parameters[66];
            let v39 = parameters[9];
            let v40 = parameters[69];
            let v43 = parameters[10];
            let v44 = parameters[93];
            let v47 = parameters[11];
            let v48 = parameters[78];
            let v50 = parameters[71];
            let v52 = 1e0f64;
            let v58 = parameters[12];
            let v62 = parameters[94];
            let v63 = parameters[95];
            let v65 = parameters[96];
            let v71 = parameters[13];
            let v75 = parameters[42];
            let v76 = parameters[97];
            let v82 = parameters[44];
            let v86 = parameters[31];
            let v87 = parameters[79];
            let v89 = parameters[72];
            let v95 = parameters[33];
            let v99 = parameters[34];
            let v100 = parameters[80];
            let v102 = parameters[75];
            let v108 = parameters[35];
            let v112 = parameters[36];
            let v113 = parameters[73];
            let v119 = parameters[37];
            let v123 = parameters[38];
            let v124 = parameters[76];
            let v130 = parameters[39];
            let v134 = parameters[45];
            let v136 = parameters[46];
            let v138 = parameters[47];
            let v139 = parameters[74];
            let v145 = parameters[48];
            let v149 = parameters[49];
            let v150 = parameters[77];
            let v156 = parameters[50];
            let v160 = parameters[81];
            let v165 = parameters[41];
            let v166 = parameters[82];
            let v170 = parameters[98];
            let v171 = parameters[101];
            let v172 = parameters[102];
            let v178 = parameters[99];
            let v179 = parameters[103];
            let v183 = 2e0f64;
            let v186 = 5e-1f64;
            let v187 = parameters[17];
            let v192 = -5e-1f64;
            let v201 = 3e0f64;
            let v210 = 4e0f64;
            let v222 = parameters[24];
            let v227 = -5e-1f64;
            let v250 = parameters[28];
            let v255 = -5e-1f64;
            let v278 = parameters[16];
            let v280 = parameters[18];
            let v283 = parameters[21];
            let v285 = parameters[25];
            let v288 = parameters[23];
            let v290 = parameters[27];
            let v292 = parameters[29];
            let v295 = parameters[4];
            let v298 = parameters[3];
            let v299 = parameters[70];
            let v306 = parameters[51];
            let v310 = parameters[52];
            let v317 = parameters[54];
            let v321 = parameters[55];
            let v328 = parameters[5];
            let v332 = parameters[59];
            let v336 = parameters[60];
            let v341 = node_potentials[8];
            let v342 = node_potentials[9];
            let v344 = node_potentials[7];
            let v346 = node_potentials[6];
            let v348 = node_potentials[5];
            let v350 = node_potentials[10];
            let v353 = parameters[14];
            let v355 = parameters[19];
            let v360 = -1e0f64;
            let v390 = -5e-1f64;
            let v422 = -1e0f64;
            let v452 = -5e-1f64;
            let v483 = parameters[26];
            let v488 = -1e0f64;
            let v503 = parameters[85];
            let v533 = parameters[86];
            let v613 = -5e-1f64;
            let v645 = -1e0f64;
            let v768 = -5e-1f64;
            let v800 = parameters[30];
            let v802 = node_potentials[11];
            let v807 = -1e0f64;
            let v837 = -5e-1f64;
            let v885 = 1e-4f64;
            let v888 = 1e-8f64;
            let v898 = parameters[88];
            let v900 = parameters[89];
            let v923 = parameters[43];
            let v943 = parameters[32];
            let v960 = parameters[100];
            let v1065 = parameters[40];
            let v1069 = 1e-2f64;
            let v1087 = node_potentials[0];
            let v1126 = node_potentials[1];
            let v1134 = node_potentials[2];
            let v1158 = node_potentials[3];
            let v1167 = parameters[56];
            let v1168 = parameters[57];
            let v1172 = parameters[58];
            let v1174 = 1.44e0f64;
            let v1194 = parameters[61];
            let v1197 = parameters[22];
            let v1210 = parameters[87];
            let v1215 = parameters[15];
            let v1218 = parameters[20];
            let v1276 = parameters[83];
            let v1279 = parameters[84];
            let v1291 = 1e0f64;
            let v1292 = 1e0f64;
            let v1293 = 1e0f64;
            let v1294 = 1e0f64;
            let v1295 = 1e0f64;
            let v1296 = 1e0f64;
            let v1297 = 1e0f64;
            let v1298 = 1e0f64;
            let v1299 = 1e0f64;
            let v1300 = 1e0f64;
            let v1301 = 1e0f64;
            let v1302 = 1e0f64;
            let v1303 = 1e0f64;
            let v1396 = -1e0f64;
            let v1584 = 2e0f64;
            let v1713 = 0e0f64;
            let v1805 = Lanes([0e0f64; 3]);
            let v1887 = Lanes([0e0f64; 3]);
            let v2109 = Lanes([0e0f64; 3]);
            let v2329 = Lanes([0e0f64; 3]);
            let v2343 = Lanes([0e0f64; 3]);
            let v2511 = Lanes([0e0f64; 5]);
            let v2512 = Lanes([0e0f64; 6]);
            let v2769 = Lanes([0e0f64; 4]);
            let v2802 = Lanes([0e0f64; 3]);
            let v2836 = Lanes([0e0f64; 4]);
            let v2891 = Lanes([0e0f64; 3]);
            let v2900 = Lanes([0e0f64; 5]);
            let v2913 = Lanes([0e0f64; 3]);
            let v2922 = Lanes([0e0f64; 6]);
            let v2960 = Lanes([0e0f64; 3]);
            let v3162 = ddt_scale();
            let v3 = v1 + v2;
            let v8 = (v4 + v5) + v7;
            let v12 = (v9 * v8) / v11;
            let v1350 = (v1292 * v9) / v11;
            let v13 = v8 / v3;
            let v1351 = v1292 / v3;
            let v14 = v8 - v3;
            let v18 = v15 * (v13.powf(v16));
            let v1356 = (v1351 * (v16 * (v13.powf((v16 - v1291))))) * v15;
            let v22 = v19 * (v13.powf(v20));
            let v1361 = (v1351 * (v20 * (v13.powf((v20 - v1291))))) * v19;
            let v26 = v23 * (v13.powf(v24));
            let v1366 = (v1351 * (v24 * (v13.powf((v24 - v1291))))) * v23;
            let v30 = v27 * (v13.powf(v28));
            let v1371 = (v1351 * (v28 * (v13.powf((v28 - v1291))))) * v27;
            let v34 = v31 * (v13.powf(v32));
            let v1376 = (v1351 * (v32 * (v13.powf((v32 - v1291))))) * v31;
            let v38 = v35 * (v13.powf(v36));
            let v1381 = (v1351 * (v36 * (v13.powf((v36 - v1291))))) * v35;
            let v42 = v39 * (v13.powf(v40));
            let v1386 = (v1351 * (v40 * (v13.powf((v40 - v1291))))) * v39;
            let v46 = v43 * (v13.powf(v44));
            let v1391 = (v1351 * (v44 * (v13.powf((v44 - v1291))))) * v43;
            let v49 = v13.powf(v48);
            let v1395 = v1351 * (v48 * (v13.powf((v48 - v1291))));
            let v51 = -v50;
            let v53 = v52 - v13;
            let v1397 = v1351 * v1396;
            let v55 = (v51 * v53) / v12;
            let v56 = v55.exp();
            let v1402 = (((v1397 * v51) - (v1350 * v55)) / v12) * v56;
            let v57 = v49 * v56;
            let v59 = v52 / v58;
            let v61 = v47 * (v57.powf(v59));
            let v1410 = (((v1395 * v56) + (v1402 * v49)) * (v59 * (v57.powf((v59 - v1291))))) * v47;
            let v64 = v13.powf(v63);
            let v66 = -v65;
            let v68 = (v66 * v53) / v12;
            let v69 = v68.exp();
            let v70 = v64 * v69;
            let v72 = v52 / v71;
            let v74 = v62 * (v70.powf(v72));
            let v1427 = ((((v1351 * (v63 * (v13.powf((v63 - v1291))))) * v69) + (((((v1397 * v66) - (v1350 * v68)) / v12) * v69) * v64)) * (v72 * (v70.powf((v72 - v1291))))) * v62;
            let v77 = -v76;
            let v79 = (v77 * v53) / v12;
            let v80 = v79.exp();
            let v81 = v49 * v80;
            let v83 = v52 / v82;
            let v85 = v75 * (v81.powf(v83));
            let v1440 = (((v1395 * v80) + (((((v1397 * v77) - (v1350 * v79)) / v12) * v80) * v49)) * (v83 * (v81.powf((v83 - v1291))))) * v75;
            let v88 = v13.powf(v87);
            let v1444 = v1351 * (v87 * (v13.powf((v87 - v1291))));
            let v90 = -v89;
            let v92 = (v90 * v53) / v12;
            let v93 = v92.exp();
            let v94 = v88 * v93;
            let v96 = v52 / v95;
            let v98 = v86 * (v94.powf(v96));
            let v1457 = (((v1444 * v93) + (((((v1397 * v90) - (v1350 * v92)) / v12) * v93) * v88)) * (v96 * (v94.powf((v96 - v1291))))) * v86;
            let v101 = v13.powf(v100);
            let v1461 = v1351 * (v100 * (v13.powf((v100 - v1291))));
            let v103 = -v102;
            let v105 = (v103 * v53) / v12;
            let v106 = v105.exp();
            let v107 = v101 * v106;
            let v109 = v52 / v108;
            let v111 = v99 * (v107.powf(v109));
            let v1474 = (((v1461 * v106) + (((((v1397 * v103) - (v1350 * v105)) / v12) * v106) * v101)) * (v109 * (v107.powf((v109 - v1291))))) * v99;
            let v114 = -v113;
            let v116 = (v114 * v53) / v12;
            let v117 = v116.exp();
            let v118 = v88 * v117;
            let v1480 = v1444 * v117;
            let v1481 = ((((v1397 * v114) - (v1350 * v116)) / v12) * v117) * v88;
            let v120 = v52 / v119;
            let v121 = v118.powf(v120);
            let v1485 = v120 * (v118.powf((v120 - v1291)));
            let v122 = v112 * v121;
            let v1487 = ((v1480 + v1481) * v1485) * v112;
            let v125 = -v124;
            let v127 = (v125 * v53) / v12;
            let v128 = v127.exp();
            let v129 = v101 * v128;
            let v1493 = v1461 * v128;
            let v1494 = ((((v1397 * v125) - (v1350 * v127)) / v12) * v128) * v101;
            let v131 = v52 / v130;
            let v132 = v129.powf(v131);
            let v1498 = v131 * (v129.powf((v131 - v1291)));
            let v133 = v123 * v132;
            let v1500 = ((v1493 + v1494) * v1498) * v123;
            let v135 = v134 * v121;
            let v1503 = ((v1480 + v1481) * v1485) * v134;
            let v137 = v136 * v132;
            let v1506 = ((v1493 + v1494) * v1498) * v136;
            let v140 = -v139;
            let v142 = (v140 * v53) / v12;
            let v143 = v142.exp();
            let v144 = v88 * v143;
            let v146 = v52 / v145;
            let v148 = v138 * (v144.powf(v146));
            let v1519 = (((v1444 * v143) + (((((v1397 * v140) - (v1350 * v142)) / v12) * v143) * v88)) * (v146 * (v144.powf((v146 - v1291))))) * v138;
            let v151 = -v150;
            let v153 = (v151 * v53) / v12;
            let v154 = v153.exp();
            let v155 = v101 * v154;
            let v157 = v52 / v156;
            let v159 = v149 * (v155.powf(v157));
            let v1532 = (((v1461 * v154) + (((((v1397 * v151) - (v1350 * v153)) / v12) * v154) * v101)) * (v157 * (v155.powf((v157 - v1291))))) * v149;
            let v1533 = v1292 * v160;
            let v162 = v52 + (v14 * v160);
            let v163 = v58 * v162;
            let v1534 = v1533 * v58;
            let v164 = v71 * v162;
            let v1535 = v1533 * v71;
            let v169 = v165 * (v52 + (v14 * v166));
            let v1537 = (v1292 * v166) * v165;
            let v174 = v171 + (v14 * v172);
            let v182 = v178 * (v52 + (v14 * v179));
            let v184 = v12 / v13;
            let v185 = v183 * v184;
            let v1548 = ((v1350 - (v1351 * v184)) / v13) * v183;
            let v188 = v186 * v187;
            let v190 = (v188 * v13) / v12;
            let v191 = v190.exp();
            let v193 = v192 * v187;
            let v195 = (v193 * v13) / v12;
            let v196 = v195.exp();
            let v197 = v191 - v196;
            let v198 = v197.ln();
            let v199 = v185 * v198;
            let v202 = v201 * v12;
            let v203 = v13.ln();
            let v204 = v202 * v203;
            let v1573 = ((v1350 * v201) * v203) + ((v1351 * (v1291 / v13)) * v202);
            let v206 = v13 - v52;
            let v208 = ((v199 * v13) - v204) - (v89 * v206);
            let v1576 = (((((v1548 * v198) + (((((((v1351 * v188) - (v1350 * v190)) / v12) * v191) - ((((v1351 * v193) - (v1350 * v195)) / v12) * v196)) * (v1291 / v197)) * v185)) * v13) + (v1351 * v199)) - v1573) - (v1351 * v89);
            let v209 = v183 * v12;
            let v1577 = v1350 * v183;
            let v212 = (-v208) / v12;
            let v213 = v212.exp();
            let v216 = (v52 + (v210 * v213)).sqrt();
            let v218 = v186 * (v52 + v216);
            let v219 = v218.ln();
            let v221 = v208 + (v209 * v219);
            let v1594 = v1576 + ((v1577 * v219) + (((((((((v1576 * v1396) - (v1350 * v212)) / v12) * v213) * v210) * (v1291 / (v1584 * v216))) * v186) * (v1291 / v218)) * v209));
            let v223 = v186 * v222;
            let v225 = (v223 * v13) / v12;
            let v226 = v225.exp();
            let v228 = v227 * v222;
            let v230 = (v228 * v13) / v12;
            let v231 = v230.exp();
            let v232 = v226 - v231;
            let v233 = v232.ln();
            let v234 = v185 * v233;
            let v238 = ((v234 * v13) - v204) - (v113 * v206);
            let v1616 = (((((v1548 * v233) + (((((((v1351 * v223) - (v1350 * v225)) / v12) * v226) - ((((v1351 * v228) - (v1350 * v230)) / v12) * v231)) * (v1291 / v232)) * v185)) * v13) + (v1351 * v234)) - v1573) - (v1351 * v113);
            let v240 = (-v238) / v12;
            let v241 = v240.exp();
            let v244 = (v52 + (v210 * v241)).sqrt();
            let v246 = v186 * (v52 + v244);
            let v247 = v246.ln();
            let v249 = v238 + (v209 * v247);
            let v1632 = v1616 + ((v1577 * v247) + (((((((((v1616 * v1396) - (v1350 * v240)) / v12) * v241) * v210) * (v1291 / (v1584 * v244))) * v186) * (v1291 / v246)) * v209));
            let v251 = v186 * v250;
            let v253 = (v251 * v13) / v12;
            let v254 = v253.exp();
            let v256 = v255 * v250;
            let v258 = (v256 * v13) / v12;
            let v259 = v258.exp();
            let v260 = v254 - v259;
            let v261 = v260.ln();
            let v262 = v185 * v261;
            let v266 = ((v262 * v13) - v204) - (v139 * v206);
            let v1654 = (((((v1548 * v261) + (((((((v1351 * v251) - (v1350 * v253)) / v12) * v254) - ((((v1351 * v256) - (v1350 * v258)) / v12) * v259)) * (v1291 / v260)) * v185)) * v13) + (v1351 * v262)) - v1573) - (v1351 * v139);
            let v268 = (-v266) / v12;
            let v269 = v268.exp();
            let v272 = (v52 + (v210 * v269)).sqrt();
            let v274 = v186 * (v52 + v272);
            let v275 = v274.ln();
            let v277 = v266 + (v209 * v275);
            let v1670 = v1654 + ((v1577 * v275) + (((((((((v1654 * v1396) - (v1350 * v268)) / v12) * v269) * v210) * (v1291 / (v1584 * v272))) * v186) * (v1291 / v274)) * v209));
            let v279 = v187 / v221;
            let v282 = v278 * (v279.powf(v280));
            let v1678 = ((((v1594 * v279) * v1396) / v221) * (v280 * (v279.powf((v280 - v1291))))) * v278;
            let v284 = v222 / v249;
            let v286 = v284.powf(v285);
            let v1685 = (((v1632 * v284) * v1396) / v249) * (v285 * (v284.powf((v285 - v1291))));
            let v287 = v283 * v286;
            let v1686 = v1685 * v283;
            let v289 = v288 * v286;
            let v1687 = v1685 * v288;
            let v291 = v250 / v277;
            let v294 = v290 * (v291.powf(v292));
            let v1695 = ((((v1670 * v291) * v1396) / v277) * (v292 * (v291.powf((v292 - v1291))))) * v290;
            let v296 = v295 * v49;
            let v297 = v296 * v56;
            let v1699 = ((v1395 * v295) * v56) + (v1402 * v296);
            let v301 = v298 * (v13.powf(v299));
            let v1704 = (v1351 * (v299 * (v13.powf((v299 - v1291))))) * v298;
            let v302 = -(v170 * (v52 + (v14 * v174)));
            let v1705 = (((v1292 * v174) + ((v1292 * v172) * v14)) * v170) * v1396;
            let v303 = v182 * v12;
            let v1708 = (((v1292 * v179) * v178) * v12) + (v1350 * v182);
            let v304 = v302 / v303;
            let v305 = v304.exp();
            let v1712 = ((v1705 - (v1708 * v304)) / v303) * v305;
            let v307 = if v306 > v0 { 1.0 } else { 0.0 };
            let v309: f64;
            if v307 != 0.0 {
                let v308 = v52 / v306;
                v309 = v308;
            } else {
                v309 = v0;
            }
            let v311 = if v310 > v0 { 1.0 } else { 0.0 };
            let v313: f64;
            if v311 != 0.0 {
                let v312 = v52 / v310;
                v313 = v312;
            } else {
                v313 = v0;
            }
            let v314 = if v15 > v0 { 1.0 } else { 0.0 };
            let v316: f64;
            let v1304: f64;
            if v314 != 0.0 {
                let v315 = v52 / v18;
                let v1716 = ((v1356 * v315) * v1396) / v18;
                v316 = v315;
                v1304 = v1716;
            } else {
                v316 = v0;
                v1304 = v1713;
            }
            let v318 = if v317 > v0 { 1.0 } else { 0.0 };
            let v320: f64;
            if v318 != 0.0 {
                let v319 = v52 / v317;
                v320 = v319;
            } else {
                v320 = v0;
            }
            let v322 = if v321 > v0 { 1.0 } else { 0.0 };
            let v324: f64;
            if v322 != 0.0 {
                let v323 = v52 / v321;
                v324 = v323;
            } else {
                v324 = v0;
            }
            let v325 = if v298 > v0 { 1.0 } else { 0.0 };
            let v327: f64;
            let v1305: f64;
            if v325 != 0.0 {
                let v326 = v52 / v301;
                let v1719 = ((v1704 * v326) * v1396) / v301;
                v327 = v326;
                v1305 = v1719;
            } else {
                v327 = v0;
                v1305 = v1713;
            }
            let v329 = if v328 > v0 { 1.0 } else { 0.0 };
            let v331: f64;
            if v329 != 0.0 {
                let v330 = v52 / v328;
                v331 = v330;
            } else {
                v331 = v0;
            }
            let v333 = if v332 > v0 { 1.0 } else { 0.0 };
            let v335: f64;
            if v333 != 0.0 {
                let v334 = v52 / v332;
                v335 = v334;
            } else {
                v335 = v0;
            }
            let v337 = if v336 > v0 { 1.0 } else { 0.0 };
            let v339: f64;
            if v337 != 0.0 {
                let v338 = v52 / v336;
                v339 = v338;
            } else {
                v339 = v0;
            }
            let v340: f64;
            if v337 != 0.0 {
                v340 = v0;
            } else {
                v340 = v52;
            }
            let v343 = v341 - v342;
            let v1722 = (Lanes([v1293, 0.0])) - (Lanes([0.0, v1294]));
            let v345 = v344 - v342;
            let v1725 = (Lanes([v1295, 0.0])) - (Lanes([0.0, v1294]));
            let v347 = v341 - v346;
            let v1728 = (Lanes([0.0, v1293])) - (Lanes([v1296, 0.0]));
            let v349 = v341 - v348;
            let v1731 = (Lanes([0.0, v1293])) - (Lanes([v1297, 0.0]));
            let v351 = v344 - v350;
            let v1734 = (Lanes([v1295, 0.0])) - (Lanes([0.0, v1298]));
            let v352 = -v221;
            let v1735 = v1594 * v1396;
            let v354 = v352 * v353;
            let v1736 = v1735 * v353;
            let v356 = if v355 <= v0 { 1.0 } else { 0.0 };
            let v878: f64;
            let v1306: Lanes<3>;
            if v356 != 0.0 {
                let v357 = v343 + v354;
                let v1787 = Lanes([0.0, v1722[0], v1722[1]]);
                let v1789 = v1787 + (Lanes([v1736, 0.0, 0.0]));
                let v358 = if v357 > v0 { 1.0 } else { 0.0 };
                let v382: f64;
                let v383: f64;
                let v1307: Lanes<3>;
                let v1308: Lanes<3>;
                if v358 != 0.0 {
                    let v359 = v52 - v353;
                    let v362 = v359.powf((v360 - v280));
                    let v365 = v52 - ((v362 * v359) * v359);
                    let v367 = v52 - v280;
                    let v368 = (v221 * v365) / v367;
                    let v369 = v186 * v280;
                    let v371 = (v369 * v357) / v221;
                    let v372 = v359 + v371;
                    let v374 = (v357 * v372) * v362;
                    let v1816 = ((v1789 * v372) + ((((v1789 * v369) - (Lanes([(v1594 * v371), 0.0, 0.0]))) / v221) * v357)) * v362;
                    let v1817 = Lanes([((v1594 * v365) / v367), 0.0, 0.0]);
                    v382 = v368;
                    v383 = v374;
                    v1307 = v1817;
                    v1308 = v1816;
                } else {
                    let v375 = v343 / v221;
                    let v376 = v52 - v375;
                    let v377 = v52 - v280;
                    let v379 = v52 - (v376.powf(v377));
                    let v381 = (v221 * v379) / v377;
                    let v1804 = ((Lanes([(v1594 * v379), 0.0, 0.0])) + ((((((v1787 - (Lanes([(v1594 * v375), 0.0, 0.0]))) / v221) * v1396) * (v377 * (v376.powf((v377 - v1291))))) * v1396) * v221)) / v377;
                    v382 = v381;
                    v383 = v0;
                    v1307 = v1804;
                    v1308 = v1805;
                }
                let v384 = v382 + v383;
                let v1818 = v1307 + v1308;
                v878 = v384;
                v1306 = v1818;
            } else {
                let v1737 = v1736 * v354;
                let v387 = (v210 * v355) * v355;
                let v389 = ((v354 * v354) + v387).sqrt();
                let v392 = v390 * (v354 + v389);
                let v1743 = (v1736 + ((v1737 + v1737) * (v1291 / (v1584 * v389)))) * v390;
                let v393 = v392 / v221;
                let v394 = v52 - v393;
                let v395 = v52 - v280;
                let v396 = v394.powf(v395);
                let v1748 = v395 - v1291;
                let v399 = v343 + v354;
                let v1756 = Lanes([0.0, v1722[0], v1722[1]]);
                let v1757 = Lanes([v1736, 0.0, 0.0]);
                let v1758 = v1756 + v1757;
                let v1759 = v1758 * v399;
                let v402 = ((v399 * v399) + v387).sqrt();
                let v405 = (v186 * (v399 - v402)) - v354;
                let v1766 = ((v1758 - ((v1759 + v1759) * (v1291 / (v1584 * v402)))) * v186) - v1757;
                let v406 = v405 / v221;
                let v407 = v52 - v406;
                let v408 = v407.powf(v395);
                let v413 = (v52 - v353).powf((-v280));
                let v418 = (((v352 * v408) / v395) + (v413 * ((v343 - v405) + v392))) - ((v352 * v396) / v395);
                let v1786 = ((((Lanes([(v1735 * v408), 0.0, 0.0])) + (((((v1766 - (Lanes([(v1594 * v406), 0.0, 0.0]))) / v221) * v1396) * (v395 * (v407.powf(v1748)))) * v352)) / v395) + (((v1756 - v1766) + (Lanes([v1743, 0.0, 0.0]))) * v413)) - (Lanes([(((v1735 * v396) + (((((v1743 - (v1594 * v393)) / v221) * v1396) * (v395 * (v394.powf(v1748)))) * v352)) / v395), 0.0, 0.0]));
                v878 = v418;
                v1306 = v1786;
            }
            let v1189: f64;
            let v1309: Lanes<3>;
            if v356 != 0.0 {
                let v419 = v345 + v354;
                let v1869 = Lanes([0.0, v1725[0], v1725[1]]);
                let v1871 = v1869 + (Lanes([v1736, 0.0, 0.0]));
                let v420 = if v419 > v0 { 1.0 } else { 0.0 };
                let v444: f64;
                let v445: f64;
                let v1310: Lanes<3>;
                let v1311: Lanes<3>;
                if v420 != 0.0 {
                    let v421 = v52 - v353;
                    let v424 = v421.powf((v422 - v280));
                    let v427 = v52 - ((v424 * v421) * v421);
                    let v429 = v52 - v280;
                    let v430 = (v221 * v427) / v429;
                    let v431 = v186 * v280;
                    let v433 = (v431 * v419) / v221;
                    let v434 = v421 + v433;
                    let v436 = (v419 * v434) * v424;
                    let v1898 = ((v1871 * v434) + ((((v1871 * v431) - (Lanes([(v1594 * v433), 0.0, 0.0]))) / v221) * v419)) * v424;
                    let v1899 = Lanes([((v1594 * v427) / v429), 0.0, 0.0]);
                    v444 = v430;
                    v445 = v436;
                    v1310 = v1899;
                    v1311 = v1898;
                } else {
                    let v437 = v345 / v221;
                    let v438 = v52 - v437;
                    let v439 = v52 - v280;
                    let v441 = v52 - (v438.powf(v439));
                    let v443 = (v221 * v441) / v439;
                    let v1886 = ((Lanes([(v1594 * v441), 0.0, 0.0])) + ((((((v1869 - (Lanes([(v1594 * v437), 0.0, 0.0]))) / v221) * v1396) * (v439 * (v438.powf((v439 - v1291))))) * v1396) * v221)) / v439;
                    v444 = v443;
                    v445 = v0;
                    v1310 = v1886;
                    v1311 = v1887;
                }
                let v446 = v444 + v445;
                let v1900 = v1310 + v1311;
                v1189 = v446;
                v1309 = v1900;
            } else {
                let v1819 = v1736 * v354;
                let v449 = (v210 * v355) * v355;
                let v451 = ((v354 * v354) + v449).sqrt();
                let v454 = v452 * (v354 + v451);
                let v1825 = (v1736 + ((v1819 + v1819) * (v1291 / (v1584 * v451)))) * v452;
                let v455 = v454 / v221;
                let v456 = v52 - v455;
                let v457 = v52 - v280;
                let v458 = v456.powf(v457);
                let v1830 = v457 - v1291;
                let v461 = v345 + v354;
                let v1838 = Lanes([0.0, v1725[0], v1725[1]]);
                let v1839 = Lanes([v1736, 0.0, 0.0]);
                let v1840 = v1838 + v1839;
                let v1841 = v1840 * v461;
                let v464 = ((v461 * v461) + v449).sqrt();
                let v467 = (v186 * (v461 - v464)) - v354;
                let v1848 = ((v1840 - ((v1841 + v1841) * (v1291 / (v1584 * v464)))) * v186) - v1839;
                let v468 = v467 / v221;
                let v469 = v52 - v468;
                let v470 = v469.powf(v457);
                let v475 = (v52 - v353).powf((-v280));
                let v480 = (((v352 * v470) / v457) + (v475 * ((v345 - v467) + v454))) - ((v352 * v458) / v457);
                let v1868 = ((((Lanes([(v1735 * v470), 0.0, 0.0])) + (((((v1848 - (Lanes([(v1594 * v468), 0.0, 0.0]))) / v221) * v1396) * (v457 * (v469.powf(v1830)))) * v352)) / v457) + (((v1838 - v1848) + (Lanes([v1825, 0.0, 0.0]))) * v475)) - (Lanes([(((v1735 * v458) + (((((v1825 - (v1594 * v455)) / v221) * v1396) * (v457 * (v456.powf(v1830)))) * v352)) / v457), 0.0, 0.0]));
                v1189 = v480;
                v1309 = v1868;
            }
            let v481 = -v249;
            let v1901 = v1632 * v1396;
            let v482 = v481 * v353;
            let v1902 = v1901 * v353;
            let v484 = if v483 <= v0 { 1.0 } else { 0.0 };
            let v881: f64;
            let v1312: Lanes<3>;
            if v484 != 0.0 {
                let v485 = v347 + v482;
                let v2067 = Lanes([0.0, v1728[0], v1728[1]]);
                let v2069 = v2067 + (Lanes([v1902, 0.0, 0.0]));
                let v486 = if v485 > v0 { 1.0 } else { 0.0 };
                let v528: f64;
                let v530: f64;
                let v1313: Lanes<3>;
                let v1314: Lanes<3>;
                if v486 != 0.0 {
                    let v487 = v52 - v353;
                    let v490 = v487.powf((v488 - v285));
                    let v493 = v52 - ((v490 * v487) * v487);
                    let v495 = v52 - v285;
                    let v496 = (v249 * v493) / v495;
                    let v497 = v186 * v285;
                    let v499 = (v497 * v485) / v249;
                    let v500 = v487 + v499;
                    let v502 = (v485 * v500) * v490;
                    let v2120 = ((v2069 * v500) + ((((v2069 * v497) - (Lanes([(v1632 * v499), 0.0, 0.0]))) / v249) * v485)) * v490;
                    let v2121 = Lanes([((v1632 * v493) / v495), 0.0, 0.0]);
                    v528 = v496;
                    v530 = v502;
                    v1313 = v2121;
                    v1314 = v2120;
                } else {
                    let v507 = if (if v503 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v347 < (-v503) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v529: f64;
                    let v1315: Lanes<3>;
                    if v507 != 0.0 {
                        let v508 = v503 / v249;
                        let v509 = v52 + v508;
                        let v510 = v52 - v285;
                        let v511 = v509.powf(v510);
                        let v2092 = v1728 * v510;
                        let v514 = v249 + v503;
                        let v515 = (v510 * (v347 + v503)) / v514;
                        let v516 = v52 - v515;
                        let v518 = v52 - (v511 * v516);
                        let v520 = (v249 * v518) / v510;
                        let v2108 = ((Lanes([(v1632 * v518), 0.0, 0.0])) + ((((Lanes([(((((v1632 * v508) * v1396) / v249) * (v510 * (v509.powf((v510 - v1291))))) * v516), 0.0, 0.0])) + (((((Lanes([0.0, v2092[0], v2092[1]])) - (Lanes([(v1632 * v515), 0.0, 0.0]))) / v514) * v1396) * v511)) * v1396) * v249)) / v510;
                        v529 = v520;
                        v1315 = v2108;
                    } else {
                        let v521 = v347 / v249;
                        let v522 = v52 - v521;
                        let v523 = v52 - v285;
                        let v525 = v52 - (v522.powf(v523));
                        let v527 = (v249 * v525) / v523;
                        let v2084 = ((Lanes([(v1632 * v525), 0.0, 0.0])) + ((((((v2067 - (Lanes([(v1632 * v521), 0.0, 0.0]))) / v249) * v1396) * (v523 * (v522.powf((v523 - v1291))))) * v1396) * v249)) / v523;
                        v529 = v527;
                        v1315 = v2084;
                    }
                    v528 = v529;
                    v530 = v0;
                    v1313 = v1315;
                    v1314 = v2109;
                }
                let v531 = v528 + v530;
                let v2122 = v1313 + v1314;
                v881 = v531;
                v1312 = v2122;
            } else {
                let v535 = if (if v503 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v533 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v882: f64;
                let v1316: Lanes<3>;
                if v535 != 0.0 {
                    let v537 = v503 - v482;
                    let v1953 = v1902 * v1396;
                    let v538 = (v503 + v482) / v537;
                    let v1956 = (v1902 - (v1953 * v538)) / v537;
                    let v540 = v538 - v52;
                    let v1958 = v1956 * v540;
                    let v543 = (v210 * v483) * v483;
                    let v545 = ((v540 * v540) + v543).sqrt();
                    let v546 = v538 + v52;
                    let v1963 = v1956 * v546;
                    let v549 = (v210 * v533) * v533;
                    let v551 = ((v546 * v546) + v549).sqrt();
                    let v552 = v545 + v551;
                    let v553 = (v183 * v538) / v552;
                    let v557 = v186 * (((v553 * v537) - v503) - v482);
                    let v1976 = ((((((v1956 * v183) - ((((v1958 + v1958) * (v1291 / (v1584 * v545))) + ((v1963 + v1963) * (v1291 / (v1584 * v551)))) * v553)) / v552) * v537) + (v1953 * v553)) - v1902) * v186;
                    let v558 = v557 / v249;
                    let v559 = v52 - v558;
                    let v560 = v52 - v285;
                    let v1981 = v560 - v1291;
                    let v562 = v52 - (v559.powf(v560));
                    let v1990 = v1728 * v183;
                    let v1992 = Lanes([v1902, 0.0, 0.0]);
                    let v568 = (((v183 * v347) + v503) + v482) / v537;
                    let v1997 = (((Lanes([0.0, v1990[0], v1990[1]])) + v1992) - (Lanes([(v1953 * v568), 0.0, 0.0]))) / v537;
                    let v570 = v568 - v52;
                    let v1999 = v1997 * v570;
                    let v573 = ((v570 * v570) + v543).sqrt();
                    let v574 = v568 + v52;
                    let v2004 = v1997 * v574;
                    let v577 = ((v574 * v574) + v549).sqrt();
                    let v578 = v573 + v577;
                    let v579 = (v183 * v568) / v578;
                    let v2012 = ((v1997 * v183) - ((((v1999 + v1999) * (v1291 / (v1584 * v573))) + ((v2004 + v2004) * (v1291 / (v1584 * v577)))) * v579)) / v578;
                    let v583 = v186 * (((v579 * v537) - v503) - v482);
                    let v2018 = (((v2012 * v537) + (Lanes([(v1953 * v579), 0.0, 0.0]))) - v1992) * v186;
                    let v584 = v583 / v249;
                    let v585 = v52 - v584;
                    let v587 = v52 - (v585.powf(v560));
                    let v591 = v186 * (v579 + v52);
                    let v2033 = v2012 * v186;
                    let v592 = v503 / v249;
                    let v593 = v52 + v592;
                    let v594 = -v285;
                    let v595 = v593.powf(v594);
                    let v2037 = v594 - v1291;
                    let v596 = v482 / v249;
                    let v597 = v52 + v596;
                    let v598 = v597.powf(v594);
                    let v599 = v52 - v591;
                    let v602 = (v599 * v595) + (v591 * v598);
                    let v604 = (v347 - v583) + v557;
                    let v607 = ((v604 * v602) + ((v249 * v587) / v560)) - ((v249 * v562) / v560);
                    let v2066 = ((((((Lanes([0.0, v1728[0], v1728[1]])) - v2018) + (Lanes([v1976, 0.0, 0.0]))) * v602) + (((((v2033 * v1396) * v595) + (Lanes([(((((v1632 * v592) * v1396) / v249) * (v594 * (v593.powf(v2037)))) * v599), 0.0, 0.0]))) + ((v2033 * v598) + (Lanes([((((v1902 - (v1632 * v596)) / v249) * (v594 * (v597.powf(v2037)))) * v591), 0.0, 0.0])))) * v604)) + (((Lanes([(v1632 * v587), 0.0, 0.0])) + ((((((v2018 - (Lanes([(v1632 * v584), 0.0, 0.0]))) / v249) * v1396) * (v560 * (v585.powf(v1981)))) * v1396) * v249)) / v560)) - (Lanes([(((v1632 * v562) + ((((((v1976 - (v1632 * v558)) / v249) * v1396) * (v560 * (v559.powf(v1981)))) * v1396) * v249)) / v560), 0.0, 0.0]));
                    v882 = v607;
                    v1316 = v2066;
                } else {
                    let v1903 = v1902 * v482;
                    let v610 = (v210 * v483) * v483;
                    let v612 = ((v482 * v482) + v610).sqrt();
                    let v615 = v613 * (v482 + v612);
                    let v1909 = (v1902 + ((v1903 + v1903) * (v1291 / (v1584 * v612)))) * v613;
                    let v616 = v615 / v249;
                    let v617 = v52 - v616;
                    let v618 = v52 - v285;
                    let v619 = v617.powf(v618);
                    let v1914 = v618 - v1291;
                    let v622 = v347 + v482;
                    let v1922 = Lanes([0.0, v1728[0], v1728[1]]);
                    let v1923 = Lanes([v1902, 0.0, 0.0]);
                    let v1924 = v1922 + v1923;
                    let v1925 = v1924 * v622;
                    let v625 = ((v622 * v622) + v610).sqrt();
                    let v628 = (v186 * (v622 - v625)) - v482;
                    let v1932 = ((v1924 - ((v1925 + v1925) * (v1291 / (v1584 * v625)))) * v186) - v1923;
                    let v629 = v628 / v249;
                    let v630 = v52 - v629;
                    let v631 = v630.powf(v618);
                    let v636 = (v52 - v353).powf((-v285));
                    let v641 = (((v481 * v631) / v618) + (v636 * ((v347 - v628) + v615))) - ((v481 * v619) / v618);
                    let v1952 = ((((Lanes([(v1901 * v631), 0.0, 0.0])) + (((((v1932 - (Lanes([(v1632 * v629), 0.0, 0.0]))) / v249) * v1396) * (v618 * (v630.powf(v1914)))) * v481)) / v618) + (((v1922 - v1932) + (Lanes([v1909, 0.0, 0.0]))) * v636)) - (Lanes([(((v1901 * v619) + (((((v1909 - (v1632 * v616)) / v249) * v1396) * (v618 * (v617.powf(v1914)))) * v481)) / v618), 0.0, 0.0]));
                    v882 = v641;
                    v1316 = v1952;
                }
                v881 = v882;
                v1312 = v1316;
            }
            let v1201: f64;
            let v1317: Lanes<3>;
            if v484 != 0.0 {
                let v642 = v351 + v482;
                let v2287 = Lanes([0.0, v1734[0], v1734[1]]);
                let v2289 = v2287 + (Lanes([v1902, 0.0, 0.0]));
                let v643 = if v642 > v0 { 1.0 } else { 0.0 };
                let v684: f64;
                let v686: f64;
                let v1318: Lanes<3>;
                let v1319: Lanes<3>;
                if v643 != 0.0 {
                    let v644 = v52 - v353;
                    let v647 = v644.powf((v645 - v285));
                    let v650 = v52 - ((v647 * v644) * v644);
                    let v652 = v52 - v285;
                    let v653 = (v249 * v650) / v652;
                    let v654 = v186 * v285;
                    let v656 = (v654 * v642) / v249;
                    let v657 = v644 + v656;
                    let v659 = (v642 * v657) * v647;
                    let v2340 = ((v2289 * v657) + ((((v2289 * v654) - (Lanes([(v1632 * v656), 0.0, 0.0]))) / v249) * v642)) * v647;
                    let v2341 = Lanes([((v1632 * v650) / v652), 0.0, 0.0]);
                    v684 = v653;
                    v686 = v659;
                    v1318 = v2341;
                    v1319 = v2340;
                } else {
                    let v663 = if (if v503 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v351 < (-v503) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v685: f64;
                    let v1320: Lanes<3>;
                    if v663 != 0.0 {
                        let v664 = v503 / v249;
                        let v665 = v52 + v664;
                        let v666 = v52 - v285;
                        let v667 = v665.powf(v666);
                        let v2312 = v1734 * v666;
                        let v670 = v249 + v503;
                        let v671 = (v666 * (v351 + v503)) / v670;
                        let v672 = v52 - v671;
                        let v674 = v52 - (v667 * v672);
                        let v676 = (v249 * v674) / v666;
                        let v2328 = ((Lanes([(v1632 * v674), 0.0, 0.0])) + ((((Lanes([(((((v1632 * v664) * v1396) / v249) * (v666 * (v665.powf((v666 - v1291))))) * v672), 0.0, 0.0])) + (((((Lanes([0.0, v2312[0], v2312[1]])) - (Lanes([(v1632 * v671), 0.0, 0.0]))) / v670) * v1396) * v667)) * v1396) * v249)) / v666;
                        v685 = v676;
                        v1320 = v2328;
                    } else {
                        let v677 = v351 / v249;
                        let v678 = v52 - v677;
                        let v679 = v52 - v285;
                        let v681 = v52 - (v678.powf(v679));
                        let v683 = (v249 * v681) / v679;
                        let v2304 = ((Lanes([(v1632 * v681), 0.0, 0.0])) + ((((((v2287 - (Lanes([(v1632 * v677), 0.0, 0.0]))) / v249) * v1396) * (v679 * (v678.powf((v679 - v1291))))) * v1396) * v249)) / v679;
                        v685 = v683;
                        v1320 = v2304;
                    }
                    v684 = v685;
                    v686 = v0;
                    v1318 = v1320;
                    v1319 = v2329;
                }
                let v687 = v684 + v686;
                let v2342 = v1318 + v1319;
                v1201 = v687;
                v1317 = v2342;
            } else {
                let v690 = if (if v503 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v533 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1202: f64;
                let v1321: Lanes<3>;
                if v690 != 0.0 {
                    let v692 = v503 - v482;
                    let v2173 = v1902 * v1396;
                    let v693 = (v503 + v482) / v692;
                    let v2176 = (v1902 - (v2173 * v693)) / v692;
                    let v695 = v693 - v52;
                    let v2178 = v2176 * v695;
                    let v698 = (v210 * v483) * v483;
                    let v700 = ((v695 * v695) + v698).sqrt();
                    let v701 = v693 + v52;
                    let v2183 = v2176 * v701;
                    let v704 = (v210 * v533) * v533;
                    let v706 = ((v701 * v701) + v704).sqrt();
                    let v707 = v700 + v706;
                    let v708 = (v183 * v693) / v707;
                    let v712 = v186 * (((v708 * v692) - v503) - v482);
                    let v2196 = ((((((v2176 * v183) - ((((v2178 + v2178) * (v1291 / (v1584 * v700))) + ((v2183 + v2183) * (v1291 / (v1584 * v706)))) * v708)) / v707) * v692) + (v2173 * v708)) - v1902) * v186;
                    let v713 = v712 / v249;
                    let v714 = v52 - v713;
                    let v715 = v52 - v285;
                    let v2201 = v715 - v1291;
                    let v717 = v52 - (v714.powf(v715));
                    let v2210 = v1734 * v183;
                    let v2212 = Lanes([v1902, 0.0, 0.0]);
                    let v723 = (((v183 * v351) + v503) + v482) / v692;
                    let v2217 = (((Lanes([0.0, v2210[0], v2210[1]])) + v2212) - (Lanes([(v2173 * v723), 0.0, 0.0]))) / v692;
                    let v725 = v723 - v52;
                    let v2219 = v2217 * v725;
                    let v728 = ((v725 * v725) + v698).sqrt();
                    let v729 = v723 + v52;
                    let v2224 = v2217 * v729;
                    let v732 = ((v729 * v729) + v704).sqrt();
                    let v733 = v728 + v732;
                    let v734 = (v183 * v723) / v733;
                    let v2232 = ((v2217 * v183) - ((((v2219 + v2219) * (v1291 / (v1584 * v728))) + ((v2224 + v2224) * (v1291 / (v1584 * v732)))) * v734)) / v733;
                    let v738 = v186 * (((v734 * v692) - v503) - v482);
                    let v2238 = (((v2232 * v692) + (Lanes([(v2173 * v734), 0.0, 0.0]))) - v2212) * v186;
                    let v739 = v738 / v249;
                    let v740 = v52 - v739;
                    let v742 = v52 - (v740.powf(v715));
                    let v746 = v186 * (v734 + v52);
                    let v2253 = v2232 * v186;
                    let v747 = v503 / v249;
                    let v748 = v52 + v747;
                    let v749 = -v285;
                    let v750 = v748.powf(v749);
                    let v2257 = v749 - v1291;
                    let v751 = v482 / v249;
                    let v752 = v52 + v751;
                    let v753 = v752.powf(v749);
                    let v754 = v52 - v746;
                    let v757 = (v754 * v750) + (v746 * v753);
                    let v759 = (v351 - v738) + v712;
                    let v762 = ((v759 * v757) + ((v249 * v742) / v715)) - ((v249 * v717) / v715);
                    let v2286 = ((((((Lanes([0.0, v1734[0], v1734[1]])) - v2238) + (Lanes([v2196, 0.0, 0.0]))) * v757) + (((((v2253 * v1396) * v750) + (Lanes([(((((v1632 * v747) * v1396) / v249) * (v749 * (v748.powf(v2257)))) * v754), 0.0, 0.0]))) + ((v2253 * v753) + (Lanes([((((v1902 - (v1632 * v751)) / v249) * (v749 * (v752.powf(v2257)))) * v746), 0.0, 0.0])))) * v759)) + (((Lanes([(v1632 * v742), 0.0, 0.0])) + ((((((v2238 - (Lanes([(v1632 * v739), 0.0, 0.0]))) / v249) * v1396) * (v715 * (v740.powf(v2201)))) * v1396) * v249)) / v715)) - (Lanes([(((v1632 * v717) + ((((((v2196 - (v1632 * v713)) / v249) * v1396) * (v715 * (v714.powf(v2201)))) * v1396) * v249)) / v715), 0.0, 0.0]));
                    v1202 = v762;
                    v1321 = v2286;
                } else {
                    let v2123 = v1902 * v482;
                    let v765 = (v210 * v483) * v483;
                    let v767 = ((v482 * v482) + v765).sqrt();
                    let v770 = v768 * (v482 + v767);
                    let v2129 = (v1902 + ((v2123 + v2123) * (v1291 / (v1584 * v767)))) * v768;
                    let v771 = v770 / v249;
                    let v772 = v52 - v771;
                    let v773 = v52 - v285;
                    let v774 = v772.powf(v773);
                    let v2134 = v773 - v1291;
                    let v777 = v351 + v482;
                    let v2142 = Lanes([0.0, v1734[0], v1734[1]]);
                    let v2143 = Lanes([v1902, 0.0, 0.0]);
                    let v2144 = v2142 + v2143;
                    let v2145 = v2144 * v777;
                    let v780 = ((v777 * v777) + v765).sqrt();
                    let v783 = (v186 * (v777 - v780)) - v482;
                    let v2152 = ((v2144 - ((v2145 + v2145) * (v1291 / (v1584 * v780)))) * v186) - v2143;
                    let v784 = v783 / v249;
                    let v785 = v52 - v784;
                    let v786 = v785.powf(v773);
                    let v791 = (v52 - v353).powf((-v285));
                    let v796 = (((v481 * v786) / v773) + (v791 * ((v351 - v783) + v770))) - ((v481 * v774) / v773);
                    let v2172 = ((((Lanes([(v1901 * v786), 0.0, 0.0])) + (((((v2152 - (Lanes([(v1632 * v784), 0.0, 0.0]))) / v249) * v1396) * (v773 * (v785.powf(v2134)))) * v481)) / v773) + (((v2142 - v2152) + (Lanes([v2129, 0.0, 0.0]))) * v791)) - (Lanes([(((v1901 * v774) + (((((v2129 - (v1632 * v771)) / v249) * v1396) * (v773 * (v772.powf(v2134)))) * v481)) / v773), 0.0, 0.0]));
                    v1202 = v796;
                    v1321 = v2172;
                }
                v1201 = v1202;
                v1317 = v1321;
            }
            let v797 = if v290 > v0 { 1.0 } else { 0.0 };
            let v1207: f64;
            let v1322: Lanes<3>;
            if v797 != 0.0 {
                let v798 = -v277;
                let v2344 = v1670 * v1396;
                let v799 = v798 * v353;
                let v2345 = v2344 * v353;
                let v801 = if v800 <= v0 { 1.0 } else { 0.0 };
                let v1208: f64;
                let v1323: Lanes<3>;
                if v801 != 0.0 {
                    let v803 = v802 - v350;
                    let v2401 = (Lanes([0.0, v1299])) - (Lanes([v1298, 0.0]));
                    let v804 = v803 + v799;
                    let v2402 = Lanes([0.0, v2401[0], v2401[1]]);
                    let v2404 = v2402 + (Lanes([v2345, 0.0, 0.0]));
                    let v805 = if v804 > v0 { 1.0 } else { 0.0 };
                    let v829: f64;
                    let v830: f64;
                    let v1324: Lanes<3>;
                    let v1325: Lanes<3>;
                    if v805 != 0.0 {
                        let v806 = v52 - v353;
                        let v809 = v806.powf((v807 - v292));
                        let v812 = v52 - ((v809 * v806) * v806);
                        let v814 = v52 - v292;
                        let v815 = (v277 * v812) / v814;
                        let v816 = v186 * v292;
                        let v818 = (v816 * v804) / v277;
                        let v819 = v806 + v818;
                        let v821 = (v804 * v819) * v809;
                        let v2430 = ((v2404 * v819) + ((((v2404 * v816) - (Lanes([(v1670 * v818), 0.0, 0.0]))) / v277) * v804)) * v809;
                        let v2431 = Lanes([((v1670 * v812) / v814), 0.0, 0.0]);
                        v829 = v815;
                        v830 = v821;
                        v1324 = v2431;
                        v1325 = v2430;
                    } else {
                        let v822 = v803 / v277;
                        let v823 = v52 - v822;
                        let v824 = v52 - v292;
                        let v826 = v52 - (v823.powf(v824));
                        let v828 = (v277 * v826) / v824;
                        let v2419 = ((Lanes([(v1670 * v826), 0.0, 0.0])) + ((((((v2402 - (Lanes([(v1670 * v822), 0.0, 0.0]))) / v277) * v1396) * (v824 * (v823.powf((v824 - v1291))))) * v1396) * v277)) / v824;
                        v829 = v828;
                        v830 = v0;
                        v1324 = v2419;
                        v1325 = v2343;
                    }
                    let v831 = v829 + v830;
                    let v2432 = v1324 + v1325;
                    v1208 = v831;
                    v1323 = v2432;
                } else {
                    let v2346 = v2345 * v799;
                    let v834 = (v210 * v800) * v800;
                    let v836 = ((v799 * v799) + v834).sqrt();
                    let v839 = v837 * (v799 + v836);
                    let v2352 = (v2345 + ((v2346 + v2346) * (v1291 / (v1584 * v836)))) * v837;
                    let v840 = v839 / v277;
                    let v841 = v52 - v840;
                    let v842 = v52 - v292;
                    let v843 = v841.powf(v842);
                    let v2357 = v842 - v1291;
                    let v846 = v802 - v350;
                    let v2367 = (Lanes([0.0, v1299])) - (Lanes([v1298, 0.0]));
                    let v847 = v846 + v799;
                    let v2368 = Lanes([0.0, v2367[0], v2367[1]]);
                    let v2369 = Lanes([v2345, 0.0, 0.0]);
                    let v2370 = v2368 + v2369;
                    let v2371 = v2370 * v847;
                    let v850 = ((v847 * v847) + v834).sqrt();
                    let v853 = (v186 * (v847 - v850)) - v799;
                    let v2378 = ((v2370 - ((v2371 + v2371) * (v1291 / (v1584 * v850)))) * v186) - v2369;
                    let v854 = v853 / v277;
                    let v855 = v52 - v854;
                    let v856 = v855.powf(v842);
                    let v861 = (v52 - v353).powf((-v292));
                    let v866 = (((v798 * v856) / v842) + (v861 * ((v846 - v853) + v839))) - ((v798 * v843) / v842);
                    let v2398 = ((((Lanes([(v2344 * v856), 0.0, 0.0])) + (((((v2378 - (Lanes([(v1670 * v854), 0.0, 0.0]))) / v277) * v1396) * (v842 * (v855.powf(v2357)))) * v798)) / v842) + (((v2368 - v2378) + (Lanes([v2352, 0.0, 0.0]))) * v861)) - (Lanes([(((v2344 * v843) + (((((v2352 - (v1670 * v840)) / v277) * v1396) * (v842 * (v841.powf(v2357)))) * v798)) / v842), 0.0, 0.0]));
                    v1208 = v866;
                    v1323 = v2398;
                }
                v1207 = v1208;
                v1322 = v1323;
            } else {
                v1207 = v0;
                v1322 = v2343;
            }
            let v867 = v163 * v12;
            let v868 = v343 / v867;
            let v2437 = Lanes([0.0, v1722[0], v1722[1]]);
            let v869 = rspice_limexp(v868);
            let v870 = v869 - v52;
            let v871 = v61 * v870;
            let v2445 = (Lanes([(v1410 * v870), 0.0, 0.0])) + ((((v2437 - (Lanes([(((v1534 * v12) + (v1350 * v163)) * v868), 0.0, 0.0]))) / v867) * v869) * v61);
            let v872 = v164 * v12;
            let v873 = v347 / v872;
            let v2450 = Lanes([0.0, v1728[0], v1728[1]]);
            let v874 = rspice_limexp(v873);
            let v875 = v61 * v74;
            let v876 = v874 - v52;
            let v877 = v875 * v876;
            let v2461 = (Lanes([(((v1410 * v74) + (v1427 * v61)) * v876), 0.0, 0.0])) + ((((v2450 - (Lanes([(((v1535 * v12) + (v1350 * v164)) * v873), 0.0, 0.0]))) / v872) * v874) * v875);
            let v2462 = v1306 * v313;
            let v2463 = v1312 * v309;
            let v884 = (v52 + (v878 * v313)) + (v881 * v309);
            let v2466 = (Lanes([v2462[0], 0.0, v2462[1], v2462[2]])) + (Lanes([v2463[0], v2463[1], v2463[2], 0.0]));
            let v886 = v884 - v885;
            let v2467 = v2466 * v886;
            let v890 = ((v886 * v886) + v888).sqrt();
            let v2473 = (((v2467 + v2467) * (v1291 / (v1584 * v890))) + v2466) * v186;
            let v894 = (v186 * ((v890 + v884) - v885)) + v885;
            let v2477 = (v2445 * v316) + (Lanes([(v1304 * v871), 0.0, 0.0]));
            let v2478 = v2461 * v320;
            let v897 = (v871 * v316) + (v877 * v320);
            let v2481 = (Lanes([v2477[0], 0.0, v2477[1], v2477[2]])) + (Lanes([v2478[0], v2478[1], v2478[2], 0.0]));
            let v899 = if v898 < v186 { 1.0 } else { 0.0 };
            let v914: f64;
            let v1326: Lanes<4>;
            if v899 != 0.0 {
                let v901 = v52 / v900;
                let v904 = (v894.powf(v901)) + (v210 * v897);
                let v907 = v186 * (v894 + (v904.powf(v900)));
                let v2502 = (v2473 + (((v2473 * (v901 * (v894.powf((v901 - v1291))))) + (v2481 * v210)) * (v900 * (v904.powf((v900 - v1291)))))) * v186;
                v914 = v907;
                v1326 = v2502;
            } else {
                let v908 = v186 * v894;
                let v910 = v52 + (v210 * v897);
                let v912 = v52 + (v910.powf(v900));
                let v913 = v908 * v912;
                let v2490 = ((v2473 * v186) * v912) + (((v2481 * v210) * (v900 * (v910.powf((v900 - v1291))))) * v908);
                v914 = v913;
                v1326 = v2490;
            }
            let v915 = v877 / v914;
            let v2506 = ((Lanes([v2461[0], v2461[1], v2461[2], 0.0])) - (v1326 * v915)) / v914;
            let v916 = v871 / v914;
            let v2510 = ((Lanes([v2445[0], 0.0, v2445[1], v2445[2]])) - (v1326 * v916)) / v914;
            let v917 = if v75 > v0 { 1.0 } else { 0.0 };
            let v1139: f64;
            let v1204: f64;
            let v1247: f64;
            let v1327: Lanes<5>;
            let v1328: Lanes<5>;
            let v1329: Lanes<6>;
            if v917 != 0.0 {
                let v918 = v82 * v12;
                let v2513 = v1350 * v82;
                let v919 = v351 / v918;
                let v920 = rspice_limexp(v919);
                let v921 = v347 / v918;
                let v922 = rspice_limexp(v921);
                let v2525 = ((((Lanes([0.0, v1734[0], v1734[1]])) - (Lanes([(v2513 * v919), 0.0, 0.0]))) / v918) * v920) * v923;
                let v925 = v52 - v923;
                let v2526 = (((v2450 - (Lanes([(v2513 * v921), 0.0, 0.0]))) / v918) * v922) * v925;
                let v928 = ((v923 * v920) + (v925 * v922)) - v52;
                let v929 = v85 * v928;
                let v2533 = (Lanes([(v1440 * v928), 0.0, 0.0, 0.0, 0.0])) + (((Lanes([v2525[0], 0.0, v2525[1], 0.0, v2525[2]])) + (Lanes([v2526[0], v2526[1], 0.0, v2526[2], 0.0]))) * v85);
                let v933 = (v52 + (v210 * (v929 * v324))).sqrt();
                let v935 = v186 * (v52 + v933);
                let v2539 = (((v2533 * v324) * v210) * (v1291 / (v1584 * v933))) * v186;
                let v2542 = (Lanes([0.0, v1299])) - (Lanes([v1298, 0.0]));
                let v937 = (v802 - v350) / v918;
                let v938 = rspice_limexp(v937);
                let v939 = v938 - v52;
                let v2552 = (Lanes([(v1440 * v939), 0.0, 0.0])) + (((((Lanes([0.0, v2542[0], v2542[1]])) - (Lanes([(v2513 * v937), 0.0, 0.0]))) / v918) * v938) * v85);
                let v942 = (v929 - (v85 * v939)) / v935;
                let v2556 = v2539 * v942;
                let v2559 = (((Lanes([v2533[0], v2533[1], v2533[2], v2533[3], v2533[4], 0.0])) - (Lanes([v2552[0], 0.0, 0.0, 0.0, v2552[1], v2552[2]]))) - (Lanes([v2556[0], v2556[1], v2556[2], v2556[3], v2556[4], 0.0]))) / v935;
                v1139 = v935;
                v1204 = v929;
                v1247 = v942;
                v1327 = v2539;
                v1328 = v2533;
                v1329 = v2559;
            } else {
                v1139 = v52;
                v1204 = v0;
                v1247 = v0;
                v1327 = v2511;
                v1328 = v2511;
                v1329 = v2512;
            }
            let v944 = if v943 == v52 { 1.0 } else { 0.0 };
            let v1220: f64;
            let v1231: f64;
            let v1330: Lanes<3>;
            let v1331: Lanes<3>;
            if v944 != 0.0 {
                let v945 = v95 * v12;
                let v946 = v343 / v945;
                let v947 = rspice_limexp(v946);
                let v2692 = ((v2437 - (Lanes([((v1350 * v95) * v946), 0.0, 0.0]))) / v945) * v947;
                let v948 = v108 * v12;
                let v949 = v343 / v948;
                let v950 = rspice_limexp(v949);
                let v2698 = ((v2437 - (Lanes([((v1350 * v108) * v949), 0.0, 0.0]))) / v948) * v950;
                let v951 = if v170 > v0 { 1.0 } else { 0.0 };
                let v1221: f64;
                let v1332: Lanes<3>;
                if v951 != 0.0 {
                    let v953 = (v302 - v343) / v303;
                    let v954 = rspice_limexp(v953);
                    let v955 = v947 - v52;
                    let v957 = v950 - v52;
                    let v963 = ((v98 * v955) + (v111 * v957)) - (v960 * (v954 - v305));
                    let v2727 = (((Lanes([(v1457 * v955), 0.0, 0.0])) + (v2692 * v98)) + ((Lanes([(v1474 * v957), 0.0, 0.0])) + (v2698 * v111))) - (((((((Lanes([v1705, 0.0, 0.0])) - v2437) - (Lanes([(v1708 * v953), 0.0, 0.0]))) / v303) * v954) - (Lanes([v1712, 0.0, 0.0]))) * v960);
                    v1221 = v963;
                    v1332 = v2727;
                } else {
                    let v964 = v947 - v52;
                    let v966 = v950 - v52;
                    let v968 = (v98 * v964) + (v111 * v966);
                    let v2707 = ((Lanes([(v1457 * v964), 0.0, 0.0])) + (v2692 * v98)) + ((Lanes([(v1474 * v966), 0.0, 0.0])) + (v2698 * v111));
                    v1221 = v968;
                    v1332 = v2707;
                }
                v1220 = v1221;
                v1231 = v0;
                v1330 = v1332;
                v1331 = v1887;
            } else {
                let v969 = if v943 == v0 { 1.0 } else { 0.0 };
                let v1222: f64;
                let v1232: f64;
                let v1333: Lanes<3>;
                let v1334: Lanes<3>;
                if v969 != 0.0 {
                    let v970 = v95 * v12;
                    let v971 = v345 / v970;
                    let v2647 = Lanes([0.0, v1725[0], v1725[1]]);
                    let v972 = rspice_limexp(v971);
                    let v2651 = ((v2647 - (Lanes([((v1350 * v95) * v971), 0.0, 0.0]))) / v970) * v972;
                    let v973 = v108 * v12;
                    let v974 = v345 / v973;
                    let v975 = rspice_limexp(v974);
                    let v2657 = ((v2647 - (Lanes([((v1350 * v108) * v974), 0.0, 0.0]))) / v973) * v975;
                    let v976 = if v170 > v0 { 1.0 } else { 0.0 };
                    let v1233: f64;
                    let v1335: Lanes<3>;
                    if v976 != 0.0 {
                        let v978 = (v302 - v345) / v303;
                        let v979 = rspice_limexp(v978);
                        let v980 = v972 - v52;
                        let v982 = v975 - v52;
                        let v987 = ((v98 * v980) + (v111 * v982)) - (v960 * (v979 - v305));
                        let v2686 = (((Lanes([(v1457 * v980), 0.0, 0.0])) + (v2651 * v98)) + ((Lanes([(v1474 * v982), 0.0, 0.0])) + (v2657 * v111))) - (((((((Lanes([v1705, 0.0, 0.0])) - v2647) - (Lanes([(v1708 * v978), 0.0, 0.0]))) / v303) * v979) - (Lanes([v1712, 0.0, 0.0]))) * v960);
                        v1233 = v987;
                        v1335 = v2686;
                    } else {
                        let v988 = v972 - v52;
                        let v990 = v975 - v52;
                        let v992 = (v98 * v988) + (v111 * v990);
                        let v2666 = ((Lanes([(v1457 * v988), 0.0, 0.0])) + (v2651 * v98)) + ((Lanes([(v1474 * v990), 0.0, 0.0])) + (v2657 * v111));
                        v1233 = v992;
                        v1335 = v2666;
                    }
                    v1222 = v0;
                    v1232 = v1233;
                    v1333 = v1805;
                    v1334 = v1335;
                } else {
                    let v993 = v95 * v12;
                    let v2560 = v1350 * v95;
                    let v994 = v343 / v993;
                    let v995 = rspice_limexp(v994);
                    let v2565 = ((v2437 - (Lanes([(v2560 * v994), 0.0, 0.0]))) / v993) * v995;
                    let v996 = v108 * v12;
                    let v2566 = v1350 * v108;
                    let v997 = v343 / v996;
                    let v998 = rspice_limexp(v997);
                    let v2571 = ((v2437 - (Lanes([(v2566 * v997), 0.0, 0.0]))) / v996) * v998;
                    let v999 = if v170 > v0 { 1.0 } else { 0.0 };
                    let v1223: f64;
                    let v1336: Lanes<3>;
                    if v999 != 0.0 {
                        let v1001 = (v302 - v343) / v303;
                        let v1002 = rspice_limexp(v1001);
                        let v1003 = v995 - v52;
                        let v1005 = v998 - v52;
                        let v1011 = v943 * (((v98 * v1003) + (v111 * v1005)) - (v960 * (v1002 - v305)));
                        let v2602 = ((((Lanes([(v1457 * v1003), 0.0, 0.0])) + (v2565 * v98)) + ((Lanes([(v1474 * v1005), 0.0, 0.0])) + (v2571 * v111))) - (((((((Lanes([v1705, 0.0, 0.0])) - v2437) - (Lanes([(v1708 * v1001), 0.0, 0.0]))) / v303) * v1002) - (Lanes([v1712, 0.0, 0.0]))) * v960)) * v943;
                        v1223 = v1011;
                        v1336 = v2602;
                    } else {
                        let v1012 = v995 - v52;
                        let v1014 = v998 - v52;
                        let v1017 = v943 * ((v98 * v1012) + (v111 * v1014));
                        let v2581 = (((Lanes([(v1457 * v1012), 0.0, 0.0])) + (v2565 * v98)) + ((Lanes([(v1474 * v1014), 0.0, 0.0])) + (v2571 * v111))) * v943;
                        v1223 = v1017;
                        v1336 = v2581;
                    }
                    let v1018 = v345 / v993;
                    let v2604 = Lanes([0.0, v1725[0], v1725[1]]);
                    let v1019 = rspice_limexp(v1018);
                    let v2608 = ((v2604 - (Lanes([(v2560 * v1018), 0.0, 0.0]))) / v993) * v1019;
                    let v1020 = v345 / v996;
                    let v1021 = rspice_limexp(v1020);
                    let v2613 = ((v2604 - (Lanes([(v2566 * v1020), 0.0, 0.0]))) / v996) * v1021;
                    let v1234: f64;
                    let v1337: Lanes<3>;
                    if v999 != 0.0 {
                        let v1023 = (v302 - v345) / v303;
                        let v1024 = rspice_limexp(v1023);
                        let v1025 = v52 - v943;
                        let v1026 = v1019 - v52;
                        let v1028 = v1021 - v52;
                        let v1034 = v1025 * (((v98 * v1026) + (v111 * v1028)) - (v960 * (v1024 - v305)));
                        let v2644 = ((((Lanes([(v1457 * v1026), 0.0, 0.0])) + (v2608 * v98)) + ((Lanes([(v1474 * v1028), 0.0, 0.0])) + (v2613 * v111))) - (((((((Lanes([v1705, 0.0, 0.0])) - v2604) - (Lanes([(v1708 * v1023), 0.0, 0.0]))) / v303) * v1024) - (Lanes([v1712, 0.0, 0.0]))) * v960)) * v1025;
                        v1234 = v1034;
                        v1337 = v2644;
                    } else {
                        let v1035 = v52 - v943;
                        let v1036 = v1019 - v52;
                        let v1038 = v1021 - v52;
                        let v1041 = v1035 * ((v98 * v1036) + (v111 * v1038));
                        let v2623 = (((Lanes([(v1457 * v1036), 0.0, 0.0])) + (v2608 * v98)) + ((Lanes([(v1474 * v1038), 0.0, 0.0])) + (v2613 * v111))) * v1035;
                        v1234 = v1041;
                        v1337 = v2623;
                    }
                    v1222 = v1223;
                    v1232 = v1234;
                    v1333 = v1336;
                    v1334 = v1337;
                }
                v1220 = v1222;
                v1231 = v1232;
                v1330 = v1333;
                v1331 = v1334;
            }
            let v1042 = v119 * v12;
            let v2728 = v1350 * v119;
            let v1043 = v347 / v1042;
            let v1044 = rspice_limexp(v1043);
            let v1045 = v130 * v12;
            let v2734 = v1350 * v130;
            let v1046 = v347 / v1045;
            let v1047 = rspice_limexp(v1046);
            let v1048 = v1044 - v52;
            let v1050 = v1047 - v52;
            let v1052 = (v122 * v1048) + (v133 * v1050);
            let v2748 = ((Lanes([(v1487 * v1048), 0.0, 0.0])) + ((((v2450 - (Lanes([(v2728 * v1043), 0.0, 0.0]))) / v1042) * v1044) * v122)) + ((Lanes([(v1500 * v1050), 0.0, 0.0])) + ((((v2450 - (Lanes([(v2734 * v1046), 0.0, 0.0]))) / v1045) * v1047) * v133));
            let v1055 = if (if v134 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v136 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1237: f64;
            let v1338: Lanes<3>;
            if v1055 != 0.0 {
                let v1056 = v351 / v1042;
                let v2750 = Lanes([0.0, v1734[0], v1734[1]]);
                let v1057 = rspice_limexp(v1056);
                let v1058 = v351 / v1045;
                let v1059 = rspice_limexp(v1058);
                let v1060 = v1057 - v52;
                let v1062 = v1059 - v52;
                let v1064 = (v135 * v1060) + (v137 * v1062);
                let v2768 = ((Lanes([(v1503 * v1060), 0.0, 0.0])) + ((((v2750 - (Lanes([(v2728 * v1056), 0.0, 0.0]))) / v1042) * v1057) * v135)) + ((Lanes([(v1506 * v1062), 0.0, 0.0])) + ((((v2750 - (Lanes([(v2734 * v1058), 0.0, 0.0]))) / v1045) * v1059) * v137));
                v1237 = v1064;
                v1338 = v2768;
            } else {
                v1237 = v0;
                v1338 = v2329;
            }
            let v1066 = if v1065 > v0 { 1.0 } else { 0.0 };
            let v1084: f64;
            let v1339: Lanes<4>;
            if v1066 != 0.0 {
                let v1067 = v249 - v347;
                let v2771 = (Lanes([v1632, 0.0, 0.0])) - v2450;
                let v2772 = v2771 * v1067;
                let v1071 = ((v1067 * v1067) + v1069).sqrt();
                let v1073 = v186 * (v1071 + v1067);
                let v2778 = (((v2772 + v2772) * (v1291 / (v1584 * v1071))) + v2771) * v186;
                let v1074 = v1065 * v1073;
                let v1075 = -v169;
                let v1076 = v285 - v52;
                let v1077 = v1073.powf(v1076);
                let v1079 = rspice_limexp((v1075 * v1077));
                let v1080 = v1074 * v1079;
                let v1082 = (v916 - v915) - v1052;
                let v1083 = v1082 * v1080;
                let v2797 = (((v2778 * v1065) * v1079) + ((((Lanes([((v1537 * v1396) * v1077), 0.0, 0.0])) + ((v2778 * (v1076 * (v1073.powf((v1076 - v1291))))) * v1075)) * v1079) * v1074)) * v1082;
                let v2799 = (((v2510 - v2506) - (Lanes([v2748[0], v2748[1], v2748[2], 0.0]))) * v1080) + (Lanes([v2797[0], v2797[1], v2797[2], 0.0]));
                v1084 = v1083;
                v1339 = v2799;
            } else {
                v1084 = v0;
                v1339 = v2769;
            }
            let v1085 = v1052 - v1084;
            let v2801 = (Lanes([v2748[0], v2748[1], v2748[2], 0.0])) - v1339;
            let v1086 = if v19 > v0 { 1.0 } else { 0.0 };
            let v1251: f64;
            let v1340: Lanes<3>;
            if v1086 != 0.0 {
                let v2805 = (Lanes([v1300, 0.0])) - (Lanes([0.0, v1297]));
                let v1089 = (v1087 - v348) / v22;
                let v2810 = ((Lanes([v2805[0], 0.0, v2805[1]])) - (Lanes([0.0, (v1361 * v1089), 0.0]))) / v22;
                v1251 = v1089;
                v1340 = v2810;
            } else {
                v1251 = v0;
                v1340 = v2802;
            }
            let v1090 = v347 / v12;
            let v1091 = rspice_limexp(v1090);
            let v1092 = v349 / v12;
            let v1093 = rspice_limexp(v1092);
            let v1096 = (v52 + (v297 * v1091)).sqrt();
            let v2828 = ((Lanes([(v1699 * v1091), 0.0, 0.0])) + ((((v2450 - (Lanes([(v1350 * v1090), 0.0, 0.0]))) / v12) * v1091) * v297)) * (v1291 / (v1584 * v1096));
            let v1099 = (v52 + (v297 * v1093)).sqrt();
            let v2835 = ((Lanes([(v1699 * v1093), 0.0, 0.0])) + (((((Lanes([0.0, v1731[0], v1731[1]])) - (Lanes([(v1350 * v1092), 0.0, 0.0]))) / v12) * v1093) * v297)) * (v1291 / (v1584 * v1099));
            let v1100 = if v23 > v0 { 1.0 } else { 0.0 };
            let v1255: f64;
            let v1341: Lanes<4>;
            if v1100 != 0.0 {
                let v1102 = v1099 + v52;
                let v1103 = (v1096 + v52) / v1102;
                let v2837 = v2835 * v1103;
                let v2838 = Lanes([v2828[0], 0.0, v2828[1], v2828[2]]);
                let v1104 = v348 - v346;
                let v2844 = (Lanes([v1297, 0.0])) - (Lanes([0.0, v1296]));
                let v1107 = (v1096 - v1099) - (v1103.ln());
                let v1110 = (v1104 + (v12 * v1107)) / v26;
                let v2859 = (((Lanes([0.0, v2844[0], v2844[1], 0.0])) + ((Lanes([(v1350 * v1107), 0.0, 0.0, 0.0])) + (((v2838 - (Lanes([v2835[0], v2835[1], 0.0, v2835[2]]))) - (((v2838 - (Lanes([v2837[0], v2837[1], 0.0, v2837[2]]))) / v1102) * (v1291 / v1103))) * v12))) - (Lanes([(v1366 * v1110), 0.0, 0.0, 0.0]))) / v26;
                let v1111 = v327 * v26;
                let v1114 = (v186 * v327) * v331;
                let v2869 = v2844 * v1104;
                let v1117 = ((v1104 * v1104) + v1069).sqrt();
                let v2875 = ((v2869 + v2869) * (v1291 / (v1584 * v1117))) * v1114;
                let v1119 = v52 + (v1114 * v1117);
                let v1120 = (v1111 * v1110) / v1119;
                let v2879 = ((Lanes([(((v1305 * v186) * v331) * v1117), 0.0, 0.0])) + (Lanes([0.0, v2875[0], v2875[1]]))) * v1120;
                let v2883 = ((((Lanes([(((v1305 * v26) + (v1366 * v327)) * v1110), 0.0, 0.0, 0.0])) + (v2859 * v1111)) - (Lanes([v2879[0], v2879[1], v2879[2], 0.0]))) / v1119) * v1120;
                let v1123 = (v52 + (v1120 * v1120)).sqrt();
                let v1124 = v1110 / v1123;
                let v2890 = (v2859 - (((v2883 + v2883) * (v1291 / (v1584 * v1123))) * v1124)) / v1123;
                v1255 = v1124;
                v1341 = v2890;
            } else {
                v1255 = v0;
                v1341 = v2836;
            }
            let v1125 = if v27 > v0 { 1.0 } else { 0.0 };
            let v1259: f64;
            let v1342: Lanes<3>;
            if v1125 != 0.0 {
                let v2894 = (Lanes([v1301, 0.0])) - (Lanes([0.0, v1295]));
                let v1128 = (v1126 - v344) / v30;
                let v2899 = ((Lanes([v2894[0], 0.0, v2894[1]])) - (Lanes([0.0, (v1371 * v1128), 0.0]))) / v30;
                v1259 = v1128;
                v1342 = v2899;
            } else {
                v1259 = v0;
                v1342 = v2891;
            }
            let v1129 = if v31 > v0 { 1.0 } else { 0.0 };
            let v1263: f64;
            let v1343: Lanes<5>;
            if v1129 != 0.0 {
                let v1130 = v344 - v341;
                let v2904 = ((Lanes([v1295, 0.0])) - (Lanes([0.0, v1293]))) * v914;
                let v2905 = v1326 * v1130;
                let v1132 = (v1130 * v914) / v34;
                let v2912 = (((Lanes([0.0, 0.0, v2904[0], v2904[1], 0.0])) + (Lanes([v2905[0], v2905[1], 0.0, v2905[2], v2905[3]]))) - (Lanes([(v1376 * v1132), 0.0, 0.0, 0.0, 0.0]))) / v34;
                v1263 = v1132;
                v1343 = v2912;
            } else {
                v1263 = v0;
                v1343 = v2900;
            }
            let v1133 = if v35 > v0 { 1.0 } else { 0.0 };
            let v1267: f64;
            let v1344: Lanes<3>;
            if v1133 != 0.0 {
                let v2916 = (Lanes([v1302, 0.0])) - (Lanes([0.0, v1294]));
                let v1136 = (v1134 - v342) / v38;
                let v2921 = ((Lanes([v2916[0], 0.0, v2916[1]])) - (Lanes([0.0, (v1381 * v1136), 0.0]))) / v38;
                v1267 = v1136;
                v1344 = v2921;
            } else {
                v1267 = v0;
                v1344 = v2913;
            }
            let v1137 = if v43 > v0 { 1.0 } else { 0.0 };
            let v1271: f64;
            let v1345: Lanes<6>;
            if v1137 != 0.0 {
                let v1138 = v350 - v348;
                let v2926 = ((Lanes([0.0, v1298])) - (Lanes([v1297, 0.0]))) * v1139;
                let v2927 = v1327 * v1138;
                let v1141 = (v1138 * v1139) / v46;
                let v2934 = (((Lanes([0.0, v2926[0], 0.0, 0.0, 0.0, v2926[1]])) + (Lanes([v2927[0], 0.0, v2927[1], v2927[2], v2927[3], v2927[4]]))) - (Lanes([(v1391 * v1141), 0.0, 0.0, 0.0, 0.0, 0.0]))) / v46;
                v1271 = v1141;
                v1345 = v2934;
            } else {
                v1271 = v0;
                v1345 = v2922;
            }
            let v1144 = if (if v138 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v149 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1244: f64;
            let v1346: Lanes<3>;
            if v1144 != 0.0 {
                let v1145 = v802 - v350;
                let v2937 = (Lanes([0.0, v1299])) - (Lanes([v1298, 0.0]));
                let v1146 = v145 * v12;
                let v1147 = v1145 / v1146;
                let v2940 = Lanes([0.0, v2937[0], v2937[1]]);
                let v1148 = rspice_limexp(v1147);
                let v1149 = v156 * v12;
                let v1150 = v1145 / v1149;
                let v1151 = rspice_limexp(v1150);
                let v1152 = v1148 - v52;
                let v1154 = v1151 - v52;
                let v1156 = (v148 * v1152) + (v159 * v1154);
                let v2959 = ((Lanes([(v1519 * v1152), 0.0, 0.0])) + ((((v2940 - (Lanes([((v1350 * v145) * v1147), 0.0, 0.0]))) / v1146) * v1148) * v148)) + ((Lanes([(v1532 * v1154), 0.0, 0.0])) + ((((v2940 - (Lanes([((v1350 * v156) * v1150), 0.0, 0.0]))) / v1149) * v1151) * v159));
                v1244 = v1156;
                v1346 = v2959;
            } else {
                v1244 = v0;
                v1346 = v2343;
            }
            let v1157 = if v39 > v0 { 1.0 } else { 0.0 };
            let v1240: f64;
            let v1347: Lanes<3>;
            if v1157 != 0.0 {
                let v2963 = (Lanes([v1303, 0.0])) - (Lanes([0.0, v1299]));
                let v1160 = (v1158 - v802) / v42;
                let v2968 = ((Lanes([v2963[0], 0.0, v2963[1]])) - (Lanes([0.0, (v1386 * v1160), 0.0]))) / v42;
                v1240 = v1160;
                v1347 = v2968;
            } else {
                v1240 = v0;
                v1347 = v2960;
            }
            let v1161 = if v871 > v0 { 1.0 } else { 0.0 };
            let v1162: f64;
            if v1161 != 0.0 {
                v1162 = v52;
            } else {
                v1162 = v0;
            }
            let v1164 = (v871 * v1162) * v339;
            let v2970 = (v2445 * v1162) * v339;
            let v1165 = v1164 + v52;
            let v1166 = v1164 / v1165;
            let v1171 = v1167 * (v52 + (v1168 * v894));
            let v1176 = rspice_limexp(((v347 * v335) / v1174));
            let v1177 = v1172 * v1176;
            let v2980 = ((v2970 - (v2970 * v1166)) / v1165) * v1166;
            let v1179 = v340 + (v1166 * v1166);
            let v2982 = ((((v1728 * v335) / v1174) * v1176) * v1172) * v1179;
            let v2983 = (v2980 + v2980) * v1177;
            let v1182 = v52 + ((v1177 * v1179) * v1162);
            let v1183 = v1171 * v1182;
            let v2995 = ((Lanes([(v1678 * v878), 0.0, 0.0])) + (v1306 * v282)) * v943;
            let v2997 = v2445 * v1183;
            let v1187 = (v1183 * v871) / v914;
            let v1188 = ((v282 * v878) * v943) + v1187;
            let v3004 = (Lanes([v2995[0], 0.0, v2995[1], v2995[2]])) + ((((((((v2473 * v1168) * v1167) * v1182) + ((((Lanes([0.0, v2982[0], v2982[1], 0.0])) + (Lanes([v2983[0], 0.0, v2983[1], v2983[2]]))) * v1162) * v1171)) * v871) + (Lanes([v2997[0], 0.0, v2997[1], v2997[2]]))) - (v1326 * v1187)) / v914);
            let v1191 = v52 - v943;
            let v1192 = (v282 * v1189) * v1191;
            let v3009 = ((Lanes([(v1678 * v1189), 0.0, 0.0])) + (v1309 * v282)) * v1191;
            let v1199 = ((v287 * v881) + (v1194 * v877)) + (v1197 * v1096);
            let v3017 = (((Lanes([(v1686 * v881), 0.0, 0.0])) + (v1312 * v287)) + (v2461 * v1194)) + (v2828 * v1197);
            let v1200 = v1197 * v1099;
            let v3018 = v2835 * v1197;
            let v3022 = (Lanes([(v1687 * v1201), 0.0, 0.0])) + (v1317 * v289);
            let v1206 = (v289 * v1201) + (v1194 * v1204);
            let v3025 = (Lanes([v3022[0], 0.0, v3022[1], 0.0, v3022[2]])) + (v1328 * v1194);
            let v1211 = v802 - v350;
            let v3032 = (Lanes([0.0, v1299])) - (Lanes([v1298, 0.0]));
            let v3033 = v3032 * v1210;
            let v1213 = (v294 * v1207) + (v1210 * v1211);
            let v3035 = ((Lanes([(v1695 * v1207), 0.0, 0.0])) + (v1322 * v294)) + (Lanes([0.0, v3033[0], v3033[1]]));
            let v1216 = (v1126 - v1134) * v1215;
            let v3039 = ((Lanes([v1301, 0.0])) - (Lanes([0.0, v1302]))) * v1215;
            let v1219 = (v1126 - v1087) * v1218;
            let v3043 = ((Lanes([0.0, v1301])) - (Lanes([v1300, 0.0]))) * v1218;
            let v3045 = v1722 * v1220;
            let v3047 = (v1330 * v343) + (Lanes([0.0, v3045[0], v3045[1]]));
            let v3049 = v1728 * v1085;
            let v1227 = v916 - v915;
            let v1228 = v346 - v342;
            let v3059 = ((Lanes([v1296, 0.0])) - (Lanes([0.0, v1294]))) * v1227;
            let v3062 = ((Lanes([v3047[0], 0.0, v3047[1], v3047[2]])) + ((v2801 * v347) + (Lanes([0.0, v3049[0], v3049[1], 0.0])))) + (((v2510 - v2506) * v1228) + (Lanes([0.0, v3059[0], 0.0, v3059[1]])));
            let v3064 = v1725 * v1231;
            let v3066 = (v1331 * v345) + (Lanes([0.0, v3064[0], v3064[1]]));
            let v3069 = (Lanes([v3062[0], v3062[1], 0.0, v3062[2], v3062[3]])) + (Lanes([v3066[0], 0.0, v3066[1], 0.0, v3066[2]]));
            let v3071 = v1734 * v1237;
            let v3073 = (v1338 * v351) + (Lanes([0.0, v3071[0], v3071[1]]));
            let v3076 = (Lanes([v3069[0], v3069[1], v3069[2], v3069[3], v3069[4], 0.0])) + (Lanes([v3073[0], 0.0, v3073[1], 0.0, 0.0, v3073[2]]));
            let v1241 = v1158 - v802;
            let v3081 = ((Lanes([v1303, 0.0])) - (Lanes([0.0, v1299]))) * v1240;
            let v3083 = (v1347 * v1241) + (Lanes([v3081[0], 0.0, v3081[1]]));
            let v3088 = v3032 * v1244;
            let v3090 = (v1346 * v1211) + (Lanes([0.0, v3088[0], v3088[1]]));
            let v1248 = v344 - v802;
            let v3097 = ((Lanes([v1295, 0.0])) - (Lanes([0.0, v1299]))) * v1247;
            let v3099 = (v1329 * v1248) + (Lanes([0.0, 0.0, v3097[0], 0.0, 0.0, v3097[1]]));
            let v3101 = (((Lanes([0.0, v3076[0], v3076[1], v3076[2], v3076[3], v3076[4], v3076[5], 0.0])) + (Lanes([v3083[0], v3083[1], 0.0, 0.0, 0.0, 0.0, 0.0, v3083[2]]))) + (Lanes([0.0, v3090[0], 0.0, 0.0, 0.0, 0.0, v3090[1], v3090[2]]))) + (Lanes([0.0, v3099[0], v3099[1], v3099[2], v3099[3], 0.0, v3099[4], v3099[5]]));
            let v1252 = v1087 - v348;
            let v3106 = ((Lanes([v1300, 0.0])) - (Lanes([0.0, v1297]))) * v1251;
            let v3108 = (v1340 * v1252) + (Lanes([v3106[0], 0.0, v3106[1]]));
            let v1256 = v348 - v346;
            let v3116 = ((Lanes([v1297, 0.0])) - (Lanes([0.0, v1296]))) * v1255;
            let v3118 = (v1341 * v1256) + (Lanes([0.0, v3116[0], v3116[1], 0.0]));
            let v3120 = ((Lanes([0.0, v3101[0], v3101[1], 0.0, v3101[2], v3101[3], v3101[4], v3101[5], v3101[6], v3101[7]])) + (Lanes([v3108[0], 0.0, v3108[1], v3108[2], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, v3118[0], v3118[1], v3118[2], 0.0, v3118[3], 0.0, 0.0, 0.0]));
            let v1260 = v1126 - v344;
            let v3125 = ((Lanes([v1301, 0.0])) - (Lanes([0.0, v1295]))) * v1259;
            let v3127 = (v1342 * v1260) + (Lanes([v3125[0], 0.0, v3125[1]]));
            let v1264 = v344 - v341;
            let v3135 = ((Lanes([v1295, 0.0])) - (Lanes([0.0, v1293]))) * v1263;
            let v3137 = (v1343 * v1264) + (Lanes([0.0, 0.0, v3135[0], v3135[1], 0.0]));
            let v3139 = ((Lanes([v3120[0], 0.0, v3120[1], v3120[2], v3120[3], v3120[4], v3120[5], v3120[6], v3120[7], v3120[8], v3120[9]])) + (Lanes([0.0, v3127[0], 0.0, v3127[1], 0.0, 0.0, v3127[2], 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v3137[0], 0.0, v3137[1], v3137[2], v3137[3], v3137[4], 0.0, 0.0]));
            let v1268 = v1134 - v342;
            let v3144 = ((Lanes([v1302, 0.0])) - (Lanes([0.0, v1294]))) * v1267;
            let v3146 = (v1344 * v1268) + (Lanes([v3144[0], 0.0, v3144[1]]));
            let v1272 = v350 - v348;
            let v3154 = ((Lanes([0.0, v1298])) - (Lanes([v1297, 0.0]))) * v1271;
            let v3156 = (v1345 * v1272) + (Lanes([0.0, v3154[0], 0.0, 0.0, 0.0, v3154[1]]));
            let v1275 = -((((((((((((((v1220 * v343) + (v1085 * v347)) + (v1227 * v1228)) + (v1231 * v345)) + (v1237 * v351)) + (v1240 * v1241)) + (v1244 * v1211)) + (v1247 * v1248)) + (v1251 * v1252)) + (v1255 * v1256)) + (v1259 * v1260)) + (v1263 * v1264)) + (v1267 * v1268)) + (v1271 * v1272));
            let v3159 = (((Lanes([v3139[0], v3139[1], 0.0, v3139[2], v3139[3], v3139[4], v3139[5], v3139[6], v3139[7], v3139[8], v3139[9], v3139[10]])) + (Lanes([0.0, 0.0, v3146[0], 0.0, v3146[1], 0.0, 0.0, 0.0, 0.0, v3146[2], 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, 0.0, v3156[0], v3156[1], v3156[2], v3156[3], v3156[4], 0.0, v3156[5], 0.0]))) * v1396;
            let v1277 = if v1276 > v0 { 1.0 } else { 0.0 };
            let v1289: f64;
            let v1348: f64;
            if v1277 != 0.0 {
                let v1278 = v7 / v1276;
                let v3160 = v1292 / v1276;
                v1289 = v1278;
                v1348 = v3160;
            } else {
                v1289 = v0;
                v1348 = v1713;
            }
            let v1280 = v7 * v1279;
            let v3161 = v1292 * v1279;
            let v1281 = ddt(6568, v1188);
            let v3163 = v3004 * v3162;
            let v1282 = ddt(6570, v1192);
            let v3164 = v3009 * v3162;
            let v1283 = ddt(6572, v1199);
            let v3165 = v3017 * v3162;
            let v1284 = ddt(6574, v1200);
            let v3166 = v3018 * v3162;
            let v1285 = ddt(6576, v1206);
            let v3167 = v3025 * v3162;
            let v1286 = ddt(6578, v1216);
            let v3168 = v3039 * v3162;
            let v1287 = ddt(6580, v1219);
            let v3169 = v3043 * v3162;
            let v1288 = ddt(6585, v1213);
            let v3170 = v3035 * v3162;
            let v1290 = ddt(6589, v1280);
            let v3172 = v1330[0];
            let v3173 = v1330[1];
            let v3174 = v1330[2];
            let v3175 = v1331[0];
            let v3176 = v1331[1];
            let v3177 = v1331[2];
            let v3178 = v2510[0];
            let v3179 = v2510[1];
            let v3180 = v2510[2];
            let v3181 = v2510[3];
            let v3182 = v2506[0];
            let v3183 = v2506[1];
            let v3184 = v2506[2];
            let v3185 = v2506[3];
            let v3186 = v2801[0];
            let v3187 = v2801[1];
            let v3188 = v2801[2];
            let v3189 = v2801[3];
            let v3190 = v1338[0];
            let v3191 = v1338[1];
            let v3192 = v1338[2];
            let v3193 = v1340[0];
            let v3194 = v1340[1];
            let v3195 = v1340[2];
            let v3196 = v1341[0];
            let v3197 = v1341[1];
            let v3198 = v1341[2];
            let v3199 = v1341[3];
            let v3200 = v1342[0];
            let v3201 = v1342[1];
            let v3202 = v1342[2];
            let v3203 = v1343[0];
            let v3204 = v1343[1];
            let v3205 = v1343[2];
            let v3206 = v1343[3];
            let v3207 = v1343[4];
            let v3208 = v1344[0];
            let v3209 = v1344[1];
            let v3210 = v1344[2];
            let v3211 = v1345[0];
            let v3212 = v1345[1];
            let v3213 = v1345[2];
            let v3214 = v1345[3];
            let v3215 = v1345[4];
            let v3216 = v1345[5];
            let v3217 = v3163[0];
            let v3218 = v3163[1];
            let v3219 = v3163[2];
            let v3220 = v3163[3];
            let v3221 = v3164[0];
            let v3222 = v3164[1];
            let v3223 = v3164[2];
            let v3224 = v3165[0];
            let v3225 = v3165[1];
            let v3226 = v3165[2];
            let v3227 = v3166[0];
            let v3228 = v3166[1];
            let v3229 = v3166[2];
            let v3230 = v3167[0];
            let v3231 = v3167[1];
            let v3232 = v3167[2];
            let v3233 = v3167[3];
            let v3234 = v3167[4];
            let v3235 = v3168[0];
            let v3236 = v3168[1];
            let v3237 = v3169[0];
            let v3238 = v3169[1];
            let v3239 = v1346[0];
            let v3240 = v1346[1];
            let v3241 = v1346[2];
            let v3242 = v1329[0];
            let v3243 = v1329[1];
            let v3244 = v1329[2];
            let v3245 = v1329[3];
            let v3246 = v1329[4];
            let v3247 = v1329[5];
            let v3248 = v1347[0];
            let v3249 = v1347[1];
            let v3250 = v1347[2];
            let v3251 = v3170[0];
            let v3252 = v3170[1];
            let v3253 = v3170[2];
            let v3254 = v1348;
            let v3255 = v3159[0];
            let v3256 = v3159[1];
            let v3257 = v3159[2];
            let v3258 = v3159[3];
            let v3259 = v3159[4];
            let v3260 = v3159[5];
            let v3261 = v3159[6];
            let v3262 = v3159[7];
            let v3263 = v3159[8];
            let v3264 = v3159[9];
            let v3265 = v3159[10];
            let v3266 = v3159[11];
            let v3267 = (v3161 * v3162);
            let v3268 = v3004[0];
            let v3269 = v3004[1];
            let v3270 = v3004[2];
            let v3271 = v3004[3];
            let v3272 = v3009[0];
            let v3273 = v3009[1];
            let v3274 = v3009[2];
            let v3275 = v3017[0];
            let v3276 = v3017[1];
            let v3277 = v3017[2];
            let v3278 = v3018[0];
            let v3279 = v3018[1];
            let v3280 = v3018[2];
            let v3281 = v3025[0];
            let v3282 = v3025[1];
            let v3283 = v3025[2];
            let v3284 = v3025[3];
            let v3285 = v3025[4];
            let v3286 = v3039[0];
            let v3287 = v3039[1];
            let v3288 = v3043[0];
            let v3289 = v3043[1];
            let v3290 = v3035[0];
            let v3291 = v3035[1];
            let v3292 = v3035[2];
            let v3293 = v3161;
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1220),
            [4, 8, 9],
            [v3172, v3173, v3174],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1231),
            [4, 7, 9],
            [v3175, v3176, v3177],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(9),
            multiplicity * (v916),
            [4, 6, 8, 9],
            [v3178, v3179, v3180, v3181],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(9),
            Some(6),
            multiplicity * (v915),
            [4, 6, 8, 9],
            [v3182, v3183, v3184, v3185],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1085),
            [4, 6, 8, 9],
            [v3186, v3187, v3188, v3189],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1237),
            [4, 7, 10],
            [v3190, v3191, v3192],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(5),
            multiplicity * (v1251),
            [0, 4, 5],
            [v3193, v3194, v3195],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v1255),
            [4, 5, 6, 8],
            [v3196, v3197, v3198, v3199],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(7),
            multiplicity * (v1259),
            [1, 4, 7],
            [v3200, v3201, v3202],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1263),
            [4, 6, 7, 8, 9],
            [v3203, v3204, v3205, v3206, v3207],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(9),
            multiplicity * (v1267),
            [2, 4, 9],
            [v3208, v3209, v3210],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1271),
            [4, 5, 6, 7, 8, 10],
            [v3211, v3212, v3213, v3214, v3215, v3216],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1281),
            [4, 6, 8, 9],
            [v3217, v3218, v3219, v3220],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1282),
            [4, 7, 9],
            [v3221, v3222, v3223],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1283),
            [4, 6, 8],
            [v3224, v3225, v3226],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(5),
            multiplicity * (v1284),
            [4, 5, 8],
            [v3227, v3228, v3229],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1285),
            [4, 6, 7, 8, 10],
            [v3230, v3231, v3232, v3233, v3234],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v1286),
            [1, 2],
            [v3235, v3236],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (v1287),
            [0, 1],
            [v3237, v3238],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1244),
            [4, 10, 11],
            [v3239, v3240, v3241],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (v1247),
            [4, 6, 7, 8, 10, 11],
            [v3242, v3243, v3244, v3245, v3246, v3247],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(11),
            multiplicity * (v1240),
            [3, 4, 11],
            [v3248, v3249, v3250],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1288),
            [4, 10, 11],
            [v3251, v3252, v3253],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v1289),
            [4],
            [v3254],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<12, 0>(
            Some(4),
            None,
            multiplicity * (v1275),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [v3255, v3256, v3257, v3258, v3259, v3260, v3261, v3262, v3263, v3264, v3265, v3266],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v1290),
            [4],
            [v3267],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v1220;
        self.canonical_reactive[1] = v1231;
        self.canonical_reactive[2] = v916;
        self.canonical_reactive[3] = v915;
        self.canonical_reactive[4] = v1085;
        self.canonical_reactive[5] = v1237;
        self.canonical_reactive[6] = v1251;
        self.canonical_reactive[7] = v1255;
        self.canonical_reactive[8] = v1259;
        self.canonical_reactive[9] = v1263;
        self.canonical_reactive[10] = v1267;
        self.canonical_reactive[11] = v1271;
        self.canonical_reactive[12] = v1188;
        self.canonical_reactive[13] = v3268;
        self.canonical_reactive[14] = v3269;
        self.canonical_reactive[15] = v3270;
        self.canonical_reactive[16] = v3271;
        self.canonical_reactive[17] = v1192;
        self.canonical_reactive[18] = v3272;
        self.canonical_reactive[19] = v3273;
        self.canonical_reactive[20] = v3274;
        self.canonical_reactive[21] = v1199;
        self.canonical_reactive[22] = v3275;
        self.canonical_reactive[23] = v3276;
        self.canonical_reactive[24] = v3277;
        self.canonical_reactive[25] = v1200;
        self.canonical_reactive[26] = v3278;
        self.canonical_reactive[27] = v3279;
        self.canonical_reactive[28] = v3280;
        self.canonical_reactive[29] = v1206;
        self.canonical_reactive[30] = v3281;
        self.canonical_reactive[31] = v3282;
        self.canonical_reactive[32] = v3283;
        self.canonical_reactive[33] = v3284;
        self.canonical_reactive[34] = v3285;
        self.canonical_reactive[35] = v1216;
        self.canonical_reactive[36] = v3286;
        self.canonical_reactive[37] = v3287;
        self.canonical_reactive[38] = v1219;
        self.canonical_reactive[39] = v3288;
        self.canonical_reactive[40] = v3289;
        self.canonical_reactive[41] = v1244;
        self.canonical_reactive[42] = v1247;
        self.canonical_reactive[43] = v1240;
        self.canonical_reactive[44] = v1213;
        self.canonical_reactive[45] = v3290;
        self.canonical_reactive[46] = v3291;
        self.canonical_reactive[47] = v3292;
        self.canonical_reactive[48] = v1289;
        self.canonical_reactive[49] = v1275;
        self.canonical_reactive[50] = v1280;
        self.canonical_reactive[51] = v3293;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(9),
            &[4, 6, 8, 9],
            &[cached[13], cached[14], cached[15], cached[16]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(9),
            &[4, 7, 9],
            &[cached[18], cached[19], cached[20]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 6, 8],
            &[cached[22], cached[23], cached[24]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[4, 5, 8],
            &[cached[26], cached[27], cached[28]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(10),
            &[4, 6, 7, 8, 10],
            &[cached[30], cached[31], cached[32], cached[33], cached[34]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[36], cached[37]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[39], cached[40]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(10),
            &[4, 10, 11],
            &[cached[45], cached[46], cached[47]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[51]],
            &[],
            &[],
            multiplicity,
        );
    }

}

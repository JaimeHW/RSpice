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
        let parameter_given = &*self.param_given;
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
            let slot = match operator { 9699 => 0usize, 9701 => 1usize, 9703 => 2usize, 9705 => 3usize, 9707 => 4usize, 9709 => 5usize, 9711 => 6usize, 9713 => 7usize, 9715 => 8usize, 9717 => 9usize, _ => usize::MAX };
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
            let v1 = if parameter_given[6] { 1.0 } else { 0.0 };
            let v2 = 1e0f64;
            let v3 = if parameter_given[7] { 1.0 } else { 0.0 };
            let v4 = 1e-2f64;
            let v5 = if parameter_given[10] { 1.0 } else { 0.0 };
            let v6 = parameters[10];
            let v7 = 1e-12f64;
            let v9 = if parameter_given[11] { 1.0 } else { 0.0 };
            let v10 = parameters[11];
            let v12 = if parameter_given[3] { 1.0 } else { 0.0 };
            let v13 = if parameter_given[4] { 1.0 } else { 0.0 };
            let v14 = -1e0f64;
            let v15 = if parameter_given[5] { 1.0 } else { 0.0 };
            let v16 = parameters[5];
            let v17 = parameters[12];
            let v19 = parameters[74];
            let v23 = parameters[75];
            let v27 = parameters[20];
            let v31 = parameters[79];
            let v35 = parameters[80];
            let v40 = 2.7315e2f64;
            let v41 = parameters[13];
            let v43 = temperature;
            let v44 = parameters[0];
            let v47 = parameters[8];
            let v49 = parameters[9];
            let v51 = parameters[14];
            let v58 = parameters[15];
            let v68 = 1.380662e-23f64;
            let v70 = 1.602189e-19f64;
            let v73 = parameters[26];
            let v74 = parameters[90];
            let v76 = parameters[89];
            let v78 = parameters[88];
            let v87 = parameters[122];
            let v88 = parameters[28];
            let v92 = parameters[113];
            let v101 = parameters[72];
            let v105 = 5e-1f64;
            let v107 = 4e0f64;
            let v109 = parameters[73];
            let v123 = parameters[27];
            let v124 = parameters[125];
            let v125 = parameters[29];
            let v129 = parameters[121];
            let v157 = parameters[31];
            let v158 = parameters[33];
            let v162 = parameters[120];
            let v182 = parameters[54];
            let v183 = parameters[123];
            let v184 = parameters[56];
            let v188 = parameters[114];
            let v200 = parameters[58];
            let v201 = parameters[124];
            let v202 = parameters[59];
            let v206 = parameters[117];
            let v218 = parameters[60];
            let v219 = parameters[61];
            let v223 = parameters[115];
            let v235 = parameters[62];
            let v236 = parameters[63];
            let v240 = parameters[118];
            let v252 = parameters[64];
            let v260 = parameters[65];
            let v268 = parameters[66];
            let v269 = parameters[67];
            let v273 = parameters[116];
            let v281 = parameters[68];
            let v282 = parameters[69];
            let v286 = parameters[119];
            let v294 = node_potentials[3];
            let v315 = parameters[126];
            let v318 = if parameter_given[109] { 1.0 } else { 0.0 };
            let v319 = parameters[16];
            let v320 = parameters[109];
            let v323 = parameters[107];
            let v326 = if parameter_given[108] { 1.0 } else { 0.0 };
            let v327 = parameters[17];
            let v328 = parameters[108];
            let v333 = if parameter_given[106] { 1.0 } else { 0.0 };
            let v334 = parameters[21];
            let v335 = parameters[106];
            let v338 = parameters[104];
            let v341 = if parameter_given[105] { 1.0 } else { 0.0 };
            let v342 = parameters[22];
            let v343 = parameters[105];
            let v348 = parameters[23];
            let v349 = parameters[103];
            let v352 = parameters[24];
            let v353 = parameters[111];
            let v356 = if parameter_given[110] { 1.0 } else { 0.0 };
            let v357 = parameters[25];
            let v358 = parameters[110];
            let v363 = parameters[101];
            let v364 = parameters[132];
            let v422 = parameters[129];
            let v427 = parameters[84];
            let v428 = parameters[127];
            let v432 = parameters[86];
            let v433 = parameters[128];
            let v437 = parameters[91];
            let v438 = parameters[92];
            let v444 = parameters[93];
            let v448 = 2e0f64;
            let v451 = parameters[37];
            let v456 = -5e-1f64;
            let v465 = 3e0f64;
            let v485 = parameters[42];
            let v490 = -5e-1f64;
            let v513 = parameters[36];
            let v515 = parameters[38];
            let v518 = parameters[41];
            let v520 = parameters[43];
            let v523 = parameters[48];
            let v525 = parameters[19];
            let v531 = parameters[18];
            let v532 = parameters[112];
            let v539 = parameters[70];
            let v540 = parameters[130];
            let v544 = parameters[71];
            let v545 = parameters[131];
            let v550 = 1e-3f64;
            let v553 = 1e3f64;
            let v593 = node_potentials[7];
            let v594 = node_potentials[8];
            let v597 = node_potentials[6];
            let v600 = node_potentials[5];
            let v603 = node_potentials[4];
            let v608 = node_potentials[9];
            let v611 = node_potentials[1];
            let v612 = node_potentials[2];
            let v616 = node_potentials[0];
            let v625 = node_potentials[10];
            let v626 = node_potentials[11];
            let v628 = parameters[34];
            let v630 = parameters[39];
            let v664 = -5e-1f64;
            let v701 = parameters[44];
            let v706 = -1e0f64;
            let v721 = parameters[45];
            let v751 = parameters[46];
            let v831 = -5e-1f64;
            let v900 = 1e-4f64;
            let v903 = 1e-8f64;
            let v912 = parameters[30];
            let v963 = parameters[32];
            let v979 = 5.0005e-1f64;
            let v980 = parameters[55];
            let v1004 = parameters[57];
            let v1297 = parameters[83];
            let v1299 = 2e-2f64;
            let v1302 = 1.01e0f64;
            let v1330 = parameters[85];
            let v1334 = parameters[87];
            let v1361 = parameters[97];
            let v1363 = parameters[95];
            let v1366 = parameters[94];
            let v1370 = 1e-1f64;
            let v1382 = parameters[96];
            let v1420 = parameters[2];
            let v1478 = -5e-1f64;
            let v1516 = -1e0f64;
            let v1639 = -5e-1f64;
            let v1675 = 1.44e0f64;
            let v1683 = parameters[76];
            let v1684 = parameters[77];
            let v1688 = parameters[78];
            let v1707 = parameters[81];
            let v1710 = parameters[47];
            let v1720 = parameters[35];
            let v1722 = parameters[40];
            let v1724 = parameters[102];
            let v1726 = parameters[82];
            let v1729 = 3.333333333333333e-1f64;
            let v1746 = parameters[1];
            let v1747 = 0e0f64;
            let v1748 = 0e0f64;
            let v1749 = 0e0f64;
            let v1750 = 0e0f64;
            let v1751 = 0e0f64;
            let v1752 = 0e0f64;
            let v1753 = 0e0f64;
            let v1754 = 0e0f64;
            let v1755 = 0e0f64;
            let v1756 = 0e0f64;
            let v1757 = 0e0f64;
            let v1758 = 0e0f64;
            let v1759 = 0e0f64;
            let v1773 = 1e0f64;
            let v1774 = 1e0f64;
            let v1775 = 1e0f64;
            let v1776 = 1e0f64;
            let v1777 = 1e0f64;
            let v1778 = 1e0f64;
            let v1779 = 1e0f64;
            let v1780 = 1e0f64;
            let v1781 = 1e0f64;
            let v1782 = 1e0f64;
            let v1783 = 1e0f64;
            let v1784 = 1e0f64;
            let v1785 = 1e0f64;
            let v1866 = -1e0f64;
            let v2096 = 2e0f64;
            let v2191 = 0e0f64;
            let v2357 = Lanes([0e0f64; 3]);
            let v2580 = Lanes([0e0f64; 3]);
            let v2709 = Lanes([0e0f64; 5]);
            let v3004 = Lanes([0e0f64; 4]);
            let v3096 = Lanes([0e0f64; 4]);
            let v3146 = Lanes([0e0f64; 3]);
            let v3303 = Lanes([0e0f64; 5]);
            let v3345 = Lanes([0e0f64; 4]);
            let v3384 = Lanes([0e0f64; 2]);
            let v3588 = Lanes([0e0f64; 3]);
            let v3891 = ddt_scale();
            if v1 != 0.0 {
            } else {
            }
            if v3 != 0.0 {
            } else {
            }
            let v1426: f64;
            if v5 != 0.0 {
                v1426 = v6;
            } else {
                let v8 = ctx.simparam_or("gmin", v7);
                v1426 = v8;
            }
            let v82: f64;
            if v9 != 0.0 {
                v82 = v10;
            } else {
                let v11 = ctx.simparam_or("pnjmaxi", v2);
                v82 = v11;
            }
            let v590: f64;
            if v12 != 0.0 {
                v590 = v2;
            } else {
                let v591: f64;
                if v13 != 0.0 {
                    v591 = v14;
                } else {
                    let v592: f64;
                    if v15 != 0.0 {
                        v592 = v16;
                    } else {
                        v592 = v2;
                    }
                    v591 = v592;
                }
                v590 = v591;
            }
            let v18 = v17.ln();
            let v20 = if v19 > v0 { 1.0 } else { 0.0 };
            let v22: f64;
            if v20 != 0.0 {
                let v21 = v2 / v19;
                v22 = v21;
            } else {
                v22 = v0;
            }
            let v24 = if v23 > v0 { 1.0 } else { 0.0 };
            let v26: f64;
            if v24 != 0.0 {
                let v25 = v2 / v23;
                v26 = v25;
            } else {
                v26 = v0;
            }
            let v28 = if v27 > v0 { 1.0 } else { 0.0 };
            let v30: f64;
            if v28 != 0.0 {
                let v29 = v2 / v27;
                v30 = v29;
            } else {
                v30 = v0;
            }
            let v32 = if v31 > v0 { 1.0 } else { 0.0 };
            let v34: f64;
            if v32 != 0.0 {
                let v33 = v2 / v31;
                v34 = v33;
            } else {
                v34 = v0;
            }
            let v36 = if v35 > v0 { 1.0 } else { 0.0 };
            let v38: f64;
            if v36 != 0.0 {
                let v37 = v2 / v35;
                v38 = v37;
            } else {
                v38 = v0;
            }
            let v39: f64;
            if v36 != 0.0 {
                v39 = v0;
            } else {
                v39 = v2;
            }
            let v42 = v40 + v41;
            let v45 = v43 + v44;
            let v46 = v45 - v40;
            let v48 = if v46 < v47 { 1.0 } else { 0.0 };
            if v48 != 0.0 {
            } else {
            }
            let v50 = if v46 > v49 { 1.0 } else { 0.0 };
            if v50 != 0.0 {
            } else {
            }
            let v52 = v51 + v2;
            let v53 = if v46 < v52 { 1.0 } else { 0.0 };
            let v65: f64;
            if v53 != 0.0 {
                let v57 = v51 + (((v46 - v51) - v2).exp());
                v65 = v57;
            } else {
                let v60 = if v46 > (v58 - v2) { 1.0 } else { 0.0 };
                let v66: f64;
                if v60 != 0.0 {
                    let v64 = v58 - (((v58 - v46) - v2).exp());
                    v66 = v64;
                } else {
                    v66 = v46;
                }
                v65 = v66;
            }
            let v67 = v65 + v40;
            let v71 = (v68 * v67) / v70;
            let v72 = v67 / v42;
            let v75 = if v74 > v0 { 1.0 } else { 0.0 };
            let v1025: f64;
            if v75 != 0.0 {
                let v77 = v76 * v71;
                let v86 = v77 * (((((-v78) / v77).exp()) + (v82 / v74)).ln());
                v1025 = v86;
            } else {
                v1025 = v0;
            }
            let v89 = v87 / v88;
            let v93 = -v92;
            let v94 = v2 - v72;
            let v96 = v71 * v88;
            let v99 = (v73 * (v72.powf(v89))) * (((v93 * v94) / v96).exp());
            let v100 = if v99 > v0 { 1.0 } else { 0.0 };
            let v862: f64;
            if v100 != 0.0 {
                let v104 = if (if v101 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v82 > v101 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v863: f64;
                if v104 != 0.0 {
                    let v118 = v96 * ((v2 + ((((v105 * v82) * ((v107 / v101).powf(v109))).powf((v2 / (v2 - v109)))) / v99)).ln());
                    v863 = v118;
                } else {
                    let v122 = v96 * ((v2 + (v82 / v99)).ln());
                    v863 = v122;
                }
                v862 = v863;
            } else {
                v862 = v0;
            }
            let v126 = v124 / v125;
            let v130 = -v129;
            let v132 = v71 * v125;
            let v135 = (v123 * (v72.powf(v126))) * (((v130 * v94) / v132).exp());
            let v137 = if v100 != 0.0 && (if v135 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v878: f64;
            if v137 != 0.0 {
                let v139 = if v20 != 0.0 && (if v82 > v19 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v879: f64;
                if v139 != 0.0 {
                    let v151 = v132 * ((v2 + ((((v105 * v82) * ((v107 / v19).powf(v109))).powf((v2 / (v2 - v109)))) / (v99 * v135))).ln());
                    v879 = v151;
                } else {
                    let v156 = v132 * ((v2 + (v82 / (v99 * v135))).ln());
                    v879 = v156;
                }
                v878 = v879;
            } else {
                v878 = v0;
            }
            let v159 = v87 / v158;
            let v163 = -v162;
            let v165 = v71 * v158;
            let v168 = (v157 * (v72.powf(v159))) * (((v163 * v94) / v165).exp());
            let v169 = if v168 > v0 { 1.0 } else { 0.0 };
            let v943: f64;
            if v169 != 0.0 {
                let v171 = if v24 != 0.0 && (if v82 > v23 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v944: f64;
                if v171 != 0.0 {
                    let v177 = v165 * ((v2 + (((v82 * v82) * v26) / v168)).ln());
                    v944 = v177;
                } else {
                    let v181 = v165 * ((v2 + (v82 / v168)).ln());
                    v944 = v181;
                }
                v943 = v944;
            } else {
                v943 = v0;
            }
            let v185 = v183 / v184;
            let v189 = -v188;
            let v191 = v71 * v184;
            let v194 = (v182 * (v72.powf(v185))) * (((v189 * v94) / v191).exp());
            let v195 = if v194 > v0 { 1.0 } else { 0.0 };
            let v983: f64;
            if v195 != 0.0 {
                let v199 = v191 * ((v2 + (v82 / v194)).ln());
                v983 = v199;
            } else {
                v983 = v0;
            }
            let v203 = v201 / v202;
            let v207 = -v206;
            let v209 = v71 * v202;
            let v212 = (v200 * (v72.powf(v203))) * (((v207 * v94) / v209).exp());
            let v213 = if v212 > v0 { 1.0 } else { 0.0 };
            let v994: f64;
            if v213 != 0.0 {
                let v217 = v209 * ((v2 + (v82 / v212)).ln());
                v994 = v217;
            } else {
                v994 = v0;
            }
            let v220 = v183 / v219;
            let v221 = v72.powf(v220);
            let v224 = -v223;
            let v226 = v71 * v219;
            let v228 = ((v224 * v94) / v226).exp();
            let v229 = (v218 * v221) * v228;
            let v230 = if v229 > v0 { 1.0 } else { 0.0 };
            let v1185: f64;
            if v230 != 0.0 {
                let v234 = v226 * ((v2 + (v82 / v229)).ln());
                v1185 = v234;
            } else {
                v1185 = v0;
            }
            let v237 = v201 / v236;
            let v238 = v72.powf(v237);
            let v241 = -v240;
            let v243 = v71 * v236;
            let v245 = ((v241 * v94) / v243).exp();
            let v246 = (v235 * v238) * v245;
            let v247 = if v246 > v0 { 1.0 } else { 0.0 };
            let v1196: f64;
            if v247 != 0.0 {
                let v251 = v243 * ((v2 + (v82 / v246)).ln());
                v1196 = v251;
            } else {
                v1196 = v0;
            }
            let v254 = (v252 * v221) * v228;
            let v255 = if v254 > v0 { 1.0 } else { 0.0 };
            let v1216: f64;
            if v255 != 0.0 {
                let v259 = v226 * ((v2 + (v82 / v254)).ln());
                v1216 = v259;
            } else {
                v1216 = v0;
            }
            let v262 = (v260 * v238) * v245;
            let v263 = if v262 > v0 { 1.0 } else { 0.0 };
            let v1226: f64;
            if v263 != 0.0 {
                let v267 = v243 * ((v2 + (v82 / v262)).ln());
                v1226 = v267;
            } else {
                v1226 = v0;
            }
            let v280 = if ((v268 * (v72.powf((v183 / v269)))) * ((((-v273) * v94) / (v71 * v269)).exp())) > v0 { 1.0 } else { 0.0 };
            if v280 != 0.0 {
            } else {
            }
            let v293 = if ((v281 * (v72.powf((v201 / v282)))) * ((((-v286) * v94) / (v71 * v282)).exp())) > v0 { 1.0 } else { 0.0 };
            if v293 != 0.0 {
            } else {
            }
            let v296 = (v45 + v294) - v40;
            let v297 = if v296 < v52 { 1.0 } else { 0.0 };
            let v308: f64;
            let v1786: f64;
            if v297 != 0.0 {
                let v300 = ((v296 - v51) - v2).exp();
                let v1870 = v1774 * v300;
                let v301 = v51 + v300;
                v308 = v301;
                v1786 = v1870;
            } else {
                let v303 = if v296 > (v58 - v2) { 1.0 } else { 0.0 };
                let v309: f64;
                let v1787: f64;
                if v303 != 0.0 {
                    let v306 = ((v58 - v296) - v2).exp();
                    let v307 = v58 - v306;
                    let v1869 = ((v1774 * v1866) * v306) * v1866;
                    v309 = v307;
                    v1787 = v1869;
                } else {
                    v309 = v296;
                    v1787 = v1774;
                }
                v308 = v309;
                v1786 = v1787;
            }
            let v310 = v308 + v40;
            let v312 = (v68 * v310) / v70;
            let v1872 = (v1786 * v68) / v70;
            let v313 = v310 / v42;
            let v1873 = v1786 / v42;
            let v314 = v310 - v42;
            let v317 = v101 * (v313.powf(v315));
            let v1878 = (v1873 * (v315 * (v313.powf((v315 - v1773))))) * v101;
            let v549: f64;
            let v1788: f64;
            if v318 != 0.0 {
                let v322 = v319 * (v313.powf(v320));
                let v1888 = (v1873 * (v320 * (v313.powf((v320 - v1773))))) * v319;
                v549 = v322;
                v1788 = v1888;
            } else {
                let v325 = v319 * (v313.powf(v323));
                let v1883 = (v1873 * (v323 * (v313.powf((v323 - v1773))))) * v319;
                v549 = v325;
                v1788 = v1883;
            }
            let v555: f64;
            let v1789: f64;
            if v326 != 0.0 {
                let v330 = v327 * (v313.powf(v328));
                let v1898 = (v1873 * (v328 * (v313.powf((v328 - v1773))))) * v327;
                v555 = v330;
                v1789 = v1898;
            } else {
                let v332 = v327 * (v313.powf(v323));
                let v1893 = (v1873 * (v323 * (v313.powf((v323 - v1773))))) * v327;
                v555 = v332;
                v1789 = v1893;
            }
            let v559: f64;
            let v1790: f64;
            if v333 != 0.0 {
                let v337 = v334 * (v313.powf(v335));
                let v1908 = (v1873 * (v335 * (v313.powf((v335 - v1773))))) * v334;
                v559 = v337;
                v1790 = v1908;
            } else {
                let v340 = v334 * (v313.powf(v338));
                let v1903 = (v1873 * (v338 * (v313.powf((v338 - v1773))))) * v334;
                v559 = v340;
                v1790 = v1903;
            }
            let v563: f64;
            let v1791: f64;
            if v341 != 0.0 {
                let v345 = v342 * (v313.powf(v343));
                let v1918 = (v1873 * (v343 * (v313.powf((v343 - v1773))))) * v342;
                v563 = v345;
                v1791 = v1918;
            } else {
                let v347 = v342 * (v313.powf(v338));
                let v1913 = (v1873 * (v338 * (v313.powf((v338 - v1773))))) * v342;
                v563 = v347;
                v1791 = v1913;
            }
            let v351 = v348 * (v313.powf(v349));
            let v1923 = (v1873 * (v349 * (v313.powf((v349 - v1773))))) * v348;
            let v355 = v352 * (v313.powf(v353));
            let v570: f64;
            let v1792: f64;
            if v356 != 0.0 {
                let v360 = v357 * (v313.powf(v358));
                let v1933 = (v1873 * (v358 * (v313.powf((v358 - v1773))))) * v357;
                v570 = v360;
                v1792 = v1933;
            } else {
                let v362 = v357 * (v313.powf(v323));
                let v1928 = (v1873 * (v323 * (v313.powf((v323 - v1773))))) * v357;
                v570 = v362;
                v1792 = v1928;
            }
            let v367 = v363 * (v2 + (v314 * v364));
            let v1935 = (v1786 * v364) * v363;
            let v369 = v73 * (v313.powf(v89));
            let v370 = v2 - v313;
            let v1941 = v1873 * v1866;
            let v371 = v93 * v370;
            let v1942 = v1941 * v93;
            let v372 = v312 * v88;
            let v373 = v371 / v372;
            let v374 = v373.exp();
            let v375 = v369 * v374;
            let v1950 = (((v1873 * (v89 * (v313.powf((v89 - v1773))))) * v73) * v374) + ((((v1942 - ((v1872 * v88) * v373)) / v372) * v374) * v369);
            let v377 = v123 * (v313.powf(v126));
            let v379 = v312 * v125;
            let v380 = (v130 * v370) / v379;
            let v381 = v380.exp();
            let v382 = v377 * v381;
            let v1964 = (((v1873 * (v126 * (v313.powf((v126 - v1773))))) * v123) * v381) + (((((v1941 * v130) - ((v1872 * v125) * v380)) / v379) * v381) * v377);
            let v384 = v157 * (v313.powf(v159));
            let v386 = v312 * v158;
            let v1971 = v1872 * v158;
            let v387 = (v163 * v370) / v386;
            let v388 = v387.exp();
            let v389 = v384 * v388;
            let v1978 = (((v1873 * (v159 * (v313.powf((v159 - v1773))))) * v157) * v388) + (((((v1941 * v163) - (v1971 * v387)) / v386) * v388) * v384);
            let v391 = v182 * (v313.powf(v185));
            let v393 = v312 * v184;
            let v1985 = v1872 * v184;
            let v394 = (v189 * v370) / v393;
            let v395 = v394.exp();
            let v396 = v391 * v395;
            let v1992 = (((v1873 * (v185 * (v313.powf((v185 - v1773))))) * v182) * v395) + (((((v1941 * v189) - (v1985 * v394)) / v393) * v395) * v391);
            let v398 = v200 * (v313.powf(v203));
            let v400 = v312 * v202;
            let v1999 = v1872 * v202;
            let v401 = (v207 * v370) / v400;
            let v402 = v401.exp();
            let v403 = v398 * v402;
            let v2006 = (((v1873 * (v203 * (v313.powf((v203 - v1773))))) * v200) * v402) + (((((v1941 * v207) - (v1999 * v401)) / v400) * v402) * v398);
            let v404 = v313.powf(v220);
            let v2010 = v1873 * (v220 * (v313.powf((v220 - v1773))));
            let v405 = v218 * v404;
            let v407 = v312 * v219;
            let v2013 = v1872 * v219;
            let v408 = (v224 * v370) / v407;
            let v409 = v408.exp();
            let v2017 = (((v1941 * v224) - (v2013 * v408)) / v407) * v409;
            let v410 = v405 * v409;
            let v2020 = ((v2010 * v218) * v409) + (v2017 * v405);
            let v411 = v313.powf(v237);
            let v2024 = v1873 * (v237 * (v313.powf((v237 - v1773))));
            let v412 = v235 * v411;
            let v414 = v312 * v236;
            let v2027 = v1872 * v236;
            let v415 = (v241 * v370) / v414;
            let v416 = v415.exp();
            let v2031 = (((v1941 * v241) - (v2027 * v415)) / v414) * v416;
            let v417 = v412 * v416;
            let v2034 = ((v2024 * v235) * v416) + (v2031 * v412);
            let v418 = v252 * v404;
            let v419 = v418 * v409;
            let v2038 = ((v2010 * v252) * v409) + (v2017 * v418);
            let v420 = v260 * v411;
            let v421 = v420 * v416;
            let v2042 = ((v2024 * v260) * v416) + (v2031 * v420);
            let v2043 = v1786 * v422;
            let v424 = v2 + (v314 * v422);
            let v425 = v88 * v424;
            let v2044 = v2043 * v88;
            let v426 = v125 * v424;
            let v2045 = v2043 * v125;
            let v431 = v427 * (v2 + (v314 * v428));
            let v2047 = (v1786 * v428) * v427;
            let v436 = v432 * (v2 + (v314 * v433));
            let v2049 = (v1786 * v433) * v432;
            let v440 = v437 + (v314 * v438);
            let v447 = v76 * (v2 + (v314 * v444));
            let v449 = v312 / v313;
            let v450 = v448 * v449;
            let v2060 = ((v1872 - (v1873 * v449)) / v313) * v448;
            let v452 = v105 * v451;
            let v454 = (v452 * v313) / v312;
            let v455 = v454.exp();
            let v457 = v456 * v451;
            let v459 = (v457 * v313) / v312;
            let v460 = v459.exp();
            let v461 = v455 - v460;
            let v462 = v461.ln();
            let v463 = v450 * v462;
            let v466 = v465 * v312;
            let v467 = v313.ln();
            let v468 = v466 * v467;
            let v2085 = ((v1872 * v465) * v467) + ((v1873 * (v1773 / v313)) * v466);
            let v470 = v313 - v2;
            let v472 = ((v463 * v313) - v468) - (v188 * v470);
            let v2088 = (((((v2060 * v462) + (((((((v1873 * v452) - (v1872 * v454)) / v312) * v455) - ((((v1873 * v457) - (v1872 * v459)) / v312) * v460)) * (v1773 / v461)) * v450)) * v313) + (v1873 * v463)) - v2085) - (v1873 * v188);
            let v473 = v448 * v312;
            let v2089 = v1872 * v448;
            let v475 = (-v472) / v312;
            let v476 = v475.exp();
            let v479 = (v2 + (v107 * v476)).sqrt();
            let v481 = v105 * (v2 + v479);
            let v482 = v481.ln();
            let v484 = v472 + (v473 * v482);
            let v2106 = v2088 + ((v2089 * v482) + (((((((((v2088 * v1866) - (v1872 * v475)) / v312) * v476) * v107) * (v1773 / (v2096 * v479))) * v105) * (v1773 / v481)) * v473));
            let v486 = v105 * v485;
            let v488 = (v486 * v313) / v312;
            let v489 = v488.exp();
            let v491 = v490 * v485;
            let v493 = (v491 * v313) / v312;
            let v494 = v493.exp();
            let v495 = v489 - v494;
            let v496 = v495.ln();
            let v497 = v450 * v496;
            let v501 = ((v497 * v313) - v468) - (v223 * v470);
            let v2128 = (((((v2060 * v496) + (((((((v1873 * v486) - (v1872 * v488)) / v312) * v489) - ((((v1873 * v491) - (v1872 * v493)) / v312) * v494)) * (v1773 / v495)) * v450)) * v313) + (v1873 * v497)) - v2085) - (v1873 * v223);
            let v503 = (-v501) / v312;
            let v504 = v503.exp();
            let v507 = (v2 + (v107 * v504)).sqrt();
            let v509 = v105 * (v2 + v507);
            let v510 = v509.ln();
            let v512 = v501 + (v473 * v510);
            let v2144 = v2128 + ((v2089 * v510) + (((((((((v2128 * v1866) - (v1872 * v503)) / v312) * v504) * v107) * (v1773 / (v2096 * v507))) * v105) * (v1773 / v509)) * v473));
            let v514 = v451 / v484;
            let v517 = v513 * (v514.powf(v515));
            let v2152 = ((((v2106 * v514) * v1866) / v484) * (v515 * (v514.powf((v515 - v1773))))) * v513;
            let v519 = v485 / v512;
            let v521 = v519.powf(v520);
            let v2159 = (((v2144 * v519) * v1866) / v512) * (v520 * (v519.powf((v520 - v1773))));
            let v522 = v518 * v521;
            let v2160 = v2159 * v518;
            let v524 = v523 * v521;
            let v2161 = v2159 * v523;
            let v527 = v525 * (v313.powf(v87));
            let v528 = v371 / v312;
            let v529 = v528.exp();
            let v530 = v527 * v529;
            let v2173 = (((v1873 * (v87 * (v313.powf((v87 - v1773))))) * v525) * v529) + ((((v1942 - (v1872 * v528)) / v312) * v529) * v527);
            let v534 = v531 * (v313.powf(v532));
            let v2178 = (v1873 * (v532 * (v313.powf((v532 - v1773))))) * v531;
            let v535 = -(v78 * (v2 + (v314 * v440)));
            let v2179 = (((v1786 * v440) + ((v1786 * v438) * v314)) * v78) * v1866;
            let v536 = v447 * v312;
            let v2182 = (((v1786 * v444) * v76) * v312) + (v1872 * v447);
            let v537 = v535 / v536;
            let v538 = v537.exp();
            let v2186 = ((v2179 - (v2182 * v537)) / v536) * v538;
            let v543 = v539 * (v2 + (v314 * v540));
            let v2188 = (v1786 * v540) * v539;
            let v548 = v544 * (v2 + (v314 * v545));
            let v2190 = (v1786 * v545) * v544;
            let v551 = if v549 > v550 { 1.0 } else { 0.0 };
            let v554: f64;
            let v1793: f64;
            if v551 != 0.0 {
                let v552 = v2 / v549;
                let v2194 = ((v1788 * v552) * v1866) / v549;
                v554 = v552;
                v1793 = v2194;
            } else {
                v554 = v553;
                v1793 = v2191;
            }
            let v556 = if v555 > v550 { 1.0 } else { 0.0 };
            let v558: f64;
            let v1794: f64;
            if v556 != 0.0 {
                let v557 = v2 / v555;
                let v2197 = ((v1789 * v557) * v1866) / v555;
                v558 = v557;
                v1794 = v2197;
            } else {
                v558 = v553;
                v1794 = v2191;
            }
            let v560 = if v559 > v550 { 1.0 } else { 0.0 };
            let v562: f64;
            let v1795: f64;
            if v560 != 0.0 {
                let v561 = v2 / v559;
                let v2200 = ((v1790 * v561) * v1866) / v559;
                v562 = v561;
                v1795 = v2200;
            } else {
                v562 = v553;
                v1795 = v2191;
            }
            let v564 = if v563 > v550 { 1.0 } else { 0.0 };
            let v566: f64;
            let v1796: f64;
            if v564 != 0.0 {
                let v565 = v2 / v563;
                let v2203 = ((v1791 * v565) * v1866) / v563;
                v566 = v565;
                v1796 = v2203;
            } else {
                v566 = v553;
                v1796 = v2191;
            }
            let v567 = if v351 > v550 { 1.0 } else { 0.0 };
            let v569: f64;
            let v1797: f64;
            if v567 != 0.0 {
                let v568 = v2 / v351;
                let v2206 = ((v1923 * v568) * v1866) / v351;
                v569 = v568;
                v1797 = v2206;
            } else {
                v569 = v553;
                v1797 = v2191;
            }
            let v571 = if v570 > v550 { 1.0 } else { 0.0 };
            let v573: f64;
            let v1798: f64;
            if v571 != 0.0 {
                let v572 = v2 / v570;
                let v2209 = ((v1792 * v572) * v1866) / v570;
                v573 = v572;
                v1798 = v2209;
            } else {
                v573 = v553;
                v1798 = v2191;
            }
            let v574 = if v355 > v550 { 1.0 } else { 0.0 };
            if v574 != 0.0 {
            } else {
            }
            let v575 = if v367 > v550 { 1.0 } else { 0.0 };
            let v577: f64;
            let v1799: f64;
            if v575 != 0.0 {
                let v576 = v2 / v367;
                let v2212 = ((v1935 * v576) * v1866) / v367;
                v577 = v576;
                v1799 = v2212;
            } else {
                v577 = v553;
                v1799 = v2191;
            }
            let v578 = if v543 > v0 { 1.0 } else { 0.0 };
            let v580: f64;
            let v1800: f64;
            if v578 != 0.0 {
                let v579 = v2 / v543;
                let v2215 = ((v2188 * v579) * v1866) / v543;
                v580 = v579;
                v1800 = v2215;
            } else {
                v580 = v0;
                v1800 = v2191;
            }
            let v581 = if v548 > v0 { 1.0 } else { 0.0 };
            let v583: f64;
            let v1801: f64;
            if v581 != 0.0 {
                let v582 = v2 / v548;
                let v2218 = ((v2190 * v582) * v1866) / v548;
                v583 = v582;
                v1801 = v2218;
            } else {
                v583 = v0;
                v1801 = v2191;
            }
            let v584 = if v317 > v0 { 1.0 } else { 0.0 };
            let v586: f64;
            let v1802: f64;
            if v584 != 0.0 {
                let v585 = v2 / v317;
                let v2221 = ((v1878 * v585) * v1866) / v317;
                v586 = v585;
                v1802 = v2221;
            } else {
                v586 = v0;
                v1802 = v2191;
            }
            let v587 = if v534 > v0 { 1.0 } else { 0.0 };
            let v589: f64;
            let v1803: f64;
            if v587 != 0.0 {
                let v588 = v2 / v534;
                let v2224 = ((v2178 * v588) * v1866) / v534;
                v589 = v588;
                v1803 = v2224;
            } else {
                v589 = v0;
                v1803 = v2191;
            }
            let v596 = v590 * (v593 - v594);
            let v2228 = ((Lanes([v1775, 0.0])) - (Lanes([0.0, v1776]))) * v590;
            let v599 = v590 * (v597 - v594);
            let v2232 = ((Lanes([v1777, 0.0])) - (Lanes([0.0, v1776]))) * v590;
            let v602 = v590 * (v593 - v600);
            let v2236 = ((Lanes([0.0, v1775])) - (Lanes([v1778, 0.0]))) * v590;
            let v605 = v590 * (v593 - v603);
            let v2240 = ((Lanes([0.0, v1775])) - (Lanes([v1779, 0.0]))) * v590;
            let v607 = v590 * (v597 - v603);
            let v2244 = ((Lanes([0.0, v1777])) - (Lanes([v1779, 0.0]))) * v590;
            let v610 = v590 * (v597 - v608);
            let v2248 = ((Lanes([v1777, 0.0])) - (Lanes([0.0, v1780]))) * v590;
            let v613 = v611 - v612;
            let v2251 = (Lanes([v1781, 0.0])) - (Lanes([0.0, v1782]));
            let v615 = v590 * (v600 - v594);
            let v2255 = ((Lanes([v1778, 0.0])) - (Lanes([0.0, v1776]))) * v590;
            let v617 = v611 - v616;
            let v2258 = (Lanes([0.0, v1781])) - (Lanes([v1783, 0.0]));
            let v618 = v616 - v603;
            let v2261 = (Lanes([v1783, 0.0])) - (Lanes([0.0, v1779]));
            let v620 = v590 * (v603 - v600);
            let v2265 = ((Lanes([v1779, 0.0])) - (Lanes([0.0, v1778]))) * v590;
            let v621 = v611 - v597;
            let v2268 = (Lanes([v1781, 0.0])) - (Lanes([0.0, v1777]));
            let v622 = v597 - v593;
            let v2271 = (Lanes([v1777, 0.0])) - (Lanes([0.0, v1775]));
            let v623 = v612 - v594;
            let v2274 = (Lanes([v1782, 0.0])) - (Lanes([0.0, v1776]));
            let v624 = v608 - v603;
            let v2277 = (Lanes([0.0, v1780])) - (Lanes([v1779, 0.0]));
            let v627 = -v484;
            let v2278 = v2106 * v1866;
            let v629 = v627 * v628;
            let v2279 = v2278 * v628;
            let v631 = if v630 <= v0 { 1.0 } else { 0.0 };
            let v893: f64;
            let v1804: Lanes<3>;
            if v631 != 0.0 {
                let v632 = v596 + v629;
                let v2339 = Lanes([0.0, v2228[0], v2228[1]]);
                let v2341 = v2339 + (Lanes([v2279, 0.0, 0.0]));
                let v633 = if v632 > v0 { 1.0 } else { 0.0 };
                let v656: f64;
                let v657: f64;
                let v1805: Lanes<3>;
                let v1806: Lanes<3>;
                if v633 != 0.0 {
                    let v634 = v2 - v628;
                    let v636 = v634.powf((-v515));
                    let v638 = v2 - (v636 * v634);
                    let v640 = v2 - v515;
                    let v641 = (v484 * v638) / v640;
                    let v642 = v105 * v515;
                    let v644 = v484 * v634;
                    let v645 = (v642 * v632) / v644;
                    let v646 = v2 + v645;
                    let v648 = (v632 * v646) * v636;
                    let v2369 = ((v2341 * v646) + ((((v2341 * v642) - (Lanes([((v2106 * v634) * v645), 0.0, 0.0]))) / v644) * v632)) * v636;
                    let v2370 = Lanes([((v2106 * v638) / v640), 0.0, 0.0]);
                    v656 = v641;
                    v657 = v648;
                    v1805 = v2370;
                    v1806 = v2369;
                } else {
                    let v649 = v596 / v484;
                    let v650 = v2 - v649;
                    let v651 = v2 - v515;
                    let v653 = v2 - (v650.powf(v651));
                    let v655 = (v484 * v653) / v651;
                    let v2356 = ((Lanes([(v2106 * v653), 0.0, 0.0])) + ((((((v2339 - (Lanes([(v2106 * v649), 0.0, 0.0]))) / v484) * v1866) * (v651 * (v650.powf((v651 - v1773))))) * v1866) * v484)) / v651;
                    v656 = v655;
                    v657 = v0;
                    v1805 = v2356;
                    v1806 = v2357;
                }
                let v658 = v656 + v657;
                let v2371 = v1805 + v1806;
                v893 = v658;
                v1804 = v2371;
            } else {
                let v2280 = v2279 * v629;
                let v661 = (v107 * v630) * v630;
                let v663 = ((v629 * v629) + v661).sqrt();
                let v666 = v664 * (v629 + v663);
                let v2286 = (v2279 + ((v2280 + v2280) * (v1773 / (v2096 * v663)))) * v664;
                let v667 = v666 / v484;
                let v668 = v2 - v667;
                let v669 = v2 - v515;
                let v670 = v668.powf(v669);
                let v2291 = v669 - v1773;
                let v673 = v596 + v629;
                let v2299 = Lanes([0.0, v2228[0], v2228[1]]);
                let v2300 = Lanes([v2279, 0.0, 0.0]);
                let v2301 = v2299 + v2300;
                let v2302 = v2301 * v673;
                let v676 = ((v673 * v673) + v661).sqrt();
                let v679 = (v105 * (v673 - v676)) - v629;
                let v2309 = ((v2301 - ((v2302 + v2302) * (v1773 / (v2096 * v676)))) * v105) - v2300;
                let v680 = v679 / v484;
                let v681 = v2 - v680;
                let v682 = v681.powf(v669);
                let v685 = v2 - v628;
                let v687 = v685.powf((-v515));
                let v689 = (v596 - v679) + v666;
                let v2325 = (v2299 - v2309) + (Lanes([v2286, 0.0, 0.0]));
                let v690 = v687 * v689;
                let v691 = v105 * v515;
                let v693 = v484 * v685;
                let v694 = (v691 * v689) / v693;
                let v695 = v2 + v694;
                let v698 = (((v627 * v682) / v669) + (v690 * v695)) - ((v627 * v670) / v669);
                let v2338 = ((((Lanes([(v2278 * v682), 0.0, 0.0])) + (((((v2309 - (Lanes([(v2106 * v680), 0.0, 0.0]))) / v484) * v1866) * (v669 * (v681.powf(v2291)))) * v627)) / v669) + (((v2325 * v687) * v695) + ((((v2325 * v691) - (Lanes([((v2106 * v685) * v694), 0.0, 0.0]))) / v693) * v690))) - (Lanes([(((v2278 * v670) + (((((v2286 - (v2106 * v667)) / v484) * v1866) * (v669 * (v668.powf(v2291)))) * v627)) / v669), 0.0, 0.0]));
                v893 = v698;
                v1804 = v2338;
            }
            let v699 = -v512;
            let v2372 = v2144 * v1866;
            let v700 = v699 * v628;
            let v2373 = v2372 * v628;
            let v702 = if v701 <= v0 { 1.0 } else { 0.0 };
            let v896: f64;
            let v1807: Lanes<3>;
            if v702 != 0.0 {
                let v703 = v602 + v700;
                let v2538 = Lanes([0.0, v2236[0], v2236[1]]);
                let v2540 = v2538 + (Lanes([v2373, 0.0, 0.0]));
                let v704 = if v703 > v0 { 1.0 } else { 0.0 };
                let v746: f64;
                let v748: f64;
                let v1808: Lanes<3>;
                let v1809: Lanes<3>;
                if v704 != 0.0 {
                    let v705 = v2 - v628;
                    let v708 = v705.powf((v706 - v520));
                    let v711 = v2 - ((v708 * v705) * v705);
                    let v713 = v2 - v520;
                    let v714 = (v512 * v711) / v713;
                    let v715 = v105 * v520;
                    let v717 = (v715 * v703) / v512;
                    let v718 = v705 + v717;
                    let v720 = (v703 * v718) * v708;
                    let v2591 = ((v2540 * v718) + ((((v2540 * v715) - (Lanes([(v2144 * v717), 0.0, 0.0]))) / v512) * v703)) * v708;
                    let v2592 = Lanes([((v2144 * v711) / v713), 0.0, 0.0]);
                    v746 = v714;
                    v748 = v720;
                    v1808 = v2592;
                    v1809 = v2591;
                } else {
                    let v725 = if (if v721 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v602 < (-v721) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v747: f64;
                    let v1810: Lanes<3>;
                    if v725 != 0.0 {
                        let v726 = v721 / v512;
                        let v727 = v2 + v726;
                        let v728 = v2 - v520;
                        let v729 = v727.powf(v728);
                        let v2563 = v2236 * v728;
                        let v732 = v512 + v721;
                        let v733 = (v728 * (v602 + v721)) / v732;
                        let v734 = v2 - v733;
                        let v736 = v2 - (v729 * v734);
                        let v738 = (v512 * v736) / v728;
                        let v2579 = ((Lanes([(v2144 * v736), 0.0, 0.0])) + ((((Lanes([(((((v2144 * v726) * v1866) / v512) * (v728 * (v727.powf((v728 - v1773))))) * v734), 0.0, 0.0])) + (((((Lanes([0.0, v2563[0], v2563[1]])) - (Lanes([(v2144 * v733), 0.0, 0.0]))) / v732) * v1866) * v729)) * v1866) * v512)) / v728;
                        v747 = v738;
                        v1810 = v2579;
                    } else {
                        let v739 = v602 / v512;
                        let v740 = v2 - v739;
                        let v741 = v2 - v520;
                        let v743 = v2 - (v740.powf(v741));
                        let v745 = (v512 * v743) / v741;
                        let v2555 = ((Lanes([(v2144 * v743), 0.0, 0.0])) + ((((((v2538 - (Lanes([(v2144 * v739), 0.0, 0.0]))) / v512) * v1866) * (v741 * (v740.powf((v741 - v1773))))) * v1866) * v512)) / v741;
                        v747 = v745;
                        v1810 = v2555;
                    }
                    v746 = v747;
                    v748 = v0;
                    v1808 = v1810;
                    v1809 = v2580;
                }
                let v749 = v746 + v748;
                let v2593 = v1808 + v1809;
                v896 = v749;
                v1807 = v2593;
            } else {
                let v753 = if (if v721 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v751 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v897: f64;
                let v1811: Lanes<3>;
                if v753 != 0.0 {
                    let v755 = v721 - v700;
                    let v2424 = v2373 * v1866;
                    let v756 = (v721 + v700) / v755;
                    let v2427 = (v2373 - (v2424 * v756)) / v755;
                    let v758 = v756 - v2;
                    let v2429 = v2427 * v758;
                    let v761 = (v107 * v701) * v701;
                    let v763 = ((v758 * v758) + v761).sqrt();
                    let v764 = v756 + v2;
                    let v2434 = v2427 * v764;
                    let v767 = (v107 * v751) * v751;
                    let v769 = ((v764 * v764) + v767).sqrt();
                    let v770 = v763 + v769;
                    let v771 = (v448 * v756) / v770;
                    let v775 = v105 * (((v771 * v755) - v721) - v700);
                    let v2447 = ((((((v2427 * v448) - ((((v2429 + v2429) * (v1773 / (v2096 * v763))) + ((v2434 + v2434) * (v1773 / (v2096 * v769)))) * v771)) / v770) * v755) + (v2424 * v771)) - v2373) * v105;
                    let v776 = v775 / v512;
                    let v777 = v2 - v776;
                    let v778 = v2 - v520;
                    let v2452 = v778 - v1773;
                    let v780 = v2 - (v777.powf(v778));
                    let v2461 = v2236 * v448;
                    let v2463 = Lanes([v2373, 0.0, 0.0]);
                    let v786 = (((v448 * v602) + v721) + v700) / v755;
                    let v2468 = (((Lanes([0.0, v2461[0], v2461[1]])) + v2463) - (Lanes([(v2424 * v786), 0.0, 0.0]))) / v755;
                    let v788 = v786 - v2;
                    let v2470 = v2468 * v788;
                    let v791 = ((v788 * v788) + v761).sqrt();
                    let v792 = v786 + v2;
                    let v2475 = v2468 * v792;
                    let v795 = ((v792 * v792) + v767).sqrt();
                    let v796 = v791 + v795;
                    let v797 = (v448 * v786) / v796;
                    let v2483 = ((v2468 * v448) - ((((v2470 + v2470) * (v1773 / (v2096 * v791))) + ((v2475 + v2475) * (v1773 / (v2096 * v795)))) * v797)) / v796;
                    let v801 = v105 * (((v797 * v755) - v721) - v700);
                    let v2489 = (((v2483 * v755) + (Lanes([(v2424 * v797), 0.0, 0.0]))) - v2463) * v105;
                    let v802 = v801 / v512;
                    let v803 = v2 - v802;
                    let v805 = v2 - (v803.powf(v778));
                    let v809 = v105 * (v797 + v2);
                    let v2504 = v2483 * v105;
                    let v810 = v721 / v512;
                    let v811 = v2 + v810;
                    let v812 = -v520;
                    let v813 = v811.powf(v812);
                    let v2508 = v812 - v1773;
                    let v814 = v700 / v512;
                    let v815 = v2 + v814;
                    let v816 = v815.powf(v812);
                    let v817 = v2 - v809;
                    let v820 = (v817 * v813) + (v809 * v816);
                    let v822 = (v602 - v801) + v775;
                    let v825 = ((v822 * v820) + ((v512 * v805) / v778)) - ((v512 * v780) / v778);
                    let v2537 = ((((((Lanes([0.0, v2236[0], v2236[1]])) - v2489) + (Lanes([v2447, 0.0, 0.0]))) * v820) + (((((v2504 * v1866) * v813) + (Lanes([(((((v2144 * v810) * v1866) / v512) * (v812 * (v811.powf(v2508)))) * v817), 0.0, 0.0]))) + ((v2504 * v816) + (Lanes([((((v2373 - (v2144 * v814)) / v512) * (v812 * (v815.powf(v2508)))) * v809), 0.0, 0.0])))) * v822)) + (((Lanes([(v2144 * v805), 0.0, 0.0])) + ((((((v2489 - (Lanes([(v2144 * v802), 0.0, 0.0]))) / v512) * v1866) * (v778 * (v803.powf(v2452)))) * v1866) * v512)) / v778)) - (Lanes([(((v2144 * v780) + ((((((v2447 - (v2144 * v776)) / v512) * v1866) * (v778 * (v777.powf(v2452)))) * v1866) * v512)) / v778), 0.0, 0.0]));
                    v897 = v825;
                    v1811 = v2537;
                } else {
                    let v2374 = v2373 * v700;
                    let v828 = (v107 * v701) * v701;
                    let v830 = ((v700 * v700) + v828).sqrt();
                    let v833 = v831 * (v700 + v830);
                    let v2380 = (v2373 + ((v2374 + v2374) * (v1773 / (v2096 * v830)))) * v831;
                    let v834 = v833 / v512;
                    let v835 = v2 - v834;
                    let v836 = v2 - v520;
                    let v837 = v835.powf(v836);
                    let v2385 = v836 - v1773;
                    let v840 = v602 + v700;
                    let v2393 = Lanes([0.0, v2236[0], v2236[1]]);
                    let v2394 = Lanes([v2373, 0.0, 0.0]);
                    let v2395 = v2393 + v2394;
                    let v2396 = v2395 * v840;
                    let v843 = ((v840 * v840) + v828).sqrt();
                    let v846 = (v105 * (v840 - v843)) - v700;
                    let v2403 = ((v2395 - ((v2396 + v2396) * (v1773 / (v2096 * v843)))) * v105) - v2394;
                    let v847 = v846 / v512;
                    let v848 = v2 - v847;
                    let v849 = v848.powf(v836);
                    let v854 = (v2 - v628).powf((-v520));
                    let v859 = (((v699 * v849) / v836) + (v854 * ((v602 - v846) + v833))) - ((v699 * v837) / v836);
                    let v2423 = ((((Lanes([(v2372 * v849), 0.0, 0.0])) + (((((v2403 - (Lanes([(v2144 * v847), 0.0, 0.0]))) / v512) * v1866) * (v836 * (v848.powf(v2385)))) * v699)) / v836) + (((v2393 - v2403) + (Lanes([v2380, 0.0, 0.0]))) * v854)) - (Lanes([(((v2372 * v837) + (((((v2380 - (v2144 * v834)) / v512) * v1866) * (v836 * (v835.powf(v2385)))) * v699)) / v836), 0.0, 0.0]));
                    v897 = v859;
                    v1811 = v2423;
                }
                v896 = v897;
                v1807 = v1811;
            }
            let v860 = v425 * v312;
            let v861 = v2 / v860;
            let v2599 = ((((v2044 * v312) + (v1872 * v425)) * v861) * v1866) / v860;
            let v864 = if v596 < v862 { 1.0 } else { 0.0 };
            let v873: f64;
            let v1812: Lanes<3>;
            if v864 != 0.0 {
                let v2611 = v2228 * v861;
                let v866 = (v596 * v861).exp();
                let v2616 = ((Lanes([0.0, v2611[0], v2611[1]])) + (Lanes([(v2599 * v596), 0.0, 0.0]))) * v866;
                v873 = v866;
                v1812 = v2616;
            } else {
                let v868 = (v862 * v861).exp();
                let v869 = v596 - v862;
                let v2602 = v2228 * v861;
                let v871 = v2 + (v869 * v861);
                let v872 = v868 * v871;
                let v2610 = (Lanes([(((v2599 * v862) * v868) * v871), 0.0, 0.0])) + (((Lanes([0.0, v2602[0], v2602[1]])) + (Lanes([(v2599 * v869), 0.0, 0.0]))) * v868);
                v873 = v872;
                v1812 = v2610;
            }
            let v874 = v873 - v2;
            let v875 = v375 * v874;
            let v2620 = (Lanes([(v1950 * v874), 0.0, 0.0])) + (v1812 * v375);
            let v876 = v426 * v312;
            let v877 = v2 / v876;
            let v2626 = ((((v2045 * v312) + (v1872 * v426)) * v877) * v1866) / v876;
            let v880 = if v602 < v878 { 1.0 } else { 0.0 };
            let v890: f64;
            let v1813: Lanes<3>;
            if v880 != 0.0 {
                let v2638 = v2236 * v877;
                let v882 = (v602 * v877).exp();
                let v2643 = ((Lanes([0.0, v2638[0], v2638[1]])) + (Lanes([(v2626 * v602), 0.0, 0.0]))) * v882;
                v890 = v882;
                v1813 = v2643;
            } else {
                let v884 = (v878 * v877).exp();
                let v885 = v602 - v878;
                let v2629 = v2236 * v877;
                let v887 = v2 + (v885 * v877);
                let v888 = v884 * v887;
                let v2637 = (Lanes([(((v2626 * v878) * v884) * v887), 0.0, 0.0])) + (((Lanes([0.0, v2629[0], v2629[1]])) + (Lanes([(v2626 * v885), 0.0, 0.0]))) * v884);
                v890 = v888;
                v1813 = v2637;
            }
            let v889 = v375 * v382;
            let v891 = v890 - v2;
            let v892 = v889 * v891;
            let v2650 = (Lanes([(((v1950 * v382) + (v1964 * v375)) * v891), 0.0, 0.0])) + (v1813 * v889);
            let v2654 = (v1804 * v583) + (Lanes([(v1801 * v893), 0.0, 0.0]));
            let v2658 = (v1807 * v580) + (Lanes([(v1800 * v896), 0.0, 0.0]));
            let v2661 = (Lanes([v2654[0], 0.0, v2654[1], v2654[2]])) + (Lanes([v2658[0], v2658[1], v2658[2], 0.0]));
            let v901 = ((v2 + (v893 * v583)) + (v896 * v580)) - v900;
            let v2662 = v2661 * v901;
            let v905 = ((v901 * v901) + v903).sqrt();
            let v2668 = (((v2662 + v2662) * (v1773 / (v2096 * v905))) + v2661) * v105;
            let v908 = (v105 * (v905 + v901)) + v900;
            let v2672 = (v2620 * v586) + (Lanes([(v1802 * v875), 0.0, 0.0]));
            let v2673 = v2650 * v22;
            let v911 = (v875 * v586) + (v892 * v22);
            let v2676 = (Lanes([v2672[0], 0.0, v2672[1], v2672[2]])) + (Lanes([v2673[0], v2673[1], v2673[2], 0.0]));
            let v913 = if v912 < v105 { 1.0 } else { 0.0 };
            let v936: f64;
            let v1814: Lanes<4>;
            if v913 != 0.0 {
                let v914 = v2 / v109;
                let v917 = (v908.powf(v914)) + (v107 * v911);
                let v2693 = (v2668 * (v914 * (v908.powf((v914 - v1773))))) + (v2676 * v107);
                let v918 = if v917 > v903 { 1.0 } else { 0.0 };
                let v937: f64;
                let v1815: Lanes<4>;
                if v918 != 0.0 {
                    let v921 = v105 * (v908 + (v917.powf(v109)));
                    let v2700 = (v2668 + (v2693 * (v109 * (v917.powf((v109 - v1773)))))) * v105;
                    v937 = v921;
                    v1815 = v2700;
                } else {
                    let v924 = v105 * (v908 + (v903.powf(v109)));
                    let v2694 = v2668 * v105;
                    v937 = v924;
                    v1815 = v2694;
                }
                v936 = v937;
                v1814 = v1815;
            } else {
                let v2677 = v2676 * v107;
                let v926 = v2 + (v107 * v911);
                let v927 = if v926 > v903 { 1.0 } else { 0.0 };
                let v938: f64;
                let v1816: Lanes<4>;
                if v927 != 0.0 {
                    let v928 = v105 * v908;
                    let v930 = v2 + (v926.powf(v109));
                    let v931 = v928 * v930;
                    let v2687 = ((v2668 * v105) * v930) + ((v2677 * (v109 * (v926.powf((v109 - v1773))))) * v928);
                    v938 = v931;
                    v1816 = v2687;
                } else {
                    let v934 = v2 + (v903.powf(v109));
                    let v935 = (v105 * v908) * v934;
                    let v2679 = (v2668 * v105) * v934;
                    v938 = v935;
                    v1816 = v2679;
                }
                v936 = v938;
                v1814 = v1816;
            }
            let v939 = v892 / v936;
            let v2704 = ((Lanes([v2650[0], v2650[1], v2650[2], 0.0])) - (v1814 * v939)) / v936;
            let v940 = v875 / v936;
            let v2708 = ((Lanes([v2620[0], 0.0, v2620[1], v2620[2]])) - (v1814 * v940)) / v936;
            let v941 = if v157 > v0 { 1.0 } else { 0.0 };
            let v1293: f64;
            let v1717: f64;
            let v1817: Lanes<5>;
            let v1818: Lanes<5>;
            if v941 != 0.0 {
                let v942 = v2 / v386;
                let v2712 = ((v1971 * v942) * v1866) / v386;
                let v945 = if v610 < v943 { 1.0 } else { 0.0 };
                let v964: f64;
                let v1819: Lanes<3>;
                if v945 != 0.0 {
                    let v2724 = v2248 * v942;
                    let v947 = (v610 * v942).exp();
                    let v2729 = ((Lanes([0.0, v2724[0], v2724[1]])) + (Lanes([(v2712 * v610), 0.0, 0.0]))) * v947;
                    v964 = v947;
                    v1819 = v2729;
                } else {
                    let v949 = (v943 * v942).exp();
                    let v950 = v610 - v943;
                    let v2715 = v2248 * v942;
                    let v952 = v2 + (v950 * v942);
                    let v953 = v949 * v952;
                    let v2723 = (Lanes([(((v2712 * v943) * v949) * v952), 0.0, 0.0])) + (((Lanes([0.0, v2715[0], v2715[1]])) + (Lanes([(v2712 * v950), 0.0, 0.0]))) * v949);
                    v964 = v953;
                    v1819 = v2723;
                }
                let v954 = if v602 < v943 { 1.0 } else { 0.0 };
                let v967: f64;
                let v1820: Lanes<3>;
                if v954 != 0.0 {
                    let v2741 = v2236 * v942;
                    let v956 = (v602 * v942).exp();
                    let v2746 = ((Lanes([0.0, v2741[0], v2741[1]])) + (Lanes([(v2712 * v602), 0.0, 0.0]))) * v956;
                    v967 = v956;
                    v1820 = v2746;
                } else {
                    let v958 = (v943 * v942).exp();
                    let v959 = v602 - v943;
                    let v2732 = v2236 * v942;
                    let v961 = v2 + (v959 * v942);
                    let v962 = v958 * v961;
                    let v2740 = (Lanes([(((v2712 * v943) * v958) * v961), 0.0, 0.0])) + (((Lanes([0.0, v2732[0], v2732[1]])) + (Lanes([(v2712 * v959), 0.0, 0.0]))) * v958);
                    v967 = v962;
                    v1820 = v2740;
                }
                let v2747 = v1819 * v963;
                let v966 = v2 - v963;
                let v2748 = v1820 * v966;
                let v970 = ((v963 * v964) + (v966 * v967)) - v2;
                let v971 = v389 * v970;
                let v2755 = (Lanes([(v1978 * v970), 0.0, 0.0, 0.0, 0.0])) + (((Lanes([v2747[0], 0.0, v2747[1], 0.0, v2747[2]])) + (Lanes([v2748[0], v2748[1], 0.0, v2748[2], 0.0]))) * v389);
                let v2757 = (v2755 * v26) * v107;
                let v974 = v2 + (v107 * (v971 * v26));
                let v975 = if v974 > v903 { 1.0 } else { 0.0 };
                let v1294: f64;
                let v1821: Lanes<5>;
                if v975 != 0.0 {
                    let v976 = v974.sqrt();
                    let v978 = v105 * (v2 + v976);
                    let v2761 = (v2757 * (v1773 / (v2096 * v976))) * v105;
                    v1294 = v978;
                    v1821 = v2761;
                } else {
                    v1294 = v979;
                    v1821 = v2709;
                }
                v1293 = v1294;
                v1717 = v971;
                v1817 = v1821;
                v1818 = v2755;
            } else {
                v1293 = v2;
                v1717 = v0;
                v1817 = v2709;
                v1818 = v2709;
            }
            let v981 = if v980 == v2 { 1.0 } else { 0.0 };
            let v1389: f64;
            let v1399: f64;
            let v1822: Lanes<4>;
            let v1823: Lanes<4>;
            if v981 != 0.0 {
                let v982 = v2 / v393;
                let v3007 = ((v1985 * v982) * v1866) / v393;
                let v984 = if v596 < v983 { 1.0 } else { 0.0 };
                let v1010: f64;
                let v1824: Lanes<3>;
                if v984 != 0.0 {
                    let v3019 = v2228 * v982;
                    let v986 = (v596 * v982).exp();
                    let v3024 = ((Lanes([0.0, v3019[0], v3019[1]])) + (Lanes([(v3007 * v596), 0.0, 0.0]))) * v986;
                    v1010 = v986;
                    v1824 = v3024;
                } else {
                    let v988 = (v983 * v982).exp();
                    let v989 = v596 - v983;
                    let v3010 = v2228 * v982;
                    let v991 = v2 + (v989 * v982);
                    let v992 = v988 * v991;
                    let v3018 = (Lanes([(((v3007 * v983) * v988) * v991), 0.0, 0.0])) + (((Lanes([0.0, v3010[0], v3010[1]])) + (Lanes([(v3007 * v989), 0.0, 0.0]))) * v988);
                    v1010 = v992;
                    v1824 = v3018;
                }
                let v993 = v2 / v400;
                let v3027 = ((v1999 * v993) * v1866) / v400;
                let v995 = if v596 < v994 { 1.0 } else { 0.0 };
                let v1013: f64;
                let v1825: Lanes<3>;
                if v995 != 0.0 {
                    let v3039 = v2228 * v993;
                    let v997 = (v596 * v993).exp();
                    let v3044 = ((Lanes([0.0, v3039[0], v3039[1]])) + (Lanes([(v3027 * v596), 0.0, 0.0]))) * v997;
                    v1013 = v997;
                    v1825 = v3044;
                } else {
                    let v999 = (v994 * v993).exp();
                    let v1000 = v596 - v994;
                    let v3030 = v2228 * v993;
                    let v1002 = v2 + (v1000 * v993);
                    let v1003 = v999 * v1002;
                    let v3038 = (Lanes([(((v3027 * v994) * v999) * v1002), 0.0, 0.0])) + (((Lanes([0.0, v3030[0], v3030[1]])) + (Lanes([(v3027 * v1000), 0.0, 0.0]))) * v999);
                    v1013 = v1003;
                    v1825 = v3038;
                }
                let v1005 = if v1004 > v0 { 1.0 } else { 0.0 };
                let v1035: f64;
                let v1826: Lanes<4>;
                if v1005 != 0.0 {
                    let v1008 = v2 + (v1004 * (v908 - v2));
                    let v1009 = v396 * v1008;
                    let v1011 = v1010 - v2;
                    let v3061 = v1824 * v1009;
                    let v1014 = v1013 - v2;
                    let v3067 = (Lanes([(v2006 * v1014), 0.0, 0.0])) + (v1825 * v403);
                    let v1016 = (v1009 * v1011) + (v403 * v1014);
                    let v3069 = ((((Lanes([(v1992 * v1008), 0.0, 0.0, 0.0])) + ((v2668 * v1004) * v396)) * v1011) + (Lanes([v3061[0], 0.0, v3061[1], v3061[2]]))) + (Lanes([v3067[0], 0.0, v3067[1], v3067[2]]));
                    v1035 = v1016;
                    v1826 = v3069;
                } else {
                    let v1017 = v1010 - v2;
                    let v1019 = v1013 - v2;
                    let v1021 = (v396 * v1017) + (v403 * v1019);
                    let v3053 = ((Lanes([(v1992 * v1017), 0.0, 0.0])) + (v1824 * v396)) + ((Lanes([(v2006 * v1019), 0.0, 0.0])) + (v1825 * v403));
                    let v3054 = Lanes([v3053[0], 0.0, v3053[1], v3053[2]]);
                    v1035 = v1021;
                    v1826 = v3054;
                }
                let v1022 = if v78 > v0 { 1.0 } else { 0.0 };
                let v1390: f64;
                let v1827: Lanes<4>;
                if v1022 != 0.0 {
                    let v1023 = v535 - v596;
                    let v3072 = (Lanes([v2179, 0.0, 0.0])) - (Lanes([0.0, v2228[0], v2228[1]]));
                    let v1024 = v2 / v536;
                    let v3075 = ((v2182 * v1024) * v1866) / v536;
                    let v1026 = if v1023 < v1025 { 1.0 } else { 0.0 };
                    let v1036: f64;
                    let v1828: Lanes<3>;
                    if v1026 != 0.0 {
                        let v1028 = (v1023 * v1024).exp();
                        let v3090 = ((v3072 * v1024) + (Lanes([(v3075 * v1023), 0.0, 0.0]))) * v1028;
                        v1036 = v1028;
                        v1828 = v3090;
                    } else {
                        let v1030 = (v1025 * v1024).exp();
                        let v1031 = v1023 - v1025;
                        let v1033 = v2 + (v1031 * v1024);
                        let v1034 = v1030 * v1033;
                        let v3085 = (Lanes([(((v3075 * v1025) * v1030) * v1033), 0.0, 0.0])) + (((v3072 * v1024) + (Lanes([(v3075 * v1031), 0.0, 0.0]))) * v1030);
                        v1036 = v1034;
                        v1828 = v3085;
                    }
                    let v3093 = (v1828 - (Lanes([v2186, 0.0, 0.0]))) * v74;
                    let v1039 = v1035 - (v74 * (v1036 - v538));
                    let v3095 = v1826 - (Lanes([v3093[0], 0.0, v3093[1], v3093[2]]));
                    v1390 = v1039;
                    v1827 = v3095;
                } else {
                    v1390 = v1035;
                    v1827 = v1826;
                }
                v1389 = v1390;
                v1399 = v0;
                v1822 = v1827;
                v1823 = v3096;
            } else {
                let v1040 = if v980 == v0 { 1.0 } else { 0.0 };
                let v1391: f64;
                let v1400: f64;
                let v1829: Lanes<4>;
                let v1830: Lanes<4>;
                if v1040 != 0.0 {
                    let v1041 = v2 / v393;
                    let v2929 = ((v1985 * v1041) * v1866) / v393;
                    let v1042 = if v599 < v983 { 1.0 } else { 0.0 };
                    let v1061: f64;
                    let v1831: Lanes<3>;
                    if v1042 != 0.0 {
                        let v2941 = v2232 * v1041;
                        let v1044 = (v599 * v1041).exp();
                        let v2946 = ((Lanes([0.0, v2941[0], v2941[1]])) + (Lanes([(v2929 * v599), 0.0, 0.0]))) * v1044;
                        v1061 = v1044;
                        v1831 = v2946;
                    } else {
                        let v1046 = (v983 * v1041).exp();
                        let v1047 = v599 - v983;
                        let v2932 = v2232 * v1041;
                        let v1049 = v2 + (v1047 * v1041);
                        let v1050 = v1046 * v1049;
                        let v2940 = (Lanes([(((v2929 * v983) * v1046) * v1049), 0.0, 0.0])) + (((Lanes([0.0, v2932[0], v2932[1]])) + (Lanes([(v2929 * v1047), 0.0, 0.0]))) * v1046);
                        v1061 = v1050;
                        v1831 = v2940;
                    }
                    let v1051 = v2 / v400;
                    let v2949 = ((v1999 * v1051) * v1866) / v400;
                    let v1052 = if v599 < v994 { 1.0 } else { 0.0 };
                    let v1064: f64;
                    let v1832: Lanes<3>;
                    if v1052 != 0.0 {
                        let v2961 = v2232 * v1051;
                        let v1054 = (v599 * v1051).exp();
                        let v2966 = ((Lanes([0.0, v2961[0], v2961[1]])) + (Lanes([(v2949 * v599), 0.0, 0.0]))) * v1054;
                        v1064 = v1054;
                        v1832 = v2966;
                    } else {
                        let v1056 = (v994 * v1051).exp();
                        let v1057 = v599 - v994;
                        let v2952 = v2232 * v1051;
                        let v1059 = v2 + (v1057 * v1051);
                        let v1060 = v1056 * v1059;
                        let v2960 = (Lanes([(((v2949 * v994) * v1056) * v1059), 0.0, 0.0])) + (((Lanes([0.0, v2952[0], v2952[1]])) + (Lanes([(v2949 * v1057), 0.0, 0.0]))) * v1056);
                        v1064 = v1060;
                        v1832 = v2960;
                    }
                    let v1062 = v1061 - v2;
                    let v1065 = v1064 - v2;
                    let v1067 = (v396 * v1062) + (v403 * v1065);
                    let v2975 = ((Lanes([(v1992 * v1062), 0.0, 0.0])) + (v1831 * v396)) + ((Lanes([(v2006 * v1065), 0.0, 0.0])) + (v1832 * v403));
                    let v1068 = if v78 > v0 { 1.0 } else { 0.0 };
                    let v1401: f64;
                    let v1833: Lanes<4>;
                    if v1068 != 0.0 {
                        let v1069 = v535 - v596;
                        let v2979 = (Lanes([v2179, 0.0, 0.0])) - (Lanes([0.0, v2228[0], v2228[1]]));
                        let v1070 = v2 / v536;
                        let v2982 = ((v2182 * v1070) * v1866) / v536;
                        let v1071 = if v1069 < v1025 { 1.0 } else { 0.0 };
                        let v1080: f64;
                        let v1834: Lanes<3>;
                        if v1071 != 0.0 {
                            let v1073 = (v1069 * v1070).exp();
                            let v2997 = ((v2979 * v1070) + (Lanes([(v2982 * v1069), 0.0, 0.0]))) * v1073;
                            v1080 = v1073;
                            v1834 = v2997;
                        } else {
                            let v1075 = (v1025 * v1070).exp();
                            let v1076 = v1069 - v1025;
                            let v1078 = v2 + (v1076 * v1070);
                            let v1079 = v1075 * v1078;
                            let v2992 = (Lanes([(((v2982 * v1025) * v1075) * v1078), 0.0, 0.0])) + (((v2979 * v1070) + (Lanes([(v2982 * v1076), 0.0, 0.0]))) * v1075);
                            v1080 = v1079;
                            v1834 = v2992;
                        }
                        let v3000 = (v1834 - (Lanes([v2186, 0.0, 0.0]))) * v74;
                        let v1083 = v1067 - (v74 * (v1080 - v538));
                        let v3003 = (Lanes([v2975[0], v2975[1], 0.0, v2975[2]])) - (Lanes([v3000[0], 0.0, v3000[1], v3000[2]]));
                        v1401 = v1083;
                        v1833 = v3003;
                    } else {
                        let v2976 = Lanes([v2975[0], v2975[1], 0.0, v2975[2]]);
                        v1401 = v1067;
                        v1833 = v2976;
                    }
                    v1391 = v0;
                    v1400 = v1401;
                    v1829 = v3004;
                    v1830 = v1833;
                } else {
                    let v1084 = v2 / v393;
                    let v2764 = ((v1985 * v1084) * v1866) / v393;
                    let v1085 = if v596 < v983 { 1.0 } else { 0.0 };
                    let v1109: f64;
                    let v1835: Lanes<3>;
                    if v1085 != 0.0 {
                        let v2776 = v2228 * v1084;
                        let v1087 = (v596 * v1084).exp();
                        let v2781 = ((Lanes([0.0, v2776[0], v2776[1]])) + (Lanes([(v2764 * v596), 0.0, 0.0]))) * v1087;
                        v1109 = v1087;
                        v1835 = v2781;
                    } else {
                        let v1089 = (v983 * v1084).exp();
                        let v1090 = v596 - v983;
                        let v2767 = v2228 * v1084;
                        let v1092 = v2 + (v1090 * v1084);
                        let v1093 = v1089 * v1092;
                        let v2775 = (Lanes([(((v2764 * v983) * v1089) * v1092), 0.0, 0.0])) + (((Lanes([0.0, v2767[0], v2767[1]])) + (Lanes([(v2764 * v1090), 0.0, 0.0]))) * v1089);
                        v1109 = v1093;
                        v1835 = v2775;
                    }
                    let v1094 = v2 / v400;
                    let v2784 = ((v1999 * v1094) * v1866) / v400;
                    let v1095 = if v596 < v994 { 1.0 } else { 0.0 };
                    let v1112: f64;
                    let v1836: Lanes<3>;
                    if v1095 != 0.0 {
                        let v2796 = v2228 * v1094;
                        let v1097 = (v596 * v1094).exp();
                        let v2801 = ((Lanes([0.0, v2796[0], v2796[1]])) + (Lanes([(v2784 * v596), 0.0, 0.0]))) * v1097;
                        v1112 = v1097;
                        v1836 = v2801;
                    } else {
                        let v1099 = (v994 * v1094).exp();
                        let v1100 = v596 - v994;
                        let v2787 = v2228 * v1094;
                        let v1102 = v2 + (v1100 * v1094);
                        let v1103 = v1099 * v1102;
                        let v2795 = (Lanes([(((v2784 * v994) * v1099) * v1102), 0.0, 0.0])) + (((Lanes([0.0, v2787[0], v2787[1]])) + (Lanes([(v2784 * v1100), 0.0, 0.0]))) * v1099);
                        v1112 = v1103;
                        v1836 = v2795;
                    }
                    let v1104 = if v1004 > v0 { 1.0 } else { 0.0 };
                    let v1135: f64;
                    let v1837: Lanes<4>;
                    if v1104 != 0.0 {
                        let v1107 = v2 + (v1004 * (v908 - v2));
                        let v1108 = v396 * v1107;
                        let v1110 = v1109 - v2;
                        let v2819 = v1835 * v1108;
                        let v1113 = v1112 - v2;
                        let v2825 = (Lanes([(v2006 * v1113), 0.0, 0.0])) + (v1836 * v403);
                        let v1116 = v980 * ((v1108 * v1110) + (v403 * v1113));
                        let v2828 = (((((Lanes([(v1992 * v1107), 0.0, 0.0, 0.0])) + ((v2668 * v1004) * v396)) * v1110) + (Lanes([v2819[0], 0.0, v2819[1], v2819[2]]))) + (Lanes([v2825[0], 0.0, v2825[1], v2825[2]]))) * v980;
                        v1135 = v1116;
                        v1837 = v2828;
                    } else {
                        let v1117 = v1109 - v2;
                        let v1119 = v1112 - v2;
                        let v1122 = v980 * ((v396 * v1117) + (v403 * v1119));
                        let v2811 = (((Lanes([(v1992 * v1117), 0.0, 0.0])) + (v1835 * v396)) + ((Lanes([(v2006 * v1119), 0.0, 0.0])) + (v1836 * v403))) * v980;
                        let v2812 = Lanes([v2811[0], 0.0, v2811[1], v2811[2]]);
                        v1135 = v1122;
                        v1837 = v2812;
                    }
                    let v1123 = if v78 > v0 { 1.0 } else { 0.0 };
                    let v1392: f64;
                    let v1838: Lanes<4>;
                    if v1123 != 0.0 {
                        let v1124 = v535 - v596;
                        let v2831 = (Lanes([v2179, 0.0, 0.0])) - (Lanes([0.0, v2228[0], v2228[1]]));
                        let v1125 = v2 / v536;
                        let v2834 = ((v2182 * v1125) * v1866) / v536;
                        let v1126 = if v1124 < v1025 { 1.0 } else { 0.0 };
                        let v1137: f64;
                        let v1839: Lanes<3>;
                        if v1126 != 0.0 {
                            let v1128 = (v1124 * v1125).exp();
                            let v2849 = ((v2831 * v1125) + (Lanes([(v2834 * v1124), 0.0, 0.0]))) * v1128;
                            v1137 = v1128;
                            v1839 = v2849;
                        } else {
                            let v1130 = (v1025 * v1125).exp();
                            let v1131 = v1124 - v1025;
                            let v1133 = v2 + (v1131 * v1125);
                            let v1134 = v1130 * v1133;
                            let v2844 = (Lanes([(((v2834 * v1025) * v1130) * v1133), 0.0, 0.0])) + (((v2831 * v1125) + (Lanes([(v2834 * v1131), 0.0, 0.0]))) * v1130);
                            v1137 = v1134;
                            v1839 = v2844;
                        }
                        let v1136 = v980 * v74;
                        let v2852 = (v1839 - (Lanes([v2186, 0.0, 0.0]))) * v1136;
                        let v1140 = v1135 - (v1136 * (v1137 - v538));
                        let v2854 = v1837 - (Lanes([v2852[0], 0.0, v2852[1], v2852[2]]));
                        v1392 = v1140;
                        v1838 = v2854;
                    } else {
                        v1392 = v1135;
                        v1838 = v1837;
                    }
                    let v1141 = if v599 < v983 { 1.0 } else { 0.0 };
                    let v1160: f64;
                    let v1840: Lanes<3>;
                    if v1141 != 0.0 {
                        let v2866 = v2232 * v1084;
                        let v1143 = (v599 * v1084).exp();
                        let v2871 = ((Lanes([0.0, v2866[0], v2866[1]])) + (Lanes([(v2764 * v599), 0.0, 0.0]))) * v1143;
                        v1160 = v1143;
                        v1840 = v2871;
                    } else {
                        let v1145 = (v983 * v1084).exp();
                        let v1146 = v599 - v983;
                        let v2857 = v2232 * v1084;
                        let v1148 = v2 + (v1146 * v1084);
                        let v1149 = v1145 * v1148;
                        let v2865 = (Lanes([(((v2764 * v983) * v1145) * v1148), 0.0, 0.0])) + (((Lanes([0.0, v2857[0], v2857[1]])) + (Lanes([(v2764 * v1146), 0.0, 0.0]))) * v1145);
                        v1160 = v1149;
                        v1840 = v2865;
                    }
                    let v1150 = if v599 < v994 { 1.0 } else { 0.0 };
                    let v1163: f64;
                    let v1841: Lanes<3>;
                    if v1150 != 0.0 {
                        let v2883 = v2232 * v1094;
                        let v1152 = (v599 * v1094).exp();
                        let v2888 = ((Lanes([0.0, v2883[0], v2883[1]])) + (Lanes([(v2784 * v599), 0.0, 0.0]))) * v1152;
                        v1163 = v1152;
                        v1841 = v2888;
                    } else {
                        let v1154 = (v994 * v1094).exp();
                        let v1155 = v599 - v994;
                        let v2874 = v2232 * v1094;
                        let v1157 = v2 + (v1155 * v1094);
                        let v1158 = v1154 * v1157;
                        let v2882 = (Lanes([(((v2784 * v994) * v1154) * v1157), 0.0, 0.0])) + (((Lanes([0.0, v2874[0], v2874[1]])) + (Lanes([(v2784 * v1155), 0.0, 0.0]))) * v1154);
                        v1163 = v1158;
                        v1841 = v2882;
                    }
                    let v1159 = v2 - v980;
                    let v1161 = v1160 - v2;
                    let v1164 = v1163 - v2;
                    let v1167 = v1159 * ((v396 * v1161) + (v403 * v1164));
                    let v2898 = (((Lanes([(v1992 * v1161), 0.0, 0.0])) + (v1840 * v396)) + ((Lanes([(v2006 * v1164), 0.0, 0.0])) + (v1841 * v403))) * v1159;
                    let v1402: f64;
                    let v1842: Lanes<4>;
                    if v1123 != 0.0 {
                        let v1168 = v535 - v596;
                        let v2902 = (Lanes([v2179, 0.0, 0.0])) - (Lanes([0.0, v2228[0], v2228[1]]));
                        let v1169 = v2 / v536;
                        let v2905 = ((v2182 * v1169) * v1866) / v536;
                        let v1170 = if v1168 < v1025 { 1.0 } else { 0.0 };
                        let v1180: f64;
                        let v1843: Lanes<3>;
                        if v1170 != 0.0 {
                            let v1172 = (v1168 * v1169).exp();
                            let v2920 = ((v2902 * v1169) + (Lanes([(v2905 * v1168), 0.0, 0.0]))) * v1172;
                            v1180 = v1172;
                            v1843 = v2920;
                        } else {
                            let v1174 = (v1025 * v1169).exp();
                            let v1175 = v1168 - v1025;
                            let v1177 = v2 + (v1175 * v1169);
                            let v1178 = v1174 * v1177;
                            let v2915 = (Lanes([(((v2905 * v1025) * v1174) * v1177), 0.0, 0.0])) + (((v2902 * v1169) + (Lanes([(v2905 * v1175), 0.0, 0.0]))) * v1174);
                            v1180 = v1178;
                            v1843 = v2915;
                        }
                        let v1179 = v1159 * v74;
                        let v2923 = (v1843 - (Lanes([v2186, 0.0, 0.0]))) * v1179;
                        let v1183 = v1167 - (v1179 * (v1180 - v538));
                        let v2926 = (Lanes([v2898[0], v2898[1], 0.0, v2898[2]])) - (Lanes([v2923[0], 0.0, v2923[1], v2923[2]]));
                        v1402 = v1183;
                        v1842 = v2926;
                    } else {
                        let v2899 = Lanes([v2898[0], v2898[1], 0.0, v2898[2]]);
                        v1402 = v1167;
                        v1842 = v2899;
                    }
                    v1391 = v1392;
                    v1400 = v1402;
                    v1829 = v1838;
                    v1830 = v1842;
                }
                v1389 = v1391;
                v1399 = v1400;
                v1822 = v1829;
                v1823 = v1830;
            }
            let v1184 = v2 / v407;
            let v3099 = ((v2013 * v1184) * v1866) / v407;
            let v1186 = if v602 < v1185 { 1.0 } else { 0.0 };
            let v1206: f64;
            let v1844: Lanes<3>;
            if v1186 != 0.0 {
                let v3111 = v2236 * v1184;
                let v1188 = (v602 * v1184).exp();
                let v3116 = ((Lanes([0.0, v3111[0], v3111[1]])) + (Lanes([(v3099 * v602), 0.0, 0.0]))) * v1188;
                v1206 = v1188;
                v1844 = v3116;
            } else {
                let v1190 = (v1185 * v1184).exp();
                let v1191 = v602 - v1185;
                let v3102 = v2236 * v1184;
                let v1193 = v2 + (v1191 * v1184);
                let v1194 = v1190 * v1193;
                let v3110 = (Lanes([(((v3099 * v1185) * v1190) * v1193), 0.0, 0.0])) + (((Lanes([0.0, v3102[0], v3102[1]])) + (Lanes([(v3099 * v1191), 0.0, 0.0]))) * v1190);
                v1206 = v1194;
                v1844 = v3110;
            }
            let v1195 = v2 / v414;
            let v3119 = ((v2027 * v1195) * v1866) / v414;
            let v1197 = if v602 < v1196 { 1.0 } else { 0.0 };
            let v1209: f64;
            let v1845: Lanes<3>;
            if v1197 != 0.0 {
                let v3131 = v2236 * v1195;
                let v1199 = (v602 * v1195).exp();
                let v3136 = ((Lanes([0.0, v3131[0], v3131[1]])) + (Lanes([(v3119 * v602), 0.0, 0.0]))) * v1199;
                v1209 = v1199;
                v1845 = v3136;
            } else {
                let v1201 = (v1196 * v1195).exp();
                let v1202 = v602 - v1196;
                let v3122 = v2236 * v1195;
                let v1204 = v2 + (v1202 * v1195);
                let v1205 = v1201 * v1204;
                let v3130 = (Lanes([(((v3119 * v1196) * v1201) * v1204), 0.0, 0.0])) + (((Lanes([0.0, v3122[0], v3122[1]])) + (Lanes([(v3119 * v1202), 0.0, 0.0]))) * v1201);
                v1209 = v1205;
                v1845 = v3130;
            }
            let v1207 = v1206 - v2;
            let v1210 = v1209 - v2;
            let v1212 = (v410 * v1207) + (v417 * v1210);
            let v3145 = ((Lanes([(v2020 * v1207), 0.0, 0.0])) + (v1844 * v410)) + ((Lanes([(v2034 * v1210), 0.0, 0.0])) + (v1845 * v417));
            let v1215 = if (if v252 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v260 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1405: f64;
            let v1846: Lanes<3>;
            if v1215 != 0.0 {
                let v1217 = if v610 < v1216 { 1.0 } else { 0.0 };
                let v1236: f64;
                let v1847: Lanes<3>;
                if v1217 != 0.0 {
                    let v3158 = v2248 * v1184;
                    let v1219 = (v610 * v1184).exp();
                    let v3163 = ((Lanes([0.0, v3158[0], v3158[1]])) + (Lanes([(v3099 * v610), 0.0, 0.0]))) * v1219;
                    v1236 = v1219;
                    v1847 = v3163;
                } else {
                    let v1221 = (v1216 * v1184).exp();
                    let v1222 = v610 - v1216;
                    let v3149 = v2248 * v1184;
                    let v1224 = v2 + (v1222 * v1184);
                    let v1225 = v1221 * v1224;
                    let v3157 = (Lanes([(((v3099 * v1216) * v1221) * v1224), 0.0, 0.0])) + (((Lanes([0.0, v3149[0], v3149[1]])) + (Lanes([(v3099 * v1222), 0.0, 0.0]))) * v1221);
                    v1236 = v1225;
                    v1847 = v3157;
                }
                let v1227 = if v610 < v1226 { 1.0 } else { 0.0 };
                let v1239: f64;
                let v1848: Lanes<3>;
                if v1227 != 0.0 {
                    let v3175 = v2248 * v1195;
                    let v1229 = (v610 * v1195).exp();
                    let v3180 = ((Lanes([0.0, v3175[0], v3175[1]])) + (Lanes([(v3119 * v610), 0.0, 0.0]))) * v1229;
                    v1239 = v1229;
                    v1848 = v3180;
                } else {
                    let v1231 = (v1226 * v1195).exp();
                    let v1232 = v610 - v1226;
                    let v3166 = v2248 * v1195;
                    let v1234 = v2 + (v1232 * v1195);
                    let v1235 = v1231 * v1234;
                    let v3174 = (Lanes([(((v3119 * v1226) * v1231) * v1234), 0.0, 0.0])) + (((Lanes([0.0, v3166[0], v3166[1]])) + (Lanes([(v3119 * v1232), 0.0, 0.0]))) * v1231);
                    v1239 = v1235;
                    v1848 = v3174;
                }
                let v1237 = v1236 - v2;
                let v1240 = v1239 - v2;
                let v1242 = (v419 * v1237) + (v421 * v1240);
                let v3189 = ((Lanes([(v2038 * v1237), 0.0, 0.0])) + (v1847 * v419)) + ((Lanes([(v2042 * v1240), 0.0, 0.0])) + (v1848 * v421));
                v1405 = v1242;
                v1846 = v3189;
            } else {
                v1405 = v0;
                v1846 = v3146;
            }
            let v1243 = v602 / v312;
            let v3191 = Lanes([0.0, v2236[0], v2236[1]]);
            let v3194 = (v3191 - (Lanes([(v1872 * v1243), 0.0, 0.0]))) / v312;
            let v1244 = if v1243 < v18 { 1.0 } else { 0.0 };
            let v1257: f64;
            let v1849: Lanes<3>;
            if v1244 != 0.0 {
                let v1245 = v1243.exp();
                let v3196 = v3194 * v1245;
                v1257 = v1245;
                v1849 = v3196;
            } else {
                let v1246 = v18.exp();
                let v1249 = v1246 * (v2 + (v1243 - v18));
                let v3195 = v3194 * v1246;
                v1257 = v1249;
                v1849 = v3195;
            }
            let v1250 = v605 / v312;
            let v3201 = ((Lanes([0.0, v2240[0], v2240[1]])) - (Lanes([(v1872 * v1250), 0.0, 0.0]))) / v312;
            let v1251 = if v1250 < v18 { 1.0 } else { 0.0 };
            let v1261: f64;
            let v1850: Lanes<3>;
            if v1251 != 0.0 {
                let v1252 = v1250.exp();
                let v3203 = v3201 * v1252;
                v1261 = v1252;
                v1850 = v3203;
            } else {
                let v1253 = v18.exp();
                let v1256 = v1253 * (v2 + (v1250 - v18));
                let v3202 = v3201 * v1253;
                v1261 = v1256;
                v1850 = v3202;
            }
            let v1260 = (v2 + (v530 * v1257)).sqrt();
            let v3210 = ((Lanes([(v2173 * v1257), 0.0, 0.0])) + (v1849 * v530)) * (v1773 / (v2096 * v1260));
            let v1264 = (v2 + (v530 * v1261)).sqrt();
            let v3217 = ((Lanes([(v2173 * v1261), 0.0, 0.0])) + (v1850 * v530)) * (v1773 / (v2096 * v1264));
            let v1265 = v618 * v554;
            let v3218 = v2261 * v554;
            let v3222 = (Lanes([v3218[0], 0.0, v3218[1]])) + (Lanes([0.0, (v1793 * v618), 0.0]));
            let v1267 = v1264 + v2;
            let v1268 = (v1260 + v2) / v1267;
            let v3223 = v3217 * v1268;
            let v3224 = Lanes([v3210[0], 0.0, v3210[1], v3210[2]]);
            let v1271 = (v1260 - v1264) - (v1268.ln());
            let v1273 = v620 + (v312 * v1271);
            let v1274 = v1273 * v558;
            let v3242 = (((Lanes([0.0, v2265[0], v2265[1], 0.0])) + ((Lanes([(v1872 * v1271), 0.0, 0.0, 0.0])) + (((v3224 - (Lanes([v3217[0], v3217[1], 0.0, v3217[2]]))) - (((v3224 - (Lanes([v3223[0], v3223[1], 0.0, v3223[2]]))) / v1267) * (v1773 / v1268))) * v312))) * v558) + (Lanes([(v1794 * v1273), 0.0, 0.0, 0.0]));
            let v1277 = (v105 * v589) * v30;
            let v3249 = v2265 * v620;
            let v1280 = ((v620 * v620) + v4).sqrt();
            let v3255 = ((v3249 + v3249) * (v1773 / (v2096 * v1280))) * v1277;
            let v1282 = v2 + (v1277 * v1280);
            let v1283 = v558 * v1282;
            let v1284 = (v589 * v1274) / v1283;
            let v3263 = ((Lanes([(v1794 * v1282), 0.0, 0.0])) + (((Lanes([(((v1803 * v105) * v30) * v1280), 0.0, 0.0])) + (Lanes([0.0, v3255[0], v3255[1]]))) * v558)) * v1284;
            let v3267 = ((((Lanes([(v1803 * v1274), 0.0, 0.0, 0.0])) + (v3242 * v589)) - (Lanes([v3263[0], v3263[1], v3263[2], 0.0]))) / v1283) * v1284;
            let v1287 = (v2 + (v1284 * v1284)).sqrt();
            let v1288 = v1274 / v1287;
            let v3274 = (v3242 - (((v3267 + v3267) * (v1773 / (v2096 * v1287))) * v1288)) / v1287;
            let v1289 = v621 * v562;
            let v3275 = v2268 * v562;
            let v3279 = (Lanes([v3275[0], 0.0, v3275[1]])) + (Lanes([0.0, (v1795 * v621), 0.0]));
            let v1290 = v622 * v936;
            let v3280 = v2271 * v936;
            let v3281 = v1814 * v622;
            let v1291 = v1290 * v566;
            let v3288 = (((Lanes([0.0, 0.0, v3280[0], v3280[1], 0.0])) + (Lanes([v3281[0], v3281[1], 0.0, v3281[2], v3281[3]]))) * v566) + (Lanes([(v1796 * v1290), 0.0, 0.0, 0.0, 0.0]));
            let v1292 = v623 * v569;
            let v3289 = v2274 * v569;
            let v3293 = (Lanes([v3289[0], 0.0, v3289[1]])) + (Lanes([0.0, (v1797 * v623), 0.0]));
            let v1295 = v624 * v1293;
            let v3294 = v2277 * v1293;
            let v3295 = v1817 * v624;
            let v1296 = v1295 * v573;
            let v3302 = (((Lanes([0.0, v3294[0], 0.0, 0.0, 0.0, v3294[1]])) + (Lanes([v3295[0], 0.0, v3295[1], v3295[2], v3295[3], v3295[4]]))) * v573) + (Lanes([(v1798 * v1295), 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v1298 = if v1297 > v0 { 1.0 } else { 0.0 };
            let v1385: f64;
            let v1851: Lanes<5>;
            if v1298 != 0.0 {
                let v1301 = v1299 * (v431 + v2);
                let v1304 = v2 / (v1302 - v520);
                let v1305 = v1301.powf(v1304);
                let v1307 = (v512 - v602) - v1305;
                let v3311 = Lanes([((v2047 * v1299) * (v1304 * (v1301.powf((v1304 - v1773))))), 0.0, 0.0]);
                let v3312 = ((Lanes([v2144, 0.0, 0.0])) - v3191) - v3311;
                let v3313 = v3312 * v1307;
                let v1310 = ((v1307 * v1307) + v4).sqrt();
                let v1313 = (v105 * (v1310 + v1307)) + v1305;
                let v3320 = ((((v3313 + v3313) * (v1773 / (v2096 * v1310))) + v3312) * v105) + v3311;
                let v1314 = -v431;
                let v1315 = v520 - v2;
                let v1316 = v1313.powf(v1315);
                let v1317 = v1314 * v1316;
                let v3329 = (Lanes([((v2047 * v1866) * v1316), 0.0, 0.0])) + ((v3320 * (v1315 * (v1313.powf((v1315 - v1773))))) * v1314);
                let v1318 = if v1317 < v18 { 1.0 } else { 0.0 };
                let v1325: f64;
                let v1852: Lanes<3>;
                if v1318 != 0.0 {
                    let v1319 = v1317.exp();
                    let v3331 = v3329 * v1319;
                    v1325 = v1319;
                    v1852 = v3331;
                } else {
                    let v1320 = v18.exp();
                    let v1323 = v1320 * (v2 + (v1317 - v18));
                    let v3330 = v3329 * v1320;
                    v1325 = v1323;
                    v1852 = v3330;
                }
                let v1324 = v1297 * v1313;
                let v1326 = v1324 * v1325;
                let v1328 = (v626 - v939) - v1212;
                let v1329 = v1328 * v1326;
                let v3342 = (((v3320 * v1297) * v1325) + (v1852 * v1324)) * v1328;
                let v3344 = ((((Lanes([0.0, 0.0, 0.0, 0.0, v1785])) - (Lanes([v2704[0], v2704[1], v2704[2], v2704[3], 0.0]))) - (Lanes([v3145[0], v3145[1], v3145[2], 0.0, 0.0]))) * v1326) + (Lanes([v3342[0], v3342[1], v3342[2], 0.0, 0.0]));
                v1385 = v1329;
                v1851 = v3344;
            } else {
                v1385 = v0;
                v1851 = v3303;
            }
            let v1331 = if v1330 > v0 { 1.0 } else { 0.0 };
            let v1435: f64;
            let v1853: Lanes<4>;
            if v1331 != 0.0 {
                let v1333 = v1299 * (v436 + v2);
                let v1336 = v2 / (v1302 - v1334);
                let v1337 = v1333.powf(v1336);
                let v3351 = v2244 * v1866;
                let v1339 = (v0 - v607) - v1337;
                let v3353 = Lanes([((v2049 * v1299) * (v1336 * (v1333.powf((v1336 - v1773))))), 0.0, 0.0]);
                let v3354 = (Lanes([0.0, v3351[0], v3351[1]])) - v3353;
                let v3355 = v3354 * v1339;
                let v1342 = ((v1339 * v1339) + v4).sqrt();
                let v1345 = (v105 * (v1342 + v1339)) + v1337;
                let v3362 = ((((v3355 + v3355) * (v1773 / (v2096 * v1342))) + v3354) * v105) + v3353;
                let v1346 = -v436;
                let v1347 = v1334 - v2;
                let v1348 = v1345.powf(v1347);
                let v1349 = v1346 * v1348;
                let v3371 = (Lanes([((v2049 * v1866) * v1348), 0.0, 0.0])) + ((v3362 * (v1347 * (v1345.powf((v1347 - v1773))))) * v1346);
                let v1350 = if v1349 < v18 { 1.0 } else { 0.0 };
                let v1357: f64;
                let v1854: Lanes<3>;
                if v1350 != 0.0 {
                    let v1351 = v1349.exp();
                    let v3373 = v3371 * v1351;
                    v1357 = v1351;
                    v1854 = v3373;
                } else {
                    let v1352 = v18.exp();
                    let v1355 = v1352 * (v2 + (v1349 - v18));
                    let v3372 = v3371 * v1352;
                    v1357 = v1355;
                    v1854 = v3372;
                }
                let v1356 = v1330 * v1345;
                let v1358 = v1356 * v1357;
                let v1359 = -v1265;
                let v1360 = v1359 * v1358;
                let v3379 = (v3222 * v1866) * v1358;
                let v3380 = (((v3362 * v1330) * v1357) + (v1854 * v1356)) * v1359;
                let v3383 = (Lanes([v3379[0], v3379[1], v3379[2], 0.0])) + (Lanes([0.0, v3380[0], v3380[1], v3380[2]]));
                v1435 = v1360;
                v1853 = v3383;
            } else {
                v1435 = v0;
                v1853 = v3345;
            }
            let v1365 = if (if v1361 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1363 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1387: f64;
            let v1855: Lanes<4>;
            if v1365 != 0.0 {
                let v1367 = if v1366 > v0 { 1.0 } else { 0.0 };
                let v1379: f64;
                let v1856: Lanes<2>;
                if v1367 != 0.0 {
                    let v3386 = (v2236 / v1366) * v1866;
                    let v1371 = (v2 - (v602 / v1366)) - v1370;
                    let v3387 = v3386 * v1371;
                    let v1374 = ((v1371 * v1371) + v900).sqrt();
                    let v1378 = v1363 * (v1370 + (v105 * (v1371 + v1374)));
                    let v3394 = ((v3386 + ((v3387 + v3387) * (v1773 / (v2096 * v1374)))) * v105) * v1363;
                    v1379 = v1378;
                    v1856 = v3394;
                } else {
                    v1379 = v1363;
                    v1856 = v3384;
                }
                let v1380 = v940 / v1379;
                let v3395 = v1856 * v1380;
                let v1381 = v1380 - v2;
                let v1384 = v1361 * (v1381.powf(v1382));
                let v3403 = (((v2708 - (Lanes([0.0, v3395[0], v3395[1], 0.0]))) / v1379) * (v1382 * (v1381.powf((v1382 - v1773))))) * v1361;
                v1387 = v1384;
                v1855 = v3403;
            } else {
                v1387 = v0;
                v1855 = v3004;
            }
            let v1388 = (v1212 - v1385) - v1387;
            let v3407 = ((Lanes([v3145[0], v3145[1], v3145[2], 0.0, 0.0])) - v1851) - (Lanes([v1855[0], v1855[1], v1855[2], v1855[3], 0.0]));
            let v3409 = v2228 * v1389;
            let v3411 = (v1822 * v596) + (Lanes([0.0, 0.0, v3409[0], v3409[1]]));
            let v3413 = v2236 * v1388;
            let v1396 = v626 - v939;
            let v3418 = Lanes([0.0, 0.0, 0.0, 0.0, v1785]);
            let v3422 = v2255 * v1396;
            let v3425 = ((Lanes([v3411[0], v3411[1], v3411[2], v3411[3], 0.0])) + ((v3407 * v602) + (Lanes([0.0, v3413[0], v3413[1], 0.0, 0.0])))) + (((v3418 - (Lanes([v2704[0], v2704[1], v2704[2], v2704[3], 0.0]))) * v615) + (Lanes([0.0, v3422[0], 0.0, v3422[1], 0.0])));
            let v3427 = v2232 * v1399;
            let v3429 = (v1823 * v599) + (Lanes([0.0, v3427[0], 0.0, v3427[1]]));
            let v3432 = (Lanes([v3425[0], v3425[1], 0.0, v3425[2], v3425[3], v3425[4]])) + (Lanes([v3429[0], 0.0, v3429[1], v3429[2], v3429[3], 0.0]));
            let v3434 = v2248 * v1405;
            let v3436 = (v1846 * v610) + (Lanes([0.0, v3434[0], v3434[1]]));
            let v3439 = (Lanes([v3432[0], v3432[1], v3432[2], v3432[3], v3432[4], 0.0, v3432[5]])) + (Lanes([v3436[0], 0.0, v3436[1], 0.0, 0.0, v3436[2], 0.0]));
            let v3441 = v2261 * v1265;
            let v3443 = (v3222 * v618) + (Lanes([v3441[0], 0.0, v3441[1]]));
            let v3448 = v2265 * v1288;
            let v3450 = (v3274 * v620) + (Lanes([0.0, v3448[0], v3448[1], 0.0]));
            let v3452 = ((Lanes([0.0, v3439[0], 0.0, v3439[1], v3439[2], v3439[3], v3439[4], v3439[5], v3439[6]])) + (Lanes([v3443[0], v3443[1], v3443[2], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, v3450[0], v3450[1], v3450[2], 0.0, v3450[3], 0.0, 0.0, 0.0]));
            let v3454 = v2268 * v1289;
            let v3456 = (v3279 * v621) + (Lanes([v3454[0], 0.0, v3454[1]]));
            let v3461 = v2271 * v1291;
            let v3463 = (v3288 * v622) + (Lanes([0.0, 0.0, v3461[0], v3461[1], 0.0]));
            let v3465 = ((Lanes([v3452[0], 0.0, v3452[1], v3452[2], v3452[3], v3452[4], v3452[5], v3452[6], v3452[7], v3452[8]])) + (Lanes([0.0, v3456[0], v3456[1], 0.0, 0.0, v3456[2], 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, v3463[0], 0.0, v3463[1], v3463[2], v3463[3], v3463[4], 0.0, 0.0]));
            let v3467 = v2274 * v1292;
            let v3469 = (v3293 * v623) + (Lanes([v3467[0], 0.0, v3467[1]]));
            let v3474 = v2277 * v1296;
            let v3476 = (v3302 * v624) + (Lanes([0.0, v3474[0], 0.0, 0.0, 0.0, v3474[1]]));
            let v1421 = -v1420;
            let v1422 = v1421 * (((((((((((v1389 * v596) + (v1388 * v602)) + (v1396 * v615)) + (v1399 * v599)) + (v1405 * v610)) + (v1265 * v618)) + (v1288 * v620)) + (v1289 * v621)) + (v1291 * v622)) + (v1292 * v623)) + (v1296 * v624));
            let v3479 = (((Lanes([v3465[0], v3465[1], 0.0, v3465[2], v3465[3], v3465[4], v3465[5], v3465[6], v3465[7], v3465[8], v3465[9]])) + (Lanes([0.0, 0.0, v3469[0], v3469[1], 0.0, 0.0, 0.0, 0.0, v3469[2], 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v3476[0], v3476[1], v3476[2], v3476[3], v3476[4], 0.0, v3476[5], 0.0]))) * v1421;
            let v1423 = v294 * v577;
            let v3482 = (v1774 * v577) + (v1799 * v294);
            let v1424 = v626 - v940;
            let v3484 = v3418 - (Lanes([v2708[0], v2708[1], v2708[2], v2708[3], 0.0]));
            let v1425 = v626 - v625;
            let v3487 = (Lanes([0.0, v1785])) - (Lanes([v1784, 0.0]));
            let v3488 = v2228 * v1426;
            let v3491 = v2232 * v1426;
            let v3494 = v2248 * v1426;
            let v3497 = v2236 * v1426;
            let v3500 = v2244 * v1426;
            let v1438 = v590 * (v1389 + (v1426 * v596));
            let v3503 = (v1822 + (Lanes([0.0, 0.0, v3488[0], v3488[1]]))) * v590;
            let v1439 = v590 * (v1399 + (v1426 * v599));
            let v3504 = (v1823 + (Lanes([0.0, v3491[0], 0.0, v3491[1]]))) * v590;
            let v1440 = v590 * v626;
            let v3505 = v1785 * v590;
            let v1441 = v590 * v939;
            let v3506 = v2704 * v590;
            let v1442 = v590 * (v1388 + (v1426 * v602));
            let v3507 = (v3407 + (Lanes([0.0, v3497[0], v3497[1], 0.0, 0.0]))) * v590;
            let v1443 = v590 * (v1435 + (v1426 * v607));
            let v3508 = (v1853 + (Lanes([0.0, 0.0, v3500[0], v3500[1]]))) * v590;
            let v1444 = v590 * (v1405 + (v1426 * v610));
            let v3509 = (v1846 + (Lanes([0.0, v3494[0], v3494[1]]))) * v590;
            let v1445 = v590 * v1288;
            let v3510 = v3274 * v590;
            let v1702: f64;
            let v1857: Lanes<3>;
            if v631 != 0.0 {
                let v1446 = v599 + v629;
                let v3570 = Lanes([0.0, v2232[0], v2232[1]]);
                let v3572 = v3570 + (Lanes([v2279, 0.0, 0.0]));
                let v1447 = if v1446 > v0 { 1.0 } else { 0.0 };
                let v1470: f64;
                let v1471: f64;
                let v1858: Lanes<3>;
                let v1859: Lanes<3>;
                if v1447 != 0.0 {
                    let v1448 = v2 - v628;
                    let v1450 = v1448.powf((-v515));
                    let v1452 = v2 - (v1450 * v1448);
                    let v1454 = v2 - v515;
                    let v1455 = (v484 * v1452) / v1454;
                    let v1456 = v105 * v515;
                    let v1458 = v484 * v1448;
                    let v1459 = (v1456 * v1446) / v1458;
                    let v1460 = v2 + v1459;
                    let v1462 = (v1446 * v1460) * v1450;
                    let v3600 = ((v3572 * v1460) + ((((v3572 * v1456) - (Lanes([((v2106 * v1448) * v1459), 0.0, 0.0]))) / v1458) * v1446)) * v1450;
                    let v3601 = Lanes([((v2106 * v1452) / v1454), 0.0, 0.0]);
                    v1470 = v1455;
                    v1471 = v1462;
                    v1858 = v3601;
                    v1859 = v3600;
                } else {
                    let v1463 = v599 / v484;
                    let v1464 = v2 - v1463;
                    let v1465 = v2 - v515;
                    let v1467 = v2 - (v1464.powf(v1465));
                    let v1469 = (v484 * v1467) / v1465;
                    let v3587 = ((Lanes([(v2106 * v1467), 0.0, 0.0])) + ((((((v3570 - (Lanes([(v2106 * v1463), 0.0, 0.0]))) / v484) * v1866) * (v1465 * (v1464.powf((v1465 - v1773))))) * v1866) * v484)) / v1465;
                    v1470 = v1469;
                    v1471 = v0;
                    v1858 = v3587;
                    v1859 = v3588;
                }
                let v1472 = v1470 + v1471;
                let v3602 = v1858 + v1859;
                v1702 = v1472;
                v1857 = v3602;
            } else {
                let v3511 = v2279 * v629;
                let v1475 = (v107 * v630) * v630;
                let v1477 = ((v629 * v629) + v1475).sqrt();
                let v1480 = v1478 * (v629 + v1477);
                let v3517 = (v2279 + ((v3511 + v3511) * (v1773 / (v2096 * v1477)))) * v1478;
                let v1481 = v1480 / v484;
                let v1482 = v2 - v1481;
                let v1483 = v2 - v515;
                let v1484 = v1482.powf(v1483);
                let v3522 = v1483 - v1773;
                let v1487 = v599 + v629;
                let v3530 = Lanes([0.0, v2232[0], v2232[1]]);
                let v3531 = Lanes([v2279, 0.0, 0.0]);
                let v3532 = v3530 + v3531;
                let v3533 = v3532 * v1487;
                let v1490 = ((v1487 * v1487) + v1475).sqrt();
                let v1493 = (v105 * (v1487 - v1490)) - v629;
                let v3540 = ((v3532 - ((v3533 + v3533) * (v1773 / (v2096 * v1490)))) * v105) - v3531;
                let v1494 = v1493 / v484;
                let v1495 = v2 - v1494;
                let v1496 = v1495.powf(v1483);
                let v1499 = v2 - v628;
                let v1501 = v1499.powf((-v515));
                let v1503 = (v599 - v1493) + v1480;
                let v3556 = (v3530 - v3540) + (Lanes([v3517, 0.0, 0.0]));
                let v1504 = v1501 * v1503;
                let v1505 = v105 * v515;
                let v1507 = v484 * v1499;
                let v1508 = (v1505 * v1503) / v1507;
                let v1509 = v2 + v1508;
                let v1512 = (((v627 * v1496) / v1483) + (v1504 * v1509)) - ((v627 * v1484) / v1483);
                let v3569 = ((((Lanes([(v2278 * v1496), 0.0, 0.0])) + (((((v3540 - (Lanes([(v2106 * v1494), 0.0, 0.0]))) / v484) * v1866) * (v1483 * (v1495.powf(v3522)))) * v627)) / v1483) + (((v3556 * v1501) * v1509) + ((((v3556 * v1505) - (Lanes([((v2106 * v1499) * v1508), 0.0, 0.0]))) / v1507) * v1504))) - (Lanes([(((v2278 * v1484) + (((((v3517 - (v2106 * v1481)) / v484) * v1866) * (v1483 * (v1482.powf(v3522)))) * v627)) / v1483), 0.0, 0.0]));
                v1702 = v1512;
                v1857 = v3569;
            }
            let v1714: f64;
            let v1860: Lanes<3>;
            if v702 != 0.0 {
                let v1513 = v610 + v700;
                let v3767 = Lanes([0.0, v2248[0], v2248[1]]);
                let v3769 = v3767 + (Lanes([v2373, 0.0, 0.0]));
                let v1514 = if v1513 > v0 { 1.0 } else { 0.0 };
                let v1555: f64;
                let v1557: f64;
                let v1861: Lanes<3>;
                let v1862: Lanes<3>;
                if v1514 != 0.0 {
                    let v1515 = v2 - v628;
                    let v1518 = v1515.powf((v1516 - v520));
                    let v1521 = v2 - ((v1518 * v1515) * v1515);
                    let v1523 = v2 - v520;
                    let v1524 = (v512 * v1521) / v1523;
                    let v1525 = v105 * v520;
                    let v1527 = (v1525 * v1513) / v512;
                    let v1528 = v1515 + v1527;
                    let v1530 = (v1513 * v1528) * v1518;
                    let v3819 = ((v3769 * v1528) + ((((v3769 * v1525) - (Lanes([(v2144 * v1527), 0.0, 0.0]))) / v512) * v1513)) * v1518;
                    let v3820 = Lanes([((v2144 * v1521) / v1523), 0.0, 0.0]);
                    v1555 = v1524;
                    v1557 = v1530;
                    v1861 = v3820;
                    v1862 = v3819;
                } else {
                    let v1534 = if (if v721 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v610 < (-v721) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v1556: f64;
                    let v1863: Lanes<3>;
                    if v1534 != 0.0 {
                        let v1535 = v721 / v512;
                        let v1536 = v2 + v1535;
                        let v1537 = v2 - v520;
                        let v1538 = v1536.powf(v1537);
                        let v3792 = v2248 * v1537;
                        let v1541 = v512 + v721;
                        let v1542 = (v1537 * (v610 + v721)) / v1541;
                        let v1543 = v2 - v1542;
                        let v1545 = v2 - (v1538 * v1543);
                        let v1547 = (v512 * v1545) / v1537;
                        let v3808 = ((Lanes([(v2144 * v1545), 0.0, 0.0])) + ((((Lanes([(((((v2144 * v1535) * v1866) / v512) * (v1537 * (v1536.powf((v1537 - v1773))))) * v1543), 0.0, 0.0])) + (((((Lanes([0.0, v3792[0], v3792[1]])) - (Lanes([(v2144 * v1542), 0.0, 0.0]))) / v1541) * v1866) * v1538)) * v1866) * v512)) / v1537;
                        v1556 = v1547;
                        v1863 = v3808;
                    } else {
                        let v1548 = v610 / v512;
                        let v1549 = v2 - v1548;
                        let v1550 = v2 - v520;
                        let v1552 = v2 - (v1549.powf(v1550));
                        let v1554 = (v512 * v1552) / v1550;
                        let v3784 = ((Lanes([(v2144 * v1552), 0.0, 0.0])) + ((((((v3767 - (Lanes([(v2144 * v1548), 0.0, 0.0]))) / v512) * v1866) * (v1550 * (v1549.powf((v1550 - v1773))))) * v1866) * v512)) / v1550;
                        v1556 = v1554;
                        v1863 = v3784;
                    }
                    v1555 = v1556;
                    v1557 = v0;
                    v1861 = v1863;
                    v1862 = v3146;
                }
                let v1558 = v1555 + v1557;
                let v3821 = v1861 + v1862;
                v1714 = v1558;
                v1860 = v3821;
            } else {
                let v1561 = if (if v721 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v751 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1715: f64;
                let v1864: Lanes<3>;
                if v1561 != 0.0 {
                    let v1563 = v721 - v700;
                    let v3653 = v2373 * v1866;
                    let v1564 = (v721 + v700) / v1563;
                    let v3656 = (v2373 - (v3653 * v1564)) / v1563;
                    let v1566 = v1564 - v2;
                    let v3658 = v3656 * v1566;
                    let v1569 = (v107 * v701) * v701;
                    let v1571 = ((v1566 * v1566) + v1569).sqrt();
                    let v1572 = v1564 + v2;
                    let v3663 = v3656 * v1572;
                    let v1575 = (v107 * v751) * v751;
                    let v1577 = ((v1572 * v1572) + v1575).sqrt();
                    let v1578 = v1571 + v1577;
                    let v1579 = (v448 * v1564) / v1578;
                    let v1583 = v105 * (((v1579 * v1563) - v721) - v700);
                    let v3676 = ((((((v3656 * v448) - ((((v3658 + v3658) * (v1773 / (v2096 * v1571))) + ((v3663 + v3663) * (v1773 / (v2096 * v1577)))) * v1579)) / v1578) * v1563) + (v3653 * v1579)) - v2373) * v105;
                    let v1584 = v1583 / v512;
                    let v1585 = v2 - v1584;
                    let v1586 = v2 - v520;
                    let v3681 = v1586 - v1773;
                    let v1588 = v2 - (v1585.powf(v1586));
                    let v3690 = v2248 * v448;
                    let v3692 = Lanes([v2373, 0.0, 0.0]);
                    let v1594 = (((v448 * v610) + v721) + v700) / v1563;
                    let v3697 = (((Lanes([0.0, v3690[0], v3690[1]])) + v3692) - (Lanes([(v3653 * v1594), 0.0, 0.0]))) / v1563;
                    let v1596 = v1594 - v2;
                    let v3699 = v3697 * v1596;
                    let v1599 = ((v1596 * v1596) + v1569).sqrt();
                    let v1600 = v1594 + v2;
                    let v3704 = v3697 * v1600;
                    let v1603 = ((v1600 * v1600) + v1575).sqrt();
                    let v1604 = v1599 + v1603;
                    let v1605 = (v448 * v1594) / v1604;
                    let v3712 = ((v3697 * v448) - ((((v3699 + v3699) * (v1773 / (v2096 * v1599))) + ((v3704 + v3704) * (v1773 / (v2096 * v1603)))) * v1605)) / v1604;
                    let v1609 = v105 * (((v1605 * v1563) - v721) - v700);
                    let v3718 = (((v3712 * v1563) + (Lanes([(v3653 * v1605), 0.0, 0.0]))) - v3692) * v105;
                    let v1610 = v1609 / v512;
                    let v1611 = v2 - v1610;
                    let v1613 = v2 - (v1611.powf(v1586));
                    let v1617 = v105 * (v1605 + v2);
                    let v3733 = v3712 * v105;
                    let v1618 = v721 / v512;
                    let v1619 = v2 + v1618;
                    let v1620 = -v520;
                    let v1621 = v1619.powf(v1620);
                    let v3737 = v1620 - v1773;
                    let v1622 = v700 / v512;
                    let v1623 = v2 + v1622;
                    let v1624 = v1623.powf(v1620);
                    let v1625 = v2 - v1617;
                    let v1628 = (v1625 * v1621) + (v1617 * v1624);
                    let v1630 = (v610 - v1609) + v1583;
                    let v1633 = ((v1630 * v1628) + ((v512 * v1613) / v1586)) - ((v512 * v1588) / v1586);
                    let v3766 = ((((((Lanes([0.0, v2248[0], v2248[1]])) - v3718) + (Lanes([v3676, 0.0, 0.0]))) * v1628) + (((((v3733 * v1866) * v1621) + (Lanes([(((((v2144 * v1618) * v1866) / v512) * (v1620 * (v1619.powf(v3737)))) * v1625), 0.0, 0.0]))) + ((v3733 * v1624) + (Lanes([((((v2373 - (v2144 * v1622)) / v512) * (v1620 * (v1623.powf(v3737)))) * v1617), 0.0, 0.0])))) * v1630)) + (((Lanes([(v2144 * v1613), 0.0, 0.0])) + ((((((v3718 - (Lanes([(v2144 * v1610), 0.0, 0.0]))) / v512) * v1866) * (v1586 * (v1611.powf(v3681)))) * v1866) * v512)) / v1586)) - (Lanes([(((v2144 * v1588) + ((((((v3676 - (v2144 * v1584)) / v512) * v1866) * (v1586 * (v1585.powf(v3681)))) * v1866) * v512)) / v1586), 0.0, 0.0]));
                    v1715 = v1633;
                    v1864 = v3766;
                } else {
                    let v3603 = v2373 * v700;
                    let v1636 = (v107 * v701) * v701;
                    let v1638 = ((v700 * v700) + v1636).sqrt();
                    let v1641 = v1639 * (v700 + v1638);
                    let v3609 = (v2373 + ((v3603 + v3603) * (v1773 / (v2096 * v1638)))) * v1639;
                    let v1642 = v1641 / v512;
                    let v1643 = v2 - v1642;
                    let v1644 = v2 - v520;
                    let v1645 = v1643.powf(v1644);
                    let v3614 = v1644 - v1773;
                    let v1648 = v610 + v700;
                    let v3622 = Lanes([0.0, v2248[0], v2248[1]]);
                    let v3623 = Lanes([v2373, 0.0, 0.0]);
                    let v3624 = v3622 + v3623;
                    let v3625 = v3624 * v1648;
                    let v1651 = ((v1648 * v1648) + v1636).sqrt();
                    let v1654 = (v105 * (v1648 - v1651)) - v700;
                    let v3632 = ((v3624 - ((v3625 + v3625) * (v1773 / (v2096 * v1651)))) * v105) - v3623;
                    let v1655 = v1654 / v512;
                    let v1656 = v2 - v1655;
                    let v1657 = v1656.powf(v1644);
                    let v1662 = (v2 - v628).powf((-v520));
                    let v1667 = (((v699 * v1657) / v1644) + (v1662 * ((v610 - v1654) + v1641))) - ((v699 * v1645) / v1644);
                    let v3652 = ((((Lanes([(v2372 * v1657), 0.0, 0.0])) + (((((v3632 - (Lanes([(v2144 * v1655), 0.0, 0.0]))) / v512) * v1866) * (v1644 * (v1656.powf(v3614)))) * v699)) / v1644) + (((v3622 - v3632) + (Lanes([v3609, 0.0, 0.0]))) * v1662)) - (Lanes([(((v2372 * v1645) + (((((v3609 - (v2144 * v1642)) / v512) * v1866) * (v1644 * (v1643.powf(v3614)))) * v699)) / v1644), 0.0, 0.0]));
                    v1715 = v1667;
                    v1864 = v3652;
                }
                v1714 = v1715;
                v1860 = v1864;
            }
            let v1668 = if v875 > v0 { 1.0 } else { 0.0 };
            let v1669: f64;
            if v1668 != 0.0 {
                v1669 = v2;
            } else {
                v1669 = v0;
            }
            let v1671 = (v875 * v1669) * v38;
            let v3823 = (v2620 * v1669) * v38;
            let v1672 = v1671 + v2;
            let v1673 = v1671 / v1672;
            let v3826 = (v3823 - (v3823 * v1673)) / v1672;
            let v1676 = (v602 * v34) / v1675;
            let v3828 = (v2236 * v34) / v1675;
            let v1677 = if v1676 < v18 { 1.0 } else { 0.0 };
            let v1689: f64;
            let v1865: Lanes<2>;
            if v1677 != 0.0 {
                let v1678 = v1676.exp();
                let v3830 = v3828 * v1678;
                v1689 = v1678;
                v1865 = v3830;
            } else {
                let v1679 = v18.exp();
                let v1682 = v1679 * (v2 + (v1676 - v18));
                let v3829 = v3828 * v1679;
                v1689 = v1682;
                v1865 = v3829;
            }
            let v1687 = v1683 * (v2 + (v1684 * v908));
            let v1690 = v1688 * v1689;
            let v3834 = v3826 * v1673;
            let v1692 = v39 + (v1673 * v1673);
            let v3836 = (v1865 * v1688) * v1692;
            let v3837 = (v3834 + v3834) * v1690;
            let v1695 = v2 + ((v1690 * v1692) * v1669);
            let v1696 = v1687 * v1695;
            let v3849 = ((Lanes([(v2152 * v893), 0.0, 0.0])) + (v1804 * v517)) * v980;
            let v3851 = v2620 * v1696;
            let v1700 = (v1696 * v875) / v936;
            let v1704 = v2 - v980;
            let v3876 = (Lanes([(v2161 * v1714), 0.0, 0.0])) + (v1860 * v524);
            let v1721 = v613 * v1720;
            let v3880 = v2251 * v1720;
            let v1723 = v617 * v1722;
            let v3881 = v2258 * v1722;
            let v1725 = v294 * v1724;
            let v3882 = v1774 * v1724;
            let v1727 = v1726 * v625;
            let v3883 = v1784 * v1726;
            let v1730 = (v1726 * v626) * v1729;
            let v3885 = (v1785 * v1726) * v1729;
            let v1731 = v590 * (((v517 * v893) * v980) + v1700);
            let v3886 = ((Lanes([v3849[0], 0.0, v3849[1], v3849[2]])) + ((((((((v2668 * v1684) * v1683) * v1695) + ((((Lanes([0.0, v3836[0], v3836[1], 0.0])) + (Lanes([v3837[0], 0.0, v3837[1], v3837[2]]))) * v1669) * v1687)) * v875) + (Lanes([v3851[0], 0.0, v3851[1], v3851[2]]))) - (v1814 * v1700)) / v936)) * v590;
            let v1732 = v590 * ((v517 * v1702) * v1704);
            let v3887 = (((Lanes([(v2152 * v1702), 0.0, 0.0])) + (v1857 * v517)) * v1704) * v590;
            let v1733 = v590 * (((v522 * v896) + (v1707 * v892)) + (v1710 * v1260));
            let v3888 = ((((Lanes([(v2160 * v896), 0.0, 0.0])) + (v1807 * v522)) + (v2650 * v1707)) + (v3210 * v1710)) * v590;
            let v1734 = v590 * (v1710 * v1264);
            let v3889 = (v3217 * v1710) * v590;
            let v1735 = v590 * ((v524 * v1714) + (v1707 * v1717));
            let v3890 = ((Lanes([v3876[0], 0.0, v3876[1], 0.0, v3876[2]])) + (v1818 * v1707)) * v590;
            let v1736 = ddt(9699, v1731);
            let v3892 = v3886 * v3891;
            let v1737 = ddt(9701, v1732);
            let v3893 = v3887 * v3891;
            let v1738 = ddt(9703, v1733);
            let v3894 = v3888 * v3891;
            let v1739 = ddt(9705, v1734);
            let v3895 = v3889 * v3891;
            let v1740 = ddt(9707, v1735);
            let v3896 = v3890 * v3891;
            let v1741 = ddt(9709, v1721);
            let v3897 = v3880 * v3891;
            let v1742 = ddt(9711, v1723);
            let v3898 = v3881 * v3891;
            let v1743 = ddt(9713, v1727);
            let v3899 = v3883 * v3891;
            let v1744 = ddt(9715, v1730);
            let v3900 = v3885 * v3891;
            let v1745 = ddt(9717, v1725);
            let v3901 = v3882 * v3891;
            let v1760: f64;
            let v1761: f64;
            let v1762: f64;
            let v1763: f64;
            let v1764: f64;
            let v1765: f64;
            let v1766: f64;
            let v1767: f64;
            let v1768: f64;
            let v1769: f64;
            let v1770: f64;
            let v1771: f64;
            let v1772: f64;
            if v1746 != 0.0 {
                v1760 = v1747;
                v1761 = v1748;
                v1762 = v1749;
                v1763 = v1750;
                v1764 = v1751;
                v1765 = v1752;
                v1766 = v1753;
                v1767 = v1754;
                v1768 = v1755;
                v1769 = v1756;
                v1770 = v1757;
                v1771 = v1758;
                v1772 = v1759;
            } else {
                v1760 = v0;
                v1761 = v0;
                v1762 = v0;
                v1763 = v0;
                v1764 = v0;
                v1765 = v0;
                v1766 = v0;
                v1767 = v0;
                v1768 = v0;
                v1769 = v0;
                v1770 = v0;
                v1771 = v0;
                v1772 = v0;
            }
            let v3902 = v3503[0];
            let v3903 = v3503[1];
            let v3904 = v3503[2];
            let v3905 = v3503[3];
            let v3906 = v3504[0];
            let v3907 = v3504[1];
            let v3908 = v3504[2];
            let v3909 = v3504[3];
            let v3910 = v3505;
            let v3911 = v3506[0];
            let v3912 = v3506[1];
            let v3913 = v3506[2];
            let v3914 = v3506[3];
            let v3915 = v3507[0];
            let v3916 = v3507[1];
            let v3917 = v3507[2];
            let v3918 = v3507[3];
            let v3919 = v3507[4];
            let v3920 = v3508[0];
            let v3921 = v3508[1];
            let v3922 = v3508[2];
            let v3923 = v3508[3];
            let v3924 = v3509[0];
            let v3925 = v3509[1];
            let v3926 = v3509[2];
            let v3927 = v3222[0];
            let v3928 = v3222[1];
            let v3929 = v3222[2];
            let v3930 = v3510[0];
            let v3931 = v3510[1];
            let v3932 = v3510[2];
            let v3933 = v3510[3];
            let v3934 = v3279[0];
            let v3935 = v3279[1];
            let v3936 = v3279[2];
            let v3937 = v3288[0];
            let v3938 = v3288[1];
            let v3939 = v3288[2];
            let v3940 = v3288[3];
            let v3941 = v3288[4];
            let v3942 = v3293[0];
            let v3943 = v3293[1];
            let v3944 = v3293[2];
            let v3945 = v3302[0];
            let v3946 = v3302[1];
            let v3947 = v3302[2];
            let v3948 = v3302[3];
            let v3949 = v3302[4];
            let v3950 = v3302[5];
            let v3951 = v3484[0];
            let v3952 = v3484[1];
            let v3953 = v3484[2];
            let v3954 = v3484[3];
            let v3955 = v3484[4];
            let v3956 = v3487[0];
            let v3957 = v3487[1];
            let v3958 = v3482;
            let v3959 = v3479[0];
            let v3960 = v3479[1];
            let v3961 = v3479[2];
            let v3962 = v3479[3];
            let v3963 = v3479[4];
            let v3964 = v3479[5];
            let v3965 = v3479[6];
            let v3966 = v3479[7];
            let v3967 = v3479[8];
            let v3968 = v3479[9];
            let v3969 = v3479[10];
            let v3970 = v3892[0];
            let v3971 = v3892[1];
            let v3972 = v3892[2];
            let v3973 = v3892[3];
            let v3974 = v3893[0];
            let v3975 = v3893[1];
            let v3976 = v3893[2];
            let v3977 = v3894[0];
            let v3978 = v3894[1];
            let v3979 = v3894[2];
            let v3980 = v3895[0];
            let v3981 = v3895[1];
            let v3982 = v3895[2];
            let v3983 = v3896[0];
            let v3984 = v3896[1];
            let v3985 = v3896[2];
            let v3986 = v3896[3];
            let v3987 = v3896[4];
            let v3988 = v3897[0];
            let v3989 = v3897[1];
            let v3990 = v3898[0];
            let v3991 = v3898[1];
            let v3992 = v3899;
            let v3993 = v3900;
            let v3994 = v3901;
            let v3995 = v3886[0];
            let v3996 = v3886[1];
            let v3997 = v3886[2];
            let v3998 = v3886[3];
            let v3999 = v3887[0];
            let v4000 = v3887[1];
            let v4001 = v3887[2];
            let v4002 = v3888[0];
            let v4003 = v3888[1];
            let v4004 = v3888[2];
            let v4005 = v3889[0];
            let v4006 = v3889[1];
            let v4007 = v3889[2];
            let v4008 = v3890[0];
            let v4009 = v3890[1];
            let v4010 = v3890[2];
            let v4011 = v3890[3];
            let v4012 = v3890[4];
            let v4013 = v3880[0];
            let v4014 = v3880[1];
            let v4015 = v3881[0];
            let v4016 = v3881[1];
            let v4017 = v3883;
            let v4018 = v3885;
            let v4019 = v3882;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1438),
            [3, 5, 7, 8],
            [v3902, v3903, v3904, v3905],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (v1439),
            [3, 6, 7, 8],
            [v3906, v3907, v3908, v3909],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(5),
            Some(8),
            multiplicity * (v1440),
            [11],
            [v3910],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(5),
            multiplicity * (v1441),
            [3, 5, 7, 8],
            [v3911, v3912, v3913, v3914],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (v1442),
            [3, 5, 7, 8, 11],
            [v3915, v3916, v3917, v3918, v3919],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(4),
            multiplicity * (v1443),
            [0, 3, 4, 6],
            [v3920, v3921, v3922, v3923],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(9),
            multiplicity * (v1444),
            [3, 6, 9],
            [v3924, v3925, v3926],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(4),
            multiplicity * (v1265),
            [0, 3, 4],
            [v3927, v3928, v3929],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(4),
            Some(5),
            multiplicity * (v1445),
            [3, 4, 5, 7],
            [v3930, v3931, v3932, v3933],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(6),
            multiplicity * (v1289),
            [1, 3, 6],
            [v3934, v3935, v3936],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (v1291),
            [3, 5, 6, 7, 8],
            [v3937, v3938, v3939, v3940, v3941],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(8),
            multiplicity * (v1292),
            [2, 3, 8],
            [v3942, v3943, v3944],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(4),
            multiplicity * (v1296),
            [3, 4, 5, 6, 7, 9],
            [v3945, v3946, v3947, v3948, v3949, v3950],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            None,
            multiplicity * (v1424),
            [3, 5, 7, 8, 11],
            [v3951, v3952, v3953, v3954, v3955],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(11),
            None,
            multiplicity * (v1425),
            [10, 11],
            [v3956, v3957],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v1423),
            [3],
            [v3958],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<11, 0>(
            Some(3),
            None,
            multiplicity * (v1422),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11],
            [v3959, v3960, v3961, v3962, v3963, v3964, v3965, v3966, v3967, v3968, v3969],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1736),
            [3, 5, 7, 8],
            [v3970, v3971, v3972, v3973],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(8),
            multiplicity * (v1737),
            [3, 6, 8],
            [v3974, v3975, v3976],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(5),
            multiplicity * (v1738),
            [3, 5, 7],
            [v3977, v3978, v3979],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(4),
            multiplicity * (v1739),
            [3, 4, 7],
            [v3980, v3981, v3982],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (v1740),
            [3, 5, 6, 7, 9],
            [v3983, v3984, v3985, v3986, v3987],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v1741),
            [1, 2],
            [v3988, v3989],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (v1742),
            [0, 1],
            [v3990, v3991],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(10),
            None,
            multiplicity * (v1743),
            [10],
            [v3992],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(11),
            None,
            multiplicity * (v1744),
            [11],
            [v3993],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(3),
            None,
            multiplicity * (v1745),
            [3],
            [v3994],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1760),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1761),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(8),
            multiplicity * (v1762),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(8),
            multiplicity * (v1763),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(8),
            multiplicity * (v1764),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(9),
            multiplicity * (v1765),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(9),
            multiplicity * (v1766),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(4),
            multiplicity * (v1767),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(4),
            Some(5),
            multiplicity * (v1768),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(6),
            multiplicity * (v1769),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (v1770),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(8),
            multiplicity * (v1771),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(4),
            multiplicity * (v1772),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v1438;
        self.canonical_reactive[1] = v1439;
        self.canonical_reactive[2] = v1440;
        self.canonical_reactive[3] = v1441;
        self.canonical_reactive[4] = v1442;
        self.canonical_reactive[5] = v1443;
        self.canonical_reactive[6] = v1444;
        self.canonical_reactive[7] = v1265;
        self.canonical_reactive[8] = v1445;
        self.canonical_reactive[9] = v1289;
        self.canonical_reactive[10] = v1291;
        self.canonical_reactive[11] = v1292;
        self.canonical_reactive[12] = v1296;
        self.canonical_reactive[13] = v1424;
        self.canonical_reactive[14] = v1425;
        self.canonical_reactive[15] = v1423;
        self.canonical_reactive[16] = v1422;
        self.canonical_reactive[17] = v1731;
        self.canonical_reactive[18] = v3995;
        self.canonical_reactive[19] = v3996;
        self.canonical_reactive[20] = v3997;
        self.canonical_reactive[21] = v3998;
        self.canonical_reactive[22] = v1732;
        self.canonical_reactive[23] = v3999;
        self.canonical_reactive[24] = v4000;
        self.canonical_reactive[25] = v4001;
        self.canonical_reactive[26] = v1733;
        self.canonical_reactive[27] = v4002;
        self.canonical_reactive[28] = v4003;
        self.canonical_reactive[29] = v4004;
        self.canonical_reactive[30] = v1734;
        self.canonical_reactive[31] = v4005;
        self.canonical_reactive[32] = v4006;
        self.canonical_reactive[33] = v4007;
        self.canonical_reactive[34] = v1735;
        self.canonical_reactive[35] = v4008;
        self.canonical_reactive[36] = v4009;
        self.canonical_reactive[37] = v4010;
        self.canonical_reactive[38] = v4011;
        self.canonical_reactive[39] = v4012;
        self.canonical_reactive[40] = v1721;
        self.canonical_reactive[41] = v4013;
        self.canonical_reactive[42] = v4014;
        self.canonical_reactive[43] = v1723;
        self.canonical_reactive[44] = v4015;
        self.canonical_reactive[45] = v4016;
        self.canonical_reactive[46] = v1727;
        self.canonical_reactive[47] = v4017;
        self.canonical_reactive[48] = v1730;
        self.canonical_reactive[49] = v4018;
        self.canonical_reactive[50] = v1725;
        self.canonical_reactive[51] = v4019;
        self.canonical_reactive[52] = v1760;
        self.canonical_reactive[53] = v1761;
        self.canonical_reactive[54] = v1762;
        self.canonical_reactive[55] = v1763;
        self.canonical_reactive[56] = v1764;
        self.canonical_reactive[57] = v1765;
        self.canonical_reactive[58] = v1766;
        self.canonical_reactive[59] = v1767;
        self.canonical_reactive[60] = v1768;
        self.canonical_reactive[61] = v1769;
        self.canonical_reactive[62] = v1770;
        self.canonical_reactive[63] = v1771;
        self.canonical_reactive[64] = v1772;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(8),
            &[3, 5, 7, 8],
            &[cached[18], cached[19], cached[20], cached[21]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(8),
            &[3, 6, 8],
            &[cached[23], cached[24], cached[25]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[3, 5, 7],
            &[cached[27], cached[28], cached[29]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(4),
            &[3, 4, 7],
            &[cached[31], cached[32], cached[33]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(9),
            &[3, 5, 6, 7, 9],
            &[cached[35], cached[36], cached[37], cached[38], cached[39]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(10),
            None,
            &[10],
            &[cached[47]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            None,
            &[11],
            &[cached[49]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            None,
            &[3],
            &[cached[51]],
            &[],
            &[],
            multiplicity,
        );
    }

}

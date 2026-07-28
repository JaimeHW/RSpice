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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 10542 => 0usize, 10544 => 1usize, 10546 => 2usize, 10548 => 3usize, 10550 => 4usize, 10552 => 5usize, 10554 => 6usize, 10556 => 7usize, 10558 => 8usize, 10560 => 9usize, 10562 => 10usize, _ => usize::MAX };
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
            let v285 = parameters[68];
            let v286 = parameters[69];
            let v290 = parameters[119];
            let v302 = node_potentials[4];
            let v323 = parameters[126];
            let v326 = if parameter_given[109] { 1.0 } else { 0.0 };
            let v327 = parameters[16];
            let v328 = parameters[109];
            let v331 = parameters[107];
            let v334 = if parameter_given[108] { 1.0 } else { 0.0 };
            let v335 = parameters[17];
            let v336 = parameters[108];
            let v341 = if parameter_given[106] { 1.0 } else { 0.0 };
            let v342 = parameters[21];
            let v343 = parameters[106];
            let v346 = parameters[104];
            let v349 = if parameter_given[105] { 1.0 } else { 0.0 };
            let v350 = parameters[22];
            let v351 = parameters[105];
            let v356 = parameters[23];
            let v357 = parameters[103];
            let v360 = parameters[24];
            let v361 = parameters[111];
            let v364 = if parameter_given[110] { 1.0 } else { 0.0 };
            let v365 = parameters[25];
            let v366 = parameters[110];
            let v371 = parameters[101];
            let v372 = parameters[132];
            let v444 = parameters[129];
            let v449 = parameters[84];
            let v450 = parameters[127];
            let v454 = parameters[86];
            let v455 = parameters[128];
            let v459 = parameters[91];
            let v460 = parameters[92];
            let v466 = parameters[93];
            let v470 = 2e0f64;
            let v473 = parameters[37];
            let v478 = -5e-1f64;
            let v487 = 3e0f64;
            let v507 = parameters[42];
            let v512 = -5e-1f64;
            let v535 = parameters[50];
            let v540 = -5e-1f64;
            let v563 = parameters[36];
            let v565 = parameters[38];
            let v568 = parameters[41];
            let v570 = parameters[43];
            let v573 = parameters[48];
            let v575 = parameters[49];
            let v577 = parameters[51];
            let v580 = parameters[19];
            let v586 = parameters[18];
            let v587 = parameters[112];
            let v594 = parameters[70];
            let v595 = parameters[130];
            let v599 = parameters[71];
            let v600 = parameters[131];
            let v605 = 1e-3f64;
            let v608 = 1e3f64;
            let v650 = node_potentials[8];
            let v651 = node_potentials[9];
            let v654 = node_potentials[7];
            let v657 = node_potentials[6];
            let v660 = node_potentials[5];
            let v665 = node_potentials[10];
            let v668 = node_potentials[1];
            let v669 = node_potentials[2];
            let v673 = node_potentials[0];
            let v682 = node_potentials[11];
            let v687 = node_potentials[3];
            let v689 = node_potentials[12];
            let v690 = node_potentials[13];
            let v692 = parameters[34];
            let v694 = parameters[39];
            let v728 = -5e-1f64;
            let v765 = parameters[44];
            let v770 = -1e0f64;
            let v785 = parameters[45];
            let v815 = parameters[46];
            let v895 = -5e-1f64;
            let v964 = 1e-4f64;
            let v967 = 1e-8f64;
            let v976 = parameters[30];
            let v1027 = parameters[32];
            let v1043 = 5.0005e-1f64;
            let v1059 = parameters[55];
            let v1083 = parameters[57];
            let v1376 = parameters[83];
            let v1378 = 2e-2f64;
            let v1381 = 1.01e0f64;
            let v1409 = parameters[85];
            let v1413 = parameters[87];
            let v1440 = parameters[97];
            let v1442 = parameters[95];
            let v1445 = parameters[94];
            let v1449 = 1e-1f64;
            let v1461 = parameters[96];
            let v1539 = parameters[2];
            let v1572 = parameters[52];
            let v1606 = -5e-1f64;
            let v1673 = -5e-1f64;
            let v1711 = -1e0f64;
            let v1834 = -5e-1f64;
            let v1870 = 1.44e0f64;
            let v1878 = parameters[76];
            let v1879 = parameters[77];
            let v1883 = parameters[78];
            let v1902 = parameters[81];
            let v1905 = parameters[47];
            let v1918 = parameters[53];
            let v1921 = parameters[35];
            let v1923 = parameters[40];
            let v1925 = parameters[102];
            let v1927 = parameters[82];
            let v1930 = 3.333333333333333e-1f64;
            let v1949 = parameters[1];
            let v1950 = 0e0f64;
            let v1951 = 0e0f64;
            let v1952 = 0e0f64;
            let v1953 = 0e0f64;
            let v1954 = 0e0f64;
            let v1955 = 0e0f64;
            let v1956 = 0e0f64;
            let v1957 = 0e0f64;
            let v1958 = 0e0f64;
            let v1959 = 0e0f64;
            let v1960 = 0e0f64;
            let v1961 = 0e0f64;
            let v1962 = 0e0f64;
            let v1963 = 0e0f64;
            let v1964 = 0e0f64;
            let v1980 = 1e0f64;
            let v1981 = Lanes([1e0f64; 1]);
            let v1982 = Lanes([1e0f64; 1]);
            let v1983 = Lanes([1e0f64; 1]);
            let v1984 = Lanes([1e0f64; 1]);
            let v1985 = Lanes([1e0f64; 1]);
            let v1986 = Lanes([1e0f64; 1]);
            let v1987 = Lanes([1e0f64; 1]);
            let v1988 = Lanes([1e0f64; 1]);
            let v1989 = Lanes([1e0f64; 1]);
            let v1990 = Lanes([1e0f64; 1]);
            let v1991 = Lanes([1e0f64; 1]);
            let v1992 = Lanes([1e0f64; 1]);
            let v1993 = Lanes([1e0f64; 1]);
            let v1994 = Lanes([1e0f64; 1]);
            let v2085 = -1e0f64;
            let v2348 = 2e0f64;
            let v2489 = Lanes([0e0f64; 1]);
            let v2669 = Lanes([0e0f64; 3]);
            let v2892 = Lanes([0e0f64; 3]);
            let v3021 = Lanes([0e0f64; 5]);
            let v3022 = Lanes([0e0f64; 6]);
            let v3345 = Lanes([0e0f64; 4]);
            let v3437 = Lanes([0e0f64; 4]);
            let v3487 = Lanes([0e0f64; 3]);
            let v3649 = Lanes([0e0f64; 5]);
            let v3691 = Lanes([0e0f64; 4]);
            let v3730 = Lanes([0e0f64; 2]);
            let v3754 = Lanes([0e0f64; 3]);
            let v4101 = Lanes([0e0f64; 3]);
            let v4412 = ddt_scale();
            if v1 != 0.0 {
            } else {
            }
            if v3 != 0.0 {
            } else {
            }
            let v1545: f64;
            if v5 != 0.0 {
                v1545 = v6;
            } else {
                let v8 = ctx.simparam_or("gmin", v7);
                v1545 = v8;
            }
            let v82: f64;
            if v9 != 0.0 {
                v82 = v10;
            } else {
                let v11 = ctx.simparam_or("pnjmaxi", v2);
                v82 = v11;
            }
            let v647: f64;
            if v12 != 0.0 {
                v647 = v2;
            } else {
                let v648: f64;
                if v13 != 0.0 {
                    v648 = v14;
                } else {
                    let v649: f64;
                    if v15 != 0.0 {
                        v649 = v16;
                    } else {
                        v649 = v2;
                    }
                    v648 = v649;
                }
                v647 = v648;
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
            let v1104: f64;
            if v75 != 0.0 {
                let v77 = v76 * v71;
                let v86 = v77 * (((((-v78) / v77).exp()) + (v82 / v74)).ln());
                v1104 = v86;
            } else {
                v1104 = v0;
            }
            let v89 = v87 / v88;
            let v93 = -v92;
            let v94 = v2 - v72;
            let v96 = v71 * v88;
            let v99 = (v73 * (v72.powf(v89))) * (((v93 * v94) / v96).exp());
            let v100 = if v99 > v0 { 1.0 } else { 0.0 };
            let v926: f64;
            if v100 != 0.0 {
                let v104 = if (if v101 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v82 > v101 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v927: f64;
                if v104 != 0.0 {
                    let v118 = v96 * ((v2 + ((((v105 * v82) * ((v107 / v101).powf(v109))).powf((v2 / (v2 - v109)))) / v99)).ln());
                    v927 = v118;
                } else {
                    let v122 = v96 * ((v2 + (v82 / v99)).ln());
                    v927 = v122;
                }
                v926 = v927;
            } else {
                v926 = v0;
            }
            let v126 = v124 / v125;
            let v130 = -v129;
            let v132 = v71 * v125;
            let v135 = (v123 * (v72.powf(v126))) * (((v130 * v94) / v132).exp());
            let v137 = if v100 != 0.0 && (if v135 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v942: f64;
            if v137 != 0.0 {
                let v139 = if v20 != 0.0 && (if v82 > v19 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v943: f64;
                if v139 != 0.0 {
                    let v151 = v132 * ((v2 + ((((v105 * v82) * ((v107 / v19).powf(v109))).powf((v2 / (v2 - v109)))) / (v99 * v135))).ln());
                    v943 = v151;
                } else {
                    let v156 = v132 * ((v2 + (v82 / (v99 * v135))).ln());
                    v943 = v156;
                }
                v942 = v943;
            } else {
                v942 = v0;
            }
            let v159 = v87 / v158;
            let v163 = -v162;
            let v165 = v71 * v158;
            let v168 = (v157 * (v72.powf(v159))) * (((v163 * v94) / v165).exp());
            let v169 = if v168 > v0 { 1.0 } else { 0.0 };
            let v1007: f64;
            if v169 != 0.0 {
                let v171 = if v24 != 0.0 && (if v82 > v23 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1008: f64;
                if v171 != 0.0 {
                    let v177 = v165 * ((v2 + (((v82 * v82) * v26) / v168)).ln());
                    v1008 = v177;
                } else {
                    let v181 = v165 * ((v2 + (v82 / v168)).ln());
                    v1008 = v181;
                }
                v1007 = v1008;
            } else {
                v1007 = v0;
            }
            let v185 = v183 / v184;
            let v189 = -v188;
            let v191 = v71 * v184;
            let v194 = (v182 * (v72.powf(v185))) * (((v189 * v94) / v191).exp());
            let v195 = if v194 > v0 { 1.0 } else { 0.0 };
            let v1062: f64;
            if v195 != 0.0 {
                let v199 = v191 * ((v2 + (v82 / v194)).ln());
                v1062 = v199;
            } else {
                v1062 = v0;
            }
            let v203 = v201 / v202;
            let v207 = -v206;
            let v209 = v71 * v202;
            let v212 = (v200 * (v72.powf(v203))) * (((v207 * v94) / v209).exp());
            let v213 = if v212 > v0 { 1.0 } else { 0.0 };
            let v1073: f64;
            if v213 != 0.0 {
                let v217 = v209 * ((v2 + (v82 / v212)).ln());
                v1073 = v217;
            } else {
                v1073 = v0;
            }
            let v220 = v183 / v219;
            let v221 = v72.powf(v220);
            let v224 = -v223;
            let v226 = v71 * v219;
            let v228 = ((v224 * v94) / v226).exp();
            let v229 = (v218 * v221) * v228;
            let v230 = if v229 > v0 { 1.0 } else { 0.0 };
            let v1264: f64;
            if v230 != 0.0 {
                let v234 = v226 * ((v2 + (v82 / v229)).ln());
                v1264 = v234;
            } else {
                v1264 = v0;
            }
            let v237 = v201 / v236;
            let v238 = v72.powf(v237);
            let v241 = -v240;
            let v243 = v71 * v236;
            let v245 = ((v241 * v94) / v243).exp();
            let v246 = (v235 * v238) * v245;
            let v247 = if v246 > v0 { 1.0 } else { 0.0 };
            let v1275: f64;
            if v247 != 0.0 {
                let v251 = v243 * ((v2 + (v82 / v246)).ln());
                v1275 = v251;
            } else {
                v1275 = v0;
            }
            let v254 = (v252 * v221) * v228;
            let v255 = if v254 > v0 { 1.0 } else { 0.0 };
            let v1295: f64;
            if v255 != 0.0 {
                let v259 = v226 * ((v2 + (v82 / v254)).ln());
                v1295 = v259;
            } else {
                v1295 = v0;
            }
            let v262 = (v260 * v238) * v245;
            let v263 = if v262 > v0 { 1.0 } else { 0.0 };
            let v1305: f64;
            if v263 != 0.0 {
                let v267 = v243 * ((v2 + (v82 / v262)).ln());
                v1305 = v267;
            } else {
                v1305 = v0;
            }
            let v270 = v183 / v269;
            let v274 = -v273;
            let v276 = v71 * v269;
            let v279 = (v268 * (v72.powf(v270))) * (((v274 * v94) / v276).exp());
            let v280 = if v279 > v0 { 1.0 } else { 0.0 };
            let v1472: f64;
            if v280 != 0.0 {
                let v284 = v276 * ((v2 + (v82 / v279)).ln());
                v1472 = v284;
            } else {
                v1472 = v0;
            }
            let v287 = v201 / v286;
            let v291 = -v290;
            let v293 = v71 * v286;
            let v296 = (v285 * (v72.powf(v287))) * (((v291 * v94) / v293).exp());
            let v297 = if v296 > v0 { 1.0 } else { 0.0 };
            let v1483: f64;
            if v297 != 0.0 {
                let v301 = v293 * ((v2 + (v82 / v296)).ln());
                v1483 = v301;
            } else {
                v1483 = v0;
            }
            let v304 = (v45 + v302) - v40;
            let v305 = if v304 < v52 { 1.0 } else { 0.0 };
            let v316: f64;
            let v1995: Lanes<1>;
            if v305 != 0.0 {
                let v308 = ((v304 - v51) - v2).exp();
                let v2089 = v1981 * v308;
                let v309 = v51 + v308;
                v316 = v309;
                v1995 = v2089;
            } else {
                let v311 = if v304 > (v58 - v2) { 1.0 } else { 0.0 };
                let v317: f64;
                let v1996: Lanes<1>;
                if v311 != 0.0 {
                    let v314 = ((v58 - v304) - v2).exp();
                    let v315 = v58 - v314;
                    let v2088 = ((v1981 * v2085) * v314) * v2085;
                    v317 = v315;
                    v1996 = v2088;
                } else {
                    v317 = v304;
                    v1996 = v1981;
                }
                v316 = v317;
                v1995 = v1996;
            }
            let v318 = v316 + v40;
            let v320 = (v68 * v318) / v70;
            let v2091 = (v1995 * v68) / v70;
            let v321 = v318 / v42;
            let v2092 = v1995 / v42;
            let v322 = v318 - v42;
            let v325 = v101 * (v321.powf(v323));
            let v2097 = (v2092 * (v323 * (v321.powf((v323 - v1980))))) * v101;
            let v604: f64;
            let v1997: Lanes<1>;
            if v326 != 0.0 {
                let v330 = v327 * (v321.powf(v328));
                let v2107 = (v2092 * (v328 * (v321.powf((v328 - v1980))))) * v327;
                v604 = v330;
                v1997 = v2107;
            } else {
                let v333 = v327 * (v321.powf(v331));
                let v2102 = (v2092 * (v331 * (v321.powf((v331 - v1980))))) * v327;
                v604 = v333;
                v1997 = v2102;
            }
            let v610: f64;
            let v1998: Lanes<1>;
            if v334 != 0.0 {
                let v338 = v335 * (v321.powf(v336));
                let v2117 = (v2092 * (v336 * (v321.powf((v336 - v1980))))) * v335;
                v610 = v338;
                v1998 = v2117;
            } else {
                let v340 = v335 * (v321.powf(v331));
                let v2112 = (v2092 * (v331 * (v321.powf((v331 - v1980))))) * v335;
                v610 = v340;
                v1998 = v2112;
            }
            let v614: f64;
            let v1999: Lanes<1>;
            if v341 != 0.0 {
                let v345 = v342 * (v321.powf(v343));
                let v2127 = (v2092 * (v343 * (v321.powf((v343 - v1980))))) * v342;
                v614 = v345;
                v1999 = v2127;
            } else {
                let v348 = v342 * (v321.powf(v346));
                let v2122 = (v2092 * (v346 * (v321.powf((v346 - v1980))))) * v342;
                v614 = v348;
                v1999 = v2122;
            }
            let v618: f64;
            let v2000: Lanes<1>;
            if v349 != 0.0 {
                let v353 = v350 * (v321.powf(v351));
                let v2137 = (v2092 * (v351 * (v321.powf((v351 - v1980))))) * v350;
                v618 = v353;
                v2000 = v2137;
            } else {
                let v355 = v350 * (v321.powf(v346));
                let v2132 = (v2092 * (v346 * (v321.powf((v346 - v1980))))) * v350;
                v618 = v355;
                v2000 = v2132;
            }
            let v359 = v356 * (v321.powf(v357));
            let v2142 = (v2092 * (v357 * (v321.powf((v357 - v1980))))) * v356;
            let v363 = v360 * (v321.powf(v361));
            let v2147 = (v2092 * (v361 * (v321.powf((v361 - v1980))))) * v360;
            let v625: f64;
            let v2001: Lanes<1>;
            if v364 != 0.0 {
                let v368 = v365 * (v321.powf(v366));
                let v2157 = (v2092 * (v366 * (v321.powf((v366 - v1980))))) * v365;
                v625 = v368;
                v2001 = v2157;
            } else {
                let v370 = v365 * (v321.powf(v331));
                let v2152 = (v2092 * (v331 * (v321.powf((v331 - v1980))))) * v365;
                v625 = v370;
                v2001 = v2152;
            }
            let v375 = v371 * (v2 + (v322 * v372));
            let v2159 = (v1995 * v372) * v371;
            let v377 = v73 * (v321.powf(v89));
            let v378 = v2 - v321;
            let v2165 = v2092 * v2085;
            let v379 = v93 * v378;
            let v2166 = v2165 * v93;
            let v380 = v320 * v88;
            let v381 = v379 / v380;
            let v382 = v381.exp();
            let v383 = v377 * v382;
            let v2174 = (((v2092 * (v89 * (v321.powf((v89 - v1980))))) * v73) * v382) + ((((v2166 - ((v2091 * v88) * v381)) / v380) * v382) * v377);
            let v385 = v123 * (v321.powf(v126));
            let v387 = v320 * v125;
            let v388 = (v130 * v378) / v387;
            let v389 = v388.exp();
            let v390 = v385 * v389;
            let v2188 = (((v2092 * (v126 * (v321.powf((v126 - v1980))))) * v123) * v389) + (((((v2165 * v130) - ((v2091 * v125) * v388)) / v387) * v389) * v385);
            let v392 = v157 * (v321.powf(v159));
            let v394 = v320 * v158;
            let v2195 = v2091 * v158;
            let v395 = (v163 * v378) / v394;
            let v396 = v395.exp();
            let v397 = v392 * v396;
            let v2202 = (((v2092 * (v159 * (v321.powf((v159 - v1980))))) * v157) * v396) + (((((v2165 * v163) - (v2195 * v395)) / v394) * v396) * v392);
            let v399 = v182 * (v321.powf(v185));
            let v401 = v320 * v184;
            let v2209 = v2091 * v184;
            let v402 = (v189 * v378) / v401;
            let v403 = v402.exp();
            let v404 = v399 * v403;
            let v2216 = (((v2092 * (v185 * (v321.powf((v185 - v1980))))) * v182) * v403) + (((((v2165 * v189) - (v2209 * v402)) / v401) * v403) * v399);
            let v406 = v200 * (v321.powf(v203));
            let v408 = v320 * v202;
            let v2223 = v2091 * v202;
            let v409 = (v207 * v378) / v408;
            let v410 = v409.exp();
            let v411 = v406 * v410;
            let v2230 = (((v2092 * (v203 * (v321.powf((v203 - v1980))))) * v200) * v410) + (((((v2165 * v207) - (v2223 * v409)) / v408) * v410) * v406);
            let v412 = v321.powf(v220);
            let v2234 = v2092 * (v220 * (v321.powf((v220 - v1980))));
            let v413 = v218 * v412;
            let v415 = v320 * v219;
            let v2237 = v2091 * v219;
            let v416 = (v224 * v378) / v415;
            let v417 = v416.exp();
            let v2241 = (((v2165 * v224) - (v2237 * v416)) / v415) * v417;
            let v418 = v413 * v417;
            let v2244 = ((v2234 * v218) * v417) + (v2241 * v413);
            let v419 = v321.powf(v237);
            let v2248 = v2092 * (v237 * (v321.powf((v237 - v1980))));
            let v420 = v235 * v419;
            let v422 = v320 * v236;
            let v2251 = v2091 * v236;
            let v423 = (v241 * v378) / v422;
            let v424 = v423.exp();
            let v2255 = (((v2165 * v241) - (v2251 * v423)) / v422) * v424;
            let v425 = v420 * v424;
            let v2258 = ((v2248 * v235) * v424) + (v2255 * v420);
            let v426 = v252 * v412;
            let v427 = v426 * v417;
            let v2262 = ((v2234 * v252) * v417) + (v2241 * v426);
            let v428 = v260 * v419;
            let v429 = v428 * v424;
            let v2266 = ((v2248 * v260) * v424) + (v2255 * v428);
            let v431 = v268 * (v321.powf(v270));
            let v433 = v320 * v269;
            let v2273 = v2091 * v269;
            let v434 = (v274 * v378) / v433;
            let v435 = v434.exp();
            let v436 = v431 * v435;
            let v2280 = (((v2092 * (v270 * (v321.powf((v270 - v1980))))) * v268) * v435) + (((((v2165 * v274) - (v2273 * v434)) / v433) * v435) * v431);
            let v438 = v285 * (v321.powf(v287));
            let v440 = v320 * v286;
            let v2287 = v2091 * v286;
            let v441 = (v291 * v378) / v440;
            let v442 = v441.exp();
            let v443 = v438 * v442;
            let v2294 = (((v2092 * (v287 * (v321.powf((v287 - v1980))))) * v285) * v442) + (((((v2165 * v291) - (v2287 * v441)) / v440) * v442) * v438);
            let v2295 = v1995 * v444;
            let v446 = v2 + (v322 * v444);
            let v447 = v88 * v446;
            let v2296 = v2295 * v88;
            let v448 = v125 * v446;
            let v2297 = v2295 * v125;
            let v453 = v449 * (v2 + (v322 * v450));
            let v2299 = (v1995 * v450) * v449;
            let v458 = v454 * (v2 + (v322 * v455));
            let v2301 = (v1995 * v455) * v454;
            let v462 = v459 + (v322 * v460);
            let v469 = v76 * (v2 + (v322 * v466));
            let v471 = v320 / v321;
            let v472 = v470 * v471;
            let v2312 = ((v2091 - (v2092 * v471)) / v321) * v470;
            let v474 = v105 * v473;
            let v476 = (v474 * v321) / v320;
            let v477 = v476.exp();
            let v479 = v478 * v473;
            let v481 = (v479 * v321) / v320;
            let v482 = v481.exp();
            let v483 = v477 - v482;
            let v484 = v483.ln();
            let v485 = v472 * v484;
            let v488 = v487 * v320;
            let v489 = v321.ln();
            let v490 = v488 * v489;
            let v2337 = ((v2091 * v487) * v489) + ((v2092 * (v1980 / v321)) * v488);
            let v492 = v321 - v2;
            let v494 = ((v485 * v321) - v490) - (v188 * v492);
            let v2340 = (((((v2312 * v484) + (((((((v2092 * v474) - (v2091 * v476)) / v320) * v477) - ((((v2092 * v479) - (v2091 * v481)) / v320) * v482)) * (v1980 / v483)) * v472)) * v321) + (v2092 * v485)) - v2337) - (v2092 * v188);
            let v495 = v470 * v320;
            let v2341 = v2091 * v470;
            let v497 = (-v494) / v320;
            let v498 = v497.exp();
            let v501 = (v2 + (v107 * v498)).sqrt();
            let v503 = v105 * (v2 + v501);
            let v504 = v503.ln();
            let v506 = v494 + (v495 * v504);
            let v2358 = v2340 + ((v2341 * v504) + (((((((((v2340 * v2085) - (v2091 * v497)) / v320) * v498) * v107) * (v1980 / (v2348 * v501))) * v105) * (v1980 / v503)) * v495));
            let v508 = v105 * v507;
            let v510 = (v508 * v321) / v320;
            let v511 = v510.exp();
            let v513 = v512 * v507;
            let v515 = (v513 * v321) / v320;
            let v516 = v515.exp();
            let v517 = v511 - v516;
            let v518 = v517.ln();
            let v519 = v472 * v518;
            let v523 = ((v519 * v321) - v490) - (v223 * v492);
            let v2380 = (((((v2312 * v518) + (((((((v2092 * v508) - (v2091 * v510)) / v320) * v511) - ((((v2092 * v513) - (v2091 * v515)) / v320) * v516)) * (v1980 / v517)) * v472)) * v321) + (v2092 * v519)) - v2337) - (v2092 * v223);
            let v525 = (-v523) / v320;
            let v526 = v525.exp();
            let v529 = (v2 + (v107 * v526)).sqrt();
            let v531 = v105 * (v2 + v529);
            let v532 = v531.ln();
            let v534 = v523 + (v495 * v532);
            let v2396 = v2380 + ((v2341 * v532) + (((((((((v2380 * v2085) - (v2091 * v525)) / v320) * v526) * v107) * (v1980 / (v2348 * v529))) * v105) * (v1980 / v531)) * v495));
            let v536 = v105 * v535;
            let v538 = (v536 * v321) / v320;
            let v539 = v538.exp();
            let v541 = v540 * v535;
            let v543 = (v541 * v321) / v320;
            let v544 = v543.exp();
            let v545 = v539 - v544;
            let v546 = v545.ln();
            let v547 = v472 * v546;
            let v551 = ((v547 * v321) - v490) - (v273 * v492);
            let v2418 = (((((v2312 * v546) + (((((((v2092 * v536) - (v2091 * v538)) / v320) * v539) - ((((v2092 * v541) - (v2091 * v543)) / v320) * v544)) * (v1980 / v545)) * v472)) * v321) + (v2092 * v547)) - v2337) - (v2092 * v273);
            let v553 = (-v551) / v320;
            let v554 = v553.exp();
            let v557 = (v2 + (v107 * v554)).sqrt();
            let v559 = v105 * (v2 + v557);
            let v560 = v559.ln();
            let v562 = v551 + (v495 * v560);
            let v2434 = v2418 + ((v2341 * v560) + (((((((((v2418 * v2085) - (v2091 * v553)) / v320) * v554) * v107) * (v1980 / (v2348 * v557))) * v105) * (v1980 / v559)) * v495));
            let v564 = v473 / v506;
            let v567 = v563 * (v564.powf(v565));
            let v2442 = ((((v2358 * v564) * v2085) / v506) * (v565 * (v564.powf((v565 - v1980))))) * v563;
            let v569 = v507 / v534;
            let v571 = v569.powf(v570);
            let v2449 = (((v2396 * v569) * v2085) / v534) * (v570 * (v569.powf((v570 - v1980))));
            let v572 = v568 * v571;
            let v2450 = v2449 * v568;
            let v574 = v573 * v571;
            let v2451 = v2449 * v573;
            let v576 = v535 / v562;
            let v579 = v575 * (v576.powf(v577));
            let v2459 = ((((v2434 * v576) * v2085) / v562) * (v577 * (v576.powf((v577 - v1980))))) * v575;
            let v582 = v580 * (v321.powf(v87));
            let v583 = v379 / v320;
            let v584 = v583.exp();
            let v585 = v582 * v584;
            let v2471 = (((v2092 * (v87 * (v321.powf((v87 - v1980))))) * v580) * v584) + ((((v2166 - (v2091 * v583)) / v320) * v584) * v582);
            let v589 = v586 * (v321.powf(v587));
            let v2476 = (v2092 * (v587 * (v321.powf((v587 - v1980))))) * v586;
            let v590 = -(v78 * (v2 + (v322 * v462)));
            let v2477 = (((v1995 * v462) + ((v1995 * v460) * v322)) * v78) * v2085;
            let v591 = v469 * v320;
            let v2480 = (((v1995 * v466) * v76) * v320) + (v2091 * v469);
            let v592 = v590 / v591;
            let v593 = v592.exp();
            let v2484 = ((v2477 - (v2480 * v592)) / v591) * v593;
            let v598 = v594 * (v2 + (v322 * v595));
            let v2486 = (v1995 * v595) * v594;
            let v603 = v599 * (v2 + (v322 * v600));
            let v2488 = (v1995 * v600) * v599;
            let v606 = if v604 > v605 { 1.0 } else { 0.0 };
            let v609: f64;
            let v2002: Lanes<1>;
            if v606 != 0.0 {
                let v607 = v2 / v604;
                let v2492 = ((v1997 * v607) * v2085) / v604;
                v609 = v607;
                v2002 = v2492;
            } else {
                v609 = v608;
                v2002 = v2489;
            }
            let v611 = if v610 > v605 { 1.0 } else { 0.0 };
            let v613: f64;
            let v2003: Lanes<1>;
            if v611 != 0.0 {
                let v612 = v2 / v610;
                let v2495 = ((v1998 * v612) * v2085) / v610;
                v613 = v612;
                v2003 = v2495;
            } else {
                v613 = v608;
                v2003 = v2489;
            }
            let v615 = if v614 > v605 { 1.0 } else { 0.0 };
            let v617: f64;
            let v2004: Lanes<1>;
            if v615 != 0.0 {
                let v616 = v2 / v614;
                let v2498 = ((v1999 * v616) * v2085) / v614;
                v617 = v616;
                v2004 = v2498;
            } else {
                v617 = v608;
                v2004 = v2489;
            }
            let v619 = if v618 > v605 { 1.0 } else { 0.0 };
            let v621: f64;
            let v2005: Lanes<1>;
            if v619 != 0.0 {
                let v620 = v2 / v618;
                let v2501 = ((v2000 * v620) * v2085) / v618;
                v621 = v620;
                v2005 = v2501;
            } else {
                v621 = v608;
                v2005 = v2489;
            }
            let v622 = if v359 > v605 { 1.0 } else { 0.0 };
            let v624: f64;
            let v2006: Lanes<1>;
            if v622 != 0.0 {
                let v623 = v2 / v359;
                let v2504 = ((v2142 * v623) * v2085) / v359;
                v624 = v623;
                v2006 = v2504;
            } else {
                v624 = v608;
                v2006 = v2489;
            }
            let v626 = if v625 > v605 { 1.0 } else { 0.0 };
            let v628: f64;
            let v2007: Lanes<1>;
            if v626 != 0.0 {
                let v627 = v2 / v625;
                let v2507 = ((v2001 * v627) * v2085) / v625;
                v628 = v627;
                v2007 = v2507;
            } else {
                v628 = v608;
                v2007 = v2489;
            }
            let v629 = if v363 > v605 { 1.0 } else { 0.0 };
            let v631: f64;
            let v2008: Lanes<1>;
            if v629 != 0.0 {
                let v630 = v2 / v363;
                let v2510 = ((v2147 * v630) * v2085) / v363;
                v631 = v630;
                v2008 = v2510;
            } else {
                v631 = v608;
                v2008 = v2489;
            }
            let v632 = if v375 > v605 { 1.0 } else { 0.0 };
            let v634: f64;
            let v2009: Lanes<1>;
            if v632 != 0.0 {
                let v633 = v2 / v375;
                let v2513 = ((v2159 * v633) * v2085) / v375;
                v634 = v633;
                v2009 = v2513;
            } else {
                v634 = v608;
                v2009 = v2489;
            }
            let v635 = if v598 > v0 { 1.0 } else { 0.0 };
            let v637: f64;
            let v2010: Lanes<1>;
            if v635 != 0.0 {
                let v636 = v2 / v598;
                let v2516 = ((v2486 * v636) * v2085) / v598;
                v637 = v636;
                v2010 = v2516;
            } else {
                v637 = v0;
                v2010 = v2489;
            }
            let v638 = if v603 > v0 { 1.0 } else { 0.0 };
            let v640: f64;
            let v2011: Lanes<1>;
            if v638 != 0.0 {
                let v639 = v2 / v603;
                let v2519 = ((v2488 * v639) * v2085) / v603;
                v640 = v639;
                v2011 = v2519;
            } else {
                v640 = v0;
                v2011 = v2489;
            }
            let v641 = if v325 > v0 { 1.0 } else { 0.0 };
            let v643: f64;
            let v2012: Lanes<1>;
            if v641 != 0.0 {
                let v642 = v2 / v325;
                let v2522 = ((v2097 * v642) * v2085) / v325;
                v643 = v642;
                v2012 = v2522;
            } else {
                v643 = v0;
                v2012 = v2489;
            }
            let v644 = if v589 > v0 { 1.0 } else { 0.0 };
            let v646: f64;
            let v2013: Lanes<1>;
            if v644 != 0.0 {
                let v645 = v2 / v589;
                let v2525 = ((v2476 * v645) * v2085) / v589;
                v646 = v645;
                v2013 = v2525;
            } else {
                v646 = v0;
                v2013 = v2489;
            }
            let v653 = v647 * (v650 - v651);
            let v2529 = ((Lanes([v1982[0], 0.0])) - (Lanes([0.0, v1983[0]]))) * v647;
            let v656 = v647 * (v654 - v651);
            let v2533 = ((Lanes([v1984[0], 0.0])) - (Lanes([0.0, v1983[0]]))) * v647;
            let v659 = v647 * (v650 - v657);
            let v2537 = ((Lanes([0.0, v1982[0]])) - (Lanes([v1985[0], 0.0]))) * v647;
            let v662 = v647 * (v650 - v660);
            let v2541 = ((Lanes([0.0, v1982[0]])) - (Lanes([v1986[0], 0.0]))) * v647;
            let v664 = v647 * (v654 - v660);
            let v2545 = ((Lanes([0.0, v1984[0]])) - (Lanes([v1986[0], 0.0]))) * v647;
            let v667 = v647 * (v654 - v665);
            let v2549 = ((Lanes([v1984[0], 0.0])) - (Lanes([0.0, v1987[0]]))) * v647;
            let v670 = v668 - v669;
            let v2552 = (Lanes([v1988[0], 0.0])) - (Lanes([0.0, v1989[0]]));
            let v672 = v647 * (v657 - v651);
            let v2556 = ((Lanes([v1985[0], 0.0])) - (Lanes([0.0, v1983[0]]))) * v647;
            let v674 = v668 - v673;
            let v2559 = (Lanes([0.0, v1988[0]])) - (Lanes([v1990[0], 0.0]));
            let v675 = v673 - v660;
            let v2562 = (Lanes([v1990[0], 0.0])) - (Lanes([0.0, v1986[0]]));
            let v677 = v647 * (v660 - v657);
            let v2566 = ((Lanes([v1986[0], 0.0])) - (Lanes([0.0, v1985[0]]))) * v647;
            let v678 = v668 - v654;
            let v2569 = (Lanes([v1988[0], 0.0])) - (Lanes([0.0, v1984[0]]));
            let v679 = v654 - v650;
            let v2572 = (Lanes([v1984[0], 0.0])) - (Lanes([0.0, v1982[0]]));
            let v680 = v669 - v651;
            let v2575 = (Lanes([v1989[0], 0.0])) - (Lanes([0.0, v1983[0]]));
            let v681 = v665 - v660;
            let v2578 = (Lanes([0.0, v1987[0]])) - (Lanes([v1986[0], 0.0]));
            let v684 = v647 * (v682 - v665);
            let v2582 = ((Lanes([0.0, v1991[0]])) - (Lanes([v1987[0], 0.0]))) * v647;
            let v686 = v647 * (v654 - v682);
            let v2586 = ((Lanes([v1984[0], 0.0])) - (Lanes([0.0, v1991[0]]))) * v647;
            let v688 = v687 - v682;
            let v2589 = (Lanes([v1992[0], 0.0])) - (Lanes([0.0, v1991[0]]));
            let v691 = -v506;
            let v2590 = v2358 * v2085;
            let v693 = v691 * v692;
            let v2591 = v2590 * v692;
            let v695 = if v694 <= v0 { 1.0 } else { 0.0 };
            let v957: f64;
            let v2014: Lanes<3>;
            if v695 != 0.0 {
                let v696 = v653 + v693;
                let v2651 = Lanes([0.0, v2529[0], v2529[1]]);
                let v2653 = v2651 + (Lanes([v2591[0], 0.0, 0.0]));
                let v697 = if v696 > v0 { 1.0 } else { 0.0 };
                let v720: f64;
                let v721: f64;
                let v2015: Lanes<3>;
                let v2016: Lanes<3>;
                if v697 != 0.0 {
                    let v698 = v2 - v692;
                    let v700 = v698.powf((-v565));
                    let v702 = v2 - (v700 * v698);
                    let v704 = v2 - v565;
                    let v705 = (v506 * v702) / v704;
                    let v2671 = (v2358 * v702) / v704;
                    let v706 = v105 * v565;
                    let v708 = v506 * v698;
                    let v709 = (v706 * v696) / v708;
                    let v2674 = (v2358 * v698) * v709;
                    let v710 = v2 + v709;
                    let v712 = (v696 * v710) * v700;
                    let v2681 = ((v2653 * v710) + ((((v2653 * v706) - (Lanes([v2674[0], 0.0, 0.0]))) / v708) * v696)) * v700;
                    let v2682 = Lanes([v2671[0], 0.0, 0.0]);
                    v720 = v705;
                    v721 = v712;
                    v2015 = v2682;
                    v2016 = v2681;
                } else {
                    let v713 = v653 / v506;
                    let v2654 = v2358 * v713;
                    let v714 = v2 - v713;
                    let v715 = v2 - v565;
                    let v717 = v2 - (v714.powf(v715));
                    let v2664 = v2358 * v717;
                    let v719 = (v506 * v717) / v715;
                    let v2668 = ((Lanes([v2664[0], 0.0, 0.0])) + ((((((v2651 - (Lanes([v2654[0], 0.0, 0.0]))) / v506) * v2085) * (v715 * (v714.powf((v715 - v1980))))) * v2085) * v506)) / v715;
                    v720 = v719;
                    v721 = v0;
                    v2015 = v2668;
                    v2016 = v2669;
                }
                let v722 = v720 + v721;
                let v2683 = v2015 + v2016;
                v957 = v722;
                v2014 = v2683;
            } else {
                let v2592 = v2591 * v693;
                let v725 = (v107 * v694) * v694;
                let v727 = ((v693 * v693) + v725).sqrt();
                let v730 = v728 * (v693 + v727);
                let v2598 = (v2591 + ((v2592 + v2592) * (v1980 / (v2348 * v727)))) * v728;
                let v731 = v730 / v506;
                let v732 = v2 - v731;
                let v733 = v2 - v565;
                let v734 = v732.powf(v733);
                let v2603 = v733 - v1980;
                let v2610 = ((v2590 * v734) + (((((v2598 - (v2358 * v731)) / v506) * v2085) * (v733 * (v732.powf(v2603)))) * v691)) / v733;
                let v737 = v653 + v693;
                let v2611 = Lanes([0.0, v2529[0], v2529[1]]);
                let v2612 = Lanes([v2591[0], 0.0, 0.0]);
                let v2613 = v2611 + v2612;
                let v2614 = v2613 * v737;
                let v740 = ((v737 * v737) + v725).sqrt();
                let v743 = (v105 * (v737 - v740)) - v693;
                let v2621 = ((v2613 - ((v2614 + v2614) * (v1980 / (v2348 * v740)))) * v105) - v2612;
                let v744 = v743 / v506;
                let v2622 = v2358 * v744;
                let v745 = v2 - v744;
                let v746 = v745.powf(v733);
                let v2630 = v2590 * v746;
                let v749 = v2 - v692;
                let v751 = v749.powf((-v565));
                let v753 = (v653 - v743) + v730;
                let v2637 = (v2611 - v2621) + (Lanes([v2598[0], 0.0, 0.0]));
                let v754 = v751 * v753;
                let v755 = v105 * v565;
                let v757 = v506 * v749;
                let v758 = (v755 * v753) / v757;
                let v2641 = (v2358 * v749) * v758;
                let v759 = v2 + v758;
                let v762 = (((v691 * v746) / v733) + (v754 * v759)) - ((v691 * v734) / v733);
                let v2650 = ((((Lanes([v2630[0], 0.0, 0.0])) + (((((v2621 - (Lanes([v2622[0], 0.0, 0.0]))) / v506) * v2085) * (v733 * (v745.powf(v2603)))) * v691)) / v733) + (((v2637 * v751) * v759) + ((((v2637 * v755) - (Lanes([v2641[0], 0.0, 0.0]))) / v757) * v754))) - (Lanes([v2610[0], 0.0, 0.0]));
                v957 = v762;
                v2014 = v2650;
            }
            let v763 = -v534;
            let v2684 = v2396 * v2085;
            let v764 = v763 * v692;
            let v2685 = v2684 * v692;
            let v766 = if v765 <= v0 { 1.0 } else { 0.0 };
            let v960: f64;
            let v2017: Lanes<3>;
            if v766 != 0.0 {
                let v767 = v659 + v764;
                let v2850 = Lanes([0.0, v2537[0], v2537[1]]);
                let v2852 = v2850 + (Lanes([v2685[0], 0.0, 0.0]));
                let v768 = if v767 > v0 { 1.0 } else { 0.0 };
                let v810: f64;
                let v812: f64;
                let v2018: Lanes<3>;
                let v2019: Lanes<3>;
                if v768 != 0.0 {
                    let v769 = v2 - v692;
                    let v772 = v769.powf((v770 - v570));
                    let v775 = v2 - ((v772 * v769) * v769);
                    let v777 = v2 - v570;
                    let v778 = (v534 * v775) / v777;
                    let v2894 = (v2396 * v775) / v777;
                    let v779 = v105 * v570;
                    let v781 = (v779 * v767) / v534;
                    let v2896 = v2396 * v781;
                    let v782 = v769 + v781;
                    let v784 = (v767 * v782) * v772;
                    let v2903 = ((v2852 * v782) + ((((v2852 * v779) - (Lanes([v2896[0], 0.0, 0.0]))) / v534) * v767)) * v772;
                    let v2904 = Lanes([v2894[0], 0.0, 0.0]);
                    v810 = v778;
                    v812 = v784;
                    v2018 = v2904;
                    v2019 = v2903;
                } else {
                    let v789 = if (if v785 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v659 < (-v785) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v811: f64;
                    let v2020: Lanes<3>;
                    if v789 != 0.0 {
                        let v790 = v785 / v534;
                        let v791 = v2 + v790;
                        let v792 = v2 - v570;
                        let v793 = v791.powf(v792);
                        let v2875 = v2537 * v792;
                        let v796 = v534 + v785;
                        let v797 = (v792 * (v659 + v785)) / v796;
                        let v2876 = v2396 * v797;
                        let v798 = v2 - v797;
                        let v2882 = ((((v2396 * v790) * v2085) / v534) * (v792 * (v791.powf((v792 - v1980))))) * v798;
                        let v800 = v2 - (v793 * v798);
                        let v2887 = v2396 * v800;
                        let v802 = (v534 * v800) / v792;
                        let v2891 = ((Lanes([v2887[0], 0.0, 0.0])) + ((((Lanes([v2882[0], 0.0, 0.0])) + (((((Lanes([0.0, v2875[0], v2875[1]])) - (Lanes([v2876[0], 0.0, 0.0]))) / v796) * v2085) * v793)) * v2085) * v534)) / v792;
                        v811 = v802;
                        v2020 = v2891;
                    } else {
                        let v803 = v659 / v534;
                        let v2853 = v2396 * v803;
                        let v804 = v2 - v803;
                        let v805 = v2 - v570;
                        let v807 = v2 - (v804.powf(v805));
                        let v2863 = v2396 * v807;
                        let v809 = (v534 * v807) / v805;
                        let v2867 = ((Lanes([v2863[0], 0.0, 0.0])) + ((((((v2850 - (Lanes([v2853[0], 0.0, 0.0]))) / v534) * v2085) * (v805 * (v804.powf((v805 - v1980))))) * v2085) * v534)) / v805;
                        v811 = v809;
                        v2020 = v2867;
                    }
                    v810 = v811;
                    v812 = v0;
                    v2018 = v2020;
                    v2019 = v2892;
                }
                let v813 = v810 + v812;
                let v2905 = v2018 + v2019;
                v960 = v813;
                v2017 = v2905;
            } else {
                let v817 = if (if v785 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v815 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v961: f64;
                let v2021: Lanes<3>;
                if v817 != 0.0 {
                    let v819 = v785 - v764;
                    let v2736 = v2685 * v2085;
                    let v820 = (v785 + v764) / v819;
                    let v2739 = (v2685 - (v2736 * v820)) / v819;
                    let v822 = v820 - v2;
                    let v2741 = v2739 * v822;
                    let v825 = (v107 * v765) * v765;
                    let v827 = ((v822 * v822) + v825).sqrt();
                    let v828 = v820 + v2;
                    let v2746 = v2739 * v828;
                    let v831 = (v107 * v815) * v815;
                    let v833 = ((v828 * v828) + v831).sqrt();
                    let v834 = v827 + v833;
                    let v835 = (v470 * v820) / v834;
                    let v839 = v105 * (((v835 * v819) - v785) - v764);
                    let v2759 = ((((((v2739 * v470) - ((((v2741 + v2741) * (v1980 / (v2348 * v827))) + ((v2746 + v2746) * (v1980 / (v2348 * v833)))) * v835)) / v834) * v819) + (v2736 * v835)) - v2685) * v105;
                    let v840 = v839 / v534;
                    let v841 = v2 - v840;
                    let v842 = v2 - v570;
                    let v2764 = v842 - v1980;
                    let v844 = v2 - (v841.powf(v842));
                    let v2772 = ((v2396 * v844) + ((((((v2759 - (v2396 * v840)) / v534) * v2085) * (v842 * (v841.powf(v2764)))) * v2085) * v534)) / v842;
                    let v2773 = v2537 * v470;
                    let v2775 = Lanes([v2685[0], 0.0, 0.0]);
                    let v850 = (((v470 * v659) + v785) + v764) / v819;
                    let v2777 = v2736 * v850;
                    let v2780 = (((Lanes([0.0, v2773[0], v2773[1]])) + v2775) - (Lanes([v2777[0], 0.0, 0.0]))) / v819;
                    let v852 = v850 - v2;
                    let v2782 = v2780 * v852;
                    let v855 = ((v852 * v852) + v825).sqrt();
                    let v856 = v850 + v2;
                    let v2787 = v2780 * v856;
                    let v859 = ((v856 * v856) + v831).sqrt();
                    let v860 = v855 + v859;
                    let v861 = (v470 * v850) / v860;
                    let v2795 = ((v2780 * v470) - ((((v2782 + v2782) * (v1980 / (v2348 * v855))) + ((v2787 + v2787) * (v1980 / (v2348 * v859)))) * v861)) / v860;
                    let v2797 = v2736 * v861;
                    let v865 = v105 * (((v861 * v819) - v785) - v764);
                    let v2801 = (((v2795 * v819) + (Lanes([v2797[0], 0.0, 0.0]))) - v2775) * v105;
                    let v866 = v865 / v534;
                    let v2802 = v2396 * v866;
                    let v867 = v2 - v866;
                    let v869 = v2 - (v867.powf(v842));
                    let v2811 = v2396 * v869;
                    let v873 = v105 * (v861 + v2);
                    let v2816 = v2795 * v105;
                    let v874 = v785 / v534;
                    let v875 = v2 + v874;
                    let v876 = -v570;
                    let v877 = v875.powf(v876);
                    let v2820 = v876 - v1980;
                    let v878 = v764 / v534;
                    let v879 = v2 + v878;
                    let v880 = v879.powf(v876);
                    let v881 = v2 - v873;
                    let v2832 = ((((v2396 * v874) * v2085) / v534) * (v876 * (v875.powf(v2820)))) * v881;
                    let v2836 = (((v2685 - (v2396 * v878)) / v534) * (v876 * (v879.powf(v2820)))) * v873;
                    let v884 = (v881 * v877) + (v873 * v880);
                    let v886 = (v659 - v865) + v839;
                    let v889 = ((v886 * v884) + ((v534 * v869) / v842)) - ((v534 * v844) / v842);
                    let v2849 = ((((((Lanes([0.0, v2537[0], v2537[1]])) - v2801) + (Lanes([v2759[0], 0.0, 0.0]))) * v884) + (((((v2816 * v2085) * v877) + (Lanes([v2832[0], 0.0, 0.0]))) + ((v2816 * v880) + (Lanes([v2836[0], 0.0, 0.0])))) * v886)) + (((Lanes([v2811[0], 0.0, 0.0])) + ((((((v2801 - (Lanes([v2802[0], 0.0, 0.0]))) / v534) * v2085) * (v842 * (v867.powf(v2764)))) * v2085) * v534)) / v842)) - (Lanes([v2772[0], 0.0, 0.0]));
                    v961 = v889;
                    v2021 = v2849;
                } else {
                    let v2686 = v2685 * v764;
                    let v892 = (v107 * v765) * v765;
                    let v894 = ((v764 * v764) + v892).sqrt();
                    let v897 = v895 * (v764 + v894);
                    let v2692 = (v2685 + ((v2686 + v2686) * (v1980 / (v2348 * v894)))) * v895;
                    let v898 = v897 / v534;
                    let v899 = v2 - v898;
                    let v900 = v2 - v570;
                    let v901 = v899.powf(v900);
                    let v2697 = v900 - v1980;
                    let v2704 = ((v2684 * v901) + (((((v2692 - (v2396 * v898)) / v534) * v2085) * (v900 * (v899.powf(v2697)))) * v763)) / v900;
                    let v904 = v659 + v764;
                    let v2705 = Lanes([0.0, v2537[0], v2537[1]]);
                    let v2706 = Lanes([v2685[0], 0.0, 0.0]);
                    let v2707 = v2705 + v2706;
                    let v2708 = v2707 * v904;
                    let v907 = ((v904 * v904) + v892).sqrt();
                    let v910 = (v105 * (v904 - v907)) - v764;
                    let v2715 = ((v2707 - ((v2708 + v2708) * (v1980 / (v2348 * v907)))) * v105) - v2706;
                    let v911 = v910 / v534;
                    let v2716 = v2396 * v911;
                    let v912 = v2 - v911;
                    let v913 = v912.powf(v900);
                    let v2724 = v2684 * v913;
                    let v918 = (v2 - v692).powf((-v570));
                    let v923 = (((v763 * v913) / v900) + (v918 * ((v659 - v910) + v897))) - ((v763 * v901) / v900);
                    let v2735 = ((((Lanes([v2724[0], 0.0, 0.0])) + (((((v2715 - (Lanes([v2716[0], 0.0, 0.0]))) / v534) * v2085) * (v900 * (v912.powf(v2697)))) * v763)) / v900) + (((v2705 - v2715) + (Lanes([v2692[0], 0.0, 0.0]))) * v918)) - (Lanes([v2704[0], 0.0, 0.0]));
                    v961 = v923;
                    v2021 = v2735;
                }
                v960 = v961;
                v2017 = v2021;
            }
            let v924 = v447 * v320;
            let v925 = v2 / v924;
            let v2911 = ((((v2296 * v320) + (v2091 * v447)) * v925) * v2085) / v924;
            let v928 = if v653 < v926 { 1.0 } else { 0.0 };
            let v937: f64;
            let v2022: Lanes<3>;
            if v928 != 0.0 {
                let v2923 = v2529 * v925;
                let v2924 = v2911 * v653;
                let v930 = (v653 * v925).exp();
                let v2928 = ((Lanes([0.0, v2923[0], v2923[1]])) + (Lanes([v2924[0], 0.0, 0.0]))) * v930;
                v937 = v930;
                v2022 = v2928;
            } else {
                let v932 = (v926 * v925).exp();
                let v933 = v653 - v926;
                let v2914 = v2529 * v925;
                let v2915 = v2911 * v933;
                let v935 = v2 + (v933 * v925);
                let v936 = v932 * v935;
                let v2919 = ((v2911 * v926) * v932) * v935;
                let v2922 = (Lanes([v2919[0], 0.0, 0.0])) + (((Lanes([0.0, v2914[0], v2914[1]])) + (Lanes([v2915[0], 0.0, 0.0]))) * v932);
                v937 = v936;
                v2022 = v2922;
            }
            let v938 = v937 - v2;
            let v939 = v383 * v938;
            let v2929 = v2174 * v938;
            let v2932 = (Lanes([v2929[0], 0.0, 0.0])) + (v2022 * v383);
            let v940 = v448 * v320;
            let v941 = v2 / v940;
            let v2938 = ((((v2297 * v320) + (v2091 * v448)) * v941) * v2085) / v940;
            let v944 = if v659 < v942 { 1.0 } else { 0.0 };
            let v954: f64;
            let v2023: Lanes<3>;
            if v944 != 0.0 {
                let v2950 = v2537 * v941;
                let v2951 = v2938 * v659;
                let v946 = (v659 * v941).exp();
                let v2955 = ((Lanes([0.0, v2950[0], v2950[1]])) + (Lanes([v2951[0], 0.0, 0.0]))) * v946;
                v954 = v946;
                v2023 = v2955;
            } else {
                let v948 = (v942 * v941).exp();
                let v949 = v659 - v942;
                let v2941 = v2537 * v941;
                let v2942 = v2938 * v949;
                let v951 = v2 + (v949 * v941);
                let v952 = v948 * v951;
                let v2946 = ((v2938 * v942) * v948) * v951;
                let v2949 = (Lanes([v2946[0], 0.0, 0.0])) + (((Lanes([0.0, v2941[0], v2941[1]])) + (Lanes([v2942[0], 0.0, 0.0]))) * v948);
                v954 = v952;
                v2023 = v2949;
            }
            let v953 = v383 * v390;
            let v955 = v954 - v2;
            let v956 = v953 * v955;
            let v2959 = ((v2174 * v390) + (v2188 * v383)) * v955;
            let v2962 = (Lanes([v2959[0], 0.0, 0.0])) + (v2023 * v953);
            let v2964 = v2011 * v957;
            let v2966 = (v2014 * v640) + (Lanes([v2964[0], 0.0, 0.0]));
            let v2968 = v2010 * v960;
            let v2970 = (v2017 * v637) + (Lanes([v2968[0], 0.0, 0.0]));
            let v2973 = (Lanes([v2966[0], 0.0, v2966[1], v2966[2]])) + (Lanes([v2970[0], v2970[1], v2970[2], 0.0]));
            let v965 = ((v2 + (v957 * v640)) + (v960 * v637)) - v964;
            let v2974 = v2973 * v965;
            let v969 = ((v965 * v965) + v967).sqrt();
            let v2980 = (((v2974 + v2974) * (v1980 / (v2348 * v969))) + v2973) * v105;
            let v972 = (v105 * (v969 + v965)) + v964;
            let v2982 = v2012 * v939;
            let v2984 = (v2932 * v643) + (Lanes([v2982[0], 0.0, 0.0]));
            let v2985 = v2962 * v22;
            let v975 = (v939 * v643) + (v956 * v22);
            let v2988 = (Lanes([v2984[0], 0.0, v2984[1], v2984[2]])) + (Lanes([v2985[0], v2985[1], v2985[2], 0.0]));
            let v977 = if v976 < v105 { 1.0 } else { 0.0 };
            let v1000: f64;
            let v2024: Lanes<4>;
            if v977 != 0.0 {
                let v978 = v2 / v109;
                let v981 = (v972.powf(v978)) + (v107 * v975);
                let v3005 = (v2980 * (v978 * (v972.powf((v978 - v1980))))) + (v2988 * v107);
                let v982 = if v981 > v967 { 1.0 } else { 0.0 };
                let v1001: f64;
                let v2025: Lanes<4>;
                if v982 != 0.0 {
                    let v985 = v105 * (v972 + (v981.powf(v109)));
                    let v3012 = (v2980 + (v3005 * (v109 * (v981.powf((v109 - v1980)))))) * v105;
                    v1001 = v985;
                    v2025 = v3012;
                } else {
                    let v988 = v105 * (v972 + (v967.powf(v109)));
                    let v3006 = v2980 * v105;
                    v1001 = v988;
                    v2025 = v3006;
                }
                v1000 = v1001;
                v2024 = v2025;
            } else {
                let v2989 = v2988 * v107;
                let v990 = v2 + (v107 * v975);
                let v991 = if v990 > v967 { 1.0 } else { 0.0 };
                let v1002: f64;
                let v2026: Lanes<4>;
                if v991 != 0.0 {
                    let v992 = v105 * v972;
                    let v994 = v2 + (v990.powf(v109));
                    let v995 = v992 * v994;
                    let v2999 = ((v2980 * v105) * v994) + ((v2989 * (v109 * (v990.powf((v109 - v1980))))) * v992);
                    v1002 = v995;
                    v2026 = v2999;
                } else {
                    let v998 = v2 + (v967.powf(v109));
                    let v999 = (v105 * v972) * v998;
                    let v2991 = (v2980 * v105) * v998;
                    v1002 = v999;
                    v2026 = v2991;
                }
                v1000 = v1002;
                v2024 = v2026;
            }
            let v1003 = v956 / v1000;
            let v3016 = ((Lanes([v2962[0], v2962[1], v2962[2], 0.0])) - (v2024 * v1003)) / v1000;
            let v1004 = v939 / v1000;
            let v3020 = ((Lanes([v2932[0], 0.0, v2932[1], v2932[2]])) - (v2024 * v1004)) / v1000;
            let v1005 = if v157 > v0 { 1.0 } else { 0.0 };
            let v1372: f64;
            let v1524: f64;
            let v1912: f64;
            let v2027: Lanes<5>;
            let v2028: Lanes<6>;
            let v2029: Lanes<5>;
            if v1005 != 0.0 {
                let v1006 = v2 / v394;
                let v3025 = ((v2195 * v1006) * v2085) / v394;
                let v1009 = if v667 < v1007 { 1.0 } else { 0.0 };
                let v1028: f64;
                let v2030: Lanes<3>;
                if v1009 != 0.0 {
                    let v3037 = v2549 * v1006;
                    let v3038 = v3025 * v667;
                    let v1011 = (v667 * v1006).exp();
                    let v3042 = ((Lanes([0.0, v3037[0], v3037[1]])) + (Lanes([v3038[0], 0.0, 0.0]))) * v1011;
                    v1028 = v1011;
                    v2030 = v3042;
                } else {
                    let v1013 = (v1007 * v1006).exp();
                    let v1014 = v667 - v1007;
                    let v3028 = v2549 * v1006;
                    let v3029 = v3025 * v1014;
                    let v1016 = v2 + (v1014 * v1006);
                    let v1017 = v1013 * v1016;
                    let v3033 = ((v3025 * v1007) * v1013) * v1016;
                    let v3036 = (Lanes([v3033[0], 0.0, 0.0])) + (((Lanes([0.0, v3028[0], v3028[1]])) + (Lanes([v3029[0], 0.0, 0.0]))) * v1013);
                    v1028 = v1017;
                    v2030 = v3036;
                }
                let v1018 = if v659 < v1007 { 1.0 } else { 0.0 };
                let v1031: f64;
                let v2031: Lanes<3>;
                if v1018 != 0.0 {
                    let v3054 = v2537 * v1006;
                    let v3055 = v3025 * v659;
                    let v1020 = (v659 * v1006).exp();
                    let v3059 = ((Lanes([0.0, v3054[0], v3054[1]])) + (Lanes([v3055[0], 0.0, 0.0]))) * v1020;
                    v1031 = v1020;
                    v2031 = v3059;
                } else {
                    let v1022 = (v1007 * v1006).exp();
                    let v1023 = v659 - v1007;
                    let v3045 = v2537 * v1006;
                    let v3046 = v3025 * v1023;
                    let v1025 = v2 + (v1023 * v1006);
                    let v1026 = v1022 * v1025;
                    let v3050 = ((v3025 * v1007) * v1022) * v1025;
                    let v3053 = (Lanes([v3050[0], 0.0, 0.0])) + (((Lanes([0.0, v3045[0], v3045[1]])) + (Lanes([v3046[0], 0.0, 0.0]))) * v1022);
                    v1031 = v1026;
                    v2031 = v3053;
                }
                let v3060 = v2030 * v1027;
                let v1030 = v2 - v1027;
                let v3061 = v2031 * v1030;
                let v1034 = ((v1027 * v1028) + (v1030 * v1031)) - v2;
                let v1035 = v397 * v1034;
                let v3065 = v2202 * v1034;
                let v3068 = (Lanes([v3065[0], 0.0, 0.0, 0.0, 0.0])) + (((Lanes([v3060[0], 0.0, v3060[1], 0.0, v3060[2]])) + (Lanes([v3061[0], v3061[1], 0.0, v3061[2], 0.0]))) * v397);
                let v3070 = (v3068 * v26) * v107;
                let v1038 = v2 + (v107 * (v1035 * v26));
                let v1039 = if v1038 > v967 { 1.0 } else { 0.0 };
                let v1057: f64;
                let v2032: Lanes<5>;
                if v1039 != 0.0 {
                    let v1040 = v1038.sqrt();
                    let v1042 = v105 * (v2 + v1040);
                    let v3074 = (v3070 * (v1980 / (v2348 * v1040))) * v105;
                    v1057 = v1042;
                    v2032 = v3074;
                } else {
                    v1057 = v1043;
                    v2032 = v3021;
                }
                let v1044 = if v684 < v1007 { 1.0 } else { 0.0 };
                let v1053: f64;
                let v2033: Lanes<3>;
                if v1044 != 0.0 {
                    let v3086 = v2582 * v1006;
                    let v3087 = v3025 * v684;
                    let v1046 = (v684 * v1006).exp();
                    let v3091 = ((Lanes([0.0, v3086[0], v3086[1]])) + (Lanes([v3087[0], 0.0, 0.0]))) * v1046;
                    v1053 = v1046;
                    v2033 = v3091;
                } else {
                    let v1048 = (v1007 * v1006).exp();
                    let v1049 = v684 - v1007;
                    let v3077 = v2582 * v1006;
                    let v3078 = v3025 * v1049;
                    let v1051 = v2 + (v1049 * v1006);
                    let v1052 = v1048 * v1051;
                    let v3082 = ((v3025 * v1007) * v1048) * v1051;
                    let v3085 = (Lanes([v3082[0], 0.0, 0.0])) + (((Lanes([0.0, v3077[0], v3077[1]])) + (Lanes([v3078[0], 0.0, 0.0]))) * v1048);
                    v1053 = v1052;
                    v2033 = v3085;
                }
                let v1054 = v1053 - v2;
                let v3092 = v2202 * v1054;
                let v3095 = (Lanes([v3092[0], 0.0, 0.0])) + (v2033 * v397);
                let v1058 = (v1035 - (v397 * v1054)) / v1057;
                let v3099 = v2032 * v1058;
                let v3102 = (((Lanes([v3068[0], v3068[1], v3068[2], v3068[3], v3068[4], 0.0])) - (Lanes([v3095[0], 0.0, 0.0, 0.0, v3095[1], v3095[2]]))) - (Lanes([v3099[0], v3099[1], v3099[2], v3099[3], v3099[4], 0.0]))) / v1057;
                v1372 = v1057;
                v1524 = v1058;
                v1912 = v1035;
                v2027 = v2032;
                v2028 = v3102;
                v2029 = v3068;
            } else {
                v1372 = v2;
                v1524 = v0;
                v1912 = v0;
                v2027 = v3021;
                v2028 = v3022;
                v2029 = v3021;
            }
            let v1060 = if v1059 == v2 { 1.0 } else { 0.0 };
            let v1500: f64;
            let v1510: f64;
            let v2034: Lanes<4>;
            let v2035: Lanes<4>;
            if v1060 != 0.0 {
                let v1061 = v2 / v401;
                let v3348 = ((v2209 * v1061) * v2085) / v401;
                let v1063 = if v653 < v1062 { 1.0 } else { 0.0 };
                let v1089: f64;
                let v2036: Lanes<3>;
                if v1063 != 0.0 {
                    let v3360 = v2529 * v1061;
                    let v3361 = v3348 * v653;
                    let v1065 = (v653 * v1061).exp();
                    let v3365 = ((Lanes([0.0, v3360[0], v3360[1]])) + (Lanes([v3361[0], 0.0, 0.0]))) * v1065;
                    v1089 = v1065;
                    v2036 = v3365;
                } else {
                    let v1067 = (v1062 * v1061).exp();
                    let v1068 = v653 - v1062;
                    let v3351 = v2529 * v1061;
                    let v3352 = v3348 * v1068;
                    let v1070 = v2 + (v1068 * v1061);
                    let v1071 = v1067 * v1070;
                    let v3356 = ((v3348 * v1062) * v1067) * v1070;
                    let v3359 = (Lanes([v3356[0], 0.0, 0.0])) + (((Lanes([0.0, v3351[0], v3351[1]])) + (Lanes([v3352[0], 0.0, 0.0]))) * v1067);
                    v1089 = v1071;
                    v2036 = v3359;
                }
                let v1072 = v2 / v408;
                let v3368 = ((v2223 * v1072) * v2085) / v408;
                let v1074 = if v653 < v1073 { 1.0 } else { 0.0 };
                let v1092: f64;
                let v2037: Lanes<3>;
                if v1074 != 0.0 {
                    let v3380 = v2529 * v1072;
                    let v3381 = v3368 * v653;
                    let v1076 = (v653 * v1072).exp();
                    let v3385 = ((Lanes([0.0, v3380[0], v3380[1]])) + (Lanes([v3381[0], 0.0, 0.0]))) * v1076;
                    v1092 = v1076;
                    v2037 = v3385;
                } else {
                    let v1078 = (v1073 * v1072).exp();
                    let v1079 = v653 - v1073;
                    let v3371 = v2529 * v1072;
                    let v3372 = v3368 * v1079;
                    let v1081 = v2 + (v1079 * v1072);
                    let v1082 = v1078 * v1081;
                    let v3376 = ((v3368 * v1073) * v1078) * v1081;
                    let v3379 = (Lanes([v3376[0], 0.0, 0.0])) + (((Lanes([0.0, v3371[0], v3371[1]])) + (Lanes([v3372[0], 0.0, 0.0]))) * v1078);
                    v1092 = v1082;
                    v2037 = v3379;
                }
                let v1084 = if v1083 > v0 { 1.0 } else { 0.0 };
                let v1114: f64;
                let v2038: Lanes<4>;
                if v1084 != 0.0 {
                    let v1087 = v2 + (v1083 * (v972 - v2));
                    let v1088 = v404 * v1087;
                    let v3397 = v2216 * v1087;
                    let v1090 = v1089 - v2;
                    let v3402 = v2036 * v1088;
                    let v1093 = v1092 - v2;
                    let v3405 = v2230 * v1093;
                    let v3408 = (Lanes([v3405[0], 0.0, 0.0])) + (v2037 * v411);
                    let v1095 = (v1088 * v1090) + (v411 * v1093);
                    let v3410 = ((((Lanes([v3397[0], 0.0, 0.0, 0.0])) + ((v2980 * v1083) * v404)) * v1090) + (Lanes([v3402[0], 0.0, v3402[1], v3402[2]]))) + (Lanes([v3408[0], 0.0, v3408[1], v3408[2]]));
                    v1114 = v1095;
                    v2038 = v3410;
                } else {
                    let v1096 = v1089 - v2;
                    let v3386 = v2216 * v1096;
                    let v1098 = v1092 - v2;
                    let v3390 = v2230 * v1098;
                    let v1100 = (v404 * v1096) + (v411 * v1098);
                    let v3394 = ((Lanes([v3386[0], 0.0, 0.0])) + (v2036 * v404)) + ((Lanes([v3390[0], 0.0, 0.0])) + (v2037 * v411));
                    let v3395 = Lanes([v3394[0], 0.0, v3394[1], v3394[2]]);
                    v1114 = v1100;
                    v2038 = v3395;
                }
                let v1101 = if v78 > v0 { 1.0 } else { 0.0 };
                let v1501: f64;
                let v2039: Lanes<4>;
                if v1101 != 0.0 {
                    let v1102 = v590 - v653;
                    let v3413 = (Lanes([v2477[0], 0.0, 0.0])) - (Lanes([0.0, v2529[0], v2529[1]]));
                    let v1103 = v2 / v591;
                    let v3416 = ((v2480 * v1103) * v2085) / v591;
                    let v1105 = if v1102 < v1104 { 1.0 } else { 0.0 };
                    let v1115: f64;
                    let v2040: Lanes<3>;
                    if v1105 != 0.0 {
                        let v3428 = v3416 * v1102;
                        let v1107 = (v1102 * v1103).exp();
                        let v3431 = ((v3413 * v1103) + (Lanes([v3428[0], 0.0, 0.0]))) * v1107;
                        v1115 = v1107;
                        v2040 = v3431;
                    } else {
                        let v1109 = (v1104 * v1103).exp();
                        let v1110 = v1102 - v1104;
                        let v3420 = v3416 * v1110;
                        let v1112 = v2 + (v1110 * v1103);
                        let v1113 = v1109 * v1112;
                        let v3423 = ((v3416 * v1104) * v1109) * v1112;
                        let v3426 = (Lanes([v3423[0], 0.0, 0.0])) + (((v3413 * v1103) + (Lanes([v3420[0], 0.0, 0.0]))) * v1109);
                        v1115 = v1113;
                        v2040 = v3426;
                    }
                    let v3434 = (v2040 - (Lanes([v2484[0], 0.0, 0.0]))) * v74;
                    let v1118 = v1114 - (v74 * (v1115 - v593));
                    let v3436 = v2038 - (Lanes([v3434[0], 0.0, v3434[1], v3434[2]]));
                    v1501 = v1118;
                    v2039 = v3436;
                } else {
                    v1501 = v1114;
                    v2039 = v2038;
                }
                v1500 = v1501;
                v1510 = v0;
                v2034 = v2039;
                v2035 = v3437;
            } else {
                let v1119 = if v1059 == v0 { 1.0 } else { 0.0 };
                let v1502: f64;
                let v1511: f64;
                let v2041: Lanes<4>;
                let v2042: Lanes<4>;
                if v1119 != 0.0 {
                    let v1120 = v2 / v401;
                    let v3270 = ((v2209 * v1120) * v2085) / v401;
                    let v1121 = if v656 < v1062 { 1.0 } else { 0.0 };
                    let v1140: f64;
                    let v2043: Lanes<3>;
                    if v1121 != 0.0 {
                        let v3282 = v2533 * v1120;
                        let v3283 = v3270 * v656;
                        let v1123 = (v656 * v1120).exp();
                        let v3287 = ((Lanes([0.0, v3282[0], v3282[1]])) + (Lanes([v3283[0], 0.0, 0.0]))) * v1123;
                        v1140 = v1123;
                        v2043 = v3287;
                    } else {
                        let v1125 = (v1062 * v1120).exp();
                        let v1126 = v656 - v1062;
                        let v3273 = v2533 * v1120;
                        let v3274 = v3270 * v1126;
                        let v1128 = v2 + (v1126 * v1120);
                        let v1129 = v1125 * v1128;
                        let v3278 = ((v3270 * v1062) * v1125) * v1128;
                        let v3281 = (Lanes([v3278[0], 0.0, 0.0])) + (((Lanes([0.0, v3273[0], v3273[1]])) + (Lanes([v3274[0], 0.0, 0.0]))) * v1125);
                        v1140 = v1129;
                        v2043 = v3281;
                    }
                    let v1130 = v2 / v408;
                    let v3290 = ((v2223 * v1130) * v2085) / v408;
                    let v1131 = if v656 < v1073 { 1.0 } else { 0.0 };
                    let v1143: f64;
                    let v2044: Lanes<3>;
                    if v1131 != 0.0 {
                        let v3302 = v2533 * v1130;
                        let v3303 = v3290 * v656;
                        let v1133 = (v656 * v1130).exp();
                        let v3307 = ((Lanes([0.0, v3302[0], v3302[1]])) + (Lanes([v3303[0], 0.0, 0.0]))) * v1133;
                        v1143 = v1133;
                        v2044 = v3307;
                    } else {
                        let v1135 = (v1073 * v1130).exp();
                        let v1136 = v656 - v1073;
                        let v3293 = v2533 * v1130;
                        let v3294 = v3290 * v1136;
                        let v1138 = v2 + (v1136 * v1130);
                        let v1139 = v1135 * v1138;
                        let v3298 = ((v3290 * v1073) * v1135) * v1138;
                        let v3301 = (Lanes([v3298[0], 0.0, 0.0])) + (((Lanes([0.0, v3293[0], v3293[1]])) + (Lanes([v3294[0], 0.0, 0.0]))) * v1135);
                        v1143 = v1139;
                        v2044 = v3301;
                    }
                    let v1141 = v1140 - v2;
                    let v3308 = v2216 * v1141;
                    let v1144 = v1143 - v2;
                    let v3312 = v2230 * v1144;
                    let v1146 = (v404 * v1141) + (v411 * v1144);
                    let v3316 = ((Lanes([v3308[0], 0.0, 0.0])) + (v2043 * v404)) + ((Lanes([v3312[0], 0.0, 0.0])) + (v2044 * v411));
                    let v1147 = if v78 > v0 { 1.0 } else { 0.0 };
                    let v1512: f64;
                    let v2045: Lanes<4>;
                    if v1147 != 0.0 {
                        let v1148 = v590 - v653;
                        let v3320 = (Lanes([v2477[0], 0.0, 0.0])) - (Lanes([0.0, v2529[0], v2529[1]]));
                        let v1149 = v2 / v591;
                        let v3323 = ((v2480 * v1149) * v2085) / v591;
                        let v1150 = if v1148 < v1104 { 1.0 } else { 0.0 };
                        let v1159: f64;
                        let v2046: Lanes<3>;
                        if v1150 != 0.0 {
                            let v3335 = v3323 * v1148;
                            let v1152 = (v1148 * v1149).exp();
                            let v3338 = ((v3320 * v1149) + (Lanes([v3335[0], 0.0, 0.0]))) * v1152;
                            v1159 = v1152;
                            v2046 = v3338;
                        } else {
                            let v1154 = (v1104 * v1149).exp();
                            let v1155 = v1148 - v1104;
                            let v3327 = v3323 * v1155;
                            let v1157 = v2 + (v1155 * v1149);
                            let v1158 = v1154 * v1157;
                            let v3330 = ((v3323 * v1104) * v1154) * v1157;
                            let v3333 = (Lanes([v3330[0], 0.0, 0.0])) + (((v3320 * v1149) + (Lanes([v3327[0], 0.0, 0.0]))) * v1154);
                            v1159 = v1158;
                            v2046 = v3333;
                        }
                        let v3341 = (v2046 - (Lanes([v2484[0], 0.0, 0.0]))) * v74;
                        let v1162 = v1146 - (v74 * (v1159 - v593));
                        let v3344 = (Lanes([v3316[0], v3316[1], 0.0, v3316[2]])) - (Lanes([v3341[0], 0.0, v3341[1], v3341[2]]));
                        v1512 = v1162;
                        v2045 = v3344;
                    } else {
                        let v3317 = Lanes([v3316[0], v3316[1], 0.0, v3316[2]]);
                        v1512 = v1146;
                        v2045 = v3317;
                    }
                    v1502 = v0;
                    v1511 = v1512;
                    v2041 = v3345;
                    v2042 = v2045;
                } else {
                    let v1163 = v2 / v401;
                    let v3105 = ((v2209 * v1163) * v2085) / v401;
                    let v1164 = if v653 < v1062 { 1.0 } else { 0.0 };
                    let v1188: f64;
                    let v2047: Lanes<3>;
                    if v1164 != 0.0 {
                        let v3117 = v2529 * v1163;
                        let v3118 = v3105 * v653;
                        let v1166 = (v653 * v1163).exp();
                        let v3122 = ((Lanes([0.0, v3117[0], v3117[1]])) + (Lanes([v3118[0], 0.0, 0.0]))) * v1166;
                        v1188 = v1166;
                        v2047 = v3122;
                    } else {
                        let v1168 = (v1062 * v1163).exp();
                        let v1169 = v653 - v1062;
                        let v3108 = v2529 * v1163;
                        let v3109 = v3105 * v1169;
                        let v1171 = v2 + (v1169 * v1163);
                        let v1172 = v1168 * v1171;
                        let v3113 = ((v3105 * v1062) * v1168) * v1171;
                        let v3116 = (Lanes([v3113[0], 0.0, 0.0])) + (((Lanes([0.0, v3108[0], v3108[1]])) + (Lanes([v3109[0], 0.0, 0.0]))) * v1168);
                        v1188 = v1172;
                        v2047 = v3116;
                    }
                    let v1173 = v2 / v408;
                    let v3125 = ((v2223 * v1173) * v2085) / v408;
                    let v1174 = if v653 < v1073 { 1.0 } else { 0.0 };
                    let v1191: f64;
                    let v2048: Lanes<3>;
                    if v1174 != 0.0 {
                        let v3137 = v2529 * v1173;
                        let v3138 = v3125 * v653;
                        let v1176 = (v653 * v1173).exp();
                        let v3142 = ((Lanes([0.0, v3137[0], v3137[1]])) + (Lanes([v3138[0], 0.0, 0.0]))) * v1176;
                        v1191 = v1176;
                        v2048 = v3142;
                    } else {
                        let v1178 = (v1073 * v1173).exp();
                        let v1179 = v653 - v1073;
                        let v3128 = v2529 * v1173;
                        let v3129 = v3125 * v1179;
                        let v1181 = v2 + (v1179 * v1173);
                        let v1182 = v1178 * v1181;
                        let v3133 = ((v3125 * v1073) * v1178) * v1181;
                        let v3136 = (Lanes([v3133[0], 0.0, 0.0])) + (((Lanes([0.0, v3128[0], v3128[1]])) + (Lanes([v3129[0], 0.0, 0.0]))) * v1178);
                        v1191 = v1182;
                        v2048 = v3136;
                    }
                    let v1183 = if v1083 > v0 { 1.0 } else { 0.0 };
                    let v1214: f64;
                    let v2049: Lanes<4>;
                    if v1183 != 0.0 {
                        let v1186 = v2 + (v1083 * (v972 - v2));
                        let v1187 = v404 * v1186;
                        let v3155 = v2216 * v1186;
                        let v1189 = v1188 - v2;
                        let v3160 = v2047 * v1187;
                        let v1192 = v1191 - v2;
                        let v3163 = v2230 * v1192;
                        let v3166 = (Lanes([v3163[0], 0.0, 0.0])) + (v2048 * v411);
                        let v1195 = v1059 * ((v1187 * v1189) + (v411 * v1192));
                        let v3169 = (((((Lanes([v3155[0], 0.0, 0.0, 0.0])) + ((v2980 * v1083) * v404)) * v1189) + (Lanes([v3160[0], 0.0, v3160[1], v3160[2]]))) + (Lanes([v3166[0], 0.0, v3166[1], v3166[2]]))) * v1059;
                        v1214 = v1195;
                        v2049 = v3169;
                    } else {
                        let v1196 = v1188 - v2;
                        let v3143 = v2216 * v1196;
                        let v1198 = v1191 - v2;
                        let v3147 = v2230 * v1198;
                        let v1201 = v1059 * ((v404 * v1196) + (v411 * v1198));
                        let v3152 = (((Lanes([v3143[0], 0.0, 0.0])) + (v2047 * v404)) + ((Lanes([v3147[0], 0.0, 0.0])) + (v2048 * v411))) * v1059;
                        let v3153 = Lanes([v3152[0], 0.0, v3152[1], v3152[2]]);
                        v1214 = v1201;
                        v2049 = v3153;
                    }
                    let v1202 = if v78 > v0 { 1.0 } else { 0.0 };
                    let v1503: f64;
                    let v2050: Lanes<4>;
                    if v1202 != 0.0 {
                        let v1203 = v590 - v653;
                        let v3172 = (Lanes([v2477[0], 0.0, 0.0])) - (Lanes([0.0, v2529[0], v2529[1]]));
                        let v1204 = v2 / v591;
                        let v3175 = ((v2480 * v1204) * v2085) / v591;
                        let v1205 = if v1203 < v1104 { 1.0 } else { 0.0 };
                        let v1216: f64;
                        let v2051: Lanes<3>;
                        if v1205 != 0.0 {
                            let v3187 = v3175 * v1203;
                            let v1207 = (v1203 * v1204).exp();
                            let v3190 = ((v3172 * v1204) + (Lanes([v3187[0], 0.0, 0.0]))) * v1207;
                            v1216 = v1207;
                            v2051 = v3190;
                        } else {
                            let v1209 = (v1104 * v1204).exp();
                            let v1210 = v1203 - v1104;
                            let v3179 = v3175 * v1210;
                            let v1212 = v2 + (v1210 * v1204);
                            let v1213 = v1209 * v1212;
                            let v3182 = ((v3175 * v1104) * v1209) * v1212;
                            let v3185 = (Lanes([v3182[0], 0.0, 0.0])) + (((v3172 * v1204) + (Lanes([v3179[0], 0.0, 0.0]))) * v1209);
                            v1216 = v1213;
                            v2051 = v3185;
                        }
                        let v1215 = v1059 * v74;
                        let v3193 = (v2051 - (Lanes([v2484[0], 0.0, 0.0]))) * v1215;
                        let v1219 = v1214 - (v1215 * (v1216 - v593));
                        let v3195 = v2049 - (Lanes([v3193[0], 0.0, v3193[1], v3193[2]]));
                        v1503 = v1219;
                        v2050 = v3195;
                    } else {
                        v1503 = v1214;
                        v2050 = v2049;
                    }
                    let v1220 = if v656 < v1062 { 1.0 } else { 0.0 };
                    let v1239: f64;
                    let v2052: Lanes<3>;
                    if v1220 != 0.0 {
                        let v3207 = v2533 * v1163;
                        let v3208 = v3105 * v656;
                        let v1222 = (v656 * v1163).exp();
                        let v3212 = ((Lanes([0.0, v3207[0], v3207[1]])) + (Lanes([v3208[0], 0.0, 0.0]))) * v1222;
                        v1239 = v1222;
                        v2052 = v3212;
                    } else {
                        let v1224 = (v1062 * v1163).exp();
                        let v1225 = v656 - v1062;
                        let v3198 = v2533 * v1163;
                        let v3199 = v3105 * v1225;
                        let v1227 = v2 + (v1225 * v1163);
                        let v1228 = v1224 * v1227;
                        let v3203 = ((v3105 * v1062) * v1224) * v1227;
                        let v3206 = (Lanes([v3203[0], 0.0, 0.0])) + (((Lanes([0.0, v3198[0], v3198[1]])) + (Lanes([v3199[0], 0.0, 0.0]))) * v1224);
                        v1239 = v1228;
                        v2052 = v3206;
                    }
                    let v1229 = if v656 < v1073 { 1.0 } else { 0.0 };
                    let v1242: f64;
                    let v2053: Lanes<3>;
                    if v1229 != 0.0 {
                        let v3224 = v2533 * v1173;
                        let v3225 = v3125 * v656;
                        let v1231 = (v656 * v1173).exp();
                        let v3229 = ((Lanes([0.0, v3224[0], v3224[1]])) + (Lanes([v3225[0], 0.0, 0.0]))) * v1231;
                        v1242 = v1231;
                        v2053 = v3229;
                    } else {
                        let v1233 = (v1073 * v1173).exp();
                        let v1234 = v656 - v1073;
                        let v3215 = v2533 * v1173;
                        let v3216 = v3125 * v1234;
                        let v1236 = v2 + (v1234 * v1173);
                        let v1237 = v1233 * v1236;
                        let v3220 = ((v3125 * v1073) * v1233) * v1236;
                        let v3223 = (Lanes([v3220[0], 0.0, 0.0])) + (((Lanes([0.0, v3215[0], v3215[1]])) + (Lanes([v3216[0], 0.0, 0.0]))) * v1233);
                        v1242 = v1237;
                        v2053 = v3223;
                    }
                    let v1238 = v2 - v1059;
                    let v1240 = v1239 - v2;
                    let v3230 = v2216 * v1240;
                    let v1243 = v1242 - v2;
                    let v3234 = v2230 * v1243;
                    let v1246 = v1238 * ((v404 * v1240) + (v411 * v1243));
                    let v3239 = (((Lanes([v3230[0], 0.0, 0.0])) + (v2052 * v404)) + ((Lanes([v3234[0], 0.0, 0.0])) + (v2053 * v411))) * v1238;
                    let v1513: f64;
                    let v2054: Lanes<4>;
                    if v1202 != 0.0 {
                        let v1247 = v590 - v653;
                        let v3243 = (Lanes([v2477[0], 0.0, 0.0])) - (Lanes([0.0, v2529[0], v2529[1]]));
                        let v1248 = v2 / v591;
                        let v3246 = ((v2480 * v1248) * v2085) / v591;
                        let v1249 = if v1247 < v1104 { 1.0 } else { 0.0 };
                        let v1259: f64;
                        let v2055: Lanes<3>;
                        if v1249 != 0.0 {
                            let v3258 = v3246 * v1247;
                            let v1251 = (v1247 * v1248).exp();
                            let v3261 = ((v3243 * v1248) + (Lanes([v3258[0], 0.0, 0.0]))) * v1251;
                            v1259 = v1251;
                            v2055 = v3261;
                        } else {
                            let v1253 = (v1104 * v1248).exp();
                            let v1254 = v1247 - v1104;
                            let v3250 = v3246 * v1254;
                            let v1256 = v2 + (v1254 * v1248);
                            let v1257 = v1253 * v1256;
                            let v3253 = ((v3246 * v1104) * v1253) * v1256;
                            let v3256 = (Lanes([v3253[0], 0.0, 0.0])) + (((v3243 * v1248) + (Lanes([v3250[0], 0.0, 0.0]))) * v1253);
                            v1259 = v1257;
                            v2055 = v3256;
                        }
                        let v1258 = v1238 * v74;
                        let v3264 = (v2055 - (Lanes([v2484[0], 0.0, 0.0]))) * v1258;
                        let v1262 = v1246 - (v1258 * (v1259 - v593));
                        let v3267 = (Lanes([v3239[0], v3239[1], 0.0, v3239[2]])) - (Lanes([v3264[0], 0.0, v3264[1], v3264[2]]));
                        v1513 = v1262;
                        v2054 = v3267;
                    } else {
                        let v3240 = Lanes([v3239[0], v3239[1], 0.0, v3239[2]]);
                        v1513 = v1246;
                        v2054 = v3240;
                    }
                    v1502 = v1503;
                    v1511 = v1513;
                    v2041 = v2050;
                    v2042 = v2054;
                }
                v1500 = v1502;
                v1510 = v1511;
                v2034 = v2041;
                v2035 = v2042;
            }
            let v1263 = v2 / v415;
            let v3440 = ((v2237 * v1263) * v2085) / v415;
            let v1265 = if v659 < v1264 { 1.0 } else { 0.0 };
            let v1285: f64;
            let v2056: Lanes<3>;
            if v1265 != 0.0 {
                let v3452 = v2537 * v1263;
                let v3453 = v3440 * v659;
                let v1267 = (v659 * v1263).exp();
                let v3457 = ((Lanes([0.0, v3452[0], v3452[1]])) + (Lanes([v3453[0], 0.0, 0.0]))) * v1267;
                v1285 = v1267;
                v2056 = v3457;
            } else {
                let v1269 = (v1264 * v1263).exp();
                let v1270 = v659 - v1264;
                let v3443 = v2537 * v1263;
                let v3444 = v3440 * v1270;
                let v1272 = v2 + (v1270 * v1263);
                let v1273 = v1269 * v1272;
                let v3448 = ((v3440 * v1264) * v1269) * v1272;
                let v3451 = (Lanes([v3448[0], 0.0, 0.0])) + (((Lanes([0.0, v3443[0], v3443[1]])) + (Lanes([v3444[0], 0.0, 0.0]))) * v1269);
                v1285 = v1273;
                v2056 = v3451;
            }
            let v1274 = v2 / v422;
            let v3460 = ((v2251 * v1274) * v2085) / v422;
            let v1276 = if v659 < v1275 { 1.0 } else { 0.0 };
            let v1288: f64;
            let v2057: Lanes<3>;
            if v1276 != 0.0 {
                let v3472 = v2537 * v1274;
                let v3473 = v3460 * v659;
                let v1278 = (v659 * v1274).exp();
                let v3477 = ((Lanes([0.0, v3472[0], v3472[1]])) + (Lanes([v3473[0], 0.0, 0.0]))) * v1278;
                v1288 = v1278;
                v2057 = v3477;
            } else {
                let v1280 = (v1275 * v1274).exp();
                let v1281 = v659 - v1275;
                let v3463 = v2537 * v1274;
                let v3464 = v3460 * v1281;
                let v1283 = v2 + (v1281 * v1274);
                let v1284 = v1280 * v1283;
                let v3468 = ((v3460 * v1275) * v1280) * v1283;
                let v3471 = (Lanes([v3468[0], 0.0, 0.0])) + (((Lanes([0.0, v3463[0], v3463[1]])) + (Lanes([v3464[0], 0.0, 0.0]))) * v1280);
                v1288 = v1284;
                v2057 = v3471;
            }
            let v1286 = v1285 - v2;
            let v3478 = v2244 * v1286;
            let v1289 = v1288 - v2;
            let v3482 = v2258 * v1289;
            let v1291 = (v418 * v1286) + (v425 * v1289);
            let v3486 = ((Lanes([v3478[0], 0.0, 0.0])) + (v2056 * v418)) + ((Lanes([v3482[0], 0.0, 0.0])) + (v2057 * v425));
            let v1294 = if (if v252 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v260 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1516: f64;
            let v2058: Lanes<3>;
            if v1294 != 0.0 {
                let v1296 = if v667 < v1295 { 1.0 } else { 0.0 };
                let v1315: f64;
                let v2059: Lanes<3>;
                if v1296 != 0.0 {
                    let v3499 = v2549 * v1263;
                    let v3500 = v3440 * v667;
                    let v1298 = (v667 * v1263).exp();
                    let v3504 = ((Lanes([0.0, v3499[0], v3499[1]])) + (Lanes([v3500[0], 0.0, 0.0]))) * v1298;
                    v1315 = v1298;
                    v2059 = v3504;
                } else {
                    let v1300 = (v1295 * v1263).exp();
                    let v1301 = v667 - v1295;
                    let v3490 = v2549 * v1263;
                    let v3491 = v3440 * v1301;
                    let v1303 = v2 + (v1301 * v1263);
                    let v1304 = v1300 * v1303;
                    let v3495 = ((v3440 * v1295) * v1300) * v1303;
                    let v3498 = (Lanes([v3495[0], 0.0, 0.0])) + (((Lanes([0.0, v3490[0], v3490[1]])) + (Lanes([v3491[0], 0.0, 0.0]))) * v1300);
                    v1315 = v1304;
                    v2059 = v3498;
                }
                let v1306 = if v667 < v1305 { 1.0 } else { 0.0 };
                let v1318: f64;
                let v2060: Lanes<3>;
                if v1306 != 0.0 {
                    let v3516 = v2549 * v1274;
                    let v3517 = v3460 * v667;
                    let v1308 = (v667 * v1274).exp();
                    let v3521 = ((Lanes([0.0, v3516[0], v3516[1]])) + (Lanes([v3517[0], 0.0, 0.0]))) * v1308;
                    v1318 = v1308;
                    v2060 = v3521;
                } else {
                    let v1310 = (v1305 * v1274).exp();
                    let v1311 = v667 - v1305;
                    let v3507 = v2549 * v1274;
                    let v3508 = v3460 * v1311;
                    let v1313 = v2 + (v1311 * v1274);
                    let v1314 = v1310 * v1313;
                    let v3512 = ((v3460 * v1305) * v1310) * v1313;
                    let v3515 = (Lanes([v3512[0], 0.0, 0.0])) + (((Lanes([0.0, v3507[0], v3507[1]])) + (Lanes([v3508[0], 0.0, 0.0]))) * v1310);
                    v1318 = v1314;
                    v2060 = v3515;
                }
                let v1316 = v1315 - v2;
                let v3522 = v2262 * v1316;
                let v1319 = v1318 - v2;
                let v3526 = v2266 * v1319;
                let v1321 = (v427 * v1316) + (v429 * v1319);
                let v3530 = ((Lanes([v3522[0], 0.0, 0.0])) + (v2059 * v427)) + ((Lanes([v3526[0], 0.0, 0.0])) + (v2060 * v429));
                v1516 = v1321;
                v2058 = v3530;
            } else {
                v1516 = v0;
                v2058 = v3487;
            }
            let v1322 = v659 / v320;
            let v3531 = v2091 * v1322;
            let v3532 = Lanes([0.0, v2537[0], v2537[1]]);
            let v3535 = (v3532 - (Lanes([v3531[0], 0.0, 0.0]))) / v320;
            let v1323 = if v1322 < v18 { 1.0 } else { 0.0 };
            let v1336: f64;
            let v2061: Lanes<3>;
            if v1323 != 0.0 {
                let v1324 = v1322.exp();
                let v3537 = v3535 * v1324;
                v1336 = v1324;
                v2061 = v3537;
            } else {
                let v1325 = v18.exp();
                let v1328 = v1325 * (v2 + (v1322 - v18));
                let v3536 = v3535 * v1325;
                v1336 = v1328;
                v2061 = v3536;
            }
            let v1329 = v662 / v320;
            let v3538 = v2091 * v1329;
            let v3542 = ((Lanes([0.0, v2541[0], v2541[1]])) - (Lanes([v3538[0], 0.0, 0.0]))) / v320;
            let v1330 = if v1329 < v18 { 1.0 } else { 0.0 };
            let v1340: f64;
            let v2062: Lanes<3>;
            if v1330 != 0.0 {
                let v1331 = v1329.exp();
                let v3544 = v3542 * v1331;
                v1340 = v1331;
                v2062 = v3544;
            } else {
                let v1332 = v18.exp();
                let v1335 = v1332 * (v2 + (v1329 - v18));
                let v3543 = v3542 * v1332;
                v1340 = v1335;
                v2062 = v3543;
            }
            let v3545 = v2471 * v1336;
            let v1339 = (v2 + (v585 * v1336)).sqrt();
            let v3551 = ((Lanes([v3545[0], 0.0, 0.0])) + (v2061 * v585)) * (v1980 / (v2348 * v1339));
            let v3552 = v2471 * v1340;
            let v1343 = (v2 + (v585 * v1340)).sqrt();
            let v3558 = ((Lanes([v3552[0], 0.0, 0.0])) + (v2062 * v585)) * (v1980 / (v2348 * v1343));
            let v1344 = v675 * v609;
            let v3559 = v2562 * v609;
            let v3560 = v2002 * v675;
            let v3563 = (Lanes([v3559[0], 0.0, v3559[1]])) + (Lanes([0.0, v3560[0], 0.0]));
            let v1346 = v1343 + v2;
            let v1347 = (v1339 + v2) / v1346;
            let v3564 = v3558 * v1347;
            let v3565 = Lanes([v3551[0], 0.0, v3551[1], v3551[2]]);
            let v1350 = (v1339 - v1343) - (v1347.ln());
            let v3574 = v2091 * v1350;
            let v1352 = v677 + (v320 * v1350);
            let v1353 = v1352 * v613;
            let v3581 = v2003 * v1352;
            let v3583 = (((Lanes([0.0, v2566[0], v2566[1], 0.0])) + ((Lanes([v3574[0], 0.0, 0.0, 0.0])) + (((v3565 - (Lanes([v3558[0], v3558[1], 0.0, v3558[2]]))) - (((v3565 - (Lanes([v3564[0], v3564[1], 0.0, v3564[2]]))) / v1346) * (v1980 / v1347))) * v320))) * v613) + (Lanes([v3581[0], 0.0, 0.0, 0.0]));
            let v3584 = v2013 * v1353;
            let v1356 = (v105 * v646) * v30;
            let v3590 = v2566 * v677;
            let v1359 = ((v677 * v677) + v4).sqrt();
            let v3595 = ((v2013 * v105) * v30) * v1359;
            let v3596 = ((v3590 + v3590) * (v1980 / (v2348 * v1359))) * v1356;
            let v1361 = v2 + (v1356 * v1359);
            let v1362 = v613 * v1361;
            let v3600 = v2003 * v1361;
            let v1363 = (v646 * v1353) / v1362;
            let v3604 = ((Lanes([v3600[0], 0.0, 0.0])) + (((Lanes([v3595[0], 0.0, 0.0])) + (Lanes([0.0, v3596[0], v3596[1]]))) * v613)) * v1363;
            let v3608 = ((((Lanes([v3584[0], 0.0, 0.0, 0.0])) + (v3583 * v646)) - (Lanes([v3604[0], v3604[1], v3604[2], 0.0]))) / v1362) * v1363;
            let v1366 = (v2 + (v1363 * v1363)).sqrt();
            let v1367 = v1353 / v1366;
            let v3615 = (v3583 - (((v3608 + v3608) * (v1980 / (v2348 * v1366))) * v1367)) / v1366;
            let v1368 = v678 * v617;
            let v3616 = v2569 * v617;
            let v3617 = v2004 * v678;
            let v3620 = (Lanes([v3616[0], 0.0, v3616[1]])) + (Lanes([0.0, v3617[0], 0.0]));
            let v1369 = v679 * v1000;
            let v3621 = v2572 * v1000;
            let v3622 = v2024 * v679;
            let v1370 = v1369 * v621;
            let v3627 = v2005 * v1369;
            let v3629 = (((Lanes([0.0, 0.0, v3621[0], v3621[1], 0.0])) + (Lanes([v3622[0], v3622[1], 0.0, v3622[2], v3622[3]]))) * v621) + (Lanes([v3627[0], 0.0, 0.0, 0.0, 0.0]));
            let v1371 = v680 * v624;
            let v3630 = v2575 * v624;
            let v3631 = v2006 * v680;
            let v3634 = (Lanes([v3630[0], 0.0, v3630[1]])) + (Lanes([0.0, v3631[0], 0.0]));
            let v1373 = v681 * v1372;
            let v3635 = v2578 * v1372;
            let v3636 = v2027 * v681;
            let v1374 = v1373 * v628;
            let v3641 = v2007 * v1373;
            let v3643 = (((Lanes([0.0, v3635[0], 0.0, 0.0, 0.0, v3635[1]])) + (Lanes([v3636[0], 0.0, v3636[1], v3636[2], v3636[3], v3636[4]]))) * v628) + (Lanes([v3641[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v1375 = v688 * v631;
            let v3644 = v2589 * v631;
            let v3645 = v2008 * v688;
            let v3648 = (Lanes([v3644[0], 0.0, v3644[1]])) + (Lanes([0.0, v3645[0], 0.0]));
            let v1377 = if v1376 > v0 { 1.0 } else { 0.0 };
            let v1464: f64;
            let v2063: Lanes<5>;
            if v1377 != 0.0 {
                let v1380 = v1378 * (v453 + v2);
                let v1383 = v2 / (v1381 - v570);
                let v1384 = v1380.powf(v1383);
                let v3654 = (v2299 * v1378) * (v1383 * (v1380.powf((v1383 - v1980))));
                let v1386 = (v534 - v659) - v1384;
                let v3657 = Lanes([v3654[0], 0.0, 0.0]);
                let v3658 = ((Lanes([v2396[0], 0.0, 0.0])) - v3532) - v3657;
                let v3659 = v3658 * v1386;
                let v1389 = ((v1386 * v1386) + v4).sqrt();
                let v1392 = (v105 * (v1389 + v1386)) + v1384;
                let v3666 = ((((v3659 + v3659) * (v1980 / (v2348 * v1389))) + v3658) * v105) + v3657;
                let v1393 = -v453;
                let v1394 = v570 - v2;
                let v1395 = v1392.powf(v1394);
                let v1396 = v1393 * v1395;
                let v3672 = (v2299 * v2085) * v1395;
                let v3675 = (Lanes([v3672[0], 0.0, 0.0])) + ((v3666 * (v1394 * (v1392.powf((v1394 - v1980))))) * v1393);
                let v1397 = if v1396 < v18 { 1.0 } else { 0.0 };
                let v1404: f64;
                let v2064: Lanes<3>;
                if v1397 != 0.0 {
                    let v1398 = v1396.exp();
                    let v3677 = v3675 * v1398;
                    v1404 = v1398;
                    v2064 = v3677;
                } else {
                    let v1399 = v18.exp();
                    let v1402 = v1399 * (v2 + (v1396 - v18));
                    let v3676 = v3675 * v1399;
                    v1404 = v1402;
                    v2064 = v3676;
                }
                let v1403 = v1376 * v1392;
                let v1405 = v1403 * v1404;
                let v1407 = (v690 - v1003) - v1291;
                let v1408 = v1407 * v1405;
                let v3688 = (((v3666 * v1376) * v1404) + (v2064 * v1403)) * v1407;
                let v3690 = ((((Lanes([0.0, 0.0, 0.0, 0.0, v1994[0]])) - (Lanes([v3016[0], v3016[1], v3016[2], v3016[3], 0.0]))) - (Lanes([v3486[0], v3486[1], v3486[2], 0.0, 0.0]))) * v1405) + (Lanes([v3688[0], v3688[1], v3688[2], 0.0, 0.0]));
                v1464 = v1408;
                v2063 = v3690;
            } else {
                v1464 = v0;
                v2063 = v3649;
            }
            let v1410 = if v1409 > v0 { 1.0 } else { 0.0 };
            let v1554: f64;
            let v2065: Lanes<4>;
            if v1410 != 0.0 {
                let v1412 = v1378 * (v458 + v2);
                let v1415 = v2 / (v1381 - v1413);
                let v1416 = v1412.powf(v1415);
                let v3696 = (v2301 * v1378) * (v1415 * (v1412.powf((v1415 - v1980))));
                let v3697 = v2545 * v2085;
                let v1418 = (v0 - v664) - v1416;
                let v3699 = Lanes([v3696[0], 0.0, 0.0]);
                let v3700 = (Lanes([0.0, v3697[0], v3697[1]])) - v3699;
                let v3701 = v3700 * v1418;
                let v1421 = ((v1418 * v1418) + v4).sqrt();
                let v1424 = (v105 * (v1421 + v1418)) + v1416;
                let v3708 = ((((v3701 + v3701) * (v1980 / (v2348 * v1421))) + v3700) * v105) + v3699;
                let v1425 = -v458;
                let v1426 = v1413 - v2;
                let v1427 = v1424.powf(v1426);
                let v1428 = v1425 * v1427;
                let v3714 = (v2301 * v2085) * v1427;
                let v3717 = (Lanes([v3714[0], 0.0, 0.0])) + ((v3708 * (v1426 * (v1424.powf((v1426 - v1980))))) * v1425);
                let v1429 = if v1428 < v18 { 1.0 } else { 0.0 };
                let v1436: f64;
                let v2066: Lanes<3>;
                if v1429 != 0.0 {
                    let v1430 = v1428.exp();
                    let v3719 = v3717 * v1430;
                    v1436 = v1430;
                    v2066 = v3719;
                } else {
                    let v1431 = v18.exp();
                    let v1434 = v1431 * (v2 + (v1428 - v18));
                    let v3718 = v3717 * v1431;
                    v1436 = v1434;
                    v2066 = v3718;
                }
                let v1435 = v1409 * v1424;
                let v1437 = v1435 * v1436;
                let v1438 = -v1344;
                let v1439 = v1438 * v1437;
                let v3725 = (v3563 * v2085) * v1437;
                let v3726 = (((v3708 * v1409) * v1436) + (v2066 * v1435)) * v1438;
                let v3729 = (Lanes([v3725[0], v3725[1], v3725[2], 0.0])) + (Lanes([0.0, v3726[0], v3726[1], v3726[2]]));
                v1554 = v1439;
                v2065 = v3729;
            } else {
                v1554 = v0;
                v2065 = v3691;
            }
            let v1444 = if (if v1440 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1442 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1466: f64;
            let v2067: Lanes<4>;
            if v1444 != 0.0 {
                let v1446 = if v1445 > v0 { 1.0 } else { 0.0 };
                let v1458: f64;
                let v2068: Lanes<2>;
                if v1446 != 0.0 {
                    let v3732 = (v2537 / v1445) * v2085;
                    let v1450 = (v2 - (v659 / v1445)) - v1449;
                    let v3733 = v3732 * v1450;
                    let v1453 = ((v1450 * v1450) + v964).sqrt();
                    let v1457 = v1442 * (v1449 + (v105 * (v1450 + v1453)));
                    let v3740 = ((v3732 + ((v3733 + v3733) * (v1980 / (v2348 * v1453)))) * v105) * v1442;
                    v1458 = v1457;
                    v2068 = v3740;
                } else {
                    v1458 = v1442;
                    v2068 = v3730;
                }
                let v1459 = v1004 / v1458;
                let v3741 = v2068 * v1459;
                let v1460 = v1459 - v2;
                let v1463 = v1440 * (v1460.powf(v1461));
                let v3749 = (((v3020 - (Lanes([0.0, v3741[0], v3741[1], 0.0]))) / v1458) * (v1461 * (v1460.powf((v1461 - v1980))))) * v1440;
                v1466 = v1463;
                v2067 = v3749;
            } else {
                v1466 = v0;
                v2067 = v3345;
            }
            let v1467 = (v1291 - v1464) - v1466;
            let v3753 = ((Lanes([v3486[0], v3486[1], v3486[2], 0.0, 0.0])) - v2063) - (Lanes([v2067[0], v2067[1], v2067[2], v2067[3], 0.0]));
            let v1470 = if (if v268 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v285 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1521: f64;
            let v2069: Lanes<3>;
            if v1470 != 0.0 {
                let v1471 = v2 / v433;
                let v3757 = ((v2273 * v1471) * v2085) / v433;
                let v1473 = if v684 < v1472 { 1.0 } else { 0.0 };
                let v1493: f64;
                let v2070: Lanes<3>;
                if v1473 != 0.0 {
                    let v3769 = v2582 * v1471;
                    let v3770 = v3757 * v684;
                    let v1475 = (v684 * v1471).exp();
                    let v3774 = ((Lanes([0.0, v3769[0], v3769[1]])) + (Lanes([v3770[0], 0.0, 0.0]))) * v1475;
                    v1493 = v1475;
                    v2070 = v3774;
                } else {
                    let v1477 = (v1472 * v1471).exp();
                    let v1478 = v684 - v1472;
                    let v3760 = v2582 * v1471;
                    let v3761 = v3757 * v1478;
                    let v1480 = v2 + (v1478 * v1471);
                    let v1481 = v1477 * v1480;
                    let v3765 = ((v3757 * v1472) * v1477) * v1480;
                    let v3768 = (Lanes([v3765[0], 0.0, 0.0])) + (((Lanes([0.0, v3760[0], v3760[1]])) + (Lanes([v3761[0], 0.0, 0.0]))) * v1477);
                    v1493 = v1481;
                    v2070 = v3768;
                }
                let v1482 = v2 / v440;
                let v3777 = ((v2287 * v1482) * v2085) / v440;
                let v1484 = if v684 < v1483 { 1.0 } else { 0.0 };
                let v1496: f64;
                let v2071: Lanes<3>;
                if v1484 != 0.0 {
                    let v3789 = v2582 * v1482;
                    let v3790 = v3777 * v684;
                    let v1486 = (v684 * v1482).exp();
                    let v3794 = ((Lanes([0.0, v3789[0], v3789[1]])) + (Lanes([v3790[0], 0.0, 0.0]))) * v1486;
                    v1496 = v1486;
                    v2071 = v3794;
                } else {
                    let v1488 = (v1483 * v1482).exp();
                    let v1489 = v684 - v1483;
                    let v3780 = v2582 * v1482;
                    let v3781 = v3777 * v1489;
                    let v1491 = v2 + (v1489 * v1482);
                    let v1492 = v1488 * v1491;
                    let v3785 = ((v3777 * v1483) * v1488) * v1491;
                    let v3788 = (Lanes([v3785[0], 0.0, 0.0])) + (((Lanes([0.0, v3780[0], v3780[1]])) + (Lanes([v3781[0], 0.0, 0.0]))) * v1488);
                    v1496 = v1492;
                    v2071 = v3788;
                }
                let v1494 = v1493 - v2;
                let v3795 = v2280 * v1494;
                let v1497 = v1496 - v2;
                let v3799 = v2294 * v1497;
                let v1499 = (v436 * v1494) + (v443 * v1497);
                let v3803 = ((Lanes([v3795[0], 0.0, 0.0])) + (v2070 * v436)) + ((Lanes([v3799[0], 0.0, 0.0])) + (v2071 * v443));
                v1521 = v1499;
                v2069 = v3803;
            } else {
                v1521 = v0;
                v2069 = v3754;
            }
            let v3805 = v2529 * v1500;
            let v3807 = (v2034 * v653) + (Lanes([0.0, 0.0, v3805[0], v3805[1]]));
            let v3809 = v2537 * v1467;
            let v1507 = v690 - v1003;
            let v3814 = Lanes([0.0, 0.0, 0.0, 0.0, v1994[0]]);
            let v3818 = v2556 * v1507;
            let v3821 = ((Lanes([v3807[0], v3807[1], v3807[2], v3807[3], 0.0])) + ((v3753 * v659) + (Lanes([0.0, v3809[0], v3809[1], 0.0, 0.0])))) + (((v3814 - (Lanes([v3016[0], v3016[1], v3016[2], v3016[3], 0.0]))) * v672) + (Lanes([0.0, v3818[0], 0.0, v3818[1], 0.0])));
            let v3823 = v2533 * v1510;
            let v3825 = (v2035 * v656) + (Lanes([0.0, v3823[0], 0.0, v3823[1]]));
            let v3828 = (Lanes([v3821[0], v3821[1], 0.0, v3821[2], v3821[3], v3821[4]])) + (Lanes([v3825[0], 0.0, v3825[1], v3825[2], v3825[3], 0.0]));
            let v3830 = v2549 * v1516;
            let v3832 = (v2058 * v667) + (Lanes([0.0, v3830[0], v3830[1]]));
            let v3835 = (Lanes([v3828[0], v3828[1], v3828[2], v3828[3], v3828[4], 0.0, v3828[5]])) + (Lanes([v3832[0], 0.0, v3832[1], 0.0, 0.0, v3832[2], 0.0]));
            let v3837 = v2589 * v1375;
            let v3839 = (v3648 * v688) + (Lanes([v3837[0], 0.0, v3837[1]]));
            let v3844 = v2582 * v1521;
            let v3846 = (v2069 * v684) + (Lanes([0.0, v3844[0], v3844[1]]));
            let v3850 = v2586 * v1524;
            let v3852 = (v2028 * v686) + (Lanes([0.0, 0.0, v3850[0], 0.0, 0.0, v3850[1]]));
            let v3854 = (((Lanes([0.0, v3835[0], v3835[1], v3835[2], v3835[3], v3835[4], v3835[5], 0.0, v3835[6]])) + (Lanes([v3839[0], v3839[1], 0.0, 0.0, 0.0, 0.0, 0.0, v3839[2], 0.0]))) + (Lanes([0.0, v3846[0], 0.0, 0.0, 0.0, 0.0, v3846[1], v3846[2], 0.0]))) + (Lanes([0.0, v3852[0], v3852[1], v3852[2], v3852[3], 0.0, v3852[4], v3852[5], 0.0]));
            let v3856 = v2562 * v1344;
            let v3858 = (v3563 * v675) + (Lanes([v3856[0], 0.0, v3856[1]]));
            let v3863 = v2566 * v1367;
            let v3865 = (v3615 * v677) + (Lanes([0.0, v3863[0], v3863[1], 0.0]));
            let v3867 = ((Lanes([0.0, v3854[0], v3854[1], 0.0, v3854[2], v3854[3], v3854[4], v3854[5], v3854[6], v3854[7], v3854[8]])) + (Lanes([v3858[0], 0.0, v3858[1], v3858[2], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, v3865[0], v3865[1], v3865[2], 0.0, v3865[3], 0.0, 0.0, 0.0, 0.0]));
            let v3869 = v2569 * v1368;
            let v3871 = (v3620 * v678) + (Lanes([v3869[0], 0.0, v3869[1]]));
            let v3876 = v2572 * v1370;
            let v3878 = (v3629 * v679) + (Lanes([0.0, 0.0, v3876[0], v3876[1], 0.0]));
            let v3880 = ((Lanes([v3867[0], 0.0, v3867[1], v3867[2], v3867[3], v3867[4], v3867[5], v3867[6], v3867[7], v3867[8], v3867[9], v3867[10]])) + (Lanes([0.0, v3871[0], 0.0, v3871[1], 0.0, 0.0, v3871[2], 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v3878[0], 0.0, v3878[1], v3878[2], v3878[3], v3878[4], 0.0, 0.0, 0.0]));
            let v3882 = v2575 * v1371;
            let v3884 = (v3634 * v680) + (Lanes([v3882[0], 0.0, v3882[1]]));
            let v3889 = v2578 * v1374;
            let v3891 = (v3643 * v681) + (Lanes([0.0, v3889[0], 0.0, 0.0, 0.0, v3889[1]]));
            let v1540 = -v1539;
            let v1541 = v1540 * ((((((((((((((v1500 * v653) + (v1467 * v659)) + (v1507 * v672)) + (v1510 * v656)) + (v1516 * v667)) + (v1375 * v688)) + (v1521 * v684)) + (v1524 * v686)) + (v1344 * v675)) + (v1367 * v677)) + (v1368 * v678)) + (v1370 * v679)) + (v1371 * v680)) + (v1374 * v681));
            let v3894 = (((Lanes([v3880[0], v3880[1], 0.0, v3880[2], v3880[3], v3880[4], v3880[5], v3880[6], v3880[7], v3880[8], v3880[9], v3880[10], v3880[11]])) + (Lanes([0.0, 0.0, v3884[0], 0.0, v3884[1], 0.0, 0.0, 0.0, 0.0, v3884[2], 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, 0.0, v3891[0], v3891[1], v3891[2], v3891[3], v3891[4], 0.0, v3891[5], 0.0, 0.0]))) * v1540;
            let v1542 = v302 * v634;
            let v3897 = (v1981 * v634) + (v2009 * v302);
            let v1543 = v690 - v1004;
            let v3899 = v3814 - (Lanes([v3020[0], v3020[1], v3020[2], v3020[3], 0.0]));
            let v1544 = v690 - v689;
            let v3902 = (Lanes([0.0, v1994[0]])) - (Lanes([v1993[0], 0.0]));
            let v3903 = v2529 * v1545;
            let v3906 = v2533 * v1545;
            let v3909 = v2549 * v1545;
            let v3912 = v2537 * v1545;
            let v3915 = v2545 * v1545;
            let v3918 = v2582 * v1545;
            let v1559 = v647 * (v1500 + (v1545 * v653));
            let v3921 = (v2034 + (Lanes([0.0, 0.0, v3903[0], v3903[1]]))) * v647;
            let v1560 = v647 * (v1510 + (v1545 * v656));
            let v3922 = (v2035 + (Lanes([0.0, v3906[0], 0.0, v3906[1]]))) * v647;
            let v1561 = v647 * v690;
            let v3923 = v1994 * v647;
            let v1562 = v647 * v1003;
            let v3924 = v3016 * v647;
            let v1563 = v647 * (v1467 + (v1545 * v659));
            let v3925 = (v3753 + (Lanes([0.0, v3912[0], v3912[1], 0.0, 0.0]))) * v647;
            let v1564 = v647 * (v1554 + (v1545 * v664));
            let v3926 = (v2065 + (Lanes([0.0, 0.0, v3915[0], v3915[1]]))) * v647;
            let v1565 = v647 * (v1516 + (v1545 * v667));
            let v3927 = (v2058 + (Lanes([0.0, v3909[0], v3909[1]]))) * v647;
            let v1566 = v647 * v1367;
            let v3928 = v3615 * v647;
            let v1567 = v647 * (v1521 + (v1545 * v684));
            let v3929 = (v2069 + (Lanes([0.0, v3918[0], v3918[1]]))) * v647;
            let v1568 = v647 * v1524;
            let v3930 = v2028 * v647;
            let v1569 = if v575 > v0 { 1.0 } else { 0.0 };
            let v1915: f64;
            let v2072: Lanes<3>;
            if v1569 != 0.0 {
                let v1570 = -v562;
                let v3931 = v2434 * v2085;
                let v1571 = v1570 * v692;
                let v3932 = v3931 * v692;
                let v1573 = if v1572 <= v0 { 1.0 } else { 0.0 };
                let v1916: f64;
                let v2073: Lanes<3>;
                if v1573 != 0.0 {
                    let v1574 = v684 + v1571;
                    let v3992 = Lanes([0.0, v2582[0], v2582[1]]);
                    let v3994 = v3992 + (Lanes([v3932[0], 0.0, 0.0]));
                    let v1575 = if v1574 > v0 { 1.0 } else { 0.0 };
                    let v1598: f64;
                    let v1599: f64;
                    let v2074: Lanes<3>;
                    let v2075: Lanes<3>;
                    if v1575 != 0.0 {
                        let v1576 = v2 - v692;
                        let v1578 = v1576.powf((-v577));
                        let v1580 = v2 - (v1578 * v1576);
                        let v1582 = v2 - v577;
                        let v1583 = (v562 * v1580) / v1582;
                        let v4011 = (v2434 * v1580) / v1582;
                        let v1584 = v105 * v577;
                        let v1586 = v562 * v1576;
                        let v1587 = (v1584 * v1574) / v1586;
                        let v4014 = (v2434 * v1576) * v1587;
                        let v1588 = v2 + v1587;
                        let v1590 = (v1574 * v1588) * v1578;
                        let v4021 = ((v3994 * v1588) + ((((v3994 * v1584) - (Lanes([v4014[0], 0.0, 0.0]))) / v1586) * v1574)) * v1578;
                        let v4022 = Lanes([v4011[0], 0.0, 0.0]);
                        v1598 = v1583;
                        v1599 = v1590;
                        v2074 = v4022;
                        v2075 = v4021;
                    } else {
                        let v1591 = v684 / v562;
                        let v3995 = v2434 * v1591;
                        let v1592 = v2 - v1591;
                        let v1593 = v2 - v577;
                        let v1595 = v2 - (v1592.powf(v1593));
                        let v4005 = v2434 * v1595;
                        let v1597 = (v562 * v1595) / v1593;
                        let v4009 = ((Lanes([v4005[0], 0.0, 0.0])) + ((((((v3992 - (Lanes([v3995[0], 0.0, 0.0]))) / v562) * v2085) * (v1593 * (v1592.powf((v1593 - v1980))))) * v2085) * v562)) / v1593;
                        v1598 = v1597;
                        v1599 = v0;
                        v2074 = v4009;
                        v2075 = v3754;
                    }
                    let v1600 = v1598 + v1599;
                    let v4023 = v2074 + v2075;
                    v1916 = v1600;
                    v2073 = v4023;
                } else {
                    let v3933 = v3932 * v1571;
                    let v1603 = (v107 * v1572) * v1572;
                    let v1605 = ((v1571 * v1571) + v1603).sqrt();
                    let v1608 = v1606 * (v1571 + v1605);
                    let v3939 = (v3932 + ((v3933 + v3933) * (v1980 / (v2348 * v1605)))) * v1606;
                    let v1609 = v1608 / v562;
                    let v1610 = v2 - v1609;
                    let v1611 = v2 - v577;
                    let v1612 = v1610.powf(v1611);
                    let v3944 = v1611 - v1980;
                    let v3951 = ((v3931 * v1612) + (((((v3939 - (v2434 * v1609)) / v562) * v2085) * (v1611 * (v1610.powf(v3944)))) * v1570)) / v1611;
                    let v1615 = v684 + v1571;
                    let v3952 = Lanes([0.0, v2582[0], v2582[1]]);
                    let v3953 = Lanes([v3932[0], 0.0, 0.0]);
                    let v3954 = v3952 + v3953;
                    let v3955 = v3954 * v1615;
                    let v1618 = ((v1615 * v1615) + v1603).sqrt();
                    let v1621 = (v105 * (v1615 - v1618)) - v1571;
                    let v3962 = ((v3954 - ((v3955 + v3955) * (v1980 / (v2348 * v1618)))) * v105) - v3953;
                    let v1622 = v1621 / v562;
                    let v3963 = v2434 * v1622;
                    let v1623 = v2 - v1622;
                    let v1624 = v1623.powf(v1611);
                    let v3971 = v3931 * v1624;
                    let v1627 = v2 - v692;
                    let v1629 = v1627.powf((-v577));
                    let v1631 = (v684 - v1621) + v1608;
                    let v3978 = (v3952 - v3962) + (Lanes([v3939[0], 0.0, 0.0]));
                    let v1632 = v1629 * v1631;
                    let v1633 = v105 * v577;
                    let v1635 = v562 * v1627;
                    let v1636 = (v1633 * v1631) / v1635;
                    let v3982 = (v2434 * v1627) * v1636;
                    let v1637 = v2 + v1636;
                    let v1640 = (((v1570 * v1624) / v1611) + (v1632 * v1637)) - ((v1570 * v1612) / v1611);
                    let v3991 = ((((Lanes([v3971[0], 0.0, 0.0])) + (((((v3962 - (Lanes([v3963[0], 0.0, 0.0]))) / v562) * v2085) * (v1611 * (v1623.powf(v3944)))) * v1570)) / v1611) + (((v3978 * v1629) * v1637) + ((((v3978 * v1633) - (Lanes([v3982[0], 0.0, 0.0]))) / v1635) * v1632))) - (Lanes([v3951[0], 0.0, 0.0]));
                    v1916 = v1640;
                    v2073 = v3991;
                }
                v1915 = v1916;
                v2072 = v2073;
            } else {
                v1915 = v0;
                v2072 = v3754;
            }
            let v1897: f64;
            let v2076: Lanes<3>;
            if v695 != 0.0 {
                let v1641 = v656 + v693;
                let v4083 = Lanes([0.0, v2533[0], v2533[1]]);
                let v4085 = v4083 + (Lanes([v2591[0], 0.0, 0.0]));
                let v1642 = if v1641 > v0 { 1.0 } else { 0.0 };
                let v1665: f64;
                let v1666: f64;
                let v2077: Lanes<3>;
                let v2078: Lanes<3>;
                if v1642 != 0.0 {
                    let v1643 = v2 - v692;
                    let v1645 = v1643.powf((-v565));
                    let v1647 = v2 - (v1645 * v1643);
                    let v1649 = v2 - v565;
                    let v1650 = (v506 * v1647) / v1649;
                    let v4103 = (v2358 * v1647) / v1649;
                    let v1651 = v105 * v565;
                    let v1653 = v506 * v1643;
                    let v1654 = (v1651 * v1641) / v1653;
                    let v4106 = (v2358 * v1643) * v1654;
                    let v1655 = v2 + v1654;
                    let v1657 = (v1641 * v1655) * v1645;
                    let v4113 = ((v4085 * v1655) + ((((v4085 * v1651) - (Lanes([v4106[0], 0.0, 0.0]))) / v1653) * v1641)) * v1645;
                    let v4114 = Lanes([v4103[0], 0.0, 0.0]);
                    v1665 = v1650;
                    v1666 = v1657;
                    v2077 = v4114;
                    v2078 = v4113;
                } else {
                    let v1658 = v656 / v506;
                    let v4086 = v2358 * v1658;
                    let v1659 = v2 - v1658;
                    let v1660 = v2 - v565;
                    let v1662 = v2 - (v1659.powf(v1660));
                    let v4096 = v2358 * v1662;
                    let v1664 = (v506 * v1662) / v1660;
                    let v4100 = ((Lanes([v4096[0], 0.0, 0.0])) + ((((((v4083 - (Lanes([v4086[0], 0.0, 0.0]))) / v506) * v2085) * (v1660 * (v1659.powf((v1660 - v1980))))) * v2085) * v506)) / v1660;
                    v1665 = v1664;
                    v1666 = v0;
                    v2077 = v4100;
                    v2078 = v4101;
                }
                let v1667 = v1665 + v1666;
                let v4115 = v2077 + v2078;
                v1897 = v1667;
                v2076 = v4115;
            } else {
                let v4024 = v2591 * v693;
                let v1670 = (v107 * v694) * v694;
                let v1672 = ((v693 * v693) + v1670).sqrt();
                let v1675 = v1673 * (v693 + v1672);
                let v4030 = (v2591 + ((v4024 + v4024) * (v1980 / (v2348 * v1672)))) * v1673;
                let v1676 = v1675 / v506;
                let v1677 = v2 - v1676;
                let v1678 = v2 - v565;
                let v1679 = v1677.powf(v1678);
                let v4035 = v1678 - v1980;
                let v4042 = ((v2590 * v1679) + (((((v4030 - (v2358 * v1676)) / v506) * v2085) * (v1678 * (v1677.powf(v4035)))) * v691)) / v1678;
                let v1682 = v656 + v693;
                let v4043 = Lanes([0.0, v2533[0], v2533[1]]);
                let v4044 = Lanes([v2591[0], 0.0, 0.0]);
                let v4045 = v4043 + v4044;
                let v4046 = v4045 * v1682;
                let v1685 = ((v1682 * v1682) + v1670).sqrt();
                let v1688 = (v105 * (v1682 - v1685)) - v693;
                let v4053 = ((v4045 - ((v4046 + v4046) * (v1980 / (v2348 * v1685)))) * v105) - v4044;
                let v1689 = v1688 / v506;
                let v4054 = v2358 * v1689;
                let v1690 = v2 - v1689;
                let v1691 = v1690.powf(v1678);
                let v4062 = v2590 * v1691;
                let v1694 = v2 - v692;
                let v1696 = v1694.powf((-v565));
                let v1698 = (v656 - v1688) + v1675;
                let v4069 = (v4043 - v4053) + (Lanes([v4030[0], 0.0, 0.0]));
                let v1699 = v1696 * v1698;
                let v1700 = v105 * v565;
                let v1702 = v506 * v1694;
                let v1703 = (v1700 * v1698) / v1702;
                let v4073 = (v2358 * v1694) * v1703;
                let v1704 = v2 + v1703;
                let v1707 = (((v691 * v1691) / v1678) + (v1699 * v1704)) - ((v691 * v1679) / v1678);
                let v4082 = ((((Lanes([v4062[0], 0.0, 0.0])) + (((((v4053 - (Lanes([v4054[0], 0.0, 0.0]))) / v506) * v2085) * (v1678 * (v1690.powf(v4035)))) * v691)) / v1678) + (((v4069 * v1696) * v1704) + ((((v4069 * v1700) - (Lanes([v4073[0], 0.0, 0.0]))) / v1702) * v1699))) - (Lanes([v4042[0], 0.0, 0.0]));
                v1897 = v1707;
                v2076 = v4082;
            }
            let v1909: f64;
            let v2079: Lanes<3>;
            if v766 != 0.0 {
                let v1708 = v667 + v764;
                let v4280 = Lanes([0.0, v2549[0], v2549[1]]);
                let v4282 = v4280 + (Lanes([v2685[0], 0.0, 0.0]));
                let v1709 = if v1708 > v0 { 1.0 } else { 0.0 };
                let v1750: f64;
                let v1752: f64;
                let v2080: Lanes<3>;
                let v2081: Lanes<3>;
                if v1709 != 0.0 {
                    let v1710 = v2 - v692;
                    let v1713 = v1710.powf((v1711 - v570));
                    let v1716 = v2 - ((v1713 * v1710) * v1710);
                    let v1718 = v2 - v570;
                    let v1719 = (v534 * v1716) / v1718;
                    let v4323 = (v2396 * v1716) / v1718;
                    let v1720 = v105 * v570;
                    let v1722 = (v1720 * v1708) / v534;
                    let v4325 = v2396 * v1722;
                    let v1723 = v1710 + v1722;
                    let v1725 = (v1708 * v1723) * v1713;
                    let v4332 = ((v4282 * v1723) + ((((v4282 * v1720) - (Lanes([v4325[0], 0.0, 0.0]))) / v534) * v1708)) * v1713;
                    let v4333 = Lanes([v4323[0], 0.0, 0.0]);
                    v1750 = v1719;
                    v1752 = v1725;
                    v2080 = v4333;
                    v2081 = v4332;
                } else {
                    let v1729 = if (if v785 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v667 < (-v785) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v1751: f64;
                    let v2082: Lanes<3>;
                    if v1729 != 0.0 {
                        let v1730 = v785 / v534;
                        let v1731 = v2 + v1730;
                        let v1732 = v2 - v570;
                        let v1733 = v1731.powf(v1732);
                        let v4305 = v2549 * v1732;
                        let v1736 = v534 + v785;
                        let v1737 = (v1732 * (v667 + v785)) / v1736;
                        let v4306 = v2396 * v1737;
                        let v1738 = v2 - v1737;
                        let v4312 = ((((v2396 * v1730) * v2085) / v534) * (v1732 * (v1731.powf((v1732 - v1980))))) * v1738;
                        let v1740 = v2 - (v1733 * v1738);
                        let v4317 = v2396 * v1740;
                        let v1742 = (v534 * v1740) / v1732;
                        let v4321 = ((Lanes([v4317[0], 0.0, 0.0])) + ((((Lanes([v4312[0], 0.0, 0.0])) + (((((Lanes([0.0, v4305[0], v4305[1]])) - (Lanes([v4306[0], 0.0, 0.0]))) / v1736) * v2085) * v1733)) * v2085) * v534)) / v1732;
                        v1751 = v1742;
                        v2082 = v4321;
                    } else {
                        let v1743 = v667 / v534;
                        let v4283 = v2396 * v1743;
                        let v1744 = v2 - v1743;
                        let v1745 = v2 - v570;
                        let v1747 = v2 - (v1744.powf(v1745));
                        let v4293 = v2396 * v1747;
                        let v1749 = (v534 * v1747) / v1745;
                        let v4297 = ((Lanes([v4293[0], 0.0, 0.0])) + ((((((v4280 - (Lanes([v4283[0], 0.0, 0.0]))) / v534) * v2085) * (v1745 * (v1744.powf((v1745 - v1980))))) * v2085) * v534)) / v1745;
                        v1751 = v1749;
                        v2082 = v4297;
                    }
                    v1750 = v1751;
                    v1752 = v0;
                    v2080 = v2082;
                    v2081 = v3487;
                }
                let v1753 = v1750 + v1752;
                let v4334 = v2080 + v2081;
                v1909 = v1753;
                v2079 = v4334;
            } else {
                let v1756 = if (if v785 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v815 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1910: f64;
                let v2083: Lanes<3>;
                if v1756 != 0.0 {
                    let v1758 = v785 - v764;
                    let v4166 = v2685 * v2085;
                    let v1759 = (v785 + v764) / v1758;
                    let v4169 = (v2685 - (v4166 * v1759)) / v1758;
                    let v1761 = v1759 - v2;
                    let v4171 = v4169 * v1761;
                    let v1764 = (v107 * v765) * v765;
                    let v1766 = ((v1761 * v1761) + v1764).sqrt();
                    let v1767 = v1759 + v2;
                    let v4176 = v4169 * v1767;
                    let v1770 = (v107 * v815) * v815;
                    let v1772 = ((v1767 * v1767) + v1770).sqrt();
                    let v1773 = v1766 + v1772;
                    let v1774 = (v470 * v1759) / v1773;
                    let v1778 = v105 * (((v1774 * v1758) - v785) - v764);
                    let v4189 = ((((((v4169 * v470) - ((((v4171 + v4171) * (v1980 / (v2348 * v1766))) + ((v4176 + v4176) * (v1980 / (v2348 * v1772)))) * v1774)) / v1773) * v1758) + (v4166 * v1774)) - v2685) * v105;
                    let v1779 = v1778 / v534;
                    let v1780 = v2 - v1779;
                    let v1781 = v2 - v570;
                    let v4194 = v1781 - v1980;
                    let v1783 = v2 - (v1780.powf(v1781));
                    let v4202 = ((v2396 * v1783) + ((((((v4189 - (v2396 * v1779)) / v534) * v2085) * (v1781 * (v1780.powf(v4194)))) * v2085) * v534)) / v1781;
                    let v4203 = v2549 * v470;
                    let v4205 = Lanes([v2685[0], 0.0, 0.0]);
                    let v1789 = (((v470 * v667) + v785) + v764) / v1758;
                    let v4207 = v4166 * v1789;
                    let v4210 = (((Lanes([0.0, v4203[0], v4203[1]])) + v4205) - (Lanes([v4207[0], 0.0, 0.0]))) / v1758;
                    let v1791 = v1789 - v2;
                    let v4212 = v4210 * v1791;
                    let v1794 = ((v1791 * v1791) + v1764).sqrt();
                    let v1795 = v1789 + v2;
                    let v4217 = v4210 * v1795;
                    let v1798 = ((v1795 * v1795) + v1770).sqrt();
                    let v1799 = v1794 + v1798;
                    let v1800 = (v470 * v1789) / v1799;
                    let v4225 = ((v4210 * v470) - ((((v4212 + v4212) * (v1980 / (v2348 * v1794))) + ((v4217 + v4217) * (v1980 / (v2348 * v1798)))) * v1800)) / v1799;
                    let v4227 = v4166 * v1800;
                    let v1804 = v105 * (((v1800 * v1758) - v785) - v764);
                    let v4231 = (((v4225 * v1758) + (Lanes([v4227[0], 0.0, 0.0]))) - v4205) * v105;
                    let v1805 = v1804 / v534;
                    let v4232 = v2396 * v1805;
                    let v1806 = v2 - v1805;
                    let v1808 = v2 - (v1806.powf(v1781));
                    let v4241 = v2396 * v1808;
                    let v1812 = v105 * (v1800 + v2);
                    let v4246 = v4225 * v105;
                    let v1813 = v785 / v534;
                    let v1814 = v2 + v1813;
                    let v1815 = -v570;
                    let v1816 = v1814.powf(v1815);
                    let v4250 = v1815 - v1980;
                    let v1817 = v764 / v534;
                    let v1818 = v2 + v1817;
                    let v1819 = v1818.powf(v1815);
                    let v1820 = v2 - v1812;
                    let v4262 = ((((v2396 * v1813) * v2085) / v534) * (v1815 * (v1814.powf(v4250)))) * v1820;
                    let v4266 = (((v2685 - (v2396 * v1817)) / v534) * (v1815 * (v1818.powf(v4250)))) * v1812;
                    let v1823 = (v1820 * v1816) + (v1812 * v1819);
                    let v1825 = (v667 - v1804) + v1778;
                    let v1828 = ((v1825 * v1823) + ((v534 * v1808) / v1781)) - ((v534 * v1783) / v1781);
                    let v4279 = ((((((Lanes([0.0, v2549[0], v2549[1]])) - v4231) + (Lanes([v4189[0], 0.0, 0.0]))) * v1823) + (((((v4246 * v2085) * v1816) + (Lanes([v4262[0], 0.0, 0.0]))) + ((v4246 * v1819) + (Lanes([v4266[0], 0.0, 0.0])))) * v1825)) + (((Lanes([v4241[0], 0.0, 0.0])) + ((((((v4231 - (Lanes([v4232[0], 0.0, 0.0]))) / v534) * v2085) * (v1781 * (v1806.powf(v4194)))) * v2085) * v534)) / v1781)) - (Lanes([v4202[0], 0.0, 0.0]));
                    v1910 = v1828;
                    v2083 = v4279;
                } else {
                    let v4116 = v2685 * v764;
                    let v1831 = (v107 * v765) * v765;
                    let v1833 = ((v764 * v764) + v1831).sqrt();
                    let v1836 = v1834 * (v764 + v1833);
                    let v4122 = (v2685 + ((v4116 + v4116) * (v1980 / (v2348 * v1833)))) * v1834;
                    let v1837 = v1836 / v534;
                    let v1838 = v2 - v1837;
                    let v1839 = v2 - v570;
                    let v1840 = v1838.powf(v1839);
                    let v4127 = v1839 - v1980;
                    let v4134 = ((v2684 * v1840) + (((((v4122 - (v2396 * v1837)) / v534) * v2085) * (v1839 * (v1838.powf(v4127)))) * v763)) / v1839;
                    let v1843 = v667 + v764;
                    let v4135 = Lanes([0.0, v2549[0], v2549[1]]);
                    let v4136 = Lanes([v2685[0], 0.0, 0.0]);
                    let v4137 = v4135 + v4136;
                    let v4138 = v4137 * v1843;
                    let v1846 = ((v1843 * v1843) + v1831).sqrt();
                    let v1849 = (v105 * (v1843 - v1846)) - v764;
                    let v4145 = ((v4137 - ((v4138 + v4138) * (v1980 / (v2348 * v1846)))) * v105) - v4136;
                    let v1850 = v1849 / v534;
                    let v4146 = v2396 * v1850;
                    let v1851 = v2 - v1850;
                    let v1852 = v1851.powf(v1839);
                    let v4154 = v2684 * v1852;
                    let v1857 = (v2 - v692).powf((-v570));
                    let v1862 = (((v763 * v1852) / v1839) + (v1857 * ((v667 - v1849) + v1836))) - ((v763 * v1840) / v1839);
                    let v4165 = ((((Lanes([v4154[0], 0.0, 0.0])) + (((((v4145 - (Lanes([v4146[0], 0.0, 0.0]))) / v534) * v2085) * (v1839 * (v1851.powf(v4127)))) * v763)) / v1839) + (((v4135 - v4145) + (Lanes([v4122[0], 0.0, 0.0]))) * v1857)) - (Lanes([v4134[0], 0.0, 0.0]));
                    v1910 = v1862;
                    v2083 = v4165;
                }
                v1909 = v1910;
                v2079 = v2083;
            }
            let v1863 = if v939 > v0 { 1.0 } else { 0.0 };
            let v1864: f64;
            if v1863 != 0.0 {
                v1864 = v2;
            } else {
                v1864 = v0;
            }
            let v1866 = (v939 * v1864) * v38;
            let v4336 = (v2932 * v1864) * v38;
            let v1867 = v1866 + v2;
            let v1868 = v1866 / v1867;
            let v4339 = (v4336 - (v4336 * v1868)) / v1867;
            let v1871 = (v659 * v34) / v1870;
            let v4341 = (v2537 * v34) / v1870;
            let v1872 = if v1871 < v18 { 1.0 } else { 0.0 };
            let v1884: f64;
            let v2084: Lanes<2>;
            if v1872 != 0.0 {
                let v1873 = v1871.exp();
                let v4343 = v4341 * v1873;
                v1884 = v1873;
                v2084 = v4343;
            } else {
                let v1874 = v18.exp();
                let v1877 = v1874 * (v2 + (v1871 - v18));
                let v4342 = v4341 * v1874;
                v1884 = v1877;
                v2084 = v4342;
            }
            let v1882 = v1878 * (v2 + (v1879 * v972));
            let v1885 = v1883 * v1884;
            let v4347 = v4339 * v1868;
            let v1887 = v39 + (v1868 * v1868);
            let v4349 = (v2084 * v1883) * v1887;
            let v4350 = (v4347 + v4347) * v1885;
            let v1890 = v2 + ((v1885 * v1887) * v1864);
            let v1891 = v1882 * v1890;
            let v4358 = v2442 * v957;
            let v4362 = ((Lanes([v4358[0], 0.0, 0.0])) + (v2014 * v567)) * v1059;
            let v4364 = v2932 * v1891;
            let v1895 = (v1891 * v939) / v1000;
            let v4372 = v2442 * v1897;
            let v1899 = v2 - v1059;
            let v4377 = v2450 * v960;
            let v4386 = v2451 * v1909;
            let v4389 = (Lanes([v4386[0], 0.0, 0.0])) + (v2079 * v574);
            let v4393 = v2459 * v1915;
            let v4397 = v2582 * v1918;
            let v1922 = v670 * v1921;
            let v4400 = v2552 * v1921;
            let v1924 = v674 * v1923;
            let v4401 = v2559 * v1923;
            let v1926 = v302 * v1925;
            let v4402 = v1981 * v1925;
            let v1928 = v1927 * v689;
            let v4403 = v1993 * v1927;
            let v1931 = (v1927 * v690) * v1930;
            let v4405 = (v1994 * v1927) * v1930;
            let v1932 = v647 * (((v567 * v957) * v1059) + v1895);
            let v4406 = ((Lanes([v4362[0], 0.0, v4362[1], v4362[2]])) + ((((((((v2980 * v1879) * v1878) * v1890) + ((((Lanes([0.0, v4349[0], v4349[1], 0.0])) + (Lanes([v4350[0], 0.0, v4350[1], v4350[2]]))) * v1864) * v1882)) * v939) + (Lanes([v4364[0], 0.0, v4364[1], v4364[2]]))) - (v2024 * v1895)) / v1000)) * v647;
            let v1933 = v647 * ((v567 * v1897) * v1899);
            let v4407 = (((Lanes([v4372[0], 0.0, 0.0])) + (v2076 * v567)) * v1899) * v647;
            let v1934 = v647 * (((v572 * v960) + (v1902 * v956)) + (v1905 * v1339));
            let v4408 = ((((Lanes([v4377[0], 0.0, 0.0])) + (v2017 * v572)) + (v2962 * v1902)) + (v3551 * v1905)) * v647;
            let v1935 = v647 * (v1905 * v1343);
            let v4409 = (v3558 * v1905) * v647;
            let v1936 = v647 * ((v574 * v1909) + (v1902 * v1912));
            let v4410 = ((Lanes([v4389[0], 0.0, v4389[1], 0.0, v4389[2]])) + (v2029 * v1902)) * v647;
            let v1937 = v647 * ((v579 * v1915) + (v1918 * v684));
            let v4411 = (((Lanes([v4393[0], 0.0, 0.0])) + (v2072 * v579)) + (Lanes([0.0, v4397[0], v4397[1]]))) * v647;
            let v1938 = ddt(10542, v1932);
            let v4413 = v4406 * v4412;
            let v1939 = ddt(10544, v1933);
            let v4414 = v4407 * v4412;
            let v1940 = ddt(10546, v1934);
            let v4415 = v4408 * v4412;
            let v1941 = ddt(10548, v1935);
            let v4416 = v4409 * v4412;
            let v1942 = ddt(10550, v1936);
            let v4417 = v4410 * v4412;
            let v1943 = ddt(10552, v1922);
            let v4418 = v4400 * v4412;
            let v1944 = ddt(10554, v1924);
            let v4419 = v4401 * v4412;
            let v1945 = ddt(10556, v1937);
            let v4420 = v4411 * v4412;
            let v1946 = ddt(10558, v1928);
            let v4421 = v4403 * v4412;
            let v1947 = ddt(10560, v1931);
            let v4422 = v4405 * v4412;
            let v1948 = ddt(10562, v1926);
            let v4423 = v4402 * v4412;
            let v1965: f64;
            let v1966: f64;
            let v1967: f64;
            let v1968: f64;
            let v1969: f64;
            let v1970: f64;
            let v1971: f64;
            let v1972: f64;
            let v1973: f64;
            let v1974: f64;
            let v1975: f64;
            let v1976: f64;
            let v1977: f64;
            let v1978: f64;
            let v1979: f64;
            if v1949 != 0.0 {
                v1965 = v1950;
                v1966 = v1951;
                v1967 = v1952;
                v1968 = v1953;
                v1969 = v1954;
                v1970 = v1955;
                v1971 = v1956;
                v1972 = v1957;
                v1973 = v1958;
                v1974 = v1959;
                v1975 = v1960;
                v1976 = v1961;
                v1977 = v1962;
                v1978 = v1963;
                v1979 = v1964;
            } else {
                v1965 = v0;
                v1966 = v0;
                v1967 = v0;
                v1968 = v0;
                v1969 = v0;
                v1970 = v0;
                v1971 = v0;
                v1972 = v0;
                v1973 = v0;
                v1974 = v0;
                v1975 = v0;
                v1976 = v0;
                v1977 = v0;
                v1978 = v0;
                v1979 = v0;
            }
            let v4424 = v3921[0];
            let v4425 = v3921[1];
            let v4426 = v3921[2];
            let v4427 = v3921[3];
            let v4428 = v3922[0];
            let v4429 = v3922[1];
            let v4430 = v3922[2];
            let v4431 = v3922[3];
            let v4432 = v3923[0];
            let v4433 = v3924[0];
            let v4434 = v3924[1];
            let v4435 = v3924[2];
            let v4436 = v3924[3];
            let v4437 = v3925[0];
            let v4438 = v3925[1];
            let v4439 = v3925[2];
            let v4440 = v3925[3];
            let v4441 = v3925[4];
            let v4442 = v3926[0];
            let v4443 = v3926[1];
            let v4444 = v3926[2];
            let v4445 = v3926[3];
            let v4446 = v3927[0];
            let v4447 = v3927[1];
            let v4448 = v3927[2];
            let v4449 = v3563[0];
            let v4450 = v3563[1];
            let v4451 = v3563[2];
            let v4452 = v3928[0];
            let v4453 = v3928[1];
            let v4454 = v3928[2];
            let v4455 = v3928[3];
            let v4456 = v3620[0];
            let v4457 = v3620[1];
            let v4458 = v3620[2];
            let v4459 = v3629[0];
            let v4460 = v3629[1];
            let v4461 = v3629[2];
            let v4462 = v3629[3];
            let v4463 = v3629[4];
            let v4464 = v3634[0];
            let v4465 = v3634[1];
            let v4466 = v3634[2];
            let v4467 = v3643[0];
            let v4468 = v3643[1];
            let v4469 = v3643[2];
            let v4470 = v3643[3];
            let v4471 = v3643[4];
            let v4472 = v3643[5];
            let v4473 = v3929[0];
            let v4474 = v3929[1];
            let v4475 = v3929[2];
            let v4476 = v3930[0];
            let v4477 = v3930[1];
            let v4478 = v3930[2];
            let v4479 = v3930[3];
            let v4480 = v3930[4];
            let v4481 = v3930[5];
            let v4482 = v3648[0];
            let v4483 = v3648[1];
            let v4484 = v3648[2];
            let v4485 = v3899[0];
            let v4486 = v3899[1];
            let v4487 = v3899[2];
            let v4488 = v3899[3];
            let v4489 = v3899[4];
            let v4490 = v3902[0];
            let v4491 = v3902[1];
            let v4492 = v3897[0];
            let v4493 = v3894[0];
            let v4494 = v3894[1];
            let v4495 = v3894[2];
            let v4496 = v3894[3];
            let v4497 = v3894[4];
            let v4498 = v3894[5];
            let v4499 = v3894[6];
            let v4500 = v3894[7];
            let v4501 = v3894[8];
            let v4502 = v3894[9];
            let v4503 = v3894[10];
            let v4504 = v3894[11];
            let v4505 = v3894[12];
            let v4506 = v4413[0];
            let v4507 = v4413[1];
            let v4508 = v4413[2];
            let v4509 = v4413[3];
            let v4510 = v4414[0];
            let v4511 = v4414[1];
            let v4512 = v4414[2];
            let v4513 = v4415[0];
            let v4514 = v4415[1];
            let v4515 = v4415[2];
            let v4516 = v4416[0];
            let v4517 = v4416[1];
            let v4518 = v4416[2];
            let v4519 = v4417[0];
            let v4520 = v4417[1];
            let v4521 = v4417[2];
            let v4522 = v4417[3];
            let v4523 = v4417[4];
            let v4524 = v4418[0];
            let v4525 = v4418[1];
            let v4526 = v4419[0];
            let v4527 = v4419[1];
            let v4528 = v4420[0];
            let v4529 = v4420[1];
            let v4530 = v4420[2];
            let v4531 = v4421[0];
            let v4532 = v4422[0];
            let v4533 = v4423[0];
            let v4534 = v4406[0];
            let v4535 = v4406[1];
            let v4536 = v4406[2];
            let v4537 = v4406[3];
            let v4538 = v4407[0];
            let v4539 = v4407[1];
            let v4540 = v4407[2];
            let v4541 = v4408[0];
            let v4542 = v4408[1];
            let v4543 = v4408[2];
            let v4544 = v4409[0];
            let v4545 = v4409[1];
            let v4546 = v4409[2];
            let v4547 = v4410[0];
            let v4548 = v4410[1];
            let v4549 = v4410[2];
            let v4550 = v4410[3];
            let v4551 = v4410[4];
            let v4552 = v4400[0];
            let v4553 = v4400[1];
            let v4554 = v4401[0];
            let v4555 = v4401[1];
            let v4556 = v4411[0];
            let v4557 = v4411[1];
            let v4558 = v4411[2];
            let v4559 = v4403[0];
            let v4560 = v4405[0];
            let v4561 = v4402[0];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1559),
            [4, 6, 8, 9],
            [v4424, v4425, v4426, v4427],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1560),
            [4, 7, 8, 9],
            [v4428, v4429, v4430, v4431],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(6),
            Some(9),
            multiplicity * (v1561),
            [13],
            [v4432],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(9),
            Some(6),
            multiplicity * (v1562),
            [4, 6, 8, 9],
            [v4433, v4434, v4435, v4436],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1563),
            [4, 6, 8, 9, 13],
            [v4437, v4438, v4439, v4440, v4441],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(5),
            multiplicity * (v1564),
            [0, 4, 5, 7],
            [v4442, v4443, v4444, v4445],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1565),
            [4, 7, 10],
            [v4446, v4447, v4448],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(0),
            Some(5),
            multiplicity * (v1344),
            [0, 4, 5],
            [v4449, v4450, v4451],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (v1566),
            [4, 5, 6, 8],
            [v4452, v4453, v4454, v4455],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(7),
            multiplicity * (v1368),
            [1, 4, 7],
            [v4456, v4457, v4458],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1370),
            [4, 6, 7, 8, 9],
            [v4459, v4460, v4461, v4462, v4463],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(9),
            multiplicity * (v1371),
            [2, 4, 9],
            [v4464, v4465, v4466],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1374),
            [4, 5, 6, 7, 8, 10],
            [v4467, v4468, v4469, v4470, v4471, v4472],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1567),
            [4, 10, 11],
            [v4473, v4474, v4475],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (v1568),
            [4, 6, 7, 8, 10, 11],
            [v4476, v4477, v4478, v4479, v4480, v4481],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(11),
            multiplicity * (v1375),
            [3, 4, 11],
            [v4482, v4483, v4484],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            None,
            multiplicity * (v1543),
            [4, 6, 8, 9, 13],
            [v4485, v4486, v4487, v4488, v4489],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(13),
            None,
            multiplicity * (v1544),
            [12, 13],
            [v4490, v4491],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v1542),
            [4],
            [v4492],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<13, 0>(
            Some(4),
            None,
            multiplicity * (v1541),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13],
            [v4493, v4494, v4495, v4496, v4497, v4498, v4499, v4500, v4501, v4502, v4503, v4504, v4505],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1938),
            [4, 6, 8, 9],
            [v4506, v4507, v4508, v4509],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1939),
            [4, 7, 9],
            [v4510, v4511, v4512],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(6),
            multiplicity * (v1940),
            [4, 6, 8],
            [v4513, v4514, v4515],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(8),
            Some(5),
            multiplicity * (v1941),
            [4, 5, 8],
            [v4516, v4517, v4518],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1942),
            [4, 6, 7, 8, 10],
            [v4519, v4520, v4521, v4522, v4523],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v1943),
            [1, 2],
            [v4524, v4525],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (v1944),
            [0, 1],
            [v4526, v4527],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(10),
            multiplicity * (v1945),
            [4, 10, 11],
            [v4528, v4529, v4530],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(12),
            None,
            multiplicity * (v1946),
            [12],
            [v4531],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(13),
            None,
            multiplicity * (v1947),
            [13],
            [v4532],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v1948),
            [4],
            [v4533],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1965),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(9),
            multiplicity * (v1966),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1967),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(9),
            multiplicity * (v1968),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(9),
            multiplicity * (v1969),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1970),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(10),
            multiplicity * (v1971),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(5),
            multiplicity * (v1972),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(5),
            Some(6),
            multiplicity * (v1973),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(7),
            multiplicity * (v1974),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1975),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(9),
            multiplicity * (v1976),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1977),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(11),
            multiplicity * (v1978),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(3),
            Some(11),
            multiplicity * (v1979),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        self.canonical_reactive[0] = v1559;
        self.canonical_reactive[1] = v1560;
        self.canonical_reactive[2] = v1561;
        self.canonical_reactive[3] = v1562;
        self.canonical_reactive[4] = v1563;
        self.canonical_reactive[5] = v1564;
        self.canonical_reactive[6] = v1565;
        self.canonical_reactive[7] = v1344;
        self.canonical_reactive[8] = v1566;
        self.canonical_reactive[9] = v1368;
        self.canonical_reactive[10] = v1370;
        self.canonical_reactive[11] = v1371;
        self.canonical_reactive[12] = v1374;
        self.canonical_reactive[13] = v1567;
        self.canonical_reactive[14] = v1568;
        self.canonical_reactive[15] = v1375;
        self.canonical_reactive[16] = v1543;
        self.canonical_reactive[17] = v1544;
        self.canonical_reactive[18] = v1542;
        self.canonical_reactive[19] = v1541;
        self.canonical_reactive[20] = v1932;
        self.canonical_reactive[21] = v4534;
        self.canonical_reactive[22] = v4535;
        self.canonical_reactive[23] = v4536;
        self.canonical_reactive[24] = v4537;
        self.canonical_reactive[25] = v1933;
        self.canonical_reactive[26] = v4538;
        self.canonical_reactive[27] = v4539;
        self.canonical_reactive[28] = v4540;
        self.canonical_reactive[29] = v1934;
        self.canonical_reactive[30] = v4541;
        self.canonical_reactive[31] = v4542;
        self.canonical_reactive[32] = v4543;
        self.canonical_reactive[33] = v1935;
        self.canonical_reactive[34] = v4544;
        self.canonical_reactive[35] = v4545;
        self.canonical_reactive[36] = v4546;
        self.canonical_reactive[37] = v1936;
        self.canonical_reactive[38] = v4547;
        self.canonical_reactive[39] = v4548;
        self.canonical_reactive[40] = v4549;
        self.canonical_reactive[41] = v4550;
        self.canonical_reactive[42] = v4551;
        self.canonical_reactive[43] = v1922;
        self.canonical_reactive[44] = v4552;
        self.canonical_reactive[45] = v4553;
        self.canonical_reactive[46] = v1924;
        self.canonical_reactive[47] = v4554;
        self.canonical_reactive[48] = v4555;
        self.canonical_reactive[49] = v1937;
        self.canonical_reactive[50] = v4556;
        self.canonical_reactive[51] = v4557;
        self.canonical_reactive[52] = v4558;
        self.canonical_reactive[53] = v1928;
        self.canonical_reactive[54] = v4559;
        self.canonical_reactive[55] = v1931;
        self.canonical_reactive[56] = v4560;
        self.canonical_reactive[57] = v1926;
        self.canonical_reactive[58] = v4561;
        self.canonical_reactive[59] = v1965;
        self.canonical_reactive[60] = v1966;
        self.canonical_reactive[61] = v1967;
        self.canonical_reactive[62] = v1968;
        self.canonical_reactive[63] = v1969;
        self.canonical_reactive[64] = v1970;
        self.canonical_reactive[65] = v1971;
        self.canonical_reactive[66] = v1972;
        self.canonical_reactive[67] = v1973;
        self.canonical_reactive[68] = v1974;
        self.canonical_reactive[69] = v1975;
        self.canonical_reactive[70] = v1976;
        self.canonical_reactive[71] = v1977;
        self.canonical_reactive[72] = v1978;
        self.canonical_reactive[73] = v1979;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(9),
            &[4, 6, 8, 9],
            &[cached[21], cached[22], cached[23], cached[24]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(9),
            &[4, 7, 9],
            &[cached[26], cached[27], cached[28]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(6),
            &[4, 6, 8],
            &[cached[30], cached[31], cached[32]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(8),
            Some(5),
            &[4, 5, 8],
            &[cached[34], cached[35], cached[36]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(10),
            &[4, 6, 7, 8, 10],
            &[cached[38], cached[39], cached[40], cached[41], cached[42]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[44], cached[45]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[47], cached[48]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(11),
            Some(10),
            &[4, 10, 11],
            &[cached[50], cached[51], cached[52]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(12),
            None,
            &[12],
            &[cached[54]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(13),
            None,
            &[13],
            &[cached[56]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[58]],
            &[],
            &[],
            multiplicity,
        );
    }

}

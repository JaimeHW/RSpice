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
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12])];
        let ddt_scale_value = self.ddt_coefficients.derivative_scale;
        let ddt_scale = move || ddt_scale_value;
        let ddt_state = self.stamp_state.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_coefficients = self.ddt_coefficients;
        let mut ddt = |operator: usize, value: f64| -> f64 {
            let _ = operator;
            let slot = match operator { 13541 => 0usize, 13609 => 1usize, 13615 => 2usize, 13625 => 3usize, 13631 => 4usize, 13637 => 5usize, 13645 => 6usize, 13653 => 7usize, 13673 => 8usize, 13692 => 9usize, 13963 => 10usize, _ => usize::MAX };
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
            let v8 = parameters[33];
            let v10 = parameters[4];
            let v11 = 2.7315e2f64;
            let v13 = temperature;
            let v14 = parameters[0];
            let v17 = parameters[154];
            let v19 = 1e-12f64;
            let v21 = parameters[1];
            let v24 = parameters[134];
            let v26 = 1e-3f64;
            let v27 = 2e0f64;
            let v28 = parameters[67];
            let v32 = parameters[114];
            let v33 = parameters[115];
            let v36 = parameters[116];
            let v40 = 5e-2f64;
            let v42 = 1e-1f64;
            let v57 = parameters[66];
            let v59 = parameters[71];
            let v60 = parameters[72];
            let v64 = parameters[117];
            let v65 = parameters[118];
            let v68 = parameters[119];
            let v88 = parameters[83];
            let v91 = node_potentials[4];
            let v97 = parameters[125];
            let v106 = 8.617086918058125e-5f64;
            let v154 = 3e0f64;
            let v155 = -3e0f64;
            let v161 = parameters[105];
            let v178 = -3e0f64;
            let v181 = parameters[64];
            let v184 = parameters[110];
            let v201 = -3e0f64;
            let v204 = parameters[80];
            let v222 = -3e0f64;
            let v242 = -3e0f64;
            let v261 = -3e0f64;
            let v264 = parameters[27];
            let v267 = parameters[109];
            let v284 = -3e0f64;
            let v287 = parameters[138];
            let v290 = parameters[140];
            let v315 = parameters[65];
            let v317 = parameters[137];
            let v320 = parameters[139];
            let v323 = parameters[75];
            let v331 = parameters[70];
            let v334 = parameters[54];
            let v335 = parameters[97];
            let v340 = parameters[56];
            let v341 = parameters[98];
            let v342 = parameters[96];
            let v347 = parameters[55];
            let v348 = parameters[101];
            let v353 = parameters[57];
            let v354 = parameters[102];
            let v358 = parameters[58];
            let v359 = parameters[104];
            let v363 = parameters[59];
            let v365 = parameters[60];
            let v366 = parameters[99];
            let v370 = parameters[122];
            let v372 = parameters[10];
            let v391 = 6.931471805599453e-4f64;
            let v393 = parameters[123];
            let v395 = parameters[11];
            let v414 = 6.931471805599453e-4f64;
            let v416 = parameters[43];
            let v417 = parameters[124];
            let v421 = 1e-6f64;
            let v424 = 5e-1f64;
            let v425 = 5e-7f64;
            let v434 = parameters[9];
            let v435 = 4e0f64;
            let v438 = parameters[121];
            let v450 = parameters[12];
            let v455 = parameters[30];
            let v456 = parameters[103];
            let v461 = parameters[20];
            let v462 = 6e0f64;
            let v463 = parameters[21];
            let v469 = parameters[113];
            let v475 = parameters[31];
            let v476 = parameters[32];
            let v487 = parameters[16];
            let v491 = parameters[17];
            let v495 = parameters[111];
            let v501 = parameters[18];
            let v502 = parameters[19];
            let v509 = parameters[24];
            let v511 = parameters[25];
            let v512 = parameters[107];
            let v518 = parameters[28];
            let v519 = parameters[106];
            let v524 = parameters[26];
            let v525 = parameters[108];
            let v531 = parameters[29];
            let v537 = parameters[112];
            let v542 = parameters[22];
            let v543 = parameters[23];
            let v552 = parameters[149];
            let v553 = parameters[150];
            let v561 = parameters[155];
            let v564 = parameters[157];
            let v570 = -5e-1f64;
            let v573 = parameters[35];
            let v582 = parameters[34];
            let v594 = -5e-1f64;
            let v597 = parameters[37];
            let v606 = parameters[36];
            let v618 = parameters[14];
            let v621 = parameters[13];
            let v624 = parameters[133];
            let v625 = parameters[141];
            let v634 = 3.5e0f64;
            let v635 = parameters[142];
            let v642 = parameters[135];
            let v647 = parameters[136];
            let v652 = parameters[86];
            let v657 = parameters[120];
            let v662 = parameters[87];
            let v668 = parameters[88];
            let v673 = parameters[89];
            let v678 = parameters[90];
            let v679 = parameters[100];
            let v684 = 3e2f64;
            let v686 = 5.25e2f64;
            let v689 = 7.2e-4f64;
            let v692 = 1.6e-6f64;
            let v697 = 1.081e0f64;
            let v699 = parameters[92];
            let v701 = parameters[146];
            let v703 = parameters[148];
            let v715 = node_potentials[7];
            let v716 = node_potentials[8];
            let v719 = node_potentials[9];
            let v722 = node_potentials[5];
            let v725 = node_potentials[6];
            let v730 = node_potentials[3];
            let v735 = node_potentials[2];
            let v738 = node_potentials[1];
            let v743 = node_potentials[0];
            let v746 = node_potentials[11];
            let v749 = node_potentials[10];
            let v763 = parameters[151];
            let v864 = parameters[153];
            let v875 = 1e2f64;
            let v891 = 2e-1f64;
            let v906 = parameters[62];
            let v907 = parameters[61];
            let v917 = parameters[63];
            let v932 = -1e0f64;
            let v975 = parameters[152];
            let v993 = parameters[73];
            let v1009 = 1e-5f64;
            let v1013 = 1e-40f64;
            let v1029 = -1e0f64;
            let v1060 = parameters[74];
            let v1068 = -1e0f64;
            let v1092 = parameters[76];
            let v1147 = 1.0000000000000002e-2f64;
            let v1151 = 5.000000000000001e-3f64;
            let v1165 = parameters[15];
            let v1171 = 1e-4f64;
            let v1185 = parameters[156];
            let v1196 = parameters[158];
            let v1211 = parameters[159];
            let v1234 = 1e3f64;
            let v1236 = 4e1f64;
            let v1239 = 2.3538526683702e17f64;
            let v1267 = parameters[93];
            let v1369 = 1e-30f64;
            let v1372 = -2e0f64;
            let v1388 = 1.6666666666666666e-1f64;
            let v1394 = -1e-3f64;
            let v1410 = 3.333333333333333e-1f64;
            let v1412 = 2.5e-1f64;
            let v1447 = -2e0f64;
            let v1468 = -1e-3f64;
            let v1521 = parameters[8];
            let v1523 = parameters[143];
            let v1531 = parameters[144];
            let v1586 = parameters[5];
            let v1640 = 1.21e-2f64;
            let v1643 = 6.05e-3f64;
            let v1662 = parameters[84];
            let v1665 = 1e-6f64;
            let v1666 = 1e-12f64;
            let v1667 = -1e0f64;
            let v1669 = -1e0f64;
            let v1672 = -1e0f64;
            let v1675 = 5e-13f64;
            let v1678 = -1e0f64;
            let v1684 = -1e0f64;
            let v1688 = parameters[82];
            let v1692 = parameters[81];
            let v1722 = 1.0000000000000002e-2f64;
            let v1725 = 5.000000000000001e-3f64;
            let v1747 = parameters[39];
            let v1749 = parameters[44];
            let v1752 = parameters[42];
            let v1765 = parameters[41];
            let v1774 = parameters[40];
            let v1781 = parameters[46];
            let v1783 = parameters[45];
            let v1791 = parameters[7];
            let v1811 = parameters[47];
            let v1841 = 1e-7f64;
            let v1867 = parameters[48];
            let v1871 = parameters[49];
            let v1875 = parameters[52];
            let v1879 = parameters[51];
            let v1894 = parameters[50];
            let v1918 = parameters[53];
            let v2029 = parameters[68];
            let v2058 = parameters[77];
            let v2128 = -1e0f64;
            let v2161 = parameters[85];
            let v2184 = parameters[79];
            let v2194 = parameters[91];
            let v2257 = parameters[6];
            let v2285 = parameters[95];
            let v2290 = parameters[94];
            let v2319 = -1e0f64;
            let v2329 = parameters[147];
            let v2335 = parameters[145];
            let v2359 = -1e0f64;
            let v2385 = parameters[69];
            let v2390 = parameters[78];
            let v2417 = 0e0f64;
            let v2421 = 0e0f64;
            let v2424 = parameters[130];
            let v2434 = parameters[131];
            let v2439 = parameters[132];
            let v2450 = 0e0f64;
            let v2451 = node_potentials[12];
            let v2457 = 0e0f64;
            let v2458 = 0e0f64;
            let v2459 = 0e0f64;
            let v2460 = 0e0f64;
            let v2461 = 0e0f64;
            let v2462 = 0e0f64;
            let v2463 = 0e0f64;
            let v2464 = 0e0f64;
            let v2465 = 0e0f64;
            let v2466 = 0e0f64;
            let v2467 = 0e0f64;
            let v2468 = 0e0f64;
            let v2469 = 0e0f64;
            let v2470 = 0e0f64;
            let v2471 = 0e0f64;
            let v2472 = 0e0f64;
            let v2473 = 0e0f64;
            let v2474 = 0e0f64;
            let v2475 = 0e0f64;
            let v2476 = 0e0f64;
            let v2477 = 0e0f64;
            let v2478 = 0e0f64;
            let v2479 = 0e0f64;
            let v2480 = 0e0f64;
            let v2481 = 0e0f64;
            let v2482 = 0e0f64;
            let v2483 = 0e0f64;
            let v2515 = 1e0f64;
            let v2516 = Lanes([1e0f64; 1]);
            let v2517 = Lanes([1e0f64; 1]);
            let v2518 = Lanes([1e0f64; 1]);
            let v2519 = Lanes([1e0f64; 1]);
            let v2520 = Lanes([1e0f64; 1]);
            let v2521 = Lanes([1e0f64; 1]);
            let v2522 = Lanes([1e0f64; 1]);
            let v2523 = Lanes([1e0f64; 1]);
            let v2524 = Lanes([1e0f64; 1]);
            let v2525 = Lanes([1e0f64; 1]);
            let v2526 = Lanes([1e0f64; 1]);
            let v2527 = Lanes([1e0f64; 1]);
            let v2528 = Lanes([1e0f64; 1]);
            let v2676 = -1e0f64;
            let v2947 = Lanes([0e0f64; 1]);
            let v2994 = 2e0f64;
            let v3108 = -1.5e0f64;
            let v3149 = -1.5e0f64;
            let v3443 = Lanes([0e0f64; 3]);
            let v3468 = 0e0f64;
            let v3489 = Lanes([0e0f64; 4]);
            let v4087 = Lanes([0e0f64; 3]);
            let v4422 = Lanes([0e0f64; 10]);
            let v4535 = Lanes([0e0f64; 3]);
            let v4617 = Lanes([0e0f64; 5]);
            let v5439 = Lanes([0e0f64; 6]);
            let v5514 = Lanes([0e0f64; 4]);
            let v5546 = ddt_scale();
            let v5615 = Lanes([0e0f64; 3]);
            let v5623 = Lanes([0e0f64; 3]);
            let v3 = if v1 == v2 { 1.0 } else { 0.0 };
            let v688: f64;
            let v1844: f64;
            if v3 != 0.0 {
                v688 = v5;
                v1844 = v4;
            } else {
                v688 = v7;
                v1844 = v6;
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
            let v25 = if v24 > v0 { 1.0 } else { 0.0 };
            let v1583: f64;
            if v25 != 0.0 {
                v1583 = v16;
            } else {
                v1583 = v0;
            }
            let v30 = v27.powf((v27 - v28));
            let v31 = v2 / v30;
            let v39 = v32 + (((v33 * v12) * v12) / (v12 + v36));
            let v43 = (v39 - v40) / v42;
            let v44 = if v39 < v40 { 1.0 } else { 0.0 };
            let v114: f64;
            if v44 != 0.0 {
                let v49 = v40 + (v42 * ((v2 + (v43.exp())).ln()));
                v114 = v49;
            } else {
                let v55 = v39 + (v42 * ((v2 + ((-v43).exp())).ln()));
                v114 = v55;
            }
            let v56 = v2 / v32;
            let v58 = v2 / v57;
            let v62 = v27.powf((v27 - v60));
            let v63 = v2 / v62;
            let v71 = v64 + (((v65 * v12) * v12) / (v12 + v68));
            let v73 = (v71 - v40) / v42;
            let v74 = if v71 < v40 { 1.0 } else { 0.0 };
            let v134: f64;
            if v74 != 0.0 {
                let v79 = v40 + (v42 * ((v2 + (v73.exp())).ln()));
                v134 = v79;
            } else {
                let v85 = v71 + (v42 * ((v2 + ((-v73).exp())).ln()));
                v134 = v85;
            }
            let v86 = v2 / v64;
            let v87 = v2 / v59;
            let v90 = v2 - (v2 / v88);
            let v92 = if v91 < v0 { 1.0 } else { 0.0 };
            let v96: f64;
            let v2529: Lanes<1>;
            if v92 != 0.0 {
                let v93 = v2 - v91;
                let v95 = -(v93.ln());
                let v2680 = ((v2516 * v2676) * (v2515 / v93)) * v2676;
                v96 = v95;
                v2529 = v2680;
            } else {
                v96 = v91;
                v2529 = v2516;
            }
            let v98 = if v96 < v97 { 1.0 } else { 0.0 };
            let v103: f64;
            let v2530: Lanes<1>;
            if v98 != 0.0 {
                v103 = v96;
                v2530 = v2529;
            } else {
                let v100 = v2 + (v96 - v97);
                let v2682 = v2529 * (v2515 / v100);
                let v102 = v97 + (v100.ln());
                v103 = v102;
                v2530 = v2682;
            }
            let v104 = v15 + v103;
            let v105 = v104 / v12;
            let v2683 = v2530 / v12;
            let v107 = v106 * v104;
            let v2684 = v2530 * v106;
            let v109 = v2 / v107;
            let v2687 = ((v2684 * v109) * v2676) / v107;
            let v111 = v109 - (v2 / (v106 * v12));
            let v112 = v104 - v12;
            let v113 = v105.ln();
            let v2689 = v2683 * (v2515 / v105);
            let v115 = v33 * v104;
            let v117 = v104 + v36;
            let v118 = (v115 * v104) / v117;
            let v119 = v114 - v118;
            let v2697 = (((((v2530 * v33) * v104) + (v2530 * v115)) - (v2530 * v118)) / v117) * v2676;
            let v121 = (v119 - v40) / v42;
            let v2698 = v2697 / v42;
            let v122 = if v119 < v40 { 1.0 } else { 0.0 };
            let v568: f64;
            let v2531: Lanes<1>;
            if v122 != 0.0 {
                let v123 = v121.exp();
                let v124 = v2 + v123;
                let v2708 = ((v2698 * v123) * (v2515 / v124)) * v42;
                let v127 = v40 + (v42 * (v124.ln()));
                v568 = v127;
                v2531 = v2708;
            } else {
                let v129 = (-v121).exp();
                let v130 = v2 + v129;
                let v133 = v119 + (v42 * (v130.ln()));
                let v2704 = v2697 + ((((v2698 * v2676) * v129) * (v2515 / v130)) * v42);
                v568 = v133;
                v2531 = v2704;
            }
            let v135 = v65 * v104;
            let v137 = v104 + v68;
            let v138 = (v135 * v104) / v137;
            let v139 = v134 - v138;
            let v2716 = (((((v2530 * v65) * v104) + (v2530 * v135)) - (v2530 * v138)) / v137) * v2676;
            let v141 = (v139 - v40) / v42;
            let v2717 = v2716 / v42;
            let v142 = if v139 < v40 { 1.0 } else { 0.0 };
            let v592: f64;
            let v2532: Lanes<1>;
            if v142 != 0.0 {
                let v143 = v141.exp();
                let v144 = v2 + v143;
                let v2727 = ((v2717 * v143) * (v2515 / v144)) * v42;
                let v147 = v40 + (v42 * (v144.ln()));
                v592 = v147;
                v2532 = v2727;
            } else {
                let v149 = (-v141).exp();
                let v150 = v2 + v149;
                let v153 = v139 + (v42 * (v150.ln()));
                let v2723 = v2716 + ((((v2717 * v2676) * v149) * (v2515 / v150)) * v42);
                v592 = v153;
                v2532 = v2723;
            }
            let v156 = v155 * v107;
            let v160 = v2 - v105;
            let v2734 = v2683 * v2676;
            let v163 = ((v156 * v113) + (v57 * v105)) + (v160 * v161);
            let v2736 = ((((v2684 * v155) * v113) + (v2689 * v156)) + (v2683 * v57)) + (v2734 * v161);
            let v165 = (v40 - v163) / v107;
            let v2740 = ((v2736 * v2676) - (v2684 * v165)) / v107;
            let v166 = if v40 < v163 { 1.0 } else { 0.0 };
            let v307: f64;
            let v2533: Lanes<1>;
            if v166 != 0.0 {
                let v167 = v165.exp();
                let v168 = v2 + v167;
                let v169 = v168.ln();
                let v171 = v163 + (v107 * v169);
                let v2754 = v2736 + ((v2684 * v169) + (((v2740 * v167) * (v2515 / v168)) * v107));
                v307 = v171;
                v2533 = v2754;
            } else {
                let v173 = (-v165).exp();
                let v174 = v2 + v173;
                let v175 = v174.ln();
                let v2747 = (v2684 * v175) + ((((v2740 * v2676) * v173) * (v2515 / v174)) * v107);
                let v177 = v40 + (v107 * v175);
                v307 = v177;
                v2533 = v2747;
            }
            let v179 = v178 * v107;
            let v185 = v160 * v184;
            let v2761 = v2734 * v184;
            let v186 = ((v179 * v113) + (v181 * v105)) + v185;
            let v2762 = ((((v2684 * v178) * v113) + (v2689 * v179)) + (v2683 * v181)) + v2761;
            let v188 = (v40 - v186) / v107;
            let v2766 = ((v2762 * v2676) - (v2684 * v188)) / v107;
            let v189 = if v40 < v186 { 1.0 } else { 0.0 };
            let v820: f64;
            let v2534: Lanes<1>;
            if v189 != 0.0 {
                let v190 = v188.exp();
                let v191 = v2 + v190;
                let v192 = v191.ln();
                let v194 = v186 + (v107 * v192);
                let v2780 = v2762 + ((v2684 * v192) + (((v2766 * v190) * (v2515 / v191)) * v107));
                v820 = v194;
                v2534 = v2780;
            } else {
                let v196 = (-v188).exp();
                let v197 = v2 + v196;
                let v198 = v197.ln();
                let v2773 = (v2684 * v198) + ((((v2766 * v2676) * v196) * (v2515 / v197)) * v107);
                let v200 = v40 + (v107 * v198);
                v820 = v200;
                v2534 = v2773;
            }
            let v202 = v201 * v107;
            let v207 = ((v202 * v113) + (v204 * v105)) + v185;
            let v2787 = ((((v2684 * v201) * v113) + (v2689 * v202)) + (v2683 * v204)) + v2761;
            let v209 = (v40 - v207) / v107;
            let v2791 = ((v2787 * v2676) - (v2684 * v209)) / v107;
            let v210 = if v40 < v207 { 1.0 } else { 0.0 };
            let v2192: f64;
            let v2535: Lanes<1>;
            if v210 != 0.0 {
                let v211 = v209.exp();
                let v212 = v2 + v211;
                let v213 = v212.ln();
                let v215 = v207 + (v107 * v213);
                let v2805 = v2787 + ((v2684 * v213) + (((v2791 * v211) * (v2515 / v212)) * v107));
                v2192 = v215;
                v2535 = v2805;
            } else {
                let v217 = (-v209).exp();
                let v218 = v2 + v217;
                let v219 = v218.ln();
                let v2798 = (v2684 * v219) + ((((v2791 * v2676) * v217) * (v2515 / v218)) * v107);
                let v221 = v40 + (v107 * v219);
                v2192 = v221;
                v2535 = v2798;
            }
            let v223 = v222 * v107;
            let v225 = v59 * v105;
            let v2810 = v2683 * v59;
            let v227 = ((v223 * v113) + v225) + v185;
            let v2812 = ((((v2684 * v222) * v113) + (v2689 * v223)) + v2810) + v2761;
            let v229 = (v40 - v227) / v107;
            let v2816 = ((v2812 * v2676) - (v2684 * v229)) / v107;
            let v230 = if v40 < v227 { 1.0 } else { 0.0 };
            let v325: f64;
            let v2536: Lanes<1>;
            if v230 != 0.0 {
                let v231 = v229.exp();
                let v232 = v2 + v231;
                let v233 = v232.ln();
                let v235 = v227 + (v107 * v233);
                let v2830 = v2812 + ((v2684 * v233) + (((v2816 * v231) * (v2515 / v232)) * v107));
                v325 = v235;
                v2536 = v2830;
            } else {
                let v237 = (-v229).exp();
                let v238 = v2 + v237;
                let v239 = v238.ln();
                let v2823 = (v2684 * v239) + ((((v2816 * v2676) * v237) * (v2515 / v238)) * v107);
                let v241 = v40 + (v107 * v239);
                v325 = v241;
                v2536 = v2823;
            }
            let v243 = v242 * v107;
            let v246 = ((v243 * v113) + v225) + v185;
            let v2836 = ((((v2684 * v242) * v113) + (v2689 * v243)) + v2810) + v2761;
            let v248 = (v40 - v246) / v107;
            let v2840 = ((v2836 * v2676) - (v2684 * v248)) / v107;
            let v249 = if v40 < v246 { 1.0 } else { 0.0 };
            let v309: f64;
            let v2537: Lanes<1>;
            if v249 != 0.0 {
                let v250 = v248.exp();
                let v251 = v2 + v250;
                let v252 = v251.ln();
                let v254 = v246 + (v107 * v252);
                let v2854 = v2836 + ((v2684 * v252) + (((v2840 * v250) * (v2515 / v251)) * v107));
                v309 = v254;
                v2537 = v2854;
            } else {
                let v256 = (-v248).exp();
                let v257 = v2 + v256;
                let v258 = v257.ln();
                let v2847 = (v2684 * v258) + ((((v2840 * v2676) * v256) * (v2515 / v257)) * v107);
                let v260 = v40 + (v107 * v258);
                v309 = v260;
                v2537 = v2847;
            }
            let v262 = v261 * v107;
            let v269 = ((v262 * v113) + (v264 * v105)) + (v160 * v267);
            let v2862 = ((((v2684 * v261) * v113) + (v2689 * v262)) + (v2683 * v264)) + (v2734 * v267);
            let v271 = (v40 - v269) / v107;
            let v2866 = ((v2862 * v2676) - (v2684 * v271)) / v107;
            let v272 = if v40 < v269 { 1.0 } else { 0.0 };
            let v1224: f64;
            let v2538: Lanes<1>;
            if v272 != 0.0 {
                let v273 = v271.exp();
                let v274 = v2 + v273;
                let v275 = v274.ln();
                let v277 = v269 + (v107 * v275);
                let v2880 = v2862 + ((v2684 * v275) + (((v2866 * v273) * (v2515 / v274)) * v107));
                v1224 = v277;
                v2538 = v2880;
            } else {
                let v279 = (-v271).exp();
                let v280 = v2 + v279;
                let v281 = v280.ln();
                let v2873 = (v2684 * v281) + ((((v2866 * v2676) * v279) * (v2515 / v280)) * v107);
                let v283 = v40 + (v107 * v281);
                v1224 = v283;
                v2538 = v2873;
            }
            let v285 = v284 * v107;
            let v292 = ((v285 * v113) + (v287 * v105)) + (v160 * v290);
            let v2888 = ((((v2684 * v284) * v113) + (v2689 * v285)) + (v2683 * v287)) + (v2734 * v290);
            let v294 = (v40 - v292) / v107;
            let v2892 = ((v2888 * v2676) - (v2684 * v294)) / v107;
            let v295 = if v40 < v292 { 1.0 } else { 0.0 };
            let v318: f64;
            let v2539: Lanes<1>;
            if v295 != 0.0 {
                let v296 = v294.exp();
                let v297 = v2 + v296;
                let v298 = v297.ln();
                let v300 = v292 + (v107 * v298);
                let v2906 = v2888 + ((v2684 * v298) + (((v2892 * v296) * (v2515 / v297)) * v107));
                v318 = v300;
                v2539 = v2906;
            } else {
                let v302 = (-v294).exp();
                let v303 = v2 + v302;
                let v304 = v303.ln();
                let v2899 = (v2684 * v304) + ((((v2892 * v2676) * v302) * (v2515 / v303)) * v107);
                let v306 = v40 + (v107 * v304);
                v318 = v306;
                v2539 = v2899;
            }
            let v308 = v2 / v307;
            let v2909 = ((v2533 * v308) * v2676) / v307;
            let v310 = v2 / v309;
            let v2912 = ((v2537 * v310) * v2676) / v309;
            let v311 = v57 * v308;
            let v312 = v311.powf(v28);
            let v2917 = (v2909 * v57) * (v28 * (v311.powf((v28 - v2515))));
            let v313 = v59 * v310;
            let v314 = v313.powf(v60);
            let v2919 = v60 - v2515;
            let v2922 = (v2912 * v59) * (v60 * (v313.powf(v2919)));
            let v316 = v315 * v312;
            let v2923 = v2917 * v315;
            let v319 = v287 / v318;
            let v322 = v317 * (v319.powf(v320));
            let v2931 = ((((v2539 * v319) * v2676) / v318) * (v320 * (v319.powf((v320 - v2515))))) * v317;
            let v324 = v2 - v323;
            let v326 = v59 / v325;
            let v2938 = ((((v2536 * v326) * v2676) / v325) * (v60 * (v326.powf(v2919)))) * v324;
            let v329 = (v324 * (v326.powf(v60))) + v323;
            let v330 = v2 / v329;
            let v2941 = ((v2938 * v330) * v2676) / v329;
            let v332 = v331 * v329;
            let v2942 = v2938 * v331;
            let v333 = v323 * v330;
            let v2943 = v2941 * v323;
            let v337 = (v113 * v335).exp();
            let v338 = v334 * v337;
            let v2946 = ((v2689 * v335) * v337) * v334;
            let v339 = if v338 < v22 { 1.0 } else { 0.0 };
            let v1927: f64;
            let v2540: Lanes<1>;
            if v339 != 0.0 {
                v1927 = v22;
                v2540 = v2947;
            } else {
                v1927 = v338;
                v2540 = v2946;
            }
            let v343 = v341 - v342;
            let v345 = (v113 * v343).exp();
            let v346 = v340 * v345;
            let v2950 = ((v2689 * v343) * v345) * v340;
            let v350 = (v113 * v348).exp();
            let v351 = v347 * v350;
            let v2953 = ((v2689 * v348) * v350) * v347;
            let v352 = if v351 < v22 { 1.0 } else { 0.0 };
            let v1920: f64;
            let v2541: Lanes<1>;
            if v352 != 0.0 {
                v1920 = v22;
                v2541 = v2947;
            } else {
                v1920 = v351;
                v2541 = v2953;
            }
            let v356 = (v113 * v354).exp();
            let v357 = v353 * v356;
            let v2956 = ((v2689 * v354) * v356) * v353;
            let v361 = (v113 * v359).exp();
            let v2958 = (v2689 * v359) * v361;
            let v362 = v358 * v361;
            let v2959 = v2958 * v358;
            let v364 = v363 * v361;
            let v2960 = v2958 * v363;
            let v368 = (v113 * v366).exp();
            let v369 = v365 * v368;
            let v2963 = ((v2689 * v366) * v368) * v365;
            let v371 = if v370 != v0 { 1.0 } else { 0.0 };
            let v441: f64;
            let v2542: Lanes<1>;
            if v371 != 0.0 {
                let v375 = v372 * (v2 + (v112 * v370));
                let v2965 = (v2530 * v370) * v372;
                let v377 = (v375 - v2) / v26;
                let v2966 = v2965 / v26;
                let v378 = if v375 < v2 { 1.0 } else { 0.0 };
                let v390: f64;
                let v2543: Lanes<1>;
                if v378 != 0.0 {
                    let v379 = v377.exp();
                    let v380 = v2 + v379;
                    let v2976 = ((v2966 * v379) * (v2515 / v380)) * v26;
                    let v383 = v2 + (v26 * (v380.ln()));
                    v390 = v383;
                    v2543 = v2976;
                } else {
                    let v385 = (-v377).exp();
                    let v386 = v2 + v385;
                    let v389 = v375 + (v26 * (v386.ln()));
                    let v2972 = v2965 + ((((v2966 * v2676) * v385) * (v2515 / v386)) * v26);
                    v390 = v389;
                    v2543 = v2972;
                }
                let v392 = v390 - v391;
                v441 = v392;
                v2542 = v2543;
            } else {
                v441 = v372;
                v2542 = v2947;
            }
            let v394 = if v393 != v0 { 1.0 } else { 0.0 };
            let v1119: f64;
            let v2544: Lanes<1>;
            if v394 != 0.0 {
                let v398 = v395 * (v2 + (v112 * v393));
                let v2978 = (v2530 * v393) * v395;
                let v400 = (v398 - v2) / v26;
                let v2979 = v2978 / v26;
                let v401 = if v398 < v2 { 1.0 } else { 0.0 };
                let v413: f64;
                let v2545: Lanes<1>;
                if v401 != 0.0 {
                    let v402 = v400.exp();
                    let v403 = v2 + v402;
                    let v2989 = ((v2979 * v402) * (v2515 / v403)) * v26;
                    let v406 = v2 + (v26 * (v403.ln()));
                    v413 = v406;
                    v2545 = v2989;
                } else {
                    let v408 = (-v400).exp();
                    let v409 = v2 + v408;
                    let v412 = v398 + (v26 * (v409.ln()));
                    let v2985 = v2978 + ((((v2979 * v2676) * v408) * (v2515 / v409)) * v26);
                    v413 = v412;
                    v2545 = v2985;
                }
                let v415 = v413 - v414;
                v1119 = v415;
                v2544 = v2545;
            } else {
                v1119 = v395;
                v2544 = v2947;
            }
            let v420 = v416 * (v2 + (v417 * v112));
            let v2991 = (v2530 * v417) * v416;
            let v422 = v420 * v420;
            let v2992 = v2991 * v420;
            let v2993 = v2992 + v2992;
            let v423 = if v420 < v0 { 1.0 } else { 0.0 };
            let v1763: f64;
            let v2546: Lanes<1>;
            if v423 != 0.0 {
                let v427 = (v422 + v421).sqrt();
                let v428 = v427 - v420;
                let v429 = v425 / v428;
                let v3006 = ((((v2993 * (v2515 / (v2994 * v427))) - v2991) * v429) * v2676) / v428;
                v1763 = v429;
                v2546 = v3006;
            } else {
                let v431 = (v422 + v421).sqrt();
                let v433 = v424 * (v431 + v420);
                let v2999 = ((v2993 * (v2515 / (v2994 * v431))) + v2991) * v424;
                v1763 = v433;
                v2546 = v2999;
            }
            let v439 = ((v435 - v341) - v342) + v438;
            let v442 = (v113 * v439) / v441;
            let v443 = v442.exp();
            let v444 = v434 * v443;
            let v445 = -v161;
            let v447 = (v445 * v111) / v441;
            let v448 = v447.exp();
            let v449 = v444 * v448;
            let v3020 = ((((((v2689 * v439) - (v2542 * v442)) / v441) * v443) * v434) * v448) + (((((v2687 * v445) - (v2542 * v447)) / v441) * v448) * v444);
            let v451 = v2 - v341;
            let v453 = (v113 * v451).exp();
            let v454 = v450 * v453;
            let v3023 = ((v2689 * v451) * v453) * v450;
            let v457 = v2 - v456;
            let v459 = (v113 * v457).exp();
            let v460 = v455 * v459;
            let v3026 = ((v2689 * v457) * v459) * v455;
            let v465 = v462 - (v27 * v463);
            let v467 = (v113 * v465).exp();
            let v468 = v461 * v467;
            let v470 = -v469;
            let v471 = v470 * v111;
            let v3030 = v2687 * v470;
            let v473 = (v471 / v463).exp();
            let v474 = v468 * v473;
            let v3035 = ((((v2689 * v465) * v467) * v461) * v473) + (((v3030 / v463) * v473) * v468);
            let v478 = v462 - (v27 * v476);
            let v480 = (v113 * v478).exp();
            let v481 = v475 * v480;
            let v482 = -v184;
            let v485 = ((v482 * v111) / v476).exp();
            let v486 = v481 * v485;
            let v3044 = ((((v2689 * v478) * v480) * v475) * v485) + ((((v2687 * v482) / v476) * v485) * v481);
            let v489 = (v435 - v335) + v438;
            let v490 = v113 * v489;
            let v3045 = v2689 * v489;
            let v493 = (v490 / v491).exp();
            let v494 = v487 * v493;
            let v496 = -v495;
            let v497 = v496 * v111;
            let v3049 = v2687 * v496;
            let v499 = (v497 / v491).exp();
            let v500 = v494 * v499;
            let v3054 = ((((v3045 / v491) * v493) * v487) * v499) + (((v3049 / v491) * v499) * v494);
            let v504 = (v490 / v502).exp();
            let v505 = v501 * v504;
            let v507 = (v497 / v502).exp();
            let v508 = v505 * v507;
            let v3062 = ((((v3045 / v502) * v504) * v501) * v507) + (((v3049 / v502) * v507) * v505);
            let v510 = if v509 == v2 { 1.0 } else { 0.0 };
            let v1246: f64;
            let v1259: f64;
            let v1301: f64;
            let v2547: Lanes<1>;
            let v2548: Lanes<1>;
            let v2549: Lanes<1>;
            if v510 != 0.0 {
                let v513 = -v512;
                let v516 = ((v513 * v111) / v491).exp();
                let v517 = v511 * v516;
                let v3066 = (((v2687 * v513) / v491) * v516) * v511;
                let v520 = -v519;
                let v522 = (v520 * v111).exp();
                let v523 = v518 * v522;
                let v3069 = ((v2687 * v520) * v522) * v518;
                let v526 = -v525;
                let v529 = ((v526 * v111) / v502).exp();
                let v530 = v524 * v529;
                let v3073 = (((v2687 * v526) / v502) * v529) * v524;
                v1246 = v517;
                v1259 = v523;
                v1301 = v530;
                v2547 = v3066;
                v2548 = v3069;
                v2549 = v3073;
            } else {
                v1246 = v0;
                v1259 = v0;
                v1301 = v0;
                v2547 = v2947;
                v2548 = v2947;
                v2549 = v2947;
            }
            let v533 = (v435 - v456) + v438;
            let v535 = (v113 * v533).exp();
            let v536 = v531 * v535;
            let v538 = -v537;
            let v540 = (v538 * v111).exp();
            let v541 = v536 * v540;
            let v3081 = ((((v2689 * v533) * v535) * v531) * v540) + (((v2687 * v538) * v540) * v536);
            let v545 = v462 - (v27 * v543);
            let v547 = (v113 * v545).exp();
            let v548 = v542 * v547;
            let v550 = (v471 / v543).exp();
            let v551 = v548 * v550;
            let v3089 = ((((v2689 * v545) * v547) * v542) * v550) + (((v3030 / v543) * v550) * v548);
            let v554 = v435 / v553;
            let v556 = (v113 * v554).exp();
            let v557 = v552 * v556;
            let v559 = (v471 / v553).exp();
            let v560 = v557 * v559;
            let v3097 = ((((v2689 * v554) * v556) * v552) * v559) + (((v3030 / v553) * v559) * v557);
            let v562 = v105.sqrt();
            let v563 = v561 * v562;
            let v566 = (v564 * v112).exp();
            let v567 = v563 * v566;
            let v3106 = (((v2683 * (v2515 / (v2994 * v562))) * v561) * v566) + (((v2530 * v564) * v566) * v563);
            let v569 = v568 * v56;
            let v571 = v569.powf(v570);
            let v3111 = (v2531 * v56) * (v570 * (v569.powf(v3108)));
            let v572 = v2 / v312;
            let v3114 = ((v2917 * v572) * v2676) / v312;
            let v574 = v573 * v568;
            let v575 = v574 * v568;
            let v576 = v575 * v571;
            let v578 = (v576 * v572) * v57;
            let v581 = ((v578 * v308) * v56) * v56;
            let v3130 = (((((((((((v2531 * v573) * v568) + (v2531 * v574)) * v571) + (v3111 * v575)) * v572) + (v3114 * v576)) * v57) * v308) + (v2909 * v578)) * v56) * v56;
            let v583 = v582 * v571;
            let v584 = v583 * v307;
            let v587 = ((v584 * v307) * v58) * v58;
            let v588 = v587 * v312;
            let v590 = (v573 - v581).exp();
            let v591 = v588 * v590;
            let v3147 = ((((((((((v3111 * v582) * v307) + (v2533 * v583)) * v307) + (v2533 * v584)) * v58) * v58) * v312) + (v2917 * v587)) * v590) + (((v3130 * v2676) * v590) * v588);
            let v593 = v592 * v86;
            let v595 = v593.powf(v594);
            let v3152 = (v2532 * v86) * (v594 * (v593.powf(v3149)));
            let v596 = v2 / v314;
            let v598 = v597 * v592;
            let v599 = v598 * v592;
            let v600 = v599 * v595;
            let v602 = (v600 * v596) * v59;
            let v605 = ((v602 * v310) * v86) * v86;
            let v3171 = (((((((((((v2532 * v597) * v592) + (v2532 * v598)) * v595) + (v3152 * v599)) * v596) + ((((v2922 * v596) * v2676) / v314) * v600)) * v59) * v310) + (v2912 * v602)) * v86) * v86;
            let v607 = v606 * v595;
            let v608 = v607 * v309;
            let v611 = ((v608 * v309) * v87) * v87;
            let v612 = v611 * v314;
            let v614 = (v597 - v605).exp();
            let v615 = v612 * v614;
            let v3188 = ((((((((((v3152 * v606) * v309) + (v2537 * v607)) * v309) + (v2537 * v608)) * v87) * v87) * v314) + (v2922 * v611)) * v614) + (((v3171 * v2676) * v614) * v612);
            let v617 = (v113 * v342).exp();
            let v3190 = (v2689 * v342) * v617;
            let v619 = v618 * v617;
            let v620 = v619 * v330;
            let v3194 = ((v3190 * v618) * v330) + (v2941 * v619);
            let v622 = v621 * v617;
            let v623 = v622 * v572;
            let v3198 = ((v3190 * v621) * v572) + (v3114 * v622);
            let v626 = v435 - v625;
            let v628 = (v113 * v626).exp();
            let v629 = v624 * v628;
            let v630 = -v290;
            let v632 = (v630 * v111).exp();
            let v3203 = (v2687 * v630) * v632;
            let v633 = v629 * v632;
            let v3206 = ((((v2689 * v626) * v628) * v624) * v632) + (v3203 * v629);
            let v637 = v634 - (v424 * v635);
            let v639 = (v113 * v637).exp();
            let v640 = v24 * v639;
            let v641 = v640 * v632;
            let v3212 = ((((v2689 * v637) * v639) * v24) * v632) + (v3203 * v640);
            let v643 = v2 - v625;
            let v645 = (v113 * v643).exp();
            let v646 = v642 * v645;
            let v3215 = ((v2689 * v643) * v645) * v642;
            let v648 = v2 - v635;
            let v650 = (v113 * v648).exp();
            let v651 = v647 * v650;
            let v3218 = ((v2689 * v648) * v650) * v647;
            let v653 = v341 - v27;
            let v655 = (v113 * v653).exp();
            let v656 = v652 * v655;
            let v658 = -v657;
            let v660 = (v658 * v111).exp();
            let v661 = v656 * v660;
            let v3226 = ((((v2689 * v653) * v655) * v652) * v660) + (((v2687 * v658) * v660) * v656);
            let v664 = (v342 + v341) - v2;
            let v666 = (v113 * v664).exp();
            let v667 = v662 * v666;
            let v3229 = ((v2689 * v664) * v666) * v662;
            let v669 = v366 - v2;
            let v671 = (v113 * v669).exp();
            let v672 = v668 * v671;
            let v3232 = ((v2689 * v669) * v671) * v668;
            let v674 = v667 + v672;
            let v3233 = v3229 + v3232;
            let v676 = v662 + v668;
            let v677 = (v673 * v674) / v676;
            let v3235 = (v3233 * v673) / v676;
            let v680 = v679 - v2;
            let v682 = (v113 * v680).exp();
            let v683 = v678 * v682;
            let v3238 = ((v2689 * v680) * v682) * v678;
            let v685 = v104 - v684;
            let v687 = if v104 < v686 { 1.0 } else { 0.0 };
            let v1845: f64;
            let v2550: Lanes<1>;
            if v687 != 0.0 {
                let v693 = v692 * v685;
                let v696 = v688 * ((v2 + (v689 * v685)) - (v693 * v685));
                let v3245 = ((v2530 * v689) - (((v2530 * v692) * v685) + (v2530 * v693))) * v688;
                v1845 = v696;
                v2550 = v3245;
            } else {
                let v698 = v688 * v697;
                v1845 = v698;
                v2550 = v2947;
            }
            let v700 = v699 * v617;
            let v3246 = v3190 * v699;
            let v705 = v701 * ((v15 / v12).powf(v703));
            let v706 = if v353 > v0 { 1.0 } else { 0.0 };
            let v1970: f64;
            let v2551: Lanes<1>;
            if v706 != 0.0 {
                let v707 = v2 / v357;
                let v3249 = ((v2956 * v707) * v2676) / v357;
                let v708 = if v707 > v23 { 1.0 } else { 0.0 };
                let v1971: f64;
                let v2552: Lanes<1>;
                if v708 != 0.0 {
                    v1971 = v23;
                    v2552 = v2947;
                } else {
                    v1971 = v707;
                    v2552 = v3249;
                }
                v1970 = v1971;
                v2551 = v2552;
            } else {
                v1970 = v0;
                v2551 = v2947;
            }
            let v709 = if v358 > v0 { 1.0 } else { 0.0 };
            let v1975: f64;
            let v2553: Lanes<1>;
            if v709 != 0.0 {
                let v710 = v2 / v362;
                let v3252 = ((v2959 * v710) * v2676) / v362;
                let v711 = if v710 > v23 { 1.0 } else { 0.0 };
                let v1976: f64;
                let v2554: Lanes<1>;
                if v711 != 0.0 {
                    v1976 = v23;
                    v2554 = v2947;
                } else {
                    v1976 = v710;
                    v2554 = v3252;
                }
                v1975 = v1976;
                v2553 = v2554;
            } else {
                v1975 = v0;
                v2553 = v2947;
            }
            let v712 = if v363 > v0 { 1.0 } else { 0.0 };
            let v1980: f64;
            let v2555: Lanes<1>;
            if v712 != 0.0 {
                let v713 = v2 / v364;
                let v3255 = ((v2960 * v713) * v2676) / v364;
                let v714 = if v713 > v23 { 1.0 } else { 0.0 };
                let v1981: f64;
                let v2556: Lanes<1>;
                if v714 != 0.0 {
                    v1981 = v23;
                    v2556 = v2947;
                } else {
                    v1981 = v713;
                    v2556 = v3255;
                }
                v1980 = v1981;
                v2555 = v2556;
            } else {
                v1980 = v0;
                v2555 = v2947;
            }
            let v718 = v1 * (v715 - v716);
            let v3259 = ((Lanes([v2517[0], 0.0])) - (Lanes([0.0, v2518[0]]))) * v1;
            let v721 = v1 * (v715 - v719);
            let v3263 = ((Lanes([v2517[0], 0.0])) - (Lanes([0.0, v2519[0]]))) * v1;
            let v724 = v1 * (v715 - v722);
            let v3267 = ((Lanes([0.0, v2517[0]])) - (Lanes([v2520[0], 0.0]))) * v1;
            let v727 = v1 * (v725 - v722);
            let v3271 = ((Lanes([0.0, v2521[0]])) - (Lanes([v2520[0], 0.0]))) * v1;
            let v729 = v1 * (v725 - v715);
            let v3275 = ((Lanes([v2521[0], 0.0])) - (Lanes([0.0, v2517[0]]))) * v1;
            let v732 = v1 * (v730 - v716);
            let v3279 = ((Lanes([v2522[0], 0.0])) - (Lanes([0.0, v2518[0]]))) * v1;
            let v734 = v1 * (v716 - v719);
            let v3283 = ((Lanes([v2518[0], 0.0])) - (Lanes([0.0, v2519[0]]))) * v1;
            let v737 = v1 * (v735 - v722);
            let v3287 = ((Lanes([v2523[0], 0.0])) - (Lanes([0.0, v2520[0]]))) * v1;
            let v740 = v1 * (v738 - v725);
            let v3291 = ((Lanes([v2524[0], 0.0])) - (Lanes([0.0, v2521[0]]))) * v1;
            let v742 = v1 * (v738 - v735);
            let v3295 = ((Lanes([v2524[0], 0.0])) - (Lanes([0.0, v2523[0]]))) * v1;
            let v745 = v1 * (v738 - v743);
            let v3299 = ((Lanes([0.0, v2524[0]])) - (Lanes([v2525[0], 0.0]))) * v1;
            let v748 = v1 * (v746 - v716);
            let v3303 = ((Lanes([0.0, v2526[0]])) - (Lanes([v2518[0], 0.0]))) * v1;
            let v751 = v1 * (v749 - v746);
            let v3307 = ((Lanes([v2527[0], 0.0])) - (Lanes([0.0, v2526[0]]))) * v1;
            let v3310 = (Lanes([v3275[0], v3275[1], 0.0])) + (Lanes([0.0, v3263[0], v3263[1]]));
            let v3313 = (Lanes([v3310[0], v3310[1], 0.0, v3310[2]])) - (Lanes([0.0, 0.0, v3283[0], v3283[1]]));
            let v754 = ((v729 + v721) - v734) - v748;
            let v3316 = (Lanes([v3313[0], v3313[1], v3313[2], v3313[3], 0.0])) - (Lanes([0.0, 0.0, v3303[0], 0.0, v3303[1]]));
            let v3317 = v3299 * v2676;
            let v3320 = (Lanes([v3317[0], v3317[1], 0.0])) + (Lanes([0.0, v3291[0], v3291[1]]));
            let v3323 = (Lanes([v3320[0], v3320[1], v3320[2], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v3316[0], v3316[1], v3316[2], v3316[3], v3316[4]]));
            let v758 = (((-v745) + v740) + v754) - v751;
            let v3326 = (Lanes([v3323[0], v3323[1], v3323[2], v3323[3], v3323[4], v3323[5], 0.0, v3323[6]])) - (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v3307[0], v3307[1]]));
            let v759 = v745 + v758;
            let v3328 = (Lanes([v3299[0], v3299[1], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + v3326;
            let v760 = v732 - v748;
            let v3331 = (Lanes([v3279[0], v3279[1], 0.0])) - (Lanes([0.0, v3303[0], v3303[1]]));
            let v761 = v760 - v751;
            let v3334 = (Lanes([v3331[0], v3331[1], 0.0, v3331[2]])) - (Lanes([0.0, 0.0, v3307[0], v3307[1]]));
            let v762 = v721 * v109;
            let v3335 = v3263 * v109;
            let v3336 = v2687 * v721;
            let v3339 = (Lanes([0.0, v3335[0], v3335[1]])) + (Lanes([v3336[0], 0.0, 0.0]));
            let v764 = if v762 < v763 { 1.0 } else { 0.0 };
            let v1007: f64;
            let v2557: Lanes<3>;
            if v764 != 0.0 {
                let v765 = v762.exp();
                let v3341 = v3339 * v765;
                v1007 = v765;
                v2557 = v3341;
            } else {
                let v766 = v763.exp();
                let v769 = v766 * (v2 + (v762 - v763));
                let v3340 = v3339 * v766;
                v1007 = v769;
                v2557 = v3340;
            }
            let v770 = v724 * v109;
            let v3342 = v3267 * v109;
            let v3343 = v2687 * v724;
            let v3346 = (Lanes([0.0, v3342[0], v3342[1]])) + (Lanes([v3343[0], 0.0, 0.0]));
            let v771 = v770 / v441;
            let v3347 = v2542 * v771;
            let v3350 = (v3346 - (Lanes([v3347[0], 0.0, 0.0]))) / v441;
            let v772 = if v771 < v763 { 1.0 } else { 0.0 };
            let v1112: f64;
            let v2558: Lanes<3>;
            if v772 != 0.0 {
                let v773 = v771.exp();
                let v3352 = v3350 * v773;
                v1112 = v773;
                v2558 = v3352;
            } else {
                let v774 = v763.exp();
                let v777 = v774 * (v2 + (v771 - v763));
                let v3351 = v3350 * v774;
                v1112 = v777;
                v2558 = v3351;
            }
            let v778 = v754 * v109;
            let v3353 = v3316 * v109;
            let v3354 = v2687 * v754;
            let v3357 = (Lanes([0.0, v3353[0], v3353[1], v3353[2], v3353[3], v3353[4]])) + (Lanes([v3354[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v779 = if v778 < v763 { 1.0 } else { 0.0 };
            let v1498: f64;
            let v2559: Lanes<6>;
            if v779 != 0.0 {
                let v780 = v778.exp();
                let v3359 = v3357 * v780;
                v1498 = v780;
                v2559 = v3359;
            } else {
                let v781 = v763.exp();
                let v784 = v781 * (v2 + (v778 - v763));
                let v3358 = v3357 * v781;
                v1498 = v784;
                v2559 = v3358;
            }
            let v785 = v729 * v109;
            let v3360 = v3275 * v109;
            let v3361 = v2687 * v729;
            let v3364 = (Lanes([0.0, v3360[0], v3360[1]])) + (Lanes([v3361[0], 0.0, 0.0]));
            let v786 = if v785 < v763 { 1.0 } else { 0.0 };
            let v1741: f64;
            let v2560: Lanes<3>;
            if v786 != 0.0 {
                let v787 = v785.exp();
                let v3366 = v3364 * v787;
                v1741 = v787;
                v2560 = v3366;
            } else {
                let v788 = v763.exp();
                let v791 = v788 * (v2 + (v785 - v763));
                let v3365 = v3364 * v788;
                v1741 = v791;
                v2560 = v3365;
            }
            let v792 = v759 * v109;
            let v3367 = v3328 * v109;
            let v3368 = v2687 * v759;
            let v3371 = (Lanes([v3367[0], v3367[1], 0.0, v3367[2], v3367[3], v3367[4], v3367[5], v3367[6], v3367[7]])) + (Lanes([0.0, 0.0, v3368[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v793 = if v792 < v763 { 1.0 } else { 0.0 };
            let v1595: f64;
            let v2561: Lanes<9>;
            if v793 != 0.0 {
                let v794 = v792.exp();
                let v3373 = v3371 * v794;
                v1595 = v794;
                v2561 = v3373;
            } else {
                let v795 = v763.exp();
                let v798 = v795 * (v2 + (v792 - v763));
                let v3372 = v3371 * v795;
                v1595 = v798;
                v2561 = v3372;
            }
            let v799 = v732 * v109;
            let v3374 = v3279 * v109;
            let v3375 = v2687 * v732;
            let v3378 = (Lanes([v3374[0], 0.0, v3374[1]])) + (Lanes([0.0, v3375[0], 0.0]));
            let v800 = if v799 < v763 { 1.0 } else { 0.0 };
            let v1526: f64;
            let v2562: Lanes<3>;
            if v800 != 0.0 {
                let v801 = v799.exp();
                let v3380 = v3378 * v801;
                v1526 = v801;
                v2562 = v3380;
            } else {
                let v802 = v763.exp();
                let v805 = v802 * (v2 + (v799 - v763));
                let v3379 = v3378 * v802;
                v1526 = v805;
                v2562 = v3379;
            }
            let v806 = v761 * v109;
            let v3381 = v3334 * v109;
            let v3382 = v2687 * v761;
            let v3385 = (Lanes([v3381[0], 0.0, v3381[1], v3381[2], v3381[3]])) + (Lanes([0.0, v3382[0], 0.0, 0.0, 0.0]));
            let v807 = if v806 < v763 { 1.0 } else { 0.0 };
            let v1607: f64;
            let v2563: Lanes<5>;
            if v807 != 0.0 {
                let v808 = v806.exp();
                let v3387 = v3385 * v808;
                v1607 = v808;
                v2563 = v3387;
            } else {
                let v809 = v763.exp();
                let v812 = v809 * (v2 + (v806 - v763));
                let v3386 = v3385 * v809;
                v1607 = v812;
                v2563 = v3386;
            }
            let v813 = v760 * v109;
            let v3388 = v3331 * v109;
            let v3389 = v2687 * v760;
            let v3392 = (Lanes([v3388[0], 0.0, v3388[1], v3388[2]])) + (Lanes([0.0, v3389[0], 0.0, 0.0]));
            let v814 = if v813 < v763 { 1.0 } else { 0.0 };
            let v1542: f64;
            let v2564: Lanes<4>;
            if v814 != 0.0 {
                let v815 = v813.exp();
                let v3394 = v3392 * v815;
                v1542 = v815;
                v2564 = v3394;
            } else {
                let v816 = v763.exp();
                let v819 = v816 * (v2 + (v813 - v763));
                let v3393 = v3392 * v816;
                v1542 = v819;
                v2564 = v3393;
            }
            let v821 = v759 - v820;
            let v3395 = Lanes([v3328[0], v3328[1], 0.0, v3328[2], v3328[3], v3328[4], v3328[5], v3328[6], v3328[7]]);
            let v822 = v821 * v109;
            let v3399 = v2687 * v821;
            let v3401 = ((v3395 - (Lanes([0.0, 0.0, v2534[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v109) + (Lanes([0.0, 0.0, v3399[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v823 = if v822 < v763 { 1.0 } else { 0.0 };
            let v2223: f64;
            let v2565: Lanes<9>;
            if v823 != 0.0 {
                let v824 = v822.exp();
                let v3403 = v3401 * v824;
                v2223 = v824;
                v2565 = v3403;
            } else {
                let v825 = v763.exp();
                let v828 = v825 * (v2 + (v822 - v763));
                let v3402 = v3401 * v825;
                v2223 = v828;
                v2565 = v3402;
            }
            let v829 = v754 - v820;
            let v3404 = Lanes([0.0, v3316[0], v3316[1], v3316[2], v3316[3], v3316[4]]);
            let v830 = v829 * v109;
            let v3408 = v2687 * v829;
            let v3410 = ((v3404 - (Lanes([v2534[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) * v109) + (Lanes([v3408[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v831 = if v830 < v763 { 1.0 } else { 0.0 };
            let v1500: f64;
            let v2566: Lanes<6>;
            if v831 != 0.0 {
                let v832 = v830.exp();
                let v3412 = v3410 * v832;
                v1500 = v832;
                v2566 = v3412;
            } else {
                let v833 = v763.exp();
                let v836 = v833 * (v2 + (v830 - v763));
                let v3411 = v3410 * v833;
                v1500 = v836;
                v2566 = v3411;
            }
            let v837 = v721 - v820;
            let v838 = v837 * v109;
            let v3417 = v2687 * v837;
            let v3419 = (((Lanes([0.0, v3263[0], v3263[1]])) - (Lanes([v2534[0], 0.0, 0.0]))) * v109) + (Lanes([v3417[0], 0.0, 0.0]));
            let v839 = if v838 < v763 { 1.0 } else { 0.0 };
            let v853: f64;
            let v2567: Lanes<3>;
            if v839 != 0.0 {
                let v840 = v838.exp();
                let v3421 = v3419 * v840;
                v853 = v840;
                v2567 = v3421;
            } else {
                let v841 = v763.exp();
                let v844 = v841 * (v2 + (v838 - v763));
                let v3420 = v3419 * v841;
                v853 = v844;
                v2567 = v3420;
            }
            let v845 = v718 - v820;
            let v3422 = Lanes([0.0, v3259[0], v3259[1]]);
            let v3423 = Lanes([v2534[0], 0.0, 0.0]);
            let v846 = v845 * v109;
            let v3426 = v2687 * v845;
            let v3428 = ((v3422 - v3423) * v109) + (Lanes([v3426[0], 0.0, 0.0]));
            let v847 = if v846 < v763 { 1.0 } else { 0.0 };
            let v857: f64;
            let v2568: Lanes<3>;
            if v847 != 0.0 {
                let v848 = v846.exp();
                let v3430 = v3428 * v848;
                v857 = v848;
                v2568 = v3430;
            } else {
                let v849 = v763.exp();
                let v852 = v849 * (v2 + (v846 - v763));
                let v3429 = v3428 * v849;
                v857 = v852;
                v2568 = v3429;
            }
            let v856 = (v2 + (v435 * v853)).sqrt();
            let v3434 = (v2567 * v435) * (v2515 / (v2994 * v856));
            let v860 = (v2 + (v435 * v857)).sqrt();
            let v3438 = (v2568 * v435) * (v2515 / (v2994 * v860));
            let v862 = v2 + v860;
            let v863 = (v27 * v857) / v862;
            let v3442 = ((v2568 * v27) - (v3438 * v863)) / v862;
            let v865 = if v863 < v864 { 1.0 } else { 0.0 };
            let v952: f64;
            let v2569: Lanes<3>;
            if v865 != 0.0 {
                v952 = v864;
                v2569 = v3443;
            } else {
                v952 = v863;
                v2569 = v3442;
            }
            let v3444 = Lanes([v3434[0], v3434[1], 0.0, v3434[2]]);
            let v867 = v856 + v2;
            let v868 = v867 / v862;
            let v3447 = v3438 * v868;
            let v870 = (v856 - v860) - (v868.ln());
            let v871 = v107 * v870;
            let v3454 = v2684 * v870;
            let v3457 = (Lanes([v3454[0], 0.0, 0.0, 0.0])) + (((v3444 - (Lanes([v3438[0], v3438[1], v3438[2], 0.0]))) - (((v3444 - (Lanes([v3447[0], v3447[1], v3447[2], 0.0]))) / v862) * (v2515 / v868))) * v107);
            let v3458 = Lanes([0.0, 0.0, v3283[0], v3283[1]]);
            let v873 = (v871 + v734) / v369;
            let v3460 = v2963 * v873;
            let v3463 = ((v3457 + v3458) - (Lanes([v3460[0], 0.0, 0.0, 0.0]))) / v369;
            let v874 = if v873 > v0 { 1.0 } else { 0.0 };
            let v1063: f64;
            let v1076: f64;
            let v1091: f64;
            let v1118: f64;
            let v1793: f64;
            let v1829: f64;
            let v2180: f64;
            let v2570: Lanes<4>;
            let v2571: Lanes<4>;
            let v2572: Lanes<4>;
            let v2573: Lanes<4>;
            let v2574: Lanes<4>;
            let v2575: Lanes<4>;
            let v2576: Lanes<4>;
            if v874 != 0.0 {
                let v876 = if v718 < v875 { 1.0 } else { 0.0 };
                let v889: f64;
                let v2577: Lanes<2>;
                if v876 != 0.0 {
                    v889 = v718;
                    v2577 = v3259;
                } else {
                    let v878 = v2 + (v718 - v875);
                    let v3491 = v3259 * (v2515 / v878);
                    let v880 = v875 + (v878.ln());
                    v889 = v880;
                    v2577 = v3491;
                }
                let v881 = v27 * v107;
                let v882 = v424 * v873;
                let v883 = v882 * v369;
                let v3495 = v2963 * v882;
                let v3497 = ((v3463 * v424) * v369) + (Lanes([v3495[0], 0.0, 0.0, 0.0]));
                let v3499 = v2687 * v883;
                let v885 = (v883 * v109) + v2;
                let v886 = v885.ln();
                let v3504 = (v2684 * v27) * v886;
                let v890 = (v820 + (v881 * v886)) - v889;
                let v3511 = ((Lanes([v2534[0], 0.0, 0.0, 0.0])) + ((Lanes([v3504[0], 0.0, 0.0, 0.0])) + ((((v3497 * v109) + (Lanes([v3499[0], 0.0, 0.0, 0.0]))) * (v2515 / v885)) * v881))) - (Lanes([0.0, v2577[0], v2577[1], 0.0]));
                let v892 = v891 * v820;
                let v893 = v892 * v892;
                let v3513 = (v2534 * v891) * v892;
                let v3514 = v3513 + v3513;
                let v894 = v890 * v890;
                let v3515 = v3511 * v890;
                let v3516 = v3515 + v3515;
                let v895 = if v890 < v0 { 1.0 } else { 0.0 };
                let v905: f64;
                let v2578: Lanes<4>;
                if v895 != 0.0 {
                    let v3524 = v3514 * v424;
                    let v898 = (v894 + v893).sqrt();
                    let v899 = v898 - v890;
                    let v900 = (v424 * v893) / v899;
                    let v3534 = ((Lanes([v3524[0], 0.0, 0.0, 0.0])) - ((((v3516 + (Lanes([v3514[0], 0.0, 0.0, 0.0]))) * (v2515 / (v2994 * v898))) - v3511) * v900)) / v899;
                    v905 = v900;
                    v2578 = v3534;
                } else {
                    let v902 = (v894 + v893).sqrt();
                    let v904 = v424 * (v902 + v890);
                    let v3523 = (((v3516 + (Lanes([v3514[0], 0.0, 0.0, 0.0]))) * (v2515 / (v2994 * v902))) + v3511) * v424;
                    v905 = v904;
                    v2578 = v3523;
                }
                let v908 = v906 * v907;
                let v909 = v905 + v908;
                let v3538 = v2963 * v906;
                let v913 = v907 * (v905 + (v906 * v369));
                let v914 = (v905 * v909) / v913;
                let v3544 = (((v2578 * v909) + (v2578 * v905)) - (((v2578 + (Lanes([v3538[0], 0.0, 0.0, 0.0]))) * v907) * v914)) / v913;
                let v915 = v873 / v914;
                let v3547 = (v3463 - (v3544 * v915)) / v914;
                let v918 = (v915 - v2) / v917;
                let v3548 = v3547 / v917;
                let v919 = if v915 < v2 { 1.0 } else { 0.0 };
                let v931: f64;
                let v2579: Lanes<4>;
                if v919 != 0.0 {
                    let v920 = v918.exp();
                    let v921 = v2 + v920;
                    let v3558 = ((v3548 * v920) * (v2515 / v921)) * v917;
                    let v924 = v2 + (v917 * (v921.ln()));
                    v931 = v924;
                    v2579 = v3558;
                } else {
                    let v926 = (-v918).exp();
                    let v927 = v2 + v926;
                    let v930 = v915 + (v917 * (v927.ln()));
                    let v3554 = v3547 + ((((v3548 * v2676) * v926) * (v2515 / v927)) * v917);
                    v931 = v930;
                    v2579 = v3554;
                }
                let v938 = v2 + (v917 * ((v2 + ((v932 / v917).exp())).ln()));
                let v939 = v931 / v938;
                let v3559 = v2579 / v938;
                let v940 = v905 / v908;
                let v3560 = v2578 / v908;
                let v941 = v435 * v939;
                let v942 = v941 * v940;
                let v943 = v2 + v940;
                let v946 = (v2 + (v942 * v943)).sqrt();
                let v948 = v27 * v939;
                let v949 = v948 * v943;
                let v950 = (v2 + v946) / v949;
                let v3577 = (((((((v3559 * v435) * v940) + (v3560 * v941)) * v943) + (v3560 * v942)) * (v2515 / (v2994 * v946))) - ((((v3559 * v27) * v943) + (v3560 * v948)) * v950)) / v949;
                let v953 = v952 * v950;
                let v3579 = v2569 * v950;
                let v3582 = (Lanes([v3579[0], v3579[1], v3579[2], 0.0])) + (v3577 * v952);
                let v955 = v2 + v953;
                let v956 = ((v2 - v950) + v953) / v955;
                let v3586 = (((v3577 * v2676) + v3582) - (v3582 * v956)) / v955;
                let v957 = v883 * v956;
                let v958 = v957 * v109;
                let v3591 = v2687 * v957;
                let v3593 = (((v3497 * v956) + (v3586 * v883)) * v109) + (Lanes([v3591[0], 0.0, 0.0, 0.0]));
                let v961 = (v952 + v958) + v2;
                let v3597 = v2569 * v961;
                let v963 = (v27 * v958) + (v952 * v961);
                let v3601 = (v3593 * v27) + ((Lanes([v3597[0], v3597[1], v3597[2], 0.0])) + (((Lanes([v2569[0], v2569[1], v2569[2], 0.0])) + v3593) * v952));
                let v965 = v424 * (v958 - v2);
                let v3602 = v3593 * v424;
                let v3603 = v3602 * v965;
                let v967 = (v965 * v965) + v963;
                let v3605 = (v3603 + v3603) + v3601;
                let v968 = if v958 >= v2 { 1.0 } else { 0.0 };
                let v974: f64;
                let v2580: Lanes<4>;
                if v968 != 0.0 {
                    let v969 = v967.sqrt();
                    let v970 = v965 + v969;
                    let v3616 = v3602 + (v3605 * (v2515 / (v2994 * v969)));
                    v974 = v970;
                    v2580 = v3616;
                } else {
                    let v971 = v967.sqrt();
                    let v972 = v971 - v965;
                    let v973 = v963 / v972;
                    let v3612 = (v3601 - (((v3605 * (v2515 / (v2994 * v971))) - v3602) * v973)) / v972;
                    v974 = v973;
                    v2580 = v3612;
                }
                let v976 = if v974 < v975 { 1.0 } else { 0.0 };
                let v977: f64;
                let v2581: Lanes<4>;
                if v976 != 0.0 {
                    v977 = v975;
                    v2581 = v3489;
                } else {
                    v977 = v974;
                    v2581 = v2580;
                }
                let v978 = v977 + v2;
                let v979 = v977 * v978;
                let v981 = (v820 * v109).exp();
                let v982 = v979 * v981;
                let v3625 = (((v2534 * v109) + (v2687 * v820)) * v981) * v979;
                let v3627 = (((v2581 * v978) + (v2581 * v977)) * v981) + (Lanes([v3625[0], 0.0, 0.0, 0.0]));
                let v983 = v424 * v907;
                let v985 = v983 * (v873 - v906);
                let v3628 = v3463 * v983;
                let v987 = (v907 * v369) * v906;
                let v3631 = ((v2963 * v907) * v906) * v873;
                let v3635 = v3628 * v985;
                let v991 = ((v985 * v985) + (v987 * v873)).sqrt();
                let v992 = v985 + v991;
                let v3641 = v3628 + (((v3635 + v3635) + ((Lanes([v3631[0], 0.0, 0.0, 0.0])) + (v3463 * v987))) * (v2515 / (v2994 * v991)));
                let v994 = if v993 == v0 { 1.0 } else { 0.0 };
                let v1077: f64;
                let v2582: Lanes<4>;
                if v994 != 0.0 {
                    let v995 = v325 * v42;
                    let v3651 = v2536 * v42;
                    let v3652 = Lanes([v3651[0], 0.0, 0.0, 0.0]);
                    v1077 = v995;
                    v2582 = v3652;
                } else {
                    let v997 = v873 + v914;
                    let v998 = (v27 * v873) / v997;
                    let v999 = v42 + v998;
                    let v1000 = v325 * v999;
                    let v3647 = v2536 * v999;
                    let v3650 = (Lanes([v3647[0], 0.0, 0.0, 0.0])) + ((((v3463 * v27) - ((v3463 + v3544) * v998)) / v997) * v325);
                    v1077 = v1000;
                    v2582 = v3650;
                }
                let v1002 = v906 + v873;
                let v1003 = (v906 * v873) / v1002;
                let v3656 = ((v3463 * v906) - (v3463 * v1003)) / v1002;
                let v1004 = v906 / v1002;
                let v3659 = ((v3463 * v1004) * v2676) / v1002;
                v1063 = v992;
                v1076 = v1077;
                v1091 = v1004;
                v1118 = v982;
                v1793 = v956;
                v1829 = v1003;
                v2180 = v977;
                v2570 = v3641;
                v2571 = v2582;
                v2572 = v3659;
                v2573 = v3627;
                v2574 = v3586;
                v2575 = v3656;
                v2576 = v2581;
            } else {
                let v1006 = (v27 * v853) / v867;
                let v3467 = ((v2567 * v27) - (v3434 * v1006)) / v867;
                let v1018 = if (if (v734.abs()) < (v1009 * v107) { 1.0 } else { 0.0 }) != 0.0 || (if (v871.abs()) < ((v1013 * v107) * (v856 + v860)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1794: f64;
                let v2583: Lanes<4>;
                if v1018 != 0.0 {
                    let v1020 = v424 * (v1006 + v952);
                    let v3479 = ((Lanes([v3467[0], v3467[1], 0.0, v3467[2]])) + (Lanes([v2569[0], v2569[1], v2569[2], 0.0]))) * v424;
                    let v1021 = v1020 + v2;
                    let v1022 = v1020 / v1021;
                    let v3482 = (v3479 - (v3479 * v1022)) / v1021;
                    v1794 = v1022;
                    v2583 = v3482;
                } else {
                    let v1024 = (v871 + v721) - v718;
                    let v1025 = v871 / v1024;
                    let v3475 = (v3457 - (((v3457 + (Lanes([0.0, v3263[0], 0.0, v3263[1]]))) - (Lanes([0.0, v3259[0], v3259[1], 0.0]))) * v1025)) / v1024;
                    v1794 = v1025;
                    v2583 = v3475;
                }
                let v1026 = v42 * v325;
                let v3483 = v2536 * v42;
                let v1028 = v2 - (v873 / v906);
                let v3485 = (v3463 / v906) * v2676;
                let v3486 = Lanes([v3483[0], 0.0, 0.0, 0.0]);
                let v3487 = Lanes([v2557[0], v2557[1], 0.0, v2557[2]]);
                let v3488 = Lanes([v3467[0], v3467[1], 0.0, v3467[2]]);
                v1063 = v734;
                v1076 = v1026;
                v1091 = v1028;
                v1118 = v1007;
                v1793 = v1794;
                v1829 = v873;
                v2180 = v1006;
                v2570 = v3458;
                v2571 = v3486;
                v2572 = v3485;
                v2573 = v3487;
                v2574 = v2583;
                v2575 = v3463;
                v2576 = v3488;
            }
            let v1032 = v2 - (v154.powf((v1029 / v28)));
            let v1033 = v307 * v1032;
            let v3660 = v2533 * v1032;
            let v1034 = v42 * v307;
            let v3661 = v2533 * v42;
            let v3662 = Lanes([0.0, v3267[0], v3267[1]]);
            let v3663 = Lanes([v3660[0], 0.0, 0.0]);
            let v1036 = (v724 - v1033) / v1034;
            let v3665 = v3661 * v1036;
            let v3668 = ((v3662 - v3663) - (Lanes([v3665[0], 0.0, 0.0]))) / v1034;
            let v1037 = if v724 < v1033 { 1.0 } else { 0.0 };
            let v1049: f64;
            let v2584: Lanes<3>;
            if v1037 != 0.0 {
                let v1038 = v1036.exp();
                let v1039 = v2 + v1038;
                let v1040 = v1039.ln();
                let v3681 = v3661 * v1040;
                let v1042 = v724 - (v1034 * v1040);
                let v3685 = v3662 - ((Lanes([v3681[0], 0.0, 0.0])) + (((v3668 * v1038) * (v2515 / v1039)) * v1034));
                v1049 = v1042;
                v2584 = v3685;
            } else {
                let v1044 = (-v1036).exp();
                let v1045 = v2 + v1044;
                let v1046 = v1045.ln();
                let v3673 = v3661 * v1046;
                let v1048 = v1033 - (v1034 * v1046);
                let v3677 = v3663 - ((Lanes([v3673[0], 0.0, 0.0])) + ((((v3668 * v2676) * v1044) * (v2515 / v1045)) * v1034));
                v1049 = v1048;
                v2584 = v3677;
            }
            let v3687 = v2909 * v1049;
            let v1051 = v2 - (v1049 * v308);
            let v3690 = ((v2584 * v308) + (Lanes([v3687[0], 0.0, 0.0]))) * v2676;
            let v1052 = v2 - v28;
            let v1053 = v1051.powf(v1052);
            let v3691 = v1052 - v2515;
            let v3694 = v3690 * (v1052 * (v1051.powf(v3691)));
            let v1054 = v307 / v1052;
            let v3695 = v2533 / v1052;
            let v1055 = v2 - v1053;
            let v3697 = v3695 * v1055;
            let v1059 = (v1054 * v1055) + (v154 * (v724 - v1049));
            let v3703 = ((Lanes([v3697[0], 0.0, 0.0])) + ((v3694 * v2676) * v1054)) + ((v3662 - v2584) * v154);
            let v1061 = if v1060 == v2 { 1.0 } else { 0.0 };
            let v1073: f64;
            let v2585: Lanes<4>;
            if v1061 != 0.0 {
                let v3707 = Lanes([0.0, v3259[0], v3259[1], 0.0]);
                v1073 = v718;
                v2585 = v3707;
            } else {
                let v1062 = if v1060 == v27 { 1.0 } else { 0.0 };
                let v1074: f64;
                let v2586: Lanes<4>;
                if v1062 != 0.0 {
                    let v1064 = v718 + v1063;
                    let v3706 = (Lanes([0.0, v3259[0], v3259[1], 0.0])) + v2570;
                    v1074 = v1064;
                    v2586 = v3706;
                } else {
                    let v3704 = Lanes([0.0, v3263[0], 0.0, v3263[1]]);
                    v1074 = v721;
                    v2586 = v3704;
                }
                v1073 = v1074;
                v2585 = v2586;
            }
            let v3708 = v2943 * v2676;
            let v1066 = v2 - v333;
            let v1067 = (v27 - v333) / v1066;
            let v3711 = (v3708 - (v3708 * v1067)) / v1066;
            let v1069 = v1068 / v60;
            let v1071 = v2 - (v1067.powf(v1069));
            let v1072 = v325 * v1071;
            let v3719 = (v2536 * v1071) + (((v3711 * (v1069 * (v1067.powf((v1069 - v2515))))) * v2676) * v325);
            let v3720 = Lanes([v3719[0], 0.0, 0.0, 0.0]);
            let v1078 = (v1073 - v1072) / v1076;
            let v3724 = ((v2585 - v3720) - (v2571 * v1078)) / v1076;
            let v1079 = if v1073 < v1072 { 1.0 } else { 0.0 };
            let v1096: f64;
            let v2587: Lanes<4>;
            if v1079 != 0.0 {
                let v1080 = v1078.exp();
                let v1081 = v2 + v1080;
                let v1082 = v1081.ln();
                let v1084 = v1073 - (v1076 * v1082);
                let v3739 = v2585 - ((v2571 * v1082) + (((v3724 * v1080) * (v2515 / v1081)) * v1076));
                v1096 = v1084;
                v2587 = v3739;
            } else {
                let v1086 = (-v1078).exp();
                let v1087 = v2 + v1086;
                let v1088 = v1087.ln();
                let v1090 = v1072 - (v1076 * v1088);
                let v3732 = v3720 - ((v2571 * v1088) + ((((v3724 * v2676) * v1086) * (v2515 / v1087)) * v1076));
                v1096 = v1090;
                v2587 = v3732;
            }
            let v1093 = v1091.powf(v1092);
            let v3743 = v2572 * (v1092 * (v1091.powf((v1092 - v2515))));
            let v1094 = v2 - v60;
            let v1095 = v325 / v1094;
            let v3744 = v2536 / v1094;
            let v1097 = v1096 / v325;
            let v3745 = v2536 * v1097;
            let v1098 = v2 - v1097;
            let v1099 = v1098.powf(v1094);
            let v3750 = v1094 - v2515;
            let v1101 = v2 - (v1093 * v1099);
            let v3758 = v3744 * v1101;
            let v1103 = v1093 * v1067;
            let v3763 = v3711 * v1093;
            let v1104 = v1073 - v1096;
            let v1106 = (v1095 * v1101) + (v1103 * v1104);
            let v3771 = v3708 * v1106;
            let v3775 = v2943 * v718;
            let v3776 = v3259 * v333;
            let v3779 = (Lanes([v3775[0], 0.0, 0.0])) + (Lanes([0.0, v3776[0], v3776[1]]));
            let v1109 = (v1066 * v1106) + (v333 * v718);
            let v3781 = ((Lanes([v3771[0], 0.0, 0.0, 0.0])) + ((((Lanes([v3758[0], 0.0, 0.0, 0.0])) + ((((v3743 * v1099) + (((((v2587 - (Lanes([v3745[0], 0.0, 0.0, 0.0]))) / v325) * v2676) * (v1094 * (v1098.powf(v3750)))) * v1093)) * v2676) * v1095)) + ((((v3743 * v1067) + (Lanes([v3763[0], 0.0, 0.0, 0.0]))) * v1104) + ((v2585 - v2587) * v1103))) * v1066)) + (Lanes([v3779[0], v3779[1], v3779[2], 0.0]));
            let v1111 = (v435 * v449) / v454;
            let v3785 = ((v3020 * v435) - (v3023 * v1111)) / v454;
            let v1113 = v1111 * v1112;
            let v3786 = v3785 * v1112;
            let v3789 = (Lanes([v3786[0], 0.0, 0.0])) + (v2558 * v1111);
            let v1115 = (v2 + v1113).sqrt();
            let v3792 = v3789 * (v2515 / (v2994 * v1115));
            let v1116 = v2 + v1115;
            let v1117 = v1113 / v1116;
            let v3795 = (v3789 - (v3792 * v1117)) / v1116;
            let v1120 = v2 / v1119;
            let v1121 = v1118.powf(v1120);
            let v3803 = v1118.ln();
            let v3805 = (((v2544 * v1120) * v2676) / v1119) * (v1121 * v3803);
            let v3807 = (v2573 * (v1120 * (v1118.powf((v1120 - v2515))))) + (Lanes([v3805[0], 0.0, 0.0, 0.0]));
            let v1122 = v1111 * v1121;
            let v3808 = v3785 * v1121;
            let v3811 = (Lanes([v3808[0], 0.0, 0.0, 0.0])) + (v3807 * v1111);
            let v1124 = (v2 + v1122).sqrt();
            let v1125 = v2 + v1124;
            let v1126 = v1122 / v1125;
            let v3817 = (v3811 - ((v3811 * (v2515 / (v2994 * v1124))) * v1126)) / v1125;
            let v1127 = if v699 == v0 { 1.0 } else { 0.0 };
            let v1148: f64;
            let v2588: Lanes<5>;
            if v1127 != 0.0 {
                let v1128 = v1059 / v623;
                let v3856 = v3198 * v1128;
                let v3859 = (v3703 - (Lanes([v3856[0], 0.0, 0.0]))) / v623;
                let v1130 = v1109 / v620;
                let v3860 = v3194 * v1130;
                let v3863 = (v3781 - (Lanes([v3860[0], 0.0, 0.0, 0.0]))) / v620;
                let v1131 = (v2 + v1128) + v1130;
                let v3866 = (Lanes([v3859[0], v3859[1], v3859[2], 0.0, 0.0])) + (Lanes([v3863[0], 0.0, v3863[1], v3863[2], v3863[3]]));
                v1148 = v1131;
                v2588 = v3866;
            } else {
                let v1132 = v1059 / v623;
                let v3818 = v3198 * v1132;
                let v1133 = v1132 + v2;
                let v1134 = v1133 * v700;
                let v3823 = v3246 * v1133;
                let v3827 = v2687 * v1134;
                let v1137 = (-v1109) / v620;
                let v3831 = v3194 * v1137;
                let v1138 = v1137 * v700;
                let v3836 = v3246 * v1137;
                let v3840 = v2687 * v1138;
                let v1140 = (v1134 * v109).exp();
                let v3843 = ((((((v3703 - (Lanes([v3818[0], 0.0, 0.0]))) / v623) * v700) + (Lanes([v3823[0], 0.0, 0.0]))) * v109) + (Lanes([v3827[0], 0.0, 0.0]))) * v1140;
                let v1141 = (v1138 * v109).exp();
                let v3844 = (((((((v3781 * v2676) - (Lanes([v3831[0], 0.0, 0.0, 0.0]))) / v620) * v700) + (Lanes([v3836[0], 0.0, 0.0, 0.0]))) * v109) + (Lanes([v3840[0], 0.0, 0.0, 0.0]))) * v1141;
                let v1144 = (v700 * v109).exp();
                let v1145 = v1144 - v2;
                let v1146 = (v1140 - v1141) / v1145;
                let v3852 = (((v3246 * v109) + (v2687 * v700)) * v1144) * v1146;
                let v3855 = (((Lanes([v3843[0], v3843[1], v3843[2], 0.0, 0.0])) - (Lanes([v3844[0], 0.0, v3844[1], v3844[2], v3844[3]]))) - (Lanes([v3852[0], 0.0, 0.0, 0.0, 0.0]))) / v1145;
                v1148 = v1146;
                v2588 = v3855;
            }
            let v1149 = v1148 * v1148;
            let v3867 = v2588 * v1148;
            let v3868 = v3867 + v3867;
            let v1150 = if v1148 < v0 { 1.0 } else { 0.0 };
            let v1160: f64;
            let v2589: Lanes<5>;
            if v1150 != 0.0 {
                let v1153 = (v1149 + v1147).sqrt();
                let v1154 = v1153 - v1148;
                let v1155 = v1151 / v1154;
                let v3880 = ((((v3868 * (v2515 / (v2994 * v1153))) - v2588) * v1155) * v2676) / v1154;
                v1160 = v1155;
                v2589 = v3880;
            } else {
                let v1157 = (v1149 + v1147).sqrt();
                let v1159 = v424 * (v1157 + v1148);
                let v3873 = ((v3868 * (v2515 / (v2994 * v1157))) + v2588) * v424;
                v1160 = v1159;
                v2589 = v3873;
            }
            let v3884 = ((Lanes([v3795[0], v3795[1], v3795[2], 0.0, 0.0])) + (Lanes([v3817[0], 0.0, v3817[1], v3817[2], v3817[3]]))) * v424;
            let v1163 = v2 + (v424 * (v1117 + v1126));
            let v1164 = v1160 * v1163;
            let v3887 = (v2589 * v1163) + (v3884 * v1160);
            let v1166 = v1165 * v449;
            let v1167 = v1166 * v1121;
            let v3889 = (v3020 * v1165) * v1121;
            let v3892 = (Lanes([v3889[0], 0.0, 0.0, 0.0])) + (v3807 * v1166);
            let v1168 = v449 * v1112;
            let v3893 = v3020 * v1112;
            let v3896 = (Lanes([v3893[0], 0.0, 0.0])) + (v2558 * v449);
            let v3897 = Lanes([v3896[0], v3896[1], v3896[2], 0.0, 0.0]);
            let v3898 = Lanes([v3892[0], 0.0, v3892[1], v3892[2], v3892[3]]);
            let v1170 = (v1168 - v1167) / v1164;
            let v3902 = ((v3897 - v3898) - (v3887 * v1170)) / v1164;
            let v1172 = v724 / v1171;
            let v3903 = v3267 / v1171;
            let v1173 = if v724 < v0 { 1.0 } else { 0.0 };
            let v1184: f64;
            let v2590: Lanes<2>;
            if v1173 != 0.0 {
                let v1174 = v1172.exp();
                let v1175 = v2 + v1174;
                let v1177 = v1171 * (v1175.ln());
                let v3913 = ((v3903 * v1174) * (v2515 / v1175)) * v1171;
                v1184 = v1177;
                v2590 = v3913;
            } else {
                let v1179 = (-v1172).exp();
                let v1180 = v2 + v1179;
                let v1183 = v724 + (v1171 * (v1180.ln()));
                let v3909 = v3267 + ((((v3903 * v2676) * v1179) * (v2515 / v1180)) * v1171);
                v1184 = v1183;
                v2590 = v3909;
            }
            let v1186 = v1184 / v1185;
            let v3914 = v2590 / v1185;
            let v1187 = if v1186 < v763 { 1.0 } else { 0.0 };
            let v1193: f64;
            let v2591: Lanes<2>;
            if v1187 != 0.0 {
                let v1188 = v1186.exp();
                let v3916 = v3914 * v1188;
                v1193 = v1188;
                v2591 = v3916;
            } else {
                let v1189 = v763.exp();
                let v1192 = v1189 * (v2 + (v1186 - v763));
                let v3915 = v3914 * v1189;
                v1193 = v1192;
                v2591 = v3915;
            }
            let v1194 = v1193 - v2;
            let v1195 = v567 * v1194;
            let v3917 = v3106 * v1194;
            let v3918 = v2591 * v567;
            let v3921 = (Lanes([v3917[0], 0.0, 0.0])) + (Lanes([0.0, v3918[0], v3918[1]]));
            let v1198 = (v724 - v1196) / v26;
            let v3922 = v3267 / v26;
            let v1199 = if v724 < v1196 { 1.0 } else { 0.0 };
            let v1212: f64;
            let v2592: Lanes<2>;
            if v1199 != 0.0 {
                let v1200 = v1198.exp();
                let v1201 = v2 + v1200;
                let v1204 = v724 - (v26 * (v1201.ln()));
                let v3933 = v3267 - (((v3922 * v1200) * (v2515 / v1201)) * v26);
                v1212 = v1204;
                v2592 = v3933;
            } else {
                let v1206 = (-v1198).exp();
                let v1207 = v2 + v1206;
                let v1210 = v1196 - (v26 * (v1207.ln()));
                let v3928 = ((((v3922 * v2676) * v1206) * (v2515 / v1207)) * v26) * v2676;
                v1212 = v1210;
                v2592 = v3928;
            }
            let v1213 = v1211 * v1212;
            let v1214 = v1196 - v1212;
            let v1215 = v1214 * v1214;
            let v1216 = v1213 * v1215;
            let v3940 = ((v2592 * v1211) * v1215) + (((v2592 * v2676) * (v27 * v1214)) * v1213);
            let v1217 = v770 / v491;
            let v3941 = v3346 / v491;
            let v1218 = if v1217 < v763 { 1.0 } else { 0.0 };
            let v1243: f64;
            let v2593: Lanes<3>;
            if v1218 != 0.0 {
                let v1219 = v1217.exp();
                let v3943 = v3941 * v1219;
                v1243 = v1219;
                v2593 = v3943;
            } else {
                let v1220 = v763.exp();
                let v1223 = v1220 * (v2 + (v1217 - v763));
                let v3942 = v3941 * v1220;
                v1243 = v1223;
                v2593 = v3942;
            }
            let v1989: f64;
            let v2594: Lanes<5>;
            if v510 != 0.0 {
                let v1225 = v724 - v1224;
                let v1226 = v1225 * v109;
                let v3971 = v2687 * v1225;
                let v3973 = ((v3662 - (Lanes([v2538[0], 0.0, 0.0]))) * v109) + (Lanes([v3971[0], 0.0, 0.0]));
                let v1227 = if v1226 < v763 { 1.0 } else { 0.0 };
                let v1249: f64;
                let v2595: Lanes<3>;
                if v1227 != 0.0 {
                    let v1228 = v1226.exp();
                    let v3975 = v3973 * v1228;
                    v1249 = v1228;
                    v2595 = v3975;
                } else {
                    let v1229 = v763.exp();
                    let v1232 = v1229 * (v2 + (v1226 - v763));
                    let v3974 = v3973 * v1229;
                    v1249 = v1232;
                    v2595 = v3974;
                }
                let v1233 = v1170 / v449;
                let v3976 = v3020 * v1233;
                let v3979 = (v3902 - (Lanes([v3976[0], 0.0, 0.0, 0.0, 0.0]))) / v449;
                let v1235 = v1233 - v1234;
                let v1237 = if v1235 < v1236 { 1.0 } else { 0.0 };
                let v1262: f64;
                let v2596: Lanes<5>;
                if v1237 != 0.0 {
                    let v1238 = v1235.exp();
                    let v3981 = v3979 * v1238;
                    v1262 = v1238;
                    v2596 = v3981;
                } else {
                    let v1242 = v1239 * (v2 + (v1235 - v1236));
                    let v3980 = v3979 * v1239;
                    v1262 = v1242;
                    v2596 = v3980;
                }
                let v1244 = v1243 - v2;
                let v3982 = v3054 * v1244;
                let v3985 = (Lanes([v3982[0], 0.0, 0.0])) + (v2593 * v500);
                let v1247 = v1246 * v27;
                let v3987 = (v2547 * v27) * v1244;
                let v1252 = (v2 + (v435 * v1249)).sqrt();
                let v1253 = v2 + v1252;
                let v1254 = (v1247 * v1244) / v1253;
                let v1255 = v1109 / v620;
                let v3998 = v3194 * v1255;
                let v1256 = v2 + v1255;
                let v4002 = ((((Lanes([v3987[0], 0.0, 0.0])) + (v2593 * v1247)) - (((v2595 * v435) * (v2515 / (v2994 * v1252))) * v1254)) / v1253) * v1256;
                let v4003 = ((v3781 - (Lanes([v3998[0], 0.0, 0.0, 0.0]))) / v620) * v1254;
                let v1260 = v1118 - v2;
                let v1261 = v1259 * v1260;
                let v4009 = v2548 * v1260;
                let v4013 = ((Lanes([v4009[0], 0.0, 0.0, 0.0])) + (v2573 * v1259)) * v1262;
                let v1264 = v2 + v1262;
                let v1265 = (v1261 * v1262) / v1264;
                let v1266 = ((v500 * v1244) + (v1254 * v1256)) + v1265;
                let v4020 = ((Lanes([v3985[0], v3985[1], v3985[2], 0.0, 0.0])) + ((Lanes([v4002[0], v4002[1], v4002[2], 0.0, 0.0])) + (Lanes([v4003[0], 0.0, v4003[1], v4003[2], v4003[3]])))) + ((((Lanes([v4013[0], 0.0, v4013[1], v4013[2], v4013[3]])) + (v2596 * v1261)) - (v2596 * v1265)) / v1264);
                v1989 = v1266;
                v2594 = v4020;
            } else {
                let v1268 = if v1267 == v0 { 1.0 } else { 0.0 };
                let v1990: f64;
                let v2597: Lanes<5>;
                if v1268 != 0.0 {
                    let v1269 = v1243 - v2;
                    let v1270 = v500 * v1269;
                    let v3963 = v3054 * v1269;
                    let v3966 = (Lanes([v3963[0], 0.0, 0.0])) + (v2593 * v500);
                    let v3967 = Lanes([v3966[0], v3966[1], v3966[2], 0.0, 0.0]);
                    v1990 = v1270;
                    v2597 = v3967;
                } else {
                    let v1271 = v2 - v1267;
                    let v3944 = v2593 * v1271;
                    let v1276 = v1267 * ((v1243 + v1118) - v27);
                    let v1277 = v1109 / v620;
                    let v3949 = v3194 * v1277;
                    let v1278 = v2 + v1277;
                    let v3954 = ((v3781 - (Lanes([v3949[0], 0.0, 0.0, 0.0]))) / v620) * v1276;
                    let v1280 = (v1271 * (v1243 - v2)) + (v1276 * v1278);
                    let v1281 = v500 * v1280;
                    let v3959 = v3054 * v1280;
                    let v3962 = (Lanes([v3959[0], 0.0, 0.0, 0.0, 0.0])) + (((Lanes([v3944[0], v3944[1], v3944[2], 0.0, 0.0])) + (((((Lanes([v2593[0], v2593[1], v2593[2], 0.0, 0.0])) + (Lanes([v2573[0], 0.0, v2573[1], v2573[2], v2573[3]]))) * v1267) * v1278) + (Lanes([v3954[0], 0.0, v3954[1], v3954[2], v3954[3]])))) * v500);
                    v1990 = v1281;
                    v2597 = v3962;
                }
                v1989 = v1990;
                v2594 = v2597;
            }
            let v1282 = v727 * v109;
            let v4021 = v3271 * v109;
            let v4022 = v2687 * v727;
            let v4025 = (Lanes([0.0, v4021[0], v4021[1]])) + (Lanes([v4022[0], 0.0, 0.0]));
            let v1283 = v1282 / v502;
            let v4026 = v4025 / v502;
            let v1284 = if v1283 < v763 { 1.0 } else { 0.0 };
            let v1298: f64;
            let v2598: Lanes<3>;
            if v1284 != 0.0 {
                let v1285 = v1283.exp();
                let v4028 = v4026 * v1285;
                v1298 = v1285;
                v2598 = v4028;
            } else {
                let v1286 = v763.exp();
                let v1289 = v1286 * (v2 + (v1283 - v763));
                let v4027 = v4026 * v1286;
                v1298 = v1289;
                v2598 = v4027;
            }
            let v2003: f64;
            let v2599: Lanes<3>;
            if v510 != 0.0 {
                let v1290 = v727 - v1224;
                let v1291 = v1290 * v109;
                let v4037 = v2687 * v1290;
                let v4039 = (((Lanes([0.0, v3271[0], v3271[1]])) - (Lanes([v2538[0], 0.0, 0.0]))) * v109) + (Lanes([v4037[0], 0.0, 0.0]));
                let v1292 = if v1291 < v763 { 1.0 } else { 0.0 };
                let v1304: f64;
                let v2600: Lanes<3>;
                if v1292 != 0.0 {
                    let v1293 = v1291.exp();
                    let v4041 = v4039 * v1293;
                    v1304 = v1293;
                    v2600 = v4041;
                } else {
                    let v1294 = v763.exp();
                    let v1297 = v1294 * (v2 + (v1291 - v763));
                    let v4040 = v4039 * v1294;
                    v1304 = v1297;
                    v2600 = v4040;
                }
                let v1299 = v1298 - v2;
                let v4042 = v3062 * v1299;
                let v1302 = v1301 * v27;
                let v4047 = (v2549 * v27) * v1299;
                let v1307 = (v2 + (v435 * v1304)).sqrt();
                let v1308 = v2 + v1307;
                let v1309 = (v1302 * v1299) / v1308;
                let v1310 = (v508 * v1299) + v1309;
                let v4058 = ((Lanes([v4042[0], 0.0, 0.0])) + (v2598 * v508)) + ((((Lanes([v4047[0], 0.0, 0.0])) + (v2598 * v1302)) - (((v2600 * v435) * (v2515 / (v2994 * v1307))) * v1309)) / v1308);
                v2003 = v1310;
                v2599 = v4058;
            } else {
                let v1311 = v1298 - v2;
                let v1312 = v508 * v1311;
                let v4029 = v3062 * v1311;
                let v4032 = (Lanes([v4029[0], 0.0, 0.0])) + (v2598 * v508);
                v2003 = v1312;
                v2599 = v4032;
            }
            let v1313 = v770 / v463;
            let v4059 = v3346 / v463;
            let v1314 = if v1313 < v763 { 1.0 } else { 0.0 };
            let v1320: f64;
            let v2601: Lanes<3>;
            if v1314 != 0.0 {
                let v1315 = v1313.exp();
                let v4061 = v4059 * v1315;
                v1320 = v1315;
                v2601 = v4061;
            } else {
                let v1316 = v763.exp();
                let v1319 = v1316 * (v2 + (v1313 - v763));
                let v4060 = v4059 * v1316;
                v1320 = v1319;
                v2601 = v4060;
            }
            let v1321 = v1320 - v2;
            let v1322 = v474 * v1321;
            let v4062 = v3035 * v1321;
            let v4065 = (Lanes([v4062[0], 0.0, 0.0])) + (v2601 * v474);
            let v1323 = v1282 / v543;
            let v4066 = v4025 / v543;
            let v1324 = if v1323 < v763 { 1.0 } else { 0.0 };
            let v1330: f64;
            let v2602: Lanes<3>;
            if v1324 != 0.0 {
                let v1325 = v1323.exp();
                let v4068 = v4066 * v1325;
                v1330 = v1325;
                v2602 = v4068;
            } else {
                let v1326 = v763.exp();
                let v1329 = v1326 * (v2 + (v1323 - v763));
                let v4067 = v4066 * v1326;
                v1330 = v1329;
                v2602 = v4067;
            }
            let v1331 = v1330 - v2;
            let v1332 = v551 * v1331;
            let v4069 = v3089 * v1331;
            let v4072 = (Lanes([v4069[0], 0.0, 0.0])) + (v2602 * v551);
            let v1333 = v778 / v476;
            let v4073 = v3357 / v476;
            let v1334 = if v1333 < v763 { 1.0 } else { 0.0 };
            let v1340: f64;
            let v2603: Lanes<6>;
            if v1334 != 0.0 {
                let v1335 = v1333.exp();
                let v4075 = v4073 * v1335;
                v1340 = v1335;
                v2603 = v4075;
            } else {
                let v1336 = v763.exp();
                let v1339 = v1336 * (v2 + (v1333 - v763));
                let v4074 = v4073 * v1336;
                v1340 = v1339;
                v2603 = v4074;
            }
            let v1341 = v1340 - v2;
            let v1342 = v486 * v1341;
            let v4076 = v3044 * v1341;
            let v4079 = (Lanes([v4076[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2603 * v486);
            let v1343 = v1282 / v553;
            let v4080 = v4025 / v553;
            let v1344 = if v1343 < v763 { 1.0 } else { 0.0 };
            let v1350: f64;
            let v2604: Lanes<3>;
            if v1344 != 0.0 {
                let v1345 = v1343.exp();
                let v4082 = v4080 * v1345;
                v1350 = v1345;
                v2604 = v4082;
            } else {
                let v1346 = v763.exp();
                let v1349 = v1346 * (v2 + (v1343 - v763));
                let v4081 = v4080 * v1346;
                v1350 = v1349;
                v2604 = v4081;
            }
            let v1351 = v1350 - v2;
            let v1352 = v560 * v1351;
            let v4083 = v3097 * v1351;
            let v4086 = (Lanes([v4083[0], 0.0, 0.0])) + (v2604 * v560);
            let v1356 = if (if (if v582 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v573 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1173 != 0.0 { 1.0 } else { 0.0 };
            let v1994: f64;
            let v2605: Lanes<3>;
            if v1356 != 0.0 {
                let v1357 = v27 * v1053;
                let v1358 = v30 / v1357;
                let v1359 = v2 - v1358;
                let v1360 = v581 * v1359;
                let v4093 = v3130 * v1359;
                let v4096 = (Lanes([v4093[0], 0.0, 0.0])) + ((((((v3694 * v27) * v1358) * v2676) / v1357) * v2676) * v581);
                let v1361 = if v1360 < v763 { 1.0 } else { 0.0 };
                let v1422: f64;
                let v2606: Lanes<3>;
                if v1361 != 0.0 {
                    let v1362 = v1360.exp();
                    let v4098 = v4096 * v1362;
                    v1422 = v1362;
                    v2606 = v4098;
                } else {
                    let v1363 = v763.exp();
                    let v1366 = v1363 * (v2 + (v1360 - v763));
                    let v4097 = v4096 * v1363;
                    v1422 = v1366;
                    v2606 = v4097;
                }
                let v1367 = v724 * v308;
                let v4099 = v3267 * v308;
                let v4100 = v2909 * v724;
                let v4103 = (Lanes([0.0, v4099[0], v4099[1]])) + (Lanes([v4100[0], 0.0, 0.0]));
                let v4104 = v4103 * v1367;
                let v1371 = ((v1367 * v1367) + v1369).sqrt();
                let v1373 = v1372 - v28;
                let v1374 = v1371.powf(v1373);
                let v1378 = v28 - v2;
                let v1382 = v462 * v1367;
                let v1383 = v1382 * v1367;
                let v1384 = v1378 + v1367;
                let v1386 = (v28 * ((v2 - (v28 * v28)) - ((v154 * v1367) * v1378))) - (v1383 * v1384);
                let v1389 = (v1374 * v1386) * v1388;
                let v1390 = v724 * v30;
                let v4130 = (v3267 * v30) * v581;
                let v4131 = v3130 * v1390;
                let v1392 = v568 * v1389;
                let v4135 = v2531 * v1389;
                let v1393 = (v1390 * v581) / v1392;
                let v4141 = (((Lanes([0.0, v4130[0], v4130[1]])) + (Lanes([v4131[0], 0.0, 0.0]))) - (((Lanes([v4135[0], 0.0, 0.0])) + (((((((v4104 + v4104) * (v2515 / (v2994 * v1371))) * (v1373 * (v1371.powf((v1373 - v2515))))) * v1386) + ((((((v4103 * v154) * v1378) * v2676) * v28) - (((((v4103 * v462) * v1367) + (v4103 * v1382)) * v1384) + (v4103 * v1383))) * v1374)) * v1388) * v568)) * v1393)) / v1392;
                let v1395 = if v1393 < v1394 { 1.0 } else { 0.0 };
                let v1419: f64;
                let v2607: Lanes<3>;
                if v1395 != 0.0 {
                    let v1396 = if v1393 < v763 { 1.0 } else { 0.0 };
                    let v1403: f64;
                    let v2608: Lanes<3>;
                    if v1396 != 0.0 {
                        let v1397 = v1393.exp();
                        let v4156 = v4141 * v1397;
                        v1403 = v1397;
                        v2608 = v4156;
                    } else {
                        let v1398 = v763.exp();
                        let v1401 = v1398 * (v2 + (v1393 - v763));
                        let v4155 = v4141 * v1398;
                        v1403 = v1401;
                        v2608 = v4155;
                    }
                    let v1402 = -v724;
                    let v1405 = (v2 - v1403) / v1393;
                    let v1406 = v2 + v1405;
                    let v1407 = v1402 * v1406;
                    let v4162 = (v3267 * v2676) * v1406;
                    let v4165 = (Lanes([0.0, v4162[0], v4162[1]])) + ((((v2608 * v2676) - (v4141 * v1405)) / v1393) * v1402);
                    v1419 = v1407;
                    v2607 = v4165;
                } else {
                    let v1408 = v724 * v424;
                    let v1409 = v1408 * v1393;
                    let v4143 = (v3267 * v424) * v1393;
                    let v1411 = v1393 * v1410;
                    let v1414 = v2 + (v1412 * v1393);
                    let v1416 = v2 + (v1411 * v1414);
                    let v1417 = v1409 * v1416;
                    let v4154 = (((Lanes([0.0, v4143[0], v4143[1]])) + (v4141 * v1408)) * v1416) + ((((v4141 * v1410) * v1414) + ((v4141 * v1412) * v1411)) * v1409);
                    v1419 = v1417;
                    v2607 = v4154;
                }
                let v1418 = v27 * v591;
                let v1420 = v1418 * v1419;
                let v4167 = (v3147 * v27) * v1419;
                let v1421 = v1420 * v1053;
                let v1423 = v1421 * v1422;
                let v4178 = v2909 * v1423;
                let v1425 = (v1423 * v308) * v31;
                let v4181 = ((((((((Lanes([v4167[0], 0.0, 0.0])) + (v2607 * v1418)) * v1053) + (v3694 * v1420)) * v1422) + (v2606 * v1421)) * v308) + (Lanes([v4178[0], 0.0, 0.0]))) * v31;
                v1994 = v1425;
                v2605 = v4181;
            } else {
                v1994 = v0;
                v2605 = v4087;
            }
            let v1430 = if (if (if v606 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v597 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v718 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1709: f64;
            let v2609: Lanes<3>;
            if v1430 != 0.0 {
                let v1431 = v718 * v310;
                let v4182 = v3259 * v310;
                let v4183 = v2912 * v718;
                let v4186 = (Lanes([0.0, v4182[0], v4182[1]])) + (Lanes([v4183[0], 0.0, 0.0]));
                let v1432 = v2 - v1431;
                let v1433 = v1432.powf(v1094);
                let v4190 = (v4186 * v2676) * (v1094 * (v1432.powf(v3750)));
                let v1434 = v27 * v1433;
                let v1435 = v62 / v1434;
                let v1436 = v2 - v1435;
                let v1437 = v605 * v1436;
                let v4196 = v3171 * v1436;
                let v4199 = (Lanes([v4196[0], 0.0, 0.0])) + ((((((v4190 * v27) * v1435) * v2676) / v1434) * v2676) * v605);
                let v1438 = if v1437 < v763 { 1.0 } else { 0.0 };
                let v1494: f64;
                let v2610: Lanes<3>;
                if v1438 != 0.0 {
                    let v1439 = v1437.exp();
                    let v4201 = v4199 * v1439;
                    v1494 = v1439;
                    v2610 = v4201;
                } else {
                    let v1440 = v763.exp();
                    let v1443 = v1440 * (v2 + (v1437 - v763));
                    let v4200 = v4199 * v1440;
                    v1494 = v1443;
                    v2610 = v4200;
                }
                let v4202 = v4186 * v1431;
                let v1446 = ((v1431 * v1431) + v1369).sqrt();
                let v1448 = v1447 - v60;
                let v1449 = v1446.powf(v1448);
                let v1453 = v60 - v2;
                let v1457 = v462 * v1431;
                let v1458 = v1457 * v1431;
                let v1459 = v1453 + v1431;
                let v1461 = (v60 * ((v2 - (v60 * v60)) - ((v154 * v1431) * v1453))) - (v1458 * v1459);
                let v1463 = (v1449 * v1461) * v1388;
                let v1464 = v718 * v62;
                let v4228 = (v3259 * v62) * v605;
                let v4229 = v3171 * v1464;
                let v1466 = v592 * v1463;
                let v4233 = v2532 * v1463;
                let v1467 = (v1464 * v605) / v1466;
                let v4239 = (((Lanes([0.0, v4228[0], v4228[1]])) + (Lanes([v4229[0], 0.0, 0.0]))) - (((Lanes([v4233[0], 0.0, 0.0])) + (((((((v4202 + v4202) * (v2515 / (v2994 * v1446))) * (v1448 * (v1446.powf((v1448 - v2515))))) * v1461) + ((((((v4186 * v154) * v1453) * v2676) * v60) - (((((v4186 * v462) * v1431) + (v4186 * v1457)) * v1459) + (v4186 * v1458))) * v1449)) * v1388) * v592)) * v1467)) / v1466;
                let v1469 = if v1467 < v1468 { 1.0 } else { 0.0 };
                let v1491: f64;
                let v2611: Lanes<3>;
                if v1469 != 0.0 {
                    let v1470 = if v1467 < v763 { 1.0 } else { 0.0 };
                    let v1477: f64;
                    let v2612: Lanes<3>;
                    if v1470 != 0.0 {
                        let v1471 = v1467.exp();
                        let v4254 = v4239 * v1471;
                        v1477 = v1471;
                        v2612 = v4254;
                    } else {
                        let v1472 = v763.exp();
                        let v1475 = v1472 * (v2 + (v1467 - v763));
                        let v4253 = v4239 * v1472;
                        v1477 = v1475;
                        v2612 = v4253;
                    }
                    let v1476 = -v718;
                    let v1479 = (v2 - v1477) / v1467;
                    let v1480 = v2 + v1479;
                    let v1481 = v1476 * v1480;
                    let v4260 = (v3259 * v2676) * v1480;
                    let v4263 = (Lanes([0.0, v4260[0], v4260[1]])) + ((((v2612 * v2676) - (v4239 * v1479)) / v1467) * v1476);
                    v1491 = v1481;
                    v2611 = v4263;
                } else {
                    let v1482 = v718 * v424;
                    let v1483 = v1482 * v1467;
                    let v4241 = (v3259 * v424) * v1467;
                    let v1484 = v1467 * v1410;
                    let v1486 = v2 + (v1412 * v1467);
                    let v1488 = v2 + (v1484 * v1486);
                    let v1489 = v1483 * v1488;
                    let v4252 = (((Lanes([0.0, v4241[0], v4241[1]])) + (v4239 * v1482)) * v1488) + ((((v4239 * v1410) * v1486) + ((v4239 * v1412) * v1484)) * v1483);
                    v1491 = v1489;
                    v2611 = v4252;
                }
                let v1490 = v27 * v615;
                let v1492 = v1490 * v1491;
                let v4265 = (v3188 * v27) * v1491;
                let v1493 = v1492 * v1433;
                let v1495 = v1493 * v1494;
                let v4276 = v2912 * v1495;
                let v1497 = (v1495 * v310) * v63;
                let v4279 = ((((((((Lanes([v4265[0], 0.0, 0.0])) + (v2611 * v1490)) * v1433) + (v4190 * v1492)) * v1494) + (v2610 * v1493)) * v310) + (Lanes([v4276[0], 0.0, 0.0]))) * v63;
                v1709 = v1497;
                v2609 = v4279;
            } else {
                v1709 = v0;
                v2609 = v3443;
            }
            let v1499 = v1111 * v1498;
            let v4280 = v3785 * v1498;
            let v4283 = (Lanes([v4280[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2559 * v1111);
            let v1501 = v435 * v1500;
            let v4284 = v2566 * v435;
            let v1504 = (v2 + v1499).sqrt();
            let v1505 = v2 + v1504;
            let v1506 = (v1499 - v1111) / v1505;
            let v4292 = ((v4283 - (Lanes([v3785[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) - ((v4283 * (v2515 / (v2994 * v1504))) * v1506)) / v1505;
            let v1508 = (v2 + v1501).sqrt();
            let v1509 = v2 + v1508;
            let v1510 = v1501 / v1509;
            let v4298 = (v4284 - ((v4284 * (v2515 / (v2994 * v1508))) * v1510)) / v1509;
            let v1511 = v27 * v541;
            let v4299 = v3081 * v27;
            let v1512 = v1498 - v2;
            let v4300 = v4299 * v1512;
            let v1515 = (v435 * v541) / v460;
            let v4307 = ((v3081 * v435) - (v3026 * v1515)) / v460;
            let v4308 = v4307 * v1498;
            let v1518 = (v2 + (v1515 * v1498)).sqrt();
            let v1519 = v2 + v1518;
            let v1520 = (v1511 * v1512) / v1519;
            let v4317 = (((Lanes([v4300[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2559 * v1511)) - ((((Lanes([v4308[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2559 * v1515)) * (v2515 / (v2994 * v1518))) * v1520)) / v1519;
            let v1522 = if v1521 == v2 { 1.0 } else { 0.0 };
            let v1591: f64;
            let v2019: f64;
            let v2613: Lanes<7>;
            let v2614: Lanes<5>;
            if v1522 != 0.0 {
                let v1524 = v1523 * v27;
                let v1525 = v1524 * v633;
                let v1527 = v1007 - v1526;
                let v4355 = Lanes([0.0, v2557[0], v2557[1], 0.0, v2557[2]]);
                let v4358 = (v3206 * v1524) * v1527;
                let v1529 = v633 / v646;
                let v1530 = v435 * v1529;
                let v4365 = ((v3206 - (v3215 * v1529)) / v646) * v435;
                let v4366 = v2562 * v1531;
                let v1533 = v1007 + (v1531 * v1526);
                let v4369 = v4365 * v1533;
                let v1536 = (v2 + (v1530 * v1533)).sqrt();
                let v1537 = v2 + v1536;
                let v1538 = (v1525 * v1527) / v1537;
                let v4378 = (((Lanes([0.0, v4358[0], 0.0, 0.0, 0.0])) + ((v4355 - (Lanes([v2562[0], v2562[1], 0.0, v2562[2], 0.0]))) * v1525)) - ((((Lanes([0.0, v4369[0], 0.0, 0.0, 0.0])) + ((v4355 + (Lanes([v4366[0], v4366[1], 0.0, v4366[2], 0.0]))) * v1530)) * (v2515 / (v2994 * v1536))) * v1538)) / v1537;
                let v1540 = (v2 - v1523) * v27;
                let v1541 = v1540 * v633;
                let v1543 = v1498 - v1542;
                let v4380 = Lanes([0.0, v2559[0], v2559[1], v2559[2], v2559[3], v2559[4], v2559[5]]);
                let v4383 = (v3206 * v1540) * v1543;
                let v4387 = v2564 * v1531;
                let v1546 = v1498 + (v1531 * v1542);
                let v4390 = v4365 * v1546;
                let v1549 = (v2 + (v1530 * v1546)).sqrt();
                let v1550 = v2 + v1549;
                let v1551 = (v1541 * v1543) / v1550;
                let v4399 = (((Lanes([0.0, v4383[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4380 - (Lanes([v2564[0], v2564[1], 0.0, 0.0, v2564[2], 0.0, v2564[3]]))) * v1541)) - ((((Lanes([0.0, v4390[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4380 + (Lanes([v4387[0], v4387[1], 0.0, 0.0, v4387[2], 0.0, v4387[3]]))) * v1530)) * (v2515 / (v2994 * v1549))) * v1551)) / v1550;
                v1591 = v1551;
                v2019 = v1538;
                v2613 = v4399;
                v2614 = v4378;
            } else {
                let v1552 = v1523 * v27;
                let v1553 = v1552 * v633;
                let v1554 = v1007 - v2;
                let v4319 = (v3206 * v1552) * v1554;
                let v1556 = v633 / v646;
                let v1557 = v435 * v1556;
                let v4326 = ((v3206 - (v3215 * v1556)) / v646) * v435;
                let v4327 = v4326 * v1007;
                let v1560 = (v2 + (v1557 * v1007)).sqrt();
                let v1561 = v2 + v1560;
                let v1562 = (v1553 * v1554) / v1561;
                let v4336 = (((Lanes([v4319[0], 0.0, 0.0])) + (v2557 * v1553)) - ((((Lanes([v4327[0], 0.0, 0.0])) + (v2557 * v1557)) * (v2515 / (v2994 * v1560))) * v1562)) / v1561;
                let v1564 = (v2 - v1523) * v27;
                let v1565 = v1564 * v633;
                let v4338 = (v3206 * v1564) * v1512;
                let v4342 = v4326 * v1498;
                let v1569 = (v2 + (v1557 * v1498)).sqrt();
                let v1570 = v2 + v1569;
                let v1571 = (v1565 * v1512) / v1570;
                let v4351 = (((Lanes([v4338[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2559 * v1565)) - ((((Lanes([v4342[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2559 * v1557)) * (v2515 / (v2994 * v1569))) * v1571)) / v1570;
                let v4352 = Lanes([0.0, v4351[0], v4351[1], v4351[2], v4351[3], v4351[4], v4351[5]]);
                let v4353 = Lanes([0.0, v4336[0], v4336[1], 0.0, v4336[2]]);
                v1591 = v1571;
                v2019 = v1562;
                v2613 = v4352;
                v2614 = v4353;
            }
            let v1572 = v27 * v641;
            let v1573 = v1526 - v2;
            let v4401 = (v3212 * v27) * v1573;
            let v1575 = v1531 * v435;
            let v1576 = v641 / v651;
            let v1577 = v1575 * v1576;
            let v4409 = (((v3212 - (v3218 * v1576)) / v651) * v1575) * v1526;
            let v1580 = (v2 + (v1577 * v1526)).sqrt();
            let v1581 = v2 + v1580;
            let v1582 = (v1572 * v1573) / v1581;
            let v4419 = v3279 * v1583;
            let v1585 = v1582 + (v732 * v1583);
            let v4421 = ((((Lanes([0.0, v4401[0], 0.0])) + (v2562 * v1572)) - ((((Lanes([0.0, v4409[0], 0.0])) + (v2562 * v1577)) * (v2515 / (v2994 * v1580))) * v1582)) / v1581) + (Lanes([v4419[0], 0.0, v4419[1]]));
            let v1588 = if v8 > v0 { 1.0 } else { 0.0 };
            let v1589 = if (if v1586 > v0 { 1.0 } else { 0.0 }) != 0.0 && v1588 != 0.0 { 1.0 } else { 0.0 };
            let v1713: f64;
            let v1716: f64;
            let v2015: f64;
            let v2023: f64;
            let v2254: f64;
            let v2615: Lanes<6>;
            let v2616: Lanes<10>;
            let v2617: Lanes<7>;
            let v2618: Lanes<10>;
            let v2619: Lanes<10>;
            if v1589 != 0.0 {
                let v1590 = v1520 * v9;
                let v4423 = v4317 * v9;
                let v1592 = v1591 * v9;
                let v4424 = v2613 * v9;
                let v1593 = v8 * v27;
                let v1594 = v1593 * v541;
                let v1596 = v1595 - v2;
                let v4426 = (v3081 * v1593) * v1596;
                let v4430 = v4307 * v1595;
                let v1600 = (v2 + (v1515 * v1595)).sqrt();
                let v1601 = v2 + v1600;
                let v1602 = (v1594 * v1596) / v1601;
                let v4439 = (((Lanes([0.0, 0.0, v4426[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2561 * v1594)) - ((((Lanes([0.0, 0.0, v4430[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2561 * v1515)) * (v2515 / (v2994 * v1600))) * v1602)) / v1601;
                let v1653: f64;
                let v2620: Lanes<10>;
                if v1522 != 0.0 {
                    let v1605 = ((v2 - v1523) * v8) * v27;
                    let v1606 = v1605 * v633;
                    let v1608 = v1595 - v1607;
                    let v4461 = Lanes([v2561[0], v2561[1], 0.0, v2561[2], v2561[3], v2561[4], v2561[5], v2561[6], v2561[7], v2561[8]]);
                    let v4464 = (v3206 * v1605) * v1608;
                    let v1611 = (v435 * v633) / v646;
                    let v4472 = v2563 * v1531;
                    let v1613 = v1595 + (v1531 * v1607);
                    let v4475 = (((v3206 * v435) - (v3215 * v1611)) / v646) * v1613;
                    let v1616 = (v2 + (v1611 * v1613)).sqrt();
                    let v1617 = v2 + v1616;
                    let v1618 = (v1606 * v1608) / v1617;
                    let v4484 = (((Lanes([0.0, 0.0, 0.0, v4464[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4461 - (Lanes([0.0, 0.0, v2563[0], v2563[1], 0.0, 0.0, v2563[2], 0.0, v2563[3], v2563[4]]))) * v1606)) - ((((Lanes([0.0, 0.0, 0.0, v4475[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4461 + (Lanes([0.0, 0.0, v4472[0], v4472[1], 0.0, 0.0, v4472[2], 0.0, v4472[3], v4472[4]]))) * v1611)) * (v2515 / (v2994 * v1616))) * v1618)) / v1617;
                    v1653 = v1618;
                    v2620 = v4484;
                } else {
                    let v1621 = ((v2 - v1523) * v8) * v27;
                    let v1622 = v1621 * v633;
                    let v4441 = (v3206 * v1621) * v1596;
                    let v1625 = (v435 * v633) / v646;
                    let v4449 = (((v3206 * v435) - (v3215 * v1625)) / v646) * v1595;
                    let v1628 = (v2 + (v1625 * v1595)).sqrt();
                    let v1629 = v2 + v1628;
                    let v1630 = (v1622 * v1596) / v1629;
                    let v4458 = (((Lanes([0.0, 0.0, v4441[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2561 * v1622)) - ((((Lanes([0.0, 0.0, v4449[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2561 * v1625)) * (v2515 / (v2994 * v1628))) * v1630)) / v1629;
                    let v4459 = Lanes([v4458[0], v4458[1], 0.0, v4458[2], v4458[3], v4458[4], v4458[5], v4458[6], v4458[7], v4458[8]]);
                    v1653 = v1630;
                    v2620 = v4459;
                }
                let v1631 = if v1586 == v2 { 1.0 } else { 0.0 };
                let v1659: f64;
                let v2621: Lanes<10>;
                if v1631 != 0.0 {
                    let v1633 = v8 * (v541 + v633);
                    let v1634 = v1633 * v357;
                    let v4489 = (((v3081 + v3206) * v8) * v357) + (v2956 * v1633);
                    let v1635 = v1634 * v109;
                    let v1637 = v27 - (v1635.ln());
                    let v4498 = (v2684 * v1637) + (((((v4489 * v109) + (v2687 * v1634)) * (v2515 / v1635)) * v2676) * v107);
                    let v1639 = v759 - (v107 * v1637);
                    let v4500 = v3395 - (Lanes([0.0, 0.0, v4498[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
                    let v1641 = v1639 * v1639;
                    let v4501 = v4500 * v1639;
                    let v4502 = v4501 + v4501;
                    let v1642 = if v1639 < v0 { 1.0 } else { 0.0 };
                    let v1652: f64;
                    let v2622: Lanes<9>;
                    if v1642 != 0.0 {
                        let v1645 = (v1641 + v1640).sqrt();
                        let v1646 = v1645 - v1639;
                        let v1647 = v1643 / v1646;
                        let v4514 = ((((v4502 * (v2515 / (v2994 * v1645))) - v4500) * v1647) * v2676) / v1646;
                        v1652 = v1647;
                        v2622 = v4514;
                    } else {
                        let v1649 = (v1641 + v1640).sqrt();
                        let v1651 = v424 * (v1649 + v1639);
                        let v4507 = ((v4502 * (v2515 / (v2994 * v1649))) + v4500) * v424;
                        v1652 = v1651;
                        v2622 = v4507;
                    }
                    let v1654 = v1602 + v1653;
                    let v4518 = v2956 * v1654;
                    let v1657 = (v1634 + (v1654 * v357)) + v1652;
                    let v4523 = Lanes([v2622[0], v2622[1], 0.0, v2622[2], v2622[3], v2622[4], v2622[5], v2622[6], v2622[7], v2622[8]]);
                    let v1658 = v1652 / v1657;
                    let v4527 = (v4523 - ((((Lanes([0.0, 0.0, 0.0, v4489[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([v4439[0], v4439[1], 0.0, v4439[2], v4439[3], v4439[4], v4439[5], v4439[6], v4439[7], v4439[8]])) + v2620) * v357) + (Lanes([0.0, 0.0, 0.0, v4518[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])))) + v4523) * v1658)) / v1657;
                    v1659 = v1658;
                    v2621 = v4527;
                } else {
                    v1659 = v2;
                    v2621 = v4422;
                }
                let v1660 = v1659 * v1602;
                let v4529 = v4439 * v1659;
                let v4531 = (v2621 * v1602) + (Lanes([v4529[0], v4529[1], 0.0, v4529[2], v4529[3], v4529[4], v4529[5], v4529[6], v4529[7], v4529[8]]));
                let v1661 = v1659 * v1653;
                let v4534 = (v2621 * v1653) + (v2620 * v1659);
                v1713 = v1590;
                v1716 = v1660;
                v2015 = v1592;
                v2023 = v1661;
                v2254 = v1659;
                v2615 = v4423;
                v2616 = v4531;
                v2617 = v4424;
                v2618 = v4534;
                v2619 = v2621;
            } else {
                v1713 = v1520;
                v1716 = v0;
                v2015 = v1591;
                v2023 = v0;
                v2254 = v2;
                v2615 = v4317;
                v2616 = v4422;
                v2617 = v2613;
                v2618 = v4422;
                v2619 = v4422;
            }
            let v1663 = if v1662 == v2 { 1.0 } else { 0.0 };
            let v1710: f64;
            let v2623: Lanes<3>;
            if v1663 != 0.0 {
                let v1664 = v729 + v718;
                let v4538 = (Lanes([v3275[0], v3275[1], 0.0])) + (Lanes([0.0, v3259[0], v3259[1]]));
                let v1670 = (v1667 * v1664) * v1669;
                let v1671 = v1670 * v1664;
                let v4543 = (((v4538 * v1667) * v1669) * v1664) + (v4538 * v1670);
                let v1674 = if (v1672 * v1664) < v0 { 1.0 } else { 0.0 };
                let v1700: f64;
                let v2624: Lanes<3>;
                if v1674 != 0.0 {
                    let v1677 = (v1671 + v1666).sqrt();
                    let v1680 = v1677 - (v1678 * v1664);
                    let v1681 = v1675 / v1680;
                    let v4557 = ((((v4543 * (v2515 / (v2994 * v1677))) - (v4538 * v1678)) * v1681) * v2676) / v1680;
                    v1700 = v1681;
                    v2624 = v4557;
                } else {
                    let v1683 = (v1671 + v1666).sqrt();
                    let v1687 = v424 * (v1683 + (v1684 * v1664));
                    let v4549 = ((v4543 * (v2515 / (v2994 * v1683))) + (v4538 * v1684)) * v424;
                    v1700 = v1687;
                    v2624 = v4549;
                }
                let v1691 = v2 / (v2 - (v90.powf(v1688)));
                let v1693 = v90 * v1692;
                let v1699 = (((v1691 * v1691) * (v90.powf((v1688 - v2)))) * v1688) / v1692;
                let v1701 = if v1700 < v1693 { 1.0 } else { 0.0 };
                let v1711: f64;
                let v2625: Lanes<3>;
                if v1701 != 0.0 {
                    let v1702 = v1700 / v1692;
                    let v1704 = v2 - (v1702.powf(v1688));
                    let v1705 = v2 / v1704;
                    let v4567 = (((((v2624 / v1692) * (v1688 * (v1702.powf((v1688 - v2515))))) * v2676) * v1705) * v2676) / v1704;
                    v1711 = v1705;
                    v2625 = v4567;
                } else {
                    let v4558 = v2624 * v1699;
                    let v1708 = v1691 + ((v1700 - v1693) * v1699);
                    v1711 = v1708;
                    v2625 = v4558;
                }
                v1710 = v1711;
                v2623 = v2625;
            } else {
                v1710 = v2;
                v2623 = v4535;
            }
            let v1712 = v1709 * v1710;
            let v4568 = v2609 * v1710;
            let v4569 = v2623 * v1709;
            let v4572 = (Lanes([v4568[0], 0.0, v4568[1], v4568[2]])) + (Lanes([0.0, v4569[0], v4569[1], v4569[2]]));
            let v1714 = v1713 * v1710;
            let v4574 = v2623 * v1713;
            let v4576 = (v2615 * v1710) + (Lanes([0.0, v4574[0], v4574[1], v4574[2], 0.0, 0.0]));
            let v1715 = v1342 * v1710;
            let v4578 = v2623 * v1342;
            let v4580 = (v4079 * v1710) + (Lanes([0.0, v4578[0], v4578[1], v4578[2], 0.0, 0.0]));
            let v1717 = v1716 * v1710;
            let v4582 = v2623 * v1716;
            let v4584 = (v2616 * v1710) + (Lanes([0.0, 0.0, 0.0, 0.0, v4582[0], v4582[1], v4582[2], 0.0, 0.0, 0.0]));
            let v1718 = v1059 / v623;
            let v4585 = v3198 * v1718;
            let v4588 = (v3703 - (Lanes([v4585[0], 0.0, 0.0]))) / v623;
            let v1720 = v1109 / v620;
            let v4589 = v3194 * v1720;
            let v4592 = (v3781 - (Lanes([v4589[0], 0.0, 0.0, 0.0]))) / v620;
            let v1721 = (v2 + v1718) + v1720;
            let v4595 = (Lanes([v4588[0], v4588[1], v4588[2], 0.0, 0.0])) + (Lanes([v4592[0], 0.0, v4592[1], v4592[2], v4592[3]]));
            let v1723 = v1721 * v1721;
            let v4596 = v4595 * v1721;
            let v4597 = v4596 + v4596;
            let v1724 = if v1721 < v0 { 1.0 } else { 0.0 };
            let v1734: f64;
            let v2626: Lanes<5>;
            if v1724 != 0.0 {
                let v1727 = (v1723 + v1722).sqrt();
                let v1728 = v1727 - v1721;
                let v1729 = v1725 / v1728;
                let v4609 = ((((v4597 * (v2515 / (v2994 * v1727))) - v4595) * v1729) * v2676) / v1728;
                v1734 = v1729;
                v2626 = v4609;
            } else {
                let v1731 = (v1723 + v1722).sqrt();
                let v1733 = v424 * (v1731 + v1721);
                let v4602 = ((v4597 * (v2515 / (v2994 * v1731))) + v4595) * v424;
                v1734 = v1733;
                v2626 = v4602;
            }
            let v1735 = v1734 * v1163;
            let v1736 = v346 / v1735;
            let v4616 = ((Lanes([v2950[0], 0.0, 0.0, 0.0, 0.0])) - (((v2626 * v1163) + (v3884 * v1734)) * v1736)) / v1735;
            let v1737 = if v1736 < v22 { 1.0 } else { 0.0 };
            let v1738: f64;
            let v2627: Lanes<5>;
            if v1737 != 0.0 {
                v1738 = v22;
                v2627 = v4617;
            } else {
                v1738 = v1736;
                v2627 = v4616;
            }
            let v1739 = v154 * v1738;
            let v4618 = v2627 * v154;
            let v1740 = v27 * v107;
            let v1742 = v1741 - v2;
            let v4620 = (v2684 * v27) * v1742;
            let v4625 = ((Lanes([v4620[0], 0.0, 0.0])) + (v2560 * v1740)) + (Lanes([0.0, v3275[0], v3275[1]]));
            let v1745 = ((v1740 * v1742) + v729) / v1739;
            let v4626 = v4618 * v1745;
            let v4630 = ((Lanes([v4625[0], 0.0, v4625[1], v4625[2], 0.0, 0.0])) - (Lanes([v4626[0], v4626[1], 0.0, v4626[2], v4626[3], v4626[4]]))) / v1739;
            let v1746 = if v1170 > v0 { 1.0 } else { 0.0 };
            let v1960: f64;
            let v2628: Lanes<5>;
            if v1746 != 0.0 {
                let v1748 = if v1747 == v2 { 1.0 } else { 0.0 };
                let v1910: f64;
                let v2629: Lanes<5>;
                if v1748 != 0.0 {
                    let v1750 = if v718 < v1749 { 1.0 } else { 0.0 };
                    let v1911: f64;
                    let v2630: Lanes<5>;
                    if v1750 != 0.0 {
                        let v1753 = (-v1170) / v1752;
                        let v4809 = (v3902 * v2676) / v1752;
                        let v1754 = if v1753 < v763 { 1.0 } else { 0.0 };
                        let v1761: f64;
                        let v2631: Lanes<5>;
                        if v1754 != 0.0 {
                            let v1755 = v1753.exp();
                            let v4811 = v4809 * v1755;
                            v1761 = v1755;
                            v2631 = v4811;
                        } else {
                            let v1756 = v763.exp();
                            let v1759 = v1756 * (v2 + (v1753 - v763));
                            let v4810 = v4809 * v1756;
                            v1761 = v1759;
                            v2631 = v4810;
                        }
                        let v1760 = v1749 - v718;
                        let v1762 = v1760 * v1761;
                        let v4813 = (v3259 * v2676) * v1761;
                        let v4816 = (Lanes([0.0, 0.0, v4813[0], v4813[1], 0.0])) + (v2631 * v1760);
                        let v1764 = -v1763;
                        let v1766 = v1762.powf(v1765);
                        let v1767 = v1764 * v1766;
                        let v4822 = (v2546 * v2676) * v1766;
                        let v4825 = (Lanes([v4822[0], 0.0, 0.0, 0.0, 0.0])) + ((v4816 * (v1765 * (v1762.powf((v1765 - v2515))))) * v1764);
                        let v1768 = if v1767 < v763 { 1.0 } else { 0.0 };
                        let v1777: f64;
                        let v2632: Lanes<5>;
                        if v1768 != 0.0 {
                            let v1769 = v1767.exp();
                            let v4827 = v4825 * v1769;
                            v1777 = v1769;
                            v2632 = v4827;
                        } else {
                            let v1770 = v763.exp();
                            let v1773 = v1770 * (v2 + (v1767 - v763));
                            let v4826 = v4825 * v1770;
                            v1777 = v1773;
                            v2632 = v4826;
                        }
                        let v1775 = v1774 / v1763;
                        let v1776 = v1775 * v1762;
                        let v4831 = (((v2546 * v1775) * v2676) / v1763) * v1762;
                        let v1778 = v1776 * v1777;
                        let v4837 = (((Lanes([v4831[0], 0.0, 0.0, 0.0, 0.0])) + (v4816 * v1775)) * v1777) + (v2632 * v1776);
                        v1911 = v1778;
                        v2630 = v4837;
                    } else {
                        v1911 = v0;
                        v2630 = v4617;
                    }
                    v1910 = v1911;
                    v2629 = v2630;
                } else {
                    let v1779 = if v1747 == v27 { 1.0 } else { 0.0 };
                    let v1912: f64;
                    let v2633: Lanes<5>;
                    if v1779 != 0.0 {
                        let v1780 = if v718 < v820 { 1.0 } else { 0.0 };
                        let v1913: f64;
                        let v2634: Lanes<5>;
                        if v1780 != 0.0 {
                            let v1785 = (v27 * v1781) / (v1783 * v1783);
                            let v1786 = v820 - v718;
                            let v4686 = v3423 - v3422;
                            let v1787 = v1786 / v1091;
                            let v4688 = Lanes([v4686[0], v4686[1], v4686[2], 0.0]);
                            let v1790 = ((v27 * v1787) / v1785).sqrt();
                            let v4695 = ((((v4688 - (v2572 * v1787)) / v1091) * v27) / v1785) * (v2515 / (v2994 * v1790));
                            let v1792 = if v1791 == v0 { 1.0 } else { 0.0 };
                            let v1799: f64;
                            let v2635: Lanes<4>;
                            if v1792 != 0.0 {
                                v1799 = v1783;
                                v2635 = v3489;
                            } else {
                                let v1796 = v2 - (v424 * v1793);
                                let v4697 = (v2574 * v424) * v2676;
                                let v1797 = v1783 * v1796;
                                let v1798 = v1797 * v1796;
                                let v4701 = ((v4697 * v1783) * v1796) + (v4697 * v1797);
                                v1799 = v1798;
                                v2635 = v4701;
                            }
                            let v4705 = v4695 * v1790;
                            let v4707 = v2635 * v1799;
                            let v1804 = ((v1790 * v1790) + (v1799 * v1799)).sqrt();
                            let v1805 = (v1790 * v1799) / v1804;
                            let v4715 = (((v4695 * v1799) + (v2635 * v1790)) - ((((v4705 + v4705) + (v4707 + v4707)) * (v2515 / (v2994 * v1804))) * v1805)) / v1804;
                            let v1806 = v1786 / v1805;
                            let v4718 = (v4688 - (v4715 * v1806)) / v1805;
                            let v1807 = v424 * v1805;
                            let v4719 = v4715 * v424;
                            let v1808 = v1807 * v1785;
                            let v4720 = v4719 * v1785;
                            let v1810 = v1806 + (v1808 * v1091);
                            let v4724 = v4718 + ((v4720 * v1091) + (v2572 * v1808));
                            let v1837: f64;
                            let v2636: Lanes<5>;
                            if v1792 != 0.0 {
                                let v4759 = Lanes([v4724[0], 0.0, v4724[1], v4724[2], v4724[3]]);
                                v1837 = v1810;
                                v2636 = v4759;
                            } else {
                                let v1812 = v27 * v1811;
                                let v1820 = v906 * (v2 + (v1812 * (v2 + (v27 * v1793))));
                                let v1821 = v1170 / v1820;
                                let v4728 = (((v2574 * v27) * v1812) * v906) * v1821;
                                let v1822 = ((v2 + v1811) / (v2 + v1812)) - v1821;
                                let v4733 = v4720 * v1822;
                                let v1824 = v1806 - (v1808 * v1822);
                                let v4738 = (Lanes([v4718[0], 0.0, v4718[1], v4718[2], v4718[3]])) - ((Lanes([v4733[0], 0.0, v4733[1], v4733[2], v4733[3]])) + ((((v3902 - (Lanes([v4728[0], 0.0, v4728[1], v4728[2], v4728[3]]))) / v1820) * v2676) * v1808));
                                let v1825 = v1824 - v1810;
                                let v4739 = Lanes([v4724[0], 0.0, v4724[1], v4724[2], v4724[3]]);
                                let v4741 = (v4738 - v4739) * v1825;
                                let v1827 = v42 * v1806;
                                let v1828 = v1827 * v1806;
                                let v4750 = (((((v4718 * v42) * v1806) + (v4718 * v1827)) * v1829) + (v2575 * v1828)) / v906;
                                let v1834 = ((v1825 * v1825) + ((v1828 * v1829) / v906)).sqrt();
                                let v1836 = v424 * ((v1824 + v1810) + v1834);
                                let v4758 = ((v4738 + v4739) + (((v4741 + v4741) + (Lanes([v4750[0], 0.0, v4750[1], v4750[2], v4750[3]]))) * (v2515 / (v2994 * v1834)))) * v424;
                                v1837 = v1836;
                                v2636 = v4758;
                            }
                            let v1839 = (v1837 - v1806) / v1837;
                            let v4764 = ((v2636 - (Lanes([v4718[0], 0.0, v4718[1], v4718[2], v4718[3]]))) - (v2636 * v1839)) / v1837;
                            let v1842 = if (v1839.abs()) > v1841 { 1.0 } else { 0.0 };
                            let v1914: f64;
                            let v2637: Lanes<5>;
                            if v1842 != 0.0 {
                                let v1843 = v1807 / v1839;
                                let v4779 = ((Lanes([v4719[0], 0.0, v4719[1], v4719[2], v4719[3]])) - (v4764 * v1843)) / v1839;
                                let v1846 = v1844 / v1845;
                                let v1847 = v1846 * v1837;
                                let v4783 = (((v2550 * v1846) * v2676) / v1845) * v1837;
                                let v1848 = v1847 * v1843;
                                let v4790 = v2550 * v2676;
                                let v1850 = (-v1845) / v1837;
                                let v4794 = ((Lanes([v4790[0], 0.0, 0.0, 0.0, 0.0])) - (v2636 * v1850)) / v1837;
                                let v1851 = v1850.exp();
                                let v1852 = v1799 / v1843;
                                let v1853 = v2 + v1852;
                                let v1855 = (v1850 * v1853).exp();
                                let v1856 = v1851 - v1855;
                                let v1857 = v1848 * v1856;
                                let v4807 = (((((Lanes([v4783[0], 0.0, 0.0, 0.0, 0.0])) + (v2636 * v1846)) * v1843) + (v4779 * v1847)) * v1856) + (((v4794 * v1851) - (((v4794 * v1853) + ((((Lanes([v2635[0], 0.0, v2635[1], v2635[2], v2635[3]])) - (v4779 * v1852)) / v1843) * v1850)) * v1855)) * v1848);
                                v1914 = v1857;
                                v2637 = v4807;
                            } else {
                                let v1858 = v1844 * v1799;
                                let v4766 = v2550 * v2676;
                                let v1860 = (-v1845) / v1837;
                                let v1861 = v1860.exp();
                                let v1862 = v1858 * v1861;
                                let v4772 = (v2635 * v1844) * v1861;
                                let v4775 = (Lanes([v4772[0], 0.0, v4772[1], v4772[2], v4772[3]])) + (((((Lanes([v4766[0], 0.0, 0.0, 0.0, 0.0])) - (v2636 * v1860)) / v1837) * v1861) * v1858);
                                v1914 = v1862;
                                v2637 = v4775;
                            }
                            v1913 = v1914;
                            v2634 = v2637;
                        } else {
                            v1913 = v0;
                            v2634 = v4617;
                        }
                        v1912 = v1913;
                        v2633 = v2634;
                    } else {
                        let v1863 = if v1747 == v154 { 1.0 } else { 0.0 };
                        let v1915: f64;
                        let v2638: Lanes<5>;
                        if v1863 != 0.0 {
                            let v1864 = if v718 < v1749 { 1.0 } else { 0.0 };
                            let v1916: f64;
                            let v2639: Lanes<5>;
                            if v1864 != 0.0 {
                                let v1865 = v1749 - v718;
                                let v4631 = v3259 * v2676;
                                let v1866 = v1865.powf(v1765);
                                let v1868 = v1867 + v1170;
                                let v1869 = v1170 / v1868;
                                let v1870 = v2 - v1869;
                                let v1872 = v1870.powf(v1871);
                                let v1873 = v1866 * v1872;
                                let v4644 = (v4631 * (v1765 * (v1865.powf((v1765 - v2515))))) * v1872;
                                let v4647 = (Lanes([0.0, 0.0, v4644[0], v4644[1], 0.0])) + (((((v3902 - (v3902 * v1869)) / v1868) * v2676) * (v1871 * (v1870.powf((v1871 - v2515))))) * v1866);
                                let v1874 = if v1791 == v0 { 1.0 } else { 0.0 };
                                let v1898: f64;
                                let v2640: Lanes<5>;
                                if v1874 != 0.0 {
                                    v1898 = v1873;
                                    v2640 = v4647;
                                } else {
                                    let v1877 = (v1170 - v1875) / v1867;
                                    let v4648 = v3902 / v1867;
                                    let v1880 = (v1877 - v2) / v1879;
                                    let v4649 = v4648 / v1879;
                                    let v1881 = if v1877 < v2 { 1.0 } else { 0.0 };
                                    let v1893: f64;
                                    let v2641: Lanes<5>;
                                    if v1881 != 0.0 {
                                        let v1882 = v1880.exp();
                                        let v1883 = v2 + v1882;
                                        let v4659 = ((v4649 * v1882) * (v2515 / v1883)) * v1879;
                                        let v1886 = v2 + (v1879 * (v1883.ln()));
                                        v1893 = v1886;
                                        v2641 = v4659;
                                    } else {
                                        let v1888 = (-v1880).exp();
                                        let v1889 = v2 + v1888;
                                        let v1892 = v1877 + (v1879 * (v1889.ln()));
                                        let v4655 = v4648 + ((((v4649 * v2676) * v1888) * (v2515 / v1889)) * v1879);
                                        v1893 = v1892;
                                        v2641 = v4655;
                                    }
                                    let v1895 = v1893.powf(v1894);
                                    let v1896 = v1873 * v1895;
                                    let v4666 = (v4647 * v1895) + ((v2641 * (v1894 * (v1893.powf((v1894 - v2515))))) * v1873);
                                    v1898 = v1896;
                                    v2640 = v4666;
                                }
                                let v1897 = -v1763;
                                let v1899 = v1897 * v1898;
                                let v4668 = (v2546 * v2676) * v1898;
                                let v4671 = (Lanes([v4668[0], 0.0, 0.0, 0.0, 0.0])) + (v2640 * v1897);
                                let v1900 = if v1899 < v763 { 1.0 } else { 0.0 };
                                let v1908: f64;
                                let v2642: Lanes<5>;
                                if v1900 != 0.0 {
                                    let v1901 = v1899.exp();
                                    let v4673 = v4671 * v1901;
                                    v1908 = v1901;
                                    v2642 = v4673;
                                } else {
                                    let v1902 = v763.exp();
                                    let v1905 = v1902 * (v2 + (v1899 - v763));
                                    let v4672 = v4671 * v1902;
                                    v1908 = v1905;
                                    v2642 = v4672;
                                }
                                let v1906 = v1774 / v1763;
                                let v1907 = v1906 * v1865;
                                let v4677 = (((v2546 * v1906) * v2676) / v1763) * v1865;
                                let v4678 = v4631 * v1906;
                                let v1909 = v1907 * v1908;
                                let v4682 = ((Lanes([v4677[0], 0.0, 0.0])) + (Lanes([0.0, v4678[0], v4678[1]]))) * v1908;
                                let v4685 = (Lanes([v4682[0], 0.0, v4682[1], v4682[2], 0.0])) + (v2642 * v1907);
                                v1916 = v1909;
                                v2639 = v4685;
                            } else {
                                v1916 = v0;
                                v2639 = v4617;
                            }
                            v1915 = v1916;
                            v2638 = v2639;
                        } else {
                            v1915 = v0;
                            v2638 = v4617;
                        }
                        v1912 = v1915;
                        v2633 = v2638;
                    }
                    v1910 = v1912;
                    v2629 = v2633;
                }
                let v1917 = if v1910 > v0 { 1.0 } else { 0.0 };
                let v1961: f64;
                let v2643: Lanes<5>;
                if v1917 != 0.0 {
                    let v1919 = if v1918 == v2 { 1.0 } else { 0.0 };
                    let v1962: f64;
                    let v2644: Lanes<5>;
                    if v1919 != 0.0 {
                        let v1921 = v1920 + v1739;
                        let v4842 = (Lanes([v2541[0], 0.0, 0.0, 0.0, 0.0])) + v4618;
                        let v1922 = v1170 * v1921;
                        let v1923 = v107 / v1922;
                        let v1924 = v1164 / v449;
                        let v4850 = v3020 * v1924;
                        let v4855 = v3054 * v1924;
                        let v1928 = v1927 / v1921;
                        let v1929 = (v1923 + (v1924 * v500)) + v1928;
                        let v4863 = ((((Lanes([v2684[0], 0.0, 0.0, 0.0, 0.0])) - (((v3902 * v1921) + (v4842 * v1170)) * v1923)) / v1922) + ((((v3887 - (Lanes([v4850[0], 0.0, 0.0, 0.0, 0.0]))) / v449) * v500) + (Lanes([v4855[0], 0.0, 0.0, 0.0, 0.0])))) + (((Lanes([v2540[0], 0.0, 0.0, 0.0, 0.0])) - (v4842 * v1928)) / v1921);
                        let v1930 = if v1747 == v154 { 1.0 } else { 0.0 };
                        let v1963: f64;
                        let v2645: Lanes<5>;
                        if v1930 != 0.0 {
                            let v1932 = (v1910 - v1929) / v1665;
                            let v4875 = (v2629 - v4863) / v1665;
                            let v1933 = if v1910 < v1929 { 1.0 } else { 0.0 };
                            let v1945: f64;
                            let v2646: Lanes<5>;
                            if v1933 != 0.0 {
                                let v1934 = v1932.exp();
                                let v1935 = v2 + v1934;
                                let v1938 = v1910 - (v1665 * (v1935.ln()));
                                let v4886 = v2629 - (((v4875 * v1934) * (v2515 / v1935)) * v1665);
                                v1945 = v1938;
                                v2646 = v4886;
                            } else {
                                let v1940 = (-v1932).exp();
                                let v1941 = v2 + v1940;
                                let v1944 = v1929 - (v1665 * (v1941.ln()));
                                let v4881 = v4863 - ((((v4875 * v2676) * v1940) * (v2515 / v1941)) * v1665);
                                v1945 = v1944;
                                v2646 = v4881;
                            }
                            let v1946 = v1170 * v1945;
                            let v4889 = (v3902 * v1945) + (v2646 * v1170);
                            v1963 = v1946;
                            v2645 = v4889;
                        } else {
                            let v1947 = v1170 * v1910;
                            let v1949 = v1910 + v1929;
                            let v1950 = (v1947 * v1929) / v1949;
                            let v4873 = (((((v3902 * v1910) + (v2629 * v1170)) * v1929) + (v4863 * v1947)) - ((v2629 + v4863) * v1950)) / v1949;
                            v1963 = v1950;
                            v2645 = v4873;
                        }
                        v1962 = v1963;
                        v2644 = v2645;
                    } else {
                        let v1951 = v1170 * v1910;
                        let v4840 = (v3902 * v1910) + (v2629 * v1170);
                        v1962 = v1951;
                        v2644 = v4840;
                    }
                    v1961 = v1962;
                    v2643 = v2644;
                } else {
                    v1961 = v0;
                    v2643 = v4617;
                }
                v1960 = v1961;
                v2628 = v2643;
            } else {
                v1960 = v0;
                v2628 = v4617;
            }
            let v1952 = if v1118 > v0 { 1.0 } else { 0.0 };
            let v1954: f64;
            let v2647: Lanes<4>;
            if v1952 != 0.0 {
                let v1953 = v107 * v3803;
                let v4893 = v2684 * v3803;
                let v4896 = (Lanes([v4893[0], 0.0, 0.0, 0.0])) + ((v2573 * (v2515 / v1118)) * v107);
                v1954 = v1953;
                v2647 = v4896;
            } else {
                let v4890 = Lanes([0.0, v3263[0], 0.0, v3263[1]]);
                v1954 = v721;
                v2647 = v4890;
            }
            let v2000: f64;
            let v2648: Lanes<3>;
            if v510 != 0.0 {
                let v4898 = Lanes([v3259[0], v3259[1], 0.0]);
                v2000 = v718;
                v2648 = v4898;
            } else {
                let v4897 = Lanes([v3263[0], 0.0, v3263[1]]);
                v2000 = v721;
                v2648 = v4897;
            }
            let v1955 = v724 - v1954;
            let v1957 = v1954 - v718;
            let v4909 = (v3463 * v1957) + ((v2647 - (Lanes([0.0, v3259[0], v3259[1], 0.0]))) * v873);
            let v4913 = v2647 * v1960;
            let v4916 = (((v3902 * v1955) + (((Lanes([0.0, v3267[0], v3267[1], 0.0, 0.0])) - (Lanes([v2647[0], 0.0, v2647[1], v2647[2], v2647[3]]))) * v1170)) + (Lanes([v4909[0], 0.0, v4909[1], v4909[2], v4909[3]]))) - ((v2628 * v1954) + (Lanes([v4913[0], 0.0, v4913[1], v4913[2], v4913[3]])));
            let v4917 = v3287 * v737;
            let v4918 = v4917 + v4917;
            let v1967 = (v737 * v737) / v1927;
            let v4919 = v2540 * v1967;
            let v4923 = ((Lanes([v4918[0], 0.0, v4918[1]])) - (Lanes([0.0, v4919[0], 0.0]))) / v1927;
            let v4926 = (Lanes([0.0, v4916[0], v4916[1], v4916[2], v4916[3], v4916[4]])) + (Lanes([v4923[0], v4923[1], v4923[2], 0.0, 0.0, 0.0]));
            let v1969 = v758 * v758;
            let v4927 = v3326 * v758;
            let v4929 = (v4927 + v4927) * v1970;
            let v4930 = v2551 * v1969;
            let v4933 = (Lanes([v4929[0], v4929[1], 0.0, v4929[2], v4929[3], v4929[4], v4929[5], v4929[6], v4929[7]])) + (Lanes([0.0, 0.0, v4930[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v1974 = v751 * v751;
            let v4937 = v3307 * v751;
            let v4939 = (v4937 + v4937) * v1975;
            let v4940 = v2553 * v1974;
            let v4943 = (Lanes([0.0, v4939[0], v4939[1]])) + (Lanes([v4940[0], 0.0, 0.0]));
            let v1979 = v748 * v748;
            let v4946 = v3303 * v748;
            let v4948 = (v4946 + v4946) * v1980;
            let v4949 = v2555 * v1979;
            let v4952 = (Lanes([0.0, v4948[0], v4948[1]])) + (Lanes([v4949[0], 0.0, 0.0]));
            let v4955 = v3291 * v740;
            let v4956 = v4955 + v4955;
            let v1985 = (v740 * v740) / v1920;
            let v4957 = v2541 * v1985;
            let v4961 = ((Lanes([v4956[0], 0.0, v4956[1]])) - (Lanes([0.0, v4957[0], 0.0]))) / v1920;
            let v4965 = v3275 * v1745;
            let v4967 = (v4630 * v729) + (Lanes([0.0, 0.0, v4965[0], v4965[1], 0.0, 0.0]));
            let v4972 = v3267 * v16;
            let v1997 = ((((v1989 + v1322) + (v16 * v724)) - v1994) + v1216) + v1195;
            let v4980 = ((((v2594 + (Lanes([v4065[0], v4065[1], v4065[2], 0.0, 0.0]))) + (Lanes([0.0, v4972[0], v4972[1], 0.0, 0.0]))) - (Lanes([v2605[0], v2605[1], v2605[2], 0.0, 0.0]))) + (Lanes([0.0, v3940[0], v3940[1], 0.0, 0.0]))) + (Lanes([v3921[0], v3921[1], v3921[2], 0.0, 0.0]));
            let v4982 = v3267 * v1997;
            let v4984 = (v4980 * v724) + (Lanes([0.0, v4982[0], v4982[1], 0.0, 0.0]));
            let v4987 = v4572 * v2000;
            let v4988 = v2648 * v1712;
            let v4991 = (Lanes([v4987[0], v4987[1], v4987[2], v4987[3], 0.0])) + (Lanes([0.0, 0.0, v4988[0], v4988[1], v4988[2]]));
            let v2005 = (v2003 + v1332) + v1352;
            let v4995 = (v2599 + v4072) + v4086;
            let v4997 = v3271 * v2005;
            let v4999 = (v4995 * v727) + (Lanes([0.0, v4997[0], v4997[1]]));
            let v2009 = v16 * v754;
            let v5003 = v3316 * v16;
            let v2010 = (v1714 + v1715) + v2009;
            let v5004 = Lanes([0.0, v5003[0], v5003[1], v5003[2], v5003[3], v5003[4]]);
            let v5007 = v3316 * v2010;
            let v5009 = (((v4576 + v4580) + v5004) * v754) + (Lanes([0.0, v5007[0], v5007[1], v5007[2], v5007[3], v5007[4]]));
            let v5011 = (((((((((Lanes([0.0, 0.0, v4926[0], v4926[1], v4926[2], 0.0, v4926[3], v4926[4], v4926[5], 0.0, 0.0])) + (Lanes([v4933[0], v4933[1], 0.0, v4933[2], 0.0, v4933[3], v4933[4], v4933[5], v4933[6], v4933[7], v4933[8]]))) + (Lanes([0.0, 0.0, 0.0, v4943[0], 0.0, 0.0, 0.0, 0.0, 0.0, v4943[1], v4943[2]]))) + (Lanes([0.0, 0.0, 0.0, v4952[0], 0.0, 0.0, 0.0, v4952[1], 0.0, 0.0, v4952[2]]))) + (Lanes([0.0, v4961[0], 0.0, v4961[1], 0.0, v4961[2], 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4967[0], v4967[1], v4967[2], v4967[3], v4967[4], v4967[5], 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4984[0], v4984[1], 0.0, v4984[2], v4984[3], v4984[4], 0.0, 0.0]))) - (Lanes([0.0, 0.0, 0.0, v4991[0], 0.0, v4991[1], v4991[2], v4991[3], v4991[4], 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4999[0], v4999[1], v4999[2], 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v5009[0], 0.0, v5009[1], v5009[2], v5009[3], v5009[4], 0.0, v5009[5]]));
            let v5013 = v3328 * v1717;
            let v5015 = (v4584 * v759) + (Lanes([v5013[0], v5013[1], 0.0, 0.0, v5013[2], v5013[3], v5013[4], v5013[5], v5013[6], v5013[7]]));
            let v2016 = v754 - v760;
            let v5023 = ((Lanes([0.0, v3316[0], v3316[1], v3316[2], v3316[3], v3316[4]])) - (Lanes([v3331[0], 0.0, 0.0, v3331[1], 0.0, v3331[2]]))) * v2015;
            let v5025 = (v2617 * v2016) + (Lanes([v5023[0], 0.0, v5023[1], v5023[2], v5023[3], v5023[4], v5023[5]]));
            let v2020 = v718 - v732;
            let v5032 = ((Lanes([0.0, v3259[0], v3259[1]])) - (Lanes([v3279[0], 0.0, v3279[1]]))) * v2019;
            let v5034 = (v2614 * v2020) + (Lanes([v5032[0], 0.0, v5032[1], v5032[2], 0.0]));
            let v2024 = v759 - v761;
            let v5041 = ((Lanes([v3328[0], v3328[1], 0.0, v3328[2], v3328[3], v3328[4], v3328[5], v3328[6], v3328[7]])) - (Lanes([0.0, 0.0, v3334[0], 0.0, 0.0, v3334[1], 0.0, v3334[2], v3334[3]]))) * v2023;
            let v5043 = (v2618 * v2024) + (Lanes([v5041[0], v5041[1], v5041[2], 0.0, v5041[3], v5041[4], v5041[5], v5041[6], v5041[7], v5041[8]]));
            let v5047 = v3279 * v1585;
            let v5049 = (v4421 * v732) + (Lanes([v5047[0], 0.0, v5047[1]]));
            let v2028 = (((((((((((((((((v1170 * v1955) + (v873 * v1957)) - (v1960 * v1954)) + v1967) + (v1969 * v1970)) + (v1974 * v1975)) + (v1979 * v1980)) + v1985) + (v1745 * v729)) + (v1997 * v724)) - (v1712 * v2000)) + (v2005 * v727)) + (v2010 * v754)) + (v1717 * v759)) + (v2015 * v2016)) + (v2019 * v2020)) + (v2023 * v2024)) + (v1585 * v732);
            let v5051 = (((((Lanes([v5011[0], v5011[1], v5011[2], 0.0, v5011[3], v5011[4], v5011[5], v5011[6], v5011[7], v5011[8], v5011[9], v5011[10]])) + (Lanes([v5015[0], v5015[1], 0.0, v5015[2], v5015[3], 0.0, v5015[4], v5015[5], v5015[6], v5015[7], v5015[8], v5015[9]]))) + (Lanes([0.0, 0.0, 0.0, v5025[0], v5025[1], 0.0, v5025[2], v5025[3], v5025[4], v5025[5], 0.0, v5025[6]]))) + (Lanes([0.0, 0.0, 0.0, v5034[0], v5034[1], 0.0, 0.0, v5034[2], v5034[3], v5034[4], 0.0, 0.0]))) + (Lanes([v5043[0], v5043[1], 0.0, v5043[2], v5043[3], 0.0, v5043[4], v5043[5], v5043[6], v5043[7], v5043[8], v5043[9]]))) + (Lanes([0.0, 0.0, 0.0, v5049[0], v5049[1], 0.0, 0.0, 0.0, v5049[2], 0.0, 0.0, 0.0]));
            let v2030 = v2 - v2029;
            let v2031 = v2030 * v316;
            let v5052 = v2923 * v2030;
            let v2032 = v2031 * v1059;
            let v5053 = v5052 * v1059;
            let v5056 = (Lanes([v5053[0], 0.0, 0.0])) + (v3703 * v2031);
            let v5057 = Lanes([0.0, v3271[0], v3271[1]]);
            let v5058 = Lanes([v3660[0], 0.0, 0.0]);
            let v2034 = (v727 - v1033) / v1034;
            let v5060 = v3661 * v2034;
            let v5063 = ((v5057 - v5058) - (Lanes([v5060[0], 0.0, 0.0]))) / v1034;
            let v2035 = if v727 < v1033 { 1.0 } else { 0.0 };
            let v2048: f64;
            let v2649: Lanes<3>;
            if v2035 != 0.0 {
                let v2036 = v2034.exp();
                let v2037 = v2 + v2036;
                let v2038 = v2037.ln();
                let v5076 = v3661 * v2038;
                let v2040 = v727 - (v1034 * v2038);
                let v5080 = v5057 - ((Lanes([v5076[0], 0.0, 0.0])) + (((v5063 * v2036) * (v2515 / v2037)) * v1034));
                v2048 = v2040;
                v2649 = v5080;
            } else {
                let v2042 = (-v2034).exp();
                let v2043 = v2 + v2042;
                let v2044 = v2043.ln();
                let v5068 = v3661 * v2044;
                let v2046 = v1033 - (v1034 * v2044);
                let v5072 = v5058 - ((Lanes([v5068[0], 0.0, 0.0])) + ((((v5063 * v2676) * v2042) * (v2515 / v2043)) * v1034));
                v2048 = v2046;
                v2649 = v5072;
            }
            let v2047 = v2029 * v316;
            let v5083 = v2909 * v2048;
            let v2050 = v2 - (v2048 * v308);
            let v2052 = v2 - (v2050.powf(v1052));
            let v5091 = v3695 * v2052;
            let v2056 = (v1054 * v2052) + (v154 * (v727 - v2048));
            let v2057 = v2047 * v2056;
            let v5098 = (v2923 * v2029) * v2056;
            let v5101 = (Lanes([v5098[0], 0.0, 0.0])) + ((((Lanes([v5091[0], 0.0, 0.0])) + ((((((v2649 * v308) + (Lanes([v5083[0], 0.0, 0.0]))) * v2676) * (v1052 * (v2050.powf(v3691)))) * v2676) * v1054)) + ((v5057 - v2649) * v154)) * v2047);
            let v2059 = v2058 * v332;
            let v2060 = v2059 * v1109;
            let v5103 = (v2942 * v2058) * v1109;
            let v5106 = (Lanes([v5103[0], 0.0, 0.0, 0.0])) + (v3781 * v2059);
            let v2061 = v667 * v454;
            let v5109 = (v3229 * v454) + (v3023 * v667);
            let v2062 = v424 * v2061;
            let v5110 = v5109 * v424;
            let v2063 = v2062 * v1117;
            let v5111 = v5110 * v1117;
            let v2064 = v2063 * v1734;
            let v5115 = ((Lanes([v5111[0], 0.0, 0.0])) + (v3795 * v2062)) * v1734;
            let v5118 = (Lanes([v5115[0], v5115[1], v5115[2], 0.0, 0.0])) + (v2626 * v2063);
            let v2065 = v2062 * v1126;
            let v5119 = v5110 * v1126;
            let v2066 = v2065 * v1734;
            let v5123 = ((Lanes([v5119[0], 0.0, 0.0, 0.0])) + (v3817 * v2062)) * v1734;
            let v5126 = (Lanes([v5123[0], 0.0, v5123[1], v5123[2], v5123[3]])) + (v2626 * v2065);
            let v2067 = v42 * v325;
            let v5127 = v2536 * v42;
            let v5128 = Lanes([v3719[0], 0.0, 0.0, 0.0, 0.0, 0.0]);
            let v2069 = (v754 - v1072) / v2067;
            let v5130 = v5127 * v2069;
            let v5133 = ((v3404 - v5128) - (Lanes([v5130[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v2067;
            let v2070 = if v754 < v1072 { 1.0 } else { 0.0 };
            let v2082: f64;
            let v2650: Lanes<6>;
            if v2070 != 0.0 {
                let v2071 = v2069.exp();
                let v2072 = v2 + v2071;
                let v2073 = v2072.ln();
                let v5146 = v5127 * v2073;
                let v2075 = v754 - (v2067 * v2073);
                let v5150 = v3404 - ((Lanes([v5146[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v5133 * v2071) * (v2515 / v2072)) * v2067));
                v2082 = v2075;
                v2650 = v5150;
            } else {
                let v2077 = (-v2069).exp();
                let v2078 = v2 + v2077;
                let v2079 = v2078.ln();
                let v5138 = v5127 * v2079;
                let v2081 = v1072 - (v2067 * v2079);
                let v5142 = v5128 - ((Lanes([v5138[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v5133 * v2676) * v2077) * (v2515 / v2078)) * v2067));
                v2082 = v2081;
                v2650 = v5142;
            }
            let v2083 = v2082 / v325;
            let v5151 = v2536 * v2083;
            let v2084 = v2 - v2083;
            let v2086 = v2 - (v2084.powf(v1094));
            let v5160 = v3744 * v2086;
            let v2088 = v754 - v2082;
            let v5165 = v3711 * v2088;
            let v2090 = (v1095 * v2086) + (v1067 * v2088);
            let v5170 = v3708 * v2090;
            let v5174 = v2943 * v754;
            let v5175 = v3316 * v333;
            let v2093 = (v1066 * v2090) + (v333 * v754);
            let v5180 = v2942 * v2093;
            let v2095 = v2 - v2058;
            let v2097 = ((v332 * v2093) * v2095) * v9;
            let v5185 = (((Lanes([v5180[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([v5170[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([v5160[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((v2650 - (Lanes([v5151[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v325) * v2676) * (v1094 * (v2084.powf(v3750)))) * v2676) * v1095)) + ((Lanes([v5165[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3404 - v2650) * v1067))) * v1066)) + ((Lanes([v5174[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, v5175[0], v5175[1], v5175[2], v5175[3], v5175[4]])))) * v332)) * v2095) * v9;
            let v5186 = Lanes([0.0, 0.0, v3719[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let v2099 = (v759 - v1072) / v2067;
            let v5188 = v5127 * v2099;
            let v5191 = ((v3395 - v5186) - (Lanes([0.0, 0.0, v5188[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v2067;
            let v2100 = if v759 < v1072 { 1.0 } else { 0.0 };
            let v2112: f64;
            let v2651: Lanes<9>;
            if v2100 != 0.0 {
                let v2101 = v2099.exp();
                let v2102 = v2 + v2101;
                let v2103 = v2102.ln();
                let v5204 = v5127 * v2103;
                let v2105 = v759 - (v2067 * v2103);
                let v5208 = v3395 - ((Lanes([0.0, 0.0, v5204[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v5191 * v2101) * (v2515 / v2102)) * v2067));
                v2112 = v2105;
                v2651 = v5208;
            } else {
                let v2107 = (-v2099).exp();
                let v2108 = v2 + v2107;
                let v2109 = v2108.ln();
                let v5196 = v5127 * v2109;
                let v2111 = v1072 - (v2067 * v2109);
                let v5200 = v5186 - ((Lanes([0.0, 0.0, v5196[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v5191 * v2676) * v2107) * (v2515 / v2108)) * v2067));
                v2112 = v2111;
                v2651 = v5200;
            }
            let v2113 = v2112 / v325;
            let v5209 = v2536 * v2113;
            let v2114 = v2 - v2113;
            let v2116 = v2 - (v2114.powf(v1094));
            let v5218 = v3744 * v2116;
            let v2118 = v759 - v2112;
            let v5223 = v3711 * v2118;
            let v2120 = (v1095 * v2116) + (v1067 * v2118);
            let v5228 = v3708 * v2120;
            let v5232 = v2943 * v759;
            let v5233 = v3328 * v333;
            let v2123 = (v1066 * v2120) + (v333 * v759);
            let v5238 = v2942 * v2123;
            let v2126 = ((v332 * v2123) * v2095) * v8;
            let v5243 = (((Lanes([0.0, 0.0, v5238[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, v5228[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, v5218[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((v2651 - (Lanes([0.0, 0.0, v5209[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v325) * v2676) * (v1094 * (v2114.powf(v3750)))) * v2676) * v1095)) + ((Lanes([0.0, 0.0, v5223[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3395 - v2651) * v1067))) * v1066)) + ((Lanes([0.0, 0.0, v5232[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (Lanes([v5233[0], v5233[1], 0.0, v5233[2], v5233[3], v5233[4], v5233[5], v5233[6], v5233[7]])))) * v332)) * v2095) * v8;
            let v2127 = v42 * v318;
            let v5244 = v2539 * v42;
            let v2131 = v2 - (v27.powf((v2128 / v320)));
            let v2132 = v318 * v2131;
            let v5245 = v2539 * v2131;
            let v5246 = Lanes([v3279[0], 0.0, v3279[1]]);
            let v5247 = Lanes([0.0, v5245[0], 0.0]);
            let v2134 = (v732 - v2132) / v2127;
            let v5249 = v5244 * v2134;
            let v5252 = ((v5246 - v5247) - (Lanes([0.0, v5249[0], 0.0]))) / v2127;
            let v2135 = if v732 < v2132 { 1.0 } else { 0.0 };
            let v2149: f64;
            let v2652: Lanes<3>;
            if v2135 != 0.0 {
                let v2136 = v2134.exp();
                let v2137 = v2 + v2136;
                let v2138 = v2137.ln();
                let v5265 = v5244 * v2138;
                let v2140 = v732 - (v2127 * v2138);
                let v5269 = v5246 - ((Lanes([0.0, v5265[0], 0.0])) + (((v5252 * v2136) * (v2515 / v2137)) * v2127));
                v2149 = v2140;
                v2652 = v5269;
            } else {
                let v2142 = (-v2134).exp();
                let v2143 = v2 + v2142;
                let v2144 = v2143.ln();
                let v5257 = v5244 * v2144;
                let v2146 = v2132 - (v2127 * v2144);
                let v5261 = v5247 - ((Lanes([0.0, v5257[0], 0.0])) + ((((v5252 * v2676) * v2142) * (v2515 / v2143)) * v2127));
                v2149 = v2146;
                v2652 = v5261;
            }
            let v2147 = v2 - v320;
            let v2148 = v318 / v2147;
            let v2150 = v2149 / v318;
            let v5271 = v2539 * v2150;
            let v2151 = v2 - v2150;
            let v2153 = v2 - (v2151.powf(v2147));
            let v5281 = (v2539 / v2147) * v2153;
            let v2157 = (v2148 * v2153) + (v27 * (v732 - v2149));
            let v2158 = v322 * v2157;
            let v5288 = v2931 * v2157;
            let v5291 = (Lanes([0.0, v5288[0], 0.0])) + ((((Lanes([0.0, v5281[0], 0.0])) + ((((((v2652 - (Lanes([0.0, v5271[0], 0.0]))) / v318) * v2676) * (v2147 * (v2151.powf((v2147 - v2515))))) * v2676) * v2148)) + ((v5246 - v2652) * v27)) * v322);
            let v2159 = v661 * v454;
            let v2160 = v449 / v454;
            let v2162 = v2 / v2161;
            let v2163 = v2160.powf(v2162);
            let v2164 = v2159 * v2163;
            let v5304 = (((v3226 * v454) + (v3023 * v661)) * v2163) + ((((v3020 - (v3023 * v2160)) / v454) * (v2162 * (v2160.powf((v2162 - v2515))))) * v2159);
            let v2165 = v2161 * v107;
            let v5305 = v2684 * v2161;
            let v2166 = v724 / v2165;
            let v5306 = v5305 * v2166;
            let v5309 = (v3662 - (Lanes([v5306[0], 0.0, 0.0]))) / v2165;
            let v2167 = if v2166 < v763 { 1.0 } else { 0.0 };
            let v2173: f64;
            let v2653: Lanes<3>;
            if v2167 != 0.0 {
                let v2168 = v2166.exp();
                let v5311 = v5309 * v2168;
                v2173 = v2168;
                v2653 = v5311;
            } else {
                let v2169 = v763.exp();
                let v2172 = v2169 * (v2 + (v2166 - v763));
                let v5310 = v5309 * v2169;
                v2173 = v2172;
                v2653 = v5310;
            }
            let v2174 = v2164 * v2173;
            let v5312 = v5304 * v2173;
            let v5315 = (Lanes([v5312[0], 0.0, 0.0])) + (v2653 * v2164);
            let v2175 = v435 * v672;
            let v2177 = (v2175 * v107) / v369;
            let v5322 = ((((v3232 * v435) * v107) + (v2684 * v2175)) - (v2963 * v2177)) / v369;
            let v2178 = v424 * v2177;
            let v2179 = v2178 * v1793;
            let v5324 = (v5322 * v424) * v1793;
            let v2182 = (v2180 + v952) + v27;
            let v2183 = v2179 * v2182;
            let v5332 = (((Lanes([v5324[0], 0.0, 0.0, 0.0])) + (v2574 * v2178)) * v2182) + ((v2576 + (Lanes([v2569[0], v2569[1], v2569[2], 0.0]))) * v2179);
            let v2185 = if v2184 == v0 { 1.0 } else { 0.0 };
            let v2215: f64;
            let v2654: Lanes<6>;
            if v2185 != 0.0 {
                let v2186 = v677 * v424;
                let v5357 = v5109 * v1506;
                let v5361 = v5322 * v1510;
                let v2189 = (v2061 * v1506) + (v2177 * v1510);
                let v5366 = (v3235 * v424) * v2189;
                let v2191 = (v2186 * v2189) / v674;
                let v5370 = v3233 * v2191;
                let v5373 = (((Lanes([v5366[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([v5357[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4292 * v2061)) + ((Lanes([v5361[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4298 * v2177))) * v2186)) - (Lanes([v5370[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v674;
                v2215 = v2191;
                v2654 = v5373;
            } else {
                let v2195 = (v754 - v2192) / v2194;
                let v2196 = v2195 * v109;
                let v5337 = v2687 * v2195;
                let v5339 = (((v3404 - (Lanes([v2535[0], 0.0, 0.0, 0.0, 0.0, 0.0]))) / v2194) * v109) + (Lanes([v5337[0], 0.0, 0.0, 0.0, 0.0, 0.0]));
                let v2197 = if v2196 < v763 { 1.0 } else { 0.0 };
                let v2205: f64;
                let v2655: Lanes<6>;
                if v2197 != 0.0 {
                    let v2198 = v2196.exp();
                    let v5341 = v5339 * v2198;
                    v2205 = v2198;
                    v2655 = v5341;
                } else {
                    let v2199 = v763.exp();
                    let v2202 = v2199 * (v2 + (v2196 - v763));
                    let v5340 = v5339 * v2199;
                    v2205 = v2202;
                    v2655 = v5340;
                }
                let v2203 = v1511 * v683;
                let v5345 = ((v4299 * v683) + (v3238 * v1511)) * v1498;
                let v2208 = (v2 + (v435 * v2205)).sqrt();
                let v2209 = v2 + v2208;
                let v2210 = (v2203 * v1498) / v2209;
                let v5355 = (((Lanes([v5345[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2559 * v2203)) - (((v2655 * v435) * (v2515 / (v2994 * v2208))) * v2210)) / v2209;
                v2215 = v2210;
                v2654 = v5355;
            }
            let v2214 = if (if (if v1586 == v2 { 1.0 } else { 0.0 }) != 0.0 || (if v1586 == v154 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1588 != 0.0 { 1.0 } else { 0.0 };
            let v2400: f64;
            let v2409: f64;
            let v2656: Lanes<10>;
            let v2657: Lanes<6>;
            if v2214 != 0.0 {
                let v2216 = v2215 * v9;
                let v5374 = v2654 * v9;
                let v2255: f64;
                let v2658: Lanes<9>;
                if v2185 != 0.0 {
                    let v2217 = v1111 * v1595;
                    let v5398 = v3785 * v1595;
                    let v5401 = (Lanes([0.0, 0.0, v5398[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2561 * v1111);
                    let v2220 = (v2 + v2217).sqrt();
                    let v2221 = v2 + v2220;
                    let v2222 = (v2217 - v1111) / v2221;
                    let v2224 = v435 * v2223;
                    let v5410 = v2565 * v435;
                    let v2226 = (v2 + v2224).sqrt();
                    let v2227 = v2 + v2226;
                    let v2228 = v2224 / v2227;
                    let v2229 = v424 * v8;
                    let v2230 = v2229 * v677;
                    let v5418 = v5109 * v2222;
                    let v5422 = v5322 * v2228;
                    let v2233 = (v2061 * v2222) + (v2177 * v2228);
                    let v5427 = (v3235 * v2229) * v2233;
                    let v2235 = (v2230 * v2233) / v674;
                    let v5431 = v3233 * v2235;
                    let v5434 = (((Lanes([0.0, 0.0, v5427[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, v5418[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v5401 - (Lanes([0.0, 0.0, v3785[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) - ((v5401 * (v2515 / (v2994 * v2220))) * v2222)) / v2221) * v2061)) + ((Lanes([0.0, 0.0, v5422[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v5410 - ((v5410 * (v2515 / (v2994 * v2226))) * v2228)) / v2227) * v2177))) * v2230)) - (Lanes([0.0, 0.0, v5431[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v674;
                    v2255 = v2235;
                    v2658 = v5434;
                } else {
                    let v2236 = v759 - v2192;
                    let v2237 = v2236 * v109;
                    let v5378 = v2687 * v2236;
                    let v5380 = ((v3395 - (Lanes([0.0, 0.0, v2535[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v109) + (Lanes([0.0, 0.0, v5378[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
                    let v2238 = if v2237 < v763 { 1.0 } else { 0.0 };
                    let v2248: f64;
                    let v2659: Lanes<9>;
                    if v2238 != 0.0 {
                        let v2239 = v2237.exp();
                        let v5382 = v5380 * v2239;
                        v2248 = v2239;
                        v2659 = v5382;
                    } else {
                        let v2240 = v763.exp();
                        let v2243 = v2240 * (v2 + (v2237 - v763));
                        let v5381 = v5380 * v2240;
                        v2248 = v2243;
                        v2659 = v5381;
                    }
                    let v2244 = v27 * v8;
                    let v2245 = v2244 * v541;
                    let v2246 = v2245 * v683;
                    let v5387 = (((v3081 * v2244) * v683) + (v3238 * v2245)) * v1595;
                    let v2251 = (v2 + (v435 * v2248)).sqrt();
                    let v2252 = v2 + v2251;
                    let v2253 = (v2246 * v1595) / v2252;
                    let v5397 = (((Lanes([0.0, 0.0, v5387[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2561 * v2246)) - (((v2659 * v435) * (v2515 / (v2994 * v2251))) * v2253)) / v2252;
                    v2255 = v2253;
                    v2658 = v5397;
                }
                let v2256 = v2254 * v2255;
                let v5436 = v2658 * v2254;
                let v5438 = (v2619 * v2255) + (Lanes([v5436[0], v5436[1], 0.0, v5436[2], v5436[3], v5436[4], v5436[5], v5436[6], v5436[7], v5436[8]]));
                v2400 = v2256;
                v2409 = v2216;
                v2656 = v5438;
                v2657 = v5374;
            } else {
                v2400 = v0;
                v2409 = v2215;
                v2656 = v4422;
                v2657 = v2654;
            }
            let v2258 = if v2257 == v2 { 1.0 } else { 0.0 };
            let v2362: f64;
            let v2364: f64;
            let v2372: f64;
            let v2381: f64;
            let v2660: Lanes<5>;
            let v2661: Lanes<3>;
            let v2662: Lanes<5>;
            let v2663: Lanes<6>;
            if v2258 != 0.0 {
                let v2259 = -v28;
                let v5443 = v3690 * (v2259 * (v1051.powf((v2259 - v2515))));
                let v2261 = (v1051.powf(v2259)) - v154;
                let v2262 = if v1036 < v0 { 1.0 } else { 0.0 };
                let v2270: f64;
                let v2664: Lanes<3>;
                if v2262 != 0.0 {
                    let v2263 = v1036.exp();
                    let v2264 = v2 + v2263;
                    let v2265 = v2 / v2264;
                    let v5452 = (((v3668 * v2263) * v2265) * v2676) / v2264;
                    v2270 = v2265;
                    v2664 = v5452;
                } else {
                    let v2267 = (-v1036).exp();
                    let v5445 = (v3668 * v2676) * v2267;
                    let v2268 = v2 + v2267;
                    let v2269 = v2267 / v2268;
                    let v5448 = (v5445 - (v5445 * v2269)) / v2268;
                    v2270 = v2269;
                    v2664 = v5448;
                }
                let v2272 = (v2261 * v2270) + v154;
                let v5456 = v5052 * v2272;
                let v5459 = (Lanes([v5456[0], 0.0, 0.0])) + (((v5443 * v2270) + (v2664 * v2261)) * v2031);
                let v5461 = v2687 * v1113;
                let v2275 = (v1113 * v109) / v441;
                let v5464 = v2542 * v2275;
                let v2276 = v424 / v1115;
                let v2277 = v2275 * v2276;
                let v2278 = v2062 * v1734;
                let v5474 = v5110 * v1734;
                let v5479 = ((((((v3789 * v109) + (Lanes([v5461[0], 0.0, 0.0]))) - (Lanes([v5464[0], 0.0, 0.0]))) / v441) * v2276) + ((((v3792 * v2276) * v2676) / v1115) * v2275)) * v2278;
                let v2280 = v2174 / v2165;
                let v5482 = v5305 * v2280;
                let v5485 = (v5315 - (Lanes([v5482[0], 0.0, 0.0]))) / v2165;
                let v2281 = v891 * v729;
                let v2283 = ((v2031 * v2272) + (v2278 * v2277)) + v2280;
                let v2284 = v2281 * v2283;
                let v5491 = (v3275 * v891) * v2283;
                let v5492 = (((Lanes([v5459[0], v5459[1], v5459[2], 0.0, 0.0])) + ((((Lanes([v5474[0], 0.0, 0.0, 0.0, 0.0])) + (v2626 * v2062)) * v2277) + (Lanes([v5479[0], v5479[1], v5479[2], 0.0, 0.0])))) + (Lanes([v5485[0], v5485[1], v5485[2], 0.0, 0.0]))) * v2281;
                let v5495 = (Lanes([0.0, 0.0, v5491[0], v5491[1], 0.0, 0.0])) + (Lanes([v5492[0], v5492[1], 0.0, v5492[2], v5492[3], v5492[4]]));
                let v2286 = v2 - v2285;
                let v2287 = v2286 * v2174;
                let v5496 = v5315 * v2286;
                let v5497 = v5315 * v2285;
                let v2289 = v2064 + (v2285 * v2174);
                let v5499 = v5118 + (Lanes([v5497[0], v5497[1], v5497[2], 0.0, 0.0]));
                let v2292 = (v2290 * v2289) + v2066;
                let v5501 = (v5499 * v2290) + v5126;
                let v2293 = v2 - v2290;
                let v2294 = v2293 * v2289;
                let v5502 = v5499 * v2293;
                v2362 = v2294;
                v2364 = v2287;
                v2372 = v2292;
                v2381 = v2284;
                v2660 = v5502;
                v2661 = v5496;
                v2662 = v5501;
                v2663 = v5495;
            } else {
                v2362 = v2064;
                v2364 = v2174;
                v2372 = v2066;
                v2381 = v0;
                v2660 = v5118;
                v2661 = v5315;
                v2662 = v5126;
                v2663 = v5439;
            }
            let v2296 = (v1 * v873) * v21;
            let v5504 = (v3463 * v1) * v21;
            let v2298 = (v1 * v1170) * v21;
            let v5506 = (v3902 * v1) * v21;
            let v2300 = (v1 * v2005) * v21;
            let v5508 = (v4995 * v1) * v21;
            let v2302 = (v1 * v1997) * v21;
            let v5510 = (v4980 * v1) * v21;
            let v2499: f64;
            let v2500: f64;
            let v2665: Lanes<4>;
            let v2666: Lanes<4>;
            if v510 != 0.0 {
                let v2305 = (v1 * (-v1712)) * v21;
                let v5517 = ((v4572 * v2676) * v1) * v21;
                v2499 = v2305;
                v2500 = v0;
                v2665 = v5517;
                v2666 = v5514;
            } else {
                let v2308 = (v1 * (-v1712)) * v21;
                let v5513 = ((v4572 * v2676) * v1) * v21;
                v2499 = v0;
                v2500 = v2308;
                v2665 = v5514;
                v2666 = v5513;
            }
            let v2310 = (v1 * v2015) * v21;
            let v5519 = (v2617 * v1) * v21;
            let v2312 = (v1 * v2019) * v21;
            let v5521 = (v2614 * v1) * v21;
            let v2314 = (v1 * v2023) * v21;
            let v5523 = (v2618 * v1) * v21;
            let v2316 = (v1 * v1585) * v21;
            let v5525 = (v4421 * v1) * v21;
            let v2318 = (v1 * v1745) * v21;
            let v5527 = (v4630 * v1) * v21;
            let v2322 = (v1 * (v2319 * v1960)) * v21;
            let v5530 = ((v2628 * v2319) * v1) * v21;
            let v5531 = v3287 * v1;
            let v2324 = (v1 * v737) / v1927;
            let v5532 = v2540 * v2324;
            let v2325 = v2324 * v21;
            let v5537 = (((Lanes([v5531[0], 0.0, v5531[1]])) - (Lanes([0.0, v5532[0], 0.0]))) / v1927) * v21;
            let v5538 = v3291 * v1;
            let v2327 = (v1 * v740) / v1920;
            let v5539 = v2541 * v2327;
            let v2328 = v2327 * v21;
            let v5544 = (((Lanes([v5538[0], 0.0, v5538[1]])) - (Lanes([0.0, v5539[0], 0.0]))) / v1920) * v21;
            let v2332 = (ddt(13541, (v2329 * v91))) * v21;
            let v5548 = ((v2516 * v2329) * v5546) * v21;
            let v2333 = v2 - v703;
            let v2334 = if v701 > v22 { 1.0 } else { 0.0 };
            let v2356: f64;
            let v2667: Lanes<1>;
            if v2334 != 0.0 {
                let v2336 = if v2335 == v0 { 1.0 } else { 0.0 };
                let v2357: f64;
                let v2668: Lanes<1>;
                if v2336 != 0.0 {
                    let v2338 = (v91 / v705) * v21;
                    let v5561 = (v2516 / v705) * v21;
                    v2357 = v2338;
                    v2668 = v5561;
                } else {
                    let v2340 = if (v2333.abs()) < v1665 { 1.0 } else { 0.0 };
                    let v2358: f64;
                    let v2669: Lanes<1>;
                    if v2340 != 0.0 {
                        let v2342 = (v15 / v705) * v21;
                        let v2344 = v2 + (v91 / v15);
                        let v2346 = v2342 * (v2344.ln());
                        let v5559 = ((v2516 / v15) * (v2515 / v2344)) * v2342;
                        v2358 = v2346;
                        v2669 = v5559;
                    } else {
                        let v2349 = (v15 / (v2333 * v705)) * v21;
                        let v2351 = v2 + (v91 / v15);
                        let v2354 = v2349 * ((v2351.powf(v2333)) - v2);
                        let v5555 = ((v2516 / v15) * (v2333 * (v2351.powf((v2333 - v2515))))) * v2349;
                        v2358 = v2354;
                        v2669 = v5555;
                    }
                    v2357 = v2358;
                    v2668 = v2669;
                }
                v2356 = v2357;
                v2667 = v2668;
            } else {
                let v2355 = v91 / v20;
                let v5549 = v2516 / v20;
                v2356 = v2355;
                v2667 = v5549;
            }
            let v2361 = (v2359 * v2028) * v21;
            let v5563 = (v5051 * v2359) * v21;
            let v2368 = (ddt(13609, (v1 * ((v2032 + v2362) + v2364)))) * v21;
            let v5570 = (((((Lanes([v5056[0], v5056[1], v5056[2], 0.0, 0.0])) + v2660) + (Lanes([v2661[0], v2661[1], v2661[2], 0.0, 0.0]))) * v1) * v5546) * v21;
            let v2371 = (ddt(13615, (v1 * v2057))) * v21;
            let v5573 = ((v5101 * v1) * v5546) * v21;
            let v2377 = (ddt(13625, (v1 * ((v2060 + v2372) + v2183)))) * v21;
            let v5580 = (((((Lanes([v5106[0], 0.0, v5106[1], v5106[2], v5106[3]])) + v2662) + (Lanes([v5332[0], 0.0, v5332[1], v5332[2], v5332[3]]))) * v1) * v5546) * v21;
            let v2380 = (ddt(13631, (v1 * v2158))) * v21;
            let v5583 = ((v5291 * v1) * v5546) * v21;
            let v2384 = (ddt(13637, (v1 * v2381))) * v21;
            let v5586 = ((v2663 * v1) * v5546) * v21;
            let v2386 = v1 * v2385;
            let v2389 = (ddt(13645, (v2386 * v742))) * v21;
            let v5589 = ((v3295 * v2386) * v5546) * v21;
            let v2391 = v1 * v2390;
            let v2394 = (ddt(13653, (v2391 * v745))) * v21;
            let v5592 = ((v3299 * v2391) * v5546) * v21;
            let v2396 = (v1 * v1717) * v21;
            let v5594 = (v4584 * v1) * v21;
            let v2397 = v1 * v758;
            let v5596 = (v3326 * v1) * v1970;
            let v5597 = v2551 * v2397;
            let v2399 = (v2397 * v1970) * v21;
            let v5601 = ((Lanes([v5596[0], v5596[1], 0.0, v5596[2], v5596[3], v5596[4], v5596[5], v5596[6], v5596[7]])) + (Lanes([0.0, 0.0, v5597[0], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v21;
            let v2404 = (ddt(13673, (v1 * (v2126 + v2400)))) * v21;
            let v5606 = ((((Lanes([v5243[0], v5243[1], 0.0, v5243[2], v5243[3], v5243[4], v5243[5], v5243[6], v5243[7], v5243[8]])) + v2656) * v1) * v5546) * v21;
            let v2408 = (v1 * ((v1715 + v2009) + v1714)) * v21;
            let v5610 = (((v4580 + v5004) + v4576) * v1) * v21;
            let v2413 = (ddt(13692, (v1 * (v2097 + v2409)))) * v21;
            let v5614 = (((v5185 + v2657) * v1) * v5546) * v21;
            let v2501: f64;
            let v2502: f64;
            let v2670: Lanes<3>;
            if v709 != 0.0 {
                let v2414 = v1 * v751;
                let v5617 = (v3307 * v1) * v1975;
                let v5618 = v2553 * v2414;
                let v2416 = (v2414 * v1975) * v21;
                let v5622 = ((Lanes([0.0, v5617[0], v5617[1]])) + (Lanes([v5618[0], 0.0, 0.0]))) * v21;
                v2501 = v2416;
                v2502 = v0;
                v2670 = v5622;
            } else {
                v2501 = v0;
                v2502 = v2417;
                v2670 = v5615;
            }
            let v2503: f64;
            let v2504: f64;
            let v2671: Lanes<3>;
            if v712 != 0.0 {
                let v2418 = v1 * v748;
                let v5625 = (v3303 * v1) * v1980;
                let v5626 = v2555 * v2418;
                let v2420 = (v2418 * v1980) * v21;
                let v5630 = ((Lanes([0.0, v5625[0], v5625[1]])) + (Lanes([v5626[0], 0.0, 0.0]))) * v21;
                v2503 = v2420;
                v2504 = v0;
                v2671 = v5630;
            } else {
                v2503 = v0;
                v2504 = v2421;
                v2671 = v5623;
            }
            let v2423 = (v1168 + v1167) / v1164;
            let v5634 = ((v3897 + v3898) - (v3887 * v2423)) / v1164;
            let v2425 = if v2424 > v0 { 1.0 } else { 0.0 };
            let v2428: f64;
            let v2672: Lanes<5>;
            if v2425 != 0.0 {
                let v2426 = v1960 / v2423;
                let v2427 = v2426.abs();
                let v5641 = ((v2628 - (v5634 * v2426)) / v2423) * ((v2994 * (if v2426 >= v3468 { 1.0 } else { 0.0 })) - v2515);
                v2428 = v2427;
                v2672 = v5641;
            } else {
                v2428 = v0;
                v2672 = v4617;
            }
            let v2429 = if v2423 > v0 { 1.0 } else { 0.0 };
            let v2436: f64;
            let v2673: Lanes<5>;
            if v2429 != 0.0 {
                let v2431 = (v2362 + v2372) / v2423;
                let v5652 = ((v2660 + v2662) - (v5634 * v2431)) / v2423;
                v2436 = v2431;
                v2673 = v5652;
            } else {
                let v2432 = v667 * v1734;
                let v5642 = v3229 * v1734;
                let v2433 = v2432 * v1164;
                let v5648 = (((Lanes([v5642[0], 0.0, 0.0, 0.0, 0.0])) + (v2626 * v667)) * v1164) + (v3887 * v2432);
                v2436 = v2433;
                v2673 = v5648;
            }
            let v2435 = if v2434 == v2 { 1.0 } else { 0.0 };
            let v2452: f64;
            let v2674: Lanes<5>;
            if v2435 != 0.0 {
                let v2437 = v2290 * v2436;
                let v5654 = v2673 * v2290;
                v2452 = v2437;
                v2674 = v5654;
            } else {
                let v2438 = if v2434 == v27 { 1.0 } else { 0.0 };
                let v2453: f64;
                let v2675: Lanes<5>;
                if v2438 != 0.0 {
                    let v2440 = v2439 * v2436;
                    let v5653 = v2673 * v2439;
                    v2453 = v2440;
                    v2675 = v5653;
                } else {
                    v2453 = v0;
                    v2675 = v4617;
                }
                v2452 = v2453;
                v2674 = v2675;
            }
            let v2442 = if (v1989 + v2003) < v0 { 1.0 } else { 0.0 };
            if v2442 != 0.0 {
            } else {
            }
            let v2445 = if ((v1322 + v1332) + v1352) < v0 { 1.0 } else { 0.0 };
            if v2445 != 0.0 {
            } else {
            }
            let v2446 = if v1715 < v0 { 1.0 } else { 0.0 };
            if v2446 != 0.0 {
            } else {
            }
            let v2447 = if v1714 < v0 { 1.0 } else { 0.0 };
            if v2447 != 0.0 {
            } else {
            }
            let v2448 = if v8 == v0 { 1.0 } else { 0.0 };
            if v2448 != 0.0 {
            } else {
            }
            let v2449 = if v1717 < v0 { 1.0 } else { 0.0 };
            if v2449 != 0.0 {
            } else {
            }
            let v2454 = ddt(13963, v2451);
            let v2455 = v2452 * v2454;
            let v5656 = v2674 * v2454;
            let v5657 = (v2528 * v5546) * v2452;
            let v5660 = (Lanes([v5656[0], v5656[1], v5656[2], v5656[3], v5656[4], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v5657[0]]));
            let v2456 = v2428 * v2451;
            let v5661 = v2672 * v2451;
            let v5662 = v2528 * v2428;
            let v5665 = (Lanes([v5661[0], v5661[1], v5661[2], v5661[3], v5661[4], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, v5662[0]]));
            let v2505: f64;
            let v2506: f64;
            if v510 != 0.0 {
                v2505 = v2471;
                v2506 = v0;
            } else {
                v2505 = v0;
                v2506 = v2472;
            }
            let v2484: f64;
            let v2486: f64;
            let v2488: f64;
            let v2490: f64;
            let v2507: f64;
            let v2509: f64;
            let v2511: f64;
            let v2513: f64;
            if v709 != 0.0 {
                let v2485: f64;
                let v2487: f64;
                let v2508: f64;
                let v2510: f64;
                let v2512: f64;
                if v712 != 0.0 {
                    v2485 = v2476;
                    v2487 = v0;
                    v2508 = v2477;
                    v2510 = v2478;
                    v2512 = v0;
                } else {
                    v2485 = v0;
                    v2487 = v2479;
                    v2508 = v0;
                    v2510 = v0;
                    v2512 = v2480;
                }
                v2484 = v2485;
                v2486 = v2487;
                v2488 = v0;
                v2490 = v0;
                v2507 = v2508;
                v2509 = v2510;
                v2511 = v2512;
                v2513 = v0;
            } else {
                let v2489: f64;
                let v2491: f64;
                let v2514: f64;
                if v712 != 0.0 {
                    v2489 = v2481;
                    v2491 = v0;
                    v2514 = v2482;
                } else {
                    v2489 = v0;
                    v2491 = v2483;
                    v2514 = v0;
                }
                v2484 = v0;
                v2486 = v0;
                v2488 = v2489;
                v2490 = v2491;
                v2507 = v0;
                v2509 = v0;
                v2511 = v0;
                v2513 = v2514;
            }
            let v2497 = if (((((v2314 + v2328) + v2389) + v2394) + v2396) + v2404) == v0 { 1.0 } else { 0.0 };
            if v2497 != 0.0 {
            } else {
            }
            let v2498 = if v21 != v2 { 1.0 } else { 0.0 };
            if v2498 != 0.0 {
            } else {
            }
            let v5666 = v5504[0];
            let v5667 = v5504[1];
            let v5668 = v5504[2];
            let v5669 = v5504[3];
            let v5670 = v5506[0];
            let v5671 = v5506[1];
            let v5672 = v5506[2];
            let v5673 = v5506[3];
            let v5674 = v5506[4];
            let v5675 = v5508[0];
            let v5676 = v5508[1];
            let v5677 = v5508[2];
            let v5678 = v5510[0];
            let v5679 = v5510[1];
            let v5680 = v5510[2];
            let v5681 = v5510[3];
            let v5682 = v5510[4];
            let v5683 = v2665[0];
            let v5684 = v2665[1];
            let v5685 = v2665[2];
            let v5686 = v2665[3];
            let v5687 = v2666[0];
            let v5688 = v2666[1];
            let v5689 = v2666[2];
            let v5690 = v2666[3];
            let v5691 = v5519[0];
            let v5692 = v5519[1];
            let v5693 = v5519[2];
            let v5694 = v5519[3];
            let v5695 = v5519[4];
            let v5696 = v5519[5];
            let v5697 = v5519[6];
            let v5698 = v5521[0];
            let v5699 = v5521[1];
            let v5700 = v5521[2];
            let v5701 = v5521[3];
            let v5702 = v5521[4];
            let v5703 = v5523[0];
            let v5704 = v5523[1];
            let v5705 = v5523[2];
            let v5706 = v5523[3];
            let v5707 = v5523[4];
            let v5708 = v5523[5];
            let v5709 = v5523[6];
            let v5710 = v5523[7];
            let v5711 = v5523[8];
            let v5712 = v5523[9];
            let v5713 = v5525[0];
            let v5714 = v5525[1];
            let v5715 = v5525[2];
            let v5716 = v5527[0];
            let v5717 = v5527[1];
            let v5718 = v5527[2];
            let v5719 = v5527[3];
            let v5720 = v5527[4];
            let v5721 = v5527[5];
            let v5722 = v5530[0];
            let v5723 = v5530[1];
            let v5724 = v5530[2];
            let v5725 = v5530[3];
            let v5726 = v5530[4];
            let v5727 = v5537[0];
            let v5728 = v5537[1];
            let v5729 = v5537[2];
            let v5730 = v5544[0];
            let v5731 = v5544[1];
            let v5732 = v5544[2];
            let v5733 = v2667[0];
            let v5734 = v5548[0];
            let v5735 = v5563[0];
            let v5736 = v5563[1];
            let v5737 = v5563[2];
            let v5738 = v5563[3];
            let v5739 = v5563[4];
            let v5740 = v5563[5];
            let v5741 = v5563[6];
            let v5742 = v5563[7];
            let v5743 = v5563[8];
            let v5744 = v5563[9];
            let v5745 = v5563[10];
            let v5746 = v5563[11];
            let v5747 = v5570[0];
            let v5748 = v5570[1];
            let v5749 = v5570[2];
            let v5750 = v5570[3];
            let v5751 = v5570[4];
            let v5752 = v5573[0];
            let v5753 = v5573[1];
            let v5754 = v5573[2];
            let v5755 = v5580[0];
            let v5756 = v5580[1];
            let v5757 = v5580[2];
            let v5758 = v5580[3];
            let v5759 = v5580[4];
            let v5760 = v5583[0];
            let v5761 = v5583[1];
            let v5762 = v5583[2];
            let v5763 = v5586[0];
            let v5764 = v5586[1];
            let v5765 = v5586[2];
            let v5766 = v5586[3];
            let v5767 = v5586[4];
            let v5768 = v5586[5];
            let v5769 = v5589[0];
            let v5770 = v5589[1];
            let v5771 = v5592[0];
            let v5772 = v5592[1];
            let v5773 = v5594[0];
            let v5774 = v5594[1];
            let v5775 = v5594[2];
            let v5776 = v5594[3];
            let v5777 = v5594[4];
            let v5778 = v5594[5];
            let v5779 = v5594[6];
            let v5780 = v5594[7];
            let v5781 = v5594[8];
            let v5782 = v5594[9];
            let v5783 = v5601[0];
            let v5784 = v5601[1];
            let v5785 = v5601[2];
            let v5786 = v5601[3];
            let v5787 = v5601[4];
            let v5788 = v5601[5];
            let v5789 = v5601[6];
            let v5790 = v5601[7];
            let v5791 = v5601[8];
            let v5792 = v5606[0];
            let v5793 = v5606[1];
            let v5794 = v5606[2];
            let v5795 = v5606[3];
            let v5796 = v5606[4];
            let v5797 = v5606[5];
            let v5798 = v5606[6];
            let v5799 = v5606[7];
            let v5800 = v5606[8];
            let v5801 = v5606[9];
            let v5802 = v5610[0];
            let v5803 = v5610[1];
            let v5804 = v5610[2];
            let v5805 = v5610[3];
            let v5806 = v5610[4];
            let v5807 = v5610[5];
            let v5808 = v5614[0];
            let v5809 = v5614[1];
            let v5810 = v5614[2];
            let v5811 = v5614[3];
            let v5812 = v5614[4];
            let v5813 = v5614[5];
            let v5814 = v2670[0];
            let v5815 = v2670[1];
            let v5816 = v2670[2];
            let v5817 = v2671[0];
            let v5818 = v2671[1];
            let v5819 = v2671[2];
            let v5820 = v2528[0];
            let v5821 = v5660[0];
            let v5822 = v5660[1];
            let v5823 = v5660[2];
            let v5824 = v5660[3];
            let v5825 = v5660[4];
            let v5826 = v5660[5];
            let v5827 = v5665[0];
            let v5828 = v5665[1];
            let v5829 = v5665[2];
            let v5830 = v5665[3];
            let v5831 = v5665[4];
            let v5832 = v5665[5];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (v2296),
            [4, 7, 8, 9],
            [v5666, v5667, v5668, v5669],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(5),
            multiplicity * (v2298),
            [4, 5, 7, 8, 9],
            [v5670, v5671, v5672, v5673, v5674],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(5),
            multiplicity * (v2300),
            [4, 5, 6],
            [v5675, v5676, v5677],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2302),
            [4, 5, 7, 8, 9],
            [v5678, v5679, v5680, v5681, v5682],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (v2499),
            [4, 6, 7, 8],
            [v5683, v5684, v5685, v5686],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (v2500),
            [4, 6, 7, 8],
            [v5687, v5688, v5689, v5690],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(3),
            multiplicity * (v2310),
            [3, 4, 6, 7, 8, 9, 11],
            [v5691, v5692, v5693, v5694, v5695, v5696, v5697],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (v2312),
            [3, 4, 7, 8, 9],
            [v5698, v5699, v5700, v5701, v5702],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(3),
            multiplicity * (v2314),
            [0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            [v5703, v5704, v5705, v5706, v5707, v5708, v5709, v5710, v5711, v5712],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(8),
            multiplicity * (v2316),
            [3, 4, 8],
            [v5713, v5714, v5715],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2318),
            [4, 5, 6, 7, 8, 9],
            [v5716, v5717, v5718, v5719, v5720, v5721],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (v2322),
            [4, 5, 7, 8, 9],
            [v5722, v5723, v5724, v5725, v5726],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(5),
            multiplicity * (v2325),
            [2, 4, 5],
            [v5727, v5728, v5729],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(6),
            multiplicity * (v2328),
            [1, 4, 6],
            [v5730, v5731, v5732],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v2356),
            [4],
            [v5733],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v2332),
            [4],
            [v5734],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<12, 0>(
            Some(4),
            None,
            multiplicity * (v2361),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [v5735, v5736, v5737, v5738, v5739, v5740, v5741, v5742, v5743, v5744, v5745, v5746],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2368),
            [4, 5, 7, 8, 9],
            [v5747, v5748, v5749, v5750, v5751],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(5),
            multiplicity * (v2371),
            [4, 5, 6],
            [v5752, v5753, v5754],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (v2377),
            [4, 5, 7, 8, 9],
            [v5755, v5756, v5757, v5758, v5759],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(8),
            multiplicity * (v2380),
            [3, 4, 8],
            [v5760, v5761, v5762],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2384),
            [4, 5, 6, 7, 8, 9],
            [v5763, v5764, v5765, v5766, v5767, v5768],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v2389),
            [1, 2],
            [v5769, v5770],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (v2394),
            [0, 1],
            [v5771, v5772],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(10),
            multiplicity * (v2396),
            [0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            [v5773, v5774, v5775, v5776, v5777, v5778, v5779, v5780, v5781, v5782],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(10),
            multiplicity * (v2399),
            [0, 1, 4, 6, 7, 8, 9, 10, 11],
            [v5783, v5784, v5785, v5786, v5787, v5788, v5789, v5790, v5791],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(10),
            multiplicity * (v2404),
            [0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            [v5792, v5793, v5794, v5795, v5796, v5797, v5798, v5799, v5800, v5801],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(11),
            multiplicity * (v2408),
            [4, 6, 7, 8, 9, 11],
            [v5802, v5803, v5804, v5805, v5806, v5807],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(11),
            multiplicity * (v2413),
            [4, 6, 7, 8, 9, 11],
            [v5808, v5809, v5810, v5811, v5812, v5813],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(10),
            Some(11),
            multiplicity * (v2501),
            [4, 10, 11],
            [v5814, v5815, v5816],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(10), Some(11), 0, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            0,
            v2502,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(11),
            Some(8),
            multiplicity * (v2503),
            [4, 8, 11],
            [v5817, v5818, v5819],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_potential_branch_local(Some(11), Some(8), 1, multiplicity);
        stamper.stamp_potential_sparse_local::<0, 0>(
            1,
            v2504,
            [],
            [],
            [],
            [],
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(12),
            None,
            multiplicity * (v2450),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(12),
            None,
            multiplicity * (v2451),
            [12],
            [v5820],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2455),
            [4, 5, 7, 8, 9, 12],
            [v5821, v5822, v5823, v5824, v5825, v5826],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (v2456),
            [4, 5, 7, 8, 9, 12],
            [v5827, v5828, v5829, v5830, v5831, v5832],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            Some(5),
            multiplicity * (v2451),
            [12],
            [v5820],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (v2457),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2458),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(2),
            Some(5),
            multiplicity * (v2459),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(6),
            multiplicity * (v2460),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2461),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2462),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(5),
            multiplicity * (v2463),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(5),
            multiplicity * (v2464),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(11),
            multiplicity * (v2465),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(11),
            multiplicity * (v2466),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(11),
            multiplicity * (v2467),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(11),
            multiplicity * (v2468),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(10),
            multiplicity * (v2469),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(10),
            multiplicity * (v2470),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(8),
            Some(7),
            multiplicity * (v2505),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(9),
            Some(7),
            multiplicity * (v2506),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(7),
            Some(3),
            multiplicity * (v2473),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(6),
            Some(3),
            multiplicity * (v2474),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(1),
            Some(3),
            multiplicity * (v2475),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(10),
            multiplicity * (v2484),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(11),
            multiplicity * (v2507),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(8),
            multiplicity * (v2509),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(10),
            multiplicity * (v2486),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(10),
            Some(8),
            multiplicity * (v2511),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(11),
            multiplicity * (v2488),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(11),
            Some(8),
            multiplicity * (v2513),
            [],
            [],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<0, 0>(
            Some(0),
            Some(8),
            multiplicity * (v2490),
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

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
            let v2526 = 1e0f64;
            let v2527 = 1e0f64;
            let v2528 = 1e0f64;
            let v2529 = 1e0f64;
            let v2530 = 1e0f64;
            let v2531 = 1e0f64;
            let v2532 = 1e0f64;
            let v2533 = 1e0f64;
            let v2534 = 1e0f64;
            let v2535 = 1e0f64;
            let v2536 = 1e0f64;
            let v2537 = 1e0f64;
            let v2538 = 1e0f64;
            let v2539 = 1e0f64;
            let v2687 = -1e0f64;
            let v2958 = 0e0f64;
            let v3005 = 2e0f64;
            let v3119 = -1.5e0f64;
            let v3160 = -1.5e0f64;
            let v3454 = Lanes([0e0f64; 3]);
            let v3479 = 0e0f64;
            let v3500 = Lanes([0e0f64; 4]);
            let v4098 = Lanes([0e0f64; 3]);
            let v4433 = Lanes([0e0f64; 10]);
            let v4546 = Lanes([0e0f64; 3]);
            let v4628 = Lanes([0e0f64; 5]);
            let v5450 = Lanes([0e0f64; 6]);
            let v5525 = Lanes([0e0f64; 4]);
            let v5557 = ddt_scale();
            let v5636 = Lanes([0e0f64; 3]);
            let v5644 = Lanes([0e0f64; 3]);
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
            let v2540: f64;
            if v92 != 0.0 {
                let v93 = v2 - v91;
                let v95 = -(v93.ln());
                let v2691 = ((v2527 * v2687) * (v2526 / v93)) * v2687;
                v96 = v95;
                v2540 = v2691;
            } else {
                v96 = v91;
                v2540 = v2527;
            }
            let v98 = if v96 < v97 { 1.0 } else { 0.0 };
            let v103: f64;
            let v2541: f64;
            if v98 != 0.0 {
                v103 = v96;
                v2541 = v2540;
            } else {
                let v100 = v2 + (v96 - v97);
                let v2693 = v2540 * (v2526 / v100);
                let v102 = v97 + (v100.ln());
                v103 = v102;
                v2541 = v2693;
            }
            let v104 = v15 + v103;
            let v105 = v104 / v12;
            let v2694 = v2541 / v12;
            let v107 = v106 * v104;
            let v2695 = v2541 * v106;
            let v109 = v2 / v107;
            let v2698 = ((v2695 * v109) * v2687) / v107;
            let v111 = v109 - (v2 / (v106 * v12));
            let v112 = v104 - v12;
            let v113 = v105.ln();
            let v2700 = v2694 * (v2526 / v105);
            let v115 = v33 * v104;
            let v117 = v104 + v36;
            let v118 = (v115 * v104) / v117;
            let v119 = v114 - v118;
            let v2708 = (((((v2541 * v33) * v104) + (v2541 * v115)) - (v2541 * v118)) / v117) * v2687;
            let v121 = (v119 - v40) / v42;
            let v2709 = v2708 / v42;
            let v122 = if v119 < v40 { 1.0 } else { 0.0 };
            let v568: f64;
            let v2542: f64;
            if v122 != 0.0 {
                let v123 = v121.exp();
                let v124 = v2 + v123;
                let v2719 = ((v2709 * v123) * (v2526 / v124)) * v42;
                let v127 = v40 + (v42 * (v124.ln()));
                v568 = v127;
                v2542 = v2719;
            } else {
                let v129 = (-v121).exp();
                let v130 = v2 + v129;
                let v133 = v119 + (v42 * (v130.ln()));
                let v2715 = v2708 + ((((v2709 * v2687) * v129) * (v2526 / v130)) * v42);
                v568 = v133;
                v2542 = v2715;
            }
            let v135 = v65 * v104;
            let v137 = v104 + v68;
            let v138 = (v135 * v104) / v137;
            let v139 = v134 - v138;
            let v2727 = (((((v2541 * v65) * v104) + (v2541 * v135)) - (v2541 * v138)) / v137) * v2687;
            let v141 = (v139 - v40) / v42;
            let v2728 = v2727 / v42;
            let v142 = if v139 < v40 { 1.0 } else { 0.0 };
            let v592: f64;
            let v2543: f64;
            if v142 != 0.0 {
                let v143 = v141.exp();
                let v144 = v2 + v143;
                let v2738 = ((v2728 * v143) * (v2526 / v144)) * v42;
                let v147 = v40 + (v42 * (v144.ln()));
                v592 = v147;
                v2543 = v2738;
            } else {
                let v149 = (-v141).exp();
                let v150 = v2 + v149;
                let v153 = v139 + (v42 * (v150.ln()));
                let v2734 = v2727 + ((((v2728 * v2687) * v149) * (v2526 / v150)) * v42);
                v592 = v153;
                v2543 = v2734;
            }
            let v156 = v155 * v107;
            let v160 = v2 - v105;
            let v2745 = v2694 * v2687;
            let v163 = ((v156 * v113) + (v57 * v105)) + (v160 * v161);
            let v2747 = ((((v2695 * v155) * v113) + (v2700 * v156)) + (v2694 * v57)) + (v2745 * v161);
            let v165 = (v40 - v163) / v107;
            let v2751 = ((v2747 * v2687) - (v2695 * v165)) / v107;
            let v166 = if v40 < v163 { 1.0 } else { 0.0 };
            let v307: f64;
            let v2544: f64;
            if v166 != 0.0 {
                let v167 = v165.exp();
                let v168 = v2 + v167;
                let v169 = v168.ln();
                let v171 = v163 + (v107 * v169);
                let v2765 = v2747 + ((v2695 * v169) + (((v2751 * v167) * (v2526 / v168)) * v107));
                v307 = v171;
                v2544 = v2765;
            } else {
                let v173 = (-v165).exp();
                let v174 = v2 + v173;
                let v175 = v174.ln();
                let v2758 = (v2695 * v175) + ((((v2751 * v2687) * v173) * (v2526 / v174)) * v107);
                let v177 = v40 + (v107 * v175);
                v307 = v177;
                v2544 = v2758;
            }
            let v179 = v178 * v107;
            let v185 = v160 * v184;
            let v2772 = v2745 * v184;
            let v186 = ((v179 * v113) + (v181 * v105)) + v185;
            let v2773 = ((((v2695 * v178) * v113) + (v2700 * v179)) + (v2694 * v181)) + v2772;
            let v188 = (v40 - v186) / v107;
            let v2777 = ((v2773 * v2687) - (v2695 * v188)) / v107;
            let v189 = if v40 < v186 { 1.0 } else { 0.0 };
            let v820: f64;
            let v2545: f64;
            if v189 != 0.0 {
                let v190 = v188.exp();
                let v191 = v2 + v190;
                let v192 = v191.ln();
                let v194 = v186 + (v107 * v192);
                let v2791 = v2773 + ((v2695 * v192) + (((v2777 * v190) * (v2526 / v191)) * v107));
                v820 = v194;
                v2545 = v2791;
            } else {
                let v196 = (-v188).exp();
                let v197 = v2 + v196;
                let v198 = v197.ln();
                let v2784 = (v2695 * v198) + ((((v2777 * v2687) * v196) * (v2526 / v197)) * v107);
                let v200 = v40 + (v107 * v198);
                v820 = v200;
                v2545 = v2784;
            }
            let v202 = v201 * v107;
            let v207 = ((v202 * v113) + (v204 * v105)) + v185;
            let v2798 = ((((v2695 * v201) * v113) + (v2700 * v202)) + (v2694 * v204)) + v2772;
            let v209 = (v40 - v207) / v107;
            let v2802 = ((v2798 * v2687) - (v2695 * v209)) / v107;
            let v210 = if v40 < v207 { 1.0 } else { 0.0 };
            let v2192: f64;
            let v2546: f64;
            if v210 != 0.0 {
                let v211 = v209.exp();
                let v212 = v2 + v211;
                let v213 = v212.ln();
                let v215 = v207 + (v107 * v213);
                let v2816 = v2798 + ((v2695 * v213) + (((v2802 * v211) * (v2526 / v212)) * v107));
                v2192 = v215;
                v2546 = v2816;
            } else {
                let v217 = (-v209).exp();
                let v218 = v2 + v217;
                let v219 = v218.ln();
                let v2809 = (v2695 * v219) + ((((v2802 * v2687) * v217) * (v2526 / v218)) * v107);
                let v221 = v40 + (v107 * v219);
                v2192 = v221;
                v2546 = v2809;
            }
            let v223 = v222 * v107;
            let v225 = v59 * v105;
            let v2821 = v2694 * v59;
            let v227 = ((v223 * v113) + v225) + v185;
            let v2823 = ((((v2695 * v222) * v113) + (v2700 * v223)) + v2821) + v2772;
            let v229 = (v40 - v227) / v107;
            let v2827 = ((v2823 * v2687) - (v2695 * v229)) / v107;
            let v230 = if v40 < v227 { 1.0 } else { 0.0 };
            let v325: f64;
            let v2547: f64;
            if v230 != 0.0 {
                let v231 = v229.exp();
                let v232 = v2 + v231;
                let v233 = v232.ln();
                let v235 = v227 + (v107 * v233);
                let v2841 = v2823 + ((v2695 * v233) + (((v2827 * v231) * (v2526 / v232)) * v107));
                v325 = v235;
                v2547 = v2841;
            } else {
                let v237 = (-v229).exp();
                let v238 = v2 + v237;
                let v239 = v238.ln();
                let v2834 = (v2695 * v239) + ((((v2827 * v2687) * v237) * (v2526 / v238)) * v107);
                let v241 = v40 + (v107 * v239);
                v325 = v241;
                v2547 = v2834;
            }
            let v243 = v242 * v107;
            let v246 = ((v243 * v113) + v225) + v185;
            let v2847 = ((((v2695 * v242) * v113) + (v2700 * v243)) + v2821) + v2772;
            let v248 = (v40 - v246) / v107;
            let v2851 = ((v2847 * v2687) - (v2695 * v248)) / v107;
            let v249 = if v40 < v246 { 1.0 } else { 0.0 };
            let v309: f64;
            let v2548: f64;
            if v249 != 0.0 {
                let v250 = v248.exp();
                let v251 = v2 + v250;
                let v252 = v251.ln();
                let v254 = v246 + (v107 * v252);
                let v2865 = v2847 + ((v2695 * v252) + (((v2851 * v250) * (v2526 / v251)) * v107));
                v309 = v254;
                v2548 = v2865;
            } else {
                let v256 = (-v248).exp();
                let v257 = v2 + v256;
                let v258 = v257.ln();
                let v2858 = (v2695 * v258) + ((((v2851 * v2687) * v256) * (v2526 / v257)) * v107);
                let v260 = v40 + (v107 * v258);
                v309 = v260;
                v2548 = v2858;
            }
            let v262 = v261 * v107;
            let v269 = ((v262 * v113) + (v264 * v105)) + (v160 * v267);
            let v2873 = ((((v2695 * v261) * v113) + (v2700 * v262)) + (v2694 * v264)) + (v2745 * v267);
            let v271 = (v40 - v269) / v107;
            let v2877 = ((v2873 * v2687) - (v2695 * v271)) / v107;
            let v272 = if v40 < v269 { 1.0 } else { 0.0 };
            let v1224: f64;
            let v2549: f64;
            if v272 != 0.0 {
                let v273 = v271.exp();
                let v274 = v2 + v273;
                let v275 = v274.ln();
                let v277 = v269 + (v107 * v275);
                let v2891 = v2873 + ((v2695 * v275) + (((v2877 * v273) * (v2526 / v274)) * v107));
                v1224 = v277;
                v2549 = v2891;
            } else {
                let v279 = (-v271).exp();
                let v280 = v2 + v279;
                let v281 = v280.ln();
                let v2884 = (v2695 * v281) + ((((v2877 * v2687) * v279) * (v2526 / v280)) * v107);
                let v283 = v40 + (v107 * v281);
                v1224 = v283;
                v2549 = v2884;
            }
            let v285 = v284 * v107;
            let v292 = ((v285 * v113) + (v287 * v105)) + (v160 * v290);
            let v2899 = ((((v2695 * v284) * v113) + (v2700 * v285)) + (v2694 * v287)) + (v2745 * v290);
            let v294 = (v40 - v292) / v107;
            let v2903 = ((v2899 * v2687) - (v2695 * v294)) / v107;
            let v295 = if v40 < v292 { 1.0 } else { 0.0 };
            let v318: f64;
            let v2550: f64;
            if v295 != 0.0 {
                let v296 = v294.exp();
                let v297 = v2 + v296;
                let v298 = v297.ln();
                let v300 = v292 + (v107 * v298);
                let v2917 = v2899 + ((v2695 * v298) + (((v2903 * v296) * (v2526 / v297)) * v107));
                v318 = v300;
                v2550 = v2917;
            } else {
                let v302 = (-v294).exp();
                let v303 = v2 + v302;
                let v304 = v303.ln();
                let v2910 = (v2695 * v304) + ((((v2903 * v2687) * v302) * (v2526 / v303)) * v107);
                let v306 = v40 + (v107 * v304);
                v318 = v306;
                v2550 = v2910;
            }
            let v308 = v2 / v307;
            let v2920 = ((v2544 * v308) * v2687) / v307;
            let v310 = v2 / v309;
            let v2923 = ((v2548 * v310) * v2687) / v309;
            let v311 = v57 * v308;
            let v312 = v311.powf(v28);
            let v2928 = (v2920 * v57) * (v28 * (v311.powf((v28 - v2526))));
            let v313 = v59 * v310;
            let v314 = v313.powf(v60);
            let v2930 = v60 - v2526;
            let v2933 = (v2923 * v59) * (v60 * (v313.powf(v2930)));
            let v316 = v315 * v312;
            let v2934 = v2928 * v315;
            let v319 = v287 / v318;
            let v322 = v317 * (v319.powf(v320));
            let v2942 = ((((v2550 * v319) * v2687) / v318) * (v320 * (v319.powf((v320 - v2526))))) * v317;
            let v324 = v2 - v323;
            let v326 = v59 / v325;
            let v2949 = ((((v2547 * v326) * v2687) / v325) * (v60 * (v326.powf(v2930)))) * v324;
            let v329 = (v324 * (v326.powf(v60))) + v323;
            let v330 = v2 / v329;
            let v2952 = ((v2949 * v330) * v2687) / v329;
            let v332 = v331 * v329;
            let v2953 = v2949 * v331;
            let v333 = v323 * v330;
            let v2954 = v2952 * v323;
            let v337 = (v113 * v335).exp();
            let v338 = v334 * v337;
            let v2957 = ((v2700 * v335) * v337) * v334;
            let v339 = if v338 < v22 { 1.0 } else { 0.0 };
            let v1927: f64;
            let v2551: f64;
            if v339 != 0.0 {
                v1927 = v22;
                v2551 = v2958;
            } else {
                v1927 = v338;
                v2551 = v2957;
            }
            let v343 = v341 - v342;
            let v345 = (v113 * v343).exp();
            let v346 = v340 * v345;
            let v2961 = ((v2700 * v343) * v345) * v340;
            let v350 = (v113 * v348).exp();
            let v351 = v347 * v350;
            let v2964 = ((v2700 * v348) * v350) * v347;
            let v352 = if v351 < v22 { 1.0 } else { 0.0 };
            let v1920: f64;
            let v2552: f64;
            if v352 != 0.0 {
                v1920 = v22;
                v2552 = v2958;
            } else {
                v1920 = v351;
                v2552 = v2964;
            }
            let v356 = (v113 * v354).exp();
            let v357 = v353 * v356;
            let v2967 = ((v2700 * v354) * v356) * v353;
            let v361 = (v113 * v359).exp();
            let v2969 = (v2700 * v359) * v361;
            let v362 = v358 * v361;
            let v2970 = v2969 * v358;
            let v364 = v363 * v361;
            let v2971 = v2969 * v363;
            let v368 = (v113 * v366).exp();
            let v369 = v365 * v368;
            let v2974 = ((v2700 * v366) * v368) * v365;
            let v371 = if v370 != v0 { 1.0 } else { 0.0 };
            let v441: f64;
            let v2553: f64;
            if v371 != 0.0 {
                let v375 = v372 * (v2 + (v112 * v370));
                let v2976 = (v2541 * v370) * v372;
                let v377 = (v375 - v2) / v26;
                let v2977 = v2976 / v26;
                let v378 = if v375 < v2 { 1.0 } else { 0.0 };
                let v390: f64;
                let v2554: f64;
                if v378 != 0.0 {
                    let v379 = v377.exp();
                    let v380 = v2 + v379;
                    let v2987 = ((v2977 * v379) * (v2526 / v380)) * v26;
                    let v383 = v2 + (v26 * (v380.ln()));
                    v390 = v383;
                    v2554 = v2987;
                } else {
                    let v385 = (-v377).exp();
                    let v386 = v2 + v385;
                    let v389 = v375 + (v26 * (v386.ln()));
                    let v2983 = v2976 + ((((v2977 * v2687) * v385) * (v2526 / v386)) * v26);
                    v390 = v389;
                    v2554 = v2983;
                }
                let v392 = v390 - v391;
                v441 = v392;
                v2553 = v2554;
            } else {
                v441 = v372;
                v2553 = v2958;
            }
            let v394 = if v393 != v0 { 1.0 } else { 0.0 };
            let v1119: f64;
            let v2555: f64;
            if v394 != 0.0 {
                let v398 = v395 * (v2 + (v112 * v393));
                let v2989 = (v2541 * v393) * v395;
                let v400 = (v398 - v2) / v26;
                let v2990 = v2989 / v26;
                let v401 = if v398 < v2 { 1.0 } else { 0.0 };
                let v413: f64;
                let v2556: f64;
                if v401 != 0.0 {
                    let v402 = v400.exp();
                    let v403 = v2 + v402;
                    let v3000 = ((v2990 * v402) * (v2526 / v403)) * v26;
                    let v406 = v2 + (v26 * (v403.ln()));
                    v413 = v406;
                    v2556 = v3000;
                } else {
                    let v408 = (-v400).exp();
                    let v409 = v2 + v408;
                    let v412 = v398 + (v26 * (v409.ln()));
                    let v2996 = v2989 + ((((v2990 * v2687) * v408) * (v2526 / v409)) * v26);
                    v413 = v412;
                    v2556 = v2996;
                }
                let v415 = v413 - v414;
                v1119 = v415;
                v2555 = v2556;
            } else {
                v1119 = v395;
                v2555 = v2958;
            }
            let v420 = v416 * (v2 + (v417 * v112));
            let v3002 = (v2541 * v417) * v416;
            let v422 = v420 * v420;
            let v3003 = v3002 * v420;
            let v3004 = v3003 + v3003;
            let v423 = if v420 < v0 { 1.0 } else { 0.0 };
            let v1763: f64;
            let v2557: f64;
            if v423 != 0.0 {
                let v427 = (v422 + v421).sqrt();
                let v428 = v427 - v420;
                let v429 = v425 / v428;
                let v3017 = ((((v3004 * (v2526 / (v3005 * v427))) - v3002) * v429) * v2687) / v428;
                v1763 = v429;
                v2557 = v3017;
            } else {
                let v431 = (v422 + v421).sqrt();
                let v433 = v424 * (v431 + v420);
                let v3010 = ((v3004 * (v2526 / (v3005 * v431))) + v3002) * v424;
                v1763 = v433;
                v2557 = v3010;
            }
            let v439 = ((v435 - v341) - v342) + v438;
            let v442 = (v113 * v439) / v441;
            let v443 = v442.exp();
            let v444 = v434 * v443;
            let v445 = -v161;
            let v447 = (v445 * v111) / v441;
            let v448 = v447.exp();
            let v449 = v444 * v448;
            let v3031 = ((((((v2700 * v439) - (v2553 * v442)) / v441) * v443) * v434) * v448) + (((((v2698 * v445) - (v2553 * v447)) / v441) * v448) * v444);
            let v451 = v2 - v341;
            let v453 = (v113 * v451).exp();
            let v454 = v450 * v453;
            let v3034 = ((v2700 * v451) * v453) * v450;
            let v457 = v2 - v456;
            let v459 = (v113 * v457).exp();
            let v460 = v455 * v459;
            let v3037 = ((v2700 * v457) * v459) * v455;
            let v465 = v462 - (v27 * v463);
            let v467 = (v113 * v465).exp();
            let v468 = v461 * v467;
            let v470 = -v469;
            let v471 = v470 * v111;
            let v3041 = v2698 * v470;
            let v473 = (v471 / v463).exp();
            let v474 = v468 * v473;
            let v3046 = ((((v2700 * v465) * v467) * v461) * v473) + (((v3041 / v463) * v473) * v468);
            let v478 = v462 - (v27 * v476);
            let v480 = (v113 * v478).exp();
            let v481 = v475 * v480;
            let v482 = -v184;
            let v485 = ((v482 * v111) / v476).exp();
            let v486 = v481 * v485;
            let v3055 = ((((v2700 * v478) * v480) * v475) * v485) + ((((v2698 * v482) / v476) * v485) * v481);
            let v489 = (v435 - v335) + v438;
            let v490 = v113 * v489;
            let v3056 = v2700 * v489;
            let v493 = (v490 / v491).exp();
            let v494 = v487 * v493;
            let v496 = -v495;
            let v497 = v496 * v111;
            let v3060 = v2698 * v496;
            let v499 = (v497 / v491).exp();
            let v500 = v494 * v499;
            let v3065 = ((((v3056 / v491) * v493) * v487) * v499) + (((v3060 / v491) * v499) * v494);
            let v504 = (v490 / v502).exp();
            let v505 = v501 * v504;
            let v507 = (v497 / v502).exp();
            let v508 = v505 * v507;
            let v3073 = ((((v3056 / v502) * v504) * v501) * v507) + (((v3060 / v502) * v507) * v505);
            let v510 = if v509 == v2 { 1.0 } else { 0.0 };
            let v1246: f64;
            let v1259: f64;
            let v1301: f64;
            let v2558: f64;
            let v2559: f64;
            let v2560: f64;
            if v510 != 0.0 {
                let v513 = -v512;
                let v516 = ((v513 * v111) / v491).exp();
                let v517 = v511 * v516;
                let v3077 = (((v2698 * v513) / v491) * v516) * v511;
                let v520 = -v519;
                let v522 = (v520 * v111).exp();
                let v523 = v518 * v522;
                let v3080 = ((v2698 * v520) * v522) * v518;
                let v526 = -v525;
                let v529 = ((v526 * v111) / v502).exp();
                let v530 = v524 * v529;
                let v3084 = (((v2698 * v526) / v502) * v529) * v524;
                v1246 = v517;
                v1259 = v523;
                v1301 = v530;
                v2558 = v3077;
                v2559 = v3080;
                v2560 = v3084;
            } else {
                v1246 = v0;
                v1259 = v0;
                v1301 = v0;
                v2558 = v2958;
                v2559 = v2958;
                v2560 = v2958;
            }
            let v533 = (v435 - v456) + v438;
            let v535 = (v113 * v533).exp();
            let v536 = v531 * v535;
            let v538 = -v537;
            let v540 = (v538 * v111).exp();
            let v541 = v536 * v540;
            let v3092 = ((((v2700 * v533) * v535) * v531) * v540) + (((v2698 * v538) * v540) * v536);
            let v545 = v462 - (v27 * v543);
            let v547 = (v113 * v545).exp();
            let v548 = v542 * v547;
            let v550 = (v471 / v543).exp();
            let v551 = v548 * v550;
            let v3100 = ((((v2700 * v545) * v547) * v542) * v550) + (((v3041 / v543) * v550) * v548);
            let v554 = v435 / v553;
            let v556 = (v113 * v554).exp();
            let v557 = v552 * v556;
            let v559 = (v471 / v553).exp();
            let v560 = v557 * v559;
            let v3108 = ((((v2700 * v554) * v556) * v552) * v559) + (((v3041 / v553) * v559) * v557);
            let v562 = v105.sqrt();
            let v563 = v561 * v562;
            let v566 = (v564 * v112).exp();
            let v567 = v563 * v566;
            let v3117 = (((v2694 * (v2526 / (v3005 * v562))) * v561) * v566) + (((v2541 * v564) * v566) * v563);
            let v569 = v568 * v56;
            let v571 = v569.powf(v570);
            let v3122 = (v2542 * v56) * (v570 * (v569.powf(v3119)));
            let v572 = v2 / v312;
            let v3125 = ((v2928 * v572) * v2687) / v312;
            let v574 = v573 * v568;
            let v575 = v574 * v568;
            let v576 = v575 * v571;
            let v578 = (v576 * v572) * v57;
            let v581 = ((v578 * v308) * v56) * v56;
            let v3141 = (((((((((((v2542 * v573) * v568) + (v2542 * v574)) * v571) + (v3122 * v575)) * v572) + (v3125 * v576)) * v57) * v308) + (v2920 * v578)) * v56) * v56;
            let v583 = v582 * v571;
            let v584 = v583 * v307;
            let v587 = ((v584 * v307) * v58) * v58;
            let v588 = v587 * v312;
            let v590 = (v573 - v581).exp();
            let v591 = v588 * v590;
            let v3158 = ((((((((((v3122 * v582) * v307) + (v2544 * v583)) * v307) + (v2544 * v584)) * v58) * v58) * v312) + (v2928 * v587)) * v590) + (((v3141 * v2687) * v590) * v588);
            let v593 = v592 * v86;
            let v595 = v593.powf(v594);
            let v3163 = (v2543 * v86) * (v594 * (v593.powf(v3160)));
            let v596 = v2 / v314;
            let v598 = v597 * v592;
            let v599 = v598 * v592;
            let v600 = v599 * v595;
            let v602 = (v600 * v596) * v59;
            let v605 = ((v602 * v310) * v86) * v86;
            let v3182 = (((((((((((v2543 * v597) * v592) + (v2543 * v598)) * v595) + (v3163 * v599)) * v596) + ((((v2933 * v596) * v2687) / v314) * v600)) * v59) * v310) + (v2923 * v602)) * v86) * v86;
            let v607 = v606 * v595;
            let v608 = v607 * v309;
            let v611 = ((v608 * v309) * v87) * v87;
            let v612 = v611 * v314;
            let v614 = (v597 - v605).exp();
            let v615 = v612 * v614;
            let v3199 = ((((((((((v3163 * v606) * v309) + (v2548 * v607)) * v309) + (v2548 * v608)) * v87) * v87) * v314) + (v2933 * v611)) * v614) + (((v3182 * v2687) * v614) * v612);
            let v617 = (v113 * v342).exp();
            let v3201 = (v2700 * v342) * v617;
            let v619 = v618 * v617;
            let v620 = v619 * v330;
            let v3205 = ((v3201 * v618) * v330) + (v2952 * v619);
            let v622 = v621 * v617;
            let v623 = v622 * v572;
            let v3209 = ((v3201 * v621) * v572) + (v3125 * v622);
            let v626 = v435 - v625;
            let v628 = (v113 * v626).exp();
            let v629 = v624 * v628;
            let v630 = -v290;
            let v632 = (v630 * v111).exp();
            let v3214 = (v2698 * v630) * v632;
            let v633 = v629 * v632;
            let v3217 = ((((v2700 * v626) * v628) * v624) * v632) + (v3214 * v629);
            let v637 = v634 - (v424 * v635);
            let v639 = (v113 * v637).exp();
            let v640 = v24 * v639;
            let v641 = v640 * v632;
            let v3223 = ((((v2700 * v637) * v639) * v24) * v632) + (v3214 * v640);
            let v643 = v2 - v625;
            let v645 = (v113 * v643).exp();
            let v646 = v642 * v645;
            let v3226 = ((v2700 * v643) * v645) * v642;
            let v648 = v2 - v635;
            let v650 = (v113 * v648).exp();
            let v651 = v647 * v650;
            let v3229 = ((v2700 * v648) * v650) * v647;
            let v653 = v341 - v27;
            let v655 = (v113 * v653).exp();
            let v656 = v652 * v655;
            let v658 = -v657;
            let v660 = (v658 * v111).exp();
            let v661 = v656 * v660;
            let v3237 = ((((v2700 * v653) * v655) * v652) * v660) + (((v2698 * v658) * v660) * v656);
            let v664 = (v342 + v341) - v2;
            let v666 = (v113 * v664).exp();
            let v667 = v662 * v666;
            let v3240 = ((v2700 * v664) * v666) * v662;
            let v669 = v366 - v2;
            let v671 = (v113 * v669).exp();
            let v672 = v668 * v671;
            let v3243 = ((v2700 * v669) * v671) * v668;
            let v674 = v667 + v672;
            let v3244 = v3240 + v3243;
            let v676 = v662 + v668;
            let v677 = (v673 * v674) / v676;
            let v3246 = (v3244 * v673) / v676;
            let v680 = v679 - v2;
            let v682 = (v113 * v680).exp();
            let v683 = v678 * v682;
            let v3249 = ((v2700 * v680) * v682) * v678;
            let v685 = v104 - v684;
            let v687 = if v104 < v686 { 1.0 } else { 0.0 };
            let v1845: f64;
            let v2561: f64;
            if v687 != 0.0 {
                let v693 = v692 * v685;
                let v696 = v688 * ((v2 + (v689 * v685)) - (v693 * v685));
                let v3256 = ((v2541 * v689) - (((v2541 * v692) * v685) + (v2541 * v693))) * v688;
                v1845 = v696;
                v2561 = v3256;
            } else {
                let v698 = v688 * v697;
                v1845 = v698;
                v2561 = v2958;
            }
            let v700 = v699 * v617;
            let v3257 = v3201 * v699;
            let v705 = v701 * ((v15 / v12).powf(v703));
            let v706 = if v353 > v0 { 1.0 } else { 0.0 };
            let v1970: f64;
            let v2562: f64;
            if v706 != 0.0 {
                let v707 = v2 / v357;
                let v3260 = ((v2967 * v707) * v2687) / v357;
                let v708 = if v707 > v23 { 1.0 } else { 0.0 };
                let v1971: f64;
                let v2563: f64;
                if v708 != 0.0 {
                    v1971 = v23;
                    v2563 = v2958;
                } else {
                    v1971 = v707;
                    v2563 = v3260;
                }
                v1970 = v1971;
                v2562 = v2563;
            } else {
                v1970 = v0;
                v2562 = v2958;
            }
            let v709 = if v358 > v0 { 1.0 } else { 0.0 };
            let v1975: f64;
            let v2564: f64;
            if v709 != 0.0 {
                let v710 = v2 / v362;
                let v3263 = ((v2970 * v710) * v2687) / v362;
                let v711 = if v710 > v23 { 1.0 } else { 0.0 };
                let v1976: f64;
                let v2565: f64;
                if v711 != 0.0 {
                    v1976 = v23;
                    v2565 = v2958;
                } else {
                    v1976 = v710;
                    v2565 = v3263;
                }
                v1975 = v1976;
                v2564 = v2565;
            } else {
                v1975 = v0;
                v2564 = v2958;
            }
            let v712 = if v363 > v0 { 1.0 } else { 0.0 };
            let v1980: f64;
            let v2566: f64;
            if v712 != 0.0 {
                let v713 = v2 / v364;
                let v3266 = ((v2971 * v713) * v2687) / v364;
                let v714 = if v713 > v23 { 1.0 } else { 0.0 };
                let v1981: f64;
                let v2567: f64;
                if v714 != 0.0 {
                    v1981 = v23;
                    v2567 = v2958;
                } else {
                    v1981 = v713;
                    v2567 = v3266;
                }
                v1980 = v1981;
                v2566 = v2567;
            } else {
                v1980 = v0;
                v2566 = v2958;
            }
            let v718 = v1 * (v715 - v716);
            let v3270 = ((Lanes([v2528, 0.0])) - (Lanes([0.0, v2529]))) * v1;
            let v721 = v1 * (v715 - v719);
            let v3274 = ((Lanes([v2528, 0.0])) - (Lanes([0.0, v2530]))) * v1;
            let v724 = v1 * (v715 - v722);
            let v3278 = ((Lanes([0.0, v2528])) - (Lanes([v2531, 0.0]))) * v1;
            let v727 = v1 * (v725 - v722);
            let v3282 = ((Lanes([0.0, v2532])) - (Lanes([v2531, 0.0]))) * v1;
            let v729 = v1 * (v725 - v715);
            let v3286 = ((Lanes([v2532, 0.0])) - (Lanes([0.0, v2528]))) * v1;
            let v732 = v1 * (v730 - v716);
            let v3290 = ((Lanes([v2533, 0.0])) - (Lanes([0.0, v2529]))) * v1;
            let v734 = v1 * (v716 - v719);
            let v3294 = ((Lanes([v2529, 0.0])) - (Lanes([0.0, v2530]))) * v1;
            let v737 = v1 * (v735 - v722);
            let v3298 = ((Lanes([v2534, 0.0])) - (Lanes([0.0, v2531]))) * v1;
            let v740 = v1 * (v738 - v725);
            let v3302 = ((Lanes([v2535, 0.0])) - (Lanes([0.0, v2532]))) * v1;
            let v742 = v1 * (v738 - v735);
            let v3306 = ((Lanes([v2535, 0.0])) - (Lanes([0.0, v2534]))) * v1;
            let v745 = v1 * (v738 - v743);
            let v3310 = ((Lanes([0.0, v2535])) - (Lanes([v2536, 0.0]))) * v1;
            let v748 = v1 * (v746 - v716);
            let v3314 = ((Lanes([0.0, v2537])) - (Lanes([v2529, 0.0]))) * v1;
            let v751 = v1 * (v749 - v746);
            let v3318 = ((Lanes([v2538, 0.0])) - (Lanes([0.0, v2537]))) * v1;
            let v3321 = (Lanes([v3286[0], v3286[1], 0.0])) + (Lanes([0.0, v3274[0], v3274[1]]));
            let v3324 = (Lanes([v3321[0], v3321[1], 0.0, v3321[2]])) - (Lanes([0.0, 0.0, v3294[0], v3294[1]]));
            let v754 = ((v729 + v721) - v734) - v748;
            let v3327 = (Lanes([v3324[0], v3324[1], v3324[2], v3324[3], 0.0])) - (Lanes([0.0, 0.0, v3314[0], 0.0, v3314[1]]));
            let v3328 = v3310 * v2687;
            let v3331 = (Lanes([v3328[0], v3328[1], 0.0])) + (Lanes([0.0, v3302[0], v3302[1]]));
            let v3334 = (Lanes([v3331[0], v3331[1], v3331[2], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, v3327[0], v3327[1], v3327[2], v3327[3], v3327[4]]));
            let v758 = (((-v745) + v740) + v754) - v751;
            let v3337 = (Lanes([v3334[0], v3334[1], v3334[2], v3334[3], v3334[4], v3334[5], 0.0, v3334[6]])) - (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, v3318[0], v3318[1]]));
            let v759 = v745 + v758;
            let v3339 = (Lanes([v3310[0], v3310[1], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + v3337;
            let v760 = v732 - v748;
            let v3342 = (Lanes([v3290[0], v3290[1], 0.0])) - (Lanes([0.0, v3314[0], v3314[1]]));
            let v761 = v760 - v751;
            let v3345 = (Lanes([v3342[0], v3342[1], 0.0, v3342[2]])) - (Lanes([0.0, 0.0, v3318[0], v3318[1]]));
            let v762 = v721 * v109;
            let v3346 = v3274 * v109;
            let v3350 = (Lanes([0.0, v3346[0], v3346[1]])) + (Lanes([(v2698 * v721), 0.0, 0.0]));
            let v764 = if v762 < v763 { 1.0 } else { 0.0 };
            let v1007: f64;
            let v2568: Lanes<3>;
            if v764 != 0.0 {
                let v765 = v762.exp();
                let v3352 = v3350 * v765;
                v1007 = v765;
                v2568 = v3352;
            } else {
                let v766 = v763.exp();
                let v769 = v766 * (v2 + (v762 - v763));
                let v3351 = v3350 * v766;
                v1007 = v769;
                v2568 = v3351;
            }
            let v770 = v724 * v109;
            let v3353 = v3278 * v109;
            let v3357 = (Lanes([0.0, v3353[0], v3353[1]])) + (Lanes([(v2698 * v724), 0.0, 0.0]));
            let v771 = v770 / v441;
            let v3361 = (v3357 - (Lanes([(v2553 * v771), 0.0, 0.0]))) / v441;
            let v772 = if v771 < v763 { 1.0 } else { 0.0 };
            let v1112: f64;
            let v2569: Lanes<3>;
            if v772 != 0.0 {
                let v773 = v771.exp();
                let v3363 = v3361 * v773;
                v1112 = v773;
                v2569 = v3363;
            } else {
                let v774 = v763.exp();
                let v777 = v774 * (v2 + (v771 - v763));
                let v3362 = v3361 * v774;
                v1112 = v777;
                v2569 = v3362;
            }
            let v778 = v754 * v109;
            let v3364 = v3327 * v109;
            let v3368 = (Lanes([0.0, v3364[0], v3364[1], v3364[2], v3364[3], v3364[4]])) + (Lanes([(v2698 * v754), 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v779 = if v778 < v763 { 1.0 } else { 0.0 };
            let v1498: f64;
            let v2570: Lanes<6>;
            if v779 != 0.0 {
                let v780 = v778.exp();
                let v3370 = v3368 * v780;
                v1498 = v780;
                v2570 = v3370;
            } else {
                let v781 = v763.exp();
                let v784 = v781 * (v2 + (v778 - v763));
                let v3369 = v3368 * v781;
                v1498 = v784;
                v2570 = v3369;
            }
            let v785 = v729 * v109;
            let v3371 = v3286 * v109;
            let v3375 = (Lanes([0.0, v3371[0], v3371[1]])) + (Lanes([(v2698 * v729), 0.0, 0.0]));
            let v786 = if v785 < v763 { 1.0 } else { 0.0 };
            let v1741: f64;
            let v2571: Lanes<3>;
            if v786 != 0.0 {
                let v787 = v785.exp();
                let v3377 = v3375 * v787;
                v1741 = v787;
                v2571 = v3377;
            } else {
                let v788 = v763.exp();
                let v791 = v788 * (v2 + (v785 - v763));
                let v3376 = v3375 * v788;
                v1741 = v791;
                v2571 = v3376;
            }
            let v792 = v759 * v109;
            let v3378 = v3339 * v109;
            let v3382 = (Lanes([v3378[0], v3378[1], 0.0, v3378[2], v3378[3], v3378[4], v3378[5], v3378[6], v3378[7]])) + (Lanes([0.0, 0.0, (v2698 * v759), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v793 = if v792 < v763 { 1.0 } else { 0.0 };
            let v1595: f64;
            let v2572: Lanes<9>;
            if v793 != 0.0 {
                let v794 = v792.exp();
                let v3384 = v3382 * v794;
                v1595 = v794;
                v2572 = v3384;
            } else {
                let v795 = v763.exp();
                let v798 = v795 * (v2 + (v792 - v763));
                let v3383 = v3382 * v795;
                v1595 = v798;
                v2572 = v3383;
            }
            let v799 = v732 * v109;
            let v3385 = v3290 * v109;
            let v3389 = (Lanes([v3385[0], 0.0, v3385[1]])) + (Lanes([0.0, (v2698 * v732), 0.0]));
            let v800 = if v799 < v763 { 1.0 } else { 0.0 };
            let v1526: f64;
            let v2573: Lanes<3>;
            if v800 != 0.0 {
                let v801 = v799.exp();
                let v3391 = v3389 * v801;
                v1526 = v801;
                v2573 = v3391;
            } else {
                let v802 = v763.exp();
                let v805 = v802 * (v2 + (v799 - v763));
                let v3390 = v3389 * v802;
                v1526 = v805;
                v2573 = v3390;
            }
            let v806 = v761 * v109;
            let v3392 = v3345 * v109;
            let v3396 = (Lanes([v3392[0], 0.0, v3392[1], v3392[2], v3392[3]])) + (Lanes([0.0, (v2698 * v761), 0.0, 0.0, 0.0]));
            let v807 = if v806 < v763 { 1.0 } else { 0.0 };
            let v1607: f64;
            let v2574: Lanes<5>;
            if v807 != 0.0 {
                let v808 = v806.exp();
                let v3398 = v3396 * v808;
                v1607 = v808;
                v2574 = v3398;
            } else {
                let v809 = v763.exp();
                let v812 = v809 * (v2 + (v806 - v763));
                let v3397 = v3396 * v809;
                v1607 = v812;
                v2574 = v3397;
            }
            let v813 = v760 * v109;
            let v3399 = v3342 * v109;
            let v3403 = (Lanes([v3399[0], 0.0, v3399[1], v3399[2]])) + (Lanes([0.0, (v2698 * v760), 0.0, 0.0]));
            let v814 = if v813 < v763 { 1.0 } else { 0.0 };
            let v1542: f64;
            let v2575: Lanes<4>;
            if v814 != 0.0 {
                let v815 = v813.exp();
                let v3405 = v3403 * v815;
                v1542 = v815;
                v2575 = v3405;
            } else {
                let v816 = v763.exp();
                let v819 = v816 * (v2 + (v813 - v763));
                let v3404 = v3403 * v816;
                v1542 = v819;
                v2575 = v3404;
            }
            let v821 = v759 - v820;
            let v3406 = Lanes([v3339[0], v3339[1], 0.0, v3339[2], v3339[3], v3339[4], v3339[5], v3339[6], v3339[7]]);
            let v822 = v821 * v109;
            let v3412 = ((v3406 - (Lanes([0.0, 0.0, v2545, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v109) + (Lanes([0.0, 0.0, (v2698 * v821), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v823 = if v822 < v763 { 1.0 } else { 0.0 };
            let v2223: f64;
            let v2576: Lanes<9>;
            if v823 != 0.0 {
                let v824 = v822.exp();
                let v3414 = v3412 * v824;
                v2223 = v824;
                v2576 = v3414;
            } else {
                let v825 = v763.exp();
                let v828 = v825 * (v2 + (v822 - v763));
                let v3413 = v3412 * v825;
                v2223 = v828;
                v2576 = v3413;
            }
            let v829 = v754 - v820;
            let v3415 = Lanes([0.0, v3327[0], v3327[1], v3327[2], v3327[3], v3327[4]]);
            let v830 = v829 * v109;
            let v3421 = ((v3415 - (Lanes([v2545, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v109) + (Lanes([(v2698 * v829), 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v831 = if v830 < v763 { 1.0 } else { 0.0 };
            let v1500: f64;
            let v2577: Lanes<6>;
            if v831 != 0.0 {
                let v832 = v830.exp();
                let v3423 = v3421 * v832;
                v1500 = v832;
                v2577 = v3423;
            } else {
                let v833 = v763.exp();
                let v836 = v833 * (v2 + (v830 - v763));
                let v3422 = v3421 * v833;
                v1500 = v836;
                v2577 = v3422;
            }
            let v837 = v721 - v820;
            let v838 = v837 * v109;
            let v3430 = (((Lanes([0.0, v3274[0], v3274[1]])) - (Lanes([v2545, 0.0, 0.0]))) * v109) + (Lanes([(v2698 * v837), 0.0, 0.0]));
            let v839 = if v838 < v763 { 1.0 } else { 0.0 };
            let v853: f64;
            let v2578: Lanes<3>;
            if v839 != 0.0 {
                let v840 = v838.exp();
                let v3432 = v3430 * v840;
                v853 = v840;
                v2578 = v3432;
            } else {
                let v841 = v763.exp();
                let v844 = v841 * (v2 + (v838 - v763));
                let v3431 = v3430 * v841;
                v853 = v844;
                v2578 = v3431;
            }
            let v845 = v718 - v820;
            let v3433 = Lanes([0.0, v3270[0], v3270[1]]);
            let v3434 = Lanes([v2545, 0.0, 0.0]);
            let v846 = v845 * v109;
            let v3439 = ((v3433 - v3434) * v109) + (Lanes([(v2698 * v845), 0.0, 0.0]));
            let v847 = if v846 < v763 { 1.0 } else { 0.0 };
            let v857: f64;
            let v2579: Lanes<3>;
            if v847 != 0.0 {
                let v848 = v846.exp();
                let v3441 = v3439 * v848;
                v857 = v848;
                v2579 = v3441;
            } else {
                let v849 = v763.exp();
                let v852 = v849 * (v2 + (v846 - v763));
                let v3440 = v3439 * v849;
                v857 = v852;
                v2579 = v3440;
            }
            let v856 = (v2 + (v435 * v853)).sqrt();
            let v3445 = (v2578 * v435) * (v2526 / (v3005 * v856));
            let v860 = (v2 + (v435 * v857)).sqrt();
            let v3449 = (v2579 * v435) * (v2526 / (v3005 * v860));
            let v862 = v2 + v860;
            let v863 = (v27 * v857) / v862;
            let v3453 = ((v2579 * v27) - (v3449 * v863)) / v862;
            let v865 = if v863 < v864 { 1.0 } else { 0.0 };
            let v952: f64;
            let v2580: Lanes<3>;
            if v865 != 0.0 {
                v952 = v864;
                v2580 = v3454;
            } else {
                v952 = v863;
                v2580 = v3453;
            }
            let v3455 = Lanes([v3445[0], v3445[1], 0.0, v3445[2]]);
            let v867 = v856 + v2;
            let v868 = v867 / v862;
            let v3458 = v3449 * v868;
            let v870 = (v856 - v860) - (v868.ln());
            let v871 = v107 * v870;
            let v3468 = (Lanes([(v2695 * v870), 0.0, 0.0, 0.0])) + (((v3455 - (Lanes([v3449[0], v3449[1], v3449[2], 0.0]))) - (((v3455 - (Lanes([v3458[0], v3458[1], v3458[2], 0.0]))) / v862) * (v2526 / v868))) * v107);
            let v3469 = Lanes([0.0, 0.0, v3294[0], v3294[1]]);
            let v873 = (v871 + v734) / v369;
            let v3474 = ((v3468 + v3469) - (Lanes([(v2974 * v873), 0.0, 0.0, 0.0]))) / v369;
            let v874 = if v873 > v0 { 1.0 } else { 0.0 };
            let v1063: f64;
            let v1076: f64;
            let v1091: f64;
            let v1118: f64;
            let v1793: f64;
            let v1829: f64;
            let v2180: f64;
            let v2581: Lanes<4>;
            let v2582: Lanes<4>;
            let v2583: Lanes<4>;
            let v2584: Lanes<4>;
            let v2585: Lanes<4>;
            let v2586: Lanes<4>;
            let v2587: Lanes<4>;
            if v874 != 0.0 {
                let v876 = if v718 < v875 { 1.0 } else { 0.0 };
                let v889: f64;
                let v2588: Lanes<2>;
                if v876 != 0.0 {
                    v889 = v718;
                    v2588 = v3270;
                } else {
                    let v878 = v2 + (v718 - v875);
                    let v3502 = v3270 * (v2526 / v878);
                    let v880 = v875 + (v878.ln());
                    v889 = v880;
                    v2588 = v3502;
                }
                let v881 = v27 * v107;
                let v882 = v424 * v873;
                let v883 = v882 * v369;
                let v3508 = ((v3474 * v424) * v369) + (Lanes([(v2974 * v882), 0.0, 0.0, 0.0]));
                let v885 = (v883 * v109) + v2;
                let v886 = v885.ln();
                let v890 = (v820 + (v881 * v886)) - v889;
                let v3522 = ((Lanes([v2545, 0.0, 0.0, 0.0])) + ((Lanes([((v2695 * v27) * v886), 0.0, 0.0, 0.0])) + ((((v3508 * v109) + (Lanes([(v2698 * v883), 0.0, 0.0, 0.0]))) * (v2526 / v885)) * v881))) - (Lanes([0.0, v2588[0], v2588[1], 0.0]));
                let v892 = v891 * v820;
                let v893 = v892 * v892;
                let v3524 = (v2545 * v891) * v892;
                let v3525 = v3524 + v3524;
                let v894 = v890 * v890;
                let v3526 = v3522 * v890;
                let v3527 = v3526 + v3526;
                let v895 = if v890 < v0 { 1.0 } else { 0.0 };
                let v905: f64;
                let v2589: Lanes<4>;
                if v895 != 0.0 {
                    let v898 = (v894 + v893).sqrt();
                    let v899 = v898 - v890;
                    let v900 = (v424 * v893) / v899;
                    let v3545 = ((Lanes([(v3525 * v424), 0.0, 0.0, 0.0])) - ((((v3527 + (Lanes([v3525, 0.0, 0.0, 0.0]))) * (v2526 / (v3005 * v898))) - v3522) * v900)) / v899;
                    v905 = v900;
                    v2589 = v3545;
                } else {
                    let v902 = (v894 + v893).sqrt();
                    let v904 = v424 * (v902 + v890);
                    let v3534 = (((v3527 + (Lanes([v3525, 0.0, 0.0, 0.0]))) * (v2526 / (v3005 * v902))) + v3522) * v424;
                    v905 = v904;
                    v2589 = v3534;
                }
                let v908 = v906 * v907;
                let v909 = v905 + v908;
                let v913 = v907 * (v905 + (v906 * v369));
                let v914 = (v905 * v909) / v913;
                let v3555 = (((v2589 * v909) + (v2589 * v905)) - (((v2589 + (Lanes([(v2974 * v906), 0.0, 0.0, 0.0]))) * v907) * v914)) / v913;
                let v915 = v873 / v914;
                let v3558 = (v3474 - (v3555 * v915)) / v914;
                let v918 = (v915 - v2) / v917;
                let v3559 = v3558 / v917;
                let v919 = if v915 < v2 { 1.0 } else { 0.0 };
                let v931: f64;
                let v2590: Lanes<4>;
                if v919 != 0.0 {
                    let v920 = v918.exp();
                    let v921 = v2 + v920;
                    let v3569 = ((v3559 * v920) * (v2526 / v921)) * v917;
                    let v924 = v2 + (v917 * (v921.ln()));
                    v931 = v924;
                    v2590 = v3569;
                } else {
                    let v926 = (-v918).exp();
                    let v927 = v2 + v926;
                    let v930 = v915 + (v917 * (v927.ln()));
                    let v3565 = v3558 + ((((v3559 * v2687) * v926) * (v2526 / v927)) * v917);
                    v931 = v930;
                    v2590 = v3565;
                }
                let v938 = v2 + (v917 * ((v2 + ((v932 / v917).exp())).ln()));
                let v939 = v931 / v938;
                let v3570 = v2590 / v938;
                let v940 = v905 / v908;
                let v3571 = v2589 / v908;
                let v941 = v435 * v939;
                let v942 = v941 * v940;
                let v943 = v2 + v940;
                let v946 = (v2 + (v942 * v943)).sqrt();
                let v948 = v27 * v939;
                let v949 = v948 * v943;
                let v950 = (v2 + v946) / v949;
                let v3588 = (((((((v3570 * v435) * v940) + (v3571 * v941)) * v943) + (v3571 * v942)) * (v2526 / (v3005 * v946))) - ((((v3570 * v27) * v943) + (v3571 * v948)) * v950)) / v949;
                let v953 = v952 * v950;
                let v3590 = v2580 * v950;
                let v3593 = (Lanes([v3590[0], v3590[1], v3590[2], 0.0])) + (v3588 * v952);
                let v955 = v2 + v953;
                let v956 = ((v2 - v950) + v953) / v955;
                let v3597 = (((v3588 * v2687) + v3593) - (v3593 * v956)) / v955;
                let v957 = v883 * v956;
                let v958 = v957 * v109;
                let v3604 = (((v3508 * v956) + (v3597 * v883)) * v109) + (Lanes([(v2698 * v957), 0.0, 0.0, 0.0]));
                let v961 = (v952 + v958) + v2;
                let v3608 = v2580 * v961;
                let v963 = (v27 * v958) + (v952 * v961);
                let v3612 = (v3604 * v27) + ((Lanes([v3608[0], v3608[1], v3608[2], 0.0])) + (((Lanes([v2580[0], v2580[1], v2580[2], 0.0])) + v3604) * v952));
                let v965 = v424 * (v958 - v2);
                let v3613 = v3604 * v424;
                let v3614 = v3613 * v965;
                let v967 = (v965 * v965) + v963;
                let v3616 = (v3614 + v3614) + v3612;
                let v968 = if v958 >= v2 { 1.0 } else { 0.0 };
                let v974: f64;
                let v2591: Lanes<4>;
                if v968 != 0.0 {
                    let v969 = v967.sqrt();
                    let v970 = v965 + v969;
                    let v3627 = v3613 + (v3616 * (v2526 / (v3005 * v969)));
                    v974 = v970;
                    v2591 = v3627;
                } else {
                    let v971 = v967.sqrt();
                    let v972 = v971 - v965;
                    let v973 = v963 / v972;
                    let v3623 = (v3612 - (((v3616 * (v2526 / (v3005 * v971))) - v3613) * v973)) / v972;
                    v974 = v973;
                    v2591 = v3623;
                }
                let v976 = if v974 < v975 { 1.0 } else { 0.0 };
                let v977: f64;
                let v2592: Lanes<4>;
                if v976 != 0.0 {
                    v977 = v975;
                    v2592 = v3500;
                } else {
                    v977 = v974;
                    v2592 = v2591;
                }
                let v978 = v977 + v2;
                let v979 = v977 * v978;
                let v981 = (v820 * v109).exp();
                let v982 = v979 * v981;
                let v3638 = (((v2592 * v978) + (v2592 * v977)) * v981) + (Lanes([((((v2545 * v109) + (v2698 * v820)) * v981) * v979), 0.0, 0.0, 0.0]));
                let v983 = v424 * v907;
                let v985 = v983 * (v873 - v906);
                let v3639 = v3474 * v983;
                let v987 = (v907 * v369) * v906;
                let v3646 = v3639 * v985;
                let v991 = ((v985 * v985) + (v987 * v873)).sqrt();
                let v992 = v985 + v991;
                let v3652 = v3639 + (((v3646 + v3646) + ((Lanes([(((v2974 * v907) * v906) * v873), 0.0, 0.0, 0.0])) + (v3474 * v987))) * (v2526 / (v3005 * v991)));
                let v994 = if v993 == v0 { 1.0 } else { 0.0 };
                let v1077: f64;
                let v2593: Lanes<4>;
                if v994 != 0.0 {
                    let v995 = v325 * v42;
                    let v3663 = Lanes([(v2547 * v42), 0.0, 0.0, 0.0]);
                    v1077 = v995;
                    v2593 = v3663;
                } else {
                    let v997 = v873 + v914;
                    let v998 = (v27 * v873) / v997;
                    let v999 = v42 + v998;
                    let v1000 = v325 * v999;
                    let v3661 = (Lanes([(v2547 * v999), 0.0, 0.0, 0.0])) + ((((v3474 * v27) - ((v3474 + v3555) * v998)) / v997) * v325);
                    v1077 = v1000;
                    v2593 = v3661;
                }
                let v1002 = v906 + v873;
                let v1003 = (v906 * v873) / v1002;
                let v3667 = ((v3474 * v906) - (v3474 * v1003)) / v1002;
                let v1004 = v906 / v1002;
                let v3670 = ((v3474 * v1004) * v2687) / v1002;
                v1063 = v992;
                v1076 = v1077;
                v1091 = v1004;
                v1118 = v982;
                v1793 = v956;
                v1829 = v1003;
                v2180 = v977;
                v2581 = v3652;
                v2582 = v2593;
                v2583 = v3670;
                v2584 = v3638;
                v2585 = v3597;
                v2586 = v3667;
                v2587 = v2592;
            } else {
                let v1006 = (v27 * v853) / v867;
                let v3478 = ((v2578 * v27) - (v3445 * v1006)) / v867;
                let v1018 = if (if (v734.abs()) < (v1009 * v107) { 1.0 } else { 0.0 }) != 0.0 || (if (v871.abs()) < ((v1013 * v107) * (v856 + v860)) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1794: f64;
                let v2594: Lanes<4>;
                if v1018 != 0.0 {
                    let v1020 = v424 * (v1006 + v952);
                    let v3490 = ((Lanes([v3478[0], v3478[1], 0.0, v3478[2]])) + (Lanes([v2580[0], v2580[1], v2580[2], 0.0]))) * v424;
                    let v1021 = v1020 + v2;
                    let v1022 = v1020 / v1021;
                    let v3493 = (v3490 - (v3490 * v1022)) / v1021;
                    v1794 = v1022;
                    v2594 = v3493;
                } else {
                    let v1024 = (v871 + v721) - v718;
                    let v1025 = v871 / v1024;
                    let v3486 = (v3468 - (((v3468 + (Lanes([0.0, v3274[0], 0.0, v3274[1]]))) - (Lanes([0.0, v3270[0], v3270[1], 0.0]))) * v1025)) / v1024;
                    v1794 = v1025;
                    v2594 = v3486;
                }
                let v1026 = v42 * v325;
                let v1028 = v2 - (v873 / v906);
                let v3496 = (v3474 / v906) * v2687;
                let v3497 = Lanes([(v2547 * v42), 0.0, 0.0, 0.0]);
                let v3498 = Lanes([v2568[0], v2568[1], 0.0, v2568[2]]);
                let v3499 = Lanes([v3478[0], v3478[1], 0.0, v3478[2]]);
                v1063 = v734;
                v1076 = v1026;
                v1091 = v1028;
                v1118 = v1007;
                v1793 = v1794;
                v1829 = v873;
                v2180 = v1006;
                v2581 = v3469;
                v2582 = v3497;
                v2583 = v3496;
                v2584 = v3498;
                v2585 = v2594;
                v2586 = v3474;
                v2587 = v3499;
            }
            let v1032 = v2 - (v154.powf((v1029 / v28)));
            let v1033 = v307 * v1032;
            let v3671 = v2544 * v1032;
            let v1034 = v42 * v307;
            let v3672 = v2544 * v42;
            let v3673 = Lanes([0.0, v3278[0], v3278[1]]);
            let v3674 = Lanes([v3671, 0.0, 0.0]);
            let v1036 = (v724 - v1033) / v1034;
            let v3679 = ((v3673 - v3674) - (Lanes([(v3672 * v1036), 0.0, 0.0]))) / v1034;
            let v1037 = if v724 < v1033 { 1.0 } else { 0.0 };
            let v1049: f64;
            let v2595: Lanes<3>;
            if v1037 != 0.0 {
                let v1038 = v1036.exp();
                let v1039 = v2 + v1038;
                let v1040 = v1039.ln();
                let v1042 = v724 - (v1034 * v1040);
                let v3696 = v3673 - ((Lanes([(v3672 * v1040), 0.0, 0.0])) + (((v3679 * v1038) * (v2526 / v1039)) * v1034));
                v1049 = v1042;
                v2595 = v3696;
            } else {
                let v1044 = (-v1036).exp();
                let v1045 = v2 + v1044;
                let v1046 = v1045.ln();
                let v1048 = v1033 - (v1034 * v1046);
                let v3688 = v3674 - ((Lanes([(v3672 * v1046), 0.0, 0.0])) + ((((v3679 * v2687) * v1044) * (v2526 / v1045)) * v1034));
                v1049 = v1048;
                v2595 = v3688;
            }
            let v1051 = v2 - (v1049 * v308);
            let v3701 = ((v2595 * v308) + (Lanes([(v2920 * v1049), 0.0, 0.0]))) * v2687;
            let v1052 = v2 - v28;
            let v1053 = v1051.powf(v1052);
            let v3702 = v1052 - v2526;
            let v3705 = v3701 * (v1052 * (v1051.powf(v3702)));
            let v1054 = v307 / v1052;
            let v3706 = v2544 / v1052;
            let v1055 = v2 - v1053;
            let v1059 = (v1054 * v1055) + (v154 * (v724 - v1049));
            let v3714 = ((Lanes([(v3706 * v1055), 0.0, 0.0])) + ((v3705 * v2687) * v1054)) + ((v3673 - v2595) * v154);
            let v1061 = if v1060 == v2 { 1.0 } else { 0.0 };
            let v1073: f64;
            let v2596: Lanes<4>;
            if v1061 != 0.0 {
                let v3718 = Lanes([0.0, v3270[0], v3270[1], 0.0]);
                v1073 = v718;
                v2596 = v3718;
            } else {
                let v1062 = if v1060 == v27 { 1.0 } else { 0.0 };
                let v1074: f64;
                let v2597: Lanes<4>;
                if v1062 != 0.0 {
                    let v1064 = v718 + v1063;
                    let v3717 = (Lanes([0.0, v3270[0], v3270[1], 0.0])) + v2581;
                    v1074 = v1064;
                    v2597 = v3717;
                } else {
                    let v3715 = Lanes([0.0, v3274[0], 0.0, v3274[1]]);
                    v1074 = v721;
                    v2597 = v3715;
                }
                v1073 = v1074;
                v2596 = v2597;
            }
            let v3719 = v2954 * v2687;
            let v1066 = v2 - v333;
            let v1067 = (v27 - v333) / v1066;
            let v3722 = (v3719 - (v3719 * v1067)) / v1066;
            let v1069 = v1068 / v60;
            let v1071 = v2 - (v1067.powf(v1069));
            let v1072 = v325 * v1071;
            let v3730 = (v2547 * v1071) + (((v3722 * (v1069 * (v1067.powf((v1069 - v2526))))) * v2687) * v325);
            let v3731 = Lanes([v3730, 0.0, 0.0, 0.0]);
            let v1078 = (v1073 - v1072) / v1076;
            let v3735 = ((v2596 - v3731) - (v2582 * v1078)) / v1076;
            let v1079 = if v1073 < v1072 { 1.0 } else { 0.0 };
            let v1096: f64;
            let v2598: Lanes<4>;
            if v1079 != 0.0 {
                let v1080 = v1078.exp();
                let v1081 = v2 + v1080;
                let v1082 = v1081.ln();
                let v1084 = v1073 - (v1076 * v1082);
                let v3750 = v2596 - ((v2582 * v1082) + (((v3735 * v1080) * (v2526 / v1081)) * v1076));
                v1096 = v1084;
                v2598 = v3750;
            } else {
                let v1086 = (-v1078).exp();
                let v1087 = v2 + v1086;
                let v1088 = v1087.ln();
                let v1090 = v1072 - (v1076 * v1088);
                let v3743 = v3731 - ((v2582 * v1088) + ((((v3735 * v2687) * v1086) * (v2526 / v1087)) * v1076));
                v1096 = v1090;
                v2598 = v3743;
            }
            let v1093 = v1091.powf(v1092);
            let v3754 = v2583 * (v1092 * (v1091.powf((v1092 - v2526))));
            let v1094 = v2 - v60;
            let v1095 = v325 / v1094;
            let v3755 = v2547 / v1094;
            let v1097 = v1096 / v325;
            let v1098 = v2 - v1097;
            let v1099 = v1098.powf(v1094);
            let v3761 = v1094 - v2526;
            let v1101 = v2 - (v1093 * v1099);
            let v1103 = v1093 * v1067;
            let v1104 = v1073 - v1096;
            let v1106 = (v1095 * v1101) + (v1103 * v1104);
            let v3787 = v3270 * v333;
            let v3790 = (Lanes([(v2954 * v718), 0.0, 0.0])) + (Lanes([0.0, v3787[0], v3787[1]]));
            let v1109 = (v1066 * v1106) + (v333 * v718);
            let v3792 = ((Lanes([(v3719 * v1106), 0.0, 0.0, 0.0])) + ((((Lanes([(v3755 * v1101), 0.0, 0.0, 0.0])) + ((((v3754 * v1099) + (((((v2598 - (Lanes([(v2547 * v1097), 0.0, 0.0, 0.0]))) / v325) * v2687) * (v1094 * (v1098.powf(v3761)))) * v1093)) * v2687) * v1095)) + ((((v3754 * v1067) + (Lanes([(v3722 * v1093), 0.0, 0.0, 0.0]))) * v1104) + ((v2596 - v2598) * v1103))) * v1066)) + (Lanes([v3790[0], v3790[1], v3790[2], 0.0]));
            let v1111 = (v435 * v449) / v454;
            let v3796 = ((v3031 * v435) - (v3034 * v1111)) / v454;
            let v1113 = v1111 * v1112;
            let v3800 = (Lanes([(v3796 * v1112), 0.0, 0.0])) + (v2569 * v1111);
            let v1115 = (v2 + v1113).sqrt();
            let v3803 = v3800 * (v2526 / (v3005 * v1115));
            let v1116 = v2 + v1115;
            let v1117 = v1113 / v1116;
            let v3806 = (v3800 - (v3803 * v1117)) / v1116;
            let v1120 = v2 / v1119;
            let v1121 = v1118.powf(v1120);
            let v3814 = v1118.ln();
            let v3818 = (v2584 * (v1120 * (v1118.powf((v1120 - v2526))))) + (Lanes([((((v2555 * v1120) * v2687) / v1119) * (v1121 * v3814)), 0.0, 0.0, 0.0]));
            let v1122 = v1111 * v1121;
            let v3822 = (Lanes([(v3796 * v1121), 0.0, 0.0, 0.0])) + (v3818 * v1111);
            let v1124 = (v2 + v1122).sqrt();
            let v1125 = v2 + v1124;
            let v1126 = v1122 / v1125;
            let v3828 = (v3822 - ((v3822 * (v2526 / (v3005 * v1124))) * v1126)) / v1125;
            let v1127 = if v699 == v0 { 1.0 } else { 0.0 };
            let v1148: f64;
            let v2599: Lanes<5>;
            if v1127 != 0.0 {
                let v1128 = v1059 / v623;
                let v3870 = (v3714 - (Lanes([(v3209 * v1128), 0.0, 0.0]))) / v623;
                let v1130 = v1109 / v620;
                let v3874 = (v3792 - (Lanes([(v3205 * v1130), 0.0, 0.0, 0.0]))) / v620;
                let v1131 = (v2 + v1128) + v1130;
                let v3877 = (Lanes([v3870[0], v3870[1], v3870[2], 0.0, 0.0])) + (Lanes([v3874[0], 0.0, v3874[1], v3874[2], v3874[3]]));
                v1148 = v1131;
                v2599 = v3877;
            } else {
                let v1132 = v1059 / v623;
                let v1133 = v1132 + v2;
                let v1134 = v1133 * v700;
                let v1137 = (-v1109) / v620;
                let v1138 = v1137 * v700;
                let v1140 = (v1134 * v109).exp();
                let v3854 = ((((((v3714 - (Lanes([(v3209 * v1132), 0.0, 0.0]))) / v623) * v700) + (Lanes([(v3257 * v1133), 0.0, 0.0]))) * v109) + (Lanes([(v2698 * v1134), 0.0, 0.0]))) * v1140;
                let v1141 = (v1138 * v109).exp();
                let v3855 = (((((((v3792 * v2687) - (Lanes([(v3205 * v1137), 0.0, 0.0, 0.0]))) / v620) * v700) + (Lanes([(v3257 * v1137), 0.0, 0.0, 0.0]))) * v109) + (Lanes([(v2698 * v1138), 0.0, 0.0, 0.0]))) * v1141;
                let v1144 = (v700 * v109).exp();
                let v1145 = v1144 - v2;
                let v1146 = (v1140 - v1141) / v1145;
                let v3866 = (((Lanes([v3854[0], v3854[1], v3854[2], 0.0, 0.0])) - (Lanes([v3855[0], 0.0, v3855[1], v3855[2], v3855[3]]))) - (Lanes([((((v3257 * v109) + (v2698 * v700)) * v1144) * v1146), 0.0, 0.0, 0.0, 0.0]))) / v1145;
                v1148 = v1146;
                v2599 = v3866;
            }
            let v1149 = v1148 * v1148;
            let v3878 = v2599 * v1148;
            let v3879 = v3878 + v3878;
            let v1150 = if v1148 < v0 { 1.0 } else { 0.0 };
            let v1160: f64;
            let v2600: Lanes<5>;
            if v1150 != 0.0 {
                let v1153 = (v1149 + v1147).sqrt();
                let v1154 = v1153 - v1148;
                let v1155 = v1151 / v1154;
                let v3891 = ((((v3879 * (v2526 / (v3005 * v1153))) - v2599) * v1155) * v2687) / v1154;
                v1160 = v1155;
                v2600 = v3891;
            } else {
                let v1157 = (v1149 + v1147).sqrt();
                let v1159 = v424 * (v1157 + v1148);
                let v3884 = ((v3879 * (v2526 / (v3005 * v1157))) + v2599) * v424;
                v1160 = v1159;
                v2600 = v3884;
            }
            let v3895 = ((Lanes([v3806[0], v3806[1], v3806[2], 0.0, 0.0])) + (Lanes([v3828[0], 0.0, v3828[1], v3828[2], v3828[3]]))) * v424;
            let v1163 = v2 + (v424 * (v1117 + v1126));
            let v1164 = v1160 * v1163;
            let v3898 = (v2600 * v1163) + (v3895 * v1160);
            let v1166 = v1165 * v449;
            let v1167 = v1166 * v1121;
            let v3903 = (Lanes([((v3031 * v1165) * v1121), 0.0, 0.0, 0.0])) + (v3818 * v1166);
            let v1168 = v449 * v1112;
            let v3907 = (Lanes([(v3031 * v1112), 0.0, 0.0])) + (v2569 * v449);
            let v3908 = Lanes([v3907[0], v3907[1], v3907[2], 0.0, 0.0]);
            let v3909 = Lanes([v3903[0], 0.0, v3903[1], v3903[2], v3903[3]]);
            let v1170 = (v1168 - v1167) / v1164;
            let v3913 = ((v3908 - v3909) - (v3898 * v1170)) / v1164;
            let v1172 = v724 / v1171;
            let v3914 = v3278 / v1171;
            let v1173 = if v724 < v0 { 1.0 } else { 0.0 };
            let v1184: f64;
            let v2601: Lanes<2>;
            if v1173 != 0.0 {
                let v1174 = v1172.exp();
                let v1175 = v2 + v1174;
                let v1177 = v1171 * (v1175.ln());
                let v3924 = ((v3914 * v1174) * (v2526 / v1175)) * v1171;
                v1184 = v1177;
                v2601 = v3924;
            } else {
                let v1179 = (-v1172).exp();
                let v1180 = v2 + v1179;
                let v1183 = v724 + (v1171 * (v1180.ln()));
                let v3920 = v3278 + ((((v3914 * v2687) * v1179) * (v2526 / v1180)) * v1171);
                v1184 = v1183;
                v2601 = v3920;
            }
            let v1186 = v1184 / v1185;
            let v3925 = v2601 / v1185;
            let v1187 = if v1186 < v763 { 1.0 } else { 0.0 };
            let v1193: f64;
            let v2602: Lanes<2>;
            if v1187 != 0.0 {
                let v1188 = v1186.exp();
                let v3927 = v3925 * v1188;
                v1193 = v1188;
                v2602 = v3927;
            } else {
                let v1189 = v763.exp();
                let v1192 = v1189 * (v2 + (v1186 - v763));
                let v3926 = v3925 * v1189;
                v1193 = v1192;
                v2602 = v3926;
            }
            let v1194 = v1193 - v2;
            let v1195 = v567 * v1194;
            let v3929 = v2602 * v567;
            let v3932 = (Lanes([(v3117 * v1194), 0.0, 0.0])) + (Lanes([0.0, v3929[0], v3929[1]]));
            let v1198 = (v724 - v1196) / v26;
            let v3933 = v3278 / v26;
            let v1199 = if v724 < v1196 { 1.0 } else { 0.0 };
            let v1212: f64;
            let v2603: Lanes<2>;
            if v1199 != 0.0 {
                let v1200 = v1198.exp();
                let v1201 = v2 + v1200;
                let v1204 = v724 - (v26 * (v1201.ln()));
                let v3944 = v3278 - (((v3933 * v1200) * (v2526 / v1201)) * v26);
                v1212 = v1204;
                v2603 = v3944;
            } else {
                let v1206 = (-v1198).exp();
                let v1207 = v2 + v1206;
                let v1210 = v1196 - (v26 * (v1207.ln()));
                let v3939 = ((((v3933 * v2687) * v1206) * (v2526 / v1207)) * v26) * v2687;
                v1212 = v1210;
                v2603 = v3939;
            }
            let v1213 = v1211 * v1212;
            let v1214 = v1196 - v1212;
            let v1215 = v1214 * v1214;
            let v1216 = v1213 * v1215;
            let v3951 = ((v2603 * v1211) * v1215) + (((v2603 * v2687) * (v27 * v1214)) * v1213);
            let v1217 = v770 / v491;
            let v3952 = v3357 / v491;
            let v1218 = if v1217 < v763 { 1.0 } else { 0.0 };
            let v1243: f64;
            let v2604: Lanes<3>;
            if v1218 != 0.0 {
                let v1219 = v1217.exp();
                let v3954 = v3952 * v1219;
                v1243 = v1219;
                v2604 = v3954;
            } else {
                let v1220 = v763.exp();
                let v1223 = v1220 * (v2 + (v1217 - v763));
                let v3953 = v3952 * v1220;
                v1243 = v1223;
                v2604 = v3953;
            }
            let v1989: f64;
            let v2605: Lanes<5>;
            if v510 != 0.0 {
                let v1225 = v724 - v1224;
                let v1226 = v1225 * v109;
                let v3984 = ((v3673 - (Lanes([v2549, 0.0, 0.0]))) * v109) + (Lanes([(v2698 * v1225), 0.0, 0.0]));
                let v1227 = if v1226 < v763 { 1.0 } else { 0.0 };
                let v1249: f64;
                let v2606: Lanes<3>;
                if v1227 != 0.0 {
                    let v1228 = v1226.exp();
                    let v3986 = v3984 * v1228;
                    v1249 = v1228;
                    v2606 = v3986;
                } else {
                    let v1229 = v763.exp();
                    let v1232 = v1229 * (v2 + (v1226 - v763));
                    let v3985 = v3984 * v1229;
                    v1249 = v1232;
                    v2606 = v3985;
                }
                let v1233 = v1170 / v449;
                let v3990 = (v3913 - (Lanes([(v3031 * v1233), 0.0, 0.0, 0.0, 0.0]))) / v449;
                let v1235 = v1233 - v1234;
                let v1237 = if v1235 < v1236 { 1.0 } else { 0.0 };
                let v1262: f64;
                let v2607: Lanes<5>;
                if v1237 != 0.0 {
                    let v1238 = v1235.exp();
                    let v3992 = v3990 * v1238;
                    v1262 = v1238;
                    v2607 = v3992;
                } else {
                    let v1242 = v1239 * (v2 + (v1235 - v1236));
                    let v3991 = v3990 * v1239;
                    v1262 = v1242;
                    v2607 = v3991;
                }
                let v1244 = v1243 - v2;
                let v3996 = (Lanes([(v3065 * v1244), 0.0, 0.0])) + (v2604 * v500);
                let v1247 = v1246 * v27;
                let v1252 = (v2 + (v435 * v1249)).sqrt();
                let v1253 = v2 + v1252;
                let v1254 = (v1247 * v1244) / v1253;
                let v1255 = v1109 / v620;
                let v1256 = v2 + v1255;
                let v4013 = ((((Lanes([((v2558 * v27) * v1244), 0.0, 0.0])) + (v2604 * v1247)) - (((v2606 * v435) * (v2526 / (v3005 * v1252))) * v1254)) / v1253) * v1256;
                let v4014 = ((v3792 - (Lanes([(v3205 * v1255), 0.0, 0.0, 0.0]))) / v620) * v1254;
                let v1260 = v1118 - v2;
                let v1261 = v1259 * v1260;
                let v4024 = ((Lanes([(v2559 * v1260), 0.0, 0.0, 0.0])) + (v2584 * v1259)) * v1262;
                let v1264 = v2 + v1262;
                let v1265 = (v1261 * v1262) / v1264;
                let v1266 = ((v500 * v1244) + (v1254 * v1256)) + v1265;
                let v4031 = ((Lanes([v3996[0], v3996[1], v3996[2], 0.0, 0.0])) + ((Lanes([v4013[0], v4013[1], v4013[2], 0.0, 0.0])) + (Lanes([v4014[0], 0.0, v4014[1], v4014[2], v4014[3]])))) + ((((Lanes([v4024[0], 0.0, v4024[1], v4024[2], v4024[3]])) + (v2607 * v1261)) - (v2607 * v1265)) / v1264);
                v1989 = v1266;
                v2605 = v4031;
            } else {
                let v1268 = if v1267 == v0 { 1.0 } else { 0.0 };
                let v1990: f64;
                let v2608: Lanes<5>;
                if v1268 != 0.0 {
                    let v1269 = v1243 - v2;
                    let v1270 = v500 * v1269;
                    let v3977 = (Lanes([(v3065 * v1269), 0.0, 0.0])) + (v2604 * v500);
                    let v3978 = Lanes([v3977[0], v3977[1], v3977[2], 0.0, 0.0]);
                    v1990 = v1270;
                    v2608 = v3978;
                } else {
                    let v1271 = v2 - v1267;
                    let v3955 = v2604 * v1271;
                    let v1276 = v1267 * ((v1243 + v1118) - v27);
                    let v1277 = v1109 / v620;
                    let v1278 = v2 + v1277;
                    let v3965 = ((v3792 - (Lanes([(v3205 * v1277), 0.0, 0.0, 0.0]))) / v620) * v1276;
                    let v1280 = (v1271 * (v1243 - v2)) + (v1276 * v1278);
                    let v1281 = v500 * v1280;
                    let v3973 = (Lanes([(v3065 * v1280), 0.0, 0.0, 0.0, 0.0])) + (((Lanes([v3955[0], v3955[1], v3955[2], 0.0, 0.0])) + (((((Lanes([v2604[0], v2604[1], v2604[2], 0.0, 0.0])) + (Lanes([v2584[0], 0.0, v2584[1], v2584[2], v2584[3]]))) * v1267) * v1278) + (Lanes([v3965[0], 0.0, v3965[1], v3965[2], v3965[3]])))) * v500);
                    v1990 = v1281;
                    v2608 = v3973;
                }
                v1989 = v1990;
                v2605 = v2608;
            }
            let v1282 = v727 * v109;
            let v4032 = v3282 * v109;
            let v4036 = (Lanes([0.0, v4032[0], v4032[1]])) + (Lanes([(v2698 * v727), 0.0, 0.0]));
            let v1283 = v1282 / v502;
            let v4037 = v4036 / v502;
            let v1284 = if v1283 < v763 { 1.0 } else { 0.0 };
            let v1298: f64;
            let v2609: Lanes<3>;
            if v1284 != 0.0 {
                let v1285 = v1283.exp();
                let v4039 = v4037 * v1285;
                v1298 = v1285;
                v2609 = v4039;
            } else {
                let v1286 = v763.exp();
                let v1289 = v1286 * (v2 + (v1283 - v763));
                let v4038 = v4037 * v1286;
                v1298 = v1289;
                v2609 = v4038;
            }
            let v2003: f64;
            let v2610: Lanes<3>;
            if v510 != 0.0 {
                let v1290 = v727 - v1224;
                let v1291 = v1290 * v109;
                let v4050 = (((Lanes([0.0, v3282[0], v3282[1]])) - (Lanes([v2549, 0.0, 0.0]))) * v109) + (Lanes([(v2698 * v1290), 0.0, 0.0]));
                let v1292 = if v1291 < v763 { 1.0 } else { 0.0 };
                let v1304: f64;
                let v2611: Lanes<3>;
                if v1292 != 0.0 {
                    let v1293 = v1291.exp();
                    let v4052 = v4050 * v1293;
                    v1304 = v1293;
                    v2611 = v4052;
                } else {
                    let v1294 = v763.exp();
                    let v1297 = v1294 * (v2 + (v1291 - v763));
                    let v4051 = v4050 * v1294;
                    v1304 = v1297;
                    v2611 = v4051;
                }
                let v1299 = v1298 - v2;
                let v1302 = v1301 * v27;
                let v1307 = (v2 + (v435 * v1304)).sqrt();
                let v1308 = v2 + v1307;
                let v1309 = (v1302 * v1299) / v1308;
                let v1310 = (v508 * v1299) + v1309;
                let v4069 = ((Lanes([(v3073 * v1299), 0.0, 0.0])) + (v2609 * v508)) + ((((Lanes([((v2560 * v27) * v1299), 0.0, 0.0])) + (v2609 * v1302)) - (((v2611 * v435) * (v2526 / (v3005 * v1307))) * v1309)) / v1308);
                v2003 = v1310;
                v2610 = v4069;
            } else {
                let v1311 = v1298 - v2;
                let v1312 = v508 * v1311;
                let v4043 = (Lanes([(v3073 * v1311), 0.0, 0.0])) + (v2609 * v508);
                v2003 = v1312;
                v2610 = v4043;
            }
            let v1313 = v770 / v463;
            let v4070 = v3357 / v463;
            let v1314 = if v1313 < v763 { 1.0 } else { 0.0 };
            let v1320: f64;
            let v2612: Lanes<3>;
            if v1314 != 0.0 {
                let v1315 = v1313.exp();
                let v4072 = v4070 * v1315;
                v1320 = v1315;
                v2612 = v4072;
            } else {
                let v1316 = v763.exp();
                let v1319 = v1316 * (v2 + (v1313 - v763));
                let v4071 = v4070 * v1316;
                v1320 = v1319;
                v2612 = v4071;
            }
            let v1321 = v1320 - v2;
            let v1322 = v474 * v1321;
            let v4076 = (Lanes([(v3046 * v1321), 0.0, 0.0])) + (v2612 * v474);
            let v1323 = v1282 / v543;
            let v4077 = v4036 / v543;
            let v1324 = if v1323 < v763 { 1.0 } else { 0.0 };
            let v1330: f64;
            let v2613: Lanes<3>;
            if v1324 != 0.0 {
                let v1325 = v1323.exp();
                let v4079 = v4077 * v1325;
                v1330 = v1325;
                v2613 = v4079;
            } else {
                let v1326 = v763.exp();
                let v1329 = v1326 * (v2 + (v1323 - v763));
                let v4078 = v4077 * v1326;
                v1330 = v1329;
                v2613 = v4078;
            }
            let v1331 = v1330 - v2;
            let v1332 = v551 * v1331;
            let v4083 = (Lanes([(v3100 * v1331), 0.0, 0.0])) + (v2613 * v551);
            let v1333 = v778 / v476;
            let v4084 = v3368 / v476;
            let v1334 = if v1333 < v763 { 1.0 } else { 0.0 };
            let v1340: f64;
            let v2614: Lanes<6>;
            if v1334 != 0.0 {
                let v1335 = v1333.exp();
                let v4086 = v4084 * v1335;
                v1340 = v1335;
                v2614 = v4086;
            } else {
                let v1336 = v763.exp();
                let v1339 = v1336 * (v2 + (v1333 - v763));
                let v4085 = v4084 * v1336;
                v1340 = v1339;
                v2614 = v4085;
            }
            let v1341 = v1340 - v2;
            let v1342 = v486 * v1341;
            let v4090 = (Lanes([(v3055 * v1341), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2614 * v486);
            let v1343 = v1282 / v553;
            let v4091 = v4036 / v553;
            let v1344 = if v1343 < v763 { 1.0 } else { 0.0 };
            let v1350: f64;
            let v2615: Lanes<3>;
            if v1344 != 0.0 {
                let v1345 = v1343.exp();
                let v4093 = v4091 * v1345;
                v1350 = v1345;
                v2615 = v4093;
            } else {
                let v1346 = v763.exp();
                let v1349 = v1346 * (v2 + (v1343 - v763));
                let v4092 = v4091 * v1346;
                v1350 = v1349;
                v2615 = v4092;
            }
            let v1351 = v1350 - v2;
            let v1352 = v560 * v1351;
            let v4097 = (Lanes([(v3108 * v1351), 0.0, 0.0])) + (v2615 * v560);
            let v1356 = if (if (if v582 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v573 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1173 != 0.0 { 1.0 } else { 0.0 };
            let v1994: f64;
            let v2616: Lanes<3>;
            if v1356 != 0.0 {
                let v1357 = v27 * v1053;
                let v1358 = v30 / v1357;
                let v1359 = v2 - v1358;
                let v1360 = v581 * v1359;
                let v4107 = (Lanes([(v3141 * v1359), 0.0, 0.0])) + ((((((v3705 * v27) * v1358) * v2687) / v1357) * v2687) * v581);
                let v1361 = if v1360 < v763 { 1.0 } else { 0.0 };
                let v1422: f64;
                let v2617: Lanes<3>;
                if v1361 != 0.0 {
                    let v1362 = v1360.exp();
                    let v4109 = v4107 * v1362;
                    v1422 = v1362;
                    v2617 = v4109;
                } else {
                    let v1363 = v763.exp();
                    let v1366 = v1363 * (v2 + (v1360 - v763));
                    let v4108 = v4107 * v1363;
                    v1422 = v1366;
                    v2617 = v4108;
                }
                let v1367 = v724 * v308;
                let v4110 = v3278 * v308;
                let v4114 = (Lanes([0.0, v4110[0], v4110[1]])) + (Lanes([(v2920 * v724), 0.0, 0.0]));
                let v4115 = v4114 * v1367;
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
                let v4141 = (v3278 * v30) * v581;
                let v1392 = v568 * v1389;
                let v1393 = (v1390 * v581) / v1392;
                let v4152 = (((Lanes([0.0, v4141[0], v4141[1]])) + (Lanes([(v3141 * v1390), 0.0, 0.0]))) - (((Lanes([(v2542 * v1389), 0.0, 0.0])) + (((((((v4115 + v4115) * (v2526 / (v3005 * v1371))) * (v1373 * (v1371.powf((v1373 - v2526))))) * v1386) + ((((((v4114 * v154) * v1378) * v2687) * v28) - (((((v4114 * v462) * v1367) + (v4114 * v1382)) * v1384) + (v4114 * v1383))) * v1374)) * v1388) * v568)) * v1393)) / v1392;
                let v1395 = if v1393 < v1394 { 1.0 } else { 0.0 };
                let v1419: f64;
                let v2618: Lanes<3>;
                if v1395 != 0.0 {
                    let v1396 = if v1393 < v763 { 1.0 } else { 0.0 };
                    let v1403: f64;
                    let v2619: Lanes<3>;
                    if v1396 != 0.0 {
                        let v1397 = v1393.exp();
                        let v4167 = v4152 * v1397;
                        v1403 = v1397;
                        v2619 = v4167;
                    } else {
                        let v1398 = v763.exp();
                        let v1401 = v1398 * (v2 + (v1393 - v763));
                        let v4166 = v4152 * v1398;
                        v1403 = v1401;
                        v2619 = v4166;
                    }
                    let v1402 = -v724;
                    let v1405 = (v2 - v1403) / v1393;
                    let v1406 = v2 + v1405;
                    let v1407 = v1402 * v1406;
                    let v4173 = (v3278 * v2687) * v1406;
                    let v4176 = (Lanes([0.0, v4173[0], v4173[1]])) + ((((v2619 * v2687) - (v4152 * v1405)) / v1393) * v1402);
                    v1419 = v1407;
                    v2618 = v4176;
                } else {
                    let v1408 = v724 * v424;
                    let v1409 = v1408 * v1393;
                    let v4154 = (v3278 * v424) * v1393;
                    let v1411 = v1393 * v1410;
                    let v1414 = v2 + (v1412 * v1393);
                    let v1416 = v2 + (v1411 * v1414);
                    let v1417 = v1409 * v1416;
                    let v4165 = (((Lanes([0.0, v4154[0], v4154[1]])) + (v4152 * v1408)) * v1416) + ((((v4152 * v1410) * v1414) + ((v4152 * v1412) * v1411)) * v1409);
                    v1419 = v1417;
                    v2618 = v4165;
                }
                let v1418 = v27 * v591;
                let v1420 = v1418 * v1419;
                let v1421 = v1420 * v1053;
                let v1423 = v1421 * v1422;
                let v1425 = (v1423 * v308) * v31;
                let v4192 = ((((((((Lanes([((v3158 * v27) * v1419), 0.0, 0.0])) + (v2618 * v1418)) * v1053) + (v3705 * v1420)) * v1422) + (v2617 * v1421)) * v308) + (Lanes([(v2920 * v1423), 0.0, 0.0]))) * v31;
                v1994 = v1425;
                v2616 = v4192;
            } else {
                v1994 = v0;
                v2616 = v4098;
            }
            let v1430 = if (if (if v606 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v597 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v718 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1709: f64;
            let v2620: Lanes<3>;
            if v1430 != 0.0 {
                let v1431 = v718 * v310;
                let v4193 = v3270 * v310;
                let v4197 = (Lanes([0.0, v4193[0], v4193[1]])) + (Lanes([(v2923 * v718), 0.0, 0.0]));
                let v1432 = v2 - v1431;
                let v1433 = v1432.powf(v1094);
                let v4201 = (v4197 * v2687) * (v1094 * (v1432.powf(v3761)));
                let v1434 = v27 * v1433;
                let v1435 = v62 / v1434;
                let v1436 = v2 - v1435;
                let v1437 = v605 * v1436;
                let v4210 = (Lanes([(v3182 * v1436), 0.0, 0.0])) + ((((((v4201 * v27) * v1435) * v2687) / v1434) * v2687) * v605);
                let v1438 = if v1437 < v763 { 1.0 } else { 0.0 };
                let v1494: f64;
                let v2621: Lanes<3>;
                if v1438 != 0.0 {
                    let v1439 = v1437.exp();
                    let v4212 = v4210 * v1439;
                    v1494 = v1439;
                    v2621 = v4212;
                } else {
                    let v1440 = v763.exp();
                    let v1443 = v1440 * (v2 + (v1437 - v763));
                    let v4211 = v4210 * v1440;
                    v1494 = v1443;
                    v2621 = v4211;
                }
                let v4213 = v4197 * v1431;
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
                let v4239 = (v3270 * v62) * v605;
                let v1466 = v592 * v1463;
                let v1467 = (v1464 * v605) / v1466;
                let v4250 = (((Lanes([0.0, v4239[0], v4239[1]])) + (Lanes([(v3182 * v1464), 0.0, 0.0]))) - (((Lanes([(v2543 * v1463), 0.0, 0.0])) + (((((((v4213 + v4213) * (v2526 / (v3005 * v1446))) * (v1448 * (v1446.powf((v1448 - v2526))))) * v1461) + ((((((v4197 * v154) * v1453) * v2687) * v60) - (((((v4197 * v462) * v1431) + (v4197 * v1457)) * v1459) + (v4197 * v1458))) * v1449)) * v1388) * v592)) * v1467)) / v1466;
                let v1469 = if v1467 < v1468 { 1.0 } else { 0.0 };
                let v1491: f64;
                let v2622: Lanes<3>;
                if v1469 != 0.0 {
                    let v1470 = if v1467 < v763 { 1.0 } else { 0.0 };
                    let v1477: f64;
                    let v2623: Lanes<3>;
                    if v1470 != 0.0 {
                        let v1471 = v1467.exp();
                        let v4265 = v4250 * v1471;
                        v1477 = v1471;
                        v2623 = v4265;
                    } else {
                        let v1472 = v763.exp();
                        let v1475 = v1472 * (v2 + (v1467 - v763));
                        let v4264 = v4250 * v1472;
                        v1477 = v1475;
                        v2623 = v4264;
                    }
                    let v1476 = -v718;
                    let v1479 = (v2 - v1477) / v1467;
                    let v1480 = v2 + v1479;
                    let v1481 = v1476 * v1480;
                    let v4271 = (v3270 * v2687) * v1480;
                    let v4274 = (Lanes([0.0, v4271[0], v4271[1]])) + ((((v2623 * v2687) - (v4250 * v1479)) / v1467) * v1476);
                    v1491 = v1481;
                    v2622 = v4274;
                } else {
                    let v1482 = v718 * v424;
                    let v1483 = v1482 * v1467;
                    let v4252 = (v3270 * v424) * v1467;
                    let v1484 = v1467 * v1410;
                    let v1486 = v2 + (v1412 * v1467);
                    let v1488 = v2 + (v1484 * v1486);
                    let v1489 = v1483 * v1488;
                    let v4263 = (((Lanes([0.0, v4252[0], v4252[1]])) + (v4250 * v1482)) * v1488) + ((((v4250 * v1410) * v1486) + ((v4250 * v1412) * v1484)) * v1483);
                    v1491 = v1489;
                    v2622 = v4263;
                }
                let v1490 = v27 * v615;
                let v1492 = v1490 * v1491;
                let v1493 = v1492 * v1433;
                let v1495 = v1493 * v1494;
                let v1497 = (v1495 * v310) * v63;
                let v4290 = ((((((((Lanes([((v3199 * v27) * v1491), 0.0, 0.0])) + (v2622 * v1490)) * v1433) + (v4201 * v1492)) * v1494) + (v2621 * v1493)) * v310) + (Lanes([(v2923 * v1495), 0.0, 0.0]))) * v63;
                v1709 = v1497;
                v2620 = v4290;
            } else {
                v1709 = v0;
                v2620 = v3454;
            }
            let v1499 = v1111 * v1498;
            let v4294 = (Lanes([(v3796 * v1498), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2570 * v1111);
            let v1501 = v435 * v1500;
            let v4295 = v2577 * v435;
            let v1504 = (v2 + v1499).sqrt();
            let v1505 = v2 + v1504;
            let v1506 = (v1499 - v1111) / v1505;
            let v4303 = ((v4294 - (Lanes([v3796, 0.0, 0.0, 0.0, 0.0, 0.0]))) - ((v4294 * (v2526 / (v3005 * v1504))) * v1506)) / v1505;
            let v1508 = (v2 + v1501).sqrt();
            let v1509 = v2 + v1508;
            let v1510 = v1501 / v1509;
            let v4309 = (v4295 - ((v4295 * (v2526 / (v3005 * v1508))) * v1510)) / v1509;
            let v1511 = v27 * v541;
            let v4310 = v3092 * v27;
            let v1512 = v1498 - v2;
            let v1515 = (v435 * v541) / v460;
            let v4318 = ((v3092 * v435) - (v3037 * v1515)) / v460;
            let v1518 = (v2 + (v1515 * v1498)).sqrt();
            let v1519 = v2 + v1518;
            let v1520 = (v1511 * v1512) / v1519;
            let v4328 = (((Lanes([(v4310 * v1512), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2570 * v1511)) - ((((Lanes([(v4318 * v1498), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2570 * v1515)) * (v2526 / (v3005 * v1518))) * v1520)) / v1519;
            let v1522 = if v1521 == v2 { 1.0 } else { 0.0 };
            let v1591: f64;
            let v2019: f64;
            let v2624: Lanes<7>;
            let v2625: Lanes<5>;
            if v1522 != 0.0 {
                let v1524 = v1523 * v27;
                let v1525 = v1524 * v633;
                let v1527 = v1007 - v1526;
                let v4366 = Lanes([0.0, v2568[0], v2568[1], 0.0, v2568[2]]);
                let v1529 = v633 / v646;
                let v1530 = v435 * v1529;
                let v4376 = ((v3217 - (v3226 * v1529)) / v646) * v435;
                let v4377 = v2573 * v1531;
                let v1533 = v1007 + (v1531 * v1526);
                let v1536 = (v2 + (v1530 * v1533)).sqrt();
                let v1537 = v2 + v1536;
                let v1538 = (v1525 * v1527) / v1537;
                let v4389 = (((Lanes([0.0, ((v3217 * v1524) * v1527), 0.0, 0.0, 0.0])) + ((v4366 - (Lanes([v2573[0], v2573[1], 0.0, v2573[2], 0.0]))) * v1525)) - ((((Lanes([0.0, (v4376 * v1533), 0.0, 0.0, 0.0])) + ((v4366 + (Lanes([v4377[0], v4377[1], 0.0, v4377[2], 0.0]))) * v1530)) * (v2526 / (v3005 * v1536))) * v1538)) / v1537;
                let v1540 = (v2 - v1523) * v27;
                let v1541 = v1540 * v633;
                let v1543 = v1498 - v1542;
                let v4391 = Lanes([0.0, v2570[0], v2570[1], v2570[2], v2570[3], v2570[4], v2570[5]]);
                let v4398 = v2575 * v1531;
                let v1546 = v1498 + (v1531 * v1542);
                let v1549 = (v2 + (v1530 * v1546)).sqrt();
                let v1550 = v2 + v1549;
                let v1551 = (v1541 * v1543) / v1550;
                let v4410 = (((Lanes([0.0, ((v3217 * v1540) * v1543), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4391 - (Lanes([v2575[0], v2575[1], 0.0, 0.0, v2575[2], 0.0, v2575[3]]))) * v1541)) - ((((Lanes([0.0, (v4376 * v1546), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4391 + (Lanes([v4398[0], v4398[1], 0.0, 0.0, v4398[2], 0.0, v4398[3]]))) * v1530)) * (v2526 / (v3005 * v1549))) * v1551)) / v1550;
                v1591 = v1551;
                v2019 = v1538;
                v2624 = v4410;
                v2625 = v4389;
            } else {
                let v1552 = v1523 * v27;
                let v1553 = v1552 * v633;
                let v1554 = v1007 - v2;
                let v1556 = v633 / v646;
                let v1557 = v435 * v1556;
                let v4337 = ((v3217 - (v3226 * v1556)) / v646) * v435;
                let v1560 = (v2 + (v1557 * v1007)).sqrt();
                let v1561 = v2 + v1560;
                let v1562 = (v1553 * v1554) / v1561;
                let v4347 = (((Lanes([((v3217 * v1552) * v1554), 0.0, 0.0])) + (v2568 * v1553)) - ((((Lanes([(v4337 * v1007), 0.0, 0.0])) + (v2568 * v1557)) * (v2526 / (v3005 * v1560))) * v1562)) / v1561;
                let v1564 = (v2 - v1523) * v27;
                let v1565 = v1564 * v633;
                let v1569 = (v2 + (v1557 * v1498)).sqrt();
                let v1570 = v2 + v1569;
                let v1571 = (v1565 * v1512) / v1570;
                let v4362 = (((Lanes([((v3217 * v1564) * v1512), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2570 * v1565)) - ((((Lanes([(v4337 * v1498), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2570 * v1557)) * (v2526 / (v3005 * v1569))) * v1571)) / v1570;
                let v4363 = Lanes([0.0, v4362[0], v4362[1], v4362[2], v4362[3], v4362[4], v4362[5]]);
                let v4364 = Lanes([0.0, v4347[0], v4347[1], 0.0, v4347[2]]);
                v1591 = v1571;
                v2019 = v1562;
                v2624 = v4363;
                v2625 = v4364;
            }
            let v1572 = v27 * v641;
            let v1573 = v1526 - v2;
            let v1575 = v1531 * v435;
            let v1576 = v641 / v651;
            let v1577 = v1575 * v1576;
            let v1580 = (v2 + (v1577 * v1526)).sqrt();
            let v1581 = v2 + v1580;
            let v1582 = (v1572 * v1573) / v1581;
            let v4430 = v3290 * v1583;
            let v1585 = v1582 + (v732 * v1583);
            let v4432 = ((((Lanes([0.0, ((v3223 * v27) * v1573), 0.0])) + (v2573 * v1572)) - ((((Lanes([0.0, ((((v3223 - (v3229 * v1576)) / v651) * v1575) * v1526), 0.0])) + (v2573 * v1577)) * (v2526 / (v3005 * v1580))) * v1582)) / v1581) + (Lanes([v4430[0], 0.0, v4430[1]]));
            let v1588 = if v8 > v0 { 1.0 } else { 0.0 };
            let v1589 = if (if v1586 > v0 { 1.0 } else { 0.0 }) != 0.0 && v1588 != 0.0 { 1.0 } else { 0.0 };
            let v1713: f64;
            let v1716: f64;
            let v2015: f64;
            let v2023: f64;
            let v2254: f64;
            let v2626: Lanes<6>;
            let v2627: Lanes<10>;
            let v2628: Lanes<7>;
            let v2629: Lanes<10>;
            let v2630: Lanes<10>;
            if v1589 != 0.0 {
                let v1590 = v1520 * v9;
                let v4434 = v4328 * v9;
                let v1592 = v1591 * v9;
                let v4435 = v2624 * v9;
                let v1593 = v8 * v27;
                let v1594 = v1593 * v541;
                let v1596 = v1595 - v2;
                let v1600 = (v2 + (v1515 * v1595)).sqrt();
                let v1601 = v2 + v1600;
                let v1602 = (v1594 * v1596) / v1601;
                let v4450 = (((Lanes([0.0, 0.0, ((v3092 * v1593) * v1596), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2572 * v1594)) - ((((Lanes([0.0, 0.0, (v4318 * v1595), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2572 * v1515)) * (v2526 / (v3005 * v1600))) * v1602)) / v1601;
                let v1653: f64;
                let v2631: Lanes<10>;
                if v1522 != 0.0 {
                    let v1605 = ((v2 - v1523) * v8) * v27;
                    let v1606 = v1605 * v633;
                    let v1608 = v1595 - v1607;
                    let v4472 = Lanes([v2572[0], v2572[1], 0.0, v2572[2], v2572[3], v2572[4], v2572[5], v2572[6], v2572[7], v2572[8]]);
                    let v1611 = (v435 * v633) / v646;
                    let v4483 = v2574 * v1531;
                    let v1613 = v1595 + (v1531 * v1607);
                    let v1616 = (v2 + (v1611 * v1613)).sqrt();
                    let v1617 = v2 + v1616;
                    let v1618 = (v1606 * v1608) / v1617;
                    let v4495 = (((Lanes([0.0, 0.0, 0.0, ((v3217 * v1605) * v1608), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4472 - (Lanes([0.0, 0.0, v2574[0], v2574[1], 0.0, 0.0, v2574[2], 0.0, v2574[3], v2574[4]]))) * v1606)) - ((((Lanes([0.0, 0.0, 0.0, ((((v3217 * v435) - (v3226 * v1611)) / v646) * v1613), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v4472 + (Lanes([0.0, 0.0, v4483[0], v4483[1], 0.0, 0.0, v4483[2], 0.0, v4483[3], v4483[4]]))) * v1611)) * (v2526 / (v3005 * v1616))) * v1618)) / v1617;
                    v1653 = v1618;
                    v2631 = v4495;
                } else {
                    let v1621 = ((v2 - v1523) * v8) * v27;
                    let v1622 = v1621 * v633;
                    let v1625 = (v435 * v633) / v646;
                    let v1628 = (v2 + (v1625 * v1595)).sqrt();
                    let v1629 = v2 + v1628;
                    let v1630 = (v1622 * v1596) / v1629;
                    let v4469 = (((Lanes([0.0, 0.0, ((v3217 * v1621) * v1596), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2572 * v1622)) - ((((Lanes([0.0, 0.0, ((((v3217 * v435) - (v3226 * v1625)) / v646) * v1595), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2572 * v1625)) * (v2526 / (v3005 * v1628))) * v1630)) / v1629;
                    let v4470 = Lanes([v4469[0], v4469[1], 0.0, v4469[2], v4469[3], v4469[4], v4469[5], v4469[6], v4469[7], v4469[8]]);
                    v1653 = v1630;
                    v2631 = v4470;
                }
                let v1631 = if v1586 == v2 { 1.0 } else { 0.0 };
                let v1659: f64;
                let v2632: Lanes<10>;
                if v1631 != 0.0 {
                    let v1633 = v8 * (v541 + v633);
                    let v1634 = v1633 * v357;
                    let v4500 = (((v3092 + v3217) * v8) * v357) + (v2967 * v1633);
                    let v1635 = v1634 * v109;
                    let v1637 = v27 - (v1635.ln());
                    let v1639 = v759 - (v107 * v1637);
                    let v4511 = v3406 - (Lanes([0.0, 0.0, ((v2695 * v1637) + (((((v4500 * v109) + (v2698 * v1634)) * (v2526 / v1635)) * v2687) * v107)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
                    let v1641 = v1639 * v1639;
                    let v4512 = v4511 * v1639;
                    let v4513 = v4512 + v4512;
                    let v1642 = if v1639 < v0 { 1.0 } else { 0.0 };
                    let v1652: f64;
                    let v2633: Lanes<9>;
                    if v1642 != 0.0 {
                        let v1645 = (v1641 + v1640).sqrt();
                        let v1646 = v1645 - v1639;
                        let v1647 = v1643 / v1646;
                        let v4525 = ((((v4513 * (v2526 / (v3005 * v1645))) - v4511) * v1647) * v2687) / v1646;
                        v1652 = v1647;
                        v2633 = v4525;
                    } else {
                        let v1649 = (v1641 + v1640).sqrt();
                        let v1651 = v424 * (v1649 + v1639);
                        let v4518 = ((v4513 * (v2526 / (v3005 * v1649))) + v4511) * v424;
                        v1652 = v1651;
                        v2633 = v4518;
                    }
                    let v1654 = v1602 + v1653;
                    let v1657 = (v1634 + (v1654 * v357)) + v1652;
                    let v4534 = Lanes([v2633[0], v2633[1], 0.0, v2633[2], v2633[3], v2633[4], v2633[5], v2633[6], v2633[7], v2633[8]]);
                    let v1658 = v1652 / v1657;
                    let v4538 = (v4534 - ((((Lanes([0.0, 0.0, 0.0, v4500, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([v4450[0], v4450[1], 0.0, v4450[2], v4450[3], v4450[4], v4450[5], v4450[6], v4450[7], v4450[8]])) + v2631) * v357) + (Lanes([0.0, 0.0, 0.0, (v2967 * v1654), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])))) + v4534) * v1658)) / v1657;
                    v1659 = v1658;
                    v2632 = v4538;
                } else {
                    v1659 = v2;
                    v2632 = v4433;
                }
                let v1660 = v1659 * v1602;
                let v4540 = v4450 * v1659;
                let v4542 = (v2632 * v1602) + (Lanes([v4540[0], v4540[1], 0.0, v4540[2], v4540[3], v4540[4], v4540[5], v4540[6], v4540[7], v4540[8]]));
                let v1661 = v1659 * v1653;
                let v4545 = (v2632 * v1653) + (v2631 * v1659);
                v1713 = v1590;
                v1716 = v1660;
                v2015 = v1592;
                v2023 = v1661;
                v2254 = v1659;
                v2626 = v4434;
                v2627 = v4542;
                v2628 = v4435;
                v2629 = v4545;
                v2630 = v2632;
            } else {
                v1713 = v1520;
                v1716 = v0;
                v2015 = v1591;
                v2023 = v0;
                v2254 = v2;
                v2626 = v4328;
                v2627 = v4433;
                v2628 = v2624;
                v2629 = v4433;
                v2630 = v4433;
            }
            let v1663 = if v1662 == v2 { 1.0 } else { 0.0 };
            let v1710: f64;
            let v2634: Lanes<3>;
            if v1663 != 0.0 {
                let v1664 = v729 + v718;
                let v4549 = (Lanes([v3286[0], v3286[1], 0.0])) + (Lanes([0.0, v3270[0], v3270[1]]));
                let v1670 = (v1667 * v1664) * v1669;
                let v1671 = v1670 * v1664;
                let v4554 = (((v4549 * v1667) * v1669) * v1664) + (v4549 * v1670);
                let v1674 = if (v1672 * v1664) < v0 { 1.0 } else { 0.0 };
                let v1700: f64;
                let v2635: Lanes<3>;
                if v1674 != 0.0 {
                    let v1677 = (v1671 + v1666).sqrt();
                    let v1680 = v1677 - (v1678 * v1664);
                    let v1681 = v1675 / v1680;
                    let v4568 = ((((v4554 * (v2526 / (v3005 * v1677))) - (v4549 * v1678)) * v1681) * v2687) / v1680;
                    v1700 = v1681;
                    v2635 = v4568;
                } else {
                    let v1683 = (v1671 + v1666).sqrt();
                    let v1687 = v424 * (v1683 + (v1684 * v1664));
                    let v4560 = ((v4554 * (v2526 / (v3005 * v1683))) + (v4549 * v1684)) * v424;
                    v1700 = v1687;
                    v2635 = v4560;
                }
                let v1691 = v2 / (v2 - (v90.powf(v1688)));
                let v1693 = v90 * v1692;
                let v1699 = (((v1691 * v1691) * (v90.powf((v1688 - v2)))) * v1688) / v1692;
                let v1701 = if v1700 < v1693 { 1.0 } else { 0.0 };
                let v1711: f64;
                let v2636: Lanes<3>;
                if v1701 != 0.0 {
                    let v1702 = v1700 / v1692;
                    let v1704 = v2 - (v1702.powf(v1688));
                    let v1705 = v2 / v1704;
                    let v4578 = (((((v2635 / v1692) * (v1688 * (v1702.powf((v1688 - v2526))))) * v2687) * v1705) * v2687) / v1704;
                    v1711 = v1705;
                    v2636 = v4578;
                } else {
                    let v4569 = v2635 * v1699;
                    let v1708 = v1691 + ((v1700 - v1693) * v1699);
                    v1711 = v1708;
                    v2636 = v4569;
                }
                v1710 = v1711;
                v2634 = v2636;
            } else {
                v1710 = v2;
                v2634 = v4546;
            }
            let v1712 = v1709 * v1710;
            let v4579 = v2620 * v1710;
            let v4580 = v2634 * v1709;
            let v4583 = (Lanes([v4579[0], 0.0, v4579[1], v4579[2]])) + (Lanes([0.0, v4580[0], v4580[1], v4580[2]]));
            let v1714 = v1713 * v1710;
            let v4585 = v2634 * v1713;
            let v4587 = (v2626 * v1710) + (Lanes([0.0, v4585[0], v4585[1], v4585[2], 0.0, 0.0]));
            let v1715 = v1342 * v1710;
            let v4589 = v2634 * v1342;
            let v4591 = (v4090 * v1710) + (Lanes([0.0, v4589[0], v4589[1], v4589[2], 0.0, 0.0]));
            let v1717 = v1716 * v1710;
            let v4593 = v2634 * v1716;
            let v4595 = (v2627 * v1710) + (Lanes([0.0, 0.0, 0.0, 0.0, v4593[0], v4593[1], v4593[2], 0.0, 0.0, 0.0]));
            let v1718 = v1059 / v623;
            let v4599 = (v3714 - (Lanes([(v3209 * v1718), 0.0, 0.0]))) / v623;
            let v1720 = v1109 / v620;
            let v4603 = (v3792 - (Lanes([(v3205 * v1720), 0.0, 0.0, 0.0]))) / v620;
            let v1721 = (v2 + v1718) + v1720;
            let v4606 = (Lanes([v4599[0], v4599[1], v4599[2], 0.0, 0.0])) + (Lanes([v4603[0], 0.0, v4603[1], v4603[2], v4603[3]]));
            let v1723 = v1721 * v1721;
            let v4607 = v4606 * v1721;
            let v4608 = v4607 + v4607;
            let v1724 = if v1721 < v0 { 1.0 } else { 0.0 };
            let v1734: f64;
            let v2637: Lanes<5>;
            if v1724 != 0.0 {
                let v1727 = (v1723 + v1722).sqrt();
                let v1728 = v1727 - v1721;
                let v1729 = v1725 / v1728;
                let v4620 = ((((v4608 * (v2526 / (v3005 * v1727))) - v4606) * v1729) * v2687) / v1728;
                v1734 = v1729;
                v2637 = v4620;
            } else {
                let v1731 = (v1723 + v1722).sqrt();
                let v1733 = v424 * (v1731 + v1721);
                let v4613 = ((v4608 * (v2526 / (v3005 * v1731))) + v4606) * v424;
                v1734 = v1733;
                v2637 = v4613;
            }
            let v1735 = v1734 * v1163;
            let v1736 = v346 / v1735;
            let v4627 = ((Lanes([v2961, 0.0, 0.0, 0.0, 0.0])) - (((v2637 * v1163) + (v3895 * v1734)) * v1736)) / v1735;
            let v1737 = if v1736 < v22 { 1.0 } else { 0.0 };
            let v1738: f64;
            let v2638: Lanes<5>;
            if v1737 != 0.0 {
                v1738 = v22;
                v2638 = v4628;
            } else {
                v1738 = v1736;
                v2638 = v4627;
            }
            let v1739 = v154 * v1738;
            let v4629 = v2638 * v154;
            let v1740 = v27 * v107;
            let v1742 = v1741 - v2;
            let v4636 = ((Lanes([((v2695 * v27) * v1742), 0.0, 0.0])) + (v2571 * v1740)) + (Lanes([0.0, v3286[0], v3286[1]]));
            let v1745 = ((v1740 * v1742) + v729) / v1739;
            let v4637 = v4629 * v1745;
            let v4641 = ((Lanes([v4636[0], 0.0, v4636[1], v4636[2], 0.0, 0.0])) - (Lanes([v4637[0], v4637[1], 0.0, v4637[2], v4637[3], v4637[4]]))) / v1739;
            let v1746 = if v1170 > v0 { 1.0 } else { 0.0 };
            let v1960: f64;
            let v2639: Lanes<5>;
            if v1746 != 0.0 {
                let v1748 = if v1747 == v2 { 1.0 } else { 0.0 };
                let v1910: f64;
                let v2640: Lanes<5>;
                if v1748 != 0.0 {
                    let v1750 = if v718 < v1749 { 1.0 } else { 0.0 };
                    let v1911: f64;
                    let v2641: Lanes<5>;
                    if v1750 != 0.0 {
                        let v1753 = (-v1170) / v1752;
                        let v4820 = (v3913 * v2687) / v1752;
                        let v1754 = if v1753 < v763 { 1.0 } else { 0.0 };
                        let v1761: f64;
                        let v2642: Lanes<5>;
                        if v1754 != 0.0 {
                            let v1755 = v1753.exp();
                            let v4822 = v4820 * v1755;
                            v1761 = v1755;
                            v2642 = v4822;
                        } else {
                            let v1756 = v763.exp();
                            let v1759 = v1756 * (v2 + (v1753 - v763));
                            let v4821 = v4820 * v1756;
                            v1761 = v1759;
                            v2642 = v4821;
                        }
                        let v1760 = v1749 - v718;
                        let v1762 = v1760 * v1761;
                        let v4824 = (v3270 * v2687) * v1761;
                        let v4827 = (Lanes([0.0, 0.0, v4824[0], v4824[1], 0.0])) + (v2642 * v1760);
                        let v1764 = -v1763;
                        let v1766 = v1762.powf(v1765);
                        let v1767 = v1764 * v1766;
                        let v4836 = (Lanes([((v2557 * v2687) * v1766), 0.0, 0.0, 0.0, 0.0])) + ((v4827 * (v1765 * (v1762.powf((v1765 - v2526))))) * v1764);
                        let v1768 = if v1767 < v763 { 1.0 } else { 0.0 };
                        let v1777: f64;
                        let v2643: Lanes<5>;
                        if v1768 != 0.0 {
                            let v1769 = v1767.exp();
                            let v4838 = v4836 * v1769;
                            v1777 = v1769;
                            v2643 = v4838;
                        } else {
                            let v1770 = v763.exp();
                            let v1773 = v1770 * (v2 + (v1767 - v763));
                            let v4837 = v4836 * v1770;
                            v1777 = v1773;
                            v2643 = v4837;
                        }
                        let v1775 = v1774 / v1763;
                        let v1776 = v1775 * v1762;
                        let v1778 = v1776 * v1777;
                        let v4848 = (((Lanes([((((v2557 * v1775) * v2687) / v1763) * v1762), 0.0, 0.0, 0.0, 0.0])) + (v4827 * v1775)) * v1777) + (v2643 * v1776);
                        v1911 = v1778;
                        v2641 = v4848;
                    } else {
                        v1911 = v0;
                        v2641 = v4628;
                    }
                    v1910 = v1911;
                    v2640 = v2641;
                } else {
                    let v1779 = if v1747 == v27 { 1.0 } else { 0.0 };
                    let v1912: f64;
                    let v2644: Lanes<5>;
                    if v1779 != 0.0 {
                        let v1780 = if v718 < v820 { 1.0 } else { 0.0 };
                        let v1913: f64;
                        let v2645: Lanes<5>;
                        if v1780 != 0.0 {
                            let v1785 = (v27 * v1781) / (v1783 * v1783);
                            let v1786 = v820 - v718;
                            let v4697 = v3434 - v3433;
                            let v1787 = v1786 / v1091;
                            let v4699 = Lanes([v4697[0], v4697[1], v4697[2], 0.0]);
                            let v1790 = ((v27 * v1787) / v1785).sqrt();
                            let v4706 = ((((v4699 - (v2583 * v1787)) / v1091) * v27) / v1785) * (v2526 / (v3005 * v1790));
                            let v1792 = if v1791 == v0 { 1.0 } else { 0.0 };
                            let v1799: f64;
                            let v2646: Lanes<4>;
                            if v1792 != 0.0 {
                                v1799 = v1783;
                                v2646 = v3500;
                            } else {
                                let v1796 = v2 - (v424 * v1793);
                                let v4708 = (v2585 * v424) * v2687;
                                let v1797 = v1783 * v1796;
                                let v1798 = v1797 * v1796;
                                let v4712 = ((v4708 * v1783) * v1796) + (v4708 * v1797);
                                v1799 = v1798;
                                v2646 = v4712;
                            }
                            let v4716 = v4706 * v1790;
                            let v4718 = v2646 * v1799;
                            let v1804 = ((v1790 * v1790) + (v1799 * v1799)).sqrt();
                            let v1805 = (v1790 * v1799) / v1804;
                            let v4726 = (((v4706 * v1799) + (v2646 * v1790)) - ((((v4716 + v4716) + (v4718 + v4718)) * (v2526 / (v3005 * v1804))) * v1805)) / v1804;
                            let v1806 = v1786 / v1805;
                            let v4729 = (v4699 - (v4726 * v1806)) / v1805;
                            let v1807 = v424 * v1805;
                            let v4730 = v4726 * v424;
                            let v1808 = v1807 * v1785;
                            let v4731 = v4730 * v1785;
                            let v1810 = v1806 + (v1808 * v1091);
                            let v4735 = v4729 + ((v4731 * v1091) + (v2583 * v1808));
                            let v1837: f64;
                            let v2647: Lanes<5>;
                            if v1792 != 0.0 {
                                let v4770 = Lanes([v4735[0], 0.0, v4735[1], v4735[2], v4735[3]]);
                                v1837 = v1810;
                                v2647 = v4770;
                            } else {
                                let v1812 = v27 * v1811;
                                let v1820 = v906 * (v2 + (v1812 * (v2 + (v27 * v1793))));
                                let v1821 = v1170 / v1820;
                                let v4739 = (((v2585 * v27) * v1812) * v906) * v1821;
                                let v1822 = ((v2 + v1811) / (v2 + v1812)) - v1821;
                                let v4744 = v4731 * v1822;
                                let v1824 = v1806 - (v1808 * v1822);
                                let v4749 = (Lanes([v4729[0], 0.0, v4729[1], v4729[2], v4729[3]])) - ((Lanes([v4744[0], 0.0, v4744[1], v4744[2], v4744[3]])) + ((((v3913 - (Lanes([v4739[0], 0.0, v4739[1], v4739[2], v4739[3]]))) / v1820) * v2687) * v1808));
                                let v1825 = v1824 - v1810;
                                let v4750 = Lanes([v4735[0], 0.0, v4735[1], v4735[2], v4735[3]]);
                                let v4752 = (v4749 - v4750) * v1825;
                                let v1827 = v42 * v1806;
                                let v1828 = v1827 * v1806;
                                let v4761 = (((((v4729 * v42) * v1806) + (v4729 * v1827)) * v1829) + (v2586 * v1828)) / v906;
                                let v1834 = ((v1825 * v1825) + ((v1828 * v1829) / v906)).sqrt();
                                let v1836 = v424 * ((v1824 + v1810) + v1834);
                                let v4769 = ((v4749 + v4750) + (((v4752 + v4752) + (Lanes([v4761[0], 0.0, v4761[1], v4761[2], v4761[3]]))) * (v2526 / (v3005 * v1834)))) * v424;
                                v1837 = v1836;
                                v2647 = v4769;
                            }
                            let v1839 = (v1837 - v1806) / v1837;
                            let v4775 = ((v2647 - (Lanes([v4729[0], 0.0, v4729[1], v4729[2], v4729[3]]))) - (v2647 * v1839)) / v1837;
                            let v1842 = if (v1839.abs()) > v1841 { 1.0 } else { 0.0 };
                            let v1914: f64;
                            let v2648: Lanes<5>;
                            if v1842 != 0.0 {
                                let v1843 = v1807 / v1839;
                                let v4790 = ((Lanes([v4730[0], 0.0, v4730[1], v4730[2], v4730[3]])) - (v4775 * v1843)) / v1839;
                                let v1846 = v1844 / v1845;
                                let v1847 = v1846 * v1837;
                                let v1848 = v1847 * v1843;
                                let v1850 = (-v1845) / v1837;
                                let v4805 = ((Lanes([(v2561 * v2687), 0.0, 0.0, 0.0, 0.0])) - (v2647 * v1850)) / v1837;
                                let v1851 = v1850.exp();
                                let v1852 = v1799 / v1843;
                                let v1853 = v2 + v1852;
                                let v1855 = (v1850 * v1853).exp();
                                let v1856 = v1851 - v1855;
                                let v1857 = v1848 * v1856;
                                let v4818 = (((((Lanes([((((v2561 * v1846) * v2687) / v1845) * v1837), 0.0, 0.0, 0.0, 0.0])) + (v2647 * v1846)) * v1843) + (v4790 * v1847)) * v1856) + (((v4805 * v1851) - (((v4805 * v1853) + ((((Lanes([v2646[0], 0.0, v2646[1], v2646[2], v2646[3]])) - (v4790 * v1852)) / v1843) * v1850)) * v1855)) * v1848);
                                v1914 = v1857;
                                v2648 = v4818;
                            } else {
                                let v1858 = v1844 * v1799;
                                let v1860 = (-v1845) / v1837;
                                let v1861 = v1860.exp();
                                let v1862 = v1858 * v1861;
                                let v4783 = (v2646 * v1844) * v1861;
                                let v4786 = (Lanes([v4783[0], 0.0, v4783[1], v4783[2], v4783[3]])) + (((((Lanes([(v2561 * v2687), 0.0, 0.0, 0.0, 0.0])) - (v2647 * v1860)) / v1837) * v1861) * v1858);
                                v1914 = v1862;
                                v2648 = v4786;
                            }
                            v1913 = v1914;
                            v2645 = v2648;
                        } else {
                            v1913 = v0;
                            v2645 = v4628;
                        }
                        v1912 = v1913;
                        v2644 = v2645;
                    } else {
                        let v1863 = if v1747 == v154 { 1.0 } else { 0.0 };
                        let v1915: f64;
                        let v2649: Lanes<5>;
                        if v1863 != 0.0 {
                            let v1864 = if v718 < v1749 { 1.0 } else { 0.0 };
                            let v1916: f64;
                            let v2650: Lanes<5>;
                            if v1864 != 0.0 {
                                let v1865 = v1749 - v718;
                                let v4642 = v3270 * v2687;
                                let v1866 = v1865.powf(v1765);
                                let v1868 = v1867 + v1170;
                                let v1869 = v1170 / v1868;
                                let v1870 = v2 - v1869;
                                let v1872 = v1870.powf(v1871);
                                let v1873 = v1866 * v1872;
                                let v4655 = (v4642 * (v1765 * (v1865.powf((v1765 - v2526))))) * v1872;
                                let v4658 = (Lanes([0.0, 0.0, v4655[0], v4655[1], 0.0])) + (((((v3913 - (v3913 * v1869)) / v1868) * v2687) * (v1871 * (v1870.powf((v1871 - v2526))))) * v1866);
                                let v1874 = if v1791 == v0 { 1.0 } else { 0.0 };
                                let v1898: f64;
                                let v2651: Lanes<5>;
                                if v1874 != 0.0 {
                                    v1898 = v1873;
                                    v2651 = v4658;
                                } else {
                                    let v1877 = (v1170 - v1875) / v1867;
                                    let v4659 = v3913 / v1867;
                                    let v1880 = (v1877 - v2) / v1879;
                                    let v4660 = v4659 / v1879;
                                    let v1881 = if v1877 < v2 { 1.0 } else { 0.0 };
                                    let v1893: f64;
                                    let v2652: Lanes<5>;
                                    if v1881 != 0.0 {
                                        let v1882 = v1880.exp();
                                        let v1883 = v2 + v1882;
                                        let v4670 = ((v4660 * v1882) * (v2526 / v1883)) * v1879;
                                        let v1886 = v2 + (v1879 * (v1883.ln()));
                                        v1893 = v1886;
                                        v2652 = v4670;
                                    } else {
                                        let v1888 = (-v1880).exp();
                                        let v1889 = v2 + v1888;
                                        let v1892 = v1877 + (v1879 * (v1889.ln()));
                                        let v4666 = v4659 + ((((v4660 * v2687) * v1888) * (v2526 / v1889)) * v1879);
                                        v1893 = v1892;
                                        v2652 = v4666;
                                    }
                                    let v1895 = v1893.powf(v1894);
                                    let v1896 = v1873 * v1895;
                                    let v4677 = (v4658 * v1895) + ((v2652 * (v1894 * (v1893.powf((v1894 - v2526))))) * v1873);
                                    v1898 = v1896;
                                    v2651 = v4677;
                                }
                                let v1897 = -v1763;
                                let v1899 = v1897 * v1898;
                                let v4682 = (Lanes([((v2557 * v2687) * v1898), 0.0, 0.0, 0.0, 0.0])) + (v2651 * v1897);
                                let v1900 = if v1899 < v763 { 1.0 } else { 0.0 };
                                let v1908: f64;
                                let v2653: Lanes<5>;
                                if v1900 != 0.0 {
                                    let v1901 = v1899.exp();
                                    let v4684 = v4682 * v1901;
                                    v1908 = v1901;
                                    v2653 = v4684;
                                } else {
                                    let v1902 = v763.exp();
                                    let v1905 = v1902 * (v2 + (v1899 - v763));
                                    let v4683 = v4682 * v1902;
                                    v1908 = v1905;
                                    v2653 = v4683;
                                }
                                let v1906 = v1774 / v1763;
                                let v1907 = v1906 * v1865;
                                let v4689 = v4642 * v1906;
                                let v1909 = v1907 * v1908;
                                let v4693 = ((Lanes([((((v2557 * v1906) * v2687) / v1763) * v1865), 0.0, 0.0])) + (Lanes([0.0, v4689[0], v4689[1]]))) * v1908;
                                let v4696 = (Lanes([v4693[0], 0.0, v4693[1], v4693[2], 0.0])) + (v2653 * v1907);
                                v1916 = v1909;
                                v2650 = v4696;
                            } else {
                                v1916 = v0;
                                v2650 = v4628;
                            }
                            v1915 = v1916;
                            v2649 = v2650;
                        } else {
                            v1915 = v0;
                            v2649 = v4628;
                        }
                        v1912 = v1915;
                        v2644 = v2649;
                    }
                    v1910 = v1912;
                    v2640 = v2644;
                }
                let v1917 = if v1910 > v0 { 1.0 } else { 0.0 };
                let v1961: f64;
                let v2654: Lanes<5>;
                if v1917 != 0.0 {
                    let v1919 = if v1918 == v2 { 1.0 } else { 0.0 };
                    let v1962: f64;
                    let v2655: Lanes<5>;
                    if v1919 != 0.0 {
                        let v1921 = v1920 + v1739;
                        let v4853 = (Lanes([v2552, 0.0, 0.0, 0.0, 0.0])) + v4629;
                        let v1922 = v1170 * v1921;
                        let v1923 = v107 / v1922;
                        let v1924 = v1164 / v449;
                        let v1928 = v1927 / v1921;
                        let v1929 = (v1923 + (v1924 * v500)) + v1928;
                        let v4874 = ((((Lanes([v2695, 0.0, 0.0, 0.0, 0.0])) - (((v3913 * v1921) + (v4853 * v1170)) * v1923)) / v1922) + ((((v3898 - (Lanes([(v3031 * v1924), 0.0, 0.0, 0.0, 0.0]))) / v449) * v500) + (Lanes([(v3065 * v1924), 0.0, 0.0, 0.0, 0.0])))) + (((Lanes([v2551, 0.0, 0.0, 0.0, 0.0])) - (v4853 * v1928)) / v1921);
                        let v1930 = if v1747 == v154 { 1.0 } else { 0.0 };
                        let v1963: f64;
                        let v2656: Lanes<5>;
                        if v1930 != 0.0 {
                            let v1932 = (v1910 - v1929) / v1665;
                            let v4886 = (v2640 - v4874) / v1665;
                            let v1933 = if v1910 < v1929 { 1.0 } else { 0.0 };
                            let v1945: f64;
                            let v2657: Lanes<5>;
                            if v1933 != 0.0 {
                                let v1934 = v1932.exp();
                                let v1935 = v2 + v1934;
                                let v1938 = v1910 - (v1665 * (v1935.ln()));
                                let v4897 = v2640 - (((v4886 * v1934) * (v2526 / v1935)) * v1665);
                                v1945 = v1938;
                                v2657 = v4897;
                            } else {
                                let v1940 = (-v1932).exp();
                                let v1941 = v2 + v1940;
                                let v1944 = v1929 - (v1665 * (v1941.ln()));
                                let v4892 = v4874 - ((((v4886 * v2687) * v1940) * (v2526 / v1941)) * v1665);
                                v1945 = v1944;
                                v2657 = v4892;
                            }
                            let v1946 = v1170 * v1945;
                            let v4900 = (v3913 * v1945) + (v2657 * v1170);
                            v1963 = v1946;
                            v2656 = v4900;
                        } else {
                            let v1947 = v1170 * v1910;
                            let v1949 = v1910 + v1929;
                            let v1950 = (v1947 * v1929) / v1949;
                            let v4884 = (((((v3913 * v1910) + (v2640 * v1170)) * v1929) + (v4874 * v1947)) - ((v2640 + v4874) * v1950)) / v1949;
                            v1963 = v1950;
                            v2656 = v4884;
                        }
                        v1962 = v1963;
                        v2655 = v2656;
                    } else {
                        let v1951 = v1170 * v1910;
                        let v4851 = (v3913 * v1910) + (v2640 * v1170);
                        v1962 = v1951;
                        v2655 = v4851;
                    }
                    v1961 = v1962;
                    v2654 = v2655;
                } else {
                    v1961 = v0;
                    v2654 = v4628;
                }
                v1960 = v1961;
                v2639 = v2654;
            } else {
                v1960 = v0;
                v2639 = v4628;
            }
            let v1952 = if v1118 > v0 { 1.0 } else { 0.0 };
            let v1954: f64;
            let v2658: Lanes<4>;
            if v1952 != 0.0 {
                let v1953 = v107 * v3814;
                let v4907 = (Lanes([(v2695 * v3814), 0.0, 0.0, 0.0])) + ((v2584 * (v2526 / v1118)) * v107);
                v1954 = v1953;
                v2658 = v4907;
            } else {
                let v4901 = Lanes([0.0, v3274[0], 0.0, v3274[1]]);
                v1954 = v721;
                v2658 = v4901;
            }
            let v2000: f64;
            let v2659: Lanes<3>;
            if v510 != 0.0 {
                let v4909 = Lanes([v3270[0], v3270[1], 0.0]);
                v2000 = v718;
                v2659 = v4909;
            } else {
                let v4908 = Lanes([v3274[0], 0.0, v3274[1]]);
                v2000 = v721;
                v2659 = v4908;
            }
            let v1955 = v724 - v1954;
            let v1957 = v1954 - v718;
            let v4920 = (v3474 * v1957) + ((v2658 - (Lanes([0.0, v3270[0], v3270[1], 0.0]))) * v873);
            let v4924 = v2658 * v1960;
            let v4927 = (((v3913 * v1955) + (((Lanes([0.0, v3278[0], v3278[1], 0.0, 0.0])) - (Lanes([v2658[0], 0.0, v2658[1], v2658[2], v2658[3]]))) * v1170)) + (Lanes([v4920[0], 0.0, v4920[1], v4920[2], v4920[3]]))) - ((v2639 * v1954) + (Lanes([v4924[0], 0.0, v4924[1], v4924[2], v4924[3]])));
            let v4928 = v3298 * v737;
            let v4929 = v4928 + v4928;
            let v1967 = (v737 * v737) / v1927;
            let v4934 = ((Lanes([v4929[0], 0.0, v4929[1]])) - (Lanes([0.0, (v2551 * v1967), 0.0]))) / v1927;
            let v4937 = (Lanes([0.0, v4927[0], v4927[1], v4927[2], v4927[3], v4927[4]])) + (Lanes([v4934[0], v4934[1], v4934[2], 0.0, 0.0, 0.0]));
            let v1969 = v758 * v758;
            let v4938 = v3337 * v758;
            let v4940 = (v4938 + v4938) * v1970;
            let v4944 = (Lanes([v4940[0], v4940[1], 0.0, v4940[2], v4940[3], v4940[4], v4940[5], v4940[6], v4940[7]])) + (Lanes([0.0, 0.0, (v2562 * v1969), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
            let v1974 = v751 * v751;
            let v4948 = v3318 * v751;
            let v4950 = (v4948 + v4948) * v1975;
            let v4954 = (Lanes([0.0, v4950[0], v4950[1]])) + (Lanes([(v2564 * v1974), 0.0, 0.0]));
            let v1979 = v748 * v748;
            let v4957 = v3314 * v748;
            let v4959 = (v4957 + v4957) * v1980;
            let v4963 = (Lanes([0.0, v4959[0], v4959[1]])) + (Lanes([(v2566 * v1979), 0.0, 0.0]));
            let v4966 = v3302 * v740;
            let v4967 = v4966 + v4966;
            let v1985 = (v740 * v740) / v1920;
            let v4972 = ((Lanes([v4967[0], 0.0, v4967[1]])) - (Lanes([0.0, (v2552 * v1985), 0.0]))) / v1920;
            let v4976 = v3286 * v1745;
            let v4978 = (v4641 * v729) + (Lanes([0.0, 0.0, v4976[0], v4976[1], 0.0, 0.0]));
            let v4983 = v3278 * v16;
            let v1997 = ((((v1989 + v1322) + (v16 * v724)) - v1994) + v1216) + v1195;
            let v4991 = ((((v2605 + (Lanes([v4076[0], v4076[1], v4076[2], 0.0, 0.0]))) + (Lanes([0.0, v4983[0], v4983[1], 0.0, 0.0]))) - (Lanes([v2616[0], v2616[1], v2616[2], 0.0, 0.0]))) + (Lanes([0.0, v3951[0], v3951[1], 0.0, 0.0]))) + (Lanes([v3932[0], v3932[1], v3932[2], 0.0, 0.0]));
            let v4993 = v3278 * v1997;
            let v4995 = (v4991 * v724) + (Lanes([0.0, v4993[0], v4993[1], 0.0, 0.0]));
            let v4998 = v4583 * v2000;
            let v4999 = v2659 * v1712;
            let v5002 = (Lanes([v4998[0], v4998[1], v4998[2], v4998[3], 0.0])) + (Lanes([0.0, 0.0, v4999[0], v4999[1], v4999[2]]));
            let v2005 = (v2003 + v1332) + v1352;
            let v5006 = (v2610 + v4083) + v4097;
            let v5008 = v3282 * v2005;
            let v5010 = (v5006 * v727) + (Lanes([0.0, v5008[0], v5008[1]]));
            let v2009 = v16 * v754;
            let v5014 = v3327 * v16;
            let v2010 = (v1714 + v1715) + v2009;
            let v5015 = Lanes([0.0, v5014[0], v5014[1], v5014[2], v5014[3], v5014[4]]);
            let v5018 = v3327 * v2010;
            let v5020 = (((v4587 + v4591) + v5015) * v754) + (Lanes([0.0, v5018[0], v5018[1], v5018[2], v5018[3], v5018[4]]));
            let v5022 = (((((((((Lanes([0.0, 0.0, v4937[0], v4937[1], v4937[2], 0.0, v4937[3], v4937[4], v4937[5], 0.0, 0.0])) + (Lanes([v4944[0], v4944[1], 0.0, v4944[2], 0.0, v4944[3], v4944[4], v4944[5], v4944[6], v4944[7], v4944[8]]))) + (Lanes([0.0, 0.0, 0.0, v4954[0], 0.0, 0.0, 0.0, 0.0, 0.0, v4954[1], v4954[2]]))) + (Lanes([0.0, 0.0, 0.0, v4963[0], 0.0, 0.0, 0.0, v4963[1], 0.0, 0.0, v4963[2]]))) + (Lanes([0.0, v4972[0], 0.0, v4972[1], 0.0, v4972[2], 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4978[0], v4978[1], v4978[2], v4978[3], v4978[4], v4978[5], 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v4995[0], v4995[1], 0.0, v4995[2], v4995[3], v4995[4], 0.0, 0.0]))) - (Lanes([0.0, 0.0, 0.0, v5002[0], 0.0, v5002[1], v5002[2], v5002[3], v5002[4], 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v5010[0], v5010[1], v5010[2], 0.0, 0.0, 0.0, 0.0, 0.0]))) + (Lanes([0.0, 0.0, 0.0, v5020[0], 0.0, v5020[1], v5020[2], v5020[3], v5020[4], 0.0, v5020[5]]));
            let v5024 = v3339 * v1717;
            let v5026 = (v4595 * v759) + (Lanes([v5024[0], v5024[1], 0.0, 0.0, v5024[2], v5024[3], v5024[4], v5024[5], v5024[6], v5024[7]]));
            let v2016 = v754 - v760;
            let v5034 = ((Lanes([0.0, v3327[0], v3327[1], v3327[2], v3327[3], v3327[4]])) - (Lanes([v3342[0], 0.0, 0.0, v3342[1], 0.0, v3342[2]]))) * v2015;
            let v5036 = (v2628 * v2016) + (Lanes([v5034[0], 0.0, v5034[1], v5034[2], v5034[3], v5034[4], v5034[5]]));
            let v2020 = v718 - v732;
            let v5043 = ((Lanes([0.0, v3270[0], v3270[1]])) - (Lanes([v3290[0], 0.0, v3290[1]]))) * v2019;
            let v5045 = (v2625 * v2020) + (Lanes([v5043[0], 0.0, v5043[1], v5043[2], 0.0]));
            let v2024 = v759 - v761;
            let v5052 = ((Lanes([v3339[0], v3339[1], 0.0, v3339[2], v3339[3], v3339[4], v3339[5], v3339[6], v3339[7]])) - (Lanes([0.0, 0.0, v3345[0], 0.0, 0.0, v3345[1], 0.0, v3345[2], v3345[3]]))) * v2023;
            let v5054 = (v2629 * v2024) + (Lanes([v5052[0], v5052[1], v5052[2], 0.0, v5052[3], v5052[4], v5052[5], v5052[6], v5052[7], v5052[8]]));
            let v5058 = v3290 * v1585;
            let v5060 = (v4432 * v732) + (Lanes([v5058[0], 0.0, v5058[1]]));
            let v2028 = (((((((((((((((((v1170 * v1955) + (v873 * v1957)) - (v1960 * v1954)) + v1967) + (v1969 * v1970)) + (v1974 * v1975)) + (v1979 * v1980)) + v1985) + (v1745 * v729)) + (v1997 * v724)) - (v1712 * v2000)) + (v2005 * v727)) + (v2010 * v754)) + (v1717 * v759)) + (v2015 * v2016)) + (v2019 * v2020)) + (v2023 * v2024)) + (v1585 * v732);
            let v5062 = (((((Lanes([v5022[0], v5022[1], v5022[2], 0.0, v5022[3], v5022[4], v5022[5], v5022[6], v5022[7], v5022[8], v5022[9], v5022[10]])) + (Lanes([v5026[0], v5026[1], 0.0, v5026[2], v5026[3], 0.0, v5026[4], v5026[5], v5026[6], v5026[7], v5026[8], v5026[9]]))) + (Lanes([0.0, 0.0, 0.0, v5036[0], v5036[1], 0.0, v5036[2], v5036[3], v5036[4], v5036[5], 0.0, v5036[6]]))) + (Lanes([0.0, 0.0, 0.0, v5045[0], v5045[1], 0.0, 0.0, v5045[2], v5045[3], v5045[4], 0.0, 0.0]))) + (Lanes([v5054[0], v5054[1], 0.0, v5054[2], v5054[3], 0.0, v5054[4], v5054[5], v5054[6], v5054[7], v5054[8], v5054[9]]))) + (Lanes([0.0, 0.0, 0.0, v5060[0], v5060[1], 0.0, 0.0, 0.0, v5060[2], 0.0, 0.0, 0.0]));
            let v2030 = v2 - v2029;
            let v2031 = v2030 * v316;
            let v5063 = v2934 * v2030;
            let v2032 = v2031 * v1059;
            let v5067 = (Lanes([(v5063 * v1059), 0.0, 0.0])) + (v3714 * v2031);
            let v5068 = Lanes([0.0, v3282[0], v3282[1]]);
            let v5069 = Lanes([v3671, 0.0, 0.0]);
            let v2034 = (v727 - v1033) / v1034;
            let v5074 = ((v5068 - v5069) - (Lanes([(v3672 * v2034), 0.0, 0.0]))) / v1034;
            let v2035 = if v727 < v1033 { 1.0 } else { 0.0 };
            let v2048: f64;
            let v2660: Lanes<3>;
            if v2035 != 0.0 {
                let v2036 = v2034.exp();
                let v2037 = v2 + v2036;
                let v2038 = v2037.ln();
                let v2040 = v727 - (v1034 * v2038);
                let v5091 = v5068 - ((Lanes([(v3672 * v2038), 0.0, 0.0])) + (((v5074 * v2036) * (v2526 / v2037)) * v1034));
                v2048 = v2040;
                v2660 = v5091;
            } else {
                let v2042 = (-v2034).exp();
                let v2043 = v2 + v2042;
                let v2044 = v2043.ln();
                let v2046 = v1033 - (v1034 * v2044);
                let v5083 = v5069 - ((Lanes([(v3672 * v2044), 0.0, 0.0])) + ((((v5074 * v2687) * v2042) * (v2526 / v2043)) * v1034));
                v2048 = v2046;
                v2660 = v5083;
            }
            let v2047 = v2029 * v316;
            let v2050 = v2 - (v2048 * v308);
            let v2052 = v2 - (v2050.powf(v1052));
            let v2056 = (v1054 * v2052) + (v154 * (v727 - v2048));
            let v2057 = v2047 * v2056;
            let v5112 = (Lanes([((v2934 * v2029) * v2056), 0.0, 0.0])) + ((((Lanes([(v3706 * v2052), 0.0, 0.0])) + ((((((v2660 * v308) + (Lanes([(v2920 * v2048), 0.0, 0.0]))) * v2687) * (v1052 * (v2050.powf(v3702)))) * v2687) * v1054)) + ((v5068 - v2660) * v154)) * v2047);
            let v2059 = v2058 * v332;
            let v2060 = v2059 * v1109;
            let v5117 = (Lanes([((v2953 * v2058) * v1109), 0.0, 0.0, 0.0])) + (v3792 * v2059);
            let v2061 = v667 * v454;
            let v5120 = (v3240 * v454) + (v3034 * v667);
            let v2062 = v424 * v2061;
            let v5121 = v5120 * v424;
            let v2063 = v2062 * v1117;
            let v2064 = v2063 * v1734;
            let v5126 = ((Lanes([(v5121 * v1117), 0.0, 0.0])) + (v3806 * v2062)) * v1734;
            let v5129 = (Lanes([v5126[0], v5126[1], v5126[2], 0.0, 0.0])) + (v2637 * v2063);
            let v2065 = v2062 * v1126;
            let v2066 = v2065 * v1734;
            let v5134 = ((Lanes([(v5121 * v1126), 0.0, 0.0, 0.0])) + (v3828 * v2062)) * v1734;
            let v5137 = (Lanes([v5134[0], 0.0, v5134[1], v5134[2], v5134[3]])) + (v2637 * v2065);
            let v2067 = v42 * v325;
            let v5138 = v2547 * v42;
            let v5139 = Lanes([v3730, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let v2069 = (v754 - v1072) / v2067;
            let v5144 = ((v3415 - v5139) - (Lanes([(v5138 * v2069), 0.0, 0.0, 0.0, 0.0, 0.0]))) / v2067;
            let v2070 = if v754 < v1072 { 1.0 } else { 0.0 };
            let v2082: f64;
            let v2661: Lanes<6>;
            if v2070 != 0.0 {
                let v2071 = v2069.exp();
                let v2072 = v2 + v2071;
                let v2073 = v2072.ln();
                let v2075 = v754 - (v2067 * v2073);
                let v5161 = v3415 - ((Lanes([(v5138 * v2073), 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v5144 * v2071) * (v2526 / v2072)) * v2067));
                v2082 = v2075;
                v2661 = v5161;
            } else {
                let v2077 = (-v2069).exp();
                let v2078 = v2 + v2077;
                let v2079 = v2078.ln();
                let v2081 = v1072 - (v2067 * v2079);
                let v5153 = v5139 - ((Lanes([(v5138 * v2079), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v5144 * v2687) * v2077) * (v2526 / v2078)) * v2067));
                v2082 = v2081;
                v2661 = v5153;
            }
            let v2083 = v2082 / v325;
            let v2084 = v2 - v2083;
            let v2086 = v2 - (v2084.powf(v1094));
            let v2088 = v754 - v2082;
            let v2090 = (v1095 * v2086) + (v1067 * v2088);
            let v5186 = v3327 * v333;
            let v2093 = (v1066 * v2090) + (v333 * v754);
            let v2095 = v2 - v2058;
            let v2097 = ((v332 * v2093) * v2095) * v9;
            let v5196 = (((Lanes([(v2953 * v2093), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([(v3719 * v2090), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([(v3755 * v2086), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((v2661 - (Lanes([(v2547 * v2083), 0.0, 0.0, 0.0, 0.0, 0.0]))) / v325) * v2687) * (v1094 * (v2084.powf(v3761)))) * v2687) * v1095)) + ((Lanes([(v3722 * v2088), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3415 - v2661) * v1067))) * v1066)) + ((Lanes([(v2954 * v754), 0.0, 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, v5186[0], v5186[1], v5186[2], v5186[3], v5186[4]])))) * v332)) * v2095) * v9;
            let v5197 = Lanes([0.0, 0.0, v3730, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let v2099 = (v759 - v1072) / v2067;
            let v5202 = ((v3406 - v5197) - (Lanes([0.0, 0.0, (v5138 * v2099), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v2067;
            let v2100 = if v759 < v1072 { 1.0 } else { 0.0 };
            let v2112: f64;
            let v2662: Lanes<9>;
            if v2100 != 0.0 {
                let v2101 = v2099.exp();
                let v2102 = v2 + v2101;
                let v2103 = v2102.ln();
                let v2105 = v759 - (v2067 * v2103);
                let v5219 = v3406 - ((Lanes([0.0, 0.0, (v5138 * v2103), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v5202 * v2101) * (v2526 / v2102)) * v2067));
                v2112 = v2105;
                v2662 = v5219;
            } else {
                let v2107 = (-v2099).exp();
                let v2108 = v2 + v2107;
                let v2109 = v2108.ln();
                let v2111 = v1072 - (v2067 * v2109);
                let v5211 = v5197 - ((Lanes([0.0, 0.0, (v5138 * v2109), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v5202 * v2687) * v2107) * (v2526 / v2108)) * v2067));
                v2112 = v2111;
                v2662 = v5211;
            }
            let v2113 = v2112 / v325;
            let v2114 = v2 - v2113;
            let v2116 = v2 - (v2114.powf(v1094));
            let v2118 = v759 - v2112;
            let v2120 = (v1095 * v2116) + (v1067 * v2118);
            let v5244 = v3339 * v333;
            let v2123 = (v1066 * v2120) + (v333 * v759);
            let v2126 = ((v332 * v2123) * v2095) * v8;
            let v5254 = (((Lanes([0.0, 0.0, (v2953 * v2123), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, (v3719 * v2120), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, (v3755 * v2116), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((((v2662 - (Lanes([0.0, 0.0, (v2547 * v2113), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v325) * v2687) * (v1094 * (v2114.powf(v3761)))) * v2687) * v1095)) + ((Lanes([0.0, 0.0, (v3722 * v2118), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((v3406 - v2662) * v1067))) * v1066)) + ((Lanes([0.0, 0.0, (v2954 * v759), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (Lanes([v5244[0], v5244[1], 0.0, v5244[2], v5244[3], v5244[4], v5244[5], v5244[6], v5244[7]])))) * v332)) * v2095) * v8;
            let v2127 = v42 * v318;
            let v5255 = v2550 * v42;
            let v2131 = v2 - (v27.powf((v2128 / v320)));
            let v2132 = v318 * v2131;
            let v5257 = Lanes([v3290[0], 0.0, v3290[1]]);
            let v5258 = Lanes([0.0, (v2550 * v2131), 0.0]);
            let v2134 = (v732 - v2132) / v2127;
            let v5263 = ((v5257 - v5258) - (Lanes([0.0, (v5255 * v2134), 0.0]))) / v2127;
            let v2135 = if v732 < v2132 { 1.0 } else { 0.0 };
            let v2149: f64;
            let v2663: Lanes<3>;
            if v2135 != 0.0 {
                let v2136 = v2134.exp();
                let v2137 = v2 + v2136;
                let v2138 = v2137.ln();
                let v2140 = v732 - (v2127 * v2138);
                let v5280 = v5257 - ((Lanes([0.0, (v5255 * v2138), 0.0])) + (((v5263 * v2136) * (v2526 / v2137)) * v2127));
                v2149 = v2140;
                v2663 = v5280;
            } else {
                let v2142 = (-v2134).exp();
                let v2143 = v2 + v2142;
                let v2144 = v2143.ln();
                let v2146 = v2132 - (v2127 * v2144);
                let v5272 = v5258 - ((Lanes([0.0, (v5255 * v2144), 0.0])) + ((((v5263 * v2687) * v2142) * (v2526 / v2143)) * v2127));
                v2149 = v2146;
                v2663 = v5272;
            }
            let v2147 = v2 - v320;
            let v2148 = v318 / v2147;
            let v2150 = v2149 / v318;
            let v2151 = v2 - v2150;
            let v2153 = v2 - (v2151.powf(v2147));
            let v2157 = (v2148 * v2153) + (v27 * (v732 - v2149));
            let v2158 = v322 * v2157;
            let v5302 = (Lanes([0.0, (v2942 * v2157), 0.0])) + ((((Lanes([0.0, ((v2550 / v2147) * v2153), 0.0])) + ((((((v2663 - (Lanes([0.0, (v2550 * v2150), 0.0]))) / v318) * v2687) * (v2147 * (v2151.powf((v2147 - v2526))))) * v2687) * v2148)) + ((v5257 - v2663) * v27)) * v322);
            let v2159 = v661 * v454;
            let v2160 = v449 / v454;
            let v2162 = v2 / v2161;
            let v2163 = v2160.powf(v2162);
            let v2164 = v2159 * v2163;
            let v5315 = (((v3237 * v454) + (v3034 * v661)) * v2163) + ((((v3031 - (v3034 * v2160)) / v454) * (v2162 * (v2160.powf((v2162 - v2526))))) * v2159);
            let v2165 = v2161 * v107;
            let v5316 = v2695 * v2161;
            let v2166 = v724 / v2165;
            let v5320 = (v3673 - (Lanes([(v5316 * v2166), 0.0, 0.0]))) / v2165;
            let v2167 = if v2166 < v763 { 1.0 } else { 0.0 };
            let v2173: f64;
            let v2664: Lanes<3>;
            if v2167 != 0.0 {
                let v2168 = v2166.exp();
                let v5322 = v5320 * v2168;
                v2173 = v2168;
                v2664 = v5322;
            } else {
                let v2169 = v763.exp();
                let v2172 = v2169 * (v2 + (v2166 - v763));
                let v5321 = v5320 * v2169;
                v2173 = v2172;
                v2664 = v5321;
            }
            let v2174 = v2164 * v2173;
            let v5326 = (Lanes([(v5315 * v2173), 0.0, 0.0])) + (v2664 * v2164);
            let v2175 = v435 * v672;
            let v2177 = (v2175 * v107) / v369;
            let v5333 = ((((v3243 * v435) * v107) + (v2695 * v2175)) - (v2974 * v2177)) / v369;
            let v2178 = v424 * v2177;
            let v2179 = v2178 * v1793;
            let v2182 = (v2180 + v952) + v27;
            let v2183 = v2179 * v2182;
            let v5343 = (((Lanes([((v5333 * v424) * v1793), 0.0, 0.0, 0.0])) + (v2585 * v2178)) * v2182) + ((v2587 + (Lanes([v2580[0], v2580[1], v2580[2], 0.0]))) * v2179);
            let v2185 = if v2184 == v0 { 1.0 } else { 0.0 };
            let v2215: f64;
            let v2665: Lanes<6>;
            if v2185 != 0.0 {
                let v2186 = v677 * v424;
                let v2189 = (v2061 * v1506) + (v2177 * v1510);
                let v2191 = (v2186 * v2189) / v674;
                let v5384 = (((Lanes([((v3246 * v424) * v2189), 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([(v5120 * v1506), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4303 * v2061)) + ((Lanes([(v5333 * v1510), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v4309 * v2177))) * v2186)) - (Lanes([(v3244 * v2191), 0.0, 0.0, 0.0, 0.0, 0.0]))) / v674;
                v2215 = v2191;
                v2665 = v5384;
            } else {
                let v2195 = (v754 - v2192) / v2194;
                let v2196 = v2195 * v109;
                let v5350 = (((v3415 - (Lanes([v2546, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v2194) * v109) + (Lanes([(v2698 * v2195), 0.0, 0.0, 0.0, 0.0, 0.0]));
                let v2197 = if v2196 < v763 { 1.0 } else { 0.0 };
                let v2205: f64;
                let v2666: Lanes<6>;
                if v2197 != 0.0 {
                    let v2198 = v2196.exp();
                    let v5352 = v5350 * v2198;
                    v2205 = v2198;
                    v2666 = v5352;
                } else {
                    let v2199 = v763.exp();
                    let v2202 = v2199 * (v2 + (v2196 - v763));
                    let v5351 = v5350 * v2199;
                    v2205 = v2202;
                    v2666 = v5351;
                }
                let v2203 = v1511 * v683;
                let v2208 = (v2 + (v435 * v2205)).sqrt();
                let v2209 = v2 + v2208;
                let v2210 = (v2203 * v1498) / v2209;
                let v5366 = (((Lanes([(((v4310 * v683) + (v3249 * v1511)) * v1498), 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2570 * v2203)) - (((v2666 * v435) * (v2526 / (v3005 * v2208))) * v2210)) / v2209;
                v2215 = v2210;
                v2665 = v5366;
            }
            let v2214 = if (if (if v1586 == v2 { 1.0 } else { 0.0 }) != 0.0 || (if v1586 == v154 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v1588 != 0.0 { 1.0 } else { 0.0 };
            let v2400: f64;
            let v2409: f64;
            let v2667: Lanes<10>;
            let v2668: Lanes<6>;
            if v2214 != 0.0 {
                let v2216 = v2215 * v9;
                let v5385 = v2665 * v9;
                let v2255: f64;
                let v2669: Lanes<9>;
                if v2185 != 0.0 {
                    let v2217 = v1111 * v1595;
                    let v5412 = (Lanes([0.0, 0.0, (v3796 * v1595), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2572 * v1111);
                    let v2220 = (v2 + v2217).sqrt();
                    let v2221 = v2 + v2220;
                    let v2222 = (v2217 - v1111) / v2221;
                    let v2224 = v435 * v2223;
                    let v5421 = v2576 * v435;
                    let v2226 = (v2 + v2224).sqrt();
                    let v2227 = v2 + v2226;
                    let v2228 = v2224 / v2227;
                    let v2229 = v424 * v8;
                    let v2230 = v2229 * v677;
                    let v2233 = (v2061 * v2222) + (v2177 * v2228);
                    let v2235 = (v2230 * v2233) / v674;
                    let v5445 = (((Lanes([0.0, 0.0, ((v3246 * v2229) * v2233), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((Lanes([0.0, 0.0, (v5120 * v2222), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + ((((v5412 - (Lanes([0.0, 0.0, v3796, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) - ((v5412 * (v2526 / (v3005 * v2220))) * v2222)) / v2221) * v2061)) + ((Lanes([0.0, 0.0, (v5333 * v2228), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (((v5421 - ((v5421 * (v2526 / (v3005 * v2226))) * v2228)) / v2227) * v2177))) * v2230)) - (Lanes([0.0, 0.0, (v3244 * v2235), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) / v674;
                    v2255 = v2235;
                    v2669 = v5445;
                } else {
                    let v2236 = v759 - v2192;
                    let v2237 = v2236 * v109;
                    let v5391 = ((v3406 - (Lanes([0.0, 0.0, v2546, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v109) + (Lanes([0.0, 0.0, (v2698 * v2236), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]));
                    let v2238 = if v2237 < v763 { 1.0 } else { 0.0 };
                    let v2248: f64;
                    let v2670: Lanes<9>;
                    if v2238 != 0.0 {
                        let v2239 = v2237.exp();
                        let v5393 = v5391 * v2239;
                        v2248 = v2239;
                        v2670 = v5393;
                    } else {
                        let v2240 = v763.exp();
                        let v2243 = v2240 * (v2 + (v2237 - v763));
                        let v5392 = v5391 * v2240;
                        v2248 = v2243;
                        v2670 = v5392;
                    }
                    let v2244 = v27 * v8;
                    let v2245 = v2244 * v541;
                    let v2246 = v2245 * v683;
                    let v2251 = (v2 + (v435 * v2248)).sqrt();
                    let v2252 = v2 + v2251;
                    let v2253 = (v2246 * v1595) / v2252;
                    let v5408 = (((Lanes([0.0, 0.0, ((((v3092 * v2244) * v683) + (v3249 * v2245)) * v1595), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])) + (v2572 * v2246)) - (((v2670 * v435) * (v2526 / (v3005 * v2251))) * v2253)) / v2252;
                    v2255 = v2253;
                    v2669 = v5408;
                }
                let v2256 = v2254 * v2255;
                let v5447 = v2669 * v2254;
                let v5449 = (v2630 * v2255) + (Lanes([v5447[0], v5447[1], 0.0, v5447[2], v5447[3], v5447[4], v5447[5], v5447[6], v5447[7], v5447[8]]));
                v2400 = v2256;
                v2409 = v2216;
                v2667 = v5449;
                v2668 = v5385;
            } else {
                v2400 = v0;
                v2409 = v2215;
                v2667 = v4433;
                v2668 = v2665;
            }
            let v2258 = if v2257 == v2 { 1.0 } else { 0.0 };
            let v2362: f64;
            let v2364: f64;
            let v2372: f64;
            let v2381: f64;
            let v2671: Lanes<5>;
            let v2672: Lanes<3>;
            let v2673: Lanes<5>;
            let v2674: Lanes<6>;
            if v2258 != 0.0 {
                let v2259 = -v28;
                let v5454 = v3701 * (v2259 * (v1051.powf((v2259 - v2526))));
                let v2261 = (v1051.powf(v2259)) - v154;
                let v2262 = if v1036 < v0 { 1.0 } else { 0.0 };
                let v2270: f64;
                let v2675: Lanes<3>;
                if v2262 != 0.0 {
                    let v2263 = v1036.exp();
                    let v2264 = v2 + v2263;
                    let v2265 = v2 / v2264;
                    let v5463 = (((v3679 * v2263) * v2265) * v2687) / v2264;
                    v2270 = v2265;
                    v2675 = v5463;
                } else {
                    let v2267 = (-v1036).exp();
                    let v5456 = (v3679 * v2687) * v2267;
                    let v2268 = v2 + v2267;
                    let v2269 = v2267 / v2268;
                    let v5459 = (v5456 - (v5456 * v2269)) / v2268;
                    v2270 = v2269;
                    v2675 = v5459;
                }
                let v2272 = (v2261 * v2270) + v154;
                let v5470 = (Lanes([(v5063 * v2272), 0.0, 0.0])) + (((v5454 * v2270) + (v2675 * v2261)) * v2031);
                let v2275 = (v1113 * v109) / v441;
                let v2276 = v424 / v1115;
                let v2277 = v2275 * v2276;
                let v2278 = v2062 * v1734;
                let v5490 = ((((((v3800 * v109) + (Lanes([(v2698 * v1113), 0.0, 0.0]))) - (Lanes([(v2553 * v2275), 0.0, 0.0]))) / v441) * v2276) + ((((v3803 * v2276) * v2687) / v1115) * v2275)) * v2278;
                let v2280 = v2174 / v2165;
                let v5496 = (v5326 - (Lanes([(v5316 * v2280), 0.0, 0.0]))) / v2165;
                let v2281 = v891 * v729;
                let v2283 = ((v2031 * v2272) + (v2278 * v2277)) + v2280;
                let v2284 = v2281 * v2283;
                let v5502 = (v3286 * v891) * v2283;
                let v5503 = (((Lanes([v5470[0], v5470[1], v5470[2], 0.0, 0.0])) + ((((Lanes([(v5121 * v1734), 0.0, 0.0, 0.0, 0.0])) + (v2637 * v2062)) * v2277) + (Lanes([v5490[0], v5490[1], v5490[2], 0.0, 0.0])))) + (Lanes([v5496[0], v5496[1], v5496[2], 0.0, 0.0]))) * v2281;
                let v5506 = (Lanes([0.0, 0.0, v5502[0], v5502[1], 0.0, 0.0])) + (Lanes([v5503[0], v5503[1], 0.0, v5503[2], v5503[3], v5503[4]]));
                let v2286 = v2 - v2285;
                let v2287 = v2286 * v2174;
                let v5507 = v5326 * v2286;
                let v5508 = v5326 * v2285;
                let v2289 = v2064 + (v2285 * v2174);
                let v5510 = v5129 + (Lanes([v5508[0], v5508[1], v5508[2], 0.0, 0.0]));
                let v2292 = (v2290 * v2289) + v2066;
                let v5512 = (v5510 * v2290) + v5137;
                let v2293 = v2 - v2290;
                let v2294 = v2293 * v2289;
                let v5513 = v5510 * v2293;
                v2362 = v2294;
                v2364 = v2287;
                v2372 = v2292;
                v2381 = v2284;
                v2671 = v5513;
                v2672 = v5507;
                v2673 = v5512;
                v2674 = v5506;
            } else {
                v2362 = v2064;
                v2364 = v2174;
                v2372 = v2066;
                v2381 = v0;
                v2671 = v5129;
                v2672 = v5326;
                v2673 = v5137;
                v2674 = v5450;
            }
            let v2296 = (v1 * v873) * v21;
            let v5515 = (v3474 * v1) * v21;
            let v2298 = (v1 * v1170) * v21;
            let v5517 = (v3913 * v1) * v21;
            let v2300 = (v1 * v2005) * v21;
            let v5519 = (v5006 * v1) * v21;
            let v2302 = (v1 * v1997) * v21;
            let v5521 = (v4991 * v1) * v21;
            let v2499: f64;
            let v2500: f64;
            let v2676: Lanes<4>;
            let v2677: Lanes<4>;
            if v510 != 0.0 {
                let v2305 = (v1 * (-v1712)) * v21;
                let v5528 = ((v4583 * v2687) * v1) * v21;
                v2499 = v2305;
                v2500 = v0;
                v2676 = v5528;
                v2677 = v5525;
            } else {
                let v2308 = (v1 * (-v1712)) * v21;
                let v5524 = ((v4583 * v2687) * v1) * v21;
                v2499 = v0;
                v2500 = v2308;
                v2676 = v5525;
                v2677 = v5524;
            }
            let v2310 = (v1 * v2015) * v21;
            let v5530 = (v2628 * v1) * v21;
            let v2312 = (v1 * v2019) * v21;
            let v5532 = (v2625 * v1) * v21;
            let v2314 = (v1 * v2023) * v21;
            let v5534 = (v2629 * v1) * v21;
            let v2316 = (v1 * v1585) * v21;
            let v5536 = (v4432 * v1) * v21;
            let v2318 = (v1 * v1745) * v21;
            let v5538 = (v4641 * v1) * v21;
            let v2322 = (v1 * (v2319 * v1960)) * v21;
            let v5541 = ((v2639 * v2319) * v1) * v21;
            let v5542 = v3298 * v1;
            let v2324 = (v1 * v737) / v1927;
            let v2325 = v2324 * v21;
            let v5548 = (((Lanes([v5542[0], 0.0, v5542[1]])) - (Lanes([0.0, (v2551 * v2324), 0.0]))) / v1927) * v21;
            let v5549 = v3302 * v1;
            let v2327 = (v1 * v740) / v1920;
            let v2328 = v2327 * v21;
            let v5555 = (((Lanes([v5549[0], 0.0, v5549[1]])) - (Lanes([0.0, (v2552 * v2327), 0.0]))) / v1920) * v21;
            let v2330 = v2329 * v91;
            let v5556 = v2527 * v2329;
            let v2332 = (ddt(13541, v2330)) * v21;
            let v5559 = (v5556 * v5557) * v21;
            let v2515 = v2330 * v21;
            let v5560 = v5556 * v21;
            let v2333 = v2 - v703;
            let v2334 = if v701 > v22 { 1.0 } else { 0.0 };
            let v2356: f64;
            let v2678: f64;
            if v2334 != 0.0 {
                let v2336 = if v2335 == v0 { 1.0 } else { 0.0 };
                let v2357: f64;
                let v2679: f64;
                if v2336 != 0.0 {
                    let v2338 = (v91 / v705) * v21;
                    let v5573 = (v2527 / v705) * v21;
                    v2357 = v2338;
                    v2679 = v5573;
                } else {
                    let v2340 = if (v2333.abs()) < v1665 { 1.0 } else { 0.0 };
                    let v2358: f64;
                    let v2680: f64;
                    if v2340 != 0.0 {
                        let v2342 = (v15 / v705) * v21;
                        let v2344 = v2 + (v91 / v15);
                        let v2346 = v2342 * (v2344.ln());
                        let v5571 = ((v2527 / v15) * (v2526 / v2344)) * v2342;
                        v2358 = v2346;
                        v2680 = v5571;
                    } else {
                        let v2349 = (v15 / (v2333 * v705)) * v21;
                        let v2351 = v2 + (v91 / v15);
                        let v2354 = v2349 * ((v2351.powf(v2333)) - v2);
                        let v5567 = ((v2527 / v15) * (v2333 * (v2351.powf((v2333 - v2526))))) * v2349;
                        v2358 = v2354;
                        v2680 = v5567;
                    }
                    v2357 = v2358;
                    v2679 = v2680;
                }
                v2356 = v2357;
                v2678 = v2679;
            } else {
                let v2355 = v91 / v20;
                let v5561 = v2527 / v20;
                v2356 = v2355;
                v2678 = v5561;
            }
            let v2361 = (v2359 * v2028) * v21;
            let v5575 = (v5062 * v2359) * v21;
            let v2366 = v1 * ((v2032 + v2362) + v2364);
            let v5580 = (((Lanes([v5067[0], v5067[1], v5067[2], 0.0, 0.0])) + v2671) + (Lanes([v2672[0], v2672[1], v2672[2], 0.0, 0.0]))) * v1;
            let v2368 = (ddt(13609, v2366)) * v21;
            let v5582 = (v5580 * v5557) * v21;
            let v2516 = v2366 * v21;
            let v5583 = v5580 * v21;
            let v2369 = v1 * v2057;
            let v5584 = v5112 * v1;
            let v2371 = (ddt(13615, v2369)) * v21;
            let v5586 = (v5584 * v5557) * v21;
            let v2517 = v2369 * v21;
            let v5587 = v5584 * v21;
            let v2375 = v1 * ((v2060 + v2372) + v2183);
            let v5592 = (((Lanes([v5117[0], 0.0, v5117[1], v5117[2], v5117[3]])) + v2673) + (Lanes([v5343[0], 0.0, v5343[1], v5343[2], v5343[3]]))) * v1;
            let v2377 = (ddt(13625, v2375)) * v21;
            let v5594 = (v5592 * v5557) * v21;
            let v2518 = v2375 * v21;
            let v5595 = v5592 * v21;
            let v2378 = v1 * v2158;
            let v5596 = v5302 * v1;
            let v2380 = (ddt(13631, v2378)) * v21;
            let v5598 = (v5596 * v5557) * v21;
            let v2519 = v2378 * v21;
            let v5599 = v5596 * v21;
            let v2382 = v1 * v2381;
            let v5600 = v2674 * v1;
            let v2384 = (ddt(13637, v2382)) * v21;
            let v5602 = (v5600 * v5557) * v21;
            let v2520 = v2382 * v21;
            let v5603 = v5600 * v21;
            let v2386 = v1 * v2385;
            let v2387 = v2386 * v742;
            let v5604 = v3306 * v2386;
            let v2389 = (ddt(13645, v2387)) * v21;
            let v5606 = (v5604 * v5557) * v21;
            let v2521 = v2387 * v21;
            let v5607 = v5604 * v21;
            let v2391 = v1 * v2390;
            let v2392 = v2391 * v745;
            let v5608 = v3310 * v2391;
            let v2394 = (ddt(13653, v2392)) * v21;
            let v5610 = (v5608 * v5557) * v21;
            let v2522 = v2392 * v21;
            let v5611 = v5608 * v21;
            let v2396 = (v1 * v1717) * v21;
            let v5613 = (v4595 * v1) * v21;
            let v2397 = v1 * v758;
            let v5615 = (v3337 * v1) * v1970;
            let v2399 = (v2397 * v1970) * v21;
            let v5620 = ((Lanes([v5615[0], v5615[1], 0.0, v5615[2], v5615[3], v5615[4], v5615[5], v5615[6], v5615[7]])) + (Lanes([0.0, 0.0, (v2562 * v2397), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]))) * v21;
            let v2402 = v1 * (v2126 + v2400);
            let v5623 = ((Lanes([v5254[0], v5254[1], 0.0, v5254[2], v5254[3], v5254[4], v5254[5], v5254[6], v5254[7], v5254[8]])) + v2667) * v1;
            let v2404 = (ddt(13673, v2402)) * v21;
            let v5625 = (v5623 * v5557) * v21;
            let v2523 = v2402 * v21;
            let v5626 = v5623 * v21;
            let v2408 = (v1 * ((v1715 + v2009) + v1714)) * v21;
            let v5630 = (((v4591 + v5015) + v4587) * v1) * v21;
            let v2411 = v1 * (v2097 + v2409);
            let v5632 = (v5196 + v2668) * v1;
            let v2413 = (ddt(13692, v2411)) * v21;
            let v5634 = (v5632 * v5557) * v21;
            let v2524 = v2411 * v21;
            let v5635 = v5632 * v21;
            let v2501: f64;
            let v2502: f64;
            let v2681: Lanes<3>;
            if v709 != 0.0 {
                let v2414 = v1 * v751;
                let v5638 = (v3318 * v1) * v1975;
                let v2416 = (v2414 * v1975) * v21;
                let v5643 = ((Lanes([0.0, v5638[0], v5638[1]])) + (Lanes([(v2564 * v2414), 0.0, 0.0]))) * v21;
                v2501 = v2416;
                v2502 = v0;
                v2681 = v5643;
            } else {
                v2501 = v0;
                v2502 = v2417;
                v2681 = v5636;
            }
            let v2503: f64;
            let v2504: f64;
            let v2682: Lanes<3>;
            if v712 != 0.0 {
                let v2418 = v1 * v748;
                let v5646 = (v3314 * v1) * v1980;
                let v2420 = (v2418 * v1980) * v21;
                let v5651 = ((Lanes([0.0, v5646[0], v5646[1]])) + (Lanes([(v2566 * v2418), 0.0, 0.0]))) * v21;
                v2503 = v2420;
                v2504 = v0;
                v2682 = v5651;
            } else {
                v2503 = v0;
                v2504 = v2421;
                v2682 = v5644;
            }
            let v2423 = (v1168 + v1167) / v1164;
            let v5655 = ((v3908 + v3909) - (v3898 * v2423)) / v1164;
            let v2425 = if v2424 > v0 { 1.0 } else { 0.0 };
            let v2428: f64;
            let v2683: Lanes<5>;
            if v2425 != 0.0 {
                let v2426 = v1960 / v2423;
                let v2427 = v2426.abs();
                let v5662 = ((v2639 - (v5655 * v2426)) / v2423) * ((v3005 * (if v2426 >= v3479 { 1.0 } else { 0.0 })) - v2526);
                v2428 = v2427;
                v2683 = v5662;
            } else {
                v2428 = v0;
                v2683 = v4628;
            }
            let v2429 = if v2423 > v0 { 1.0 } else { 0.0 };
            let v2436: f64;
            let v2684: Lanes<5>;
            if v2429 != 0.0 {
                let v2431 = (v2362 + v2372) / v2423;
                let v5673 = ((v2671 + v2673) - (v5655 * v2431)) / v2423;
                v2436 = v2431;
                v2684 = v5673;
            } else {
                let v2432 = v667 * v1734;
                let v2433 = v2432 * v1164;
                let v5669 = (((Lanes([(v3240 * v1734), 0.0, 0.0, 0.0, 0.0])) + (v2637 * v667)) * v1164) + (v3898 * v2432);
                v2436 = v2433;
                v2684 = v5669;
            }
            let v2435 = if v2434 == v2 { 1.0 } else { 0.0 };
            let v2452: f64;
            let v2685: Lanes<5>;
            if v2435 != 0.0 {
                let v2437 = v2290 * v2436;
                let v5675 = v2684 * v2290;
                v2452 = v2437;
                v2685 = v5675;
            } else {
                let v2438 = if v2434 == v27 { 1.0 } else { 0.0 };
                let v2453: f64;
                let v2686: Lanes<5>;
                if v2438 != 0.0 {
                    let v2440 = v2439 * v2436;
                    let v5674 = v2684 * v2439;
                    v2453 = v2440;
                    v2686 = v5674;
                } else {
                    v2453 = v0;
                    v2686 = v4628;
                }
                v2452 = v2453;
                v2685 = v2686;
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
            let v5677 = v2685 * v2454;
            let v5681 = (Lanes([v5677[0], v5677[1], v5677[2], v5677[3], v5677[4], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, ((v2539 * v5557) * v2452)]));
            let v2525 = v2452 * v2451;
            let v5682 = v2685 * v2451;
            let v5686 = (Lanes([v5682[0], v5682[1], v5682[2], v5682[3], v5682[4], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (v2539 * v2452)]));
            let v2456 = v2428 * v2451;
            let v5687 = v2683 * v2451;
            let v5691 = (Lanes([v5687[0], v5687[1], v5687[2], v5687[3], v5687[4], 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, 0.0, (v2539 * v2428)]));
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
            let v5692 = v5515[0];
            let v5693 = v5515[1];
            let v5694 = v5515[2];
            let v5695 = v5515[3];
            let v5696 = v5517[0];
            let v5697 = v5517[1];
            let v5698 = v5517[2];
            let v5699 = v5517[3];
            let v5700 = v5517[4];
            let v5701 = v5519[0];
            let v5702 = v5519[1];
            let v5703 = v5519[2];
            let v5704 = v5521[0];
            let v5705 = v5521[1];
            let v5706 = v5521[2];
            let v5707 = v5521[3];
            let v5708 = v5521[4];
            let v5709 = v2676[0];
            let v5710 = v2676[1];
            let v5711 = v2676[2];
            let v5712 = v2676[3];
            let v5713 = v2677[0];
            let v5714 = v2677[1];
            let v5715 = v2677[2];
            let v5716 = v2677[3];
            let v5717 = v5530[0];
            let v5718 = v5530[1];
            let v5719 = v5530[2];
            let v5720 = v5530[3];
            let v5721 = v5530[4];
            let v5722 = v5530[5];
            let v5723 = v5530[6];
            let v5724 = v5532[0];
            let v5725 = v5532[1];
            let v5726 = v5532[2];
            let v5727 = v5532[3];
            let v5728 = v5532[4];
            let v5729 = v5534[0];
            let v5730 = v5534[1];
            let v5731 = v5534[2];
            let v5732 = v5534[3];
            let v5733 = v5534[4];
            let v5734 = v5534[5];
            let v5735 = v5534[6];
            let v5736 = v5534[7];
            let v5737 = v5534[8];
            let v5738 = v5534[9];
            let v5739 = v5536[0];
            let v5740 = v5536[1];
            let v5741 = v5536[2];
            let v5742 = v5538[0];
            let v5743 = v5538[1];
            let v5744 = v5538[2];
            let v5745 = v5538[3];
            let v5746 = v5538[4];
            let v5747 = v5538[5];
            let v5748 = v5541[0];
            let v5749 = v5541[1];
            let v5750 = v5541[2];
            let v5751 = v5541[3];
            let v5752 = v5541[4];
            let v5753 = v5548[0];
            let v5754 = v5548[1];
            let v5755 = v5548[2];
            let v5756 = v5555[0];
            let v5757 = v5555[1];
            let v5758 = v5555[2];
            let v5759 = v2678;
            let v5760 = v5559;
            let v5761 = v5575[0];
            let v5762 = v5575[1];
            let v5763 = v5575[2];
            let v5764 = v5575[3];
            let v5765 = v5575[4];
            let v5766 = v5575[5];
            let v5767 = v5575[6];
            let v5768 = v5575[7];
            let v5769 = v5575[8];
            let v5770 = v5575[9];
            let v5771 = v5575[10];
            let v5772 = v5575[11];
            let v5773 = v5582[0];
            let v5774 = v5582[1];
            let v5775 = v5582[2];
            let v5776 = v5582[3];
            let v5777 = v5582[4];
            let v5778 = v5586[0];
            let v5779 = v5586[1];
            let v5780 = v5586[2];
            let v5781 = v5594[0];
            let v5782 = v5594[1];
            let v5783 = v5594[2];
            let v5784 = v5594[3];
            let v5785 = v5594[4];
            let v5786 = v5598[0];
            let v5787 = v5598[1];
            let v5788 = v5598[2];
            let v5789 = v5602[0];
            let v5790 = v5602[1];
            let v5791 = v5602[2];
            let v5792 = v5602[3];
            let v5793 = v5602[4];
            let v5794 = v5602[5];
            let v5795 = v5606[0];
            let v5796 = v5606[1];
            let v5797 = v5610[0];
            let v5798 = v5610[1];
            let v5799 = v5613[0];
            let v5800 = v5613[1];
            let v5801 = v5613[2];
            let v5802 = v5613[3];
            let v5803 = v5613[4];
            let v5804 = v5613[5];
            let v5805 = v5613[6];
            let v5806 = v5613[7];
            let v5807 = v5613[8];
            let v5808 = v5613[9];
            let v5809 = v5620[0];
            let v5810 = v5620[1];
            let v5811 = v5620[2];
            let v5812 = v5620[3];
            let v5813 = v5620[4];
            let v5814 = v5620[5];
            let v5815 = v5620[6];
            let v5816 = v5620[7];
            let v5817 = v5620[8];
            let v5818 = v5625[0];
            let v5819 = v5625[1];
            let v5820 = v5625[2];
            let v5821 = v5625[3];
            let v5822 = v5625[4];
            let v5823 = v5625[5];
            let v5824 = v5625[6];
            let v5825 = v5625[7];
            let v5826 = v5625[8];
            let v5827 = v5625[9];
            let v5828 = v5630[0];
            let v5829 = v5630[1];
            let v5830 = v5630[2];
            let v5831 = v5630[3];
            let v5832 = v5630[4];
            let v5833 = v5630[5];
            let v5834 = v5634[0];
            let v5835 = v5634[1];
            let v5836 = v5634[2];
            let v5837 = v5634[3];
            let v5838 = v5634[4];
            let v5839 = v5634[5];
            let v5840 = v2681[0];
            let v5841 = v2681[1];
            let v5842 = v2681[2];
            let v5843 = v2682[0];
            let v5844 = v2682[1];
            let v5845 = v2682[2];
            let v5846 = v2539;
            let v5847 = v5681[0];
            let v5848 = v5681[1];
            let v5849 = v5681[2];
            let v5850 = v5681[3];
            let v5851 = v5681[4];
            let v5852 = v5681[5];
            let v5853 = v5691[0];
            let v5854 = v5691[1];
            let v5855 = v5691[2];
            let v5856 = v5691[3];
            let v5857 = v5691[4];
            let v5858 = v5691[5];
            let v5859 = v5560;
            let v5860 = v5583[0];
            let v5861 = v5583[1];
            let v5862 = v5583[2];
            let v5863 = v5583[3];
            let v5864 = v5583[4];
            let v5865 = v5587[0];
            let v5866 = v5587[1];
            let v5867 = v5587[2];
            let v5868 = v5595[0];
            let v5869 = v5595[1];
            let v5870 = v5595[2];
            let v5871 = v5595[3];
            let v5872 = v5595[4];
            let v5873 = v5599[0];
            let v5874 = v5599[1];
            let v5875 = v5599[2];
            let v5876 = v5603[0];
            let v5877 = v5603[1];
            let v5878 = v5603[2];
            let v5879 = v5603[3];
            let v5880 = v5603[4];
            let v5881 = v5603[5];
            let v5882 = v5607[0];
            let v5883 = v5607[1];
            let v5884 = v5611[0];
            let v5885 = v5611[1];
            let v5886 = v5626[0];
            let v5887 = v5626[1];
            let v5888 = v5626[2];
            let v5889 = v5626[3];
            let v5890 = v5626[4];
            let v5891 = v5626[5];
            let v5892 = v5626[6];
            let v5893 = v5626[7];
            let v5894 = v5626[8];
            let v5895 = v5626[9];
            let v5896 = v5635[0];
            let v5897 = v5635[1];
            let v5898 = v5635[2];
            let v5899 = v5635[3];
            let v5900 = v5635[4];
            let v5901 = v5635[5];
            let v5902 = v5686[0];
            let v5903 = v5686[1];
            let v5904 = v5686[2];
            let v5905 = v5686[3];
            let v5906 = v5686[4];
            let v5907 = v5686[5];
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(9),
            multiplicity * (v2296),
            [4, 7, 8, 9],
            [v5692, v5693, v5694, v5695],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(5),
            multiplicity * (v2298),
            [4, 5, 7, 8, 9],
            [v5696, v5697, v5698, v5699, v5700],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(5),
            multiplicity * (v2300),
            [4, 5, 6],
            [v5701, v5702, v5703],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2302),
            [4, 5, 7, 8, 9],
            [v5704, v5705, v5706, v5707, v5708],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (v2499),
            [4, 6, 7, 8],
            [v5709, v5710, v5711, v5712],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (v2500),
            [4, 6, 7, 8],
            [v5713, v5714, v5715, v5716],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(3),
            multiplicity * (v2310),
            [3, 4, 6, 7, 8, 9, 11],
            [v5717, v5718, v5719, v5720, v5721, v5722, v5723],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (v2312),
            [3, 4, 7, 8, 9],
            [v5724, v5725, v5726, v5727, v5728],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(3),
            multiplicity * (v2314),
            [0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            [v5729, v5730, v5731, v5732, v5733, v5734, v5735, v5736, v5737, v5738],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(8),
            multiplicity * (v2316),
            [3, 4, 8],
            [v5739, v5740, v5741],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2318),
            [4, 5, 6, 7, 8, 9],
            [v5742, v5743, v5744, v5745, v5746, v5747],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (v2322),
            [4, 5, 7, 8, 9],
            [v5748, v5749, v5750, v5751, v5752],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(2),
            Some(5),
            multiplicity * (v2325),
            [2, 4, 5],
            [v5753, v5754, v5755],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(1),
            Some(6),
            multiplicity * (v2328),
            [1, 4, 6],
            [v5756, v5757, v5758],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v2356),
            [4],
            [v5759],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(4),
            None,
            multiplicity * (v2332),
            [4],
            [v5760],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<12, 0>(
            Some(4),
            None,
            multiplicity * (v2361),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [v5761, v5762, v5763, v5764, v5765, v5766, v5767, v5768, v5769, v5770, v5771, v5772],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2368),
            [4, 5, 7, 8, 9],
            [v5773, v5774, v5775, v5776, v5777],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(6),
            Some(5),
            multiplicity * (v2371),
            [4, 5, 6],
            [v5778, v5779, v5780],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (v2377),
            [4, 5, 7, 8, 9],
            [v5781, v5782, v5783, v5784, v5785],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(3),
            Some(8),
            multiplicity * (v2380),
            [3, 4, 8],
            [v5786, v5787, v5788],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2384),
            [4, 5, 6, 7, 8, 9],
            [v5789, v5790, v5791, v5792, v5793, v5794],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(2),
            multiplicity * (v2389),
            [1, 2],
            [v5795, v5796],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<2, 0>(
            Some(1),
            Some(0),
            multiplicity * (v2394),
            [0, 1],
            [v5797, v5798],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(10),
            multiplicity * (v2396),
            [0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            [v5799, v5800, v5801, v5802, v5803, v5804, v5805, v5806, v5807, v5808],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(10),
            multiplicity * (v2399),
            [0, 1, 4, 6, 7, 8, 9, 10, 11],
            [v5809, v5810, v5811, v5812, v5813, v5814, v5815, v5816, v5817],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(10),
            multiplicity * (v2404),
            [0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            [v5818, v5819, v5820, v5821, v5822, v5823, v5824, v5825, v5826, v5827],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(11),
            multiplicity * (v2408),
            [4, 6, 7, 8, 9, 11],
            [v5828, v5829, v5830, v5831, v5832, v5833],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(11),
            multiplicity * (v2413),
            [4, 6, 7, 8, 9, 11],
            [v5834, v5835, v5836, v5837, v5838, v5839],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<3, 0>(
            Some(10),
            Some(11),
            multiplicity * (v2501),
            [4, 10, 11],
            [v5840, v5841, v5842],
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
            [v5843, v5844, v5845],
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
            [v5846],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (v2455),
            [4, 5, 7, 8, 9, 12],
            [v5847, v5848, v5849, v5850, v5851, v5852],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(7),
            multiplicity * (v2456),
            [4, 5, 7, 8, 9, 12],
            [v5853, v5854, v5855, v5856, v5857, v5858],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(9),
            Some(5),
            multiplicity * (v2451),
            [12],
            [v5846],
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
        self.canonical_reactive[0] = v2296;
        self.canonical_reactive[1] = v2298;
        self.canonical_reactive[2] = v2300;
        self.canonical_reactive[3] = v2302;
        self.canonical_reactive[4] = v2499;
        self.canonical_reactive[5] = v2500;
        self.canonical_reactive[6] = v2310;
        self.canonical_reactive[7] = v2312;
        self.canonical_reactive[8] = v2314;
        self.canonical_reactive[9] = v2316;
        self.canonical_reactive[10] = v2318;
        self.canonical_reactive[11] = v2322;
        self.canonical_reactive[12] = v2325;
        self.canonical_reactive[13] = v2328;
        self.canonical_reactive[14] = v2356;
        self.canonical_reactive[15] = v2515;
        self.canonical_reactive[16] = v5859;
        self.canonical_reactive[17] = v2361;
        self.canonical_reactive[18] = v2516;
        self.canonical_reactive[19] = v5860;
        self.canonical_reactive[20] = v5861;
        self.canonical_reactive[21] = v5862;
        self.canonical_reactive[22] = v5863;
        self.canonical_reactive[23] = v5864;
        self.canonical_reactive[24] = v2517;
        self.canonical_reactive[25] = v5865;
        self.canonical_reactive[26] = v5866;
        self.canonical_reactive[27] = v5867;
        self.canonical_reactive[28] = v2518;
        self.canonical_reactive[29] = v5868;
        self.canonical_reactive[30] = v5869;
        self.canonical_reactive[31] = v5870;
        self.canonical_reactive[32] = v5871;
        self.canonical_reactive[33] = v5872;
        self.canonical_reactive[34] = v2519;
        self.canonical_reactive[35] = v5873;
        self.canonical_reactive[36] = v5874;
        self.canonical_reactive[37] = v5875;
        self.canonical_reactive[38] = v2520;
        self.canonical_reactive[39] = v5876;
        self.canonical_reactive[40] = v5877;
        self.canonical_reactive[41] = v5878;
        self.canonical_reactive[42] = v5879;
        self.canonical_reactive[43] = v5880;
        self.canonical_reactive[44] = v5881;
        self.canonical_reactive[45] = v2521;
        self.canonical_reactive[46] = v5882;
        self.canonical_reactive[47] = v5883;
        self.canonical_reactive[48] = v2522;
        self.canonical_reactive[49] = v5884;
        self.canonical_reactive[50] = v5885;
        self.canonical_reactive[51] = v2396;
        self.canonical_reactive[52] = v2399;
        self.canonical_reactive[53] = v2523;
        self.canonical_reactive[54] = v5886;
        self.canonical_reactive[55] = v5887;
        self.canonical_reactive[56] = v5888;
        self.canonical_reactive[57] = v5889;
        self.canonical_reactive[58] = v5890;
        self.canonical_reactive[59] = v5891;
        self.canonical_reactive[60] = v5892;
        self.canonical_reactive[61] = v5893;
        self.canonical_reactive[62] = v5894;
        self.canonical_reactive[63] = v5895;
        self.canonical_reactive[64] = v2408;
        self.canonical_reactive[65] = v2524;
        self.canonical_reactive[66] = v5896;
        self.canonical_reactive[67] = v5897;
        self.canonical_reactive[68] = v5898;
        self.canonical_reactive[69] = v5899;
        self.canonical_reactive[70] = v5900;
        self.canonical_reactive[71] = v5901;
        self.canonical_reactive[72] = v2501;
        self.canonical_reactive[73] = v2502;
        self.canonical_reactive[74] = v2503;
        self.canonical_reactive[75] = v2504;
        self.canonical_reactive[76] = v2450;
        self.canonical_reactive[77] = v2451;
        self.canonical_reactive[78] = v2525;
        self.canonical_reactive[79] = v5902;
        self.canonical_reactive[80] = v5903;
        self.canonical_reactive[81] = v5904;
        self.canonical_reactive[82] = v5905;
        self.canonical_reactive[83] = v5906;
        self.canonical_reactive[84] = v5907;
        self.canonical_reactive[85] = v2456;
        self.canonical_reactive[86] = v2451;
        self.canonical_reactive[87] = v2457;
        self.canonical_reactive[88] = v2458;
        self.canonical_reactive[89] = v2459;
        self.canonical_reactive[90] = v2460;
        self.canonical_reactive[91] = v2461;
        self.canonical_reactive[92] = v2462;
        self.canonical_reactive[93] = v2463;
        self.canonical_reactive[94] = v2464;
        self.canonical_reactive[95] = v2465;
        self.canonical_reactive[96] = v2466;
        self.canonical_reactive[97] = v2467;
        self.canonical_reactive[98] = v2468;
        self.canonical_reactive[99] = v2469;
        self.canonical_reactive[100] = v2470;
        self.canonical_reactive[101] = v2505;
        self.canonical_reactive[102] = v2506;
        self.canonical_reactive[103] = v2473;
        self.canonical_reactive[104] = v2474;
        self.canonical_reactive[105] = v2475;
        self.canonical_reactive[106] = v2484;
        self.canonical_reactive[107] = v2507;
        self.canonical_reactive[108] = v2509;
        self.canonical_reactive[109] = v2486;
        self.canonical_reactive[110] = v2511;
        self.canonical_reactive[111] = v2488;
        self.canonical_reactive[112] = v2513;
        self.canonical_reactive[113] = v2490;
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let multiplicity = self.multiplicity;
        let cached = &*self.canonical_reactive;
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(4),
            None,
            &[4],
            &[cached[16]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[4, 5, 7, 8, 9],
            &[cached[19], cached[20], cached[21], cached[22], cached[23]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(5),
            &[4, 5, 6],
            &[cached[25], cached[26], cached[27]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(9),
            &[4, 5, 7, 8, 9],
            &[cached[29], cached[30], cached[31], cached[32], cached[33]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(3),
            Some(8),
            &[3, 4, 8],
            &[cached[35], cached[36], cached[37]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(7),
            &[4, 5, 6, 7, 8, 9],
            &[cached[39], cached[40], cached[41], cached[42], cached[43], cached[44]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(2),
            &[1, 2],
            &[cached[46], cached[47]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(0),
            &[0, 1],
            &[cached[49], cached[50]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(1),
            Some(10),
            &[0, 1, 3, 4, 6, 7, 8, 9, 10, 11],
            &[cached[54], cached[55], cached[56], cached[57], cached[58], cached[59], cached[60], cached[61], cached[62], cached[63]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(6),
            Some(11),
            &[4, 6, 7, 8, 9, 11],
            &[cached[66], cached[67], cached[68], cached[69], cached[70], cached[71]],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_indexed_dense_local(
            Some(7),
            Some(5),
            &[4, 5, 7, 8, 9, 12],
            &[cached[79], cached[80], cached[81], cached[82], cached[83], cached[84]],
            &[],
            &[],
            multiplicity,
        );
    }

}

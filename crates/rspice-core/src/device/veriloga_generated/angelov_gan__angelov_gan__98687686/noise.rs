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
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 16] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_GGI_GDI_RGD", label: Some("Rgd"), kind: GeneratedNoiseKind::White, equation: 25, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "ggi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_GGI_RG", label: Some("Rg"), kind: GeneratedNoiseKind::White, equation: 29, is_current: false, branch_ordinal: Some(8), pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(13), name: "ggi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_SII_RS", label: Some("Rs"), kind: GeneratedNoiseKind::White, equation: 33, is_current: false, branch_ordinal: Some(12), pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "sii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_DII_RD", label: Some("Rd"), kind: GeneratedNoiseKind::White, equation: 37, is_current: false, branch_ordinal: Some(16), pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "dii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS_NOISE", label: Some("Ids noise"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_IDS_FLICKER", label: Some("Ids flicker"), kind: GeneratedNoiseKind::Flicker, equation: 44, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IA_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(17), name: "ia", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IB_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(18), name: "ib", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_DRAIN", label: Some("drain"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GI_SI_GATE", label: Some("gate"), kind: GeneratedNoiseKind::Flicker, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GSI_SI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GDI_DI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GSI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GDI_DI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16]), ctx.node_voltage(self.nodes[17]), ctx.node_voltage(self.nodes[18])];
            let v0 = 0e0f64;
            let v1 = node_potentials[12];
            let v2 = node_potentials[8];
            let v4 = node_potentials[10];
            let v5 = node_potentials[5];
            let v9 = node_potentials[11];
            let v11 = node_potentials[4];
            let v13 = node_potentials[16];
            let v14 = if parameter_given[3] { 1.0 } else { 0.0 };
            let v15 = parameters[3];
            let v16 = 2.7315e2f64;
            let v18 = temperature;
            let v19 = parameters[2];
            let v21 = if parameter_given[100] { 1.0 } else { 0.0 };
            let v22 = parameters[100];
            let v24 = 3.0015e2f64;
            let v25 = parameters[1];
            let v27 = node_potentials[3];
            let v31 = 8.617333262e-5f64;
            let v37 = parameters[66];
            let v40 = 1e0f64;
            let v42 = parameters[8];
            let v43 = parameters[68];
            let v47 = parameters[20];
            let v48 = parameters[80];
            let v52 = parameters[26];
            let v53 = parameters[72];
            let v57 = parameters[58];
            let v58 = parameters[9];
            let v59 = parameters[78];
            let v62 = parameters[45];
            let v63 = parameters[79];
            let v66 = parameters[21];
            let v67 = parameters[81];
            let v70 = parameters[4];
            let v72 = 4e0f64;
            let v75 = parameters[6];
            let v78 = parameters[62];
            let v79 = parameters[63];
            let v80 = if parameter_given[43] { 1.0 } else { 0.0 };
            let v82 = if parameter_given[44] { 1.0 } else { 0.0 };
            let v84 = 5e-1f64;
            let v85 = parameters[44];
            let v88 = parameters[43];
            let v89 = parameters[19];
            let v92 = parameters[64];
            let v94 = parameters[11];
            let v95 = parameters[18];
            let v96 = 1e-12f64;
            let v102 = parameters[69];
            let v107 = parameters[13];
            let v108 = parameters[70];
            let v113 = parameters[10];
            let v115 = parameters[15];
            let v121 = parameters[22];
            let v133 = parameters[12];
            let v148 = parameters[14];
            let v157 = parameters[16];
            let v177 = parameters[17];
            let v185 = parameters[23];
            let v200 = 2e0f64;
            let v228 = 3e0f64;
            let v294 = parameters[65];
            let v311 = parameters[47];
            let v312 = parameters[48];
            let v315 = parameters[50];
            let v325 = parameters[76];
            let v331 = parameters[5];
            let v334 = -1e0f64;
            let v342 = parameters[83];
            let v345 = parameters[84];
            let v350 = parameters[85];
            let v371 = parameters[42];
            let v376 = 1e-3f64;
            let v377 = parameters[82];
            let v410 = parameters[60];
            let v412 = parameters[51];
            let v414 = parameters[0];
            let v415 = 1.3806503e-23f64;
            let v416 = 5.5226012e-23f64;
            let v419 = parameters[49];
            let v421 = parameters[46];
            let v423 = 5.5226012e-23f64;
            let v427 = 5.5226012e-23f64;
            let v433 = 5.5226012e-23f64;
            let v439 = parameters[7];
            let v444 = parameters[93];
            let v446 = parameters[95];
            let v455 = parameters[99];
            let v461 = parameters[94];
            let v468 = parameters[96];
            let v470 = parameters[98];
            let v472 = 5.5226012e-23f64;
            let v475 = parameters[87];
            let v482 = parameters[86];
            let v485 = 5.5226012e-23f64;
            let v487 = parameters[88];
            let v494 = 5.5226012e-23f64;
            let v498 = parameters[89];
            let v500 = parameters[90];
            let v502 = parameters[91];
            let v505 = parameters[92];
            let v507 = 3.204352924e-19f64;
            let v510 = 3.204352924e-19f64;
            let v607 = 1e0f64;
            let v608 = Lanes([1e0f64; 1]);
            let v609 = Lanes([1e0f64; 1]);
            let v610 = Lanes([1e0f64; 1]);
            let v611 = Lanes([1e0f64; 1]);
            let v612 = Lanes([1e0f64; 1]);
            let v613 = Lanes([1e0f64; 1]);
            let v630 = -1e0f64;
            let v638 = Lanes([0e0f64; 1]);
            let v639 = 0e0f64;
            let v641 = 2e0f64;
            let v745 = Lanes([0e0f64; 6]);
            let v3 = v1 - v2;
            let v626 = (Lanes([0.0, v608[0]])) - (Lanes([v609[0], 0.0]));
            let v6 = v4 - v5;
            let v629 = (Lanes([0.0, v610[0]])) - (Lanes([v611[0], 0.0]));
            let v7 = -v6;
            let v631 = v629 * v630;
            let v8 = v5 - v2;
            let v634 = (Lanes([v611[0], 0.0])) - (Lanes([0.0, v609[0]]));
            let v10 = v9 - v2;
            let v12 = v11 - v2;
            let v637 = (Lanes([v612[0], 0.0])) - (Lanes([0.0, v609[0]]));
            let v26: f64;
            if v14 != 0.0 {
                let v17 = v15 + v16;
                v26 = v17;
            } else {
                let v20 = v18 + v19;
                v26 = v20;
            }
            let v33: f64;
            if v21 != 0.0 {
                let v23 = v22 + v16;
                v33 = v23;
            } else {
                v33 = v24;
            }
            let v30: f64;
            let v614: Lanes<1>;
            if v25 != 0.0 {
                let v644 = v613 * ((v641 * (if v27 >= v639 { 1.0 } else { 0.0 })) - v607);
                let v29 = v26 + (v27.abs());
                v30 = v29;
                v614 = v644;
            } else {
                v30 = v26;
                v614 = v638;
            }
            let v32 = v30 * v31;
            let v34 = v30 - v33;
            let v35 = v34.abs();
            let v648 = v614 * ((v641 * (if v34 >= v639 { 1.0 } else { 0.0 })) - v607);
            let v39 = if (if v35 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v37 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v112: f64;
            let v122: f64;
            let v154: f64;
            let v160: f64;
            let v335: f64;
            let v399: f64;
            let v615: Lanes<1>;
            let v616: Lanes<1>;
            let v617: Lanes<1>;
            let v618: Lanes<1>;
            if v39 != 0.0 {
                let v41 = v35.abs();
                let v652 = v648 * ((v641 * (if v35 >= v639 { 1.0 } else { 0.0 })) - v607);
                let v46 = v42 * (v40 + (v43 * v41));
                let v654 = (v652 * v43) * v42;
                let v51 = v47 * (v40 + (v48 * v41));
                let v656 = (v652 * v48) * v47;
                let v56 = v52 * (v40 + (v53 * v41));
                let v657 = v648 * v59;
                let v61 = v58 + (v59 * v35);
                let v65 = v62 + (v63 * v35);
                let v658 = v648 * v67;
                let v69 = v66 + (v67 * v35);
                let v77 = if (if (if v70 == v40 { 1.0 } else { 0.0 }) != 0.0 || (if v70 == v72 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v75 == v72 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v77 != 0.0 {
                } else {
                }
                v112 = v61;
                v122 = v69;
                v154 = v46;
                v160 = v51;
                v335 = v65;
                v399 = v56;
                v615 = v657;
                v616 = v658;
                v617 = v654;
                v618 = v656;
            } else {
                v112 = v58;
                v122 = v66;
                v154 = v42;
                v160 = v47;
                v335 = v62;
                v399 = v52;
                v615 = v638;
                v616 = v638;
                v617 = v638;
                v618 = v638;
            }
            let v83 = if (if v80 == 0.0 { 1.0 } else { 0.0 }) != 0.0 && v82 != 0.0 { 1.0 } else { 0.0 };
            let v333: f64;
            if v83 != 0.0 {
                let v87 = (v84 / v85) / v32;
                v333 = v87;
            } else {
                v333 = v88;
            }
            let v90 = v89 * v8;
            let v91 = v90.cosh();
            let v662 = v637 * v92;
            let v663 = ((v634 * v89) * (v90.sinh())) * v91;
            let v98 = v96 + (v91 * v91);
            let v99 = v95 / v98;
            let v101 = v94 * (v40 + v99);
            let v103 = v35.abs();
            let v672 = v648 * ((v641 * (if v35 >= v639 { 1.0 } else { 0.0 })) - v607);
            let v105 = v40 + (v102 * v103);
            let v106 = v101 * v105;
            let v674 = (((((v663 + v663) * v99) * v630) / v98) * v94) * v105;
            let v675 = (v672 * v102) * v101;
            let v678 = (Lanes([0.0, v674[0], v674[1]])) + (Lanes([v675[0], 0.0, 0.0]));
            let v111 = v107 * (v40 + (v108 * v103));
            let v680 = (v672 * v108) * v107;
            let v117 = (v115 * v8).tanh();
            let v685 = ((v634 * v115) * (v607 - (v117 * v117))) * v113;
            let v688 = (Lanes([v615[0], 0.0, 0.0])) + (Lanes([0.0, v685[0], v685[1]]));
            let v691 = (Lanes([v688[0], 0.0, v688[1], v688[2]])) - (Lanes([0.0, v662[0], 0.0, v662[1]]));
            let v123 = v7 - v122;
            let v694 = (Lanes([0.0, v631[0], v631[1]])) - (Lanes([v616[0], 0.0, 0.0]));
            let v124 = v121 * v123;
            let v698 = ((v694 * v121) * v123) + (v694 * v124);
            let v126 = (((v112 - v113) + (v113 * v117)) - (v92 * v12)) - (v124 * v123);
            let v128 = v40 + (v59 * v103);
            let v129 = v126 * v128;
            let v704 = (v672 * v59) * v126;
            let v706 = (((Lanes([v691[0], v691[1], v691[2], v691[3], 0.0])) - (Lanes([v698[0], 0.0, v698[1], 0.0, v698[2]]))) * v128) + (Lanes([v704[0], 0.0, 0.0, 0.0, 0.0]));
            let v130 = v3 - v129;
            let v709 = (Lanes([0.0, 0.0, 0.0, v626[0], 0.0, v626[1]])) - (Lanes([v706[0], v706[1], v706[2], v706[3], v706[4], 0.0]));
            let v131 = v130 * v130;
            let v710 = v709 * v130;
            let v711 = v710 + v710;
            let v712 = v678 * v130;
            let v134 = v133 * v131;
            let v716 = v711 * v133;
            let v136 = v111 * v130;
            let v718 = v680 * v130;
            let v138 = ((v106 * v130) + v134) + (v136 * v131);
            let v725 = (((Lanes([v712[0], 0.0, v712[1], v712[2], 0.0, 0.0])) + (v709 * v106)) + v716) + ((((Lanes([v718[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v709 * v111)) * v131) + (v711 * v136));
            let v139 = v138.tanh();
            let v728 = v725 * (v607 - (v139 * v139));
            let v140 = v40 + v139;
            let v141 = rspice_limexp(v138);
            let v143 = rspice_limexp((-v138));
            let v146 = (v84 * (v141 - v143)).tanh();
            let v736 = (((v725 * v141) - ((v725 * v630) * v143)) * v84) * (v607 - (v146 * v146));
            let v147 = v40 + v146;
            let v150 = v148 + (v115 * v140);
            let v739 = v634 * v150;
            let v152 = (v150 * v8).tanh();
            let v744 = (((v728 * v115) * v8) + (Lanes([0.0, 0.0, v739[0], v739[1], 0.0, 0.0]))) * (v607 - (v152 * v152));
            let v153 = if v70 == v0 { 1.0 } else { 0.0 };
            let v317: f64;
            let v619: Lanes<6>;
            if v153 != 0.0 {
                let v155 = v154 * v140;
                let v1029 = v617 * v140;
                let v1036 = v634 * v157;
                let v161 = rspice_limexp(v123);
                let v1038 = v618 * v161;
                let v1041 = (Lanes([v1038[0], 0.0, 0.0])) + ((v694 * v161) * v160);
                let v1046 = ((Lanes([0.0, v1036[0], v1036[1], 0.0])) + (Lanes([v1041[0], v1041[1], 0.0, v1041[2]]))) * (v155 * v152);
                let v1048 = (((((Lanes([v1029[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v728 * v154)) * v152) + (v744 * v155)) * ((v40 + (v157 * v8)) + (v160 * v161))) + (Lanes([v1046[0], 0.0, v1046[1], v1046[2], v1046[3], 0.0]));
                v317 = v147;
                v619 = v1048;
            } else {
                let v164 = if v70 == v40 { 1.0 } else { 0.0 };
                let v318: f64;
                let v620: Lanes<6>;
                if v164 != 0.0 {
                    let v165 = v6 - v129;
                    let v955 = (Lanes([0.0, 0.0, v629[0], 0.0, v629[1]])) - v706;
                    let v166 = v165 * v165;
                    let v956 = v955 * v165;
                    let v957 = v956 + v956;
                    let v167 = v166 * v165;
                    let v961 = v678 * v165;
                    let v967 = v680 * v167;
                    let v173 = (((v106 * v165) + (v133 * v166)) + (v111 * v167)).tanh();
                    let v974 = ((((Lanes([v961[0], 0.0, v961[1], v961[2], 0.0])) + (v955 * v106)) + (v957 * v133)) + ((Lanes([v967[0], 0.0, 0.0, 0.0, 0.0])) + (((v957 * v165) + (v955 * v166)) * v111))) * (v607 - (v173 * v173));
                    let v174 = v40 + v173;
                    let v176 = v148 + (v115 * v174);
                    let v179 = v157 + (v177 * v140);
                    let v180 = v154 * v140;
                    let v977 = v617 * v140;
                    let v181 = v40 + v152;
                    let v985 = v634 * v179;
                    let v188 = rspice_limexp((v185 * (v8 - v122)));
                    let v993 = v618 * v188;
                    let v996 = (Lanes([v993[0], 0.0, 0.0])) + (((((Lanes([0.0, v634[0], v634[1]])) - (Lanes([v616[0], 0.0, 0.0]))) * v185) * v188) * v160);
                    let v192 = v157 + (v177 * v174);
                    let v1004 = v634 * v176;
                    let v194 = (v176 * v8).tanh();
                    let v195 = v154 * v174;
                    let v1010 = v617 * v174;
                    let v196 = v40 - v194;
                    let v1019 = v634 * v192;
                    let v1025 = (((((Lanes([v1010[0], 0.0, 0.0, 0.0, 0.0])) + (v974 * v154)) * v196) + ((((((v974 * v115) * v8) + (Lanes([0.0, 0.0, v1004[0], v1004[1], 0.0]))) * (v607 - (v194 * v194))) * v630) * v195)) * (v40 - (v192 * v8))) + (((((v974 * v177) * v8) + (Lanes([0.0, 0.0, v1019[0], v1019[1], 0.0]))) * v630) * (v195 * v196));
                    let v1028 = (((((((Lanes([v977[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v728 * v154)) * v181) + (v744 * v180)) * ((v40 + (v179 * v8)) + (v160 * v188))) + (((((v728 * v177) * v8) + (Lanes([0.0, 0.0, v985[0], v985[1], 0.0, 0.0]))) + (Lanes([v996[0], 0.0, v996[1], v996[2], 0.0, 0.0]))) * (v180 * v181))) - (Lanes([v1025[0], v1025[1], v1025[2], v1025[3], v1025[4], 0.0]))) * v84;
                    v318 = v147;
                    v620 = v1028;
                } else {
                    let v201 = if v70 == v200 { 1.0 } else { 0.0 };
                    let v319: f64;
                    let v621: Lanes<6>;
                    if v201 != 0.0 {
                        let v203 = v111 * v131;
                        let v903 = v680 * v131;
                        let v205 = (v130 + v134) + (v203 * v130);
                        let v206 = v106 * v205;
                        let v911 = v678 * v205;
                        let v914 = (Lanes([v911[0], 0.0, v911[1], v911[2], 0.0, 0.0])) + (((v709 + v716) + ((((Lanes([v903[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v711 * v111)) * v130) + (v709 * v203))) * v106);
                        let v207 = rspice_limexp(v206);
                        let v209 = rspice_limexp((-v206));
                        let v212 = (v84 * (v207 - v209)).tanh();
                        let v922 = (((v914 * v207) - ((v914 * v630) * v209)) * v84) * (v607 - (v212 * v212));
                        let v213 = v40 + v212;
                        let v215 = v148 + (v115 * v213);
                        let v925 = v634 * v215;
                        let v217 = (v215 * v8).tanh();
                        let v219 = v157 + (v177 * v213);
                        let v220 = v154 * v213;
                        let v932 = v617 * v213;
                        let v940 = v634 * v219;
                        let v225 = rspice_limexp((v185 * v123));
                        let v945 = v618 * v225;
                        let v948 = (Lanes([v945[0], 0.0, 0.0])) + (((v694 * v185) * v225) * v160);
                        let v953 = (((((Lanes([v932[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v922 * v154)) * v217) + (((((v922 * v115) * v8) + (Lanes([0.0, 0.0, v925[0], v925[1], 0.0, 0.0]))) * (v607 - (v217 * v217))) * v220)) * ((v40 + (v219 * v8)) + (v160 * v225))) + (((((v922 * v177) * v8) + (Lanes([0.0, 0.0, v940[0], v940[1], 0.0, 0.0]))) + (Lanes([v948[0], 0.0, v948[1], 0.0, v948[2], 0.0]))) * (v220 * v217));
                        v319 = v213;
                        v621 = v953;
                    } else {
                        let v229 = if v70 == v228 { 1.0 } else { 0.0 };
                        let v320: f64;
                        let v622: Lanes<6>;
                        if v229 != 0.0 {
                            let v231 = v111 * v131;
                            let v794 = v680 * v131;
                            let v233 = (v130 + v134) + (v231 * v130);
                            let v234 = v106 * v233;
                            let v802 = v678 * v233;
                            let v805 = (Lanes([v802[0], 0.0, v802[1], v802[2], 0.0, 0.0])) + (((v709 + v716) + ((((Lanes([v794[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v711 * v111)) * v130) + (v709 * v231))) * v106);
                            let v235 = v6 - v129;
                            let v807 = (Lanes([0.0, 0.0, v629[0], 0.0, v629[1]])) - v706;
                            let v236 = v235 * v235;
                            let v808 = v807 * v235;
                            let v809 = v808 + v808;
                            let v239 = v111 * v235;
                            let v812 = v680 * v235;
                            let v241 = (v235 + (v133 * v236)) + (v239 * v236);
                            let v242 = v106 * v241;
                            let v820 = v678 * v241;
                            let v823 = (Lanes([v820[0], 0.0, v820[1], v820[2], 0.0])) + (((v807 + (v809 * v133)) + ((((Lanes([v812[0], 0.0, 0.0, 0.0, 0.0])) + (v807 * v111)) * v236) + (v809 * v239))) * v106);
                            let v243 = rspice_limexp(v234);
                            let v245 = rspice_limexp((-v234));
                            let v248 = (v84 * (v243 - v245)).tanh();
                            let v831 = (((v805 * v243) - ((v805 * v630) * v245)) * v84) * (v607 - (v248 * v248));
                            let v249 = v40 + v248;
                            let v250 = rspice_limexp(v242);
                            let v252 = rspice_limexp((-v242));
                            let v255 = (v84 * (v250 - v252)).tanh();
                            let v839 = (((v823 * v250) - ((v823 * v630) * v252)) * v84) * (v607 - (v255 * v255));
                            let v256 = v40 + v255;
                            let v258 = v148 + (v115 * v249);
                            let v260 = v148 + (v115 * v256);
                            let v843 = v634 * v258;
                            let v262 = (v258 * v8).tanh();
                            let v850 = v634 * v260;
                            let v264 = (v260 * v8).tanh();
                            let v266 = v157 + (v177 * v256);
                            let v268 = v157 + (v177 * v249);
                            let v269 = v154 * v249;
                            let v858 = v617 * v249;
                            let v270 = v40 + v262;
                            let v866 = v634 * v268;
                            let v276 = rspice_limexp((v185 * (v8 - v122)));
                            let v874 = v618 * v276;
                            let v877 = (Lanes([v874[0], 0.0, 0.0])) + (((((Lanes([0.0, v634[0], v634[1]])) - (Lanes([v616[0], 0.0, 0.0]))) * v185) * v276) * v160);
                            let v279 = v154 * v256;
                            let v883 = v617 * v256;
                            let v280 = v40 - v264;
                            let v892 = v634 * v266;
                            let v898 = (((((Lanes([v883[0], 0.0, 0.0, 0.0, 0.0])) + (v839 * v154)) * v280) + ((((((v839 * v115) * v8) + (Lanes([0.0, 0.0, v850[0], v850[1], 0.0]))) * (v607 - (v264 * v264))) * v630) * v279)) * (v40 - (v266 * v8))) + (((((v839 * v177) * v8) + (Lanes([0.0, 0.0, v892[0], v892[1], 0.0]))) * v630) * (v279 * v280));
                            let v901 = (((((((Lanes([v858[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v831 * v154)) * v270) + (((((v831 * v115) * v8) + (Lanes([0.0, 0.0, v843[0], v843[1], 0.0, 0.0]))) * (v607 - (v262 * v262))) * v269)) * ((v40 + (v268 * v8)) + (v160 * v276))) + (((((v831 * v177) * v8) + (Lanes([0.0, 0.0, v866[0], v866[1], 0.0, 0.0]))) + (Lanes([v877[0], 0.0, v877[1], v877[2], 0.0, 0.0]))) * (v269 * v270))) - (Lanes([v898[0], v898[1], v898[2], v898[3], v898[4], 0.0]))) * v84;
                            v320 = v249;
                            v622 = v901;
                        } else {
                            let v284 = if v70 == v72 { 1.0 } else { 0.0 };
                            let v623: Lanes<6>;
                            if v284 != 0.0 {
                                let v286 = v157 + (v177 * v140);
                                let v747 = v736 * v115;
                                let v288 = v148 + (v115 * v147);
                                let v749 = v634 * v288;
                                let v290 = (v288 * v8).tanh();
                                let v756 = v637 * v288;
                                let v292 = (v288 * v12).tanh();
                                let v293 = v154 * v140;
                                let v762 = v617 * v140;
                                let v296 = v290 + (v294 * v292);
                                let v771 = v637 * v294;
                                let v299 = v8 + (v294 * v12);
                                let v776 = ((Lanes([0.0, v634[0], v634[1]])) + (Lanes([v771[0], 0.0, v771[1]]))) * v286;
                                let v304 = rspice_limexp((v185 * (v8 - v122)));
                                let v784 = v618 * v304;
                                let v787 = (Lanes([v784[0], 0.0, 0.0])) + (((((Lanes([0.0, v634[0], v634[1]])) - (Lanes([v616[0], 0.0, 0.0]))) * v185) * v304) * v160);
                                let v792 = (((((Lanes([v762[0], 0.0, 0.0, 0.0, 0.0, 0.0])) + (v728 * v154)) * v296) + (((((v747 * v8) + (Lanes([0.0, 0.0, v749[0], v749[1], 0.0, 0.0]))) * (v607 - (v290 * v290))) + ((((v747 * v12) + (Lanes([0.0, v756[0], 0.0, v756[1], 0.0, 0.0]))) * (v607 - (v292 * v292))) * v294)) * v293)) * ((v40 + (v286 * v299)) + (v160 * v304))) + (((((v728 * v177) * v299) + (Lanes([0.0, v776[0], v776[1], v776[2], 0.0, 0.0]))) + (Lanes([v787[0], 0.0, v787[1], v787[2], 0.0, 0.0]))) * (v293 * v296));
                                v623 = v792;
                            } else {
                                v623 = v745;
                            }
                            v320 = v147;
                            v622 = v623;
                        }
                        v319 = v320;
                        v621 = v622;
                    }
                    v318 = v319;
                    v620 = v621;
                }
                v317 = v318;
                v619 = v620;
            }
            let v310 = if (if v153 != 0.0 || (if v70 == v40 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v70 == v72 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v324: f64;
            let v329: f64;
            if v310 != 0.0 {
                let v313 = v312 * v140;
                let v314 = v311 + v313;
                let v316 = v315 + v313;
                v324 = v316;
                v329 = v314;
            } else {
                let v321 = v312 * v317;
                let v322 = v311 + v321;
                let v323 = v315 + v321;
                v324 = v323;
                v329 = v322;
            }
            let v327 = v40 + (v325 * v103);
            let v328 = v324 * v327;
            let v330 = v329 * v327;
            let v332 = if v331 == v0 { 1.0 } else { 0.0 };
            let v366: f64;
            let v369: f64;
            let v372: f64;
            let v381: f64;
            let v384: f64;
            let v387: f64;
            let v389: f64;
            if v332 != 0.0 {
                let v339 = rspice_limexp((v333 * ((v334 * v335).tanh())));
                let v340 = v10 - v335;
                let v343 = (-v10) - v342;
                let v344 = v6 - v335;
                let v346 = v7 - v345;
                v366 = v343;
                v369 = v0;
                v372 = v340;
                v381 = v339;
                v384 = v346;
                v387 = v0;
                v389 = v344;
            } else {
                let v349 = rspice_limexp(((-v333) * v335));
                let v351 = -v350;
                let v353 = rspice_limexp((v351 * v342));
                let v355 = rspice_limexp((v351 * v345));
                let v356 = if v331 == v40 { 1.0 } else { 0.0 };
                let v373: f64;
                let v390: f64;
                if v356 != 0.0 {
                    let v358 = (v10 - v335).tanh();
                    let v360 = (v6 - v335).tanh();
                    v373 = v358;
                    v390 = v360;
                } else {
                    let v361 = v10 - v335;
                    let v362 = v6 - v335;
                    v373 = v361;
                    v390 = v362;
                }
                let v364 = (-v10) - v342;
                let v365 = v7 - v345;
                v366 = v364;
                v369 = v353;
                v372 = v373;
                v381 = v349;
                v384 = v365;
                v387 = v355;
                v389 = v390;
            }
            let v378 = v376 * v377;
            let v383 = v371 * (((rspice_limexp((v333 * v372))) - (v378 * ((rspice_limexp((v350 * v366))) - v369))) - v381);
            let v396 = v371 * (((rspice_limexp((v333 * v389))) - (v378 * ((rspice_limexp((v350 * v384))) - v387))) - v381);
            let v397 = if v75 == v0 { 1.0 } else { 0.0 };
            if v397 != 0.0 {
            } else {
                let v398 = if v75 == v40 { 1.0 } else { 0.0 };
                if v398 != 0.0 {
                } else {
                    let v400 = if v75 == v200 { 1.0 } else { 0.0 };
                    if v400 != 0.0 {
                    } else {
                        let v401 = if v75 == v228 { 1.0 } else { 0.0 };
                        if v401 != 0.0 {
                        } else {
                            let v402 = if v75 == v72 { 1.0 } else { 0.0 };
                            if v402 != 0.0 {
                            } else {
                            }
                        }
                    }
                }
            }
            let v405 = if (if v75 == v200 { 1.0 } else { 0.0 }) != 0.0 || (if v75 == v72 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v405 != 0.0 {
            } else {
            }
            let v406 = if v57 > v0 { 1.0 } else { 0.0 };
            if v406 != 0.0 {
            } else {
            }
            let v409 = if (if v79 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v78 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v409 != 0.0 {
            } else {
            }
            let v411 = if v410 > v0 { 1.0 } else { 0.0 };
            if v411 != 0.0 {
            } else {
            }
            let v413 = if v412 > v0 { 1.0 } else { 0.0 };
            if v413 != 0.0 {
            } else {
            }
            let v519: f64;
            let v520: f64;
            if v414 != 0.0 {
                let v418 = (v416 * v30) * v412;
                v519 = v40;
                v520 = v418;
            } else {
                v519 = v0;
                v520 = v0;
            }
            let v420 = if v419 > v0 { 1.0 } else { 0.0 };
            if v420 != 0.0 {
            } else {
            }
            let v422 = if v421 > v0 { 1.0 } else { 0.0 };
            let v521: f64;
            let v523: f64;
            if v422 != 0.0 {
                let v522: f64;
                let v524: f64;
                if v414 != 0.0 {
                    let v425 = (v423 * v30) * v421;
                    v522 = v40;
                    v524 = v425;
                } else {
                    v522 = v0;
                    v524 = v0;
                }
                v521 = v522;
                v523 = v524;
            } else {
                v521 = v0;
                v523 = v0;
            }
            let v426 = if v315 > v0 { 1.0 } else { 0.0 };
            let v525: f64;
            let v527: f64;
            if v426 != 0.0 {
                let v526: f64;
                let v528: f64;
                if v414 != 0.0 {
                    let v429 = (v427 * v30) * v328;
                    v526 = v40;
                    v528 = v429;
                } else {
                    v526 = v0;
                    v528 = v0;
                }
                v525 = v526;
                v527 = v528;
            } else {
                v525 = v0;
                v527 = v0;
            }
            let v432 = if (if v311 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v312 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v529: f64;
            let v531: f64;
            if v432 != 0.0 {
                let v530: f64;
                let v532: f64;
                if v414 != 0.0 {
                    let v435 = (v433 * v30) * v330;
                    v530 = v40;
                    v532 = v435;
                } else {
                    v530 = v0;
                    v532 = v0;
                }
                v529 = v530;
                v531 = v532;
            } else {
                v529 = v0;
                v531 = v0;
            }
            let v1049 = v619[5];
            let v438 = v1049 / (v40 + (v1049 * v315));
            let v440 = if v439 == v0 { 1.0 } else { 0.0 };
            let v533: f64;
            let v535: f64;
            let v537: f64;
            let v539: f64;
            let v541: f64;
            let v543: f64;
            let v546: f64;
            let v549: f64;
            let v552: f64;
            let v555: f64;
            let v558: f64;
            let v561: f64;
            let v564: f64;
            let v567: f64;
            let v570: f64;
            let v573: f64;
            let v576: f64;
            let v579: f64;
            let v583: f64;
            let v587: f64;
            if v440 != 0.0 {
                let v443 = (v13.abs()) + (v396.abs());
                let v467 = (((v455 * v72) * v415) * v30) * (((((((v444 + v16) * (v40 + (((v446 * v140) * (v152.abs())) * (v40 + (v157 * v8))))) / v30) * v443) + ((v461 * v443) * v443)).abs()).sqrt());
                let v534: f64;
                let v536: f64;
                let v538: f64;
                let v540: f64;
                let v542: f64;
                if v414 != 0.0 {
                    let v469 = v467 * v468;
                    v534 = v40;
                    v536 = v467;
                    v538 = v40;
                    v540 = v469;
                    v542 = v470;
                } else {
                    v534 = v0;
                    v536 = v0;
                    v538 = v0;
                    v540 = v0;
                    v542 = v0;
                }
                v533 = v534;
                v535 = v536;
                v537 = v538;
                v539 = v540;
                v541 = v542;
                v543 = v0;
                v546 = v0;
                v549 = v0;
                v552 = v0;
                v555 = v0;
                v558 = v0;
                v561 = v0;
                v564 = v0;
                v567 = v0;
                v570 = v0;
                v573 = v0;
                v576 = v0;
                v579 = v0;
                v583 = v0;
                v587 = v0;
            } else {
                let v471 = if v439 == v40 { 1.0 } else { 0.0 };
                let v544: f64;
                let v547: f64;
                let v550: f64;
                let v553: f64;
                let v556: f64;
                let v559: f64;
                let v562: f64;
                let v565: f64;
                let v568: f64;
                let v571: f64;
                let v574: f64;
                let v577: f64;
                let v580: f64;
                let v584: f64;
                let v588: f64;
                if v471 != 0.0 {
                    let v545: f64;
                    let v548: f64;
                    let v551: f64;
                    let v554: f64;
                    let v557: f64;
                    let v560: f64;
                    let v563: f64;
                    let v566: f64;
                    let v569: f64;
                    let v572: f64;
                    let v575: f64;
                    let v578: f64;
                    let v581: f64;
                    let v585: f64;
                    let v589: f64;
                    if v414 != 0.0 {
                        let v476 = ((v472 * v30) * v438) * v475;
                        let v477 = if v438 > v0 { 1.0 } else { 0.0 };
                        let v493: f64;
                        if v477 != 0.0 {
                            let v484 = (((((v399 * v399) * v72) * v415) * v30) * v482) / v438;
                            v493 = v484;
                        } else {
                            v493 = v0;
                        }
                        let v492 = (((v485 * v30) * v487) * v399) * ((v475 * v482).sqrt());
                        let v499 = (((v494 * v30) * v438) * v475) * v498;
                        let v501 = if v500 > v0 { 1.0 } else { 0.0 };
                        let v582: f64;
                        let v586: f64;
                        let v590: f64;
                        if v501 != 0.0 {
                            let v504 = v500 * (v13.powf(v502));
                            v582 = v40;
                            v586 = v504;
                            v590 = v505;
                        } else {
                            v582 = v0;
                            v586 = v0;
                            v590 = v0;
                        }
                        v545 = v40;
                        v548 = v492;
                        v551 = v40;
                        v554 = v492;
                        v557 = v40;
                        v560 = v476;
                        v563 = v40;
                        v566 = v493;
                        v569 = v200;
                        v572 = v40;
                        v575 = v499;
                        v578 = v40;
                        v581 = v582;
                        v585 = v586;
                        v589 = v590;
                    } else {
                        v545 = v0;
                        v548 = v0;
                        v551 = v0;
                        v554 = v0;
                        v557 = v0;
                        v560 = v0;
                        v563 = v0;
                        v566 = v0;
                        v569 = v0;
                        v572 = v0;
                        v575 = v0;
                        v578 = v0;
                        v581 = v0;
                        v585 = v0;
                        v589 = v0;
                    }
                    v544 = v545;
                    v547 = v548;
                    v550 = v551;
                    v553 = v554;
                    v556 = v557;
                    v559 = v560;
                    v562 = v563;
                    v565 = v566;
                    v568 = v569;
                    v571 = v572;
                    v574 = v575;
                    v577 = v578;
                    v580 = v581;
                    v584 = v585;
                    v588 = v589;
                } else {
                    let v506 = if v439 == v200 { 1.0 } else { 0.0 };
                    if v506 != 0.0 {
                    } else {
                    }
                    v544 = v0;
                    v547 = v0;
                    v550 = v0;
                    v553 = v0;
                    v556 = v0;
                    v559 = v0;
                    v562 = v0;
                    v565 = v0;
                    v568 = v0;
                    v571 = v0;
                    v574 = v0;
                    v577 = v0;
                    v580 = v0;
                    v584 = v0;
                    v588 = v0;
                }
                v533 = v0;
                v535 = v0;
                v537 = v0;
                v539 = v0;
                v541 = v0;
                v543 = v544;
                v546 = v547;
                v549 = v550;
                v552 = v553;
                v555 = v556;
                v558 = v559;
                v561 = v562;
                v564 = v565;
                v567 = v568;
                v570 = v571;
                v573 = v574;
                v576 = v577;
                v579 = v580;
                v583 = v584;
                v587 = v588;
            }
            let v591: f64;
            let v592: f64;
            let v593: f64;
            let v594: f64;
            let v595: f64;
            let v597: f64;
            let v599: f64;
            let v601: f64;
            let v603: f64;
            let v605: f64;
            if v414 != 0.0 {
                let v508 = v383.abs();
                let v509 = v507 * v508;
                let v511 = v396.abs();
                let v512 = v510 * v511;
                let v513 = if v500 > v0 { 1.0 } else { 0.0 };
                let v596: f64;
                let v598: f64;
                let v600: f64;
                let v602: f64;
                let v604: f64;
                let v606: f64;
                if v513 != 0.0 {
                    let v515 = v500 * (v508.powf(v502));
                    let v517 = v500 * (v511.powf(v502));
                    v596 = v40;
                    v598 = v515;
                    v600 = v505;
                    v602 = v40;
                    v604 = v517;
                    v606 = v505;
                } else {
                    v596 = v0;
                    v598 = v0;
                    v600 = v0;
                    v602 = v0;
                    v604 = v0;
                    v606 = v0;
                }
                v591 = v40;
                v592 = v509;
                v593 = v40;
                v594 = v512;
                v595 = v596;
                v597 = v598;
                v599 = v600;
                v601 = v602;
                v603 = v604;
                v605 = v606;
            } else {
                v591 = v0;
                v592 = v0;
                v593 = v0;
                v594 = v0;
                v595 = v0;
                v597 = v0;
                v599 = v0;
                v601 = v0;
                v603 = v0;
                v605 = v0;
            }
            let v518 = if v25 == v40 { 1.0 } else { 0.0 };
            if v518 != 0.0 {
            } else {
            }
        if v519 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v520;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v521 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v523;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v525 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v527;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v529 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v531;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v533 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v535;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v537 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v539;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v541);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v543 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v546;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v549 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v552;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v555 == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v558;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v561 == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v564;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v567);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v570 == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v573;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v576);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v579 == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v583;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v587);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v591 == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v592;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v593 == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v594;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v595 == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v597;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v599);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v601 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v603;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v605);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

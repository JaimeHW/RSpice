#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use super::state::Instance;
use rspice_veriloga_runtime::GeneratedEvalContext;
pub use rspice_veriloga_runtime::{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor};


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
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_GDI_RGD", label: Some("Rgd"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GI_RG", label: Some("Rg"), kind: GeneratedNoiseKind::White, equation: 26, is_current: false, branch_ordinal: Some(7), pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_SII_RS", label: Some("Rs"), kind: GeneratedNoiseKind::White, equation: 30, is_current: false, branch_ordinal: Some(11), pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "sii", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_D_RD", label: Some("Rd"), kind: GeneratedNoiseKind::White, equation: 35, is_current: false, branch_ordinal: Some(16), pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS_NOISE", label: Some("Ids noise"), kind: GeneratedNoiseKind::White, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_IDS_FLICKER", label: Some("Ids flicker"), kind: GeneratedNoiseKind::Flicker, equation: 39, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IA_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 40, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "ia", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_IB_GND_CORRELATED_NOISE", label: Some("correlated noise"), kind: GeneratedNoiseKind::White, equation: 42, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(15), name: "ib", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_DRAIN", label: Some("drain"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GI_SI_GATE", label: Some("gate"), kind: GeneratedNoiseKind::Flicker, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(4), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GSI_SI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GDI_DI_SHOT", label: Some("shot"), kind: GeneratedNoiseKind::White, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GSI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gsi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_GDI_DI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "gdi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15])];
            let v0 = 0e0f64;
            let v1 = node_potentials[8];
            let v2 = node_potentials[5];
            let v4 = node_potentials[4];
            let v5 = node_potentials[3];
            let v9 = node_potentials[7];
            let v11 = node_potentials[13];
            let v12 = if parameter_given[3] { 1.0 } else { 0.0 };
            let v13 = parameters[3];
            let v14 = 2.7315e2f64;
            let v16 = temperature;
            let v17 = parameters[2];
            let v19 = if parameter_given[85] { 1.0 } else { 0.0 };
            let v20 = parameters[85];
            let v22 = 3.0015e2f64;
            let v23 = parameters[1];
            let v25 = node_potentials[11];
            let v29 = 8.617333262e-5f64;
            let v35 = parameters[57];
            let v38 = 1e0f64;
            let v39 = parameters[8];
            let v40 = parameters[59];
            let v44 = parameters[11];
            let v45 = parameters[60];
            let v49 = parameters[20];
            let v50 = parameters[63];
            let v54 = parameters[25];
            let v55 = parameters[61];
            let v59 = parameters[53];
            let v60 = parameters[9];
            let v61 = parameters[68];
            let v64 = parameters[41];
            let v65 = parameters[69];
            let v68 = parameters[21];
            let v69 = parameters[70];
            let v72 = if parameter_given[39] { 1.0 } else { 0.0 };
            let v74 = if parameter_given[40] { 1.0 } else { 0.0 };
            let v76 = 5e-1f64;
            let v77 = parameters[40];
            let v80 = parameters[39];
            let v81 = parameters[19];
            let v85 = parameters[18];
            let v91 = parameters[10];
            let v93 = parameters[15];
            let v98 = parameters[22];
            let v108 = parameters[12];
            let v111 = parameters[13];
            let v124 = parameters[14];
            let v129 = parameters[4];
            let v134 = parameters[16];
            let v154 = parameters[17];
            let v175 = 2e0f64;
            let v202 = 3e0f64;
            let v259 = parameters[43];
            let v260 = parameters[44];
            let v263 = parameters[46];
            let v274 = parameters[66];
            let v279 = parameters[5];
            let v282 = -1e0f64;
            let v300 = parameters[38];
            let v314 = parameters[6];
            let v321 = parameters[55];
            let v323 = parameters[47];
            let v325 = parameters[0];
            let v326 = 4e0f64;
            let v327 = 1.3806503e-23f64;
            let v328 = 5.5226012e-23f64;
            let v331 = parameters[45];
            let v333 = parameters[42];
            let v335 = parameters[50];
            let v336 = 5.5226012e-23f64;
            let v342 = 5.5226012e-23f64;
            let v348 = parameters[48];
            let v349 = 5.5226012e-23f64;
            let v353 = parameters[7];
            let v358 = parameters[78];
            let v360 = parameters[80];
            let v369 = parameters[84];
            let v375 = parameters[79];
            let v382 = parameters[81];
            let v384 = parameters[83];
            let v386 = 5.5226012e-23f64;
            let v389 = parameters[72];
            let v396 = parameters[71];
            let v399 = 5.5226012e-23f64;
            let v401 = parameters[73];
            let v408 = 5.5226012e-23f64;
            let v412 = parameters[74];
            let v414 = parameters[75];
            let v416 = parameters[76];
            let v419 = parameters[77];
            let v421 = 3.204352924e-19f64;
            let v424 = 3.204352924e-19f64;
            let v523 = 1e0f64;
            let v524 = 1e0f64;
            let v525 = 1e0f64;
            let v526 = 1e0f64;
            let v527 = 1e0f64;
            let v528 = 1e0f64;
            let v545 = -1e0f64;
            let v550 = 0e0f64;
            let v551 = 0e0f64;
            let v553 = 2e0f64;
            let v628 = Lanes([0e0f64; 5]);
            let v3 = v1 - v2;
            let v541 = (Lanes([0.0, v524])) - (Lanes([v525, 0.0]));
            let v6 = v4 - v5;
            let v544 = (Lanes([0.0, v526])) - (Lanes([v527, 0.0]));
            let v7 = -v6;
            let v546 = v544 * v545;
            let v8 = v5 - v2;
            let v549 = (Lanes([v527, 0.0])) - (Lanes([0.0, v525]));
            let v10 = v9 - v5;
            let v24: f64;
            if v12 != 0.0 {
                let v15 = v13 + v14;
                v24 = v15;
            } else {
                let v18 = v16 + v17;
                v24 = v18;
            }
            let v31: f64;
            if v19 != 0.0 {
                let v21 = v20 + v14;
                v31 = v21;
            } else {
                v31 = v22;
            }
            let v28: f64;
            let v529: f64;
            if v23 != 0.0 {
                let v556 = v528 * ((v553 * (if v25 >= v551 { 1.0 } else { 0.0 })) - v523);
                let v27 = v24 + (v25.abs());
                v28 = v27;
                v529 = v556;
            } else {
                v28 = v24;
                v529 = v550;
            }
            let v30 = v28 * v29;
            let v32 = v28 - v31;
            let v33 = v32.abs();
            let v560 = v529 * ((v553 * (if v32 >= v551 { 1.0 } else { 0.0 })) - v523);
            let v36 = if v35 > v0 { 1.0 } else { 0.0 };
            let v37 = if (if v33 > v0 { 1.0 } else { 0.0 }) != 0.0 || v36 != 0.0 { 1.0 } else { 0.0 };
            let v84: f64;
            let v90: f64;
            let v101: f64;
            let v131: f64;
            let v137: f64;
            let v283: f64;
            let v317: f64;
            let v530: f64;
            let v531: f64;
            let v532: f64;
            let v533: f64;
            let v534: f64;
            if v37 != 0.0 {
                let v43 = v39 * (v38 + (v40 * v33));
                let v562 = (v560 * v40) * v39;
                let v48 = v44 * (v38 + (v45 * v33));
                let v564 = (v560 * v45) * v44;
                let v53 = v49 * (v38 + (v50 * v33));
                let v566 = (v560 * v50) * v49;
                let v58 = v54 * (v38 + (v55 * v33));
                let v567 = v560 * v61;
                let v63 = v60 + (v61 * v33);
                let v67 = v64 + (v65 * v33);
                let v568 = v560 * v69;
                let v71 = v68 + (v69 * v33);
                v84 = v48;
                v90 = v63;
                v101 = v71;
                v131 = v43;
                v137 = v53;
                v283 = v67;
                v317 = v58;
                v530 = v564;
                v531 = v567;
                v532 = v568;
                v533 = v562;
                v534 = v566;
            } else {
                v84 = v44;
                v90 = v60;
                v101 = v68;
                v131 = v39;
                v137 = v49;
                v283 = v64;
                v317 = v54;
                v530 = v550;
                v531 = v550;
                v532 = v550;
                v533 = v550;
                v534 = v550;
            }
            let v75 = if (if v72 == 0.0 { 1.0 } else { 0.0 }) != 0.0 && v74 != 0.0 { 1.0 } else { 0.0 };
            let v281: f64;
            if v75 != 0.0 {
                let v79 = (v76 / v77) / v30;
                v281 = v79;
            } else {
                v281 = v80;
            }
            let v82 = v81 * v8;
            let v83 = v82.cosh();
            let v86 = v83 * v83;
            let v572 = ((v549 * v81) * (v82.sinh())) * v83;
            let v87 = v85 / v86;
            let v88 = v38 + v87;
            let v89 = v84 * v88;
            let v578 = ((((v572 + v572) * v87) * v545) / v86) * v84;
            let v581 = (Lanes([0.0, 0.0, (v530 * v88)])) + (Lanes([v578[0], v578[1], 0.0]));
            let v95 = (v93 * v8).tanh();
            let v586 = ((v549 * v93) * (v523 - (v95 * v95))) * v91;
            let v589 = (Lanes([0.0, 0.0, v531])) + (Lanes([v586[0], v586[1], 0.0]));
            let v100 = v98 * (v7 - v68);
            let v102 = v7 - v101;
            let v593 = (Lanes([v546[0], v546[1], 0.0])) - (Lanes([0.0, 0.0, v532]));
            let v594 = (v546 * v98) * v102;
            let v597 = (Lanes([v594[0], v594[1], 0.0])) + (v593 * v100);
            let v104 = ((v90 - v91) + (v91 * v95)) - (v100 * v102);
            let v600 = (Lanes([v589[0], 0.0, v589[1], v589[2]])) - (Lanes([v597[0], v597[1], 0.0, v597[2]]));
            let v105 = v3 - v104;
            let v603 = (Lanes([0.0, 0.0, v541[0], v541[1], 0.0])) - (Lanes([v600[0], v600[1], v600[2], 0.0, v600[3]]));
            let v106 = v105 * v105;
            let v604 = v603 * v105;
            let v605 = v604 + v604;
            let v606 = v581 * v105;
            let v109 = v108 * v106;
            let v610 = v605 * v108;
            let v112 = v111 * v105;
            let v114 = ((v89 * v105) + v109) + (v112 * v106);
            let v115 = v114.tanh();
            let v619 = ((((Lanes([v606[0], 0.0, v606[1], 0.0, v606[2]])) + (v603 * v89)) + v610) + (((v603 * v111) * v106) + (v605 * v112))) * (v523 - (v115 * v115));
            let v116 = v38 + v115;
            let v123 = v38 + ((v76 * ((rspice_limexp(v114)) - (rspice_limexp((-v114))))).tanh());
            let v126 = v124 + (v93 * v116);
            let v622 = v549 * v126;
            let v128 = (v126 * v8).tanh();
            let v627 = (((v619 * v93) * v8) + (Lanes([v622[0], 0.0, v622[1], 0.0, 0.0]))) * (v523 - (v128 * v128));
            let v130 = if v129 == v0 { 1.0 } else { 0.0 };
            let v265: f64;
            let v535: Lanes<5>;
            if v130 != 0.0 {
                let v132 = v131 * v116;
                let v857 = v549 * v134;
                let v138 = rspice_limexp(v102);
                let v862 = (Lanes([0.0, 0.0, (v534 * v138)])) + ((v593 * v138) * v137);
                let v867 = ((Lanes([v857[0], 0.0, v857[1], 0.0])) + (Lanes([v862[0], v862[1], 0.0, v862[2]]))) * (v132 * v128);
                let v869 = (((((Lanes([0.0, 0.0, 0.0, 0.0, (v533 * v116)])) + (v619 * v131)) * v128) + (v627 * v132)) * ((v38 + (v134 * v8)) + (v137 * v138))) + (Lanes([v867[0], v867[1], v867[2], 0.0, v867[3]]));
                v265 = v123;
                v535 = v869;
            } else {
                let v141 = if v129 == v38 { 1.0 } else { 0.0 };
                let v266: f64;
                let v536: Lanes<5>;
                if v141 != 0.0 {
                    let v142 = v6 - v104;
                    let v780 = (Lanes([v544[0], v544[1], 0.0, 0.0])) - v600;
                    let v143 = v142 * v142;
                    let v781 = v780 * v142;
                    let v782 = v781 + v781;
                    let v786 = v581 * v142;
                    let v150 = (((v89 * v142) + (v108 * v143)) + (v111 * (v143 * v142))).tanh();
                    let v796 = ((((Lanes([v786[0], 0.0, v786[1], v786[2]])) + (v780 * v89)) + (v782 * v108)) + (((v782 * v142) + (v780 * v143)) * v111)) * (v523 - (v150 * v150));
                    let v151 = v38 + v150;
                    let v153 = v124 + (v93 * v151);
                    let v156 = v134 + (v154 * v116);
                    let v157 = v131 * v116;
                    let v158 = v38 + v128;
                    let v807 = v549 * v156;
                    let v163 = rspice_limexp((v8 - v101));
                    let v817 = (Lanes([0.0, 0.0, (v534 * v163)])) + ((((Lanes([v549[0], v549[1], 0.0])) - (Lanes([0.0, 0.0, v532]))) * v163) * v137);
                    let v167 = v134 + (v154 * v151);
                    let v825 = v549 * v153;
                    let v169 = (v153 * v8).tanh();
                    let v170 = v131 * v151;
                    let v171 = v38 - v169;
                    let v840 = v549 * v167;
                    let v846 = (((((Lanes([0.0, 0.0, 0.0, (v533 * v151)])) + (v796 * v131)) * v171) + ((((((v796 * v93) * v8) + (Lanes([v825[0], 0.0, v825[1], 0.0]))) * (v523 - (v169 * v169))) * v545) * v170)) * (v38 - (v167 * v8))) + (((((v796 * v154) * v8) + (Lanes([v840[0], 0.0, v840[1], 0.0]))) * v545) * (v170 * v171));
                    let v849 = (((((((Lanes([0.0, 0.0, 0.0, 0.0, (v533 * v116)])) + (v619 * v131)) * v158) + (v627 * v157)) * ((v38 + (v156 * v8)) + (v137 * v163))) + (((((v619 * v154) * v8) + (Lanes([v807[0], 0.0, v807[1], 0.0, 0.0]))) + (Lanes([v817[0], 0.0, v817[1], 0.0, v817[2]]))) * (v157 * v158))) - (Lanes([v846[0], v846[1], v846[2], 0.0, v846[3]]))) * v76;
                    v266 = v123;
                    v536 = v849;
                } else {
                    let v176 = if v129 == v175 { 1.0 } else { 0.0 };
                    let v267: f64;
                    let v537: Lanes<5>;
                    if v176 != 0.0 {
                        let v178 = v111 * v106;
                        let v180 = (v105 + v109) + (v178 * v105);
                        let v181 = v89 * v180;
                        let v737 = v581 * v180;
                        let v740 = (Lanes([v737[0], 0.0, v737[1], 0.0, v737[2]])) + (((v603 + v610) + (((v605 * v111) * v105) + (v603 * v178))) * v89);
                        let v182 = rspice_limexp(v181);
                        let v184 = rspice_limexp((-v181));
                        let v187 = (v76 * (v182 - v184)).tanh();
                        let v748 = (((v740 * v182) - ((v740 * v545) * v184)) * v76) * (v523 - (v187 * v187));
                        let v188 = v38 + v187;
                        let v190 = v124 + (v93 * v188);
                        let v751 = v549 * v190;
                        let v192 = (v190 * v8).tanh();
                        let v194 = v134 + (v154 * v188);
                        let v195 = v131 * v188;
                        let v766 = v549 * v194;
                        let v199 = rspice_limexp(v102);
                        let v773 = (Lanes([0.0, 0.0, (v534 * v199)])) + ((v593 * v199) * v137);
                        let v778 = (((((Lanes([0.0, 0.0, 0.0, 0.0, (v533 * v188)])) + (v748 * v131)) * v192) + (((((v748 * v93) * v8) + (Lanes([v751[0], 0.0, v751[1], 0.0, 0.0]))) * (v523 - (v192 * v192))) * v195)) * ((v38 + (v194 * v8)) + (v137 * v199))) + (((((v748 * v154) * v8) + (Lanes([v766[0], 0.0, v766[1], 0.0, 0.0]))) + (Lanes([v773[0], v773[1], 0.0, 0.0, v773[2]]))) * (v195 * v192));
                        v267 = v188;
                        v537 = v778;
                    } else {
                        let v203 = if v129 == v202 { 1.0 } else { 0.0 };
                        let v268: f64;
                        let v538: Lanes<5>;
                        if v203 != 0.0 {
                            let v205 = v111 * v106;
                            let v207 = (v105 + v109) + (v205 * v105);
                            let v208 = v89 * v207;
                            let v635 = v581 * v207;
                            let v638 = (Lanes([v635[0], 0.0, v635[1], 0.0, v635[2]])) + (((v603 + v610) + (((v605 * v111) * v105) + (v603 * v205))) * v89);
                            let v209 = v6 - v104;
                            let v640 = (Lanes([v544[0], v544[1], 0.0, 0.0])) - v600;
                            let v210 = v209 * v209;
                            let v641 = v640 * v209;
                            let v642 = v641 + v641;
                            let v213 = v111 * v209;
                            let v215 = (v209 + (v108 * v210)) + (v213 * v210);
                            let v216 = v89 * v215;
                            let v650 = v581 * v215;
                            let v653 = (Lanes([v650[0], 0.0, v650[1], v650[2]])) + (((v640 + (v642 * v108)) + (((v640 * v111) * v210) + (v642 * v213))) * v89);
                            let v217 = rspice_limexp(v208);
                            let v219 = rspice_limexp((-v208));
                            let v222 = (v76 * (v217 - v219)).tanh();
                            let v661 = (((v638 * v217) - ((v638 * v545) * v219)) * v76) * (v523 - (v222 * v222));
                            let v223 = v38 + v222;
                            let v224 = rspice_limexp(v216);
                            let v226 = rspice_limexp((-v216));
                            let v229 = (v76 * (v224 - v226)).tanh();
                            let v669 = (((v653 * v224) - ((v653 * v545) * v226)) * v76) * (v523 - (v229 * v229));
                            let v230 = v38 + v229;
                            let v232 = v124 + (v93 * v223);
                            let v234 = v124 + (v93 * v230);
                            let v673 = v549 * v232;
                            let v236 = (v232 * v8).tanh();
                            let v680 = v549 * v234;
                            let v238 = (v234 * v8).tanh();
                            let v240 = v134 + (v154 * v230);
                            let v242 = v134 + (v154 * v223);
                            let v243 = v131 * v223;
                            let v244 = v38 + v236;
                            let v696 = v549 * v242;
                            let v249 = rspice_limexp((v8 - v101));
                            let v706 = (Lanes([0.0, 0.0, (v534 * v249)])) + ((((Lanes([v549[0], v549[1], 0.0])) - (Lanes([0.0, 0.0, v532]))) * v249) * v137);
                            let v252 = v131 * v230;
                            let v253 = v38 - v238;
                            let v721 = v549 * v240;
                            let v727 = (((((Lanes([0.0, 0.0, 0.0, (v533 * v230)])) + (v669 * v131)) * v253) + ((((((v669 * v93) * v8) + (Lanes([v680[0], 0.0, v680[1], 0.0]))) * (v523 - (v238 * v238))) * v545) * v252)) * (v38 - (v240 * v8))) + (((((v669 * v154) * v8) + (Lanes([v721[0], 0.0, v721[1], 0.0]))) * v545) * (v252 * v253));
                            let v730 = (((((((Lanes([0.0, 0.0, 0.0, 0.0, (v533 * v223)])) + (v661 * v131)) * v244) + (((((v661 * v93) * v8) + (Lanes([v673[0], 0.0, v673[1], 0.0, 0.0]))) * (v523 - (v236 * v236))) * v243)) * ((v38 + (v242 * v8)) + (v137 * v249))) + (((((v661 * v154) * v8) + (Lanes([v696[0], 0.0, v696[1], 0.0, 0.0]))) + (Lanes([v706[0], 0.0, v706[1], 0.0, v706[2]]))) * (v243 * v244))) - (Lanes([v727[0], v727[1], v727[2], 0.0, v727[3]]))) * v76;
                            v268 = v223;
                            v538 = v730;
                        } else {
                            v268 = v123;
                            v538 = v628;
                        }
                        v267 = v268;
                        v537 = v538;
                    }
                    v266 = v267;
                    v536 = v537;
                }
                v265 = v266;
                v535 = v536;
            }
            let v258 = if v130 != 0.0 || (if v129 == v38 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v273: f64;
            let v278: f64;
            if v258 != 0.0 {
                let v261 = v260 * v116;
                let v262 = v259 + v261;
                let v264 = v263 + v261;
                v273 = v264;
                v278 = v262;
            } else {
                let v269 = v260 * v265;
                let v270 = v259 + v269;
                let v271 = v263 + v269;
                v273 = v271;
                v278 = v270;
            }
            let v272 = if v33 != 0.0 || v36 != 0.0 { 1.0 } else { 0.0 };
            let v341: f64;
            if v272 != 0.0 {
                let v277 = v273 * (v38 + (v274 * v33));
                v341 = v277;
            } else {
                v341 = v273;
            }
            let v280 = if v279 == v0 { 1.0 } else { 0.0 };
            let v301: f64;
            let v305: f64;
            let v308: f64;
            if v280 != 0.0 {
                let v287 = rspice_limexp((v281 * ((v282 * v283).tanh())));
                let v288 = v3 - v283;
                let v289 = v10 - v283;
                v301 = v288;
                v305 = v287;
                v308 = v289;
            } else {
                let v292 = rspice_limexp(((-v281) * v283));
                let v293 = if v279 == v38 { 1.0 } else { 0.0 };
                let v302: f64;
                let v309: f64;
                if v293 != 0.0 {
                    let v295 = (v3 - v283).tanh();
                    let v297 = (v10 - v283).tanh();
                    v302 = v295;
                    v309 = v297;
                } else {
                    let v298 = v3 - v283;
                    let v299 = v10 - v283;
                    v302 = v298;
                    v309 = v299;
                }
                v301 = v302;
                v305 = v292;
                v308 = v309;
            }
            let v307 = v300 * ((rspice_limexp((v281 * v301))) - v305);
            let v313 = v300 * ((rspice_limexp((v281 * v308))) - v305);
            let v315 = if v314 == v0 { 1.0 } else { 0.0 };
            if v315 != 0.0 {
            } else {
                let v316 = if v314 == v38 { 1.0 } else { 0.0 };
                if v316 != 0.0 {
                } else {
                    let v318 = if v314 == v175 { 1.0 } else { 0.0 };
                    if v318 != 0.0 {
                    } else {
                    }
                }
            }
            let v319 = if v314 == v175 { 1.0 } else { 0.0 };
            if v319 != 0.0 {
            } else {
            }
            let v320 = if v59 > v0 { 1.0 } else { 0.0 };
            if v320 != 0.0 {
            } else {
            }
            let v322 = if v321 > v0 { 1.0 } else { 0.0 };
            if v322 != 0.0 {
            } else {
            }
            let v324 = if v323 > v0 { 1.0 } else { 0.0 };
            let v433: f64;
            let v435: f64;
            if v324 != 0.0 {
                let v434: f64;
                let v436: f64;
                if v325 != 0.0 {
                    let v330 = (v328 * v28) * v323;
                    v434 = v38;
                    v436 = v330;
                } else {
                    v434 = v0;
                    v436 = v0;
                }
                v433 = v434;
                v435 = v436;
            } else {
                v433 = v0;
                v435 = v0;
            }
            let v332 = if v331 > v0 { 1.0 } else { 0.0 };
            if v332 != 0.0 {
            } else {
            }
            let v334 = if v333 > v0 { 1.0 } else { 0.0 };
            let v437: f64;
            let v439: f64;
            if v334 != 0.0 {
                let v438: f64;
                let v440: f64;
                if v325 != 0.0 {
                    let v338 = (v336 * v28) * v333;
                    v438 = v38;
                    v440 = v338;
                } else {
                    v438 = v0;
                    v440 = v0;
                }
                v437 = v438;
                v439 = v440;
            } else {
                let v339 = if v335 > v0 { 1.0 } else { 0.0 };
                if v339 != 0.0 {
                } else {
                }
                v437 = v0;
                v439 = v0;
            }
            let v340 = if v263 > v0 { 1.0 } else { 0.0 };
            let v441: f64;
            let v443: f64;
            if v340 != 0.0 {
                let v442: f64;
                let v444: f64;
                if v325 != 0.0 {
                    let v344 = (v342 * v28) * v341;
                    v442 = v38;
                    v444 = v344;
                } else {
                    v442 = v0;
                    v444 = v0;
                }
                v441 = v442;
                v443 = v444;
            } else {
                v441 = v0;
                v443 = v0;
            }
            let v347 = if (if v259 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v260 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v445: f64;
            let v447: f64;
            if v347 != 0.0 {
                let v446: f64;
                let v448: f64;
                if v325 != 0.0 {
                    let v351 = (v349 * v28) * v278;
                    v446 = v38;
                    v448 = v351;
                } else {
                    v446 = v0;
                    v448 = v0;
                }
                v445 = v446;
                v447 = v448;
            } else {
                let v352 = if v348 > v0 { 1.0 } else { 0.0 };
                if v352 != 0.0 {
                } else {
                }
                v445 = v0;
                v447 = v0;
            }
            let v354 = if v353 == v0 { 1.0 } else { 0.0 };
            let v449: f64;
            let v451: f64;
            let v453: f64;
            let v455: f64;
            let v457: f64;
            let v459: f64;
            let v462: f64;
            let v465: f64;
            let v468: f64;
            let v471: f64;
            let v474: f64;
            let v477: f64;
            let v480: f64;
            let v483: f64;
            let v486: f64;
            let v489: f64;
            let v492: f64;
            let v495: f64;
            let v499: f64;
            let v503: f64;
            if v354 != 0.0 {
                let v357 = (v11.abs()) + (v313.abs());
                let v381 = (((v369 * v326) * v327) * v28) * (((((((v358 + v14) * (v38 + (((v360 * v116) * (v128.abs())) * (v38 + (v134 * v8))))) / v28) * v357) + ((v375 * v357) * v357)).abs()).sqrt());
                let v450: f64;
                let v452: f64;
                let v454: f64;
                let v456: f64;
                let v458: f64;
                if v325 != 0.0 {
                    let v383 = v381 * v382;
                    v450 = v38;
                    v452 = v381;
                    v454 = v38;
                    v456 = v383;
                    v458 = v384;
                } else {
                    v450 = v0;
                    v452 = v0;
                    v454 = v0;
                    v456 = v0;
                    v458 = v0;
                }
                v449 = v450;
                v451 = v452;
                v453 = v454;
                v455 = v456;
                v457 = v458;
                v459 = v0;
                v462 = v0;
                v465 = v0;
                v468 = v0;
                v471 = v0;
                v474 = v0;
                v477 = v0;
                v480 = v0;
                v483 = v0;
                v486 = v0;
                v489 = v0;
                v492 = v0;
                v495 = v0;
                v499 = v0;
                v503 = v0;
            } else {
                let v385 = if v353 == v38 { 1.0 } else { 0.0 };
                let v460: f64;
                let v463: f64;
                let v466: f64;
                let v469: f64;
                let v472: f64;
                let v475: f64;
                let v478: f64;
                let v481: f64;
                let v484: f64;
                let v487: f64;
                let v490: f64;
                let v493: f64;
                let v496: f64;
                let v500: f64;
                let v504: f64;
                if v385 != 0.0 {
                    let v461: f64;
                    let v464: f64;
                    let v467: f64;
                    let v470: f64;
                    let v473: f64;
                    let v476: f64;
                    let v479: f64;
                    let v482: f64;
                    let v485: f64;
                    let v488: f64;
                    let v491: f64;
                    let v494: f64;
                    let v497: f64;
                    let v501: f64;
                    let v505: f64;
                    if v325 != 0.0 {
                        let v870 = v535[1];
                        let v390 = ((v386 * v28) * v870) * v389;
                        let v391 = if v870 > v0 { 1.0 } else { 0.0 };
                        let v407: f64;
                        if v391 != 0.0 {
                            let v398 = (((((v317 * v317) * v326) * v327) * v28) * v396) / v870;
                            v407 = v398;
                        } else {
                            v407 = v0;
                        }
                        let v406 = (((v399 * v28) * v401) * v317) * ((v389 * v396).sqrt());
                        let v413 = (((v408 * v28) * v870) * v389) * v412;
                        let v415 = if v414 > v0 { 1.0 } else { 0.0 };
                        let v498: f64;
                        let v502: f64;
                        let v506: f64;
                        if v415 != 0.0 {
                            let v418 = v414 * (v11.powf(v416));
                            v498 = v38;
                            v502 = v418;
                            v506 = v419;
                        } else {
                            v498 = v0;
                            v502 = v0;
                            v506 = v0;
                        }
                        v461 = v38;
                        v464 = v406;
                        v467 = v38;
                        v470 = v406;
                        v473 = v38;
                        v476 = v390;
                        v479 = v38;
                        v482 = v407;
                        v485 = v175;
                        v488 = v38;
                        v491 = v413;
                        v494 = v38;
                        v497 = v498;
                        v501 = v502;
                        v505 = v506;
                    } else {
                        v461 = v0;
                        v464 = v0;
                        v467 = v0;
                        v470 = v0;
                        v473 = v0;
                        v476 = v0;
                        v479 = v0;
                        v482 = v0;
                        v485 = v0;
                        v488 = v0;
                        v491 = v0;
                        v494 = v0;
                        v497 = v0;
                        v501 = v0;
                        v505 = v0;
                    }
                    v460 = v461;
                    v463 = v464;
                    v466 = v467;
                    v469 = v470;
                    v472 = v473;
                    v475 = v476;
                    v478 = v479;
                    v481 = v482;
                    v484 = v485;
                    v487 = v488;
                    v490 = v491;
                    v493 = v494;
                    v496 = v497;
                    v500 = v501;
                    v504 = v505;
                } else {
                    let v420 = if v353 == v175 { 1.0 } else { 0.0 };
                    if v420 != 0.0 {
                    } else {
                    }
                    v460 = v0;
                    v463 = v0;
                    v466 = v0;
                    v469 = v0;
                    v472 = v0;
                    v475 = v0;
                    v478 = v0;
                    v481 = v0;
                    v484 = v0;
                    v487 = v0;
                    v490 = v0;
                    v493 = v0;
                    v496 = v0;
                    v500 = v0;
                    v504 = v0;
                }
                v449 = v0;
                v451 = v0;
                v453 = v0;
                v455 = v0;
                v457 = v0;
                v459 = v460;
                v462 = v463;
                v465 = v466;
                v468 = v469;
                v471 = v472;
                v474 = v475;
                v477 = v478;
                v480 = v481;
                v483 = v484;
                v486 = v487;
                v489 = v490;
                v492 = v493;
                v495 = v496;
                v499 = v500;
                v503 = v504;
            }
            let v507: f64;
            let v508: f64;
            let v509: f64;
            let v510: f64;
            let v511: f64;
            let v513: f64;
            let v515: f64;
            let v517: f64;
            let v519: f64;
            let v521: f64;
            if v325 != 0.0 {
                let v422 = v307.abs();
                let v423 = v421 * v422;
                let v425 = v313.abs();
                let v426 = v424 * v425;
                let v427 = if v414 > v0 { 1.0 } else { 0.0 };
                let v512: f64;
                let v514: f64;
                let v516: f64;
                let v518: f64;
                let v520: f64;
                let v522: f64;
                if v427 != 0.0 {
                    let v429 = v414 * (v422.powf(v416));
                    let v431 = v414 * (v425.powf(v416));
                    v512 = v38;
                    v514 = v429;
                    v516 = v419;
                    v518 = v38;
                    v520 = v431;
                    v522 = v419;
                } else {
                    v512 = v0;
                    v514 = v0;
                    v516 = v0;
                    v518 = v0;
                    v520 = v0;
                    v522 = v0;
                }
                v507 = v38;
                v508 = v423;
                v509 = v38;
                v510 = v426;
                v511 = v512;
                v513 = v514;
                v515 = v516;
                v517 = v518;
                v519 = v520;
                v521 = v522;
            } else {
                v507 = v0;
                v508 = v0;
                v509 = v0;
                v510 = v0;
                v511 = v0;
                v513 = v0;
                v515 = v0;
                v517 = v0;
                v519 = v0;
                v521 = v0;
            }
            let v432 = if v23 != 0.0 && v35 != 0.0 { 1.0 } else { 0.0 };
            if v432 != 0.0 {
            } else {
            }
        if v433 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v435;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v437 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v439;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v441 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v443;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v445 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v447;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd / self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v449 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v451;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v453 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v455;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v457);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v459 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v462;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v465 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v468;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v471 == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v474;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v477 == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v480;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v483);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v486 == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v489;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v492);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v495 == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v499;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v503);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v507 == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v508;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v509 == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v510;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v511 == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v513;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v515);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v517 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v519;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v521);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

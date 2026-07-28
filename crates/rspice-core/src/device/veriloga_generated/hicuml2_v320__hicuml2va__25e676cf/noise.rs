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
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 19] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BP_RBX", label: Some("rbx"), kind: GeneratedNoiseKind::White, equation: 47, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_BI_RBI", label: Some("rbi"), kind: GeneratedNoiseKind::White, equation: 48, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_C_RCX", label: Some("rcx"), kind: GeneratedNoiseKind::White, equation: 49, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "c", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_EI_E_RE", label: Some("re"), kind: GeneratedNoiseKind::White, equation: 50, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_S_RSU", label: Some("rsu"), kind: GeneratedNoiseKind::White, equation: 51, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BI_EI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_BP_EI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 53, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_EI_E_FLICKER_RE", label: Some("flicker_re"), kind: GeneratedNoiseKind::Flicker, equation: 54, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEBTB", label: Some("ibebtb"), kind: GeneratedNoiseKind::White, equation: 55, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_EI_IBEP", label: Some("ibep"), kind: GeneratedNoiseKind::White, equation: 56, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_BI_IAVL", label: Some("iavl"), kind: GeneratedNoiseKind::White, equation: 57, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_CI_IBCI", label: Some("ibci"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_CI_IBCBTB", label: Some("ibcbtb"), kind: GeneratedNoiseKind::White, equation: 59, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BP_CI_IJBCX", label: Some("ijbcx"), kind: GeneratedNoiseKind::White, equation: 60, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "bp", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_CI_IJSC", label: Some("ijsc"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_GND_IBEI", label: Some("ibei"), kind: GeneratedNoiseKind::White, equation: 62, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "n1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_GND_IT", label: Some("it"), kind: GeneratedNoiseKind::White, equation: 67, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "n2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_CI_EI_IT", label: Some("it"), kind: GeneratedNoiseKind::White, equation: 70, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "ci", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_BI_EI_IBEI", label: Some("ibei"), kind: GeneratedNoiseKind::White, equation: 71, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "bi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "ei", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14])];
        let branch_unknown_flows = [ctx.branch_current(self.branches[0]), ctx.branch_current(self.branches[1]), ctx.branch_current(self.branches[2]), ctx.branch_current(self.branches[3]), ctx.branch_current(self.branches[4]), ctx.branch_current(self.branches[5])];
            let v0 = 0e0f64;
            let v1 = parameters[148];
            let v2 = node_potentials[8];
            let v3 = node_potentials[6];
            let v6 = node_potentials[5];
            let v10 = node_potentials[7];
            let v15 = node_potentials[1];
            let v18 = node_potentials[9];
            let v21 = node_potentials[3];
            let v22 = node_potentials[0];
            let v25 = parameters[0];
            let v26 = 3.1e2f64;
            let v28 = 1.6021918e-19f64;
            let v29 = 1.3806226e-23f64;
            let v30 = 1.602176634e-19f64;
            let v31 = 1.380649e-23f64;
            let v33 = parameters[146];
            let v34 = 2.7315e2f64;
            let v36 = temperature;
            let v40 = 3e2f64;
            let v43 = 1e0f64;
            let v45 = parameters[121];
            let v49 = parameters[122];
            let v51 = parameters[131];
            let v53 = parameters[117];
            let v56 = parameters[118];
            let v59 = parameters[119];
            let v63 = 5e-1f64;
            let v71 = parameters[120];
            let v74 = 3e0f64;
            let v78 = parameters[130];
            let v80 = parameters[138];
            let v82 = 1.5e0f64;
            let v84 = parameters[107];
            let v86 = parameters[52];
            let v87 = parameters[106];
            let v93 = parameters[105];
            let v94 = parameters[104];
            let v97 = parameters[22];
            let v101 = 7e-1f64;
            let v102 = parameters[32];
            let v104 = parameters[47];
            let v107 = parameters[86];
            let v109 = parameters[88];
            let v111 = parameters[87];
            let v114 = parameters[66];
            let v117 = parameters[115];
            let v118 = 1e-2f64;
            let v120 = parameters[116];
            let v128 = 1e9f64;
            let v129 = 1.7e8f64;
            let v135 = 6e0f64;
            let v142 = parameters[147];
            let v144 = 7.314999999999998e1f64;
            let v146 = 7.314999999999998e1f64;
            let v147 = 6e2f64;
            let v149 = 6e2f64;
            let v172 = parameters[39];
            let v174 = 2e0f64;
            let v176 = parameters[40];
            let v180 = -5e-1f64;
            let v195 = 4e0f64;
            let v207 = parameters[41];
            let v213 = parameters[42];
            let v218 = parameters[14];
            let v219 = parameters[124];
            let v227 = parameters[16];
            let v228 = parameters[17];
            let v238 = parameters[48];
            let v242 = -5e-1f64;
            let v268 = parameters[49];
            let v274 = parameters[50];
            let v279 = 2.4e0f64;
            let v280 = parameters[23];
            let v287 = parameters[2];
            let v295 = parameters[1];
            let v296 = parameters[123];
            let v303 = parameters[10];
            let v304 = parameters[126];
            let v308 = parameters[8];
            let v311 = 1e-5f64;
            let v314 = parameters[9];
            let v315 = parameters[125];
            let v317 = parameters[127];
            let v331 = parameters[3];
            let v336 = parameters[4];
            let v342 = parameters[6];
            let v348 = parameters[75];
            let v353 = parameters[74];
            let v358 = parameters[79];
            let v360 = parameters[133];
            let v364 = parameters[78];
            let v365 = parameters[132];
            let v369 = parameters[128];
            let v372 = parameters[129];
            let v377 = parameters[69];
            let v378 = parameters[71];
            let v385 = parameters[139];
            let v389 = parameters[33];
            let v390 = parameters[140];
            let v394 = parameters[37];
            let v398 = parameters[38];
            let v413 = parameters[89];
            let v414 = parameters[134];
            let v418 = parameters[43];
            let v421 = parameters[44];
            let v425 = -5e-1f64;
            let v451 = parameters[45];
            let v457 = parameters[46];
            let v462 = parameters[18];
            let v464 = parameters[20];
            let v465 = parameters[21];
            let v472 = parameters[27];
            let v480 = parameters[29];
            let v494 = -1.5e0f64;
            let v509 = -1.5e0f64;
            let v516 = parameters[28];
            let v520 = parameters[30];
            let v523 = parameters[31];
            let v527 = 1.0f64;
            let v529 = parameters[53];
            let v533 = -5e-1f64;
            let v559 = parameters[54];
            let v564 = parameters[55];
            let v574 = parameters[25];
            let v579 = parameters[57];
            let v582 = parameters[58];
            let v586 = -5e-1f64;
            let v612 = parameters[59];
            let v618 = 2.4e0f64;
            let v619 = 0.0f64;
            let v620 = -2.4e0f64;
            let v623 = -2.4e0f64;
            let v629 = -5e-1f64;
            let v660 = parameters[60];
            let v667 = parameters[99];
            let v674 = parameters[97];
            let v675 = parameters[101];
            let v676 = parameters[63];
            let v678 = parameters[62];
            let v684 = -5e-1f64;
            let v710 = parameters[64];
            let v723 = parameters[96];
            let v724 = parameters[136];
            let v728 = parameters[90];
            let v729 = parameters[135];
            let v733 = parameters[95];
            let v734 = parameters[137];
            let v738 = parameters[142];
            let v739 = parameters[141];
            let v741 = parameters[149];
            let v746 = node_potentials[4];
            let v748 = 7.314999999999998e1f64;
            let v750 = 7.314999999999998e1f64;
            let v751 = 6e2f64;
            let v753 = 6e2f64;
            let v780 = -5e-1f64;
            let v831 = -5e-1f64;
            let v958 = -5e-1f64;
            let v1013 = -1.5e0f64;
            let v1028 = -1.5e0f64;
            let v1043 = 1.0f64;
            let v1048 = -5e-1f64;
            let v1094 = -5e-1f64;
            let v1125 = 2.4e0f64;
            let v1126 = 0.0f64;
            let v1127 = -2.4e0f64;
            let v1130 = -2.4e0f64;
            let v1136 = -5e-1f64;
            let v1183 = -5e-1f64;
            let v1231 = parameters[15];
            let v1235 = 8e1f64;
            let v1262 = parameters[13];
            let v1286 = 1.921812e0f64;
            let v1315 = parameters[51];
            let v1316 = 1e2f64;
            let v1352 = 1e-1f64;
            let v1459 = parameters[11];
            let v1479 = 1e-3f64;
            let v1497 = parameters[12];
            let v1503 = 5e-2f64;
            let v1514 = -8.754687373538999e-1f64;
            let v1540 = parameters[67];
            let v1545 = parameters[68];
            let v1569 = parameters[80];
            let v1580 = parameters[77];
            let v1590 = parameters[76];
            let v1593 = parameters[81];
            let v1601 = parameters[85];
            let v1630 = 1e-6f64;
            let v1633 = 3.2e2f64;
            let v1637 = parameters[70];
            let v1645 = parameters[83];
            let v1651 = -1e10f64;
            let v1653 = -1e10f64;
            let v1656 = parameters[84];
            let v1659 = parameters[82];
            let v1660 = -2e0f64;
            let v1665 = parameters[73];
            let v1677 = parameters[72];
            let v1695 = 5e-3f64;
            let v1720 = 2.5e-1f64;
            let v1794 = parameters[5];
            let v1829 = parameters[7];
            let v1864 = -1e10f64;
            let v1866 = -1e10f64;
            let v1871 = -2e0f64;
            let v1997 = -2e0f64;
            let v2091 = 3e-1f64;
            let v2120 = -1e10f64;
            let v2122 = -1e10f64;
            let v2127 = -2e0f64;
            let v2251 = parameters[93];
            let v2257 = parameters[24];
            let v2299 = parameters[35];
            let v2304 = parameters[36];
            let v2346 = parameters[34];
            let v2352 = 1e-4f64;
            let v2367 = parameters[92];
            let v2383 = parameters[91];
            let v2397 = parameters[94];
            let v2404 = parameters[19];
            let v2513 = parameters[56];
            let v2551 = parameters[26];
            let v2666 = parameters[61];
            let v2717 = parameters[65];
            let v2763 = parameters[100];
            let v2795 = node_potentials[2];
            let v2841 = parameters[102];
            let v2845 = parameters[103];
            let v2850 = parameters[145];
            let v2855 = 0e0f64;
            let v2861 = parameters[110];
            let v2864 = parameters[111];
            let v2867 = parameters[112];
            let v2868 = -1e0f64;
            let v2872 = parameters[113];
            let v2874 = parameters[114];
            let v2891 = parameters[109];
            let v2915 = branch_unknown_flows[1];
            let v5 = v1 * (v2 - v3);
            let v8 = v1 * (v2 - v6);
            let v9 = v5 - v8;
            let v12 = v1 * (v10 - v3);
            let v14 = v1 * (v10 - v6);
            let v16 = v15 - v6;
            let v17 = v1 * v16;
            let v20 = v1 * (v18 - v6);
            let v24 = v1 * (v21 - v22);
            let v27 = if v25 <= v26 { 1.0 } else { 0.0 };
            let v37: f64;
            let v38: f64;
            if v27 != 0.0 {
                v37 = v29;
                v38 = v28;
            } else {
                v37 = v31;
                v38 = v30;
            }
            let v32 = ctx.simparam_or("gmin", v0);
            let v35 = v33 + v34;
            let v39 = v37 / v38;
            let v41 = v39 * v40;
            let v42 = v39 * v35;
            let v44 = v43 / v42;
            let v48 = (v45 * v35) * (v35.ln());
            let v50 = v49 * v35;
            let v52 = v51 * v35;
            let v55 = (v53 + v48) + v50;
            let v64 = (v55 + ((v56 + v48) + v50)) * v63;
            let v66 = (v55 + ((v59 + v48) + v50)) * v63;
            let v68 = (v53 + v56) * v63;
            let v70 = (v53 + v59) * v63;
            let v73 = (v71 + v59) * v63;
            let v76 = v74 - (v45 / v39);
            let v77 = v76 + v43;
            let v79 = v77 - v78;
            let v81 = v77 - v80;
            let v83 = v76 - v82;
            let v89 = (v43 - v84) * (v86 + v87);
            let v90 = if v89 >= v87 { 1.0 } else { 0.0 };
            let v570: f64;
            let v572: f64;
            let v2824: f64;
            if v90 != 0.0 {
                let v91 = v89 - v87;
                let v92 = v86 - v91;
                v570 = v91;
                v572 = v92;
                v2824 = v87;
            } else {
                v570 = v0;
                v572 = v86;
                v2824 = v89;
            }
            let v96 = v94 - (v93 * v94);
            let v98 = if v97 != v0 { 1.0 } else { 0.0 };
            let v2363: f64;
            if v98 != 0.0 {
                let v99 = v43 / v97;
                v2363 = v99;
            } else {
                v2363 = v0;
            }
            let v100 = if v25 <= v40 { 1.0 } else { 0.0 };
            let v474: f64;
            if v100 != 0.0 {
                v474 = v0;
            } else {
                v474 = v101;
            }
            let v105 = if v104 > v0 { 1.0 } else { 0.0 };
            let v106 = if (if v102 > v0 { 1.0 } else { 0.0 }) != 0.0 && v105 != 0.0 { 1.0 } else { 0.0 };
            let v383: f64;
            if v106 != 0.0 {
                v383 = v43;
            } else {
                v383 = v0;
            }
            let v108 = if v107 != v0 { 1.0 } else { 0.0 };
            let v2804: f64;
            if v108 != 0.0 {
                let v116 = if (if (if v109 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v111 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v114 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v2805: f64;
                if v116 != 0.0 {
                    v2805 = v0;
                } else {
                    v2805 = v107;
                }
                v2804 = v2805;
            } else {
                v2804 = v107;
            }
            let v122 = if (if v117 >= v118 { 1.0 } else { 0.0 }) != 0.0 || (if v120 >= v118 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1705: f64;
            let v1708: f64;
            let v1712: f64;
            let v1715: f64;
            let v1738: f64;
            let v1740: f64;
            let v1753: f64;
            let v1755: f64;
            if v122 != 0.0 {
                let v124 = v63 * (v117 - v120);
                let v125 = if v120 < v117 { 1.0 } else { 0.0 };
                let v126: f64;
                let v130: f64;
                if v125 != 0.0 {
                    v126 = v120;
                    v130 = v117;
                } else {
                    v126 = v117;
                    v130 = v120;
                }
                let v127 = if v126 < v118 { 1.0 } else { 0.0 };
                let v1709: f64;
                let v1739: f64;
                let v1741: f64;
                let v1754: f64;
                let v1756: f64;
                if v127 != 0.0 {
                    let v132 = (v43 + v130).ln();
                    v1709 = v132;
                    v1739 = v129;
                    v1741 = v128;
                    v1754 = v129;
                    v1756 = v128;
                } else {
                    let v133 = v43 / v117;
                    let v134 = v43 / v120;
                    let v136 = v117 / v135;
                    let v137 = v120 / v135;
                    let v141 = ((v43 + v117) / (v43 + v120)).ln();
                    v1709 = v141;
                    v1739 = v136;
                    v1741 = v134;
                    v1754 = v137;
                    v1756 = v133;
                }
                v1705 = v124;
                v1708 = v1709;
                v1712 = v126;
                v1715 = v130;
                v1738 = v1739;
                v1740 = v1741;
                v1753 = v1754;
                v1755 = v1756;
            } else {
                v1705 = v0;
                v1708 = v0;
                v1712 = v120;
                v1715 = v117;
                v1738 = v129;
                v1740 = v128;
                v1753 = v129;
                v1755 = v128;
            }
            let v143 = v36 + v142;
            let v145 = if v143 < v144 { 1.0 } else { 0.0 };
            let v150: f64;
            if v145 != 0.0 {
                v150 = v146;
            } else {
                let v148 = if v143 > v147 { 1.0 } else { 0.0 };
                let v151: f64;
                if v148 != 0.0 {
                    v151 = v149;
                } else {
                    v151 = v143;
                }
                v150 = v151;
            }
            let v152 = v39 * v150;
            let v153 = v43 / v152;
            let v154 = v150 - v35;
            let v155 = v35 / v150;
            let v156 = v150 / v35;
            let v157 = v156.ln();
            let v160 = (v45 * v150) * (v150.ln());
            let v161 = v49 * v150;
            let v163 = (v53 + v160) + v161;
            let v169 = (v163 + ((v56 + v160) + v161)) * v63;
            let v171 = (v163 + ((v59 + v160) + v161)) * v63;
            let v173 = if v172 > v0 { 1.0 } else { 0.0 };
            let v288: f64;
            let v502: f64;
            let v1275: f64;
            if v173 != 0.0 {
                let v193 = ((((v174 * v42) * (((((v176 * v63) * v44).exp()) - (((v180 * v176) * v44).exp())).ln())) * v156) + (v68 * (v43 - v156))) - ((v76 * v152) * v157);
                let v206 = v193 + ((v174 * v152) * ((v63 * (v43 + ((v43 + (v195 * (((-v193) * v153).exp()))).sqrt()))).ln()));
                let v212 = v172 * ((v207 * ((v176 / v206).ln())).exp());
                let v214 = v213.abs();
                let v215 = if v213 > v0 { 1.0 } else { 0.0 };
                let v1276: f64;
                if v215 != 0.0 {
                    let v217 = (v213 * v206) / v176;
                    v1276 = v217;
                } else {
                    v1276 = v214;
                }
                v288 = v206;
                v502 = v212;
                v1275 = v1276;
            } else {
                v288 = v176;
                v502 = v172;
                v1275 = v213;
            }
            let v221 = v56 * v44;
            let v222 = v43 - v155;
            let v225 = ((v219 * v157) + (v221 * v222)).exp();
            let v226 = v218 * v225;
            let v229 = v76 / v228;
            let v231 = v68 * v44;
            let v232 = v231 * v222;
            let v236 = v227 * (((v229 * v157) + (v232 / v228)).exp());
            let v402: f64;
            let v406: f64;
            let v1328: f64;
            if v105 != 0.0 {
                let v255 = ((((v174 * v42) * (((((v238 * v63) * v44).exp()) - (((v242 * v238) * v44).exp())).ln())) * v156) + (v70 * (v43 - v156))) - ((v76 * v152) * v157);
                let v267 = v255 + ((v174 * v152) * ((v63 * (v43 + ((v43 + (v195 * (((-v255) * v153).exp()))).sqrt()))).ln()));
                let v273 = v104 * ((v268 * ((v238 / v267).ln())).exp());
                let v275 = v274.abs();
                let v276 = if v274 > v0 { 1.0 } else { 0.0 };
                let v1329: f64;
                if v276 != 0.0 {
                    let v278 = (v274 * v267) / v238;
                    v1329 = v278;
                } else {
                    v1329 = v275;
                }
                v402 = v267;
                v406 = v273;
                v1328 = v1329;
            } else {
                v402 = v238;
                v406 = v104;
                v1328 = v274;
            }
            let v1327: f64;
            if v100 != 0.0 {
                v1327 = v279;
            } else {
                v1327 = v1328;
            }
            let v282 = v59 * v44;
            let v283 = v282 * v222;
            let v286 = v280 * (((v79 * v157) + v283).exp());
            let v289 = v288 / v176;
            let v294 = v287 * (v174 - ((v207 * (v289.ln())).exp()));
            let v298 = v53 * v44;
            let v302 = v295 * (((v296 * v157) + (v298 * v222)).exp());
            let v307 = v303 * ((v304 * v157).exp());
            let v313 = if v100 != 0.0 && (if ((v308 - v43).abs()) < v311 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1483: f64;
            if v313 != 0.0 {
                let v323 = v314 * (((v315 * v153) * (((v317 * v157).exp()) - v43)).exp());
                v1483 = v323;
            } else {
                let v330 = v308 * (((v315 * v153) * (((v317 * v157).exp()) - v43)).exp());
                v1483 = v330;
            }
            let v332 = v315 * v44;
            let v335 = v331 * ((v332 * v222).exp());
            let v338 = (v53 - v56) * v44;
            let v341 = v336 * ((v338 * v222).exp());
            let v344 = (v53 - v59) * v44;
            let v347 = v342 * ((v344 * v222).exp());
            let v349 = v78 - v52;
            let v352 = v348 * ((v349 * v157).exp());
            let v357 = v43 / (v353 * ((v78 * v157).exp()));
            let v359 = if v358 > v0 { 1.0 } else { 0.0 };
            let v1551: f64;
            let v1555: f64;
            if v359 != 0.0 {
                let v363 = v358 * (v43 - (v360 * v154));
                v1551 = v363;
                v1555 = v364;
            } else {
                let v368 = v364 * (v43 + (v365 * v154));
                v1551 = v358;
                v1555 = v368;
            }
            let v376 = v114 * ((v43 + (v369 * v154)) + ((v372 * v154) * v154));
            let v379 = v78 - v43;
            let v382 = v378 * ((v379 * v157).exp());
            let v384 = if v383 == v43 { 1.0 } else { 0.0 };
            let v2321: f64;
            let v2327: f64;
            if v384 != 0.0 {
                let v388 = v102 * ((v385 * v154).exp());
                let v393 = v389 * ((v390 * v154).exp());
                v2321 = v393;
                v2327 = v388;
            } else {
                v2321 = v389;
                v2327 = v102;
            }
            let v397 = if (if v394 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v8 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v2282: f64;
            let v2291: f64;
            if v397 != 0.0 {
                let v400 = if v105 != 0.0 && (if v238 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v2283: f64;
                let v2292: f64;
                if v400 != 0.0 {
                    let v401 = v66 / v171;
                    let v403 = v402 / v238;
                    let v408 = (((v401.sqrt()) * v403) * v406) / v104;
                    let v410 = (v394 * v408) * v403;
                    let v412 = v398 / (v408 * v401);
                    v2283 = v410;
                    v2292 = v412;
                } else {
                    v2283 = v394;
                    v2292 = v398;
                }
                v2282 = v2283;
                v2291 = v2292;
            } else {
                v2282 = v0;
                v2291 = v43;
            }
            let v417 = v413 * ((v414 * v157).exp());
            let v419 = if v418 > v0 { 1.0 } else { 0.0 };
            let v485: f64;
            let v487: f64;
            let v2436: f64;
            if v419 != 0.0 {
                let v438 = ((((v174 * v42) * (((((v421 * v63) * v44).exp()) - (((v425 * v421) * v44).exp())).ln())) * v156) + (v68 * (v43 - v156))) - ((v76 * v152) * v157);
                let v450 = v438 + ((v174 * v152) * ((v63 * (v43 + ((v43 + (v195 * (((-v438) * v153).exp()))).sqrt()))).ln()));
                let v456 = v418 * ((v451 * ((v421 / v450).ln())).exp());
                let v458 = v457.abs();
                let v459 = if v457 > v0 { 1.0 } else { 0.0 };
                let v2437: f64;
                if v459 != 0.0 {
                    let v461 = (v457 * v450) / v421;
                    v2437 = v461;
                } else {
                    v2437 = v458;
                }
                v485 = v450;
                v487 = v456;
                v2436 = v2437;
            } else {
                v485 = v421;
                v487 = v418;
                v2436 = v457;
            }
            let v463 = v462 * v225;
            let v466 = v76 / v465;
            let v471 = v464 * (((v466 * v157) + (v232 / v465)).exp());
            let v478 = if (if v472 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v12 < v474 { 1.0 } else { 0.0 }) != 0.0 || (if v5 < v474 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v2480: f64;
            let v2485: f64;
            if v478 != 0.0 {
                let v479 = v64 / v169;
                let v484 = if (if (if v480 == v43 { 1.0 } else { 0.0 }) != 0.0 && v419 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v421 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v513: f64;
                let v517: f64;
                if v484 != 0.0 {
                    let v486 = v485 / v421;
                    let v492 = (((v487 / v418) * (v479.sqrt())) * v486) * v486;
                    let v497 = ((v418 / v487) * (v479.powf(v494))) / v486;
                    v513 = v492;
                    v517 = v497;
                } else {
                    let v501 = if (if (if v480 == v0 { 1.0 } else { 0.0 }) != 0.0 && v173 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v176 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v514: f64;
                    let v518: f64;
                    if v501 != 0.0 {
                        let v507 = (((v502 / v172) * (v479.sqrt())) * v289) * v289;
                        let v512 = ((v172 / v502) * (v479.powf(v509))) / v289;
                        v514 = v507;
                        v518 = v512;
                    } else {
                        v514 = v43;
                        v518 = v43;
                    }
                    v513 = v514;
                    v517 = v518;
                }
                let v515 = v472 * v513;
                let v519 = v516 * v517;
                v2480 = v515;
                v2485 = v519;
            } else {
                v2480 = v0;
                v2485 = v43;
            }
            let v526 = v520 * (((-(v288 - v176)) / v523).exp());
            let v569: f64;
            let v2519: f64;
            let v2526: f64;
            if v527 != 0.0 {
                let v546 = ((((v174 * v42) * (((((v529 * v63) * v44).exp()) - (((v533 * v529) * v44).exp())).ln())) * v156) + (v70 * (v43 - v156))) - ((v76 * v152) * v157);
                let v558 = v546 + ((v174 * v152) * ((v63 * (v43 + ((v43 + (v195 * (((-v546) * v153).exp()))).sqrt()))).ln()));
                let v563 = (v559 * ((v529 / v558).ln())).exp();
                let v565 = v564.abs();
                let v566 = if v564 > v0 { 1.0 } else { 0.0 };
                let v2527: f64;
                if v566 != 0.0 {
                    let v568 = (v564 * v558) / v529;
                    v2527 = v568;
                } else {
                    v2527 = v565;
                }
                v569 = v563;
                v2519 = v558;
                v2526 = v2527;
            } else {
                v569 = v43;
                v2519 = v529;
                v2526 = v564;
            }
            let v2525: f64;
            if v100 != 0.0 {
                v2525 = v279;
            } else {
                v2525 = v2526;
            }
            let v571 = v569 * v570;
            let v573 = v569 * v572;
            let v578 = v574 * (((v81 * v157) + v283).exp());
            let v716: f64;
            let v2672: f64;
            let v2680: f64;
            let v2690: f64;
            if v100 != 0.0 {
                let v580 = if v579 > v0 { 1.0 } else { 0.0 };
                let v2673: f64;
                let v2681: f64;
                let v2691: f64;
                if v580 != 0.0 {
                    let v599 = ((((v174 * v42) * (((((v582 * v63) * v44).exp()) - (((v586 * v582) * v44).exp())).ln())) * v156) + (v73 * (v43 - v156))) - ((v76 * v152) * v157);
                    let v611 = v599 + ((v174 * v152) * ((v63 * (v43 + ((v43 + (v195 * (((-v599) * v153).exp()))).sqrt()))).ln()));
                    let v617 = v579 * ((v612 * ((v582 / v611).ln())).exp());
                    let v2692: f64;
                    if v619 != 0.0 {
                        let v622 = (v620 * v611) / v582;
                        v2692 = v622;
                    } else {
                        v2692 = v618;
                    }
                    v2673 = v617;
                    v2681 = v611;
                    v2691 = v2692;
                } else {
                    v2673 = v579;
                    v2681 = v582;
                    v2691 = v623;
                }
                v716 = v279;
                v2672 = v2673;
                v2680 = v2681;
                v2690 = v2691;
            } else {
                let v624 = if v579 > v0 { 1.0 } else { 0.0 };
                let v2674: f64;
                let v2682: f64;
                let v2693: f64;
                if v624 != 0.0 {
                    let v642 = ((((v174 * v42) * (((((v582 * v63) * v44).exp()) - (((v629 * v582) * v44).exp())).ln())) * v156) + (v73 * (v43 - v156))) - ((v76 * v152) * v157);
                    let v654 = v642 + ((v174 * v152) * ((v63 * (v43 + ((v43 + (v195 * (((-v642) * v153).exp()))).sqrt()))).ln()));
                    let v659 = v579 * ((v612 * ((v582 / v654).ln())).exp());
                    let v661 = -v660;
                    let v662 = v661.abs();
                    let v663 = if v661 > v0 { 1.0 } else { 0.0 };
                    let v2694: f64;
                    if v663 != 0.0 {
                        let v665 = (v661 * v654) / v582;
                        v2694 = v665;
                    } else {
                        v2694 = v662;
                    }
                    v2674 = v659;
                    v2682 = v654;
                    v2693 = v2694;
                } else {
                    let v666 = -v660;
                    v2674 = v579;
                    v2682 = v582;
                    v2693 = v666;
                }
                v716 = v660;
                v2672 = v2674;
                v2680 = v2682;
                v2690 = v2693;
            }
            let v669 = v71 * v44;
            let v673 = v667 * (((v83 * v157) + (v669 * v222)).exp());
            let v677 = if v676 > v0 { 1.0 } else { 0.0 };
            let v2722: f64;
            let v2728: f64;
            let v2735: f64;
            if v677 != 0.0 {
                let v679 = if v678 > v0 { 1.0 } else { 0.0 };
                let v2723: f64;
                let v2729: f64;
                let v2736: f64;
                if v679 != 0.0 {
                    let v697 = ((((v174 * v42) * (((((v676 * v63) * v44).exp()) - (((v684 * v676) * v44).exp())).ln())) * v156) + (v73 * (v43 - v156))) - ((v76 * v152) * v157);
                    let v709 = v697 + ((v174 * v152) * ((v63 * (v43 + ((v43 + (v195 * (((-v697) * v153).exp()))).sqrt()))).ln()));
                    let v715 = v678 * ((v710 * ((v676 / v709).ln())).exp());
                    let v717 = -v716;
                    let v718 = v717.abs();
                    let v719 = if v717 > v0 { 1.0 } else { 0.0 };
                    let v2737: f64;
                    if v719 != 0.0 {
                        let v721 = (v717 * v709) / v676;
                        v2737 = v721;
                    } else {
                        v2737 = v718;
                    }
                    v2723 = v715;
                    v2729 = v709;
                    v2736 = v2737;
                } else {
                    let v722 = -v716;
                    v2723 = v678;
                    v2729 = v676;
                    v2736 = v722;
                }
                v2722 = v2723;
                v2728 = v2729;
                v2735 = v2736;
            } else {
                v2722 = v678;
                v2728 = v676;
                v2735 = v716;
            }
            let v727 = v723 * ((v724 * v157).exp());
            let v732 = v728 * ((v729 * v157).exp());
            let v737 = v733 * ((v734 * v157).exp());
            let v742 = if v738 >= v741 { 1.0 } else { 0.0 };
            let v744 = if v738 > v0 { 1.0 } else { 0.0 };
            let v745 = if (if (if v739 != v0 { 1.0 } else { 0.0 }) != 0.0 && v742 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v744 != 0.0 { 1.0 } else { 0.0 };
            let v1232: f64;
            let v1239: f64;
            let v1252: f64;
            let v1259: f64;
            let v1260: f64;
            let v1269: f64;
            let v1271: f64;
            let v1272: f64;
            let v1318: f64;
            let v1321: f64;
            let v1323: f64;
            let v1470: f64;
            let v1481: f64;
            let v1491: f64;
            let v1539: f64;
            let v1549: f64;
            let v1553: f64;
            let v1576: f64;
            let v1578: f64;
            let v1613: f64;
            let v1667: f64;
            let v1797: f64;
            let v1800: f64;
            let v2263: f64;
            let v2279: f64;
            let v2288: f64;
            let v2319: f64;
            let v2325: f64;
            let v2365: f64;
            let v2410: f64;
            let v2423: f64;
            let v2430: f64;
            let v2432: f64;
            let v2433: f64;
            let v2478: f64;
            let v2483: f64;
            let v2508: f64;
            let v2515: f64;
            let v2517: f64;
            let v2521: f64;
            let v2557: f64;
            let v2564: f64;
            let v2668: f64;
            let v2676: f64;
            let v2684: f64;
            let v2719: f64;
            let v2725: f64;
            let v2731: f64;
            let v2769: f64;
            let v2791: f64;
            let v2796: f64;
            let v2800: f64;
            let v2853: f64;
            if v745 != 0.0 {
                let v747 = v143 + v746;
                let v749 = if v747 < v748 { 1.0 } else { 0.0 };
                let v754: f64;
                if v749 != 0.0 {
                    v754 = v750;
                } else {
                    let v752 = if v747 > v751 { 1.0 } else { 0.0 };
                    let v755: f64;
                    if v752 != 0.0 {
                        v755 = v753;
                    } else {
                        v755 = v747;
                    }
                    v754 = v755;
                }
                let v756 = v39 * v754;
                let v757 = v43 / v756;
                let v758 = v754 - v35;
                let v759 = v35 / v754;
                let v760 = v754 / v35;
                let v761 = v760.ln();
                let v764 = (v45 * v754) * (v754.ln());
                let v765 = v49 * v754;
                let v767 = (v53 + v764) + v765;
                let v773 = (v767 + ((v56 + v764) + v765)) * v63;
                let v775 = (v767 + ((v59 + v764) + v765)) * v63;
                let v871: f64;
                let v1021: f64;
                let v1273: f64;
                if v173 != 0.0 {
                    let v793 = ((((v174 * v42) * (((((v176 * v63) * v44).exp()) - (((v780 * v176) * v44).exp())).ln())) * v760) + (v68 * (v43 - v760))) - ((v76 * v756) * v761);
                    let v805 = v793 + ((v174 * v756) * ((v63 * (v43 + ((v43 + (v195 * (((-v793) * v757).exp()))).sqrt()))).ln()));
                    let v810 = v172 * ((v207 * ((v176 / v805).ln())).exp());
                    let v811 = v213.abs();
                    let v812 = if v213 > v0 { 1.0 } else { 0.0 };
                    let v1274: f64;
                    if v812 != 0.0 {
                        let v814 = (v213 * v805) / v176;
                        v1274 = v814;
                    } else {
                        v1274 = v811;
                    }
                    v871 = v805;
                    v1021 = v810;
                    v1273 = v1274;
                } else {
                    v871 = v176;
                    v1021 = v172;
                    v1273 = v213;
                }
                let v816 = v43 - v759;
                let v819 = ((v219 * v761) + (v221 * v816)).exp();
                let v820 = v218 * v819;
                let v822 = v231 * v816;
                let v826 = v227 * (((v229 * v761) + (v822 / v228)).exp());
                let v940: f64;
                let v944: f64;
                let v1325: f64;
                if v105 != 0.0 {
                    let v844 = ((((v174 * v42) * (((((v238 * v63) * v44).exp()) - (((v831 * v238) * v44).exp())).ln())) * v760) + (v70 * (v43 - v760))) - ((v76 * v756) * v761);
                    let v856 = v844 + ((v174 * v756) * ((v63 * (v43 + ((v43 + (v195 * (((-v844) * v757).exp()))).sqrt()))).ln()));
                    let v861 = v104 * ((v268 * ((v238 / v856).ln())).exp());
                    let v862 = v274.abs();
                    let v863 = if v274 > v0 { 1.0 } else { 0.0 };
                    let v1326: f64;
                    if v863 != 0.0 {
                        let v865 = (v274 * v856) / v238;
                        v1326 = v865;
                    } else {
                        v1326 = v862;
                    }
                    v940 = v856;
                    v944 = v861;
                    v1325 = v1326;
                } else {
                    v940 = v238;
                    v944 = v104;
                    v1325 = v274;
                }
                let v1324: f64;
                if v100 != 0.0 {
                    v1324 = v279;
                } else {
                    v1324 = v1325;
                }
                let v867 = v282 * v816;
                let v870 = v280 * (((v79 * v761) + v867).exp());
                let v872 = v871 / v176;
                let v877 = v287 * (v174 - ((v207 * (v872.ln())).exp()));
                let v882 = v295 * (((v296 * v761) + (v298 * v816)).exp());
                let v885 = v303 * ((v304 * v761).exp());
                let v1482: f64;
                if v313 != 0.0 {
                    let v892 = v314 * (((v315 * v757) * (((v317 * v761).exp()) - v43)).exp());
                    v1482 = v892;
                } else {
                    let v899 = v308 * (((v315 * v757) * (((v317 * v761).exp()) - v43)).exp());
                    v1482 = v899;
                }
                let v902 = v331 * ((v332 * v816).exp());
                let v905 = v336 * ((v338 * v816).exp());
                let v908 = v342 * ((v344 * v816).exp());
                let v911 = v348 * ((v349 * v761).exp());
                let v915 = v43 / (v353 * ((v78 * v761).exp()));
                let v1550: f64;
                let v1554: f64;
                if v359 != 0.0 {
                    let v918 = v358 * (v43 - (v360 * v758));
                    v1550 = v918;
                    v1554 = v364;
                } else {
                    let v921 = v364 * (v43 + (v365 * v758));
                    v1550 = v358;
                    v1554 = v921;
                }
                let v927 = v114 * ((v43 + (v369 * v758)) + ((v372 * v758) * v758));
                let v930 = v378 * ((v379 * v761).exp());
                let v2320: f64;
                let v2326: f64;
                if v384 != 0.0 {
                    let v933 = v102 * ((v385 * v758).exp());
                    let v936 = v389 * ((v390 * v758).exp());
                    v2320 = v936;
                    v2326 = v933;
                } else {
                    v2320 = v389;
                    v2326 = v102;
                }
                let v2280: f64;
                let v2289: f64;
                if v397 != 0.0 {
                    let v938 = if v105 != 0.0 && (if v238 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2281: f64;
                    let v2290: f64;
                    if v938 != 0.0 {
                        let v939 = v66 / v775;
                        let v941 = v940 / v238;
                        let v946 = (((v939.sqrt()) * v941) * v944) / v104;
                        let v948 = (v394 * v946) * v941;
                        let v950 = v398 / (v946 * v939);
                        v2281 = v948;
                        v2290 = v950;
                    } else {
                        v2281 = v394;
                        v2290 = v398;
                    }
                    v2280 = v2281;
                    v2289 = v2290;
                } else {
                    v2280 = v0;
                    v2289 = v43;
                }
                let v953 = v413 * ((v414 * v761).exp());
                let v1004: f64;
                let v1006: f64;
                let v2434: f64;
                if v419 != 0.0 {
                    let v971 = ((((v174 * v42) * (((((v421 * v63) * v44).exp()) - (((v958 * v421) * v44).exp())).ln())) * v760) + (v68 * (v43 - v760))) - ((v76 * v756) * v761);
                    let v983 = v971 + ((v174 * v756) * ((v63 * (v43 + ((v43 + (v195 * (((-v971) * v757).exp()))).sqrt()))).ln()));
                    let v988 = v418 * ((v451 * ((v421 / v983).ln())).exp());
                    let v989 = v457.abs();
                    let v990 = if v457 > v0 { 1.0 } else { 0.0 };
                    let v2435: f64;
                    if v990 != 0.0 {
                        let v992 = (v457 * v983) / v421;
                        v2435 = v992;
                    } else {
                        v2435 = v989;
                    }
                    v1004 = v983;
                    v1006 = v988;
                    v2434 = v2435;
                } else {
                    v1004 = v421;
                    v1006 = v418;
                    v2434 = v457;
                }
                let v993 = v462 * v819;
                let v998 = v464 * (((v466 * v761) + (v822 / v465)).exp());
                let v2479: f64;
                let v2484: f64;
                if v478 != 0.0 {
                    let v999 = v64 / v773;
                    let v1003 = if (if (if v480 == v43 { 1.0 } else { 0.0 }) != 0.0 && v419 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v421 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v1032: f64;
                    let v1035: f64;
                    if v1003 != 0.0 {
                        let v1005 = v1004 / v421;
                        let v1011 = (((v1006 / v418) * (v999.sqrt())) * v1005) * v1005;
                        let v1016 = ((v418 / v1006) * (v999.powf(v1013))) / v1005;
                        v1032 = v1011;
                        v1035 = v1016;
                    } else {
                        let v1020 = if (if (if v480 == v0 { 1.0 } else { 0.0 }) != 0.0 && v173 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v176 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v1033: f64;
                        let v1036: f64;
                        if v1020 != 0.0 {
                            let v1026 = (((v1021 / v172) * (v999.sqrt())) * v872) * v872;
                            let v1031 = ((v172 / v1021) * (v999.powf(v1028))) / v872;
                            v1033 = v1026;
                            v1036 = v1031;
                        } else {
                            v1033 = v43;
                            v1036 = v43;
                        }
                        v1032 = v1033;
                        v1035 = v1036;
                    }
                    let v1034 = v472 * v1032;
                    let v1037 = v516 * v1035;
                    v2479 = v1034;
                    v2484 = v1037;
                } else {
                    v2479 = v0;
                    v2484 = v43;
                }
                let v1042 = v520 * (((-(v871 - v176)) / v523).exp());
                let v1082: f64;
                let v2518: f64;
                let v2523: f64;
                if v1043 != 0.0 {
                    let v1061 = ((((v174 * v42) * (((((v529 * v63) * v44).exp()) - (((v1048 * v529) * v44).exp())).ln())) * v760) + (v70 * (v43 - v760))) - ((v76 * v756) * v761);
                    let v1073 = v1061 + ((v174 * v756) * ((v63 * (v43 + ((v43 + (v195 * (((-v1061) * v757).exp()))).sqrt()))).ln()));
                    let v1077 = (v559 * ((v529 / v1073).ln())).exp();
                    let v1078 = v564.abs();
                    let v1079 = if v564 > v0 { 1.0 } else { 0.0 };
                    let v2524: f64;
                    if v1079 != 0.0 {
                        let v1081 = (v564 * v1073) / v529;
                        v2524 = v1081;
                    } else {
                        v2524 = v1078;
                    }
                    v1082 = v1077;
                    v2518 = v1073;
                    v2523 = v2524;
                } else {
                    v1082 = v43;
                    v2518 = v529;
                    v2523 = v564;
                }
                let v2522: f64;
                if v100 != 0.0 {
                    v2522 = v279;
                } else {
                    v2522 = v2523;
                }
                let v1083 = v1082 * v570;
                let v1084 = v1082 * v572;
                let v1088 = v574 * (((v81 * v761) + v867).exp());
                let v1214: f64;
                let v2669: f64;
                let v2677: f64;
                let v2685: f64;
                if v100 != 0.0 {
                    let v1089 = if v579 > v0 { 1.0 } else { 0.0 };
                    let v2670: f64;
                    let v2678: f64;
                    let v2686: f64;
                    if v1089 != 0.0 {
                        let v1107 = ((((v174 * v42) * (((((v582 * v63) * v44).exp()) - (((v1094 * v582) * v44).exp())).ln())) * v760) + (v73 * (v43 - v760))) - ((v76 * v756) * v761);
                        let v1119 = v1107 + ((v174 * v756) * ((v63 * (v43 + ((v43 + (v195 * (((-v1107) * v757).exp()))).sqrt()))).ln()));
                        let v1124 = v579 * ((v612 * ((v582 / v1119).ln())).exp());
                        let v2687: f64;
                        if v1126 != 0.0 {
                            let v1129 = (v1127 * v1119) / v582;
                            v2687 = v1129;
                        } else {
                            v2687 = v1125;
                        }
                        v2670 = v1124;
                        v2678 = v1119;
                        v2686 = v2687;
                    } else {
                        v2670 = v579;
                        v2678 = v582;
                        v2686 = v1130;
                    }
                    v1214 = v279;
                    v2669 = v2670;
                    v2677 = v2678;
                    v2685 = v2686;
                } else {
                    let v1131 = if v579 > v0 { 1.0 } else { 0.0 };
                    let v2671: f64;
                    let v2679: f64;
                    let v2688: f64;
                    if v1131 != 0.0 {
                        let v1149 = ((((v174 * v42) * (((((v582 * v63) * v44).exp()) - (((v1136 * v582) * v44).exp())).ln())) * v760) + (v73 * (v43 - v760))) - ((v76 * v756) * v761);
                        let v1161 = v1149 + ((v174 * v756) * ((v63 * (v43 + ((v43 + (v195 * (((-v1149) * v757).exp()))).sqrt()))).ln()));
                        let v1166 = v579 * ((v612 * ((v582 / v1161).ln())).exp());
                        let v1167 = -v660;
                        let v1168 = v1167.abs();
                        let v1169 = if v1167 > v0 { 1.0 } else { 0.0 };
                        let v2689: f64;
                        if v1169 != 0.0 {
                            let v1171 = (v1167 * v1161) / v582;
                            v2689 = v1171;
                        } else {
                            v2689 = v1168;
                        }
                        v2671 = v1166;
                        v2679 = v1161;
                        v2688 = v2689;
                    } else {
                        let v1172 = -v660;
                        v2671 = v579;
                        v2679 = v582;
                        v2688 = v1172;
                    }
                    v1214 = v660;
                    v2669 = v2671;
                    v2677 = v2679;
                    v2685 = v2688;
                }
                let v1177 = v667 * (((v83 * v761) + (v669 * v816)).exp());
                let v2720: f64;
                let v2726: f64;
                let v2732: f64;
                if v677 != 0.0 {
                    let v1178 = if v678 > v0 { 1.0 } else { 0.0 };
                    let v2721: f64;
                    let v2727: f64;
                    let v2733: f64;
                    if v1178 != 0.0 {
                        let v1196 = ((((v174 * v42) * (((((v676 * v63) * v44).exp()) - (((v1183 * v676) * v44).exp())).ln())) * v760) + (v73 * (v43 - v760))) - ((v76 * v756) * v761);
                        let v1208 = v1196 + ((v174 * v756) * ((v63 * (v43 + ((v43 + (v195 * (((-v1196) * v757).exp()))).sqrt()))).ln()));
                        let v1213 = v678 * ((v710 * ((v676 / v1208).ln())).exp());
                        let v1215 = -v1214;
                        let v1216 = v1215.abs();
                        let v1217 = if v1215 > v0 { 1.0 } else { 0.0 };
                        let v2734: f64;
                        if v1217 != 0.0 {
                            let v1219 = (v1215 * v1208) / v676;
                            v2734 = v1219;
                        } else {
                            v2734 = v1216;
                        }
                        v2721 = v1213;
                        v2727 = v1208;
                        v2733 = v2734;
                    } else {
                        let v1220 = -v1214;
                        v2721 = v678;
                        v2727 = v676;
                        v2733 = v1220;
                    }
                    v2720 = v2721;
                    v2726 = v2727;
                    v2732 = v2733;
                } else {
                    v2720 = v678;
                    v2726 = v676;
                    v2732 = v1214;
                }
                let v1223 = v723 * ((v724 * v761).exp());
                let v1226 = v728 * ((v729 * v761).exp());
                let v1229 = v733 * ((v734 * v761).exp());
                v1232 = v756;
                v1239 = v820;
                v1252 = v826;
                v1259 = v882;
                v1260 = v757;
                v1269 = v1021;
                v1271 = v871;
                v1272 = v1273;
                v1318 = v944;
                v1321 = v940;
                v1323 = v1324;
                v1470 = v885;
                v1481 = v1482;
                v1491 = v877;
                v1539 = v927;
                v1549 = v1550;
                v1553 = v1554;
                v1576 = v911;
                v1578 = v915;
                v1613 = v902;
                v1667 = v930;
                v1797 = v905;
                v1800 = v908;
                v2263 = v870;
                v2279 = v2280;
                v2288 = v2289;
                v2319 = v2320;
                v2325 = v2326;
                v2365 = v953;
                v2410 = v993;
                v2423 = v998;
                v2430 = v1006;
                v2432 = v1004;
                v2433 = v2434;
                v2478 = v2479;
                v2483 = v2484;
                v2508 = v1042;
                v2515 = v1084;
                v2517 = v2518;
                v2521 = v2522;
                v2557 = v1088;
                v2564 = v1083;
                v2668 = v2669;
                v2676 = v2677;
                v2684 = v2685;
                v2719 = v2720;
                v2725 = v2726;
                v2731 = v2732;
                v2769 = v1177;
                v2791 = v1229;
                v2796 = v1223;
                v2800 = v1226;
                v2853 = v754;
            } else {
                v1232 = v152;
                v1239 = v226;
                v1252 = v236;
                v1259 = v302;
                v1260 = v153;
                v1269 = v502;
                v1271 = v288;
                v1272 = v1275;
                v1318 = v406;
                v1321 = v402;
                v1323 = v1327;
                v1470 = v307;
                v1481 = v1483;
                v1491 = v294;
                v1539 = v376;
                v1549 = v1551;
                v1553 = v1555;
                v1576 = v352;
                v1578 = v357;
                v1613 = v335;
                v1667 = v382;
                v1797 = v341;
                v1800 = v347;
                v2263 = v286;
                v2279 = v2282;
                v2288 = v2291;
                v2319 = v2321;
                v2325 = v2327;
                v2365 = v417;
                v2410 = v463;
                v2423 = v471;
                v2430 = v487;
                v2432 = v485;
                v2433 = v2436;
                v2478 = v2480;
                v2483 = v2485;
                v2508 = v526;
                v2515 = v573;
                v2517 = v2519;
                v2521 = v2525;
                v2557 = v578;
                v2564 = v571;
                v2668 = v2672;
                v2676 = v2680;
                v2684 = v2690;
                v2719 = v2722;
                v2725 = v2728;
                v2731 = v2735;
                v2769 = v673;
                v2791 = v737;
                v2796 = v727;
                v2800 = v732;
                v2853 = v150;
            }
            let v1230 = if v218 > v0 { 1.0 } else { 0.0 };
            let v2380: f64;
            if v1230 != 0.0 {
                let v1234 = v5 / (v1231 * v1232);
                let v1236 = if v1234 > v1235 { 1.0 } else { 0.0 };
                let v1240: f64;
                let v1241: f64;
                if v1236 != 0.0 {
                    let v1238 = v43 + (v1234 - v1235);
                    v1240 = v1238;
                    v1241 = v1235;
                } else {
                    v1240 = v43;
                    v1241 = v1234;
                }
                let v1245 = v1239 * ((v1240 * (rspice_limexp(v1241))) - v43);
                v2380 = v1245;
            } else {
                v2380 = v0;
            }
            let v1246 = if v227 > v0 { 1.0 } else { 0.0 };
            let v2807: f64;
            if v1246 != 0.0 {
                let v1248 = v5 / (v228 * v1232);
                let v1249 = if v1248 > v1235 { 1.0 } else { 0.0 };
                let v1253: f64;
                let v1254: f64;
                if v1249 != 0.0 {
                    let v1251 = v43 + (v1248 - v1235);
                    v1253 = v1251;
                    v1254 = v1235;
                } else {
                    v1253 = v43;
                    v1254 = v1248;
                }
                let v1258 = v1252 * ((v1253 * (rspice_limexp(v1254))) - v43);
                v2807 = v1258;
            } else {
                v2807 = v0;
            }
            let v1265 = v1259 * (rspice_limexp(((v5 * v1260) / v1262)));
            let v1268 = v1259 * (rspice_limexp((v8 * v1260)));
            let v1270 = if v1269 > v0 { 1.0 } else { 0.0 };
            let v1494: f64;
            let v2252: f64;
            if v1270 != 0.0 {
                let v1282 = v1271 * (v43 - (((-(v1272.ln())) / v207).exp()));
                let v1284 = (v1282 - v5) * v1260;
                let v1288 = ((v1284 * v1284) + v1286).sqrt();
                let v1290 = (v1284 + v1288) * v63;
                let v1292 = v1282 - (v1232 * v1290);
                let v1293 = v1290 / v1288;
                let v1296 = (v43 - (v1292 / v1271)).ln();
                let v1304 = v1269 * (((((-v207) * v1296).exp()) * v1293) + (v1272 * (v43 - v1293)));
                let v1305 = v43 - v207;
                let v1314 = v1269 * (((v1271 * (v43 - ((v1296 * v1305).exp()))) / v1305) + (v1272 * (v5 - v1292)));
                v1494 = v1314;
                v2252 = v1304;
            } else {
                v1494 = v0;
                v2252 = v0;
            }
            let v1317 = if v1315 < v1316 { 1.0 } else { 0.0 };
            let v1498: f64;
            let v2253: f64;
            if v1317 != 0.0 {
                let v1319 = if v1318 > v0 { 1.0 } else { 0.0 };
                let v1499: f64;
                let v2254: f64;
                if v1319 != 0.0 {
                    let v1320 = v268 / v195;
                    let v1322 = v1315 - v1321;
                    let v1335 = v1321 * (v43 - (((-(v1323.ln())) / v268).exp()));
                    let v1336 = v1323 * v1318;
                    let v1342 = v1318 * (((v1320 - v268) * ((v1315 / v1321).ln())).exp());
                    let v1344 = (v1335 - v8) * v1260;
                    let v1345 = if v1344 < v1235 { 1.0 } else { 0.0 };
                    let v1356: f64;
                    let v1386: f64;
                    if v1345 != 0.0 {
                        let v1346 = v1344.exp();
                        let v1347 = v43 + v1346;
                        let v1348 = v1346 / v1347;
                        let v1351 = v1335 - (v1232 * (v1347.ln()));
                        v1356 = v1351;
                        v1386 = v1348;
                    } else {
                        v1356 = v8;
                        v1386 = v43;
                    }
                    let v1355 = (v1352 * v1322) + (v195 * v1232);
                    let v1358 = (v1322 + v1356) / v1355;
                    let v1359 = if v1358 < v1235 { 1.0 } else { 0.0 };
                    let v1376: f64;
                    let v1388: f64;
                    if v1359 != 0.0 {
                        let v1360 = v1358.exp();
                        let v1361 = v43 + v1360;
                        let v1362 = v1360 / v1361;
                        let v1371 = (-v1322) + (v1355 * ((v1361.ln()) - (((-(v1322 + v1335)) / v1355).exp())));
                        v1376 = v1371;
                        v1388 = v1362;
                    } else {
                        v1376 = v1356;
                        v1388 = v43;
                    }
                    let v1375 = (v43 - (v1356 / v1321)).ln();
                    let v1379 = (v43 - (v1376 / v1321)).ln();
                    let v1380 = v43 - v268;
                    let v1381 = v43 - v1320;
                    let v1399 = ((((v1318 * ((v1379 * (-v268)).exp())) * v1386) * v1388) + ((v1342 * ((v1375 * (-v1320)).exp())) * (v43 - v1388))) + (v1336 * (v43 - v1386));
                    let v1419 = (((((v1318 * (v43 - ((v1379 * v1380).exp()))) / v1380) + ((v1342 * (v43 - ((v1375 * v1381).exp()))) / v1381)) - ((v1342 * (v43 - ((v1379 * v1381).exp()))) / v1381)) * v1321) + (v1336 * (v8 - v1356));
                    v1499 = v1419;
                    v2254 = v1399;
                } else {
                    v1499 = v0;
                    v2254 = v0;
                }
                v1498 = v1499;
                v2253 = v2254;
            } else {
                let v1420 = if v1318 > v0 { 1.0 } else { 0.0 };
                let v1500: f64;
                let v2255: f64;
                if v1420 != 0.0 {
                    let v1426 = v1321 * (v43 - (((-(v1323.ln())) / v268).exp()));
                    let v1428 = (v1426 - v8) * v1260;
                    let v1431 = ((v1428 * v1428) + v1286).sqrt();
                    let v1433 = (v1428 + v1431) * v63;
                    let v1435 = v1426 - (v1232 * v1433);
                    let v1436 = v1433 / v1431;
                    let v1439 = (v43 - (v1435 / v1321)).ln();
                    let v1447 = v1318 * (((((-v268) * v1439).exp()) * v1436) + (v1323 * (v43 - v1436)));
                    let v1448 = v43 - v268;
                    let v1457 = v1318 * (((v1321 * (v43 - ((v1439 * v1448).exp()))) / v1448) + (v1323 * (v8 - v1435)));
                    v1500 = v1457;
                    v2255 = v1447;
                } else {
                    v1500 = v0;
                    v2255 = v0;
                }
                v1498 = v1500;
                v2253 = v2255;
            }
            let v1458 = if v303 > v0 { 1.0 } else { 0.0 };
            let v1492: f64;
            if v1458 != 0.0 {
                let v1460 = v1459 * v1232;
                let v1462 = (v1271 - v5) / v1460;
                let v1477 = v1470 * (v43 - ((v207 * ((v43 - ((v1271 - ((v1460 * (v1462 + (((v1462 * v1462) + v1286).sqrt()))) * v63)) / v1271)).ln())).exp()));
                let v1480 = if (v1477.abs()) > v1479 { 1.0 } else { 0.0 };
                let v1493: f64;
                if v1480 != 0.0 {
                    let v1487 = (v1481 * ((v1477.exp()) - v43)) / v1477;
                    v1493 = v1487;
                } else {
                    let v1490 = v1481 * (v43 + (v1477 * v63));
                    v1493 = v1490;
                }
                v1492 = v1493;
            } else {
                v1492 = v1481;
            }
            let v1504 = v1503 * v1491;
            let v1506 = (((v1491 + (v1492 * v1494)) + (v1497 * v1498)) / v1504) - v43;
            let v1513 = v1504 * (v43 + ((v1506 + (((v1506 * v1506) + v1286).sqrt())) * v63));
            let v1518 = v1321 * (v43 - ((v1514 / v268).exp()));
            let v1520 = (v1518 - v8) * v1260;
            let v1523 = ((v1520 * v1520) + v1286).sqrt();
            let v1525 = (v1520 + v1523) * v63;
            let v1528 = v1525 / v1523;
            let v1538 = ((((-v268) * ((v43 - ((v1518 - (v1232 * v1525)) / v1321)).ln())).exp()) * v1528) + (v279 * (v43 - v1528));
            let v1548 = (v1539 + (v1540 * ((v43 / v1538) - v43))) + (v1545 * (v1538 - v43));
            let v1557: f64;
            if v359 != 0.0 {
                let v1552 = v1549 - v8;
                v1557 = v1552;
            } else {
                let v1556 = v9 - v1553;
                v1557 = v1556;
            }
            let v1575: f64;
            if v100 != 0.0 {
                let v1559 = (v1557 - v1232) * v1260;
                let v1566 = v1232 + (v1232 * ((v1559 + (((v1559 * v1559) + v1286).sqrt())) * v63));
                v1575 = v1566;
            } else {
                let v1567 = v1557 / v41;
                let v1574 = v41 * ((v1567 + (((v1567 * v1567) + v1569).sqrt())) * v63);
                v1575 = v1574;
            }
            let v1591 = (v1575 - v1576) / v1590;
            let v1599 = ((v1575 * v1578) / ((((v43 + ((v1580 * ((v1575 / v1576).ln())).exp())).ln()) / v1580).exp())) * (v43 + (v63 * (v1591 + (((v1591 * v1591) + v1593).sqrt()))));
            let v1603 = if (if v1548 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v1601 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1621: f64;
            if v1603 != 0.0 {
                let v1604 = v63 * v1513;
                let v1622: f64;
                if v100 != 0.0 {
                    let v1611 = v1604 + ((((v1604 * v1604) + (v1548 * v1265)) + (v1601 * v1268)).sqrt());
                    v1622 = v1611;
                } else {
                    let v1620 = v1604 + ((((v1604 * v1604) + ((v1613 * v1539) * v1265)) + (v1601 * v1268)).sqrt());
                    v1622 = v1620;
                }
                v1621 = v1622;
            } else {
                v1621 = v1513;
            }
            let v1623 = v1265 / v1621;
            let v1624 = v1268 / v1621;
            let v1625 = v1548 * v1623;
            let v1626 = if v25 >= v26 { 1.0 } else { 0.0 };
            let v1793: f64;
            if v1626 != 0.0 {
                let v1628 = (v1613 * v1539) * v1623;
                v1793 = v1628;
            } else {
                let v1629 = v1613 * v1625;
                v1793 = v1629;
            }
            let v1631 = v1630 * v1599;
            let v1634 = if v25 >= v1633 { 1.0 } else { 0.0 };
            let v1635 = if (if v1623 >= v1631 { 1.0 } else { 0.0 }) != 0.0 || v1634 != 0.0 { 1.0 } else { 0.0 };
            let v1813: f64;
            let v1819: f64;
            let v2362: f64;
            if v1635 != 0.0 {
                let v1636 = v1623 / v1599;
                let v1644 = ((v377 * ((v1637 * (v1636.ln())).exp())) * v1623) / (v43 + v1637);
                let v1648 = if v1645 < (v1503 * (v348 / v353)) { 1.0 } else { 0.0 };
                let v1669: f64;
                if v1648 != 0.0 {
                    v1669 = v0;
                } else {
                    let v1650 = (v1623 - v1599) / v1645;
                    let v1652 = if v1650 < v1651 { 1.0 } else { 0.0 };
                    let v1654: f64;
                    if v1652 != 0.0 {
                        v1654 = v1653;
                    } else {
                        v1654 = v1650;
                    }
                    let v1664 = v1659 * ((v1660 / (v1654 + (((v1654 * v1654) + v1656).sqrt()))).exp());
                    v1669 = v1664;
                }
                let v1666 = v43 - v1665;
                let v1673 = (v1666 * v1667) * (((v1669 * v1260).exp()) - v43);
                let v1675 = v43 - (v43 / v1636);
                let v1684 = (v1675 + (((v1675 * v1675) + v1677).sqrt())) / (v43 + ((v43 + v1677).sqrt()));
                let v1687 = ((v1669 - v1659) * v1260).exp();
                let v1690 = ((v1667 * v1684) * v1684) * v1687;
                let v1700 = if (if (if (if v117 < v118 { 1.0 } else { 0.0 }) != 0.0 && (if v120 < v118 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v1684 * v117) < v1695 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v1684 * v120) < v1695 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v1791: f64;
                if v1700 != 0.0 {
                    let v1702 = (v1665 * v1690) * v1623;
                    v1791 = v1702;
                } else {
                    let v1703 = v43 - v1684;
                    let v1704 = v1703 - v43;
                    let v1707 = if (v1705.abs()) > v1479 { 1.0 } else { 0.0 };
                    let v1781: f64;
                    if v1707 != 0.0 {
                        let v1711 = (v1704 * v1708).exp();
                        let v1713 = if v1712 < v118 { 1.0 } else { 0.0 };
                        let v1782: f64;
                        if v1713 != 0.0 {
                            let v1717 = (v43 - v1711) / (v1711 * v1715);
                            let v1718 = v1715 * v1717;
                            let v1730 = ((v174 * ((v1718 * (v63 + ((v1720 * v1715) * v1717))) - (v63 * ((v43 + v1718).ln())))) / v1715) / v1715;
                            v1782 = v1730;
                        } else {
                            let v1734 = (v1711 - v43) / (v120 - (v1711 * v117));
                            let v1742 = v1738 * v1740;
                            let v1757 = v1753 * v1755;
                            let v1766 = ((((((v43 + (v120 * v1734)).ln()) * (v63 - v1742)) * v1740) + ((v1742 + (v1738 * v1734)) * v1734)) - (((((v43 + (v117 * v1734)).ln()) * (v63 - v1757)) * v1755) + ((v1757 + (v1753 * v1734)) * v1734))) / v1705;
                            v1782 = v1766;
                        }
                        v1781 = v1782;
                    } else {
                        let v1770 = (v43 - v1703) / (v43 + (v1703 * v117));
                        let v1778 = ((v1770 * v1770) * (v43 + ((v1738 * v174) * v1770))) / (v43 + (v117 * v1770));
                        v1781 = v1778;
                    }
                    let v1784 = (((v1665 * v1667) * v1687) * v1781) * v1623;
                    v1791 = v1784;
                }
                let v1788 = (v1673 * v1623) + ((v1666 * v1690) * v1623);
                let v1814: f64;
                let v1820: f64;
                if v1626 != 0.0 {
                    let v1792 = ((v1625 + v1788) + v1644) + v1791;
                    let v1802 = ((v1793 + (v1794 * v1788)) + (v1797 * v1644)) + (v1800 * v1791);
                    v1814 = v1802;
                    v1820 = v1792;
                } else {
                    let v1808 = (((v1613 * v1625) + v1788) + (v1797 * v1644)) + (v1800 * v1791);
                    let v1811 = ((v1625 + v1788) + v1644) + v1791;
                    v1814 = v1808;
                    v1820 = v1811;
                }
                v1813 = v1814;
                v1819 = v1820;
                v2362 = v1788;
            } else {
                v1813 = v1793;
                v1819 = v1625;
                v2362 = v0;
            }
            let v1812 = v1601 * v1624;
            let v1825 = if (if v1626 != 0.0 && (if v1813 > ((ctx.simparam_or("reltol", v311)) * v1621) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v100 != 0.0 && (if v1819 > ((ctx.simparam_or("reltol", v311)) * v1621) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v2245: f64;
            let v2246: f64;
            let v2248: f64;
            let v2360: f64;
            if v1825 != 0.0 {
                let v1831 = (v1513 + ((v1625 * v1813).sqrt())) + (v1829 * v1812);
                let mut v1832: f64 = 0.0;
                let mut v1835: f64 = 0.0;
                let mut v1839: f64 = 0.0;
                v1832 = v1831;
                v1835 = v1831;
                v1839 = v0;
                loop {
                    let v1841 = if (if (v1832.abs()) >= ((ctx.simparam_or("reltol", v311)) * (v1835.abs())) { 1.0 } else { 0.0 }) != 0.0 && (if v1839 <= v1316 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v1841 == 0.0 {
                        break;
                    }
                    let v1842 = v1265 / v1835;
                    let v1843 = v1268 / v1835;
                    let v1844 = v1548 * v1842;
                    let v2049: f64;
                    let v2056: f64;
                    if v1626 != 0.0 {
                        let v1845 = v1613 * v1539;
                        let v1846 = v1845 * v1842;
                        v2049 = v1846;
                        v2056 = v1845;
                    } else {
                        let v1847 = v1613 * v1844;
                        let v1848 = v1613 * v1548;
                        v2049 = v1847;
                        v2056 = v1848;
                    }
                    let v1850 = if (if v1842 >= v1631 { 1.0 } else { 0.0 }) != 0.0 || v1634 != 0.0 { 1.0 } else { 0.0 };
                    let v2078: f64;
                    let v2084: f64;
                    if v1850 != 0.0 {
                        let v1851 = v1842 / v1599;
                        let v1855 = v377 * ((v1637 * (v1851.ln())).exp());
                        let v1858 = (v1855 * v1842) / (v43 + v1637);
                        let v1861 = if v1645 < (v1503 * (v348 / v353)) { 1.0 } else { 0.0 };
                        let v1882: f64;
                        let v1890: f64;
                        if v1861 != 0.0 {
                            v1882 = v0;
                            v1890 = v0;
                        } else {
                            let v1863 = (v1842 - v1599) / v1645;
                            let v1865 = if v1863 < v1864 { 1.0 } else { 0.0 };
                            let v1867: f64;
                            if v1865 != 0.0 {
                                v1867 = v1866;
                            } else {
                                v1867 = v1863;
                            }
                            let v1870 = ((v1867 * v1867) + v1656).sqrt();
                            let v1872 = v1867 + v1870;
                            let v1875 = v1659 * ((v1871 / v1872).exp());
                            let v1879 = (v174 * v1875) / ((v1645 * v1870) * v1872);
                            v1882 = v1875;
                            v1890 = v1879;
                        }
                        let v1880 = v43 - v1665;
                        let v1881 = v1880 * v1667;
                        let v1884 = (v1882 * v1260).exp();
                        let v1886 = v1881 * (v1884 - v43);
                        let v1892 = v1886 + ((((v1881 * v1842) * v1884) * v1260) * v1890);
                        let v1894 = v43 - (v43 / v1851);
                        let v1897 = ((v1894 * v1894) + v1677).sqrt();
                        let v1902 = (v1894 + v1897) / (v43 + ((v43 + v1677).sqrt()));
                        let v1905 = ((v1882 - v1659) * v1260).exp();
                        let v1908 = ((v1667 * v1902) * v1902) * v1905;
                        let v1915 = v1908 * ((v43 + (v174 / (v1851 * v1897))) + ((v1260 * v1842) * v1890));
                        let v1924 = if (if (if (if v117 < v118 { 1.0 } else { 0.0 }) != 0.0 && (if v120 < v118 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v1902 * v117) < v1695 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v1902 * v120) < v1695 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v2046: f64;
                        let v2048: f64;
                        if v1924 != 0.0 {
                            let v1926 = (v1665 * v1908) * v1842;
                            let v1927 = v1665 * v1915;
                            v2046 = v1926;
                            v2048 = v1927;
                        } else {
                            let v1928 = v43 - v1902;
                            let v1929 = v1928 - v43;
                            let v1933 = (v1929 * (v43 - v1894)) / (v1897 * v1842);
                            let v1935 = if (v1705.abs()) > v1479 { 1.0 } else { 0.0 };
                            let v2029: f64;
                            let v2037: f64;
                            if v1935 != 0.0 {
                                let v1937 = (v1929 * v1708).exp();
                                let v1938 = if v1712 < v118 { 1.0 } else { 0.0 };
                                let v2030: f64;
                                let v2038: f64;
                                if v1938 != 0.0 {
                                    let v1940 = v1937 * v1715;
                                    let v1941 = (v43 - v1937) / v1940;
                                    let v1942 = v1715 * v1941;
                                    let v1943 = v43 + v1942;
                                    let v1953 = ((v174 * ((v1942 * (v63 + ((v1720 * v1715) * v1941))) - (v63 * (v1943.ln())))) / v1715) / v1715;
                                    let v1960 = (((v43 + v1943) * v1941) * (((-v1708) * v1933) / v1940)) / v1943;
                                    v2030 = v1953;
                                    v2038 = v1960;
                                } else {
                                    let v1962 = v120 - (v1937 * v117);
                                    let v1964 = (v1937 - v43) / v1962;
                                    let v1966 = v43 + (v120 * v1964);
                                    let v1968 = v1738 * v1740;
                                    let v1969 = v63 - v1968;
                                    let v1972 = v1738 * v1964;
                                    let v1981 = v43 + (v117 * v1964);
                                    let v1983 = v1753 * v1755;
                                    let v1984 = v63 - v1983;
                                    let v1987 = v1753 * v1964;
                                    let v1996 = (((((v1966.ln()) * v1969) * v1740) + ((v1968 + v1972) * v1964)) - ((((v1981.ln()) * v1984) * v1755) + ((v1983 + v1987) * v1964))) / v1705;
                                    let v2006 = (((((v1969 / v1966) + v1968) + (v1972 * v174)) - (((v1984 / v1981) + v1983) + (v1987 * v174))) * (((((v1997 * v1705) / (v1962 * v1962)) * v1937) * v1708) * v1933)) / v1705;
                                    v2030 = v1996;
                                    v2038 = v2006;
                                }
                                v2029 = v2030;
                                v2037 = v2038;
                            } else {
                                let v2009 = v43 + (v1928 * v117);
                                let v2010 = (v43 - v1928) / v2009;
                                let v2012 = v43 + (v117 * v2010);
                                let v2018 = ((v2010 * v2010) * (v43 + ((v1738 * v174) * v2010))) / v2012;
                                let v2026 = (v2010 * (v43 + (v43 / (v2012 * v2012)))) * (((-v1933) * v2012) / v2009);
                                v2029 = v2018;
                                v2037 = v2026;
                            }
                            let v2028 = (v1665 * v1667) * v1905;
                            let v2031 = v2028 * v2029;
                            let v2032 = v2031 * v1842;
                            let v2040 = (v2031 + ((v2032 * v1890) * v1260)) + ((v2028 * v1842) * v2037);
                            v2046 = v2032;
                            v2048 = v2040;
                        }
                        let v2043 = v1880 * v1915;
                        let v2045 = (v1886 * v1842) + ((v1880 * v1908) * v1842);
                        let v2079: f64;
                        let v2085: f64;
                        if v1626 != 0.0 {
                            let v2055 = ((v2049 + (v1794 * v2045)) + (v1797 * v1858)) + (v1800 * v2046);
                            let v2062 = ((v2056 + (v1794 * (v1892 + v2043))) + (v1797 * v1855)) + (v1800 * v2048);
                            v2079 = v2055;
                            v2085 = v2062;
                        } else {
                            let v2068 = (((v1613 * v1844) + v2045) + (v1797 * v1858)) + (v1800 * v2046);
                            let v2075 = (((v1613 * v1548) + (v1892 + v2043)) + (v1797 * v1855)) + (v1800 * v2048);
                            v2079 = v2068;
                            v2085 = v2075;
                        }
                        v2078 = v2079;
                        v2084 = v2085;
                    } else {
                        v2078 = v2049;
                        v2084 = v2056;
                    }
                    let v2077 = (v1829 * v1601) * v1843;
                    let v2090 = (-(v1835 - ((v1513 + v2078) + v2077))) / (v43 + (((v2084 * v1842) + v2077) / v1835));
                    let v2093 = (v2091 * v1835).abs();
                    let v2095 = if (v2090.abs()) > v2093 { 1.0 } else { 0.0 };
                    let v2098: f64;
                    if v2095 != 0.0 {
                        let v2096 = if v2090 >= v0 { 1.0 } else { 0.0 };
                        let v2099: f64;
                        if v2096 != 0.0 {
                            v2099 = v2093;
                        } else {
                            let v2097 = -v2093;
                            v2099 = v2097;
                        }
                        v2098 = v2099;
                    } else {
                        v2098 = v2090;
                    }
                    let v2100 = v1835 + v2098;
                    let v2101 = v1839 + v43;
                    v1832 = v2098;
                    v1835 = v2100;
                    v1839 = v2101;
                }
                let v2102 = v1265 / v1835;
                let v2103 = v1268 / v1835;
                let v2104 = v1548 * v2102;
                if v1626 != 0.0 {
                } else {
                }
                let v2106 = if (if v2102 >= v1631 { 1.0 } else { 0.0 }) != 0.0 || v1634 != 0.0 { 1.0 } else { 0.0 };
                let v2249: f64;
                let v2361: f64;
                if v2106 != 0.0 {
                    let v2107 = v2102 / v1599;
                    let v2114 = ((v377 * ((v1637 * (v2107.ln())).exp())) * v2102) / (v43 + v1637);
                    let v2117 = if v1645 < (v1503 * (v348 / v353)) { 1.0 } else { 0.0 };
                    let v2134: f64;
                    if v2117 != 0.0 {
                        v2134 = v0;
                    } else {
                        let v2119 = (v2102 - v1599) / v1645;
                        let v2121 = if v2119 < v2120 { 1.0 } else { 0.0 };
                        let v2123: f64;
                        if v2121 != 0.0 {
                            v2123 = v2122;
                        } else {
                            v2123 = v2119;
                        }
                        let v2131 = v1659 * ((v2127 / (v2123 + (((v2123 * v2123) + v1656).sqrt()))).exp());
                        v2134 = v2131;
                    }
                    let v2132 = v43 - v1665;
                    let v2138 = (v2132 * v1667) * (((v2134 * v1260).exp()) - v43);
                    let v2140 = v43 - (v43 / v2107);
                    let v2148 = (v2140 + (((v2140 * v2140) + v1677).sqrt())) / (v43 + ((v43 + v1677).sqrt()));
                    let v2151 = ((v2134 - v1659) * v1260).exp();
                    let v2154 = ((v1667 * v2148) * v2148) * v2151;
                    let v2163 = if (if (if (if v117 < v118 { 1.0 } else { 0.0 }) != 0.0 && (if v120 < v118 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v2148 * v117) < v1695 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (v2148 * v120) < v1695 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2240: f64;
                    if v2163 != 0.0 {
                        let v2165 = (v1665 * v2154) * v2102;
                        v2240 = v2165;
                    } else {
                        let v2166 = v43 - v2148;
                        let v2167 = v2166 - v43;
                        let v2169 = if (v1705.abs()) > v1479 { 1.0 } else { 0.0 };
                        let v2230: f64;
                        if v2169 != 0.0 {
                            let v2171 = (v2167 * v1708).exp();
                            let v2172 = if v1712 < v118 { 1.0 } else { 0.0 };
                            let v2231: f64;
                            if v2172 != 0.0 {
                                let v2175 = (v43 - v2171) / (v2171 * v1715);
                                let v2176 = v1715 * v2175;
                                let v2187 = ((v174 * ((v2176 * (v63 + ((v1720 * v1715) * v2175))) - (v63 * ((v43 + v2176).ln())))) / v1715) / v1715;
                                v2231 = v2187;
                            } else {
                                let v2191 = (v2171 - v43) / (v120 - (v2171 * v117));
                                let v2195 = v1738 * v1740;
                                let v2206 = v1753 * v1755;
                                let v2215 = ((((((v43 + (v120 * v2191)).ln()) * (v63 - v2195)) * v1740) + ((v2195 + (v1738 * v2191)) * v2191)) - (((((v43 + (v117 * v2191)).ln()) * (v63 - v2206)) * v1755) + ((v2206 + (v1753 * v2191)) * v2191))) / v1705;
                                v2231 = v2215;
                            }
                            v2230 = v2231;
                        } else {
                            let v2219 = (v43 - v2166) / (v43 + (v2166 * v117));
                            let v2227 = ((v2219 * v2219) * (v43 + ((v1738 * v174) * v2219))) / (v43 + (v117 * v2219));
                            v2230 = v2227;
                        }
                        let v2233 = (((v1665 * v1667) * v2151) * v2230) * v2102;
                        v2240 = v2233;
                    }
                    let v2237 = (v2138 * v2102) + ((v2132 * v2154) * v2102);
                    let v2250: f64;
                    if v1626 != 0.0 {
                        let v2241 = ((v2104 + v2237) + v2114) + v2240;
                        v2250 = v2241;
                    } else {
                        let v2244 = ((v2104 + v2237) + v2114) + v2240;
                        v2250 = v2244;
                    }
                    v2249 = v2250;
                    v2361 = v2237;
                } else {
                    v2249 = v2104;
                    v2361 = v0;
                }
                v2245 = v2102;
                v2246 = v2103;
                v2248 = v2249;
                v2360 = v2361;
            } else {
                v2245 = v1623;
                v2246 = v1624;
                v2248 = v1819;
                v2360 = v2362;
            }
            let v2247 = v2245 - v2246;
            let v2256 = if v280 > v0 { 1.0 } else { 0.0 };
            let v2782: f64;
            if v2256 != 0.0 {
                let v2259 = v8 / (v2257 * v1232);
                let v2260 = if v2259 > v1235 { 1.0 } else { 0.0 };
                let v2264: f64;
                let v2265: f64;
                if v2260 != 0.0 {
                    let v2262 = v43 + (v2259 - v1235);
                    v2264 = v2262;
                    v2265 = v1235;
                } else {
                    v2264 = v43;
                    v2265 = v2259;
                }
                let v2269 = v2263 * ((v2264 * (rspice_limexp(v2265))) - v43);
                v2782 = v2269;
            } else {
                v2782 = v0;
            }
            let v2816: f64;
            if v397 != 0.0 {
                let v2272 = if (if v1318 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1321 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v2817: f64;
                if v2272 != 0.0 {
                    let v2278 = (((v43 / v268) - v43) * ((v2253 / v1318).ln())).exp();
                    let v2296 = (((-v2279) * v8) / (v1321 * v2278)) * (((-v2288) * v2278).exp());
                    v2817 = v2296;
                } else {
                    v2817 = v0;
                }
                v2816 = v2817;
            } else {
                v2816 = v0;
            }
            let v2778: f64;
            if v384 != 0.0 {
                let v2297 = v1321 - v8;
                let v2298 = if v2297 > v0 { 1.0 } else { 0.0 };
                let v2779: f64;
                if v2298 != 0.0 {
                    let v2300 = if v2299 > v0 { 1.0 } else { 0.0 };
                    let v2329: f64;
                    if v2300 != 0.0 {
                        let v2318 = (v1352 * ((((((v2253 / v1318) / v1352).exp()) - v174) + (v174 * (((v43 - (v2245 / (((v2299 * v1576) * v1578) + (v2304 * v2245)))) / v1352).cosh()))).ln())).sqrt();
                        v2329 = v2318;
                    } else {
                        v2329 = v43;
                    }
                    let v2322 = v2319 / v2253;
                    let v2323 = v2319 / v1318;
                    let v2324 = if v2297 > v2323 { 1.0 } else { 0.0 };
                    let v2348: f64;
                    if v2324 != 0.0 {
                        let v2339 = (v2325 * (((-v2322) / (v2323 * v2329)).exp())) * (v2323 + ((v43 + (v2322 / v2323)) * (v2297 - v2323)));
                        v2348 = v2339;
                    } else {
                        let v2345 = (v2325 * v2297) * (((-v2322) / (v2297 * v2329)).exp());
                        v2348 = v2345;
                    }
                    let v2347 = if v2346 > v0 { 1.0 } else { 0.0 };
                    let v2780: f64;
                    if v2347 != 0.0 {
                        let v2350 = v43 - (v2346 * v2348);
                        let v2358 = (v2245 * v2348) / (v63 * (v2350 + (((v2350 * v2350) + v2352).sqrt())));
                        v2780 = v2358;
                    } else {
                        let v2359 = v2245 * v2348;
                        v2780 = v2359;
                    }
                    v2779 = v2780;
                } else {
                    v2779 = v0;
                }
                v2778 = v2779;
            } else {
                v2778 = v0;
            }
            let v2364 = v2360 * v2363;
            let v2366 = if v2365 > v0 { 1.0 } else { 0.0 };
            let v2786: f64;
            if v2366 != 0.0 {
                let v2373 = v43 + (((v1494 + v1498) + v2248) / ((v43 + v2367) * v1491));
                let v2379 = v2365 / (v63 * (v2373 + (((v2373 * v2373) + v118).sqrt())));
                let v2381 = if v2380 > v0 { 1.0 } else { 0.0 };
                let v2395: f64;
                if v2381 != 0.0 {
                    let v2385 = ((v2379 * v2380) * v2383) * v1260;
                    let v2386 = if v2385 < v1630 { 1.0 } else { 0.0 };
                    let v2396: f64;
                    if v2386 != 0.0 {
                        let v2389 = v2379 * (v43 - (v63 * v2385));
                        v2396 = v2389;
                    } else {
                        let v2393 = (v2379 * ((v43 + v2385).ln())) / v2385;
                        v2396 = v2393;
                    }
                    v2395 = v2396;
                } else {
                    v2395 = v2379;
                }
                let v2394 = if v2248 > v0 { 1.0 } else { 0.0 };
                let v2787: f64;
                if v2394 != 0.0 {
                    let v2402 = (v2395 * (v1494 + (v2248 * v2397))) / (v1494 + v2248);
                    v2787 = v2402;
                } else {
                    v2787 = v2395;
                }
                v2786 = v2787;
            } else {
                v2786 = v0;
            }
            let v2403 = if v462 > v0 { 1.0 } else { 0.0 };
            let v2783: f64;
            if v2403 != 0.0 {
                let v2406 = v12 / (v2404 * v1232);
                let v2407 = if v2406 > v1235 { 1.0 } else { 0.0 };
                let v2411: f64;
                let v2412: f64;
                if v2407 != 0.0 {
                    let v2409 = v43 + (v2406 - v1235);
                    v2411 = v2409;
                    v2412 = v1235;
                } else {
                    v2411 = v43;
                    v2412 = v2406;
                }
                let v2416 = v2410 * ((v2411 * (rspice_limexp(v2412))) - v43);
                v2783 = v2416;
            } else {
                v2783 = v0;
            }
            let v2417 = if v464 > v0 { 1.0 } else { 0.0 };
            let v2818: f64;
            if v2417 != 0.0 {
                let v2419 = v12 / (v465 * v1232);
                let v2420 = if v2419 > v1235 { 1.0 } else { 0.0 };
                let v2424: f64;
                let v2425: f64;
                if v2420 != 0.0 {
                    let v2422 = v43 + (v2419 - v1235);
                    v2424 = v2422;
                    v2425 = v1235;
                } else {
                    v2424 = v43;
                    v2425 = v2419;
                }
                let v2429 = v2423 * ((v2424 * (rspice_limexp(v2425))) - v43);
                v2818 = v2429;
            } else {
                v2818 = v0;
            }
            let v2431 = if v2430 > v0 { 1.0 } else { 0.0 };
            let v2471: f64;
            if v2431 != 0.0 {
                let v2443 = v2432 * (v43 - (((-(v2433.ln())) / v451).exp()));
                let v2445 = (v2443 - v12) * v1260;
                let v2448 = ((v2445 * v2445) + v1286).sqrt();
                let v2450 = (v2445 + v2448) * v63;
                let v2453 = v2450 / v2448;
                let v2464 = v2430 * (((((-v451) * ((v43 - ((v2443 - (v1232 * v2450)) / v2432)).ln())).exp()) * v2453) + (v2433 * (v43 - v2453)));
                v2471 = v2464;
            } else {
                v2471 = v0;
            }
            let v2813: f64;
            if v478 != 0.0 {
                let v2468 = if (if (if v480 == v43 { 1.0 } else { 0.0 }) != 0.0 && v2431 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v2432 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v2814: f64;
                if v2468 != 0.0 {
                    let v2475 = ((v43 - (v43 / v451)) * ((v2471 / v2430).ln())).exp();
                    let v2489 = (((-(v12 / v2432)) * v2478) * v2475) * (((-v2483) / v2475).exp());
                    v2814 = v2489;
                } else {
                    let v2493 = if (if (if v480 == v0 { 1.0 } else { 0.0 }) != 0.0 && v1270 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v1271 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v2815: f64;
                    if v2493 != 0.0 {
                        let v2499 = ((v43 - (v43 / v207)) * ((v2252 / v1269).ln())).exp();
                        let v2507 = (((-(v5 / v1271)) * v2478) * v2499) * (((-v2483) / v2499).exp());
                        v2815 = v2507;
                    } else {
                        v2815 = v0;
                    }
                    v2814 = v2815;
                }
                v2813 = v2814;
            } else {
                v2813 = v0;
            }
            let v2512 = v2508 * (((v5 / v523).exp()) - v43);
            let v2514 = if v2513 < v1316 { 1.0 } else { 0.0 };
            if v2514 != 0.0 {
                let v2516 = if v2515 > v0 { 1.0 } else { 0.0 };
                if v2516 != 0.0 {
                    let v2520 = v2513 - v2517;
                    let v2533 = v2517 * (v43 - (((-(v2521.ln())) / v559).exp()));
                    let v2535 = (v2533 - v14) * v1260;
                    let v2536 = if v2535 < v1235 { 1.0 } else { 0.0 };
                    let v2545: f64;
                    if v2536 != 0.0 {
                        let v2541 = v2533 - (v1232 * ((v43 + (v2535.exp())).ln()));
                        v2545 = v2541;
                    } else {
                        v2545 = v14;
                    }
                    let v2548 = if ((v2520 + v2545) / ((v1352 * v2520) + (v195 * v1232))) < v1235 { 1.0 } else { 0.0 };
                    if v2548 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v2549 = if v2515 > v0 { 1.0 } else { 0.0 };
                if v2549 != 0.0 {
                } else {
                }
            }
            let v2550 = if v574 > v0 { 1.0 } else { 0.0 };
            let v2784: f64;
            if v2550 != 0.0 {
                let v2553 = v14 / (v2551 * v1232);
                let v2554 = if v2553 > v1235 { 1.0 } else { 0.0 };
                let v2558: f64;
                let v2559: f64;
                if v2554 != 0.0 {
                    let v2556 = v43 + (v2553 - v1235);
                    v2558 = v2556;
                    v2559 = v1235;
                } else {
                    v2558 = v43;
                    v2559 = v2553;
                }
                let v2563 = v2557 * ((v2558 * (rspice_limexp(v2559))) - v43);
                v2784 = v2563;
            } else {
                v2784 = v0;
            }
            let v2819: f64;
            if v2514 != 0.0 {
                let v2565 = if v2564 > v0 { 1.0 } else { 0.0 };
                let v2820: f64;
                if v2565 != 0.0 {
                    let v2566 = v559 / v195;
                    let v2567 = v2513 - v2517;
                    let v2573 = v2517 * (v43 - (((-(v2521.ln())) / v559).exp()));
                    let v2574 = v2521 * v2564;
                    let v2580 = v2564 * (((v2566 - v559) * ((v2513 / v2517).ln())).exp());
                    let v2582 = (v2573 - v17) * v1260;
                    let v2583 = if v2582 < v1235 { 1.0 } else { 0.0 };
                    let v2592: f64;
                    if v2583 != 0.0 {
                        let v2588 = v2573 - (v1232 * ((v43 + (v2582.exp())).ln()));
                        v2592 = v2588;
                    } else {
                        v2592 = v17;
                    }
                    let v2591 = (v1352 * v2567) + (v195 * v1232);
                    let v2594 = (v2567 + v2592) / v2591;
                    let v2595 = if v2594 < v1235 { 1.0 } else { 0.0 };
                    let v2611: f64;
                    if v2595 != 0.0 {
                        let v2606 = (-v2567) + (v2591 * (((v43 + (v2594.exp())).ln()) - (((-(v2567 + v2573)) / v2591).exp())));
                        v2611 = v2606;
                    } else {
                        v2611 = v2592;
                    }
                    let v2614 = (v43 - (v2611 / v2517)).ln();
                    let v2615 = v43 - v559;
                    let v2616 = v43 - v2566;
                    let v2636 = (((((v2564 * (v43 - ((v2614 * v2615).exp()))) / v2615) + ((v2580 * (v43 - ((((v43 - (v2592 / v2517)).ln()) * v2616).exp()))) / v2616)) - ((v2580 * (v43 - ((v2614 * v2616).exp()))) / v2616)) * v2517) + (v2574 * (v17 - v2592));
                    v2820 = v2636;
                } else {
                    v2820 = v0;
                }
                v2819 = v2820;
            } else {
                let v2637 = if v2564 > v0 { 1.0 } else { 0.0 };
                let v2821: f64;
                if v2637 != 0.0 {
                    let v2643 = v2517 * (v43 - (((-(v2521.ln())) / v559).exp()));
                    let v2645 = (v2643 - v17) * v1260;
                    let v2652 = v2643 - (v1232 * ((v2645 + (((v2645 * v2645) + v1286).sqrt())) * v63));
                    let v2656 = v43 - v559;
                    let v2665 = v2564 * (((v2517 * (v43 - ((((v43 - (v2652 / v2517)).ln()) * v2656).exp()))) / v2656) + (v2521 * (v17 - v2652)));
                    v2821 = v2665;
                } else {
                    v2821 = v0;
                }
                v2819 = v2821;
            }
            let v2667 = if v2666 < v1316 { 1.0 } else { 0.0 };
            if v2667 != 0.0 {
                let v2675 = if v2668 > v0 { 1.0 } else { 0.0 };
                if v2675 != 0.0 {
                    let v2683 = v2666 - v2676;
                    let v2700 = v2676 * (v43 - (((-(v2684.ln())) / v612).exp()));
                    let v2702 = (v2700 - v20) * v1260;
                    let v2703 = if v2702 < v1235 { 1.0 } else { 0.0 };
                    let v2712: f64;
                    if v2703 != 0.0 {
                        let v2708 = v2700 - (v1232 * ((v43 + (v2702.exp())).ln()));
                        v2712 = v2708;
                    } else {
                        v2712 = v20;
                    }
                    let v2715 = if ((v2683 + v2712) / ((v1352 * v2683) + (v195 * v1232))) < v1235 { 1.0 } else { 0.0 };
                    if v2715 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v2716 = if v2668 > v0 { 1.0 } else { 0.0 };
                if v2716 != 0.0 {
                } else {
                }
            }
            if v677 != 0.0 {
                let v2718 = if v2717 < v1316 { 1.0 } else { 0.0 };
                if v2718 != 0.0 {
                    let v2724 = if v2719 > v0 { 1.0 } else { 0.0 };
                    if v2724 != 0.0 {
                        let v2730 = v2717 - v2725;
                        let v2743 = v2725 * (v43 - (((-(v2731.ln())) / v710).exp()));
                        let v2745 = (v2743 - v24) * v1260;
                        let v2746 = if v2745 < v1235 { 1.0 } else { 0.0 };
                        let v2755: f64;
                        if v2746 != 0.0 {
                            let v2751 = v2743 - (v1232 * ((v43 + (v2745.exp())).ln()));
                            v2755 = v2751;
                        } else {
                            v2755 = v24;
                        }
                        let v2758 = if ((v2730 + v2755) / ((v1352 * v2730) + (v195 * v1232))) < v1235 { 1.0 } else { 0.0 };
                        if v2758 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                    let v2759 = if v2719 > v0 { 1.0 } else { 0.0 };
                    if v2759 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v2760 = if v674 > v0 { 1.0 } else { 0.0 };
            if v2760 != 0.0 {
                let v2761 = if v675 > v0 { 1.0 } else { 0.0 };
                if v2761 != 0.0 {
                } else {
                }
            } else {
            }
            let v2762 = if v667 > v0 { 1.0 } else { 0.0 };
            let v2785: f64;
            if v2762 != 0.0 {
                let v2765 = v20 / (v2763 * v1232);
                let v2766 = if v2765 > v1235 { 1.0 } else { 0.0 };
                let v2770: f64;
                let v2771: f64;
                if v2766 != 0.0 {
                    let v2768 = v43 + (v2765 - v1235);
                    v2770 = v2768;
                    v2771 = v1235;
                } else {
                    v2770 = v43;
                    v2771 = v2765;
                }
                let v2775 = v2769 * ((v2770 * (rspice_limexp(v2771))) - v43);
                v2785 = v2775;
            } else {
                v2785 = v0;
            }
            let v2776 = if v742 != 0.0 && v744 != 0.0 { 1.0 } else { 0.0 };
            if v2776 != 0.0 {
                let v2777 = if v739 == v43 { 1.0 } else { 0.0 };
                if v2777 != 0.0 {
                } else {
                    let v2781 = if v739 == v174 { 1.0 } else { 0.0 };
                    if v2781 != 0.0 {
                        let v2790 = if (if v2786 >= v741 { 1.0 } else { 0.0 }) != 0.0 && (if v2786 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if v2790 != 0.0 {
                        } else {
                        }
                        let v2794 = if (if v2791 >= v741 { 1.0 } else { 0.0 }) != 0.0 && (if v2791 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if v2794 != 0.0 {
                        } else {
                        }
                        let v2799 = if (if v2796 >= v741 { 1.0 } else { 0.0 }) != 0.0 && (if v2796 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if v2799 != 0.0 {
                        } else {
                        }
                        let v2803 = if (if v2800 >= v741 { 1.0 } else { 0.0 }) != 0.0 && (if v2800 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if v2803 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                }
            } else {
            }
            let v2806 = if v2804 != v0 { 1.0 } else { 0.0 };
            if v2806 != 0.0 {
            } else {
            }
            let v2810 = if (if v413 >= v741 { 1.0 } else { 0.0 }) != 0.0 && (if v413 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v2810 != 0.0 {
                let v2811 = if v2251 > v0 { 1.0 } else { 0.0 };
                if v2811 != 0.0 {
                } else {
                }
            } else {
            }
            let v2812 = if v480 == v43 { 1.0 } else { 0.0 };
            if v2812 != 0.0 {
            } else {
            }
            let v2822 = v1 * v2819;
            let v2823 = 0e0f64;
            let v2825 = v2824 * v16;
            let v2826 = 0e0f64;
            let v2829 = if (if v728 >= v741 { 1.0 } else { 0.0 }) != 0.0 && (if v728 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v2913: f64;
            if v2829 != 0.0 {
                let v2831 = (v15 - v10) / v2800;
                v2913 = v2831;
            } else {
                v2913 = v0;
            }
            let v2834 = if (if v733 >= v741 { 1.0 } else { 0.0 }) != 0.0 && (if v733 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v2834 != 0.0 {
            } else {
            }
            let v2837 = if (if v723 >= v741 { 1.0 } else { 0.0 }) != 0.0 && (if v723 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v2837 != 0.0 {
            } else {
            }
            let v2839 = v96 * (v15 - v2795);
            let v2840 = 0e0f64;
            if v1634 != 0.0 {
                if v2762 != 0.0 {
                } else {
                }
            } else {
                if v1626 != 0.0 {
                } else {
                }
            }
            let v2844 = if (if v2841 >= v741 { 1.0 } else { 0.0 }) != 0.0 && (if v2841 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v2844 != 0.0 {
                let v2846 = if v2845 > v0 { 1.0 } else { 0.0 };
                if v2846 != 0.0 {
                } else {
                }
            } else {
            }
            let v2849 = if (if (if v739 >= v43 { 1.0 } else { 0.0 }) != 0.0 && v742 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v744 != 0.0 { 1.0 } else { 0.0 };
            if v2849 != 0.0 {
                let v2851 = if v2850 > v0 { 1.0 } else { 0.0 };
                if v2851 != 0.0 {
                } else {
                }
            } else {
            }
            let v2854 = (v195 * v37) * v2853;
            let v2914: f64;
            let v2967: f64;
            let v2968: f64;
            if v2829 != 0.0 {
                let v2856 = v2854 / v2800;
                v2914 = v2855;
                v2967 = v43;
                v2968 = v2856;
            } else {
                v2914 = v0;
                v2967 = v0;
                v2968 = v0;
            }
            let v2969: f64;
            let v2970: f64;
            if v2810 != 0.0 {
                let v2857 = v2854 / v2786;
                v2969 = v43;
                v2970 = v2857;
            } else {
                v2969 = v0;
                v2970 = v0;
            }
            let v2971: f64;
            let v2972: f64;
            if v2837 != 0.0 {
                let v2858 = v2854 / v2796;
                v2971 = v43;
                v2972 = v2858;
            } else {
                v2971 = v0;
                v2972 = v0;
            }
            let v2973: f64;
            let v2974: f64;
            if v2834 != 0.0 {
                let v2859 = v2854 / v2791;
                v2973 = v43;
                v2974 = v2859;
            } else {
                v2973 = v0;
                v2974 = v0;
            }
            let v2975: f64;
            let v2976: f64;
            if v2844 != 0.0 {
                let v2860 = v2854 / v2841;
                v2975 = v43;
                v2976 = v2860;
            } else {
                v2975 = v0;
                v2976 = v0;
            }
            let v2866 = v2861 * (((v2380 + v2783).abs()).powf(v2864));
            let v2869 = if v2867 == v2868 { 1.0 } else { 0.0 };
            let v2977: f64;
            let v2978: f64;
            let v2979: f64;
            let v2980: f64;
            let v2981: f64;
            let v2982: f64;
            if v2869 != 0.0 {
                v2977 = v43;
                v2978 = v2866;
                v2979 = v43;
                v2980 = v0;
                v2981 = v0;
                v2982 = v0;
            } else {
                v2977 = v0;
                v2978 = v0;
                v2979 = v0;
                v2980 = v43;
                v2981 = v2866;
                v2982 = v43;
            }
            let v2983: f64;
            let v2984: f64;
            let v2985: f64;
            if v2834 != 0.0 {
                let v2876 = v2872 * ((((v3 - v2795) / v2791).abs()).powf(v2874));
                v2983 = v43;
                v2984 = v2876;
                v2985 = v43;
            } else {
                v2983 = v0;
                v2984 = v0;
                v2985 = v0;
            }
            let v2877 = v174 * v38;
            let v2986: f64;
            let v2987: f64;
            if v1634 != 0.0 {
                let v2879 = v2877 * (v2813.abs());
                v2986 = v43;
                v2987 = v2879;
            } else {
                v2986 = v0;
                v2987 = v0;
            }
            let v2881 = v2877 * (v2783.abs());
            let v2882 = v2877 * v2778;
            let v2884 = v2877 * (v2782.abs());
            let v2886 = v2877 * (v2816.abs());
            let v2888 = v2877 * (v2784.abs());
            let v2890 = v2877 * (v2785.abs());
            let v2896 = if (if v2891 == v43 { 1.0 } else { 0.0 }) != 0.0 && (if (if v109 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v111 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v2988: f64;
            let v2989: f64;
            let v2990: f64;
            let v2991: f64;
            let v2992: f64;
            let v2993: f64;
            let v2994: f64;
            let v2995: f64;
            if v2896 != 0.0 {
                let v2897 = if v2380 > v0 { 1.0 } else { 0.0 };
                let v2899: f64;
                if v2897 != 0.0 {
                    let v2898 = v2247 / v2380;
                    v2899 = v2898;
                } else {
                    v2899 = v128;
                }
                let v2904 = if (v2899 * ((v174 * v111) - (v109 * v109))) > v0 { 1.0 } else { 0.0 };
                if v2904 != 0.0 {
                } else {
                }
                let v2906 = v2877 * (v2380.abs());
                let v2908 = v2877 * (v2247.abs());
                v2988 = v43;
                v2989 = v2906;
                v2990 = v43;
                v2991 = v2908;
                v2992 = v0;
                v2993 = v0;
                v2994 = v0;
                v2995 = v0;
            } else {
                let v2910 = v2877 * (v2247.abs());
                let v2912 = v2877 * (v2380.abs());
                v2988 = v0;
                v2989 = v0;
                v2990 = v0;
                v2991 = v0;
                v2992 = v43;
                v2993 = v2910;
                v2994 = v43;
                v2995 = v2912;
            }
            let v2921 = if (((((v2823 + v2826) + v2913) + v2840) + v2914) + v2915) != v0 { 1.0 } else { 0.0 };
            if v2921 != 0.0 {
            } else {
            }
            let v2941 = (((((-(0e0f64)) - (0e0f64)) - ((-(0e0f64)) - (0e0f64))) + (-(0e0f64))) - (-(0e0f64))) + ((-(0e0f64)) - (0e0f64));
            let v2943 = if (v2941.abs()) > v32 { 1.0 } else { 0.0 };
            if v2943 != 0.0 {
            } else {
                let v2944 = if v2941 >= v0 { 1.0 } else { 0.0 };
                if v2944 != 0.0 {
                } else {
                }
            }
            let v2950 = -(0e0f64);
            let v2958 = ((((-(0e0f64)) - (-(0e0f64))) - v2950) + (-(0e0f64))) + (-(0e0f64));
            let v2960 = if (v2958.abs()) > v32 { 1.0 } else { 0.0 };
            if v2960 != 0.0 {
            } else {
                let v2961 = if v2958 >= v0 { 1.0 } else { 0.0 };
                if v2961 != 0.0 {
                } else {
                }
            }
            let v2963 = (0e0f64) - v2950;
            let v2965 = if (v2963.abs()) > v32 { 1.0 } else { 0.0 };
            if v2965 != 0.0 {
            } else {
                let v2966 = if v2963 >= v0 { 1.0 } else { 0.0 };
                if v2966 != 0.0 {
                } else {
                }
            }
        if v2967 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2968;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2969 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2970;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2971 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2972;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2973 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2974;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2975 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2976;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2977 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2978;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v2979);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2980 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2981;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v2982);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2983 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2984;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v2985);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2986 == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2987;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2881;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2882;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2884;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2886;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2888;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v2890;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2988 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2989;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2990 == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2991;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2992 == 0.0 {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2993;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v2994 == 0.0 {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v2995;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

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
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_SI_G_S_SHOT_INT", label: Some("g-s shot int"), kind: GeneratedNoiseKind::White, equation: 178, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_DI_G_D_SHOT_INT", label: Some("g-d shot int"), kind: GeneratedNoiseKind::White, equation: 179, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_FPS4_G_S_SHOT_EXT", label: Some("g-s shot ext"), kind: GeneratedNoiseKind::White, equation: 180, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(13), name: "fps4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI2P_FP4_G_D_SHOT_EXT", label: Some("g-d shot ext"), kind: GeneratedNoiseKind::White, equation: 181, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi2p", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(17), name: "fp4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 182, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_CHANNEL", label: Some("channel"), kind: GeneratedNoiseKind::White, equation: 183, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI_FPS1_RFPS1", label: Some("rfps1"), kind: GeneratedNoiseKind::White, equation: 184, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "si", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "fps1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FPS1_FPS2_RFPS2", label: Some("rfps2"), kind: GeneratedNoiseKind::White, equation: 185, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(10), name: "fps1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "fps2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FPS2_FPS3_RFPS3", label: Some("rfps3"), kind: GeneratedNoiseKind::White, equation: 186, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "fps2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(12), name: "fps3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FPS3_FPS4_RFPS4", label: Some("rfps4"), kind: GeneratedNoiseKind::White, equation: 187, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "fps3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(13), name: "fps4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP1_DI_RFP1", label: Some("rfp1"), kind: GeneratedNoiseKind::White, equation: 188, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "fp1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP2_FP1_RFP2", label: Some("rfp2"), kind: GeneratedNoiseKind::White, equation: 189, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(15), name: "fp2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(14), name: "fp1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP3_FP2_RFP3", label: Some("rfp3"), kind: GeneratedNoiseKind::White, equation: 190, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(16), name: "fp3", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(15), name: "fp2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FP4_FP3_RFP4", label: Some("rfp4"), kind: GeneratedNoiseKind::White, equation: 191, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(17), name: "fp4", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(16), name: "fp3", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SRC_S_RCS", label: Some("rcs"), kind: GeneratedNoiseKind::White, equation: 192, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(19), name: "src", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DRC_RCD", label: Some("rcd"), kind: GeneratedNoiseKind::White, equation: 193, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(18), name: "drc", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16]), ctx.node_voltage(self.nodes[17]), ctx.node_voltage(self.nodes[18]), ctx.node_voltage(self.nodes[19]), ctx.node_voltage(self.nodes[20]), ctx.node_voltage(self.nodes[21]), ctx.node_voltage(self.nodes[22]), ctx.node_voltage(self.nodes[23]), ctx.node_voltage(self.nodes[24]), ctx.node_voltage(self.nodes[25]), ctx.node_voltage(self.nodes[26]), ctx.node_voltage(self.nodes[27]), ctx.node_voltage(self.nodes[28]), ctx.node_voltage(self.nodes[29])];
            let v0 = 0e0f64;
            let v1 = parameters[5];
            let v2 = 2.7315e2f64;
            let v4 = temperature;
            let v5 = 1e0f64;
            let v6 = 0.0f64;
            let v7 = node_potentials[4];
            let v8 = parameters[3];
            let v11 = 3.1499999999999773e0f64;
            let v13 = 3.1499999999999773e0f64;
            let v14 = 1.77315e3f64;
            let v16 = 1.77315e3f64;
            let v17 = parameters[50];
            let v19 = parameters[30];
            let v20 = parameters[0];
            let v22 = parameters[2];
            let v24 = parameters[31];
            let v28 = parameters[29];
            let v29 = parameters[54];
            let v35 = parameters[66];
            let v41 = parameters[353];
            let v45 = parameters[48];
            let v51 = parameters[49];
            let v56 = 1e-1f64;
            let v72 = parameters[324];
            let v74 = parameters[325];
            let v76 = parameters[326];
            let v77 = parameters[327];
            let v86 = 1.38062e-23f64;
            let v88 = 1.60219e-19f64;
            let v90 = parameters[336];
            let v96 = 3e0f64;
            let v98 = parameters[21];
            let v101 = 1e-2f64;
            let v103 = parameters[22];
            let v107 = parameters[23];
            let v111 = parameters[24];
            let v115 = parameters[25];
            let v119 = parameters[26];
            let v123 = parameters[7];
            let v124 = parameters[8];
            let v130 = parameters[81];
            let v131 = parameters[82];
            let v137 = parameters[103];
            let v138 = parameters[104];
            let v144 = parameters[125];
            let v145 = parameters[126];
            let v151 = parameters[147];
            let v152 = parameters[148];
            let v158 = parameters[87];
            let v162 = parameters[109];
            let v166 = parameters[131];
            let v170 = parameters[153];
            let v174 = parameters[89];
            let v178 = parameters[111];
            let v182 = parameters[133];
            let v186 = parameters[155];
            let v190 = parameters[169];
            let v191 = parameters[170];
            let v197 = parameters[191];
            let v198 = parameters[192];
            let v204 = parameters[213];
            let v205 = parameters[214];
            let v211 = parameters[235];
            let v212 = parameters[236];
            let v218 = parameters[175];
            let v222 = parameters[197];
            let v226 = parameters[219];
            let v230 = parameters[241];
            let v234 = parameters[177];
            let v238 = parameters[199];
            let v242 = parameters[221];
            let v246 = parameters[243];
            let v250 = parameters[6];
            let v251 = node_potentials[5];
            let v252 = node_potentials[9];
            let v255 = node_potentials[8];
            let v258 = parameters[52];
            let v260 = node_potentials[19];
            let v261 = node_potentials[0];
            let v264 = node_potentials[2];
            let v272 = parameters[53];
            let v273 = 5e-1f64;
            let v283 = 1e-3f64;
            let v291 = parameters[55];
            let v292 = parameters[56];
            let v294 = parameters[33];
            let v298 = node_potentials[13];
            let v304 = parameters[328];
            let v306 = node_potentials[1];
            let v308 = parameters[331];
            let v310 = node_potentials[21];
            let v311 = parameters[335];
            let v314 = parameters[334];
            let v316 = 5e1f64;
            let v318 = -5e1f64;
            let v320 = node_potentials[20];
            let v324 = 2e0f64;
            let v326 = node_potentials[22];
            let v327 = node_potentials[23];
            let v328 = node_potentials[24];
            let v332 = -5e1f64;
            let v336 = parameters[338];
            let v338 = node_potentials[25];
            let v339 = node_potentials[26];
            let v340 = node_potentials[27];
            let v344 = -5e1f64;
            let v348 = parameters[337];
            let v353 = node_potentials[17];
            let v379 = parameters[67];
            let v382 = parameters[68];
            let v387 = node_potentials[18];
            let v393 = parameters[78];
            let v395 = node_potentials[7];
            let v396 = node_potentials[10];
            let v407 = node_potentials[3];
            let v410 = parameters[100];
            let v412 = node_potentials[11];
            let v425 = parameters[122];
            let v427 = node_potentials[12];
            let v440 = parameters[144];
            let v454 = parameters[166];
            let v464 = node_potentials[14];
            let v469 = parameters[188];
            let v479 = node_potentials[15];
            let v484 = parameters[210];
            let v494 = node_potentials[16];
            let v499 = parameters[232];
            let v513 = parameters[233];
            let v514 = parameters[354];
            let v517 = parameters[239];
            let v519 = parameters[237];
            let v520 = parameters[234];
            let v521 = parameters[248];
            let v522 = parameters[247];
            let v523 = parameters[249];
            let v524 = parameters[253];
            let v525 = parameters[244];
            let v526 = parameters[245];
            let v527 = parameters[246];
            let v528 = parameters[252];
            let v529 = parameters[251];
            let v530 = parameters[250];
            let v531 = parameters[39];
            let v532 = parameters[47];
            let v533 = parameters[45];
            let v534 = parameters[42];
            let v545 = 2.302585092994046e0f64;
            let v568 = parameters[51];
            let v591 = -5e1f64;
            let v622 = -5e1f64;
            let v709 = -5e1f64;
            let v722 = -5e1f64;
            let v727 = -5e1f64;
            let v740 = -5e1f64;
            let v765 = -5e1f64;
            let v794 = -5e1f64;
            let v861 = -5e1f64;
            let v874 = -5e1f64;
            let v879 = -5e1f64;
            let v892 = -5e1f64;
            let v894 = 1e-38f64;
            let v895 = 1e-57f64;
            let v896 = 2e-19f64;
            let v897 = 4e0f64;
            let v898 = 6e0f64;
            let v899 = 1.5e1f64;
            let v907 = -5e1f64;
            let v912 = -5e1f64;
            let v921 = -5e1f64;
            let v923 = parameters[211];
            let v926 = parameters[217];
            let v928 = parameters[215];
            let v929 = parameters[212];
            let v930 = parameters[226];
            let v931 = parameters[225];
            let v932 = parameters[227];
            let v933 = parameters[231];
            let v934 = parameters[222];
            let v935 = parameters[223];
            let v936 = parameters[224];
            let v937 = parameters[230];
            let v938 = parameters[229];
            let v939 = parameters[228];
            let v994 = -5e1f64;
            let v1025 = -5e1f64;
            let v1112 = -5e1f64;
            let v1125 = -5e1f64;
            let v1130 = -5e1f64;
            let v1143 = -5e1f64;
            let v1168 = -5e1f64;
            let v1197 = -5e1f64;
            let v1264 = -5e1f64;
            let v1277 = -5e1f64;
            let v1282 = -5e1f64;
            let v1295 = -5e1f64;
            let v1304 = -5e1f64;
            let v1309 = -5e1f64;
            let v1318 = -5e1f64;
            let v1320 = parameters[189];
            let v1323 = parameters[195];
            let v1325 = parameters[193];
            let v1326 = parameters[190];
            let v1327 = parameters[204];
            let v1328 = parameters[203];
            let v1329 = parameters[205];
            let v1330 = parameters[209];
            let v1331 = parameters[200];
            let v1332 = parameters[201];
            let v1333 = parameters[202];
            let v1334 = parameters[208];
            let v1335 = parameters[207];
            let v1336 = parameters[206];
            let v1391 = -5e1f64;
            let v1422 = -5e1f64;
            let v1509 = -5e1f64;
            let v1522 = -5e1f64;
            let v1527 = -5e1f64;
            let v1540 = -5e1f64;
            let v1565 = -5e1f64;
            let v1594 = -5e1f64;
            let v1661 = -5e1f64;
            let v1674 = -5e1f64;
            let v1679 = -5e1f64;
            let v1692 = -5e1f64;
            let v1701 = -5e1f64;
            let v1706 = -5e1f64;
            let v1715 = -5e1f64;
            let v1717 = parameters[167];
            let v1720 = parameters[173];
            let v1722 = parameters[171];
            let v1723 = parameters[168];
            let v1724 = parameters[182];
            let v1725 = parameters[181];
            let v1726 = parameters[183];
            let v1727 = parameters[187];
            let v1728 = parameters[178];
            let v1729 = parameters[179];
            let v1730 = parameters[180];
            let v1731 = parameters[186];
            let v1732 = parameters[185];
            let v1733 = parameters[184];
            let v1788 = -5e1f64;
            let v1819 = -5e1f64;
            let v1906 = -5e1f64;
            let v1919 = -5e1f64;
            let v1924 = -5e1f64;
            let v1937 = -5e1f64;
            let v1962 = -5e1f64;
            let v1991 = -5e1f64;
            let v2058 = -5e1f64;
            let v2071 = -5e1f64;
            let v2076 = -5e1f64;
            let v2089 = -5e1f64;
            let v2098 = -5e1f64;
            let v2103 = -5e1f64;
            let v2112 = -5e1f64;
            let v2114 = parameters[79];
            let v2117 = parameters[85];
            let v2119 = parameters[83];
            let v2120 = parameters[80];
            let v2121 = parameters[94];
            let v2122 = parameters[93];
            let v2123 = parameters[95];
            let v2124 = parameters[99];
            let v2125 = parameters[90];
            let v2126 = parameters[91];
            let v2127 = parameters[92];
            let v2128 = parameters[98];
            let v2129 = parameters[97];
            let v2130 = parameters[96];
            let v2185 = -5e1f64;
            let v2216 = -5e1f64;
            let v2303 = -5e1f64;
            let v2316 = -5e1f64;
            let v2321 = -5e1f64;
            let v2334 = -5e1f64;
            let v2359 = -5e1f64;
            let v2388 = -5e1f64;
            let v2455 = -5e1f64;
            let v2468 = -5e1f64;
            let v2473 = -5e1f64;
            let v2486 = -5e1f64;
            let v2495 = -5e1f64;
            let v2500 = -5e1f64;
            let v2509 = -5e1f64;
            let v2511 = parameters[101];
            let v2514 = parameters[107];
            let v2516 = parameters[105];
            let v2517 = parameters[102];
            let v2518 = parameters[116];
            let v2519 = parameters[115];
            let v2520 = parameters[117];
            let v2521 = parameters[121];
            let v2522 = parameters[112];
            let v2523 = parameters[113];
            let v2524 = parameters[114];
            let v2525 = parameters[120];
            let v2526 = parameters[119];
            let v2527 = parameters[118];
            let v2582 = -5e1f64;
            let v2613 = -5e1f64;
            let v2700 = -5e1f64;
            let v2713 = -5e1f64;
            let v2718 = -5e1f64;
            let v2731 = -5e1f64;
            let v2756 = -5e1f64;
            let v2785 = -5e1f64;
            let v2852 = -5e1f64;
            let v2865 = -5e1f64;
            let v2870 = -5e1f64;
            let v2883 = -5e1f64;
            let v2892 = -5e1f64;
            let v2897 = -5e1f64;
            let v2906 = -5e1f64;
            let v2908 = parameters[123];
            let v2911 = parameters[129];
            let v2913 = parameters[127];
            let v2914 = parameters[124];
            let v2915 = parameters[138];
            let v2916 = parameters[137];
            let v2917 = parameters[139];
            let v2918 = parameters[143];
            let v2919 = parameters[134];
            let v2920 = parameters[135];
            let v2921 = parameters[136];
            let v2922 = parameters[142];
            let v2923 = parameters[141];
            let v2924 = parameters[140];
            let v2979 = -5e1f64;
            let v3010 = -5e1f64;
            let v3097 = -5e1f64;
            let v3110 = -5e1f64;
            let v3115 = -5e1f64;
            let v3128 = -5e1f64;
            let v3153 = -5e1f64;
            let v3182 = -5e1f64;
            let v3249 = -5e1f64;
            let v3262 = -5e1f64;
            let v3267 = -5e1f64;
            let v3280 = -5e1f64;
            let v3289 = -5e1f64;
            let v3294 = -5e1f64;
            let v3303 = -5e1f64;
            let v3305 = parameters[145];
            let v3308 = parameters[151];
            let v3310 = parameters[149];
            let v3311 = parameters[146];
            let v3312 = parameters[160];
            let v3313 = parameters[159];
            let v3314 = parameters[161];
            let v3315 = parameters[165];
            let v3316 = parameters[156];
            let v3317 = parameters[157];
            let v3318 = parameters[158];
            let v3319 = parameters[164];
            let v3320 = parameters[163];
            let v3321 = parameters[162];
            let v3376 = -5e1f64;
            let v3407 = -5e1f64;
            let v3494 = -5e1f64;
            let v3507 = -5e1f64;
            let v3512 = -5e1f64;
            let v3525 = -5e1f64;
            let v3550 = -5e1f64;
            let v3579 = -5e1f64;
            let v3646 = -5e1f64;
            let v3659 = -5e1f64;
            let v3664 = -5e1f64;
            let v3677 = -5e1f64;
            let v3686 = -5e1f64;
            let v3691 = -5e1f64;
            let v3700 = -5e1f64;
            let v3704 = parameters[61];
            let v3705 = parameters[60];
            let v3706 = parameters[62];
            let v3707 = parameters[65];
            let v3708 = parameters[57];
            let v3709 = parameters[58];
            let v3710 = parameters[59];
            let v3711 = parameters[64];
            let v3712 = parameters[63];
            let v3713 = parameters[46];
            let v3768 = -5e1f64;
            let v3799 = -5e1f64;
            let v3886 = -5e1f64;
            let v3899 = -5e1f64;
            let v3904 = -5e1f64;
            let v3917 = -5e1f64;
            let v3942 = -5e1f64;
            let v3971 = -5e1f64;
            let v4038 = -5e1f64;
            let v4051 = -5e1f64;
            let v4056 = -5e1f64;
            let v4069 = -5e1f64;
            let v4071 = 0.0f64;
            let v4078 = -5e1f64;
            let v4080 = -5e1f64;
            let v4082 = 0.0f64;
            let v4089 = -5e1f64;
            let v4093 = parameters[73];
            let v4094 = parameters[72];
            let v4095 = parameters[74];
            let v4096 = parameters[77];
            let v4097 = parameters[69];
            let v4098 = parameters[70];
            let v4099 = parameters[71];
            let v4100 = parameters[76];
            let v4101 = parameters[75];
            let v4156 = -5e1f64;
            let v4187 = -5e1f64;
            let v4274 = -5e1f64;
            let v4287 = -5e1f64;
            let v4292 = -5e1f64;
            let v4305 = -5e1f64;
            let v4330 = -5e1f64;
            let v4359 = -5e1f64;
            let v4426 = -5e1f64;
            let v4439 = -5e1f64;
            let v4444 = -5e1f64;
            let v4457 = -5e1f64;
            let v4459 = 0.0f64;
            let v4466 = -5e1f64;
            let v4468 = -5e1f64;
            let v4470 = 0.0f64;
            let v4477 = -5e1f64;
            let v4479 = parameters[1];
            let v4480 = parameters[35];
            let v4481 = parameters[36];
            let v4482 = parameters[37];
            let v4483 = parameters[38];
            let v4484 = parameters[40];
            let v4485 = parameters[41];
            let v4486 = parameters[32];
            let v4487 = parameters[34];
            let v4488 = parameters[44];
            let v4489 = parameters[43];
            let v4546 = -5e1f64;
            let v4577 = -5e1f64;
            let v4672 = -5e1f64;
            let v4686 = -5e1f64;
            let v4697 = -5e1f64;
            let v4711 = -5e1f64;
            let v4769 = -5e1f64;
            let v4798 = -5e1f64;
            let v4865 = -5e1f64;
            let v4879 = -5e1f64;
            let v4890 = -5e1f64;
            let v4904 = -5e1f64;
            let v4926 = 6.666666666666666e-1f64;
            let v4955 = 0.0f64;
            let v4962 = -5e1f64;
            let v4964 = -5e1f64;
            let v4966 = 0.0f64;
            let v4973 = -5e1f64;
            let v4975 = parameters[322];
            let v4978 = parameters[254];
            let v4982 = parameters[260];
            let v4983 = parameters[262];
            let v4984 = parameters[261];
            let v4985 = parameters[258];
            let v4986 = parameters[278];
            let v4987 = parameters[277];
            let v4988 = parameters[255];
            let v4990 = parameters[259];
            let v4992 = parameters[276];
            let v4993 = parameters[270];
            let v4994 = parameters[271];
            let v4995 = parameters[269];
            let v4997 = parameters[268];
            let v4998 = parameters[257];
            let v4999 = parameters[256];
            let v5004 = 5.184705528587072e21f64;
            let v5008 = -5e1f64;
            let v5010 = 1.9287498479639178e-22f64;
            let v5022 = 5.184705528587072e21f64;
            let v5026 = -5e1f64;
            let v5028 = 1.9287498479639178e-22f64;
            let v5033 = 5.184705528587072e21f64;
            let v5037 = -5e1f64;
            let v5039 = 1.9287498479639178e-22f64;
            let v5050 = 5.184705528587072e21f64;
            let v5054 = -5e1f64;
            let v5056 = 1.9287498479639178e-22f64;
            let v5070 = 5.184705528587072e21f64;
            let v5074 = -5e1f64;
            let v5076 = 1.9287498479639178e-22f64;
            let v5084 = 5.184705528587072e21f64;
            let v5088 = -5e1f64;
            let v5090 = 1.9287498479639178e-22f64;
            let v5107 = 5.184705528587072e21f64;
            let v5111 = -5e1f64;
            let v5113 = 1.9287498479639178e-22f64;
            let v5122 = 5.184705528587072e21f64;
            let v5126 = -5e1f64;
            let v5128 = 1.9287498479639178e-22f64;
            let v5145 = -5e1f64;
            let v5179 = 5.184705528587072e21f64;
            let v5183 = -5e1f64;
            let v5185 = 1.9287498479639178e-22f64;
            let v5195 = parameters[265];
            let v5196 = parameters[267];
            let v5197 = parameters[266];
            let v5198 = parameters[263];
            let v5199 = parameters[281];
            let v5200 = parameters[280];
            let v5201 = parameters[264];
            let v5203 = parameters[279];
            let v5204 = parameters[274];
            let v5205 = parameters[275];
            let v5206 = parameters[273];
            let v5208 = parameters[272];
            let v5209 = 5.184705528587072e21f64;
            let v5213 = -5e1f64;
            let v5215 = 1.9287498479639178e-22f64;
            let v5227 = 5.184705528587072e21f64;
            let v5231 = -5e1f64;
            let v5233 = 1.9287498479639178e-22f64;
            let v5238 = 5.184705528587072e21f64;
            let v5242 = -5e1f64;
            let v5244 = 1.9287498479639178e-22f64;
            let v5255 = 5.184705528587072e21f64;
            let v5259 = -5e1f64;
            let v5261 = 1.9287498479639178e-22f64;
            let v5275 = 5.184705528587072e21f64;
            let v5279 = -5e1f64;
            let v5281 = 1.9287498479639178e-22f64;
            let v5289 = 5.184705528587072e21f64;
            let v5293 = -5e1f64;
            let v5295 = 1.9287498479639178e-22f64;
            let v5312 = 5.184705528587072e21f64;
            let v5316 = -5e1f64;
            let v5318 = 1.9287498479639178e-22f64;
            let v5327 = 5.184705528587072e21f64;
            let v5331 = -5e1f64;
            let v5333 = 1.9287498479639178e-22f64;
            let v5350 = -5e1f64;
            let v5381 = 5.184705528587072e21f64;
            let v5385 = -5e1f64;
            let v5387 = 1.9287498479639178e-22f64;
            let v5395 = parameters[282];
            let v5397 = parameters[285];
            let v5398 = parameters[286];
            let v5399 = parameters[283];
            let v5400 = -5e1f64;
            let v5402 = -5e1f64;
            let v5404 = -5e1f64;
            let v5406 = -5e1f64;
            let v5408 = 1.0f64;
            let v5414 = -5e1f64;
            let v5419 = -5e1f64;
            let v5421 = 1.0f64;
            let v5422 = -5e1f64;
            let v5424 = -5e1f64;
            let v5433 = -5e1f64;
            let v5452 = -5e1f64;
            let v5454 = parameters[289];
            let v5455 = parameters[290];
            let v5456 = parameters[287];
            let v5457 = -5e1f64;
            let v5459 = -5e1f64;
            let v5461 = -5e1f64;
            let v5463 = -5e1f64;
            let v5465 = 1.0f64;
            let v5471 = -5e1f64;
            let v5476 = -5e1f64;
            let v5478 = 1.0f64;
            let v5479 = -5e1f64;
            let v5481 = -5e1f64;
            let v5490 = -5e1f64;
            let v5509 = -5e1f64;
            let v5514 = 5.184705528587072e21f64;
            let v5518 = -5e1f64;
            let v5520 = 1.9287498479639178e-22f64;
            let v5529 = 5.184705528587072e21f64;
            let v5533 = -5e1f64;
            let v5535 = 1.9287498479639178e-22f64;
            let v5539 = 5.184705528587072e21f64;
            let v5543 = -5e1f64;
            let v5545 = 1.9287498479639178e-22f64;
            let v5555 = 5.184705528587072e21f64;
            let v5559 = -5e1f64;
            let v5561 = 1.9287498479639178e-22f64;
            let v5574 = 5.184705528587072e21f64;
            let v5578 = -5e1f64;
            let v5580 = 1.9287498479639178e-22f64;
            let v5588 = 5.184705528587072e21f64;
            let v5592 = -5e1f64;
            let v5594 = 1.9287498479639178e-22f64;
            let v5611 = 5.184705528587072e21f64;
            let v5615 = -5e1f64;
            let v5617 = 1.9287498479639178e-22f64;
            let v5626 = 5.184705528587072e21f64;
            let v5630 = -5e1f64;
            let v5632 = 1.9287498479639178e-22f64;
            let v5649 = -5e1f64;
            let v5678 = 5.184705528587072e21f64;
            let v5682 = -5e1f64;
            let v5684 = 1.9287498479639178e-22f64;
            let v5695 = 5.184705528587072e21f64;
            let v5699 = -5e1f64;
            let v5701 = 1.9287498479639178e-22f64;
            let v5710 = 5.184705528587072e21f64;
            let v5714 = -5e1f64;
            let v5716 = 1.9287498479639178e-22f64;
            let v5720 = 5.184705528587072e21f64;
            let v5724 = -5e1f64;
            let v5726 = 1.9287498479639178e-22f64;
            let v5736 = 5.184705528587072e21f64;
            let v5740 = -5e1f64;
            let v5742 = 1.9287498479639178e-22f64;
            let v5755 = 5.184705528587072e21f64;
            let v5759 = -5e1f64;
            let v5761 = 1.9287498479639178e-22f64;
            let v5769 = 5.184705528587072e21f64;
            let v5773 = -5e1f64;
            let v5775 = 1.9287498479639178e-22f64;
            let v5792 = 5.184705528587072e21f64;
            let v5796 = -5e1f64;
            let v5798 = 1.9287498479639178e-22f64;
            let v5807 = 5.184705528587072e21f64;
            let v5811 = -5e1f64;
            let v5813 = 1.9287498479639178e-22f64;
            let v5830 = -5e1f64;
            let v5859 = 5.184705528587072e21f64;
            let v5863 = -5e1f64;
            let v5865 = 1.9287498479639178e-22f64;
            let v5873 = -5e1f64;
            let v5875 = -5e1f64;
            let v5877 = -5e1f64;
            let v5879 = -5e1f64;
            let v5881 = 1.0f64;
            let v5887 = -5e1f64;
            let v5892 = -5e1f64;
            let v5894 = 1.0f64;
            let v5895 = -5e1f64;
            let v5897 = -5e1f64;
            let v5906 = -5e1f64;
            let v5925 = -5e1f64;
            let v5927 = -5e1f64;
            let v5929 = -5e1f64;
            let v5931 = -5e1f64;
            let v5933 = -5e1f64;
            let v5935 = 1.0f64;
            let v5941 = -5e1f64;
            let v5946 = -5e1f64;
            let v5948 = 1.0f64;
            let v5949 = -5e1f64;
            let v5951 = -5e1f64;
            let v5960 = -5e1f64;
            let v5979 = -5e1f64;
            let v5981 = parameters[291];
            let v5985 = parameters[294];
            let v5986 = parameters[296];
            let v5987 = parameters[295];
            let v5988 = parameters[292];
            let v5989 = 6e2f64;
            let v5990 = parameters[311];
            let v5991 = parameters[299];
            let v5992 = parameters[300];
            let v5993 = parameters[297];
            let v5995 = -0e0f64;
            let v5998 = -5e1f64;
            let v6004 = -2.4e3f64;
            let v6007 = -5e1f64;
            let v6010 = -5e1f64;
            let v6016 = -5e1f64;
            let v6024 = -5e1f64;
            let v6029 = -5e1f64;
            let v6037 = -5e1f64;
            let v6042 = -5e1f64;
            let v6051 = -5e1f64;
            let v6070 = -5e1f64;
            let v6072 = parameters[301];
            let v6074 = parameters[304];
            let v6075 = parameters[305];
            let v6076 = parameters[302];
            let v6077 = -0e0f64;
            let v6080 = -5e1f64;
            let v6083 = -2.4e3f64;
            let v6086 = -5e1f64;
            let v6089 = -5e1f64;
            let v6094 = -5e1f64;
            let v6096 = 1.0f64;
            let v6097 = -2.404e3f64;
            let v6100 = -5e1f64;
            let v6104 = -5e1f64;
            let v6106 = 1.0f64;
            let v6107 = 0e0f64;
            let v6111 = -5e1f64;
            let v6116 = -5e1f64;
            let v6118 = 1e2f64;
            let v6125 = -5e1f64;
            let v6144 = -5e1f64;
            let v6146 = parameters[308];
            let v6147 = parameters[306];
            let v6150 = parameters[309];
            let v6155 = 5e0f64;
            let v6157 = parameters[310];
            let v6169 = parameters[312];
            let v6171 = parameters[313];
            let v6182 = parameters[317];
            let v6183 = parameters[316];
            let v6188 = -5e1f64;
            let v6198 = -5e1f64;
            let v6201 = -5e1f64;
            let v6206 = -5e1f64;
            let v6214 = -5e1f64;
            let v6219 = -5e1f64;
            let v6227 = -5e1f64;
            let v6232 = -5e1f64;
            let v6241 = -5e1f64;
            let v6260 = -5e1f64;
            let v6263 = parameters[319];
            let v6264 = parameters[318];
            let v6265 = -5e1f64;
            let v6275 = -5e1f64;
            let v6278 = -5e1f64;
            let v6283 = -5e1f64;
            let v6291 = -5e1f64;
            let v6296 = -5e1f64;
            let v6304 = -5e1f64;
            let v6309 = -5e1f64;
            let v6318 = -5e1f64;
            let v6337 = -5e1f64;
            let v6346 = node_potentials[6];
            let v6351 = parameters[27];
            let v6353 = parameters[28];
            let v6356 = -5e1f64;
            let v6362 = -5e1f64;
            let v6368 = -5e1f64;
            let v6374 = -5e1f64;
            let v6380 = -5e1f64;
            let v6386 = -5e1f64;
            let v6388 = parameters[347];
            let v6390 = parameters[348];
            let v6403 = parameters[349];
            let v6432 = parameters[350];
            let v6437 = parameters[351];
            let v6443 = parameters[352];
            let v6444 = 5.52248e-23f64;
            let v6453 = 5.52248e-23f64;
            let v6459 = 5.52248e-23f64;
            let v6465 = 5.52248e-23f64;
            let v6471 = 5.52248e-23f64;
            let v6477 = 5.52248e-23f64;
            let v6483 = 5.52248e-23f64;
            let v6489 = 5.52248e-23f64;
            let v6495 = 5.52248e-23f64;
            let v6500 = 5.52248e-23f64;
            let v6503 = 5.52248e-23f64;
            let v6506 = parameters[320];
            let v6561 = 1e0f64;
            let v6562 = 1e0f64;
            let v6563 = 1e0f64;
            let v6564 = 1e0f64;
            let v6565 = 1e0f64;
            let v6566 = 1e0f64;
            let v6567 = 1e0f64;
            let v6568 = 1e0f64;
            let v6569 = 1e0f64;
            let v6594 = 0e0f64;
            let v6608 = 2e0f64;
            let v6609 = -1e0f64;
            let v6610 = Lanes([0e0f64; 4]);
            let v6614 = 0e0f64;
            let v6663 = Lanes([0e0f64; 2]);
            let v6728 = Lanes([0e0f64; 4]);
            let v3 = v1 + v2;
            if v6 != 0.0 {
            } else {
            }
            let v10 = (v4 + v8) + v7;
            let v12 = if v10 < v11 { 1.0 } else { 0.0 };
            let v46: f64;
            let v6570: f64;
            if v12 != 0.0 {
                v46 = v13;
                v6570 = v6594;
            } else {
                let v15 = if v10 > v14 { 1.0 } else { 0.0 };
                let v47: f64;
                let v6571: f64;
                if v15 != 0.0 {
                    v47 = v16;
                    v6571 = v6594;
                } else {
                    v47 = v10;
                    v6571 = v6562;
                }
                v46 = v47;
                v6570 = v6571;
            }
            let v18 = if v17 == v0 { 1.0 } else { 0.0 };
            let v40: f64;
            let v59: f64;
            if v18 != 0.0 {
                let v23 = (v19 / v20) / v22;
                let v26 = (v24 / v20) / v22;
                v40 = v23;
                v59 = v26;
            } else {
                let v33 = ((v19 / v20) + ((v28 * v29) / v20)) / v22;
                let v39 = ((v24 / v20) + ((v28 * v35) / v20)) / v22;
                v40 = v33;
                v59 = v39;
            }
            let v44 = if (if v40 >= v41 { 1.0 } else { 0.0 }) != 0.0 && (if v40 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v6341: f64;
            if v44 != 0.0 {
                let v48 = v46 - v3;
                let v55 = v40 * ((v5 + (v45 * v48)) + ((v51 * v48) * v48));
                let v57 = v56 * v40;
                let v58 = if v55 < v57 { 1.0 } else { 0.0 };
                let v6342: f64;
                if v58 != 0.0 {
                    v6342 = v57;
                } else {
                    v6342 = v55;
                }
                v6341 = v6342;
            } else {
                v6341 = v0;
            }
            let v62 = if (if v59 >= v41 { 1.0 } else { 0.0 }) != 0.0 && (if v59 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v6339: f64;
            if v62 != 0.0 {
                let v63 = v46 - v3;
                let v69 = v59 * ((v5 + (v45 * v63)) + ((v51 * v63) * v63));
                let v70 = v56 * v59;
                let v71 = if v69 < v70 { 1.0 } else { 0.0 };
                let v6340: f64;
                if v71 != 0.0 {
                    v6340 = v70;
                } else {
                    v6340 = v69;
                }
                v6339 = v6340;
            } else {
                v6339 = v0;
            }
            let v75 = (v72 / v22) / v74;
            let v81 = v75 * (v76 + ((v77 * v20) / v74));
            let v85 = v75 * (((v5 - v77) * v20) / v74);
            let v89 = (v86 * v46) / v88;
            let v6596 = (v6570 * v86) / v88;
            let v91 = v46 - v3;
            let v93 = v5 + (v90 * v91);
            let v94 = if v93 < v56 { 1.0 } else { 0.0 };
            let v321: f64;
            if v94 != 0.0 {
                v321 = v56;
            } else {
                v321 = v93;
            }
            let v95 = v46 / v3;
            let v6597 = v6570 / v3;
            let v97 = (v95 * v95) * v95;
            let v102 = if (v5 + (v98 * v91)) < v101 { 1.0 } else { 0.0 };
            if v102 != 0.0 {
            } else {
            }
            let v106 = if (v5 + (v103 * v91)) < v101 { 1.0 } else { 0.0 };
            if v106 != 0.0 {
            } else {
            }
            let v110 = if (v5 + (v107 * v91)) < v101 { 1.0 } else { 0.0 };
            if v110 != 0.0 {
            } else {
            }
            let v114 = if (v5 + (v111 * v91)) < v101 { 1.0 } else { 0.0 };
            if v114 != 0.0 {
            } else {
            }
            let v118 = if (v5 + (v115 * v91)) < v101 { 1.0 } else { 0.0 };
            if v118 != 0.0 {
            } else {
            }
            let v122 = if (v5 + (v119 * v91)) < v101 { 1.0 } else { 0.0 };
            if v122 != 0.0 {
            } else {
            }
            if v102 != 0.0 {
            } else {
            }
            if v106 != 0.0 {
            } else {
            }
            if v110 != 0.0 {
            } else {
            }
            if v114 != 0.0 {
            } else {
            }
            if v118 != 0.0 {
            } else {
            }
            if v122 != 0.0 {
            } else {
            }
            let v6598 = v6570 * v124;
            let v126 = v5 + (v124 * v91);
            let v127 = if v126 < v101 { 1.0 } else { 0.0 };
            let v128: f64;
            let v6572: f64;
            if v127 != 0.0 {
                v128 = v101;
                v6572 = v6594;
            } else {
                v128 = v126;
                v6572 = v6598;
            }
            let v129 = v123 * v128;
            let v6599 = v6572 * v123;
            let v133 = v5 + (v131 * v91);
            let v134 = if v133 < v101 { 1.0 } else { 0.0 };
            let v135: f64;
            if v134 != 0.0 {
                v135 = v101;
            } else {
                v135 = v133;
            }
            let v136 = v130 * v135;
            let v140 = v5 + (v138 * v91);
            let v141 = if v140 < v101 { 1.0 } else { 0.0 };
            let v142: f64;
            if v141 != 0.0 {
                v142 = v101;
            } else {
                v142 = v140;
            }
            let v143 = v137 * v142;
            let v147 = v5 + (v145 * v91);
            let v148 = if v147 < v101 { 1.0 } else { 0.0 };
            let v149: f64;
            if v148 != 0.0 {
                v149 = v101;
            } else {
                v149 = v147;
            }
            let v150 = v144 * v149;
            let v154 = v5 + (v152 * v91);
            let v155 = if v154 < v101 { 1.0 } else { 0.0 };
            let v156: f64;
            if v155 != 0.0 {
                v156 = v101;
            } else {
                v156 = v154;
            }
            let v157 = v151 * v156;
            let v161 = if (v5 + (v158 * v91)) < v101 { 1.0 } else { 0.0 };
            if v161 != 0.0 {
            } else {
            }
            let v165 = if (v5 + (v162 * v91)) < v101 { 1.0 } else { 0.0 };
            if v165 != 0.0 {
            } else {
            }
            let v169 = if (v5 + (v166 * v91)) < v101 { 1.0 } else { 0.0 };
            if v169 != 0.0 {
            } else {
            }
            let v173 = if (v5 + (v170 * v91)) < v101 { 1.0 } else { 0.0 };
            if v173 != 0.0 {
            } else {
            }
            let v177 = if (v5 + (v174 * v91)) < v101 { 1.0 } else { 0.0 };
            if v177 != 0.0 {
            } else {
            }
            let v181 = if (v5 + (v178 * v91)) < v101 { 1.0 } else { 0.0 };
            if v181 != 0.0 {
            } else {
            }
            let v185 = if (v5 + (v182 * v91)) < v101 { 1.0 } else { 0.0 };
            if v185 != 0.0 {
            } else {
            }
            let v189 = if (v5 + (v186 * v91)) < v101 { 1.0 } else { 0.0 };
            if v189 != 0.0 {
            } else {
            }
            let v193 = v5 + (v191 * v91);
            let v194 = if v193 < v101 { 1.0 } else { 0.0 };
            let v195: f64;
            if v194 != 0.0 {
                v195 = v101;
            } else {
                v195 = v193;
            }
            let v196 = v190 * v195;
            let v200 = v5 + (v198 * v91);
            let v201 = if v200 < v101 { 1.0 } else { 0.0 };
            let v202: f64;
            if v201 != 0.0 {
                v202 = v101;
            } else {
                v202 = v200;
            }
            let v203 = v197 * v202;
            let v207 = v5 + (v205 * v91);
            let v208 = if v207 < v101 { 1.0 } else { 0.0 };
            let v209: f64;
            if v208 != 0.0 {
                v209 = v101;
            } else {
                v209 = v207;
            }
            let v210 = v204 * v209;
            let v214 = v5 + (v212 * v91);
            let v215 = if v214 < v101 { 1.0 } else { 0.0 };
            let v216: f64;
            if v215 != 0.0 {
                v216 = v101;
            } else {
                v216 = v214;
            }
            let v217 = v211 * v216;
            let v221 = if (v5 + (v218 * v91)) < v101 { 1.0 } else { 0.0 };
            if v221 != 0.0 {
            } else {
            }
            let v225 = if (v5 + (v222 * v91)) < v101 { 1.0 } else { 0.0 };
            if v225 != 0.0 {
            } else {
            }
            let v229 = if (v5 + (v226 * v91)) < v101 { 1.0 } else { 0.0 };
            if v229 != 0.0 {
            } else {
            }
            let v233 = if (v5 + (v230 * v91)) < v101 { 1.0 } else { 0.0 };
            if v233 != 0.0 {
            } else {
            }
            let v237 = if (v5 + (v234 * v91)) < v101 { 1.0 } else { 0.0 };
            if v237 != 0.0 {
            } else {
            }
            let v241 = if (v5 + (v238 * v91)) < v101 { 1.0 } else { 0.0 };
            if v241 != 0.0 {
            } else {
            }
            let v245 = if (v5 + (v242 * v91)) < v101 { 1.0 } else { 0.0 };
            if v245 != 0.0 {
            } else {
            }
            let v249 = if (v5 + (v246 * v91)) < v101 { 1.0 } else { 0.0 };
            if v249 != 0.0 {
            } else {
            }
            let v254 = v250 * (v251 - v252);
            let v6603 = ((Lanes([v6563, 0.0])) - (Lanes([0.0, v6564]))) * v250;
            let v257 = v250 * (v255 - v252);
            let v6607 = ((Lanes([v6565, 0.0])) - (Lanes([0.0, v6564]))) * v250;
            let v259 = if v258 == v0 { 1.0 } else { 0.0 };
            let v301: f64;
            if v259 != 0.0 {
                let v263 = v250 * (v260 - v261);
                let v266 = v250 * (v260 - v264);
                let v267 = if v263 <= v266 { 1.0 } else { 0.0 };
                let v302: f64;
                if v267 != 0.0 {
                    v302 = v266;
                } else {
                    v302 = v263;
                }
                v301 = v302;
            } else {
                let v269 = v250 * (v260 - v261);
                let v271 = v250 * (v260 - v264);
                let v290: f64;
                if v259 != 0.0 {
                    let v275 = v269 - v271;
                    let v280 = v273 * ((v269 + v271) + (((v275 * v275) + v272).sqrt()));
                    v290 = v280;
                } else {
                    let v282 = v269 - v271;
                    let v289 = v273 * ((v269 + v271) + (v282 * (((v283 / v272) * v282).tanh())));
                    v290 = v289;
                }
                v301 = v290;
            }
            let v300 = v250 * (v298 - v260);
            let v303 = (v291 + (v5 / ((v28 * v292) * v294))) - v301;
            let v305 = if v304 == v5 { 1.0 } else { 0.0 };
            let v380: f64;
            let v4490: f64;
            let v6573: Lanes<4>;
            if v305 != 0.0 {
                let v315 = (((v261 - v306) - v308) - (v310 * v311)) / v314;
                let v317 = if v315 > v316 { 1.0 } else { 0.0 };
                if v317 != 0.0 {
                } else {
                    let v319 = if v315 < v318 { 1.0 } else { 0.0 };
                    if v319 != 0.0 {
                    } else {
                    }
                }
                let v323 = v5 + (v320 * v321);
                v380 = v323;
                v4490 = v5;
                v6573 = v6610;
            } else {
                let v325 = if v304 == v324 { 1.0 } else { 0.0 };
                let v4491: f64;
                let v6574: Lanes<4>;
                if v325 != 0.0 {
                    let v330 = (v328 - v327) / v89;
                    let v331 = if v330 > v316 { 1.0 } else { 0.0 };
                    if v331 != 0.0 {
                    } else {
                        let v333 = if v330 < v332 { 1.0 } else { 0.0 };
                        if v333 != 0.0 {
                        } else {
                        }
                    }
                    let v334 = v327 - v326;
                    let v337 = (v334.abs()) / v336;
                    let v6619 = (((Lanes([0.0, v6567])) - (Lanes([v6566, 0.0]))) * ((v6608 * (if v334 >= v6614 { 1.0 } else { 0.0 })) - v6561)) / v336;
                    let v342 = (v339 - v340) / v89;
                    let v343 = if v342 > v316 { 1.0 } else { 0.0 };
                    if v343 != 0.0 {
                    } else {
                        let v345 = if v342 < v344 { 1.0 } else { 0.0 };
                        if v345 != 0.0 {
                        } else {
                        }
                    }
                    let v346 = v339 - v338;
                    let v6627 = (((Lanes([0.0, v6569])) - (Lanes([v6568, 0.0]))) * ((v6608 * (if v346 >= v6614 { 1.0 } else { 0.0 })) - v6561)) / v348;
                    let v351 = (v5 + v337) + ((v346.abs()) / v348);
                    let v352 = v5 / v351;
                    let v6633 = ((((Lanes([v6619[0], v6619[1], 0.0, 0.0])) + (Lanes([0.0, 0.0, v6627[0], v6627[1]]))) * v352) * v6609) / v351;
                    v4491 = v352;
                    v6574 = v6633;
                } else {
                    v4491 = v5;
                    v6574 = v6610;
                }
                v380 = v5;
                v4490 = v4491;
                v6573 = v6574;
            }
            let v390: f64;
            if v259 != 0.0 {
                let v355 = v250 * (v353 - v261);
                let v357 = v250 * (v353 - v264);
                let v358 = if v355 <= v357 { 1.0 } else { 0.0 };
                let v391: f64;
                if v358 != 0.0 {
                    v391 = v357;
                } else {
                    v391 = v355;
                }
                v390 = v391;
            } else {
                let v360 = v250 * (v353 - v261);
                let v362 = v250 * (v353 - v264);
                let v378: f64;
                if v259 != 0.0 {
                    let v364 = v360 - v362;
                    let v369 = v273 * ((v360 + v362) + (((v364 * v364) + v272).sqrt()));
                    v378 = v369;
                } else {
                    let v371 = v360 - v362;
                    let v377 = v273 * ((v360 + v362) + (v371 * (((v283 / v272) * v371).tanh())));
                    v378 = v377;
                }
                v390 = v378;
            }
            let v389 = v250 * (v387 - v353);
            let v392 = (v379 + (v5 / (((v380 * v28) * v382) * v294))) - v390;
            let v394 = if v393 == v5 { 1.0 } else { 0.0 };
            let v2116: f64;
            let v2118: f64;
            if v394 != 0.0 {
                let v398 = v250 * (v395 - v396);
                let v400 = v250 * (v264 - v396);
                v2116 = v398;
                v2118 = v400;
            } else {
                let v402 = v250 * (v264 - v396);
                let v404 = v250 * (v395 - v396);
                v2116 = v402;
                v2118 = v404;
            }
            let v406 = v250 * (v252 - v396);
            let v409 = v250 * (v407 - v396);
            let v411 = if v410 == v5 { 1.0 } else { 0.0 };
            let v2513: f64;
            let v2515: f64;
            if v411 != 0.0 {
                let v414 = v250 * (v395 - v412);
                let v416 = v250 * (v264 - v412);
                v2513 = v414;
                v2515 = v416;
            } else {
                let v418 = v250 * (v264 - v412);
                let v420 = v250 * (v395 - v412);
                v2513 = v418;
                v2515 = v420;
            }
            let v422 = v250 * (v396 - v412);
            let v424 = v250 * (v407 - v412);
            let v426 = if v425 == v5 { 1.0 } else { 0.0 };
            let v2910: f64;
            let v2912: f64;
            if v426 != 0.0 {
                let v429 = v250 * (v395 - v427);
                let v431 = v250 * (v264 - v427);
                v2910 = v429;
                v2912 = v431;
            } else {
                let v433 = v250 * (v264 - v427);
                let v435 = v250 * (v395 - v427);
                v2910 = v433;
                v2912 = v435;
            }
            let v437 = v250 * (v412 - v427);
            let v439 = v250 * (v407 - v427);
            let v441 = if v440 == v5 { 1.0 } else { 0.0 };
            let v3307: f64;
            let v3309: f64;
            if v441 != 0.0 {
                let v443 = v250 * (v395 - v298);
                let v445 = v250 * (v264 - v298);
                v3307 = v443;
                v3309 = v445;
            } else {
                let v447 = v250 * (v264 - v298);
                let v449 = v250 * (v395 - v298);
                v3307 = v447;
                v3309 = v449;
            }
            let v451 = v250 * (v427 - v298);
            let v453 = v250 * (v407 - v298);
            let v455 = if v454 == v5 { 1.0 } else { 0.0 };
            let v1719: f64;
            let v1721: f64;
            if v455 != 0.0 {
                let v457 = v250 * (v395 - v251);
                let v459 = v250 * (v264 - v251);
                v1719 = v457;
                v1721 = v459;
            } else {
                let v461 = v250 * (v264 - v251);
                let v463 = v250 * (v395 - v251);
                v1719 = v461;
                v1721 = v463;
            }
            let v466 = v250 * (v464 - v251);
            let v468 = v250 * (v407 - v251);
            let v470 = if v469 == v5 { 1.0 } else { 0.0 };
            let v1322: f64;
            let v1324: f64;
            if v470 != 0.0 {
                let v472 = v250 * (v395 - v464);
                let v474 = v250 * (v264 - v464);
                v1322 = v472;
                v1324 = v474;
            } else {
                let v476 = v250 * (v264 - v464);
                let v478 = v250 * (v395 - v464);
                v1322 = v476;
                v1324 = v478;
            }
            let v481 = v250 * (v479 - v464);
            let v483 = v250 * (v407 - v464);
            let v485 = if v484 == v5 { 1.0 } else { 0.0 };
            let v925: f64;
            let v927: f64;
            if v485 != 0.0 {
                let v487 = v250 * (v395 - v479);
                let v489 = v250 * (v264 - v479);
                v925 = v487;
                v927 = v489;
            } else {
                let v491 = v250 * (v264 - v479);
                let v493 = v250 * (v395 - v479);
                v925 = v491;
                v927 = v493;
            }
            let v496 = v250 * (v494 - v479);
            let v498 = v250 * (v407 - v479);
            let v500 = if v499 == v5 { 1.0 } else { 0.0 };
            let v516: f64;
            let v518: f64;
            if v500 != 0.0 {
                let v502 = v250 * (v395 - v494);
                let v504 = v250 * (v264 - v494);
                v516 = v502;
                v518 = v504;
            } else {
                let v506 = v250 * (v264 - v494);
                let v508 = v250 * (v395 - v494);
                v516 = v506;
                v518 = v508;
            }
            let v510 = v250 * (v353 - v494);
            let v512 = v250 * (v407 - v494);
            let v515 = if v513 > v514 { 1.0 } else { 0.0 };
            if v515 != 0.0 {
                let v542: f64;
                if v259 != 0.0 {
                    let v537 = ((v510 * v510) + v272).sqrt();
                    v542 = v537;
                } else {
                    let v541 = v510 * (((v283 / v272) * v510).tanh());
                    v542 = v541;
                }
                let v543 = v516 - v510;
                let v544 = v524 * v89;
                let v547 = v521 / (v545 * v89);
                let v549 = v547 + (v523 * v542);
                let v551 = v520 + (v530 * v91);
                let v552 = v95.powf(v532);
                let v553 = if v531 != v0 { 1.0 } else { 0.0 };
                let v560: f64;
                if v553 != 0.0 {
                    let v559 = v542 / ((v5 + ((v542 / v531).powf(v527))).powf((v5 / v527)));
                    v560 = v559;
                } else {
                    v560 = v0;
                }
                let v564 = v551 - ((v522 - (v560 * v0)) * v542);
                let v566 = (v324 * v549) * v89;
                let v567 = v217 * v566;
                let v570 = (v568 * v544) / v324;
                let v571 = v564 - v570;
                let v587: f64;
                if v259 != 0.0 {
                    let v573 = v516 - v543;
                    let v578 = v273 * ((v516 + v543) + (((v573 * v573) + v272).sqrt()));
                    v587 = v578;
                } else {
                    let v580 = v516 - v543;
                    let v586 = v273 * ((v516 + v543) + (v580 * (((v283 / v272) * v580).tanh())));
                    v587 = v586;
                }
                let v589 = (v587 - v571) / v544;
                let v590 = if v589 > v316 { 1.0 } else { 0.0 };
                let v614: f64;
                if v590 != 0.0 {
                    v614 = v0;
                } else {
                    let v592 = if v589 < v591 { 1.0 } else { 0.0 };
                    let v615: f64;
                    if v592 != 0.0 {
                        v615 = v5;
                    } else {
                        let v595 = v5 / (v5 + (v589.exp()));
                        v615 = v595;
                    }
                    v614 = v615;
                }
                let v611: f64;
                if v259 != 0.0 {
                    let v597 = v516 - v543;
                    let v602 = v273 * ((v516 + v543) + (((v597 * v597) + v272).sqrt()));
                    v611 = v602;
                } else {
                    let v604 = v516 - v543;
                    let v610 = v273 * ((v516 + v543) + (v604 * (((v283 / v272) * v604).tanh())));
                    v611 = v610;
                }
                let v613 = (v568 * v56) * v544;
                let v619 = (v611 - (v564 - (v613 * v614))) / v566;
                let v620 = if v619 > v316 { 1.0 } else { 0.0 };
                let v630: f64;
                if v620 != 0.0 {
                    let v621 = v567 * v619;
                    v630 = v621;
                } else {
                    let v623 = if v619 < v622 { 1.0 } else { 0.0 };
                    let v631: f64;
                    if v623 != 0.0 {
                        let v625 = v567 * (v619.exp());
                        v631 = v625;
                    } else {
                        let v629 = v567 * ((v5 + (v619.exp())).ln());
                        v631 = v629;
                    }
                    v630 = v631;
                }
                let v642 = v525 * ((v5 + (v533 * v3)) / (v5 + (v533 * v46)));
                let v653 = (((v642 * (v5 + ((v534 * v542) / v513))) / (v5 + ((v529 * v630) / v217))) * v513) / (v526 / (v552 * (v5 + ((v528 * v630) / v217))));
                let v663 = (((v653 * ((v5 + (((v324 * v630) / v217) / v653)).sqrt())) - v653) * (v5 - v614)) + (v566 * v614);
                let v664 = v510 / v663;
                let v678: f64;
                if v259 != 0.0 {
                    let v665 = v0 - v664;
                    let v670 = v273 * (v664 + (((v665 * v665) + v272).sqrt()));
                    v678 = v670;
                } else {
                    let v671 = v0 - v664;
                    let v677 = v273 * (v664 + (v671 * (((v283 / v272) * v671).tanh())));
                    v678 = v677;
                }
                let v681 = v5 / v527;
                let v684 = v510 * (v5 / ((v5 + (v678.powf(v527))).powf(v681)));
                let v685 = -v510;
                let v686 = v685 / v663;
                let v700: f64;
                if v259 != 0.0 {
                    let v687 = v0 - v686;
                    let v692 = v273 * (v686 + (((v687 * v687) + v272).sqrt()));
                    v700 = v692;
                } else {
                    let v693 = v0 - v686;
                    let v699 = v273 * (v686 + (v693 * (((v283 / v272) * v693).tanh())));
                    v700 = v699;
                }
                let v705 = v685 * (v5 / ((v5 + (v700.powf(v527))).powf(v681)));
                let v707 = (v516 - v571) / v544;
                let v708 = if v707 > v316 { 1.0 } else { 0.0 };
                let v715: f64;
                if v708 != 0.0 {
                    v715 = v0;
                } else {
                    let v710 = if v707 < v709 { 1.0 } else { 0.0 };
                    let v716: f64;
                    if v710 != 0.0 {
                        v716 = v5;
                    } else {
                        let v713 = v5 / (v5 + (v707.exp()));
                        v716 = v713;
                    }
                    v715 = v716;
                }
                let v720 = ((v543 - v705) - (v564 - (v613 * v715))) / v566;
                let v721 = if v720 > v316 { 1.0 } else { 0.0 };
                if v721 != 0.0 {
                } else {
                    let v723 = if v720 < v722 { 1.0 } else { 0.0 };
                    if v723 != 0.0 {
                    } else {
                    }
                }
                let v725 = (v543 - v571) / v544;
                let v726 = if v725 > v316 { 1.0 } else { 0.0 };
                let v733: f64;
                if v726 != 0.0 {
                    v733 = v0;
                } else {
                    let v728 = if v725 < v727 { 1.0 } else { 0.0 };
                    let v734: f64;
                    if v728 != 0.0 {
                        v734 = v5;
                    } else {
                        let v731 = v5 / (v5 + (v725.exp()));
                        v734 = v731;
                    }
                    v733 = v734;
                }
                let v738 = ((v516 - v684) - (v564 - (v613 * v733))) / v566;
                let v739 = if v738 > v316 { 1.0 } else { 0.0 };
                if v739 != 0.0 {
                } else {
                    let v741 = if v738 < v740 { 1.0 } else { 0.0 };
                    if v741 != 0.0 {
                    } else {
                    }
                }
                if v259 != 0.0 {
                } else {
                }
                let v743 = (v324 * v547) * v89;
                let v744 = v217 * v743;
                let v745 = v551 - v570;
                let v761: f64;
                if v259 != 0.0 {
                    let v747 = v516 - v543;
                    let v752 = v273 * ((v516 + v543) + (((v747 * v747) + v272).sqrt()));
                    v761 = v752;
                } else {
                    let v754 = v516 - v543;
                    let v760 = v273 * ((v516 + v543) + (v754 * (((v283 / v272) * v754).tanh())));
                    v761 = v760;
                }
                let v763 = (v761 - v745) / v544;
                let v764 = if v763 > v316 { 1.0 } else { 0.0 };
                let v786: f64;
                if v764 != 0.0 {
                    v786 = v0;
                } else {
                    let v766 = if v763 < v765 { 1.0 } else { 0.0 };
                    let v787: f64;
                    if v766 != 0.0 {
                        v787 = v5;
                    } else {
                        let v769 = v5 / (v5 + (v763.exp()));
                        v787 = v769;
                    }
                    v786 = v787;
                }
                let v785: f64;
                if v259 != 0.0 {
                    let v771 = v516 - v543;
                    let v776 = v273 * ((v516 + v543) + (((v771 * v771) + v272).sqrt()));
                    v785 = v776;
                } else {
                    let v778 = v516 - v543;
                    let v784 = v273 * ((v516 + v543) + (v778 * (((v283 / v272) * v778).tanh())));
                    v785 = v784;
                }
                let v791 = (v785 - (v551 - (v613 * v786))) / v743;
                let v792 = if v791 > v316 { 1.0 } else { 0.0 };
                let v805: f64;
                if v792 != 0.0 {
                    let v793 = v744 * v791;
                    v805 = v793;
                } else {
                    let v795 = if v791 < v794 { 1.0 } else { 0.0 };
                    let v806: f64;
                    if v795 != 0.0 {
                        let v797 = v744 * (v791.exp());
                        v806 = v797;
                    } else {
                        let v801 = v744 * ((v5 + (v791.exp())).ln());
                        v806 = v801;
                    }
                    v805 = v806;
                }
                let v804 = (v642 * v513) / (v526 / v552);
                let v817 = (((v804 * ((v5 + (((v324 * v805) / v217) / v804)).sqrt())) - v804) * (v5 - v786)) + (v743 * v786);
                let v818 = v510 / v817;
                let v832: f64;
                if v259 != 0.0 {
                    let v819 = v0 - v818;
                    let v824 = v273 * (v818 + (((v819 * v819) + v272).sqrt()));
                    v832 = v824;
                } else {
                    let v825 = v0 - v818;
                    let v831 = v273 * (v818 + (v825 * (((v283 / v272) * v825).tanh())));
                    v832 = v831;
                }
                let v837 = v510 * (v5 / ((v5 + (v832.powf(v527))).powf(v681)));
                let v838 = v685 / v817;
                let v852: f64;
                if v259 != 0.0 {
                    let v839 = v0 - v838;
                    let v844 = v273 * (v838 + (((v839 * v839) + v272).sqrt()));
                    v852 = v844;
                } else {
                    let v845 = v0 - v838;
                    let v851 = v273 * (v838 + (v845 * (((v283 / v272) * v845).tanh())));
                    v852 = v851;
                }
                let v857 = v685 * (v5 / ((v5 + (v852.powf(v527))).powf(v681)));
                let v859 = (v516 - v745) / v544;
                let v860 = if v859 > v316 { 1.0 } else { 0.0 };
                let v867: f64;
                if v860 != 0.0 {
                    v867 = v0;
                } else {
                    let v862 = if v859 < v861 { 1.0 } else { 0.0 };
                    let v868: f64;
                    if v862 != 0.0 {
                        v868 = v5;
                    } else {
                        let v865 = v5 / (v5 + (v859.exp()));
                        v868 = v865;
                    }
                    v867 = v868;
                }
                let v872 = ((v543 - v857) - (v551 - (v613 * v867))) / v743;
                let v873 = if v872 > v316 { 1.0 } else { 0.0 };
                if v873 != 0.0 {
                } else {
                    let v875 = if v872 < v874 { 1.0 } else { 0.0 };
                    if v875 != 0.0 {
                    } else {
                    }
                }
                let v877 = (v543 - v745) / v544;
                let v878 = if v877 > v316 { 1.0 } else { 0.0 };
                let v885: f64;
                if v878 != 0.0 {
                    v885 = v0;
                } else {
                    let v880 = if v877 < v879 { 1.0 } else { 0.0 };
                    let v886: f64;
                    if v880 != 0.0 {
                        v886 = v5;
                    } else {
                        let v883 = v5 / (v5 + (v877.exp()));
                        v886 = v883;
                    }
                    v885 = v886;
                }
                let v890 = ((v516 - v837) - (v551 - (v613 * v885))) / v743;
                let v891 = if v890 > v316 { 1.0 } else { 0.0 };
                if v891 != 0.0 {
                } else {
                    let v893 = if v890 < v892 { 1.0 } else { 0.0 };
                    if v893 != 0.0 {
                    } else {
                    }
                }
                let v900 = if v517 == v5 { 1.0 } else { 0.0 };
                if v900 != 0.0 {
                    let v903 = v551 - ((v568 * v273) * v544);
                    let v905 = (v518 - v903) / v743;
                    let v906 = if v905 > v316 { 1.0 } else { 0.0 };
                    if v906 != 0.0 {
                    } else {
                        let v908 = if v905 < v907 { 1.0 } else { 0.0 };
                        if v908 != 0.0 {
                        } else {
                        }
                    }
                    let v910 = (v512 - v903) / v743;
                    let v911 = if v910 > v316 { 1.0 } else { 0.0 };
                    if v911 != 0.0 {
                    } else {
                        let v913 = if v910 < v912 { 1.0 } else { 0.0 };
                        if v913 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v914 = if v519 == v5 { 1.0 } else { 0.0 };
                if v914 != 0.0 {
                    let v919 = (v516 - (v551 - ((v568 * v273) * v544))) / v743;
                    let v920 = if v919 > v316 { 1.0 } else { 0.0 };
                    if v920 != 0.0 {
                    } else {
                        let v922 = if v919 < v921 { 1.0 } else { 0.0 };
                        if v922 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if v500 != 0.0 {
            } else {
            }
            let v924 = if v923 > v514 { 1.0 } else { 0.0 };
            if v924 != 0.0 {
                let v947: f64;
                if v259 != 0.0 {
                    let v942 = ((v496 * v496) + v272).sqrt();
                    v947 = v942;
                } else {
                    let v946 = v496 * (((v283 / v272) * v496).tanh());
                    v947 = v946;
                }
                let v948 = v925 - v496;
                let v949 = v933 * v89;
                let v951 = v930 / (v545 * v89);
                let v953 = v951 + (v932 * v947);
                let v955 = v929 + (v939 * v91);
                let v956 = v95.powf(v532);
                let v957 = if v531 != v0 { 1.0 } else { 0.0 };
                let v964: f64;
                if v957 != 0.0 {
                    let v963 = v947 / ((v5 + ((v947 / v531).powf(v936))).powf((v5 / v936)));
                    v964 = v963;
                } else {
                    v964 = v0;
                }
                let v968 = v955 - ((v931 - (v964 * v0)) * v947);
                let v970 = (v324 * v953) * v89;
                let v971 = v210 * v970;
                let v973 = (v568 * v949) / v324;
                let v974 = v968 - v973;
                let v990: f64;
                if v259 != 0.0 {
                    let v976 = v925 - v948;
                    let v981 = v273 * ((v925 + v948) + (((v976 * v976) + v272).sqrt()));
                    v990 = v981;
                } else {
                    let v983 = v925 - v948;
                    let v989 = v273 * ((v925 + v948) + (v983 * (((v283 / v272) * v983).tanh())));
                    v990 = v989;
                }
                let v992 = (v990 - v974) / v949;
                let v993 = if v992 > v316 { 1.0 } else { 0.0 };
                let v1017: f64;
                if v993 != 0.0 {
                    v1017 = v0;
                } else {
                    let v995 = if v992 < v994 { 1.0 } else { 0.0 };
                    let v1018: f64;
                    if v995 != 0.0 {
                        v1018 = v5;
                    } else {
                        let v998 = v5 / (v5 + (v992.exp()));
                        v1018 = v998;
                    }
                    v1017 = v1018;
                }
                let v1014: f64;
                if v259 != 0.0 {
                    let v1000 = v925 - v948;
                    let v1005 = v273 * ((v925 + v948) + (((v1000 * v1000) + v272).sqrt()));
                    v1014 = v1005;
                } else {
                    let v1007 = v925 - v948;
                    let v1013 = v273 * ((v925 + v948) + (v1007 * (((v283 / v272) * v1007).tanh())));
                    v1014 = v1013;
                }
                let v1016 = (v568 * v56) * v949;
                let v1022 = (v1014 - (v968 - (v1016 * v1017))) / v970;
                let v1023 = if v1022 > v316 { 1.0 } else { 0.0 };
                let v1033: f64;
                if v1023 != 0.0 {
                    let v1024 = v971 * v1022;
                    v1033 = v1024;
                } else {
                    let v1026 = if v1022 < v1025 { 1.0 } else { 0.0 };
                    let v1034: f64;
                    if v1026 != 0.0 {
                        let v1028 = v971 * (v1022.exp());
                        v1034 = v1028;
                    } else {
                        let v1032 = v971 * ((v5 + (v1022.exp())).ln());
                        v1034 = v1032;
                    }
                    v1033 = v1034;
                }
                let v1045 = v934 * ((v5 + (v533 * v3)) / (v5 + (v533 * v46)));
                let v1056 = (((v1045 * (v5 + ((v534 * v947) / v923))) / (v5 + ((v938 * v1033) / v210))) * v923) / (v935 / (v956 * (v5 + ((v937 * v1033) / v210))));
                let v1066 = (((v1056 * ((v5 + (((v324 * v1033) / v210) / v1056)).sqrt())) - v1056) * (v5 - v1017)) + (v970 * v1017);
                let v1067 = v496 / v1066;
                let v1081: f64;
                if v259 != 0.0 {
                    let v1068 = v0 - v1067;
                    let v1073 = v273 * (v1067 + (((v1068 * v1068) + v272).sqrt()));
                    v1081 = v1073;
                } else {
                    let v1074 = v0 - v1067;
                    let v1080 = v273 * (v1067 + (v1074 * (((v283 / v272) * v1074).tanh())));
                    v1081 = v1080;
                }
                let v1084 = v5 / v936;
                let v1087 = v496 * (v5 / ((v5 + (v1081.powf(v936))).powf(v1084)));
                let v1088 = -v496;
                let v1089 = v1088 / v1066;
                let v1103: f64;
                if v259 != 0.0 {
                    let v1090 = v0 - v1089;
                    let v1095 = v273 * (v1089 + (((v1090 * v1090) + v272).sqrt()));
                    v1103 = v1095;
                } else {
                    let v1096 = v0 - v1089;
                    let v1102 = v273 * (v1089 + (v1096 * (((v283 / v272) * v1096).tanh())));
                    v1103 = v1102;
                }
                let v1108 = v1088 * (v5 / ((v5 + (v1103.powf(v936))).powf(v1084)));
                let v1110 = (v925 - v974) / v949;
                let v1111 = if v1110 > v316 { 1.0 } else { 0.0 };
                let v1118: f64;
                if v1111 != 0.0 {
                    v1118 = v0;
                } else {
                    let v1113 = if v1110 < v1112 { 1.0 } else { 0.0 };
                    let v1119: f64;
                    if v1113 != 0.0 {
                        v1119 = v5;
                    } else {
                        let v1116 = v5 / (v5 + (v1110.exp()));
                        v1119 = v1116;
                    }
                    v1118 = v1119;
                }
                let v1123 = ((v948 - v1108) - (v968 - (v1016 * v1118))) / v970;
                let v1124 = if v1123 > v316 { 1.0 } else { 0.0 };
                if v1124 != 0.0 {
                } else {
                    let v1126 = if v1123 < v1125 { 1.0 } else { 0.0 };
                    if v1126 != 0.0 {
                    } else {
                    }
                }
                let v1128 = (v948 - v974) / v949;
                let v1129 = if v1128 > v316 { 1.0 } else { 0.0 };
                let v1136: f64;
                if v1129 != 0.0 {
                    v1136 = v0;
                } else {
                    let v1131 = if v1128 < v1130 { 1.0 } else { 0.0 };
                    let v1137: f64;
                    if v1131 != 0.0 {
                        v1137 = v5;
                    } else {
                        let v1134 = v5 / (v5 + (v1128.exp()));
                        v1137 = v1134;
                    }
                    v1136 = v1137;
                }
                let v1141 = ((v925 - v1087) - (v968 - (v1016 * v1136))) / v970;
                let v1142 = if v1141 > v316 { 1.0 } else { 0.0 };
                if v1142 != 0.0 {
                } else {
                    let v1144 = if v1141 < v1143 { 1.0 } else { 0.0 };
                    if v1144 != 0.0 {
                    } else {
                    }
                }
                if v259 != 0.0 {
                } else {
                }
                let v1146 = (v324 * v951) * v89;
                let v1147 = v210 * v1146;
                let v1148 = v955 - v973;
                let v1164: f64;
                if v259 != 0.0 {
                    let v1150 = v925 - v948;
                    let v1155 = v273 * ((v925 + v948) + (((v1150 * v1150) + v272).sqrt()));
                    v1164 = v1155;
                } else {
                    let v1157 = v925 - v948;
                    let v1163 = v273 * ((v925 + v948) + (v1157 * (((v283 / v272) * v1157).tanh())));
                    v1164 = v1163;
                }
                let v1166 = (v1164 - v1148) / v949;
                let v1167 = if v1166 > v316 { 1.0 } else { 0.0 };
                let v1189: f64;
                if v1167 != 0.0 {
                    v1189 = v0;
                } else {
                    let v1169 = if v1166 < v1168 { 1.0 } else { 0.0 };
                    let v1190: f64;
                    if v1169 != 0.0 {
                        v1190 = v5;
                    } else {
                        let v1172 = v5 / (v5 + (v1166.exp()));
                        v1190 = v1172;
                    }
                    v1189 = v1190;
                }
                let v1188: f64;
                if v259 != 0.0 {
                    let v1174 = v925 - v948;
                    let v1179 = v273 * ((v925 + v948) + (((v1174 * v1174) + v272).sqrt()));
                    v1188 = v1179;
                } else {
                    let v1181 = v925 - v948;
                    let v1187 = v273 * ((v925 + v948) + (v1181 * (((v283 / v272) * v1181).tanh())));
                    v1188 = v1187;
                }
                let v1194 = (v1188 - (v955 - (v1016 * v1189))) / v1146;
                let v1195 = if v1194 > v316 { 1.0 } else { 0.0 };
                let v1208: f64;
                if v1195 != 0.0 {
                    let v1196 = v1147 * v1194;
                    v1208 = v1196;
                } else {
                    let v1198 = if v1194 < v1197 { 1.0 } else { 0.0 };
                    let v1209: f64;
                    if v1198 != 0.0 {
                        let v1200 = v1147 * (v1194.exp());
                        v1209 = v1200;
                    } else {
                        let v1204 = v1147 * ((v5 + (v1194.exp())).ln());
                        v1209 = v1204;
                    }
                    v1208 = v1209;
                }
                let v1207 = (v1045 * v923) / (v935 / v956);
                let v1220 = (((v1207 * ((v5 + (((v324 * v1208) / v210) / v1207)).sqrt())) - v1207) * (v5 - v1189)) + (v1146 * v1189);
                let v1221 = v496 / v1220;
                let v1235: f64;
                if v259 != 0.0 {
                    let v1222 = v0 - v1221;
                    let v1227 = v273 * (v1221 + (((v1222 * v1222) + v272).sqrt()));
                    v1235 = v1227;
                } else {
                    let v1228 = v0 - v1221;
                    let v1234 = v273 * (v1221 + (v1228 * (((v283 / v272) * v1228).tanh())));
                    v1235 = v1234;
                }
                let v1240 = v496 * (v5 / ((v5 + (v1235.powf(v936))).powf(v1084)));
                let v1241 = v1088 / v1220;
                let v1255: f64;
                if v259 != 0.0 {
                    let v1242 = v0 - v1241;
                    let v1247 = v273 * (v1241 + (((v1242 * v1242) + v272).sqrt()));
                    v1255 = v1247;
                } else {
                    let v1248 = v0 - v1241;
                    let v1254 = v273 * (v1241 + (v1248 * (((v283 / v272) * v1248).tanh())));
                    v1255 = v1254;
                }
                let v1260 = v1088 * (v5 / ((v5 + (v1255.powf(v936))).powf(v1084)));
                let v1262 = (v925 - v1148) / v949;
                let v1263 = if v1262 > v316 { 1.0 } else { 0.0 };
                let v1270: f64;
                if v1263 != 0.0 {
                    v1270 = v0;
                } else {
                    let v1265 = if v1262 < v1264 { 1.0 } else { 0.0 };
                    let v1271: f64;
                    if v1265 != 0.0 {
                        v1271 = v5;
                    } else {
                        let v1268 = v5 / (v5 + (v1262.exp()));
                        v1271 = v1268;
                    }
                    v1270 = v1271;
                }
                let v1275 = ((v948 - v1260) - (v955 - (v1016 * v1270))) / v1146;
                let v1276 = if v1275 > v316 { 1.0 } else { 0.0 };
                if v1276 != 0.0 {
                } else {
                    let v1278 = if v1275 < v1277 { 1.0 } else { 0.0 };
                    if v1278 != 0.0 {
                    } else {
                    }
                }
                let v1280 = (v948 - v1148) / v949;
                let v1281 = if v1280 > v316 { 1.0 } else { 0.0 };
                let v1288: f64;
                if v1281 != 0.0 {
                    v1288 = v0;
                } else {
                    let v1283 = if v1280 < v1282 { 1.0 } else { 0.0 };
                    let v1289: f64;
                    if v1283 != 0.0 {
                        v1289 = v5;
                    } else {
                        let v1286 = v5 / (v5 + (v1280.exp()));
                        v1289 = v1286;
                    }
                    v1288 = v1289;
                }
                let v1293 = ((v925 - v1240) - (v955 - (v1016 * v1288))) / v1146;
                let v1294 = if v1293 > v316 { 1.0 } else { 0.0 };
                if v1294 != 0.0 {
                } else {
                    let v1296 = if v1293 < v1295 { 1.0 } else { 0.0 };
                    if v1296 != 0.0 {
                    } else {
                    }
                }
                let v1297 = if v926 == v5 { 1.0 } else { 0.0 };
                if v1297 != 0.0 {
                    let v1300 = v955 - ((v568 * v273) * v949);
                    let v1302 = (v927 - v1300) / v1146;
                    let v1303 = if v1302 > v316 { 1.0 } else { 0.0 };
                    if v1303 != 0.0 {
                    } else {
                        let v1305 = if v1302 < v1304 { 1.0 } else { 0.0 };
                        if v1305 != 0.0 {
                        } else {
                        }
                    }
                    let v1307 = (v498 - v1300) / v1146;
                    let v1308 = if v1307 > v316 { 1.0 } else { 0.0 };
                    if v1308 != 0.0 {
                    } else {
                        let v1310 = if v1307 < v1309 { 1.0 } else { 0.0 };
                        if v1310 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v1311 = if v928 == v5 { 1.0 } else { 0.0 };
                if v1311 != 0.0 {
                    let v1316 = (v925 - (v955 - ((v568 * v273) * v949))) / v1146;
                    let v1317 = if v1316 > v316 { 1.0 } else { 0.0 };
                    if v1317 != 0.0 {
                    } else {
                        let v1319 = if v1316 < v1318 { 1.0 } else { 0.0 };
                        if v1319 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if v485 != 0.0 {
            } else {
            }
            let v1321 = if v1320 > v514 { 1.0 } else { 0.0 };
            if v1321 != 0.0 {
                let v1344: f64;
                if v259 != 0.0 {
                    let v1339 = ((v481 * v481) + v272).sqrt();
                    v1344 = v1339;
                } else {
                    let v1343 = v481 * (((v283 / v272) * v481).tanh());
                    v1344 = v1343;
                }
                let v1345 = v1322 - v481;
                let v1346 = v1330 * v89;
                let v1348 = v1327 / (v545 * v89);
                let v1350 = v1348 + (v1329 * v1344);
                let v1352 = v1326 + (v1336 * v91);
                let v1353 = v95.powf(v532);
                let v1354 = if v531 != v0 { 1.0 } else { 0.0 };
                let v1361: f64;
                if v1354 != 0.0 {
                    let v1360 = v1344 / ((v5 + ((v1344 / v531).powf(v1333))).powf((v5 / v1333)));
                    v1361 = v1360;
                } else {
                    v1361 = v0;
                }
                let v1365 = v1352 - ((v1328 - (v1361 * v0)) * v1344);
                let v1367 = (v324 * v1350) * v89;
                let v1368 = v203 * v1367;
                let v1370 = (v568 * v1346) / v324;
                let v1371 = v1365 - v1370;
                let v1387: f64;
                if v259 != 0.0 {
                    let v1373 = v1322 - v1345;
                    let v1378 = v273 * ((v1322 + v1345) + (((v1373 * v1373) + v272).sqrt()));
                    v1387 = v1378;
                } else {
                    let v1380 = v1322 - v1345;
                    let v1386 = v273 * ((v1322 + v1345) + (v1380 * (((v283 / v272) * v1380).tanh())));
                    v1387 = v1386;
                }
                let v1389 = (v1387 - v1371) / v1346;
                let v1390 = if v1389 > v316 { 1.0 } else { 0.0 };
                let v1414: f64;
                if v1390 != 0.0 {
                    v1414 = v0;
                } else {
                    let v1392 = if v1389 < v1391 { 1.0 } else { 0.0 };
                    let v1415: f64;
                    if v1392 != 0.0 {
                        v1415 = v5;
                    } else {
                        let v1395 = v5 / (v5 + (v1389.exp()));
                        v1415 = v1395;
                    }
                    v1414 = v1415;
                }
                let v1411: f64;
                if v259 != 0.0 {
                    let v1397 = v1322 - v1345;
                    let v1402 = v273 * ((v1322 + v1345) + (((v1397 * v1397) + v272).sqrt()));
                    v1411 = v1402;
                } else {
                    let v1404 = v1322 - v1345;
                    let v1410 = v273 * ((v1322 + v1345) + (v1404 * (((v283 / v272) * v1404).tanh())));
                    v1411 = v1410;
                }
                let v1413 = (v568 * v56) * v1346;
                let v1419 = (v1411 - (v1365 - (v1413 * v1414))) / v1367;
                let v1420 = if v1419 > v316 { 1.0 } else { 0.0 };
                let v1430: f64;
                if v1420 != 0.0 {
                    let v1421 = v1368 * v1419;
                    v1430 = v1421;
                } else {
                    let v1423 = if v1419 < v1422 { 1.0 } else { 0.0 };
                    let v1431: f64;
                    if v1423 != 0.0 {
                        let v1425 = v1368 * (v1419.exp());
                        v1431 = v1425;
                    } else {
                        let v1429 = v1368 * ((v5 + (v1419.exp())).ln());
                        v1431 = v1429;
                    }
                    v1430 = v1431;
                }
                let v1442 = v1331 * ((v5 + (v533 * v3)) / (v5 + (v533 * v46)));
                let v1453 = (((v1442 * (v5 + ((v534 * v1344) / v1320))) / (v5 + ((v1335 * v1430) / v203))) * v1320) / (v1332 / (v1353 * (v5 + ((v1334 * v1430) / v203))));
                let v1463 = (((v1453 * ((v5 + (((v324 * v1430) / v203) / v1453)).sqrt())) - v1453) * (v5 - v1414)) + (v1367 * v1414);
                let v1464 = v481 / v1463;
                let v1478: f64;
                if v259 != 0.0 {
                    let v1465 = v0 - v1464;
                    let v1470 = v273 * (v1464 + (((v1465 * v1465) + v272).sqrt()));
                    v1478 = v1470;
                } else {
                    let v1471 = v0 - v1464;
                    let v1477 = v273 * (v1464 + (v1471 * (((v283 / v272) * v1471).tanh())));
                    v1478 = v1477;
                }
                let v1481 = v5 / v1333;
                let v1484 = v481 * (v5 / ((v5 + (v1478.powf(v1333))).powf(v1481)));
                let v1485 = -v481;
                let v1486 = v1485 / v1463;
                let v1500: f64;
                if v259 != 0.0 {
                    let v1487 = v0 - v1486;
                    let v1492 = v273 * (v1486 + (((v1487 * v1487) + v272).sqrt()));
                    v1500 = v1492;
                } else {
                    let v1493 = v0 - v1486;
                    let v1499 = v273 * (v1486 + (v1493 * (((v283 / v272) * v1493).tanh())));
                    v1500 = v1499;
                }
                let v1505 = v1485 * (v5 / ((v5 + (v1500.powf(v1333))).powf(v1481)));
                let v1507 = (v1322 - v1371) / v1346;
                let v1508 = if v1507 > v316 { 1.0 } else { 0.0 };
                let v1515: f64;
                if v1508 != 0.0 {
                    v1515 = v0;
                } else {
                    let v1510 = if v1507 < v1509 { 1.0 } else { 0.0 };
                    let v1516: f64;
                    if v1510 != 0.0 {
                        v1516 = v5;
                    } else {
                        let v1513 = v5 / (v5 + (v1507.exp()));
                        v1516 = v1513;
                    }
                    v1515 = v1516;
                }
                let v1520 = ((v1345 - v1505) - (v1365 - (v1413 * v1515))) / v1367;
                let v1521 = if v1520 > v316 { 1.0 } else { 0.0 };
                if v1521 != 0.0 {
                } else {
                    let v1523 = if v1520 < v1522 { 1.0 } else { 0.0 };
                    if v1523 != 0.0 {
                    } else {
                    }
                }
                let v1525 = (v1345 - v1371) / v1346;
                let v1526 = if v1525 > v316 { 1.0 } else { 0.0 };
                let v1533: f64;
                if v1526 != 0.0 {
                    v1533 = v0;
                } else {
                    let v1528 = if v1525 < v1527 { 1.0 } else { 0.0 };
                    let v1534: f64;
                    if v1528 != 0.0 {
                        v1534 = v5;
                    } else {
                        let v1531 = v5 / (v5 + (v1525.exp()));
                        v1534 = v1531;
                    }
                    v1533 = v1534;
                }
                let v1538 = ((v1322 - v1484) - (v1365 - (v1413 * v1533))) / v1367;
                let v1539 = if v1538 > v316 { 1.0 } else { 0.0 };
                if v1539 != 0.0 {
                } else {
                    let v1541 = if v1538 < v1540 { 1.0 } else { 0.0 };
                    if v1541 != 0.0 {
                    } else {
                    }
                }
                if v259 != 0.0 {
                } else {
                }
                let v1543 = (v324 * v1348) * v89;
                let v1544 = v203 * v1543;
                let v1545 = v1352 - v1370;
                let v1561: f64;
                if v259 != 0.0 {
                    let v1547 = v1322 - v1345;
                    let v1552 = v273 * ((v1322 + v1345) + (((v1547 * v1547) + v272).sqrt()));
                    v1561 = v1552;
                } else {
                    let v1554 = v1322 - v1345;
                    let v1560 = v273 * ((v1322 + v1345) + (v1554 * (((v283 / v272) * v1554).tanh())));
                    v1561 = v1560;
                }
                let v1563 = (v1561 - v1545) / v1346;
                let v1564 = if v1563 > v316 { 1.0 } else { 0.0 };
                let v1586: f64;
                if v1564 != 0.0 {
                    v1586 = v0;
                } else {
                    let v1566 = if v1563 < v1565 { 1.0 } else { 0.0 };
                    let v1587: f64;
                    if v1566 != 0.0 {
                        v1587 = v5;
                    } else {
                        let v1569 = v5 / (v5 + (v1563.exp()));
                        v1587 = v1569;
                    }
                    v1586 = v1587;
                }
                let v1585: f64;
                if v259 != 0.0 {
                    let v1571 = v1322 - v1345;
                    let v1576 = v273 * ((v1322 + v1345) + (((v1571 * v1571) + v272).sqrt()));
                    v1585 = v1576;
                } else {
                    let v1578 = v1322 - v1345;
                    let v1584 = v273 * ((v1322 + v1345) + (v1578 * (((v283 / v272) * v1578).tanh())));
                    v1585 = v1584;
                }
                let v1591 = (v1585 - (v1352 - (v1413 * v1586))) / v1543;
                let v1592 = if v1591 > v316 { 1.0 } else { 0.0 };
                let v1605: f64;
                if v1592 != 0.0 {
                    let v1593 = v1544 * v1591;
                    v1605 = v1593;
                } else {
                    let v1595 = if v1591 < v1594 { 1.0 } else { 0.0 };
                    let v1606: f64;
                    if v1595 != 0.0 {
                        let v1597 = v1544 * (v1591.exp());
                        v1606 = v1597;
                    } else {
                        let v1601 = v1544 * ((v5 + (v1591.exp())).ln());
                        v1606 = v1601;
                    }
                    v1605 = v1606;
                }
                let v1604 = (v1442 * v1320) / (v1332 / v1353);
                let v1617 = (((v1604 * ((v5 + (((v324 * v1605) / v203) / v1604)).sqrt())) - v1604) * (v5 - v1586)) + (v1543 * v1586);
                let v1618 = v481 / v1617;
                let v1632: f64;
                if v259 != 0.0 {
                    let v1619 = v0 - v1618;
                    let v1624 = v273 * (v1618 + (((v1619 * v1619) + v272).sqrt()));
                    v1632 = v1624;
                } else {
                    let v1625 = v0 - v1618;
                    let v1631 = v273 * (v1618 + (v1625 * (((v283 / v272) * v1625).tanh())));
                    v1632 = v1631;
                }
                let v1637 = v481 * (v5 / ((v5 + (v1632.powf(v1333))).powf(v1481)));
                let v1638 = v1485 / v1617;
                let v1652: f64;
                if v259 != 0.0 {
                    let v1639 = v0 - v1638;
                    let v1644 = v273 * (v1638 + (((v1639 * v1639) + v272).sqrt()));
                    v1652 = v1644;
                } else {
                    let v1645 = v0 - v1638;
                    let v1651 = v273 * (v1638 + (v1645 * (((v283 / v272) * v1645).tanh())));
                    v1652 = v1651;
                }
                let v1657 = v1485 * (v5 / ((v5 + (v1652.powf(v1333))).powf(v1481)));
                let v1659 = (v1322 - v1545) / v1346;
                let v1660 = if v1659 > v316 { 1.0 } else { 0.0 };
                let v1667: f64;
                if v1660 != 0.0 {
                    v1667 = v0;
                } else {
                    let v1662 = if v1659 < v1661 { 1.0 } else { 0.0 };
                    let v1668: f64;
                    if v1662 != 0.0 {
                        v1668 = v5;
                    } else {
                        let v1665 = v5 / (v5 + (v1659.exp()));
                        v1668 = v1665;
                    }
                    v1667 = v1668;
                }
                let v1672 = ((v1345 - v1657) - (v1352 - (v1413 * v1667))) / v1543;
                let v1673 = if v1672 > v316 { 1.0 } else { 0.0 };
                if v1673 != 0.0 {
                } else {
                    let v1675 = if v1672 < v1674 { 1.0 } else { 0.0 };
                    if v1675 != 0.0 {
                    } else {
                    }
                }
                let v1677 = (v1345 - v1545) / v1346;
                let v1678 = if v1677 > v316 { 1.0 } else { 0.0 };
                let v1685: f64;
                if v1678 != 0.0 {
                    v1685 = v0;
                } else {
                    let v1680 = if v1677 < v1679 { 1.0 } else { 0.0 };
                    let v1686: f64;
                    if v1680 != 0.0 {
                        v1686 = v5;
                    } else {
                        let v1683 = v5 / (v5 + (v1677.exp()));
                        v1686 = v1683;
                    }
                    v1685 = v1686;
                }
                let v1690 = ((v1322 - v1637) - (v1352 - (v1413 * v1685))) / v1543;
                let v1691 = if v1690 > v316 { 1.0 } else { 0.0 };
                if v1691 != 0.0 {
                } else {
                    let v1693 = if v1690 < v1692 { 1.0 } else { 0.0 };
                    if v1693 != 0.0 {
                    } else {
                    }
                }
                let v1694 = if v1323 == v5 { 1.0 } else { 0.0 };
                if v1694 != 0.0 {
                    let v1697 = v1352 - ((v568 * v273) * v1346);
                    let v1699 = (v1324 - v1697) / v1543;
                    let v1700 = if v1699 > v316 { 1.0 } else { 0.0 };
                    if v1700 != 0.0 {
                    } else {
                        let v1702 = if v1699 < v1701 { 1.0 } else { 0.0 };
                        if v1702 != 0.0 {
                        } else {
                        }
                    }
                    let v1704 = (v483 - v1697) / v1543;
                    let v1705 = if v1704 > v316 { 1.0 } else { 0.0 };
                    if v1705 != 0.0 {
                    } else {
                        let v1707 = if v1704 < v1706 { 1.0 } else { 0.0 };
                        if v1707 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v1708 = if v1325 == v5 { 1.0 } else { 0.0 };
                if v1708 != 0.0 {
                    let v1713 = (v1322 - (v1352 - ((v568 * v273) * v1346))) / v1543;
                    let v1714 = if v1713 > v316 { 1.0 } else { 0.0 };
                    if v1714 != 0.0 {
                    } else {
                        let v1716 = if v1713 < v1715 { 1.0 } else { 0.0 };
                        if v1716 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if v470 != 0.0 {
            } else {
            }
            let v1718 = if v1717 > v514 { 1.0 } else { 0.0 };
            if v1718 != 0.0 {
                let v1741: f64;
                if v259 != 0.0 {
                    let v1736 = ((v466 * v466) + v272).sqrt();
                    v1741 = v1736;
                } else {
                    let v1740 = v466 * (((v283 / v272) * v466).tanh());
                    v1741 = v1740;
                }
                let v1742 = v1719 - v466;
                let v1743 = v1727 * v89;
                let v1745 = v1724 / (v545 * v89);
                let v1747 = v1745 + (v1726 * v1741);
                let v1749 = v1723 + (v1733 * v91);
                let v1750 = v95.powf(v532);
                let v1751 = if v531 != v0 { 1.0 } else { 0.0 };
                let v1758: f64;
                if v1751 != 0.0 {
                    let v1757 = v1741 / ((v5 + ((v1741 / v531).powf(v1730))).powf((v5 / v1730)));
                    v1758 = v1757;
                } else {
                    v1758 = v0;
                }
                let v1762 = v1749 - ((v1725 - (v1758 * v0)) * v1741);
                let v1764 = (v324 * v1747) * v89;
                let v1765 = v196 * v1764;
                let v1767 = (v568 * v1743) / v324;
                let v1768 = v1762 - v1767;
                let v1784: f64;
                if v259 != 0.0 {
                    let v1770 = v1719 - v1742;
                    let v1775 = v273 * ((v1719 + v1742) + (((v1770 * v1770) + v272).sqrt()));
                    v1784 = v1775;
                } else {
                    let v1777 = v1719 - v1742;
                    let v1783 = v273 * ((v1719 + v1742) + (v1777 * (((v283 / v272) * v1777).tanh())));
                    v1784 = v1783;
                }
                let v1786 = (v1784 - v1768) / v1743;
                let v1787 = if v1786 > v316 { 1.0 } else { 0.0 };
                let v1811: f64;
                if v1787 != 0.0 {
                    v1811 = v0;
                } else {
                    let v1789 = if v1786 < v1788 { 1.0 } else { 0.0 };
                    let v1812: f64;
                    if v1789 != 0.0 {
                        v1812 = v5;
                    } else {
                        let v1792 = v5 / (v5 + (v1786.exp()));
                        v1812 = v1792;
                    }
                    v1811 = v1812;
                }
                let v1808: f64;
                if v259 != 0.0 {
                    let v1794 = v1719 - v1742;
                    let v1799 = v273 * ((v1719 + v1742) + (((v1794 * v1794) + v272).sqrt()));
                    v1808 = v1799;
                } else {
                    let v1801 = v1719 - v1742;
                    let v1807 = v273 * ((v1719 + v1742) + (v1801 * (((v283 / v272) * v1801).tanh())));
                    v1808 = v1807;
                }
                let v1810 = (v568 * v56) * v1743;
                let v1816 = (v1808 - (v1762 - (v1810 * v1811))) / v1764;
                let v1817 = if v1816 > v316 { 1.0 } else { 0.0 };
                let v1827: f64;
                if v1817 != 0.0 {
                    let v1818 = v1765 * v1816;
                    v1827 = v1818;
                } else {
                    let v1820 = if v1816 < v1819 { 1.0 } else { 0.0 };
                    let v1828: f64;
                    if v1820 != 0.0 {
                        let v1822 = v1765 * (v1816.exp());
                        v1828 = v1822;
                    } else {
                        let v1826 = v1765 * ((v5 + (v1816.exp())).ln());
                        v1828 = v1826;
                    }
                    v1827 = v1828;
                }
                let v1839 = v1728 * ((v5 + (v533 * v3)) / (v5 + (v533 * v46)));
                let v1850 = (((v1839 * (v5 + ((v534 * v1741) / v1717))) / (v5 + ((v1732 * v1827) / v196))) * v1717) / (v1729 / (v1750 * (v5 + ((v1731 * v1827) / v196))));
                let v1860 = (((v1850 * ((v5 + (((v324 * v1827) / v196) / v1850)).sqrt())) - v1850) * (v5 - v1811)) + (v1764 * v1811);
                let v1861 = v466 / v1860;
                let v1875: f64;
                if v259 != 0.0 {
                    let v1862 = v0 - v1861;
                    let v1867 = v273 * (v1861 + (((v1862 * v1862) + v272).sqrt()));
                    v1875 = v1867;
                } else {
                    let v1868 = v0 - v1861;
                    let v1874 = v273 * (v1861 + (v1868 * (((v283 / v272) * v1868).tanh())));
                    v1875 = v1874;
                }
                let v1878 = v5 / v1730;
                let v1881 = v466 * (v5 / ((v5 + (v1875.powf(v1730))).powf(v1878)));
                let v1882 = -v466;
                let v1883 = v1882 / v1860;
                let v1897: f64;
                if v259 != 0.0 {
                    let v1884 = v0 - v1883;
                    let v1889 = v273 * (v1883 + (((v1884 * v1884) + v272).sqrt()));
                    v1897 = v1889;
                } else {
                    let v1890 = v0 - v1883;
                    let v1896 = v273 * (v1883 + (v1890 * (((v283 / v272) * v1890).tanh())));
                    v1897 = v1896;
                }
                let v1902 = v1882 * (v5 / ((v5 + (v1897.powf(v1730))).powf(v1878)));
                let v1904 = (v1719 - v1768) / v1743;
                let v1905 = if v1904 > v316 { 1.0 } else { 0.0 };
                let v1912: f64;
                if v1905 != 0.0 {
                    v1912 = v0;
                } else {
                    let v1907 = if v1904 < v1906 { 1.0 } else { 0.0 };
                    let v1913: f64;
                    if v1907 != 0.0 {
                        v1913 = v5;
                    } else {
                        let v1910 = v5 / (v5 + (v1904.exp()));
                        v1913 = v1910;
                    }
                    v1912 = v1913;
                }
                let v1917 = ((v1742 - v1902) - (v1762 - (v1810 * v1912))) / v1764;
                let v1918 = if v1917 > v316 { 1.0 } else { 0.0 };
                if v1918 != 0.0 {
                } else {
                    let v1920 = if v1917 < v1919 { 1.0 } else { 0.0 };
                    if v1920 != 0.0 {
                    } else {
                    }
                }
                let v1922 = (v1742 - v1768) / v1743;
                let v1923 = if v1922 > v316 { 1.0 } else { 0.0 };
                let v1930: f64;
                if v1923 != 0.0 {
                    v1930 = v0;
                } else {
                    let v1925 = if v1922 < v1924 { 1.0 } else { 0.0 };
                    let v1931: f64;
                    if v1925 != 0.0 {
                        v1931 = v5;
                    } else {
                        let v1928 = v5 / (v5 + (v1922.exp()));
                        v1931 = v1928;
                    }
                    v1930 = v1931;
                }
                let v1935 = ((v1719 - v1881) - (v1762 - (v1810 * v1930))) / v1764;
                let v1936 = if v1935 > v316 { 1.0 } else { 0.0 };
                if v1936 != 0.0 {
                } else {
                    let v1938 = if v1935 < v1937 { 1.0 } else { 0.0 };
                    if v1938 != 0.0 {
                    } else {
                    }
                }
                if v259 != 0.0 {
                } else {
                }
                let v1940 = (v324 * v1745) * v89;
                let v1941 = v196 * v1940;
                let v1942 = v1749 - v1767;
                let v1958: f64;
                if v259 != 0.0 {
                    let v1944 = v1719 - v1742;
                    let v1949 = v273 * ((v1719 + v1742) + (((v1944 * v1944) + v272).sqrt()));
                    v1958 = v1949;
                } else {
                    let v1951 = v1719 - v1742;
                    let v1957 = v273 * ((v1719 + v1742) + (v1951 * (((v283 / v272) * v1951).tanh())));
                    v1958 = v1957;
                }
                let v1960 = (v1958 - v1942) / v1743;
                let v1961 = if v1960 > v316 { 1.0 } else { 0.0 };
                let v1983: f64;
                if v1961 != 0.0 {
                    v1983 = v0;
                } else {
                    let v1963 = if v1960 < v1962 { 1.0 } else { 0.0 };
                    let v1984: f64;
                    if v1963 != 0.0 {
                        v1984 = v5;
                    } else {
                        let v1966 = v5 / (v5 + (v1960.exp()));
                        v1984 = v1966;
                    }
                    v1983 = v1984;
                }
                let v1982: f64;
                if v259 != 0.0 {
                    let v1968 = v1719 - v1742;
                    let v1973 = v273 * ((v1719 + v1742) + (((v1968 * v1968) + v272).sqrt()));
                    v1982 = v1973;
                } else {
                    let v1975 = v1719 - v1742;
                    let v1981 = v273 * ((v1719 + v1742) + (v1975 * (((v283 / v272) * v1975).tanh())));
                    v1982 = v1981;
                }
                let v1988 = (v1982 - (v1749 - (v1810 * v1983))) / v1940;
                let v1989 = if v1988 > v316 { 1.0 } else { 0.0 };
                let v2002: f64;
                if v1989 != 0.0 {
                    let v1990 = v1941 * v1988;
                    v2002 = v1990;
                } else {
                    let v1992 = if v1988 < v1991 { 1.0 } else { 0.0 };
                    let v2003: f64;
                    if v1992 != 0.0 {
                        let v1994 = v1941 * (v1988.exp());
                        v2003 = v1994;
                    } else {
                        let v1998 = v1941 * ((v5 + (v1988.exp())).ln());
                        v2003 = v1998;
                    }
                    v2002 = v2003;
                }
                let v2001 = (v1839 * v1717) / (v1729 / v1750);
                let v2014 = (((v2001 * ((v5 + (((v324 * v2002) / v196) / v2001)).sqrt())) - v2001) * (v5 - v1983)) + (v1940 * v1983);
                let v2015 = v466 / v2014;
                let v2029: f64;
                if v259 != 0.0 {
                    let v2016 = v0 - v2015;
                    let v2021 = v273 * (v2015 + (((v2016 * v2016) + v272).sqrt()));
                    v2029 = v2021;
                } else {
                    let v2022 = v0 - v2015;
                    let v2028 = v273 * (v2015 + (v2022 * (((v283 / v272) * v2022).tanh())));
                    v2029 = v2028;
                }
                let v2034 = v466 * (v5 / ((v5 + (v2029.powf(v1730))).powf(v1878)));
                let v2035 = v1882 / v2014;
                let v2049: f64;
                if v259 != 0.0 {
                    let v2036 = v0 - v2035;
                    let v2041 = v273 * (v2035 + (((v2036 * v2036) + v272).sqrt()));
                    v2049 = v2041;
                } else {
                    let v2042 = v0 - v2035;
                    let v2048 = v273 * (v2035 + (v2042 * (((v283 / v272) * v2042).tanh())));
                    v2049 = v2048;
                }
                let v2054 = v1882 * (v5 / ((v5 + (v2049.powf(v1730))).powf(v1878)));
                let v2056 = (v1719 - v1942) / v1743;
                let v2057 = if v2056 > v316 { 1.0 } else { 0.0 };
                let v2064: f64;
                if v2057 != 0.0 {
                    v2064 = v0;
                } else {
                    let v2059 = if v2056 < v2058 { 1.0 } else { 0.0 };
                    let v2065: f64;
                    if v2059 != 0.0 {
                        v2065 = v5;
                    } else {
                        let v2062 = v5 / (v5 + (v2056.exp()));
                        v2065 = v2062;
                    }
                    v2064 = v2065;
                }
                let v2069 = ((v1742 - v2054) - (v1749 - (v1810 * v2064))) / v1940;
                let v2070 = if v2069 > v316 { 1.0 } else { 0.0 };
                if v2070 != 0.0 {
                } else {
                    let v2072 = if v2069 < v2071 { 1.0 } else { 0.0 };
                    if v2072 != 0.0 {
                    } else {
                    }
                }
                let v2074 = (v1742 - v1942) / v1743;
                let v2075 = if v2074 > v316 { 1.0 } else { 0.0 };
                let v2082: f64;
                if v2075 != 0.0 {
                    v2082 = v0;
                } else {
                    let v2077 = if v2074 < v2076 { 1.0 } else { 0.0 };
                    let v2083: f64;
                    if v2077 != 0.0 {
                        v2083 = v5;
                    } else {
                        let v2080 = v5 / (v5 + (v2074.exp()));
                        v2083 = v2080;
                    }
                    v2082 = v2083;
                }
                let v2087 = ((v1719 - v2034) - (v1749 - (v1810 * v2082))) / v1940;
                let v2088 = if v2087 > v316 { 1.0 } else { 0.0 };
                if v2088 != 0.0 {
                } else {
                    let v2090 = if v2087 < v2089 { 1.0 } else { 0.0 };
                    if v2090 != 0.0 {
                    } else {
                    }
                }
                let v2091 = if v1720 == v5 { 1.0 } else { 0.0 };
                if v2091 != 0.0 {
                    let v2094 = v1749 - ((v568 * v273) * v1743);
                    let v2096 = (v1721 - v2094) / v1940;
                    let v2097 = if v2096 > v316 { 1.0 } else { 0.0 };
                    if v2097 != 0.0 {
                    } else {
                        let v2099 = if v2096 < v2098 { 1.0 } else { 0.0 };
                        if v2099 != 0.0 {
                        } else {
                        }
                    }
                    let v2101 = (v468 - v2094) / v1940;
                    let v2102 = if v2101 > v316 { 1.0 } else { 0.0 };
                    if v2102 != 0.0 {
                    } else {
                        let v2104 = if v2101 < v2103 { 1.0 } else { 0.0 };
                        if v2104 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v2105 = if v1722 == v5 { 1.0 } else { 0.0 };
                if v2105 != 0.0 {
                    let v2110 = (v1719 - (v1749 - ((v568 * v273) * v1743))) / v1940;
                    let v2111 = if v2110 > v316 { 1.0 } else { 0.0 };
                    if v2111 != 0.0 {
                    } else {
                        let v2113 = if v2110 < v2112 { 1.0 } else { 0.0 };
                        if v2113 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if v455 != 0.0 {
            } else {
            }
            let v2115 = if v2114 > v514 { 1.0 } else { 0.0 };
            if v2115 != 0.0 {
                let v2138: f64;
                if v259 != 0.0 {
                    let v2133 = ((v406 * v406) + v272).sqrt();
                    v2138 = v2133;
                } else {
                    let v2137 = v406 * (((v283 / v272) * v406).tanh());
                    v2138 = v2137;
                }
                let v2139 = v2116 - v406;
                let v2140 = v2124 * v89;
                let v2142 = v2121 / (v545 * v89);
                let v2144 = v2142 + (v2123 * v2138);
                let v2146 = v2120 + (v2130 * v91);
                let v2147 = v95.powf(v532);
                let v2148 = if v531 != v0 { 1.0 } else { 0.0 };
                let v2155: f64;
                if v2148 != 0.0 {
                    let v2154 = v2138 / ((v5 + ((v2138 / v531).powf(v2127))).powf((v5 / v2127)));
                    v2155 = v2154;
                } else {
                    v2155 = v0;
                }
                let v2159 = v2146 - ((v2122 - (v2155 * v0)) * v2138);
                let v2161 = (v324 * v2144) * v89;
                let v2162 = v136 * v2161;
                let v2164 = (v568 * v2140) / v324;
                let v2165 = v2159 - v2164;
                let v2181: f64;
                if v259 != 0.0 {
                    let v2167 = v2116 - v2139;
                    let v2172 = v273 * ((v2116 + v2139) + (((v2167 * v2167) + v272).sqrt()));
                    v2181 = v2172;
                } else {
                    let v2174 = v2116 - v2139;
                    let v2180 = v273 * ((v2116 + v2139) + (v2174 * (((v283 / v272) * v2174).tanh())));
                    v2181 = v2180;
                }
                let v2183 = (v2181 - v2165) / v2140;
                let v2184 = if v2183 > v316 { 1.0 } else { 0.0 };
                let v2208: f64;
                if v2184 != 0.0 {
                    v2208 = v0;
                } else {
                    let v2186 = if v2183 < v2185 { 1.0 } else { 0.0 };
                    let v2209: f64;
                    if v2186 != 0.0 {
                        v2209 = v5;
                    } else {
                        let v2189 = v5 / (v5 + (v2183.exp()));
                        v2209 = v2189;
                    }
                    v2208 = v2209;
                }
                let v2205: f64;
                if v259 != 0.0 {
                    let v2191 = v2116 - v2139;
                    let v2196 = v273 * ((v2116 + v2139) + (((v2191 * v2191) + v272).sqrt()));
                    v2205 = v2196;
                } else {
                    let v2198 = v2116 - v2139;
                    let v2204 = v273 * ((v2116 + v2139) + (v2198 * (((v283 / v272) * v2198).tanh())));
                    v2205 = v2204;
                }
                let v2207 = (v568 * v56) * v2140;
                let v2213 = (v2205 - (v2159 - (v2207 * v2208))) / v2161;
                let v2214 = if v2213 > v316 { 1.0 } else { 0.0 };
                let v2224: f64;
                if v2214 != 0.0 {
                    let v2215 = v2162 * v2213;
                    v2224 = v2215;
                } else {
                    let v2217 = if v2213 < v2216 { 1.0 } else { 0.0 };
                    let v2225: f64;
                    if v2217 != 0.0 {
                        let v2219 = v2162 * (v2213.exp());
                        v2225 = v2219;
                    } else {
                        let v2223 = v2162 * ((v5 + (v2213.exp())).ln());
                        v2225 = v2223;
                    }
                    v2224 = v2225;
                }
                let v2236 = v2125 * ((v5 + (v533 * v3)) / (v5 + (v533 * v46)));
                let v2247 = (((v2236 * (v5 + ((v534 * v2138) / v2114))) / (v5 + ((v2129 * v2224) / v136))) * v2114) / (v2126 / (v2147 * (v5 + ((v2128 * v2224) / v136))));
                let v2257 = (((v2247 * ((v5 + (((v324 * v2224) / v136) / v2247)).sqrt())) - v2247) * (v5 - v2208)) + (v2161 * v2208);
                let v2258 = v406 / v2257;
                let v2272: f64;
                if v259 != 0.0 {
                    let v2259 = v0 - v2258;
                    let v2264 = v273 * (v2258 + (((v2259 * v2259) + v272).sqrt()));
                    v2272 = v2264;
                } else {
                    let v2265 = v0 - v2258;
                    let v2271 = v273 * (v2258 + (v2265 * (((v283 / v272) * v2265).tanh())));
                    v2272 = v2271;
                }
                let v2275 = v5 / v2127;
                let v2278 = v406 * (v5 / ((v5 + (v2272.powf(v2127))).powf(v2275)));
                let v2279 = -v406;
                let v2280 = v2279 / v2257;
                let v2294: f64;
                if v259 != 0.0 {
                    let v2281 = v0 - v2280;
                    let v2286 = v273 * (v2280 + (((v2281 * v2281) + v272).sqrt()));
                    v2294 = v2286;
                } else {
                    let v2287 = v0 - v2280;
                    let v2293 = v273 * (v2280 + (v2287 * (((v283 / v272) * v2287).tanh())));
                    v2294 = v2293;
                }
                let v2299 = v2279 * (v5 / ((v5 + (v2294.powf(v2127))).powf(v2275)));
                let v2301 = (v2116 - v2165) / v2140;
                let v2302 = if v2301 > v316 { 1.0 } else { 0.0 };
                let v2309: f64;
                if v2302 != 0.0 {
                    v2309 = v0;
                } else {
                    let v2304 = if v2301 < v2303 { 1.0 } else { 0.0 };
                    let v2310: f64;
                    if v2304 != 0.0 {
                        v2310 = v5;
                    } else {
                        let v2307 = v5 / (v5 + (v2301.exp()));
                        v2310 = v2307;
                    }
                    v2309 = v2310;
                }
                let v2314 = ((v2139 - v2299) - (v2159 - (v2207 * v2309))) / v2161;
                let v2315 = if v2314 > v316 { 1.0 } else { 0.0 };
                if v2315 != 0.0 {
                } else {
                    let v2317 = if v2314 < v2316 { 1.0 } else { 0.0 };
                    if v2317 != 0.0 {
                    } else {
                    }
                }
                let v2319 = (v2139 - v2165) / v2140;
                let v2320 = if v2319 > v316 { 1.0 } else { 0.0 };
                let v2327: f64;
                if v2320 != 0.0 {
                    v2327 = v0;
                } else {
                    let v2322 = if v2319 < v2321 { 1.0 } else { 0.0 };
                    let v2328: f64;
                    if v2322 != 0.0 {
                        v2328 = v5;
                    } else {
                        let v2325 = v5 / (v5 + (v2319.exp()));
                        v2328 = v2325;
                    }
                    v2327 = v2328;
                }
                let v2332 = ((v2116 - v2278) - (v2159 - (v2207 * v2327))) / v2161;
                let v2333 = if v2332 > v316 { 1.0 } else { 0.0 };
                if v2333 != 0.0 {
                } else {
                    let v2335 = if v2332 < v2334 { 1.0 } else { 0.0 };
                    if v2335 != 0.0 {
                    } else {
                    }
                }
                if v259 != 0.0 {
                } else {
                }
                let v2337 = (v324 * v2142) * v89;
                let v2338 = v136 * v2337;
                let v2339 = v2146 - v2164;
                let v2355: f64;
                if v259 != 0.0 {
                    let v2341 = v2116 - v2139;
                    let v2346 = v273 * ((v2116 + v2139) + (((v2341 * v2341) + v272).sqrt()));
                    v2355 = v2346;
                } else {
                    let v2348 = v2116 - v2139;
                    let v2354 = v273 * ((v2116 + v2139) + (v2348 * (((v283 / v272) * v2348).tanh())));
                    v2355 = v2354;
                }
                let v2357 = (v2355 - v2339) / v2140;
                let v2358 = if v2357 > v316 { 1.0 } else { 0.0 };
                let v2380: f64;
                if v2358 != 0.0 {
                    v2380 = v0;
                } else {
                    let v2360 = if v2357 < v2359 { 1.0 } else { 0.0 };
                    let v2381: f64;
                    if v2360 != 0.0 {
                        v2381 = v5;
                    } else {
                        let v2363 = v5 / (v5 + (v2357.exp()));
                        v2381 = v2363;
                    }
                    v2380 = v2381;
                }
                let v2379: f64;
                if v259 != 0.0 {
                    let v2365 = v2116 - v2139;
                    let v2370 = v273 * ((v2116 + v2139) + (((v2365 * v2365) + v272).sqrt()));
                    v2379 = v2370;
                } else {
                    let v2372 = v2116 - v2139;
                    let v2378 = v273 * ((v2116 + v2139) + (v2372 * (((v283 / v272) * v2372).tanh())));
                    v2379 = v2378;
                }
                let v2385 = (v2379 - (v2146 - (v2207 * v2380))) / v2337;
                let v2386 = if v2385 > v316 { 1.0 } else { 0.0 };
                let v2399: f64;
                if v2386 != 0.0 {
                    let v2387 = v2338 * v2385;
                    v2399 = v2387;
                } else {
                    let v2389 = if v2385 < v2388 { 1.0 } else { 0.0 };
                    let v2400: f64;
                    if v2389 != 0.0 {
                        let v2391 = v2338 * (v2385.exp());
                        v2400 = v2391;
                    } else {
                        let v2395 = v2338 * ((v5 + (v2385.exp())).ln());
                        v2400 = v2395;
                    }
                    v2399 = v2400;
                }
                let v2398 = (v2236 * v2114) / (v2126 / v2147);
                let v2411 = (((v2398 * ((v5 + (((v324 * v2399) / v136) / v2398)).sqrt())) - v2398) * (v5 - v2380)) + (v2337 * v2380);
                let v2412 = v406 / v2411;
                let v2426: f64;
                if v259 != 0.0 {
                    let v2413 = v0 - v2412;
                    let v2418 = v273 * (v2412 + (((v2413 * v2413) + v272).sqrt()));
                    v2426 = v2418;
                } else {
                    let v2419 = v0 - v2412;
                    let v2425 = v273 * (v2412 + (v2419 * (((v283 / v272) * v2419).tanh())));
                    v2426 = v2425;
                }
                let v2431 = v406 * (v5 / ((v5 + (v2426.powf(v2127))).powf(v2275)));
                let v2432 = v2279 / v2411;
                let v2446: f64;
                if v259 != 0.0 {
                    let v2433 = v0 - v2432;
                    let v2438 = v273 * (v2432 + (((v2433 * v2433) + v272).sqrt()));
                    v2446 = v2438;
                } else {
                    let v2439 = v0 - v2432;
                    let v2445 = v273 * (v2432 + (v2439 * (((v283 / v272) * v2439).tanh())));
                    v2446 = v2445;
                }
                let v2451 = v2279 * (v5 / ((v5 + (v2446.powf(v2127))).powf(v2275)));
                let v2453 = (v2116 - v2339) / v2140;
                let v2454 = if v2453 > v316 { 1.0 } else { 0.0 };
                let v2461: f64;
                if v2454 != 0.0 {
                    v2461 = v0;
                } else {
                    let v2456 = if v2453 < v2455 { 1.0 } else { 0.0 };
                    let v2462: f64;
                    if v2456 != 0.0 {
                        v2462 = v5;
                    } else {
                        let v2459 = v5 / (v5 + (v2453.exp()));
                        v2462 = v2459;
                    }
                    v2461 = v2462;
                }
                let v2466 = ((v2139 - v2451) - (v2146 - (v2207 * v2461))) / v2337;
                let v2467 = if v2466 > v316 { 1.0 } else { 0.0 };
                if v2467 != 0.0 {
                } else {
                    let v2469 = if v2466 < v2468 { 1.0 } else { 0.0 };
                    if v2469 != 0.0 {
                    } else {
                    }
                }
                let v2471 = (v2139 - v2339) / v2140;
                let v2472 = if v2471 > v316 { 1.0 } else { 0.0 };
                let v2479: f64;
                if v2472 != 0.0 {
                    v2479 = v0;
                } else {
                    let v2474 = if v2471 < v2473 { 1.0 } else { 0.0 };
                    let v2480: f64;
                    if v2474 != 0.0 {
                        v2480 = v5;
                    } else {
                        let v2477 = v5 / (v5 + (v2471.exp()));
                        v2480 = v2477;
                    }
                    v2479 = v2480;
                }
                let v2484 = ((v2116 - v2431) - (v2146 - (v2207 * v2479))) / v2337;
                let v2485 = if v2484 > v316 { 1.0 } else { 0.0 };
                if v2485 != 0.0 {
                } else {
                    let v2487 = if v2484 < v2486 { 1.0 } else { 0.0 };
                    if v2487 != 0.0 {
                    } else {
                    }
                }
                let v2488 = if v2117 == v5 { 1.0 } else { 0.0 };
                if v2488 != 0.0 {
                    let v2491 = v2146 - ((v568 * v273) * v2140);
                    let v2493 = (v2118 - v2491) / v2337;
                    let v2494 = if v2493 > v316 { 1.0 } else { 0.0 };
                    if v2494 != 0.0 {
                    } else {
                        let v2496 = if v2493 < v2495 { 1.0 } else { 0.0 };
                        if v2496 != 0.0 {
                        } else {
                        }
                    }
                    let v2498 = (v409 - v2491) / v2337;
                    let v2499 = if v2498 > v316 { 1.0 } else { 0.0 };
                    if v2499 != 0.0 {
                    } else {
                        let v2501 = if v2498 < v2500 { 1.0 } else { 0.0 };
                        if v2501 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v2502 = if v2119 == v5 { 1.0 } else { 0.0 };
                if v2502 != 0.0 {
                    let v2507 = (v2116 - (v2146 - ((v568 * v273) * v2140))) / v2337;
                    let v2508 = if v2507 > v316 { 1.0 } else { 0.0 };
                    if v2508 != 0.0 {
                    } else {
                        let v2510 = if v2507 < v2509 { 1.0 } else { 0.0 };
                        if v2510 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if v394 != 0.0 {
            } else {
            }
            let v2512 = if v2511 > v514 { 1.0 } else { 0.0 };
            if v2512 != 0.0 {
                let v2535: f64;
                if v259 != 0.0 {
                    let v2530 = ((v422 * v422) + v272).sqrt();
                    v2535 = v2530;
                } else {
                    let v2534 = v422 * (((v283 / v272) * v422).tanh());
                    v2535 = v2534;
                }
                let v2536 = v2513 - v422;
                let v2537 = v2521 * v89;
                let v2539 = v2518 / (v545 * v89);
                let v2541 = v2539 + (v2520 * v2535);
                let v2543 = v2517 + (v2527 * v91);
                let v2544 = v95.powf(v532);
                let v2545 = if v531 != v0 { 1.0 } else { 0.0 };
                let v2552: f64;
                if v2545 != 0.0 {
                    let v2551 = v2535 / ((v5 + ((v2535 / v531).powf(v2524))).powf((v5 / v2524)));
                    v2552 = v2551;
                } else {
                    v2552 = v0;
                }
                let v2556 = v2543 - ((v2519 - (v2552 * v0)) * v2535);
                let v2558 = (v324 * v2541) * v89;
                let v2559 = v143 * v2558;
                let v2561 = (v568 * v2537) / v324;
                let v2562 = v2556 - v2561;
                let v2578: f64;
                if v259 != 0.0 {
                    let v2564 = v2513 - v2536;
                    let v2569 = v273 * ((v2513 + v2536) + (((v2564 * v2564) + v272).sqrt()));
                    v2578 = v2569;
                } else {
                    let v2571 = v2513 - v2536;
                    let v2577 = v273 * ((v2513 + v2536) + (v2571 * (((v283 / v272) * v2571).tanh())));
                    v2578 = v2577;
                }
                let v2580 = (v2578 - v2562) / v2537;
                let v2581 = if v2580 > v316 { 1.0 } else { 0.0 };
                let v2605: f64;
                if v2581 != 0.0 {
                    v2605 = v0;
                } else {
                    let v2583 = if v2580 < v2582 { 1.0 } else { 0.0 };
                    let v2606: f64;
                    if v2583 != 0.0 {
                        v2606 = v5;
                    } else {
                        let v2586 = v5 / (v5 + (v2580.exp()));
                        v2606 = v2586;
                    }
                    v2605 = v2606;
                }
                let v2602: f64;
                if v259 != 0.0 {
                    let v2588 = v2513 - v2536;
                    let v2593 = v273 * ((v2513 + v2536) + (((v2588 * v2588) + v272).sqrt()));
                    v2602 = v2593;
                } else {
                    let v2595 = v2513 - v2536;
                    let v2601 = v273 * ((v2513 + v2536) + (v2595 * (((v283 / v272) * v2595).tanh())));
                    v2602 = v2601;
                }
                let v2604 = (v568 * v56) * v2537;
                let v2610 = (v2602 - (v2556 - (v2604 * v2605))) / v2558;
                let v2611 = if v2610 > v316 { 1.0 } else { 0.0 };
                let v2621: f64;
                if v2611 != 0.0 {
                    let v2612 = v2559 * v2610;
                    v2621 = v2612;
                } else {
                    let v2614 = if v2610 < v2613 { 1.0 } else { 0.0 };
                    let v2622: f64;
                    if v2614 != 0.0 {
                        let v2616 = v2559 * (v2610.exp());
                        v2622 = v2616;
                    } else {
                        let v2620 = v2559 * ((v5 + (v2610.exp())).ln());
                        v2622 = v2620;
                    }
                    v2621 = v2622;
                }
                let v2633 = v2522 * ((v5 + (v533 * v3)) / (v5 + (v533 * v46)));
                let v2644 = (((v2633 * (v5 + ((v534 * v2535) / v2511))) / (v5 + ((v2526 * v2621) / v143))) * v2511) / (v2523 / (v2544 * (v5 + ((v2525 * v2621) / v143))));
                let v2654 = (((v2644 * ((v5 + (((v324 * v2621) / v143) / v2644)).sqrt())) - v2644) * (v5 - v2605)) + (v2558 * v2605);
                let v2655 = v422 / v2654;
                let v2669: f64;
                if v259 != 0.0 {
                    let v2656 = v0 - v2655;
                    let v2661 = v273 * (v2655 + (((v2656 * v2656) + v272).sqrt()));
                    v2669 = v2661;
                } else {
                    let v2662 = v0 - v2655;
                    let v2668 = v273 * (v2655 + (v2662 * (((v283 / v272) * v2662).tanh())));
                    v2669 = v2668;
                }
                let v2672 = v5 / v2524;
                let v2675 = v422 * (v5 / ((v5 + (v2669.powf(v2524))).powf(v2672)));
                let v2676 = -v422;
                let v2677 = v2676 / v2654;
                let v2691: f64;
                if v259 != 0.0 {
                    let v2678 = v0 - v2677;
                    let v2683 = v273 * (v2677 + (((v2678 * v2678) + v272).sqrt()));
                    v2691 = v2683;
                } else {
                    let v2684 = v0 - v2677;
                    let v2690 = v273 * (v2677 + (v2684 * (((v283 / v272) * v2684).tanh())));
                    v2691 = v2690;
                }
                let v2696 = v2676 * (v5 / ((v5 + (v2691.powf(v2524))).powf(v2672)));
                let v2698 = (v2513 - v2562) / v2537;
                let v2699 = if v2698 > v316 { 1.0 } else { 0.0 };
                let v2706: f64;
                if v2699 != 0.0 {
                    v2706 = v0;
                } else {
                    let v2701 = if v2698 < v2700 { 1.0 } else { 0.0 };
                    let v2707: f64;
                    if v2701 != 0.0 {
                        v2707 = v5;
                    } else {
                        let v2704 = v5 / (v5 + (v2698.exp()));
                        v2707 = v2704;
                    }
                    v2706 = v2707;
                }
                let v2711 = ((v2536 - v2696) - (v2556 - (v2604 * v2706))) / v2558;
                let v2712 = if v2711 > v316 { 1.0 } else { 0.0 };
                if v2712 != 0.0 {
                } else {
                    let v2714 = if v2711 < v2713 { 1.0 } else { 0.0 };
                    if v2714 != 0.0 {
                    } else {
                    }
                }
                let v2716 = (v2536 - v2562) / v2537;
                let v2717 = if v2716 > v316 { 1.0 } else { 0.0 };
                let v2724: f64;
                if v2717 != 0.0 {
                    v2724 = v0;
                } else {
                    let v2719 = if v2716 < v2718 { 1.0 } else { 0.0 };
                    let v2725: f64;
                    if v2719 != 0.0 {
                        v2725 = v5;
                    } else {
                        let v2722 = v5 / (v5 + (v2716.exp()));
                        v2725 = v2722;
                    }
                    v2724 = v2725;
                }
                let v2729 = ((v2513 - v2675) - (v2556 - (v2604 * v2724))) / v2558;
                let v2730 = if v2729 > v316 { 1.0 } else { 0.0 };
                if v2730 != 0.0 {
                } else {
                    let v2732 = if v2729 < v2731 { 1.0 } else { 0.0 };
                    if v2732 != 0.0 {
                    } else {
                    }
                }
                if v259 != 0.0 {
                } else {
                }
                let v2734 = (v324 * v2539) * v89;
                let v2735 = v143 * v2734;
                let v2736 = v2543 - v2561;
                let v2752: f64;
                if v259 != 0.0 {
                    let v2738 = v2513 - v2536;
                    let v2743 = v273 * ((v2513 + v2536) + (((v2738 * v2738) + v272).sqrt()));
                    v2752 = v2743;
                } else {
                    let v2745 = v2513 - v2536;
                    let v2751 = v273 * ((v2513 + v2536) + (v2745 * (((v283 / v272) * v2745).tanh())));
                    v2752 = v2751;
                }
                let v2754 = (v2752 - v2736) / v2537;
                let v2755 = if v2754 > v316 { 1.0 } else { 0.0 };
                let v2777: f64;
                if v2755 != 0.0 {
                    v2777 = v0;
                } else {
                    let v2757 = if v2754 < v2756 { 1.0 } else { 0.0 };
                    let v2778: f64;
                    if v2757 != 0.0 {
                        v2778 = v5;
                    } else {
                        let v2760 = v5 / (v5 + (v2754.exp()));
                        v2778 = v2760;
                    }
                    v2777 = v2778;
                }
                let v2776: f64;
                if v259 != 0.0 {
                    let v2762 = v2513 - v2536;
                    let v2767 = v273 * ((v2513 + v2536) + (((v2762 * v2762) + v272).sqrt()));
                    v2776 = v2767;
                } else {
                    let v2769 = v2513 - v2536;
                    let v2775 = v273 * ((v2513 + v2536) + (v2769 * (((v283 / v272) * v2769).tanh())));
                    v2776 = v2775;
                }
                let v2782 = (v2776 - (v2543 - (v2604 * v2777))) / v2734;
                let v2783 = if v2782 > v316 { 1.0 } else { 0.0 };
                let v2796: f64;
                if v2783 != 0.0 {
                    let v2784 = v2735 * v2782;
                    v2796 = v2784;
                } else {
                    let v2786 = if v2782 < v2785 { 1.0 } else { 0.0 };
                    let v2797: f64;
                    if v2786 != 0.0 {
                        let v2788 = v2735 * (v2782.exp());
                        v2797 = v2788;
                    } else {
                        let v2792 = v2735 * ((v5 + (v2782.exp())).ln());
                        v2797 = v2792;
                    }
                    v2796 = v2797;
                }
                let v2795 = (v2633 * v2511) / (v2523 / v2544);
                let v2808 = (((v2795 * ((v5 + (((v324 * v2796) / v143) / v2795)).sqrt())) - v2795) * (v5 - v2777)) + (v2734 * v2777);
                let v2809 = v422 / v2808;
                let v2823: f64;
                if v259 != 0.0 {
                    let v2810 = v0 - v2809;
                    let v2815 = v273 * (v2809 + (((v2810 * v2810) + v272).sqrt()));
                    v2823 = v2815;
                } else {
                    let v2816 = v0 - v2809;
                    let v2822 = v273 * (v2809 + (v2816 * (((v283 / v272) * v2816).tanh())));
                    v2823 = v2822;
                }
                let v2828 = v422 * (v5 / ((v5 + (v2823.powf(v2524))).powf(v2672)));
                let v2829 = v2676 / v2808;
                let v2843: f64;
                if v259 != 0.0 {
                    let v2830 = v0 - v2829;
                    let v2835 = v273 * (v2829 + (((v2830 * v2830) + v272).sqrt()));
                    v2843 = v2835;
                } else {
                    let v2836 = v0 - v2829;
                    let v2842 = v273 * (v2829 + (v2836 * (((v283 / v272) * v2836).tanh())));
                    v2843 = v2842;
                }
                let v2848 = v2676 * (v5 / ((v5 + (v2843.powf(v2524))).powf(v2672)));
                let v2850 = (v2513 - v2736) / v2537;
                let v2851 = if v2850 > v316 { 1.0 } else { 0.0 };
                let v2858: f64;
                if v2851 != 0.0 {
                    v2858 = v0;
                } else {
                    let v2853 = if v2850 < v2852 { 1.0 } else { 0.0 };
                    let v2859: f64;
                    if v2853 != 0.0 {
                        v2859 = v5;
                    } else {
                        let v2856 = v5 / (v5 + (v2850.exp()));
                        v2859 = v2856;
                    }
                    v2858 = v2859;
                }
                let v2863 = ((v2536 - v2848) - (v2543 - (v2604 * v2858))) / v2734;
                let v2864 = if v2863 > v316 { 1.0 } else { 0.0 };
                if v2864 != 0.0 {
                } else {
                    let v2866 = if v2863 < v2865 { 1.0 } else { 0.0 };
                    if v2866 != 0.0 {
                    } else {
                    }
                }
                let v2868 = (v2536 - v2736) / v2537;
                let v2869 = if v2868 > v316 { 1.0 } else { 0.0 };
                let v2876: f64;
                if v2869 != 0.0 {
                    v2876 = v0;
                } else {
                    let v2871 = if v2868 < v2870 { 1.0 } else { 0.0 };
                    let v2877: f64;
                    if v2871 != 0.0 {
                        v2877 = v5;
                    } else {
                        let v2874 = v5 / (v5 + (v2868.exp()));
                        v2877 = v2874;
                    }
                    v2876 = v2877;
                }
                let v2881 = ((v2513 - v2828) - (v2543 - (v2604 * v2876))) / v2734;
                let v2882 = if v2881 > v316 { 1.0 } else { 0.0 };
                if v2882 != 0.0 {
                } else {
                    let v2884 = if v2881 < v2883 { 1.0 } else { 0.0 };
                    if v2884 != 0.0 {
                    } else {
                    }
                }
                let v2885 = if v2514 == v5 { 1.0 } else { 0.0 };
                if v2885 != 0.0 {
                    let v2888 = v2543 - ((v568 * v273) * v2537);
                    let v2890 = (v2515 - v2888) / v2734;
                    let v2891 = if v2890 > v316 { 1.0 } else { 0.0 };
                    if v2891 != 0.0 {
                    } else {
                        let v2893 = if v2890 < v2892 { 1.0 } else { 0.0 };
                        if v2893 != 0.0 {
                        } else {
                        }
                    }
                    let v2895 = (v424 - v2888) / v2734;
                    let v2896 = if v2895 > v316 { 1.0 } else { 0.0 };
                    if v2896 != 0.0 {
                    } else {
                        let v2898 = if v2895 < v2897 { 1.0 } else { 0.0 };
                        if v2898 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v2899 = if v2516 == v5 { 1.0 } else { 0.0 };
                if v2899 != 0.0 {
                    let v2904 = (v2513 - (v2543 - ((v568 * v273) * v2537))) / v2734;
                    let v2905 = if v2904 > v316 { 1.0 } else { 0.0 };
                    if v2905 != 0.0 {
                    } else {
                        let v2907 = if v2904 < v2906 { 1.0 } else { 0.0 };
                        if v2907 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if v411 != 0.0 {
            } else {
            }
            let v2909 = if v2908 > v514 { 1.0 } else { 0.0 };
            if v2909 != 0.0 {
                let v2932: f64;
                if v259 != 0.0 {
                    let v2927 = ((v437 * v437) + v272).sqrt();
                    v2932 = v2927;
                } else {
                    let v2931 = v437 * (((v283 / v272) * v437).tanh());
                    v2932 = v2931;
                }
                let v2933 = v2910 - v437;
                let v2934 = v2918 * v89;
                let v2936 = v2915 / (v545 * v89);
                let v2938 = v2936 + (v2917 * v2932);
                let v2940 = v2914 + (v2924 * v91);
                let v2941 = v95.powf(v532);
                let v2942 = if v531 != v0 { 1.0 } else { 0.0 };
                let v2949: f64;
                if v2942 != 0.0 {
                    let v2948 = v2932 / ((v5 + ((v2932 / v531).powf(v2921))).powf((v5 / v2921)));
                    v2949 = v2948;
                } else {
                    v2949 = v0;
                }
                let v2953 = v2940 - ((v2916 - (v2949 * v0)) * v2932);
                let v2955 = (v324 * v2938) * v89;
                let v2956 = v150 * v2955;
                let v2958 = (v568 * v2934) / v324;
                let v2959 = v2953 - v2958;
                let v2975: f64;
                if v259 != 0.0 {
                    let v2961 = v2910 - v2933;
                    let v2966 = v273 * ((v2910 + v2933) + (((v2961 * v2961) + v272).sqrt()));
                    v2975 = v2966;
                } else {
                    let v2968 = v2910 - v2933;
                    let v2974 = v273 * ((v2910 + v2933) + (v2968 * (((v283 / v272) * v2968).tanh())));
                    v2975 = v2974;
                }
                let v2977 = (v2975 - v2959) / v2934;
                let v2978 = if v2977 > v316 { 1.0 } else { 0.0 };
                let v3002: f64;
                if v2978 != 0.0 {
                    v3002 = v0;
                } else {
                    let v2980 = if v2977 < v2979 { 1.0 } else { 0.0 };
                    let v3003: f64;
                    if v2980 != 0.0 {
                        v3003 = v5;
                    } else {
                        let v2983 = v5 / (v5 + (v2977.exp()));
                        v3003 = v2983;
                    }
                    v3002 = v3003;
                }
                let v2999: f64;
                if v259 != 0.0 {
                    let v2985 = v2910 - v2933;
                    let v2990 = v273 * ((v2910 + v2933) + (((v2985 * v2985) + v272).sqrt()));
                    v2999 = v2990;
                } else {
                    let v2992 = v2910 - v2933;
                    let v2998 = v273 * ((v2910 + v2933) + (v2992 * (((v283 / v272) * v2992).tanh())));
                    v2999 = v2998;
                }
                let v3001 = (v568 * v56) * v2934;
                let v3007 = (v2999 - (v2953 - (v3001 * v3002))) / v2955;
                let v3008 = if v3007 > v316 { 1.0 } else { 0.0 };
                let v3018: f64;
                if v3008 != 0.0 {
                    let v3009 = v2956 * v3007;
                    v3018 = v3009;
                } else {
                    let v3011 = if v3007 < v3010 { 1.0 } else { 0.0 };
                    let v3019: f64;
                    if v3011 != 0.0 {
                        let v3013 = v2956 * (v3007.exp());
                        v3019 = v3013;
                    } else {
                        let v3017 = v2956 * ((v5 + (v3007.exp())).ln());
                        v3019 = v3017;
                    }
                    v3018 = v3019;
                }
                let v3030 = v2919 * ((v5 + (v533 * v3)) / (v5 + (v533 * v46)));
                let v3041 = (((v3030 * (v5 + ((v534 * v2932) / v2908))) / (v5 + ((v2923 * v3018) / v150))) * v2908) / (v2920 / (v2941 * (v5 + ((v2922 * v3018) / v150))));
                let v3051 = (((v3041 * ((v5 + (((v324 * v3018) / v150) / v3041)).sqrt())) - v3041) * (v5 - v3002)) + (v2955 * v3002);
                let v3052 = v437 / v3051;
                let v3066: f64;
                if v259 != 0.0 {
                    let v3053 = v0 - v3052;
                    let v3058 = v273 * (v3052 + (((v3053 * v3053) + v272).sqrt()));
                    v3066 = v3058;
                } else {
                    let v3059 = v0 - v3052;
                    let v3065 = v273 * (v3052 + (v3059 * (((v283 / v272) * v3059).tanh())));
                    v3066 = v3065;
                }
                let v3069 = v5 / v2921;
                let v3072 = v437 * (v5 / ((v5 + (v3066.powf(v2921))).powf(v3069)));
                let v3073 = -v437;
                let v3074 = v3073 / v3051;
                let v3088: f64;
                if v259 != 0.0 {
                    let v3075 = v0 - v3074;
                    let v3080 = v273 * (v3074 + (((v3075 * v3075) + v272).sqrt()));
                    v3088 = v3080;
                } else {
                    let v3081 = v0 - v3074;
                    let v3087 = v273 * (v3074 + (v3081 * (((v283 / v272) * v3081).tanh())));
                    v3088 = v3087;
                }
                let v3093 = v3073 * (v5 / ((v5 + (v3088.powf(v2921))).powf(v3069)));
                let v3095 = (v2910 - v2959) / v2934;
                let v3096 = if v3095 > v316 { 1.0 } else { 0.0 };
                let v3103: f64;
                if v3096 != 0.0 {
                    v3103 = v0;
                } else {
                    let v3098 = if v3095 < v3097 { 1.0 } else { 0.0 };
                    let v3104: f64;
                    if v3098 != 0.0 {
                        v3104 = v5;
                    } else {
                        let v3101 = v5 / (v5 + (v3095.exp()));
                        v3104 = v3101;
                    }
                    v3103 = v3104;
                }
                let v3108 = ((v2933 - v3093) - (v2953 - (v3001 * v3103))) / v2955;
                let v3109 = if v3108 > v316 { 1.0 } else { 0.0 };
                if v3109 != 0.0 {
                } else {
                    let v3111 = if v3108 < v3110 { 1.0 } else { 0.0 };
                    if v3111 != 0.0 {
                    } else {
                    }
                }
                let v3113 = (v2933 - v2959) / v2934;
                let v3114 = if v3113 > v316 { 1.0 } else { 0.0 };
                let v3121: f64;
                if v3114 != 0.0 {
                    v3121 = v0;
                } else {
                    let v3116 = if v3113 < v3115 { 1.0 } else { 0.0 };
                    let v3122: f64;
                    if v3116 != 0.0 {
                        v3122 = v5;
                    } else {
                        let v3119 = v5 / (v5 + (v3113.exp()));
                        v3122 = v3119;
                    }
                    v3121 = v3122;
                }
                let v3126 = ((v2910 - v3072) - (v2953 - (v3001 * v3121))) / v2955;
                let v3127 = if v3126 > v316 { 1.0 } else { 0.0 };
                if v3127 != 0.0 {
                } else {
                    let v3129 = if v3126 < v3128 { 1.0 } else { 0.0 };
                    if v3129 != 0.0 {
                    } else {
                    }
                }
                if v259 != 0.0 {
                } else {
                }
                let v3131 = (v324 * v2936) * v89;
                let v3132 = v150 * v3131;
                let v3133 = v2940 - v2958;
                let v3149: f64;
                if v259 != 0.0 {
                    let v3135 = v2910 - v2933;
                    let v3140 = v273 * ((v2910 + v2933) + (((v3135 * v3135) + v272).sqrt()));
                    v3149 = v3140;
                } else {
                    let v3142 = v2910 - v2933;
                    let v3148 = v273 * ((v2910 + v2933) + (v3142 * (((v283 / v272) * v3142).tanh())));
                    v3149 = v3148;
                }
                let v3151 = (v3149 - v3133) / v2934;
                let v3152 = if v3151 > v316 { 1.0 } else { 0.0 };
                let v3174: f64;
                if v3152 != 0.0 {
                    v3174 = v0;
                } else {
                    let v3154 = if v3151 < v3153 { 1.0 } else { 0.0 };
                    let v3175: f64;
                    if v3154 != 0.0 {
                        v3175 = v5;
                    } else {
                        let v3157 = v5 / (v5 + (v3151.exp()));
                        v3175 = v3157;
                    }
                    v3174 = v3175;
                }
                let v3173: f64;
                if v259 != 0.0 {
                    let v3159 = v2910 - v2933;
                    let v3164 = v273 * ((v2910 + v2933) + (((v3159 * v3159) + v272).sqrt()));
                    v3173 = v3164;
                } else {
                    let v3166 = v2910 - v2933;
                    let v3172 = v273 * ((v2910 + v2933) + (v3166 * (((v283 / v272) * v3166).tanh())));
                    v3173 = v3172;
                }
                let v3179 = (v3173 - (v2940 - (v3001 * v3174))) / v3131;
                let v3180 = if v3179 > v316 { 1.0 } else { 0.0 };
                let v3193: f64;
                if v3180 != 0.0 {
                    let v3181 = v3132 * v3179;
                    v3193 = v3181;
                } else {
                    let v3183 = if v3179 < v3182 { 1.0 } else { 0.0 };
                    let v3194: f64;
                    if v3183 != 0.0 {
                        let v3185 = v3132 * (v3179.exp());
                        v3194 = v3185;
                    } else {
                        let v3189 = v3132 * ((v5 + (v3179.exp())).ln());
                        v3194 = v3189;
                    }
                    v3193 = v3194;
                }
                let v3192 = (v3030 * v2908) / (v2920 / v2941);
                let v3205 = (((v3192 * ((v5 + (((v324 * v3193) / v150) / v3192)).sqrt())) - v3192) * (v5 - v3174)) + (v3131 * v3174);
                let v3206 = v437 / v3205;
                let v3220: f64;
                if v259 != 0.0 {
                    let v3207 = v0 - v3206;
                    let v3212 = v273 * (v3206 + (((v3207 * v3207) + v272).sqrt()));
                    v3220 = v3212;
                } else {
                    let v3213 = v0 - v3206;
                    let v3219 = v273 * (v3206 + (v3213 * (((v283 / v272) * v3213).tanh())));
                    v3220 = v3219;
                }
                let v3225 = v437 * (v5 / ((v5 + (v3220.powf(v2921))).powf(v3069)));
                let v3226 = v3073 / v3205;
                let v3240: f64;
                if v259 != 0.0 {
                    let v3227 = v0 - v3226;
                    let v3232 = v273 * (v3226 + (((v3227 * v3227) + v272).sqrt()));
                    v3240 = v3232;
                } else {
                    let v3233 = v0 - v3226;
                    let v3239 = v273 * (v3226 + (v3233 * (((v283 / v272) * v3233).tanh())));
                    v3240 = v3239;
                }
                let v3245 = v3073 * (v5 / ((v5 + (v3240.powf(v2921))).powf(v3069)));
                let v3247 = (v2910 - v3133) / v2934;
                let v3248 = if v3247 > v316 { 1.0 } else { 0.0 };
                let v3255: f64;
                if v3248 != 0.0 {
                    v3255 = v0;
                } else {
                    let v3250 = if v3247 < v3249 { 1.0 } else { 0.0 };
                    let v3256: f64;
                    if v3250 != 0.0 {
                        v3256 = v5;
                    } else {
                        let v3253 = v5 / (v5 + (v3247.exp()));
                        v3256 = v3253;
                    }
                    v3255 = v3256;
                }
                let v3260 = ((v2933 - v3245) - (v2940 - (v3001 * v3255))) / v3131;
                let v3261 = if v3260 > v316 { 1.0 } else { 0.0 };
                if v3261 != 0.0 {
                } else {
                    let v3263 = if v3260 < v3262 { 1.0 } else { 0.0 };
                    if v3263 != 0.0 {
                    } else {
                    }
                }
                let v3265 = (v2933 - v3133) / v2934;
                let v3266 = if v3265 > v316 { 1.0 } else { 0.0 };
                let v3273: f64;
                if v3266 != 0.0 {
                    v3273 = v0;
                } else {
                    let v3268 = if v3265 < v3267 { 1.0 } else { 0.0 };
                    let v3274: f64;
                    if v3268 != 0.0 {
                        v3274 = v5;
                    } else {
                        let v3271 = v5 / (v5 + (v3265.exp()));
                        v3274 = v3271;
                    }
                    v3273 = v3274;
                }
                let v3278 = ((v2910 - v3225) - (v2940 - (v3001 * v3273))) / v3131;
                let v3279 = if v3278 > v316 { 1.0 } else { 0.0 };
                if v3279 != 0.0 {
                } else {
                    let v3281 = if v3278 < v3280 { 1.0 } else { 0.0 };
                    if v3281 != 0.0 {
                    } else {
                    }
                }
                let v3282 = if v2911 == v5 { 1.0 } else { 0.0 };
                if v3282 != 0.0 {
                    let v3285 = v2940 - ((v568 * v273) * v2934);
                    let v3287 = (v2912 - v3285) / v3131;
                    let v3288 = if v3287 > v316 { 1.0 } else { 0.0 };
                    if v3288 != 0.0 {
                    } else {
                        let v3290 = if v3287 < v3289 { 1.0 } else { 0.0 };
                        if v3290 != 0.0 {
                        } else {
                        }
                    }
                    let v3292 = (v439 - v3285) / v3131;
                    let v3293 = if v3292 > v316 { 1.0 } else { 0.0 };
                    if v3293 != 0.0 {
                    } else {
                        let v3295 = if v3292 < v3294 { 1.0 } else { 0.0 };
                        if v3295 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v3296 = if v2913 == v5 { 1.0 } else { 0.0 };
                if v3296 != 0.0 {
                    let v3301 = (v2910 - (v2940 - ((v568 * v273) * v2934))) / v3131;
                    let v3302 = if v3301 > v316 { 1.0 } else { 0.0 };
                    if v3302 != 0.0 {
                    } else {
                        let v3304 = if v3301 < v3303 { 1.0 } else { 0.0 };
                        if v3304 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if v426 != 0.0 {
            } else {
            }
            let v3306 = if v3305 > v514 { 1.0 } else { 0.0 };
            if v3306 != 0.0 {
                let v3329: f64;
                if v259 != 0.0 {
                    let v3324 = ((v451 * v451) + v272).sqrt();
                    v3329 = v3324;
                } else {
                    let v3328 = v451 * (((v283 / v272) * v451).tanh());
                    v3329 = v3328;
                }
                let v3330 = v3307 - v451;
                let v3331 = v3315 * v89;
                let v3333 = v3312 / (v545 * v89);
                let v3335 = v3333 + (v3314 * v3329);
                let v3337 = v3311 + (v3321 * v91);
                let v3338 = v95.powf(v532);
                let v3339 = if v531 != v0 { 1.0 } else { 0.0 };
                let v3346: f64;
                if v3339 != 0.0 {
                    let v3345 = v3329 / ((v5 + ((v3329 / v531).powf(v3318))).powf((v5 / v3318)));
                    v3346 = v3345;
                } else {
                    v3346 = v0;
                }
                let v3350 = v3337 - ((v3313 - (v3346 * v0)) * v3329);
                let v3352 = (v324 * v3335) * v89;
                let v3353 = v157 * v3352;
                let v3355 = (v568 * v3331) / v324;
                let v3356 = v3350 - v3355;
                let v3372: f64;
                if v259 != 0.0 {
                    let v3358 = v3307 - v3330;
                    let v3363 = v273 * ((v3307 + v3330) + (((v3358 * v3358) + v272).sqrt()));
                    v3372 = v3363;
                } else {
                    let v3365 = v3307 - v3330;
                    let v3371 = v273 * ((v3307 + v3330) + (v3365 * (((v283 / v272) * v3365).tanh())));
                    v3372 = v3371;
                }
                let v3374 = (v3372 - v3356) / v3331;
                let v3375 = if v3374 > v316 { 1.0 } else { 0.0 };
                let v3399: f64;
                if v3375 != 0.0 {
                    v3399 = v0;
                } else {
                    let v3377 = if v3374 < v3376 { 1.0 } else { 0.0 };
                    let v3400: f64;
                    if v3377 != 0.0 {
                        v3400 = v5;
                    } else {
                        let v3380 = v5 / (v5 + (v3374.exp()));
                        v3400 = v3380;
                    }
                    v3399 = v3400;
                }
                let v3396: f64;
                if v259 != 0.0 {
                    let v3382 = v3307 - v3330;
                    let v3387 = v273 * ((v3307 + v3330) + (((v3382 * v3382) + v272).sqrt()));
                    v3396 = v3387;
                } else {
                    let v3389 = v3307 - v3330;
                    let v3395 = v273 * ((v3307 + v3330) + (v3389 * (((v283 / v272) * v3389).tanh())));
                    v3396 = v3395;
                }
                let v3398 = (v568 * v56) * v3331;
                let v3404 = (v3396 - (v3350 - (v3398 * v3399))) / v3352;
                let v3405 = if v3404 > v316 { 1.0 } else { 0.0 };
                let v3415: f64;
                if v3405 != 0.0 {
                    let v3406 = v3353 * v3404;
                    v3415 = v3406;
                } else {
                    let v3408 = if v3404 < v3407 { 1.0 } else { 0.0 };
                    let v3416: f64;
                    if v3408 != 0.0 {
                        let v3410 = v3353 * (v3404.exp());
                        v3416 = v3410;
                    } else {
                        let v3414 = v3353 * ((v5 + (v3404.exp())).ln());
                        v3416 = v3414;
                    }
                    v3415 = v3416;
                }
                let v3427 = v3316 * ((v5 + (v533 * v3)) / (v5 + (v533 * v46)));
                let v3438 = (((v3427 * (v5 + ((v534 * v3329) / v3305))) / (v5 + ((v3320 * v3415) / v157))) * v3305) / (v3317 / (v3338 * (v5 + ((v3319 * v3415) / v157))));
                let v3448 = (((v3438 * ((v5 + (((v324 * v3415) / v157) / v3438)).sqrt())) - v3438) * (v5 - v3399)) + (v3352 * v3399);
                let v3449 = v451 / v3448;
                let v3463: f64;
                if v259 != 0.0 {
                    let v3450 = v0 - v3449;
                    let v3455 = v273 * (v3449 + (((v3450 * v3450) + v272).sqrt()));
                    v3463 = v3455;
                } else {
                    let v3456 = v0 - v3449;
                    let v3462 = v273 * (v3449 + (v3456 * (((v283 / v272) * v3456).tanh())));
                    v3463 = v3462;
                }
                let v3466 = v5 / v3318;
                let v3469 = v451 * (v5 / ((v5 + (v3463.powf(v3318))).powf(v3466)));
                let v3470 = -v451;
                let v3471 = v3470 / v3448;
                let v3485: f64;
                if v259 != 0.0 {
                    let v3472 = v0 - v3471;
                    let v3477 = v273 * (v3471 + (((v3472 * v3472) + v272).sqrt()));
                    v3485 = v3477;
                } else {
                    let v3478 = v0 - v3471;
                    let v3484 = v273 * (v3471 + (v3478 * (((v283 / v272) * v3478).tanh())));
                    v3485 = v3484;
                }
                let v3490 = v3470 * (v5 / ((v5 + (v3485.powf(v3318))).powf(v3466)));
                let v3492 = (v3307 - v3356) / v3331;
                let v3493 = if v3492 > v316 { 1.0 } else { 0.0 };
                let v3500: f64;
                if v3493 != 0.0 {
                    v3500 = v0;
                } else {
                    let v3495 = if v3492 < v3494 { 1.0 } else { 0.0 };
                    let v3501: f64;
                    if v3495 != 0.0 {
                        v3501 = v5;
                    } else {
                        let v3498 = v5 / (v5 + (v3492.exp()));
                        v3501 = v3498;
                    }
                    v3500 = v3501;
                }
                let v3505 = ((v3330 - v3490) - (v3350 - (v3398 * v3500))) / v3352;
                let v3506 = if v3505 > v316 { 1.0 } else { 0.0 };
                if v3506 != 0.0 {
                } else {
                    let v3508 = if v3505 < v3507 { 1.0 } else { 0.0 };
                    if v3508 != 0.0 {
                    } else {
                    }
                }
                let v3510 = (v3330 - v3356) / v3331;
                let v3511 = if v3510 > v316 { 1.0 } else { 0.0 };
                let v3518: f64;
                if v3511 != 0.0 {
                    v3518 = v0;
                } else {
                    let v3513 = if v3510 < v3512 { 1.0 } else { 0.0 };
                    let v3519: f64;
                    if v3513 != 0.0 {
                        v3519 = v5;
                    } else {
                        let v3516 = v5 / (v5 + (v3510.exp()));
                        v3519 = v3516;
                    }
                    v3518 = v3519;
                }
                let v3523 = ((v3307 - v3469) - (v3350 - (v3398 * v3518))) / v3352;
                let v3524 = if v3523 > v316 { 1.0 } else { 0.0 };
                if v3524 != 0.0 {
                } else {
                    let v3526 = if v3523 < v3525 { 1.0 } else { 0.0 };
                    if v3526 != 0.0 {
                    } else {
                    }
                }
                if v259 != 0.0 {
                } else {
                }
                let v3528 = (v324 * v3333) * v89;
                let v3529 = v157 * v3528;
                let v3530 = v3337 - v3355;
                let v3546: f64;
                if v259 != 0.0 {
                    let v3532 = v3307 - v3330;
                    let v3537 = v273 * ((v3307 + v3330) + (((v3532 * v3532) + v272).sqrt()));
                    v3546 = v3537;
                } else {
                    let v3539 = v3307 - v3330;
                    let v3545 = v273 * ((v3307 + v3330) + (v3539 * (((v283 / v272) * v3539).tanh())));
                    v3546 = v3545;
                }
                let v3548 = (v3546 - v3530) / v3331;
                let v3549 = if v3548 > v316 { 1.0 } else { 0.0 };
                let v3571: f64;
                if v3549 != 0.0 {
                    v3571 = v0;
                } else {
                    let v3551 = if v3548 < v3550 { 1.0 } else { 0.0 };
                    let v3572: f64;
                    if v3551 != 0.0 {
                        v3572 = v5;
                    } else {
                        let v3554 = v5 / (v5 + (v3548.exp()));
                        v3572 = v3554;
                    }
                    v3571 = v3572;
                }
                let v3570: f64;
                if v259 != 0.0 {
                    let v3556 = v3307 - v3330;
                    let v3561 = v273 * ((v3307 + v3330) + (((v3556 * v3556) + v272).sqrt()));
                    v3570 = v3561;
                } else {
                    let v3563 = v3307 - v3330;
                    let v3569 = v273 * ((v3307 + v3330) + (v3563 * (((v283 / v272) * v3563).tanh())));
                    v3570 = v3569;
                }
                let v3576 = (v3570 - (v3337 - (v3398 * v3571))) / v3528;
                let v3577 = if v3576 > v316 { 1.0 } else { 0.0 };
                let v3590: f64;
                if v3577 != 0.0 {
                    let v3578 = v3529 * v3576;
                    v3590 = v3578;
                } else {
                    let v3580 = if v3576 < v3579 { 1.0 } else { 0.0 };
                    let v3591: f64;
                    if v3580 != 0.0 {
                        let v3582 = v3529 * (v3576.exp());
                        v3591 = v3582;
                    } else {
                        let v3586 = v3529 * ((v5 + (v3576.exp())).ln());
                        v3591 = v3586;
                    }
                    v3590 = v3591;
                }
                let v3589 = (v3427 * v3305) / (v3317 / v3338);
                let v3602 = (((v3589 * ((v5 + (((v324 * v3590) / v157) / v3589)).sqrt())) - v3589) * (v5 - v3571)) + (v3528 * v3571);
                let v3603 = v451 / v3602;
                let v3617: f64;
                if v259 != 0.0 {
                    let v3604 = v0 - v3603;
                    let v3609 = v273 * (v3603 + (((v3604 * v3604) + v272).sqrt()));
                    v3617 = v3609;
                } else {
                    let v3610 = v0 - v3603;
                    let v3616 = v273 * (v3603 + (v3610 * (((v283 / v272) * v3610).tanh())));
                    v3617 = v3616;
                }
                let v3622 = v451 * (v5 / ((v5 + (v3617.powf(v3318))).powf(v3466)));
                let v3623 = v3470 / v3602;
                let v3637: f64;
                if v259 != 0.0 {
                    let v3624 = v0 - v3623;
                    let v3629 = v273 * (v3623 + (((v3624 * v3624) + v272).sqrt()));
                    v3637 = v3629;
                } else {
                    let v3630 = v0 - v3623;
                    let v3636 = v273 * (v3623 + (v3630 * (((v283 / v272) * v3630).tanh())));
                    v3637 = v3636;
                }
                let v3642 = v3470 * (v5 / ((v5 + (v3637.powf(v3318))).powf(v3466)));
                let v3644 = (v3307 - v3530) / v3331;
                let v3645 = if v3644 > v316 { 1.0 } else { 0.0 };
                let v3652: f64;
                if v3645 != 0.0 {
                    v3652 = v0;
                } else {
                    let v3647 = if v3644 < v3646 { 1.0 } else { 0.0 };
                    let v3653: f64;
                    if v3647 != 0.0 {
                        v3653 = v5;
                    } else {
                        let v3650 = v5 / (v5 + (v3644.exp()));
                        v3653 = v3650;
                    }
                    v3652 = v3653;
                }
                let v3657 = ((v3330 - v3642) - (v3337 - (v3398 * v3652))) / v3528;
                let v3658 = if v3657 > v316 { 1.0 } else { 0.0 };
                if v3658 != 0.0 {
                } else {
                    let v3660 = if v3657 < v3659 { 1.0 } else { 0.0 };
                    if v3660 != 0.0 {
                    } else {
                    }
                }
                let v3662 = (v3330 - v3530) / v3331;
                let v3663 = if v3662 > v316 { 1.0 } else { 0.0 };
                let v3670: f64;
                if v3663 != 0.0 {
                    v3670 = v0;
                } else {
                    let v3665 = if v3662 < v3664 { 1.0 } else { 0.0 };
                    let v3671: f64;
                    if v3665 != 0.0 {
                        v3671 = v5;
                    } else {
                        let v3668 = v5 / (v5 + (v3662.exp()));
                        v3671 = v3668;
                    }
                    v3670 = v3671;
                }
                let v3675 = ((v3307 - v3622) - (v3337 - (v3398 * v3670))) / v3528;
                let v3676 = if v3675 > v316 { 1.0 } else { 0.0 };
                if v3676 != 0.0 {
                } else {
                    let v3678 = if v3675 < v3677 { 1.0 } else { 0.0 };
                    if v3678 != 0.0 {
                    } else {
                    }
                }
                let v3679 = if v3308 == v5 { 1.0 } else { 0.0 };
                if v3679 != 0.0 {
                    let v3682 = v3337 - ((v568 * v273) * v3331);
                    let v3684 = (v3309 - v3682) / v3528;
                    let v3685 = if v3684 > v316 { 1.0 } else { 0.0 };
                    if v3685 != 0.0 {
                    } else {
                        let v3687 = if v3684 < v3686 { 1.0 } else { 0.0 };
                        if v3687 != 0.0 {
                        } else {
                        }
                    }
                    let v3689 = (v453 - v3682) / v3528;
                    let v3690 = if v3689 > v316 { 1.0 } else { 0.0 };
                    if v3690 != 0.0 {
                    } else {
                        let v3692 = if v3689 < v3691 { 1.0 } else { 0.0 };
                        if v3692 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v3693 = if v3310 == v5 { 1.0 } else { 0.0 };
                if v3693 != 0.0 {
                    let v3698 = (v3307 - (v3337 - ((v568 * v273) * v3331))) / v3528;
                    let v3699 = if v3698 > v316 { 1.0 } else { 0.0 };
                    if v3699 != 0.0 {
                    } else {
                        let v3701 = if v3698 < v3700 { 1.0 } else { 0.0 };
                        if v3701 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            if v441 != 0.0 {
            } else {
            }
            let v3703 = if v18 != 0.0 && (if v29 > v514 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v3703 != 0.0 {
                let v3721: f64;
                if v259 != 0.0 {
                    let v3716 = ((v300 * v300) + v272).sqrt();
                    v3721 = v3716;
                } else {
                    let v3720 = v300 * (((v283 / v272) * v300).tanh());
                    v3721 = v3720;
                }
                let v3722 = v303 - v300;
                let v3723 = v3707 * v89;
                let v3725 = v3704 / (v545 * v89);
                let v3727 = v3725 + (v3706 * v3721);
                let v3729 = v291 + (v3713 * v91);
                let v3730 = v95.powf(v532);
                let v3731 = if v531 != v0 { 1.0 } else { 0.0 };
                let v3738: f64;
                if v3731 != 0.0 {
                    let v3737 = v3721 / ((v5 + ((v3721 / v531).powf(v3710))).powf((v5 / v3710)));
                    v3738 = v3737;
                } else {
                    v3738 = v0;
                }
                let v3742 = v3729 - ((v3705 - (v3738 * v0)) * v3721);
                let v3744 = (v324 * v3727) * v89;
                let v3745 = v292 * v3744;
                let v3747 = (v568 * v3723) / v324;
                let v3748 = v3742 - v3747;
                let v3764: f64;
                if v259 != 0.0 {
                    let v3750 = v303 - v3722;
                    let v3755 = v273 * ((v303 + v3722) + (((v3750 * v3750) + v272).sqrt()));
                    v3764 = v3755;
                } else {
                    let v3757 = v303 - v3722;
                    let v3763 = v273 * ((v303 + v3722) + (v3757 * (((v283 / v272) * v3757).tanh())));
                    v3764 = v3763;
                }
                let v3766 = (v3764 - v3748) / v3723;
                let v3767 = if v3766 > v316 { 1.0 } else { 0.0 };
                let v3791: f64;
                if v3767 != 0.0 {
                    v3791 = v0;
                } else {
                    let v3769 = if v3766 < v3768 { 1.0 } else { 0.0 };
                    let v3792: f64;
                    if v3769 != 0.0 {
                        v3792 = v5;
                    } else {
                        let v3772 = v5 / (v5 + (v3766.exp()));
                        v3792 = v3772;
                    }
                    v3791 = v3792;
                }
                let v3788: f64;
                if v259 != 0.0 {
                    let v3774 = v303 - v3722;
                    let v3779 = v273 * ((v303 + v3722) + (((v3774 * v3774) + v272).sqrt()));
                    v3788 = v3779;
                } else {
                    let v3781 = v303 - v3722;
                    let v3787 = v273 * ((v303 + v3722) + (v3781 * (((v283 / v272) * v3781).tanh())));
                    v3788 = v3787;
                }
                let v3790 = (v568 * v56) * v3723;
                let v3796 = (v3788 - (v3742 - (v3790 * v3791))) / v3744;
                let v3797 = if v3796 > v316 { 1.0 } else { 0.0 };
                let v3807: f64;
                if v3797 != 0.0 {
                    let v3798 = v3745 * v3796;
                    v3807 = v3798;
                } else {
                    let v3800 = if v3796 < v3799 { 1.0 } else { 0.0 };
                    let v3808: f64;
                    if v3800 != 0.0 {
                        let v3802 = v3745 * (v3796.exp());
                        v3808 = v3802;
                    } else {
                        let v3806 = v3745 * ((v5 + (v3796.exp())).ln());
                        v3808 = v3806;
                    }
                    v3807 = v3808;
                }
                let v3819 = v3708 * ((v5 + (v533 * v3)) / (v5 + (v533 * v46)));
                let v3830 = (((v3819 * (v5 + ((v534 * v3721) / v29))) / (v5 + ((v3712 * v3807) / v292))) * v29) / (v3709 / (v3730 * (v5 + ((v3711 * v3807) / v292))));
                let v3840 = (((v3830 * ((v5 + (((v324 * v3807) / v292) / v3830)).sqrt())) - v3830) * (v5 - v3791)) + (v3744 * v3791);
                let v3841 = v300 / v3840;
                let v3855: f64;
                if v259 != 0.0 {
                    let v3842 = v0 - v3841;
                    let v3847 = v273 * (v3841 + (((v3842 * v3842) + v272).sqrt()));
                    v3855 = v3847;
                } else {
                    let v3848 = v0 - v3841;
                    let v3854 = v273 * (v3841 + (v3848 * (((v283 / v272) * v3848).tanh())));
                    v3855 = v3854;
                }
                let v3858 = v5 / v3710;
                let v3861 = v300 * (v5 / ((v5 + (v3855.powf(v3710))).powf(v3858)));
                let v3862 = -v300;
                let v3863 = v3862 / v3840;
                let v3877: f64;
                if v259 != 0.0 {
                    let v3864 = v0 - v3863;
                    let v3869 = v273 * (v3863 + (((v3864 * v3864) + v272).sqrt()));
                    v3877 = v3869;
                } else {
                    let v3870 = v0 - v3863;
                    let v3876 = v273 * (v3863 + (v3870 * (((v283 / v272) * v3870).tanh())));
                    v3877 = v3876;
                }
                let v3882 = v3862 * (v5 / ((v5 + (v3877.powf(v3710))).powf(v3858)));
                let v3884 = (v303 - v3748) / v3723;
                let v3885 = if v3884 > v316 { 1.0 } else { 0.0 };
                let v3892: f64;
                if v3885 != 0.0 {
                    v3892 = v0;
                } else {
                    let v3887 = if v3884 < v3886 { 1.0 } else { 0.0 };
                    let v3893: f64;
                    if v3887 != 0.0 {
                        v3893 = v5;
                    } else {
                        let v3890 = v5 / (v5 + (v3884.exp()));
                        v3893 = v3890;
                    }
                    v3892 = v3893;
                }
                let v3897 = ((v3722 - v3882) - (v3742 - (v3790 * v3892))) / v3744;
                let v3898 = if v3897 > v316 { 1.0 } else { 0.0 };
                if v3898 != 0.0 {
                } else {
                    let v3900 = if v3897 < v3899 { 1.0 } else { 0.0 };
                    if v3900 != 0.0 {
                    } else {
                    }
                }
                let v3902 = (v3722 - v3748) / v3723;
                let v3903 = if v3902 > v316 { 1.0 } else { 0.0 };
                let v3910: f64;
                if v3903 != 0.0 {
                    v3910 = v0;
                } else {
                    let v3905 = if v3902 < v3904 { 1.0 } else { 0.0 };
                    let v3911: f64;
                    if v3905 != 0.0 {
                        v3911 = v5;
                    } else {
                        let v3908 = v5 / (v5 + (v3902.exp()));
                        v3911 = v3908;
                    }
                    v3910 = v3911;
                }
                let v3915 = ((v303 - v3861) - (v3742 - (v3790 * v3910))) / v3744;
                let v3916 = if v3915 > v316 { 1.0 } else { 0.0 };
                if v3916 != 0.0 {
                } else {
                    let v3918 = if v3915 < v3917 { 1.0 } else { 0.0 };
                    if v3918 != 0.0 {
                    } else {
                    }
                }
                if v259 != 0.0 {
                } else {
                }
                let v3920 = (v324 * v3725) * v89;
                let v3921 = v292 * v3920;
                let v3922 = v3729 - v3747;
                let v3938: f64;
                if v259 != 0.0 {
                    let v3924 = v303 - v3722;
                    let v3929 = v273 * ((v303 + v3722) + (((v3924 * v3924) + v272).sqrt()));
                    v3938 = v3929;
                } else {
                    let v3931 = v303 - v3722;
                    let v3937 = v273 * ((v303 + v3722) + (v3931 * (((v283 / v272) * v3931).tanh())));
                    v3938 = v3937;
                }
                let v3940 = (v3938 - v3922) / v3723;
                let v3941 = if v3940 > v316 { 1.0 } else { 0.0 };
                let v3963: f64;
                if v3941 != 0.0 {
                    v3963 = v0;
                } else {
                    let v3943 = if v3940 < v3942 { 1.0 } else { 0.0 };
                    let v3964: f64;
                    if v3943 != 0.0 {
                        v3964 = v5;
                    } else {
                        let v3946 = v5 / (v5 + (v3940.exp()));
                        v3964 = v3946;
                    }
                    v3963 = v3964;
                }
                let v3962: f64;
                if v259 != 0.0 {
                    let v3948 = v303 - v3722;
                    let v3953 = v273 * ((v303 + v3722) + (((v3948 * v3948) + v272).sqrt()));
                    v3962 = v3953;
                } else {
                    let v3955 = v303 - v3722;
                    let v3961 = v273 * ((v303 + v3722) + (v3955 * (((v283 / v272) * v3955).tanh())));
                    v3962 = v3961;
                }
                let v3968 = (v3962 - (v3729 - (v3790 * v3963))) / v3920;
                let v3969 = if v3968 > v316 { 1.0 } else { 0.0 };
                let v3982: f64;
                if v3969 != 0.0 {
                    let v3970 = v3921 * v3968;
                    v3982 = v3970;
                } else {
                    let v3972 = if v3968 < v3971 { 1.0 } else { 0.0 };
                    let v3983: f64;
                    if v3972 != 0.0 {
                        let v3974 = v3921 * (v3968.exp());
                        v3983 = v3974;
                    } else {
                        let v3978 = v3921 * ((v5 + (v3968.exp())).ln());
                        v3983 = v3978;
                    }
                    v3982 = v3983;
                }
                let v3981 = (v3819 * v29) / (v3709 / v3730);
                let v3994 = (((v3981 * ((v5 + (((v324 * v3982) / v292) / v3981)).sqrt())) - v3981) * (v5 - v3963)) + (v3920 * v3963);
                let v3995 = v300 / v3994;
                let v4009: f64;
                if v259 != 0.0 {
                    let v3996 = v0 - v3995;
                    let v4001 = v273 * (v3995 + (((v3996 * v3996) + v272).sqrt()));
                    v4009 = v4001;
                } else {
                    let v4002 = v0 - v3995;
                    let v4008 = v273 * (v3995 + (v4002 * (((v283 / v272) * v4002).tanh())));
                    v4009 = v4008;
                }
                let v4014 = v300 * (v5 / ((v5 + (v4009.powf(v3710))).powf(v3858)));
                let v4015 = v3862 / v3994;
                let v4029: f64;
                if v259 != 0.0 {
                    let v4016 = v0 - v4015;
                    let v4021 = v273 * (v4015 + (((v4016 * v4016) + v272).sqrt()));
                    v4029 = v4021;
                } else {
                    let v4022 = v0 - v4015;
                    let v4028 = v273 * (v4015 + (v4022 * (((v283 / v272) * v4022).tanh())));
                    v4029 = v4028;
                }
                let v4034 = v3862 * (v5 / ((v5 + (v4029.powf(v3710))).powf(v3858)));
                let v4036 = (v303 - v3922) / v3723;
                let v4037 = if v4036 > v316 { 1.0 } else { 0.0 };
                let v4044: f64;
                if v4037 != 0.0 {
                    v4044 = v0;
                } else {
                    let v4039 = if v4036 < v4038 { 1.0 } else { 0.0 };
                    let v4045: f64;
                    if v4039 != 0.0 {
                        v4045 = v5;
                    } else {
                        let v4042 = v5 / (v5 + (v4036.exp()));
                        v4045 = v4042;
                    }
                    v4044 = v4045;
                }
                let v4049 = ((v3722 - v4034) - (v3729 - (v3790 * v4044))) / v3920;
                let v4050 = if v4049 > v316 { 1.0 } else { 0.0 };
                if v4050 != 0.0 {
                } else {
                    let v4052 = if v4049 < v4051 { 1.0 } else { 0.0 };
                    if v4052 != 0.0 {
                    } else {
                    }
                }
                let v4054 = (v3722 - v3922) / v3723;
                let v4055 = if v4054 > v316 { 1.0 } else { 0.0 };
                let v4062: f64;
                if v4055 != 0.0 {
                    v4062 = v0;
                } else {
                    let v4057 = if v4054 < v4056 { 1.0 } else { 0.0 };
                    let v4063: f64;
                    if v4057 != 0.0 {
                        v4063 = v5;
                    } else {
                        let v4060 = v5 / (v5 + (v4054.exp()));
                        v4063 = v4060;
                    }
                    v4062 = v4063;
                }
                let v4067 = ((v303 - v4014) - (v3729 - (v3790 * v4062))) / v3920;
                let v4068 = if v4067 > v316 { 1.0 } else { 0.0 };
                if v4068 != 0.0 {
                } else {
                    let v4070 = if v4067 < v4069 { 1.0 } else { 0.0 };
                    if v4070 != 0.0 {
                    } else {
                    }
                }
                if v4071 != 0.0 {
                    let v4076 = (v0 - (v3729 - ((v568 * v273) * v3723))) / v3920;
                    let v4077 = if v4076 > v316 { 1.0 } else { 0.0 };
                    if v4077 != 0.0 {
                    } else {
                        let v4079 = if v4076 < v4078 { 1.0 } else { 0.0 };
                        if v4079 != 0.0 {
                        } else {
                        }
                    }
                    if v4077 != 0.0 {
                    } else {
                        let v4081 = if v4076 < v4080 { 1.0 } else { 0.0 };
                        if v4081 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                if v4082 != 0.0 {
                    let v4087 = (v303 - (v3729 - ((v568 * v273) * v3723))) / v3920;
                    let v4088 = if v4087 > v316 { 1.0 } else { 0.0 };
                    if v4088 != 0.0 {
                    } else {
                        let v4090 = if v4087 < v4089 { 1.0 } else { 0.0 };
                        if v4090 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            let v4092 = if v18 != 0.0 && (if v35 > v514 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v4092 != 0.0 {
                let v4109: f64;
                if v259 != 0.0 {
                    let v4104 = ((v389 * v389) + v272).sqrt();
                    v4109 = v4104;
                } else {
                    let v4108 = v389 * (((v283 / v272) * v389).tanh());
                    v4109 = v4108;
                }
                let v4110 = v392 - v389;
                let v4111 = v4096 * v89;
                let v4113 = v4093 / (v545 * v89);
                let v4115 = v4113 + (v4095 * v4109);
                let v4117 = v379 + (v3713 * v91);
                let v4118 = v95.powf(v532);
                let v4119 = if v531 != v0 { 1.0 } else { 0.0 };
                let v4126: f64;
                if v4119 != 0.0 {
                    let v4125 = v4109 / ((v5 + ((v4109 / v531).powf(v4099))).powf((v5 / v4099)));
                    v4126 = v4125;
                } else {
                    v4126 = v0;
                }
                let v4130 = v4117 - ((v4094 - (v4126 * v0)) * v4109);
                let v4132 = (v324 * v4115) * v89;
                let v4133 = v382 * v4132;
                let v4135 = (v568 * v4111) / v324;
                let v4136 = v4130 - v4135;
                let v4152: f64;
                if v259 != 0.0 {
                    let v4138 = v392 - v4110;
                    let v4143 = v273 * ((v392 + v4110) + (((v4138 * v4138) + v272).sqrt()));
                    v4152 = v4143;
                } else {
                    let v4145 = v392 - v4110;
                    let v4151 = v273 * ((v392 + v4110) + (v4145 * (((v283 / v272) * v4145).tanh())));
                    v4152 = v4151;
                }
                let v4154 = (v4152 - v4136) / v4111;
                let v4155 = if v4154 > v316 { 1.0 } else { 0.0 };
                let v4179: f64;
                if v4155 != 0.0 {
                    v4179 = v0;
                } else {
                    let v4157 = if v4154 < v4156 { 1.0 } else { 0.0 };
                    let v4180: f64;
                    if v4157 != 0.0 {
                        v4180 = v5;
                    } else {
                        let v4160 = v5 / (v5 + (v4154.exp()));
                        v4180 = v4160;
                    }
                    v4179 = v4180;
                }
                let v4176: f64;
                if v259 != 0.0 {
                    let v4162 = v392 - v4110;
                    let v4167 = v273 * ((v392 + v4110) + (((v4162 * v4162) + v272).sqrt()));
                    v4176 = v4167;
                } else {
                    let v4169 = v392 - v4110;
                    let v4175 = v273 * ((v392 + v4110) + (v4169 * (((v283 / v272) * v4169).tanh())));
                    v4176 = v4175;
                }
                let v4178 = (v568 * v56) * v4111;
                let v4184 = (v4176 - (v4130 - (v4178 * v4179))) / v4132;
                let v4185 = if v4184 > v316 { 1.0 } else { 0.0 };
                let v4195: f64;
                if v4185 != 0.0 {
                    let v4186 = v4133 * v4184;
                    v4195 = v4186;
                } else {
                    let v4188 = if v4184 < v4187 { 1.0 } else { 0.0 };
                    let v4196: f64;
                    if v4188 != 0.0 {
                        let v4190 = v4133 * (v4184.exp());
                        v4196 = v4190;
                    } else {
                        let v4194 = v4133 * ((v5 + (v4184.exp())).ln());
                        v4196 = v4194;
                    }
                    v4195 = v4196;
                }
                let v4207 = v4097 * ((v5 + (v533 * v3)) / (v5 + (v533 * v46)));
                let v4218 = (((v4207 * (v5 + ((v534 * v4109) / v35))) / (v5 + ((v4101 * v4195) / v382))) * v35) / (v4098 / (v4118 * (v5 + ((v4100 * v4195) / v382))));
                let v4228 = (((v4218 * ((v5 + (((v324 * v4195) / v382) / v4218)).sqrt())) - v4218) * (v5 - v4179)) + (v4132 * v4179);
                let v4229 = v389 / v4228;
                let v4243: f64;
                if v259 != 0.0 {
                    let v4230 = v0 - v4229;
                    let v4235 = v273 * (v4229 + (((v4230 * v4230) + v272).sqrt()));
                    v4243 = v4235;
                } else {
                    let v4236 = v0 - v4229;
                    let v4242 = v273 * (v4229 + (v4236 * (((v283 / v272) * v4236).tanh())));
                    v4243 = v4242;
                }
                let v4246 = v5 / v4099;
                let v4249 = v389 * (v5 / ((v5 + (v4243.powf(v4099))).powf(v4246)));
                let v4250 = -v389;
                let v4251 = v4250 / v4228;
                let v4265: f64;
                if v259 != 0.0 {
                    let v4252 = v0 - v4251;
                    let v4257 = v273 * (v4251 + (((v4252 * v4252) + v272).sqrt()));
                    v4265 = v4257;
                } else {
                    let v4258 = v0 - v4251;
                    let v4264 = v273 * (v4251 + (v4258 * (((v283 / v272) * v4258).tanh())));
                    v4265 = v4264;
                }
                let v4270 = v4250 * (v5 / ((v5 + (v4265.powf(v4099))).powf(v4246)));
                let v4272 = (v392 - v4136) / v4111;
                let v4273 = if v4272 > v316 { 1.0 } else { 0.0 };
                let v4280: f64;
                if v4273 != 0.0 {
                    v4280 = v0;
                } else {
                    let v4275 = if v4272 < v4274 { 1.0 } else { 0.0 };
                    let v4281: f64;
                    if v4275 != 0.0 {
                        v4281 = v5;
                    } else {
                        let v4278 = v5 / (v5 + (v4272.exp()));
                        v4281 = v4278;
                    }
                    v4280 = v4281;
                }
                let v4285 = ((v4110 - v4270) - (v4130 - (v4178 * v4280))) / v4132;
                let v4286 = if v4285 > v316 { 1.0 } else { 0.0 };
                if v4286 != 0.0 {
                } else {
                    let v4288 = if v4285 < v4287 { 1.0 } else { 0.0 };
                    if v4288 != 0.0 {
                    } else {
                    }
                }
                let v4290 = (v4110 - v4136) / v4111;
                let v4291 = if v4290 > v316 { 1.0 } else { 0.0 };
                let v4298: f64;
                if v4291 != 0.0 {
                    v4298 = v0;
                } else {
                    let v4293 = if v4290 < v4292 { 1.0 } else { 0.0 };
                    let v4299: f64;
                    if v4293 != 0.0 {
                        v4299 = v5;
                    } else {
                        let v4296 = v5 / (v5 + (v4290.exp()));
                        v4299 = v4296;
                    }
                    v4298 = v4299;
                }
                let v4303 = ((v392 - v4249) - (v4130 - (v4178 * v4298))) / v4132;
                let v4304 = if v4303 > v316 { 1.0 } else { 0.0 };
                if v4304 != 0.0 {
                } else {
                    let v4306 = if v4303 < v4305 { 1.0 } else { 0.0 };
                    if v4306 != 0.0 {
                    } else {
                    }
                }
                if v259 != 0.0 {
                } else {
                }
                let v4308 = (v324 * v4113) * v89;
                let v4309 = v382 * v4308;
                let v4310 = v4117 - v4135;
                let v4326: f64;
                if v259 != 0.0 {
                    let v4312 = v392 - v4110;
                    let v4317 = v273 * ((v392 + v4110) + (((v4312 * v4312) + v272).sqrt()));
                    v4326 = v4317;
                } else {
                    let v4319 = v392 - v4110;
                    let v4325 = v273 * ((v392 + v4110) + (v4319 * (((v283 / v272) * v4319).tanh())));
                    v4326 = v4325;
                }
                let v4328 = (v4326 - v4310) / v4111;
                let v4329 = if v4328 > v316 { 1.0 } else { 0.0 };
                let v4351: f64;
                if v4329 != 0.0 {
                    v4351 = v0;
                } else {
                    let v4331 = if v4328 < v4330 { 1.0 } else { 0.0 };
                    let v4352: f64;
                    if v4331 != 0.0 {
                        v4352 = v5;
                    } else {
                        let v4334 = v5 / (v5 + (v4328.exp()));
                        v4352 = v4334;
                    }
                    v4351 = v4352;
                }
                let v4350: f64;
                if v259 != 0.0 {
                    let v4336 = v392 - v4110;
                    let v4341 = v273 * ((v392 + v4110) + (((v4336 * v4336) + v272).sqrt()));
                    v4350 = v4341;
                } else {
                    let v4343 = v392 - v4110;
                    let v4349 = v273 * ((v392 + v4110) + (v4343 * (((v283 / v272) * v4343).tanh())));
                    v4350 = v4349;
                }
                let v4356 = (v4350 - (v4117 - (v4178 * v4351))) / v4308;
                let v4357 = if v4356 > v316 { 1.0 } else { 0.0 };
                let v4370: f64;
                if v4357 != 0.0 {
                    let v4358 = v4309 * v4356;
                    v4370 = v4358;
                } else {
                    let v4360 = if v4356 < v4359 { 1.0 } else { 0.0 };
                    let v4371: f64;
                    if v4360 != 0.0 {
                        let v4362 = v4309 * (v4356.exp());
                        v4371 = v4362;
                    } else {
                        let v4366 = v4309 * ((v5 + (v4356.exp())).ln());
                        v4371 = v4366;
                    }
                    v4370 = v4371;
                }
                let v4369 = (v4207 * v35) / (v4098 / v4118);
                let v4382 = (((v4369 * ((v5 + (((v324 * v4370) / v382) / v4369)).sqrt())) - v4369) * (v5 - v4351)) + (v4308 * v4351);
                let v4383 = v389 / v4382;
                let v4397: f64;
                if v259 != 0.0 {
                    let v4384 = v0 - v4383;
                    let v4389 = v273 * (v4383 + (((v4384 * v4384) + v272).sqrt()));
                    v4397 = v4389;
                } else {
                    let v4390 = v0 - v4383;
                    let v4396 = v273 * (v4383 + (v4390 * (((v283 / v272) * v4390).tanh())));
                    v4397 = v4396;
                }
                let v4402 = v389 * (v5 / ((v5 + (v4397.powf(v4099))).powf(v4246)));
                let v4403 = v4250 / v4382;
                let v4417: f64;
                if v259 != 0.0 {
                    let v4404 = v0 - v4403;
                    let v4409 = v273 * (v4403 + (((v4404 * v4404) + v272).sqrt()));
                    v4417 = v4409;
                } else {
                    let v4410 = v0 - v4403;
                    let v4416 = v273 * (v4403 + (v4410 * (((v283 / v272) * v4410).tanh())));
                    v4417 = v4416;
                }
                let v4422 = v4250 * (v5 / ((v5 + (v4417.powf(v4099))).powf(v4246)));
                let v4424 = (v392 - v4310) / v4111;
                let v4425 = if v4424 > v316 { 1.0 } else { 0.0 };
                let v4432: f64;
                if v4425 != 0.0 {
                    v4432 = v0;
                } else {
                    let v4427 = if v4424 < v4426 { 1.0 } else { 0.0 };
                    let v4433: f64;
                    if v4427 != 0.0 {
                        v4433 = v5;
                    } else {
                        let v4430 = v5 / (v5 + (v4424.exp()));
                        v4433 = v4430;
                    }
                    v4432 = v4433;
                }
                let v4437 = ((v4110 - v4422) - (v4117 - (v4178 * v4432))) / v4308;
                let v4438 = if v4437 > v316 { 1.0 } else { 0.0 };
                if v4438 != 0.0 {
                } else {
                    let v4440 = if v4437 < v4439 { 1.0 } else { 0.0 };
                    if v4440 != 0.0 {
                    } else {
                    }
                }
                let v4442 = (v4110 - v4310) / v4111;
                let v4443 = if v4442 > v316 { 1.0 } else { 0.0 };
                let v4450: f64;
                if v4443 != 0.0 {
                    v4450 = v0;
                } else {
                    let v4445 = if v4442 < v4444 { 1.0 } else { 0.0 };
                    let v4451: f64;
                    if v4445 != 0.0 {
                        v4451 = v5;
                    } else {
                        let v4448 = v5 / (v5 + (v4442.exp()));
                        v4451 = v4448;
                    }
                    v4450 = v4451;
                }
                let v4455 = ((v392 - v4402) - (v4117 - (v4178 * v4450))) / v4308;
                let v4456 = if v4455 > v316 { 1.0 } else { 0.0 };
                if v4456 != 0.0 {
                } else {
                    let v4458 = if v4455 < v4457 { 1.0 } else { 0.0 };
                    if v4458 != 0.0 {
                    } else {
                    }
                }
                if v4459 != 0.0 {
                    let v4464 = (v0 - (v4117 - ((v568 * v273) * v4111))) / v4308;
                    let v4465 = if v4464 > v316 { 1.0 } else { 0.0 };
                    if v4465 != 0.0 {
                    } else {
                        let v4467 = if v4464 < v4466 { 1.0 } else { 0.0 };
                        if v4467 != 0.0 {
                        } else {
                        }
                    }
                    if v4465 != 0.0 {
                    } else {
                        let v4469 = if v4464 < v4468 { 1.0 } else { 0.0 };
                        if v4469 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                if v4470 != 0.0 {
                    let v4475 = (v392 - (v4117 - ((v568 * v273) * v4111))) / v4308;
                    let v4476 = if v4475 > v316 { 1.0 } else { 0.0 };
                    if v4476 != 0.0 {
                    } else {
                        let v4478 = if v4475 < v4477 { 1.0 } else { 0.0 };
                        if v4478 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            let v4499: f64;
            let v6575: Lanes<2>;
            if v259 != 0.0 {
                let v6641 = v6603 * v254;
                let v4494 = ((v254 * v254) + v272).sqrt();
                let v6645 = (v6641 + v6641) * (v6561 / (v6608 * v4494));
                v4499 = v4494;
                v6575 = v6645;
            } else {
                let v4495 = v283 / v272;
                let v4497 = (v4495 * v254).tanh();
                let v4498 = v254 * v4497;
                let v6640 = (v6603 * v4497) + (((v6603 * v4495) * (v6561 - (v4497 * v4497))) * v254);
                v4499 = v4498;
                v6575 = v6640;
            }
            let v4500 = v257 - v254;
            let v6646 = Lanes([0.0, v6607[0], v6607[1]]);
            let v6648 = v6646 - (Lanes([v6603[0], 0.0, v6603[1]]));
            let v4501 = v4485 * v89;
            let v6649 = v6596 * v4485;
            let v4502 = v545 * v89;
            let v4503 = v4481 / v4502;
            let v6654 = v6575 * v4484;
            let v4505 = v4503 + (v4484 * v4499);
            let v6657 = (Lanes([((((v6596 * v545) * v4503) * v6609) / v4502), 0.0, 0.0])) + (Lanes([0.0, v6654[0], v6654[1]]));
            let v6658 = v6570 * v3713;
            let v4507 = v4480 + (v3713 * v91);
            let v4508 = v95.powf(v532);
            let v6662 = v6597 * (v532 * (v95.powf((v532 - v6561))));
            let v4509 = if v531 != v0 { 1.0 } else { 0.0 };
            let v4516: f64;
            let v6576: Lanes<2>;
            if v4509 != 0.0 {
                let v4510 = v4499 / v531;
                let v4512 = v5 + (v4510.powf(v4487));
                let v4513 = v5 / v4487;
                let v4514 = v4512.powf(v4513);
                let v4515 = v4499 / v4514;
                let v6675 = (v6575 - ((((v6575 / v531) * (v4487 * (v4510.powf((v4487 - v6561))))) * (v4513 * (v4512.powf((v4513 - v6561))))) * v4515)) / v4514;
                v4516 = v4515;
                v6576 = v6675;
            } else {
                v4516 = v0;
                v6576 = v6663;
            }
            let v4518 = v4482 - (v4516 * v4483);
            let v6680 = (((v6576 * v4483) * v6609) * v4499) + (v6575 * v4518);
            let v4520 = v4507 - (v4518 * v4499);
            let v6683 = (Lanes([v6658, 0.0, 0.0])) - (Lanes([0.0, v6680[0], v6680[1]]));
            let v4521 = v324 * v4505;
            let v4522 = v4521 * v89;
            let v6688 = ((v6657 * v324) * v89) + (Lanes([(v6596 * v4521), 0.0, 0.0]));
            let v4523 = v129 * v4522;
            let v6692 = (Lanes([(v6599 * v4522), 0.0, 0.0])) + (v6688 * v129);
            let v4525 = (v568 * v4501) / v324;
            let v4526 = v4520 - v4525;
            let v6696 = v6683 - (Lanes([((v6649 * v568) / v324), 0.0, 0.0]));
            let v4542: f64;
            let v6577: Lanes<3>;
            if v259 != 0.0 {
                let v4528 = v257 - v4500;
                let v6710 = (v6646 - v6648) * v4528;
                let v4531 = ((v4528 * v4528) + v272).sqrt();
                let v4533 = v273 * ((v257 + v4500) + v4531);
                let v6716 = ((v6646 + v6648) + ((v6710 + v6710) * (v6561 / (v6608 * v4531)))) * v273;
                v4542 = v4533;
                v6577 = v6716;
            } else {
                let v4535 = v257 - v4500;
                let v6698 = v6646 - v6648;
                let v4536 = v283 / v272;
                let v4538 = (v4536 * v4535).tanh();
                let v4541 = v273 * ((v257 + v4500) + (v4535 * v4538));
                let v6707 = ((v6646 + v6648) + ((v6698 * v4538) + (((v6698 * v4536) * (v6561 - (v4538 * v4538))) * v4535))) * v273;
                v4542 = v4541;
                v6577 = v6707;
            }
            let v6718 = Lanes([v6696[0], v6696[1], 0.0, v6696[2]]);
            let v4544 = (v4542 - v4526) / v4501;
            let v6723 = (((Lanes([0.0, v6577[0], v6577[1], v6577[2]])) - v6718) - (Lanes([(v6649 * v4544), 0.0, 0.0, 0.0]))) / v4501;
            let v4545 = if v4544 > v316 { 1.0 } else { 0.0 };
            let v4569: f64;
            let v6578: Lanes<4>;
            if v4545 != 0.0 {
                v4569 = v0;
                v6578 = v6728;
            } else {
                let v4547 = if v4544 < v4546 { 1.0 } else { 0.0 };
                let v4570: f64;
                let v6579: Lanes<4>;
                if v4547 != 0.0 {
                    v4570 = v5;
                    v6579 = v6728;
                } else {
                    let v4548 = v4544.exp();
                    let v4549 = v5 + v4548;
                    let v4550 = v5 / v4549;
                    let v6727 = (((v6723 * v4548) * v4550) * v6609) / v4549;
                    v4570 = v4550;
                    v6579 = v6727;
                }
                v4569 = v4570;
                v6578 = v6579;
            }
            let v4566: f64;
            let v6580: Lanes<3>;
            if v259 != 0.0 {
                let v4552 = v257 - v4500;
                let v6742 = (v6646 - v6648) * v4552;
                let v4555 = ((v4552 * v4552) + v272).sqrt();
                let v4557 = v273 * ((v257 + v4500) + v4555);
                let v6748 = ((v6646 + v6648) + ((v6742 + v6742) * (v6561 / (v6608 * v4555)))) * v273;
                v4566 = v4557;
                v6580 = v6748;
            } else {
                let v4559 = v257 - v4500;
                let v6730 = v6646 - v6648;
                let v4560 = v283 / v272;
                let v4562 = (v4560 * v4559).tanh();
                let v4565 = v273 * ((v257 + v4500) + (v4559 * v4562));
                let v6739 = ((v6646 + v6648) + ((v6730 * v4562) + (((v6730 * v4560) * (v6561 - (v4562 * v4562))) * v4559))) * v273;
                v4566 = v4565;
                v6580 = v6739;
            }
            let v4567 = v568 * v56;
            let v4568 = v4567 * v4501;
            let v6749 = v6649 * v4567;
            let v6754 = Lanes([v6683[0], v6683[1], 0.0, v6683[2]]);
            let v4574 = (v4566 - (v4520 - (v4568 * v4569))) / v4522;
            let v6758 = v6688 * v4574;
            let v6761 = (((Lanes([0.0, v6580[0], v6580[1], v6580[2]])) - (v6754 - ((Lanes([(v6749 * v4569), 0.0, 0.0, 0.0])) + (v6578 * v4568)))) - (Lanes([v6758[0], v6758[1], 0.0, v6758[2]]))) / v4522;
            let v4575 = if v4574 > v316 { 1.0 } else { 0.0 };
            let v4585: f64;
            let v6581: Lanes<4>;
            if v4575 != 0.0 {
                let v4576 = v4523 * v4574;
                let v6774 = v6692 * v4574;
                let v6777 = (Lanes([v6774[0], v6774[1], 0.0, v6774[2]])) + (v6761 * v4523);
                v4585 = v4576;
                v6581 = v6777;
            } else {
                let v4578 = if v4574 < v4577 { 1.0 } else { 0.0 };
                let v4586: f64;
                let v6582: Lanes<4>;
                if v4578 != 0.0 {
                    let v4579 = v4574.exp();
                    let v4580 = v4523 * v4579;
                    let v6770 = v6692 * v4579;
                    let v6773 = (Lanes([v6770[0], v6770[1], 0.0, v6770[2]])) + ((v6761 * v4579) * v4523);
                    v4586 = v4580;
                    v6582 = v6773;
                } else {
                    let v4581 = v4574.exp();
                    let v4582 = v5 + v4581;
                    let v4583 = v4582.ln();
                    let v4584 = v4523 * v4583;
                    let v6765 = v6692 * v4583;
                    let v6768 = (Lanes([v6765[0], v6765[1], 0.0, v6765[2]])) + (((v6761 * v4581) * (v6561 / v4582)) * v4523);
                    v4586 = v4584;
                    v6582 = v6768;
                }
                v4585 = v4586;
                v6581 = v6582;
            }
            let v4588 = (v4488 * v4585) / v129;
            let v4589 = v5 + v4588;
            let v4590 = v4508 * v4589;
            let v4591 = v294 / v4590;
            let v6789 = ((((Lanes([(v6662 * v4589), 0.0, 0.0, 0.0])) + ((((v6581 * v4488) - (Lanes([(v6599 * v4588), 0.0, 0.0, 0.0]))) / v129) * v4508)) * v4591) * v6609) / v4590;
            let v4595 = v5 + (v533 * v46);
            let v4596 = (v5 + (v533 * v3)) / v4595;
            let v4597 = v4486 * v4596;
            let v4600 = v5 + ((v534 * v4499) / v4479);
            let v6798 = ((v6575 * v534) / v4479) * v4597;
            let v6801 = (Lanes([((((((v6570 * v533) * v4596) * v6609) / v4595) * v4486) * v4600), 0.0, 0.0])) + (Lanes([0.0, v6798[0], v6798[1]]));
            let v4603 = (v4489 * v4585) / v129;
            let v4604 = v5 + v4603;
            let v4605 = (v4597 * v4600) / v4604;
            let v6810 = ((Lanes([v6801[0], v6801[1], 0.0, v6801[2]])) - ((((v6581 * v4489) - (Lanes([(v6599 * v4603), 0.0, 0.0, 0.0]))) / v129) * v4605)) / v4604;
            let v4606 = v324 * v4569;
            let v4607 = v4606 * v89;
            let v4610 = v5 - v4569;
            let v6820 = v6578 * v6609;
            let v4612 = ((v4607 * v4591) / v4479) + (v4610 * v4605);
            let v6824 = ((((((v6578 * v324) * v89) + (Lanes([(v6596 * v4606), 0.0, 0.0, 0.0]))) * v4591) + (v6789 * v4607)) / v4479) + ((v6820 * v4605) + (v6810 * v4610));
            let v4614 = (v4605 * v4479) / v4591;
            let v6828 = ((v6810 * v4479) - (v6789 * v4614)) / v4591;
            let v4616 = (v324 * v4585) / v129;
            let v4617 = v4616 / v4614;
            let v4619 = (v5 + v4617).sqrt();
            let v4621 = (v4614 * v4619) - v4614;
            let v4623 = v4522 * v4569;
            let v6847 = v6688 * v4569;
            let v6850 = (Lanes([v6847[0], v6847[1], 0.0, v6847[2]])) + (v6578 * v4522);
            let v4624 = (v4614 * v4610) + v4623;
            let v6851 = ((v6828 * v4610) + (v6820 * v4614)) + v6850;
            let v4626 = (v4621 * v4610) + v4623;
            let v6855 = (((((v6828 * v4619) + (((((((v6581 * v324) - (Lanes([(v6599 * v4616), 0.0, 0.0, 0.0]))) / v129) - (v6828 * v4617)) / v4614) * (v6561 / (v6608 * v4619))) * v4614)) - v6828) * v4610) + (v6820 * v4621)) + v6850;
            let v4627 = v254 / v4626;
            let v6859 = ((Lanes([0.0, v6603[0], 0.0, v6603[1]])) - (v6855 * v4627)) / v4626;
            let v4641: f64;
            let v6583: Lanes<4>;
            if v259 != 0.0 {
                let v4628 = v0 - v4627;
                let v6871 = (v6859 * v6609) * v4628;
                let v4631 = ((v4628 * v4628) + v272).sqrt();
                let v4633 = v273 * (v4627 + v4631);
                let v6877 = (v6859 + ((v6871 + v6871) * (v6561 / (v6608 * v4631)))) * v273;
                v4641 = v4633;
                v6583 = v6877;
            } else {
                let v4634 = v0 - v4627;
                let v6860 = v6859 * v6609;
                let v4635 = v283 / v272;
                let v4637 = (v4635 * v4634).tanh();
                let v4640 = v273 * (v4627 + (v4634 * v4637));
                let v6869 = (v6859 + ((v6860 * v4637) + (((v6860 * v4635) * (v6561 - (v4637 * v4637))) * v4634))) * v273;
                v4641 = v4640;
                v6583 = v6869;
            }
            let v6878 = v4487 - v6561;
            let v4643 = v5 + (v4641.powf(v4487));
            let v4644 = v5 / v4487;
            let v4645 = v4643.powf(v4644);
            let v6882 = v4644 - v6561;
            let v4646 = v5 / v4645;
            let v4647 = v254 * v4646;
            let v6889 = v6603 * v4646;
            let v6892 = (Lanes([0.0, v6889[0], 0.0, v6889[1]])) + ((((((v6583 * (v4487 * (v4641.powf(v6878)))) * (v4644 * (v4643.powf(v6882)))) * v4646) * v6609) / v4645) * v254);
            let v4648 = -v254;
            let v6893 = v6603 * v6609;
            let v4649 = v4648 / v4626;
            let v6897 = ((Lanes([0.0, v6893[0], 0.0, v6893[1]])) - (v6855 * v4649)) / v4626;
            let v4663: f64;
            let v6584: Lanes<4>;
            if v259 != 0.0 {
                let v4650 = v0 - v4649;
                let v6909 = (v6897 * v6609) * v4650;
                let v4653 = ((v4650 * v4650) + v272).sqrt();
                let v4655 = v273 * (v4649 + v4653);
                let v6915 = (v6897 + ((v6909 + v6909) * (v6561 / (v6608 * v4653)))) * v273;
                v4663 = v4655;
                v6584 = v6915;
            } else {
                let v4656 = v0 - v4649;
                let v6898 = v6897 * v6609;
                let v4657 = v283 / v272;
                let v4659 = (v4657 * v4656).tanh();
                let v4662 = v273 * (v4649 + (v4656 * v4659));
                let v6907 = (v6897 + ((v6898 * v4659) + (((v6898 * v4657) * (v6561 - (v4659 * v4659))) * v4656))) * v273;
                v4663 = v4662;
                v6584 = v6907;
            }
            let v4665 = v5 + (v4663.powf(v4487));
            let v4666 = v4665.powf(v4644);
            let v4667 = v5 / v4666;
            let v4668 = v4648 * v4667;
            let v6925 = v6893 * v4667;
            let v6928 = (Lanes([0.0, v6925[0], 0.0, v6925[1]])) + ((((((v6584 * (v4487 * (v4663.powf(v6878)))) * (v4644 * (v4665.powf(v6882)))) * v4667) * v6609) / v4666) * v4648);
            let v6929 = Lanes([0.0, 0.0, v6607[0], v6607[1]]);
            let v4670 = (v257 - v4526) / v4501;
            let v6934 = ((v6929 - v6718) - (Lanes([(v6649 * v4670), 0.0, 0.0, 0.0]))) / v4501;
            let v4671 = if v4670 > v316 { 1.0 } else { 0.0 };
            let v4678: f64;
            let v6585: Lanes<4>;
            if v4671 != 0.0 {
                v4678 = v0;
                v6585 = v6728;
            } else {
                let v4673 = if v4670 < v4672 { 1.0 } else { 0.0 };
                let v4679: f64;
                let v6586: Lanes<4>;
                if v4673 != 0.0 {
                    v4679 = v5;
                    v6586 = v6728;
                } else {
                    let v4674 = v4670.exp();
                    let v4675 = v5 + v4674;
                    let v4676 = v5 / v4675;
                    let v6938 = (((v6934 * v4674) * v4676) * v6609) / v4675;
                    v4679 = v4676;
                    v6586 = v6938;
                }
                v4678 = v4679;
                v6585 = v6586;
            }
            let v6939 = Lanes([0.0, v6648[0], v6648[1], v6648[2]]);
            let v4683 = ((v4500 - v4668) - (v4520 - (v4568 * v4678))) / v4522;
            let v6947 = v6688 * v4683;
            let v6950 = (((v6939 - v6928) - (v6754 - ((Lanes([(v6749 * v4678), 0.0, 0.0, 0.0])) + (v6585 * v4568)))) - (Lanes([v6947[0], v6947[1], 0.0, v6947[2]]))) / v4522;
            let v4684 = if v4683 > v316 { 1.0 } else { 0.0 };
            let v4719: f64;
            let v6587: Lanes<4>;
            if v4684 != 0.0 {
                let v4685 = v4523 * v4683;
                let v6963 = v6692 * v4683;
                let v6966 = (Lanes([v6963[0], v6963[1], 0.0, v6963[2]])) + (v6950 * v4523);
                v4719 = v4685;
                v6587 = v6966;
            } else {
                let v4687 = if v4683 < v4686 { 1.0 } else { 0.0 };
                let v4720: f64;
                let v6588: Lanes<4>;
                if v4687 != 0.0 {
                    let v4688 = v4683.exp();
                    let v4689 = v4523 * v4688;
                    let v6959 = v6692 * v4688;
                    let v6962 = (Lanes([v6959[0], v6959[1], 0.0, v6959[2]])) + ((v6950 * v4688) * v4523);
                    v4720 = v4689;
                    v6588 = v6962;
                } else {
                    let v4690 = v4683.exp();
                    let v4691 = v5 + v4690;
                    let v4692 = v4691.ln();
                    let v4693 = v4523 * v4692;
                    let v6954 = v6692 * v4692;
                    let v6957 = (Lanes([v6954[0], v6954[1], 0.0, v6954[2]])) + (((v6950 * v4690) * (v6561 / v4691)) * v4523);
                    v4720 = v4693;
                    v6588 = v6957;
                }
                v4719 = v4720;
                v6587 = v6588;
            }
            let v4695 = (v4500 - v4526) / v4501;
            let v6971 = ((v6939 - v6718) - (Lanes([(v6649 * v4695), 0.0, 0.0, 0.0]))) / v4501;
            let v4696 = if v4695 > v316 { 1.0 } else { 0.0 };
            let v4703: f64;
            let v6589: Lanes<4>;
            if v4696 != 0.0 {
                v4703 = v0;
                v6589 = v6728;
            } else {
                let v4698 = if v4695 < v4697 { 1.0 } else { 0.0 };
                let v4704: f64;
                let v6590: Lanes<4>;
                if v4698 != 0.0 {
                    v4704 = v5;
                    v6590 = v6728;
                } else {
                    let v4699 = v4695.exp();
                    let v4700 = v5 + v4699;
                    let v4701 = v5 / v4700;
                    let v6975 = (((v6971 * v4699) * v4701) * v6609) / v4700;
                    v4704 = v4701;
                    v6590 = v6975;
                }
                v4703 = v4704;
                v6589 = v6590;
            }
            let v4708 = ((v257 - v4647) - (v4520 - (v4568 * v4703))) / v4522;
            let v6983 = v6688 * v4708;
            let v6986 = (((v6929 - v6892) - (v6754 - ((Lanes([(v6749 * v4703), 0.0, 0.0, 0.0])) + (v6589 * v4568)))) - (Lanes([v6983[0], v6983[1], 0.0, v6983[2]]))) / v4522;
            let v4709 = if v4708 > v316 { 1.0 } else { 0.0 };
            let v4721: f64;
            let v6591: Lanes<4>;
            if v4709 != 0.0 {
                let v4710 = v4523 * v4708;
                let v6999 = v6692 * v4708;
                let v7002 = (Lanes([v6999[0], v6999[1], 0.0, v6999[2]])) + (v6986 * v4523);
                v4721 = v4710;
                v6591 = v7002;
            } else {
                let v4712 = if v4708 < v4711 { 1.0 } else { 0.0 };
                let v4722: f64;
                let v6592: Lanes<4>;
                if v4712 != 0.0 {
                    let v4713 = v4708.exp();
                    let v4714 = v4523 * v4713;
                    let v6995 = v6692 * v4713;
                    let v6998 = (Lanes([v6995[0], v6995[1], 0.0, v6995[2]])) + ((v6986 * v4713) * v4523);
                    v4722 = v4714;
                    v6592 = v6998;
                } else {
                    let v4715 = v4708.exp();
                    let v4716 = v5 + v4715;
                    let v4717 = v4716.ln();
                    let v4718 = v4523 * v4717;
                    let v6990 = v6692 * v4717;
                    let v6993 = (Lanes([v6990[0], v6990[1], 0.0, v6990[2]])) + (((v6986 * v4715) * (v6561 / v4716)) * v4523);
                    v4722 = v4718;
                    v6592 = v6993;
                }
                v4721 = v4722;
                v6591 = v6592;
            }
            let v4724 = (v4719 - v4721) / v129;
            let v4725 = v4724 / v4624;
            let v7010 = ((((v6587 - v6591) - (Lanes([(v6599 * v4724), 0.0, 0.0, 0.0]))) / v129) - (v6851 * v4725)) / v4624;
            let v4733: f64;
            let v6593: Lanes<4>;
            if v259 != 0.0 {
                let v7018 = v7010 * v4725;
                let v4728 = ((v4725 * v4725) + v272).sqrt();
                let v7022 = (v7018 + v7018) * (v6561 / (v6608 * v4728));
                v4733 = v4728;
                v6593 = v7022;
            } else {
                let v4729 = v283 / v272;
                let v4731 = (v4729 * v4725).tanh();
                let v4732 = v4725 * v4731;
                let v7017 = (v7010 * v4731) + (((v7010 * v4729) * (v6561 - (v4731 * v4731))) * v4725);
                v4733 = v4732;
                v6593 = v7017;
            }
            let v4735 = v5 + (v4733.powf(v4487));
            let v4736 = v4735.powf(v4644);
            let v4737 = v4725 / v4736;
            let v4738 = v4612 * v4737;
            let v4740 = (v250 * v20) * v22;
            let v4741 = v4740 * v273;
            let v4743 = v4741 * (v4719 + v4721);
            let v4744 = v4743 * v4738;
            let v4745 = v4744 * v4490;
            let v7040 = ((((v6587 + v6591) * v4741) * v4738) + (((v6824 * v4737) + (((v7010 - (((v6593 * (v4487 * (v4733.powf(v6878)))) * (v4644 * (v4735.powf(v6882)))) * v4737)) / v4736) * v4612)) * v4743)) * v4490;
            let v7041 = v6573 * v4744;
            let v7044 = (Lanes([v7040[0], v7040[1], v7040[2], v7040[3], 0.0, 0.0, 0.0, 0.0])) + (Lanes([0.0, 0.0, 0.0, 0.0, v7041[0], v7041[1], v7041[2], v7041[3]]));
            let v4747 = (v324 * v4503) * v89;
            let v4748 = v129 * v4747;
            let v4749 = v4507 - v4525;
            let v4765: f64;
            if v259 != 0.0 {
                let v4751 = v257 - v4500;
                let v4756 = v273 * ((v257 + v4500) + (((v4751 * v4751) + v272).sqrt()));
                v4765 = v4756;
            } else {
                let v4758 = v257 - v4500;
                let v4764 = v273 * ((v257 + v4500) + (v4758 * (((v283 / v272) * v4758).tanh())));
                v4765 = v4764;
            }
            let v4767 = (v4765 - v4749) / v4501;
            let v4768 = if v4767 > v316 { 1.0 } else { 0.0 };
            let v4790: f64;
            if v4768 != 0.0 {
                v4790 = v0;
            } else {
                let v4770 = if v4767 < v4769 { 1.0 } else { 0.0 };
                let v4791: f64;
                if v4770 != 0.0 {
                    v4791 = v5;
                } else {
                    let v4773 = v5 / (v5 + (v4767.exp()));
                    v4791 = v4773;
                }
                v4790 = v4791;
            }
            let v4789: f64;
            if v259 != 0.0 {
                let v4775 = v257 - v4500;
                let v4780 = v273 * ((v257 + v4500) + (((v4775 * v4775) + v272).sqrt()));
                v4789 = v4780;
            } else {
                let v4782 = v257 - v4500;
                let v4788 = v273 * ((v257 + v4500) + (v4782 * (((v283 / v272) * v4782).tanh())));
                v4789 = v4788;
            }
            let v4795 = (v4789 - (v4507 - (v4568 * v4790))) / v4747;
            let v4796 = if v4795 > v316 { 1.0 } else { 0.0 };
            let v4809: f64;
            if v4796 != 0.0 {
                let v4797 = v4748 * v4795;
                v4809 = v4797;
            } else {
                let v4799 = if v4795 < v4798 { 1.0 } else { 0.0 };
                let v4810: f64;
                if v4799 != 0.0 {
                    let v4801 = v4748 * (v4795.exp());
                    v4810 = v4801;
                } else {
                    let v4805 = v4748 * ((v5 + (v4795.exp())).ln());
                    v4810 = v4805;
                }
                v4809 = v4810;
            }
            let v4808 = (v4597 * v4479) / (v294 / v4508);
            let v4821 = (((v4808 * ((v5 + (((v324 * v4809) / v129) / v4808)).sqrt())) - v4808) * (v5 - v4790)) + (v4747 * v4790);
            let v4822 = v254 / v4821;
            let v4836: f64;
            if v259 != 0.0 {
                let v4823 = v0 - v4822;
                let v4828 = v273 * (v4822 + (((v4823 * v4823) + v272).sqrt()));
                v4836 = v4828;
            } else {
                let v4829 = v0 - v4822;
                let v4835 = v273 * (v4822 + (v4829 * (((v283 / v272) * v4829).tanh())));
                v4836 = v4835;
            }
            let v4841 = v254 * (v5 / ((v5 + (v4836.powf(v4487))).powf(v4644)));
            let v4842 = v4648 / v4821;
            let v4856: f64;
            if v259 != 0.0 {
                let v4843 = v0 - v4842;
                let v4848 = v273 * (v4842 + (((v4843 * v4843) + v272).sqrt()));
                v4856 = v4848;
            } else {
                let v4849 = v0 - v4842;
                let v4855 = v273 * (v4842 + (v4849 * (((v283 / v272) * v4849).tanh())));
                v4856 = v4855;
            }
            let v4861 = v4648 * (v5 / ((v5 + (v4856.powf(v4487))).powf(v4644)));
            let v4863 = (v257 - v4749) / v4501;
            let v4864 = if v4863 > v316 { 1.0 } else { 0.0 };
            let v4871: f64;
            if v4864 != 0.0 {
                v4871 = v0;
            } else {
                let v4866 = if v4863 < v4865 { 1.0 } else { 0.0 };
                let v4872: f64;
                if v4866 != 0.0 {
                    v4872 = v5;
                } else {
                    let v4869 = v5 / (v5 + (v4863.exp()));
                    v4872 = v4869;
                }
                v4871 = v4872;
            }
            let v4876 = ((v4500 - v4861) - (v4507 - (v4568 * v4871))) / v4747;
            let v4877 = if v4876 > v316 { 1.0 } else { 0.0 };
            let v4912: f64;
            if v4877 != 0.0 {
                let v4878 = v4748 * v4876;
                v4912 = v4878;
            } else {
                let v4880 = if v4876 < v4879 { 1.0 } else { 0.0 };
                let v4913: f64;
                if v4880 != 0.0 {
                    let v4882 = v4748 * (v4876.exp());
                    v4913 = v4882;
                } else {
                    let v4886 = v4748 * ((v5 + (v4876.exp())).ln());
                    v4913 = v4886;
                }
                v4912 = v4913;
            }
            let v4888 = (v4500 - v4749) / v4501;
            let v4889 = if v4888 > v316 { 1.0 } else { 0.0 };
            let v4896: f64;
            if v4889 != 0.0 {
                v4896 = v0;
            } else {
                let v4891 = if v4888 < v4890 { 1.0 } else { 0.0 };
                let v4897: f64;
                if v4891 != 0.0 {
                    v4897 = v5;
                } else {
                    let v4894 = v5 / (v5 + (v4888.exp()));
                    v4897 = v4894;
                }
                v4896 = v4897;
            }
            let v4901 = ((v257 - v4841) - (v4507 - (v4568 * v4896))) / v4747;
            let v4902 = if v4901 > v316 { 1.0 } else { 0.0 };
            let v4918: f64;
            if v4902 != 0.0 {
                let v4903 = v4748 * v4901;
                v4918 = v4903;
            } else {
                let v4905 = if v4901 < v4904 { 1.0 } else { 0.0 };
                let v4919: f64;
                if v4905 != 0.0 {
                    let v4907 = v4748 * (v4901.exp());
                    v4919 = v4907;
                } else {
                    let v4911 = v4748 * ((v5 + (v4901.exp())).ln());
                    v4919 = v4911;
                }
                v4918 = v4919;
            }
            let v4915 = (v4912 * v4912) + v894;
            let v4921 = (v4918 * v4918) + v894;
            let v4925 = (v4912 * v4918) + v894;
            let v4927 = v4915 + v4921;
            let v4946 = (v324 * ((((v324 * ((v4915 * v4912) + v895)) + (v96 * ((v4921 * v4918) + v895))) + ((v897 * v4915) * v4918)) + ((v898 * v4921) * v4912))) / (v899 * (v4927 + (v324 * v4925)));
            let v4948 = v20 * v22;
            let v4950 = (v4948 * v4479) * v250;
            let v4952 = (v4950 * (((v4926 * (v4927 + v4925)) / ((v4912 + v4918) + v896)) - v4946)) * v4490;
            let v4954 = (v4950 * v4946) * v4490;
            if v4955 != 0.0 {
                let v4960 = (v0 - (v4507 - ((v568 * v273) * v4501))) / v4747;
                let v4961 = if v4960 > v316 { 1.0 } else { 0.0 };
                if v4961 != 0.0 {
                } else {
                    let v4963 = if v4960 < v4962 { 1.0 } else { 0.0 };
                    if v4963 != 0.0 {
                    } else {
                    }
                }
                if v4961 != 0.0 {
                } else {
                    let v4965 = if v4960 < v4964 { 1.0 } else { 0.0 };
                    if v4965 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            if v4966 != 0.0 {
                let v4971 = (v257 - (v4507 - ((v568 * v273) * v4501))) / v4747;
                let v4972 = if v4971 > v316 { 1.0 } else { 0.0 };
                if v4972 != 0.0 {
                } else {
                    let v4974 = if v4971 < v4973 { 1.0 } else { 0.0 };
                    if v4974 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v4976 = if v4975 == v0 { 1.0 } else { 0.0 };
            if v4976 != 0.0 {
            } else {
            }
            let v4977 = v255 - v251;
            let v4979 = if v4978 == v5 { 1.0 } else { 0.0 };
            let v6392: f64;
            let v6394: f64;
            let v6396: f64;
            let v6405: f64;
            let v6407: f64;
            let v6409: f64;
            let v6416: f64;
            let v6417: f64;
            let v6418: f64;
            let v6424: f64;
            let v6425: f64;
            let v6426: f64;
            if v4979 != 0.0 {
                let v4981 = v250 * (v255 - v298);
                let v4989 = v5 - v4988;
                let v4991 = v4989 * v4990;
                let v4996 = v4989 * v4995;
                let v5002 = (v4998 / v89) * (-v4999);
                let v5003 = if v5002 > v316 { 1.0 } else { 0.0 };
                let v5012: f64;
                if v5003 != 0.0 {
                    let v5007 = v5004 * (v5 + (v5002 - v316));
                    v5012 = v5007;
                } else {
                    let v5009 = if v5002 < v5008 { 1.0 } else { 0.0 };
                    let v5013: f64;
                    if v5009 != 0.0 {
                        v5013 = v5010;
                    } else {
                        let v5011 = v5002.exp();
                        v5013 = v5011;
                    }
                    v5012 = v5013;
                }
                let v5014 = -v4981;
                let v5017 = (v4986 * (v5014 - v4987)) + v5002;
                let v5020 = ((-v4986) * v4987) + v5002;
                let v5021 = if v5017 > v316 { 1.0 } else { 0.0 };
                let v5030: f64;
                if v5021 != 0.0 {
                    let v5025 = v5022 * (v5 + (v5017 - v316));
                    v5030 = v5025;
                } else {
                    let v5027 = if v5017 < v5026 { 1.0 } else { 0.0 };
                    let v5031: f64;
                    if v5027 != 0.0 {
                        v5031 = v5028;
                    } else {
                        let v5029 = v5017.exp();
                        v5031 = v5029;
                    }
                    v5030 = v5031;
                }
                let v5032 = if v5020 > v316 { 1.0 } else { 0.0 };
                let v5041: f64;
                if v5032 != 0.0 {
                    let v5036 = v5033 * (v5 + (v5020 - v316));
                    v5041 = v5036;
                } else {
                    let v5038 = if v5020 < v5037 { 1.0 } else { 0.0 };
                    let v5042: f64;
                    if v5038 != 0.0 {
                        v5042 = v5039;
                    } else {
                        let v5040 = v5020.exp();
                        v5042 = v5040;
                    }
                    v5041 = v5042;
                }
                let v5043 = v5030 - v5041;
                let v5045 = (v4740 * v4991) * v97;
                let v5046 = v4985 / v89;
                let v5048 = (v5046 * v4981) + v5002;
                let v5049 = if v5048 > v316 { 1.0 } else { 0.0 };
                let v5058: f64;
                if v5049 != 0.0 {
                    let v5053 = v5050 * (v5 + (v5048 - v316));
                    v5058 = v5053;
                } else {
                    let v5055 = if v5048 < v5054 { 1.0 } else { 0.0 };
                    let v5059: f64;
                    if v5055 != 0.0 {
                        v5059 = v5056;
                    } else {
                        let v5057 = v5048.exp();
                        v5059 = v5057;
                    }
                    v5058 = v5059;
                }
                let v5060 = if v4984 == v5 { 1.0 } else { 0.0 };
                let v5191: f64;
                if v5060 != 0.0 {
                    let v5064 = v5045 * ((v5058 - (v4992 * v5043)) - v5012);
                    v5191 = v5064;
                } else {
                    let v5068 = (v4986 * ((-v4982) - v4987)) + v5002;
                    let v5069 = if v5068 > v316 { 1.0 } else { 0.0 };
                    let v5078: f64;
                    if v5069 != 0.0 {
                        let v5073 = v5070 * (v5 + (v5068 - v316));
                        v5078 = v5073;
                    } else {
                        let v5075 = if v5068 < v5074 { 1.0 } else { 0.0 };
                        let v5079: f64;
                        if v5075 != 0.0 {
                            v5079 = v5076;
                        } else {
                            let v5077 = v5068.exp();
                            v5079 = v5077;
                        }
                        v5078 = v5079;
                    }
                    let v5080 = v5078 - v5041;
                    let v5082 = (v5046 * v4982) + v5002;
                    let v5083 = if v5082 > v316 { 1.0 } else { 0.0 };
                    let v5092: f64;
                    if v5083 != 0.0 {
                        let v5087 = v5084 * (v5 + (v5082 - v316));
                        v5092 = v5087;
                    } else {
                        let v5089 = if v5082 < v5088 { 1.0 } else { 0.0 };
                        let v5093: f64;
                        if v5089 != 0.0 {
                            v5093 = v5090;
                        } else {
                            let v5091 = v5082.exp();
                            v5093 = v5091;
                        }
                        v5092 = v5093;
                    }
                    let v5094 = v4992 * v5080;
                    let v5096 = (v5092 - v5094) - v5012;
                    let v5097 = v4992 * v5043;
                    let v5100 = v5045 * ((v5058 - v5097) - v5012);
                    let v5101 = if v4984 > v0 { 1.0 } else { 0.0 };
                    let v5154: f64;
                    if v5101 != 0.0 {
                        let v5103 = (v4984 * v4985) / v89;
                        let v5105 = (v5103 * v4982) + v5002;
                        let v5106 = if v5105 > v316 { 1.0 } else { 0.0 };
                        let v5115: f64;
                        if v5106 != 0.0 {
                            let v5110 = v5107 * (v5 + (v5105 - v316));
                            v5115 = v5110;
                        } else {
                            let v5112 = if v5105 < v5111 { 1.0 } else { 0.0 };
                            let v5116: f64;
                            if v5112 != 0.0 {
                                v5116 = v5113;
                            } else {
                                let v5114 = v5105.exp();
                                v5116 = v5114;
                            }
                            v5115 = v5116;
                        }
                        let v5118 = (v5115 - v5094) - v5012;
                        let v5120 = (v5103 * v4981) + v5002;
                        let v5121 = if v5120 > v316 { 1.0 } else { 0.0 };
                        let v5130: f64;
                        if v5121 != 0.0 {
                            let v5125 = v5122 * (v5 + (v5120 - v316));
                            v5130 = v5125;
                        } else {
                            let v5127 = if v5120 < v5126 { 1.0 } else { 0.0 };
                            let v5131: f64;
                            if v5127 != 0.0 {
                                v5131 = v5128;
                            } else {
                                let v5129 = v5120.exp();
                                v5131 = v5129;
                            }
                            v5130 = v5131;
                        }
                        let v5136 = ((v5045 * v5096) / v5118) * ((v5130 - v5097) - v5012);
                        v5154 = v5136;
                    } else {
                        let v5137 = v5045 * v5096;
                        v5154 = v5137;
                    }
                    let v5139 = (v4983 * v4983) * v89;
                    let v5143 = (v4981 - (v4982 - (v5139 / v324))) / v5139;
                    let v5144 = if v5143 > v316 { 1.0 } else { 0.0 };
                    let v5150: f64;
                    if v5144 != 0.0 {
                        v5150 = v0;
                    } else {
                        let v5146 = if v5143 < v5145 { 1.0 } else { 0.0 };
                        let v5151: f64;
                        if v5146 != 0.0 {
                            v5151 = v5;
                        } else {
                            let v5149 = v5 / (v5 + (v5143.exp()));
                            v5151 = v5149;
                        }
                        v5150 = v5151;
                    }
                    let v5156 = (v5150 * v5100) + ((v5 - v5150) * v5154);
                    v5191 = v5156;
                }
                let v5157 = v4981 / v4993;
                let v5165: f64;
                if v259 != 0.0 {
                    let v5160 = ((v5157 * v5157) + v272).sqrt();
                    v5165 = v5160;
                } else {
                    let v5164 = v5157 * (((v283 / v272) * v5157).tanh());
                    v5165 = v5164;
                }
                let v5168 = v5 / v4994;
                let v5173 = ((-v250) * v20) * v22;
                let v5175 = (v5173 * v4996) * v97;
                let v5176 = v4997 / v89;
                let v5177 = v5176 * (v5014 / ((v5 + (v5165.powf(v4994))).powf(v5168)));
                let v5178 = if v5177 > v316 { 1.0 } else { 0.0 };
                let v5187: f64;
                if v5178 != 0.0 {
                    let v5182 = v5179 * (v5 + (v5177 - v316));
                    v5187 = v5182;
                } else {
                    let v5184 = if v5177 < v5183 { 1.0 } else { 0.0 };
                    let v5188: f64;
                    if v5184 != 0.0 {
                        v5188 = v5185;
                    } else {
                        let v5186 = v5177.exp();
                        v5188 = v5186;
                    }
                    v5187 = v5188;
                }
                let v5192 = v5191 + (v5175 * (v5187 - v5));
                let v5194 = v250 * (v255 - v353);
                let v5202 = v4989 * v5201;
                let v5207 = v4989 * v5206;
                let v5217: f64;
                if v5003 != 0.0 {
                    let v5212 = v5209 * (v5 + (v5002 - v316));
                    v5217 = v5212;
                } else {
                    let v5214 = if v5002 < v5213 { 1.0 } else { 0.0 };
                    let v5218: f64;
                    if v5214 != 0.0 {
                        v5218 = v5215;
                    } else {
                        let v5216 = v5002.exp();
                        v5218 = v5216;
                    }
                    v5217 = v5218;
                }
                let v5219 = -v5194;
                let v5222 = (v5199 * (v5219 - v5200)) + v5002;
                let v5225 = ((-v5199) * v5200) + v5002;
                let v5226 = if v5222 > v316 { 1.0 } else { 0.0 };
                let v5235: f64;
                if v5226 != 0.0 {
                    let v5230 = v5227 * (v5 + (v5222 - v316));
                    v5235 = v5230;
                } else {
                    let v5232 = if v5222 < v5231 { 1.0 } else { 0.0 };
                    let v5236: f64;
                    if v5232 != 0.0 {
                        v5236 = v5233;
                    } else {
                        let v5234 = v5222.exp();
                        v5236 = v5234;
                    }
                    v5235 = v5236;
                }
                let v5237 = if v5225 > v316 { 1.0 } else { 0.0 };
                let v5246: f64;
                if v5237 != 0.0 {
                    let v5241 = v5238 * (v5 + (v5225 - v316));
                    v5246 = v5241;
                } else {
                    let v5243 = if v5225 < v5242 { 1.0 } else { 0.0 };
                    let v5247: f64;
                    if v5243 != 0.0 {
                        v5247 = v5244;
                    } else {
                        let v5245 = v5225.exp();
                        v5247 = v5245;
                    }
                    v5246 = v5247;
                }
                let v5248 = v5235 - v5246;
                let v5250 = (v4740 * v5202) * v97;
                let v5251 = v5198 / v89;
                let v5253 = (v5251 * v5194) + v5002;
                let v5254 = if v5253 > v316 { 1.0 } else { 0.0 };
                let v5263: f64;
                if v5254 != 0.0 {
                    let v5258 = v5255 * (v5 + (v5253 - v316));
                    v5263 = v5258;
                } else {
                    let v5260 = if v5253 < v5259 { 1.0 } else { 0.0 };
                    let v5264: f64;
                    if v5260 != 0.0 {
                        v5264 = v5261;
                    } else {
                        let v5262 = v5253.exp();
                        v5264 = v5262;
                    }
                    v5263 = v5264;
                }
                let v5265 = if v5197 == v5 { 1.0 } else { 0.0 };
                let v5393: f64;
                if v5265 != 0.0 {
                    let v5269 = v5250 * ((v5263 - (v5203 * v5248)) - v5217);
                    v5393 = v5269;
                } else {
                    let v5273 = (v5199 * ((-v5195) - v5200)) + v5002;
                    let v5274 = if v5273 > v316 { 1.0 } else { 0.0 };
                    let v5283: f64;
                    if v5274 != 0.0 {
                        let v5278 = v5275 * (v5 + (v5273 - v316));
                        v5283 = v5278;
                    } else {
                        let v5280 = if v5273 < v5279 { 1.0 } else { 0.0 };
                        let v5284: f64;
                        if v5280 != 0.0 {
                            v5284 = v5281;
                        } else {
                            let v5282 = v5273.exp();
                            v5284 = v5282;
                        }
                        v5283 = v5284;
                    }
                    let v5285 = v5283 - v5246;
                    let v5287 = (v5251 * v5195) + v5002;
                    let v5288 = if v5287 > v316 { 1.0 } else { 0.0 };
                    let v5297: f64;
                    if v5288 != 0.0 {
                        let v5292 = v5289 * (v5 + (v5287 - v316));
                        v5297 = v5292;
                    } else {
                        let v5294 = if v5287 < v5293 { 1.0 } else { 0.0 };
                        let v5298: f64;
                        if v5294 != 0.0 {
                            v5298 = v5295;
                        } else {
                            let v5296 = v5287.exp();
                            v5298 = v5296;
                        }
                        v5297 = v5298;
                    }
                    let v5299 = v5203 * v5285;
                    let v5301 = (v5297 - v5299) - v5217;
                    let v5302 = v5203 * v5248;
                    let v5305 = v5250 * ((v5263 - v5302) - v5217);
                    let v5306 = if v5197 > v0 { 1.0 } else { 0.0 };
                    let v5359: f64;
                    if v5306 != 0.0 {
                        let v5308 = (v5197 * v5198) / v89;
                        let v5310 = (v5308 * v5195) + v5002;
                        let v5311 = if v5310 > v316 { 1.0 } else { 0.0 };
                        let v5320: f64;
                        if v5311 != 0.0 {
                            let v5315 = v5312 * (v5 + (v5310 - v316));
                            v5320 = v5315;
                        } else {
                            let v5317 = if v5310 < v5316 { 1.0 } else { 0.0 };
                            let v5321: f64;
                            if v5317 != 0.0 {
                                v5321 = v5318;
                            } else {
                                let v5319 = v5310.exp();
                                v5321 = v5319;
                            }
                            v5320 = v5321;
                        }
                        let v5323 = (v5320 - v5299) - v5217;
                        let v5325 = (v5308 * v5194) + v5002;
                        let v5326 = if v5325 > v316 { 1.0 } else { 0.0 };
                        let v5335: f64;
                        if v5326 != 0.0 {
                            let v5330 = v5327 * (v5 + (v5325 - v316));
                            v5335 = v5330;
                        } else {
                            let v5332 = if v5325 < v5331 { 1.0 } else { 0.0 };
                            let v5336: f64;
                            if v5332 != 0.0 {
                                v5336 = v5333;
                            } else {
                                let v5334 = v5325.exp();
                                v5336 = v5334;
                            }
                            v5335 = v5336;
                        }
                        let v5341 = ((v5250 * v5301) / v5323) * ((v5335 - v5302) - v5217);
                        v5359 = v5341;
                    } else {
                        let v5342 = v5250 * v5301;
                        v5359 = v5342;
                    }
                    let v5344 = (v5196 * v5196) * v89;
                    let v5348 = (v5194 - (v5195 - (v5344 / v324))) / v5344;
                    let v5349 = if v5348 > v316 { 1.0 } else { 0.0 };
                    let v5355: f64;
                    if v5349 != 0.0 {
                        v5355 = v0;
                    } else {
                        let v5351 = if v5348 < v5350 { 1.0 } else { 0.0 };
                        let v5356: f64;
                        if v5351 != 0.0 {
                            v5356 = v5;
                        } else {
                            let v5354 = v5 / (v5 + (v5348.exp()));
                            v5356 = v5354;
                        }
                        v5355 = v5356;
                    }
                    let v5361 = (v5355 * v5305) + ((v5 - v5355) * v5359);
                    v5393 = v5361;
                }
                let v5362 = v5194 / v5204;
                let v5370: f64;
                if v259 != 0.0 {
                    let v5365 = ((v5362 * v5362) + v272).sqrt();
                    v5370 = v5365;
                } else {
                    let v5369 = v5362 * (((v283 / v272) * v5362).tanh());
                    v5370 = v5369;
                }
                let v5373 = v5 / v5205;
                let v5377 = (v5173 * v5207) * v97;
                let v5378 = v5208 / v89;
                let v5379 = v5378 * (v5219 / ((v5 + (v5370.powf(v5205))).powf(v5373)));
                let v5380 = if v5379 > v316 { 1.0 } else { 0.0 };
                let v5389: f64;
                if v5380 != 0.0 {
                    let v5384 = v5381 * (v5 + (v5379 - v316));
                    v5389 = v5384;
                } else {
                    let v5386 = if v5379 < v5385 { 1.0 } else { 0.0 };
                    let v5390: f64;
                    if v5386 != 0.0 {
                        v5390 = v5387;
                    } else {
                        let v5388 = v5379.exp();
                        v5390 = v5388;
                    }
                    v5389 = v5390;
                }
                let v5394 = v5393 + (v5377 * (v5389 - v5));
                let v5396 = if v5395 == v5 { 1.0 } else { 0.0 };
                if v5396 != 0.0 {
                    if v5003 != 0.0 {
                    } else {
                        let v5401 = if v5002 < v5400 { 1.0 } else { 0.0 };
                        if v5401 != 0.0 {
                        } else {
                        }
                    }
                    if v5021 != 0.0 {
                    } else {
                        let v5403 = if v5017 < v5402 { 1.0 } else { 0.0 };
                        if v5403 != 0.0 {
                        } else {
                        }
                    }
                    if v5032 != 0.0 {
                    } else {
                        let v5405 = if v5020 < v5404 { 1.0 } else { 0.0 };
                        if v5405 != 0.0 {
                        } else {
                        }
                    }
                    if v5049 != 0.0 {
                    } else {
                        let v5407 = if v5048 < v5406 { 1.0 } else { 0.0 };
                        if v5407 != 0.0 {
                        } else {
                        }
                    }
                    if v5408 != 0.0 {
                    } else {
                        let v5412 = (v4986 * ((-v4982) - v4987)) + v5002;
                        let v5413 = if v5412 > v316 { 1.0 } else { 0.0 };
                        if v5413 != 0.0 {
                        } else {
                            let v5415 = if v5412 < v5414 { 1.0 } else { 0.0 };
                            if v5415 != 0.0 {
                            } else {
                            }
                        }
                        let v5417 = (v5046 * v4982) + v5002;
                        let v5418 = if v5417 > v316 { 1.0 } else { 0.0 };
                        if v5418 != 0.0 {
                        } else {
                            let v5420 = if v5417 < v5419 { 1.0 } else { 0.0 };
                            if v5420 != 0.0 {
                            } else {
                            }
                        }
                        if v5421 != 0.0 {
                            if v5418 != 0.0 {
                            } else {
                                let v5423 = if v5417 < v5422 { 1.0 } else { 0.0 };
                                if v5423 != 0.0 {
                                } else {
                                }
                            }
                            if v5049 != 0.0 {
                            } else {
                                let v5425 = if v5048 < v5424 { 1.0 } else { 0.0 };
                                if v5425 != 0.0 {
                                } else {
                                }
                            }
                        } else {
                        }
                        let v5427 = (v4983 * v4983) * v89;
                        let v5431 = (v4981 - (v4982 - (v5427 / v324))) / v5427;
                        let v5432 = if v5431 > v316 { 1.0 } else { 0.0 };
                        if v5432 != 0.0 {
                        } else {
                            let v5434 = if v5431 < v5433 { 1.0 } else { 0.0 };
                            if v5434 != 0.0 {
                            } else {
                            }
                        }
                    }
                    let v5435 = v4981 / v5397;
                    let v5443: f64;
                    if v259 != 0.0 {
                        let v5438 = ((v5435 * v5435) + v272).sqrt();
                        v5443 = v5438;
                    } else {
                        let v5442 = v5435 * (((v283 / v272) * v5435).tanh());
                        v5443 = v5442;
                    }
                    let v5450 = (v5399 / v89) * (v5014 / ((v5 + (v5443.powf(v5398))).powf((v5 / v5398))));
                    let v5451 = if v5450 > v316 { 1.0 } else { 0.0 };
                    if v5451 != 0.0 {
                    } else {
                        let v5453 = if v5450 < v5452 { 1.0 } else { 0.0 };
                        if v5453 != 0.0 {
                        } else {
                        }
                    }
                    if v5003 != 0.0 {
                    } else {
                        let v5458 = if v5002 < v5457 { 1.0 } else { 0.0 };
                        if v5458 != 0.0 {
                        } else {
                        }
                    }
                    if v5226 != 0.0 {
                    } else {
                        let v5460 = if v5222 < v5459 { 1.0 } else { 0.0 };
                        if v5460 != 0.0 {
                        } else {
                        }
                    }
                    if v5237 != 0.0 {
                    } else {
                        let v5462 = if v5225 < v5461 { 1.0 } else { 0.0 };
                        if v5462 != 0.0 {
                        } else {
                        }
                    }
                    if v5254 != 0.0 {
                    } else {
                        let v5464 = if v5253 < v5463 { 1.0 } else { 0.0 };
                        if v5464 != 0.0 {
                        } else {
                        }
                    }
                    if v5465 != 0.0 {
                    } else {
                        let v5469 = (v5199 * ((-v5195) - v5200)) + v5002;
                        let v5470 = if v5469 > v316 { 1.0 } else { 0.0 };
                        if v5470 != 0.0 {
                        } else {
                            let v5472 = if v5469 < v5471 { 1.0 } else { 0.0 };
                            if v5472 != 0.0 {
                            } else {
                            }
                        }
                        let v5474 = (v5251 * v5195) + v5002;
                        let v5475 = if v5474 > v316 { 1.0 } else { 0.0 };
                        if v5475 != 0.0 {
                        } else {
                            let v5477 = if v5474 < v5476 { 1.0 } else { 0.0 };
                            if v5477 != 0.0 {
                            } else {
                            }
                        }
                        if v5478 != 0.0 {
                            if v5475 != 0.0 {
                            } else {
                                let v5480 = if v5474 < v5479 { 1.0 } else { 0.0 };
                                if v5480 != 0.0 {
                                } else {
                                }
                            }
                            if v5254 != 0.0 {
                            } else {
                                let v5482 = if v5253 < v5481 { 1.0 } else { 0.0 };
                                if v5482 != 0.0 {
                                } else {
                                }
                            }
                        } else {
                        }
                        let v5484 = (v5196 * v5196) * v89;
                        let v5488 = (v5194 - (v5195 - (v5484 / v324))) / v5484;
                        let v5489 = if v5488 > v316 { 1.0 } else { 0.0 };
                        if v5489 != 0.0 {
                        } else {
                            let v5491 = if v5488 < v5490 { 1.0 } else { 0.0 };
                            if v5491 != 0.0 {
                            } else {
                            }
                        }
                    }
                    let v5492 = v5194 / v5454;
                    let v5500: f64;
                    if v259 != 0.0 {
                        let v5495 = ((v5492 * v5492) + v272).sqrt();
                        v5500 = v5495;
                    } else {
                        let v5499 = v5492 * (((v283 / v272) * v5492).tanh());
                        v5500 = v5499;
                    }
                    let v5507 = (v5456 / v89) * (v5219 / ((v5 + (v5500.powf(v5455))).powf((v5 / v5455))));
                    let v5508 = if v5507 > v316 { 1.0 } else { 0.0 };
                    if v5508 != 0.0 {
                    } else {
                        let v5510 = if v5507 < v5509 { 1.0 } else { 0.0 };
                        if v5510 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v5511 = if v4988 != v0 { 1.0 } else { 0.0 };
                let v6393: f64;
                let v6395: f64;
                let v6397: f64;
                let v6406: f64;
                let v6408: f64;
                let v6410: f64;
                if v5511 != 0.0 {
                    let v5512 = v4988 * v4990;
                    let v5513 = v4988 * v4995;
                    let v5522: f64;
                    if v5003 != 0.0 {
                        let v5517 = v5514 * (v5 + (v5002 - v316));
                        v5522 = v5517;
                    } else {
                        let v5519 = if v5002 < v5518 { 1.0 } else { 0.0 };
                        let v5523: f64;
                        if v5519 != 0.0 {
                            v5523 = v5520;
                        } else {
                            let v5521 = v5002.exp();
                            v5523 = v5521;
                        }
                        v5522 = v5523;
                    }
                    let v5524 = -v257;
                    let v5527 = (v4986 * (v5524 - v4987)) + v5002;
                    let v5528 = if v5527 > v316 { 1.0 } else { 0.0 };
                    let v5537: f64;
                    if v5528 != 0.0 {
                        let v5532 = v5529 * (v5 + (v5527 - v316));
                        v5537 = v5532;
                    } else {
                        let v5534 = if v5527 < v5533 { 1.0 } else { 0.0 };
                        let v5538: f64;
                        if v5534 != 0.0 {
                            v5538 = v5535;
                        } else {
                            let v5536 = v5527.exp();
                            v5538 = v5536;
                        }
                        v5537 = v5538;
                    }
                    let v5547: f64;
                    if v5032 != 0.0 {
                        let v5542 = v5539 * (v5 + (v5020 - v316));
                        v5547 = v5542;
                    } else {
                        let v5544 = if v5020 < v5543 { 1.0 } else { 0.0 };
                        let v5548: f64;
                        if v5544 != 0.0 {
                            v5548 = v5545;
                        } else {
                            let v5546 = v5020.exp();
                            v5548 = v5546;
                        }
                        v5547 = v5548;
                    }
                    let v5549 = v5537 - v5547;
                    let v5551 = (v4740 * v5512) * v97;
                    let v5553 = (v5046 * v257) + v5002;
                    let v5554 = if v5553 > v316 { 1.0 } else { 0.0 };
                    let v5563: f64;
                    if v5554 != 0.0 {
                        let v5558 = v5555 * (v5 + (v5553 - v316));
                        v5563 = v5558;
                    } else {
                        let v5560 = if v5553 < v5559 { 1.0 } else { 0.0 };
                        let v5564: f64;
                        if v5560 != 0.0 {
                            v5564 = v5561;
                        } else {
                            let v5562 = v5553.exp();
                            v5564 = v5562;
                        }
                        v5563 = v5564;
                    }
                    let v5690: f64;
                    if v5060 != 0.0 {
                        let v5568 = v5551 * ((v5563 - (v4992 * v5549)) - v5522);
                        v5690 = v5568;
                    } else {
                        let v5572 = (v4986 * ((-v4982) - v4987)) + v5002;
                        let v5573 = if v5572 > v316 { 1.0 } else { 0.0 };
                        let v5582: f64;
                        if v5573 != 0.0 {
                            let v5577 = v5574 * (v5 + (v5572 - v316));
                            v5582 = v5577;
                        } else {
                            let v5579 = if v5572 < v5578 { 1.0 } else { 0.0 };
                            let v5583: f64;
                            if v5579 != 0.0 {
                                v5583 = v5580;
                            } else {
                                let v5581 = v5572.exp();
                                v5583 = v5581;
                            }
                            v5582 = v5583;
                        }
                        let v5584 = v5582 - v5547;
                        let v5586 = (v5046 * v4982) + v5002;
                        let v5587 = if v5586 > v316 { 1.0 } else { 0.0 };
                        let v5596: f64;
                        if v5587 != 0.0 {
                            let v5591 = v5588 * (v5 + (v5586 - v316));
                            v5596 = v5591;
                        } else {
                            let v5593 = if v5586 < v5592 { 1.0 } else { 0.0 };
                            let v5597: f64;
                            if v5593 != 0.0 {
                                v5597 = v5594;
                            } else {
                                let v5595 = v5586.exp();
                                v5597 = v5595;
                            }
                            v5596 = v5597;
                        }
                        let v5598 = v4992 * v5584;
                        let v5600 = (v5596 - v5598) - v5522;
                        let v5601 = v4992 * v5549;
                        let v5604 = v5551 * ((v5563 - v5601) - v5522);
                        let v5605 = if v4984 > v0 { 1.0 } else { 0.0 };
                        let v5658: f64;
                        if v5605 != 0.0 {
                            let v5607 = (v4984 * v4985) / v89;
                            let v5609 = (v5607 * v4982) + v5002;
                            let v5610 = if v5609 > v316 { 1.0 } else { 0.0 };
                            let v5619: f64;
                            if v5610 != 0.0 {
                                let v5614 = v5611 * (v5 + (v5609 - v316));
                                v5619 = v5614;
                            } else {
                                let v5616 = if v5609 < v5615 { 1.0 } else { 0.0 };
                                let v5620: f64;
                                if v5616 != 0.0 {
                                    v5620 = v5617;
                                } else {
                                    let v5618 = v5609.exp();
                                    v5620 = v5618;
                                }
                                v5619 = v5620;
                            }
                            let v5622 = (v5619 - v5598) - v5522;
                            let v5624 = (v5607 * v257) + v5002;
                            let v5625 = if v5624 > v316 { 1.0 } else { 0.0 };
                            let v5634: f64;
                            if v5625 != 0.0 {
                                let v5629 = v5626 * (v5 + (v5624 - v316));
                                v5634 = v5629;
                            } else {
                                let v5631 = if v5624 < v5630 { 1.0 } else { 0.0 };
                                let v5635: f64;
                                if v5631 != 0.0 {
                                    v5635 = v5632;
                                } else {
                                    let v5633 = v5624.exp();
                                    v5635 = v5633;
                                }
                                v5634 = v5635;
                            }
                            let v5640 = ((v5551 * v5600) / v5622) * ((v5634 - v5601) - v5522);
                            v5658 = v5640;
                        } else {
                            let v5641 = v5551 * v5600;
                            v5658 = v5641;
                        }
                        let v5643 = (v4983 * v4983) * v89;
                        let v5647 = (v257 - (v4982 - (v5643 / v324))) / v5643;
                        let v5648 = if v5647 > v316 { 1.0 } else { 0.0 };
                        let v5654: f64;
                        if v5648 != 0.0 {
                            v5654 = v0;
                        } else {
                            let v5650 = if v5647 < v5649 { 1.0 } else { 0.0 };
                            let v5655: f64;
                            if v5650 != 0.0 {
                                v5655 = v5;
                            } else {
                                let v5653 = v5 / (v5 + (v5647.exp()));
                                v5655 = v5653;
                            }
                            v5654 = v5655;
                        }
                        let v5660 = (v5654 * v5604) + ((v5 - v5654) * v5658);
                        v5690 = v5660;
                    }
                    let v5661 = v257 / v4993;
                    let v5669: f64;
                    if v259 != 0.0 {
                        let v5664 = ((v5661 * v5661) + v272).sqrt();
                        v5669 = v5664;
                    } else {
                        let v5668 = v5661 * (((v283 / v272) * v5661).tanh());
                        v5669 = v5668;
                    }
                    let v5675 = (v5173 * v5513) * v97;
                    let v5676 = v5176 * (v5524 / ((v5 + (v5669.powf(v4994))).powf(v5168)));
                    let v5677 = if v5676 > v316 { 1.0 } else { 0.0 };
                    let v5686: f64;
                    if v5677 != 0.0 {
                        let v5681 = v5678 * (v5 + (v5676 - v316));
                        v5686 = v5681;
                    } else {
                        let v5683 = if v5676 < v5682 { 1.0 } else { 0.0 };
                        let v5687: f64;
                        if v5683 != 0.0 {
                            v5687 = v5684;
                        } else {
                            let v5685 = v5676.exp();
                            v5687 = v5685;
                        }
                        v5686 = v5687;
                    }
                    let v5691 = v5690 + (v5675 * (v5686 - v5));
                    let v5692 = v250 * v4977;
                    let v5693 = v4988 * v5201;
                    let v5694 = v4988 * v5206;
                    let v5703: f64;
                    if v5003 != 0.0 {
                        let v5698 = v5695 * (v5 + (v5002 - v316));
                        v5703 = v5698;
                    } else {
                        let v5700 = if v5002 < v5699 { 1.0 } else { 0.0 };
                        let v5704: f64;
                        if v5700 != 0.0 {
                            v5704 = v5701;
                        } else {
                            let v5702 = v5002.exp();
                            v5704 = v5702;
                        }
                        v5703 = v5704;
                    }
                    let v5705 = -v5692;
                    let v5708 = (v5199 * (v5705 - v5200)) + v5002;
                    let v5709 = if v5708 > v316 { 1.0 } else { 0.0 };
                    let v5718: f64;
                    if v5709 != 0.0 {
                        let v5713 = v5710 * (v5 + (v5708 - v316));
                        v5718 = v5713;
                    } else {
                        let v5715 = if v5708 < v5714 { 1.0 } else { 0.0 };
                        let v5719: f64;
                        if v5715 != 0.0 {
                            v5719 = v5716;
                        } else {
                            let v5717 = v5708.exp();
                            v5719 = v5717;
                        }
                        v5718 = v5719;
                    }
                    let v5728: f64;
                    if v5237 != 0.0 {
                        let v5723 = v5720 * (v5 + (v5225 - v316));
                        v5728 = v5723;
                    } else {
                        let v5725 = if v5225 < v5724 { 1.0 } else { 0.0 };
                        let v5729: f64;
                        if v5725 != 0.0 {
                            v5729 = v5726;
                        } else {
                            let v5727 = v5225.exp();
                            v5729 = v5727;
                        }
                        v5728 = v5729;
                    }
                    let v5730 = v5718 - v5728;
                    let v5732 = (v4740 * v5693) * v97;
                    let v5734 = (v5251 * v5692) + v5002;
                    let v5735 = if v5734 > v316 { 1.0 } else { 0.0 };
                    let v5744: f64;
                    if v5735 != 0.0 {
                        let v5739 = v5736 * (v5 + (v5734 - v316));
                        v5744 = v5739;
                    } else {
                        let v5741 = if v5734 < v5740 { 1.0 } else { 0.0 };
                        let v5745: f64;
                        if v5741 != 0.0 {
                            v5745 = v5742;
                        } else {
                            let v5743 = v5734.exp();
                            v5745 = v5743;
                        }
                        v5744 = v5745;
                    }
                    let v5871: f64;
                    if v5265 != 0.0 {
                        let v5749 = v5732 * ((v5744 - (v5203 * v5730)) - v5703);
                        v5871 = v5749;
                    } else {
                        let v5753 = (v5199 * ((-v5195) - v5200)) + v5002;
                        let v5754 = if v5753 > v316 { 1.0 } else { 0.0 };
                        let v5763: f64;
                        if v5754 != 0.0 {
                            let v5758 = v5755 * (v5 + (v5753 - v316));
                            v5763 = v5758;
                        } else {
                            let v5760 = if v5753 < v5759 { 1.0 } else { 0.0 };
                            let v5764: f64;
                            if v5760 != 0.0 {
                                v5764 = v5761;
                            } else {
                                let v5762 = v5753.exp();
                                v5764 = v5762;
                            }
                            v5763 = v5764;
                        }
                        let v5765 = v5763 - v5728;
                        let v5767 = (v5251 * v5195) + v5002;
                        let v5768 = if v5767 > v316 { 1.0 } else { 0.0 };
                        let v5777: f64;
                        if v5768 != 0.0 {
                            let v5772 = v5769 * (v5 + (v5767 - v316));
                            v5777 = v5772;
                        } else {
                            let v5774 = if v5767 < v5773 { 1.0 } else { 0.0 };
                            let v5778: f64;
                            if v5774 != 0.0 {
                                v5778 = v5775;
                            } else {
                                let v5776 = v5767.exp();
                                v5778 = v5776;
                            }
                            v5777 = v5778;
                        }
                        let v5779 = v5203 * v5765;
                        let v5781 = (v5777 - v5779) - v5703;
                        let v5782 = v5203 * v5730;
                        let v5785 = v5732 * ((v5744 - v5782) - v5703);
                        let v5786 = if v5197 > v0 { 1.0 } else { 0.0 };
                        let v5839: f64;
                        if v5786 != 0.0 {
                            let v5788 = (v5197 * v5198) / v89;
                            let v5790 = (v5788 * v5195) + v5002;
                            let v5791 = if v5790 > v316 { 1.0 } else { 0.0 };
                            let v5800: f64;
                            if v5791 != 0.0 {
                                let v5795 = v5792 * (v5 + (v5790 - v316));
                                v5800 = v5795;
                            } else {
                                let v5797 = if v5790 < v5796 { 1.0 } else { 0.0 };
                                let v5801: f64;
                                if v5797 != 0.0 {
                                    v5801 = v5798;
                                } else {
                                    let v5799 = v5790.exp();
                                    v5801 = v5799;
                                }
                                v5800 = v5801;
                            }
                            let v5803 = (v5800 - v5779) - v5703;
                            let v5805 = (v5788 * v5692) + v5002;
                            let v5806 = if v5805 > v316 { 1.0 } else { 0.0 };
                            let v5815: f64;
                            if v5806 != 0.0 {
                                let v5810 = v5807 * (v5 + (v5805 - v316));
                                v5815 = v5810;
                            } else {
                                let v5812 = if v5805 < v5811 { 1.0 } else { 0.0 };
                                let v5816: f64;
                                if v5812 != 0.0 {
                                    v5816 = v5813;
                                } else {
                                    let v5814 = v5805.exp();
                                    v5816 = v5814;
                                }
                                v5815 = v5816;
                            }
                            let v5821 = ((v5732 * v5781) / v5803) * ((v5815 - v5782) - v5703);
                            v5839 = v5821;
                        } else {
                            let v5822 = v5732 * v5781;
                            v5839 = v5822;
                        }
                        let v5824 = (v5196 * v5196) * v89;
                        let v5828 = (v5692 - (v5195 - (v5824 / v324))) / v5824;
                        let v5829 = if v5828 > v316 { 1.0 } else { 0.0 };
                        let v5835: f64;
                        if v5829 != 0.0 {
                            v5835 = v0;
                        } else {
                            let v5831 = if v5828 < v5830 { 1.0 } else { 0.0 };
                            let v5836: f64;
                            if v5831 != 0.0 {
                                v5836 = v5;
                            } else {
                                let v5834 = v5 / (v5 + (v5828.exp()));
                                v5836 = v5834;
                            }
                            v5835 = v5836;
                        }
                        let v5841 = (v5835 * v5785) + ((v5 - v5835) * v5839);
                        v5871 = v5841;
                    }
                    let v5842 = v5692 / v5204;
                    let v5850: f64;
                    if v259 != 0.0 {
                        let v5845 = ((v5842 * v5842) + v272).sqrt();
                        v5850 = v5845;
                    } else {
                        let v5849 = v5842 * (((v283 / v272) * v5842).tanh());
                        v5850 = v5849;
                    }
                    let v5856 = (v5173 * v5694) * v97;
                    let v5857 = v5378 * (v5705 / ((v5 + (v5850.powf(v5205))).powf(v5373)));
                    let v5858 = if v5857 > v316 { 1.0 } else { 0.0 };
                    let v5867: f64;
                    if v5858 != 0.0 {
                        let v5862 = v5859 * (v5 + (v5857 - v316));
                        v5867 = v5862;
                    } else {
                        let v5864 = if v5857 < v5863 { 1.0 } else { 0.0 };
                        let v5868: f64;
                        if v5864 != 0.0 {
                            v5868 = v5865;
                        } else {
                            let v5866 = v5857.exp();
                            v5868 = v5866;
                        }
                        v5867 = v5868;
                    }
                    let v5872 = v5871 + (v5856 * (v5867 - v5));
                    if v5396 != 0.0 {
                        if v5003 != 0.0 {
                        } else {
                            let v5874 = if v5002 < v5873 { 1.0 } else { 0.0 };
                            if v5874 != 0.0 {
                            } else {
                            }
                        }
                        if v5528 != 0.0 {
                        } else {
                            let v5876 = if v5527 < v5875 { 1.0 } else { 0.0 };
                            if v5876 != 0.0 {
                            } else {
                            }
                        }
                        if v5032 != 0.0 {
                        } else {
                            let v5878 = if v5020 < v5877 { 1.0 } else { 0.0 };
                            if v5878 != 0.0 {
                            } else {
                            }
                        }
                        if v5554 != 0.0 {
                        } else {
                            let v5880 = if v5553 < v5879 { 1.0 } else { 0.0 };
                            if v5880 != 0.0 {
                            } else {
                            }
                        }
                        if v5881 != 0.0 {
                        } else {
                            let v5885 = (v4986 * ((-v4982) - v4987)) + v5002;
                            let v5886 = if v5885 > v316 { 1.0 } else { 0.0 };
                            if v5886 != 0.0 {
                            } else {
                                let v5888 = if v5885 < v5887 { 1.0 } else { 0.0 };
                                if v5888 != 0.0 {
                                } else {
                                }
                            }
                            let v5890 = (v5046 * v4982) + v5002;
                            let v5891 = if v5890 > v316 { 1.0 } else { 0.0 };
                            if v5891 != 0.0 {
                            } else {
                                let v5893 = if v5890 < v5892 { 1.0 } else { 0.0 };
                                if v5893 != 0.0 {
                                } else {
                                }
                            }
                            if v5894 != 0.0 {
                                if v5891 != 0.0 {
                                } else {
                                    let v5896 = if v5890 < v5895 { 1.0 } else { 0.0 };
                                    if v5896 != 0.0 {
                                    } else {
                                    }
                                }
                                if v5554 != 0.0 {
                                } else {
                                    let v5898 = if v5553 < v5897 { 1.0 } else { 0.0 };
                                    if v5898 != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                            }
                            let v5900 = (v4983 * v4983) * v89;
                            let v5904 = (v257 - (v4982 - (v5900 / v324))) / v5900;
                            let v5905 = if v5904 > v316 { 1.0 } else { 0.0 };
                            if v5905 != 0.0 {
                            } else {
                                let v5907 = if v5904 < v5906 { 1.0 } else { 0.0 };
                                if v5907 != 0.0 {
                                } else {
                                }
                            }
                        }
                        let v5908 = v257 / v5397;
                        let v5916: f64;
                        if v259 != 0.0 {
                            let v5911 = ((v5908 * v5908) + v272).sqrt();
                            v5916 = v5911;
                        } else {
                            let v5915 = v5908 * (((v283 / v272) * v5908).tanh());
                            v5916 = v5915;
                        }
                        let v5923 = (v5399 / v89) * (v5524 / ((v5 + (v5916.powf(v5398))).powf((v5 / v5398))));
                        let v5924 = if v5923 > v316 { 1.0 } else { 0.0 };
                        if v5924 != 0.0 {
                        } else {
                            let v5926 = if v5923 < v5925 { 1.0 } else { 0.0 };
                            if v5926 != 0.0 {
                            } else {
                            }
                        }
                        if v5003 != 0.0 {
                        } else {
                            let v5928 = if v5002 < v5927 { 1.0 } else { 0.0 };
                            if v5928 != 0.0 {
                            } else {
                            }
                        }
                        if v5709 != 0.0 {
                        } else {
                            let v5930 = if v5708 < v5929 { 1.0 } else { 0.0 };
                            if v5930 != 0.0 {
                            } else {
                            }
                        }
                        if v5237 != 0.0 {
                        } else {
                            let v5932 = if v5225 < v5931 { 1.0 } else { 0.0 };
                            if v5932 != 0.0 {
                            } else {
                            }
                        }
                        if v5735 != 0.0 {
                        } else {
                            let v5934 = if v5734 < v5933 { 1.0 } else { 0.0 };
                            if v5934 != 0.0 {
                            } else {
                            }
                        }
                        if v5935 != 0.0 {
                        } else {
                            let v5939 = (v5199 * ((-v5195) - v5200)) + v5002;
                            let v5940 = if v5939 > v316 { 1.0 } else { 0.0 };
                            if v5940 != 0.0 {
                            } else {
                                let v5942 = if v5939 < v5941 { 1.0 } else { 0.0 };
                                if v5942 != 0.0 {
                                } else {
                                }
                            }
                            let v5944 = (v5251 * v5195) + v5002;
                            let v5945 = if v5944 > v316 { 1.0 } else { 0.0 };
                            if v5945 != 0.0 {
                            } else {
                                let v5947 = if v5944 < v5946 { 1.0 } else { 0.0 };
                                if v5947 != 0.0 {
                                } else {
                                }
                            }
                            if v5948 != 0.0 {
                                if v5945 != 0.0 {
                                } else {
                                    let v5950 = if v5944 < v5949 { 1.0 } else { 0.0 };
                                    if v5950 != 0.0 {
                                    } else {
                                    }
                                }
                                if v5735 != 0.0 {
                                } else {
                                    let v5952 = if v5734 < v5951 { 1.0 } else { 0.0 };
                                    if v5952 != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                            }
                            let v5954 = (v5196 * v5196) * v89;
                            let v5958 = (v5692 - (v5195 - (v5954 / v324))) / v5954;
                            let v5959 = if v5958 > v316 { 1.0 } else { 0.0 };
                            if v5959 != 0.0 {
                            } else {
                                let v5961 = if v5958 < v5960 { 1.0 } else { 0.0 };
                                if v5961 != 0.0 {
                                } else {
                                }
                            }
                        }
                        let v5962 = v5692 / v5454;
                        let v5970: f64;
                        if v259 != 0.0 {
                            let v5965 = ((v5962 * v5962) + v272).sqrt();
                            v5970 = v5965;
                        } else {
                            let v5969 = v5962 * (((v283 / v272) * v5962).tanh());
                            v5970 = v5969;
                        }
                        let v5977 = (v5456 / v89) * (v5705 / ((v5 + (v5970.powf(v5455))).powf((v5 / v5455))));
                        let v5978 = if v5977 > v316 { 1.0 } else { 0.0 };
                        if v5978 != 0.0 {
                        } else {
                            let v5980 = if v5977 < v5979 { 1.0 } else { 0.0 };
                            if v5980 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    v6393 = v5691;
                    v6395 = v5551;
                    v6397 = v5675;
                    v6406 = v5872;
                    v6408 = v5732;
                    v6410 = v5856;
                } else {
                    v6393 = v0;
                    v6395 = v0;
                    v6397 = v0;
                    v6406 = v0;
                    v6408 = v0;
                    v6410 = v0;
                }
                v6392 = v6393;
                v6394 = v6395;
                v6396 = v6397;
                v6405 = v6406;
                v6407 = v6408;
                v6409 = v6410;
                v6416 = v5192;
                v6417 = v5045;
                v6418 = v5175;
                v6424 = v5394;
                v6425 = v5250;
                v6426 = v5377;
            } else {
                v6392 = v0;
                v6394 = v0;
                v6396 = v0;
                v6405 = v0;
                v6407 = v0;
                v6409 = v0;
                v6416 = v0;
                v6417 = v0;
                v6418 = v0;
                v6424 = v0;
                v6425 = v0;
                v6426 = v0;
            }
            let v5982 = if v5981 == v5 { 1.0 } else { 0.0 };
            if v5982 != 0.0 {
                let v5984 = v250 * (v255 - v395);
                let v5994 = v0 / v89;
                let v5996 = v5994 * v5995;
                let v5997 = if v5996 > v316 { 1.0 } else { 0.0 };
                if v5997 != 0.0 {
                } else {
                    let v5999 = if v5996 < v5998 { 1.0 } else { 0.0 };
                    if v5999 != 0.0 {
                    } else {
                    }
                }
                let v6000 = -v5984;
                let v6002 = v897 * (v6000 - v5989);
                let v6003 = v6002 + v5996;
                let v6005 = v6004 + v5996;
                let v6006 = if v6003 > v316 { 1.0 } else { 0.0 };
                if v6006 != 0.0 {
                } else {
                    let v6008 = if v6003 < v6007 { 1.0 } else { 0.0 };
                    if v6008 != 0.0 {
                    } else {
                    }
                }
                let v6009 = if v6005 > v316 { 1.0 } else { 0.0 };
                if v6009 != 0.0 {
                } else {
                    let v6011 = if v6005 < v6010 { 1.0 } else { 0.0 };
                    if v6011 != 0.0 {
                    } else {
                    }
                }
                let v6012 = v5988 / v89;
                let v6014 = (v6012 * v5984) + v5996;
                let v6015 = if v6014 > v316 { 1.0 } else { 0.0 };
                if v6015 != 0.0 {
                } else {
                    let v6017 = if v6014 < v6016 { 1.0 } else { 0.0 };
                    if v6017 != 0.0 {
                    } else {
                    }
                }
                let v6018 = if v5987 == v5 { 1.0 } else { 0.0 };
                if v6018 != 0.0 {
                } else {
                    let v6022 = (v897 * ((-v5985) - v5989)) + v5996;
                    let v6023 = if v6022 > v316 { 1.0 } else { 0.0 };
                    if v6023 != 0.0 {
                    } else {
                        let v6025 = if v6022 < v6024 { 1.0 } else { 0.0 };
                        if v6025 != 0.0 {
                        } else {
                        }
                    }
                    let v6027 = (v6012 * v5985) + v5996;
                    let v6028 = if v6027 > v316 { 1.0 } else { 0.0 };
                    if v6028 != 0.0 {
                    } else {
                        let v6030 = if v6027 < v6029 { 1.0 } else { 0.0 };
                        if v6030 != 0.0 {
                        } else {
                        }
                    }
                    let v6031 = if v5987 > v0 { 1.0 } else { 0.0 };
                    if v6031 != 0.0 {
                        let v6033 = (v5987 * v5988) / v89;
                        let v6035 = (v6033 * v5985) + v5996;
                        let v6036 = if v6035 > v316 { 1.0 } else { 0.0 };
                        if v6036 != 0.0 {
                        } else {
                            let v6038 = if v6035 < v6037 { 1.0 } else { 0.0 };
                            if v6038 != 0.0 {
                            } else {
                            }
                        }
                        let v6040 = (v6033 * v5984) + v5996;
                        let v6041 = if v6040 > v316 { 1.0 } else { 0.0 };
                        if v6041 != 0.0 {
                        } else {
                            let v6043 = if v6040 < v6042 { 1.0 } else { 0.0 };
                            if v6043 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v6045 = (v5986 * v5986) * v89;
                    let v6049 = (v5984 - (v5985 - (v6045 / v324))) / v6045;
                    let v6050 = if v6049 > v316 { 1.0 } else { 0.0 };
                    if v6050 != 0.0 {
                    } else {
                        let v6052 = if v6049 < v6051 { 1.0 } else { 0.0 };
                        if v6052 != 0.0 {
                        } else {
                        }
                    }
                }
                let v6053 = v5984 / v5991;
                let v6061: f64;
                if v259 != 0.0 {
                    let v6056 = ((v6053 * v6053) + v272).sqrt();
                    v6061 = v6056;
                } else {
                    let v6060 = v6053 * (((v283 / v272) * v6053).tanh());
                    v6061 = v6060;
                }
                let v6068 = (v5993 / v89) * (v6000 / ((v5 + (v6061.powf(v5992))).powf((v5 / v5992))));
                let v6069 = if v6068 > v316 { 1.0 } else { 0.0 };
                if v6069 != 0.0 {
                } else {
                    let v6071 = if v6068 < v6070 { 1.0 } else { 0.0 };
                    if v6071 != 0.0 {
                    } else {
                    }
                }
                let v6073 = if v6072 == v5 { 1.0 } else { 0.0 };
                if v6073 != 0.0 {
                    let v6078 = v5994 * v6077;
                    let v6079 = if v6078 > v316 { 1.0 } else { 0.0 };
                    if v6079 != 0.0 {
                    } else {
                        let v6081 = if v6078 < v6080 { 1.0 } else { 0.0 };
                        if v6081 != 0.0 {
                        } else {
                        }
                    }
                    let v6082 = v6002 + v6078;
                    let v6084 = v6083 + v6078;
                    let v6085 = if v6082 > v316 { 1.0 } else { 0.0 };
                    if v6085 != 0.0 {
                    } else {
                        let v6087 = if v6082 < v6086 { 1.0 } else { 0.0 };
                        if v6087 != 0.0 {
                        } else {
                        }
                    }
                    let v6088 = if v6084 > v316 { 1.0 } else { 0.0 };
                    if v6088 != 0.0 {
                    } else {
                        let v6090 = if v6084 < v6089 { 1.0 } else { 0.0 };
                        if v6090 != 0.0 {
                        } else {
                        }
                    }
                    let v6092 = (v5994 * v5984) + v6078;
                    let v6093 = if v6092 > v316 { 1.0 } else { 0.0 };
                    if v6093 != 0.0 {
                    } else {
                        let v6095 = if v6092 < v6094 { 1.0 } else { 0.0 };
                        if v6095 != 0.0 {
                        } else {
                        }
                    }
                    if v6096 != 0.0 {
                    } else {
                        let v6098 = v6097 + v6078;
                        let v6099 = if v6098 > v316 { 1.0 } else { 0.0 };
                        if v6099 != 0.0 {
                        } else {
                            let v6101 = if v6098 < v6100 { 1.0 } else { 0.0 };
                            if v6101 != 0.0 {
                            } else {
                            }
                        }
                        let v6102 = v5994 + v6078;
                        let v6103 = if v6102 > v316 { 1.0 } else { 0.0 };
                        if v6103 != 0.0 {
                        } else {
                            let v6105 = if v6102 < v6104 { 1.0 } else { 0.0 };
                            if v6105 != 0.0 {
                            } else {
                            }
                        }
                        if v6106 != 0.0 {
                            let v6108 = v6107 / v89;
                            let v6109 = v6108 + v6078;
                            let v6110 = if v6109 > v316 { 1.0 } else { 0.0 };
                            if v6110 != 0.0 {
                            } else {
                                let v6112 = if v6109 < v6111 { 1.0 } else { 0.0 };
                                if v6112 != 0.0 {
                                } else {
                                }
                            }
                            let v6114 = (v6108 * v5984) + v6078;
                            let v6115 = if v6114 > v316 { 1.0 } else { 0.0 };
                            if v6115 != 0.0 {
                            } else {
                                let v6117 = if v6114 < v6116 { 1.0 } else { 0.0 };
                                if v6117 != 0.0 {
                                } else {
                                }
                            }
                        } else {
                        }
                        let v6119 = v6118 * v89;
                        let v6123 = (v5984 - (v5 - (v6119 / v324))) / v6119;
                        let v6124 = if v6123 > v316 { 1.0 } else { 0.0 };
                        if v6124 != 0.0 {
                        } else {
                            let v6126 = if v6123 < v6125 { 1.0 } else { 0.0 };
                            if v6126 != 0.0 {
                            } else {
                            }
                        }
                    }
                    let v6127 = v5984 / v6074;
                    let v6135: f64;
                    if v259 != 0.0 {
                        let v6130 = ((v6127 * v6127) + v272).sqrt();
                        v6135 = v6130;
                    } else {
                        let v6134 = v6127 * (((v283 / v272) * v6127).tanh());
                        v6135 = v6134;
                    }
                    let v6142 = (v6076 / v89) * (v6000 / ((v5 + (v6135.powf(v6075))).powf((v5 / v6075))));
                    let v6143 = if v6142 > v316 { 1.0 } else { 0.0 };
                    if v6143 != 0.0 {
                    } else {
                        let v6145 = if v6142 < v6144 { 1.0 } else { 0.0 };
                        if v6145 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
                let v6149 = if v5984 <= (v6146 * v6147) { 1.0 } else { 0.0 };
                if v6149 != 0.0 {
                } else {
                    let v6151 = if v6150 >= v5 { 1.0 } else { 0.0 };
                    if v6151 != 0.0 {
                        let v6152 = if v6150 >= v324 { 1.0 } else { 0.0 };
                        if v6152 != 0.0 {
                            let v6153 = if v6150 >= v96 { 1.0 } else { 0.0 };
                            if v6153 != 0.0 {
                                let v6154 = if v6150 >= v897 { 1.0 } else { 0.0 };
                                if v6154 != 0.0 {
                                    let v6156 = if v6150 >= v6155 { 1.0 } else { 0.0 };
                                    if v6156 != 0.0 {
                                    } else {
                                    }
                                } else {
                                }
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                }
                let v6160 = if (if v6157 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v5990 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v6160 != 0.0 {
                } else {
                }
            } else {
            }
            let v6164 = v250 * ((v260 - v387) + (v260 - v255));
            let v6168 = v250 * ((v387 - v260) + (v387 - v255));
            let v6170 = if v6169 == v5 { 1.0 } else { 0.0 };
            if v6170 != 0.0 {
                let v6172 = if v6171 == v0 { 1.0 } else { 0.0 };
                let v6181: f64;
                let v6262: f64;
                if v6172 != 0.0 {
                    let v6176 = v250 * ((v264 - v261) + (v264 - v255));
                    let v6180 = v250 * ((v261 - v264) + (v261 - v255));
                    v6181 = v6176;
                    v6262 = v6180;
                } else {
                    v6181 = v6164;
                    v6262 = v6168;
                }
                let v6184 = v0 / v89;
                let v6186 = v6184 * (-v4999);
                let v6187 = if v6186 > v316 { 1.0 } else { 0.0 };
                if v6187 != 0.0 {
                } else {
                    let v6189 = if v6186 < v6188 { 1.0 } else { 0.0 };
                    if v6189 != 0.0 {
                    } else {
                    }
                }
                let v6190 = -v6181;
                let v6193 = (v6182 * (v6190 - v6183)) + v6186;
                let v6196 = ((-v6182) * v6183) + v6186;
                let v6197 = if v6193 > v316 { 1.0 } else { 0.0 };
                if v6197 != 0.0 {
                } else {
                    let v6199 = if v6193 < v6198 { 1.0 } else { 0.0 };
                    if v6199 != 0.0 {
                    } else {
                    }
                }
                let v6200 = if v6196 > v316 { 1.0 } else { 0.0 };
                if v6200 != 0.0 {
                } else {
                    let v6202 = if v6196 < v6201 { 1.0 } else { 0.0 };
                    if v6202 != 0.0 {
                    } else {
                    }
                }
                let v6204 = (v6184 * v6181) + v6186;
                let v6205 = if v6204 > v316 { 1.0 } else { 0.0 };
                if v6205 != 0.0 {
                } else {
                    let v6207 = if v6204 < v6206 { 1.0 } else { 0.0 };
                    if v6207 != 0.0 {
                    } else {
                    }
                }
                let v6208 = if v4984 == v5 { 1.0 } else { 0.0 };
                if v6208 != 0.0 {
                } else {
                    let v6212 = (v6182 * ((-v4982) - v6183)) + v6186;
                    let v6213 = if v6212 > v316 { 1.0 } else { 0.0 };
                    if v6213 != 0.0 {
                    } else {
                        let v6215 = if v6212 < v6214 { 1.0 } else { 0.0 };
                        if v6215 != 0.0 {
                        } else {
                        }
                    }
                    let v6217 = (v6184 * v4982) + v6186;
                    let v6218 = if v6217 > v316 { 1.0 } else { 0.0 };
                    if v6218 != 0.0 {
                    } else {
                        let v6220 = if v6217 < v6219 { 1.0 } else { 0.0 };
                        if v6220 != 0.0 {
                        } else {
                        }
                    }
                    let v6221 = if v4984 > v0 { 1.0 } else { 0.0 };
                    if v6221 != 0.0 {
                        let v6223 = (v4984 * v0) / v89;
                        let v6225 = (v6223 * v4982) + v6186;
                        let v6226 = if v6225 > v316 { 1.0 } else { 0.0 };
                        if v6226 != 0.0 {
                        } else {
                            let v6228 = if v6225 < v6227 { 1.0 } else { 0.0 };
                            if v6228 != 0.0 {
                            } else {
                            }
                        }
                        let v6230 = (v6223 * v6181) + v6186;
                        let v6231 = if v6230 > v316 { 1.0 } else { 0.0 };
                        if v6231 != 0.0 {
                        } else {
                            let v6233 = if v6230 < v6232 { 1.0 } else { 0.0 };
                            if v6233 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v6235 = (v4983 * v4983) * v89;
                    let v6239 = (v6181 - (v4982 - (v6235 / v324))) / v6235;
                    let v6240 = if v6239 > v316 { 1.0 } else { 0.0 };
                    if v6240 != 0.0 {
                    } else {
                        let v6242 = if v6239 < v6241 { 1.0 } else { 0.0 };
                        if v6242 != 0.0 {
                        } else {
                        }
                    }
                }
                let v6243 = v6181 / v4993;
                let v6251: f64;
                if v259 != 0.0 {
                    let v6246 = ((v6243 * v6243) + v272).sqrt();
                    v6251 = v6246;
                } else {
                    let v6250 = v6243 * (((v283 / v272) * v6243).tanh());
                    v6251 = v6250;
                }
                let v6258 = (v4997 / v89) * (v6190 / ((v5 + (v6251.powf(v4994))).powf((v5 / v4994))));
                let v6259 = if v6258 > v316 { 1.0 } else { 0.0 };
                if v6259 != 0.0 {
                } else {
                    let v6261 = if v6258 < v6260 { 1.0 } else { 0.0 };
                    if v6261 != 0.0 {
                    } else {
                    }
                }
                if v6187 != 0.0 {
                } else {
                    let v6266 = if v6186 < v6265 { 1.0 } else { 0.0 };
                    if v6266 != 0.0 {
                    } else {
                    }
                }
                let v6267 = -v6262;
                let v6270 = (v6263 * (v6267 - v6264)) + v6186;
                let v6273 = ((-v6263) * v6264) + v6186;
                let v6274 = if v6270 > v316 { 1.0 } else { 0.0 };
                if v6274 != 0.0 {
                } else {
                    let v6276 = if v6270 < v6275 { 1.0 } else { 0.0 };
                    if v6276 != 0.0 {
                    } else {
                    }
                }
                let v6277 = if v6273 > v316 { 1.0 } else { 0.0 };
                if v6277 != 0.0 {
                } else {
                    let v6279 = if v6273 < v6278 { 1.0 } else { 0.0 };
                    if v6279 != 0.0 {
                    } else {
                    }
                }
                let v6281 = (v6184 * v6262) + v6186;
                let v6282 = if v6281 > v316 { 1.0 } else { 0.0 };
                if v6282 != 0.0 {
                } else {
                    let v6284 = if v6281 < v6283 { 1.0 } else { 0.0 };
                    if v6284 != 0.0 {
                    } else {
                    }
                }
                let v6285 = if v5197 == v5 { 1.0 } else { 0.0 };
                if v6285 != 0.0 {
                } else {
                    let v6289 = (v6263 * ((-v5195) - v6264)) + v6186;
                    let v6290 = if v6289 > v316 { 1.0 } else { 0.0 };
                    if v6290 != 0.0 {
                    } else {
                        let v6292 = if v6289 < v6291 { 1.0 } else { 0.0 };
                        if v6292 != 0.0 {
                        } else {
                        }
                    }
                    let v6294 = (v6184 * v5195) + v6186;
                    let v6295 = if v6294 > v316 { 1.0 } else { 0.0 };
                    if v6295 != 0.0 {
                    } else {
                        let v6297 = if v6294 < v6296 { 1.0 } else { 0.0 };
                        if v6297 != 0.0 {
                        } else {
                        }
                    }
                    let v6298 = if v5197 > v0 { 1.0 } else { 0.0 };
                    if v6298 != 0.0 {
                        let v6300 = (v5197 * v0) / v89;
                        let v6302 = (v6300 * v5195) + v6186;
                        let v6303 = if v6302 > v316 { 1.0 } else { 0.0 };
                        if v6303 != 0.0 {
                        } else {
                            let v6305 = if v6302 < v6304 { 1.0 } else { 0.0 };
                            if v6305 != 0.0 {
                            } else {
                            }
                        }
                        let v6307 = (v6300 * v6262) + v6186;
                        let v6308 = if v6307 > v316 { 1.0 } else { 0.0 };
                        if v6308 != 0.0 {
                        } else {
                            let v6310 = if v6307 < v6309 { 1.0 } else { 0.0 };
                            if v6310 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v6312 = (v5196 * v5196) * v89;
                    let v6316 = (v6262 - (v5195 - (v6312 / v324))) / v6312;
                    let v6317 = if v6316 > v316 { 1.0 } else { 0.0 };
                    if v6317 != 0.0 {
                    } else {
                        let v6319 = if v6316 < v6318 { 1.0 } else { 0.0 };
                        if v6319 != 0.0 {
                        } else {
                        }
                    }
                }
                let v6320 = v6262 / v5204;
                let v6328: f64;
                if v259 != 0.0 {
                    let v6323 = ((v6320 * v6320) + v272).sqrt();
                    v6328 = v6323;
                } else {
                    let v6327 = v6320 * (((v283 / v272) * v6320).tanh());
                    v6328 = v6327;
                }
                let v6335 = (v5208 / v89) * (v6267 / ((v5 + (v6328.powf(v5205))).powf((v5 / v5205))));
                let v6336 = if v6335 > v316 { 1.0 } else { 0.0 };
                if v6336 != 0.0 {
                } else {
                    let v6338 = if v6335 < v6337 { 1.0 } else { 0.0 };
                    if v6338 != 0.0 {
                    } else {
                    }
                }
                if v6172 != 0.0 {
                } else {
                }
            } else {
            }
            if v62 != 0.0 {
            } else {
            }
            if v44 != 0.0 {
            } else {
            }
            let v6345 = if (if v81 >= v41 { 1.0 } else { 0.0 }) != 0.0 && (if v81 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v6345 != 0.0 {
            } else {
            }
            let v6349 = if (if v85 >= v41 { 1.0 } else { 0.0 }) != 0.0 && (if v85 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v6349 != 0.0 {
            } else {
            }
            let v6354 = ((v6346 - v264) - v6351) / v6353;
            let v6355 = if v6354 > v316 { 1.0 } else { 0.0 };
            if v6355 != 0.0 {
            } else {
                let v6357 = if v6354 < v6356 { 1.0 } else { 0.0 };
                if v6357 != 0.0 {
                } else {
                }
            }
            let v6360 = ((v6346 - v261) - v6351) / v6353;
            let v6361 = if v6360 > v316 { 1.0 } else { 0.0 };
            if v6361 != 0.0 {
            } else {
                let v6363 = if v6360 < v6362 { 1.0 } else { 0.0 };
                if v6363 != 0.0 {
                } else {
                }
            }
            let v6366 = ((v264 - v261) - v6351) / v6353;
            let v6367 = if v6366 > v316 { 1.0 } else { 0.0 };
            if v6367 != 0.0 {
            } else {
                let v6369 = if v6366 < v6368 { 1.0 } else { 0.0 };
                if v6369 != 0.0 {
                } else {
                }
            }
            let v6372 = ((v407 - v264) - v6351) / v6353;
            let v6373 = if v6372 > v316 { 1.0 } else { 0.0 };
            if v6373 != 0.0 {
            } else {
                let v6375 = if v6372 < v6374 { 1.0 } else { 0.0 };
                if v6375 != 0.0 {
                } else {
                }
            }
            let v6378 = ((v407 - v261) - v6351) / v6353;
            let v6379 = if v6378 > v316 { 1.0 } else { 0.0 };
            if v6379 != 0.0 {
            } else {
                let v6381 = if v6378 < v6380 { 1.0 } else { 0.0 };
                if v6381 != 0.0 {
                } else {
                }
            }
            let v6384 = ((v6346 - v407) - v6351) / v6353;
            let v6385 = if v6384 > v316 { 1.0 } else { 0.0 };
            if v6385 != 0.0 {
            } else {
                let v6387 = if v6384 < v6386 { 1.0 } else { 0.0 };
                if v6387 != 0.0 {
                } else {
                }
            }
            let v6389 = if v6388 == v5 { 1.0 } else { 0.0 };
            let v6508: f64;
            let v6509: f64;
            let v6510: f64;
            let v6511: f64;
            let v6512: f64;
            let v6513: f64;
            let v6514: f64;
            let v6515: f64;
            let v6516: f64;
            let v6517: f64;
            let v6518: f64;
            let v6519: f64;
            let v6520: f64;
            let v6521: f64;
            let v6523: f64;
            let v6525: f64;
            let v6527: f64;
            let v6529: f64;
            let v6531: f64;
            let v6533: f64;
            let v6535: f64;
            let v6537: f64;
            let v6539: f64;
            let v6541: f64;
            let v6543: f64;
            let v6545: f64;
            let v6547: f64;
            let v6549: f64;
            let v6551: f64;
            let v6553: f64;
            let v6555: f64;
            let v6557: f64;
            let v6559: f64;
            if v6389 != 0.0 {
                let v6391 = v6390 * v88;
                let v6402 = v6391 * ((v6392 + (v324 * (v6394 + v6396))).abs());
                let v6404 = v6403 * v88;
                let v6415 = v6404 * ((v6405 + (v324 * (v6407 + v6409))).abs());
                let v6423 = v6391 * ((v6416 + (v324 * (v6417 + v6418))).abs());
                let v6431 = v6404 * ((v6424 + (v324 * (v6425 + v6426))).abs());
                let v6439 = (v6432 * (v4948 / v4479)) * (((v4745.abs()) / v4948).powf(v6437));
                let v6440 = if v4745 < v0 { 1.0 } else { 0.0 };
                let v6442: f64;
                if v6440 != 0.0 {
                    let v6441 = -v6439;
                    v6442 = v6441;
                } else {
                    v6442 = v6439;
                }
                let v6450 = (((v6444 * v46) * (v7044[2])) * (v4952 + v4954)) / (v4950 * v123);
                let v6451 = if v28 != v0 { 1.0 } else { 0.0 };
                let v6452 = if v2115 != 0.0 && v6451 != 0.0 { 1.0 } else { 0.0 };
                let v6522: f64;
                let v6524: f64;
                if v6452 != 0.0 {
                    let v6457 = (v6453 * v46) / ((v28 * v2114) / v4948);
                    v6522 = v5;
                    v6524 = v6457;
                } else {
                    v6522 = v0;
                    v6524 = v0;
                }
                let v6458 = if v2512 != 0.0 && v6451 != 0.0 { 1.0 } else { 0.0 };
                let v6526: f64;
                let v6528: f64;
                if v6458 != 0.0 {
                    let v6463 = (v6459 * v46) / ((v28 * v2511) / v4948);
                    v6526 = v5;
                    v6528 = v6463;
                } else {
                    v6526 = v0;
                    v6528 = v0;
                }
                let v6464 = if v2909 != 0.0 && v6451 != 0.0 { 1.0 } else { 0.0 };
                let v6530: f64;
                let v6532: f64;
                if v6464 != 0.0 {
                    let v6469 = (v6465 * v46) / ((v28 * v2908) / v4948);
                    v6530 = v5;
                    v6532 = v6469;
                } else {
                    v6530 = v0;
                    v6532 = v0;
                }
                let v6470 = if v3306 != 0.0 && v6451 != 0.0 { 1.0 } else { 0.0 };
                let v6534: f64;
                let v6536: f64;
                if v6470 != 0.0 {
                    let v6475 = (v6471 * v46) / ((v28 * v3305) / v4948);
                    v6534 = v5;
                    v6536 = v6475;
                } else {
                    v6534 = v0;
                    v6536 = v0;
                }
                let v6476 = if v1718 != 0.0 && v6451 != 0.0 { 1.0 } else { 0.0 };
                let v6538: f64;
                let v6540: f64;
                if v6476 != 0.0 {
                    let v6481 = (v6477 * v46) / ((v28 * v1717) / v4948);
                    v6538 = v5;
                    v6540 = v6481;
                } else {
                    v6538 = v0;
                    v6540 = v0;
                }
                let v6482 = if v1321 != 0.0 && v6451 != 0.0 { 1.0 } else { 0.0 };
                let v6542: f64;
                let v6544: f64;
                if v6482 != 0.0 {
                    let v6487 = (v6483 * v46) / ((v28 * v1320) / v4948);
                    v6542 = v5;
                    v6544 = v6487;
                } else {
                    v6542 = v0;
                    v6544 = v0;
                }
                let v6488 = if v924 != 0.0 && v6451 != 0.0 { 1.0 } else { 0.0 };
                let v6546: f64;
                let v6548: f64;
                if v6488 != 0.0 {
                    let v6493 = (v6489 * v46) / ((v28 * v923) / v4948);
                    v6546 = v5;
                    v6548 = v6493;
                } else {
                    v6546 = v0;
                    v6548 = v0;
                }
                let v6494 = if v515 != 0.0 && v6451 != 0.0 { 1.0 } else { 0.0 };
                let v6550: f64;
                let v6552: f64;
                if v6494 != 0.0 {
                    let v6499 = (v6495 * v46) / ((v28 * v513) / v4948);
                    v6550 = v5;
                    v6552 = v6499;
                } else {
                    v6550 = v0;
                    v6552 = v0;
                }
                let v6554: f64;
                let v6556: f64;
                if v44 != 0.0 {
                    let v6502 = (v6500 * v46) / v6341;
                    v6554 = v5;
                    v6556 = v6502;
                } else {
                    v6554 = v0;
                    v6556 = v0;
                }
                let v6558: f64;
                let v6560: f64;
                if v62 != 0.0 {
                    let v6505 = (v6503 * v46) / v6339;
                    v6558 = v5;
                    v6560 = v6505;
                } else {
                    v6558 = v0;
                    v6560 = v0;
                }
                v6508 = v5;
                v6509 = v6402;
                v6510 = v5;
                v6511 = v6415;
                v6512 = v5;
                v6513 = v6423;
                v6514 = v5;
                v6515 = v6431;
                v6516 = v5;
                v6517 = v6442;
                v6518 = v6443;
                v6519 = v5;
                v6520 = v6450;
                v6521 = v6522;
                v6523 = v6524;
                v6525 = v6526;
                v6527 = v6528;
                v6529 = v6530;
                v6531 = v6532;
                v6533 = v6534;
                v6535 = v6536;
                v6537 = v6538;
                v6539 = v6540;
                v6541 = v6542;
                v6543 = v6544;
                v6545 = v6546;
                v6547 = v6548;
                v6549 = v6550;
                v6551 = v6552;
                v6553 = v6554;
                v6555 = v6556;
                v6557 = v6558;
                v6559 = v6560;
            } else {
                v6508 = v0;
                v6509 = v0;
                v6510 = v0;
                v6511 = v0;
                v6512 = v0;
                v6513 = v0;
                v6514 = v0;
                v6515 = v0;
                v6516 = v0;
                v6517 = v0;
                v6518 = v0;
                v6519 = v0;
                v6520 = v0;
                v6521 = v0;
                v6523 = v0;
                v6525 = v0;
                v6527 = v0;
                v6529 = v0;
                v6531 = v0;
                v6533 = v0;
                v6535 = v0;
                v6537 = v0;
                v6539 = v0;
                v6541 = v0;
                v6543 = v0;
                v6545 = v0;
                v6547 = v0;
                v6549 = v0;
                v6551 = v0;
                v6553 = v0;
                v6555 = v0;
                v6557 = v0;
                v6559 = v0;
            }
            if v62 != 0.0 {
            } else {
            }
            if v44 != 0.0 {
            } else {
            }
            let v6507 = if v6506 > v0 { 1.0 } else { 0.0 };
            if v6507 != 0.0 {
            } else {
            }
        if v6508 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6509;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6510 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6511;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6512 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6513;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6514 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6515;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6516 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6517;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v6518);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6519 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6520;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6521 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6523;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6525 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6527;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6529 == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6531;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6533 == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6535;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6537 == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6539;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6541 == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6543;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6545 == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6547;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6549 == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6551;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6553 == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6555;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v6557 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v6559;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

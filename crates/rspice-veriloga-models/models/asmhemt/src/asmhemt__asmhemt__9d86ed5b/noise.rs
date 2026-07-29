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
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 8] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_IDS", label: Some("ids"), kind: GeneratedNoiseKind::White, equation: 65, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_FP4_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 66, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(18), name: "fp4", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_FP4S_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 67, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(22), name: "fp4s", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 68, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 69, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("igs"), kind: GeneratedNoiseKind::White, equation: 70, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("igd"), kind: GeneratedNoiseKind::White, equation: 71, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 108, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(7), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16]), ctx.node_voltage(self.nodes[17]), ctx.node_voltage(self.nodes[18]), ctx.node_voltage(self.nodes[19]), ctx.node_voltage(self.nodes[20]), ctx.node_voltage(self.nodes[21]), ctx.node_voltage(self.nodes[22])];
            let v0 = 0e0f64;
            let v1 = 1e0f64;
            let v2 = 1e-2f64;
            let v3 = 0.0f64;
            let v4 = parameters[31];
            let v6 = parameters[32];
            let v9 = parameters[34];
            let v10 = parameters[149];
            let v13 = parameters[0];
            let v14 = 2.7315e2f64;
            let v16 = node_potentials[7];
            let v17 = node_potentials[8];
            let v19 = node_potentials[9];
            let v22 = node_potentials[3];
            let v26 = -1e0f64;
            let v32 = 1e-1f64;
            let v34 = node_potentials[0];
            let v35 = node_potentials[2];
            let v41 = temperature;
            let v42 = node_potentials[4];
            let v44 = parameters[274];
            let v46 = 8.617087e-5f64;
            let v48 = parameters[81];
            let v51 = node_potentials[6];
            let v52 = node_potentials[5];
            let v53 = 5e-1f64;
            let v57 = 2.5e-1f64;
            let v58 = parameters[128];
            let v65 = parameters[100];
            let v66 = parameters[101];
            let v67 = -1e0f64;
            let v72 = parameters[104];
            let v73 = parameters[105];
            let v74 = -1e0f64;
            let v79 = parameters[106];
            let v80 = parameters[107];
            let v81 = -1e0f64;
            let v86 = parameters[102];
            let v87 = parameters[103];
            let v88 = -1e0f64;
            let v93 = 2e0f64;
            let v95 = node_potentials[1];
            let v96 = parameters[113];
            let v98 = parameters[116];
            let v101 = parameters[117];
            let v104 = parameters[118];
            let v106 = parameters[114];
            let v108 = parameters[115];
            let v110 = 3e0f64;
            let v112 = parameters[10];
            let v113 = parameters[121];
            let v116 = parameters[126];
            let v119 = 4e0f64;
            let v122 = node_potentials[12];
            let v123 = 1e-12f64;
            let v126 = node_potentials[14];
            let v129 = 2.5000000000000003e-61f64;
            let v136 = 2.5000000000000003e-61f64;
            let v141 = parameters[89];
            let v147 = parameters[91];
            let v152 = parameters[90];
            let v158 = parameters[92];
            let v163 = parameters[93];
            let v164 = parameters[13];
            let v168 = parameters[94];
            let v169 = parameters[17];
            let v173 = parameters[95];
            let v174 = parameters[36];
            let v178 = parameters[96];
            let v179 = parameters[37];
            let v183 = 5e0f64;
            let v209 = parameters[147];
            let v213 = parameters[148];
            let v217 = parameters[9];
            let v218 = parameters[1];
            let v220 = parameters[2];
            let v222 = parameters[26];
            let v224 = parameters[27];
            let v246 = parameters[22];
            let v254 = parameters[23];
            let v265 = parameters[24];
            let v277 = parameters[11];
            let v282 = parameters[3];
            let v283 = parameters[4];
            let v285 = 1.602176634e-19f64;
            let v287 = 3.24e17f64;
            let v292 = parameters[30];
            let v300 = 1e-4f64;
            let v307 = 5.19105229416e-2f64;
            let v310 = 2.718281828459045e0f64;
            let v316 = 3.6e-1f64;
            let v337 = parameters[28];
            let v340 = 6.666666666666666e-1f64;
            let v354 = 2e2f64;
            let v358 = -3e0f64;
            let v371 = -1e0f64;
            let v382 = -1e0f64;
            let v394 = 1e-19f64;
            let v399 = 4e-18f64;
            let v406 = -3.333333333333333e-1f64;
            let v410 = parameters[29];
            let v418 = 3.7e1f64;
            let v420 = -3.7e1f64;
            let v426 = -3.7e1f64;
            let v455 = -1e0f64;
            let v466 = 4e-18f64;
            let v471 = -3.333333333333333e-1f64;
            let v482 = -3.7e1f64;
            let v488 = -3.7e1f64;
            let v516 = -1e0f64;
            let v536 = parameters[20];
            let v539 = parameters[19];
            let v551 = parameters[14];
            let v554 = parameters[15];
            let v558 = parameters[16];
            let v564 = 3.6e-1f64;
            let v574 = parameters[18];
            let v577 = -1e0f64;
            let v584 = 3.6e-1f64;
            let v617 = -3e0f64;
            let v630 = -1e0f64;
            let v641 = -1e0f64;
            let v657 = 4e-18f64;
            let v664 = -3.333333333333333e-1f64;
            let v676 = -3.7e1f64;
            let v682 = -3.7e1f64;
            let v711 = -1e0f64;
            let v722 = 4e-18f64;
            let v736 = -3.7e1f64;
            let v742 = -3.7e1f64;
            let v756 = -3.333333333333333e-1f64;
            let v759 = -3.333333333333333e-1f64;
            let v774 = -1e0f64;
            let v802 = parameters[5];
            let v805 = parameters[21];
            let v810 = parameters[25];
            let v818 = parameters[269];
            let v819 = parameters[271];
            let v823 = parameters[270];
            let v824 = parameters[272];
            let v828 = parameters[268];
            let v829 = parameters[273];
            let v838 = 8e1f64;
            let v843 = parameters[6];
            let v853 = 6e0f64;
            let v858 = 1e26f64;
            let v859 = parameters[233];
            let v862 = parameters[232];
            let v865 = parameters[231];
            let v876 = 8.333333333333333e-2f64;
            let v880 = 8.333333333333333e-3f64;
            let v893 = -1e0f64;
            let v897 = parameters[56];
            let v900 = parameters[57];
            let v904 = parameters[63];
            let v905 = parameters[71];
            let v915 = parameters[60];
            let v919 = parameters[64];
            let v920 = parameters[72];
            let v929 = parameters[67];
            let v930 = parameters[75];
            let v933 = parameters[77];
            let v936 = parameters[61];
            let v937 = parameters[79];
            let v956 = 1e-3f64;
            let v964 = parameters[69];
            let v969 = parameters[65];
            let v970 = parameters[73];
            let v979 = parameters[68];
            let v980 = parameters[76];
            let v983 = parameters[78];
            let v986 = parameters[62];
            let v987 = parameters[80];
            let v1011 = parameters[70];
            let v1016 = parameters[66];
            let v1017 = parameters[74];
            let v1044 = parameters[58];
            let v1102 = parameters[59];
            let v1278 = if parameter_given[45] { 1.0 } else { 0.0 };
            let v1279 = if parameter_given[44] { 1.0 } else { 0.0 };
            let v1283 = parameters[50];
            let v1301 = parameters[12];
            let v1315 = parameters[38];
            let v1319 = parameters[35];
            let v1320 = parameters[51];
            let v1325 = parameters[40];
            let v1326 = parameters[52];
            let v1329 = parameters[46];
            let v1333 = parameters[45];
            let v1352 = 9e-1f64;
            let v1356 = 1.0000000000000002e-2f64;
            let v1360 = -5.538513813741708e-3f64;
            let v1363 = parameters[42];
            let v1370 = parameters[48];
            let v1371 = parameters[54];
            let v1405 = parameters[39];
            let v1411 = parameters[41];
            let v1412 = parameters[53];
            let v1415 = parameters[47];
            let v1419 = parameters[44];
            let v1441 = 1.0000000000000002e-2f64;
            let v1445 = -5.538513813741708e-3f64;
            let v1448 = parameters[43];
            let v1455 = parameters[49];
            let v1456 = parameters[55];
            let v1477 = node_potentials[18];
            let v1478 = node_potentials[22];
            let v1480 = parameters[260];
            let v1482 = parameters[265];
            let v1483 = 1e-10f64;
            let v1488 = 5.522438177818063e-23f64;
            let v1531 = 3.204353268e-19f64;
            let v1535 = 3.204353268e-19f64;
            let v1540 = parameters[150];
            let v1542 = node_potentials[15];
            let v1549 = -1e0f64;
            let v1558 = parameters[165];
            let v1560 = parameters[166];
            let v1564 = parameters[159];
            let v1565 = parameters[162];
            let v1568 = parameters[167];
            let v1569 = parameters[168];
            let v1578 = parameters[160];
            let v1580 = parameters[161];
            let v1584 = parameters[158];
            let v1598 = 5.19105229416e-2f64;
            let v1606 = 3.6e-1f64;
            let v1627 = parameters[169];
            let v1646 = -3e0f64;
            let v1659 = -1e0f64;
            let v1670 = -1e0f64;
            let v1686 = 4e-18f64;
            let v1693 = -3.333333333333333e-1f64;
            let v1697 = parameters[170];
            let v1706 = -3.7e1f64;
            let v1712 = -3.7e1f64;
            let v1741 = -1e0f64;
            let v1752 = 4e-18f64;
            let v1757 = -3.333333333333333e-1f64;
            let v1768 = -3.7e1f64;
            let v1774 = -3.7e1f64;
            let v1802 = -1e0f64;
            let v1810 = parameters[163];
            let v1812 = parameters[164];
            let v1832 = 3.6e-1f64;
            let v1844 = -1e0f64;
            let v1851 = 3.6e-1f64;
            let v1884 = -3e0f64;
            let v1897 = -1e0f64;
            let v1908 = -1e0f64;
            let v1924 = 4e-18f64;
            let v1931 = -3.333333333333333e-1f64;
            let v1943 = -3.7e1f64;
            let v1949 = -3.7e1f64;
            let v1978 = -1e0f64;
            let v1989 = 4e-18f64;
            let v2003 = -3.7e1f64;
            let v2006 = -3.7e1f64;
            let v2034 = 5.19105229416e-2f64;
            let v2042 = 3.6e-1f64;
            let v2081 = -3e0f64;
            let v2094 = -1e0f64;
            let v2105 = -1e0f64;
            let v2121 = 4e-18f64;
            let v2128 = -3.333333333333333e-1f64;
            let v2140 = -3.7e1f64;
            let v2146 = -3.7e1f64;
            let v2175 = -1e0f64;
            let v2186 = 4e-18f64;
            let v2191 = -3.333333333333333e-1f64;
            let v2202 = -3.7e1f64;
            let v2208 = -3.7e1f64;
            let v2236 = -1e0f64;
            let v2264 = 3.6e-1f64;
            let v2276 = -1e0f64;
            let v2283 = 3.6e-1f64;
            let v2316 = -3e0f64;
            let v2329 = -1e0f64;
            let v2340 = -1e0f64;
            let v2356 = 4e-18f64;
            let v2363 = -3.333333333333333e-1f64;
            let v2375 = -3.7e1f64;
            let v2381 = -3.7e1f64;
            let v2410 = -1e0f64;
            let v2421 = 4e-18f64;
            let v2435 = -3.7e1f64;
            let v2438 = -3.7e1f64;
            let v2440 = parameters[151];
            let v2442 = node_potentials[19];
            let v2449 = -1e0f64;
            let v2489 = 5.19105229416e-2f64;
            let v2497 = 3.6e-1f64;
            let v2536 = -3e0f64;
            let v2549 = -1e0f64;
            let v2560 = -1e0f64;
            let v2576 = 4e-18f64;
            let v2583 = -3.333333333333333e-1f64;
            let v2595 = -3.7e1f64;
            let v2601 = -3.7e1f64;
            let v2630 = -1e0f64;
            let v2641 = 4e-18f64;
            let v2646 = -3.333333333333333e-1f64;
            let v2657 = -3.7e1f64;
            let v2663 = -3.7e1f64;
            let v2691 = -1e0f64;
            let v2719 = 3.6e-1f64;
            let v2731 = -1e0f64;
            let v2738 = 3.6e-1f64;
            let v2771 = -3e0f64;
            let v2784 = -1e0f64;
            let v2795 = -1e0f64;
            let v2811 = 4e-18f64;
            let v2818 = -3.333333333333333e-1f64;
            let v2830 = -3.7e1f64;
            let v2836 = -3.7e1f64;
            let v2865 = -1e0f64;
            let v2876 = 4e-18f64;
            let v2890 = -3.7e1f64;
            let v2893 = -3.7e1f64;
            let v2919 = 5.19105229416e-2f64;
            let v2927 = 3.6e-1f64;
            let v2966 = -3e0f64;
            let v2979 = -1e0f64;
            let v2990 = -1e0f64;
            let v3006 = 4e-18f64;
            let v3013 = -3.333333333333333e-1f64;
            let v3025 = -3.7e1f64;
            let v3031 = -3.7e1f64;
            let v3060 = -1e0f64;
            let v3071 = 4e-18f64;
            let v3076 = -3.333333333333333e-1f64;
            let v3087 = -3.7e1f64;
            let v3093 = -3.7e1f64;
            let v3121 = -1e0f64;
            let v3149 = 3.6e-1f64;
            let v3161 = -1e0f64;
            let v3168 = 3.6e-1f64;
            let v3201 = -3e0f64;
            let v3214 = -1e0f64;
            let v3225 = -1e0f64;
            let v3241 = 4e-18f64;
            let v3248 = -3.333333333333333e-1f64;
            let v3260 = -3.7e1f64;
            let v3266 = -3.7e1f64;
            let v3295 = -1e0f64;
            let v3306 = 4e-18f64;
            let v3320 = -3.7e1f64;
            let v3323 = -3.7e1f64;
            let v3325 = parameters[152];
            let v3327 = node_potentials[16];
            let v3335 = -1e0f64;
            let v3344 = parameters[178];
            let v3346 = parameters[179];
            let v3350 = parameters[172];
            let v3351 = parameters[175];
            let v3354 = parameters[180];
            let v3355 = parameters[181];
            let v3364 = parameters[173];
            let v3366 = parameters[174];
            let v3370 = parameters[171];
            let v3384 = 5.19105229416e-2f64;
            let v3392 = 3.6e-1f64;
            let v3413 = parameters[182];
            let v3432 = -3e0f64;
            let v3445 = -1e0f64;
            let v3456 = -1e0f64;
            let v3472 = 4e-18f64;
            let v3479 = -3.333333333333333e-1f64;
            let v3483 = parameters[183];
            let v3492 = -3.7e1f64;
            let v3498 = -3.7e1f64;
            let v3527 = -1e0f64;
            let v3538 = 4e-18f64;
            let v3543 = -3.333333333333333e-1f64;
            let v3554 = -3.7e1f64;
            let v3560 = -3.7e1f64;
            let v3588 = -1e0f64;
            let v3596 = parameters[176];
            let v3598 = parameters[177];
            let v3618 = 3.6e-1f64;
            let v3630 = -1e0f64;
            let v3637 = 3.6e-1f64;
            let v3670 = -3e0f64;
            let v3683 = -1e0f64;
            let v3694 = -1e0f64;
            let v3710 = 4e-18f64;
            let v3717 = -3.333333333333333e-1f64;
            let v3729 = -3.7e1f64;
            let v3735 = -3.7e1f64;
            let v3764 = -1e0f64;
            let v3775 = 4e-18f64;
            let v3789 = -3.7e1f64;
            let v3792 = -3.7e1f64;
            let v3820 = 5.19105229416e-2f64;
            let v3828 = 3.6e-1f64;
            let v3867 = -3e0f64;
            let v3880 = -1e0f64;
            let v3891 = -1e0f64;
            let v3907 = 4e-18f64;
            let v3914 = -3.333333333333333e-1f64;
            let v3926 = -3.7e1f64;
            let v3932 = -3.7e1f64;
            let v3961 = -1e0f64;
            let v3972 = 4e-18f64;
            let v3977 = -3.333333333333333e-1f64;
            let v3988 = -3.7e1f64;
            let v3994 = -3.7e1f64;
            let v4022 = -1e0f64;
            let v4050 = 3.6e-1f64;
            let v4062 = -1e0f64;
            let v4069 = 3.6e-1f64;
            let v4102 = -3e0f64;
            let v4115 = -1e0f64;
            let v4126 = -1e0f64;
            let v4142 = 4e-18f64;
            let v4149 = -3.333333333333333e-1f64;
            let v4161 = -3.7e1f64;
            let v4167 = -3.7e1f64;
            let v4196 = -1e0f64;
            let v4207 = 4e-18f64;
            let v4221 = -3.7e1f64;
            let v4224 = -3.7e1f64;
            let v4226 = parameters[153];
            let v4228 = node_potentials[20];
            let v4236 = -1e0f64;
            let v4276 = 5.19105229416e-2f64;
            let v4284 = 3.6e-1f64;
            let v4323 = -3e0f64;
            let v4336 = -1e0f64;
            let v4347 = -1e0f64;
            let v4363 = 4e-18f64;
            let v4370 = -3.333333333333333e-1f64;
            let v4382 = -3.7e1f64;
            let v4388 = -3.7e1f64;
            let v4417 = -1e0f64;
            let v4428 = 4e-18f64;
            let v4433 = -3.333333333333333e-1f64;
            let v4444 = -3.7e1f64;
            let v4450 = -3.7e1f64;
            let v4478 = -1e0f64;
            let v4506 = 3.6e-1f64;
            let v4518 = -1e0f64;
            let v4525 = 3.6e-1f64;
            let v4558 = -3e0f64;
            let v4571 = -1e0f64;
            let v4582 = -1e0f64;
            let v4598 = 4e-18f64;
            let v4605 = -3.333333333333333e-1f64;
            let v4617 = -3.7e1f64;
            let v4623 = -3.7e1f64;
            let v4652 = -1e0f64;
            let v4663 = 4e-18f64;
            let v4677 = -3.7e1f64;
            let v4680 = -3.7e1f64;
            let v4708 = 5.19105229416e-2f64;
            let v4716 = 3.6e-1f64;
            let v4755 = -3e0f64;
            let v4768 = -1e0f64;
            let v4779 = -1e0f64;
            let v4795 = 4e-18f64;
            let v4802 = -3.333333333333333e-1f64;
            let v4814 = -3.7e1f64;
            let v4820 = -3.7e1f64;
            let v4849 = -1e0f64;
            let v4860 = 4e-18f64;
            let v4865 = -3.333333333333333e-1f64;
            let v4876 = -3.7e1f64;
            let v4882 = -3.7e1f64;
            let v4910 = -1e0f64;
            let v4938 = 3.6e-1f64;
            let v4950 = -1e0f64;
            let v4957 = 3.6e-1f64;
            let v4990 = -3e0f64;
            let v5003 = -1e0f64;
            let v5014 = -1e0f64;
            let v5030 = 4e-18f64;
            let v5037 = -3.333333333333333e-1f64;
            let v5049 = -3.7e1f64;
            let v5055 = -3.7e1f64;
            let v5084 = -1e0f64;
            let v5095 = 4e-18f64;
            let v5109 = -3.7e1f64;
            let v5112 = -3.7e1f64;
            let v5114 = parameters[154];
            let v5116 = node_potentials[17];
            let v5124 = -1e0f64;
            let v5133 = parameters[191];
            let v5135 = parameters[192];
            let v5139 = parameters[185];
            let v5140 = parameters[188];
            let v5143 = parameters[193];
            let v5144 = parameters[194];
            let v5153 = parameters[186];
            let v5155 = parameters[187];
            let v5159 = parameters[184];
            let v5173 = 5.19105229416e-2f64;
            let v5181 = 3.6e-1f64;
            let v5202 = parameters[195];
            let v5221 = -3e0f64;
            let v5234 = -1e0f64;
            let v5245 = -1e0f64;
            let v5261 = 4e-18f64;
            let v5268 = -3.333333333333333e-1f64;
            let v5272 = parameters[196];
            let v5281 = -3.7e1f64;
            let v5287 = -3.7e1f64;
            let v5316 = -1e0f64;
            let v5327 = 4e-18f64;
            let v5332 = -3.333333333333333e-1f64;
            let v5343 = -3.7e1f64;
            let v5349 = -3.7e1f64;
            let v5377 = -1e0f64;
            let v5385 = parameters[189];
            let v5387 = parameters[190];
            let v5407 = 3.6e-1f64;
            let v5419 = -1e0f64;
            let v5426 = 3.6e-1f64;
            let v5459 = -3e0f64;
            let v5472 = -1e0f64;
            let v5483 = -1e0f64;
            let v5499 = 4e-18f64;
            let v5506 = -3.333333333333333e-1f64;
            let v5518 = -3.7e1f64;
            let v5524 = -3.7e1f64;
            let v5553 = -1e0f64;
            let v5564 = 4e-18f64;
            let v5578 = -3.7e1f64;
            let v5581 = -3.7e1f64;
            let v5609 = 5.19105229416e-2f64;
            let v5617 = 3.6e-1f64;
            let v5656 = -3e0f64;
            let v5669 = -1e0f64;
            let v5680 = -1e0f64;
            let v5696 = 4e-18f64;
            let v5703 = -3.333333333333333e-1f64;
            let v5715 = -3.7e1f64;
            let v5721 = -3.7e1f64;
            let v5750 = -1e0f64;
            let v5761 = 4e-18f64;
            let v5766 = -3.333333333333333e-1f64;
            let v5777 = -3.7e1f64;
            let v5783 = -3.7e1f64;
            let v5811 = -1e0f64;
            let v5839 = 3.6e-1f64;
            let v5851 = -1e0f64;
            let v5858 = 3.6e-1f64;
            let v5891 = -3e0f64;
            let v5904 = -1e0f64;
            let v5915 = -1e0f64;
            let v5931 = 4e-18f64;
            let v5938 = -3.333333333333333e-1f64;
            let v5950 = -3.7e1f64;
            let v5956 = -3.7e1f64;
            let v5985 = -1e0f64;
            let v5996 = 4e-18f64;
            let v6010 = -3.7e1f64;
            let v6013 = -3.7e1f64;
            let v6015 = parameters[155];
            let v6017 = node_potentials[21];
            let v6025 = -1e0f64;
            let v6065 = 5.19105229416e-2f64;
            let v6073 = 3.6e-1f64;
            let v6112 = -3e0f64;
            let v6125 = -1e0f64;
            let v6136 = -1e0f64;
            let v6152 = 4e-18f64;
            let v6159 = -3.333333333333333e-1f64;
            let v6171 = -3.7e1f64;
            let v6177 = -3.7e1f64;
            let v6206 = -1e0f64;
            let v6217 = 4e-18f64;
            let v6222 = -3.333333333333333e-1f64;
            let v6233 = -3.7e1f64;
            let v6239 = -3.7e1f64;
            let v6267 = -1e0f64;
            let v6295 = 3.6e-1f64;
            let v6307 = -1e0f64;
            let v6314 = 3.6e-1f64;
            let v6347 = -3e0f64;
            let v6360 = -1e0f64;
            let v6371 = -1e0f64;
            let v6387 = 4e-18f64;
            let v6394 = -3.333333333333333e-1f64;
            let v6406 = -3.7e1f64;
            let v6412 = -3.7e1f64;
            let v6441 = -1e0f64;
            let v6452 = 4e-18f64;
            let v6466 = -3.7e1f64;
            let v6469 = -3.7e1f64;
            let v6497 = 5.19105229416e-2f64;
            let v6505 = 3.6e-1f64;
            let v6544 = -3e0f64;
            let v6557 = -1e0f64;
            let v6568 = -1e0f64;
            let v6584 = 4e-18f64;
            let v6591 = -3.333333333333333e-1f64;
            let v6603 = -3.7e1f64;
            let v6609 = -3.7e1f64;
            let v6638 = -1e0f64;
            let v6649 = 4e-18f64;
            let v6654 = -3.333333333333333e-1f64;
            let v6665 = -3.7e1f64;
            let v6671 = -3.7e1f64;
            let v6699 = -1e0f64;
            let v6727 = 3.6e-1f64;
            let v6739 = -1e0f64;
            let v6746 = 3.6e-1f64;
            let v6779 = -3e0f64;
            let v6792 = -1e0f64;
            let v6803 = -1e0f64;
            let v6819 = 4e-18f64;
            let v6826 = -3.333333333333333e-1f64;
            let v6838 = -3.7e1f64;
            let v6844 = -3.7e1f64;
            let v6873 = -1e0f64;
            let v6884 = 4e-18f64;
            let v6898 = -3.7e1f64;
            let v6901 = -3.7e1f64;
            let v6903 = parameters[156];
            let v6912 = -1e0f64;
            let v6921 = parameters[204];
            let v6923 = parameters[205];
            let v6927 = parameters[198];
            let v6928 = parameters[201];
            let v6931 = parameters[206];
            let v6932 = parameters[207];
            let v6941 = parameters[199];
            let v6943 = parameters[200];
            let v6947 = parameters[197];
            let v6961 = 5.19105229416e-2f64;
            let v6969 = 3.6e-1f64;
            let v6990 = parameters[208];
            let v7009 = -3e0f64;
            let v7022 = -1e0f64;
            let v7033 = -1e0f64;
            let v7049 = 4e-18f64;
            let v7056 = -3.333333333333333e-1f64;
            let v7060 = parameters[209];
            let v7069 = -3.7e1f64;
            let v7075 = -3.7e1f64;
            let v7104 = -1e0f64;
            let v7115 = 4e-18f64;
            let v7120 = -3.333333333333333e-1f64;
            let v7131 = -3.7e1f64;
            let v7137 = -3.7e1f64;
            let v7165 = -1e0f64;
            let v7173 = parameters[202];
            let v7175 = parameters[203];
            let v7195 = 3.6e-1f64;
            let v7207 = -1e0f64;
            let v7214 = 3.6e-1f64;
            let v7247 = -3e0f64;
            let v7260 = -1e0f64;
            let v7271 = -1e0f64;
            let v7287 = 4e-18f64;
            let v7294 = -3.333333333333333e-1f64;
            let v7306 = -3.7e1f64;
            let v7312 = -3.7e1f64;
            let v7341 = -1e0f64;
            let v7352 = 4e-18f64;
            let v7366 = -3.7e1f64;
            let v7369 = -3.7e1f64;
            let v7397 = 5.19105229416e-2f64;
            let v7405 = 3.6e-1f64;
            let v7444 = -3e0f64;
            let v7457 = -1e0f64;
            let v7468 = -1e0f64;
            let v7484 = 4e-18f64;
            let v7491 = -3.333333333333333e-1f64;
            let v7503 = -3.7e1f64;
            let v7509 = -3.7e1f64;
            let v7538 = -1e0f64;
            let v7549 = 4e-18f64;
            let v7554 = -3.333333333333333e-1f64;
            let v7565 = -3.7e1f64;
            let v7571 = -3.7e1f64;
            let v7599 = -1e0f64;
            let v7627 = 3.6e-1f64;
            let v7639 = -1e0f64;
            let v7646 = 3.6e-1f64;
            let v7679 = -3e0f64;
            let v7692 = -1e0f64;
            let v7703 = -1e0f64;
            let v7719 = 4e-18f64;
            let v7726 = -3.333333333333333e-1f64;
            let v7738 = -3.7e1f64;
            let v7744 = -3.7e1f64;
            let v7773 = -1e0f64;
            let v7784 = 4e-18f64;
            let v7798 = -3.7e1f64;
            let v7801 = -3.7e1f64;
            let v7803 = parameters[157];
            let v7812 = -1e0f64;
            let v7852 = 5.19105229416e-2f64;
            let v7860 = 3.6e-1f64;
            let v7899 = -3e0f64;
            let v7912 = -1e0f64;
            let v7923 = -1e0f64;
            let v7939 = 4e-18f64;
            let v7946 = -3.333333333333333e-1f64;
            let v7958 = -3.7e1f64;
            let v7964 = -3.7e1f64;
            let v7993 = -1e0f64;
            let v8004 = 4e-18f64;
            let v8009 = -3.333333333333333e-1f64;
            let v8020 = -3.7e1f64;
            let v8026 = -3.7e1f64;
            let v8054 = -1e0f64;
            let v8082 = 3.6e-1f64;
            let v8094 = -1e0f64;
            let v8101 = 3.6e-1f64;
            let v8134 = -3e0f64;
            let v8147 = -1e0f64;
            let v8158 = -1e0f64;
            let v8174 = 4e-18f64;
            let v8181 = -3.333333333333333e-1f64;
            let v8193 = -3.7e1f64;
            let v8199 = -3.7e1f64;
            let v8228 = -1e0f64;
            let v8239 = 4e-18f64;
            let v8253 = -3.7e1f64;
            let v8256 = -3.7e1f64;
            let v8284 = 5.19105229416e-2f64;
            let v8292 = 3.6e-1f64;
            let v8331 = -3e0f64;
            let v8344 = -1e0f64;
            let v8355 = -1e0f64;
            let v8371 = 4e-18f64;
            let v8378 = -3.333333333333333e-1f64;
            let v8390 = -3.7e1f64;
            let v8396 = -3.7e1f64;
            let v8425 = -1e0f64;
            let v8436 = 4e-18f64;
            let v8441 = -3.333333333333333e-1f64;
            let v8452 = -3.7e1f64;
            let v8458 = -3.7e1f64;
            let v8486 = -1e0f64;
            let v8514 = 3.6e-1f64;
            let v8526 = -1e0f64;
            let v8533 = 3.6e-1f64;
            let v8566 = -3e0f64;
            let v8579 = -1e0f64;
            let v8590 = -1e0f64;
            let v8606 = 4e-18f64;
            let v8613 = -3.333333333333333e-1f64;
            let v8625 = -3.7e1f64;
            let v8631 = -3.7e1f64;
            let v8660 = -1e0f64;
            let v8671 = 4e-18f64;
            let v8685 = -3.7e1f64;
            let v8688 = -3.7e1f64;
            let v8690 = parameters[255];
            let v8692 = parameters[258];
            let v8693 = parameters[256];
            let v8695 = parameters[257];
            let v8716 = node_potentials[10];
            let v8719 = parameters[210];
            let v8723 = parameters[214];
            let v8729 = parameters[213];
            let v8730 = parameters[211];
            let v8759 = parameters[212];
            let v8772 = parameters[279];
            let v8773 = parameters[285];
            let v8776 = parameters[275];
            let v8777 = parameters[283];
            let v8780 = parameters[277];
            let v8781 = parameters[281];
            let v8785 = parameters[280];
            let v8786 = parameters[286];
            let v8789 = parameters[276];
            let v8790 = parameters[284];
            let v8793 = parameters[278];
            let v8794 = parameters[282];
            let v8822 = parameters[259];
            let v8858 = parameters[261];
            let v8861 = 1e-22f64;
            let v8869 = parameters[262];
            let v8876 = parameters[263];
            let v8898 = parameters[8];
            let v8900 = parameters[264];
            if v3 != 0.0 {
                let v8 = if (if v4 == v0 { 1.0 } else { 0.0 }) != 0.0 || (if v6 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v8 != 0.0 {
                } else {
                }
            } else {
            }
            let v11 = if v10 == v1 { 1.0 } else { 0.0 };
            let v1280: f64;
            if v11 != 0.0 {
                let v12 = if v9 == v0 { 1.0 } else { 0.0 };
                let v1281: f64;
                if v12 != 0.0 {
                    v1281 = v1;
                } else {
                    v1281 = v9;
                }
                v1280 = v1281;
            } else {
                v1280 = v9;
            }
            let v15 = v13 + v14;
            let v18 = v16 - v17;
            let v20 = v19 - v17;
            let v21 = v19 - v16;
            let v23 = v22 - v17;
            let v24 = v22 - v16;
            let v25 = if v18 < v0 { 1.0 } else { 0.0 };
            let v28: f64;
            let v279: f64;
            let v297: f64;
            let v844: f64;
            if v25 != 0.0 {
                let v27 = v26 * v18;
                v28 = v27;
                v279 = v24;
                v297 = v21;
                v844 = v26;
            } else {
                v28 = v18;
                v279 = v23;
                v297 = v20;
                v844 = v1;
            }
            let v33 = (((v28 * v28) + v2).sqrt()) - v32;
            let v36 = v34 - v35;
            let v37 = v36 * v36;
            let v40 = ((v37 + v2).sqrt()) - v32;
            let v45 = (v41 + v42) + v44;
            let v47 = v46 * v45;
            let v49 = if v48 == v0 { 1.0 } else { 0.0 };
            let v225: f64;
            let v232: f64;
            let v239: f64;
            let v247: f64;
            let v251: f64;
            let v268: f64;
            let v272: f64;
            let v524: f64;
            let v530: f64;
            let v1287: f64;
            let v1294: f64;
            let v1377: f64;
            let v1381: f64;
            let v1388: f64;
            let v1462: f64;
            let v1467: f64;
            let v1471: f64;
            if v49 != 0.0 {
                v225 = v0;
                v232 = v0;
                v239 = v0;
                v247 = v0;
                v251 = v0;
                v268 = v0;
                v272 = v0;
                v524 = v0;
                v530 = v0;
                v1287 = v0;
                v1294 = v0;
                v1377 = v0;
                v1381 = v0;
                v1388 = v0;
                v1462 = v0;
                v1467 = v0;
                v1471 = v0;
            } else {
                let v50 = if v48 == v1 { 1.0 } else { 0.0 };
                let v226: f64;
                let v233: f64;
                let v240: f64;
                let v248: f64;
                let v252: f64;
                let v269: f64;
                let v273: f64;
                let v525: f64;
                let v531: f64;
                let v1288: f64;
                let v1295: f64;
                let v1378: f64;
                let v1382: f64;
                let v1389: f64;
                let v1463: f64;
                let v1468: f64;
                let v1472: f64;
                if v50 != 0.0 {
                    let v55 = v52 - v47;
                    let v64 = v53 * ((v52 + v47) + (((v55 * v55) + ((v57 * v58) * v58)).sqrt()));
                    let v71 = v65 + (v66 * (rspice_limited_exp((v67 / v64))));
                    let v78 = v72 + (v73 * (rspice_limited_exp((v74 / v64))));
                    let v85 = v79 + (v80 * (rspice_limited_exp((v81 / v64))));
                    let v92 = v86 + (v87 * (rspice_limited_exp((v88 / v64))));
                    v226 = v0;
                    v233 = v0;
                    v240 = v0;
                    v248 = v0;
                    v252 = v92;
                    v269 = v0;
                    v273 = v71;
                    v525 = v0;
                    v531 = v0;
                    v1288 = v0;
                    v1295 = v0;
                    v1378 = v78;
                    v1382 = v0;
                    v1389 = v0;
                    v1463 = v0;
                    v1468 = v0;
                    v1472 = v85;
                } else {
                    let v94 = if v48 == v93 { 1.0 } else { 0.0 };
                    let v227: f64;
                    let v234: f64;
                    let v241: f64;
                    let v249: f64;
                    let v270: f64;
                    let v526: f64;
                    let v532: f64;
                    let v1289: f64;
                    let v1296: f64;
                    let v1383: f64;
                    let v1390: f64;
                    let v1464: f64;
                    let v1469: f64;
                    if v94 != 0.0 {
                        let v97 = v96 * v51;
                        let v105 = (((-v98) * v52) + (v101 * v51)) + v104;
                        let v107 = v106 * v51;
                        let v109 = v108 * v51;
                        v227 = v107;
                        v234 = v0;
                        v241 = v0;
                        v249 = v109;
                        v270 = v97;
                        v526 = v0;
                        v532 = v0;
                        v1289 = v0;
                        v1296 = v0;
                        v1383 = v0;
                        v1390 = v0;
                        v1464 = v0;
                        v1469 = v105;
                    } else {
                        let v111 = if v48 == v110 { 1.0 } else { 0.0 };
                        let v235: f64;
                        let v242: f64;
                        let v527: f64;
                        let v533: f64;
                        let v1290: f64;
                        let v1297: f64;
                        let v1384: f64;
                        let v1391: f64;
                        let v1465: f64;
                        if v111 != 0.0 {
                            let v118 = (v52 / v113) * ((v45 / v15).powf(v116));
                            v235 = v0;
                            v242 = v0;
                            v527 = v0;
                            v533 = v0;
                            v1290 = v0;
                            v1297 = v0;
                            v1384 = v0;
                            v1391 = v0;
                            v1465 = v118;
                        } else {
                            let v120 = if v48 == v119 { 1.0 } else { 0.0 };
                            let v236: f64;
                            let v243: f64;
                            let v528: f64;
                            let v534: f64;
                            let v1291: f64;
                            let v1298: f64;
                            let v1385: f64;
                            let v1392: f64;
                            if v120 != 0.0 {
                                let v127 = v122 - (v36.abs());
                                let v133 = v53 * (v127 + (((v127 * v127) + v129).sqrt()));
                                let v134 = v126 - ((v95 - v35).abs());
                                let v140 = v53 * (v134 + (((v134 * v134) + v136).sqrt()));
                                let v150 = (v133 * v141) / (((v133 * v133) + (v141 * v141)).sqrt());
                                let v151 = ((v147 * v112).abs()) * v150;
                                let v161 = (v140 * v152) / (((v140 * v140) + (v152 * v152)).sqrt());
                                let v162 = ((v158 * v112).abs()) * v161;
                                let v167 = ((v163 * v164).abs()) * v161;
                                let v172 = ((v168 * v169).abs()) * v161;
                                let v177 = ((v173 * v174).abs()) * v150;
                                let v182 = ((v178 * v179).abs()) * v150;
                                v236 = v151;
                                v243 = v162;
                                v528 = v167;
                                v534 = v172;
                                v1291 = v177;
                                v1298 = v0;
                                v1385 = v182;
                                v1392 = v0;
                            } else {
                                let v184 = if v48 == v183 { 1.0 } else { 0.0 };
                                let v237: f64;
                                let v244: f64;
                                let v1292: f64;
                                let v1299: f64;
                                let v1386: f64;
                                let v1393: f64;
                                if v184 != 0.0 {
                                    let v192 = (v52 * v141) / (((v52 * v52) + (v141 * v141)).sqrt());
                                    let v193 = ((v147 * v112).abs()) * v192;
                                    let v196 = ((v173 * v174).abs()) * v192;
                                    let v199 = ((v178 * v179).abs()) * v192;
                                    let v207 = (v51 * v152) / (((v51 * v51) + (v152 * v152)).sqrt());
                                    let v208 = ((v158 * v112).abs()) * v207;
                                    let v212 = ((v209 * v174).abs()) * v207;
                                    let v216 = ((v213 * v179).abs()) * v207;
                                    v237 = v193;
                                    v244 = v208;
                                    v1292 = v196;
                                    v1299 = v212;
                                    v1386 = v199;
                                    v1393 = v216;
                                } else {
                                    v237 = v0;
                                    v244 = v0;
                                    v1292 = v0;
                                    v1299 = v0;
                                    v1386 = v0;
                                    v1393 = v0;
                                }
                                v236 = v237;
                                v243 = v244;
                                v528 = v0;
                                v534 = v0;
                                v1291 = v1292;
                                v1298 = v1299;
                                v1385 = v1386;
                                v1392 = v1393;
                            }
                            v235 = v236;
                            v242 = v243;
                            v527 = v528;
                            v533 = v534;
                            v1290 = v1291;
                            v1297 = v1298;
                            v1384 = v1385;
                            v1391 = v1392;
                            v1465 = v0;
                        }
                        v227 = v0;
                        v234 = v235;
                        v241 = v242;
                        v249 = v0;
                        v270 = v0;
                        v526 = v527;
                        v532 = v533;
                        v1289 = v1290;
                        v1296 = v1297;
                        v1383 = v1384;
                        v1390 = v1391;
                        v1464 = v1465;
                        v1469 = v0;
                    }
                    v226 = v227;
                    v233 = v234;
                    v240 = v241;
                    v248 = v249;
                    v252 = v0;
                    v269 = v270;
                    v273 = v0;
                    v525 = v526;
                    v531 = v532;
                    v1288 = v1289;
                    v1295 = v1296;
                    v1378 = v0;
                    v1382 = v1383;
                    v1389 = v1390;
                    v1463 = v1464;
                    v1468 = v1469;
                    v1472 = v0;
                }
                v225 = v226;
                v232 = v233;
                v239 = v240;
                v247 = v248;
                v251 = v252;
                v268 = v269;
                v272 = v273;
                v524 = v525;
                v530 = v531;
                v1287 = v1288;
                v1294 = v1295;
                v1377 = v1378;
                v1381 = v1382;
                v1388 = v1389;
                v1462 = v1463;
                v1467 = v1468;
                v1471 = v1472;
            }
            let v219 = v217 / v218;
            let v221 = v217 / v220;
            let v231 = v47 * ((v1 + v222) + ((v224 + v225) * v33));
            let v263 = v45 / v15;
            let v264 = v263 - v1;
            let v281 = ((((((v112 + v232) + v239) - ((((v246 + v247) - v251) * (v33 * v254)) / (((v33 * v33) + (v254 * v254)).sqrt()))) - (v264 * v265)) + v268) + v272) + (((v221 / (v221 + v219)) * v277) * v279);
            let v284 = v93 * v283;
            let v288 = (v284 * v285) * v287;
            let v296 = v281 + (v231 * (((v282 / ((v288 * v231) * v231)) * v292).ln()));
            let v298 = v297 - v296;
            let v306 = ((v53 * (v298 + (((v298 * v298) + v300).sqrt()))) + v296) - v281;
            let v309 = v219 / (v307 * v231);
            let v311 = v310 / v309;
            let v312 = v1 / v309;
            let v313 = v219 / v285;
            let v314 = v53 * v306;
            let v315 = v306 * v306;
            let v320 = v314 + (v53 * ((v315 + v316).sqrt()));
            let v322 = v320 * v320;
            let v323 = v311 * v311;
            let v328 = v312 * v312;
            let v338 = v337 / v110;
            let v341 = (v313 * v320).powf(v340);
            let v348 = (v93 * v337) / v110;
            let v351 = ((v320 + (v231 * (v1 - ((v309 * ((v320 * v311) / ((v322 + v323).sqrt()))).ln())))) - (v338 * v341)) / ((v320 * (v1 + (v231 / ((v320 * v312) / ((v322 + v328).sqrt()))))) + (v348 * v341));
            let v352 = v93 * v231;
            let v353 = v306 / v352;
            let v355 = if v353 < v354 { 1.0 } else { 0.0 };
            let v389: f64;
            if v355 != 0.0 {
                let v377 = ((v352 * v313) * (((v110 * v353) / v119) + (((rspice_limited_exp((v353 / v119))) + (rspice_limited_exp(((v358 * v353) / v119)))).ln()))) / ((v1 / v351) + ((v313 / v287) * (rspice_limited_exp(((v371 * v306) / v352)))));
                v389 = v377;
            } else {
                let v388 = ((v352 * v313) * v353) / ((v1 / v351) + ((v313 / v287) * (rspice_limited_exp(((v382 * v306) / v352)))));
                v389 = v388;
            }
            let v391 = v306 - (v389 / v313);
            let v395 = if ((v391 - v306).abs()) > v394 { 1.0 } else { 0.0 };
            let v543: f64;
            if v395 != 0.0 {
                let v396 = v306 - v391;
                let v403 = (v53 * v396) + (v53 * (((v396 * v396) + v399).sqrt()));
                let v404 = v313.powf(v340);
                let v405 = v403.powf(v340);
                let v407 = v403.powf(v406);
                let v408 = v337 * v404;
                let v411 = v410 * v404;
                let v413 = v391 / v231;
                let v415 = v413 - ((v408 * v405) / v231);
                let v417 = v413 - ((v411 * v405) / v231);
                let v419 = if v415 >= v418 { 1.0 } else { 0.0 };
                let v433: f64;
                if v419 != 0.0 {
                    v433 = v415;
                } else {
                    let v421 = if v415 <= v420 { 1.0 } else { 0.0 };
                    let v434: f64;
                    if v421 != 0.0 {
                        v434 = v0;
                    } else {
                        let v424 = ((v415.exp()) + v1).ln();
                        v434 = v424;
                    }
                    v433 = v434;
                }
                let v425 = if v417 >= v418 { 1.0 } else { 0.0 };
                let v437: f64;
                if v425 != 0.0 {
                    v437 = v417;
                } else {
                    let v427 = if v417 <= v426 { 1.0 } else { 0.0 };
                    let v438: f64;
                    if v427 != 0.0 {
                        v438 = v0;
                    } else {
                        let v430 = ((v417.exp()) + v1).ln();
                        v438 = v430;
                    }
                    v437 = v438;
                }
                let v432 = v287 * v231;
                let v443 = rspice_limited_exp(v415);
                let v449 = rspice_limited_exp(v417);
                let v462 = v391 - ((((v313 * v403) - (v432 * v433)) - (v432 * v437)) / (((v455 * v313) - (((v443 * v287) * (v1 + (v340 * (v408 * v407)))) / (v1 + v443))) - (((v449 * v287) * (v1 + (v340 * (v411 * v407)))) / (v1 + v449))));
                let v463 = v306 - v462;
                let v470 = (v53 * v463) + (v53 * (((v463 * v463) + v466).sqrt()));
                let v472 = v470.powf(v471);
                let v473 = v470.powf(v340);
                let v476 = v462 / v231;
                let v478 = v476 - ((v408 * v473) / v231);
                let v480 = v476 - ((v411 * v473) / v231);
                let v481 = if v478 >= v418 { 1.0 } else { 0.0 };
                let v494: f64;
                if v481 != 0.0 {
                    v494 = v478;
                } else {
                    let v483 = if v478 <= v482 { 1.0 } else { 0.0 };
                    let v495: f64;
                    if v483 != 0.0 {
                        v495 = v0;
                    } else {
                        let v486 = ((v478.exp()) + v1).ln();
                        v495 = v486;
                    }
                    v494 = v495;
                }
                let v487 = if v480 >= v418 { 1.0 } else { 0.0 };
                let v498: f64;
                if v487 != 0.0 {
                    v498 = v480;
                } else {
                    let v489 = if v480 <= v488 { 1.0 } else { 0.0 };
                    let v499: f64;
                    if v489 != 0.0 {
                        v499 = v0;
                    } else {
                        let v492 = ((v480.exp()) + v1).ln();
                        v499 = v492;
                    }
                    v498 = v499;
                }
                let v504 = rspice_limited_exp(v478);
                let v510 = rspice_limited_exp(v480);
                let v523 = v462 - ((((v313 * v470) - (v432 * v494)) - (v432 * v498)) / (((v516 * v313) - (((v504 * v287) * (v1 + (v340 * (v408 * v472)))) / (v1 + v504))) - (((v510 * v287) * (v1 + (v340 * (v411 * v472)))) / (v1 + v510))));
                v543 = v523;
            } else {
                v543 = v391;
            }
            let v537 = v263.powf(v536);
            let v538 = (v164 - v524) * v537;
            let v540 = v263.powf(v539);
            let v542 = v219 / v217;
            let v546 = v542 * ((v306 - v543).abs());
            let v547 = v221 / v217;
            let v559 = v558 * (v547 * ((v279 - v543).abs()));
            let v568 = v314 + (v53 * ((v315 + v564).sqrt()));
            let v569 = ((v93 * ((v169 - v530) * v540)) / (v538 / (((v1 + (v551 * v546)) + (v554 * (v546 * v546))) + v559))) * v282;
            let v580 = v28 * ((v1 + ((v28 / ((v569 * v568) / (v569 + v568))).powf(v574))).powf((v577 / v574)));
            let v581 = v306 - v580;
            let v588 = (v53 * v581) + (v53 * (((v581 * v581) + v584).sqrt()));
            let v590 = v588 * v588;
            let v604 = (v313 * v588).powf(v340);
            let v612 = ((v588 + (v231 * (v1 - ((v309 * ((v588 * v311) / ((v590 + v323).sqrt()))).ln())))) - (v338 * v604)) / ((v588 * (v1 + (v231 / ((v588 * v312) / ((v590 + v328).sqrt()))))) + (v348 * v604));
            let v613 = v581 / v352;
            let v614 = if v613 < v354 { 1.0 } else { 0.0 };
            let v648: f64;
            if v614 != 0.0 {
                let v636 = ((v352 * v313) * (((v110 * v613) / v119) + (((rspice_limited_exp((v613 / v119))) + (rspice_limited_exp(((v617 * v613) / v119)))).ln()))) / ((v1 / v612) + ((v313 / v287) * (rspice_limited_exp(((v630 * v581) / v352)))));
                v648 = v636;
            } else {
                let v647 = ((v352 * v313) * v613) / ((v1 / v612) + ((v313 / v287) * (rspice_limited_exp(((v641 * v581) / v352)))));
                v648 = v647;
            }
            let v650 = v581 - (v648 / v313);
            let v653 = if ((v650 - v581).abs()) > v394 { 1.0 } else { 0.0 };
            let v784: f64;
            if v653 != 0.0 {
                let v654 = v581 - v650;
                let v661 = (v53 * v654) + (v53 * (((v654 * v654) + v657).sqrt()));
                let v662 = v313.powf(v340);
                let v663 = v661.powf(v340);
                let v665 = v661.powf(v664);
                let v666 = v337 * v662;
                let v668 = v410 * v662;
                let v670 = v650 / v231;
                let v672 = v670 - ((v666 * v663) / v231);
                let v674 = v670 - ((v668 * v663) / v231);
                let v675 = if v672 >= v418 { 1.0 } else { 0.0 };
                let v689: f64;
                if v675 != 0.0 {
                    v689 = v672;
                } else {
                    let v677 = if v672 <= v676 { 1.0 } else { 0.0 };
                    let v690: f64;
                    if v677 != 0.0 {
                        v690 = v0;
                    } else {
                        let v680 = ((v672.exp()) + v1).ln();
                        v690 = v680;
                    }
                    v689 = v690;
                }
                let v681 = if v674 >= v418 { 1.0 } else { 0.0 };
                let v693: f64;
                if v681 != 0.0 {
                    v693 = v674;
                } else {
                    let v683 = if v674 <= v682 { 1.0 } else { 0.0 };
                    let v694: f64;
                    if v683 != 0.0 {
                        v694 = v0;
                    } else {
                        let v686 = ((v674.exp()) + v1).ln();
                        v694 = v686;
                    }
                    v693 = v694;
                }
                let v688 = v287 * v231;
                let v699 = rspice_limited_exp(v672);
                let v705 = rspice_limited_exp(v674);
                let v718 = v650 - ((((v313 * v661) - (v688 * v689)) - (v688 * v693)) / (((v711 * v313) - (((v699 * v287) * (v1 + (v340 * (v666 * v665)))) / (v1 + v699))) - (((v705 * v287) * (v1 + (v340 * (v668 * v665)))) / (v1 + v705))));
                let v719 = v581 - v718;
                let v726 = (v53 * v719) + (v53 * (((v719 * v719) + v722).sqrt()));
                let v727 = v726.powf(v340);
                let v730 = v718 / v231;
                let v732 = v730 - ((v666 * v727) / v231);
                let v734 = v730 - ((v668 * v727) / v231);
                let v735 = if v732 >= v418 { 1.0 } else { 0.0 };
                let v748: f64;
                if v735 != 0.0 {
                    v748 = v732;
                } else {
                    let v737 = if v732 <= v736 { 1.0 } else { 0.0 };
                    let v749: f64;
                    if v737 != 0.0 {
                        v749 = v0;
                    } else {
                        let v740 = ((v732.exp()) + v1).ln();
                        v749 = v740;
                    }
                    v748 = v749;
                }
                let v741 = if v734 >= v418 { 1.0 } else { 0.0 };
                let v752: f64;
                if v741 != 0.0 {
                    v752 = v734;
                } else {
                    let v743 = if v734 <= v742 { 1.0 } else { 0.0 };
                    let v753: f64;
                    if v743 != 0.0 {
                        v753 = v0;
                    } else {
                        let v746 = ((v734.exp()) + v1).ln();
                        v753 = v746;
                    }
                    v752 = v753;
                }
                let v762 = rspice_limited_exp(v732);
                let v768 = rspice_limited_exp(v734);
                let v782 = (v718 - ((((v313 * v726) - (v688 * v748)) - (v688 * v752)) / (((v774 * v313) - (((v762 * v287) * (v1 + (v340 * (v666 * (v726.powf(v756)))))) / (v1 + v762))) - (((v768 * v287) * (v1 + (v340 * (v668 * (v726.powf(v759)))))) / (v1 + v768))))) + v580;
                v784 = v782;
            } else {
                let v783 = v650 + v580;
                v784 = v783;
            }
            let v786 = v53 * (v543 + v784);
            let v787 = v784 - v543;
            let v788 = v306 - v786;
            let v792 = v542 * (v788.abs());
            let v799 = v538 / (((v1 + (v551 * v792)) + ((v554 * v792) * v792)) + v559);
            let v815 = (v1 + (((v810 * v810) * v787) * v787)).sqrt();
            let v817 = ((((((v799 * v219) * v283) * v802) / v282) * (v1 + (v805 * (v33 - v580)))) / v815) * ((v788 + v231) * v787);
            let v827 = v823 * (v1 + (v824 * v264));
            let v832 = v828 * (v1 + (v829 * v264));
            let v833 = if (v818 * (v1 + (v819 * v264))) > v0 { 1.0 } else { 0.0 };
            if v833 != 0.0 {
                let v834 = v40 - v832;
                let v835 = if v834 > v0 { 1.0 } else { 0.0 };
                if v835 != 0.0 {
                    let v839 = if (v834 / (v827 * v47)) > v838 { 1.0 } else { 0.0 };
                    if v839 != 0.0 {
                    } else {
                    }
                } else {
                    let v842 = if (v834 / (v827 * v47)) > v838 { 1.0 } else { 0.0 };
                    if v842 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v845 = v283 * v802;
            let v847 = (v306 + v231) - v786;
            let v856 = v788 + (((v53 * v787) * v787) / (v853 * v847));
            let v869 = (v217 / (v218 + (v865 / (v1 + ((v858 * (((((v219 * v283) * v802) * v282) * v856) / v859)).powf(v862)))))) * v283;
            let v877 = v787 * v787;
            let v892 = (-(((v869 * v282) * v802) * v53)) * (((v306 - ((v543 + (v93 * v784)) / v110)) + ((v876 * v877) / v847)) + ((v880 * (v877 * v787)) / (v847 * v847)));
            let v895 = (v893 * (((v869 * v802) * v282) * v856)) - v892;
            let v896 = if v844 < v0 { 1.0 } else { 0.0 };
            let v8768: f64;
            let v8770: f64;
            if v896 != 0.0 {
                v8768 = v895;
                v8770 = v892;
            } else {
                v8768 = v892;
                v8770 = v895;
            }
            let v898 = if v897 == v0 { 1.0 } else { 0.0 };
            let v1264: f64;
            let v1271: f64;
            if v898 != 0.0 {
                v1264 = v0;
                v1271 = v0;
            } else {
                let v899 = if v897 == v1 { 1.0 } else { 0.0 };
                let v1265: f64;
                let v1272: f64;
                if v899 != 0.0 {
                    let v909 = (v283 * v282) * v802;
                    let v914 = (v909 * ((v904 + (v264 * v905)).abs())) * ((rspice_limited_exp((v20 / ((v900 * v46) * v45)))) - v1);
                    let v927 = (v909 * ((v919 + (v264 * v920)).abs())) * ((rspice_limited_exp((v21 / ((v915 * v46) * v45)))) - v1);
                    v1265 = v914;
                    v1272 = v927;
                } else {
                    let v928 = if v897 == v93 { 1.0 } else { 0.0 };
                    let v1266: f64;
                    let v1273: f64;
                    if v928 != 0.0 {
                        let v948 = (v283 * v282) * v802;
                        let v954 = -v20;
                        let v961 = v954 - (v53 * (v954 - (((v954 * v954) + v956).sqrt())));
                        let v978 = ((v948 * ((v904 * ((v905 * v264).exp())).abs())) * ((rspice_limited_exp(((v20 - (v929 + (v264 * v930))) / (((v900 + (v264 * v933)) * v46) * v15)))) - v1)) * (v1 + (((v961 / v218) * (v969 * ((v970 * v264).exp()))) * (rspice_limited_exp((((v961.sqrt()) + v964) / (((v936 + (v264 * v937)) * v46) * v15))))));
                        let v1002 = -v21;
                        let v1008 = v1002 - (v53 * (v1002 - (((v1002 * v1002) + v956).sqrt())));
                        let v1025 = ((v948 * ((v919 * ((v920 * v264).exp())).abs())) * ((rspice_limited_exp(((v21 - (v979 + (v264 * v980))) / (((v915 + (v264 * v983)) * v46) * v15)))) - v1)) * (v1 + (((v1008 / v218) * (v1016 * ((v1017 * v264).exp()))) * (rspice_limited_exp((((v1008.sqrt()) + v1011) / (((v986 + (v264 * v987)) * v46) * v15))))));
                        v1266 = v978;
                        v1273 = v1025;
                    } else {
                        let v1026 = if v897 == v110 { 1.0 } else { 0.0 };
                        let v1267: f64;
                        let v1274: f64;
                        if v1026 != 0.0 {
                            let v1028 = v929 + (v264 * v930);
                            let v1030 = v900 + (v264 * v933);
                            let v1032 = v936 + (v264 * v937);
                            let v1035 = v969 * ((v970 * v264).exp());
                            let v1037 = (v283 * v282) * v802;
                            let v1041 = (v1037 * v904) * ((v905 * v264).exp());
                            let v1042 = if v1041 > v0 { 1.0 } else { 0.0 };
                            let v1268: f64;
                            if v1042 != 0.0 {
                                let v1043 = if v20 > v0 { 1.0 } else { 0.0 };
                                let v1050: f64;
                                if v1043 != 0.0 {
                                    let v1047 = (v20.powf(v1044)) / (v1030 * v47);
                                    v1050 = v1047;
                                } else {
                                    let v1049 = v20 / (v1030 * v47);
                                    v1050 = v1049;
                                }
                                let v1051 = if v1050 > v838 { 1.0 } else { 0.0 };
                                let v1054: f64;
                                let v1055: f64;
                                if v1051 != 0.0 {
                                    let v1053 = v1 + (v1050 - v838);
                                    v1054 = v1053;
                                    v1055 = v838;
                                } else {
                                    v1054 = v1;
                                    v1055 = v1050;
                                }
                                let v1064 = (v1041 * ((v1054 * (v1055.exp())) - v1)) * (((-v1028) / (v1030 * v47)).exp());
                                let v1065 = -v20;
                                let v1071 = v1065 - (v53 * (v1065 - (((v1065 * v1065) + v956).sqrt())));
                                let v1075 = ((v1071.sqrt()) + v964) / (v1032 * v47);
                                let v1076 = if v1075 > v838 { 1.0 } else { 0.0 };
                                let v1080: f64;
                                let v1082: f64;
                                if v1076 != 0.0 {
                                    let v1078 = v1 + (v1075 - v838);
                                    v1080 = v1078;
                                    v1082 = v838;
                                } else {
                                    v1080 = v1;
                                    v1082 = v1075;
                                }
                                let v1086 = v1064 * (v1 + (((v1071 * v1035) * v1080) * (v1082.exp())));
                                v1268 = v1086;
                            } else {
                                v1268 = v0;
                            }
                            let v1088 = v979 + (v264 * v980);
                            let v1090 = v915 + (v264 * v983);
                            let v1092 = v986 + (v264 * v987);
                            let v1095 = v1016 * ((v1017 * v264).exp());
                            let v1099 = (v1037 * v919) * ((v920 * v264).exp());
                            let v1100 = if v1099 > v0 { 1.0 } else { 0.0 };
                            let v1275: f64;
                            if v1100 != 0.0 {
                                let v1101 = if v21 > v0 { 1.0 } else { 0.0 };
                                let v1108: f64;
                                if v1101 != 0.0 {
                                    let v1105 = (v21.powf(v1102)) / (v1090 * v47);
                                    v1108 = v1105;
                                } else {
                                    let v1107 = v21 / (v1090 * v47);
                                    v1108 = v1107;
                                }
                                let v1109 = if v1108 > v838 { 1.0 } else { 0.0 };
                                let v1112: f64;
                                let v1113: f64;
                                if v1109 != 0.0 {
                                    let v1111 = v1 + (v1108 - v838);
                                    v1112 = v1111;
                                    v1113 = v838;
                                } else {
                                    v1112 = v1;
                                    v1113 = v1108;
                                }
                                let v1122 = (v1099 * ((v1112 * (v1113.exp())) - v1)) * (((-v1088) / (v1090 * v47)).exp());
                                let v1123 = -v21;
                                let v1129 = v1123 - (v53 * (v1123 - (((v1123 * v1123) + v956).sqrt())));
                                let v1133 = ((v1129.sqrt()) + v1011) / (v1092 * v47);
                                let v1134 = if v1133 > v838 { 1.0 } else { 0.0 };
                                let v1138: f64;
                                let v1140: f64;
                                if v1134 != 0.0 {
                                    let v1136 = v1 + (v1133 - v838);
                                    v1138 = v1136;
                                    v1140 = v838;
                                } else {
                                    v1138 = v1;
                                    v1140 = v1133;
                                }
                                let v1144 = v1122 * (v1 + (((v1129 * v1095) * v1138) * (v1140.exp())));
                                v1275 = v1144;
                            } else {
                                v1275 = v0;
                            }
                            v1267 = v1268;
                            v1274 = v1275;
                        } else {
                            let v1145 = if v897 == v119 { 1.0 } else { 0.0 };
                            let v1269: f64;
                            let v1276: f64;
                            if v1145 != 0.0 {
                                let v1147 = v929 + (v264 * v930);
                                let v1149 = v900 + (v264 * v933);
                                let v1151 = v936 + (v264 * v937);
                                let v1153 = (v283 * v282) * v802;
                                let v1157 = (v1153 * v969) * ((v970 * v264).exp());
                                let v1161 = (v1153 * v904) * ((v905 * v264).exp());
                                let v1162 = if v1161 > v0 { 1.0 } else { 0.0 };
                                let v1270: f64;
                                if v1162 != 0.0 {
                                    let v1163 = if v20 > v0 { 1.0 } else { 0.0 };
                                    let v1169: f64;
                                    if v1163 != 0.0 {
                                        let v1166 = (v20.powf(v1044)) / (v1149 * v47);
                                        v1169 = v1166;
                                    } else {
                                        let v1168 = v20 / (v1149 * v47);
                                        v1169 = v1168;
                                    }
                                    let v1170 = if v1169 > v838 { 1.0 } else { 0.0 };
                                    let v1173: f64;
                                    let v1174: f64;
                                    if v1170 != 0.0 {
                                        let v1172 = v1 + (v1169 - v838);
                                        v1173 = v1172;
                                        v1174 = v838;
                                    } else {
                                        v1173 = v1;
                                        v1174 = v1169;
                                    }
                                    let v1183 = (v1161 * ((v1173 * (v1174.exp())) - v1)) * (((-v1147) / (v1149 * v47)).exp());
                                    let v1184 = -v20;
                                    let v1192 = v1151 * v47;
                                    let v1193 = (((v1184 - (v53 * (v1184 - ((v1184 * v1184).sqrt())))).sqrt()) + v964) / v1192;
                                    let v1194 = if v1193 > v838 { 1.0 } else { 0.0 };
                                    let v1197: f64;
                                    let v1198: f64;
                                    if v1194 != 0.0 {
                                        let v1196 = v1 + (v1193 - v838);
                                        v1197 = v1196;
                                        v1198 = v838;
                                    } else {
                                        v1197 = v1;
                                        v1198 = v1193;
                                    }
                                    let v1205 = v1183 - (v1157 * ((v1197 * (v1198.exp())) - ((v964 / v1192).exp())));
                                    v1270 = v1205;
                                } else {
                                    v1270 = v0;
                                }
                                let v1207 = v979 + (v264 * v980);
                                let v1209 = v915 + (v264 * v983);
                                let v1211 = v986 + (v264 * v987);
                                let v1215 = (v1153 * v1016) * ((v1017 * v264).exp());
                                let v1219 = (v1153 * v919) * ((v920 * v264).exp());
                                let v1220 = if v1219 > v0 { 1.0 } else { 0.0 };
                                let v1277: f64;
                                if v1220 != 0.0 {
                                    let v1221 = if v21 > v0 { 1.0 } else { 0.0 };
                                    let v1227: f64;
                                    if v1221 != 0.0 {
                                        let v1224 = (v21.powf(v1102)) / (v1209 * v47);
                                        v1227 = v1224;
                                    } else {
                                        let v1226 = v21 / (v1209 * v47);
                                        v1227 = v1226;
                                    }
                                    let v1228 = if v1227 > v838 { 1.0 } else { 0.0 };
                                    let v1231: f64;
                                    let v1232: f64;
                                    if v1228 != 0.0 {
                                        let v1230 = v1 + (v1227 - v838);
                                        v1231 = v1230;
                                        v1232 = v838;
                                    } else {
                                        v1231 = v1;
                                        v1232 = v1227;
                                    }
                                    let v1241 = (v1219 * ((v1231 * (v1232.exp())) - v1)) * (((-v1207) / (v1209 * v47)).exp());
                                    let v1242 = -v21;
                                    let v1250 = v1211 * v47;
                                    let v1251 = (((v1242 - (v53 * (v1242 - ((v1242 * v1242).sqrt())))).sqrt()) + v1011) / v1250;
                                    let v1252 = if v1251 > v838 { 1.0 } else { 0.0 };
                                    let v1255: f64;
                                    let v1256: f64;
                                    if v1252 != 0.0 {
                                        let v1254 = v1 + (v1251 - v838);
                                        v1255 = v1254;
                                        v1256 = v838;
                                    } else {
                                        v1255 = v1;
                                        v1256 = v1251;
                                    }
                                    let v1263 = v1241 - (v1215 * ((v1255 * (v1256.exp())) - ((v1011 / v1250).exp())));
                                    v1277 = v1263;
                                } else {
                                    v1277 = v0;
                                }
                                v1269 = v1270;
                                v1276 = v1277;
                            } else {
                                v1269 = v0;
                                v1276 = v0;
                            }
                            v1267 = v1269;
                            v1274 = v1276;
                        }
                        v1266 = v1267;
                        v1273 = v1274;
                    }
                    v1265 = v1266;
                    v1272 = v1273;
                }
                v1264 = v1265;
                v1271 = v1272;
            }
            if v898 != 0.0 {
            } else {
            }
            let v1282 = if v1280 == v1 { 1.0 } else { 0.0 };
            let v1518: f64;
            let v1521: f64;
            if v1282 != 0.0 {
                let v1285 = v1 - (v1283 * v264);
                let v1304 = ((v1301 / v285) * v279) * v221;
                let v1305 = (((v174 * v1285) - v1287) - v1294) + v1304;
                let v1306 = v1 + v1305;
                let v1307 = v1305 - v1;
                let v1322 = v1319 * (v263.powf(v1320));
                let v1323 = v845 * ((v285 * (v1306 - (v53 * (v1306 - (((v1307 * v1307) + v956).sqrt()))))) * (v1 + (v1315 * v588)));
                let v1324 = v1323 * v1322;
                let v1331 = v1329 / (v1323 * (v1325 * (v263.powf(v1326))));
                let v1332 = if v1278 != v0 { 1.0 } else { 0.0 };
                let v1368: f64;
                if v1332 != 0.0 {
                    let v1334 = v1 + v1333;
                    let v1336 = (v1334.sqrt()) * v817;
                    let v1337 = v1336 / v1324;
                    let v1338 = v1337 * v93;
                    let v1340 = v1334 + (v1337 * v1337);
                    let v1349 = v1 - (((v1336 * v93) / (((v1340 - v1338).sqrt()) + ((v1340 + v1338).sqrt()))) / v1324);
                    v1368 = v1349;
                } else {
                    let v1351 = (v817 / v1324).abs();
                    let v1354 = v1351 - v1352;
                    let v1367 = (v1 - ((v53 * (((v1351 + v1352) - (((v1354 * v1354) + v1356).sqrt())) - v1360)).powf(v1363))).powf((v1 / v1363));
                    v1368 = v1367;
                }
                let v1379 = (((v1370 * (v1 + (v1371 * v264))) / v845) + (v1331 / v1368)) + v1377;
                let v1395 = (((v179 * v1285) - v1381) - v1388) + v1304;
                let v1396 = v1 + v1395;
                let v1397 = v1395 - v1;
                let v1409 = v845 * ((v285 * (v1396 - (v53 * (v1396 - (((v1397 * v1397) + v956).sqrt()))))) * (v1 + (v1405 * v588)));
                let v1410 = v1409 * v1322;
                let v1417 = v1415 / (v1409 * (v1411 * (v263.powf(v1412))));
                let v1418 = if v1279 != v0 { 1.0 } else { 0.0 };
                let v1453: f64;
                if v1418 != 0.0 {
                    let v1420 = v1 + v1419;
                    let v1422 = (v1420.sqrt()) * v817;
                    let v1423 = v1422 / v1410;
                    let v1424 = v1423 * v93;
                    let v1426 = v1420 + (v1423 * v1423);
                    let v1435 = v1 - (((v1422 * v93) / (((v1426 - v1424).sqrt()) + ((v1426 + v1424).sqrt()))) / v1410);
                    v1453 = v1435;
                } else {
                    let v1437 = (v817 / v1410).abs();
                    let v1439 = v1437 - v1352;
                    let v1452 = (v1 - ((v53 * (((v1437 + v1352) - (((v1439 * v1439) + v1441).sqrt())) - v1445)).powf(v1448))).powf((v1 / v1448));
                    v1453 = v1452;
                }
                let v1474 = v1 / ((((((v1455 * (v1 + (v1456 * v264))) / v845) + (v1417 / v1453)) + v1462) + v1467) + v1471);
                let v1475 = v1 / v1379;
                let v1476 = if v10 == v0 { 1.0 } else { 0.0 };
                if v1476 != 0.0 {
                } else {
                }
                v1518 = v1474;
                v1521 = v1475;
            } else {
                let v1479 = if v10 == v0 { 1.0 } else { 0.0 };
                if v1479 != 0.0 {
                } else {
                }
                v1518 = v0;
                v1521 = v0;
            }
            let v1481 = if v1480 == v1 { 1.0 } else { 0.0 };
            let v8937: f64;
            let v8938: f64;
            let v8939: f64;
            let v8942: f64;
            let v8945: f64;
            let v8948: f64;
            let v8951: f64;
            let v8954: f64;
            let v8957: f64;
            let v8960: f64;
            if v1481 != 0.0 {
                let v1499 = v799 / v815;
                let v1503 = v784 * v784;
                let v1505 = v543 * v543;
                let v1514 = ((((v1482 / (((if v817 >= v1483 { v817 } else { v1483 }) * v282) * v282)) * (((((((((v1488 * v45) * v285) * v283) * v802) * v219) * v285) * v283) * v802) * v219)) * (v1499 * v1499)) * (((v315 * v787) + (((v1503 * v784) - (v1505 * v543)) / v110)) - (v306 * (v1503 - v1505)))) * v843;
                let v8940: f64;
                let v8943: f64;
                let v8946: f64;
                let v8949: f64;
                let v8952: f64;
                let v8955: f64;
                let v8958: f64;
                let v8961: f64;
                if v1282 != 0.0 {
                    let v1515 = if v10 == v0 { 1.0 } else { 0.0 };
                    let v8941: f64;
                    let v8944: f64;
                    let v8947: f64;
                    let v8950: f64;
                    let v8953: f64;
                    let v8956: f64;
                    let v8959: f64;
                    let v8962: f64;
                    if v1515 != 0.0 {
                        let v1517 = (v119 * v47) * v285;
                        let v1520 = (v1517 * v1518) * v843;
                        let v1523 = (v1517 * v1521) * v843;
                        v8941 = v1;
                        v8944 = v1520;
                        v8947 = v1;
                        v8950 = v1523;
                        v8953 = v0;
                        v8956 = v0;
                        v8959 = v0;
                        v8962 = v0;
                    } else {
                        let v1525 = (v119 * v47) * v285;
                        let v1527 = (v1525 * v1518) * v843;
                        let v1529 = (v1525 * v1521) * v843;
                        v8941 = v0;
                        v8944 = v0;
                        v8947 = v0;
                        v8950 = v0;
                        v8953 = v1;
                        v8956 = v1527;
                        v8959 = v1;
                        v8962 = v1529;
                    }
                    v8940 = v8941;
                    v8943 = v8944;
                    v8946 = v8947;
                    v8949 = v8950;
                    v8952 = v8953;
                    v8955 = v8956;
                    v8958 = v8959;
                    v8961 = v8962;
                } else {
                    v8940 = v0;
                    v8943 = v0;
                    v8946 = v0;
                    v8949 = v0;
                    v8952 = v0;
                    v8955 = v0;
                    v8958 = v0;
                    v8961 = v0;
                }
                v8937 = v1;
                v8938 = v1514;
                v8939 = v8940;
                v8942 = v8943;
                v8945 = v8946;
                v8948 = v8949;
                v8951 = v8952;
                v8954 = v8955;
                v8957 = v8958;
                v8960 = v8961;
            } else {
                v8937 = v0;
                v8938 = v0;
                v8939 = v0;
                v8942 = v0;
                v8945 = v0;
                v8948 = v0;
                v8951 = v0;
                v8954 = v0;
                v8957 = v0;
                v8960 = v0;
            }
            let v1530 = if v897 != v0 { 1.0 } else { 0.0 };
            let v8963: f64;
            let v8964: f64;
            let v8965: f64;
            let v8966: f64;
            if v1530 != 0.0 {
                let v1534 = (v1531 * (v1264.abs())) * v843;
                let v1538 = (v1535 * (v1271.abs())) * v843;
                v8963 = v1;
                v8964 = v1534;
                v8965 = v1;
                v8966 = v1538;
            } else {
                v8963 = v0;
                v8964 = v0;
                v8965 = v0;
                v8966 = v0;
            }
            let v1539 = if v10 == v0 { 1.0 } else { 0.0 };
            let v8838: f64;
            if v1539 != 0.0 {
                let v1541 = if v1540 != v0 { 1.0 } else { 0.0 };
                let v8839: f64;
                if v1541 != 0.0 {
                    let v1543 = v1542 - v16;
                    let v1544 = if v1540 == v1 { 1.0 } else { 0.0 };
                    let v1551: f64;
                    let v1552: f64;
                    if v1544 != 0.0 {
                        let v1545 = v19 - v1542;
                        v1551 = v1545;
                        v1552 = v21;
                    } else {
                        let v1546 = v35 - v16;
                        let v1547 = v35 - v1542;
                        v1551 = v1547;
                        v1552 = v1546;
                    }
                    let v1548 = if v1543 < v0 { 1.0 } else { 0.0 };
                    let v1553: f64;
                    let v1589: f64;
                    let v2008: f64;
                    if v1548 != 0.0 {
                        let v1550 = v1549 * v1543;
                        v1553 = v1550;
                        v1589 = v1551;
                        v2008 = v1549;
                    } else {
                        v1553 = v1543;
                        v1589 = v1552;
                        v2008 = v1;
                    }
                    let v1557 = (((v1553 * v1553) + v2).sqrt()) - v32;
                    let v1563 = v47 * ((v1 + v1558) + (v1560 * v1557));
                    let v1577 = (v1564 + (v264 * v1565)) - ((v1568 * (v1557 * v1569)) / (((v1557 * v1557) + (v1569 * v1569)).sqrt()));
                    let v1579 = v217 / v1578;
                    let v1588 = v1577 + (v1563 * (((v1580 / ((v288 * v1563) * v1563)) * v1584).ln()));
                    let v1590 = v1589 - v1588;
                    let v1597 = ((v53 * (v1590 + (((v1590 * v1590) + v300).sqrt()))) + v1588) - v1577;
                    let v1600 = v1579 / (v1598 * v1563);
                    let v1601 = v310 / v1600;
                    let v1602 = v1 / v1600;
                    let v1603 = v1579 / v285;
                    let v1604 = v53 * v1597;
                    let v1605 = v1597 * v1597;
                    let v1610 = v1604 + (v53 * ((v1605 + v1606).sqrt()));
                    let v1612 = v1610 * v1610;
                    let v1613 = v1601 * v1601;
                    let v1618 = v1602 * v1602;
                    let v1628 = v1627 / v110;
                    let v1630 = (v1603 * v1610).powf(v340);
                    let v1637 = (v93 * v1627) / v110;
                    let v1640 = ((v1610 + (v1563 * (v1 - ((v1600 * ((v1610 * v1601) / ((v1612 + v1613).sqrt()))).ln())))) - (v1628 * v1630)) / ((v1610 * (v1 + (v1563 / ((v1610 * v1602) / ((v1612 + v1618).sqrt()))))) + (v1637 * v1630));
                    let v1641 = v93 * v1563;
                    let v1642 = v1597 / v1641;
                    let v1643 = if v1642 < v354 { 1.0 } else { 0.0 };
                    let v1677: f64;
                    if v1643 != 0.0 {
                        let v1665 = ((v1641 * v1603) * (((v110 * v1642) / v119) + (((rspice_limited_exp((v1642 / v119))) + (rspice_limited_exp(((v1646 * v1642) / v119)))).ln()))) / ((v1 / v1640) + ((v1603 / v287) * (rspice_limited_exp(((v1659 * v1597) / v1641)))));
                        v1677 = v1665;
                    } else {
                        let v1676 = ((v1641 * v1603) * v1642) / ((v1 / v1640) + ((v1603 / v287) * (rspice_limited_exp(((v1670 * v1597) / v1641)))));
                        v1677 = v1676;
                    }
                    let v1679 = v1597 - (v1677 / v1603);
                    let v1682 = if ((v1679 - v1597).abs()) > v394 { 1.0 } else { 0.0 };
                    let v1815: f64;
                    if v1682 != 0.0 {
                        let v1683 = v1597 - v1679;
                        let v1690 = (v53 * v1683) + (v53 * (((v1683 * v1683) + v1686).sqrt()));
                        let v1691 = v1603.powf(v340);
                        let v1692 = v1690.powf(v340);
                        let v1694 = v1690.powf(v1693);
                        let v1695 = v1627 * v1691;
                        let v1698 = v1697 * v1691;
                        let v1700 = v1679 / v1563;
                        let v1702 = v1700 - ((v1695 * v1692) / v1563);
                        let v1704 = v1700 - ((v1698 * v1692) / v1563);
                        let v1705 = if v1702 >= v418 { 1.0 } else { 0.0 };
                        let v1719: f64;
                        if v1705 != 0.0 {
                            v1719 = v1702;
                        } else {
                            let v1707 = if v1702 <= v1706 { 1.0 } else { 0.0 };
                            let v1720: f64;
                            if v1707 != 0.0 {
                                v1720 = v0;
                            } else {
                                let v1710 = ((v1702.exp()) + v1).ln();
                                v1720 = v1710;
                            }
                            v1719 = v1720;
                        }
                        let v1711 = if v1704 >= v418 { 1.0 } else { 0.0 };
                        let v1723: f64;
                        if v1711 != 0.0 {
                            v1723 = v1704;
                        } else {
                            let v1713 = if v1704 <= v1712 { 1.0 } else { 0.0 };
                            let v1724: f64;
                            if v1713 != 0.0 {
                                v1724 = v0;
                            } else {
                                let v1716 = ((v1704.exp()) + v1).ln();
                                v1724 = v1716;
                            }
                            v1723 = v1724;
                        }
                        let v1718 = v287 * v1563;
                        let v1729 = rspice_limited_exp(v1702);
                        let v1735 = rspice_limited_exp(v1704);
                        let v1748 = v1679 - ((((v1603 * v1690) - (v1718 * v1719)) - (v1718 * v1723)) / (((v1741 * v1603) - (((v1729 * v287) * (v1 + (v340 * (v1695 * v1694)))) / (v1 + v1729))) - (((v1735 * v287) * (v1 + (v340 * (v1698 * v1694)))) / (v1 + v1735))));
                        let v1749 = v1597 - v1748;
                        let v1756 = (v53 * v1749) + (v53 * (((v1749 * v1749) + v1752).sqrt()));
                        let v1758 = v1756.powf(v1757);
                        let v1759 = v1756.powf(v340);
                        let v1762 = v1748 / v1563;
                        let v1764 = v1762 - ((v1695 * v1759) / v1563);
                        let v1766 = v1762 - ((v1698 * v1759) / v1563);
                        let v1767 = if v1764 >= v418 { 1.0 } else { 0.0 };
                        let v1780: f64;
                        if v1767 != 0.0 {
                            v1780 = v1764;
                        } else {
                            let v1769 = if v1764 <= v1768 { 1.0 } else { 0.0 };
                            let v1781: f64;
                            if v1769 != 0.0 {
                                v1781 = v0;
                            } else {
                                let v1772 = ((v1764.exp()) + v1).ln();
                                v1781 = v1772;
                            }
                            v1780 = v1781;
                        }
                        let v1773 = if v1766 >= v418 { 1.0 } else { 0.0 };
                        let v1784: f64;
                        if v1773 != 0.0 {
                            v1784 = v1766;
                        } else {
                            let v1775 = if v1766 <= v1774 { 1.0 } else { 0.0 };
                            let v1785: f64;
                            if v1775 != 0.0 {
                                v1785 = v0;
                            } else {
                                let v1778 = ((v1766.exp()) + v1).ln();
                                v1785 = v1778;
                            }
                            v1784 = v1785;
                        }
                        let v1790 = rspice_limited_exp(v1764);
                        let v1796 = rspice_limited_exp(v1766);
                        let v1809 = v1748 - ((((v1603 * v1756) - (v1718 * v1780)) - (v1718 * v1784)) / (((v1802 * v1603) - (((v1790 * v287) * (v1 + (v340 * (v1695 * v1758)))) / (v1 + v1790))) - (((v1796 * v287) * (v1 + (v340 * (v1698 * v1758)))) / (v1 + v1796))));
                        v1815 = v1809;
                    } else {
                        v1815 = v1679;
                    }
                    let v1818 = (v1579 / v217) * ((v1597 - v1815).abs());
                    let v1836 = v1604 + (v53 * ((v1605 + v1832).sqrt()));
                    let v1837 = ((v93 * (v1812 * v540)) / ((v1810 * v537) / (((v1 + (v551 * v1818)) + (v554 * (v1818 * v1818))) + (v558 * (v547 * ((v279 - v1815).abs())))))) * v1580;
                    let v1848 = v1597 - (v1553 * ((v1 + ((v1553 / ((v1837 * v1836) / (v1837 + v1836))).powf(v574))).powf((v1844 / v574))));
                    let v1855 = (v53 * v1848) + (v53 * (((v1848 * v1848) + v1851).sqrt()));
                    let v1857 = v1855 * v1855;
                    let v1871 = (v1603 * v1855).powf(v340);
                    let v1879 = ((v1855 + (v1563 * (v1 - ((v1600 * ((v1855 * v1601) / ((v1857 + v1613).sqrt()))).ln())))) - (v1628 * v1871)) / ((v1855 * (v1 + (v1563 / ((v1855 * v1602) / ((v1857 + v1618).sqrt()))))) + (v1637 * v1871));
                    let v1880 = v1848 / v1641;
                    let v1881 = if v1880 < v354 { 1.0 } else { 0.0 };
                    let v1915: f64;
                    if v1881 != 0.0 {
                        let v1903 = ((v1641 * v1603) * (((v110 * v1880) / v119) + (((rspice_limited_exp((v1880 / v119))) + (rspice_limited_exp(((v1884 * v1880) / v119)))).ln()))) / ((v1 / v1879) + ((v1603 / v287) * (rspice_limited_exp(((v1897 * v1848) / v1641)))));
                        v1915 = v1903;
                    } else {
                        let v1914 = ((v1641 * v1603) * v1880) / ((v1 / v1879) + ((v1603 / v287) * (rspice_limited_exp(((v1908 * v1848) / v1641)))));
                        v1915 = v1914;
                    }
                    let v1917 = v1848 - (v1915 / v1603);
                    let v1920 = if ((v1917 - v1848).abs()) > v394 { 1.0 } else { 0.0 };
                    if v1920 != 0.0 {
                        let v1921 = v1848 - v1917;
                        let v1928 = (v53 * v1921) + (v53 * (((v1921 * v1921) + v1924).sqrt()));
                        let v1929 = v1603.powf(v340);
                        let v1930 = v1928.powf(v340);
                        let v1932 = v1928.powf(v1931);
                        let v1933 = v1627 * v1929;
                        let v1935 = v1697 * v1929;
                        let v1937 = v1917 / v1563;
                        let v1939 = v1937 - ((v1933 * v1930) / v1563);
                        let v1941 = v1937 - ((v1935 * v1930) / v1563);
                        let v1942 = if v1939 >= v418 { 1.0 } else { 0.0 };
                        let v1956: f64;
                        if v1942 != 0.0 {
                            v1956 = v1939;
                        } else {
                            let v1944 = if v1939 <= v1943 { 1.0 } else { 0.0 };
                            let v1957: f64;
                            if v1944 != 0.0 {
                                v1957 = v0;
                            } else {
                                let v1947 = ((v1939.exp()) + v1).ln();
                                v1957 = v1947;
                            }
                            v1956 = v1957;
                        }
                        let v1948 = if v1941 >= v418 { 1.0 } else { 0.0 };
                        let v1960: f64;
                        if v1948 != 0.0 {
                            v1960 = v1941;
                        } else {
                            let v1950 = if v1941 <= v1949 { 1.0 } else { 0.0 };
                            let v1961: f64;
                            if v1950 != 0.0 {
                                v1961 = v0;
                            } else {
                                let v1953 = ((v1941.exp()) + v1).ln();
                                v1961 = v1953;
                            }
                            v1960 = v1961;
                        }
                        let v1955 = v287 * v1563;
                        let v1966 = rspice_limited_exp(v1939);
                        let v1972 = rspice_limited_exp(v1941);
                        let v1985 = v1917 - ((((v1603 * v1928) - (v1955 * v1956)) - (v1955 * v1960)) / (((v1978 * v1603) - (((v1966 * v287) * (v1 + (v340 * (v1933 * v1932)))) / (v1 + v1966))) - (((v1972 * v287) * (v1 + (v340 * (v1935 * v1932)))) / (v1 + v1972))));
                        let v1986 = v1848 - v1985;
                        let v1994 = ((v53 * v1986) + (v53 * (((v1986 * v1986) + v1989).sqrt()))).powf(v340);
                        let v1997 = v1985 / v1563;
                        let v1999 = v1997 - ((v1933 * v1994) / v1563);
                        let v2001 = v1997 - ((v1935 * v1994) / v1563);
                        let v2002 = if v1999 >= v418 { 1.0 } else { 0.0 };
                        if v2002 != 0.0 {
                        } else {
                            let v2004 = if v1999 <= v2003 { 1.0 } else { 0.0 };
                            if v2004 != 0.0 {
                            } else {
                            }
                        }
                        let v2005 = if v2001 >= v418 { 1.0 } else { 0.0 };
                        if v2005 != 0.0 {
                        } else {
                            let v2007 = if v2001 <= v2006 { 1.0 } else { 0.0 };
                            if v2007 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v2009 = if v2008 < v0 { 1.0 } else { 0.0 };
                    if v2009 != 0.0 {
                    } else {
                    }
                    v8839 = v1563;
                } else {
                    v8839 = v231;
                }
                v8838 = v8839;
            } else {
                let v2010 = if v1540 != v0 { 1.0 } else { 0.0 };
                let v8840: f64;
                if v2010 != 0.0 {
                    let v2011 = if v1540 == v1 { 1.0 } else { 0.0 };
                    let v2013: f64;
                    if v2011 != 0.0 {
                        v2013 = v21;
                    } else {
                        let v2012 = v35 - v16;
                        v2013 = v2012;
                    }
                    let v2015 = v47 * (v1 + v1558);
                    let v2017 = v1564 + (v264 * v1565);
                    let v2018 = v217 / v1578;
                    let v2025 = v2017 + (v2015 * (((v1580 / ((v288 * v2015) * v2015)) * v1584).ln()));
                    let v2026 = v2013 - v2025;
                    let v2033 = ((v53 * (v2026 + (((v2026 * v2026) + v300).sqrt()))) + v2025) - v2017;
                    let v2036 = v2018 / (v2034 * v2015);
                    let v2037 = v310 / v2036;
                    let v2038 = v1 / v2036;
                    let v2039 = v2018 / v285;
                    let v2040 = v53 * v2033;
                    let v2041 = v2033 * v2033;
                    let v2046 = v2040 + (v53 * ((v2041 + v2042).sqrt()));
                    let v2048 = v2046 * v2046;
                    let v2049 = v2037 * v2037;
                    let v2054 = v2038 * v2038;
                    let v2063 = v1627 / v110;
                    let v2065 = (v2039 * v2046).powf(v340);
                    let v2072 = (v93 * v1627) / v110;
                    let v2075 = ((v2046 + (v2015 * (v1 - ((v2036 * ((v2046 * v2037) / ((v2048 + v2049).sqrt()))).ln())))) - (v2063 * v2065)) / ((v2046 * (v1 + (v2015 / ((v2046 * v2038) / ((v2048 + v2054).sqrt()))))) + (v2072 * v2065));
                    let v2076 = v93 * v2015;
                    let v2077 = v2033 / v2076;
                    let v2078 = if v2077 < v354 { 1.0 } else { 0.0 };
                    let v2112: f64;
                    if v2078 != 0.0 {
                        let v2100 = ((v2076 * v2039) * (((v110 * v2077) / v119) + (((rspice_limited_exp((v2077 / v119))) + (rspice_limited_exp(((v2081 * v2077) / v119)))).ln()))) / ((v1 / v2075) + ((v2039 / v287) * (rspice_limited_exp(((v2094 * v2033) / v2076)))));
                        v2112 = v2100;
                    } else {
                        let v2111 = ((v2076 * v2039) * v2077) / ((v1 / v2075) + ((v2039 / v287) * (rspice_limited_exp(((v2105 * v2033) / v2076)))));
                        v2112 = v2111;
                    }
                    let v2114 = v2033 - (v2112 / v2039);
                    let v2117 = if ((v2114 - v2033).abs()) > v394 { 1.0 } else { 0.0 };
                    let v2247: f64;
                    if v2117 != 0.0 {
                        let v2118 = v2033 - v2114;
                        let v2125 = (v53 * v2118) + (v53 * (((v2118 * v2118) + v2121).sqrt()));
                        let v2126 = v2039.powf(v340);
                        let v2127 = v2125.powf(v340);
                        let v2129 = v2125.powf(v2128);
                        let v2130 = v1627 * v2126;
                        let v2132 = v1697 * v2126;
                        let v2134 = v2114 / v2015;
                        let v2136 = v2134 - ((v2130 * v2127) / v2015);
                        let v2138 = v2134 - ((v2132 * v2127) / v2015);
                        let v2139 = if v2136 >= v418 { 1.0 } else { 0.0 };
                        let v2153: f64;
                        if v2139 != 0.0 {
                            v2153 = v2136;
                        } else {
                            let v2141 = if v2136 <= v2140 { 1.0 } else { 0.0 };
                            let v2154: f64;
                            if v2141 != 0.0 {
                                v2154 = v0;
                            } else {
                                let v2144 = ((v2136.exp()) + v1).ln();
                                v2154 = v2144;
                            }
                            v2153 = v2154;
                        }
                        let v2145 = if v2138 >= v418 { 1.0 } else { 0.0 };
                        let v2157: f64;
                        if v2145 != 0.0 {
                            v2157 = v2138;
                        } else {
                            let v2147 = if v2138 <= v2146 { 1.0 } else { 0.0 };
                            let v2158: f64;
                            if v2147 != 0.0 {
                                v2158 = v0;
                            } else {
                                let v2150 = ((v2138.exp()) + v1).ln();
                                v2158 = v2150;
                            }
                            v2157 = v2158;
                        }
                        let v2152 = v287 * v2015;
                        let v2163 = rspice_limited_exp(v2136);
                        let v2169 = rspice_limited_exp(v2138);
                        let v2182 = v2114 - ((((v2039 * v2125) - (v2152 * v2153)) - (v2152 * v2157)) / (((v2175 * v2039) - (((v2163 * v287) * (v1 + (v340 * (v2130 * v2129)))) / (v1 + v2163))) - (((v2169 * v287) * (v1 + (v340 * (v2132 * v2129)))) / (v1 + v2169))));
                        let v2183 = v2033 - v2182;
                        let v2190 = (v53 * v2183) + (v53 * (((v2183 * v2183) + v2186).sqrt()));
                        let v2192 = v2190.powf(v2191);
                        let v2193 = v2190.powf(v340);
                        let v2196 = v2182 / v2015;
                        let v2198 = v2196 - ((v2130 * v2193) / v2015);
                        let v2200 = v2196 - ((v2132 * v2193) / v2015);
                        let v2201 = if v2198 >= v418 { 1.0 } else { 0.0 };
                        let v2214: f64;
                        if v2201 != 0.0 {
                            v2214 = v2198;
                        } else {
                            let v2203 = if v2198 <= v2202 { 1.0 } else { 0.0 };
                            let v2215: f64;
                            if v2203 != 0.0 {
                                v2215 = v0;
                            } else {
                                let v2206 = ((v2198.exp()) + v1).ln();
                                v2215 = v2206;
                            }
                            v2214 = v2215;
                        }
                        let v2207 = if v2200 >= v418 { 1.0 } else { 0.0 };
                        let v2218: f64;
                        if v2207 != 0.0 {
                            v2218 = v2200;
                        } else {
                            let v2209 = if v2200 <= v2208 { 1.0 } else { 0.0 };
                            let v2219: f64;
                            if v2209 != 0.0 {
                                v2219 = v0;
                            } else {
                                let v2212 = ((v2200.exp()) + v1).ln();
                                v2219 = v2212;
                            }
                            v2218 = v2219;
                        }
                        let v2224 = rspice_limited_exp(v2198);
                        let v2230 = rspice_limited_exp(v2200);
                        let v2243 = v2182 - ((((v2039 * v2190) - (v2152 * v2214)) - (v2152 * v2218)) / (((v2236 * v2039) - (((v2224 * v287) * (v1 + (v340 * (v2130 * v2192)))) / (v1 + v2224))) - (((v2230 * v287) * (v1 + (v340 * (v2132 * v2192)))) / (v1 + v2230))));
                        v2247 = v2243;
                    } else {
                        v2247 = v2114;
                    }
                    let v2250 = (v2018 / v217) * ((v2033 - v2247).abs());
                    let v2268 = v2040 + (v53 * ((v2041 + v2264).sqrt()));
                    let v2269 = ((v93 * (v1812 * v540)) / ((v1810 * v537) / (((v1 + (v551 * v2250)) + (v554 * (v2250 * v2250))) + (v558 * (v547 * ((v279 - v2247).abs())))))) * v1580;
                    let v2280 = v2033 - (v0 * ((v1 + ((v0 / ((v2269 * v2268) / (v2269 + v2268))).powf(v574))).powf((v2276 / v574))));
                    let v2287 = (v53 * v2280) + (v53 * (((v2280 * v2280) + v2283).sqrt()));
                    let v2289 = v2287 * v2287;
                    let v2303 = (v2039 * v2287).powf(v340);
                    let v2311 = ((v2287 + (v2015 * (v1 - ((v2036 * ((v2287 * v2037) / ((v2289 + v2049).sqrt()))).ln())))) - (v2063 * v2303)) / ((v2287 * (v1 + (v2015 / ((v2287 * v2038) / ((v2289 + v2054).sqrt()))))) + (v2072 * v2303));
                    let v2312 = v2280 / v2076;
                    let v2313 = if v2312 < v354 { 1.0 } else { 0.0 };
                    let v2347: f64;
                    if v2313 != 0.0 {
                        let v2335 = ((v2076 * v2039) * (((v110 * v2312) / v119) + (((rspice_limited_exp((v2312 / v119))) + (rspice_limited_exp(((v2316 * v2312) / v119)))).ln()))) / ((v1 / v2311) + ((v2039 / v287) * (rspice_limited_exp(((v2329 * v2280) / v2076)))));
                        v2347 = v2335;
                    } else {
                        let v2346 = ((v2076 * v2039) * v2312) / ((v1 / v2311) + ((v2039 / v287) * (rspice_limited_exp(((v2340 * v2280) / v2076)))));
                        v2347 = v2346;
                    }
                    let v2349 = v2280 - (v2347 / v2039);
                    let v2352 = if ((v2349 - v2280).abs()) > v394 { 1.0 } else { 0.0 };
                    if v2352 != 0.0 {
                        let v2353 = v2280 - v2349;
                        let v2360 = (v53 * v2353) + (v53 * (((v2353 * v2353) + v2356).sqrt()));
                        let v2361 = v2039.powf(v340);
                        let v2362 = v2360.powf(v340);
                        let v2364 = v2360.powf(v2363);
                        let v2365 = v1627 * v2361;
                        let v2367 = v1697 * v2361;
                        let v2369 = v2349 / v2015;
                        let v2371 = v2369 - ((v2365 * v2362) / v2015);
                        let v2373 = v2369 - ((v2367 * v2362) / v2015);
                        let v2374 = if v2371 >= v418 { 1.0 } else { 0.0 };
                        let v2388: f64;
                        if v2374 != 0.0 {
                            v2388 = v2371;
                        } else {
                            let v2376 = if v2371 <= v2375 { 1.0 } else { 0.0 };
                            let v2389: f64;
                            if v2376 != 0.0 {
                                v2389 = v0;
                            } else {
                                let v2379 = ((v2371.exp()) + v1).ln();
                                v2389 = v2379;
                            }
                            v2388 = v2389;
                        }
                        let v2380 = if v2373 >= v418 { 1.0 } else { 0.0 };
                        let v2392: f64;
                        if v2380 != 0.0 {
                            v2392 = v2373;
                        } else {
                            let v2382 = if v2373 <= v2381 { 1.0 } else { 0.0 };
                            let v2393: f64;
                            if v2382 != 0.0 {
                                v2393 = v0;
                            } else {
                                let v2385 = ((v2373.exp()) + v1).ln();
                                v2393 = v2385;
                            }
                            v2392 = v2393;
                        }
                        let v2387 = v287 * v2015;
                        let v2398 = rspice_limited_exp(v2371);
                        let v2404 = rspice_limited_exp(v2373);
                        let v2417 = v2349 - ((((v2039 * v2360) - (v2387 * v2388)) - (v2387 * v2392)) / (((v2410 * v2039) - (((v2398 * v287) * (v1 + (v340 * (v2365 * v2364)))) / (v1 + v2398))) - (((v2404 * v287) * (v1 + (v340 * (v2367 * v2364)))) / (v1 + v2404))));
                        let v2418 = v2280 - v2417;
                        let v2426 = ((v53 * v2418) + (v53 * (((v2418 * v2418) + v2421).sqrt()))).powf(v340);
                        let v2429 = v2417 / v2015;
                        let v2431 = v2429 - ((v2365 * v2426) / v2015);
                        let v2433 = v2429 - ((v2367 * v2426) / v2015);
                        let v2434 = if v2431 >= v418 { 1.0 } else { 0.0 };
                        if v2434 != 0.0 {
                        } else {
                            let v2436 = if v2431 <= v2435 { 1.0 } else { 0.0 };
                            if v2436 != 0.0 {
                            } else {
                            }
                        }
                        let v2437 = if v2433 >= v418 { 1.0 } else { 0.0 };
                        if v2437 != 0.0 {
                        } else {
                            let v2439 = if v2433 <= v2438 { 1.0 } else { 0.0 };
                            if v2439 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    v8840 = v2015;
                } else {
                    v8840 = v231;
                }
                v8838 = v8840;
            }
            let v8836: f64;
            if v1539 != 0.0 {
                let v2441 = if v2440 != v0 { 1.0 } else { 0.0 };
                let v8837: f64;
                if v2441 != 0.0 {
                    let v2443 = v17 - v2442;
                    let v2444 = if v2440 == v1 { 1.0 } else { 0.0 };
                    let v2451: f64;
                    let v2452: f64;
                    if v2444 != 0.0 {
                        let v2445 = v19 - v2442;
                        v2451 = v20;
                        v2452 = v2445;
                    } else {
                        let v2446 = v35 - v2442;
                        let v2447 = v35 - v17;
                        v2451 = v2447;
                        v2452 = v2446;
                    }
                    let v2448 = if v2443 < v0 { 1.0 } else { 0.0 };
                    let v2453: f64;
                    let v2480: f64;
                    let v2895: f64;
                    if v2448 != 0.0 {
                        let v2450 = v2449 * v2443;
                        v2453 = v2450;
                        v2480 = v2451;
                        v2895 = v2449;
                    } else {
                        v2453 = v2443;
                        v2480 = v2452;
                        v2895 = v1;
                    }
                    let v2457 = (((v2453 * v2453) + v2).sqrt()) - v32;
                    let v2461 = v47 * ((v1 + v1558) + (v1560 * v2457));
                    let v2471 = (v1564 + (v264 * v1565)) - ((v1568 * (v2457 * v1569)) / (((v2457 * v2457) + (v1569 * v1569)).sqrt()));
                    let v2472 = v217 / v1578;
                    let v2479 = v2471 + (v2461 * (((v1580 / ((v288 * v2461) * v2461)) * v1584).ln()));
                    let v2481 = v2480 - v2479;
                    let v2488 = ((v53 * (v2481 + (((v2481 * v2481) + v300).sqrt()))) + v2479) - v2471;
                    let v2491 = v2472 / (v2489 * v2461);
                    let v2492 = v310 / v2491;
                    let v2493 = v1 / v2491;
                    let v2494 = v2472 / v285;
                    let v2495 = v53 * v2488;
                    let v2496 = v2488 * v2488;
                    let v2501 = v2495 + (v53 * ((v2496 + v2497).sqrt()));
                    let v2503 = v2501 * v2501;
                    let v2504 = v2492 * v2492;
                    let v2509 = v2493 * v2493;
                    let v2518 = v1627 / v110;
                    let v2520 = (v2494 * v2501).powf(v340);
                    let v2527 = (v93 * v1627) / v110;
                    let v2530 = ((v2501 + (v2461 * (v1 - ((v2491 * ((v2501 * v2492) / ((v2503 + v2504).sqrt()))).ln())))) - (v2518 * v2520)) / ((v2501 * (v1 + (v2461 / ((v2501 * v2493) / ((v2503 + v2509).sqrt()))))) + (v2527 * v2520));
                    let v2531 = v93 * v2461;
                    let v2532 = v2488 / v2531;
                    let v2533 = if v2532 < v354 { 1.0 } else { 0.0 };
                    let v2567: f64;
                    if v2533 != 0.0 {
                        let v2555 = ((v2531 * v2494) * (((v110 * v2532) / v119) + (((rspice_limited_exp((v2532 / v119))) + (rspice_limited_exp(((v2536 * v2532) / v119)))).ln()))) / ((v1 / v2530) + ((v2494 / v287) * (rspice_limited_exp(((v2549 * v2488) / v2531)))));
                        v2567 = v2555;
                    } else {
                        let v2566 = ((v2531 * v2494) * v2532) / ((v1 / v2530) + ((v2494 / v287) * (rspice_limited_exp(((v2560 * v2488) / v2531)))));
                        v2567 = v2566;
                    }
                    let v2569 = v2488 - (v2567 / v2494);
                    let v2572 = if ((v2569 - v2488).abs()) > v394 { 1.0 } else { 0.0 };
                    let v2702: f64;
                    if v2572 != 0.0 {
                        let v2573 = v2488 - v2569;
                        let v2580 = (v53 * v2573) + (v53 * (((v2573 * v2573) + v2576).sqrt()));
                        let v2581 = v2494.powf(v340);
                        let v2582 = v2580.powf(v340);
                        let v2584 = v2580.powf(v2583);
                        let v2585 = v1627 * v2581;
                        let v2587 = v1697 * v2581;
                        let v2589 = v2569 / v2461;
                        let v2591 = v2589 - ((v2585 * v2582) / v2461);
                        let v2593 = v2589 - ((v2587 * v2582) / v2461);
                        let v2594 = if v2591 >= v418 { 1.0 } else { 0.0 };
                        let v2608: f64;
                        if v2594 != 0.0 {
                            v2608 = v2591;
                        } else {
                            let v2596 = if v2591 <= v2595 { 1.0 } else { 0.0 };
                            let v2609: f64;
                            if v2596 != 0.0 {
                                v2609 = v0;
                            } else {
                                let v2599 = ((v2591.exp()) + v1).ln();
                                v2609 = v2599;
                            }
                            v2608 = v2609;
                        }
                        let v2600 = if v2593 >= v418 { 1.0 } else { 0.0 };
                        let v2612: f64;
                        if v2600 != 0.0 {
                            v2612 = v2593;
                        } else {
                            let v2602 = if v2593 <= v2601 { 1.0 } else { 0.0 };
                            let v2613: f64;
                            if v2602 != 0.0 {
                                v2613 = v0;
                            } else {
                                let v2605 = ((v2593.exp()) + v1).ln();
                                v2613 = v2605;
                            }
                            v2612 = v2613;
                        }
                        let v2607 = v287 * v2461;
                        let v2618 = rspice_limited_exp(v2591);
                        let v2624 = rspice_limited_exp(v2593);
                        let v2637 = v2569 - ((((v2494 * v2580) - (v2607 * v2608)) - (v2607 * v2612)) / (((v2630 * v2494) - (((v2618 * v287) * (v1 + (v340 * (v2585 * v2584)))) / (v1 + v2618))) - (((v2624 * v287) * (v1 + (v340 * (v2587 * v2584)))) / (v1 + v2624))));
                        let v2638 = v2488 - v2637;
                        let v2645 = (v53 * v2638) + (v53 * (((v2638 * v2638) + v2641).sqrt()));
                        let v2647 = v2645.powf(v2646);
                        let v2648 = v2645.powf(v340);
                        let v2651 = v2637 / v2461;
                        let v2653 = v2651 - ((v2585 * v2648) / v2461);
                        let v2655 = v2651 - ((v2587 * v2648) / v2461);
                        let v2656 = if v2653 >= v418 { 1.0 } else { 0.0 };
                        let v2669: f64;
                        if v2656 != 0.0 {
                            v2669 = v2653;
                        } else {
                            let v2658 = if v2653 <= v2657 { 1.0 } else { 0.0 };
                            let v2670: f64;
                            if v2658 != 0.0 {
                                v2670 = v0;
                            } else {
                                let v2661 = ((v2653.exp()) + v1).ln();
                                v2670 = v2661;
                            }
                            v2669 = v2670;
                        }
                        let v2662 = if v2655 >= v418 { 1.0 } else { 0.0 };
                        let v2673: f64;
                        if v2662 != 0.0 {
                            v2673 = v2655;
                        } else {
                            let v2664 = if v2655 <= v2663 { 1.0 } else { 0.0 };
                            let v2674: f64;
                            if v2664 != 0.0 {
                                v2674 = v0;
                            } else {
                                let v2667 = ((v2655.exp()) + v1).ln();
                                v2674 = v2667;
                            }
                            v2673 = v2674;
                        }
                        let v2679 = rspice_limited_exp(v2653);
                        let v2685 = rspice_limited_exp(v2655);
                        let v2698 = v2637 - ((((v2494 * v2645) - (v2607 * v2669)) - (v2607 * v2673)) / (((v2691 * v2494) - (((v2679 * v287) * (v1 + (v340 * (v2585 * v2647)))) / (v1 + v2679))) - (((v2685 * v287) * (v1 + (v340 * (v2587 * v2647)))) / (v1 + v2685))));
                        v2702 = v2698;
                    } else {
                        v2702 = v2569;
                    }
                    let v2705 = (v2472 / v217) * ((v2488 - v2702).abs());
                    let v2723 = v2495 + (v53 * ((v2496 + v2719).sqrt()));
                    let v2724 = ((v93 * (v1812 * v540)) / ((v1810 * v537) / (((v1 + (v551 * v2705)) + (v554 * (v2705 * v2705))) + (v558 * (v547 * ((v279 - v2702).abs())))))) * v1580;
                    let v2735 = v2488 - (v2453 * ((v1 + ((v2453 / ((v2724 * v2723) / (v2724 + v2723))).powf(v574))).powf((v2731 / v574))));
                    let v2742 = (v53 * v2735) + (v53 * (((v2735 * v2735) + v2738).sqrt()));
                    let v2744 = v2742 * v2742;
                    let v2758 = (v2494 * v2742).powf(v340);
                    let v2766 = ((v2742 + (v2461 * (v1 - ((v2491 * ((v2742 * v2492) / ((v2744 + v2504).sqrt()))).ln())))) - (v2518 * v2758)) / ((v2742 * (v1 + (v2461 / ((v2742 * v2493) / ((v2744 + v2509).sqrt()))))) + (v2527 * v2758));
                    let v2767 = v2735 / v2531;
                    let v2768 = if v2767 < v354 { 1.0 } else { 0.0 };
                    let v2802: f64;
                    if v2768 != 0.0 {
                        let v2790 = ((v2531 * v2494) * (((v110 * v2767) / v119) + (((rspice_limited_exp((v2767 / v119))) + (rspice_limited_exp(((v2771 * v2767) / v119)))).ln()))) / ((v1 / v2766) + ((v2494 / v287) * (rspice_limited_exp(((v2784 * v2735) / v2531)))));
                        v2802 = v2790;
                    } else {
                        let v2801 = ((v2531 * v2494) * v2767) / ((v1 / v2766) + ((v2494 / v287) * (rspice_limited_exp(((v2795 * v2735) / v2531)))));
                        v2802 = v2801;
                    }
                    let v2804 = v2735 - (v2802 / v2494);
                    let v2807 = if ((v2804 - v2735).abs()) > v394 { 1.0 } else { 0.0 };
                    if v2807 != 0.0 {
                        let v2808 = v2735 - v2804;
                        let v2815 = (v53 * v2808) + (v53 * (((v2808 * v2808) + v2811).sqrt()));
                        let v2816 = v2494.powf(v340);
                        let v2817 = v2815.powf(v340);
                        let v2819 = v2815.powf(v2818);
                        let v2820 = v1627 * v2816;
                        let v2822 = v1697 * v2816;
                        let v2824 = v2804 / v2461;
                        let v2826 = v2824 - ((v2820 * v2817) / v2461);
                        let v2828 = v2824 - ((v2822 * v2817) / v2461);
                        let v2829 = if v2826 >= v418 { 1.0 } else { 0.0 };
                        let v2843: f64;
                        if v2829 != 0.0 {
                            v2843 = v2826;
                        } else {
                            let v2831 = if v2826 <= v2830 { 1.0 } else { 0.0 };
                            let v2844: f64;
                            if v2831 != 0.0 {
                                v2844 = v0;
                            } else {
                                let v2834 = ((v2826.exp()) + v1).ln();
                                v2844 = v2834;
                            }
                            v2843 = v2844;
                        }
                        let v2835 = if v2828 >= v418 { 1.0 } else { 0.0 };
                        let v2847: f64;
                        if v2835 != 0.0 {
                            v2847 = v2828;
                        } else {
                            let v2837 = if v2828 <= v2836 { 1.0 } else { 0.0 };
                            let v2848: f64;
                            if v2837 != 0.0 {
                                v2848 = v0;
                            } else {
                                let v2840 = ((v2828.exp()) + v1).ln();
                                v2848 = v2840;
                            }
                            v2847 = v2848;
                        }
                        let v2842 = v287 * v2461;
                        let v2853 = rspice_limited_exp(v2826);
                        let v2859 = rspice_limited_exp(v2828);
                        let v2872 = v2804 - ((((v2494 * v2815) - (v2842 * v2843)) - (v2842 * v2847)) / (((v2865 * v2494) - (((v2853 * v287) * (v1 + (v340 * (v2820 * v2819)))) / (v1 + v2853))) - (((v2859 * v287) * (v1 + (v340 * (v2822 * v2819)))) / (v1 + v2859))));
                        let v2873 = v2735 - v2872;
                        let v2881 = ((v53 * v2873) + (v53 * (((v2873 * v2873) + v2876).sqrt()))).powf(v340);
                        let v2884 = v2872 / v2461;
                        let v2886 = v2884 - ((v2820 * v2881) / v2461);
                        let v2888 = v2884 - ((v2822 * v2881) / v2461);
                        let v2889 = if v2886 >= v418 { 1.0 } else { 0.0 };
                        if v2889 != 0.0 {
                        } else {
                            let v2891 = if v2886 <= v2890 { 1.0 } else { 0.0 };
                            if v2891 != 0.0 {
                            } else {
                            }
                        }
                        let v2892 = if v2888 >= v418 { 1.0 } else { 0.0 };
                        if v2892 != 0.0 {
                        } else {
                            let v2894 = if v2888 <= v2893 { 1.0 } else { 0.0 };
                            if v2894 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v2896 = if v2895 < v0 { 1.0 } else { 0.0 };
                    if v2896 != 0.0 {
                    } else {
                    }
                    v8837 = v2461;
                } else {
                    v8837 = v8838;
                }
                v8836 = v8837;
            } else {
                let v2897 = if v2440 != v0 { 1.0 } else { 0.0 };
                let v8841: f64;
                if v2897 != 0.0 {
                    let v2898 = if v2440 == v1 { 1.0 } else { 0.0 };
                    if v2898 != 0.0 {
                    } else {
                    }
                    let v2900 = v47 * (v1 + v1558);
                    let v2902 = v1564 + (v264 * v1565);
                    let v2903 = v217 / v1578;
                    let v2910 = v2902 + (v2900 * (((v1580 / ((v288 * v2900) * v2900)) * v1584).ln()));
                    let v2911 = v0 - v2910;
                    let v2918 = ((v53 * (v2911 + (((v2911 * v2911) + v300).sqrt()))) + v2910) - v2902;
                    let v2921 = v2903 / (v2919 * v2900);
                    let v2922 = v310 / v2921;
                    let v2923 = v1 / v2921;
                    let v2924 = v2903 / v285;
                    let v2925 = v53 * v2918;
                    let v2926 = v2918 * v2918;
                    let v2931 = v2925 + (v53 * ((v2926 + v2927).sqrt()));
                    let v2933 = v2931 * v2931;
                    let v2934 = v2922 * v2922;
                    let v2939 = v2923 * v2923;
                    let v2948 = v1627 / v110;
                    let v2950 = (v2924 * v2931).powf(v340);
                    let v2957 = (v93 * v1627) / v110;
                    let v2960 = ((v2931 + (v2900 * (v1 - ((v2921 * ((v2931 * v2922) / ((v2933 + v2934).sqrt()))).ln())))) - (v2948 * v2950)) / ((v2931 * (v1 + (v2900 / ((v2931 * v2923) / ((v2933 + v2939).sqrt()))))) + (v2957 * v2950));
                    let v2961 = v93 * v2900;
                    let v2962 = v2918 / v2961;
                    let v2963 = if v2962 < v354 { 1.0 } else { 0.0 };
                    let v2997: f64;
                    if v2963 != 0.0 {
                        let v2985 = ((v2961 * v2924) * (((v110 * v2962) / v119) + (((rspice_limited_exp((v2962 / v119))) + (rspice_limited_exp(((v2966 * v2962) / v119)))).ln()))) / ((v1 / v2960) + ((v2924 / v287) * (rspice_limited_exp(((v2979 * v2918) / v2961)))));
                        v2997 = v2985;
                    } else {
                        let v2996 = ((v2961 * v2924) * v2962) / ((v1 / v2960) + ((v2924 / v287) * (rspice_limited_exp(((v2990 * v2918) / v2961)))));
                        v2997 = v2996;
                    }
                    let v2999 = v2918 - (v2997 / v2924);
                    let v3002 = if ((v2999 - v2918).abs()) > v394 { 1.0 } else { 0.0 };
                    let v3132: f64;
                    if v3002 != 0.0 {
                        let v3003 = v2918 - v2999;
                        let v3010 = (v53 * v3003) + (v53 * (((v3003 * v3003) + v3006).sqrt()));
                        let v3011 = v2924.powf(v340);
                        let v3012 = v3010.powf(v340);
                        let v3014 = v3010.powf(v3013);
                        let v3015 = v1627 * v3011;
                        let v3017 = v1697 * v3011;
                        let v3019 = v2999 / v2900;
                        let v3021 = v3019 - ((v3015 * v3012) / v2900);
                        let v3023 = v3019 - ((v3017 * v3012) / v2900);
                        let v3024 = if v3021 >= v418 { 1.0 } else { 0.0 };
                        let v3038: f64;
                        if v3024 != 0.0 {
                            v3038 = v3021;
                        } else {
                            let v3026 = if v3021 <= v3025 { 1.0 } else { 0.0 };
                            let v3039: f64;
                            if v3026 != 0.0 {
                                v3039 = v0;
                            } else {
                                let v3029 = ((v3021.exp()) + v1).ln();
                                v3039 = v3029;
                            }
                            v3038 = v3039;
                        }
                        let v3030 = if v3023 >= v418 { 1.0 } else { 0.0 };
                        let v3042: f64;
                        if v3030 != 0.0 {
                            v3042 = v3023;
                        } else {
                            let v3032 = if v3023 <= v3031 { 1.0 } else { 0.0 };
                            let v3043: f64;
                            if v3032 != 0.0 {
                                v3043 = v0;
                            } else {
                                let v3035 = ((v3023.exp()) + v1).ln();
                                v3043 = v3035;
                            }
                            v3042 = v3043;
                        }
                        let v3037 = v287 * v2900;
                        let v3048 = rspice_limited_exp(v3021);
                        let v3054 = rspice_limited_exp(v3023);
                        let v3067 = v2999 - ((((v2924 * v3010) - (v3037 * v3038)) - (v3037 * v3042)) / (((v3060 * v2924) - (((v3048 * v287) * (v1 + (v340 * (v3015 * v3014)))) / (v1 + v3048))) - (((v3054 * v287) * (v1 + (v340 * (v3017 * v3014)))) / (v1 + v3054))));
                        let v3068 = v2918 - v3067;
                        let v3075 = (v53 * v3068) + (v53 * (((v3068 * v3068) + v3071).sqrt()));
                        let v3077 = v3075.powf(v3076);
                        let v3078 = v3075.powf(v340);
                        let v3081 = v3067 / v2900;
                        let v3083 = v3081 - ((v3015 * v3078) / v2900);
                        let v3085 = v3081 - ((v3017 * v3078) / v2900);
                        let v3086 = if v3083 >= v418 { 1.0 } else { 0.0 };
                        let v3099: f64;
                        if v3086 != 0.0 {
                            v3099 = v3083;
                        } else {
                            let v3088 = if v3083 <= v3087 { 1.0 } else { 0.0 };
                            let v3100: f64;
                            if v3088 != 0.0 {
                                v3100 = v0;
                            } else {
                                let v3091 = ((v3083.exp()) + v1).ln();
                                v3100 = v3091;
                            }
                            v3099 = v3100;
                        }
                        let v3092 = if v3085 >= v418 { 1.0 } else { 0.0 };
                        let v3103: f64;
                        if v3092 != 0.0 {
                            v3103 = v3085;
                        } else {
                            let v3094 = if v3085 <= v3093 { 1.0 } else { 0.0 };
                            let v3104: f64;
                            if v3094 != 0.0 {
                                v3104 = v0;
                            } else {
                                let v3097 = ((v3085.exp()) + v1).ln();
                                v3104 = v3097;
                            }
                            v3103 = v3104;
                        }
                        let v3109 = rspice_limited_exp(v3083);
                        let v3115 = rspice_limited_exp(v3085);
                        let v3128 = v3067 - ((((v2924 * v3075) - (v3037 * v3099)) - (v3037 * v3103)) / (((v3121 * v2924) - (((v3109 * v287) * (v1 + (v340 * (v3015 * v3077)))) / (v1 + v3109))) - (((v3115 * v287) * (v1 + (v340 * (v3017 * v3077)))) / (v1 + v3115))));
                        v3132 = v3128;
                    } else {
                        v3132 = v2999;
                    }
                    let v3135 = (v2903 / v217) * ((v2918 - v3132).abs());
                    let v3153 = v2925 + (v53 * ((v2926 + v3149).sqrt()));
                    let v3154 = ((v93 * (v1812 * v540)) / ((v1810 * v537) / (((v1 + (v551 * v3135)) + (v554 * (v3135 * v3135))) + (v558 * (v547 * ((v279 - v3132).abs())))))) * v1580;
                    let v3165 = v2918 - (v0 * ((v1 + ((v0 / ((v3154 * v3153) / (v3154 + v3153))).powf(v574))).powf((v3161 / v574))));
                    let v3172 = (v53 * v3165) + (v53 * (((v3165 * v3165) + v3168).sqrt()));
                    let v3174 = v3172 * v3172;
                    let v3188 = (v2924 * v3172).powf(v340);
                    let v3196 = ((v3172 + (v2900 * (v1 - ((v2921 * ((v3172 * v2922) / ((v3174 + v2934).sqrt()))).ln())))) - (v2948 * v3188)) / ((v3172 * (v1 + (v2900 / ((v3172 * v2923) / ((v3174 + v2939).sqrt()))))) + (v2957 * v3188));
                    let v3197 = v3165 / v2961;
                    let v3198 = if v3197 < v354 { 1.0 } else { 0.0 };
                    let v3232: f64;
                    if v3198 != 0.0 {
                        let v3220 = ((v2961 * v2924) * (((v110 * v3197) / v119) + (((rspice_limited_exp((v3197 / v119))) + (rspice_limited_exp(((v3201 * v3197) / v119)))).ln()))) / ((v1 / v3196) + ((v2924 / v287) * (rspice_limited_exp(((v3214 * v3165) / v2961)))));
                        v3232 = v3220;
                    } else {
                        let v3231 = ((v2961 * v2924) * v3197) / ((v1 / v3196) + ((v2924 / v287) * (rspice_limited_exp(((v3225 * v3165) / v2961)))));
                        v3232 = v3231;
                    }
                    let v3234 = v3165 - (v3232 / v2924);
                    let v3237 = if ((v3234 - v3165).abs()) > v394 { 1.0 } else { 0.0 };
                    if v3237 != 0.0 {
                        let v3238 = v3165 - v3234;
                        let v3245 = (v53 * v3238) + (v53 * (((v3238 * v3238) + v3241).sqrt()));
                        let v3246 = v2924.powf(v340);
                        let v3247 = v3245.powf(v340);
                        let v3249 = v3245.powf(v3248);
                        let v3250 = v1627 * v3246;
                        let v3252 = v1697 * v3246;
                        let v3254 = v3234 / v2900;
                        let v3256 = v3254 - ((v3250 * v3247) / v2900);
                        let v3258 = v3254 - ((v3252 * v3247) / v2900);
                        let v3259 = if v3256 >= v418 { 1.0 } else { 0.0 };
                        let v3273: f64;
                        if v3259 != 0.0 {
                            v3273 = v3256;
                        } else {
                            let v3261 = if v3256 <= v3260 { 1.0 } else { 0.0 };
                            let v3274: f64;
                            if v3261 != 0.0 {
                                v3274 = v0;
                            } else {
                                let v3264 = ((v3256.exp()) + v1).ln();
                                v3274 = v3264;
                            }
                            v3273 = v3274;
                        }
                        let v3265 = if v3258 >= v418 { 1.0 } else { 0.0 };
                        let v3277: f64;
                        if v3265 != 0.0 {
                            v3277 = v3258;
                        } else {
                            let v3267 = if v3258 <= v3266 { 1.0 } else { 0.0 };
                            let v3278: f64;
                            if v3267 != 0.0 {
                                v3278 = v0;
                            } else {
                                let v3270 = ((v3258.exp()) + v1).ln();
                                v3278 = v3270;
                            }
                            v3277 = v3278;
                        }
                        let v3272 = v287 * v2900;
                        let v3283 = rspice_limited_exp(v3256);
                        let v3289 = rspice_limited_exp(v3258);
                        let v3302 = v3234 - ((((v2924 * v3245) - (v3272 * v3273)) - (v3272 * v3277)) / (((v3295 * v2924) - (((v3283 * v287) * (v1 + (v340 * (v3250 * v3249)))) / (v1 + v3283))) - (((v3289 * v287) * (v1 + (v340 * (v3252 * v3249)))) / (v1 + v3289))));
                        let v3303 = v3165 - v3302;
                        let v3311 = ((v53 * v3303) + (v53 * (((v3303 * v3303) + v3306).sqrt()))).powf(v340);
                        let v3314 = v3302 / v2900;
                        let v3316 = v3314 - ((v3250 * v3311) / v2900);
                        let v3318 = v3314 - ((v3252 * v3311) / v2900);
                        let v3319 = if v3316 >= v418 { 1.0 } else { 0.0 };
                        if v3319 != 0.0 {
                        } else {
                            let v3321 = if v3316 <= v3320 { 1.0 } else { 0.0 };
                            if v3321 != 0.0 {
                            } else {
                            }
                        }
                        let v3322 = if v3318 >= v418 { 1.0 } else { 0.0 };
                        if v3322 != 0.0 {
                        } else {
                            let v3324 = if v3318 <= v3323 { 1.0 } else { 0.0 };
                            if v3324 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    v8841 = v2900;
                } else {
                    v8841 = v8838;
                }
                v8836 = v8841;
            }
            let v8834: f64;
            if v1539 != 0.0 {
                let v3326 = if v3325 != v0 { 1.0 } else { 0.0 };
                let v8835: f64;
                if v3326 != 0.0 {
                    let v3328 = v3327 - v1542;
                    let v3329 = if v3325 == v1 { 1.0 } else { 0.0 };
                    let v3337: f64;
                    let v3338: f64;
                    if v3329 != 0.0 {
                        let v3330 = v19 - v1542;
                        let v3331 = v19 - v3327;
                        v3337 = v3331;
                        v3338 = v3330;
                    } else {
                        let v3332 = v35 - v1542;
                        let v3333 = v35 - v3327;
                        v3337 = v3333;
                        v3338 = v3332;
                    }
                    let v3334 = if v3328 < v0 { 1.0 } else { 0.0 };
                    let v3339: f64;
                    let v3375: f64;
                    let v3794: f64;
                    if v3334 != 0.0 {
                        let v3336 = v3335 * v3328;
                        v3339 = v3336;
                        v3375 = v3337;
                        v3794 = v3335;
                    } else {
                        v3339 = v3328;
                        v3375 = v3338;
                        v3794 = v1;
                    }
                    let v3343 = (((v3339 * v3339) + v2).sqrt()) - v32;
                    let v3349 = v47 * ((v1 + v3344) + (v3346 * v3343));
                    let v3363 = (v3350 - (v264 * v3351)) - ((v3354 * (v3343 * v3355)) / (((v3343 * v3343) + (v3355 * v3355)).sqrt()));
                    let v3365 = v217 / v3364;
                    let v3374 = v3363 + (v3349 * (((v3366 / ((v288 * v3349) * v3349)) * v3370).ln()));
                    let v3376 = v3375 - v3374;
                    let v3383 = ((v53 * (v3376 + (((v3376 * v3376) + v300).sqrt()))) + v3374) - v3363;
                    let v3386 = v3365 / (v3384 * v3349);
                    let v3387 = v310 / v3386;
                    let v3388 = v1 / v3386;
                    let v3389 = v3365 / v285;
                    let v3390 = v53 * v3383;
                    let v3391 = v3383 * v3383;
                    let v3396 = v3390 + (v53 * ((v3391 + v3392).sqrt()));
                    let v3398 = v3396 * v3396;
                    let v3399 = v3387 * v3387;
                    let v3404 = v3388 * v3388;
                    let v3414 = v3413 / v110;
                    let v3416 = (v3389 * v3396).powf(v340);
                    let v3423 = (v93 * v3413) / v110;
                    let v3426 = ((v3396 + (v3349 * (v1 - ((v3386 * ((v3396 * v3387) / ((v3398 + v3399).sqrt()))).ln())))) - (v3414 * v3416)) / ((v3396 * (v1 + (v3349 / ((v3396 * v3388) / ((v3398 + v3404).sqrt()))))) + (v3423 * v3416));
                    let v3427 = v93 * v3349;
                    let v3428 = v3383 / v3427;
                    let v3429 = if v3428 < v354 { 1.0 } else { 0.0 };
                    let v3463: f64;
                    if v3429 != 0.0 {
                        let v3451 = ((v3427 * v3389) * (((v110 * v3428) / v119) + (((rspice_limited_exp((v3428 / v119))) + (rspice_limited_exp(((v3432 * v3428) / v119)))).ln()))) / ((v1 / v3426) + ((v3389 / v287) * (rspice_limited_exp(((v3445 * v3383) / v3427)))));
                        v3463 = v3451;
                    } else {
                        let v3462 = ((v3427 * v3389) * v3428) / ((v1 / v3426) + ((v3389 / v287) * (rspice_limited_exp(((v3456 * v3383) / v3427)))));
                        v3463 = v3462;
                    }
                    let v3465 = v3383 - (v3463 / v3389);
                    let v3468 = if ((v3465 - v3383).abs()) > v394 { 1.0 } else { 0.0 };
                    let v3601: f64;
                    if v3468 != 0.0 {
                        let v3469 = v3383 - v3465;
                        let v3476 = (v53 * v3469) + (v53 * (((v3469 * v3469) + v3472).sqrt()));
                        let v3477 = v3389.powf(v340);
                        let v3478 = v3476.powf(v340);
                        let v3480 = v3476.powf(v3479);
                        let v3481 = v3413 * v3477;
                        let v3484 = v3483 * v3477;
                        let v3486 = v3465 / v3349;
                        let v3488 = v3486 - ((v3481 * v3478) / v3349);
                        let v3490 = v3486 - ((v3484 * v3478) / v3349);
                        let v3491 = if v3488 >= v418 { 1.0 } else { 0.0 };
                        let v3505: f64;
                        if v3491 != 0.0 {
                            v3505 = v3488;
                        } else {
                            let v3493 = if v3488 <= v3492 { 1.0 } else { 0.0 };
                            let v3506: f64;
                            if v3493 != 0.0 {
                                v3506 = v0;
                            } else {
                                let v3496 = ((v3488.exp()) + v1).ln();
                                v3506 = v3496;
                            }
                            v3505 = v3506;
                        }
                        let v3497 = if v3490 >= v418 { 1.0 } else { 0.0 };
                        let v3509: f64;
                        if v3497 != 0.0 {
                            v3509 = v3490;
                        } else {
                            let v3499 = if v3490 <= v3498 { 1.0 } else { 0.0 };
                            let v3510: f64;
                            if v3499 != 0.0 {
                                v3510 = v0;
                            } else {
                                let v3502 = ((v3490.exp()) + v1).ln();
                                v3510 = v3502;
                            }
                            v3509 = v3510;
                        }
                        let v3504 = v287 * v3349;
                        let v3515 = rspice_limited_exp(v3488);
                        let v3521 = rspice_limited_exp(v3490);
                        let v3534 = v3465 - ((((v3389 * v3476) - (v3504 * v3505)) - (v3504 * v3509)) / (((v3527 * v3389) - (((v3515 * v287) * (v1 + (v340 * (v3481 * v3480)))) / (v1 + v3515))) - (((v3521 * v287) * (v1 + (v340 * (v3484 * v3480)))) / (v1 + v3521))));
                        let v3535 = v3383 - v3534;
                        let v3542 = (v53 * v3535) + (v53 * (((v3535 * v3535) + v3538).sqrt()));
                        let v3544 = v3542.powf(v3543);
                        let v3545 = v3542.powf(v340);
                        let v3548 = v3534 / v3349;
                        let v3550 = v3548 - ((v3481 * v3545) / v3349);
                        let v3552 = v3548 - ((v3484 * v3545) / v3349);
                        let v3553 = if v3550 >= v418 { 1.0 } else { 0.0 };
                        let v3566: f64;
                        if v3553 != 0.0 {
                            v3566 = v3550;
                        } else {
                            let v3555 = if v3550 <= v3554 { 1.0 } else { 0.0 };
                            let v3567: f64;
                            if v3555 != 0.0 {
                                v3567 = v0;
                            } else {
                                let v3558 = ((v3550.exp()) + v1).ln();
                                v3567 = v3558;
                            }
                            v3566 = v3567;
                        }
                        let v3559 = if v3552 >= v418 { 1.0 } else { 0.0 };
                        let v3570: f64;
                        if v3559 != 0.0 {
                            v3570 = v3552;
                        } else {
                            let v3561 = if v3552 <= v3560 { 1.0 } else { 0.0 };
                            let v3571: f64;
                            if v3561 != 0.0 {
                                v3571 = v0;
                            } else {
                                let v3564 = ((v3552.exp()) + v1).ln();
                                v3571 = v3564;
                            }
                            v3570 = v3571;
                        }
                        let v3576 = rspice_limited_exp(v3550);
                        let v3582 = rspice_limited_exp(v3552);
                        let v3595 = v3534 - ((((v3389 * v3542) - (v3504 * v3566)) - (v3504 * v3570)) / (((v3588 * v3389) - (((v3576 * v287) * (v1 + (v340 * (v3481 * v3544)))) / (v1 + v3576))) - (((v3582 * v287) * (v1 + (v340 * (v3484 * v3544)))) / (v1 + v3582))));
                        v3601 = v3595;
                    } else {
                        v3601 = v3465;
                    }
                    let v3604 = (v3365 / v217) * ((v3383 - v3601).abs());
                    let v3622 = v3390 + (v53 * ((v3391 + v3618).sqrt()));
                    let v3623 = ((v93 * (v3598 * v540)) / ((v3596 * v537) / (((v1 + (v551 * v3604)) + (v554 * (v3604 * v3604))) + (v558 * (v547 * ((v279 - v3601).abs())))))) * v3366;
                    let v3634 = v3383 - (v3339 * ((v1 + ((v3339 / ((v3623 * v3622) / (v3623 + v3622))).powf(v574))).powf((v3630 / v574))));
                    let v3641 = (v53 * v3634) + (v53 * (((v3634 * v3634) + v3637).sqrt()));
                    let v3643 = v3641 * v3641;
                    let v3657 = (v3389 * v3641).powf(v340);
                    let v3665 = ((v3641 + (v3349 * (v1 - ((v3386 * ((v3641 * v3387) / ((v3643 + v3399).sqrt()))).ln())))) - (v3414 * v3657)) / ((v3641 * (v1 + (v3349 / ((v3641 * v3388) / ((v3643 + v3404).sqrt()))))) + (v3423 * v3657));
                    let v3666 = v3634 / v3427;
                    let v3667 = if v3666 < v354 { 1.0 } else { 0.0 };
                    let v3701: f64;
                    if v3667 != 0.0 {
                        let v3689 = ((v3427 * v3389) * (((v110 * v3666) / v119) + (((rspice_limited_exp((v3666 / v119))) + (rspice_limited_exp(((v3670 * v3666) / v119)))).ln()))) / ((v1 / v3665) + ((v3389 / v287) * (rspice_limited_exp(((v3683 * v3634) / v3427)))));
                        v3701 = v3689;
                    } else {
                        let v3700 = ((v3427 * v3389) * v3666) / ((v1 / v3665) + ((v3389 / v287) * (rspice_limited_exp(((v3694 * v3634) / v3427)))));
                        v3701 = v3700;
                    }
                    let v3703 = v3634 - (v3701 / v3389);
                    let v3706 = if ((v3703 - v3634).abs()) > v394 { 1.0 } else { 0.0 };
                    if v3706 != 0.0 {
                        let v3707 = v3634 - v3703;
                        let v3714 = (v53 * v3707) + (v53 * (((v3707 * v3707) + v3710).sqrt()));
                        let v3715 = v3389.powf(v340);
                        let v3716 = v3714.powf(v340);
                        let v3718 = v3714.powf(v3717);
                        let v3719 = v3413 * v3715;
                        let v3721 = v3483 * v3715;
                        let v3723 = v3703 / v3349;
                        let v3725 = v3723 - ((v3719 * v3716) / v3349);
                        let v3727 = v3723 - ((v3721 * v3716) / v3349);
                        let v3728 = if v3725 >= v418 { 1.0 } else { 0.0 };
                        let v3742: f64;
                        if v3728 != 0.0 {
                            v3742 = v3725;
                        } else {
                            let v3730 = if v3725 <= v3729 { 1.0 } else { 0.0 };
                            let v3743: f64;
                            if v3730 != 0.0 {
                                v3743 = v0;
                            } else {
                                let v3733 = ((v3725.exp()) + v1).ln();
                                v3743 = v3733;
                            }
                            v3742 = v3743;
                        }
                        let v3734 = if v3727 >= v418 { 1.0 } else { 0.0 };
                        let v3746: f64;
                        if v3734 != 0.0 {
                            v3746 = v3727;
                        } else {
                            let v3736 = if v3727 <= v3735 { 1.0 } else { 0.0 };
                            let v3747: f64;
                            if v3736 != 0.0 {
                                v3747 = v0;
                            } else {
                                let v3739 = ((v3727.exp()) + v1).ln();
                                v3747 = v3739;
                            }
                            v3746 = v3747;
                        }
                        let v3741 = v287 * v3349;
                        let v3752 = rspice_limited_exp(v3725);
                        let v3758 = rspice_limited_exp(v3727);
                        let v3771 = v3703 - ((((v3389 * v3714) - (v3741 * v3742)) - (v3741 * v3746)) / (((v3764 * v3389) - (((v3752 * v287) * (v1 + (v340 * (v3719 * v3718)))) / (v1 + v3752))) - (((v3758 * v287) * (v1 + (v340 * (v3721 * v3718)))) / (v1 + v3758))));
                        let v3772 = v3634 - v3771;
                        let v3780 = ((v53 * v3772) + (v53 * (((v3772 * v3772) + v3775).sqrt()))).powf(v340);
                        let v3783 = v3771 / v3349;
                        let v3785 = v3783 - ((v3719 * v3780) / v3349);
                        let v3787 = v3783 - ((v3721 * v3780) / v3349);
                        let v3788 = if v3785 >= v418 { 1.0 } else { 0.0 };
                        if v3788 != 0.0 {
                        } else {
                            let v3790 = if v3785 <= v3789 { 1.0 } else { 0.0 };
                            if v3790 != 0.0 {
                            } else {
                            }
                        }
                        let v3791 = if v3787 >= v418 { 1.0 } else { 0.0 };
                        if v3791 != 0.0 {
                        } else {
                            let v3793 = if v3787 <= v3792 { 1.0 } else { 0.0 };
                            if v3793 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v3795 = if v3794 < v0 { 1.0 } else { 0.0 };
                    if v3795 != 0.0 {
                    } else {
                    }
                    v8835 = v3349;
                } else {
                    v8835 = v8836;
                }
                v8834 = v8835;
            } else {
                let v3796 = if v3325 != v0 { 1.0 } else { 0.0 };
                let v8842: f64;
                if v3796 != 0.0 {
                    let v3797 = if v3325 == v1 { 1.0 } else { 0.0 };
                    let v3799: f64;
                    if v3797 != 0.0 {
                        v3799 = v21;
                    } else {
                        let v3798 = v35 - v16;
                        v3799 = v3798;
                    }
                    let v3801 = v47 * (v1 + v3344);
                    let v3803 = v3350 - (v264 * v3351);
                    let v3804 = v217 / v3364;
                    let v3811 = v3803 + (v3801 * (((v3366 / ((v288 * v3801) * v3801)) * v3370).ln()));
                    let v3812 = v3799 - v3811;
                    let v3819 = ((v53 * (v3812 + (((v3812 * v3812) + v300).sqrt()))) + v3811) - v3803;
                    let v3822 = v3804 / (v3820 * v3801);
                    let v3823 = v310 / v3822;
                    let v3824 = v1 / v3822;
                    let v3825 = v3804 / v285;
                    let v3826 = v53 * v3819;
                    let v3827 = v3819 * v3819;
                    let v3832 = v3826 + (v53 * ((v3827 + v3828).sqrt()));
                    let v3834 = v3832 * v3832;
                    let v3835 = v3823 * v3823;
                    let v3840 = v3824 * v3824;
                    let v3849 = v3413 / v110;
                    let v3851 = (v3825 * v3832).powf(v340);
                    let v3858 = (v93 * v3413) / v110;
                    let v3861 = ((v3832 + (v3801 * (v1 - ((v3822 * ((v3832 * v3823) / ((v3834 + v3835).sqrt()))).ln())))) - (v3849 * v3851)) / ((v3832 * (v1 + (v3801 / ((v3832 * v3824) / ((v3834 + v3840).sqrt()))))) + (v3858 * v3851));
                    let v3862 = v93 * v3801;
                    let v3863 = v3819 / v3862;
                    let v3864 = if v3863 < v354 { 1.0 } else { 0.0 };
                    let v3898: f64;
                    if v3864 != 0.0 {
                        let v3886 = ((v3862 * v3825) * (((v110 * v3863) / v119) + (((rspice_limited_exp((v3863 / v119))) + (rspice_limited_exp(((v3867 * v3863) / v119)))).ln()))) / ((v1 / v3861) + ((v3825 / v287) * (rspice_limited_exp(((v3880 * v3819) / v3862)))));
                        v3898 = v3886;
                    } else {
                        let v3897 = ((v3862 * v3825) * v3863) / ((v1 / v3861) + ((v3825 / v287) * (rspice_limited_exp(((v3891 * v3819) / v3862)))));
                        v3898 = v3897;
                    }
                    let v3900 = v3819 - (v3898 / v3825);
                    let v3903 = if ((v3900 - v3819).abs()) > v394 { 1.0 } else { 0.0 };
                    let v4033: f64;
                    if v3903 != 0.0 {
                        let v3904 = v3819 - v3900;
                        let v3911 = (v53 * v3904) + (v53 * (((v3904 * v3904) + v3907).sqrt()));
                        let v3912 = v3825.powf(v340);
                        let v3913 = v3911.powf(v340);
                        let v3915 = v3911.powf(v3914);
                        let v3916 = v3413 * v3912;
                        let v3918 = v3483 * v3912;
                        let v3920 = v3900 / v3801;
                        let v3922 = v3920 - ((v3916 * v3913) / v3801);
                        let v3924 = v3920 - ((v3918 * v3913) / v3801);
                        let v3925 = if v3922 >= v418 { 1.0 } else { 0.0 };
                        let v3939: f64;
                        if v3925 != 0.0 {
                            v3939 = v3922;
                        } else {
                            let v3927 = if v3922 <= v3926 { 1.0 } else { 0.0 };
                            let v3940: f64;
                            if v3927 != 0.0 {
                                v3940 = v0;
                            } else {
                                let v3930 = ((v3922.exp()) + v1).ln();
                                v3940 = v3930;
                            }
                            v3939 = v3940;
                        }
                        let v3931 = if v3924 >= v418 { 1.0 } else { 0.0 };
                        let v3943: f64;
                        if v3931 != 0.0 {
                            v3943 = v3924;
                        } else {
                            let v3933 = if v3924 <= v3932 { 1.0 } else { 0.0 };
                            let v3944: f64;
                            if v3933 != 0.0 {
                                v3944 = v0;
                            } else {
                                let v3936 = ((v3924.exp()) + v1).ln();
                                v3944 = v3936;
                            }
                            v3943 = v3944;
                        }
                        let v3938 = v287 * v3801;
                        let v3949 = rspice_limited_exp(v3922);
                        let v3955 = rspice_limited_exp(v3924);
                        let v3968 = v3900 - ((((v3825 * v3911) - (v3938 * v3939)) - (v3938 * v3943)) / (((v3961 * v3825) - (((v3949 * v287) * (v1 + (v340 * (v3916 * v3915)))) / (v1 + v3949))) - (((v3955 * v287) * (v1 + (v340 * (v3918 * v3915)))) / (v1 + v3955))));
                        let v3969 = v3819 - v3968;
                        let v3976 = (v53 * v3969) + (v53 * (((v3969 * v3969) + v3972).sqrt()));
                        let v3978 = v3976.powf(v3977);
                        let v3979 = v3976.powf(v340);
                        let v3982 = v3968 / v3801;
                        let v3984 = v3982 - ((v3916 * v3979) / v3801);
                        let v3986 = v3982 - ((v3918 * v3979) / v3801);
                        let v3987 = if v3984 >= v418 { 1.0 } else { 0.0 };
                        let v4000: f64;
                        if v3987 != 0.0 {
                            v4000 = v3984;
                        } else {
                            let v3989 = if v3984 <= v3988 { 1.0 } else { 0.0 };
                            let v4001: f64;
                            if v3989 != 0.0 {
                                v4001 = v0;
                            } else {
                                let v3992 = ((v3984.exp()) + v1).ln();
                                v4001 = v3992;
                            }
                            v4000 = v4001;
                        }
                        let v3993 = if v3986 >= v418 { 1.0 } else { 0.0 };
                        let v4004: f64;
                        if v3993 != 0.0 {
                            v4004 = v3986;
                        } else {
                            let v3995 = if v3986 <= v3994 { 1.0 } else { 0.0 };
                            let v4005: f64;
                            if v3995 != 0.0 {
                                v4005 = v0;
                            } else {
                                let v3998 = ((v3986.exp()) + v1).ln();
                                v4005 = v3998;
                            }
                            v4004 = v4005;
                        }
                        let v4010 = rspice_limited_exp(v3984);
                        let v4016 = rspice_limited_exp(v3986);
                        let v4029 = v3968 - ((((v3825 * v3976) - (v3938 * v4000)) - (v3938 * v4004)) / (((v4022 * v3825) - (((v4010 * v287) * (v1 + (v340 * (v3916 * v3978)))) / (v1 + v4010))) - (((v4016 * v287) * (v1 + (v340 * (v3918 * v3978)))) / (v1 + v4016))));
                        v4033 = v4029;
                    } else {
                        v4033 = v3900;
                    }
                    let v4036 = (v3804 / v217) * ((v3819 - v4033).abs());
                    let v4054 = v3826 + (v53 * ((v3827 + v4050).sqrt()));
                    let v4055 = ((v93 * (v3598 * v540)) / ((v3596 * v537) / (((v1 + (v551 * v4036)) + (v554 * (v4036 * v4036))) + (v558 * (v547 * ((v279 - v4033).abs())))))) * v3366;
                    let v4066 = v3819 - (v0 * ((v1 + ((v0 / ((v4055 * v4054) / (v4055 + v4054))).powf(v574))).powf((v4062 / v574))));
                    let v4073 = (v53 * v4066) + (v53 * (((v4066 * v4066) + v4069).sqrt()));
                    let v4075 = v4073 * v4073;
                    let v4089 = (v3825 * v4073).powf(v340);
                    let v4097 = ((v4073 + (v3801 * (v1 - ((v3822 * ((v4073 * v3823) / ((v4075 + v3835).sqrt()))).ln())))) - (v3849 * v4089)) / ((v4073 * (v1 + (v3801 / ((v4073 * v3824) / ((v4075 + v3840).sqrt()))))) + (v3858 * v4089));
                    let v4098 = v4066 / v3862;
                    let v4099 = if v4098 < v354 { 1.0 } else { 0.0 };
                    let v4133: f64;
                    if v4099 != 0.0 {
                        let v4121 = ((v3862 * v3825) * (((v110 * v4098) / v119) + (((rspice_limited_exp((v4098 / v119))) + (rspice_limited_exp(((v4102 * v4098) / v119)))).ln()))) / ((v1 / v4097) + ((v3825 / v287) * (rspice_limited_exp(((v4115 * v4066) / v3862)))));
                        v4133 = v4121;
                    } else {
                        let v4132 = ((v3862 * v3825) * v4098) / ((v1 / v4097) + ((v3825 / v287) * (rspice_limited_exp(((v4126 * v4066) / v3862)))));
                        v4133 = v4132;
                    }
                    let v4135 = v4066 - (v4133 / v3825);
                    let v4138 = if ((v4135 - v4066).abs()) > v394 { 1.0 } else { 0.0 };
                    if v4138 != 0.0 {
                        let v4139 = v4066 - v4135;
                        let v4146 = (v53 * v4139) + (v53 * (((v4139 * v4139) + v4142).sqrt()));
                        let v4147 = v3825.powf(v340);
                        let v4148 = v4146.powf(v340);
                        let v4150 = v4146.powf(v4149);
                        let v4151 = v3413 * v4147;
                        let v4153 = v3483 * v4147;
                        let v4155 = v4135 / v3801;
                        let v4157 = v4155 - ((v4151 * v4148) / v3801);
                        let v4159 = v4155 - ((v4153 * v4148) / v3801);
                        let v4160 = if v4157 >= v418 { 1.0 } else { 0.0 };
                        let v4174: f64;
                        if v4160 != 0.0 {
                            v4174 = v4157;
                        } else {
                            let v4162 = if v4157 <= v4161 { 1.0 } else { 0.0 };
                            let v4175: f64;
                            if v4162 != 0.0 {
                                v4175 = v0;
                            } else {
                                let v4165 = ((v4157.exp()) + v1).ln();
                                v4175 = v4165;
                            }
                            v4174 = v4175;
                        }
                        let v4166 = if v4159 >= v418 { 1.0 } else { 0.0 };
                        let v4178: f64;
                        if v4166 != 0.0 {
                            v4178 = v4159;
                        } else {
                            let v4168 = if v4159 <= v4167 { 1.0 } else { 0.0 };
                            let v4179: f64;
                            if v4168 != 0.0 {
                                v4179 = v0;
                            } else {
                                let v4171 = ((v4159.exp()) + v1).ln();
                                v4179 = v4171;
                            }
                            v4178 = v4179;
                        }
                        let v4173 = v287 * v3801;
                        let v4184 = rspice_limited_exp(v4157);
                        let v4190 = rspice_limited_exp(v4159);
                        let v4203 = v4135 - ((((v3825 * v4146) - (v4173 * v4174)) - (v4173 * v4178)) / (((v4196 * v3825) - (((v4184 * v287) * (v1 + (v340 * (v4151 * v4150)))) / (v1 + v4184))) - (((v4190 * v287) * (v1 + (v340 * (v4153 * v4150)))) / (v1 + v4190))));
                        let v4204 = v4066 - v4203;
                        let v4212 = ((v53 * v4204) + (v53 * (((v4204 * v4204) + v4207).sqrt()))).powf(v340);
                        let v4215 = v4203 / v3801;
                        let v4217 = v4215 - ((v4151 * v4212) / v3801);
                        let v4219 = v4215 - ((v4153 * v4212) / v3801);
                        let v4220 = if v4217 >= v418 { 1.0 } else { 0.0 };
                        if v4220 != 0.0 {
                        } else {
                            let v4222 = if v4217 <= v4221 { 1.0 } else { 0.0 };
                            if v4222 != 0.0 {
                            } else {
                            }
                        }
                        let v4223 = if v4219 >= v418 { 1.0 } else { 0.0 };
                        if v4223 != 0.0 {
                        } else {
                            let v4225 = if v4219 <= v4224 { 1.0 } else { 0.0 };
                            if v4225 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    v8842 = v3801;
                } else {
                    v8842 = v8836;
                }
                v8834 = v8842;
            }
            let v8832: f64;
            if v1539 != 0.0 {
                let v4227 = if v4226 != v0 { 1.0 } else { 0.0 };
                let v8833: f64;
                if v4227 != 0.0 {
                    let v4229 = v2442 - v4228;
                    let v4230 = if v4226 == v1 { 1.0 } else { 0.0 };
                    let v4238: f64;
                    let v4239: f64;
                    if v4230 != 0.0 {
                        let v4231 = v19 - v4228;
                        let v4232 = v19 - v2442;
                        v4238 = v4232;
                        v4239 = v4231;
                    } else {
                        let v4233 = v35 - v4228;
                        let v4234 = v35 - v2442;
                        v4238 = v4234;
                        v4239 = v4233;
                    }
                    let v4235 = if v4229 < v0 { 1.0 } else { 0.0 };
                    let v4240: f64;
                    let v4267: f64;
                    let v4682: f64;
                    if v4235 != 0.0 {
                        let v4237 = v4236 * v4229;
                        v4240 = v4237;
                        v4267 = v4238;
                        v4682 = v4236;
                    } else {
                        v4240 = v4229;
                        v4267 = v4239;
                        v4682 = v1;
                    }
                    let v4244 = (((v4240 * v4240) + v2).sqrt()) - v32;
                    let v4248 = v47 * ((v1 + v3344) + (v3346 * v4244));
                    let v4258 = (v3350 + (v264 * v3351)) - ((v3354 * (v4244 * v3355)) / (((v4244 * v4244) + (v3355 * v3355)).sqrt()));
                    let v4259 = v217 / v3364;
                    let v4266 = v4258 + (v4248 * (((v3366 / ((v288 * v4248) * v4248)) * v3370).ln()));
                    let v4268 = v4267 - v4266;
                    let v4275 = ((v53 * (v4268 + (((v4268 * v4268) + v300).sqrt()))) + v4266) - v4258;
                    let v4278 = v4259 / (v4276 * v4248);
                    let v4279 = v310 / v4278;
                    let v4280 = v1 / v4278;
                    let v4281 = v4259 / v285;
                    let v4282 = v53 * v4275;
                    let v4283 = v4275 * v4275;
                    let v4288 = v4282 + (v53 * ((v4283 + v4284).sqrt()));
                    let v4290 = v4288 * v4288;
                    let v4291 = v4279 * v4279;
                    let v4296 = v4280 * v4280;
                    let v4305 = v3413 / v110;
                    let v4307 = (v4281 * v4288).powf(v340);
                    let v4314 = (v93 * v3413) / v110;
                    let v4317 = ((v4288 + (v4248 * (v1 - ((v4278 * ((v4288 * v4279) / ((v4290 + v4291).sqrt()))).ln())))) - (v4305 * v4307)) / ((v4288 * (v1 + (v4248 / ((v4288 * v4280) / ((v4290 + v4296).sqrt()))))) + (v4314 * v4307));
                    let v4318 = v93 * v4248;
                    let v4319 = v4275 / v4318;
                    let v4320 = if v4319 < v354 { 1.0 } else { 0.0 };
                    let v4354: f64;
                    if v4320 != 0.0 {
                        let v4342 = ((v4318 * v4281) * (((v110 * v4319) / v119) + (((rspice_limited_exp((v4319 / v119))) + (rspice_limited_exp(((v4323 * v4319) / v119)))).ln()))) / ((v1 / v4317) + ((v4281 / v287) * (rspice_limited_exp(((v4336 * v4275) / v4318)))));
                        v4354 = v4342;
                    } else {
                        let v4353 = ((v4318 * v4281) * v4319) / ((v1 / v4317) + ((v4281 / v287) * (rspice_limited_exp(((v4347 * v4275) / v4318)))));
                        v4354 = v4353;
                    }
                    let v4356 = v4275 - (v4354 / v4281);
                    let v4359 = if ((v4356 - v4275).abs()) > v394 { 1.0 } else { 0.0 };
                    let v4489: f64;
                    if v4359 != 0.0 {
                        let v4360 = v4275 - v4356;
                        let v4367 = (v53 * v4360) + (v53 * (((v4360 * v4360) + v4363).sqrt()));
                        let v4368 = v4281.powf(v340);
                        let v4369 = v4367.powf(v340);
                        let v4371 = v4367.powf(v4370);
                        let v4372 = v3413 * v4368;
                        let v4374 = v3483 * v4368;
                        let v4376 = v4356 / v4248;
                        let v4378 = v4376 - ((v4372 * v4369) / v4248);
                        let v4380 = v4376 - ((v4374 * v4369) / v4248);
                        let v4381 = if v4378 >= v418 { 1.0 } else { 0.0 };
                        let v4395: f64;
                        if v4381 != 0.0 {
                            v4395 = v4378;
                        } else {
                            let v4383 = if v4378 <= v4382 { 1.0 } else { 0.0 };
                            let v4396: f64;
                            if v4383 != 0.0 {
                                v4396 = v0;
                            } else {
                                let v4386 = ((v4378.exp()) + v1).ln();
                                v4396 = v4386;
                            }
                            v4395 = v4396;
                        }
                        let v4387 = if v4380 >= v418 { 1.0 } else { 0.0 };
                        let v4399: f64;
                        if v4387 != 0.0 {
                            v4399 = v4380;
                        } else {
                            let v4389 = if v4380 <= v4388 { 1.0 } else { 0.0 };
                            let v4400: f64;
                            if v4389 != 0.0 {
                                v4400 = v0;
                            } else {
                                let v4392 = ((v4380.exp()) + v1).ln();
                                v4400 = v4392;
                            }
                            v4399 = v4400;
                        }
                        let v4394 = v287 * v4248;
                        let v4405 = rspice_limited_exp(v4378);
                        let v4411 = rspice_limited_exp(v4380);
                        let v4424 = v4356 - ((((v4281 * v4367) - (v4394 * v4395)) - (v4394 * v4399)) / (((v4417 * v4281) - (((v4405 * v287) * (v1 + (v340 * (v4372 * v4371)))) / (v1 + v4405))) - (((v4411 * v287) * (v1 + (v340 * (v4374 * v4371)))) / (v1 + v4411))));
                        let v4425 = v4275 - v4424;
                        let v4432 = (v53 * v4425) + (v53 * (((v4425 * v4425) + v4428).sqrt()));
                        let v4434 = v4432.powf(v4433);
                        let v4435 = v4432.powf(v340);
                        let v4438 = v4424 / v4248;
                        let v4440 = v4438 - ((v4372 * v4435) / v4248);
                        let v4442 = v4438 - ((v4374 * v4435) / v4248);
                        let v4443 = if v4440 >= v418 { 1.0 } else { 0.0 };
                        let v4456: f64;
                        if v4443 != 0.0 {
                            v4456 = v4440;
                        } else {
                            let v4445 = if v4440 <= v4444 { 1.0 } else { 0.0 };
                            let v4457: f64;
                            if v4445 != 0.0 {
                                v4457 = v0;
                            } else {
                                let v4448 = ((v4440.exp()) + v1).ln();
                                v4457 = v4448;
                            }
                            v4456 = v4457;
                        }
                        let v4449 = if v4442 >= v418 { 1.0 } else { 0.0 };
                        let v4460: f64;
                        if v4449 != 0.0 {
                            v4460 = v4442;
                        } else {
                            let v4451 = if v4442 <= v4450 { 1.0 } else { 0.0 };
                            let v4461: f64;
                            if v4451 != 0.0 {
                                v4461 = v0;
                            } else {
                                let v4454 = ((v4442.exp()) + v1).ln();
                                v4461 = v4454;
                            }
                            v4460 = v4461;
                        }
                        let v4466 = rspice_limited_exp(v4440);
                        let v4472 = rspice_limited_exp(v4442);
                        let v4485 = v4424 - ((((v4281 * v4432) - (v4394 * v4456)) - (v4394 * v4460)) / (((v4478 * v4281) - (((v4466 * v287) * (v1 + (v340 * (v4372 * v4434)))) / (v1 + v4466))) - (((v4472 * v287) * (v1 + (v340 * (v4374 * v4434)))) / (v1 + v4472))));
                        v4489 = v4485;
                    } else {
                        v4489 = v4356;
                    }
                    let v4492 = (v4259 / v217) * ((v4275 - v4489).abs());
                    let v4510 = v4282 + (v53 * ((v4283 + v4506).sqrt()));
                    let v4511 = ((v93 * (v3598 * v540)) / ((v3596 * v537) / (((v1 + (v551 * v4492)) + (v554 * (v4492 * v4492))) + (v558 * (v547 * ((v279 - v4489).abs())))))) * v3366;
                    let v4522 = v4275 - (v4240 * ((v1 + ((v4240 / ((v4511 * v4510) / (v4511 + v4510))).powf(v574))).powf((v4518 / v574))));
                    let v4529 = (v53 * v4522) + (v53 * (((v4522 * v4522) + v4525).sqrt()));
                    let v4531 = v4529 * v4529;
                    let v4545 = (v4281 * v4529).powf(v340);
                    let v4553 = ((v4529 + (v4248 * (v1 - ((v4278 * ((v4529 * v4279) / ((v4531 + v4291).sqrt()))).ln())))) - (v4305 * v4545)) / ((v4529 * (v1 + (v4248 / ((v4529 * v4280) / ((v4531 + v4296).sqrt()))))) + (v4314 * v4545));
                    let v4554 = v4522 / v4318;
                    let v4555 = if v4554 < v354 { 1.0 } else { 0.0 };
                    let v4589: f64;
                    if v4555 != 0.0 {
                        let v4577 = ((v4318 * v4281) * (((v110 * v4554) / v119) + (((rspice_limited_exp((v4554 / v119))) + (rspice_limited_exp(((v4558 * v4554) / v119)))).ln()))) / ((v1 / v4553) + ((v4281 / v287) * (rspice_limited_exp(((v4571 * v4522) / v4318)))));
                        v4589 = v4577;
                    } else {
                        let v4588 = ((v4318 * v4281) * v4554) / ((v1 / v4553) + ((v4281 / v287) * (rspice_limited_exp(((v4582 * v4522) / v4318)))));
                        v4589 = v4588;
                    }
                    let v4591 = v4522 - (v4589 / v4281);
                    let v4594 = if ((v4591 - v4522).abs()) > v394 { 1.0 } else { 0.0 };
                    if v4594 != 0.0 {
                        let v4595 = v4522 - v4591;
                        let v4602 = (v53 * v4595) + (v53 * (((v4595 * v4595) + v4598).sqrt()));
                        let v4603 = v4281.powf(v340);
                        let v4604 = v4602.powf(v340);
                        let v4606 = v4602.powf(v4605);
                        let v4607 = v3413 * v4603;
                        let v4609 = v3483 * v4603;
                        let v4611 = v4591 / v4248;
                        let v4613 = v4611 - ((v4607 * v4604) / v4248);
                        let v4615 = v4611 - ((v4609 * v4604) / v4248);
                        let v4616 = if v4613 >= v418 { 1.0 } else { 0.0 };
                        let v4630: f64;
                        if v4616 != 0.0 {
                            v4630 = v4613;
                        } else {
                            let v4618 = if v4613 <= v4617 { 1.0 } else { 0.0 };
                            let v4631: f64;
                            if v4618 != 0.0 {
                                v4631 = v0;
                            } else {
                                let v4621 = ((v4613.exp()) + v1).ln();
                                v4631 = v4621;
                            }
                            v4630 = v4631;
                        }
                        let v4622 = if v4615 >= v418 { 1.0 } else { 0.0 };
                        let v4634: f64;
                        if v4622 != 0.0 {
                            v4634 = v4615;
                        } else {
                            let v4624 = if v4615 <= v4623 { 1.0 } else { 0.0 };
                            let v4635: f64;
                            if v4624 != 0.0 {
                                v4635 = v0;
                            } else {
                                let v4627 = ((v4615.exp()) + v1).ln();
                                v4635 = v4627;
                            }
                            v4634 = v4635;
                        }
                        let v4629 = v287 * v4248;
                        let v4640 = rspice_limited_exp(v4613);
                        let v4646 = rspice_limited_exp(v4615);
                        let v4659 = v4591 - ((((v4281 * v4602) - (v4629 * v4630)) - (v4629 * v4634)) / (((v4652 * v4281) - (((v4640 * v287) * (v1 + (v340 * (v4607 * v4606)))) / (v1 + v4640))) - (((v4646 * v287) * (v1 + (v340 * (v4609 * v4606)))) / (v1 + v4646))));
                        let v4660 = v4522 - v4659;
                        let v4668 = ((v53 * v4660) + (v53 * (((v4660 * v4660) + v4663).sqrt()))).powf(v340);
                        let v4671 = v4659 / v4248;
                        let v4673 = v4671 - ((v4607 * v4668) / v4248);
                        let v4675 = v4671 - ((v4609 * v4668) / v4248);
                        let v4676 = if v4673 >= v418 { 1.0 } else { 0.0 };
                        if v4676 != 0.0 {
                        } else {
                            let v4678 = if v4673 <= v4677 { 1.0 } else { 0.0 };
                            if v4678 != 0.0 {
                            } else {
                            }
                        }
                        let v4679 = if v4675 >= v418 { 1.0 } else { 0.0 };
                        if v4679 != 0.0 {
                        } else {
                            let v4681 = if v4675 <= v4680 { 1.0 } else { 0.0 };
                            if v4681 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v4683 = if v4682 < v0 { 1.0 } else { 0.0 };
                    if v4683 != 0.0 {
                    } else {
                    }
                    v8833 = v4248;
                } else {
                    v8833 = v8834;
                }
                v8832 = v8833;
            } else {
                let v4684 = if v4226 != v0 { 1.0 } else { 0.0 };
                let v8843: f64;
                if v4684 != 0.0 {
                    let v4685 = if v4226 == v1 { 1.0 } else { 0.0 };
                    let v4687: f64;
                    if v4685 != 0.0 {
                        v4687 = v20;
                    } else {
                        let v4686 = v35 - v17;
                        v4687 = v4686;
                    }
                    let v4689 = v47 * (v1 + v3344);
                    let v4691 = v3350 + (v264 * v3351);
                    let v4692 = v217 / v3364;
                    let v4699 = v4691 + (v4689 * (((v3366 / ((v288 * v4689) * v4689)) * v3370).ln()));
                    let v4700 = v4687 - v4699;
                    let v4707 = ((v53 * (v4700 + (((v4700 * v4700) + v300).sqrt()))) + v4699) - v4691;
                    let v4710 = v4692 / (v4708 * v4689);
                    let v4711 = v310 / v4710;
                    let v4712 = v1 / v4710;
                    let v4713 = v4692 / v285;
                    let v4714 = v53 * v4707;
                    let v4715 = v4707 * v4707;
                    let v4720 = v4714 + (v53 * ((v4715 + v4716).sqrt()));
                    let v4722 = v4720 * v4720;
                    let v4723 = v4711 * v4711;
                    let v4728 = v4712 * v4712;
                    let v4737 = v3413 / v110;
                    let v4739 = (v4713 * v4720).powf(v340);
                    let v4746 = (v93 * v3413) / v110;
                    let v4749 = ((v4720 + (v4689 * (v1 - ((v4710 * ((v4720 * v4711) / ((v4722 + v4723).sqrt()))).ln())))) - (v4737 * v4739)) / ((v4720 * (v1 + (v4689 / ((v4720 * v4712) / ((v4722 + v4728).sqrt()))))) + (v4746 * v4739));
                    let v4750 = v93 * v4689;
                    let v4751 = v4707 / v4750;
                    let v4752 = if v4751 < v354 { 1.0 } else { 0.0 };
                    let v4786: f64;
                    if v4752 != 0.0 {
                        let v4774 = ((v4750 * v4713) * (((v110 * v4751) / v119) + (((rspice_limited_exp((v4751 / v119))) + (rspice_limited_exp(((v4755 * v4751) / v119)))).ln()))) / ((v1 / v4749) + ((v4713 / v287) * (rspice_limited_exp(((v4768 * v4707) / v4750)))));
                        v4786 = v4774;
                    } else {
                        let v4785 = ((v4750 * v4713) * v4751) / ((v1 / v4749) + ((v4713 / v287) * (rspice_limited_exp(((v4779 * v4707) / v4750)))));
                        v4786 = v4785;
                    }
                    let v4788 = v4707 - (v4786 / v4713);
                    let v4791 = if ((v4788 - v4707).abs()) > v394 { 1.0 } else { 0.0 };
                    let v4921: f64;
                    if v4791 != 0.0 {
                        let v4792 = v4707 - v4788;
                        let v4799 = (v53 * v4792) + (v53 * (((v4792 * v4792) + v4795).sqrt()));
                        let v4800 = v4713.powf(v340);
                        let v4801 = v4799.powf(v340);
                        let v4803 = v4799.powf(v4802);
                        let v4804 = v3413 * v4800;
                        let v4806 = v3483 * v4800;
                        let v4808 = v4788 / v4689;
                        let v4810 = v4808 - ((v4804 * v4801) / v4689);
                        let v4812 = v4808 - ((v4806 * v4801) / v4689);
                        let v4813 = if v4810 >= v418 { 1.0 } else { 0.0 };
                        let v4827: f64;
                        if v4813 != 0.0 {
                            v4827 = v4810;
                        } else {
                            let v4815 = if v4810 <= v4814 { 1.0 } else { 0.0 };
                            let v4828: f64;
                            if v4815 != 0.0 {
                                v4828 = v0;
                            } else {
                                let v4818 = ((v4810.exp()) + v1).ln();
                                v4828 = v4818;
                            }
                            v4827 = v4828;
                        }
                        let v4819 = if v4812 >= v418 { 1.0 } else { 0.0 };
                        let v4831: f64;
                        if v4819 != 0.0 {
                            v4831 = v4812;
                        } else {
                            let v4821 = if v4812 <= v4820 { 1.0 } else { 0.0 };
                            let v4832: f64;
                            if v4821 != 0.0 {
                                v4832 = v0;
                            } else {
                                let v4824 = ((v4812.exp()) + v1).ln();
                                v4832 = v4824;
                            }
                            v4831 = v4832;
                        }
                        let v4826 = v287 * v4689;
                        let v4837 = rspice_limited_exp(v4810);
                        let v4843 = rspice_limited_exp(v4812);
                        let v4856 = v4788 - ((((v4713 * v4799) - (v4826 * v4827)) - (v4826 * v4831)) / (((v4849 * v4713) - (((v4837 * v287) * (v1 + (v340 * (v4804 * v4803)))) / (v1 + v4837))) - (((v4843 * v287) * (v1 + (v340 * (v4806 * v4803)))) / (v1 + v4843))));
                        let v4857 = v4707 - v4856;
                        let v4864 = (v53 * v4857) + (v53 * (((v4857 * v4857) + v4860).sqrt()));
                        let v4866 = v4864.powf(v4865);
                        let v4867 = v4864.powf(v340);
                        let v4870 = v4856 / v4689;
                        let v4872 = v4870 - ((v4804 * v4867) / v4689);
                        let v4874 = v4870 - ((v4806 * v4867) / v4689);
                        let v4875 = if v4872 >= v418 { 1.0 } else { 0.0 };
                        let v4888: f64;
                        if v4875 != 0.0 {
                            v4888 = v4872;
                        } else {
                            let v4877 = if v4872 <= v4876 { 1.0 } else { 0.0 };
                            let v4889: f64;
                            if v4877 != 0.0 {
                                v4889 = v0;
                            } else {
                                let v4880 = ((v4872.exp()) + v1).ln();
                                v4889 = v4880;
                            }
                            v4888 = v4889;
                        }
                        let v4881 = if v4874 >= v418 { 1.0 } else { 0.0 };
                        let v4892: f64;
                        if v4881 != 0.0 {
                            v4892 = v4874;
                        } else {
                            let v4883 = if v4874 <= v4882 { 1.0 } else { 0.0 };
                            let v4893: f64;
                            if v4883 != 0.0 {
                                v4893 = v0;
                            } else {
                                let v4886 = ((v4874.exp()) + v1).ln();
                                v4893 = v4886;
                            }
                            v4892 = v4893;
                        }
                        let v4898 = rspice_limited_exp(v4872);
                        let v4904 = rspice_limited_exp(v4874);
                        let v4917 = v4856 - ((((v4713 * v4864) - (v4826 * v4888)) - (v4826 * v4892)) / (((v4910 * v4713) - (((v4898 * v287) * (v1 + (v340 * (v4804 * v4866)))) / (v1 + v4898))) - (((v4904 * v287) * (v1 + (v340 * (v4806 * v4866)))) / (v1 + v4904))));
                        v4921 = v4917;
                    } else {
                        v4921 = v4788;
                    }
                    let v4924 = (v4692 / v217) * ((v4707 - v4921).abs());
                    let v4942 = v4714 + (v53 * ((v4715 + v4938).sqrt()));
                    let v4943 = ((v93 * (v3598 * v540)) / ((v3596 * v537) / (((v1 + (v551 * v4924)) + (v554 * (v4924 * v4924))) + (v558 * (v547 * ((v279 - v4921).abs())))))) * v3366;
                    let v4954 = v4707 - (v0 * ((v1 + ((v0 / ((v4943 * v4942) / (v4943 + v4942))).powf(v574))).powf((v4950 / v574))));
                    let v4961 = (v53 * v4954) + (v53 * (((v4954 * v4954) + v4957).sqrt()));
                    let v4963 = v4961 * v4961;
                    let v4977 = (v4713 * v4961).powf(v340);
                    let v4985 = ((v4961 + (v4689 * (v1 - ((v4710 * ((v4961 * v4711) / ((v4963 + v4723).sqrt()))).ln())))) - (v4737 * v4977)) / ((v4961 * (v1 + (v4689 / ((v4961 * v4712) / ((v4963 + v4728).sqrt()))))) + (v4746 * v4977));
                    let v4986 = v4954 / v4750;
                    let v4987 = if v4986 < v354 { 1.0 } else { 0.0 };
                    let v5021: f64;
                    if v4987 != 0.0 {
                        let v5009 = ((v4750 * v4713) * (((v110 * v4986) / v119) + (((rspice_limited_exp((v4986 / v119))) + (rspice_limited_exp(((v4990 * v4986) / v119)))).ln()))) / ((v1 / v4985) + ((v4713 / v287) * (rspice_limited_exp(((v5003 * v4954) / v4750)))));
                        v5021 = v5009;
                    } else {
                        let v5020 = ((v4750 * v4713) * v4986) / ((v1 / v4985) + ((v4713 / v287) * (rspice_limited_exp(((v5014 * v4954) / v4750)))));
                        v5021 = v5020;
                    }
                    let v5023 = v4954 - (v5021 / v4713);
                    let v5026 = if ((v5023 - v4954).abs()) > v394 { 1.0 } else { 0.0 };
                    if v5026 != 0.0 {
                        let v5027 = v4954 - v5023;
                        let v5034 = (v53 * v5027) + (v53 * (((v5027 * v5027) + v5030).sqrt()));
                        let v5035 = v4713.powf(v340);
                        let v5036 = v5034.powf(v340);
                        let v5038 = v5034.powf(v5037);
                        let v5039 = v3413 * v5035;
                        let v5041 = v3483 * v5035;
                        let v5043 = v5023 / v4689;
                        let v5045 = v5043 - ((v5039 * v5036) / v4689);
                        let v5047 = v5043 - ((v5041 * v5036) / v4689);
                        let v5048 = if v5045 >= v418 { 1.0 } else { 0.0 };
                        let v5062: f64;
                        if v5048 != 0.0 {
                            v5062 = v5045;
                        } else {
                            let v5050 = if v5045 <= v5049 { 1.0 } else { 0.0 };
                            let v5063: f64;
                            if v5050 != 0.0 {
                                v5063 = v0;
                            } else {
                                let v5053 = ((v5045.exp()) + v1).ln();
                                v5063 = v5053;
                            }
                            v5062 = v5063;
                        }
                        let v5054 = if v5047 >= v418 { 1.0 } else { 0.0 };
                        let v5066: f64;
                        if v5054 != 0.0 {
                            v5066 = v5047;
                        } else {
                            let v5056 = if v5047 <= v5055 { 1.0 } else { 0.0 };
                            let v5067: f64;
                            if v5056 != 0.0 {
                                v5067 = v0;
                            } else {
                                let v5059 = ((v5047.exp()) + v1).ln();
                                v5067 = v5059;
                            }
                            v5066 = v5067;
                        }
                        let v5061 = v287 * v4689;
                        let v5072 = rspice_limited_exp(v5045);
                        let v5078 = rspice_limited_exp(v5047);
                        let v5091 = v5023 - ((((v4713 * v5034) - (v5061 * v5062)) - (v5061 * v5066)) / (((v5084 * v4713) - (((v5072 * v287) * (v1 + (v340 * (v5039 * v5038)))) / (v1 + v5072))) - (((v5078 * v287) * (v1 + (v340 * (v5041 * v5038)))) / (v1 + v5078))));
                        let v5092 = v4954 - v5091;
                        let v5100 = ((v53 * v5092) + (v53 * (((v5092 * v5092) + v5095).sqrt()))).powf(v340);
                        let v5103 = v5091 / v4689;
                        let v5105 = v5103 - ((v5039 * v5100) / v4689);
                        let v5107 = v5103 - ((v5041 * v5100) / v4689);
                        let v5108 = if v5105 >= v418 { 1.0 } else { 0.0 };
                        if v5108 != 0.0 {
                        } else {
                            let v5110 = if v5105 <= v5109 { 1.0 } else { 0.0 };
                            if v5110 != 0.0 {
                            } else {
                            }
                        }
                        let v5111 = if v5107 >= v418 { 1.0 } else { 0.0 };
                        if v5111 != 0.0 {
                        } else {
                            let v5113 = if v5107 <= v5112 { 1.0 } else { 0.0 };
                            if v5113 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    v8843 = v4689;
                } else {
                    v8843 = v8834;
                }
                v8832 = v8843;
            }
            let v8830: f64;
            if v1539 != 0.0 {
                let v5115 = if v5114 != v0 { 1.0 } else { 0.0 };
                let v8831: f64;
                if v5115 != 0.0 {
                    let v5117 = v5116 - v3327;
                    let v5118 = if v5114 == v1 { 1.0 } else { 0.0 };
                    let v5126: f64;
                    let v5127: f64;
                    if v5118 != 0.0 {
                        let v5119 = v19 - v3327;
                        let v5120 = v19 - v5116;
                        v5126 = v5120;
                        v5127 = v5119;
                    } else {
                        let v5121 = v35 - v3327;
                        let v5122 = v35 - v5116;
                        v5126 = v5122;
                        v5127 = v5121;
                    }
                    let v5123 = if v5117 < v0 { 1.0 } else { 0.0 };
                    let v5128: f64;
                    let v5164: f64;
                    let v5583: f64;
                    if v5123 != 0.0 {
                        let v5125 = v5124 * v5117;
                        v5128 = v5125;
                        v5164 = v5126;
                        v5583 = v5124;
                    } else {
                        v5128 = v5117;
                        v5164 = v5127;
                        v5583 = v1;
                    }
                    let v5132 = (((v5128 * v5128) + v2).sqrt()) - v32;
                    let v5138 = v47 * ((v1 + v5133) + (v5135 * v5132));
                    let v5152 = (v5139 - (v264 * v5140)) - ((v5143 * (v5132 * v5144)) / (((v5132 * v5132) + (v5144 * v5144)).sqrt()));
                    let v5154 = v217 / v5153;
                    let v5163 = v5152 + (v5138 * (((v5155 / ((v288 * v5138) * v5138)) * v5159).ln()));
                    let v5165 = v5164 - v5163;
                    let v5172 = ((v53 * (v5165 + (((v5165 * v5165) + v300).sqrt()))) + v5163) - v5152;
                    let v5175 = v5154 / (v5173 * v5138);
                    let v5176 = v310 / v5175;
                    let v5177 = v1 / v5175;
                    let v5178 = v5154 / v285;
                    let v5179 = v53 * v5172;
                    let v5180 = v5172 * v5172;
                    let v5185 = v5179 + (v53 * ((v5180 + v5181).sqrt()));
                    let v5187 = v5185 * v5185;
                    let v5188 = v5176 * v5176;
                    let v5193 = v5177 * v5177;
                    let v5203 = v5202 / v110;
                    let v5205 = (v5178 * v5185).powf(v340);
                    let v5212 = (v93 * v5202) / v110;
                    let v5215 = ((v5185 + (v5138 * (v1 - ((v5175 * ((v5185 * v5176) / ((v5187 + v5188).sqrt()))).ln())))) - (v5203 * v5205)) / ((v5185 * (v1 + (v5138 / ((v5185 * v5177) / ((v5187 + v5193).sqrt()))))) + (v5212 * v5205));
                    let v5216 = v93 * v5138;
                    let v5217 = v5172 / v5216;
                    let v5218 = if v5217 < v354 { 1.0 } else { 0.0 };
                    let v5252: f64;
                    if v5218 != 0.0 {
                        let v5240 = ((v5216 * v5178) * (((v110 * v5217) / v119) + (((rspice_limited_exp((v5217 / v119))) + (rspice_limited_exp(((v5221 * v5217) / v119)))).ln()))) / ((v1 / v5215) + ((v5178 / v287) * (rspice_limited_exp(((v5234 * v5172) / v5216)))));
                        v5252 = v5240;
                    } else {
                        let v5251 = ((v5216 * v5178) * v5217) / ((v1 / v5215) + ((v5178 / v287) * (rspice_limited_exp(((v5245 * v5172) / v5216)))));
                        v5252 = v5251;
                    }
                    let v5254 = v5172 - (v5252 / v5178);
                    let v5257 = if ((v5254 - v5172).abs()) > v394 { 1.0 } else { 0.0 };
                    let v5390: f64;
                    if v5257 != 0.0 {
                        let v5258 = v5172 - v5254;
                        let v5265 = (v53 * v5258) + (v53 * (((v5258 * v5258) + v5261).sqrt()));
                        let v5266 = v5178.powf(v340);
                        let v5267 = v5265.powf(v340);
                        let v5269 = v5265.powf(v5268);
                        let v5270 = v5202 * v5266;
                        let v5273 = v5272 * v5266;
                        let v5275 = v5254 / v5138;
                        let v5277 = v5275 - ((v5270 * v5267) / v5138);
                        let v5279 = v5275 - ((v5273 * v5267) / v5138);
                        let v5280 = if v5277 >= v418 { 1.0 } else { 0.0 };
                        let v5294: f64;
                        if v5280 != 0.0 {
                            v5294 = v5277;
                        } else {
                            let v5282 = if v5277 <= v5281 { 1.0 } else { 0.0 };
                            let v5295: f64;
                            if v5282 != 0.0 {
                                v5295 = v0;
                            } else {
                                let v5285 = ((v5277.exp()) + v1).ln();
                                v5295 = v5285;
                            }
                            v5294 = v5295;
                        }
                        let v5286 = if v5279 >= v418 { 1.0 } else { 0.0 };
                        let v5298: f64;
                        if v5286 != 0.0 {
                            v5298 = v5279;
                        } else {
                            let v5288 = if v5279 <= v5287 { 1.0 } else { 0.0 };
                            let v5299: f64;
                            if v5288 != 0.0 {
                                v5299 = v0;
                            } else {
                                let v5291 = ((v5279.exp()) + v1).ln();
                                v5299 = v5291;
                            }
                            v5298 = v5299;
                        }
                        let v5293 = v287 * v5138;
                        let v5304 = rspice_limited_exp(v5277);
                        let v5310 = rspice_limited_exp(v5279);
                        let v5323 = v5254 - ((((v5178 * v5265) - (v5293 * v5294)) - (v5293 * v5298)) / (((v5316 * v5178) - (((v5304 * v287) * (v1 + (v340 * (v5270 * v5269)))) / (v1 + v5304))) - (((v5310 * v287) * (v1 + (v340 * (v5273 * v5269)))) / (v1 + v5310))));
                        let v5324 = v5172 - v5323;
                        let v5331 = (v53 * v5324) + (v53 * (((v5324 * v5324) + v5327).sqrt()));
                        let v5333 = v5331.powf(v5332);
                        let v5334 = v5331.powf(v340);
                        let v5337 = v5323 / v5138;
                        let v5339 = v5337 - ((v5270 * v5334) / v5138);
                        let v5341 = v5337 - ((v5273 * v5334) / v5138);
                        let v5342 = if v5339 >= v418 { 1.0 } else { 0.0 };
                        let v5355: f64;
                        if v5342 != 0.0 {
                            v5355 = v5339;
                        } else {
                            let v5344 = if v5339 <= v5343 { 1.0 } else { 0.0 };
                            let v5356: f64;
                            if v5344 != 0.0 {
                                v5356 = v0;
                            } else {
                                let v5347 = ((v5339.exp()) + v1).ln();
                                v5356 = v5347;
                            }
                            v5355 = v5356;
                        }
                        let v5348 = if v5341 >= v418 { 1.0 } else { 0.0 };
                        let v5359: f64;
                        if v5348 != 0.0 {
                            v5359 = v5341;
                        } else {
                            let v5350 = if v5341 <= v5349 { 1.0 } else { 0.0 };
                            let v5360: f64;
                            if v5350 != 0.0 {
                                v5360 = v0;
                            } else {
                                let v5353 = ((v5341.exp()) + v1).ln();
                                v5360 = v5353;
                            }
                            v5359 = v5360;
                        }
                        let v5365 = rspice_limited_exp(v5339);
                        let v5371 = rspice_limited_exp(v5341);
                        let v5384 = v5323 - ((((v5178 * v5331) - (v5293 * v5355)) - (v5293 * v5359)) / (((v5377 * v5178) - (((v5365 * v287) * (v1 + (v340 * (v5270 * v5333)))) / (v1 + v5365))) - (((v5371 * v287) * (v1 + (v340 * (v5273 * v5333)))) / (v1 + v5371))));
                        v5390 = v5384;
                    } else {
                        v5390 = v5254;
                    }
                    let v5393 = (v5154 / v217) * ((v5172 - v5390).abs());
                    let v5411 = v5179 + (v53 * ((v5180 + v5407).sqrt()));
                    let v5412 = ((v93 * (v5387 * v540)) / ((v5385 * v537) / (((v1 + (v551 * v5393)) + (v554 * (v5393 * v5393))) + (v558 * (v547 * ((v279 - v5390).abs())))))) * v5155;
                    let v5423 = v5172 - (v5128 * ((v1 + ((v5128 / ((v5412 * v5411) / (v5412 + v5411))).powf(v574))).powf((v5419 / v574))));
                    let v5430 = (v53 * v5423) + (v53 * (((v5423 * v5423) + v5426).sqrt()));
                    let v5432 = v5430 * v5430;
                    let v5446 = (v5178 * v5430).powf(v340);
                    let v5454 = ((v5430 + (v5138 * (v1 - ((v5175 * ((v5430 * v5176) / ((v5432 + v5188).sqrt()))).ln())))) - (v5203 * v5446)) / ((v5430 * (v1 + (v5138 / ((v5430 * v5177) / ((v5432 + v5193).sqrt()))))) + (v5212 * v5446));
                    let v5455 = v5423 / v5216;
                    let v5456 = if v5455 < v354 { 1.0 } else { 0.0 };
                    let v5490: f64;
                    if v5456 != 0.0 {
                        let v5478 = ((v5216 * v5178) * (((v110 * v5455) / v119) + (((rspice_limited_exp((v5455 / v119))) + (rspice_limited_exp(((v5459 * v5455) / v119)))).ln()))) / ((v1 / v5454) + ((v5178 / v287) * (rspice_limited_exp(((v5472 * v5423) / v5216)))));
                        v5490 = v5478;
                    } else {
                        let v5489 = ((v5216 * v5178) * v5455) / ((v1 / v5454) + ((v5178 / v287) * (rspice_limited_exp(((v5483 * v5423) / v5216)))));
                        v5490 = v5489;
                    }
                    let v5492 = v5423 - (v5490 / v5178);
                    let v5495 = if ((v5492 - v5423).abs()) > v394 { 1.0 } else { 0.0 };
                    if v5495 != 0.0 {
                        let v5496 = v5423 - v5492;
                        let v5503 = (v53 * v5496) + (v53 * (((v5496 * v5496) + v5499).sqrt()));
                        let v5504 = v5178.powf(v340);
                        let v5505 = v5503.powf(v340);
                        let v5507 = v5503.powf(v5506);
                        let v5508 = v5202 * v5504;
                        let v5510 = v5272 * v5504;
                        let v5512 = v5492 / v5138;
                        let v5514 = v5512 - ((v5508 * v5505) / v5138);
                        let v5516 = v5512 - ((v5510 * v5505) / v5138);
                        let v5517 = if v5514 >= v418 { 1.0 } else { 0.0 };
                        let v5531: f64;
                        if v5517 != 0.0 {
                            v5531 = v5514;
                        } else {
                            let v5519 = if v5514 <= v5518 { 1.0 } else { 0.0 };
                            let v5532: f64;
                            if v5519 != 0.0 {
                                v5532 = v0;
                            } else {
                                let v5522 = ((v5514.exp()) + v1).ln();
                                v5532 = v5522;
                            }
                            v5531 = v5532;
                        }
                        let v5523 = if v5516 >= v418 { 1.0 } else { 0.0 };
                        let v5535: f64;
                        if v5523 != 0.0 {
                            v5535 = v5516;
                        } else {
                            let v5525 = if v5516 <= v5524 { 1.0 } else { 0.0 };
                            let v5536: f64;
                            if v5525 != 0.0 {
                                v5536 = v0;
                            } else {
                                let v5528 = ((v5516.exp()) + v1).ln();
                                v5536 = v5528;
                            }
                            v5535 = v5536;
                        }
                        let v5530 = v287 * v5138;
                        let v5541 = rspice_limited_exp(v5514);
                        let v5547 = rspice_limited_exp(v5516);
                        let v5560 = v5492 - ((((v5178 * v5503) - (v5530 * v5531)) - (v5530 * v5535)) / (((v5553 * v5178) - (((v5541 * v287) * (v1 + (v340 * (v5508 * v5507)))) / (v1 + v5541))) - (((v5547 * v287) * (v1 + (v340 * (v5510 * v5507)))) / (v1 + v5547))));
                        let v5561 = v5423 - v5560;
                        let v5569 = ((v53 * v5561) + (v53 * (((v5561 * v5561) + v5564).sqrt()))).powf(v340);
                        let v5572 = v5560 / v5138;
                        let v5574 = v5572 - ((v5508 * v5569) / v5138);
                        let v5576 = v5572 - ((v5510 * v5569) / v5138);
                        let v5577 = if v5574 >= v418 { 1.0 } else { 0.0 };
                        if v5577 != 0.0 {
                        } else {
                            let v5579 = if v5574 <= v5578 { 1.0 } else { 0.0 };
                            if v5579 != 0.0 {
                            } else {
                            }
                        }
                        let v5580 = if v5576 >= v418 { 1.0 } else { 0.0 };
                        if v5580 != 0.0 {
                        } else {
                            let v5582 = if v5576 <= v5581 { 1.0 } else { 0.0 };
                            if v5582 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v5584 = if v5583 < v0 { 1.0 } else { 0.0 };
                    if v5584 != 0.0 {
                    } else {
                    }
                    v8831 = v5138;
                } else {
                    v8831 = v8832;
                }
                v8830 = v8831;
            } else {
                let v5585 = if v5114 != v0 { 1.0 } else { 0.0 };
                let v8844: f64;
                if v5585 != 0.0 {
                    let v5586 = if v5114 == v1 { 1.0 } else { 0.0 };
                    let v5588: f64;
                    if v5586 != 0.0 {
                        v5588 = v21;
                    } else {
                        let v5587 = v35 - v16;
                        v5588 = v5587;
                    }
                    let v5590 = v47 * (v1 + v5133);
                    let v5592 = v5139 - (v264 * v5140);
                    let v5593 = v217 / v5153;
                    let v5600 = v5592 + (v5590 * (((v5155 / ((v288 * v5590) * v5590)) * v5159).ln()));
                    let v5601 = v5588 - v5600;
                    let v5608 = ((v53 * (v5601 + (((v5601 * v5601) + v300).sqrt()))) + v5600) - v5592;
                    let v5611 = v5593 / (v5609 * v5590);
                    let v5612 = v310 / v5611;
                    let v5613 = v1 / v5611;
                    let v5614 = v5593 / v285;
                    let v5615 = v53 * v5608;
                    let v5616 = v5608 * v5608;
                    let v5621 = v5615 + (v53 * ((v5616 + v5617).sqrt()));
                    let v5623 = v5621 * v5621;
                    let v5624 = v5612 * v5612;
                    let v5629 = v5613 * v5613;
                    let v5638 = v5202 / v110;
                    let v5640 = (v5614 * v5621).powf(v340);
                    let v5647 = (v93 * v5202) / v110;
                    let v5650 = ((v5621 + (v5590 * (v1 - ((v5611 * ((v5621 * v5612) / ((v5623 + v5624).sqrt()))).ln())))) - (v5638 * v5640)) / ((v5621 * (v1 + (v5590 / ((v5621 * v5613) / ((v5623 + v5629).sqrt()))))) + (v5647 * v5640));
                    let v5651 = v93 * v5590;
                    let v5652 = v5608 / v5651;
                    let v5653 = if v5652 < v354 { 1.0 } else { 0.0 };
                    let v5687: f64;
                    if v5653 != 0.0 {
                        let v5675 = ((v5651 * v5614) * (((v110 * v5652) / v119) + (((rspice_limited_exp((v5652 / v119))) + (rspice_limited_exp(((v5656 * v5652) / v119)))).ln()))) / ((v1 / v5650) + ((v5614 / v287) * (rspice_limited_exp(((v5669 * v5608) / v5651)))));
                        v5687 = v5675;
                    } else {
                        let v5686 = ((v5651 * v5614) * v5652) / ((v1 / v5650) + ((v5614 / v287) * (rspice_limited_exp(((v5680 * v5608) / v5651)))));
                        v5687 = v5686;
                    }
                    let v5689 = v5608 - (v5687 / v5614);
                    let v5692 = if ((v5689 - v5608).abs()) > v394 { 1.0 } else { 0.0 };
                    let v5822: f64;
                    if v5692 != 0.0 {
                        let v5693 = v5608 - v5689;
                        let v5700 = (v53 * v5693) + (v53 * (((v5693 * v5693) + v5696).sqrt()));
                        let v5701 = v5614.powf(v340);
                        let v5702 = v5700.powf(v340);
                        let v5704 = v5700.powf(v5703);
                        let v5705 = v5202 * v5701;
                        let v5707 = v5272 * v5701;
                        let v5709 = v5689 / v5590;
                        let v5711 = v5709 - ((v5705 * v5702) / v5590);
                        let v5713 = v5709 - ((v5707 * v5702) / v5590);
                        let v5714 = if v5711 >= v418 { 1.0 } else { 0.0 };
                        let v5728: f64;
                        if v5714 != 0.0 {
                            v5728 = v5711;
                        } else {
                            let v5716 = if v5711 <= v5715 { 1.0 } else { 0.0 };
                            let v5729: f64;
                            if v5716 != 0.0 {
                                v5729 = v0;
                            } else {
                                let v5719 = ((v5711.exp()) + v1).ln();
                                v5729 = v5719;
                            }
                            v5728 = v5729;
                        }
                        let v5720 = if v5713 >= v418 { 1.0 } else { 0.0 };
                        let v5732: f64;
                        if v5720 != 0.0 {
                            v5732 = v5713;
                        } else {
                            let v5722 = if v5713 <= v5721 { 1.0 } else { 0.0 };
                            let v5733: f64;
                            if v5722 != 0.0 {
                                v5733 = v0;
                            } else {
                                let v5725 = ((v5713.exp()) + v1).ln();
                                v5733 = v5725;
                            }
                            v5732 = v5733;
                        }
                        let v5727 = v287 * v5590;
                        let v5738 = rspice_limited_exp(v5711);
                        let v5744 = rspice_limited_exp(v5713);
                        let v5757 = v5689 - ((((v5614 * v5700) - (v5727 * v5728)) - (v5727 * v5732)) / (((v5750 * v5614) - (((v5738 * v287) * (v1 + (v340 * (v5705 * v5704)))) / (v1 + v5738))) - (((v5744 * v287) * (v1 + (v340 * (v5707 * v5704)))) / (v1 + v5744))));
                        let v5758 = v5608 - v5757;
                        let v5765 = (v53 * v5758) + (v53 * (((v5758 * v5758) + v5761).sqrt()));
                        let v5767 = v5765.powf(v5766);
                        let v5768 = v5765.powf(v340);
                        let v5771 = v5757 / v5590;
                        let v5773 = v5771 - ((v5705 * v5768) / v5590);
                        let v5775 = v5771 - ((v5707 * v5768) / v5590);
                        let v5776 = if v5773 >= v418 { 1.0 } else { 0.0 };
                        let v5789: f64;
                        if v5776 != 0.0 {
                            v5789 = v5773;
                        } else {
                            let v5778 = if v5773 <= v5777 { 1.0 } else { 0.0 };
                            let v5790: f64;
                            if v5778 != 0.0 {
                                v5790 = v0;
                            } else {
                                let v5781 = ((v5773.exp()) + v1).ln();
                                v5790 = v5781;
                            }
                            v5789 = v5790;
                        }
                        let v5782 = if v5775 >= v418 { 1.0 } else { 0.0 };
                        let v5793: f64;
                        if v5782 != 0.0 {
                            v5793 = v5775;
                        } else {
                            let v5784 = if v5775 <= v5783 { 1.0 } else { 0.0 };
                            let v5794: f64;
                            if v5784 != 0.0 {
                                v5794 = v0;
                            } else {
                                let v5787 = ((v5775.exp()) + v1).ln();
                                v5794 = v5787;
                            }
                            v5793 = v5794;
                        }
                        let v5799 = rspice_limited_exp(v5773);
                        let v5805 = rspice_limited_exp(v5775);
                        let v5818 = v5757 - ((((v5614 * v5765) - (v5727 * v5789)) - (v5727 * v5793)) / (((v5811 * v5614) - (((v5799 * v287) * (v1 + (v340 * (v5705 * v5767)))) / (v1 + v5799))) - (((v5805 * v287) * (v1 + (v340 * (v5707 * v5767)))) / (v1 + v5805))));
                        v5822 = v5818;
                    } else {
                        v5822 = v5689;
                    }
                    let v5825 = (v5593 / v217) * ((v5608 - v5822).abs());
                    let v5843 = v5615 + (v53 * ((v5616 + v5839).sqrt()));
                    let v5844 = ((v93 * (v5387 * v540)) / ((v5385 * v537) / (((v1 + (v551 * v5825)) + (v554 * (v5825 * v5825))) + (v558 * (v547 * ((v279 - v5822).abs())))))) * v5155;
                    let v5855 = v5608 - (v0 * ((v1 + ((v0 / ((v5844 * v5843) / (v5844 + v5843))).powf(v574))).powf((v5851 / v574))));
                    let v5862 = (v53 * v5855) + (v53 * (((v5855 * v5855) + v5858).sqrt()));
                    let v5864 = v5862 * v5862;
                    let v5878 = (v5614 * v5862).powf(v340);
                    let v5886 = ((v5862 + (v5590 * (v1 - ((v5611 * ((v5862 * v5612) / ((v5864 + v5624).sqrt()))).ln())))) - (v5638 * v5878)) / ((v5862 * (v1 + (v5590 / ((v5862 * v5613) / ((v5864 + v5629).sqrt()))))) + (v5647 * v5878));
                    let v5887 = v5855 / v5651;
                    let v5888 = if v5887 < v354 { 1.0 } else { 0.0 };
                    let v5922: f64;
                    if v5888 != 0.0 {
                        let v5910 = ((v5651 * v5614) * (((v110 * v5887) / v119) + (((rspice_limited_exp((v5887 / v119))) + (rspice_limited_exp(((v5891 * v5887) / v119)))).ln()))) / ((v1 / v5886) + ((v5614 / v287) * (rspice_limited_exp(((v5904 * v5855) / v5651)))));
                        v5922 = v5910;
                    } else {
                        let v5921 = ((v5651 * v5614) * v5887) / ((v1 / v5886) + ((v5614 / v287) * (rspice_limited_exp(((v5915 * v5855) / v5651)))));
                        v5922 = v5921;
                    }
                    let v5924 = v5855 - (v5922 / v5614);
                    let v5927 = if ((v5924 - v5855).abs()) > v394 { 1.0 } else { 0.0 };
                    if v5927 != 0.0 {
                        let v5928 = v5855 - v5924;
                        let v5935 = (v53 * v5928) + (v53 * (((v5928 * v5928) + v5931).sqrt()));
                        let v5936 = v5614.powf(v340);
                        let v5937 = v5935.powf(v340);
                        let v5939 = v5935.powf(v5938);
                        let v5940 = v5202 * v5936;
                        let v5942 = v5272 * v5936;
                        let v5944 = v5924 / v5590;
                        let v5946 = v5944 - ((v5940 * v5937) / v5590);
                        let v5948 = v5944 - ((v5942 * v5937) / v5590);
                        let v5949 = if v5946 >= v418 { 1.0 } else { 0.0 };
                        let v5963: f64;
                        if v5949 != 0.0 {
                            v5963 = v5946;
                        } else {
                            let v5951 = if v5946 <= v5950 { 1.0 } else { 0.0 };
                            let v5964: f64;
                            if v5951 != 0.0 {
                                v5964 = v0;
                            } else {
                                let v5954 = ((v5946.exp()) + v1).ln();
                                v5964 = v5954;
                            }
                            v5963 = v5964;
                        }
                        let v5955 = if v5948 >= v418 { 1.0 } else { 0.0 };
                        let v5967: f64;
                        if v5955 != 0.0 {
                            v5967 = v5948;
                        } else {
                            let v5957 = if v5948 <= v5956 { 1.0 } else { 0.0 };
                            let v5968: f64;
                            if v5957 != 0.0 {
                                v5968 = v0;
                            } else {
                                let v5960 = ((v5948.exp()) + v1).ln();
                                v5968 = v5960;
                            }
                            v5967 = v5968;
                        }
                        let v5962 = v287 * v5590;
                        let v5973 = rspice_limited_exp(v5946);
                        let v5979 = rspice_limited_exp(v5948);
                        let v5992 = v5924 - ((((v5614 * v5935) - (v5962 * v5963)) - (v5962 * v5967)) / (((v5985 * v5614) - (((v5973 * v287) * (v1 + (v340 * (v5940 * v5939)))) / (v1 + v5973))) - (((v5979 * v287) * (v1 + (v340 * (v5942 * v5939)))) / (v1 + v5979))));
                        let v5993 = v5855 - v5992;
                        let v6001 = ((v53 * v5993) + (v53 * (((v5993 * v5993) + v5996).sqrt()))).powf(v340);
                        let v6004 = v5992 / v5590;
                        let v6006 = v6004 - ((v5940 * v6001) / v5590);
                        let v6008 = v6004 - ((v5942 * v6001) / v5590);
                        let v6009 = if v6006 >= v418 { 1.0 } else { 0.0 };
                        if v6009 != 0.0 {
                        } else {
                            let v6011 = if v6006 <= v6010 { 1.0 } else { 0.0 };
                            if v6011 != 0.0 {
                            } else {
                            }
                        }
                        let v6012 = if v6008 >= v418 { 1.0 } else { 0.0 };
                        if v6012 != 0.0 {
                        } else {
                            let v6014 = if v6008 <= v6013 { 1.0 } else { 0.0 };
                            if v6014 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    v8844 = v5590;
                } else {
                    v8844 = v8832;
                }
                v8830 = v8844;
            }
            let v8828: f64;
            if v1539 != 0.0 {
                let v6016 = if v6015 != v0 { 1.0 } else { 0.0 };
                let v8829: f64;
                if v6016 != 0.0 {
                    let v6018 = v4228 - v6017;
                    let v6019 = if v6015 == v1 { 1.0 } else { 0.0 };
                    let v6027: f64;
                    let v6028: f64;
                    if v6019 != 0.0 {
                        let v6020 = v19 - v6017;
                        let v6021 = v19 - v4228;
                        v6027 = v6021;
                        v6028 = v6020;
                    } else {
                        let v6022 = v35 - v6017;
                        let v6023 = v35 - v4228;
                        v6027 = v6023;
                        v6028 = v6022;
                    }
                    let v6024 = if v6018 < v0 { 1.0 } else { 0.0 };
                    let v6029: f64;
                    let v6056: f64;
                    let v6471: f64;
                    if v6024 != 0.0 {
                        let v6026 = v6025 * v6018;
                        v6029 = v6026;
                        v6056 = v6027;
                        v6471 = v6025;
                    } else {
                        v6029 = v6018;
                        v6056 = v6028;
                        v6471 = v1;
                    }
                    let v6033 = (((v6029 * v6029) + v2).sqrt()) - v32;
                    let v6037 = v47 * ((v1 + v5133) + (v5135 * v6033));
                    let v6047 = (v5139 + (v264 * v5140)) - ((v5143 * (v6033 * v5144)) / (((v6033 * v6033) + (v5144 * v5144)).sqrt()));
                    let v6048 = v217 / v5153;
                    let v6055 = v6047 + (v6037 * (((v5155 / ((v288 * v6037) * v6037)) * v5159).ln()));
                    let v6057 = v6056 - v6055;
                    let v6064 = ((v53 * (v6057 + (((v6057 * v6057) + v300).sqrt()))) + v6055) - v6047;
                    let v6067 = v6048 / (v6065 * v6037);
                    let v6068 = v310 / v6067;
                    let v6069 = v1 / v6067;
                    let v6070 = v6048 / v285;
                    let v6071 = v53 * v6064;
                    let v6072 = v6064 * v6064;
                    let v6077 = v6071 + (v53 * ((v6072 + v6073).sqrt()));
                    let v6079 = v6077 * v6077;
                    let v6080 = v6068 * v6068;
                    let v6085 = v6069 * v6069;
                    let v6094 = v5202 / v110;
                    let v6096 = (v6070 * v6077).powf(v340);
                    let v6103 = (v93 * v5202) / v110;
                    let v6106 = ((v6077 + (v6037 * (v1 - ((v6067 * ((v6077 * v6068) / ((v6079 + v6080).sqrt()))).ln())))) - (v6094 * v6096)) / ((v6077 * (v1 + (v6037 / ((v6077 * v6069) / ((v6079 + v6085).sqrt()))))) + (v6103 * v6096));
                    let v6107 = v93 * v6037;
                    let v6108 = v6064 / v6107;
                    let v6109 = if v6108 < v354 { 1.0 } else { 0.0 };
                    let v6143: f64;
                    if v6109 != 0.0 {
                        let v6131 = ((v6107 * v6070) * (((v110 * v6108) / v119) + (((rspice_limited_exp((v6108 / v119))) + (rspice_limited_exp(((v6112 * v6108) / v119)))).ln()))) / ((v1 / v6106) + ((v6070 / v287) * (rspice_limited_exp(((v6125 * v6064) / v6107)))));
                        v6143 = v6131;
                    } else {
                        let v6142 = ((v6107 * v6070) * v6108) / ((v1 / v6106) + ((v6070 / v287) * (rspice_limited_exp(((v6136 * v6064) / v6107)))));
                        v6143 = v6142;
                    }
                    let v6145 = v6064 - (v6143 / v6070);
                    let v6148 = if ((v6145 - v6064).abs()) > v394 { 1.0 } else { 0.0 };
                    let v6278: f64;
                    if v6148 != 0.0 {
                        let v6149 = v6064 - v6145;
                        let v6156 = (v53 * v6149) + (v53 * (((v6149 * v6149) + v6152).sqrt()));
                        let v6157 = v6070.powf(v340);
                        let v6158 = v6156.powf(v340);
                        let v6160 = v6156.powf(v6159);
                        let v6161 = v5202 * v6157;
                        let v6163 = v5272 * v6157;
                        let v6165 = v6145 / v6037;
                        let v6167 = v6165 - ((v6161 * v6158) / v6037);
                        let v6169 = v6165 - ((v6163 * v6158) / v6037);
                        let v6170 = if v6167 >= v418 { 1.0 } else { 0.0 };
                        let v6184: f64;
                        if v6170 != 0.0 {
                            v6184 = v6167;
                        } else {
                            let v6172 = if v6167 <= v6171 { 1.0 } else { 0.0 };
                            let v6185: f64;
                            if v6172 != 0.0 {
                                v6185 = v0;
                            } else {
                                let v6175 = ((v6167.exp()) + v1).ln();
                                v6185 = v6175;
                            }
                            v6184 = v6185;
                        }
                        let v6176 = if v6169 >= v418 { 1.0 } else { 0.0 };
                        let v6188: f64;
                        if v6176 != 0.0 {
                            v6188 = v6169;
                        } else {
                            let v6178 = if v6169 <= v6177 { 1.0 } else { 0.0 };
                            let v6189: f64;
                            if v6178 != 0.0 {
                                v6189 = v0;
                            } else {
                                let v6181 = ((v6169.exp()) + v1).ln();
                                v6189 = v6181;
                            }
                            v6188 = v6189;
                        }
                        let v6183 = v287 * v6037;
                        let v6194 = rspice_limited_exp(v6167);
                        let v6200 = rspice_limited_exp(v6169);
                        let v6213 = v6145 - ((((v6070 * v6156) - (v6183 * v6184)) - (v6183 * v6188)) / (((v6206 * v6070) - (((v6194 * v287) * (v1 + (v340 * (v6161 * v6160)))) / (v1 + v6194))) - (((v6200 * v287) * (v1 + (v340 * (v6163 * v6160)))) / (v1 + v6200))));
                        let v6214 = v6064 - v6213;
                        let v6221 = (v53 * v6214) + (v53 * (((v6214 * v6214) + v6217).sqrt()));
                        let v6223 = v6221.powf(v6222);
                        let v6224 = v6221.powf(v340);
                        let v6227 = v6213 / v6037;
                        let v6229 = v6227 - ((v6161 * v6224) / v6037);
                        let v6231 = v6227 - ((v6163 * v6224) / v6037);
                        let v6232 = if v6229 >= v418 { 1.0 } else { 0.0 };
                        let v6245: f64;
                        if v6232 != 0.0 {
                            v6245 = v6229;
                        } else {
                            let v6234 = if v6229 <= v6233 { 1.0 } else { 0.0 };
                            let v6246: f64;
                            if v6234 != 0.0 {
                                v6246 = v0;
                            } else {
                                let v6237 = ((v6229.exp()) + v1).ln();
                                v6246 = v6237;
                            }
                            v6245 = v6246;
                        }
                        let v6238 = if v6231 >= v418 { 1.0 } else { 0.0 };
                        let v6249: f64;
                        if v6238 != 0.0 {
                            v6249 = v6231;
                        } else {
                            let v6240 = if v6231 <= v6239 { 1.0 } else { 0.0 };
                            let v6250: f64;
                            if v6240 != 0.0 {
                                v6250 = v0;
                            } else {
                                let v6243 = ((v6231.exp()) + v1).ln();
                                v6250 = v6243;
                            }
                            v6249 = v6250;
                        }
                        let v6255 = rspice_limited_exp(v6229);
                        let v6261 = rspice_limited_exp(v6231);
                        let v6274 = v6213 - ((((v6070 * v6221) - (v6183 * v6245)) - (v6183 * v6249)) / (((v6267 * v6070) - (((v6255 * v287) * (v1 + (v340 * (v6161 * v6223)))) / (v1 + v6255))) - (((v6261 * v287) * (v1 + (v340 * (v6163 * v6223)))) / (v1 + v6261))));
                        v6278 = v6274;
                    } else {
                        v6278 = v6145;
                    }
                    let v6281 = (v6048 / v217) * ((v6064 - v6278).abs());
                    let v6299 = v6071 + (v53 * ((v6072 + v6295).sqrt()));
                    let v6300 = ((v93 * (v5387 * v540)) / ((v5385 * v537) / (((v1 + (v551 * v6281)) + (v554 * (v6281 * v6281))) + (v558 * (v547 * ((v279 - v6278).abs())))))) * v5155;
                    let v6311 = v6064 - (v6029 * ((v1 + ((v6029 / ((v6300 * v6299) / (v6300 + v6299))).powf(v574))).powf((v6307 / v574))));
                    let v6318 = (v53 * v6311) + (v53 * (((v6311 * v6311) + v6314).sqrt()));
                    let v6320 = v6318 * v6318;
                    let v6334 = (v6070 * v6318).powf(v340);
                    let v6342 = ((v6318 + (v6037 * (v1 - ((v6067 * ((v6318 * v6068) / ((v6320 + v6080).sqrt()))).ln())))) - (v6094 * v6334)) / ((v6318 * (v1 + (v6037 / ((v6318 * v6069) / ((v6320 + v6085).sqrt()))))) + (v6103 * v6334));
                    let v6343 = v6311 / v6107;
                    let v6344 = if v6343 < v354 { 1.0 } else { 0.0 };
                    let v6378: f64;
                    if v6344 != 0.0 {
                        let v6366 = ((v6107 * v6070) * (((v110 * v6343) / v119) + (((rspice_limited_exp((v6343 / v119))) + (rspice_limited_exp(((v6347 * v6343) / v119)))).ln()))) / ((v1 / v6342) + ((v6070 / v287) * (rspice_limited_exp(((v6360 * v6311) / v6107)))));
                        v6378 = v6366;
                    } else {
                        let v6377 = ((v6107 * v6070) * v6343) / ((v1 / v6342) + ((v6070 / v287) * (rspice_limited_exp(((v6371 * v6311) / v6107)))));
                        v6378 = v6377;
                    }
                    let v6380 = v6311 - (v6378 / v6070);
                    let v6383 = if ((v6380 - v6311).abs()) > v394 { 1.0 } else { 0.0 };
                    if v6383 != 0.0 {
                        let v6384 = v6311 - v6380;
                        let v6391 = (v53 * v6384) + (v53 * (((v6384 * v6384) + v6387).sqrt()));
                        let v6392 = v6070.powf(v340);
                        let v6393 = v6391.powf(v340);
                        let v6395 = v6391.powf(v6394);
                        let v6396 = v5202 * v6392;
                        let v6398 = v5272 * v6392;
                        let v6400 = v6380 / v6037;
                        let v6402 = v6400 - ((v6396 * v6393) / v6037);
                        let v6404 = v6400 - ((v6398 * v6393) / v6037);
                        let v6405 = if v6402 >= v418 { 1.0 } else { 0.0 };
                        let v6419: f64;
                        if v6405 != 0.0 {
                            v6419 = v6402;
                        } else {
                            let v6407 = if v6402 <= v6406 { 1.0 } else { 0.0 };
                            let v6420: f64;
                            if v6407 != 0.0 {
                                v6420 = v0;
                            } else {
                                let v6410 = ((v6402.exp()) + v1).ln();
                                v6420 = v6410;
                            }
                            v6419 = v6420;
                        }
                        let v6411 = if v6404 >= v418 { 1.0 } else { 0.0 };
                        let v6423: f64;
                        if v6411 != 0.0 {
                            v6423 = v6404;
                        } else {
                            let v6413 = if v6404 <= v6412 { 1.0 } else { 0.0 };
                            let v6424: f64;
                            if v6413 != 0.0 {
                                v6424 = v0;
                            } else {
                                let v6416 = ((v6404.exp()) + v1).ln();
                                v6424 = v6416;
                            }
                            v6423 = v6424;
                        }
                        let v6418 = v287 * v6037;
                        let v6429 = rspice_limited_exp(v6402);
                        let v6435 = rspice_limited_exp(v6404);
                        let v6448 = v6380 - ((((v6070 * v6391) - (v6418 * v6419)) - (v6418 * v6423)) / (((v6441 * v6070) - (((v6429 * v287) * (v1 + (v340 * (v6396 * v6395)))) / (v1 + v6429))) - (((v6435 * v287) * (v1 + (v340 * (v6398 * v6395)))) / (v1 + v6435))));
                        let v6449 = v6311 - v6448;
                        let v6457 = ((v53 * v6449) + (v53 * (((v6449 * v6449) + v6452).sqrt()))).powf(v340);
                        let v6460 = v6448 / v6037;
                        let v6462 = v6460 - ((v6396 * v6457) / v6037);
                        let v6464 = v6460 - ((v6398 * v6457) / v6037);
                        let v6465 = if v6462 >= v418 { 1.0 } else { 0.0 };
                        if v6465 != 0.0 {
                        } else {
                            let v6467 = if v6462 <= v6466 { 1.0 } else { 0.0 };
                            if v6467 != 0.0 {
                            } else {
                            }
                        }
                        let v6468 = if v6464 >= v418 { 1.0 } else { 0.0 };
                        if v6468 != 0.0 {
                        } else {
                            let v6470 = if v6464 <= v6469 { 1.0 } else { 0.0 };
                            if v6470 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v6472 = if v6471 < v0 { 1.0 } else { 0.0 };
                    if v6472 != 0.0 {
                    } else {
                    }
                    v8829 = v6037;
                } else {
                    v8829 = v8830;
                }
                v8828 = v8829;
            } else {
                let v6473 = if v6015 != v0 { 1.0 } else { 0.0 };
                let v8845: f64;
                if v6473 != 0.0 {
                    let v6474 = if v6015 == v1 { 1.0 } else { 0.0 };
                    let v6476: f64;
                    if v6474 != 0.0 {
                        v6476 = v20;
                    } else {
                        let v6475 = v35 - v17;
                        v6476 = v6475;
                    }
                    let v6478 = v47 * (v1 + v5133);
                    let v6480 = v5139 + (v264 * v5140);
                    let v6481 = v217 / v5153;
                    let v6488 = v6480 + (v6478 * (((v5155 / ((v288 * v6478) * v6478)) * v5159).ln()));
                    let v6489 = v6476 - v6488;
                    let v6496 = ((v53 * (v6489 + (((v6489 * v6489) + v300).sqrt()))) + v6488) - v6480;
                    let v6499 = v6481 / (v6497 * v6478);
                    let v6500 = v310 / v6499;
                    let v6501 = v1 / v6499;
                    let v6502 = v6481 / v285;
                    let v6503 = v53 * v6496;
                    let v6504 = v6496 * v6496;
                    let v6509 = v6503 + (v53 * ((v6504 + v6505).sqrt()));
                    let v6511 = v6509 * v6509;
                    let v6512 = v6500 * v6500;
                    let v6517 = v6501 * v6501;
                    let v6526 = v5202 / v110;
                    let v6528 = (v6502 * v6509).powf(v340);
                    let v6535 = (v93 * v5202) / v110;
                    let v6538 = ((v6509 + (v6478 * (v1 - ((v6499 * ((v6509 * v6500) / ((v6511 + v6512).sqrt()))).ln())))) - (v6526 * v6528)) / ((v6509 * (v1 + (v6478 / ((v6509 * v6501) / ((v6511 + v6517).sqrt()))))) + (v6535 * v6528));
                    let v6539 = v93 * v6478;
                    let v6540 = v6496 / v6539;
                    let v6541 = if v6540 < v354 { 1.0 } else { 0.0 };
                    let v6575: f64;
                    if v6541 != 0.0 {
                        let v6563 = ((v6539 * v6502) * (((v110 * v6540) / v119) + (((rspice_limited_exp((v6540 / v119))) + (rspice_limited_exp(((v6544 * v6540) / v119)))).ln()))) / ((v1 / v6538) + ((v6502 / v287) * (rspice_limited_exp(((v6557 * v6496) / v6539)))));
                        v6575 = v6563;
                    } else {
                        let v6574 = ((v6539 * v6502) * v6540) / ((v1 / v6538) + ((v6502 / v287) * (rspice_limited_exp(((v6568 * v6496) / v6539)))));
                        v6575 = v6574;
                    }
                    let v6577 = v6496 - (v6575 / v6502);
                    let v6580 = if ((v6577 - v6496).abs()) > v394 { 1.0 } else { 0.0 };
                    let v6710: f64;
                    if v6580 != 0.0 {
                        let v6581 = v6496 - v6577;
                        let v6588 = (v53 * v6581) + (v53 * (((v6581 * v6581) + v6584).sqrt()));
                        let v6589 = v6502.powf(v340);
                        let v6590 = v6588.powf(v340);
                        let v6592 = v6588.powf(v6591);
                        let v6593 = v5202 * v6589;
                        let v6595 = v5272 * v6589;
                        let v6597 = v6577 / v6478;
                        let v6599 = v6597 - ((v6593 * v6590) / v6478);
                        let v6601 = v6597 - ((v6595 * v6590) / v6478);
                        let v6602 = if v6599 >= v418 { 1.0 } else { 0.0 };
                        let v6616: f64;
                        if v6602 != 0.0 {
                            v6616 = v6599;
                        } else {
                            let v6604 = if v6599 <= v6603 { 1.0 } else { 0.0 };
                            let v6617: f64;
                            if v6604 != 0.0 {
                                v6617 = v0;
                            } else {
                                let v6607 = ((v6599.exp()) + v1).ln();
                                v6617 = v6607;
                            }
                            v6616 = v6617;
                        }
                        let v6608 = if v6601 >= v418 { 1.0 } else { 0.0 };
                        let v6620: f64;
                        if v6608 != 0.0 {
                            v6620 = v6601;
                        } else {
                            let v6610 = if v6601 <= v6609 { 1.0 } else { 0.0 };
                            let v6621: f64;
                            if v6610 != 0.0 {
                                v6621 = v0;
                            } else {
                                let v6613 = ((v6601.exp()) + v1).ln();
                                v6621 = v6613;
                            }
                            v6620 = v6621;
                        }
                        let v6615 = v287 * v6478;
                        let v6626 = rspice_limited_exp(v6599);
                        let v6632 = rspice_limited_exp(v6601);
                        let v6645 = v6577 - ((((v6502 * v6588) - (v6615 * v6616)) - (v6615 * v6620)) / (((v6638 * v6502) - (((v6626 * v287) * (v1 + (v340 * (v6593 * v6592)))) / (v1 + v6626))) - (((v6632 * v287) * (v1 + (v340 * (v6595 * v6592)))) / (v1 + v6632))));
                        let v6646 = v6496 - v6645;
                        let v6653 = (v53 * v6646) + (v53 * (((v6646 * v6646) + v6649).sqrt()));
                        let v6655 = v6653.powf(v6654);
                        let v6656 = v6653.powf(v340);
                        let v6659 = v6645 / v6478;
                        let v6661 = v6659 - ((v6593 * v6656) / v6478);
                        let v6663 = v6659 - ((v6595 * v6656) / v6478);
                        let v6664 = if v6661 >= v418 { 1.0 } else { 0.0 };
                        let v6677: f64;
                        if v6664 != 0.0 {
                            v6677 = v6661;
                        } else {
                            let v6666 = if v6661 <= v6665 { 1.0 } else { 0.0 };
                            let v6678: f64;
                            if v6666 != 0.0 {
                                v6678 = v0;
                            } else {
                                let v6669 = ((v6661.exp()) + v1).ln();
                                v6678 = v6669;
                            }
                            v6677 = v6678;
                        }
                        let v6670 = if v6663 >= v418 { 1.0 } else { 0.0 };
                        let v6681: f64;
                        if v6670 != 0.0 {
                            v6681 = v6663;
                        } else {
                            let v6672 = if v6663 <= v6671 { 1.0 } else { 0.0 };
                            let v6682: f64;
                            if v6672 != 0.0 {
                                v6682 = v0;
                            } else {
                                let v6675 = ((v6663.exp()) + v1).ln();
                                v6682 = v6675;
                            }
                            v6681 = v6682;
                        }
                        let v6687 = rspice_limited_exp(v6661);
                        let v6693 = rspice_limited_exp(v6663);
                        let v6706 = v6645 - ((((v6502 * v6653) - (v6615 * v6677)) - (v6615 * v6681)) / (((v6699 * v6502) - (((v6687 * v287) * (v1 + (v340 * (v6593 * v6655)))) / (v1 + v6687))) - (((v6693 * v287) * (v1 + (v340 * (v6595 * v6655)))) / (v1 + v6693))));
                        v6710 = v6706;
                    } else {
                        v6710 = v6577;
                    }
                    let v6713 = (v6481 / v217) * ((v6496 - v6710).abs());
                    let v6731 = v6503 + (v53 * ((v6504 + v6727).sqrt()));
                    let v6732 = ((v93 * (v5387 * v540)) / ((v5385 * v537) / (((v1 + (v551 * v6713)) + (v554 * (v6713 * v6713))) + (v558 * (v547 * ((v279 - v6710).abs())))))) * v5155;
                    let v6743 = v6496 - (v0 * ((v1 + ((v0 / ((v6732 * v6731) / (v6732 + v6731))).powf(v574))).powf((v6739 / v574))));
                    let v6750 = (v53 * v6743) + (v53 * (((v6743 * v6743) + v6746).sqrt()));
                    let v6752 = v6750 * v6750;
                    let v6766 = (v6502 * v6750).powf(v340);
                    let v6774 = ((v6750 + (v6478 * (v1 - ((v6499 * ((v6750 * v6500) / ((v6752 + v6512).sqrt()))).ln())))) - (v6526 * v6766)) / ((v6750 * (v1 + (v6478 / ((v6750 * v6501) / ((v6752 + v6517).sqrt()))))) + (v6535 * v6766));
                    let v6775 = v6743 / v6539;
                    let v6776 = if v6775 < v354 { 1.0 } else { 0.0 };
                    let v6810: f64;
                    if v6776 != 0.0 {
                        let v6798 = ((v6539 * v6502) * (((v110 * v6775) / v119) + (((rspice_limited_exp((v6775 / v119))) + (rspice_limited_exp(((v6779 * v6775) / v119)))).ln()))) / ((v1 / v6774) + ((v6502 / v287) * (rspice_limited_exp(((v6792 * v6743) / v6539)))));
                        v6810 = v6798;
                    } else {
                        let v6809 = ((v6539 * v6502) * v6775) / ((v1 / v6774) + ((v6502 / v287) * (rspice_limited_exp(((v6803 * v6743) / v6539)))));
                        v6810 = v6809;
                    }
                    let v6812 = v6743 - (v6810 / v6502);
                    let v6815 = if ((v6812 - v6743).abs()) > v394 { 1.0 } else { 0.0 };
                    if v6815 != 0.0 {
                        let v6816 = v6743 - v6812;
                        let v6823 = (v53 * v6816) + (v53 * (((v6816 * v6816) + v6819).sqrt()));
                        let v6824 = v6502.powf(v340);
                        let v6825 = v6823.powf(v340);
                        let v6827 = v6823.powf(v6826);
                        let v6828 = v5202 * v6824;
                        let v6830 = v5272 * v6824;
                        let v6832 = v6812 / v6478;
                        let v6834 = v6832 - ((v6828 * v6825) / v6478);
                        let v6836 = v6832 - ((v6830 * v6825) / v6478);
                        let v6837 = if v6834 >= v418 { 1.0 } else { 0.0 };
                        let v6851: f64;
                        if v6837 != 0.0 {
                            v6851 = v6834;
                        } else {
                            let v6839 = if v6834 <= v6838 { 1.0 } else { 0.0 };
                            let v6852: f64;
                            if v6839 != 0.0 {
                                v6852 = v0;
                            } else {
                                let v6842 = ((v6834.exp()) + v1).ln();
                                v6852 = v6842;
                            }
                            v6851 = v6852;
                        }
                        let v6843 = if v6836 >= v418 { 1.0 } else { 0.0 };
                        let v6855: f64;
                        if v6843 != 0.0 {
                            v6855 = v6836;
                        } else {
                            let v6845 = if v6836 <= v6844 { 1.0 } else { 0.0 };
                            let v6856: f64;
                            if v6845 != 0.0 {
                                v6856 = v0;
                            } else {
                                let v6848 = ((v6836.exp()) + v1).ln();
                                v6856 = v6848;
                            }
                            v6855 = v6856;
                        }
                        let v6850 = v287 * v6478;
                        let v6861 = rspice_limited_exp(v6834);
                        let v6867 = rspice_limited_exp(v6836);
                        let v6880 = v6812 - ((((v6502 * v6823) - (v6850 * v6851)) - (v6850 * v6855)) / (((v6873 * v6502) - (((v6861 * v287) * (v1 + (v340 * (v6828 * v6827)))) / (v1 + v6861))) - (((v6867 * v287) * (v1 + (v340 * (v6830 * v6827)))) / (v1 + v6867))));
                        let v6881 = v6743 - v6880;
                        let v6889 = ((v53 * v6881) + (v53 * (((v6881 * v6881) + v6884).sqrt()))).powf(v340);
                        let v6892 = v6880 / v6478;
                        let v6894 = v6892 - ((v6828 * v6889) / v6478);
                        let v6896 = v6892 - ((v6830 * v6889) / v6478);
                        let v6897 = if v6894 >= v418 { 1.0 } else { 0.0 };
                        if v6897 != 0.0 {
                        } else {
                            let v6899 = if v6894 <= v6898 { 1.0 } else { 0.0 };
                            if v6899 != 0.0 {
                            } else {
                            }
                        }
                        let v6900 = if v6896 >= v418 { 1.0 } else { 0.0 };
                        if v6900 != 0.0 {
                        } else {
                            let v6902 = if v6896 <= v6901 { 1.0 } else { 0.0 };
                            if v6902 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    v8845 = v6478;
                } else {
                    v8845 = v8830;
                }
                v8828 = v8845;
            }
            let v8826: f64;
            if v1539 != 0.0 {
                let v6904 = if v6903 != v0 { 1.0 } else { 0.0 };
                let v8827: f64;
                if v6904 != 0.0 {
                    let v6905 = v1477 - v5116;
                    let v6906 = if v6903 == v1 { 1.0 } else { 0.0 };
                    let v6914: f64;
                    let v6915: f64;
                    if v6906 != 0.0 {
                        let v6907 = v19 - v5116;
                        let v6908 = v19 - v1477;
                        v6914 = v6908;
                        v6915 = v6907;
                    } else {
                        let v6909 = v35 - v5116;
                        let v6910 = v35 - v1477;
                        v6914 = v6910;
                        v6915 = v6909;
                    }
                    let v6911 = if v6905 < v0 { 1.0 } else { 0.0 };
                    let v6916: f64;
                    let v6952: f64;
                    let v7371: f64;
                    if v6911 != 0.0 {
                        let v6913 = v6912 * v6905;
                        v6916 = v6913;
                        v6952 = v6914;
                        v7371 = v6912;
                    } else {
                        v6916 = v6905;
                        v6952 = v6915;
                        v7371 = v1;
                    }
                    let v6920 = (((v6916 * v6916) + v2).sqrt()) - v32;
                    let v6926 = v47 * ((v1 + v6921) + (v6923 * v6920));
                    let v6940 = (v6927 - (v264 * v6928)) - ((v6931 * (v6920 * v6932)) / (((v6920 * v6920) + (v6932 * v6932)).sqrt()));
                    let v6942 = v217 / v6941;
                    let v6951 = v6940 + (v6926 * (((v6943 / ((v288 * v6926) * v6926)) * v6947).ln()));
                    let v6953 = v6952 - v6951;
                    let v6960 = ((v53 * (v6953 + (((v6953 * v6953) + v300).sqrt()))) + v6951) - v6940;
                    let v6963 = v6942 / (v6961 * v6926);
                    let v6964 = v310 / v6963;
                    let v6965 = v1 / v6963;
                    let v6966 = v6942 / v285;
                    let v6967 = v53 * v6960;
                    let v6968 = v6960 * v6960;
                    let v6973 = v6967 + (v53 * ((v6968 + v6969).sqrt()));
                    let v6975 = v6973 * v6973;
                    let v6976 = v6964 * v6964;
                    let v6981 = v6965 * v6965;
                    let v6991 = v6990 / v110;
                    let v6993 = (v6966 * v6973).powf(v340);
                    let v7000 = (v93 * v6990) / v110;
                    let v7003 = ((v6973 + (v6926 * (v1 - ((v6963 * ((v6973 * v6964) / ((v6975 + v6976).sqrt()))).ln())))) - (v6991 * v6993)) / ((v6973 * (v1 + (v6926 / ((v6973 * v6965) / ((v6975 + v6981).sqrt()))))) + (v7000 * v6993));
                    let v7004 = v93 * v6926;
                    let v7005 = v6960 / v7004;
                    let v7006 = if v7005 < v354 { 1.0 } else { 0.0 };
                    let v7040: f64;
                    if v7006 != 0.0 {
                        let v7028 = ((v7004 * v6966) * (((v110 * v7005) / v119) + (((rspice_limited_exp((v7005 / v119))) + (rspice_limited_exp(((v7009 * v7005) / v119)))).ln()))) / ((v1 / v7003) + ((v6966 / v287) * (rspice_limited_exp(((v7022 * v6960) / v7004)))));
                        v7040 = v7028;
                    } else {
                        let v7039 = ((v7004 * v6966) * v7005) / ((v1 / v7003) + ((v6966 / v287) * (rspice_limited_exp(((v7033 * v6960) / v7004)))));
                        v7040 = v7039;
                    }
                    let v7042 = v6960 - (v7040 / v6966);
                    let v7045 = if ((v7042 - v6960).abs()) > v394 { 1.0 } else { 0.0 };
                    let v7178: f64;
                    if v7045 != 0.0 {
                        let v7046 = v6960 - v7042;
                        let v7053 = (v53 * v7046) + (v53 * (((v7046 * v7046) + v7049).sqrt()));
                        let v7054 = v6966.powf(v340);
                        let v7055 = v7053.powf(v340);
                        let v7057 = v7053.powf(v7056);
                        let v7058 = v6990 * v7054;
                        let v7061 = v7060 * v7054;
                        let v7063 = v7042 / v6926;
                        let v7065 = v7063 - ((v7058 * v7055) / v6926);
                        let v7067 = v7063 - ((v7061 * v7055) / v6926);
                        let v7068 = if v7065 >= v418 { 1.0 } else { 0.0 };
                        let v7082: f64;
                        if v7068 != 0.0 {
                            v7082 = v7065;
                        } else {
                            let v7070 = if v7065 <= v7069 { 1.0 } else { 0.0 };
                            let v7083: f64;
                            if v7070 != 0.0 {
                                v7083 = v0;
                            } else {
                                let v7073 = ((v7065.exp()) + v1).ln();
                                v7083 = v7073;
                            }
                            v7082 = v7083;
                        }
                        let v7074 = if v7067 >= v418 { 1.0 } else { 0.0 };
                        let v7086: f64;
                        if v7074 != 0.0 {
                            v7086 = v7067;
                        } else {
                            let v7076 = if v7067 <= v7075 { 1.0 } else { 0.0 };
                            let v7087: f64;
                            if v7076 != 0.0 {
                                v7087 = v0;
                            } else {
                                let v7079 = ((v7067.exp()) + v1).ln();
                                v7087 = v7079;
                            }
                            v7086 = v7087;
                        }
                        let v7081 = v287 * v6926;
                        let v7092 = rspice_limited_exp(v7065);
                        let v7098 = rspice_limited_exp(v7067);
                        let v7111 = v7042 - ((((v6966 * v7053) - (v7081 * v7082)) - (v7081 * v7086)) / (((v7104 * v6966) - (((v7092 * v287) * (v1 + (v340 * (v7058 * v7057)))) / (v1 + v7092))) - (((v7098 * v287) * (v1 + (v340 * (v7061 * v7057)))) / (v1 + v7098))));
                        let v7112 = v6960 - v7111;
                        let v7119 = (v53 * v7112) + (v53 * (((v7112 * v7112) + v7115).sqrt()));
                        let v7121 = v7119.powf(v7120);
                        let v7122 = v7119.powf(v340);
                        let v7125 = v7111 / v6926;
                        let v7127 = v7125 - ((v7058 * v7122) / v6926);
                        let v7129 = v7125 - ((v7061 * v7122) / v6926);
                        let v7130 = if v7127 >= v418 { 1.0 } else { 0.0 };
                        let v7143: f64;
                        if v7130 != 0.0 {
                            v7143 = v7127;
                        } else {
                            let v7132 = if v7127 <= v7131 { 1.0 } else { 0.0 };
                            let v7144: f64;
                            if v7132 != 0.0 {
                                v7144 = v0;
                            } else {
                                let v7135 = ((v7127.exp()) + v1).ln();
                                v7144 = v7135;
                            }
                            v7143 = v7144;
                        }
                        let v7136 = if v7129 >= v418 { 1.0 } else { 0.0 };
                        let v7147: f64;
                        if v7136 != 0.0 {
                            v7147 = v7129;
                        } else {
                            let v7138 = if v7129 <= v7137 { 1.0 } else { 0.0 };
                            let v7148: f64;
                            if v7138 != 0.0 {
                                v7148 = v0;
                            } else {
                                let v7141 = ((v7129.exp()) + v1).ln();
                                v7148 = v7141;
                            }
                            v7147 = v7148;
                        }
                        let v7153 = rspice_limited_exp(v7127);
                        let v7159 = rspice_limited_exp(v7129);
                        let v7172 = v7111 - ((((v6966 * v7119) - (v7081 * v7143)) - (v7081 * v7147)) / (((v7165 * v6966) - (((v7153 * v287) * (v1 + (v340 * (v7058 * v7121)))) / (v1 + v7153))) - (((v7159 * v287) * (v1 + (v340 * (v7061 * v7121)))) / (v1 + v7159))));
                        v7178 = v7172;
                    } else {
                        v7178 = v7042;
                    }
                    let v7181 = (v6942 / v217) * ((v6960 - v7178).abs());
                    let v7199 = v6967 + (v53 * ((v6968 + v7195).sqrt()));
                    let v7200 = ((v93 * (v7175 * v540)) / ((v7173 * v537) / (((v1 + (v551 * v7181)) + (v554 * (v7181 * v7181))) + (v558 * (v547 * ((v279 - v7178).abs())))))) * v6943;
                    let v7211 = v6960 - (v6916 * ((v1 + ((v6916 / ((v7200 * v7199) / (v7200 + v7199))).powf(v574))).powf((v7207 / v574))));
                    let v7218 = (v53 * v7211) + (v53 * (((v7211 * v7211) + v7214).sqrt()));
                    let v7220 = v7218 * v7218;
                    let v7234 = (v6966 * v7218).powf(v340);
                    let v7242 = ((v7218 + (v6926 * (v1 - ((v6963 * ((v7218 * v6964) / ((v7220 + v6976).sqrt()))).ln())))) - (v6991 * v7234)) / ((v7218 * (v1 + (v6926 / ((v7218 * v6965) / ((v7220 + v6981).sqrt()))))) + (v7000 * v7234));
                    let v7243 = v7211 / v7004;
                    let v7244 = if v7243 < v354 { 1.0 } else { 0.0 };
                    let v7278: f64;
                    if v7244 != 0.0 {
                        let v7266 = ((v7004 * v6966) * (((v110 * v7243) / v119) + (((rspice_limited_exp((v7243 / v119))) + (rspice_limited_exp(((v7247 * v7243) / v119)))).ln()))) / ((v1 / v7242) + ((v6966 / v287) * (rspice_limited_exp(((v7260 * v7211) / v7004)))));
                        v7278 = v7266;
                    } else {
                        let v7277 = ((v7004 * v6966) * v7243) / ((v1 / v7242) + ((v6966 / v287) * (rspice_limited_exp(((v7271 * v7211) / v7004)))));
                        v7278 = v7277;
                    }
                    let v7280 = v7211 - (v7278 / v6966);
                    let v7283 = if ((v7280 - v7211).abs()) > v394 { 1.0 } else { 0.0 };
                    if v7283 != 0.0 {
                        let v7284 = v7211 - v7280;
                        let v7291 = (v53 * v7284) + (v53 * (((v7284 * v7284) + v7287).sqrt()));
                        let v7292 = v6966.powf(v340);
                        let v7293 = v7291.powf(v340);
                        let v7295 = v7291.powf(v7294);
                        let v7296 = v6990 * v7292;
                        let v7298 = v7060 * v7292;
                        let v7300 = v7280 / v6926;
                        let v7302 = v7300 - ((v7296 * v7293) / v6926);
                        let v7304 = v7300 - ((v7298 * v7293) / v6926);
                        let v7305 = if v7302 >= v418 { 1.0 } else { 0.0 };
                        let v7319: f64;
                        if v7305 != 0.0 {
                            v7319 = v7302;
                        } else {
                            let v7307 = if v7302 <= v7306 { 1.0 } else { 0.0 };
                            let v7320: f64;
                            if v7307 != 0.0 {
                                v7320 = v0;
                            } else {
                                let v7310 = ((v7302.exp()) + v1).ln();
                                v7320 = v7310;
                            }
                            v7319 = v7320;
                        }
                        let v7311 = if v7304 >= v418 { 1.0 } else { 0.0 };
                        let v7323: f64;
                        if v7311 != 0.0 {
                            v7323 = v7304;
                        } else {
                            let v7313 = if v7304 <= v7312 { 1.0 } else { 0.0 };
                            let v7324: f64;
                            if v7313 != 0.0 {
                                v7324 = v0;
                            } else {
                                let v7316 = ((v7304.exp()) + v1).ln();
                                v7324 = v7316;
                            }
                            v7323 = v7324;
                        }
                        let v7318 = v287 * v6926;
                        let v7329 = rspice_limited_exp(v7302);
                        let v7335 = rspice_limited_exp(v7304);
                        let v7348 = v7280 - ((((v6966 * v7291) - (v7318 * v7319)) - (v7318 * v7323)) / (((v7341 * v6966) - (((v7329 * v287) * (v1 + (v340 * (v7296 * v7295)))) / (v1 + v7329))) - (((v7335 * v287) * (v1 + (v340 * (v7298 * v7295)))) / (v1 + v7335))));
                        let v7349 = v7211 - v7348;
                        let v7357 = ((v53 * v7349) + (v53 * (((v7349 * v7349) + v7352).sqrt()))).powf(v340);
                        let v7360 = v7348 / v6926;
                        let v7362 = v7360 - ((v7296 * v7357) / v6926);
                        let v7364 = v7360 - ((v7298 * v7357) / v6926);
                        let v7365 = if v7362 >= v418 { 1.0 } else { 0.0 };
                        if v7365 != 0.0 {
                        } else {
                            let v7367 = if v7362 <= v7366 { 1.0 } else { 0.0 };
                            if v7367 != 0.0 {
                            } else {
                            }
                        }
                        let v7368 = if v7364 >= v418 { 1.0 } else { 0.0 };
                        if v7368 != 0.0 {
                        } else {
                            let v7370 = if v7364 <= v7369 { 1.0 } else { 0.0 };
                            if v7370 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v7372 = if v7371 < v0 { 1.0 } else { 0.0 };
                    if v7372 != 0.0 {
                    } else {
                    }
                    v8827 = v6926;
                } else {
                    v8827 = v8828;
                }
                v8826 = v8827;
            } else {
                let v7373 = if v6903 != v0 { 1.0 } else { 0.0 };
                let v8846: f64;
                if v7373 != 0.0 {
                    let v7374 = if v6903 == v1 { 1.0 } else { 0.0 };
                    let v7376: f64;
                    if v7374 != 0.0 {
                        v7376 = v21;
                    } else {
                        let v7375 = v35 - v16;
                        v7376 = v7375;
                    }
                    let v7378 = v47 * (v1 + v6921);
                    let v7380 = v6927 - (v264 * v6928);
                    let v7381 = v217 / v6941;
                    let v7388 = v7380 + (v7378 * (((v6943 / ((v288 * v7378) * v7378)) * v6947).ln()));
                    let v7389 = v7376 - v7388;
                    let v7396 = ((v53 * (v7389 + (((v7389 * v7389) + v300).sqrt()))) + v7388) - v7380;
                    let v7399 = v7381 / (v7397 * v7378);
                    let v7400 = v310 / v7399;
                    let v7401 = v1 / v7399;
                    let v7402 = v7381 / v285;
                    let v7403 = v53 * v7396;
                    let v7404 = v7396 * v7396;
                    let v7409 = v7403 + (v53 * ((v7404 + v7405).sqrt()));
                    let v7411 = v7409 * v7409;
                    let v7412 = v7400 * v7400;
                    let v7417 = v7401 * v7401;
                    let v7426 = v6990 / v110;
                    let v7428 = (v7402 * v7409).powf(v340);
                    let v7435 = (v93 * v6990) / v110;
                    let v7438 = ((v7409 + (v7378 * (v1 - ((v7399 * ((v7409 * v7400) / ((v7411 + v7412).sqrt()))).ln())))) - (v7426 * v7428)) / ((v7409 * (v1 + (v7378 / ((v7409 * v7401) / ((v7411 + v7417).sqrt()))))) + (v7435 * v7428));
                    let v7439 = v93 * v7378;
                    let v7440 = v7396 / v7439;
                    let v7441 = if v7440 < v354 { 1.0 } else { 0.0 };
                    let v7475: f64;
                    if v7441 != 0.0 {
                        let v7463 = ((v7439 * v7402) * (((v110 * v7440) / v119) + (((rspice_limited_exp((v7440 / v119))) + (rspice_limited_exp(((v7444 * v7440) / v119)))).ln()))) / ((v1 / v7438) + ((v7402 / v287) * (rspice_limited_exp(((v7457 * v7396) / v7439)))));
                        v7475 = v7463;
                    } else {
                        let v7474 = ((v7439 * v7402) * v7440) / ((v1 / v7438) + ((v7402 / v287) * (rspice_limited_exp(((v7468 * v7396) / v7439)))));
                        v7475 = v7474;
                    }
                    let v7477 = v7396 - (v7475 / v7402);
                    let v7480 = if ((v7477 - v7396).abs()) > v394 { 1.0 } else { 0.0 };
                    let v7610: f64;
                    if v7480 != 0.0 {
                        let v7481 = v7396 - v7477;
                        let v7488 = (v53 * v7481) + (v53 * (((v7481 * v7481) + v7484).sqrt()));
                        let v7489 = v7402.powf(v340);
                        let v7490 = v7488.powf(v340);
                        let v7492 = v7488.powf(v7491);
                        let v7493 = v6990 * v7489;
                        let v7495 = v7060 * v7489;
                        let v7497 = v7477 / v7378;
                        let v7499 = v7497 - ((v7493 * v7490) / v7378);
                        let v7501 = v7497 - ((v7495 * v7490) / v7378);
                        let v7502 = if v7499 >= v418 { 1.0 } else { 0.0 };
                        let v7516: f64;
                        if v7502 != 0.0 {
                            v7516 = v7499;
                        } else {
                            let v7504 = if v7499 <= v7503 { 1.0 } else { 0.0 };
                            let v7517: f64;
                            if v7504 != 0.0 {
                                v7517 = v0;
                            } else {
                                let v7507 = ((v7499.exp()) + v1).ln();
                                v7517 = v7507;
                            }
                            v7516 = v7517;
                        }
                        let v7508 = if v7501 >= v418 { 1.0 } else { 0.0 };
                        let v7520: f64;
                        if v7508 != 0.0 {
                            v7520 = v7501;
                        } else {
                            let v7510 = if v7501 <= v7509 { 1.0 } else { 0.0 };
                            let v7521: f64;
                            if v7510 != 0.0 {
                                v7521 = v0;
                            } else {
                                let v7513 = ((v7501.exp()) + v1).ln();
                                v7521 = v7513;
                            }
                            v7520 = v7521;
                        }
                        let v7515 = v287 * v7378;
                        let v7526 = rspice_limited_exp(v7499);
                        let v7532 = rspice_limited_exp(v7501);
                        let v7545 = v7477 - ((((v7402 * v7488) - (v7515 * v7516)) - (v7515 * v7520)) / (((v7538 * v7402) - (((v7526 * v287) * (v1 + (v340 * (v7493 * v7492)))) / (v1 + v7526))) - (((v7532 * v287) * (v1 + (v340 * (v7495 * v7492)))) / (v1 + v7532))));
                        let v7546 = v7396 - v7545;
                        let v7553 = (v53 * v7546) + (v53 * (((v7546 * v7546) + v7549).sqrt()));
                        let v7555 = v7553.powf(v7554);
                        let v7556 = v7553.powf(v340);
                        let v7559 = v7545 / v7378;
                        let v7561 = v7559 - ((v7493 * v7556) / v7378);
                        let v7563 = v7559 - ((v7495 * v7556) / v7378);
                        let v7564 = if v7561 >= v418 { 1.0 } else { 0.0 };
                        let v7577: f64;
                        if v7564 != 0.0 {
                            v7577 = v7561;
                        } else {
                            let v7566 = if v7561 <= v7565 { 1.0 } else { 0.0 };
                            let v7578: f64;
                            if v7566 != 0.0 {
                                v7578 = v0;
                            } else {
                                let v7569 = ((v7561.exp()) + v1).ln();
                                v7578 = v7569;
                            }
                            v7577 = v7578;
                        }
                        let v7570 = if v7563 >= v418 { 1.0 } else { 0.0 };
                        let v7581: f64;
                        if v7570 != 0.0 {
                            v7581 = v7563;
                        } else {
                            let v7572 = if v7563 <= v7571 { 1.0 } else { 0.0 };
                            let v7582: f64;
                            if v7572 != 0.0 {
                                v7582 = v0;
                            } else {
                                let v7575 = ((v7563.exp()) + v1).ln();
                                v7582 = v7575;
                            }
                            v7581 = v7582;
                        }
                        let v7587 = rspice_limited_exp(v7561);
                        let v7593 = rspice_limited_exp(v7563);
                        let v7606 = v7545 - ((((v7402 * v7553) - (v7515 * v7577)) - (v7515 * v7581)) / (((v7599 * v7402) - (((v7587 * v287) * (v1 + (v340 * (v7493 * v7555)))) / (v1 + v7587))) - (((v7593 * v287) * (v1 + (v340 * (v7495 * v7555)))) / (v1 + v7593))));
                        v7610 = v7606;
                    } else {
                        v7610 = v7477;
                    }
                    let v7613 = (v7381 / v217) * ((v7396 - v7610).abs());
                    let v7631 = v7403 + (v53 * ((v7404 + v7627).sqrt()));
                    let v7632 = ((v93 * (v7175 * v540)) / ((v7173 * v537) / (((v1 + (v551 * v7613)) + (v554 * (v7613 * v7613))) + (v558 * (v547 * ((v279 - v7610).abs())))))) * v6943;
                    let v7643 = v7396 - (v0 * ((v1 + ((v0 / ((v7632 * v7631) / (v7632 + v7631))).powf(v574))).powf((v7639 / v574))));
                    let v7650 = (v53 * v7643) + (v53 * (((v7643 * v7643) + v7646).sqrt()));
                    let v7652 = v7650 * v7650;
                    let v7666 = (v7402 * v7650).powf(v340);
                    let v7674 = ((v7650 + (v7378 * (v1 - ((v7399 * ((v7650 * v7400) / ((v7652 + v7412).sqrt()))).ln())))) - (v7426 * v7666)) / ((v7650 * (v1 + (v7378 / ((v7650 * v7401) / ((v7652 + v7417).sqrt()))))) + (v7435 * v7666));
                    let v7675 = v7643 / v7439;
                    let v7676 = if v7675 < v354 { 1.0 } else { 0.0 };
                    let v7710: f64;
                    if v7676 != 0.0 {
                        let v7698 = ((v7439 * v7402) * (((v110 * v7675) / v119) + (((rspice_limited_exp((v7675 / v119))) + (rspice_limited_exp(((v7679 * v7675) / v119)))).ln()))) / ((v1 / v7674) + ((v7402 / v287) * (rspice_limited_exp(((v7692 * v7643) / v7439)))));
                        v7710 = v7698;
                    } else {
                        let v7709 = ((v7439 * v7402) * v7675) / ((v1 / v7674) + ((v7402 / v287) * (rspice_limited_exp(((v7703 * v7643) / v7439)))));
                        v7710 = v7709;
                    }
                    let v7712 = v7643 - (v7710 / v7402);
                    let v7715 = if ((v7712 - v7643).abs()) > v394 { 1.0 } else { 0.0 };
                    if v7715 != 0.0 {
                        let v7716 = v7643 - v7712;
                        let v7723 = (v53 * v7716) + (v53 * (((v7716 * v7716) + v7719).sqrt()));
                        let v7724 = v7402.powf(v340);
                        let v7725 = v7723.powf(v340);
                        let v7727 = v7723.powf(v7726);
                        let v7728 = v6990 * v7724;
                        let v7730 = v7060 * v7724;
                        let v7732 = v7712 / v7378;
                        let v7734 = v7732 - ((v7728 * v7725) / v7378);
                        let v7736 = v7732 - ((v7730 * v7725) / v7378);
                        let v7737 = if v7734 >= v418 { 1.0 } else { 0.0 };
                        let v7751: f64;
                        if v7737 != 0.0 {
                            v7751 = v7734;
                        } else {
                            let v7739 = if v7734 <= v7738 { 1.0 } else { 0.0 };
                            let v7752: f64;
                            if v7739 != 0.0 {
                                v7752 = v0;
                            } else {
                                let v7742 = ((v7734.exp()) + v1).ln();
                                v7752 = v7742;
                            }
                            v7751 = v7752;
                        }
                        let v7743 = if v7736 >= v418 { 1.0 } else { 0.0 };
                        let v7755: f64;
                        if v7743 != 0.0 {
                            v7755 = v7736;
                        } else {
                            let v7745 = if v7736 <= v7744 { 1.0 } else { 0.0 };
                            let v7756: f64;
                            if v7745 != 0.0 {
                                v7756 = v0;
                            } else {
                                let v7748 = ((v7736.exp()) + v1).ln();
                                v7756 = v7748;
                            }
                            v7755 = v7756;
                        }
                        let v7750 = v287 * v7378;
                        let v7761 = rspice_limited_exp(v7734);
                        let v7767 = rspice_limited_exp(v7736);
                        let v7780 = v7712 - ((((v7402 * v7723) - (v7750 * v7751)) - (v7750 * v7755)) / (((v7773 * v7402) - (((v7761 * v287) * (v1 + (v340 * (v7728 * v7727)))) / (v1 + v7761))) - (((v7767 * v287) * (v1 + (v340 * (v7730 * v7727)))) / (v1 + v7767))));
                        let v7781 = v7643 - v7780;
                        let v7789 = ((v53 * v7781) + (v53 * (((v7781 * v7781) + v7784).sqrt()))).powf(v340);
                        let v7792 = v7780 / v7378;
                        let v7794 = v7792 - ((v7728 * v7789) / v7378);
                        let v7796 = v7792 - ((v7730 * v7789) / v7378);
                        let v7797 = if v7794 >= v418 { 1.0 } else { 0.0 };
                        if v7797 != 0.0 {
                        } else {
                            let v7799 = if v7794 <= v7798 { 1.0 } else { 0.0 };
                            if v7799 != 0.0 {
                            } else {
                            }
                        }
                        let v7800 = if v7796 >= v418 { 1.0 } else { 0.0 };
                        if v7800 != 0.0 {
                        } else {
                            let v7802 = if v7796 <= v7801 { 1.0 } else { 0.0 };
                            if v7802 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    v8846 = v7378;
                } else {
                    v8846 = v8828;
                }
                v8826 = v8846;
            }
            let v8824: f64;
            if v1539 != 0.0 {
                let v7804 = if v7803 != v0 { 1.0 } else { 0.0 };
                let v8825: f64;
                if v7804 != 0.0 {
                    let v7805 = v6017 - v1478;
                    let v7806 = if v7803 == v1 { 1.0 } else { 0.0 };
                    let v7814: f64;
                    let v7815: f64;
                    if v7806 != 0.0 {
                        let v7807 = v19 - v1478;
                        let v7808 = v19 - v6017;
                        v7814 = v7808;
                        v7815 = v7807;
                    } else {
                        let v7809 = v35 - v1478;
                        let v7810 = v35 - v6017;
                        v7814 = v7810;
                        v7815 = v7809;
                    }
                    let v7811 = if v7805 < v0 { 1.0 } else { 0.0 };
                    let v7816: f64;
                    let v7843: f64;
                    let v8258: f64;
                    if v7811 != 0.0 {
                        let v7813 = v7812 * v7805;
                        v7816 = v7813;
                        v7843 = v7814;
                        v8258 = v7812;
                    } else {
                        v7816 = v7805;
                        v7843 = v7815;
                        v8258 = v1;
                    }
                    let v7820 = (((v7816 * v7816) + v2).sqrt()) - v32;
                    let v7824 = v47 * ((v1 + v6921) + (v6923 * v7820));
                    let v7834 = (v6927 + (v264 * v6928)) - ((v6931 * (v7820 * v6932)) / (((v7820 * v7820) + (v6932 * v6932)).sqrt()));
                    let v7835 = v217 / v6941;
                    let v7842 = v7834 + (v7824 * (((v6943 / ((v288 * v7824) * v7824)) * v6947).ln()));
                    let v7844 = v7843 - v7842;
                    let v7851 = ((v53 * (v7844 + (((v7844 * v7844) + v300).sqrt()))) + v7842) - v7834;
                    let v7854 = v7835 / (v7852 * v7824);
                    let v7855 = v310 / v7854;
                    let v7856 = v1 / v7854;
                    let v7857 = v7835 / v285;
                    let v7858 = v53 * v7851;
                    let v7859 = v7851 * v7851;
                    let v7864 = v7858 + (v53 * ((v7859 + v7860).sqrt()));
                    let v7866 = v7864 * v7864;
                    let v7867 = v7855 * v7855;
                    let v7872 = v7856 * v7856;
                    let v7881 = v6990 / v110;
                    let v7883 = (v7857 * v7864).powf(v340);
                    let v7890 = (v93 * v6990) / v110;
                    let v7893 = ((v7864 + (v7824 * (v1 - ((v7854 * ((v7864 * v7855) / ((v7866 + v7867).sqrt()))).ln())))) - (v7881 * v7883)) / ((v7864 * (v1 + (v7824 / ((v7864 * v7856) / ((v7866 + v7872).sqrt()))))) + (v7890 * v7883));
                    let v7894 = v93 * v7824;
                    let v7895 = v7851 / v7894;
                    let v7896 = if v7895 < v354 { 1.0 } else { 0.0 };
                    let v7930: f64;
                    if v7896 != 0.0 {
                        let v7918 = ((v7894 * v7857) * (((v110 * v7895) / v119) + (((rspice_limited_exp((v7895 / v119))) + (rspice_limited_exp(((v7899 * v7895) / v119)))).ln()))) / ((v1 / v7893) + ((v7857 / v287) * (rspice_limited_exp(((v7912 * v7851) / v7894)))));
                        v7930 = v7918;
                    } else {
                        let v7929 = ((v7894 * v7857) * v7895) / ((v1 / v7893) + ((v7857 / v287) * (rspice_limited_exp(((v7923 * v7851) / v7894)))));
                        v7930 = v7929;
                    }
                    let v7932 = v7851 - (v7930 / v7857);
                    let v7935 = if ((v7932 - v7851).abs()) > v394 { 1.0 } else { 0.0 };
                    let v8065: f64;
                    if v7935 != 0.0 {
                        let v7936 = v7851 - v7932;
                        let v7943 = (v53 * v7936) + (v53 * (((v7936 * v7936) + v7939).sqrt()));
                        let v7944 = v7857.powf(v340);
                        let v7945 = v7943.powf(v340);
                        let v7947 = v7943.powf(v7946);
                        let v7948 = v6990 * v7944;
                        let v7950 = v7060 * v7944;
                        let v7952 = v7932 / v7824;
                        let v7954 = v7952 - ((v7948 * v7945) / v7824);
                        let v7956 = v7952 - ((v7950 * v7945) / v7824);
                        let v7957 = if v7954 >= v418 { 1.0 } else { 0.0 };
                        let v7971: f64;
                        if v7957 != 0.0 {
                            v7971 = v7954;
                        } else {
                            let v7959 = if v7954 <= v7958 { 1.0 } else { 0.0 };
                            let v7972: f64;
                            if v7959 != 0.0 {
                                v7972 = v0;
                            } else {
                                let v7962 = ((v7954.exp()) + v1).ln();
                                v7972 = v7962;
                            }
                            v7971 = v7972;
                        }
                        let v7963 = if v7956 >= v418 { 1.0 } else { 0.0 };
                        let v7975: f64;
                        if v7963 != 0.0 {
                            v7975 = v7956;
                        } else {
                            let v7965 = if v7956 <= v7964 { 1.0 } else { 0.0 };
                            let v7976: f64;
                            if v7965 != 0.0 {
                                v7976 = v0;
                            } else {
                                let v7968 = ((v7956.exp()) + v1).ln();
                                v7976 = v7968;
                            }
                            v7975 = v7976;
                        }
                        let v7970 = v287 * v7824;
                        let v7981 = rspice_limited_exp(v7954);
                        let v7987 = rspice_limited_exp(v7956);
                        let v8000 = v7932 - ((((v7857 * v7943) - (v7970 * v7971)) - (v7970 * v7975)) / (((v7993 * v7857) - (((v7981 * v287) * (v1 + (v340 * (v7948 * v7947)))) / (v1 + v7981))) - (((v7987 * v287) * (v1 + (v340 * (v7950 * v7947)))) / (v1 + v7987))));
                        let v8001 = v7851 - v8000;
                        let v8008 = (v53 * v8001) + (v53 * (((v8001 * v8001) + v8004).sqrt()));
                        let v8010 = v8008.powf(v8009);
                        let v8011 = v8008.powf(v340);
                        let v8014 = v8000 / v7824;
                        let v8016 = v8014 - ((v7948 * v8011) / v7824);
                        let v8018 = v8014 - ((v7950 * v8011) / v7824);
                        let v8019 = if v8016 >= v418 { 1.0 } else { 0.0 };
                        let v8032: f64;
                        if v8019 != 0.0 {
                            v8032 = v8016;
                        } else {
                            let v8021 = if v8016 <= v8020 { 1.0 } else { 0.0 };
                            let v8033: f64;
                            if v8021 != 0.0 {
                                v8033 = v0;
                            } else {
                                let v8024 = ((v8016.exp()) + v1).ln();
                                v8033 = v8024;
                            }
                            v8032 = v8033;
                        }
                        let v8025 = if v8018 >= v418 { 1.0 } else { 0.0 };
                        let v8036: f64;
                        if v8025 != 0.0 {
                            v8036 = v8018;
                        } else {
                            let v8027 = if v8018 <= v8026 { 1.0 } else { 0.0 };
                            let v8037: f64;
                            if v8027 != 0.0 {
                                v8037 = v0;
                            } else {
                                let v8030 = ((v8018.exp()) + v1).ln();
                                v8037 = v8030;
                            }
                            v8036 = v8037;
                        }
                        let v8042 = rspice_limited_exp(v8016);
                        let v8048 = rspice_limited_exp(v8018);
                        let v8061 = v8000 - ((((v7857 * v8008) - (v7970 * v8032)) - (v7970 * v8036)) / (((v8054 * v7857) - (((v8042 * v287) * (v1 + (v340 * (v7948 * v8010)))) / (v1 + v8042))) - (((v8048 * v287) * (v1 + (v340 * (v7950 * v8010)))) / (v1 + v8048))));
                        v8065 = v8061;
                    } else {
                        v8065 = v7932;
                    }
                    let v8068 = (v7835 / v217) * ((v7851 - v8065).abs());
                    let v8086 = v7858 + (v53 * ((v7859 + v8082).sqrt()));
                    let v8087 = ((v93 * (v7175 * v540)) / ((v7173 * v537) / (((v1 + (v551 * v8068)) + (v554 * (v8068 * v8068))) + (v558 * (v547 * ((v279 - v8065).abs())))))) * v6943;
                    let v8098 = v7851 - (v7816 * ((v1 + ((v7816 / ((v8087 * v8086) / (v8087 + v8086))).powf(v574))).powf((v8094 / v574))));
                    let v8105 = (v53 * v8098) + (v53 * (((v8098 * v8098) + v8101).sqrt()));
                    let v8107 = v8105 * v8105;
                    let v8121 = (v7857 * v8105).powf(v340);
                    let v8129 = ((v8105 + (v7824 * (v1 - ((v7854 * ((v8105 * v7855) / ((v8107 + v7867).sqrt()))).ln())))) - (v7881 * v8121)) / ((v8105 * (v1 + (v7824 / ((v8105 * v7856) / ((v8107 + v7872).sqrt()))))) + (v7890 * v8121));
                    let v8130 = v8098 / v7894;
                    let v8131 = if v8130 < v354 { 1.0 } else { 0.0 };
                    let v8165: f64;
                    if v8131 != 0.0 {
                        let v8153 = ((v7894 * v7857) * (((v110 * v8130) / v119) + (((rspice_limited_exp((v8130 / v119))) + (rspice_limited_exp(((v8134 * v8130) / v119)))).ln()))) / ((v1 / v8129) + ((v7857 / v287) * (rspice_limited_exp(((v8147 * v8098) / v7894)))));
                        v8165 = v8153;
                    } else {
                        let v8164 = ((v7894 * v7857) * v8130) / ((v1 / v8129) + ((v7857 / v287) * (rspice_limited_exp(((v8158 * v8098) / v7894)))));
                        v8165 = v8164;
                    }
                    let v8167 = v8098 - (v8165 / v7857);
                    let v8170 = if ((v8167 - v8098).abs()) > v394 { 1.0 } else { 0.0 };
                    if v8170 != 0.0 {
                        let v8171 = v8098 - v8167;
                        let v8178 = (v53 * v8171) + (v53 * (((v8171 * v8171) + v8174).sqrt()));
                        let v8179 = v7857.powf(v340);
                        let v8180 = v8178.powf(v340);
                        let v8182 = v8178.powf(v8181);
                        let v8183 = v6990 * v8179;
                        let v8185 = v7060 * v8179;
                        let v8187 = v8167 / v7824;
                        let v8189 = v8187 - ((v8183 * v8180) / v7824);
                        let v8191 = v8187 - ((v8185 * v8180) / v7824);
                        let v8192 = if v8189 >= v418 { 1.0 } else { 0.0 };
                        let v8206: f64;
                        if v8192 != 0.0 {
                            v8206 = v8189;
                        } else {
                            let v8194 = if v8189 <= v8193 { 1.0 } else { 0.0 };
                            let v8207: f64;
                            if v8194 != 0.0 {
                                v8207 = v0;
                            } else {
                                let v8197 = ((v8189.exp()) + v1).ln();
                                v8207 = v8197;
                            }
                            v8206 = v8207;
                        }
                        let v8198 = if v8191 >= v418 { 1.0 } else { 0.0 };
                        let v8210: f64;
                        if v8198 != 0.0 {
                            v8210 = v8191;
                        } else {
                            let v8200 = if v8191 <= v8199 { 1.0 } else { 0.0 };
                            let v8211: f64;
                            if v8200 != 0.0 {
                                v8211 = v0;
                            } else {
                                let v8203 = ((v8191.exp()) + v1).ln();
                                v8211 = v8203;
                            }
                            v8210 = v8211;
                        }
                        let v8205 = v287 * v7824;
                        let v8216 = rspice_limited_exp(v8189);
                        let v8222 = rspice_limited_exp(v8191);
                        let v8235 = v8167 - ((((v7857 * v8178) - (v8205 * v8206)) - (v8205 * v8210)) / (((v8228 * v7857) - (((v8216 * v287) * (v1 + (v340 * (v8183 * v8182)))) / (v1 + v8216))) - (((v8222 * v287) * (v1 + (v340 * (v8185 * v8182)))) / (v1 + v8222))));
                        let v8236 = v8098 - v8235;
                        let v8244 = ((v53 * v8236) + (v53 * (((v8236 * v8236) + v8239).sqrt()))).powf(v340);
                        let v8247 = v8235 / v7824;
                        let v8249 = v8247 - ((v8183 * v8244) / v7824);
                        let v8251 = v8247 - ((v8185 * v8244) / v7824);
                        let v8252 = if v8249 >= v418 { 1.0 } else { 0.0 };
                        if v8252 != 0.0 {
                        } else {
                            let v8254 = if v8249 <= v8253 { 1.0 } else { 0.0 };
                            if v8254 != 0.0 {
                            } else {
                            }
                        }
                        let v8255 = if v8251 >= v418 { 1.0 } else { 0.0 };
                        if v8255 != 0.0 {
                        } else {
                            let v8257 = if v8251 <= v8256 { 1.0 } else { 0.0 };
                            if v8257 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    let v8259 = if v8258 < v0 { 1.0 } else { 0.0 };
                    if v8259 != 0.0 {
                    } else {
                    }
                    v8825 = v7824;
                } else {
                    v8825 = v8826;
                }
                v8824 = v8825;
            } else {
                let v8260 = if v7803 != v0 { 1.0 } else { 0.0 };
                let v8847: f64;
                if v8260 != 0.0 {
                    let v8261 = if v7803 == v1 { 1.0 } else { 0.0 };
                    let v8263: f64;
                    if v8261 != 0.0 {
                        v8263 = v20;
                    } else {
                        let v8262 = v35 - v17;
                        v8263 = v8262;
                    }
                    let v8265 = v47 * (v1 + v6921);
                    let v8267 = v6927 + (v264 * v6928);
                    let v8268 = v217 / v6941;
                    let v8275 = v8267 + (v8265 * (((v6943 / ((v288 * v8265) * v8265)) * v6947).ln()));
                    let v8276 = v8263 - v8275;
                    let v8283 = ((v53 * (v8276 + (((v8276 * v8276) + v300).sqrt()))) + v8275) - v8267;
                    let v8286 = v8268 / (v8284 * v8265);
                    let v8287 = v310 / v8286;
                    let v8288 = v1 / v8286;
                    let v8289 = v8268 / v285;
                    let v8290 = v53 * v8283;
                    let v8291 = v8283 * v8283;
                    let v8296 = v8290 + (v53 * ((v8291 + v8292).sqrt()));
                    let v8298 = v8296 * v8296;
                    let v8299 = v8287 * v8287;
                    let v8304 = v8288 * v8288;
                    let v8313 = v6990 / v110;
                    let v8315 = (v8289 * v8296).powf(v340);
                    let v8322 = (v93 * v6990) / v110;
                    let v8325 = ((v8296 + (v8265 * (v1 - ((v8286 * ((v8296 * v8287) / ((v8298 + v8299).sqrt()))).ln())))) - (v8313 * v8315)) / ((v8296 * (v1 + (v8265 / ((v8296 * v8288) / ((v8298 + v8304).sqrt()))))) + (v8322 * v8315));
                    let v8326 = v93 * v8265;
                    let v8327 = v8283 / v8326;
                    let v8328 = if v8327 < v354 { 1.0 } else { 0.0 };
                    let v8362: f64;
                    if v8328 != 0.0 {
                        let v8350 = ((v8326 * v8289) * (((v110 * v8327) / v119) + (((rspice_limited_exp((v8327 / v119))) + (rspice_limited_exp(((v8331 * v8327) / v119)))).ln()))) / ((v1 / v8325) + ((v8289 / v287) * (rspice_limited_exp(((v8344 * v8283) / v8326)))));
                        v8362 = v8350;
                    } else {
                        let v8361 = ((v8326 * v8289) * v8327) / ((v1 / v8325) + ((v8289 / v287) * (rspice_limited_exp(((v8355 * v8283) / v8326)))));
                        v8362 = v8361;
                    }
                    let v8364 = v8283 - (v8362 / v8289);
                    let v8367 = if ((v8364 - v8283).abs()) > v394 { 1.0 } else { 0.0 };
                    let v8497: f64;
                    if v8367 != 0.0 {
                        let v8368 = v8283 - v8364;
                        let v8375 = (v53 * v8368) + (v53 * (((v8368 * v8368) + v8371).sqrt()));
                        let v8376 = v8289.powf(v340);
                        let v8377 = v8375.powf(v340);
                        let v8379 = v8375.powf(v8378);
                        let v8380 = v6990 * v8376;
                        let v8382 = v7060 * v8376;
                        let v8384 = v8364 / v8265;
                        let v8386 = v8384 - ((v8380 * v8377) / v8265);
                        let v8388 = v8384 - ((v8382 * v8377) / v8265);
                        let v8389 = if v8386 >= v418 { 1.0 } else { 0.0 };
                        let v8403: f64;
                        if v8389 != 0.0 {
                            v8403 = v8386;
                        } else {
                            let v8391 = if v8386 <= v8390 { 1.0 } else { 0.0 };
                            let v8404: f64;
                            if v8391 != 0.0 {
                                v8404 = v0;
                            } else {
                                let v8394 = ((v8386.exp()) + v1).ln();
                                v8404 = v8394;
                            }
                            v8403 = v8404;
                        }
                        let v8395 = if v8388 >= v418 { 1.0 } else { 0.0 };
                        let v8407: f64;
                        if v8395 != 0.0 {
                            v8407 = v8388;
                        } else {
                            let v8397 = if v8388 <= v8396 { 1.0 } else { 0.0 };
                            let v8408: f64;
                            if v8397 != 0.0 {
                                v8408 = v0;
                            } else {
                                let v8400 = ((v8388.exp()) + v1).ln();
                                v8408 = v8400;
                            }
                            v8407 = v8408;
                        }
                        let v8402 = v287 * v8265;
                        let v8413 = rspice_limited_exp(v8386);
                        let v8419 = rspice_limited_exp(v8388);
                        let v8432 = v8364 - ((((v8289 * v8375) - (v8402 * v8403)) - (v8402 * v8407)) / (((v8425 * v8289) - (((v8413 * v287) * (v1 + (v340 * (v8380 * v8379)))) / (v1 + v8413))) - (((v8419 * v287) * (v1 + (v340 * (v8382 * v8379)))) / (v1 + v8419))));
                        let v8433 = v8283 - v8432;
                        let v8440 = (v53 * v8433) + (v53 * (((v8433 * v8433) + v8436).sqrt()));
                        let v8442 = v8440.powf(v8441);
                        let v8443 = v8440.powf(v340);
                        let v8446 = v8432 / v8265;
                        let v8448 = v8446 - ((v8380 * v8443) / v8265);
                        let v8450 = v8446 - ((v8382 * v8443) / v8265);
                        let v8451 = if v8448 >= v418 { 1.0 } else { 0.0 };
                        let v8464: f64;
                        if v8451 != 0.0 {
                            v8464 = v8448;
                        } else {
                            let v8453 = if v8448 <= v8452 { 1.0 } else { 0.0 };
                            let v8465: f64;
                            if v8453 != 0.0 {
                                v8465 = v0;
                            } else {
                                let v8456 = ((v8448.exp()) + v1).ln();
                                v8465 = v8456;
                            }
                            v8464 = v8465;
                        }
                        let v8457 = if v8450 >= v418 { 1.0 } else { 0.0 };
                        let v8468: f64;
                        if v8457 != 0.0 {
                            v8468 = v8450;
                        } else {
                            let v8459 = if v8450 <= v8458 { 1.0 } else { 0.0 };
                            let v8469: f64;
                            if v8459 != 0.0 {
                                v8469 = v0;
                            } else {
                                let v8462 = ((v8450.exp()) + v1).ln();
                                v8469 = v8462;
                            }
                            v8468 = v8469;
                        }
                        let v8474 = rspice_limited_exp(v8448);
                        let v8480 = rspice_limited_exp(v8450);
                        let v8493 = v8432 - ((((v8289 * v8440) - (v8402 * v8464)) - (v8402 * v8468)) / (((v8486 * v8289) - (((v8474 * v287) * (v1 + (v340 * (v8380 * v8442)))) / (v1 + v8474))) - (((v8480 * v287) * (v1 + (v340 * (v8382 * v8442)))) / (v1 + v8480))));
                        v8497 = v8493;
                    } else {
                        v8497 = v8364;
                    }
                    let v8500 = (v8268 / v217) * ((v8283 - v8497).abs());
                    let v8518 = v8290 + (v53 * ((v8291 + v8514).sqrt()));
                    let v8519 = ((v93 * (v7175 * v540)) / ((v7173 * v537) / (((v1 + (v551 * v8500)) + (v554 * (v8500 * v8500))) + (v558 * (v547 * ((v279 - v8497).abs())))))) * v6943;
                    let v8530 = v8283 - (v0 * ((v1 + ((v0 / ((v8519 * v8518) / (v8519 + v8518))).powf(v574))).powf((v8526 / v574))));
                    let v8537 = (v53 * v8530) + (v53 * (((v8530 * v8530) + v8533).sqrt()));
                    let v8539 = v8537 * v8537;
                    let v8553 = (v8289 * v8537).powf(v340);
                    let v8561 = ((v8537 + (v8265 * (v1 - ((v8286 * ((v8537 * v8287) / ((v8539 + v8299).sqrt()))).ln())))) - (v8313 * v8553)) / ((v8537 * (v1 + (v8265 / ((v8537 * v8288) / ((v8539 + v8304).sqrt()))))) + (v8322 * v8553));
                    let v8562 = v8530 / v8326;
                    let v8563 = if v8562 < v354 { 1.0 } else { 0.0 };
                    let v8597: f64;
                    if v8563 != 0.0 {
                        let v8585 = ((v8326 * v8289) * (((v110 * v8562) / v119) + (((rspice_limited_exp((v8562 / v119))) + (rspice_limited_exp(((v8566 * v8562) / v119)))).ln()))) / ((v1 / v8561) + ((v8289 / v287) * (rspice_limited_exp(((v8579 * v8530) / v8326)))));
                        v8597 = v8585;
                    } else {
                        let v8596 = ((v8326 * v8289) * v8562) / ((v1 / v8561) + ((v8289 / v287) * (rspice_limited_exp(((v8590 * v8530) / v8326)))));
                        v8597 = v8596;
                    }
                    let v8599 = v8530 - (v8597 / v8289);
                    let v8602 = if ((v8599 - v8530).abs()) > v394 { 1.0 } else { 0.0 };
                    if v8602 != 0.0 {
                        let v8603 = v8530 - v8599;
                        let v8610 = (v53 * v8603) + (v53 * (((v8603 * v8603) + v8606).sqrt()));
                        let v8611 = v8289.powf(v340);
                        let v8612 = v8610.powf(v340);
                        let v8614 = v8610.powf(v8613);
                        let v8615 = v6990 * v8611;
                        let v8617 = v7060 * v8611;
                        let v8619 = v8599 / v8265;
                        let v8621 = v8619 - ((v8615 * v8612) / v8265);
                        let v8623 = v8619 - ((v8617 * v8612) / v8265);
                        let v8624 = if v8621 >= v418 { 1.0 } else { 0.0 };
                        let v8638: f64;
                        if v8624 != 0.0 {
                            v8638 = v8621;
                        } else {
                            let v8626 = if v8621 <= v8625 { 1.0 } else { 0.0 };
                            let v8639: f64;
                            if v8626 != 0.0 {
                                v8639 = v0;
                            } else {
                                let v8629 = ((v8621.exp()) + v1).ln();
                                v8639 = v8629;
                            }
                            v8638 = v8639;
                        }
                        let v8630 = if v8623 >= v418 { 1.0 } else { 0.0 };
                        let v8642: f64;
                        if v8630 != 0.0 {
                            v8642 = v8623;
                        } else {
                            let v8632 = if v8623 <= v8631 { 1.0 } else { 0.0 };
                            let v8643: f64;
                            if v8632 != 0.0 {
                                v8643 = v0;
                            } else {
                                let v8635 = ((v8623.exp()) + v1).ln();
                                v8643 = v8635;
                            }
                            v8642 = v8643;
                        }
                        let v8637 = v287 * v8265;
                        let v8648 = rspice_limited_exp(v8621);
                        let v8654 = rspice_limited_exp(v8623);
                        let v8667 = v8599 - ((((v8289 * v8610) - (v8637 * v8638)) - (v8637 * v8642)) / (((v8660 * v8289) - (((v8648 * v287) * (v1 + (v340 * (v8615 * v8614)))) / (v1 + v8648))) - (((v8654 * v287) * (v1 + (v340 * (v8617 * v8614)))) / (v1 + v8654))));
                        let v8668 = v8530 - v8667;
                        let v8676 = ((v53 * v8668) + (v53 * (((v8668 * v8668) + v8671).sqrt()))).powf(v340);
                        let v8679 = v8667 / v8265;
                        let v8681 = v8679 - ((v8615 * v8676) / v8265);
                        let v8683 = v8679 - ((v8617 * v8676) / v8265);
                        let v8684 = if v8681 >= v418 { 1.0 } else { 0.0 };
                        if v8684 != 0.0 {
                        } else {
                            let v8686 = if v8681 <= v8685 { 1.0 } else { 0.0 };
                            if v8686 != 0.0 {
                            } else {
                            }
                        }
                        let v8687 = if v8683 >= v418 { 1.0 } else { 0.0 };
                        if v8687 != 0.0 {
                        } else {
                            let v8689 = if v8683 <= v8688 { 1.0 } else { 0.0 };
                            if v8689 != 0.0 {
                            } else {
                            }
                        }
                    } else {
                    }
                    v8847 = v8265;
                } else {
                    v8847 = v8826;
                }
                v8824 = v8847;
            }
            let v8691 = if v8690 == v1 { 1.0 } else { 0.0 };
            if v8691 != 0.0 {
                let v8702 = if ((v8692 * (v8693 + ((v283 / v110) / v8695))) / ((v8695 * v802) * v282)) > v0 { 1.0 } else { 0.0 };
                if v8702 != 0.0 {
                } else {
                }
            } else {
                let v8703 = if v8690 == v93 { 1.0 } else { 0.0 };
                if v8703 != 0.0 {
                    let v8709 = (v8695 * v802) * v282;
                    let v8714 = (v8692 * ((v284 / v110) / v8695)) / v8709;
                    let v8715 = if ((v8692 * (v8693 + ((v283 / v110) / v8695))) / v8709) > v0 { 1.0 } else { 0.0 };
                    if v8715 != 0.0 {
                    } else {
                    }
                    let v8717 = if v8714 > v0 { 1.0 } else { 0.0 };
                    if v8717 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            let v8718 = if v8690 == v93 { 1.0 } else { 0.0 };
            let v8762: f64;
            let v8765: f64;
            if v8718 != 0.0 {
                let v8722 = (v845 * v8719) * (v8716 - v35);
                let v8740 = (if ((v845 * v8730) - ((v845 * (if v8729 <= (v8730 / (v93 * v8723)) { v8729 } else { (v8730 / (v93 * v8723)) })) * ((v36 * v8723) / ((v37 + (v8723 * v8723)).sqrt())))) >= v0 { ((v845 * v8730) - ((v845 * (if v8729 <= (v8730 / (v93 * v8723)) { v8729 } else { (v8730 / (v93 * v8723)) })) * ((v36 * v8723) / ((v37 + (v8723 * v8723)).sqrt())))) } else { v0 }) * (v8716 - v34);
                v8762 = v8740;
                v8765 = v8722;
            } else {
                let v8743 = (v845 * v8719) * (v95 - v35);
                let v8758 = (if ((v845 * v8730) - ((v845 * (if v8729 <= (v8730 / (v93 * v8723)) { v8729 } else { (v8730 / (v93 * v8723)) })) * ((v36 * v8723) / ((v37 + (v8723 * v8723)).sqrt())))) >= v0 { ((v845 * v8730) - ((v845 * (if v8729 <= (v8730 / (v93 * v8723)) { v8729 } else { (v8730 / (v93 * v8723)) })) * ((v36 * v8723) / ((v37 + (v8723 * v8723)).sqrt())))) } else { v0 }) * (v95 - v34);
                v8762 = v8758;
                v8765 = v8743;
            }
            let v8761 = (v845 * v8759) * v36;
            let v8769 = v8768 + ((-v8762) + v8761);
            let v8771 = v8770 + ((-v8765) - v8761);
            let v8775 = v8772 + (v264 * v8773);
            let v8779 = v8776 + (v264 * v8777);
            let v8784 = v8780 * ((v8781 * v264).exp());
            let v8792 = v8789 + (v264 * v8790);
            let v8801 = if ((v34 - v22) - (v8785 + (v264 * v8786))) >= v0 { ((v34 - v22) - (v8785 + (v264 * v8786))) } else { v0 };
            let v8802 = if (v845 * (v8793 * ((v8794 * v264).exp()))) > v0 { 1.0 } else { 0.0 };
            if v8802 != 0.0 {
                let v8803 = if v8801 > v0 { 1.0 } else { 0.0 };
                if v8803 != 0.0 {
                    let v8806 = if (v8801 / (v8792 * v47)) > v838 { 1.0 } else { 0.0 };
                    if v8806 != 0.0 {
                    } else {
                    }
                } else {
                    let v8809 = if (v8801 / (v8792 * v47)) > v838 { 1.0 } else { 0.0 };
                    if v8809 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v8812 = if ((v35 - v22) - v8775) >= v0 { ((v35 - v22) - v8775) } else { v0 };
            let v8814 = if (v845 * v8784) > v0 { 1.0 } else { 0.0 };
            if v8814 != 0.0 {
                let v8815 = if v8812 > v0 { 1.0 } else { 0.0 };
                if v8815 != 0.0 {
                    let v8818 = if (v8812 / (v8779 * v47)) > v838 { 1.0 } else { 0.0 };
                    if v8818 != 0.0 {
                    } else {
                    }
                } else {
                    let v8821 = if (v8812 / (v8779 * v47)) > v838 { 1.0 } else { 0.0 };
                    if v8821 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v8823 = if v8822 == v1 { 1.0 } else { 0.0 };
            let v8967: f64;
            let v8968: f64;
            let v8969: f64;
            if v8823 != 0.0 {
                let v8862 = if v8769 >= v8861 { v8769 } else { v8861 };
                let v8865 = if v8771 >= v8861 { v8771 } else { v8861 };
                let v8895 = ((((((v8824 * v285) * v285) * v285) / ((v845 * v282) * v282)) * (v817 * v817)) * ((v282 / ((v788 + v8824) * (if v787 >= v123 { v787 } else { v123 }))) / (v219 * v219))) * (((((((v8858 * v8824) * v219) * (v1 / v8862)) * (v1 - (v8769 / v8865))) + ((v8858 + ((v8869 * v8824) * v219)) * ((v8862 / v8865).ln()))) + ((v8869 + ((v8876 * v8824) * v219)) * (v8771 - v8769))) + ((v8876 / v93) * ((v8769 * v8769) - (v8771 * v8771))));
                let v8897: f64;
                if v896 != 0.0 {
                    let v8896 = -v8895;
                    v8897 = v8896;
                } else {
                    v8897 = v8895;
                }
                let v8899 = v8897 * v8898;
                v8967 = v1;
                v8968 = v8899;
                v8969 = v8900;
            } else {
                v8967 = v0;
                v8968 = v0;
                v8969 = v0;
            }
            if v8718 != 0.0 {
            } else {
            }
            if v1539 != 0.0 {
                let v8901 = if v1540 != v0 { 1.0 } else { 0.0 };
                if v8901 != 0.0 {
                    let v8902 = if v1540 == v1 { 1.0 } else { 0.0 };
                    if v8902 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v8903 = if v1540 != v0 { 1.0 } else { 0.0 };
                if v8903 != 0.0 {
                    let v8904 = if v1540 == v1 { 1.0 } else { 0.0 };
                    if v8904 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if v1539 != 0.0 {
                let v8905 = if v2440 != v0 { 1.0 } else { 0.0 };
                if v8905 != 0.0 {
                    let v8906 = if v2440 == v1 { 1.0 } else { 0.0 };
                    if v8906 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v8907 = if v2440 != v0 { 1.0 } else { 0.0 };
                if v8907 != 0.0 {
                    let v8908 = if v2440 == v1 { 1.0 } else { 0.0 };
                    if v8908 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if v1539 != 0.0 {
                let v8909 = if v3325 != v0 { 1.0 } else { 0.0 };
                if v8909 != 0.0 {
                    let v8910 = if v3325 == v1 { 1.0 } else { 0.0 };
                    if v8910 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v8911 = if v3325 != v0 { 1.0 } else { 0.0 };
                if v8911 != 0.0 {
                    let v8912 = if v3325 == v1 { 1.0 } else { 0.0 };
                    if v8912 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if v1539 != 0.0 {
                let v8913 = if v4226 != v0 { 1.0 } else { 0.0 };
                if v8913 != 0.0 {
                    let v8914 = if v4226 == v1 { 1.0 } else { 0.0 };
                    if v8914 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v8915 = if v4226 != v0 { 1.0 } else { 0.0 };
                if v8915 != 0.0 {
                    let v8916 = if v4226 == v1 { 1.0 } else { 0.0 };
                    if v8916 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if v1539 != 0.0 {
                let v8917 = if v5114 != v0 { 1.0 } else { 0.0 };
                if v8917 != 0.0 {
                    let v8918 = if v5114 == v1 { 1.0 } else { 0.0 };
                    if v8918 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v8919 = if v5114 != v0 { 1.0 } else { 0.0 };
                if v8919 != 0.0 {
                    let v8920 = if v5114 == v1 { 1.0 } else { 0.0 };
                    if v8920 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if v1539 != 0.0 {
                let v8921 = if v6015 != v0 { 1.0 } else { 0.0 };
                if v8921 != 0.0 {
                    let v8922 = if v6015 == v1 { 1.0 } else { 0.0 };
                    if v8922 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v8923 = if v6015 != v0 { 1.0 } else { 0.0 };
                if v8923 != 0.0 {
                    let v8924 = if v6015 == v1 { 1.0 } else { 0.0 };
                    if v8924 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if v1539 != 0.0 {
                let v8925 = if v6903 != v0 { 1.0 } else { 0.0 };
                if v8925 != 0.0 {
                    let v8926 = if v6903 == v1 { 1.0 } else { 0.0 };
                    if v8926 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v8927 = if v6903 != v0 { 1.0 } else { 0.0 };
                if v8927 != 0.0 {
                    let v8928 = if v6903 == v1 { 1.0 } else { 0.0 };
                    if v8928 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if v1539 != 0.0 {
                let v8929 = if v7803 != v0 { 1.0 } else { 0.0 };
                if v8929 != 0.0 {
                    let v8930 = if v7803 == v1 { 1.0 } else { 0.0 };
                    if v8930 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v8931 = if v7803 != v0 { 1.0 } else { 0.0 };
                if v8931 != 0.0 {
                    let v8932 = if v7803 == v1 { 1.0 } else { 0.0 };
                    if v8932 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            let v8935 = if (if v4 == v1 { 1.0 } else { 0.0 }) != 0.0 && (if v6 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v8935 != 0.0 {
            } else {
            }
            let v8936 = if v844 > v0 { 1.0 } else { 0.0 };
            if v8936 != 0.0 {
            } else {
            }
        if v8937 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v8938;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v8939 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v8942;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v8945 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v8948;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v8951 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v8954;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v8957 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v8960;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v8963 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v8964;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v8965 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v8966;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v8967 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v8968;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            let psd = psd.abs();
            let exponent: Option<f64> = Some(v8969);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

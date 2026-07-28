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
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 84, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI2_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 85, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "di2", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI2_DI1_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 86, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "di2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "di1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI1_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 87, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI1_SI_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 88, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "si1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GE_RG", label: Some("rg"), kind: GeneratedNoiseKind::White, equation: 89, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "ge", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 90, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 93, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(16), name: "n", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 94, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS_V", label: Some("igs_v"), kind: GeneratedNoiseKind::White, equation: 98, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD_V", label: Some("igd_v"), kind: GeneratedNoiseKind::White, equation: 99, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD_V", label: Some("igd_v"), kind: GeneratedNoiseKind::White, equation: 100, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS_V", label: Some("igs_v"), kind: GeneratedNoiseKind::White, equation: 101, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_E_IGB", label: Some("igb"), kind: GeneratedNoiseKind::White, equation: 102, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "e", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGBS_V", label: Some("igbs_v"), kind: GeneratedNoiseKind::White, equation: 103, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGBD_V", label: Some("igbd_v"), kind: GeneratedNoiseKind::White, equation: 104, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(11), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13]), ctx.node_voltage(self.nodes[14]), ctx.node_voltage(self.nodes[15]), ctx.node_voltage(self.nodes[16])];
            let v0 = 0e0f64;
            let v1 = 1e0f64;
            let v2 = 1.0f64;
            let v3 = parameters[74];
            let v5 = parameters[1791];
            let v8 = parameters[81];
            let v11 = parameters[60];
            let v13 = -1e0f64;
            let v14 = parameters[103];
            let v15 = 8.8542e-12f64;
            let v17 = parameters[1088];
            let v19 = parameters[102];
            let v20 = parameters[91];
            let v22 = 4e0f64;
            let v23 = 2e0f64;
            let v24 = parameters[109];
            let v25 = 1e-6f64;
            let v26 = parameters[110];
            let v28 = parameters[0];
            let v31 = parameters[111];
            let v32 = parameters[5];
            let v35 = parameters[112];
            let v40 = parameters[117];
            let v41 = parameters[118];
            let v45 = parameters[119];
            let v48 = parameters[120];
            let v52 = parameters[113];
            let v53 = parameters[114];
            let v57 = parameters[115];
            let v60 = parameters[116];
            let v67 = parameters[84];
            let v70 = parameters[83];
            let v77 = parameters[85];
            let v78 = parameters[88];
            let v87 = parameters[86];
            let v90 = 1e-9f64;
            let v96 = parameters[61];
            let v100 = parameters[62];
            let v101 = 5e0f64;
            let v103 = parameters[121];
            let v104 = parameters[122];
            let v108 = parameters[123];
            let v111 = parameters[124];
            let v115 = parameters[125];
            let v117 = parameters[43];
            let v120 = parameters[126];
            let v121 = 1e-12f64;
            let v126 = parameters[127];
            let v127 = parameters[128];
            let v131 = parameters[129];
            let v134 = parameters[130];
            let v138 = parameters[131];
            let v142 = parameters[132];
            let v152 = parameters[59];
            let v164 = parameters[133];
            let v165 = parameters[134];
            let v168 = parameters[135];
            let v171 = parameters[136];
            let v180 = parameters[95];
            let v182 = parameters[96];
            let v185 = 1e-38f64;
            let v188 = -8.7498233534e1f64;
            let v196 = 1e22f64;
            let v197 = 1e18f64;
            let v200 = parameters[1802];
            let v202 = parameters[1803];
            let v205 = parameters[92];
            let v209 = parameters[89];
            let v211 = parameters[3];
            let v276 = 3e0f64;
            let v278 = 3.141592653589793e0f64;
            let v279 = parameters[2];
            let v286 = -8.7498233534e1f64;
            let v287 = 6.283185307179586e0f64;
            let v295 = parameters[1801];
            let v296 = parameters[1800];
            let v297 = parameters[1799];
            let v298 = parameters[40];
            let v301 = parameters[44];
            let v304 = parameters[45];
            let v306 = parameters[56];
            let v308 = parameters[46];
            let v310 = parameters[47];
            let v315 = parameters[48];
            let v317 = parameters[49];
            let v326 = parameters[50];
            let v328 = parameters[51];
            let v339 = parameters[52];
            let v341 = parameters[53];
            let v354 = parameters[54];
            let v356 = parameters[55];
            let v415 = 1.60219e-19f64;
            let v416 = -1.60219e-19f64;
            let v422 = parameters[93];
            let v424 = parameters[94];
            let v427 = parameters[87];
            let v433 = parameters[1085];
            let v434 = parameters[1680];
            let v435 = parameters[137];
            let v436 = parameters[138];
            let v439 = parameters[139];
            let v442 = parameters[140];
            let v445 = parameters[141];
            let v448 = parameters[142];
            let v451 = parameters[145];
            let v452 = parameters[146];
            let v455 = parameters[147];
            let v458 = parameters[148];
            let v461 = parameters[149];
            let v464 = parameters[150];
            let v467 = parameters[188];
            let v468 = parameters[189];
            let v471 = parameters[190];
            let v474 = parameters[191];
            let v477 = parameters[192];
            let v480 = parameters[193];
            let v483 = parameters[200];
            let v484 = parameters[201];
            let v487 = parameters[202];
            let v490 = parameters[203];
            let v493 = parameters[204];
            let v496 = parameters[205];
            let v499 = parameters[206];
            let v500 = parameters[207];
            let v503 = parameters[208];
            let v506 = parameters[209];
            let v509 = parameters[210];
            let v512 = parameters[211];
            let v515 = parameters[218];
            let v516 = parameters[219];
            let v519 = parameters[220];
            let v522 = parameters[221];
            let v525 = parameters[222];
            let v528 = parameters[223];
            let v531 = parameters[224];
            let v532 = parameters[225];
            let v535 = parameters[226];
            let v538 = parameters[227];
            let v541 = parameters[228];
            let v544 = parameters[229];
            let v547 = parameters[230];
            let v548 = parameters[231];
            let v551 = parameters[232];
            let v554 = parameters[233];
            let v557 = parameters[234];
            let v560 = parameters[235];
            let v563 = parameters[236];
            let v564 = parameters[237];
            let v567 = parameters[238];
            let v570 = parameters[239];
            let v573 = parameters[240];
            let v576 = parameters[241];
            let v579 = parameters[242];
            let v580 = parameters[243];
            let v583 = parameters[244];
            let v586 = parameters[245];
            let v589 = parameters[246];
            let v592 = parameters[247];
            let v595 = parameters[248];
            let v596 = parameters[249];
            let v599 = parameters[250];
            let v602 = parameters[251];
            let v605 = parameters[252];
            let v608 = parameters[253];
            let v611 = parameters[266];
            let v612 = parameters[267];
            let v615 = parameters[268];
            let v618 = parameters[269];
            let v621 = parameters[270];
            let v624 = parameters[271];
            let v627 = parameters[272];
            let v628 = parameters[273];
            let v631 = parameters[274];
            let v634 = parameters[275];
            let v637 = parameters[276];
            let v640 = parameters[277];
            let v643 = parameters[278];
            let v644 = parameters[279];
            let v647 = parameters[280];
            let v650 = parameters[281];
            let v653 = parameters[282];
            let v656 = parameters[283];
            let v659 = parameters[284];
            let v660 = parameters[285];
            let v663 = parameters[286];
            let v666 = parameters[287];
            let v669 = parameters[288];
            let v672 = parameters[289];
            let v675 = parameters[296];
            let v676 = parameters[297];
            let v679 = parameters[298];
            let v682 = parameters[299];
            let v685 = parameters[300];
            let v688 = parameters[301];
            let v691 = parameters[302];
            let v692 = parameters[303];
            let v695 = parameters[304];
            let v698 = parameters[305];
            let v701 = parameters[306];
            let v704 = parameters[307];
            let v707 = parameters[308];
            let v708 = parameters[309];
            let v711 = parameters[310];
            let v714 = parameters[311];
            let v717 = parameters[312];
            let v720 = parameters[313];
            let v723 = parameters[314];
            let v724 = parameters[315];
            let v727 = parameters[316];
            let v730 = parameters[317];
            let v733 = parameters[318];
            let v736 = parameters[319];
            let v739 = parameters[320];
            let v740 = parameters[321];
            let v743 = parameters[322];
            let v746 = parameters[323];
            let v749 = parameters[324];
            let v752 = parameters[325];
            let v755 = parameters[326];
            let v756 = parameters[327];
            let v759 = parameters[328];
            let v762 = parameters[329];
            let v765 = parameters[330];
            let v768 = parameters[331];
            let v771 = parameters[332];
            let v772 = parameters[333];
            let v775 = parameters[334];
            let v778 = parameters[335];
            let v781 = parameters[336];
            let v784 = parameters[337];
            let v787 = parameters[338];
            let v788 = parameters[339];
            let v791 = parameters[340];
            let v794 = parameters[341];
            let v797 = parameters[342];
            let v800 = parameters[343];
            let v803 = parameters[344];
            let v804 = parameters[345];
            let v807 = parameters[346];
            let v810 = parameters[347];
            let v813 = parameters[348];
            let v816 = parameters[349];
            let v819 = parameters[350];
            let v820 = parameters[351];
            let v823 = parameters[352];
            let v826 = parameters[353];
            let v829 = parameters[354];
            let v832 = parameters[355];
            let v835 = parameters[403];
            let v836 = parameters[404];
            let v839 = parameters[405];
            let v842 = parameters[406];
            let v845 = parameters[407];
            let v848 = parameters[408];
            let v851 = parameters[409];
            let v852 = parameters[410];
            let v855 = parameters[411];
            let v858 = parameters[412];
            let v861 = parameters[413];
            let v864 = parameters[414];
            let v867 = parameters[415];
            let v868 = parameters[416];
            let v871 = parameters[417];
            let v874 = parameters[418];
            let v877 = parameters[419];
            let v880 = parameters[420];
            let v883 = parameters[421];
            let v884 = parameters[422];
            let v887 = parameters[423];
            let v890 = parameters[424];
            let v893 = parameters[425];
            let v896 = parameters[426];
            let v899 = parameters[455];
            let v900 = parameters[456];
            let v903 = parameters[457];
            let v906 = parameters[458];
            let v909 = parameters[459];
            let v912 = parameters[460];
            let v915 = parameters[467];
            let v916 = parameters[468];
            let v919 = parameters[469];
            let v922 = parameters[470];
            let v925 = parameters[471];
            let v928 = parameters[472];
            let v931 = parameters[506];
            let v932 = parameters[507];
            let v935 = parameters[508];
            let v938 = parameters[509];
            let v941 = parameters[510];
            let v944 = parameters[511];
            let v947 = parameters[512];
            let v948 = parameters[513];
            let v951 = parameters[514];
            let v954 = parameters[515];
            let v957 = parameters[516];
            let v960 = parameters[517];
            let v963 = parameters[479];
            let v964 = parameters[480];
            let v967 = parameters[481];
            let v970 = parameters[482];
            let v973 = parameters[483];
            let v976 = parameters[484];
            let v979 = parameters[485];
            let v980 = parameters[486];
            let v983 = parameters[487];
            let v986 = parameters[488];
            let v989 = parameters[489];
            let v992 = parameters[490];
            let v995 = parameters[518];
            let v996 = parameters[519];
            let v999 = parameters[520];
            let v1002 = parameters[521];
            let v1005 = parameters[522];
            let v1008 = parameters[523];
            let v1011 = parameters[524];
            let v1012 = parameters[525];
            let v1015 = parameters[526];
            let v1018 = parameters[527];
            let v1021 = parameters[528];
            let v1024 = parameters[529];
            let v1027 = parameters[492];
            let v1028 = parameters[493];
            let v1031 = parameters[494];
            let v1034 = parameters[495];
            let v1037 = parameters[496];
            let v1040 = parameters[497];
            let v1043 = parameters[531];
            let v1044 = parameters[532];
            let v1047 = parameters[533];
            let v1050 = parameters[534];
            let v1053 = parameters[535];
            let v1056 = parameters[536];
            let v1059 = parameters[543];
            let v1060 = parameters[544];
            let v1063 = parameters[545];
            let v1066 = parameters[546];
            let v1069 = parameters[547];
            let v1072 = parameters[548];
            let v1075 = parameters[605];
            let v1076 = parameters[606];
            let v1079 = parameters[607];
            let v1082 = parameters[608];
            let v1085 = parameters[609];
            let v1088 = parameters[610];
            let v1091 = parameters[623];
            let v1092 = parameters[624];
            let v1095 = parameters[625];
            let v1098 = parameters[626];
            let v1101 = parameters[627];
            let v1104 = parameters[628];
            let v1107 = parameters[629];
            let v1108 = parameters[630];
            let v1111 = parameters[631];
            let v1114 = parameters[632];
            let v1117 = parameters[633];
            let v1120 = parameters[634];
            let v1123 = parameters[641];
            let v1124 = parameters[642];
            let v1127 = parameters[643];
            let v1130 = parameters[644];
            let v1133 = parameters[645];
            let v1136 = parameters[646];
            let v1139 = parameters[677];
            let v1140 = parameters[678];
            let v1143 = parameters[679];
            let v1146 = parameters[680];
            let v1149 = parameters[681];
            let v1152 = parameters[682];
            let v1155 = parameters[689];
            let v1156 = parameters[690];
            let v1159 = parameters[691];
            let v1162 = parameters[692];
            let v1165 = parameters[693];
            let v1168 = parameters[694];
            let v1171 = parameters[707];
            let v1172 = parameters[708];
            let v1175 = parameters[709];
            let v1178 = parameters[710];
            let v1181 = parameters[711];
            let v1184 = parameters[712];
            let v1187 = parameters[713];
            let v1188 = parameters[714];
            let v1191 = parameters[715];
            let v1194 = parameters[716];
            let v1197 = parameters[717];
            let v1200 = parameters[718];
            let v1203 = parameters[719];
            let v1204 = parameters[720];
            let v1207 = parameters[721];
            let v1210 = parameters[722];
            let v1213 = parameters[723];
            let v1216 = parameters[724];
            let v1219 = parameters[725];
            let v1220 = parameters[726];
            let v1223 = parameters[727];
            let v1226 = parameters[728];
            let v1229 = parameters[729];
            let v1232 = parameters[730];
            let v1235 = parameters[731];
            let v1236 = parameters[732];
            let v1239 = parameters[733];
            let v1242 = parameters[734];
            let v1245 = parameters[735];
            let v1248 = parameters[736];
            let v1251 = parameters[1025];
            let v1252 = parameters[1027];
            let v1255 = parameters[1028];
            let v1258 = parameters[1029];
            let v1261 = parameters[1030];
            let v1264 = parameters[1031];
            let v1267 = parameters[1038];
            let v1268 = parameters[1039];
            let v1271 = parameters[1040];
            let v1274 = parameters[1041];
            let v1277 = parameters[1042];
            let v1280 = parameters[1043];
            let v1283 = parameters[1044];
            let v1284 = parameters[1045];
            let v1287 = parameters[1046];
            let v1290 = parameters[1047];
            let v1293 = parameters[1048];
            let v1296 = parameters[1049];
            let v1299 = parameters[1050];
            let v1300 = parameters[1051];
            let v1303 = parameters[1052];
            let v1306 = parameters[1053];
            let v1309 = parameters[1054];
            let v1312 = parameters[1055];
            let v1315 = parameters[1056];
            let v1316 = parameters[1057];
            let v1319 = parameters[1058];
            let v1322 = parameters[1059];
            let v1325 = parameters[1060];
            let v1328 = parameters[1061];
            let v1331 = parameters[1062];
            let v1332 = parameters[1063];
            let v1335 = parameters[1064];
            let v1338 = parameters[1065];
            let v1341 = parameters[1066];
            let v1344 = parameters[1067];
            let v1347 = parameters[1068];
            let v1348 = parameters[1069];
            let v1351 = parameters[1070];
            let v1354 = parameters[1071];
            let v1357 = parameters[1072];
            let v1360 = parameters[1073];
            let v1363 = parameters[925];
            let v1364 = parameters[926];
            let v1367 = parameters[927];
            let v1370 = parameters[928];
            let v1373 = parameters[929];
            let v1376 = parameters[930];
            let v1379 = parameters[931];
            let v1380 = parameters[932];
            let v1383 = parameters[933];
            let v1386 = parameters[934];
            let v1389 = parameters[935];
            let v1392 = parameters[936];
            let v1395 = parameters[937];
            let v1396 = parameters[938];
            let v1399 = parameters[939];
            let v1402 = parameters[940];
            let v1405 = parameters[941];
            let v1408 = parameters[942];
            let v1411 = parameters[949];
            let v1412 = parameters[950];
            let v1415 = parameters[951];
            let v1418 = parameters[952];
            let v1421 = parameters[953];
            let v1424 = parameters[954];
            let v1427 = parameters[943];
            let v1428 = parameters[944];
            let v1431 = parameters[945];
            let v1434 = parameters[946];
            let v1437 = parameters[947];
            let v1440 = parameters[948];
            let v1443 = parameters[955];
            let v1444 = parameters[956];
            let v1447 = parameters[957];
            let v1450 = parameters[958];
            let v1453 = parameters[959];
            let v1456 = parameters[960];
            let v1459 = parameters[985];
            let v1460 = parameters[986];
            let v1463 = parameters[987];
            let v1466 = parameters[988];
            let v1469 = parameters[989];
            let v1472 = parameters[990];
            let v1475 = parameters[991];
            let v1476 = parameters[992];
            let v1479 = parameters[993];
            let v1482 = parameters[994];
            let v1485 = parameters[995];
            let v1488 = parameters[996];
            let v1491 = parameters[1009];
            let v1492 = parameters[1010];
            let v1495 = parameters[1011];
            let v1498 = parameters[1012];
            let v1501 = parameters[1013];
            let v1504 = parameters[1014];
            let v1507 = parameters[1015];
            let v1508 = parameters[1016];
            let v1511 = parameters[1017];
            let v1514 = parameters[1018];
            let v1517 = parameters[1019];
            let v1520 = parameters[1020];
            let v1523 = parameters[1119];
            let v1524 = parameters[1120];
            let v1527 = parameters[1121];
            let v1530 = parameters[1122];
            let v1533 = parameters[1123];
            let v1536 = parameters[1124];
            let v1539 = parameters[1125];
            let v1540 = parameters[1126];
            let v1543 = parameters[1127];
            let v1546 = parameters[1128];
            let v1549 = parameters[1129];
            let v1552 = parameters[1130];
            let v1555 = parameters[1131];
            let v1556 = parameters[1132];
            let v1559 = parameters[1133];
            let v1562 = parameters[1134];
            let v1565 = parameters[1135];
            let v1568 = parameters[1136];
            let v1571 = parameters[1137];
            let v1572 = parameters[1138];
            let v1575 = parameters[1139];
            let v1578 = parameters[1140];
            let v1581 = parameters[1141];
            let v1584 = parameters[1142];
            let v1587 = parameters[1143];
            let v1588 = parameters[1144];
            let v1591 = parameters[1145];
            let v1594 = parameters[1146];
            let v1597 = parameters[1147];
            let v1600 = parameters[1148];
            let v1603 = parameters[1149];
            let v1604 = parameters[1150];
            let v1607 = parameters[1151];
            let v1610 = parameters[1152];
            let v1613 = parameters[1153];
            let v1616 = parameters[1154];
            let v1619 = parameters[1155];
            let v1620 = parameters[1156];
            let v1623 = parameters[1157];
            let v1626 = parameters[1158];
            let v1629 = parameters[1159];
            let v1632 = parameters[1160];
            let v1635 = parameters[1161];
            let v1636 = parameters[1162];
            let v1639 = parameters[1163];
            let v1642 = parameters[1164];
            let v1645 = parameters[1165];
            let v1648 = parameters[1166];
            let v1651 = parameters[1167];
            let v1652 = parameters[1168];
            let v1655 = parameters[1169];
            let v1658 = parameters[1170];
            let v1661 = parameters[1171];
            let v1664 = parameters[1172];
            let v1667 = parameters[1173];
            let v1668 = parameters[1174];
            let v1671 = parameters[1175];
            let v1674 = parameters[1176];
            let v1677 = parameters[1177];
            let v1680 = parameters[1178];
            let v1683 = parameters[1179];
            let v1684 = parameters[1180];
            let v1687 = parameters[1181];
            let v1690 = parameters[1182];
            let v1693 = parameters[1183];
            let v1696 = parameters[1184];
            let v1699 = parameters[1185];
            let v1700 = parameters[1186];
            let v1703 = parameters[1187];
            let v1706 = parameters[1188];
            let v1709 = parameters[1189];
            let v1712 = parameters[1190];
            let v1715 = parameters[1191];
            let v1716 = parameters[1192];
            let v1719 = parameters[1193];
            let v1722 = parameters[1194];
            let v1725 = parameters[1195];
            let v1728 = parameters[1196];
            let v1731 = parameters[1197];
            let v1732 = parameters[1198];
            let v1735 = parameters[1199];
            let v1738 = parameters[1200];
            let v1741 = parameters[1201];
            let v1744 = parameters[1202];
            let v1747 = parameters[1203];
            let v1748 = parameters[1204];
            let v1751 = parameters[1205];
            let v1754 = parameters[1206];
            let v1757 = parameters[1207];
            let v1760 = parameters[1208];
            let v1763 = parameters[1209];
            let v1764 = parameters[1210];
            let v1767 = parameters[1211];
            let v1770 = parameters[1212];
            let v1773 = parameters[1213];
            let v1776 = parameters[1214];
            let v1779 = parameters[1215];
            let v1780 = parameters[1216];
            let v1783 = parameters[1217];
            let v1786 = parameters[1218];
            let v1789 = parameters[1219];
            let v1792 = parameters[1220];
            let v1795 = parameters[1221];
            let v1796 = parameters[1222];
            let v1799 = parameters[1223];
            let v1802 = parameters[1224];
            let v1805 = parameters[1225];
            let v1808 = parameters[1226];
            let v1811 = parameters[1227];
            let v1812 = parameters[1228];
            let v1815 = parameters[1229];
            let v1818 = parameters[1230];
            let v1821 = parameters[1231];
            let v1824 = parameters[1232];
            let v1827 = parameters[1233];
            let v1828 = parameters[1234];
            let v1831 = parameters[1235];
            let v1834 = parameters[1236];
            let v1837 = parameters[1237];
            let v1840 = parameters[1238];
            let v1843 = parameters[1239];
            let v1844 = parameters[1240];
            let v1847 = parameters[1241];
            let v1850 = parameters[1242];
            let v1853 = parameters[1243];
            let v1856 = parameters[1244];
            let v1859 = parameters[1245];
            let v1860 = parameters[1246];
            let v1863 = parameters[1247];
            let v1866 = parameters[1248];
            let v1869 = parameters[1249];
            let v1872 = parameters[1250];
            let v1875 = parameters[1251];
            let v1876 = parameters[1252];
            let v1879 = parameters[1253];
            let v1882 = parameters[1254];
            let v1885 = parameters[1255];
            let v1888 = parameters[1256];
            let v1891 = parameters[1257];
            let v1892 = parameters[1258];
            let v1895 = parameters[1259];
            let v1898 = parameters[1260];
            let v1901 = parameters[1261];
            let v1904 = parameters[1262];
            let v1907 = parameters[1113];
            let v1908 = parameters[1114];
            let v1911 = parameters[1115];
            let v1914 = parameters[1116];
            let v1917 = parameters[1117];
            let v1920 = parameters[1118];
            let v1923 = parameters[1263];
            let v1924 = parameters[1264];
            let v1927 = parameters[1265];
            let v1930 = parameters[1266];
            let v1933 = parameters[1267];
            let v1936 = parameters[1268];
            let v1939 = parameters[1269];
            let v1940 = parameters[1270];
            let v1943 = parameters[1271];
            let v1946 = parameters[1272];
            let v1949 = parameters[1273];
            let v1952 = parameters[1274];
            let v1955 = parameters[1275];
            let v1956 = parameters[1276];
            let v1959 = parameters[1277];
            let v1962 = parameters[1278];
            let v1965 = parameters[1279];
            let v1968 = parameters[1280];
            let v1971 = parameters[1281];
            let v1972 = parameters[1282];
            let v1975 = parameters[1283];
            let v1978 = parameters[1284];
            let v1981 = parameters[1285];
            let v1984 = parameters[1286];
            let v1987 = parameters[1287];
            let v1988 = parameters[1288];
            let v1991 = parameters[1289];
            let v1994 = parameters[1290];
            let v1997 = parameters[1291];
            let v2000 = parameters[1292];
            let v2003 = parameters[1329];
            let v2004 = parameters[1330];
            let v2007 = parameters[1331];
            let v2010 = parameters[1332];
            let v2013 = parameters[1333];
            let v2016 = parameters[1334];
            let v2019 = parameters[1335];
            let v2020 = parameters[1336];
            let v2023 = parameters[1337];
            let v2026 = parameters[1338];
            let v2029 = parameters[1339];
            let v2032 = parameters[1340];
            let v2035 = parameters[1341];
            let v2036 = parameters[1342];
            let v2039 = parameters[1343];
            let v2042 = parameters[1344];
            let v2045 = parameters[1345];
            let v2048 = parameters[1346];
            let v2051 = parameters[1347];
            let v2052 = parameters[1348];
            let v2055 = parameters[1349];
            let v2058 = parameters[1350];
            let v2061 = parameters[1351];
            let v2064 = parameters[1352];
            let v2067 = parameters[1299];
            let v2068 = parameters[1300];
            let v2071 = parameters[1301];
            let v2074 = parameters[1302];
            let v2077 = parameters[1303];
            let v2080 = parameters[1304];
            let v2083 = parameters[1305];
            let v2084 = parameters[1306];
            let v2087 = parameters[1307];
            let v2090 = parameters[1308];
            let v2093 = parameters[1309];
            let v2096 = parameters[1310];
            let v2099 = parameters[1311];
            let v2100 = parameters[1312];
            let v2103 = parameters[1313];
            let v2106 = parameters[1314];
            let v2109 = parameters[1315];
            let v2112 = parameters[1316];
            let v2115 = parameters[1317];
            let v2116 = parameters[1318];
            let v2119 = parameters[1319];
            let v2122 = parameters[1320];
            let v2125 = parameters[1321];
            let v2128 = parameters[1322];
            let v2131 = parameters[1353];
            let v2132 = parameters[1354];
            let v2135 = parameters[1355];
            let v2138 = parameters[1356];
            let v2141 = parameters[1357];
            let v2144 = parameters[1358];
            let v2147 = parameters[1359];
            let v2148 = parameters[1360];
            let v2151 = parameters[1361];
            let v2154 = parameters[1362];
            let v2157 = parameters[1363];
            let v2160 = parameters[1364];
            let v2163 = parameters[1365];
            let v2164 = parameters[1366];
            let v2167 = parameters[1367];
            let v2170 = parameters[1368];
            let v2173 = parameters[1369];
            let v2176 = parameters[1370];
            let v2179 = parameters[1371];
            let v2180 = parameters[1372];
            let v2183 = parameters[1373];
            let v2186 = parameters[1374];
            let v2189 = parameters[1375];
            let v2192 = parameters[1376];
            let v2195 = parameters[1444];
            let v2196 = parameters[1445];
            let v2199 = parameters[1446];
            let v2202 = parameters[1447];
            let v2205 = parameters[1448];
            let v2208 = parameters[1449];
            let v2211 = parameters[1450];
            let v2212 = parameters[1451];
            let v2215 = parameters[1452];
            let v2218 = parameters[1453];
            let v2221 = parameters[1454];
            let v2224 = parameters[1455];
            let v2227 = parameters[1462];
            let v2228 = parameters[1463];
            let v2231 = parameters[1464];
            let v2234 = parameters[1465];
            let v2237 = parameters[1466];
            let v2240 = parameters[1467];
            let v2243 = parameters[1468];
            let v2244 = parameters[1469];
            let v2247 = parameters[1470];
            let v2250 = parameters[1471];
            let v2253 = parameters[1472];
            let v2256 = parameters[1473];
            let v2259 = parameters[1456];
            let v2260 = parameters[1457];
            let v2263 = parameters[1458];
            let v2266 = parameters[1459];
            let v2269 = parameters[1460];
            let v2272 = parameters[1461];
            let v2275 = parameters[1474];
            let v2276 = parameters[1475];
            let v2279 = parameters[1476];
            let v2282 = parameters[1477];
            let v2285 = parameters[1478];
            let v2288 = parameters[1479];
            let v2291 = parameters[1480];
            let v2292 = parameters[1481];
            let v2295 = parameters[1482];
            let v2298 = parameters[1483];
            let v2301 = parameters[1484];
            let v2304 = parameters[1485];
            let v2307 = parameters[1486];
            let v2308 = parameters[1487];
            let v2311 = parameters[1488];
            let v2314 = parameters[1489];
            let v2317 = parameters[1490];
            let v2320 = parameters[1491];
            let v2323 = parameters[1492];
            let v2324 = parameters[1493];
            let v2327 = parameters[1494];
            let v2330 = parameters[1495];
            let v2333 = parameters[1496];
            let v2336 = parameters[1497];
            let v2339 = parameters[1498];
            let v2340 = parameters[1499];
            let v2343 = parameters[1500];
            let v2346 = parameters[1501];
            let v2349 = parameters[1502];
            let v2352 = parameters[1503];
            let v2355 = parameters[1510];
            let v2356 = parameters[1511];
            let v2359 = parameters[1512];
            let v2362 = parameters[1513];
            let v2365 = parameters[1514];
            let v2368 = parameters[1515];
            let v2371 = parameters[1516];
            let v2372 = parameters[1517];
            let v2375 = parameters[1518];
            let v2378 = parameters[1519];
            let v2381 = parameters[1520];
            let v2384 = parameters[1521];
            let v2387 = parameters[1522];
            let v2388 = parameters[1523];
            let v2391 = parameters[1524];
            let v2394 = parameters[1525];
            let v2397 = parameters[1526];
            let v2400 = parameters[1527];
            let v2403 = parameters[1762];
            let v2404 = parameters[1763];
            let v2407 = parameters[1764];
            let v2410 = parameters[1765];
            let v2413 = parameters[1766];
            let v2416 = parameters[1767];
            let v2419 = parameters[1530];
            let v2420 = parameters[1531];
            let v2423 = parameters[1532];
            let v2426 = parameters[1533];
            let v2429 = parameters[1534];
            let v2432 = parameters[1535];
            let v2435 = parameters[1536];
            let v2436 = parameters[1537];
            let v2439 = parameters[1538];
            let v2442 = parameters[1539];
            let v2445 = parameters[1540];
            let v2448 = parameters[1541];
            let v2451 = parameters[28];
            let v2452 = parameters[29];
            let v2455 = parameters[30];
            let v2458 = parameters[31];
            let v2461 = parameters[32];
            let v2464 = parameters[33];
            let v2467 = parameters[34];
            let v2468 = parameters[35];
            let v2471 = parameters[36];
            let v2474 = parameters[37];
            let v2477 = parameters[38];
            let v2480 = parameters[39];
            let v2483 = parameters[1547];
            let v2484 = parameters[1548];
            let v2487 = parameters[1549];
            let v2490 = parameters[1550];
            let v2493 = parameters[1551];
            let v2496 = parameters[1552];
            let v2499 = parameters[1553];
            let v2500 = parameters[1554];
            let v2503 = parameters[1555];
            let v2506 = parameters[1556];
            let v2509 = parameters[1557];
            let v2512 = parameters[1558];
            let v2515 = parameters[1559];
            let v2516 = parameters[1560];
            let v2519 = parameters[1561];
            let v2522 = parameters[1562];
            let v2525 = parameters[1563];
            let v2528 = parameters[1564];
            let v2531 = parameters[1565];
            let v2532 = parameters[1566];
            let v2535 = parameters[1567];
            let v2538 = parameters[1568];
            let v2541 = parameters[1569];
            let v2544 = parameters[1570];
            let v2547 = parameters[1571];
            let v2548 = parameters[1572];
            let v2551 = parameters[1573];
            let v2554 = parameters[1574];
            let v2557 = parameters[1575];
            let v2560 = parameters[1576];
            let v2563 = parameters[1577];
            let v2564 = parameters[1578];
            let v2567 = parameters[1579];
            let v2570 = parameters[1580];
            let v2573 = parameters[1581];
            let v2576 = parameters[1582];
            let v2579 = parameters[1650];
            let v2580 = parameters[1651];
            let v2583 = parameters[1652];
            let v2586 = parameters[1653];
            let v2589 = parameters[1654];
            let v2592 = parameters[1655];
            let v2595 = parameters[737];
            let v2596 = parameters[738];
            let v2599 = parameters[739];
            let v2602 = parameters[740];
            let v2605 = parameters[741];
            let v2608 = parameters[742];
            let v2611 = parameters[755];
            let v2612 = parameters[756];
            let v2615 = parameters[757];
            let v2618 = parameters[758];
            let v2621 = parameters[759];
            let v2624 = parameters[760];
            let v2627 = parameters[767];
            let v2628 = parameters[768];
            let v2631 = parameters[769];
            let v2634 = parameters[770];
            let v2637 = parameters[771];
            let v2640 = parameters[772];
            let v2643 = parameters[785];
            let v2644 = parameters[786];
            let v2647 = parameters[787];
            let v2650 = parameters[788];
            let v2653 = parameters[789];
            let v2656 = parameters[790];
            let v2659 = parameters[791];
            let v2660 = parameters[792];
            let v2663 = parameters[793];
            let v2666 = parameters[794];
            let v2669 = parameters[795];
            let v2672 = parameters[796];
            let v2675 = parameters[809];
            let v2676 = parameters[810];
            let v2679 = parameters[811];
            let v2682 = parameters[812];
            let v2685 = parameters[813];
            let v2688 = parameters[814];
            let v2691 = parameters[821];
            let v2692 = parameters[822];
            let v2695 = parameters[823];
            let v2698 = parameters[824];
            let v2701 = parameters[825];
            let v2704 = parameters[826];
            let v2707 = parameters[845];
            let v2708 = parameters[846];
            let v2711 = parameters[847];
            let v2714 = parameters[848];
            let v2717 = parameters[849];
            let v2720 = parameters[850];
            let v2723 = parameters[863];
            let v2724 = parameters[864];
            let v2727 = parameters[865];
            let v2730 = parameters[866];
            let v2733 = parameters[867];
            let v2736 = parameters[868];
            let v2739 = parameters[875];
            let v2740 = parameters[876];
            let v2743 = parameters[877];
            let v2746 = parameters[878];
            let v2749 = parameters[879];
            let v2752 = parameters[880];
            let v2755 = parameters[881];
            let v2756 = parameters[882];
            let v2759 = parameters[883];
            let v2762 = parameters[884];
            let v2765 = parameters[885];
            let v2768 = parameters[886];
            let v2771 = parameters[575];
            let v2772 = parameters[576];
            let v2775 = parameters[577];
            let v2778 = parameters[578];
            let v2781 = parameters[579];
            let v2784 = parameters[580];
            let v2787 = parameters[555];
            let v2788 = parameters[556];
            let v2791 = parameters[557];
            let v2794 = parameters[558];
            let v2797 = parameters[559];
            let v2800 = parameters[560];
            let v2803 = parameters[568];
            let v2804 = parameters[569];
            let v2807 = parameters[570];
            let v2810 = parameters[571];
            let v2813 = parameters[572];
            let v2816 = parameters[573];
            let v2819 = parameters[961];
            let v2820 = parameters[962];
            let v2823 = parameters[963];
            let v2826 = parameters[964];
            let v2829 = parameters[965];
            let v2832 = parameters[966];
            let v2835 = parameters[967];
            let v2836 = parameters[968];
            let v2839 = parameters[969];
            let v2842 = parameters[970];
            let v2845 = parameters[971];
            let v2848 = parameters[972];
            let v2851 = parameters[973];
            let v2852 = parameters[974];
            let v2855 = parameters[975];
            let v2858 = parameters[976];
            let v2861 = parameters[977];
            let v2864 = parameters[978];
            let v2867 = parameters[979];
            let v2868 = parameters[980];
            let v2871 = parameters[981];
            let v2874 = parameters[982];
            let v2877 = parameters[983];
            let v2880 = parameters[984];
            let v2883 = parameters[1741];
            let v2884 = parameters[1742];
            let v2887 = parameters[1743];
            let v2890 = parameters[1744];
            let v2893 = parameters[1745];
            let v2896 = parameters[1746];
            let v2899 = parameters[1750];
            let v2900 = parameters[1751];
            let v2903 = parameters[1752];
            let v2906 = parameters[1753];
            let v2909 = parameters[1754];
            let v2912 = parameters[1755];
            let v2915 = parameters[1756];
            let v2916 = parameters[1757];
            let v2919 = parameters[1758];
            let v2922 = parameters[1759];
            let v2925 = parameters[1760];
            let v2928 = parameters[1761];
            let v2931 = parameters[1768];
            let v2932 = parameters[1769];
            let v2935 = parameters[1770];
            let v2938 = parameters[1771];
            let v2941 = parameters[1772];
            let v2944 = parameters[1773];
            let v2947 = parameters[1774];
            let v2948 = parameters[1775];
            let v2951 = parameters[1776];
            let v2954 = parameters[1777];
            let v2957 = parameters[1778];
            let v2960 = parameters[1779];
            let v2963 = parameters[1780];
            let v2964 = parameters[1781];
            let v2967 = parameters[1782];
            let v2970 = parameters[1783];
            let v2973 = parameters[1784];
            let v2976 = parameters[1785];
            let v2979 = parameters[176];
            let v2980 = parameters[177];
            let v2983 = parameters[178];
            let v2986 = parameters[179];
            let v2989 = parameters[180];
            let v2992 = parameters[181];
            let v2995 = parameters[182];
            let v2996 = parameters[183];
            let v2999 = parameters[184];
            let v3002 = parameters[185];
            let v3005 = parameters[186];
            let v3008 = parameters[187];
            let v3011 = parameters[1689];
            let v3012 = parameters[1690];
            let v3015 = parameters[1691];
            let v3018 = parameters[1692];
            let v3021 = parameters[1693];
            let v3024 = parameters[1694];
            let v3027 = parameters[1701];
            let v3028 = parameters[1702];
            let v3031 = parameters[1703];
            let v3034 = parameters[1704];
            let v3037 = parameters[1705];
            let v3040 = parameters[1706];
            let v3043 = parameters[1695];
            let v3044 = parameters[1696];
            let v3047 = parameters[1697];
            let v3050 = parameters[1698];
            let v3053 = parameters[1699];
            let v3056 = parameters[1700];
            let v3059 = parameters[356];
            let v3060 = parameters[357];
            let v3063 = parameters[358];
            let v3066 = parameters[359];
            let v3069 = parameters[360];
            let v3072 = parameters[361];
            let v3075 = parameters[362];
            let v3076 = parameters[363];
            let v3079 = parameters[364];
            let v3082 = parameters[365];
            let v3085 = parameters[366];
            let v3088 = parameters[367];
            let v3091 = parameters[368];
            let v3092 = parameters[369];
            let v3095 = parameters[370];
            let v3098 = parameters[371];
            let v3101 = parameters[372];
            let v3104 = parameters[373];
            let v3107 = parameters[659];
            let v3108 = parameters[660];
            let v3111 = parameters[661];
            let v3114 = parameters[662];
            let v3117 = parameters[663];
            let v3120 = parameters[664];
            let v3123 = parameters[827];
            let v3124 = parameters[828];
            let v3127 = parameters[829];
            let v3130 = parameters[830];
            let v3133 = parameters[831];
            let v3136 = parameters[832];
            let v3140 = parameters[386];
            let v3141 = parameters[387];
            let v3144 = parameters[388];
            let v3147 = parameters[389];
            let v3150 = parameters[390];
            let v3153 = parameters[391];
            let v3156 = parameters[392];
            let v3157 = parameters[393];
            let v3160 = parameters[394];
            let v3163 = parameters[395];
            let v3166 = parameters[396];
            let v3169 = parameters[397];
            let v3172 = parameters[374];
            let v3173 = parameters[375];
            let v3176 = parameters[376];
            let v3179 = parameters[377];
            let v3182 = parameters[378];
            let v3185 = parameters[379];
            let v3188 = parameters[380];
            let v3189 = parameters[381];
            let v3192 = parameters[382];
            let v3195 = parameters[383];
            let v3198 = parameters[384];
            let v3201 = parameters[385];
            let v3204 = parameters[70];
            let v3213 = parameters[1377];
            let v3214 = parameters[1378];
            let v3217 = parameters[1379];
            let v3220 = parameters[1380];
            let v3223 = parameters[1381];
            let v3226 = parameters[1382];
            let v3229 = parameters[1383];
            let v3230 = parameters[1384];
            let v3233 = parameters[1385];
            let v3236 = parameters[1386];
            let v3239 = parameters[1387];
            let v3242 = parameters[1388];
            let v3245 = parameters[1389];
            let v3246 = parameters[1390];
            let v3249 = parameters[1391];
            let v3252 = parameters[1392];
            let v3255 = parameters[1393];
            let v3258 = parameters[1394];
            let v3261 = parameters[1395];
            let v3262 = parameters[1396];
            let v3265 = parameters[1397];
            let v3268 = parameters[1398];
            let v3271 = parameters[1399];
            let v3274 = parameters[1400];
            let v3277 = parameters[1407];
            let v3278 = parameters[1408];
            let v3281 = parameters[1409];
            let v3284 = parameters[1410];
            let v3287 = parameters[1411];
            let v3290 = parameters[1412];
            let v3293 = parameters[1413];
            let v3294 = parameters[1414];
            let v3297 = parameters[1415];
            let v3300 = parameters[1416];
            let v3303 = parameters[1417];
            let v3306 = parameters[1418];
            let v3309 = parameters[1419];
            let v3310 = parameters[1420];
            let v3313 = parameters[1421];
            let v3316 = parameters[1422];
            let v3319 = parameters[1423];
            let v3322 = parameters[1424];
            let v3325 = parameters[1425];
            let v3326 = parameters[1426];
            let v3329 = parameters[1427];
            let v3332 = parameters[1428];
            let v3335 = parameters[1429];
            let v3338 = parameters[1430];
            let v3341 = parameters[66];
            let v3343 = parameters[212];
            let v3344 = parameters[213];
            let v3347 = parameters[214];
            let v3350 = parameters[215];
            let v3353 = parameters[216];
            let v3356 = parameters[217];
            let v3359 = parameters[194];
            let v3360 = parameters[195];
            let v3363 = parameters[196];
            let v3366 = parameters[197];
            let v3369 = parameters[198];
            let v3372 = parameters[199];
            let v3375 = parameters[254];
            let v3376 = parameters[255];
            let v3379 = parameters[256];
            let v3382 = parameters[257];
            let v3385 = parameters[258];
            let v3388 = parameters[259];
            let v3391 = parameters[473];
            let v3392 = parameters[474];
            let v3395 = parameters[475];
            let v3398 = parameters[476];
            let v3401 = parameters[477];
            let v3404 = parameters[478];
            let v3407 = parameters[537];
            let v3408 = parameters[538];
            let v3411 = parameters[539];
            let v3414 = parameters[540];
            let v3417 = parameters[541];
            let v3420 = parameters[542];
            let v3423 = parameters[549];
            let v3424 = parameters[550];
            let v3427 = parameters[551];
            let v3430 = parameters[552];
            let v3433 = parameters[553];
            let v3436 = parameters[554];
            let v3439 = parameters[997];
            let v3440 = parameters[998];
            let v3443 = parameters[999];
            let v3446 = parameters[1000];
            let v3449 = parameters[1001];
            let v3452 = parameters[1002];
            let v3455 = parameters[1003];
            let v3456 = parameters[1004];
            let v3459 = parameters[1005];
            let v3462 = parameters[1006];
            let v3465 = parameters[1007];
            let v3468 = parameters[1008];
            let v3471 = parameters[1032];
            let v3472 = parameters[1033];
            let v3475 = parameters[1034];
            let v3478 = parameters[1035];
            let v3481 = parameters[1036];
            let v3484 = parameters[1037];
            let v3487 = parameters[290];
            let v3488 = parameters[291];
            let v3491 = parameters[292];
            let v3494 = parameters[293];
            let v3497 = parameters[294];
            let v3500 = parameters[295];
            let v3503 = parameters[461];
            let v3504 = parameters[462];
            let v3507 = parameters[463];
            let v3510 = parameters[464];
            let v3513 = parameters[465];
            let v3516 = parameters[466];
            let v3519 = parameters[500];
            let v3520 = parameters[501];
            let v3523 = parameters[502];
            let v3526 = parameters[503];
            let v3529 = parameters[504];
            let v3532 = parameters[505];
            let v3535 = parameters[611];
            let v3536 = parameters[612];
            let v3539 = parameters[613];
            let v3542 = parameters[614];
            let v3545 = parameters[615];
            let v3548 = parameters[616];
            let v3551 = parameters[647];
            let v3552 = parameters[648];
            let v3555 = parameters[649];
            let v3558 = parameters[650];
            let v3561 = parameters[651];
            let v3564 = parameters[652];
            let v3567 = parameters[635];
            let v3568 = parameters[636];
            let v3571 = parameters[637];
            let v3574 = parameters[638];
            let v3577 = parameters[639];
            let v3580 = parameters[640];
            let v3583 = parameters[683];
            let v3584 = parameters[684];
            let v3587 = parameters[685];
            let v3590 = parameters[686];
            let v3593 = parameters[687];
            let v3596 = parameters[688];
            let v3599 = parameters[695];
            let v3600 = parameters[696];
            let v3603 = parameters[697];
            let v3606 = parameters[698];
            let v3609 = parameters[699];
            let v3612 = parameters[700];
            let v3615 = parameters[743];
            let v3616 = parameters[744];
            let v3619 = parameters[745];
            let v3622 = parameters[746];
            let v3625 = parameters[747];
            let v3628 = parameters[748];
            let v3631 = parameters[773];
            let v3632 = parameters[774];
            let v3635 = parameters[775];
            let v3638 = parameters[776];
            let v3641 = parameters[777];
            let v3644 = parameters[778];
            let v3647 = parameters[797];
            let v3648 = parameters[798];
            let v3651 = parameters[799];
            let v3654 = parameters[800];
            let v3657 = parameters[801];
            let v3660 = parameters[802];
            let v3663 = parameters[851];
            let v3664 = parameters[852];
            let v3667 = parameters[853];
            let v3670 = parameters[854];
            let v3673 = parameters[855];
            let v3676 = parameters[856];
            let v3679 = parameters[562];
            let v3680 = parameters[563];
            let v3683 = parameters[564];
            let v3686 = parameters[565];
            let v3689 = parameters[566];
            let v3692 = parameters[567];
            let v3695 = parameters[665];
            let v3696 = parameters[666];
            let v3699 = parameters[667];
            let v3702 = parameters[668];
            let v3705 = parameters[669];
            let v3708 = parameters[670];
            let v3711 = parameters[833];
            let v3712 = parameters[834];
            let v3715 = parameters[835];
            let v3718 = parameters[836];
            let v3721 = parameters[837];
            let v3724 = parameters[838];
            let v3727 = parameters[67];
            let v3729 = parameters[617];
            let v3730 = parameters[618];
            let v3733 = parameters[619];
            let v3736 = parameters[620];
            let v3739 = parameters[621];
            let v3742 = parameters[622];
            let v3745 = parameters[582];
            let v3747 = parameters[585];
            let v3752 = -8.7498233534e1f64;
            let v3758 = parameters[653];
            let v3759 = parameters[654];
            let v3762 = parameters[655];
            let v3765 = parameters[656];
            let v3768 = parameters[657];
            let v3771 = parameters[658];
            let v3774 = parameters[701];
            let v3775 = parameters[702];
            let v3778 = parameters[703];
            let v3781 = parameters[704];
            let v3784 = parameters[705];
            let v3787 = parameters[706];
            let v3790 = parameters[749];
            let v3791 = parameters[750];
            let v3794 = parameters[751];
            let v3797 = parameters[752];
            let v3800 = parameters[753];
            let v3803 = parameters[754];
            let v3806 = parameters[761];
            let v3807 = parameters[762];
            let v3810 = parameters[763];
            let v3813 = parameters[764];
            let v3816 = parameters[765];
            let v3819 = parameters[766];
            let v3822 = parameters[779];
            let v3823 = parameters[780];
            let v3826 = parameters[781];
            let v3829 = parameters[782];
            let v3832 = parameters[783];
            let v3835 = parameters[784];
            let v3838 = parameters[803];
            let v3839 = parameters[804];
            let v3842 = parameters[805];
            let v3845 = parameters[806];
            let v3848 = parameters[807];
            let v3851 = parameters[808];
            let v3854 = parameters[815];
            let v3855 = parameters[816];
            let v3858 = parameters[817];
            let v3861 = parameters[818];
            let v3864 = parameters[819];
            let v3867 = parameters[820];
            let v3870 = parameters[857];
            let v3871 = parameters[858];
            let v3874 = parameters[859];
            let v3877 = parameters[860];
            let v3880 = parameters[861];
            let v3883 = parameters[862];
            let v3886 = parameters[869];
            let v3887 = parameters[870];
            let v3890 = parameters[871];
            let v3893 = parameters[872];
            let v3896 = parameters[873];
            let v3899 = parameters[874];
            let v3902 = parameters[671];
            let v3903 = parameters[672];
            let v3906 = parameters[673];
            let v3909 = parameters[674];
            let v3912 = parameters[675];
            let v3915 = parameters[676];
            let v3918 = parameters[839];
            let v3919 = parameters[840];
            let v3922 = parameters[841];
            let v3925 = parameters[842];
            let v3928 = parameters[843];
            let v3931 = parameters[844];
            let v3934 = parameters[260];
            let v3935 = parameters[261];
            let v3938 = parameters[262];
            let v3941 = parameters[263];
            let v3944 = parameters[264];
            let v3947 = parameters[265];
            let v3950 = parameters[161];
            let v3952 = parameters[162];
            let v3957 = -8.7498233534e1f64;
            let v3963 = parameters[21];
            let v3967 = parameters[588];
            let v3974 = parameters[163];
            let v3979 = parameters[73];
            let v3981 = parameters[1668];
            let v3984 = parameters[1669];
            let v3987 = parameters[1670];
            let v3990 = parameters[1671];
            let v3993 = parameters[1672];
            let v3996 = parameters[1673];
            let v3999 = parameters[57];
            let v4001 = parameters[1807];
            let v4002 = parameters[1808];
            let v4005 = parameters[1809];
            let v4008 = parameters[1810];
            let v4011 = parameters[1811];
            let v4014 = parameters[1812];
            let v4017 = parameters[1814];
            let v4018 = parameters[1815];
            let v4021 = parameters[1816];
            let v4024 = parameters[1817];
            let v4027 = parameters[1818];
            let v4030 = parameters[1819];
            let v4033 = parameters[1821];
            let v4034 = parameters[1822];
            let v4037 = parameters[1823];
            let v4040 = parameters[1824];
            let v4043 = parameters[1825];
            let v4046 = parameters[1826];
            let v4049 = parameters[1829];
            let v4050 = parameters[1830];
            let v4053 = parameters[1831];
            let v4056 = parameters[1832];
            let v4059 = parameters[1833];
            let v4062 = parameters[1834];
            let v4065 = parameters[1835];
            let v4066 = parameters[1836];
            let v4069 = parameters[1837];
            let v4072 = parameters[1838];
            let v4075 = parameters[1839];
            let v4078 = parameters[1840];
            let v4081 = parameters[1841];
            let v4082 = parameters[1842];
            let v4085 = parameters[1843];
            let v4088 = parameters[1844];
            let v4091 = parameters[1845];
            let v4094 = parameters[1846];
            let v4097 = parameters[1853];
            let v4098 = parameters[1854];
            let v4101 = parameters[1855];
            let v4104 = parameters[1856];
            let v4107 = parameters[1857];
            let v4110 = parameters[1858];
            let v4113 = parameters[1859];
            let v4114 = parameters[1860];
            let v4117 = parameters[1861];
            let v4120 = parameters[1862];
            let v4123 = parameters[1863];
            let v4126 = parameters[1864];
            let v4129 = parameters[1869];
            let v4130 = parameters[1870];
            let v4133 = parameters[1871];
            let v4136 = parameters[1872];
            let v4139 = parameters[1873];
            let v4142 = parameters[1874];
            let v4145 = parameters[1875];
            let v4146 = parameters[1876];
            let v4149 = parameters[1877];
            let v4152 = parameters[1878];
            let v4155 = parameters[1879];
            let v4158 = parameters[1880];
            let v4161 = parameters[1881];
            let v4162 = parameters[1882];
            let v4165 = parameters[1883];
            let v4168 = parameters[1884];
            let v4171 = parameters[1885];
            let v4174 = parameters[1886];
            let v4177 = parameters[100];
            let v4179 = parameters[101];
            let v4184 = -8.7498233534e1f64;
            let v4190 = parameters[158];
            let v4192 = parameters[159];
            let v4197 = -8.7498233534e1f64;
            let v4203 = parameters[152];
            let v4205 = parameters[153];
            let v4210 = -8.7498233534e1f64;
            let v4216 = parameters[154];
            let v4218 = parameters[155];
            let v4223 = -8.7498233534e1f64;
            let v4229 = parameters[156];
            let v4231 = parameters[157];
            let v4236 = -8.7498233534e1f64;
            let v4243 = parameters[428];
            let v4245 = parameters[429];
            let v4250 = -8.7498233534e1f64;
            let v4256 = parameters[432];
            let v4258 = parameters[433];
            let v4263 = -8.7498233534e1f64;
            let v4269 = parameters[434];
            let v4271 = parameters[435];
            let v4276 = -8.7498233534e1f64;
            let v4283 = parameters[581];
            let v4285 = parameters[584];
            let v4290 = -8.7498233534e1f64;
            let v4296 = parameters[583];
            let v4298 = parameters[586];
            let v4303 = -8.7498233534e1f64;
            let v4313 = parameters[99];
            let v4319 = parameters[160];
            let v4325 = parameters[587];
            let v4332 = parameters[98];
            let v4335 = parameters[427];
            let v4338 = parameters[589];
            let v4349 = parameters[591];
            let v4351 = parameters[593];
            let v4356 = parameters[599];
            let v4357 = parameters[601];
            let v4362 = parameters[595];
            let v4363 = parameters[597];
            let v4369 = parameters[592];
            let v4370 = parameters[594];
            let v4376 = parameters[600];
            let v4377 = parameters[602];
            let v4383 = parameters[596];
            let v4384 = parameters[598];
            let v4389 = parameters[590];
            let v4401 = parameters[64];
            let v4403 = parameters[912];
            let v4404 = parameters[913];
            let v4409 = parameters[915];
            let v4410 = parameters[916];
            let v4415 = parameters[909];
            let v4416 = parameters[910];
            let v4421 = parameters[1021];
            let v4422 = parameters[1023];
            let v4428 = parameters[1022];
            let v4429 = parameters[1024];
            let v4435 = parameters[444];
            let v4436 = parameters[445];
            let v4443 = parameters[446];
            let v4444 = parameters[447];
            let v4450 = parameters[448];
            let v4451 = parameters[449];
            let v4459 = parameters[430];
            let v4460 = parameters[431];
            let v4466 = parameters[436];
            let v4467 = parameters[437];
            let v4474 = parameters[438];
            let v4475 = parameters[439];
            let v4480 = parameters[442];
            let v4483 = parameters[443];
            let v4488 = parameters[440];
            let v4489 = parameters[441];
            let v4494 = parameters[167];
            let v4495 = parameters[168];
            let v4500 = parameters[169];
            let v4501 = parameters[170];
            let v4509 = parameters[398];
            let v4513 = parameters[399];
            let v4522 = 5e-2f64;
            let v4525 = 1e24f64;
            let v4528 = 1e31f64;
            let v4532 = 4.61e0f64;
            let v4536 = 1e-2f64;
            let v4540 = parameters[1682];
            let v4542 = 1.2e0f64;
            let v4546 = 8.5e4f64;
            let v4552 = 6e-1f64;
            let v4562 = 1.06e0f64;
            let v4571 = 2e-1f64;
            let v4581 = 3e-2f64;
            let v4618 = parameters[69];
            let v4622 = parameters[68];
            let v4627 = parameters[1108];
            let v4629 = parameters[1649];
            let v4637 = 1e-3f64;
            let v4639 = parameters[71];
            let v4656 = 2e-2f64;
            let v4660 = parameters[4];
            let v4671 = 2.6e0f64;
            let v4678 = 1.4e1f64;
            let v4681 = 2.4e1f64;
            let v4686 = 1.39e-1f64;
            let v4691 = 1.12e1f64;
            let v4694 = 8.02e0f64;
            let v4697 = 6.18e0f64;
            let v4701 = parameters[1795];
            let v4703 = parameters[1794];
            let v4705 = parameters[1796];
            let v4707 = parameters[76];
            let v4709 = parameters[1074];
            let v4710 = parameters[6];
            let v4712 = parameters[1075];
            let v4715 = 1.2e1f64;
            let v4723 = parameters[77];
            let v4725 = parameters[1078];
            let v4726 = parameters[18];
            let v4728 = parameters[1079];
            let v4729 = parameters[19];
            let v4731 = parameters[1080];
            let v4735 = parameters[1084];
            let v4745 = if parameter_given[1083] { 1.0 } else { 0.0 };
            let v4746 = parameters[1083];
            let v4747 = 1.417e3f64;
            let v4748 = 4.705e2f64;
            let v4750 = parameters[97];
            let v4751 = 9.68e22f64;
            let v4753 = 6.8e-1f64;
            let v4755 = 3.43e26f64;
            let v4757 = 5.22e1f64;
            let v4762 = 4.34e1f64;
            let v4767 = 1e-4f64;
            let v4769 = 2.23e22f64;
            let v4771 = 7.19e-1f64;
            let v4773 = 6.1e26f64;
            let v4775 = 4.49e1f64;
            let v4780 = 2.9e1f64;
            let v4791 = 1e-18f64;
            let v4798 = 1.4281480067421144e0f64;
            let v4800 = 1.7724538509055159e0f64;
            let v4814 = parameters[1092];
            let v4817 = parameters[1093];
            let v4819 = parameters[1082];
            let v4824 = parameters[20];
            let v4828 = parameters[1086];
            let v4846 = -1e-10f64;
            let v4859 = parameters[1094];
            let v4860 = parameters[1095];
            let v4863 = parameters[1096];
            let v4866 = parameters[1097];
            let v4869 = parameters[1098];
            let v4876 = parameters[151];
            let v4882 = parameters[78];
            let v4884 = if parameter_given[1542] { 1.0 } else { 0.0 };
            let v4885 = if parameter_given[85] { 1.0 } else { 0.0 };
            let v4889 = if parameter_given[1543] { 1.0 } else { 0.0 };
            let v4894 = parameters[1089];
            let v4895 = parameters[1090];
            let v4897 = 5e-1f64;
            let v4900 = parameters[90];
            let v4903 = parameters[1081];
            let v4907 = 1e-7f64;
            let v4909 = 3.9e0f64;
            let v4910 = parameters[1087];
            let v4914 = 2.3e0f64;
            let v4919 = 1.05e0f64;
            let v4926 = 1.7e12f64;
            let v4931 = 8e1f64;
            let v4933 = 3.7e1f64;
            let v4935 = -3.7e1f64;
            let v4942 = 1.5707963267948966e0f64;
            let v5011 = -3.7e1f64;
            let v5018 = 1.5707963267948966e0f64;
            let v5071 = 7e-1f64;
            let v5088 = -3.7e1f64;
            let v5095 = 1.5707963267948966e0f64;
            let v5157 = parameters[41];
            let v5180 = -3.7e1f64;
            let v5187 = 1.5707963267948966e0f64;
            let v5256 = -3.7e1f64;
            let v5263 = 1.5707963267948966e0f64;
            let v5331 = -3.7e1f64;
            let v5338 = 1.5707963267948966e0f64;
            let v5405 = -3.7e1f64;
            let v5412 = 1.5707963267948966e0f64;
            let v5479 = -3.7e1f64;
            let v5486 = 1.5707963267948966e0f64;
            let v5542 = -3.7e1f64;
            let v5544 = 1.5707963267948966e0f64;
            let v5570 = 1e-8f64;
            let v5573 = 1e6f64;
            let v5591 = if parameter_given[172] { 1.0 } else { 0.0 };
            let v5597 = 4e1f64;
            let v5604 = parameters[172];
            let v5605 = if parameter_given[174] { 1.0 } else { 0.0 };
            let v5617 = parameters[174];
            let v5618 = if parameter_given[173] { 1.0 } else { 0.0 };
            let v5630 = parameters[173];
            let v5640 = parameters[171];
            let v5655 = 4.97232e-7f64;
            let v5656 = 7.45669e11f64;
            let v5657 = 3.42537e-7f64;
            let v5658 = 1.16645e12f64;
            let v5659 = parameters[1109];
            let v5674 = parameters[1717];
            let v5675 = 2.7315e2f64;
            let v5676 = -2.7315e2f64;
            let v5678 = 3.0015e2f64;
            let v5680 = parameters[1806];
            let v5684 = parameters[1827];
            let v5685 = 1e9f64;
            let v5689 = parameters[1828];
            let v5695 = parameters[1813];
            let v5701 = parameters[1820];
            let v5707 = parameters[1847];
            let v5709 = parameters[1850];
            let v5712 = parameters[1851];
            let v5721 = 2.5e-1f64;
            let v5722 = 2.5e-7f64;
            let v5727 = parameters[1848];
            let v5734 = 2.5e-7f64;
            let v5739 = parameters[1849];
            let v5746 = 2.5e-7f64;
            let v5751 = 1.001e0f64;
            let v5755 = 2.5e-7f64;
            let v5761 = 2.5e-7f64;
            let v5767 = 2.5e-7f64;
            let v5774 = 2.5e-7f64;
            let v5779 = 2.5e-4f64;
            let v5781 = 2.001e0f64;
            let v5785 = 2.5e-7f64;
            let v5791 = 2.5e-7f64;
            let v5797 = 2.5e-7f64;
            let v5804 = 2.5e-7f64;
            let v5809 = 2.5e-4f64;
            let v5814 = 2.5e-7f64;
            let v5820 = 2.5e-7f64;
            let v5826 = 2.5e-7f64;
            let v5833 = 2.5e-7f64;
            let v5838 = 2.5e-4f64;
            let v5843 = 2.5e-7f64;
            let v5849 = 2.5e-7f64;
            let v5855 = 2.5e-7f64;
            let v5862 = 2.5e-7f64;
            let v5867 = 2.5e-4f64;
            let v5872 = 2.5e-7f64;
            let v5878 = 2.5e-7f64;
            let v5884 = 2.5e-7f64;
            let v5891 = 2.5e-7f64;
            let v5896 = 2.5e-4f64;
            let v5901 = 2.5e-7f64;
            let v5907 = 2.5e-7f64;
            let v5913 = 2.5e-7f64;
            let v5920 = 2.5e-7f64;
            let v5925 = 2.5e-4f64;
            let v5940 = 2.75e0f64;
            let v5943 = 7.8e-1f64;
            let v5951 = 2.25e-6f64;
            let v5961 = 9.99e-1f64;
            let v5970 = 1.5e0f64;
            let v5978 = 2.5e-5f64;
            let v5983 = 2.5e-3f64;
            let v5986 = parameters[1893];
            let v5988 = 9.24e5f64;
            let v5989 = 1.81e4f64;
            let v5990 = 9.059e5f64;
            let v5996 = 9.059e5f64;
            let v6001 = 9.059e5f64;
            let v6007 = 2.5e-5f64;
            let v6013 = 9.059e5f64;
            let v6018 = 9.059e5f64;
            let v6023 = 9.059e5f64;
            let v6029 = 2.5e-5f64;
            let v6035 = 9.059e5f64;
            let v6040 = 9.059e5f64;
            let v6045 = 9.059e5f64;
            let v6051 = 2.5e-5f64;
            let v6058 = 2.13444e7f64;
            let v6063 = 2.31e3f64;
            let v6065 = 5.5e0f64;
            let v6066 = parameters[1894];
            let v6068 = 8e0f64;
            let v6069 = 2.5e0f64;
            let v6074 = 2.5e0f64;
            let v6078 = 2.5e0f64;
            let v6083 = 2.5e-5f64;
            let v6089 = 2.5e0f64;
            let v6093 = 2.5e0f64;
            let v6097 = 2.5e0f64;
            let v6102 = 2.5e-5f64;
            let v6108 = 2.5e0f64;
            let v6112 = 2.5e0f64;
            let v6116 = 2.5e0f64;
            let v6121 = 2.5e-5f64;
            let v6128 = 2.5e-5f64;
            let v6133 = 2.5e-3f64;
            let v6135 = 1.2066e2f64;
            let v6136 = parameters[1895];
            let v6141 = parameters[1896];
            let v6146 = 1.07e2f64;
            let v6147 = parameters[1897];
            let v6152 = parameters[1898];
            let v6154 = 1e-1f64;
            let v6160 = 2.5e-5f64;
            let v6166 = 2.5e-5f64;
            let v6172 = 2.5e-5f64;
            let v6179 = 2.5e-5f64;
            let v6184 = 2.5e-3f64;
            let v6186 = 1.03e2f64;
            let v6187 = parameters[1899];
            let v6192 = parameters[1900];
            let v6197 = 8.33e2f64;
            let v6198 = parameters[1901];
            let v6203 = 3.4e0f64;
            let v6204 = parameters[1902];
            let v6209 = parameters[1852];
            let v6211 = parameters[1867];
            let v6215 = parameters[1868];
            let v6221 = parameters[1865];
            let v6228 = 2.5e-5f64;
            let v6235 = parameters[1866];
            let v6242 = 2.5e-5f64;
            let v6247 = parameters[1890];
            let v6255 = 2.5000000000000005e-3f64;
            let v6266 = 2.5000000000000005e-3f64;
            let v6273 = parameters[1887];
            let v6277 = parameters[1891];
            let v6285 = 2.5000000000000005e-3f64;
            let v6296 = 2.5000000000000005e-3f64;
            let v6303 = parameters[1888];
            let v6307 = parameters[1892];
            let v6315 = 2.5000000000000005e-3f64;
            let v6326 = 2.5000000000000005e-3f64;
            let v6333 = parameters[1889];
            let v6339 = 3.14e0f64;
            let v6341 = 3.85e-2f64;
            let v6342 = -4.6e0f64;
            let v6348 = 7.5893e-7f64;
            let v6354 = 6.9583e-5f64;
            let v6356 = 6e0f64;
            let v6360 = 6.583e-4f64;
            let v6363 = 6.5e-3f64;
            let v6366 = 2.6e-2f64;
            let v6369 = 1.371e-1f64;
            let v6372 = 3.88e-1f64;
            let v6375 = 9.59e-1f64;
            let v6386 = -4.6e0f64;
            let v6409 = 3.88e-1f64;
            let v6422 = -4.6e0f64;
            let v6445 = 3.88e-1f64;
            let v6455 = parameters[58];
            let v6457 = parameters[889];
            let v6460 = parameters[890];
            let v6464 = parameters[891];
            let v6471 = parameters[892];
            let v6473 = parameters[893];
            let v6475 = parameters[894];
            let v6480 = parameters[895];
            let v6483 = parameters[896];
            let v6493 = 9e-2f64;
            let v6498 = parameters[897];
            let v6501 = 3.7e2f64;
            let v6503 = parameters[898];
            let v6506 = parameters[899];
            let v6509 = parameters[900];
            let v6519 = 1.0000000000000002e-2f64;
            let v6526 = parameters[905];
            let v6530 = parameters[906];
            let v6536 = 2.4e-1f64;
            let v6550 = 8.208e-1f64;
            let v6552 = parameters[907];
            let v6553 = 1e-5f64;
            let v6560 = 9e-4f64;
            let v6566 = parameters[904];
            let v6572 = parameters[901];
            let v6575 = parameters[902];
            let v6579 = 2.5e-1f64;
            let v6584 = parameters[903];
            let v6588 = temperature;
            let v6589 = node_potentials[4];
            let v6591 = parameters[22];
            let v6599 = 8.617087e-5f64;
            let v6602 = parameters[1786];
            let v6603 = parameters[80];
            let v6608 = parameters[1788];
            let v6615 = parameters[1790];
            let v6617 = parameters[1787];
            let v6621 = parameters[1789];
            let v6656 = 1.0000000000000002e-2f64;
            let v6661 = 2.1e2f64;
            let v6692 = 1.0000000000000002e-2f64;
            let v6724 = 1.0000000000000002e-2f64;
            let v6734 = 1.0000000000000002e-2f64;
            let v6742 = 1.0000000000000002e-2f64;
            let v6753 = parameters[106];
            let v6754 = parameters[1718];
            let v6757 = parameters[1719];
            let v6769 = parameters[105];
            let v6771 = 5.1728373261e-2f64;
            let v6779 = parameters[107];
            let v6788 = -8.7498233534e1f64;
            let v6790 = 5.1728373261e-2f64;
            let v6797 = -1e1f64;
            let v6799 = -1e-6f64;
            let v6802 = 4e-6f64;
            let v6809 = 3.313029364696188e-34f64;
            let v6814 = 1.6689520000000002e-30f64;
            let v6816 = 3.4618e-31f64;
            let v6820 = 4.389473684210526e0f64;
            let v6835 = 3.4618e-31f64;
            let v6836 = 3.493821377127659e-68f64;
            let v6845 = -8.7498233534e1f64;
            let v6857 = 9e-1f64;
            let v6858 = -9e-1f64;
            let v6861 = -9e-1f64;
            let v6865 = -9e-1f64;
            let v6869 = -9e-1f64;
            let v6874 = -9e-1f64;
            let v6891 = -9e-1f64;
            let v6895 = -9e-1f64;
            let v6899 = -9e-1f64;
            let v6903 = -9e-1f64;
            let v6908 = -9e-1f64;
            let v6966 = -1e1f64;
            let v6968 = -1e-6f64;
            let v6971 = 4e-6f64;
            let v6977 = parameters[75];
            let v6998 = -1e1f64;
            let v7000 = -1e-6f64;
            let v7003 = 4e-6f64;
            let v7030 = -1e1f64;
            let v7032 = -1e-6f64;
            let v7035 = 4e-6f64;
            let v7043 = 1e3f64;
            let v7047 = 0e0f64;
            let v7049 = 0e0f64;
            let v7053 = -9e-1f64;
            let v7056 = 0e0f64;
            let v7058 = -9e-1f64;
            let v7062 = -9e-1f64;
            let v7066 = -9e-1f64;
            let v7071 = -9e-1f64;
            let v7082 = 0e0f64;
            let v7085 = 0e0f64;
            let v7099 = 0e0f64;
            let v7101 = 0e0f64;
            let v7124 = -1e1f64;
            let v7126 = -1e-6f64;
            let v7129 = 4e-6f64;
            let v7155 = -1e1f64;
            let v7157 = -1e-6f64;
            let v7160 = 4e-6f64;
            let v7187 = -1e1f64;
            let v7189 = -1e-6f64;
            let v7192 = 4e-6f64;
            let v7200 = parameters[450];
            let v7205 = -1e1f64;
            let v7207 = -1e-6f64;
            let v7210 = 4e-6f64;
            let v7219 = parameters[452];
            let v7224 = -1e1f64;
            let v7226 = -1e-6f64;
            let v7229 = 4e-6f64;
            let v7236 = parameters[1720];
            let v7246 = -9e-1f64;
            let v7249 = -9e-1f64;
            let v7253 = -9e-1f64;
            let v7257 = -9e-1f64;
            let v7262 = -9e-1f64;
            let v7277 = -9e-1f64;
            let v7280 = -9e-1f64;
            let v7284 = -9e-1f64;
            let v7288 = -9e-1f64;
            let v7293 = -9e-1f64;
            let v7353 = -1e1f64;
            let v7355 = -1e-6f64;
            let v7358 = 4e-6f64;
            let v7366 = 0e0f64;
            let v7372 = -9e-1f64;
            let v7375 = -9e-1f64;
            let v7379 = -9e-1f64;
            let v7383 = -9e-1f64;
            let v7388 = -9e-1f64;
            let v7399 = 0e0f64;
            let v7406 = 0e0f64;
            let v7504 = -1e1f64;
            let v7506 = -1e-6f64;
            let v7509 = 4e-6f64;
            let v7518 = parameters[561];
            let v7540 = -1e1f64;
            let v7542 = -1e-6f64;
            let v7545 = 4e-6f64;
            let v7576 = -1e1f64;
            let v7578 = -1e-6f64;
            let v7581 = 4e-6f64;
            let v7614 = -1e1f64;
            let v7616 = -1e-6f64;
            let v7619 = 4e-6f64;
            let v7650 = -1e1f64;
            let v7652 = -1e-6f64;
            let v7655 = 4e-6f64;
            let v7667 = parameters[574];
            let v7689 = -1e1f64;
            let v7691 = -1e-6f64;
            let v7694 = 4e-6f64;
            let v7703 = parameters[451];
            let v7709 = -1e1f64;
            let v7711 = -1e-6f64;
            let v7714 = 4e-6f64;
            let v7726 = -1e1f64;
            let v7728 = -1e-6f64;
            let v7731 = 4e-6f64;
            let v7739 = parameters[498];
            let v7741 = parameters[499];
            let v7762 = -1e1f64;
            let v7764 = -1e-6f64;
            let v7767 = 4e-6f64;
            let v7775 = parameters[1026];
            let v7791 = -1e1f64;
            let v7793 = -1e-6f64;
            let v7796 = 4e-6f64;
            let v7806 = parameters[1747];
            let v7807 = parameters[1748];
            let v7808 = parameters[1749];
            let v7828 = -9e-1f64;
            let v7831 = -9e-1f64;
            let v7835 = -9e-1f64;
            let v7839 = -9e-1f64;
            let v7844 = -9e-1f64;
            let v7859 = -9e-1f64;
            let v7862 = -9e-1f64;
            let v7866 = -9e-1f64;
            let v7870 = -9e-1f64;
            let v7875 = -9e-1f64;
            let v7889 = -8.7498233534e1f64;
            let v7913 = -8.7498233534e1f64;
            let v7943 = -1e-2f64;
            let v7945 = -1e-12f64;
            let v7948 = 4e-12f64;
            let v7957 = -8.7498233534e1f64;
            let v7981 = -8.7498233534e1f64;
            let v8007 = -1e-2f64;
            let v8009 = -1e-12f64;
            let v8012 = 4e-12f64;
            let v8192 = -1e1f64;
            let v8194 = -1e-6f64;
            let v8197 = 4e-6f64;
            let v8210 = -1e1f64;
            let v8212 = -1e-6f64;
            let v8215 = 4e-6f64;
            let v8226 = -1e1f64;
            let v8228 = -1e-6f64;
            let v8231 = 4e-6f64;
            let v8239 = -1e1f64;
            let v8241 = -1e-6f64;
            let v8244 = 4e-6f64;
            let v8251 = -1e1f64;
            let v8253 = -1e-6f64;
            let v8256 = 4e-6f64;
            let v8270 = -1e1f64;
            let v8272 = -1e-6f64;
            let v8275 = 4e-6f64;
            let v8288 = -1e1f64;
            let v8290 = -1e-6f64;
            let v8293 = 4e-6f64;
            let v8303 = -1e1f64;
            let v8305 = -1e-6f64;
            let v8308 = 4e-6f64;
            let v8322 = -1e1f64;
            let v8324 = -1e-6f64;
            let v8327 = 4e-6f64;
            let v8339 = -1e1f64;
            let v8341 = -1e-6f64;
            let v8344 = 4e-6f64;
            let v8356 = -9e-1f64;
            let v8359 = -9e-1f64;
            let v8363 = -9e-1f64;
            let v8367 = -9e-1f64;
            let v8372 = -9e-1f64;
            let v8385 = -8.7498233534e1f64;
            let v8409 = -8.7498233534e1f64;
            let v8435 = -1e-2f64;
            let v8437 = -1e-12f64;
            let v8440 = 4e-12f64;
            let v8492 = 2.5e-7f64;
            let v8518 = 2.5e-7f64;
            let v8571 = 2.5e-7f64;
            let v8605 = 2.5e-7f64;
            let v8657 = 2.5e-7f64;
            let v8691 = 2.5e-7f64;
            let v8708 = 2.5e-5f64;
            let v8716 = 2.5e-5f64;
            let v8733 = 2.5e-5f64;
            let v8741 = 2.5e-5f64;
            let v8757 = -1e1f64;
            let v8759 = -1e-6f64;
            let v8762 = 4e-6f64;
            let v8770 = 1.0000000000000002e-2f64;
            let v8830 = -1e1f64;
            let v8832 = -1e-6f64;
            let v8835 = 4e-6f64;
            let v8844 = 0e0f64;
            let v8848 = 0e0f64;
            let v8864 = -1e1f64;
            let v8866 = -1e-6f64;
            let v8869 = 4e-6f64;
            let v8896 = -1e1f64;
            let v8898 = -1e-6f64;
            let v8901 = 4e-6f64;
            let v8917 = parameters[164];
            let v8933 = -1e1f64;
            let v8935 = -1e-6f64;
            let v8938 = 4e-6f64;
            let v8947 = 0e0f64;
            let v8949 = parameters[165];
            let v8965 = -1e1f64;
            let v8967 = -1e-6f64;
            let v8970 = 4e-6f64;
            let v8979 = parameters[166];
            let v8995 = -1e1f64;
            let v8997 = -1e-6f64;
            let v9000 = 4e-6f64;
            let v9024 = -1e1f64;
            let v9026 = -1e-6f64;
            let v9029 = 4e-6f64;
            let v9036 = parameters[917];
            let v9038 = parameters[923];
            let v9054 = -1e1f64;
            let v9056 = -1e-6f64;
            let v9059 = 4e-6f64;
            let v9066 = parameters[918];
            let v9083 = -1e1f64;
            let v9085 = -1e-6f64;
            let v9088 = 4e-6f64;
            let v9095 = parameters[919];
            let v9097 = parameters[924];
            let v9113 = -1e1f64;
            let v9115 = -1e-6f64;
            let v9118 = 4e-6f64;
            let v9125 = parameters[920];
            let v9142 = -1e1f64;
            let v9144 = -1e-6f64;
            let v9147 = 4e-6f64;
            let v9173 = -1e1f64;
            let v9175 = -1e-6f64;
            let v9178 = 4e-6f64;
            let v9204 = -1e1f64;
            let v9206 = -1e-6f64;
            let v9209 = 4e-6f64;
            let v9222 = -1e1f64;
            let v9369 = -1e1f64;
            let v9371 = -1e-6f64;
            let v9374 = 4e-6f64;
            let v9381 = -1e1f64;
            let v9383 = -1e-6f64;
            let v9386 = 4e-6f64;
            let v9394 = parameters[1437];
            let v9408 = parameters[1438];
            let v9422 = parameters[1439];
            let v9425 = 1e-25f64;
            let v9437 = parameters[1440];
            let v9440 = 1e-20f64;
            let v9454 = parameters[1584];
            let v9456 = parameters[1721];
            let v9472 = -1e1f64;
            let v9474 = -1e-6f64;
            let v9477 = 4e-6f64;
            let v9484 = parameters[1585];
            let v9501 = -1e1f64;
            let v9503 = -1e-6f64;
            let v9506 = 4e-6f64;
            let v9513 = parameters[1586];
            let v9515 = parameters[1722];
            let v9531 = -1e1f64;
            let v9533 = -1e-6f64;
            let v9536 = 4e-6f64;
            let v9543 = parameters[1587];
            let v9560 = -1e1f64;
            let v9562 = -1e-6f64;
            let v9565 = 4e-6f64;
            let v9572 = parameters[1588];
            let v9574 = parameters[1723];
            let v9590 = -1e1f64;
            let v9592 = -1e-6f64;
            let v9595 = 4e-6f64;
            let v9602 = parameters[1589];
            let v9619 = -1e1f64;
            let v9621 = -1e-6f64;
            let v9624 = 4e-6f64;
            let v9631 = parameters[1590];
            let v9632 = parameters[1724];
            let v9636 = -1e1f64;
            let v9638 = -1e-6f64;
            let v9641 = 4e-6f64;
            let v9648 = parameters[1591];
            let v9651 = -1e1f64;
            let v9653 = -1e-6f64;
            let v9656 = 4e-6f64;
            let v9663 = parameters[1592];
            let v9664 = parameters[1725];
            let v9668 = -1e1f64;
            let v9670 = -1e-6f64;
            let v9673 = 4e-6f64;
            let v9680 = parameters[1593];
            let v9683 = -1e1f64;
            let v9685 = -1e-6f64;
            let v9688 = 4e-6f64;
            let v9695 = parameters[1594];
            let v9696 = parameters[1726];
            let v9700 = -1e1f64;
            let v9702 = -1e-6f64;
            let v9705 = 4e-6f64;
            let v9712 = parameters[1595];
            let v9715 = -1e1f64;
            let v9717 = -1e-6f64;
            let v9720 = 4e-6f64;
            let v9730 = parameters[1727];
            let v9733 = parameters[1620];
            let v9736 = parameters[1614];
            let v9738 = parameters[1616];
            let v9740 = parameters[1618];
            let v9742 = parameters[1728];
            let v9745 = parameters[1621];
            let v9748 = parameters[1615];
            let v9750 = parameters[1617];
            let v9752 = parameters[1619];
            let v9754 = parameters[1630];
            let v9755 = parameters[1729];
            let v9761 = parameters[1631];
            let v9762 = parameters[1730];
            let v9768 = parameters[1632];
            let v9769 = parameters[1731];
            let v9775 = parameters[1633];
            let v9776 = parameters[1732];
            let v9782 = parameters[1634];
            let v9783 = parameters[1636];
            let v9788 = parameters[1733];
            let v9794 = parameters[1635];
            let v9796 = parameters[1734];
            let v9802 = parameters[1637];
            let v9803 = parameters[1735];
            let v9808 = -1e1f64;
            let v9810 = parameters[1638];
            let v9811 = parameters[1736];
            let v9816 = -1e1f64;
            let v9818 = parameters[1639];
            let v9819 = parameters[1737];
            let v9824 = -1e1f64;
            let v9826 = parameters[1640];
            let v9827 = parameters[1738];
            let v9832 = -1e1f64;
            let v9834 = parameters[1641];
            let v9835 = parameters[1739];
            let v9840 = -1e1f64;
            let v9842 = parameters[1642];
            let v9843 = parameters[1740];
            let v9848 = -1e1f64;
            let v9850 = if parameter_given[1106] { 1.0 } else { 0.0 };
            let v9856 = -8.7498233534e1f64;
            let v9861 = -1e0f64;
            let v9863 = -1e-8f64;
            let v9866 = 4e-8f64;
            let v9874 = -8.7498233534e1f64;
            let v9878 = -1e0f64;
            let v9880 = -1e-8f64;
            let v9883 = 4e-8f64;
            let v9898 = -8.7498233534e1f64;
            let v9905 = -1e0f64;
            let v9907 = -1e-8f64;
            let v9910 = 4e-8f64;
            let v9917 = -8.7498233534e1f64;
            let v9922 = -1e0f64;
            let v9924 = -1e-8f64;
            let v9927 = 4e-8f64;
            let v9942 = -8.7498233534e1f64;
            let v9947 = -1e0f64;
            let v9949 = -1e-8f64;
            let v9952 = 4e-8f64;
            let v9958 = parameters[104];
            let v9968 = -8.7498233534e1f64;
            let v9974 = -1e0f64;
            let v9976 = -1e-8f64;
            let v9979 = 4e-8f64;
            let v9991 = parameters[1106];
            let v9992 = if parameter_given[1107] { 1.0 } else { 0.0 };
            let v10001 = -8.7498233534e1f64;
            let v10005 = 2.5000000000000002e-21f64;
            let v10015 = -8.7498233534e1f64;
            let v10020 = -8.7498233534e1f64;
            let v10025 = 2.5000000000000002e-21f64;
            let v10033 = -8.7498233534e1f64;
            let v10045 = 3.33333333e-1f64;
            let v10047 = parameters[11];
            let v10050 = parameters[13];
            let v10060 = parameters[1626];
            let v10064 = parameters[1628];
            let v10066 = parameters[1622];
            let v10068 = 1e1f64;
            let v10080 = -8.7498233534e1f64;
            let v10083 = parameters[1624];
            let v10086 = -1e1f64;
            let v10088 = -1e-6f64;
            let v10091 = 4e-6f64;
            let v10102 = -8.7498233534e1f64;
            let v10116 = parameters[12];
            let v10119 = parameters[14];
            let v10128 = parameters[1627];
            let v10132 = parameters[1629];
            let v10134 = parameters[1623];
            let v10147 = -8.7498233534e1f64;
            let v10150 = parameters[1625];
            let v10153 = -1e1f64;
            let v10155 = -1e-6f64;
            let v10158 = 4e-6f64;
            let v10169 = -8.7498233534e1f64;
            let v10203 = parameters[1602];
            let v10207 = parameters[1596];
            let v10213 = parameters[1608];
            let v10222 = parameters[1604];
            let v10226 = parameters[1598];
            let v10232 = parameters[1610];
            let v10241 = parameters[1606];
            let v10245 = parameters[1600];
            let v10251 = parameters[1612];
            let v10260 = parameters[1603];
            let v10264 = parameters[1597];
            let v10270 = parameters[1609];
            let v10279 = parameters[1605];
            let v10283 = parameters[1599];
            let v10289 = parameters[1611];
            let v10298 = parameters[1607];
            let v10302 = parameters[1601];
            let v10308 = parameters[1613];
            let v10317 = node_potentials[11];
            let v10318 = node_potentials[6];
            let v10321 = node_potentials[5];
            let v10326 = node_potentials[3];
            let v10334 = node_potentials[10];
            let v10339 = node_potentials[14];
            let v10342 = node_potentials[13];
            let v10346 = -1e0f64;
            let v10348 = -1e0f64;
            let v10361 = 9.5e-1f64;
            let v10367 = 4e-3f64;
            let v10528 = 4e-1f64;
            let v10541 = parameters[175];
            let v10565 = -8.7498233534e1f64;
            let v10575 = -8.7498233534e1f64;
            let v10585 = 6.66666667e-1f64;
            let v10586 = 1.2879922655862042e-25f64;
            let v10587 = parameters[1804];
            let v10629 = parameters[108];
            let v10639 = -8.7498233534e1f64;
            let v10644 = parameters[23];
            let v10646 = -1e0f64;
            let v10648 = -1e-8f64;
            let v10651 = 4e-8f64;
            let v10666 = -8.7498233534e1f64;
            let v10674 = 2.5e-9f64;
            let v10682 = -1e0f64;
            let v10684 = -1e-8f64;
            let v10687 = 4e-8f64;
            let v10699 = -1e3f64;
            let v10701 = -1.0000000000000002e-2f64;
            let v10704 = 4.000000000000001e-2f64;
            let v10744 = parameters[1805];
            let v10759 = -8.7498233534e1f64;
            let v10762 = -8.7498233534e1f64;
            let v10772 = -1e0f64;
            let v10781 = -3.33333333e-1f64;
            let v10786 = -1e0f64;
            let v10789 = 9e0f64;
            let v10790 = 2.222222222222222e-1f64;
            let v10792 = -1.333333333e0f64;
            let v10819 = -8.7498233534e1f64;
            let v10822 = -8.7498233534e1f64;
            let v10832 = -1e0f64;
            let v10840 = -3.33333333e-1f64;
            let v10845 = -1e0f64;
            let v10848 = 2.222222222222222e-1f64;
            let v10850 = -1.333333333e0f64;
            let v10962 = parameters[604];
            let v10969 = parameters[24];
            let v10980 = parameters[908];
            let v11051 = -1e-1f64;
            let v11053 = -1.0000000000000002e-10f64;
            let v11056 = 4.0000000000000007e-10f64;
            let v11074 = -1e3f64;
            let v11076 = -1.0000000000000002e-2f64;
            let v11079 = 4.000000000000001e-2f64;
            let v11132 = -8.7498233534e1f64;
            let v11135 = -8.7498233534e1f64;
            let v11145 = -1e0f64;
            let v11154 = -3.33333333e-1f64;
            let v11159 = -1e0f64;
            let v11162 = 2.222222222222222e-1f64;
            let v11164 = -1.333333333e0f64;
            let v11191 = -8.7498233534e1f64;
            let v11194 = -8.7498233534e1f64;
            let v11204 = -1e0f64;
            let v11212 = -3.33333333e-1f64;
            let v11217 = -1e0f64;
            let v11220 = 2.222222222222222e-1f64;
            let v11222 = -1.333333333e0f64;
            let v11292 = 0e0f64;
            let v11304 = 0e0f64;
            let v11320 = -8.7498233534e1f64;
            let v11326 = -1e0f64;
            let v11328 = -1e-8f64;
            let v11331 = 4e-8f64;
            let v11346 = -8.7498233534e1f64;
            let v11354 = 2.5e-9f64;
            let v11362 = -1e0f64;
            let v11364 = -1e-8f64;
            let v11367 = 4e-8f64;
            let v11377 = -1e3f64;
            let v11379 = -1.0000000000000002e-2f64;
            let v11382 = 4.000000000000001e-2f64;
            let v11436 = -8.7498233534e1f64;
            let v11439 = -8.7498233534e1f64;
            let v11449 = -1e0f64;
            let v11458 = -3.33333333e-1f64;
            let v11463 = -1e0f64;
            let v11466 = 2.222222222222222e-1f64;
            let v11468 = -1.333333333e0f64;
            let v11495 = -8.7498233534e1f64;
            let v11498 = -8.7498233534e1f64;
            let v11508 = -1e0f64;
            let v11516 = -3.33333333e-1f64;
            let v11521 = -1e0f64;
            let v11524 = 2.222222222222222e-1f64;
            let v11526 = -1.333333333e0f64;
            let v11605 = 0e0f64;
            let v11619 = 0e0f64;
            let v11706 = -1e-1f64;
            let v11708 = -1.0000000000000002e-10f64;
            let v11711 = 4.0000000000000007e-10f64;
            let v11729 = -1e3f64;
            let v11731 = -1.0000000000000002e-2f64;
            let v11734 = 4.000000000000001e-2f64;
            let v11787 = -8.7498233534e1f64;
            let v11790 = -8.7498233534e1f64;
            let v11800 = -1e0f64;
            let v11809 = -3.33333333e-1f64;
            let v11814 = -1e0f64;
            let v11817 = 2.222222222222222e-1f64;
            let v11819 = -1.333333333e0f64;
            let v11846 = -8.7498233534e1f64;
            let v11849 = -8.7498233534e1f64;
            let v11859 = -1e0f64;
            let v11867 = -3.33333333e-1f64;
            let v11872 = -1e0f64;
            let v11875 = 2.222222222222222e-1f64;
            let v11877 = -1.333333333e0f64;
            let v11950 = 6.25e-4f64;
            let v11952 = parameters[603];
            let v11969 = -8.7498233534e1f64;
            let v11974 = parameters[1529];
            let v12001 = 1.4142135623730951e0f64;
            let v12021 = -8.7498233534e1f64;
            let v12050 = 1e-15f64;
            let v12076 = -1e-15f64;
            let v12112 = 1e0f64;
            let v12134 = parameters[400];
            let v12169 = -1e-8f64;
            let v12171 = -1e-24f64;
            let v12174 = 4e-24f64;
            let v12203 = parameters[887];
            let v12204 = parameters[888];
            let v12215 = 0e0f64;
            let v12221 = 0e0f64;
            let v12224 = 0e0f64;
            let v12227 = -1e-8f64;
            let v12229 = -1e-24f64;
            let v12232 = 4e-24f64;
            let v12323 = -8.7498233534e1f64;
            let v12337 = -8.7498233534e1f64;
            let v12345 = -8.7498233534e1f64;
            let v12353 = -8.7498233534e1f64;
            let v12373 = -1e0f64;
            let v12388 = parameters[453];
            let v12407 = -1e-2f64;
            let v12409 = -1e-12f64;
            let v12412 = 4e-12f64;
            let v12441 = 0e0f64;
            let v12458 = 2.5e-7f64;
            let v12471 = -8.7498233534e1f64;
            let v12499 = node_potentials[8];
            let v12518 = node_potentials[2];
            let v12522 = parameters[921];
            let v12528 = parameters[911];
            let v12534 = node_potentials[9];
            let v12554 = node_potentials[0];
            let v12558 = parameters[922];
            let v12564 = parameters[914];
            let v12603 = parameters[25];
            let v12606 = 0e0f64;
            let v12653 = parameters[63];
            let v12656 = parameters[65];
            let v12663 = parameters[144];
            let v12684 = 1.4142135623730951e0f64;
            let v12704 = -8.7498233534e1f64;
            let v12707 = -1.2e0f64;
            let v12739 = -1e-15f64;
            let v12758 = node_potentials[7];
            let v12761 = parameters[454];
            let v12765 = parameters[1];
            let v12767 = parameters[530];
            let v12769 = parameters[491];
            let v12777 = -1e-1f64;
            let v12779 = -1.0000000000000002e-10f64;
            let v12782 = 4.0000000000000007e-10f64;
            let v12798 = parameters[143];
            let v12812 = 1.4142135623730951e0f64;
            let v12830 = -8.7498233534e1f64;
            let v12833 = -1.2e0f64;
            let v12848 = -1e-15f64;
            let v12862 = parameters[1441];
            let v12863 = -1e4f64;
            let v12881 = parameters[1442];
            let v12882 = -1e4f64;
            let v12902 = -3.7e1f64;
            let v12912 = parameters[1110];
            let v12928 = -9.82222e11f64;
            let v12933 = 3.75956e-7f64;
            let v12940 = parameters[27];
            let v12952 = -3.7e1f64;
            let v12965 = 8e-2f64;
            let v12978 = parameters[1111];
            let v12996 = -7.45669e11f64;
            let v13008 = parameters[1112];
            let v13036 = parameters[26];
            let v13060 = 2e-4f64;
            let v13070 = parameters[82];
            let v13074 = -1e-2f64;
            let v13076 = -1e-12f64;
            let v13079 = 4e-12f64;
            let v13098 = parameters[1104];
            let v13113 = -1e-2f64;
            let v13115 = -1e-12f64;
            let v13118 = 4e-12f64;
            let v13135 = parameters[1105];
            let v13152 = -1e2f64;
            let v13154 = -1e-4f64;
            let v13157 = 4e-4f64;
            let v13173 = -1e-2f64;
            let v13181 = -1e1f64;
            let v13183 = -1e-6f64;
            let v13186 = 4e-6f64;
            let v13211 = -1e-2f64;
            let v13216 = -1e1f64;
            let v13218 = -1e-6f64;
            let v13221 = 4e-6f64;
            let v13247 = -1e1f64;
            let v13249 = -1e-6f64;
            let v13252 = 4e-6f64;
            let v13272 = -1e2f64;
            let v13274 = -1e-4f64;
            let v13277 = 4e-4f64;
            let v13294 = -1e-2f64;
            let v13304 = -1e2f64;
            let v13306 = -1e-4f64;
            let v13309 = 4e-4f64;
            let v13325 = -1e-2f64;
            let v13332 = -1e1f64;
            let v13334 = -1e-6f64;
            let v13337 = 4e-6f64;
            let v13362 = -1e-2f64;
            let v13367 = -1e1f64;
            let v13369 = -1e-6f64;
            let v13372 = 4e-6f64;
            let v13390 = -1e1f64;
            let v13392 = -1e-6f64;
            let v13395 = 4e-6f64;
            let v13415 = -1e2f64;
            let v13417 = -1e-4f64;
            let v13420 = 4e-4f64;
            let v13437 = -1e-2f64;
            let v13462 = parameters[1643];
            let v13468 = parameters[1645];
            let v13474 = parameters[1647];
            let v13501 = parameters[1644];
            let v13507 = parameters[1646];
            let v13513 = parameters[1648];
            let v13731 = parameters[1683];
            let v13734 = parameters[1684];
            let v13737 = parameters[1687];
            let v13741 = parameters[79];
            let v13747 = parameters[1681];
            let v13754 = -8.7498233534e1f64;
            let v13768 = parameters[1688];
            let v13776 = 4.112842231783458e-57f64;
            let v13781 = 1e10f64;
            let v13796 = -8.7498233534e1f64;
            let v13861 = parameters[1685];
            let v13872 = parameters[1686];
            let v13880 = -8.7498233534e1f64;
            let v13886 = 4.112739300563051e-57f64;
            let v13893 = 1e14f64;
            let v13901 = parameters[72];
            let v13911 = parameters[1707];
            let v13919 = parameters[1708];
            let v13920 = parameters[1709];
            let v13925 = parameters[1710];
            let v13926 = parameters[1711];
            let v13931 = parameters[1712];
            let v13932 = parameters[1713];
            let v13937 = parameters[1714];
            let v13938 = parameters[1715];
            let v13945 = 7.5e0f64;
            let v13948 = 2.5298e0f64;
            let v13962 = -1e-2f64;
            let v13964 = -1e-12f64;
            let v13967 = 4e-12f64;
            let v14088 = 1.5e1f64;
            let v14111 = parameters[1716];
            let v14141 = parameters[1910];
            let v14143 = parameters[1912];
            let v14147 = -1e1f64;
            let v14149 = -1e-6f64;
            let v14152 = 4e-6f64;
            let v14158 = parameters[1904];
            let v14160 = parameters[1913];
            let v14178 = -1e1f64;
            let v14180 = -1e-6f64;
            let v14183 = 4e-6f64;
            let v14190 = parameters[1906];
            let v14195 = 1e0f64;
            let v14200 = parameters[1907];
            let v14206 = parameters[1905];
            let v14210 = -1e5f64;
            let v14212 = -1e2f64;
            let v14215 = 4e2f64;
            let v14226 = parameters[1917];
            let v14228 = parameters[1916];
            let v14231 = 6.25e-2f64;
            let v14238 = parameters[1903];
            let v14245 = parameters[1908];
            let v14248 = parameters[1914];
            let v14261 = parameters[1911];
            let v14267 = -1e1f64;
            let v14269 = -1e-6f64;
            let v14272 = 4e-6f64;
            let v14296 = -1e1f64;
            let v14298 = -1e-6f64;
            let v14301 = 4e-6f64;
            let v14312 = 1e0f64;
            let v14325 = -1e5f64;
            let v14327 = -1e2f64;
            let v14330 = 4e2f64;
            let v14376 = parameters[1909];
            let v14387 = parameters[1915];
            let v14462 = 3.20438e-19f64;
            let v14466 = 3.20438e-19f64;
            let v14470 = 3.20438e-19f64;
            let v14474 = 3.20438e-19f64;
            let v14478 = 3.20438e-19f64;
            let v14482 = 3.20438e-19f64;
            let v14485 = 3.20438e-19f64;
            let v14540 = 3.20438e-19f64;
            let v14600 = 1e0f64;
            let v14601 = Lanes([1e0f64; 1]);
            let v14602 = Lanes([1e0f64; 1]);
            let v14603 = Lanes([1e0f64; 1]);
            let v14604 = Lanes([1e0f64; 1]);
            let v14605 = Lanes([1e0f64; 1]);
            let v14864 = Lanes([0e0f64; 1]);
            let v14869 = 2e0f64;
            let v14888 = -1e0f64;
            let v16366 = Lanes([0e0f64; 3]);
            let v16392 = Lanes([0e0f64; 3]);
            let v16573 = -3.33333333e-1f64;
            let v16697 = -3.33333333e-1f64;
            let v16802 = Lanes([0e0f64; 5]);
            let v17222 = -3.33333333e-1f64;
            if v2 != 0.0 {
                let v10 = if (if (if v3 == v0 { 1.0 } else { 0.0 }) != 0.0 || (if v5 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v8 == v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v10 != 0.0 {
                } else {
                }
            } else {
            }
            let v12 = if v11 == v1 { 1.0 } else { 0.0 };
            let v9888: f64;
            if v12 != 0.0 {
                v9888 = v1;
            } else {
                v9888 = v13;
            }
            let v16 = v14 * v15;
            let v18 = v17 * v15;
            let v21 = v14 / v19;
            let v37 = v28 * v32;
            let v51 = ((v40 + ((v25 * v41) / v28)) + (v45 / v32)) + ((v48 * v25) / v37);
            let v63 = ((v52 + ((v25 * v53) / v28)) + (v57 / v32)) + ((v60 * v25) / v37);
            let v64 = v28 + (((v24 + ((v25 * v26) / v28)) + (v31 / v32)) + ((v35 * v25) / v37));
            let v65 = if v64 <= v0 { 1.0 } else { 0.0 };
            let v66: f64;
            if v65 != 0.0 {
                v66 = v28;
            } else {
                v66 = v64;
            }
            let v68 = -v67;
            let v69 = v66.powf(v68);
            let v73 = v66 + v51;
            let v82 = v66 - (v23 * (v63 + (v70 * v69)));
            let v84 = v73 - (v23 * (v63 + (v70 * (v73.powf(v68)))));
            let v86 = v66 - (v23 * (v77 + (v78 * v69)));
            let v88 = v86 - v87;
            let v89 = if v82 <= v0 { 1.0 } else { 0.0 };
            let v3969: f64;
            if v89 != 0.0 {
                v3969 = v66;
            } else {
                let v91 = if v82 <= v90 { 1.0 } else { 0.0 };
                if v91 != 0.0 {
                } else {
                }
                v3969 = v82;
            }
            let v92 = if v84 <= v0 { 1.0 } else { 0.0 };
            let v154: f64;
            if v92 != 0.0 {
                v154 = v66;
            } else {
                let v93 = if v84 <= v90 { 1.0 } else { 0.0 };
                if v93 != 0.0 {
                } else {
                }
                v154 = v84;
            }
            let v94 = if v86 <= v0 { 1.0 } else { 0.0 };
            let v4481: f64;
            if v94 != 0.0 {
                v4481 = v66;
            } else {
                let v95 = if v86 <= v90 { 1.0 } else { 0.0 };
                if v95 != 0.0 {
                } else {
                }
                v4481 = v86;
            }
            let v97 = if v96 != v0 { 1.0 } else { 0.0 };
            if v97 != 0.0 {
                let v98 = if v88 <= v0 { 1.0 } else { 0.0 };
                if v98 != 0.0 {
                } else {
                    let v99 = if v88 <= v90 { 1.0 } else { 0.0 };
                    if v99 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v102 = if v100 == v101 { 1.0 } else { 0.0 };
            let v146: f64;
            let v148: f64;
            if v102 != 0.0 {
                let v123 = v28 * v117;
                let v125 = ((((v103 + ((v25 * v104) / v28)) + (v108 / v32)) + ((v111 * v25) / v37)) + ((v25 * v115) / v117)) + ((v120 * v121) / v123);
                let v145 = ((((v126 + ((v25 * v127) / v28)) + (v131 / v32)) + ((v134 * v25) / v37)) + ((v25 * v138) / v117)) + ((v142 * v121) / v123);
                v146 = v125;
                v148 = v145;
            } else {
                v146 = v0;
                v148 = v0;
            }
            let v147 = v117 + v146;
            let v149 = v147 + v148;
            let v159: f64;
            if v102 != 0.0 {
                let v150 = if v149 <= v0 { 1.0 } else { 0.0 };
                let v160: f64;
                if v150 != 0.0 {
                    v160 = v117;
                } else {
                    let v151 = if v149 <= v90 { 1.0 } else { 0.0 };
                    if v151 != 0.0 {
                    } else {
                    }
                    v160 = v149;
                }
                v159 = v160;
            } else {
                v159 = v149;
            }
            let v153 = v32 * v152;
            let v155 = v25 / v154;
            let v156 = v1 / v32;
            let v158 = v25 / (v154 * v32);
            let v174: f64;
            let v177: f64;
            if v102 != 0.0 {
                let v161 = v25 / v159;
                let v163 = v121 / (v159 * v154);
                v174 = v161;
                v177 = v163;
            } else {
                v174 = v0;
                v177 = v0;
            }
            let v179 = ((((v164 + (v155 * v165)) + (v156 * v168)) + (v158 * v171)) + (v174 * v0)) + (v177 * v0);
            let v181 = if v180 != v0 { 1.0 } else { 0.0 };
            let v194: f64;
            if v181 != 0.0 {
                let v184 = v1 + (v32 / v182);
                let v186 = if v184 > v185 { 1.0 } else { 0.0 };
                let v190: f64;
                if v186 != 0.0 {
                    let v187 = v184.ln();
                    v190 = v187;
                } else {
                    v190 = v188;
                }
                let v193 = v179 * (v1 + ((v180 / v32) * v190));
                v194 = v193;
            } else {
                v194 = v179;
            }
            let v195 = if v194 <= v0 { 1.0 } else { 0.0 };
            let v417: f64;
            if v195 != 0.0 {
                v417 = v196;
            } else {
                let v198 = if v194 <= v197 { 1.0 } else { 0.0 };
                if v198 != 0.0 {
                } else {
                }
                v417 = v194;
            }
            let v199 = if v100 == v0 { 1.0 } else { 0.0 };
            let v378: f64;
            let v388: f64;
            let v399: f64;
            if v199 != 0.0 {
                let v204 = if (if v200 == v0 { 1.0 } else { 0.0 }) != 0.0 || (if v202 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v379: f64;
                let v389: f64;
                let v400: f64;
                if v204 != 0.0 {
                    let v206 = v23 * v205;
                    let v210 = ((v206 * v19) * v15) / v209;
                    let v212 = v205 * v211;
                    v379 = v210;
                    v389 = v206;
                    v400 = v212;
                } else {
                    let v214 = v200 - v202;
                    let v219 = v23 * (((v205 * v205) + ((v214 * v214) / v22)).sqrt());
                    let v222 = ((v219 * v19) * v15) / v209;
                    let v225 = (v205 * (v200 + v202)) / v23;
                    v379 = v222;
                    v389 = v219;
                    v400 = v225;
                }
                v378 = v379;
                v388 = v389;
                v399 = v400;
            } else {
                let v226 = if v100 == v1 { 1.0 } else { 0.0 };
                let v380: f64;
                let v390: f64;
                let v401: f64;
                if v226 != 0.0 {
                    let v229 = if (if v200 == v0 { 1.0 } else { 0.0 }) != 0.0 || (if v202 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v381: f64;
                    let v391: f64;
                    let v402: f64;
                    if v229 != 0.0 {
                        let v231 = (v23 * v205) + v211;
                        let v234 = ((v231 * v19) * v15) / v209;
                        let v235 = v205 * v211;
                        v381 = v234;
                        v391 = v231;
                        v402 = v235;
                    } else {
                        let v237 = v200 - v202;
                        let v243 = (v23 * (((v205 * v205) + ((v237 * v237) / v22)).sqrt())) + v200;
                        let v246 = ((v243 * v19) * v15) / v209;
                        let v249 = (v205 * (v200 + v202)) / v23;
                        v381 = v246;
                        v391 = v243;
                        v402 = v249;
                    }
                    v380 = v381;
                    v390 = v391;
                    v401 = v402;
                } else {
                    let v250 = if v100 == v23 { 1.0 } else { 0.0 };
                    let v382: f64;
                    let v392: f64;
                    let v403: f64;
                    if v250 != 0.0 {
                        let v253 = if (if v200 == v0 { 1.0 } else { 0.0 }) != 0.0 || (if v202 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v383: f64;
                        let v393: f64;
                        let v404: f64;
                        if v253 != 0.0 {
                            let v256 = (v23 * v205) + (v23 * v211);
                            let v259 = ((v256 * v19) * v15) / v209;
                            let v260 = v205 * v211;
                            v383 = v259;
                            v393 = v256;
                            v404 = v260;
                        } else {
                            let v262 = v200 - v202;
                            let v269 = ((v23 * (((v205 * v205) + ((v262 * v262) / v22)).sqrt())) + v200) + v202;
                            let v272 = ((v269 * v19) * v15) / v209;
                            let v275 = (v205 * (v200 + v202)) / v23;
                            v383 = v272;
                            v393 = v269;
                            v404 = v275;
                        }
                        v382 = v383;
                        v392 = v393;
                        v403 = v404;
                    } else {
                        let v277 = if v100 == v276 { 1.0 } else { 0.0 };
                        let v384: f64;
                        let v394: f64;
                        let v405: f64;
                        if v277 != 0.0 {
                            let v280 = v278 * v279;
                            let v283 = v1 + ((v23 * v209) / v279);
                            let v284 = if v283 > v185 { 1.0 } else { 0.0 };
                            let v290: f64;
                            if v284 != 0.0 {
                                let v285 = v283.ln();
                                v290 = v285;
                            } else {
                                v290 = v286;
                            }
                            let v291 = ((v287 * v19) * v15) / v290;
                            let v293 = (v280 * v279) / v22;
                            v384 = v291;
                            v394 = v280;
                            v405 = v293;
                        } else {
                            let v294 = if v100 == v22 { 1.0 } else { 0.0 };
                            let v385: f64;
                            let v395: f64;
                            let v406: f64;
                            if v294 != 0.0 {
                                v385 = v296;
                                v395 = v295;
                                v406 = v297;
                            } else {
                                let v386: f64;
                                let v396: f64;
                                let v407: f64;
                                if v102 != 0.0 {
                                    let v300 = v23 * (v147 + v298);
                                    let v302 = v300 + v301;
                                    let v303 = v147 * v298;
                                    let v305 = v303 + v304;
                                    let v307 = if v306 > v1 { 1.0 } else { 0.0 };
                                    let v319: f64;
                                    let v322: f64;
                                    let v374: f64;
                                    let v412: f64;
                                    if v307 != 0.0 {
                                        let v309 = v300 + v308;
                                        let v311 = v303 + v310;
                                        let v312 = v302 + v309;
                                        let v313 = v305 + v311;
                                        v319 = v309;
                                        v322 = v311;
                                        v374 = v312;
                                        v412 = v313;
                                    } else {
                                        v319 = v0;
                                        v322 = v0;
                                        v374 = v302;
                                        v412 = v305;
                                    }
                                    let v314 = if v306 > v23 { 1.0 } else { 0.0 };
                                    let v331: f64;
                                    let v335: f64;
                                    let v373: f64;
                                    let v411: f64;
                                    if v314 != 0.0 {
                                        let v316 = v300 + v315;
                                        let v318 = v303 + v317;
                                        let v321 = (v302 + v319) + v316;
                                        let v324 = (v305 + v322) + v318;
                                        v331 = v316;
                                        v335 = v318;
                                        v373 = v321;
                                        v411 = v324;
                                    } else {
                                        v331 = v0;
                                        v335 = v0;
                                        v373 = v374;
                                        v411 = v412;
                                    }
                                    let v325 = if v306 > v276 { 1.0 } else { 0.0 };
                                    let v345: f64;
                                    let v350: f64;
                                    let v372: f64;
                                    let v410: f64;
                                    if v325 != 0.0 {
                                        let v327 = v300 + v326;
                                        let v329 = v303 + v328;
                                        let v333 = ((v302 + v319) + v331) + v327;
                                        let v337 = ((v305 + v322) + v335) + v329;
                                        v345 = v327;
                                        v350 = v329;
                                        v372 = v333;
                                        v410 = v337;
                                    } else {
                                        v345 = v0;
                                        v350 = v0;
                                        v372 = v373;
                                        v410 = v411;
                                    }
                                    let v338 = if v306 > v22 { 1.0 } else { 0.0 };
                                    let v361: f64;
                                    let v367: f64;
                                    let v371: f64;
                                    let v409: f64;
                                    if v338 != 0.0 {
                                        let v340 = v300 + v339;
                                        let v342 = v303 + v341;
                                        let v347 = (((v302 + v319) + v331) + v345) + v340;
                                        let v352 = (((v305 + v322) + v335) + v350) + v342;
                                        v361 = v340;
                                        v367 = v342;
                                        v371 = v347;
                                        v409 = v352;
                                    } else {
                                        v361 = v0;
                                        v367 = v0;
                                        v371 = v372;
                                        v409 = v410;
                                    }
                                    let v353 = if v306 > v101 { 1.0 } else { 0.0 };
                                    let v370: f64;
                                    let v408: f64;
                                    if v353 != 0.0 {
                                        let v363 = ((((v302 + v319) + v331) + v345) + v361) + (v300 + v354);
                                        let v369 = ((((v305 + v322) + v335) + v350) + v367) + (v303 + v356);
                                        v370 = v363;
                                        v408 = v369;
                                    } else {
                                        v370 = v371;
                                        v408 = v409;
                                    }
                                    let v377 = ((v370 * v19) * v15) / v209;
                                    v386 = v377;
                                    v396 = v370;
                                    v407 = v408;
                                } else {
                                    v386 = v0;
                                    v396 = v0;
                                    v407 = v0;
                                }
                                v385 = v386;
                                v395 = v396;
                                v406 = v407;
                            }
                            v384 = v385;
                            v394 = v395;
                            v405 = v406;
                        }
                        v382 = v384;
                        v392 = v394;
                        v403 = v405;
                    }
                    v380 = v382;
                    v390 = v392;
                    v401 = v403;
                }
                v378 = v380;
                v388 = v390;
                v399 = v401;
            }
            let v414 = (v23 * v378) / (((v388 * v388) * v16) / v399);
            let v420 = ((v416 * v417) * v399) / v378;
            let v421 = v378 / v388;
            if v97 != 0.0 {
            } else {
            }
            let v423 = v388 - v422;
            let v425 = v388 - v424;
            let v430: f64;
            if v102 != 0.0 {
                let v429 = v423 - ((v23 * v306) * v427);
                v430 = v429;
            } else {
                v430 = v423;
            }
            if v102 != 0.0 {
                if v97 != 0.0 {
                    let v431 = if v430 <= v0 { 1.0 } else { 0.0 };
                    if v431 != 0.0 {
                    } else {
                        let v432 = if v159 <= v90 { 1.0 } else { 0.0 };
                        if v432 != 0.0 {
                        } else {
                        }
                    }
                } else {
                }
            } else {
            }
            let v450 = ((((v435 + (v155 * v436)) + (v156 * v439)) + (v158 * v442)) + (v174 * v445)) + (v177 * v448);
            let v466 = ((((v451 + (v155 * v452)) + (v156 * v455)) + (v158 * v458)) + (v174 * v461)) + (v177 * v464);
            let v482 = ((((v467 + (v155 * v468)) + (v156 * v471)) + (v158 * v474)) + (v174 * v477)) + (v177 * v480);
            let v498 = ((((v483 + (v155 * v484)) + (v156 * v487)) + (v158 * v490)) + (v174 * v493)) + (v177 * v496);
            let v514 = ((((v499 + (v155 * v500)) + (v156 * v503)) + (v158 * v506)) + (v174 * v509)) + (v177 * v512);
            let v530 = ((((v515 + (v155 * v516)) + (v156 * v519)) + (v158 * v522)) + (v174 * v525)) + (v177 * v528);
            let v546 = ((((v531 + (v155 * v532)) + (v156 * v535)) + (v158 * v538)) + (v174 * v541)) + (v177 * v544);
            let v562 = ((((v547 + (v155 * v548)) + (v156 * v551)) + (v158 * v554)) + (v174 * v557)) + (v177 * v560);
            let v578 = ((((v563 + (v155 * v564)) + (v156 * v567)) + (v158 * v570)) + (v174 * v573)) + (v177 * v576);
            let v594 = ((((v579 + (v155 * v580)) + (v156 * v583)) + (v158 * v586)) + (v174 * v589)) + (v177 * v592);
            let v610 = ((((v595 + (v155 * v596)) + (v156 * v599)) + (v158 * v602)) + (v174 * v605)) + (v177 * v608);
            let v626 = ((((v611 + (v155 * v612)) + (v156 * v615)) + (v158 * v618)) + (v174 * v621)) + (v177 * v624);
            let v642 = ((((v627 + (v155 * v628)) + (v156 * v631)) + (v158 * v634)) + (v174 * v637)) + (v177 * v640);
            let v658 = ((((v643 + (v155 * v644)) + (v156 * v647)) + (v158 * v650)) + (v174 * v653)) + (v177 * v656);
            let v674 = ((((v659 + (v155 * v660)) + (v156 * v663)) + (v158 * v666)) + (v174 * v669)) + (v177 * v672);
            let v690 = ((((v675 + (v155 * v676)) + (v156 * v679)) + (v158 * v682)) + (v174 * v685)) + (v177 * v688);
            let v706 = ((((v691 + (v155 * v692)) + (v156 * v695)) + (v158 * v698)) + (v174 * v701)) + (v177 * v704);
            let v722 = ((((v707 + (v155 * v708)) + (v156 * v711)) + (v158 * v714)) + (v174 * v717)) + (v177 * v720);
            let v738 = ((((v723 + (v155 * v724)) + (v156 * v727)) + (v158 * v730)) + (v174 * v733)) + (v177 * v736);
            let v754 = ((((v739 + (v155 * v740)) + (v156 * v743)) + (v158 * v746)) + (v174 * v749)) + (v177 * v752);
            let v770 = ((((v755 + (v155 * v756)) + (v156 * v759)) + (v158 * v762)) + (v174 * v765)) + (v177 * v768);
            let v786 = ((((v771 + (v155 * v772)) + (v156 * v775)) + (v158 * v778)) + (v174 * v781)) + (v177 * v784);
            let v802 = ((((v787 + (v155 * v788)) + (v156 * v791)) + (v158 * v794)) + (v174 * v797)) + (v177 * v800);
            let v818 = ((((v803 + (v155 * v804)) + (v156 * v807)) + (v158 * v810)) + (v174 * v813)) + (v177 * v816);
            let v834 = ((((v819 + (v155 * v820)) + (v156 * v823)) + (v158 * v826)) + (v174 * v829)) + (v177 * v832);
            let v850 = ((((v835 + (v155 * v836)) + (v156 * v839)) + (v158 * v842)) + (v174 * v845)) + (v177 * v848);
            let v866 = ((((v851 + (v155 * v852)) + (v156 * v855)) + (v158 * v858)) + (v174 * v861)) + (v177 * v864);
            let v882 = ((((v867 + (v155 * v868)) + (v156 * v871)) + (v158 * v874)) + (v174 * v877)) + (v177 * v880);
            let v898 = ((((v883 + (v155 * v884)) + (v156 * v887)) + (v158 * v890)) + (v174 * v893)) + (v177 * v896);
            let v914 = ((((v899 + (v155 * v900)) + (v156 * v903)) + (v158 * v906)) + (v174 * v909)) + (v177 * v912);
            let v930 = ((((v915 + (v155 * v916)) + (v156 * v919)) + (v158 * v922)) + (v174 * v925)) + (v177 * v928);
            let v946 = ((((v931 + (v155 * v932)) + (v156 * v935)) + (v158 * v938)) + (v174 * v941)) + (v177 * v944);
            let v962 = ((((v947 + (v155 * v948)) + (v156 * v951)) + (v158 * v954)) + (v174 * v957)) + (v177 * v960);
            let v978 = ((((v963 + (v155 * v964)) + (v156 * v967)) + (v158 * v970)) + (v174 * v973)) + (v177 * v976);
            let v994 = ((((v979 + (v155 * v980)) + (v156 * v983)) + (v158 * v986)) + (v174 * v989)) + (v177 * v992);
            let v1010 = ((((v995 + (v155 * v996)) + (v156 * v999)) + (v158 * v1002)) + (v174 * v1005)) + (v177 * v1008);
            let v1026 = ((((v1011 + (v155 * v1012)) + (v156 * v1015)) + (v158 * v1018)) + (v174 * v1021)) + (v177 * v1024);
            let v1042 = ((((v1027 + (v155 * v1028)) + (v156 * v1031)) + (v158 * v1034)) + (v174 * v1037)) + (v177 * v1040);
            let v1058 = ((((v1043 + (v155 * v1044)) + (v156 * v1047)) + (v158 * v1050)) + (v174 * v1053)) + (v177 * v1056);
            let v1074 = ((((v1059 + (v155 * v1060)) + (v156 * v1063)) + (v158 * v1066)) + (v174 * v1069)) + (v177 * v1072);
            let v1090 = ((((v1075 + (v155 * v1076)) + (v156 * v1079)) + (v158 * v1082)) + (v174 * v1085)) + (v177 * v1088);
            let v1106 = ((((v1091 + (v155 * v1092)) + (v156 * v1095)) + (v158 * v1098)) + (v174 * v1101)) + (v177 * v1104);
            let v1122 = ((((v1107 + (v155 * v1108)) + (v156 * v1111)) + (v158 * v1114)) + (v174 * v1117)) + (v177 * v1120);
            let v1138 = ((((v1123 + (v155 * v1124)) + (v156 * v1127)) + (v158 * v1130)) + (v174 * v1133)) + (v177 * v1136);
            let v1154 = ((((v1139 + (v155 * v1140)) + (v156 * v1143)) + (v158 * v1146)) + (v174 * v1149)) + (v177 * v1152);
            let v1170 = ((((v1155 + (v155 * v1156)) + (v156 * v1159)) + (v158 * v1162)) + (v174 * v1165)) + (v177 * v1168);
            let v1186 = ((((v1171 + (v155 * v1172)) + (v156 * v1175)) + (v158 * v1178)) + (v174 * v1181)) + (v177 * v1184);
            let v1202 = ((((v1187 + (v155 * v1188)) + (v156 * v1191)) + (v158 * v1194)) + (v174 * v1197)) + (v177 * v1200);
            let v1218 = ((((v1203 + (v155 * v1204)) + (v156 * v1207)) + (v158 * v1210)) + (v174 * v1213)) + (v177 * v1216);
            let v1234 = ((((v1219 + (v155 * v1220)) + (v156 * v1223)) + (v158 * v1226)) + (v174 * v1229)) + (v177 * v1232);
            let v1250 = ((((v1235 + (v155 * v1236)) + (v156 * v1239)) + (v158 * v1242)) + (v174 * v1245)) + (v177 * v1248);
            let v1266 = ((((v1251 + (v155 * v1252)) + (v156 * v1255)) + (v158 * v1258)) + (v174 * v1261)) + (v177 * v1264);
            let v1282 = ((((v1267 + (v155 * v1268)) + (v156 * v1271)) + (v158 * v1274)) + (v174 * v1277)) + (v177 * v1280);
            let v1298 = ((((v1283 + (v155 * v1284)) + (v156 * v1287)) + (v158 * v1290)) + (v174 * v1293)) + (v177 * v1296);
            let v1314 = ((((v1299 + (v155 * v1300)) + (v156 * v1303)) + (v158 * v1306)) + (v174 * v1309)) + (v177 * v1312);
            let v1330 = ((((v1315 + (v155 * v1316)) + (v156 * v1319)) + (v158 * v1322)) + (v174 * v1325)) + (v177 * v1328);
            let v1346 = ((((v1331 + (v155 * v1332)) + (v156 * v1335)) + (v158 * v1338)) + (v174 * v1341)) + (v177 * v1344);
            let v1362 = ((((v1347 + (v155 * v1348)) + (v156 * v1351)) + (v158 * v1354)) + (v174 * v1357)) + (v177 * v1360);
            let v1378 = ((((v1363 + (v155 * v1364)) + (v156 * v1367)) + (v158 * v1370)) + (v174 * v1373)) + (v177 * v1376);
            let v1394 = ((((v1379 + (v155 * v1380)) + (v156 * v1383)) + (v158 * v1386)) + (v174 * v1389)) + (v177 * v1392);
            let v1410 = ((((v1395 + (v155 * v1396)) + (v156 * v1399)) + (v158 * v1402)) + (v174 * v1405)) + (v177 * v1408);
            let v1426 = ((((v1411 + (v155 * v1412)) + (v156 * v1415)) + (v158 * v1418)) + (v174 * v1421)) + (v177 * v1424);
            let v1442 = ((((v1427 + (v155 * v1428)) + (v156 * v1431)) + (v158 * v1434)) + (v174 * v1437)) + (v177 * v1440);
            let v1458 = ((((v1443 + (v155 * v1444)) + (v156 * v1447)) + (v158 * v1450)) + (v174 * v1453)) + (v177 * v1456);
            let v1474 = ((((v1459 + (v155 * v1460)) + (v156 * v1463)) + (v158 * v1466)) + (v174 * v1469)) + (v177 * v1472);
            let v1490 = ((((v1475 + (v155 * v1476)) + (v156 * v1479)) + (v158 * v1482)) + (v174 * v1485)) + (v177 * v1488);
            let v1506 = ((((v1491 + (v155 * v1492)) + (v156 * v1495)) + (v158 * v1498)) + (v174 * v1501)) + (v177 * v1504);
            let v1522 = ((((v1507 + (v155 * v1508)) + (v156 * v1511)) + (v158 * v1514)) + (v174 * v1517)) + (v177 * v1520);
            let v1538 = ((((v1523 + (v155 * v1524)) + (v156 * v1527)) + (v158 * v1530)) + (v174 * v1533)) + (v177 * v1536);
            let v1554 = ((((v1539 + (v155 * v1540)) + (v156 * v1543)) + (v158 * v1546)) + (v174 * v1549)) + (v177 * v1552);
            let v1570 = ((((v1555 + (v155 * v1556)) + (v156 * v1559)) + (v158 * v1562)) + (v174 * v1565)) + (v177 * v1568);
            let v1586 = ((((v1571 + (v155 * v1572)) + (v156 * v1575)) + (v158 * v1578)) + (v174 * v1581)) + (v177 * v1584);
            let v1602 = ((((v1587 + (v155 * v1588)) + (v156 * v1591)) + (v158 * v1594)) + (v174 * v1597)) + (v177 * v1600);
            let v1618 = ((((v1603 + (v155 * v1604)) + (v156 * v1607)) + (v158 * v1610)) + (v174 * v1613)) + (v177 * v1616);
            let v1634 = ((((v1619 + (v155 * v1620)) + (v156 * v1623)) + (v158 * v1626)) + (v174 * v1629)) + (v177 * v1632);
            let v1650 = ((((v1635 + (v155 * v1636)) + (v156 * v1639)) + (v158 * v1642)) + (v174 * v1645)) + (v177 * v1648);
            let v1666 = ((((v1651 + (v155 * v1652)) + (v156 * v1655)) + (v158 * v1658)) + (v174 * v1661)) + (v177 * v1664);
            let v1682 = ((((v1667 + (v155 * v1668)) + (v156 * v1671)) + (v158 * v1674)) + (v174 * v1677)) + (v177 * v1680);
            let v1698 = ((((v1683 + (v155 * v1684)) + (v156 * v1687)) + (v158 * v1690)) + (v174 * v1693)) + (v177 * v1696);
            let v1714 = ((((v1699 + (v155 * v1700)) + (v156 * v1703)) + (v158 * v1706)) + (v174 * v1709)) + (v177 * v1712);
            let v1730 = ((((v1715 + (v155 * v1716)) + (v156 * v1719)) + (v158 * v1722)) + (v174 * v1725)) + (v177 * v1728);
            let v1746 = ((((v1731 + (v155 * v1732)) + (v156 * v1735)) + (v158 * v1738)) + (v174 * v1741)) + (v177 * v1744);
            let v1762 = ((((v1747 + (v155 * v1748)) + (v156 * v1751)) + (v158 * v1754)) + (v174 * v1757)) + (v177 * v1760);
            let v1778 = ((((v1763 + (v155 * v1764)) + (v156 * v1767)) + (v158 * v1770)) + (v174 * v1773)) + (v177 * v1776);
            let v1794 = ((((v1779 + (v155 * v1780)) + (v156 * v1783)) + (v158 * v1786)) + (v174 * v1789)) + (v177 * v1792);
            let v1810 = ((((v1795 + (v155 * v1796)) + (v156 * v1799)) + (v158 * v1802)) + (v174 * v1805)) + (v177 * v1808);
            let v1826 = ((((v1811 + (v155 * v1812)) + (v156 * v1815)) + (v158 * v1818)) + (v174 * v1821)) + (v177 * v1824);
            let v1842 = ((((v1827 + (v155 * v1828)) + (v156 * v1831)) + (v158 * v1834)) + (v174 * v1837)) + (v177 * v1840);
            let v1858 = ((((v1843 + (v155 * v1844)) + (v156 * v1847)) + (v158 * v1850)) + (v174 * v1853)) + (v177 * v1856);
            let v1874 = ((((v1859 + (v155 * v1860)) + (v156 * v1863)) + (v158 * v1866)) + (v174 * v1869)) + (v177 * v1872);
            let v1890 = ((((v1875 + (v155 * v1876)) + (v156 * v1879)) + (v158 * v1882)) + (v174 * v1885)) + (v177 * v1888);
            let v1906 = ((((v1891 + (v155 * v1892)) + (v156 * v1895)) + (v158 * v1898)) + (v174 * v1901)) + (v177 * v1904);
            let v1922 = ((((v1907 + (v155 * v1908)) + (v156 * v1911)) + (v158 * v1914)) + (v174 * v1917)) + (v177 * v1920);
            let v1938 = ((((v1923 + (v155 * v1924)) + (v156 * v1927)) + (v158 * v1930)) + (v174 * v1933)) + (v177 * v1936);
            let v1954 = ((((v1939 + (v155 * v1940)) + (v156 * v1943)) + (v158 * v1946)) + (v174 * v1949)) + (v177 * v1952);
            let v1970 = ((((v1955 + (v155 * v1956)) + (v156 * v1959)) + (v158 * v1962)) + (v174 * v1965)) + (v177 * v1968);
            let v1986 = ((((v1971 + (v155 * v1972)) + (v156 * v1975)) + (v158 * v1978)) + (v174 * v1981)) + (v177 * v1984);
            let v2002 = ((((v1987 + (v155 * v1988)) + (v156 * v1991)) + (v158 * v1994)) + (v174 * v1997)) + (v177 * v2000);
            let v2018 = ((((v2003 + (v155 * v2004)) + (v156 * v2007)) + (v158 * v2010)) + (v174 * v2013)) + (v177 * v2016);
            let v2034 = ((((v2019 + (v155 * v2020)) + (v156 * v2023)) + (v158 * v2026)) + (v174 * v2029)) + (v177 * v2032);
            let v2050 = ((((v2035 + (v155 * v2036)) + (v156 * v2039)) + (v158 * v2042)) + (v174 * v2045)) + (v177 * v2048);
            let v2066 = ((((v2051 + (v155 * v2052)) + (v156 * v2055)) + (v158 * v2058)) + (v174 * v2061)) + (v177 * v2064);
            let v2082 = ((((v2067 + (v155 * v2068)) + (v156 * v2071)) + (v158 * v2074)) + (v174 * v2077)) + (v177 * v2080);
            let v2098 = ((((v2083 + (v155 * v2084)) + (v156 * v2087)) + (v158 * v2090)) + (v174 * v2093)) + (v177 * v2096);
            let v2114 = ((((v2099 + (v155 * v2100)) + (v156 * v2103)) + (v158 * v2106)) + (v174 * v2109)) + (v177 * v2112);
            let v2130 = ((((v2115 + (v155 * v2116)) + (v156 * v2119)) + (v158 * v2122)) + (v174 * v2125)) + (v177 * v2128);
            let v2146 = ((((v2131 + (v155 * v2132)) + (v156 * v2135)) + (v158 * v2138)) + (v174 * v2141)) + (v177 * v2144);
            let v2162 = ((((v2147 + (v155 * v2148)) + (v156 * v2151)) + (v158 * v2154)) + (v174 * v2157)) + (v177 * v2160);
            let v2178 = ((((v2163 + (v155 * v2164)) + (v156 * v2167)) + (v158 * v2170)) + (v174 * v2173)) + (v177 * v2176);
            let v2194 = ((((v2179 + (v155 * v2180)) + (v156 * v2183)) + (v158 * v2186)) + (v174 * v2189)) + (v177 * v2192);
            let v2210 = ((((v2195 + (v155 * v2196)) + (v156 * v2199)) + (v158 * v2202)) + (v174 * v2205)) + (v177 * v2208);
            let v2226 = ((((v2211 + (v155 * v2212)) + (v156 * v2215)) + (v158 * v2218)) + (v174 * v2221)) + (v177 * v2224);
            let v2242 = ((((v2227 + (v155 * v2228)) + (v156 * v2231)) + (v158 * v2234)) + (v174 * v2237)) + (v177 * v2240);
            let v2258 = ((((v2243 + (v155 * v2244)) + (v156 * v2247)) + (v158 * v2250)) + (v174 * v2253)) + (v177 * v2256);
            let v2274 = ((((v2259 + (v155 * v2260)) + (v156 * v2263)) + (v158 * v2266)) + (v174 * v2269)) + (v177 * v2272);
            let v2290 = ((((v2275 + (v155 * v2276)) + (v156 * v2279)) + (v158 * v2282)) + (v174 * v2285)) + (v177 * v2288);
            let v2306 = ((((v2291 + (v155 * v2292)) + (v156 * v2295)) + (v158 * v2298)) + (v174 * v2301)) + (v177 * v2304);
            let v2322 = ((((v2307 + (v155 * v2308)) + (v156 * v2311)) + (v158 * v2314)) + (v174 * v2317)) + (v177 * v2320);
            let v2338 = ((((v2323 + (v155 * v2324)) + (v156 * v2327)) + (v158 * v2330)) + (v174 * v2333)) + (v177 * v2336);
            let v2354 = ((((v2339 + (v155 * v2340)) + (v156 * v2343)) + (v158 * v2346)) + (v174 * v2349)) + (v177 * v2352);
            let v2370 = ((((v2355 + (v155 * v2356)) + (v156 * v2359)) + (v158 * v2362)) + (v174 * v2365)) + (v177 * v2368);
            let v2386 = ((((v2371 + (v155 * v2372)) + (v156 * v2375)) + (v158 * v2378)) + (v174 * v2381)) + (v177 * v2384);
            let v2402 = ((((v2387 + (v155 * v2388)) + (v156 * v2391)) + (v158 * v2394)) + (v174 * v2397)) + (v177 * v2400);
            let v2418 = ((((v2403 + (v155 * v2404)) + (v156 * v2407)) + (v158 * v2410)) + (v174 * v2413)) + (v177 * v2416);
            let v2434 = ((((v2419 + (v155 * v2420)) + (v156 * v2423)) + (v158 * v2426)) + (v174 * v2429)) + (v177 * v2432);
            let v2450 = ((((v2435 + (v155 * v2436)) + (v156 * v2439)) + (v158 * v2442)) + (v174 * v2445)) + (v177 * v2448);
            let v2466 = ((((v2451 + (v155 * v2452)) + (v156 * v2455)) + (v158 * v2458)) + (v174 * v2461)) + (v177 * v2464);
            let v2482 = ((((v2467 + (v155 * v2468)) + (v156 * v2471)) + (v158 * v2474)) + (v174 * v2477)) + (v177 * v2480);
            let v2498 = ((((v2483 + (v155 * v2484)) + (v156 * v2487)) + (v158 * v2490)) + (v174 * v2493)) + (v177 * v2496);
            let v2514 = ((((v2499 + (v155 * v2500)) + (v156 * v2503)) + (v158 * v2506)) + (v174 * v2509)) + (v177 * v2512);
            let v2530 = ((((v2515 + (v155 * v2516)) + (v156 * v2519)) + (v158 * v2522)) + (v174 * v2525)) + (v177 * v2528);
            let v2546 = ((((v2531 + (v155 * v2532)) + (v156 * v2535)) + (v158 * v2538)) + (v174 * v2541)) + (v177 * v2544);
            let v2562 = ((((v2547 + (v155 * v2548)) + (v156 * v2551)) + (v158 * v2554)) + (v174 * v2557)) + (v177 * v2560);
            let v2578 = ((((v2563 + (v155 * v2564)) + (v156 * v2567)) + (v158 * v2570)) + (v174 * v2573)) + (v177 * v2576);
            let v2594 = ((((v2579 + (v155 * v2580)) + (v156 * v2583)) + (v158 * v2586)) + (v174 * v2589)) + (v177 * v2592);
            let v2610 = ((((v2595 + (v155 * v2596)) + (v156 * v2599)) + (v158 * v2602)) + (v174 * v2605)) + (v177 * v2608);
            let v2626 = ((((v2611 + (v155 * v2612)) + (v156 * v2615)) + (v158 * v2618)) + (v174 * v2621)) + (v177 * v2624);
            let v2642 = ((((v2627 + (v155 * v2628)) + (v156 * v2631)) + (v158 * v2634)) + (v174 * v2637)) + (v177 * v2640);
            let v2658 = ((((v2643 + (v155 * v2644)) + (v156 * v2647)) + (v158 * v2650)) + (v174 * v2653)) + (v177 * v2656);
            let v2674 = ((((v2659 + (v155 * v2660)) + (v156 * v2663)) + (v158 * v2666)) + (v174 * v2669)) + (v177 * v2672);
            let v2690 = ((((v2675 + (v155 * v2676)) + (v156 * v2679)) + (v158 * v2682)) + (v174 * v2685)) + (v177 * v2688);
            let v2706 = ((((v2691 + (v155 * v2692)) + (v156 * v2695)) + (v158 * v2698)) + (v174 * v2701)) + (v177 * v2704);
            let v2722 = ((((v2707 + (v155 * v2708)) + (v156 * v2711)) + (v158 * v2714)) + (v174 * v2717)) + (v177 * v2720);
            let v2738 = ((((v2723 + (v155 * v2724)) + (v156 * v2727)) + (v158 * v2730)) + (v174 * v2733)) + (v177 * v2736);
            let v2754 = ((((v2739 + (v155 * v2740)) + (v156 * v2743)) + (v158 * v2746)) + (v174 * v2749)) + (v177 * v2752);
            let v2770 = ((((v2755 + (v155 * v2756)) + (v156 * v2759)) + (v158 * v2762)) + (v174 * v2765)) + (v177 * v2768);
            let v2786 = ((((v2771 + (v155 * v2772)) + (v156 * v2775)) + (v158 * v2778)) + (v174 * v2781)) + (v177 * v2784);
            let v2802 = ((((v2787 + (v155 * v2788)) + (v156 * v2791)) + (v158 * v2794)) + (v174 * v2797)) + (v177 * v2800);
            let v2818 = ((((v2803 + (v155 * v2804)) + (v156 * v2807)) + (v158 * v2810)) + (v174 * v2813)) + (v177 * v2816);
            let v2834 = ((((v2819 + (v155 * v2820)) + (v156 * v2823)) + (v158 * v2826)) + (v174 * v2829)) + (v177 * v2832);
            let v2850 = ((((v2835 + (v155 * v2836)) + (v156 * v2839)) + (v158 * v2842)) + (v174 * v2845)) + (v177 * v2848);
            let v2866 = ((((v2851 + (v155 * v2852)) + (v156 * v2855)) + (v158 * v2858)) + (v174 * v2861)) + (v177 * v2864);
            let v2882 = ((((v2867 + (v155 * v2868)) + (v156 * v2871)) + (v158 * v2874)) + (v174 * v2877)) + (v177 * v2880);
            let v2898 = ((((v2883 + (v155 * v2884)) + (v156 * v2887)) + (v158 * v2890)) + (v174 * v2893)) + (v177 * v2896);
            let v2914 = ((((v2899 + (v155 * v2900)) + (v156 * v2903)) + (v158 * v2906)) + (v174 * v2909)) + (v177 * v2912);
            let v2930 = ((((v2915 + (v155 * v2916)) + (v156 * v2919)) + (v158 * v2922)) + (v174 * v2925)) + (v177 * v2928);
            let v2946 = ((((v2931 + (v155 * v2932)) + (v156 * v2935)) + (v158 * v2938)) + (v174 * v2941)) + (v177 * v2944);
            let v2962 = ((((v2947 + (v155 * v2948)) + (v156 * v2951)) + (v158 * v2954)) + (v174 * v2957)) + (v177 * v2960);
            let v2978 = ((((v2963 + (v155 * v2964)) + (v156 * v2967)) + (v158 * v2970)) + (v174 * v2973)) + (v177 * v2976);
            let v2994 = ((((v2979 + (v155 * v2980)) + (v156 * v2983)) + (v158 * v2986)) + (v174 * v2989)) + (v177 * v2992);
            let v3010 = ((((v2995 + (v155 * v2996)) + (v156 * v2999)) + (v158 * v3002)) + (v174 * v3005)) + (v177 * v3008);
            let v3026 = ((((v3011 + (v155 * v3012)) + (v156 * v3015)) + (v158 * v3018)) + (v174 * v3021)) + (v177 * v3024);
            let v3042 = ((((v3027 + (v155 * v3028)) + (v156 * v3031)) + (v158 * v3034)) + (v174 * v3037)) + (v177 * v3040);
            let v3058 = ((((v3043 + (v155 * v3044)) + (v156 * v3047)) + (v158 * v3050)) + (v174 * v3053)) + (v177 * v3056);
            let v4533: f64;
            let v4570: f64;
            let v8812: f64;
            let v8814: f64;
            let v9257: f64;
            let v9270: f64;
            let v9272: f64;
            let v9280: f64;
            let v9283: f64;
            let v13257: f64;
            let v13261: f64;
            let v13267: f64;
            let v13288: f64;
            let v13400: f64;
            let v13404: f64;
            let v13410: f64;
            let v13431: f64;
            if v97 != 0.0 {
                let v3074 = ((((v3059 + (v155 * v3060)) + (v156 * v3063)) + (v158 * v3066)) + (v174 * v3069)) + (v177 * v3072);
                let v3090 = ((((v3075 + (v155 * v3076)) + (v156 * v3079)) + (v158 * v3082)) + (v174 * v3085)) + (v177 * v3088);
                let v3106 = ((((v3091 + (v155 * v3092)) + (v156 * v3095)) + (v158 * v3098)) + (v174 * v3101)) + (v177 * v3104);
                let v3122 = ((((v3107 + (v155 * v3108)) + (v156 * v3111)) + (v158 * v3114)) + (v174 * v3117)) + (v177 * v3120);
                let v3138 = ((((v3123 + (v155 * v3124)) + (v156 * v3127)) + (v158 * v3130)) + (v174 * v3133)) + (v177 * v3136);
                let v3139 = if v96 == v23 { 1.0 } else { 0.0 };
                let v9271: f64;
                let v9273: f64;
                let v9281: f64;
                let v9284: f64;
                if v3139 != 0.0 {
                    let v3155 = ((((v3140 + (v155 * v3141)) + (v156 * v3144)) + (v158 * v3147)) + (v174 * v3150)) + (v177 * v3153);
                    let v3171 = ((((v3156 + (v155 * v3157)) + (v156 * v3160)) + (v158 * v3163)) + (v174 * v3166)) + (v177 * v3169);
                    let v3187 = ((((v3172 + (v155 * v3173)) + (v156 * v3176)) + (v158 * v3179)) + (v174 * v3182)) + (v177 * v3185);
                    let v3203 = ((((v3188 + (v155 * v3189)) + (v156 * v3192)) + (v158 * v3195)) + (v174 * v3198)) + (v177 * v3201);
                    v9271 = v3187;
                    v9273 = v3203;
                    v9281 = v3155;
                    v9284 = v3171;
                } else {
                    v9271 = v0;
                    v9273 = v0;
                    v9281 = v0;
                    v9284 = v0;
                }
                let v3212 = if (if (if v3204 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v3204 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if v100 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v100 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v102 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v13258: f64;
                let v13262: f64;
                let v13268: f64;
                let v13289: f64;
                let v13401: f64;
                let v13405: f64;
                let v13411: f64;
                let v13432: f64;
                if v3212 != 0.0 {
                    let v3228 = ((((v3213 + (v155 * v3214)) + (v156 * v3217)) + (v158 * v3220)) + (v174 * v3223)) + (v177 * v3226);
                    let v3244 = ((((v3229 + (v155 * v3230)) + (v156 * v3233)) + (v158 * v3236)) + (v174 * v3239)) + (v177 * v3242);
                    let v3260 = ((((v3245 + (v155 * v3246)) + (v156 * v3249)) + (v158 * v3252)) + (v174 * v3255)) + (v177 * v3258);
                    let v3276 = ((((v3261 + (v155 * v3262)) + (v156 * v3265)) + (v158 * v3268)) + (v174 * v3271)) + (v177 * v3274);
                    let v3292 = ((((v3277 + (v155 * v3278)) + (v156 * v3281)) + (v158 * v3284)) + (v174 * v3287)) + (v177 * v3290);
                    let v3308 = ((((v3293 + (v155 * v3294)) + (v156 * v3297)) + (v158 * v3300)) + (v174 * v3303)) + (v177 * v3306);
                    let v3324 = ((((v3309 + (v155 * v3310)) + (v156 * v3313)) + (v158 * v3316)) + (v174 * v3319)) + (v177 * v3322);
                    let v3340 = ((((v3325 + (v155 * v3326)) + (v156 * v3329)) + (v158 * v3332)) + (v174 * v3335)) + (v177 * v3338);
                    v13258 = v3244;
                    v13262 = v3228;
                    v13268 = v3276;
                    v13289 = v3260;
                    v13401 = v3308;
                    v13405 = v3292;
                    v13411 = v3340;
                    v13432 = v3324;
                } else {
                    v13258 = v0;
                    v13262 = v0;
                    v13268 = v0;
                    v13289 = v0;
                    v13401 = v0;
                    v13405 = v0;
                    v13411 = v0;
                    v13432 = v0;
                }
                v4533 = v3090;
                v4570 = v3074;
                v8812 = v3122;
                v8814 = v3138;
                v9257 = v3106;
                v9270 = v9271;
                v9272 = v9273;
                v9280 = v9281;
                v9283 = v9284;
                v13257 = v13258;
                v13261 = v13262;
                v13267 = v13268;
                v13288 = v13289;
                v13400 = v13401;
                v13404 = v13405;
                v13410 = v13411;
                v13431 = v13432;
            } else {
                v4533 = v0;
                v4570 = v0;
                v8812 = v0;
                v8814 = v0;
                v9257 = v0;
                v9270 = v0;
                v9272 = v0;
                v9280 = v0;
                v9283 = v0;
                v13257 = v0;
                v13261 = v0;
                v13267 = v0;
                v13288 = v0;
                v13400 = v0;
                v13404 = v0;
                v13410 = v0;
                v13431 = v0;
            }
            let v3342 = if v3341 != v0 { 1.0 } else { 0.0 };
            let v4237: f64;
            let v4277: f64;
            let v4304: f64;
            let v4368: f64;
            let v4375: f64;
            let v4382: f64;
            let v4392: f64;
            let v4427: f64;
            let v4442: f64;
            let v4456: f64;
            let v4565: f64;
            let v4597: f64;
            let v4599: f64;
            let v6887: f64;
            let v6893: f64;
            let v6936: f64;
            let v6955: f64;
            let v7010: f64;
            let v7012: f64;
            let v8876: f64;
            let v8879: f64;
            let v10460: f64;
            let v10468: f64;
            let v10472: f64;
            if v3342 != 0.0 {
                let v3358 = ((((v3343 + (v155 * v3344)) + (v156 * v3347)) + (v158 * v3350)) + (v174 * v3353)) + (v177 * v3356);
                let v3374 = ((((v3359 + (v155 * v3360)) + (v156 * v3363)) + (v158 * v3366)) + (v174 * v3369)) + (v177 * v3372);
                let v3390 = ((((v3375 + (v155 * v3376)) + (v156 * v3379)) + (v158 * v3382)) + (v174 * v3385)) + (v177 * v3388);
                let v3406 = ((((v3391 + (v155 * v3392)) + (v156 * v3395)) + (v158 * v3398)) + (v174 * v3401)) + (v177 * v3404);
                let v3422 = ((((v3407 + (v155 * v3408)) + (v156 * v3411)) + (v158 * v3414)) + (v174 * v3417)) + (v177 * v3420);
                let v3438 = ((((v3423 + (v155 * v3424)) + (v156 * v3427)) + (v158 * v3430)) + (v174 * v3433)) + (v177 * v3436);
                let v3454 = ((((v3439 + (v155 * v3440)) + (v156 * v3443)) + (v158 * v3446)) + (v174 * v3449)) + (v177 * v3452);
                let v3470 = ((((v3455 + (v155 * v3456)) + (v156 * v3459)) + (v158 * v3462)) + (v174 * v3465)) + (v177 * v3468);
                let v3486 = ((((v3471 + (v155 * v3472)) + (v156 * v3475)) + (v158 * v3478)) + (v174 * v3481)) + (v177 * v3484);
                let v3502 = ((((v3487 + (v155 * v3488)) + (v156 * v3491)) + (v158 * v3494)) + (v174 * v3497)) + (v177 * v3500);
                let v3518 = ((((v3503 + (v155 * v3504)) + (v156 * v3507)) + (v158 * v3510)) + (v174 * v3513)) + (v177 * v3516);
                let v3534 = ((((v3519 + (v155 * v3520)) + (v156 * v3523)) + (v158 * v3526)) + (v174 * v3529)) + (v177 * v3532);
                let v3550 = ((((v3535 + (v155 * v3536)) + (v156 * v3539)) + (v158 * v3542)) + (v174 * v3545)) + (v177 * v3548);
                let v3566 = ((((v3551 + (v155 * v3552)) + (v156 * v3555)) + (v158 * v3558)) + (v174 * v3561)) + (v177 * v3564);
                let v3582 = ((((v3567 + (v155 * v3568)) + (v156 * v3571)) + (v158 * v3574)) + (v174 * v3577)) + (v177 * v3580);
                let v3598 = ((((v3583 + (v155 * v3584)) + (v156 * v3587)) + (v158 * v3590)) + (v174 * v3593)) + (v177 * v3596);
                let v3614 = ((((v3599 + (v155 * v3600)) + (v156 * v3603)) + (v158 * v3606)) + (v174 * v3609)) + (v177 * v3612);
                let v3630 = ((((v3615 + (v155 * v3616)) + (v156 * v3619)) + (v158 * v3622)) + (v174 * v3625)) + (v177 * v3628);
                let v3646 = ((((v3631 + (v155 * v3632)) + (v156 * v3635)) + (v158 * v3638)) + (v174 * v3641)) + (v177 * v3644);
                let v3662 = ((((v3647 + (v155 * v3648)) + (v156 * v3651)) + (v158 * v3654)) + (v174 * v3657)) + (v177 * v3660);
                let v3678 = ((((v3663 + (v155 * v3664)) + (v156 * v3667)) + (v158 * v3670)) + (v174 * v3673)) + (v177 * v3676);
                let v3694 = ((((v3679 + (v155 * v3680)) + (v156 * v3683)) + (v158 * v3686)) + (v174 * v3689)) + (v177 * v3692);
                let v8877: f64;
                let v8880: f64;
                if v97 != 0.0 {
                    let v3710 = ((((v3695 + (v155 * v3696)) + (v156 * v3699)) + (v158 * v3702)) + (v174 * v3705)) + (v177 * v3708);
                    let v3726 = ((((v3711 + (v155 * v3712)) + (v156 * v3715)) + (v158 * v3718)) + (v174 * v3721)) + (v177 * v3724);
                    v8877 = v3710;
                    v8880 = v3726;
                } else {
                    v8877 = v0;
                    v8880 = v0;
                }
                v4237 = v3358;
                v4277 = v3406;
                v4304 = v3550;
                v4368 = v3566;
                v4375 = v3614;
                v4382 = v3598;
                v4392 = v3582;
                v4427 = v3486;
                v4442 = v3422;
                v4456 = v3438;
                v4565 = v3390;
                v4597 = v3454;
                v4599 = v3470;
                v6887 = v3630;
                v6893 = v3646;
                v6936 = v3662;
                v6955 = v3678;
                v7010 = v3518;
                v7012 = v3694;
                v8876 = v8877;
                v8879 = v8880;
                v10460 = v3534;
                v10468 = v3502;
                v10472 = v3374;
            } else {
                v4237 = v0;
                v4277 = v0;
                v4304 = v0;
                v4368 = v0;
                v4375 = v0;
                v4382 = v0;
                v4392 = v0;
                v4427 = v0;
                v4442 = v0;
                v4456 = v0;
                v4565 = v0;
                v4597 = v0;
                v4599 = v0;
                v6887 = v0;
                v6893 = v0;
                v6936 = v0;
                v6955 = v0;
                v7010 = v0;
                v7012 = v0;
                v8876 = v0;
                v8879 = v0;
                v10460 = v0;
                v10468 = v0;
                v10472 = v0;
            }
            let v3728 = if v3727 == v1 { 1.0 } else { 0.0 };
            let v7045: f64;
            let v7048: f64;
            let v7055: f64;
            let v7081: f64;
            let v7084: f64;
            let v7098: f64;
            let v7100: f64;
            let v7365: f64;
            let v7398: f64;
            let v7405: f64;
            let v8842: f64;
            let v8846: f64;
            let v8945: f64;
            if v3728 != 0.0 {
                let v3744 = ((((v3729 + (v155 * v3730)) + (v156 * v3733)) + (v158 * v3736)) + (v174 * v3739)) + (v177 * v3742);
                let v3746 = if v3745 != v0 { 1.0 } else { 0.0 };
                let v3965: f64;
                if v3746 != 0.0 {
                    let v3749 = v1 + (v32 / v3747);
                    let v3750 = if v3749 > v185 { 1.0 } else { 0.0 };
                    let v3754: f64;
                    if v3750 != 0.0 {
                        let v3751 = v3749.ln();
                        v3754 = v3751;
                    } else {
                        v3754 = v3752;
                    }
                    let v3757 = v3744 * (v1 + ((v3745 / v32) * v3754));
                    v3965 = v3757;
                } else {
                    v3965 = v3744;
                }
                let v3773 = ((((v3758 + (v155 * v3759)) + (v156 * v3762)) + (v158 * v3765)) + (v174 * v3768)) + (v177 * v3771);
                let v3789 = ((((v3774 + (v155 * v3775)) + (v156 * v3778)) + (v158 * v3781)) + (v174 * v3784)) + (v177 * v3787);
                let v3805 = ((((v3790 + (v155 * v3791)) + (v156 * v3794)) + (v158 * v3797)) + (v174 * v3800)) + (v177 * v3803);
                let v3821 = ((((v3806 + (v155 * v3807)) + (v156 * v3810)) + (v158 * v3813)) + (v174 * v3816)) + (v177 * v3819);
                let v3837 = ((((v3822 + (v155 * v3823)) + (v156 * v3826)) + (v158 * v3829)) + (v174 * v3832)) + (v177 * v3835);
                let v3853 = ((((v3838 + (v155 * v3839)) + (v156 * v3842)) + (v158 * v3845)) + (v174 * v3848)) + (v177 * v3851);
                let v3869 = ((((v3854 + (v155 * v3855)) + (v156 * v3858)) + (v158 * v3861)) + (v174 * v3864)) + (v177 * v3867);
                let v3885 = ((((v3870 + (v155 * v3871)) + (v156 * v3874)) + (v158 * v3877)) + (v174 * v3880)) + (v177 * v3883);
                let v3901 = ((((v3886 + (v155 * v3887)) + (v156 * v3890)) + (v158 * v3893)) + (v174 * v3896)) + (v177 * v3899);
                let v8843: f64;
                let v8847: f64;
                if v97 != 0.0 {
                    let v3917 = ((((v3902 + (v155 * v3903)) + (v156 * v3906)) + (v158 * v3909)) + (v174 * v3912)) + (v177 * v3915);
                    let v3933 = ((((v3918 + (v155 * v3919)) + (v156 * v3922)) + (v158 * v3925)) + (v174 * v3928)) + (v177 * v3931);
                    v8843 = v3917;
                    v8847 = v3933;
                } else {
                    v8843 = v8844;
                    v8847 = v8848;
                }
                let v3949 = ((((v3934 + (v155 * v3935)) + (v156 * v3938)) + (v158 * v3941)) + (v174 * v3944)) + (v177 * v3947);
                let v3951 = if v3950 != v0 { 1.0 } else { 0.0 };
                let v3973: f64;
                if v3951 != 0.0 {
                    let v3954 = v1 + (v32 / v3952);
                    let v3955 = if v3954 > v185 { 1.0 } else { 0.0 };
                    let v3959: f64;
                    if v3955 != 0.0 {
                        let v3956 = v3954.ln();
                        v3959 = v3956;
                    } else {
                        v3959 = v3957;
                    }
                    let v3962 = v3949 * (v1 + ((v3950 / v32) * v3959));
                    v3973 = v3962;
                } else {
                    v3973 = v3949;
                }
                let v3964 = if v3963 != v0 { 1.0 } else { 0.0 };
                let v7046: f64;
                let v8946: f64;
                if v3964 != 0.0 {
                    let v3966 = v32 - v3963;
                    let v3972 = v3965 * (v1 + ((v3966 * v3967) * v3969));
                    let v3978 = v3973 * (v1 + ((v3966 * v3974) * v3969));
                    v7046 = v3972;
                    v8946 = v3978;
                } else {
                    v7046 = v3965;
                    v8946 = v3973;
                }
                v7045 = v7046;
                v7048 = v3805;
                v7055 = v3837;
                v7081 = v3773;
                v7084 = v3853;
                v7098 = v3789;
                v7100 = v3885;
                v7365 = v3821;
                v7398 = v3869;
                v7405 = v3901;
                v8842 = v8843;
                v8846 = v8847;
                v8945 = v8946;
            } else {
                v7045 = v7047;
                v7048 = v7049;
                v7055 = v7056;
                v7081 = v7082;
                v7084 = v7085;
                v7098 = v7099;
                v7100 = v7101;
                v7365 = v7366;
                v7398 = v7399;
                v7405 = v7406;
                v8842 = v8844;
                v8846 = v8848;
                v8945 = v8947;
            }
            let v3980 = if v3979 != v0 { 1.0 } else { 0.0 };
            let v3983 = if v3980 != 0.0 && (if v3981 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v4634: f64;
            if v3983 != 0.0 {
                let v3998 = ((((v3981 + (v155 * v3984)) + (v156 * v3987)) + (v158 * v3990)) + (v174 * v3993)) + (v177 * v3996);
                v4634 = v3998;
            } else {
                v4634 = v0;
            }
            let v4000 = if v3999 == v1 { 1.0 } else { 0.0 };
            let v4663: f64;
            let v4667: f64;
            let v4672: f64;
            let v4676: f64;
            let v4679: f64;
            let v4682: f64;
            let v4684: f64;
            let v4687: f64;
            let v4689: f64;
            let v4692: f64;
            let v4695: f64;
            if v4000 != 0.0 {
                let v4016 = ((((v4001 + (v155 * v4002)) + (v156 * v4005)) + (v158 * v4008)) + (v174 * v4011)) + (v177 * v4014);
                let v4032 = ((((v4017 + (v155 * v4018)) + (v156 * v4021)) + (v158 * v4024)) + (v174 * v4027)) + (v177 * v4030);
                let v4048 = ((((v4033 + (v155 * v4034)) + (v156 * v4037)) + (v158 * v4040)) + (v174 * v4043)) + (v177 * v4046);
                let v4064 = ((((v4049 + (v155 * v4050)) + (v156 * v4053)) + (v158 * v4056)) + (v174 * v4059)) + (v177 * v4062);
                let v4080 = ((((v4065 + (v155 * v4066)) + (v156 * v4069)) + (v158 * v4072)) + (v174 * v4075)) + (v177 * v4078);
                let v4096 = ((((v4081 + (v155 * v4082)) + (v156 * v4085)) + (v158 * v4088)) + (v174 * v4091)) + (v177 * v4094);
                let v4112 = ((((v4097 + (v155 * v4098)) + (v156 * v4101)) + (v158 * v4104)) + (v174 * v4107)) + (v177 * v4110);
                let v4128 = ((((v4113 + (v155 * v4114)) + (v156 * v4117)) + (v158 * v4120)) + (v174 * v4123)) + (v177 * v4126);
                let v4144 = ((((v4129 + (v155 * v4130)) + (v156 * v4133)) + (v158 * v4136)) + (v174 * v4139)) + (v177 * v4142);
                let v4160 = ((((v4145 + (v155 * v4146)) + (v156 * v4149)) + (v158 * v4152)) + (v174 * v4155)) + (v177 * v4158);
                let v4176 = ((((v4161 + (v155 * v4162)) + (v156 * v4165)) + (v158 * v4168)) + (v174 * v4171)) + (v177 * v4174);
                v4663 = v4016;
                v4667 = v4032;
                v4672 = v4048;
                v4676 = v4064;
                v4679 = v4080;
                v4682 = v4096;
                v4684 = v4112;
                v4687 = v4128;
                v4689 = v4144;
                v4692 = v4160;
                v4695 = v4176;
            } else {
                v4663 = v0;
                v4667 = v0;
                v4672 = v0;
                v4676 = v0;
                v4679 = v0;
                v4682 = v0;
                v4684 = v0;
                v4687 = v0;
                v4689 = v0;
                v4692 = v0;
                v4695 = v0;
            }
            let v4178 = if v4177 != v0 { 1.0 } else { 0.0 };
            let v4311: f64;
            if v4178 != 0.0 {
                let v4181 = v1 + (v32 / v4179);
                let v4182 = if v4181 > v185 { 1.0 } else { 0.0 };
                let v4186: f64;
                if v4182 != 0.0 {
                    let v4183 = v4181.ln();
                    v4186 = v4183;
                } else {
                    v4186 = v4184;
                }
                let v4189 = v450 * (v1 + ((v4177 / v32) * v4186));
                v4311 = v4189;
            } else {
                v4311 = v450;
            }
            let v4191 = if v4190 != v0 { 1.0 } else { 0.0 };
            let v4318: f64;
            if v4191 != 0.0 {
                let v4194 = v1 + (v32 / v4192);
                let v4195 = if v4194 > v185 { 1.0 } else { 0.0 };
                let v4199: f64;
                if v4195 != 0.0 {
                    let v4196 = v4194.ln();
                    v4199 = v4196;
                } else {
                    v4199 = v4197;
                }
                let v4202 = v594 * (v1 + ((v4190 / v32) * v4199));
                v4318 = v4202;
            } else {
                v4318 = v594;
            }
            let v4204 = if v4203 != v0 { 1.0 } else { 0.0 };
            let v4554: f64;
            if v4204 != 0.0 {
                let v4207 = v1 + (v32 / v4205);
                let v4208 = if v4207 > v185 { 1.0 } else { 0.0 };
                let v4212: f64;
                if v4208 != 0.0 {
                    let v4209 = v4207.ln();
                    v4212 = v4209;
                } else {
                    v4212 = v4210;
                }
                let v4215 = v498 * (v1 + ((v4203 / v32) * v4212));
                v4554 = v4215;
            } else {
                v4554 = v498;
            }
            let v4217 = if v4216 != v0 { 1.0 } else { 0.0 };
            let v4556: f64;
            if v4217 != 0.0 {
                let v4220 = v1 + (v32 / v4218);
                let v4221 = if v4220 > v185 { 1.0 } else { 0.0 };
                let v4225: f64;
                if v4221 != 0.0 {
                    let v4222 = v4220.ln();
                    v4225 = v4222;
                } else {
                    v4225 = v4223;
                }
                let v4228 = v514 * (v1 + ((v4216 / v32) * v4225));
                v4556 = v4228;
            } else {
                v4556 = v514;
            }
            let v4230 = if v4229 != v0 { 1.0 } else { 0.0 };
            let v4558: f64;
            if v4230 != 0.0 {
                let v4233 = v1 + (v32 / v4231);
                let v4234 = if v4233 > v185 { 1.0 } else { 0.0 };
                let v4239: f64;
                if v4234 != 0.0 {
                    let v4235 = v4233.ln();
                    v4239 = v4235;
                } else {
                    v4239 = v4236;
                }
                let v4242 = v4237 * (v1 + ((v4229 / v32) * v4239));
                v4558 = v4242;
            } else {
                v4558 = v4237;
            }
            let v4244 = if v4243 != v0 { 1.0 } else { 0.0 };
            let v4458: f64;
            if v4244 != 0.0 {
                let v4247 = v1 + (v32 / v4245);
                let v4248 = if v4247 > v185 { 1.0 } else { 0.0 };
                let v4252: f64;
                if v4248 != 0.0 {
                    let v4249 = v4247.ln();
                    v4252 = v4249;
                } else {
                    v4252 = v4250;
                }
                let v4255 = v914 * (v1 + ((v4243 / v32) * v4252));
                v4458 = v4255;
            } else {
                v4458 = v914;
            }
            let v4257 = if v4256 != v0 { 1.0 } else { 0.0 };
            let v4465: f64;
            if v4257 != 0.0 {
                let v4260 = v1 + (v32 / v4258);
                let v4261 = if v4260 > v185 { 1.0 } else { 0.0 };
                let v4265: f64;
                if v4261 != 0.0 {
                    let v4262 = v4260.ln();
                    v4265 = v4262;
                } else {
                    v4265 = v4263;
                }
                let v4268 = v930 * (v1 + ((v4256 / v32) * v4265));
                v4465 = v4268;
            } else {
                v4465 = v930;
            }
            let v4270 = if v4269 != v0 { 1.0 } else { 0.0 };
            let v4472: f64;
            if v4270 != 0.0 {
                let v4273 = v1 + (v32 / v4271);
                let v4274 = if v4273 > v185 { 1.0 } else { 0.0 };
                let v4279: f64;
                if v4274 != 0.0 {
                    let v4275 = v4273.ln();
                    v4279 = v4275;
                } else {
                    v4279 = v4276;
                }
                let v4282 = v4277 * (v1 + ((v4269 / v32) * v4279));
                v4472 = v4282;
            } else {
                v4472 = v4277;
            }
            let v4284 = if v4283 != v0 { 1.0 } else { 0.0 };
            let v4324: f64;
            if v4284 != 0.0 {
                let v4287 = v1 + (v32 / v4285);
                let v4288 = if v4287 > v185 { 1.0 } else { 0.0 };
                let v4292: f64;
                if v4288 != 0.0 {
                    let v4289 = v4287.ln();
                    v4292 = v4289;
                } else {
                    v4292 = v4290;
                }
                let v4295 = v1090 * (v1 + ((v4283 / v32) * v4292));
                v4324 = v4295;
            } else {
                v4324 = v1090;
            }
            let v4297 = if v4296 != v0 { 1.0 } else { 0.0 };
            let v4391: f64;
            if v4297 != 0.0 {
                let v4300 = v1 + (v32 / v4298);
                let v4301 = if v4300 > v185 { 1.0 } else { 0.0 };
                let v4306: f64;
                if v4301 != 0.0 {
                    let v4302 = v4300.ln();
                    v4306 = v4302;
                } else {
                    v4306 = v4303;
                }
                let v4309 = v4304 * (v1 + ((v4296 / v32) * v4306));
                v4391 = v4309;
            } else {
                v4391 = v4304;
            }
            let v4310 = if v3963 != v0 { 1.0 } else { 0.0 };
            let v4331: f64;
            let v4340: f64;
            let v4563: f64;
            if v4310 != 0.0 {
                let v4312 = v32 - v3963;
                let v4317 = v4311 * (v1 + ((v4312 * v4313) * v3969));
                let v4323 = v4318 * (v1 + ((v4312 * v4319) * v3969));
                let v4329 = v4324 * (v1 + ((v4312 * v4325) * v3969));
                v4331 = v4317;
                v4340 = v4329;
                v4563 = v4323;
            } else {
                v4331 = v4311;
                v4340 = v4324;
                v4563 = v4318;
            }
            let v4330 = v3969.ln();
            let v4334 = v4331 + (v4332 * v3969);
            let v4337 = v898 + (v4335 * v3969);
            let v4339 = if v4338 > v0 { 1.0 } else { 0.0 };
            let v4579: f64;
            if v4339 != 0.0 {
                let v4346 = v4340 * (v1 - (v1122 * (((-v4338) * v4330).exp())));
                v4579 = v4346;
            } else {
                let v4348 = v4340 * (v1 - v1122);
                v4579 = v4348;
            }
            let v4350 = -v3969;
            let v4355 = v1138 + (v4349 * (rspice_limited_exp((v4350 / v4351))));
            let v4361 = v1170 + (v4356 * (rspice_limited_exp((v4350 / v4357))));
            let v4367 = v1154 + (v4362 * (rspice_limited_exp((v4350 / v4363))));
            let v4601: f64;
            let v4604: f64;
            let v4606: f64;
            let v4608: f64;
            if v3342 != 0.0 {
                let v4374 = v4368 + (v4369 * (rspice_limited_exp((v4350 / v4370))));
                let v4381 = v4375 + (v4376 * (rspice_limited_exp((v4350 / v4377))));
                let v4388 = v4382 + (v4383 * (rspice_limited_exp((v4350 / v4384))));
                let v4390 = if v4389 > v0 { 1.0 } else { 0.0 };
                let v4602: f64;
                if v4390 != 0.0 {
                    let v4398 = v4391 * (v1 - (v4392 * (((-v4389) * v4330).exp())));
                    v4602 = v4398;
                } else {
                    let v4400 = v4391 * (v1 - v4392);
                    v4602 = v4400;
                }
                v4601 = v4602;
                v4604 = v4374;
                v4606 = v4388;
                v4608 = v4381;
            } else {
                v4601 = v4391;
                v4604 = v4368;
                v4606 = v4382;
                v4608 = v4375;
            }
            let v4402 = if v4401 == v1 { 1.0 } else { 0.0 };
            let v4587: f64;
            let v4589: f64;
            let v4591: f64;
            if v4402 != 0.0 {
                let v4408 = v1394 + (v4403 * (rspice_limited_exp((v4350 / v4404))));
                let v4414 = v1410 + (v4409 * (rspice_limited_exp((v4350 / v4410))));
                v4587 = v1378;
                v4589 = v4408;
                v4591 = v4414;
            } else {
                let v4420 = v1378 + (v4415 * (rspice_limited_exp((v4350 / v4416))));
                v4587 = v4420;
                v4589 = v1394;
                v4591 = v1410;
            }
            let v4426 = v1266 + (v4421 * (rspice_limited_exp((v4350 / v4422))));
            let v10437: f64;
            if v3342 != 0.0 {
                let v4434 = v4427 + (v4428 * (((-v4429) * v4330).exp()));
                v10437 = v4434;
            } else {
                v10437 = v4427;
            }
            let v4441 = v1058 + (v4435 * (((-v4436) * v4330).exp()));
            let v4613: f64;
            if v3342 != 0.0 {
                let v4449 = v4442 + (v4443 * (((-v4444) * v4330).exp()));
                v4613 = v4449;
            } else {
                v4613 = v4442;
            }
            let v4454 = v4450 * (rspice_limited_exp((v4350 / v4451)));
            let v4455 = v1074 + v4454;
            let v9185: f64;
            if v3342 != 0.0 {
                let v4457 = v4456 + v4454;
                v9185 = v4457;
            } else {
                v9185 = v4456;
            }
            let v4464 = v4458 + (v4459 * (rspice_limited_exp((v4350 / v4460))));
            let v4470 = v4466 * (rspice_limited_exp((v4350 / v4467)));
            let v4471 = v4465 + v4470;
            let v4548: f64;
            if v3342 != 0.0 {
                let v4473 = v4472 + v4470;
                v4548 = v4473;
            } else {
                v4548 = v4472;
            }
            let v4479 = v994 + (v4474 * (rspice_limited_exp((v4350 / v4475))));
            let v4482 = -v4481;
            let v4487 = v1026 + (v4480 * (rspice_limited_exp((v4482 / v4483))));
            let v4493 = v946 + (v4488 * (rspice_limited_exp((v4482 / v4489))));
            let v4499 = v2994 + (v4494 * (rspice_limited_exp((v4350 / v4495))));
            let v4505 = v3010 + (v4500 * (rspice_limited_exp((v4350 / v4501))));
            let v4506 = if v866 > v0 { 1.0 } else { 0.0 };
            let v4508 = if v4506 != 0.0 || (if v882 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v12138: f64;
            if v4508 != 0.0 {
                let v4511 = (v23 * v399) / v388;
                let v4518 = v4511 * (v1 + (v4509 * (rspice_limited_exp(((-v4511) / v4513)))));
                v12138 = v4518;
            } else {
                v12138 = v0;
            }
            let v4519 = if v423 <= v90 { 1.0 } else { 0.0 };
            if v4519 != 0.0 {
            } else {
            }
            let v4520 = if v425 <= v90 { 1.0 } else { 0.0 };
            if v4520 != 0.0 {
            } else {
            }
            let v4521 = if v3042 <= v0 { 1.0 } else { 0.0 };
            let v4537: f64;
            if v4521 != 0.0 {
                v4537 = v4522;
            } else {
                v4537 = v3042;
            }
            let v4523 = if v466 < v0 { 1.0 } else { 0.0 };
            let v14526: f64;
            if v4523 != 0.0 {
                v14526 = v0;
            } else {
                let v4527 = if (if v466 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v466 <= v4525 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14527: f64;
                if v4527 != 0.0 {
                    v14527 = v466;
                } else {
                    let v4529 = if v466 > v4528 { 1.0 } else { 0.0 };
                    let v14528: f64;
                    if v4529 != 0.0 {
                        v14528 = v0;
                    } else {
                        v14528 = v466;
                    }
                    v14527 = v14528;
                }
                v14526 = v14527;
            }
            let v4530 = if v530 < v0 { 1.0 } else { 0.0 };
            if v4530 != 0.0 {
            } else {
            }
            let v4531 = if v4334 <= v0 { 1.0 } else { 0.0 };
            let v9957: f64;
            if v4531 != 0.0 {
                v9957 = v4532;
            } else {
                v9957 = v4334;
            }
            let v9254: f64;
            if v97 != 0.0 {
                let v4534 = if v4533 < v25 { 1.0 } else { 0.0 };
                let v9255: f64;
                if v4534 != 0.0 {
                    v9255 = v25;
                } else {
                    v9255 = v4533;
                }
                v9254 = v9255;
            } else {
                v9254 = v4533;
            }
            let v4535 = if v2882 < v0 { 1.0 } else { 0.0 };
            let v7430: f64;
            if v4535 != 0.0 {
                v7430 = v4536;
            } else {
                v7430 = v2882;
            }
            let v4538 = if v4537 < v0 { 1.0 } else { 0.0 };
            let v13757: f64;
            if v4538 != 0.0 {
                v13757 = v4522;
            } else {
                v13757 = v4537;
            }
            let v4539 = if v3026 < v0 { 1.0 } else { 0.0 };
            let v13762: f64;
            if v4539 != 0.0 {
                v13762 = v4540;
            } else {
                v13762 = v3026;
            }
            let v4541 = if v3058 < v0 { 1.0 } else { 0.0 };
            let v13759: f64;
            if v4541 != 0.0 {
                v13759 = v4542;
            } else {
                v13759 = v3058;
            }
            let v4543 = if v2466 < v0 { 1.0 } else { 0.0 };
            if v4543 != 0.0 {
            } else {
            }
            let v4544 = if v2482 < v0 { 1.0 } else { 0.0 };
            if v4544 != 0.0 {
            } else {
            }
            let v4545 = if v4464 <= v0 { 1.0 } else { 0.0 };
            let v6979: f64;
            if v4545 != 0.0 {
                v6979 = v4546;
            } else {
                v6979 = v4464;
            }
            let v4547 = if v4471 <= v0 { 1.0 } else { 0.0 };
            let v7105: f64;
            if v4547 != 0.0 {
                v7105 = v4546;
            } else {
                v7105 = v4471;
            }
            let v4550 = if v3342 != 0.0 && (if v4548 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v7136: f64;
            if v4550 != 0.0 {
                v7136 = v4546;
            } else {
                v7136 = v4548;
            }
            let v4551 = if v546 <= v0 { 1.0 } else { 0.0 };
            let v5593: f64;
            if v4551 != 0.0 {
                v5593 = v4552;
            } else {
                v5593 = v546;
            }
            let v4553 = if v562 <= v0 { 1.0 } else { 0.0 };
            let v5607: f64;
            if v4553 != 0.0 {
                v5607 = v4552;
            } else {
                v5607 = v562;
            }
            let v4555 = if v4554 < v0 { 1.0 } else { 0.0 };
            if v4555 != 0.0 {
            } else {
            }
            let v4557 = if v4556 < v0 { 1.0 } else { 0.0 };
            if v4557 != 0.0 {
            } else {
            }
            let v4560 = if v3342 != 0.0 && (if v4558 < v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v4560 != 0.0 {
            } else {
            }
            let v4561 = if v626 <= v0 { 1.0 } else { 0.0 };
            let v5620: f64;
            if v4561 != 0.0 {
                v5620 = v4562;
            } else {
                v5620 = v626;
            }
            let v4564 = if v4563 < v0 { 1.0 } else { 0.0 };
            let v8915: f64;
            if v4564 != 0.0 {
                v8915 = v0;
            } else {
                v8915 = v4563;
            }
            let v4566 = if v4565 < v0 { 1.0 } else { 0.0 };
            let v8977: f64;
            if v4566 != 0.0 {
                v8977 = v0;
            } else {
                v8977 = v4565;
            }
            let v4567 = if v658 < v4350 { 1.0 } else { 0.0 };
            let v5631: f64;
            if v4567 != 0.0 {
                v5631 = v0;
            } else {
                v5631 = v658;
            }
            let v4568 = if v722 < v0 { 1.0 } else { 0.0 };
            let v9226: f64;
            if v4568 != 0.0 {
                v9226 = v0;
            } else {
                v9226 = v722;
            }
            let v4569 = if v754 < v0 { 1.0 } else { 0.0 };
            let v9240: f64;
            if v4569 != 0.0 {
                v9240 = v0;
            } else {
                v9240 = v754;
            }
            let v4573 = if v97 != 0.0 && (if v4570 < v4571 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v4574: f64;
            if v4573 != 0.0 {
                v4574 = v4571;
            } else {
                v4574 = v4570;
            }
            let v4576 = if v97 != 0.0 && (if v4574 > v4542 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v10362: f64;
            if v4576 != 0.0 {
                v10362 = v4542;
            } else {
                v10362 = v4574;
            }
            let v4577 = if v4479 < v23 { 1.0 } else { 0.0 };
            let v12338: f64;
            if v4577 != 0.0 {
                v12338 = v23;
            } else {
                v12338 = v4479;
            }
            let v4578 = if v4487 < v23 { 1.0 } else { 0.0 };
            let v12443: f64;
            if v4578 != 0.0 {
                v12443 = v23;
            } else {
                v12443 = v4487;
            }
            let v4580 = if v4579 < v0 { 1.0 } else { 0.0 };
            let v6565: f64;
            if v4580 != 0.0 {
                v6565 = v4581;
            } else {
                v6565 = v4579;
            }
            let v4582 = if v4355 < v0 { 1.0 } else { 0.0 };
            let v6470: f64;
            if v4582 != 0.0 {
                v6470 = v0;
            } else {
                v6470 = v4355;
            }
            let v4583 = if v4367 < v0 { 1.0 } else { 0.0 };
            let v6499: f64;
            if v4583 != 0.0 {
                v6499 = v0;
            } else {
                v6499 = v4367;
            }
            let v4584 = if v4361 < v0 { 1.0 } else { 0.0 };
            let v6573: f64;
            if v4584 != 0.0 {
                v6573 = v0;
            } else {
                v6573 = v4361;
            }
            let v4585 = if v1186 < v0 { 1.0 } else { 0.0 };
            let v6959: f64;
            if v4585 != 0.0 {
                v6959 = v0;
            } else {
                v6959 = v1186;
            }
            let v4586 = if v1106 < v0 { 1.0 } else { 0.0 };
            let v6458: f64;
            if v4586 != 0.0 {
                v6458 = v0;
            } else {
                v6458 = v1106;
            }
            let v4588 = if v4587 < v0 { 1.0 } else { 0.0 };
            let v10981: f64;
            if v4588 != 0.0 {
                v10981 = v0;
            } else {
                v10981 = v4587;
            }
            let v4590 = if v4589 < v0 { 1.0 } else { 0.0 };
            let v12516: f64;
            if v4590 != 0.0 {
                v12516 = v0;
            } else {
                v12516 = v4589;
            }
            let v4592 = if v4591 < v0 { 1.0 } else { 0.0 };
            let v12552: f64;
            if v4592 != 0.0 {
                v12552 = v0;
            } else {
                v12552 = v4591;
            }
            let v4593 = if v1426 < v0 { 1.0 } else { 0.0 };
            let v12543: f64;
            if v4593 != 0.0 {
                v12543 = v0;
            } else {
                v12543 = v1426;
            }
            let v4594 = if v1442 < v0 { 1.0 } else { 0.0 };
            let v10971: f64;
            if v4594 != 0.0 {
                v10971 = v0;
            } else {
                v10971 = v1442;
            }
            let v4595 = if v4426 < v0 { 1.0 } else { 0.0 };
            if v4595 != 0.0 {
            } else {
            }
            let v4596 = if v1474 < v0 { 1.0 } else { 0.0 };
            if v4596 != 0.0 {
            } else {
            }
            let v6885: f64;
            let v6933: f64;
            let v6953: f64;
            let v10517: f64;
            if v3342 != 0.0 {
                let v4598 = if v4597 < v0 { 1.0 } else { 0.0 };
                if v4598 != 0.0 {
                } else {
                }
                let v4600 = if v4599 < v0 { 1.0 } else { 0.0 };
                if v4600 != 0.0 {
                } else {
                }
                let v4603 = if v4601 < v0 { 1.0 } else { 0.0 };
                let v6886: f64;
                if v4603 != 0.0 {
                    v6886 = v0;
                } else {
                    v6886 = v4601;
                }
                let v4605 = if v4604 < v0 { 1.0 } else { 0.0 };
                let v6934: f64;
                if v4605 != 0.0 {
                    v6934 = v0;
                } else {
                    v6934 = v4604;
                }
                let v4607 = if v4606 < v0 { 1.0 } else { 0.0 };
                let v10518: f64;
                if v4607 != 0.0 {
                    v10518 = v0;
                } else {
                    v10518 = v4606;
                }
                let v4609 = if v4608 < v0 { 1.0 } else { 0.0 };
                let v6954: f64;
                if v4609 != 0.0 {
                    v6954 = v0;
                } else {
                    v6954 = v4608;
                }
                v6885 = v6886;
                v6933 = v6934;
                v6953 = v6954;
                v10517 = v10518;
            } else {
                v6885 = v4601;
                v6933 = v4604;
                v6953 = v4608;
                v10517 = v4606;
            }
            let v4610 = if v1490 < v0 { 1.0 } else { 0.0 };
            if v4610 != 0.0 {
            } else {
            }
            let v4611 = if v1506 <= v0 { 1.0 } else { 0.0 };
            let v12268: f64;
            if v4611 != 0.0 {
                v12268 = v4562;
            } else {
                v12268 = v1506;
            }
            let v4612 = if v4441 < v23 { 1.0 } else { 0.0 };
            let v7199: f64;
            if v4612 != 0.0 {
                v7199 = v23;
            } else {
                v7199 = v4441;
            }
            let v7217: f64;
            if v3342 != 0.0 {
                let v4614 = if v4613 < v23 { 1.0 } else { 0.0 };
                let v7218: f64;
                if v4614 != 0.0 {
                    v7218 = v23;
                } else {
                    v7218 = v4613;
                }
                v7217 = v7218;
            } else {
                v7217 = v4613;
            }
            let v4615 = if v4455 < v0 { 1.0 } else { 0.0 };
            let v9154: f64;
            if v4615 != 0.0 {
                v9154 = v0;
            } else {
                v9154 = v4455;
            }
            let v4616 = if v1986 < v0 { 1.0 } else { 0.0 };
            let v13168: f64;
            if v4616 != 0.0 {
                v13168 = v0;
            } else {
                v13168 = v1986;
            }
            let v4617 = if v2114 < v0 { 1.0 } else { 0.0 };
            let v13320: f64;
            if v4617 != 0.0 {
                v13320 = v0;
            } else {
                v13320 = v2114;
            }
            let v4619 = if v4618 != v0 { 1.0 } else { 0.0 };
            let v12897: f64;
            let v12947: f64;
            if v4619 != 0.0 {
                let v4620 = if v1618 <= v0 { 1.0 } else { 0.0 };
                let v12898: f64;
                if v4620 != 0.0 {
                    v12898 = v276;
                } else {
                    v12898 = v1618;
                }
                let v4621 = if v1698 <= v0 { 1.0 } else { 0.0 };
                let v12948: f64;
                if v4621 != 0.0 {
                    v12948 = v1;
                } else {
                    v12948 = v1698;
                }
                v12897 = v12898;
                v12947 = v12948;
            } else {
                v12897 = v1618;
                v12947 = v1698;
            }
            let v4623 = if v4622 != v0 { 1.0 } else { 0.0 };
            let v5661: f64;
            let v13047: f64;
            if v4623 != 0.0 {
                let v4624 = if v1938 <= v0 { 1.0 } else { 0.0 };
                let v5662: f64;
                if v4624 != 0.0 {
                    v5662 = v1;
                } else {
                    v5662 = v1938;
                }
                let v4625 = if v1778 <= v0 { 1.0 } else { 0.0 };
                let v13048: f64;
                if v4625 != 0.0 {
                    v13048 = v1;
                } else {
                    v13048 = v1778;
                }
                v5661 = v5662;
                v13047 = v13048;
            } else {
                v5661 = v1938;
                v13047 = v1778;
            }
            let v4626 = if v4623 != 0.0 || v4619 != 0.0 { 1.0 } else { 0.0 };
            if v4626 != 0.0 {
                let v4628 = if v4627 <= v0 { 1.0 } else { 0.0 };
                if v4628 != 0.0 {
                } else {
                }
            } else {
            }
            let v4631 = if v4629 >= (v3969 / v23) { 1.0 } else { 0.0 };
            if v4631 != 0.0 {
            } else {
            }
            let v4632 = if v2594 <= v0 { 1.0 } else { 0.0 };
            if v4632 != 0.0 {
            } else {
            }
            let v4633 = if v3979 == v1 { 1.0 } else { 0.0 };
            let v4636 = if v4633 != 0.0 && (if v4634 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v13705: f64;
            if v4636 != 0.0 {
                let v4638 = if v4634 < v4637 { 1.0 } else { 0.0 };
                let v13706: f64;
                if v4638 != 0.0 {
                    v13706 = v0;
                } else {
                    v13706 = v4634;
                }
                v13705 = v13706;
            } else {
                v13705 = v4634;
            }
            let v4640 = if v4639 == v23 { 1.0 } else { 0.0 };
            if v4640 != 0.0 {
                let v4641 = if v2290 < v0 { 1.0 } else { 0.0 };
                if v4641 != 0.0 {
                } else {
                }
                let v4642 = if v2306 < v0 { 1.0 } else { 0.0 };
                if v4642 != 0.0 {
                } else {
                }
                let v4643 = if v2322 < v0 { 1.0 } else { 0.0 };
                if v4643 != 0.0 {
                } else {
                }
                let v4644 = if v2338 < v0 { 1.0 } else { 0.0 };
                if v4644 != 0.0 {
                } else {
                }
                let v4645 = if v2354 < v0 { 1.0 } else { 0.0 };
                if v4645 != 0.0 {
                } else {
                }
                let v4646 = if v2370 < v0 { 1.0 } else { 0.0 };
                if v4646 != 0.0 {
                } else {
                }
                let v4647 = if v2386 < v0 { 1.0 } else { 0.0 };
                if v4647 != 0.0 {
                } else {
                }
                let v4648 = if v2402 < v0 { 1.0 } else { 0.0 };
                if v4648 != 0.0 {
                } else {
                }
            } else {
            }
            let v4649 = if v434 <= v0 { 1.0 } else { 0.0 };
            let v14430: f64;
            if v4649 != 0.0 {
                v14430 = v1;
            } else {
                let v4650 = if v434 > v23 { 1.0 } else { 0.0 };
                let v14431: f64;
                if v4650 != 0.0 {
                    v14431 = v1;
                } else {
                    v14431 = v434;
                }
                v14430 = v14431;
            }
            let v4651 = if v2498 < v0 { 1.0 } else { 0.0 };
            if v4651 != 0.0 {
            } else {
            }
            let v4652 = if v2514 < v0 { 1.0 } else { 0.0 };
            if v4652 != 0.0 {
            } else {
            }
            let v4653 = if v2434 < v0 { 1.0 } else { 0.0 };
            if v4653 != 0.0 {
            } else {
            }
            let v4654 = if v2450 < v0 { 1.0 } else { 0.0 };
            if v4654 != 0.0 {
            } else {
            }
            let v4655 = if v2530 < v0 { 1.0 } else { 0.0 };
            if v4655 != 0.0 {
            } else {
            }
            let v4657 = if v2546 <= v4656 { 1.0 } else { 0.0 };
            if v4657 != 0.0 {
            } else {
            }
            let v4658 = if v2562 <= v4656 { 1.0 } else { 0.0 };
            if v4658 != 0.0 {
            } else {
            }
            let v4659 = if v2578 <= v4656 { 1.0 } else { 0.0 };
            if v4659 != 0.0 {
            } else {
            }
            let v4662 = if v433 < (-v4660) { 1.0 } else { 0.0 };
            let v4743: f64;
            if v4662 != 0.0 {
                v4743 = v0;
            } else {
                v4743 = v433;
            }
            let v5681: f64;
            let v5696: f64;
            let v5702: f64;
            let v5717: f64;
            let v5730: f64;
            let v5742: f64;
            let v6219: f64;
            let v6233: f64;
            let v6271: f64;
            let v6301: f64;
            let v6331: f64;
            if v4000 != 0.0 {
                let v4666 = if (if v4663 < v1 { 1.0 } else { 0.0 }) != 0.0 || (if v4663 > v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5682: f64;
                if v4666 != 0.0 {
                    v5682 = v23;
                } else {
                    v5682 = v4663;
                }
                let v4670 = if (if v4667 < v1 { 1.0 } else { 0.0 }) != 0.0 || (if v4667 > v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5697: f64;
                if v4670 != 0.0 {
                    v5697 = v4671;
                } else {
                    v5697 = v4667;
                }
                let v4675 = if (if v4672 < v1 { 1.0 } else { 0.0 }) != 0.0 || (if v4672 > v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5703: f64;
                if v4675 != 0.0 {
                    v5703 = v4671;
                } else {
                    v5703 = v4672;
                }
                let v4677 = if v4676 < v0 { 1.0 } else { 0.0 };
                let v5718: f64;
                if v4677 != 0.0 {
                    v5718 = v4678;
                } else {
                    v5718 = v4676;
                }
                let v4680 = if v4679 < v0 { 1.0 } else { 0.0 };
                let v5731: f64;
                if v4680 != 0.0 {
                    v5731 = v4681;
                } else {
                    v5731 = v4679;
                }
                let v4683 = if v4682 < v0 { 1.0 } else { 0.0 };
                let v5743: f64;
                if v4683 != 0.0 {
                    v5743 = v4681;
                } else {
                    v5743 = v4682;
                }
                let v4685 = if v4684 < v0 { 1.0 } else { 0.0 };
                let v6220: f64;
                if v4685 != 0.0 {
                    v6220 = v4686;
                } else {
                    v6220 = v4684;
                }
                let v4688 = if v4687 < v0 { 1.0 } else { 0.0 };
                let v6234: f64;
                if v4688 != 0.0 {
                    v6234 = v23;
                } else {
                    v6234 = v4687;
                }
                let v4690 = if v4689 < v0 { 1.0 } else { 0.0 };
                let v6272: f64;
                if v4690 != 0.0 {
                    v6272 = v4691;
                } else {
                    v6272 = v4689;
                }
                let v4693 = if v4692 < v0 { 1.0 } else { 0.0 };
                let v6302: f64;
                if v4693 != 0.0 {
                    v6302 = v4694;
                } else {
                    v6302 = v4692;
                }
                let v4696 = if v4695 < v0 { 1.0 } else { 0.0 };
                let v6332: f64;
                if v4696 != 0.0 {
                    v6332 = v4697;
                } else {
                    v6332 = v4695;
                }
                v5681 = v5682;
                v5696 = v5697;
                v5702 = v5703;
                v5717 = v5718;
                v5730 = v5731;
                v5742 = v5743;
                v6219 = v6220;
                v6233 = v6234;
                v6271 = v6272;
                v6301 = v6302;
                v6331 = v6332;
            } else {
                v5681 = v4663;
                v5696 = v4667;
                v5702 = v4672;
                v5717 = v4676;
                v5730 = v4679;
                v5742 = v4682;
                v6219 = v4684;
                v6233 = v4687;
                v6271 = v4689;
                v6301 = v4692;
                v6331 = v4695;
            }
            let v4700 = if (if v3 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v5 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v4700 != 0.0 {
                let v4702 = if v4701 != v0 { 1.0 } else { 0.0 };
                if v4702 != 0.0 {
                } else {
                }
                let v4704 = if v4703 != v0 { 1.0 } else { 0.0 };
                if v4704 != 0.0 {
                } else {
                }
                if v102 != 0.0 {
                    let v4706 = if v4705 != v0 { 1.0 } else { 0.0 };
                    if v4706 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v4708 = if v4707 != v0 { 1.0 } else { 0.0 };
            let v14422: f64;
            if v4708 != 0.0 {
                let v4711 = v4709 / v4710;
                let v4713 = v4712 * v32;
                let v4714 = if v4710 == v23 { 1.0 } else { 0.0 };
                let v4716: f64;
                if v4714 != 0.0 {
                    v4716 = v4715;
                } else {
                    v4716 = v276;
                }
                let v4721 = v1 / (if v4637 >= ((v4711 + (v4713 / v4716)) / v152) { v4637 } else { ((v4711 + (v4713 / v4716)) / v152) });
                let v4722 = if v4707 == v23 { 1.0 } else { 0.0 };
                if v4722 != 0.0 {
                } else {
                }
                v14422 = v4721;
            } else {
                v14422 = v0;
            }
            let v4724 = if v4723 == v0 { 1.0 } else { 0.0 };
            let v4875: f64;
            let v4878: f64;
            if v4724 != 0.0 {
                let v4727 = v4725 * v4726;
                let v4730 = v4728 * v4729;
                v4875 = v4727;
                v4878 = v4730;
            } else {
                let v4732 = if v4731 > v0 { 1.0 } else { 0.0 };
                let v4790: f64;
                if v4732 != 0.0 {
                    let v4739 = (v4660 * v205) + ((v211 + ((v4660 - v211) * v4735)) * v4731);
                    v4790 = v4739;
                } else {
                    let v4742 = v4660 * (if v90 >= (v205 + v4731) { v90 } else { (v205 + v4731) });
                    v4790 = v4742;
                }
                let v4744 = v4660 + v4743;
                let v4797: f64;
                if v4745 != 0.0 {
                    v4797 = v4746;
                } else {
                    let v4749: f64;
                    if v12 != 0.0 {
                        v4749 = v4747;
                    } else {
                        v4749 = v4748;
                    }
                    let v4787: f64;
                    if v12 != 0.0 {
                        let v4756 = v4755 / v4750;
                        let v4768 = ((v4757 + ((v4749 - v4757) / (v1 + ((v4750 / v4751).powf(v4753))))) - (v4762 / (v1 + (v4756 * v4756)))) * v4767;
                        v4787 = v4768;
                    } else {
                        let v4774 = v4773 / v4750;
                        let v4785 = ((v4775 + ((v4749 - v4775) / (v1 + ((v4750 / v4769).powf(v4771))))) - (v4780 / (v1 + (v4774 * v4774)))) * v4767;
                        v4787 = v4785;
                    }
                    let v4789 = v1 / ((v415 * v4750) * v4787);
                    v4797 = v4789;
                }
                let v4796 = if v4790 <= (if v4791 >= (v211 * (v205 + (if v0 <= v4731 { v0 } else { v4731 }))) { v4791 } else { (v211 * (v205 + (if v0 <= v4731 { v0 } else { v4731 }))) }) { v4790 } else { (if v4791 >= (v211 * (v205 + (if v0 <= v4731 { v0 } else { v4731 }))) { v4791 } else { (v211 * (v205 + (if v0 <= v4731 { v0 } else { v4731 }))) }) };
                let v4812 = ((v4797 / v4798) / (v4800 * v32)) * (((v1 / (v4796.sqrt())) - (v23 / (v4790.sqrt()))) + ((v4796 / (v4790 * v4790)).sqrt()));
                let v4815 = (v4790 * v32) + v4814;
                let v4823 = ((v4819 * v4815) / (v4797 * ((v4744 * v32) + v4817))).sqrt();
                let v4827 = rspice_limited_exp((v23 * (v4824 / v4823)));
                let v4829 = if v4828 == v1 { 1.0 } else { 0.0 };
                let v4841: f64;
                let v4843: f64;
                if v4829 != 0.0 {
                    let v4831 = (v4797 * v4823) / v4819;
                    let v4833 = v4827 * (v1 + v4831);
                    let v4835 = (v4833 + v1) - v4831;
                    let v4837 = (v4833 - v1) + v4831;
                    v4841 = v4835;
                    v4843 = v4837;
                } else {
                    let v4838 = v4827 + v1;
                    let v4839 = v4827 - v1;
                    v4841 = v4838;
                    v4843 = v4839;
                }
                let v4845 = ((v4797 * v4823) * v4841) / (v4815 * v4843);
                let v4847 = if v4731 < v4846 { 1.0 } else { 0.0 };
                let v4857: f64;
                if v4847 != 0.0 {
                    let v4851 = v4819 / (((-v4731) * v211) * v32);
                    let v4852 = v4845 + v4812;
                    let v4855 = (v4852 * v4851) / (v4852 + v4851);
                    v4857 = v4855;
                } else {
                    let v4856 = v4845 + v4812;
                    v4857 = v4856;
                }
                let v4873 = (v4857 / v152) * (if v0 >= ((((v4859 + (v4860 * v211)) + (v4863 * v4660)) + (v4866 * v4824)) + (v4869 * v4731)) { v0 } else { ((((v4859 + (v4860 * v211)) + (v4863 * v4660)) + (v4866 * v4824)) + (v4869 * v4731)) });
                v4875 = v4873;
                v4878 = v4873;
            }
            let v4874 = if v4401 == v0 { 1.0 } else { 0.0 };
            let v10997: f64;
            let v11000: f64;
            if v4874 != 0.0 {
                let v4877 = if v4875 < v4876 { 1.0 } else { 0.0 };
                let v10998: f64;
                if v4877 != 0.0 {
                    v10998 = v0;
                } else {
                    v10998 = v4875;
                }
                let v4879 = if v4878 < v4876 { 1.0 } else { 0.0 };
                let v11001: f64;
                if v4879 != 0.0 {
                    v11001 = v0;
                } else {
                    v11001 = v4878;
                }
                v10997 = v10998;
                v11000 = v11001;
            } else {
                let v4880 = if v4875 <= v4876 { 1.0 } else { 0.0 };
                let v10999: f64;
                if v4880 != 0.0 {
                    v10999 = v4876;
                } else {
                    v10999 = v4875;
                }
                let v4881 = if v4878 <= v4876 { 1.0 } else { 0.0 };
                let v11002: f64;
                if v4881 != 0.0 {
                    v11002 = v4876;
                } else {
                    v11002 = v4878;
                }
                v10997 = v10999;
                v11000 = v11002;
            }
            let v4883 = if v4882 != v1 { 1.0 } else { 0.0 };
            if v4883 != 0.0 {
                if v4884 != 0.0 {
                } else {
                    let v4887 = if v4885 != 0.0 && (if v77 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v4887 != 0.0 {
                    } else {
                        let v4888 = if v4882 == v276 { 1.0 } else { 0.0 };
                        if v4888 != 0.0 {
                        } else {
                        }
                    }
                }
                if v4889 != 0.0 {
                } else {
                    let v4891 = if v4885 != 0.0 && (if v77 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v4891 != 0.0 {
                    } else {
                        let v4892 = if v4882 == v276 { 1.0 } else { 0.0 };
                        if v4892 != 0.0 {
                        } else {
                        }
                    }
                }
            } else {
            }
            let v4893 = if v4882 == v23 { 1.0 } else { 0.0 };
            if v4893 != 0.0 {
                let v4896 = v4894 + v4895;
                let v4899 = v4897 * (v4660 - v211);
                let v4902 = if v0 >= (v4899 - v4900) { v0 } else { (v4899 - v4900) };
                let v4905 = if v0 >= (v4731 + v4903) { v0 } else { (v4731 + v4903) };
                let v4906 = if v4895 > v0 { 1.0 } else { 0.0 };
                if v4906 != 0.0 {
                    let v4913 = if ((v4907 * v17) / (v4909 * v4910)) > v185 { 1.0 } else { 0.0 };
                    if v4913 != 0.0 {
                    } else {
                    }
                } else {
                    let v4915 = v4896 + v4900;
                    let v4918 = v4914 + ((v4571 * v4915) / v4905);
                    let v4921 = (v4915 - v4905).abs();
                    let v4922 = v4910 * v4919;
                    let v4930 = v4926 * ((v18 * ((if v4905 <= v4915 { v4905 } else { v4915 }) - (v4910 / (v4918 + v1)))) / v4910);
                    let v4932 = if v4930 > v4931 { 1.0 } else { 0.0 };
                    if v4932 != 0.0 {
                    } else {
                        let v4934 = if v4930 > v4933 { 1.0 } else { 0.0 };
                        if v4934 != 0.0 {
                        } else {
                            let v4936 = if v4930 < v4935 { 1.0 } else { 0.0 };
                            if v4936 != 0.0 {
                            } else {
                            }
                        }
                    }
                    let v4946 = if ((v4910 + (v4942 * (v4921 * (v4897 * (if (v4905 / v4915) <= (v4915 / v4905) { (v4905 / v4915) } else { (v4915 / v4905) }))))) / v4910) > v185 { 1.0 } else { 0.0 };
                    if v4946 != 0.0 {
                    } else {
                    }
                    let v4947 = v4922 / v4896;
                    let v4948 = v4947 + v1;
                    let v4971 = if (((((((((v4900 * v4900) + ((v23 * v4896) * v4900)) + ((v4896 * v4896) * v4948)).sqrt()) * (v4948.sqrt())) + v4900) + (v4896 * v4947)) + v4896) / ((v4900 * ((v4948 * (v4947 + v22)).sqrt())) + (v4900 * (v4947 + v23)))) > v185 { 1.0 } else { 0.0 };
                    if v4971 != 0.0 {
                    } else {
                    }
                    let v4972 = v4918 * v4919;
                    let v4973 = v4972 * v4972;
                    let v4974 = v4973 + v1;
                    let v4976 = v4972 * v4900;
                    let v4994 = if ((((((v4974 * (((v4976 * v4976) + (((v23 * v4972) * v4922) * v4900)) + ((v4974 * v4922) * v4922))).sqrt()) + v4976) + (v4973 * v4922)) + v4922) / (((v4974.sqrt()) + v1) * v4976)) > v185 { 1.0 } else { 0.0 };
                    if v4994 != 0.0 {
                    } else {
                    }
                }
                if v4906 != 0.0 {
                    let v4995 = v4902 + v4900;
                    let v4998 = v4914 + ((v4571 * v4995) / v4899);
                    let v5000 = (v4995 - v4899).abs();
                    let v5001 = v4910 * v4919;
                    let v5008 = v4926 * ((v18 * ((if v4899 <= v4995 { v4899 } else { v4995 }) - (v4910 / (v4998 + v1)))) / v4910);
                    let v5009 = if v5008 > v4931 { 1.0 } else { 0.0 };
                    if v5009 != 0.0 {
                    } else {
                        let v5010 = if v5008 > v4933 { 1.0 } else { 0.0 };
                        if v5010 != 0.0 {
                        } else {
                            let v5012 = if v5008 < v5011 { 1.0 } else { 0.0 };
                            if v5012 != 0.0 {
                            } else {
                            }
                        }
                    }
                    let v5022 = if ((v4910 + (v5018 * (v5000 * (v4897 * (if (v4899 / v4995) <= (v4995 / v4899) { (v4899 / v4995) } else { (v4995 / v4899) }))))) / v4910) > v185 { 1.0 } else { 0.0 };
                    if v5022 != 0.0 {
                    } else {
                    }
                    let v5023 = v5001 / v4902;
                    let v5024 = v5023 + v1;
                    let v5047 = if (((((((((v4900 * v4900) + ((v23 * v4902) * v4900)) + ((v4902 * v4902) * v5024)).sqrt()) * (v5024.sqrt())) + v4900) + (v4902 * v5023)) + v4902) / ((v4900 * ((v5024 * (v5023 + v22)).sqrt())) + (v4900 * (v5023 + v23)))) > v185 { 1.0 } else { 0.0 };
                    if v5047 != 0.0 {
                    } else {
                    }
                    let v5048 = v4998 * v4919;
                    let v5049 = v5048 * v5048;
                    let v5050 = v5049 + v1;
                    let v5052 = v5048 * v4900;
                    let v5070 = if ((((((v5050 * (((v5052 * v5052) + (((v23 * v5048) * v5001) * v4900)) + ((v5050 * v5001) * v5001))).sqrt()) + v5052) + (v5049 * v5001)) + v5001) / (((v5050.sqrt()) + v1) * v5052)) > v185 { 1.0 } else { 0.0 };
                    if v5070 != 0.0 {
                    } else {
                    }
                } else {
                    let v5072 = v4902 + v4900;
                    let v5075 = v4914 + ((v4571 * v5072) / v4899);
                    let v5077 = (v5072 - v4899).abs();
                    let v5078 = v4910 * v4919;
                    let v5085 = v4926 * ((v18 * ((if v4899 <= v5072 { v4899 } else { v5072 }) - (v4910 / (v5075 + v1)))) / v4910);
                    let v5086 = if v5085 > v4931 { 1.0 } else { 0.0 };
                    if v5086 != 0.0 {
                    } else {
                        let v5087 = if v5085 > v4933 { 1.0 } else { 0.0 };
                        if v5087 != 0.0 {
                        } else {
                            let v5089 = if v5085 < v5088 { 1.0 } else { 0.0 };
                            if v5089 != 0.0 {
                            } else {
                            }
                        }
                    }
                    let v5099 = if ((v4910 + (v5095 * (v5077 * (v4897 * (if (v4899 / v5072) <= (v5072 / v4899) { (v4899 / v5072) } else { (v5072 / v4899) }))))) / v4910) > v185 { 1.0 } else { 0.0 };
                    if v5099 != 0.0 {
                    } else {
                    }
                    let v5100 = v5078 / v4902;
                    let v5101 = v5100 + v1;
                    let v5124 = if (((((((((v4900 * v4900) + ((v23 * v4902) * v4900)) + ((v4902 * v4902) * v5101)).sqrt()) * (v5101.sqrt())) + v4900) + (v4902 * v5100)) + v4902) / ((v4900 * ((v5101 * (v5100 + v22)).sqrt())) + (v4900 * (v5100 + v23)))) > v185 { 1.0 } else { 0.0 };
                    if v5124 != 0.0 {
                    } else {
                    }
                    let v5125 = v5075 * v4919;
                    let v5126 = v5125 * v5125;
                    let v5127 = v5126 + v1;
                    let v5129 = v5125 * v4900;
                    let v5147 = if ((((((v5127 * (((v5129 * v5129) + (((v23 * v5125) * v5078) * v4900)) + ((v5127 * v5078) * v5078))).sqrt()) + v5129) + (v5126 * v5078)) + v5078) / (((v5127.sqrt()) + v1) * v5129)) > v185 { 1.0 } else { 0.0 };
                    if v5147 != 0.0 {
                    } else {
                    }
                }
                if v4906 != 0.0 {
                } else {
                    let v5148 = if v4731 > v0 { 1.0 } else { 0.0 };
                    if v5148 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v5149 = if v4882 == v276 { 1.0 } else { 0.0 };
            if v5149 != 0.0 {
                let v5150 = v4894 + v4895;
                let v5152 = v4897 * (v4660 - v117);
                let v5154 = if v0 >= (v5152 - v4900) { v0 } else { (v5152 - v4900) };
                let v5156 = if v0 >= (v4731 + v4903) { v0 } else { (v4731 + v4903) };
                let v5158 = v4897 * v5157;
                let v5159 = if v4895 > v0 { 1.0 } else { 0.0 };
                if v5159 != 0.0 {
                    let v5163 = if ((v4907 * v17) / (v4909 * v4910)) > v185 { 1.0 } else { 0.0 };
                    if v5163 != 0.0 {
                    } else {
                    }
                } else {
                    let v5164 = v5150 + v4900;
                    let v5167 = v4914 + ((v4571 * v5164) / v5156);
                    let v5169 = (v5164 - v5156).abs();
                    let v5170 = v4910 * v4919;
                    let v5177 = v4926 * ((v18 * ((if v5156 <= v5164 { v5156 } else { v5164 }) - (v4910 / (v5167 + v1)))) / v4910);
                    let v5178 = if v5177 > v4931 { 1.0 } else { 0.0 };
                    if v5178 != 0.0 {
                    } else {
                        let v5179 = if v5177 > v4933 { 1.0 } else { 0.0 };
                        if v5179 != 0.0 {
                        } else {
                            let v5181 = if v5177 < v5180 { 1.0 } else { 0.0 };
                            if v5181 != 0.0 {
                            } else {
                            }
                        }
                    }
                    let v5191 = if ((v4910 + (v5187 * (v5169 * (v4897 * (if (v5156 / v5164) <= (v5164 / v5156) { (v5156 / v5164) } else { (v5164 / v5156) }))))) / v4910) > v185 { 1.0 } else { 0.0 };
                    if v5191 != 0.0 {
                    } else {
                    }
                    let v5192 = v5170 / v5150;
                    let v5193 = v5192 + v1;
                    let v5216 = if (((((((((v4900 * v4900) + ((v23 * v5150) * v4900)) + ((v5150 * v5150) * v5193)).sqrt()) * (v5193.sqrt())) + v4900) + (v5150 * v5192)) + v5150) / ((v4900 * ((v5193 * (v5192 + v22)).sqrt())) + (v4900 * (v5192 + v23)))) > v185 { 1.0 } else { 0.0 };
                    if v5216 != 0.0 {
                    } else {
                    }
                    let v5217 = v5167 * v4919;
                    let v5218 = v5217 * v5217;
                    let v5219 = v5218 + v1;
                    let v5221 = v5217 * v4900;
                    let v5239 = if ((((((v5219 * (((v5221 * v5221) + (((v23 * v5217) * v5170) * v4900)) + ((v5219 * v5170) * v5170))).sqrt()) + v5221) + (v5218 * v5170)) + v5170) / (((v5219.sqrt()) + v1) * v5221)) > v185 { 1.0 } else { 0.0 };
                    if v5239 != 0.0 {
                    } else {
                    }
                }
                let v5240 = v4894 + v4900;
                let v5243 = v4914 + ((v4571 * v5240) / v5158);
                let v5245 = (v5240 - v5158).abs();
                let v5246 = v4910 * v4919;
                let v5253 = v4926 * ((v18 * ((if v5158 <= v5240 { v5158 } else { v5240 }) - (v4910 / (v5243 + v1)))) / v4910);
                let v5254 = if v5253 > v4931 { 1.0 } else { 0.0 };
                if v5254 != 0.0 {
                } else {
                    let v5255 = if v5253 > v4933 { 1.0 } else { 0.0 };
                    if v5255 != 0.0 {
                    } else {
                        let v5257 = if v5253 < v5256 { 1.0 } else { 0.0 };
                        if v5257 != 0.0 {
                        } else {
                        }
                    }
                }
                let v5267 = if ((v4910 + (v5263 * (v5245 * (v4897 * (if (v5158 / v5240) <= (v5240 / v5158) { (v5158 / v5240) } else { (v5240 / v5158) }))))) / v4910) > v185 { 1.0 } else { 0.0 };
                if v5267 != 0.0 {
                } else {
                }
                let v5268 = v5246 / v4894;
                let v5269 = v5268 + v1;
                let v5270 = v4900 * v4900;
                let v5292 = if ((((((((v5270 + ((v23 * v4894) * v4900)) + ((v4894 * v4894) * v5269)).sqrt()) * (v5269.sqrt())) + v4900) + (v4894 * v5268)) + v4894) / ((v4900 * ((v5269 * (v5268 + v22)).sqrt())) + (v4900 * (v5268 + v23)))) > v185 { 1.0 } else { 0.0 };
                if v5292 != 0.0 {
                } else {
                }
                let v5293 = v5243 * v4919;
                let v5294 = v5293 * v5293;
                let v5295 = v5294 + v1;
                let v5297 = v5293 * v4900;
                let v5315 = if ((((((v5295 * (((v5297 * v5297) + (((v23 * v5293) * v5246) * v4900)) + ((v5295 * v5246) * v5246))).sqrt()) + v5297) + (v5294 * v5246)) + v5246) / (((v5295.sqrt()) + v1) * v5297)) > v185 { 1.0 } else { 0.0 };
                if v5315 != 0.0 {
                } else {
                }
                if v5159 != 0.0 {
                    let v5316 = v5154 + v4900;
                    let v5319 = v4914 + ((v4571 * v5316) / v5152);
                    let v5321 = (v5316 - v5152).abs();
                    let v5328 = v4926 * ((v18 * ((if v5152 <= v5316 { v5152 } else { v5316 }) - (v4910 / (v5319 + v1)))) / v4910);
                    let v5329 = if v5328 > v4931 { 1.0 } else { 0.0 };
                    if v5329 != 0.0 {
                    } else {
                        let v5330 = if v5328 > v4933 { 1.0 } else { 0.0 };
                        if v5330 != 0.0 {
                        } else {
                            let v5332 = if v5328 < v5331 { 1.0 } else { 0.0 };
                            if v5332 != 0.0 {
                            } else {
                            }
                        }
                    }
                    let v5342 = if ((v4910 + (v5338 * (v5321 * (v4897 * (if (v5152 / v5316) <= (v5316 / v5152) { (v5152 / v5316) } else { (v5316 / v5152) }))))) / v4910) > v185 { 1.0 } else { 0.0 };
                    if v5342 != 0.0 {
                    } else {
                    }
                    let v5343 = v5246 / v5154;
                    let v5344 = v5343 + v1;
                    let v5366 = if ((((((((v5270 + ((v23 * v5154) * v4900)) + ((v5154 * v5154) * v5344)).sqrt()) * (v5344.sqrt())) + v4900) + (v5154 * v5343)) + v5154) / ((v4900 * ((v5344 * (v5343 + v22)).sqrt())) + (v4900 * (v5343 + v23)))) > v185 { 1.0 } else { 0.0 };
                    if v5366 != 0.0 {
                    } else {
                    }
                    let v5367 = v5319 * v4919;
                    let v5368 = v5367 * v5367;
                    let v5369 = v5368 + v1;
                    let v5371 = v5367 * v4900;
                    let v5389 = if ((((((v5369 * (((v5371 * v5371) + (((v23 * v5367) * v5246) * v4900)) + ((v5369 * v5246) * v5246))).sqrt()) + v5371) + (v5368 * v5246)) + v5246) / (((v5369.sqrt()) + v1) * v5371)) > v185 { 1.0 } else { 0.0 };
                    if v5389 != 0.0 {
                    } else {
                    }
                } else {
                    let v5390 = v5154 + v4900;
                    let v5393 = v4914 + ((v4571 * v5390) / v5152);
                    let v5395 = (v5390 - v5152).abs();
                    let v5402 = v4926 * ((v18 * ((if v5152 <= v5390 { v5152 } else { v5390 }) - (v4910 / (v5393 + v1)))) / v4910);
                    let v5403 = if v5402 > v4931 { 1.0 } else { 0.0 };
                    if v5403 != 0.0 {
                    } else {
                        let v5404 = if v5402 > v4933 { 1.0 } else { 0.0 };
                        if v5404 != 0.0 {
                        } else {
                            let v5406 = if v5402 < v5405 { 1.0 } else { 0.0 };
                            if v5406 != 0.0 {
                            } else {
                            }
                        }
                    }
                    let v5416 = if ((v4910 + (v5412 * (v5395 * (v4897 * (if (v5152 / v5390) <= (v5390 / v5152) { (v5152 / v5390) } else { (v5390 / v5152) }))))) / v4910) > v185 { 1.0 } else { 0.0 };
                    if v5416 != 0.0 {
                    } else {
                    }
                    let v5417 = v5246 / v5154;
                    let v5418 = v5417 + v1;
                    let v5440 = if ((((((((v5270 + ((v23 * v5154) * v4900)) + ((v5154 * v5154) * v5418)).sqrt()) * (v5418.sqrt())) + v4900) + (v5154 * v5417)) + v5154) / ((v4900 * ((v5418 * (v5417 + v22)).sqrt())) + (v4900 * (v5417 + v23)))) > v185 { 1.0 } else { 0.0 };
                    if v5440 != 0.0 {
                    } else {
                    }
                    let v5441 = v5393 * v4919;
                    let v5442 = v5441 * v5441;
                    let v5443 = v5442 + v1;
                    let v5445 = v5441 * v4900;
                    let v5463 = if ((((((v5443 * (((v5445 * v5445) + (((v23 * v5441) * v5246) * v4900)) + ((v5443 * v5246) * v5246))).sqrt()) + v5445) + (v5442 * v5246)) + v5246) / (((v5443.sqrt()) + v1) * v5445)) > v185 { 1.0 } else { 0.0 };
                    if v5463 != 0.0 {
                    } else {
                    }
                }
                let v5464 = v5154 + v4900;
                let v5467 = v4914 + ((v4571 * v5464) / v5152);
                let v5469 = (v5464 - v5152).abs();
                let v5474 = v18 * ((if v5152 <= v5464 { v5152 } else { v5464 }) - (v4910 / (v5467 + v1)));
                let v5476 = v4926 * (v5474 / v4910);
                let v5477 = if v5476 > v4931 { 1.0 } else { 0.0 };
                if v5477 != 0.0 {
                } else {
                    let v5478 = if v5476 > v4933 { 1.0 } else { 0.0 };
                    if v5478 != 0.0 {
                    } else {
                        let v5480 = if v5476 < v5479 { 1.0 } else { 0.0 };
                        if v5480 != 0.0 {
                        } else {
                        }
                    }
                }
                let v5485 = v5469 * (v4897 * (if (v5152 / v5464) <= (v5464 / v5152) { (v5152 / v5464) } else { (v5464 / v5152) }));
                let v5490 = if ((v4910 + (v5486 * v5485)) / v4910) > v185 { 1.0 } else { 0.0 };
                if v5490 != 0.0 {
                } else {
                }
                let v5491 = v5246 / v5154;
                let v5492 = v5491 + v1;
                let v5502 = ((((v5270 + ((v23 * v5154) * v4900)) + ((v5154 * v5154) * v5492)).sqrt()) * (v5492.sqrt())) + v4900;
                let v5503 = v5154 * v5491;
                let v5512 = (v4900 * ((v5492 * (v5491 + v22)).sqrt())) + (v4900 * (v5491 + v23));
                let v5514 = if (((v5502 + v5503) + v5154) / v5512) > v185 { 1.0 } else { 0.0 };
                if v5514 != 0.0 {
                } else {
                }
                let v5515 = v5467 * v4919;
                let v5516 = v5515 * v5515;
                let v5517 = v5516 + v1;
                let v5518 = v5517.sqrt();
                let v5519 = v5515 * v4900;
                let v5520 = v5519 * v5519;
                let v5523 = ((v23 * v5515) * v5246) * v4900;
                let v5525 = v5517 * v5246;
                let v5531 = v5516 * v5246;
                let v5537 = if ((((((v5517 * ((v5520 + v5523) + (v5525 * v5246))).sqrt()) + v5519) + v5531) + v5246) / ((v5518 + v1) * v5519)) > v185 { 1.0 } else { 0.0 };
                if v5537 != 0.0 {
                } else {
                }
                let v5539 = v4926 * (v5474 / v4910);
                let v5540 = if v5539 > v4931 { 1.0 } else { 0.0 };
                if v5540 != 0.0 {
                } else {
                    let v5541 = if v5539 > v4933 { 1.0 } else { 0.0 };
                    if v5541 != 0.0 {
                    } else {
                        let v5543 = if v5539 < v5542 { 1.0 } else { 0.0 };
                        if v5543 != 0.0 {
                        } else {
                        }
                    }
                }
                let v5548 = if ((v4910 + (v5544 * v5485)) / v4910) > v185 { 1.0 } else { 0.0 };
                if v5548 != 0.0 {
                } else {
                }
                let v5552 = if (((v5502 + v5503) + v5154) / v5512) > v185 { 1.0 } else { 0.0 };
                if v5552 != 0.0 {
                } else {
                }
                let v5564 = if ((((((v5517 * ((v5520 + v5523) + (v5525 * v5246))).sqrt()) + v5519) + v5531) + v5246) / ((v5518 + v1) * v5519)) > v185 { 1.0 } else { 0.0 };
                if v5564 != 0.0 {
                } else {
                }
                if v5159 != 0.0 {
                } else {
                    let v5565 = if v4731 > v0 { 1.0 } else { 0.0 };
                    if v5565 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v5568 = if (v1 + (v205 / v20)) > v185 { 1.0 } else { 0.0 };
            if v5568 != 0.0 {
            } else {
            }
            let v5569 = if v100 != v101 { 1.0 } else { 0.0 };
            if v5569 != 0.0 {
            } else {
            }
            let v5571 = v21 * v209;
            let v5572 = v5570 / v5571;
            let v5577 = v1 / (v153 * ((v423 * v5573).powf(v1458)));
            let v5580 = ((v5571 * v4897) * v211).sqrt();
            let v5590 = (((v16 * v399) / v378) * (v1 + ((v399 * v378) / (((v23 * v16) * v388) * v388)))).sqrt();
            let v5592 = if v5591 == 0.0 { 1.0 } else { 0.0 };
            let v10593: f64;
            if v5592 != 0.0 {
                let v5596 = ((v5593 * v3969) / v5590) + v25;
                let v5598 = if v5596 < v5597 { 1.0 } else { 0.0 };
                let v10594: f64;
                if v5598 != 0.0 {
                    let v5601 = v4897 / ((v5596.cosh()) - v1);
                    v10594 = v5601;
                } else {
                    let v5603 = rspice_limited_exp((-v5596));
                    v10594 = v5603;
                }
                v10593 = v10594;
            } else {
                v10593 = v5604;
            }
            let v5606 = if v5605 == 0.0 { 1.0 } else { 0.0 };
            let v10535: f64;
            if v5606 != 0.0 {
                let v5610 = ((v5607 * v3969) / v5590) + v25;
                let v5611 = if v5610 < v5597 { 1.0 } else { 0.0 };
                let v10536: f64;
                if v5611 != 0.0 {
                    let v5614 = v4897 / ((v5610.cosh()) - v1);
                    v10536 = v5614;
                } else {
                    let v5616 = rspice_limited_exp((-v5610));
                    v10536 = v5616;
                }
                v10535 = v10536;
            } else {
                v10535 = v5617;
            }
            let v5619 = if v5618 == 0.0 { 1.0 } else { 0.0 };
            let v10601: f64;
            if v5619 != 0.0 {
                let v5623 = ((v5620 * v3969) / v5590) + v25;
                let v5624 = if v5623 < v5597 { 1.0 } else { 0.0 };
                let v10602: f64;
                if v5624 != 0.0 {
                    let v5627 = v4897 / ((v5623.cosh()) - v1);
                    v10602 = v5627;
                } else {
                    let v5629 = rspice_limited_exp((-v5623));
                    v10602 = v5629;
                }
                v10601 = v10602;
            } else {
                v10601 = v5630;
            }
            let v5635 = ((v1 + (v5631 / v3969)).sqrt()) - v1;
            let v5638 = ((v5620 * v3969) / v5590) + v25;
            let v5639 = if v5638 < v5597 { 1.0 } else { 0.0 };
            let v10609: f64;
            if v5639 != 0.0 {
                let v5646 = v1 / (if (v1 + (v5640 * ((v5638.cosh()) - v23))) >= v25 { (v1 + (v5640 * ((v5638.cosh()) - v23))) } else { v25 });
                v10609 = v5646;
            } else {
                let v5648 = rspice_limited_exp((-v5638));
                let v5651 = v5648 / (if (v5640 + v5648) >= v25 { (v5640 + v5648) } else { v25 });
                v10609 = v5651;
            }
            let v5654 = ((v415 * v417) * v399) / v378;
            let v5671: f64;
            let v13024: f64;
            if v12 != 0.0 {
                v5671 = v5655;
                v13024 = v5656;
            } else {
                v5671 = v5657;
                v13024 = v5658;
            }
            let v5660 = v5659 * v5659;
            let v5663 = v5659 * v5661;
            let v5667 = ((v4627 / v5659).powf(v1922)) / v5660;
            let v5673 = (v423 * v5671) * (((v4627 / v5663).powf(v1922)) / (v5663 * v5663));
            let v5677 = if v5674 < v5676 { 1.0 } else { 0.0 };
            let v6595: f64;
            if v5677 != 0.0 {
                v6595 = v5678;
            } else {
                let v5679 = v5674 + v5675;
                v6595 = v5679;
            }
            let v10871: f64;
            let v10878: f64;
            let v10881: f64;
            let v10886: f64;
            let v10890: f64;
            let v10897: f64;
            let v10900: f64;
            let v10905: f64;
            let v10909: f64;
            let v10916: f64;
            let v10919: f64;
            let v10924: f64;
            let v10926: f64;
            if v4000 != 0.0 {
                let v5683 = v5680 - v5681;
                let v5687 = v117 * v5685;
                let v5692 = v1 + (rspice_limited_exp((((v5684 * v5685) - v5687) / v5689)));
                let v5694 = (v5683 / v5692) + v5681;
                let v5700 = ((v5695 - v5696) / v5692) + v5696;
                let v5706 = ((v5701 - v5702) / v5692) + v5702;
                let v5715 = v1 + (rspice_limited_exp((((v5709 * v5685) - v5687) / v5712)));
                let v5719 = ((-v5707) / v5715) + v5717;
                let v5726 = v4897 * (v5719 + (((v5719 * v5719) + v5722).sqrt()));
                let v5732 = ((-v5727) / v5715) + v5730;
                let v5738 = v4897 * (v5732 + (((v5732 * v5732) + v5734).sqrt()));
                let v5744 = ((-v5739) / v5715) + v5742;
                let v5750 = v4897 * (v5744 + (((v5744 * v5744) + v5746).sqrt()));
                let v5753 = v5751 * (v5694 - v5751);
                let v5754 = v5753 * v5753;
                let v5783 = v5751 * (v5694 - v5781);
                let v5784 = v5783 * v5783;
                let v5812 = v5751 * (v5700 - v5751);
                let v5813 = v5812 * v5812;
                let v5841 = v5751 * (v5700 - v5781);
                let v5842 = v5841 * v5841;
                let v5870 = v5751 * (v5706 - v5751);
                let v5871 = v5870 * v5870;
                let v5899 = v5751 * (v5706 - v5781);
                let v5900 = v5899 * v5899;
                let v5928 = v399 / v423;
                let v5941 = v298 * v5685;
                let v5947 = v1 / (v1 + (rspice_limited_exp(((v5940 - v5941) / v5943))));
                let v5949 = v5947 - v4897;
                let v5955 = v4897 * ((v5947 + v4897) + (((v5949 * v5949) + v5951).sqrt()));
                let v5960 = (((v1 - v5955) * (v5694 - v5681)) / v5683) + v5955;
                let v5966 = v1 / (v1 + (rspice_limited_exp(((v5960 - v5961) / v4767))));
                let v5974 = ((((v4897 * v298) * v298) * v197) - ((v5970 * v298) * v5685)) + v23;
                let v5976 = v5974 - v22;
                let v5985 = v5941 - ((v4897 * ((v5974 + v22) - (((v5976 * v5976) + v5978).sqrt()))) + v5983);
                let v5987 = v5985.powf(v5986);
                let v5992 = v23.powf(v5986);
                let v6064 = (v4897 * (((v4897 * (((v5660 + ((v5987 * v5990) / v5992)) + v5989) + (((((v5660 + ((v5987 * v5996) / v5992)) - v5989) * ((v5660 + ((v5987 * v6001) / v5992)) - v5989)) + v6007).sqrt()))) + v5988) - (((((v4897 * (((v5660 + ((v5987 * v6013) / v5992)) + v5989) + (((((v5660 + ((v5987 * v6018) / v5992)) - v5989) * ((v5660 + ((v5987 * v6023) / v5992)) - v5989)) + v6029).sqrt()))) - v5988) * ((v4897 * (((v5660 + ((v5987 * v6035) / v5992)) + v5989) + (((((v5660 + ((v5987 * v6040) / v5992)) - v5989) * ((v5660 + ((v5987 * v6045) / v5992)) - v5989)) + v6051).sqrt()))) - v5988)) + v6058).sqrt()))) + v6063;
                let v6067 = v5985.powf(v6066);
                let v6071 = v23.powf(v6066);
                let v6140 = (v6135 * (v22.powf(v6136))) / (v5941.powf(v6136));
                let v6151 = (v6146 * (v22.powf(v6147))) / (v5941.powf(v6147));
                let v6156 = v5071 + ((v5985.powf(v6152)) * v6154);
                let v6157 = v6156 + v4897;
                let v6158 = v6156 - v4897;
                let v6159 = v6158 * v6158;
                let v6191 = (v6186 * (v22.powf(v6187))) / (v5941.powf(v6187));
                let v6202 = (v6197 * (v22.powf(v6198))) / (v5941.powf(v6198));
                let v6210 = v6209 * v5685;
                let v6212 = v6211 * ((v4897 * (((v4897 * ((v6065 + ((v6067 * v6069) / v6071)) + ((((v6065 + ((v6067 * v6074) / v6071)) * (v6065 + ((v6067 * v6078) / v6071))) + v6083).sqrt()))) + v6068) - (((((v4897 * ((v6065 + ((v6067 * v6089) / v6071)) + ((((v6065 + ((v6067 * v6093) / v6071)) * (v6065 + ((v6067 * v6097) / v6071))) + v6102).sqrt()))) - v6068) * ((v4897 * ((v6065 + ((v6067 * v6108) / v6071)) + ((((v6065 + ((v6067 * v6112) / v6071)) * (v6065 + ((v6067 * v6116) / v6071))) + v6121).sqrt()))) - v6068)) + v6128).sqrt()))) + v6133);
                let v6216 = v6215 * ((v23 * (v22.powf(v6141))) / (v5941.powf(v6141)));
                let v6226 = v6219 + (v6221 * ((v6064 / (v5687.powf(v6212))) - (v6064 / (v6210.powf(v6212)))));
                let v6232 = v4897 * (v6226 + (((v6226 * v6226) + v6228).sqrt()));
                let v6240 = v6233 + (v6235 * ((v6140 / (v5687.powf(v6216))) - (v6140 / (v6210.powf(v6216)))));
                let v6246 = v4897 * (v6240 + (((v6240 * v6240) + v6242).sqrt()));
                let v6248 = v6247 * ((v4897 * (((v4897 * (v6157 + ((v6159 + v6160).sqrt()))) + v1) - (((((v4897 * (v6157 + ((v6159 + v6166).sqrt()))) - v1) * ((v4897 * (v6157 + ((v6159 + v6172).sqrt()))) - v1)) + v6179).sqrt()))) + v6184);
                let v6253 = v6151 / ((v1 + (v101 * (v5687.powf(v6248)))).sqrt());
                let v6264 = v6151 / ((v1 + (v101 * (v6210.powf(v6248)))).sqrt());
                let v6278 = v6277 * ((v5970 * (v22.powf(v6192))) / (v5941.powf(v6192)));
                let v6283 = v6191 / ((v1 + (v101 * (v5687.powf(v6278)))).sqrt());
                let v6294 = v6191 / ((v1 + (v101 * (v6210.powf(v6278)))).sqrt());
                let v6308 = v6307 * ((v6203 * (v22.powf(v6204))) / (v5941.powf(v6204)));
                let v6313 = v6202 / ((v1 + (v101 * (v5687.powf(v6308)))).sqrt());
                let v6324 = v6202 / ((v1 + (v101 * (v6210.powf(v6308)))).sqrt());
                let v6337 = v5694 / v23;
                let v6343 = v1 + v6337;
                let v6344 = v6343 - v1;
                let v6350 = (v23 * v6343) - v276;
                let v6355 = v23 * v6344;
                let v17973 = v6355 * v6355;
                let v17974 = v17973 * v6355;
                let v17975 = v17974 * v6355;
                let v17976 = v17975 * v6355;
                let v6382 = (((v415 * v6337) * ((v6339.powf(v6337)) / (((((((((v6341 * ((v6342 * v6344).exp())) + (v6348 * (((((((v6350 * v6350) * v6350) * v6350) * v6350) * v6350) * v6350) * v6350))) + (v6354 * (v17976 * v6355))) - (v6360 * v17976)) + (v6363 * v17975)) - (v6366 * v17974)) + (v6369 * v17973)) - (v6372 * v6344)) + v6375))) * (((v6271 + (v6273 * ((v4897 * (v6253 + (((v6253 * v6253) + v6255).sqrt()))) - (v4897 * (v6264 + (((v6264 * v6264) + v6266).sqrt())))))) * v5573).powf(v5694))) * (((v423.powf(((v4897 * (((v4897 * (v5753 + ((v5754 + v5755).sqrt()))) + v1) - (((((v4897 * (v5753 + ((v5754 + v5761).sqrt()))) - v1) * ((v4897 * (v5753 + ((v5754 + v5767).sqrt()))) - v1)) + v5774).sqrt()))) + v5779))) * (v5928.powf(((v4897 * (((v4897 * (v5783 + ((v5784 + v5785).sqrt()))) + v1) - (((((v4897 * (v5783 + ((v5784 + v5791).sqrt()))) - v1) * ((v4897 * (v5783 + ((v5784 + v5797).sqrt()))) - v1)) + v5804).sqrt()))) + v5809)))) / v378);
                let v6383 = v5700 / v23;
                let v6387 = v1 + v6383;
                let v6388 = v6387 - v1;
                let v6393 = (v23 * v6387) - v276;
                let v6397 = v23 * v6388;
                let v17983 = v6397 * v6397;
                let v17984 = v17983 * v6397;
                let v17985 = v17984 * v6397;
                let v17986 = v17985 * v6397;
                let v6418 = (((v415 * v6383) * ((v6339.powf(v6383)) / (((((((((v6341 * ((v6386 * v6388).exp())) + (v6348 * (((((((v6393 * v6393) * v6393) * v6393) * v6393) * v6393) * v6393) * v6393))) + (v6354 * (v17986 * v6397))) - (v6360 * v17986)) + (v6363 * v17985)) - (v6366 * v17984)) + (v6369 * v17983)) - (v6409 * v6388)) + v6375))) * (((v6301 + (v6303 * ((v4897 * (v6283 + (((v6283 * v6283) + v6285).sqrt()))) - (v4897 * (v6294 + (((v6294 * v6294) + v6296).sqrt())))))) * v5573).powf(v5700))) * (((v423.powf(((v4897 * (((v4897 * (v5812 + ((v5813 + v5814).sqrt()))) + v1) - (((((v4897 * (v5812 + ((v5813 + v5820).sqrt()))) - v1) * ((v4897 * (v5812 + ((v5813 + v5826).sqrt()))) - v1)) + v5833).sqrt()))) + v5838))) * (v5928.powf(((v4897 * (((v4897 * (v5841 + ((v5842 + v5843).sqrt()))) + v1) - (((((v4897 * (v5841 + ((v5842 + v5849).sqrt()))) - v1) * ((v4897 * (v5841 + ((v5842 + v5855).sqrt()))) - v1)) + v5862).sqrt()))) + v5867)))) / v378);
                let v6419 = v5706 / v23;
                let v6423 = v1 + v6419;
                let v6424 = v6423 - v1;
                let v6429 = (v23 * v6423) - v276;
                let v6433 = v23 * v6424;
                let v17993 = v6433 * v6433;
                let v17994 = v17993 * v6433;
                let v17995 = v17994 * v6433;
                let v17996 = v17995 * v6433;
                let v6454 = (((v415 * v6419) * ((v6339.powf(v6419)) / (((((((((v6341 * ((v6422 * v6424).exp())) + (v6348 * (((((((v6429 * v6429) * v6429) * v6429) * v6429) * v6429) * v6429) * v6429))) + (v6354 * (v17996 * v6433))) - (v6360 * v17996)) + (v6363 * v17995)) - (v6366 * v17994)) + (v6369 * v17993)) - (v6445 * v6424)) + v6375))) * (((v6331 + (v6333 * ((v4897 * (v6313 + (((v6313 * v6313) + v6315).sqrt()))) - (v4897 * (v6324 + (((v6324 * v6324) + v6326).sqrt())))))) * v5573).powf(v5706))) * (((v423.powf(((v4897 * (((v4897 * (v5870 + ((v5871 + v5872).sqrt()))) + v1) - (((((v4897 * (v5870 + ((v5871 + v5878).sqrt()))) - v1) * ((v4897 * (v5870 + ((v5871 + v5884).sqrt()))) - v1)) + v5891).sqrt()))) + v5896))) * (v5928.powf(((v4897 * (((v4897 * (v5899 + ((v5900 + v5901).sqrt()))) + v1) - (((((v4897 * (v5899 + ((v5900 + v5907).sqrt()))) - v1) * ((v4897 * (v5899 + ((v5900 + v5913).sqrt()))) - v1)) + v5920).sqrt()))) + v5925)))) / v378);
                v10871 = v5726;
                v10878 = v5694;
                v10881 = v6382;
                v10886 = v6232;
                v10890 = v5738;
                v10897 = v5700;
                v10900 = v6418;
                v10905 = v6246;
                v10909 = v5750;
                v10916 = v5706;
                v10919 = v6454;
                v10924 = v5960;
                v10926 = v5966;
            } else {
                v10871 = v0;
                v10878 = v0;
                v10881 = v0;
                v10886 = v0;
                v10890 = v0;
                v10897 = v0;
                v10900 = v0;
                v10905 = v0;
                v10909 = v0;
                v10916 = v0;
                v10919 = v0;
                v10924 = v1;
                v10926 = v0;
            }
            let v6456 = if v6455 == v1 { 1.0 } else { 0.0 };
            let v6853: f64;
            let v6918: f64;
            let v6932: f64;
            let v6949: f64;
            let v9007: f64;
            if v6456 != 0.0 {
                let v6462 = v298 * v5685;
                let v6469 = v6457 + ((v6458 - v6457) / ((rspice_limited_exp((((v6460 * v5685) - v6462) / v6464))) + v1));
                let v6488 = ((((v6470 - v6471) - ((v6473 * v5685) * v6475)) + (v6462 * v6475)) / (v1 + (rspice_limited_exp((((v6480 * v5685) - v6462) / v6483))))) + v6471;
                let v6489 = v6470 + v4571;
                let v6491 = v6488 - v6489;
                let v6497 = v4897 * ((v6488 + v6489) - (((v6491 * v6491) + v6493).sqrt()));
                let v6500 = v6498 - v6499;
                let v6515 = (((v6500 * v6501) / (v6462.powf(v6503))) + (v6500 / (v1 + (rspice_limited_exp(((v6462 - (v6506 * v5685)) / v6509)))))) + v6499;
                let v6517 = v6515 - v6498;
                let v6523 = v4897 * ((v6515 + v6498) - (((v6517 * v6517) + v6519).sqrt()));
                let v6525 = v117 / (v117 + v298);
                let v6532 = (((v6526 * v298) * v298) * v197) - (v6530 * v4637);
                let v6537 = v6526 + v6536;
                let v6556 = v4767 / ((((v6532 + (((v6532 * v6532) + ((((((v22 * v6530) * v4637) * v6537) * v298) * v298) * v197)).sqrt())) / ((((v23 * v6537) * v298) * v298) * v197)) - v6550) - (v6552 * v6553));
                let v6558 = v6556 - v1;
                let v6571 = (v6565 * (v6525 + (v6566 * (v1 - v6525)))) * (v4897 * ((v6556 + v1) - (((v6558 * v6558) + v6560).sqrt())));
                let v6577 = (v6575 * v5685) - v6462;
                let v6587 = ((v6572 - v6573) * ((v4897 * (v6577 + (((v6577 * v6577) + v6579).sqrt()))).powf(v6584))) + v6573;
                v6853 = v6571;
                v6918 = v6497;
                v6932 = v6523;
                v6949 = v6587;
                v9007 = v6469;
            } else {
                v6853 = v6565;
                v6918 = v6470;
                v6932 = v6499;
                v6949 = v6573;
                v9007 = v6458;
            }
            let v6594: f64;
            let v14606: Lanes<1>;
            if v4700 != 0.0 {
                let v6592 = (v6588 + v6589) + v6591;
                v6594 = v6592;
                v14606 = v14601;
            } else {
                let v6593 = v6588 + v6591;
                v6594 = v6593;
                v14606 = v14864;
            }
            let v6596 = v6594 / v6595;
            let v14865 = v14606 / v6595;
            let v6597 = v6596 - v1;
            let v6598 = v6594 - v6595;
            let v6600 = v6599 * v6594;
            let v14866 = v14606 * v6599;
            let v6601 = v6599 * v6595;
            let v6604 = if v6603 != v0 { 1.0 } else { 0.0 };
            let v6781: f64;
            let v7821: f64;
            let v7934: f64;
            let v7938: f64;
            let v8071: f64;
            let v8778: f64;
            let v10549: f64;
            let v14607: Lanes<1>;
            let v14608: Lanes<1>;
            let v14609: Lanes<1>;
            let v14610: Lanes<1>;
            let v14611: Lanes<1>;
            let v14612: Lanes<1>;
            let v14613: Lanes<1>;
            if v6604 != 0.0 {
                let v6606 = v6594 - v6602;
                let v14867 = v14606 * v6606;
                let v6610 = (v5721 * v6608) * v6608;
                let v6612 = ((v6606 * v6606) + v6610).sqrt();
                let v6614 = v4897 * ((v6594 + v6602) + v6612);
                let v14874 = (v14606 + ((v14867 + v14867) * (v14600 / (v14869 * v6612)))) * v4897;
                let v6616 = -v6615;
                let v6619 = v6616 * (v6594 - v6617);
                let v14875 = v14606 * v6616;
                let v14876 = v14875 * v6619;
                let v6623 = (v5721 * v6621) * v6621;
                let v6625 = ((v6619 * v6619) + v6623).sqrt();
                let v6627 = v4897 * (v6619 + v6625);
                let v14882 = (v14875 + ((v14876 + v14876) * (v14600 / (v14869 * v6625)))) * v4897;
                let v6628 = if v6603 == v1 { 1.0 } else { 0.0 };
                let v6750: f64;
                let v7822: f64;
                let v7935: f64;
                let v7939: f64;
                let v8072: f64;
                let v8779: f64;
                let v14614: Lanes<1>;
                let v14615: Lanes<1>;
                let v14616: Lanes<1>;
                let v14617: Lanes<1>;
                let v14618: Lanes<1>;
                let v14619: Lanes<1>;
                if v6628 != 0.0 {
                    let v6630 = v6595 - v6602;
                    let v6635 = v4897 * ((v6595 + v6602) + (((v6630 * v6630) + v6610).sqrt()));
                    let v6637 = v6616 * (v6595 - v6617);
                    let v6642 = v4897 * (v6637 + (((v6637 * v6637) + v6623).sqrt()));
                    let v6643 = if v6595 > v6602 { 1.0 } else { 0.0 };
                    let v6652: f64;
                    let v14620: Lanes<1>;
                    if v6643 != 0.0 {
                        let v14927 = v14874 + v14882;
                        let v6647 = (((v6614 + v6627) - v6635) - v6642) + v6595;
                        v6652 = v6647;
                        v14620 = v14927;
                    } else {
                        let v14926 = v14874 + v14882;
                        let v6651 = (((v6614 + v6627) - v6635) - v6642) + v6602;
                        v6652 = v6651;
                        v14620 = v14926;
                    }
                    let v6654 = v6594 - v6652;
                    let v14930 = (v14606 - v14620) * v6654;
                    let v6658 = ((v6654 * v6654) + v6656).sqrt();
                    let v6660 = v4897 * ((v6594 + v6652) + v6658);
                    let v14936 = ((v14606 + v14620) + ((v14930 + v14930) * (v14600 / (v14869 * v6658)))) * v4897;
                    v6750 = v6660;
                    v7822 = v0;
                    v7935 = v0;
                    v7939 = v0;
                    v8072 = v0;
                    v8779 = v0;
                    v14614 = v14936;
                    v14615 = v14864;
                    v14616 = v14864;
                    v14617 = v14864;
                    v14618 = v14864;
                    v14619 = v14864;
                } else {
                    let v6662 = if v6602 > v6661 { 1.0 } else { 0.0 };
                    let v6670: f64;
                    if v6662 != 0.0 {
                        v6670 = v6661;
                    } else {
                        v6670 = v6602;
                    }
                    let v6663 = v6594 - v6661;
                    let v6665 = (v4897 * v6663).tanh();
                    let v14887 = ((v14606 * v4897) * (v14600 - (v6665 * v6665))) * v4897;
                    let v6667 = v4897 + (v4897 * v6665);
                    let v6668 = v1 - v6667;
                    let v14889 = v14887 * v14888;
                    let v6669 = if v6595 > v6661 { 1.0 } else { 0.0 };
                    let v6751: f64;
                    let v14621: Lanes<1>;
                    if v6669 != 0.0 {
                        let v6672 = v6661 - v6670;
                        let v6679 = v6616 * (v6661 - v6617);
                        let v14908 = v14874 + v14882;
                        let v6688 = (((v6614 + v6627) - (v4897 * ((v6661 + v6670) + (((v6672 * v6672) + v6610).sqrt())))) - (v4897 * (v6679 + (((v6679 * v6679) + v6623).sqrt())))) + v6661;
                        let v6690 = v6594 - v6688;
                        let v14911 = (v14606 - v14908) * v6690;
                        let v6694 = ((v6690 * v6690) + v6692).sqrt();
                        let v6696 = v4897 * ((v6594 + v6688) + v6694);
                        let v14917 = ((v14606 + v14908) + ((v14911 + v14911) * (v14600 / (v14869 * v6694)))) * v4897;
                        v6751 = v6696;
                        v14621 = v14917;
                    } else {
                        let v6698 = v6595 - v6670;
                        let v6703 = v4897 * ((v6595 + v6670) + (((v6698 * v6698) + v6610).sqrt()));
                        let v6705 = v6616 * (v6595 - v6617);
                        let v6710 = v4897 * (v6705 + (((v6705 * v6705) + v6623).sqrt()));
                        let v6711 = if v6595 > v6670 { 1.0 } else { 0.0 };
                        let v6720: f64;
                        let v14622: Lanes<1>;
                        if v6711 != 0.0 {
                            let v14891 = v14874 + v14882;
                            let v6715 = (((v6614 + v6627) - v6703) - v6710) + v6595;
                            v6720 = v6715;
                            v14622 = v14891;
                        } else {
                            let v14890 = v14874 + v14882;
                            let v6719 = (((v6614 + v6627) - v6703) - v6710) + v6670;
                            v6720 = v6719;
                            v14622 = v14890;
                        }
                        let v6722 = v6594 - v6720;
                        let v14894 = (v14606 - v14622) * v6722;
                        let v6726 = ((v6722 * v6722) + v6724).sqrt();
                        let v6728 = v4897 * ((v6594 + v6720) + v6726);
                        let v6731 = (v6668 * v6728) + (v6667 * v6594);
                        let v14907 = ((v14889 * v6728) + ((((v14606 + v14622) + ((v14894 + v14894) * (v14600 / (v14869 * v6726)))) * v4897) * v6668)) + ((v14887 * v6594) + (v14606 * v6667));
                        v6751 = v6731;
                        v14621 = v14907;
                    }
                    let v14918 = v14606 * v6663;
                    let v6736 = ((v6663 * v6663) + v6734).sqrt();
                    let v6738 = v4897 * ((v6594 + v6661) - v6736);
                    let v14924 = (v14606 - ((v14918 + v14918) * (v14600 / (v14869 * v6736)))) * v4897;
                    let v6740 = v6595 - v6661;
                    let v6747 = v6738 - (v4897 * ((v6595 + v6661) - (((v6740 * v6740) + v6742).sqrt())));
                    let v6749 = (v6738 - v6661) / v6595;
                    let v14925 = v14924 / v6595;
                    v6750 = v6751;
                    v7822 = v6749;
                    v7935 = v6668;
                    v7939 = v6667;
                    v8072 = v6747;
                    v8779 = v6738;
                    v14614 = v14621;
                    v14615 = v14925;
                    v14616 = v14889;
                    v14617 = v14887;
                    v14618 = v14924;
                    v14619 = v14924;
                }
                let v6752 = v6599 * v6750;
                let v14937 = v14614 * v6599;
                v6781 = v6750;
                v7821 = v7822;
                v7934 = v7935;
                v7938 = v7939;
                v8071 = v8072;
                v8778 = v8779;
                v10549 = v6752;
                v14607 = v14614;
                v14608 = v14615;
                v14609 = v14616;
                v14610 = v14617;
                v14611 = v14618;
                v14612 = v14619;
                v14613 = v14937;
            } else {
                v6781 = v0;
                v7821 = v0;
                v7934 = v0;
                v7938 = v0;
                v8071 = v0;
                v8778 = v0;
                v10549 = v0;
                v14607 = v14864;
                v14608 = v14864;
                v14609 = v14864;
                v14610 = v14864;
                v14611 = v14864;
                v14612 = v14864;
                v14613 = v14864;
            }
            let v6755 = v6754 * v6594;
            let v6758 = v6594 + v6757;
            let v6759 = (v6755 * v6594) / v6758;
            let v6760 = v6753 - v6759;
            let v14945 = (((((v14606 * v6754) * v6594) + (v14606 * v6755)) - (v14606 * v6759)) / v6758) * v14888;
            let v6765 = v6753 - (((v6754 * v6595) * v6595) / (v6595 + v6757));
            let v6766 = v6594 / v5678;
            let v14946 = v14606 / v5678;
            let v6767 = v6766.sqrt();
            let v6768 = v6766 * v6767;
            let v14952 = (v14946 * v6767) + ((v14946 * (v14600 / (v14869 * v6767))) * v6766);
            let v6770 = v6769 * v6768;
            let v14953 = v14952 * v6769;
            let v6773 = v23 * v6600;
            let v14954 = v14866 * v23;
            let v6774 = v6760 / v6773;
            let v14957 = (v14945 - (v14954 * v6774)) / v6773;
            let v6775 = (v6753 / v6771) - v6774;
            let v6776 = rspice_limited_exp(v6775);
            let v6777 = v6770 * v6776;
            let v14963 = (v14953 * v6776) + (((v14957 * v14888) * (rspice_limited_exp_derivative(v6775))) * v6770);
            let v6778 = if v6603 == v0 { 1.0 } else { 0.0 };
            let v6837: f64;
            let v9901: f64;
            let v14623: Lanes<1>;
            let v14624: Lanes<1>;
            if v6778 != 0.0 {
                let v6780 = v6779 * v6768;
                let v14975 = v14952 * v6779;
                v6837 = v6780;
                v9901 = v0;
                v14623 = v14975;
                v14624 = v14864;
            } else {
                let v6782 = v6781 / v5678;
                let v14964 = v14607 / v5678;
                let v6783 = v6779 * v6782;
                let v6784 = v6782.sqrt();
                let v6785 = v6783 * v6784;
                let v14971 = ((v14964 * v6779) * v6784) + ((v14964 * (v14600 / (v14869 * v6784))) * v6783);
                let v6786 = if v6770 > v185 { 1.0 } else { 0.0 };
                let v6789: f64;
                let v14625: Lanes<1>;
                if v6786 != 0.0 {
                    let v6787 = v6770.ln();
                    let v14973 = v14953 * (v14600 / v6770);
                    v6789 = v6787;
                    v14625 = v14973;
                } else {
                    v6789 = v6788;
                    v14625 = v14864;
                }
                let v6793 = (v6789 + (v6753 / v6790)) - v6774;
                let v14974 = v14625 - v14957;
                v6837 = v6785;
                v9901 = v6793;
                v14623 = v14971;
                v14624 = v14974;
            }
            let v14976 = v14606 * v2914;
            let v6796 = (v1 + (v2914 * v6598)) - v25;
            let v6798 = if v6796 < v6797 { 1.0 } else { 0.0 };
            let v6807: f64;
            let v14626: Lanes<1>;
            if v6798 != 0.0 {
                let v6800 = v6799 / v6796;
                let v14986 = ((v14976 * v6800) * v14888) / v6796;
                v6807 = v6800;
                v14626 = v14986;
            } else {
                let v14977 = v14976 * v6796;
                let v6804 = ((v6796 * v6796) + v6802).sqrt();
                let v6806 = v4897 * (v6796 + v6804);
                let v14983 = (v14976 + ((v14977 + v14977) * (v14600 / (v14869 * v6804)))) * v4897;
                v6807 = v6806;
                v14626 = v14983;
            }
            let v6808 = v6600 * v415;
            let v14987 = v14866 * v415;
            let v6811 = (v23 * v399) / v388;
            let v6812 = v6809 / v6811;
            let v6813 = v6812 * v6812;
            let v6815 = v6813 / v6814;
            let v6817 = v6813 / v6816;
            let v6822 = (v6815 - v6817) / v6808;
            let v6827 = (v6815 - (v22 * v6815)) / v6808;
            let v6831 = (v6815 - (v22 * v6817)) / v6808;
            let v6834 = ((v1 + (v6820 * (rspice_limited_exp(v6822)))) + (rspice_limited_exp(v6827))) + (v6820 * (rspice_limited_exp(v6831)));
            let v6838 = v6836 * v6837;
            let v6839 = v6835 / v6838;
            let v6841 = (v6839 * v6808) / v6811;
            let v6842 = v6841 * v6834;
            let v15017 = ((((((((v14623 * v6836) * v6839) * v14888) / v6838) * v6808) + (v14987 * v6839)) / v6811) * v6834) + ((((((((v14987 * v6822) * v14888) / v6808) * (rspice_limited_exp_derivative(v6822))) * v6820) + ((((v14987 * v6827) * v14888) / v6808) * (rspice_limited_exp_derivative(v6827)))) + (((((v14987 * v6831) * v14888) / v6808) * (rspice_limited_exp_derivative(v6831))) * v6820)) * v6841);
            let v6843 = if v6842 > v185 { 1.0 } else { 0.0 };
            let v6847: f64;
            let v14627: Lanes<1>;
            if v6843 != 0.0 {
                let v6844 = v6842.ln();
                let v15019 = v15017 * (v14600 / v6842);
                v6847 = v6844;
                v14627 = v15019;
            } else {
                v6847 = v6845;
                v14627 = v14864;
            }
            let v6846 = -v6600;
            let v6851 = v850 * ((v6815 / v415) + (v6846 * v6847));
            let v15024 = (((v14866 * v14888) * v6847) + (v14627 * v6846)) * v850;
            let v6852 = v6596.ln();
            let v15026 = v14865 * (v14600 / v6596);
            let v8792: f64;
            let v8798: f64;
            let v8804: f64;
            let v10394: f64;
            let v10400: f64;
            let v10410: f64;
            let v10439: f64;
            let v10445: f64;
            let v10462: f64;
            let v10476: f64;
            let v10482: f64;
            let v10486: f64;
            let v10492: f64;
            let v10496: f64;
            let v10502: f64;
            let v10520: f64;
            let v10619: f64;
            let v10940: f64;
            let v10986: f64;
            let v11302: f64;
            let v11603: f64;
            let v11617: f64;
            let v12161: f64;
            let v12164: f64;
            let v14628: Lanes<1>;
            let v14629: Lanes<1>;
            let v14630: Lanes<1>;
            let v14631: Lanes<1>;
            let v14632: Lanes<1>;
            let v14633: Lanes<1>;
            let v14634: Lanes<1>;
            let v14635: Lanes<1>;
            let v14636: Lanes<1>;
            let v14637: Lanes<1>;
            let v14638: Lanes<1>;
            let v14639: Lanes<1>;
            let v14640: Lanes<1>;
            let v14641: Lanes<1>;
            let v14642: Lanes<1>;
            let v14643: Lanes<1>;
            let v14644: Lanes<1>;
            let v14645: Lanes<1>;
            let v14646: Lanes<1>;
            let v14647: Lanes<1>;
            if v6778 != 0.0 {
                let v6855 = (v2610 * v6852).exp();
                let v6856 = v6853 * v6855;
                let v15921 = ((v15026 * v2610) * v6855) * v6853;
                let v6860 = v2642 * v6598;
                let v15923 = v14606 * v2642;
                let v6868 = (v6860 - (v6865 * v6856)) - v4767;
                let v6872 = (v6860 - (v6869 * v6856)) - v4767;
                let v6879 = ((v6868 * v6872) - ((v22 * (v6874 * v6856)) * v4767)).sqrt();
                let v6883 = v6856 + ((v6858 * v6856) + (v4897 * (((v6860 - (v6861 * v6856)) - v4767) + v6879)));
                let v15943 = v15921 + ((v15921 * v6858) + (((v15923 - (v15921 * v6861)) + (((((v15923 - (v15921 * v6865)) * v6872) + ((v15923 - (v15921 * v6869)) * v6868)) - (((v15921 * v6874) * v22) * v4767)) * (v14600 / (v14869 * v6879)))) * v4897));
                let v6884 = if v3341 == v1 { 1.0 } else { 0.0 };
                let v10477: f64;
                let v14648: Lanes<1>;
                if v6884 != 0.0 {
                    let v6889 = (v6887 * v6852).exp();
                    let v6890 = v6885 * v6889;
                    let v15946 = ((v15026 * v6887) * v6889) * v6885;
                    let v6894 = v6893 * v6598;
                    let v15948 = v14606 * v6893;
                    let v6902 = (v6894 - (v6899 * v6890)) - v4767;
                    let v6906 = (v6894 - (v6903 * v6890)) - v4767;
                    let v6913 = ((v6902 * v6906) - ((v22 * (v6908 * v6890)) * v4767)).sqrt();
                    let v6917 = v6890 + ((v6891 * v6890) + (v4897 * (((v6894 - (v6895 * v6890)) - v4767) + v6913)));
                    let v15968 = v15946 + ((v15946 * v6891) + (((v15948 - (v15946 * v6895)) + (((((v15948 - (v15946 * v6899)) * v6906) + ((v15948 - (v15946 * v6903)) * v6902)) - (((v15946 * v6908) * v22) * v4767)) * (v14600 / (v14869 * v6913)))) * v4897));
                    v10477 = v6917;
                    v14648 = v15968;
                } else {
                    v10477 = v0;
                    v14648 = v14864;
                }
                let v6919 = -v6918;
                let v15969 = v14606 * v2674;
                let v6922 = ((v2674 * v6598) - v6919) - v25;
                let v15970 = v15969 * v6922;
                let v6927 = ((v6922 * v6922) - ((v22 * v6919) * v25)).sqrt();
                let v15976 = (v15969 + ((v15970 + v15970) * (v14600 / (v14869 * v6927)))) * v4897;
                let v6931 = v6918 + (v6919 + (v4897 * (v6922 + v6927)));
                let v10487: f64;
                let v14649: Lanes<1>;
                if v3342 != 0.0 {
                    let v6935 = -v6933;
                    let v15977 = v14606 * v6936;
                    let v6939 = ((v6936 * v6598) - v6935) - v25;
                    let v15978 = v15977 * v6939;
                    let v6944 = ((v6939 * v6939) - ((v22 * v6935) * v25)).sqrt();
                    let v15984 = (v15977 + ((v15978 + v15978) * (v14600 / (v14869 * v6944)))) * v4897;
                    let v6948 = v6933 + (v6935 + (v4897 * (v6939 + v6944)));
                    v10487 = v6948;
                    v14649 = v15984;
                } else {
                    v10487 = v0;
                    v14649 = v14864;
                }
                let v6951 = (v2722 * v6852).exp();
                let v6952 = v6949 * v6951;
                let v15987 = ((v15026 * v2722) * v6951) * v6949;
                let v10497: f64;
                let v14650: Lanes<1>;
                if v3342 != 0.0 {
                    let v6957 = (v6955 * v6852).exp();
                    let v6958 = v6953 * v6957;
                    let v15990 = ((v15026 * v6955) * v6957) * v6953;
                    v10497 = v6958;
                    v14650 = v15990;
                } else {
                    v10497 = v0;
                    v14650 = v14864;
                }
                let v6961 = (v2754 * v6852).exp();
                let v6962 = v6959 * v6961;
                let v15993 = ((v15026 * v2754) * v6961) * v6959;
                let v15994 = v14606 * v2834;
                let v6965 = (v1 + (v2834 * v6598)) - v25;
                let v6967 = if v6965 < v6966 { 1.0 } else { 0.0 };
                let v6976: f64;
                let v14651: Lanes<1>;
                if v6967 != 0.0 {
                    let v6969 = v6968 / v6965;
                    let v16004 = ((v15994 * v6969) * v14888) / v6965;
                    v6976 = v6969;
                    v14651 = v16004;
                } else {
                    let v15995 = v15994 * v6965;
                    let v6973 = ((v6965 * v6965) + v6971).sqrt();
                    let v6975 = v4897 * (v6965 + v6973);
                    let v16001 = (v15994 + ((v15995 + v15995) * (v14600 / (v14869 * v6973)))) * v4897;
                    v6976 = v6975;
                    v14651 = v16001;
                }
                let v6978 = if v6977 != v0 { 1.0 } else { 0.0 };
                let v8793: f64;
                let v14652: Lanes<1>;
                if v6978 != 0.0 {
                    let v6980 = -v6979;
                    let v6981 = -v2802;
                    let v16017 = v14606 * v6981;
                    let v6984 = ((v6981 * v6598) - v6980) - v25;
                    let v16018 = v16017 * v6984;
                    let v6989 = ((v6984 * v6984) - ((v22 * v6980) * v25)).sqrt();
                    let v16024 = (v16017 + ((v16018 + v16018) * (v14600 / (v14869 * v6989)))) * v4897;
                    let v6993 = v6979 + (v6980 + (v4897 * (v6984 + v6989)));
                    v8793 = v6993;
                    v14652 = v16024;
                } else {
                    let v6994 = -v2802;
                    let v16005 = v14606 * v6994;
                    let v6997 = (v1 + (v6994 * v6598)) - v25;
                    let v6999 = if v6997 < v6998 { 1.0 } else { 0.0 };
                    let v7008: f64;
                    let v14653: Lanes<1>;
                    if v6999 != 0.0 {
                        let v7001 = v7000 / v6997;
                        let v16015 = ((v16005 * v7001) * v14888) / v6997;
                        v7008 = v7001;
                        v14653 = v16015;
                    } else {
                        let v16006 = v16005 * v6997;
                        let v7005 = ((v6997 * v6997) + v7003).sqrt();
                        let v7007 = v4897 * (v6997 + v7005);
                        let v16012 = (v16005 + ((v16006 + v16006) * (v14600 / (v14869 * v7005)))) * v4897;
                        v7008 = v7007;
                        v14653 = v16012;
                    }
                    let v7009 = v6979 * v7008;
                    let v16016 = v14653 * v6979;
                    v8793 = v7009;
                    v14652 = v16016;
                }
                let v10446: f64;
                let v14654: Lanes<1>;
                if v3342 != 0.0 {
                    let v7042: f64;
                    let v14655: Lanes<1>;
                    if v6978 != 0.0 {
                        let v7011 = -v7010;
                        let v7013 = -v7012;
                        let v16037 = v14606 * v7013;
                        let v7016 = ((v7013 * v6598) - v7011) - v25;
                        let v16038 = v16037 * v7016;
                        let v7021 = ((v7016 * v7016) - ((v22 * v7011) * v25)).sqrt();
                        let v16044 = (v16037 + ((v16038 + v16038) * (v14600 / (v14869 * v7021)))) * v4897;
                        let v7025 = v7010 + (v7011 + (v4897 * (v7016 + v7021)));
                        v7042 = v7025;
                        v14655 = v16044;
                    } else {
                        let v7026 = -v7012;
                        let v16025 = v14606 * v7026;
                        let v7029 = (v1 + (v7026 * v6598)) - v25;
                        let v7031 = if v7029 < v7030 { 1.0 } else { 0.0 };
                        let v7040: f64;
                        let v14656: Lanes<1>;
                        if v7031 != 0.0 {
                            let v7033 = v7032 / v7029;
                            let v16035 = ((v16025 * v7033) * v14888) / v7029;
                            v7040 = v7033;
                            v14656 = v16035;
                        } else {
                            let v16026 = v16025 * v7029;
                            let v7037 = ((v7029 * v7029) + v7035).sqrt();
                            let v7039 = v4897 * (v7029 + v7037);
                            let v16032 = (v16025 + ((v16026 + v16026) * (v14600 / (v14869 * v7037)))) * v4897;
                            v7040 = v7039;
                            v14656 = v16032;
                        }
                        let v7041 = v7010 * v7040;
                        let v16036 = v14656 * v7010;
                        v7042 = v7041;
                        v14655 = v16036;
                    }
                    let v7044 = if v7042 < v7043 { 1.0 } else { 0.0 };
                    let v10447: f64;
                    let v14657: Lanes<1>;
                    if v7044 != 0.0 {
                        v10447 = v7043;
                        v14657 = v14864;
                    } else {
                        v10447 = v7042;
                        v14657 = v14655;
                    }
                    v10446 = v10447;
                    v14654 = v14657;
                } else {
                    v10446 = v0;
                    v14654 = v14864;
                }
                let v11303: f64;
                let v11604: f64;
                let v11618: f64;
                if v3728 != 0.0 {
                    let v7052 = v7045 * ((v7048 * v6852).exp());
                    let v7057 = v7055 * v6598;
                    let v7080 = v7052 + ((v7053 * v7052) + (v4897 * (((v7057 - (v7058 * v7052)) - v4767) + (((((v7057 - (v7062 * v7052)) - v4767) * ((v7057 - (v7066 * v7052)) - v4767)) - ((v22 * (v7071 * v7052)) * v4767)).sqrt()))));
                    let v7083 = -v7081;
                    let v7088 = ((v7084 * v6598) - v7083) - v25;
                    let v7097 = v7081 + (v7083 + (v4897 * (v7088 + (((v7088 * v7088) - ((v22 * v7083) * v25)).sqrt()))));
                    let v7104 = v7098 * ((v7100 * v6852).exp());
                    v11303 = v7080;
                    v11604 = v7097;
                    v11618 = v7104;
                } else {
                    v11303 = v11304;
                    v11604 = v11605;
                    v11618 = v11619;
                }
                let v8799: f64;
                let v14658: Lanes<1>;
                if v6978 != 0.0 {
                    let v7106 = -v7105;
                    let v7107 = -v2802;
                    let v16057 = v14606 * v7107;
                    let v7110 = ((v7107 * v6598) - v7106) - v25;
                    let v16058 = v16057 * v7110;
                    let v7115 = ((v7110 * v7110) - ((v22 * v7106) * v25)).sqrt();
                    let v16064 = (v16057 + ((v16058 + v16058) * (v14600 / (v14869 * v7115)))) * v4897;
                    let v7119 = v7105 + (v7106 + (v4897 * (v7110 + v7115)));
                    v8799 = v7119;
                    v14658 = v16064;
                } else {
                    let v7120 = -v2802;
                    let v16045 = v14606 * v7120;
                    let v7123 = (v1 + (v7120 * v6598)) - v25;
                    let v7125 = if v7123 < v7124 { 1.0 } else { 0.0 };
                    let v7134: f64;
                    let v14659: Lanes<1>;
                    if v7125 != 0.0 {
                        let v7127 = v7126 / v7123;
                        let v16055 = ((v16045 * v7127) * v14888) / v7123;
                        v7134 = v7127;
                        v14659 = v16055;
                    } else {
                        let v16046 = v16045 * v7123;
                        let v7131 = ((v7123 * v7123) + v7129).sqrt();
                        let v7133 = v4897 * (v7123 + v7131);
                        let v16052 = (v16045 + ((v16046 + v16046) * (v14600 / (v14869 * v7131)))) * v4897;
                        v7134 = v7133;
                        v14659 = v16052;
                    }
                    let v7135 = v7105 * v7134;
                    let v16056 = v14659 * v7105;
                    v8799 = v7135;
                    v14658 = v16056;
                }
                let v10411: f64;
                let v14660: Lanes<1>;
                if v3342 != 0.0 {
                    let v7167: f64;
                    let v14661: Lanes<1>;
                    if v6978 != 0.0 {
                        let v7137 = -v7136;
                        let v7138 = -v2802;
                        let v16077 = v14606 * v7138;
                        let v7141 = ((v7138 * v6598) - v7137) - v25;
                        let v16078 = v16077 * v7141;
                        let v7146 = ((v7141 * v7141) - ((v22 * v7137) * v25)).sqrt();
                        let v16084 = (v16077 + ((v16078 + v16078) * (v14600 / (v14869 * v7146)))) * v4897;
                        let v7150 = v7136 + (v7137 + (v4897 * (v7141 + v7146)));
                        v7167 = v7150;
                        v14661 = v16084;
                    } else {
                        let v7151 = -v2802;
                        let v16065 = v14606 * v7151;
                        let v7154 = (v1 + (v7151 * v6598)) - v25;
                        let v7156 = if v7154 < v7155 { 1.0 } else { 0.0 };
                        let v7165: f64;
                        let v14662: Lanes<1>;
                        if v7156 != 0.0 {
                            let v7158 = v7157 / v7154;
                            let v16075 = ((v16065 * v7158) * v14888) / v7154;
                            v7165 = v7158;
                            v14662 = v16075;
                        } else {
                            let v16066 = v16065 * v7154;
                            let v7162 = ((v7154 * v7154) + v7160).sqrt();
                            let v7164 = v4897 * (v7154 + v7162);
                            let v16072 = (v16065 + ((v16066 + v16066) * (v14600 / (v14869 * v7162)))) * v4897;
                            v7165 = v7164;
                            v14662 = v16072;
                        }
                        let v7166 = v7136 * v7165;
                        let v16076 = v14662 * v7136;
                        v7167 = v7166;
                        v14661 = v16076;
                    }
                    let v7168 = if v7167 < v7043 { 1.0 } else { 0.0 };
                    let v10412: f64;
                    let v14663: Lanes<1>;
                    if v7168 != 0.0 {
                        v10412 = v7043;
                        v14663 = v14864;
                    } else {
                        v10412 = v7167;
                        v14663 = v14661;
                    }
                    v10411 = v10412;
                    v14660 = v14663;
                } else {
                    v10411 = v0;
                    v14660 = v14864;
                }
                let v8805: f64;
                if v6978 != 0.0 {
                    let v7169 = -v4493;
                    let v7173 = (((-v2818) * v6598) - v7169) - v25;
                    let v7182 = v4493 + (v7169 + (v4897 * (v7173 + (((v7173 * v7173) - ((v22 * v7169) * v25)).sqrt()))));
                    v8805 = v7182;
                } else {
                    let v7186 = (v1 + ((-v2818) * v6598)) - v25;
                    let v7188 = if v7186 < v7187 { 1.0 } else { 0.0 };
                    let v7197: f64;
                    if v7188 != 0.0 {
                        let v7190 = v7189 / v7186;
                        v7197 = v7190;
                    } else {
                        let v7196 = v4897 * (v7186 + (((v7186 * v7186) + v7192).sqrt()));
                        v7197 = v7196;
                    }
                    let v7198 = v4493 * v7197;
                    v8805 = v7198;
                }
                let v16086 = (v14606 * v7200) * v7199;
                let v7204 = (v7199 * (v1 + (v7200 * v6598))) - v23;
                let v7206 = if v7204 < v7205 { 1.0 } else { 0.0 };
                let v7215: f64;
                let v14664: Lanes<1>;
                if v7206 != 0.0 {
                    let v7208 = v7207 / v7204;
                    let v16096 = ((v16086 * v7208) * v14888) / v7204;
                    v7215 = v7208;
                    v14664 = v16096;
                } else {
                    let v16087 = v16086 * v7204;
                    let v7212 = ((v7204 * v7204) + v7210).sqrt();
                    let v7214 = v4897 * (v7204 + v7212);
                    let v16093 = (v16086 + ((v16087 + v16087) * (v14600 / (v14869 * v7212)))) * v4897;
                    v7215 = v7214;
                    v14664 = v16093;
                }
                let v7216 = v7215 + v23;
                let v10395: f64;
                let v14665: Lanes<1>;
                if v3342 != 0.0 {
                    let v16098 = (v14606 * v7219) * v7217;
                    let v7223 = (v7217 * (v1 + (v7219 * v6598))) - v23;
                    let v7225 = if v7223 < v7224 { 1.0 } else { 0.0 };
                    let v7234: f64;
                    let v14666: Lanes<1>;
                    if v7225 != 0.0 {
                        let v7227 = v7226 / v7223;
                        let v16108 = ((v16098 * v7227) * v14888) / v7223;
                        v7234 = v7227;
                        v14666 = v16108;
                    } else {
                        let v16099 = v16098 * v7223;
                        let v7231 = ((v7223 * v7223) + v7229).sqrt();
                        let v7233 = v4897 * (v7223 + v7231);
                        let v16105 = (v16098 + ((v16099 + v16099) * (v14600 / (v14869 * v7231)))) * v4897;
                        v7234 = v7233;
                        v14666 = v16105;
                    }
                    let v7235 = v7234 + v23;
                    v10395 = v7235;
                    v14665 = v14666;
                } else {
                    v10395 = v0;
                    v14665 = v14864;
                }
                let v7238 = v2898 + (v7236 / v3969);
                let v7239 = v7238 * v6597;
                let v16109 = v14865 * v7238;
                v8792 = v8793;
                v8798 = v8799;
                v8804 = v8805;
                v10394 = v10395;
                v10400 = v7216;
                v10410 = v10411;
                v10439 = v4426;
                v10445 = v10446;
                v10462 = v1042;
                v10476 = v10477;
                v10482 = v6883;
                v10486 = v10487;
                v10492 = v6931;
                v10496 = v10497;
                v10502 = v6952;
                v10520 = v6932;
                v10619 = v7239;
                v10940 = v6962;
                v10986 = v6976;
                v11302 = v11303;
                v11603 = v11604;
                v11617 = v11618;
                v12161 = v0;
                v12164 = v0;
                v14628 = v14652;
                v14629 = v14658;
                v14630 = v14665;
                v14631 = v14664;
                v14632 = v14660;
                v14633 = v14864;
                v14634 = v14654;
                v14635 = v14864;
                v14636 = v14648;
                v14637 = v15943;
                v14638 = v14649;
                v14639 = v15976;
                v14640 = v14650;
                v14641 = v15987;
                v14642 = v14864;
                v14643 = v16109;
                v14644 = v15993;
                v14645 = v14651;
                v14646 = v14864;
                v14647 = v14864;
            } else {
                let v7240 = if v6603 == v1 { 1.0 } else { 0.0 };
                let v8794: f64;
                let v8800: f64;
                let v8806: f64;
                let v10396: f64;
                let v10401: f64;
                let v10413: f64;
                let v10440: f64;
                let v10448: f64;
                let v10463: f64;
                let v10478: f64;
                let v10483: f64;
                let v10488: f64;
                let v10493: f64;
                let v10498: f64;
                let v10503: f64;
                let v10521: f64;
                let v10620: f64;
                let v10941: f64;
                let v10987: f64;
                let v11305: f64;
                let v11606: f64;
                let v11620: f64;
                let v12162: f64;
                let v12165: f64;
                let v14667: Lanes<1>;
                let v14668: Lanes<1>;
                let v14669: Lanes<1>;
                let v14670: Lanes<1>;
                let v14671: Lanes<1>;
                let v14672: Lanes<1>;
                let v14673: Lanes<1>;
                let v14674: Lanes<1>;
                let v14675: Lanes<1>;
                let v14676: Lanes<1>;
                let v14677: Lanes<1>;
                let v14678: Lanes<1>;
                let v14679: Lanes<1>;
                let v14680: Lanes<1>;
                let v14681: Lanes<1>;
                let v14682: Lanes<1>;
                let v14683: Lanes<1>;
                let v14684: Lanes<1>;
                let v14685: Lanes<1>;
                let v14686: Lanes<1>;
                if v7240 != 0.0 {
                    let v7241 = v2626 * v6596;
                    let v7242 = v2610 + v7241;
                    let v15553 = (v14865 * v2626) * v6852;
                    let v7244 = (v7242 * v6852).exp();
                    let v7245 = v6853 * v7244;
                    let v15557 = ((v15553 + (v15026 * v7242)) * v7244) * v6853;
                    let v7248 = v2642 * v6598;
                    let v15559 = v14606 * v2642;
                    let v7256 = (v7248 - (v7253 * v7245)) - v4767;
                    let v7260 = (v7248 - (v7257 * v7245)) - v4767;
                    let v7267 = ((v7256 * v7260) - ((v22 * (v7262 * v7245)) * v4767)).sqrt();
                    let v7271 = v7245 + ((v7246 * v7245) + (v4897 * (((v7248 - (v7249 * v7245)) - v4767) + v7267)));
                    let v15579 = v15557 + ((v15557 * v7246) + (((v15559 - (v15557 * v7249)) + (((((v15559 - (v15557 * v7253)) * v7260) + ((v15559 - (v15557 * v7257)) * v7256)) - (((v15557 * v7262) * v22) * v4767)) * (v14600 / (v14869 * v7267)))) * v4897));
                    let v7272 = if v3341 == v1 { 1.0 } else { 0.0 };
                    let v10479: f64;
                    let v14687: Lanes<1>;
                    if v7272 != 0.0 {
                        let v7273 = v6887 + v7241;
                        let v7275 = (v7273 * v6852).exp();
                        let v7276 = v6885 * v7275;
                        let v15583 = ((v15553 + (v15026 * v7273)) * v7275) * v6885;
                        let v7279 = v6893 * v6598;
                        let v15585 = v14606 * v6893;
                        let v7287 = (v7279 - (v7284 * v7276)) - v4767;
                        let v7291 = (v7279 - (v7288 * v7276)) - v4767;
                        let v7298 = ((v7287 * v7291) - ((v22 * (v7293 * v7276)) * v4767)).sqrt();
                        let v7302 = v7276 + ((v7277 * v7276) + (v4897 * (((v7279 - (v7280 * v7276)) - v4767) + v7298)));
                        let v15605 = v15583 + ((v15583 * v7277) + (((v15585 - (v15583 * v7280)) + (((((v15585 - (v15583 * v7284)) * v7291) + ((v15585 - (v15583 * v7288)) * v7287)) - (((v15583 * v7293) * v22) * v4767)) * (v14600 / (v14869 * v7298)))) * v4897));
                        v10479 = v7302;
                        v14687 = v15605;
                    } else {
                        v10479 = v0;
                        v14687 = v14864;
                    }
                    let v7303 = v2690 * v6596;
                    let v7304 = v2674 + v7303;
                    let v15607 = (v14865 * v2690) * v6852;
                    let v7306 = (v7304 * v6852).exp();
                    let v7307 = v6918 * v7306;
                    let v15611 = ((v15607 + (v15026 * v7304)) * v7306) * v6918;
                    let v10489: f64;
                    let v14688: Lanes<1>;
                    if v3342 != 0.0 {
                        let v7308 = v6936 + v7303;
                        let v7310 = (v7308 * v6852).exp();
                        let v7311 = v6933 * v7310;
                        let v15615 = ((v15607 + (v15026 * v7308)) * v7310) * v6933;
                        v10489 = v7311;
                        v14688 = v15615;
                    } else {
                        v10489 = v0;
                        v14688 = v14864;
                    }
                    let v7312 = v2738 * v6596;
                    let v7313 = v2722 + v7312;
                    let v15617 = (v14865 * v2738) * v6852;
                    let v7315 = (v7313 * v6852).exp();
                    let v7316 = v6949 * v7315;
                    let v15621 = ((v15617 + (v15026 * v7313)) * v7315) * v6949;
                    let v10499: f64;
                    let v14689: Lanes<1>;
                    if v3342 != 0.0 {
                        let v7317 = v6955 + v7312;
                        let v7319 = (v7317 * v6852).exp();
                        let v7320 = v6953 * v7319;
                        let v15625 = ((v15617 + (v15026 * v7317)) * v7319) * v6953;
                        v10499 = v7320;
                        v14689 = v15625;
                    } else {
                        v10499 = v0;
                        v14689 = v14864;
                    }
                    let v7322 = v2754 + (v2755 * v6596);
                    let v7324 = (v7322 * v6852).exp();
                    let v7325 = v6959 * v7324;
                    let v15631 = ((((v14865 * v2755) * v6852) + (v15026 * v7322)) * v7324) * v6959;
                    let v7326 = v1218 * v6597;
                    let v15635 = ((v14865 * v1218) * (rspice_limited_exp_derivative(v7326))) * v1202;
                    let v7330 = v1250 * v6597;
                    let v15639 = ((v14865 * v1250) * (rspice_limited_exp_derivative(v7330))) * v1234;
                    let v7334 = v4897 + (v1202 * ((rspice_limited_exp(v7326)) - v1));
                    let v7335 = v4897 + (v1234 * ((rspice_limited_exp(v7330)) - v1));
                    let v7336 = if v6977 != v0 { 1.0 } else { 0.0 };
                    let v10522: f64;
                    let v14690: Lanes<1>;
                    if v7336 != 0.0 {
                        let v7337 = -v6932;
                        let v15652 = v14606 * v2706;
                        let v7340 = ((v2706 * v6598) - v7337) - v25;
                        let v15653 = v15652 * v7340;
                        let v7345 = ((v7340 * v7340) - ((v22 * v7337) * v25)).sqrt();
                        let v15659 = (v15652 + ((v15653 + v15653) * (v14600 / (v14869 * v7345)))) * v4897;
                        let v7349 = v6932 + (v7337 + (v4897 * (v7340 + v7345)));
                        v10522 = v7349;
                        v14690 = v15659;
                    } else {
                        let v15640 = v14606 * v2706;
                        let v7352 = (v1 + (v2706 * v6598)) - v25;
                        let v7354 = if v7352 < v7353 { 1.0 } else { 0.0 };
                        let v7363: f64;
                        let v14691: Lanes<1>;
                        if v7354 != 0.0 {
                            let v7356 = v7355 / v7352;
                            let v15650 = ((v15640 * v7356) * v14888) / v7352;
                            v7363 = v7356;
                            v14691 = v15650;
                        } else {
                            let v15641 = v15640 * v7352;
                            let v7360 = ((v7352 * v7352) + v7358).sqrt();
                            let v7362 = v4897 * (v7352 + v7360);
                            let v15647 = (v15640 + ((v15641 + v15641) * (v14600 / (v14869 * v7360)))) * v4897;
                            v7363 = v7362;
                            v14691 = v15647;
                        }
                        let v7364 = v6932 * v7363;
                        let v15651 = v14691 * v6932;
                        v10522 = v7364;
                        v14690 = v15651;
                    }
                    let v11306: f64;
                    let v11607: f64;
                    let v11621: f64;
                    if v3728 != 0.0 {
                        let v7371 = v7045 * (((v7048 + (v7365 * v6596)) * v6852).exp());
                        let v7374 = v7055 * v6598;
                        let v7397 = v7371 + ((v7372 * v7371) + (v4897 * (((v7374 - (v7375 * v7371)) - v4767) + (((((v7374 - (v7379 * v7371)) - v4767) * ((v7374 - (v7383 * v7371)) - v4767)) - ((v22 * (v7388 * v7371)) * v4767)).sqrt()))));
                        let v7404 = v7081 * (((v7084 + (v7398 * v6596)) * v6852).exp());
                        let v7411 = v7098 * (((v7100 + (v7405 * v6596)) * v6852).exp());
                        v11306 = v7397;
                        v11607 = v7404;
                        v11621 = v7411;
                    } else {
                        v11306 = v11304;
                        v11607 = v11605;
                        v11621 = v11619;
                    }
                    let v7412 = if v2834 == v2850 { 1.0 } else { 0.0 };
                    let v7499: f64;
                    let v14692: Lanes<1>;
                    if v7412 != 0.0 {
                        let v15700 = v14606 * v2834;
                        let v7414 = v1 + (v2834 * v6598);
                        v7499 = v7414;
                        v14692 = v15700;
                    } else {
                        let v7415 = if v2866 < v6595 { 1.0 } else { 0.0 };
                        let v7500: f64;
                        let v14693: Lanes<1>;
                        if v7415 != 0.0 {
                            let v15680 = v14606 * v2834;
                            let v7417 = v1 + (v2834 * v6598);
                            let v15681 = v14606 * v2850;
                            let v7421 = v2866 - v6595;
                            let v7423 = (v1 + (v2850 * (v6594 - v2866))) + (v2834 * v7421);
                            let v7425 = (v2834 - v2850) * v7421;
                            let v7426 = if v2850 < v2834 { 1.0 } else { 0.0 };
                            let v7501: f64;
                            let v14694: Lanes<1>;
                            if v7426 != 0.0 {
                                let v7428 = v7417 - v7423;
                                let v15693 = (v15680 - v15681) * v7428;
                                let v7432 = (v5721 * v7430) * v7430;
                                let v7434 = ((v7428 * v7428) + v7432).sqrt();
                                let v15699 = ((v15680 + v15681) + ((v15693 + v15693) * (v14600 / (v14869 * v7434)))) * v4897;
                                let v7442 = (v4897 * ((v7417 + v7423) + v7434)) - (v4897 * (v7425 + (((v7425 * v7425) + v7432).sqrt())));
                                v7501 = v7442;
                                v14694 = v15699;
                            } else {
                                let v7444 = v7417 - v7423;
                                let v15684 = (v15680 - v15681) * v7444;
                                let v7447 = (v5721 * v7430) * v7430;
                                let v7449 = ((v7444 * v7444) + v7447).sqrt();
                                let v15690 = ((v15680 + v15681) - ((v15684 + v15684) * (v14600 / (v14869 * v7449)))) * v4897;
                                let v7457 = (v4897 * ((v7417 + v7423) - v7449)) - (v4897 * (v7425 - (((v7425 * v7425) + v7447).sqrt())));
                                v7501 = v7457;
                                v14694 = v15690;
                            }
                            v7500 = v7501;
                            v14693 = v14694;
                        } else {
                            let v15660 = v14606 * v2850;
                            let v7459 = v1 + (v2850 * v6598);
                            let v15661 = v14606 * v2834;
                            let v7463 = v2866 - v6595;
                            let v7465 = (v1 + (v2834 * (v6594 - v2866))) + (v2850 * v7463);
                            let v7467 = (v2850 - v2834) * v7463;
                            let v7468 = if v2850 < v2834 { 1.0 } else { 0.0 };
                            let v7502: f64;
                            let v14695: Lanes<1>;
                            if v7468 != 0.0 {
                                let v7470 = v7459 - v7465;
                                let v15673 = (v15660 - v15661) * v7470;
                                let v7473 = (v5721 * v7430) * v7430;
                                let v7475 = ((v7470 * v7470) + v7473).sqrt();
                                let v15679 = ((v15660 + v15661) + ((v15673 + v15673) * (v14600 / (v14869 * v7475)))) * v4897;
                                let v7483 = (v4897 * ((v7459 + v7465) + v7475)) - (v4897 * (v7467 + (((v7467 * v7467) + v7473).sqrt())));
                                v7502 = v7483;
                                v14695 = v15679;
                            } else {
                                let v7485 = v7459 - v7465;
                                let v15664 = (v15660 - v15661) * v7485;
                                let v7488 = (v5721 * v7430) * v7430;
                                let v7490 = ((v7485 * v7485) + v7488).sqrt();
                                let v15670 = ((v15660 + v15661) - ((v15664 + v15664) * (v14600 / (v14869 * v7490)))) * v4897;
                                let v7498 = (v4897 * ((v7459 + v7465) - v7490)) - (v4897 * (v7467 - (((v7467 * v7467) + v7488).sqrt())));
                                v7502 = v7498;
                                v14695 = v15670;
                            }
                            v7500 = v7502;
                            v14693 = v14695;
                        }
                        v7499 = v7500;
                        v14692 = v14693;
                    }
                    let v7503 = v7499 - v25;
                    let v7505 = if v7503 < v7504 { 1.0 } else { 0.0 };
                    let v7514: f64;
                    let v14696: Lanes<1>;
                    if v7505 != 0.0 {
                        let v7507 = v7506 / v7503;
                        let v15710 = ((v14692 * v7507) * v14888) / v7503;
                        v7514 = v7507;
                        v14696 = v15710;
                    } else {
                        let v15701 = v14692 * v7503;
                        let v7511 = ((v7503 * v7503) + v7509).sqrt();
                        let v7513 = v4897 * (v7503 + v7511);
                        let v15707 = (v14692 + ((v15701 + v15701) * (v14600 / (v14869 * v7511)))) * v4897;
                        v7514 = v7513;
                        v14696 = v15707;
                    }
                    let v8795: f64;
                    let v14697: Lanes<1>;
                    if v7336 != 0.0 {
                        let v7515 = -v6979;
                        let v7516 = -v2802;
                        let v7519 = v7518 * v6598;
                        let v15733 = (v14606 * v7516) + (((v14606 * v7518) * v6598) + (v14606 * v7519));
                        let v7523 = (((v7516 * v6598) + (v7519 * v6598)) - v7515) - v25;
                        let v15734 = v15733 * v7523;
                        let v7528 = ((v7523 * v7523) - ((v22 * v7515) * v25)).sqrt();
                        let v15740 = (v15733 + ((v15734 + v15734) * (v14600 / (v14869 * v7528)))) * v4897;
                        let v7532 = v6979 + (v7515 + (v4897 * (v7523 + v7528)));
                        v8795 = v7532;
                        v14697 = v15740;
                    } else {
                        let v7533 = -v2802;
                        let v7536 = v7518 * v6598;
                        let v15716 = (v14606 * v7533) + (((v14606 * v7518) * v6598) + (v14606 * v7536));
                        let v7539 = ((v1 + (v7533 * v6598)) + (v7536 * v6598)) - v25;
                        let v7541 = if v7539 < v7540 { 1.0 } else { 0.0 };
                        let v7550: f64;
                        let v14698: Lanes<1>;
                        if v7541 != 0.0 {
                            let v7543 = v7542 / v7539;
                            let v15726 = ((v15716 * v7543) * v14888) / v7539;
                            v7550 = v7543;
                            v14698 = v15726;
                        } else {
                            let v15717 = v15716 * v7539;
                            let v7547 = ((v7539 * v7539) + v7545).sqrt();
                            let v7549 = v4897 * (v7539 + v7547);
                            let v15723 = (v15716 + ((v15717 + v15717) * (v14600 / (v14869 * v7547)))) * v4897;
                            v7550 = v7549;
                            v14698 = v15723;
                        }
                        let v7551 = v6979 * v7550;
                        let v15727 = v14698 * v6979;
                        v8795 = v7551;
                        v14697 = v15727;
                    }
                    let v10449: f64;
                    let v14699: Lanes<1>;
                    if v3342 != 0.0 {
                        let v7588: f64;
                        let v14700: Lanes<1>;
                        if v7336 != 0.0 {
                            let v7552 = -v7010;
                            let v7553 = -v7012;
                            let v7555 = v7518 * v6598;
                            let v15763 = (v14606 * v7553) + (((v14606 * v7518) * v6598) + (v14606 * v7555));
                            let v7559 = (((v7553 * v6598) + (v7555 * v6598)) - v7552) - v25;
                            let v15764 = v15763 * v7559;
                            let v7564 = ((v7559 * v7559) - ((v22 * v7552) * v25)).sqrt();
                            let v15770 = (v15763 + ((v15764 + v15764) * (v14600 / (v14869 * v7564)))) * v4897;
                            let v7568 = v7010 + (v7552 + (v4897 * (v7559 + v7564)));
                            v7588 = v7568;
                            v14700 = v15770;
                        } else {
                            let v7569 = -v7012;
                            let v7572 = v7518 * v6598;
                            let v15746 = (v14606 * v7569) + (((v14606 * v7518) * v6598) + (v14606 * v7572));
                            let v7575 = ((v1 + (v7569 * v6598)) + (v7572 * v6598)) - v25;
                            let v7577 = if v7575 < v7576 { 1.0 } else { 0.0 };
                            let v7586: f64;
                            let v14701: Lanes<1>;
                            if v7577 != 0.0 {
                                let v7579 = v7578 / v7575;
                                let v15756 = ((v15746 * v7579) * v14888) / v7575;
                                v7586 = v7579;
                                v14701 = v15756;
                            } else {
                                let v15747 = v15746 * v7575;
                                let v7583 = ((v7575 * v7575) + v7581).sqrt();
                                let v7585 = v4897 * (v7575 + v7583);
                                let v15753 = (v15746 + ((v15747 + v15747) * (v14600 / (v14869 * v7583)))) * v4897;
                                v7586 = v7585;
                                v14701 = v15753;
                            }
                            let v7587 = v7010 * v7586;
                            let v15757 = v14701 * v7010;
                            v7588 = v7587;
                            v14700 = v15757;
                        }
                        let v7589 = if v7588 < v7043 { 1.0 } else { 0.0 };
                        let v10450: f64;
                        let v14702: Lanes<1>;
                        if v7589 != 0.0 {
                            v10450 = v7043;
                            v14702 = v14864;
                        } else {
                            v10450 = v7588;
                            v14702 = v14700;
                        }
                        v10449 = v10450;
                        v14699 = v14702;
                    } else {
                        v10449 = v0;
                        v14699 = v14864;
                    }
                    let v8801: f64;
                    let v14703: Lanes<1>;
                    if v7336 != 0.0 {
                        let v7590 = -v7105;
                        let v7591 = -v2802;
                        let v7593 = v7518 * v6598;
                        let v15793 = (v14606 * v7591) + (((v14606 * v7518) * v6598) + (v14606 * v7593));
                        let v7597 = (((v7591 * v6598) + (v7593 * v6598)) - v7590) - v25;
                        let v15794 = v15793 * v7597;
                        let v7602 = ((v7597 * v7597) - ((v22 * v7590) * v25)).sqrt();
                        let v15800 = (v15793 + ((v15794 + v15794) * (v14600 / (v14869 * v7602)))) * v4897;
                        let v7606 = v7105 + (v7590 + (v4897 * (v7597 + v7602)));
                        v8801 = v7606;
                        v14703 = v15800;
                    } else {
                        let v7607 = -v2802;
                        let v7610 = v7518 * v6598;
                        let v15776 = (v14606 * v7607) + (((v14606 * v7518) * v6598) + (v14606 * v7610));
                        let v7613 = ((v1 + (v7607 * v6598)) + (v7610 * v6598)) - v25;
                        let v7615 = if v7613 < v7614 { 1.0 } else { 0.0 };
                        let v7624: f64;
                        let v14704: Lanes<1>;
                        if v7615 != 0.0 {
                            let v7617 = v7616 / v7613;
                            let v15786 = ((v15776 * v7617) * v14888) / v7613;
                            v7624 = v7617;
                            v14704 = v15786;
                        } else {
                            let v15777 = v15776 * v7613;
                            let v7621 = ((v7613 * v7613) + v7619).sqrt();
                            let v7623 = v4897 * (v7613 + v7621);
                            let v15783 = (v15776 + ((v15777 + v15777) * (v14600 / (v14869 * v7621)))) * v4897;
                            v7624 = v7623;
                            v14704 = v15783;
                        }
                        let v7625 = v7105 * v7624;
                        let v15787 = v14704 * v7105;
                        v8801 = v7625;
                        v14703 = v15787;
                    }
                    let v10414: f64;
                    let v14705: Lanes<1>;
                    if v3342 != 0.0 {
                        let v7662: f64;
                        let v14706: Lanes<1>;
                        if v7336 != 0.0 {
                            let v7626 = -v7136;
                            let v7627 = -v2802;
                            let v7629 = v7518 * v6598;
                            let v15823 = (v14606 * v7627) + (((v14606 * v7518) * v6598) + (v14606 * v7629));
                            let v7633 = (((v7627 * v6598) + (v7629 * v6598)) - v7626) - v25;
                            let v15824 = v15823 * v7633;
                            let v7638 = ((v7633 * v7633) - ((v22 * v7626) * v25)).sqrt();
                            let v15830 = (v15823 + ((v15824 + v15824) * (v14600 / (v14869 * v7638)))) * v4897;
                            let v7642 = v7136 + (v7626 + (v4897 * (v7633 + v7638)));
                            v7662 = v7642;
                            v14706 = v15830;
                        } else {
                            let v7643 = -v2802;
                            let v7646 = v7518 * v6598;
                            let v15806 = (v14606 * v7643) + (((v14606 * v7518) * v6598) + (v14606 * v7646));
                            let v7649 = ((v1 + (v7643 * v6598)) + (v7646 * v6598)) - v25;
                            let v7651 = if v7649 < v7650 { 1.0 } else { 0.0 };
                            let v7660: f64;
                            let v14707: Lanes<1>;
                            if v7651 != 0.0 {
                                let v7653 = v7652 / v7649;
                                let v15816 = ((v15806 * v7653) * v14888) / v7649;
                                v7660 = v7653;
                                v14707 = v15816;
                            } else {
                                let v15807 = v15806 * v7649;
                                let v7657 = ((v7649 * v7649) + v7655).sqrt();
                                let v7659 = v4897 * (v7649 + v7657);
                                let v15813 = (v15806 + ((v15807 + v15807) * (v14600 / (v14869 * v7657)))) * v4897;
                                v7660 = v7659;
                                v14707 = v15813;
                            }
                            let v7661 = v7136 * v7660;
                            let v15817 = v14707 * v7136;
                            v7662 = v7661;
                            v14706 = v15817;
                        }
                        let v7663 = if v7662 < v7043 { 1.0 } else { 0.0 };
                        let v10415: f64;
                        let v14708: Lanes<1>;
                        if v7663 != 0.0 {
                            v10415 = v7043;
                            v14708 = v14864;
                        } else {
                            v10415 = v7662;
                            v14708 = v14706;
                        }
                        v10414 = v10415;
                        v14705 = v14708;
                    } else {
                        v10414 = v0;
                        v14705 = v14864;
                    }
                    let v8807: f64;
                    if v7336 != 0.0 {
                        let v7664 = -v4493;
                        let v7672 = ((((-v2818) * v6598) + ((v7667 * v6598) * v6598)) - v7664) - v25;
                        let v7681 = v4493 + (v7664 + (v4897 * (v7672 + (((v7672 * v7672) - ((v22 * v7664) * v25)).sqrt()))));
                        v8807 = v7681;
                    } else {
                        let v7688 = ((v1 + ((-v2818) * v6598)) + ((v7667 * v6598) * v6598)) - v25;
                        let v7690 = if v7688 < v7689 { 1.0 } else { 0.0 };
                        let v7699: f64;
                        if v7690 != 0.0 {
                            let v7692 = v7691 / v7688;
                            v7699 = v7692;
                        } else {
                            let v7698 = v4897 * (v7688 + (((v7688 * v7688) + v7694).sqrt()));
                            v7699 = v7698;
                        }
                        let v7700 = v4493 * v7699;
                        v8807 = v7700;
                    }
                    let v7704 = v7703 * v6598;
                    let v7705 = v7704 * v6598;
                    let v15835 = ((v14606 * v7703) * v6598) + (v14606 * v7704);
                    let v15837 = ((v14606 * v7200) + v15835) * v7199;
                    let v7708 = (v7199 * ((v1 + (v7200 * v6598)) + v7705)) - v23;
                    let v7710 = if v7708 < v7709 { 1.0 } else { 0.0 };
                    let v7719: f64;
                    let v14709: Lanes<1>;
                    if v7710 != 0.0 {
                        let v7712 = v7711 / v7708;
                        let v15847 = ((v15837 * v7712) * v14888) / v7708;
                        v7719 = v7712;
                        v14709 = v15847;
                    } else {
                        let v15838 = v15837 * v7708;
                        let v7716 = ((v7708 * v7708) + v7714).sqrt();
                        let v7718 = v4897 * (v7708 + v7716);
                        let v15844 = (v15837 + ((v15838 + v15838) * (v14600 / (v14869 * v7716)))) * v4897;
                        v7719 = v7718;
                        v14709 = v15844;
                    }
                    let v7720 = v7719 + v23;
                    let v10397: f64;
                    let v14710: Lanes<1>;
                    if v3342 != 0.0 {
                        let v15850 = ((v14606 * v7219) + v15835) * v7217;
                        let v7725 = (v7217 * ((v1 + (v7219 * v6598)) + v7705)) - v23;
                        let v7727 = if v7725 < v7726 { 1.0 } else { 0.0 };
                        let v7736: f64;
                        let v14711: Lanes<1>;
                        if v7727 != 0.0 {
                            let v7729 = v7728 / v7725;
                            let v15860 = ((v15850 * v7729) * v14888) / v7725;
                            v7736 = v7729;
                            v14711 = v15860;
                        } else {
                            let v15851 = v15850 * v7725;
                            let v7733 = ((v7725 * v7725) + v7731).sqrt();
                            let v7735 = v4897 * (v7725 + v7733);
                            let v15857 = (v15850 + ((v15851 + v15851) * (v14600 / (v14869 * v7733)))) * v4897;
                            v7736 = v7735;
                            v14711 = v15857;
                        }
                        let v7737 = v7736 + v23;
                        v10397 = v7737;
                        v14710 = v14711;
                    } else {
                        v10397 = v0;
                        v14710 = v14864;
                    }
                    let v10464: f64;
                    let v14712: Lanes<1>;
                    if v7336 != 0.0 {
                        let v7738 = -v1042;
                        let v7742 = v7741 * v6598;
                        let v15883 = (v14606 * v7739) + (((v14606 * v7741) * v6598) + (v14606 * v7742));
                        let v7746 = (((v7739 * v6598) + (v7742 * v6598)) - v7738) - v25;
                        let v15884 = v15883 * v7746;
                        let v7751 = ((v7746 * v7746) - ((v22 * v7738) * v25)).sqrt();
                        let v15890 = (v15883 + ((v15884 + v15884) * (v14600 / (v14869 * v7751)))) * v4897;
                        let v7755 = v1042 + (v7738 + (v4897 * (v7746 + v7751)));
                        v10464 = v7755;
                        v14712 = v15890;
                    } else {
                        let v7758 = v7741 * v6598;
                        let v15866 = (v14606 * v7739) + (((v14606 * v7741) * v6598) + (v14606 * v7758));
                        let v7761 = ((v1 + (v7739 * v6598)) + (v7758 * v6598)) - v25;
                        let v7763 = if v7761 < v7762 { 1.0 } else { 0.0 };
                        let v7772: f64;
                        let v14713: Lanes<1>;
                        if v7763 != 0.0 {
                            let v7765 = v7764 / v7761;
                            let v15876 = ((v15866 * v7765) * v14888) / v7761;
                            v7772 = v7765;
                            v14713 = v15876;
                        } else {
                            let v15867 = v15866 * v7761;
                            let v7769 = ((v7761 * v7761) + v7767).sqrt();
                            let v7771 = v4897 * (v7761 + v7769);
                            let v15873 = (v15866 + ((v15867 + v15867) * (v14600 / (v14869 * v7769)))) * v4897;
                            v7772 = v7771;
                            v14713 = v15873;
                        }
                        let v7773 = v1042 * v7772;
                        let v15877 = v14713 * v1042;
                        v10464 = v7773;
                        v14712 = v15877;
                    }
                    let v10441: f64;
                    let v14714: Lanes<1>;
                    if v7336 != 0.0 {
                        let v7774 = -v4426;
                        let v15903 = v14606 * v7775;
                        let v7778 = ((v7775 * v6598) - v7774) - v25;
                        let v15904 = v15903 * v7778;
                        let v7783 = ((v7778 * v7778) - ((v22 * v7774) * v25)).sqrt();
                        let v15910 = (v15903 + ((v15904 + v15904) * (v14600 / (v14869 * v7783)))) * v4897;
                        let v7787 = v4426 + (v7774 + (v4897 * (v7778 + v7783)));
                        v10441 = v7787;
                        v14714 = v15910;
                    } else {
                        let v15891 = v14606 * v7775;
                        let v7790 = (v1 + (v7775 * v6598)) - v25;
                        let v7792 = if v7790 < v7791 { 1.0 } else { 0.0 };
                        let v7801: f64;
                        let v14715: Lanes<1>;
                        if v7792 != 0.0 {
                            let v7794 = v7793 / v7790;
                            let v15901 = ((v15891 * v7794) * v14888) / v7790;
                            v7801 = v7794;
                            v14715 = v15901;
                        } else {
                            let v15892 = v15891 * v7790;
                            let v7798 = ((v7790 * v7790) + v7796).sqrt();
                            let v7800 = v4897 * (v7790 + v7798);
                            let v15898 = (v15891 + ((v15892 + v15892) * (v14600 / (v14869 * v7798)))) * v4897;
                            v7801 = v7800;
                            v14715 = v15898;
                        }
                        let v7802 = v4426 * v7801;
                        let v15902 = v14715 * v4426;
                        v10441 = v7802;
                        v14714 = v15902;
                    }
                    let v7804 = v2898 + (v7236 / v3969);
                    let v7810 = v7807 * (v6594 - v7808);
                    let v7812 = v1 + (rspice_limited_exp(v7810));
                    let v7813 = v7806 / v7812;
                    let v15918 = (v14865 * v7804) + (((((v14606 * v7807) * (rspice_limited_exp_derivative(v7810))) * v7813) * v14888) / v7812);
                    let v7820 = ((v7804 * v6597) + v7813) - (v7806 / (v1 + (rspice_limited_exp((v7807 * (v6595 - v7808))))));
                    v8794 = v8795;
                    v8800 = v8801;
                    v8806 = v8807;
                    v10396 = v10397;
                    v10401 = v7720;
                    v10413 = v10414;
                    v10440 = v10441;
                    v10448 = v10449;
                    v10463 = v10464;
                    v10478 = v10479;
                    v10483 = v7271;
                    v10488 = v10489;
                    v10493 = v7307;
                    v10498 = v10499;
                    v10503 = v7316;
                    v10521 = v10522;
                    v10620 = v7820;
                    v10941 = v7325;
                    v10987 = v7514;
                    v11305 = v11306;
                    v11606 = v11607;
                    v11620 = v11621;
                    v12162 = v7334;
                    v12165 = v7335;
                    v14667 = v14697;
                    v14668 = v14703;
                    v14669 = v14710;
                    v14670 = v14709;
                    v14671 = v14705;
                    v14672 = v14714;
                    v14673 = v14699;
                    v14674 = v14712;
                    v14675 = v14687;
                    v14676 = v15579;
                    v14677 = v14688;
                    v14678 = v15611;
                    v14679 = v14689;
                    v14680 = v15621;
                    v14681 = v14690;
                    v14682 = v15918;
                    v14683 = v15631;
                    v14684 = v14696;
                    v14685 = v15635;
                    v14686 = v15639;
                } else {
                    let v7823 = v2626 * v7821;
                    let v7824 = v2610 + v7823;
                    let v15028 = (v14608 * v2626) * v6852;
                    let v7826 = (v7824 * v6852).exp();
                    let v7827 = v6853 * v7826;
                    let v15032 = ((v15028 + (v15026 * v7824)) * v7826) * v6853;
                    let v7830 = v2642 * v6598;
                    let v15034 = v14606 * v2642;
                    let v7838 = (v7830 - (v7835 * v7827)) - v4767;
                    let v7842 = (v7830 - (v7839 * v7827)) - v4767;
                    let v7849 = ((v7838 * v7842) - ((v22 * (v7844 * v7827)) * v4767)).sqrt();
                    let v7853 = v7827 + ((v7828 * v7827) + (v4897 * (((v7830 - (v7831 * v7827)) - v4767) + v7849)));
                    let v15054 = v15032 + ((v15032 * v7828) + (((v15034 - (v15032 * v7831)) + (((((v15034 - (v15032 * v7835)) * v7842) + ((v15034 - (v15032 * v7839)) * v7838)) - (((v15032 * v7844) * v22) * v4767)) * (v14600 / (v14869 * v7849)))) * v4897));
                    let v7854 = if v3341 == v1 { 1.0 } else { 0.0 };
                    let v10480: f64;
                    let v14716: Lanes<1>;
                    if v7854 != 0.0 {
                        let v7855 = v6887 + v7823;
                        let v7857 = (v7855 * v6852).exp();
                        let v7858 = v6885 * v7857;
                        let v15058 = ((v15028 + (v15026 * v7855)) * v7857) * v6885;
                        let v7861 = v6893 * v6598;
                        let v15060 = v14606 * v6893;
                        let v7869 = (v7861 - (v7866 * v7858)) - v4767;
                        let v7873 = (v7861 - (v7870 * v7858)) - v4767;
                        let v7880 = ((v7869 * v7873) - ((v22 * (v7875 * v7858)) * v4767)).sqrt();
                        let v7884 = v7858 + ((v7859 * v7858) + (v4897 * (((v7861 - (v7862 * v7858)) - v4767) + v7880)));
                        let v15080 = v15058 + ((v15058 * v7859) + (((v15060 - (v15058 * v7862)) + (((((v15060 - (v15058 * v7866)) * v7873) + ((v15060 - (v15058 * v7870)) * v7869)) - (((v15058 * v7875) * v22) * v4767)) * (v14600 / (v14869 * v7880)))) * v4897));
                        v10480 = v7884;
                        v14716 = v15080;
                    } else {
                        v10480 = v0;
                        v14716 = v14864;
                    }
                    let v7885 = if v6595 > v6661 { 1.0 } else { 0.0 };
                    let v7936: f64;
                    let v7940: f64;
                    let v14717: Lanes<1>;
                    let v14718: Lanes<1>;
                    if v7885 != 0.0 {
                        let v7886 = v6661 / v6595;
                        let v7887 = if v7886 > v185 { 1.0 } else { 0.0 };
                        let v7894: f64;
                        if v7887 != 0.0 {
                            let v7888 = v7886.ln();
                            v7894 = v7888;
                        } else {
                            v7894 = v7889;
                        }
                        let v7892 = v6918 + (v2674 * (v6661 - v6595));
                        let v7899 = v6661 * ((v2674 / v7892) - ((v2690 * (v7894 + v1)) / v6595));
                        let v7903 = v7892 / (v7886.powf((v7899 + (v2690 * v7886))));
                        let v7905 = v7899 + (v2690 * v6596);
                        let v7906 = v6596.powf(v7905);
                        let v7907 = v7903 * v7906;
                        let v15099 = ((v14865 * (v7905 * (v6596.powf((v7905 - v14600))))) + ((v14865 * v2690) * (v7906 * v6852))) * v7903;
                        let v15100 = v14606 * v2674;
                        let v7909 = v6918 + (v2674 * v6598);
                        v7936 = v7907;
                        v7940 = v7909;
                        v14717 = v15099;
                        v14718 = v15100;
                    } else {
                        let v7910 = v6661 / v6595;
                        let v7911 = if v7910 > v185 { 1.0 } else { 0.0 };
                        let v7919: f64;
                        if v7911 != 0.0 {
                            let v7912 = v7910.ln();
                            v7919 = v7912;
                        } else {
                            v7919 = v7913;
                        }
                        let v7917 = v6918 * (v7910.powf((v2674 + (v2690 * v7910))));
                        let v7924 = v7917 * ((v2674 / v6661) + ((v2690 * (v7919 + v1)) / v6595));
                        let v7929 = v2674 + (v2690 * v6596);
                        let v7930 = v6596.powf(v7929);
                        let v7931 = v6918 * v7930;
                        let v15089 = ((v14865 * (v7929 * (v6596.powf((v7929 - v14600))))) + ((v14865 * v2690) * (v7930 * v6852))) * v6918;
                        let v15090 = v14606 * v7924;
                        let v7933 = (v7917 - (v7924 * (v6661 - v6595))) + (v7924 * v6598);
                        v7936 = v7931;
                        v7940 = v7933;
                        v14717 = v15089;
                        v14718 = v15090;
                    }
                    let v7942 = (v7934 * v7936) + (v7938 * v7940);
                    let v15107 = ((v14609 * v7936) + (v14717 * v7934)) + ((v14610 * v7940) + (v14718 * v7938));
                    let v7944 = if v7942 < v7943 { 1.0 } else { 0.0 };
                    let v7953: f64;
                    let v14719: Lanes<1>;
                    if v7944 != 0.0 {
                        let v7946 = v7945 / v7942;
                        let v15117 = ((v15107 * v7946) * v14888) / v7942;
                        v7953 = v7946;
                        v14719 = v15117;
                    } else {
                        let v15108 = v15107 * v7942;
                        let v7950 = ((v7942 * v7942) + v7948).sqrt();
                        let v7952 = v4897 * (v7942 + v7950);
                        let v15114 = (v15107 + ((v15108 + v15108) * (v14600 / (v14869 * v7950)))) * v4897;
                        v7953 = v7952;
                        v14719 = v15114;
                    }
                    let v10490: f64;
                    let v14720: Lanes<1>;
                    if v3342 != 0.0 {
                        let v8002: f64;
                        let v8004: f64;
                        let v14721: Lanes<1>;
                        let v14722: Lanes<1>;
                        if v7885 != 0.0 {
                            let v7954 = v6661 / v6595;
                            let v7955 = if v7954 > v185 { 1.0 } else { 0.0 };
                            let v7962: f64;
                            if v7955 != 0.0 {
                                let v7956 = v7954.ln();
                                v7962 = v7956;
                            } else {
                                v7962 = v7957;
                            }
                            let v7960 = v6933 + (v6936 * (v6661 - v6595));
                            let v7967 = v6661 * ((v6936 / v7960) - ((v2690 * (v7962 + v1)) / v6595));
                            let v7971 = v7960 / (v7954.powf((v7967 + (v2690 * v7954))));
                            let v7973 = v7967 + (v2690 * v6596);
                            let v7974 = v6596.powf(v7973);
                            let v7975 = v7971 * v7974;
                            let v15136 = ((v14865 * (v7973 * (v6596.powf((v7973 - v14600))))) + ((v14865 * v2690) * (v7974 * v6852))) * v7971;
                            let v15137 = v14606 * v6936;
                            let v7977 = v6933 + (v6936 * v6598);
                            v8002 = v7975;
                            v8004 = v7977;
                            v14721 = v15136;
                            v14722 = v15137;
                        } else {
                            let v7978 = v6661 / v6595;
                            let v7979 = if v7978 > v185 { 1.0 } else { 0.0 };
                            let v7987: f64;
                            if v7979 != 0.0 {
                                let v7980 = v7978.ln();
                                v7987 = v7980;
                            } else {
                                v7987 = v7981;
                            }
                            let v7985 = v6933 * (v7978.powf((v6936 + (v2690 * v7978))));
                            let v7992 = v7985 * ((v6936 / v6661) + ((v2690 * (v7987 + v1)) / v6595));
                            let v7997 = v6936 + (v2690 * v6596);
                            let v7998 = v6596.powf(v7997);
                            let v7999 = v6933 * v7998;
                            let v15126 = ((v14865 * (v7997 * (v6596.powf((v7997 - v14600))))) + ((v14865 * v2690) * (v7998 * v6852))) * v6933;
                            let v15127 = v14606 * v7992;
                            let v8001 = (v7985 - (v7992 * (v6661 - v6595))) + (v7992 * v6598);
                            v8002 = v7999;
                            v8004 = v8001;
                            v14721 = v15126;
                            v14722 = v15127;
                        }
                        let v8006 = (v7934 * v8002) + (v7938 * v8004);
                        let v15144 = ((v14609 * v8002) + (v14721 * v7934)) + ((v14610 * v8004) + (v14722 * v7938));
                        let v8008 = if v8006 < v8007 { 1.0 } else { 0.0 };
                        let v8017: f64;
                        let v14723: Lanes<1>;
                        if v8008 != 0.0 {
                            let v8010 = v8009 / v8006;
                            let v15154 = ((v15144 * v8010) * v14888) / v8006;
                            v8017 = v8010;
                            v14723 = v15154;
                        } else {
                            let v15145 = v15144 * v8006;
                            let v8014 = ((v8006 * v8006) + v8012).sqrt();
                            let v8016 = v4897 * (v8006 + v8014);
                            let v15151 = (v15144 + ((v15145 + v15145) * (v14600 / (v14869 * v8014)))) * v4897;
                            v8017 = v8016;
                            v14723 = v15151;
                        }
                        v10490 = v8017;
                        v14720 = v14723;
                    } else {
                        v10490 = v0;
                        v14720 = v14864;
                    }
                    let v8018 = v2738 * v7821;
                    let v8019 = v2722 + v8018;
                    let v15156 = (v14608 * v2738) * v6852;
                    let v8021 = (v8019 * v6852).exp();
                    let v8022 = v6949 * v8021;
                    let v15160 = ((v15156 + (v15026 * v8019)) * v8021) * v6949;
                    let v10500: f64;
                    let v14724: Lanes<1>;
                    if v3342 != 0.0 {
                        let v8023 = v6955 + v8018;
                        let v8025 = (v8023 * v6852).exp();
                        let v8026 = v6953 * v8025;
                        let v15164 = ((v15156 + (v15026 * v8023)) * v8025) * v6953;
                        v10500 = v8026;
                        v14724 = v15164;
                    } else {
                        v10500 = v0;
                        v14724 = v14864;
                    }
                    let v8028 = v2754 + (v2770 * v7821);
                    let v8030 = (v8028 * v6852).exp();
                    let v8031 = v6959 * v8030;
                    let v15170 = ((((v14608 * v2770) * v6852) + (v15026 * v8028)) * v8030) * v6959;
                    let v8032 = v6595 - v6661;
                    let v8034 = (v1218 * v8032) / v6595;
                    let v8036 = if (v8034.abs()) < v25 { 1.0 } else { 0.0 };
                    let v8065: f64;
                    let v14725: Lanes<1>;
                    if v8036 != 0.0 {
                        let v8037 = v1218 * v7821;
                        let v8040 = v1202 * ((rspice_limited_exp(v8037)) - v1);
                        let v15179 = ((v14608 * v1218) * (rspice_limited_exp_derivative(v8037))) * v1202;
                        v8065 = v8040;
                        v14725 = v15179;
                    } else {
                        let v8041 = v1218 * v7821;
                        let v8047 = ((rspice_limited_exp(v8034)) - v1).abs();
                        let v8048 = (v1202 * ((rspice_limited_exp(v8041)) - v1)) / v8047;
                        let v15175 = (((v14608 * v1218) * (rspice_limited_exp_derivative(v8041))) * v1202) / v8047;
                        v8065 = v8048;
                        v14725 = v15175;
                    }
                    let v8050 = (v1250 * v8032) / v6595;
                    let v8052 = if (v8050.abs()) < v25 { 1.0 } else { 0.0 };
                    let v8067: f64;
                    let v14726: Lanes<1>;
                    if v8052 != 0.0 {
                        let v8053 = v1250 * v7821;
                        let v8056 = v1234 * ((rspice_limited_exp(v8053)) - v1);
                        let v15188 = ((v14608 * v1250) * (rspice_limited_exp_derivative(v8053))) * v1234;
                        v8067 = v8056;
                        v14726 = v15188;
                    } else {
                        let v8057 = v1250 * v7821;
                        let v8063 = ((rspice_limited_exp(v8050)) - v1).abs();
                        let v8064 = (v1234 * ((rspice_limited_exp(v8057)) - v1)) / v8063;
                        let v15184 = (((v14608 * v1250) * (rspice_limited_exp_derivative(v8057))) * v1234) / v8063;
                        v8067 = v8064;
                        v14726 = v15184;
                    }
                    let v8066 = v4897 + v8065;
                    let v8068 = v4897 + v8067;
                    let v8069 = if v6977 != v0 { 1.0 } else { 0.0 };
                    let v8796: f64;
                    let v8802: f64;
                    let v8808: f64;
                    let v10416: f64;
                    let v10442: f64;
                    let v10451: f64;
                    let v10465: f64;
                    let v10523: f64;
                    let v14727: Lanes<1>;
                    let v14728: Lanes<1>;
                    let v14729: Lanes<1>;
                    let v14730: Lanes<1>;
                    let v14731: Lanes<1>;
                    let v14732: Lanes<1>;
                    let v14733: Lanes<1>;
                    if v8069 != 0.0 {
                        let v8070 = -v6932;
                        let v15284 = v14611 * v2706;
                        let v8075 = ((v2706 * v8071) - v8070) - v25;
                        let v15285 = v15284 * v8075;
                        let v8080 = ((v8075 * v8075) - ((v22 * v8070) * v25)).sqrt();
                        let v15291 = (v15284 + ((v15285 + v15285) * (v14600 / (v14869 * v8080)))) * v4897;
                        let v8084 = v6932 + (v8070 + (v4897 * (v8075 + v8080)));
                        let v8085 = -v6979;
                        let v8086 = -v2802;
                        let v8088 = v7518 * v8071;
                        let v8089 = v8088 * v8071;
                        let v15296 = ((v14611 * v7518) * v8071) + (v14611 * v8088);
                        let v8090 = (v8086 * v6598) + v8089;
                        let v15297 = (v14606 * v8086) + v15296;
                        let v8092 = (v8090 - v8085) - v25;
                        let v15298 = v15297 * v8092;
                        let v8097 = ((v8092 * v8092) - ((v22 * v8085) * v25)).sqrt();
                        let v15304 = (v15297 + ((v15298 + v15298) * (v14600 / (v14869 * v8097)))) * v4897;
                        let v8101 = v6979 + (v8085 + (v4897 * (v8092 + v8097)));
                        let v10452: f64;
                        let v14734: Lanes<1>;
                        if v3342 != 0.0 {
                            let v8102 = -v7010;
                            let v8103 = -v7012;
                            let v15306 = (v14606 * v8103) + v15296;
                            let v8107 = (((v8103 * v6598) + v8089) - v8102) - v25;
                            let v15307 = v15306 * v8107;
                            let v8112 = ((v8107 * v8107) - ((v22 * v8102) * v25)).sqrt();
                            let v15313 = (v15306 + ((v15307 + v15307) * (v14600 / (v14869 * v8112)))) * v4897;
                            let v8116 = v7010 + (v8102 + (v4897 * (v8107 + v8112)));
                            let v8117 = if v8116 < v7043 { 1.0 } else { 0.0 };
                            let v10453: f64;
                            let v14735: Lanes<1>;
                            if v8117 != 0.0 {
                                v10453 = v7043;
                                v14735 = v14864;
                            } else {
                                v10453 = v8116;
                                v14735 = v15313;
                            }
                            v10452 = v10453;
                            v14734 = v14735;
                        } else {
                            v10452 = v0;
                            v14734 = v14864;
                        }
                        let v8118 = -v7105;
                        let v8120 = (v8090 - v8118) - v25;
                        let v15314 = v15297 * v8120;
                        let v8125 = ((v8120 * v8120) - ((v22 * v8118) * v25)).sqrt();
                        let v15320 = (v15297 + ((v15314 + v15314) * (v14600 / (v14869 * v8125)))) * v4897;
                        let v8129 = v7105 + (v8118 + (v4897 * (v8120 + v8125)));
                        let v10417: f64;
                        let v14736: Lanes<1>;
                        if v3342 != 0.0 {
                            let v8130 = -v7136;
                            let v8132 = (v8090 - v8130) - v25;
                            let v15321 = v15297 * v8132;
                            let v8137 = ((v8132 * v8132) - ((v22 * v8130) * v25)).sqrt();
                            let v15327 = (v15297 + ((v15321 + v15321) * (v14600 / (v14869 * v8137)))) * v4897;
                            let v8141 = v7136 + (v8130 + (v4897 * (v8132 + v8137)));
                            let v8142 = if v8141 < v7043 { 1.0 } else { 0.0 };
                            let v10418: f64;
                            let v14737: Lanes<1>;
                            if v8142 != 0.0 {
                                v10418 = v7043;
                                v14737 = v14864;
                            } else {
                                v10418 = v8141;
                                v14737 = v15327;
                            }
                            v10417 = v10418;
                            v14736 = v14737;
                        } else {
                            v10417 = v0;
                            v14736 = v14864;
                        }
                        let v8143 = -v4493;
                        let v8150 = ((((-v2818) * v6598) + ((v7667 * v8071) * v8071)) - v8143) - v25;
                        let v8159 = v4493 + (v8143 + (v4897 * (v8150 + (((v8150 * v8150) - ((v22 * v8143) * v25)).sqrt()))));
                        let v8160 = -v1042;
                        let v8162 = v7741 * v8071;
                        let v15333 = (v14611 * v7739) + (((v14611 * v7741) * v8071) + (v14611 * v8162));
                        let v8166 = (((v7739 * v8071) + (v8162 * v8071)) - v8160) - v25;
                        let v15334 = v15333 * v8166;
                        let v8171 = ((v8166 * v8166) - ((v22 * v8160) * v25)).sqrt();
                        let v15340 = (v15333 + ((v15334 + v15334) * (v14600 / (v14869 * v8171)))) * v4897;
                        let v8175 = v1042 + (v8160 + (v4897 * (v8166 + v8171)));
                        let v8176 = -v4426;
                        let v15341 = v14611 * v7775;
                        let v8179 = ((v7775 * v8071) - v8176) - v25;
                        let v15342 = v15341 * v8179;
                        let v8184 = ((v8179 * v8179) - ((v22 * v8176) * v25)).sqrt();
                        let v15348 = (v15341 + ((v15342 + v15342) * (v14600 / (v14869 * v8184)))) * v4897;
                        let v8188 = v4426 + (v8176 + (v4897 * (v8179 + v8184)));
                        v8796 = v8101;
                        v8802 = v8129;
                        v8808 = v8159;
                        v10416 = v10417;
                        v10442 = v8188;
                        v10451 = v10452;
                        v10465 = v8175;
                        v10523 = v8084;
                        v14727 = v15304;
                        v14728 = v15320;
                        v14729 = v14736;
                        v14730 = v15348;
                        v14731 = v14734;
                        v14732 = v15340;
                        v14733 = v15291;
                    } else {
                        let v15189 = v14611 * v2706;
                        let v8191 = (v1 + (v2706 * v8071)) - v25;
                        let v8193 = if v8191 < v8192 { 1.0 } else { 0.0 };
                        let v8202: f64;
                        let v14738: Lanes<1>;
                        if v8193 != 0.0 {
                            let v8195 = v8194 / v8191;
                            let v15199 = ((v15189 * v8195) * v14888) / v8191;
                            v8202 = v8195;
                            v14738 = v15199;
                        } else {
                            let v15190 = v15189 * v8191;
                            let v8199 = ((v8191 * v8191) + v8197).sqrt();
                            let v8201 = v4897 * (v8191 + v8199);
                            let v15196 = (v15189 + ((v15190 + v15190) * (v14600 / (v14869 * v8199)))) * v4897;
                            v8202 = v8201;
                            v14738 = v15196;
                        }
                        let v8203 = v6932 * v8202;
                        let v15200 = v14738 * v6932;
                        let v8206 = v7518 * v8071;
                        let v8207 = v8206 * v8071;
                        let v15206 = ((v14611 * v7518) * v8071) + (v14611 * v8206);
                        let v15207 = ((v14606 * v2802) * v14888) + v15206;
                        let v8209 = ((v1 - (v2802 * v6598)) + v8207) - v25;
                        let v8211 = if v8209 < v8210 { 1.0 } else { 0.0 };
                        let v8220: f64;
                        let v14739: Lanes<1>;
                        if v8211 != 0.0 {
                            let v8213 = v8212 / v8209;
                            let v15217 = ((v15207 * v8213) * v14888) / v8209;
                            v8220 = v8213;
                            v14739 = v15217;
                        } else {
                            let v15208 = v15207 * v8209;
                            let v8217 = ((v8209 * v8209) + v8215).sqrt();
                            let v8219 = v4897 * (v8209 + v8217);
                            let v15214 = (v15207 + ((v15208 + v15208) * (v14600 / (v14869 * v8217)))) * v4897;
                            v8220 = v8219;
                            v14739 = v15214;
                        }
                        let v8221 = v6979 * v8220;
                        let v15218 = v14739 * v6979;
                        let v10454: f64;
                        let v14740: Lanes<1>;
                        if v3342 != 0.0 {
                            let v15221 = ((v14606 * v7012) * v14888) + v15206;
                            let v8225 = ((v1 - (v7012 * v6598)) + v8207) - v25;
                            let v8227 = if v8225 < v8226 { 1.0 } else { 0.0 };
                            let v8236: f64;
                            let v14741: Lanes<1>;
                            if v8227 != 0.0 {
                                let v8229 = v8228 / v8225;
                                let v15231 = ((v15221 * v8229) * v14888) / v8225;
                                v8236 = v8229;
                                v14741 = v15231;
                            } else {
                                let v15222 = v15221 * v8225;
                                let v8233 = ((v8225 * v8225) + v8231).sqrt();
                                let v8235 = v4897 * (v8225 + v8233);
                                let v15228 = (v15221 + ((v15222 + v15222) * (v14600 / (v14869 * v8233)))) * v4897;
                                v8236 = v8235;
                                v14741 = v15228;
                            }
                            let v8237 = v7010 * v8236;
                            let v15232 = v14741 * v7010;
                            let v8238 = if v8237 < v7043 { 1.0 } else { 0.0 };
                            let v10455: f64;
                            let v14742: Lanes<1>;
                            if v8238 != 0.0 {
                                v10455 = v7043;
                                v14742 = v14864;
                            } else {
                                v10455 = v8237;
                                v14742 = v15232;
                            }
                            v10454 = v10455;
                            v14740 = v14742;
                        } else {
                            v10454 = v0;
                            v14740 = v14864;
                        }
                        let v8240 = if v8209 < v8239 { 1.0 } else { 0.0 };
                        let v8249: f64;
                        let v14743: Lanes<1>;
                        if v8240 != 0.0 {
                            let v8242 = v8241 / v8209;
                            let v15242 = ((v15207 * v8242) * v14888) / v8209;
                            v8249 = v8242;
                            v14743 = v15242;
                        } else {
                            let v15233 = v15207 * v8209;
                            let v8246 = ((v8209 * v8209) + v8244).sqrt();
                            let v8248 = v4897 * (v8209 + v8246);
                            let v15239 = (v15207 + ((v15233 + v15233) * (v14600 / (v14869 * v8246)))) * v4897;
                            v8249 = v8248;
                            v14743 = v15239;
                        }
                        let v8250 = v7105 * v8249;
                        let v15243 = v14743 * v7105;
                        let v10419: f64;
                        let v14744: Lanes<1>;
                        if v3342 != 0.0 {
                            let v8252 = if v8209 < v8251 { 1.0 } else { 0.0 };
                            let v8261: f64;
                            let v14745: Lanes<1>;
                            if v8252 != 0.0 {
                                let v8254 = v8253 / v8209;
                                let v15253 = ((v15207 * v8254) * v14888) / v8209;
                                v8261 = v8254;
                                v14745 = v15253;
                            } else {
                                let v15244 = v15207 * v8209;
                                let v8258 = ((v8209 * v8209) + v8256).sqrt();
                                let v8260 = v4897 * (v8209 + v8258);
                                let v15250 = (v15207 + ((v15244 + v15244) * (v14600 / (v14869 * v8258)))) * v4897;
                                v8261 = v8260;
                                v14745 = v15250;
                            }
                            let v8262 = v7136 * v8261;
                            let v15254 = v14745 * v7136;
                            let v8263 = if v8262 < v7043 { 1.0 } else { 0.0 };
                            let v10420: f64;
                            let v14746: Lanes<1>;
                            if v8263 != 0.0 {
                                v10420 = v7043;
                                v14746 = v14864;
                            } else {
                                v10420 = v8262;
                                v14746 = v15254;
                            }
                            v10419 = v10420;
                            v14744 = v14746;
                        } else {
                            v10419 = v0;
                            v14744 = v14864;
                        }
                        let v8269 = ((v1 - (v2818 * v6598)) + ((v7667 * v8071) * v8071)) - v25;
                        let v8271 = if v8269 < v8270 { 1.0 } else { 0.0 };
                        let v8280: f64;
                        if v8271 != 0.0 {
                            let v8273 = v8272 / v8269;
                            v8280 = v8273;
                        } else {
                            let v8279 = v4897 * (v8269 + (((v8269 * v8269) + v8275).sqrt()));
                            v8280 = v8279;
                        }
                        let v8281 = v4493 * v8280;
                        let v8284 = v7741 * v8071;
                        let v15260 = (v14611 * v7739) + (((v14611 * v7741) * v8071) + (v14611 * v8284));
                        let v8287 = ((v1 + (v7739 * v8071)) + (v8284 * v8071)) - v25;
                        let v8289 = if v8287 < v8288 { 1.0 } else { 0.0 };
                        let v8298: f64;
                        let v14747: Lanes<1>;
                        if v8289 != 0.0 {
                            let v8291 = v8290 / v8287;
                            let v15270 = ((v15260 * v8291) * v14888) / v8287;
                            v8298 = v8291;
                            v14747 = v15270;
                        } else {
                            let v15261 = v15260 * v8287;
                            let v8295 = ((v8287 * v8287) + v8293).sqrt();
                            let v8297 = v4897 * (v8287 + v8295);
                            let v15267 = (v15260 + ((v15261 + v15261) * (v14600 / (v14869 * v8295)))) * v4897;
                            v8298 = v8297;
                            v14747 = v15267;
                        }
                        let v8299 = v1042 * v8298;
                        let v15271 = v14747 * v1042;
                        let v15272 = v14611 * v7775;
                        let v8302 = (v1 + (v7775 * v8071)) - v25;
                        let v8304 = if v8302 < v8303 { 1.0 } else { 0.0 };
                        let v8313: f64;
                        let v14748: Lanes<1>;
                        if v8304 != 0.0 {
                            let v8306 = v8305 / v8302;
                            let v15282 = ((v15272 * v8306) * v14888) / v8302;
                            v8313 = v8306;
                            v14748 = v15282;
                        } else {
                            let v15273 = v15272 * v8302;
                            let v8310 = ((v8302 * v8302) + v8308).sqrt();
                            let v8312 = v4897 * (v8302 + v8310);
                            let v15279 = (v15272 + ((v15273 + v15273) * (v14600 / (v14869 * v8310)))) * v4897;
                            v8313 = v8312;
                            v14748 = v15279;
                        }
                        let v8314 = v4426 * v8313;
                        let v15283 = v14748 * v4426;
                        v8796 = v8221;
                        v8802 = v8250;
                        v8808 = v8281;
                        v10416 = v10419;
                        v10442 = v8314;
                        v10451 = v10454;
                        v10465 = v8299;
                        v10523 = v8203;
                        v14727 = v15218;
                        v14728 = v15243;
                        v14729 = v14744;
                        v14730 = v15283;
                        v14731 = v14740;
                        v14732 = v15271;
                        v14733 = v15200;
                    }
                    let v8317 = v7703 * v8071;
                    let v8318 = v8317 * v8071;
                    let v15353 = ((v14611 * v7703) * v8071) + (v14611 * v8317);
                    let v15355 = ((v14606 * v7200) + v15353) * v7199;
                    let v8321 = (v7199 * ((v1 + (v7200 * v6598)) + v8318)) - v23;
                    let v8323 = if v8321 < v8322 { 1.0 } else { 0.0 };
                    let v8332: f64;
                    let v14749: Lanes<1>;
                    if v8323 != 0.0 {
                        let v8325 = v8324 / v8321;
                        let v15365 = ((v15355 * v8325) * v14888) / v8321;
                        v8332 = v8325;
                        v14749 = v15365;
                    } else {
                        let v15356 = v15355 * v8321;
                        let v8329 = ((v8321 * v8321) + v8327).sqrt();
                        let v8331 = v4897 * (v8321 + v8329);
                        let v15362 = (v15355 + ((v15356 + v15356) * (v14600 / (v14869 * v8329)))) * v4897;
                        v8332 = v8331;
                        v14749 = v15362;
                    }
                    let v8333 = v8332 + v23;
                    let v10398: f64;
                    let v14750: Lanes<1>;
                    if v3342 != 0.0 {
                        let v15368 = ((v14606 * v7219) + v15353) * v7217;
                        let v8338 = (v7217 * ((v1 + (v7219 * v6598)) + v8318)) - v23;
                        let v8340 = if v8338 < v8339 { 1.0 } else { 0.0 };
                        let v8349: f64;
                        let v14751: Lanes<1>;
                        if v8340 != 0.0 {
                            let v8342 = v8341 / v8338;
                            let v15378 = ((v15368 * v8342) * v14888) / v8338;
                            v8349 = v8342;
                            v14751 = v15378;
                        } else {
                            let v15369 = v15368 * v8338;
                            let v8346 = ((v8338 * v8338) + v8344).sqrt();
                            let v8348 = v4897 * (v8338 + v8346);
                            let v15375 = (v15368 + ((v15369 + v15369) * (v14600 / (v14869 * v8346)))) * v4897;
                            v8349 = v8348;
                            v14751 = v15375;
                        }
                        let v8350 = v8349 + v23;
                        v10398 = v8350;
                        v14750 = v14751;
                    } else {
                        v10398 = v0;
                        v14750 = v14864;
                    }
                    let v11307: f64;
                    let v11608: f64;
                    let v11622: f64;
                    if v3728 != 0.0 {
                        let v8355 = v7045 * (((v7048 + (v7365 * v7821)) * v6852).exp());
                        let v8358 = v7055 * v6598;
                        let v8381 = v8355 + ((v8356 * v8355) + (v4897 * (((v8358 - (v8359 * v8355)) - v4767) + (((((v8358 - (v8363 * v8355)) - v4767) * ((v8358 - (v8367 * v8355)) - v4767)) - ((v22 * (v8372 * v8355)) * v4767)).sqrt()))));
                        let v8430: f64;
                        let v8432: f64;
                        if v7885 != 0.0 {
                            let v8382 = v6661 / v6595;
                            let v8383 = if v8382 > v185 { 1.0 } else { 0.0 };
                            let v8390: f64;
                            if v8383 != 0.0 {
                                let v8384 = v8382.ln();
                                v8390 = v8384;
                            } else {
                                v8390 = v8385;
                            }
                            let v8388 = v7081 + (v7084 * (v6661 - v6595));
                            let v8395 = v6661 * ((v7084 / v8388) - ((v7398 * (v8390 + v1)) / v6595));
                            let v8403 = (v8388 / (v8382.powf((v8395 + (v7398 * v8382))))) * (v6596.powf((v8395 + (v7398 * v6596))));
                            let v8405 = v7081 + (v7084 * v6598);
                            v8430 = v8403;
                            v8432 = v8405;
                        } else {
                            let v8406 = v6661 / v6595;
                            let v8407 = if v8406 > v185 { 1.0 } else { 0.0 };
                            let v8415: f64;
                            if v8407 != 0.0 {
                                let v8408 = v8406.ln();
                                v8415 = v8408;
                            } else {
                                v8415 = v8409;
                            }
                            let v8413 = v7081 * (v8406.powf((v7084 + (v7398 * v8406))));
                            let v8420 = v8413 * ((v7084 / v6661) + ((v7398 * (v8415 + v1)) / v6595));
                            let v8427 = v7081 * (v6596.powf((v7084 + (v7398 * v6596))));
                            let v8429 = (v8413 - (v8420 * (v6661 - v6595))) + (v8420 * v6598);
                            v8430 = v8427;
                            v8432 = v8429;
                        }
                        let v8434 = (v7934 * v8430) + (v7938 * v8432);
                        let v8436 = if v8434 < v8435 { 1.0 } else { 0.0 };
                        let v8445: f64;
                        if v8436 != 0.0 {
                            let v8438 = v8437 / v8434;
                            v8445 = v8438;
                        } else {
                            let v8444 = v4897 * (v8434 + (((v8434 * v8434) + v8440).sqrt()));
                            v8445 = v8444;
                        }
                        let v8450 = v7098 * (((v7100 + (v7405 * v7821)) * v6852).exp());
                        v11307 = v8381;
                        v11608 = v8445;
                        v11622 = v8450;
                    } else {
                        v11307 = v11304;
                        v11608 = v11605;
                        v11622 = v11619;
                    }
                    let v8451 = if v2834 == v2850 { 1.0 } else { 0.0 };
                    let v8746: f64;
                    let v14752: Lanes<1>;
                    if v8451 != 0.0 {
                        let v15533 = v14606 * v2834;
                        let v8453 = v1 + (v2834 * v6598);
                        v8746 = v8453;
                        v14752 = v15533;
                    } else {
                        let v8454 = if v2866 < v6661 { 1.0 } else { 0.0 };
                        let v8747: f64;
                        let v14753: Lanes<1>;
                        if v8454 != 0.0 {
                            let v8748: f64;
                            let v14754: Lanes<1>;
                            if v7885 != 0.0 {
                                let v15495 = v14606 * v2834;
                                let v8456 = v1 + (v2834 * v6598);
                                let v15496 = v14606 * v2850;
                                let v8461 = v2834 * (v2866 - v6595);
                                let v8462 = (v1 + (v2850 * (v6594 - v2866))) + v8461;
                                let v8465 = v1 + (v2834 * (v6661 - v6595));
                                let v8469 = (v1 + (v2850 * (v6661 - v2866))) + v8461;
                                let v8470 = if v2850 < v2834 { 1.0 } else { 0.0 };
                                let v8749: f64;
                                let v14755: Lanes<1>;
                                if v8470 != 0.0 {
                                    let v8472 = v8456 - v8462;
                                    let v15517 = (v15495 - v15496) * v8472;
                                    let v8475 = (v5721 * v7430) * v7430;
                                    let v8477 = ((v8472 * v8472) + v8475).sqrt();
                                    let v15523 = ((v15495 + v15496) + ((v15517 + v15517) * (v14600 / (v14869 * v8477)))) * v4897;
                                    let v8481 = v8465 - v8469;
                                    let v8488 = ((v4897 * ((v8456 + v8462) + v8477)) - (v4897 * ((v8465 + v8469) + (((v8481 * v8481) + v8475).sqrt())))) + v8465;
                                    let v8490 = v8488 - v8456;
                                    let v15526 = (v15523 - v15495) * v8490;
                                    let v8494 = ((v8490 * v8490) + v8492).sqrt();
                                    let v8496 = v4897 * ((v8488 + v8456) + v8494);
                                    let v15532 = ((v15523 + v15495) + ((v15526 + v15526) * (v14600 / (v14869 * v8494)))) * v4897;
                                    v8749 = v8496;
                                    v14755 = v15532;
                                } else {
                                    let v8498 = v8456 - v8462;
                                    let v15499 = (v15495 - v15496) * v8498;
                                    let v8501 = (v5721 * v7430) * v7430;
                                    let v8503 = ((v8498 * v8498) + v8501).sqrt();
                                    let v15505 = ((v15495 + v15496) - ((v15499 + v15499) * (v14600 / (v14869 * v8503)))) * v4897;
                                    let v8507 = v8465 - v8469;
                                    let v8514 = ((v4897 * ((v8456 + v8462) - v8503)) - (v4897 * ((v8465 + v8469) - (((v8507 * v8507) + v8501).sqrt())))) + v8465;
                                    let v8516 = v8514 - v8456;
                                    let v15508 = (v15505 - v15495) * v8516;
                                    let v8520 = ((v8516 * v8516) + v8518).sqrt();
                                    let v8522 = v4897 * ((v8514 + v8456) - v8520);
                                    let v15514 = ((v15505 + v15495) - ((v15508 + v15508) * (v14600 / (v14869 * v8520)))) * v4897;
                                    v8749 = v8522;
                                    v14755 = v15514;
                                }
                                v8748 = v8749;
                                v14754 = v14755;
                            } else {
                                let v8523 = if v6595 > v2866 { 1.0 } else { 0.0 };
                                let v8750: f64;
                                let v14756: Lanes<1>;
                                if v8523 != 0.0 {
                                    let v15457 = v14606 * v2834;
                                    let v8525 = v1 + (v2834 * v6598);
                                    let v15458 = v14606 * v2850;
                                    let v8529 = v2866 - v6595;
                                    let v8530 = v2834 * v8529;
                                    let v8531 = (v1 + (v2850 * (v6594 - v2866))) + v8530;
                                    let v8533 = (v2834 - v2850) * v8529;
                                    let v8536 = v1 + (v2834 * (v6661 - v6595));
                                    let v8540 = (v1 + (v2850 * (v6661 - v2866))) + v8530;
                                    let v8541 = if v2850 < v2834 { 1.0 } else { 0.0 };
                                    let v8751: f64;
                                    let v14757: Lanes<1>;
                                    if v8541 != 0.0 {
                                        let v8543 = v8525 - v8531;
                                        let v15479 = (v15457 - v15458) * v8543;
                                        let v8546 = (v5721 * v7430) * v7430;
                                        let v8548 = ((v8543 * v8543) + v8546).sqrt();
                                        let v15485 = ((v15457 + v15458) + ((v15479 + v15479) * (v14600 / (v14869 * v8548)))) * v4897;
                                        let v8555 = v4897 * (v8533 + (((v8533 * v8533) + v8546).sqrt()));
                                        let v8556 = (v4897 * ((v8525 + v8531) + v8548)) - v8555;
                                        let v8558 = v8536 - v8540;
                                        let v8567 = ((v4897 * ((v8536 + v8540) + (((v8558 * v8558) + v8546).sqrt()))) - v8555) + (v2834 * (v6594 - v6661));
                                        let v8569 = v8556 - v8567;
                                        let v15488 = (v15485 - v15457) * v8569;
                                        let v8573 = ((v8569 * v8569) + v8571).sqrt();
                                        let v8575 = v4897 * ((v8556 + v8567) + v8573);
                                        let v15494 = ((v15485 + v15457) + ((v15488 + v15488) * (v14600 / (v14869 * v8573)))) * v4897;
                                        v8751 = v8575;
                                        v14757 = v15494;
                                    } else {
                                        let v8577 = v8525 - v8531;
                                        let v15461 = (v15457 - v15458) * v8577;
                                        let v8580 = (v5721 * v7430) * v7430;
                                        let v8582 = ((v8577 * v8577) + v8580).sqrt();
                                        let v15467 = ((v15457 + v15458) - ((v15461 + v15461) * (v14600 / (v14869 * v8582)))) * v4897;
                                        let v8589 = v4897 * (v8533 - (((v8533 * v8533) + v8580).sqrt()));
                                        let v8590 = (v4897 * ((v8525 + v8531) - v8582)) - v8589;
                                        let v8592 = v8536 - v8540;
                                        let v8601 = ((v4897 * ((v8536 + v8540) - (((v8592 * v8592) + v8580).sqrt()))) - v8589) + (v2834 * (v6594 - v6661));
                                        let v8603 = v8590 - v8601;
                                        let v15470 = (v15467 - v15457) * v8603;
                                        let v8607 = ((v8603 * v8603) + v8605).sqrt();
                                        let v8609 = v4897 * ((v8590 + v8601) - v8607);
                                        let v15476 = ((v15467 + v15457) - ((v15470 + v15470) * (v14600 / (v14869 * v8607)))) * v4897;
                                        v8751 = v8609;
                                        v14757 = v15476;
                                    }
                                    v8750 = v8751;
                                    v14756 = v14757;
                                } else {
                                    let v15419 = v14606 * v2850;
                                    let v8611 = v1 + (v2850 * v6598);
                                    let v15420 = v14606 * v2834;
                                    let v8615 = v2866 - v6595;
                                    let v8616 = v2850 * v8615;
                                    let v8617 = (v1 + (v2834 * (v6594 - v2866))) + v8616;
                                    let v8619 = (v2850 - v2834) * v8615;
                                    let v8622 = v1 + (v2850 * (v6661 - v6595));
                                    let v8626 = (v1 + (v2834 * (v6661 - v2866))) + v8616;
                                    let v8627 = if v2850 < v2834 { 1.0 } else { 0.0 };
                                    let v8752: f64;
                                    let v14758: Lanes<1>;
                                    if v8627 != 0.0 {
                                        let v8629 = v8617 - v8611;
                                        let v15441 = (v15420 - v15419) * v8629;
                                        let v8632 = (v5721 * v7430) * v7430;
                                        let v8634 = ((v8629 * v8629) + v8632).sqrt();
                                        let v15447 = ((v15420 + v15419) + ((v15441 + v15441) * (v14600 / (v14869 * v8634)))) * v4897;
                                        let v8641 = v4897 * (v8619 + (((v8619 * v8619) + v8632).sqrt()));
                                        let v8642 = (v4897 * ((v8617 + v8611) + v8634)) - v8641;
                                        let v8644 = v8622 - v8626;
                                        let v8653 = ((v4897 * ((v8622 + v8626) + (((v8644 * v8644) + v8632).sqrt()))) - v8641) + (v2834 * (v6594 - v6661));
                                        let v8655 = v8642 - v8653;
                                        let v15450 = (v15447 - v15420) * v8655;
                                        let v8659 = ((v8655 * v8655) + v8657).sqrt();
                                        let v8661 = v4897 * ((v8642 + v8653) + v8659);
                                        let v15456 = ((v15447 + v15420) + ((v15450 + v15450) * (v14600 / (v14869 * v8659)))) * v4897;
                                        v8752 = v8661;
                                        v14758 = v15456;
                                    } else {
                                        let v8663 = v8617 - v8611;
                                        let v15423 = (v15420 - v15419) * v8663;
                                        let v8666 = (v5721 * v7430) * v7430;
                                        let v8668 = ((v8663 * v8663) + v8666).sqrt();
                                        let v15429 = ((v15420 + v15419) - ((v15423 + v15423) * (v14600 / (v14869 * v8668)))) * v4897;
                                        let v8675 = v4897 * (v8619 - (((v8619 * v8619) + v8666).sqrt()));
                                        let v8676 = (v4897 * ((v8617 + v8611) - v8668)) - v8675;
                                        let v8678 = v8622 - v8626;
                                        let v8687 = ((v4897 * ((v8622 + v8626) - (((v8678 * v8678) + v8666).sqrt()))) - v8675) + (v2834 * (v6594 - v6661));
                                        let v8689 = v8676 - v8687;
                                        let v15432 = (v15429 - v15420) * v8689;
                                        let v8693 = ((v8689 * v8689) + v8691).sqrt();
                                        let v8695 = v4897 * ((v8676 + v8687) - v8693);
                                        let v15438 = ((v15429 + v15420) - ((v15432 + v15432) * (v14600 / (v14869 * v8693)))) * v4897;
                                        v8752 = v8695;
                                        v14758 = v15438;
                                    }
                                    v8750 = v8752;
                                    v14756 = v14758;
                                }
                                v8748 = v8750;
                                v14754 = v14756;
                            }
                            v8747 = v8748;
                            v14753 = v14754;
                        } else {
                            let v8753: f64;
                            let v14759: Lanes<1>;
                            if v7885 != 0.0 {
                                let v15399 = v14606 * v2834;
                                let v8697 = v1 + (v2834 * v6598);
                                let v15400 = v14606 * v2850;
                                let v8703 = (v1 + (v2850 * (v6594 - v6661))) + (v2834 * (v6661 - v6595));
                                let v8704 = if v2850 < v2834 { 1.0 } else { 0.0 };
                                let v8754: f64;
                                let v14760: Lanes<1>;
                                if v8704 != 0.0 {
                                    let v8706 = v8697 - v8703;
                                    let v15412 = (v15399 - v15400) * v8706;
                                    let v8710 = ((v8706 * v8706) + v8708).sqrt();
                                    let v8712 = v4897 * ((v8697 + v8703) + v8710);
                                    let v15418 = ((v15399 + v15400) + ((v15412 + v15412) * (v14600 / (v14869 * v8710)))) * v4897;
                                    v8754 = v8712;
                                    v14760 = v15418;
                                } else {
                                    let v8714 = v8697 - v8703;
                                    let v15403 = (v15399 - v15400) * v8714;
                                    let v8718 = ((v8714 * v8714) + v8716).sqrt();
                                    let v8720 = v4897 * ((v8697 + v8703) - v8718);
                                    let v15409 = ((v15399 + v15400) - ((v15403 + v15403) * (v14600 / (v14869 * v8718)))) * v4897;
                                    v8754 = v8720;
                                    v14760 = v15409;
                                }
                                v8753 = v8754;
                                v14759 = v14760;
                            } else {
                                let v15379 = v14606 * v2850;
                                let v8722 = v1 + (v2850 * v6598);
                                let v15380 = v14606 * v2834;
                                let v8728 = (v1 + (v2834 * (v6594 - v6661))) + (v2850 * (v6661 - v6595));
                                let v8729 = if v2850 < v2834 { 1.0 } else { 0.0 };
                                let v8755: f64;
                                let v14761: Lanes<1>;
                                if v8729 != 0.0 {
                                    let v8731 = v8728 - v8722;
                                    let v15392 = (v15380 - v15379) * v8731;
                                    let v8735 = ((v8731 * v8731) + v8733).sqrt();
                                    let v8737 = v4897 * ((v8728 + v8722) + v8735);
                                    let v15398 = ((v15380 + v15379) + ((v15392 + v15392) * (v14600 / (v14869 * v8735)))) * v4897;
                                    v8755 = v8737;
                                    v14761 = v15398;
                                } else {
                                    let v8739 = v8728 - v8722;
                                    let v15383 = (v15380 - v15379) * v8739;
                                    let v8743 = ((v8739 * v8739) + v8741).sqrt();
                                    let v8745 = v4897 * ((v8728 + v8722) - v8743);
                                    let v15389 = ((v15380 + v15379) - ((v15383 + v15383) * (v14600 / (v14869 * v8743)))) * v4897;
                                    v8755 = v8745;
                                    v14761 = v15389;
                                }
                                v8753 = v8755;
                                v14759 = v14761;
                            }
                            v8747 = v8753;
                            v14753 = v14759;
                        }
                        v8746 = v8747;
                        v14752 = v14753;
                    }
                    let v8756 = v8746 - v25;
                    let v8758 = if v8756 < v8757 { 1.0 } else { 0.0 };
                    let v8767: f64;
                    let v14762: Lanes<1>;
                    if v8758 != 0.0 {
                        let v8760 = v8759 / v8756;
                        let v15543 = ((v14752 * v8760) * v14888) / v8756;
                        v8767 = v8760;
                        v14762 = v15543;
                    } else {
                        let v15534 = v14752 * v8756;
                        let v8764 = ((v8756 * v8756) + v8762).sqrt();
                        let v8766 = v4897 * (v8756 + v8764);
                        let v15540 = (v14752 + ((v15534 + v15534) * (v14600 / (v14869 * v8764)))) * v4897;
                        v8767 = v8766;
                        v14762 = v15540;
                    }
                    let v8776 = v2898 + (v7236 / v3969);
                    let v8781 = v7807 * (v8778 - v7808);
                    let v8783 = v1 + (rspice_limited_exp(v8781));
                    let v8784 = v7806 / v8783;
                    let v15551 = (v14865 * v8776) + (((((v14612 * v7807) * (rspice_limited_exp_derivative(v8781))) * v8784) * v14888) / v8783);
                    let v8791 = ((v8776 * v6597) + v8784) - (v7806 / (v1 + (rspice_limited_exp((v7807 * ((v4897 * ((v6595 + v6661) - (((v8032 * v8032) + v8770).sqrt()))) - v7808))))));
                    v8794 = v8796;
                    v8800 = v8802;
                    v8806 = v8808;
                    v10396 = v10398;
                    v10401 = v8333;
                    v10413 = v10416;
                    v10440 = v10442;
                    v10448 = v10451;
                    v10463 = v10465;
                    v10478 = v10480;
                    v10483 = v7853;
                    v10488 = v10490;
                    v10493 = v7953;
                    v10498 = v10500;
                    v10503 = v8022;
                    v10521 = v10523;
                    v10620 = v8791;
                    v10941 = v8031;
                    v10987 = v8767;
                    v11305 = v11307;
                    v11606 = v11608;
                    v11620 = v11622;
                    v12162 = v8066;
                    v12165 = v8068;
                    v14667 = v14727;
                    v14668 = v14728;
                    v14669 = v14750;
                    v14670 = v14749;
                    v14671 = v14729;
                    v14672 = v14730;
                    v14673 = v14731;
                    v14674 = v14732;
                    v14675 = v14716;
                    v14676 = v15054;
                    v14677 = v14720;
                    v14678 = v14719;
                    v14679 = v14724;
                    v14680 = v15160;
                    v14681 = v14733;
                    v14682 = v15551;
                    v14683 = v15170;
                    v14684 = v14762;
                    v14685 = v14725;
                    v14686 = v14726;
                }
                v8792 = v8794;
                v8798 = v8800;
                v8804 = v8806;
                v10394 = v10396;
                v10400 = v10401;
                v10410 = v10413;
                v10439 = v10440;
                v10445 = v10448;
                v10462 = v10463;
                v10476 = v10478;
                v10482 = v10483;
                v10486 = v10488;
                v10492 = v10493;
                v10496 = v10498;
                v10502 = v10503;
                v10520 = v10521;
                v10619 = v10620;
                v10940 = v10941;
                v10986 = v10987;
                v11302 = v11305;
                v11603 = v11606;
                v11617 = v11620;
                v12161 = v12162;
                v12164 = v12165;
                v14628 = v14667;
                v14629 = v14668;
                v14630 = v14669;
                v14631 = v14670;
                v14632 = v14671;
                v14633 = v14672;
                v14634 = v14673;
                v14635 = v14674;
                v14636 = v14675;
                v14637 = v14676;
                v14638 = v14677;
                v14639 = v14678;
                v14640 = v14679;
                v14641 = v14680;
                v14642 = v14681;
                v14643 = v14682;
                v14644 = v14683;
                v14645 = v14684;
                v14646 = v14685;
                v14647 = v14686;
            }
            let v8797 = if v8792 < v7043 { 1.0 } else { 0.0 };
            let v10457: f64;
            let v14763: Lanes<1>;
            if v8797 != 0.0 {
                v10457 = v7043;
                v14763 = v14864;
            } else {
                v10457 = v8792;
                v14763 = v14628;
            }
            let v8803 = if v8798 < v7043 { 1.0 } else { 0.0 };
            let v10422: f64;
            let v14764: Lanes<1>;
            if v8803 != 0.0 {
                v10422 = v7043;
                v14764 = v14864;
            } else {
                v10422 = v8798;
                v14764 = v14629;
            }
            let v8809 = if v8804 < v7043 { 1.0 } else { 0.0 };
            let v11666: f64;
            if v8809 != 0.0 {
                v11666 = v7043;
            } else {
                v11666 = v8804;
            }
            let v10506: f64;
            let v10512: f64;
            let v11609: f64;
            let v14765: Lanes<1>;
            let v14766: Lanes<1>;
            if v97 != 0.0 {
                let v8810 = if v6977 == v0 { 1.0 } else { 0.0 };
                let v10507: f64;
                let v10513: f64;
                let v11610: f64;
                let v14767: Lanes<1>;
                let v14768: Lanes<1>;
                if v8810 != 0.0 {
                    let v8811 = if v6977 != v0 { 1.0 } else { 0.0 };
                    let v10514: f64;
                    let v14769: Lanes<1>;
                    if v8811 != 0.0 {
                        let v8813 = -v8812;
                        let v16124 = v14606 * v8814;
                        let v8817 = ((v8814 * v6598) - v8813) - v25;
                        let v16125 = v16124 * v8817;
                        let v8822 = ((v8817 * v8817) - ((v22 * v8813) * v25)).sqrt();
                        let v16131 = (v16124 + ((v16125 + v16125) * (v14600 / (v14869 * v8822)))) * v4897;
                        let v8826 = v8812 + (v8813 + (v4897 * (v8817 + v8822)));
                        v10514 = v8826;
                        v14769 = v16131;
                    } else {
                        let v16112 = v14606 * v8814;
                        let v8829 = (v1 + (v8814 * v6598)) - v25;
                        let v8831 = if v8829 < v8830 { 1.0 } else { 0.0 };
                        let v8840: f64;
                        let v14770: Lanes<1>;
                        if v8831 != 0.0 {
                            let v8833 = v8832 / v8829;
                            let v16122 = ((v16112 * v8833) * v14888) / v8829;
                            v8840 = v8833;
                            v14770 = v16122;
                        } else {
                            let v16113 = v16112 * v8829;
                            let v8837 = ((v8829 * v8829) + v8835).sqrt();
                            let v8839 = v4897 * (v8829 + v8837);
                            let v16119 = (v16112 + ((v16113 + v16113) * (v14600 / (v14869 * v8837)))) * v4897;
                            v8840 = v8839;
                            v14770 = v16119;
                        }
                        let v8841 = v8812 * v8840;
                        let v16123 = v14770 * v8812;
                        v10514 = v8841;
                        v14769 = v16123;
                    }
                    let v11611: f64;
                    if v3728 != 0.0 {
                        let v11612: f64;
                        if v8811 != 0.0 {
                            let v8845 = -v8842;
                            let v8851 = ((v8846 * v6598) - v8845) - v25;
                            let v8860 = v8842 + (v8845 + (v4897 * (v8851 + (((v8851 * v8851) - ((v22 * v8845) * v25)).sqrt()))));
                            v11612 = v8860;
                        } else {
                            let v8863 = (v1 + (v8846 * v6598)) - v25;
                            let v8865 = if v8863 < v8864 { 1.0 } else { 0.0 };
                            let v8874: f64;
                            if v8865 != 0.0 {
                                let v8867 = v8866 / v8863;
                                v8874 = v8867;
                            } else {
                                let v8873 = v4897 * (v8863 + (((v8863 * v8863) + v8869).sqrt()));
                                v8874 = v8873;
                            }
                            let v8875 = v8842 * v8874;
                            v11612 = v8875;
                        }
                        v11611 = v11612;
                    } else {
                        v11611 = v0;
                    }
                    let v10508: f64;
                    let v14771: Lanes<1>;
                    if v3342 != 0.0 {
                        let v10509: f64;
                        let v14772: Lanes<1>;
                        if v8811 != 0.0 {
                            let v8878 = -v8876;
                            let v16144 = v14606 * v8879;
                            let v8883 = ((v8879 * v6598) - v8878) - v25;
                            let v16145 = v16144 * v8883;
                            let v8888 = ((v8883 * v8883) - ((v22 * v8878) * v25)).sqrt();
                            let v16151 = (v16144 + ((v16145 + v16145) * (v14600 / (v14869 * v8888)))) * v4897;
                            let v8892 = v8876 + (v8878 + (v4897 * (v8883 + v8888)));
                            v10509 = v8892;
                            v14772 = v16151;
                        } else {
                            let v16132 = v14606 * v8879;
                            let v8895 = (v1 + (v8879 * v6598)) - v25;
                            let v8897 = if v8895 < v8896 { 1.0 } else { 0.0 };
                            let v8906: f64;
                            let v14773: Lanes<1>;
                            if v8897 != 0.0 {
                                let v8899 = v8898 / v8895;
                                let v16142 = ((v16132 * v8899) * v14888) / v8895;
                                v8906 = v8899;
                                v14773 = v16142;
                            } else {
                                let v16133 = v16132 * v8895;
                                let v8903 = ((v8895 * v8895) + v8901).sqrt();
                                let v8905 = v4897 * (v8895 + v8903);
                                let v16139 = (v16132 + ((v16133 + v16133) * (v14600 / (v14869 * v8903)))) * v4897;
                                v8906 = v8905;
                                v14773 = v16139;
                            }
                            let v8907 = v8876 * v8906;
                            let v16143 = v14773 * v8876;
                            v10509 = v8907;
                            v14772 = v16143;
                        }
                        v10508 = v10509;
                        v14771 = v14772;
                    } else {
                        v10508 = v0;
                        v14771 = v14864;
                    }
                    v10507 = v10508;
                    v10513 = v10514;
                    v11610 = v11611;
                    v14767 = v14771;
                    v14768 = v14769;
                } else {
                    let v16110 = v14606 * v8814;
                    let v8909 = v8812 + (v8814 * v6598);
                    let v11613: f64;
                    if v3728 != 0.0 {
                        let v8911 = v8842 + (v8846 * v6598);
                        v11613 = v8911;
                    } else {
                        v11613 = v0;
                    }
                    let v10510: f64;
                    let v14774: Lanes<1>;
                    if v3342 != 0.0 {
                        let v16111 = v14606 * v8879;
                        let v8913 = v8876 + (v8879 * v6598);
                        v10510 = v8913;
                        v14774 = v16111;
                    } else {
                        v10510 = v0;
                        v14774 = v14864;
                    }
                    v10507 = v10510;
                    v10513 = v8909;
                    v11610 = v11613;
                    v14767 = v14774;
                    v14768 = v16110;
                }
                v10506 = v10507;
                v10512 = v10513;
                v11609 = v11610;
                v14765 = v14767;
                v14766 = v14768;
            } else {
                v10506 = v0;
                v10512 = v0;
                v11609 = v0;
                v14765 = v14864;
                v14766 = v14864;
            }
            let v8914 = if v6977 != v0 { 1.0 } else { 0.0 };
            let v10385: f64;
            let v14775: Lanes<1>;
            if v8914 != 0.0 {
                let v8916 = -v8915;
                let v16164 = v14606 * v8917;
                let v8920 = ((v8917 * v6598) - v8916) - v25;
                let v16165 = v16164 * v8920;
                let v8925 = ((v8920 * v8920) - ((v22 * v8916) * v25)).sqrt();
                let v16171 = (v16164 + ((v16165 + v16165) * (v14600 / (v14869 * v8925)))) * v4897;
                let v8929 = v8915 + (v8916 + (v4897 * (v8920 + v8925)));
                v10385 = v8929;
                v14775 = v16171;
            } else {
                let v16152 = v14606 * v8917;
                let v8932 = (v1 + (v8917 * v6598)) - v25;
                let v8934 = if v8932 < v8933 { 1.0 } else { 0.0 };
                let v8943: f64;
                let v14776: Lanes<1>;
                if v8934 != 0.0 {
                    let v8936 = v8935 / v8932;
                    let v16162 = ((v16152 * v8936) * v14888) / v8932;
                    v8943 = v8936;
                    v14776 = v16162;
                } else {
                    let v16153 = v16152 * v8932;
                    let v8940 = ((v8932 * v8932) + v8938).sqrt();
                    let v8942 = v4897 * (v8932 + v8940);
                    let v16159 = (v16152 + ((v16153 + v16153) * (v14600 / (v14869 * v8940)))) * v4897;
                    v8943 = v8942;
                    v14776 = v16159;
                }
                let v8944 = v8915 * v8943;
                let v16163 = v14776 * v8915;
                v10385 = v8944;
                v14775 = v16163;
            }
            let v11290: f64;
            if v3728 != 0.0 {
                let v11291: f64;
                if v8914 != 0.0 {
                    let v8948 = -v8945;
                    let v8952 = ((v8949 * v6598) - v8948) - v25;
                    let v8961 = v8945 + (v8948 + (v4897 * (v8952 + (((v8952 * v8952) - ((v22 * v8948) * v25)).sqrt()))));
                    v11291 = v8961;
                } else {
                    let v8964 = (v1 + (v8949 * v6598)) - v25;
                    let v8966 = if v8964 < v8965 { 1.0 } else { 0.0 };
                    let v8975: f64;
                    if v8966 != 0.0 {
                        let v8968 = v8967 / v8964;
                        v8975 = v8968;
                    } else {
                        let v8974 = v4897 * (v8964 + (((v8964 * v8964) + v8970).sqrt()));
                        v8975 = v8974;
                    }
                    let v8976 = v8945 * v8975;
                    v11291 = v8976;
                }
                v11290 = v11291;
            } else {
                v11290 = v11292;
            }
            let v10383: f64;
            let v14777: Lanes<1>;
            if v8914 != 0.0 {
                let v8978 = -v8977;
                let v16184 = v14606 * v8979;
                let v8982 = ((v8979 * v6598) - v8978) - v25;
                let v16185 = v16184 * v8982;
                let v8987 = ((v8982 * v8982) - ((v22 * v8978) * v25)).sqrt();
                let v16191 = (v16184 + ((v16185 + v16185) * (v14600 / (v14869 * v8987)))) * v4897;
                let v8991 = v8977 + (v8978 + (v4897 * (v8982 + v8987)));
                v10383 = v8991;
                v14777 = v16191;
            } else {
                let v16172 = v14606 * v8979;
                let v8994 = (v1 + (v8979 * v6598)) - v25;
                let v8996 = if v8994 < v8995 { 1.0 } else { 0.0 };
                let v9005: f64;
                let v14778: Lanes<1>;
                if v8996 != 0.0 {
                    let v8998 = v8997 / v8994;
                    let v16182 = ((v16172 * v8998) * v14888) / v8994;
                    v9005 = v8998;
                    v14778 = v16182;
                } else {
                    let v16173 = v16172 * v8994;
                    let v9002 = ((v8994 * v8994) + v9000).sqrt();
                    let v9004 = v4897 * (v8994 + v9002);
                    let v16179 = (v16172 + ((v16173 + v16173) * (v14600 / (v14869 * v9002)))) * v4897;
                    v9005 = v9004;
                    v14778 = v16179;
                }
                let v9006 = v8977 * v9005;
                let v16183 = v14778 * v8977;
                v10383 = v9006;
                v14777 = v16183;
            }
            let v10042: f64;
            let v14779: Lanes<1>;
            if v8914 != 0.0 {
                let v9008 = -v9007;
                let v16204 = v14606 * v2658;
                let v9011 = ((v2658 * v6598) - v9008) - v25;
                let v16205 = v16204 * v9011;
                let v9016 = ((v9011 * v9011) - ((v22 * v9008) * v25)).sqrt();
                let v16211 = (v16204 + ((v16205 + v16205) * (v14600 / (v14869 * v9016)))) * v4897;
                let v9020 = v9007 + (v9008 + (v4897 * (v9011 + v9016)));
                v10042 = v9020;
                v14779 = v16211;
            } else {
                let v16192 = v14606 * v2658;
                let v9023 = (v1 + (v2658 * v6598)) - v25;
                let v9025 = if v9023 < v9024 { 1.0 } else { 0.0 };
                let v9034: f64;
                let v14780: Lanes<1>;
                if v9025 != 0.0 {
                    let v9027 = v9026 / v9023;
                    let v16202 = ((v16192 * v9027) * v14888) / v9023;
                    v9034 = v9027;
                    v14780 = v16202;
                } else {
                    let v16193 = v16192 * v9023;
                    let v9031 = ((v9023 * v9023) + v9029).sqrt();
                    let v9033 = v4897 * (v9023 + v9031);
                    let v16199 = (v16192 + ((v16193 + v16193) * (v14600 / (v14869 * v9031)))) * v4897;
                    v9034 = v9033;
                    v14780 = v16199;
                }
                let v9035 = v9007 * v9034;
                let v16203 = v14780 * v9007;
                v10042 = v9035;
                v14779 = v16203;
            }
            let v10428: f64;
            if v8914 != 0.0 {
                let v9037 = -v9036;
                let v9041 = ((v9038 * v6598) - v9037) - v25;
                let v9050 = v9036 + (v9037 + (v4897 * (v9041 + (((v9041 * v9041) - ((v22 * v9037) * v25)).sqrt()))));
                v10428 = v9050;
            } else {
                let v9053 = (v1 + (v9038 * v6598)) - v25;
                let v9055 = if v9053 < v9054 { 1.0 } else { 0.0 };
                let v9064: f64;
                if v9055 != 0.0 {
                    let v9057 = v9056 / v9053;
                    v9064 = v9057;
                } else {
                    let v9063 = v4897 * (v9053 + (((v9053 * v9053) + v9059).sqrt()));
                    v9064 = v9063;
                }
                let v9065 = v9036 * v9064;
                v10428 = v9065;
            }
            let v10425: f64;
            if v3342 != 0.0 {
                let v10426: f64;
                if v8914 != 0.0 {
                    let v9067 = -v9066;
                    let v9070 = ((v9038 * v6598) - v9067) - v25;
                    let v9079 = v9066 + (v9067 + (v4897 * (v9070 + (((v9070 * v9070) - ((v22 * v9067) * v25)).sqrt()))));
                    v10426 = v9079;
                } else {
                    let v9082 = (v1 + (v9038 * v6598)) - v25;
                    let v9084 = if v9082 < v9083 { 1.0 } else { 0.0 };
                    let v9093: f64;
                    if v9084 != 0.0 {
                        let v9086 = v9085 / v9082;
                        v9093 = v9086;
                    } else {
                        let v9092 = v4897 * (v9082 + (((v9082 * v9082) + v9088).sqrt()));
                        v9093 = v9092;
                    }
                    let v9094 = v9066 * v9093;
                    v10426 = v9094;
                }
                v10425 = v10426;
            } else {
                v10425 = v0;
            }
            let v10434: f64;
            if v8914 != 0.0 {
                let v9096 = -v9095;
                let v9100 = ((v9097 * v6598) - v9096) - v25;
                let v9109 = v9095 + (v9096 + (v4897 * (v9100 + (((v9100 * v9100) - ((v22 * v9096) * v25)).sqrt()))));
                v10434 = v9109;
            } else {
                let v9112 = (v1 + (v9097 * v6598)) - v25;
                let v9114 = if v9112 < v9113 { 1.0 } else { 0.0 };
                let v9123: f64;
                if v9114 != 0.0 {
                    let v9116 = v9115 / v9112;
                    v9123 = v9116;
                } else {
                    let v9122 = v4897 * (v9112 + (((v9112 * v9112) + v9118).sqrt()));
                    v9123 = v9122;
                }
                let v9124 = v9095 * v9123;
                v10434 = v9124;
            }
            let v10431: f64;
            if v3342 != 0.0 {
                let v10432: f64;
                if v8914 != 0.0 {
                    let v9126 = -v9125;
                    let v9129 = ((v9097 * v6598) - v9126) - v25;
                    let v9138 = v9125 + (v9126 + (v4897 * (v9129 + (((v9129 * v9129) - ((v22 * v9126) * v25)).sqrt()))));
                    v10432 = v9138;
                } else {
                    let v9141 = (v1 + (v9097 * v6598)) - v25;
                    let v9143 = if v9141 < v9142 { 1.0 } else { 0.0 };
                    let v9152: f64;
                    if v9143 != 0.0 {
                        let v9145 = v9144 / v9141;
                        v9152 = v9145;
                    } else {
                        let v9151 = v4897 * (v9141 + (((v9141 * v9141) + v9147).sqrt()));
                        v9152 = v9151;
                    }
                    let v9153 = v9125 * v9152;
                    v10432 = v9153;
                }
                v10431 = v10432;
            } else {
                v10431 = v0;
            }
            let v10407: f64;
            let v14781: Lanes<1>;
            if v8914 != 0.0 {
                let v9155 = -v9154;
                let v9156 = -v2786;
                let v16224 = v14606 * v9156;
                let v9159 = ((v9156 * v6598) - v9155) - v25;
                let v16225 = v16224 * v9159;
                let v9164 = ((v9159 * v9159) - ((v22 * v9155) * v25)).sqrt();
                let v16231 = (v16224 + ((v16225 + v16225) * (v14600 / (v14869 * v9164)))) * v4897;
                let v9168 = v9154 + (v9155 + (v4897 * (v9159 + v9164)));
                v10407 = v9168;
                v14781 = v16231;
            } else {
                let v9169 = -v2786;
                let v16212 = v14606 * v9169;
                let v9172 = (v1 + (v9169 * v6598)) - v25;
                let v9174 = if v9172 < v9173 { 1.0 } else { 0.0 };
                let v9183: f64;
                let v14782: Lanes<1>;
                if v9174 != 0.0 {
                    let v9176 = v9175 / v9172;
                    let v16222 = ((v16212 * v9176) * v14888) / v9172;
                    v9183 = v9176;
                    v14782 = v16222;
                } else {
                    let v16213 = v16212 * v9172;
                    let v9180 = ((v9172 * v9172) + v9178).sqrt();
                    let v9182 = v4897 * (v9172 + v9180);
                    let v16219 = (v16212 + ((v16213 + v16213) * (v14600 / (v14869 * v9180)))) * v4897;
                    v9183 = v9182;
                    v14782 = v16219;
                }
                let v9184 = v9154 * v9183;
                let v16223 = v14782 * v9154;
                v10407 = v9184;
                v14781 = v16223;
            }
            let v10404: f64;
            let v14783: Lanes<1>;
            if v3342 != 0.0 {
                let v10405: f64;
                let v14784: Lanes<1>;
                if v8914 != 0.0 {
                    let v9186 = -v9185;
                    let v9187 = -v2786;
                    let v16244 = v14606 * v9187;
                    let v9190 = ((v9187 * v6598) - v9186) - v25;
                    let v16245 = v16244 * v9190;
                    let v9195 = ((v9190 * v9190) - ((v22 * v9186) * v25)).sqrt();
                    let v16251 = (v16244 + ((v16245 + v16245) * (v14600 / (v14869 * v9195)))) * v4897;
                    let v9199 = v9185 + (v9186 + (v4897 * (v9190 + v9195)));
                    v10405 = v9199;
                    v14784 = v16251;
                } else {
                    let v9200 = -v2786;
                    let v16232 = v14606 * v9200;
                    let v9203 = (v1 + (v9200 * v6598)) - v25;
                    let v9205 = if v9203 < v9204 { 1.0 } else { 0.0 };
                    let v9214: f64;
                    let v14785: Lanes<1>;
                    if v9205 != 0.0 {
                        let v9207 = v9206 / v9203;
                        let v16242 = ((v16232 * v9207) * v14888) / v9203;
                        v9214 = v9207;
                        v14785 = v16242;
                    } else {
                        let v16233 = v16232 * v9203;
                        let v9211 = ((v9203 * v9203) + v9209).sqrt();
                        let v9213 = v4897 * (v9203 + v9211);
                        let v16239 = (v16232 + ((v16233 + v16233) * (v14600 / (v14869 * v9211)))) * v4897;
                        v9214 = v9213;
                        v14785 = v16239;
                    }
                    let v9215 = v9185 * v9214;
                    let v16243 = v14785 * v9185;
                    v10405 = v9215;
                    v14784 = v16243;
                }
                v10404 = v10405;
                v14783 = v14784;
            } else {
                v10404 = v0;
                v14783 = v14864;
            }
            let v9218 = v2274 * ((v2930 * v6852).exp());
            let v9223 = if ((v1 + (v2418 * v6597)) - v4536) < v9222 { 1.0 } else { 0.0 };
            if v9223 != 0.0 {
            } else {
            }
            let v16252 = v14606 * v706;
            let v9225 = v690 + (v706 * v6598);
            let v9227 = -v9226;
            let v16253 = v14606 * v738;
            let v9230 = ((v738 * v6598) - v9227) - v25;
            let v16254 = v16253 * v9230;
            let v9235 = ((v9230 * v9230) - ((v22 * v9227) * v25)).sqrt();
            let v16260 = (v16253 + ((v16254 + v16254) * (v14600 / (v14869 * v9235)))) * v4897;
            let v9239 = v9226 + (v9227 + (v4897 * (v9230 + v9235)));
            let v9241 = -v9240;
            let v16261 = v14606 * v770;
            let v9244 = ((v770 * v6598) - v9241) - v25;
            let v16262 = v16261 * v9244;
            let v9249 = ((v9244 * v9244) - ((v22 * v9241) * v25)).sqrt();
            let v16268 = (v16261 + ((v16262 + v16262) * (v14600 / (v14869 * v9249)))) * v4897;
            let v9253 = v9240 + (v9241 + (v4897 * (v9244 + v9249)));
            let v9256 = -v9254;
            let v16269 = v14606 * v9257;
            let v9260 = ((v9257 * v6598) - v9256) - v25;
            let v16270 = v16269 * v9260;
            let v9265 = ((v9260 * v9260) - ((v22 * v9256) * v25)).sqrt();
            let v16276 = (v16269 + ((v16270 + v16270) * (v14600 / (v14869 * v9265)))) * v4897;
            let v9269 = v9254 + (v9256 + (v4897 * (v9260 + v9265)));
            let v16277 = v14606 * v9272;
            let v9275 = v9270 + (v9272 * v6598);
            let v16278 = v14606 * v1330;
            let v9277 = v1314 + (v1330 * v6598);
            let v16279 = v14606 * v1362;
            let v9279 = v1346 + (v1362 * v6598);
            let v9282 = -v9280;
            let v16280 = v14606 * v9283;
            let v9287 = ((v9283 * v6598) - v9282) - v25;
            let v16281 = v16280 * v9287;
            let v9292 = ((v9287 * v9287) - ((v22 * v9282) * v25)).sqrt();
            let v16287 = (v16280 + ((v16281 + v16281) * (v14600 / (v14869 * v9292)))) * v4897;
            let v9296 = v9280 + (v9282 + (v4897 * (v9287 + v9292)));
            let v16288 = v14606 * v802;
            let v9298 = v786 + (v802 * v6598);
            let v16289 = v14606 * v834;
            let v9300 = v818 + (v834 * v6598);
            let v9301 = -v1538;
            let v9304 = ((v1554 * v6598) - v9301) - v25;
            let v9313 = v1538 + (v9301 + (v4897 * (v9304 + (((v9304 * v9304) - ((v22 * v9301) * v25)).sqrt()))));
            let v9314 = -v1634;
            let v9317 = ((v1650 * v6598) - v9314) - v25;
            let v9326 = v1634 + (v9314 + (v4897 * (v9317 + (((v9317 * v9317) - ((v22 * v9314) * v25)).sqrt()))));
            let v9327 = -v1714;
            let v9330 = ((v1730 * v6598) - v9327) - v25;
            let v9339 = v1714 + (v9327 + (v4897 * (v9330 + (((v9330 * v9330) - ((v22 * v9327) * v25)).sqrt()))));
            let v9340 = -v1794;
            let v9343 = ((v1810 * v6598) - v9340) - v25;
            let v9352 = v1794 + (v9340 + (v4897 * (v9343 + (((v9343 * v9343) - ((v22 * v9340) * v25)).sqrt()))));
            let v9353 = -v1858;
            let v9356 = ((v1874 * v6598) - v9353) - v25;
            let v9365 = v1858 + (v9353 + (v4897 * (v9356 + (((v9356 * v9356) - ((v22 * v9353) * v25)).sqrt()))));
            let v9368 = (v1 + (v2946 * v6598)) - v25;
            let v9370 = if v9368 < v9369 { 1.0 } else { 0.0 };
            let v9379: f64;
            if v9370 != 0.0 {
                let v9372 = v9371 / v9368;
                v9379 = v9372;
            } else {
                let v9378 = v4897 * (v9368 + (((v9368 * v9368) + v9374).sqrt()));
                v9379 = v9378;
            }
            let v9380 = v1970 * v9379;
            let v9382 = if v9368 < v9381 { 1.0 } else { 0.0 };
            let v9391: f64;
            if v9382 != 0.0 {
                let v9384 = v9383 / v9368;
                v9391 = v9384;
            } else {
                let v9390 = v4897 * (v9368 + (((v9368 * v9368) + v9386).sqrt()));
                v9391 = v9390;
            }
            let v9392 = v2098 * v9391;
            let v9393 = -v2210;
            let v9397 = ((v9394 * v6598) - v9393) - v25;
            let v9406 = v2210 + (v9393 + (v4897 * (v9397 + (((v9397 * v9397) - ((v22 * v9393) * v25)).sqrt()))));
            let v9407 = -v2226;
            let v9411 = ((v9408 * v6598) - v9407) - v25;
            let v9420 = v2226 + (v9407 + (v4897 * (v9411 + (((v9411 * v9411) - ((v22 * v9407) * v25)).sqrt()))));
            let v9421 = -v2242;
            let v9426 = ((v9422 * v6598) - v9421) - v9425;
            let v9435 = v2242 + (v9421 + (v4897 * (v9426 + (((v9426 * v9426) - ((v22 * v9421) * v9425)).sqrt()))));
            let v9436 = -v2258;
            let v9441 = ((v9437 * v6598) - v9436) - v9440;
            let v9450 = v2258 + (v9436 + (v4897 * (v9441 + (((v9441 * v9441) - ((v22 * v9436) * v9440)).sqrt()))));
            let v9452 = (v2978 * v6852).exp();
            let v9453 = v5673 * v9452;
            let v10048: f64;
            let v10051: f64;
            let v10055: f64;
            let v10117: f64;
            let v10120: f64;
            let v10123: f64;
            let v10183: f64;
            let v10186: f64;
            let v10189: f64;
            let v10193: f64;
            let v10196: f64;
            let v10199: f64;
            let v10205: f64;
            let v10224: f64;
            let v10243: f64;
            let v10262: f64;
            let v10281: f64;
            let v10300: f64;
            let v13460: f64;
            let v13466: f64;
            let v13472: f64;
            let v13499: f64;
            let v13505: f64;
            let v13511: f64;
            if v97 != 0.0 {
                let v10184: f64;
                if v8914 != 0.0 {
                    let v9455 = -v9454;
                    let v9459 = ((v9456 * v6598) - v9455) - v25;
                    let v9468 = v9454 + (v9455 + (v4897 * (v9459 + (((v9459 * v9459) - ((v22 * v9455) * v25)).sqrt()))));
                    v10184 = v9468;
                } else {
                    let v9471 = (v1 + (v9456 * v6598)) - v25;
                    let v9473 = if v9471 < v9472 { 1.0 } else { 0.0 };
                    let v9482: f64;
                    if v9473 != 0.0 {
                        let v9475 = v9474 / v9471;
                        v9482 = v9475;
                    } else {
                        let v9481 = v4897 * (v9471 + (((v9471 * v9471) + v9477).sqrt()));
                        v9482 = v9481;
                    }
                    let v9483 = v9454 * v9482;
                    v10184 = v9483;
                }
                let v10194: f64;
                if v8914 != 0.0 {
                    let v9485 = -v9484;
                    let v9488 = ((v9456 * v6598) - v9485) - v25;
                    let v9497 = v9484 + (v9485 + (v4897 * (v9488 + (((v9488 * v9488) - ((v22 * v9485) * v25)).sqrt()))));
                    v10194 = v9497;
                } else {
                    let v9500 = (v1 + (v9456 * v6598)) - v25;
                    let v9502 = if v9500 < v9501 { 1.0 } else { 0.0 };
                    let v9511: f64;
                    if v9502 != 0.0 {
                        let v9504 = v9503 / v9500;
                        v9511 = v9504;
                    } else {
                        let v9510 = v4897 * (v9500 + (((v9500 * v9500) + v9506).sqrt()));
                        v9511 = v9510;
                    }
                    let v9512 = v9484 * v9511;
                    v10194 = v9512;
                }
                let v10187: f64;
                if v8914 != 0.0 {
                    let v9514 = -v9513;
                    let v9518 = ((v9515 * v6598) - v9514) - v25;
                    let v9527 = v9513 + (v9514 + (v4897 * (v9518 + (((v9518 * v9518) - ((v22 * v9514) * v25)).sqrt()))));
                    v10187 = v9527;
                } else {
                    let v9530 = (v1 + (v9515 * v6598)) - v25;
                    let v9532 = if v9530 < v9531 { 1.0 } else { 0.0 };
                    let v9541: f64;
                    if v9532 != 0.0 {
                        let v9534 = v9533 / v9530;
                        v9541 = v9534;
                    } else {
                        let v9540 = v4897 * (v9530 + (((v9530 * v9530) + v9536).sqrt()));
                        v9541 = v9540;
                    }
                    let v9542 = v9513 * v9541;
                    v10187 = v9542;
                }
                let v10197: f64;
                if v8914 != 0.0 {
                    let v9544 = -v9543;
                    let v9547 = ((v9515 * v6598) - v9544) - v25;
                    let v9556 = v9543 + (v9544 + (v4897 * (v9547 + (((v9547 * v9547) - ((v22 * v9544) * v25)).sqrt()))));
                    v10197 = v9556;
                } else {
                    let v9559 = (v1 + (v9515 * v6598)) - v25;
                    let v9561 = if v9559 < v9560 { 1.0 } else { 0.0 };
                    let v9570: f64;
                    if v9561 != 0.0 {
                        let v9563 = v9562 / v9559;
                        v9570 = v9563;
                    } else {
                        let v9569 = v4897 * (v9559 + (((v9559 * v9559) + v9565).sqrt()));
                        v9570 = v9569;
                    }
                    let v9571 = v9543 * v9570;
                    v10197 = v9571;
                }
                let v10190: f64;
                if v8914 != 0.0 {
                    let v9573 = -v9572;
                    let v9577 = ((v9574 * v6598) - v9573) - v25;
                    let v9586 = v9572 + (v9573 + (v4897 * (v9577 + (((v9577 * v9577) - ((v22 * v9573) * v25)).sqrt()))));
                    v10190 = v9586;
                } else {
                    let v9589 = (v1 + (v9574 * v6598)) - v25;
                    let v9591 = if v9589 < v9590 { 1.0 } else { 0.0 };
                    let v9600: f64;
                    if v9591 != 0.0 {
                        let v9593 = v9592 / v9589;
                        v9600 = v9593;
                    } else {
                        let v9599 = v4897 * (v9589 + (((v9589 * v9589) + v9595).sqrt()));
                        v9600 = v9599;
                    }
                    let v9601 = v9572 * v9600;
                    v10190 = v9601;
                }
                let v10200: f64;
                if v8914 != 0.0 {
                    let v9603 = -v9602;
                    let v9606 = ((v9574 * v6598) - v9603) - v25;
                    let v9615 = v9602 + (v9603 + (v4897 * (v9606 + (((v9606 * v9606) - ((v22 * v9603) * v25)).sqrt()))));
                    v10200 = v9615;
                } else {
                    let v9618 = (v1 + (v9574 * v6598)) - v25;
                    let v9620 = if v9618 < v9619 { 1.0 } else { 0.0 };
                    let v9629: f64;
                    if v9620 != 0.0 {
                        let v9622 = v9621 / v9618;
                        v9629 = v9622;
                    } else {
                        let v9628 = v4897 * (v9618 + (((v9618 * v9618) + v9624).sqrt()));
                        v9629 = v9628;
                    }
                    let v9630 = v9602 * v9629;
                    v10200 = v9630;
                }
                let v9633 = v9632 * v6598;
                let v9635 = (v9631 - v9633) - v4536;
                let v9637 = if v9635 < v9636 { 1.0 } else { 0.0 };
                let v9646: f64;
                if v9637 != 0.0 {
                    let v9639 = v9638 / v9635;
                    v9646 = v9639;
                } else {
                    let v9645 = v4897 * (v9635 + (((v9635 * v9635) + v9641).sqrt()));
                    v9646 = v9645;
                }
                let v9647 = v9646 + v4536;
                let v9650 = (v9648 - v9633) - v4536;
                let v9652 = if v9650 < v9651 { 1.0 } else { 0.0 };
                let v9661: f64;
                if v9652 != 0.0 {
                    let v9654 = v9653 / v9650;
                    v9661 = v9654;
                } else {
                    let v9660 = v4897 * (v9650 + (((v9650 * v9650) + v9656).sqrt()));
                    v9661 = v9660;
                }
                let v9662 = v9661 + v4536;
                let v9665 = v9664 * v6598;
                let v9667 = (v9663 - v9665) - v4536;
                let v9669 = if v9667 < v9668 { 1.0 } else { 0.0 };
                let v9678: f64;
                if v9669 != 0.0 {
                    let v9671 = v9670 / v9667;
                    v9678 = v9671;
                } else {
                    let v9677 = v4897 * (v9667 + (((v9667 * v9667) + v9673).sqrt()));
                    v9678 = v9677;
                }
                let v9679 = v9678 + v4536;
                let v9682 = (v9680 - v9665) - v4536;
                let v9684 = if v9682 < v9683 { 1.0 } else { 0.0 };
                let v9693: f64;
                if v9684 != 0.0 {
                    let v9686 = v9685 / v9682;
                    v9693 = v9686;
                } else {
                    let v9692 = v4897 * (v9682 + (((v9682 * v9682) + v9688).sqrt()));
                    v9693 = v9692;
                }
                let v9694 = v9693 + v4536;
                let v9697 = v9696 * v6598;
                let v9699 = (v9695 - v9697) - v4536;
                let v9701 = if v9699 < v9700 { 1.0 } else { 0.0 };
                let v9710: f64;
                if v9701 != 0.0 {
                    let v9703 = v9702 / v9699;
                    v9710 = v9703;
                } else {
                    let v9709 = v4897 * (v9699 + (((v9699 * v9699) + v9705).sqrt()));
                    v9710 = v9709;
                }
                let v9711 = v9710 + v4536;
                let v9714 = (v9712 - v9697) - v4536;
                let v9716 = if v9714 < v9715 { 1.0 } else { 0.0 };
                let v9725: f64;
                if v9716 != 0.0 {
                    let v9718 = v9717 / v9714;
                    v9725 = v9718;
                } else {
                    let v9724 = v4897 * (v9714 + (((v9714 * v9714) + v9720).sqrt()));
                    v9725 = v9724;
                }
                let v9726 = v9725 + v4536;
                let v9729 = (v6765 / v6601) - (v6760 / v6600);
                let v9735 = rspice_limited_exp(((v9729 + (v9730 * v6852)) / v9733));
                let v9737 = v9736 * v9735;
                let v9739 = v9738 * v9735;
                let v9741 = v9740 * v9735;
                let v9747 = rspice_limited_exp(((v9729 + (v9742 * v6852)) / v9745));
                let v9749 = v9748 * v9747;
                let v9751 = v9750 * v9747;
                let v9753 = v9752 * v9747;
                let v9760 = v9754 * (rspice_limited_exp((((v6765 * v9755) * v6597) / v6600)));
                let v9767 = v9761 * (rspice_limited_exp((((v6765 * v9762) * v6597) / v6600)));
                let v9774 = v9768 * (rspice_limited_exp((((v6765 * v9769) * v6597) / v6600)));
                let v9781 = v9775 * (rspice_limited_exp((((v6765 * v9776) * v6597) / v6600)));
                let v9786 = ((v9783 / v423).sqrt()) + v1;
                let v9793 = (v9782 * v9786) * (rspice_limited_exp((((v6765 * v9788) * v6597) / v6600)));
                let v9801 = (v9794 * v9786) * (rspice_limited_exp((((v6765 * v9796) * v6597) / v6600)));
                let v9809 = if ((v9802 * (v1 + (v9803 * v6597))) - v4536) < v9808 { 1.0 } else { 0.0 };
                if v9809 != 0.0 {
                } else {
                }
                let v9817 = if ((v9810 * (v1 + (v9811 * v6597))) - v4536) < v9816 { 1.0 } else { 0.0 };
                if v9817 != 0.0 {
                } else {
                }
                let v9825 = if ((v9818 * (v1 + (v9819 * v6597))) - v4536) < v9824 { 1.0 } else { 0.0 };
                if v9825 != 0.0 {
                } else {
                }
                let v9833 = if ((v9826 * (v1 + (v9827 * v6597))) - v4536) < v9832 { 1.0 } else { 0.0 };
                if v9833 != 0.0 {
                } else {
                }
                let v9841 = if ((v9834 * (v1 + (v9835 * v6597))) - v4536) < v9840 { 1.0 } else { 0.0 };
                if v9841 != 0.0 {
                } else {
                }
                let v9849 = if ((v9842 * (v1 + (v9843 * v6597))) - v4536) < v9848 { 1.0 } else { 0.0 };
                if v9849 != 0.0 {
                } else {
                }
                v10048 = v9737;
                v10051 = v9739;
                v10055 = v9741;
                v10117 = v9749;
                v10120 = v9751;
                v10123 = v9753;
                v10183 = v10184;
                v10186 = v10187;
                v10189 = v10190;
                v10193 = v10194;
                v10196 = v10197;
                v10199 = v10200;
                v10205 = v9647;
                v10224 = v9679;
                v10243 = v9711;
                v10262 = v9662;
                v10281 = v9694;
                v10300 = v9726;
                v13460 = v9760;
                v13466 = v9774;
                v13472 = v9793;
                v13499 = v9767;
                v13505 = v9781;
                v13511 = v9801;
            } else {
                v10048 = v0;
                v10051 = v0;
                v10055 = v0;
                v10117 = v0;
                v10120 = v0;
                v10123 = v0;
                v10183 = v0;
                v10186 = v0;
                v10189 = v0;
                v10193 = v0;
                v10196 = v0;
                v10199 = v0;
                v10205 = v0;
                v10224 = v0;
                v10243 = v0;
                v10262 = v0;
                v10281 = v0;
                v10300 = v0;
                v13460 = v0;
                v13466 = v0;
                v13472 = v0;
                v13499 = v0;
                v13505 = v0;
                v13511 = v0;
            }
            let v9851 = if v9850 == 0.0 { 1.0 } else { 0.0 };
            let v9994: f64;
            if v9851 != 0.0 {
                let v9852 = if v451 > v0 { 1.0 } else { 0.0 };
                let v9995: f64;
                if v9852 != 0.0 {
                    let v9996: f64;
                    if v6778 != 0.0 {
                        let v9853 = v451 / v6777;
                        let v9854 = if v9853 > v185 { 1.0 } else { 0.0 };
                        let v9858: f64;
                        if v9854 != 0.0 {
                            let v9855 = v9853.ln();
                            v9858 = v9855;
                        } else {
                            v9858 = v9856;
                        }
                        let v9857 = v4897 * v6760;
                        let v9860 = v9857 - (v6600 * v9858);
                        let v9862 = if v9860 < v9861 { 1.0 } else { 0.0 };
                        let v9889: f64;
                        if v9862 != 0.0 {
                            let v9864 = v9863 / v9860;
                            v9889 = v9864;
                        } else {
                            let v9870 = v4897 * (v9860 + (((v9860 * v9860) + v9866).sqrt()));
                            v9889 = v9870;
                        }
                        let v9871 = v4750 / v6777;
                        let v9872 = if v9871 > v185 { 1.0 } else { 0.0 };
                        let v9875: f64;
                        if v9872 != 0.0 {
                            let v9873 = v9871.ln();
                            v9875 = v9873;
                        } else {
                            v9875 = v9874;
                        }
                        let v9877 = v9857 - (v6600 * v9875);
                        let v9879 = if v9877 < v9878 { 1.0 } else { 0.0 };
                        let v9890: f64;
                        if v9879 != 0.0 {
                            let v9881 = v9880 / v9877;
                            v9890 = v9881;
                        } else {
                            let v9887 = v4897 * (v9877 + (((v9877 * v9877) + v9883).sqrt()));
                            v9890 = v9887;
                        }
                        let v9895 = v9888 * (v9889 - (v9857 - (v9888 * (v9857 - v9890))));
                        v9996 = v9895;
                    } else {
                        let v9896 = if v451 > v185 { 1.0 } else { 0.0 };
                        let v9900: f64;
                        if v9896 != 0.0 {
                            let v9897 = v451.ln();
                            v9900 = v9897;
                        } else {
                            v9900 = v9898;
                        }
                        let v9899 = v4897 * v6760;
                        let v9904 = v9899 - (v6600 * (v9900 - v9901));
                        let v9906 = if v9904 < v9905 { 1.0 } else { 0.0 };
                        let v9932: f64;
                        if v9906 != 0.0 {
                            let v9908 = v9907 / v9904;
                            v9932 = v9908;
                        } else {
                            let v9914 = v4897 * (v9904 + (((v9904 * v9904) + v9910).sqrt()));
                            v9932 = v9914;
                        }
                        let v9915 = if v4750 > v185 { 1.0 } else { 0.0 };
                        let v9918: f64;
                        if v9915 != 0.0 {
                            let v9916 = v4750.ln();
                            v9918 = v9916;
                        } else {
                            v9918 = v9917;
                        }
                        let v9921 = v9899 - (v6600 * (v9918 - v9901));
                        let v9923 = if v9921 < v9922 { 1.0 } else { 0.0 };
                        let v9933: f64;
                        if v9923 != 0.0 {
                            let v9925 = v9924 / v9921;
                            v9933 = v9925;
                        } else {
                            let v9931 = v4897 * (v9921 + (((v9921 * v9921) + v9927).sqrt()));
                            v9933 = v9931;
                        }
                        let v9938 = v9888 * (v9932 - (v9899 - (v9888 * (v9899 - v9933))));
                        v9996 = v9938;
                    }
                    v9995 = v9996;
                } else {
                    let v9997: f64;
                    if v6778 != 0.0 {
                        let v9939 = v4750 / v6777;
                        let v9940 = if v9939 > v185 { 1.0 } else { 0.0 };
                        let v9944: f64;
                        if v9940 != 0.0 {
                            let v9941 = v9939.ln();
                            v9944 = v9941;
                        } else {
                            v9944 = v9942;
                        }
                        let v9943 = v4897 * v6760;
                        let v9946 = v9943 - (v6600 * v9944);
                        let v9948 = if v9946 < v9947 { 1.0 } else { 0.0 };
                        let v9960: f64;
                        if v9948 != 0.0 {
                            let v9950 = v9949 / v9946;
                            v9960 = v9950;
                        } else {
                            let v9956 = v4897 * (v9946 + (((v9946 * v9946) + v9952).sqrt()));
                            v9960 = v9956;
                        }
                        let v9965 = v9888 * (v9957 - ((v9958 + v9943) - (v9888 * (v9943 - v9960))));
                        v9997 = v9965;
                    } else {
                        let v9966 = if v4750 > v185 { 1.0 } else { 0.0 };
                        let v9970: f64;
                        if v9966 != 0.0 {
                            let v9967 = v4750.ln();
                            v9970 = v9967;
                        } else {
                            v9970 = v9968;
                        }
                        let v9969 = v4897 * v6760;
                        let v9973 = v9969 - (v6600 * (v9970 - v9901));
                        let v9975 = if v9973 < v9974 { 1.0 } else { 0.0 };
                        let v9985: f64;
                        if v9975 != 0.0 {
                            let v9977 = v9976 / v9973;
                            v9985 = v9977;
                        } else {
                            let v9983 = v4897 * (v9973 + (((v9973 * v9973) + v9979).sqrt()));
                            v9985 = v9983;
                        }
                        let v9990 = v9888 * (v9957 - ((v9958 + v9969) - (v9888 * (v9969 - v9985))));
                        v9997 = v9990;
                    }
                    v9995 = v9997;
                }
                v9994 = v9995;
            } else {
                v9994 = v9991;
            }
            let v9993 = if v9992 == 0.0 { 1.0 } else { 0.0 };
            if v9993 != 0.0 {
            } else {
            }
            let v10529: f64;
            let v10596: f64;
            let v14786: Lanes<1>;
            let v14787: Lanes<1>;
            if v6778 != 0.0 {
                let v9998 = v417 / v6777;
                let v16308 = ((v14963 * v9998) * v14888) / v6777;
                let v9999 = if v9998 > v185 { 1.0 } else { 0.0 };
                let v10002: f64;
                let v14788: Lanes<1>;
                if v9999 != 0.0 {
                    let v10000 = v9998.ln();
                    let v16310 = v16308 * (v14600 / v9998);
                    v10002 = v10000;
                    v14788 = v16310;
                } else {
                    v10002 = v10001;
                    v14788 = v14864;
                }
                let v10003 = v6600 * v10002;
                let v16313 = (v14866 * v10002) + (v14788 * v6600);
                let v16314 = v16313 * v10003;
                let v10007 = ((v10003 * v10003) + v10005).sqrt();
                let v10009 = v4897 * (v10003 + v10007);
                let v16320 = (v16313 + ((v16314 + v16314) * (v14600 / (v14869 * v10007)))) * v4897;
                let v10011 = v6777 * v6777;
                let v16321 = v14963 * v6777;
                let v10012 = (v417 * v4750) / v10011;
                let v16325 = (((v16321 + v16321) * v10012) * v14888) / v10011;
                let v10013 = if v10012 > v185 { 1.0 } else { 0.0 };
                let v10016: f64;
                let v14789: Lanes<1>;
                if v10013 != 0.0 {
                    let v10014 = v10012.ln();
                    let v16327 = v16325 * (v14600 / v10012);
                    v10016 = v10014;
                    v14789 = v16327;
                } else {
                    v10016 = v10015;
                    v14789 = v14864;
                }
                let v10017 = v6600 * v10016;
                let v16330 = (v14866 * v10016) + (v14789 * v6600);
                v10529 = v10009;
                v10596 = v10017;
                v14786 = v16320;
                v14787 = v16330;
            } else {
                let v10018 = if v417 > v185 { 1.0 } else { 0.0 };
                let v10021: f64;
                if v10018 != 0.0 {
                    let v10019 = v417.ln();
                    v10021 = v10019;
                } else {
                    v10021 = v10020;
                }
                let v10022 = v10021 - v9901;
                let v10023 = v6600 * v10022;
                let v16293 = (v14866 * v10022) + ((v14624 * v14888) * v6600);
                let v16294 = v16293 * v10023;
                let v10027 = ((v10023 * v10023) + v10025).sqrt();
                let v10029 = v4897 * (v10023 + v10027);
                let v16300 = (v16293 + ((v16294 + v16294) * (v14600 / (v14869 * v10027)))) * v4897;
                let v10030 = v417 * v4750;
                let v10031 = if v10030 > v185 { 1.0 } else { 0.0 };
                let v10034: f64;
                if v10031 != 0.0 {
                    let v10032 = v10030.ln();
                    v10034 = v10032;
                } else {
                    v10034 = v10033;
                }
                let v10036 = v10034 - (v23 * v9901);
                let v10037 = v6600 * v10036;
                let v16305 = (v14866 * v10036) + (((v14624 * v23) * v14888) * v6600);
                v10529 = v10029;
                v10596 = v10037;
                v14786 = v16300;
                v14787 = v16305;
            }
            let v10038: f64;
            let v14790: Lanes<1>;
            if v12 != 0.0 {
                v10038 = v0;
                v14790 = v14864;
            } else {
                v10038 = v6760;
                v14790 = v14945;
            }
            let v10041 = v9888 * (v9957 - (v9958 + v10038));
            let v16332 = (v14790 * v14888) * v9888;
            let v10043 = v4897 * v10042;
            let v16333 = v14779 * v4897;
            let v10044 = if v11 != v1 { 1.0 } else { 0.0 };
            let v10932: f64;
            let v12242: f64;
            let v14791: Lanes<1>;
            if v10044 != 0.0 {
                let v10046 = v10045 * v10042;
                let v16334 = v14779 * v10045;
                v10932 = v10046;
                v12242 = v10045;
                v14791 = v16334;
            } else {
                v10932 = v10043;
                v12242 = v4897;
                v14791 = v16333;
            }
            let v13439: f64;
            let v13441: f64;
            let v13444: f64;
            let v13446: f64;
            let v13448: f64;
            let v13453: f64;
            let v13478: f64;
            let v13480: f64;
            let v13483: f64;
            let v13485: f64;
            let v13487: f64;
            let v13492: f64;
            let v13517: f64;
            let v13522: f64;
            let v13535: f64;
            let v13548: f64;
            let v13553: f64;
            let v13566: f64;
            let v13579: f64;
            let v13584: f64;
            let v13597: f64;
            let v13610: f64;
            let v13615: f64;
            let v13628: f64;
            let v13641: f64;
            let v13646: f64;
            let v13659: f64;
            let v13672: f64;
            let v13677: f64;
            let v13690: f64;
            if v97 != 0.0 {
                let v10054 = v211 * v153;
                let v10057 = ((v10047 * v10048) + (v10050 * v10051)) + (v10054 * v10055);
                let v10058 = if v10057 > v0 { 1.0 } else { 0.0 };
                let v13442: f64;
                let v13445: f64;
                let v13447: f64;
                let v13449: f64;
                let v13454: f64;
                if v10058 != 0.0 {
                    let v10059 = v6600 * v9733;
                    let v10061 = -v10060;
                    let v10065 = (rspice_limited_exp((v10061 / v10059))) * v10064;
                    let v10071 = (v1 + (if (v10066 / v10057) >= v10068 { (v10066 / v10057) } else { v10068 })) - v10065;
                    let v10077 = v4897 * (v10071 + (((v10071 * v10071) + (v22 * v10065)).sqrt()));
                    let v10078 = if v10077 > v185 { 1.0 } else { 0.0 };
                    let v10081: f64;
                    if v10078 != 0.0 {
                        let v10079 = v10077.ln();
                        v10081 = v10079;
                    } else {
                        v10081 = v10080;
                    }
                    let v10082 = v10059 * v10081;
                    let v10085 = (v10083 / v10057) - v10068;
                    let v10087 = if v10085 < v10086 { 1.0 } else { 0.0 };
                    let v10096: f64;
                    if v10087 != 0.0 {
                        let v10089 = v10088 / v10085;
                        v10096 = v10089;
                    } else {
                        let v10095 = v4897 * (v10085 + (((v10085 * v10085) + v10091).sqrt()));
                        v10096 = v10095;
                    }
                    let v10099 = ((v10096 + v10068) - v1) / v10064;
                    let v10100 = if v10099 > v185 { 1.0 } else { 0.0 };
                    let v10103: f64;
                    if v10100 != 0.0 {
                        let v10101 = v10099.ln();
                        v10103 = v10101;
                    } else {
                        v10103 = v10102;
                    }
                    let v10105 = v10061 - (v10059 * v10103);
                    let v10110 = v10064 * (rspice_limited_exp(((-(v10060 + v10105)) / v10059)));
                    let v10112 = v10057 * (v1 + v10110);
                    let v10115 = ((-v10057) * v10110) / v10059;
                    v13442 = v10105;
                    v13445 = v10059;
                    v13447 = v10112;
                    v13449 = v10115;
                    v13454 = v10082;
                } else {
                    v13442 = v0;
                    v13445 = v0;
                    v13447 = v0;
                    v13449 = v0;
                    v13454 = v0;
                }
                let v10125 = ((v10116 * v10117) + (v10119 * v10120)) + (v10054 * v10123);
                let v10126 = if v10125 > v0 { 1.0 } else { 0.0 };
                let v13481: f64;
                let v13484: f64;
                let v13486: f64;
                let v13488: f64;
                let v13493: f64;
                if v10126 != 0.0 {
                    let v10127 = v6600 * v9745;
                    let v10129 = -v10128;
                    let v10133 = (rspice_limited_exp((v10129 / v10127))) * v10132;
                    let v10138 = (v1 + (if (v10134 / v10125) >= v10068 { (v10134 / v10125) } else { v10068 })) - v10133;
                    let v10144 = v4897 * (v10138 + (((v10138 * v10138) + (v22 * v10133)).sqrt()));
                    let v10145 = if v10144 > v185 { 1.0 } else { 0.0 };
                    let v10148: f64;
                    if v10145 != 0.0 {
                        let v10146 = v10144.ln();
                        v10148 = v10146;
                    } else {
                        v10148 = v10147;
                    }
                    let v10149 = v10127 * v10148;
                    let v10152 = (v10150 / v10125) - v10068;
                    let v10154 = if v10152 < v10153 { 1.0 } else { 0.0 };
                    let v10163: f64;
                    if v10154 != 0.0 {
                        let v10156 = v10155 / v10152;
                        v10163 = v10156;
                    } else {
                        let v10162 = v4897 * (v10152 + (((v10152 * v10152) + v10158).sqrt()));
                        v10163 = v10162;
                    }
                    let v10166 = ((v10163 + v10068) - v1) / v10132;
                    let v10167 = if v10166 > v185 { 1.0 } else { 0.0 };
                    let v10170: f64;
                    if v10167 != 0.0 {
                        let v10168 = v10166.ln();
                        v10170 = v10168;
                    } else {
                        v10170 = v10169;
                    }
                    let v10172 = v10129 - (v10127 * v10170);
                    let v10177 = v10132 * (rspice_limited_exp(((-(v10128 + v10172)) / v10127)));
                    let v10179 = v10125 * (v1 + v10177);
                    let v10182 = ((-v10125) * v10177) / v10127;
                    v13481 = v10172;
                    v13484 = v10127;
                    v13486 = v10179;
                    v13488 = v10182;
                    v13493 = v10149;
                } else {
                    v13481 = v0;
                    v13484 = v0;
                    v13486 = v0;
                    v13488 = v0;
                    v13493 = v0;
                }
                let v10185 = v10183 * v10047;
                let v10188 = v10186 * v10050;
                let v10192 = (v10189 * v423) * v153;
                let v10195 = v10193 * v10116;
                let v10198 = v10196 * v10119;
                let v10202 = (v10199 * v423) * v153;
                let v10204 = if v10203 > v0 { 1.0 } else { 0.0 };
                let v13523: f64;
                let v13536: f64;
                if v10204 != 0.0 {
                    let v10211 = v10205 * (v1 - ((v1 / v10203).powf((v1 / v10207))));
                    let v10221 = (((v10205 * v10203) * v10213) / v10207) / ((v1 - (v10211 / v10205)).powf((-(v1 + v10207))));
                    v13523 = v10211;
                    v13536 = v10221;
                } else {
                    v13523 = v0;
                    v13536 = v0;
                }
                let v10223 = if v10222 > v0 { 1.0 } else { 0.0 };
                let v13554: f64;
                let v13567: f64;
                if v10223 != 0.0 {
                    let v10230 = v10224 * (v1 - ((v1 / v10222).powf((v1 / v10226))));
                    let v10240 = (((v10224 * v10222) * v10232) / v10226) / ((v1 - (v10230 / v10224)).powf((-(v1 + v10226))));
                    v13554 = v10230;
                    v13567 = v10240;
                } else {
                    v13554 = v0;
                    v13567 = v0;
                }
                let v10242 = if v10241 > v0 { 1.0 } else { 0.0 };
                let v13585: f64;
                let v13598: f64;
                if v10242 != 0.0 {
                    let v10249 = v10243 * (v1 - ((v1 / v10241).powf((v1 / v10245))));
                    let v10259 = (((v10243 * v10241) * v10251) / v10245) / ((v1 - (v10249 / v10243)).powf((-(v1 + v10245))));
                    v13585 = v10249;
                    v13598 = v10259;
                } else {
                    v13585 = v0;
                    v13598 = v0;
                }
                let v10261 = if v10260 > v0 { 1.0 } else { 0.0 };
                let v13616: f64;
                let v13629: f64;
                if v10261 != 0.0 {
                    let v10268 = v10262 * (v1 - ((v1 / v10260).powf((v1 / v10264))));
                    let v10278 = (((v10262 * v10260) * v10270) / v10264) / ((v1 - (v10268 / v10262)).powf((-(v1 + v10264))));
                    v13616 = v10268;
                    v13629 = v10278;
                } else {
                    v13616 = v0;
                    v13629 = v0;
                }
                let v10280 = if v10279 > v0 { 1.0 } else { 0.0 };
                let v13647: f64;
                let v13660: f64;
                if v10280 != 0.0 {
                    let v10287 = v10281 * (v1 - ((v1 / v10279).powf((v1 / v10283))));
                    let v10297 = (((v10281 * v10279) * v10289) / v10283) / ((v1 - (v10287 / v10281)).powf((-(v1 + v10283))));
                    v13647 = v10287;
                    v13660 = v10297;
                } else {
                    v13647 = v0;
                    v13660 = v0;
                }
                let v10299 = if v10298 > v0 { 1.0 } else { 0.0 };
                let v13678: f64;
                let v13691: f64;
                if v10299 != 0.0 {
                    let v10306 = v10300 * (v1 - ((v1 / v10298).powf((v1 / v10302))));
                    let v10316 = (((v10300 * v10298) * v10308) / v10302) / ((v1 - (v10306 / v10300)).powf((-(v1 + v10302))));
                    v13678 = v10306;
                    v13691 = v10316;
                } else {
                    v13678 = v0;
                    v13691 = v0;
                }
                v13439 = v10057;
                v13441 = v13442;
                v13444 = v13445;
                v13446 = v13447;
                v13448 = v13449;
                v13453 = v13454;
                v13478 = v10125;
                v13480 = v13481;
                v13483 = v13484;
                v13485 = v13486;
                v13487 = v13488;
                v13492 = v13493;
                v13517 = v10185;
                v13522 = v13523;
                v13535 = v13536;
                v13548 = v10188;
                v13553 = v13554;
                v13566 = v13567;
                v13579 = v10192;
                v13584 = v13585;
                v13597 = v13598;
                v13610 = v10195;
                v13615 = v13616;
                v13628 = v13629;
                v13641 = v10198;
                v13646 = v13647;
                v13659 = v13660;
                v13672 = v10202;
                v13677 = v13678;
                v13690 = v13691;
            } else {
                v13439 = v0;
                v13441 = v0;
                v13444 = v0;
                v13446 = v0;
                v13448 = v0;
                v13453 = v0;
                v13478 = v0;
                v13480 = v0;
                v13483 = v0;
                v13485 = v0;
                v13487 = v0;
                v13492 = v0;
                v13517 = v0;
                v13522 = v0;
                v13535 = v0;
                v13548 = v0;
                v13553 = v0;
                v13566 = v0;
                v13579 = v0;
                v13584 = v0;
                v13597 = v0;
                v13610 = v0;
                v13615 = v0;
                v13628 = v0;
                v13641 = v0;
                v13646 = v0;
                v13659 = v0;
                v13672 = v0;
                v13677 = v0;
                v13690 = v0;
            }
            let v10320 = v9888 * (v10317 - v10318);
            let v16338 = ((Lanes([0.0, v14602[0]])) - (Lanes([v14603[0], 0.0]))) * v9888;
            let v10323 = v9888 * (v10321 - v10318);
            let v16342 = ((Lanes([v14604[0], 0.0])) - (Lanes([0.0, v14603[0]]))) * v9888;
            let v10325 = v9888 * (v10317 - v10321);
            let v10328 = v9888 * (v10326 - v10318);
            let v16346 = ((Lanes([v14605[0], 0.0])) - (Lanes([0.0, v14603[0]]))) * v9888;
            let v10330 = v9888 * (v10326 - v10321);
            let v16350 = ((Lanes([v14605[0], 0.0])) - (Lanes([0.0, v14604[0]]))) * v9888;
            let v10332 = v9888 * (v10317 - v10326);
            let v10333 = if v4707 != v23 { 1.0 } else { 0.0 };
            let v12662: f64;
            let v12796: f64;
            if v10333 != 0.0 {
                let v10336 = v9888 * (v10334 - v10321);
                let v10338 = v9888 * (v10334 - v10318);
                v12662 = v10336;
                v12796 = v10338;
            } else {
                let v10341 = v9888 * (v10339 - v10321);
                let v10344 = v9888 * (v10342 - v10318);
                v12662 = v10341;
                v12796 = v10344;
            }
            let v10345 = if v10323 < v0 { 1.0 } else { 0.0 };
            let v10350: f64;
            let v10352: f64;
            let v10357: f64;
            let v12645: f64;
            let v14792: Lanes<3>;
            let v14793: Lanes<2>;
            let v14794: Lanes<3>;
            if v10345 != 0.0 {
                let v10347 = v10320 - v10323;
                let v16355 = (Lanes([0.0, v16338[0], v16338[1]])) - (Lanes([v16342[0], v16342[1], 0.0]));
                let v10349 = v10348 * v10323;
                let v16356 = v16342 * v10348;
                let v16357 = Lanes([v16350[0], v16350[1], 0.0]);
                v10350 = v10347;
                v10352 = v10349;
                v10357 = v10330;
                v12645 = v10346;
                v14792 = v16355;
                v14793 = v16356;
                v14794 = v16357;
            } else {
                let v16351 = Lanes([0.0, v16338[0], v16338[1]]);
                let v16352 = Lanes([v16346[0], 0.0, v16346[1]]);
                v10350 = v10320;
                v10352 = v10323;
                v10357 = v10328;
                v12645 = v1;
                v14792 = v16351;
                v14793 = v16342;
                v14794 = v16352;
            }
            let v10351 = v10350 - v10041;
            let v16360 = (Lanes([0.0, v14792[0], v14792[1], v14792[2]])) - (Lanes([v16332[0], 0.0, 0.0, 0.0]));
            let v16361 = v14793 * v10352;
            let v10355 = ((v10352 * v10352) + v4536).sqrt();
            let v16365 = (v16361 + v16361) * (v14600 / (v14869 * v10355));
            let v10356 = v10355 - v6154;
            let v10947: f64;
            let v14795: Lanes<3>;
            if v97 != 0.0 {
                let v16368 = (v14793 - v16365) * v4897;
                let v10363 = v10361 * v10362;
                let v16371 = (v14794 - (Lanes([0.0, v16368[0], v16368[1]]))) * v14888;
                let v10365 = (v10363 - (v10357 - (v4897 * (v10352 - v10356)))) - v4637;
                let v16372 = v16371 * v10365;
                let v10370 = ((v10365 * v10365) + (v10367 * v10363)).sqrt();
                let v10373 = v10363 - (v4897 * (v10365 + v10370));
                let v16379 = ((v16371 + ((v16372 + v16372) * (v14600 / (v14869 * v10370)))) * v4897) * v14888;
                v10947 = v10373;
                v14795 = v16379;
            } else {
                v10947 = v0;
                v14795 = v16366;
            }
            let v16380 = v16342 * v4552;
            let v10375 = (v4552 * v10323) / v6600;
            let v16381 = v14866 * v10375;
            let v10376 = v10375.tanh();
            let v16389 = ((((Lanes([0.0, v16380[0], v16380[1]])) - (Lanes([v16381[0], 0.0, 0.0]))) / v6600) * (v14600 - (v10376 * v10376))) * v4897;
            let v10378 = v4897 + (v4897 * v10376);
            let v10379 = v1 - v10378;
            let v16390 = v16389 * v14888;
            let v10526: f64;
            let v10537: f64;
            let v10544: f64;
            let v10599: f64;
            let v10622: f64;
            let v10625: f64;
            let v10943: f64;
            let v10945: f64;
            let v10946: f64;
            let v10951: f64;
            let v11010: f64;
            let v11015: f64;
            let v12273: f64;
            let v12278: f64;
            let v12307: f64;
            let v12330: f64;
            let v12359: f64;
            let v12517: f64;
            let v12553: f64;
            let v14796: Lanes<3>;
            let v14797: Lanes<3>;
            let v14798: Lanes<3>;
            let v14799: Lanes<3>;
            let v14800: Lanes<3>;
            let v14801: Lanes<3>;
            let v14802: Lanes<3>;
            let v14803: Lanes<3>;
            let v14804: Lanes<3>;
            let v14805: Lanes<3>;
            let v14806: Lanes<3>;
            let v14807: Lanes<3>;
            let v14808: Lanes<3>;
            let v14809: Lanes<3>;
            let v14810: Lanes<3>;
            let v14811: Lanes<3>;
            let v14812: Lanes<3>;
            if v3342 != 0.0 {
                let v10382 = (v4558 * v10379) + (v4556 * v10378);
                let v16406 = (v16390 * v4558) + (v16389 * v4556);
                let v16407 = v14777 * v10379;
                let v16411 = v14775 * v10378;
                let v10387 = (v10383 * v10379) + (v10385 * v10378);
                let v16415 = ((Lanes([v16407[0], 0.0, 0.0])) + (v16390 * v10383)) + ((Lanes([v16411[0], 0.0, 0.0])) + (v16389 * v10385));
                let v10390 = (v4597 * v10379) + (v1474 * v10378);
                let v16418 = (v16390 * v4597) + (v16389 * v1474);
                let v10393 = (v4599 * v10379) + (v1490 * v10378);
                let v16421 = (v16390 * v4599) + (v16389 * v1490);
                let v16422 = v14630 * v10379;
                let v16426 = v14631 * v10378;
                let v10403 = (v10394 * v10379) + (v10400 * v10378);
                let v16430 = ((Lanes([v16422[0], 0.0, 0.0])) + (v16390 * v10394)) + ((Lanes([v16426[0], 0.0, 0.0])) + (v16389 * v10400));
                let v16431 = v14783 * v10379;
                let v16435 = v14781 * v10378;
                let v10409 = (v10404 * v10379) + (v10407 * v10378);
                let v16439 = ((Lanes([v16431[0], 0.0, 0.0])) + (v16390 * v10404)) + ((Lanes([v16435[0], 0.0, 0.0])) + (v16389 * v10407));
                let v16440 = v14632 * v10379;
                let v16444 = v14764 * v10378;
                let v10424 = (v10410 * v10379) + (v10422 * v10378);
                let v16448 = ((Lanes([v16440[0], 0.0, 0.0])) + (v16390 * v10410)) + ((Lanes([v16444[0], 0.0, 0.0])) + (v16389 * v10422));
                let v10430 = (v10425 * v10379) + (v10428 * v10378);
                let v10436 = (v10431 * v10379) + (v10434 * v10378);
                let v16450 = v14633 * v10378;
                let v10444 = (v10437 * v10379) + (v10439 * v10378);
                let v16454 = (v16390 * v10437) + ((Lanes([v16450[0], 0.0, 0.0])) + (v16389 * v10439));
                let v16455 = v14634 * v10379;
                let v16459 = v14763 * v10378;
                let v10459 = (v10445 * v10379) + (v10457 * v10378);
                let v16463 = ((Lanes([v16455[0], 0.0, 0.0])) + (v16390 * v10445)) + ((Lanes([v16459[0], 0.0, 0.0])) + (v16389 * v10457));
                let v16465 = v14635 * v10378;
                let v10467 = (v10460 * v10379) + (v10462 * v10378);
                let v16469 = (v16390 * v10460) + ((Lanes([v16465[0], 0.0, 0.0])) + (v16389 * v10462));
                let v10471 = (v10468 * v10379) + (v674 * v10378);
                let v16472 = (v16390 * v10468) + (v16389 * v674);
                let v10475 = (v10472 * v10379) + (v482 * v10378);
                let v16475 = (v16390 * v10472) + (v16389 * v482);
                let v16476 = v14636 * v10379;
                let v16480 = v14637 * v10378;
                let v10485 = (v10476 * v10379) + (v10482 * v10378);
                let v16484 = ((Lanes([v16476[0], 0.0, 0.0])) + (v16390 * v10476)) + ((Lanes([v16480[0], 0.0, 0.0])) + (v16389 * v10482));
                let v16485 = v14638 * v10379;
                let v16489 = v14639 * v10378;
                let v10495 = (v10486 * v10379) + (v10492 * v10378);
                let v16493 = ((Lanes([v16485[0], 0.0, 0.0])) + (v16390 * v10486)) + ((Lanes([v16489[0], 0.0, 0.0])) + (v16389 * v10492));
                let v16494 = v14640 * v10379;
                let v16498 = v14641 * v10378;
                let v10505 = (v10496 * v10379) + (v10502 * v10378);
                let v16502 = ((Lanes([v16494[0], 0.0, 0.0])) + (v16390 * v10496)) + ((Lanes([v16498[0], 0.0, 0.0])) + (v16389 * v10502));
                let v16503 = v14765 * v10379;
                let v16507 = v14766 * v10378;
                let v10516 = (v10506 * v10379) + (v10512 * v10378);
                let v16511 = ((Lanes([v16503[0], 0.0, 0.0])) + (v16390 * v10506)) + ((Lanes([v16507[0], 0.0, 0.0])) + (v16389 * v10512));
                let v16513 = v14642 * v10378;
                let v10525 = (v10517 * v10379) + (v10520 * v10378);
                let v16517 = (v16390 * v10517) + ((Lanes([v16513[0], 0.0, 0.0])) + (v16389 * v10520));
                v10526 = v10403;
                v10537 = v10382;
                v10544 = v10475;
                v10599 = v10387;
                v10622 = v10471;
                v10625 = v10485;
                v10943 = v10525;
                v10945 = v10495;
                v10946 = v10516;
                v10951 = v10505;
                v11010 = v10459;
                v11015 = v10467;
                v12273 = v10390;
                v12278 = v10393;
                v12307 = v10444;
                v12330 = v10424;
                v12359 = v10409;
                v12517 = v10430;
                v12553 = v10436;
                v14796 = v16430;
                v14797 = v16406;
                v14798 = v16475;
                v14799 = v16415;
                v14800 = v16472;
                v14801 = v16484;
                v14802 = v16517;
                v14803 = v16493;
                v14804 = v16511;
                v14805 = v16502;
                v14806 = v16463;
                v14807 = v16469;
                v14808 = v16418;
                v14809 = v16421;
                v14810 = v16454;
                v14811 = v16448;
                v14812 = v16439;
            } else {
                let v16391 = Lanes([v14631[0], 0.0, 0.0]);
                let v16393 = Lanes([v14775[0], 0.0, 0.0]);
                let v16394 = Lanes([v14637[0], 0.0, 0.0]);
                let v16395 = Lanes([v14642[0], 0.0, 0.0]);
                let v16396 = Lanes([v14639[0], 0.0, 0.0]);
                let v16397 = Lanes([v14766[0], 0.0, 0.0]);
                let v16398 = Lanes([v14641[0], 0.0, 0.0]);
                let v16399 = Lanes([v14763[0], 0.0, 0.0]);
                let v16400 = Lanes([v14635[0], 0.0, 0.0]);
                let v16401 = Lanes([v14633[0], 0.0, 0.0]);
                let v16402 = Lanes([v14764[0], 0.0, 0.0]);
                let v16403 = Lanes([v14781[0], 0.0, 0.0]);
                v10526 = v10400;
                v10537 = v4556;
                v10544 = v482;
                v10599 = v10385;
                v10622 = v674;
                v10625 = v10482;
                v10943 = v10520;
                v10945 = v10492;
                v10946 = v10512;
                v10951 = v10502;
                v11010 = v10457;
                v11015 = v10462;
                v12273 = v1474;
                v12278 = v1490;
                v12307 = v10439;
                v12330 = v10422;
                v12359 = v10407;
                v12517 = v10428;
                v12553 = v10434;
                v14796 = v16391;
                v14797 = v16392;
                v14798 = v16392;
                v14799 = v16393;
                v14800 = v16392;
                v14801 = v16394;
                v14802 = v16395;
                v14803 = v16396;
                v14804 = v16397;
                v14805 = v16398;
                v14806 = v16399;
                v14807 = v16400;
                v14808 = v16392;
                v14809 = v16392;
                v14810 = v16401;
                v14811 = v16402;
                v14812 = v16403;
            }
            let v10527 = v1 / v10526;
            let v16520 = ((v14796 * v10527) * v14888) / v10526;
            let v10531 = (v10528 + v10529) + v578;
            let v10532 = v23 * v421;
            let v10534 = v10532 / (v414 + v23);
            let v16522 = v16365 * v10537;
            let v10540 = v10535 * (v4554 + (v10537 * v10356));
            let v16525 = ((v14797 * v10356) + (Lanes([0.0, v16522[0], v16522[1]]))) * v10535;
            let v10542 = if v10541 == v0 { 1.0 } else { 0.0 };
            let v10555: f64;
            let v14813: Lanes<3>;
            if v10542 != 0.0 {
                let v10556: f64;
                let v14814: Lanes<3>;
                if v6778 != 0.0 {
                    let v10543 = v6600 * v6807;
                    let v10547 = v1 + ((v10544 + v10540) / v10534);
                    let v10548 = v10543 * v10547;
                    let v16540 = ((v14866 * v6807) + (v14626 * v6600)) * v10547;
                    let v16543 = (Lanes([v16540[0], 0.0, 0.0])) + (((v14798 + v16525) / v10534) * v10543);
                    v10556 = v10548;
                    v14814 = v16543;
                } else {
                    let v10550 = v10549 * v6807;
                    let v10553 = v1 + ((v10544 + v10540) / v10534);
                    let v10554 = v10550 * v10553;
                    let v16531 = ((v14613 * v6807) + (v14626 * v10549)) * v10553;
                    let v16534 = (Lanes([v16531[0], 0.0, 0.0])) + (((v14798 + v16525) / v10534) * v10550);
                    v10556 = v10554;
                    v14814 = v16534;
                }
                v10555 = v10556;
                v14813 = v14814;
            } else {
                v10555 = v10541;
                v14813 = v16392;
            }
            let v10557 = v420 / v10555;
            let v16546 = ((v14813 * v10557) * v14888) / v10555;
            let v10561 = ((v415 * v6837) * v23) * v399;
            let v10562 = (v378 * v10555) / v10561;
            let v16551 = (((v14623 * v415) * v23) * v399) * v10562;
            let v16554 = ((v14813 * v378) - (Lanes([v16551[0], 0.0, 0.0]))) / v10561;
            let v10563 = if v10562 > v185 { 1.0 } else { 0.0 };
            let v10566: f64;
            let v14815: Lanes<3>;
            if v10563 != 0.0 {
                let v10564 = v10562.ln();
                let v16556 = v16554 * (v14600 / v10562);
                v10566 = v10564;
                v14815 = v16556;
            } else {
                v10566 = v10565;
                v14815 = v16392;
            }
            let v10567 = v10557 * v414;
            let v16557 = v16546 * v414;
            let v16558 = v16557 * v10567;
            let v10571 = ((rspice_limited_exp(v10567)) - v10567) - v1;
            let v10572 = (v10567 * v10567) / v10571;
            let v16565 = ((v16558 + v16558) - (((v16557 * (rspice_limited_exp_derivative(v10567))) - v16557) * v10572)) / v10571;
            let v10573 = if v10572 > v185 { 1.0 } else { 0.0 };
            let v10576: f64;
            let v14816: Lanes<3>;
            if v10573 != 0.0 {
                let v10574 = v10572.ln();
                let v16567 = v16565 * (v14600 / v10572);
                v10576 = v10574;
                v14816 = v16567;
            } else {
                v10576 = v10575;
                v14816 = v16392;
            }
            let v10577 = v10576 + v10566;
            let v16568 = v14816 + v14815;
            let v16570 = (v14813 * v10068) / v414;
            let v10580 = v23 * v5654;
            let v10581 = ((v10068 * v10555) / v414) + v10580;
            let v10583 = v388 * v16;
            let v10584 = (v6600 * v378) / v10583;
            let v10588 = v10587 * v10586;
            let v10591 = (v10588 * (v10584.powf(v10585))) / v6808;
            let v16580 = (((((v14866 * v378) / v10583) * (v10585 * (v10584.powf(v16573)))) * v10588) - (v14987 * v10591)) / v6808;
            let v10595 = (-v530) * v10593;
            let v10598 = v10595 * (v10596 - v10531);
            let v16582 = (v14787 - v14786) * v10595;
            let v10603 = (-v10599) * v10601;
            let v10604 = v10356 + v4536;
            let v10605 = v10604.sqrt();
            let v10607 = v10356 + (v610 * v10605);
            let v16591 = (v16365 + ((v16365 * (v14600 / (v14869 * v10605))) * v610)) * v10603;
            let v10610 = v4499 * v10609;
            let v10612 = v10610 * (v10604.powf(v4505));
            let v16598 = (v16365 * (v4505 * (v10604.powf((v4505 - v14600))))) * v10610;
            let v10614 = v642 * v5635;
            let v10615 = v10531.sqrt();
            let v10616 = v10614 * v10615;
            let v16604 = (v14786 * (v14600 / (v14869 * v10615))) * v10614;
            let v16611 = ((((Lanes([v16582[0], 0.0, 0.0])) + (((((v14799 * v14888) * v10601) * v10607) + (Lanes([0.0, v16591[0], v16591[1]]))) + (Lanes([0.0, v16598[0], v16598[1]])))) + (Lanes([v16604[0], 0.0, 0.0]))) + (Lanes([v14643[0], 0.0, 0.0]))) + v14800;
            let v10624 = v10351 - ((((v10598 + ((v10603 * v10607) + v10612)) + v10616) + v10619) + v10622);
            let v16613 = v16360 - (Lanes([v16611[0], v16611[1], v16611[2], 0.0]));
            let v10628 = ((v10625 * v421) * v423) / v3969;
            let v16616 = ((v14801 * v421) * v423) / v3969;
            let v10726: f64;
            let v14817: Lanes<4>;
            if v6778 != 0.0 {
                let v10632 = (v10628 * v10555) * v415;
                let v16663 = v14623 * v10632;
                let v10634 = (v10632 * v6837) * v211;
                let v10635 = (v10532 * v10629) / v10634;
                let v10636 = v10635.powf(v10555);
                let v16677 = ((((((((((v16616 * v10555) + (v14813 * v10628)) * v415) * v6837) + (Lanes([v16663[0], 0.0, 0.0]))) * v211) * v10635) * v14888) / v10634) * (v10555 * (v10635.powf((v10555 - v14600))))) + (v14813 * (v10636 * (v10635.ln())));
                let v10637 = if v10636 > v185 { 1.0 } else { 0.0 };
                let v10640: f64;
                let v14818: Lanes<3>;
                if v10637 != 0.0 {
                    let v10638 = v10636.ln();
                    let v16679 = v16677 * (v14600 / v10636);
                    v10640 = v10638;
                    v14818 = v16679;
                } else {
                    v10640 = v10639;
                    v14818 = v16392;
                }
                let v10642 = -(v6851 + v10640);
                let v16682 = ((Lanes([v15024[0], 0.0, 0.0])) + v14818) * v14888;
                let v16683 = Lanes([v16682[0], v16682[1], v16682[2], 0.0]);
                let v16684 = v16613 + v16683;
                let v10645 = (v10624 + v10642) + v10644;
                let v10647 = if v10645 < v10646 { 1.0 } else { 0.0 };
                let v10656: f64;
                let v14819: Lanes<4>;
                if v10647 != 0.0 {
                    let v10649 = v10648 / v10645;
                    let v16694 = ((v16684 * v10649) * v14888) / v10645;
                    v10656 = v10649;
                    v14819 = v16694;
                } else {
                    let v16685 = v16684 * v10645;
                    let v10653 = ((v10645 * v10645) + v10651).sqrt();
                    let v10655 = v4897 * (v10645 + v10653);
                    let v16691 = (v16684 + ((v16685 + v16685) * (v14600 / (v14869 * v10653)))) * v4897;
                    v10656 = v10655;
                    v14819 = v16691;
                }
                let v10657 = v10656 - v10642;
                let v16695 = v14819 - v16683;
                v10726 = v10657;
                v14817 = v16695;
            } else {
                let v10660 = (v10628 * v10555) * v415;
                let v16622 = v14623 * v10660;
                let v10662 = (v10660 * v6837) * v211;
                let v10663 = (v10532 * v10629) / v10662;
                let v16628 = ((((((((v16616 * v10555) + (v14813 * v10628)) * v415) * v6837) + (Lanes([v16622[0], 0.0, 0.0]))) * v211) * v10663) * v14888) / v10662;
                let v10664 = if v10663 > v185 { 1.0 } else { 0.0 };
                let v10668: f64;
                let v14820: Lanes<3>;
                if v10664 != 0.0 {
                    let v10665 = v10663.ln();
                    let v16630 = v16628 * (v14600 / v10663);
                    v10668 = v10665;
                    v14820 = v16630;
                } else {
                    v10668 = v10666;
                    v14820 = v16392;
                }
                let v10667 = -v10555;
                let v10669 = v10667 * v10668;
                let v16634 = ((v14813 * v14888) * v10668) + (v14820 * v10667);
                let v16635 = v15024 * v14888;
                let v10672 = v10669 - v4536;
                let v16636 = v16634 * v10672;
                let v10676 = ((v10672 * v10672) + v10674).sqrt();
                let v10679 = (-v6851) + (v4897 * ((v10669 + v4536) + v10676));
                let v16644 = (Lanes([v16635[0], 0.0, 0.0])) + ((v16634 + ((v16636 + v16636) * (v14600 / (v14869 * v10676)))) * v4897);
                let v16645 = Lanes([v16644[0], v16644[1], v16644[2], 0.0]);
                let v16646 = v16613 + v16645;
                let v10681 = (v10624 + v10679) + v10644;
                let v10683 = if v10681 < v10682 { 1.0 } else { 0.0 };
                let v10692: f64;
                let v14821: Lanes<4>;
                if v10683 != 0.0 {
                    let v10685 = v10684 / v10681;
                    let v16656 = ((v16646 * v10685) * v14888) / v10681;
                    v10692 = v10685;
                    v14821 = v16656;
                } else {
                    let v16647 = v16646 * v10681;
                    let v10689 = ((v10681 * v10681) + v10687).sqrt();
                    let v10691 = v4897 * (v10681 + v10689);
                    let v16653 = (v16646 + ((v16647 + v16647) * (v14600 / (v14869 * v10689)))) * v4897;
                    v10692 = v10691;
                    v14821 = v16653;
                }
                let v10693 = v10692 - v10679;
                let v16657 = v14821 - v16645;
                v10726 = v10693;
                v14817 = v16657;
            }
            let v10694 = -v10557;
            let v16696 = v16546 * v14888;
            let v10695 = v10694.powf(v10585);
            let v16700 = v16696 * (v10585 * (v10694.powf(v16697)));
            let v10730: f64;
            let v10732: f64;
            let v14822: Lanes<4>;
            let v14823: Lanes<4>;
            if v97 != 0.0 {
                let v10696 = v23 * v10529;
                let v16710 = v14786 * v23;
                let v16711 = v16710 + v15024;
                let v10698 = (v10696 + v6851) - v10357;
                let v16714 = (Lanes([0.0, v16711[0], 0.0, 0.0])) - (Lanes([v14794[0], 0.0, v14794[1], v14794[2]]));
                let v10700 = if v10698 < v10699 { 1.0 } else { 0.0 };
                let v10709: f64;
                let v14824: Lanes<4>;
                if v10700 != 0.0 {
                    let v10702 = v10701 / v10698;
                    let v16724 = ((v16714 * v10702) * v14888) / v10698;
                    v10709 = v10702;
                    v14824 = v16724;
                } else {
                    let v16715 = v16714 * v10698;
                    let v10706 = ((v10698 * v10698) + v10704).sqrt();
                    let v10708 = v4897 * (v10698 + v10706);
                    let v16721 = (v16714 + ((v16715 + v16715) * (v14600 / (v14869 * v10706)))) * v4897;
                    v10709 = v10708;
                    v14824 = v16721;
                }
                let v16725 = v16276 * v14888;
                let v10711 = v23 * v10555;
                let v10712 = (-v9269) / v10711;
                let v10713 = v10709.sqrt();
                let v10714 = v10696.sqrt();
                let v16736 = v16710 * (v14600 / (v14869 * v10714));
                let v10715 = v10713 - v10714;
                let v16739 = (((Lanes([v16725[0], 0.0, 0.0])) - ((v14813 * v23) * v10712)) / v10711) * v10715;
                let v10717 = v10694 - (v10712 * v10715);
                let v16744 = (Lanes([0.0, v16696[0], v16696[1], v16696[2]])) - ((Lanes([0.0, v16739[0], v16739[1], v16739[2]])) + (((v14824 * (v14600 / (v14869 * v10713))) - (Lanes([0.0, v16736[0], 0.0, 0.0]))) * v10712));
                let v16747 = v16580 * v10695;
                let v16750 = (Lanes([v16747[0], 0.0, 0.0])) + (v16700 * v10591);
                let v10720 = (v10717 + v10577) + (v10591 * v10695);
                let v16752 = (v16744 + (Lanes([0.0, v16568[0], v16568[1], v16568[2]]))) + (Lanes([0.0, v16750[0], v16750[1], v16750[2]]));
                let v10721 = v10717 + v10566;
                let v16754 = v16744 + (Lanes([0.0, v14815[0], v14815[1], v14815[2]]));
                v10730 = v10721;
                v10732 = v10720;
                v14822 = v16754;
                v14823 = v16752;
            } else {
                let v16702 = v16580 * v10695;
                let v10724 = (v10694 + v10577) + (v10591 * v10695);
                let v16706 = (v16696 + v16568) + ((Lanes([v16702[0], 0.0, 0.0])) + (v16700 * v10591));
                let v10725 = v10694 + v10566;
                let v16707 = v16696 + v14815;
                let v16708 = Lanes([0.0, v16707[0], v16707[1], v16707[2]]);
                let v16709 = Lanes([0.0, v16706[0], v16706[1], v16706[2]]);
                v10730 = v10725;
                v10732 = v10724;
                v14822 = v16708;
                v14823 = v16709;
            }
            let v16755 = Lanes([v15024[0], 0.0, 0.0, 0.0]);
            let v10728 = (v10726 - v6851) / v10555;
            let v16757 = v14813 * v10728;
            let v16760 = ((v14817 - v16755) - (Lanes([v16757[0], v16757[1], v16757[2], 0.0]))) / v10555;
            let v16761 = v16760 * v14888;
            let v10731 = (-v10728) + v10730;
            let v16764 = (Lanes([0.0, v16761[0], v16761[1], v16761[2], v16761[3]])) + (Lanes([v14822[0], v14822[1], v14822[2], v14822[3], 0.0]));
            let v10734 = v4897 * (v10728 - v10732);
            let v10735 = rspice_limited_exp(v10734);
            let v16770 = (((Lanes([0.0, v16760[0], v16760[1], v16760[2], v16760[3]])) - (Lanes([v14823[0], v14823[1], v14823[2], v14823[3], 0.0]))) * v4897) * (rspice_limited_exp_derivative(v10734));
            let v10736 = if v10735 > v4907 { 1.0 } else { 0.0 };
            let v10865: f64;
            let v14825: Lanes<5>;
            if v10736 != 0.0 {
                let v10737 = v1 + v10735;
                let v10738 = v10737.ln();
                let v16777 = (v16770 * (v14600 / v10737)) * v10738;
                let v10741 = (v1 + (v10738 * v10738)).sqrt();
                let v10743 = v23 * (v1 - v10741);
                let v16783 = (((v16777 + v16777) * (v14600 / (v14869 * v10741))) * v14888) * v23;
                let v16785 = Lanes([0.0, v16546[0], v16546[1], v16546[2], 0.0]);
                let v10747 = ((v10743 * v10744) + v10557) * v414;
                let v16787 = ((v16783 * v10744) + v16785) * v414;
                let v10750 = ((rspice_limited_exp(v10747)) - v10747) - v1;
                let v10751 = v10747 / v10750;
                let v16793 = (v16787 - (((v16787 * (rspice_limited_exp_derivative(v10747))) - v16787) * v10751)) / v10750;
                let v10752 = v10747 * v10751;
                let v16796 = (v16787 * v10751) + (v16793 * v10747);
                let v10754 = -(v10743 + v10557);
                let v10755 = v10754.ln();
                let v16800 = ((v16783 + v16785) * v14888) * (v14600 / v10754);
                let v10756 = -v10743;
                let v16801 = v16783 * v14888;
                let v10757 = if v10756 > v185 { 1.0 } else { 0.0 };
                let v10764: f64;
                let v14826: Lanes<5>;
                if v10757 != 0.0 {
                    let v10758 = v10756.ln();
                    let v16804 = v16801 * (v14600 / v10756);
                    v10764 = v10758;
                    v14826 = v16804;
                } else {
                    v10764 = v10759;
                    v14826 = v16802;
                }
                let v10760 = if v10752 > v185 { 1.0 } else { 0.0 };
                let v10766: f64;
                let v14827: Lanes<5>;
                if v10760 != 0.0 {
                    let v10761 = v10752.ln();
                    let v16806 = v16796 * (v14600 / v10752);
                    v10766 = v10761;
                    v14827 = v16806;
                } else {
                    v10766 = v10762;
                    v14827 = v16802;
                }
                let v10769 = (v10585 * v10755).exp();
                let v16812 = v16580 * v10769;
                let v10771 = (((v10731 - v10743) + v10764) + v10766) + (v10591 * v10769);
                let v16816 = (((v16764 - v16783) + v14826) + v14827) + ((Lanes([0.0, v16812[0], 0.0, 0.0, 0.0])) + (((v16800 * v10585) * v10769) * v10591));
                let v10773 = v1 / v10743;
                let v10775 = v23 / v10747;
                let v10780 = v10585 * v10591;
                let v16826 = v16580 * v10585;
                let v10783 = (v10781 * v10755).exp();
                let v16829 = v16826 * v10783;
                let v10785 = ((v10772 + v10773) + (((v10775 - v10751) - v1) * v414)) - (v10780 * v10783);
                let v16833 = ((((v16783 * v10773) * v14888) / v10743) + (((((v16787 * v10775) * v14888) / v10747) - v16793) * v414)) - ((Lanes([0.0, v16829[0], 0.0, 0.0, 0.0])) + (((v16800 * v10781) * v10783) * v10780));
                let v10787 = v10743 * v10743;
                let v16834 = v16783 * v10743;
                let v10788 = v10786 / v10787;
                let v10791 = v10790 * v10591;
                let v10794 = (v10792 * v10755).exp();
                let v16842 = (v16580 * v10790) * v10794;
                let v10796 = v10788 - (v10791 * v10794);
                let v10797 = v10771 / v10785;
                let v10799 = v23 * v10785;
                let v10800 = v10799 * v10785;
                let v10801 = (v10771 * v10796) / v10800;
                let v10802 = v1 + v10801;
                let v10804 = v10743 - (v10797 * v10802);
                let v16863 = v16783 - ((((v16816 - (v16833 * v10797)) / v10785) * v10802) + (((((v16816 * v10796) + ((((((v16834 + v16834) * v10788) * v14888) / v10787) - ((Lanes([0.0, v16842[0], 0.0, 0.0, 0.0])) + (((v16800 * v10792) * v10794) * v10791))) * v10771)) - ((((v16833 * v23) * v10785) + (v16833 * v10799)) * v10801)) / v10800) * v10797));
                let v10807 = ((v10804 * v10744) + v10557) * v414;
                let v16866 = ((v16863 * v10744) + v16785) * v414;
                let v10810 = ((rspice_limited_exp(v10807)) - v10807) - v1;
                let v10811 = v10807 / v10810;
                let v16872 = (v16866 - (((v16866 * (rspice_limited_exp_derivative(v10807))) - v16866) * v10811)) / v10810;
                let v10812 = v10807 * v10811;
                let v16875 = (v16866 * v10811) + (v16872 * v10807);
                let v10814 = -(v10804 + v10557);
                let v10815 = v10814.ln();
                let v16879 = ((v16863 + v16785) * v14888) * (v14600 / v10814);
                let v10816 = -v10804;
                let v16880 = v16863 * v14888;
                let v10817 = if v10816 > v185 { 1.0 } else { 0.0 };
                let v10824: f64;
                let v14828: Lanes<5>;
                if v10817 != 0.0 {
                    let v10818 = v10816.ln();
                    let v16882 = v16880 * (v14600 / v10816);
                    v10824 = v10818;
                    v14828 = v16882;
                } else {
                    v10824 = v10819;
                    v14828 = v16802;
                }
                let v10820 = if v10812 > v185 { 1.0 } else { 0.0 };
                let v10826: f64;
                let v14829: Lanes<5>;
                if v10820 != 0.0 {
                    let v10821 = v10812.ln();
                    let v16884 = v16875 * (v14600 / v10812);
                    v10826 = v10821;
                    v14829 = v16884;
                } else {
                    v10826 = v10822;
                    v14829 = v16802;
                }
                let v10829 = (v10585 * v10815).exp();
                let v16890 = v16580 * v10829;
                let v10831 = (((v10731 - v10804) + v10824) + v10826) + (v10591 * v10829);
                let v16894 = (((v16764 - v16863) + v14828) + v14829) + ((Lanes([0.0, v16890[0], 0.0, 0.0, 0.0])) + (((v16879 * v10585) * v10829) * v10591));
                let v10833 = v1 / v10804;
                let v10835 = v23 / v10807;
                let v10842 = (v10840 * v10815).exp();
                let v16906 = v16826 * v10842;
                let v10844 = ((v10832 + v10833) + (((v10835 - v10811) - v1) * v414)) - (v10780 * v10842);
                let v16910 = ((((v16863 * v10833) * v14888) / v10804) + (((((v16866 * v10835) * v14888) / v10807) - v16872) * v414)) - ((Lanes([0.0, v16906[0], 0.0, 0.0, 0.0])) + (((v16879 * v10840) * v10842) * v10780));
                let v10846 = v10804 * v10804;
                let v16911 = v16863 * v10804;
                let v10847 = v10845 / v10846;
                let v10849 = v10848 * v10591;
                let v10852 = (v10850 * v10815).exp();
                let v16919 = (v16580 * v10848) * v10852;
                let v10854 = v10847 - (v10849 * v10852);
                let v10855 = v10831 / v10844;
                let v10857 = v23 * v10844;
                let v10858 = v10857 * v10844;
                let v10859 = (v10831 * v10854) / v10858;
                let v10860 = v1 + v10859;
                let v10862 = v10804 - (v10855 * v10860);
                let v16940 = v16863 - ((((v16894 - (v16910 * v10855)) / v10844) * v10860) + (((((v16894 * v10854) + ((((((v16911 + v16911) * v10847) * v14888) / v10846) - ((Lanes([0.0, v16919[0], 0.0, 0.0, 0.0])) + (((v16879 * v10850) * v10852) * v10849))) * v10831)) - ((((v16910 * v23) * v10844) + (v16910 * v10857)) * v10859)) / v10858) * v10855));
                v10865 = v10862;
                v14825 = v16940;
            } else {
                let v10863 = -v10735;
                let v10864 = v10863 * v10735;
                let v16774 = ((v16770 * v14888) * v10735) + (v16770 * v10863);
                v10865 = v10864;
                v14825 = v16774;
            }
            let v10866 = -v10865;
            let v10867 = v10866 * v10555;
            let v16943 = v14813 * v10866;
            let v16945 = ((v14825 * v14888) * v10555) + (Lanes([0.0, v16943[0], v16943[1], v16943[2], 0.0]));
            let v10933: f64;
            let v14830: Lanes<5>;
            if v4000 != 0.0 {
                let v10868 = v10624 - v6851;
                let v16946 = v16613 - v16755;
                let v10869 = v10868 / v10555;
                let v16947 = v14813 * v10869;
                let v16950 = (v16946 - (Lanes([v16947[0], v16947[1], v16947[2], 0.0]))) / v10555;
                let v16951 = v16950 * v10869;
                let v10875 = ((v10869 * v10869) + ((v5721 * v10871) * v10871)).sqrt();
                let v10877 = v4897 * (v10869 + v10875);
                let v16957 = (v16950 + ((v16951 + v16951) * (v14600 / (v14869 * v10875)))) * v4897;
                let v10879 = v10878 / v23;
                let v10882 = v10881 * (v10877.powf(v10879));
                let v10883 = v10869 - v10877;
                let v10884 = rspice_limited_exp(v10883);
                let v10888 = (v10868 - v10886) / v10555;
                let v16969 = v14813 * v10888;
                let v16972 = (v16946 - (Lanes([v16969[0], v16969[1], v16969[2], 0.0]))) / v10555;
                let v16973 = v16972 * v10888;
                let v10894 = ((v10888 * v10888) + ((v5721 * v10890) * v10890)).sqrt();
                let v10896 = v4897 * (v10888 + v10894);
                let v16979 = (v16972 + ((v16973 + v16973) * (v14600 / (v14869 * v10894)))) * v4897;
                let v10898 = v10897 / v23;
                let v10901 = v10900 * (v10896.powf(v10898));
                let v10902 = v10888 - v10896;
                let v10903 = rspice_limited_exp(v10902);
                let v10907 = (v10868 - v10905) / v10555;
                let v16991 = v14813 * v10907;
                let v16994 = (v16946 - (Lanes([v16991[0], v16991[1], v16991[2], 0.0]))) / v10555;
                let v16995 = v16994 * v10907;
                let v10913 = ((v10907 * v10907) + ((v5721 * v10909) * v10909)).sqrt();
                let v10915 = v4897 * (v10907 + v10913);
                let v17001 = (v16994 + ((v16995 + v16995) * (v14600 / (v14869 * v10913)))) * v4897;
                let v10917 = v10916 / v23;
                let v10920 = v10919 * (v10915.powf(v10917));
                let v10921 = v10907 - v10915;
                let v10922 = rspice_limited_exp(v10921);
                let v17016 = ((((((v16957 * (v10879 * (v10877.powf((v10879 - v14600))))) * v10881) * v10884) + (((v16950 - v16957) * (rspice_limited_exp_derivative(v10883))) * v10882)) + ((((v16979 * (v10898 * (v10896.powf((v10898 - v14600))))) * v10900) * v10903) + (((v16972 - v16979) * (rspice_limited_exp_derivative(v10902))) * v10901))) + ((((v17001 * (v10917 * (v10915.powf((v10917 - v14600))))) * v10919) * v10922) + (((v16994 - v17001) * (rspice_limited_exp_derivative(v10921))) * v10920))) * v10926;
                let v10930 = (v10924 * v10867) + (v10926 * (((v10882 * v10884) + (v10901 * v10903)) + (v10920 * v10922)));
                let v17018 = (v16945 * v10924) + (Lanes([0.0, v17016[0], v17016[1], v17016[2], v17016[3]]));
                v10933 = v10930;
                v14830 = v17018;
            } else {
                v10933 = v10867;
                v14830 = v16945;
            }
            let v10931 = v4536 / v421;
            let v10934 = v10932 * v10933;
            let v17019 = v14791 * v10933;
            let v10936 = v5572 * (v5654 + v10934);
            let v10937 = v10933 / v10931;
            let v10939 = v4897 * (v1 + v10937);
            let v10942 = v10939.powf(v10940);
            let v17026 = v10940 - v14600;
            let v17032 = v14644 * (v10942 * (v10939.ln()));
            let v17034 = (((v14830 / v10931) * v4897) * (v10940 * (v10939.powf(v17026)))) + (Lanes([0.0, v17032[0], 0.0, 0.0, 0.0]));
            let v10944 = v10936.powf(v10943);
            let v17035 = v10943 - v14600;
            let v17041 = v14802 * (v10944 * (v10936.ln()));
            let v17043 = ((((Lanes([0.0, v17019[0], 0.0, 0.0, 0.0])) + (v14830 * v10932)) * v5572) * (v10943 * (v10936.powf(v17035)))) + (Lanes([0.0, v17041[0], v17041[1], v17041[2], 0.0]));
            let v10957: f64;
            let v14831: Lanes<5>;
            if v97 != 0.0 {
                let v17053 = v14804 * v10947;
                let v17054 = v14795 * v10946;
                let v10949 = v10945 + (v10946 * v10947);
                let v17060 = ((Lanes([0.0, v14803[0], v14803[1], v14803[2]])) + ((Lanes([0.0, v17053[0], v17053[1], v17053[2]])) + (Lanes([v17054[0], 0.0, v17054[1], v17054[2]])))) * v10944;
                let v10952 = v10951 / v10942;
                let v10953 = (v10949 * v10944) + v10952;
                let v17068 = ((Lanes([v17060[0], v17060[1], v17060[2], v17060[3], 0.0])) + (v17043 * v10949)) + (((Lanes([0.0, v14805[0], v14805[1], v14805[2], 0.0])) - (v17034 * v10952)) / v10942);
                v10957 = v10953;
                v14831 = v17068;
            } else {
                let v17044 = v14803 * v10944;
                let v10955 = v10951 / v10942;
                let v10956 = (v10945 * v10944) + v10955;
                let v17052 = ((Lanes([0.0, v17044[0], v17044[1], v17044[2], 0.0])) + (v17043 * v10945)) + (((Lanes([0.0, v14805[0], v14805[1], v14805[2], 0.0])) - (v17034 * v10955)) / v10942);
                v10957 = v10956;
                v14831 = v17052;
            }
            let v10958 = v1 + v10957;
            let v10960 = v10958 - v1;
            let v17069 = v14831 * v10960;
            let v10964 = (v5721 * v10962) * v10962;
            let v10966 = ((v10960 * v10960) + v10964).sqrt();
            let v10970 = (v4897 * ((v10958 + v1) + v10966)) / v10969;
            let v17076 = ((v14831 + ((v17069 + v17069) * (v14600 / (v14869 * v10966)))) * v4897) / v10969;
            let v11021: f64;
            let v14832: Lanes<5>;
            if v4402 != 0.0 {
                v11021 = v0;
                v14832 = v16802;
            } else {
                let v11022: f64;
                let v14833: Lanes<5>;
                if v4874 != 0.0 {
                    let v10973 = v1 + (v10971 * v10933);
                    let v10974 = v1 / v10973;
                    let v17098 = (((v14830 * v10971) * v10974) * v14888) / v10973;
                    let v17099 = v17098 * v10974;
                    let v10977 = ((v10974 * v10974) + v4536).sqrt();
                    let v10985 = ((v10980 + (v10981 * (v4897 * (v10974 + v10977)))) * v5577) * v153;
                    let v10988 = v10985 * v10986;
                    let v17110 = v14645 * v10985;
                    let v17112 = ((((((v17098 + ((v17099 + v17099) * (v14600 / (v14869 * v10977)))) * v4897) * v10981) * v5577) * v153) * v10986) + (Lanes([0.0, v17110[0], 0.0, 0.0, 0.0]));
                    v11022 = v10988;
                    v14833 = v17112;
                } else {
                    let v10990 = v1 + (v10971 * v10933);
                    let v10991 = v1 / v10990;
                    let v17080 = (((v14830 * v10971) * v10991) * v14888) / v10990;
                    let v17081 = v17080 * v10991;
                    let v10994 = ((v10991 * v10991) + v4536).sqrt();
                    let v11008 = (v10997 + v11000) + (((v10980 + (v10981 * (v4897 * (v10991 + v10994)))) * v5577) * v153);
                    let v11009 = v11008 * v10986;
                    let v17092 = v14645 * v11008;
                    let v17094 = ((((((v17080 + ((v17081 + v17081) * (v14600 / (v14869 * v10994)))) * v4897) * v10981) * v5577) * v153) * v10986) + (Lanes([0.0, v17092[0], 0.0, 0.0, 0.0]));
                    v11022 = v11009;
                    v14833 = v17094;
                }
                v11021 = v11022;
                v14832 = v14833;
            }
            let v11011 = v23 * v11010;
            let v11012 = v11011 / v10625;
            let v17117 = (((v14806 * v23) - (v14801 * v11012)) / v10625) * v10970;
            let v11014 = (v11012 * v10970) * v3969;
            let v17121 = ((Lanes([0.0, v17117[0], v17117[1], v17117[2], 0.0])) + (v17076 * v11012)) * v3969;
            let v11028: f64;
            let v14834: Lanes<5>;
            if v6778 != 0.0 {
                let v11016 = v10933 + v6773;
                let v11017 = v11015 * v11016;
                let v17131 = v14807 * v11016;
                let v17134 = (Lanes([0.0, v17131[0], v17131[1], v17131[2], 0.0])) + ((v14830 + (Lanes([0.0, v14954[0], 0.0, 0.0, 0.0]))) * v11015);
                v11028 = v11017;
                v14834 = v17134;
            } else {
                let v17122 = v14613 * v23;
                let v11019 = v10933 + (v23 * v10549);
                let v11020 = v11015 * v11019;
                let v17125 = v14807 * v11019;
                let v17128 = (Lanes([0.0, v17125[0], v17125[1], v17125[2], 0.0])) + ((v14830 + (Lanes([0.0, v17122[0], 0.0, 0.0, 0.0]))) * v11015);
                v11028 = v11020;
                v14834 = v17128;
            }
            let v11023 = if v11021 > v0 { 1.0 } else { 0.0 };
            let v11049: f64;
            let v14835: Lanes<5>;
            if v11023 != 0.0 {
                let v11025 = (v423 * v11010) * v421;
                let v11026 = v11025 * v11021;
                let v17144 = ((v14806 * v423) * v421) * v11021;
                let v17147 = (Lanes([0.0, v17144[0], v17144[1], v17144[2], 0.0])) + (v14832 * v11025);
                let v11027 = v23 * v11026;
                let v17148 = v17147 * v23;
                let v11030 = v276 * v11028;
                let v11032 = (v11028 + v11014) + (v11030 * v11026);
                let v17154 = (v14834 + v17121) + (((v14834 * v276) * v11026) + (v17147 * v11030));
                let v11033 = v23 * v11028;
                let v11035 = v11014 + (v11033 * v11026);
                let v11036 = v11028 * v11035;
                let v11037 = v11032 * v11032;
                let v17163 = v17154 * v11032;
                let v17164 = v17163 + v17163;
                let v11038 = v23 * v11027;
                let v11040 = v11037 - (v11038 * v11036);
                let v17169 = v17164 - (((v17148 * v23) * v11036) + (((v14834 * v11035) + ((v17121 + (((v14834 * v23) * v11026) + (v17147 * v11033))) * v11028)) * v11038));
                let v11042 = v11040.sqrt();
                let v11043 = v11032 + v11042;
                let v11044 = v11043 * v11027;
                let v11045 = (v11037 - v11040) / v11044;
                let v17180 = ((v17164 - v17169) - ((((v17154 + (v17169 * (v14600 / (v14869 * v11042)))) * v11027) + (v17148 * v11043)) * v11045)) / v11044;
                v11049 = v11045;
                v14835 = v17180;
            } else {
                let v11047 = v11014 + v11028;
                let v11048 = (v11014 * v11028) / v11047;
                let v17141 = (((v17121 * v11028) + (v14834 * v11014)) - ((v17121 + v14834) * v11048)) / v11047;
                v11049 = v11048;
                v14835 = v17141;
            }
            let v11050 = v11049 - v4637;
            let v11052 = if v11050 < v11051 { 1.0 } else { 0.0 };
            let v11061: f64;
            let v14836: Lanes<5>;
            if v11052 != 0.0 {
                let v11054 = v11053 / v11050;
                let v17190 = ((v14835 * v11054) * v14888) / v11050;
                v11061 = v11054;
                v14836 = v17190;
            } else {
                let v17181 = v14835 * v11050;
                let v11058 = ((v11050 * v11050) + v11056).sqrt();
                let v11060 = v4897 * (v11050 + v11058);
                let v17187 = (v14835 + ((v17181 + v17181) * (v14600 / (v14869 * v11058)))) * v4897;
                v11061 = v11060;
                v14836 = v17187;
            }
            let v11062 = v11061 + v4637;
            let v11063 = v10352 / v11062;
            let v17192 = Lanes([0.0, 0.0, v14793[0], v14793[1], 0.0]);
            let v11064 = v11063 + v25;
            let v11065 = v11064.powf(v10526);
            let v17201 = v14796 * (v11065 * (v11064.ln()));
            let v11066 = v1 + v11065;
            let v11067 = v11066.powf(v10527);
            let v17210 = v16520 * (v11067 * (v11066.ln()));
            let v11068 = v10352 / v11067;
            let v11069 = if v11068 <= v10352 { v11068 } else { v10352 };
            let v17219 = v17192 + ((((v17192 - (((((((v17192 - (v14836 * v11063)) / v11062) * (v10526 * (v11064.powf((v10526 - v14600))))) + (Lanes([0.0, v17201[0], v17201[1], v17201[2], 0.0]))) * (v10527 * (v11066.powf((v10527 - v14600))))) + (Lanes([0.0, v17210[0], v17210[1], v17210[2], 0.0]))) * v11068)) / v11067) - v17192) * (if v11068 <= v10352 { 1.0 } else { 0.0 }));
            let v11070 = v11069 + v6851;
            let v17221 = v17219 + (Lanes([0.0, v15024[0], 0.0, 0.0, 0.0]));
            let v17225 = v16696 * (v10585 * (v10694.powf(v17222)));
            let v11104: f64;
            let v11106: f64;
            let v14837: Lanes<5>;
            let v14838: Lanes<5>;
            if v97 != 0.0 {
                let v11071 = v23 * v10529;
                let v17235 = v14786 * v23;
                let v11073 = (v11071 + v11070) - v10357;
                let v17239 = ((Lanes([0.0, v17235[0], 0.0, 0.0, 0.0])) + v17221) - (Lanes([v14794[0], 0.0, v14794[1], v14794[2], 0.0]));
                let v11075 = if v11073 < v11074 { 1.0 } else { 0.0 };
                let v11084: f64;
                let v14839: Lanes<5>;
                if v11075 != 0.0 {
                    let v11077 = v11076 / v11073;
                    let v17249 = ((v17239 * v11077) * v14888) / v11073;
                    v11084 = v11077;
                    v14839 = v17249;
                } else {
                    let v17240 = v17239 * v11073;
                    let v11081 = ((v11073 * v11073) + v11079).sqrt();
                    let v11083 = v4897 * (v11073 + v11081);
                    let v17246 = (v17239 + ((v17240 + v17240) * (v14600 / (v14869 * v11081)))) * v4897;
                    v11084 = v11083;
                    v14839 = v17246;
                }
                let v17250 = v16276 * v14888;
                let v11086 = v23 * v10555;
                let v11087 = (-v9269) / v11086;
                let v11088 = v11084.sqrt();
                let v11089 = v11071.sqrt();
                let v17261 = v17235 * (v14600 / (v14869 * v11089));
                let v11090 = v11088 - v11089;
                let v17264 = (((Lanes([v17250[0], 0.0, 0.0])) - ((v14813 * v23) * v11087)) / v11086) * v11090;
                let v11092 = v10694 - (v11087 * v11090);
                let v17269 = (Lanes([0.0, v16696[0], v16696[1], v16696[2], 0.0])) - ((Lanes([0.0, v17264[0], v17264[1], v17264[2], 0.0])) + (((v14839 * (v14600 / (v14869 * v11088))) - (Lanes([0.0, v17261[0], 0.0, 0.0, 0.0]))) * v11087));
                let v17272 = v16580 * v10695;
                let v17275 = (Lanes([v17272[0], 0.0, 0.0])) + (v17225 * v10591);
                let v11095 = (v11092 + v10577) + (v10591 * v10695);
                let v17277 = (v17269 + (Lanes([0.0, v16568[0], v16568[1], v16568[2], 0.0]))) + (Lanes([0.0, v17275[0], v17275[1], v17275[2], 0.0]));
                let v11096 = v11092 + v10566;
                let v17279 = v17269 + (Lanes([0.0, v14815[0], v14815[1], v14815[2], 0.0]));
                v11104 = v11096;
                v11106 = v11095;
                v14837 = v17279;
                v14838 = v17277;
            } else {
                let v17227 = v16580 * v10695;
                let v11099 = (v10694 + v10577) + (v10591 * v10695);
                let v17231 = (v16696 + v16568) + ((Lanes([v17227[0], 0.0, 0.0])) + (v17225 * v10591));
                let v11100 = v10694 + v10566;
                let v17232 = v16696 + v14815;
                let v17233 = Lanes([0.0, v17232[0], v17232[1], v17232[2], 0.0]);
                let v17234 = Lanes([0.0, v17231[0], v17231[1], v17231[2], 0.0]);
                v11104 = v11100;
                v11106 = v11099;
                v14837 = v17233;
                v14838 = v17234;
            }
            let v11102 = (v10726 - v11070) / v10555;
            let v17282 = v14813 * v11102;
            let v17285 = (((Lanes([0.0, v14817[0], v14817[1], v14817[2], v14817[3]])) - v17221) - (Lanes([0.0, v17282[0], v17282[1], v17282[2], 0.0]))) / v10555;
            let v11105 = (-v11102) + v11104;
            let v17287 = (v17285 * v14888) + v14837;
            let v11108 = (v11102 - v11106) * v4897;
            let v11109 = rspice_limited_exp(v11108);
            let v17291 = ((v17285 - v14838) * v4897) * (rspice_limited_exp_derivative(v11108));
            let v11110 = if v11109 > v4907 { 1.0 } else { 0.0 };
            let v11237: f64;
            let v14840: Lanes<5>;
            if v11110 != 0.0 {
                let v11111 = v1 + v11109;
                let v11112 = v11111.ln();
                let v17298 = (v17291 * (v14600 / v11111)) * v11112;
                let v11115 = (v1 + (v11112 * v11112)).sqrt();
                let v11117 = v23 * (v1 - v11115);
                let v17304 = (((v17298 + v17298) * (v14600 / (v14869 * v11115))) * v14888) * v23;
                let v17306 = Lanes([0.0, v16546[0], v16546[1], v16546[2], 0.0]);
                let v11120 = ((v11117 * v10744) + v10557) * v414;
                let v17308 = ((v17304 * v10744) + v17306) * v414;
                let v11123 = ((rspice_limited_exp(v11120)) - v11120) - v1;
                let v11124 = v11120 / v11123;
                let v17314 = (v17308 - (((v17308 * (rspice_limited_exp_derivative(v11120))) - v17308) * v11124)) / v11123;
                let v11125 = v11120 * v11124;
                let v17317 = (v17308 * v11124) + (v17314 * v11120);
                let v11127 = -(v11117 + v10557);
                let v11128 = v11127.ln();
                let v17321 = ((v17304 + v17306) * v14888) * (v14600 / v11127);
                let v11129 = -v11117;
                let v17322 = v17304 * v14888;
                let v11130 = if v11129 > v185 { 1.0 } else { 0.0 };
                let v11137: f64;
                let v14841: Lanes<5>;
                if v11130 != 0.0 {
                    let v11131 = v11129.ln();
                    let v17324 = v17322 * (v14600 / v11129);
                    v11137 = v11131;
                    v14841 = v17324;
                } else {
                    v11137 = v11132;
                    v14841 = v16802;
                }
                let v11133 = if v11125 > v185 { 1.0 } else { 0.0 };
                let v11139: f64;
                let v14842: Lanes<5>;
                if v11133 != 0.0 {
                    let v11134 = v11125.ln();
                    let v17326 = v17317 * (v14600 / v11125);
                    v11139 = v11134;
                    v14842 = v17326;
                } else {
                    v11139 = v11135;
                    v14842 = v16802;
                }
                let v11142 = (v10585 * v11128).exp();
                let v17332 = v16580 * v11142;
                let v11144 = (((v11105 - v11117) + v11137) + v11139) + (v10591 * v11142);
                let v17336 = (((v17287 - v17304) + v14841) + v14842) + ((Lanes([0.0, v17332[0], 0.0, 0.0, 0.0])) + (((v17321 * v10585) * v11142) * v10591));
                let v11146 = v1 / v11117;
                let v11148 = v23 / v11120;
                let v11153 = v10585 * v10591;
                let v17346 = v16580 * v10585;
                let v11156 = (v11154 * v11128).exp();
                let v17349 = v17346 * v11156;
                let v11158 = ((v11145 + v11146) + (((v11148 - v11124) - v1) * v414)) - (v11153 * v11156);
                let v17353 = ((((v17304 * v11146) * v14888) / v11117) + (((((v17308 * v11148) * v14888) / v11120) - v17314) * v414)) - ((Lanes([0.0, v17349[0], 0.0, 0.0, 0.0])) + (((v17321 * v11154) * v11156) * v11153));
                let v11160 = v11117 * v11117;
                let v17354 = v17304 * v11117;
                let v11161 = v11159 / v11160;
                let v11163 = v11162 * v10591;
                let v11166 = (v11164 * v11128).exp();
                let v17362 = (v16580 * v11162) * v11166;
                let v11168 = v11161 - (v11163 * v11166);
                let v11169 = v11144 / v11158;
                let v11171 = v23 * v11158;
                let v11172 = v11171 * v11158;
                let v11173 = (v11144 * v11168) / v11172;
                let v11174 = v1 + v11173;
                let v11176 = v11117 - (v11169 * v11174);
                let v17383 = v17304 - ((((v17336 - (v17353 * v11169)) / v11158) * v11174) + (((((v17336 * v11168) + ((((((v17354 + v17354) * v11161) * v14888) / v11160) - ((Lanes([0.0, v17362[0], 0.0, 0.0, 0.0])) + (((v17321 * v11164) * v11166) * v11163))) * v11144)) - ((((v17353 * v23) * v11158) + (v17353 * v11171)) * v11173)) / v11172) * v11169));
                let v11179 = ((v11176 * v10744) + v10557) * v414;
                let v17386 = ((v17383 * v10744) + v17306) * v414;
                let v11182 = ((rspice_limited_exp(v11179)) - v11179) - v1;
                let v11183 = v11179 / v11182;
                let v17392 = (v17386 - (((v17386 * (rspice_limited_exp_derivative(v11179))) - v17386) * v11183)) / v11182;
                let v11184 = v11179 * v11183;
                let v17395 = (v17386 * v11183) + (v17392 * v11179);
                let v11186 = -(v11176 + v10557);
                let v11187 = v11186.ln();
                let v17399 = ((v17383 + v17306) * v14888) * (v14600 / v11186);
                let v11188 = -v11176;
                let v17400 = v17383 * v14888;
                let v11189 = if v11188 > v185 { 1.0 } else { 0.0 };
                let v11196: f64;
                let v14843: Lanes<5>;
                if v11189 != 0.0 {
                    let v11190 = v11188.ln();
                    let v17402 = v17400 * (v14600 / v11188);
                    v11196 = v11190;
                    v14843 = v17402;
                } else {
                    v11196 = v11191;
                    v14843 = v16802;
                }
                let v11192 = if v11184 > v185 { 1.0 } else { 0.0 };
                let v11198: f64;
                let v14844: Lanes<5>;
                if v11192 != 0.0 {
                    let v11193 = v11184.ln();
                    let v17404 = v17395 * (v14600 / v11184);
                    v11198 = v11193;
                    v14844 = v17404;
                } else {
                    v11198 = v11194;
                    v14844 = v16802;
                }
                let v11201 = (v10585 * v11187).exp();
                let v17410 = v16580 * v11201;
                let v11203 = (((v11105 - v11176) + v11196) + v11198) + (v10591 * v11201);
                let v17414 = (((v17287 - v17383) + v14843) + v14844) + ((Lanes([0.0, v17410[0], 0.0, 0.0, 0.0])) + (((v17399 * v10585) * v11201) * v10591));
                let v11205 = v1 / v11176;
                let v11207 = v23 / v11179;
                let v11214 = (v11212 * v11187).exp();
                let v17426 = v17346 * v11214;
                let v11216 = ((v11204 + v11205) + (((v11207 - v11183) - v1) * v414)) - (v11153 * v11214);
                let v17430 = ((((v17383 * v11205) * v14888) / v11176) + (((((v17386 * v11207) * v14888) / v11179) - v17392) * v414)) - ((Lanes([0.0, v17426[0], 0.0, 0.0, 0.0])) + (((v17399 * v11212) * v11214) * v11153));
                let v11218 = v11176 * v11176;
                let v17431 = v17383 * v11176;
                let v11219 = v11217 / v11218;
                let v11221 = v11220 * v10591;
                let v11224 = (v11222 * v11187).exp();
                let v17439 = (v16580 * v11220) * v11224;
                let v11226 = v11219 - (v11221 * v11224);
                let v11227 = v11203 / v11216;
                let v11229 = v23 * v11216;
                let v11230 = v11229 * v11216;
                let v11231 = (v11203 * v11226) / v11230;
                let v11232 = v1 + v11231;
                let v11234 = v11176 - (v11227 * v11232);
                let v17460 = v17383 - ((((v17414 - (v17430 * v11227)) / v11216) * v11232) + (((((v17414 * v11226) + ((((((v17431 + v17431) * v11219) * v14888) / v11218) - ((Lanes([0.0, v17439[0], 0.0, 0.0, 0.0])) + (((v17399 * v11222) * v11224) * v11221))) * v11203)) - ((((v17430 * v23) * v11216) + (v17430 * v11229)) * v11231)) / v11230) * v11227));
                v11237 = v11234;
                v14840 = v17460;
            } else {
                let v11235 = -v11109;
                let v11236 = v11235 * v11109;
                let v17295 = ((v17291 * v14888) * v11109) + (v17291 * v11235);
                v11237 = v11236;
                v14840 = v17295;
            }
            let v11238 = -v11237;
            let v11239 = v11238 * v10555;
            let v17463 = v14813 * v11238;
            let v17465 = ((v14840 * v14888) * v10555) + (Lanes([0.0, v17463[0], v17463[1], v17463[2], 0.0]));
            let v12120: f64;
            let v14845: Lanes<5>;
            if v4000 != 0.0 {
                let v11240 = v10624 - v11070;
                let v17467 = (Lanes([0.0, v16613[0], v16613[1], v16613[2], v16613[3]])) - v17221;
                let v11241 = v11240 / v10555;
                let v17468 = v14813 * v11241;
                let v17471 = (v17467 - (Lanes([0.0, v17468[0], v17468[1], v17468[2], 0.0]))) / v10555;
                let v17472 = v17471 * v11241;
                let v11246 = ((v11241 * v11241) + ((v5721 * v10871) * v10871)).sqrt();
                let v11248 = v4897 * (v11241 + v11246);
                let v17478 = (v17471 + ((v17472 + v17472) * (v14600 / (v14869 * v11246)))) * v4897;
                let v11249 = v10878 / v23;
                let v11251 = v10881 * (v11248.powf(v11249));
                let v11252 = v11241 - v11248;
                let v11253 = rspice_limited_exp(v11252);
                let v11256 = (v11240 - v10886) / v10555;
                let v17490 = v14813 * v11256;
                let v17493 = (v17467 - (Lanes([0.0, v17490[0], v17490[1], v17490[2], 0.0]))) / v10555;
                let v17494 = v17493 * v11256;
                let v11261 = ((v11256 * v11256) + ((v5721 * v10890) * v10890)).sqrt();
                let v11263 = v4897 * (v11256 + v11261);
                let v17500 = (v17493 + ((v17494 + v17494) * (v14600 / (v14869 * v11261)))) * v4897;
                let v11264 = v10897 / v23;
                let v11266 = v10900 * (v11263.powf(v11264));
                let v11267 = v11256 - v11263;
                let v11268 = rspice_limited_exp(v11267);
                let v11271 = (v11240 - v10905) / v10555;
                let v17512 = v14813 * v11271;
                let v17515 = (v17467 - (Lanes([0.0, v17512[0], v17512[1], v17512[2], 0.0]))) / v10555;
                let v17516 = v17515 * v11271;
                let v11276 = ((v11271 * v11271) + ((v5721 * v10909) * v10909)).sqrt();
                let v11278 = v4897 * (v11271 + v11276);
                let v17522 = (v17515 + ((v17516 + v17516) * (v14600 / (v14869 * v11276)))) * v4897;
                let v11279 = v10916 / v23;
                let v11281 = v10919 * (v11278.powf(v11279));
                let v11282 = v11271 - v11278;
                let v11283 = rspice_limited_exp(v11282);
                let v11289 = (v10924 * v11239) + (v10926 * (((v11251 * v11253) + (v11266 * v11268)) + (v11281 * v11283)));
                let v17538 = (v17465 * v10924) + (((((((v17478 * (v11249 * (v11248.powf((v11249 - v14600))))) * v10881) * v11253) + (((v17471 - v17478) * (rspice_limited_exp_derivative(v11252))) * v11251)) + ((((v17500 * (v11264 * (v11263.powf((v11264 - v14600))))) * v10900) * v11268) + (((v17493 - v17500) * (rspice_limited_exp_derivative(v11267))) * v11266))) + ((((v17522 * (v11279 * (v11278.powf((v11279 - v14600))))) * v10919) * v11283) + (((v17515 - v17522) * (rspice_limited_exp_derivative(v11282))) * v11281))) * v10926);
                v12120 = v11289;
                v14845 = v17538;
            } else {
                v12120 = v11239;
                v14845 = v17465;
            }
            let v12213: f64;
            let v12220: f64;
            let v12223: f64;
            let v12440: f64;
            let v12605: f64;
            if v3728 != 0.0 {
                let v11301 = v10351 - ((((v10598 + ((((-v11290) * v10601) * v10607) + v10612)) + v10616) + v10619) + v10622);
                let v11310 = ((v11302 * v421) * v423) / v3969;
                let v11404: f64;
                if v6778 != 0.0 {
                    let v11317 = ((v10532 * v10629) / ((((v11310 * v10555) * v415) * v6837) * v211)).powf(v10555);
                    let v11318 = if v11317 > v185 { 1.0 } else { 0.0 };
                    let v11321: f64;
                    if v11318 != 0.0 {
                        let v11319 = v11317.ln();
                        v11321 = v11319;
                    } else {
                        v11321 = v11320;
                    }
                    let v11323 = -(v6851 + v11321);
                    let v11325 = (v11301 + v11323) + v10644;
                    let v11327 = if v11325 < v11326 { 1.0 } else { 0.0 };
                    let v11336: f64;
                    if v11327 != 0.0 {
                        let v11329 = v11328 / v11325;
                        v11336 = v11329;
                    } else {
                        let v11335 = v4897 * (v11325 + (((v11325 * v11325) + v11331).sqrt()));
                        v11336 = v11335;
                    }
                    let v11337 = v11336 - v11323;
                    v11404 = v11337;
                } else {
                    let v11343 = (v10532 * v10629) / ((((v11310 * v10555) * v415) * v6837) * v211);
                    let v11344 = if v11343 > v185 { 1.0 } else { 0.0 };
                    let v11348: f64;
                    if v11344 != 0.0 {
                        let v11345 = v11343.ln();
                        v11348 = v11345;
                    } else {
                        v11348 = v11346;
                    }
                    let v11349 = (-v10555) * v11348;
                    let v11352 = v11349 - v4536;
                    let v11359 = (-v6851) + (v4897 * ((v11349 + v4536) + (((v11352 * v11352) + v11354).sqrt())));
                    let v11361 = (v11301 + v11359) + v10644;
                    let v11363 = if v11361 < v11362 { 1.0 } else { 0.0 };
                    let v11372: f64;
                    if v11363 != 0.0 {
                        let v11365 = v11364 / v11361;
                        v11372 = v11365;
                    } else {
                        let v11371 = v4897 * (v11361 + (((v11361 * v11361) + v11367).sqrt()));
                        v11372 = v11371;
                    }
                    let v11373 = v11372 - v11359;
                    v11404 = v11373;
                }
                let v11408: f64;
                let v11410: f64;
                if v97 != 0.0 {
                    let v11374 = v23 * v10529;
                    let v11376 = (v11374 + v6851) - v10357;
                    let v11378 = if v11376 < v11377 { 1.0 } else { 0.0 };
                    let v11387: f64;
                    if v11378 != 0.0 {
                        let v11380 = v11379 / v11376;
                        v11387 = v11380;
                    } else {
                        let v11386 = v4897 * (v11376 + (((v11376 * v11376) + v11382).sqrt()));
                        v11387 = v11386;
                    }
                    let v11395 = v10694 - (((-v9269) / (v23 * v10555)) * ((v11387.sqrt()) - (v11374.sqrt())));
                    let v11398 = (v11395 + v10577) + (v10591 * v10695);
                    let v11399 = v11395 + v10566;
                    v11408 = v11399;
                    v11410 = v11398;
                } else {
                    let v11402 = (v10694 + v10577) + (v10591 * v10695);
                    let v11403 = v10694 + v10566;
                    v11408 = v11403;
                    v11410 = v11402;
                }
                let v11406 = (v11404 - v6851) / v10555;
                let v11409 = (-v11406) + v11408;
                let v11413 = rspice_limited_exp((v4897 * (v11406 - v11410)));
                let v11414 = if v11413 > v4907 { 1.0 } else { 0.0 };
                let v11541: f64;
                if v11414 != 0.0 {
                    let v11416 = (v1 + v11413).ln();
                    let v11421 = v23 * (v1 - ((v1 + (v11416 * v11416)).sqrt()));
                    let v11424 = ((v11421 * v10744) + v10557) * v414;
                    let v11428 = v11424 / (((rspice_limited_exp(v11424)) - v11424) - v1);
                    let v11429 = v11424 * v11428;
                    let v11432 = (-(v11421 + v10557)).ln();
                    let v11433 = -v11421;
                    let v11434 = if v11433 > v185 { 1.0 } else { 0.0 };
                    let v11441: f64;
                    if v11434 != 0.0 {
                        let v11435 = v11433.ln();
                        v11441 = v11435;
                    } else {
                        v11441 = v11436;
                    }
                    let v11437 = if v11429 > v185 { 1.0 } else { 0.0 };
                    let v11443: f64;
                    if v11437 != 0.0 {
                        let v11438 = v11429.ln();
                        v11443 = v11438;
                    } else {
                        v11443 = v11439;
                    }
                    let v11448 = (((v11409 - v11421) + v11441) + v11443) + (v10591 * ((v10585 * v11432).exp()));
                    let v11457 = v10585 * v10591;
                    let v11462 = ((v11449 + (v1 / v11421)) + ((((v23 / v11424) - v11428) - v1) * v414)) - (v11457 * ((v11458 * v11432).exp()));
                    let v11480 = v11421 - ((v11448 / v11462) * (v1 + ((v11448 * ((v11463 / (v11421 * v11421)) - ((v11466 * v10591) * ((v11468 * v11432).exp())))) / ((v23 * v11462) * v11462))));
                    let v11483 = ((v11480 * v10744) + v10557) * v414;
                    let v11487 = v11483 / (((rspice_limited_exp(v11483)) - v11483) - v1);
                    let v11488 = v11483 * v11487;
                    let v11491 = (-(v11480 + v10557)).ln();
                    let v11492 = -v11480;
                    let v11493 = if v11492 > v185 { 1.0 } else { 0.0 };
                    let v11500: f64;
                    if v11493 != 0.0 {
                        let v11494 = v11492.ln();
                        v11500 = v11494;
                    } else {
                        v11500 = v11495;
                    }
                    let v11496 = if v11488 > v185 { 1.0 } else { 0.0 };
                    let v11502: f64;
                    if v11496 != 0.0 {
                        let v11497 = v11488.ln();
                        v11502 = v11497;
                    } else {
                        v11502 = v11498;
                    }
                    let v11507 = (((v11409 - v11480) + v11500) + v11502) + (v10591 * ((v10585 * v11491).exp()));
                    let v11520 = ((v11508 + (v1 / v11480)) + ((((v23 / v11483) - v11487) - v1) * v414)) - (v11457 * ((v11516 * v11491).exp()));
                    let v11538 = v11480 - ((v11507 / v11520) * (v1 + ((v11507 * ((v11521 / (v11480 * v11480)) - ((v11524 * v10591) * ((v11526 * v11491).exp())))) / ((v23 * v11520) * v11520))));
                    v11541 = v11538;
                } else {
                    let v11540 = (-v11413) * v11413;
                    v11541 = v11540;
                }
                let v11543 = (-v11541) * v10555;
                let v11594: f64;
                if v4000 != 0.0 {
                    let v11544 = v11301 - v6851;
                    let v11545 = v11544 / v10555;
                    let v11552 = v4897 * (v11545 + (((v11545 * v11545) + ((v5721 * v10871) * v10871)).sqrt()));
                    let v11560 = (v11544 - v10886) / v10555;
                    let v11567 = v4897 * (v11560 + (((v11560 * v11560) + ((v5721 * v10890) * v10890)).sqrt()));
                    let v11575 = (v11544 - v10905) / v10555;
                    let v11582 = v4897 * (v11575 + (((v11575 * v11575) + ((v5721 * v10909) * v10909)).sqrt()));
                    let v11593 = (v10924 * v11543) + (v10926 * ((((v10881 * (v11552.powf((v10878 / v23)))) * (rspice_limited_exp((v11545 - v11552)))) + ((v10900 * (v11567.powf((v10897 / v23)))) * (rspice_limited_exp((v11560 - v11567))))) + ((v10919 * (v11582.powf((v10916 / v23)))) * (rspice_limited_exp((v11575 - v11582))))));
                    v11594 = v11593;
                } else {
                    v11594 = v11543;
                }
                let v11601 = (v4897 * (v1 + (v11594 / v10931))).powf(v10940);
                let v11602 = (v5572 * (v5654 + (v10932 * v11594))).powf(v10943);
                let v11628: f64;
                if v97 != 0.0 {
                    let v11624 = ((v11603 + (v11609 * v10947)) * v11602) + (v11617 / v11601);
                    v11628 = v11624;
                } else {
                    let v11627 = (v11603 * v11602) + (v11617 / v11601);
                    v11628 = v11627;
                }
                let v11629 = v1 + v11628;
                let v11631 = v11629 - v1;
                let v11637 = (v4897 * ((v11629 + v1) + (((v11631 * v11631) + v10964).sqrt()))) / v10969;
                let v11676: f64;
                if v4402 != 0.0 {
                    v11676 = v0;
                } else {
                    let v11677: f64;
                    if v4874 != 0.0 {
                        let v11640 = v1 / (v1 + (v10971 * v11594));
                        let v11650 = (((v10980 + (v10981 * (v4897 * (v11640 + (((v11640 * v11640) + v4536).sqrt()))))) * v5577) * v153) * v10986;
                        v11677 = v11650;
                    } else {
                        let v11653 = v1 / (v1 + (v10971 * v11594));
                        let v11665 = ((v10997 + v11000) + (((v10980 + (v10981 * (v4897 * (v11653 + (((v11653 * v11653) + v4536).sqrt()))))) * v5577) * v153)) * v10986;
                        v11677 = v11665;
                    }
                    v11676 = v11677;
                }
                let v11670 = (((v23 * v11666) / v11302) * v11637) * v3969;
                let v11683: f64;
                if v6778 != 0.0 {
                    let v11672 = v11015 * (v11594 + v6773);
                    v11683 = v11672;
                } else {
                    let v11675 = v11015 * (v11594 + (v23 * v10549));
                    v11683 = v11675;
                }
                let v11678 = if v11676 > v0 { 1.0 } else { 0.0 };
                let v11704: f64;
                if v11678 != 0.0 {
                    let v11681 = ((v423 * v11666) * v421) * v11676;
                    let v11682 = v23 * v11681;
                    let v11687 = (v11683 + v11670) + ((v276 * v11683) * v11681);
                    let v11692 = v11687 * v11687;
                    let v11695 = v11692 - ((v23 * v11682) * (v11683 * (v11670 + ((v23 * v11683) * v11681))));
                    let v11700 = (v11692 - v11695) / ((v11687 + (v11695.sqrt())) * v11682);
                    v11704 = v11700;
                } else {
                    let v11703 = (v11670 * v11683) / (v11670 + v11683);
                    v11704 = v11703;
                }
                let v11705 = v11704 - v4637;
                let v11707 = if v11705 < v11706 { 1.0 } else { 0.0 };
                let v11716: f64;
                if v11707 != 0.0 {
                    let v11709 = v11708 / v11705;
                    v11716 = v11709;
                } else {
                    let v11715 = v4897 * (v11705 + (((v11705 * v11705) + v11711).sqrt()));
                    v11716 = v11715;
                }
                let v11724 = if (v10352 / ((v1 + (((v10352 / (v11716 + v4637)) + v25).powf(v10526))).powf(v10527))) <= v10352 { (v10352 / ((v1 + (((v10352 / (v11716 + v4637)) + v25).powf(v10526))).powf(v10527))) } else { v10352 };
                let v11725 = v11724 + v6851;
                let v11759: f64;
                let v11761: f64;
                if v97 != 0.0 {
                    let v11726 = v23 * v10529;
                    let v11728 = (v11726 + v11725) - v10357;
                    let v11730 = if v11728 < v11729 { 1.0 } else { 0.0 };
                    let v11739: f64;
                    if v11730 != 0.0 {
                        let v11732 = v11731 / v11728;
                        v11739 = v11732;
                    } else {
                        let v11738 = v4897 * (v11728 + (((v11728 * v11728) + v11734).sqrt()));
                        v11739 = v11738;
                    }
                    let v11747 = v10694 - (((-v9269) / (v23 * v10555)) * ((v11739.sqrt()) - (v11726.sqrt())));
                    let v11750 = (v11747 + v10577) + (v10591 * v10695);
                    let v11751 = v11747 + v10566;
                    v11759 = v11751;
                    v11761 = v11750;
                } else {
                    let v11754 = (v10694 + v10577) + (v10591 * v10695);
                    let v11755 = v10694 + v10566;
                    v11759 = v11755;
                    v11761 = v11754;
                }
                let v11757 = (v11404 - v11725) / v10555;
                let v11760 = (-v11757) + v11759;
                let v11764 = rspice_limited_exp(((v11757 - v11761) * v4897));
                let v11765 = if v11764 > v4907 { 1.0 } else { 0.0 };
                let v11892: f64;
                if v11765 != 0.0 {
                    let v11767 = (v1 + v11764).ln();
                    let v11772 = v23 * (v1 - ((v1 + (v11767 * v11767)).sqrt()));
                    let v11775 = ((v11772 * v10744) + v10557) * v414;
                    let v11779 = v11775 / (((rspice_limited_exp(v11775)) - v11775) - v1);
                    let v11780 = v11775 * v11779;
                    let v11783 = (-(v11772 + v10557)).ln();
                    let v11784 = -v11772;
                    let v11785 = if v11784 > v185 { 1.0 } else { 0.0 };
                    let v11792: f64;
                    if v11785 != 0.0 {
                        let v11786 = v11784.ln();
                        v11792 = v11786;
                    } else {
                        v11792 = v11787;
                    }
                    let v11788 = if v11780 > v185 { 1.0 } else { 0.0 };
                    let v11794: f64;
                    if v11788 != 0.0 {
                        let v11789 = v11780.ln();
                        v11794 = v11789;
                    } else {
                        v11794 = v11790;
                    }
                    let v11799 = (((v11760 - v11772) + v11792) + v11794) + (v10591 * ((v10585 * v11783).exp()));
                    let v11808 = v10585 * v10591;
                    let v11813 = ((v11800 + (v1 / v11772)) + ((((v23 / v11775) - v11779) - v1) * v414)) - (v11808 * ((v11809 * v11783).exp()));
                    let v11831 = v11772 - ((v11799 / v11813) * (v1 + ((v11799 * ((v11814 / (v11772 * v11772)) - ((v11817 * v10591) * ((v11819 * v11783).exp())))) / ((v23 * v11813) * v11813))));
                    let v11834 = ((v11831 * v10744) + v10557) * v414;
                    let v11838 = v11834 / (((rspice_limited_exp(v11834)) - v11834) - v1);
                    let v11839 = v11834 * v11838;
                    let v11842 = (-(v11831 + v10557)).ln();
                    let v11843 = -v11831;
                    let v11844 = if v11843 > v185 { 1.0 } else { 0.0 };
                    let v11851: f64;
                    if v11844 != 0.0 {
                        let v11845 = v11843.ln();
                        v11851 = v11845;
                    } else {
                        v11851 = v11846;
                    }
                    let v11847 = if v11839 > v185 { 1.0 } else { 0.0 };
                    let v11853: f64;
                    if v11847 != 0.0 {
                        let v11848 = v11839.ln();
                        v11853 = v11848;
                    } else {
                        v11853 = v11849;
                    }
                    let v11858 = (((v11760 - v11831) + v11851) + v11853) + (v10591 * ((v10585 * v11842).exp()));
                    let v11871 = ((v11859 + (v1 / v11831)) + ((((v23 / v11834) - v11838) - v1) * v414)) - (v11808 * ((v11867 * v11842).exp()));
                    let v11889 = v11831 - ((v11858 / v11871) * (v1 + ((v11858 * ((v11872 / (v11831 * v11831)) - ((v11875 * v10591) * ((v11877 * v11842).exp())))) / ((v23 * v11871) * v11871))));
                    v11892 = v11889;
                } else {
                    let v11891 = (-v11764) * v11764;
                    v11892 = v11891;
                }
                let v11894 = (-v11892) * v10555;
                let v11945: f64;
                if v4000 != 0.0 {
                    let v11895 = v11301 - v11725;
                    let v11896 = v11895 / v10555;
                    let v11903 = v4897 * (v11896 + (((v11896 * v11896) + ((v5721 * v10871) * v10871)).sqrt()));
                    let v11911 = (v11895 - v10886) / v10555;
                    let v11918 = v4897 * (v11911 + (((v11911 * v11911) + ((v5721 * v10890) * v10890)).sqrt()));
                    let v11926 = (v11895 - v10905) / v10555;
                    let v11933 = v4897 * (v11926 + (((v11926 * v11926) + ((v5721 * v10909) * v10909)).sqrt()));
                    let v11944 = (v10924 * v11894) + (v10926 * ((((v10881 * (v11903.powf((v10878 / v23)))) * (rspice_limited_exp((v11896 - v11903)))) + ((v10900 * (v11918.powf((v10897 / v23)))) * (rspice_limited_exp((v11911 - v11918))))) + ((v10919 * (v11933.powf((v10916 / v23)))) * (rspice_limited_exp((v11926 - v11933))))));
                    v11945 = v11944;
                } else {
                    v11945 = v11894;
                }
                let v11947 = v4897 * (v11594 + v11945);
                let v11948 = v11594 - v11945;
                let v11951 = (v11724 * v11724) / v11950;
                let v11953 = if v11952 != v0 { 1.0 } else { 0.0 };
                let v12214: f64;
                if v11953 != 0.0 {
                    let v11960 = v11947 + (((v11952 * (v1 - (rspice_limited_exp((-v11951))))) * v4897) * v11948);
                    v12214 = v11960;
                } else {
                    v12214 = v11947;
                }
                v12213 = v12214;
                v12220 = v11594;
                v12223 = v11945;
                v12440 = v11948;
                v12605 = v11947;
            } else {
                v12213 = v12215;
                v12220 = v12221;
                v12223 = v12224;
                v12440 = v12441;
                v12605 = v12606;
            }
            let v12149: f64;
            let v12634: f64;
            let v12636: f64;
            if v97 != 0.0 {
                let v11964 = (v9269 / (v23 * v10555)) * (v6600.sqrt());
                let v11965 = v11964 / v23;
                let v11966 = v417 / v6837;
                let v11967 = if v11966 > v185 { 1.0 } else { 0.0 };
                let v11971: f64;
                if v11967 != 0.0 {
                    let v11968 = v11966.ln();
                    v11971 = v11968;
                } else {
                    v11971 = v11969;
                }
                let v11977 = (v10332 - (((v10041 - v6760) - (v6600 * v11971)) + v11974)) / v6600;
                let v11983 = if (v11977 * v6600) > (v10529 + (v11964 * ((v10529 * v6600).sqrt()))) { 1.0 } else { 0.0 };
                let v12044: f64;
                let v12046: f64;
                if v11983 != 0.0 {
                    let v11988 = (((v11977 - v1) + (v11965 * v11965)).sqrt()) - v11965;
                    let v11990 = v1 + (v11988 * v11988);
                    let v11991 = -v11990;
                    let v11993 = if (v11991.abs()) < v4907 { 1.0 } else { 0.0 };
                    let v11999: f64;
                    if v11993 != 0.0 {
                        let v11996 = v11991 + ((v4897 * v11991) * v11991);
                        v11999 = v11996;
                    } else {
                        let v11998 = (rspice_limited_exp(v11991)) - v1;
                        v11999 = v11998;
                    }
                    v12044 = v11999;
                    v12046 = v11990;
                } else {
                    let v12005 = (v11977 * v4897) - (v276 * (v1 + (v11964 / v12001)));
                    let v12010 = v12005 + (((v12005 * v12005) + (v6356 * v11977)).sqrt());
                    let v12011 = if v11977 < v0 { 1.0 } else { 0.0 };
                    let v12045: f64;
                    let v12047: f64;
                    if v12011 != 0.0 {
                        let v12013 = (v11977 - v12010) / v11964;
                        let v12015 = v12013 * v12013;
                        let v12016 = (-v12010) + v12015;
                        let v12018 = (v1 - v12010) + v12015;
                        let v12019 = if v12018 > v185 { 1.0 } else { 0.0 };
                        let v12022: f64;
                        if v12019 != 0.0 {
                            let v12020 = v12018.ln();
                            v12022 = v12020;
                        } else {
                            v12022 = v12021;
                        }
                        let v12023 = -v12022;
                        v12045 = v12016;
                        v12047 = v12023;
                    } else {
                        let v12025 = rspice_limited_exp((-v12010));
                        let v12031 = ((((v11977 - v1) + v12025) + (v11965 * v11965)).sqrt()) - v11965;
                        let v12034 = (v1 - v12025) + (v12031 * v12031);
                        let v12035 = -v12034;
                        let v12037 = if (v12035.abs()) < v4907 { 1.0 } else { 0.0 };
                        let v12043: f64;
                        if v12037 != 0.0 {
                            let v12040 = v12035 + ((v4897 * v12035) * v12035);
                            v12043 = v12040;
                        } else {
                            let v12042 = (rspice_limited_exp(v12035)) - v1;
                            v12043 = v12042;
                        }
                        v12045 = v12043;
                        v12047 = v12034;
                    }
                    v12044 = v12045;
                    v12046 = v12047;
                }
                let v12049 = (v12044 + v12046).sqrt();
                let v12051 = if v12046 > v12050 { 1.0 } else { 0.0 };
                let v12102: f64;
                let v12635: f64;
                if v12051 != 0.0 {
                    let v12061 = v12046 - (((-(v11977 - v12046)) + (v11964 * v12049)) / (v1 - (((v11964 * v4897) * v12044) / v12049)));
                    let v12062 = -v12061;
                    let v12064 = if (v12062.abs()) < v4907 { 1.0 } else { 0.0 };
                    let v12070: f64;
                    if v12064 != 0.0 {
                        let v12067 = v12062 + ((v4897 * v12062) * v12062);
                        v12070 = v12067;
                    } else {
                        let v12069 = (rspice_limited_exp(v12062)) - v1;
                        v12070 = v12069;
                    }
                    let v12075 = ((-v11964) * ((v12070 + v12061).sqrt())) * v6600;
                    v12102 = v12061;
                    v12635 = v12075;
                } else {
                    let v12077 = if v12046 < v12076 { 1.0 } else { 0.0 };
                    let v12100: f64;
                    let v12103: f64;
                    if v12077 != 0.0 {
                        let v12087 = v12046 - (((-(v11977 - v12046)) - (v11964 * v12049)) / (v1 + (((v11964 * v4897) * v12044) / v12049)));
                        let v12088 = -v12087;
                        let v12090 = if (v12088.abs()) < v4907 { 1.0 } else { 0.0 };
                        let v12096: f64;
                        if v12090 != 0.0 {
                            let v12093 = v12088 + ((v4897 * v12088) * v12088);
                            v12096 = v12093;
                        } else {
                            let v12095 = (rspice_limited_exp(v12088)) - v1;
                            v12096 = v12095;
                        }
                        let v12099 = v11964 * ((v12096 + v12087).sqrt());
                        v12100 = v12099;
                        v12103 = v12087;
                    } else {
                        v12100 = v0;
                        v12103 = v0;
                    }
                    let v12101 = v12100 * v6600;
                    v12102 = v12103;
                    v12635 = v12101;
                }
                let v12108 = (v11964 * (rspice_limited_exp(((-v12102) / v23)))) * v6600;
                let v12110 = v12102 - v1;
                let v12119 = v1 + (v11964 / ((v4897 * ((v12102 + v1) + (((v12110 * v12110) + v12112).sqrt()))).sqrt()));
                v12149 = v12108;
                v12634 = v12635;
                v12636 = v12119;
            } else {
                v12149 = v0;
                v12634 = v0;
                v12636 = v0;
            }
            let v12121 = v10933 + v12120;
            let v12122 = v4897 * v12121;
            let v17540 = (v14830 + v14845) * v4897;
            let v12123 = v10933 - v12120;
            let v17541 = v14830 - v14845;
            let v12124 = v11069 * v11069;
            let v17542 = v17219 * v11069;
            let v12125 = v12124 / v11950;
            let v17544 = (v17542 + v17542) / v11950;
            let v12126 = if v11952 != v0 { 1.0 } else { 0.0 };
            let v12150: f64;
            let v14846: Lanes<5>;
            if v12126 != 0.0 {
                let v12127 = -v12125;
                let v12131 = (v11952 * (v1 - (rspice_limited_exp(v12127)))) * v4897;
                let v12133 = v12122 + (v12131 * v12123);
                let v17554 = v17540 + (((((((v17544 * v14888) * (rspice_limited_exp_derivative(v12127))) * v14888) * v11952) * v4897) * v12123) + (v17541 * v12131));
                v12150 = v12133;
                v14846 = v17554;
            } else {
                v12150 = v12122;
                v14846 = v17540;
            }
            let v12631: f64;
            if v4506 != 0.0 {
                let v12146 = v1 / ((v1 / ((v421 * v209) / v4900)) + (((v12138 / (v1 + ((v12122 / v12134).powf(v4337)))) * v866) / v16));
                v12631 = v12146;
            } else {
                v12631 = v421;
            }
            let v12148 = if v97 != 0.0 && (if v882 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v12148 != 0.0 {
            } else {
            }
            let v17555 = v14791 * v12150;
            let v12153 = v5572 * (v5654 + (v10932 * v12150));
            let v17559 = ((Lanes([0.0, v17555[0], 0.0, 0.0, 0.0])) + (v14846 * v10932)) * v5572;
            let v12188: f64;
            let v14847: Lanes<5>;
            if v6778 != 0.0 {
                let v12156 = v4897 * (v1 + (v12150 / v10931));
                let v12157 = v12156.powf(v10940);
                let v17603 = v14644 * (v12157 * (v12156.ln()));
                let v17605 = (((v14846 / v10931) * v4897) * (v10940 * (v12156.powf(v17026)))) + (Lanes([0.0, v17603[0], 0.0, 0.0, 0.0]));
                v12188 = v12157;
                v14847 = v17605;
            } else {
                let v12158 = -v12125;
                let v12160 = v1 - (rspice_limited_exp(v12158));
                let v17564 = v14646 * v10933;
                let v17568 = v14647 * v12120;
                let v12167 = (v12161 * v10933) + (v12164 * v12120);
                let v12168 = v12167 * v12160;
                let v17575 = ((((Lanes([0.0, v17564[0], 0.0, 0.0, 0.0])) + (v14830 * v12161)) + ((Lanes([0.0, v17568[0], 0.0, 0.0, 0.0])) + (v14845 * v12164))) * v12160) + ((((v17544 * v14888) * (rspice_limited_exp_derivative(v12158))) * v14888) * v12167);
                let v12170 = if v12168 < v12169 { 1.0 } else { 0.0 };
                let v12179: f64;
                let v14848: Lanes<5>;
                if v12170 != 0.0 {
                    let v12172 = v12171 / v12168;
                    let v17585 = ((v17575 * v12172) * v14888) / v12168;
                    v12179 = v12172;
                    v14848 = v17585;
                } else {
                    let v17576 = v17575 * v12168;
                    let v12176 = ((v12168 * v12168) + v12174).sqrt();
                    let v12178 = v4897 * (v12168 + v12176);
                    let v17582 = (v17575 + ((v17576 + v17576) * (v14600 / (v14869 * v12176)))) * v4897;
                    v12179 = v12178;
                    v14848 = v17582;
                }
                let v12182 = v4897 * (v1 + (v12179 / v10931));
                let v12183 = v12182.powf(v10940);
                let v17593 = v14644 * (v12183 * (v12182.ln()));
                let v17595 = (((v14848 / v10931) * v4897) * (v10940 * (v12182.powf(v17026)))) + (Lanes([0.0, v17593[0], 0.0, 0.0, 0.0]));
                v12188 = v12183;
                v14847 = v17595;
            }
            let v12184 = v12153.powf(v10943);
            let v17611 = v14802 * (v12184 * (v12153.ln()));
            let v17613 = (v17559 * (v10943 * (v12153.powf(v17035)))) + (Lanes([0.0, v17611[0], v17611[1], v17611[2], 0.0]));
            let v12194: f64;
            let v14849: Lanes<5>;
            if v97 != 0.0 {
                let v17623 = v14804 * v10947;
                let v17624 = v14795 * v10946;
                let v12186 = v10945 + (v10946 * v10947);
                let v17630 = ((Lanes([0.0, v14803[0], v14803[1], v14803[2]])) + ((Lanes([0.0, v17623[0], v17623[1], v17623[2]])) + (Lanes([v17624[0], 0.0, v17624[1], v17624[2]])))) * v12184;
                let v12189 = v10951 / v12188;
                let v12190 = (v12186 * v12184) + v12189;
                let v17638 = ((Lanes([v17630[0], v17630[1], v17630[2], v17630[3], 0.0])) + (v17613 * v12186)) + (((Lanes([0.0, v14805[0], v14805[1], v14805[2], 0.0])) - (v14847 * v12189)) / v12188);
                v12194 = v12190;
                v14849 = v17638;
            } else {
                let v17614 = v14803 * v12184;
                let v12192 = v10951 / v12188;
                let v12193 = (v10945 * v12184) + v12192;
                let v17622 = ((Lanes([0.0, v17614[0], v17614[1], v17614[2], 0.0])) + (v17613 * v10945)) + (((Lanes([0.0, v14805[0], v14805[1], v14805[2], 0.0])) - (v14847 * v12192)) / v12188);
                v12194 = v12193;
                v14849 = v17622;
            }
            let v12195 = v1 + v12194;
            let v12197 = v12195 - v1;
            let v17639 = v14849 * v12197;
            let v12200 = ((v12197 * v12197) + v10964).sqrt();
            let v12205 = -v12204;
            let v12206 = v12205 * v11069;
            let v12210 = v10969 * (v1 - (v12203 * (rspice_limited_exp(v12206))));
            let v12211 = (v4897 * ((v12195 + v1) + v12200)) / v12210;
            let v17654 = (((v14849 + ((v17639 + v17639) * (v14600 / (v14869 * v12200)))) * v4897) - ((((((v17219 * v12205) * (rspice_limited_exp_derivative(v12206))) * v12203) * v14888) * v10969) * v12211)) / v12210;
            let v12212 = v10625 / v12211;
            let v17658 = ((Lanes([0.0, v14801[0], v14801[1], v14801[2], 0.0])) - (v17654 * v12212)) / v12211;
            let v12258: f64;
            if v3728 != 0.0 {
                let v12248: f64;
                if v6778 != 0.0 {
                    let v12219 = (v4897 * (v1 + (v12213 / v10931))).powf(v10940);
                    v12248 = v12219;
                } else {
                    let v12226 = (v12161 * v12220) + (v12164 * v12223);
                    let v12228 = if v12226 < v12227 { 1.0 } else { 0.0 };
                    let v12237: f64;
                    if v12228 != 0.0 {
                        let v12230 = v12229 / v12226;
                        v12237 = v12230;
                    } else {
                        let v12236 = v4897 * (v12226 + (((v12226 * v12226) + v12232).sqrt()));
                        v12237 = v12236;
                    }
                    let v12241 = (v4897 * (v1 + (v12237 / v10931))).powf(v10940);
                    v12248 = v12241;
                }
                let v12250 = (v11603 * ((v5572 * (v5654 + (v12242 * v12213))).powf(v10943))) + (v11617 / v12248);
                v12258 = v12250;
            } else {
                let v12257 = (v10945 * ((v5572 * (v5654 + (v12242 * v12150))).powf(v10943))) + (v10951 / v12188);
                v12258 = v12257;
            }
            let v12259 = v1 + v12258;
            let v12261 = v12259 - v1;
            let v12267 = (v4897 * ((v12259 + v1) + (((v12261 * v12261) + v10964).sqrt()))) / v12210;
            let v12271 = ((v12268 * v3969) / v5590) + v25;
            let v12272 = if v12271 < v5597 { 1.0 } else { 0.0 };
            let v12296: f64;
            let v14850: Lanes<3>;
            if v12272 != 0.0 {
                let v12276 = (v12271.cosh()) - v1;
                let v12279 = ((v4897 * v12273) / v12276) + v12278;
                let v17663 = ((v14808 * v4897) / v12276) + v14809;
                v12296 = v12279;
                v14850 = v17663;
            } else {
                let v12281 = rspice_limited_exp((-v12271));
                let v12283 = (v12273 * v12281) + v12278;
                let v17660 = (v14808 * v12281) + v14809;
                v12296 = v12283;
                v14850 = v17660;
            }
            let v12284 = if v1522 > v0 { 1.0 } else { 0.0 };
            let v12303: f64;
            let v14851: Lanes<5>;
            if v12284 != 0.0 {
                let v12286 = (v1522 * v12122) / v11014;
                let v17675 = ((v17540 * v1522) - (v17121 * v12286)) / v11014;
                let v12287 = v1 + v12286;
                v12303 = v12287;
                v14851 = v17675;
            } else {
                let v12289 = (v1522 * v12122) / v11014;
                let v12290 = v1 - v12289;
                let v12291 = v1 / v12290;
                let v17671 = ((((((v17540 * v1522) - (v17121 * v12289)) / v11014) * v14888) * v12291) * v14888) / v12290;
                v12303 = v12291;
                v14851 = v17671;
            }
            let v12292 = v10352 - v11069;
            let v17676 = v17192 - v17219;
            let v12298: f64;
            let v14852: Lanes<5>;
            if v6778 != 0.0 {
                let v12293 = v12122 + v6773;
                let v17681 = v17540 + (Lanes([0.0, v14954[0], 0.0, 0.0, 0.0]));
                v12298 = v12293;
                v14852 = v17681;
            } else {
                let v17677 = v14613 * v23;
                let v12295 = v12122 + (v23 * v10549);
                let v17679 = v17540 + (Lanes([0.0, v17677[0], 0.0, 0.0, 0.0]));
                v12298 = v12295;
                v14852 = v17679;
            }
            let v12297 = if v12296 > v0 { 1.0 } else { 0.0 };
            let v12327: f64;
            let v14853: Lanes<5>;
            if v12297 != 0.0 {
                let v12299 = v11062 + v12298;
                let v12300 = v12298 / v12299;
                let v12301 = v12298 / v12296;
                let v17686 = v14850 * v12301;
                let v12302 = v12301 * v12300;
                let v12304 = v12302 * v12303;
                let v12305 = v12292 / v12304;
                let v17698 = (v17676 - (((((((v14852 - (Lanes([0.0, v17686[0], v17686[1], v17686[2], 0.0]))) / v12296) * v12300) + (((v14852 - ((v14836 + v14852) * v12300)) / v12299) * v12301)) * v12303) + (v14851 * v12302)) * v12305)) / v12304;
                let v12306 = v1 + v12305;
                v12327 = v12306;
                v14853 = v17698;
            } else {
                v12327 = v1;
                v14853 = v16802;
            }
            let v12308 = if v12307 > v0 { 1.0 } else { 0.0 };
            let v12328: f64;
            let v14854: Lanes<5>;
            if v12308 != 0.0 {
                let v12309 = if v1282 < v0 { 1.0 } else { 0.0 };
                let v12316: f64;
                let v14855: Lanes<5>;
                if v12309 != 0.0 {
                    let v12310 = v1 / v12307;
                    let v17704 = ((v14810 * v12310) * v14888) / v12307;
                    let v12312 = v12310 - (v1282 * v12122);
                    let v12313 = v1 / v12312;
                    let v17710 = ((((Lanes([0.0, v17704[0], v17704[1], v17704[2], 0.0])) - (v17540 * v1282)) * v12313) * v14888) / v12312;
                    v12316 = v12313;
                    v14855 = v17710;
                } else {
                    let v12315 = v12307 + (v1282 * v12122);
                    let v17701 = (Lanes([0.0, v14810[0], v14810[1], v14810[2], 0.0])) + (v17540 * v1282);
                    v12316 = v12315;
                    v14855 = v17701;
                }
                let v12317 = v12292 / v12316;
                let v12318 = v11062 + v11014;
                let v12319 = v12317 / v12318;
                let v17717 = (((v17676 - (v14855 * v12317)) / v12316) - ((v14836 + v17121) * v12319)) / v12318;
                let v12320 = v1 + v12319;
                let v12321 = if v12320 > v185 { 1.0 } else { 0.0 };
                let v12324: f64;
                let v14856: Lanes<5>;
                if v12321 != 0.0 {
                    let v12322 = v12320.ln();
                    let v17719 = v17717 * (v14600 / v12320);
                    v12324 = v12322;
                    v14856 = v17719;
                } else {
                    v12324 = v12323;
                    v14856 = v16802;
                }
                let v17722 = (v14855 * v12324) + (v14856 * v12316);
                let v12326 = v1 + (v12316 * v12324);
                v12328 = v12326;
                v14854 = v17722;
            } else {
                v12328 = v1;
                v14854 = v16802;
            }
            let v12329 = v12327 * v12328;
            let v17725 = (v14853 * v12328) + (v14854 * v12327);
            let v17726 = v14811 * v23;
            let v12332 = (v23 * v12330) / v12212;
            let v12333 = v12332 * v3969;
            let v12334 = v12123 / v12333;
            let v17734 = (v17541 - (((((Lanes([0.0, v17726[0], v17726[1], v17726[2], 0.0])) - (v17658 * v12332)) / v12212) * v3969) * v12334)) / v12333;
            let v12335 = if v12334 > v185 { 1.0 } else { 0.0 };
            let v12339: f64;
            let v14857: Lanes<5>;
            if v12335 != 0.0 {
                let v12336 = v12334.ln();
                let v17736 = v17734 * (v14600 / v12334);
                v12339 = v12336;
                v14857 = v17736;
            } else {
                v12339 = v12337;
                v14857 = v16802;
            }
            let v12340 = v12338 * v12339;
            let v12341 = rspice_limited_exp(v12340);
            let v17739 = (v14857 * v12338) * (rspice_limited_exp_derivative(v12340));
            let v12342 = v1 / v12338;
            let v12343 = if v978 > v185 { 1.0 } else { 0.0 };
            let v12346: f64;
            if v12343 != 0.0 {
                let v12344 = v978.ln();
                v12346 = v12344;
            } else {
                v12346 = v12345;
            }
            let v12349 = v1 + (rspice_limited_exp((v12342 * v12346)));
            let v12350 = v978 + v12341;
            let v12351 = if v12350 > v185 { 1.0 } else { 0.0 };
            let v12354: f64;
            let v14858: Lanes<5>;
            if v12351 != 0.0 {
                let v12352 = v12350.ln();
                let v17741 = v17739 * (v14600 / v12350);
                v12354 = v12352;
                v14858 = v17741;
            } else {
                v12354 = v12353;
                v14858 = v16802;
            }
            let v12355 = v12342 * v12354;
            let v12360 = v4897 * v12359;
            let v12361 = v12360 * v12122;
            let v17747 = (v14812 * v4897) * v12122;
            let v12362 = v12361 * v12123;
            let v12364 = ((v1 + (rspice_limited_exp(v12355))) / v12349) + (v12362 * v12123);
            let v12365 = v23 * v10555;
            let v17758 = v14813 * v23;
            let v12366 = v12122 + v12365;
            let v17759 = Lanes([0.0, v17758[0], v17758[1], v17758[2], 0.0]);
            let v12367 = v9279 / v12366;
            let v12368 = v9277 + v12367;
            let v12369 = v12368 * v12123;
            let v17772 = (((((Lanes([0.0, v16278[0], 0.0, 0.0, 0.0])) + (((Lanes([0.0, v16279[0], 0.0, 0.0, 0.0])) - ((v17540 + v17759) * v12367)) / v12366)) * v12123) + (v17541 * v12368)) * v12123) + (v17541 * v12369);
            let v12372 = ((v12369 * v12123) + v1) - v4637;
            let v17773 = v17772 * v12372;
            let v12376 = ((v12372 * v12372) + v10367).sqrt();
            let v12381 = (v1 + (v12373 + (v4897 * (v12372 + v12376)))).sqrt();
            let v12383 = v4897 * (v1 + v12381);
            let v12384 = v12364 * v12383;
            let v17786 = (((((v14858 * v12342) * (rspice_limited_exp_derivative(v12355))) / v12349) + ((((((Lanes([0.0, v17747[0], v17747[1], v17747[2], 0.0])) + (v17540 * v12360)) * v12123) + (v17541 * v12361)) * v12123) + (v17541 * v12362))) * v12383) + (((((v17772 + ((v17773 + v17773) * (v14600 / (v14869 * v12376)))) * v4897) * (v14600 / (v14869 * v12381))) * v4897) * v12364);
            let v12386 = v12384 - v1;
            let v17787 = v17786 * v12386;
            let v12389 = v5721 * v12388;
            let v12392 = ((v12386 * v12386) + (v12389 * v12388)).sqrt();
            let v12394 = v4897 * ((v12384 + v1) + v12392);
            let v17793 = (v17786 + ((v17787 + v17787) * (v14600 / (v14869 * v12392)))) * v4897;
            let v12395 = v9298 * v12123;
            let v17794 = v16288 * v12123;
            let v12397 = v9239 + (v12395 * v12123);
            let v12398 = if v0 >= v12397 { v0 } else { v12397 };
            let v12400 = (v12398 * v12122) + v12365;
            let v12401 = v9225 / v12400;
            let v12402 = -v12401;
            let v12403 = rspice_limited_exp(v12402);
            let v17816 = ((((Lanes([0.0, v16252[0], 0.0, 0.0, 0.0])) - (((((((Lanes([0.0, v16260[0], 0.0, 0.0, 0.0])) + ((((Lanes([0.0, v17794[0], 0.0, 0.0, 0.0])) + (v17541 * v9298)) * v12123) + (v17541 * v12395))) * (v14600 - (if v0 >= v12397 { 1.0 } else { 0.0 }))) * v12122) + (v17540 * v12398)) + v17759) * v12401)) / v12400) * v14888) * (rspice_limited_exp_derivative(v12402));
            let v12404 = if v96 == v23 { 1.0 } else { 0.0 };
            let v12595: f64;
            let v14859: Lanes<5>;
            if v12404 != 0.0 {
                let v17817 = v16277 * v10356;
                let v17818 = v16365 * v9275;
                let v12406 = v9296 + (v9275 * v10356);
                let v17823 = (Lanes([v16287[0], 0.0, 0.0])) + ((Lanes([v17817[0], 0.0, 0.0])) + (Lanes([0.0, v17818[0], v17818[1]])));
                let v12408 = if v12406 < v12407 { 1.0 } else { 0.0 };
                let v12417: f64;
                let v14860: Lanes<3>;
                if v12408 != 0.0 {
                    let v12410 = v12409 / v12406;
                    let v17833 = ((v17823 * v12410) * v14888) / v12406;
                    v12417 = v12410;
                    v14860 = v17833;
                } else {
                    let v17824 = v17823 * v12406;
                    let v12414 = ((v12406 * v12406) + v12412).sqrt();
                    let v12416 = v4897 * (v12406 + v12414);
                    let v17830 = (v17823 + ((v17824 + v17824) * (v14600 / (v14869 * v12414)))) * v4897;
                    v12417 = v12416;
                    v14860 = v17830;
                }
                let v12418 = v9300 * v12123;
                let v17834 = v16289 * v12123;
                let v12420 = v9253 + (v12418 * v12123);
                let v12421 = if v0 >= v12420 { v0 } else { v12420 };
                let v12423 = (v12421 * v12122) + v12365;
                let v12424 = v12417 / v12423;
                let v12426 = (v10362 - v10947).sqrt();
                let v12428 = v12426 - (v10362.sqrt());
                let v12429 = -v12424;
                let v12430 = v12429 * v12428;
                let v17860 = ((v14795 * v14888) * (v14600 / (v14869 * v12426))) * v12429;
                let v12431 = rspice_limited_exp(v12430);
                let v17864 = ((((((Lanes([0.0, v14860[0], v14860[1], v14860[2], 0.0])) - (((((((Lanes([0.0, v16268[0], 0.0, 0.0, 0.0])) + ((((Lanes([0.0, v17834[0], 0.0, 0.0, 0.0])) + (v17541 * v9300)) * v12123) + (v17541 * v12418))) * (v14600 - (if v0 >= v12420 { 1.0 } else { 0.0 }))) * v12122) + (v17540 * v12421)) + v17759) * v12424)) / v12423) * v14888) * v12428) + (Lanes([v17860[0], 0.0, v17860[1], v17860[2], 0.0]))) * (rspice_limited_exp_derivative(v12430));
                v12595 = v12431;
                v14859 = v17864;
            } else {
                v12595 = v1;
                v14859 = v16802;
            }
            let v12438: f64;
            if v3728 != 0.0 {
                let v12434 = ((v23 * v11666) * v12267) / v11302;
                v12438 = v12434;
            } else {
                let v12437 = ((v23 * v11666) * v12267) / v10625;
                v12438 = v12437;
            }
            let v12439 = v12438 * v4481;
            let v12450: f64;
            if v3728 != 0.0 {
                let v12444 = (v12440 / v12439).powf(v12443);
                v12450 = v12444;
            } else {
                let v12446 = (v12123 / v12439).powf(v12443);
                v12450 = v12446;
            }
            let v12447 = v1 / v12443;
            let v12456 = v962 - v6154;
            let v12463 = ((v1 + ((v1010 + v12450).powf(v12447))) / (v1 + (v1010.powf(v12447)))) * (v4897 * ((v962 + v6154) + (((v12456 * v12456) + v12458).sqrt())));
            let v12464 = if v1298 != v0 { 1.0 } else { 0.0 };
            let v12621: f64;
            if v12464 != 0.0 {
                let v12468 = v1 + ((v12292 / v1298) / (v11062 + v12439));
                let v12469 = if v12468 > v185 { 1.0 } else { 0.0 };
                let v12472: f64;
                if v12469 != 0.0 {
                    let v12470 = v12468.ln();
                    v12472 = v12470;
                } else {
                    v12472 = v12471;
                }
                let v12474 = v1 + (v1298 * v12472);
                v12621 = v12474;
            } else {
                v12621 = v1;
            }
            let v12475 = v10581 + v12122;
            let v17865 = Lanes([0.0, v16570[0], v16570[1], v16570[2], 0.0]);
            let v12476 = v10581 / v12475;
            let v12477 = v23 - v12476;
            let v17872 = v14813 * v12477;
            let v12479 = v12122 + (v12477 * v10555);
            let v17875 = v17540 + (((((v17865 - ((v17865 + v17540) * v12476)) / v12475) * v14888) * v10555) + (Lanes([0.0, v17872[0], v17872[1], v17872[2], 0.0])));
            let v12480 = v12479 * v12123;
            let v17878 = (v17875 * v12123) + (v17541 * v12479);
            let v12598: f64;
            let v13904: f64;
            let v14371: f64;
            let v14403: f64;
            let v14413: f64;
            let v14861: Lanes<5>;
            if v4874 != 0.0 {
                let v12482 = v1 + (v10971 * v12122);
                let v12483 = v1 / v12482;
                let v17913 = (((v17540 * v10971) * v12483) * v14888) / v12482;
                let v17914 = v17913 * v12483;
                let v12486 = ((v12483 * v12483) + v4536).sqrt();
                let v12490 = v10980 + (v10981 * (v4897 * (v12483 + v12486)));
                let v17922 = v14645 * v12490;
                let v12492 = (v10986 * v12490) * v5577;
                let v12493 = v153 * v10628;
                let v17928 = (v16616 * v153) * v12479;
                let v12495 = v12211 * v12394;
                let v12496 = (v12493 * v12479) / v12495;
                let v17940 = (((((Lanes([0.0, v17928[0], v17928[1], v17928[2], 0.0])) + (v17875 * v12493)) - (((v17654 * v12394) + (v17793 * v12211)) * v12496)) / v12495) * v12492) + ((((Lanes([0.0, v17922[0], 0.0, 0.0, 0.0])) + ((((v17913 + ((v17914 + v17914) * (v14600 / (v14869 * v12486)))) * v4897) * v10981) * v10986)) * v5577) * v12496);
                let v12498 = v1 + (v12496 * v12492);
                v12598 = v12498;
                v13904 = v12492;
                v14371 = v12372;
                v14403 = v11000;
                v14413 = v10997;
                v14861 = v17940;
            } else {
                let v12599: f64;
                let v13905: f64;
                let v14372: f64;
                let v14404: f64;
                let v14414: f64;
                let v14862: Lanes<5>;
                if v4402 != 0.0 {
                    let v12502 = (v9888 * (v10317 - v12499)) - v9994;
                    let v12510 = v1 / (v1 + (v10971 * (v4897 * (v12502 + (((v12502 * v12502) + v6154).sqrt())))));
                    let v12519 = v12518 - v12499;
                    let v12533 = v10986 * (v10997 + ((v12528 + ((v12516 * (v1 + (v12517 * (((v12519 * v12519) + v25).powf((v4897 * v12522)))))) * (v4897 * (v12510 + (((v12510 * v12510) + v4536).sqrt()))))) * v5577));
                    let v12537 = (v9888 * (v10317 - v12534)) - v9994;
                    let v12546 = v1 / (v1 + (v12543 * (v4897 * (v12537 + (((v12537 * v12537) + v6154).sqrt())))));
                    let v12555 = v12554 - v12534;
                    let v12569 = v10986 * (v11000 + ((v12564 + ((v12552 * (v1 + (v12553 * (((v12555 * v12555) + v25).powf((v4897 * v12558)))))) * (v4897 * (v12546 + (((v12546 * v12546) + v4536).sqrt()))))) * v5577));
                    v12599 = v1;
                    v13905 = v0;
                    v14372 = v12537;
                    v14404 = v12569;
                    v14414 = v12533;
                    v14862 = v16802;
                } else {
                    let v12570 = if v4401 == v23 { 1.0 } else { 0.0 };
                    let v12600: f64;
                    let v13906: f64;
                    let v14863: Lanes<5>;
                    if v12570 != 0.0 {
                        let v12572 = v1 + (v10971 * v12122);
                        let v12573 = v1 / v12572;
                        let v17882 = (((v17540 * v10971) * v12573) * v14888) / v12572;
                        let v17883 = v17882 * v12573;
                        let v12576 = ((v12573 * v12573) + v4536).sqrt();
                        let v12583 = (((v10980 + (v10981 * (v4897 * (v12573 + v12576)))) * v5577) + v10997) + v11000;
                        let v12584 = v10986 * v12583;
                        let v17892 = v14645 * v12583;
                        let v12585 = v153 * v10628;
                        let v17897 = (v16616 * v153) * v12479;
                        let v12587 = v12211 * v12394;
                        let v12588 = (v12585 * v12479) / v12587;
                        let v17909 = (((((Lanes([0.0, v17897[0], v17897[1], v17897[2], 0.0])) + (v17875 * v12585)) - (((v17654 * v12394) + (v17793 * v12211)) * v12588)) / v12587) * v12584) + (((Lanes([0.0, v17892[0], 0.0, 0.0, 0.0])) + (((((v17882 + ((v17883 + v17883) * (v14600 / (v14869 * v12576)))) * v4897) * v10981) * v5577) * v10986)) * v12588);
                        let v12590 = v1 + (v12588 * v12584);
                        v12600 = v12590;
                        v13906 = v12584;
                        v14863 = v17909;
                    } else {
                        v12600 = v0;
                        v13906 = v0;
                        v14863 = v16802;
                    }
                    v12599 = v12600;
                    v13905 = v13906;
                    v14372 = v12372;
                    v14404 = v0;
                    v14414 = v0;
                    v14862 = v14863;
                }
                v12598 = v12599;
                v13904 = v13905;
                v14371 = v14372;
                v14403 = v14404;
                v14413 = v14414;
                v14861 = v14862;
            }
            let v12591 = v153 * v10628;
            let v12592 = v12591 * v12480;
            let v17942 = (v16616 * v153) * v12480;
            let v12593 = v12592 * v12329;
            let v12594 = v12593 * v12403;
            let v12597 = v12211 * v12394;
            let v12601 = v12597 * v12598;
            let v12602 = (v12594 * v12595) / v12601;
            let v12604 = v12602 * v12603;
            let v17964 = ((((((((((Lanes([0.0, v17942[0], v17942[1], v17942[2], 0.0])) + (v17878 * v12591)) * v12329) + (v17725 * v12592)) * v12403) + (v17816 * v12593)) * v12595) + (v14859 * v12594)) - (((((v17654 * v12394) + (v17793 * v12211)) * v12598) + (v14861 * v12597)) * v12602)) / v12601) * v12603;
            let v12623: f64;
            let v12640: f64;
            if v3728 != 0.0 {
                let v12609 = ((v23 * v12605) + v10555) / v12463;
                let v12613 = v12605 + ((v12440 * v12440) / (v6356 * v12609));
                v12623 = v12613;
                v12640 = v12609;
            } else {
                let v12616 = ((v23 * v12122) + v10555) / v12463;
                let v12620 = v12122 + ((v12123 * v12123) / (v6356 * v12616));
                v12623 = v12620;
                v12640 = v12616;
            }
            let v12627 = ((v1 / v12621) * v12623) + ((v12621 - v1) * v12120);
            let v12628 = if v3979 == v23 { 1.0 } else { 0.0 };
            if v12628 != 0.0 {
            } else {
            }
            let v12633 = (((v153 * v425) * v4481) * v12631) * v12627;
            let v14370: f64;
            if v97 != 0.0 {
                if v102 != 0.0 {
                } else {
                }
                let v12644 = ((v12636 - v1) * v4897) * (v12122 + ((v12123 * v12123) / (v6356 * v12640)));
                v14370 = v12644;
            } else {
                v14370 = v14371;
            }
            let v12646 = if v12645 < v0 { 1.0 } else { 0.0 };
            if v12646 != 0.0 {
            } else {
            }
            let v14368: f64;
            if v4883 != 0.0 {
                let v14369: f64;
                if v10333 != 0.0 {
                    let v12648 = v9888 * (v10334 - v10321);
                    v14369 = v12648;
                } else {
                    let v12650 = v9888 * (v10339 - v10321);
                    v14369 = v12650;
                }
                v14368 = v14369;
            } else {
                v14368 = v14370;
            }
            let v12651 = if v4882 == v0 { 1.0 } else { 0.0 };
            if v12651 != 0.0 {
                if v10333 != 0.0 {
                } else {
                }
            } else {
                let v12652 = if v4882 == v1 { 1.0 } else { 0.0 };
                if v12652 != 0.0 {
                    if v10333 != 0.0 {
                        let v12654 = if v12653 == v1 { 1.0 } else { 0.0 };
                        if v12654 != 0.0 {
                        } else {
                        }
                    } else {
                        let v12655 = if v12653 == v1 { 1.0 } else { 0.0 };
                        if v12655 != 0.0 {
                        } else {
                        }
                    }
                } else {
                    if v10333 != 0.0 {
                    } else {
                    }
                }
            }
            let v12657 = if v12656 == v1 { 1.0 } else { 0.0 };
            let v14367: f64;
            if v12657 != 0.0 {
                let v12660 = (v25 / v12365) * (v6600.sqrt());
                let v12661 = v12660 / v23;
                let v12666 = (-(v12662 - v12663)) / v6600;
                let v12671 = v10529 + (v12660 * ((v10529 * v6600).sqrt()));
                let v12672 = if (v12666 * v6600) > v12671 { 1.0 } else { 0.0 };
                let v12722: f64;
                let v12724: f64;
                if v12672 != 0.0 {
                    let v12677 = (((v12666 - v1) + (v12661 * v12661)).sqrt()) - v12661;
                    let v12679 = v1 + (v12677 * v12677);
                    let v12682 = (rspice_limited_exp((-v12679))) - v1;
                    v12722 = v12682;
                    v12724 = v12679;
                } else {
                    let v12688 = (v12666 * v4897) - (v276 * (v1 + (v12660 / v12684)));
                    let v12693 = v12688 + (((v12688 * v12688) + (v6356 * v12666)).sqrt());
                    let v12694 = if v12666 < v0 { 1.0 } else { 0.0 };
                    let v12723: f64;
                    let v12725: f64;
                    if v12694 != 0.0 {
                        let v12696 = (v12666 - v12693) / v12660;
                        let v12698 = v12696 * v12696;
                        let v12699 = (-v12693) + v12698;
                        let v12701 = (v1 - v12693) + v12698;
                        let v12702 = if v12701 > v185 { 1.0 } else { 0.0 };
                        let v12705: f64;
                        if v12702 != 0.0 {
                            let v12703 = v12701.ln();
                            v12705 = v12703;
                        } else {
                            v12705 = v12704;
                        }
                        let v12706 = -v12705;
                        v12723 = v12699;
                        v12725 = v12706;
                    } else {
                        let v12709 = rspice_limited_exp((v12707 * v12693));
                        let v12715 = ((((v12666 - v1) + v12709) + (v12661 * v12661)).sqrt()) - v12661;
                        let v12718 = (v1 - v12709) + (v12715 * v12715);
                        let v12721 = (rspice_limited_exp((-v12718))) - v1;
                        v12723 = v12721;
                        v12725 = v12718;
                    }
                    v12722 = v12723;
                    v12724 = v12725;
                }
                let v12727 = (v12722 + v12724).sqrt();
                let v12728 = if v12724 > v12050 { 1.0 } else { 0.0 };
                let v12751: f64;
                if v12728 != 0.0 {
                    let v12738 = v12724 - (((-(v12666 - v12724)) + (v12660 * v12727)) / (v1 - (((v12660 * v4897) * v12722) / v12727)));
                    v12751 = v12738;
                } else {
                    let v12740 = if v12724 < v12739 { 1.0 } else { 0.0 };
                    let v12752: f64;
                    if v12740 != 0.0 {
                        let v12750 = v12724 - (((-(v12666 - v12724)) - (v12660 * v12727)) / (v1 + (((v12660 * v4897) * v12722) / v12727)));
                        v12752 = v12750;
                    } else {
                        v12752 = v0;
                    }
                    v12751 = v12752;
                }
                let v12760 = (v12758 - v10318).abs();
                let v12766 = (((v23 * v12761) / v10625) * v10970) * v12765;
                let v12768 = v1 / v12767;
                let v12772 = v12769 * (((v12660 * (rspice_limited_exp(((-v12751) / v23)))) * v6600) + (v23 * v10549));
                let v12776 = ((v12766 * v12772) / (v12766 + v12772)) - v4637;
                let v12778 = if v12776 < v12777 { 1.0 } else { 0.0 };
                let v12787: f64;
                if v12778 != 0.0 {
                    let v12780 = v12779 / v12776;
                    v12787 = v12780;
                } else {
                    let v12786 = v4897 * (v12776 + (((v12776 * v12776) + v12782).sqrt()));
                    v12787 = v12786;
                }
                let v12801 = (-((v12796 + (if (v12760 / ((v1 + (((v12760 / (v12787 + v4637)) + v25).powf(v12767))).powf(v12768))) <= v12760 { (v12760 / ((v1 + (((v12760 / (v12787 + v4637)) + v25).powf(v12767))).powf(v12768))) } else { v12760 })) - v12798)) / v6600;
                let v12803 = if (v12801 * v6600) > v12671 { 1.0 } else { 0.0 };
                let v12845: f64;
                if v12803 != 0.0 {
                    let v12808 = (((v12801 - v1) + (v12661 * v12661)).sqrt()) - v12661;
                    let v12810 = v1 + (v12808 * v12808);
                    v12845 = v12810;
                } else {
                    let v12816 = (v12801 * v4897) - (v276 * (v1 + (v12660 / v12812)));
                    let v12821 = v12816 + (((v12816 * v12816) + (v6356 * v12801)).sqrt());
                    let v12822 = if v12801 < v0 { 1.0 } else { 0.0 };
                    let v12846: f64;
                    if v12822 != 0.0 {
                        let v12824 = (v12801 - v12821) / v12660;
                        let v12827 = (v1 - v12821) + (v12824 * v12824);
                        let v12828 = if v12827 > v185 { 1.0 } else { 0.0 };
                        let v12831: f64;
                        if v12828 != 0.0 {
                            let v12829 = v12827.ln();
                            v12831 = v12829;
                        } else {
                            v12831 = v12830;
                        }
                        let v12832 = -v12831;
                        v12846 = v12832;
                    } else {
                        let v12835 = rspice_limited_exp((v12833 * v12821));
                        let v12841 = ((((v12801 - v1) + v12835) + (v12661 * v12661)).sqrt()) - v12661;
                        let v12844 = (v1 - v12835) + (v12841 * v12841);
                        v12846 = v12844;
                    }
                    v12845 = v12846;
                }
                let v12847 = if v12845 > v12050 { 1.0 } else { 0.0 };
                if v12847 != 0.0 {
                } else {
                    let v12849 = if v12845 < v12848 { 1.0 } else { 0.0 };
                    if v12849 != 0.0 {
                    } else {
                    }
                }
                if v102 != 0.0 {
                } else {
                }
                v14367 = v12801;
            } else {
                v14367 = v14368;
            }
            let v12850 = if v4639 == v1 { 1.0 } else { 0.0 };
            let v14366: f64;
            if v12850 != 0.0 {
                let v12856 = if (if ((v9406 + (v9420 * v3969)) / v3969) <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v9218 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v12856 != 0.0 {
                } else {
                }
                v14366 = v14367;
            } else {
                let v14373: f64;
                if v4640 != 0.0 {
                    let v12860 = if ((v9435 + (v9450 * v3969)) / v3969) <= v0 { 1.0 } else { 0.0 };
                    let v14374: f64;
                    if v12860 != 0.0 {
                        v14374 = v14367;
                    } else {
                        let v12861 = v2370 * v10726;
                        let v12865 = if v12861 < (v12863 * v12862) { 1.0 } else { 0.0 };
                        let v12876: f64;
                        if v12865 != 0.0 {
                            let v12868 = ((-v12862) * v12862) / v12861;
                            v12876 = v12868;
                        } else {
                            let v12875 = v4897 * (v12861 + (((v12861 * v12861) + ((v22 * v12862) * v12862)).sqrt()));
                            v12876 = v12875;
                        }
                        let v12880 = v10726 * ((v1 / (v1 + v12876)) + v2386);
                        let v12884 = if v12880 < (v12882 * v12881) { 1.0 } else { 0.0 };
                        let v12895: f64;
                        if v12884 != 0.0 {
                            let v12887 = ((-v12881) * v12881) / v12880;
                            v12895 = v12887;
                        } else {
                            let v12894 = v4897 * (v12880 + (((v12880 * v12880) + ((v22 * v12881) * v12881)).sqrt()));
                            v12895 = v12894;
                        }
                        v14374 = v12895;
                    }
                    v14373 = v14374;
                } else {
                    v14373 = v14367;
                }
                v14366 = v14373;
            }
            let v13721: f64;
            let v13723: f64;
            let v14365: f64;
            if v4619 != 0.0 {
                let v12900 = ((v12122 - v1602) / v12897) / v6600;
                let v12901 = if v12900 > v4933 { 1.0 } else { 0.0 };
                let v12909: f64;
                if v12901 != 0.0 {
                    v12909 = v12900;
                } else {
                    let v12903 = if v12900 < v12902 { 1.0 } else { 0.0 };
                    let v12910: f64;
                    if v12903 != 0.0 {
                        let v12904 = v12900.exp();
                        v12910 = v12904;
                    } else {
                        let v12907 = (v1 + (v12900.exp())).ln();
                        v12910 = v12907;
                    }
                    v12909 = v12910;
                }
                let v12913 = -v12912;
                let v12917 = ((v9313 - (v1570 * v12122)) - v12913) - v25;
                let v12934 = v423 * v3969;
                let v12942 = (v12940 * (((((v12934 * v12933) * v5667) * v10332) * ((v12897 * v6600) * v12909)) * (rspice_limited_exp((((v12928 * v5659) * (v12913 + (v4897 * (v12917 + (((v12917 * v12917) - ((v22 * v12913) * v25)).sqrt()))))) * (v1 + (v1586 * v12122))))))) * v9452;
                let v12945 = (v10041 - (v6760 / v23)) - v10529;
                let v12946 = v12945 - v10332;
                let v12950 = (v12946 / v12947) / v6600;
                let v12951 = if v12950 > v4933 { 1.0 } else { 0.0 };
                let v12959: f64;
                if v12951 != 0.0 {
                    v12959 = v12950;
                } else {
                    let v12953 = if v12950 < v12952 { 1.0 } else { 0.0 };
                    let v12960: f64;
                    if v12953 != 0.0 {
                        let v12954 = v12950.exp();
                        v12960 = v12954;
                    } else {
                        let v12957 = (v1 + (v12950.exp())).ln();
                        v12960 = v12957;
                    }
                    v12959 = v12960;
                }
                let v12961 = (v12947 * v6600) * v12959;
                let v12980: f64;
                if v97 != 0.0 {
                    v12980 = v12149;
                } else {
                    let v12962 = if v12945 <= v0 { 1.0 } else { 0.0 };
                    let v12981: f64;
                    if v12962 != 0.0 {
                        let v12963 = v12946 - v4656;
                        let v12970 = v4897 * (v12963 + (((v12963 * v12963) - (v12965 * v12945)).sqrt()));
                        v12981 = v12970;
                    } else {
                        let v12971 = v12946 - v4656;
                        let v12977 = v4897 * (v12971 + (((v12971 * v12971) + (v12965 * v12945)).sqrt()));
                        v12981 = v12977;
                    }
                    v12980 = v12981;
                }
                let v12979 = -v12978;
                let v12985 = ((v9326 - (v1666 * v12980)) - v12979) - v25;
                let v12993 = v12979 + (v4897 * (v12985 + (((v12985 * v12985) - ((v22 * v12979) * v25)).sqrt())));
                let v13007 = (v12940 * (((((v12934 * v5655) * v5667) * v10332) * v12961) * (rspice_limited_exp((((v12996 * v5659) * v12993) * (v1 + (v1682 * v12980))))))) * v9452;
                v13721 = v12942;
                v13723 = v13007;
                v14365 = v12993;
            } else {
                v13721 = v0;
                v13723 = v0;
                v14365 = v14366;
            }
            let v13709: f64;
            let v13711: f64;
            let v13713: f64;
            let v13717: f64;
            let v14364: f64;
            if v4623 != 0.0 {
                let v13009 = -v13008;
                let v13013 = ((v9339 - (v1746 * v12122)) - v13009) - v25;
                let v13026 = (-v13024) * v5659;
                let v13043 = ((((((v13036 * v423) * v3969) * v5671) * v5667) * (v12122 * (rspice_limited_exp(((v13026 * (v13009 + (v4897 * (v13013 + (((v13013 * v13013) - ((v22 * v13009) * v25)).sqrt()))))) * (v1 + (v1762 * v12122))))))) * ((v10332 + (v4897 * v10356)) + (v4897 * (v10328 + v10330)))) * v9452;
                let v13049 = v13047 * (((v12124 + v4536).sqrt()) - v6154);
                let v13051 = rspice_limited_exp((-v13049));
                let v13061 = (v13049 * v13049) + v13060;
                let v13063 = (v13043 * ((v1 - ((v13049 + v1) * v13051)) + v4767)) / v13061;
                let v13065 = (v13043 * (((v13049 + v13051) - v1) + v4767)) / v13061;
                let v13066 = v10320 - v9994;
                let v13069 = ((v13066 * v13066) + v4767).sqrt();
                let v13071 = if v13070 == v1 { 1.0 } else { 0.0 };
                let v13088: f64;
                let v13093: f64;
                if v13071 != 0.0 {
                    let v13073 = v9352 - (v1826 * v13069);
                    let v13075 = if v13073 < v13074 { 1.0 } else { 0.0 };
                    let v13084: f64;
                    if v13075 != 0.0 {
                        let v13077 = v13076 / v13073;
                        v13084 = v13077;
                    } else {
                        let v13083 = v4897 * (v13073 + (((v13073 * v13073) + v13079).sqrt()));
                        v13084 = v13083;
                    }
                    let v13085 = if v1842 < v4536 { 1.0 } else { 0.0 };
                    let v13089: f64;
                    if v13085 != 0.0 {
                        v13089 = v4536;
                    } else {
                        v13089 = v1842;
                    }
                    v13088 = v13089;
                    v13093 = v13084;
                } else {
                    let v13087 = v9352 - (v1826 * v13069);
                    v13088 = v1842;
                    v13093 = v13087;
                }
                let v13092 = v13026 * v5661;
                let v13096 = rspice_limited_exp(((v13092 * v13093) * (v1 + (v13088 * v13069))));
                let v13097 = if v12645 > v0 { 1.0 } else { 0.0 };
                let v13715: f64;
                let v13719: f64;
                if v13097 != 0.0 {
                    let v13102 = (((v9453 * v13098) * v10320) * v13069) * v13096;
                    v13715 = v13102;
                    v13719 = v0;
                } else {
                    let v13106 = (((v9453 * v13098) * v10320) * v13069) * v13096;
                    v13715 = v0;
                    v13719 = v13106;
                }
                let v13107 = v10325 - v9994;
                let v13110 = ((v13107 * v13107) + v4767).sqrt();
                let v13127: f64;
                let v13131: f64;
                if v13071 != 0.0 {
                    let v13112 = v9365 - (v1890 * v13110);
                    let v13114 = if v13112 < v13113 { 1.0 } else { 0.0 };
                    let v13123: f64;
                    if v13114 != 0.0 {
                        let v13116 = v13115 / v13112;
                        v13123 = v13116;
                    } else {
                        let v13122 = v4897 * (v13112 + (((v13112 * v13112) + v13118).sqrt()));
                        v13123 = v13122;
                    }
                    let v13124 = if v1906 < v4536 { 1.0 } else { 0.0 };
                    let v13128: f64;
                    if v13124 != 0.0 {
                        v13128 = v4536;
                    } else {
                        v13128 = v1906;
                    }
                    v13127 = v13128;
                    v13131 = v13123;
                } else {
                    let v13126 = v9365 - (v1890 * v13110);
                    v13127 = v1906;
                    v13131 = v13126;
                }
                let v13130 = v1 + (v13127 * v13110);
                let v13134 = rspice_limited_exp(((v13092 * v13131) * v13130));
                let v13714: f64;
                let v13718: f64;
                if v13097 != 0.0 {
                    let v13139 = (((v9453 * v13135) * v10325) * v13110) * v13134;
                    v13714 = v13715;
                    v13718 = v13139;
                } else {
                    let v13143 = (((v9453 * v13135) * v10325) * v13110) * v13134;
                    v13714 = v13143;
                    v13718 = v13719;
                }
                v13709 = v13063;
                v13711 = v13065;
                v13713 = v13714;
                v13717 = v13718;
                v14364 = v13130;
            } else {
                v13709 = v0;
                v13711 = v0;
                v13713 = v0;
                v13717 = v0;
                v14364 = v14365;
            }
            let v13144 = if v3204 != v0 { 1.0 } else { 0.0 };
            let v14353: f64;
            if v13144 != 0.0 {
                let v13147 = if (if v1954 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v9380 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14363: f64;
                if v13147 != 0.0 {
                    v14363 = v14364;
                } else {
                    let v13151 = (((-v10325) - v2002) + v9994) / v5571;
                    let v13153 = if v13151 < v13152 { 1.0 } else { 0.0 };
                    let v13162: f64;
                    if v13153 != 0.0 {
                        let v13155 = v13154 / v13151;
                        v13162 = v13155;
                    } else {
                        let v13161 = v4897 * (v13151 + (((v13151 * v13151) + v13157).sqrt()));
                        v13162 = v13161;
                    }
                    let v13164 = v9380 / (v13162 + v4637);
                    if v97 != 0.0 {
                        let v13167 = ((-v10330) * v10330) * v10330;
                        let v13174 = if (v13167 / ((v13168 + (v13167.abs())) + v6553)) < v13173 { 1.0 } else { 0.0 };
                        if v13174 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                    v14363 = v13164;
                }
                let v13175 = if v3204 == v276 { 1.0 } else { 0.0 };
                let v13177 = if v13175 != 0.0 && (if v2018 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14361: f64;
                if v13177 != 0.0 {
                    let v14362: f64;
                    if v97 != 0.0 {
                        let v13180 = (v1 + (v2962 * v6598)) - v25;
                        let v13182 = if v13180 < v13181 { 1.0 } else { 0.0 };
                        let v13191: f64;
                        if v13182 != 0.0 {
                            let v13184 = v13183 / v13180;
                            v13191 = v13184;
                        } else {
                            let v13190 = v4897 * (v13180 + (((v13180 * v13180) + v13186).sqrt()));
                            v13191 = v13190;
                        }
                        let v13203 = ((v2018 * v423) * v6777) * (rspice_limited_exp(((((((v2034 * v10325) * v10325) - ((v2050 * v13191) * v10325)) - v2066) + v9994) / v6600)));
                        let v13206 = ((-v10330) * v10330) * v10330;
                        let v13212 = if (v13206 / ((v13168 + (v13206.abs())) + v6553)) < v13211 { 1.0 } else { 0.0 };
                        if v13212 != 0.0 {
                        } else {
                        }
                        v14362 = v13203;
                    } else {
                        let v13215 = (v1 + (v2962 * v6598)) - v25;
                        let v13217 = if v13215 < v13216 { 1.0 } else { 0.0 };
                        let v13226: f64;
                        if v13217 != 0.0 {
                            let v13219 = v13218 / v13215;
                            v13226 = v13219;
                        } else {
                            let v13225 = v4897 * (v13215 + (((v13215 * v13215) + v13221).sqrt()));
                            v13226 = v13225;
                        }
                        let v13238 = ((v2018 * v423) * v6777) * (rspice_limited_exp(((((((v2034 * v10325) * v10325) - ((v2050 * v13226) * v10325)) - v2066) + v9994) / v6600)));
                        v14362 = v13238;
                    }
                    v14361 = v14362;
                } else {
                    v14361 = v14363;
                }
                let v13246 = if (if v97 != 0.0 && (if (if v3204 == v23 { 1.0 } else { 0.0 }) != 0.0 || v13175 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if v100 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v100 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v102 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14359: f64;
                if v13246 != 0.0 {
                    let v13248 = if v9368 < v13247 { 1.0 } else { 0.0 };
                    let v13259: f64;
                    if v13248 != 0.0 {
                        let v13250 = v13249 / v9368;
                        v13259 = v13250;
                    } else {
                        let v13256 = v4897 * (v9368 + (((v9368 * v9368) + v13252).sqrt()));
                        v13259 = v13256;
                    }
                    let v13260 = v13257 * v13259;
                    let v13265 = if (if v13261 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v13260 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v14360: f64;
                    if v13265 != 0.0 {
                        v14360 = v14361;
                    } else {
                        let v13271 = (((-v10325) - v13267) + v9994) / v5571;
                        let v13273 = if v13271 < v13272 { 1.0 } else { 0.0 };
                        let v13282: f64;
                        if v13273 != 0.0 {
                            let v13275 = v13274 / v13271;
                            v13282 = v13275;
                        } else {
                            let v13281 = v4897 * (v13271 + (((v13271 * v13271) + v13277).sqrt()));
                            v13282 = v13281;
                        }
                        let v13284 = v13260 / (v13282 + v4637);
                        let v13287 = ((-v10330) * v10330) * v10330;
                        let v13295 = if (v13287 / ((v13288 + (v13287.abs())) + v6553)) < v13294 { 1.0 } else { 0.0 };
                        if v13295 != 0.0 {
                        } else {
                        }
                        v14360 = v13284;
                    }
                    v14359 = v14360;
                } else {
                    v14359 = v14361;
                }
                let v13296 = if v12645 > v0 { 1.0 } else { 0.0 };
                if v13296 != 0.0 {
                } else {
                }
                let v13299 = if (if v2082 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v9392 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14358: f64;
                if v13299 != 0.0 {
                    v14358 = v14359;
                } else {
                    let v13303 = (((-v10320) - v2130) + v9994) / v5571;
                    let v13305 = if v13303 < v13304 { 1.0 } else { 0.0 };
                    let v13314: f64;
                    if v13305 != 0.0 {
                        let v13307 = v13306 / v13303;
                        v13314 = v13307;
                    } else {
                        let v13313 = v4897 * (v13303 + (((v13303 * v13303) + v13309).sqrt()));
                        v13314 = v13313;
                    }
                    let v13316 = v9392 / (v13314 + v4637);
                    if v97 != 0.0 {
                        let v13319 = ((-v10328) * v10328) * v10328;
                        let v13326 = if (v13319 / ((v13320 + (v13319.abs())) + v6553)) < v13325 { 1.0 } else { 0.0 };
                        if v13326 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                    v14358 = v13316;
                }
                let v13328 = if v13175 != 0.0 && (if v2146 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14356: f64;
                if v13328 != 0.0 {
                    let v14357: f64;
                    if v97 != 0.0 {
                        let v13331 = (v1 + (v2962 * v6598)) - v25;
                        let v13333 = if v13331 < v13332 { 1.0 } else { 0.0 };
                        let v13342: f64;
                        if v13333 != 0.0 {
                            let v13335 = v13334 / v13331;
                            v13342 = v13335;
                        } else {
                            let v13341 = v4897 * (v13331 + (((v13331 * v13331) + v13337).sqrt()));
                            v13342 = v13341;
                        }
                        let v13354 = ((v2146 * v423) * v6777) * (rspice_limited_exp(((((((v2162 * v10320) * v10320) - ((v2178 * v13342) * v10320)) - v2194) + v9994) / v6600)));
                        let v13357 = ((-v10328) * v10328) * v10328;
                        let v13363 = if (v13357 / ((v13320 + (v13357.abs())) + v6553)) < v13362 { 1.0 } else { 0.0 };
                        if v13363 != 0.0 {
                        } else {
                        }
                        v14357 = v13354;
                    } else {
                        let v13366 = (v1 + (v2962 * v6598)) - v25;
                        let v13368 = if v13366 < v13367 { 1.0 } else { 0.0 };
                        let v13377: f64;
                        if v13368 != 0.0 {
                            let v13370 = v13369 / v13366;
                            v13377 = v13370;
                        } else {
                            let v13376 = v4897 * (v13366 + (((v13366 * v13366) + v13372).sqrt()));
                            v13377 = v13376;
                        }
                        let v13389 = ((v2146 * v423) * v6777) * (rspice_limited_exp(((((((v2162 * v10320) * v10320) - ((v2178 * v13377) * v10320)) - v2194) + v9994) / v6600)));
                        v14357 = v13389;
                    }
                    v14356 = v14357;
                } else {
                    v14356 = v14358;
                }
                let v14354: f64;
                if v13246 != 0.0 {
                    let v13391 = if v9368 < v13390 { 1.0 } else { 0.0 };
                    let v13402: f64;
                    if v13391 != 0.0 {
                        let v13393 = v13392 / v9368;
                        v13402 = v13393;
                    } else {
                        let v13399 = v4897 * (v9368 + (((v9368 * v9368) + v13395).sqrt()));
                        v13402 = v13399;
                    }
                    let v13403 = v13400 * v13402;
                    let v13408 = if (if v13404 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v13403 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v14355: f64;
                    if v13408 != 0.0 {
                        v14355 = v14356;
                    } else {
                        let v13414 = (((-v10320) - v13410) + v9994) / v5571;
                        let v13416 = if v13414 < v13415 { 1.0 } else { 0.0 };
                        let v13425: f64;
                        if v13416 != 0.0 {
                            let v13418 = v13417 / v13414;
                            v13425 = v13418;
                        } else {
                            let v13424 = v4897 * (v13414 + (((v13414 * v13414) + v13420).sqrt()));
                            v13425 = v13424;
                        }
                        let v13427 = v13403 / (v13425 + v4637);
                        let v13430 = ((-v10328) * v10328) * v10328;
                        let v13438 = if (v13430 / ((v13431 + (v13430.abs())) + v6553)) < v13437 { 1.0 } else { 0.0 };
                        if v13438 != 0.0 {
                        } else {
                        }
                        v14355 = v13427;
                    }
                    v14354 = v14355;
                } else {
                    v14354 = v14356;
                }
                if v13296 != 0.0 {
                } else {
                }
                v14353 = v14354;
            } else {
                v14353 = v14364;
            }
            let v14346: f64;
            if v97 != 0.0 {
                let v13440 = if v13439 > v0 { 1.0 } else { 0.0 };
                let v14350: f64;
                if v13440 != 0.0 {
                    let v13443 = if v10328 < v13441 { 1.0 } else { 0.0 };
                    let v14351: f64;
                    if v13443 != 0.0 {
                        let v13452 = v13446 + (v13448 * (v10328 - v13441));
                        v14351 = v13452;
                    } else {
                        let v13455 = if v10328 <= v13453 { 1.0 } else { 0.0 };
                        let v14352: f64;
                        if v13455 != 0.0 {
                            let v13459 = rspice_limited_exp((-((v10060 + v10328) / v13444)));
                            v14352 = v13459;
                        } else {
                            v14352 = v14353;
                        }
                        v14351 = v14352;
                    }
                    v14350 = v14351;
                } else {
                    v14350 = v14353;
                }
                let v13461 = if v13460 > v0 { 1.0 } else { 0.0 };
                if v13461 != 0.0 {
                    let v13465 = if (v13462 - v10328) < (v13462 * v4637) { 1.0 } else { 0.0 };
                    if v13465 != 0.0 {
                    } else {
                    }
                } else {
                }
                let v13467 = if v13466 > v0 { 1.0 } else { 0.0 };
                if v13467 != 0.0 {
                    let v13471 = if (v13468 - v10328) < (v13468 * v4637) { 1.0 } else { 0.0 };
                    if v13471 != 0.0 {
                    } else {
                    }
                } else {
                }
                let v13473 = if v13472 > v0 { 1.0 } else { 0.0 };
                if v13473 != 0.0 {
                    let v13477 = if (v13474 - v10328) < (v13474 * v4637) { 1.0 } else { 0.0 };
                    if v13477 != 0.0 {
                    } else {
                    }
                } else {
                }
                let v13479 = if v13478 > v0 { 1.0 } else { 0.0 };
                let v14347: f64;
                if v13479 != 0.0 {
                    let v13482 = if v10330 < v13480 { 1.0 } else { 0.0 };
                    let v14348: f64;
                    if v13482 != 0.0 {
                        let v13491 = v13485 + (v13487 * (v10330 - v13480));
                        v14348 = v13491;
                    } else {
                        let v13494 = if v10330 <= v13492 { 1.0 } else { 0.0 };
                        let v14349: f64;
                        if v13494 != 0.0 {
                            let v13498 = rspice_limited_exp((-((v10128 + v10330) / v13483)));
                            v14349 = v13498;
                        } else {
                            v14349 = v14350;
                        }
                        v14348 = v14349;
                    }
                    v14347 = v14348;
                } else {
                    v14347 = v14350;
                }
                let v13500 = if v13499 > v0 { 1.0 } else { 0.0 };
                if v13500 != 0.0 {
                    let v13504 = if (v13501 - v10330) < (v13501 * v4637) { 1.0 } else { 0.0 };
                    if v13504 != 0.0 {
                    } else {
                    }
                } else {
                }
                let v13506 = if v13505 > v0 { 1.0 } else { 0.0 };
                if v13506 != 0.0 {
                    let v13510 = if (v13507 - v10330) < (v13507 * v4637) { 1.0 } else { 0.0 };
                    if v13510 != 0.0 {
                    } else {
                    }
                } else {
                }
                let v13512 = if v13511 > v0 { 1.0 } else { 0.0 };
                if v13512 != 0.0 {
                    let v13516 = if (v13513 - v10330) < (v13513 * v4637) { 1.0 } else { 0.0 };
                    if v13516 != 0.0 {
                    } else {
                    }
                } else {
                }
                let v13518 = if v13517 > v0 { 1.0 } else { 0.0 };
                if v13518 != 0.0 {
                    let v13519 = v10328 / v10205;
                    let v13520 = if v13519 < v6857 { 1.0 } else { 0.0 };
                    if v13520 != 0.0 {
                        let v13521 = if v10203 > v0 { 1.0 } else { 0.0 };
                        if v13521 != 0.0 {
                            let v13524 = if v10328 > v13522 { 1.0 } else { 0.0 };
                            if v13524 != 0.0 {
                                let v13525 = v1 - v13519;
                                let v13526 = if v10207 != v1 { 1.0 } else { 0.0 };
                                if v13526 != 0.0 {
                                    let v13527 = if v10207 == v4897 { 1.0 } else { 0.0 };
                                    if v13527 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13528 = if v13525 > v185 { 1.0 } else { 0.0 };
                                    if v13528 != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let v13530 = v1 - (v13522 / v10205);
                                let v13531 = if v10207 != v1 { 1.0 } else { 0.0 };
                                if v13531 != 0.0 {
                                    let v13532 = if v10207 == v4897 { 1.0 } else { 0.0 };
                                    if v13532 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13533 = if v13530 > v185 { 1.0 } else { 0.0 };
                                    if v13533 != 0.0 {
                                    } else {
                                    }
                                }
                                let v13538 = v1 - ((v10328 - v13522) / v13535);
                                let v13539 = if v10213 != v1 { 1.0 } else { 0.0 };
                                if v13539 != 0.0 {
                                    let v13540 = if v10213 == v4897 { 1.0 } else { 0.0 };
                                    if v13540 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13541 = if v13538 > v185 { 1.0 } else { 0.0 };
                                    if v13541 != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let v13542 = v1 - v13519;
                            let v13543 = if v10207 != v1 { 1.0 } else { 0.0 };
                            if v13543 != 0.0 {
                                let v13544 = if v10207 == v4897 { 1.0 } else { 0.0 };
                                if v13544 != 0.0 {
                                } else {
                                }
                            } else {
                                let v13545 = if v13542 > v185 { 1.0 } else { 0.0 };
                                if v13545 != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let v13546 = if v10207 != v1 { 1.0 } else { 0.0 };
                        if v13546 != 0.0 {
                            let v13547 = if v10207 == v4897 { 1.0 } else { 0.0 };
                            if v13547 != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                let v13549 = if v13548 > v0 { 1.0 } else { 0.0 };
                if v13549 != 0.0 {
                    let v13550 = v10328 / v10224;
                    let v13551 = if v13550 < v6857 { 1.0 } else { 0.0 };
                    if v13551 != 0.0 {
                        let v13552 = if v10222 > v0 { 1.0 } else { 0.0 };
                        if v13552 != 0.0 {
                            let v13555 = if v10328 > v13553 { 1.0 } else { 0.0 };
                            if v13555 != 0.0 {
                                let v13556 = v1 - v13550;
                                let v13557 = if v10226 != v1 { 1.0 } else { 0.0 };
                                if v13557 != 0.0 {
                                    let v13558 = if v10226 == v4897 { 1.0 } else { 0.0 };
                                    if v13558 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13559 = if v13556 > v185 { 1.0 } else { 0.0 };
                                    if v13559 != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let v13561 = v1 - (v13553 / v10224);
                                let v13562 = if v10226 != v1 { 1.0 } else { 0.0 };
                                if v13562 != 0.0 {
                                    let v13563 = if v10226 == v4897 { 1.0 } else { 0.0 };
                                    if v13563 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13564 = if v13561 > v185 { 1.0 } else { 0.0 };
                                    if v13564 != 0.0 {
                                    } else {
                                    }
                                }
                                let v13569 = v1 - ((v10328 - v13553) / v13566);
                                let v13570 = if v10232 != v1 { 1.0 } else { 0.0 };
                                if v13570 != 0.0 {
                                    let v13571 = if v10232 == v4897 { 1.0 } else { 0.0 };
                                    if v13571 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13572 = if v13569 > v185 { 1.0 } else { 0.0 };
                                    if v13572 != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let v13573 = v1 - v13550;
                            let v13574 = if v10226 != v1 { 1.0 } else { 0.0 };
                            if v13574 != 0.0 {
                                let v13575 = if v10226 == v4897 { 1.0 } else { 0.0 };
                                if v13575 != 0.0 {
                                } else {
                                }
                            } else {
                                let v13576 = if v13573 > v185 { 1.0 } else { 0.0 };
                                if v13576 != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let v13577 = if v10226 != v1 { 1.0 } else { 0.0 };
                        if v13577 != 0.0 {
                            let v13578 = if v10226 == v4897 { 1.0 } else { 0.0 };
                            if v13578 != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                let v13580 = if v13579 > v0 { 1.0 } else { 0.0 };
                if v13580 != 0.0 {
                    let v13581 = v10328 / v10243;
                    let v13582 = if v13581 < v6857 { 1.0 } else { 0.0 };
                    if v13582 != 0.0 {
                        let v13583 = if v10241 > v0 { 1.0 } else { 0.0 };
                        if v13583 != 0.0 {
                            let v13586 = if v10328 > v13584 { 1.0 } else { 0.0 };
                            if v13586 != 0.0 {
                                let v13587 = v1 - v13581;
                                let v13588 = if v10245 != v1 { 1.0 } else { 0.0 };
                                if v13588 != 0.0 {
                                    let v13589 = if v10245 == v4897 { 1.0 } else { 0.0 };
                                    if v13589 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13590 = if v13587 > v185 { 1.0 } else { 0.0 };
                                    if v13590 != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let v13592 = v1 - (v13584 / v10243);
                                let v13593 = if v10245 != v1 { 1.0 } else { 0.0 };
                                if v13593 != 0.0 {
                                    let v13594 = if v10245 == v4897 { 1.0 } else { 0.0 };
                                    if v13594 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13595 = if v13592 > v185 { 1.0 } else { 0.0 };
                                    if v13595 != 0.0 {
                                    } else {
                                    }
                                }
                                let v13600 = v1 - ((v10328 - v13584) / v13597);
                                let v13601 = if v10251 != v1 { 1.0 } else { 0.0 };
                                if v13601 != 0.0 {
                                    let v13602 = if v10251 == v4897 { 1.0 } else { 0.0 };
                                    if v13602 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13603 = if v13600 > v185 { 1.0 } else { 0.0 };
                                    if v13603 != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let v13604 = v1 - v13581;
                            let v13605 = if v10245 != v1 { 1.0 } else { 0.0 };
                            if v13605 != 0.0 {
                                let v13606 = if v10245 == v4897 { 1.0 } else { 0.0 };
                                if v13606 != 0.0 {
                                } else {
                                }
                            } else {
                                let v13607 = if v13604 > v185 { 1.0 } else { 0.0 };
                                if v13607 != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let v13608 = if v10245 != v1 { 1.0 } else { 0.0 };
                        if v13608 != 0.0 {
                            let v13609 = if v10245 == v4897 { 1.0 } else { 0.0 };
                            if v13609 != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                let v13611 = if v13610 > v0 { 1.0 } else { 0.0 };
                if v13611 != 0.0 {
                    let v13612 = v10330 / v10262;
                    let v13613 = if v13612 < v6857 { 1.0 } else { 0.0 };
                    if v13613 != 0.0 {
                        let v13614 = if v10260 > v0 { 1.0 } else { 0.0 };
                        if v13614 != 0.0 {
                            let v13617 = if v10330 > v13615 { 1.0 } else { 0.0 };
                            if v13617 != 0.0 {
                                let v13618 = v1 - v13612;
                                let v13619 = if v10264 != v1 { 1.0 } else { 0.0 };
                                if v13619 != 0.0 {
                                    let v13620 = if v10264 == v4897 { 1.0 } else { 0.0 };
                                    if v13620 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13621 = if v13618 > v185 { 1.0 } else { 0.0 };
                                    if v13621 != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let v13623 = v1 - (v13615 / v10262);
                                let v13624 = if v10264 != v1 { 1.0 } else { 0.0 };
                                if v13624 != 0.0 {
                                    let v13625 = if v10264 == v4897 { 1.0 } else { 0.0 };
                                    if v13625 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13626 = if v13623 > v185 { 1.0 } else { 0.0 };
                                    if v13626 != 0.0 {
                                    } else {
                                    }
                                }
                                let v13631 = v1 - ((v10330 - v13615) / v13628);
                                let v13632 = if v10270 != v1 { 1.0 } else { 0.0 };
                                if v13632 != 0.0 {
                                    let v13633 = if v10270 == v4897 { 1.0 } else { 0.0 };
                                    if v13633 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13634 = if v13631 > v185 { 1.0 } else { 0.0 };
                                    if v13634 != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let v13635 = v1 - v13612;
                            let v13636 = if v10264 != v1 { 1.0 } else { 0.0 };
                            if v13636 != 0.0 {
                                let v13637 = if v10264 == v4897 { 1.0 } else { 0.0 };
                                if v13637 != 0.0 {
                                } else {
                                }
                            } else {
                                let v13638 = if v13635 > v185 { 1.0 } else { 0.0 };
                                if v13638 != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let v13639 = if v10264 != v1 { 1.0 } else { 0.0 };
                        if v13639 != 0.0 {
                            let v13640 = if v10264 == v4897 { 1.0 } else { 0.0 };
                            if v13640 != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                let v13642 = if v13641 > v0 { 1.0 } else { 0.0 };
                if v13642 != 0.0 {
                    let v13643 = v10330 / v10281;
                    let v13644 = if v13643 < v6857 { 1.0 } else { 0.0 };
                    if v13644 != 0.0 {
                        let v13645 = if v10279 > v0 { 1.0 } else { 0.0 };
                        if v13645 != 0.0 {
                            let v13648 = if v10330 > v13646 { 1.0 } else { 0.0 };
                            if v13648 != 0.0 {
                                let v13649 = v1 - v13643;
                                let v13650 = if v10283 != v1 { 1.0 } else { 0.0 };
                                if v13650 != 0.0 {
                                    let v13651 = if v10283 == v4897 { 1.0 } else { 0.0 };
                                    if v13651 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13652 = if v13649 > v185 { 1.0 } else { 0.0 };
                                    if v13652 != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let v13654 = v1 - (v13646 / v10281);
                                let v13655 = if v10283 != v1 { 1.0 } else { 0.0 };
                                if v13655 != 0.0 {
                                    let v13656 = if v10283 == v4897 { 1.0 } else { 0.0 };
                                    if v13656 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13657 = if v13654 > v185 { 1.0 } else { 0.0 };
                                    if v13657 != 0.0 {
                                    } else {
                                    }
                                }
                                let v13662 = v1 - ((v10330 - v13646) / v13659);
                                let v13663 = if v10289 != v1 { 1.0 } else { 0.0 };
                                if v13663 != 0.0 {
                                    let v13664 = if v10289 == v4897 { 1.0 } else { 0.0 };
                                    if v13664 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13665 = if v13662 > v185 { 1.0 } else { 0.0 };
                                    if v13665 != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let v13666 = v1 - v13643;
                            let v13667 = if v10283 != v1 { 1.0 } else { 0.0 };
                            if v13667 != 0.0 {
                                let v13668 = if v10283 == v4897 { 1.0 } else { 0.0 };
                                if v13668 != 0.0 {
                                } else {
                                }
                            } else {
                                let v13669 = if v13666 > v185 { 1.0 } else { 0.0 };
                                if v13669 != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let v13670 = if v10283 != v1 { 1.0 } else { 0.0 };
                        if v13670 != 0.0 {
                            let v13671 = if v10283 == v4897 { 1.0 } else { 0.0 };
                            if v13671 != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                let v13673 = if v13672 > v0 { 1.0 } else { 0.0 };
                if v13673 != 0.0 {
                    let v13674 = v10330 / v10300;
                    let v13675 = if v13674 < v6857 { 1.0 } else { 0.0 };
                    if v13675 != 0.0 {
                        let v13676 = if v10298 > v0 { 1.0 } else { 0.0 };
                        if v13676 != 0.0 {
                            let v13679 = if v10330 > v13677 { 1.0 } else { 0.0 };
                            if v13679 != 0.0 {
                                let v13680 = v1 - v13674;
                                let v13681 = if v10302 != v1 { 1.0 } else { 0.0 };
                                if v13681 != 0.0 {
                                    let v13682 = if v10302 == v4897 { 1.0 } else { 0.0 };
                                    if v13682 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13683 = if v13680 > v185 { 1.0 } else { 0.0 };
                                    if v13683 != 0.0 {
                                    } else {
                                    }
                                }
                            } else {
                                let v13685 = v1 - (v13677 / v10300);
                                let v13686 = if v10302 != v1 { 1.0 } else { 0.0 };
                                if v13686 != 0.0 {
                                    let v13687 = if v10302 == v4897 { 1.0 } else { 0.0 };
                                    if v13687 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13688 = if v13685 > v185 { 1.0 } else { 0.0 };
                                    if v13688 != 0.0 {
                                    } else {
                                    }
                                }
                                let v13693 = v1 - ((v10330 - v13677) / v13690);
                                let v13694 = if v10308 != v1 { 1.0 } else { 0.0 };
                                if v13694 != 0.0 {
                                    let v13695 = if v10308 == v4897 { 1.0 } else { 0.0 };
                                    if v13695 != 0.0 {
                                    } else {
                                    }
                                } else {
                                    let v13696 = if v13693 > v185 { 1.0 } else { 0.0 };
                                    if v13696 != 0.0 {
                                    } else {
                                    }
                                }
                            }
                        } else {
                            let v13697 = v1 - v13674;
                            let v13698 = if v10302 != v1 { 1.0 } else { 0.0 };
                            if v13698 != 0.0 {
                                let v13699 = if v10302 == v4897 { 1.0 } else { 0.0 };
                                if v13699 != 0.0 {
                                } else {
                                }
                            } else {
                                let v13700 = if v13697 > v185 { 1.0 } else { 0.0 };
                                if v13700 != 0.0 {
                                } else {
                                }
                            }
                        }
                    } else {
                        let v13701 = if v10302 != v1 { 1.0 } else { 0.0 };
                        if v13701 != 0.0 {
                            let v13702 = if v10302 == v4897 { 1.0 } else { 0.0 };
                            if v13702 != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                } else {
                }
                v14346 = v14347;
            } else {
                v14346 = v14353;
            }
            let v14345: f64;
            if v97 != 0.0 {
                let v13704 = v9888 * (v10334 - v10326);
                v14345 = v13704;
            } else {
                v14345 = v14346;
            }
            let v13707 = if v13705 != v0 { 1.0 } else { 0.0 };
            let v13708 = if v3980 != 0.0 && v13707 != 0.0 { 1.0 } else { 0.0 };
            if v13708 != 0.0 {
            } else {
            }
            if v12628 != 0.0 {
            } else {
            }
            let v13710 = v153 * v13709;
            let v13712 = v153 * v13711;
            let v13716 = v153 * v13713;
            let v13720 = v153 * v13717;
            let v13722 = v153 * v13721;
            let v13724 = v153 * v13723;
            let v13725 = if v96 == v0 { 1.0 } else { 0.0 };
            let v14137: f64;
            let v14138: f64;
            if v13725 != 0.0 {
                let v13726 = v13722 + v13724;
                let v13727 = v13726 * v10378;
                let v13728 = v13726 * v10379;
                v14137 = v13727;
                v14138 = v13728;
            } else {
                v14137 = v0;
                v14138 = v0;
            }
            let v17966 = (v17964 * v9888)[4];
            let v13729 = v11011 / v12212;
            let v13736 = if (if (if v4540 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v13731 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v13734 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v14342: f64;
            let v14424: f64;
            if v13736 != 0.0 {
                let v13739 = v3969 - (v23 * v13737);
                let v13740 = if v13739 <= v0 { 1.0 } else { 0.0 };
                let v13745: f64;
                if v13740 != 0.0 {
                    v13745 = v3969;
                } else {
                    v13745 = v13739;
                }
                let v13742 = if v13741 == v1 { 1.0 } else { 0.0 };
                let v13744 = if v13742 != 0.0 || (if v13741 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14343: f64;
                let v14425: f64;
                if v13744 != 0.0 {
                    let v13746 = v13745 * v13745;
                    let v13748 = if v13747 > v0 { 1.0 } else { 0.0 };
                    let v13823: f64;
                    if v13748 != 0.0 {
                        let v13751 = ((v12292 / v5580) + v13747) / v13729;
                        let v13752 = if v13751 > v185 { 1.0 } else { 0.0 };
                        let v13755: f64;
                        if v13752 != 0.0 {
                            let v13753 = v13751.ln();
                            v13755 = v13753;
                        } else {
                            v13755 = v13754;
                        }
                        let v13756 = v5580 * v13755;
                        v13823 = v13756;
                    } else {
                        v13823 = v0;
                    }
                    let v13797: f64;
                    if v13742 != 0.0 {
                        let v13764 = (v13762 / (v1 + ((v12150 / v13757).powf(v13759)))) / v4540;
                        let v13766 = v13764 - v1;
                        let v13775 = v4540 * (v4897 * ((v13764 + v1) + (((v13766 * v13766) + ((v5721 * v13768) * v13768)).sqrt())));
                        v13797 = v13775;
                    } else {
                        v13797 = v4540;
                    }
                    let v13780 = ((v13776 * v6600) * (v12604.abs())) * v12212;
                    let v13783 = (v13781 * v12631) * v13746;
                    let v13785 = (v12631 * v10933) / v415;
                    let v13787 = (v12631 * v12120) / v415;
                    let v13790 = (v6600 / v415) * (v12631 + v10544);
                    let v13792 = v13787 + v13790;
                    let v13793 = (v13785 + v13790) / v13792;
                    let v13794 = if v13793 > v185 { 1.0 } else { 0.0 };
                    let v13798: f64;
                    if v13794 != 0.0 {
                        let v13795 = v13793.ln();
                        v13798 = v13795;
                    } else {
                        v13798 = v13796;
                    }
                    let v13827 = ((v13780 / v13783) * (((v13797 * v13798) + (v13731 * (v13785 - v13787))) + ((v4897 * v13734) * ((v13785 * v13785) - (v13787 * v13787))))) + ((((((v6808 * v12604) * v12604) / (((v13781 * v13746) * v423) * v153)) * v13823) * ((v13797 + (v13731 * v13787)) + ((v13734 * v13787) * v13787))) / (v13792 * v13792));
                    let v13837 = ((((v13797 * v415) * v6600) / (((((v423 * v153) * v13745) * v13781) * v13790) * v13790)) * v12604) * v12604;
                    let v13838 = v13837 + v13827;
                    let v13839 = if v13838 > v0 { 1.0 } else { 0.0 };
                    let v14426: f64;
                    if v13839 != 0.0 {
                        let v13841 = (v13827 * v13837) / v13838;
                        v14426 = v13841;
                    } else {
                        v14426 = v0;
                    }
                    v14343 = v13783;
                    v14425 = v14426;
                } else {
                    let v13842 = if v13741 == v23 { 1.0 } else { 0.0 };
                    let v14344: f64;
                    let v14427: f64;
                    if v13842 != 0.0 {
                        let v13847 = (v13762 / (v1 + ((v12150 / v13757).powf(v13759)))) / v4540;
                        let v13849 = v13847 - v1;
                        let v13863 = v1 + (v13861 * v12123);
                        let v13868 = ((v4540 * (v4897 * ((v13847 + v1) + (((v13849 * v13849) + ((v5721 * v13768) * v13768)).sqrt())))) + (v13731 * v12122)) + ((v13734 * v12122) * v12122);
                        let v13871 = if (if (v1 + ((v6773 / v11014) * v12123)) > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v13863 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v14428: f64;
                        if v13871 != 0.0 {
                            let v13874 = v13863.powf((-v13872));
                            let v13877 = (v10933 + v4897) / (v12120 + v4897);
                            let v13878 = if v13877 > v185 { 1.0 } else { 0.0 };
                            let v13881: f64;
                            if v13878 != 0.0 {
                                let v13879 = v13877.ln();
                                v13881 = v13879;
                            } else {
                                v13881 = v13880;
                            }
                            let v13900 = (((((((v13881 * (v12121 + v1)) / (v23 * v12123)) * (v13886 * v6600)) * v17966) * v17966) * v13868) * v13874) / ((((((v13893 * v421) * v421) * v423) * v423) * v13745) * v153);
                            v14428 = v13900;
                        } else {
                            v14428 = v0;
                        }
                        v14344 = v13863;
                        v14427 = v14428;
                    } else {
                        v14344 = v14345;
                        v14427 = v0;
                    }
                    v14343 = v14344;
                    v14425 = v14427;
                }
                v14342 = v14343;
                v14424 = v14425;
            } else {
                v14342 = v14345;
                v14424 = v0;
            }
            let v13902 = if v13901 == v0 { 1.0 } else { 0.0 };
            let v14341: f64;
            let v14453: f64;
            let v14455: f64;
            if v13902 != 0.0 {
                let v13903 = v12212 * v12633;
                let v13915 = ((v22 * v6600) * v415) * ((v13903 / ((v13903 * v13904) + (v3969 * v3969))) * v13911);
                v14341 = v14342;
                v14453 = v13915;
                v14455 = v0;
            } else {
                let v13916 = if v13901 == v1 { 1.0 } else { 0.0 };
                let v14375: f64;
                let v14454: f64;
                let v14456: f64;
                if v13916 != 0.0 {
                    let v13917 = v12122 / v11014;
                    let v13918 = v13917 * v13917;
                    let v13924 = v13919 * (v1 + ((v13918 * v13920) * v3969));
                    let v13930 = v13925 * (v1 + ((v13918 * v13926) * v3969));
                    let v13942 = v13937 * (v1 + ((v13918 * v13938) * v3969));
                    let v13944 = (v276 * v13924) * v13924;
                    let v13947 = (v13945 * v13930) * v13930;
                    let v13949 = v13948 * (v13931 * (v1 + ((v13918 * v13932) * v3969)));
                    let v13951 = v11069 / v11062;
                    let v13953 = (v12120 / v10933) * (v1 - v13951);
                    let v13955 = (v12394 * v12394) * v12394;
                    let v13961 = rspice_limited_exp((-(v9225 / (((if v0 >= v9239 { v0 } else { v9239 }) * v10933) + v12365))));
                    let v14057: f64;
                    if v12404 != 0.0 {
                        let v13963 = if v9296 < v13962 { 1.0 } else { 0.0 };
                        let v13972: f64;
                        if v13963 != 0.0 {
                            let v13965 = v13964 / v9296;
                            v13972 = v13965;
                        } else {
                            let v13971 = v4897 * (v9296 + (((v9296 * v9296) + v13967).sqrt()));
                            v13972 = v13971;
                        }
                        let v13983 = rspice_limited_exp(((-(v13972 / (((if v0 >= v9253 { v0 } else { v9253 }) * v10933) + v12365))) * (((v10362 - v10947).sqrt()) - (v10362.sqrt()))));
                        v14057 = v13983;
                    } else {
                        v14057 = v1;
                    }
                    let v13985 = v5572 * (v12634 + v10934);
                    let v13989 = (v4897 * (v1 + (v10937.abs()))).powf(v10940);
                    let v14002: f64;
                    if v97 != 0.0 {
                        let v13996 = ((v10945 + (v10946 * v10947)) * ((v13985.abs()).powf(v10943))) + (v10951 / v13989);
                        v14002 = v13996;
                    } else {
                        let v14001 = (v10945 * ((v13985.abs()).powf(v10943))) + (v10951 / v13989);
                        v14002 = v14001;
                    }
                    let v14003 = v1 + v14002;
                    let v14005 = v14003 - v1;
                    let v14011 = (v4897 * ((v14003 + v1) + (((v14005 * v14005) + v10964).sqrt()))) / v10969;
                    let v14012 = v1 + v12389;
                    let v14017 = v10933 + ((v23 - (v10581 / (v10581 + v10933))) * v10555);
                    let v14060: f64;
                    if v4874 != 0.0 {
                        let v14020 = v1 / (v1 + (v10971 * v10933));
                        let v14034 = v1 + (((v12591 * v14017) / (v14011 * v14012)) * ((v10986 * (v10980 + (v10981 * (v4897 * (v14020 + (((v14020 * v14020) + v4536).sqrt())))))) * v5577));
                        v14060 = v14034;
                    } else {
                        let v14061: f64;
                        if v4402 != 0.0 {
                            v14061 = v1;
                        } else {
                            let v14035 = if v4401 == v23 { 1.0 } else { 0.0 };
                            let v14062: f64;
                            if v14035 != 0.0 {
                                let v14038 = v1 / (v1 + (v10971 * v10933));
                                let v14054 = v1 + (((v12591 * v14017) / (v14011 * v14012)) * (v10986 * ((v10997 + v11000) + ((v10980 + (v10981 * (v4897 * (v14038 + (((v14038 * v14038) + v4536).sqrt()))))) * v5577))));
                                v14062 = v14054;
                            } else {
                                v14062 = v0;
                            }
                            v14061 = v14062;
                        }
                        v14060 = v14061;
                    }
                    let v14064 = (((v12591 * v10933) * v13961) * v14057) / ((v14011 * v14012) * v14060);
                    let v14065 = v1 + v13953;
                    let v14066 = v1 - v13953;
                    let v14069 = ((v23 * v12476) / v10933) * v10555;
                    let v14070 = v14065 + v14069;
                    let v14071 = v14066 * v14066;
                    let v14072 = v14071 * v14066;
                    let v14074 = v14070 * v14070;
                    let v14075 = v14074 * v14070;
                    let v14076 = v14075 * v14070;
                    let v14078 = v4897 * v14065;
                    let v14080 = v14071 / (v6356 * v14070);
                    let v14081 = v12329 / v12394;
                    let v14093 = v12329 / v6356;
                    let v14107 = (v13949 * ((v14093 * v12394) * ((v14066 / v14070) - (v14072 / (v276 * v14075))))) / (((v14081 * (v14078 + v14080)) * ((v14093 * v13955) * (((v14065 / v14074) - ((((v6356 * v14065) + v14069) * v14071) / (v14088 * v14076))) + ((v14072 * v14066) / (v10789 * (v14076 * v14070)))))).sqrt());
                    let v14108 = if v14107 > v1 { 1.0 } else { 0.0 };
                    let v14457: f64;
                    if v14108 != 0.0 {
                        v14457 = v1;
                    } else {
                        let v14109 = if v14107 < v0 { 1.0 } else { 0.0 };
                        let v14458: f64;
                        if v14109 != 0.0 {
                            v14458 = v0;
                        } else {
                            v14458 = v14107;
                        }
                        v14457 = v14458;
                    }
                    let v14123 = (((v22 * v6600) * v415) * (v14081 * (((v1 + (((v13942 * v13942) / (v14111 + v12122)) * v13951)) * v14078) + (v13944 * v14080)))) * v14064;
                    v14375 = v13947;
                    v14454 = v14123;
                    v14456 = v14457;
                } else {
                    v14375 = v14342;
                    v14454 = v0;
                    v14456 = v0;
                }
                v14341 = v14375;
                v14453 = v14454;
                v14455 = v14456;
            }
            let v14124 = if v12645 > v0 { 1.0 } else { 0.0 };
            if v14124 != 0.0 {
            } else {
            }
            if v12628 != 0.0 {
            } else {
            }
            if v12657 != 0.0 {
            } else {
            }
            if v14124 != 0.0 {
                if v97 != 0.0 {
                    let v14130 = if (if v3204 == v23 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if v100 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v100 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v102 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v14130 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                if v97 != 0.0 {
                    let v14136 = if (if v3204 == v23 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if v100 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v100 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v102 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v14136 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if v13725 != 0.0 {
            } else {
            }
            if v97 != 0.0 {
            } else {
            }
            if v10333 != 0.0 {
                if v12657 != 0.0 {
                } else {
                }
                let v14139 = if v4882 == v1 { 1.0 } else { 0.0 };
                if v14139 != 0.0 {
                } else {
                }
            } else {
                if v12657 != 0.0 {
                } else {
                }
                let v14140 = if v4882 == v1 { 1.0 } else { 0.0 };
                if v14140 != 0.0 {
                } else {
                }
            }
            if v97 != 0.0 {
            } else {
            }
            let v14408: f64;
            let v14418: f64;
            if v4402 != 0.0 {
                let v14142 = if v14141 > v0 { 1.0 } else { 0.0 };
                let v14340: f64;
                let v14379: f64;
                let v14409: f64;
                if v14142 != 0.0 {
                    let v14146 = (v1 + (v14143 * v6598)) - v25;
                    let v14148 = if v14146 < v14147 { 1.0 } else { 0.0 };
                    let v14157: f64;
                    if v14148 != 0.0 {
                        let v14150 = v14149 / v14146;
                        v14157 = v14150;
                    } else {
                        let v14156 = v4897 * (v14146 + (((v14146 * v14146) + v14152).sqrt()));
                        v14157 = v14156;
                    }
                    let v14205: f64;
                    if v8914 != 0.0 {
                        let v14159 = -v14158;
                        let v14164 = (((-v14160) * v6598) - v14159) - v25;
                        let v14173 = v14158 + (v14159 + (v4897 * (v14164 + (((v14164 * v14164) - ((v22 * v14159) * v25)).sqrt()))));
                        v14205 = v14173;
                    } else {
                        let v14177 = (v1 + ((-v14160) * v6598)) - v25;
                        let v14179 = if v14177 < v14178 { 1.0 } else { 0.0 };
                        let v14188: f64;
                        if v14179 != 0.0 {
                            let v14181 = v14180 / v14177;
                            v14188 = v14181;
                        } else {
                            let v14187 = v4897 * (v14177 + (((v14177 * v14177) + v14183).sqrt()));
                            v14188 = v14187;
                        }
                        let v14189 = v14158 * v14188;
                        v14205 = v14189;
                    }
                    let v14191 = v10933 - v14190;
                    let v14193 = v14191 - v6154;
                    let v14199 = v4897 * ((v14191 + v6154) + (((v14193 * v14193) + v14195).sqrt()));
                    let v14201 = v10068 * v14200;
                    let v14209 = v14205 * (v1 + (v14206 * ((v14201 * v14199) / (v14201 + v14199))));
                    let v14211 = if v14209 < v14210 { 1.0 } else { 0.0 };
                    let v14220: f64;
                    if v14211 != 0.0 {
                        let v14213 = v14212 / v14209;
                        v14220 = v14213;
                    } else {
                        let v14219 = v4897 * (v14209 + (((v14209 * v14209) + v14215).sqrt()));
                        v14220 = v14219;
                    }
                    let v14223 = ((v153 * v423) * v415) * v14220;
                    let v14225 = (v12534 - v12758).abs();
                    let v14227 = if v14226 == v0 { 1.0 } else { 0.0 };
                    let v14240: f64;
                    if v14227 != 0.0 {
                        v14240 = v1;
                    } else {
                        let v14229 = v14225 - v14228;
                        let v14237 = v1 + ((v4897 * (v14229 + (((v14229 * v14229) + v14231).sqrt()))) * v14226);
                        v14240 = v14237;
                    }
                    let v14243 = (v14157 * v14141) * v5577;
                    let v14244 = ((v14223 * v14238) * v14240) * v14243;
                    let v14246 = v22 - v14245;
                    let v14247 = v14225.powf(v14246);
                    let v14253 = v1 / v14245;
                    let v14260 = v14243 * ((v1 + (((((v14247 / (v14247 + (v14248 * (v14244.powf(v14246))))).powf(v14253)) * v14225) / v14244).powf(v14245))).powf(v14253));
                    v14340 = v14223;
                    v14379 = v14157;
                    v14409 = v14260;
                } else {
                    v14340 = v14341;
                    v14379 = v0;
                    v14409 = v0;
                }
                let v14262 = if v14261 > v0 { 1.0 } else { 0.0 };
                let v14419: f64;
                if v14262 != 0.0 {
                    let v14263 = if v14141 == v0 { 1.0 } else { 0.0 };
                    let v14339: f64;
                    let v14378: f64;
                    if v14263 != 0.0 {
                        let v14266 = (v1 + (v14143 * v6598)) - v25;
                        let v14268 = if v14266 < v14267 { 1.0 } else { 0.0 };
                        let v14277: f64;
                        if v14268 != 0.0 {
                            let v14270 = v14269 / v14266;
                            v14277 = v14270;
                        } else {
                            let v14276 = v4897 * (v14266 + (((v14266 * v14266) + v14272).sqrt()));
                            v14277 = v14276;
                        }
                        let v14321: f64;
                        if v8914 != 0.0 {
                            let v14278 = -v14158;
                            let v14282 = (((-v14160) * v6598) - v14278) - v25;
                            let v14291 = v14158 + (v14278 + (v4897 * (v14282 + (((v14282 * v14282) - ((v22 * v14278) * v25)).sqrt()))));
                            v14321 = v14291;
                        } else {
                            let v14295 = (v1 + ((-v14160) * v6598)) - v25;
                            let v14297 = if v14295 < v14296 { 1.0 } else { 0.0 };
                            let v14306: f64;
                            if v14297 != 0.0 {
                                let v14299 = v14298 / v14295;
                                v14306 = v14299;
                            } else {
                                let v14305 = v4897 * (v14295 + (((v14295 * v14295) + v14301).sqrt()));
                                v14306 = v14305;
                            }
                            let v14307 = v14158 * v14306;
                            v14321 = v14307;
                        }
                        let v14308 = v10933 - v14190;
                        let v14310 = v14308 - v6154;
                        let v14316 = v4897 * ((v14308 + v6154) + (((v14310 * v14310) + v14312).sqrt()));
                        let v14317 = v10068 * v14200;
                        let v14324 = v14321 * (v1 + (v14206 * ((v14317 * v14316) / (v14317 + v14316))));
                        let v14326 = if v14324 < v14325 { 1.0 } else { 0.0 };
                        let v14335: f64;
                        if v14326 != 0.0 {
                            let v14328 = v14327 / v14324;
                            v14335 = v14328;
                        } else {
                            let v14334 = v4897 * (v14324 + (((v14324 * v14324) + v14330).sqrt()));
                            v14335 = v14334;
                        }
                        let v14338 = ((v153 * v423) * v415) * v14335;
                        v14339 = v14338;
                        v14378 = v14277;
                    } else {
                        v14339 = v14340;
                        v14378 = v14379;
                    }
                    let v14381 = (v14378 * v14261) * v5577;
                    let v14382 = (v14339 * v14376) * v14381;
                    let v14384 = (v10318 - v12499).abs();
                    let v14385 = v22 - v14245;
                    let v14386 = v14384.powf(v14385);
                    let v14392 = v1 / v14245;
                    let v14399 = v14381 * ((v1 + (((((v14386 / (v14386 + (v14387 * (v14382.powf(v14385))))).powf(v14392)) * v14384) / v14382).powf(v14245))).powf(v14392));
                    v14419 = v14399;
                } else {
                    v14419 = v0;
                }
                v14408 = v14409;
                v14418 = v14419;
            } else {
                v14408 = v0;
                v14418 = v0;
            }
            let v14400 = if v4401 != v23 { 1.0 } else { 0.0 };
            let v14402 = if v14400 != 0.0 && (if v11000 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v14434: f64;
            let v14438: f64;
            if v14402 != 0.0 {
                let v14405 = v1 / v14403;
                let v14407 = if v4402 != 0.0 && (if v14141 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14439: f64;
                if v14407 != 0.0 {
                    let v14410 = v1 / v14408;
                    v14439 = v14410;
                } else {
                    v14439 = v0;
                }
                v14434 = v14405;
                v14438 = v14439;
            } else {
                v14434 = v0;
                v14438 = v0;
            }
            let v14412 = if v14400 != 0.0 && (if v10997 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v14443: f64;
            let v14447: f64;
            if v14412 != 0.0 {
                let v14415 = v1 / v14413;
                let v14417 = if v4402 != 0.0 && (if v14261 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14448: f64;
                if v14417 != 0.0 {
                    let v14420 = v1 / v14418;
                    v14448 = v14420;
                } else {
                    v14448 = v0;
                }
                v14443 = v14415;
                v14447 = v14448;
            } else {
                v14443 = v0;
                v14447 = v0;
            }
            let v14421 = if v4633 != 0.0 && v13707 != 0.0 { 1.0 } else { 0.0 };
            if v14421 != 0.0 {
            } else {
            }
            if v12628 != 0.0 {
            } else {
            }
            if v4708 != 0.0 {
                let v14423 = if v4707 == v23 { 1.0 } else { 0.0 };
                if v14423 != 0.0 {
                } else {
                }
            } else {
            }
            let v14429 = v12645 * v14424;
            let v14552: f64;
            let v14553: f64;
            let v14554: f64;
            let v14556: f64;
            if v14402 != 0.0 {
                let v14433 = (v22 * v6600) * v415;
                let v14435 = v14433 * v14434;
                let v14437 = if v4402 != 0.0 && (if v14141 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14555: f64;
                let v14557: f64;
                if v14437 != 0.0 {
                    let v14440 = v14433 * v14438;
                    v14555 = v1;
                    v14557 = v14440;
                } else {
                    v14555 = v0;
                    v14557 = v0;
                }
                v14552 = v1;
                v14553 = v14435;
                v14554 = v14555;
                v14556 = v14557;
            } else {
                v14552 = v0;
                v14553 = v0;
                v14554 = v0;
                v14556 = v0;
            }
            let v14558: f64;
            let v14559: f64;
            let v14560: f64;
            let v14562: f64;
            if v14412 != 0.0 {
                let v14442 = (v22 * v6600) * v415;
                let v14444 = v14442 * v14443;
                let v14446 = if v4402 != 0.0 && (if v14261 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v14561: f64;
                let v14563: f64;
                if v14446 != 0.0 {
                    let v14449 = v14442 * v14447;
                    v14561 = v1;
                    v14563 = v14449;
                } else {
                    v14561 = v0;
                    v14563 = v0;
                }
                v14558 = v1;
                v14559 = v14444;
                v14560 = v14561;
                v14562 = v14563;
            } else {
                v14558 = v0;
                v14559 = v0;
                v14560 = v0;
                v14562 = v0;
            }
            let v14564: f64;
            let v14565: f64;
            if v4708 != 0.0 {
                let v14452 = ((v22 * v6600) * v415) * v14422;
                v14564 = v1;
                v14565 = v14452;
            } else {
                v14564 = v0;
                v14565 = v0;
            }
            let v14566: f64;
            let v14567: f64;
            let v14568: f64;
            let v14569: f64;
            let v14570: f64;
            let v14571: f64;
            if v13902 != 0.0 {
                v14566 = v1;
                v14567 = v14453;
                v14568 = v0;
                v14569 = v0;
                v14570 = v0;
                v14571 = v0;
            } else {
                let v14461 = v14453 * (v1 - (v14455 * v14455));
                v14566 = v0;
                v14567 = v0;
                v14568 = v1;
                v14569 = v14453;
                v14570 = v1;
                v14571 = v14461;
            }
            let v14572: f64;
            let v14574: f64;
            let v14576: f64;
            let v14578: f64;
            let v14580: f64;
            let v14582: f64;
            let v14584: f64;
            let v14586: f64;
            if v4623 != 0.0 {
                let v14573: f64;
                let v14575: f64;
                let v14577: f64;
                let v14579: f64;
                let v14581: f64;
                let v14583: f64;
                let v14585: f64;
                let v14587: f64;
                if v14124 != 0.0 {
                    let v14465 = v14462 * ((v13712 + v13716).abs());
                    let v14469 = v14466 * ((v13710 + v13720).abs());
                    v14573 = v1;
                    v14575 = v14465;
                    v14577 = v1;
                    v14579 = v14469;
                    v14581 = v0;
                    v14583 = v0;
                    v14585 = v0;
                    v14587 = v0;
                } else {
                    let v14473 = v14470 * ((v13712 + v13716).abs());
                    let v14477 = v14474 * ((v13710 + v13720).abs());
                    v14573 = v0;
                    v14575 = v0;
                    v14577 = v0;
                    v14579 = v0;
                    v14581 = v1;
                    v14583 = v14473;
                    v14585 = v1;
                    v14587 = v14477;
                }
                v14572 = v14573;
                v14574 = v14575;
                v14576 = v14577;
                v14578 = v14579;
                v14580 = v14581;
                v14582 = v14583;
                v14584 = v14585;
                v14586 = v14587;
            } else {
                v14572 = v0;
                v14574 = v0;
                v14576 = v0;
                v14578 = v0;
                v14580 = v0;
                v14582 = v0;
                v14584 = v0;
                v14586 = v0;
            }
            let v14588: f64;
            let v14590: f64;
            let v14592: f64;
            let v14594: f64;
            let v14596: f64;
            let v14598: f64;
            if v4619 != 0.0 {
                let v14589: f64;
                let v14591: f64;
                let v14593: f64;
                let v14595: f64;
                let v14597: f64;
                let v14599: f64;
                if v97 != 0.0 {
                    let v14481 = v14478 * ((v13722 + v13724).abs());
                    v14589 = v1;
                    v14591 = v14481;
                    v14593 = v0;
                    v14595 = v0;
                    v14597 = v0;
                    v14599 = v0;
                } else {
                    let v14484 = v14482 * (v14137.abs());
                    let v14487 = v14485 * (v14138.abs());
                    v14589 = v0;
                    v14591 = v0;
                    v14593 = v1;
                    v14595 = v14484;
                    v14597 = v1;
                    v14599 = v14487;
                }
                v14588 = v14589;
                v14590 = v14591;
                v14592 = v14593;
                v14594 = v14595;
                v14596 = v14597;
                v14598 = v14599;
            } else {
                v14588 = v0;
                v14590 = v0;
                v14592 = v0;
                v14594 = v0;
                v14596 = v0;
                v14598 = v0;
            }
            if v4700 != 0.0 {
                if v14402 != 0.0 {
                    let v14489 = if v4402 != 0.0 && (if v14141 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v14489 != 0.0 {
                    } else {
                    }
                } else {
                }
                if v14412 != 0.0 {
                    let v14491 = if v4402 != 0.0 && (if v14261 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v14491 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            if v14124 != 0.0 {
                if v97 != 0.0 {
                    let v14497 = if (if v3204 == v23 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if v100 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v100 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v102 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v14497 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                if v97 != 0.0 {
                    let v14503 = if (if v3204 == v23 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if v100 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v100 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v102 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v14503 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            if v13725 != 0.0 {
            } else {
            }
            if v14124 != 0.0 {
                let v14510 = if (if v97 != 0.0 && (if v3204 == v23 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if v100 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v100 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v102 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v14510 != 0.0 {
                } else {
                }
            } else {
                let v14517 = if (if v97 != 0.0 && (if v3204 == v23 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if v100 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v100 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v102 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v14517 != 0.0 {
                } else {
                }
            }
            if v97 != 0.0 {
            } else {
            }
            if v97 != 0.0 {
                let v14525 = if (if (if v3204 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v3204 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if (if v100 == v23 { 1.0 } else { 0.0 }) != 0.0 || (if v100 == v276 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v102 != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v14525 != 0.0 {
                } else {
                }
            } else {
            }
            let v14529 = if v14526 > v0 { 1.0 } else { 0.0 };
            if v14529 != 0.0 {
                if v6778 != 0.0 {
                    let v14531 = if (v14526 / v6777) > v185 { 1.0 } else { 0.0 };
                    if v14531 != 0.0 {
                    } else {
                    }
                } else {
                    let v14532 = if v14526 > v185 { 1.0 } else { 0.0 };
                    if v14532 != 0.0 {
                    } else {
                    }
                }
            } else {
            }
            let v14545 = if (((v421 * v421) * (v6600 * (v6600 + (((v10068 * v6600) / v414) + v10580)))) / (((v14540 * v6777) * v16) * v6600)) > v185 { 1.0 } else { 0.0 };
            if v14545 != 0.0 {
            } else {
            }
            if v14124 != 0.0 {
            } else {
            }
            if v97 != 0.0 {
            } else {
            }
            let v14546 = if v4882 == v1 { 1.0 } else { 0.0 };
            if v14546 != 0.0 {
            } else {
            }
            if v14546 != 0.0 {
            } else {
            }
            if v14546 != 0.0 {
            } else {
            }
            let v14547 = if v4707 == v23 { 1.0 } else { 0.0 };
            if v14547 != 0.0 {
                if v14546 != 0.0 {
                } else {
                }
                if v14546 != 0.0 {
                } else {
                }
            } else {
                if v14546 != 0.0 {
                } else {
                }
                if v14546 != 0.0 {
                } else {
                }
            }
            if v14547 != 0.0 {
                if v14546 != 0.0 {
                } else {
                }
            } else {
                if v14546 != 0.0 {
                } else {
                }
            }
            if v14547 != 0.0 {
                if v14546 != 0.0 {
                } else {
                }
            } else {
                if v14546 != 0.0 {
                } else {
                }
            }
            if v14546 != 0.0 {
            } else {
            }
            if v14546 != 0.0 {
            } else {
            }
            if v13725 != 0.0 {
            } else {
            }
            if v14400 != 0.0 {
                let v14551 = if v4402 != 0.0 && (if (if v14141 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v14261 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v14551 != 0.0 {
                } else {
                }
            } else {
            }
        {
            let psd = v14429;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = Some(v14430);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14552 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14553;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14554 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14556;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14558 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14559;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14560 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14562;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14564 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14565;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14566 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14567;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14568 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14569;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14570 == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14571;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14572 == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14574;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14576 == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14578;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 10, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14580 == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14582;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14584 == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14586;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14588 == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14590;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 13, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14592 == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14594;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v14596 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v14598;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

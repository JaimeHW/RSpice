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
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 11] = [
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI_RD", label: Some("Rd"), kind: GeneratedNoiseKind::White, equation: 23, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS", label: Some("Rs"), kind: GeneratedNoiseKind::White, equation: 24, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_FG_GE_RG", label: Some("Rg"), kind: GeneratedNoiseKind::White, equation: 29, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "fg", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "ge", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 30, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("Id"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("Igs"), kind: GeneratedNoiseKind::White, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("Igd"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("Igd"), kind: GeneratedNoiseKind::White, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("Igs"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGB", label: Some("Igb"), kind: GeneratedNoiseKind::White, equation: 36, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGB", label: Some("Igb"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8])];
            let v0 = 0e0f64;
            let v1 = 1e0f64;
            let v2 = 1.0f64;
            let v3 = parameters[18];
            let v5 = parameters[310];
            let v8 = parameters[12];
            let v10 = -1e0f64;
            let v11 = parameters[13];
            let v13 = -1e0f64;
            let v14 = parameters[59];
            let v15 = 8.85418e-12f64;
            let v17 = parameters[21];
            let v19 = parameters[1];
            let v20 = parameters[2];
            let v22 = parameters[0];
            let v23 = parameters[23];
            let v26 = parameters[24];
            let v28 = parameters[29];
            let v31 = parameters[30];
            let v35 = parameters[25];
            let v36 = parameters[26];
            let v39 = parameters[27];
            let v42 = parameters[28];
            let v45 = parameters[35];
            let v48 = parameters[36];
            let v52 = parameters[31];
            let v53 = parameters[32];
            let v56 = parameters[33];
            let v59 = parameters[34];
            let v62 = 2e0f64;
            let v66 = 1e-9f64;
            let v72 = parameters[37];
            let v73 = parameters[38];
            let v76 = parameters[39];
            let v79 = parameters[40];
            let v82 = parameters[41];
            let v83 = parameters[42];
            let v86 = parameters[43];
            let v89 = parameters[44];
            let v100 = 1e-6f64;
            let v104 = parameters[191];
            let v105 = parameters[319];
            let v108 = parameters[320];
            let v111 = parameters[321];
            let v114 = parameters[199];
            let v115 = parameters[325];
            let v118 = parameters[326];
            let v121 = parameters[327];
            let v124 = parameters[195];
            let v125 = parameters[322];
            let v128 = parameters[323];
            let v131 = parameters[324];
            let v134 = parameters[202];
            let v135 = parameters[328];
            let v138 = parameters[329];
            let v141 = parameters[330];
            let v144 = parameters[203];
            let v145 = parameters[331];
            let v148 = parameters[332];
            let v151 = parameters[333];
            let v154 = parameters[204];
            let v155 = parameters[334];
            let v158 = parameters[335];
            let v161 = parameters[336];
            let v164 = parameters[57];
            let v165 = parameters[337];
            let v168 = parameters[338];
            let v171 = parameters[339];
            let v174 = parameters[58];
            let v175 = parameters[340];
            let v178 = parameters[341];
            let v181 = parameters[342];
            let v184 = parameters[51];
            let v185 = parameters[343];
            let v188 = parameters[344];
            let v191 = parameters[345];
            let v194 = parameters[50];
            let v195 = parameters[346];
            let v198 = parameters[347];
            let v201 = parameters[348];
            let v204 = parameters[63];
            let v205 = parameters[349];
            let v208 = parameters[350];
            let v211 = parameters[351];
            let v214 = parameters[64];
            let v215 = parameters[352];
            let v218 = parameters[353];
            let v221 = parameters[354];
            let v224 = parameters[65];
            let v225 = parameters[355];
            let v228 = parameters[356];
            let v231 = parameters[357];
            let v234 = parameters[68];
            let v235 = parameters[358];
            let v238 = parameters[359];
            let v241 = parameters[360];
            let v244 = parameters[276];
            let v245 = parameters[361];
            let v248 = parameters[362];
            let v251 = parameters[363];
            let v254 = parameters[291];
            let v255 = parameters[751];
            let v258 = parameters[752];
            let v261 = parameters[753];
            let v264 = parameters[294];
            let v265 = parameters[757];
            let v268 = parameters[758];
            let v271 = parameters[759];
            let v274 = parameters[293];
            let v275 = parameters[754];
            let v278 = parameters[755];
            let v281 = parameters[756];
            let v286 = parameters[277];
            let v287 = parameters[364];
            let v290 = parameters[365];
            let v293 = parameters[366];
            let v296 = parameters[278];
            let v297 = parameters[367];
            let v300 = parameters[368];
            let v303 = parameters[369];
            let v306 = parameters[275];
            let v307 = parameters[370];
            let v310 = parameters[371];
            let v313 = parameters[372];
            let v316 = parameters[272];
            let v317 = parameters[373];
            let v320 = parameters[374];
            let v323 = parameters[375];
            let v326 = parameters[273];
            let v327 = parameters[376];
            let v330 = parameters[377];
            let v333 = parameters[378];
            let v336 = parameters[274];
            let v337 = parameters[379];
            let v340 = parameters[380];
            let v343 = parameters[381];
            let v346 = parameters[283];
            let v347 = parameters[382];
            let v350 = parameters[383];
            let v353 = parameters[384];
            let v358 = parameters[284];
            let v359 = parameters[385];
            let v362 = parameters[386];
            let v365 = parameters[387];
            let v368 = parameters[285];
            let v369 = parameters[388];
            let v372 = parameters[389];
            let v375 = parameters[390];
            let v378 = parameters[282];
            let v379 = parameters[391];
            let v382 = parameters[392];
            let v385 = parameters[393];
            let v388 = parameters[279];
            let v389 = parameters[394];
            let v392 = parameters[395];
            let v395 = parameters[396];
            let v398 = parameters[280];
            let v399 = parameters[397];
            let v402 = parameters[398];
            let v405 = parameters[399];
            let v408 = parameters[281];
            let v409 = parameters[400];
            let v412 = parameters[401];
            let v415 = parameters[402];
            let v418 = parameters[71];
            let v419 = parameters[403];
            let v422 = parameters[404];
            let v425 = parameters[405];
            let v428 = parameters[72];
            let v429 = parameters[406];
            let v432 = parameters[407];
            let v435 = parameters[408];
            let v438 = parameters[73];
            let v439 = parameters[409];
            let v442 = parameters[410];
            let v445 = parameters[411];
            let v448 = parameters[74];
            let v449 = parameters[412];
            let v452 = parameters[413];
            let v455 = parameters[414];
            let v458 = parameters[75];
            let v459 = parameters[415];
            let v462 = parameters[416];
            let v465 = parameters[417];
            let v468 = parameters[84];
            let v469 = parameters[418];
            let v472 = parameters[419];
            let v475 = parameters[420];
            let v478 = parameters[76];
            let v479 = parameters[421];
            let v482 = parameters[422];
            let v485 = parameters[423];
            let v488 = parameters[87];
            let v489 = parameters[430];
            let v492 = parameters[431];
            let v495 = parameters[432];
            let v498 = parameters[88];
            let v499 = parameters[433];
            let v502 = parameters[434];
            let v505 = parameters[435];
            let v508 = parameters[61];
            let v509 = parameters[436];
            let v512 = parameters[437];
            let v515 = parameters[438];
            let v518 = parameters[62];
            let v519 = parameters[439];
            let v522 = parameters[440];
            let v525 = parameters[441];
            let v528 = parameters[85];
            let v529 = parameters[424];
            let v532 = parameters[425];
            let v535 = parameters[426];
            let v538 = parameters[86];
            let v539 = parameters[427];
            let v542 = parameters[428];
            let v545 = parameters[429];
            let v548 = parameters[113];
            let v549 = parameters[460];
            let v552 = parameters[461];
            let v555 = parameters[462];
            let v558 = parameters[89];
            let v559 = parameters[442];
            let v562 = parameters[443];
            let v565 = parameters[444];
            let v568 = parameters[90];
            let v569 = parameters[445];
            let v572 = parameters[446];
            let v575 = parameters[447];
            let v578 = parameters[91];
            let v579 = parameters[448];
            let v582 = parameters[449];
            let v585 = parameters[450];
            let v588 = parameters[92];
            let v589 = parameters[451];
            let v592 = parameters[452];
            let v595 = parameters[453];
            let v598 = parameters[93];
            let v599 = parameters[454];
            let v602 = parameters[455];
            let v605 = parameters[456];
            let v608 = parameters[94];
            let v609 = parameters[457];
            let v612 = parameters[458];
            let v615 = parameters[459];
            let v618 = parameters[116];
            let v619 = parameters[463];
            let v622 = parameters[464];
            let v625 = parameters[465];
            let v628 = parameters[123];
            let v629 = parameters[466];
            let v632 = parameters[467];
            let v635 = parameters[468];
            let v638 = parameters[124];
            let v639 = parameters[469];
            let v642 = parameters[470];
            let v645 = parameters[471];
            let v648 = parameters[122];
            let v649 = parameters[472];
            let v652 = parameters[473];
            let v655 = parameters[474];
            let v658 = parameters[135];
            let v659 = parameters[475];
            let v662 = parameters[476];
            let v665 = parameters[477];
            let v668 = parameters[139];
            let v669 = parameters[478];
            let v672 = parameters[479];
            let v675 = parameters[480];
            let v678 = parameters[145];
            let v679 = parameters[481];
            let v682 = parameters[482];
            let v685 = parameters[483];
            let v688 = parameters[148];
            let v689 = parameters[484];
            let v692 = parameters[485];
            let v695 = parameters[486];
            let v698 = parameters[155];
            let v699 = parameters[487];
            let v702 = parameters[488];
            let v705 = parameters[489];
            let v708 = parameters[142];
            let v709 = parameters[490];
            let v712 = parameters[491];
            let v715 = parameters[492];
            let v718 = parameters[163];
            let v719 = parameters[493];
            let v722 = parameters[494];
            let v725 = parameters[495];
            let v728 = parameters[157];
            let v729 = parameters[496];
            let v732 = parameters[497];
            let v735 = parameters[498];
            let v738 = parameters[156];
            let v739 = parameters[499];
            let v742 = parameters[500];
            let v745 = parameters[501];
            let v748 = parameters[158];
            let v749 = parameters[502];
            let v752 = parameters[503];
            let v755 = parameters[504];
            let v758 = parameters[160];
            let v759 = parameters[505];
            let v762 = parameters[506];
            let v765 = parameters[507];
            let v768 = parameters[161];
            let v769 = parameters[508];
            let v772 = parameters[509];
            let v775 = parameters[510];
            let v778 = parameters[136];
            let v779 = parameters[511];
            let v782 = parameters[512];
            let v785 = parameters[513];
            let v788 = parameters[166];
            let v789 = parameters[514];
            let v792 = parameters[515];
            let v795 = parameters[516];
            let v798 = parameters[167];
            let v799 = parameters[517];
            let v802 = parameters[518];
            let v805 = parameters[519];
            let v808 = parameters[173];
            let v809 = parameters[520];
            let v812 = parameters[521];
            let v815 = parameters[522];
            let v818 = parameters[176];
            let v819 = parameters[523];
            let v822 = parameters[524];
            let v825 = parameters[525];
            let v828 = parameters[182];
            let v829 = parameters[526];
            let v832 = parameters[527];
            let v835 = parameters[528];
            let v838 = parameters[170];
            let v839 = parameters[529];
            let v842 = parameters[530];
            let v845 = parameters[531];
            let v848 = parameters[183];
            let v849 = parameters[532];
            let v852 = parameters[533];
            let v855 = parameters[534];
            let v858 = parameters[186];
            let v859 = parameters[535];
            let v862 = parameters[536];
            let v865 = parameters[537];
            let v868 = parameters[119];
            let v869 = parameters[538];
            let v872 = parameters[539];
            let v875 = parameters[540];
            let v878 = parameters[130];
            let v879 = parameters[541];
            let v882 = parameters[542];
            let v885 = parameters[543];
            let v888 = parameters[205];
            let v889 = parameters[544];
            let v892 = parameters[545];
            let v895 = parameters[546];
            let v898 = parameters[305];
            let v899 = parameters[547];
            let v902 = parameters[548];
            let v905 = parameters[549];
            let v908 = parameters[306];
            let v909 = parameters[550];
            let v912 = parameters[551];
            let v915 = parameters[552];
            let v918 = parameters[307];
            let v919 = parameters[553];
            let v922 = parameters[554];
            let v925 = parameters[555];
            let v928 = parameters[308];
            let v929 = parameters[556];
            let v932 = parameters[557];
            let v935 = parameters[558];
            let v938 = parameters[210];
            let v939 = parameters[559];
            let v942 = parameters[560];
            let v945 = parameters[561];
            let v948 = parameters[214];
            let v949 = parameters[562];
            let v952 = parameters[563];
            let v955 = parameters[564];
            let v958 = parameters[208];
            let v959 = parameters[565];
            let v962 = parameters[566];
            let v965 = parameters[567];
            let v968 = parameters[206];
            let v969 = parameters[568];
            let v972 = parameters[569];
            let v975 = parameters[570];
            let v978 = parameters[207];
            let v979 = parameters[571];
            let v982 = parameters[572];
            let v985 = parameters[573];
            let v988 = parameters[209];
            let v989 = parameters[574];
            let v992 = parameters[575];
            let v995 = parameters[576];
            let v998 = parameters[256];
            let v999 = parameters[577];
            let v1002 = parameters[578];
            let v1005 = parameters[579];
            let v1008 = parameters[257];
            let v1009 = parameters[580];
            let v1012 = parameters[581];
            let v1015 = parameters[582];
            let v1018 = parameters[258];
            let v1019 = parameters[583];
            let v1022 = parameters[584];
            let v1025 = parameters[585];
            let v1028 = parameters[217];
            let v1029 = parameters[706];
            let v1032 = parameters[707];
            let v1035 = parameters[708];
            let v1038 = parameters[218];
            let v1039 = parameters[709];
            let v1042 = parameters[710];
            let v1045 = parameters[711];
            let v1048 = parameters[219];
            let v1049 = parameters[712];
            let v1052 = parameters[713];
            let v1055 = parameters[714];
            let v1058 = parameters[220];
            let v1059 = parameters[715];
            let v1062 = parameters[716];
            let v1065 = parameters[717];
            let v1068 = parameters[221];
            let v1069 = parameters[718];
            let v1072 = parameters[719];
            let v1075 = parameters[720];
            let v1078 = parameters[222];
            let v1079 = parameters[721];
            let v1082 = parameters[722];
            let v1085 = parameters[723];
            let v1088 = parameters[223];
            let v1089 = parameters[724];
            let v1092 = parameters[725];
            let v1095 = parameters[726];
            let v1098 = parameters[224];
            let v1099 = parameters[727];
            let v1102 = parameters[728];
            let v1105 = parameters[729];
            let v1108 = parameters[225];
            let v1109 = parameters[730];
            let v1112 = parameters[731];
            let v1115 = parameters[732];
            let v1118 = parameters[226];
            let v1119 = parameters[586];
            let v1122 = parameters[587];
            let v1125 = parameters[588];
            let v1128 = parameters[227];
            let v1129 = parameters[589];
            let v1132 = parameters[590];
            let v1135 = parameters[591];
            let v1138 = parameters[228];
            let v1139 = parameters[592];
            let v1142 = parameters[593];
            let v1145 = parameters[594];
            let v1148 = parameters[230];
            let v1149 = parameters[595];
            let v1152 = parameters[596];
            let v1155 = parameters[597];
            let v1158 = parameters[229];
            let v1159 = parameters[598];
            let v1162 = parameters[599];
            let v1165 = parameters[600];
            let v1168 = parameters[250];
            let v1169 = parameters[619];
            let v1172 = parameters[620];
            let v1175 = parameters[621];
            let v1178 = parameters[251];
            let v1179 = parameters[622];
            let v1182 = parameters[623];
            let v1185 = parameters[624];
            let v1188 = parameters[244];
            let v1189 = parameters[601];
            let v1192 = parameters[602];
            let v1195 = parameters[603];
            let v1198 = parameters[245];
            let v1199 = parameters[604];
            let v1202 = parameters[605];
            let v1205 = parameters[606];
            let v1208 = parameters[231];
            let v1209 = parameters[637];
            let v1212 = parameters[638];
            let v1215 = parameters[639];
            let v1218 = parameters[232];
            let v1219 = parameters[643];
            let v1222 = parameters[644];
            let v1225 = parameters[645];
            let v1228 = parameters[233];
            let v1229 = parameters[649];
            let v1232 = parameters[650];
            let v1235 = parameters[651];
            let v1238 = parameters[242];
            let v1239 = parameters[655];
            let v1242 = parameters[656];
            let v1245 = parameters[657];
            let v1248 = parameters[236];
            let v1249 = parameters[640];
            let v1252 = parameters[641];
            let v1255 = parameters[642];
            let v1258 = parameters[237];
            let v1259 = parameters[646];
            let v1262 = parameters[647];
            let v1265 = parameters[648];
            let v1268 = parameters[238];
            let v1269 = parameters[652];
            let v1272 = parameters[653];
            let v1275 = parameters[654];
            let v1278 = parameters[243];
            let v1279 = parameters[658];
            let v1282 = parameters[659];
            let v1285 = parameters[660];
            let v1288 = parameters[240];
            let v1289 = parameters[661];
            let v1292 = parameters[662];
            let v1295 = parameters[663];
            let v1298 = parameters[241];
            let v1299 = parameters[664];
            let v1302 = parameters[665];
            let v1305 = parameters[666];
            let v1308 = parameters[100];
            let v1309 = parameters[679];
            let v1312 = parameters[680];
            let v1315 = parameters[681];
            let v1318 = parameters[129];
            let v1319 = parameters[682];
            let v1322 = parameters[683];
            let v1325 = parameters[684];
            let v1328 = parameters[103];
            let v1329 = parameters[685];
            let v1332 = parameters[686];
            let v1335 = parameters[687];
            let v1338 = parameters[106];
            let v1339 = parameters[688];
            let v1342 = parameters[689];
            let v1345 = parameters[690];
            let v1348 = parameters[110];
            let v1349 = parameters[691];
            let v1352 = parameters[692];
            let v1355 = parameters[693];
            let v1358 = parameters[111];
            let v1359 = parameters[694];
            let v1362 = parameters[695];
            let v1365 = parameters[696];
            let v1368 = parameters[112];
            let v1369 = parameters[697];
            let v1372 = parameters[698];
            let v1375 = parameters[699];
            let v1378 = parameters[137];
            let v1379 = parameters[700];
            let v1382 = parameters[701];
            let v1385 = parameters[702];
            let v1388 = parameters[187];
            let v1389 = parameters[703];
            let v1392 = parameters[704];
            let v1395 = parameters[705];
            let v1398 = parameters[95];
            let v1399 = parameters[739];
            let v1402 = parameters[740];
            let v1405 = parameters[741];
            let v1408 = parameters[96];
            let v1409 = parameters[742];
            let v1412 = parameters[743];
            let v1415 = parameters[744];
            let v1418 = parameters[97];
            let v1419 = parameters[745];
            let v1422 = parameters[746];
            let v1425 = parameters[747];
            let v1428 = parameters[98];
            let v1429 = parameters[748];
            let v1432 = parameters[749];
            let v1435 = parameters[750];
            let v1438 = parameters[20];
            let v1440 = parameters[317];
            let v1443 = parameters[733];
            let v1446 = parameters[734];
            let v1449 = parameters[735];
            let v1452 = 3.9e0f64;
            let v1453 = 3.4531302e-11f64;
            let v1454 = parameters[45];
            let v1456 = 3.4531302e-11f64;
            let v1457 = parameters[47];
            let v1459 = 3.4531302e-11f64;
            let v1460 = parameters[46];
            let v1462 = parameters[49];
            let v1465 = if parameter_given[47] { 1.0 } else { 0.0 };
            let v1467 = parameters[60];
            let v1470 = parameters[48];
            let v1472 = parameters[138];
            let v1481 = parameters[140];
            let v1483 = parameters[141];
            let v1488 = parameters[146];
            let v1489 = parameters[147];
            let v1494 = parameters[151];
            let v1495 = parameters[152];
            let v1496 = parameters[153];
            let v1501 = parameters[149];
            let v1502 = parameters[150];
            let v1507 = parameters[143];
            let v1508 = parameters[144];
            let v1513 = parameters[164];
            let v1514 = parameters[165];
            let v1519 = parameters[188];
            let v1528 = parameters[168];
            let v1529 = parameters[169];
            let v1534 = parameters[174];
            let v1535 = parameters[175];
            let v1540 = parameters[179];
            let v1541 = parameters[180];
            let v1542 = parameters[181];
            let v1547 = parameters[177];
            let v1548 = parameters[178];
            let v1553 = parameters[171];
            let v1554 = parameters[172];
            let v1559 = parameters[184];
            let v1560 = parameters[185];
            let v1565 = parameters[14];
            let v1567 = parameters[196];
            let v1568 = parameters[197];
            let v1573 = parameters[200];
            let v1574 = parameters[201];
            let v1579 = parameters[192];
            let v1580 = parameters[193];
            let v1585 = parameters[211];
            let v1586 = parameters[212];
            let v1591 = parameters[114];
            let v1592 = 1e6f64;
            let v1594 = parameters[115];
            let v1599 = parameters[117];
            let v1600 = parameters[118];
            let v1605 = parameters[125];
            let v1606 = parameters[126];
            let v1611 = parameters[127];
            let v1612 = parameters[128];
            let v1617 = parameters[101];
            let v1618 = parameters[102];
            let v1623 = parameters[132];
            let v1624 = parameters[133];
            let v1629 = parameters[104];
            let v1630 = parameters[105];
            let v1635 = parameters[107];
            let v1636 = parameters[108];
            let v1641 = parameters[77];
            let v1642 = parameters[79];
            let v1643 = parameters[80];
            let v1648 = parameters[78];
            let v1649 = parameters[81];
            let v1650 = parameters[82];
            let v1657 = 3e-2f64;
            let v1665 = parameters[190];
            let v1669 = parameters[194];
            let v1673 = parameters[198];
            let v1691 = 1e-38f64;
            let v1692 = 5e-1f64;
            let v1696 = 3e0f64;
            let v1697 = 3.333333333333333e-1f64;
            let v1699 = 3.333333333333333e-1f64;
            let v1700 = 3.333333333333333e-1f64;
            let v1702 = 1e-8f64;
            let v1713 = parameters[296];
            let v1719 = parameters[215];
            let v1720 = parameters[7];
            let v1722 = parameters[216];
            let v1723 = parameters[8];
            let v1725 = 1e-3f64;
            let v1740 = parameters[297];
            let v1742 = 3.0015e2f64;
            let v1743 = 2.7315e2f64;
            let v1745 = 4.97232e-7f64;
            let v1746 = 3.42537e-7f64;
            let v1747 = 7.45669e11f64;
            let v1748 = 1.16645e12f64;
            let v1749 = parameters[99];
            let v1753 = parameters[239];
            let v1769 = parameters[316];
            let v1770 = parameters[313];
            let v1772 = parameters[315];
            let v1777 = parameters[314];
            let v1783 = 1e3f64;
            let v1784 = parameters[19];
            let v1786 = temperature;
            let v1787 = node_potentials[4];
            let v1789 = parameters[9];
            let v1792 = parameters[298];
            let v1799 = 2.5e-1f64;
            let v1800 = 1e-2f64;
            let v1801 = 2.5e-5f64;
            let v1809 = 8.61708e-5f64;
            let v1811 = parameters[55];
            let v1812 = parameters[299];
            let v1815 = parameters[300];
            let v1822 = parameters[54];
            let v1824 = 5.1728331239999994e-2f64;
            let v1842 = parameters[52];
            let v1849 = 4e0f64;
            let v1850 = 1e-4f64;
            let v1851 = 4e-8f64;
            let v1858 = if parameter_given[58] { 1.0 } else { 0.0 };
            let v1861 = -1e0f64;
            let v1869 = parameters[53];
            let v1893 = 9e-1f64;
            let v1897 = 4e-6f64;
            let v1903 = 9.000011111097395e-1f64;
            let v1906 = parameters[159];
            let v1911 = 4e-6f64;
            let v1922 = 4e-6f64;
            let v1938 = 4e-6f64;
            let v1943 = parameters[120];
            let v1950 = 4e-6f64;
            let v1956 = 9.000011111097395e-1f64;
            let v1960 = 4e-6f64;
            let v1966 = 9.000011111097395e-1f64;
            let v1970 = 4e-6f64;
            let v1976 = 9.000011111097395e-1f64;
            let v1980 = -9e-1f64;
            let v1981 = parameters[309];
            let v1983 = -9e-1f64;
            let v1986 = -9e-1f64;
            let v1989 = -9e-1f64;
            let v1993 = -3.6e-4f64;
            let v2001 = parameters[131];
            let v2009 = 4e-6f64;
            let v2015 = 9.000011111097395e-1f64;
            let v2018 = parameters[121];
            let v2024 = 4e-6f64;
            let v2051 = 4e-6f64;
            let v2057 = parameters[301];
            let v2058 = parameters[302];
            let v2069 = 4e-6f64;
            let v2079 = 4e-6f64;
            let v2090 = node_potentials[8];
            let v2091 = node_potentials[6];
            let v2094 = node_potentials[5];
            let v2099 = node_potentials[3];
            let v2107 = -1e0f64;
            let v2111 = 4e-4f64;
            let v2114 = 2e-2f64;
            let v2126 = 3.75e-1f64;
            let v2140 = 3.141592653589793e0f64;
            let v2149 = 4e1f64;
            let v2165 = parameters[83];
            let v2190 = -1e0f64;
            let v2227 = 1.60219e-19f64;
            let v2239 = 4e-4f64;
            let v2265 = -1.2e0f64;
            let v2281 = 4e-6f64;
            let v2286 = 4e-1f64;
            let v2319 = parameters[70];
            let v2323 = parameters[66];
            let v2325 = parameters[67];
            let v2331 = parameters[69];
            let v2351 = parameters[303];
            let v2352 = parameters[304];
            let v2366 = parameters[10];
            let v2368 = 3.20438e-19f64;
            let v2377 = 3.947841e1f64;
            let v2378 = 3.675753940198048e0f64;
            let v2421 = 6.534e-2f64;
            let v2424 = 8.57973e0f64;
            let v2428 = 7.895683e1f64;
            let v2433 = -4e0f64;
            let v2448 = 2.8985507246376816e0f64;
            let v2453 = 5e1f64;
            let v2483 = -2e0f64;
            let v2497 = -2e0f64;
            let v2502 = -4e0f64;
            let v2517 = 1e1f64;
            let v2518 = -1e1f64;
            let v2531 = -2e0f64;
            let v2536 = -4e0f64;
            let v2550 = -1e1f64;
            let v2556 = 1.05e0f64;
            let v2579 = -5e-1f64;
            let v2593 = -2.5e-1f64;
            let v2617 = -2e0f64;
            let v2623 = -1e0f64;
            let v2660 = -5e-1f64;
            let v2674 = -2.5e-1f64;
            let v2697 = -2e0f64;
            let v2703 = -1e0f64;
            let v2739 = -5e-1f64;
            let v2753 = -2.5e-1f64;
            let v2776 = -2e0f64;
            let v2782 = -1e0f64;
            let v2818 = -5e-1f64;
            let v2832 = -2.5e-1f64;
            let v2855 = -2e0f64;
            let v2861 = -1e0f64;
            let v2897 = -5e-1f64;
            let v2911 = -2.5e-1f64;
            let v2934 = -2e0f64;
            let v2940 = -1e0f64;
            let v3042 = parameters[154];
            let v3049 = parameters[11];
            let v3160 = 4.0000000000000007e-10f64;
            let v3225 = -4e0f64;
            let v3239 = 2.8985507246376816e0f64;
            let v3271 = -2e0f64;
            let v3285 = -2e0f64;
            let v3290 = -4e0f64;
            let v3304 = -1e1f64;
            let v3317 = -2e0f64;
            let v3322 = -4e0f64;
            let v3336 = -1e1f64;
            let v3363 = -5e-1f64;
            let v3377 = -2.5e-1f64;
            let v3401 = -2e0f64;
            let v3407 = -1e0f64;
            let v3443 = -5e-1f64;
            let v3457 = -2.5e-1f64;
            let v3480 = -2e0f64;
            let v3486 = -1e0f64;
            let v3522 = -5e-1f64;
            let v3536 = -2.5e-1f64;
            let v3559 = -2e0f64;
            let v3565 = -1e0f64;
            let v3601 = -5e-1f64;
            let v3615 = -2.5e-1f64;
            let v3638 = -2e0f64;
            let v3644 = -1e0f64;
            let v3680 = -5e-1f64;
            let v3694 = -2.5e-1f64;
            let v3717 = -2e0f64;
            let v3723 = -1e0f64;
            let v3783 = 6.25e-4f64;
            let v3785 = parameters[162];
            let v3802 = parameters[189];
            let v3923 = 8e-1f64;
            let v3926 = 2e-1f64;
            let v3935 = parameters[109];
            let v3955 = parameters[134];
            let v3988 = parameters[213];
            let v4138 = 1.6666666666666666e-1f64;
            let v4142 = 1.6666666666666666e-1f64;
            let v4153 = 3.4531302e-11f64;
            let v4174 = 8e1f64;
            let v4177 = parameters[17];
            let v4192 = -9.82222e11f64;
            let v4197 = 3.75956e-7f64;
            let v4218 = 8e-2f64;
            let v4236 = -7.45669e11f64;
            let v4247 = 6e-1f64;
            let v4259 = parameters[16];
            let v4287 = 1e-1f64;
            let v4300 = 2e-4f64;
            let v4324 = parameters[234];
            let v4347 = parameters[235];
            let v4356 = parameters[15];
            let v4366 = parameters[288];
            let v4368 = parameters[289];
            let v4371 = parameters[290];
            let v4378 = parameters[287];
            let v4387 = parameters[22];
            let v4397 = parameters[292];
            let v4405 = 4.112842231783458e-57f64;
            let v4410 = 1e10f64;
            let v4488 = parameters[295];
            let v4522 = parameters[286];
            let v4523 = 3.20438e-19f64;
            let v4527 = 3.20438e-19f64;
            let v4531 = 3.20438e-19f64;
            let v4535 = 3.20438e-19f64;
            let v4539 = 3.20438e-19f64;
            let v4542 = 3.20438e-19f64;
            if v2 != 0.0 {
                let v7 = if (if v3 == v0 { 1.0 } else { 0.0 }) != 0.0 || (if v5 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v7 != 0.0 {
                } else {
                }
            } else {
            }
            let v9 = if v8 == v1 { 1.0 } else { 0.0 };
            let v1872: f64;
            if v9 != 0.0 {
                v1872 = v1;
            } else {
                v1872 = v10;
            }
            let v12 = if v11 == v1 { 1.0 } else { 0.0 };
            let v2233: f64;
            if v12 != 0.0 {
                v2233 = v1;
            } else {
                v2233 = v13;
            }
            let v16 = v14 * v15;
            let v18 = if v17 == v0 { 1.0 } else { 0.0 };
            let v25: f64;
            if v18 != 0.0 {
                let v21 = v19 / v20;
                v25 = v21;
            } else {
                v25 = v19;
            }
            let v24 = v22 + v23;
            let v27 = v25 + v26;
            let v30 = v24.powf((-v28));
            let v33 = v27.powf((-v31));
            let v34 = v30 * v33;
            let v47 = v24.powf((-v45));
            let v50 = v27.powf((-v48));
            let v51 = v47 * v50;
            let v61 = ((v52 + (v53 * v47)) + (v56 * v50)) + (v59 * v51);
            let v64 = v24 - (v62 * (((v35 + (v36 * v30)) + (v39 * v33)) + (v42 * v34)));
            let v65 = if v64 <= v0 { 1.0 } else { 0.0 };
            if v65 != 0.0 {
            } else {
                let v67 = if v64 <= v66 { 1.0 } else { 0.0 };
                if v67 != 0.0 {
                } else {
                }
            }
            let v69 = v27 - (v62 * v61);
            let v70 = if v69 <= v0 { 1.0 } else { 0.0 };
            if v70 != 0.0 {
            } else {
                let v71 = if v69 <= v66 { 1.0 } else { 0.0 };
                if v71 != 0.0 {
                } else {
                }
            }
            let v91 = ((v82 + (v83 * v47)) + (v86 * v50)) + (v89 * v51);
            let v93 = v24 - (v62 * (((v72 + (v73 * v30)) + (v76 * v33)) + (v79 * v34)));
            let v94 = if v93 <= v0 { 1.0 } else { 0.0 };
            if v94 != 0.0 {
            } else {
                let v95 = if v93 <= v66 { 1.0 } else { 0.0 };
                if v95 != 0.0 {
                } else {
                }
            }
            let v97 = v27 - (v62 * v91);
            let v98 = if v97 <= v0 { 1.0 } else { 0.0 };
            if v98 != 0.0 {
            } else {
                let v99 = if v97 <= v66 { 1.0 } else { 0.0 };
                if v99 != 0.0 {
                } else {
                }
            }
            let v101 = v100 / v64;
            let v102 = v100 / v69;
            let v103 = v101 * v102;
            let v113 = ((v104 + (v105 * v101)) + (v108 * v102)) + (v111 * v103);
            let v123 = ((v114 + (v115 * v101)) + (v118 * v102)) + (v121 * v103);
            let v133 = ((v124 + (v125 * v101)) + (v128 * v102)) + (v131 * v103);
            let v143 = ((v134 + (v135 * v101)) + (v138 * v102)) + (v141 * v103);
            let v153 = ((v144 + (v145 * v101)) + (v148 * v102)) + (v151 * v103);
            let v163 = ((v154 + (v155 * v101)) + (v158 * v102)) + (v161 * v103);
            let v173 = ((v164 + (v165 * v101)) + (v168 * v102)) + (v171 * v103);
            let v183 = ((v174 + (v175 * v101)) + (v178 * v102)) + (v181 * v103);
            let v193 = ((v184 + (v185 * v101)) + (v188 * v102)) + (v191 * v103);
            let v203 = ((v194 + (v195 * v101)) + (v198 * v102)) + (v201 * v103);
            let v213 = ((v204 + (v205 * v101)) + (v208 * v102)) + (v211 * v103);
            let v223 = ((v214 + (v215 * v101)) + (v218 * v102)) + (v221 * v103);
            let v233 = ((v224 + (v225 * v101)) + (v228 * v102)) + (v231 * v103);
            let v243 = ((v234 + (v235 * v101)) + (v238 * v102)) + (v241 * v103);
            let v253 = ((v244 + (v245 * v101)) + (v248 * v102)) + (v251 * v103);
            let v263 = ((v254 + (v255 * v101)) + (v258 * v102)) + (v261 * v103);
            let v273 = ((v264 + (v265 * v101)) + (v268 * v102)) + (v271 * v103);
            let v283 = ((v274 + (v275 * v101)) + (v278 * v102)) + (v281 * v103);
            let v284 = if v253 < v0 { 1.0 } else { 0.0 };
            let v2203: f64;
            if v284 != 0.0 {
                v2203 = v0;
            } else {
                let v285 = if v253 > v1 { 1.0 } else { 0.0 };
                let v2204: f64;
                if v285 != 0.0 {
                    v2204 = v1;
                } else {
                    v2204 = v253;
                }
                v2203 = v2204;
            }
            let v295 = ((v286 + (v287 * v101)) + (v290 * v102)) + (v293 * v103);
            let v305 = ((v296 + (v297 * v101)) + (v300 * v102)) + (v303 * v103);
            let v315 = ((v306 + (v307 * v101)) + (v310 * v102)) + (v313 * v103);
            let v325 = ((v316 + (v317 * v101)) + (v320 * v102)) + (v323 * v103);
            let v335 = ((v326 + (v327 * v101)) + (v330 * v102)) + (v333 * v103);
            let v345 = ((v336 + (v337 * v101)) + (v340 * v102)) + (v343 * v103);
            let v355 = ((v346 + (v347 * v101)) + (v350 * v102)) + (v353 * v103);
            let v356 = if v355 < v0 { 1.0 } else { 0.0 };
            let v2216: f64;
            if v356 != 0.0 {
                v2216 = v0;
            } else {
                let v357 = if v355 > v1 { 1.0 } else { 0.0 };
                let v2217: f64;
                if v357 != 0.0 {
                    v2217 = v1;
                } else {
                    v2217 = v355;
                }
                v2216 = v2217;
            }
            let v367 = ((v358 + (v359 * v101)) + (v362 * v102)) + (v365 * v103);
            let v377 = ((v368 + (v369 * v101)) + (v372 * v102)) + (v375 * v103);
            let v387 = ((v378 + (v379 * v101)) + (v382 * v102)) + (v385 * v103);
            let v397 = ((v388 + (v389 * v101)) + (v392 * v102)) + (v395 * v103);
            let v407 = ((v398 + (v399 * v101)) + (v402 * v102)) + (v405 * v103);
            let v417 = ((v408 + (v409 * v101)) + (v412 * v102)) + (v415 * v103);
            let v427 = ((v418 + (v419 * v101)) + (v422 * v102)) + (v425 * v103);
            let v437 = ((v428 + (v429 * v101)) + (v432 * v102)) + (v435 * v103);
            let v447 = ((v438 + (v439 * v101)) + (v442 * v102)) + (v445 * v103);
            let v457 = ((v448 + (v449 * v101)) + (v452 * v102)) + (v455 * v103);
            let v467 = ((v458 + (v459 * v101)) + (v462 * v102)) + (v465 * v103);
            let v477 = ((v468 + (v469 * v101)) + (v472 * v102)) + (v475 * v103);
            let v487 = ((v478 + (v479 * v101)) + (v482 * v102)) + (v485 * v103);
            let v497 = ((v488 + (v489 * v101)) + (v492 * v102)) + (v495 * v103);
            let v507 = ((v498 + (v499 * v101)) + (v502 * v102)) + (v505 * v103);
            let v517 = ((v508 + (v509 * v101)) + (v512 * v102)) + (v515 * v103);
            let v527 = ((v518 + (v519 * v101)) + (v522 * v102)) + (v525 * v103);
            let v537 = ((v528 + (v529 * v101)) + (v532 * v102)) + (v535 * v103);
            let v547 = ((v538 + (v539 * v101)) + (v542 * v102)) + (v545 * v103);
            let v557 = ((v548 + (v549 * v101)) + (v552 * v102)) + (v555 * v103);
            let v567 = ((v558 + (v559 * v101)) + (v562 * v102)) + (v565 * v103);
            let v577 = ((v568 + (v569 * v101)) + (v572 * v102)) + (v575 * v103);
            let v587 = ((v578 + (v579 * v101)) + (v582 * v102)) + (v585 * v103);
            let v597 = ((v588 + (v589 * v101)) + (v592 * v102)) + (v595 * v103);
            let v607 = ((v598 + (v599 * v101)) + (v602 * v102)) + (v605 * v103);
            let v617 = ((v608 + (v609 * v101)) + (v612 * v102)) + (v615 * v103);
            let v627 = ((v618 + (v619 * v101)) + (v622 * v102)) + (v625 * v103);
            let v637 = ((v628 + (v629 * v101)) + (v632 * v102)) + (v635 * v103);
            let v647 = ((v638 + (v639 * v101)) + (v642 * v102)) + (v645 * v103);
            let v657 = ((v648 + (v649 * v101)) + (v652 * v102)) + (v655 * v103);
            let v667 = ((v658 + (v659 * v101)) + (v662 * v102)) + (v665 * v103);
            let v677 = ((v668 + (v669 * v101)) + (v672 * v102)) + (v675 * v103);
            let v687 = ((v678 + (v679 * v101)) + (v682 * v102)) + (v685 * v103);
            let v697 = ((v688 + (v689 * v101)) + (v692 * v102)) + (v695 * v103);
            let v707 = ((v698 + (v699 * v101)) + (v702 * v102)) + (v705 * v103);
            let v717 = ((v708 + (v709 * v101)) + (v712 * v102)) + (v715 * v103);
            let v727 = ((v718 + (v719 * v101)) + (v722 * v102)) + (v725 * v103);
            let v737 = ((v728 + (v729 * v101)) + (v732 * v102)) + (v735 * v103);
            let v747 = ((v738 + (v739 * v101)) + (v742 * v102)) + (v745 * v103);
            let v757 = ((v748 + (v749 * v101)) + (v752 * v102)) + (v755 * v103);
            let v767 = ((v758 + (v759 * v101)) + (v762 * v102)) + (v765 * v103);
            let v777 = ((v768 + (v769 * v101)) + (v772 * v102)) + (v775 * v103);
            let v787 = ((v778 + (v779 * v101)) + (v782 * v102)) + (v785 * v103);
            let v797 = ((v788 + (v789 * v101)) + (v792 * v102)) + (v795 * v103);
            let v807 = ((v798 + (v799 * v101)) + (v802 * v102)) + (v805 * v103);
            let v817 = ((v808 + (v809 * v101)) + (v812 * v102)) + (v815 * v103);
            let v827 = ((v818 + (v819 * v101)) + (v822 * v102)) + (v825 * v103);
            let v837 = ((v828 + (v829 * v101)) + (v832 * v102)) + (v835 * v103);
            let v847 = ((v838 + (v839 * v101)) + (v842 * v102)) + (v845 * v103);
            let v857 = ((v848 + (v849 * v101)) + (v852 * v102)) + (v855 * v103);
            let v867 = ((v858 + (v859 * v101)) + (v862 * v102)) + (v865 * v103);
            let v877 = ((v868 + (v869 * v101)) + (v872 * v102)) + (v875 * v103);
            let v887 = ((v878 + (v879 * v101)) + (v882 * v102)) + (v885 * v103);
            let v897 = ((v888 + (v889 * v101)) + (v892 * v102)) + (v895 * v103);
            let v907 = ((v898 + (v899 * v101)) + (v902 * v102)) + (v905 * v103);
            let v917 = ((v908 + (v909 * v101)) + (v912 * v102)) + (v915 * v103);
            let v927 = ((v918 + (v919 * v101)) + (v922 * v102)) + (v925 * v103);
            let v937 = ((v928 + (v929 * v101)) + (v932 * v102)) + (v935 * v103);
            let v947 = ((v938 + (v939 * v101)) + (v942 * v102)) + (v945 * v103);
            let v957 = ((v948 + (v949 * v101)) + (v952 * v102)) + (v955 * v103);
            let v967 = ((v958 + (v959 * v101)) + (v962 * v102)) + (v965 * v103);
            let v977 = ((v968 + (v969 * v101)) + (v972 * v102)) + (v975 * v103);
            let v987 = ((v978 + (v979 * v101)) + (v982 * v102)) + (v985 * v103);
            let v997 = ((v988 + (v989 * v101)) + (v992 * v102)) + (v995 * v103);
            let v1007 = ((v998 + (v999 * v101)) + (v1002 * v102)) + (v1005 * v103);
            let v1017 = ((v1008 + (v1009 * v101)) + (v1012 * v102)) + (v1015 * v103);
            let v1027 = ((v1018 + (v1019 * v101)) + (v1022 * v102)) + (v1025 * v103);
            let v1037 = ((v1028 + (v101 * v1029)) + (v102 * v1032)) + (v103 * v1035);
            let v1047 = ((v1038 + (v101 * v1039)) + (v102 * v1042)) + (v103 * v1045);
            let v1057 = ((v1048 + (v101 * v1049)) + (v102 * v1052)) + (v103 * v1055);
            let v1067 = ((v1058 + (v101 * v1059)) + (v102 * v1062)) + (v103 * v1065);
            let v1077 = ((v1068 + (v101 * v1069)) + (v102 * v1072)) + (v103 * v1075);
            let v1087 = ((v1078 + (v101 * v1079)) + (v102 * v1082)) + (v103 * v1085);
            let v1097 = ((v1088 + (v101 * v1089)) + (v102 * v1092)) + (v103 * v1095);
            let v1107 = ((v1098 + (v101 * v1099)) + (v102 * v1102)) + (v103 * v1105);
            let v1117 = ((v1108 + (v101 * v1109)) + (v102 * v1112)) + (v103 * v1115);
            let v1127 = ((v1118 + (v1119 * v101)) + (v1122 * v102)) + (v1125 * v103);
            let v1137 = ((v1128 + (v1129 * v101)) + (v1132 * v102)) + (v1135 * v103);
            let v1147 = ((v1138 + (v1139 * v101)) + (v1142 * v102)) + (v1145 * v103);
            let v1157 = ((v1148 + (v1149 * v101)) + (v1152 * v102)) + (v1155 * v103);
            let v1167 = ((v1158 + (v1159 * v101)) + (v1162 * v102)) + (v1165 * v103);
            let v1177 = ((v1168 + (v1169 * v101)) + (v1172 * v102)) + (v1175 * v103);
            let v1187 = ((v1178 + (v1179 * v101)) + (v1182 * v102)) + (v1185 * v103);
            let v1197 = ((v1188 + (v1189 * v101)) + (v1192 * v102)) + (v1195 * v103);
            let v1207 = ((v1198 + (v1199 * v101)) + (v1202 * v102)) + (v1205 * v103);
            let v1217 = ((v1208 + (v1209 * v101)) + (v1212 * v102)) + (v1215 * v103);
            let v1227 = ((v1218 + (v1219 * v101)) + (v1222 * v102)) + (v1225 * v103);
            let v1237 = ((v1228 + (v1229 * v101)) + (v1232 * v102)) + (v1235 * v103);
            let v1247 = ((v1238 + (v1239 * v101)) + (v1242 * v102)) + (v1245 * v103);
            let v1257 = ((v1248 + (v1249 * v101)) + (v1252 * v102)) + (v1255 * v103);
            let v1267 = ((v1258 + (v1259 * v101)) + (v1262 * v102)) + (v1265 * v103);
            let v1277 = ((v1268 + (v1269 * v101)) + (v1272 * v102)) + (v1275 * v103);
            let v1287 = ((v1278 + (v1279 * v101)) + (v1282 * v102)) + (v1285 * v103);
            let v1297 = ((v1288 + (v1289 * v101)) + (v1292 * v102)) + (v1295 * v103);
            let v1307 = ((v1298 + (v1299 * v101)) + (v1302 * v102)) + (v1305 * v103);
            let v1317 = ((v1308 + (v1309 * v101)) + (v1312 * v102)) + (v1315 * v103);
            let v1327 = ((v1318 + (v1319 * v101)) + (v1322 * v102)) + (v1325 * v103);
            let v1337 = ((v1328 + (v1329 * v101)) + (v1332 * v102)) + (v1335 * v103);
            let v1347 = ((v1338 + (v1339 * v101)) + (v1342 * v102)) + (v1345 * v103);
            let v1357 = ((v1348 + (v1349 * v101)) + (v1352 * v102)) + (v1355 * v103);
            let v1367 = ((v1358 + (v1359 * v101)) + (v1362 * v102)) + (v1365 * v103);
            let v1377 = ((v1368 + (v1369 * v101)) + (v1372 * v102)) + (v1375 * v103);
            let v1387 = ((v1378 + (v1379 * v101)) + (v1382 * v102)) + (v1385 * v103);
            let v1397 = ((v1388 + (v1389 * v101)) + (v1392 * v102)) + (v1395 * v103);
            let v1407 = ((v1398 + (v1399 * v101)) + (v1402 * v102)) + (v1405 * v103);
            let v1417 = ((v1408 + (v1409 * v101)) + (v1412 * v102)) + (v1415 * v103);
            let v1427 = ((v1418 + (v1419 * v101)) + (v1422 * v102)) + (v1425 * v103);
            let v1437 = ((v1428 + (v1429 * v101)) + (v1432 * v102)) + (v1435 * v103);
            let v1439 = if v1438 == v1 { 1.0 } else { 0.0 };
            let v1442 = if v1439 != 0.0 && (if v1440 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v4493: f64;
            if v1442 != 0.0 {
                let v1451 = ((v1440 + (v1443 * v101)) + (v1446 * v102)) + (v1449 * v103);
                v4493 = v1451;
            } else {
                v4493 = v0;
            }
            let v1455 = v1453 / v1454;
            let v1458 = v1456 / v1457;
            let v1461 = v1459 / v1460;
            let v1463 = v16 / v1462;
            let v1464 = v14 / v1452;
            let v1466 = if v1465 == 0.0 { 1.0 } else { 0.0 };
            let v4154: f64;
            if v1466 != 0.0 {
                let v1471 = ((v1454 * v1467) / v1452) - v1470;
                v4154 = v1471;
            } else {
                v4154 = v1457;
            }
            let v1473 = if v1472 > v0 { 1.0 } else { 0.0 };
            let v1655: f64;
            if v1473 != 0.0 {
                let v1478 = v667 * (v1 - (v1387 * (v64.powf((-v1472)))));
                v1655 = v1478;
            } else {
                let v1480 = v667 * (v1 - v1387);
                v1655 = v1480;
            }
            let v1482 = -v64;
            let v1487 = v677 + (v1481 * (rspice_limited_exp((v1482 / v1483))));
            let v1493 = v687 + (v1488 * (rspice_limited_exp((v1482 / v1489))));
            let v1500 = v1494 + (v1495 * (rspice_limited_exp((v1482 / v1496))));
            let v1506 = v697 + (v1501 * (rspice_limited_exp((v1482 / v1502))));
            let v1512 = v717 + (v1507 * (rspice_limited_exp((v1482 / v1508))));
            let v1518 = v727 + (v1513 * (rspice_limited_exp((v1482 / v1514))));
            let v1520 = if v1519 > v0 { 1.0 } else { 0.0 };
            let v3071: f64;
            if v1520 != 0.0 {
                let v1525 = v797 * (v1 - (v1397 * (v64.powf((-v1519)))));
                v3071 = v1525;
            } else {
                let v1527 = v797 * (v1 - v1397);
                v3071 = v1527;
            }
            let v1533 = v807 + (v1528 * (rspice_limited_exp((v1482 / v1529))));
            let v1539 = v817 + (v1534 * (rspice_limited_exp((v1482 / v1535))));
            let v1546 = v1540 + (v1541 * (rspice_limited_exp((v1482 / v1542))));
            let v1552 = v827 + (v1547 * (rspice_limited_exp((v1482 / v1548))));
            let v1558 = v847 + (v1553 * (rspice_limited_exp((v1482 / v1554))));
            let v1564 = v857 + (v1559 * (rspice_limited_exp((v1482 / v1560))));
            let v1566 = if v1565 == v1 { 1.0 } else { 0.0 };
            let v1667: f64;
            let v1671: f64;
            let v1675: f64;
            if v1566 != 0.0 {
                let v1572 = v133 + (v1567 * (rspice_limited_exp((v1482 / v1568))));
                let v1578 = v123 + (v1573 * (rspice_limited_exp((v1482 / v1574))));
                v1667 = v113;
                v1671 = v1572;
                v1675 = v1578;
            } else {
                let v1584 = v113 + (v1579 * (rspice_limited_exp((v1482 / v1580))));
                v1667 = v1584;
                v1671 = v133;
                v1675 = v123;
            }
            let v1590 = v947 + (v1585 * (rspice_limited_exp((v1482 / v1586))));
            let v1598 = v557 + (v1591 * ((v64 * v1592).powf((-v1594))));
            let v1604 = v627 + (v1599 * (rspice_limited_exp((v1482 / v1600))));
            let v1610 = v637 + (v1605 * (rspice_limited_exp((v1482 / v1606))));
            let v1616 = v647 + (v1611 * (rspice_limited_exp((v1482 / v1612))));
            let v1622 = v1317 + (v1617 * (rspice_limited_exp((v1482 / v1618))));
            let v1628 = v1327 + (v1623 * (rspice_limited_exp((v1482 / v1624))));
            let v1634 = v1337 + (v1629 * (rspice_limited_exp((v1482 / v1630))));
            let v1640 = v1347 + (v1635 * (rspice_limited_exp((v1482 / v1636))));
            let v1647 = v1641 + (v1642 * (rspice_limited_exp((v1482 / v1643))));
            let v1654 = v1648 + (v1649 * (rspice_limited_exp((v1482 / v1650))));
            let v1656 = if v1655 < v0 { 1.0 } else { 0.0 };
            let v1890: f64;
            if v1656 != 0.0 {
                v1890 = v1657;
            } else {
                v1890 = v1655;
            }
            let v1658 = if v1487 < v0 { 1.0 } else { 0.0 };
            let v1917: f64;
            if v1658 != 0.0 {
                v1917 = v0;
            } else {
                v1917 = v1487;
            }
            let v1659 = if v1512 < v0 { 1.0 } else { 0.0 };
            let v3031: f64;
            if v1659 != 0.0 {
                v3031 = v0;
            } else {
                v3031 = v1512;
            }
            let v1660 = if v1506 < v0 { 1.0 } else { 0.0 };
            let v1928: f64;
            if v1660 != 0.0 {
                v1928 = v0;
            } else {
                v1928 = v1506;
            }
            let v1661 = if v707 < v0 { 1.0 } else { 0.0 };
            let v1931: f64;
            if v1661 != 0.0 {
                v1931 = v0;
            } else {
                v1931 = v707;
            }
            let v1662 = if v1628 < v0 { 1.0 } else { 0.0 };
            let v2005: f64;
            if v1662 != 0.0 {
                v2005 = v0;
            } else {
                v2005 = v1628;
            }
            let v1663 = if v437 <= v0 { 1.0 } else { 0.0 };
            if v1663 != 0.0 {
            } else {
            }
            let v1664 = if v487 <= v0 { 1.0 } else { 0.0 };
            if v1664 != 0.0 {
            } else {
            }
            let v1666 = if v1665 < v0 { 1.0 } else { 0.0 };
            let v1736: f64;
            if v1666 != 0.0 {
                v1736 = v0;
            } else {
                v1736 = v1665;
            }
            let v1668 = if v1667 < v0 { 1.0 } else { 0.0 };
            let v1738: f64;
            if v1668 != 0.0 {
                v1738 = v0;
            } else {
                v1738 = v1667;
            }
            let v1670 = if v1669 < v0 { 1.0 } else { 0.0 };
            let v1728: f64;
            if v1670 != 0.0 {
                v1728 = v0;
            } else {
                v1728 = v1669;
            }
            let v1672 = if v1671 < v0 { 1.0 } else { 0.0 };
            let v1732: f64;
            if v1672 != 0.0 {
                v1732 = v0;
            } else {
                v1732 = v1671;
            }
            let v1674 = if v1673 < v0 { 1.0 } else { 0.0 };
            let v1730: f64;
            if v1674 != 0.0 {
                v1730 = v0;
            } else {
                v1730 = v1673;
            }
            let v1676 = if v1675 < v0 { 1.0 } else { 0.0 };
            let v1734: f64;
            if v1676 != 0.0 {
                v1734 = v0;
            } else {
                v1734 = v1675;
            }
            let v1677 = if v143 < v0 { 1.0 } else { 0.0 };
            let v3088: f64;
            if v1677 != 0.0 {
                v3088 = v0;
            } else {
                v3088 = v143;
            }
            let v1678 = if v977 < v0 { 1.0 } else { 0.0 };
            if v1678 != 0.0 {
            } else {
            }
            let v1679 = if v987 < v0 { 1.0 } else { 0.0 };
            if v1679 != 0.0 {
            } else {
            }
            let v1680 = if v967 <= v0 { 1.0 } else { 0.0 };
            if v1680 != 0.0 {
            } else {
            }
            let v1681 = if v1598 < v62 { 1.0 } else { 0.0 };
            let v1689: f64;
            if v1681 != 0.0 {
                v1689 = v62;
            } else {
                v1689 = v1598;
            }
            let v1685 = ((v1 + (v547 / v64)).sqrt()) - v1;
            let v1688 = v1462 + (v1464 * (v1454 + v1460));
            let v1690 = v1 / v1689;
            let v1693 = v1692 * v787;
            let v1694 = v1692 * v867;
            let v1695 = if v8 != v1 { 1.0 } else { 0.0 };
            let v3002: f64;
            let v3012: f64;
            let v3903: f64;
            if v1695 != 0.0 {
                let v1698 = v1697 * v787;
                let v1701 = v1700 * v867;
                v3002 = v1698;
                v3012 = v1701;
                v3903 = v1699;
            } else {
                v3002 = v1693;
                v3012 = v1694;
                v3903 = v1692;
            }
            let v1703 = v1464 * v1454;
            let v1704 = v1702 / v1703;
            let v1708 = v1 / (((v69 * v1592).powf(v163)) * v20);
            let v1710 = (v1703 * v1462).sqrt();
            let v1711 = v1464 * v1460;
            let v1712 = v1702 / v1711;
            let v1715 = if v1713 >= (v64 / v62) { 1.0 } else { 0.0 };
            let v4374: f64;
            if v1715 != 0.0 {
                v4374 = v0;
            } else {
                v4374 = v1713;
            }
            let v1718 = if (if v3 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v5 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v1718 != 0.0 {
            } else {
            }
            let v1721 = v1719 * v1720;
            let v1724 = v1722 * v1723;
            let v1726 = if v1721 <= v1725 { 1.0 } else { 0.0 };
            let v3114: f64;
            if v1726 != 0.0 {
                v3114 = v1725;
            } else {
                v3114 = v1721;
            }
            let v1727 = if v1724 <= v1725 { 1.0 } else { 0.0 };
            let v3115: f64;
            if v1727 != 0.0 {
                v3115 = v1725;
            } else {
                v3115 = v1724;
            }
            let v3097: f64;
            let v3099: f64;
            let v4061: f64;
            let v4063: f64;
            let v4087: f64;
            let v4089: f64;
            if v1566 != 0.0 {
                let v1729 = if v1728 <= v0 { 1.0 } else { 0.0 };
                let v4062: f64;
                if v1729 != 0.0 {
                    v4062 = v0;
                } else {
                    v4062 = v1728;
                }
                let v1731 = if v1730 <= v0 { 1.0 } else { 0.0 };
                let v4088: f64;
                if v1731 != 0.0 {
                    v4088 = v0;
                } else {
                    v4088 = v1730;
                }
                let v1733 = if v1732 <= v0 { 1.0 } else { 0.0 };
                let v4064: f64;
                if v1733 != 0.0 {
                    v4064 = v0;
                } else {
                    v4064 = v1732;
                }
                let v1735 = if v1734 <= v0 { 1.0 } else { 0.0 };
                let v4090: f64;
                if v1735 != 0.0 {
                    v4090 = v0;
                } else {
                    v4090 = v1734;
                }
                v3097 = v1736;
                v3099 = v1738;
                v4061 = v4062;
                v4063 = v4064;
                v4087 = v4088;
                v4089 = v4090;
            } else {
                let v1737 = if v1736 <= v0 { 1.0 } else { 0.0 };
                let v3098: f64;
                if v1737 != 0.0 {
                    v3098 = v0;
                } else {
                    v3098 = v1736;
                }
                let v1739 = if v1738 <= v0 { 1.0 } else { 0.0 };
                let v3100: f64;
                if v1739 != 0.0 {
                    v3100 = v0;
                } else {
                    v3100 = v1738;
                }
                v3097 = v3098;
                v3099 = v3100;
                v4061 = v1728;
                v4063 = v1732;
                v4087 = v1730;
                v4089 = v1734;
            }
            let v1741 = if v1740 <= v0 { 1.0 } else { 0.0 };
            let v1806: f64;
            if v1741 != 0.0 {
                v1806 = v1742;
            } else {
                let v1744 = v1740 + v1743;
                v1806 = v1744;
            }
            let v1766: f64;
            if v9 != 0.0 {
                v1766 = v1745;
            } else {
                v1766 = v1746;
            }
            let v4267: f64;
            if v9 != 0.0 {
                v4267 = v1747;
            } else {
                v4267 = v1748;
            }
            let v1751 = v1749 * v1307;
            let v1759 = (rspice_limited_exp((v1297 * ((if (v1753 / v1749) >= v1691 { (v1753 / v1749) } else { v1691 }).ln())))) / (v1749 * v1749);
            let v1768 = (v69 * v1766) * ((rspice_limited_exp((v1297 * ((if (v1753 / v1751) >= v1691 { (v1753 / v1751) } else { v1691 }).ln())))) / (v1751 * v1751));
            let v1780 = (v1769 * (v1770 + ((v69 / v1696) / v1772))) / ((v1772 * v20) * (v24 - v1777));
            let v1781 = if v1780 > v1725 { 1.0 } else { 0.0 };
            let v4518: f64;
            if v1781 != 0.0 {
                let v1782 = v1 / v1780;
                v4518 = v1782;
            } else {
                let v1785 = if v1784 != v0 { 1.0 } else { 0.0 };
                if v1785 != 0.0 {
                } else {
                }
                v4518 = v1783;
            }
            let v1794: f64;
            if v1718 != 0.0 {
                let v1790 = (v1786 + v1787) + v1789;
                v1794 = v1790;
            } else {
                let v1791 = v1786 + v1789;
                v1794 = v1791;
            }
            let v1793 = v1792 + v1743;
            let v1795 = if v1794 > v1793 { 1.0 } else { 0.0 };
            if v1795 != 0.0 {
            } else {
            }
            let v1797 = v1794 - v1793;
            let v1805 = v1692 * ((v1794 + v1793) - (((v1797 * v1797) + v1801).sqrt()));
            let v1807 = v1805 / v1806;
            let v1808 = v1805 - v1806;
            let v1810 = v1809 * v1805;
            let v1818 = v1811 - (((v1812 * v1805) * v1805) / (v1805 + v1815));
            let v1819 = v1805 / v1742;
            let v1826 = v62 * v1810;
            let v1830 = (v1822 * (v1819 * (v1819.sqrt()))) * (rspice_limited_exp(((v1811 / v1824) - (v1818 / v1826))));
            let v1836 = v1810 * ((if ((v193 * v203) / (v1830 * v1830)) >= v1691 { ((v193 * v203) / (v1830 * v1830)) } else { v1691 }).ln());
            let v1840 = v1810 * ((if (v203 / v1830) >= v1691 { (v203 / v1830) } else { v1691 }).ln());
            let v1841 = v1692 * v1818;
            let v1847 = v1841 - (v1810 * ((if (v1842 / v1830) >= v1691 { (v1842 / v1830) } else { v1691 }).ln()));
            let v1856 = v1841 - (v1692 * (v1847 + (((v1847 * v1847) + v1851).sqrt())));
            let v1857 = if v1842 != v0 { 1.0 } else { 0.0 };
            let v1860 = if v1857 != 0.0 && (if v1858 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v1875: f64;
            if v1860 != 0.0 {
                let v1862 = if v11 == v1861 { 1.0 } else { 0.0 };
                let v1876: f64;
                if v1862 != 0.0 {
                    let v1865 = (v183 - (v1692 * v1811)) + v1856;
                    v1876 = v1865;
                } else {
                    let v1868 = (v183 + (v1692 * v1811)) - v1856;
                    v1876 = v1868;
                }
                v1875 = v1876;
            } else {
                v1875 = v183;
            }
            let v1870 = v1818 / v62;
            let v1871 = v1869 + v1870;
            let v1874 = v1872 * (v173 - v1871);
            let v1878 = v1872 * (v1875 - v1871);
            let v1885 = v1871 - (v1872 * (if v1870 <= (v1810 * ((if (v193 / v1830) >= v1691 { (v193 / v1830) } else { v1691 }).ln())) { v1870 } else { (v1810 * ((if (v193 / v1830) >= v1691 { (v193 / v1830) } else { v1691 }).ln())) }));
            let v1887 = v1872 * (v173 - v1885);
            let v1889 = v1872 * (v1875 - v1885);
            let v1895 = v1893 + (v737 * v1808);
            let v1905 = (v1890 * (v1807.powf(v747))) * ((v1 + (v1692 * (v1895 + (((v1895 * v1895) + v1897).sqrt())))) - v1903);
            let v1909 = (v1 + (v1906 * v1808)) - v100;
            let v1916 = v1493 * (v1692 * (v1909 + (((v1909 * v1909) + v1911).sqrt())));
            let v1920 = (v1 + (v757 * v1808)) - v100;
            let v1927 = v1917 * (v1692 * (v1920 + (((v1920 * v1920) + v1922).sqrt())));
            let v1930 = v1928 * (v1807.powf(v767));
            let v1933 = v1931 * (v1807.powf(v777));
            let v1936 = (v1 + (v897 * v1808)) - v100;
            let v1942 = v1692 * (v1936 + (((v1936 * v1936) + v1938).sqrt()));
            let v1948 = v1893 - ((v877 * (v1 + (v101 * v1943))) * v1808);
            let v1949 = v1948 * v1948;
            let v1958 = v1622 * ((v1 + (v1692 * (v1948 + ((v1949 + v1950).sqrt())))) - v1956);
            let v1959 = if v1958 < v1783 { 1.0 } else { 0.0 };
            let v3123: f64;
            if v1959 != 0.0 {
                v3123 = v1783;
            } else {
                v3123 = v1958;
            }
            let v1968 = v1634 * ((v1 + (v1692 * (v1948 + ((v1949 + v1960).sqrt())))) - v1966);
            let v1969 = if v1968 < v1783 { 1.0 } else { 0.0 };
            let v3919: f64;
            if v1969 != 0.0 {
                v3919 = v1783;
            } else {
                v3919 = v1968;
            }
            let v1978 = v1640 * ((v1 + (v1692 * (v1948 + ((v1949 + v1970).sqrt())))) - v1976);
            let v1979 = if v1978 < v1783 { 1.0 } else { 0.0 };
            let v3962: f64;
            if v1979 != 0.0 {
                v3962 = v1783;
            } else {
                v3962 = v1978;
            }
            let v1982 = v1981 * v1808;
            let v2000 = v457 * (v1 + (v1980 + (v1692 * (((v1982 - v1983) - v1850) + (((((v1982 - v1986) - v1850) * ((v1982 - v1989) - v1850)) - v1993).sqrt())))));
            let v2007 = v1893 - ((v887 * (v1 + (v101 * v2001))) * v1808);
            let v2017 = v2005 * ((v1 + (v1692 * (v2007 + (((v2007 * v2007) + v2009).sqrt())))) - v2015);
            let v2022 = (v1689 * (v1 + (v2018 * v1808))) - v62;
            let v2029 = (v1692 * (v2022 + (((v2022 * v2022) + v2024).sqrt()))) + v62;
            let v2031 = v567 + (v577 * v1808);
            let v2032 = -v587;
            let v2035 = ((v597 * v1808) - v2032) - v100;
            let v2044 = v587 + (v2032 + (v1692 * (v2035 + (((v2035 * v2035) - ((v1849 * v2032) * v100)).sqrt()))));
            let v2046 = v607 + (v617 * v1808);
            let v2049 = (v1 - (v657 * v1808)) - v100;
            let v2056 = v1604 * (v1692 * (v2049 + (((v2049 * v2049) + v2051).sqrt())));
            let v2061 = v1807 - v1;
            let v2062 = (v2057 + (v2058 / v64)) * v2061;
            let v2064 = v1027 * (v1807.powf(v907));
            let v2067 = (v1 + (v917 * v1808)) - v100;
            let v2074 = v1207 * (v1692 * (v2067 + (((v2067 * v2067) + v2069).sqrt())));
            let v2077 = (v1 + (v927 * v1808)) - v100;
            let v2084 = v1187 * (v1692 * (v2077 + (((v2077 * v2077) + v2079).sqrt())));
            let v2088 = rspice_limited_exp((v937 * ((if v1807 >= v1691 { v1807 } else { v1691 }).ln())));
            let v2089 = v1768 * v2088;
            let v2093 = v1872 * (v2090 - v2091);
            let v2096 = v1872 * (v2094 - v2091);
            let v2098 = v1872 * (v2090 - v2094);
            let v2101 = v1872 * (v2099 - v2091);
            let v2103 = v1872 * (v2099 - v2094);
            let v2105 = v1872 * (v2090 - v2099);
            let v2106 = if v2096 < v0 { 1.0 } else { 0.0 };
            let v2109: f64;
            let v2118: f64;
            let v2120: f64;
            let v4099: f64;
            let v4322: f64;
            if v2106 != 0.0 {
                let v2108 = -v2096;
                v2109 = v2108;
                v2118 = v2103;
                v2120 = v2098;
                v4099 = v2101;
                v4322 = v2107;
            } else {
                v2109 = v2096;
                v2118 = v2101;
                v2120 = v2093;
                v4099 = v2103;
                v4322 = v1;
            }
            let v2115 = (((v2109 * v2109) + v2111).sqrt()) - v2114;
            let v2117 = v1692 * (v2115 - v2109);
            let v2119 = v2118 + v2117;
            let v2121 = v2120 - v1874;
            let v2122 = v2118 - v1878;
            let v2130 = (v1462 * (v1703 + (v2126 * v1462))).sqrt();
            let v2145 = v2130 + (((((v517 + (v527 * ((((v2121 * v1711) + (v2122 * (v1703 + v1462))) / v1688) + v2117))).atan()) / v2140) + v1692) * ((((v1464 * v1462) * v1454).sqrt()) - v2130));
            let v2148 = ((v437 * v64) / v2145) + v100;
            let v2150 = if v2148 < v2149 { 1.0 } else { 0.0 };
            let v2294: f64;
            if v2150 != 0.0 {
                let v2153 = v1692 / ((v2148.cosh()) - v1);
                v2294 = v2153;
            } else {
                let v2155 = rspice_limited_exp((-v2148));
                v2294 = v2155;
            }
            let v2158 = ((v487 * v64) / v2145) + v100;
            let v2159 = if v2158 < v2149 { 1.0 } else { 0.0 };
            let v2301: f64;
            if v2159 != 0.0 {
                let v2162 = v1692 / ((v2158.cosh()) - v1);
                v2301 = v2162;
            } else {
                let v2164 = rspice_limited_exp((-v2158));
                v2301 = v2164;
            }
            let v2308: f64;
            if v2159 != 0.0 {
                let v2171 = v1 / (if (v1 + (v2165 * ((v2158.cosh()) - v62))) >= v100 { (v1 + (v2165 * ((v2158.cosh()) - v62))) } else { v100 });
                v2308 = v2171;
            } else {
                let v2173 = rspice_limited_exp((-v2158));
                let v2176 = v2173 / (if (v2173 + v2165) >= v100 { (v2173 + v2165) } else { v100 });
                v2308 = v2176;
            }
            let v2179 = ((v967 * v64) / v2145) + v100;
            let v2180 = if v2179 < v2149 { 1.0 } else { 0.0 };
            let v3977: f64;
            if v2180 != 0.0 {
                let v2185 = ((v1692 * v977) / ((v2179.cosh()) - v1)) + v987;
                v3977 = v2185;
            } else {
                let v2189 = (v977 * (rspice_limited_exp((-v2179)))) + v987;
                v3977 = v2189;
            }
            let v2191 = if v11 == v2190 { 1.0 } else { 0.0 };
            let v2218: f64;
            let v2219: f64;
            let v2235: f64;
            let v2251: f64;
            let v2274: f64;
            if v2191 != 0.0 {
                let v2193 = (v315 * v64) / v2145;
                let v2194 = if v2193 > v2149 { 1.0 } else { 0.0 };
                let v2200: f64;
                if v2194 != 0.0 {
                    let v2196 = (rspice_limited_exp(v2193)) / v62;
                    v2200 = v2196;
                } else {
                    let v2198 = (v2193.cosh()) - v1;
                    v2200 = v2198;
                }
                let v2202 = v325 - ((v1692 * v335) / v2200);
                v2218 = v2202;
                v2219 = v345;
                v2235 = v295;
                v2251 = v305;
                v2274 = v2203;
            } else {
                let v2206 = (v387 * v64) / v2145;
                let v2207 = if v2206 > v2149 { 1.0 } else { 0.0 };
                let v2213: f64;
                if v2207 != 0.0 {
                    let v2209 = (rspice_limited_exp(v2206)) / v62;
                    v2213 = v2209;
                } else {
                    let v2211 = (v2206.cosh()) - v1;
                    v2213 = v2211;
                }
                let v2215 = v397 - ((v1692 * v407) / v2213);
                v2218 = v2215;
                v2219 = v417;
                v2235 = v367;
                v2251 = v377;
                v2274 = v2216;
            }
            let v2220 = v2218 - v2219;
            let v2226 = v2219 + (v1692 * (v2220 + (((v2220 * v2220) + v1850).sqrt())));
            let v2230 = v62 * v1461;
            let v2232 = ((v2227 * v1842) * v16) / (v2230 * v1461);
            let v2248: f64;
            if v1857 != 0.0 {
                let v2237 = v2233 * ((v1872 * v2119) - v2235);
                let v2247 = ((v1 + ((v1692 * (v2237 + (((v2237 * v2237) + v2239).sqrt()))) / v2232)).sqrt()) - v1;
                v2248 = v2247;
            } else {
                v2248 = v0;
            }
            let v2252 = -v2251;
            let v2255 = ((-((v2232 * v2248) * v2248)) - v2252) - v1800;
            let v2269 = v1461 + v1463;
            let v2271 = ((-v1461) * v1463) / (v2269 * v1455);
            let v2279 = (v2271 * v2226) * ((v2122 - (((v1872 * v2233) * v2274) * (-(v2252 + (v1692 * (v2255 + (((v2255 * v2255) - ((v1849 * v2252) * v1800)).sqrt()))))))) - (v2265 - v2117));
            let v2285 = v1692 * (v2119 + (((v2119 * v2119) + v2281).sqrt()));
            let v2288 = (v2286 + v1840) + v447;
            let v2289 = if v2288 < v0 { 1.0 } else { 0.0 };
            let v2359: f64;
            if v2289 != 0.0 {
                v2359 = v0;
            } else {
                let v2292 = (v537 * v1685) * (v2288.sqrt());
                v2359 = v2292;
            }
            let v2303 = v2115 + v1800;
            let v2338 = v1455 + ((v1463 * v1461) / v2269);
            let v2342 = (v1810 * ((v2338 + v213) + (((v2323 * v2119) + ((v2325 * v2119) * v2119)) + (v2294 * (((v223 + (v243 * v2119)) + ((v2331 * v2119) * v2119)) + ((v233 + (v2319 * v2285)) * v2115)))))) / v2338;
            let v2345 = ((v2227 * v203) * v1462) / v1455;
            let v2364 = ((((((((-v427) * v2294) * (v1836 - v2288)) + ((((-(v2000 + (v477 * v2119))) * v2301) * (v2115 + (v467 * (v2303.sqrt())))) + ((v1647 * v2308) * (v2303.powf(v1654))))) + v2359) + (((-v497) / (v64 + v507)) * v2115)) + (v2345 * (v1 - ((v1692 * v1462) / (v1462 + v1711))))) + (v2062 + (((v2351 + (v2352 / v64)) * v2119) * v2061))) + v2279;
            let v2367 = (v2121 - v2364) + v2366;
            let v2373 = (((v2368 * v1830) * v1462) * v1462) / (v16 * v1810);
            let v2374 = v1455 / v1463;
            let v2375 = v1461 / v1463;
            let v2376 = v2373.ln();
            let v2379 = v2378 - v2376;
            let v2380 = v2374 * v2374;
            let v2384 = v2374 / (((v2375 * v2374) + v2375) + v2374);
            let v2385 = v2367 / v2342;
            let v2386 = v2122 - v2364;
            let v2387 = v2386 + v2366;
            let v2388 = v2387 / v2342;
            let v2389 = v2385 - v2379;
            let v2394 = ((((v2380 * v2389) * v2389) + v2377).ln()) - v2376;
            let v2397 = v1 + v2375;
            let v2398 = (v2394 + (v2375 * v2388)) / v2397;
            let v2403 = if (if (v2388 + (v2384 * (v2385 - v2388))) <= v2394 { (v2388 + (v2384 * (v2385 - v2388))) } else { v2394 }) <= v2379 { (if (v2388 + (v2384 * (v2385 - v2388))) <= v2394 { (v2388 + (v2384 * (v2385 - v2388))) } else { v2394 }) } else { v2379 };
            let v2406 = v1 + v2374;
            let v2407 = (v2403 + (v2374 * v2385)) / v2406;
            let v2408 = v2388 - v2398;
            let v2409 = v2375 * v2375;
            let v2414 = ((v2409 * v2408) * v2408) - (v2373 * (v2398.exp()));
            let v2415 = if v2414 < v0 { 1.0 } else { 0.0 };
            let v2480: f64;
            if v2415 != 0.0 {
                let v2417 = (v2388 - v2403) * v2375;
                let v2418 = v2149 * v2374;
                let v2419 = v2418 + v2417;
                let v2420 = v2418 * v2417;
                let v2423 = (v2421 * v2419) + v1;
                let v2427 = ((v2419 * v2424) + v2420) + v2377;
                let v2454 = if ((((-v2427) + ((((v2433 * v2423) * ((v2428 * v2419) + (v2377 * v2420))) + (v2427 * v2427)).sqrt())) / (v62 * v2423)) * (v1 - (((-((v2385 - (((v2379 * v2406) - v2403) / v2374)) + v62)) / v2448).exp()))) <= v2453 { ((((-v2427) + ((((v2433 * v2423) * ((v2428 * v2419) + (v2377 * v2420))) + (v2427 * v2427)).sqrt())) / (v62 * v2423)) * (v1 - (((-((v2385 - (((v2379 * v2406) - v2403) / v2374)) + v62)) / v2448).exp()))) } else { v2453 };
                v2480 = v2454;
            } else {
                v2480 = v2414;
            }
            let v2455 = if v2385 >= v2379 { v2385 } else { v2379 };
            let v2456 = v2455 - v2379;
            let v2462 = v2379 * v2406;
            let v2465 = ((v2462 - v2403) / v2374) - v2379;
            let v2472 = (((((v2380 * v2456) * v2456) + v2377).ln()) - v2376) - ((((((v2380 * v2465) * v2465) + v2377).ln()) - v2376) - v2379);
            let v2473 = v2455 - v2472;
            let v2474 = -v2373;
            let v2476 = v2474 * (v2472.exp());
            let v2477 = v2380 * v2473;
            let v2487 = v2472 + ((-(((v2477 * v2473) + v2476) - v2480)) / ((v2483 * v2477) + v2476));
            let v2488 = v2455 - v2487;
            let v2489 = v2380 * v2488;
            let v2491 = (v2489 * v2488) - v2480;
            let v2492 = v1 / v2491;
            let v2501 = v1 / (((v2497 * v2489) * v2492) - v1);
            let v2507 = v62 * v2380;
            let v2510 = ((((v2491.abs()).ln()) - v2376) - v2487) * v2501;
            let v2521 = v2487 + (if (if ((-v2510) - ((((v1692 * v2510) * v2510) * (((((v2502 * v2489) * v2489) * v2492) * v2492) + (v2507 * v2492))) * v2501)) >= v2518 { ((-v2510) - ((((v1692 * v2510) * v2510) * (((((v2502 * v2489) * v2489) * v2492) * v2492) + (v2507 * v2492))) * v2501)) } else { v2518 }) <= v2517 { (if ((-v2510) - ((((v1692 * v2510) * v2510) * (((((v2502 * v2489) * v2489) * v2492) * v2492) + (v2507 * v2492))) * v2501)) >= v2518 { ((-v2510) - ((((v1692 * v2510) * v2510) * (((((v2502 * v2489) * v2489) * v2492) * v2492) + (v2507 * v2492))) * v2501)) } else { v2518 }) } else { v2517 });
            let v2522 = v2455 - v2521;
            let v2523 = v2380 * v2522;
            let v2525 = (v2523 * v2522) - v2480;
            let v2526 = v1 / v2525;
            let v2535 = v1 / (((v2531 * v2523) * v2526) - v1);
            let v2543 = ((((v2525.abs()).ln()) - v2376) - v2521) * v2535;
            let v2554 = v2379 - v1849;
            let v2555 = if (v2521 + (if (if ((-v2543) - ((((v1692 * v2543) * v2543) * (((((v2536 * v2523) * v2523) * v2526) * v2526) + (v2507 * v2526))) * v2535)) >= v2550 { ((-v2543) - ((((v1692 * v2543) * v2543) * (((((v2536 * v2523) * v2523) * v2526) * v2526) + (v2507 * v2526))) * v2535)) } else { v2550 }) <= v2517 { (if ((-v2543) - ((((v1692 * v2543) * v2543) * (((((v2536 * v2523) * v2523) * v2526) * v2526) + (v2507 * v2526))) * v2535)) >= v2550 { ((-v2543) - ((((v1692 * v2543) * v2543) * (((((v2536 * v2523) * v2523) * v2526) * v2526) + (v2507 * v2526))) * v2535)) } else { v2550 }) } else { v2517 })) >= v2554 { (v2521 + (if (if ((-v2543) - ((((v1692 * v2543) * v2543) * (((((v2536 * v2523) * v2523) * v2526) * v2526) + (v2507 * v2526))) * v2535)) >= v2550 { ((-v2543) - ((((v1692 * v2543) * v2543) * (((((v2536 * v2523) * v2523) * v2526) * v2526) + (v2507 * v2526))) * v2535)) } else { v2550 }) <= v2517 { (if ((-v2543) - ((((v1692 * v2543) * v2543) * (((((v2536 * v2523) * v2523) * v2526) * v2526) + (v2507 * v2526))) * v2535)) >= v2550 { ((-v2543) - ((((v1692 * v2543) * v2543) * (((((v2536 * v2523) * v2523) * v2526) * v2526) + (v2507 * v2526))) * v2535)) } else { v2550 }) } else { v2517 })) } else { v2554 };
            let v2563 = if (v2407 - ((v1 + ((v2407 - (v2556 * v2555)).exp())).ln())) <= v2555 { (v2407 - ((v1 + ((v2407 - (v2556 * v2555)).exp())).ln())) } else { v2555 };
            let v2564 = v2385 - v2563;
            let v2565 = v2374 * v2564;
            let v2567 = v2474 * (v2563.exp());
            let v2569 = (v2565 * v2565) + v2567;
            let v2570 = if v2569 < v0 { 1.0 } else { 0.0 };
            let v2596: f64;
            let v2597: f64;
            let v2603: f64;
            let v2615: f64;
            let v2621: f64;
            if v2570 != 0.0 {
                let v2572 = (-v2569).sqrt();
                let v2573 = v1692 * v2572;
                let v2575 = v1 / (v2573.sin());
                let v2576 = v2575 * v2575;
                let v2578 = (v2573.cos()) * v2575;
                let v2581 = (v2579 * v2578) / v2572;
                let v2583 = (v1799 * v2576) + v2581;
                v2596 = v2572;
                v2597 = v2578;
                v2603 = v2576;
                v2615 = v2581;
                v2621 = v2583;
            } else {
                let v2584 = v2569.sqrt();
                let v2587 = v1 / ((v1692 * v2584).sinh());
                let v2588 = v2587 * v2587;
                let v2590 = (v1 + v2588).sqrt();
                let v2592 = (v1692 * v2590) / v2584;
                let v2595 = (v2593 * v2588) + v2592;
                v2596 = v2584;
                v2597 = v2590;
                v2603 = v2588;
                v2615 = v2592;
                v2621 = v2595;
            }
            let v2599 = v2565 + (v2596 * v2597);
            let v2600 = v1 / v2599;
            let v2601 = v2388 - v2385;
            let v2609 = (v2601 + v2564) - (((((v2569 * v2603) * v2600) * v2600).abs()).ln());
            let v2620 = ((v2617 * v2374) * v2565) + v2567;
            let v2622 = v2621 * v2620;
            let v2624 = -v2374;
            let v2644 = v2563 + ((-(v2567 + (v2599 * ((v2375 * v2609) + v2565)))) / (((v2567 - (v2374 * (v2565 + v2599))) + (v2565 * v2622)) + (v2375 * ((((v2623 + (v62 * ((v2624 + v2622) * v2600))) - (((v1 / v2569) - v2615) * v2620)) * v2599) + (v2609 * (v2622 - v2374))))));
            let v2645 = v2385 - v2644;
            let v2646 = v2374 * v2645;
            let v2648 = v2474 * (v2644.exp());
            let v2650 = (v2646 * v2646) + v2648;
            let v2651 = if v2650 < v0 { 1.0 } else { 0.0 };
            let v2677: f64;
            let v2678: f64;
            let v2683: f64;
            let v2695: f64;
            let v2701: f64;
            if v2651 != 0.0 {
                let v2653 = (-v2650).sqrt();
                let v2654 = v1692 * v2653;
                let v2656 = v1 / (v2654.sin());
                let v2657 = v2656 * v2656;
                let v2659 = (v2654.cos()) * v2656;
                let v2662 = (v2660 * v2659) / v2653;
                let v2664 = (v1799 * v2657) + v2662;
                v2677 = v2653;
                v2678 = v2659;
                v2683 = v2657;
                v2695 = v2662;
                v2701 = v2664;
            } else {
                let v2665 = v2650.sqrt();
                let v2668 = v1 / ((v1692 * v2665).sinh());
                let v2669 = v2668 * v2668;
                let v2671 = (v1 + v2669).sqrt();
                let v2673 = (v1692 * v2671) / v2665;
                let v2676 = (v2674 * v2669) + v2673;
                v2677 = v2665;
                v2678 = v2671;
                v2683 = v2669;
                v2695 = v2673;
                v2701 = v2676;
            }
            let v2680 = v2646 + (v2677 * v2678);
            let v2681 = v1 / v2680;
            let v2689 = (v2601 + v2645) - (((((v2650 * v2683) * v2681) * v2681).abs()).ln());
            let v2700 = ((v2697 * v2374) * v2646) + v2648;
            let v2702 = v2701 * v2700;
            let v2723 = v2644 + ((-(v2648 + (v2680 * ((v2375 * v2689) + v2646)))) / (((v2648 - (v2374 * (v2646 + v2680))) + (v2646 * v2702)) + (v2375 * ((((v2703 + (v62 * ((v2624 + v2702) * v2681))) - (((v1 / v2650) - v2695) * v2700)) * v2680) + (v2689 * (v2702 - v2374))))));
            let v2724 = v2385 - v2723;
            let v2725 = v2374 * v2724;
            let v2727 = v2474 * (v2723.exp());
            let v2729 = (v2725 * v2725) + v2727;
            let v2730 = if v2729 < v0 { 1.0 } else { 0.0 };
            let v2756: f64;
            let v2757: f64;
            let v2762: f64;
            let v2774: f64;
            let v2780: f64;
            if v2730 != 0.0 {
                let v2732 = (-v2729).sqrt();
                let v2733 = v1692 * v2732;
                let v2735 = v1 / (v2733.sin());
                let v2736 = v2735 * v2735;
                let v2738 = (v2733.cos()) * v2735;
                let v2741 = (v2739 * v2738) / v2732;
                let v2743 = (v1799 * v2736) + v2741;
                v2756 = v2732;
                v2757 = v2738;
                v2762 = v2736;
                v2774 = v2741;
                v2780 = v2743;
            } else {
                let v2744 = v2729.sqrt();
                let v2747 = v1 / ((v1692 * v2744).sinh());
                let v2748 = v2747 * v2747;
                let v2750 = (v1 + v2748).sqrt();
                let v2752 = (v1692 * v2750) / v2744;
                let v2755 = (v2753 * v2748) + v2752;
                v2756 = v2744;
                v2757 = v2750;
                v2762 = v2748;
                v2774 = v2752;
                v2780 = v2755;
            }
            let v2759 = v2725 + (v2756 * v2757);
            let v2760 = v1 / v2759;
            let v2768 = (v2601 + v2724) - (((((v2729 * v2762) * v2760) * v2760).abs()).ln());
            let v2779 = ((v2776 * v2374) * v2725) + v2727;
            let v2781 = v2780 * v2779;
            let v2802 = v2723 + ((-(v2727 + (v2759 * ((v2375 * v2768) + v2725)))) / (((v2727 - (v2374 * (v2725 + v2759))) + (v2725 * v2781)) + (v2375 * ((((v2782 + (v62 * ((v2624 + v2781) * v2760))) - (((v1 / v2729) - v2774) * v2779)) * v2759) + (v2768 * (v2781 - v2374))))));
            let v2803 = v2385 - v2802;
            let v2804 = v2374 * v2803;
            let v2806 = v2474 * (v2802.exp());
            let v2808 = (v2804 * v2804) + v2806;
            let v2809 = if v2808 < v0 { 1.0 } else { 0.0 };
            let v2835: f64;
            let v2836: f64;
            let v2841: f64;
            let v2853: f64;
            let v2859: f64;
            if v2809 != 0.0 {
                let v2811 = (-v2808).sqrt();
                let v2812 = v1692 * v2811;
                let v2814 = v1 / (v2812.sin());
                let v2815 = v2814 * v2814;
                let v2817 = (v2812.cos()) * v2814;
                let v2820 = (v2818 * v2817) / v2811;
                let v2822 = (v1799 * v2815) + v2820;
                v2835 = v2811;
                v2836 = v2817;
                v2841 = v2815;
                v2853 = v2820;
                v2859 = v2822;
            } else {
                let v2823 = v2808.sqrt();
                let v2826 = v1 / ((v1692 * v2823).sinh());
                let v2827 = v2826 * v2826;
                let v2829 = (v1 + v2827).sqrt();
                let v2831 = (v1692 * v2829) / v2823;
                let v2834 = (v2832 * v2827) + v2831;
                v2835 = v2823;
                v2836 = v2829;
                v2841 = v2827;
                v2853 = v2831;
                v2859 = v2834;
            }
            let v2838 = v2804 + (v2835 * v2836);
            let v2839 = v1 / v2838;
            let v2847 = (v2601 + v2803) - (((((v2808 * v2841) * v2839) * v2839).abs()).ln());
            let v2858 = ((v2855 * v2374) * v2804) + v2806;
            let v2860 = v2859 * v2858;
            let v2881 = v2802 + ((-(v2806 + (v2838 * ((v2375 * v2847) + v2804)))) / (((v2806 - (v2374 * (v2804 + v2838))) + (v2804 * v2860)) + (v2375 * ((((v2861 + (v62 * ((v2624 + v2860) * v2839))) - (((v1 / v2808) - v2853) * v2858)) * v2838) + (v2847 * (v2860 - v2374))))));
            let v2882 = v2385 - v2881;
            let v2883 = v2374 * v2882;
            let v2885 = v2474 * (v2881.exp());
            let v2887 = (v2883 * v2883) + v2885;
            let v2888 = if v2887 < v0 { 1.0 } else { 0.0 };
            let v2914: f64;
            let v2915: f64;
            let v2920: f64;
            let v2932: f64;
            let v2938: f64;
            if v2888 != 0.0 {
                let v2890 = (-v2887).sqrt();
                let v2891 = v1692 * v2890;
                let v2893 = v1 / (v2891.sin());
                let v2894 = v2893 * v2893;
                let v2896 = (v2891.cos()) * v2893;
                let v2899 = (v2897 * v2896) / v2890;
                let v2901 = (v1799 * v2894) + v2899;
                v2914 = v2890;
                v2915 = v2896;
                v2920 = v2894;
                v2932 = v2899;
                v2938 = v2901;
            } else {
                let v2902 = v2887.sqrt();
                let v2905 = v1 / ((v1692 * v2902).sinh());
                let v2906 = v2905 * v2905;
                let v2908 = (v1 + v2906).sqrt();
                let v2910 = (v1692 * v2908) / v2902;
                let v2913 = (v2911 * v2906) + v2910;
                v2914 = v2902;
                v2915 = v2908;
                v2920 = v2906;
                v2932 = v2910;
                v2938 = v2913;
            }
            let v2917 = v2883 + (v2914 * v2915);
            let v2918 = v1 / v2917;
            let v2926 = (v2601 + v2882) - (((((v2887 * v2920) * v2918) * v2918).abs()).ln());
            let v2937 = ((v2934 * v2374) * v2883) + v2885;
            let v2939 = v2938 * v2937;
            let v2960 = v2881 + ((-(v2885 + (v2917 * ((v2375 * v2926) + v2883)))) / (((v2885 - (v2374 * (v2883 + v2917))) + (v2883 * v2939)) + (v2375 * ((((v2940 + (v62 * ((v2624 + v2939) * v2918))) - (((v1 / v2887) - v2932) * v2937)) * v2917) + (v2926 * (v2939 - v2374))))));
            let v2961 = v2385 - v2960;
            let v2963 = v2373 * (v2960.exp());
            let v2966 = ((v2380 * v2961) * v2961) - v2963;
            let v2967 = if v2966 < v0 { 1.0 } else { 0.0 };
            let v2983: f64;
            let v2985: f64;
            if v2967 != 0.0 {
                let v2969 = (-v2966).sqrt();
                let v2970 = v1692 * v2969;
                let v2972 = v2969 / (v2970.tan());
                let v2973 = v2970.sin();
                let v2975 = (-v2973) * v2973;
                v2983 = v2972;
                v2985 = v2975;
            } else {
                let v2976 = v2966.sqrt();
                let v2977 = v1692 * v2976;
                let v2978 = v2977.sinh();
                let v2979 = v2978 * v2978;
                let v2981 = v2976 / (v2977.tanh());
                v2983 = v2981;
                v2985 = v2979;
            }
            let v2989 = ((v2374 * v2961) - v2983) / (v1 - (v2966 / (v2985 * v2963)));
            let v2991 = (v2961 * v1455) * v2342;
            let v2993 = (v2989 * v1463) * v2342;
            let v2994 = v2993 - v2991;
            let v2997 = v2388 - (v2994 / (v1461 * v2342));
            let v3000 = ((v2960 + v2997) * v2342) / v62;
            let v3001 = v2993 / v1455;
            let v3005 = ((v3002 * v2991) / v1455) + v2345;
            let v3015 = ((v3012 * v2994) / v1461) + v2345;
            let v3022 = v1800 / v1455;
            let v3026 = v1692 * (v1 + ((v3001 / v3022).abs()));
            let v3038 = v1 + (((v1927 + (v2118 * v1916)) * (((v1704 * (v1692 * (v3005 + (((v3005 * v3005) + v1725).sqrt())))).abs()).powf((v3031 + (v1518 * v2118))))) + (v1930 / (v3026.powf(v1933))));
            let v3040 = v3038 - v1;
            let v3044 = (v1799 * v3042) * v3042;
            let v3062 = v1 + (((v1533 + (v2118 * v1539)) * (((v1712 * (v1692 * (v3015 + (((v3015 * v3015) + v1725).sqrt())))).abs()).powf((v1558 + (v1564 * v2118))))) + (v1552 / (v3026.powf(v837))));
            let v3064 = v3062 - v1;
            let v3078 = ((v2367 - (v2991 / v1455)) / v2342).exp();
            let v3080 = ((v2386 - (v2994 / v1461)) / v2342).exp();
            let v3081 = v3078 + v3080;
            let v3086 = ((v3078 / v3081) * (v1905 / ((v1692 * ((v3038 + v1) + (((v3040 * v3040) + v3044).sqrt()))) / v3049))) + ((v3080 / v3081) * (v3071 / ((v1692 * ((v3062 + v1) + (((v3064 * v3064) + v3044).sqrt()))) / v3049)));
            let v3132: f64;
            if v1566 != 0.0 {
                v3132 = v0;
            } else {
                let v3087 = if v1565 == v0 { 1.0 } else { 0.0 };
                let v3133: f64;
                if v3087 != 0.0 {
                    let v3091 = v1 / (v1 + (v3088 * v3001));
                    let v3105 = (((v3097 + (v3099 * (v1692 * (v3091 + (((v3091 * v3091) + v1800).sqrt()))))) * v1708) * v20) * v1942;
                    v3133 = v3105;
                } else {
                    let v3108 = v1 / (v1 + (v3088 * v3001));
                    let v3122 = (((((v3114 + v3115) + v3097) + (v3099 * (v1692 * (v3108 + (((v3108 * v3108) + v1800).sqrt()))))) * v1708) * v20) * v1942;
                    v3133 = v3122;
                }
                v3132 = v3133;
            }
            let v3124 = v62 * v3123;
            let v3126 = (v3124 / v3086) * v64;
            let v3131 = v1357 * ((v3001 + (v1377 * v2285)) + (v1826 * v1367));
            let v3134 = if v3132 == v0 { 1.0 } else { 0.0 };
            let v3157: f64;
            if v3134 != 0.0 {
                let v3137 = (v3126 * v3131) / (v3126 + v3131);
                v3157 = v3137;
            } else {
                let v3140 = ((v69 * v3123) * v1455) * v3132;
                let v3141 = v62 * v3140;
                let v3145 = (v3131 + v3126) + ((v1696 * v3131) * v3140);
                let v3156 = (v3145 - (((v3145 * v3145) - ((v62 * v3141) * (v3131 * (v3126 + ((v62 * v3131) * v3140))))).sqrt())) / v3141;
                v3157 = v3156;
            }
            let v3158 = v3157 - v1725;
            let v3165 = (v1692 * (v3158 + (((v3158 * v3158) + v3160).sqrt()))) + v1725;
            let v3170 = v2109 / ((v1 + ((v2109 / v3165).powf(v2029))).powf(v1690));
            let v3171 = if v3170 > v2109 { 1.0 } else { 0.0 };
            let v3172: f64;
            if v3171 != 0.0 {
                v3172 = v2109;
            } else {
                v3172 = v3170;
            }
            let v3174 = (v2367 - v3172) / v2342;
            let v3176 = (v2387 - v3172) / v2342;
            let v3177 = v3174 - v2379;
            let v3182 = ((((v2380 * v3177) * v3177) + v2377).ln()) - v2376;
            let v3185 = ((v2462 - v2997) / v2374) - v2379;
            let v3195 = ((v3182 - ((((((v2380 * v3185) * v3185) + v2377).ln()) - v2376) - v2379)) + (v2375 * v3176)) / v2397;
            let v3200 = if (if (v3176 + (v2384 * (v3174 - v3176))) <= v3182 { (v3176 + (v2384 * (v3174 - v3176))) } else { v3182 }) <= v2379 { (if (v3176 + (v2384 * (v3174 - v3176))) <= v3182 { (v3176 + (v2384 * (v3174 - v3176))) } else { v3182 }) } else { v2379 };
            let v3203 = (v3200 + (v2374 * v3174)) / v2406;
            let v3204 = v3176 - v3195;
            let v3209 = ((v2409 * v3204) * v3204) - (v2373 * (v3195.exp()));
            let v3210 = if v3209 < v0 { 1.0 } else { 0.0 };
            let v3268: f64;
            if v3210 != 0.0 {
                let v3212 = (v3176 - v3200) * v2375;
                let v3213 = v2149 * v2374;
                let v3214 = v3213 + v3212;
                let v3215 = v3213 * v3212;
                let v3217 = (v2421 * v3214) + v1;
                let v3220 = ((v3214 * v2424) + v3215) + v2377;
                let v3244 = if ((((-v3220) + ((((v3225 * v3217) * ((v2428 * v3214) + (v2377 * v3215))) + (v3220 * v3220)).sqrt())) / (v62 * v3217)) * (v1 - (((-((v3174 - ((v2462 - v3200) / v2374)) + v62)) / v3239).exp()))) <= v2453 { ((((-v3220) + ((((v3225 * v3217) * ((v2428 * v3214) + (v2377 * v3215))) + (v3220 * v3220)).sqrt())) / (v62 * v3217)) * (v1 - (((-((v3174 - ((v2462 - v3200) / v2374)) + v62)) / v3239).exp()))) } else { v2453 };
                v3268 = v3244;
            } else {
                v3268 = v3209;
            }
            let v3245 = if v3174 >= v2379 { v3174 } else { v2379 };
            let v3246 = v3245 - v2379;
            let v3254 = ((v2462 - v3200) / v2374) - v2379;
            let v3261 = (((((v2380 * v3246) * v3246) + v2377).ln()) - v2376) - ((((((v2380 * v3254) * v3254) + v2377).ln()) - v2376) - v2379);
            let v3262 = v3245 - v3261;
            let v3264 = v2474 * (v3261.exp());
            let v3265 = v2380 * v3262;
            let v3275 = v3261 + ((-(((v3265 * v3262) + v3264) - v3268)) / ((v3271 * v3265) + v3264));
            let v3276 = v3245 - v3275;
            let v3277 = v2380 * v3276;
            let v3279 = (v3277 * v3276) - v3268;
            let v3280 = v1 / v3279;
            let v3289 = v1 / (((v3285 * v3277) * v3280) - v1);
            let v3297 = ((((v3279.abs()).ln()) - v2376) - v3275) * v3289;
            let v3307 = v3275 + (if (if ((-v3297) - ((((v1692 * v3297) * v3297) * (((((v3290 * v3277) * v3277) * v3280) * v3280) + (v2507 * v3280))) * v3289)) >= v3304 { ((-v3297) - ((((v1692 * v3297) * v3297) * (((((v3290 * v3277) * v3277) * v3280) * v3280) + (v2507 * v3280))) * v3289)) } else { v3304 }) <= v2517 { (if ((-v3297) - ((((v1692 * v3297) * v3297) * (((((v3290 * v3277) * v3277) * v3280) * v3280) + (v2507 * v3280))) * v3289)) >= v3304 { ((-v3297) - ((((v1692 * v3297) * v3297) * (((((v3290 * v3277) * v3277) * v3280) * v3280) + (v2507 * v3280))) * v3289)) } else { v3304 }) } else { v2517 });
            let v3308 = v3245 - v3307;
            let v3309 = v2380 * v3308;
            let v3311 = (v3309 * v3308) - v3268;
            let v3312 = v1 / v3311;
            let v3321 = v1 / (((v3317 * v3309) * v3312) - v1);
            let v3329 = ((((v3311.abs()).ln()) - v2376) - v3307) * v3321;
            let v3340 = if (v3307 + (if (if ((-v3329) - ((((v1692 * v3329) * v3329) * (((((v3322 * v3309) * v3309) * v3312) * v3312) + (v2507 * v3312))) * v3321)) >= v3336 { ((-v3329) - ((((v1692 * v3329) * v3329) * (((((v3322 * v3309) * v3309) * v3312) * v3312) + (v2507 * v3312))) * v3321)) } else { v3336 }) <= v2517 { (if ((-v3329) - ((((v1692 * v3329) * v3329) * (((((v3322 * v3309) * v3309) * v3312) * v3312) + (v2507 * v3312))) * v3321)) >= v3336 { ((-v3329) - ((((v1692 * v3329) * v3329) * (((((v3322 * v3309) * v3309) * v3312) * v3312) + (v2507 * v3312))) * v3321)) } else { v3336 }) } else { v2517 })) >= v2554 { (v3307 + (if (if ((-v3329) - ((((v1692 * v3329) * v3329) * (((((v3322 * v3309) * v3309) * v3312) * v3312) + (v2507 * v3312))) * v3321)) >= v3336 { ((-v3329) - ((((v1692 * v3329) * v3329) * (((((v3322 * v3309) * v3309) * v3312) * v3312) + (v2507 * v3312))) * v3321)) } else { v3336 }) <= v2517 { (if ((-v3329) - ((((v1692 * v3329) * v3329) * (((((v3322 * v3309) * v3309) * v3312) * v3312) + (v2507 * v3312))) * v3321)) >= v3336 { ((-v3329) - ((((v1692 * v3329) * v3329) * (((((v3322 * v3309) * v3309) * v3312) * v3312) + (v2507 * v3312))) * v3321)) } else { v3336 }) } else { v2517 })) } else { v2554 };
            let v3347 = if (v3203 - ((v1 + ((v3203 - (v2556 * v3340)).exp())).ln())) <= v3340 { (v3203 - ((v1 + ((v3203 - (v2556 * v3340)).exp())).ln())) } else { v3340 };
            let v3348 = v3174 - v3347;
            let v3349 = v2374 * v3348;
            let v3351 = v2474 * (v3347.exp());
            let v3353 = (v3349 * v3349) + v3351;
            let v3354 = if v3353 < v0 { 1.0 } else { 0.0 };
            let v3380: f64;
            let v3381: f64;
            let v3387: f64;
            let v3399: f64;
            let v3405: f64;
            if v3354 != 0.0 {
                let v3356 = (-v3353).sqrt();
                let v3357 = v1692 * v3356;
                let v3359 = v1 / (v3357.sin());
                let v3360 = v3359 * v3359;
                let v3362 = (v3357.cos()) * v3359;
                let v3365 = (v3363 * v3362) / v3356;
                let v3367 = (v1799 * v3360) + v3365;
                v3380 = v3356;
                v3381 = v3362;
                v3387 = v3360;
                v3399 = v3365;
                v3405 = v3367;
            } else {
                let v3368 = v3353.sqrt();
                let v3371 = v1 / ((v1692 * v3368).sinh());
                let v3372 = v3371 * v3371;
                let v3374 = (v1 + v3372).sqrt();
                let v3376 = (v1692 * v3374) / v3368;
                let v3379 = (v3377 * v3372) + v3376;
                v3380 = v3368;
                v3381 = v3374;
                v3387 = v3372;
                v3399 = v3376;
                v3405 = v3379;
            }
            let v3383 = v3349 + (v3380 * v3381);
            let v3384 = v1 / v3383;
            let v3385 = v3176 - v3174;
            let v3393 = (v3385 + v3348) - (((((v3353 * v3387) * v3384) * v3384).abs()).ln());
            let v3404 = ((v3401 * v2374) * v3349) + v3351;
            let v3406 = v3405 * v3404;
            let v3427 = v3347 + ((-(v3351 + (v3383 * ((v2375 * v3393) + v3349)))) / (((v3351 - (v2374 * (v3349 + v3383))) + (v3349 * v3406)) + (v2375 * ((((v3407 + (v62 * ((v2624 + v3406) * v3384))) - (((v1 / v3353) - v3399) * v3404)) * v3383) + (v3393 * (v3406 - v2374))))));
            let v3428 = v3174 - v3427;
            let v3429 = v2374 * v3428;
            let v3431 = v2474 * (v3427.exp());
            let v3433 = (v3429 * v3429) + v3431;
            let v3434 = if v3433 < v0 { 1.0 } else { 0.0 };
            let v3460: f64;
            let v3461: f64;
            let v3466: f64;
            let v3478: f64;
            let v3484: f64;
            if v3434 != 0.0 {
                let v3436 = (-v3433).sqrt();
                let v3437 = v1692 * v3436;
                let v3439 = v1 / (v3437.sin());
                let v3440 = v3439 * v3439;
                let v3442 = (v3437.cos()) * v3439;
                let v3445 = (v3443 * v3442) / v3436;
                let v3447 = (v1799 * v3440) + v3445;
                v3460 = v3436;
                v3461 = v3442;
                v3466 = v3440;
                v3478 = v3445;
                v3484 = v3447;
            } else {
                let v3448 = v3433.sqrt();
                let v3451 = v1 / ((v1692 * v3448).sinh());
                let v3452 = v3451 * v3451;
                let v3454 = (v1 + v3452).sqrt();
                let v3456 = (v1692 * v3454) / v3448;
                let v3459 = (v3457 * v3452) + v3456;
                v3460 = v3448;
                v3461 = v3454;
                v3466 = v3452;
                v3478 = v3456;
                v3484 = v3459;
            }
            let v3463 = v3429 + (v3460 * v3461);
            let v3464 = v1 / v3463;
            let v3472 = (v3385 + v3428) - (((((v3433 * v3466) * v3464) * v3464).abs()).ln());
            let v3483 = ((v3480 * v2374) * v3429) + v3431;
            let v3485 = v3484 * v3483;
            let v3506 = v3427 + ((-(v3431 + (v3463 * ((v2375 * v3472) + v3429)))) / (((v3431 - (v2374 * (v3429 + v3463))) + (v3429 * v3485)) + (v2375 * ((((v3486 + (v62 * ((v2624 + v3485) * v3464))) - (((v1 / v3433) - v3478) * v3483)) * v3463) + (v3472 * (v3485 - v2374))))));
            let v3507 = v3174 - v3506;
            let v3508 = v2374 * v3507;
            let v3510 = v2474 * (v3506.exp());
            let v3512 = (v3508 * v3508) + v3510;
            let v3513 = if v3512 < v0 { 1.0 } else { 0.0 };
            let v3539: f64;
            let v3540: f64;
            let v3545: f64;
            let v3557: f64;
            let v3563: f64;
            if v3513 != 0.0 {
                let v3515 = (-v3512).sqrt();
                let v3516 = v1692 * v3515;
                let v3518 = v1 / (v3516.sin());
                let v3519 = v3518 * v3518;
                let v3521 = (v3516.cos()) * v3518;
                let v3524 = (v3522 * v3521) / v3515;
                let v3526 = (v1799 * v3519) + v3524;
                v3539 = v3515;
                v3540 = v3521;
                v3545 = v3519;
                v3557 = v3524;
                v3563 = v3526;
            } else {
                let v3527 = v3512.sqrt();
                let v3530 = v1 / ((v1692 * v3527).sinh());
                let v3531 = v3530 * v3530;
                let v3533 = (v1 + v3531).sqrt();
                let v3535 = (v1692 * v3533) / v3527;
                let v3538 = (v3536 * v3531) + v3535;
                v3539 = v3527;
                v3540 = v3533;
                v3545 = v3531;
                v3557 = v3535;
                v3563 = v3538;
            }
            let v3542 = v3508 + (v3539 * v3540);
            let v3543 = v1 / v3542;
            let v3551 = (v3385 + v3507) - (((((v3512 * v3545) * v3543) * v3543).abs()).ln());
            let v3562 = ((v3559 * v2374) * v3508) + v3510;
            let v3564 = v3563 * v3562;
            let v3585 = v3506 + ((-(v3510 + (v3542 * ((v2375 * v3551) + v3508)))) / (((v3510 - (v2374 * (v3508 + v3542))) + (v3508 * v3564)) + (v2375 * ((((v3565 + (v62 * ((v2624 + v3564) * v3543))) - (((v1 / v3512) - v3557) * v3562)) * v3542) + (v3551 * (v3564 - v2374))))));
            let v3586 = v3174 - v3585;
            let v3587 = v2374 * v3586;
            let v3589 = v2474 * (v3585.exp());
            let v3591 = (v3587 * v3587) + v3589;
            let v3592 = if v3591 < v0 { 1.0 } else { 0.0 };
            let v3618: f64;
            let v3619: f64;
            let v3624: f64;
            let v3636: f64;
            let v3642: f64;
            if v3592 != 0.0 {
                let v3594 = (-v3591).sqrt();
                let v3595 = v1692 * v3594;
                let v3597 = v1 / (v3595.sin());
                let v3598 = v3597 * v3597;
                let v3600 = (v3595.cos()) * v3597;
                let v3603 = (v3601 * v3600) / v3594;
                let v3605 = (v1799 * v3598) + v3603;
                v3618 = v3594;
                v3619 = v3600;
                v3624 = v3598;
                v3636 = v3603;
                v3642 = v3605;
            } else {
                let v3606 = v3591.sqrt();
                let v3609 = v1 / ((v1692 * v3606).sinh());
                let v3610 = v3609 * v3609;
                let v3612 = (v1 + v3610).sqrt();
                let v3614 = (v1692 * v3612) / v3606;
                let v3617 = (v3615 * v3610) + v3614;
                v3618 = v3606;
                v3619 = v3612;
                v3624 = v3610;
                v3636 = v3614;
                v3642 = v3617;
            }
            let v3621 = v3587 + (v3618 * v3619);
            let v3622 = v1 / v3621;
            let v3630 = (v3385 + v3586) - (((((v3591 * v3624) * v3622) * v3622).abs()).ln());
            let v3641 = ((v3638 * v2374) * v3587) + v3589;
            let v3643 = v3642 * v3641;
            let v3664 = v3585 + ((-(v3589 + (v3621 * ((v2375 * v3630) + v3587)))) / (((v3589 - (v2374 * (v3587 + v3621))) + (v3587 * v3643)) + (v2375 * ((((v3644 + (v62 * ((v2624 + v3643) * v3622))) - (((v1 / v3591) - v3636) * v3641)) * v3621) + (v3630 * (v3643 - v2374))))));
            let v3665 = v3174 - v3664;
            let v3666 = v2374 * v3665;
            let v3668 = v2474 * (v3664.exp());
            let v3670 = (v3666 * v3666) + v3668;
            let v3671 = if v3670 < v0 { 1.0 } else { 0.0 };
            let v3697: f64;
            let v3698: f64;
            let v3703: f64;
            let v3715: f64;
            let v3721: f64;
            if v3671 != 0.0 {
                let v3673 = (-v3670).sqrt();
                let v3674 = v1692 * v3673;
                let v3676 = v1 / (v3674.sin());
                let v3677 = v3676 * v3676;
                let v3679 = (v3674.cos()) * v3676;
                let v3682 = (v3680 * v3679) / v3673;
                let v3684 = (v1799 * v3677) + v3682;
                v3697 = v3673;
                v3698 = v3679;
                v3703 = v3677;
                v3715 = v3682;
                v3721 = v3684;
            } else {
                let v3685 = v3670.sqrt();
                let v3688 = v1 / ((v1692 * v3685).sinh());
                let v3689 = v3688 * v3688;
                let v3691 = (v1 + v3689).sqrt();
                let v3693 = (v1692 * v3691) / v3685;
                let v3696 = (v3694 * v3689) + v3693;
                v3697 = v3685;
                v3698 = v3691;
                v3703 = v3689;
                v3715 = v3693;
                v3721 = v3696;
            }
            let v3700 = v3666 + (v3697 * v3698);
            let v3701 = v1 / v3700;
            let v3709 = (v3385 + v3665) - (((((v3670 * v3703) * v3701) * v3701).abs()).ln());
            let v3720 = ((v3717 * v2374) * v3666) + v3668;
            let v3722 = v3721 * v3720;
            let v3743 = v3664 + ((-(v3668 + (v3700 * ((v2375 * v3709) + v3666)))) / (((v3668 - (v2374 * (v3666 + v3700))) + (v3666 * v3722)) + (v2375 * ((((v3723 + (v62 * ((v2624 + v3722) * v3701))) - (((v1 / v3670) - v3715) * v3720)) * v3700) + (v3709 * (v3722 - v2374))))));
            let v3744 = v3174 - v3743;
            let v3746 = v2373 * (v3743.exp());
            let v3749 = ((v2380 * v3744) * v3744) - v3746;
            let v3750 = if v3749 < v0 { 1.0 } else { 0.0 };
            let v3766: f64;
            let v3768: f64;
            if v3750 != 0.0 {
                let v3752 = (-v3749).sqrt();
                let v3753 = v1692 * v3752;
                let v3755 = v3752 / (v3753.tan());
                let v3756 = v3753.sin();
                let v3758 = (-v3756) * v3756;
                v3766 = v3755;
                v3768 = v3758;
            } else {
                let v3759 = v3749.sqrt();
                let v3760 = v1692 * v3759;
                let v3761 = v3760.sinh();
                let v3762 = v3761 * v3761;
                let v3764 = v3759 / (v3760.tanh());
                v3766 = v3764;
                v3768 = v3762;
            }
            let v3772 = ((v2374 * v3744) - v3766) / (v1 - (v3749 / (v3768 * v3746)));
            let v3774 = (v3744 * v1455) * v2342;
            let v3776 = (v3772 * v1463) * v2342;
            let v3777 = v3776 - v3774;
            let v3778 = v3776 / v1455;
            let v3780 = v1692 * (v3001 + v3778);
            let v3781 = v3001 - v3778;
            let v3782 = v3172 * v3172;
            let v3784 = v3782 / v3783;
            let v3786 = if v3785 != v0 { 1.0 } else { 0.0 };
            let v3817: f64;
            if v3786 != 0.0 {
                let v3798 = ((v2991 + v3774) / (v62 * v1455)) + ((((v3785 * (v1 - (rspice_limited_exp((-v3784))))) * v1692) * (v2991 - v3774)) / v1455);
                v3817 = v3798;
            } else {
                let v3801 = (v2991 + v3774) / (v62 * v1455);
                v3817 = v3801;
            }
            let v3803 = if v3802 != v0 { 1.0 } else { 0.0 };
            let v3826: f64;
            if v3803 != 0.0 {
                let v3814 = ((v2994 + v3777) / v2230) + ((((v3802 * (v1 - (rspice_limited_exp((-v3784))))) * v1692) * (v2994 - v3777)) / v1461);
                v3826 = v3814;
            } else {
                let v3816 = (v2994 + v3777) / v2230;
                v3826 = v3816;
            }
            let v3819 = (v3002 * v3817) + v2345;
            let v3828 = (v3012 * v3826) + v2345;
            let v3838 = v1692 * (v1 + ((v3780 / v3022).abs()));
            let v3851 = v1 + (((v1927 + (v2119 * v1916)) * (((v1704 * (v1692 * (v3819 + (((v3819 * v3819) + v1725).sqrt())))).abs()).powf((v3031 + (v1518 * v2119))))) + ((v1930 + (v2119 * v1500)) / (v3838.powf(v1933))));
            let v3853 = v3851 - v1;
            let v3873 = v1 + (((v1533 + (v2119 * v1539)) * (((v1712 * (v1692 * (v3828 + (((v3828 * v3828) + v1725).sqrt())))).abs()).powf((v1558 + (v1564 * v2119))))) + ((v1552 + (v2119 * v1546)) / (v3838.powf(v837))));
            let v3875 = v3873 - v1;
            let v3891 = ((v2367 - ((v2991 + v3774) / (v62 * v1455))) / v2342).exp();
            let v3893 = ((v2386 - ((v2994 + v3777) / v2230)) / v2342).exp();
            let v3894 = v3891 + v3893;
            let v3899 = ((v3891 / v3894) * (v1905 / ((v1692 * ((v3851 + v1) + (((v3853 * v3853) + v3044).sqrt()))) / v3049))) + ((v3893 / v3894) * (v3071 / ((v1692 * ((v3873 + v1) + (((v3875 * v3875) + v3044).sqrt()))) / v3049)));
            let v3902 = ((v3899 * v1455) * v69) / v64;
            let v3910 = v1 + (v1927 * (((v1704 * (v2345 + (v3903 * v3780))).abs()).powf(v3031)));
            let v3912 = v3910 - v1;
            let v3925 = v3923 + (v2017 * v2119);
            let v3934 = (v3781 / (((v62 * v3919) / v3899) * v64)) * (v3926 + (v1692 * (v3925 + (((v3925 * v3925) + v1800).sqrt()))));
            let v3951 = ((v1 + ((v3935 + (v3934 * v3934)).sqrt())) / (v1 + (v3935.sqrt()))) + ((((v1692 * ((v2056 - (v1610 * v2285)) - (v1616 * v2119))) * v3780) * v3781) * v3781);
            let v3953 = v3951 - v1;
            let v3961 = v1692 * ((v3951 + v1) + (((v3953 * v3953) + ((v1799 * v3955) * v3955)).sqrt()));
            let v3966 = (((v62 * v3962) * ((v1692 * ((v3910 + v1) + (((v3912 * v3912) + v3044).sqrt()))) / v3049)) / v1905) * v93;
            let v3967 = if v997 > v0 { 1.0 } else { 0.0 };
            let v3983: f64;
            if v3967 != 0.0 {
                let v3970 = v1 + ((v997 * v3780) / v3126);
                v3983 = v3970;
            } else {
                let v3974 = v1 / (v1 - ((v997 * v3780) / v3126));
                v3983 = v3974;
            }
            let v3975 = v2109 - v3172;
            let v3976 = v3780 + v1826;
            let v3978 = if v3977 > v0 { 1.0 } else { 0.0 };
            let v4006: f64;
            if v3978 != 0.0 {
                let v3986 = v1 + (v3975 / (((v3976 / v3977) * (v3976 / (v3165 + v3976))) * v3983));
                v4006 = v3986;
            } else {
                v4006 = v1;
            }
            let v3987 = if v1590 > v0 { 1.0 } else { 0.0 };
            let v4007: f64;
            if v3987 != 0.0 {
                let v3989 = if v3988 < v0 { 1.0 } else { 0.0 };
                let v3997: f64;
                if v3989 != 0.0 {
                    let v3993 = v1 / ((v1 / v1590) - (v3988 * v3780));
                    v3997 = v3993;
                } else {
                    let v3996 = v1590 * (v1 + (v3988 * v3780));
                    v3997 = v3996;
                }
                let v4005 = v1 + (v3997 * ((if (v1 + ((v3975 / v3997) / (v3165 + v3126))) >= v1691 { (v1 + ((v3975 / v3997) / (v3165 + v3126))) } else { v1691 }).ln()));
                v4007 = v4005;
            } else {
                v4007 = v1;
            }
            let v4008 = v4006 * v4007;
            let v4009 = if v957 > v0 { 1.0 } else { 0.0 };
            let v4162: f64;
            if v4009 != 0.0 {
                let v4017 = v1 + (v957 * ((if (v1 + ((v3975 / v957) / (v3165 + v3966))) >= v1691 { (v1 + ((v3975 / v957) / (v3165 + v3966))) } else { v1691 }).ln()));
                v4162 = v4017;
            } else {
                v4162 = v1;
            }
            let v4018 = if v2031 != v0 { 1.0 } else { 0.0 };
            let v4131: f64;
            if v4018 != 0.0 {
                let v4028 = rspice_limited_exp((-(v2031 / (((if v0 >= (v2044 + ((v2046 * v3781) * v3781)) { v0 } else { (v2044 + ((v2046 * v3781) * v3781)) }) * v3780) + (v62 * v2342)))));
                v4131 = v4028;
            } else {
                v4131 = v1;
            }
            let v4033 = v1463 * v2342;
            let v4042 = (((v4033 * v62) * v1810) * (v2989 - v3772)) + (((((v4033 * v1463) * v2342) * v1692) * ((v2989 * v2989) - (v3772 * v3772))) / v1455);
            let v4043 = v3780 + v1810;
            let v4133: f64;
            let v4482: f64;
            let v4509: f64;
            let v4512: f64;
            if v1566 != 0.0 {
                let v4044 = v2093 - v1887;
                let v4055 = (v1 / (v1 + (v3088 * (v1692 * (v4044 + (((v4044 * v4044) + v1850).sqrt())))))) - ((v1692 * v2101) * v153);
                let v4069 = v1942 * (v3114 + ((v4061 + (v4063 * (v1692 * (v4055 + (((v4055 * v4055) + v1800).sqrt()))))) * v1708));
                let v4070 = v2098 - v1887;
                let v4081 = (v1 / (v1 + (v3088 * (v1692 * (v4070 + (((v4070 * v4070) + v1850).sqrt())))))) - ((v1692 * v2103) * v153);
                let v4095 = v1942 * (v3115 + ((v4087 + (v4089 * (v1692 * (v4081 + (((v4081 * v4081) + v1800).sqrt()))))) * v1708));
                v4133 = v1;
                v4482 = v0;
                v4509 = v4095;
                v4512 = v4069;
            } else {
                let v4103 = (v1 / (v1 + (v3088 * v3780))) - ((v1692 * (v4099 + v2118)) * v153);
                let v4107 = v4103 + (((v4103 * v4103) + v1800).sqrt());
                let v4112 = v1942 * ((v3097 + (v3099 * (v1692 * v4107))) * v1708);
                let v4115 = ((v20 * v3902) * v4043) / v3961;
                let v4117 = v1 + (v4115 * v4112);
                let v4118 = if v1565 == v62 { 1.0 } else { 0.0 };
                let v4134: f64;
                let v4483: f64;
                let v4510: f64;
                let v4513: f64;
                if v4118 != 0.0 {
                    let v4125 = (v1942 * (((v3114 + v3115) + v3097) + (v3099 * (v1692 * v4107)))) * v1708;
                    let v4127 = v1 + (v4115 * v4125);
                    v4134 = v4127;
                    v4483 = v4125;
                    v4510 = v0;
                    v4513 = v0;
                } else {
                    v4134 = v4117;
                    v4483 = v4112;
                    v4510 = v3115;
                    v4513 = v3114;
                }
                v4133 = v4134;
                v4482 = v4483;
                v4509 = v4510;
                v4512 = v4513;
            }
            let v4137 = v20 * (((((v3902 / v1455) * v4042) * v4008) * v4131) / (v3961 * v4133));
            let v4141 = v4138 * (v2993 + (v62 * v3776));
            let v4145 = v4142 * ((v62 * v2993) + v3776);
            let v4146 = if v1407 > v0 { 1.0 } else { 0.0 };
            let v4411: f64;
            if v4146 != 0.0 {
                let v4160 = v4153 / (((v4154 * v1452) / v1467) + (((v1462 / (v1 + (((v3780 + (v1417 * v2345)) / v1427).powf(v1437)))) * v1407) / v1464));
                v4411 = v4160;
            } else {
                v4411 = v1458;
            }
            let v4163 = (v97 * v93) / v4162;
            let v4165 = (-v4141) * v4163;
            let v4167 = (-v4145) * v4163;
            let v4173 = if (if ((v1007 + (v1017 * v64)) / v64) <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v2064 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v4173 != 0.0 {
            } else {
                let v4176 = if v3975 > (v2064 / v4174) { 1.0 } else { 0.0 };
                if v4176 != 0.0 {
                } else {
                }
            }
            let v4178 = if v4177 != v0 { 1.0 } else { 0.0 };
            let v4254: f64;
            let v4255: f64;
            if v4178 != 0.0 {
                let v4198 = v69 * v64;
                let v4204 = (((((v4198 * v4197) * v1759) * v2105) * ((v1077 * v1810) * ((if (v1 + (rspice_limited_exp((((v3780 - v1067) / v1077) / v1810)))) >= v1691 { (v1 + (rspice_limited_exp((((v3780 - v1067) / v1077) / v1810)))) } else { v1691 }).ln()))) * (rspice_limited_exp((((v4192 * v1749) * (v1037 - (v1047 * v3780))) * (v1 + (v1057 * v3780)))))) * v2088;
                let v4205 = v1874 - v1840;
                let v4206 = v4205 - v2105;
                let v4214 = (v1117 * v1810) * ((if (v1 + (rspice_limited_exp(((v4206 / v1117) / v1810)))) >= v1691 { (v1 + (rspice_limited_exp(((v4206 / v1117) / v1810)))) } else { v1691 }).ln());
                let v4215 = if v4205 <= v0 { 1.0 } else { 0.0 };
                let v4231: f64;
                if v4215 != 0.0 {
                    let v4216 = v4206 - v2114;
                    let v4223 = v1692 * (v4216 + (((v4216 * v4216) - (v4218 * v4205)).sqrt()));
                    v4231 = v4223;
                } else {
                    let v4224 = v4206 - v2114;
                    let v4230 = v1692 * (v4224 + (((v4224 * v4224) + (v4218 * v4205)).sqrt()));
                    v4231 = v4230;
                }
                let v4246 = (((((v4198 * v1745) * v1759) * v2105) * v4214) * (rspice_limited_exp((((v4236 * v1749) * (v1087 - (v1097 * v4231))) * (v1 + (v1107 * v4231)))))) * v2088;
                v4254 = v4204;
                v4255 = v4246;
            } else {
                v4254 = v0;
                v4255 = v0;
            }
            let v4252 = v1692 + (v1692 * (((v4247 * v2096) / v1810).tanh()));
            let v4256 = v4254 + v4255;
            let v4257 = v4252 * v4256;
            let v4258 = (v1 - v4252) * v4256;
            let v4260 = if v4259 != v0 { 1.0 } else { 0.0 };
            let v4496: f64;
            let v4498: f64;
            let v4500: f64;
            let v4504: f64;
            if v4260 != 0.0 {
                let v4262 = v2121 - (v1157 * v3000);
                let v4269 = (-v4267) * v1749;
                let v4284 = (((((v69 * v64) * v1766) * v1759) * (v3780 * (rspice_limited_exp(((v4269 * (v1127 - (v1137 * v4262))) * (v1 + (v1147 * v4262))))))) * ((v2105 + (v1692 * v2115)) + (v1692 * (v2101 + v2103)))) * v2088;
                let v4289 = v1167 * (((v3782 + v1800).sqrt()) - v4287);
                let v4291 = rspice_limited_exp((-v4289));
                let v4301 = (v4289 * v4289) + v4300;
                let v4303 = (v4284 * ((v1 - ((v4289 + v1) * v4291)) + v1850)) / v4301;
                let v4305 = (v4284 * (((v4289 + v4291) - v1) + v1850)) / v4301;
                let v4308 = v2118 - v1889;
                let v4310 = (v2093 - v1887) + ((v1247 * v2271) * v4308);
                let v4313 = ((v4310 * v4310) + v1850).sqrt();
                let v4318 = v4269 * v1307;
                let v4321 = rspice_limited_exp(((v4318 * (v1217 - (v1227 * v4313))) * (v1 + (v1237 * v4313))));
                let v4323 = if v4322 > v0 { 1.0 } else { 0.0 };
                let v4502: f64;
                let v4506: f64;
                if v4323 != 0.0 {
                    let v4328 = (((v2089 * v4324) * v2093) * v4313) * v4321;
                    v4502 = v4328;
                    v4506 = v0;
                } else {
                    let v4332 = (((v2089 * v4324) * v2093) * v4313) * v4321;
                    v4502 = v0;
                    v4506 = v4332;
                }
                let v4336 = (v2098 - v1887) + ((v1287 * v2271) * v4308);
                let v4339 = ((v4336 * v4336) + v1850).sqrt();
                let v4346 = rspice_limited_exp(((v4318 * (v1257 - (v1267 * v4339))) * (v1 + (v1277 * v4339))));
                let v4501: f64;
                let v4505: f64;
                if v4323 != 0.0 {
                    let v4351 = (((v2089 * v4347) * v2098) * v4339) * v4346;
                    v4501 = v4502;
                    v4505 = v4351;
                } else {
                    let v4355 = (((v2089 * v4347) * v2098) * v4339) * v4346;
                    v4501 = v4355;
                    v4505 = v4506;
                }
                v4496 = v4303;
                v4498 = v4305;
                v4500 = v4501;
                v4504 = v4505;
            } else {
                v4496 = v0;
                v4498 = v0;
                v4500 = v0;
                v4504 = v0;
            }
            let v4357 = if v4356 != v0 { 1.0 } else { 0.0 };
            if v4357 != 0.0 {
                let v4360 = if (if v1197 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v2074 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v4360 != 0.0 {
                } else {
                }
                let v4361 = if v4322 > v0 { 1.0 } else { 0.0 };
                if v4361 != 0.0 {
                } else {
                }
                let v4364 = if (if v1177 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v2084 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v4364 != 0.0 {
                } else {
                }
                if v4361 != 0.0 {
                } else {
                }
            } else {
            }
            let v4365 = v3124 / v3899;
            let v4373 = if (if (if v4366 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4368 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v4371 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v4520: f64;
            if v4373 != 0.0 {
                let v4376 = v64 - (v62 * v4374);
                let v4377 = v4376 * v4376;
                let v4379 = if v4378 <= v0 { 1.0 } else { 0.0 };
                let v4452: f64;
                if v4379 != 0.0 {
                    v4452 = v0;
                } else {
                    let v4385 = v1710 * ((if (((v3975 / v1710) + v4378) / v4365) >= v1691 { (((v3975 / v1710) + v4378) / v4365) } else { v1691 }).ln());
                    let v4386 = if v4385 < v0 { 1.0 } else { 0.0 };
                    let v4453: f64;
                    if v4386 != 0.0 {
                        v4453 = v0;
                    } else {
                        v4453 = v4385;
                    }
                    v4452 = v4453;
                }
                let v4388 = if v4387 == v1 { 1.0 } else { 0.0 };
                let v4421: f64;
                if v4388 != 0.0 {
                    let v4393 = (v263 / (v1 + ((v3817 / v273).powf(v283)))) / v4366;
                    let v4395 = v4393 - v1;
                    let v4404 = v4366 * (v1692 * ((v4393 + v1) + (((v4395 * v4395) + ((v1799 * v4397) * v4397)).sqrt())));
                    v4421 = v4404;
                } else {
                    v4421 = v4366;
                }
                let v4415 = (v4411 * v3001) / v2227;
                let v4417 = (v4411 * v3778) / v2227;
                let v4420 = (v1810 / v2227) * (v4411 + v213);
                let v4423 = v4417 + v4420;
                let v4457 = (((((v4405 * v1810) * (v4137.abs())) * v3899) / ((v4410 * v4411) * v4377)) * (((v4421 * ((if ((v4415 + v4420) / v4423) >= v1691 { ((v4415 + v4420) / v4423) } else { v1691 }).ln())) + (v4368 * (v4415 - v4417))) + ((v1692 * v4371) * ((v4415 * v4415) - (v4417 * v4417))))) + (((((((v2227 * v1810) * v4137) * v4137) / (((v4410 * v4377) * v69) * v20)) * v4452) * ((v4421 + (v4368 * v4417)) + ((v4371 * v4417) * v4417))) / (v4423 * v4423));
                let v4467 = ((((v4421 * v2227) * v1810) / (((((v69 * v20) * v4376) * v4410) * v4420) * v4420)) * v4137) * v4137;
                let v4468 = v4467 + v4457;
                let v4469 = if v4468 > v0 { 1.0 } else { 0.0 };
                let v4521: f64;
                if v4469 != 0.0 {
                    let v4471 = (v4457 * v4467) / v4468;
                    v4521 = v4471;
                } else {
                    v4521 = v0;
                }
                v4520 = v4521;
            } else {
                v4520 = v0;
            }
            let v4472 = if v4322 > v0 { 1.0 } else { 0.0 };
            let v4477: f64;
            let v4478: f64;
            if v4472 != 0.0 {
                let v4473 = v20 * v4167;
                let v4474 = v20 * v4165;
                v4477 = v4473;
                v4478 = v4474;
            } else {
                let v4475 = v20 * v4165;
                let v4476 = v20 * v4167;
                v4477 = v4475;
                v4478 = v4476;
            }
            let v4481 = v3899 * (-(v4477 + v4478));
            let v4491 = (v1849 * v1810) * v2227;
            let v4492 = v4491 * ((v4481 / ((v4481 * v4482) + (v64 * v64))) * v4488);
            let v4495 = if v1439 != 0.0 && (if v4493 != v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v4495 != 0.0 {
            } else {
            }
            let v4497 = v20 * v4496;
            let v4499 = v20 * v4498;
            let v4503 = v20 * v4500;
            let v4507 = v20 * v4504;
            if v4472 != 0.0 {
            } else {
            }
            let v4508 = if v1565 == v62 { 1.0 } else { 0.0 };
            let v4546: f64;
            let v4547: f64;
            let v4548: f64;
            let v4549: f64;
            if v4508 != 0.0 {
                v4546 = v0;
                v4547 = v0;
                v4548 = v0;
                v4549 = v0;
            } else {
                let v4515 = v4491 * (v1 / v4509);
                let v4516 = v4491 * (v1 / v4512);
                v4546 = v1;
                v4547 = v4515;
                v4548 = v1;
                v4549 = v4516;
            }
            if v4495 != 0.0 {
            } else {
            }
            let v4517 = if v1784 == v0 { 1.0 } else { 0.0 };
            let v4550: f64;
            let v4551: f64;
            if v4517 != 0.0 {
                v4550 = v0;
                v4551 = v0;
            } else {
                let v4519 = v4491 * v4518;
                v4550 = v1;
                v4551 = v4519;
            }
            let v4552: f64;
            let v4554: f64;
            let v4556: f64;
            let v4558: f64;
            let v4560: f64;
            let v4562: f64;
            let v4564: f64;
            let v4566: f64;
            if v4260 != 0.0 {
                let v4553: f64;
                let v4555: f64;
                let v4557: f64;
                let v4559: f64;
                let v4561: f64;
                let v4563: f64;
                let v4565: f64;
                let v4567: f64;
                if v4472 != 0.0 {
                    let v4526 = v4523 * ((v4499 + v4503).abs());
                    let v4530 = v4527 * ((v4497 + v4507).abs());
                    v4553 = v1;
                    v4555 = v4526;
                    v4557 = v1;
                    v4559 = v4530;
                    v4561 = v0;
                    v4563 = v0;
                    v4565 = v0;
                    v4567 = v0;
                } else {
                    let v4534 = v4531 * ((v4499 + v4503).abs());
                    let v4538 = v4535 * ((v4497 + v4507).abs());
                    v4553 = v0;
                    v4555 = v0;
                    v4557 = v0;
                    v4559 = v0;
                    v4561 = v1;
                    v4563 = v4534;
                    v4565 = v1;
                    v4567 = v4538;
                }
                v4552 = v4553;
                v4554 = v4555;
                v4556 = v4557;
                v4558 = v4559;
                v4560 = v4561;
                v4562 = v4563;
                v4564 = v4565;
                v4566 = v4567;
            } else {
                v4552 = v0;
                v4554 = v0;
                v4556 = v0;
                v4558 = v0;
                v4560 = v0;
                v4562 = v0;
                v4564 = v0;
                v4566 = v0;
            }
            let v4568: f64;
            let v4569: f64;
            let v4570: f64;
            let v4571: f64;
            if v4178 != 0.0 {
                let v4541 = v4539 * (v4257.abs());
                let v4544 = v4542 * (v4258.abs());
                v4568 = v1;
                v4569 = v4541;
                v4570 = v1;
                v4571 = v4544;
            } else {
                v4568 = v0;
                v4569 = v0;
                v4570 = v0;
                v4571 = v0;
            }
            if v1718 != 0.0 {
                let v4545 = if v1565 != v62 { 1.0 } else { 0.0 };
                if v4545 != 0.0 {
                } else {
                }
            } else {
            }
            if v4472 != 0.0 {
            } else {
            }
            if v4472 != 0.0 {
            } else {
            }
            if v4472 != 0.0 {
            } else {
            }
        if v4546 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v4547;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v4548 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v4549;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v4550 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v4551;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v4520;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = Some(v4522);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        {
            let psd = v4492;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v4552 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v4554;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v4556 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v4558;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v4560 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v4562;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v4564 == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v4566;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v4568 == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v4569;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v4570 == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v4571;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 10, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

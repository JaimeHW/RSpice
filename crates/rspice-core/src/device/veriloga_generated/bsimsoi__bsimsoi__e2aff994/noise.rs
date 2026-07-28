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
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 21] = [
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 1, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 4, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "N2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 5, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "N1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 8, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("igs"), kind: GeneratedNoiseKind::White, equation: 14, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("igd"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_BI_IGB", label: Some("igb"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 18, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 19, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 20, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 21, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "N2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 22, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "N1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 25, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("igs"), kind: GeneratedNoiseKind::White, equation: 31, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("igd"), kind: GeneratedNoiseKind::White, equation: 32, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_BI_IGB", label: Some("igb"), kind: GeneratedNoiseKind::White, equation: 33, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF_EDGEFET", label: Some("1overf_edgefet"), kind: GeneratedNoiseKind::Flicker, equation: 34, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GM_RG", label: Some("rg"), kind: GeneratedNoiseKind::White, equation: 58, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(9), name: "gm", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 61, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 64, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
];

impl Instance {
    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {
        if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {
            return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });
        }
        let parameters = &self.params.values;
        let parameter_given = &*self.param_given;
        let temperature = ctx.temperature();
        let node_potentials = [ctx.node_voltage(self.nodes[0]), ctx.node_voltage(self.nodes[1]), ctx.node_voltage(self.nodes[2]), ctx.node_voltage(self.nodes[3]), ctx.node_voltage(self.nodes[4]), ctx.node_voltage(self.nodes[5]), ctx.node_voltage(self.nodes[6]), ctx.node_voltage(self.nodes[7]), ctx.node_voltage(self.nodes[8]), ctx.node_voltage(self.nodes[9]), ctx.node_voltage(self.nodes[10]), ctx.node_voltage(self.nodes[11]), ctx.node_voltage(self.nodes[12]), ctx.node_voltage(self.nodes[13])];
            let v0 = 0e0f64;
            let v1 = 1e0f64;
            let v2 = 1.602176462e-19f64;
            let v3 = 8.617342301212761e-5f64;
            let v4 = parameters[30];
            let v6 = -1e0f64;
            let v7 = parameters[109];
            let v8 = 8.8541878128e-12f64;
            let v10 = parameters[110];
            let v12 = parameters[76];
            let v15 = if parameter_given[77] { 1.0 } else { 0.0 };
            let v18 = 3.9e0f64;
            let v20 = parameters[78];
            let v22 = parameters[77];
            let v23 = parameters[0];
            let v24 = parameters[49];
            let v26 = parameters[1];
            let v27 = parameters[50];
            let v29 = parameters[51];
            let v32 = parameters[2];
            let v34 = parameters[53];
            let v37 = parameters[58];
            let v40 = parameters[59];
            let v44 = parameters[54];
            let v45 = parameters[55];
            let v48 = parameters[56];
            let v51 = parameters[57];
            let v54 = parameters[64];
            let v57 = parameters[65];
            let v61 = parameters[60];
            let v62 = parameters[61];
            let v65 = parameters[62];
            let v68 = parameters[63];
            let v71 = 2e0f64;
            let v75 = 1e-9f64;
            let v77 = parameters[1375];
            let v78 = parameters[1376];
            let v86 = parameters[66];
            let v87 = parameters[67];
            let v90 = parameters[68];
            let v93 = parameters[69];
            let v96 = parameters[70];
            let v97 = parameters[71];
            let v100 = parameters[72];
            let v103 = parameters[73];
            let v114 = parameters[927];
            let v127 = 1e-6f64;
            let v132 = parameters[48];
            let v134 = parameters[52];
            let v137 = parameters[1026];
            let v144 = parameters[1027];
            let v181 = parameters[1025];
            let v190 = parameters[115];
            let v191 = parameters[116];
            let v194 = parameters[117];
            let v197 = parameters[118];
            let v200 = parameters[119];
            let v201 = parameters[120];
            let v204 = parameters[121];
            let v207 = parameters[122];
            let v210 = parameters[129];
            let v211 = parameters[130];
            let v214 = parameters[131];
            let v217 = parameters[132];
            let v220 = parameters[142];
            let v221 = parameters[143];
            let v224 = parameters[144];
            let v227 = parameters[145];
            let v230 = parameters[79];
            let v231 = parameters[88];
            let v234 = parameters[89];
            let v237 = parameters[90];
            let v240 = parameters[91];
            let v241 = parameters[100];
            let v244 = parameters[101];
            let v247 = parameters[102];
            let v250 = parameters[103];
            let v251 = parameters[104];
            let v254 = parameters[105];
            let v257 = parameters[106];
            let v260 = parameters[232];
            let v261 = parameters[233];
            let v264 = parameters[234];
            let v267 = parameters[235];
            let v270 = parameters[236];
            let v271 = parameters[243];
            let v274 = parameters[244];
            let v277 = parameters[245];
            let v280 = parameters[246];
            let v281 = parameters[247];
            let v284 = parameters[248];
            let v287 = parameters[249];
            let v290 = parameters[250];
            let v291 = parameters[251];
            let v294 = parameters[252];
            let v297 = parameters[253];
            let v300 = parameters[170];
            let v301 = parameters[171];
            let v304 = parameters[172];
            let v307 = parameters[173];
            let v310 = parameters[174];
            let v311 = parameters[175];
            let v314 = parameters[176];
            let v317 = parameters[177];
            let v320 = parameters[178];
            let v321 = parameters[179];
            let v324 = parameters[180];
            let v327 = parameters[181];
            let v330 = parameters[186];
            let v331 = parameters[187];
            let v334 = parameters[188];
            let v337 = parameters[189];
            let v340 = parameters[182];
            let v341 = parameters[183];
            let v344 = parameters[184];
            let v347 = parameters[185];
            let v350 = parameters[254];
            let v351 = parameters[255];
            let v354 = parameters[256];
            let v357 = parameters[257];
            let v360 = parameters[258];
            let v361 = parameters[259];
            let v364 = parameters[260];
            let v367 = parameters[261];
            let v370 = parameters[262];
            let v371 = parameters[263];
            let v374 = parameters[264];
            let v377 = parameters[265];
            let v380 = parameters[1164];
            let v381 = parameters[1165];
            let v384 = parameters[1166];
            let v387 = parameters[1167];
            let v390 = parameters[1191];
            let v391 = parameters[1192];
            let v394 = parameters[1193];
            let v397 = parameters[1194];
            let v400 = parameters[288];
            let v401 = parameters[291];
            let v404 = parameters[292];
            let v407 = parameters[293];
            let v410 = parameters[270];
            let v411 = parameters[271];
            let v414 = parameters[272];
            let v417 = parameters[273];
            let v420 = parameters[1176];
            let v421 = parameters[1177];
            let v424 = parameters[1178];
            let v427 = parameters[1179];
            let v430 = parameters[275];
            let v431 = parameters[276];
            let v434 = parameters[277];
            let v437 = parameters[278];
            let v440 = parameters[146];
            let v441 = parameters[147];
            let v444 = parameters[148];
            let v447 = parameters[149];
            let v450 = parameters[1238];
            let v451 = parameters[1239];
            let v454 = parameters[1240];
            let v457 = parameters[1241];
            let v460 = parameters[150];
            let v461 = parameters[151];
            let v464 = parameters[152];
            let v467 = parameters[153];
            let v470 = parameters[1242];
            let v471 = parameters[1243];
            let v474 = parameters[1244];
            let v477 = parameters[1245];
            let v480 = parameters[154];
            let v481 = parameters[155];
            let v484 = parameters[156];
            let v487 = parameters[157];
            let v490 = parameters[158];
            let v491 = parameters[159];
            let v494 = parameters[160];
            let v497 = parameters[161];
            let v500 = parameters[162];
            let v501 = parameters[163];
            let v504 = parameters[164];
            let v507 = parameters[165];
            let v510 = parameters[166];
            let v511 = parameters[167];
            let v514 = parameters[168];
            let v517 = parameters[169];
            let v520 = parameters[1246];
            let v521 = parameters[1247];
            let v524 = parameters[1248];
            let v527 = parameters[1249];
            let v530 = parameters[1250];
            let v531 = parameters[1251];
            let v534 = parameters[1252];
            let v537 = parameters[1253];
            let v540 = parameters[1254];
            let v541 = parameters[1255];
            let v544 = parameters[1256];
            let v547 = parameters[1257];
            let v550 = parameters[1258];
            let v551 = parameters[1259];
            let v554 = parameters[1260];
            let v557 = parameters[1261];
            let v560 = parameters[218];
            let v561 = parameters[225];
            let v564 = parameters[226];
            let v567 = parameters[227];
            let v570 = parameters[208];
            let v571 = parameters[215];
            let v574 = parameters[216];
            let v577 = parameters[217];
            let v580 = parameters[1196];
            let v581 = parameters[1203];
            let v584 = parameters[1204];
            let v587 = parameters[1205];
            let v590 = parameters[111];
            let v591 = parameters[112];
            let v594 = parameters[113];
            let v597 = parameters[114];
            let v600 = parameters[190];
            let v601 = parameters[191];
            let v604 = parameters[192];
            let v607 = parameters[193];
            let v610 = parameters[194];
            let v611 = parameters[195];
            let v614 = parameters[196];
            let v617 = parameters[197];
            let v620 = parameters[203];
            let v621 = parameters[205];
            let v624 = parameters[206];
            let v627 = parameters[207];
            let v630 = parameters[309];
            let v631 = parameters[310];
            let v634 = parameters[311];
            let v637 = parameters[312];
            let v640 = parameters[337];
            let v641 = parameters[340];
            let v644 = parameters[341];
            let v647 = parameters[342];
            let v650 = parameters[348];
            let v651 = parameters[355];
            let v654 = parameters[356];
            let v657 = parameters[357];
            let v660 = parameters[372];
            let v661 = parameters[375];
            let v664 = parameters[376];
            let v667 = parameters[377];
            let v670 = parameters[362];
            let v671 = parameters[363];
            let v674 = parameters[364];
            let v677 = parameters[365];
            let v680 = parameters[382];
            let v681 = parameters[383];
            let v684 = parameters[384];
            let v687 = parameters[385];
            let v690 = parameters[390];
            let v691 = parameters[397];
            let v694 = parameters[398];
            let v697 = parameters[399];
            let v700 = parameters[404];
            let v701 = parameters[407];
            let v704 = parameters[408];
            let v707 = parameters[409];
            let v710 = parameters[415];
            let v711 = parameters[418];
            let v714 = parameters[419];
            let v717 = parameters[420];
            let v720 = parameters[457];
            let v721 = parameters[458];
            let v724 = parameters[459];
            let v727 = parameters[460];
            let v730 = parameters[467];
            let v731 = parameters[468];
            let v734 = parameters[469];
            let v737 = parameters[470];
            let v740 = parameters[439];
            let v741 = parameters[440];
            let v744 = parameters[441];
            let v747 = parameters[442];
            let v750 = parameters[443];
            let v751 = parameters[444];
            let v754 = parameters[445];
            let v757 = parameters[446];
            let v760 = parameters[449];
            let v761 = parameters[450];
            let v764 = parameters[451];
            let v767 = parameters[452];
            let v770 = parameters[453];
            let v771 = parameters[454];
            let v774 = parameters[455];
            let v777 = parameters[456];
            let v780 = parameters[463];
            let v781 = parameters[464];
            let v784 = parameters[465];
            let v787 = parameters[466];
            let v790 = parameters[477];
            let v791 = parameters[480];
            let v794 = parameters[481];
            let v797 = parameters[482];
            let v800 = parameters[473];
            let v801 = parameters[474];
            let v804 = parameters[475];
            let v807 = parameters[476];
            let v810 = parameters[498];
            let v811 = parameters[499];
            let v814 = parameters[500];
            let v817 = parameters[501];
            let v820 = parameters[530];
            let v821 = parameters[533];
            let v824 = parameters[534];
            let v827 = parameters[535];
            let v830 = parameters[540];
            let v831 = parameters[541];
            let v834 = parameters[542];
            let v837 = parameters[543];
            let v840 = parameters[421];
            let v841 = parameters[422];
            let v844 = parameters[423];
            let v847 = parameters[424];
            let v850 = parameters[425];
            let v851 = parameters[426];
            let v854 = parameters[427];
            let v857 = parameters[428];
            let v860 = parameters[429];
            let v861 = parameters[430];
            let v864 = parameters[431];
            let v867 = parameters[432];
            let v870 = parameters[434];
            let v871 = parameters[435];
            let v874 = parameters[436];
            let v877 = parameters[437];
            let v880 = parameters[548];
            let v881 = parameters[551];
            let v884 = parameters[552];
            let v887 = parameters[553];
            let v890 = parameters[544];
            let v891 = parameters[545];
            let v894 = parameters[546];
            let v897 = parameters[547];
            let v900 = parameters[295];
            let v901 = parameters[296];
            let v904 = parameters[297];
            let v907 = parameters[298];
            let v910 = parameters[510];
            let v911 = parameters[511];
            let v914 = parameters[512];
            let v917 = parameters[513];
            let v920 = parameters[325];
            let v921 = parameters[326];
            let v924 = parameters[327];
            let v927 = parameters[328];
            let v930 = parameters[329];
            let v931 = parameters[330];
            let v934 = parameters[331];
            let v937 = parameters[332];
            let v940 = parameters[483];
            let v941 = parameters[484];
            let v944 = parameters[485];
            let v947 = parameters[486];
            let v950 = parameters[315];
            let v951 = parameters[316];
            let v954 = parameters[317];
            let v957 = parameters[318];
            let v960 = parameters[883];
            let v961 = parameters[884];
            let v964 = parameters[885];
            let v967 = parameters[886];
            let v970 = parameters[887];
            let v971 = parameters[888];
            let v974 = parameters[889];
            let v977 = parameters[890];
            let v980 = parameters[601];
            let v981 = parameters[604];
            let v984 = parameters[605];
            let v987 = parameters[606];
            let v990 = parameters[607];
            let v991 = parameters[608];
            let v994 = parameters[609];
            let v997 = parameters[610];
            let v1000 = parameters[611];
            let v1001 = parameters[612];
            let v1004 = parameters[613];
            let v1007 = parameters[614];
            let v1010 = parameters[615];
            let v1011 = parameters[616];
            let v1014 = parameters[617];
            let v1017 = parameters[618];
            let v1020 = parameters[662];
            let v1021 = parameters[663];
            let v1024 = parameters[664];
            let v1027 = parameters[665];
            let v1030 = parameters[1361];
            let v1031 = parameters[1362];
            let v1034 = parameters[1363];
            let v1037 = parameters[1364];
            let v1040 = parameters[1365];
            let v1041 = parameters[1366];
            let v1044 = parameters[1367];
            let v1047 = parameters[1368];
            let v1050 = parameters[1369];
            let v1051 = parameters[1370];
            let v1054 = parameters[1371];
            let v1057 = parameters[1372];
            let v1060 = parameters[932];
            let v1061 = parameters[934];
            let v1064 = parameters[936];
            let v1067 = parameters[938];
            let v1070 = parameters[933];
            let v1071 = parameters[935];
            let v1074 = parameters[937];
            let v1077 = parameters[939];
            let v1080 = parameters[940];
            let v1081 = parameters[941];
            let v1084 = parameters[942];
            let v1087 = parameters[943];
            let v1090 = parameters[944];
            let v1091 = parameters[945];
            let v1094 = parameters[946];
            let v1097 = parameters[947];
            let v1100 = parameters[952];
            let v1101 = parameters[954];
            let v1104 = parameters[956];
            let v1107 = parameters[958];
            let v1110 = parameters[953];
            let v1111 = parameters[955];
            let v1114 = parameters[957];
            let v1117 = parameters[959];
            let v1120 = parameters[968];
            let v1121 = parameters[970];
            let v1124 = parameters[972];
            let v1127 = parameters[974];
            let v1130 = parameters[969];
            let v1131 = parameters[971];
            let v1134 = parameters[973];
            let v1137 = parameters[975];
            let v1140 = parameters[992];
            let v1141 = parameters[994];
            let v1144 = parameters[996];
            let v1147 = parameters[998];
            let v1150 = parameters[993];
            let v1151 = parameters[995];
            let v1154 = parameters[997];
            let v1157 = parameters[999];
            let v1160 = parameters[1000];
            let v1161 = parameters[1002];
            let v1164 = parameters[1004];
            let v1167 = parameters[1006];
            let v1170 = parameters[1001];
            let v1171 = parameters[1003];
            let v1174 = parameters[1005];
            let v1177 = parameters[1007];
            let v1180 = parameters[555];
            let v1181 = parameters[556];
            let v1184 = parameters[557];
            let v1187 = parameters[558];
            let v1190 = parameters[559];
            let v1191 = parameters[560];
            let v1194 = parameters[561];
            let v1197 = parameters[562];
            let v1200 = parameters[563];
            let v1201 = parameters[565];
            let v1204 = parameters[567];
            let v1207 = parameters[569];
            let v1210 = parameters[564];
            let v1211 = parameters[566];
            let v1214 = parameters[568];
            let v1217 = parameters[570];
            let v1220 = parameters[571];
            let v1221 = parameters[572];
            let v1224 = parameters[573];
            let v1227 = parameters[574];
            let v1230 = parameters[575];
            let v1231 = parameters[576];
            let v1234 = parameters[577];
            let v1237 = parameters[578];
            let v1240 = parameters[579];
            let v1241 = parameters[582];
            let v1244 = parameters[581];
            let v1247 = parameters[580];
            let v1250 = parameters[583];
            let v1251 = parameters[584];
            let v1254 = parameters[585];
            let v1257 = parameters[586];
            let v1260 = parameters[594];
            let v1261 = parameters[589];
            let v1264 = parameters[591];
            let v1267 = parameters[593];
            let v1270 = parameters[921];
            let v1271 = parameters[922];
            let v1274 = parameters[923];
            let v1277 = parameters[924];
            let v1280 = parameters[1125];
            let v1281 = parameters[1126];
            let v1284 = parameters[1127];
            let v1287 = parameters[1128];
            let v1290 = parameters[1129];
            let v1291 = parameters[1130];
            let v1294 = parameters[1131];
            let v1297 = parameters[1132];
            let v1300 = parameters[1133];
            let v1301 = parameters[1134];
            let v1304 = parameters[1135];
            let v1307 = parameters[1136];
            let v1310 = parameters[799];
            let v1311 = parameters[802];
            let v1314 = parameters[803];
            let v1317 = parameters[804];
            let v1320 = parameters[805];
            let v1321 = parameters[807];
            let v1324 = parameters[808];
            let v1327 = parameters[809];
            let v1330 = parameters[806];
            let v1331 = parameters[810];
            let v1334 = parameters[811];
            let v1337 = parameters[812];
            let v1340 = parameters[813];
            let v1341 = parameters[814];
            let v1344 = parameters[815];
            let v1347 = parameters[816];
            let v1350 = parameters[821];
            let v1351 = parameters[824];
            let v1354 = parameters[825];
            let v1357 = parameters[826];
            let v1360 = parameters[827];
            let v1361 = parameters[829];
            let v1364 = parameters[830];
            let v1367 = parameters[831];
            let v1370 = parameters[828];
            let v1371 = parameters[832];
            let v1374 = parameters[833];
            let v1377 = parameters[834];
            let v1380 = parameters[835];
            let v1381 = parameters[836];
            let v1384 = parameters[837];
            let v1387 = parameters[838];
            let v1390 = parameters[859];
            let v1391 = parameters[860];
            let v1394 = parameters[861];
            let v1397 = parameters[862];
            let v1400 = parameters[847];
            let v1401 = parameters[848];
            let v1404 = parameters[849];
            let v1407 = parameters[850];
            let v1410 = parameters[1032];
            let v1411 = parameters[1033];
            let v1414 = parameters[1034];
            let v1417 = parameters[1035];
            let v1420 = parameters[1037];
            let v1421 = parameters[1038];
            let v1424 = parameters[1039];
            let v1427 = parameters[1040];
            let v1430 = parameters[1042];
            let v1431 = parameters[1043];
            let v1434 = parameters[1044];
            let v1437 = parameters[1045];
            let v1440 = parameters[1046];
            let v1441 = parameters[1047];
            let v1444 = parameters[1048];
            let v1447 = parameters[1049];
            let v1450 = parameters[1051];
            let v1451 = parameters[1052];
            let v1454 = parameters[1053];
            let v1457 = parameters[1054];
            let v1460 = parameters[1055];
            let v1461 = parameters[1056];
            let v1464 = parameters[1057];
            let v1467 = parameters[1058];
            let v1470 = parameters[1060];
            let v1471 = parameters[1061];
            let v1474 = parameters[1062];
            let v1477 = parameters[1063];
            let v1480 = parameters[1064];
            let v1481 = parameters[1065];
            let v1484 = parameters[1066];
            let v1487 = parameters[1067];
            let v1490 = parameters[1070];
            let v1491 = parameters[1071];
            let v1494 = parameters[1072];
            let v1497 = parameters[1073];
            let v1500 = parameters[1085];
            let v1501 = parameters[1086];
            let v1504 = parameters[1087];
            let v1507 = parameters[1088];
            let v1510 = parameters[1089];
            let v1511 = parameters[1090];
            let v1514 = parameters[1091];
            let v1517 = parameters[1092];
            let v1520 = parameters[706];
            let v1521 = parameters[732];
            let v1524 = parameters[733];
            let v1527 = parameters[734];
            let v1530 = parameters[684];
            let v1531 = parameters[685];
            let v1534 = parameters[686];
            let v1537 = parameters[687];
            let v1540 = parameters[688];
            let v1541 = parameters[689];
            let v1544 = parameters[690];
            let v1547 = parameters[691];
            let v1550 = parameters[692];
            let v1551 = parameters[693];
            let v1554 = parameters[694];
            let v1557 = parameters[695];
            let v1560 = parameters[672];
            let v1561 = parameters[673];
            let v1564 = parameters[674];
            let v1567 = parameters[675];
            let v1570 = parameters[676];
            let v1571 = parameters[677];
            let v1574 = parameters[678];
            let v1577 = parameters[679];
            let v1580 = parameters[680];
            let v1581 = parameters[681];
            let v1584 = parameters[682];
            let v1587 = parameters[683];
            let v1590 = parameters[707];
            let v1591 = parameters[735];
            let v1594 = parameters[737];
            let v1597 = parameters[739];
            let v1600 = parameters[726];
            let v1601 = parameters[736];
            let v1604 = parameters[738];
            let v1607 = parameters[740];
            let v1610 = parameters[708];
            let v1611 = parameters[741];
            let v1614 = parameters[742];
            let v1617 = parameters[743];
            let v1620 = parameters[709];
            let v1621 = parameters[744];
            let v1624 = parameters[745];
            let v1627 = parameters[746];
            let v1630 = parameters[710];
            let v1631 = parameters[747];
            let v1634 = parameters[749];
            let v1637 = parameters[751];
            let v1640 = parameters[711];
            let v1641 = parameters[748];
            let v1644 = parameters[750];
            let v1647 = parameters[752];
            let v1650 = parameters[712];
            let v1651 = parameters[753];
            let v1654 = parameters[754];
            let v1657 = parameters[755];
            let v1660 = parameters[713];
            let v1661 = parameters[756];
            let v1664 = parameters[757];
            let v1667 = parameters[758];
            let v1670 = parameters[714];
            let v1671 = parameters[759];
            let v1674 = parameters[761];
            let v1677 = parameters[763];
            let v1680 = parameters[715];
            let v1681 = parameters[760];
            let v1684 = parameters[762];
            let v1687 = parameters[764];
            let v1690 = parameters[716];
            let v1691 = parameters[765];
            let v1694 = parameters[766];
            let v1697 = parameters[767];
            let v1700 = parameters[717];
            let v1701 = parameters[768];
            let v1704 = parameters[769];
            let v1707 = parameters[770];
            let v1710 = parameters[720];
            let v1711 = parameters[771];
            let v1714 = parameters[772];
            let v1717 = parameters[773];
            let v1720 = parameters[718];
            let v1721 = parameters[774];
            let v1724 = parameters[775];
            let v1727 = parameters[776];
            let v1730 = parameters[719];
            let v1731 = parameters[777];
            let v1734 = parameters[778];
            let v1737 = parameters[779];
            let v1740 = parameters[721];
            let v1741 = parameters[780];
            let v1744 = parameters[781];
            let v1747 = parameters[782];
            let v1750 = parameters[1075];
            let v1751 = parameters[1078];
            let v1754 = parameters[1079];
            let v1757 = parameters[1080];
            let v1760 = parameters[1081];
            let v1761 = parameters[1082];
            let v1764 = parameters[1083];
            let v1767 = parameters[1084];
            let v1770 = parameters[489];
            let v1771 = parameters[494];
            let v1774 = parameters[495];
            let v1777 = parameters[496];
            let v1780 = parameters[514];
            let v1781 = parameters[515];
            let v1784 = parameters[516];
            let v1787 = parameters[517];
            let v1790 = parameters[518];
            let v1791 = parameters[519];
            let v1794 = parameters[520];
            let v1797 = parameters[521];
            let v1800 = parameters[522];
            let v1801 = parameters[523];
            let v1804 = parameters[524];
            let v1807 = parameters[525];
            let v1810 = parameters[526];
            let v1811 = parameters[527];
            let v1814 = parameters[528];
            let v1817 = parameters[529];
            let v1820 = parameters[1300];
            let v1821 = parameters[1301];
            let v1824 = parameters[1302];
            let v1827 = parameters[1303];
            let v1830 = parameters[1308];
            let v1831 = parameters[1309];
            let v1834 = parameters[1310];
            let v1837 = parameters[1311];
            let v1840 = parameters[1304];
            let v1841 = parameters[1305];
            let v1844 = parameters[1306];
            let v1847 = parameters[1307];
            let v1850 = parameters[1312];
            let v1851 = parameters[1313];
            let v1854 = parameters[1314];
            let v1857 = parameters[1315];
            let v1860 = parameters[1156];
            let v1861 = parameters[1157];
            let v1864 = parameters[1158];
            let v1867 = parameters[1159];
            let v1870 = parameters[1152];
            let v1871 = parameters[1153];
            let v1874 = parameters[1154];
            let v1877 = parameters[1155];
            let v1880 = parameters[1160];
            let v1881 = parameters[1161];
            let v1884 = parameters[1162];
            let v1887 = parameters[1163];
            let v1890 = parameters[1168];
            let v1891 = parameters[1169];
            let v1894 = parameters[1170];
            let v1897 = parameters[1171];
            let v1900 = parameters[1186];
            let v1901 = parameters[1187];
            let v1904 = parameters[1188];
            let v1907 = parameters[1189];
            let v1910 = parameters[1206];
            let v1911 = parameters[1207];
            let v1914 = parameters[1208];
            let v1917 = parameters[1209];
            let v1920 = parameters[1210];
            let v1921 = parameters[1211];
            let v1924 = parameters[1212];
            let v1927 = parameters[1213];
            let v1930 = parameters[1214];
            let v1931 = parameters[1215];
            let v1934 = parameters[1216];
            let v1937 = parameters[1217];
            let v1940 = parameters[1218];
            let v1941 = parameters[1219];
            let v1944 = parameters[1220];
            let v1947 = parameters[1221];
            let v1950 = parameters[1222];
            let v1951 = parameters[1223];
            let v1954 = parameters[1224];
            let v1957 = parameters[1225];
            let v1960 = parameters[1226];
            let v1961 = parameters[1227];
            let v1964 = parameters[1228];
            let v1967 = parameters[1229];
            let v1970 = parameters[1230];
            let v1971 = parameters[1231];
            let v1974 = parameters[1232];
            let v1977 = parameters[1233];
            let v1980 = parameters[1234];
            let v1981 = parameters[1235];
            let v1984 = parameters[1236];
            let v1987 = parameters[1237];
            let v1990 = parameters[1265];
            let v1991 = parameters[1272];
            let v1994 = parameters[1273];
            let v1997 = parameters[1274];
            let v2000 = parameters[1275];
            let v2001 = parameters[1276];
            let v2004 = parameters[1277];
            let v2007 = parameters[1278];
            let v2010 = parameters[1283];
            let v2011 = parameters[1284];
            let v2014 = parameters[1285];
            let v2017 = parameters[1286];
            let v2020 = parameters[1279];
            let v2021 = parameters[1280];
            let v2024 = parameters[1281];
            let v2027 = parameters[1282];
            let v2030 = parameters[1287];
            let v2031 = parameters[1288];
            let v2034 = parameters[1289];
            let v2037 = parameters[1290];
            let v2040 = parameters[1291];
            let v2041 = parameters[1292];
            let v2044 = parameters[1293];
            let v2047 = parameters[1294];
            let v2050 = parameters[1323];
            let v2051 = parameters[1324];
            let v2054 = parameters[1325];
            let v2057 = parameters[1326];
            let v2060 = parameters[1327];
            let v2061 = parameters[1328];
            let v2064 = parameters[1329];
            let v2067 = parameters[1330];
            let v2070 = parameters[1331];
            let v2071 = parameters[1332];
            let v2074 = parameters[1333];
            let v2077 = parameters[1334];
            let v2080 = parameters[1335];
            let v2081 = parameters[1336];
            let v2084 = parameters[1337];
            let v2087 = parameters[1338];
            let v2090 = parameters[1339];
            let v2091 = parameters[1340];
            let v2094 = parameters[1341];
            let v2097 = parameters[1342];
            let v2100 = parameters[1343];
            let v2101 = parameters[1344];
            let v2104 = parameters[1345];
            let v2107 = parameters[1346];
            let v2110 = parameters[1384];
            let v2111 = parameters[1385];
            let v2114 = parameters[1386];
            let v2117 = parameters[1387];
            let v2120 = parameters[1389];
            let v2121 = parameters[1390];
            let v2124 = parameters[1391];
            let v2127 = parameters[1392];
            let v2130 = parameters[35];
            let v2132 = parameters[1172];
            let v2133 = parameters[1173];
            let v2136 = parameters[1174];
            let v2139 = parameters[1175];
            let v2142 = parameters[284];
            let v2143 = parameters[285];
            let v2146 = parameters[286];
            let v2149 = parameters[287];
            let v2152 = parameters[198];
            let v2153 = parameters[199];
            let v2156 = parameters[200];
            let v2159 = parameters[201];
            let v2162 = parameters[343];
            let v2163 = parameters[344];
            let v2166 = parameters[345];
            let v2169 = parameters[346];
            let v2172 = parameters[358];
            let v2173 = parameters[359];
            let v2176 = parameters[360];
            let v2179 = parameters[361];
            let v2182 = parameters[378];
            let v2183 = parameters[379];
            let v2186 = parameters[380];
            let v2189 = parameters[381];
            let v2192 = parameters[386];
            let v2193 = parameters[387];
            let v2196 = parameters[388];
            let v2199 = parameters[389];
            let v2202 = parameters[400];
            let v2203 = parameters[401];
            let v2206 = parameters[402];
            let v2209 = parameters[403];
            let v2212 = parameters[410];
            let v2213 = parameters[411];
            let v2216 = parameters[412];
            let v2219 = parameters[413];
            let v2222 = parameters[536];
            let v2223 = parameters[537];
            let v2226 = parameters[538];
            let v2229 = parameters[539];
            let v2232 = parameters[305];
            let v2233 = parameters[306];
            let v2236 = parameters[307];
            let v2239 = parameters[308];
            let v2242 = parameters[490];
            let v2243 = parameters[491];
            let v2246 = parameters[492];
            let v2249 = parameters[493];
            let v2252 = parameters[506];
            let v2253 = parameters[507];
            let v2256 = parameters[508];
            let v2259 = parameters[509];
            let v2262 = parameters[80];
            let v2263 = parameters[81];
            let v2269 = parameters[82];
            let v2270 = parameters[83];
            let v2277 = parameters[84];
            let v2278 = parameters[85];
            let v2284 = parameters[86];
            let v2285 = parameters[87];
            let v2292 = parameters[237];
            let v2293 = parameters[238];
            let v2299 = parameters[239];
            let v2300 = parameters[240];
            let v2306 = parameters[241];
            let v2307 = parameters[242];
            let v2314 = parameters[282];
            let v2315 = parameters[283];
            let v2327 = parameters[289];
            let v2328 = parameters[290];
            let v2336 = parameters[24];
            let v2338 = parameters[42];
            let v2340 = parameters[339];
            let v2342 = parameters[338];
            let v2355 = parameters[333];
            let v2357 = parameters[334];
            let v2362 = parameters[335];
            let v2363 = parameters[336];
            let v2370 = parameters[349];
            let v2371 = parameters[350];
            let v2377 = parameters[351];
            let v2378 = parameters[352];
            let v2384 = parameters[353];
            let v2385 = parameters[354];
            let v2394 = parameters[366];
            let v2395 = parameters[367];
            let v2401 = parameters[368];
            let v2402 = parameters[369];
            let v2408 = parameters[370];
            let v2409 = parameters[371];
            let v2416 = parameters[373];
            let v2417 = parameters[374];
            let v2427 = parameters[391];
            let v2428 = parameters[392];
            let v2434 = parameters[393];
            let v2435 = parameters[394];
            let v2441 = parameters[395];
            let v2442 = parameters[396];
            let v2451 = parameters[202];
            let v2459 = parameters[204];
            let v2465 = parameters[531];
            let v2466 = parameters[532];
            let v2476 = parameters[313];
            let v2477 = parameters[314];
            let v2485 = 5e-1f64;
            let v2487 = parameters[549];
            let v2488 = parameters[550];
            let v2496 = parameters[405];
            let v2497 = parameters[406];
            let v2509 = parameters[299];
            let v2510 = parameters[300];
            let v2516 = parameters[301];
            let v2517 = parameters[302];
            let v2523 = parameters[303];
            let v2524 = parameters[304];
            let v2533 = parameters[487];
            let v2534 = parameters[488];
            let v2542 = 2.5e-1f64;
            let v2547 = parameters[502];
            let v2548 = parameters[505];
            let v2558 = parameters[602];
            let v2559 = parameters[603];
            let v2567 = parameters[800];
            let v2570 = parameters[801];
            let v2574 = parameters[822];
            let v2577 = parameters[823];
            let v2581 = parameters[724];
            let v2584 = parameters[725];
            let v2588 = parameters[727];
            let v2591 = parameters[728];
            let v2595 = parameters[729];
            let v2598 = parameters[730];
            let v2602 = parameters[723];
            let v2603 = parameters[731];
            let v2607 = parameters[92];
            let v2608 = parameters[93];
            let v2614 = parameters[94];
            let v2615 = parameters[95];
            let v2622 = parameters[96];
            let v2623 = parameters[97];
            let v2629 = parameters[98];
            let v2631 = parameters[99];
            let v2638 = parameters[29];
            let v2640 = parameters[123];
            let v2641 = parameters[124];
            let v2647 = parameters[125];
            let v2648 = parameters[126];
            let v2654 = parameters[127];
            let v2655 = parameters[128];
            let v2662 = parameters[133];
            let v2663 = parameters[134];
            let v2669 = parameters[135];
            let v2670 = parameters[136];
            let v2676 = parameters[137];
            let v2677 = parameters[138];
            let v2684 = parameters[319];
            let v2685 = parameters[320];
            let v2691 = parameters[321];
            let v2692 = parameters[322];
            let v2698 = parameters[323];
            let v2699 = parameters[324];
            let v2706 = parameters[416];
            let v2707 = parameters[417];
            let v2716 = parameters[209];
            let v2717 = parameters[210];
            let v2723 = parameters[211];
            let v2724 = parameters[212];
            let v2730 = parameters[213];
            let v2731 = parameters[214];
            let v2738 = parameters[1197];
            let v2739 = parameters[1198];
            let v2745 = parameters[1199];
            let v2746 = parameters[1200];
            let v2752 = parameters[1201];
            let v2753 = parameters[1202];
            let v2760 = parameters[219];
            let v2761 = parameters[220];
            let v2767 = parameters[221];
            let v2768 = parameters[222];
            let v2774 = parameters[223];
            let v2775 = parameters[224];
            let v2782 = parameters[1266];
            let v2783 = parameters[1267];
            let v2789 = parameters[1268];
            let v2790 = parameters[1269];
            let v2796 = parameters[1270];
            let v2797 = parameters[1271];
            let v2804 = parameters[447];
            let v2805 = parameters[448];
            let v2813 = parameters[1036];
            let v2817 = parameters[1041];
            let v2821 = parameters[1050];
            let v2825 = parameters[1068];
            let v2829 = parameters[1074];
            let v2833 = parameters[33];
            let v2835 = parameters[461];
            let v2836 = parameters[462];
            let v2844 = parameters[471];
            let v2845 = parameters[472];
            let v2853 = parameters[478];
            let v2854 = parameters[479];
            let v2880 = parameters[141];
            let v2885 = parameters[37];
            let v2899 = 6.7e-2f64;
            let v2908 = 1e1f64;
            let v2910 = parameters[1396];
            let v2912 = parameters[895];
            let v2913 = parameters[898];
            let v2915 = parameters[896];
            let v2916 = parameters[897];
            let v2918 = if parameter_given[3] { 1.0 } else { 0.0 };
            let v2919 = parameters[438];
            let v2920 = parameters[3];
            let v2922 = parameters[9];
            let v2926 = parameters[8];
            let v2927 = 9e0f64;
            let v2935 = parameters[6];
            let v2945 = 1.0f64;
            let v2959 = 1.0f64;
            let v2960 = 1.0f64;
            let v2964 = 5e0f64;
            let v2974 = 3e0f64;
            let v2976 = 4e0f64;
            let v2979 = 6e0f64;
            let v2993 = 7e0f64;
            let v3003 = 8e0f64;
            let v3014 = 0.0f64;
            let v3063 = 1.0f64;
            let v3064 = 1.0f64;
            let v3109 = 0.0f64;
            let v3153 = 1.0f64;
            let v3154 = 1.0f64;
            let v3197 = 0.0f64;
            let v3243 = 1.0f64;
            let v3244 = 1.0f64;
            let v3287 = 0.0f64;
            let v3331 = 1.0f64;
            let v3332 = 1.0f64;
            let v3380 = 1.0f64;
            let v3381 = 1.0f64;
            let v3429 = 1.0f64;
            let v3432 = 0.0f64;
            let v3478 = 1.0f64;
            let v3483 = 0.0f64;
            let v3530 = 1.0f64;
            let v3543 = 1.0f64;
            let v3713 = if parameter_given[4] { 1.0 } else { 0.0 };
            let v3714 = parameters[4];
            let v3735 = 0.0f64;
            let v3749 = 0.0f64;
            let v3750 = 1.0f64;
            let v3800 = 0.0f64;
            let v3851 = 0.0f64;
            let v3852 = 1.0f64;
            let v3897 = 0.0f64;
            let v3941 = 0.0f64;
            let v3942 = 1.0f64;
            let v3985 = 0.0f64;
            let v4031 = 0.0f64;
            let v4032 = 1.0f64;
            let v4075 = 0.0f64;
            let v4119 = 0.0f64;
            let v4120 = 1.0f64;
            let v4168 = 0.0f64;
            let v4169 = 1.0f64;
            let v4217 = 0.0f64;
            let v4220 = 0.0f64;
            let v4266 = 0.0f64;
            let v4271 = 0.0f64;
            let v4318 = 0.0f64;
            let v4331 = 0.0f64;
            let v4508 = parameters[1347];
            let v4524 = parameters[900];
            let v4525 = parameters[21];
            let v4527 = parameters[22];
            let v4532 = parameters[899];
            let v4538 = 1e3f64;
            let v4539 = parameters[7];
            let v4544 = parameters[722];
            let v4546 = 1e-38f64;
            let v4558 = parameters[703];
            let v4559 = parameters[702];
            let v4561 = parameters[705];
            let v4562 = parameters[704];
            let v4564 = parameters[1373];
            let v4566 = parameters[1378];
            let v4570 = parameters[1377];
            let v4578 = parameters[1381];
            let v4583 = parameters[1101];
            let v4585 = parameters[41];
            let v4587 = parameters[1099];
            let v4592 = parameters[40];
            let v4593 = 0.0f64;
            let v4595 = parameters[1028];
            let v4596 = 2.7315e2f64;
            let v4597 = -2.7315e2f64;
            let v4599 = 3.0015e2f64;
            let v4601 = temperature;
            let v4602 = parameters[23];
            let v4605 = 0.0f64;
            let v4607 = node_potentials[4];
            let v4608 = node_potentials[5];
            let v4619 = parameters[108];
            let v4620 = parameters[1029];
            let v4623 = parameters[1030];
            let v4629 = parameters[107];
            let v4668 = parameters[5];
            let v4670 = 0.0f64;
            let v4671 = parameters[43];
            let v4673 = parameters[45];
            let v4675 = 0.0f64;
            let v4682 = 1.0f64;
            let v4685 = 0.0f64;
            let v4690 = 4e-1f64;
            let v4705 = parameters[1031];
            let v4710 = 1e-3f64;
            let v4711 = 4e-6f64;
            let v4717 = parameters[1059];
            let v4724 = 3.333333333333333e-1f64;
            let v4725 = parameters[347];
            let v4737 = 4e-6f64;
            let v4747 = 4e-6f64;
            let v4763 = 4e-6f64;
            let v4776 = 4e-6f64;
            let v4783 = 4e-6f64;
            let v4799 = 1e2f64;
            let v4807 = parameters[1069];
            let v4813 = 4e-6f64;
            let v4824 = 4e-6f64;
            let v4831 = 4e-6f64;
            let v4841 = 4e-6f64;
            let v4851 = 4e-6f64;
            let v4871 = 4e-6f64;
            let v4882 = 4e-6f64;
            let v4892 = 4e-6f64;
            let v4902 = 4e-6f64;
            let v4912 = 4e-6f64;
            let v4918 = parameters[901];
            let v4919 = parameters[1093];
            let v4924 = 4e-6f64;
            let v4930 = parameters[902];
            let v4931 = 4e-6f64;
            let v4937 = parameters[903];
            let v4938 = parameters[1094];
            let v4943 = 4e-6f64;
            let v4949 = parameters[904];
            let v4950 = 4e-6f64;
            let v4956 = parameters[905];
            let v4957 = parameters[1095];
            let v4962 = 4e-6f64;
            let v4968 = parameters[906];
            let v4969 = 4e-6f64;
            let v4975 = parameters[907];
            let v4976 = parameters[1096];
            let v4979 = 1e-2f64;
            let v4982 = 4e-6f64;
            let v4988 = parameters[908];
            let v4992 = 4e-6f64;
            let v4998 = parameters[909];
            let v4999 = parameters[1097];
            let v5004 = 4e-6f64;
            let v5010 = parameters[910];
            let v5014 = 4e-6f64;
            let v5020 = parameters[911];
            let v5021 = parameters[1098];
            let v5026 = 4e-6f64;
            let v5032 = parameters[912];
            let v5036 = 4e-6f64;
            let v5209 = if parameter_given[17] { 1.0 } else { 0.0 };
            let v5210 = parameters[17];
            let v5226 = if parameter_given[18] { 1.0 } else { 0.0 };
            let v5227 = parameters[18];
            let v5243 = if parameter_given[19] { 1.0 } else { 0.0 };
            let v5244 = parameters[926];
            let v5246 = parameters[19];
            let v5264 = if parameter_given[20] { 1.0 } else { 0.0 };
            let v5266 = parameters[20];
            let v5284 = parameters[10];
            let v5286 = parameters[11];
            let v5291 = parameters[12];
            let v5296 = parameters[1111];
            let v5298 = parameters[1104];
            let v5300 = parameters[1112];
            let v5302 = parameters[1108];
            let v5304 = parameters[1109];
            let v5307 = parameters[1110];
            let v5312 = parameters[1117];
            let v5314 = parameters[1118];
            let v5316 = parameters[1114];
            let v5318 = parameters[1115];
            let v5321 = parameters[1116];
            let v5326 = parameters[1107];
            let v5348 = parameters[1102];
            let v5352 = parameters[1103];
            let v5356 = parameters[1105];
            let v5364 = parameters[1106];
            let v5370 = parameters[1113];
            let v5374 = parameters[1119];
            let v5375 = parameters[1120];
            let v5379 = parameters[1121];
            let v5380 = parameters[1122];
            let v5389 = parameters[27];
            let v5401 = parameters[34];
            let v5404 = parameters[13];
            let v5405 = parameters[14];
            let v5406 = parameters[15];
            let v5407 = if parameter_given[13] { 1.0 } else { 0.0 };
            let v5409 = if parameter_given[14] { 1.0 } else { 0.0 };
            let v5412 = if parameter_given[15] { 1.0 } else { 0.0 };
            let v5415 = if parameter_given[16] { 1.0 } else { 0.0 };
            let v5416 = parameters[16];
            let v5420 = parameters[1137];
            let v5425 = 1e-1f64;
            let v5429 = -1e1f64;
            let v5436 = -1e1f64;
            let v5443 = 5e-2f64;
            let v5445 = 2.5e-3f64;
            let v5448 = 2e1f64;
            let v5449 = -2e1f64;
            let v5456 = -2e1f64;
            let v5466 = parameters[1123];
            let v5472 = parameters[1124];
            let v5490 = node_potentials[8];
            let v5491 = node_potentials[10];
            let v5494 = node_potentials[11];
            let v5497 = node_potentials[6];
            let v5500 = node_potentials[7];
            let v5512 = node_potentials[3];
            let v5522 = -1e0f64;
            let v5529 = parameters[1146];
            let v5531 = 8e1f64;
            let v5540 = 6.931471805599453e-1f64;
            let v5553 = 6e-1f64;
            let v5613 = parameters[74];
            let v5618 = 3.75e-1f64;
            let v5623 = parameters[75];
            let v5638 = 3.141592653589793e0f64;
            let v5647 = 4e1f64;
            let v5660 = 2.5000000000000005e-3f64;
            let v5679 = parameters[266];
            let v5682 = parameters[267];
            let v5686 = parameters[268];
            let v5688 = parameters[269];
            let v5692 = parameters[280];
            let v5695 = parameters[281];
            let v5702 = parameters[274];
            let v5708 = parameters[279];
            let v5724 = 6.250000000000001e-4f64;
            let v5737 = 6.25e-10f64;
            let v5742 = 1.25e-5f64;
            let v5744 = parameters[1077];
            let v5749 = parameters[1076];
            let v5756 = -8e1f64;
            let v5758 = 1.804851387e-35f64;
            let v5777 = parameters[25];
            let v5798 = 3.204352924e-19f64;
            let v5808 = 4e-6f64;
            let v5819 = 3.204352924e-19f64;
            let v5832 = 7.071067811865475e-1f64;
            let v5835 = 1e-7f64;
            let v5837 = 1.25e0f64;
            let v5840 = 7.324648775608221e-1f64;
            let v5848 = 8.485281374238571e0f64;
            let v5863 = 6.4e1f64;
            let v5967 = 8.485281374238571e0f64;
            let v6022 = parameters[294];
            let v6039 = 1.6666666666666666e-1f64;
            let v6052 = 1.25e0f64;
            let v6103 = 1.2e1f64;
            let v6194 = 1e-40f64;
            let v6473 = 6.4e-7f64;
            let v6476 = 8e-4f64;
            let v6665 = -5e0f64;
            let v6689 = 3.7e1f64;
            let v6691 = -3.7e1f64;
            let v6698 = -3.7e1f64;
            let v6750 = -2e0f64;
            let v6823 = -5e0f64;
            let v6848 = -3.7e1f64;
            let v6855 = -3.7e1f64;
            let v6907 = -2e0f64;
            let v6980 = -5e0f64;
            let v7005 = -3.7e1f64;
            let v7012 = -3.7e1f64;
            let v7064 = -2e0f64;
            let v7129 = 3.912023005e0f64;
            let v7159 = 1e-5f64;
            let v7160 = 4.0000000000000007e-10f64;
            let v7176 = 1e-8f64;
            let v7202 = 5.625e-7f64;
            let v7207 = 1e6f64;
            let v7268 = parameters[1349];
            let v7270 = parameters[1350];
            let v7279 = parameters[1351];
            let v7284 = parameters[1352];
            let v7292 = 6.25e-8f64;
            let v7300 = 4.0000000000000007e-10f64;
            let v7932 = -5e0f64;
            let v7957 = -3.7e1f64;
            let v7964 = -3.7e1f64;
            let v8016 = -2e0f64;
            let v8089 = -5e0f64;
            let v8114 = -3.7e1f64;
            let v8121 = -3.7e1f64;
            let v8173 = -2e0f64;
            let v8246 = -5e0f64;
            let v8271 = -3.7e1f64;
            let v8278 = -3.7e1f64;
            let v8330 = -2e0f64;
            let v8399 = 1e-10f64;
            let v8415 = -3.7e1f64;
            let v8422 = -3.7e1f64;
            let v8467 = -2e0f64;
            let v8507 = -2e0f64;
            let v8570 = 4.0000000000000007e-10f64;
            let v8592 = 1.25e-1f64;
            let v8608 = 4.0000000000000007e-10f64;
            let v8615 = parameters[46];
            let v8684 = 4.0000000000000007e-10f64;
            let v8693 = 1e-35f64;
            let v8725 = 5.625e-7f64;
            let v8750 = 4e-6f64;
            let v8772 = parameters[414];
            let v8812 = parameters[433];
            let v8819 = 5.540622384e34f64;
            let v8850 = parameters[503];
            let v8860 = parameters[504];
            let v8879 = -1e0f64;
            let v8984 = -1e0f64;
            let v8986 = 4e-3f64;
            let v8999 = 2.5e-5f64;
            let v9004 = 2.5e-3f64;
            let v9029 = 4.0000000000000007e-10f64;
            let v9075 = parameters[1009];
            let v9081 = parameters[1008];
            let v9100 = 1.115e0f64;
            let v9134 = parameters[595];
            let v9143 = parameters[920];
            let v9156 = parameters[554];
            let v9179 = parameters[36];
            let v9186 = 1e-4f64;
            let v9200 = parameters[44];
            let v9224 = parameters[666];
            let v9241 = parameters[913];
            let v9245 = 3.8025850929940455e0f64;
            let v9254 = parameters[915];
            let v9258 = 3.8025850929940455e0f64;
            let v9267 = parameters[917];
            let v9271 = 3.8025850929940455e0f64;
            let v9282 = 9e-1f64;
            let v9380 = parameters[919];
            let v9404 = parameters[914];
            let v9408 = 3.8025850929940455e0f64;
            let v9417 = parameters[916];
            let v9421 = 3.8025850929940455e0f64;
            let v9430 = parameters[918];
            let v9434 = 3.8025850929940455e0f64;
            let v9586 = 2e-1f64;
            let v9601 = 2.5000000000000005e-3f64;
            let v9609 = parameters[231];
            let v9612 = parameters[230];
            let v9615 = 4e-6f64;
            let v9620 = 7e-1f64;
            let v9621 = parameters[229];
            let v9625 = parameters[228];
            let v9626 = 1.9e-9f64;
            let v9629 = 3.453133246992e-11f64;
            let v9638 = parameters[1379];
            let v9654 = if parameter_given[867] { 1.0 } else { 0.0 };
            let v9657 = parameters[32];
            let v9659 = parameters[1394];
            let v9661 = parameters[1393];
            let v9681 = 3.453133e-11f64;
            let v9683 = parameters[1388];
            let v9688 = parameters[1382];
            let v9694 = parameters[47];
            let v9702 = parameters[140];
            let v9712 = 3.204352924e-19f64;
            let v9727 = 1.25e0f64;
            let v9737 = 8.485281374238571e0f64;
            let v9855 = 8.485281374238571e0f64;
            let v10535 = -5e0f64;
            let v10557 = -3.7e1f64;
            let v10567 = -3.7e1f64;
            let v10619 = -2e0f64;
            let v10692 = -5e0f64;
            let v10714 = -3.7e1f64;
            let v10724 = -3.7e1f64;
            let v10776 = -2e0f64;
            let v10849 = -5e0f64;
            let v10871 = -3.7e1f64;
            let v10881 = -3.7e1f64;
            let v10933 = -2e0f64;
            let v11021 = 4.0000000000000007e-10f64;
            let v11058 = 5.625e-7f64;
            let v11071 = 2.5000000000000005e-3f64;
            let v11147 = 6.25e-8f64;
            let v11155 = 4.0000000000000007e-10f64;
            let v11787 = -5e0f64;
            let v11809 = -3.7e1f64;
            let v11819 = -3.7e1f64;
            let v11871 = -2e0f64;
            let v11944 = -5e0f64;
            let v11966 = -3.7e1f64;
            let v11976 = -3.7e1f64;
            let v12028 = -2e0f64;
            let v12101 = -5e0f64;
            let v12123 = -3.7e1f64;
            let v12133 = -3.7e1f64;
            let v12185 = -2e0f64;
            let v12260 = -3.7e1f64;
            let v12277 = -3.7e1f64;
            let v12322 = -2e0f64;
            let v12362 = -2e0f64;
            let v12425 = 4.0000000000000007e-10f64;
            let v12462 = 4.0000000000000007e-10f64;
            let v12482 = 4.0000000000000007e-10f64;
            let v12516 = parameters[1380];
            let v12543 = parameters[38];
            let v12562 = parameters[671];
            let v12566 = -3.7e1f64;
            let v12575 = parameters[696];
            let v12584 = parameters[700];
            let v12587 = parameters[701];
            let v12602 = -3.7e1f64;
            let v12611 = parameters[697];
            let v12617 = parameters[698];
            let v12620 = parameters[699];
            let v12634 = 1.0f64;
            let v12636 = 0.0f64;
            let v12680 = 2e-4f64;
            let v12694 = parameters[1295];
            let v12699 = 4e-12f64;
            let v12729 = 4e-12f64;
            let v12767 = parameters[1011];
            let v12793 = 4.112737976006692e-57f64;
            let v12801 = parameters[1012];
            let v12802 = parameters[1013];
            let v12805 = parameters[1014];
            let v12813 = parameters[1015];
            let v12825 = 1e10f64;
            let v12873 = parameters[1016];
            let v12875 = parameters[1017];
            let v12883 = parameters[1010];
            let v12887 = parameters[1019];
            let v12888 = parameters[1022];
            let v12893 = parameters[1020];
            let v12894 = parameters[1023];
            let v12899 = parameters[1297];
            let v12900 = parameters[1298];
            let v12905 = parameters[1021];
            let v12906 = parameters[1024];
            let v12915 = parameters[1296];
            let v12924 = parameters[39];
            let v12943 = parameters[1018];
            let v12972 = parameters[1299];
            let v12980 = 2.5000000000000005e-3f64;
            let v12997 = 6e1f64;
            let v13003 = 1.44e2f64;
            let v13009 = 1.5e1f64;
            let v13023 = 3.95e-1f64;
            let v13041 = 3.204352924e-19f64;
            let v13045 = 3.204352924e-19f64;
            let v13049 = 3.204352924e-19f64;
            let v13067 = 4e-6f64;
            let v13081 = 2.5000000000000005e-3f64;
            let v13092 = parameters[1183];
            let v13095 = parameters[1195];
            let v13099 = parameters[1181];
            let v13101 = parameters[1182];
            let v13105 = parameters[1184];
            let v13108 = parameters[1185];
            let v13114 = parameters[1180];
            let v13120 = parameters[1190];
            let v13135 = 6.250000000000001e-4f64;
            let v13162 = parameters[1264];
            let v13167 = parameters[1263];
            let v13171 = parameters[1262];
            let v13187 = -8e1f64;
            let v13211 = parameters[1151];
            let v13221 = 3.204352924e-19f64;
            let v13227 = parameters[1148];
            let v13228 = parameters[1149];
            let v13229 = parameters[1150];
            let v13244 = 1.25e0f64;
            let v13254 = 8.485281374238571e0f64;
            let v13372 = 8.485281374238571e0f64;
            let v14054 = -5e0f64;
            let v14079 = -3.7e1f64;
            let v14086 = -3.7e1f64;
            let v14138 = -2e0f64;
            let v14211 = -5e0f64;
            let v14236 = -3.7e1f64;
            let v14243 = -3.7e1f64;
            let v14295 = -2e0f64;
            let v14368 = -5e0f64;
            let v14393 = -3.7e1f64;
            let v14400 = -3.7e1f64;
            let v14452 = -2e0f64;
            let v14537 = 4.0000000000000007e-10f64;
            let v14574 = 5.625e-7f64;
            let v14652 = 6.25e-8f64;
            let v14660 = 4.0000000000000007e-10f64;
            let v15292 = -5e0f64;
            let v15317 = -3.7e1f64;
            let v15324 = -3.7e1f64;
            let v15376 = -2e0f64;
            let v15449 = -5e0f64;
            let v15474 = -3.7e1f64;
            let v15481 = -3.7e1f64;
            let v15533 = -2e0f64;
            let v15606 = -5e0f64;
            let v15631 = -3.7e1f64;
            let v15638 = -3.7e1f64;
            let v15690 = -2e0f64;
            let v15776 = -3.7e1f64;
            let v15783 = -3.7e1f64;
            let v15828 = -2e0f64;
            let v15868 = -2e0f64;
            let v15947 = 4.0000000000000007e-10f64;
            let v16022 = parameters[1147];
            let v16025 = 1.4142135623730951e0f64;
            let v16060 = 1e0f64;
            let v16078 = 2.01491e-1f64;
            let v16080 = 4.02982e-1f64;
            let v16083 = 2.446562e0f64;
            let v16088 = -6.8e1f64;
            let v16090 = -1e2f64;
            let v16091 = -1.1e2f64;
            let v16093 = 1.804851387e-35f64;
            let v16094 = -9e1f64;
            let v16100 = 7.8125e-2f64;
            let v16103 = 9.375e-1f64;
            let v16175 = 1e0f64;
            let v16187 = 1e0f64;
            let v16203 = 2.5000000000000005e-3f64;
            let v16228 = 5.625e-7f64;
            let v16271 = parameters[497];
            let v16343 = -2e0f64;
            let v16350 = -2e0f64;
            let v16391 = -2e0f64;
            let v16398 = -2e0f64;
            let v16443 = -2e0f64;
            let v16450 = -2e0f64;
            let v16482 = -2e0f64;
            let v16489 = -2e0f64;
            let v16534 = 6.25e-8f64;
            let v16541 = 2.5e-7f64;
            let v16558 = 1e0f64;
            let v16581 = -6.8e1f64;
            let v16583 = -1e2f64;
            let v16584 = -1.1e2f64;
            let v16586 = 1.804851387e-35f64;
            let v16587 = -9e1f64;
            let v16593 = 7.8125e-2f64;
            let v16596 = 9.375e-1f64;
            let v16675 = 1e0f64;
            let v16699 = 8e-1f64;
            let v16702 = 1.2e0f64;
            let v16721 = 2.5000000000000005e-3f64;
            let v16743 = 5.625e-7f64;
            let v16769 = 4e-6f64;
            let v16975 = -1e0f64;
            let v16989 = 2.5e-5f64;
            let v16994 = 2.5e-3f64;
            let v17029 = parameters[26];
            let v17206 = -3.7e1f64;
            let v17239 = -3.7e1f64;
            let v17268 = 1.0f64;
            let v17270 = 0.0f64;
            let v17330 = 4e-12f64;
            let v17359 = 4e-12f64;
            let v17399 = 3.8025850929940455e0f64;
            let v17411 = 3.8025850929940455e0f64;
            let v17423 = 3.8025850929940455e0f64;
            let v17553 = 3.8025850929940455e0f64;
            let v17565 = 3.8025850929940455e0f64;
            let v17577 = 3.8025850929940455e0f64;
            let v17700 = parameters[28];
            let v17724 = 4.112737976006692e-57f64;
            let v17741 = parameters[1319];
            let v17743 = parameters[1320];
            let v17751 = 3.204352924e-19f64;
            let v17753 = parameters[1322];
            let v17795 = 1e0f64;
            let v17820 = -6.8e1f64;
            let v17822 = -1e2f64;
            let v17823 = -1.1e2f64;
            let v17825 = 1.804851387e-35f64;
            let v17826 = -9e1f64;
            let v17832 = 7.8125e-2f64;
            let v17835 = 9.375e-1f64;
            let v17907 = 1e0f64;
            let v17938 = -5e-1f64;
            let v17942 = -5e-1f64;
            let v18018 = parameters[1321];
            let v18170 = 2.5000000000000005e-3f64;
            let v18227 = 3.204352924e-19f64;
            let v18231 = 3.204352924e-19f64;
            let v18235 = 3.204352924e-19f64;
            let v18238 = parameters[31];
            let v18247 = 3.204352924e-19f64;
            let v18254 = 3.204352924e-19f64;
            let v18302 = 1e0f64;
            let v18327 = -6.8e1f64;
            let v18329 = -1e2f64;
            let v18330 = -1.1e2f64;
            let v18332 = 1.804851387e-35f64;
            let v18333 = -9e1f64;
            let v18339 = 7.8125e-2f64;
            let v18342 = 9.375e-1f64;
            let v18414 = 1e0f64;
            let v18426 = 1e0f64;
            let v18441 = 2.5000000000000005e-3f64;
            let v18458 = 5.625e-7f64;
            let v18488 = 2.5e-7f64;
            let v18493 = parameters[1353];
            let v18495 = parameters[1354];
            let v18498 = parameters[1348];
            let v18508 = parameters[1355];
            let v18516 = 6.25e-8f64;
            let v18531 = 1e0f64;
            let v18554 = -6.8e1f64;
            let v18556 = -1e2f64;
            let v18557 = -1.1e2f64;
            let v18559 = 1.804851387e-35f64;
            let v18560 = -9e1f64;
            let v18566 = 7.8125e-2f64;
            let v18569 = 9.375e-1f64;
            let v18648 = 1e0f64;
            let v18673 = 2.5000000000000005e-3f64;
            let v18683 = 2.5000000000000005e-3f64;
            let v18695 = parameters[139];
            let v18733 = 5.625e-7f64;
            let v18781 = 6.25e-2f64;
            let v18787 = 6.25e-2f64;
            let v18861 = 3.204352924e-19f64;
            let v18903 = 1e0f64;
            let v18928 = -6.8e1f64;
            let v18930 = -1e2f64;
            let v18931 = -1.1e2f64;
            let v18933 = 1.804851387e-35f64;
            let v18934 = -9e1f64;
            let v18940 = 7.8125e-2f64;
            let v18943 = 9.375e-1f64;
            let v19015 = 1e0f64;
            let v19027 = 1e0f64;
            let v19042 = 2.5000000000000005e-3f64;
            let v19061 = 5.625e-7f64;
            let v19090 = 2.5e-7f64;
            let v19103 = 1e0f64;
            let v19126 = -6.8e1f64;
            let v19128 = -1e2f64;
            let v19129 = -1.1e2f64;
            let v19131 = 1.804851387e-35f64;
            let v19132 = -9e1f64;
            let v19138 = 7.8125e-2f64;
            let v19141 = 9.375e-1f64;
            let v19220 = 1e0f64;
            let v19226 = 1e0f64;
            let v19241 = 0e0f64;
            let v19248 = 6.25e-2f64;
            let v19254 = 6.25e-2f64;
            let v19276 = 0e0f64;
            let v19359 = 4e-6f64;
            let v19372 = 2.5000000000000005e-3f64;
            let v19390 = 6.250000000000001e-4f64;
            let v19444 = 3.204352924e-19f64;
            let v19488 = 1e0f64;
            let v19513 = -6.8e1f64;
            let v19515 = -1e2f64;
            let v19516 = -1.1e2f64;
            let v19518 = 1.804851387e-35f64;
            let v19519 = -9e1f64;
            let v19525 = 7.8125e-2f64;
            let v19528 = 9.375e-1f64;
            let v19607 = 2.5e-7f64;
            let v19620 = 1e0f64;
            let v19643 = -6.8e1f64;
            let v19645 = -1e2f64;
            let v19646 = -1.1e2f64;
            let v19648 = 1.804851387e-35f64;
            let v19649 = -9e1f64;
            let v19655 = 7.8125e-2f64;
            let v19658 = 9.375e-1f64;
            let v19730 = 1e0f64;
            let v19743 = 1e0f64;
            let v19765 = parameters[1316];
            let v19781 = 4.112737976006692e-57f64;
            let v19835 = parameters[1317];
            let v19836 = parameters[1318];
            let v19898 = 0.0f64;
            let v19900 = 0.0f64;
            let v19902 = parameters[1359];
            let v19903 = parameters[1358];
            let v19906 = 0.0f64;
            let v19908 = 0.0f64;
            let v19911 = parameters[1357];
            let v19912 = parameters[1356];
            let v19914 = parameters[1360];
            let v19945 = parameters[1374];
            let v19949 = 1.0f64;
            let v19951 = 0.0f64;
            let v5 = if v4 == v1 { 1.0 } else { 0.0 };
            let v4661: f64;
            if v5 != 0.0 {
                v4661 = v1;
            } else {
                v4661 = v6;
            }
            let v9 = v7 * v8;
            let v11 = v10 * v8;
            let v13 = v11 / v12;
            let v14 = v7 / v10;
            let v16 = if v15 == 0.0 { 1.0 } else { 0.0 };
            let v9630: f64;
            if v16 != 0.0 {
                let v21 = ((v12 * v10) / v18) - v20;
                v9630 = v21;
            } else {
                v9630 = v22;
            }
            let v25 = v23 * v24;
            let v28 = v26 * v27;
            let v30 = v25 + v29;
            let v31 = if v30 <= v0 { 1.0 } else { 0.0 };
            if v31 != 0.0 {
            } else {
            }
            let v35 = (v28 / v32) + v34;
            let v36 = if v35 <= v0 { 1.0 } else { 0.0 };
            if v36 != 0.0 {
            } else {
            }
            let v38 = -v37;
            let v39 = v30.powf(v38);
            let v41 = -v40;
            let v42 = v35.powf(v41);
            let v43 = v39 * v42;
            let v55 = -v54;
            let v56 = v30.powf(v55);
            let v58 = -v57;
            let v59 = v35.powf(v58);
            let v60 = v56 * v59;
            let v70 = ((v61 + (v62 * v56)) + (v65 * v59)) + (v68 * v60);
            let v73 = v30 - (v71 * (((v44 + (v45 * v39)) + (v48 * v42)) + (v51 * v43)));
            let v74 = if v73 <= v0 { 1.0 } else { 0.0 };
            if v74 != 0.0 {
            } else {
                let v76 = if v73 <= v75 { 1.0 } else { 0.0 };
                if v76 != 0.0 {
                } else {
                }
            }
            let v80 = v35 - (v77 * v78);
            let v81 = v71 - v77;
            let v83 = v80 - (v81 * v70);
            let v84 = if v83 <= v0 { 1.0 } else { 0.0 };
            if v84 != 0.0 {
            } else {
                let v85 = if v83 <= v75 { 1.0 } else { 0.0 };
                if v85 != 0.0 {
                } else {
                }
            }
            let v105 = ((v96 + (v97 * v56)) + (v100 * v59)) + (v103 * v60);
            let v107 = v30 - (v71 * (((v86 + (v87 * v39)) + (v90 * v42)) + (v93 * v43)));
            let v108 = if v107 <= v0 { 1.0 } else { 0.0 };
            if v108 != 0.0 {
            } else {
                let v109 = if v107 <= v75 { 1.0 } else { 0.0 };
                if v109 != 0.0 {
                } else {
                }
            }
            let v111 = v80 - (v81 * v105);
            let v112 = if v111 <= v0 { 1.0 } else { 0.0 };
            if v112 != 0.0 {
            } else {
                let v113 = if v111 <= v75 { 1.0 } else { 0.0 };
                if v113 != 0.0 {
                } else {
                }
            }
            let v115 = v30.powf(v54);
            let v118 = v35.powf(v57);
            let v125 = v35 - (v71 * (((v114 + (v97 / v115)) + (v100 / v118)) + ((v103 / v115) / v118)));
            let v126 = if v125 <= v0 { 1.0 } else { 0.0 };
            if v126 != 0.0 {
            } else {
            }
            let v128 = v127 / v73;
            let v129 = v127 / v83;
            let v130 = v127 / v107;
            let v131 = v127 / v111;
            let v133 = v127 / v132;
            let v135 = v127 / v134;
            let v136 = v128 * v129;
            let v138 = if v137 != v0 { 1.0 } else { 0.0 };
            let v151: f64;
            let v162: f64;
            if v138 != 0.0 {
                let v140 = if v137 <= (-v30) { 1.0 } else { 0.0 };
                let v152: f64;
                let v163: f64;
                if v140 != 0.0 {
                    v152 = v39;
                    v163 = v56;
                } else {
                    let v141 = v30 + v137;
                    let v142 = v141.powf(v38);
                    let v143 = v141.powf(v55);
                    v152 = v142;
                    v163 = v143;
                }
                v151 = v152;
                v162 = v163;
            } else {
                v151 = v39;
                v162 = v56;
            }
            let v145 = if v144 != v0 { 1.0 } else { 0.0 };
            let v153: f64;
            let v164: f64;
            if v145 != 0.0 {
                let v147 = if v144 <= (-v35) { 1.0 } else { 0.0 };
                let v154: f64;
                let v165: f64;
                if v147 != 0.0 {
                    v154 = v42;
                    v165 = v59;
                } else {
                    let v148 = v35 + v144;
                    let v149 = v148.powf(v41);
                    let v150 = v148.powf(v58);
                    v154 = v149;
                    v165 = v150;
                }
                v153 = v154;
                v164 = v165;
            } else {
                v153 = v42;
                v164 = v59;
            }
            let v172 = ((v61 + (v62 * v162)) + (v65 * v164)) + (v68 * (v162 * v164));
            let v175 = (v30 - (v71 * (((v44 + (v45 * v151)) + (v48 * v153)) + (v51 * (v151 * v153))))) + v137;
            let v176 = if v175 <= v0 { 1.0 } else { 0.0 };
            if v176 != 0.0 {
            } else {
            }
            let v179 = (v35 - (v71 * v172)) + v144;
            let v180 = if v179 <= v0 { 1.0 } else { 0.0 };
            if v180 != 0.0 {
            } else {
            }
            let v182 = if v181 == v1 { 1.0 } else { 0.0 };
            let v187: f64;
            let v188: f64;
            if v182 != 0.0 {
                let v183 = v127 / v175;
                let v184 = v127 / v179;
                v187 = v183;
                v188 = v184;
            } else {
                let v185 = v1 / v175;
                let v186 = v1 / v179;
                v187 = v185;
                v188 = v186;
            }
            let v189 = v187 * v188;
            let v199 = ((v190 + (v187 * v191)) + (v188 * v194)) + (v189 * v197);
            let v209 = ((v200 + (v187 * v201)) + (v188 * v204)) + (v189 * v207);
            let v219 = ((v210 + (v187 * v211)) + (v188 * v214)) + (v189 * v217);
            let v229 = ((v220 + (v187 * v221)) + (v188 * v224)) + (v189 * v227);
            let v239 = ((v230 + (v187 * v231)) + (v188 * v234)) + (v189 * v237);
            let v249 = ((v240 + (v187 * v241)) + (v188 * v244)) + (v189 * v247);
            let v259 = ((v250 + (v187 * v251)) + (v188 * v254)) + (v189 * v257);
            let v269 = ((v260 + (v187 * v261)) + (v188 * v264)) + (v189 * v267);
            let v279 = ((v270 + (v187 * v271)) + (v188 * v274)) + (v189 * v277);
            let v289 = ((v280 + (v281 * v187)) + (v284 * v188)) + (v287 * v189);
            let v299 = ((v290 + (v291 * v187)) + (v294 * v188)) + (v297 * v189);
            let v309 = ((v300 + (v301 * v187)) + (v304 * v188)) + (v307 * v189);
            let v319 = ((v310 + (v311 * v187)) + (v314 * v188)) + (v317 * v189);
            let v329 = ((v320 + (v321 * v187)) + (v324 * v188)) + (v327 * v189);
            let v339 = ((v330 + (v331 * v187)) + (v334 * v188)) + (v337 * v189);
            let v349 = ((v340 + (v341 * v187)) + (v344 * v188)) + (v347 * v189);
            let v359 = ((v350 + (v351 * v187)) + (v354 * v188)) + (v357 * v189);
            let v369 = ((v360 + (v187 * v361)) + (v188 * v364)) + (v189 * v367);
            let v379 = ((v370 + (v187 * v371)) + (v188 * v374)) + (v189 * v377);
            let v389 = ((v380 + (v187 * v381)) + (v188 * v384)) + (v189 * v387);
            let v399 = ((v390 + (v187 * v391)) + (v188 * v394)) + (v189 * v397);
            let v409 = ((v400 + (v187 * v401)) + (v188 * v404)) + (v189 * v407);
            let v419 = ((v410 + (v187 * v411)) + (v188 * v414)) + (v189 * v417);
            let v429 = ((v420 + (v187 * v421)) + (v188 * v424)) + (v189 * v427);
            let v439 = ((v430 + (v187 * v431)) + (v188 * v434)) + (v189 * v437);
            let v449 = ((v440 + (v187 * v441)) + (v188 * v444)) + (v189 * v447);
            let v459 = ((v450 + (v187 * v451)) + (v188 * v454)) + (v189 * v457);
            let v469 = ((v460 + (v187 * v461)) + (v188 * v464)) + (v189 * v467);
            let v479 = ((v470 + (v187 * v471)) + (v188 * v474)) + (v189 * v477);
            let v489 = ((v480 + (v187 * v481)) + (v188 * v484)) + (v189 * v487);
            let v499 = ((v490 + (v187 * v491)) + (v188 * v494)) + (v189 * v497);
            let v509 = ((v500 + (v187 * v501)) + (v188 * v504)) + (v189 * v507);
            let v519 = ((v510 + (v187 * v511)) + (v188 * v514)) + (v189 * v517);
            let v529 = ((v520 + (v187 * v521)) + (v188 * v524)) + (v189 * v527);
            let v539 = ((v530 + (v187 * v531)) + (v188 * v534)) + (v189 * v537);
            let v549 = ((v540 + (v187 * v541)) + (v188 * v544)) + (v189 * v547);
            let v559 = ((v550 + (v187 * v551)) + (v188 * v554)) + (v189 * v557);
            let v569 = ((v560 + (v187 * v561)) + (v188 * v564)) + (v189 * v567);
            let v579 = ((v570 + (v187 * v571)) + (v188 * v574)) + (v189 * v577);
            let v589 = ((v580 + (v187 * v581)) + (v188 * v584)) + (v189 * v587);
            let v599 = ((v590 + (v187 * v591)) + (v188 * v594)) + (v189 * v597);
            let v609 = ((v600 + (v187 * v601)) + (v188 * v604)) + (v189 * v607);
            let v619 = ((v610 + (v187 * v611)) + (v188 * v614)) + (v189 * v617);
            let v629 = ((v620 + (v187 * v621)) + (v188 * v624)) + (v189 * v627);
            let v639 = ((v630 + (v187 * v631)) + (v188 * v634)) + (v189 * v637);
            let v649 = ((v640 + (v187 * v641)) + (v188 * v644)) + (v189 * v647);
            let v659 = ((v650 + (v187 * v651)) + (v188 * v654)) + (v189 * v657);
            let v669 = ((v660 + (v187 * v661)) + (v188 * v664)) + (v189 * v667);
            let v679 = ((v670 + (v187 * v671)) + (v188 * v674)) + (v189 * v677);
            let v689 = ((v680 + (v187 * v681)) + (v188 * v684)) + (v189 * v687);
            let v699 = ((v690 + (v187 * v691)) + (v188 * v694)) + (v189 * v697);
            let v709 = ((v700 + (v187 * v701)) + (v188 * v704)) + (v189 * v707);
            let v719 = ((v710 + (v187 * v711)) + (v188 * v714)) + (v189 * v717);
            let v729 = ((v720 + (v187 * v721)) + (v188 * v724)) + (v189 * v727);
            let v739 = ((v730 + (v187 * v731)) + (v188 * v734)) + (v189 * v737);
            let v749 = ((v740 + (v187 * v741)) + (v188 * v744)) + (v189 * v747);
            let v759 = ((v750 + (v187 * v751)) + (v188 * v754)) + (v189 * v757);
            let v769 = ((v760 + (v187 * v761)) + (v188 * v764)) + (v189 * v767);
            let v779 = ((v770 + (v187 * v771)) + (v188 * v774)) + (v189 * v777);
            let v789 = ((v780 + (v187 * v781)) + (v188 * v784)) + (v189 * v787);
            let v799 = ((v790 + (v187 * v791)) + (v188 * v794)) + (v189 * v797);
            let v809 = ((v800 + (v187 * v801)) + (v188 * v804)) + (v189 * v807);
            let v819 = ((v810 + (v187 * v811)) + (v188 * v814)) + (v189 * v817);
            let v829 = ((v820 + (v187 * v821)) + (v188 * v824)) + (v189 * v827);
            let v839 = ((v830 + (v187 * v831)) + (v188 * v834)) + (v189 * v837);
            let v849 = ((v840 + (v187 * v841)) + (v188 * v844)) + (v189 * v847);
            let v859 = ((v850 + (v187 * v851)) + (v188 * v854)) + (v189 * v857);
            let v869 = ((v860 + (v187 * v861)) + (v188 * v864)) + (v189 * v867);
            let v879 = ((v870 + (v187 * v871)) + (v188 * v874)) + (v189 * v877);
            let v889 = ((v880 + (v187 * v881)) + (v188 * v884)) + (v189 * v887);
            let v899 = ((v890 + (v187 * v891)) + (v188 * v894)) + (v189 * v897);
            let v909 = ((v900 + (v187 * v901)) + (v188 * v904)) + (v189 * v907);
            let v919 = ((v910 + (v187 * v911)) + (v188 * v914)) + (v189 * v917);
            let v929 = ((v920 + (v187 * v921)) + (v188 * v924)) + (v189 * v927);
            let v939 = ((v930 + (v931 * v187)) + (v934 * v188)) + (v937 * v189);
            let v949 = ((v940 + (v187 * v941)) + (v188 * v944)) + (v189 * v947);
            let v959 = ((v950 + (v187 * v951)) + (v188 * v954)) + (v189 * v957);
            let v969 = ((v960 + (v187 * v961)) + (v188 * v964)) + (v189 * v967);
            let v979 = ((v970 + (v187 * v971)) + (v188 * v974)) + (v189 * v977);
            let v989 = ((v980 + (v187 * v981)) + (v188 * v984)) + (v189 * v987);
            let v999 = ((v990 + (v187 * v991)) + (v188 * v994)) + (v189 * v997);
            let v1009 = ((v1000 + (v187 * v1001)) + (v188 * v1004)) + (v189 * v1007);
            let v1019 = ((v1010 + (v187 * v1011)) + (v188 * v1014)) + (v189 * v1017);
            let v1029 = ((v1020 + (v187 * v1021)) + (v188 * v1024)) + (v189 * v1027);
            let v1039 = ((v1030 + (v187 * v1031)) + (v188 * v1034)) + (v189 * v1037);
            let v1049 = ((v1040 + (v187 * v1041)) + (v188 * v1044)) + (v189 * v1047);
            let v1059 = ((v1050 + (v187 * v1051)) + (v188 * v1054)) + (v189 * v1057);
            let v1069 = ((v1060 + (v1061 * v187)) + (v1064 * v188)) + (v1067 * v189);
            let v1079 = ((v1070 + (v1071 * v187)) + (v1074 * v188)) + (v1077 * v189);
            let v1089 = ((v1080 + (v1081 * v187)) + (v1084 * v188)) + (v1087 * v189);
            let v1099 = ((v1090 + (v1091 * v187)) + (v1094 * v188)) + (v1097 * v189);
            let v1109 = ((v1100 + (v1101 * v187)) + (v1104 * v188)) + (v1107 * v189);
            let v1119 = ((v1110 + (v1111 * v187)) + (v1114 * v188)) + (v1117 * v189);
            let v1129 = ((v1120 + (v1121 * v187)) + (v1124 * v188)) + (v1127 * v189);
            let v1139 = ((v1130 + (v1131 * v187)) + (v1134 * v188)) + (v1137 * v189);
            let v1149 = ((v1140 + (v1141 * v187)) + (v1144 * v188)) + (v1147 * v189);
            let v1159 = ((v1150 + (v1151 * v187)) + (v1154 * v188)) + (v1157 * v189);
            let v1169 = ((v1160 + (v1161 * v187)) + (v1164 * v188)) + (v1167 * v189);
            let v1179 = ((v1170 + (v1171 * v187)) + (v1174 * v188)) + (v1177 * v189);
            let v1189 = ((v1180 + (v1181 * v187)) + (v1184 * v188)) + (v1187 * v189);
            let v1199 = ((v1190 + (v1191 * v187)) + (v1194 * v188)) + (v1197 * v189);
            let v1209 = ((v1200 + (v187 * v1201)) + (v188 * v1204)) + (v1207 * v189);
            let v1219 = ((v1210 + (v187 * v1211)) + (v188 * v1214)) + (v1217 * v189);
            let v1229 = ((v1220 + (v1221 * v187)) + (v1224 * v188)) + (v1227 * v189);
            let v1239 = ((v1230 + (v1231 * v187)) + (v1234 * v188)) + (v1237 * v189);
            let v1249 = ((v1240 + (v1241 * v187)) + (v1244 * v188)) + (v1247 * v189);
            let v1259 = ((v1250 + (v1251 * v187)) + (v1254 * v188)) + (v1257 * v189);
            let v1269 = ((v1260 + (v1261 * v187)) + (v1264 * v188)) + (v1267 * v189);
            let v1279 = ((v1270 + (v1271 * v187)) + (v1274 * v188)) + (v1277 * v189);
            let v1289 = ((v1280 + (v187 * v1281)) + (v188 * v1284)) + (v189 * v1287);
            let v1299 = ((v1290 + (v187 * v1291)) + (v188 * v1294)) + (v189 * v1297);
            let v1309 = ((v1300 + (v187 * v1301)) + (v188 * v1304)) + (v189 * v1307);
            let v1319 = ((v1310 + (v187 * v1311)) + (v188 * v1314)) + (v189 * v1317);
            let v1329 = ((v1320 + (v187 * v1321)) + (v188 * v1324)) + (v189 * v1327);
            let v1339 = ((v1330 + (v1331 * v187)) + (v1334 * v188)) + (v1337 * v189);
            let v1349 = ((v1340 + (v187 * v1341)) + (v188 * v1344)) + (v189 * v1347);
            let v1359 = ((v1350 + (v187 * v1351)) + (v188 * v1354)) + (v189 * v1357);
            let v1369 = ((v1360 + (v187 * v1361)) + (v188 * v1364)) + (v189 * v1367);
            let v1379 = ((v1370 + (v1371 * v187)) + (v1374 * v188)) + (v1377 * v189);
            let v1389 = ((v1380 + (v187 * v1381)) + (v188 * v1384)) + (v189 * v1387);
            let v1399 = ((v1390 + (v187 * v1391)) + (v188 * v1394)) + (v189 * v1397);
            let v1409 = ((v1400 + (v187 * v1401)) + (v188 * v1404)) + (v189 * v1407);
            let v1419 = ((v1410 + (v187 * v1411)) + (v188 * v1414)) + (v189 * v1417);
            let v1429 = ((v1420 + (v187 * v1421)) + (v188 * v1424)) + (v189 * v1427);
            let v1439 = ((v1430 + (v187 * v1431)) + (v188 * v1434)) + (v189 * v1437);
            let v1449 = ((v1440 + (v187 * v1441)) + (v188 * v1444)) + (v189 * v1447);
            let v1459 = ((v1450 + (v187 * v1451)) + (v188 * v1454)) + (v189 * v1457);
            let v1469 = ((v1460 + (v187 * v1461)) + (v188 * v1464)) + (v189 * v1467);
            let v1479 = ((v1470 + (v187 * v1471)) + (v188 * v1474)) + (v189 * v1477);
            let v1489 = ((v1480 + (v187 * v1481)) + (v188 * v1484)) + (v189 * v1487);
            let v1499 = ((v1490 + (v187 * v1491)) + (v188 * v1494)) + (v189 * v1497);
            let v1509 = ((v1500 + (v187 * v1501)) + (v188 * v1504)) + (v189 * v1507);
            let v1519 = ((v1510 + (v187 * v1511)) + (v188 * v1514)) + (v189 * v1517);
            let v1529 = ((v1520 + (v187 * v1521)) + (v188 * v1524)) + (v189 * v1527);
            let v1539 = ((v1530 + (v187 * v1531)) + (v188 * v1534)) + (v189 * v1537);
            let v1549 = ((v1540 + (v1541 * v187)) + (v1544 * v188)) + (v1547 * v189);
            let v1559 = ((v1550 + (v187 * v1551)) + (v188 * v1554)) + (v189 * v1557);
            let v1569 = ((v1560 + (v187 * v1561)) + (v188 * v1564)) + (v189 * v1567);
            let v1579 = ((v1570 + (v1571 * v187)) + (v1574 * v188)) + (v1577 * v189);
            let v1589 = ((v1580 + (v187 * v1581)) + (v188 * v1584)) + (v189 * v1587);
            let v1599 = ((v1590 + (v187 * v1591)) + (v188 * v1594)) + (v189 * v1597);
            let v1609 = ((v1600 + (v1601 * v187)) + (v1604 * v188)) + (v1607 * v189);
            let v1619 = ((v1610 + (v187 * v1611)) + (v188 * v1614)) + (v189 * v1617);
            let v1629 = ((v1620 + (v187 * v1621)) + (v188 * v1624)) + (v189 * v1627);
            let v1639 = ((v1630 + (v187 * v1631)) + (v188 * v1634)) + (v189 * v1637);
            let v1649 = ((v1640 + (v1641 * v187)) + (v1644 * v188)) + (v1647 * v189);
            let v1659 = ((v1650 + (v187 * v1651)) + (v188 * v1654)) + (v189 * v1657);
            let v1669 = ((v1660 + (v187 * v1661)) + (v188 * v1664)) + (v189 * v1667);
            let v1679 = ((v1670 + (v187 * v1671)) + (v188 * v1674)) + (v189 * v1677);
            let v1689 = ((v1680 + (v1681 * v187)) + (v1684 * v188)) + (v1687 * v189);
            let v1699 = ((v1690 + (v187 * v1691)) + (v188 * v1694)) + (v189 * v1697);
            let v1709 = ((v1700 + (v187 * v1701)) + (v188 * v1704)) + (v189 * v1707);
            let v1719 = ((v1710 + (v187 * v1711)) + (v188 * v1714)) + (v189 * v1717);
            let v1729 = ((v1720 + (v187 * v1721)) + (v188 * v1724)) + (v189 * v1727);
            let v1739 = ((v1730 + (v187 * v1731)) + (v188 * v1734)) + (v189 * v1737);
            let v1749 = ((v1740 + (v187 * v1741)) + (v188 * v1744)) + (v189 * v1747);
            let v1759 = ((v1750 + (v187 * v1751)) + (v188 * v1754)) + (v189 * v1757);
            let v1769 = ((v1760 + (v187 * v1761)) + (v188 * v1764)) + (v189 * v1767);
            let v1779 = ((v1770 + (v187 * v1771)) + (v188 * v1774)) + (v189 * v1777);
            let v1789 = ((v1780 + (v187 * v1781)) + (v188 * v1784)) + (v189 * v1787);
            let v1799 = ((v1790 + (v187 * v1791)) + (v188 * v1794)) + (v189 * v1797);
            let v1809 = ((v1800 + (v187 * v1801)) + (v188 * v1804)) + (v189 * v1807);
            let v1819 = ((v1810 + (v187 * v1811)) + (v188 * v1814)) + (v189 * v1817);
            let v1829 = ((v1820 + (v187 * v1821)) + (v188 * v1824)) + (v189 * v1827);
            let v1839 = ((v1830 + (v187 * v1831)) + (v188 * v1834)) + (v189 * v1837);
            let v1849 = ((v1840 + (v187 * v1841)) + (v188 * v1844)) + (v189 * v1847);
            let v1859 = ((v1850 + (v187 * v1851)) + (v188 * v1854)) + (v189 * v1857);
            let v1869 = ((v1860 + (v187 * v1861)) + (v188 * v1864)) + (v189 * v1867);
            let v1879 = ((v1870 + (v187 * v1871)) + (v188 * v1874)) + (v189 * v1877);
            let v1889 = ((v1880 + (v187 * v1881)) + (v188 * v1884)) + (v189 * v1887);
            let v1899 = ((v1890 + (v187 * v1891)) + (v188 * v1894)) + (v189 * v1897);
            let v1909 = ((v1900 + (v187 * v1901)) + (v188 * v1904)) + (v189 * v1907);
            let v1919 = ((v1910 + (v187 * v1911)) + (v188 * v1914)) + (v189 * v1917);
            let v1929 = ((v1920 + (v187 * v1921)) + (v188 * v1924)) + (v189 * v1927);
            let v1939 = ((v1930 + (v187 * v1931)) + (v188 * v1934)) + (v189 * v1937);
            let v1949 = ((v1940 + (v187 * v1941)) + (v188 * v1944)) + (v189 * v1947);
            let v1959 = ((v1950 + (v187 * v1951)) + (v188 * v1954)) + (v189 * v1957);
            let v1969 = ((v1960 + (v187 * v1961)) + (v188 * v1964)) + (v189 * v1967);
            let v1979 = ((v1970 + (v187 * v1971)) + (v188 * v1974)) + (v189 * v1977);
            let v1989 = ((v1980 + (v187 * v1981)) + (v188 * v1984)) + (v189 * v1987);
            let v1999 = ((v1990 + (v187 * v1991)) + (v188 * v1994)) + (v189 * v1997);
            let v2009 = ((v2000 + (v187 * v2001)) + (v188 * v2004)) + (v189 * v2007);
            let v2019 = ((v2010 + (v187 * v2011)) + (v188 * v2014)) + (v189 * v2017);
            let v2029 = ((v2020 + (v187 * v2021)) + (v188 * v2024)) + (v189 * v2027);
            let v2039 = ((v2030 + (v187 * v2031)) + (v188 * v2034)) + (v189 * v2037);
            let v2049 = ((v2040 + (v187 * v2041)) + (v188 * v2044)) + (v189 * v2047);
            let v2059 = ((v2050 + (v187 * v2051)) + (v188 * v2054)) + (v189 * v2057);
            let v2069 = ((v2060 + (v187 * v2061)) + (v188 * v2064)) + (v189 * v2067);
            let v2079 = ((v2070 + (v187 * v2071)) + (v188 * v2074)) + (v189 * v2077);
            let v2089 = ((v2080 + (v187 * v2081)) + (v188 * v2084)) + (v189 * v2087);
            let v2099 = ((v2090 + (v187 * v2091)) + (v188 * v2094)) + (v189 * v2097);
            let v2109 = ((v2100 + (v187 * v2101)) + (v188 * v2104)) + (v189 * v2107);
            let v2119 = ((v2110 + (v187 * v2111)) + (v188 * v2114)) + (v189 * v2117);
            let v2129 = ((v2120 + (v187 * v2121)) + (v188 * v2124)) + (v189 * v2127);
            let v2131 = if v2130 != v0 { 1.0 } else { 0.0 };
            let v2323: f64;
            let v2325: f64;
            let v2350: f64;
            let v2392: f64;
            let v2425: f64;
            let v2449: f64;
            let v2457: f64;
            let v2474: f64;
            let v2506: f64;
            let v2531: f64;
            let v2544: f64;
            let v2556: f64;
            let v2864: f64;
            if v2131 != 0.0 {
                let v2141 = ((v2132 + (v187 * v2133)) + (v188 * v2136)) + (v189 * v2139);
                let v2151 = ((v2142 + (v187 * v2143)) + (v188 * v2146)) + (v189 * v2149);
                let v2161 = ((v2152 + (v187 * v2153)) + (v188 * v2156)) + (v189 * v2159);
                let v2171 = ((v2162 + (v187 * v2163)) + (v188 * v2166)) + (v189 * v2169);
                let v2181 = ((v2172 + (v187 * v2173)) + (v188 * v2176)) + (v189 * v2179);
                let v2191 = ((v2182 + (v187 * v2183)) + (v188 * v2186)) + (v189 * v2189);
                let v2201 = ((v2192 + (v187 * v2193)) + (v188 * v2196)) + (v189 * v2199);
                let v2211 = ((v2202 + (v187 * v2203)) + (v188 * v2206)) + (v189 * v2209);
                let v2221 = ((v2212 + (v187 * v2213)) + (v188 * v2216)) + (v189 * v2219);
                let v2231 = ((v2222 + (v187 * v2223)) + (v188 * v2226)) + (v189 * v2229);
                let v2241 = ((v2232 + (v187 * v2233)) + (v188 * v2236)) + (v189 * v2239);
                let v2251 = ((v2242 + (v187 * v2243)) + (v188 * v2246)) + (v189 * v2249);
                let v2261 = ((v2252 + (v187 * v2253)) + (v188 * v2256)) + (v189 * v2259);
                v2323 = v2141;
                v2325 = v2151;
                v2350 = v2171;
                v2392 = v2181;
                v2425 = v2191;
                v2449 = v2211;
                v2457 = v2161;
                v2474 = v2231;
                v2506 = v2221;
                v2531 = v2241;
                v2544 = v2251;
                v2556 = v2261;
                v2864 = v2201;
            } else {
                v2323 = v0;
                v2325 = v0;
                v2350 = v0;
                v2392 = v0;
                v2425 = v0;
                v2449 = v0;
                v2457 = v0;
                v2474 = v0;
                v2506 = v0;
                v2531 = v0;
                v2544 = v0;
                v2556 = v0;
                v2864 = v0;
            }
            let v2291 = v239 * ((v1 + ((v2262 * (if ((v128.powf(v2263)) - (v133.powf(v2263))) >= v0 { ((v128.powf(v2263)) - (v133.powf(v2263))) } else { v0 })) + (v2269 * (if ((v128.powf(v2270)) - (v133.powf(v2270))) >= v0 { ((v128.powf(v2270)) - (v133.powf(v2270))) } else { v0 })))) + ((v2277 * (if ((v129.powf(v2278)) - (v135.powf(v2278))) >= v0 { ((v129.powf(v2278)) - (v135.powf(v2278))) } else { v0 })) + (v2284 * (v136.powf(v2285)))));
            let v2313 = v279 * ((v1 + (v2292 * (if ((v128.powf(v2293)) - (v133.powf(v2293))) >= v0 { ((v128.powf(v2293)) - (v133.powf(v2293))) } else { v0 }))) + ((v2299 * (if ((v129.powf(v2300)) - (v135.powf(v2300))) >= v0 { ((v129.powf(v2300)) - (v135.powf(v2300))) } else { v0 })) + (v2306 * (v136.powf(v2307)))));
            let v2321 = v1 + (v2314 * (if ((v128.powf(v2315)) - (v133.powf(v2315))) >= v0 { ((v128.powf(v2315)) - (v133.powf(v2315))) } else { v0 }));
            let v2322 = v369 * v2321;
            let v2889: f64;
            let v2891: f64;
            if v2131 != 0.0 {
                let v2324 = v2323 * v2321;
                let v2326 = v2325 * v2321;
                v2889 = v2326;
                v2891 = v2324;
            } else {
                v2889 = v2325;
                v2891 = v2323;
            }
            let v2335 = v409 * (v1 + (v2327 * (if ((v128.powf(v2328)) - (v133.powf(v2328))) >= v0 { ((v128.powf(v2328)) - (v133.powf(v2328))) } else { v0 })));
            let v2337 = v2336 * v649;
            let v2339 = if v2338 != v1 { 1.0 } else { 0.0 };
            let v2896: f64;
            let v4769: f64;
            if v2339 != 0.0 {
                let v2341 = if v2340 > v0 { 1.0 } else { 0.0 };
                let v2897: f64;
                let v4770: f64;
                if v2341 != 0.0 {
                    let v2348 = v1 - (v2342 * (if ((v128.powf(v2340)) - (v133.powf(v2340))) >= v0 { ((v128.powf(v2340)) - (v133.powf(v2340))) } else { v0 }));
                    let v2349 = v2337 * v2348;
                    let v4771: f64;
                    if v2131 != 0.0 {
                        let v2351 = v2350 * v2348;
                        v4771 = v2351;
                    } else {
                        v4771 = v2350;
                    }
                    v2897 = v2349;
                    v4770 = v4771;
                } else {
                    let v2352 = v1 - v2342;
                    let v2353 = v2337 * v2352;
                    let v4772: f64;
                    if v2131 != 0.0 {
                        let v2354 = v2350 * v2352;
                        v4772 = v2354;
                    } else {
                        v4772 = v2350;
                    }
                    v2897 = v2353;
                    v4770 = v4772;
                }
                v2896 = v2897;
                v4769 = v4770;
            } else {
                let v2356 = -v73;
                let v2367 = (v1 - (v2355 * (rspice_limited_exp((v2356 / v2357))))) - (v2362 * (rspice_limited_exp((v2356 / v2363))));
                let v2368 = v2337 * v2367;
                let v4773: f64;
                if v2131 != 0.0 {
                    let v2369 = v2350 * v2367;
                    v4773 = v2369;
                } else {
                    v4773 = v2350;
                }
                v2896 = v2368;
                v4769 = v4773;
            }
            let v2390 = (v1 + (v2370 * (if ((v128.powf(v2371)) - (v133.powf(v2371))) >= v0 { ((v128.powf(v2371)) - (v133.powf(v2371))) } else { v0 }))) + ((v2377 * (if ((v129.powf(v2378)) - (v135.powf(v2378))) >= v0 { ((v129.powf(v2378)) - (v135.powf(v2378))) } else { v0 })) + (v2384 * (v136.powf(v2385))));
            let v2391 = v659 * v2390;
            let v4775: f64;
            if v2131 != 0.0 {
                let v2393 = v2392 * v2390;
                v4775 = v2393;
            } else {
                v4775 = v2392;
            }
            let v2415 = v679 * ((v1 + (v2394 * (if ((v128.powf(v2395)) - (v133.powf(v2395))) >= v0 { ((v128.powf(v2395)) - (v133.powf(v2395))) } else { v0 }))) + ((v2401 * (if ((v129.powf(v2402)) - (v135.powf(v2402))) >= v0 { ((v129.powf(v2402)) - (v135.powf(v2402))) } else { v0 })) + (v2408 * (v136.powf(v2409)))));
            let v2423 = v1 + (v2416 * (if ((v128.powf(v2417)) - (v133.powf(v2417))) >= v0 { ((v128.powf(v2417)) - (v133.powf(v2417))) } else { v0 }));
            let v2424 = v669 * v2423;
            let v4789: f64;
            if v2131 != 0.0 {
                let v2426 = v2425 * v2423;
                v4789 = v2426;
            } else {
                v4789 = v2425;
            }
            let v2447 = (v1 + (v2427 * (if ((v128.powf(v2428)) - (v133.powf(v2428))) >= v0 { ((v128.powf(v2428)) - (v133.powf(v2428))) } else { v0 }))) + ((v2434 * (if ((v129.powf(v2435)) - (v135.powf(v2435))) >= v0 { ((v129.powf(v2435)) - (v135.powf(v2435))) } else { v0 })) + (v2441 * (v136.powf(v2442))));
            let v2448 = v699 * v2447;
            let v4782: f64;
            if v2131 != 0.0 {
                let v2450 = v2449 * v2447;
                v4782 = v2450;
            } else {
                v4782 = v2449;
            }
            let v2455 = if ((v128.powf(v2451)) - (v133.powf(v2451))) >= v0 { ((v128.powf(v2451)) - (v133.powf(v2451))) } else { v0 };
            let v2456 = v619 * v2455;
            let v4721: f64;
            if v2131 != 0.0 {
                let v2458 = v2457 * v2455;
                v4721 = v2458;
            } else {
                v4721 = v2457;
            }
            let v2464 = v629 * (if ((v128.powf(v2459)) - (v133.powf(v2459))) >= v0 { ((v128.powf(v2459)) - (v133.powf(v2459))) } else { v0 });
            let v2472 = v1 + (v2465 * (if ((v128.powf(v2466)) - (v133.powf(v2466))) >= v0 { ((v128.powf(v2466)) - (v133.powf(v2466))) } else { v0 }));
            let v2473 = v829 * v2472;
            let v5571: f64;
            if v2131 != 0.0 {
                let v2475 = v2474 * v2472;
                v5571 = v2475;
            } else {
                v5571 = v2474;
            }
            let v2486 = if (v639 * (v1 + (v2476 * (if ((v128.powf(v2477)) - (v133.powf(v2477))) >= v0 { ((v128.powf(v2477)) - (v133.powf(v2477))) } else { v0 })))) <= v2485 { (v639 * (v1 + (v2476 * (if ((v128.powf(v2477)) - (v133.powf(v2477))) >= v0 { ((v128.powf(v2477)) - (v133.powf(v2477))) } else { v0 })))) } else { v2485 };
            let v2495 = v889 * (v1 + (v2487 * (if ((v128.powf(v2488)) - (v133.powf(v2488))) >= v0 { ((v128.powf(v2488)) - (v133.powf(v2488))) } else { v0 })));
            let v2503 = v1 + (v2496 * (if ((v128.powf(v2497)) - (v133.powf(v2497))) >= v0 { ((v128.powf(v2497)) - (v133.powf(v2497))) } else { v0 }));
            let v2505 = if (v709 * v2503) >= v0 { (v709 * v2503) } else { v0 };
            let v5575: f64;
            if v2131 != 0.0 {
                let v2508 = if (v2506 * v2503) >= v0 { (v2506 * v2503) } else { v0 };
                v5575 = v2508;
            } else {
                v5575 = v2506;
            }
            let v2529 = (v1 + (v2509 * (if ((v128.powf(v2510)) - (v133.powf(v2510))) >= v0 { ((v128.powf(v2510)) - (v133.powf(v2510))) } else { v0 }))) + ((v2516 * (if ((v129.powf(v2517)) - (v135.powf(v2517))) >= v0 { ((v129.powf(v2517)) - (v135.powf(v2517))) } else { v0 })) + (v2523 * (v136.powf(v2524))));
            let v2530 = v909 * v2529;
            let v4801: f64;
            if v2131 != 0.0 {
                let v2532 = v2531 * v2529;
                v4801 = v2532;
            } else {
                v4801 = v2531;
            }
            let v2540 = v1 + (v2533 * (if ((v128.powf(v2534)) - (v133.powf(v2534))) >= v0 { ((v128.powf(v2534)) - (v133.powf(v2534))) } else { v0 }));
            let v2543 = if (v949 * v2540) >= v2542 { (v949 * v2540) } else { v2542 };
            let v5579: f64;
            if v2131 != 0.0 {
                let v2546 = if (v2544 * v2540) >= v2542 { (v2544 * v2540) } else { v2542 };
                v5579 = v2546;
            } else {
                v5579 = v2544;
            }
            let v2554 = v1 + (v2547 * (if ((v128.powf(v2548)) - (v133.powf(v2548))) >= v0 { ((v128.powf(v2548)) - (v133.powf(v2548))) } else { v0 }));
            let v2555 = v819 * v2554;
            let v4830: f64;
            if v2131 != 0.0 {
                let v2557 = v2556 * v2554;
                v4830 = v2557;
            } else {
                v4830 = v2556;
            }
            let v2566 = v989 * (v1 + (v2558 * (if ((v128.powf(v2559)) - (v133.powf(v2559))) >= v0 { ((v128.powf(v2559)) - (v133.powf(v2559))) } else { v0 })));
            let v2573 = v1319 * ((v1 + (v2567 * v128)) + (v2570 * v129));
            let v2580 = v1359 * ((v1 + (v2574 * v128)) + (v2577 * v129));
            let v2587 = v1599 * ((v1 + (v2581 * v128)) + (v2584 * v129));
            let v2594 = v1639 * ((v1 + (v2588 * v128)) + (v2591 * v129));
            let v2601 = v1679 * ((v1 + (v2595 * v128)) + (v2598 * v129));
            let v2606 = v2602 * (v1 + (v2603 * v128));
            let v2630 = v131 * v130;
            let v2637 = v249 * ((v1 + ((v2607 * (if ((v130.powf(v2608)) - (v133.powf(v2608))) >= v0 { ((v130.powf(v2608)) - (v133.powf(v2608))) } else { v0 })) + (v2614 * (if ((v130.powf(v2615)) - (v133.powf(v2615))) >= v0 { ((v130.powf(v2615)) - (v133.powf(v2615))) } else { v0 })))) + ((v2622 * (if ((v131.powf(v2623)) - (v135.powf(v2623))) >= v0 { ((v131.powf(v2623)) - (v135.powf(v2623))) } else { v0 })) + (v2629 * (v2630.powf(v2631)))));
            let v2639 = if v2638 == v1 { 1.0 } else { 0.0 };
            let v2878: f64;
            if v2639 != 0.0 {
                v2878 = v2291;
            } else {
                v2878 = v2637;
            }
            let v2661 = v199 * ((v1 + (v2640 * (if ((v128.powf(v2641)) - (v133.powf(v2641))) >= v0 { ((v128.powf(v2641)) - (v133.powf(v2641))) } else { v0 }))) + ((v2647 * (if ((v129.powf(v2648)) - (v135.powf(v2648))) >= v0 { ((v129.powf(v2648)) - (v135.powf(v2648))) } else { v0 })) + (v2654 * (v136.powf(v2655)))));
            let v2683 = v219 * ((v1 + (v2662 * (if ((v130.powf(v2663)) - (v133.powf(v2663))) >= v0 { ((v130.powf(v2663)) - (v133.powf(v2663))) } else { v0 }))) + ((v2669 * (if ((v131.powf(v2670)) - (v135.powf(v2670))) >= v0 { ((v131.powf(v2670)) - (v135.powf(v2670))) } else { v0 })) + (v2676 * (v2630.powf(v2677)))));
            let v2705 = v959 * ((v1 + (v2684 * (if ((v130.powf(v2685)) - (v133.powf(v2685))) >= v0 { ((v130.powf(v2685)) - (v133.powf(v2685))) } else { v0 }))) + ((v2691 * (if ((v131.powf(v2692)) - (v135.powf(v2692))) >= v0 { ((v131.powf(v2692)) - (v135.powf(v2692))) } else { v0 })) + (v2698 * (v2630.powf(v2699)))));
            let v2715 = if (v719 * (v1 + (v2706 * (if ((v130.powf(v2707)) - (v133.powf(v2707))) >= v0 { ((v130.powf(v2707)) - (v133.powf(v2707))) } else { v0 })))) >= v0 { (v719 * (v1 + (v2706 * (if ((v130.powf(v2707)) - (v133.powf(v2707))) >= v0 { ((v130.powf(v2707)) - (v133.powf(v2707))) } else { v0 })))) } else { v0 };
            let v2737 = v579 * ((v1 + (v2716 * (if ((v128.powf(v2717)) - (v133.powf(v2717))) >= v0 { ((v128.powf(v2717)) - (v133.powf(v2717))) } else { v0 }))) + ((v2723 * (if ((v129.powf(v2724)) - (v135.powf(v2724))) >= v0 { ((v129.powf(v2724)) - (v135.powf(v2724))) } else { v0 })) + (v2730 * (v136.powf(v2731)))));
            let v2759 = v589 * ((v1 + (v2738 * (if ((v128.powf(v2739)) - (v133.powf(v2739))) >= v0 { ((v128.powf(v2739)) - (v133.powf(v2739))) } else { v0 }))) + ((v2745 * (if ((v129.powf(v2746)) - (v135.powf(v2746))) >= v0 { ((v129.powf(v2746)) - (v135.powf(v2746))) } else { v0 })) + (v2752 * (v136.powf(v2753)))));
            let v2781 = v569 * ((v1 + (v2760 * (if ((v128.powf(v2761)) - (v133.powf(v2761))) >= v0 { ((v128.powf(v2761)) - (v133.powf(v2761))) } else { v0 }))) + ((v2767 * (if ((v129.powf(v2768)) - (v135.powf(v2768))) >= v0 { ((v129.powf(v2768)) - (v135.powf(v2768))) } else { v0 })) + (v2774 * (v136.powf(v2775)))));
            let v2803 = v1999 * ((v1 + (v2782 * (if ((v128.powf(v2783)) - (v133.powf(v2783))) >= v0 { ((v128.powf(v2783)) - (v133.powf(v2783))) } else { v0 }))) + ((v2789 * (if ((v129.powf(v2790)) - (v135.powf(v2790))) >= v0 { ((v129.powf(v2790)) - (v135.powf(v2790))) } else { v0 })) + (v2796 * (v136.powf(v2797)))));
            let v2812 = v759 * (v1 + (v2804 * (if ((v128.powf(v2805)) - (v133.powf(v2805))) >= v0 { ((v128.powf(v2805)) - (v133.powf(v2805))) } else { v0 })));
            let v2816 = v1419 * (v1 + (v128 * v2813));
            let v2820 = v1429 * (v1 + (v128 * v2817));
            let v2824 = v1449 * (v1 + (v128 * v2821));
            let v2828 = v1489 * (v1 + (v128 * v2825));
            let v2832 = v1499 * (v1 + (v128 * v2829));
            let v2834 = if v2833 == v1 { 1.0 } else { 0.0 };
            let v4517: f64;
            let v4519: f64;
            let v4522: f64;
            if v2834 != 0.0 {
                let v2843 = v729 * (v1 + (v2835 * (if ((v128.powf(v2836)) - (v133.powf(v2836))) >= v0 { ((v128.powf(v2836)) - (v133.powf(v2836))) } else { v0 })));
                let v2852 = v739 * (v1 + (v2844 * (if ((v128.powf(v2845)) - (v133.powf(v2845))) >= v0 { ((v128.powf(v2845)) - (v133.powf(v2845))) } else { v0 })));
                v4517 = v2843;
                v4519 = v2852;
                v4522 = v799;
            } else {
                let v2861 = v799 * (v1 + (v2853 * (if ((v128.powf(v2854)) - (v133.powf(v2854))) >= v0 { ((v128.powf(v2854)) - (v133.powf(v2854))) } else { v0 })));
                v4517 = v729;
                v4519 = v739;
                v4522 = v2861;
            }
            let v2862 = if v689 < v1 { 1.0 } else { 0.0 };
            let v2903: f64;
            if v2862 != 0.0 {
                v2903 = v1;
            } else {
                let v2863 = if v689 > v71 { 1.0 } else { 0.0 };
                let v2904: f64;
                if v2863 != 0.0 {
                    v2904 = v71;
                } else {
                    v2904 = v689;
                }
                v2903 = v2904;
            }
            let v4791: f64;
            if v2131 != 0.0 {
                let v2865 = if v2864 < v1 { 1.0 } else { 0.0 };
                let v4792: f64;
                if v2865 != 0.0 {
                    v4792 = v1;
                } else {
                    let v2866 = if v2864 > v71 { 1.0 } else { 0.0 };
                    let v4793: f64;
                    if v2866 != 0.0 {
                        v4793 = v71;
                    } else {
                        v4793 = v2864;
                    }
                    v4792 = v4793;
                }
                v4791 = v4792;
            } else {
                v4791 = v2864;
            }
            let v2867 = if v1349 < v0 { 1.0 } else { 0.0 };
            if v2867 != 0.0 {
            } else {
            }
            let v2868 = if v1389 < v0 { 1.0 } else { 0.0 };
            if v2868 != 0.0 {
            } else {
            }
            let v2869 = if v979 <= v0 { 1.0 } else { 0.0 };
            if v2869 != 0.0 {
            } else {
            }
            let v2870 = if v969 <= v0 { 1.0 } else { 0.0 };
            if v2870 != 0.0 {
            } else {
            }
            let v2871 = if v869 < v0 { 1.0 } else { 0.0 };
            if v2871 != 0.0 {
            } else {
            }
            let v2872 = if v269 < v0 { 1.0 } else { 0.0 };
            if v2872 != 0.0 {
            } else {
            }
            let v2873 = if v2313 < v0 { 1.0 } else { 0.0 };
            if v2873 != 0.0 {
            } else {
            }
            let v2874 = if v2737 < v0 { 1.0 } else { 0.0 };
            if v2874 != 0.0 {
            } else {
            }
            let v2875 = if v2759 < v0 { 1.0 } else { 0.0 };
            if v2875 != 0.0 {
            } else {
            }
            let v2876 = if v229 <= v0 { 1.0 } else { 0.0 };
            if v2876 != 0.0 {
            } else {
            }
            let v2877 = if v2291 <= v0 { 1.0 } else { 0.0 };
            if v2877 != 0.0 {
            } else {
            }
            let v2879 = if v2878 <= v0 { 1.0 } else { 0.0 };
            if v2879 != 0.0 {
            } else {
            }
            let v2881 = if v2880 <= v0 { 1.0 } else { 0.0 };
            if v2881 != 0.0 {
            } else {
            }
            let v2882 = if v1879 <= v0 { 1.0 } else { 0.0 };
            if v2882 != 0.0 {
            } else {
            }
            let v2883 = if v259 <= v0 { 1.0 } else { 0.0 };
            if v2883 != 0.0 {
            } else {
            }
            let v2884 = if v599 <= v0 { 1.0 } else { 0.0 };
            if v2884 != 0.0 {
            } else {
            }
            let v2886 = if v2885 != v0 { 1.0 } else { 0.0 };
            if v2886 != 0.0 {
                let v2887 = if v1719 <= v0 { 1.0 } else { 0.0 };
                if v2887 != 0.0 {
                } else {
                }
            } else {
            }
            let v2888 = if v2322 < v0 { 1.0 } else { 0.0 };
            if v2888 != 0.0 {
            } else {
            }
            if v2131 != 0.0 {
                let v2890 = if v2889 < v0 { 1.0 } else { 0.0 };
                if v2890 != 0.0 {
                } else {
                }
                let v2892 = if v2891 < v0 { 1.0 } else { 0.0 };
                if v2892 != 0.0 {
                } else {
                }
            } else {
            }
            let v2893 = if v1729 < v0 { 1.0 } else { 0.0 };
            let v12717: f64;
            if v2893 != 0.0 {
                v12717 = v0;
            } else {
                v12717 = v1729;
            }
            let v2894 = if v1739 < v0 { 1.0 } else { 0.0 };
            let v12746: f64;
            if v2894 != 0.0 {
                v12746 = v0;
            } else {
                v12746 = v1739;
            }
            let v2895 = if v1839 < v0 { 1.0 } else { 0.0 };
            let v4877: f64;
            if v2895 != 0.0 {
                v4877 = v0;
            } else {
                v4877 = v1839;
            }
            let v2898 = if v2896 <= v0 { 1.0 } else { 0.0 };
            let v4729: f64;
            if v2898 != 0.0 {
                v4729 = v2899;
            } else {
                v4729 = v2896;
            }
            let v2900 = if v2391 < v0 { 1.0 } else { 0.0 };
            let v4732: f64;
            if v2900 != 0.0 {
                v4732 = v0;
            } else {
                v4732 = v2391;
            }
            let v2901 = if v2415 < v0 { 1.0 } else { 0.0 };
            let v4759: f64;
            if v2901 != 0.0 {
                v4759 = v0;
            } else {
                v4759 = v2415;
            }
            let v2902 = if v2424 < v0 { 1.0 } else { 0.0 };
            let v4753: f64;
            if v2902 != 0.0 {
                v4753 = v0;
            } else {
                v4753 = v2424;
            }
            let v2905 = if v2903 < v0 { 1.0 } else { 0.0 };
            let v4756: f64;
            if v2905 != 0.0 {
                v4756 = v0;
            } else {
                v4756 = v2903;
            }
            let v2906 = if v1239 <= v0 { 1.0 } else { 0.0 };
            let v9094: f64;
            if v2906 != 0.0 {
                v9094 = v1;
            } else {
                v9094 = v1239;
            }
            let v2907 = if v1099 <= v0 { 1.0 } else { 0.0 };
            if v2907 != 0.0 {
            } else {
            }
            let v2909 = if v1089 <= v0 { 1.0 } else { 0.0 };
            if v2909 != 0.0 {
            } else {
            }
            let v2911 = if v2910 < v0 { 1.0 } else { 0.0 };
            if v2911 != 0.0 {
            } else {
            }
            let v2914 = v2912 - v2913;
            let v2917 = v2916 - v2913;
            let v3759: f64;
            let v3809: f64;
            let v4348: f64;
            let v4496: f64;
            let v4506: f64;
            let v5077: f64;
            let v5094: f64;
            if v2918 != 0.0 {
                let v2921 = v2919 * v2920;
                v3759 = v0;
                v3809 = v0;
                v4348 = v0;
                v4496 = v0;
                v4506 = v2921;
                v5077 = v0;
                v5094 = v0;
            } else {
                let v2925 = if (if v2922 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v2919 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v3760: f64;
                let v3810: f64;
                let v4349: f64;
                let v4497: f64;
                let v4507: f64;
                let v5078: f64;
                let v5095: f64;
                if v2925 != 0.0 {
                    let v2928 = if v2926 < v2927 { 1.0 } else { 0.0 };
                    let v2967: f64;
                    let v3020: f64;
                    let v3556: f64;
                    let v5079: f64;
                    let v5096: f64;
                    if v2928 != 0.0 {
                        let v2930 = if (v32 % v71) != v0 { 1.0 } else { 0.0 };
                        let v2946: f64;
                        let v2952: f64;
                        let v2968: f64;
                        let v3021: f64;
                        if v2930 != 0.0 {
                            let v2934 = v71 * (if ((v32 - v1) / v71) >= v0 { ((v32 - v1) / v71) } else { v0 });
                            v2946 = v2934;
                            v2952 = v2934;
                            v2968 = v1;
                            v3021 = v1;
                        } else {
                            let v2936 = if v2935 == v1 { 1.0 } else { 0.0 };
                            let v2947: f64;
                            let v2953: f64;
                            let v2969: f64;
                            let v3022: f64;
                            if v2936 != 0.0 {
                                let v2940 = v71 * (if ((v32 / v71) - v1) >= v0 { ((v32 / v71) - v1) } else { v0 });
                                v2947 = v32;
                                v2953 = v2940;
                                v2969 = v0;
                                v3022 = v71;
                            } else {
                                let v2944 = v71 * (if ((v32 / v71) - v1) >= v0 { ((v32 / v71) - v1) } else { v0 });
                                v2947 = v2944;
                                v2953 = v32;
                                v2969 = v71;
                                v3022 = v0;
                            }
                            v2946 = v2947;
                            v2952 = v2953;
                            v2968 = v2969;
                            v3021 = v3022;
                        }
                        let v3557: f64;
                        if v2945 != 0.0 {
                            let v2948 = if v2946 == v0 { 1.0 } else { 0.0 };
                            let v3558: f64;
                            if v2948 != 0.0 {
                                v3558 = v0;
                            } else {
                                let v2951 = (v2919 * v2914) / (v83 * v2946);
                                v3558 = v2951;
                            }
                            v3557 = v3558;
                        } else {
                            let v2954 = if v2952 == v0 { 1.0 } else { 0.0 };
                            let v3559: f64;
                            if v2954 != 0.0 {
                                v3559 = v0;
                            } else {
                                let v2957 = (v2919 * v2914) / (v83 * v2952);
                                v3559 = v2957;
                            }
                            v3557 = v3559;
                        }
                        v2967 = v2968;
                        v3020 = v3021;
                        v3556 = v3557;
                        v5079 = v2946;
                        v5096 = v2952;
                    } else {
                        v2967 = v0;
                        v3020 = v0;
                        v3556 = v0;
                        v5079 = v0;
                        v5096 = v0;
                    }
                    let v2958 = if v2926 == v0 { 1.0 } else { 0.0 };
                    let v3555: f64;
                    let v3575: f64;
                    if v2958 != 0.0 {
                        let v3576: f64;
                        if v2959 != 0.0 {
                            let v3577: f64;
                            if v2960 != 0.0 {
                                let v2966 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3578: f64;
                                if v2966 != 0.0 {
                                    let v2970 = if v2967 == v0 { 1.0 } else { 0.0 };
                                    let v3579: f64;
                                    if v2970 != 0.0 {
                                        v3579 = v0;
                                    } else {
                                        let v2973 = (v2919 * v2914) / (v83 * v2967);
                                        v3579 = v2973;
                                    }
                                    v3578 = v3579;
                                } else {
                                    let v2981 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3580: f64;
                                    if v2981 != 0.0 {
                                        let v2982 = v2914 + v2915;
                                        let v2983 = if v2982 == v0 { 1.0 } else { 0.0 };
                                        if v2983 != 0.0 {
                                        } else {
                                        }
                                        let v2985 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2983 != 0.0 { 1.0 } else { 0.0 };
                                        let v3581: f64;
                                        if v2985 != 0.0 {
                                            v3581 = v0;
                                        } else {
                                            let v2989 = (v2919 * v83) / ((v2974 * v2967) * v2982);
                                            v3581 = v2989;
                                        }
                                        v3580 = v3581;
                                    } else {
                                        v3580 = v0;
                                    }
                                    v3578 = v3580;
                                }
                                v3577 = v3578;
                            } else {
                                let v2995 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3582: f64;
                                if v2995 != 0.0 {
                                    let v2996 = if v2967 == v0 { 1.0 } else { 0.0 };
                                    let v3583: f64;
                                    if v2996 != 0.0 {
                                        v3583 = v0;
                                    } else {
                                        let v2999 = (v2919 * v2914) / (v83 * v2967);
                                        v3583 = v2999;
                                    }
                                    v3582 = v3583;
                                } else {
                                    let v3005 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3584: f64;
                                    if v3005 != 0.0 {
                                        let v3006 = v2914 + v2915;
                                        let v3007 = if v3006 == v0 { 1.0 } else { 0.0 };
                                        if v3007 != 0.0 {
                                        } else {
                                        }
                                        let v3009 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3007 != 0.0 { 1.0 } else { 0.0 };
                                        let v3585: f64;
                                        if v3009 != 0.0 {
                                            v3585 = v0;
                                        } else {
                                            let v3013 = (v2919 * v83) / ((v2974 * v2967) * v3006);
                                            v3585 = v3013;
                                        }
                                        v3584 = v3585;
                                    } else {
                                        v3584 = v0;
                                    }
                                    v3582 = v3584;
                                }
                                v3577 = v3582;
                            }
                            v3576 = v3577;
                        } else {
                            let v3586: f64;
                            if v3014 != 0.0 {
                                let v3019 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3587: f64;
                                if v3019 != 0.0 {
                                    let v3023 = if v3020 == v0 { 1.0 } else { 0.0 };
                                    let v3588: f64;
                                    if v3023 != 0.0 {
                                        v3588 = v0;
                                    } else {
                                        let v3026 = (v2919 * v2914) / (v83 * v3020);
                                        v3588 = v3026;
                                    }
                                    v3587 = v3588;
                                } else {
                                    let v3031 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3589: f64;
                                    if v3031 != 0.0 {
                                        let v3032 = v2914 + v2915;
                                        let v3033 = if v3032 == v0 { 1.0 } else { 0.0 };
                                        if v3033 != 0.0 {
                                        } else {
                                        }
                                        let v3035 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3033 != 0.0 { 1.0 } else { 0.0 };
                                        let v3590: f64;
                                        if v3035 != 0.0 {
                                            v3590 = v0;
                                        } else {
                                            let v3039 = (v2919 * v83) / ((v2974 * v3020) * v3032);
                                            v3590 = v3039;
                                        }
                                        v3589 = v3590;
                                    } else {
                                        v3589 = v0;
                                    }
                                    v3587 = v3589;
                                }
                                v3586 = v3587;
                            } else {
                                let v3044 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3591: f64;
                                if v3044 != 0.0 {
                                    let v3045 = if v3020 == v0 { 1.0 } else { 0.0 };
                                    let v3592: f64;
                                    if v3045 != 0.0 {
                                        v3592 = v0;
                                    } else {
                                        let v3048 = (v2919 * v2914) / (v83 * v3020);
                                        v3592 = v3048;
                                    }
                                    v3591 = v3592;
                                } else {
                                    let v3053 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3593: f64;
                                    if v3053 != 0.0 {
                                        let v3054 = v2914 + v2915;
                                        let v3055 = if v3054 == v0 { 1.0 } else { 0.0 };
                                        if v3055 != 0.0 {
                                        } else {
                                        }
                                        let v3057 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3055 != 0.0 { 1.0 } else { 0.0 };
                                        let v3594: f64;
                                        if v3057 != 0.0 {
                                            v3594 = v0;
                                        } else {
                                            let v3061 = (v2919 * v83) / ((v2974 * v3020) * v3054);
                                            v3594 = v3061;
                                        }
                                        v3593 = v3594;
                                    } else {
                                        v3593 = v0;
                                    }
                                    v3591 = v3593;
                                }
                                v3586 = v3591;
                            }
                            v3576 = v3586;
                        }
                        v3555 = v3556;
                        v3575 = v3576;
                    } else {
                        let v3062 = if v2926 == v1 { 1.0 } else { 0.0 };
                        let v3560: f64;
                        let v3595: f64;
                        if v3062 != 0.0 {
                            let v3596: f64;
                            if v3063 != 0.0 {
                                let v3597: f64;
                                if v3064 != 0.0 {
                                    let v3069 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3598: f64;
                                    if v3069 != 0.0 {
                                        let v3070 = if v2967 == v0 { 1.0 } else { 0.0 };
                                        let v3599: f64;
                                        if v3070 != 0.0 {
                                            v3599 = v0;
                                        } else {
                                            let v3073 = (v2919 * v2914) / (v83 * v2967);
                                            v3599 = v3073;
                                        }
                                        v3598 = v3599;
                                    } else {
                                        let v3078 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3600: f64;
                                        if v3078 != 0.0 {
                                            let v3079 = v2914 + v2915;
                                            let v3080 = if v3079 == v0 { 1.0 } else { 0.0 };
                                            if v3080 != 0.0 {
                                            } else {
                                            }
                                            let v3082 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3080 != 0.0 { 1.0 } else { 0.0 };
                                            let v3601: f64;
                                            if v3082 != 0.0 {
                                                v3601 = v0;
                                            } else {
                                                let v3086 = (v2919 * v83) / ((v2974 * v2967) * v3079);
                                                v3601 = v3086;
                                            }
                                            v3600 = v3601;
                                        } else {
                                            v3600 = v0;
                                        }
                                        v3598 = v3600;
                                    }
                                    v3597 = v3598;
                                } else {
                                    let v3091 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3602: f64;
                                    if v3091 != 0.0 {
                                        let v3092 = if v2967 == v0 { 1.0 } else { 0.0 };
                                        let v3603: f64;
                                        if v3092 != 0.0 {
                                            v3603 = v0;
                                        } else {
                                            let v3095 = (v2919 * v2914) / (v83 * v2967);
                                            v3603 = v3095;
                                        }
                                        v3602 = v3603;
                                    } else {
                                        let v3100 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3604: f64;
                                        if v3100 != 0.0 {
                                            let v3101 = v2914 + v2915;
                                            let v3102 = if v3101 == v0 { 1.0 } else { 0.0 };
                                            if v3102 != 0.0 {
                                            } else {
                                            }
                                            let v3104 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3102 != 0.0 { 1.0 } else { 0.0 };
                                            let v3605: f64;
                                            if v3104 != 0.0 {
                                                v3605 = v0;
                                            } else {
                                                let v3108 = (v2919 * v83) / ((v2974 * v2967) * v3101);
                                                v3605 = v3108;
                                            }
                                            v3604 = v3605;
                                        } else {
                                            v3604 = v0;
                                        }
                                        v3602 = v3604;
                                    }
                                    v3597 = v3602;
                                }
                                v3596 = v3597;
                            } else {
                                let v3606: f64;
                                if v3109 != 0.0 {
                                    let v3114 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3607: f64;
                                    if v3114 != 0.0 {
                                        let v3115 = if v3020 == v0 { 1.0 } else { 0.0 };
                                        let v3608: f64;
                                        if v3115 != 0.0 {
                                            v3608 = v0;
                                        } else {
                                            let v3118 = (v2919 * v2914) / (v83 * v3020);
                                            v3608 = v3118;
                                        }
                                        v3607 = v3608;
                                    } else {
                                        let v3123 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3609: f64;
                                        if v3123 != 0.0 {
                                            let v3124 = if v2914 == v0 { 1.0 } else { 0.0 };
                                            if v3124 != 0.0 {
                                            } else {
                                            }
                                            let v3126 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3124 != 0.0 { 1.0 } else { 0.0 };
                                            let v3610: f64;
                                            if v3126 != 0.0 {
                                                v3610 = v0;
                                            } else {
                                                let v3130 = (v2919 * v83) / ((v2979 * v3020) * v2914);
                                                v3610 = v3130;
                                            }
                                            v3609 = v3610;
                                        } else {
                                            v3609 = v0;
                                        }
                                        v3607 = v3609;
                                    }
                                    v3606 = v3607;
                                } else {
                                    let v3135 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3611: f64;
                                    if v3135 != 0.0 {
                                        let v3136 = if v3020 == v0 { 1.0 } else { 0.0 };
                                        let v3612: f64;
                                        if v3136 != 0.0 {
                                            v3612 = v0;
                                        } else {
                                            let v3139 = (v2919 * v2914) / (v83 * v3020);
                                            v3612 = v3139;
                                        }
                                        v3611 = v3612;
                                    } else {
                                        let v3144 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3613: f64;
                                        if v3144 != 0.0 {
                                            let v3145 = if v2914 == v0 { 1.0 } else { 0.0 };
                                            if v3145 != 0.0 {
                                            } else {
                                            }
                                            let v3147 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3145 != 0.0 { 1.0 } else { 0.0 };
                                            let v3614: f64;
                                            if v3147 != 0.0 {
                                                v3614 = v0;
                                            } else {
                                                let v3151 = (v2919 * v83) / ((v2979 * v3020) * v2914);
                                                v3614 = v3151;
                                            }
                                            v3613 = v3614;
                                        } else {
                                            v3613 = v0;
                                        }
                                        v3611 = v3613;
                                    }
                                    v3606 = v3611;
                                }
                                v3596 = v3606;
                            }
                            v3560 = v3556;
                            v3595 = v3596;
                        } else {
                            let v3152 = if v2926 == v71 { 1.0 } else { 0.0 };
                            let v3561: f64;
                            let v3615: f64;
                            if v3152 != 0.0 {
                                let v3616: f64;
                                if v3153 != 0.0 {
                                    let v3617: f64;
                                    if v3154 != 0.0 {
                                        let v3159 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3618: f64;
                                        if v3159 != 0.0 {
                                            let v3160 = if v2967 == v0 { 1.0 } else { 0.0 };
                                            let v3619: f64;
                                            if v3160 != 0.0 {
                                                v3619 = v0;
                                            } else {
                                                let v3163 = (v2919 * v2914) / (v83 * v2967);
                                                v3619 = v3163;
                                            }
                                            v3618 = v3619;
                                        } else {
                                            let v3168 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3620: f64;
                                            if v3168 != 0.0 {
                                                let v3169 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                if v3169 != 0.0 {
                                                } else {
                                                }
                                                let v3171 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3169 != 0.0 { 1.0 } else { 0.0 };
                                                let v3621: f64;
                                                if v3171 != 0.0 {
                                                    v3621 = v0;
                                                } else {
                                                    let v3175 = (v2919 * v83) / ((v2979 * v2967) * v2914);
                                                    v3621 = v3175;
                                                }
                                                v3620 = v3621;
                                            } else {
                                                v3620 = v0;
                                            }
                                            v3618 = v3620;
                                        }
                                        v3617 = v3618;
                                    } else {
                                        let v3180 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3622: f64;
                                        if v3180 != 0.0 {
                                            let v3181 = if v2967 == v0 { 1.0 } else { 0.0 };
                                            let v3623: f64;
                                            if v3181 != 0.0 {
                                                v3623 = v0;
                                            } else {
                                                let v3184 = (v2919 * v2914) / (v83 * v2967);
                                                v3623 = v3184;
                                            }
                                            v3622 = v3623;
                                        } else {
                                            let v3189 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3624: f64;
                                            if v3189 != 0.0 {
                                                let v3190 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                if v3190 != 0.0 {
                                                } else {
                                                }
                                                let v3192 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3190 != 0.0 { 1.0 } else { 0.0 };
                                                let v3625: f64;
                                                if v3192 != 0.0 {
                                                    v3625 = v0;
                                                } else {
                                                    let v3196 = (v2919 * v83) / ((v2979 * v2967) * v2914);
                                                    v3625 = v3196;
                                                }
                                                v3624 = v3625;
                                            } else {
                                                v3624 = v0;
                                            }
                                            v3622 = v3624;
                                        }
                                        v3617 = v3622;
                                    }
                                    v3616 = v3617;
                                } else {
                                    let v3626: f64;
                                    if v3197 != 0.0 {
                                        let v3202 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3627: f64;
                                        if v3202 != 0.0 {
                                            let v3203 = if v3020 == v0 { 1.0 } else { 0.0 };
                                            let v3628: f64;
                                            if v3203 != 0.0 {
                                                v3628 = v0;
                                            } else {
                                                let v3206 = (v2919 * v2914) / (v83 * v3020);
                                                v3628 = v3206;
                                            }
                                            v3627 = v3628;
                                        } else {
                                            let v3211 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3629: f64;
                                            if v3211 != 0.0 {
                                                let v3212 = v2914 + v2915;
                                                let v3213 = if v3212 == v0 { 1.0 } else { 0.0 };
                                                if v3213 != 0.0 {
                                                } else {
                                                }
                                                let v3215 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3213 != 0.0 { 1.0 } else { 0.0 };
                                                let v3630: f64;
                                                if v3215 != 0.0 {
                                                    v3630 = v0;
                                                } else {
                                                    let v3219 = (v2919 * v83) / ((v2974 * v3020) * v3212);
                                                    v3630 = v3219;
                                                }
                                                v3629 = v3630;
                                            } else {
                                                v3629 = v0;
                                            }
                                            v3627 = v3629;
                                        }
                                        v3626 = v3627;
                                    } else {
                                        let v3224 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3631: f64;
                                        if v3224 != 0.0 {
                                            let v3225 = if v3020 == v0 { 1.0 } else { 0.0 };
                                            let v3632: f64;
                                            if v3225 != 0.0 {
                                                v3632 = v0;
                                            } else {
                                                let v3228 = (v2919 * v2914) / (v83 * v3020);
                                                v3632 = v3228;
                                            }
                                            v3631 = v3632;
                                        } else {
                                            let v3233 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3633: f64;
                                            if v3233 != 0.0 {
                                                let v3234 = v2914 + v2915;
                                                let v3235 = if v3234 == v0 { 1.0 } else { 0.0 };
                                                if v3235 != 0.0 {
                                                } else {
                                                }
                                                let v3237 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3235 != 0.0 { 1.0 } else { 0.0 };
                                                let v3634: f64;
                                                if v3237 != 0.0 {
                                                    v3634 = v0;
                                                } else {
                                                    let v3241 = (v2919 * v83) / ((v2974 * v3020) * v3234);
                                                    v3634 = v3241;
                                                }
                                                v3633 = v3634;
                                            } else {
                                                v3633 = v0;
                                            }
                                            v3631 = v3633;
                                        }
                                        v3626 = v3631;
                                    }
                                    v3616 = v3626;
                                }
                                v3561 = v3556;
                                v3615 = v3616;
                            } else {
                                let v3242 = if v2926 == v2974 { 1.0 } else { 0.0 };
                                let v3562: f64;
                                let v3635: f64;
                                if v3242 != 0.0 {
                                    let v3636: f64;
                                    if v3243 != 0.0 {
                                        let v3637: f64;
                                        if v3244 != 0.0 {
                                            let v3249 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3638: f64;
                                            if v3249 != 0.0 {
                                                let v3250 = if v2967 == v0 { 1.0 } else { 0.0 };
                                                let v3639: f64;
                                                if v3250 != 0.0 {
                                                    v3639 = v0;
                                                } else {
                                                    let v3253 = (v2919 * v2914) / (v83 * v2967);
                                                    v3639 = v3253;
                                                }
                                                v3638 = v3639;
                                            } else {
                                                let v3258 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3640: f64;
                                                if v3258 != 0.0 {
                                                    let v3259 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                    if v3259 != 0.0 {
                                                    } else {
                                                    }
                                                    let v3261 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3259 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3641: f64;
                                                    if v3261 != 0.0 {
                                                        v3641 = v0;
                                                    } else {
                                                        let v3265 = (v2919 * v83) / ((v2979 * v2967) * v2914);
                                                        v3641 = v3265;
                                                    }
                                                    v3640 = v3641;
                                                } else {
                                                    v3640 = v0;
                                                }
                                                v3638 = v3640;
                                            }
                                            v3637 = v3638;
                                        } else {
                                            let v3270 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3642: f64;
                                            if v3270 != 0.0 {
                                                let v3271 = if v2967 == v0 { 1.0 } else { 0.0 };
                                                let v3643: f64;
                                                if v3271 != 0.0 {
                                                    v3643 = v0;
                                                } else {
                                                    let v3274 = (v2919 * v2914) / (v83 * v2967);
                                                    v3643 = v3274;
                                                }
                                                v3642 = v3643;
                                            } else {
                                                let v3279 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3644: f64;
                                                if v3279 != 0.0 {
                                                    let v3280 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                    if v3280 != 0.0 {
                                                    } else {
                                                    }
                                                    let v3282 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3280 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3645: f64;
                                                    if v3282 != 0.0 {
                                                        v3645 = v0;
                                                    } else {
                                                        let v3286 = (v2919 * v83) / ((v2979 * v2967) * v2914);
                                                        v3645 = v3286;
                                                    }
                                                    v3644 = v3645;
                                                } else {
                                                    v3644 = v0;
                                                }
                                                v3642 = v3644;
                                            }
                                            v3637 = v3642;
                                        }
                                        v3636 = v3637;
                                    } else {
                                        let v3646: f64;
                                        if v3287 != 0.0 {
                                            let v3292 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3647: f64;
                                            if v3292 != 0.0 {
                                                let v3293 = if v3020 == v0 { 1.0 } else { 0.0 };
                                                let v3648: f64;
                                                if v3293 != 0.0 {
                                                    v3648 = v0;
                                                } else {
                                                    let v3296 = (v2919 * v2914) / (v83 * v3020);
                                                    v3648 = v3296;
                                                }
                                                v3647 = v3648;
                                            } else {
                                                let v3301 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3649: f64;
                                                if v3301 != 0.0 {
                                                    let v3302 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                    if v3302 != 0.0 {
                                                    } else {
                                                    }
                                                    let v3304 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3302 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3650: f64;
                                                    if v3304 != 0.0 {
                                                        v3650 = v0;
                                                    } else {
                                                        let v3308 = (v2919 * v83) / ((v2979 * v3020) * v2914);
                                                        v3650 = v3308;
                                                    }
                                                    v3649 = v3650;
                                                } else {
                                                    v3649 = v0;
                                                }
                                                v3647 = v3649;
                                            }
                                            v3646 = v3647;
                                        } else {
                                            let v3313 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3651: f64;
                                            if v3313 != 0.0 {
                                                let v3314 = if v3020 == v0 { 1.0 } else { 0.0 };
                                                let v3652: f64;
                                                if v3314 != 0.0 {
                                                    v3652 = v0;
                                                } else {
                                                    let v3317 = (v2919 * v2914) / (v83 * v3020);
                                                    v3652 = v3317;
                                                }
                                                v3651 = v3652;
                                            } else {
                                                let v3322 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3653: f64;
                                                if v3322 != 0.0 {
                                                    let v3323 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                    if v3323 != 0.0 {
                                                    } else {
                                                    }
                                                    let v3325 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3323 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3654: f64;
                                                    if v3325 != 0.0 {
                                                        v3654 = v0;
                                                    } else {
                                                        let v3329 = (v2919 * v83) / ((v2979 * v3020) * v2914);
                                                        v3654 = v3329;
                                                    }
                                                    v3653 = v3654;
                                                } else {
                                                    v3653 = v0;
                                                }
                                                v3651 = v3653;
                                            }
                                            v3646 = v3651;
                                        }
                                        v3636 = v3646;
                                    }
                                    v3562 = v3556;
                                    v3635 = v3636;
                                } else {
                                    let v3330 = if v2926 == v2976 { 1.0 } else { 0.0 };
                                    let v3563: f64;
                                    let v3655: f64;
                                    if v3330 != 0.0 {
                                        let v3656: f64;
                                        if v3331 != 0.0 {
                                            let v3657: f64;
                                            if v3332 != 0.0 {
                                                let v3337 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3658: f64;
                                                if v3337 != 0.0 {
                                                    let v3338 = if v2967 == v0 { 1.0 } else { 0.0 };
                                                    let v3659: f64;
                                                    if v3338 != 0.0 {
                                                        v3659 = v0;
                                                    } else {
                                                        let v3341 = (v2919 * v2914) / (v83 * v2967);
                                                        v3659 = v3341;
                                                    }
                                                    v3658 = v3659;
                                                } else {
                                                    let v3346 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3660: f64;
                                                    if v3346 != 0.0 {
                                                        let v3347 = v2914 + v2915;
                                                        let v3348 = if v3347 == v0 { 1.0 } else { 0.0 };
                                                        if v3348 != 0.0 {
                                                        } else {
                                                        }
                                                        let v3350 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3348 != 0.0 { 1.0 } else { 0.0 };
                                                        let v3661: f64;
                                                        if v3350 != 0.0 {
                                                            v3661 = v0;
                                                        } else {
                                                            let v3354 = (v2919 * v83) / ((v2974 * v2967) * v3347);
                                                            v3661 = v3354;
                                                        }
                                                        v3660 = v3661;
                                                    } else {
                                                        v3660 = v0;
                                                    }
                                                    v3658 = v3660;
                                                }
                                                v3657 = v3658;
                                            } else {
                                                let v3359 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3662: f64;
                                                if v3359 != 0.0 {
                                                    let v3360 = if v2967 == v0 { 1.0 } else { 0.0 };
                                                    let v3663: f64;
                                                    if v3360 != 0.0 {
                                                        v3663 = v0;
                                                    } else {
                                                        let v3363 = (v2919 * v2914) / (v83 * v2967);
                                                        v3663 = v3363;
                                                    }
                                                    v3662 = v3663;
                                                } else {
                                                    let v3368 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3664: f64;
                                                    if v3368 != 0.0 {
                                                        let v3369 = v2914 + v2915;
                                                        let v3370 = if v3369 == v0 { 1.0 } else { 0.0 };
                                                        if v3370 != 0.0 {
                                                        } else {
                                                        }
                                                        let v3372 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3370 != 0.0 { 1.0 } else { 0.0 };
                                                        let v3665: f64;
                                                        if v3372 != 0.0 {
                                                            v3665 = v0;
                                                        } else {
                                                            let v3376 = (v2919 * v83) / ((v2974 * v2967) * v3369);
                                                            v3665 = v3376;
                                                        }
                                                        v3664 = v3665;
                                                    } else {
                                                        v3664 = v0;
                                                    }
                                                    v3662 = v3664;
                                                }
                                                v3657 = v3662;
                                            }
                                            v3656 = v3657;
                                        } else {
                                            let v3378 = (v2919 * v2917) / v83;
                                            v3656 = v3378;
                                        }
                                        v3563 = v3556;
                                        v3655 = v3656;
                                    } else {
                                        let v3379 = if v2926 == v2964 { 1.0 } else { 0.0 };
                                        let v3564: f64;
                                        let v3666: f64;
                                        if v3379 != 0.0 {
                                            let v3667: f64;
                                            if v3380 != 0.0 {
                                                let v3668: f64;
                                                if v3381 != 0.0 {
                                                    let v3386 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3669: f64;
                                                    if v3386 != 0.0 {
                                                        let v3387 = if v2967 == v0 { 1.0 } else { 0.0 };
                                                        let v3670: f64;
                                                        if v3387 != 0.0 {
                                                            v3670 = v0;
                                                        } else {
                                                            let v3390 = (v2919 * v2914) / (v83 * v2967);
                                                            v3670 = v3390;
                                                        }
                                                        v3669 = v3670;
                                                    } else {
                                                        let v3395 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3671: f64;
                                                        if v3395 != 0.0 {
                                                            let v3396 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                            if v3396 != 0.0 {
                                                            } else {
                                                            }
                                                            let v3398 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3396 != 0.0 { 1.0 } else { 0.0 };
                                                            let v3672: f64;
                                                            if v3398 != 0.0 {
                                                                v3672 = v0;
                                                            } else {
                                                                let v3402 = (v2919 * v83) / ((v2979 * v2967) * v2914);
                                                                v3672 = v3402;
                                                            }
                                                            v3671 = v3672;
                                                        } else {
                                                            v3671 = v0;
                                                        }
                                                        v3669 = v3671;
                                                    }
                                                    v3668 = v3669;
                                                } else {
                                                    let v3407 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3673: f64;
                                                    if v3407 != 0.0 {
                                                        let v3408 = if v2967 == v0 { 1.0 } else { 0.0 };
                                                        let v3674: f64;
                                                        if v3408 != 0.0 {
                                                            v3674 = v0;
                                                        } else {
                                                            let v3411 = (v2919 * v2914) / (v83 * v2967);
                                                            v3674 = v3411;
                                                        }
                                                        v3673 = v3674;
                                                    } else {
                                                        let v3416 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3675: f64;
                                                        if v3416 != 0.0 {
                                                            let v3417 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                            if v3417 != 0.0 {
                                                            } else {
                                                            }
                                                            let v3419 = if (if v2967 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3417 != 0.0 { 1.0 } else { 0.0 };
                                                            let v3676: f64;
                                                            if v3419 != 0.0 {
                                                                v3676 = v0;
                                                            } else {
                                                                let v3423 = (v2919 * v83) / ((v2979 * v2967) * v2914);
                                                                v3676 = v3423;
                                                            }
                                                            v3675 = v3676;
                                                        } else {
                                                            v3675 = v0;
                                                        }
                                                        v3673 = v3675;
                                                    }
                                                    v3668 = v3673;
                                                }
                                                v3667 = v3668;
                                            } else {
                                                let v3424 = if v3020 == v0 { 1.0 } else { 0.0 };
                                                let v3677: f64;
                                                if v3424 != 0.0 {
                                                    v3677 = v0;
                                                } else {
                                                    let v3427 = (v2919 * v2917) / (v83 * v3020);
                                                    v3677 = v3427;
                                                }
                                                v3667 = v3677;
                                            }
                                            v3564 = v3556;
                                            v3666 = v3667;
                                        } else {
                                            let v3428 = if v2926 == v2979 { 1.0 } else { 0.0 };
                                            let v3565: f64;
                                            let v3678: f64;
                                            if v3428 != 0.0 {
                                                let v3679: f64;
                                                if v3429 != 0.0 {
                                                    let v3431 = (v2919 * v2917) / v83;
                                                    v3679 = v3431;
                                                } else {
                                                    let v3680: f64;
                                                    if v3432 != 0.0 {
                                                        let v3437 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3681: f64;
                                                        if v3437 != 0.0 {
                                                            let v3438 = if v3020 == v0 { 1.0 } else { 0.0 };
                                                            let v3682: f64;
                                                            if v3438 != 0.0 {
                                                                v3682 = v0;
                                                            } else {
                                                                let v3441 = (v2919 * v2914) / (v83 * v3020);
                                                                v3682 = v3441;
                                                            }
                                                            v3681 = v3682;
                                                        } else {
                                                            let v3446 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3683: f64;
                                                            if v3446 != 0.0 {
                                                                let v3447 = v2914 + v2915;
                                                                let v3448 = if v3447 == v0 { 1.0 } else { 0.0 };
                                                                if v3448 != 0.0 {
                                                                } else {
                                                                }
                                                                let v3450 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3448 != 0.0 { 1.0 } else { 0.0 };
                                                                let v3684: f64;
                                                                if v3450 != 0.0 {
                                                                    v3684 = v0;
                                                                } else {
                                                                    let v3454 = (v2919 * v83) / ((v2974 * v3020) * v3447);
                                                                    v3684 = v3454;
                                                                }
                                                                v3683 = v3684;
                                                            } else {
                                                                v3683 = v0;
                                                            }
                                                            v3681 = v3683;
                                                        }
                                                        v3680 = v3681;
                                                    } else {
                                                        let v3459 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3685: f64;
                                                        if v3459 != 0.0 {
                                                            let v3460 = if v3020 == v0 { 1.0 } else { 0.0 };
                                                            let v3686: f64;
                                                            if v3460 != 0.0 {
                                                                v3686 = v0;
                                                            } else {
                                                                let v3463 = (v2919 * v2914) / (v83 * v3020);
                                                                v3686 = v3463;
                                                            }
                                                            v3685 = v3686;
                                                        } else {
                                                            let v3468 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3687: f64;
                                                            if v3468 != 0.0 {
                                                                let v3469 = v2914 + v2915;
                                                                let v3470 = if v3469 == v0 { 1.0 } else { 0.0 };
                                                                if v3470 != 0.0 {
                                                                } else {
                                                                }
                                                                let v3472 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3470 != 0.0 { 1.0 } else { 0.0 };
                                                                let v3688: f64;
                                                                if v3472 != 0.0 {
                                                                    v3688 = v0;
                                                                } else {
                                                                    let v3476 = (v2919 * v83) / ((v2974 * v3020) * v3469);
                                                                    v3688 = v3476;
                                                                }
                                                                v3687 = v3688;
                                                            } else {
                                                                v3687 = v0;
                                                            }
                                                            v3685 = v3687;
                                                        }
                                                        v3680 = v3685;
                                                    }
                                                    v3679 = v3680;
                                                }
                                                v3565 = v3556;
                                                v3678 = v3679;
                                            } else {
                                                let v3477 = if v2926 == v2993 { 1.0 } else { 0.0 };
                                                let v3566: f64;
                                                let v3689: f64;
                                                if v3477 != 0.0 {
                                                    let v3690: f64;
                                                    if v3478 != 0.0 {
                                                        let v3479 = if v2967 == v0 { 1.0 } else { 0.0 };
                                                        let v3691: f64;
                                                        if v3479 != 0.0 {
                                                            v3691 = v0;
                                                        } else {
                                                            let v3482 = (v2919 * v2917) / (v83 * v2967);
                                                            v3691 = v3482;
                                                        }
                                                        v3690 = v3691;
                                                    } else {
                                                        let v3692: f64;
                                                        if v3483 != 0.0 {
                                                            let v3488 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3693: f64;
                                                            if v3488 != 0.0 {
                                                                let v3489 = if v3020 == v0 { 1.0 } else { 0.0 };
                                                                let v3694: f64;
                                                                if v3489 != 0.0 {
                                                                    v3694 = v0;
                                                                } else {
                                                                    let v3492 = (v2919 * v2914) / (v83 * v3020);
                                                                    v3694 = v3492;
                                                                }
                                                                v3693 = v3694;
                                                            } else {
                                                                let v3497 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let v3695: f64;
                                                                if v3497 != 0.0 {
                                                                    let v3498 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                                    if v3498 != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let v3500 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3498 != 0.0 { 1.0 } else { 0.0 };
                                                                    let v3696: f64;
                                                                    if v3500 != 0.0 {
                                                                        v3696 = v0;
                                                                    } else {
                                                                        let v3504 = (v2919 * v83) / ((v2979 * v3020) * v2914);
                                                                        v3696 = v3504;
                                                                    }
                                                                    v3695 = v3696;
                                                                } else {
                                                                    v3695 = v0;
                                                                }
                                                                v3693 = v3695;
                                                            }
                                                            v3692 = v3693;
                                                        } else {
                                                            let v3509 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3697: f64;
                                                            if v3509 != 0.0 {
                                                                let v3510 = if v3020 == v0 { 1.0 } else { 0.0 };
                                                                let v3698: f64;
                                                                if v3510 != 0.0 {
                                                                    v3698 = v0;
                                                                } else {
                                                                    let v3513 = (v2919 * v2914) / (v83 * v3020);
                                                                    v3698 = v3513;
                                                                }
                                                                v3697 = v3698;
                                                            } else {
                                                                let v3518 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let v3699: f64;
                                                                if v3518 != 0.0 {
                                                                    let v3519 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                                    if v3519 != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let v3521 = if (if v3020 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3519 != 0.0 { 1.0 } else { 0.0 };
                                                                    let v3700: f64;
                                                                    if v3521 != 0.0 {
                                                                        v3700 = v0;
                                                                    } else {
                                                                        let v3525 = (v2919 * v83) / ((v2979 * v3020) * v2914);
                                                                        v3700 = v3525;
                                                                    }
                                                                    v3699 = v3700;
                                                                } else {
                                                                    v3699 = v0;
                                                                }
                                                                v3697 = v3699;
                                                            }
                                                            v3692 = v3697;
                                                        }
                                                        v3690 = v3692;
                                                    }
                                                    v3566 = v3556;
                                                    v3689 = v3690;
                                                } else {
                                                    let v3526 = if v2926 == v3003 { 1.0 } else { 0.0 };
                                                    let v3567: f64;
                                                    let v3701: f64;
                                                    if v3526 != 0.0 {
                                                        let v3528 = (v2919 * v2917) / v83;
                                                        v3567 = v3556;
                                                        v3701 = v3528;
                                                    } else {
                                                        let v3529 = if v2926 == v2927 { 1.0 } else { 0.0 };
                                                        let v3568: f64;
                                                        let v3702: f64;
                                                        if v3529 != 0.0 {
                                                            let v3569: f64;
                                                            let v3703: f64;
                                                            if v3530 != 0.0 {
                                                                let v3533 = ((v2485 * v2919) * v2914) / v83;
                                                                let v3534 = if v32 == v71 { 1.0 } else { 0.0 };
                                                                let v3570: f64;
                                                                if v3534 != 0.0 {
                                                                    v3570 = v0;
                                                                } else {
                                                                    let v3538 = (v2919 * v2914) / (v83 * (v32 - v71));
                                                                    v3570 = v3538;
                                                                }
                                                                v3569 = v3570;
                                                                v3703 = v3533;
                                                            } else {
                                                                let v3541 = (v2919 * v2914) / (v83 * v32);
                                                                v3569 = v3541;
                                                                v3703 = v0;
                                                            }
                                                            v3568 = v3569;
                                                            v3702 = v3703;
                                                        } else {
                                                            let v3542 = if v2926 == v2908 { 1.0 } else { 0.0 };
                                                            let v3571: f64;
                                                            let v3704: f64;
                                                            if v3542 != 0.0 {
                                                                let v3572: f64;
                                                                let v3705: f64;
                                                                if v3543 != 0.0 {
                                                                    let v3546 = (v2919 * v2914) / (v83 * v32);
                                                                    v3572 = v3546;
                                                                    v3705 = v0;
                                                                } else {
                                                                    let v3549 = ((v2485 * v2919) * v2914) / v83;
                                                                    let v3550 = if v32 == v71 { 1.0 } else { 0.0 };
                                                                    let v3573: f64;
                                                                    if v3550 != 0.0 {
                                                                        v3573 = v0;
                                                                    } else {
                                                                        let v3554 = (v2919 * v2914) / (v83 * (v32 - v71));
                                                                        v3573 = v3554;
                                                                    }
                                                                    v3572 = v3573;
                                                                    v3705 = v3549;
                                                                }
                                                                v3571 = v3572;
                                                                v3704 = v3705;
                                                            } else {
                                                                v3571 = v0;
                                                                v3704 = v0;
                                                            }
                                                            v3568 = v3571;
                                                            v3702 = v3704;
                                                        }
                                                        v3567 = v3568;
                                                        v3701 = v3702;
                                                    }
                                                    v3566 = v3567;
                                                    v3689 = v3701;
                                                }
                                                v3565 = v3566;
                                                v3678 = v3689;
                                            }
                                            v3564 = v3565;
                                            v3666 = v3678;
                                        }
                                        v3563 = v3564;
                                        v3655 = v3666;
                                    }
                                    v3562 = v3563;
                                    v3635 = v3655;
                                }
                                v3561 = v3562;
                                v3615 = v3635;
                            }
                            v3560 = v3561;
                            v3595 = v3615;
                        }
                        v3555 = v3560;
                        v3575 = v3595;
                    }
                    let v3574 = if v3555 <= v0 { 1.0 } else { 0.0 };
                    let v3710: f64;
                    if v3574 != 0.0 {
                        v3710 = v3575;
                    } else {
                        let v3706 = if v3575 <= v0 { 1.0 } else { 0.0 };
                        let v3711: f64;
                        if v3706 != 0.0 {
                            v3711 = v3555;
                        } else {
                            let v3709 = (v3555 * v3575) / (v3555 + v3575);
                            v3711 = v3709;
                        }
                        v3710 = v3711;
                    }
                    let v3712 = if v3710 == v0 { 1.0 } else { 0.0 };
                    if v3712 != 0.0 {
                    } else {
                    }
                    v3760 = v2967;
                    v3810 = v3020;
                    v4349 = v3555;
                    v4497 = v3575;
                    v4507 = v3710;
                    v5078 = v5079;
                    v5095 = v5096;
                } else {
                    v3760 = v0;
                    v3810 = v0;
                    v4349 = v0;
                    v4497 = v0;
                    v4507 = v0;
                    v5078 = v0;
                    v5095 = v0;
                }
                v3759 = v3760;
                v3809 = v3810;
                v4348 = v4349;
                v4496 = v4497;
                v4506 = v4507;
                v5077 = v5078;
                v5094 = v5095;
            }
            let v4510: f64;
            let v5070: f64;
            let v5076: f64;
            let v5087: f64;
            let v5093: f64;
            if v3713 != 0.0 {
                let v3715 = v2919 * v3714;
                v4510 = v3715;
                v5070 = v3759;
                v5076 = v5077;
                v5087 = v3809;
                v5093 = v5094;
            } else {
                let v3718 = if (if v2922 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v2919 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4511: f64;
                let v5071: f64;
                let v5080: f64;
                let v5088: f64;
                let v5097: f64;
                if v3718 != 0.0 {
                    let v3719 = if v2926 < v2927 { 1.0 } else { 0.0 };
                    let v3756: f64;
                    let v3806: f64;
                    let v4344: f64;
                    let v5081: f64;
                    let v5098: f64;
                    if v3719 != 0.0 {
                        let v3721 = if (v32 % v71) != v0 { 1.0 } else { 0.0 };
                        let v3736: f64;
                        let v3742: f64;
                        let v3757: f64;
                        let v3807: f64;
                        if v3721 != 0.0 {
                            let v3725 = v71 * (if ((v32 - v1) / v71) >= v0 { ((v32 - v1) / v71) } else { v0 });
                            v3736 = v3725;
                            v3742 = v3725;
                            v3757 = v1;
                            v3807 = v1;
                        } else {
                            let v3726 = if v2935 == v1 { 1.0 } else { 0.0 };
                            let v3737: f64;
                            let v3743: f64;
                            let v3758: f64;
                            let v3808: f64;
                            if v3726 != 0.0 {
                                let v3730 = v71 * (if ((v32 / v71) - v1) >= v0 { ((v32 / v71) - v1) } else { v0 });
                                v3737 = v32;
                                v3743 = v3730;
                                v3758 = v0;
                                v3808 = v71;
                            } else {
                                let v3734 = v71 * (if ((v32 / v71) - v1) >= v0 { ((v32 / v71) - v1) } else { v0 });
                                v3737 = v3734;
                                v3743 = v32;
                                v3758 = v71;
                                v3808 = v0;
                            }
                            v3736 = v3737;
                            v3742 = v3743;
                            v3757 = v3758;
                            v3807 = v3808;
                        }
                        let v4345: f64;
                        if v3735 != 0.0 {
                            let v3738 = if v3736 == v0 { 1.0 } else { 0.0 };
                            let v4346: f64;
                            if v3738 != 0.0 {
                                v4346 = v0;
                            } else {
                                let v3741 = (v2919 * v2914) / (v83 * v3736);
                                v4346 = v3741;
                            }
                            v4345 = v4346;
                        } else {
                            let v3744 = if v3742 == v0 { 1.0 } else { 0.0 };
                            let v4347: f64;
                            if v3744 != 0.0 {
                                v4347 = v0;
                            } else {
                                let v3747 = (v2919 * v2914) / (v83 * v3742);
                                v4347 = v3747;
                            }
                            v4345 = v4347;
                        }
                        v3756 = v3757;
                        v3806 = v3807;
                        v4344 = v4345;
                        v5081 = v3736;
                        v5098 = v3742;
                    } else {
                        v3756 = v3759;
                        v3806 = v3809;
                        v4344 = v4348;
                        v5081 = v5077;
                        v5098 = v5094;
                    }
                    let v3748 = if v2926 == v0 { 1.0 } else { 0.0 };
                    let v4343: f64;
                    let v4365: f64;
                    if v3748 != 0.0 {
                        let v4366: f64;
                        if v3749 != 0.0 {
                            let v4367: f64;
                            if v3750 != 0.0 {
                                let v3755 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v4368: f64;
                                if v3755 != 0.0 {
                                    let v3761 = if v3756 == v0 { 1.0 } else { 0.0 };
                                    let v4369: f64;
                                    if v3761 != 0.0 {
                                        v4369 = v0;
                                    } else {
                                        let v3764 = (v2919 * v2914) / (v83 * v3756);
                                        v4369 = v3764;
                                    }
                                    v4368 = v4369;
                                } else {
                                    let v3769 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v4370: f64;
                                    if v3769 != 0.0 {
                                        let v3770 = v2914 + v2915;
                                        let v3771 = if v3770 == v0 { 1.0 } else { 0.0 };
                                        if v3771 != 0.0 {
                                        } else {
                                        }
                                        let v3773 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3771 != 0.0 { 1.0 } else { 0.0 };
                                        let v4371: f64;
                                        if v3773 != 0.0 {
                                            v4371 = v0;
                                        } else {
                                            let v3777 = (v2919 * v83) / ((v2974 * v3756) * v3770);
                                            v4371 = v3777;
                                        }
                                        v4370 = v4371;
                                    } else {
                                        v4370 = v0;
                                    }
                                    v4368 = v4370;
                                }
                                v4367 = v4368;
                            } else {
                                let v3782 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v4372: f64;
                                if v3782 != 0.0 {
                                    let v3783 = if v3756 == v0 { 1.0 } else { 0.0 };
                                    let v4373: f64;
                                    if v3783 != 0.0 {
                                        v4373 = v0;
                                    } else {
                                        let v3786 = (v2919 * v2914) / (v83 * v3756);
                                        v4373 = v3786;
                                    }
                                    v4372 = v4373;
                                } else {
                                    let v3791 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v4374: f64;
                                    if v3791 != 0.0 {
                                        let v3792 = v2914 + v2915;
                                        let v3793 = if v3792 == v0 { 1.0 } else { 0.0 };
                                        if v3793 != 0.0 {
                                        } else {
                                        }
                                        let v3795 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3793 != 0.0 { 1.0 } else { 0.0 };
                                        let v4375: f64;
                                        if v3795 != 0.0 {
                                            v4375 = v0;
                                        } else {
                                            let v3799 = (v2919 * v83) / ((v2974 * v3756) * v3792);
                                            v4375 = v3799;
                                        }
                                        v4374 = v4375;
                                    } else {
                                        v4374 = v0;
                                    }
                                    v4372 = v4374;
                                }
                                v4367 = v4372;
                            }
                            v4366 = v4367;
                        } else {
                            let v4376: f64;
                            if v3800 != 0.0 {
                                let v3805 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v4377: f64;
                                if v3805 != 0.0 {
                                    let v3811 = if v3806 == v0 { 1.0 } else { 0.0 };
                                    let v4378: f64;
                                    if v3811 != 0.0 {
                                        v4378 = v0;
                                    } else {
                                        let v3814 = (v2919 * v2914) / (v83 * v3806);
                                        v4378 = v3814;
                                    }
                                    v4377 = v4378;
                                } else {
                                    let v3819 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v4379: f64;
                                    if v3819 != 0.0 {
                                        let v3820 = v2914 + v2915;
                                        let v3821 = if v3820 == v0 { 1.0 } else { 0.0 };
                                        if v3821 != 0.0 {
                                        } else {
                                        }
                                        let v3823 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3821 != 0.0 { 1.0 } else { 0.0 };
                                        let v4380: f64;
                                        if v3823 != 0.0 {
                                            v4380 = v0;
                                        } else {
                                            let v3827 = (v2919 * v83) / ((v2974 * v3806) * v3820);
                                            v4380 = v3827;
                                        }
                                        v4379 = v4380;
                                    } else {
                                        v4379 = v0;
                                    }
                                    v4377 = v4379;
                                }
                                v4376 = v4377;
                            } else {
                                let v3832 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v4381: f64;
                                if v3832 != 0.0 {
                                    let v3833 = if v3806 == v0 { 1.0 } else { 0.0 };
                                    let v4382: f64;
                                    if v3833 != 0.0 {
                                        v4382 = v0;
                                    } else {
                                        let v3836 = (v2919 * v2914) / (v83 * v3806);
                                        v4382 = v3836;
                                    }
                                    v4381 = v4382;
                                } else {
                                    let v3841 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v4383: f64;
                                    if v3841 != 0.0 {
                                        let v3842 = v2914 + v2915;
                                        let v3843 = if v3842 == v0 { 1.0 } else { 0.0 };
                                        if v3843 != 0.0 {
                                        } else {
                                        }
                                        let v3845 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3843 != 0.0 { 1.0 } else { 0.0 };
                                        let v4384: f64;
                                        if v3845 != 0.0 {
                                            v4384 = v0;
                                        } else {
                                            let v3849 = (v2919 * v83) / ((v2974 * v3806) * v3842);
                                            v4384 = v3849;
                                        }
                                        v4383 = v4384;
                                    } else {
                                        v4383 = v0;
                                    }
                                    v4381 = v4383;
                                }
                                v4376 = v4381;
                            }
                            v4366 = v4376;
                        }
                        v4343 = v4344;
                        v4365 = v4366;
                    } else {
                        let v3850 = if v2926 == v1 { 1.0 } else { 0.0 };
                        let v4350: f64;
                        let v4385: f64;
                        if v3850 != 0.0 {
                            let v4386: f64;
                            if v3851 != 0.0 {
                                let v4387: f64;
                                if v3852 != 0.0 {
                                    let v3857 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v4388: f64;
                                    if v3857 != 0.0 {
                                        let v3858 = if v3756 == v0 { 1.0 } else { 0.0 };
                                        let v4389: f64;
                                        if v3858 != 0.0 {
                                            v4389 = v0;
                                        } else {
                                            let v3861 = (v2919 * v2914) / (v83 * v3756);
                                            v4389 = v3861;
                                        }
                                        v4388 = v4389;
                                    } else {
                                        let v3866 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v4390: f64;
                                        if v3866 != 0.0 {
                                            let v3867 = v2914 + v2915;
                                            let v3868 = if v3867 == v0 { 1.0 } else { 0.0 };
                                            if v3868 != 0.0 {
                                            } else {
                                            }
                                            let v3870 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3868 != 0.0 { 1.0 } else { 0.0 };
                                            let v4391: f64;
                                            if v3870 != 0.0 {
                                                v4391 = v0;
                                            } else {
                                                let v3874 = (v2919 * v83) / ((v2974 * v3756) * v3867);
                                                v4391 = v3874;
                                            }
                                            v4390 = v4391;
                                        } else {
                                            v4390 = v0;
                                        }
                                        v4388 = v4390;
                                    }
                                    v4387 = v4388;
                                } else {
                                    let v3879 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v4392: f64;
                                    if v3879 != 0.0 {
                                        let v3880 = if v3756 == v0 { 1.0 } else { 0.0 };
                                        let v4393: f64;
                                        if v3880 != 0.0 {
                                            v4393 = v0;
                                        } else {
                                            let v3883 = (v2919 * v2914) / (v83 * v3756);
                                            v4393 = v3883;
                                        }
                                        v4392 = v4393;
                                    } else {
                                        let v3888 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v4394: f64;
                                        if v3888 != 0.0 {
                                            let v3889 = v2914 + v2915;
                                            let v3890 = if v3889 == v0 { 1.0 } else { 0.0 };
                                            if v3890 != 0.0 {
                                            } else {
                                            }
                                            let v3892 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3890 != 0.0 { 1.0 } else { 0.0 };
                                            let v4395: f64;
                                            if v3892 != 0.0 {
                                                v4395 = v0;
                                            } else {
                                                let v3896 = (v2919 * v83) / ((v2974 * v3756) * v3889);
                                                v4395 = v3896;
                                            }
                                            v4394 = v4395;
                                        } else {
                                            v4394 = v0;
                                        }
                                        v4392 = v4394;
                                    }
                                    v4387 = v4392;
                                }
                                v4386 = v4387;
                            } else {
                                let v4396: f64;
                                if v3897 != 0.0 {
                                    let v3902 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v4397: f64;
                                    if v3902 != 0.0 {
                                        let v3903 = if v3806 == v0 { 1.0 } else { 0.0 };
                                        let v4398: f64;
                                        if v3903 != 0.0 {
                                            v4398 = v0;
                                        } else {
                                            let v3906 = (v2919 * v2914) / (v83 * v3806);
                                            v4398 = v3906;
                                        }
                                        v4397 = v4398;
                                    } else {
                                        let v3911 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v4399: f64;
                                        if v3911 != 0.0 {
                                            let v3912 = if v2914 == v0 { 1.0 } else { 0.0 };
                                            if v3912 != 0.0 {
                                            } else {
                                            }
                                            let v3914 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3912 != 0.0 { 1.0 } else { 0.0 };
                                            let v4400: f64;
                                            if v3914 != 0.0 {
                                                v4400 = v0;
                                            } else {
                                                let v3918 = (v2919 * v83) / ((v2979 * v3806) * v2914);
                                                v4400 = v3918;
                                            }
                                            v4399 = v4400;
                                        } else {
                                            v4399 = v0;
                                        }
                                        v4397 = v4399;
                                    }
                                    v4396 = v4397;
                                } else {
                                    let v3923 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v4401: f64;
                                    if v3923 != 0.0 {
                                        let v3924 = if v3806 == v0 { 1.0 } else { 0.0 };
                                        let v4402: f64;
                                        if v3924 != 0.0 {
                                            v4402 = v0;
                                        } else {
                                            let v3927 = (v2919 * v2914) / (v83 * v3806);
                                            v4402 = v3927;
                                        }
                                        v4401 = v4402;
                                    } else {
                                        let v3932 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v4403: f64;
                                        if v3932 != 0.0 {
                                            let v3933 = if v2914 == v0 { 1.0 } else { 0.0 };
                                            if v3933 != 0.0 {
                                            } else {
                                            }
                                            let v3935 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3933 != 0.0 { 1.0 } else { 0.0 };
                                            let v4404: f64;
                                            if v3935 != 0.0 {
                                                v4404 = v0;
                                            } else {
                                                let v3939 = (v2919 * v83) / ((v2979 * v3806) * v2914);
                                                v4404 = v3939;
                                            }
                                            v4403 = v4404;
                                        } else {
                                            v4403 = v0;
                                        }
                                        v4401 = v4403;
                                    }
                                    v4396 = v4401;
                                }
                                v4386 = v4396;
                            }
                            v4350 = v4344;
                            v4385 = v4386;
                        } else {
                            let v3940 = if v2926 == v71 { 1.0 } else { 0.0 };
                            let v4351: f64;
                            let v4405: f64;
                            if v3940 != 0.0 {
                                let v4406: f64;
                                if v3941 != 0.0 {
                                    let v4407: f64;
                                    if v3942 != 0.0 {
                                        let v3947 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v4408: f64;
                                        if v3947 != 0.0 {
                                            let v3948 = if v3756 == v0 { 1.0 } else { 0.0 };
                                            let v4409: f64;
                                            if v3948 != 0.0 {
                                                v4409 = v0;
                                            } else {
                                                let v3951 = (v2919 * v2914) / (v83 * v3756);
                                                v4409 = v3951;
                                            }
                                            v4408 = v4409;
                                        } else {
                                            let v3956 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v4410: f64;
                                            if v3956 != 0.0 {
                                                let v3957 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                if v3957 != 0.0 {
                                                } else {
                                                }
                                                let v3959 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3957 != 0.0 { 1.0 } else { 0.0 };
                                                let v4411: f64;
                                                if v3959 != 0.0 {
                                                    v4411 = v0;
                                                } else {
                                                    let v3963 = (v2919 * v83) / ((v2979 * v3756) * v2914);
                                                    v4411 = v3963;
                                                }
                                                v4410 = v4411;
                                            } else {
                                                v4410 = v0;
                                            }
                                            v4408 = v4410;
                                        }
                                        v4407 = v4408;
                                    } else {
                                        let v3968 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v4412: f64;
                                        if v3968 != 0.0 {
                                            let v3969 = if v3756 == v0 { 1.0 } else { 0.0 };
                                            let v4413: f64;
                                            if v3969 != 0.0 {
                                                v4413 = v0;
                                            } else {
                                                let v3972 = (v2919 * v2914) / (v83 * v3756);
                                                v4413 = v3972;
                                            }
                                            v4412 = v4413;
                                        } else {
                                            let v3977 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v4414: f64;
                                            if v3977 != 0.0 {
                                                let v3978 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                if v3978 != 0.0 {
                                                } else {
                                                }
                                                let v3980 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3978 != 0.0 { 1.0 } else { 0.0 };
                                                let v4415: f64;
                                                if v3980 != 0.0 {
                                                    v4415 = v0;
                                                } else {
                                                    let v3984 = (v2919 * v83) / ((v2979 * v3756) * v2914);
                                                    v4415 = v3984;
                                                }
                                                v4414 = v4415;
                                            } else {
                                                v4414 = v0;
                                            }
                                            v4412 = v4414;
                                        }
                                        v4407 = v4412;
                                    }
                                    v4406 = v4407;
                                } else {
                                    let v4416: f64;
                                    if v3985 != 0.0 {
                                        let v3990 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v4417: f64;
                                        if v3990 != 0.0 {
                                            let v3991 = if v3806 == v0 { 1.0 } else { 0.0 };
                                            let v4418: f64;
                                            if v3991 != 0.0 {
                                                v4418 = v0;
                                            } else {
                                                let v3994 = (v2919 * v2914) / (v83 * v3806);
                                                v4418 = v3994;
                                            }
                                            v4417 = v4418;
                                        } else {
                                            let v3999 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v4419: f64;
                                            if v3999 != 0.0 {
                                                let v4000 = v2914 + v2915;
                                                let v4001 = if v4000 == v0 { 1.0 } else { 0.0 };
                                                if v4001 != 0.0 {
                                                } else {
                                                }
                                                let v4003 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4001 != 0.0 { 1.0 } else { 0.0 };
                                                let v4420: f64;
                                                if v4003 != 0.0 {
                                                    v4420 = v0;
                                                } else {
                                                    let v4007 = (v2919 * v83) / ((v2974 * v3806) * v4000);
                                                    v4420 = v4007;
                                                }
                                                v4419 = v4420;
                                            } else {
                                                v4419 = v0;
                                            }
                                            v4417 = v4419;
                                        }
                                        v4416 = v4417;
                                    } else {
                                        let v4012 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v4421: f64;
                                        if v4012 != 0.0 {
                                            let v4013 = if v3806 == v0 { 1.0 } else { 0.0 };
                                            let v4422: f64;
                                            if v4013 != 0.0 {
                                                v4422 = v0;
                                            } else {
                                                let v4016 = (v2919 * v2914) / (v83 * v3806);
                                                v4422 = v4016;
                                            }
                                            v4421 = v4422;
                                        } else {
                                            let v4021 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v4423: f64;
                                            if v4021 != 0.0 {
                                                let v4022 = v2914 + v2915;
                                                let v4023 = if v4022 == v0 { 1.0 } else { 0.0 };
                                                if v4023 != 0.0 {
                                                } else {
                                                }
                                                let v4025 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4023 != 0.0 { 1.0 } else { 0.0 };
                                                let v4424: f64;
                                                if v4025 != 0.0 {
                                                    v4424 = v0;
                                                } else {
                                                    let v4029 = (v2919 * v83) / ((v2974 * v3806) * v4022);
                                                    v4424 = v4029;
                                                }
                                                v4423 = v4424;
                                            } else {
                                                v4423 = v0;
                                            }
                                            v4421 = v4423;
                                        }
                                        v4416 = v4421;
                                    }
                                    v4406 = v4416;
                                }
                                v4351 = v4344;
                                v4405 = v4406;
                            } else {
                                let v4030 = if v2926 == v2974 { 1.0 } else { 0.0 };
                                let v4352: f64;
                                let v4425: f64;
                                if v4030 != 0.0 {
                                    let v4426: f64;
                                    if v4031 != 0.0 {
                                        let v4427: f64;
                                        if v4032 != 0.0 {
                                            let v4037 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v4428: f64;
                                            if v4037 != 0.0 {
                                                let v4038 = if v3756 == v0 { 1.0 } else { 0.0 };
                                                let v4429: f64;
                                                if v4038 != 0.0 {
                                                    v4429 = v0;
                                                } else {
                                                    let v4041 = (v2919 * v2914) / (v83 * v3756);
                                                    v4429 = v4041;
                                                }
                                                v4428 = v4429;
                                            } else {
                                                let v4046 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v4430: f64;
                                                if v4046 != 0.0 {
                                                    let v4047 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                    if v4047 != 0.0 {
                                                    } else {
                                                    }
                                                    let v4049 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4047 != 0.0 { 1.0 } else { 0.0 };
                                                    let v4431: f64;
                                                    if v4049 != 0.0 {
                                                        v4431 = v0;
                                                    } else {
                                                        let v4053 = (v2919 * v83) / ((v2979 * v3756) * v2914);
                                                        v4431 = v4053;
                                                    }
                                                    v4430 = v4431;
                                                } else {
                                                    v4430 = v0;
                                                }
                                                v4428 = v4430;
                                            }
                                            v4427 = v4428;
                                        } else {
                                            let v4058 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v4432: f64;
                                            if v4058 != 0.0 {
                                                let v4059 = if v3756 == v0 { 1.0 } else { 0.0 };
                                                let v4433: f64;
                                                if v4059 != 0.0 {
                                                    v4433 = v0;
                                                } else {
                                                    let v4062 = (v2919 * v2914) / (v83 * v3756);
                                                    v4433 = v4062;
                                                }
                                                v4432 = v4433;
                                            } else {
                                                let v4067 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v4434: f64;
                                                if v4067 != 0.0 {
                                                    let v4068 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                    if v4068 != 0.0 {
                                                    } else {
                                                    }
                                                    let v4070 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4068 != 0.0 { 1.0 } else { 0.0 };
                                                    let v4435: f64;
                                                    if v4070 != 0.0 {
                                                        v4435 = v0;
                                                    } else {
                                                        let v4074 = (v2919 * v83) / ((v2979 * v3756) * v2914);
                                                        v4435 = v4074;
                                                    }
                                                    v4434 = v4435;
                                                } else {
                                                    v4434 = v0;
                                                }
                                                v4432 = v4434;
                                            }
                                            v4427 = v4432;
                                        }
                                        v4426 = v4427;
                                    } else {
                                        let v4436: f64;
                                        if v4075 != 0.0 {
                                            let v4080 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v4437: f64;
                                            if v4080 != 0.0 {
                                                let v4081 = if v3806 == v0 { 1.0 } else { 0.0 };
                                                let v4438: f64;
                                                if v4081 != 0.0 {
                                                    v4438 = v0;
                                                } else {
                                                    let v4084 = (v2919 * v2914) / (v83 * v3806);
                                                    v4438 = v4084;
                                                }
                                                v4437 = v4438;
                                            } else {
                                                let v4089 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v4439: f64;
                                                if v4089 != 0.0 {
                                                    let v4090 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                    if v4090 != 0.0 {
                                                    } else {
                                                    }
                                                    let v4092 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4090 != 0.0 { 1.0 } else { 0.0 };
                                                    let v4440: f64;
                                                    if v4092 != 0.0 {
                                                        v4440 = v0;
                                                    } else {
                                                        let v4096 = (v2919 * v83) / ((v2979 * v3806) * v2914);
                                                        v4440 = v4096;
                                                    }
                                                    v4439 = v4440;
                                                } else {
                                                    v4439 = v0;
                                                }
                                                v4437 = v4439;
                                            }
                                            v4436 = v4437;
                                        } else {
                                            let v4101 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v4441: f64;
                                            if v4101 != 0.0 {
                                                let v4102 = if v3806 == v0 { 1.0 } else { 0.0 };
                                                let v4442: f64;
                                                if v4102 != 0.0 {
                                                    v4442 = v0;
                                                } else {
                                                    let v4105 = (v2919 * v2914) / (v83 * v3806);
                                                    v4442 = v4105;
                                                }
                                                v4441 = v4442;
                                            } else {
                                                let v4110 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v4443: f64;
                                                if v4110 != 0.0 {
                                                    let v4111 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                    if v4111 != 0.0 {
                                                    } else {
                                                    }
                                                    let v4113 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4111 != 0.0 { 1.0 } else { 0.0 };
                                                    let v4444: f64;
                                                    if v4113 != 0.0 {
                                                        v4444 = v0;
                                                    } else {
                                                        let v4117 = (v2919 * v83) / ((v2979 * v3806) * v2914);
                                                        v4444 = v4117;
                                                    }
                                                    v4443 = v4444;
                                                } else {
                                                    v4443 = v0;
                                                }
                                                v4441 = v4443;
                                            }
                                            v4436 = v4441;
                                        }
                                        v4426 = v4436;
                                    }
                                    v4352 = v4344;
                                    v4425 = v4426;
                                } else {
                                    let v4118 = if v2926 == v2976 { 1.0 } else { 0.0 };
                                    let v4353: f64;
                                    let v4445: f64;
                                    if v4118 != 0.0 {
                                        let v4446: f64;
                                        if v4119 != 0.0 {
                                            let v4447: f64;
                                            if v4120 != 0.0 {
                                                let v4125 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v4448: f64;
                                                if v4125 != 0.0 {
                                                    let v4126 = if v3756 == v0 { 1.0 } else { 0.0 };
                                                    let v4449: f64;
                                                    if v4126 != 0.0 {
                                                        v4449 = v0;
                                                    } else {
                                                        let v4129 = (v2919 * v2914) / (v83 * v3756);
                                                        v4449 = v4129;
                                                    }
                                                    v4448 = v4449;
                                                } else {
                                                    let v4134 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v4450: f64;
                                                    if v4134 != 0.0 {
                                                        let v4135 = v2914 + v2915;
                                                        let v4136 = if v4135 == v0 { 1.0 } else { 0.0 };
                                                        if v4136 != 0.0 {
                                                        } else {
                                                        }
                                                        let v4138 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4136 != 0.0 { 1.0 } else { 0.0 };
                                                        let v4451: f64;
                                                        if v4138 != 0.0 {
                                                            v4451 = v0;
                                                        } else {
                                                            let v4142 = (v2919 * v83) / ((v2974 * v3756) * v4135);
                                                            v4451 = v4142;
                                                        }
                                                        v4450 = v4451;
                                                    } else {
                                                        v4450 = v0;
                                                    }
                                                    v4448 = v4450;
                                                }
                                                v4447 = v4448;
                                            } else {
                                                let v4147 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v4452: f64;
                                                if v4147 != 0.0 {
                                                    let v4148 = if v3756 == v0 { 1.0 } else { 0.0 };
                                                    let v4453: f64;
                                                    if v4148 != 0.0 {
                                                        v4453 = v0;
                                                    } else {
                                                        let v4151 = (v2919 * v2914) / (v83 * v3756);
                                                        v4453 = v4151;
                                                    }
                                                    v4452 = v4453;
                                                } else {
                                                    let v4156 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v4454: f64;
                                                    if v4156 != 0.0 {
                                                        let v4157 = v2914 + v2915;
                                                        let v4158 = if v4157 == v0 { 1.0 } else { 0.0 };
                                                        if v4158 != 0.0 {
                                                        } else {
                                                        }
                                                        let v4160 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4158 != 0.0 { 1.0 } else { 0.0 };
                                                        let v4455: f64;
                                                        if v4160 != 0.0 {
                                                            v4455 = v0;
                                                        } else {
                                                            let v4164 = (v2919 * v83) / ((v2974 * v3756) * v4157);
                                                            v4455 = v4164;
                                                        }
                                                        v4454 = v4455;
                                                    } else {
                                                        v4454 = v0;
                                                    }
                                                    v4452 = v4454;
                                                }
                                                v4447 = v4452;
                                            }
                                            v4446 = v4447;
                                        } else {
                                            let v4166 = (v2919 * v2917) / v83;
                                            v4446 = v4166;
                                        }
                                        v4353 = v4344;
                                        v4445 = v4446;
                                    } else {
                                        let v4167 = if v2926 == v2964 { 1.0 } else { 0.0 };
                                        let v4354: f64;
                                        let v4456: f64;
                                        if v4167 != 0.0 {
                                            let v4457: f64;
                                            if v4168 != 0.0 {
                                                let v4458: f64;
                                                if v4169 != 0.0 {
                                                    let v4174 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v4459: f64;
                                                    if v4174 != 0.0 {
                                                        let v4175 = if v3756 == v0 { 1.0 } else { 0.0 };
                                                        let v4460: f64;
                                                        if v4175 != 0.0 {
                                                            v4460 = v0;
                                                        } else {
                                                            let v4178 = (v2919 * v2914) / (v83 * v3756);
                                                            v4460 = v4178;
                                                        }
                                                        v4459 = v4460;
                                                    } else {
                                                        let v4183 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v4461: f64;
                                                        if v4183 != 0.0 {
                                                            let v4184 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                            if v4184 != 0.0 {
                                                            } else {
                                                            }
                                                            let v4186 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4184 != 0.0 { 1.0 } else { 0.0 };
                                                            let v4462: f64;
                                                            if v4186 != 0.0 {
                                                                v4462 = v0;
                                                            } else {
                                                                let v4190 = (v2919 * v83) / ((v2979 * v3756) * v2914);
                                                                v4462 = v4190;
                                                            }
                                                            v4461 = v4462;
                                                        } else {
                                                            v4461 = v0;
                                                        }
                                                        v4459 = v4461;
                                                    }
                                                    v4458 = v4459;
                                                } else {
                                                    let v4195 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v4463: f64;
                                                    if v4195 != 0.0 {
                                                        let v4196 = if v3756 == v0 { 1.0 } else { 0.0 };
                                                        let v4464: f64;
                                                        if v4196 != 0.0 {
                                                            v4464 = v0;
                                                        } else {
                                                            let v4199 = (v2919 * v2914) / (v83 * v3756);
                                                            v4464 = v4199;
                                                        }
                                                        v4463 = v4464;
                                                    } else {
                                                        let v4204 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v4465: f64;
                                                        if v4204 != 0.0 {
                                                            let v4205 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                            if v4205 != 0.0 {
                                                            } else {
                                                            }
                                                            let v4207 = if (if v3756 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4205 != 0.0 { 1.0 } else { 0.0 };
                                                            let v4466: f64;
                                                            if v4207 != 0.0 {
                                                                v4466 = v0;
                                                            } else {
                                                                let v4211 = (v2919 * v83) / ((v2979 * v3756) * v2914);
                                                                v4466 = v4211;
                                                            }
                                                            v4465 = v4466;
                                                        } else {
                                                            v4465 = v0;
                                                        }
                                                        v4463 = v4465;
                                                    }
                                                    v4458 = v4463;
                                                }
                                                v4457 = v4458;
                                            } else {
                                                let v4212 = if v3806 == v0 { 1.0 } else { 0.0 };
                                                let v4467: f64;
                                                if v4212 != 0.0 {
                                                    v4467 = v0;
                                                } else {
                                                    let v4215 = (v2919 * v2917) / (v83 * v3806);
                                                    v4467 = v4215;
                                                }
                                                v4457 = v4467;
                                            }
                                            v4354 = v4344;
                                            v4456 = v4457;
                                        } else {
                                            let v4216 = if v2926 == v2979 { 1.0 } else { 0.0 };
                                            let v4355: f64;
                                            let v4468: f64;
                                            if v4216 != 0.0 {
                                                let v4469: f64;
                                                if v4217 != 0.0 {
                                                    let v4219 = (v2919 * v2917) / v83;
                                                    v4469 = v4219;
                                                } else {
                                                    let v4470: f64;
                                                    if v4220 != 0.0 {
                                                        let v4225 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v4471: f64;
                                                        if v4225 != 0.0 {
                                                            let v4226 = if v3806 == v0 { 1.0 } else { 0.0 };
                                                            let v4472: f64;
                                                            if v4226 != 0.0 {
                                                                v4472 = v0;
                                                            } else {
                                                                let v4229 = (v2919 * v2914) / (v83 * v3806);
                                                                v4472 = v4229;
                                                            }
                                                            v4471 = v4472;
                                                        } else {
                                                            let v4234 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v4473: f64;
                                                            if v4234 != 0.0 {
                                                                let v4235 = v2914 + v2915;
                                                                let v4236 = if v4235 == v0 { 1.0 } else { 0.0 };
                                                                if v4236 != 0.0 {
                                                                } else {
                                                                }
                                                                let v4238 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4236 != 0.0 { 1.0 } else { 0.0 };
                                                                let v4474: f64;
                                                                if v4238 != 0.0 {
                                                                    v4474 = v0;
                                                                } else {
                                                                    let v4242 = (v2919 * v83) / ((v2974 * v3806) * v4235);
                                                                    v4474 = v4242;
                                                                }
                                                                v4473 = v4474;
                                                            } else {
                                                                v4473 = v0;
                                                            }
                                                            v4471 = v4473;
                                                        }
                                                        v4470 = v4471;
                                                    } else {
                                                        let v4247 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v4475: f64;
                                                        if v4247 != 0.0 {
                                                            let v4248 = if v3806 == v0 { 1.0 } else { 0.0 };
                                                            let v4476: f64;
                                                            if v4248 != 0.0 {
                                                                v4476 = v0;
                                                            } else {
                                                                let v4251 = (v2919 * v2914) / (v83 * v3806);
                                                                v4476 = v4251;
                                                            }
                                                            v4475 = v4476;
                                                        } else {
                                                            let v4256 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v4477: f64;
                                                            if v4256 != 0.0 {
                                                                let v4257 = v2914 + v2915;
                                                                let v4258 = if v4257 == v0 { 1.0 } else { 0.0 };
                                                                if v4258 != 0.0 {
                                                                } else {
                                                                }
                                                                let v4260 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4258 != 0.0 { 1.0 } else { 0.0 };
                                                                let v4478: f64;
                                                                if v4260 != 0.0 {
                                                                    v4478 = v0;
                                                                } else {
                                                                    let v4264 = (v2919 * v83) / ((v2974 * v3806) * v4257);
                                                                    v4478 = v4264;
                                                                }
                                                                v4477 = v4478;
                                                            } else {
                                                                v4477 = v0;
                                                            }
                                                            v4475 = v4477;
                                                        }
                                                        v4470 = v4475;
                                                    }
                                                    v4469 = v4470;
                                                }
                                                v4355 = v4344;
                                                v4468 = v4469;
                                            } else {
                                                let v4265 = if v2926 == v2993 { 1.0 } else { 0.0 };
                                                let v4356: f64;
                                                let v4479: f64;
                                                if v4265 != 0.0 {
                                                    let v4480: f64;
                                                    if v4266 != 0.0 {
                                                        let v4267 = if v3756 == v0 { 1.0 } else { 0.0 };
                                                        let v4481: f64;
                                                        if v4267 != 0.0 {
                                                            v4481 = v0;
                                                        } else {
                                                            let v4270 = (v2919 * v2917) / (v83 * v3756);
                                                            v4481 = v4270;
                                                        }
                                                        v4480 = v4481;
                                                    } else {
                                                        let v4482: f64;
                                                        if v4271 != 0.0 {
                                                            let v4276 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2964 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v4483: f64;
                                                            if v4276 != 0.0 {
                                                                let v4277 = if v3806 == v0 { 1.0 } else { 0.0 };
                                                                let v4484: f64;
                                                                if v4277 != 0.0 {
                                                                    v4484 = v0;
                                                                } else {
                                                                    let v4280 = (v2919 * v2914) / (v83 * v3806);
                                                                    v4484 = v4280;
                                                                }
                                                                v4483 = v4484;
                                                            } else {
                                                                let v4285 = if (if (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2979 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let v4485: f64;
                                                                if v4285 != 0.0 {
                                                                    let v4286 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                                    if v4286 != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let v4288 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4286 != 0.0 { 1.0 } else { 0.0 };
                                                                    let v4486: f64;
                                                                    if v4288 != 0.0 {
                                                                        v4486 = v0;
                                                                    } else {
                                                                        let v4292 = (v2919 * v83) / ((v2979 * v3806) * v2914);
                                                                        v4486 = v4292;
                                                                    }
                                                                    v4485 = v4486;
                                                                } else {
                                                                    v4485 = v0;
                                                                }
                                                                v4483 = v4485;
                                                            }
                                                            v4482 = v4483;
                                                        } else {
                                                            let v4297 = if (if (if v2922 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2974 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2993 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v4487: f64;
                                                            if v4297 != 0.0 {
                                                                let v4298 = if v3806 == v0 { 1.0 } else { 0.0 };
                                                                let v4488: f64;
                                                                if v4298 != 0.0 {
                                                                    v4488 = v0;
                                                                } else {
                                                                    let v4301 = (v2919 * v2914) / (v83 * v3806);
                                                                    v4488 = v4301;
                                                                }
                                                                v4487 = v4488;
                                                            } else {
                                                                let v4306 = if (if (if v2922 == v71 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v2976 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2922 == v3003 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let v4489: f64;
                                                                if v4306 != 0.0 {
                                                                    let v4307 = if v2914 == v0 { 1.0 } else { 0.0 };
                                                                    if v4307 != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let v4309 = if (if v3806 == v0 { 1.0 } else { 0.0 }) != 0.0 || v4307 != 0.0 { 1.0 } else { 0.0 };
                                                                    let v4490: f64;
                                                                    if v4309 != 0.0 {
                                                                        v4490 = v0;
                                                                    } else {
                                                                        let v4313 = (v2919 * v83) / ((v2979 * v3806) * v2914);
                                                                        v4490 = v4313;
                                                                    }
                                                                    v4489 = v4490;
                                                                } else {
                                                                    v4489 = v0;
                                                                }
                                                                v4487 = v4489;
                                                            }
                                                            v4482 = v4487;
                                                        }
                                                        v4480 = v4482;
                                                    }
                                                    v4356 = v4344;
                                                    v4479 = v4480;
                                                } else {
                                                    let v4314 = if v2926 == v3003 { 1.0 } else { 0.0 };
                                                    let v4357: f64;
                                                    let v4491: f64;
                                                    if v4314 != 0.0 {
                                                        let v4316 = (v2919 * v2917) / v83;
                                                        v4357 = v4344;
                                                        v4491 = v4316;
                                                    } else {
                                                        let v4317 = if v2926 == v2927 { 1.0 } else { 0.0 };
                                                        let v4358: f64;
                                                        let v4492: f64;
                                                        if v4317 != 0.0 {
                                                            let v4359: f64;
                                                            let v4493: f64;
                                                            if v4318 != 0.0 {
                                                                let v4321 = ((v2485 * v2919) * v2914) / v83;
                                                                let v4322 = if v32 == v71 { 1.0 } else { 0.0 };
                                                                let v4360: f64;
                                                                if v4322 != 0.0 {
                                                                    v4360 = v0;
                                                                } else {
                                                                    let v4326 = (v2919 * v2914) / (v83 * (v32 - v71));
                                                                    v4360 = v4326;
                                                                }
                                                                v4359 = v4360;
                                                                v4493 = v4321;
                                                            } else {
                                                                let v4329 = (v2919 * v2914) / (v83 * v32);
                                                                v4359 = v4329;
                                                                v4493 = v0;
                                                            }
                                                            v4358 = v4359;
                                                            v4492 = v4493;
                                                        } else {
                                                            let v4330 = if v2926 == v2908 { 1.0 } else { 0.0 };
                                                            let v4361: f64;
                                                            let v4494: f64;
                                                            if v4330 != 0.0 {
                                                                let v4362: f64;
                                                                let v4495: f64;
                                                                if v4331 != 0.0 {
                                                                    let v4334 = (v2919 * v2914) / (v83 * v32);
                                                                    v4362 = v4334;
                                                                    v4495 = v0;
                                                                } else {
                                                                    let v4337 = ((v2485 * v2919) * v2914) / v83;
                                                                    let v4338 = if v32 == v71 { 1.0 } else { 0.0 };
                                                                    let v4363: f64;
                                                                    if v4338 != 0.0 {
                                                                        v4363 = v0;
                                                                    } else {
                                                                        let v4342 = (v2919 * v2914) / (v83 * (v32 - v71));
                                                                        v4363 = v4342;
                                                                    }
                                                                    v4362 = v4363;
                                                                    v4495 = v4337;
                                                                }
                                                                v4361 = v4362;
                                                                v4494 = v4495;
                                                            } else {
                                                                v4361 = v0;
                                                                v4494 = v4496;
                                                            }
                                                            v4358 = v4361;
                                                            v4492 = v4494;
                                                        }
                                                        v4357 = v4358;
                                                        v4491 = v4492;
                                                    }
                                                    v4356 = v4357;
                                                    v4479 = v4491;
                                                }
                                                v4355 = v4356;
                                                v4468 = v4479;
                                            }
                                            v4354 = v4355;
                                            v4456 = v4468;
                                        }
                                        v4353 = v4354;
                                        v4445 = v4456;
                                    }
                                    v4352 = v4353;
                                    v4425 = v4445;
                                }
                                v4351 = v4352;
                                v4405 = v4425;
                            }
                            v4350 = v4351;
                            v4385 = v4405;
                        }
                        v4343 = v4350;
                        v4365 = v4385;
                    }
                    let v4364 = if v4343 <= v0 { 1.0 } else { 0.0 };
                    let v4502: f64;
                    if v4364 != 0.0 {
                        v4502 = v4365;
                    } else {
                        let v4498 = if v4365 <= v0 { 1.0 } else { 0.0 };
                        let v4503: f64;
                        if v4498 != 0.0 {
                            v4503 = v4343;
                        } else {
                            let v4501 = (v4343 * v4365) / (v4343 + v4365);
                            v4503 = v4501;
                        }
                        v4502 = v4503;
                    }
                    let v4504 = if v4502 == v0 { 1.0 } else { 0.0 };
                    if v4504 != 0.0 {
                    } else {
                    }
                    v4511 = v4502;
                    v5071 = v3756;
                    v5080 = v5081;
                    v5088 = v3806;
                    v5097 = v5098;
                } else {
                    v4511 = v0;
                    v5071 = v3759;
                    v5080 = v5077;
                    v5088 = v3809;
                    v5097 = v5094;
                }
                v4510 = v4511;
                v5070 = v5071;
                v5076 = v5080;
                v5087 = v5088;
                v5093 = v5097;
            }
            let v4505 = if v2833 == v0 { 1.0 } else { 0.0 };
            let v7135: f64;
            let v7138: f64;
            if v4505 != 0.0 {
                let v4509 = if v4506 < v4508 { 1.0 } else { 0.0 };
                let v7139: f64;
                if v4509 != 0.0 {
                    v7139 = v0;
                } else {
                    v7139 = v4506;
                }
                let v4512 = if v4510 < v4508 { 1.0 } else { 0.0 };
                let v7136: f64;
                if v4512 != 0.0 {
                    v7136 = v0;
                } else {
                    v7136 = v4510;
                }
                v7135 = v7136;
                v7138 = v7139;
            } else {
                let v4513 = if v4506 <= v4508 { 1.0 } else { 0.0 };
                let v7140: f64;
                if v4513 != 0.0 {
                    v7140 = v4508;
                } else {
                    v7140 = v4506;
                }
                let v4514 = if v4510 <= v4508 { 1.0 } else { 0.0 };
                let v7137: f64;
                if v4514 != 0.0 {
                    v7137 = v4508;
                } else {
                    v7137 = v4510;
                }
                v7135 = v7137;
                v7138 = v7140;
            }
            let v7221: f64;
            let v7223: f64;
            let v8910: f64;
            let v8912: f64;
            let v8935: f64;
            let v8937: f64;
            if v2834 != 0.0 {
                let v4515 = if v779 <= v0 { 1.0 } else { 0.0 };
                let v8911: f64;
                if v4515 != 0.0 {
                    v8911 = v0;
                } else {
                    v8911 = v779;
                }
                let v4516 = if v789 <= v0 { 1.0 } else { 0.0 };
                let v8936: f64;
                if v4516 != 0.0 {
                    v8936 = v0;
                } else {
                    v8936 = v789;
                }
                let v4518 = if v4517 <= v0 { 1.0 } else { 0.0 };
                let v8913: f64;
                if v4518 != 0.0 {
                    v8913 = v0;
                } else {
                    v8913 = v4517;
                }
                let v4520 = if v4519 <= v0 { 1.0 } else { 0.0 };
                let v8938: f64;
                if v4520 != 0.0 {
                    v8938 = v0;
                } else {
                    v8938 = v4519;
                }
                v7221 = v809;
                v7223 = v4522;
                v8910 = v8911;
                v8912 = v8913;
                v8935 = v8936;
                v8937 = v8938;
            } else {
                let v4521 = if v809 <= v0 { 1.0 } else { 0.0 };
                let v7222: f64;
                if v4521 != 0.0 {
                    v7222 = v0;
                } else {
                    v7222 = v809;
                }
                let v4523 = if v4522 <= v0 { 1.0 } else { 0.0 };
                let v7224: f64;
                if v4523 != 0.0 {
                    v7224 = v0;
                } else {
                    v7224 = v4522;
                }
                v7221 = v7222;
                v7223 = v7224;
                v8910 = v779;
                v8912 = v4517;
                v8935 = v789;
                v8937 = v4519;
            }
            let v4535 = (v4524 * (v4525 + ((v125 / v2974) / v4527))) / ((v4527 * v32) * (v30 - v4532));
            let v4536 = if v4535 > v0 { 1.0 } else { 0.0 };
            let v9086: f64;
            if v4536 != 0.0 {
                let v4537 = v1 / v4535;
                v9086 = v4537;
            } else {
                let v4540 = if v4539 != v0 { 1.0 } else { 0.0 };
                if v4540 != 0.0 {
                } else {
                }
                v9086 = v4538;
            }
            let v4542 = v12 * v1719;
            let v4551 = (rspice_limited_exp((v1749 * ((if (v4544 / v12) >= v4546 { (v4544 / v12) } else { v4546 }).ln())))) / (v12 * v12);
            let v4557 = (rspice_limited_exp((v1749 * ((if (v4544 / v4542) >= v4546 { (v4544 / v4542) } else { v4546 }).ln())))) / (v4542 * v4542);
            let v4560: f64;
            if v5 != 0.0 {
                v4560 = v4558;
            } else {
                v4560 = v4559;
            }
            let v4563: f64;
            if v5 != 0.0 {
                v4563 = v4561;
            } else {
                v4563 = v4562;
            }
            let v4565 = v83 / v4564;
            let v4569 = (v4560 * (v4565 + v4566)) * v4557;
            let v4573 = (v4560 * (v4565 + v4570)) * v4557;
            let v4575 = (-v4563) * v12;
            let v4576 = v4575 * v1719;
            let v4579 = v4578 / v32;
            let v4582 = (v4560 * ((v4565 * v73) + v4579)) * v4551;
            let v4589 = if (if v4585 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v4587 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v4591 = if v4589 != 0.0 && (if (v4583 + v83) > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v4591 != 0.0 {
                let v4594 = if v4592 != 0.0 && v4593 != 0.0 { 1.0 } else { 0.0 };
                if v4594 != 0.0 {
                    if v1 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            if v4591 != 0.0 {
            } else {
            }
            let v4598 = if v4595 <= v4597 { 1.0 } else { 0.0 };
            let v4615: f64;
            if v4598 != 0.0 {
                v4615 = v4599;
            } else {
                let v4600 = v4595 + v4596;
                v4615 = v4600;
            }
            let v4603 = v4601 + v4602;
            let v4609: f64;
            if v4589 != 0.0 {
                let v4606 = if (if v4592 != v0 { 1.0 } else { 0.0 }) != 0.0 && v4605 != 0.0 { 1.0 } else { 0.0 };
                let v4610: f64;
                if v4606 != 0.0 {
                    let v4611: f64;
                    if v1 != 0.0 {
                        v4611 = v4607;
                    } else {
                        v4611 = v4608;
                    }
                    v4610 = v4611;
                } else {
                    v4610 = v4608;
                }
                v4609 = v4610;
            } else {
                v4609 = v0;
            }
            let v4612 = v4609 + v4603;
            let v4613 = v3 * v4612;
            let v4614 = v1 / v4613;
            let v4616 = v4612 / v4615;
            let v4617 = v4612 - v4615;
            let v4626 = v4619 - (((v4620 * v4612) * v4612) / (v4612 + v4623));
            let v4633 = v71 * v4613;
            let v4637 = (v4629 * (v4616 * (v4616.sqrt()))) * (rspice_limited_exp(((v4626 / (v71 * (v3 * v4615))) - (v4626 / v4633))));
            let v4691: f64;
            if v4591 != 0.0 {
                let v4640 = (if (v2291 / v4637) >= v4546 { (v2291 / v4637) } else { v4546 }).ln();
                let v4643 = ((v4640 * v4640) + v127).sqrt();
                v4691 = v4643;
            } else {
                let v4646 = (if (v2291 / v4637) >= v4546 { (v2291 / v4637) } else { v4546 }).ln();
                v4691 = v4646;
            }
            let v13181: f64;
            if v4591 != 0.0 {
                let v4651 = (if ((v1879 * v229) / (v4637 * v4637)) >= v4546 { ((v1879 * v229) / (v4637 * v4637)) } else { v4546 }).ln();
                let v4654 = ((v4651 * v4651) + v127).sqrt();
                v13181 = v4654;
            } else {
                let v4659 = (if ((v1879 * v229) / (v4637 * v4637)) >= v4546 { ((v1879 * v229) / (v4637 * v4637)) } else { v4546 }).ln();
                v13181 = v4659;
            }
            let v4660 = if v259 > v0 { 1.0 } else { 0.0 };
            let v8893: f64;
            if v4660 != 0.0 {
                let v4669 = (((-v4661) * v4613) * ((if (v259 / v229) >= v4546 { (v259 / v229) } else { v4546 }).ln())) + v4668;
                v8893 = v4669;
            } else {
                v8893 = v0;
            }
            if v4670 != 0.0 {
                let v4672 = if v4671 != v0 { 1.0 } else { 0.0 };
                if v4672 != 0.0 {
                } else {
                }
                let v4674 = if v4673 != v0 { 1.0 } else { 0.0 };
                if v4674 != 0.0 {
                } else {
                }
            } else {
                let v4676 = if v4592 != 0.0 && v4675 != 0.0 { 1.0 } else { 0.0 };
                if v4676 != 0.0 {
                    let v4677 = if v4671 != v0 { 1.0 } else { 0.0 };
                    if v4677 != 0.0 {
                    } else {
                    }
                    let v4678 = if v4673 != v0 { 1.0 } else { 0.0 };
                    if v4678 != 0.0 {
                    } else {
                    }
                } else {
                    let v4679 = if v4671 == v0 { 1.0 } else { 0.0 };
                    if v4679 != 0.0 {
                        let v4680 = if v4673 != v0 { 1.0 } else { 0.0 };
                        if v4680 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                }
            }
            let v4681 = if v4671 != v0 { 1.0 } else { 0.0 };
            let v4684 = if v4592 == v1 { 1.0 } else { 0.0 };
            let v4688 = if (if v4681 != 0.0 && v4682 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v4684 != 0.0 && v4685 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v4688 != 0.0 {
                let v4689 = if v4673 == v0 { 1.0 } else { 0.0 };
                if v4689 != 0.0 {
                } else {
                }
            } else {
            }
            let v4695 = if ((v4690 + (v4613 * v4691)) + v609) >= v4690 { ((v4690 + (v4613 * v4691)) + v609) } else { v4690 };
            let v4696 = v4695.sqrt();
            let v4697 = v71 * v9;
            let v4698 = v2 * v2291;
            let v4700 = (v4697 / v4698).sqrt();
            let v4704 = (((v9 / v11) * v12) * v599).sqrt();
            let v4706 = v4616 - v1;
            let v4708 = v1 + (v4705 * v4706);
            let v4716 = v2313 * (v2485 * (v4708 + (((v4708 * v4708) + v4711).sqrt())));
            let v4719 = v1 + (v4717 * v4706);
            let v4720 = v2456 * v4719;
            let v5566: f64;
            if v2131 != 0.0 {
                let v4722 = v4721 * v4719;
                v5566 = v4722;
            } else {
                v5566 = v0;
            }
            let v4723 = if v4 != v1 { 1.0 } else { 0.0 };
            let v4728: f64;
            if v4723 != 0.0 {
                let v4726 = v4724 * v4725;
                v4728 = v4726;
            } else {
                let v4727 = v2485 * v4725;
                v4728 = v4727;
            }
            let v4730 = v4616.powf(v2816);
            let v4731 = v4729 * v4730;
            let v4735 = (v1 + (v2820 * v4617)) - v127;
            let v4736 = v4735 * v4735;
            let v4742 = v4732 * (v2485 * (v4735 + ((v4736 + v4737).sqrt())));
            let v4745 = (v1 + (v1439 * v4617)) - v127;
            let v4746 = v4745 * v4745;
            let v4752 = v2448 * (v2485 * (v4745 + ((v4746 + v4747).sqrt())));
            let v4754 = v4616.powf(v2824);
            let v4755 = v4753 * v4754;
            let v4757 = v4616.powf(v1469);
            let v4758 = v4756 * v4757;
            let v4761 = v1 + (v1459 * v4706);
            let v4768 = v4759 * (v2485 * (v4761 + (((v4761 * v4761) + v4763).sqrt())));
            let v5593: f64;
            let v5597: f64;
            let v5601: f64;
            let v5605: f64;
            let v5609: f64;
            if v2131 != 0.0 {
                let v4774 = v4769 * v4730;
                let v4781 = v4775 * (v2485 * (v4735 + ((v4736 + v4776).sqrt())));
                let v4788 = v4782 * (v2485 * (v4745 + ((v4746 + v4783).sqrt())));
                let v4790 = v4789 * v4754;
                let v4794 = v4791 * v4757;
                v5593 = v4774;
                v5597 = v4781;
                v5601 = v4788;
                v5605 = v4790;
                v5609 = v4794;
            } else {
                v5593 = v0;
                v5597 = v0;
                v5601 = v0;
                v5605 = v0;
                v5609 = v0;
            }
            let v4795 = v4616.powf(v1479);
            let v4797 = v4616.powf((-v2828));
            let v4798 = v2530 * v4797;
            let v4800 = if v4798 < v4799 { 1.0 } else { 0.0 };
            let v5385: f64;
            if v4800 != 0.0 {
                v5385 = v4799;
            } else {
                v5385 = v4798;
            }
            let v5583: f64;
            if v2131 != 0.0 {
                let v4802 = v4801 * v4797;
                let v4803 = if v4802 < v4799 { 1.0 } else { 0.0 };
                let v5584: f64;
                if v4803 != 0.0 {
                    v5584 = v4799;
                } else {
                    v5584 = v4802;
                }
                v5583 = v5584;
            } else {
                v5583 = v0;
            }
            let v4804 = v2705 * v4797;
            let v4805 = if v4804 < v4799 { 1.0 } else { 0.0 };
            let v18465: f64;
            if v4805 != 0.0 {
                v18465 = v4799;
            } else {
                v18465 = v4804;
            }
            let v4811 = ((v1 / v2486) * (v1 + (v4807 * v4617))) - v71;
            let v4819 = v1 / ((v2485 * (v4811 + (((v4811 * v4811) + v4813).sqrt()))) + v71);
            let v4822 = (v1 - (v2832 * v4617)) - v127;
            let v4823 = v4822 * v4822;
            let v4829 = v2555 * (v2485 * (v4822 + ((v4823 + v4824).sqrt())));
            let v5589: f64;
            if v2131 != 0.0 {
                let v4836 = v4830 * (v2485 * (v4822 + ((v4823 + v4831).sqrt())));
                v5589 = v4836;
            } else {
                v5589 = v0;
            }
            let v4839 = (v1 + (v1799 * v4617)) - v127;
            let v4846 = v1789 * (v2485 * (v4839 + (((v4839 * v4839) + v4841).sqrt())));
            let v4849 = (v1 + (v1819 * v4617)) - v127;
            let v4856 = v1809 * (v2485 * (v4849 + (((v4849 * v4849) + v4851).sqrt())));
            let v4858 = v999 * (v4616.powf(v1509));
            let v4860 = v1329 + (v1339 * v4706);
            let v4862 = v1369 + (v1379 * v4706);
            let v4866 = rspice_limited_exp((v1519 * ((if v4616 >= v4546 { v4616 } else { v4546 }).ln())));
            let v4869 = (v1 + (v1849 * v4617)) - v127;
            let v4876 = v1829 * (v2485 * (v4869 + (((v4869 * v4869) + v4871).sqrt())));
            let v4880 = (v1 + (v1859 * v4617)) - v127;
            let v4887 = v4877 * (v2485 * (v4880 + (((v4880 * v4880) + v4882).sqrt())));
            let v4890 = (v1 + (v2069 * v4617)) - v127;
            let v4897 = v2059 * (v2485 * (v4890 + (((v4890 * v4890) + v4892).sqrt())));
            let v4900 = (v1 + (v2089 * v4617)) - v127;
            let v4907 = v2079 * (v2485 * (v4900 + (((v4900 * v4900) + v4902).sqrt())));
            let v4910 = (v1 + (v2109 * v4617)) - v127;
            let v4917 = v2099 * (v2485 * (v4910 + (((v4910 * v4910) + v4912).sqrt())));
            let v4922 = (v1 + (v4919 * v4617)) - v127;
            let v4923 = v4922 * v4922;
            let v4929 = v4918 * (v2485 * (v4922 + ((v4923 + v4924).sqrt())));
            let v4936 = v4930 * (v2485 * (v4922 + ((v4923 + v4931).sqrt())));
            let v4941 = (v1 + (v4938 * v4617)) - v127;
            let v4942 = v4941 * v4941;
            let v4948 = v4937 * (v2485 * (v4941 + ((v4942 + v4943).sqrt())));
            let v4955 = v4949 * (v2485 * (v4941 + ((v4942 + v4950).sqrt())));
            let v4960 = (v1 + (v4957 * v4617)) - v127;
            let v4961 = v4960 * v4960;
            let v4967 = v4956 * (v2485 * (v4960 + ((v4961 + v4962).sqrt())));
            let v4974 = v4968 * (v2485 * (v4960 + ((v4961 + v4969).sqrt())));
            let v4977 = v4976 * v4617;
            let v4980 = (v4975 - v4977) - v4979;
            let v4987 = (v2485 * (v4980 + (((v4980 * v4980) + v4982).sqrt()))) + v4979;
            let v4990 = (v4988 - v4977) - v4979;
            let v4997 = (v2485 * (v4990 + (((v4990 * v4990) + v4992).sqrt()))) + v4979;
            let v5000 = v4999 * v4617;
            let v5002 = (v4998 - v5000) - v4979;
            let v5009 = (v2485 * (v5002 + (((v5002 * v5002) + v5004).sqrt()))) + v4979;
            let v5012 = (v5010 - v5000) - v4979;
            let v5019 = (v2485 * (v5012 + (((v5012 * v5012) + v5014).sqrt()))) + v4979;
            let v5022 = v5021 * v4617;
            let v5024 = (v5020 - v5022) - v4979;
            let v5031 = (v2485 * (v5024 + (((v5024 * v5024) + v5026).sqrt()))) + v4979;
            let v5034 = (v5032 - v5022) - v4979;
            let v5041 = (v2485 * (v5034 + (((v5034 * v5034) + v5036).sqrt()))) + v4979;
            let v5042 = if v2926 < v2927 { 1.0 } else { 0.0 };
            let v5067: f64;
            let v5073: f64;
            let v5084: f64;
            let v5090: f64;
            if v5042 != 0.0 {
                let v5044 = if (v32 % v71) != v0 { 1.0 } else { 0.0 };
                let v5068: f64;
                let v5074: f64;
                let v5085: f64;
                let v5091: f64;
                if v5044 != 0.0 {
                    let v5048 = v71 * (if ((v32 - v1) / v71) >= v0 { ((v32 - v1) / v71) } else { v0 });
                    v5068 = v1;
                    v5074 = v5048;
                    v5085 = v1;
                    v5091 = v5048;
                } else {
                    let v5049 = if v2935 == v1 { 1.0 } else { 0.0 };
                    let v5069: f64;
                    let v5075: f64;
                    let v5086: f64;
                    let v5092: f64;
                    if v5049 != 0.0 {
                        let v5053 = v71 * (if ((v32 / v71) - v1) >= v0 { ((v32 / v71) - v1) } else { v0 });
                        v5069 = v0;
                        v5075 = v32;
                        v5086 = v71;
                        v5092 = v5053;
                    } else {
                        let v5057 = v71 * (if ((v32 / v71) - v1) >= v0 { ((v32 / v71) - v1) } else { v0 });
                        v5069 = v71;
                        v5075 = v5057;
                        v5086 = v0;
                        v5092 = v32;
                    }
                    v5068 = v5069;
                    v5074 = v5075;
                    v5085 = v5086;
                    v5091 = v5092;
                }
                v5067 = v5068;
                v5073 = v5074;
                v5084 = v5085;
                v5090 = v5091;
            } else {
                v5067 = v5070;
                v5073 = v5076;
                v5084 = v5087;
                v5090 = v5093;
            }
            let v5058 = v2914 + v2915;
            let v5059 = v2914 + v2914;
            let v5060 = v2917 + v2917;
            let v5062 = (v5058 + v5058) + v125;
            let v5063 = v5058 * v125;
            let v5064 = v2914 * v125;
            let v5065 = v2917 * v125;
            let v5066 = if v2926 == v0 { 1.0 } else { 0.0 };
            let v5213: f64;
            let v5230: f64;
            let v5252: f64;
            let v5272: f64;
            if v5066 != 0.0 {
                let v5083 = (v5067 * v5062) + (v5073 * v5059);
                let v5100 = (v5084 * v5062) + (v5090 * v5059);
                let v5103 = (v5067 * v5063) + (v5073 * v5064);
                let v5106 = (v5084 * v5063) + (v5090 * v5064);
                v5213 = v5103;
                v5230 = v5106;
                v5252 = v5083;
                v5272 = v5100;
            } else {
                let v5107 = if v2926 == v1 { 1.0 } else { 0.0 };
                let v5214: f64;
                let v5231: f64;
                let v5253: f64;
                let v5273: f64;
                if v5107 != 0.0 {
                    let v5110 = (v5067 * v5062) + (v5073 * v5059);
                    let v5111 = v5084 + v5090;
                    let v5112 = v5111 * v5059;
                    let v5115 = (v5067 * v5063) + (v5073 * v5064);
                    let v5116 = v5111 * v5064;
                    v5214 = v5115;
                    v5231 = v5116;
                    v5253 = v5110;
                    v5273 = v5112;
                } else {
                    let v5117 = if v2926 == v71 { 1.0 } else { 0.0 };
                    let v5215: f64;
                    let v5232: f64;
                    let v5254: f64;
                    let v5274: f64;
                    if v5117 != 0.0 {
                        let v5118 = v5067 + v5073;
                        let v5119 = v5118 * v5059;
                        let v5122 = (v5084 * v5062) + (v5090 * v5059);
                        let v5123 = v5118 * v5064;
                        let v5126 = (v5084 * v5063) + (v5090 * v5064);
                        v5215 = v5123;
                        v5232 = v5126;
                        v5254 = v5119;
                        v5274 = v5122;
                    } else {
                        let v5127 = if v2926 == v2974 { 1.0 } else { 0.0 };
                        let v5216: f64;
                        let v5233: f64;
                        let v5255: f64;
                        let v5275: f64;
                        if v5127 != 0.0 {
                            let v5128 = v5067 + v5073;
                            let v5129 = v5128 * v5059;
                            let v5130 = v5084 + v5090;
                            let v5131 = v5130 * v5059;
                            let v5132 = v5128 * v5064;
                            let v5133 = v5130 * v5064;
                            v5216 = v5132;
                            v5233 = v5133;
                            v5255 = v5129;
                            v5275 = v5131;
                        } else {
                            let v5134 = if v2926 == v2976 { 1.0 } else { 0.0 };
                            let v5217: f64;
                            let v5234: f64;
                            let v5256: f64;
                            let v5276: f64;
                            if v5134 != 0.0 {
                                let v5137 = (v5067 * v5062) + (v5073 * v5059);
                                let v5140 = (v5084 * v5060) + (v5090 * v5059);
                                let v5143 = (v5067 * v5063) + (v5073 * v5064);
                                let v5146 = (v5084 * v5065) + (v5090 * v5064);
                                v5217 = v5143;
                                v5234 = v5146;
                                v5256 = v5137;
                                v5276 = v5140;
                            } else {
                                let v5147 = if v2926 == v2964 { 1.0 } else { 0.0 };
                                let v5218: f64;
                                let v5235: f64;
                                let v5257: f64;
                                let v5277: f64;
                                if v5147 != 0.0 {
                                    let v5148 = v5067 + v5073;
                                    let v5149 = v5148 * v5059;
                                    let v5152 = (v5084 * v5060) + (v5090 * v5059);
                                    let v5153 = v5148 * v5064;
                                    let v5156 = (v5084 * v5065) + (v5090 * v5064);
                                    v5218 = v5153;
                                    v5235 = v5156;
                                    v5257 = v5149;
                                    v5277 = v5152;
                                } else {
                                    let v5157 = if v2926 == v2979 { 1.0 } else { 0.0 };
                                    let v5219: f64;
                                    let v5236: f64;
                                    let v5258: f64;
                                    let v5278: f64;
                                    if v5157 != 0.0 {
                                        let v5160 = (v5067 * v5060) + (v5073 * v5059);
                                        let v5163 = (v5084 * v5062) + (v5090 * v5059);
                                        let v5166 = (v5067 * v5065) + (v5073 * v5064);
                                        let v5169 = (v5084 * v5063) + (v5090 * v5064);
                                        v5219 = v5166;
                                        v5236 = v5169;
                                        v5258 = v5160;
                                        v5278 = v5163;
                                    } else {
                                        let v5170 = if v2926 == v2993 { 1.0 } else { 0.0 };
                                        let v5220: f64;
                                        let v5237: f64;
                                        let v5259: f64;
                                        let v5279: f64;
                                        if v5170 != 0.0 {
                                            let v5173 = (v5067 * v5060) + (v5073 * v5059);
                                            let v5174 = v5084 + v5090;
                                            let v5175 = v5174 * v5059;
                                            let v5178 = (v5067 * v5065) + (v5073 * v5064);
                                            let v5179 = v5174 * v5064;
                                            v5220 = v5178;
                                            v5237 = v5179;
                                            v5259 = v5173;
                                            v5279 = v5175;
                                        } else {
                                            let v5180 = if v2926 == v3003 { 1.0 } else { 0.0 };
                                            let v5221: f64;
                                            let v5238: f64;
                                            let v5260: f64;
                                            let v5280: f64;
                                            if v5180 != 0.0 {
                                                let v5183 = (v5067 * v5060) + (v5073 * v5059);
                                                let v5186 = (v5084 * v5060) + (v5090 * v5059);
                                                let v5189 = (v5067 * v5065) + (v5073 * v5064);
                                                let v5192 = (v5084 * v5065) + (v5090 * v5064);
                                                v5221 = v5189;
                                                v5238 = v5192;
                                                v5260 = v5183;
                                                v5280 = v5186;
                                            } else {
                                                let v5193 = if v2926 == v2927 { 1.0 } else { 0.0 };
                                                let v5222: f64;
                                                let v5239: f64;
                                                let v5261: f64;
                                                let v5281: f64;
                                                if v5193 != 0.0 {
                                                    let v5194 = v32 - v1;
                                                    let v5196 = v5062 + (v5194 * v5059);
                                                    let v5197 = v32 * v5059;
                                                    let v5199 = v5063 + (v5194 * v5064);
                                                    let v5200 = v32 * v5064;
                                                    v5222 = v5199;
                                                    v5239 = v5200;
                                                    v5261 = v5196;
                                                    v5281 = v5197;
                                                } else {
                                                    let v5201 = if v2926 == v2908 { 1.0 } else { 0.0 };
                                                    let v5223: f64;
                                                    let v5240: f64;
                                                    let v5262: f64;
                                                    let v5282: f64;
                                                    if v5201 != 0.0 {
                                                        let v5202 = v32 * v5059;
                                                        let v5203 = v32 - v1;
                                                        let v5205 = v5062 + (v5203 * v5059);
                                                        let v5206 = v32 * v5064;
                                                        let v5208 = v5063 + (v5203 * v5064);
                                                        v5223 = v5206;
                                                        v5240 = v5208;
                                                        v5262 = v5202;
                                                        v5282 = v5205;
                                                    } else {
                                                        v5223 = v0;
                                                        v5240 = v0;
                                                        v5262 = v0;
                                                        v5282 = v0;
                                                    }
                                                    v5222 = v5223;
                                                    v5239 = v5240;
                                                    v5261 = v5262;
                                                    v5281 = v5282;
                                                }
                                                v5221 = v5222;
                                                v5238 = v5239;
                                                v5260 = v5261;
                                                v5280 = v5281;
                                            }
                                            v5220 = v5221;
                                            v5237 = v5238;
                                            v5259 = v5260;
                                            v5279 = v5280;
                                        }
                                        v5219 = v5220;
                                        v5236 = v5237;
                                        v5258 = v5259;
                                        v5278 = v5279;
                                    }
                                    v5218 = v5219;
                                    v5235 = v5236;
                                    v5257 = v5258;
                                    v5277 = v5278;
                                }
                                v5217 = v5218;
                                v5234 = v5235;
                                v5256 = v5257;
                                v5276 = v5277;
                            }
                            v5216 = v5217;
                            v5233 = v5234;
                            v5255 = v5256;
                            v5275 = v5276;
                        }
                        v5215 = v5216;
                        v5232 = v5233;
                        v5254 = v5255;
                        v5274 = v5275;
                    }
                    v5214 = v5215;
                    v5231 = v5232;
                    v5253 = v5254;
                    v5273 = v5274;
                }
                v5213 = v5214;
                v5230 = v5231;
                v5252 = v5253;
                v5272 = v5273;
            }
            let v5224: f64;
            if v5209 != 0.0 {
                let v5212 = (v5210 * v27) * v24;
                v5224 = v5212;
            } else {
                v5224 = v5213;
            }
            let v5225 = if v5224 < v0 { 1.0 } else { 0.0 };
            let v9233: f64;
            if v5225 != 0.0 {
                v9233 = v0;
            } else {
                v9233 = v5224;
            }
            let v5241: f64;
            if v5226 != 0.0 {
                let v5229 = (v5227 * v27) * v24;
                v5241 = v5229;
            } else {
                v5241 = v5230;
            }
            let v5242 = if v5241 < v0 { 1.0 } else { 0.0 };
            let v9396: f64;
            if v5242 != 0.0 {
                v9396 = v0;
            } else {
                v9396 = v5241;
            }
            let v9235: f64;
            if v5243 != 0.0 {
                let v5245 = if v5244 == v0 { 1.0 } else { 0.0 };
                let v9236: f64;
                if v5245 != 0.0 {
                    let v5247 = v5246 * v27;
                    v9236 = v5247;
                } else {
                    let v5251 = if ((v5246 * v27) - (v125 * v32)) >= v0 { ((v5246 * v27) - (v125 * v32)) } else { v0 };
                    v9236 = v5251;
                }
                v9235 = v9236;
            } else {
                let v5263 = if v5252 < v0 { 1.0 } else { 0.0 };
                let v9237: f64;
                if v5263 != 0.0 {
                    v9237 = v0;
                } else {
                    v9237 = v5252;
                }
                v9235 = v9237;
            }
            let v9398: f64;
            if v5264 != 0.0 {
                let v5265 = if v5244 == v0 { 1.0 } else { 0.0 };
                let v9399: f64;
                if v5265 != 0.0 {
                    let v5267 = v5266 * v27;
                    v9399 = v5267;
                } else {
                    let v5271 = if ((v5266 * v27) - (v125 * v32)) >= v0 { ((v5266 * v27) - (v125 * v32)) } else { v0 };
                    v9399 = v5271;
                }
                v9398 = v9399;
            } else {
                let v5283 = if v5272 < v0 { 1.0 } else { 0.0 };
                let v9400: f64;
                if v5283 != 0.0 {
                    v9400 = v0;
                } else {
                    v9400 = v5272;
                }
                v9398 = v9400;
            }
            let v5295 = if (if (if v5284 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v5286 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v32 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if (if v32 > v1 { 1.0 } else { 0.0 }) != 0.0 && (if v5291 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5484: f64;
            let v5486: f64;
            let v5488: f64;
            let v5568: f64;
            let v5586: f64;
            let v5791: f64;
            let v13073: f64;
            let v13213: f64;
            if v5295 != 0.0 {
                let v5297 = v30.powf(v5296);
                let v5299 = v35 + v5298;
                let v5301 = v5299.powf(v5300);
                let v5313 = v30.powf(v5312);
                let v5315 = v5299.powf(v5314);
                let v5325 = v1 + (((v5316 / v5313) + (v5318 / v5315)) + (v5321 / (v5313 * v5315)));
                let v5330 = ((v1 + (((v5302 / v5297) + (v5304 / v5301)) + (v5307 / (v5297 * v5301)))) * (v1 + (v5326 * v4706))) + v75;
                let mut v5331: f64 = 0.0;
                let mut v5343: f64 = 0.0;
                let mut v5345: f64 = 0.0;
                v5331 = v0;
                v5343 = v0;
                v5345 = v0;
                loop {
                    let v5332 = if v5331 < v32 { 1.0 } else { 0.0 };
                    if v5332 == 0.0 {
                        break;
                    }
                    let v5333 = v1 / v32;
                    let v5334 = v2485 * v25;
                    let v5337 = v5331 * (v5291 + v25);
                    let v5344 = v5343 + (v5333 / ((v5284 + v5334) + v5337));
                    let v5346 = v5345 + (v5333 / ((v5286 + v5334) + v5337));
                    let v5347 = v5331 + v1;
                    v5331 = v5347;
                    v5343 = v5344;
                    v5345 = v5346;
                }
                let v5349 = v2485 * v25;
                let v5355 = (v1 / (v5348 + v5349)) + (v1 / (v5352 + v5349));
                let v5357 = v5356 / v5330;
                let v5358 = v5357 * v5355;
                let v5359 = v5343 + v5345;
                let v5360 = v5357 * v5359;
                let v5372 = v5359 - v5355;
                let v5373 = (v5370 / v5325) * v5372;
                let v5376 = v5325.powf(v5375);
                let v5381 = v5325.powf(v5380);
                let v5384 = v4731 * ((v1 + v5360) / (v1 + v5358));
                let v5386 = v5385 * ((v1 + (v5360 * v5364)) / (v1 + (v5358 * v5364)));
                let v5387 = v2781 + ((v5374 / v5376) * v5372);
                let v5388 = v4720 + ((v5379 / v5381) * v5372);
                let v5390 = if v5389 == v1 { 1.0 } else { 0.0 };
                let v5397: f64;
                let v5399: f64;
                let v13214: f64;
                if v5390 != 0.0 {
                    let v5392 = (v2009 / v5325) * v5372;
                    let v5394 = (v2039 / v5376) * v5372;
                    let v5396 = (v2049 / v5381) * v5372;
                    v5397 = v5394;
                    v5399 = v5396;
                    v13214 = v5392;
                } else {
                    v5397 = v0;
                    v5399 = v0;
                    v13214 = v0;
                }
                let v5398 = v2803 + v5397;
                let v5400 = v1919 + v5399;
                v5484 = v5384;
                v5486 = v5387;
                v5488 = v5398;
                v5568 = v5388;
                v5586 = v5386;
                v5791 = v5373;
                v13073 = v5400;
                v13213 = v13214;
            } else {
                v5484 = v4731;
                v5486 = v2781;
                v5488 = v2803;
                v5568 = v4720;
                v5586 = v5385;
                v5791 = v0;
                v13073 = v1919;
                v13213 = v0;
            }
            let v5402 = if v5401 == v1 { 1.0 } else { 0.0 };
            let v5463: f64;
            let v5467: f64;
            let v5473: f64;
            if v5402 != 0.0 {
                let v5403 = v26 / v32;
                let v5414 = if (if (if v5407 == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v5409 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v5412 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5464: f64;
                let v5468: f64;
                let v5474: f64;
                if v5414 != 0.0 {
                    let v5418 = if v5415 != 0.0 && (if v5416 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v5465: f64;
                    let v5469: f64;
                    let v5475: f64;
                    if v5418 != 0.0 {
                        let v5419 = v5416 + v5403;
                        let v5421 = v1 / v5420;
                        let v5424 = (v5420 * v5420) / (v5416 * v5419);
                        let v5427 = v4979 * v5420;
                        let v5442 = ((((v5425 * v5416) + v5427) * (rspice_limited_exp(((v5429 * v5416) * v5421)))) - (((v5425 * v5419) + v5427) * (rspice_limited_exp(((v5436 * v5419) * v5421))))) / v5403;
                        let v5446 = v5445 * v5420;
                        let v5462 = ((((v5443 * v5416) + v5446) * (rspice_limited_exp(((v5449 * v5416) * v5421)))) - (((v5443 * v5419) + v5446) * (rspice_limited_exp(((v5456 * v5419) * v5421))))) / v5403;
                        v5465 = v5424;
                        v5469 = v5442;
                        v5475 = v5462;
                    } else {
                        v5465 = v5404;
                        v5469 = v5405;
                        v5475 = v5406;
                    }
                    v5464 = v5465;
                    v5468 = v5469;
                    v5474 = v5475;
                } else {
                    v5464 = v5404;
                    v5468 = v5405;
                    v5474 = v5406;
                }
                v5463 = v5464;
                v5467 = v5468;
                v5473 = v5474;
            } else {
                v5463 = v0;
                v5467 = v0;
                v5473 = v0;
            }
            let v5477 = (v5463 + (v5466 * v5467)) + (v5472 * v5473);
            let v5478 = v1289 * v5477;
            let v5480 = v2029 * v5477;
            let v5485 = v5484 * (v1 + (v1309 * v5477));
            let v5487 = v5486 + (v1299 * v5477);
            let v5489 = v5488 + (v2019 * v5477);
            let v5493 = v4661 * (v5490 - v5491);
            let v5496 = v4661 * (v5490 - v5494);
            let v5498 = v5497 - v5491;
            let v5499 = v4661 * v5498;
            let v5501 = v5500 - v5491;
            let v5502 = v4661 * v5501;
            let v5504 = v4661 * (v5500 - v5494);
            let v5505 = v5499 - v5502;
            let v5507 = v4661 * (v5491 - v5500);
            let v5509 = v4661 * (v5491 - v5497);
            let v5510 = v5493 - v5499;
            let v5511 = v5493 - v5502;
            let v5514 = v4661 * (v5512 - v5491);
            let v5516 = v4661 * (v5512 - v5494);
            let v5517 = v5514 - v5502;
            let v5518 = v5514 - v5499;
            let v5519 = -v4661;
            let v5520 = v5519 * v5501;
            let v5521 = if v5505 < v0 { 1.0 } else { 0.0 };
            let v5526: f64;
            let v5527: f64;
            let v5547: f64;
            let v9216: f64;
            let v9678: f64;
            let v9691: f64;
            if v5521 != 0.0 {
                let v5524 = v4661 * (v5497 - v5494);
                let v5525 = v5519 * v5498;
                v5526 = v5502;
                v5527 = v5499;
                v5547 = v5524;
                v9216 = v5522;
                v9678 = v5518;
                v9691 = v5525;
            } else {
                v5526 = v5499;
                v5527 = v5502;
                v5547 = v5504;
                v9216 = v1;
                v9678 = v5517;
                v9691 = v5520;
            }
            let v5528 = v5526 - v5527;
            let v5530 = v5529 * v5528;
            let v5532 = if v5530 > v5531 { 1.0 } else { 0.0 };
            let v5537: f64;
            if v5532 != 0.0 {
                v5537 = v5530;
            } else {
                let v5535 = (v1 + (rspice_limited_exp(v5530))).ln();
                v5537 = v5535;
            }
            let v5536 = v71 / v5529;
            let v5542 = ((v5536 * v5537) - v5528) - (v5536 * v5540);
            let v5544 = v2485 * (v5528 - v5542);
            let v5546 = -(v5527 + v5544);
            let v5549 = -(v5547 + v5544);
            let v5552 = v5514 + (v2485 * (v5542 - v5528));
            let v5558 = v2485 + (v2485 * (((v5553 * v5505) / v4613).tanh()));
            let v5559 = v1 - v5558;
            let v5669: f64;
            let v5731: f64;
            let v7181: f64;
            let v7189: f64;
            let v7190: f64;
            let v7195: f64;
            let v7234: f64;
            let v7236: f64;
            let v8741: f64;
            let v8770: f64;
            let v13091: f64;
            let v16258: f64;
            let v16278: f64;
            if v2131 != 0.0 {
                let v5562 = (v2891 * v5559) + (v1899 * v5558);
                let v5565 = (v2889 * v5559) + (v2322 * v5558);
                let v5570 = (v5566 * v5559) + (v5568 * v5558);
                let v5574 = (v5571 * v5559) + (v2473 * v5558);
                let v5578 = (v5575 * v5559) + (v2505 * v5558);
                let v5582 = (v5579 * v5559) + (v2543 * v5558);
                let v5588 = (v5583 * v5559) + (v5586 * v5558);
                let v5592 = (v5589 * v5559) + (v4829 * v5558);
                let v5596 = (v5593 * v5559) + (v5485 * v5558);
                let v5600 = (v5597 * v5559) + (v4742 * v5558);
                let v5604 = (v5601 * v5559) + (v4752 * v5558);
                let v5608 = (v5605 * v5559) + (v4755 * v5558);
                let v5612 = (v5609 * v5559) + (v4758 * v5558);
                v5669 = v5565;
                v5731 = v5570;
                v7181 = v5612;
                v7189 = v5600;
                v7190 = v5604;
                v7195 = v5608;
                v7234 = v5588;
                v7236 = v5596;
                v8741 = v5574;
                v8770 = v5578;
                v13091 = v5562;
                v16258 = v5582;
                v16278 = v5592;
            } else {
                v5669 = v2322;
                v5731 = v5568;
                v7181 = v4758;
                v7189 = v4742;
                v7190 = v4752;
                v7195 = v4755;
                v7234 = v5586;
                v7236 = v5485;
                v8741 = v2473;
                v8770 = v2505;
                v13091 = v1899;
                v16258 = v2543;
                v16278 = v4829;
            }
            let v5617 = v14 * v12;
            let v5622 = (v5613 * (v5617 + (v5618 * v5613))).sqrt();
            let v5646 = ((v359 * v73) / (v5622 + (((((v289 + (v299 * ((((v5493 - v2661) * (v5623 * v14)) + ((v5514 - v209) * (v5617 + v5613))) / (v5613 + (v14 * (v12 + v5623)))))).atan()) / v5638) + v2485) * ((((v14 * v5613) * v12).sqrt()) - v5622)))) + v127;
            let v5648 = if v5646 < v5647 { 1.0 } else { 0.0 };
            let v5699: f64;
            if v5648 != 0.0 {
                let v5651 = v2485 / ((v5646.cosh()) - v1);
                v5699 = v5651;
            } else {
                let v5653 = rspice_limited_exp((-v5646));
                v5699 = v5653;
            }
            let v5654 = v9 / v5613;
            let v5655 = v11 / v5623;
            let v5656 = v4695 - v5546;
            let v5658 = v5656 - v5443;
            let v5665 = (v2485 * ((v5656 + v5443) + (((v5658 * v5658) + v5660).sqrt()))).sqrt();
            let v5666 = v4700 * v5665;
            let v5667 = v9 / v5666;
            let v5675 = v1 + ((((v269 + v4716) + (v5669 * v5542)) - (v2335 * v5546)) / v13);
            let v5720: f64;
            if v2639 != 0.0 {
                let v5715 = v13 + ((v5654 * v5655) / (v5654 + v5655));
                let v5719 = (((v5715 + v269) + v4716) + (((((v5686 * v5552) + ((v5688 * v5552) * v5552)) - (v5692 * v5546)) - ((v5695 * v5546) * v5546)) + (v5699 * (((((v379 + (v419 * v5552)) + ((v5702 * v5552) * v5552)) + (v439 * v5546)) + ((v5708 * v5546) * v5546)) + (((v5669 + (v5679 * v5552)) - (v5682 * v5546)) * v5542))))) / v5715;
                v5720 = v5719;
            } else {
                v5720 = v5675;
            }
            let v5722 = v5720 - v1;
            let v5728 = v2485 * ((v5720 + v1) + (((v5722 * v5722) + v5724).sqrt()));
            let v5729 = v5728 * v4613;
            let v5730 = v1 / v5729;
            let v5735 = (-(v5731 + (v2464 * v5546))) * v5542;
            let v5743 = (v2485 * (v5735 - (((v5735 * v5735) + v5737).sqrt()))) + v5742;
            let v5752 = ((v1759 + (v5744 / v73)) + (v1769 * v5546)) * ((v4616.powf(v5749)) - v1);
            let v5753 = if v449 > v0 { 1.0 } else { 0.0 };
            let v5772: f64;
            if v5753 != 0.0 {
                let v5755 = (-v469) * v5542;
                let v5757 = if v5755 < v5756 { 1.0 } else { 0.0 };
                let v5760: f64;
                if v5757 != 0.0 {
                    v5760 = v5758;
                } else {
                    let v5759 = rspice_limited_exp(v5755);
                    v5760 = v5759;
                }
                let v5768 = (-v5729) * ((if (v73 / (v73 + (v449 * (v1 + v5760)))) >= v4546 { (v73 / (v73 + (v449 * (v1 + v5760)))) } else { v4546 }).ln());
                v5772 = v5768;
            } else {
                v5772 = v0;
            }
            let v5778 = v2661 + v5777;
            let v5780 = v5527 * v5730;
            let v5784 = v5665 - v4696;
            let v5796 = ((v5493 * v5730) - (v5778 * v5730)) - ((((((v5743 + (v5772 - ((v519 + (v489 / (v73.powf(v499)))) * ((v509 * v5542).tanh())))) + ((v2737 * v5784) - (v5487 * v5546))) - v5752) + v5791) + v5478) * v5730);
            let v5797 = (v5514 * v5730) - (v209 * v5730);
            let v5803 = ((((v5798 * v9) * v2291) * v4614).sqrt()) / v13;
            let v5804 = v71 * v4691;
            let v5805 = v5527 * v4614;
            let v5806 = v5804 + v5805;
            let v5816 = v1 + (v5803 / (v71 * ((v2485 * (v5806 + (((v5806 * v5806) + v5808).sqrt()))).sqrt())));
            let v5817 = v71 * v5816;
            let v5818 = v1 / v5803;
            let v19844: f64;
            let v19845: f64;
            let v19847: f64;
            let v19851: f64;
            let v19857: f64;
            let v19863: f64;
            let v19869: f64;
            let v19884: f64;
            let v19927: f64;
            let v19957: f64;
            let v19958: f64;
            let v19959: f64;
            let v19960: f64;
            let v19962: f64;
            let v19964: f64;
            let v19967: f64;
            let v19970: f64;
            let v19973: f64;
            let v19976: f64;
            let v19979: f64;
            let v19982: f64;
            let v19984: f64;
            let v19986: f64;
            let v19988: f64;
            let v19990: f64;
            let v19992: f64;
            let v19994: f64;
            let v19996: f64;
            let v19998: f64;
            let v20000: f64;
            let v20002: f64;
            let v20004: f64;
            let v20006: f64;
            let v20008: f64;
            let v20010: f64;
            let v20013: f64;
            let v20016: f64;
            let v20019: f64;
            let v20022: f64;
            let v20025: f64;
            let v20028: f64;
            let v20030: f64;
            let v20032: f64;
            let v20034: f64;
            let v20036: f64;
            let v20038: f64;
            let v20040: f64;
            let v20042: f64;
            let v20044: f64;
            if v2639 != 0.0 {
                let v5824 = ((((v5819 * v9) * v2291) * v5730).sqrt()) / v13;
                let v5825 = v1 / v5824;
                let v5826 = v5824 * v5824;
                let v5827 = v1 / v5826;
                let v5829 = (v5655 + v339) / v5654;
                let v5830 = v12 / v5623;
                let v5831 = v5824 / v5830;
                let v5834 = v1 + (v5831 * v5832);
                let v5836 = v5835 * v5834;
                let v5838 = v1 / v5831;
                let v5839 = v5831 * v5831;
                let v5843 = v1 / (v5837 + (v5831 * v5840));
                let v5844 = v5797.abs();
                let v5845 = if v5844 <= v5836 { 1.0 } else { 0.0 };
                let v5975: f64;
                if v5845 != 0.0 {
                    let v5846 = -v5797;
                    let v5854 = (v5846 * v5838) * (v1 + (v5831 * (v5846 / ((v5848 * v5834) * v5834))));
                    v5975 = v5854;
                } else {
                    let v5856 = if v5797 < (-v5836) { 1.0 } else { 0.0 };
                    let v5976: f64;
                    if v5856 != 0.0 {
                        let v5857 = -v5797;
                        let v5859 = (v5837 * v5857) * v5838;
                        let v5861 = v5859 - v2979;
                        let v5867 = v2485 * ((v5859 + v2908) - (((v5861 * v5861) + v5863).sqrt()));
                        let v5868 = v5857 - v5867;
                        let v5872 = (v5868 * v5868) + (v5839 * (v5867 + v1));
                        let v5874 = (v71 * v5868) - v5839;
                        let v5878 = ((if (v5872 / v5839) >= v4546 { (v5872 / v5839) } else { v4546 }).ln()) - v5867;
                        let v5879 = v5872 + v5874;
                        let v5881 = v5874 * v5874;
                        let v5885 = (v5879 * v5879) + (v5878 * ((v2485 * v5881) - v5872));
                        let v5897 = v5867 + (((v5872 * v5879) * v5878) / (v5885 + (((((v5879 / v5885) * v5878) * v5878) * v5874) * ((v5881 * v4724) - v5872))));
                        let v5898 = rspice_limited_exp(v5897);
                        let v5899 = v5857 - v5897;
                        let v5903 = (v71 * v5899) + (v5839 * (v5898 - v1));
                        let v5908 = (v5899 * v5899) + (v5839 * ((v5897 + v1) - v5898));
                        let v5921 = -(v5897 + (v71 * (v5908 / (v5903 + (((v5903 * v5903) - (v2976 * ((v1 - ((v5839 * v2485) * v5898)) * v5908))).sqrt())))));
                        v5976 = v5921;
                    } else {
                        let v5933 = v5839 * v2485;
                        let v5940 = (v5797 + v5933) - (v5831 * (((v5797 + (v5839 * v2542)) - (v1 - (rspice_limited_exp((-((v5797 * v5838) * (v1 + (((((v5834 * v5837) * v5843) - v1) * v5843) * v5797)))))))).sqrt()));
                        let v5942 = rspice_limited_exp((-v5940));
                        let v5943 = v5797 - v5940;
                        let v5947 = (v71 * v5943) + (v5839 * (v1 - v5942));
                        let v5952 = (v5943 * v5943) - (v5839 * ((v5940 - v1) + v5942));
                        let v5963 = v5940 + (v71 * (v5952 / (v5947 + (((v5947 * v5947) - (v2976 * ((v1 - (v5933 * v5942)) * v5952))).sqrt()))));
                        v5976 = v5963;
                    }
                    v5975 = v5976;
                }
                let v5964 = if v5844 < v5836 { 1.0 } else { 0.0 };
                let v5996: f64;
                if v5964 != 0.0 {
                    let v5965 = -v5797;
                    let v5973 = (v5965 * v5838) * (v1 + (v5831 * (v5965 / ((v5967 * v5834) * v5834))));
                    v5996 = v5973;
                } else {
                    let v5974 = v5830 * v5830;
                    let v5977 = v5797 - v5975;
                    let v5983 = rspice_limited_exp((-v5975));
                    let v5995 = v5975 - ((((((v5974 * v5977) * v5977) * v5825) * v5825) - ((v5983 + v5975) - v1)) / ((v5983 + ((v5974 * ((v71 * v5975) - (v71 * v5797))) / v5826)) - v1));
                    v5996 = v5995;
                }
                let v5997 = v5996 * v5729;
                let v5999 = v1 + (v5824 * v5832);
                let v6000 = v1 / v5999;
                let v6001 = v5804 / v5728;
                let v6002 = v6001 + v5780;
                let v6004 = rspice_limited_exp((-v6002));
                let v6005 = v4710 * v5999;
                let v6008 = ((-v319) * v73) / v4704;
                let v6014 = v309 * ((rspice_limited_exp((v2485 * v6008))) + (v71 * (rspice_limited_exp(v6008))));
                let v6016 = v1 + (v939 / v73);
                let v6024 = ((v6016 * ((v4698 * v5613) * v5613)) / (v4697 * v5729)) + (v6022 / v5729);
                let v6025 = v5829 * v5797;
                let v6026 = v6024 - v6025;
                let v6033 = v6026 + (v5824 * ((((rspice_limited_exp((-v6026))) + v6026) - v1).sqrt()));
                let v6034 = if v6026 < v6002 { 1.0 } else { 0.0 };
                let v6635: f64;
                if v6034 != 0.0 {
                    let v6035 = if v5796 < v6033 { 1.0 } else { 0.0 };
                    let v6636: f64;
                    if v6035 != 0.0 {
                        let v6037 = if (v5796.abs()) <= v6005 { 1.0 } else { 0.0 };
                        let v6637: f64;
                        if v6037 != 0.0 {
                            let v6048 = (v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * (((v6000 * v6000) * v6039) * v5832)));
                            v6637 = v6048;
                        } else {
                            let v6050 = if v5796 < (-v6005) { 1.0 } else { 0.0 };
                            let v6638: f64;
                            if v6050 != 0.0 {
                                let v6051 = -v5796;
                                let v6054 = v6052 * (v6051 * v6000);
                                let v6056 = v6054 - v2979;
                                let v6061 = v2485 * ((v6054 + v2908) - (((v6056 * v6056) + v5863).sqrt()));
                                let v6062 = v6051 - v6061;
                                let v6066 = (v6062 * v6062) + (v5826 * (v6061 + v1));
                                let v6068 = (v71 * v6062) - v5826;
                                let v6073 = (-v6061) + ((if (v6066 * v5827) >= v4546 { (v6066 * v5827) } else { v4546 }).ln());
                                let v6074 = v6066 + v6068;
                                let v6076 = v6068 * v6068;
                                let v6080 = (v6074 * v6074) + (v6073 * ((v2485 * v6076) - v6066));
                                let v6092 = v6061 + (((v6066 * v6074) * v6073) / (v6080 + (((((v6074 / v6080) * v6073) * v6073) * v6068) * ((v6076 * v4724) - v6066))));
                                let v6093 = rspice_limited_exp(v6092);
                                let v6095 = v6092 * v6092;
                                let v6097 = v1 / (v71 + v6095);
                                let v6098 = v6095 * v6097;
                                let v6108 = v6051 - v6092;
                                let v6109 = v6004 * (v1 / v6093);
                                let v6117 = (v71 * v6108) + (v5826 * (((v6093 - v1) - v6109) + (v6004 * (v1 - (v2976 * ((v6092 * v6097) * v6097))))));
                                let v6127 = (v6108 * v6108) - (v5826 * ((((v6093 - v6092) - v1) + v6109) + (v6004 * ((v6092 - v1) - v6098))));
                                let v6142 = (-v6092) - (v71 * (v6127 / (v6117 + (((v6117 * v6117) - (v71 * (v6127 * (v71 - (v5826 * ((v6093 + v6109) - (v6004 * ((((v3003 * v6097) - (v6103 * v6098)) * v6097) * v6097)))))))).sqrt()))));
                                v6638 = v6142;
                            } else {
                                let v6145 = v1 / (v6052 + (v5824 * v5840));
                                let v6164 = (v5796 + (v5826 * v2485)) - (v5824 * (((v5796 + (v5826 * v2542)) - (v1 - (rspice_limited_exp((-((v5796 * v6000) * (v1 + (((((v5999 * v6052) * v6145) - v1) * v6145) * v5796)))))))).sqrt()));
                                let v6165 = v6002 + v2974;
                                let v6167 = v6164 - v6165;
                                let v6178 = (v2485 * ((v6164 + v6165) - (((v6167 * v6167) + v2964).sqrt()))) - (v2485 * (v6165 - (((v6165 * v6165) + v2964).sqrt())));
                                let v6179 = v5796 - v6178;
                                let v6181 = rspice_limited_exp((-v6178));
                                let v6182 = v6178 * v6178;
                                let v6184 = v1 / (v71 + v6182);
                                let v6185 = v6182 * v6184;
                                let v6204 = if v6194 >= ((v6179 * v6179) - (v5826 * (((v6181 + v6178) - v1) - (v6004 * ((v6178 + v1) + v6185))))) { v6194 } else { ((v6179 * v6179) - (v5826 * (((v6181 + v6178) - v1) - (v6004 * ((v6178 + v1) + v6185))))) };
                                let v6216 = (v71 * v6179) + (v5826 * ((v1 - v6181) - (v6004 * (v1 + (v2976 * ((v6178 * v6184) * v6184))))));
                                let v6221 = (v6002 - v6178) + ((if (v6204 / v5826) >= v4546 { (v6204 / v5826) } else { v4546 }).ln());
                                let v6222 = v6204 + v6216;
                                let v6224 = v6216 * v6216;
                                let v6226 = v6204 * (v1 - (v2485 * (v5826 * (v6181 - (v6004 * ((((v3003 * v6184) - (v6103 * v6185)) * v6184) * v6184))))));
                                let v6229 = (v6222 * v6222) + (v6221 * ((v2485 * v6224) - v6226));
                                let v6241 = v6178 + (((v6204 * v6222) * v6221) / (v6229 + (((((v6222 / v6229) * v6221) * v6221) * v6216) * ((v6224 * v4724) - v6226))));
                                let v6243 = v1 / (rspice_limited_exp(v6241));
                                let v6245 = rspice_limited_exp((v6241 - v6002));
                                let v6246 = v6241 * v6241;
                                let v6248 = v1 / (v71 + v6246);
                                let v6249 = v6246 * v6248;
                                let v6258 = v5796 - v6241;
                                let v6266 = (v71 * v6258) + (v5826 * (((v1 - v6243) + v6245) - (v6004 * (v1 + (v2976 * ((v6241 * v6248) * v6248))))));
                                let v6276 = (v6258 * v6258) - (v5826 * ((((v6243 + v6241) - v1) + v6245) - (v6004 * ((v6241 + v1) + v6249))));
                                let v6290 = v6241 + (v71 * (v6276 / (v6266 + (((v6266 * v6266) - (v71 * (v6276 * (v71 - (v5826 * ((v6243 + v6245) - (v6004 * ((((v3003 * v6248) - (v6103 * v6249)) * v6248) * v6248)))))))).sqrt()))));
                                v6638 = v6290;
                            }
                            v6637 = v6638;
                        }
                        v6636 = v6637;
                    } else {
                        let v6291 = v5830 * v5830;
                        let v6293 = v6026 - (v5997 * v5730);
                        let v6300 = v5796 - (v5824 * ((((rspice_limited_exp((-v6293))) + v6293) - v1).sqrt()));
                        let v6301 = v6002 + v2974;
                        let v6303 = v6300 - v6301;
                        let v6308 = v2485 * ((v6300 + v6301) - (((v6303 * v6303) + v5647).sqrt()));
                        let v6309 = v5796 - v6308;
                        let v6312 = (v5797 - v6308) + v6026;
                        let v6317 = ((v6309 * v6309) - ((v6291 * v6312) * v6312)) - (v5826 * v6026);
                        let v6319 = v71 * v6291;
                        let v6321 = (v71 * v6309) - (v6319 * v6312);
                        let v6322 = v6321 * v6321;
                        let v6323 = v1 - v6291;
                        let v6324 = if v6317 < v0 { 1.0 } else { 0.0 };
                        let v6326: f64;
                        if v6324 != 0.0 {
                            v6326 = v0;
                        } else {
                            v6326 = v6317;
                        }
                        let v6331 = v6326 + v6321;
                        let v6336 = v6326 * v6323;
                        let v6337 = (((v6331 * v6331) / ((v6002 - v6308) + ((if (v6326 * v5827) >= v4546 { (v6326 * v5827) } else { v4546 }).ln()))) + (v2485 * v6322)) - v6336;
                        let v6346 = v6308 + ((v6331 * v6326) / (v6337 + (((v6321 * v6331) / v6337) * ((v4724 * v6322) - v6336))));
                        let v6348 = rspice_limited_exp((v6346 - v6002));
                        let v6349 = v5796 - v6346;
                        let v6352 = (v5797 - v6346) + v6026;
                        let v6355 = v5826 * v6348;
                        let v6356 = ((v71 * v6349) - (v6319 * v6352)) + v6355;
                        let v6364 = v71 * (((v6349 * v6349) - ((v6291 * v6352) * v6352)) - (v5826 * (v6026 + v6348)));
                        let v6373 = v6346 + (v6364 / (v6356 + (((v6356 * v6356) - (v6364 * ((v71 - v6319) - v6355))).sqrt())));
                        v6636 = v6373;
                    }
                    v6635 = v6636;
                } else {
                    let v6375 = if (v5796.abs()) <= v6005 { 1.0 } else { 0.0 };
                    let v6639: f64;
                    if v6375 != 0.0 {
                        let v6385 = (v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * (((v6000 * v6000) * v6039) * v5832)));
                        v6639 = v6385;
                    } else {
                        let v6387 = if v5796 < (-v6005) { 1.0 } else { 0.0 };
                        let v6640: f64;
                        if v6387 != 0.0 {
                            let v6388 = -v5796;
                            let v6390 = v6052 * (v6388 * v6000);
                            let v6392 = v6390 - v2979;
                            let v6397 = v2485 * ((v6390 + v2908) - (((v6392 * v6392) + v5863).sqrt()));
                            let v6398 = v6388 - v6397;
                            let v6402 = (v6398 * v6398) + (v5826 * (v6397 + v1));
                            let v6404 = (v71 * v6398) - v5826;
                            let v6409 = (-v6397) + ((if (v6402 * v5827) >= v4546 { (v6402 * v5827) } else { v4546 }).ln());
                            let v6410 = v6402 + v6404;
                            let v6412 = v6404 * v6404;
                            let v6416 = (v6410 * v6410) + (v6409 * ((v2485 * v6412) - v6402));
                            let v6428 = v6397 + (((v6402 * v6410) * v6409) / (v6416 + (((((v6410 / v6416) * v6409) * v6409) * v6404) * ((v6412 * v4724) - v6402))));
                            let v6429 = rspice_limited_exp(v6428);
                            let v6431 = v6428 * v6428;
                            let v6433 = v1 / (v71 + v6431);
                            let v6434 = v6431 * v6433;
                            let v6443 = v6388 - v6428;
                            let v6444 = v6004 * (v1 / v6429);
                            let v6452 = (v71 * v6443) + (v5826 * (((v6429 - v1) - v6444) + (v6004 * (v1 - (v2976 * ((v6428 * v6433) * v6433))))));
                            let v6462 = (v6443 * v6443) - (v5826 * ((((v6429 - v6428) - v1) + v6444) + (v6004 * ((v6428 - v1) - v6434))));
                            let v6471 = (v6452 * v6452) - (v71 * (v6462 * (v71 - (v5826 * ((v6429 + v6444) - (v6004 * ((((v3003 * v6433) - (v6103 * v6434)) * v6433) * v6433)))))));
                            let v6483 = (-v6428) - (v71 * (v6462 / (v6452 + (((((v6471 * v6471) + v6473).sqrt()) - v6476).sqrt()))));
                            v6640 = v6483;
                        } else {
                            let v6486 = v1 / (v6052 + (v5824 * v5840));
                            let v6505 = (v5796 + (v5826 * v2485)) - (v5824 * (((v5796 + (v5826 * v2542)) - (v1 - (rspice_limited_exp((-((v5796 * v6000) * (v1 + (((((v5999 * v6052) * v6486) - v1) * v6486) * v5796)))))))).sqrt()));
                            let v6506 = v6002 + v2974;
                            let v6508 = v6505 - v6506;
                            let v6519 = (v2485 * ((v6505 + v6506) - (((v6508 * v6508) + v2964).sqrt()))) - (v2485 * (v6506 - (((v6506 * v6506) + v2964).sqrt())));
                            let v6520 = v5796 - v6519;
                            let v6522 = rspice_limited_exp((-v6519));
                            let v6523 = v6519 * v6519;
                            let v6525 = v1 / (v71 + v6523);
                            let v6526 = v6523 * v6525;
                            let v6544 = if v6194 >= ((v6520 * v6520) - (v5826 * (((v6522 + v6519) - v1) - (v6004 * ((v6519 + v1) + v6526))))) { v6194 } else { ((v6520 * v6520) - (v5826 * (((v6522 + v6519) - v1) - (v6004 * ((v6519 + v1) + v6526))))) };
                            let v6556 = (v71 * v6520) + (v5826 * ((v1 - v6522) - (v6004 * (v1 + (v2976 * ((v6519 * v6525) * v6525))))));
                            let v6561 = (v6002 - v6519) + ((if (v6544 / v5826) >= v4546 { (v6544 / v5826) } else { v4546 }).ln());
                            let v6562 = v6544 + v6556;
                            let v6564 = v6556 * v6556;
                            let v6566 = v6544 * (v1 - (v2485 * (v5826 * (v6522 - (v6004 * ((((v3003 * v6525) - (v6103 * v6526)) * v6525) * v6525))))));
                            let v6569 = (v6562 * v6562) + (v6561 * ((v2485 * v6564) - v6566));
                            let v6581 = v6519 + (((v6544 * v6562) * v6561) / (v6569 + (((((v6562 / v6569) * v6561) * v6561) * v6556) * ((v6564 * v4724) - v6566))));
                            let v6583 = v1 / (rspice_limited_exp(v6581));
                            let v6585 = rspice_limited_exp((v6581 - v6002));
                            let v6586 = v6581 * v6581;
                            let v6588 = v1 / (v71 + v6586);
                            let v6589 = v6586 * v6588;
                            let v6598 = v5796 - v6581;
                            let v6606 = (v71 * v6598) + (v5826 * (((v1 - v6583) + v6585) - (v6004 * (v1 + (v2976 * ((v6581 * v6588) * v6588))))));
                            let v6616 = (v6598 * v6598) - (v5826 * ((((v6583 + v6581) - v1) + v6585) - (v6004 * ((v6581 + v1) + v6589))));
                            let v6625 = (v6606 * v6606) - (v71 * (v6616 * (v71 - (v5826 * ((v6583 + v6585) - (v6004 * ((((v3003 * v6588) - (v6103 * v6589)) * v6588) * v6588)))))));
                            let v6634 = v6581 + (v71 * (v6616 / (v6606 + (((((v6625 * v6625) + v6473).sqrt()) - v6476).sqrt()))));
                            v6640 = v6634;
                        }
                        v6639 = v6640;
                    }
                    v6635 = v6639;
                }
                let v6643 = ((v6000 * v6000) * v6039) * v5832;
                let v6644 = v5613 * v5613;
                let v6652 = (((v6016 * (v4698 * v6644)) / v4697) + v6022) - ((v5829 * (v5797 * v5729)) * v349);
                let v6653 = v1 + v5829;
                let v6654 = v6653 * v5997;
                let v6655 = v6652 + v6654;
                let v6656 = v5796.abs();
                let v6657 = if v6656 <= v5835 { 1.0 } else { 0.0 };
                let v6815: f64;
                let v8410: f64;
                if v6657 != 0.0 {
                    let v6664 = (v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * v6643));
                    v6815 = v6664;
                    v8410 = v0;
                } else {
                    let v6681 = ((v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * v6643))) * (v2485 * (((v6665 * (v5796 - v71)).tanh()) + ((v2964 * (v5796 + v71)).tanh())));
                    let v6684 = ((v6635 * v5729) - v6655) / v5729;
                    let v6685 = rspice_limited_exp(v6684);
                    let v6688 = ((v6681 * v5729) - v6655) / v5729;
                    let v6690 = if v6684 > v6689 { 1.0 } else { 0.0 };
                    let v6704: f64;
                    if v6690 != 0.0 {
                        v6704 = v6684;
                    } else {
                        let v6692 = if v6684 < v6691 { 1.0 } else { 0.0 };
                        let v6705: f64;
                        if v6692 != 0.0 {
                            let v6693 = v6684.exp();
                            v6705 = v6693;
                        } else {
                            let v6696 = (v1 + (v6684.exp())).ln();
                            v6705 = v6696;
                        }
                        v6704 = v6705;
                    }
                    let v6697 = if v6688 > v6689 { 1.0 } else { 0.0 };
                    let v6706: f64;
                    if v6697 != 0.0 {
                        v6706 = v6688;
                    } else {
                        let v6699 = if v6688 < v6698 { 1.0 } else { 0.0 };
                        let v6707: f64;
                        if v6699 != 0.0 {
                            let v6700 = v6688.exp();
                            v6707 = v6700;
                        } else {
                            let v6703 = (v1 + (v6688.exp())).ln();
                            v6707 = v6703;
                        }
                        v6706 = v6707;
                    }
                    let v6712 = -((v5997 / v5729) + ((v6704 - v6706) / v6653));
                    let v6713 = rspice_limited_exp(v6712);
                    let v6715 = rspice_limited_exp((-v6635));
                    let v6716 = v6635 * v6635;
                    let v6718 = v1 / (v6716 + v71);
                    let v6720 = rspice_limited_exp((v6635 - v6002));
                    let v6721 = v5796 - v6635;
                    let v6724 = v5797 + v6712;
                    let v6733 = v6718 * v6716;
                    let v6738 = ((v6721 * v6721) - (((v5830 * v5830) * v6724) * v6724)) - (v5826 * (((((v6715 - v6713) + v6635) + v6712) + v6720) - (v6004 * ((v6635 + v1) + v6733))));
                    let v6743 = v1 + v6685;
                    let v6744 = v6653 * v6743;
                    let v6748 = v71 * v6635;
                    let v6762 = v6685 / v6744;
                    let v6764 = v6685 * v6713;
                    let v6769 = (((((((v71 * v6685) * v6724) * v5830) * v5830) / v6744) - (v71 * v5796)) + v6748) - (v5826 * (((((v6720 + (v6004 * ((((v6750 * v6635) * v6718) + ((((v6748 * v6635) * v6635) * v6718) * v6718)) - v1))) - v6715) - v6762) + (v6764 / v6744)) + v1));
                    let v6772 = ((v71 * v5830) * v5830) * v6685;
                    let v6775 = v6772 * v6685;
                    let v6814 = v6635 - ((v6738 / v6769) * (v1 + ((v6738 * ((((((v6772 * v6724) / v6744) - (v6775 / ((v6744 * v6653) * v6743))) - (v5826 * (((v6715 + v6720) - (((v71 * v6004) * v6718) * (v1 - (v6733 * (v2964 - ((v2976 * v6716) * v6718)))))) - (v6762 * (((v1 - (v6685 / v6743)) - v6713) + ((v6764 / v6743) * (v1 + (v1 / v6653)))))))) - ((v6775 * v6724) / (v6744 * v6743))) + v71)) / ((v71 * v6769) * v6769))));
                    v6815 = v6814;
                    v8410 = v6681;
                }
                let v6972: f64;
                let v8409: f64;
                if v6657 != 0.0 {
                    let v6822 = (v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * v6643));
                    v6972 = v6822;
                    v8409 = v8410;
                } else {
                    let v6839 = ((v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * v6643))) * (v2485 * (((v6823 * (v5796 - v71)).tanh()) + ((v2964 * (v5796 + v71)).tanh())));
                    let v6842 = ((v6815 * v5729) - v6655) / v5729;
                    let v6843 = rspice_limited_exp(v6842);
                    let v6846 = ((v6839 * v5729) - v6655) / v5729;
                    let v6847 = if v6842 > v6689 { 1.0 } else { 0.0 };
                    let v6861: f64;
                    if v6847 != 0.0 {
                        v6861 = v6842;
                    } else {
                        let v6849 = if v6842 < v6848 { 1.0 } else { 0.0 };
                        let v6862: f64;
                        if v6849 != 0.0 {
                            let v6850 = v6842.exp();
                            v6862 = v6850;
                        } else {
                            let v6853 = (v1 + (v6842.exp())).ln();
                            v6862 = v6853;
                        }
                        v6861 = v6862;
                    }
                    let v6854 = if v6846 > v6689 { 1.0 } else { 0.0 };
                    let v6863: f64;
                    if v6854 != 0.0 {
                        v6863 = v6846;
                    } else {
                        let v6856 = if v6846 < v6855 { 1.0 } else { 0.0 };
                        let v6864: f64;
                        if v6856 != 0.0 {
                            let v6857 = v6846.exp();
                            v6864 = v6857;
                        } else {
                            let v6860 = (v1 + (v6846.exp())).ln();
                            v6864 = v6860;
                        }
                        v6863 = v6864;
                    }
                    let v6869 = -((v5997 / v5729) + ((v6861 - v6863) / v6653));
                    let v6870 = rspice_limited_exp(v6869);
                    let v6872 = rspice_limited_exp((-v6815));
                    let v6873 = v6815 * v6815;
                    let v6875 = v1 / (v6873 + v71);
                    let v6877 = rspice_limited_exp((v6815 - v6002));
                    let v6878 = v5796 - v6815;
                    let v6881 = v5797 + v6869;
                    let v6890 = v6875 * v6873;
                    let v6895 = ((v6878 * v6878) - (((v5830 * v5830) * v6881) * v6881)) - (v5826 * (((((v6872 - v6870) + v6815) + v6869) + v6877) - (v6004 * ((v6815 + v1) + v6890))));
                    let v6900 = v1 + v6843;
                    let v6901 = v6653 * v6900;
                    let v6905 = v71 * v6815;
                    let v6919 = v6843 / v6901;
                    let v6921 = v6843 * v6870;
                    let v6926 = (((((((v71 * v6843) * v6881) * v5830) * v5830) / v6901) - (v71 * v5796)) + v6905) - (v5826 * (((((v6877 + (v6004 * ((((v6907 * v6815) * v6875) + ((((v6905 * v6815) * v6815) * v6875) * v6875)) - v1))) - v6872) - v6919) + (v6921 / v6901)) + v1));
                    let v6929 = ((v71 * v5830) * v5830) * v6843;
                    let v6932 = v6929 * v6843;
                    let v6971 = v6815 - ((v6895 / v6926) * (v1 + ((v6895 * ((((((v6929 * v6881) / v6901) - (v6932 / ((v6901 * v6653) * v6900))) - (v5826 * (((v6872 + v6877) - (((v71 * v6004) * v6875) * (v1 - (v6890 * (v2964 - ((v2976 * v6873) * v6875)))))) - (v6919 * (((v1 - (v6843 / v6900)) - v6870) + ((v6921 / v6900) * (v1 + (v1 / v6653)))))))) - ((v6932 * v6881) / (v6901 * v6900))) + v71)) / ((v71 * v6926) * v6926))));
                    v6972 = v6971;
                    v8409 = v6839;
                }
                let v7131: f64;
                let v8408: f64;
                if v6657 != 0.0 {
                    let v6979 = (v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * v6643));
                    v7131 = v6979;
                    v8408 = v8409;
                } else {
                    let v6996 = ((v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * v6643))) * (v2485 * (((v6980 * (v5796 - v71)).tanh()) + ((v2964 * (v5796 + v71)).tanh())));
                    let v6999 = ((v6972 * v5729) - v6655) / v5729;
                    let v7000 = rspice_limited_exp(v6999);
                    let v7003 = ((v6996 * v5729) - v6655) / v5729;
                    let v7004 = if v6999 > v6689 { 1.0 } else { 0.0 };
                    let v7018: f64;
                    if v7004 != 0.0 {
                        v7018 = v6999;
                    } else {
                        let v7006 = if v6999 < v7005 { 1.0 } else { 0.0 };
                        let v7019: f64;
                        if v7006 != 0.0 {
                            let v7007 = v6999.exp();
                            v7019 = v7007;
                        } else {
                            let v7010 = (v1 + (v6999.exp())).ln();
                            v7019 = v7010;
                        }
                        v7018 = v7019;
                    }
                    let v7011 = if v7003 > v6689 { 1.0 } else { 0.0 };
                    let v7020: f64;
                    if v7011 != 0.0 {
                        v7020 = v7003;
                    } else {
                        let v7013 = if v7003 < v7012 { 1.0 } else { 0.0 };
                        let v7021: f64;
                        if v7013 != 0.0 {
                            let v7014 = v7003.exp();
                            v7021 = v7014;
                        } else {
                            let v7017 = (v1 + (v7003.exp())).ln();
                            v7021 = v7017;
                        }
                        v7020 = v7021;
                    }
                    let v7026 = -((v5997 / v5729) + ((v7018 - v7020) / v6653));
                    let v7027 = rspice_limited_exp(v7026);
                    let v7029 = rspice_limited_exp((-v6972));
                    let v7030 = v6972 * v6972;
                    let v7032 = v1 / (v7030 + v71);
                    let v7034 = rspice_limited_exp((v6972 - v6002));
                    let v7035 = v5796 - v6972;
                    let v7038 = v5797 + v7026;
                    let v7047 = v7032 * v7030;
                    let v7052 = ((v7035 * v7035) - (((v5830 * v5830) * v7038) * v7038)) - (v5826 * (((((v7029 - v7027) + v6972) + v7026) + v7034) - (v6004 * ((v6972 + v1) + v7047))));
                    let v7057 = v1 + v7000;
                    let v7058 = v6653 * v7057;
                    let v7062 = v71 * v6972;
                    let v7076 = v7000 / v7058;
                    let v7078 = v7000 * v7027;
                    let v7083 = (((((((v71 * v7000) * v7038) * v5830) * v5830) / v7058) - (v71 * v5796)) + v7062) - (v5826 * (((((v7034 + (v6004 * ((((v7064 * v6972) * v7032) + ((((v7062 * v6972) * v6972) * v7032) * v7032)) - v1))) - v7029) - v7076) + (v7078 / v7058)) + v1));
                    let v7086 = ((v71 * v5830) * v5830) * v7000;
                    let v7089 = v7086 * v7000;
                    let v7128 = v6972 - ((v7052 / v7083) * (v1 + ((v7052 * ((((((v7086 * v7038) / v7058) - (v7089 / ((v7058 * v6653) * v7057))) - (v5826 * (((v7029 + v7034) - (((v71 * v6004) * v7032) * (v1 - (v7047 * (v2964 - ((v2976 * v7030) * v7032)))))) - (v7076 * (((v1 - (v7000 / v7057)) - v7027) + ((v7078 / v7057) * (v1 + (v1 / v6653)))))))) - ((v7089 * v7038) / (v7058 * v7057))) + v71)) / ((v71 * v7083) * v7083))));
                    v7131 = v7128;
                    v8408 = v6996;
                }
                let v7130 = v7129 * v5729;
                let v7132 = if v7131 <= v0 { 1.0 } else { 0.0 };
                let v9044: f64;
                let v9046: f64;
                let v9048: f64;
                let v9049: f64;
                let v9051: f64;
                let v9056: f64;
                let v9058: f64;
                let v9060: f64;
                let v9062: f64;
                let v9064: f64;
                let v9065: f64;
                let v9073: f64;
                let v9205: f64;
                let v9557: f64;
                let v9558: f64;
                let v9563: f64;
                let v9564: f64;
                let v9569: f64;
                let v12272: f64;
                let v12549: f64;
                let v12664: f64;
                let v12784: f64;
                let v12830: f64;
                let v12936: f64;
                let v12969: f64;
                let v19874: f64;
                let v19889: f64;
                if v7132 != 0.0 {
                    let v7134 = (v5796 - v7131) * v5729;
                    v9044 = v0;
                    v9046 = v0;
                    v9048 = v0;
                    v9049 = v0;
                    v9051 = v1;
                    v9056 = v1;
                    v9058 = v1;
                    v9060 = v1;
                    v9062 = v1;
                    v9064 = v1;
                    v9065 = v1;
                    v9073 = v0;
                    v9205 = v0;
                    v9557 = v7134;
                    v9558 = v0;
                    v9563 = v0;
                    v9564 = v0;
                    v9569 = v1;
                    v12272 = v8408;
                    v12549 = v0;
                    v12664 = v0;
                    v12784 = v0;
                    v12830 = v0;
                    v12936 = v0;
                    v12969 = v7130;
                    v19874 = v7135;
                    v19889 = v7138;
                } else {
                    let v7141 = v7131 * v7131;
                    let v7146 = v1 / (rspice_limited_exp(v7131));
                    let v7152 = (rspice_limited_exp((v7131 - v6002))) - (v6004 * ((v7131 + v1) + (v7141 * (v1 / (v71 + v7141)))));
                    let v7153 = v5796 - v7131;
                    let v7157 = (((v7153 * v7153) * v5827) - v7152) - v4710;
                    let v7165 = (v2485 * (v7157 + (((v7157 * v7157) + v7160).sqrt()))) + v4710;
                    let v7172 = v5824 * (v7165.sqrt());
                    let v7174 = ((v5826 * v7152) * v5729) / ((v5824 * ((v7165 + v7152).sqrt())) + v7172);
                    let v7175 = v7172 * v5729;
                    let v7177 = v7176 / v5617;
                    let v7192 = v7189 + (v7190 * v5546);
                    let v7198 = v1 + ((v7192 * ((v7177 * (v7175 + (v4728 * v7174))).powf(v4768))) + (v7195 / (rspice_limited_exp((v7181 * ((if (v2485 * (v1 + (v7174 / v7175))) >= v4546 { (v2485 * (v1 + (v7174 / v7175))) } else { v4546 }).ln()))))));
                    let v7200 = v7198 - v1;
                    let v7206 = v2485 * ((v7198 + v1) + (((v7200 * v7200) + v7202).sqrt()));
                    let v7211 = v1 / (((v83 * v7207).powf(v769)) * v32);
                    let v7243: f64;
                    if v2834 != 0.0 {
                        v7243 = v0;
                    } else {
                        let v7216 = (v1 / (v1 + (v749 * v7174))) + (v2812 * v5784);
                        let v7228 = ((v7221 + (v7223 * (v7216 + (((v7216 * v7216) + v4979).sqrt())))) * v7211) * v32;
                        let v7229 = v7228 * v4795;
                        let v7230 = if v2833 == v71 { 1.0 } else { 0.0 };
                        let v7244: f64;
                        if v7230 != 0.0 {
                            let v7233 = ((v7138 + v7228) + v7135) * v4795;
                            v7244 = v7233;
                        } else {
                            v7244 = v7229;
                        }
                        v7243 = v7244;
                    }
                    let v7235 = v71 * v7234;
                    let v7239 = ((v7235 / v7236) * v7206) * v73;
                    let v7240 = v71 * v5729;
                    let v7242 = v919 * (v7174 + v7240);
                    let v7245 = if v7243 > v0 { 1.0 } else { 0.0 };
                    let v7297: f64;
                    if v7245 != 0.0 {
                        let v7248 = ((v83 * v7234) * v13) * v7243;
                        let v7249 = v71 * v7248;
                        let v7253 = (v7242 + v7239) + ((v2974 * v7242) * v7248);
                        let v7264 = (v7253 - (((v7253 * v7253) - ((v71 * v7249) * (v7242 * (v7239 + ((v71 * v7242) * v7248))))).sqrt())) / v7249;
                        v7297 = v7264;
                    } else {
                        let v7267 = (v7239 * v7242) / (v7239 + v7242);
                        v7297 = v7267;
                    }
                    let v7272 = if (if v7268 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v7270 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7306: f64;
                    if v7272 != 0.0 {
                        v7306 = v1;
                    } else {
                        let v7276 = v73 / (v73 + ((v599 * v5666).sqrt()));
                        let v7288 = v1 + (((v7268 * v7276) - (((v7270 * v7276) * (v7174.powf(v7279))) * v5729)) / (v1 + (v7284 * v5546)));
                        let v7290 = v7288 - v5425;
                        let v7296 = v2485 * ((v7288 + v5425) + (((v7290 * v7290) + v7292).sqrt()));
                        v7306 = v7296;
                    }
                    let v7298 = v7297 - v4710;
                    let v7307 = ((v2485 * (v7298 + (((v7298 * v7298) + v7300).sqrt()))) + v4710) / v7306;
                    let v7315 = v5528 * ((v1 + (((v5528 / v7307) + v127).powf((v1 / v4819)))).powf((-v4819)));
                    let v7318 = v6001 + ((v7315 + v5527) * v5730);
                    let v7319 = -v7318;
                    let v7320 = rspice_limited_exp(v7319);
                    let v7321 = v5528 * v5730;
                    let v7324 = v6014 * (v7321 + (v329 * v5730));
                    let v7328 = (v6024 - (v6025 * v349)) + (v6653 * v7324);
                    let v7335 = v7328 + (v5824 * ((((rspice_limited_exp((-v7328))) + v7328) - v1).sqrt()));
                    let v7336 = if v7328 < v7318 { 1.0 } else { 0.0 };
                    let v7915: f64;
                    if v7336 != 0.0 {
                        let v7337 = if v5796 < v7335 { 1.0 } else { 0.0 };
                        let v7916: f64;
                        if v7337 != 0.0 {
                            let v7338 = if v6656 <= v6005 { 1.0 } else { 0.0 };
                            let v7917: f64;
                            if v7338 != 0.0 {
                                let v7345 = (v5796 * v6000) * (v1 + (((v5796 * (v1 - v7320)) * v5824) * v6643));
                                v7917 = v7345;
                            } else {
                                let v7347 = if v5796 < (-v6005) { 1.0 } else { 0.0 };
                                let v7918: f64;
                                if v7347 != 0.0 {
                                    let v7348 = -v5796;
                                    let v7350 = v6052 * (v7348 * v6000);
                                    let v7352 = v7350 - v2979;
                                    let v7357 = v2485 * ((v7350 + v2908) - (((v7352 * v7352) + v5863).sqrt()));
                                    let v7358 = v7348 - v7357;
                                    let v7362 = (v7358 * v7358) + (v5826 * (v7357 + v1));
                                    let v7364 = (v71 * v7358) - v5826;
                                    let v7369 = (-v7357) + ((if (v7362 * v5827) >= v4546 { (v7362 * v5827) } else { v4546 }).ln());
                                    let v7370 = v7362 + v7364;
                                    let v7372 = v7364 * v7364;
                                    let v7376 = (v7370 * v7370) + (v7369 * ((v2485 * v7372) - v7362));
                                    let v7388 = v7357 + (((v7362 * v7370) * v7369) / (v7376 + (((((v7370 / v7376) * v7369) * v7369) * v7364) * ((v7372 * v4724) - v7362))));
                                    let v7389 = rspice_limited_exp(v7388);
                                    let v7391 = v7388 * v7388;
                                    let v7393 = v1 / (v71 + v7391);
                                    let v7394 = v7391 * v7393;
                                    let v7403 = v7348 - v7388;
                                    let v7404 = v7320 * (v1 / v7389);
                                    let v7412 = (v71 * v7403) + (v5826 * (((v7389 - v1) - v7404) + (v7320 * (v1 - (v2976 * ((v7388 * v7393) * v7393))))));
                                    let v7422 = (v7403 * v7403) - (v5826 * ((((v7389 - v7388) - v1) + v7404) + (v7320 * ((v7388 - v1) - v7394))));
                                    let v7437 = (-v7388) - (v71 * (v7422 / (v7412 + (((v7412 * v7412) - (v71 * (v7422 * (v71 - (v5826 * ((v7389 + v7404) - (v7320 * ((((v3003 * v7393) - (v6103 * v7394)) * v7393) * v7393)))))))).sqrt()))));
                                    v7918 = v7437;
                                } else {
                                    let v7440 = v1 / (v6052 + (v5824 * v5840));
                                    let v7459 = (v5796 + (v5826 * v2485)) - (v5824 * (((v5796 + (v5826 * v2542)) - (v1 - (rspice_limited_exp((-((v5796 * v6000) * (v1 + (((((v5999 * v6052) * v7440) - v1) * v7440) * v5796)))))))).sqrt()));
                                    let v7460 = v7318 + v2974;
                                    let v7462 = v7459 - v7460;
                                    let v7473 = (v2485 * ((v7459 + v7460) - (((v7462 * v7462) + v2964).sqrt()))) - (v2485 * (v7460 - (((v7460 * v7460) + v2964).sqrt())));
                                    let v7474 = v5796 - v7473;
                                    let v7476 = rspice_limited_exp((-v7473));
                                    let v7477 = v7473 * v7473;
                                    let v7479 = v1 / (v71 + v7477);
                                    let v7480 = v7477 * v7479;
                                    let v7498 = if v6194 >= ((v7474 * v7474) - (v5826 * (((v7476 + v7473) - v1) - (v7320 * ((v7473 + v1) + v7480))))) { v6194 } else { ((v7474 * v7474) - (v5826 * (((v7476 + v7473) - v1) - (v7320 * ((v7473 + v1) + v7480))))) };
                                    let v7510 = (v71 * v7474) + (v5826 * ((v1 - v7476) - (v7320 * (v1 + (v2976 * ((v7473 * v7479) * v7479))))));
                                    let v7515 = (v7318 - v7473) + ((if (v7498 / v5826) >= v4546 { (v7498 / v5826) } else { v4546 }).ln());
                                    let v7516 = v7498 + v7510;
                                    let v7518 = v7510 * v7510;
                                    let v7520 = v7498 * (v1 - (v2485 * (v5826 * (v7476 - (v7320 * ((((v3003 * v7479) - (v6103 * v7480)) * v7479) * v7479))))));
                                    let v7523 = (v7516 * v7516) + (v7515 * ((v2485 * v7518) - v7520));
                                    let v7535 = v7473 + (((v7498 * v7516) * v7515) / (v7523 + (((((v7516 / v7523) * v7515) * v7515) * v7510) * ((v7518 * v4724) - v7520))));
                                    let v7537 = v1 / (rspice_limited_exp(v7535));
                                    let v7539 = rspice_limited_exp((v7535 - v7318));
                                    let v7540 = v7535 * v7535;
                                    let v7542 = v1 / (v71 + v7540);
                                    let v7543 = v7540 * v7542;
                                    let v7552 = v5796 - v7535;
                                    let v7560 = (v71 * v7552) + (v5826 * (((v1 - v7537) + v7539) - (v7320 * (v1 + (v2976 * ((v7535 * v7542) * v7542))))));
                                    let v7570 = (v7552 * v7552) - (v5826 * ((((v7537 + v7535) - v1) + v7539) - (v7320 * ((v7535 + v1) + v7543))));
                                    let v7584 = v7535 + (v71 * (v7570 / (v7560 + (((v7560 * v7560) - (v71 * (v7570 * (v71 - (v5826 * ((v7537 + v7539) - (v7320 * ((((v3003 * v7542) - (v6103 * v7543)) * v7542) * v7542)))))))).sqrt()))));
                                    v7918 = v7584;
                                }
                                v7917 = v7918;
                            }
                            v7916 = v7917;
                        } else {
                            let v7585 = v5830 * v5830;
                            let v7587 = v7328 - (v5997 * v5730);
                            let v7594 = v5796 - (v5824 * ((((rspice_limited_exp((-v7587))) + v7587) - v1).sqrt()));
                            let v7595 = v7318 + v2974;
                            let v7597 = v7594 - v7595;
                            let v7602 = v2485 * ((v7594 + v7595) - (((v7597 * v7597) + v5647).sqrt()));
                            let v7603 = v5796 - v7602;
                            let v7606 = (v5797 - v7602) + v7328;
                            let v7611 = ((v7603 * v7603) - ((v7585 * v7606) * v7606)) - (v5826 * v7328);
                            let v7613 = v71 * v7585;
                            let v7615 = (v71 * v7603) - (v7613 * v7606);
                            let v7616 = v7615 * v7615;
                            let v7617 = v1 - v7585;
                            let v7618 = if v7611 < v0 { 1.0 } else { 0.0 };
                            let v7620: f64;
                            if v7618 != 0.0 {
                                v7620 = v0;
                            } else {
                                v7620 = v7611;
                            }
                            let v7625 = v7620 + v7615;
                            let v7630 = v7620 * v7617;
                            let v7631 = (((v7625 * v7625) / ((v7318 - v7602) + ((if (v7620 * v5827) >= v4546 { (v7620 * v5827) } else { v4546 }).ln()))) + (v2485 * v7616)) - v7630;
                            let v7640 = v7602 + ((v7625 * v7620) / (v7631 + (((v7615 * v7625) / v7631) * ((v4724 * v7616) - v7630))));
                            let v7642 = rspice_limited_exp((v7640 - v7318));
                            let v7643 = v5796 - v7640;
                            let v7646 = (v5797 - v7640) + v7328;
                            let v7649 = v5826 * v7642;
                            let v7650 = ((v71 * v7643) - (v7613 * v7646)) + v7649;
                            let v7658 = v71 * (((v7643 * v7643) - ((v7585 * v7646) * v7646)) - (v5826 * (v7328 + v7642)));
                            let v7667 = v7640 + (v7658 / (v7650 + (((v7650 * v7650) - (v7658 * ((v71 - v7613) - v7649))).sqrt())));
                            v7916 = v7667;
                        }
                        v7915 = v7916;
                    } else {
                        let v7668 = if v6656 <= v6005 { 1.0 } else { 0.0 };
                        let v7919: f64;
                        if v7668 != 0.0 {
                            let v7675 = (v5796 * v6000) * (v1 + (((v5796 * (v1 - v7320)) * v5824) * v6643));
                            v7919 = v7675;
                        } else {
                            let v7677 = if v5796 < (-v6005) { 1.0 } else { 0.0 };
                            let v7920: f64;
                            if v7677 != 0.0 {
                                let v7678 = -v5796;
                                let v7680 = v6052 * (v7678 * v6000);
                                let v7682 = v7680 - v2979;
                                let v7687 = v2485 * ((v7680 + v2908) - (((v7682 * v7682) + v5863).sqrt()));
                                let v7688 = v7678 - v7687;
                                let v7692 = (v7688 * v7688) + (v5826 * (v7687 + v1));
                                let v7694 = (v71 * v7688) - v5826;
                                let v7699 = (-v7687) + ((if (v7692 * v5827) >= v4546 { (v7692 * v5827) } else { v4546 }).ln());
                                let v7700 = v7692 + v7694;
                                let v7702 = v7694 * v7694;
                                let v7706 = (v7700 * v7700) + (v7699 * ((v2485 * v7702) - v7692));
                                let v7718 = v7687 + (((v7692 * v7700) * v7699) / (v7706 + (((((v7700 / v7706) * v7699) * v7699) * v7694) * ((v7702 * v4724) - v7692))));
                                let v7719 = rspice_limited_exp(v7718);
                                let v7721 = v7718 * v7718;
                                let v7723 = v1 / (v71 + v7721);
                                let v7724 = v7721 * v7723;
                                let v7733 = v7678 - v7718;
                                let v7734 = v7320 * (v1 / v7719);
                                let v7742 = (v71 * v7733) + (v5826 * (((v7719 - v1) - v7734) + (v7320 * (v1 - (v2976 * ((v7718 * v7723) * v7723))))));
                                let v7752 = (v7733 * v7733) - (v5826 * ((((v7719 - v7718) - v1) + v7734) + (v7320 * ((v7718 - v1) - v7724))));
                                let v7767 = (-v7718) - (v71 * (v7752 / (v7742 + (((v7742 * v7742) - (v71 * (v7752 * (v71 - (v5826 * ((v7719 + v7734) - (v7320 * ((((v3003 * v7723) - (v6103 * v7724)) * v7723) * v7723)))))))).sqrt()))));
                                v7920 = v7767;
                            } else {
                                let v7770 = v1 / (v6052 + (v5824 * v5840));
                                let v7789 = (v5796 + (v5826 * v2485)) - (v5824 * (((v5796 + (v5826 * v2542)) - (v1 - (rspice_limited_exp((-((v5796 * v6000) * (v1 + (((((v5999 * v6052) * v7770) - v1) * v7770) * v5796)))))))).sqrt()));
                                let v7790 = v7318 + v2974;
                                let v7792 = v7789 - v7790;
                                let v7803 = (v2485 * ((v7789 + v7790) - (((v7792 * v7792) + v2964).sqrt()))) - (v2485 * (v7790 - (((v7790 * v7790) + v2964).sqrt())));
                                let v7804 = v5796 - v7803;
                                let v7806 = rspice_limited_exp((-v7803));
                                let v7807 = v7803 * v7803;
                                let v7809 = v1 / (v71 + v7807);
                                let v7810 = v7807 * v7809;
                                let v7828 = if v6194 >= ((v7804 * v7804) - (v5826 * (((v7806 + v7803) - v1) - (v7320 * ((v7803 + v1) + v7810))))) { v6194 } else { ((v7804 * v7804) - (v5826 * (((v7806 + v7803) - v1) - (v7320 * ((v7803 + v1) + v7810))))) };
                                let v7840 = (v71 * v7804) + (v5826 * ((v1 - v7806) - (v7320 * (v1 + (v2976 * ((v7803 * v7809) * v7809))))));
                                let v7845 = (v7318 - v7803) + ((if (v7828 / v5826) >= v4546 { (v7828 / v5826) } else { v4546 }).ln());
                                let v7846 = v7828 + v7840;
                                let v7848 = v7840 * v7840;
                                let v7850 = v7828 * (v1 - (v2485 * (v5826 * (v7806 - (v7320 * ((((v3003 * v7809) - (v6103 * v7810)) * v7809) * v7809))))));
                                let v7853 = (v7846 * v7846) + (v7845 * ((v2485 * v7848) - v7850));
                                let v7865 = v7803 + (((v7828 * v7846) * v7845) / (v7853 + (((((v7846 / v7853) * v7845) * v7845) * v7840) * ((v7848 * v4724) - v7850))));
                                let v7867 = v1 / (rspice_limited_exp(v7865));
                                let v7869 = rspice_limited_exp((v7865 - v7318));
                                let v7870 = v7865 * v7865;
                                let v7872 = v1 / (v71 + v7870);
                                let v7873 = v7870 * v7872;
                                let v7882 = v5796 - v7865;
                                let v7890 = (v71 * v7882) + (v5826 * (((v1 - v7867) + v7869) - (v7320 * (v1 + (v2976 * ((v7865 * v7872) * v7872))))));
                                let v7900 = (v7882 * v7882) - (v5826 * ((((v7867 + v7865) - v1) + v7869) - (v7320 * ((v7865 + v1) + v7873))));
                                let v7914 = v7865 + (v71 * (v7900 / (v7890 + (((v7890 * v7890) - (v71 * (v7900 * (v71 - (v5826 * ((v7867 + v7869) - (v7320 * ((((v3003 * v7872) - (v6103 * v7873)) * v7872) * v7872)))))))).sqrt()))));
                                v7920 = v7914;
                            }
                            v7919 = v7920;
                        }
                        v7915 = v7919;
                    }
                    let v7924 = (v6652 + ((v6653 * v7324) * v5729)) + v6654;
                    let v8081: f64;
                    let v8407: f64;
                    if v6657 != 0.0 {
                        let v7931 = (v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * v6643));
                        v8081 = v7931;
                        v8407 = v8408;
                    } else {
                        let v7948 = ((v5796 * v6000) * (v1 + (((v5796 * (v1 - v7320)) * v5824) * v6643))) * (v2485 * (((v7932 * (v5796 - v71)).tanh()) + ((v2964 * (v5796 + v71)).tanh())));
                        let v7951 = ((v7915 * v5729) - v7924) / v5729;
                        let v7952 = rspice_limited_exp(v7951);
                        let v7955 = ((v7948 * v5729) - v7924) / v5729;
                        let v7956 = if v7951 > v6689 { 1.0 } else { 0.0 };
                        let v7970: f64;
                        if v7956 != 0.0 {
                            v7970 = v7951;
                        } else {
                            let v7958 = if v7951 < v7957 { 1.0 } else { 0.0 };
                            let v7971: f64;
                            if v7958 != 0.0 {
                                let v7959 = v7951.exp();
                                v7971 = v7959;
                            } else {
                                let v7962 = (v1 + (v7951.exp())).ln();
                                v7971 = v7962;
                            }
                            v7970 = v7971;
                        }
                        let v7963 = if v7955 > v6689 { 1.0 } else { 0.0 };
                        let v7972: f64;
                        if v7963 != 0.0 {
                            v7972 = v7955;
                        } else {
                            let v7965 = if v7955 < v7964 { 1.0 } else { 0.0 };
                            let v7973: f64;
                            if v7965 != 0.0 {
                                let v7966 = v7955.exp();
                                v7973 = v7966;
                            } else {
                                let v7969 = (v1 + (v7955.exp())).ln();
                                v7973 = v7969;
                            }
                            v7972 = v7973;
                        }
                        let v7978 = -((v5997 / v5729) + ((v7970 - v7972) / v6653));
                        let v7979 = rspice_limited_exp(v7978);
                        let v7981 = rspice_limited_exp((-v7915));
                        let v7982 = v7915 * v7915;
                        let v7984 = v1 / (v7982 + v71);
                        let v7986 = rspice_limited_exp((v7915 - v7318));
                        let v7987 = v5796 - v7915;
                        let v7990 = v5797 + v7978;
                        let v7999 = v7984 * v7982;
                        let v8004 = ((v7987 * v7987) - (((v5830 * v5830) * v7990) * v7990)) - (v5826 * (((((v7981 - v7979) + v7915) + v7978) + v7986) - (v7320 * ((v7915 + v1) + v7999))));
                        let v8009 = v1 + v7952;
                        let v8010 = v6653 * v8009;
                        let v8014 = v71 * v7915;
                        let v8028 = v7952 / v8010;
                        let v8030 = v7952 * v7979;
                        let v8035 = (((((((v71 * v7952) * v7990) * v5830) * v5830) / v8010) - (v71 * v5796)) + v8014) - (v5826 * (((((v7986 + (v7320 * ((((v8016 * v7915) * v7984) + ((((v8014 * v7915) * v7915) * v7984) * v7984)) - v1))) - v7981) - v8028) + (v8030 / v8010)) + v1));
                        let v8038 = ((v71 * v5830) * v5830) * v7952;
                        let v8041 = v8038 * v7952;
                        let v8080 = v7915 - ((v8004 / v8035) * (v1 + ((v8004 * ((((((v8038 * v7990) / v8010) - (v8041 / ((v8010 * v6653) * v8009))) - (v5826 * (((v7981 + v7986) - (((v71 * v7320) * v7984) * (v1 - (v7999 * (v2964 - ((v2976 * v7982) * v7984)))))) - (v8028 * (((v1 - (v7952 / v8009)) - v7979) + ((v8030 / v8009) * (v1 + (v1 / v6653)))))))) - ((v8041 * v7990) / (v8010 * v8009))) + v71)) / ((v71 * v8035) * v8035))));
                        v8081 = v8080;
                        v8407 = v7948;
                    }
                    let v8238: f64;
                    let v8406: f64;
                    if v6657 != 0.0 {
                        let v8088 = (v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * v6643));
                        v8238 = v8088;
                        v8406 = v8407;
                    } else {
                        let v8105 = ((v5796 * v6000) * (v1 + (((v5796 * (v1 - v7320)) * v5824) * v6643))) * (v2485 * (((v8089 * (v5796 - v71)).tanh()) + ((v2964 * (v5796 + v71)).tanh())));
                        let v8108 = ((v8081 * v5729) - v7924) / v5729;
                        let v8109 = rspice_limited_exp(v8108);
                        let v8112 = ((v8105 * v5729) - v7924) / v5729;
                        let v8113 = if v8108 > v6689 { 1.0 } else { 0.0 };
                        let v8127: f64;
                        if v8113 != 0.0 {
                            v8127 = v8108;
                        } else {
                            let v8115 = if v8108 < v8114 { 1.0 } else { 0.0 };
                            let v8128: f64;
                            if v8115 != 0.0 {
                                let v8116 = v8108.exp();
                                v8128 = v8116;
                            } else {
                                let v8119 = (v1 + (v8108.exp())).ln();
                                v8128 = v8119;
                            }
                            v8127 = v8128;
                        }
                        let v8120 = if v8112 > v6689 { 1.0 } else { 0.0 };
                        let v8129: f64;
                        if v8120 != 0.0 {
                            v8129 = v8112;
                        } else {
                            let v8122 = if v8112 < v8121 { 1.0 } else { 0.0 };
                            let v8130: f64;
                            if v8122 != 0.0 {
                                let v8123 = v8112.exp();
                                v8130 = v8123;
                            } else {
                                let v8126 = (v1 + (v8112.exp())).ln();
                                v8130 = v8126;
                            }
                            v8129 = v8130;
                        }
                        let v8135 = -((v5997 / v5729) + ((v8127 - v8129) / v6653));
                        let v8136 = rspice_limited_exp(v8135);
                        let v8138 = rspice_limited_exp((-v8081));
                        let v8139 = v8081 * v8081;
                        let v8141 = v1 / (v8139 + v71);
                        let v8143 = rspice_limited_exp((v8081 - v7318));
                        let v8144 = v5796 - v8081;
                        let v8147 = v5797 + v8135;
                        let v8156 = v8141 * v8139;
                        let v8161 = ((v8144 * v8144) - (((v5830 * v5830) * v8147) * v8147)) - (v5826 * (((((v8138 - v8136) + v8081) + v8135) + v8143) - (v7320 * ((v8081 + v1) + v8156))));
                        let v8166 = v1 + v8109;
                        let v8167 = v6653 * v8166;
                        let v8171 = v71 * v8081;
                        let v8185 = v8109 / v8167;
                        let v8187 = v8109 * v8136;
                        let v8192 = (((((((v71 * v8109) * v8147) * v5830) * v5830) / v8167) - (v71 * v5796)) + v8171) - (v5826 * (((((v8143 + (v7320 * ((((v8173 * v8081) * v8141) + ((((v8171 * v8081) * v8081) * v8141) * v8141)) - v1))) - v8138) - v8185) + (v8187 / v8167)) + v1));
                        let v8195 = ((v71 * v5830) * v5830) * v8109;
                        let v8198 = v8195 * v8109;
                        let v8237 = v8081 - ((v8161 / v8192) * (v1 + ((v8161 * ((((((v8195 * v8147) / v8167) - (v8198 / ((v8167 * v6653) * v8166))) - (v5826 * (((v8138 + v8143) - (((v71 * v7320) * v8141) * (v1 - (v8156 * (v2964 - ((v2976 * v8139) * v8141)))))) - (v8185 * (((v1 - (v8109 / v8166)) - v8136) + ((v8187 / v8166) * (v1 + (v1 / v6653)))))))) - ((v8198 * v8147) / (v8167 * v8166))) + v71)) / ((v71 * v8192) * v8192))));
                        v8238 = v8237;
                        v8406 = v8105;
                    }
                    let v8395: f64;
                    let v8405: f64;
                    if v6657 != 0.0 {
                        let v8245 = (v5796 * v6000) * (v1 + (((v5796 * (v1 - v6004)) * v5824) * v6643));
                        v8395 = v8245;
                        v8405 = v8406;
                    } else {
                        let v8262 = ((v5796 * v6000) * (v1 + (((v5796 * (v1 - v7320)) * v5824) * v6643))) * (v2485 * (((v8246 * (v5796 - v71)).tanh()) + ((v2964 * (v5796 + v71)).tanh())));
                        let v8265 = ((v8238 * v5729) - v7924) / v5729;
                        let v8266 = rspice_limited_exp(v8265);
                        let v8269 = ((v8262 * v5729) - v7924) / v5729;
                        let v8270 = if v8265 > v6689 { 1.0 } else { 0.0 };
                        let v8284: f64;
                        if v8270 != 0.0 {
                            v8284 = v8265;
                        } else {
                            let v8272 = if v8265 < v8271 { 1.0 } else { 0.0 };
                            let v8285: f64;
                            if v8272 != 0.0 {
                                let v8273 = v8265.exp();
                                v8285 = v8273;
                            } else {
                                let v8276 = (v1 + (v8265.exp())).ln();
                                v8285 = v8276;
                            }
                            v8284 = v8285;
                        }
                        let v8277 = if v8269 > v6689 { 1.0 } else { 0.0 };
                        let v8286: f64;
                        if v8277 != 0.0 {
                            v8286 = v8269;
                        } else {
                            let v8279 = if v8269 < v8278 { 1.0 } else { 0.0 };
                            let v8287: f64;
                            if v8279 != 0.0 {
                                let v8280 = v8269.exp();
                                v8287 = v8280;
                            } else {
                                let v8283 = (v1 + (v8269.exp())).ln();
                                v8287 = v8283;
                            }
                            v8286 = v8287;
                        }
                        let v8292 = -((v5997 / v5729) + ((v8284 - v8286) / v6653));
                        let v8293 = rspice_limited_exp(v8292);
                        let v8295 = rspice_limited_exp((-v8238));
                        let v8296 = v8238 * v8238;
                        let v8298 = v1 / (v8296 + v71);
                        let v8300 = rspice_limited_exp((v8238 - v7318));
                        let v8301 = v5796 - v8238;
                        let v8304 = v5797 + v8292;
                        let v8313 = v8298 * v8296;
                        let v8318 = ((v8301 * v8301) - (((v5830 * v5830) * v8304) * v8304)) - (v5826 * (((((v8295 - v8293) + v8238) + v8292) + v8300) - (v7320 * ((v8238 + v1) + v8313))));
                        let v8323 = v1 + v8266;
                        let v8324 = v6653 * v8323;
                        let v8328 = v71 * v8238;
                        let v8342 = v8266 / v8324;
                        let v8344 = v8266 * v8293;
                        let v8349 = (((((((v71 * v8266) * v8304) * v5830) * v5830) / v8324) - (v71 * v5796)) + v8328) - (v5826 * (((((v8300 + (v7320 * ((((v8330 * v8238) * v8298) + ((((v8328 * v8238) * v8238) * v8298) * v8298)) - v1))) - v8295) - v8342) + (v8344 / v8324)) + v1));
                        let v8352 = ((v71 * v5830) * v5830) * v8266;
                        let v8355 = v8352 * v8266;
                        let v8394 = v8238 - ((v8318 / v8349) * (v1 + ((v8318 * ((((((v8352 * v8304) / v8324) - (v8355 / ((v8324 * v6653) * v8323))) - (v5826 * (((v8295 + v8300) - (((v71 * v7320) * v8298) * (v1 - (v8313 * (v2964 - ((v2976 * v8296) * v8298)))))) - (v8342 * (((v1 - (v8266 / v8323)) - v8293) + ((v8344 / v8323) * (v1 + (v1 / v6653)))))))) - ((v8355 * v8304) / (v8324 * v8323))) + v71)) / ((v71 * v8349) * v8349))));
                        v8395 = v8394;
                        v8405 = v8262;
                    }
                    let v8396 = v8395 - v7131;
                    let v8397 = -v7321;
                    let v8398 = rspice_limited_exp(v8397);
                    let v8400 = if v8396 < v8399 { 1.0 } else { 0.0 };
                    let v8550: f64;
                    let v8552: f64;
                    if v8400 != 0.0 {
                        let v8402 = (v8238 * v5729) - v7924;
                        let v8403 = v8402 / v5729;
                        let v8404 = rspice_limited_exp(v8403);
                        let v8413 = ((v8405 * v5729) - v7924) / v5729;
                        let v8414 = if v8403 > v6689 { 1.0 } else { 0.0 };
                        let v8428: f64;
                        if v8414 != 0.0 {
                            v8428 = v8403;
                        } else {
                            let v8416 = if v8403 < v8415 { 1.0 } else { 0.0 };
                            let v8429: f64;
                            if v8416 != 0.0 {
                                let v8417 = v8403.exp();
                                v8429 = v8417;
                            } else {
                                let v8420 = (v1 + (v8403.exp())).ln();
                                v8429 = v8420;
                            }
                            v8428 = v8429;
                        }
                        let v8421 = if v8413 > v6689 { 1.0 } else { 0.0 };
                        let v8430: f64;
                        if v8421 != 0.0 {
                            v8430 = v8413;
                        } else {
                            let v8423 = if v8413 < v8422 { 1.0 } else { 0.0 };
                            let v8431: f64;
                            if v8423 != 0.0 {
                                let v8424 = v8413.exp();
                                v8431 = v8424;
                            } else {
                                let v8427 = (v1 + (v8413.exp())).ln();
                                v8431 = v8427;
                            }
                            v8430 = v8431;
                        }
                        let v8436 = -((v5997 / v5729) + ((v8428 - v8430) / v6653));
                        let v8440 = rspice_limited_exp((-v8238));
                        let v8443 = v1 / ((v8238 * v8238) + v71);
                        let v8446 = (v71 * v8402) / v5729;
                        let v8447 = rspice_limited_exp(v8446);
                        let v8449 = rspice_limited_exp((v8446 + v8436));
                        let v8450 = v71 * v8404;
                        let v8451 = v5797 + v8436;
                        let v8456 = v6653 * (v8404 + v1);
                        let v8460 = v71 * v8238;
                        let v8479 = v8404 / v8456;
                        let v8481 = (rspice_limited_exp((v8436 + v8403))) / v8456;
                        let v8486 = -(((((((v8450 * v8451) * v5830) * v5830) / v8456) - (v71 * v5796)) + v8460) - (v5826 * ((((((rspice_limited_exp(((v8238 - v7321) - v7318))) + ((rspice_limited_exp((v8397 - v7318))) * ((((v8467 * v8238) * v8443) + ((((v8460 * v8238) * v8238) * v8443) * v8443)) - v1))) - v8440) - v8479) + v8481) + v1)));
                        let v8489 = (v5826 * (v1 - v8398)) * v7152;
                        let v8491 = (v71 * v5830) * v5830;
                        let v8495 = v8491 * v8447;
                        let v8498 = (v1 + v8450) + v8447;
                        let v8499 = (v6653 * v6653) * v8498;
                        let v8525 = v6653 * v8498;
                        let v8542 = (v8486 * v8486) - (v71 * ((((((((v8491 * v8404) * v8451) / v8456) - (v8495 / v8499)) - (v5826 * (((((((v8440 + (rspice_limited_exp(((v8238 - v7318) - v7321)))) + ((rspice_limited_exp((v7319 - v7321))) * (((v8507 * v8443) + ((((v2908 * v8238) * v8238) * v8443) * v8443)) - (((((((v3003 * v8238) * v8238) * v8238) * v8238) * v8443) * v8443) * v8443)))) - v8479) + (v8447 / v8525)) + v8481) - (v8449 / v8525)) - (v8449 / v8499)))) - ((v8495 * v8451) / v8525)) + v71) * v8489));
                        let v8543 = if v8542 >= v0 { 1.0 } else { 0.0 };
                        let v8548: f64;
                        if v8543 != 0.0 {
                            let v8547 = v71 * (v8489 / (v8486 + (v8542.sqrt())));
                            v8548 = v8547;
                        } else {
                            v8548 = v8396;
                        }
                        let v8549 = v7131 + v8548;
                        v8550 = v8548;
                        v8552 = v8549;
                    } else {
                        v8550 = v8396;
                        v8552 = v8395;
                    }
                    let v8551 = v8550 * v5729;
                    let v8553 = v8552 * v8552;
                    let v8563 = (rspice_limited_exp((v8552 - v7318))) - (v7320 * ((v8552 + v1) + (v8553 / (v71 + v8553))));
                    let v8564 = v5796 - v8552;
                    let v8568 = (((v8564 * v8564) * v5827) - v8563) - v4710;
                    let v8575 = (v2485 * (v8568 + (((v8568 * v8568) + v8570).sqrt()))) + v4710;
                    let v8584 = ((v5826 * v8563) * v5729) / ((v5824 * ((v8575 + v8563).sqrt())) + (v5824 * (v8575.sqrt())));
                    let v8586 = v2485 * (v7131 + v8552);
                    let v8589 = (((rspice_limited_exp((-v8552))) * v7146).abs()).sqrt();
                    let v8591 = v2485 * (v7152 + v8563);
                    let v8598 = v8591 + (v8592 * ((v8550 * v8550) * (v8589 - (v71 * v5827))));
                    let v8599 = v5796 - v8586;
                    let v8602 = ((v8599 * v8599) * v5827) - v8598;
                    let v8605 = v5824 * ((v8598 + v8602).sqrt());
                    let v8606 = v8602 - v4710;
                    let v8613 = (v2485 * (v8606 + (((v8606 * v8606) + v8608).sqrt()))) + v4710;
                    let v8614 = v8613.sqrt();
                    let v8616 = if v8615 == v1 { 1.0 } else { 0.0 };
                    let v8691: f64;
                    let v8697: f64;
                    let v8699: f64;
                    let v8700: f64;
                    let v9570: f64;
                    if v8616 != 0.0 {
                        let v8622 = (((v71 * v13) * v13) * v5729) / ((v2 * v9) * v259);
                        let v8623 = v1 - v8589;
                        let v8630 = v1 / ((v1 + (v8622 * v8605)).sqrt());
                        let v8632 = v8630 / (v8630 + v1);
                        let v8639 = (v8622 * (((v8632 * v8632) * v8605) * v8605)) * (v8598 / (v8598 + v8613));
                        let v8644 = (v71 * (v8605 - v8639)) + (v5826 * (v8623 + v8598));
                        let v8647 = v8639 * (v8639 - (v71 * v8605));
                        let v8656 = (v8647 * v8644) / ((v8644 * v8644) - ((v1 - (v2485 * (v5826 * (v8589 + v8598)))) * v8647));
                        let v8658 = rspice_limited_exp(v8656);
                        let v8660 = v8598 * v8658;
                        let v8662 = (v5796 - (v8586 + v8656)) + v8656;
                        let v8666 = ((v8662 * v8662) * v5827) - (v8660 / v8658);
                        let v8669 = v5824 * ((v8660 + v8666).sqrt());
                        let v8681 = (((v8550 * v8658) * ((v8623 + (v71 * (v8605 * v5827))) + v8591)) / (((v1 - (v8589 / v8658)) + (v71 * ((v8669 * v8630) * v5827))) + (v8658 * v8591))) * v5729;
                        let v8682 = v8666 - v4710;
                        let v8690 = ((v2485 * (v8682 + (((v8682 * v8682) + v8684).sqrt()))) + v4710).sqrt();
                        v8691 = v8681;
                        v8697 = v8660;
                        v8699 = v8669;
                        v8700 = v8690;
                        v9570 = v8630;
                    } else {
                        v8691 = v8551;
                        v8697 = v8598;
                        v8699 = v8605;
                        v8700 = v8614;
                        v9570 = v1;
                    }
                    let v8694 = if (v8691.abs()) > v8693 { 1.0 } else { 0.0 };
                    let v9021: f64;
                    if v8694 != 0.0 {
                        let v8696 = (v7174 - v8584) / v8691;
                        v9021 = v8696;
                    } else {
                        v9021 = v0;
                    }
                    let v8701 = v5824 * v8700;
                    let v8704 = v5729 * ((v5826 * v8697) / (v8699 + v8701));
                    let v8705 = v8701 * v5729;
                    let v8706 = v8699 * v5729;
                    let v8721 = v1 + ((v7192 * ((v7177 * (v8705 + (v4728 * v8704))).powf(v4768))) + (v7195 / (rspice_limited_exp((v7181 * ((if (v2485 * (v1 + (v8704 / v8705))) >= v4546 { (v2485 * (v1 + (v8704 / v8705))) } else { v4546 }).ln()))))));
                    let v8723 = v8721 - v1;
                    let v8729 = v2485 * ((v8721 + v1) + (((v8723 * v8723) + v8725).sqrt()));
                    let v8732 = (v7235 / (v7236 / v8729)) * v73;
                    let v8733 = if v899 > v0 { 1.0 } else { 0.0 };
                    let v8758: f64;
                    if v8733 != 0.0 {
                        let v8736 = v1 + ((v899 * v8704) / v8732);
                        v8758 = v8736;
                    } else {
                        let v8740 = v1 / (v1 - ((v899 * v8704) / v8732));
                        v8758 = v8740;
                    }
                    let v8742 = v5528 - v7315;
                    let v8743 = v8704 + v7240;
                    let v8744 = if v8741 > v0 { 1.0 } else { 0.0 };
                    let v8806: f64;
                    if v8744 != 0.0 {
                        let v8748 = v1 + (v839 * v5546);
                        let v8762 = v1 + (v8742 / ((((v8743 / v8741) * (v8743 / (v7307 + v8743))) * v8758) * (v1 / (v2485 * (v8748 + (((v8748 * v8748) + v8750).sqrt()))))));
                        v8806 = v8762;
                    } else {
                        v8806 = v1;
                    }
                    let v8763 = if v2495 <= v0 { 1.0 } else { 0.0 };
                    let v8778: f64;
                    if v8763 != 0.0 {
                        v8778 = v1;
                    } else {
                        let v8768 = v1 / (v1 + ((v2495 * (v73.sqrt())) / v8743));
                        v8778 = v8768;
                    }
                    let v8769 = v7307 + v8732;
                    let v8771 = if v8770 > v0 { 1.0 } else { 0.0 };
                    let v8807: f64;
                    if v8771 != 0.0 {
                        let v8773 = if v8772 < v0 { 1.0 } else { 0.0 };
                        let v8785: f64;
                        if v8773 != 0.0 {
                            let v8779 = (v8770 / (v1 - ((v8772 * v8704) / v8732))) / v8778;
                            v8785 = v8779;
                        } else {
                            let v8784 = (v8770 * (v1 + ((v8772 * v8704) / v8732))) / v8778;
                            v8785 = v8784;
                        }
                        let v8792 = v1 + (v8785 * ((if (v1 + ((v8742 / v8785) / v8769)) >= v4546 { (v1 + ((v8742 / v8785) / v8769)) } else { v4546 }).ln()));
                        v8807 = v8792;
                    } else {
                        let v8793 = if v8772 < v0 { 1.0 } else { 0.0 };
                        let v8804: f64;
                        if v8793 != 0.0 {
                            let v8798 = (v8770 / (v1 - ((v8772 * v8704) / v8732))) / v8778;
                            v8804 = v8798;
                        } else {
                            let v8803 = (v8770 * (v1 + ((v8772 * v8704) / v8732))) / v8778;
                            v8804 = v8803;
                        }
                        let v8805 = v1 + v8804;
                        v8807 = v8805;
                    }
                    let v8808 = v8806 * v8807;
                    let v8810 = rspice_limited_exp((v879 * v5528));
                    let v8811 = if v869 > v0 { 1.0 } else { 0.0 };
                    let v8820: f64;
                    if v8811 != 0.0 {
                        let v8818 = ((v1 + ((v1 + (v8812 * v73)) * v8810)) / v869) * v8778;
                        v8820 = v8818;
                    } else {
                        v8820 = v8819;
                    }
                    let v8823 = v8808 * (v1 + (v8742 / v8820));
                    let v8824 = if v859 > v0 { 1.0 } else { 0.0 };
                    let v8834: f64;
                    if v8824 != 0.0 {
                        let v8825 = v849 * v4704;
                        let v8827 = if v8742 > (v8825 / v5531) { 1.0 } else { 0.0 };
                        let v8835: f64;
                        if v8827 != 0.0 {
                            let v8831 = (v73 * (rspice_limited_exp((v8825 / v8742)))) / v859;
                            v8835 = v8831;
                        } else {
                            let v8833 = (v8819 * v73) / v859;
                            v8835 = v8833;
                        }
                        v8834 = v8835;
                    } else {
                        v8834 = v8819;
                    }
                    let v8838 = v8823 * (v1 + (v8742 / v8834));
                    let v8839 = if v1779 < v0 { 1.0 } else { 0.0 };
                    let v8845: f64;
                    if v8839 != 0.0 {
                        let v8842 = v1 / (v1 - (v1779 * v5546));
                        v8845 = v8842;
                    } else {
                        let v8844 = v1 + (v1779 * v5546);
                        v8845 = v8844;
                    }
                    let v8846 = v8704 * v8845;
                    let v8849 = v4799 * (v8846 / (v4799 + v8846));
                    let v8851 = v1 / v8850;
                    let v8861 = v8860 * (((v1 + ((v5528 - v8691) * v8851)) / (v1 + ((v7315 - v8691) * v8851))).ln());
                    let v8865 = v1 / ((v1 + v8861) + (v8861 * v8861));
                    let v8866 = v8729 * v8865;
                    let v8867 = if v2543 < v0 { 1.0 } else { 0.0 };
                    let v8873: f64;
                    if v8867 != 0.0 {
                        let v8870 = v1 / (v1 - (v2543 * v8849));
                        v8873 = v8870;
                    } else {
                        let v8872 = v1 + (v2543 * v8849);
                        v8873 = v8872;
                    }
                    let v8875 = v929 * (v8873 / v8866);
                    let v8878 = ((v8875 * v8875) * v8691) * v8691;
                    let v8880 = if v4 == v8879 { 1.0 } else { 0.0 };
                    let v8884: f64;
                    if v8880 != 0.0 {
                        let v8883 = v8878 / (v1 + (v8875 * v8691));
                        v8884 = v8883;
                    } else {
                        v8884 = v8878;
                    }
                    let v8890 = v2485 * (v8866 * (v1 + ((v1 + (v71 * v8884)).sqrt())));
                    let v8891 = v1 / v8890;
                    let v8892 = v7174 + v8584;
                    let v9052: f64;
                    let v12937: f64;
                    let v19875: f64;
                    let v19890: f64;
                    if v2834 != 0.0 {
                        let v8894 = v5511 - v8893;
                        let v8904 = (v1 / (v1 + (v749 * (v2485 * (v8894 + (((v8894 * v8894) + v4979).sqrt())))))) + (v2812 * v5502);
                        let v8918 = v4795 * (v7138 + ((v8910 + (v8912 * (v2485 * (v8904 + (((v8904 * v8904) + v4979).sqrt()))))) * v7211));
                        let v8919 = v5510 - v8893;
                        let v8929 = (v1 / (v1 + (v749 * (v2485 * (v8919 + (((v8919 * v8919) + v4979).sqrt())))))) + (v2812 * v5499);
                        let v8943 = v4795 * (v7135 + ((v8935 + (v8937 * (v2485 * (v8929 + (((v8929 * v8929) + v4979).sqrt()))))) * v7211));
                        v9052 = v1;
                        v12937 = v0;
                        v19875 = v8943;
                        v19890 = v8918;
                    } else {
                        let v8948 = (v1 / (v1 + (v749 * v8892))) + (v2812 * v5784);
                        let v8955 = v7221 + (v7223 * (v2485 * (v8948 + (((v8948 * v8948) + v4979).sqrt()))));
                        let v8958 = ((v4795 * v8955) * v7211) * v32;
                        let v8963 = ((((v7236 / v8890) * v13) * v83) / v73) * v8892;
                        let v8965 = v1 + (v8963 * v8958);
                        let v8966 = if v2833 == v71 { 1.0 } else { 0.0 };
                        let v9053: f64;
                        let v12938: f64;
                        let v19876: f64;
                        let v19891: f64;
                        if v8966 != 0.0 {
                            let v8971 = v4795 * ((v7138 + ((v8955 * v7211) * v32)) + v7135);
                            let v8973 = v1 + (v8963 * v8971);
                            v9053 = v8973;
                            v12938 = v8971;
                            v19876 = v0;
                            v19891 = v0;
                        } else {
                            v9053 = v8965;
                            v12938 = v8958;
                            v19876 = v7135;
                            v19891 = v7138;
                        }
                        v9052 = v9053;
                        v12937 = v12938;
                        v19875 = v19876;
                        v19890 = v19891;
                    }
                    let v8975 = (v71 * v5728) * v4613;
                    let v8979 = v7174 - v8584;
                    let v8983 = ((((v4846 + (v4856 / (v8892 + v8975))) * v8979) * v8979) + v1) - v4710;
                    let v8995 = v2485 * (v1 + ((v1 + (v8984 + (v2485 * (v8983 + (((v8983 * v8983) + v8986).sqrt()))))).sqrt()));
                    let v8997 = v8995 - v1;
                    let v9005 = (v2485 * ((v8995 + v1) - (((v8997 * v8997) + v8999).sqrt()))) + v9004;
                    let v9007 = v8979 / (v8892 + v4887);
                    let v9010 = v1 + ((v4876 * v9007) * v9007);
                    let v9019 = rspice_limited_exp((-(v4897 / (((if v0 >= (v4907 + ((v4917 * v8979) * v8979)) { v0 } else { (v4907 + ((v4917 * v8979) * v8979)) }) * v8892) + v8975))));
                    let v9020 = v8866 * v8891;
                    let v9027 = (v9021 * (v1 + (v2485 * ((v8884 * v9020) * v9020)))) - v4710;
                    let v9036 = v8704 + (v5729 * v9021);
                    let v9039 = (v9036 / ((v2485 * (v9027 + (((v9027 * v9027) + v9029).sqrt()))) + v4710)) * (v8866 / v8890);
                    v9044 = v9036;
                    v9046 = v8691;
                    v9048 = v8865;
                    v9049 = v8891;
                    v9051 = v9052;
                    v9056 = v8838;
                    v9058 = v9005;
                    v9060 = v9010;
                    v9062 = v9019;
                    v9064 = v8729;
                    v9065 = v8890;
                    v9073 = v8892;
                    v9205 = v8742;
                    v9557 = v8706;
                    v9558 = v9039;
                    v9563 = v8704;
                    v9564 = v9021;
                    v9569 = v9570;
                    v12272 = v8405;
                    v12549 = v8552;
                    v12664 = v7315;
                    v12784 = v8584;
                    v12830 = v7174;
                    v12936 = v12937;
                    v12969 = v7307;
                    v19874 = v19875;
                    v19889 = v19890;
                }
                let v9063 = (((((((((v32 * v7236) * (v83 / v73)) * v13) * v9044) * v9046) * ((v9048 * v9049) / v9051)) * v9056) / v9058) * v9060) * v9062;
                let v9068 = v7236 / ((v9064 * v9065) * v9051);
                let v9069 = if v4539 > v1 { 1.0 } else { 0.0 };
                let v19852: f64;
                let v19858: f64;
                if v9069 != 0.0 {
                    let v9084 = (v9081 * v32) * ((((((v9075 * v4613) * v9068) * v83) / v73) * v13) + ((((v9068 * v83) / v73) * v13) * v9073));
                    let v9085 = if v4539 == v71 { 1.0 } else { 0.0 };
                    let v19853: f64;
                    let v19859: f64;
                    if v9085 != 0.0 {
                        let v9088 = if (v1 / v9086) < v4508 { 1.0 } else { 0.0 };
                        let v9090: f64;
                        if v9088 != 0.0 {
                            let v9089 = v1 / v4508;
                            v9090 = v9089;
                        } else {
                            v9090 = v9086;
                        }
                        let v9093 = (v9090 * v9084) / (v9090 + v9084);
                        v19853 = v9093;
                        v19859 = v9090;
                    } else {
                        v19853 = v9084;
                        v19859 = v9086;
                    }
                    v19852 = v19853;
                    v19858 = v19859;
                } else {
                    v19852 = v0;
                    v19858 = v9086;
                }
                let v9095 = v4613 * v9094;
                let v9097 = rspice_limited_exp((v5507 / v9095));
                let v9099 = rspice_limited_exp((v5509 / v9095));
                let v9102 = (v9100 / v4613) * v4706;
                let v9103 = if v1069 == v0 { 1.0 } else { 0.0 };
                if v9103 != 0.0 {
                } else {
                }
                let v9104 = if v1079 == v0 { 1.0 } else { 0.0 };
                if v9104 != 0.0 {
                } else {
                }
                let v9105 = if v1109 == v0 { 1.0 } else { 0.0 };
                if v9105 != 0.0 {
                } else {
                    let v9107 = if (v1169 - v5507) < v4710 { 1.0 } else { 0.0 };
                    if v9107 != 0.0 {
                    } else {
                    }
                }
                let v9108 = if v1119 == v0 { 1.0 } else { 0.0 };
                if v9108 != 0.0 {
                } else {
                    let v9110 = if (v1179 - v5509) < v4710 { 1.0 } else { 0.0 };
                    if v9110 != 0.0 {
                    } else {
                    }
                }
                let v9111 = v4565 * v5613;
                let v9114 = if (if v1249 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1259 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9381: f64;
                let v9542: f64;
                if v9114 != 0.0 {
                    v9381 = v0;
                    v9542 = v0;
                } else {
                    let v9117 = rspice_limited_exp(((v1229 * v9102) / v9094));
                    let v9119 = v1249 * v9117;
                    let v9120 = v1219 * v9117;
                    let v9121 = v1259 * v9117;
                    let v9122 = v9097 - v1;
                    let v9123 = (v1209 * v9117) * v9122;
                    let v9124 = if v9123 < v7159 { 1.0 } else { 0.0 };
                    let v9140: f64;
                    let v9162: f64;
                    if v9124 != 0.0 {
                        v9140 = v1;
                        v9162 = v0;
                    } else {
                        let v9127 = v1 / ((v1 + v9123).sqrt());
                        v9140 = v9127;
                        v9162 = v9123;
                    }
                    let v9128 = v9099 - v1;
                    let v9129 = v9120 * v9128;
                    let v9130 = if v9129 < v7159 { 1.0 } else { 0.0 };
                    let v9142: f64;
                    let v9163: f64;
                    if v9130 != 0.0 {
                        v9142 = v1;
                        v9163 = v0;
                    } else {
                        let v9133 = v1 / ((v1 + v9129).sqrt());
                        v9142 = v9133;
                        v9163 = v9129;
                    }
                    let v9146 = v1 + (v9143 * ((v1269 * ((v1 / v73) + (v1 / v9134))).powf(v1279)));
                    let v9149 = (((v9111 * v9119) * v9146) * v9122) * v9140;
                    let v9152 = (((v9111 * v9121) * v9146) * v9128) * v9142;
                    let v9154 = v1189 + (v1199 * v73);
                    let v9155 = if v9154 < v1 { 1.0 } else { 0.0 };
                    let v9159: f64;
                    if v9155 != 0.0 {
                        v9159 = v1;
                    } else {
                        v9159 = v9154;
                    }
                    let v9157 = if v9156 == v1 { 1.0 } else { 0.0 };
                    if v9157 != 0.0 {
                    } else {
                        let v9161 = v1 + ((v5507 + v5509) / v9159);
                        let v9171 = if ((v9161 + (((v9161 * v9161) + (v2976 * (v9162 + v9163))).sqrt())) / v71) < v5425 { 1.0 } else { 0.0 };
                        if v9171 != 0.0 {
                        } else {
                        }
                    }
                    v9381 = v9149;
                    v9542 = v9152;
                }
                let v9174 = if (if v1129 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1139 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v9174 != 0.0 {
                } else {
                    let v9176 = if (v1149 - v5507) < v4710 { 1.0 } else { 0.0 };
                    if v9176 != 0.0 {
                    } else {
                    }
                    let v9178 = if (v1159 - v5509) < v4710 { 1.0 } else { 0.0 };
                    if v9178 != 0.0 {
                    } else {
                    }
                }
                let v9180 = if v9179 == v0 { 1.0 } else { 0.0 };
                if v9180 != 0.0 {
                    let v9184 = if (if (if v2573 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4860 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v2867 != 0.0 { 1.0 } else { 0.0 };
                    if v9184 != 0.0 {
                    } else {
                        let v9185 = if v1349 != v0 { 1.0 } else { 0.0 };
                        if v9185 != 0.0 {
                        } else {
                        }
                    }
                    let v9190 = if (if (if v2580 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4862 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v2868 != 0.0 { 1.0 } else { 0.0 };
                    if v9190 != 0.0 {
                    } else {
                        let v9191 = if v1389 != v0 { 1.0 } else { 0.0 };
                        if v9191 != 0.0 {
                        } else {
                        }
                    }
                } else {
                    let v9194 = if (if v2573 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4860 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v9194 != 0.0 {
                    } else {
                        let v9195 = if v1409 != v0 { 1.0 } else { 0.0 };
                        if v9195 != 0.0 {
                        } else {
                        }
                    }
                    let v9198 = if (if v2580 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4862 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v9198 != 0.0 {
                    } else {
                        let v9199 = if v1399 != v0 { 1.0 } else { 0.0 };
                        if v9199 != 0.0 {
                        } else {
                        }
                    }
                }
                let v9201 = if v9200 == v0 { 1.0 } else { 0.0 };
                if v9201 != 0.0 {
                    let v9204 = if (if v2566 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4858 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v9204 != 0.0 {
                    } else {
                        let v9207 = if v9205 > (v4858 / v5531) { 1.0 } else { 0.0 };
                        if v9207 != 0.0 {
                        } else {
                        }
                    }
                } else {
                    let v9208 = if v9200 == v1 { 1.0 } else { 0.0 };
                    if v9208 != 0.0 {
                        let v9215 = if (if v2566 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if (if (if v1019 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1009 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4858 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if v9215 != 0.0 {
                        } else {
                        }
                    } else {
                        let v9223 = if (if v2566 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if (if (if v1019 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1009 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4858 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if v9223 != 0.0 {
                        } else {
                        }
                        let v9227 = v1029 * (v1 + (v9224 * v4706));
                        let v9228 = if v9216 > v0 { 1.0 } else { 0.0 };
                        let v9231: f64;
                        if v9228 != 0.0 {
                            let v9229 = v9227 - v5509;
                            v9231 = v9229;
                        } else {
                            let v9230 = v9227 - v5507;
                            v9231 = v9230;
                        }
                        let v9232 = if v9231 > v0 { 1.0 } else { 0.0 };
                        if v9232 != 0.0 {
                        } else {
                        }
                    }
                }
                let v9234 = v4929 * v9233;
                let v9238 = v4948 * v9235;
                let v9240 = (v4967 * v125) * v32;
                let v9242 = -v9241;
                let v9243 = v5425.powf(v9242);
                let v9244 = if v9241 == v1 { 1.0 } else { 0.0 };
                let v9311: f64;
                if v9244 != 0.0 {
                    v9311 = v9245;
                } else {
                    let v9253 = (v1 / (v1 - v9241)) * (v1 - (((v5443 * v9241) * (v1 + v9241)) * v9243));
                    v9311 = v9253;
                }
                let v9255 = -v9254;
                let v9256 = v5425.powf(v9255);
                let v9257 = if v9254 == v1 { 1.0 } else { 0.0 };
                let v9344: f64;
                if v9257 != 0.0 {
                    v9344 = v9258;
                } else {
                    let v9266 = (v1 / (v1 - v9254)) * (v1 - (((v5443 * v9254) * (v1 + v9254)) * v9256));
                    v9344 = v9266;
                }
                let v9268 = -v9267;
                let v9269 = v5425.powf(v9268);
                let v9270 = if v9267 == v1 { 1.0 } else { 0.0 };
                let v9377: f64;
                if v9270 != 0.0 {
                    v9377 = v9271;
                } else {
                    let v9279 = (v1 / (v1 - v9267)) * (v1 - (((v5443 * v9267) * (v1 + v9267)) * v9269));
                    v9377 = v9279;
                }
                let v9280 = if v9234 > v0 { 1.0 } else { 0.0 };
                let v9384: f64;
                if v9280 != 0.0 {
                    let v9281 = v5507 / v4987;
                    let v9283 = if v9281 < v9282 { 1.0 } else { 0.0 };
                    let v9385: f64;
                    if v9283 != 0.0 {
                        let v9284 = v1 - v9281;
                        let v9285 = if v9241 != v1 { 1.0 } else { 0.0 };
                        let v9386: f64;
                        if v9285 != 0.0 {
                            let v9286 = if v9241 == v2485 { 1.0 } else { 0.0 };
                            let v9293: f64;
                            if v9286 != 0.0 {
                                let v9288 = v1 / (v9284.sqrt());
                                v9293 = v9288;
                            } else {
                                let v9291 = rspice_limited_exp((v9242 * (v9284.ln())));
                                v9293 = v9291;
                            }
                            let v9298 = ((v4987 * v9234) * (v1 - (v9284 * v9293))) / (v1 - v9241);
                            v9386 = v9298;
                        } else {
                            let v9302 = (v4987 * v9234) * (-(v9284.ln()));
                            v9386 = v9302;
                        }
                        v9385 = v9386;
                    } else {
                        let v9303 = v9281 - v1;
                        let v9313 = (v4987 * v9234) * (((v9243 * v9303) * (((v2964 * v9241) * v9303) + (v1 + v9241))) + v9311);
                        v9385 = v9313;
                    }
                    v9384 = v9385;
                } else {
                    v9384 = v0;
                }
                let v9314 = if v9238 > v0 { 1.0 } else { 0.0 };
                let v9387: f64;
                if v9314 != 0.0 {
                    let v9315 = v5507 / v5009;
                    let v9316 = if v9315 < v9282 { 1.0 } else { 0.0 };
                    let v9388: f64;
                    if v9316 != 0.0 {
                        let v9317 = v1 - v9315;
                        let v9318 = if v9254 != v1 { 1.0 } else { 0.0 };
                        let v9389: f64;
                        if v9318 != 0.0 {
                            let v9319 = if v9254 == v2485 { 1.0 } else { 0.0 };
                            let v9326: f64;
                            if v9319 != 0.0 {
                                let v9321 = v1 / (v9317.sqrt());
                                v9326 = v9321;
                            } else {
                                let v9324 = rspice_limited_exp((v9255 * (v9317.ln())));
                                v9326 = v9324;
                            }
                            let v9331 = ((v5009 * v9238) * (v1 - (v9317 * v9326))) / (v1 - v9254);
                            v9389 = v9331;
                        } else {
                            let v9335 = (v5009 * v9238) * (-(v9317.ln()));
                            v9389 = v9335;
                        }
                        v9388 = v9389;
                    } else {
                        let v9336 = v9315 - v1;
                        let v9346 = (v5009 * v9238) * (((v9256 * v9336) * (((v2964 * v9254) * v9336) + (v1 + v9254))) + v9344);
                        v9388 = v9346;
                    }
                    v9387 = v9388;
                } else {
                    v9387 = v0;
                }
                let v9347 = if v9240 > v0 { 1.0 } else { 0.0 };
                let v9391: f64;
                if v9347 != 0.0 {
                    let v9348 = v5507 / v5031;
                    let v9349 = if v9348 < v9282 { 1.0 } else { 0.0 };
                    let v9392: f64;
                    if v9349 != 0.0 {
                        let v9350 = v1 - v9348;
                        let v9351 = if v9267 != v1 { 1.0 } else { 0.0 };
                        let v9393: f64;
                        if v9351 != 0.0 {
                            let v9352 = if v9267 == v2485 { 1.0 } else { 0.0 };
                            let v9359: f64;
                            if v9352 != 0.0 {
                                let v9354 = v1 / (v9350.sqrt());
                                v9359 = v9354;
                            } else {
                                let v9357 = rspice_limited_exp((v9268 * (v9350.ln())));
                                v9359 = v9357;
                            }
                            let v9364 = ((v5031 * v9240) * (v1 - (v9350 * v9359))) / (v1 - v9267);
                            v9393 = v9364;
                        } else {
                            let v9368 = (v5031 * v9240) * (-(v9350.ln()));
                            v9393 = v9368;
                        }
                        v9392 = v9393;
                    } else {
                        let v9369 = v9348 - v1;
                        let v9379 = (v5031 * v9240) * (((v9269 * v9369) * (((v2964 * v9267) * v9369) + (v1 + v9267))) + v9377);
                        v9392 = v9379;
                    }
                    v9391 = v9392;
                } else {
                    v9391 = v0;
                }
                let v9395 = ((v9384 + v9387) + v9391) + ((v9380 * v9381) * v32);
                let v9397 = v4936 * v9396;
                let v9401 = v4955 * v9398;
                let v9403 = (v4974 * v125) * v32;
                let v9405 = -v9404;
                let v9406 = v5425.powf(v9405);
                let v9407 = if v9404 == v1 { 1.0 } else { 0.0 };
                let v9473: f64;
                if v9407 != 0.0 {
                    v9473 = v9408;
                } else {
                    let v9416 = (v1 / (v1 - v9404)) * (v1 - (((v5443 * v9404) * (v1 + v9404)) * v9406));
                    v9473 = v9416;
                }
                let v9418 = -v9417;
                let v9419 = v5425.powf(v9418);
                let v9420 = if v9417 == v1 { 1.0 } else { 0.0 };
                let v9506: f64;
                if v9420 != 0.0 {
                    v9506 = v9421;
                } else {
                    let v9429 = (v1 / (v1 - v9417)) * (v1 - (((v5443 * v9417) * (v1 + v9417)) * v9419));
                    v9506 = v9429;
                }
                let v9431 = -v9430;
                let v9432 = v5425.powf(v9431);
                let v9433 = if v9430 == v1 { 1.0 } else { 0.0 };
                let v9539: f64;
                if v9433 != 0.0 {
                    v9539 = v9434;
                } else {
                    let v9442 = (v1 / (v1 - v9430)) * (v1 - (((v5443 * v9430) * (v1 + v9430)) * v9432));
                    v9539 = v9442;
                }
                let v9443 = if v9397 > v0 { 1.0 } else { 0.0 };
                let v9545: f64;
                if v9443 != 0.0 {
                    let v9444 = v5509 / v4997;
                    let v9445 = if v9444 < v9282 { 1.0 } else { 0.0 };
                    let v9546: f64;
                    if v9445 != 0.0 {
                        let v9446 = v1 - v9444;
                        let v9447 = if v9404 != v1 { 1.0 } else { 0.0 };
                        let v9547: f64;
                        if v9447 != 0.0 {
                            let v9448 = if v9404 == v2485 { 1.0 } else { 0.0 };
                            let v9455: f64;
                            if v9448 != 0.0 {
                                let v9450 = v1 / (v9446.sqrt());
                                v9455 = v9450;
                            } else {
                                let v9453 = rspice_limited_exp((v9405 * (v9446.ln())));
                                v9455 = v9453;
                            }
                            let v9460 = ((v4997 * v9397) * (v1 - (v9446 * v9455))) / (v1 - v9404);
                            v9547 = v9460;
                        } else {
                            let v9464 = (v4997 * v9397) * (-(v9446.ln()));
                            v9547 = v9464;
                        }
                        v9546 = v9547;
                    } else {
                        let v9465 = v9444 - v1;
                        let v9475 = (v4997 * v9397) * (((v9406 * v9465) * (((v2964 * v9404) * v9465) + (v1 + v9404))) + v9473);
                        v9546 = v9475;
                    }
                    v9545 = v9546;
                } else {
                    v9545 = v0;
                }
                let v9476 = if v9401 > v0 { 1.0 } else { 0.0 };
                let v9548: f64;
                if v9476 != 0.0 {
                    let v9477 = v5509 / v5019;
                    let v9478 = if v9477 < v9282 { 1.0 } else { 0.0 };
                    let v9549: f64;
                    if v9478 != 0.0 {
                        let v9479 = v1 - v9477;
                        let v9480 = if v9417 != v1 { 1.0 } else { 0.0 };
                        let v9550: f64;
                        if v9480 != 0.0 {
                            let v9481 = if v9417 == v2485 { 1.0 } else { 0.0 };
                            let v9488: f64;
                            if v9481 != 0.0 {
                                let v9483 = v1 / (v9479.sqrt());
                                v9488 = v9483;
                            } else {
                                let v9486 = rspice_limited_exp((v9418 * (v9479.ln())));
                                v9488 = v9486;
                            }
                            let v9493 = ((v5019 * v9401) * (v1 - (v9479 * v9488))) / (v1 - v9417);
                            v9550 = v9493;
                        } else {
                            let v9497 = (v5019 * v9401) * (-(v9479.ln()));
                            v9550 = v9497;
                        }
                        v9549 = v9550;
                    } else {
                        let v9498 = v9477 - v1;
                        let v9508 = (v5019 * v9401) * (((v9419 * v9498) * (((v2964 * v9417) * v9498) + (v1 + v9417))) + v9506);
                        v9549 = v9508;
                    }
                    v9548 = v9549;
                } else {
                    v9548 = v0;
                }
                let v9509 = if v9403 > v0 { 1.0 } else { 0.0 };
                let v9552: f64;
                if v9509 != 0.0 {
                    let v9510 = v5509 / v5041;
                    let v9511 = if v9510 < v9282 { 1.0 } else { 0.0 };
                    let v9553: f64;
                    if v9511 != 0.0 {
                        let v9512 = v1 - v9510;
                        let v9513 = if v9430 != v1 { 1.0 } else { 0.0 };
                        let v9554: f64;
                        if v9513 != 0.0 {
                            let v9514 = if v9430 == v2485 { 1.0 } else { 0.0 };
                            let v9521: f64;
                            if v9514 != 0.0 {
                                let v9516 = v1 / (v9512.sqrt());
                                v9521 = v9516;
                            } else {
                                let v9519 = rspice_limited_exp((v9431 * (v9512.ln())));
                                v9521 = v9519;
                            }
                            let v9526 = ((v5041 * v9403) * (v1 - (v9512 * v9521))) / (v1 - v9430);
                            v9554 = v9526;
                        } else {
                            let v9530 = (v5041 * v9403) * (-(v9512.ln()));
                            v9554 = v9530;
                        }
                        v9553 = v9554;
                    } else {
                        let v9531 = v9510 - v1;
                        let v9541 = (v5041 * v9403) * (((v9432 * v9531) * (((v2964 * v9430) * v9531) + (v1 + v9430))) + v9539);
                        v9553 = v9541;
                    }
                    v9552 = v9553;
                } else {
                    v9552 = v0;
                }
                let v9556 = ((v9545 + v9548) + v9552) + ((v9380 * v9542) * v32);
                let v9599: f64;
                let v9606: f64;
                let v9607: f64;
                if v7132 != 0.0 {
                    v9599 = v9557;
                    v9606 = v0;
                    v9607 = v0;
                } else {
                    let v9560 = v2485 * (v9046 / v9558);
                    let v9565 = v9564 * v9046;
                    let v9568 = (v1 - v9048) * (v9563 - (v2485 * v9565));
                    let v9578 = v9557 + (v2485 * ((v9569 * v9046) * ((((v9560 * v9048) * v4724) - v1) + v9048)));
                    let v9579 = v9565 * v6039;
                    let v9595 = v2485 * (((v9048 * v9048) * (v9563 - (v9579 * ((v1 - v9560) - (v9586 * (v9560 * v9560)))))) + (v9568 * (v1 + v9048)));
                    let v9596 = v9578 - ((v9048 * (v9563 + (v9579 * v9560))) + v9568);
                    let v9598 = (v9578 - v9596) - v9595;
                    v9599 = v9596;
                    v9606 = v9598;
                    v9607 = v9595;
                }
                let v9613 = ((v9606 + v9607) + (v9609 * (v2485 * (v9599 + (((v9599 * v9599) + v9601).sqrt()))))) / v9612;
                let v9639 = ((v32 * v111) * v107) + v9638;
                let v9641 = v11 / v9630;
                let v9643 = ((-v9639) * v9641) * v9599;
                let v9644 = v9639 * (v9629 / (((v9630 * v18) / v10) + (((v9625 * v9626) / (v1 + ((v2485 * (v9613 + (((v9613 * v9613) + v9615).sqrt()))).powf((v9620 * v9621))))) / v14)));
                let v9645 = if v9216 > v0 { 1.0 } else { 0.0 };
                let v9652: f64;
                let v9653: f64;
                if v9645 != 0.0 {
                    let v9646 = -v9644;
                    let v9647 = v9646 * v9606;
                    let v9648 = v9646 * v9607;
                    v9652 = v9647;
                    v9653 = v9648;
                } else {
                    let v9649 = -v9644;
                    let v9650 = v9649 * v9607;
                    let v9651 = v9649 * v9606;
                    v9652 = v9650;
                    v9653 = v9651;
                }
                let v9655 = if v9654 == 0.0 { 1.0 } else { 0.0 };
                if v9655 != 0.0 {
                } else {
                }
                let v9656 = v111 / v4564;
                let v9658 = if v9657 == v0 { 1.0 } else { 0.0 };
                if v9658 != 0.0 {
                } else {
                }
                let v9663 = (v107 - v9659) + (v71 * v9661);
                let v9664 = if v2119 > v0 { 1.0 } else { 0.0 };
                let v9679: f64;
                if v9664 != 0.0 {
                    let v9669 = (v5519 * v4613) * ((if (v2291 / v2119) >= v4546 { (v2291 / v2119) } else { v4546 }).ln());
                    v9679 = v9669;
                } else {
                    let v9677 = (v5519 * v4613) * ((if ((((-v2291) * v2119) / v4637) / v4637) >= v4546 { ((((-v2291) * v2119) / v4637) / v4637) } else { v4546 }).ln());
                    v9679 = v9677;
                }
                let v9693 = (((v2129 * v9683) * (v9681 / v5623)) * (((v9656 * v32) * v9663) + v9688)) * ((v9678 - v9679) - v9691);
                let v9695 = if v9694 != v0 { 1.0 } else { 0.0 };
                if v9695 != 0.0 {
                    let v9697 = if (v5246 - v26) > v0 { 1.0 } else { 0.0 };
                    if v9697 != 0.0 {
                    } else {
                    }
                    let v9699 = if (v5266 - v26) > v0 { 1.0 } else { 0.0 };
                    if v9699 != 0.0 {
                    } else {
                    }
                    let v9700 = if v2910 != v0 { 1.0 } else { 0.0 };
                    if v9700 != 0.0 {
                    } else {
                    }
                } else {
                }
                let v9701 = if v4673 == v1 { 1.0 } else { 0.0 };
                let v12527: f64;
                let v12530: f64;
                let v12546: f64;
                let v12547: f64;
                let v12662: f64;
                let v12782: f64;
                let v12786: f64;
                let v12789: f64;
                let v12828: f64;
                let v12922: f64;
                let v12949: f64;
                let v12967: f64;
                let v13406: f64;
                let v13413: f64;
                let v15770: f64;
                let v19872: f64;
                let v19887: f64;
                if v9701 != 0.0 {
                    let v9705 = v5547 * v4614;
                    let v9708 = (v5496 * v4614) - ((v9702 + v5777) * v4614);
                    let v9711 = (if (v2880 / v4637) >= v4546 { (v2880 / v4637) } else { v4546 }).ln();
                    let v9717 = ((((v9712 * v9) * v2880) * v4614).sqrt()) / v13;
                    let v9719 = (v5516 * v4614) - (v209 * v4614);
                    let v9720 = v1 / v9717;
                    let v9721 = v9717 * v9717;
                    let v9722 = v1 / v9721;
                    let v9723 = v9717 / v5830;
                    let v9725 = v1 + (v9723 * v5832);
                    let v9726 = v5835 * v9725;
                    let v9728 = v1 / v9723;
                    let v9729 = v9723 * v9723;
                    let v9732 = v1 / (v9727 + (v9723 * v5840));
                    let v9733 = v9719.abs();
                    let v9734 = if v9733 <= v9726 { 1.0 } else { 0.0 };
                    let v9863: f64;
                    if v9734 != 0.0 {
                        let v9735 = -v9719;
                        let v9743 = (v9735 * v9728) * (v1 + (v9723 * (v9735 / ((v9737 * v9725) * v9725))));
                        v9863 = v9743;
                    } else {
                        let v9745 = if v9719 < (-v9726) { 1.0 } else { 0.0 };
                        let v9864: f64;
                        if v9745 != 0.0 {
                            let v9746 = -v9719;
                            let v9748 = (v9727 * v9746) * v9728;
                            let v9750 = v9748 - v2979;
                            let v9755 = v2485 * ((v9748 + v2908) - (((v9750 * v9750) + v5863).sqrt()));
                            let v9756 = v9746 - v9755;
                            let v9760 = (v9756 * v9756) + (v9729 * (v9755 + v1));
                            let v9762 = (v71 * v9756) - v9729;
                            let v9766 = ((if (v9760 / v9729) >= v4546 { (v9760 / v9729) } else { v4546 }).ln()) - v9755;
                            let v9767 = v9760 + v9762;
                            let v9769 = v9762 * v9762;
                            let v9773 = (v9767 * v9767) + (v9766 * ((v2485 * v9769) - v9760));
                            let v9785 = v9755 + (((v9760 * v9767) * v9766) / (v9773 + (((((v9767 / v9773) * v9766) * v9766) * v9762) * ((v9769 * v4724) - v9760))));
                            let v9786 = rspice_limited_exp(v9785);
                            let v9787 = v9746 - v9785;
                            let v9791 = (v71 * v9787) + (v9729 * (v9786 - v1));
                            let v9796 = (v9787 * v9787) + (v9729 * ((v9785 + v1) - v9786));
                            let v9809 = -(v9785 + (v71 * (v9796 / (v9791 + (((v9791 * v9791) - (v2976 * ((v1 - ((v9729 * v2485) * v9786)) * v9796))).sqrt())))));
                            v9864 = v9809;
                        } else {
                            let v9821 = v9729 * v2485;
                            let v9828 = (v9719 + v9821) - (v9723 * (((v9719 + (v9729 * v2542)) - (v1 - (rspice_limited_exp((-((v9719 * v9728) * (v1 + (((((v9725 * v9727) * v9732) - v1) * v9732) * v9719)))))))).sqrt()));
                            let v9830 = rspice_limited_exp((-v9828));
                            let v9831 = v9719 - v9828;
                            let v9835 = (v71 * v9831) + (v9729 * (v1 - v9830));
                            let v9840 = (v9831 * v9831) - (v9729 * ((v9828 - v1) + v9830));
                            let v9851 = v9828 + (v71 * (v9840 / (v9835 + (((v9835 * v9835) - (v2976 * ((v1 - (v9821 * v9830)) * v9840))).sqrt()))));
                            v9864 = v9851;
                        }
                        v9863 = v9864;
                    }
                    let v9852 = if v9733 < v9726 { 1.0 } else { 0.0 };
                    let v9884: f64;
                    if v9852 != 0.0 {
                        let v9853 = -v9719;
                        let v9861 = (v9853 * v9728) * (v1 + (v9723 * (v9853 / ((v9855 * v9725) * v9725))));
                        v9884 = v9861;
                    } else {
                        let v9862 = v5830 * v5830;
                        let v9865 = v9719 - v9863;
                        let v9871 = rspice_limited_exp((-v9863));
                        let v9883 = v9863 - ((((((v9862 * v9865) * v9865) * v9720) * v9720) - ((v9871 + v9863) - v1)) / ((v9871 + ((v9862 * ((v71 * v9863) - (v71 * v9719))) / v9721)) - v1));
                        v9884 = v9883;
                    }
                    let v9885 = v9884 * v4613;
                    let v9887 = v1 + (v9717 * v5832);
                    let v9888 = v1 / v9887;
                    let v9890 = (v71 * v9711) / v5728;
                    let v9891 = v9890 + v9705;
                    let v9893 = rspice_limited_exp((-v9891));
                    let v9894 = v4710 * v9887;
                    let v9895 = v2 * v2880;
                    let v9902 = ((v6016 * ((v9895 * v5613) * v5613)) / (v4697 * v4613)) + (v6022 / v4613);
                    let v9903 = v5829 * v9719;
                    let v9904 = v9902 - v9903;
                    let v9911 = v9904 + (v9717 * ((((rspice_limited_exp((-v9904))) + v9904) - v1).sqrt()));
                    let v9912 = if v9904 < v9891 { 1.0 } else { 0.0 };
                    let v10507: f64;
                    if v9912 != 0.0 {
                        let v9913 = if v9708 < v9911 { 1.0 } else { 0.0 };
                        let v10508: f64;
                        if v9913 != 0.0 {
                            let v9915 = if (v9708.abs()) <= v9894 { 1.0 } else { 0.0 };
                            let v10509: f64;
                            if v9915 != 0.0 {
                                let v9925 = (v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * (((v9888 * v9888) * v6039) * v5832)));
                                v10509 = v9925;
                            } else {
                                let v9927 = if v9708 < (-v9894) { 1.0 } else { 0.0 };
                                let v10510: f64;
                                if v9927 != 0.0 {
                                    let v9928 = -v9708;
                                    let v9930 = v6052 * (v9928 * v9888);
                                    let v9932 = v9930 - v2979;
                                    let v9937 = v2485 * ((v9930 + v2908) - (((v9932 * v9932) + v5863).sqrt()));
                                    let v9938 = v9928 - v9937;
                                    let v9942 = (v9938 * v9938) + (v9721 * (v9937 + v1));
                                    let v9944 = (v71 * v9938) - v9721;
                                    let v9949 = (-v9937) + ((if (v9942 * v9722) >= v4546 { (v9942 * v9722) } else { v4546 }).ln());
                                    let v9950 = v9942 + v9944;
                                    let v9952 = v9944 * v9944;
                                    let v9956 = (v9950 * v9950) + (v9949 * ((v2485 * v9952) - v9942));
                                    let v9968 = v9937 + (((v9942 * v9950) * v9949) / (v9956 + (((((v9950 / v9956) * v9949) * v9949) * v9944) * ((v9952 * v4724) - v9942))));
                                    let v9969 = rspice_limited_exp(v9968);
                                    let v9971 = v9968 * v9968;
                                    let v9973 = v1 / (v71 + v9971);
                                    let v9974 = v9971 * v9973;
                                    let v9983 = v9928 - v9968;
                                    let v9984 = v9893 * (v1 / v9969);
                                    let v9992 = (v71 * v9983) + (v9721 * (((v9969 - v1) - v9984) + (v9893 * (v1 - (v2976 * ((v9968 * v9973) * v9973))))));
                                    let v10002 = (v9983 * v9983) - (v9721 * ((((v9969 - v9968) - v1) + v9984) + (v9893 * ((v9968 - v1) - v9974))));
                                    let v10017 = (-v9968) - (v71 * (v10002 / (v9992 + (((v9992 * v9992) - (v71 * (v10002 * (v71 - (v9721 * ((v9969 + v9984) - (v9893 * ((((v3003 * v9973) - (v6103 * v9974)) * v9973) * v9973)))))))).sqrt()))));
                                    v10510 = v10017;
                                } else {
                                    let v10020 = v1 / (v6052 + (v9717 * v5840));
                                    let v10039 = (v9708 + (v9721 * v2485)) - (v9717 * (((v9708 + (v9721 * v2542)) - (v1 - (rspice_limited_exp((-((v9708 * v9888) * (v1 + (((((v9887 * v6052) * v10020) - v1) * v10020) * v9708)))))))).sqrt()));
                                    let v10040 = v9891 + v2974;
                                    let v10042 = v10039 - v10040;
                                    let v10053 = (v2485 * ((v10039 + v10040) - (((v10042 * v10042) + v2964).sqrt()))) - (v2485 * (v10040 - (((v10040 * v10040) + v2964).sqrt())));
                                    let v10054 = v9708 - v10053;
                                    let v10056 = rspice_limited_exp((-v10053));
                                    let v10057 = v10053 * v10053;
                                    let v10059 = v1 / (v71 + v10057);
                                    let v10060 = v10057 * v10059;
                                    let v10078 = if v6194 >= ((v10054 * v10054) - (v9721 * (((v10056 + v10053) - v1) - (v9893 * ((v10053 + v1) + v10060))))) { v6194 } else { ((v10054 * v10054) - (v9721 * (((v10056 + v10053) - v1) - (v9893 * ((v10053 + v1) + v10060))))) };
                                    let v10090 = (v71 * v10054) + (v9721 * ((v1 - v10056) - (v9893 * (v1 + (v2976 * ((v10053 * v10059) * v10059))))));
                                    let v10095 = (v9891 - v10053) + ((if (v10078 / v9721) >= v4546 { (v10078 / v9721) } else { v4546 }).ln());
                                    let v10096 = v10078 + v10090;
                                    let v10098 = v10090 * v10090;
                                    let v10100 = v10078 * (v1 - (v2485 * (v9721 * (v10056 - (v9893 * ((((v3003 * v10059) - (v6103 * v10060)) * v10059) * v10059))))));
                                    let v10103 = (v10096 * v10096) + (v10095 * ((v2485 * v10098) - v10100));
                                    let v10115 = v10053 + (((v10078 * v10096) * v10095) / (v10103 + (((((v10096 / v10103) * v10095) * v10095) * v10090) * ((v10098 * v4724) - v10100))));
                                    let v10117 = v1 / (rspice_limited_exp(v10115));
                                    let v10119 = rspice_limited_exp((v10115 - v9891));
                                    let v10120 = v10115 * v10115;
                                    let v10122 = v1 / (v71 + v10120);
                                    let v10123 = v10120 * v10122;
                                    let v10132 = v9708 - v10115;
                                    let v10140 = (v71 * v10132) + (v9721 * (((v1 - v10117) + v10119) - (v9893 * (v1 + (v2976 * ((v10115 * v10122) * v10122))))));
                                    let v10150 = (v10132 * v10132) - (v9721 * ((((v10117 + v10115) - v1) + v10119) - (v9893 * ((v10115 + v1) + v10123))));
                                    let v10164 = v10115 + (v71 * (v10150 / (v10140 + (((v10140 * v10140) - (v71 * (v10150 * (v71 - (v9721 * ((v10117 + v10119) - (v9893 * ((((v3003 * v10122) - (v6103 * v10123)) * v10122) * v10122)))))))).sqrt()))));
                                    v10510 = v10164;
                                }
                                v10509 = v10510;
                            }
                            v10508 = v10509;
                        } else {
                            let v10165 = v5830 * v5830;
                            let v10167 = v9904 - (v9885 * v4614);
                            let v10174 = v9708 - (v9717 * ((((rspice_limited_exp((-v10167))) + v10167) - v1).sqrt()));
                            let v10175 = v9891 + v2974;
                            let v10177 = v10174 - v10175;
                            let v10182 = v2485 * ((v10174 + v10175) - (((v10177 * v10177) + v5647).sqrt()));
                            let v10183 = v9708 - v10182;
                            let v10186 = (v9719 - v10182) + v9904;
                            let v10191 = ((v10183 * v10183) - ((v10165 * v10186) * v10186)) - (v9721 * v9904);
                            let v10193 = v71 * v10165;
                            let v10195 = (v71 * v10183) - (v10193 * v10186);
                            let v10196 = v10195 * v10195;
                            let v10197 = v1 - v10165;
                            let v10198 = if v10191 < v0 { 1.0 } else { 0.0 };
                            let v10200: f64;
                            if v10198 != 0.0 {
                                v10200 = v0;
                            } else {
                                v10200 = v10191;
                            }
                            let v10205 = v10200 + v10195;
                            let v10210 = v10200 * v10197;
                            let v10211 = (((v10205 * v10205) / ((v9891 - v10182) + ((if (v10200 * v9722) >= v4546 { (v10200 * v9722) } else { v4546 }).ln()))) + (v2485 * v10196)) - v10210;
                            let v10220 = v10182 + ((v10205 * v10200) / (v10211 + (((v10195 * v10205) / v10211) * ((v4724 * v10196) - v10210))));
                            let v10222 = rspice_limited_exp((v10220 - v9891));
                            let v10223 = v9708 - v10220;
                            let v10226 = (v9719 - v10220) + v9904;
                            let v10229 = v9721 * v10222;
                            let v10230 = ((v71 * v10223) - (v10193 * v10226)) + v10229;
                            let v10238 = v71 * (((v10223 * v10223) - ((v10165 * v10226) * v10226)) - (v9721 * (v9904 + v10222)));
                            let v10247 = v10220 + (v10238 / (v10230 + (((v10230 * v10230) - (v10238 * ((v71 - v10193) - v10229))).sqrt())));
                            v10508 = v10247;
                        }
                        v10507 = v10508;
                    } else {
                        let v10249 = if (v9708.abs()) <= v9894 { 1.0 } else { 0.0 };
                        let v10511: f64;
                        if v10249 != 0.0 {
                            let v10259 = (v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * (((v9888 * v9888) * v6039) * v5832)));
                            v10511 = v10259;
                        } else {
                            let v10261 = if v9708 < (-v9894) { 1.0 } else { 0.0 };
                            let v10512: f64;
                            if v10261 != 0.0 {
                                let v10262 = -v9708;
                                let v10264 = v6052 * (v10262 * v9888);
                                let v10266 = v10264 - v2979;
                                let v10271 = v2485 * ((v10264 + v2908) - (((v10266 * v10266) + v5863).sqrt()));
                                let v10272 = v10262 - v10271;
                                let v10276 = (v10272 * v10272) + (v9721 * (v10271 + v1));
                                let v10278 = (v71 * v10272) - v9721;
                                let v10283 = (-v10271) + ((if (v10276 * v9722) >= v4546 { (v10276 * v9722) } else { v4546 }).ln());
                                let v10284 = v10276 + v10278;
                                let v10286 = v10278 * v10278;
                                let v10290 = (v10284 * v10284) + (v10283 * ((v2485 * v10286) - v10276));
                                let v10302 = v10271 + (((v10276 * v10284) * v10283) / (v10290 + (((((v10284 / v10290) * v10283) * v10283) * v10278) * ((v10286 * v4724) - v10276))));
                                let v10303 = rspice_limited_exp(v10302);
                                let v10305 = v10302 * v10302;
                                let v10307 = v1 / (v71 + v10305);
                                let v10308 = v10305 * v10307;
                                let v10317 = v10262 - v10302;
                                let v10318 = v9893 * (v1 / v10303);
                                let v10326 = (v71 * v10317) + (v9721 * (((v10303 - v1) - v10318) + (v9893 * (v1 - (v2976 * ((v10302 * v10307) * v10307))))));
                                let v10336 = (v10317 * v10317) - (v9721 * ((((v10303 - v10302) - v1) + v10318) + (v9893 * ((v10302 - v1) - v10308))));
                                let v10345 = (v10326 * v10326) - (v71 * (v10336 * (v71 - (v9721 * ((v10303 + v10318) - (v9893 * ((((v3003 * v10307) - (v6103 * v10308)) * v10307) * v10307)))))));
                                let v10355 = (-v10302) - (v71 * (v10336 / (v10326 + (((((v10345 * v10345) + v6473).sqrt()) - v6476).sqrt()))));
                                v10512 = v10355;
                            } else {
                                let v10358 = v1 / (v6052 + (v9717 * v5840));
                                let v10377 = (v9708 + (v9721 * v2485)) - (v9717 * (((v9708 + (v9721 * v2542)) - (v1 - (rspice_limited_exp((-((v9708 * v9888) * (v1 + (((((v9887 * v6052) * v10358) - v1) * v10358) * v9708)))))))).sqrt()));
                                let v10378 = v9891 + v2974;
                                let v10380 = v10377 - v10378;
                                let v10391 = (v2485 * ((v10377 + v10378) - (((v10380 * v10380) + v2964).sqrt()))) - (v2485 * (v10378 - (((v10378 * v10378) + v2964).sqrt())));
                                let v10392 = v9708 - v10391;
                                let v10394 = rspice_limited_exp((-v10391));
                                let v10395 = v10391 * v10391;
                                let v10397 = v1 / (v71 + v10395);
                                let v10398 = v10395 * v10397;
                                let v10416 = if v6194 >= ((v10392 * v10392) - (v9721 * (((v10394 + v10391) - v1) - (v9893 * ((v10391 + v1) + v10398))))) { v6194 } else { ((v10392 * v10392) - (v9721 * (((v10394 + v10391) - v1) - (v9893 * ((v10391 + v1) + v10398))))) };
                                let v10428 = (v71 * v10392) + (v9721 * ((v1 - v10394) - (v9893 * (v1 + (v2976 * ((v10391 * v10397) * v10397))))));
                                let v10433 = (v9891 - v10391) + ((if (v10416 / v9721) >= v4546 { (v10416 / v9721) } else { v4546 }).ln());
                                let v10434 = v10416 + v10428;
                                let v10436 = v10428 * v10428;
                                let v10438 = v10416 * (v1 - (v2485 * (v9721 * (v10394 - (v9893 * ((((v3003 * v10397) - (v6103 * v10398)) * v10397) * v10397))))));
                                let v10441 = (v10434 * v10434) + (v10433 * ((v2485 * v10436) - v10438));
                                let v10453 = v10391 + (((v10416 * v10434) * v10433) / (v10441 + (((((v10434 / v10441) * v10433) * v10433) * v10428) * ((v10436 * v4724) - v10438))));
                                let v10455 = v1 / (rspice_limited_exp(v10453));
                                let v10457 = rspice_limited_exp((v10453 - v9891));
                                let v10458 = v10453 * v10453;
                                let v10460 = v1 / (v71 + v10458);
                                let v10461 = v10458 * v10460;
                                let v10470 = v9708 - v10453;
                                let v10478 = (v71 * v10470) + (v9721 * (((v1 - v10455) + v10457) - (v9893 * (v1 + (v2976 * ((v10453 * v10460) * v10460))))));
                                let v10488 = (v10470 * v10470) - (v9721 * ((((v10455 + v10453) - v1) + v10457) - (v9893 * ((v10453 + v1) + v10461))));
                                let v10497 = (v10478 * v10478) - (v71 * (v10488 * (v71 - (v9721 * ((v10455 + v10457) - (v9893 * ((((v3003 * v10460) - (v6103 * v10461)) * v10460) * v10460)))))));
                                let v10506 = v10453 + (v71 * (v10488 / (v10478 + (((((v10497 * v10497) + v6473).sqrt()) - v6476).sqrt()))));
                                v10512 = v10506;
                            }
                            v10511 = v10512;
                        }
                        v10507 = v10511;
                    }
                    let v10515 = ((v9888 * v9888) * v6039) * v5832;
                    let v10523 = (((v6016 * (v9895 * v6644)) / v4697) + v6022) - ((v5829 * (v9719 * v4613)) * v349);
                    let v10524 = v6653 * v9885;
                    let v10525 = v10523 + v10524;
                    let v10526 = v9708.abs();
                    let v10527 = if v10526 <= v5835 { 1.0 } else { 0.0 };
                    let v10684: f64;
                    let v12271: f64;
                    if v10527 != 0.0 {
                        let v10534 = (v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * v10515));
                        v10684 = v10534;
                        v12271 = v12272;
                    } else {
                        let v10551 = ((v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * v10515))) * (v2485 * (((v10535 * (v9708 - v71)).tanh()) + ((v2964 * (v9708 + v71)).tanh())));
                        let v10554 = ((v10507 * v4613) - v10525) / v4613;
                        let v10555 = rspice_limited_exp(v10554);
                        let v10556 = if v10554 > v6689 { 1.0 } else { 0.0 };
                        let v10573: f64;
                        if v10556 != 0.0 {
                            v10573 = v10554;
                        } else {
                            let v10558 = if v10554 < v10557 { 1.0 } else { 0.0 };
                            let v10574: f64;
                            if v10558 != 0.0 {
                                let v10559 = v10554.exp();
                                v10574 = v10559;
                            } else {
                                let v10562 = (v1 + (v10554.exp())).ln();
                                v10574 = v10562;
                            }
                            v10573 = v10574;
                        }
                        let v10565 = ((v10551 * v4613) - v10525) / v4613;
                        let v10566 = if v10565 > v6689 { 1.0 } else { 0.0 };
                        let v10575: f64;
                        if v10566 != 0.0 {
                            v10575 = v10565;
                        } else {
                            let v10568 = if v10565 < v10567 { 1.0 } else { 0.0 };
                            let v10576: f64;
                            if v10568 != 0.0 {
                                let v10569 = v10565.exp();
                                v10576 = v10569;
                            } else {
                                let v10572 = (v1 + (v10565.exp())).ln();
                                v10576 = v10572;
                            }
                            v10575 = v10576;
                        }
                        let v10581 = -((v9885 / v4613) + ((v10573 - v10575) / v6653));
                        let v10582 = rspice_limited_exp(v10581);
                        let v10584 = rspice_limited_exp((-v10507));
                        let v10585 = v10507 * v10507;
                        let v10587 = v1 / (v10585 + v71);
                        let v10589 = rspice_limited_exp((v10507 - v9891));
                        let v10590 = v9708 - v10507;
                        let v10593 = v9719 + v10581;
                        let v10602 = v10587 * v10585;
                        let v10607 = ((v10590 * v10590) - (((v5830 * v5830) * v10593) * v10593)) - (v9721 * (((((v10584 - v10582) + v10507) + v10581) + v10589) - (v9893 * ((v10507 + v1) + v10602))));
                        let v10612 = v1 + v10555;
                        let v10613 = v6653 * v10612;
                        let v10617 = v71 * v10507;
                        let v10631 = v10555 / v10613;
                        let v10633 = v10555 * v10582;
                        let v10638 = (((((((v71 * v10555) * v10593) * v5830) * v5830) / v10613) - (v71 * v9708)) + v10617) - (v9721 * (((((v10589 + (v9893 * ((((v10619 * v10507) * v10587) + ((((v10617 * v10507) * v10507) * v10587) * v10587)) - v1))) - v10584) - v10631) + (v10633 / v10613)) + v1));
                        let v10641 = ((v71 * v5830) * v5830) * v10555;
                        let v10644 = v10641 * v10555;
                        let v10683 = v10507 - ((v10607 / v10638) * (v1 + ((v10607 * ((((((v10641 * v10593) / v10613) - (v10644 / ((v10613 * v6653) * v10612))) - (v9721 * (((v10584 + v10589) - (((v71 * v9893) * v10587) * (v1 - (v10602 * (v2964 - ((v2976 * v10585) * v10587)))))) - (v10631 * (((v1 - (v10555 / v10612)) - v10582) + ((v10633 / v10612) * (v1 + (v1 / v6653)))))))) - ((v10644 * v10593) / (v10613 * v10612))) + v71)) / ((v71 * v10638) * v10638))));
                        v10684 = v10683;
                        v12271 = v10551;
                    }
                    let v10841: f64;
                    let v12270: f64;
                    if v10527 != 0.0 {
                        let v10691 = (v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * v10515));
                        v10841 = v10691;
                        v12270 = v12271;
                    } else {
                        let v10708 = ((v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * v10515))) * (v2485 * (((v10692 * (v9708 - v71)).tanh()) + ((v2964 * (v9708 + v71)).tanh())));
                        let v10711 = ((v10684 * v4613) - v10525) / v4613;
                        let v10712 = rspice_limited_exp(v10711);
                        let v10713 = if v10711 > v6689 { 1.0 } else { 0.0 };
                        let v10730: f64;
                        if v10713 != 0.0 {
                            v10730 = v10711;
                        } else {
                            let v10715 = if v10711 < v10714 { 1.0 } else { 0.0 };
                            let v10731: f64;
                            if v10715 != 0.0 {
                                let v10716 = v10711.exp();
                                v10731 = v10716;
                            } else {
                                let v10719 = (v1 + (v10711.exp())).ln();
                                v10731 = v10719;
                            }
                            v10730 = v10731;
                        }
                        let v10722 = ((v10708 * v4613) - v10525) / v4613;
                        let v10723 = if v10722 > v6689 { 1.0 } else { 0.0 };
                        let v10732: f64;
                        if v10723 != 0.0 {
                            v10732 = v10722;
                        } else {
                            let v10725 = if v10722 < v10724 { 1.0 } else { 0.0 };
                            let v10733: f64;
                            if v10725 != 0.0 {
                                let v10726 = v10722.exp();
                                v10733 = v10726;
                            } else {
                                let v10729 = (v1 + (v10722.exp())).ln();
                                v10733 = v10729;
                            }
                            v10732 = v10733;
                        }
                        let v10738 = -((v9885 / v4613) + ((v10730 - v10732) / v6653));
                        let v10739 = rspice_limited_exp(v10738);
                        let v10741 = rspice_limited_exp((-v10684));
                        let v10742 = v10684 * v10684;
                        let v10744 = v1 / (v10742 + v71);
                        let v10746 = rspice_limited_exp((v10684 - v9891));
                        let v10747 = v9708 - v10684;
                        let v10750 = v9719 + v10738;
                        let v10759 = v10744 * v10742;
                        let v10764 = ((v10747 * v10747) - (((v5830 * v5830) * v10750) * v10750)) - (v9721 * (((((v10741 - v10739) + v10684) + v10738) + v10746) - (v9893 * ((v10684 + v1) + v10759))));
                        let v10769 = v1 + v10712;
                        let v10770 = v6653 * v10769;
                        let v10774 = v71 * v10684;
                        let v10788 = v10712 / v10770;
                        let v10790 = v10712 * v10739;
                        let v10795 = (((((((v71 * v10712) * v10750) * v5830) * v5830) / v10770) - (v71 * v9708)) + v10774) - (v9721 * (((((v10746 + (v9893 * ((((v10776 * v10684) * v10744) + ((((v10774 * v10684) * v10684) * v10744) * v10744)) - v1))) - v10741) - v10788) + (v10790 / v10770)) + v1));
                        let v10798 = ((v71 * v5830) * v5830) * v10712;
                        let v10801 = v10798 * v10712;
                        let v10840 = v10684 - ((v10764 / v10795) * (v1 + ((v10764 * ((((((v10798 * v10750) / v10770) - (v10801 / ((v10770 * v6653) * v10769))) - (v9721 * (((v10741 + v10746) - (((v71 * v9893) * v10744) * (v1 - (v10759 * (v2964 - ((v2976 * v10742) * v10744)))))) - (v10788 * (((v1 - (v10712 / v10769)) - v10739) + ((v10790 / v10769) * (v1 + (v1 / v6653)))))))) - ((v10801 * v10750) / (v10770 * v10769))) + v71)) / ((v71 * v10795) * v10795))));
                        v10841 = v10840;
                        v12270 = v10708;
                    }
                    let v10999: f64;
                    let v12269: f64;
                    if v10527 != 0.0 {
                        let v10848 = (v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * v10515));
                        v10999 = v10848;
                        v12269 = v12270;
                    } else {
                        let v10865 = ((v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * v10515))) * (v2485 * (((v10849 * (v9708 - v71)).tanh()) + ((v2964 * (v9708 + v71)).tanh())));
                        let v10868 = ((v10841 * v4613) - v10525) / v4613;
                        let v10869 = rspice_limited_exp(v10868);
                        let v10870 = if v10868 > v6689 { 1.0 } else { 0.0 };
                        let v10887: f64;
                        if v10870 != 0.0 {
                            v10887 = v10868;
                        } else {
                            let v10872 = if v10868 < v10871 { 1.0 } else { 0.0 };
                            let v10888: f64;
                            if v10872 != 0.0 {
                                let v10873 = v10868.exp();
                                v10888 = v10873;
                            } else {
                                let v10876 = (v1 + (v10868.exp())).ln();
                                v10888 = v10876;
                            }
                            v10887 = v10888;
                        }
                        let v10879 = ((v10865 * v4613) - v10525) / v4613;
                        let v10880 = if v10879 > v6689 { 1.0 } else { 0.0 };
                        let v10889: f64;
                        if v10880 != 0.0 {
                            v10889 = v10879;
                        } else {
                            let v10882 = if v10879 < v10881 { 1.0 } else { 0.0 };
                            let v10890: f64;
                            if v10882 != 0.0 {
                                let v10883 = v10879.exp();
                                v10890 = v10883;
                            } else {
                                let v10886 = (v1 + (v10879.exp())).ln();
                                v10890 = v10886;
                            }
                            v10889 = v10890;
                        }
                        let v10895 = -((v9885 / v4613) + ((v10887 - v10889) / v6653));
                        let v10896 = rspice_limited_exp(v10895);
                        let v10898 = rspice_limited_exp((-v10841));
                        let v10899 = v10841 * v10841;
                        let v10901 = v1 / (v10899 + v71);
                        let v10903 = rspice_limited_exp((v10841 - v9891));
                        let v10904 = v9708 - v10841;
                        let v10907 = v9719 + v10895;
                        let v10916 = v10901 * v10899;
                        let v10921 = ((v10904 * v10904) - (((v5830 * v5830) * v10907) * v10907)) - (v9721 * (((((v10898 - v10896) + v10841) + v10895) + v10903) - (v9893 * ((v10841 + v1) + v10916))));
                        let v10926 = v1 + v10869;
                        let v10927 = v6653 * v10926;
                        let v10931 = v71 * v10841;
                        let v10945 = v10869 / v10927;
                        let v10947 = v10869 * v10896;
                        let v10952 = (((((((v71 * v10869) * v10907) * v5830) * v5830) / v10927) - (v71 * v9708)) + v10931) - (v9721 * (((((v10903 + (v9893 * ((((v10933 * v10841) * v10901) + ((((v10931 * v10841) * v10841) * v10901) * v10901)) - v1))) - v10898) - v10945) + (v10947 / v10927)) + v1));
                        let v10955 = ((v71 * v5830) * v5830) * v10869;
                        let v10958 = v10955 * v10869;
                        let v10997 = v10841 - ((v10921 / v10952) * (v1 + ((v10921 * ((((((v10955 * v10907) / v10927) - (v10958 / ((v10927 * v6653) * v10926))) - (v9721 * (((v10898 + v10903) - (((v71 * v9893) * v10901) * (v1 - (v10916 * (v2964 - ((v2976 * v10899) * v10901)))))) - (v10945 * (((v1 - (v10869 / v10926)) - v10896) + ((v10947 / v10926) * (v1 + (v1 / v6653)))))))) - ((v10958 * v10907) / (v10927 * v10926))) + v71)) / ((v71 * v10952) * v10952))));
                        v10999 = v10997;
                        v12269 = v10865;
                    }
                    let v10998 = v7129 * v4613;
                    let v11000 = if v10999 <= v0 { 1.0 } else { 0.0 };
                    let v12491: f64;
                    let v12492: f64;
                    let v12493: f64;
                    let v12501: f64;
                    let v12504: f64;
                    let v12548: f64;
                    let v12663: f64;
                    let v12783: f64;
                    let v12787: f64;
                    let v12790: f64;
                    let v12829: f64;
                    let v12923: f64;
                    let v12950: f64;
                    let v12968: f64;
                    let v15771: f64;
                    let v19873: f64;
                    let v19888: f64;
                    if v11000 != 0.0 {
                        let v11002 = (v9708 - v10999) * v4613;
                        v12491 = v11002;
                        v12492 = v9046;
                        v12493 = v9558;
                        v12501 = v9564;
                        v12504 = v9563;
                        v12548 = v12549;
                        v12663 = v12664;
                        v12783 = v12784;
                        v12787 = v1;
                        v12790 = v1;
                        v12829 = v0;
                        v12923 = v1;
                        v12950 = v1;
                        v12968 = v10998;
                        v15771 = v12269;
                        v19873 = v7135;
                        v19888 = v7138;
                    } else {
                        let v11003 = v10999 * v10999;
                        let v11008 = v1 / (rspice_limited_exp(v10999));
                        let v11014 = (rspice_limited_exp((v10999 - v9891))) - (v9893 * ((v10999 + v1) + (v11003 * (v1 / (v71 + v11003)))));
                        let v11015 = v9708 - v10999;
                        let v11019 = (((v11015 * v11015) * v9722) - v11014) - v4710;
                        let v11026 = (v2485 * (v11019 + (((v11019 * v11019) + v11021).sqrt()))) + v4710;
                        let v11033 = v9717 * (v11026.sqrt());
                        let v11035 = ((v9721 * v11014) * v4613) / ((v9717 * ((v11026 + v11014).sqrt())) + v11033);
                        let v11036 = v11033 * v4613;
                        let v11054 = v1 + (((v7189 + (v7190 * v5549)) * (((v7176 / v5617) * (v11036 + (v4728 * v11035))).powf(v4768))) + (v7195 / (rspice_limited_exp((v7181 * ((if (v2485 * (v1 + (v11035 / v11036))) >= v4546 { (v2485 * (v1 + (v11035 / v11036))) } else { v4546 }).ln()))))));
                        let v11056 = v11054 - v1;
                        let v11062 = v2485 * ((v11054 + v1) + (((v11056 * v11056) + v11058).sqrt()));
                        let v11066 = v1 / (((v83 * v7207).powf(v769)) * v32);
                        let v11067 = v4695 - v5549;
                        let v11069 = v11067 - v5443;
                        let v11076 = (v2485 * ((v11067 + v5443) + (((v11069 * v11069) + v11071).sqrt()))).sqrt();
                        let v11102: f64;
                        if v2834 != 0.0 {
                            v11102 = v0;
                        } else {
                            let v11082 = (v1 / (v1 + (v749 * v11035))) + (v2812 * (v11076 - v4696));
                            let v11090 = ((v7221 + (v7223 * (v11082 + (((v11082 * v11082) + v4979).sqrt())))) * v11066) * v32;
                            let v11091 = v11090 * v4795;
                            let v11092 = if v2833 == v71 { 1.0 } else { 0.0 };
                            let v11103: f64;
                            if v11092 != 0.0 {
                                let v11095 = ((v7138 + v11090) + v7135) * v4795;
                                v11103 = v11095;
                            } else {
                                v11103 = v11091;
                            }
                            v11102 = v11103;
                        }
                        let v11099 = (((v71 * v7234) / v7236) * v11062) * v73;
                        let v11101 = v919 * (v11035 + v4633);
                        let v11104 = if v11102 > v0 { 1.0 } else { 0.0 };
                        let v11152: f64;
                        if v11104 != 0.0 {
                            let v11107 = ((v83 * v7234) * v13) * v11102;
                            let v11108 = v71 * v11107;
                            let v11112 = (v11101 + v11099) + ((v2974 * v11101) * v11107);
                            let v11123 = (v11112 - (((v11112 * v11112) - ((v71 * v11108) * (v11101 * (v11099 + ((v71 * v11101) * v11107))))).sqrt())) / v11108;
                            v11152 = v11123;
                        } else {
                            let v11126 = (v11099 * v11101) / (v11099 + v11101);
                            v11152 = v11126;
                        }
                        let v11129 = if (if v7268 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v7270 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v11161: f64;
                        if v11129 != 0.0 {
                            v11161 = v1;
                        } else {
                            let v11133 = v73 / (v73 + ((v599 * v5666).sqrt()));
                            let v11143 = v1 + (((v7268 * v11133) - (((v7270 * v11133) * (v11035.powf(v7279))) * v4613)) / (v1 + (v7284 * v5549)));
                            let v11145 = v11143 - v5425;
                            let v11151 = v2485 * ((v11143 + v5425) + (((v11145 * v11145) + v11147).sqrt()));
                            v11161 = v11151;
                        }
                        let v11153 = v11152 - v4710;
                        let v11162 = ((v2485 * (v11153 + (((v11153 * v11153) + v11155).sqrt()))) + v4710) / v11161;
                        let v11170 = v5528 * ((v1 + (((v5528 / v11162) + v127).powf((v1 / v4819)))).powf((-v4819)));
                        let v11173 = v9890 + ((v11170 + v5547) * v4614);
                        let v11174 = -v11173;
                        let v11175 = rspice_limited_exp(v11174);
                        let v11176 = v5528 * v4614;
                        let v11179 = v6014 * (v11176 + (v329 * v4614));
                        let v11183 = (v9902 - (v9903 * v349)) + (v6653 * v11179);
                        let v11190 = v11183 + (v9717 * ((((rspice_limited_exp((-v11183))) + v11183) - v1).sqrt()));
                        let v11191 = if v11183 < v11173 { 1.0 } else { 0.0 };
                        let v11770: f64;
                        if v11191 != 0.0 {
                            let v11192 = if v9708 < v11190 { 1.0 } else { 0.0 };
                            let v11771: f64;
                            if v11192 != 0.0 {
                                let v11193 = if v10526 <= v9894 { 1.0 } else { 0.0 };
                                let v11772: f64;
                                if v11193 != 0.0 {
                                    let v11200 = (v9708 * v9888) * (v1 + (((v9708 * (v1 - v11175)) * v9717) * v10515));
                                    v11772 = v11200;
                                } else {
                                    let v11202 = if v9708 < (-v9894) { 1.0 } else { 0.0 };
                                    let v11773: f64;
                                    if v11202 != 0.0 {
                                        let v11203 = -v9708;
                                        let v11205 = v6052 * (v11203 * v9888);
                                        let v11207 = v11205 - v2979;
                                        let v11212 = v2485 * ((v11205 + v2908) - (((v11207 * v11207) + v5863).sqrt()));
                                        let v11213 = v11203 - v11212;
                                        let v11217 = (v11213 * v11213) + (v9721 * (v11212 + v1));
                                        let v11219 = (v71 * v11213) - v9721;
                                        let v11224 = (-v11212) + ((if (v11217 * v9722) >= v4546 { (v11217 * v9722) } else { v4546 }).ln());
                                        let v11225 = v11217 + v11219;
                                        let v11227 = v11219 * v11219;
                                        let v11231 = (v11225 * v11225) + (v11224 * ((v2485 * v11227) - v11217));
                                        let v11243 = v11212 + (((v11217 * v11225) * v11224) / (v11231 + (((((v11225 / v11231) * v11224) * v11224) * v11219) * ((v11227 * v4724) - v11217))));
                                        let v11244 = rspice_limited_exp(v11243);
                                        let v11246 = v11243 * v11243;
                                        let v11248 = v1 / (v71 + v11246);
                                        let v11249 = v11246 * v11248;
                                        let v11258 = v11203 - v11243;
                                        let v11259 = v11175 * (v1 / v11244);
                                        let v11267 = (v71 * v11258) + (v9721 * (((v11244 - v1) - v11259) + (v11175 * (v1 - (v2976 * ((v11243 * v11248) * v11248))))));
                                        let v11277 = (v11258 * v11258) - (v9721 * ((((v11244 - v11243) - v1) + v11259) + (v11175 * ((v11243 - v1) - v11249))));
                                        let v11292 = (-v11243) - (v71 * (v11277 / (v11267 + (((v11267 * v11267) - (v71 * (v11277 * (v71 - (v9721 * ((v11244 + v11259) - (v11175 * ((((v3003 * v11248) - (v6103 * v11249)) * v11248) * v11248)))))))).sqrt()))));
                                        v11773 = v11292;
                                    } else {
                                        let v11295 = v1 / (v6052 + (v9717 * v5840));
                                        let v11314 = (v9708 + (v9721 * v2485)) - (v9717 * (((v9708 + (v9721 * v2542)) - (v1 - (rspice_limited_exp((-((v9708 * v9888) * (v1 + (((((v9887 * v6052) * v11295) - v1) * v11295) * v9708)))))))).sqrt()));
                                        let v11315 = v11173 + v2974;
                                        let v11317 = v11314 - v11315;
                                        let v11328 = (v2485 * ((v11314 + v11315) - (((v11317 * v11317) + v2964).sqrt()))) - (v2485 * (v11315 - (((v11315 * v11315) + v2964).sqrt())));
                                        let v11329 = v9708 - v11328;
                                        let v11331 = rspice_limited_exp((-v11328));
                                        let v11332 = v11328 * v11328;
                                        let v11334 = v1 / (v71 + v11332);
                                        let v11335 = v11332 * v11334;
                                        let v11353 = if v6194 >= ((v11329 * v11329) - (v9721 * (((v11331 + v11328) - v1) - (v11175 * ((v11328 + v1) + v11335))))) { v6194 } else { ((v11329 * v11329) - (v9721 * (((v11331 + v11328) - v1) - (v11175 * ((v11328 + v1) + v11335))))) };
                                        let v11365 = (v71 * v11329) + (v9721 * ((v1 - v11331) - (v11175 * (v1 + (v2976 * ((v11328 * v11334) * v11334))))));
                                        let v11370 = (v11173 - v11328) + ((if (v11353 / v9721) >= v4546 { (v11353 / v9721) } else { v4546 }).ln());
                                        let v11371 = v11353 + v11365;
                                        let v11373 = v11365 * v11365;
                                        let v11375 = v11353 * (v1 - (v2485 * (v9721 * (v11331 - (v11175 * ((((v3003 * v11334) - (v6103 * v11335)) * v11334) * v11334))))));
                                        let v11378 = (v11371 * v11371) + (v11370 * ((v2485 * v11373) - v11375));
                                        let v11390 = v11328 + (((v11353 * v11371) * v11370) / (v11378 + (((((v11371 / v11378) * v11370) * v11370) * v11365) * ((v11373 * v4724) - v11375))));
                                        let v11392 = v1 / (rspice_limited_exp(v11390));
                                        let v11394 = rspice_limited_exp((v11390 - v11173));
                                        let v11395 = v11390 * v11390;
                                        let v11397 = v1 / (v71 + v11395);
                                        let v11398 = v11395 * v11397;
                                        let v11407 = v9708 - v11390;
                                        let v11415 = (v71 * v11407) + (v9721 * (((v1 - v11392) + v11394) - (v11175 * (v1 + (v2976 * ((v11390 * v11397) * v11397))))));
                                        let v11425 = (v11407 * v11407) - (v9721 * ((((v11392 + v11390) - v1) + v11394) - (v11175 * ((v11390 + v1) + v11398))));
                                        let v11439 = v11390 + (v71 * (v11425 / (v11415 + (((v11415 * v11415) - (v71 * (v11425 * (v71 - (v9721 * ((v11392 + v11394) - (v11175 * ((((v3003 * v11397) - (v6103 * v11398)) * v11397) * v11397)))))))).sqrt()))));
                                        v11773 = v11439;
                                    }
                                    v11772 = v11773;
                                }
                                v11771 = v11772;
                            } else {
                                let v11440 = v5830 * v5830;
                                let v11442 = v11183 - (v9885 * v4614);
                                let v11449 = v9708 - (v9717 * ((((rspice_limited_exp((-v11442))) + v11442) - v1).sqrt()));
                                let v11450 = v11173 + v2974;
                                let v11452 = v11449 - v11450;
                                let v11457 = v2485 * ((v11449 + v11450) - (((v11452 * v11452) + v5647).sqrt()));
                                let v11458 = v9708 - v11457;
                                let v11461 = (v9719 - v11457) + v11183;
                                let v11466 = ((v11458 * v11458) - ((v11440 * v11461) * v11461)) - (v9721 * v11183);
                                let v11468 = v71 * v11440;
                                let v11470 = (v71 * v11458) - (v11468 * v11461);
                                let v11471 = v11470 * v11470;
                                let v11472 = v1 - v11440;
                                let v11473 = if v11466 < v0 { 1.0 } else { 0.0 };
                                let v11475: f64;
                                if v11473 != 0.0 {
                                    v11475 = v0;
                                } else {
                                    v11475 = v11466;
                                }
                                let v11480 = v11475 + v11470;
                                let v11485 = v11475 * v11472;
                                let v11486 = (((v11480 * v11480) / ((v11173 - v11457) + ((if (v11475 * v9722) >= v4546 { (v11475 * v9722) } else { v4546 }).ln()))) + (v2485 * v11471)) - v11485;
                                let v11495 = v11457 + ((v11480 * v11475) / (v11486 + (((v11470 * v11480) / v11486) * ((v4724 * v11471) - v11485))));
                                let v11497 = rspice_limited_exp((v11495 - v11173));
                                let v11498 = v9708 - v11495;
                                let v11501 = (v9719 - v11495) + v11183;
                                let v11504 = v9721 * v11497;
                                let v11505 = ((v71 * v11498) - (v11468 * v11501)) + v11504;
                                let v11513 = v71 * (((v11498 * v11498) - ((v11440 * v11501) * v11501)) - (v9721 * (v11183 + v11497)));
                                let v11522 = v11495 + (v11513 / (v11505 + (((v11505 * v11505) - (v11513 * ((v71 - v11468) - v11504))).sqrt())));
                                v11771 = v11522;
                            }
                            v11770 = v11771;
                        } else {
                            let v11523 = if v10526 <= v9894 { 1.0 } else { 0.0 };
                            let v11774: f64;
                            if v11523 != 0.0 {
                                let v11530 = (v9708 * v9888) * (v1 + (((v9708 * (v1 - v11175)) * v9717) * v10515));
                                v11774 = v11530;
                            } else {
                                let v11532 = if v9708 < (-v9894) { 1.0 } else { 0.0 };
                                let v11775: f64;
                                if v11532 != 0.0 {
                                    let v11533 = -v9708;
                                    let v11535 = v6052 * (v11533 * v9888);
                                    let v11537 = v11535 - v2979;
                                    let v11542 = v2485 * ((v11535 + v2908) - (((v11537 * v11537) + v5863).sqrt()));
                                    let v11543 = v11533 - v11542;
                                    let v11547 = (v11543 * v11543) + (v9721 * (v11542 + v1));
                                    let v11549 = (v71 * v11543) - v9721;
                                    let v11554 = (-v11542) + ((if (v11547 * v9722) >= v4546 { (v11547 * v9722) } else { v4546 }).ln());
                                    let v11555 = v11547 + v11549;
                                    let v11557 = v11549 * v11549;
                                    let v11561 = (v11555 * v11555) + (v11554 * ((v2485 * v11557) - v11547));
                                    let v11573 = v11542 + (((v11547 * v11555) * v11554) / (v11561 + (((((v11555 / v11561) * v11554) * v11554) * v11549) * ((v11557 * v4724) - v11547))));
                                    let v11574 = rspice_limited_exp(v11573);
                                    let v11576 = v11573 * v11573;
                                    let v11578 = v1 / (v71 + v11576);
                                    let v11579 = v11576 * v11578;
                                    let v11588 = v11533 - v11573;
                                    let v11589 = v11175 * (v1 / v11574);
                                    let v11597 = (v71 * v11588) + (v9721 * (((v11574 - v1) - v11589) + (v11175 * (v1 - (v2976 * ((v11573 * v11578) * v11578))))));
                                    let v11607 = (v11588 * v11588) - (v9721 * ((((v11574 - v11573) - v1) + v11589) + (v11175 * ((v11573 - v1) - v11579))));
                                    let v11622 = (-v11573) - (v71 * (v11607 / (v11597 + (((v11597 * v11597) - (v71 * (v11607 * (v71 - (v9721 * ((v11574 + v11589) - (v11175 * ((((v3003 * v11578) - (v6103 * v11579)) * v11578) * v11578)))))))).sqrt()))));
                                    v11775 = v11622;
                                } else {
                                    let v11625 = v1 / (v6052 + (v9717 * v5840));
                                    let v11644 = (v9708 + (v9721 * v2485)) - (v9717 * (((v9708 + (v9721 * v2542)) - (v1 - (rspice_limited_exp((-((v9708 * v9888) * (v1 + (((((v9887 * v6052) * v11625) - v1) * v11625) * v9708)))))))).sqrt()));
                                    let v11645 = v11173 + v2974;
                                    let v11647 = v11644 - v11645;
                                    let v11658 = (v2485 * ((v11644 + v11645) - (((v11647 * v11647) + v2964).sqrt()))) - (v2485 * (v11645 - (((v11645 * v11645) + v2964).sqrt())));
                                    let v11659 = v9708 - v11658;
                                    let v11661 = rspice_limited_exp((-v11658));
                                    let v11662 = v11658 * v11658;
                                    let v11664 = v1 / (v71 + v11662);
                                    let v11665 = v11662 * v11664;
                                    let v11683 = if v6194 >= ((v11659 * v11659) - (v9721 * (((v11661 + v11658) - v1) - (v11175 * ((v11658 + v1) + v11665))))) { v6194 } else { ((v11659 * v11659) - (v9721 * (((v11661 + v11658) - v1) - (v11175 * ((v11658 + v1) + v11665))))) };
                                    let v11695 = (v71 * v11659) + (v9721 * ((v1 - v11661) - (v11175 * (v1 + (v2976 * ((v11658 * v11664) * v11664))))));
                                    let v11700 = (v11173 - v11658) + ((if (v11683 / v9721) >= v4546 { (v11683 / v9721) } else { v4546 }).ln());
                                    let v11701 = v11683 + v11695;
                                    let v11703 = v11695 * v11695;
                                    let v11705 = v11683 * (v1 - (v2485 * (v9721 * (v11661 - (v11175 * ((((v3003 * v11664) - (v6103 * v11665)) * v11664) * v11664))))));
                                    let v11708 = (v11701 * v11701) + (v11700 * ((v2485 * v11703) - v11705));
                                    let v11720 = v11658 + (((v11683 * v11701) * v11700) / (v11708 + (((((v11701 / v11708) * v11700) * v11700) * v11695) * ((v11703 * v4724) - v11705))));
                                    let v11722 = v1 / (rspice_limited_exp(v11720));
                                    let v11724 = rspice_limited_exp((v11720 - v11173));
                                    let v11725 = v11720 * v11720;
                                    let v11727 = v1 / (v71 + v11725);
                                    let v11728 = v11725 * v11727;
                                    let v11737 = v9708 - v11720;
                                    let v11745 = (v71 * v11737) + (v9721 * (((v1 - v11722) + v11724) - (v11175 * (v1 + (v2976 * ((v11720 * v11727) * v11727))))));
                                    let v11755 = (v11737 * v11737) - (v9721 * ((((v11722 + v11720) - v1) + v11724) - (v11175 * ((v11720 + v1) + v11728))));
                                    let v11769 = v11720 + (v71 * (v11755 / (v11745 + (((v11745 * v11745) - (v71 * (v11755 * (v71 - (v9721 * ((v11722 + v11724) - (v11175 * ((((v3003 * v11727) - (v6103 * v11728)) * v11727) * v11727)))))))).sqrt()))));
                                    v11775 = v11769;
                                }
                                v11774 = v11775;
                            }
                            v11770 = v11774;
                        }
                        let v11779 = (v10523 + ((v6653 * v11179) * v4613)) + v10524;
                        let v11936: f64;
                        let v12268: f64;
                        if v10527 != 0.0 {
                            let v11786 = (v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * v10515));
                            v11936 = v11786;
                            v12268 = v12269;
                        } else {
                            let v11803 = ((v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * v10515))) * (v2485 * (((v11787 * (v9708 - v71)).tanh()) + ((v2964 * (v9708 + v71)).tanh())));
                            let v11806 = ((v11770 * v4613) - v11779) / v4613;
                            let v11807 = rspice_limited_exp(v11806);
                            let v11808 = if v11806 > v6689 { 1.0 } else { 0.0 };
                            let v11825: f64;
                            if v11808 != 0.0 {
                                v11825 = v11806;
                            } else {
                                let v11810 = if v11806 < v11809 { 1.0 } else { 0.0 };
                                let v11826: f64;
                                if v11810 != 0.0 {
                                    let v11811 = v11806.exp();
                                    v11826 = v11811;
                                } else {
                                    let v11814 = (v1 + (v11806.exp())).ln();
                                    v11826 = v11814;
                                }
                                v11825 = v11826;
                            }
                            let v11817 = ((v11803 * v4613) - v11779) / v4613;
                            let v11818 = if v11817 > v6689 { 1.0 } else { 0.0 };
                            let v11827: f64;
                            if v11818 != 0.0 {
                                v11827 = v11817;
                            } else {
                                let v11820 = if v11817 < v11819 { 1.0 } else { 0.0 };
                                let v11828: f64;
                                if v11820 != 0.0 {
                                    let v11821 = v11817.exp();
                                    v11828 = v11821;
                                } else {
                                    let v11824 = (v1 + (v11817.exp())).ln();
                                    v11828 = v11824;
                                }
                                v11827 = v11828;
                            }
                            let v11833 = -((v9885 / v4613) + ((v11825 - v11827) / v6653));
                            let v11834 = rspice_limited_exp(v11833);
                            let v11836 = rspice_limited_exp((-v11770));
                            let v11837 = v11770 * v11770;
                            let v11839 = v1 / (v11837 + v71);
                            let v11841 = rspice_limited_exp((v11770 - v11173));
                            let v11842 = v9708 - v11770;
                            let v11845 = v9719 + v11833;
                            let v11854 = v11839 * v11837;
                            let v11859 = ((v11842 * v11842) - (((v5830 * v5830) * v11845) * v11845)) - (v9721 * (((((v11836 - v11834) + v11770) + v11833) + v11841) - (v11175 * ((v11770 + v1) + v11854))));
                            let v11864 = v1 + v11807;
                            let v11865 = v6653 * v11864;
                            let v11869 = v71 * v11770;
                            let v11883 = v11807 / v11865;
                            let v11885 = v11807 * v11834;
                            let v11890 = (((((((v71 * v11807) * v11845) * v5830) * v5830) / v11865) - (v71 * v9708)) + v11869) - (v9721 * (((((v11841 + (v11175 * ((((v11871 * v11770) * v11839) + ((((v11869 * v11770) * v11770) * v11839) * v11839)) - v1))) - v11836) - v11883) + (v11885 / v11865)) + v1));
                            let v11893 = ((v71 * v5830) * v5830) * v11807;
                            let v11896 = v11893 * v11807;
                            let v11935 = v11770 - ((v11859 / v11890) * (v1 + ((v11859 * ((((((v11893 * v11845) / v11865) - (v11896 / ((v11865 * v6653) * v11864))) - (v9721 * (((v11836 + v11841) - (((v71 * v11175) * v11839) * (v1 - (v11854 * (v2964 - ((v2976 * v11837) * v11839)))))) - (v11883 * (((v1 - (v11807 / v11864)) - v11834) + ((v11885 / v11864) * (v1 + (v1 / v6653)))))))) - ((v11896 * v11845) / (v11865 * v11864))) + v71)) / ((v71 * v11890) * v11890))));
                            v11936 = v11935;
                            v12268 = v11803;
                        }
                        let v12093: f64;
                        let v12267: f64;
                        if v10527 != 0.0 {
                            let v11943 = (v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * v10515));
                            v12093 = v11943;
                            v12267 = v12268;
                        } else {
                            let v11960 = ((v9708 * v9888) * (v1 + (((v9708 * (v1 - v11175)) * v9717) * v10515))) * (v2485 * (((v11944 * (v9708 - v71)).tanh()) + ((v2964 * (v9708 + v71)).tanh())));
                            let v11963 = ((v11936 * v4613) - v11779) / v4613;
                            let v11964 = rspice_limited_exp(v11963);
                            let v11965 = if v11963 > v6689 { 1.0 } else { 0.0 };
                            let v11982: f64;
                            if v11965 != 0.0 {
                                v11982 = v11963;
                            } else {
                                let v11967 = if v11963 < v11966 { 1.0 } else { 0.0 };
                                let v11983: f64;
                                if v11967 != 0.0 {
                                    let v11968 = v11963.exp();
                                    v11983 = v11968;
                                } else {
                                    let v11971 = (v1 + (v11963.exp())).ln();
                                    v11983 = v11971;
                                }
                                v11982 = v11983;
                            }
                            let v11974 = ((v11960 * v4613) - v11779) / v4613;
                            let v11975 = if v11974 > v6689 { 1.0 } else { 0.0 };
                            let v11984: f64;
                            if v11975 != 0.0 {
                                v11984 = v11974;
                            } else {
                                let v11977 = if v11974 < v11976 { 1.0 } else { 0.0 };
                                let v11985: f64;
                                if v11977 != 0.0 {
                                    let v11978 = v11974.exp();
                                    v11985 = v11978;
                                } else {
                                    let v11981 = (v1 + (v11974.exp())).ln();
                                    v11985 = v11981;
                                }
                                v11984 = v11985;
                            }
                            let v11990 = -((v9885 / v4613) + ((v11982 - v11984) / v6653));
                            let v11991 = rspice_limited_exp(v11990);
                            let v11993 = rspice_limited_exp((-v11936));
                            let v11994 = v11936 * v11936;
                            let v11996 = v1 / (v11994 + v71);
                            let v11998 = rspice_limited_exp((v11936 - v11173));
                            let v11999 = v9708 - v11936;
                            let v12002 = v9719 + v11990;
                            let v12011 = v11996 * v11994;
                            let v12016 = ((v11999 * v11999) - (((v5830 * v5830) * v12002) * v12002)) - (v9721 * (((((v11993 - v11991) + v11936) + v11990) + v11998) - (v11175 * ((v11936 + v1) + v12011))));
                            let v12021 = v1 + v11964;
                            let v12022 = v6653 * v12021;
                            let v12026 = v71 * v11936;
                            let v12040 = v11964 / v12022;
                            let v12042 = v11964 * v11991;
                            let v12047 = (((((((v71 * v11964) * v12002) * v5830) * v5830) / v12022) - (v71 * v9708)) + v12026) - (v9721 * (((((v11998 + (v11175 * ((((v12028 * v11936) * v11996) + ((((v12026 * v11936) * v11936) * v11996) * v11996)) - v1))) - v11993) - v12040) + (v12042 / v12022)) + v1));
                            let v12050 = ((v71 * v5830) * v5830) * v11964;
                            let v12053 = v12050 * v11964;
                            let v12092 = v11936 - ((v12016 / v12047) * (v1 + ((v12016 * ((((((v12050 * v12002) / v12022) - (v12053 / ((v12022 * v6653) * v12021))) - (v9721 * (((v11993 + v11998) - (((v71 * v11175) * v11996) * (v1 - (v12011 * (v2964 - ((v2976 * v11994) * v11996)))))) - (v12040 * (((v1 - (v11964 / v12021)) - v11991) + ((v12042 / v12021) * (v1 + (v1 / v6653)))))))) - ((v12053 * v12002) / (v12022 * v12021))) + v71)) / ((v71 * v12047) * v12047))));
                            v12093 = v12092;
                            v12267 = v11960;
                        }
                        let v12250: f64;
                        let v12266: f64;
                        if v10527 != 0.0 {
                            let v12100 = (v9708 * v9888) * (v1 + (((v9708 * (v1 - v9893)) * v9717) * v10515));
                            v12250 = v12100;
                            v12266 = v12267;
                        } else {
                            let v12117 = ((v9708 * v9888) * (v1 + (((v9708 * (v1 - v11175)) * v9717) * v10515))) * (v2485 * (((v12101 * (v9708 - v71)).tanh()) + ((v2964 * (v9708 + v71)).tanh())));
                            let v12120 = ((v12093 * v4613) - v11779) / v4613;
                            let v12121 = rspice_limited_exp(v12120);
                            let v12122 = if v12120 > v6689 { 1.0 } else { 0.0 };
                            let v12139: f64;
                            if v12122 != 0.0 {
                                v12139 = v12120;
                            } else {
                                let v12124 = if v12120 < v12123 { 1.0 } else { 0.0 };
                                let v12140: f64;
                                if v12124 != 0.0 {
                                    let v12125 = v12120.exp();
                                    v12140 = v12125;
                                } else {
                                    let v12128 = (v1 + (v12120.exp())).ln();
                                    v12140 = v12128;
                                }
                                v12139 = v12140;
                            }
                            let v12131 = ((v12117 * v4613) - v11779) / v4613;
                            let v12132 = if v12131 > v6689 { 1.0 } else { 0.0 };
                            let v12141: f64;
                            if v12132 != 0.0 {
                                v12141 = v12131;
                            } else {
                                let v12134 = if v12131 < v12133 { 1.0 } else { 0.0 };
                                let v12142: f64;
                                if v12134 != 0.0 {
                                    let v12135 = v12131.exp();
                                    v12142 = v12135;
                                } else {
                                    let v12138 = (v1 + (v12131.exp())).ln();
                                    v12142 = v12138;
                                }
                                v12141 = v12142;
                            }
                            let v12147 = -((v9885 / v4613) + ((v12139 - v12141) / v6653));
                            let v12148 = rspice_limited_exp(v12147);
                            let v12150 = rspice_limited_exp((-v12093));
                            let v12151 = v12093 * v12093;
                            let v12153 = v1 / (v12151 + v71);
                            let v12155 = rspice_limited_exp((v12093 - v11173));
                            let v12156 = v9708 - v12093;
                            let v12159 = v9719 + v12147;
                            let v12168 = v12153 * v12151;
                            let v12173 = ((v12156 * v12156) - (((v5830 * v5830) * v12159) * v12159)) - (v9721 * (((((v12150 - v12148) + v12093) + v12147) + v12155) - (v11175 * ((v12093 + v1) + v12168))));
                            let v12178 = v1 + v12121;
                            let v12179 = v6653 * v12178;
                            let v12183 = v71 * v12093;
                            let v12197 = v12121 / v12179;
                            let v12199 = v12121 * v12148;
                            let v12204 = (((((((v71 * v12121) * v12159) * v5830) * v5830) / v12179) - (v71 * v9708)) + v12183) - (v9721 * (((((v12155 + (v11175 * ((((v12185 * v12093) * v12153) + ((((v12183 * v12093) * v12093) * v12153) * v12153)) - v1))) - v12150) - v12197) + (v12199 / v12179)) + v1));
                            let v12207 = ((v71 * v5830) * v5830) * v12121;
                            let v12210 = v12207 * v12121;
                            let v12249 = v12093 - ((v12173 / v12204) * (v1 + ((v12173 * ((((((v12207 * v12159) / v12179) - (v12210 / ((v12179 * v6653) * v12178))) - (v9721 * (((v12150 + v12155) - (((v71 * v11175) * v12153) * (v1 - (v12168 * (v2964 - ((v2976 * v12151) * v12153)))))) - (v12197 * (((v1 - (v12121 / v12178)) - v12148) + ((v12199 / v12178) * (v1 + (v1 / v6653)))))))) - ((v12210 * v12159) / (v12179 * v12178))) + v71)) / ((v71 * v12204) * v12204))));
                            v12250 = v12249;
                            v12266 = v12117;
                        }
                        let v12251 = v12250 - v10999;
                        let v12252 = -v11176;
                        let v12253 = rspice_limited_exp(v12252);
                        let v12254 = if v12251 < v8399 { 1.0 } else { 0.0 };
                        let v12405: f64;
                        let v12407: f64;
                        if v12254 != 0.0 {
                            let v12256 = (v12093 * v4613) - v11779;
                            let v12257 = v12256 / v4613;
                            let v12258 = rspice_limited_exp(v12257);
                            let v12259 = if v12257 > v6689 { 1.0 } else { 0.0 };
                            let v12283: f64;
                            if v12259 != 0.0 {
                                v12283 = v12257;
                            } else {
                                let v12261 = if v12257 < v12260 { 1.0 } else { 0.0 };
                                let v12284: f64;
                                if v12261 != 0.0 {
                                    let v12262 = v12257.exp();
                                    v12284 = v12262;
                                } else {
                                    let v12265 = (v1 + (v12257.exp())).ln();
                                    v12284 = v12265;
                                }
                                v12283 = v12284;
                            }
                            let v12275 = ((v12266 * v4613) - v11779) / v4613;
                            let v12276 = if v12275 > v6689 { 1.0 } else { 0.0 };
                            let v12285: f64;
                            if v12276 != 0.0 {
                                v12285 = v12275;
                            } else {
                                let v12278 = if v12275 < v12277 { 1.0 } else { 0.0 };
                                let v12286: f64;
                                if v12278 != 0.0 {
                                    let v12279 = v12275.exp();
                                    v12286 = v12279;
                                } else {
                                    let v12282 = (v1 + (v12275.exp())).ln();
                                    v12286 = v12282;
                                }
                                v12285 = v12286;
                            }
                            let v12291 = -((v9885 / v4613) + ((v12283 - v12285) / v6653));
                            let v12295 = rspice_limited_exp((-v12093));
                            let v12298 = v1 / ((v12093 * v12093) + v71);
                            let v12301 = (v71 * v12256) / v4613;
                            let v12302 = rspice_limited_exp(v12301);
                            let v12304 = rspice_limited_exp((v12301 + v12291));
                            let v12305 = v71 * v12258;
                            let v12306 = v9719 + v12291;
                            let v12311 = v6653 * (v12258 + v1);
                            let v12315 = v71 * v12093;
                            let v12334 = v12258 / v12311;
                            let v12336 = (rspice_limited_exp((v12291 + v12257))) / v12311;
                            let v12341 = -(((((((v12305 * v12306) * v5830) * v5830) / v12311) - (v71 * v9708)) + v12315) - (v9721 * ((((((rspice_limited_exp(((v12093 - v11176) - v11173))) + ((rspice_limited_exp((v12252 - v11173))) * ((((v12322 * v12093) * v12298) + ((((v12315 * v12093) * v12093) * v12298) * v12298)) - v1))) - v12295) - v12334) + v12336) + v1)));
                            let v12344 = (v9721 * (v1 - v12253)) * v11014;
                            let v12346 = (v71 * v5830) * v5830;
                            let v12350 = v12346 * v12302;
                            let v12353 = (v1 + v12305) + v12302;
                            let v12354 = (v6653 * v6653) * v12353;
                            let v12380 = v6653 * v12353;
                            let v12397 = (v12341 * v12341) - (v71 * ((((((((v12346 * v12258) * v12306) / v12311) - (v12350 / v12354)) - (v9721 * (((((((v12295 + (rspice_limited_exp(((v12093 - v11173) - v11176)))) + ((rspice_limited_exp((v11174 - v11176))) * (((v12362 * v12298) + ((((v2908 * v12093) * v12093) * v12298) * v12298)) - (((((((v3003 * v12093) * v12093) * v12093) * v12093) * v12298) * v12298) * v12298)))) - v12334) + (v12302 / v12380)) + v12336) - (v12304 / v12380)) - (v12304 / v12354)))) - ((v12350 * v12306) / v12380)) + v71) * v12344));
                            let v12398 = if v12397 >= v0 { 1.0 } else { 0.0 };
                            let v12403: f64;
                            if v12398 != 0.0 {
                                let v12402 = v71 * (v12344 / (v12341 + (v12397.sqrt())));
                                v12403 = v12402;
                            } else {
                                v12403 = v12251;
                            }
                            let v12404 = v10999 + v12403;
                            v12405 = v12403;
                            v12407 = v12404;
                        } else {
                            v12405 = v12251;
                            v12407 = v12250;
                        }
                        let v12406 = v12405 * v4613;
                        let v12408 = v12407 * v12407;
                        let v12418 = (rspice_limited_exp((v12407 - v11173))) - (v11175 * ((v12407 + v1) + (v12408 / (v71 + v12408))));
                        let v12419 = v9708 - v12407;
                        let v12423 = (((v12419 * v12419) * v9722) - v12418) - v4710;
                        let v12430 = (v2485 * (v12423 + (((v12423 * v12423) + v12425).sqrt()))) + v4710;
                        let v12439 = ((v9721 * v12418) * v4613) / ((v9717 * ((v12430 + v12418).sqrt())) + (v9717 * (v12430.sqrt())));
                        let v12452 = (v2485 * (v11014 + v12418)) + (v8592 * ((v12405 * v12405) * (((((rspice_limited_exp((-v12407))) * v11008).abs()).sqrt()) - (v71 * v9722))));
                        let v12453 = v9708 - (v2485 * (v10999 + v12407));
                        let v12456 = ((v12453 * v12453) * v9722) - v12452;
                        let v12459 = v9717 * ((v12452 + v12456).sqrt());
                        let v12460 = v12456 - v4710;
                        let v12468 = ((v2485 * (v12460 + (((v12460 * v12460) + v12462).sqrt()))) + v4710).sqrt();
                        let v12470 = if (v12406.abs()) > v8693 { 1.0 } else { 0.0 };
                        let v12479: f64;
                        if v12470 != 0.0 {
                            let v12472 = (v11035 - v12439) / v12406;
                            v12479 = v12472;
                        } else {
                            v12479 = v9564;
                        }
                        let v12477 = v4613 * ((v9721 * v12452) / (v12459 + (v9717 * v12468)));
                        let v12478 = v12459 * v4613;
                        let v12480 = v12479 - v4710;
                        let v12487 = (v2485 * (v12480 + (((v12480 * v12480) + v12482).sqrt()))) + v4710;
                        let v12490 = (v12477 + (v4613 * v12487)) / v12487;
                        v12491 = v12478;
                        v12492 = v12406;
                        v12493 = v12490;
                        v12501 = v12479;
                        v12504 = v12477;
                        v12548 = v12407;
                        v12663 = v11170;
                        v12783 = v12439;
                        v12787 = v9062;
                        v12790 = v9060;
                        v12829 = v11035;
                        v12923 = v9065;
                        v12950 = v9056;
                        v12968 = v11162;
                        v15771 = v12266;
                        v19873 = v19874;
                        v19888 = v19889;
                    }
                    let v12519: f64;
                    let v12521: f64;
                    if v11000 != 0.0 {
                        v12519 = v0;
                        v12521 = v0;
                    } else {
                        let v12495 = v2485 * (v12492 / v12493);
                        let v12500 = v12491 + (v2485 * (v12492 * (v12495 * v4724)));
                        let v12503 = (v12501 * v12492) * v6039;
                        let v12512 = v2485 * (v12504 - (v12503 * ((v1 - v12495) - (v9586 * (v12495 * v12495)))));
                        let v12515 = (v12500 - (v12500 - (v12504 + (v12503 * v12495)))) - v12512;
                        v12519 = v12515;
                        v12521 = v12512;
                    }
                    let v12518 = -(v12516 * v9641);
                    let v12525: f64;
                    let v12526: f64;
                    if v9645 != 0.0 {
                        let v12520 = v12518 * v12519;
                        let v12522 = v12518 * v12521;
                        v12525 = v12520;
                        v12526 = v12522;
                    } else {
                        let v12523 = v12518 * v12521;
                        let v12524 = v12518 * v12519;
                        v12525 = v12523;
                        v12526 = v12524;
                    }
                    v12527 = v12525;
                    v12530 = v12526;
                    v12546 = v10999;
                    v12547 = v12548;
                    v12662 = v12663;
                    v12782 = v12783;
                    v12786 = v12787;
                    v12789 = v12790;
                    v12828 = v12829;
                    v12922 = v12923;
                    v12949 = v12950;
                    v12967 = v12968;
                    v13406 = v9711;
                    v13413 = v2880;
                    v15770 = v15771;
                    v19872 = v19873;
                    v19887 = v19888;
                } else {
                    v12527 = v0;
                    v12530 = v0;
                    v12546 = v7131;
                    v12547 = v12549;
                    v12662 = v12664;
                    v12782 = v12784;
                    v12786 = v9062;
                    v12789 = v9060;
                    v12828 = v12830;
                    v12922 = v9065;
                    v12949 = v9056;
                    v12967 = v12969;
                    v13406 = v4691;
                    v13413 = v2291;
                    v15770 = v12272;
                    v19872 = v19874;
                    v19887 = v19889;
                }
                let v12529 = v4661 * (v9652 + v12527);
                let v12532 = v4661 * (v9653 + v12530);
                let v12534 = v2587 + (v1609 * v4706);
                let v12536 = v2594 + (v1649 * v4706);
                let v12538 = v2601 + (v1689 * v4706);
                let v12540 = v1569 + (v1579 * v4706);
                let v12542 = v1539 + (v1549 * v4706);
                let v12544 = if v12543 != v0 { 1.0 } else { 0.0 };
                let v12545 = if v2886 != 0.0 || v12544 != 0.0 { 1.0 } else { 0.0 };
                let v12751: f64;
                let v12753: f64;
                let v12755: f64;
                let v12757: f64;
                let v12760: f64;
                if v12545 != 0.0 {
                    let v12553 = v5729 * (v5796 - (v2485 * (v12546 + v12547)));
                    let v12556 = ((v12553 * v12553) + v9186).sqrt();
                    let v12559 = v2485 * ((-v12553) + v12556);
                    let v12561 = v2485 * (v12553 + v12556);
                    let v12756: f64;
                    if v12544 != 0.0 {
                        let v12564 = -(v12553 / v12562);
                        let v12565 = if v12564 > v6689 { 1.0 } else { 0.0 };
                        let v12572: f64;
                        if v12565 != 0.0 {
                            v12572 = v12564;
                        } else {
                            let v12567 = if v12564 < v12566 { 1.0 } else { 0.0 };
                            let v12573: f64;
                            if v12567 != 0.0 {
                                let v12568 = v12564.exp();
                                v12573 = v12568;
                            } else {
                                let v12571 = (v1 + (v12564.exp())).ln();
                                v12573 = v12571;
                            }
                            v12572 = v12573;
                        }
                        let v12574 = v12562 * v12572;
                        let v12576 = if v12575 != v0 { 1.0 } else { 0.0 };
                        let v12579: f64;
                        if v12576 != 0.0 {
                            let v12578 = v1 - (v12559 / v12575);
                            v12579 = v12578;
                        } else {
                            v12579 = v1;
                        }
                        let v12580 = if v12579 < v4979 { 1.0 } else { 0.0 };
                        let v12592: f64;
                        if v12580 != 0.0 {
                            v12592 = v4979;
                        } else {
                            v12592 = v12579;
                        }
                        let v12583 = ((v73 * v83) / v4564) + v4579;
                        let v12598 = (((((v12583 * v12584) * v4551) * v5493) * v12574) * (rspice_limited_exp((((v12587 * v12) * (v12542 - (v1559 * v12559))) / v12592)))) * v4866;
                        let v12600 = (v12553 - v1529) / v12562;
                        let v12601 = if v12600 > v6689 { 1.0 } else { 0.0 };
                        let v12608: f64;
                        if v12601 != 0.0 {
                            v12608 = v12600;
                        } else {
                            let v12603 = if v12600 < v12602 { 1.0 } else { 0.0 };
                            let v12609: f64;
                            if v12603 != 0.0 {
                                let v12604 = v12600.exp();
                                v12609 = v12604;
                            } else {
                                let v12607 = (v1 + (v12600.exp())).ln();
                                v12609 = v12607;
                            }
                            v12608 = v12609;
                        }
                        let v12610 = v12562 * v12608;
                        let v12612 = if v12611 != v0 { 1.0 } else { 0.0 };
                        let v12615: f64;
                        if v12612 != 0.0 {
                            let v12614 = v1 - (v12561 / v12611);
                            v12615 = v12614;
                        } else {
                            v12615 = v1;
                        }
                        let v12616 = if v12615 < v4979 { 1.0 } else { 0.0 };
                        let v12625: f64;
                        if v12616 != 0.0 {
                            v12625 = v4979;
                        } else {
                            v12625 = v12615;
                        }
                        let v12633 = v32 * (v12598 + ((((((v12583 * v12617) * v4551) * v5493) * v12610) * (rspice_limited_exp((((v12620 * v12) * (v12540 - (v1589 * v12561))) / v12625)))) * v4866));
                        v12756 = v12633;
                    } else {
                        v12756 = v0;
                    }
                    let v12642 = if (if (if (if v4681 != 0.0 && v12634 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v4592 != 0.0 && v12636 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v9701 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v12516 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v12642 != 0.0 {
                        if v5 != 0.0 {
                        } else {
                        }
                        if v5 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                    let v12752: f64;
                    let v12754: f64;
                    let v12758: f64;
                    let v12761: f64;
                    if v2886 != 0.0 {
                        let v12661 = (((v32 * v4582) * (((v5816 * v5729) * v9073) * (rspice_limited_exp(((v4575 * (v12534 - (v1619 * v12561))) * (v1 + (v1629 * v12561))))))) * ((v5493 + (v2485 * v5542)) - (v2485 * (v5527 + v5526)))) * v4866;
                        let v12669 = v2606 * ((((v12662 * v12662) + v4979).sqrt()) - v5425);
                        let v12671 = rspice_limited_exp((-v12669));
                        let v12674 = ((v12669 + v12671) - v1) + v9186;
                        let v12678 = (v1 - ((v12669 + v1) * v12671)) + v9186;
                        let v12681 = (v12669 * v12669) + v12680;
                        let v12759: f64;
                        let v12762: f64;
                        if v9645 != 0.0 {
                            let v12683 = (v12661 * v12678) / v12681;
                            let v12685 = (v12661 * v12674) / v12681;
                            v12759 = v12685;
                            v12762 = v12683;
                        } else {
                            let v12687 = (v12661 * v12678) / v12681;
                            let v12689 = (v12661 * v12674) / v12681;
                            v12759 = v12687;
                            v12762 = v12689;
                        }
                        let v12690 = v5511 - v8893;
                        let v12693 = ((v12690 * v12690) + v9186).sqrt();
                        let v12695 = if v12694 == v1 { 1.0 } else { 0.0 };
                        let v12707: f64;
                        let v12711: f64;
                        if v12695 != 0.0 {
                            let v12697 = v12536 - (v1659 * v12693);
                            let v12703 = v2485 * (v12697 + (((v12697 * v12697) + v12699).sqrt()));
                            let v12704 = if v1669 < v4979 { 1.0 } else { 0.0 };
                            let v12708: f64;
                            if v12704 != 0.0 {
                                v12708 = v4979;
                            } else {
                                v12708 = v1669;
                            }
                            v12707 = v12708;
                            v12711 = v12703;
                        } else {
                            let v12706 = v12536 - (v1659 * v12693);
                            v12707 = v1669;
                            v12711 = v12706;
                        }
                        let v12715 = v4866 * v32;
                        let v12721 = ((((v12715 * v4569) * v12717) * v5511) * v12693) * (rspice_limited_exp(((v4576 * v12711) * (v1 + (v12707 * v12693)))));
                        let v12722 = v5510 - v8893;
                        let v12725 = ((v12722 * v12722) + v9186).sqrt();
                        let v12737: f64;
                        let v12741: f64;
                        if v12695 != 0.0 {
                            let v12727 = v12538 - (v1699 * v12725);
                            let v12733 = v2485 * (v12727 + (((v12727 * v12727) + v12729).sqrt()));
                            let v12734 = if v1709 < v4979 { 1.0 } else { 0.0 };
                            let v12738: f64;
                            if v12734 != 0.0 {
                                v12738 = v4979;
                            } else {
                                v12738 = v1709;
                            }
                            v12737 = v12738;
                            v12741 = v12733;
                        } else {
                            let v12736 = v12538 - (v1699 * v12725);
                            v12737 = v1709;
                            v12741 = v12736;
                        }
                        let v12750 = ((((v12715 * v4573) * v12746) * v5510) * v12725) * (rspice_limited_exp(((v4576 * v12741) * (v1 + (v12737 * v12725)))));
                        v12752 = v12721;
                        v12754 = v12750;
                        v12758 = v12759;
                        v12761 = v12762;
                    } else {
                        v12752 = v0;
                        v12754 = v0;
                        v12758 = v0;
                        v12761 = v0;
                    }
                    v12751 = v12752;
                    v12753 = v12754;
                    v12755 = v12756;
                    v12757 = v12758;
                    v12760 = v12761;
                } else {
                    v12751 = v0;
                    v12753 = v0;
                    v12755 = v0;
                    v12757 = v0;
                    v12760 = v0;
                }
                let v12764 = (v2976 * v4613) * v2;
                let v12765 = v71 * v7234;
                let v12766 = v12765 / v9068;
                let v12768 = if v12767 <= v0 { 1.0 } else { 0.0 };
                let v12855: f64;
                if v12768 != 0.0 {
                    v12855 = v0;
                } else {
                    let v12774 = v4704 * ((if (((v9205 / v4704) + v12767) / v12766) >= v4546 { (((v9205 / v4704) + v12767) / v12766) } else { v4546 }).ln());
                    let v12775 = if v12774 < v0 { 1.0 } else { 0.0 };
                    let v12856: f64;
                    if v12775 != 0.0 {
                        v12856 = v0;
                    } else {
                        v12856 = v12774;
                    }
                    v12855 = v12856;
                }
                let v12779 = (v4613 / v2) * ((v13 + v5667) + v269);
                let v12781 = (v5817 * v13) * v4613;
                let v12792 = (((v12781 * v12782) * v12786) * v12789) / v2;
                let v12797 = ((v12793 * v4613) * (v9063.abs())) * v9068;
                let v12800 = ((v2 * v4613) * v9063) * v9063;
                let v12808 = (v12801 + (v12802 * v12792)) + ((v12805 * v12792) * v12792);
                let v12809 = v12792 + v12779;
                let v12810 = v12809 * v12809;
                let v12812 = (v12801 * v2) * v4613;
                let v12815 = if v12813 >= (v73 / v71) { 1.0 } else { 0.0 };
                let v12821: f64;
                if v12815 != 0.0 {
                    v12821 = v0;
                } else {
                    v12821 = v12813;
                }
                let v12820 = if (if (if v12801 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v12802 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v12805 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v12880: f64;
                if v12820 != 0.0 {
                    let v12823 = v73 - (v71 * v12821);
                    let v12824 = v12823 * v12823;
                    let v12834 = (((v12781 * v12828) * v12786) * v12789) / v2;
                    let v12860 = ((v12797 / ((v12825 * v13) * v12824)) * (((v12801 * ((if ((v12834 + v12779) / v12809) >= v4546 { ((v12834 + v12779) / v12809) } else { v4546 }).ln())) + (v12802 * (v12834 - v12792))) + ((v2485 * v12805) * ((v12834 * v12834) - (v12792 * v12792))))) + ((((v12800 / (((v12825 * v12824) * v83) * v32)) * v12855) * v12808) / v12810);
                    let v12868 = ((v12812 / (((((v83 * v32) * v12823) * v12825) * v12779) * v12779)) * v9063) * v9063;
                    let v12869 = v12868 + v12860;
                    let v12870 = if v12869 > v0 { 1.0 } else { 0.0 };
                    let v12881: f64;
                    if v12870 != 0.0 {
                        let v12879 = ((v12860 * v12868) / v12869) / (v1 + (v12873 * ((v12828 - v12782).powf(v12875))));
                        v12881 = v12879;
                    } else {
                        v12881 = v0;
                    }
                    v12880 = v12881;
                } else {
                    v12880 = v0;
                }
                let v12882 = v9216 * v12880;
                let v12885 = (v9073 / v12766) / v73;
                let v12886 = v12885 * v12885;
                let v12892 = v12887 * (v1 + ((v12888 * v73) * v12886));
                let v12898 = v12893 * (v1 + ((v12894 * v73) * v12886));
                let v12904 = v12899 * (v1 + ((v12900 * v73) * v12886));
                let v12910 = v12905 * (v1 + ((v12906 * v73) * v12886));
                let v12917 = rspice_limited_exp(((-v73) / v12915));
                let v12919 = ((((v2974 * v12892) * v12892) - v1) * v12917) + v1;
                let v12920 = v12904 * v12904;
                let v12921 = v12898 * v12898;
                let v12925 = if v12924 == v0 { 1.0 } else { 0.0 };
                let v19961: f64;
                let v19963: f64;
                let v19965: f64;
                let v19968: f64;
                let v19971: f64;
                let v19974: f64;
                let v19977: f64;
                let v19980: f64;
                if v12925 != 0.0 {
                    let v12930 = ((((-v32) * v83) * v73) * v13) * v4613;
                    let v12935 = v9068 * (((v12930 * v12529) + (v12930 * v12532)).abs());
                    let v12945 = v12764 * ((v12935 / ((v12935 * v12936) + (v73 * v73))) * v12943);
                    v19961 = v1;
                    v19963 = v12945;
                    v19965 = v0;
                    v19968 = v0;
                    v19971 = v0;
                    v19974 = v0;
                    v19977 = v0;
                    v19980 = v0;
                } else {
                    let v12946 = if v12924 == v1 { 1.0 } else { 0.0 };
                    let v19966: f64;
                    let v19969: f64;
                    let v19972: f64;
                    let v19975: f64;
                    let v19978: f64;
                    let v19981: f64;
                    if v12946 != 0.0 {
                        let v12953 = (((v9068 * v12922) * v12949) * v13) * (v5817 * v5729);
                        let v12955 = v2485 * (v12828 + v12782);
                        let v12956 = v12955 + v2485;
                        let v12957 = v12956 * v12956;
                        let v12958 = v12957 * v12956;
                        let v12959 = v12828 - v12782;
                        let v12960 = v12959 * v12959;
                        let v12965 = v73 * v12922;
                        let v12966 = v12965 / v73;
                        let v12978 = (((v1 + ((v12920 * (v12662 / v12967)) / (v12972 + v9073))) - v1) * v12917) + v1;
                        let v12990 = v6103 * v12956;
                        let v13016 = ((((((v12965 * v12966) * v12966) * (((v12955 / v12957) - ((((v2979 * v12955) + v2485) * v12960) / ((v12997 * v12957) * v12957))) + ((v12960 * v12960) / ((v13003 * v12957) * v12958)))) * v13009) / v2976) * v12921) / (((v32 * v83) * v6103) * v12953);
                        let v13024 = ((v12966 * ((v12959 / v12990) - ((v12960 * v12959) / (v13003 * v12958)))) * v12910) / v13023;
                        let v13026 = (v12764 * ((((v12953 * v32) * v83) / v12965) * ((v12955 * (v2485 * (v12978 + (((v12978 * v12978) + v12980).sqrt())))) + ((v12960 * v12919) / v12990)))).sqrt();
                        let v13027 = if v13016 > v0 { 1.0 } else { 0.0 };
                        let v13033: f64;
                        let v13035: f64;
                        if v13027 != 0.0 {
                            let v13029 = (v12764 / v13016).sqrt();
                            let v13030 = if v13026 > v0 { 1.0 } else { 0.0 };
                            let v13034: f64;
                            if v13030 != 0.0 {
                                let v13032 = (v13024 * v13029) / v13026;
                                v13034 = v13032;
                            } else {
                                v13034 = v0;
                            }
                            v13033 = v13034;
                            v13035 = v13029;
                        } else {
                            v13033 = v0;
                            v13035 = v0;
                        }
                        let v13037 = v1 - v13033;
                        let v13038 = (v13035 * v13035) * v13037;
                        let v13040 = (v13026 * v13026) * v13037;
                        v19966 = v1;
                        v19969 = v13033;
                        v19972 = v1;
                        v19975 = v13038;
                        v19978 = v1;
                        v19981 = v13040;
                    } else {
                        v19966 = v0;
                        v19969 = v0;
                        v19972 = v0;
                        v19975 = v0;
                        v19978 = v0;
                        v19981 = v0;
                    }
                    v19961 = v0;
                    v19963 = v0;
                    v19965 = v19966;
                    v19968 = v19969;
                    v19971 = v19972;
                    v19974 = v19975;
                    v19977 = v19978;
                    v19980 = v19981;
                }
                let v19983: f64;
                let v19985: f64;
                let v19987: f64;
                let v19989: f64;
                if v2886 != 0.0 {
                    let v13044 = v13041 * ((v12757 + v12751).abs());
                    let v13048 = v13045 * ((v12760 + v12753).abs());
                    v19983 = v1;
                    v19985 = v13044;
                    v19987 = v1;
                    v19989 = v13048;
                } else {
                    v19983 = v0;
                    v19985 = v0;
                    v19987 = v0;
                    v19989 = v0;
                }
                let v19991: f64;
                let v19993: f64;
                if v12544 != 0.0 {
                    let v13051 = v13049 * (v12755.abs());
                    v19991 = v1;
                    v19993 = v13051;
                } else {
                    v19991 = v0;
                    v19993 = v0;
                }
                let v13052 = if v5389 == v1 { 1.0 } else { 0.0 };
                let v19870: f64;
                let v19885: f64;
                if v13052 != 0.0 {
                    let v13059 = if ((v4690 + (v4613 * ((if (v1879 / v4637) >= v4546 { (v1879 / v4637) } else { v4546 }).ln()))) + v609) >= v4690 { ((v4690 + (v4613 * ((if (v1879 / v4637) >= v4546 { (v1879 / v4637) } else { v4546 }).ln()))) + v609) } else { v4690 };
                    let v13065 = v1 + (v1979 * v4706);
                    let v13077 = v13059 - v5546;
                    let v13079 = v13077 - v5443;
                    let v13086 = (v2485 * ((v13077 + v5443) + (((v13079 * v13079) + v13081).sqrt()))).sqrt();
                    let v13087 = ((v4697 / (v2 * v1879)).sqrt()) * v13086;
                    let v13127 = v13 + ((v5654 * v5655) / (v5654 + v5655));
                    let v13131 = (((v13127 + v1889) + (v1869 * (v2485 * (v13065 + (((v13065 * v13065) + v13067).sqrt()))))) + (((((v13099 * v5552) + ((v13101 * v5552) * v5552)) - (v13105 * v5546)) - ((v13108 * v5546) * v5546)) + (v5699 * (((((v389 + (v429 * v5552)) + ((v13114 * v5552) * v5552)) + (v399 * v5546)) + ((v13120 * v5546) * v5546)) + (((v13091 + (v13092 * v5552)) - (v13095 * v5546)) * v5542))))) / v13127;
                    let v13133 = v13131 - v1;
                    let v13139 = v2485 * ((v13131 + v1) + (((v13133 * v13133) + v13135).sqrt()));
                    let v13140 = v13139 * v4613;
                    let v13141 = v1 / v13140;
                    let v13142 = v5493 * v13141;
                    let v13143 = v5527 * v13141;
                    let v13144 = v5778 * v13141;
                    let v13145 = v209 * v13141;
                    let v13146 = v5514 * v13141;
                    let v13147 = v13086 - (v13059.sqrt());
                    let v13150 = (v2759 * v13147) - (v5489 * v5546);
                    let v13154 = (-((v13073 * (v1 + (v1989 * v4706))) + (v1929 * v5546))) * v5542;
                    let v13161 = ((v1939 + (v1949 / v73)) + (v1959 * v5546)) * ((v4616.powf(v1969)) - v1);
                    let v13165 = v4704 * (v1 + (v13162 * v5546));
                    let v13166 = if v13165 > v0 { 1.0 } else { 0.0 };
                    let v13179: f64;
                    if v13166 != 0.0 {
                        let v13169 = (v13167 * v73) / v13165;
                        let v13170 = if v13169 < v5647 { 1.0 } else { 0.0 };
                        let v13180: f64;
                        if v13170 != 0.0 {
                            let v13175 = (v2485 * v13171) / ((v13169.cosh()) - v1);
                            v13180 = v13175;
                        } else {
                            let v13178 = v13171 * (rspice_limited_exp((-v13169)));
                            v13180 = v13178;
                        }
                        v13179 = v13180;
                    } else {
                        v13179 = v0;
                    }
                    let v13183 = v13179 * (v13181 - v13059);
                    let v13184 = if v459 > v0 { 1.0 } else { 0.0 };
                    let v13202: f64;
                    if v13184 != 0.0 {
                        let v13186 = (-v479) * v5542;
                        let v13188 = if v13186 < v13187 { 1.0 } else { 0.0 };
                        let v13190: f64;
                        if v13188 != 0.0 {
                            v13190 = v5758;
                        } else {
                            let v13189 = rspice_limited_exp(v13186);
                            v13190 = v13189;
                        }
                        let v13198 = (-v13140) * ((if (v73 / (v73 + (v459 * (v1 + v13190)))) >= v4546 { (v73 / (v73 + (v459 * (v1 + v13190)))) } else { v4546 }).ln());
                        v13202 = v13198;
                    } else {
                        v13202 = v0;
                    }
                    let v13219 = (v13142 - v13144) - ((((((((v13150 + (v13202 - ((v559 + (v529 / (v73.powf(v539)))) * ((v549 * v5542).tanh())))) + v13154) - v13161) + v13183) + v13211) + v13213) + v5480) * v13141);
                    let v13220 = v13146 - v13145;
                    let v13236 = (((((v13221 * v9) * v1879) * v13141).sqrt()) / v13) * (v1 + (v13227 * (v1 + (v13228 * (v73.powf((-v13229)))))));
                    let v13237 = v1 / v13236;
                    let v13238 = v13236 * v13236;
                    let v13239 = v1 / v13238;
                    let v13240 = v13236 / v5830;
                    let v13242 = v1 + (v13240 * v5832);
                    let v13243 = v5835 * v13242;
                    let v13245 = v1 / v13240;
                    let v13246 = v13240 * v13240;
                    let v13249 = v1 / (v13244 + (v13240 * v5840));
                    let v13250 = v13220.abs();
                    let v13251 = if v13250 <= v13243 { 1.0 } else { 0.0 };
                    let v13380: f64;
                    if v13251 != 0.0 {
                        let v13252 = -v13220;
                        let v13260 = (v13252 * v13245) * (v1 + (v13240 * (v13252 / ((v13254 * v13242) * v13242))));
                        v13380 = v13260;
                    } else {
                        let v13262 = if v13220 < (-v13243) { 1.0 } else { 0.0 };
                        let v13381: f64;
                        if v13262 != 0.0 {
                            let v13263 = -v13220;
                            let v13265 = (v13244 * v13263) * v13245;
                            let v13267 = v13265 - v2979;
                            let v13272 = v2485 * ((v13265 + v2908) - (((v13267 * v13267) + v5863).sqrt()));
                            let v13273 = v13263 - v13272;
                            let v13277 = (v13273 * v13273) + (v13246 * (v13272 + v1));
                            let v13279 = (v71 * v13273) - v13246;
                            let v13283 = ((if (v13277 / v13246) >= v4546 { (v13277 / v13246) } else { v4546 }).ln()) - v13272;
                            let v13284 = v13277 + v13279;
                            let v13286 = v13279 * v13279;
                            let v13290 = (v13284 * v13284) + (v13283 * ((v2485 * v13286) - v13277));
                            let v13302 = v13272 + (((v13277 * v13284) * v13283) / (v13290 + (((((v13284 / v13290) * v13283) * v13283) * v13279) * ((v13286 * v4724) - v13277))));
                            let v13303 = rspice_limited_exp(v13302);
                            let v13304 = v13263 - v13302;
                            let v13308 = (v71 * v13304) + (v13246 * (v13303 - v1));
                            let v13313 = (v13304 * v13304) + (v13246 * ((v13302 + v1) - v13303));
                            let v13326 = -(v13302 + (v71 * (v13313 / (v13308 + (((v13308 * v13308) - (v2976 * ((v1 - ((v13246 * v2485) * v13303)) * v13313))).sqrt())))));
                            v13381 = v13326;
                        } else {
                            let v13338 = v13246 * v2485;
                            let v13345 = (v13220 + v13338) - (v13240 * (((v13220 + (v13246 * v2542)) - (v1 - (rspice_limited_exp((-((v13220 * v13245) * (v1 + (((((v13242 * v13244) * v13249) - v1) * v13249) * v13220)))))))).sqrt()));
                            let v13347 = rspice_limited_exp((-v13345));
                            let v13348 = v13220 - v13345;
                            let v13352 = (v71 * v13348) + (v13246 * (v1 - v13347));
                            let v13357 = (v13348 * v13348) - (v13246 * ((v13345 - v1) + v13347));
                            let v13368 = v13345 + (v71 * (v13357 / (v13352 + (((v13352 * v13352) - (v2976 * ((v1 - (v13338 * v13347)) * v13357))).sqrt()))));
                            v13381 = v13368;
                        }
                        v13380 = v13381;
                    }
                    let v13369 = if v13250 < v13243 { 1.0 } else { 0.0 };
                    let v13401: f64;
                    if v13369 != 0.0 {
                        let v13370 = -v13220;
                        let v13378 = (v13370 * v13245) * (v1 + (v13240 * (v13370 / ((v13372 * v13242) * v13242))));
                        v13401 = v13378;
                    } else {
                        let v13379 = v5830 * v5830;
                        let v13382 = v13220 - v13380;
                        let v13388 = rspice_limited_exp((-v13380));
                        let v13400 = v13380 - ((((((v13379 * v13382) * v13382) * v13237) * v13237) - ((v13388 + v13380) - v1)) / ((v13388 + ((v13379 * ((v71 * v13380) - (v71 * v13220))) / v13238)) - v1));
                        v13401 = v13400;
                    }
                    let v13402 = v13401 * v13140;
                    let v13404 = v1 + (v13236 * v5832);
                    let v13405 = v1 / v13404;
                    let v13408 = (v71 * v13406) / v13139;
                    let v13409 = v13408 + v13143;
                    let v13411 = rspice_limited_exp((-v13409));
                    let v13412 = v4710 * v13404;
                    let v13414 = v2 * v13413;
                    let v13421 = ((v6016 * ((v13414 * v5613) * v5613)) / (v4697 * v13140)) + (v6022 / v13140);
                    let v13422 = v5829 * v13220;
                    let v13423 = v13421 - v13422;
                    let v13430 = v13423 + (v13236 * ((((rspice_limited_exp((-v13423))) + v13423) - v1).sqrt()));
                    let v13431 = if v13423 < v13409 { 1.0 } else { 0.0 };
                    let v14026: f64;
                    if v13431 != 0.0 {
                        let v13432 = if v13219 < v13430 { 1.0 } else { 0.0 };
                        let v14027: f64;
                        if v13432 != 0.0 {
                            let v13434 = if (v13219.abs()) <= v13412 { 1.0 } else { 0.0 };
                            let v14028: f64;
                            if v13434 != 0.0 {
                                let v13444 = (v13219 * v13405) * (v1 + (((v13219 * (v1 - v13411)) * v13236) * (((v13405 * v13405) * v6039) * v5832)));
                                v14028 = v13444;
                            } else {
                                let v13446 = if v13219 < (-v13412) { 1.0 } else { 0.0 };
                                let v14029: f64;
                                if v13446 != 0.0 {
                                    let v13447 = -v13219;
                                    let v13449 = v6052 * (v13447 * v13405);
                                    let v13451 = v13449 - v2979;
                                    let v13456 = v2485 * ((v13449 + v2908) - (((v13451 * v13451) + v5863).sqrt()));
                                    let v13457 = v13447 - v13456;
                                    let v13461 = (v13457 * v13457) + (v13238 * (v13456 + v1));
                                    let v13463 = (v71 * v13457) - v13238;
                                    let v13468 = (-v13456) + ((if (v13461 * v13239) >= v4546 { (v13461 * v13239) } else { v4546 }).ln());
                                    let v13469 = v13461 + v13463;
                                    let v13471 = v13463 * v13463;
                                    let v13475 = (v13469 * v13469) + (v13468 * ((v2485 * v13471) - v13461));
                                    let v13487 = v13456 + (((v13461 * v13469) * v13468) / (v13475 + (((((v13469 / v13475) * v13468) * v13468) * v13463) * ((v13471 * v4724) - v13461))));
                                    let v13488 = rspice_limited_exp(v13487);
                                    let v13490 = v13487 * v13487;
                                    let v13492 = v1 / (v71 + v13490);
                                    let v13493 = v13490 * v13492;
                                    let v13502 = v13447 - v13487;
                                    let v13503 = v13411 * (v1 / v13488);
                                    let v13511 = (v71 * v13502) + (v13238 * (((v13488 - v1) - v13503) + (v13411 * (v1 - (v2976 * ((v13487 * v13492) * v13492))))));
                                    let v13521 = (v13502 * v13502) - (v13238 * ((((v13488 - v13487) - v1) + v13503) + (v13411 * ((v13487 - v1) - v13493))));
                                    let v13536 = (-v13487) - (v71 * (v13521 / (v13511 + (((v13511 * v13511) - (v71 * (v13521 * (v71 - (v13238 * ((v13488 + v13503) - (v13411 * ((((v3003 * v13492) - (v6103 * v13493)) * v13492) * v13492)))))))).sqrt()))));
                                    v14029 = v13536;
                                } else {
                                    let v13539 = v1 / (v6052 + (v13236 * v5840));
                                    let v13558 = (v13219 + (v13238 * v2485)) - (v13236 * (((v13219 + (v13238 * v2542)) - (v1 - (rspice_limited_exp((-((v13219 * v13405) * (v1 + (((((v13404 * v6052) * v13539) - v1) * v13539) * v13219)))))))).sqrt()));
                                    let v13559 = v13409 + v2974;
                                    let v13561 = v13558 - v13559;
                                    let v13572 = (v2485 * ((v13558 + v13559) - (((v13561 * v13561) + v2964).sqrt()))) - (v2485 * (v13559 - (((v13559 * v13559) + v2964).sqrt())));
                                    let v13573 = v13219 - v13572;
                                    let v13575 = rspice_limited_exp((-v13572));
                                    let v13576 = v13572 * v13572;
                                    let v13578 = v1 / (v71 + v13576);
                                    let v13579 = v13576 * v13578;
                                    let v13597 = if v6194 >= ((v13573 * v13573) - (v13238 * (((v13575 + v13572) - v1) - (v13411 * ((v13572 + v1) + v13579))))) { v6194 } else { ((v13573 * v13573) - (v13238 * (((v13575 + v13572) - v1) - (v13411 * ((v13572 + v1) + v13579))))) };
                                    let v13609 = (v71 * v13573) + (v13238 * ((v1 - v13575) - (v13411 * (v1 + (v2976 * ((v13572 * v13578) * v13578))))));
                                    let v13614 = (v13409 - v13572) + ((if (v13597 / v13238) >= v4546 { (v13597 / v13238) } else { v4546 }).ln());
                                    let v13615 = v13597 + v13609;
                                    let v13617 = v13609 * v13609;
                                    let v13619 = v13597 * (v1 - (v2485 * (v13238 * (v13575 - (v13411 * ((((v3003 * v13578) - (v6103 * v13579)) * v13578) * v13578))))));
                                    let v13622 = (v13615 * v13615) + (v13614 * ((v2485 * v13617) - v13619));
                                    let v13634 = v13572 + (((v13597 * v13615) * v13614) / (v13622 + (((((v13615 / v13622) * v13614) * v13614) * v13609) * ((v13617 * v4724) - v13619))));
                                    let v13636 = v1 / (rspice_limited_exp(v13634));
                                    let v13638 = rspice_limited_exp((v13634 - v13409));
                                    let v13639 = v13634 * v13634;
                                    let v13641 = v1 / (v71 + v13639);
                                    let v13642 = v13639 * v13641;
                                    let v13651 = v13219 - v13634;
                                    let v13659 = (v71 * v13651) + (v13238 * (((v1 - v13636) + v13638) - (v13411 * (v1 + (v2976 * ((v13634 * v13641) * v13641))))));
                                    let v13669 = (v13651 * v13651) - (v13238 * ((((v13636 + v13634) - v1) + v13638) - (v13411 * ((v13634 + v1) + v13642))));
                                    let v13683 = v13634 + (v71 * (v13669 / (v13659 + (((v13659 * v13659) - (v71 * (v13669 * (v71 - (v13238 * ((v13636 + v13638) - (v13411 * ((((v3003 * v13641) - (v6103 * v13642)) * v13641) * v13641)))))))).sqrt()))));
                                    v14029 = v13683;
                                }
                                v14028 = v14029;
                            }
                            v14027 = v14028;
                        } else {
                            let v13684 = v5830 * v5830;
                            let v13686 = v13423 - (v13402 * v13141);
                            let v13693 = v13219 - (v13236 * ((((rspice_limited_exp((-v13686))) + v13686) - v1).sqrt()));
                            let v13694 = v13409 + v2974;
                            let v13696 = v13693 - v13694;
                            let v13701 = v2485 * ((v13693 + v13694) - (((v13696 * v13696) + v5647).sqrt()));
                            let v13702 = v13219 - v13701;
                            let v13705 = (v13220 - v13701) + v13423;
                            let v13710 = ((v13702 * v13702) - ((v13684 * v13705) * v13705)) - (v13238 * v13423);
                            let v13712 = v71 * v13684;
                            let v13714 = (v71 * v13702) - (v13712 * v13705);
                            let v13715 = v13714 * v13714;
                            let v13716 = v1 - v13684;
                            let v13717 = if v13710 < v0 { 1.0 } else { 0.0 };
                            let v13719: f64;
                            if v13717 != 0.0 {
                                v13719 = v0;
                            } else {
                                v13719 = v13710;
                            }
                            let v13724 = v13719 + v13714;
                            let v13729 = v13719 * v13716;
                            let v13730 = (((v13724 * v13724) / ((v13409 - v13701) + ((if (v13719 * v13239) >= v4546 { (v13719 * v13239) } else { v4546 }).ln()))) + (v2485 * v13715)) - v13729;
                            let v13739 = v13701 + ((v13724 * v13719) / (v13730 + (((v13714 * v13724) / v13730) * ((v4724 * v13715) - v13729))));
                            let v13741 = rspice_limited_exp((v13739 - v13409));
                            let v13742 = v13219 - v13739;
                            let v13745 = (v13220 - v13739) + v13423;
                            let v13748 = v13238 * v13741;
                            let v13749 = ((v71 * v13742) - (v13712 * v13745)) + v13748;
                            let v13757 = v71 * (((v13742 * v13742) - ((v13684 * v13745) * v13745)) - (v13238 * (v13423 + v13741)));
                            let v13766 = v13739 + (v13757 / (v13749 + (((v13749 * v13749) - (v13757 * ((v71 - v13712) - v13748))).sqrt())));
                            v14027 = v13766;
                        }
                        v14026 = v14027;
                    } else {
                        let v13768 = if (v13219.abs()) <= v13412 { 1.0 } else { 0.0 };
                        let v14030: f64;
                        if v13768 != 0.0 {
                            let v13778 = (v13219 * v13405) * (v1 + (((v13219 * (v1 - v13411)) * v13236) * (((v13405 * v13405) * v6039) * v5832)));
                            v14030 = v13778;
                        } else {
                            let v13780 = if v13219 < (-v13412) { 1.0 } else { 0.0 };
                            let v14031: f64;
                            if v13780 != 0.0 {
                                let v13781 = -v13219;
                                let v13783 = v6052 * (v13781 * v13405);
                                let v13785 = v13783 - v2979;
                                let v13790 = v2485 * ((v13783 + v2908) - (((v13785 * v13785) + v5863).sqrt()));
                                let v13791 = v13781 - v13790;
                                let v13795 = (v13791 * v13791) + (v13238 * (v13790 + v1));
                                let v13797 = (v71 * v13791) - v13238;
                                let v13802 = (-v13790) + ((if (v13795 * v13239) >= v4546 { (v13795 * v13239) } else { v4546 }).ln());
                                let v13803 = v13795 + v13797;
                                let v13805 = v13797 * v13797;
                                let v13809 = (v13803 * v13803) + (v13802 * ((v2485 * v13805) - v13795));
                                let v13821 = v13790 + (((v13795 * v13803) * v13802) / (v13809 + (((((v13803 / v13809) * v13802) * v13802) * v13797) * ((v13805 * v4724) - v13795))));
                                let v13822 = rspice_limited_exp(v13821);
                                let v13824 = v13821 * v13821;
                                let v13826 = v1 / (v71 + v13824);
                                let v13827 = v13824 * v13826;
                                let v13836 = v13781 - v13821;
                                let v13837 = v13411 * (v1 / v13822);
                                let v13845 = (v71 * v13836) + (v13238 * (((v13822 - v1) - v13837) + (v13411 * (v1 - (v2976 * ((v13821 * v13826) * v13826))))));
                                let v13855 = (v13836 * v13836) - (v13238 * ((((v13822 - v13821) - v1) + v13837) + (v13411 * ((v13821 - v1) - v13827))));
                                let v13864 = (v13845 * v13845) - (v71 * (v13855 * (v71 - (v13238 * ((v13822 + v13837) - (v13411 * ((((v3003 * v13826) - (v6103 * v13827)) * v13826) * v13826)))))));
                                let v13874 = (-v13821) - (v71 * (v13855 / (v13845 + (((((v13864 * v13864) + v6473).sqrt()) - v6476).sqrt()))));
                                v14031 = v13874;
                            } else {
                                let v13877 = v1 / (v6052 + (v13236 * v5840));
                                let v13896 = (v13219 + (v13238 * v2485)) - (v13236 * (((v13219 + (v13238 * v2542)) - (v1 - (rspice_limited_exp((-((v13219 * v13405) * (v1 + (((((v13404 * v6052) * v13877) - v1) * v13877) * v13219)))))))).sqrt()));
                                let v13897 = v13409 + v2974;
                                let v13899 = v13896 - v13897;
                                let v13910 = (v2485 * ((v13896 + v13897) - (((v13899 * v13899) + v2964).sqrt()))) - (v2485 * (v13897 - (((v13897 * v13897) + v2964).sqrt())));
                                let v13911 = v13219 - v13910;
                                let v13913 = rspice_limited_exp((-v13910));
                                let v13914 = v13910 * v13910;
                                let v13916 = v1 / (v71 + v13914);
                                let v13917 = v13914 * v13916;
                                let v13935 = if v6194 >= ((v13911 * v13911) - (v13238 * (((v13913 + v13910) - v1) - (v13411 * ((v13910 + v1) + v13917))))) { v6194 } else { ((v13911 * v13911) - (v13238 * (((v13913 + v13910) - v1) - (v13411 * ((v13910 + v1) + v13917))))) };
                                let v13947 = (v71 * v13911) + (v13238 * ((v1 - v13913) - (v13411 * (v1 + (v2976 * ((v13910 * v13916) * v13916))))));
                                let v13952 = (v13409 - v13910) + ((if (v13935 / v13238) >= v4546 { (v13935 / v13238) } else { v4546 }).ln());
                                let v13953 = v13935 + v13947;
                                let v13955 = v13947 * v13947;
                                let v13957 = v13935 * (v1 - (v2485 * (v13238 * (v13913 - (v13411 * ((((v3003 * v13916) - (v6103 * v13917)) * v13916) * v13916))))));
                                let v13960 = (v13953 * v13953) + (v13952 * ((v2485 * v13955) - v13957));
                                let v13972 = v13910 + (((v13935 * v13953) * v13952) / (v13960 + (((((v13953 / v13960) * v13952) * v13952) * v13947) * ((v13955 * v4724) - v13957))));
                                let v13974 = v1 / (rspice_limited_exp(v13972));
                                let v13976 = rspice_limited_exp((v13972 - v13409));
                                let v13977 = v13972 * v13972;
                                let v13979 = v1 / (v71 + v13977);
                                let v13980 = v13977 * v13979;
                                let v13989 = v13219 - v13972;
                                let v13997 = (v71 * v13989) + (v13238 * (((v1 - v13974) + v13976) - (v13411 * (v1 + (v2976 * ((v13972 * v13979) * v13979))))));
                                let v14007 = (v13989 * v13989) - (v13238 * ((((v13974 + v13972) - v1) + v13976) - (v13411 * ((v13972 + v1) + v13980))));
                                let v14016 = (v13997 * v13997) - (v71 * (v14007 * (v71 - (v13238 * ((v13974 + v13976) - (v13411 * ((((v3003 * v13979) - (v6103 * v13980)) * v13979) * v13979)))))));
                                let v14025 = v13972 + (v71 * (v14007 / (v13997 + (((((v14016 * v14016) + v6473).sqrt()) - v6476).sqrt()))));
                                v14031 = v14025;
                            }
                            v14030 = v14031;
                        }
                        v14026 = v14030;
                    }
                    let v14034 = ((v13405 * v13405) * v6039) * v5832;
                    let v14042 = (((v6016 * (v13414 * v6644)) / v4697) + v6022) - ((v5829 * (v13220 * v13140)) * v349);
                    let v14043 = v6653 * v13402;
                    let v14044 = v14042 + v14043;
                    let v14045 = v13219.abs();
                    let v14046 = if v14045 <= v5835 { 1.0 } else { 0.0 };
                    let v14203: f64;
                    let v15769: f64;
                    if v14046 != 0.0 {
                        let v14053 = (v13219 * v13405) * (v1 + (((v13219 * (v1 - v13411)) * v13236) * v14034));
                        v14203 = v14053;
                        v15769 = v15770;
                    } else {
                        let v14070 = ((v13219 * v13405) * (v1 + (((v13219 * (v1 - v13411)) * v13236) * v14034))) * (v2485 * (((v14054 * (v13219 - v71)).tanh()) + ((v2964 * (v13219 + v71)).tanh())));
                        let v14073 = ((v14026 * v13140) - v14044) / v13140;
                        let v14074 = rspice_limited_exp(v14073);
                        let v14077 = ((v14070 * v13140) - v14044) / v13140;
                        let v14078 = if v14073 > v6689 { 1.0 } else { 0.0 };
                        let v14092: f64;
                        if v14078 != 0.0 {
                            v14092 = v14073;
                        } else {
                            let v14080 = if v14073 < v14079 { 1.0 } else { 0.0 };
                            let v14093: f64;
                            if v14080 != 0.0 {
                                let v14081 = v14073.exp();
                                v14093 = v14081;
                            } else {
                                let v14084 = (v1 + (v14073.exp())).ln();
                                v14093 = v14084;
                            }
                            v14092 = v14093;
                        }
                        let v14085 = if v14077 > v6689 { 1.0 } else { 0.0 };
                        let v14094: f64;
                        if v14085 != 0.0 {
                            v14094 = v14077;
                        } else {
                            let v14087 = if v14077 < v14086 { 1.0 } else { 0.0 };
                            let v14095: f64;
                            if v14087 != 0.0 {
                                let v14088 = v14077.exp();
                                v14095 = v14088;
                            } else {
                                let v14091 = (v1 + (v14077.exp())).ln();
                                v14095 = v14091;
                            }
                            v14094 = v14095;
                        }
                        let v14100 = -((v13402 / v13140) + ((v14092 - v14094) / v6653));
                        let v14101 = rspice_limited_exp(v14100);
                        let v14103 = rspice_limited_exp((-v14026));
                        let v14104 = v14026 * v14026;
                        let v14106 = v1 / (v14104 + v71);
                        let v14108 = rspice_limited_exp((v14026 - v13409));
                        let v14109 = v13219 - v14026;
                        let v14112 = v13220 + v14100;
                        let v14121 = v14106 * v14104;
                        let v14126 = ((v14109 * v14109) - (((v5830 * v5830) * v14112) * v14112)) - (v13238 * (((((v14103 - v14101) + v14026) + v14100) + v14108) - (v13411 * ((v14026 + v1) + v14121))));
                        let v14131 = v1 + v14074;
                        let v14132 = v6653 * v14131;
                        let v14136 = v71 * v14026;
                        let v14150 = v14074 / v14132;
                        let v14152 = v14074 * v14101;
                        let v14157 = (((((((v71 * v14074) * v14112) * v5830) * v5830) / v14132) - (v71 * v13219)) + v14136) - (v13238 * (((((v14108 + (v13411 * ((((v14138 * v14026) * v14106) + ((((v14136 * v14026) * v14026) * v14106) * v14106)) - v1))) - v14103) - v14150) + (v14152 / v14132)) + v1));
                        let v14160 = ((v71 * v5830) * v5830) * v14074;
                        let v14163 = v14160 * v14074;
                        let v14202 = v14026 - ((v14126 / v14157) * (v1 + ((v14126 * ((((((v14160 * v14112) / v14132) - (v14163 / ((v14132 * v6653) * v14131))) - (v13238 * (((v14103 + v14108) - (((v71 * v13411) * v14106) * (v1 - (v14121 * (v2964 - ((v2976 * v14104) * v14106)))))) - (v14150 * (((v1 - (v14074 / v14131)) - v14101) + ((v14152 / v14131) * (v1 + (v1 / v6653)))))))) - ((v14163 * v14112) / (v14132 * v14131))) + v71)) / ((v71 * v14157) * v14157))));
                        v14203 = v14202;
                        v15769 = v14070;
                    }
                    let v14360: f64;
                    let v15768: f64;
                    if v14046 != 0.0 {
                        let v14210 = (v13219 * v13405) * (v1 + (((v13219 * (v1 - v13411)) * v13236) * v14034));
                        v14360 = v14210;
                        v15768 = v15769;
                    } else {
                        let v14227 = ((v13219 * v13405) * (v1 + (((v13219 * (v1 - v13411)) * v13236) * v14034))) * (v2485 * (((v14211 * (v13219 - v71)).tanh()) + ((v2964 * (v13219 + v71)).tanh())));
                        let v14230 = ((v14203 * v13140) - v14044) / v13140;
                        let v14231 = rspice_limited_exp(v14230);
                        let v14234 = ((v14227 * v13140) - v14044) / v13140;
                        let v14235 = if v14230 > v6689 { 1.0 } else { 0.0 };
                        let v14249: f64;
                        if v14235 != 0.0 {
                            v14249 = v14230;
                        } else {
                            let v14237 = if v14230 < v14236 { 1.0 } else { 0.0 };
                            let v14250: f64;
                            if v14237 != 0.0 {
                                let v14238 = v14230.exp();
                                v14250 = v14238;
                            } else {
                                let v14241 = (v1 + (v14230.exp())).ln();
                                v14250 = v14241;
                            }
                            v14249 = v14250;
                        }
                        let v14242 = if v14234 > v6689 { 1.0 } else { 0.0 };
                        let v14251: f64;
                        if v14242 != 0.0 {
                            v14251 = v14234;
                        } else {
                            let v14244 = if v14234 < v14243 { 1.0 } else { 0.0 };
                            let v14252: f64;
                            if v14244 != 0.0 {
                                let v14245 = v14234.exp();
                                v14252 = v14245;
                            } else {
                                let v14248 = (v1 + (v14234.exp())).ln();
                                v14252 = v14248;
                            }
                            v14251 = v14252;
                        }
                        let v14257 = -((v13402 / v13140) + ((v14249 - v14251) / v6653));
                        let v14258 = rspice_limited_exp(v14257);
                        let v14260 = rspice_limited_exp((-v14203));
                        let v14261 = v14203 * v14203;
                        let v14263 = v1 / (v14261 + v71);
                        let v14265 = rspice_limited_exp((v14203 - v13409));
                        let v14266 = v13219 - v14203;
                        let v14269 = v13220 + v14257;
                        let v14278 = v14263 * v14261;
                        let v14283 = ((v14266 * v14266) - (((v5830 * v5830) * v14269) * v14269)) - (v13238 * (((((v14260 - v14258) + v14203) + v14257) + v14265) - (v13411 * ((v14203 + v1) + v14278))));
                        let v14288 = v1 + v14231;
                        let v14289 = v6653 * v14288;
                        let v14293 = v71 * v14203;
                        let v14307 = v14231 / v14289;
                        let v14309 = v14231 * v14258;
                        let v14314 = (((((((v71 * v14231) * v14269) * v5830) * v5830) / v14289) - (v71 * v13219)) + v14293) - (v13238 * (((((v14265 + (v13411 * ((((v14295 * v14203) * v14263) + ((((v14293 * v14203) * v14203) * v14263) * v14263)) - v1))) - v14260) - v14307) + (v14309 / v14289)) + v1));
                        let v14317 = ((v71 * v5830) * v5830) * v14231;
                        let v14320 = v14317 * v14231;
                        let v14359 = v14203 - ((v14283 / v14314) * (v1 + ((v14283 * ((((((v14317 * v14269) / v14289) - (v14320 / ((v14289 * v6653) * v14288))) - (v13238 * (((v14260 + v14265) - (((v71 * v13411) * v14263) * (v1 - (v14278 * (v2964 - ((v2976 * v14261) * v14263)))))) - (v14307 * (((v1 - (v14231 / v14288)) - v14258) + ((v14309 / v14288) * (v1 + (v1 / v6653)))))))) - ((v14320 * v14269) / (v14289 * v14288))) + v71)) / ((v71 * v14314) * v14314))));
                        v14360 = v14359;
                        v15768 = v14227;
                    }
                    let v14517: f64;
                    let v15767: f64;
                    if v14046 != 0.0 {
                        let v14367 = (v13219 * v13405) * (v1 + (((v13219 * (v1 - v13411)) * v13236) * v14034));
                        v14517 = v14367;
                        v15767 = v15768;
                    } else {
                        let v14384 = ((v13219 * v13405) * (v1 + (((v13219 * (v1 - v13411)) * v13236) * v14034))) * (v2485 * (((v14368 * (v13219 - v71)).tanh()) + ((v2964 * (v13219 + v71)).tanh())));
                        let v14387 = ((v14360 * v13140) - v14044) / v13140;
                        let v14388 = rspice_limited_exp(v14387);
                        let v14391 = ((v14384 * v13140) - v14044) / v13140;
                        let v14392 = if v14387 > v6689 { 1.0 } else { 0.0 };
                        let v14406: f64;
                        if v14392 != 0.0 {
                            v14406 = v14387;
                        } else {
                            let v14394 = if v14387 < v14393 { 1.0 } else { 0.0 };
                            let v14407: f64;
                            if v14394 != 0.0 {
                                let v14395 = v14387.exp();
                                v14407 = v14395;
                            } else {
                                let v14398 = (v1 + (v14387.exp())).ln();
                                v14407 = v14398;
                            }
                            v14406 = v14407;
                        }
                        let v14399 = if v14391 > v6689 { 1.0 } else { 0.0 };
                        let v14408: f64;
                        if v14399 != 0.0 {
                            v14408 = v14391;
                        } else {
                            let v14401 = if v14391 < v14400 { 1.0 } else { 0.0 };
                            let v14409: f64;
                            if v14401 != 0.0 {
                                let v14402 = v14391.exp();
                                v14409 = v14402;
                            } else {
                                let v14405 = (v1 + (v14391.exp())).ln();
                                v14409 = v14405;
                            }
                            v14408 = v14409;
                        }
                        let v14414 = -((v13402 / v13140) + ((v14406 - v14408) / v6653));
                        let v14415 = rspice_limited_exp(v14414);
                        let v14417 = rspice_limited_exp((-v14360));
                        let v14418 = v14360 * v14360;
                        let v14420 = v1 / (v14418 + v71);
                        let v14422 = rspice_limited_exp((v14360 - v13409));
                        let v14423 = v13219 - v14360;
                        let v14426 = v13220 + v14414;
                        let v14435 = v14420 * v14418;
                        let v14440 = ((v14423 * v14423) - (((v5830 * v5830) * v14426) * v14426)) - (v13238 * (((((v14417 - v14415) + v14360) + v14414) + v14422) - (v13411 * ((v14360 + v1) + v14435))));
                        let v14445 = v1 + v14388;
                        let v14446 = v6653 * v14445;
                        let v14450 = v71 * v14360;
                        let v14464 = v14388 / v14446;
                        let v14466 = v14388 * v14415;
                        let v14471 = (((((((v71 * v14388) * v14426) * v5830) * v5830) / v14446) - (v71 * v13219)) + v14450) - (v13238 * (((((v14422 + (v13411 * ((((v14452 * v14360) * v14420) + ((((v14450 * v14360) * v14360) * v14420) * v14420)) - v1))) - v14417) - v14464) + (v14466 / v14446)) + v1));
                        let v14474 = ((v71 * v5830) * v5830) * v14388;
                        let v14477 = v14474 * v14388;
                        let v14516 = v14360 - ((v14440 / v14471) * (v1 + ((v14440 * ((((((v14474 * v14426) / v14446) - (v14477 / ((v14446 * v6653) * v14445))) - (v13238 * (((v14417 + v14422) - (((v71 * v13411) * v14420) * (v1 - (v14435 * (v2964 - ((v2976 * v14418) * v14420)))))) - (v14464 * (((v1 - (v14388 / v14445)) - v14415) + ((v14466 / v14445) * (v1 + (v1 / v6653)))))))) - ((v14477 * v14426) / (v14446 * v14445))) + v71)) / ((v71 * v14471) * v14471))));
                        v14517 = v14516;
                        v15767 = v14384;
                    }
                    let v14518 = if v14517 <= v0 { 1.0 } else { 0.0 };
                    let v19871: f64;
                    let v19886: f64;
                    if v14518 != 0.0 {
                        v19871 = v7135;
                        v19886 = v7138;
                    } else {
                        let v14519 = v14517 * v14517;
                        let v14524 = v1 / (rspice_limited_exp(v14517));
                        let v14530 = (rspice_limited_exp((v14517 - v13409))) - (v13411 * ((v14517 + v1) + (v14519 * (v1 / (v71 + v14519)))));
                        let v14531 = v13219 - v14517;
                        let v14535 = (((v14531 * v14531) * v13239) - v14530) - v4710;
                        let v14542 = (v2485 * (v14535 + (((v14535 * v14535) + v14537).sqrt()))) + v4710;
                        let v14549 = v13236 * (v14542.sqrt());
                        let v14551 = ((v13238 * v14530) * v13140) / ((v13236 * ((v14542 + v14530).sqrt())) + v14549);
                        let v14552 = v14549 * v13140;
                        let v14570 = v1 + (((v7189 + (v7190 * v5546)) * (((v7176 / v5617) * (v14552 + (v4728 * v14551))).powf(v4768))) + (v7195 / (rspice_limited_exp((v7181 * ((if (v2485 * (v1 + (v14551 / v14552))) >= v4546 { (v2485 * (v1 + (v14551 / v14552))) } else { v4546 }).ln()))))));
                        let v14572 = v14570 - v1;
                        let v14578 = v2485 * ((v14570 + v1) + (((v14572 * v14572) + v14574).sqrt()));
                        let v14582 = v1 / (((v83 * v7207).powf(v769)) * v32);
                        let v14607: f64;
                        if v2834 != 0.0 {
                            v14607 = v0;
                        } else {
                            let v14587 = (v1 / (v1 + (v749 * v14551))) + (v2812 * v13147);
                            let v14595 = ((v7221 + (v7223 * (v14587 + (((v14587 * v14587) + v4979).sqrt())))) * v14582) * v32;
                            let v14596 = v14595 * v4795;
                            let v14597 = if v2833 == v71 { 1.0 } else { 0.0 };
                            let v14608: f64;
                            if v14597 != 0.0 {
                                let v14600 = ((v7138 + v14595) + v7135) * v4795;
                                v14608 = v14600;
                            } else {
                                v14608 = v14596;
                            }
                            v14607 = v14608;
                        }
                        let v14603 = ((v12765 / v7236) * v14578) * v73;
                        let v14606 = v919 * (v14551 + (v71 * v13140));
                        let v14609 = if v14607 > v0 { 1.0 } else { 0.0 };
                        let v14657: f64;
                        if v14609 != 0.0 {
                            let v14612 = ((v83 * v7234) * v13) * v14607;
                            let v14613 = v71 * v14612;
                            let v14617 = (v14606 + v14603) + ((v2974 * v14606) * v14612);
                            let v14628 = (v14617 - (((v14617 * v14617) - ((v71 * v14613) * (v14606 * (v14603 + ((v71 * v14606) * v14612))))).sqrt())) / v14613;
                            v14657 = v14628;
                        } else {
                            let v14631 = (v14603 * v14606) / (v14603 + v14606);
                            v14657 = v14631;
                        }
                        let v14634 = if (if v7268 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v7270 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v14666: f64;
                        if v14634 != 0.0 {
                            v14666 = v1;
                        } else {
                            let v14638 = v73 / (v73 + ((v599 * v13087).sqrt()));
                            let v14648 = v1 + (((v7268 * v14638) - (((v7270 * v14638) * (v14551.powf(v7279))) * v13140)) / (v1 + (v7284 * v5546)));
                            let v14650 = v14648 - v5425;
                            let v14656 = v2485 * ((v14648 + v5425) + (((v14650 * v14650) + v14652).sqrt()));
                            v14666 = v14656;
                        }
                        let v14658 = v14657 - v4710;
                        let v14678 = v13408 + (((v5528 * ((v1 + (((v5528 / (((v2485 * (v14658 + (((v14658 * v14658) + v14660).sqrt()))) + v4710) / v14666)) + v127).powf((v1 / v4819)))).powf((-v4819)))) + v5527) * v13141);
                        let v14679 = -v14678;
                        let v14680 = rspice_limited_exp(v14679);
                        let v14681 = v5528 * v13141;
                        let v14684 = v6014 * (v14681 + (v329 * v13141));
                        let v14688 = (v13421 - (v13422 * v349)) + (v6653 * v14684);
                        let v14695 = v14688 + (v13236 * ((((rspice_limited_exp((-v14688))) + v14688) - v1).sqrt()));
                        let v14696 = if v14688 < v14678 { 1.0 } else { 0.0 };
                        let v15275: f64;
                        if v14696 != 0.0 {
                            let v14697 = if v13219 < v14695 { 1.0 } else { 0.0 };
                            let v15276: f64;
                            if v14697 != 0.0 {
                                let v14698 = if v14045 <= v13412 { 1.0 } else { 0.0 };
                                let v15277: f64;
                                if v14698 != 0.0 {
                                    let v14705 = (v13219 * v13405) * (v1 + (((v13219 * (v1 - v14680)) * v13236) * v14034));
                                    v15277 = v14705;
                                } else {
                                    let v14707 = if v13219 < (-v13412) { 1.0 } else { 0.0 };
                                    let v15278: f64;
                                    if v14707 != 0.0 {
                                        let v14708 = -v13219;
                                        let v14710 = v6052 * (v14708 * v13405);
                                        let v14712 = v14710 - v2979;
                                        let v14717 = v2485 * ((v14710 + v2908) - (((v14712 * v14712) + v5863).sqrt()));
                                        let v14718 = v14708 - v14717;
                                        let v14722 = (v14718 * v14718) + (v13238 * (v14717 + v1));
                                        let v14724 = (v71 * v14718) - v13238;
                                        let v14729 = (-v14717) + ((if (v14722 * v13239) >= v4546 { (v14722 * v13239) } else { v4546 }).ln());
                                        let v14730 = v14722 + v14724;
                                        let v14732 = v14724 * v14724;
                                        let v14736 = (v14730 * v14730) + (v14729 * ((v2485 * v14732) - v14722));
                                        let v14748 = v14717 + (((v14722 * v14730) * v14729) / (v14736 + (((((v14730 / v14736) * v14729) * v14729) * v14724) * ((v14732 * v4724) - v14722))));
                                        let v14749 = rspice_limited_exp(v14748);
                                        let v14751 = v14748 * v14748;
                                        let v14753 = v1 / (v71 + v14751);
                                        let v14754 = v14751 * v14753;
                                        let v14763 = v14708 - v14748;
                                        let v14764 = v14680 * (v1 / v14749);
                                        let v14772 = (v71 * v14763) + (v13238 * (((v14749 - v1) - v14764) + (v14680 * (v1 - (v2976 * ((v14748 * v14753) * v14753))))));
                                        let v14782 = (v14763 * v14763) - (v13238 * ((((v14749 - v14748) - v1) + v14764) + (v14680 * ((v14748 - v1) - v14754))));
                                        let v14797 = (-v14748) - (v71 * (v14782 / (v14772 + (((v14772 * v14772) - (v71 * (v14782 * (v71 - (v13238 * ((v14749 + v14764) - (v14680 * ((((v3003 * v14753) - (v6103 * v14754)) * v14753) * v14753)))))))).sqrt()))));
                                        v15278 = v14797;
                                    } else {
                                        let v14800 = v1 / (v6052 + (v13236 * v5840));
                                        let v14819 = (v13219 + (v13238 * v2485)) - (v13236 * (((v13219 + (v13238 * v2542)) - (v1 - (rspice_limited_exp((-((v13219 * v13405) * (v1 + (((((v13404 * v6052) * v14800) - v1) * v14800) * v13219)))))))).sqrt()));
                                        let v14820 = v14678 + v2974;
                                        let v14822 = v14819 - v14820;
                                        let v14833 = (v2485 * ((v14819 + v14820) - (((v14822 * v14822) + v2964).sqrt()))) - (v2485 * (v14820 - (((v14820 * v14820) + v2964).sqrt())));
                                        let v14834 = v13219 - v14833;
                                        let v14836 = rspice_limited_exp((-v14833));
                                        let v14837 = v14833 * v14833;
                                        let v14839 = v1 / (v71 + v14837);
                                        let v14840 = v14837 * v14839;
                                        let v14858 = if v6194 >= ((v14834 * v14834) - (v13238 * (((v14836 + v14833) - v1) - (v14680 * ((v14833 + v1) + v14840))))) { v6194 } else { ((v14834 * v14834) - (v13238 * (((v14836 + v14833) - v1) - (v14680 * ((v14833 + v1) + v14840))))) };
                                        let v14870 = (v71 * v14834) + (v13238 * ((v1 - v14836) - (v14680 * (v1 + (v2976 * ((v14833 * v14839) * v14839))))));
                                        let v14875 = (v14678 - v14833) + ((if (v14858 / v13238) >= v4546 { (v14858 / v13238) } else { v4546 }).ln());
                                        let v14876 = v14858 + v14870;
                                        let v14878 = v14870 * v14870;
                                        let v14880 = v14858 * (v1 - (v2485 * (v13238 * (v14836 - (v14680 * ((((v3003 * v14839) - (v6103 * v14840)) * v14839) * v14839))))));
                                        let v14883 = (v14876 * v14876) + (v14875 * ((v2485 * v14878) - v14880));
                                        let v14895 = v14833 + (((v14858 * v14876) * v14875) / (v14883 + (((((v14876 / v14883) * v14875) * v14875) * v14870) * ((v14878 * v4724) - v14880))));
                                        let v14897 = v1 / (rspice_limited_exp(v14895));
                                        let v14899 = rspice_limited_exp((v14895 - v14678));
                                        let v14900 = v14895 * v14895;
                                        let v14902 = v1 / (v71 + v14900);
                                        let v14903 = v14900 * v14902;
                                        let v14912 = v13219 - v14895;
                                        let v14920 = (v71 * v14912) + (v13238 * (((v1 - v14897) + v14899) - (v14680 * (v1 + (v2976 * ((v14895 * v14902) * v14902))))));
                                        let v14930 = (v14912 * v14912) - (v13238 * ((((v14897 + v14895) - v1) + v14899) - (v14680 * ((v14895 + v1) + v14903))));
                                        let v14944 = v14895 + (v71 * (v14930 / (v14920 + (((v14920 * v14920) - (v71 * (v14930 * (v71 - (v13238 * ((v14897 + v14899) - (v14680 * ((((v3003 * v14902) - (v6103 * v14903)) * v14902) * v14902)))))))).sqrt()))));
                                        v15278 = v14944;
                                    }
                                    v15277 = v15278;
                                }
                                v15276 = v15277;
                            } else {
                                let v14945 = v5830 * v5830;
                                let v14947 = v14688 - (v13402 * v13141);
                                let v14954 = v13219 - (v13236 * ((((rspice_limited_exp((-v14947))) + v14947) - v1).sqrt()));
                                let v14955 = v14678 + v2974;
                                let v14957 = v14954 - v14955;
                                let v14962 = v2485 * ((v14954 + v14955) - (((v14957 * v14957) + v5647).sqrt()));
                                let v14963 = v13219 - v14962;
                                let v14966 = (v13220 - v14962) + v14688;
                                let v14971 = ((v14963 * v14963) - ((v14945 * v14966) * v14966)) - (v13238 * v14688);
                                let v14973 = v71 * v14945;
                                let v14975 = (v71 * v14963) - (v14973 * v14966);
                                let v14976 = v14975 * v14975;
                                let v14977 = v1 - v14945;
                                let v14978 = if v14971 < v0 { 1.0 } else { 0.0 };
                                let v14980: f64;
                                if v14978 != 0.0 {
                                    v14980 = v0;
                                } else {
                                    v14980 = v14971;
                                }
                                let v14985 = v14980 + v14975;
                                let v14990 = v14980 * v14977;
                                let v14991 = (((v14985 * v14985) / ((v14678 - v14962) + ((if (v14980 * v13239) >= v4546 { (v14980 * v13239) } else { v4546 }).ln()))) + (v2485 * v14976)) - v14990;
                                let v15000 = v14962 + ((v14985 * v14980) / (v14991 + (((v14975 * v14985) / v14991) * ((v4724 * v14976) - v14990))));
                                let v15002 = rspice_limited_exp((v15000 - v14678));
                                let v15003 = v13219 - v15000;
                                let v15006 = (v13220 - v15000) + v14688;
                                let v15009 = v13238 * v15002;
                                let v15010 = ((v71 * v15003) - (v14973 * v15006)) + v15009;
                                let v15018 = v71 * (((v15003 * v15003) - ((v14945 * v15006) * v15006)) - (v13238 * (v14688 + v15002)));
                                let v15027 = v15000 + (v15018 / (v15010 + (((v15010 * v15010) - (v15018 * ((v71 - v14973) - v15009))).sqrt())));
                                v15276 = v15027;
                            }
                            v15275 = v15276;
                        } else {
                            let v15028 = if v14045 <= v13412 { 1.0 } else { 0.0 };
                            let v15279: f64;
                            if v15028 != 0.0 {
                                let v15035 = (v13219 * v13405) * (v1 + (((v13219 * (v1 - v14680)) * v13236) * v14034));
                                v15279 = v15035;
                            } else {
                                let v15037 = if v13219 < (-v13412) { 1.0 } else { 0.0 };
                                let v15280: f64;
                                if v15037 != 0.0 {
                                    let v15038 = -v13219;
                                    let v15040 = v6052 * (v15038 * v13405);
                                    let v15042 = v15040 - v2979;
                                    let v15047 = v2485 * ((v15040 + v2908) - (((v15042 * v15042) + v5863).sqrt()));
                                    let v15048 = v15038 - v15047;
                                    let v15052 = (v15048 * v15048) + (v13238 * (v15047 + v1));
                                    let v15054 = (v71 * v15048) - v13238;
                                    let v15059 = (-v15047) + ((if (v15052 * v13239) >= v4546 { (v15052 * v13239) } else { v4546 }).ln());
                                    let v15060 = v15052 + v15054;
                                    let v15062 = v15054 * v15054;
                                    let v15066 = (v15060 * v15060) + (v15059 * ((v2485 * v15062) - v15052));
                                    let v15078 = v15047 + (((v15052 * v15060) * v15059) / (v15066 + (((((v15060 / v15066) * v15059) * v15059) * v15054) * ((v15062 * v4724) - v15052))));
                                    let v15079 = rspice_limited_exp(v15078);
                                    let v15081 = v15078 * v15078;
                                    let v15083 = v1 / (v71 + v15081);
                                    let v15084 = v15081 * v15083;
                                    let v15093 = v15038 - v15078;
                                    let v15094 = v14680 * (v1 / v15079);
                                    let v15102 = (v71 * v15093) + (v13238 * (((v15079 - v1) - v15094) + (v14680 * (v1 - (v2976 * ((v15078 * v15083) * v15083))))));
                                    let v15112 = (v15093 * v15093) - (v13238 * ((((v15079 - v15078) - v1) + v15094) + (v14680 * ((v15078 - v1) - v15084))));
                                    let v15127 = (-v15078) - (v71 * (v15112 / (v15102 + (((v15102 * v15102) - (v71 * (v15112 * (v71 - (v13238 * ((v15079 + v15094) - (v14680 * ((((v3003 * v15083) - (v6103 * v15084)) * v15083) * v15083)))))))).sqrt()))));
                                    v15280 = v15127;
                                } else {
                                    let v15130 = v1 / (v6052 + (v13236 * v5840));
                                    let v15149 = (v13219 + (v13238 * v2485)) - (v13236 * (((v13219 + (v13238 * v2542)) - (v1 - (rspice_limited_exp((-((v13219 * v13405) * (v1 + (((((v13404 * v6052) * v15130) - v1) * v15130) * v13219)))))))).sqrt()));
                                    let v15150 = v14678 + v2974;
                                    let v15152 = v15149 - v15150;
                                    let v15163 = (v2485 * ((v15149 + v15150) - (((v15152 * v15152) + v2964).sqrt()))) - (v2485 * (v15150 - (((v15150 * v15150) + v2964).sqrt())));
                                    let v15164 = v13219 - v15163;
                                    let v15166 = rspice_limited_exp((-v15163));
                                    let v15167 = v15163 * v15163;
                                    let v15169 = v1 / (v71 + v15167);
                                    let v15170 = v15167 * v15169;
                                    let v15188 = if v6194 >= ((v15164 * v15164) - (v13238 * (((v15166 + v15163) - v1) - (v14680 * ((v15163 + v1) + v15170))))) { v6194 } else { ((v15164 * v15164) - (v13238 * (((v15166 + v15163) - v1) - (v14680 * ((v15163 + v1) + v15170))))) };
                                    let v15200 = (v71 * v15164) + (v13238 * ((v1 - v15166) - (v14680 * (v1 + (v2976 * ((v15163 * v15169) * v15169))))));
                                    let v15205 = (v14678 - v15163) + ((if (v15188 / v13238) >= v4546 { (v15188 / v13238) } else { v4546 }).ln());
                                    let v15206 = v15188 + v15200;
                                    let v15208 = v15200 * v15200;
                                    let v15210 = v15188 * (v1 - (v2485 * (v13238 * (v15166 - (v14680 * ((((v3003 * v15169) - (v6103 * v15170)) * v15169) * v15169))))));
                                    let v15213 = (v15206 * v15206) + (v15205 * ((v2485 * v15208) - v15210));
                                    let v15225 = v15163 + (((v15188 * v15206) * v15205) / (v15213 + (((((v15206 / v15213) * v15205) * v15205) * v15200) * ((v15208 * v4724) - v15210))));
                                    let v15227 = v1 / (rspice_limited_exp(v15225));
                                    let v15229 = rspice_limited_exp((v15225 - v14678));
                                    let v15230 = v15225 * v15225;
                                    let v15232 = v1 / (v71 + v15230);
                                    let v15233 = v15230 * v15232;
                                    let v15242 = v13219 - v15225;
                                    let v15250 = (v71 * v15242) + (v13238 * (((v1 - v15227) + v15229) - (v14680 * (v1 + (v2976 * ((v15225 * v15232) * v15232))))));
                                    let v15260 = (v15242 * v15242) - (v13238 * ((((v15227 + v15225) - v1) + v15229) - (v14680 * ((v15225 + v1) + v15233))));
                                    let v15274 = v15225 + (v71 * (v15260 / (v15250 + (((v15250 * v15250) - (v71 * (v15260 * (v71 - (v13238 * ((v15227 + v15229) - (v14680 * ((((v3003 * v15232) - (v6103 * v15233)) * v15232) * v15232)))))))).sqrt()))));
                                    v15280 = v15274;
                                }
                                v15279 = v15280;
                            }
                            v15275 = v15279;
                        }
                        let v15284 = (v14042 + ((v6653 * v14684) * v13140)) + v14043;
                        let v15441: f64;
                        let v15766: f64;
                        if v14046 != 0.0 {
                            let v15291 = (v13219 * v13405) * (v1 + (((v13219 * (v1 - v14680)) * v13236) * v14034));
                            v15441 = v15291;
                            v15766 = v15767;
                        } else {
                            let v15308 = ((v13219 * v13405) * (v1 + (((v13219 * (v1 - v14680)) * v13236) * v14034))) * (v2485 * (((v15292 * (v13219 - v71)).tanh()) + ((v2964 * (v13219 + v71)).tanh())));
                            let v15311 = ((v15275 * v13140) - v15284) / v13140;
                            let v15312 = rspice_limited_exp(v15311);
                            let v15315 = ((v15308 * v13140) - v15284) / v13140;
                            let v15316 = if v15311 > v6689 { 1.0 } else { 0.0 };
                            let v15330: f64;
                            if v15316 != 0.0 {
                                v15330 = v15311;
                            } else {
                                let v15318 = if v15311 < v15317 { 1.0 } else { 0.0 };
                                let v15331: f64;
                                if v15318 != 0.0 {
                                    let v15319 = v15311.exp();
                                    v15331 = v15319;
                                } else {
                                    let v15322 = (v1 + (v15311.exp())).ln();
                                    v15331 = v15322;
                                }
                                v15330 = v15331;
                            }
                            let v15323 = if v15315 > v6689 { 1.0 } else { 0.0 };
                            let v15332: f64;
                            if v15323 != 0.0 {
                                v15332 = v15315;
                            } else {
                                let v15325 = if v15315 < v15324 { 1.0 } else { 0.0 };
                                let v15333: f64;
                                if v15325 != 0.0 {
                                    let v15326 = v15315.exp();
                                    v15333 = v15326;
                                } else {
                                    let v15329 = (v1 + (v15315.exp())).ln();
                                    v15333 = v15329;
                                }
                                v15332 = v15333;
                            }
                            let v15338 = -((v13402 / v13140) + ((v15330 - v15332) / v6653));
                            let v15339 = rspice_limited_exp(v15338);
                            let v15341 = rspice_limited_exp((-v15275));
                            let v15342 = v15275 * v15275;
                            let v15344 = v1 / (v15342 + v71);
                            let v15346 = rspice_limited_exp((v15275 - v14678));
                            let v15347 = v13219 - v15275;
                            let v15350 = v13220 + v15338;
                            let v15359 = v15344 * v15342;
                            let v15364 = ((v15347 * v15347) - (((v5830 * v5830) * v15350) * v15350)) - (v13238 * (((((v15341 - v15339) + v15275) + v15338) + v15346) - (v14680 * ((v15275 + v1) + v15359))));
                            let v15369 = v1 + v15312;
                            let v15370 = v6653 * v15369;
                            let v15374 = v71 * v15275;
                            let v15388 = v15312 / v15370;
                            let v15390 = v15312 * v15339;
                            let v15395 = (((((((v71 * v15312) * v15350) * v5830) * v5830) / v15370) - (v71 * v13219)) + v15374) - (v13238 * (((((v15346 + (v14680 * ((((v15376 * v15275) * v15344) + ((((v15374 * v15275) * v15275) * v15344) * v15344)) - v1))) - v15341) - v15388) + (v15390 / v15370)) + v1));
                            let v15398 = ((v71 * v5830) * v5830) * v15312;
                            let v15401 = v15398 * v15312;
                            let v15440 = v15275 - ((v15364 / v15395) * (v1 + ((v15364 * ((((((v15398 * v15350) / v15370) - (v15401 / ((v15370 * v6653) * v15369))) - (v13238 * (((v15341 + v15346) - (((v71 * v14680) * v15344) * (v1 - (v15359 * (v2964 - ((v2976 * v15342) * v15344)))))) - (v15388 * (((v1 - (v15312 / v15369)) - v15339) + ((v15390 / v15369) * (v1 + (v1 / v6653)))))))) - ((v15401 * v15350) / (v15370 * v15369))) + v71)) / ((v71 * v15395) * v15395))));
                            v15441 = v15440;
                            v15766 = v15308;
                        }
                        let v15598: f64;
                        let v15765: f64;
                        if v14046 != 0.0 {
                            let v15448 = (v13219 * v13405) * (v1 + (((v13219 * (v1 - v14680)) * v13236) * v14034));
                            v15598 = v15448;
                            v15765 = v15766;
                        } else {
                            let v15465 = ((v13219 * v13405) * (v1 + (((v13219 * (v1 - v14680)) * v13236) * v14034))) * (v2485 * (((v15449 * (v13219 - v71)).tanh()) + ((v2964 * (v13219 + v71)).tanh())));
                            let v15468 = ((v15441 * v13140) - v15284) / v13140;
                            let v15469 = rspice_limited_exp(v15468);
                            let v15472 = ((v15465 * v13140) - v15284) / v13140;
                            let v15473 = if v15468 > v6689 { 1.0 } else { 0.0 };
                            let v15487: f64;
                            if v15473 != 0.0 {
                                v15487 = v15468;
                            } else {
                                let v15475 = if v15468 < v15474 { 1.0 } else { 0.0 };
                                let v15488: f64;
                                if v15475 != 0.0 {
                                    let v15476 = v15468.exp();
                                    v15488 = v15476;
                                } else {
                                    let v15479 = (v1 + (v15468.exp())).ln();
                                    v15488 = v15479;
                                }
                                v15487 = v15488;
                            }
                            let v15480 = if v15472 > v6689 { 1.0 } else { 0.0 };
                            let v15489: f64;
                            if v15480 != 0.0 {
                                v15489 = v15472;
                            } else {
                                let v15482 = if v15472 < v15481 { 1.0 } else { 0.0 };
                                let v15490: f64;
                                if v15482 != 0.0 {
                                    let v15483 = v15472.exp();
                                    v15490 = v15483;
                                } else {
                                    let v15486 = (v1 + (v15472.exp())).ln();
                                    v15490 = v15486;
                                }
                                v15489 = v15490;
                            }
                            let v15495 = -((v13402 / v13140) + ((v15487 - v15489) / v6653));
                            let v15496 = rspice_limited_exp(v15495);
                            let v15498 = rspice_limited_exp((-v15441));
                            let v15499 = v15441 * v15441;
                            let v15501 = v1 / (v15499 + v71);
                            let v15503 = rspice_limited_exp((v15441 - v14678));
                            let v15504 = v13219 - v15441;
                            let v15507 = v13220 + v15495;
                            let v15516 = v15501 * v15499;
                            let v15521 = ((v15504 * v15504) - (((v5830 * v5830) * v15507) * v15507)) - (v13238 * (((((v15498 - v15496) + v15441) + v15495) + v15503) - (v14680 * ((v15441 + v1) + v15516))));
                            let v15526 = v1 + v15469;
                            let v15527 = v6653 * v15526;
                            let v15531 = v71 * v15441;
                            let v15545 = v15469 / v15527;
                            let v15547 = v15469 * v15496;
                            let v15552 = (((((((v71 * v15469) * v15507) * v5830) * v5830) / v15527) - (v71 * v13219)) + v15531) - (v13238 * (((((v15503 + (v14680 * ((((v15533 * v15441) * v15501) + ((((v15531 * v15441) * v15441) * v15501) * v15501)) - v1))) - v15498) - v15545) + (v15547 / v15527)) + v1));
                            let v15555 = ((v71 * v5830) * v5830) * v15469;
                            let v15558 = v15555 * v15469;
                            let v15597 = v15441 - ((v15521 / v15552) * (v1 + ((v15521 * ((((((v15555 * v15507) / v15527) - (v15558 / ((v15527 * v6653) * v15526))) - (v13238 * (((v15498 + v15503) - (((v71 * v14680) * v15501) * (v1 - (v15516 * (v2964 - ((v2976 * v15499) * v15501)))))) - (v15545 * (((v1 - (v15469 / v15526)) - v15496) + ((v15547 / v15526) * (v1 + (v1 / v6653)))))))) - ((v15558 * v15507) / (v15527 * v15526))) + v71)) / ((v71 * v15552) * v15552))));
                            v15598 = v15597;
                            v15765 = v15465;
                        }
                        let v15755: f64;
                        let v15764: f64;
                        if v14046 != 0.0 {
                            let v15605 = (v13219 * v13405) * (v1 + (((v13219 * (v1 - v14680)) * v13236) * v14034));
                            v15755 = v15605;
                            v15764 = v15765;
                        } else {
                            let v15622 = ((v13219 * v13405) * (v1 + (((v13219 * (v1 - v14680)) * v13236) * v14034))) * (v2485 * (((v15606 * (v13219 - v71)).tanh()) + ((v2964 * (v13219 + v71)).tanh())));
                            let v15625 = ((v15598 * v13140) - v15284) / v13140;
                            let v15626 = rspice_limited_exp(v15625);
                            let v15629 = ((v15622 * v13140) - v15284) / v13140;
                            let v15630 = if v15625 > v6689 { 1.0 } else { 0.0 };
                            let v15644: f64;
                            if v15630 != 0.0 {
                                v15644 = v15625;
                            } else {
                                let v15632 = if v15625 < v15631 { 1.0 } else { 0.0 };
                                let v15645: f64;
                                if v15632 != 0.0 {
                                    let v15633 = v15625.exp();
                                    v15645 = v15633;
                                } else {
                                    let v15636 = (v1 + (v15625.exp())).ln();
                                    v15645 = v15636;
                                }
                                v15644 = v15645;
                            }
                            let v15637 = if v15629 > v6689 { 1.0 } else { 0.0 };
                            let v15646: f64;
                            if v15637 != 0.0 {
                                v15646 = v15629;
                            } else {
                                let v15639 = if v15629 < v15638 { 1.0 } else { 0.0 };
                                let v15647: f64;
                                if v15639 != 0.0 {
                                    let v15640 = v15629.exp();
                                    v15647 = v15640;
                                } else {
                                    let v15643 = (v1 + (v15629.exp())).ln();
                                    v15647 = v15643;
                                }
                                v15646 = v15647;
                            }
                            let v15652 = -((v13402 / v13140) + ((v15644 - v15646) / v6653));
                            let v15653 = rspice_limited_exp(v15652);
                            let v15655 = rspice_limited_exp((-v15598));
                            let v15656 = v15598 * v15598;
                            let v15658 = v1 / (v15656 + v71);
                            let v15660 = rspice_limited_exp((v15598 - v14678));
                            let v15661 = v13219 - v15598;
                            let v15664 = v13220 + v15652;
                            let v15673 = v15658 * v15656;
                            let v15678 = ((v15661 * v15661) - (((v5830 * v5830) * v15664) * v15664)) - (v13238 * (((((v15655 - v15653) + v15598) + v15652) + v15660) - (v14680 * ((v15598 + v1) + v15673))));
                            let v15683 = v1 + v15626;
                            let v15684 = v6653 * v15683;
                            let v15688 = v71 * v15598;
                            let v15702 = v15626 / v15684;
                            let v15704 = v15626 * v15653;
                            let v15709 = (((((((v71 * v15626) * v15664) * v5830) * v5830) / v15684) - (v71 * v13219)) + v15688) - (v13238 * (((((v15660 + (v14680 * ((((v15690 * v15598) * v15658) + ((((v15688 * v15598) * v15598) * v15658) * v15658)) - v1))) - v15655) - v15702) + (v15704 / v15684)) + v1));
                            let v15712 = ((v71 * v5830) * v5830) * v15626;
                            let v15715 = v15712 * v15626;
                            let v15754 = v15598 - ((v15678 / v15709) * (v1 + ((v15678 * ((((((v15712 * v15664) / v15684) - (v15715 / ((v15684 * v6653) * v15683))) - (v13238 * (((v15655 + v15660) - (((v71 * v14680) * v15658) * (v1 - (v15673 * (v2964 - ((v2976 * v15656) * v15658)))))) - (v15702 * (((v1 - (v15626 / v15683)) - v15653) + ((v15704 / v15683) * (v1 + (v1 / v6653)))))))) - ((v15715 * v15664) / (v15684 * v15683))) + v71)) / ((v71 * v15709) * v15709))));
                            v15755 = v15754;
                            v15764 = v15622;
                        }
                        let v15756 = v15755 - v14517;
                        let v15757 = -v14681;
                        let v15758 = rspice_limited_exp(v15757);
                        let v15759 = if v15756 < v8399 { 1.0 } else { 0.0 };
                        let v15911: f64;
                        let v15913: f64;
                        if v15759 != 0.0 {
                            let v15761 = (v15598 * v13140) - v15284;
                            let v15762 = v15761 / v13140;
                            let v15763 = rspice_limited_exp(v15762);
                            let v15774 = ((v15764 * v13140) - v15284) / v13140;
                            let v15775 = if v15762 > v6689 { 1.0 } else { 0.0 };
                            let v15789: f64;
                            if v15775 != 0.0 {
                                v15789 = v15762;
                            } else {
                                let v15777 = if v15762 < v15776 { 1.0 } else { 0.0 };
                                let v15790: f64;
                                if v15777 != 0.0 {
                                    let v15778 = v15762.exp();
                                    v15790 = v15778;
                                } else {
                                    let v15781 = (v1 + (v15762.exp())).ln();
                                    v15790 = v15781;
                                }
                                v15789 = v15790;
                            }
                            let v15782 = if v15774 > v6689 { 1.0 } else { 0.0 };
                            let v15791: f64;
                            if v15782 != 0.0 {
                                v15791 = v15774;
                            } else {
                                let v15784 = if v15774 < v15783 { 1.0 } else { 0.0 };
                                let v15792: f64;
                                if v15784 != 0.0 {
                                    let v15785 = v15774.exp();
                                    v15792 = v15785;
                                } else {
                                    let v15788 = (v1 + (v15774.exp())).ln();
                                    v15792 = v15788;
                                }
                                v15791 = v15792;
                            }
                            let v15797 = -((v13402 / v13140) + ((v15789 - v15791) / v6653));
                            let v15801 = rspice_limited_exp((-v15598));
                            let v15804 = v1 / ((v15598 * v15598) + v71);
                            let v15807 = (v71 * v15761) / v13140;
                            let v15808 = rspice_limited_exp(v15807);
                            let v15810 = rspice_limited_exp((v15807 + v15797));
                            let v15811 = v71 * v15763;
                            let v15812 = v13220 + v15797;
                            let v15817 = v6653 * (v15763 + v1);
                            let v15821 = v71 * v15598;
                            let v15840 = v15763 / v15817;
                            let v15842 = (rspice_limited_exp((v15797 + v15762))) / v15817;
                            let v15847 = -(((((((v15811 * v15812) * v5830) * v5830) / v15817) - (v71 * v13219)) + v15821) - (v13238 * ((((((rspice_limited_exp(((v15598 - v14681) - v14678))) + ((rspice_limited_exp((v15757 - v14678))) * ((((v15828 * v15598) * v15804) + ((((v15821 * v15598) * v15598) * v15804) * v15804)) - v1))) - v15801) - v15840) + v15842) + v1)));
                            let v15850 = (v13238 * (v1 - v15758)) * v14530;
                            let v15852 = (v71 * v5830) * v5830;
                            let v15856 = v15852 * v15808;
                            let v15859 = (v1 + v15811) + v15808;
                            let v15860 = (v6653 * v6653) * v15859;
                            let v15886 = v6653 * v15859;
                            let v15903 = (v15847 * v15847) - (v71 * ((((((((v15852 * v15763) * v15812) / v15817) - (v15856 / v15860)) - (v13238 * (((((((v15801 + (rspice_limited_exp(((v15598 - v14678) - v14681)))) + ((rspice_limited_exp((v14679 - v14681))) * (((v15868 * v15804) + ((((v2908 * v15598) * v15598) * v15804) * v15804)) - (((((((v3003 * v15598) * v15598) * v15598) * v15598) * v15804) * v15804) * v15804)))) - v15840) + (v15808 / v15886)) + v15842) - (v15810 / v15886)) - (v15810 / v15860)))) - ((v15856 * v15812) / v15886)) + v71) * v15850));
                            let v15904 = if v15903 >= v0 { 1.0 } else { 0.0 };
                            let v15909: f64;
                            if v15904 != 0.0 {
                                let v15908 = v71 * (v15850 / (v15847 + (v15903.sqrt())));
                                v15909 = v15908;
                            } else {
                                v15909 = v15756;
                            }
                            let v15910 = v14517 + v15909;
                            v15911 = v15909;
                            v15913 = v15910;
                        } else {
                            v15911 = v15756;
                            v15913 = v15755;
                        }
                        let v15912 = v15911 * v13140;
                        let v15914 = v15913 * v15913;
                        let v15926 = v2485 * (v14517 + v15913);
                        let v15929 = (((rspice_limited_exp((-v15913))) * v14524).abs()).sqrt();
                        let v15931 = v2485 * (v14530 + ((rspice_limited_exp((v15913 - v14678))) - (v14680 * ((v15913 + v1) + (v15914 / (v71 + v15914))))));
                        let v15937 = v15931 + (v8592 * ((v15911 * v15911) * (v15929 - (v71 * v13239))));
                        let v15938 = v13219 - v15926;
                        let v15941 = ((v15938 * v15938) * v13239) - v15937;
                        let v15944 = v13236 * ((v15937 + v15941).sqrt());
                        let v15945 = v15941 - v4710;
                        let v15952 = (v2485 * (v15945 + (((v15945 * v15945) + v15947).sqrt()))) + v4710;
                        let v15953 = if v8615 == v1 { 1.0 } else { 0.0 };
                        let v16019: f64;
                        if v15953 != 0.0 {
                            let v15959 = (((v71 * v13) * v13) * v13140) / ((v2 * v9) * v259);
                            let v15960 = v1 - v15929;
                            let v15967 = v1 / ((v1 + (v15959 * v15944)).sqrt());
                            let v15969 = v15967 / (v15967 + v1);
                            let v15976 = (v15959 * (((v15969 * v15969) * v15944) * v15944)) * (v15937 / (v15937 + v15952));
                            let v15981 = (v71 * (v15944 - v15976)) + (v13238 * (v15960 + v15937));
                            let v15984 = v15976 * (v15976 - (v71 * v15944));
                            let v15993 = (v15984 * v15981) / ((v15981 * v15981) - ((v1 - (v2485 * (v13238 * (v15929 + v15937)))) * v15984));
                            let v15995 = rspice_limited_exp(v15993);
                            let v15997 = v15937 * v15995;
                            let v15999 = (v13219 - (v15926 + v15993)) + v15993;
                            let v16018 = (((v15911 * v15995) * ((v15960 + (v71 * (v15944 * v13239))) + v15931)) / (((v1 - (v15929 / v15995)) + (v71 * (((v13236 * ((v15997 + (((v15999 * v15999) * v13239) - (v15997 / v15995))).sqrt())) * v15967) * v13239))) + (v15995 * v15931))) * v13140;
                            v16019 = v16018;
                        } else {
                            v16019 = v15912;
                        }
                        let v16021 = if (v16019.abs()) > v8693 { 1.0 } else { 0.0 };
                        if v16021 != 0.0 {
                        } else {
                        }
                        v19871 = v19872;
                        v19886 = v19887;
                    }
                    v19870 = v19871;
                    v19885 = v19886;
                } else {
                    v19870 = v19872;
                    v19885 = v19887;
                }
                v19844 = v9395;
                v19845 = v9556;
                v19847 = v9693;
                v19851 = v19852;
                v19857 = v19858;
                v19863 = v12764;
                v19869 = v19870;
                v19884 = v19885;
                v19927 = v9643;
                v19957 = v1;
                v19958 = v12882;
                v19959 = v12883;
                v19960 = v19961;
                v19962 = v19963;
                v19964 = v19965;
                v19967 = v19968;
                v19970 = v19971;
                v19973 = v19974;
                v19976 = v19977;
                v19979 = v19980;
                v19982 = v19983;
                v19984 = v19985;
                v19986 = v19987;
                v19988 = v19989;
                v19990 = v19991;
                v19992 = v19993;
                v19994 = v0;
                v19996 = v0;
                v19998 = v0;
                v20000 = v0;
                v20002 = v0;
                v20004 = v0;
                v20006 = v0;
                v20008 = v0;
                v20010 = v0;
                v20013 = v0;
                v20016 = v0;
                v20019 = v0;
                v20022 = v0;
                v20025 = v0;
                v20028 = v0;
                v20030 = v0;
                v20032 = v0;
                v20034 = v0;
                v20036 = v0;
                v20038 = v0;
                v20040 = v0;
                v20042 = v0;
                v20044 = v0;
            } else {
                let v16023 = v4691 / v5728;
                let v16029 = (v2485 * v5796) - (v2974 * (v1 + (v5803 / v16025)));
                let v16034 = v16029 + (((v16029 * v16029) + (v2979 * v5796)).sqrt());
                let v16035 = if v5796 < v0 { 1.0 } else { 0.0 };
                let v16056: f64;
                if v16035 != 0.0 {
                    let v16037 = (v5796 - v16034) / v5803;
                    let v16043 = -((if ((v1 - v16034) + (v16037 * v16037)) >= v4546 { ((v1 - v16034) + (v16037 * v16037)) } else { v4546 }).ln());
                    v16056 = v16043;
                } else {
                    let v16045 = rspice_limited_exp((-v16034));
                    let v16046 = v2485 * v5803;
                    let v16052 = ((((v5796 - v1) + v16045) + (v16046 * v16046)).sqrt()) - v16046;
                    let v16055 = ((v16052 * v16052) + v1) - v16045;
                    v16056 = v16055;
                }
                let v16057 = v16056 + v1;
                let v16058 = v16056 - v1;
                let v16059 = v16058 * v16058;
                let v16065 = (v2485 * (v16057 + ((v16059 + v16060).sqrt()))).sqrt();
                let v16066 = v71 * v16065;
                let v16069 = (v1 + (v5803 / v16066)) / v5803;
                let v16071 = v16056 - (v71 * v16023);
                let v16072 = v16071 - v5780;
                let v16077 = v16072 - ((if ((v2976 * v16069) * v16065) >= v4546 { ((v2976 * v16069) * v16065) } else { v4546 }).ln());
                let v16087 = v2485 * ((v16077 - v16078) - (((v16077 * (v16077 + v16080)) + v16083).sqrt()));
                let v16089 = if v16087 <= v16088 { 1.0 } else { 0.0 };
                let v16181: f64;
                if v16089 != 0.0 {
                    let v16092 = if v16087 < v16091 { 1.0 } else { 0.0 };
                    let v16112: f64;
                    if v16092 != 0.0 {
                        v16112 = v16093;
                    } else {
                        let v16095 = if v16087 > v16094 { 1.0 } else { 0.0 };
                        let v16113: f64;
                        if v16095 != 0.0 {
                            let v16096 = rspice_limited_exp(v16087);
                            v16113 = v16096;
                        } else {
                            let v16098 = (v16087 - v16090) / v5448;
                            let v16099 = v16098 * v16098;
                            let v16111 = rspice_limited_exp((v16090 + (v5448 * ((v16100 + (v2485 * v16098)) + (v16099 * (v16103 - (v16099 * (v6052 - v16099))))))));
                            v16113 = v16111;
                        }
                        v16112 = v16113;
                    }
                    let v16124 = v16112 * (((v1 + v16072) - v16087) - ((if ((v71 * v16069) * (((v16112 * v71) * v16069) + v16066)) >= v4546 { ((v71 * v16069) * (((v16112 * v71) * v16069) + v16066)) } else { v4546 }).ln()));
                    v16181 = v16124;
                } else {
                    let v16125 = rspice_limited_exp(v16087);
                    let v16127 = v71 * v16125;
                    let v16128 = v16127 * v16069;
                    let v16137 = v16069 + (v1 / v16065);
                    let v16143 = v16125 - (((v16127 + ((if (v16128 * (v16128 + v16066)) >= v4546 { (v16128 * (v16128 + v16066)) } else { v4546 }).ln())) - v16072) / ((v71 + (v1 / v16125)) + (v16137 / ((v16069 * v16125) + v16065))));
                    let v16144 = v71 * v16143;
                    let v16145 = v16144 * v16069;
                    let v16151 = (v16144 + ((if (v16145 * (v16145 + v16066)) >= v4546 { (v16145 * (v16145 + v16066)) } else { v4546 }).ln())) - v16072;
                    let v16152 = v1 / v16143;
                    let v16155 = (v16069 * v16143) + v16065;
                    let v16156 = v16137 / v16155;
                    let v16157 = (v71 + v16152) + v16156;
                    let v16174 = v16143 - ((v16151 / v16157) * (v1 + ((v16151 * (((-(v16152 * v16152)) - (v1 / (((v16065 * v16065) * v16065) * v16155))) - (v16156 * v16156))) / ((v71 * v16157) * v16157))));
                    v16181 = v16174;
                }
                let v16182 = v71 * v16181;
                let v16183 = v16056 - v16182;
                let v16185 = v16183 - v1;
                let v16195 = v1 + (v5803 / (((v2485 * (v16057 + ((v16059 + v16175).sqrt()))).sqrt()) + ((v2485 * ((v16183 + v1) + (((v16185 * v16185) + v16187).sqrt()))).sqrt())));
                let v16196 = v7176 / v5617;
                let v16197 = v5796 - v16056;
                let v16198 = v16195 - v1;
                let v16201 = v5729 * (v16197 - (v16182 * v16198));
                let v16207 = v2485 * (v16201 + (((v16201 * v16201) + v16203).sqrt()));
                let v16210 = ((v71 * v16195) * v5729) * v16181;
                let v16219 = v7189 + (v7190 * v5546);
                let v16224 = v1 + ((v16219 * ((v16196 * (v16207 + (v4728 * v16210))).powf(v4768))) + (v7195 / ((v2485 * (v1 + (v16210 / v16207))).powf(v7181))));
                let v16226 = v16224 - v1;
                let v16232 = v2485 * ((v16224 + v1) + (((v16226 * v16226) + v16228).sqrt()));
                let v16236 = v1 / (((v83 * v7207).powf(v769)) * v32);
                let v16297: f64;
                if v2834 != 0.0 {
                    v16297 = v0;
                } else {
                    let v16241 = (v1 / (v1 + (v749 * v16210))) + (v2812 * v5784);
                    let v16245 = v16241 + (((v16241 * v16241) + v4979).sqrt());
                    let v16298: f64;
                    if v4505 != 0.0 {
                        let v16250 = (((v7221 + (v7223 * v16245)) * v16236) * v32) * v4795;
                        v16298 = v16250;
                    } else {
                        let v16257 = ((v7138 + (((v7221 + (v7223 * v16245)) * v16236) * v32)) + v7135) * v4795;
                        v16298 = v16257;
                    }
                    v16297 = v16298;
                }
                let v16259 = v1 / v16258;
                let v16260 = v16232.powf(v16259);
                let v16261 = v1779 * v5546;
                let v16265 = v1 - v16261;
                let v16270 = v2485 * (v16265 + (((v16265 * v16265) + ((v5425 + (v16261 * v16261)).sqrt())).sqrt()));
                let v16272 = v2908 * v16271;
                let v16277 = ((v16272 * v16181) * v16270) / (v16272 + (v16181 * v16270));
                let v16279 = if v16278 < v0 { 1.0 } else { 0.0 };
                let v16305: f64;
                if v16279 != 0.0 {
                    let v16288 = (v71 * (((v7236 / v16260) * v5729) / (v7234 * v73))) * (v1 / (v1 - (v16278 * v16277)));
                    v16305 = v16288;
                } else {
                    let v16296 = (v71 * (((v7236 / v16260) * v5729) / (v7234 * v73))) * (v1 + (v16278 * v16277));
                    v16305 = v16296;
                }
                let v16299 = if v16297 > v0 { 1.0 } else { 0.0 };
                let v16502: f64;
                if v16299 != 0.0 {
                    let v16309 = (((((((v83 * v71) * v16195) * v13) * v5729) * v7234) * v16305) * v16297) / (v71 * v5729);
                    let v16310 = v2485 * v16305;
                    let v16312 = (v16181 * v16181) + v16181;
                    let v16317 = (v16310 * v16312) / (v1 + (v16310 * (v1 + v16181)));
                    let v16318 = v71 * v16305;
                    let v16320 = v16318 * (v16181 - v16317);
                    let v16321 = v16320 * v16320;
                    let v16323 = (v1 + v16321).sqrt();
                    let v16324 = if v16320 != v0 { 1.0 } else { 0.0 };
                    let v16331: f64;
                    let v16346: f64;
                    if v16324 != 0.0 {
                        let v16325 = v16320.asinh();
                        let v16328 = v16323 + ((v1 / v16320) * v16325);
                        v16331 = v16328;
                        v16346 = v16325;
                    } else {
                        let v16330 = v16323 + (v1 / v16323);
                        v16331 = v16330;
                        v16346 = v0;
                    }
                    let v16342 = ((v16317 * v16331) + ((v16309 * v16317) * ((v16181 + v16317) + v1))) - (v16305 * (v16312 - ((v16317 * v16317) + v16317)));
                    let v16354: f64;
                    if v16324 != 0.0 {
                        let v16349 = ((v16343 * v16305) * ((v16320 * v16323) - v16346)) / v16321;
                        v16354 = v16349;
                    } else {
                        let v16353 = (v16350 * v16305) * (v16320 / v16323);
                        v16354 = v16353;
                    }
                    let v16357 = v71 * v16317;
                    let v16366 = v16317 - (v16342 / ((((v16317 * v16354) + v16331) + (v16309 * ((v16181 + v16357) + v1))) + (v16305 * (v16357 + v1))));
                    let v16368 = v16318 * (v16181 - v16366);
                    let v16369 = v16368 * v16368;
                    let v16371 = (v1 + v16369).sqrt();
                    let v16372 = if v16368 != v0 { 1.0 } else { 0.0 };
                    let v16379: f64;
                    let v16394: f64;
                    if v16372 != 0.0 {
                        let v16373 = v16368.asinh();
                        let v16376 = v16371 + ((v1 / v16368) * v16373);
                        v16379 = v16376;
                        v16394 = v16373;
                    } else {
                        let v16378 = v16371 + (v1 / v16371);
                        v16379 = v16378;
                        v16394 = v16346;
                    }
                    let v16390 = ((v16366 * v16379) + ((v16309 * v16366) * ((v16181 + v16366) + v1))) - (v16305 * (v16312 - ((v16366 * v16366) + v16366)));
                    let v16402: f64;
                    if v16372 != 0.0 {
                        let v16397 = ((v16391 * v16305) * ((v16368 * v16371) - v16394)) / v16369;
                        v16402 = v16397;
                    } else {
                        let v16401 = (v16398 * v16305) * (v16368 / v16371);
                        v16402 = v16401;
                    }
                    let v16405 = v71 * v16366;
                    let v16414 = v16366 - (v16390 / ((((v16366 * v16402) + v16379) + (v16309 * ((v16181 + v16405) + v1))) + (v16305 * (v16405 + v1))));
                    v16502 = v16414;
                } else {
                    let v16415 = v2485 * v16305;
                    let v16417 = (v16181 * v16181) + v16181;
                    let v16422 = (v16415 * v16417) / (v1 + (v16415 * (v1 + v16181)));
                    let v16423 = v71 * v16305;
                    let v16425 = v16423 * (v16181 - v16422);
                    let v16426 = v16425 * v16425;
                    let v16428 = (v1 + v16426).sqrt();
                    let v16429 = if v16425 != v0 { 1.0 } else { 0.0 };
                    let v16436: f64;
                    let v16446: f64;
                    if v16429 != 0.0 {
                        let v16430 = v16425.asinh();
                        let v16433 = v16428 + ((v1 / v16425) * v16430);
                        v16436 = v16433;
                        v16446 = v16430;
                    } else {
                        let v16435 = v16428 + (v1 / v16428);
                        v16436 = v16435;
                        v16446 = v0;
                    }
                    let v16442 = (v16422 * v16436) - (v16305 * (v16417 - ((v16422 * v16422) + v16422)));
                    let v16454: f64;
                    if v16429 != 0.0 {
                        let v16449 = ((v16443 * v16305) * ((v16425 * v16428) - v16446)) / v16426;
                        v16454 = v16449;
                    } else {
                        let v16453 = (v16450 * v16305) * (v16425 / v16428);
                        v16454 = v16453;
                    }
                    let v16462 = v16422 - (v16442 / (((v16422 * v16454) + v16436) + (v16305 * ((v71 * v16422) + v1))));
                    let v16464 = v16423 * (v16181 - v16462);
                    let v16465 = v16464 * v16464;
                    let v16467 = (v1 + v16465).sqrt();
                    let v16468 = if v16464 != v0 { 1.0 } else { 0.0 };
                    let v16475: f64;
                    let v16485: f64;
                    if v16468 != 0.0 {
                        let v16469 = v16464.asinh();
                        let v16472 = v16467 + ((v1 / v16464) * v16469);
                        v16475 = v16472;
                        v16485 = v16469;
                    } else {
                        let v16474 = v16467 + (v1 / v16467);
                        v16475 = v16474;
                        v16485 = v16446;
                    }
                    let v16481 = (v16462 * v16475) - (v16305 * (v16417 - ((v16462 * v16462) + v16462)));
                    let v16493: f64;
                    if v16468 != 0.0 {
                        let v16488 = ((v16482 * v16305) * ((v16464 * v16467) - v16485)) / v16465;
                        v16493 = v16488;
                    } else {
                        let v16492 = (v16489 * v16305) * (v16464 / v16467);
                        v16493 = v16492;
                    }
                    let v16501 = v16462 - (v16481 / (((v16462 * v16493) + v16475) + (v16305 * ((v71 * v16462) + v1))));
                    v16502 = v16501;
                }
                let v16503 = v71 * v16502;
                let v16505 = (v16503 * v16195) * v5818;
                let v16513 = (v16071 - (v16503 + ((if (v16505 * (v16505 + (v5803 / v16198))) >= v4546 { (v16505 * (v16505 + (v5803 / v16198))) } else { v4546 }).ln()))) * v5729;
                let v16516 = if (if v7268 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v7270 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v16546: f64;
                if v16516 != 0.0 {
                    v16546 = v1;
                } else {
                    let v16520 = v73 / (v73 + ((v599 * v5666).sqrt()));
                    let v16530 = v1 + (((v7268 * v16520) - (((v7270 * v16520) * (v16181.powf(v7279))) * v5729)) / (v1 + (v7284 * v5546)));
                    let v16532 = v16530 - v5425;
                    let v16538 = v2485 * ((v16530 + v5425) + (((v16532 * v16532) + v16534).sqrt()));
                    v16546 = v16538;
                }
                let v16539 = v16513 - v5527;
                let v16547 = (v2485 * (v16539 + (((v16539 * v16539) + v16541).sqrt()))) / v16546;
                let v16550 = v1 / v4819;
                let v16553 = -v4819;
                let v16555 = v5528 * ((v1 + (((v5528 / v16547) + v127).powf(v16550))).powf(v16553));
                let v16563 = (v2485 * (v16057 + ((v16059 + v16558).sqrt()))).sqrt();
                let v16564 = v71 * v16563;
                let v16567 = (v1 + (v5803 / v16564)) / v5803;
                let v16568 = v16071 - ((v16555 + v5527) * v5730);
                let v16573 = v16568 - ((if ((v2976 * v16567) * v16563) >= v4546 { ((v2976 * v16567) * v16563) } else { v4546 }).ln());
                let v16580 = v2485 * ((v16573 - v16078) - (((v16573 * (v16573 + v16080)) + v16083).sqrt()));
                let v16582 = if v16580 <= v16581 { 1.0 } else { 0.0 };
                let v16669: f64;
                if v16582 != 0.0 {
                    let v16585 = if v16580 < v16584 { 1.0 } else { 0.0 };
                    let v16605: f64;
                    if v16585 != 0.0 {
                        v16605 = v16586;
                    } else {
                        let v16588 = if v16580 > v16587 { 1.0 } else { 0.0 };
                        let v16606: f64;
                        if v16588 != 0.0 {
                            let v16589 = rspice_limited_exp(v16580);
                            v16606 = v16589;
                        } else {
                            let v16591 = (v16580 - v16583) / v5448;
                            let v16592 = v16591 * v16591;
                            let v16604 = rspice_limited_exp((v16583 + (v5448 * ((v16593 + (v2485 * v16591)) + (v16592 * (v16596 - (v16592 * (v6052 - v16592))))))));
                            v16606 = v16604;
                        }
                        v16605 = v16606;
                    }
                    let v16617 = v16605 * (((v1 + v16568) - v16580) - ((if ((v71 * v16567) * (((v16605 * v71) * v16567) + v16564)) >= v4546 { ((v71 * v16567) * (((v16605 * v71) * v16567) + v16564)) } else { v4546 }).ln()));
                    v16669 = v16617;
                } else {
                    let v16618 = rspice_limited_exp(v16580);
                    let v16620 = v71 * v16618;
                    let v16621 = v16620 * v16567;
                    let v16630 = v16567 + (v1 / v16563);
                    let v16636 = v16618 - (((v16620 + ((if (v16621 * (v16621 + v16564)) >= v4546 { (v16621 * (v16621 + v16564)) } else { v4546 }).ln())) - v16568) / ((v71 + (v1 / v16618)) + (v16630 / ((v16567 * v16618) + v16563))));
                    let v16637 = v71 * v16636;
                    let v16638 = v16637 * v16567;
                    let v16644 = (v16637 + ((if (v16638 * (v16638 + v16564)) >= v4546 { (v16638 * (v16638 + v16564)) } else { v4546 }).ln())) - v16568;
                    let v16645 = v1 / v16636;
                    let v16648 = (v16567 * v16636) + v16563;
                    let v16649 = v16630 / v16648;
                    let v16650 = (v71 + v16645) + v16649;
                    let v16667 = v16636 - ((v16644 / v16650) * (v1 + ((v16644 * (((-(v16645 * v16645)) - (v1 / (((v16563 * v16563) * v16563) * v16648))) - (v16649 * v16649))) / ((v71 * v16650) * v16650))));
                    v16669 = v16667;
                }
                let v16671 = ((v16056 - v16181) - v16669) - v1;
                let v16673 = v16671 - v1;
                let v16683 = v1 + (v5803 / (v16563 + ((v2485 * ((v16671 + v1) + (((v16673 * v16673) + v16675).sqrt()))).sqrt())));
                let v16684 = v16181 - v16669;
                let v16685 = v16684 * v16684;
                let v16687 = (v1 + v16181) + v16669;
                let v16688 = v1 / v16687;
                let v16689 = v16685 * v16688;
                let v16691 = v16181 + v16669;
                let v16696 = v4724 * v16683;
                let v16697 = v16689 * v16688;
                let v16708 = v16696 * ((v16182 + v16669) + ((v2485 * ((v1 + (v16699 * v16181)) + (v16702 * v16669))) * v16697));
                let v16718 = v16696 * ((v16181 + (v71 * v16669)) + ((v2485 * ((v1 + (v16702 * v16181)) + (v16699 * v16669))) * v16697));
                let v16719 = v5729 * (v16197 - ((v16683 - v1) * (v16691 + (v4724 * v16689))));
                let v16725 = v2485 * (v16719 + (((v16719 * v16719) + v16721).sqrt()));
                let v16727 = v5729 * (v16708 + v16718);
                let v16739 = v1 + ((v16219 * ((v16196 * (v16725 + (v4728 * v16727))).powf(v4768))) + (v7195 / ((v2485 * (v1 + (v16727 / v16725))).powf(v7181))));
                let v16741 = v16739 - v1;
                let v16747 = v2485 * ((v16739 + v1) + (((v16741 * v16741) + v16743).sqrt()));
                let v16748 = v71 * v7234;
                let v16751 = (v16748 / (v7236 / v16747)) * v73;
                let v16752 = if v899 > v0 { 1.0 } else { 0.0 };
                let v16777: f64;
                if v16752 != 0.0 {
                    let v16755 = v1 + ((v899 * v16727) / v16751);
                    v16777 = v16755;
                } else {
                    let v16759 = v1 / (v1 - ((v899 * v16727) / v16751));
                    v16777 = v16759;
                }
                let v16760 = v5528 - v16555;
                let v16762 = v16727 + (v71 * v5729);
                let v16763 = if v8741 > v0 { 1.0 } else { 0.0 };
                let v16823: f64;
                if v16763 != 0.0 {
                    let v16767 = v1 + (v839 * v5546);
                    let v16781 = v1 + (v16760 / ((((v16762 / v8741) * (v16762 / (v16547 + v16762))) * v16777) * (v1 / (v2485 * (v16767 + (((v16767 * v16767) + v16769).sqrt()))))));
                    v16823 = v16781;
                } else {
                    v16823 = v1;
                }
                let v16782 = if v2495 <= v0 { 1.0 } else { 0.0 };
                let v16795: f64;
                if v16782 != 0.0 {
                    v16795 = v1;
                } else {
                    let v16787 = v1 / (v1 + ((v2495 * (v73.sqrt())) / v16762));
                    v16795 = v16787;
                }
                let v16788 = v16547 + v16751;
                let v16789 = if v8770 > v0 { 1.0 } else { 0.0 };
                let v16824: f64;
                if v16789 != 0.0 {
                    let v16790 = if v8772 < v0 { 1.0 } else { 0.0 };
                    let v16802: f64;
                    if v16790 != 0.0 {
                        let v16796 = (v8770 / (v1 - ((v8772 * v16727) / v16751))) / v16795;
                        v16802 = v16796;
                    } else {
                        let v16801 = (v8770 * (v1 + ((v8772 * v16727) / v16751))) / v16795;
                        v16802 = v16801;
                    }
                    let v16809 = v1 + (v16802 * ((if (v1 + ((v16760 / v16802) / v16788)) >= v4546 { (v1 + ((v16760 / v16802) / v16788)) } else { v4546 }).ln()));
                    v16824 = v16809;
                } else {
                    let v16810 = if v8772 < v0 { 1.0 } else { 0.0 };
                    let v16821: f64;
                    if v16810 != 0.0 {
                        let v16815 = (v8770 / (v1 - ((v8772 * v16727) / v16751))) / v16795;
                        v16821 = v16815;
                    } else {
                        let v16820 = (v8770 * (v1 + ((v8772 * v16727) / v16751))) / v16795;
                        v16821 = v16820;
                    }
                    let v16822 = v1 + v16821;
                    v16824 = v16822;
                }
                let v16825 = v16823 * v16824;
                let v16827 = rspice_limited_exp((v879 * v5528));
                let v16828 = if v869 > v0 { 1.0 } else { 0.0 };
                let v16835: f64;
                if v16828 != 0.0 {
                    let v16834 = ((v1 + ((v1 + (v8812 * v73)) * v16827)) / v869) * v16795;
                    v16835 = v16834;
                } else {
                    v16835 = v8819;
                }
                let v16838 = v16825 * (v1 + (v16760 / v16835));
                let v16839 = if v859 > v0 { 1.0 } else { 0.0 };
                let v16849: f64;
                if v16839 != 0.0 {
                    let v16840 = v849 * v4704;
                    let v16842 = if v16760 > (v16840 / v5531) { 1.0 } else { 0.0 };
                    let v16850: f64;
                    if v16842 != 0.0 {
                        let v16846 = (v73 * (rspice_limited_exp((v16840 / v16760)))) / v859;
                        v16850 = v16846;
                    } else {
                        let v16848 = (v8819 * v73) / v859;
                        v16850 = v16848;
                    }
                    v16849 = v16850;
                } else {
                    v16849 = v8819;
                }
                let v16853 = v16838 * (v1 + (v16760 / v16849));
                let v16854 = v16747.powf(v16259);
                let v16859 = ((v16272 * v16727) * v16270) / (v16272 + (v16727 * v16270));
                let v16877: f64;
                if v16279 != 0.0 {
                    let v16868 = (v71 * (((v7236 / v16854) * v5729) / (v7234 * v73))) * (v1 / (v1 - (v16278 * v16859)));
                    v16877 = v16868;
                } else {
                    let v16876 = (v71 * (((v7236 / v16854) * v5729) / (v7234 * v73))) * (v1 + (v16278 * v16859));
                    v16877 = v16876;
                }
                let v16879 = (v71 * v16877) * v16684;
                let v16882 = (v1 + (v16879 * v16879)).sqrt();
                let v16883 = if v16879 != v0 { 1.0 } else { 0.0 };
                let v16892: f64;
                if v16883 != 0.0 {
                    let v16888 = v2485 * (v16882 + ((v1 / v16879) * (v16879.asinh())));
                    v16892 = v16888;
                } else {
                    let v16891 = v2485 * (v16882 + (v1 / v16882));
                    v16892 = v16891;
                }
                let v17011: f64;
                let v18137: f64;
                let v19877: f64;
                let v19892: f64;
                if v2834 != 0.0 {
                    let v16893 = v5511 - v8893;
                    let v16903 = (v1 / (v1 + (v749 * (v2485 * (v16893 + (((v16893 * v16893) + v4979).sqrt())))))) + (v2812 * v5502);
                    let v16913 = v4795 * (v7138 + ((v8910 + (v8912 * (v2485 * (v16903 + (((v16903 * v16903) + v4979).sqrt()))))) * v16236));
                    let v16914 = v5510 - v8893;
                    let v16924 = (v1 / (v1 + (v749 * (v2485 * (v16914 + (((v16914 * v16914) + v4979).sqrt())))))) + (v2812 * v5499);
                    let v16934 = v4795 * (v7135 + ((v8935 + (v8937 * (v2485 * (v16924 + (((v16924 * v16924) + v4979).sqrt()))))) * v16236));
                    v17011 = v1;
                    v18137 = v0;
                    v19877 = v16934;
                    v19892 = v16913;
                } else {
                    let v16939 = (v1 / (v1 + (v749 * v16727))) + (v2812 * v5784);
                    let v16946 = v7221 + (v7223 * (v2485 * (v16939 + (((v16939 * v16939) + v4979).sqrt()))));
                    let v16949 = ((v4795 * v16946) * v16236) * v32;
                    let v16955 = ((((v7236 / (v16892 * v16747)) * v13) * v83) / v73) * v16727;
                    let v16957 = v1 + (v16955 * v16949);
                    let v16958 = if v2833 == v71 { 1.0 } else { 0.0 };
                    let v17012: f64;
                    let v18138: f64;
                    let v19878: f64;
                    let v19893: f64;
                    if v16958 != 0.0 {
                        let v16963 = v4795 * ((v7138 + ((v16946 * v16236) * v32)) + v7135);
                        let v16965 = v1 + (v16955 * v16963);
                        v17012 = v16965;
                        v18138 = v16963;
                        v19878 = v0;
                        v19893 = v0;
                    } else {
                        v17012 = v16957;
                        v18138 = v16949;
                        v19878 = v7135;
                        v19893 = v7138;
                    }
                    v17011 = v17012;
                    v18137 = v18138;
                    v19877 = v19878;
                    v19892 = v19893;
                }
                let v16967 = (v71 * v5728) * v4613;
                let v16974 = ((((v4846 + (v4856 / (v16727 + v16967))) * v16684) * v16684) + v1) - v4710;
                let v16985 = v2485 * (v1 + ((v1 + (v16975 + (v2485 * (v16974 + (((v16974 * v16974) + v8986).sqrt()))))).sqrt()));
                let v16987 = v16985 - v1;
                let v16997 = v16684 / (v16691 + v4887);
                let v17000 = v1 + ((v4876 * v16997) * v16997);
                let v17009 = rspice_limited_exp((-(v4897 / (((if v0 >= (v4907 + ((v4917 * v16684) * v16684)) { v0 } else { (v4907 + ((v4917 * v16684) * v16684)) }) * v16691) + v16967))));
                let v17014 = v7236 / ((v16747 * v16892) * v17011);
                let v17015 = v71 * v32;
                let v17030 = ((((((((((((v17015 * v16683) * v17014) * v83) / v73) * v13) * v5729) * v5729) * (v16684 * v16687)) * v16853) / ((v2485 * ((v16985 + v1) - (((v16987 * v16987) + v16989).sqrt()))) + v16994)) * v17000) * v17009) * v17029;
                let v17031 = if v4539 > v1 { 1.0 } else { 0.0 };
                let v19854: f64;
                let v19860: f64;
                if v17031 != 0.0 {
                    let v17043 = (v9081 * v32) * ((((((v9075 * v4613) * v17014) * v83) / v73) * v13) + ((((v17014 * v83) / v73) * v13) * v16727));
                    let v17044 = if v4539 == v71 { 1.0 } else { 0.0 };
                    let v19855: f64;
                    let v19861: f64;
                    if v17044 != 0.0 {
                        let v17046 = if (v1 / v9086) < v4508 { 1.0 } else { 0.0 };
                        let v17048: f64;
                        if v17046 != 0.0 {
                            let v17047 = v1 / v4508;
                            v17048 = v17047;
                        } else {
                            v17048 = v9086;
                        }
                        let v17051 = (v17048 * v17043) / (v17048 + v17043);
                        v19855 = v17051;
                        v19861 = v17048;
                    } else {
                        v19855 = v17043;
                        v19861 = v9086;
                    }
                    v19854 = v19855;
                    v19860 = v19861;
                } else {
                    v19854 = v0;
                    v19860 = v9086;
                }
                let v17052 = v4613 * v9094;
                let v17054 = rspice_limited_exp((v5507 / v17052));
                let v17056 = rspice_limited_exp((v5509 / v17052));
                let v17058 = (v9100 / v4613) * v4706;
                let v17059 = if v1069 == v0 { 1.0 } else { 0.0 };
                if v17059 != 0.0 {
                } else {
                }
                let v17060 = if v1079 == v0 { 1.0 } else { 0.0 };
                if v17060 != 0.0 {
                } else {
                }
                let v17061 = if v1109 == v0 { 1.0 } else { 0.0 };
                if v17061 != 0.0 {
                } else {
                    let v17063 = if (v1169 - v5507) < v4710 { 1.0 } else { 0.0 };
                    if v17063 != 0.0 {
                    } else {
                    }
                }
                let v17064 = if v1119 == v0 { 1.0 } else { 0.0 };
                if v17064 != 0.0 {
                } else {
                    let v17066 = if (v1179 - v5509) < v4710 { 1.0 } else { 0.0 };
                    if v17066 != 0.0 {
                    } else {
                    }
                }
                let v17067 = v4565 * v5613;
                let v17070 = if (if v1249 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1259 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v17531: f64;
                let v17685: f64;
                if v17070 != 0.0 {
                    v17531 = v0;
                    v17685 = v0;
                } else {
                    let v17073 = rspice_limited_exp(((v1229 * v17058) / v9094));
                    let v17075 = v1249 * v17073;
                    let v17076 = v1219 * v17073;
                    let v17077 = v1259 * v17073;
                    let v17078 = v17054 - v1;
                    let v17079 = (v1209 * v17073) * v17078;
                    let v17080 = if v17079 < v7159 { 1.0 } else { 0.0 };
                    let v17095: f64;
                    let v17115: f64;
                    if v17080 != 0.0 {
                        v17095 = v1;
                        v17115 = v0;
                    } else {
                        let v17083 = v1 / ((v1 + v17079).sqrt());
                        v17095 = v17083;
                        v17115 = v17079;
                    }
                    let v17084 = v17056 - v1;
                    let v17085 = v17076 * v17084;
                    let v17086 = if v17085 < v7159 { 1.0 } else { 0.0 };
                    let v17097: f64;
                    let v17116: f64;
                    if v17086 != 0.0 {
                        v17097 = v1;
                        v17116 = v0;
                    } else {
                        let v17089 = v1 / ((v1 + v17085).sqrt());
                        v17097 = v17089;
                        v17116 = v17085;
                    }
                    let v17100 = v1 + (v9143 * ((v1269 * ((v1 / v73) + (v1 / v9134))).powf(v1279)));
                    let v17103 = (((v17067 * v17075) * v17100) * v17078) * v17095;
                    let v17106 = (((v17067 * v17077) * v17100) * v17084) * v17097;
                    let v17108 = v1189 + (v1199 * v73);
                    let v17109 = if v17108 < v1 { 1.0 } else { 0.0 };
                    let v17112: f64;
                    if v17109 != 0.0 {
                        v17112 = v1;
                    } else {
                        v17112 = v17108;
                    }
                    let v17110 = if v9156 == v1 { 1.0 } else { 0.0 };
                    if v17110 != 0.0 {
                    } else {
                        let v17114 = v1 + ((v5507 + v5509) / v17112);
                        let v17124 = if ((v17114 + (((v17114 * v17114) + (v2976 * (v17115 + v17116))).sqrt())) / v71) < v5425 { 1.0 } else { 0.0 };
                        if v17124 != 0.0 {
                        } else {
                        }
                    }
                    v17531 = v17103;
                    v17685 = v17106;
                }
                let v17127 = if (if v1129 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1139 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v17127 != 0.0 {
                } else {
                    let v17129 = if (v1149 - v5507) < v4710 { 1.0 } else { 0.0 };
                    if v17129 != 0.0 {
                    } else {
                    }
                    let v17131 = if (v1159 - v5509) < v4710 { 1.0 } else { 0.0 };
                    if v17131 != 0.0 {
                    } else {
                    }
                }
                let v17132 = if v9179 == v0 { 1.0 } else { 0.0 };
                if v17132 != 0.0 {
                    let v17136 = if (if (if v2573 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4860 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v2867 != 0.0 { 1.0 } else { 0.0 };
                    if v17136 != 0.0 {
                    } else {
                        let v17137 = if v1349 != v0 { 1.0 } else { 0.0 };
                        if v17137 != 0.0 {
                        } else {
                        }
                    }
                    let v17141 = if (if (if v2580 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4862 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v2868 != 0.0 { 1.0 } else { 0.0 };
                    if v17141 != 0.0 {
                    } else {
                        let v17142 = if v1389 != v0 { 1.0 } else { 0.0 };
                        if v17142 != 0.0 {
                        } else {
                        }
                    }
                } else {
                    let v17145 = if (if v2573 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4860 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v17145 != 0.0 {
                    } else {
                        let v17146 = if v1409 != v0 { 1.0 } else { 0.0 };
                        if v17146 != 0.0 {
                        } else {
                        }
                    }
                    let v17149 = if (if v2580 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4862 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v17149 != 0.0 {
                    } else {
                        let v17150 = if v1399 != v0 { 1.0 } else { 0.0 };
                        if v17150 != 0.0 {
                        } else {
                        }
                    }
                }
                let v17151 = if v9200 == v0 { 1.0 } else { 0.0 };
                if v17151 != 0.0 {
                    let v17154 = if (if v2566 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4858 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v17154 != 0.0 {
                    } else {
                        let v17156 = if v16760 > (v4858 / v5531) { 1.0 } else { 0.0 };
                        if v17156 != 0.0 {
                        } else {
                        }
                    }
                } else {
                    let v17157 = if v9200 == v1 { 1.0 } else { 0.0 };
                    if v17157 != 0.0 {
                        let v17164 = if (if v2566 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if (if (if v1019 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1009 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4858 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if v17164 != 0.0 {
                        } else {
                        }
                    } else {
                        let v17171 = if (if v2566 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if (if (if v1019 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v1009 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4858 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        if v17171 != 0.0 {
                        } else {
                        }
                        let v17174 = v1029 * (v1 + (v9224 * v4706));
                        let v17175 = if v9216 > v0 { 1.0 } else { 0.0 };
                        let v17178: f64;
                        if v17175 != 0.0 {
                            let v17176 = v17174 - v5509;
                            v17178 = v17176;
                        } else {
                            let v17177 = v17174 - v5507;
                            v17178 = v17177;
                        }
                        let v17179 = if v17178 > v0 { 1.0 } else { 0.0 };
                        if v17179 != 0.0 {
                        } else {
                        }
                    }
                }
                let v17181 = v2587 + (v1609 * v4706);
                let v17183 = v2594 + (v1649 * v4706);
                let v17185 = v2601 + (v1689 * v4706);
                let v17187 = v1569 + (v1579 * v4706);
                let v17189 = v1539 + (v1549 * v4706);
                let v17190 = if v12543 != v0 { 1.0 } else { 0.0 };
                let v17191 = if v2886 != 0.0 || v17190 != 0.0 { 1.0 } else { 0.0 };
                let v17380: f64;
                let v17382: f64;
                let v17384: f64;
                let v17386: f64;
                let v17389: f64;
                if v17191 != 0.0 {
                    let v17194 = v5729 * ((v16197 + v16181) + v16669);
                    let v17197 = ((v17194 * v17194) + v9186).sqrt();
                    let v17200 = v2485 * ((-v17194) + v17197);
                    let v17202 = v2485 * (v17194 + v17197);
                    let v17385: f64;
                    if v17190 != 0.0 {
                        let v17204 = -(v17194 / v12562);
                        let v17205 = if v17204 > v6689 { 1.0 } else { 0.0 };
                        let v17212: f64;
                        if v17205 != 0.0 {
                            v17212 = v17204;
                        } else {
                            let v17207 = if v17204 < v17206 { 1.0 } else { 0.0 };
                            let v17213: f64;
                            if v17207 != 0.0 {
                                let v17208 = v17204.exp();
                                v17213 = v17208;
                            } else {
                                let v17211 = (v1 + (v17204.exp())).ln();
                                v17213 = v17211;
                            }
                            v17212 = v17213;
                        }
                        let v17214 = v12562 * v17212;
                        let v17215 = if v12575 != v0 { 1.0 } else { 0.0 };
                        let v17218: f64;
                        if v17215 != 0.0 {
                            let v17217 = v1 - (v17200 / v12575);
                            v17218 = v17217;
                        } else {
                            v17218 = v1;
                        }
                        let v17219 = if v17218 < v4979 { 1.0 } else { 0.0 };
                        let v17229: f64;
                        if v17219 != 0.0 {
                            v17229 = v4979;
                        } else {
                            v17229 = v17218;
                        }
                        let v17222 = ((v73 * v83) / v4564) + v4579;
                        let v17235 = (((((v17222 * v12584) * v4551) * v5493) * v17214) * (rspice_limited_exp((((v12587 * v12) * (v17189 - (v1559 * v17200))) / v17229)))) * v4866;
                        let v17237 = (v17194 - v1529) / v12562;
                        let v17238 = if v17237 > v6689 { 1.0 } else { 0.0 };
                        let v17245: f64;
                        if v17238 != 0.0 {
                            v17245 = v17237;
                        } else {
                            let v17240 = if v17237 < v17239 { 1.0 } else { 0.0 };
                            let v17246: f64;
                            if v17240 != 0.0 {
                                let v17241 = v17237.exp();
                                v17246 = v17241;
                            } else {
                                let v17244 = (v1 + (v17237.exp())).ln();
                                v17246 = v17244;
                            }
                            v17245 = v17246;
                        }
                        let v17247 = v12562 * v17245;
                        let v17248 = if v12611 != v0 { 1.0 } else { 0.0 };
                        let v17251: f64;
                        if v17248 != 0.0 {
                            let v17250 = v1 - (v17202 / v12611);
                            v17251 = v17250;
                        } else {
                            v17251 = v1;
                        }
                        let v17252 = if v17251 < v4979 { 1.0 } else { 0.0 };
                        let v17259: f64;
                        if v17252 != 0.0 {
                            v17259 = v4979;
                        } else {
                            v17259 = v17251;
                        }
                        let v17267 = v32 * (v17235 + ((((((v17222 * v12617) * v4551) * v5493) * v17247) * (rspice_limited_exp((((v12620 * v12) * (v17187 - (v1589 * v17202))) / v17259)))) * v4866));
                        v17385 = v17267;
                    } else {
                        v17385 = v0;
                    }
                    let v17277 = if (if (if (if v4681 != 0.0 && v17268 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v4592 != 0.0 && v17270 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4673 == v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v12516 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v17277 != 0.0 {
                        if v5 != 0.0 {
                        } else {
                        }
                        if v5 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                    let v17381: f64;
                    let v17383: f64;
                    let v17387: f64;
                    let v17390: f64;
                    if v2886 != 0.0 {
                        let v17296 = (((v32 * v4582) * (((v16683 * v5729) * v16691) * (rspice_limited_exp(((v4575 * (v17181 - (v1619 * v17202))) * (v1 + (v1629 * v17202))))))) * ((v5493 + (v2485 * v5542)) - (v2485 * (v5527 + v5526)))) * v4866;
                        let v17301 = v2606 * ((((v16555 * v16555) + v4979).sqrt()) - v5425);
                        let v17303 = rspice_limited_exp((-v17301));
                        let v17306 = ((v17301 + v17303) - v1) + v9186;
                        let v17310 = (v1 - ((v17301 + v1) * v17303)) + v9186;
                        let v17312 = (v17301 * v17301) + v12680;
                        let v17313 = if v9216 > v0 { 1.0 } else { 0.0 };
                        let v17388: f64;
                        let v17391: f64;
                        if v17313 != 0.0 {
                            let v17315 = (v17296 * v17310) / v17312;
                            let v17317 = (v17296 * v17306) / v17312;
                            v17388 = v17317;
                            v17391 = v17315;
                        } else {
                            let v17319 = (v17296 * v17310) / v17312;
                            let v17321 = (v17296 * v17306) / v17312;
                            v17388 = v17319;
                            v17391 = v17321;
                        }
                        let v17322 = v5511 - v8893;
                        let v17325 = ((v17322 * v17322) + v9186).sqrt();
                        let v17326 = if v12694 == v1 { 1.0 } else { 0.0 };
                        let v17338: f64;
                        let v17342: f64;
                        if v17326 != 0.0 {
                            let v17328 = v17183 - (v1659 * v17325);
                            let v17334 = v2485 * (v17328 + (((v17328 * v17328) + v17330).sqrt()));
                            let v17335 = if v1669 < v4979 { 1.0 } else { 0.0 };
                            let v17339: f64;
                            if v17335 != 0.0 {
                                v17339 = v4979;
                            } else {
                                v17339 = v1669;
                            }
                            v17338 = v17339;
                            v17342 = v17334;
                        } else {
                            let v17337 = v17183 - (v1659 * v17325);
                            v17338 = v1669;
                            v17342 = v17337;
                        }
                        let v17346 = v4866 * v32;
                        let v17351 = ((((v17346 * v4569) * v12717) * v5511) * v17325) * (rspice_limited_exp(((v4576 * v17342) * (v1 + (v17338 * v17325)))));
                        let v17352 = v5510 - v8893;
                        let v17355 = ((v17352 * v17352) + v9186).sqrt();
                        let v17367: f64;
                        let v17371: f64;
                        if v17326 != 0.0 {
                            let v17357 = v17185 - (v1699 * v17355);
                            let v17363 = v2485 * (v17357 + (((v17357 * v17357) + v17359).sqrt()));
                            let v17364 = if v1709 < v4979 { 1.0 } else { 0.0 };
                            let v17368: f64;
                            if v17364 != 0.0 {
                                v17368 = v4979;
                            } else {
                                v17368 = v1709;
                            }
                            v17367 = v17368;
                            v17371 = v17363;
                        } else {
                            let v17366 = v17185 - (v1699 * v17355);
                            v17367 = v1709;
                            v17371 = v17366;
                        }
                        let v17379 = ((((v17346 * v4573) * v12746) * v5510) * v17355) * (rspice_limited_exp(((v4576 * v17371) * (v1 + (v17367 * v17355)))));
                        v17381 = v17351;
                        v17383 = v17379;
                        v17387 = v17388;
                        v17390 = v17391;
                    } else {
                        v17381 = v0;
                        v17383 = v0;
                        v17387 = v0;
                        v17390 = v0;
                    }
                    v17380 = v17381;
                    v17382 = v17383;
                    v17384 = v17385;
                    v17386 = v17387;
                    v17389 = v17390;
                } else {
                    v17380 = v0;
                    v17382 = v0;
                    v17384 = v0;
                    v17386 = v0;
                    v17389 = v0;
                }
                let v17392 = v4929 * v9233;
                let v17393 = v4948 * v9235;
                let v17395 = (v4967 * v125) * v32;
                let v17396 = -v9241;
                let v17397 = v5425.powf(v17396);
                let v17398 = if v9241 == v1 { 1.0 } else { 0.0 };
                let v17462: f64;
                if v17398 != 0.0 {
                    v17462 = v17399;
                } else {
                    let v17407 = (v1 / (v1 - v9241)) * (v1 - (((v5443 * v9241) * (v1 + v9241)) * v17397));
                    v17462 = v17407;
                }
                let v17408 = -v9254;
                let v17409 = v5425.powf(v17408);
                let v17410 = if v9254 == v1 { 1.0 } else { 0.0 };
                let v17495: f64;
                if v17410 != 0.0 {
                    v17495 = v17411;
                } else {
                    let v17419 = (v1 / (v1 - v9254)) * (v1 - (((v5443 * v9254) * (v1 + v9254)) * v17409));
                    v17495 = v17419;
                }
                let v17420 = -v9267;
                let v17421 = v5425.powf(v17420);
                let v17422 = if v9267 == v1 { 1.0 } else { 0.0 };
                let v17528: f64;
                if v17422 != 0.0 {
                    v17528 = v17423;
                } else {
                    let v17431 = (v1 / (v1 - v9267)) * (v1 - (((v5443 * v9267) * (v1 + v9267)) * v17421));
                    v17528 = v17431;
                }
                let v17432 = if v17392 > v0 { 1.0 } else { 0.0 };
                let v17534: f64;
                if v17432 != 0.0 {
                    let v17433 = v5507 / v4987;
                    let v17434 = if v17433 < v9282 { 1.0 } else { 0.0 };
                    let v17535: f64;
                    if v17434 != 0.0 {
                        let v17435 = v1 - v17433;
                        let v17436 = if v9241 != v1 { 1.0 } else { 0.0 };
                        let v17536: f64;
                        if v17436 != 0.0 {
                            let v17437 = if v9241 == v2485 { 1.0 } else { 0.0 };
                            let v17444: f64;
                            if v17437 != 0.0 {
                                let v17439 = v1 / (v17435.sqrt());
                                v17444 = v17439;
                            } else {
                                let v17442 = rspice_limited_exp((v17396 * (v17435.ln())));
                                v17444 = v17442;
                            }
                            let v17449 = ((v4987 * v17392) * (v1 - (v17435 * v17444))) / (v1 - v9241);
                            v17536 = v17449;
                        } else {
                            let v17453 = (v4987 * v17392) * (-(v17435.ln()));
                            v17536 = v17453;
                        }
                        v17535 = v17536;
                    } else {
                        let v17454 = v17433 - v1;
                        let v17464 = (v4987 * v17392) * (((v17397 * v17454) * (((v2964 * v9241) * v17454) + (v1 + v9241))) + v17462);
                        v17535 = v17464;
                    }
                    v17534 = v17535;
                } else {
                    v17534 = v0;
                }
                let v17465 = if v17393 > v0 { 1.0 } else { 0.0 };
                let v17537: f64;
                if v17465 != 0.0 {
                    let v17466 = v5507 / v5009;
                    let v17467 = if v17466 < v9282 { 1.0 } else { 0.0 };
                    let v17538: f64;
                    if v17467 != 0.0 {
                        let v17468 = v1 - v17466;
                        let v17469 = if v9254 != v1 { 1.0 } else { 0.0 };
                        let v17539: f64;
                        if v17469 != 0.0 {
                            let v17470 = if v9254 == v2485 { 1.0 } else { 0.0 };
                            let v17477: f64;
                            if v17470 != 0.0 {
                                let v17472 = v1 / (v17468.sqrt());
                                v17477 = v17472;
                            } else {
                                let v17475 = rspice_limited_exp((v17408 * (v17468.ln())));
                                v17477 = v17475;
                            }
                            let v17482 = ((v5009 * v17393) * (v1 - (v17468 * v17477))) / (v1 - v9254);
                            v17539 = v17482;
                        } else {
                            let v17486 = (v5009 * v17393) * (-(v17468.ln()));
                            v17539 = v17486;
                        }
                        v17538 = v17539;
                    } else {
                        let v17487 = v17466 - v1;
                        let v17497 = (v5009 * v17393) * (((v17409 * v17487) * (((v2964 * v9254) * v17487) + (v1 + v9254))) + v17495);
                        v17538 = v17497;
                    }
                    v17537 = v17538;
                } else {
                    v17537 = v0;
                }
                let v17498 = if v17395 > v0 { 1.0 } else { 0.0 };
                let v17541: f64;
                if v17498 != 0.0 {
                    let v17499 = v5507 / v5031;
                    let v17500 = if v17499 < v9282 { 1.0 } else { 0.0 };
                    let v17542: f64;
                    if v17500 != 0.0 {
                        let v17501 = v1 - v17499;
                        let v17502 = if v9267 != v1 { 1.0 } else { 0.0 };
                        let v17543: f64;
                        if v17502 != 0.0 {
                            let v17503 = if v9267 == v2485 { 1.0 } else { 0.0 };
                            let v17510: f64;
                            if v17503 != 0.0 {
                                let v17505 = v1 / (v17501.sqrt());
                                v17510 = v17505;
                            } else {
                                let v17508 = rspice_limited_exp((v17420 * (v17501.ln())));
                                v17510 = v17508;
                            }
                            let v17515 = ((v5031 * v17395) * (v1 - (v17501 * v17510))) / (v1 - v9267);
                            v17543 = v17515;
                        } else {
                            let v17519 = (v5031 * v17395) * (-(v17501.ln()));
                            v17543 = v17519;
                        }
                        v17542 = v17543;
                    } else {
                        let v17520 = v17499 - v1;
                        let v17530 = (v5031 * v17395) * (((v17421 * v17520) * (((v2964 * v9267) * v17520) + (v1 + v9267))) + v17528);
                        v17542 = v17530;
                    }
                    v17541 = v17542;
                } else {
                    v17541 = v0;
                }
                let v17545 = ((v17534 + v17537) + v17541) + ((v9380 * v17531) * v32);
                let v17546 = v4936 * v9396;
                let v17547 = v4955 * v9398;
                let v17549 = (v4974 * v125) * v32;
                let v17550 = -v9404;
                let v17551 = v5425.powf(v17550);
                let v17552 = if v9404 == v1 { 1.0 } else { 0.0 };
                let v17616: f64;
                if v17552 != 0.0 {
                    v17616 = v17553;
                } else {
                    let v17561 = (v1 / (v1 - v9404)) * (v1 - (((v5443 * v9404) * (v1 + v9404)) * v17551));
                    v17616 = v17561;
                }
                let v17562 = -v9417;
                let v17563 = v5425.powf(v17562);
                let v17564 = if v9417 == v1 { 1.0 } else { 0.0 };
                let v17649: f64;
                if v17564 != 0.0 {
                    v17649 = v17565;
                } else {
                    let v17573 = (v1 / (v1 - v9417)) * (v1 - (((v5443 * v9417) * (v1 + v9417)) * v17563));
                    v17649 = v17573;
                }
                let v17574 = -v9430;
                let v17575 = v5425.powf(v17574);
                let v17576 = if v9430 == v1 { 1.0 } else { 0.0 };
                let v17682: f64;
                if v17576 != 0.0 {
                    v17682 = v17577;
                } else {
                    let v17585 = (v1 / (v1 - v9430)) * (v1 - (((v5443 * v9430) * (v1 + v9430)) * v17575));
                    v17682 = v17585;
                }
                let v17586 = if v17546 > v0 { 1.0 } else { 0.0 };
                let v17688: f64;
                if v17586 != 0.0 {
                    let v17587 = v5509 / v4997;
                    let v17588 = if v17587 < v9282 { 1.0 } else { 0.0 };
                    let v17689: f64;
                    if v17588 != 0.0 {
                        let v17589 = v1 - v17587;
                        let v17590 = if v9404 != v1 { 1.0 } else { 0.0 };
                        let v17690: f64;
                        if v17590 != 0.0 {
                            let v17591 = if v9404 == v2485 { 1.0 } else { 0.0 };
                            let v17598: f64;
                            if v17591 != 0.0 {
                                let v17593 = v1 / (v17589.sqrt());
                                v17598 = v17593;
                            } else {
                                let v17596 = rspice_limited_exp((v17550 * (v17589.ln())));
                                v17598 = v17596;
                            }
                            let v17603 = ((v4997 * v17546) * (v1 - (v17589 * v17598))) / (v1 - v9404);
                            v17690 = v17603;
                        } else {
                            let v17607 = (v4997 * v17546) * (-(v17589.ln()));
                            v17690 = v17607;
                        }
                        v17689 = v17690;
                    } else {
                        let v17608 = v17587 - v1;
                        let v17618 = (v4997 * v17546) * (((v17551 * v17608) * (((v2964 * v9404) * v17608) + (v1 + v9404))) + v17616);
                        v17689 = v17618;
                    }
                    v17688 = v17689;
                } else {
                    v17688 = v0;
                }
                let v17619 = if v17547 > v0 { 1.0 } else { 0.0 };
                let v17691: f64;
                if v17619 != 0.0 {
                    let v17620 = v5509 / v5019;
                    let v17621 = if v17620 < v9282 { 1.0 } else { 0.0 };
                    let v17692: f64;
                    if v17621 != 0.0 {
                        let v17622 = v1 - v17620;
                        let v17623 = if v9417 != v1 { 1.0 } else { 0.0 };
                        let v17693: f64;
                        if v17623 != 0.0 {
                            let v17624 = if v9417 == v2485 { 1.0 } else { 0.0 };
                            let v17631: f64;
                            if v17624 != 0.0 {
                                let v17626 = v1 / (v17622.sqrt());
                                v17631 = v17626;
                            } else {
                                let v17629 = rspice_limited_exp((v17562 * (v17622.ln())));
                                v17631 = v17629;
                            }
                            let v17636 = ((v5019 * v17547) * (v1 - (v17622 * v17631))) / (v1 - v9417);
                            v17693 = v17636;
                        } else {
                            let v17640 = (v5019 * v17547) * (-(v17622.ln()));
                            v17693 = v17640;
                        }
                        v17692 = v17693;
                    } else {
                        let v17641 = v17620 - v1;
                        let v17651 = (v5019 * v17547) * (((v17563 * v17641) * (((v2964 * v9417) * v17641) + (v1 + v9417))) + v17649);
                        v17692 = v17651;
                    }
                    v17691 = v17692;
                } else {
                    v17691 = v0;
                }
                let v17652 = if v17549 > v0 { 1.0 } else { 0.0 };
                let v17695: f64;
                if v17652 != 0.0 {
                    let v17653 = v5509 / v5041;
                    let v17654 = if v17653 < v9282 { 1.0 } else { 0.0 };
                    let v17696: f64;
                    if v17654 != 0.0 {
                        let v17655 = v1 - v17653;
                        let v17656 = if v9430 != v1 { 1.0 } else { 0.0 };
                        let v17697: f64;
                        if v17656 != 0.0 {
                            let v17657 = if v9430 == v2485 { 1.0 } else { 0.0 };
                            let v17664: f64;
                            if v17657 != 0.0 {
                                let v17659 = v1 / (v17655.sqrt());
                                v17664 = v17659;
                            } else {
                                let v17662 = rspice_limited_exp((v17574 * (v17655.ln())));
                                v17664 = v17662;
                            }
                            let v17669 = ((v5041 * v17549) * (v1 - (v17655 * v17664))) / (v1 - v9430);
                            v17697 = v17669;
                        } else {
                            let v17673 = (v5041 * v17549) * (-(v17655.ln()));
                            v17697 = v17673;
                        }
                        v17696 = v17697;
                    } else {
                        let v17674 = v17653 - v1;
                        let v17684 = (v5041 * v17549) * (((v17575 * v17674) * (((v2964 * v9430) * v17674) + (v1 + v9430))) + v17682);
                        v17696 = v17684;
                    }
                    v17695 = v17696;
                } else {
                    v17695 = v0;
                }
                let v17699 = ((v17688 + v17691) + v17695) + ((v9380 * v17685) * v32);
                let v17701 = if v17700 != v0 { 1.0 } else { 0.0 };
                if v17701 != 0.0 {
                } else {
                }
                let v17703 = (v2976 * v4613) * v2;
                let v17704 = v16748 / v17014;
                let v17705 = if v12767 <= v0 { 1.0 } else { 0.0 };
                let v18000: f64;
                if v17705 != 0.0 {
                    v18000 = v0;
                } else {
                    let v17711 = v4704 * ((if (((v16760 / v4704) + v12767) / v17704) >= v4546 { (((v16760 / v4704) + v12767) / v17704) } else { v4546 }).ln());
                    let v17712 = if v17711 < v0 { 1.0 } else { 0.0 };
                    let v18001: f64;
                    if v17712 != 0.0 {
                        v18001 = v0;
                    } else {
                        v18001 = v17711;
                    }
                    v18000 = v18001;
                }
                let v17713 = v4613 / v2;
                let v17716 = v17713 * ((v13 + v5667) + v269);
                let v17717 = v71 * v16683;
                let v17719 = (v17717 * v13) * v4613;
                let v17723 = (((v17719 * v16669) * v17009) * v17000) / v2;
                let v17728 = ((v17724 * v4613) * (v17030.abs())) * v17014;
                let v17729 = v2 * v4613;
                let v17731 = (v17729 * v17030) * v17030;
                let v17736 = (v12801 + (v12802 * v17723)) + ((v12805 * v17723) * v17723);
                let v17737 = v17723 + v17716;
                let v17738 = v17737 * v17737;
                let v17740 = (v12801 * v2) * v4613;
                let v17742 = if v17741 == v1 { 1.0 } else { 0.0 };
                let v19769: f64;
                let v19995: f64;
                let v19997: f64;
                let v19999: f64;
                let v20001: f64;
                let v20003: f64;
                let v20005: f64;
                if v17742 != 0.0 {
                    let v17744 = if v73 > v17743 { 1.0 } else { 0.0 };
                    let v17746: f64;
                    let v17918: f64;
                    if v17744 != 0.0 {
                        let v17745 = v73 - v17743;
                        v17746 = v17745;
                        v17918 = v17743;
                    } else {
                        v17746 = v73;
                        v17918 = v73;
                    }
                    let v17748 = if v12813 >= (v17746 / v71) { 1.0 } else { 0.0 };
                    let v17973: f64;
                    if v17748 != 0.0 {
                        v17973 = v0;
                    } else {
                        v17973 = v12813;
                    }
                    let v17750 = (v5493 - v5778) / v4613;
                    let v17757 = ((((v17751 * v9) * v17753) / v4613).sqrt()) / v13;
                    let v17759 = (v17753 / v4637).ln();
                    let v17764 = (v2485 * v17750) - (v2974 * (v1 + (v17757 / v16025)));
                    let v17769 = v17764 + (((v17764 * v17764) + (v2979 * v17750)).sqrt());
                    let v17770 = if v17750 < v0 { 1.0 } else { 0.0 };
                    let v17791: f64;
                    if v17770 != 0.0 {
                        let v17772 = (v17750 - v17769) / v17757;
                        let v17778 = -((if ((v1 - v17769) + (v17772 * v17772)) >= v4546 { ((v1 - v17769) + (v17772 * v17772)) } else { v4546 }).ln());
                        v17791 = v17778;
                    } else {
                        let v17780 = rspice_limited_exp((-v17769));
                        let v17781 = v2485 * v17757;
                        let v17787 = ((((v17750 - v1) + v17780) + (v17781 * v17781)).sqrt()) - v17781;
                        let v17790 = ((v17787 * v17787) + v1) - v17780;
                        v17791 = v17790;
                    }
                    let v17792 = v17791 + v1;
                    let v17793 = v17791 - v1;
                    let v17794 = v17793 * v17793;
                    let v17800 = (v2485 * (v17792 + ((v17794 + v17795).sqrt()))).sqrt();
                    let v17801 = v71 * v17800;
                    let v17804 = (v1 + (v17757 / v17801)) / v17757;
                    let v17807 = (v17791 - (v71 * v17759)) - v5780;
                    let v17812 = v17807 - ((if ((v2976 * v17804) * v17800) >= v4546 { ((v2976 * v17804) * v17800) } else { v4546 }).ln());
                    let v17819 = v2485 * ((v17812 - v16078) - (((v17812 * (v17812 + v16080)) + v16083).sqrt()));
                    let v17821 = if v17819 <= v17820 { 1.0 } else { 0.0 };
                    let v17931: f64;
                    if v17821 != 0.0 {
                        let v17824 = if v17819 < v17823 { 1.0 } else { 0.0 };
                        let v17844: f64;
                        if v17824 != 0.0 {
                            v17844 = v17825;
                        } else {
                            let v17827 = if v17819 > v17826 { 1.0 } else { 0.0 };
                            let v17845: f64;
                            if v17827 != 0.0 {
                                let v17828 = rspice_limited_exp(v17819);
                                v17845 = v17828;
                            } else {
                                let v17830 = (v17819 - v17822) / v5448;
                                let v17831 = v17830 * v17830;
                                let v17843 = rspice_limited_exp((v17822 + (v5448 * ((v17832 + (v2485 * v17830)) + (v17831 * (v17835 - (v17831 * (v6052 - v17831))))))));
                                v17845 = v17843;
                            }
                            v17844 = v17845;
                        }
                        let v17856 = v17844 * (((v1 + v17807) - v17819) - ((if ((v71 * v17804) * (((v17844 * v71) * v17804) + v17801)) >= v4546 { ((v71 * v17804) * (((v17844 * v71) * v17804) + v17801)) } else { v4546 }).ln()));
                        v17931 = v17856;
                    } else {
                        let v17857 = rspice_limited_exp(v17819);
                        let v17859 = v71 * v17857;
                        let v17860 = v17859 * v17804;
                        let v17869 = v17804 + (v1 / v17800);
                        let v17875 = v17857 - (((v17859 + ((if (v17860 * (v17860 + v17801)) >= v4546 { (v17860 * (v17860 + v17801)) } else { v4546 }).ln())) - v17807) / ((v71 + (v1 / v17857)) + (v17869 / ((v17804 * v17857) + v17800))));
                        let v17876 = v71 * v17875;
                        let v17877 = v17876 * v17804;
                        let v17883 = (v17876 + ((if (v17877 * (v17877 + v17801)) >= v4546 { (v17877 * (v17877 + v17801)) } else { v4546 }).ln())) - v17807;
                        let v17884 = v1 / v17875;
                        let v17887 = (v17804 * v17875) + v17800;
                        let v17888 = v17869 / v17887;
                        let v17889 = (v71 + v17884) + v17888;
                        let v17906 = v17875 - ((v17883 / v17889) * (v1 + ((v17883 * (((-(v17884 * v17884)) - (v1 / (((v17800 * v17800) * v17800) * v17887))) - (v17888 * v17888))) / ((v71 * v17889) * v17889))));
                        v17931 = v17906;
                    }
                    let v17917 = (v17014 * v13) * v83;
                    let v17922 = ((v71 * (v1 + (v17757 / (v71 * ((v2485 * (v17792 + ((v17794 + v17907).sqrt()))).sqrt()))))) * v17917) * v4613;
                    let v17925 = v73 - v17918;
                    let v17927 = v17717 * v17917;
                    let v17930 = (v17030 * v17925) / ((v17927 * v5729) * v5729);
                    let v17936 = v1 + (v2976 * (((v17931 * v17931) + v17931) - ((v17030 * v17918) / (v17922 * v4613))));
                    let v17937 = if v17936 < v1 { 1.0 } else { 0.0 };
                    let v17951: f64;
                    if v17937 != 0.0 {
                        v17951 = v0;
                    } else {
                        let v17941 = v17938 + (v2485 * (v17936.sqrt()));
                        v17951 = v17941;
                    }
                    let v17950 = v17942 + (v2485 * ((v1 + (v2976 * (((v16669 * v16669) + v16669) + v17930))).sqrt()));
                    let v17959 = (v17922 * v17951) * v17925;
                    let v17962 = ((((v71 * v17917) * v4613) * (v17950 - v16669)) * v17918) + (((v17927 * v4613) * v16669) * v17918);
                    let v17963 = v17959 + v17962;
                    let v17965 = (v1 / v17963) / v17963;
                    let v17967 = (v17959 * v17959) * v17965;
                    let v17969 = (v17962 * v17962) * v17965;
                    let v17970 = if v73 != v17918 { 1.0 } else { 0.0 };
                    let v18030: f64;
                    if v17970 != 0.0 {
                        let v17972 = (v17719 * v17950) / v2;
                        let v17976 = (v73 - (v71 * v17973)) - v17918;
                        let v17977 = v17976 * v17976;
                        let v18005 = ((v17728 / ((v12825 * v13) * v17977)) * (((v12801 * ((if ((v17972 + v17716) / v17737) >= v4546 { ((v17972 + v17716) / v17737) } else { v4546 }).ln())) + (v12802 * (v17972 - v17723))) + ((v2485 * v12805) * ((v17972 * v17972) - (v17723 * v17723))))) + ((((v17731 / (((v12825 * v17977) * v83) * v32)) * v18000) * v17736) / v17738);
                        let v18013 = ((v17740 / (((((v83 * v32) * v17976) * v12825) * v17716) * v17716)) * v17030) * v17030;
                        let v18014 = v18013 + v18005;
                        let v18015 = if v18014 > v0 { 1.0 } else { 0.0 };
                        let v18031: f64;
                        if v18015 != 0.0 {
                            let v18017 = (v18005 * v18013) / v18014;
                            v18031 = v18017;
                        } else {
                            v18031 = v0;
                        }
                        v18030 = v18031;
                    } else {
                        v18030 = v0;
                    }
                    let v18028 = ((((v18018 * v2) * v4613) / (((((v83 * v32) * v17918) * v12825) * v17716) * v17716)) * v17030) * v17030;
                    let v18029 = if v18028 > v0 { 1.0 } else { 0.0 };
                    let v18033: f64;
                    if v18029 != 0.0 {
                        v18033 = v18028;
                    } else {
                        v18033 = v0;
                    }
                    let v18036 = v9216 * ((v18030 * v17967) + (v18033 * v17969));
                    v19769 = v17973;
                    v19995 = v1;
                    v19997 = v18036;
                    v19999 = v12883;
                    v20001 = v0;
                    v20003 = v0;
                    v20005 = v0;
                } else {
                    let v18038 = if v12813 >= (v73 / v71) { 1.0 } else { 0.0 };
                    let v18044: f64;
                    if v18038 != 0.0 {
                        v18044 = v0;
                    } else {
                        v18044 = v12813;
                    }
                    let v18043 = if (if (if v12801 > v0 { 1.0 } else { 0.0 }) != 0.0 || (if v12802 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v12805 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v18094: f64;
                    if v18043 != 0.0 {
                        let v18046 = v73 - (v71 * v18044);
                        let v18047 = v18046 * v18046;
                        let v18053 = (((v17719 * v16181) * v17009) * v17000) / v2;
                        let v18077 = ((v17728 / ((v12825 * v13) * v18047)) * (((v12801 * ((if ((v18053 + v17716) / v17737) >= v4546 { ((v18053 + v17716) / v17737) } else { v4546 }).ln())) + (v12802 * (v18053 - v17723))) + ((v2485 * v12805) * ((v18053 * v18053) - (v17723 * v17723))))) + ((((v17731 / (((v12825 * v18047) * v83) * v32)) * v18000) * v17736) / v17738);
                        let v18085 = ((v17740 / (((((v83 * v32) * v18046) * v12825) * v17716) * v17716)) * v17030) * v17030;
                        let v18086 = v18085 + v18077;
                        let v18087 = if v18086 > v0 { 1.0 } else { 0.0 };
                        let v18095: f64;
                        if v18087 != 0.0 {
                            let v18093 = ((v18077 * v18085) / v18086) / (v1 + (v12873 * (v16684.powf(v12875))));
                            v18095 = v18093;
                        } else {
                            v18095 = v0;
                        }
                        v18094 = v18095;
                    } else {
                        v18094 = v0;
                    }
                    let v18096 = v9216 * v18094;
                    v19769 = v18044;
                    v19995 = v0;
                    v19997 = v0;
                    v19999 = v0;
                    v20001 = v1;
                    v20003 = v18096;
                    v20005 = v12883;
                }
                let v18098 = (v16727 / v17704) / v73;
                let v18099 = v18098 * v18098;
                let v18103 = v12887 * (v1 + ((v12888 * v73) * v18099));
                let v18107 = v12893 * (v1 + ((v12894 * v73) * v18099));
                let v18111 = v12899 * (v1 + ((v12900 * v73) * v18099));
                let v18115 = v12905 * (v1 + ((v12906 * v73) * v18099));
                let v18121 = rspice_limited_exp(((-v73) / v12915));
                let v18123 = ((((v2974 * v18103) * v18103) - v1) * v18121) + v1;
                let v18124 = v18111 * v18111;
                let v18125 = v18107 * v18107;
                let v18126 = if v12924 == v0 { 1.0 } else { 0.0 };
                let v20007: f64;
                let v20009: f64;
                let v20011: f64;
                let v20014: f64;
                let v20017: f64;
                let v20020: f64;
                let v20023: f64;
                let v20026: f64;
                if v18126 != 0.0 {
                    let v18131 = ((((-v32) * v83) * v73) * v13) * v4613;
                    let v18136 = v17014 * (((v18131 * v16708) + (v18131 * v16718)).abs());
                    let v18144 = v17703 * ((v18136 / ((v18136 * v18137) + (v73 * v73))) * v12943);
                    v20007 = v1;
                    v20009 = v18144;
                    v20011 = v0;
                    v20014 = v0;
                    v20017 = v0;
                    v20020 = v0;
                    v20023 = v0;
                    v20026 = v0;
                } else {
                    let v18145 = if v12924 == v1 { 1.0 } else { 0.0 };
                    let v20012: f64;
                    let v20015: f64;
                    let v20018: f64;
                    let v20021: f64;
                    let v20024: f64;
                    let v20027: f64;
                    if v18145 != 0.0 {
                        let v18150 = (((v17014 * v16892) * v16853) * v13) * (v17717 * v5729);
                        let v18151 = v2485 * v16691;
                        let v18152 = v18151 + v2485;
                        let v18153 = v18152 * v18152;
                        let v18154 = v18153 * v18152;
                        let v18159 = v73 * v16892;
                        let v18160 = v18159 / v73;
                        let v18168 = (((v1 + ((v18124 * (v16555 / v16547)) / (v12972 + v16727))) - v1) * v18121) + v1;
                        let v18180 = v6103 * v18152;
                        let v18203 = ((((((v18159 * v18160) * v18160) * (((v18151 / v18153) - ((((v2979 * v18151) + v2485) * v16685) / ((v12997 * v18153) * v18153))) + ((v16685 * v16685) / ((v13003 * v18153) * v18154)))) * v13009) / v2976) * v18125) / (((v32 * v83) * v6103) * v18150);
                        let v18210 = ((v18160 * ((v16684 / v18180) - ((v16685 * v16684) / (v13003 * v18154)))) * v18115) / v13023;
                        let v18212 = (v17703 * ((((v18150 * v32) * v83) / v18159) * ((v18151 * (v2485 * (v18168 + (((v18168 * v18168) + v18170).sqrt())))) + ((v16685 * v18123) / v18180)))).sqrt();
                        let v18213 = if v18203 > v0 { 1.0 } else { 0.0 };
                        let v18219: f64;
                        let v18221: f64;
                        if v18213 != 0.0 {
                            let v18215 = (v17703 / v18203).sqrt();
                            let v18216 = if v18212 > v0 { 1.0 } else { 0.0 };
                            let v18220: f64;
                            if v18216 != 0.0 {
                                let v18218 = (v18210 * v18215) / v18212;
                                v18220 = v18218;
                            } else {
                                v18220 = v0;
                            }
                            v18219 = v18220;
                            v18221 = v18215;
                        } else {
                            v18219 = v0;
                            v18221 = v0;
                        }
                        let v18223 = v1 - v18219;
                        let v18224 = (v18221 * v18221) * v18223;
                        let v18226 = (v18212 * v18212) * v18223;
                        v20012 = v1;
                        v20015 = v18219;
                        v20018 = v1;
                        v20021 = v18224;
                        v20024 = v1;
                        v20027 = v18226;
                    } else {
                        v20012 = v0;
                        v20015 = v0;
                        v20018 = v0;
                        v20021 = v0;
                        v20024 = v0;
                        v20027 = v0;
                    }
                    v20007 = v0;
                    v20009 = v0;
                    v20011 = v20012;
                    v20014 = v20015;
                    v20017 = v20018;
                    v20020 = v20021;
                    v20023 = v20024;
                    v20026 = v20027;
                }
                let v20029: f64;
                let v20031: f64;
                let v20033: f64;
                let v20035: f64;
                if v2886 != 0.0 {
                    let v18230 = v18227 * ((v17386 + v17380).abs());
                    let v18234 = v18231 * ((v17389 + v17382).abs());
                    v20029 = v1;
                    v20031 = v18230;
                    v20033 = v1;
                    v20035 = v18234;
                } else {
                    v20029 = v0;
                    v20031 = v0;
                    v20033 = v0;
                    v20035 = v0;
                }
                let v20037: f64;
                let v20039: f64;
                if v17190 != 0.0 {
                    let v18237 = v18235 * (v17384.abs());
                    v20037 = v1;
                    v20039 = v18237;
                } else {
                    v20037 = v0;
                    v20039 = v0;
                }
                let v18239 = if v18238 == v1 { 1.0 } else { 0.0 };
                let v18756: f64;
                let v18758: f64;
                let v18768: f64;
                let v18769: f64;
                let v18772: f64;
                let v18773: f64;
                let v18792: f64;
                let v18810: f64;
                let v18811: f64;
                let v18828: f64;
                if v18239 != 0.0 {
                    let v18243 = (v5493 * v4614) - ((v2683 + v5777) * v4614);
                    let v18246 = (if (v2878 / v4637) >= v4546 { (v2878 / v4637) } else { v4546 }).ln();
                    let v18252 = ((((v18247 * v9) * v2878) * v4614).sqrt()) / v13;
                    let v18253 = v1 / v18252;
                    let v18259 = ((v18254 * v9) * v259) / ((v13 * v13) * v4613);
                    let v18261: f64;
                    if v4660 != 0.0 {
                        let v18260 = v1 / v18259;
                        v18261 = v18260;
                    } else {
                        v18261 = v0;
                    }
                    let v18263: f64;
                    if v4660 != 0.0 {
                        let v18262 = v2878 / v259;
                        v18263 = v18262;
                    } else {
                        v18263 = v0;
                    }
                    let v18264 = v1 + v18263;
                    let v18265 = v18243 / v18264;
                    let v18266 = v18252 / v18264;
                    let v18270 = v2974 * (v1 + (v18266 / v16025));
                    let v18271 = (v2485 * v18265) - v18270;
                    let v18276 = v18271 + (((v18271 * v18271) + (v2979 * v18265)).sqrt());
                    let v18277 = if v18265 < v0 { 1.0 } else { 0.0 };
                    let v18298: f64;
                    if v18277 != 0.0 {
                        let v18279 = (v18265 - v18276) / v18266;
                        let v18285 = -((if ((v1 - v18276) + (v18279 * v18279)) >= v4546 { ((v1 - v18276) + (v18279 * v18279)) } else { v4546 }).ln());
                        v18298 = v18285;
                    } else {
                        let v18287 = rspice_limited_exp((-v18276));
                        let v18288 = v2485 * v18266;
                        let v18294 = ((((v18265 - v1) + v18287) + (v18288 * v18288)).sqrt()) - v18288;
                        let v18297 = ((v18294 * v18294) + v1) - v18287;
                        v18298 = v18297;
                    }
                    let v18299 = v18298 + v1;
                    let v18300 = v18298 - v1;
                    let v18301 = v18300 * v18300;
                    let v18307 = (v2485 * (v18299 + ((v18301 + v18302).sqrt()))).sqrt();
                    let v18308 = v71 * v18307;
                    let v18311 = (v1 + (v18252 / v18308)) / v18252;
                    let v18313 = v18298 - (v71 * v18246);
                    let v18314 = v18313 - v5805;
                    let v18319 = v18314 - ((if ((v2976 * v18311) * v18307) >= v4546 { ((v2976 * v18311) * v18307) } else { v4546 }).ln());
                    let v18326 = v2485 * ((v18319 - v16078) - (((v18319 * (v18319 + v16080)) + v16083).sqrt()));
                    let v18328 = if v18326 <= v18327 { 1.0 } else { 0.0 };
                    let v18420: f64;
                    if v18328 != 0.0 {
                        let v18331 = if v18326 < v18330 { 1.0 } else { 0.0 };
                        let v18351: f64;
                        if v18331 != 0.0 {
                            v18351 = v18332;
                        } else {
                            let v18334 = if v18326 > v18333 { 1.0 } else { 0.0 };
                            let v18352: f64;
                            if v18334 != 0.0 {
                                let v18335 = rspice_limited_exp(v18326);
                                v18352 = v18335;
                            } else {
                                let v18337 = (v18326 - v18329) / v5448;
                                let v18338 = v18337 * v18337;
                                let v18350 = rspice_limited_exp((v18329 + (v5448 * ((v18339 + (v2485 * v18337)) + (v18338 * (v18342 - (v18338 * (v6052 - v18338))))))));
                                v18352 = v18350;
                            }
                            v18351 = v18352;
                        }
                        let v18363 = v18351 * (((v1 + v18314) - v18326) - ((if ((v71 * v18311) * (((v18351 * v71) * v18311) + v18308)) >= v4546 { ((v71 * v18311) * (((v18351 * v71) * v18311) + v18308)) } else { v4546 }).ln()));
                        v18420 = v18363;
                    } else {
                        let v18364 = rspice_limited_exp(v18326);
                        let v18366 = v71 * v18364;
                        let v18367 = v18366 * v18311;
                        let v18376 = v18311 + (v1 / v18307);
                        let v18382 = v18364 - (((v18366 + ((if (v18367 * (v18367 + v18308)) >= v4546 { (v18367 * (v18367 + v18308)) } else { v4546 }).ln())) - v18314) / ((v71 + (v1 / v18364)) + (v18376 / ((v18311 * v18364) + v18307))));
                        let v18383 = v71 * v18382;
                        let v18384 = v18383 * v18311;
                        let v18390 = (v18383 + ((if (v18384 * (v18384 + v18308)) >= v4546 { (v18384 * (v18384 + v18308)) } else { v4546 }).ln())) - v18314;
                        let v18391 = v1 / v18382;
                        let v18394 = (v18311 * v18382) + v18307;
                        let v18395 = v18376 / v18394;
                        let v18396 = (v71 + v18391) + v18395;
                        let v18413 = v18382 - ((v18390 / v18396) * (v1 + ((v18390 * (((-(v18391 * v18391)) - (v1 / (((v18307 * v18307) * v18307) * v18394))) - (v18395 * v18395))) / ((v71 * v18396) * v18396))));
                        v18420 = v18413;
                    }
                    let v18421 = v71 * v18420;
                    let v18422 = v18298 - v18421;
                    let v18424 = v18422 - v1;
                    let v18434 = v1 + (v18252 / (((v2485 * (v18299 + ((v18301 + v18414).sqrt()))).sqrt()) + ((v2485 * ((v18422 + v1) + (((v18424 * v18424) + v18426).sqrt()))).sqrt())));
                    let v18435 = v18243 - v18298;
                    let v18436 = v18434 - v1;
                    let v18439 = v4613 * (v18435 - (v18421 * v18436));
                    let v18454 = v1 + (v16219 * ((v16196 * ((v2485 * (v18439 + (((v18439 * v18439) + v18441).sqrt()))) + (v4728 * (((v71 * v18434) * v4613) * v18420)))).powf(v4768)));
                    let v18456 = v18454 - v1;
                    let v18466 = v18465 * v107;
                    let v18467 = ((v7236 / (v2485 * ((v18454 + v1) + (((v18456 * v18456) + v18458).sqrt())))) * v4613) / v18466;
                    let v18475 = v71 * ((v18467 * ((v18420 * v18420) + v18420)) / (v1 + (v18467 * (v1 + v18420))));
                    let v18477 = (v18475 * v18434) * v18253;
                    let v18486 = ((v18313 - (v18475 + ((if (v18477 * (v18477 + (v18252 / v18436))) >= v4546 { (v18477 * (v18477 + (v18252 / v18436))) } else { v4546 }).ln()))) * v4613) - v5527;
                    let v18492 = v2485 * (v18486 + (((v18486 * v18486) + v18488).sqrt()));
                    let v18497 = if (if v18493 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v18495 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v18521: f64;
                    if v18497 != 0.0 {
                        v18521 = v18498;
                    } else {
                        let v18502 = v73 / (v73 + ((v599 * v5666).sqrt()));
                        let v18512 = v1 + (((v18493 * v18502) - (((v18495 * v18502) * v18420) * v5729)) / (v1 + (v18508 * v5546)));
                        let v18514 = v18512 - v5425;
                        let v18520 = v2485 * ((v18512 + v5425) + (((v18514 * v18514) + v18516).sqrt()));
                        v18521 = v18520;
                    }
                    let v18522 = v18492 / v18521;
                    let v18528 = v5528 * ((v1 + (((v5528 / v18522) + v127).powf(v16550))).powf(v16553));
                    let v18536 = (v2485 * (v18299 + ((v18301 + v18531).sqrt()))).sqrt();
                    let v18537 = v71 * v18536;
                    let v18540 = (v1 + (v18252 / v18537)) / v18252;
                    let v18541 = v18313 - ((v18528 + v5527) * v4614);
                    let v18546 = v18541 - ((if ((v2976 * v18540) * v18536) >= v4546 { ((v2976 * v18540) * v18536) } else { v4546 }).ln());
                    let v18553 = v2485 * ((v18546 - v16078) - (((v18546 * (v18546 + v16080)) + v16083).sqrt()));
                    let v18555 = if v18553 <= v18554 { 1.0 } else { 0.0 };
                    let v18642: f64;
                    if v18555 != 0.0 {
                        let v18558 = if v18553 < v18557 { 1.0 } else { 0.0 };
                        let v18578: f64;
                        if v18558 != 0.0 {
                            v18578 = v18559;
                        } else {
                            let v18561 = if v18553 > v18560 { 1.0 } else { 0.0 };
                            let v18579: f64;
                            if v18561 != 0.0 {
                                let v18562 = rspice_limited_exp(v18553);
                                v18579 = v18562;
                            } else {
                                let v18564 = (v18553 - v18556) / v5448;
                                let v18565 = v18564 * v18564;
                                let v18577 = rspice_limited_exp((v18556 + (v5448 * ((v18566 + (v2485 * v18564)) + (v18565 * (v18569 - (v18565 * (v6052 - v18565))))))));
                                v18579 = v18577;
                            }
                            v18578 = v18579;
                        }
                        let v18590 = v18578 * (((v1 + v18541) - v18553) - ((if ((v71 * v18540) * (((v18578 * v71) * v18540) + v18537)) >= v4546 { ((v71 * v18540) * (((v18578 * v71) * v18540) + v18537)) } else { v4546 }).ln()));
                        v18642 = v18590;
                    } else {
                        let v18591 = rspice_limited_exp(v18553);
                        let v18593 = v71 * v18591;
                        let v18594 = v18593 * v18540;
                        let v18603 = v18540 + (v1 / v18536);
                        let v18609 = v18591 - (((v18593 + ((if (v18594 * (v18594 + v18537)) >= v4546 { (v18594 * (v18594 + v18537)) } else { v4546 }).ln())) - v18541) / ((v71 + (v1 / v18591)) + (v18603 / ((v18540 * v18591) + v18536))));
                        let v18610 = v71 * v18609;
                        let v18611 = v18610 * v18540;
                        let v18617 = (v18610 + ((if (v18611 * (v18611 + v18537)) >= v4546 { (v18611 * (v18611 + v18537)) } else { v4546 }).ln())) - v18541;
                        let v18618 = v1 / v18609;
                        let v18621 = (v18540 * v18609) + v18536;
                        let v18622 = v18603 / v18621;
                        let v18623 = (v71 + v18618) + v18622;
                        let v18640 = v18609 - ((v18617 / v18623) * (v1 + ((v18617 * (((-(v18618 * v18618)) - (v1 / (((v18536 * v18536) * v18536) * v18621))) - (v18622 * v18622))) / ((v71 * v18623) * v18623))));
                        v18642 = v18640;
                    }
                    let v18644 = ((v18298 - v18420) - v18642) - v1;
                    let v18646 = v18644 - v1;
                    let v18653 = (v2485 * ((v18644 + v1) + (((v18646 * v18646) + v18648).sqrt()))).sqrt();
                    let v18656 = v18264 + (v18252 / (v18536 + v18653));
                    let v18659 = v2485 + ((v18263 * v18653) * v18253);
                    let v18661 = v18420 + v18642;
                    let v18667 = v18656 / (v18659 + (((v18659 * v18659) + ((v18656 * v18661) * v18261)).sqrt()));
                    let v18668 = v18667 - v1;
                    let v18671 = v4613 * (v18435 - (v18421 * v18668));
                    let v18681 = v4613 * (v18435 - ((v71 * v18642) * v18668));
                    let v18694 = v16196 * ((v2485 * ((v2485 * (v18671 + (((v18671 * v18671) + v18673).sqrt()))) + (v2485 * (v18681 + (((v18681 * v18681) + v18683).sqrt()))))) + (v4728 * ((v18667 * v4613) * v18661)));
                    let v18698 = (v18243 + (v18695 * v4614)) / v18264;
                    let v18700 = (v2485 * v18698) - v18270;
                    let v18705 = v18700 + (((v18700 * v18700) + (v2979 * v18698)).sqrt());
                    let v18706 = if v18698 < v0 { 1.0 } else { 0.0 };
                    let v18770: f64;
                    if v18706 != 0.0 {
                        let v18708 = (v18698 - v18705) / v18266;
                        let v18714 = -((if ((v1 - v18705) + (v18708 * v18708)) >= v4546 { ((v1 - v18705) + (v18708 * v18708)) } else { v4546 }).ln());
                        v18770 = v18714;
                    } else {
                        let v18716 = rspice_limited_exp((-v18705));
                        let v18717 = v2485 * v18266;
                        let v18723 = ((((v18698 - v1) + v18716) + (v18717 * v18717)).sqrt()) - v18717;
                        let v18726 = ((v18723 * v18723) + v1) - v18716;
                        v18770 = v18726;
                    }
                    let v18729 = v1 + (v16219 * (v18694.powf(v4768)));
                    let v18731 = v18729 - v1;
                    let v18738 = v7236 / (v2485 * ((v18729 + v1) + (((v18731 * v18731) + v18733).sqrt())));
                    let v18743 = (((v71 * v18738) * v4613) / v18466) * (v18420 - v18642);
                    let v18749 = v2485 * (v1 + ((v1 + ((v71 * v18743) * v18743)).sqrt()));
                    let v18753 = v18522 + (((v71 * v18465) / v18738) * v107);
                    let v18754 = v5528 - v18528;
                    v18756 = v18754;
                    v18758 = v18753;
                    v18768 = v18243;
                    v18769 = v18770;
                    v18772 = v18420;
                    v18773 = v18642;
                    v18792 = v18261;
                    v18810 = v18521;
                    v18811 = v18749;
                    v18828 = v18667;
                } else {
                    v18756 = v16760;
                    v18758 = v16788;
                    v18768 = v5796;
                    v18769 = v16056;
                    v18772 = v16181;
                    v18773 = v16669;
                    v18792 = v0;
                    v18810 = v1;
                    v18811 = v16892;
                    v18828 = v16683;
                }
                let v18755 = if v2715 != v0 { 1.0 } else { 0.0 };
                let v18765: f64;
                if v18755 != 0.0 {
                    let v18764 = v1 + (v2715 * ((if (v1 + ((v18756 / v2715) / v18758)) >= v4546 { (v1 + ((v18756 / v2715) / v18758)) } else { v4546 }).ln()));
                    v18765 = v18764;
                } else {
                    v18765 = v1;
                }
                let v18766 = v1 / v18765;
                let v18771 = v18768 - v18769;
                let v18774 = v18772 - v18773;
                let v18775 = v18774 * v18774;
                let v18777 = v18771 + (v71 * v18772);
                let v18779 = v18771 + (v71 * v18773);
                let v18795 = (v2542 + ((v2485 * (v18777 + (((v18777 * v18777) + v18781).sqrt()))) * v18792)).sqrt();
                let v18798 = (v2542 + ((v2485 * (v18779 + (((v18779 * v18779) + v18787).sqrt()))) * v18792)).sqrt();
                let v18802 = v71 * v18798;
                let v18803 = v1 + v18802;
                let v18805 = v18795 + v18798;
                let v18806 = v18805 * v18805;
                let v18816 = ((v18810 * v18811) * v18766) / ((v1 + v18772) + v18773);
                let v18848 = v11 / v9630;
                let v18851 = (((-(((v32 * v111) * v107) + v9638)) * v18848) * v4613) * ((v18766 * (((v18777 / (v1 + (v71 * v18795))) + (v18779 / v18803)) + (((v4724 * (v18775 / (v18806 * v18805))) * (((v16699 * (v18806 + (v18795 * v18798))) * v18816) + (v71 * v18792))) - (v18828 * ((v18772 + v18773) + ((v4724 * v18775) * v18816)))))) + ((v18765 - v1) * ((v18771 - ((v71 * (v18828 - v1)) * v18773)) + ((v18779 * (v18802 - v1)) / v18803))));
                let v18852 = if v4673 == v1 { 1.0 } else { 0.0 };
                let v19304: f64;
                if v18852 != 0.0 {
                    let v18855 = v5547 * v4614;
                    let v18857 = (v5496 * v4614) - ((v9702 + v5777) * v4614);
                    let v18860 = (if (v2880 / v4637) >= v4546 { (v2880 / v4637) } else { v4546 }).ln();
                    let v18866 = ((((v18861 * v9) * v2880) * v4614).sqrt()) / v13;
                    let v18867 = v1 / v18866;
                    let v18872 = (v2485 * v18857) - (v2974 * (v1 + (v18866 / v16025)));
                    let v18877 = v18872 + (((v18872 * v18872) + (v2979 * v18857)).sqrt());
                    let v18878 = if v18857 < v0 { 1.0 } else { 0.0 };
                    let v18899: f64;
                    if v18878 != 0.0 {
                        let v18880 = (v18857 - v18877) / v18866;
                        let v18886 = -((if ((v1 - v18877) + (v18880 * v18880)) >= v4546 { ((v1 - v18877) + (v18880 * v18880)) } else { v4546 }).ln());
                        v18899 = v18886;
                    } else {
                        let v18888 = rspice_limited_exp((-v18877));
                        let v18889 = v2485 * v18866;
                        let v18895 = ((((v18857 - v1) + v18888) + (v18889 * v18889)).sqrt()) - v18889;
                        let v18898 = ((v18895 * v18895) + v1) - v18888;
                        v18899 = v18898;
                    }
                    let v18900 = v18899 + v1;
                    let v18901 = v18899 - v1;
                    let v18902 = v18901 * v18901;
                    let v18908 = (v2485 * (v18900 + ((v18902 + v18903).sqrt()))).sqrt();
                    let v18909 = v71 * v18908;
                    let v18912 = (v1 + (v18866 / v18909)) / v18866;
                    let v18914 = v18899 - (v71 * v18860);
                    let v18915 = v18914 - v18855;
                    let v18920 = v18915 - ((if ((v2976 * v18912) * v18908) >= v4546 { ((v2976 * v18912) * v18908) } else { v4546 }).ln());
                    let v18927 = v2485 * ((v18920 - v16078) - (((v18920 * (v18920 + v16080)) + v16083).sqrt()));
                    let v18929 = if v18927 <= v18928 { 1.0 } else { 0.0 };
                    let v19021: f64;
                    if v18929 != 0.0 {
                        let v18932 = if v18927 < v18931 { 1.0 } else { 0.0 };
                        let v18952: f64;
                        if v18932 != 0.0 {
                            v18952 = v18933;
                        } else {
                            let v18935 = if v18927 > v18934 { 1.0 } else { 0.0 };
                            let v18953: f64;
                            if v18935 != 0.0 {
                                let v18936 = rspice_limited_exp(v18927);
                                v18953 = v18936;
                            } else {
                                let v18938 = (v18927 - v18930) / v5448;
                                let v18939 = v18938 * v18938;
                                let v18951 = rspice_limited_exp((v18930 + (v5448 * ((v18940 + (v2485 * v18938)) + (v18939 * (v18943 - (v18939 * (v6052 - v18939))))))));
                                v18953 = v18951;
                            }
                            v18952 = v18953;
                        }
                        let v18964 = v18952 * (((v1 + v18915) - v18927) - ((if ((v71 * v18912) * (((v18952 * v71) * v18912) + v18909)) >= v4546 { ((v71 * v18912) * (((v18952 * v71) * v18912) + v18909)) } else { v4546 }).ln()));
                        v19021 = v18964;
                    } else {
                        let v18965 = rspice_limited_exp(v18927);
                        let v18967 = v71 * v18965;
                        let v18968 = v18967 * v18912;
                        let v18977 = v18912 + (v1 / v18908);
                        let v18983 = v18965 - (((v18967 + ((if (v18968 * (v18968 + v18909)) >= v4546 { (v18968 * (v18968 + v18909)) } else { v4546 }).ln())) - v18915) / ((v71 + (v1 / v18965)) + (v18977 / ((v18912 * v18965) + v18908))));
                        let v18984 = v71 * v18983;
                        let v18985 = v18984 * v18912;
                        let v18991 = (v18984 + ((if (v18985 * (v18985 + v18909)) >= v4546 { (v18985 * (v18985 + v18909)) } else { v4546 }).ln())) - v18915;
                        let v18992 = v1 / v18983;
                        let v18995 = (v18912 * v18983) + v18908;
                        let v18996 = v18977 / v18995;
                        let v18997 = (v71 + v18992) + v18996;
                        let v19014 = v18983 - ((v18991 / v18997) * (v1 + ((v18991 * (((-(v18992 * v18992)) - (v1 / (((v18908 * v18908) * v18908) * v18995))) - (v18996 * v18996))) / ((v71 * v18997) * v18997))));
                        v19021 = v19014;
                    }
                    let v19022 = v71 * v19021;
                    let v19023 = v18899 - v19022;
                    let v19025 = v19023 - v1;
                    let v19035 = v1 + (v18866 / (((v2485 * (v18900 + ((v18902 + v19015).sqrt()))).sqrt()) + ((v2485 * ((v19023 + v1) + (((v19025 * v19025) + v19027).sqrt()))).sqrt())));
                    let v19036 = v18857 - v18899;
                    let v19037 = v19035 - v1;
                    let v19040 = v4613 * (v19036 - (v19022 * v19037));
                    let v19057 = v1 + ((v7189 + (v7190 * v5549)) * ((v16196 * ((v2485 * (v19040 + (((v19040 * v19040) + v19042).sqrt()))) + (v4728 * (((v71 * v19035) * v4613) * v19021)))).powf(v4768)));
                    let v19059 = v19057 - v1;
                    let v19069 = ((v7236 / (v2485 * ((v19057 + v1) + (((v19059 * v19059) + v19061).sqrt())))) * v4613) / (v18465 * v107);
                    let v19073 = v1 + v19021;
                    let v19077 = v71 * ((v19069 * ((v19021 * v19021) + v19021)) / (v1 + (v19069 * v19073)));
                    let v19079 = (v19077 * v19035) * v18867;
                    let v19088 = ((v18914 - (v19077 + ((if (v19079 * (v19079 + (v18866 / v19037))) >= v4546 { (v19079 * (v19079 + (v18866 / v19037))) } else { v4546 }).ln()))) * v4613) - v5547;
                    let v19108 = (v2485 * (v18900 + ((v18902 + v19103).sqrt()))).sqrt();
                    let v19109 = v71 * v19108;
                    let v19112 = (v1 + (v18866 / v19109)) / v18866;
                    let v19113 = v18914 - (((v5528 * ((v1 + (((v5528 / (v2485 * (v19088 + (((v19088 * v19088) + v19090).sqrt())))) + v127).powf(v16550))).powf(v16553))) + v5547) * v4614);
                    let v19118 = v19113 - ((if ((v2976 * v19112) * v19108) >= v4546 { ((v2976 * v19112) * v19108) } else { v4546 }).ln());
                    let v19125 = v2485 * ((v19118 - v16078) - (((v19118 * (v19118 + v16080)) + v16083).sqrt()));
                    let v19127 = if v19125 <= v19126 { 1.0 } else { 0.0 };
                    let v19214: f64;
                    if v19127 != 0.0 {
                        let v19130 = if v19125 < v19129 { 1.0 } else { 0.0 };
                        let v19150: f64;
                        if v19130 != 0.0 {
                            v19150 = v19131;
                        } else {
                            let v19133 = if v19125 > v19132 { 1.0 } else { 0.0 };
                            let v19151: f64;
                            if v19133 != 0.0 {
                                let v19134 = rspice_limited_exp(v19125);
                                v19151 = v19134;
                            } else {
                                let v19136 = (v19125 - v19128) / v5448;
                                let v19137 = v19136 * v19136;
                                let v19149 = rspice_limited_exp((v19128 + (v5448 * ((v19138 + (v2485 * v19136)) + (v19137 * (v19141 - (v19137 * (v6052 - v19137))))))));
                                v19151 = v19149;
                            }
                            v19150 = v19151;
                        }
                        let v19162 = v19150 * (((v1 + v19113) - v19125) - ((if ((v71 * v19112) * (((v19150 * v71) * v19112) + v19109)) >= v4546 { ((v71 * v19112) * (((v19150 * v71) * v19112) + v19109)) } else { v4546 }).ln()));
                        v19214 = v19162;
                    } else {
                        let v19163 = rspice_limited_exp(v19125);
                        let v19165 = v71 * v19163;
                        let v19166 = v19165 * v19112;
                        let v19175 = v19112 + (v1 / v19108);
                        let v19181 = v19163 - (((v19165 + ((if (v19166 * (v19166 + v19109)) >= v4546 { (v19166 * (v19166 + v19109)) } else { v4546 }).ln())) - v19113) / ((v71 + (v1 / v19163)) + (v19175 / ((v19112 * v19163) + v19108))));
                        let v19182 = v71 * v19181;
                        let v19183 = v19182 * v19112;
                        let v19189 = (v19182 + ((if (v19183 * (v19183 + v19109)) >= v4546 { (v19183 * (v19183 + v19109)) } else { v4546 }).ln())) - v19113;
                        let v19190 = v1 / v19181;
                        let v19193 = (v19112 * v19181) + v19108;
                        let v19194 = v19175 / v19193;
                        let v19195 = (v71 + v19190) + v19194;
                        let v19212 = v19181 - ((v19189 / v19195) * (v1 + ((v19189 * (((-(v19190 * v19190)) - (v1 / (((v19108 * v19108) * v19108) * v19193))) - (v19194 * v19194))) / ((v71 * v19195) * v19195))));
                        v19214 = v19212;
                    }
                    let v19216 = ((v18899 - v19021) - v19214) - v1;
                    let v19218 = v19216 - v1;
                    let v19225 = (v2485 * ((v19216 + v1) + (((v19218 * v19218) + v19220).sqrt()))).sqrt();
                    let v19229 = v19226 + (v18866 / (v19108 + v19225));
                    let v19232 = v2485 + ((v0 * v19225) * v18867);
                    let v19234 = v19021 + v19214;
                    let v19240 = v19229 / (v19232 + (((v19232 * v19232) + ((v19229 * v19234) * v0)).sqrt()));
                    let v19242 = v19021 - v19214;
                    let v19243 = v19242 * v19242;
                    let v19244 = v19036 + v19022;
                    let v19246 = v19036 + (v71 * v19214);
                    let v19261 = (v2542 + ((v2485 * (v19244 + (((v19244 * v19244) + v19248).sqrt()))) * v0)).sqrt();
                    let v19264 = (v2542 + ((v2485 * (v19246 + (((v19246 * v19246) + v19254).sqrt()))) * v0)).sqrt();
                    let v19268 = v71 * v19264;
                    let v19269 = v1 + v19268;
                    let v19271 = v19261 + v19264;
                    let v19272 = v19271 * v19271;
                    let v19278 = v19276 / (v19073 + v19214);
                    let v19303 = ((v12516 * v18848) * v4613) * ((((v19244 / (v1 + (v71 * v19261))) + (v19246 / v19269)) + (((v4724 * (v19243 / (v19272 * v19271))) * ((v16699 * (v19272 + (v19261 * v19264))) * v19278)) - (v19240 * (v19234 + ((v4724 * v19243) * v19278))))) + (v19241 * ((v19036 - ((v71 * (v19240 - v1)) * v19214)) + ((v19246 * (v19268 - v1)) / v19269))));
                    v19304 = v19303;
                } else {
                    v19304 = v0;
                }
                let v19308 = -((-v18851) + (v4673 * v19304));
                let v19309 = if v9654 == 0.0 { 1.0 } else { 0.0 };
                if v19309 != 0.0 {
                } else {
                }
                let v19310 = if v9657 == v0 { 1.0 } else { 0.0 };
                if v19310 != 0.0 {
                } else {
                }
                let v19313 = (v107 - v9659) + (v71 * v9661);
                let v19314 = if v2119 > v0 { 1.0 } else { 0.0 };
                let v19328: f64;
                if v19314 != 0.0 {
                    let v19319 = (v5519 * v4613) * ((if (v2291 / v2119) >= v4546 { (v2291 / v2119) } else { v4546 }).ln());
                    v19328 = v19319;
                } else {
                    let v19327 = (v5519 * v4613) * ((if ((((-v2291) * v2119) / v4637) / v4637) >= v4546 { ((((-v2291) * v2119) / v4637) / v4637) } else { v4546 }).ln());
                    v19328 = v19327;
                }
                let v19339 = (((v2129 * v9683) * (v9681 / v5623)) * ((((v111 / v4564) * v32) * v19313) + v9688)) * ((v9678 - v19328) - v9691);
                let v19341 = if (v5246 - v26) > v0 { 1.0 } else { 0.0 };
                if v19341 != 0.0 {
                } else {
                }
                let v19343 = if (v5266 - v26) > v0 { 1.0 } else { 0.0 };
                if v19343 != 0.0 {
                } else {
                }
                let v19344 = if v2910 != v0 { 1.0 } else { 0.0 };
                if v19344 != 0.0 {
                } else {
                }
                let v19345 = if v5389 == v1 { 1.0 } else { 0.0 };
                let v20041: f64;
                let v20043: f64;
                let v20045: f64;
                if v19345 != 0.0 {
                    let v19348 = (if (v1879 / v4637) >= v4546 { (v1879 / v4637) } else { v4546 }).ln();
                    let v19352 = if ((v4690 + (v4613 * v19348)) + v609) >= v4690 { ((v4690 + (v4613 * v19348)) + v609) } else { v4690 };
                    let v19357 = v1 + (v1979 * v4706);
                    let v19368 = v19352 - v5546;
                    let v19370 = v19368 - v5443;
                    let v19379 = v9 / (((v4697 / (v2 * v1879)).sqrt()) * ((v2485 * ((v19368 + v5443) + (((v19370 * v19370) + v19372).sqrt()))).sqrt()));
                    let v19386 = v1 + ((((v1889 + (v1869 * (v2485 * (v19357 + (((v19357 * v19357) + v19359).sqrt()))))) + (v13091 * v5542)) - (v1909 * v5546)) / v13);
                    let v19388 = v19386 - v1;
                    let v19394 = v2485 * ((v19386 + v1) + (((v19388 * v19388) + v19390).sqrt()));
                    let v19395 = v19394 * v4613;
                    let v19396 = v1 / v19395;
                    let v19397 = v5493 * v19396;
                    let v19398 = v5527 * v19396;
                    let v19399 = v5778 * v19396;
                    let v19403 = (-((v13073 * (v1 + (v1989 * v4706))) + (v1929 * v5546))) * v5542;
                    let v19410 = ((v1939 + (v1949 / v73)) + (v1959 * v5546)) * ((v4616.powf(v1969)) - v1);
                    let v19413 = v4704 * (v1 + (v13162 * v5546));
                    let v19414 = if v19413 > v0 { 1.0 } else { 0.0 };
                    let v19425: f64;
                    if v19414 != 0.0 {
                        let v19416 = (v13167 * v73) / v19413;
                        let v19417 = if v19416 < v5647 { 1.0 } else { 0.0 };
                        let v19426: f64;
                        if v19417 != 0.0 {
                            let v19421 = (v2485 * v13171) / ((v19416.cosh()) - v1);
                            v19426 = v19421;
                        } else {
                            let v19424 = v13171 * (rspice_limited_exp((-v19416)));
                            v19426 = v19424;
                        }
                        v19425 = v19426;
                    } else {
                        v19425 = v0;
                    }
                    let v19438 = (v19397 - v19399) - (((((((v19403 - v19410) + (v19425 * (v13181 - v19352))) + v13211) + v13213) - (v5489 * v5546)) + v5480) * v19396);
                    let v19451 = (((((v19444 * v9) * v1879) * v19396).sqrt()) / v13) * (v1 + (v13227 * (v1 + (v13228 * (v73.powf((-v13229)))))));
                    let v19452 = v19348 / v19394;
                    let v19457 = (v2485 * v19438) - (v2974 * (v1 + (v19451 / v16025)));
                    let v19462 = v19457 + (((v19457 * v19457) + (v2979 * v19438)).sqrt());
                    let v19463 = if v19438 < v0 { 1.0 } else { 0.0 };
                    let v19484: f64;
                    if v19463 != 0.0 {
                        let v19465 = (v19438 - v19462) / v19451;
                        let v19471 = -((if ((v1 - v19462) + (v19465 * v19465)) >= v4546 { ((v1 - v19462) + (v19465 * v19465)) } else { v4546 }).ln());
                        v19484 = v19471;
                    } else {
                        let v19473 = rspice_limited_exp((-v19462));
                        let v19474 = v2485 * v19451;
                        let v19480 = ((((v19438 - v1) + v19473) + (v19474 * v19474)).sqrt()) - v19474;
                        let v19483 = ((v19480 * v19480) + v1) - v19473;
                        v19484 = v19483;
                    }
                    let v19485 = v19484 + v1;
                    let v19486 = v19484 - v1;
                    let v19487 = v19486 * v19486;
                    let v19493 = (v2485 * (v19485 + ((v19487 + v19488).sqrt()))).sqrt();
                    let v19494 = v71 * v19493;
                    let v19497 = (v1 + (v19451 / v19494)) / v19451;
                    let v19499 = v19484 - (v71 * v19452);
                    let v19500 = v19499 - v19398;
                    let v19505 = v19500 - ((if ((v2976 * v19497) * v19493) >= v4546 { ((v2976 * v19497) * v19493) } else { v4546 }).ln());
                    let v19512 = v2485 * ((v19505 - v16078) - (((v19505 * (v19505 + v16080)) + v16083).sqrt()));
                    let v19514 = if v19512 <= v19513 { 1.0 } else { 0.0 };
                    let v19601: f64;
                    if v19514 != 0.0 {
                        let v19517 = if v19512 < v19516 { 1.0 } else { 0.0 };
                        let v19537: f64;
                        if v19517 != 0.0 {
                            v19537 = v19518;
                        } else {
                            let v19520 = if v19512 > v19519 { 1.0 } else { 0.0 };
                            let v19538: f64;
                            if v19520 != 0.0 {
                                let v19521 = rspice_limited_exp(v19512);
                                v19538 = v19521;
                            } else {
                                let v19523 = (v19512 - v19515) / v5448;
                                let v19524 = v19523 * v19523;
                                let v19536 = rspice_limited_exp((v19515 + (v5448 * ((v19525 + (v2485 * v19523)) + (v19524 * (v19528 - (v19524 * (v6052 - v19524))))))));
                                v19538 = v19536;
                            }
                            v19537 = v19538;
                        }
                        let v19549 = v19537 * (((v1 + v19500) - v19512) - ((if ((v71 * v19497) * (((v19537 * v71) * v19497) + v19494)) >= v4546 { ((v71 * v19497) * (((v19537 * v71) * v19497) + v19494)) } else { v4546 }).ln()));
                        v19601 = v19549;
                    } else {
                        let v19550 = rspice_limited_exp(v19512);
                        let v19552 = v71 * v19550;
                        let v19553 = v19552 * v19497;
                        let v19562 = v19497 + (v1 / v19493);
                        let v19568 = v19550 - (((v19552 + ((if (v19553 * (v19553 + v19494)) >= v4546 { (v19553 * (v19553 + v19494)) } else { v4546 }).ln())) - v19500) / ((v71 + (v1 / v19550)) + (v19562 / ((v19497 * v19550) + v19493))));
                        let v19569 = v71 * v19568;
                        let v19570 = v19569 * v19497;
                        let v19576 = (v19569 + ((if (v19570 * (v19570 + v19494)) >= v4546 { (v19570 * (v19570 + v19494)) } else { v4546 }).ln())) - v19500;
                        let v19577 = v1 / v19568;
                        let v19580 = (v19497 * v19568) + v19493;
                        let v19581 = v19562 / v19580;
                        let v19582 = (v71 + v19577) + v19581;
                        let v19599 = v19568 - ((v19576 / v19582) * (v1 + ((v19576 * (((-(v19577 * v19577)) - (v1 / (((v19493 * v19493) * v19493) * v19580))) - (v19581 * v19581))) / ((v71 * v19582) * v19582))));
                        v19601 = v19599;
                    }
                    let v19600 = v71 * v19395;
                    let v19605 = (((v19600 * v19601) + v19600) + v5527) - v5527;
                    let v19625 = (v2485 * (v19485 + ((v19487 + v19620).sqrt()))).sqrt();
                    let v19626 = v71 * v19625;
                    let v19629 = (v1 + (v19451 / v19626)) / v19451;
                    let v19630 = v19499 - (((v5528 * ((v1 + (((v5528 / (v2485 * (v19605 + (((v19605 * v19605) + v19607).sqrt())))) + v127).powf(v16550))).powf(v16553))) + v5527) * v19396);
                    let v19635 = v19630 - ((if ((v2976 * v19629) * v19625) >= v4546 { ((v2976 * v19629) * v19625) } else { v4546 }).ln());
                    let v19642 = v2485 * ((v19635 - v16078) - (((v19635 * (v19635 + v16080)) + v16083).sqrt()));
                    let v19644 = if v19642 <= v19643 { 1.0 } else { 0.0 };
                    let v19737: f64;
                    if v19644 != 0.0 {
                        let v19647 = if v19642 < v19646 { 1.0 } else { 0.0 };
                        let v19667: f64;
                        if v19647 != 0.0 {
                            v19667 = v19648;
                        } else {
                            let v19650 = if v19642 > v19649 { 1.0 } else { 0.0 };
                            let v19668: f64;
                            if v19650 != 0.0 {
                                let v19651 = rspice_limited_exp(v19642);
                                v19668 = v19651;
                            } else {
                                let v19653 = (v19642 - v19645) / v5448;
                                let v19654 = v19653 * v19653;
                                let v19666 = rspice_limited_exp((v19645 + (v5448 * ((v19655 + (v2485 * v19653)) + (v19654 * (v19658 - (v19654 * (v6052 - v19654))))))));
                                v19668 = v19666;
                            }
                            v19667 = v19668;
                        }
                        let v19679 = v19667 * (((v1 + v19630) - v19642) - ((if ((v71 * v19629) * (((v19667 * v71) * v19629) + v19626)) >= v4546 { ((v71 * v19629) * (((v19667 * v71) * v19629) + v19626)) } else { v4546 }).ln()));
                        v19737 = v19679;
                    } else {
                        let v19680 = rspice_limited_exp(v19642);
                        let v19682 = v71 * v19680;
                        let v19683 = v19682 * v19629;
                        let v19692 = v19629 + (v1 / v19625);
                        let v19698 = v19680 - (((v19682 + ((if (v19683 * (v19683 + v19626)) >= v4546 { (v19683 * (v19683 + v19626)) } else { v4546 }).ln())) - v19630) / ((v71 + (v1 / v19680)) + (v19692 / ((v19629 * v19680) + v19625))));
                        let v19699 = v71 * v19698;
                        let v19700 = v19699 * v19629;
                        let v19706 = (v19699 + ((if (v19700 * (v19700 + v19626)) >= v4546 { (v19700 * (v19700 + v19626)) } else { v4546 }).ln())) - v19630;
                        let v19707 = v1 / v19698;
                        let v19710 = (v19629 * v19698) + v19625;
                        let v19711 = v19692 / v19710;
                        let v19712 = (v71 + v19707) + v19711;
                        let v19729 = v19698 - ((v19706 / v19712) * (v1 + ((v19706 * (((-(v19707 * v19707)) - (v1 / (((v19625 * v19625) * v19625) * v19710))) - (v19711 * v19711))) / ((v71 * v19712) * v19712))));
                        v19737 = v19729;
                    }
                    let v19739 = ((v19484 - v19601) - v19737) - v1;
                    let v19741 = v19739 - v1;
                    let v19747 = v2485 * ((v19739 + v1) + (((v19741 * v19741) + v19743).sqrt()));
                    let v19751 = v1 + (v19451 / (((v2485 * (v19485 + ((v19487 + v19730).sqrt()))).sqrt()) + (v19747.sqrt())));
                    let v19759 = v19601 - v19737;
                    let v19764 = ((((((((v17015 * v19751) * v17014) * v16022) / v73) * v13) * v19395) * v19395) * (v19759 * ((v1 + v19601) + v19737))) * v16853;
                    let v19766 = v12801 * v19765;
                    let v19767 = v12802 * v19765;
                    let v19768 = v12805 * v19765;
                    let v19771 = v73 - (v71 * v19769);
                    let v19775 = v17713 * ((v13 + v19379) + v1889);
                    let v19778 = ((v71 * v19751) * v13) * v4613;
                    let v19780 = (v19778 * v19737) / v2;
                    let v19793 = v19780 + v19775;
                    let v19798 = (v19778 * v19601) / v2;
                    let v19822 = (((((v19781 * v4613) * (v19764.abs())) * v17014) / v19747) * (((v19766 * ((if ((v19798 + v19775) / v19793) >= v4546 { ((v19798 + v19775) / v19793) } else { v4546 }).ln())) + (v19767 * (v19798 - v19780))) + ((v2485 * v19768) * ((v19798 * v19798) - (v19780 * v19780))))) + ((((((v17729 * v19764) * v19764) / (((v12825 * (v19771 * v19771)) * v16022) * v32)) * v18000) * ((v19766 + (v19767 * v19780)) + ((v19768 * v19780) * v19780))) / (v19793 * v19793));
                    let v19830 = ((((v19766 * v2) * v4613) / (((((v16022 * v32) * v19771) * v12825) * v19775) * v19775)) * v19764) * v19764;
                    let v19831 = v19830 + v19822;
                    let v19832 = if v19831 > v0 { 1.0 } else { 0.0 };
                    let v19841: f64;
                    if v19832 != 0.0 {
                        let v19840 = ((v19822 * v19830) / v19831) / (v1 + (v19835 * (v19759.powf(v19836))));
                        v19841 = v19840;
                    } else {
                        v19841 = v0;
                    }
                    let v19842 = v9216 * v19841;
                    v20041 = v1;
                    v20043 = v19842;
                    v20045 = v12883;
                } else {
                    v20041 = v0;
                    v20043 = v0;
                    v20045 = v0;
                }
                let v19843 = if v9216 > v0 { 1.0 } else { 0.0 };
                if v19843 != 0.0 {
                } else {
                }
                v19844 = v17545;
                v19845 = v17699;
                v19847 = v19339;
                v19851 = v19854;
                v19857 = v19860;
                v19863 = v17703;
                v19869 = v19877;
                v19884 = v19892;
                v19927 = v19308;
                v19957 = v0;
                v19958 = v0;
                v19959 = v0;
                v19960 = v0;
                v19962 = v0;
                v19964 = v0;
                v19967 = v0;
                v19970 = v0;
                v19973 = v0;
                v19976 = v0;
                v19979 = v0;
                v19982 = v0;
                v19984 = v0;
                v19986 = v0;
                v19988 = v0;
                v19990 = v0;
                v19992 = v0;
                v19994 = v19995;
                v19996 = v19997;
                v19998 = v19999;
                v20000 = v20001;
                v20002 = v20003;
                v20004 = v20005;
                v20006 = v20007;
                v20008 = v20009;
                v20010 = v20011;
                v20013 = v20014;
                v20016 = v20017;
                v20019 = v20020;
                v20022 = v20023;
                v20025 = v20026;
                v20028 = v20029;
                v20030 = v20031;
                v20032 = v20033;
                v20034 = v20035;
                v20036 = v20037;
                v20038 = v20039;
                v20040 = v20041;
                v20042 = v20043;
                v20044 = v20045;
            }
            let v19846 = if v9216 > v0 { 1.0 } else { 0.0 };
            if v19846 != 0.0 {
            } else {
            }
            if v19846 != 0.0 {
            } else {
            }
            let v19848 = if v12543 != v0 { 1.0 } else { 0.0 };
            if v19848 != 0.0 {
            } else {
            }
            if v2886 != 0.0 {
            } else {
            }
            if v19846 != 0.0 {
            } else {
            }
            let v19849 = if v4539 == v0 { 1.0 } else { 0.0 };
            let v20046: f64;
            let v20047: f64;
            if v19849 != 0.0 {
                v20046 = v0;
                v20047 = v0;
            } else {
                let v19850 = if v4539 == v71 { 1.0 } else { 0.0 };
                let v19864: f64;
                if v19850 != 0.0 {
                    let v19862 = (v19851 * v19851) / v19857;
                    v19864 = v19862;
                } else {
                    v19864 = v19857;
                }
                let v19865 = v19863 * v19864;
                v20046 = v1;
                v20047 = v19865;
            }
            let v19866 = if v2833 != v71 { 1.0 } else { 0.0 };
            let v19868 = if v19866 != 0.0 && (if v7135 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v19880: f64;
            if v19868 != 0.0 {
                let v19879 = v1 / v19869;
                v19880 = v19879;
            } else {
                v19880 = v0;
            }
            let v20048: f64;
            let v20049: f64;
            if v19868 != 0.0 {
                let v19881 = v19863 * v19880;
                v20048 = v1;
                v20049 = v19881;
            } else {
                v20048 = v0;
                v20049 = v0;
            }
            let v19883 = if v19866 != 0.0 && (if v7138 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v19895: f64;
            if v19883 != 0.0 {
                let v19894 = v1 / v19884;
                v19895 = v19894;
            } else {
                v19895 = v0;
            }
            let v20050: f64;
            let v20051: f64;
            if v19883 != 0.0 {
                let v19896 = v19863 * v19895;
                v20050 = v1;
                v20051 = v19896;
            } else {
                v20050 = v0;
                v20051 = v0;
            }
            let v19897 = if v4539 == v2974 { 1.0 } else { 0.0 };
            if v19897 != 0.0 {
            } else {
            }
            if v4589 != 0.0 {
                if v19868 != 0.0 {
                } else {
                }
                if v19883 != 0.0 {
                } else {
                }
                let v19899 = if v4592 != 0.0 && v19898 != 0.0 { 1.0 } else { 0.0 };
                if v19899 != 0.0 {
                    if v1 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
                let v19901 = if v4592 != 0.0 && v19900 != 0.0 { 1.0 } else { 0.0 };
                if v19901 != 0.0 {
                    if v1 != 0.0 {
                    } else {
                    }
                } else {
                }
            }
            let v19904 = v19902 * v19903;
            let v19907 = if (if v4671 == v0 { 1.0 } else { 0.0 }) != 0.0 || v19906 != 0.0 { 1.0 } else { 0.0 };
            if v19907 != 0.0 {
            } else {
                let v19909 = if v4592 != 0.0 && v19908 != 0.0 { 1.0 } else { 0.0 };
                if v19909 != 0.0 {
                } else {
                    let v19910 = if v4671 == v1 { 1.0 } else { 0.0 };
                    if v19910 != 0.0 {
                        let v19923 = if ((((((v19911 * v19912) * v19914) / ((v71 * v19912) + (v19914 * v73))) * v83) / v4564) / v32) < v4710 { 1.0 } else { 0.0 };
                        if v19923 != 0.0 {
                            let v19924 = if v19904 <= v4710 { 1.0 } else { 0.0 };
                            if v19924 != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                        let v19942 = if (v1 / ((v32 * ((v1039 * (v4616.powf(v1049))) * (((((v2 * v1059) * v5613) * v83) * v73) - ((-((v19927 + v19844) + v19845)) + v19847)))) / (v83 * v83))) < v4710 { 1.0 } else { 0.0 };
                        if v19942 != 0.0 {
                            let v19943 = if v19904 <= v4710 { 1.0 } else { 0.0 };
                            if v19943 != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    }
                }
            }
            let v19944 = if v77 == v71 { 1.0 } else { 0.0 };
            if v19944 != 0.0 {
            } else {
            }
            let v19946 = if v19945 < v4710 { 1.0 } else { 0.0 };
            if v19946 != 0.0 {
            } else {
            }
            if v1 != 0.0 {
                let v19948 = if (if v4592 == v0 { 1.0 } else { 0.0 }) != 0.0 || v1 != 0.0 { 1.0 } else { 0.0 };
                if v19948 != 0.0 {
                } else {
                }
            } else {
            }
            let v19956 = if (if (if v4681 != 0.0 && v19949 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v4684 != 0.0 && v19951 != 0.0 { 1.0 } else { 0.0 }) == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4673 == v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v19956 != 0.0 {
            } else {
            }
        if v19957 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v19958;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = Some(v19959);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v19960 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v19962;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v19964 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v19967;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v19970 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v19973;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v19976 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v19979;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v19982 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v19984;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v19986 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v19988;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v19990 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v19992;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v19994 == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v19996;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
            let exponent: Option<f64> = Some(v19998);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20000 == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20002;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = Some(v20004);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20006 == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20008;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 10, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20010 == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20013;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20016 == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20019;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20022 == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20025;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 13, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20028 == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20030;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20032 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20034;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20036 == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20038;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 16, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20040 == 0.0 {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20042;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 17, value: psd }); }
            let exponent: Option<f64> = Some(v20044);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20046 == 0.0 {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20047;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 18, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20048 == 0.0 {
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20049;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 19, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v20050 == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v20051;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 20, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

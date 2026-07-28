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
pub static NOISE_SOURCES: [GeneratedNoiseDescriptor; 23] = [
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 2, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF", label: Some("1overf"), kind: GeneratedNoiseKind::Flicker, equation: 3, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 4, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N2_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 5, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(16), name: "N2", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_N1_GND_CORL", label: Some("corl"), kind: GeneratedNoiseKind::White, equation: 6, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(15), name: "N1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: None, name: "0", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI_SI_ID", label: Some("id"), kind: GeneratedNoiseKind::White, equation: 9, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_SI_IGS", label: Some("igs"), kind: GeneratedNoiseKind::White, equation: 15, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_DI_IGD", label: Some("igd"), kind: GeneratedNoiseKind::White, equation: 16, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_GI_BI_IGB", label: Some("igb"), kind: GeneratedNoiseKind::White, equation: 17, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(9), name: "gi", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI_SI_1OVERF_EDGEFET", label: Some("1overf_edgefet"), kind: GeneratedNoiseKind::Flicker, equation: 18, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_D_DI1_RD", label: Some("rd"), kind: GeneratedNoiseKind::White, equation: 35, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(6), name: "di1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DI1_DI_RDRIFT_D", label: Some("rdrift_d"), kind: GeneratedNoiseKind::White, equation: 37, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_DI1_DI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 38, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(6), name: "di1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(5), name: "di", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_S_SI1_RS", label: Some("rs"), kind: GeneratedNoiseKind::White, equation: 43, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(2), name: "s", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(8), name: "si1", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SI1_SI_RDRIFT_S", label: Some("rdrift_s"), kind: GeneratedNoiseKind::White, equation: 45, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "si1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "FLICKER_SI1_SI_FLICKER", label: Some("flicker"), kind: GeneratedNoiseKind::Flicker, equation: 46, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(8), name: "si1", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(7), name: "si", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_G_GM_RG", label: Some("rg"), kind: GeneratedNoiseKind::White, equation: 52, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(1), name: "g", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(10), name: "gm", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SBULK_BI_RBPS", label: Some("rbps"), kind: GeneratedNoiseKind::White, equation: 62, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "sbulk", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_SBULK_B_RBSB", label: Some("rbsb"), kind: GeneratedNoiseKind::White, equation: 63, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(12), name: "sbulk", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "b", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_B_BI_RBPB", label: Some("rbpb"), kind: GeneratedNoiseKind::White, equation: 64, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(3), name: "b", is_internal: false }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DBULK_BI_RBPD", label: Some("rbpd"), kind: GeneratedNoiseKind::White, equation: 65, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "dbulk", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(11), name: "bi", is_internal: true }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DBULK_B_RBDB", label: Some("rbdb"), kind: GeneratedNoiseKind::White, equation: 66, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(13), name: "dbulk", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(3), name: "b", is_internal: false }, table_len: 0, table_log_interp: false },
    GeneratedNoiseDescriptor { mechanism: "WHITE_DDBULK_D_RDB", label: Some("rdb"), kind: GeneratedNoiseKind::White, equation: 79, is_current: true, branch_ordinal: None, pos: GeneratedNoiseEndpoint { local_node: Some(14), name: "ddbulk", is_internal: true }, neg: GeneratedNoiseEndpoint { local_node: Some(0), name: "d", is_internal: false }, table_len: 0, table_log_interp: false },
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
            let v2 = parameters[39];
            let v4 = -1e0f64;
            let v5 = parameters[110];
            let v6 = 8.85418e-12f64;
            let v8 = parameters[111];
            let v10 = parameters[77];
            let v13 = if parameter_given[78] { 1.0 } else { 0.0 };
            let v15 = parameters[0];
            let v16 = parameters[52];
            let v18 = parameters[1];
            let v19 = parameters[53];
            let v21 = parameters[54];
            let v24 = parameters[2];
            let v26 = parameters[56];
            let v29 = parameters[61];
            let v32 = parameters[62];
            let v36 = parameters[57];
            let v37 = parameters[58];
            let v40 = parameters[59];
            let v43 = parameters[60];
            let v46 = parameters[67];
            let v49 = parameters[68];
            let v53 = parameters[63];
            let v54 = parameters[64];
            let v57 = parameters[65];
            let v60 = parameters[66];
            let v63 = 2e0f64;
            let v67 = 1e-9f64;
            let v73 = parameters[69];
            let v74 = parameters[70];
            let v77 = parameters[71];
            let v80 = parameters[72];
            let v83 = parameters[73];
            let v84 = parameters[74];
            let v87 = parameters[75];
            let v90 = parameters[76];
            let v101 = parameters[138];
            let v114 = 1e-6f64;
            let v119 = parameters[51];
            let v121 = parameters[55];
            let v124 = parameters[818];
            let v131 = parameters[819];
            let v168 = parameters[817];
            let v177 = parameters[116];
            let v178 = parameters[117];
            let v181 = parameters[118];
            let v184 = parameters[119];
            let v187 = parameters[126];
            let v188 = parameters[127];
            let v191 = parameters[128];
            let v194 = parameters[129];
            let v197 = parameters[139];
            let v198 = parameters[140];
            let v201 = parameters[141];
            let v204 = parameters[142];
            let v207 = parameters[80];
            let v208 = parameters[89];
            let v211 = parameters[90];
            let v214 = parameters[91];
            let v217 = parameters[92];
            let v218 = parameters[101];
            let v221 = parameters[102];
            let v224 = parameters[103];
            let v227 = parameters[104];
            let v228 = parameters[105];
            let v231 = parameters[106];
            let v234 = parameters[107];
            let v237 = parameters[209];
            let v238 = parameters[210];
            let v241 = parameters[211];
            let v244 = parameters[212];
            let v247 = parameters[213];
            let v248 = parameters[220];
            let v251 = parameters[221];
            let v254 = parameters[222];
            let v257 = parameters[223];
            let v258 = parameters[226];
            let v261 = parameters[227];
            let v264 = parameters[228];
            let v267 = parameters[233];
            let v268 = parameters[236];
            let v271 = parameters[237];
            let v274 = parameters[238];
            let v277 = parameters[143];
            let v278 = parameters[144];
            let v281 = parameters[145];
            let v284 = parameters[146];
            let v287 = parameters[147];
            let v288 = parameters[148];
            let v291 = parameters[149];
            let v294 = parameters[150];
            let v297 = parameters[151];
            let v298 = parameters[152];
            let v301 = parameters[153];
            let v304 = parameters[154];
            let v307 = parameters[155];
            let v308 = parameters[156];
            let v311 = parameters[157];
            let v314 = parameters[158];
            let v317 = parameters[159];
            let v318 = parameters[160];
            let v321 = parameters[161];
            let v324 = parameters[162];
            let v327 = parameters[163];
            let v328 = parameters[164];
            let v331 = parameters[165];
            let v334 = parameters[166];
            let v337 = parameters[195];
            let v338 = parameters[202];
            let v341 = parameters[203];
            let v344 = parameters[204];
            let v347 = parameters[185];
            let v348 = parameters[192];
            let v351 = parameters[193];
            let v354 = parameters[194];
            let v357 = parameters[112];
            let v358 = parameters[113];
            let v361 = parameters[114];
            let v364 = parameters[115];
            let v367 = parameters[167];
            let v368 = parameters[168];
            let v371 = parameters[169];
            let v374 = parameters[170];
            let v377 = parameters[171];
            let v378 = parameters[172];
            let v381 = parameters[173];
            let v384 = parameters[174];
            let v387 = parameters[180];
            let v388 = parameters[182];
            let v391 = parameters[183];
            let v394 = parameters[184];
            let v397 = parameters[253];
            let v398 = parameters[254];
            let v401 = parameters[255];
            let v404 = parameters[256];
            let v407 = parameters[273];
            let v408 = parameters[276];
            let v411 = parameters[277];
            let v414 = parameters[278];
            let v417 = parameters[284];
            let v418 = parameters[291];
            let v421 = parameters[292];
            let v424 = parameters[293];
            let v427 = parameters[308];
            let v428 = parameters[311];
            let v431 = parameters[312];
            let v434 = parameters[313];
            let v437 = parameters[298];
            let v438 = parameters[299];
            let v441 = parameters[300];
            let v444 = parameters[301];
            let v447 = parameters[318];
            let v448 = parameters[319];
            let v451 = parameters[320];
            let v454 = parameters[321];
            let v457 = parameters[326];
            let v458 = parameters[333];
            let v461 = parameters[334];
            let v464 = parameters[335];
            let v467 = parameters[340];
            let v468 = parameters[343];
            let v471 = parameters[344];
            let v474 = parameters[345];
            let v477 = parameters[351];
            let v478 = parameters[354];
            let v481 = parameters[355];
            let v484 = parameters[356];
            let v487 = parameters[393];
            let v488 = parameters[394];
            let v491 = parameters[395];
            let v494 = parameters[396];
            let v497 = parameters[403];
            let v498 = parameters[404];
            let v501 = parameters[405];
            let v504 = parameters[406];
            let v507 = parameters[375];
            let v508 = parameters[376];
            let v511 = parameters[377];
            let v514 = parameters[378];
            let v517 = parameters[379];
            let v518 = parameters[380];
            let v521 = parameters[381];
            let v524 = parameters[382];
            let v527 = parameters[385];
            let v528 = parameters[386];
            let v531 = parameters[387];
            let v534 = parameters[388];
            let v537 = parameters[389];
            let v538 = parameters[390];
            let v541 = parameters[391];
            let v544 = parameters[392];
            let v547 = parameters[399];
            let v548 = parameters[400];
            let v551 = parameters[401];
            let v554 = parameters[402];
            let v557 = parameters[413];
            let v558 = parameters[416];
            let v561 = parameters[417];
            let v564 = parameters[418];
            let v567 = parameters[409];
            let v568 = parameters[410];
            let v571 = parameters[411];
            let v574 = parameters[412];
            let v577 = parameters[434];
            let v578 = parameters[435];
            let v581 = parameters[436];
            let v584 = parameters[437];
            let v587 = parameters[460];
            let v588 = parameters[463];
            let v591 = parameters[464];
            let v594 = parameters[465];
            let v597 = parameters[470];
            let v598 = parameters[471];
            let v601 = parameters[472];
            let v604 = parameters[473];
            let v607 = parameters[357];
            let v608 = parameters[358];
            let v611 = parameters[359];
            let v614 = parameters[360];
            let v617 = parameters[361];
            let v618 = parameters[362];
            let v621 = parameters[363];
            let v624 = parameters[364];
            let v627 = parameters[365];
            let v628 = parameters[366];
            let v631 = parameters[367];
            let v634 = parameters[368];
            let v637 = parameters[370];
            let v638 = parameters[371];
            let v641 = parameters[372];
            let v644 = parameters[373];
            let v647 = parameters[478];
            let v648 = parameters[481];
            let v651 = parameters[482];
            let v654 = parameters[483];
            let v657 = parameters[474];
            let v658 = parameters[475];
            let v661 = parameters[476];
            let v664 = parameters[477];
            let v667 = parameters[239];
            let v668 = parameters[240];
            let v671 = parameters[241];
            let v674 = parameters[242];
            let v677 = parameters[419];
            let v678 = parameters[420];
            let v681 = parameters[421];
            let v684 = parameters[422];
            let v687 = parameters[259];
            let v688 = parameters[260];
            let v691 = parameters[261];
            let v694 = parameters[262];
            let v697 = parameters[682];
            let v698 = parameters[683];
            let v701 = parameters[684];
            let v704 = parameters[685];
            let v707 = parameters[686];
            let v708 = parameters[687];
            let v711 = parameters[688];
            let v714 = parameters[689];
            let v717 = parameters[484];
            let v718 = parameters[489];
            let v721 = parameters[490];
            let v724 = parameters[491];
            let v727 = parameters[494];
            let v728 = parameters[497];
            let v731 = parameters[498];
            let v734 = parameters[499];
            let v737 = parameters[935];
            let v738 = parameters[936];
            let v741 = parameters[937];
            let v744 = parameters[938];
            let v747 = parameters[939];
            let v748 = parameters[940];
            let v751 = parameters[941];
            let v754 = parameters[942];
            let v757 = parameters[943];
            let v758 = parameters[944];
            let v761 = parameters[945];
            let v764 = parameters[946];
            let v767 = parameters[630];
            let v768 = parameters[633];
            let v771 = parameters[634];
            let v774 = parameters[635];
            let v777 = parameters[636];
            let v778 = parameters[637];
            let v781 = parameters[638];
            let v784 = parameters[639];
            let v787 = parameters[640];
            let v788 = parameters[641];
            let v791 = parameters[642];
            let v794 = parameters[643];
            let v797 = parameters[644];
            let v798 = parameters[645];
            let v801 = parameters[646];
            let v804 = parameters[647];
            let v807 = parameters[648];
            let v808 = parameters[651];
            let v811 = parameters[652];
            let v814 = parameters[653];
            let v817 = parameters[654];
            let v818 = parameters[655];
            let v821 = parameters[656];
            let v824 = parameters[657];
            let v827 = parameters[658];
            let v828 = parameters[659];
            let v831 = parameters[660];
            let v834 = parameters[661];
            let v837 = parameters[662];
            let v838 = parameters[663];
            let v841 = parameters[664];
            let v844 = parameters[665];
            let v847 = parameters[824];
            let v848 = parameters[825];
            let v851 = parameters[826];
            let v854 = parameters[827];
            let v857 = parameters[829];
            let v858 = parameters[830];
            let v861 = parameters[831];
            let v864 = parameters[832];
            let v867 = parameters[834];
            let v868 = parameters[835];
            let v871 = parameters[836];
            let v874 = parameters[837];
            let v877 = parameters[838];
            let v878 = parameters[839];
            let v881 = parameters[840];
            let v884 = parameters[841];
            let v887 = parameters[843];
            let v888 = parameters[844];
            let v891 = parameters[845];
            let v894 = parameters[846];
            let v897 = parameters[847];
            let v898 = parameters[848];
            let v901 = parameters[849];
            let v904 = parameters[850];
            let v907 = parameters[852];
            let v908 = parameters[853];
            let v911 = parameters[854];
            let v914 = parameters[855];
            let v917 = parameters[856];
            let v918 = parameters[857];
            let v921 = parameters[858];
            let v924 = parameters[859];
            let v927 = parameters[862];
            let v928 = parameters[863];
            let v931 = parameters[864];
            let v934 = parameters[865];
            let v937 = parameters[877];
            let v938 = parameters[878];
            let v941 = parameters[879];
            let v944 = parameters[880];
            let v947 = parameters[885];
            let v948 = parameters[886];
            let v951 = parameters[887];
            let v954 = parameters[888];
            let v957 = parameters[881];
            let v958 = parameters[882];
            let v961 = parameters[883];
            let v964 = parameters[884];
            let v967 = parameters[537];
            let v968 = parameters[564];
            let v971 = parameters[565];
            let v974 = parameters[566];
            let v977 = parameters[538];
            let v978 = parameters[567];
            let v981 = parameters[568];
            let v984 = parameters[569];
            let v987 = parameters[539];
            let v988 = parameters[570];
            let v991 = parameters[571];
            let v994 = parameters[572];
            let v997 = parameters[540];
            let v998 = parameters[573];
            let v1001 = parameters[574];
            let v1004 = parameters[575];
            let v1007 = parameters[541];
            let v1008 = parameters[576];
            let v1011 = parameters[577];
            let v1014 = parameters[578];
            let v1017 = parameters[533];
            let v1018 = parameters[579];
            let v1021 = parameters[580];
            let v1024 = parameters[581];
            let v1027 = parameters[534];
            let v1028 = parameters[582];
            let v1031 = parameters[583];
            let v1034 = parameters[584];
            let v1037 = parameters[535];
            let v1038 = parameters[585];
            let v1041 = parameters[586];
            let v1044 = parameters[587];
            let v1047 = parameters[536];
            let v1048 = parameters[588];
            let v1051 = parameters[589];
            let v1054 = parameters[590];
            let v1057 = parameters[542];
            let v1058 = parameters[591];
            let v1061 = parameters[592];
            let v1064 = parameters[593];
            let v1067 = parameters[543];
            let v1068 = parameters[594];
            let v1071 = parameters[595];
            let v1074 = parameters[596];
            let v1077 = parameters[544];
            let v1078 = parameters[597];
            let v1081 = parameters[598];
            let v1084 = parameters[599];
            let v1087 = parameters[545];
            let v1088 = parameters[600];
            let v1091 = parameters[601];
            let v1094 = parameters[602];
            let v1097 = parameters[546];
            let v1098 = parameters[603];
            let v1101 = parameters[604];
            let v1104 = parameters[605];
            let v1107 = parameters[547];
            let v1108 = parameters[606];
            let v1111 = parameters[607];
            let v1114 = parameters[608];
            let v1117 = parameters[548];
            let v1118 = parameters[609];
            let v1121 = parameters[610];
            let v1124 = parameters[611];
            let v1127 = parameters[549];
            let v1128 = parameters[612];
            let v1131 = parameters[613];
            let v1134 = parameters[614];
            let v1137 = parameters[550];
            let v1138 = parameters[615];
            let v1141 = parameters[616];
            let v1144 = parameters[617];
            let v1147 = parameters[553];
            let v1148 = parameters[618];
            let v1151 = parameters[619];
            let v1154 = parameters[620];
            let v1157 = parameters[551];
            let v1158 = parameters[621];
            let v1161 = parameters[622];
            let v1164 = parameters[623];
            let v1167 = parameters[552];
            let v1168 = parameters[624];
            let v1171 = parameters[625];
            let v1174 = parameters[626];
            let v1177 = parameters[554];
            let v1178 = parameters[627];
            let v1181 = parameters[628];
            let v1184 = parameters[629];
            let v1187 = parameters[867];
            let v1188 = parameters[870];
            let v1191 = parameters[871];
            let v1194 = parameters[872];
            let v1197 = parameters[873];
            let v1198 = parameters[874];
            let v1201 = parameters[875];
            let v1204 = parameters[876];
            let v1207 = parameters[425];
            let v1208 = parameters[430];
            let v1211 = parameters[431];
            let v1214 = parameters[432];
            let v1217 = parameters[444];
            let v1218 = parameters[445];
            let v1221 = parameters[446];
            let v1224 = parameters[447];
            let v1227 = parameters[448];
            let v1228 = parameters[449];
            let v1231 = parameters[450];
            let v1234 = parameters[451];
            let v1237 = parameters[452];
            let v1238 = parameters[453];
            let v1241 = parameters[454];
            let v1244 = parameters[455];
            let v1247 = parameters[456];
            let v1248 = parameters[457];
            let v1251 = parameters[458];
            let v1254 = parameters[459];
            let v1257 = parameters[1046];
            let v1258 = parameters[1047];
            let v1261 = parameters[1048];
            let v1264 = parameters[1049];
            let v1267 = parameters[1054];
            let v1268 = parameters[1055];
            let v1271 = parameters[1056];
            let v1274 = parameters[1057];
            let v1277 = parameters[1050];
            let v1278 = parameters[1051];
            let v1281 = parameters[1052];
            let v1284 = parameters[1053];
            let v1287 = parameters[1058];
            let v1288 = parameters[1059];
            let v1291 = parameters[1060];
            let v1294 = parameters[1061];
            let v1297 = parameters[966];
            let v1298 = parameters[967];
            let v1301 = parameters[968];
            let v1304 = parameters[969];
            let v1307 = parameters[962];
            let v1308 = parameters[963];
            let v1311 = parameters[964];
            let v1314 = parameters[965];
            let v1317 = parameters[970];
            let v1318 = parameters[971];
            let v1321 = parameters[972];
            let v1324 = parameters[973];
            let v1327 = parameters[974];
            let v1328 = parameters[975];
            let v1331 = parameters[976];
            let v1334 = parameters[977];
            let v1337 = parameters[978];
            let v1338 = parameters[979];
            let v1341 = parameters[980];
            let v1344 = parameters[981];
            let v1347 = parameters[982];
            let v1348 = parameters[983];
            let v1351 = parameters[984];
            let v1354 = parameters[985];
            let v1357 = parameters[986];
            let v1358 = parameters[987];
            let v1361 = parameters[988];
            let v1364 = parameters[989];
            let v1367 = parameters[990];
            let v1368 = parameters[991];
            let v1371 = parameters[992];
            let v1374 = parameters[993];
            let v1377 = parameters[994];
            let v1378 = parameters[995];
            let v1381 = parameters[996];
            let v1384 = parameters[997];
            let v1387 = parameters[998];
            let v1388 = parameters[999];
            let v1391 = parameters[1000];
            let v1394 = parameters[1001];
            let v1397 = parameters[1002];
            let v1398 = parameters[1003];
            let v1401 = parameters[1004];
            let v1404 = parameters[1005];
            let v1407 = parameters[1006];
            let v1408 = parameters[1007];
            let v1411 = parameters[1008];
            let v1414 = parameters[1009];
            let v1417 = parameters[1010];
            let v1418 = parameters[1011];
            let v1421 = parameters[1012];
            let v1424 = parameters[1013];
            let v1427 = parameters[1017];
            let v1428 = parameters[1018];
            let v1431 = parameters[1019];
            let v1434 = parameters[1020];
            let v1437 = parameters[1021];
            let v1438 = parameters[1022];
            let v1441 = parameters[1023];
            let v1444 = parameters[1024];
            let v1447 = parameters[1029];
            let v1448 = parameters[1030];
            let v1451 = parameters[1031];
            let v1454 = parameters[1032];
            let v1457 = parameters[1025];
            let v1458 = parameters[1026];
            let v1461 = parameters[1027];
            let v1464 = parameters[1028];
            let v1467 = parameters[1033];
            let v1468 = parameters[1034];
            let v1471 = parameters[1035];
            let v1474 = parameters[1036];
            let v1477 = parameters[1037];
            let v1478 = parameters[1038];
            let v1481 = parameters[1039];
            let v1484 = parameters[1040];
            let v1487 = parameters[1069];
            let v1488 = parameters[1070];
            let v1491 = parameters[1071];
            let v1494 = parameters[1072];
            let v1497 = parameters[1073];
            let v1498 = parameters[1074];
            let v1501 = parameters[1075];
            let v1504 = parameters[1076];
            let v1507 = parameters[1077];
            let v1508 = parameters[1078];
            let v1511 = parameters[1079];
            let v1514 = parameters[1080];
            let v1517 = parameters[1081];
            let v1518 = parameters[1082];
            let v1521 = parameters[1083];
            let v1524 = parameters[1084];
            let v1527 = parameters[1085];
            let v1528 = parameters[1086];
            let v1531 = parameters[1087];
            let v1534 = parameters[1088];
            let v1537 = parameters[1089];
            let v1538 = parameters[1090];
            let v1541 = parameters[1091];
            let v1544 = parameters[1092];
            let v1547 = parameters[786];
            let v1548 = parameters[787];
            let v1551 = parameters[788];
            let v1554 = parameters[789];
            let v1557 = parameters[794];
            let v1558 = parameters[795];
            let v1561 = parameters[796];
            let v1564 = parameters[797];
            let v1567 = parameters[790];
            let v1568 = parameters[791];
            let v1571 = parameters[792];
            let v1574 = parameters[793];
            let v1577 = parameters[44];
            let v1579 = parameters[229];
            let v1580 = parameters[230];
            let v1583 = parameters[231];
            let v1586 = parameters[232];
            let v1589 = parameters[175];
            let v1590 = parameters[176];
            let v1593 = parameters[177];
            let v1596 = parameters[178];
            let v1599 = parameters[279];
            let v1600 = parameters[280];
            let v1603 = parameters[281];
            let v1606 = parameters[282];
            let v1609 = parameters[294];
            let v1610 = parameters[295];
            let v1613 = parameters[296];
            let v1616 = parameters[297];
            let v1619 = parameters[314];
            let v1620 = parameters[315];
            let v1623 = parameters[316];
            let v1626 = parameters[317];
            let v1629 = parameters[322];
            let v1630 = parameters[323];
            let v1633 = parameters[324];
            let v1636 = parameters[325];
            let v1639 = parameters[336];
            let v1640 = parameters[337];
            let v1643 = parameters[338];
            let v1646 = parameters[339];
            let v1649 = parameters[346];
            let v1650 = parameters[347];
            let v1653 = parameters[348];
            let v1656 = parameters[349];
            let v1659 = parameters[466];
            let v1660 = parameters[467];
            let v1663 = parameters[468];
            let v1666 = parameters[469];
            let v1669 = parameters[249];
            let v1670 = parameters[250];
            let v1673 = parameters[251];
            let v1676 = parameters[252];
            let v1679 = parameters[426];
            let v1680 = parameters[427];
            let v1683 = parameters[428];
            let v1686 = parameters[429];
            let v1689 = parameters[440];
            let v1690 = parameters[441];
            let v1693 = parameters[442];
            let v1696 = parameters[443];
            let v1699 = parameters[525];
            let v1700 = parameters[526];
            let v1703 = parameters[527];
            let v1706 = parameters[528];
            let v1709 = parameters[529];
            let v1710 = parameters[530];
            let v1713 = parameters[531];
            let v1716 = parameters[532];
            let v1719 = parameters[81];
            let v1720 = parameters[82];
            let v1726 = parameters[83];
            let v1727 = parameters[84];
            let v1734 = parameters[85];
            let v1735 = parameters[86];
            let v1741 = parameters[87];
            let v1742 = parameters[88];
            let v1749 = parameters[214];
            let v1750 = parameters[215];
            let v1756 = parameters[216];
            let v1757 = parameters[217];
            let v1763 = parameters[218];
            let v1764 = parameters[219];
            let v1771 = parameters[224];
            let v1772 = parameters[225];
            let v1782 = parameters[234];
            let v1783 = parameters[235];
            let v1791 = parameters[34];
            let v1793 = parameters[50];
            let v1795 = parameters[275];
            let v1797 = parameters[274];
            let v1810 = parameters[269];
            let v1812 = parameters[270];
            let v1817 = parameters[271];
            let v1818 = parameters[272];
            let v1825 = parameters[285];
            let v1826 = parameters[286];
            let v1832 = parameters[287];
            let v1833 = parameters[288];
            let v1839 = parameters[289];
            let v1840 = parameters[290];
            let v1849 = parameters[302];
            let v1850 = parameters[303];
            let v1856 = parameters[304];
            let v1857 = parameters[305];
            let v1863 = parameters[306];
            let v1864 = parameters[307];
            let v1871 = parameters[309];
            let v1872 = parameters[310];
            let v1882 = parameters[327];
            let v1883 = parameters[328];
            let v1889 = parameters[329];
            let v1890 = parameters[330];
            let v1896 = parameters[331];
            let v1897 = parameters[332];
            let v1906 = parameters[179];
            let v1914 = parameters[181];
            let v1920 = parameters[461];
            let v1921 = parameters[462];
            let v1931 = parameters[257];
            let v1932 = parameters[258];
            let v1940 = 5e-1f64;
            let v1942 = parameters[479];
            let v1943 = parameters[480];
            let v1951 = parameters[341];
            let v1952 = parameters[342];
            let v1964 = parameters[243];
            let v1965 = parameters[244];
            let v1971 = parameters[245];
            let v1972 = parameters[246];
            let v1978 = parameters[247];
            let v1979 = parameters[248];
            let v1988 = parameters[423];
            let v1989 = parameters[424];
            let v1997 = 2.5e-1f64;
            let v2002 = parameters[438];
            let v2003 = parameters[439];
            let v2013 = parameters[485];
            let v2014 = parameters[486];
            let v2020 = parameters[487];
            let v2021 = parameters[488];
            let v2032 = parameters[495];
            let v2033 = parameters[496];
            let v2041 = parameters[519];
            let v2042 = parameters[520];
            let v2048 = parameters[518];
            let v2051 = parameters[522];
            let v2052 = parameters[523];
            let v2058 = parameters[521];
            let v2061 = parameters[631];
            let v2064 = parameters[632];
            let v2068 = parameters[649];
            let v2071 = parameters[650];
            let v2075 = parameters[557];
            let v2078 = parameters[558];
            let v2082 = parameters[559];
            let v2085 = parameters[560];
            let v2089 = parameters[561];
            let v2092 = parameters[562];
            let v2096 = parameters[556];
            let v2097 = parameters[563];
            let v2101 = parameters[93];
            let v2102 = parameters[94];
            let v2108 = parameters[95];
            let v2109 = parameters[96];
            let v2116 = parameters[97];
            let v2117 = parameters[98];
            let v2123 = parameters[99];
            let v2125 = parameters[100];
            let v2132 = parameters[120];
            let v2133 = parameters[121];
            let v2139 = parameters[122];
            let v2140 = parameters[123];
            let v2146 = parameters[124];
            let v2147 = parameters[125];
            let v2154 = parameters[130];
            let v2155 = parameters[131];
            let v2161 = parameters[132];
            let v2162 = parameters[133];
            let v2168 = parameters[134];
            let v2169 = parameters[135];
            let v2176 = parameters[263];
            let v2177 = parameters[264];
            let v2183 = parameters[265];
            let v2184 = parameters[266];
            let v2190 = parameters[267];
            let v2191 = parameters[268];
            let v2198 = parameters[352];
            let v2199 = parameters[353];
            let v2208 = parameters[186];
            let v2209 = parameters[187];
            let v2215 = parameters[188];
            let v2216 = parameters[189];
            let v2222 = parameters[190];
            let v2223 = parameters[191];
            let v2230 = parameters[196];
            let v2231 = parameters[197];
            let v2237 = parameters[198];
            let v2238 = parameters[199];
            let v2244 = parameters[200];
            let v2245 = parameters[201];
            let v2252 = parameters[383];
            let v2253 = parameters[384];
            let v2261 = parameters[828];
            let v2265 = parameters[833];
            let v2269 = parameters[842];
            let v2273 = parameters[860];
            let v2277 = parameters[866];
            let v2281 = 0.0f64;
            let v2282 = parameters[49];
            let v2284 = parameters[909];
            let v2287 = parameters[42];
            let v2289 = parameters[397];
            let v2290 = parameters[398];
            let v2298 = parameters[407];
            let v2299 = parameters[408];
            let v2307 = parameters[414];
            let v2308 = parameters[415];
            let v2332 = parameters[47];
            let v2336 = parameters[46];
            let v2348 = 6.7e-2f64;
            let v2356 = parameters[1065];
            let v2358 = parameters[1066];
            let v2361 = parameters[801];
            let v2365 = parameters[695];
            let v2366 = parameters[698];
            let v2368 = parameters[696];
            let v2369 = parameters[697];
            let v2371 = if parameter_given[3] { 1.0 } else { 0.0 };
            let v2372 = parameters[374];
            let v2373 = parameters[3];
            let v2375 = parameters[10];
            let v2379 = parameters[9];
            let v2380 = 9e0f64;
            let v2388 = parameters[6];
            let v2398 = 1.0f64;
            let v2412 = 1.0f64;
            let v2413 = 1.0f64;
            let v2417 = 5e0f64;
            let v2427 = 3e0f64;
            let v2429 = 4e0f64;
            let v2432 = 6e0f64;
            let v2446 = 7e0f64;
            let v2456 = 8e0f64;
            let v2467 = 0.0f64;
            let v2516 = 1.0f64;
            let v2517 = 1.0f64;
            let v2562 = 0.0f64;
            let v2606 = 1.0f64;
            let v2607 = 1.0f64;
            let v2650 = 0.0f64;
            let v2696 = 1.0f64;
            let v2697 = 1.0f64;
            let v2740 = 0.0f64;
            let v2784 = 1.0f64;
            let v2785 = 1.0f64;
            let v2833 = 1.0f64;
            let v2834 = 1.0f64;
            let v2882 = 1.0f64;
            let v2885 = 0.0f64;
            let v2931 = 1.0f64;
            let v2936 = 0.0f64;
            let v2983 = 1.0f64;
            let v2995 = 1e1f64;
            let v2997 = 1.0f64;
            let v3167 = if parameter_given[4] { 1.0 } else { 0.0 };
            let v3168 = parameters[4];
            let v3189 = 0.0f64;
            let v3203 = 0.0f64;
            let v3204 = 1.0f64;
            let v3254 = 0.0f64;
            let v3305 = 0.0f64;
            let v3306 = 1.0f64;
            let v3351 = 0.0f64;
            let v3395 = 0.0f64;
            let v3396 = 1.0f64;
            let v3439 = 0.0f64;
            let v3485 = 0.0f64;
            let v3486 = 1.0f64;
            let v3529 = 0.0f64;
            let v3573 = 0.0f64;
            let v3574 = 1.0f64;
            let v3622 = 0.0f64;
            let v3623 = 1.0f64;
            let v3671 = 0.0f64;
            let v3674 = 0.0f64;
            let v3720 = 0.0f64;
            let v3725 = 0.0f64;
            let v3772 = 0.0f64;
            let v3785 = 0.0f64;
            let v3962 = parameters[1093];
            let v3978 = parameters[8];
            let v3980 = 1e6f64;
            let v3982 = 1e-38f64;
            let v3990 = parameters[11];
            let v3991 = parameters[12];
            let v3992 = parameters[13];
            let v3993 = parameters[14];
            let v3994 = parameters[15];
            let v3995 = if parameter_given[757] { 1.0 } else { 0.0 };
            let v3997 = if parameter_given[761] { 1.0 } else { 0.0 };
            let v4000 = if parameter_given[773] { 1.0 } else { 0.0 };
            let v4002 = if parameter_given[774] { 1.0 } else { 0.0 };
            let v4005 = if parameter_given[775] { 1.0 } else { 0.0 };
            let v4007 = if parameter_given[776] { 1.0 } else { 0.0 };
            let v4015 = parameters[773];
            let v4016 = parameters[777];
            let v4018 = parameters[778];
            let v4021 = parameters[779];
            let v4026 = parameters[774];
            let v4027 = parameters[780];
            let v4029 = parameters[781];
            let v4032 = parameters[782];
            let v4040 = parameters[775];
            let v4042 = parameters[776];
            let v4049 = parameters[757];
            let v4050 = parameters[758];
            let v4052 = parameters[759];
            let v4055 = parameters[760];
            let v4060 = parameters[761];
            let v4061 = parameters[762];
            let v4063 = parameters[763];
            let v4066 = parameters[764];
            let v4071 = parameters[765];
            let v4072 = parameters[766];
            let v4074 = parameters[767];
            let v4077 = parameters[768];
            let v4082 = parameters[769];
            let v4083 = parameters[770];
            let v4085 = parameters[771];
            let v4088 = parameters[772];
            let v4102 = 1e-3f64;
            let v4104 = 1e3f64;
            let v4105 = parameters[756];
            let v4143 = parameters[1097];
            let v4145 = parameters[16];
            let v4149 = parameters[1128];
            let v4151 = parameters[700];
            let v4152 = parameters[31];
            let v4154 = parameters[32];
            let v4159 = parameters[699];
            let v4165 = parameters[7];
            let v4170 = parameters[555];
            let v4183 = 4.97232e-7f64;
            let v4184 = 3.42537e-7f64;
            let v4186 = 7.45669e11f64;
            let v4187 = 1.16645e12f64;
            let v4197 = parameters[911];
            let v4204 = parameters[820];
            let v4205 = 2.7315e2f64;
            let v4206 = -2.7315e2f64;
            let v4208 = 3.0015e2f64;
            let v4210 = temperature;
            let v4211 = parameters[33];
            let v4213 = node_potentials[4];
            let v4216 = 8.617087e-5f64;
            let v4223 = parameters[109];
            let v4224 = parameters[821];
            let v4227 = parameters[822];
            let v4238 = parameters[108];
            let v4277 = parameters[5];
            let v4279 = 4e-1f64;
            let v4287 = 1.60219e-19f64;
            let v4295 = parameters[823];
            let v4299 = -1e1f64;
            let v4301 = -1e-6f64;
            let v4304 = 4e-6f64;
            let v4311 = parameters[851];
            let v4318 = 3.333333333333333e-1f64;
            let v4319 = parameters[283];
            let v4329 = -1e1f64;
            let v4331 = -1e-6f64;
            let v4334 = 4e-6f64;
            let v4345 = -1e1f64;
            let v4347 = -1e-6f64;
            let v4350 = 4e-6f64;
            let v4365 = -1e1f64;
            let v4367 = -1e-6f64;
            let v4370 = 4e-6f64;
            let v4384 = -1e1f64;
            let v4386 = -1e-6f64;
            let v4389 = 4e-6f64;
            let v4397 = -1e1f64;
            let v4399 = -1e-6f64;
            let v4402 = 4e-6f64;
            let v4420 = 1e2f64;
            let v4422 = parameters[1094];
            let v4424 = parameters[1120];
            let v4426 = parameters[1100];
            let v4427 = parameters[1121];
            let v4437 = parameters[861];
            let v4442 = -1e1f64;
            let v4444 = -1e-6f64;
            let v4447 = 4e-6f64;
            let v4458 = -1e1f64;
            let v4460 = -1e-6f64;
            let v4463 = 4e-6f64;
            let v4470 = -1e1f64;
            let v4472 = -1e-6f64;
            let v4475 = 4e-6f64;
            let v4486 = -1e1f64;
            let v4488 = -1e-6f64;
            let v4491 = 4e-6f64;
            let v4501 = -1e1f64;
            let v4503 = -1e-6f64;
            let v4506 = 4e-6f64;
            let v4520 = -1e1f64;
            let v4522 = -1e-6f64;
            let v4525 = 4e-6f64;
            let v4532 = -1e1f64;
            let v4534 = -1e-6f64;
            let v4537 = 4e-6f64;
            let v4551 = -1e1f64;
            let v4553 = -1e-6f64;
            let v4556 = 4e-6f64;
            let v4566 = -1e1f64;
            let v4568 = -1e-6f64;
            let v4571 = 4e-6f64;
            let v4582 = -1e1f64;
            let v4584 = -1e-6f64;
            let v4587 = 4e-6f64;
            let v4597 = -1e1f64;
            let v4599 = -1e-6f64;
            let v4602 = 4e-6f64;
            let v4612 = -1e1f64;
            let v4614 = -1e-6f64;
            let v4617 = 4e-6f64;
            let v4624 = parameters[889];
            let v4628 = -1e1f64;
            let v4630 = -1e-6f64;
            let v4633 = 4e-6f64;
            let v4638 = parameters[701];
            let v4641 = -1e1f64;
            let v4643 = -1e-6f64;
            let v4646 = 4e-6f64;
            let v4651 = parameters[702];
            let v4654 = parameters[890];
            let v4658 = -1e1f64;
            let v4660 = -1e-6f64;
            let v4663 = 4e-6f64;
            let v4668 = parameters[703];
            let v4671 = -1e1f64;
            let v4673 = -1e-6f64;
            let v4676 = 4e-6f64;
            let v4681 = parameters[704];
            let v4684 = parameters[891];
            let v4688 = -1e1f64;
            let v4690 = -1e-6f64;
            let v4693 = 4e-6f64;
            let v4698 = parameters[705];
            let v4701 = -1e1f64;
            let v4703 = -1e-6f64;
            let v4706 = 4e-6f64;
            let v4711 = parameters[706];
            let v4714 = parameters[707];
            let v4715 = parameters[892];
            let v4718 = 1e-2f64;
            let v4720 = -1e1f64;
            let v4722 = -1e-6f64;
            let v4725 = 4e-6f64;
            let v4732 = parameters[708];
            let v4735 = -1e1f64;
            let v4737 = -1e-6f64;
            let v4740 = 4e-6f64;
            let v4747 = parameters[709];
            let v4748 = parameters[893];
            let v4752 = -1e1f64;
            let v4754 = -1e-6f64;
            let v4757 = 4e-6f64;
            let v4764 = parameters[710];
            let v4767 = -1e1f64;
            let v4769 = -1e-6f64;
            let v4772 = 4e-6f64;
            let v4779 = parameters[711];
            let v4780 = parameters[894];
            let v4784 = -1e1f64;
            let v4786 = -1e-6f64;
            let v4789 = 4e-6f64;
            let v4796 = parameters[712];
            let v4799 = -1e1f64;
            let v4801 = -1e-6f64;
            let v4804 = 4e-6f64;
            let v4814 = parameters[895];
            let v4817 = parameters[725];
            let v4820 = parameters[719];
            let v4822 = parameters[721];
            let v4824 = parameters[723];
            let v4826 = parameters[896];
            let v4829 = parameters[726];
            let v4832 = parameters[720];
            let v4834 = parameters[722];
            let v4836 = parameters[724];
            let v4838 = parameters[735];
            let v4839 = parameters[897];
            let v4845 = parameters[737];
            let v4846 = parameters[899];
            let v4852 = parameters[739];
            let v4853 = parameters[741];
            let v4858 = parameters[901];
            let v4864 = parameters[736];
            let v4865 = parameters[898];
            let v4871 = parameters[738];
            let v4872 = parameters[900];
            let v4878 = parameters[740];
            let v4880 = parameters[902];
            let v4886 = parameters[742];
            let v4887 = parameters[903];
            let v4892 = -1e1f64;
            let v4894 = parameters[744];
            let v4895 = parameters[905];
            let v4900 = -1e1f64;
            let v4902 = parameters[746];
            let v4903 = parameters[907];
            let v4908 = -1e1f64;
            let v4910 = parameters[743];
            let v4911 = parameters[904];
            let v4916 = -1e1f64;
            let v4918 = parameters[745];
            let v4919 = parameters[906];
            let v4924 = -1e1f64;
            let v4926 = parameters[747];
            let v4927 = parameters[908];
            let v4932 = -1e1f64;
            let v5101 = if parameter_given[24] { 1.0 } else { 0.0 };
            let v5102 = parameters[24];
            let v5118 = if parameter_given[25] { 1.0 } else { 0.0 };
            let v5119 = parameters[25];
            let v5135 = if parameter_given[26] { 1.0 } else { 0.0 };
            let v5136 = parameters[137];
            let v5138 = parameters[26];
            let v5156 = if parameter_given[27] { 1.0 } else { 0.0 };
            let v5158 = parameters[27];
            let v5187 = parameters[729];
            let v5190 = -1e1f64;
            let v5202 = parameters[730];
            let v5205 = -1e1f64;
            let v5207 = parameters[17];
            let v5209 = parameters[18];
            let v5214 = parameters[19];
            let v5219 = parameters[921];
            let v5221 = parameters[914];
            let v5223 = parameters[922];
            let v5225 = parameters[918];
            let v5227 = parameters[919];
            let v5230 = parameters[920];
            let v5235 = parameters[927];
            let v5237 = parameters[928];
            let v5239 = parameters[924];
            let v5241 = parameters[925];
            let v5244 = parameters[926];
            let v5249 = parameters[917];
            let v5271 = parameters[912];
            let v5275 = parameters[913];
            let v5279 = parameters[915];
            let v5287 = parameters[916];
            let v5293 = parameters[923];
            let v5297 = parameters[929];
            let v5298 = parameters[930];
            let v5302 = parameters[931];
            let v5303 = parameters[932];
            let v5312 = parameters[37];
            let v5324 = parameters[43];
            let v5327 = parameters[20];
            let v5328 = parameters[21];
            let v5329 = parameters[22];
            let v5330 = if parameter_given[20] { 1.0 } else { 0.0 };
            let v5332 = if parameter_given[21] { 1.0 } else { 0.0 };
            let v5335 = if parameter_given[22] { 1.0 } else { 0.0 };
            let v5338 = if parameter_given[23] { 1.0 } else { 0.0 };
            let v5339 = parameters[23];
            let v5343 = parameters[947];
            let v5348 = 1e-1f64;
            let v5352 = -1e1f64;
            let v5359 = -1e1f64;
            let v5366 = 5e-2f64;
            let v5368 = 2.5e-3f64;
            let v5371 = 2e1f64;
            let v5372 = -2e1f64;
            let v5379 = -2e1f64;
            let v5389 = parameters[933];
            let v5395 = parameters[934];
            let v5411 = node_potentials[9];
            let v5412 = node_potentials[11];
            let v5415 = node_potentials[5];
            let v5418 = node_potentials[7];
            let v5422 = node_potentials[12];
            let v5425 = node_potentials[13];
            let v5428 = node_potentials[14];
            let v5433 = node_potentials[10];
            let v5437 = parameters[1110];
            let v5440 = parameters[1095];
            let v5444 = parameters[1111];
            let v5448 = node_potentials[6];
            let v5458 = -1e0f64;
            let v5465 = parameters[956];
            let v5467 = 3.7e1f64;
            let v5469 = -3.7e1f64;
            let v5480 = 6.931471805599453e-1f64;
            let v5489 = -3.7e1f64;
            let v5499 = 6.931471805599453e-1f64;
            let v5506 = parameters[1123];
            let v5571 = 0.0f64;
            let v5573 = -2.5e2f64;
            let v5576 = -1.0000000000000002e-2f64;
            let v5577 = 1.6e1f64;
            let v5583 = 2.5000000000000005e-3f64;
            let v5600 = 0.0f64;
            let v5601 = -1.25e2f64;
            let v5604 = -2.5000000000000005e-3f64;
            let v5610 = 6.250000000000001e-4f64;
            let v5624 = 6.25e-6f64;
            let v5629 = 1.25e-3f64;
            let v5631 = parameters[869];
            let v5636 = parameters[868];
            let v5643 = 8e1f64;
            let v5644 = -8e1f64;
            let v5646 = 1.804851387e-35f64;
            let v5665 = parameters[35];
            let v5683 = 3.20438e-19f64;
            let v5692 = -1e1f64;
            let v5694 = -1e-6f64;
            let v5697 = 4e-6f64;
            let v5707 = -6.931471805599453e-1f64;
            let v5709 = 1e0f64;
            let v5719 = -1e1f64;
            let v5721 = 3.20438e-19f64;
            let v5730 = 1.4142135623730951e0f64;
            let v5765 = 1e0f64;
            let v5783 = 2.01491e-1f64;
            let v5785 = 4.02982e-1f64;
            let v5788 = 2.446562e0f64;
            let v5793 = -6.8e1f64;
            let v5795 = -1e2f64;
            let v5796 = -1.1e2f64;
            let v5798 = 1.804851387e-35f64;
            let v5799 = -9e1f64;
            let v5805 = 7.8125e-2f64;
            let v5808 = 1.5e1f64;
            let v5809 = 9.375e-1f64;
            let v5810 = 1.25e0f64;
            let v5842 = 1e0f64;
            let v5860 = 1e0f64;
            let v5868 = -1e0f64;
            let v5872 = 1e0f64;
            let v5887 = 0.0f64;
            let v5888 = -5e3f64;
            let v5891 = -4e0f64;
            let v5894 = 1e0f64;
            let v5904 = 0.0f64;
            let v5905 = -5e3f64;
            let v5908 = -4e0f64;
            let v5914 = 1e0f64;
            let v5924 = 1e-8f64;
            let v5932 = 1.0f64;
            let v5933 = -2.5e2f64;
            let v5936 = -1.0000000000000002e-2f64;
            let v5940 = 2.5000000000000005e-3f64;
            let v5967 = 0.0f64;
            let v5968 = -3.75e0f64;
            let v5971 = -2.25e-6f64;
            let v5977 = 5.625e-7f64;
            let v6031 = parameters[433];
            let v6105 = -2e0f64;
            let v6112 = -2e0f64;
            let v6153 = -2e0f64;
            let v6160 = -2e0f64;
            let v6205 = -2e0f64;
            let v6212 = -2e0f64;
            let v6244 = -2e0f64;
            let v6251 = -2e0f64;
            let v6276 = parameters[1130];
            let v6278 = parameters[1131];
            let v6287 = parameters[1132];
            let v6292 = parameters[1133];
            let v6297 = 0.0f64;
            let v6298 = -1.25e0f64;
            let v6301 = -2.5e-7f64;
            let v6307 = 6.25e-8f64;
            let v6312 = 1.0f64;
            let v6314 = -2.5e0f64;
            let v6317 = -1e-6f64;
            let v6321 = 2.5e-7f64;
            let v6340 = 1e0f64;
            let v6363 = -6.8e1f64;
            let v6365 = -1e2f64;
            let v6366 = -1.1e2f64;
            let v6368 = 1.804851387e-35f64;
            let v6369 = -9e1f64;
            let v6375 = 7.8125e-2f64;
            let v6378 = 9.375e-1f64;
            let v6410 = 1e0f64;
            let v6428 = 1e0f64;
            let v6436 = -1e0f64;
            let v6440 = 1e0f64;
            let v6459 = 0.0f64;
            let v6460 = -5e3f64;
            let v6463 = -4e0f64;
            let v6469 = 1e0f64;
            let v6494 = 8e-1f64;
            let v6497 = 1.2e0f64;
            let v6514 = 1.0f64;
            let v6516 = -2.5e2f64;
            let v6519 = -1.0000000000000002e-2f64;
            let v6523 = 2.5000000000000005e-3f64;
            let v6543 = 0.0f64;
            let v6544 = -3.75e0f64;
            let v6547 = -2.25e-6f64;
            let v6553 = 5.625e-7f64;
            let v6580 = -1e1f64;
            let v6582 = -1e-6f64;
            let v6585 = 4e-6f64;
            let v6608 = parameters[350];
            let v6648 = parameters[369];
            let v6655 = 5.540622384e34f64;
            let v6714 = node_potentials[8];
            let v6812 = -1e0f64;
            let v6814 = 4e-3f64;
            let v6827 = 2.5e-5f64;
            let v6832 = 2.5e-3f64;
            let v6867 = parameters[36];
            let v6870 = parameters[1117];
            let v6880 = parameters[1113];
            let v6883 = 1.0f64;
            let v6884 = -2.5e0f64;
            let v6887 = -1e-6f64;
            let v6891 = 2.5e-7f64;
            let v6896 = parameters[1102];
            let v6898 = 0.0f64;
            let v6899 = -5e3f64;
            let v6902 = -4e0f64;
            let v6908 = 1e0f64;
            let v6913 = parameters[1103];
            let v6920 = parameters[1101];
            let v6929 = parameters[1127];
            let v6931 = 1.0f64;
            let v6932 = parameters[1126];
            let v6934 = -1.25e3f64;
            let v6937 = -2.5e-1f64;
            let v6941 = 6.25e-2f64;
            let v6949 = parameters[514];
            let v6953 = parameters[1098];
            let v6957 = node_potentials[3];
            let v6960 = -3e0f64;
            let v6964 = parameters[515];
            let v6969 = parameters[1099];
            let v6982 = 1.0f64;
            let v6983 = -1.25e2f64;
            let v6986 = -2.5000000000000005e-3f64;
            let v6990 = 6.250000000000001e-4f64;
            let v6995 = parameters[1124];
            let v7001 = parameters[1125];
            let v7004 = 1.0f64;
            let v7005 = -1.25e2f64;
            let v7008 = -2.5000000000000005e-3f64;
            let v7012 = 6.250000000000001e-4f64;
            let v7026 = parameters[1107];
            let v7029 = parameters[1122];
            let v7038 = 1.0f64;
            let v7039 = -2.5e0f64;
            let v7042 = -1e-6f64;
            let v7046 = 2.5e-7f64;
            let v7056 = parameters[1112];
            let v7060 = parameters[516];
            let v7069 = -3e0f64;
            let v7073 = parameters[517];
            let v7078 = parameters[1109];
            let v7087 = 1.0f64;
            let v7088 = -1.25e2f64;
            let v7091 = -2.5000000000000005e-3f64;
            let v7095 = 6.250000000000001e-4f64;
            let v7107 = 1.0f64;
            let v7108 = -1.25e2f64;
            let v7111 = -2.5000000000000005e-3f64;
            let v7115 = 6.250000000000001e-4f64;
            let v7137 = 1.0f64;
            let v7138 = -2.5e0f64;
            let v7141 = -1e-6f64;
            let v7145 = 2.5e-7f64;
            let v7160 = 0e0f64;
            let v7166 = parameters[1108];
            let v7174 = 1e0f64;
            let v7181 = 0.0f64;
            let v7182 = -2.5e3f64;
            let v7190 = -1e0f64;
            let v7192 = -1e0f64;
            let v7194 = -1e0f64;
            let v7202 = 1e0f64;
            let v7222 = 1e0f64;
            let v7229 = 0.0f64;
            let v7230 = -2.5e3f64;
            let v7238 = -1e0f64;
            let v7240 = -1e0f64;
            let v7242 = -1e0f64;
            let v7250 = 1e0f64;
            let v7271 = 1e0f64;
            let v7278 = 0.0f64;
            let v7279 = -2.5e3f64;
            let v7287 = -1e0f64;
            let v7289 = -1e0f64;
            let v7291 = -1e0f64;
            let v7299 = 1e0f64;
            let v7307 = parameters[28];
            let v7312 = parameters[1114];
            let v7315 = 3.20438e-19f64;
            let v7359 = 1e0f64;
            let v7385 = -6.8e1f64;
            let v7387 = -1e2f64;
            let v7388 = -1.1e2f64;
            let v7390 = 1.804851387e-35f64;
            let v7391 = -9e1f64;
            let v7397 = 7.8125e-2f64;
            let v7400 = 9.375e-1f64;
            let v7432 = 1e0f64;
            let v7450 = 1e0f64;
            let v7458 = -1e0f64;
            let v7462 = 1e0f64;
            let v7477 = 0.0f64;
            let v7478 = -5e3f64;
            let v7484 = 0.0f64;
            let v7485 = -5e3f64;
            let v7488 = parameters[1118];
            let v7490 = parameters[1096];
            let v7528 = 1e0f64;
            let v7553 = -6.8e1f64;
            let v7555 = -1e2f64;
            let v7556 = -1.1e2f64;
            let v7558 = 1.804851387e-35f64;
            let v7559 = -9e1f64;
            let v7565 = 7.8125e-2f64;
            let v7568 = 9.375e-1f64;
            let v7600 = 1e0f64;
            let v7618 = 1e0f64;
            let v7626 = -1e0f64;
            let v7630 = 1e0f64;
            let v7645 = 0.0f64;
            let v7646 = -5e3f64;
            let v7652 = 0.0f64;
            let v7653 = -5e3f64;
            let v7661 = parameters[755];
            let v7667 = parameters[754];
            let v7702 = 1.0f64;
            let v7703 = -2.5e0f64;
            let v7706 = -1e-6f64;
            let v7710 = 2.5e-7f64;
            let v7719 = parameters[493];
            let v7720 = parameters[492];
            let v7726 = parameters[505];
            let v7729 = parameters[506];
            let v7734 = 1.0f64;
            let v7735 = -2.5e-9f64;
            let v7746 = parameters[1105];
            let v7748 = 0.0f64;
            let v7749 = -5e3f64;
            let v7752 = -4e0f64;
            let v7758 = 1e0f64;
            let v7763 = parameters[1106];
            let v7769 = parameters[1104];
            let v7773 = parameters[502];
            let v7781 = 1.0f64;
            let v7782 = -2.5e3f64;
            let v7783 = parameters[504];
            let v7801 = 1.0f64;
            let v7802 = node_potentials[0];
            let v7803 = node_potentials[2];
            let v7806 = parameters[512];
            let v7811 = parameters[503];
            let v7818 = parameters[513];
            let v7822 = -1.25e2f64;
            let v7825 = -2.5000000000000005e-3f64;
            let v7829 = 6.250000000000001e-4f64;
            let v7834 = 1.0f64;
            let v7840 = -1.25e2f64;
            let v7843 = -2.5000000000000005e-3f64;
            let v7847 = 6.250000000000001e-4f64;
            let v7852 = 3.20438e-19f64;
            let v7860 = parameters[507];
            let v7862 = parameters[508];
            let v7866 = parameters[509];
            let v7868 = parameters[510];
            let v7869 = parameters[511];
            let v7873 = parameters[500];
            let v7877 = 1.0f64;
            let v7878 = -2.5e-9f64;
            let v7881 = parameters[501];
            let v7889 = 1e-4f64;
            let v7901 = -3.7e1f64;
            let v7915 = -7.45669e11f64;
            let v7932 = -3.7e1f64;
            let v7946 = -9.82222e11f64;
            let v7951 = 3.75956e-7f64;
            let v7993 = 2e-4f64;
            let v8008 = parameters[1041];
            let v8012 = -1e-2f64;
            let v8014 = -1e-12f64;
            let v8017 = 4e-12f64;
            let v8047 = -1e-2f64;
            let v8049 = -1e-12f64;
            let v8052 = 4e-12f64;
            let v8086 = parameters[45];
            let v8096 = -1e2f64;
            let v8105 = -1e-2f64;
            let v8115 = -1e2f64;
            let v8124 = -1e-2f64;
            let v8127 = parameters[748];
            let v8132 = parameters[750];
            let v8137 = parameters[752];
            let v8146 = parameters[749];
            let v8154 = parameters[751];
            let v8159 = parameters[753];
            let v8175 = parameters[713];
            let v8177 = parameters[715];
            let v8179 = parameters[717];
            let v8183 = 9e-1f64;
            let v8210 = parameters[714];
            let v8212 = parameters[716];
            let v8214 = parameters[718];
            let v8254 = parameters[38];
            let v8259 = parameters[784];
            let v8279 = 4.112842231783458e-57f64;
            let v8287 = parameters[785];
            let v8288 = parameters[799];
            let v8291 = parameters[800];
            let v8301 = 3.20438e-19f64;
            let v8303 = parameters[1068];
            let v8345 = 1e0f64;
            let v8370 = -6.8e1f64;
            let v8372 = -1e2f64;
            let v8373 = -1.1e2f64;
            let v8375 = 1.804851387e-35f64;
            let v8376 = -9e1f64;
            let v8382 = 7.8125e-2f64;
            let v8385 = 9.375e-1f64;
            let v8417 = 1e0f64;
            let v8435 = 1e0f64;
            let v8443 = -1e0f64;
            let v8447 = 1e0f64;
            let v8462 = 0.0f64;
            let v8463 = -5e3f64;
            let v8466 = -4e0f64;
            let v8469 = 1e0f64;
            let v8483 = 0e0f64;
            let v8503 = -5e-1f64;
            let v8507 = -5e-1f64;
            let v8540 = 0e0f64;
            let v8545 = 1e10f64;
            let v8586 = parameters[1067];
            let v8604 = parameters[30];
            let v8607 = parameters[783];
            let v8625 = parameters[798];
            let v8682 = parameters[802];
            let v8683 = parameters[803];
            let v8695 = parameters[811];
            let v8696 = parameters[814];
            let v8701 = parameters[812];
            let v8702 = parameters[815];
            let v8707 = parameters[1043];
            let v8708 = parameters[1044];
            let v8713 = parameters[813];
            let v8714 = parameters[816];
            let v8723 = parameters[1042];
            let v8730 = parameters[48];
            let v8748 = parameters[810];
            let v8770 = parameters[1045];
            let v8777 = 1.0f64;
            let v8778 = -2.5e2f64;
            let v8781 = -1.0000000000000002e-2f64;
            let v8785 = 2.5000000000000005e-3f64;
            let v8796 = 1.2e1f64;
            let v8804 = 6e1f64;
            let v8810 = 1.44e2f64;
            let v8829 = 3.95e-1f64;
            let v8860 = parameters[40];
            let v8870 = 3.20438e-19f64;
            let v8877 = 3.20438e-19f64;
            let v8925 = 1e0f64;
            let v8938 = parameters[1137];
            let v8952 = -6.8e1f64;
            let v8954 = -1e2f64;
            let v8955 = -1.1e2f64;
            let v8957 = 1.804851387e-35f64;
            let v8958 = -9e1f64;
            let v8964 = 7.8125e-2f64;
            let v8967 = 9.375e-1f64;
            let v9048 = 0.0f64;
            let v9049 = -5e3f64;
            let v9052 = -4e0f64;
            let v9055 = 1e0f64;
            let v9065 = 0.0f64;
            let v9066 = -5e3f64;
            let v9069 = -4e0f64;
            let v9075 = 1e0f64;
            let v9090 = 1.0f64;
            let v9091 = -2.5e2f64;
            let v9094 = -1.0000000000000002e-2f64;
            let v9098 = 2.5000000000000005e-3f64;
            let v9115 = 0.0f64;
            let v9116 = -3.75e0f64;
            let v9119 = -2.25e-6f64;
            let v9125 = 5.625e-7f64;
            let v9154 = 1.0f64;
            let v9156 = -2.5e0f64;
            let v9159 = -1e-6f64;
            let v9163 = 2.5e-7f64;
            let v9168 = parameters[1134];
            let v9170 = parameters[1135];
            let v9173 = parameters[1129];
            let v9183 = parameters[1136];
            let v9188 = 0.0f64;
            let v9189 = -1.25e0f64;
            let v9192 = -2.5e-7f64;
            let v9198 = 6.25e-8f64;
            let v9215 = 1e0f64;
            let v9239 = -6.8e1f64;
            let v9241 = -1e2f64;
            let v9242 = -1.1e2f64;
            let v9244 = 1.804851387e-35f64;
            let v9245 = -9e1f64;
            let v9251 = 7.8125e-2f64;
            let v9254 = 9.375e-1f64;
            let v9339 = 0.0f64;
            let v9340 = -5e3f64;
            let v9343 = -4e0f64;
            let v9349 = 1e0f64;
            let v9374 = 1.0f64;
            let v9375 = -2.5e2f64;
            let v9378 = -1.0000000000000002e-2f64;
            let v9382 = 2.5000000000000005e-3f64;
            let v9391 = 1.0f64;
            let v9392 = -2.5e2f64;
            let v9395 = -1.0000000000000002e-2f64;
            let v9399 = 2.5000000000000005e-3f64;
            let v9413 = parameters[136];
            let v9448 = 0.0f64;
            let v9449 = -3.75e0f64;
            let v9452 = -2.25e-6f64;
            let v9458 = 5.625e-7f64;
            let v9506 = 1.0f64;
            let v9507 = -1.25e3f64;
            let v9510 = -2.5e-1f64;
            let v9514 = 6.25e-2f64;
            let v9519 = 1.0f64;
            let v9520 = -1.25e3f64;
            let v9523 = -2.5e-1f64;
            let v9527 = 6.25e-2f64;
            let v9586 = 1.0f64;
            let v9588 = -2.5e3f64;
            let v9589 = parameters[694];
            let v9593 = if parameter_given[666] { 1.0 } else { 0.0 };
            let v9595 = parameters[41];
            let v9610 = -1e1f64;
            let v9612 = -1e-6f64;
            let v9615 = 4e-6f64;
            let v9626 = 0.0f64;
            let v9628 = -2.5e2f64;
            let v9631 = -1.0000000000000002e-2f64;
            let v9637 = 2.5000000000000005e-3f64;
            let v9653 = 0.0f64;
            let v9654 = -1.25e2f64;
            let v9657 = -2.5000000000000005e-3f64;
            let v9663 = 6.250000000000001e-4f64;
            let v9685 = parameters[1016];
            let v9690 = parameters[1015];
            let v9693 = 4e1f64;
            let v9695 = parameters[1014];
            let v9710 = parameters[961];
            let v9723 = parameters[958];
            let v9724 = parameters[959];
            let v9725 = parameters[960];
            let v9731 = 3.20438e-19f64;
            let v9775 = 1e0f64;
            let v9800 = -6.8e1f64;
            let v9802 = -1e2f64;
            let v9803 = -1.1e2f64;
            let v9805 = 1.804851387e-35f64;
            let v9806 = -9e1f64;
            let v9812 = 7.8125e-2f64;
            let v9815 = 9.375e-1f64;
            let v9847 = 1e0f64;
            let v9865 = 1e0f64;
            let v9873 = -1e0f64;
            let v9877 = 1e0f64;
            let v9897 = 1.0f64;
            let v9899 = -2.5e0f64;
            let v9902 = -1e-6f64;
            let v9906 = 2.5e-7f64;
            let v9919 = 1e0f64;
            let v9942 = -6.8e1f64;
            let v9944 = -1e2f64;
            let v9945 = -1.1e2f64;
            let v9947 = 1.804851387e-35f64;
            let v9948 = -9e1f64;
            let v9954 = 7.8125e-2f64;
            let v9957 = 9.375e-1f64;
            let v9989 = 1e0f64;
            let v10007 = 1e0f64;
            let v10015 = -1e0f64;
            let v10019 = 1e0f64;
            let v10034 = 0.0f64;
            let v10035 = -5e3f64;
            let v10038 = -4e0f64;
            let v10041 = 1e0f64;
            let v10052 = 0.0f64;
            let v10053 = -5e3f64;
            let v10056 = -4e0f64;
            let v10062 = 1e0f64;
            let v10074 = parameters[957];
            let v10087 = parameters[1062];
            let v10103 = 4.112842231783458e-57f64;
            let v10157 = parameters[1063];
            let v10158 = parameters[1064];
            let v10184 = parameters[807];
            let v10188 = parameters[809];
            let v10191 = parameters[805];
            let v10205 = parameters[806];
            let v10209 = parameters[808];
            let v10212 = parameters[804];
            let v3 = if v2 == v1 { 1.0 } else { 0.0 };
            let v4270: f64;
            if v3 != 0.0 {
                v4270 = v1;
            } else {
                v4270 = v4;
            }
            let v7 = v5 * v6;
            let v9 = v8 * v6;
            let v11 = v9 / v10;
            let v12 = v5 / v8;
            let v14 = if v13 == 0.0 { 1.0 } else { 0.0 };
            if v14 != 0.0 {
            } else {
            }
            let v17 = v15 * v16;
            let v20 = v18 * v19;
            let v22 = v17 + v21;
            let v23 = if v22 <= v0 { 1.0 } else { 0.0 };
            if v23 != 0.0 {
            } else {
            }
            let v27 = (v20 / v24) + v26;
            let v28 = if v27 <= v0 { 1.0 } else { 0.0 };
            if v28 != 0.0 {
            } else {
            }
            let v30 = -v29;
            let v31 = v22.powf(v30);
            let v33 = -v32;
            let v34 = v27.powf(v33);
            let v35 = v31 * v34;
            let v47 = -v46;
            let v48 = v22.powf(v47);
            let v50 = -v49;
            let v51 = v27.powf(v50);
            let v52 = v48 * v51;
            let v62 = ((v53 + (v54 * v48)) + (v57 * v51)) + (v60 * v52);
            let v65 = v22 - (v63 * (((v36 + (v37 * v31)) + (v40 * v34)) + (v43 * v35)));
            let v66 = if v65 <= v0 { 1.0 } else { 0.0 };
            if v66 != 0.0 {
            } else {
                let v68 = if v65 <= v67 { 1.0 } else { 0.0 };
                if v68 != 0.0 {
                } else {
                }
            }
            let v70 = v27 - (v63 * v62);
            let v71 = if v70 <= v0 { 1.0 } else { 0.0 };
            if v71 != 0.0 {
            } else {
                let v72 = if v70 <= v67 { 1.0 } else { 0.0 };
                if v72 != 0.0 {
                } else {
                }
            }
            let v92 = ((v83 + (v84 * v48)) + (v87 * v51)) + (v90 * v52);
            let v94 = v22 - (v63 * (((v73 + (v74 * v31)) + (v77 * v34)) + (v80 * v35)));
            let v95 = if v94 <= v0 { 1.0 } else { 0.0 };
            if v95 != 0.0 {
            } else {
                let v96 = if v94 <= v67 { 1.0 } else { 0.0 };
                if v96 != 0.0 {
                } else {
                }
            }
            let v98 = v27 - (v63 * v92);
            let v99 = if v98 <= v0 { 1.0 } else { 0.0 };
            if v99 != 0.0 {
            } else {
                let v100 = if v98 <= v67 { 1.0 } else { 0.0 };
                if v100 != 0.0 {
                } else {
                }
            }
            let v102 = v22.powf(v46);
            let v105 = v27.powf(v49);
            let v112 = v27 - (v63 * (((v101 + (v84 / v102)) + (v87 / v105)) + ((v90 / v102) / v105)));
            let v113 = if v112 <= v0 { 1.0 } else { 0.0 };
            if v113 != 0.0 {
            } else {
            }
            let v115 = v114 / v65;
            let v116 = v114 / v70;
            let v117 = v114 / v94;
            let v118 = v114 / v98;
            let v120 = v114 / v119;
            let v122 = v114 / v121;
            let v123 = v115 * v116;
            let v125 = if v124 != v0 { 1.0 } else { 0.0 };
            let v138: f64;
            let v149: f64;
            if v125 != 0.0 {
                let v127 = if v124 <= (-v22) { 1.0 } else { 0.0 };
                let v139: f64;
                let v150: f64;
                if v127 != 0.0 {
                    v139 = v31;
                    v150 = v48;
                } else {
                    let v128 = v22 + v124;
                    let v129 = v128.powf(v30);
                    let v130 = v128.powf(v47);
                    v139 = v129;
                    v150 = v130;
                }
                v138 = v139;
                v149 = v150;
            } else {
                v138 = v31;
                v149 = v48;
            }
            let v132 = if v131 != v0 { 1.0 } else { 0.0 };
            let v140: f64;
            let v151: f64;
            if v132 != 0.0 {
                let v134 = if v131 <= (-v27) { 1.0 } else { 0.0 };
                let v141: f64;
                let v152: f64;
                if v134 != 0.0 {
                    v141 = v34;
                    v152 = v51;
                } else {
                    let v135 = v27 + v131;
                    let v136 = v135.powf(v33);
                    let v137 = v135.powf(v50);
                    v141 = v136;
                    v152 = v137;
                }
                v140 = v141;
                v151 = v152;
            } else {
                v140 = v34;
                v151 = v51;
            }
            let v159 = ((v53 + (v54 * v149)) + (v57 * v151)) + (v60 * (v149 * v151));
            let v162 = (v22 - (v63 * (((v36 + (v37 * v138)) + (v40 * v140)) + (v43 * (v138 * v140))))) + v124;
            let v163 = if v162 <= v0 { 1.0 } else { 0.0 };
            if v163 != 0.0 {
            } else {
            }
            let v166 = (v27 - (v63 * v159)) + v131;
            let v167 = if v166 <= v0 { 1.0 } else { 0.0 };
            if v167 != 0.0 {
            } else {
            }
            let v169 = if v168 == v1 { 1.0 } else { 0.0 };
            let v174: f64;
            let v175: f64;
            if v169 != 0.0 {
                let v170 = v114 / v162;
                let v171 = v114 / v166;
                v174 = v170;
                v175 = v171;
            } else {
                let v172 = v1 / v162;
                let v173 = v1 / v166;
                v174 = v172;
                v175 = v173;
            }
            let v176 = v174 * v175;
            let v186 = ((v177 + (v174 * v178)) + (v175 * v181)) + (v176 * v184);
            let v196 = ((v187 + (v174 * v188)) + (v175 * v191)) + (v176 * v194);
            let v206 = ((v197 + (v174 * v198)) + (v175 * v201)) + (v176 * v204);
            let v216 = ((v207 + (v174 * v208)) + (v175 * v211)) + (v176 * v214);
            let v226 = ((v217 + (v174 * v218)) + (v175 * v221)) + (v176 * v224);
            let v236 = ((v227 + (v174 * v228)) + (v175 * v231)) + (v176 * v234);
            let v246 = ((v237 + (v174 * v238)) + (v175 * v241)) + (v176 * v244);
            let v256 = ((v247 + (v174 * v248)) + (v175 * v251)) + (v176 * v254);
            let v266 = ((v257 + (v174 * v258)) + (v175 * v261)) + (v176 * v264);
            let v276 = ((v267 + (v174 * v268)) + (v175 * v271)) + (v176 * v274);
            let v286 = ((v277 + (v174 * v278)) + (v175 * v281)) + (v176 * v284);
            let v296 = ((v287 + (v174 * v288)) + (v175 * v291)) + (v176 * v294);
            let v306 = ((v297 + (v174 * v298)) + (v175 * v301)) + (v176 * v304);
            let v316 = ((v307 + (v174 * v308)) + (v175 * v311)) + (v176 * v314);
            let v326 = ((v317 + (v174 * v318)) + (v175 * v321)) + (v176 * v324);
            let v336 = ((v327 + (v174 * v328)) + (v175 * v331)) + (v176 * v334);
            let v346 = ((v337 + (v174 * v338)) + (v175 * v341)) + (v176 * v344);
            let v356 = ((v347 + (v174 * v348)) + (v175 * v351)) + (v176 * v354);
            let v366 = ((v357 + (v174 * v358)) + (v175 * v361)) + (v176 * v364);
            let v376 = ((v367 + (v174 * v368)) + (v175 * v371)) + (v176 * v374);
            let v386 = ((v377 + (v174 * v378)) + (v175 * v381)) + (v176 * v384);
            let v396 = ((v387 + (v174 * v388)) + (v175 * v391)) + (v176 * v394);
            let v406 = ((v397 + (v174 * v398)) + (v175 * v401)) + (v176 * v404);
            let v416 = ((v407 + (v174 * v408)) + (v175 * v411)) + (v176 * v414);
            let v426 = ((v417 + (v174 * v418)) + (v175 * v421)) + (v176 * v424);
            let v436 = ((v427 + (v174 * v428)) + (v175 * v431)) + (v176 * v434);
            let v446 = ((v437 + (v174 * v438)) + (v175 * v441)) + (v176 * v444);
            let v456 = ((v447 + (v174 * v448)) + (v175 * v451)) + (v176 * v454);
            let v466 = ((v457 + (v174 * v458)) + (v175 * v461)) + (v176 * v464);
            let v476 = ((v467 + (v174 * v468)) + (v175 * v471)) + (v176 * v474);
            let v486 = ((v477 + (v174 * v478)) + (v175 * v481)) + (v176 * v484);
            let v496 = ((v487 + (v174 * v488)) + (v175 * v491)) + (v176 * v494);
            let v506 = ((v497 + (v174 * v498)) + (v175 * v501)) + (v176 * v504);
            let v516 = ((v507 + (v174 * v508)) + (v175 * v511)) + (v176 * v514);
            let v526 = ((v517 + (v174 * v518)) + (v175 * v521)) + (v176 * v524);
            let v536 = ((v527 + (v174 * v528)) + (v175 * v531)) + (v176 * v534);
            let v546 = ((v537 + (v174 * v538)) + (v175 * v541)) + (v176 * v544);
            let v556 = ((v547 + (v174 * v548)) + (v175 * v551)) + (v176 * v554);
            let v566 = ((v557 + (v174 * v558)) + (v175 * v561)) + (v176 * v564);
            let v576 = ((v567 + (v174 * v568)) + (v175 * v571)) + (v176 * v574);
            let v586 = ((v577 + (v174 * v578)) + (v175 * v581)) + (v176 * v584);
            let v596 = ((v587 + (v174 * v588)) + (v175 * v591)) + (v176 * v594);
            let v606 = ((v597 + (v174 * v598)) + (v175 * v601)) + (v176 * v604);
            let v616 = ((v607 + (v174 * v608)) + (v175 * v611)) + (v176 * v614);
            let v626 = ((v617 + (v174 * v618)) + (v175 * v621)) + (v176 * v624);
            let v636 = ((v627 + (v174 * v628)) + (v175 * v631)) + (v176 * v634);
            let v646 = ((v637 + (v174 * v638)) + (v175 * v641)) + (v176 * v644);
            let v656 = ((v647 + (v174 * v648)) + (v175 * v651)) + (v176 * v654);
            let v666 = ((v657 + (v174 * v658)) + (v175 * v661)) + (v176 * v664);
            let v676 = ((v667 + (v174 * v668)) + (v175 * v671)) + (v176 * v674);
            let v686 = ((v677 + (v174 * v678)) + (v175 * v681)) + (v176 * v684);
            let v696 = ((v687 + (v174 * v688)) + (v175 * v691)) + (v176 * v694);
            let v706 = ((v697 + (v174 * v698)) + (v175 * v701)) + (v176 * v704);
            let v716 = ((v707 + (v174 * v708)) + (v175 * v711)) + (v176 * v714);
            let v726 = ((v717 + (v174 * v718)) + (v175 * v721)) + (v176 * v724);
            let v736 = ((v727 + (v174 * v728)) + (v175 * v731)) + (v176 * v734);
            let v746 = ((v737 + (v174 * v738)) + (v175 * v741)) + (v176 * v744);
            let v756 = ((v747 + (v174 * v748)) + (v175 * v751)) + (v176 * v754);
            let v766 = ((v757 + (v174 * v758)) + (v175 * v761)) + (v176 * v764);
            let v776 = ((v767 + (v174 * v768)) + (v175 * v771)) + (v176 * v774);
            let v786 = ((v777 + (v174 * v778)) + (v175 * v781)) + (v176 * v784);
            let v796 = ((v787 + (v174 * v788)) + (v175 * v791)) + (v176 * v794);
            let v806 = ((v797 + (v174 * v798)) + (v175 * v801)) + (v176 * v804);
            let v816 = ((v807 + (v174 * v808)) + (v175 * v811)) + (v176 * v814);
            let v826 = ((v817 + (v174 * v818)) + (v175 * v821)) + (v176 * v824);
            let v836 = ((v827 + (v174 * v828)) + (v175 * v831)) + (v176 * v834);
            let v846 = ((v837 + (v174 * v838)) + (v175 * v841)) + (v176 * v844);
            let v856 = ((v847 + (v174 * v848)) + (v175 * v851)) + (v176 * v854);
            let v866 = ((v857 + (v174 * v858)) + (v175 * v861)) + (v176 * v864);
            let v876 = ((v867 + (v174 * v868)) + (v175 * v871)) + (v176 * v874);
            let v886 = ((v877 + (v174 * v878)) + (v175 * v881)) + (v176 * v884);
            let v896 = ((v887 + (v174 * v888)) + (v175 * v891)) + (v176 * v894);
            let v906 = ((v897 + (v174 * v898)) + (v175 * v901)) + (v176 * v904);
            let v916 = ((v907 + (v174 * v908)) + (v175 * v911)) + (v176 * v914);
            let v926 = ((v917 + (v174 * v918)) + (v175 * v921)) + (v176 * v924);
            let v936 = ((v927 + (v174 * v928)) + (v175 * v931)) + (v176 * v934);
            let v946 = ((v937 + (v174 * v938)) + (v175 * v941)) + (v176 * v944);
            let v956 = ((v947 + (v174 * v948)) + (v175 * v951)) + (v176 * v954);
            let v966 = ((v957 + (v174 * v958)) + (v175 * v961)) + (v176 * v964);
            let v976 = ((v967 + (v174 * v968)) + (v175 * v971)) + (v176 * v974);
            let v986 = ((v977 + (v174 * v978)) + (v175 * v981)) + (v176 * v984);
            let v996 = ((v987 + (v174 * v988)) + (v175 * v991)) + (v176 * v994);
            let v1006 = ((v997 + (v174 * v998)) + (v175 * v1001)) + (v176 * v1004);
            let v1016 = ((v1007 + (v174 * v1008)) + (v175 * v1011)) + (v176 * v1014);
            let v1026 = ((v1017 + (v174 * v1018)) + (v175 * v1021)) + (v176 * v1024);
            let v1036 = ((v1027 + (v174 * v1028)) + (v175 * v1031)) + (v176 * v1034);
            let v1046 = ((v1037 + (v174 * v1038)) + (v175 * v1041)) + (v176 * v1044);
            let v1056 = ((v1047 + (v174 * v1048)) + (v175 * v1051)) + (v176 * v1054);
            let v1066 = ((v1057 + (v174 * v1058)) + (v175 * v1061)) + (v176 * v1064);
            let v1076 = ((v1067 + (v174 * v1068)) + (v175 * v1071)) + (v176 * v1074);
            let v1086 = ((v1077 + (v174 * v1078)) + (v175 * v1081)) + (v176 * v1084);
            let v1096 = ((v1087 + (v174 * v1088)) + (v175 * v1091)) + (v176 * v1094);
            let v1106 = ((v1097 + (v174 * v1098)) + (v175 * v1101)) + (v176 * v1104);
            let v1116 = ((v1107 + (v174 * v1108)) + (v175 * v1111)) + (v176 * v1114);
            let v1126 = ((v1117 + (v174 * v1118)) + (v175 * v1121)) + (v176 * v1124);
            let v1136 = ((v1127 + (v174 * v1128)) + (v175 * v1131)) + (v176 * v1134);
            let v1146 = ((v1137 + (v174 * v1138)) + (v175 * v1141)) + (v176 * v1144);
            let v1156 = ((v1147 + (v174 * v1148)) + (v175 * v1151)) + (v176 * v1154);
            let v1166 = ((v1157 + (v174 * v1158)) + (v175 * v1161)) + (v176 * v1164);
            let v1176 = ((v1167 + (v174 * v1168)) + (v175 * v1171)) + (v176 * v1174);
            let v1186 = ((v1177 + (v174 * v1178)) + (v175 * v1181)) + (v176 * v1184);
            let v1196 = ((v1187 + (v174 * v1188)) + (v175 * v1191)) + (v176 * v1194);
            let v1206 = ((v1197 + (v174 * v1198)) + (v175 * v1201)) + (v176 * v1204);
            let v1216 = ((v1207 + (v174 * v1208)) + (v175 * v1211)) + (v176 * v1214);
            let v1226 = ((v1217 + (v174 * v1218)) + (v175 * v1221)) + (v176 * v1224);
            let v1236 = ((v1227 + (v174 * v1228)) + (v175 * v1231)) + (v176 * v1234);
            let v1246 = ((v1237 + (v174 * v1238)) + (v175 * v1241)) + (v176 * v1244);
            let v1256 = ((v1247 + (v174 * v1248)) + (v175 * v1251)) + (v176 * v1254);
            let v1266 = ((v1257 + (v174 * v1258)) + (v175 * v1261)) + (v176 * v1264);
            let v1276 = ((v1267 + (v174 * v1268)) + (v175 * v1271)) + (v176 * v1274);
            let v1286 = ((v1277 + (v174 * v1278)) + (v175 * v1281)) + (v176 * v1284);
            let v1296 = ((v1287 + (v174 * v1288)) + (v175 * v1291)) + (v176 * v1294);
            let v1306 = ((v1297 + (v174 * v1298)) + (v175 * v1301)) + (v176 * v1304);
            let v1316 = ((v1307 + (v174 * v1308)) + (v175 * v1311)) + (v176 * v1314);
            let v1326 = ((v1317 + (v174 * v1318)) + (v175 * v1321)) + (v176 * v1324);
            let v1336 = ((v1327 + (v174 * v1328)) + (v175 * v1331)) + (v176 * v1334);
            let v1346 = ((v1337 + (v174 * v1338)) + (v175 * v1341)) + (v176 * v1344);
            let v1356 = ((v1347 + (v174 * v1348)) + (v175 * v1351)) + (v176 * v1354);
            let v1366 = ((v1357 + (v174 * v1358)) + (v175 * v1361)) + (v176 * v1364);
            let v1376 = ((v1367 + (v174 * v1368)) + (v175 * v1371)) + (v176 * v1374);
            let v1386 = ((v1377 + (v174 * v1378)) + (v175 * v1381)) + (v176 * v1384);
            let v1396 = ((v1387 + (v174 * v1388)) + (v175 * v1391)) + (v176 * v1394);
            let v1406 = ((v1397 + (v174 * v1398)) + (v175 * v1401)) + (v176 * v1404);
            let v1416 = ((v1407 + (v174 * v1408)) + (v175 * v1411)) + (v176 * v1414);
            let v1426 = ((v1417 + (v174 * v1418)) + (v175 * v1421)) + (v176 * v1424);
            let v1436 = ((v1427 + (v174 * v1428)) + (v175 * v1431)) + (v176 * v1434);
            let v1446 = ((v1437 + (v174 * v1438)) + (v175 * v1441)) + (v176 * v1444);
            let v1456 = ((v1447 + (v174 * v1448)) + (v175 * v1451)) + (v176 * v1454);
            let v1466 = ((v1457 + (v174 * v1458)) + (v175 * v1461)) + (v176 * v1464);
            let v1476 = ((v1467 + (v174 * v1468)) + (v175 * v1471)) + (v176 * v1474);
            let v1486 = ((v1477 + (v174 * v1478)) + (v175 * v1481)) + (v176 * v1484);
            let v1496 = ((v1487 + (v174 * v1488)) + (v175 * v1491)) + (v176 * v1494);
            let v1506 = ((v1497 + (v174 * v1498)) + (v175 * v1501)) + (v176 * v1504);
            let v1516 = ((v1507 + (v174 * v1508)) + (v175 * v1511)) + (v176 * v1514);
            let v1526 = ((v1517 + (v174 * v1518)) + (v175 * v1521)) + (v176 * v1524);
            let v1536 = ((v1527 + (v174 * v1528)) + (v175 * v1531)) + (v176 * v1534);
            let v1546 = ((v1537 + (v174 * v1538)) + (v175 * v1541)) + (v176 * v1544);
            let v1556 = ((v1547 + (v174 * v1548)) + (v175 * v1551)) + (v176 * v1554);
            let v1566 = ((v1557 + (v174 * v1558)) + (v175 * v1561)) + (v176 * v1564);
            let v1576 = ((v1567 + (v174 * v1568)) + (v175 * v1571)) + (v176 * v1574);
            let v1578 = if v1577 != v0 { 1.0 } else { 0.0 };
            let v1780: f64;
            let v1805: f64;
            let v1847: f64;
            let v1880: f64;
            let v1904: f64;
            let v1912: f64;
            let v1929: f64;
            let v1961: f64;
            let v1986: f64;
            let v1999: f64;
            let v2011: f64;
            let v2030: f64;
            let v2318: f64;
            let v4515: f64;
            if v1578 != 0.0 {
                let v1588 = ((v1579 + (v174 * v1580)) + (v175 * v1583)) + (v176 * v1586);
                let v1598 = ((v1589 + (v174 * v1590)) + (v175 * v1593)) + (v176 * v1596);
                let v1608 = ((v1599 + (v174 * v1600)) + (v175 * v1603)) + (v176 * v1606);
                let v1618 = ((v1609 + (v174 * v1610)) + (v175 * v1613)) + (v176 * v1616);
                let v1628 = ((v1619 + (v174 * v1620)) + (v175 * v1623)) + (v176 * v1626);
                let v1638 = ((v1629 + (v174 * v1630)) + (v175 * v1633)) + (v176 * v1636);
                let v1648 = ((v1639 + (v174 * v1640)) + (v175 * v1643)) + (v176 * v1646);
                let v1658 = ((v1649 + (v174 * v1650)) + (v175 * v1653)) + (v176 * v1656);
                let v1668 = ((v1659 + (v174 * v1660)) + (v175 * v1663)) + (v176 * v1666);
                let v1678 = ((v1669 + (v174 * v1670)) + (v175 * v1673)) + (v176 * v1676);
                let v1688 = ((v1679 + (v174 * v1680)) + (v175 * v1683)) + (v176 * v1686);
                let v1698 = ((v1689 + (v174 * v1690)) + (v175 * v1693)) + (v176 * v1696);
                let v1708 = ((v1699 + (v174 * v1700)) + (v175 * v1703)) + (v176 * v1706);
                let v1718 = ((v1709 + (v174 * v1710)) + (v175 * v1713)) + (v176 * v1716);
                v1780 = v1588;
                v1805 = v1608;
                v1847 = v1618;
                v1880 = v1628;
                v1904 = v1648;
                v1912 = v1598;
                v1929 = v1668;
                v1961 = v1658;
                v1986 = v1678;
                v1999 = v1688;
                v2011 = v1698;
                v2030 = v1708;
                v2318 = v1638;
                v4515 = v1718;
            } else {
                v1780 = v0;
                v1805 = v0;
                v1847 = v0;
                v1880 = v0;
                v1904 = v0;
                v1912 = v0;
                v1929 = v0;
                v1961 = v0;
                v1986 = v0;
                v1999 = v0;
                v2011 = v0;
                v2030 = v0;
                v2318 = v0;
                v4515 = v0;
            }
            let v1748 = v216 * ((v1 + ((v1719 * (if ((v115.powf(v1720)) - (v120.powf(v1720))) >= v0 { ((v115.powf(v1720)) - (v120.powf(v1720))) } else { v0 })) + (v1726 * (if ((v115.powf(v1727)) - (v120.powf(v1727))) >= v0 { ((v115.powf(v1727)) - (v120.powf(v1727))) } else { v0 })))) + ((v1734 * (if ((v116.powf(v1735)) - (v122.powf(v1735))) >= v0 { ((v116.powf(v1735)) - (v122.powf(v1735))) } else { v0 })) + (v1741 * (v123.powf(v1742)))));
            let v1770 = v256 * ((v1 + (v1749 * (if ((v115.powf(v1750)) - (v120.powf(v1750))) >= v0 { ((v115.powf(v1750)) - (v120.powf(v1750))) } else { v0 }))) + ((v1756 * (if ((v116.powf(v1757)) - (v122.powf(v1757))) >= v0 { ((v116.powf(v1757)) - (v122.powf(v1757))) } else { v0 })) + (v1763 * (v123.powf(v1764)))));
            let v1778 = v1 + (v1771 * (if ((v115.powf(v1772)) - (v120.powf(v1772))) >= v0 { ((v115.powf(v1772)) - (v120.powf(v1772))) } else { v0 }));
            let v1779 = v266 * v1778;
            let v2340: f64;
            if v1578 != 0.0 {
                let v1781 = v1780 * v1778;
                v2340 = v1781;
            } else {
                v2340 = v1780;
            }
            let v1790 = v276 * (v1 + (v1782 * (if ((v115.powf(v1783)) - (v120.powf(v1783))) >= v0 { ((v115.powf(v1783)) - (v120.powf(v1783))) } else { v0 })));
            let v1792 = v1791 * v416;
            let v1794 = if v1793 != v1 { 1.0 } else { 0.0 };
            let v2345: f64;
            let v4378: f64;
            if v1794 != 0.0 {
                let v1796 = if v1795 > v0 { 1.0 } else { 0.0 };
                let v2346: f64;
                let v4379: f64;
                if v1796 != 0.0 {
                    let v1803 = v1 - (v1797 * (if ((v115.powf(v1795)) - (v120.powf(v1795))) >= v0 { ((v115.powf(v1795)) - (v120.powf(v1795))) } else { v0 }));
                    let v1804 = v1792 * v1803;
                    let v4380: f64;
                    if v1578 != 0.0 {
                        let v1806 = v1805 * v1803;
                        v4380 = v1806;
                    } else {
                        v4380 = v1805;
                    }
                    v2346 = v1804;
                    v4379 = v4380;
                } else {
                    let v1807 = v1 - v1797;
                    let v1808 = v1792 * v1807;
                    let v4381: f64;
                    if v1578 != 0.0 {
                        let v1809 = v1805 * v1807;
                        v4381 = v1809;
                    } else {
                        v4381 = v1805;
                    }
                    v2346 = v1808;
                    v4379 = v4381;
                }
                v2345 = v2346;
                v4378 = v4379;
            } else {
                let v1811 = -v65;
                let v1822 = (v1 - (v1810 * (rspice_limited_exp((v1811 / v1812))))) - (v1817 * (rspice_limited_exp((v1811 / v1818))));
                let v1823 = v1792 * v1822;
                let v4382: f64;
                if v1578 != 0.0 {
                    let v1824 = v1805 * v1822;
                    v4382 = v1824;
                } else {
                    v4382 = v1805;
                }
                v2345 = v1823;
                v4378 = v4382;
            }
            let v1845 = (v1 + (v1825 * (if ((v115.powf(v1826)) - (v120.powf(v1826))) >= v0 { ((v115.powf(v1826)) - (v120.powf(v1826))) } else { v0 }))) + ((v1832 * (if ((v116.powf(v1833)) - (v122.powf(v1833))) >= v0 { ((v116.powf(v1833)) - (v122.powf(v1833))) } else { v0 })) + (v1839 * (v123.powf(v1840))));
            let v1846 = v426 * v1845;
            let v4394: f64;
            if v1578 != 0.0 {
                let v1848 = v1847 * v1845;
                v4394 = v1848;
            } else {
                v4394 = v1847;
            }
            let v1870 = v446 * ((v1 + (v1849 * (if ((v115.powf(v1850)) - (v120.powf(v1850))) >= v0 { ((v115.powf(v1850)) - (v120.powf(v1850))) } else { v0 }))) + ((v1856 * (if ((v116.powf(v1857)) - (v122.powf(v1857))) >= v0 { ((v116.powf(v1857)) - (v122.powf(v1857))) } else { v0 })) + (v1863 * (v123.powf(v1864)))));
            let v1878 = v1 + (v1871 * (if ((v115.powf(v1872)) - (v120.powf(v1872))) >= v0 { ((v115.powf(v1872)) - (v120.powf(v1872))) } else { v0 }));
            let v1879 = v436 * v1878;
            let v4410: f64;
            if v1578 != 0.0 {
                let v1881 = v1880 * v1878;
                v4410 = v1881;
            } else {
                v4410 = v1880;
            }
            let v1902 = (v1 + (v1882 * (if ((v115.powf(v1883)) - (v120.powf(v1883))) >= v0 { ((v115.powf(v1883)) - (v120.powf(v1883))) } else { v0 }))) + ((v1889 * (if ((v116.powf(v1890)) - (v122.powf(v1890))) >= v0 { ((v116.powf(v1890)) - (v122.powf(v1890))) } else { v0 })) + (v1896 * (v123.powf(v1897))));
            let v1903 = v466 * v1902;
            let v4407: f64;
            if v1578 != 0.0 {
                let v1905 = v1904 * v1902;
                v4407 = v1905;
            } else {
                v4407 = v1904;
            }
            let v1910 = if ((v115.powf(v1906)) - (v120.powf(v1906))) >= v0 { ((v115.powf(v1906)) - (v120.powf(v1906))) } else { v0 };
            let v1911 = v386 * v1910;
            let v4315: f64;
            if v1578 != 0.0 {
                let v1913 = v1912 * v1910;
                v4315 = v1913;
            } else {
                v4315 = v1912;
            }
            let v1919 = v396 * (if ((v115.powf(v1914)) - (v120.powf(v1914))) >= v0 { ((v115.powf(v1914)) - (v120.powf(v1914))) } else { v0 });
            let v1927 = v1 + (v1920 * (if ((v115.powf(v1921)) - (v120.powf(v1921))) >= v0 { ((v115.powf(v1921)) - (v120.powf(v1921))) } else { v0 }));
            let v1928 = v596 * v1927;
            let v5521: f64;
            if v1578 != 0.0 {
                let v1930 = v1929 * v1927;
                v5521 = v1930;
            } else {
                v5521 = v1929;
            }
            let v1941 = if (v406 * (v1 + (v1931 * (if ((v115.powf(v1932)) - (v120.powf(v1932))) >= v0 { ((v115.powf(v1932)) - (v120.powf(v1932))) } else { v0 })))) <= v1940 { (v406 * (v1 + (v1931 * (if ((v115.powf(v1932)) - (v120.powf(v1932))) >= v0 { ((v115.powf(v1932)) - (v120.powf(v1932))) } else { v0 })))) } else { v1940 };
            let v1950 = v656 * (v1 + (v1942 * (if ((v115.powf(v1943)) - (v120.powf(v1943))) >= v0 { ((v115.powf(v1943)) - (v120.powf(v1943))) } else { v0 })));
            let v1958 = v1 + (v1951 * (if ((v115.powf(v1952)) - (v120.powf(v1952))) >= v0 { ((v115.powf(v1952)) - (v120.powf(v1952))) } else { v0 }));
            let v1960 = if (v476 * v1958) >= v0 { (v476 * v1958) } else { v0 };
            let v5525: f64;
            if v1578 != 0.0 {
                let v1963 = if (v1961 * v1958) >= v0 { (v1961 * v1958) } else { v0 };
                v5525 = v1963;
            } else {
                v5525 = v1961;
            }
            let v1984 = (v1 + (v1964 * (if ((v115.powf(v1965)) - (v120.powf(v1965))) >= v0 { ((v115.powf(v1965)) - (v120.powf(v1965))) } else { v0 }))) + ((v1971 * (if ((v116.powf(v1972)) - (v122.powf(v1972))) >= v0 { ((v116.powf(v1972)) - (v122.powf(v1972))) } else { v0 })) + (v1978 * (v123.powf(v1979))));
            let v1985 = v676 * v1984;
            let v4431: f64;
            if v1578 != 0.0 {
                let v1987 = v1986 * v1984;
                v4431 = v1987;
            } else {
                v4431 = v1986;
            }
            let v1995 = v1 + (v1988 * (if ((v115.powf(v1989)) - (v120.powf(v1989))) >= v0 { ((v115.powf(v1989)) - (v120.powf(v1989))) } else { v0 }));
            let v1998 = if (v686 * v1995) >= v1997 { (v686 * v1995) } else { v1997 };
            let v5529: f64;
            if v1578 != 0.0 {
                let v2001 = if (v1999 * v1995) >= v1997 { (v1999 * v1995) } else { v1997 };
                v5529 = v2001;
            } else {
                v5529 = v1999;
            }
            let v2009 = v1 + (v2002 * (if ((v115.powf(v2003)) - (v120.powf(v2003))) >= v0 { ((v115.powf(v2003)) - (v120.powf(v2003))) } else { v0 }));
            let v2010 = v586 * v2009;
            let v4480: f64;
            if v1578 != 0.0 {
                let v2012 = v2011 * v2009;
                v4480 = v2012;
            } else {
                v4480 = v2011;
            }
            let v2028 = (v1 + (v2013 * (if ((v115.powf(v2014)) - (v120.powf(v2014))) >= v0 { ((v115.powf(v2014)) - (v120.powf(v2014))) } else { v0 }))) + (v2020 * (if ((v116.powf(v2021)) - (v122.powf(v2021))) >= v0 { ((v116.powf(v2021)) - (v122.powf(v2021))) } else { v0 }));
            let v2029 = v726 * v2028;
            let v5563: f64;
            if v1578 != 0.0 {
                let v2031 = v2030 * v2028;
                v5563 = v2031;
            } else {
                v5563 = v2030;
            }
            let v2040 = v736 * (v1 + (v2032 * (if ((v116.powf(v2033)) - (v122.powf(v2033))) >= v0 { ((v116.powf(v2033)) - (v122.powf(v2033))) } else { v0 })));
            let v2050 = v2048 * (v1 + (v2041 * (if ((v116.powf(v2042)) - (v122.powf(v2042))) >= v0 { ((v116.powf(v2042)) - (v122.powf(v2042))) } else { v0 })));
            let v2060 = v2058 * (v1 + (v2051 * (if ((v116.powf(v2052)) - (v122.powf(v2052))) >= v0 { ((v116.powf(v2052)) - (v122.powf(v2052))) } else { v0 })));
            let v2067 = v776 * ((v1 + (v2061 * v115)) + (v2064 * v116));
            let v2074 = v816 * ((v1 + (v2068 * v115)) + (v2071 * v116));
            let v2081 = v1066 * ((v1 + (v2075 * v115)) + (v2078 * v116));
            let v2088 = v1096 * ((v1 + (v2082 * v115)) + (v2085 * v116));
            let v2095 = v1126 * ((v1 + (v2089 * v115)) + (v2092 * v116));
            let v2100 = v2096 * (v1 + (v2097 * v115));
            let v2131 = v226 * ((v1 + ((v2101 * (if ((v117.powf(v2102)) - (v120.powf(v2102))) >= v0 { ((v117.powf(v2102)) - (v120.powf(v2102))) } else { v0 })) + (v2108 * (if ((v117.powf(v2109)) - (v120.powf(v2109))) >= v0 { ((v117.powf(v2109)) - (v120.powf(v2109))) } else { v0 })))) + ((v2116 * (if ((v118.powf(v2117)) - (v122.powf(v2117))) >= v0 { ((v118.powf(v2117)) - (v122.powf(v2117))) } else { v0 })) + (v2123 * ((v118 * v117).powf(v2125)))));
            let v2153 = v186 * ((v1 + (v2132 * (if ((v117.powf(v2133)) - (v120.powf(v2133))) >= v0 { ((v117.powf(v2133)) - (v120.powf(v2133))) } else { v0 }))) + ((v2139 * (if ((v118.powf(v2140)) - (v122.powf(v2140))) >= v0 { ((v118.powf(v2140)) - (v122.powf(v2140))) } else { v0 })) + (v2146 * (v123.powf(v2147)))));
            let v2175 = v196 * ((v1 + (v2154 * (if ((v117.powf(v2155)) - (v120.powf(v2155))) >= v0 { ((v117.powf(v2155)) - (v120.powf(v2155))) } else { v0 }))) + ((v2161 * (if ((v118.powf(v2162)) - (v122.powf(v2162))) >= v0 { ((v118.powf(v2162)) - (v122.powf(v2162))) } else { v0 })) + (v2168 * (v123.powf(v2169)))));
            let v2197 = v696 * ((v1 + (v2176 * (if ((v117.powf(v2177)) - (v120.powf(v2177))) >= v0 { ((v117.powf(v2177)) - (v120.powf(v2177))) } else { v0 }))) + ((v2183 * (if ((v116.powf(v2184)) - (v122.powf(v2184))) >= v0 { ((v116.powf(v2184)) - (v122.powf(v2184))) } else { v0 })) + (v2190 * (v123.powf(v2191)))));
            let v2207 = if (v486 * (v1 + (v2198 * (if ((v117.powf(v2199)) - (v120.powf(v2199))) >= v0 { ((v117.powf(v2199)) - (v120.powf(v2199))) } else { v0 })))) >= v0 { (v486 * (v1 + (v2198 * (if ((v117.powf(v2199)) - (v120.powf(v2199))) >= v0 { ((v117.powf(v2199)) - (v120.powf(v2199))) } else { v0 })))) } else { v0 };
            let v2229 = v356 * ((v1 + (v2208 * (if ((v115.powf(v2209)) - (v120.powf(v2209))) >= v0 { ((v115.powf(v2209)) - (v120.powf(v2209))) } else { v0 }))) + ((v2215 * (if ((v116.powf(v2216)) - (v122.powf(v2216))) >= v0 { ((v116.powf(v2216)) - (v122.powf(v2216))) } else { v0 })) + (v2222 * (v123.powf(v2223)))));
            let v2251 = v346 * ((v1 + (v2230 * (if ((v115.powf(v2231)) - (v120.powf(v2231))) >= v0 { ((v115.powf(v2231)) - (v120.powf(v2231))) } else { v0 }))) + ((v2237 * (if ((v116.powf(v2238)) - (v122.powf(v2238))) >= v0 { ((v116.powf(v2238)) - (v122.powf(v2238))) } else { v0 })) + (v2244 * (v123.powf(v2245)))));
            let v2260 = v526 * (v1 + (v2252 * (if ((v115.powf(v2253)) - (v120.powf(v2253))) >= v0 { ((v115.powf(v2253)) - (v120.powf(v2253))) } else { v0 })));
            let v2264 = v856 * (v1 + (v115 * v2261));
            let v2268 = v866 * (v1 + (v115 * v2265));
            let v2272 = v886 * (v1 + (v115 * v2269));
            let v2276 = v926 * (v1 + (v115 * v2273));
            let v2280 = v936 * (v1 + (v115 * v2277));
            if v2281 != 0.0 {
                let v2286 = if (if v2282 == v0 { 1.0 } else { 0.0 }) != 0.0 || (if v2284 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v2286 != 0.0 {
                } else {
                }
            } else {
            }
            let v2288 = if v2287 == v1 { 1.0 } else { 0.0 };
            let v3971: f64;
            let v3973: f64;
            let v3976: f64;
            if v2288 != 0.0 {
                let v2297 = v496 * (v1 + (v2289 * (if ((v115.powf(v2290)) - (v120.powf(v2290))) >= v0 { ((v115.powf(v2290)) - (v120.powf(v2290))) } else { v0 })));
                let v2306 = v506 * (v1 + (v2298 * (if ((v115.powf(v2299)) - (v120.powf(v2299))) >= v0 { ((v115.powf(v2299)) - (v120.powf(v2299))) } else { v0 })));
                v3971 = v2297;
                v3973 = v2306;
                v3976 = v566;
            } else {
                let v2315 = v566 * (v1 + (v2307 * (if ((v115.powf(v2308)) - (v120.powf(v2308))) >= v0 { ((v115.powf(v2308)) - (v120.powf(v2308))) } else { v0 })));
                v3971 = v496;
                v3973 = v506;
                v3976 = v2315;
            }
            let v2316 = if v456 < v1 { 1.0 } else { 0.0 };
            let v2352: f64;
            if v2316 != 0.0 {
                v2352 = v1;
            } else {
                let v2317 = if v456 > v63 { 1.0 } else { 0.0 };
                let v2353: f64;
                if v2317 != 0.0 {
                    v2353 = v63;
                } else {
                    v2353 = v456;
                }
                v2352 = v2353;
            }
            let v4412: f64;
            if v1578 != 0.0 {
                let v2319 = if v2318 < v1 { 1.0 } else { 0.0 };
                let v4413: f64;
                if v2319 != 0.0 {
                    v4413 = v1;
                } else {
                    let v2320 = if v2318 > v63 { 1.0 } else { 0.0 };
                    let v4414: f64;
                    if v2320 != 0.0 {
                        v4414 = v63;
                    } else {
                        v4414 = v2318;
                    }
                    v4413 = v4414;
                }
                v4412 = v4413;
            } else {
                v4412 = v2318;
            }
            let v2321 = if v796 < v0 { 1.0 } else { 0.0 };
            if v2321 != 0.0 {
            } else {
            }
            let v2322 = if v836 < v0 { 1.0 } else { 0.0 };
            if v2322 != 0.0 {
            } else {
            }
            let v2323 = if v716 <= v0 { 1.0 } else { 0.0 };
            if v2323 != 0.0 {
            } else {
            }
            let v2324 = if v706 <= v0 { 1.0 } else { 0.0 };
            if v2324 != 0.0 {
            } else {
            }
            let v2325 = if v636 < v0 { 1.0 } else { 0.0 };
            if v2325 != 0.0 {
            } else {
            }
            let v2326 = if v246 < v0 { 1.0 } else { 0.0 };
            if v2326 != 0.0 {
            } else {
            }
            let v2327 = if v1770 < v0 { 1.0 } else { 0.0 };
            if v2327 != 0.0 {
            } else {
            }
            let v2328 = if v2229 < v0 { 1.0 } else { 0.0 };
            if v2328 != 0.0 {
            } else {
            }
            let v2329 = if v206 <= v0 { 1.0 } else { 0.0 };
            if v2329 != 0.0 {
            } else {
            }
            let v2330 = if v1748 <= v0 { 1.0 } else { 0.0 };
            if v2330 != 0.0 {
            } else {
            }
            let v2331 = if v2131 <= v0 { 1.0 } else { 0.0 };
            if v2331 != 0.0 {
            } else {
            }
            let v2333 = if v2332 != v0 { 1.0 } else { 0.0 };
            if v2333 != 0.0 {
                let v2334 = if v1016 <= v0 { 1.0 } else { 0.0 };
                if v2334 != 0.0 {
                } else {
                }
                let v2335 = if v1056 <= v0 { 1.0 } else { 0.0 };
                if v2335 != 0.0 {
                } else {
                }
            } else {
            }
            let v2337 = if v2336 != v0 { 1.0 } else { 0.0 };
            if v2337 != 0.0 {
                let v2338 = if v1156 <= v0 { 1.0 } else { 0.0 };
                if v2338 != 0.0 {
                } else {
                }
            } else {
            }
            let v2339 = if v1779 < v0 { 1.0 } else { 0.0 };
            if v2339 != 0.0 {
            } else {
            }
            if v1578 != 0.0 {
                let v2341 = if v2340 < v0 { 1.0 } else { 0.0 };
                if v2341 != 0.0 {
                } else {
                }
            } else {
            }
            let v2342 = if v1166 < v0 { 1.0 } else { 0.0 };
            let v8036: f64;
            if v2342 != 0.0 {
                v8036 = v0;
            } else {
                v8036 = v1166;
            }
            let v2343 = if v1176 < v0 { 1.0 } else { 0.0 };
            let v8069: f64;
            if v2343 != 0.0 {
                v8069 = v0;
            } else {
                v8069 = v1176;
            }
            let v2344 = if v1276 < v0 { 1.0 } else { 0.0 };
            let v4576: f64;
            if v2344 != 0.0 {
                v4576 = v0;
            } else {
                v4576 = v1276;
            }
            let v2347 = if v2345 <= v0 { 1.0 } else { 0.0 };
            let v4323: f64;
            if v2347 != 0.0 {
                v4323 = v2348;
            } else {
                v4323 = v2345;
            }
            let v2349 = if v1846 < v0 { 1.0 } else { 0.0 };
            let v4339: f64;
            if v2349 != 0.0 {
                v4339 = v0;
            } else {
                v4339 = v1846;
            }
            let v2350 = if v1870 < v0 { 1.0 } else { 0.0 };
            let v4375: f64;
            if v2350 != 0.0 {
                v4375 = v0;
            } else {
                v4375 = v1870;
            }
            let v2351 = if v1879 < v0 { 1.0 } else { 0.0 };
            let v4357: f64;
            if v2351 != 0.0 {
                v4357 = v0;
            } else {
                v4357 = v1879;
            }
            let v2354 = if v2352 < v0 { 1.0 } else { 0.0 };
            let v4360: f64;
            if v2354 != 0.0 {
                v4360 = v0;
            } else {
                v4360 = v2352;
            }
            let v2355 = if v2050 < v0 { 1.0 } else { 0.0 };
            let v7691: f64;
            if v2355 != 0.0 {
                v7691 = v0;
            } else {
                v7691 = v2050;
            }
            let v2357 = if v2356 == v1 { 1.0 } else { 0.0 };
            let v8481: f64;
            let v8538: f64;
            if v2357 != 0.0 {
                let v2359 = if v65 > v2358 { 1.0 } else { 0.0 };
                let v2362: f64;
                let v8482: f64;
                if v2359 != 0.0 {
                    let v2360 = v65 - v2358;
                    v2362 = v2360;
                    v8482 = v2358;
                } else {
                    v2362 = v65;
                    v8482 = v65;
                }
                let v2364 = if v2361 >= (v2362 / v63) { 1.0 } else { 0.0 };
                let v8539: f64;
                if v2364 != 0.0 {
                    v8539 = v0;
                } else {
                    v8539 = v2361;
                }
                v8481 = v8482;
                v8538 = v8539;
            } else {
                v8481 = v8483;
                v8538 = v8540;
            }
            let v2367 = v2365 - v2366;
            let v2370 = v2369 - v2366;
            let v3213: f64;
            let v3263: f64;
            let v3802: f64;
            let v3950: f64;
            let v3960: f64;
            let v4969: f64;
            let v4986: f64;
            if v2371 != 0.0 {
                let v2374 = v2372 * v2373;
                v3213 = v0;
                v3263 = v0;
                v3802 = v0;
                v3950 = v0;
                v3960 = v2374;
                v4969 = v0;
                v4986 = v0;
            } else {
                let v2378 = if (if v2375 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v2372 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v3214: f64;
                let v3264: f64;
                let v3803: f64;
                let v3951: f64;
                let v3961: f64;
                let v4970: f64;
                let v4987: f64;
                if v2378 != 0.0 {
                    let v2381 = if v2379 < v2380 { 1.0 } else { 0.0 };
                    let v2420: f64;
                    let v2473: f64;
                    let v3010: f64;
                    let v4971: f64;
                    let v4988: f64;
                    if v2381 != 0.0 {
                        let v2383 = if (v24 % v63) != v0 { 1.0 } else { 0.0 };
                        let v2399: f64;
                        let v2405: f64;
                        let v2421: f64;
                        let v2474: f64;
                        if v2383 != 0.0 {
                            let v2387 = v63 * (if ((v24 - v1) / v63) >= v0 { ((v24 - v1) / v63) } else { v0 });
                            v2399 = v2387;
                            v2405 = v2387;
                            v2421 = v1;
                            v2474 = v1;
                        } else {
                            let v2389 = if v2388 == v1 { 1.0 } else { 0.0 };
                            let v2400: f64;
                            let v2406: f64;
                            let v2422: f64;
                            let v2475: f64;
                            if v2389 != 0.0 {
                                let v2393 = v63 * (if ((v24 / v63) - v1) >= v0 { ((v24 / v63) - v1) } else { v0 });
                                v2400 = v24;
                                v2406 = v2393;
                                v2422 = v0;
                                v2475 = v63;
                            } else {
                                let v2397 = v63 * (if ((v24 / v63) - v1) >= v0 { ((v24 / v63) - v1) } else { v0 });
                                v2400 = v2397;
                                v2406 = v24;
                                v2422 = v63;
                                v2475 = v0;
                            }
                            v2399 = v2400;
                            v2405 = v2406;
                            v2421 = v2422;
                            v2474 = v2475;
                        }
                        let v3011: f64;
                        if v2398 != 0.0 {
                            let v2401 = if v2399 == v0 { 1.0 } else { 0.0 };
                            let v3012: f64;
                            if v2401 != 0.0 {
                                v3012 = v0;
                            } else {
                                let v2404 = (v2372 * v2367) / (v70 * v2399);
                                v3012 = v2404;
                            }
                            v3011 = v3012;
                        } else {
                            let v2407 = if v2405 == v0 { 1.0 } else { 0.0 };
                            let v3013: f64;
                            if v2407 != 0.0 {
                                v3013 = v0;
                            } else {
                                let v2410 = (v2372 * v2367) / (v70 * v2405);
                                v3013 = v2410;
                            }
                            v3011 = v3013;
                        }
                        v2420 = v2421;
                        v2473 = v2474;
                        v3010 = v3011;
                        v4971 = v2399;
                        v4988 = v2405;
                    } else {
                        v2420 = v0;
                        v2473 = v0;
                        v3010 = v0;
                        v4971 = v0;
                        v4988 = v0;
                    }
                    let v2411 = if v2379 == v0 { 1.0 } else { 0.0 };
                    let v3009: f64;
                    let v3029: f64;
                    if v2411 != 0.0 {
                        let v3030: f64;
                        if v2412 != 0.0 {
                            let v3031: f64;
                            if v2413 != 0.0 {
                                let v2419 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3032: f64;
                                if v2419 != 0.0 {
                                    let v2423 = if v2420 == v0 { 1.0 } else { 0.0 };
                                    let v3033: f64;
                                    if v2423 != 0.0 {
                                        v3033 = v0;
                                    } else {
                                        let v2426 = (v2372 * v2367) / (v70 * v2420);
                                        v3033 = v2426;
                                    }
                                    v3032 = v3033;
                                } else {
                                    let v2434 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3034: f64;
                                    if v2434 != 0.0 {
                                        let v2435 = v2367 + v2368;
                                        let v2436 = if v2435 == v0 { 1.0 } else { 0.0 };
                                        if v2436 != 0.0 {
                                        } else {
                                        }
                                        let v2438 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2436 != 0.0 { 1.0 } else { 0.0 };
                                        let v3035: f64;
                                        if v2438 != 0.0 {
                                            v3035 = v0;
                                        } else {
                                            let v2442 = (v2372 * v70) / ((v2427 * v2420) * v2435);
                                            v3035 = v2442;
                                        }
                                        v3034 = v3035;
                                    } else {
                                        v3034 = v0;
                                    }
                                    v3032 = v3034;
                                }
                                v3031 = v3032;
                            } else {
                                let v2448 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3036: f64;
                                if v2448 != 0.0 {
                                    let v2449 = if v2420 == v0 { 1.0 } else { 0.0 };
                                    let v3037: f64;
                                    if v2449 != 0.0 {
                                        v3037 = v0;
                                    } else {
                                        let v2452 = (v2372 * v2367) / (v70 * v2420);
                                        v3037 = v2452;
                                    }
                                    v3036 = v3037;
                                } else {
                                    let v2458 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3038: f64;
                                    if v2458 != 0.0 {
                                        let v2459 = v2367 + v2368;
                                        let v2460 = if v2459 == v0 { 1.0 } else { 0.0 };
                                        if v2460 != 0.0 {
                                        } else {
                                        }
                                        let v2462 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2460 != 0.0 { 1.0 } else { 0.0 };
                                        let v3039: f64;
                                        if v2462 != 0.0 {
                                            v3039 = v0;
                                        } else {
                                            let v2466 = (v2372 * v70) / ((v2427 * v2420) * v2459);
                                            v3039 = v2466;
                                        }
                                        v3038 = v3039;
                                    } else {
                                        v3038 = v0;
                                    }
                                    v3036 = v3038;
                                }
                                v3031 = v3036;
                            }
                            v3030 = v3031;
                        } else {
                            let v3040: f64;
                            if v2467 != 0.0 {
                                let v2472 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3041: f64;
                                if v2472 != 0.0 {
                                    let v2476 = if v2473 == v0 { 1.0 } else { 0.0 };
                                    let v3042: f64;
                                    if v2476 != 0.0 {
                                        v3042 = v0;
                                    } else {
                                        let v2479 = (v2372 * v2367) / (v70 * v2473);
                                        v3042 = v2479;
                                    }
                                    v3041 = v3042;
                                } else {
                                    let v2484 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3043: f64;
                                    if v2484 != 0.0 {
                                        let v2485 = v2367 + v2368;
                                        let v2486 = if v2485 == v0 { 1.0 } else { 0.0 };
                                        if v2486 != 0.0 {
                                        } else {
                                        }
                                        let v2488 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2486 != 0.0 { 1.0 } else { 0.0 };
                                        let v3044: f64;
                                        if v2488 != 0.0 {
                                            v3044 = v0;
                                        } else {
                                            let v2492 = (v2372 * v70) / ((v2427 * v2473) * v2485);
                                            v3044 = v2492;
                                        }
                                        v3043 = v3044;
                                    } else {
                                        v3043 = v0;
                                    }
                                    v3041 = v3043;
                                }
                                v3040 = v3041;
                            } else {
                                let v2497 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3045: f64;
                                if v2497 != 0.0 {
                                    let v2498 = if v2473 == v0 { 1.0 } else { 0.0 };
                                    let v3046: f64;
                                    if v2498 != 0.0 {
                                        v3046 = v0;
                                    } else {
                                        let v2501 = (v2372 * v2367) / (v70 * v2473);
                                        v3046 = v2501;
                                    }
                                    v3045 = v3046;
                                } else {
                                    let v2506 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3047: f64;
                                    if v2506 != 0.0 {
                                        let v2507 = v2367 + v2368;
                                        let v2508 = if v2507 == v0 { 1.0 } else { 0.0 };
                                        if v2508 != 0.0 {
                                        } else {
                                        }
                                        let v2510 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2508 != 0.0 { 1.0 } else { 0.0 };
                                        let v3048: f64;
                                        if v2510 != 0.0 {
                                            v3048 = v0;
                                        } else {
                                            let v2514 = (v2372 * v70) / ((v2427 * v2473) * v2507);
                                            v3048 = v2514;
                                        }
                                        v3047 = v3048;
                                    } else {
                                        v3047 = v0;
                                    }
                                    v3045 = v3047;
                                }
                                v3040 = v3045;
                            }
                            v3030 = v3040;
                        }
                        v3009 = v3010;
                        v3029 = v3030;
                    } else {
                        let v2515 = if v2379 == v1 { 1.0 } else { 0.0 };
                        let v3014: f64;
                        let v3049: f64;
                        if v2515 != 0.0 {
                            let v3050: f64;
                            if v2516 != 0.0 {
                                let v3051: f64;
                                if v2517 != 0.0 {
                                    let v2522 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3052: f64;
                                    if v2522 != 0.0 {
                                        let v2523 = if v2420 == v0 { 1.0 } else { 0.0 };
                                        let v3053: f64;
                                        if v2523 != 0.0 {
                                            v3053 = v0;
                                        } else {
                                            let v2526 = (v2372 * v2367) / (v70 * v2420);
                                            v3053 = v2526;
                                        }
                                        v3052 = v3053;
                                    } else {
                                        let v2531 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3054: f64;
                                        if v2531 != 0.0 {
                                            let v2532 = v2367 + v2368;
                                            let v2533 = if v2532 == v0 { 1.0 } else { 0.0 };
                                            if v2533 != 0.0 {
                                            } else {
                                            }
                                            let v2535 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2533 != 0.0 { 1.0 } else { 0.0 };
                                            let v3055: f64;
                                            if v2535 != 0.0 {
                                                v3055 = v0;
                                            } else {
                                                let v2539 = (v2372 * v70) / ((v2427 * v2420) * v2532);
                                                v3055 = v2539;
                                            }
                                            v3054 = v3055;
                                        } else {
                                            v3054 = v0;
                                        }
                                        v3052 = v3054;
                                    }
                                    v3051 = v3052;
                                } else {
                                    let v2544 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3056: f64;
                                    if v2544 != 0.0 {
                                        let v2545 = if v2420 == v0 { 1.0 } else { 0.0 };
                                        let v3057: f64;
                                        if v2545 != 0.0 {
                                            v3057 = v0;
                                        } else {
                                            let v2548 = (v2372 * v2367) / (v70 * v2420);
                                            v3057 = v2548;
                                        }
                                        v3056 = v3057;
                                    } else {
                                        let v2553 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3058: f64;
                                        if v2553 != 0.0 {
                                            let v2554 = v2367 + v2368;
                                            let v2555 = if v2554 == v0 { 1.0 } else { 0.0 };
                                            if v2555 != 0.0 {
                                            } else {
                                            }
                                            let v2557 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2555 != 0.0 { 1.0 } else { 0.0 };
                                            let v3059: f64;
                                            if v2557 != 0.0 {
                                                v3059 = v0;
                                            } else {
                                                let v2561 = (v2372 * v70) / ((v2427 * v2420) * v2554);
                                                v3059 = v2561;
                                            }
                                            v3058 = v3059;
                                        } else {
                                            v3058 = v0;
                                        }
                                        v3056 = v3058;
                                    }
                                    v3051 = v3056;
                                }
                                v3050 = v3051;
                            } else {
                                let v3060: f64;
                                if v2562 != 0.0 {
                                    let v2567 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3061: f64;
                                    if v2567 != 0.0 {
                                        let v2568 = if v2473 == v0 { 1.0 } else { 0.0 };
                                        let v3062: f64;
                                        if v2568 != 0.0 {
                                            v3062 = v0;
                                        } else {
                                            let v2571 = (v2372 * v2367) / (v70 * v2473);
                                            v3062 = v2571;
                                        }
                                        v3061 = v3062;
                                    } else {
                                        let v2576 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3063: f64;
                                        if v2576 != 0.0 {
                                            let v2577 = if v2367 == v0 { 1.0 } else { 0.0 };
                                            if v2577 != 0.0 {
                                            } else {
                                            }
                                            let v2579 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2577 != 0.0 { 1.0 } else { 0.0 };
                                            let v3064: f64;
                                            if v2579 != 0.0 {
                                                v3064 = v0;
                                            } else {
                                                let v2583 = (v2372 * v70) / ((v2432 * v2473) * v2367);
                                                v3064 = v2583;
                                            }
                                            v3063 = v3064;
                                        } else {
                                            v3063 = v0;
                                        }
                                        v3061 = v3063;
                                    }
                                    v3060 = v3061;
                                } else {
                                    let v2588 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3065: f64;
                                    if v2588 != 0.0 {
                                        let v2589 = if v2473 == v0 { 1.0 } else { 0.0 };
                                        let v3066: f64;
                                        if v2589 != 0.0 {
                                            v3066 = v0;
                                        } else {
                                            let v2592 = (v2372 * v2367) / (v70 * v2473);
                                            v3066 = v2592;
                                        }
                                        v3065 = v3066;
                                    } else {
                                        let v2597 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3067: f64;
                                        if v2597 != 0.0 {
                                            let v2598 = if v2367 == v0 { 1.0 } else { 0.0 };
                                            if v2598 != 0.0 {
                                            } else {
                                            }
                                            let v2600 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2598 != 0.0 { 1.0 } else { 0.0 };
                                            let v3068: f64;
                                            if v2600 != 0.0 {
                                                v3068 = v0;
                                            } else {
                                                let v2604 = (v2372 * v70) / ((v2432 * v2473) * v2367);
                                                v3068 = v2604;
                                            }
                                            v3067 = v3068;
                                        } else {
                                            v3067 = v0;
                                        }
                                        v3065 = v3067;
                                    }
                                    v3060 = v3065;
                                }
                                v3050 = v3060;
                            }
                            v3014 = v3010;
                            v3049 = v3050;
                        } else {
                            let v2605 = if v2379 == v63 { 1.0 } else { 0.0 };
                            let v3015: f64;
                            let v3069: f64;
                            if v2605 != 0.0 {
                                let v3070: f64;
                                if v2606 != 0.0 {
                                    let v3071: f64;
                                    if v2607 != 0.0 {
                                        let v2612 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3072: f64;
                                        if v2612 != 0.0 {
                                            let v2613 = if v2420 == v0 { 1.0 } else { 0.0 };
                                            let v3073: f64;
                                            if v2613 != 0.0 {
                                                v3073 = v0;
                                            } else {
                                                let v2616 = (v2372 * v2367) / (v70 * v2420);
                                                v3073 = v2616;
                                            }
                                            v3072 = v3073;
                                        } else {
                                            let v2621 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3074: f64;
                                            if v2621 != 0.0 {
                                                let v2622 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                if v2622 != 0.0 {
                                                } else {
                                                }
                                                let v2624 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2622 != 0.0 { 1.0 } else { 0.0 };
                                                let v3075: f64;
                                                if v2624 != 0.0 {
                                                    v3075 = v0;
                                                } else {
                                                    let v2628 = (v2372 * v70) / ((v2432 * v2420) * v2367);
                                                    v3075 = v2628;
                                                }
                                                v3074 = v3075;
                                            } else {
                                                v3074 = v0;
                                            }
                                            v3072 = v3074;
                                        }
                                        v3071 = v3072;
                                    } else {
                                        let v2633 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3076: f64;
                                        if v2633 != 0.0 {
                                            let v2634 = if v2420 == v0 { 1.0 } else { 0.0 };
                                            let v3077: f64;
                                            if v2634 != 0.0 {
                                                v3077 = v0;
                                            } else {
                                                let v2637 = (v2372 * v2367) / (v70 * v2420);
                                                v3077 = v2637;
                                            }
                                            v3076 = v3077;
                                        } else {
                                            let v2642 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3078: f64;
                                            if v2642 != 0.0 {
                                                let v2643 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                if v2643 != 0.0 {
                                                } else {
                                                }
                                                let v2645 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2643 != 0.0 { 1.0 } else { 0.0 };
                                                let v3079: f64;
                                                if v2645 != 0.0 {
                                                    v3079 = v0;
                                                } else {
                                                    let v2649 = (v2372 * v70) / ((v2432 * v2420) * v2367);
                                                    v3079 = v2649;
                                                }
                                                v3078 = v3079;
                                            } else {
                                                v3078 = v0;
                                            }
                                            v3076 = v3078;
                                        }
                                        v3071 = v3076;
                                    }
                                    v3070 = v3071;
                                } else {
                                    let v3080: f64;
                                    if v2650 != 0.0 {
                                        let v2655 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3081: f64;
                                        if v2655 != 0.0 {
                                            let v2656 = if v2473 == v0 { 1.0 } else { 0.0 };
                                            let v3082: f64;
                                            if v2656 != 0.0 {
                                                v3082 = v0;
                                            } else {
                                                let v2659 = (v2372 * v2367) / (v70 * v2473);
                                                v3082 = v2659;
                                            }
                                            v3081 = v3082;
                                        } else {
                                            let v2664 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3083: f64;
                                            if v2664 != 0.0 {
                                                let v2665 = v2367 + v2368;
                                                let v2666 = if v2665 == v0 { 1.0 } else { 0.0 };
                                                if v2666 != 0.0 {
                                                } else {
                                                }
                                                let v2668 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2666 != 0.0 { 1.0 } else { 0.0 };
                                                let v3084: f64;
                                                if v2668 != 0.0 {
                                                    v3084 = v0;
                                                } else {
                                                    let v2672 = (v2372 * v70) / ((v2427 * v2473) * v2665);
                                                    v3084 = v2672;
                                                }
                                                v3083 = v3084;
                                            } else {
                                                v3083 = v0;
                                            }
                                            v3081 = v3083;
                                        }
                                        v3080 = v3081;
                                    } else {
                                        let v2677 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3085: f64;
                                        if v2677 != 0.0 {
                                            let v2678 = if v2473 == v0 { 1.0 } else { 0.0 };
                                            let v3086: f64;
                                            if v2678 != 0.0 {
                                                v3086 = v0;
                                            } else {
                                                let v2681 = (v2372 * v2367) / (v70 * v2473);
                                                v3086 = v2681;
                                            }
                                            v3085 = v3086;
                                        } else {
                                            let v2686 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3087: f64;
                                            if v2686 != 0.0 {
                                                let v2687 = v2367 + v2368;
                                                let v2688 = if v2687 == v0 { 1.0 } else { 0.0 };
                                                if v2688 != 0.0 {
                                                } else {
                                                }
                                                let v2690 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2688 != 0.0 { 1.0 } else { 0.0 };
                                                let v3088: f64;
                                                if v2690 != 0.0 {
                                                    v3088 = v0;
                                                } else {
                                                    let v2694 = (v2372 * v70) / ((v2427 * v2473) * v2687);
                                                    v3088 = v2694;
                                                }
                                                v3087 = v3088;
                                            } else {
                                                v3087 = v0;
                                            }
                                            v3085 = v3087;
                                        }
                                        v3080 = v3085;
                                    }
                                    v3070 = v3080;
                                }
                                v3015 = v3010;
                                v3069 = v3070;
                            } else {
                                let v2695 = if v2379 == v2427 { 1.0 } else { 0.0 };
                                let v3016: f64;
                                let v3089: f64;
                                if v2695 != 0.0 {
                                    let v3090: f64;
                                    if v2696 != 0.0 {
                                        let v3091: f64;
                                        if v2697 != 0.0 {
                                            let v2702 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3092: f64;
                                            if v2702 != 0.0 {
                                                let v2703 = if v2420 == v0 { 1.0 } else { 0.0 };
                                                let v3093: f64;
                                                if v2703 != 0.0 {
                                                    v3093 = v0;
                                                } else {
                                                    let v2706 = (v2372 * v2367) / (v70 * v2420);
                                                    v3093 = v2706;
                                                }
                                                v3092 = v3093;
                                            } else {
                                                let v2711 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3094: f64;
                                                if v2711 != 0.0 {
                                                    let v2712 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                    if v2712 != 0.0 {
                                                    } else {
                                                    }
                                                    let v2714 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2712 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3095: f64;
                                                    if v2714 != 0.0 {
                                                        v3095 = v0;
                                                    } else {
                                                        let v2718 = (v2372 * v70) / ((v2432 * v2420) * v2367);
                                                        v3095 = v2718;
                                                    }
                                                    v3094 = v3095;
                                                } else {
                                                    v3094 = v0;
                                                }
                                                v3092 = v3094;
                                            }
                                            v3091 = v3092;
                                        } else {
                                            let v2723 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3096: f64;
                                            if v2723 != 0.0 {
                                                let v2724 = if v2420 == v0 { 1.0 } else { 0.0 };
                                                let v3097: f64;
                                                if v2724 != 0.0 {
                                                    v3097 = v0;
                                                } else {
                                                    let v2727 = (v2372 * v2367) / (v70 * v2420);
                                                    v3097 = v2727;
                                                }
                                                v3096 = v3097;
                                            } else {
                                                let v2732 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3098: f64;
                                                if v2732 != 0.0 {
                                                    let v2733 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                    if v2733 != 0.0 {
                                                    } else {
                                                    }
                                                    let v2735 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2733 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3099: f64;
                                                    if v2735 != 0.0 {
                                                        v3099 = v0;
                                                    } else {
                                                        let v2739 = (v2372 * v70) / ((v2432 * v2420) * v2367);
                                                        v3099 = v2739;
                                                    }
                                                    v3098 = v3099;
                                                } else {
                                                    v3098 = v0;
                                                }
                                                v3096 = v3098;
                                            }
                                            v3091 = v3096;
                                        }
                                        v3090 = v3091;
                                    } else {
                                        let v3100: f64;
                                        if v2740 != 0.0 {
                                            let v2745 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3101: f64;
                                            if v2745 != 0.0 {
                                                let v2746 = if v2473 == v0 { 1.0 } else { 0.0 };
                                                let v3102: f64;
                                                if v2746 != 0.0 {
                                                    v3102 = v0;
                                                } else {
                                                    let v2749 = (v2372 * v2367) / (v70 * v2473);
                                                    v3102 = v2749;
                                                }
                                                v3101 = v3102;
                                            } else {
                                                let v2754 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3103: f64;
                                                if v2754 != 0.0 {
                                                    let v2755 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                    if v2755 != 0.0 {
                                                    } else {
                                                    }
                                                    let v2757 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2755 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3104: f64;
                                                    if v2757 != 0.0 {
                                                        v3104 = v0;
                                                    } else {
                                                        let v2761 = (v2372 * v70) / ((v2432 * v2473) * v2367);
                                                        v3104 = v2761;
                                                    }
                                                    v3103 = v3104;
                                                } else {
                                                    v3103 = v0;
                                                }
                                                v3101 = v3103;
                                            }
                                            v3100 = v3101;
                                        } else {
                                            let v2766 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3105: f64;
                                            if v2766 != 0.0 {
                                                let v2767 = if v2473 == v0 { 1.0 } else { 0.0 };
                                                let v3106: f64;
                                                if v2767 != 0.0 {
                                                    v3106 = v0;
                                                } else {
                                                    let v2770 = (v2372 * v2367) / (v70 * v2473);
                                                    v3106 = v2770;
                                                }
                                                v3105 = v3106;
                                            } else {
                                                let v2775 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3107: f64;
                                                if v2775 != 0.0 {
                                                    let v2776 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                    if v2776 != 0.0 {
                                                    } else {
                                                    }
                                                    let v2778 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2776 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3108: f64;
                                                    if v2778 != 0.0 {
                                                        v3108 = v0;
                                                    } else {
                                                        let v2782 = (v2372 * v70) / ((v2432 * v2473) * v2367);
                                                        v3108 = v2782;
                                                    }
                                                    v3107 = v3108;
                                                } else {
                                                    v3107 = v0;
                                                }
                                                v3105 = v3107;
                                            }
                                            v3100 = v3105;
                                        }
                                        v3090 = v3100;
                                    }
                                    v3016 = v3010;
                                    v3089 = v3090;
                                } else {
                                    let v2783 = if v2379 == v2429 { 1.0 } else { 0.0 };
                                    let v3017: f64;
                                    let v3109: f64;
                                    if v2783 != 0.0 {
                                        let v3110: f64;
                                        if v2784 != 0.0 {
                                            let v3111: f64;
                                            if v2785 != 0.0 {
                                                let v2790 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3112: f64;
                                                if v2790 != 0.0 {
                                                    let v2791 = if v2420 == v0 { 1.0 } else { 0.0 };
                                                    let v3113: f64;
                                                    if v2791 != 0.0 {
                                                        v3113 = v0;
                                                    } else {
                                                        let v2794 = (v2372 * v2367) / (v70 * v2420);
                                                        v3113 = v2794;
                                                    }
                                                    v3112 = v3113;
                                                } else {
                                                    let v2799 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3114: f64;
                                                    if v2799 != 0.0 {
                                                        let v2800 = v2367 + v2368;
                                                        let v2801 = if v2800 == v0 { 1.0 } else { 0.0 };
                                                        if v2801 != 0.0 {
                                                        } else {
                                                        }
                                                        let v2803 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2801 != 0.0 { 1.0 } else { 0.0 };
                                                        let v3115: f64;
                                                        if v2803 != 0.0 {
                                                            v3115 = v0;
                                                        } else {
                                                            let v2807 = (v2372 * v70) / ((v2427 * v2420) * v2800);
                                                            v3115 = v2807;
                                                        }
                                                        v3114 = v3115;
                                                    } else {
                                                        v3114 = v0;
                                                    }
                                                    v3112 = v3114;
                                                }
                                                v3111 = v3112;
                                            } else {
                                                let v2812 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3116: f64;
                                                if v2812 != 0.0 {
                                                    let v2813 = if v2420 == v0 { 1.0 } else { 0.0 };
                                                    let v3117: f64;
                                                    if v2813 != 0.0 {
                                                        v3117 = v0;
                                                    } else {
                                                        let v2816 = (v2372 * v2367) / (v70 * v2420);
                                                        v3117 = v2816;
                                                    }
                                                    v3116 = v3117;
                                                } else {
                                                    let v2821 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3118: f64;
                                                    if v2821 != 0.0 {
                                                        let v2822 = v2367 + v2368;
                                                        let v2823 = if v2822 == v0 { 1.0 } else { 0.0 };
                                                        if v2823 != 0.0 {
                                                        } else {
                                                        }
                                                        let v2825 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2823 != 0.0 { 1.0 } else { 0.0 };
                                                        let v3119: f64;
                                                        if v2825 != 0.0 {
                                                            v3119 = v0;
                                                        } else {
                                                            let v2829 = (v2372 * v70) / ((v2427 * v2420) * v2822);
                                                            v3119 = v2829;
                                                        }
                                                        v3118 = v3119;
                                                    } else {
                                                        v3118 = v0;
                                                    }
                                                    v3116 = v3118;
                                                }
                                                v3111 = v3116;
                                            }
                                            v3110 = v3111;
                                        } else {
                                            let v2831 = (v2372 * v2370) / v70;
                                            v3110 = v2831;
                                        }
                                        v3017 = v3010;
                                        v3109 = v3110;
                                    } else {
                                        let v2832 = if v2379 == v2417 { 1.0 } else { 0.0 };
                                        let v3018: f64;
                                        let v3120: f64;
                                        if v2832 != 0.0 {
                                            let v3121: f64;
                                            if v2833 != 0.0 {
                                                let v3122: f64;
                                                if v2834 != 0.0 {
                                                    let v2839 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3123: f64;
                                                    if v2839 != 0.0 {
                                                        let v2840 = if v2420 == v0 { 1.0 } else { 0.0 };
                                                        let v3124: f64;
                                                        if v2840 != 0.0 {
                                                            v3124 = v0;
                                                        } else {
                                                            let v2843 = (v2372 * v2367) / (v70 * v2420);
                                                            v3124 = v2843;
                                                        }
                                                        v3123 = v3124;
                                                    } else {
                                                        let v2848 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3125: f64;
                                                        if v2848 != 0.0 {
                                                            let v2849 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                            if v2849 != 0.0 {
                                                            } else {
                                                            }
                                                            let v2851 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2849 != 0.0 { 1.0 } else { 0.0 };
                                                            let v3126: f64;
                                                            if v2851 != 0.0 {
                                                                v3126 = v0;
                                                            } else {
                                                                let v2855 = (v2372 * v70) / ((v2432 * v2420) * v2367);
                                                                v3126 = v2855;
                                                            }
                                                            v3125 = v3126;
                                                        } else {
                                                            v3125 = v0;
                                                        }
                                                        v3123 = v3125;
                                                    }
                                                    v3122 = v3123;
                                                } else {
                                                    let v2860 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3127: f64;
                                                    if v2860 != 0.0 {
                                                        let v2861 = if v2420 == v0 { 1.0 } else { 0.0 };
                                                        let v3128: f64;
                                                        if v2861 != 0.0 {
                                                            v3128 = v0;
                                                        } else {
                                                            let v2864 = (v2372 * v2367) / (v70 * v2420);
                                                            v3128 = v2864;
                                                        }
                                                        v3127 = v3128;
                                                    } else {
                                                        let v2869 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3129: f64;
                                                        if v2869 != 0.0 {
                                                            let v2870 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                            if v2870 != 0.0 {
                                                            } else {
                                                            }
                                                            let v2872 = if (if v2420 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2870 != 0.0 { 1.0 } else { 0.0 };
                                                            let v3130: f64;
                                                            if v2872 != 0.0 {
                                                                v3130 = v0;
                                                            } else {
                                                                let v2876 = (v2372 * v70) / ((v2432 * v2420) * v2367);
                                                                v3130 = v2876;
                                                            }
                                                            v3129 = v3130;
                                                        } else {
                                                            v3129 = v0;
                                                        }
                                                        v3127 = v3129;
                                                    }
                                                    v3122 = v3127;
                                                }
                                                v3121 = v3122;
                                            } else {
                                                let v2877 = if v2473 == v0 { 1.0 } else { 0.0 };
                                                let v3131: f64;
                                                if v2877 != 0.0 {
                                                    v3131 = v0;
                                                } else {
                                                    let v2880 = (v2372 * v2370) / (v70 * v2473);
                                                    v3131 = v2880;
                                                }
                                                v3121 = v3131;
                                            }
                                            v3018 = v3010;
                                            v3120 = v3121;
                                        } else {
                                            let v2881 = if v2379 == v2432 { 1.0 } else { 0.0 };
                                            let v3019: f64;
                                            let v3132: f64;
                                            if v2881 != 0.0 {
                                                let v3133: f64;
                                                if v2882 != 0.0 {
                                                    let v2884 = (v2372 * v2370) / v70;
                                                    v3133 = v2884;
                                                } else {
                                                    let v3134: f64;
                                                    if v2885 != 0.0 {
                                                        let v2890 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3135: f64;
                                                        if v2890 != 0.0 {
                                                            let v2891 = if v2473 == v0 { 1.0 } else { 0.0 };
                                                            let v3136: f64;
                                                            if v2891 != 0.0 {
                                                                v3136 = v0;
                                                            } else {
                                                                let v2894 = (v2372 * v2367) / (v70 * v2473);
                                                                v3136 = v2894;
                                                            }
                                                            v3135 = v3136;
                                                        } else {
                                                            let v2899 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3137: f64;
                                                            if v2899 != 0.0 {
                                                                let v2900 = v2367 + v2368;
                                                                let v2901 = if v2900 == v0 { 1.0 } else { 0.0 };
                                                                if v2901 != 0.0 {
                                                                } else {
                                                                }
                                                                let v2903 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2901 != 0.0 { 1.0 } else { 0.0 };
                                                                let v3138: f64;
                                                                if v2903 != 0.0 {
                                                                    v3138 = v0;
                                                                } else {
                                                                    let v2907 = (v2372 * v70) / ((v2427 * v2473) * v2900);
                                                                    v3138 = v2907;
                                                                }
                                                                v3137 = v3138;
                                                            } else {
                                                                v3137 = v0;
                                                            }
                                                            v3135 = v3137;
                                                        }
                                                        v3134 = v3135;
                                                    } else {
                                                        let v2912 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3139: f64;
                                                        if v2912 != 0.0 {
                                                            let v2913 = if v2473 == v0 { 1.0 } else { 0.0 };
                                                            let v3140: f64;
                                                            if v2913 != 0.0 {
                                                                v3140 = v0;
                                                            } else {
                                                                let v2916 = (v2372 * v2367) / (v70 * v2473);
                                                                v3140 = v2916;
                                                            }
                                                            v3139 = v3140;
                                                        } else {
                                                            let v2921 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3141: f64;
                                                            if v2921 != 0.0 {
                                                                let v2922 = v2367 + v2368;
                                                                let v2923 = if v2922 == v0 { 1.0 } else { 0.0 };
                                                                if v2923 != 0.0 {
                                                                } else {
                                                                }
                                                                let v2925 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2923 != 0.0 { 1.0 } else { 0.0 };
                                                                let v3142: f64;
                                                                if v2925 != 0.0 {
                                                                    v3142 = v0;
                                                                } else {
                                                                    let v2929 = (v2372 * v70) / ((v2427 * v2473) * v2922);
                                                                    v3142 = v2929;
                                                                }
                                                                v3141 = v3142;
                                                            } else {
                                                                v3141 = v0;
                                                            }
                                                            v3139 = v3141;
                                                        }
                                                        v3134 = v3139;
                                                    }
                                                    v3133 = v3134;
                                                }
                                                v3019 = v3010;
                                                v3132 = v3133;
                                            } else {
                                                let v2930 = if v2379 == v2446 { 1.0 } else { 0.0 };
                                                let v3020: f64;
                                                let v3143: f64;
                                                if v2930 != 0.0 {
                                                    let v3144: f64;
                                                    if v2931 != 0.0 {
                                                        let v2932 = if v2420 == v0 { 1.0 } else { 0.0 };
                                                        let v3145: f64;
                                                        if v2932 != 0.0 {
                                                            v3145 = v0;
                                                        } else {
                                                            let v2935 = (v2372 * v2370) / (v70 * v2420);
                                                            v3145 = v2935;
                                                        }
                                                        v3144 = v3145;
                                                    } else {
                                                        let v3146: f64;
                                                        if v2936 != 0.0 {
                                                            let v2941 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3147: f64;
                                                            if v2941 != 0.0 {
                                                                let v2942 = if v2473 == v0 { 1.0 } else { 0.0 };
                                                                let v3148: f64;
                                                                if v2942 != 0.0 {
                                                                    v3148 = v0;
                                                                } else {
                                                                    let v2945 = (v2372 * v2367) / (v70 * v2473);
                                                                    v3148 = v2945;
                                                                }
                                                                v3147 = v3148;
                                                            } else {
                                                                let v2950 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let v3149: f64;
                                                                if v2950 != 0.0 {
                                                                    let v2951 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                                    if v2951 != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let v2953 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2951 != 0.0 { 1.0 } else { 0.0 };
                                                                    let v3150: f64;
                                                                    if v2953 != 0.0 {
                                                                        v3150 = v0;
                                                                    } else {
                                                                        let v2957 = (v2372 * v70) / ((v2432 * v2473) * v2367);
                                                                        v3150 = v2957;
                                                                    }
                                                                    v3149 = v3150;
                                                                } else {
                                                                    v3149 = v0;
                                                                }
                                                                v3147 = v3149;
                                                            }
                                                            v3146 = v3147;
                                                        } else {
                                                            let v2962 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3151: f64;
                                                            if v2962 != 0.0 {
                                                                let v2963 = if v2473 == v0 { 1.0 } else { 0.0 };
                                                                let v3152: f64;
                                                                if v2963 != 0.0 {
                                                                    v3152 = v0;
                                                                } else {
                                                                    let v2966 = (v2372 * v2367) / (v70 * v2473);
                                                                    v3152 = v2966;
                                                                }
                                                                v3151 = v3152;
                                                            } else {
                                                                let v2971 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let v3153: f64;
                                                                if v2971 != 0.0 {
                                                                    let v2972 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                                    if v2972 != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let v2974 = if (if v2473 == v0 { 1.0 } else { 0.0 }) != 0.0 || v2972 != 0.0 { 1.0 } else { 0.0 };
                                                                    let v3154: f64;
                                                                    if v2974 != 0.0 {
                                                                        v3154 = v0;
                                                                    } else {
                                                                        let v2978 = (v2372 * v70) / ((v2432 * v2473) * v2367);
                                                                        v3154 = v2978;
                                                                    }
                                                                    v3153 = v3154;
                                                                } else {
                                                                    v3153 = v0;
                                                                }
                                                                v3151 = v3153;
                                                            }
                                                            v3146 = v3151;
                                                        }
                                                        v3144 = v3146;
                                                    }
                                                    v3020 = v3010;
                                                    v3143 = v3144;
                                                } else {
                                                    let v2979 = if v2379 == v2456 { 1.0 } else { 0.0 };
                                                    let v3021: f64;
                                                    let v3155: f64;
                                                    if v2979 != 0.0 {
                                                        let v2981 = (v2372 * v2370) / v70;
                                                        v3021 = v3010;
                                                        v3155 = v2981;
                                                    } else {
                                                        let v2982 = if v2379 == v2380 { 1.0 } else { 0.0 };
                                                        let v3022: f64;
                                                        let v3156: f64;
                                                        if v2982 != 0.0 {
                                                            let v3023: f64;
                                                            let v3157: f64;
                                                            if v2983 != 0.0 {
                                                                let v2986 = ((v1940 * v2372) * v2367) / v70;
                                                                let v2987 = if v24 == v63 { 1.0 } else { 0.0 };
                                                                let v3024: f64;
                                                                if v2987 != 0.0 {
                                                                    v3024 = v0;
                                                                } else {
                                                                    let v2991 = (v2372 * v2367) / (v70 * (v24 - v63));
                                                                    v3024 = v2991;
                                                                }
                                                                v3023 = v3024;
                                                                v3157 = v2986;
                                                            } else {
                                                                let v2994 = (v2372 * v2367) / (v70 * v24);
                                                                v3023 = v2994;
                                                                v3157 = v0;
                                                            }
                                                            v3022 = v3023;
                                                            v3156 = v3157;
                                                        } else {
                                                            let v2996 = if v2379 == v2995 { 1.0 } else { 0.0 };
                                                            let v3025: f64;
                                                            let v3158: f64;
                                                            if v2996 != 0.0 {
                                                                let v3026: f64;
                                                                let v3159: f64;
                                                                if v2997 != 0.0 {
                                                                    let v3000 = (v2372 * v2367) / (v70 * v24);
                                                                    v3026 = v3000;
                                                                    v3159 = v0;
                                                                } else {
                                                                    let v3003 = ((v1940 * v2372) * v2367) / v70;
                                                                    let v3004 = if v24 == v63 { 1.0 } else { 0.0 };
                                                                    let v3027: f64;
                                                                    if v3004 != 0.0 {
                                                                        v3027 = v0;
                                                                    } else {
                                                                        let v3008 = (v2372 * v2367) / (v70 * (v24 - v63));
                                                                        v3027 = v3008;
                                                                    }
                                                                    v3026 = v3027;
                                                                    v3159 = v3003;
                                                                }
                                                                v3025 = v3026;
                                                                v3158 = v3159;
                                                            } else {
                                                                v3025 = v0;
                                                                v3158 = v0;
                                                            }
                                                            v3022 = v3025;
                                                            v3156 = v3158;
                                                        }
                                                        v3021 = v3022;
                                                        v3155 = v3156;
                                                    }
                                                    v3020 = v3021;
                                                    v3143 = v3155;
                                                }
                                                v3019 = v3020;
                                                v3132 = v3143;
                                            }
                                            v3018 = v3019;
                                            v3120 = v3132;
                                        }
                                        v3017 = v3018;
                                        v3109 = v3120;
                                    }
                                    v3016 = v3017;
                                    v3089 = v3109;
                                }
                                v3015 = v3016;
                                v3069 = v3089;
                            }
                            v3014 = v3015;
                            v3049 = v3069;
                        }
                        v3009 = v3014;
                        v3029 = v3049;
                    }
                    let v3028 = if v3009 <= v0 { 1.0 } else { 0.0 };
                    let v3164: f64;
                    if v3028 != 0.0 {
                        v3164 = v3029;
                    } else {
                        let v3160 = if v3029 <= v0 { 1.0 } else { 0.0 };
                        let v3165: f64;
                        if v3160 != 0.0 {
                            v3165 = v3009;
                        } else {
                            let v3163 = (v3009 * v3029) / (v3009 + v3029);
                            v3165 = v3163;
                        }
                        v3164 = v3165;
                    }
                    let v3166 = if v3164 == v0 { 1.0 } else { 0.0 };
                    if v3166 != 0.0 {
                    } else {
                    }
                    v3214 = v2420;
                    v3264 = v2473;
                    v3803 = v3009;
                    v3951 = v3029;
                    v3961 = v3164;
                    v4970 = v4971;
                    v4987 = v4988;
                } else {
                    v3214 = v0;
                    v3264 = v0;
                    v3803 = v0;
                    v3951 = v0;
                    v3961 = v0;
                    v4970 = v0;
                    v4987 = v0;
                }
                v3213 = v3214;
                v3263 = v3264;
                v3802 = v3803;
                v3950 = v3951;
                v3960 = v3961;
                v4969 = v4970;
                v4986 = v4987;
            }
            let v3964: f64;
            let v4962: f64;
            let v4968: f64;
            let v4979: f64;
            let v4985: f64;
            if v3167 != 0.0 {
                let v3169 = v2372 * v3168;
                v3964 = v3169;
                v4962 = v3213;
                v4968 = v4969;
                v4979 = v3263;
                v4985 = v4986;
            } else {
                let v3172 = if (if v2375 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v2372 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v3965: f64;
                let v4963: f64;
                let v4972: f64;
                let v4980: f64;
                let v4989: f64;
                if v3172 != 0.0 {
                    let v3173 = if v2379 < v2380 { 1.0 } else { 0.0 };
                    let v3210: f64;
                    let v3260: f64;
                    let v3798: f64;
                    let v4973: f64;
                    let v4990: f64;
                    if v3173 != 0.0 {
                        let v3175 = if (v24 % v63) != v0 { 1.0 } else { 0.0 };
                        let v3190: f64;
                        let v3196: f64;
                        let v3211: f64;
                        let v3261: f64;
                        if v3175 != 0.0 {
                            let v3179 = v63 * (if ((v24 - v1) / v63) >= v0 { ((v24 - v1) / v63) } else { v0 });
                            v3190 = v3179;
                            v3196 = v3179;
                            v3211 = v1;
                            v3261 = v1;
                        } else {
                            let v3180 = if v2388 == v1 { 1.0 } else { 0.0 };
                            let v3191: f64;
                            let v3197: f64;
                            let v3212: f64;
                            let v3262: f64;
                            if v3180 != 0.0 {
                                let v3184 = v63 * (if ((v24 / v63) - v1) >= v0 { ((v24 / v63) - v1) } else { v0 });
                                v3191 = v24;
                                v3197 = v3184;
                                v3212 = v0;
                                v3262 = v63;
                            } else {
                                let v3188 = v63 * (if ((v24 / v63) - v1) >= v0 { ((v24 / v63) - v1) } else { v0 });
                                v3191 = v3188;
                                v3197 = v24;
                                v3212 = v63;
                                v3262 = v0;
                            }
                            v3190 = v3191;
                            v3196 = v3197;
                            v3211 = v3212;
                            v3261 = v3262;
                        }
                        let v3799: f64;
                        if v3189 != 0.0 {
                            let v3192 = if v3190 == v0 { 1.0 } else { 0.0 };
                            let v3800: f64;
                            if v3192 != 0.0 {
                                v3800 = v0;
                            } else {
                                let v3195 = (v2372 * v2367) / (v70 * v3190);
                                v3800 = v3195;
                            }
                            v3799 = v3800;
                        } else {
                            let v3198 = if v3196 == v0 { 1.0 } else { 0.0 };
                            let v3801: f64;
                            if v3198 != 0.0 {
                                v3801 = v0;
                            } else {
                                let v3201 = (v2372 * v2367) / (v70 * v3196);
                                v3801 = v3201;
                            }
                            v3799 = v3801;
                        }
                        v3210 = v3211;
                        v3260 = v3261;
                        v3798 = v3799;
                        v4973 = v3190;
                        v4990 = v3196;
                    } else {
                        v3210 = v3213;
                        v3260 = v3263;
                        v3798 = v3802;
                        v4973 = v4969;
                        v4990 = v4986;
                    }
                    let v3202 = if v2379 == v0 { 1.0 } else { 0.0 };
                    let v3797: f64;
                    let v3819: f64;
                    if v3202 != 0.0 {
                        let v3820: f64;
                        if v3203 != 0.0 {
                            let v3821: f64;
                            if v3204 != 0.0 {
                                let v3209 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3822: f64;
                                if v3209 != 0.0 {
                                    let v3215 = if v3210 == v0 { 1.0 } else { 0.0 };
                                    let v3823: f64;
                                    if v3215 != 0.0 {
                                        v3823 = v0;
                                    } else {
                                        let v3218 = (v2372 * v2367) / (v70 * v3210);
                                        v3823 = v3218;
                                    }
                                    v3822 = v3823;
                                } else {
                                    let v3223 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3824: f64;
                                    if v3223 != 0.0 {
                                        let v3224 = v2367 + v2368;
                                        let v3225 = if v3224 == v0 { 1.0 } else { 0.0 };
                                        if v3225 != 0.0 {
                                        } else {
                                        }
                                        let v3227 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3225 != 0.0 { 1.0 } else { 0.0 };
                                        let v3825: f64;
                                        if v3227 != 0.0 {
                                            v3825 = v0;
                                        } else {
                                            let v3231 = (v2372 * v70) / ((v2427 * v3210) * v3224);
                                            v3825 = v3231;
                                        }
                                        v3824 = v3825;
                                    } else {
                                        v3824 = v0;
                                    }
                                    v3822 = v3824;
                                }
                                v3821 = v3822;
                            } else {
                                let v3236 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3826: f64;
                                if v3236 != 0.0 {
                                    let v3237 = if v3210 == v0 { 1.0 } else { 0.0 };
                                    let v3827: f64;
                                    if v3237 != 0.0 {
                                        v3827 = v0;
                                    } else {
                                        let v3240 = (v2372 * v2367) / (v70 * v3210);
                                        v3827 = v3240;
                                    }
                                    v3826 = v3827;
                                } else {
                                    let v3245 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3828: f64;
                                    if v3245 != 0.0 {
                                        let v3246 = v2367 + v2368;
                                        let v3247 = if v3246 == v0 { 1.0 } else { 0.0 };
                                        if v3247 != 0.0 {
                                        } else {
                                        }
                                        let v3249 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3247 != 0.0 { 1.0 } else { 0.0 };
                                        let v3829: f64;
                                        if v3249 != 0.0 {
                                            v3829 = v0;
                                        } else {
                                            let v3253 = (v2372 * v70) / ((v2427 * v3210) * v3246);
                                            v3829 = v3253;
                                        }
                                        v3828 = v3829;
                                    } else {
                                        v3828 = v0;
                                    }
                                    v3826 = v3828;
                                }
                                v3821 = v3826;
                            }
                            v3820 = v3821;
                        } else {
                            let v3830: f64;
                            if v3254 != 0.0 {
                                let v3259 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3831: f64;
                                if v3259 != 0.0 {
                                    let v3265 = if v3260 == v0 { 1.0 } else { 0.0 };
                                    let v3832: f64;
                                    if v3265 != 0.0 {
                                        v3832 = v0;
                                    } else {
                                        let v3268 = (v2372 * v2367) / (v70 * v3260);
                                        v3832 = v3268;
                                    }
                                    v3831 = v3832;
                                } else {
                                    let v3273 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3833: f64;
                                    if v3273 != 0.0 {
                                        let v3274 = v2367 + v2368;
                                        let v3275 = if v3274 == v0 { 1.0 } else { 0.0 };
                                        if v3275 != 0.0 {
                                        } else {
                                        }
                                        let v3277 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3275 != 0.0 { 1.0 } else { 0.0 };
                                        let v3834: f64;
                                        if v3277 != 0.0 {
                                            v3834 = v0;
                                        } else {
                                            let v3281 = (v2372 * v70) / ((v2427 * v3260) * v3274);
                                            v3834 = v3281;
                                        }
                                        v3833 = v3834;
                                    } else {
                                        v3833 = v0;
                                    }
                                    v3831 = v3833;
                                }
                                v3830 = v3831;
                            } else {
                                let v3286 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                let v3835: f64;
                                if v3286 != 0.0 {
                                    let v3287 = if v3260 == v0 { 1.0 } else { 0.0 };
                                    let v3836: f64;
                                    if v3287 != 0.0 {
                                        v3836 = v0;
                                    } else {
                                        let v3290 = (v2372 * v2367) / (v70 * v3260);
                                        v3836 = v3290;
                                    }
                                    v3835 = v3836;
                                } else {
                                    let v3295 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3837: f64;
                                    if v3295 != 0.0 {
                                        let v3296 = v2367 + v2368;
                                        let v3297 = if v3296 == v0 { 1.0 } else { 0.0 };
                                        if v3297 != 0.0 {
                                        } else {
                                        }
                                        let v3299 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3297 != 0.0 { 1.0 } else { 0.0 };
                                        let v3838: f64;
                                        if v3299 != 0.0 {
                                            v3838 = v0;
                                        } else {
                                            let v3303 = (v2372 * v70) / ((v2427 * v3260) * v3296);
                                            v3838 = v3303;
                                        }
                                        v3837 = v3838;
                                    } else {
                                        v3837 = v0;
                                    }
                                    v3835 = v3837;
                                }
                                v3830 = v3835;
                            }
                            v3820 = v3830;
                        }
                        v3797 = v3798;
                        v3819 = v3820;
                    } else {
                        let v3304 = if v2379 == v1 { 1.0 } else { 0.0 };
                        let v3804: f64;
                        let v3839: f64;
                        if v3304 != 0.0 {
                            let v3840: f64;
                            if v3305 != 0.0 {
                                let v3841: f64;
                                if v3306 != 0.0 {
                                    let v3311 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3842: f64;
                                    if v3311 != 0.0 {
                                        let v3312 = if v3210 == v0 { 1.0 } else { 0.0 };
                                        let v3843: f64;
                                        if v3312 != 0.0 {
                                            v3843 = v0;
                                        } else {
                                            let v3315 = (v2372 * v2367) / (v70 * v3210);
                                            v3843 = v3315;
                                        }
                                        v3842 = v3843;
                                    } else {
                                        let v3320 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3844: f64;
                                        if v3320 != 0.0 {
                                            let v3321 = v2367 + v2368;
                                            let v3322 = if v3321 == v0 { 1.0 } else { 0.0 };
                                            if v3322 != 0.0 {
                                            } else {
                                            }
                                            let v3324 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3322 != 0.0 { 1.0 } else { 0.0 };
                                            let v3845: f64;
                                            if v3324 != 0.0 {
                                                v3845 = v0;
                                            } else {
                                                let v3328 = (v2372 * v70) / ((v2427 * v3210) * v3321);
                                                v3845 = v3328;
                                            }
                                            v3844 = v3845;
                                        } else {
                                            v3844 = v0;
                                        }
                                        v3842 = v3844;
                                    }
                                    v3841 = v3842;
                                } else {
                                    let v3333 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3846: f64;
                                    if v3333 != 0.0 {
                                        let v3334 = if v3210 == v0 { 1.0 } else { 0.0 };
                                        let v3847: f64;
                                        if v3334 != 0.0 {
                                            v3847 = v0;
                                        } else {
                                            let v3337 = (v2372 * v2367) / (v70 * v3210);
                                            v3847 = v3337;
                                        }
                                        v3846 = v3847;
                                    } else {
                                        let v3342 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3848: f64;
                                        if v3342 != 0.0 {
                                            let v3343 = v2367 + v2368;
                                            let v3344 = if v3343 == v0 { 1.0 } else { 0.0 };
                                            if v3344 != 0.0 {
                                            } else {
                                            }
                                            let v3346 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3344 != 0.0 { 1.0 } else { 0.0 };
                                            let v3849: f64;
                                            if v3346 != 0.0 {
                                                v3849 = v0;
                                            } else {
                                                let v3350 = (v2372 * v70) / ((v2427 * v3210) * v3343);
                                                v3849 = v3350;
                                            }
                                            v3848 = v3849;
                                        } else {
                                            v3848 = v0;
                                        }
                                        v3846 = v3848;
                                    }
                                    v3841 = v3846;
                                }
                                v3840 = v3841;
                            } else {
                                let v3850: f64;
                                if v3351 != 0.0 {
                                    let v3356 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3851: f64;
                                    if v3356 != 0.0 {
                                        let v3357 = if v3260 == v0 { 1.0 } else { 0.0 };
                                        let v3852: f64;
                                        if v3357 != 0.0 {
                                            v3852 = v0;
                                        } else {
                                            let v3360 = (v2372 * v2367) / (v70 * v3260);
                                            v3852 = v3360;
                                        }
                                        v3851 = v3852;
                                    } else {
                                        let v3365 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3853: f64;
                                        if v3365 != 0.0 {
                                            let v3366 = if v2367 == v0 { 1.0 } else { 0.0 };
                                            if v3366 != 0.0 {
                                            } else {
                                            }
                                            let v3368 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3366 != 0.0 { 1.0 } else { 0.0 };
                                            let v3854: f64;
                                            if v3368 != 0.0 {
                                                v3854 = v0;
                                            } else {
                                                let v3372 = (v2372 * v70) / ((v2432 * v3260) * v2367);
                                                v3854 = v3372;
                                            }
                                            v3853 = v3854;
                                        } else {
                                            v3853 = v0;
                                        }
                                        v3851 = v3853;
                                    }
                                    v3850 = v3851;
                                } else {
                                    let v3377 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                    let v3855: f64;
                                    if v3377 != 0.0 {
                                        let v3378 = if v3260 == v0 { 1.0 } else { 0.0 };
                                        let v3856: f64;
                                        if v3378 != 0.0 {
                                            v3856 = v0;
                                        } else {
                                            let v3381 = (v2372 * v2367) / (v70 * v3260);
                                            v3856 = v3381;
                                        }
                                        v3855 = v3856;
                                    } else {
                                        let v3386 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3857: f64;
                                        if v3386 != 0.0 {
                                            let v3387 = if v2367 == v0 { 1.0 } else { 0.0 };
                                            if v3387 != 0.0 {
                                            } else {
                                            }
                                            let v3389 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3387 != 0.0 { 1.0 } else { 0.0 };
                                            let v3858: f64;
                                            if v3389 != 0.0 {
                                                v3858 = v0;
                                            } else {
                                                let v3393 = (v2372 * v70) / ((v2432 * v3260) * v2367);
                                                v3858 = v3393;
                                            }
                                            v3857 = v3858;
                                        } else {
                                            v3857 = v0;
                                        }
                                        v3855 = v3857;
                                    }
                                    v3850 = v3855;
                                }
                                v3840 = v3850;
                            }
                            v3804 = v3798;
                            v3839 = v3840;
                        } else {
                            let v3394 = if v2379 == v63 { 1.0 } else { 0.0 };
                            let v3805: f64;
                            let v3859: f64;
                            if v3394 != 0.0 {
                                let v3860: f64;
                                if v3395 != 0.0 {
                                    let v3861: f64;
                                    if v3396 != 0.0 {
                                        let v3401 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3862: f64;
                                        if v3401 != 0.0 {
                                            let v3402 = if v3210 == v0 { 1.0 } else { 0.0 };
                                            let v3863: f64;
                                            if v3402 != 0.0 {
                                                v3863 = v0;
                                            } else {
                                                let v3405 = (v2372 * v2367) / (v70 * v3210);
                                                v3863 = v3405;
                                            }
                                            v3862 = v3863;
                                        } else {
                                            let v3410 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3864: f64;
                                            if v3410 != 0.0 {
                                                let v3411 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                if v3411 != 0.0 {
                                                } else {
                                                }
                                                let v3413 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3411 != 0.0 { 1.0 } else { 0.0 };
                                                let v3865: f64;
                                                if v3413 != 0.0 {
                                                    v3865 = v0;
                                                } else {
                                                    let v3417 = (v2372 * v70) / ((v2432 * v3210) * v2367);
                                                    v3865 = v3417;
                                                }
                                                v3864 = v3865;
                                            } else {
                                                v3864 = v0;
                                            }
                                            v3862 = v3864;
                                        }
                                        v3861 = v3862;
                                    } else {
                                        let v3422 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3866: f64;
                                        if v3422 != 0.0 {
                                            let v3423 = if v3210 == v0 { 1.0 } else { 0.0 };
                                            let v3867: f64;
                                            if v3423 != 0.0 {
                                                v3867 = v0;
                                            } else {
                                                let v3426 = (v2372 * v2367) / (v70 * v3210);
                                                v3867 = v3426;
                                            }
                                            v3866 = v3867;
                                        } else {
                                            let v3431 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3868: f64;
                                            if v3431 != 0.0 {
                                                let v3432 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                if v3432 != 0.0 {
                                                } else {
                                                }
                                                let v3434 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3432 != 0.0 { 1.0 } else { 0.0 };
                                                let v3869: f64;
                                                if v3434 != 0.0 {
                                                    v3869 = v0;
                                                } else {
                                                    let v3438 = (v2372 * v70) / ((v2432 * v3210) * v2367);
                                                    v3869 = v3438;
                                                }
                                                v3868 = v3869;
                                            } else {
                                                v3868 = v0;
                                            }
                                            v3866 = v3868;
                                        }
                                        v3861 = v3866;
                                    }
                                    v3860 = v3861;
                                } else {
                                    let v3870: f64;
                                    if v3439 != 0.0 {
                                        let v3444 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3871: f64;
                                        if v3444 != 0.0 {
                                            let v3445 = if v3260 == v0 { 1.0 } else { 0.0 };
                                            let v3872: f64;
                                            if v3445 != 0.0 {
                                                v3872 = v0;
                                            } else {
                                                let v3448 = (v2372 * v2367) / (v70 * v3260);
                                                v3872 = v3448;
                                            }
                                            v3871 = v3872;
                                        } else {
                                            let v3453 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3873: f64;
                                            if v3453 != 0.0 {
                                                let v3454 = v2367 + v2368;
                                                let v3455 = if v3454 == v0 { 1.0 } else { 0.0 };
                                                if v3455 != 0.0 {
                                                } else {
                                                }
                                                let v3457 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3455 != 0.0 { 1.0 } else { 0.0 };
                                                let v3874: f64;
                                                if v3457 != 0.0 {
                                                    v3874 = v0;
                                                } else {
                                                    let v3461 = (v2372 * v70) / ((v2427 * v3260) * v3454);
                                                    v3874 = v3461;
                                                }
                                                v3873 = v3874;
                                            } else {
                                                v3873 = v0;
                                            }
                                            v3871 = v3873;
                                        }
                                        v3870 = v3871;
                                    } else {
                                        let v3466 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                        let v3875: f64;
                                        if v3466 != 0.0 {
                                            let v3467 = if v3260 == v0 { 1.0 } else { 0.0 };
                                            let v3876: f64;
                                            if v3467 != 0.0 {
                                                v3876 = v0;
                                            } else {
                                                let v3470 = (v2372 * v2367) / (v70 * v3260);
                                                v3876 = v3470;
                                            }
                                            v3875 = v3876;
                                        } else {
                                            let v3475 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3877: f64;
                                            if v3475 != 0.0 {
                                                let v3476 = v2367 + v2368;
                                                let v3477 = if v3476 == v0 { 1.0 } else { 0.0 };
                                                if v3477 != 0.0 {
                                                } else {
                                                }
                                                let v3479 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3477 != 0.0 { 1.0 } else { 0.0 };
                                                let v3878: f64;
                                                if v3479 != 0.0 {
                                                    v3878 = v0;
                                                } else {
                                                    let v3483 = (v2372 * v70) / ((v2427 * v3260) * v3476);
                                                    v3878 = v3483;
                                                }
                                                v3877 = v3878;
                                            } else {
                                                v3877 = v0;
                                            }
                                            v3875 = v3877;
                                        }
                                        v3870 = v3875;
                                    }
                                    v3860 = v3870;
                                }
                                v3805 = v3798;
                                v3859 = v3860;
                            } else {
                                let v3484 = if v2379 == v2427 { 1.0 } else { 0.0 };
                                let v3806: f64;
                                let v3879: f64;
                                if v3484 != 0.0 {
                                    let v3880: f64;
                                    if v3485 != 0.0 {
                                        let v3881: f64;
                                        if v3486 != 0.0 {
                                            let v3491 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3882: f64;
                                            if v3491 != 0.0 {
                                                let v3492 = if v3210 == v0 { 1.0 } else { 0.0 };
                                                let v3883: f64;
                                                if v3492 != 0.0 {
                                                    v3883 = v0;
                                                } else {
                                                    let v3495 = (v2372 * v2367) / (v70 * v3210);
                                                    v3883 = v3495;
                                                }
                                                v3882 = v3883;
                                            } else {
                                                let v3500 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3884: f64;
                                                if v3500 != 0.0 {
                                                    let v3501 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                    if v3501 != 0.0 {
                                                    } else {
                                                    }
                                                    let v3503 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3501 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3885: f64;
                                                    if v3503 != 0.0 {
                                                        v3885 = v0;
                                                    } else {
                                                        let v3507 = (v2372 * v70) / ((v2432 * v3210) * v2367);
                                                        v3885 = v3507;
                                                    }
                                                    v3884 = v3885;
                                                } else {
                                                    v3884 = v0;
                                                }
                                                v3882 = v3884;
                                            }
                                            v3881 = v3882;
                                        } else {
                                            let v3512 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3886: f64;
                                            if v3512 != 0.0 {
                                                let v3513 = if v3210 == v0 { 1.0 } else { 0.0 };
                                                let v3887: f64;
                                                if v3513 != 0.0 {
                                                    v3887 = v0;
                                                } else {
                                                    let v3516 = (v2372 * v2367) / (v70 * v3210);
                                                    v3887 = v3516;
                                                }
                                                v3886 = v3887;
                                            } else {
                                                let v3521 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3888: f64;
                                                if v3521 != 0.0 {
                                                    let v3522 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                    if v3522 != 0.0 {
                                                    } else {
                                                    }
                                                    let v3524 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3522 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3889: f64;
                                                    if v3524 != 0.0 {
                                                        v3889 = v0;
                                                    } else {
                                                        let v3528 = (v2372 * v70) / ((v2432 * v3210) * v2367);
                                                        v3889 = v3528;
                                                    }
                                                    v3888 = v3889;
                                                } else {
                                                    v3888 = v0;
                                                }
                                                v3886 = v3888;
                                            }
                                            v3881 = v3886;
                                        }
                                        v3880 = v3881;
                                    } else {
                                        let v3890: f64;
                                        if v3529 != 0.0 {
                                            let v3534 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3891: f64;
                                            if v3534 != 0.0 {
                                                let v3535 = if v3260 == v0 { 1.0 } else { 0.0 };
                                                let v3892: f64;
                                                if v3535 != 0.0 {
                                                    v3892 = v0;
                                                } else {
                                                    let v3538 = (v2372 * v2367) / (v70 * v3260);
                                                    v3892 = v3538;
                                                }
                                                v3891 = v3892;
                                            } else {
                                                let v3543 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3893: f64;
                                                if v3543 != 0.0 {
                                                    let v3544 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                    if v3544 != 0.0 {
                                                    } else {
                                                    }
                                                    let v3546 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3544 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3894: f64;
                                                    if v3546 != 0.0 {
                                                        v3894 = v0;
                                                    } else {
                                                        let v3550 = (v2372 * v70) / ((v2432 * v3260) * v2367);
                                                        v3894 = v3550;
                                                    }
                                                    v3893 = v3894;
                                                } else {
                                                    v3893 = v0;
                                                }
                                                v3891 = v3893;
                                            }
                                            v3890 = v3891;
                                        } else {
                                            let v3555 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                            let v3895: f64;
                                            if v3555 != 0.0 {
                                                let v3556 = if v3260 == v0 { 1.0 } else { 0.0 };
                                                let v3896: f64;
                                                if v3556 != 0.0 {
                                                    v3896 = v0;
                                                } else {
                                                    let v3559 = (v2372 * v2367) / (v70 * v3260);
                                                    v3896 = v3559;
                                                }
                                                v3895 = v3896;
                                            } else {
                                                let v3564 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3897: f64;
                                                if v3564 != 0.0 {
                                                    let v3565 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                    if v3565 != 0.0 {
                                                    } else {
                                                    }
                                                    let v3567 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3565 != 0.0 { 1.0 } else { 0.0 };
                                                    let v3898: f64;
                                                    if v3567 != 0.0 {
                                                        v3898 = v0;
                                                    } else {
                                                        let v3571 = (v2372 * v70) / ((v2432 * v3260) * v2367);
                                                        v3898 = v3571;
                                                    }
                                                    v3897 = v3898;
                                                } else {
                                                    v3897 = v0;
                                                }
                                                v3895 = v3897;
                                            }
                                            v3890 = v3895;
                                        }
                                        v3880 = v3890;
                                    }
                                    v3806 = v3798;
                                    v3879 = v3880;
                                } else {
                                    let v3572 = if v2379 == v2429 { 1.0 } else { 0.0 };
                                    let v3807: f64;
                                    let v3899: f64;
                                    if v3572 != 0.0 {
                                        let v3900: f64;
                                        if v3573 != 0.0 {
                                            let v3901: f64;
                                            if v3574 != 0.0 {
                                                let v3579 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3902: f64;
                                                if v3579 != 0.0 {
                                                    let v3580 = if v3210 == v0 { 1.0 } else { 0.0 };
                                                    let v3903: f64;
                                                    if v3580 != 0.0 {
                                                        v3903 = v0;
                                                    } else {
                                                        let v3583 = (v2372 * v2367) / (v70 * v3210);
                                                        v3903 = v3583;
                                                    }
                                                    v3902 = v3903;
                                                } else {
                                                    let v3588 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3904: f64;
                                                    if v3588 != 0.0 {
                                                        let v3589 = v2367 + v2368;
                                                        let v3590 = if v3589 == v0 { 1.0 } else { 0.0 };
                                                        if v3590 != 0.0 {
                                                        } else {
                                                        }
                                                        let v3592 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3590 != 0.0 { 1.0 } else { 0.0 };
                                                        let v3905: f64;
                                                        if v3592 != 0.0 {
                                                            v3905 = v0;
                                                        } else {
                                                            let v3596 = (v2372 * v70) / ((v2427 * v3210) * v3589);
                                                            v3905 = v3596;
                                                        }
                                                        v3904 = v3905;
                                                    } else {
                                                        v3904 = v0;
                                                    }
                                                    v3902 = v3904;
                                                }
                                                v3901 = v3902;
                                            } else {
                                                let v3601 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                let v3906: f64;
                                                if v3601 != 0.0 {
                                                    let v3602 = if v3210 == v0 { 1.0 } else { 0.0 };
                                                    let v3907: f64;
                                                    if v3602 != 0.0 {
                                                        v3907 = v0;
                                                    } else {
                                                        let v3605 = (v2372 * v2367) / (v70 * v3210);
                                                        v3907 = v3605;
                                                    }
                                                    v3906 = v3907;
                                                } else {
                                                    let v3610 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3908: f64;
                                                    if v3610 != 0.0 {
                                                        let v3611 = v2367 + v2368;
                                                        let v3612 = if v3611 == v0 { 1.0 } else { 0.0 };
                                                        if v3612 != 0.0 {
                                                        } else {
                                                        }
                                                        let v3614 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3612 != 0.0 { 1.0 } else { 0.0 };
                                                        let v3909: f64;
                                                        if v3614 != 0.0 {
                                                            v3909 = v0;
                                                        } else {
                                                            let v3618 = (v2372 * v70) / ((v2427 * v3210) * v3611);
                                                            v3909 = v3618;
                                                        }
                                                        v3908 = v3909;
                                                    } else {
                                                        v3908 = v0;
                                                    }
                                                    v3906 = v3908;
                                                }
                                                v3901 = v3906;
                                            }
                                            v3900 = v3901;
                                        } else {
                                            let v3620 = (v2372 * v2370) / v70;
                                            v3900 = v3620;
                                        }
                                        v3807 = v3798;
                                        v3899 = v3900;
                                    } else {
                                        let v3621 = if v2379 == v2417 { 1.0 } else { 0.0 };
                                        let v3808: f64;
                                        let v3910: f64;
                                        if v3621 != 0.0 {
                                            let v3911: f64;
                                            if v3622 != 0.0 {
                                                let v3912: f64;
                                                if v3623 != 0.0 {
                                                    let v3628 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3913: f64;
                                                    if v3628 != 0.0 {
                                                        let v3629 = if v3210 == v0 { 1.0 } else { 0.0 };
                                                        let v3914: f64;
                                                        if v3629 != 0.0 {
                                                            v3914 = v0;
                                                        } else {
                                                            let v3632 = (v2372 * v2367) / (v70 * v3210);
                                                            v3914 = v3632;
                                                        }
                                                        v3913 = v3914;
                                                    } else {
                                                        let v3637 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3915: f64;
                                                        if v3637 != 0.0 {
                                                            let v3638 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                            if v3638 != 0.0 {
                                                            } else {
                                                            }
                                                            let v3640 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3638 != 0.0 { 1.0 } else { 0.0 };
                                                            let v3916: f64;
                                                            if v3640 != 0.0 {
                                                                v3916 = v0;
                                                            } else {
                                                                let v3644 = (v2372 * v70) / ((v2432 * v3210) * v2367);
                                                                v3916 = v3644;
                                                            }
                                                            v3915 = v3916;
                                                        } else {
                                                            v3915 = v0;
                                                        }
                                                        v3913 = v3915;
                                                    }
                                                    v3912 = v3913;
                                                } else {
                                                    let v3649 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                    let v3917: f64;
                                                    if v3649 != 0.0 {
                                                        let v3650 = if v3210 == v0 { 1.0 } else { 0.0 };
                                                        let v3918: f64;
                                                        if v3650 != 0.0 {
                                                            v3918 = v0;
                                                        } else {
                                                            let v3653 = (v2372 * v2367) / (v70 * v3210);
                                                            v3918 = v3653;
                                                        }
                                                        v3917 = v3918;
                                                    } else {
                                                        let v3658 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3919: f64;
                                                        if v3658 != 0.0 {
                                                            let v3659 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                            if v3659 != 0.0 {
                                                            } else {
                                                            }
                                                            let v3661 = if (if v3210 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3659 != 0.0 { 1.0 } else { 0.0 };
                                                            let v3920: f64;
                                                            if v3661 != 0.0 {
                                                                v3920 = v0;
                                                            } else {
                                                                let v3665 = (v2372 * v70) / ((v2432 * v3210) * v2367);
                                                                v3920 = v3665;
                                                            }
                                                            v3919 = v3920;
                                                        } else {
                                                            v3919 = v0;
                                                        }
                                                        v3917 = v3919;
                                                    }
                                                    v3912 = v3917;
                                                }
                                                v3911 = v3912;
                                            } else {
                                                let v3666 = if v3260 == v0 { 1.0 } else { 0.0 };
                                                let v3921: f64;
                                                if v3666 != 0.0 {
                                                    v3921 = v0;
                                                } else {
                                                    let v3669 = (v2372 * v2370) / (v70 * v3260);
                                                    v3921 = v3669;
                                                }
                                                v3911 = v3921;
                                            }
                                            v3808 = v3798;
                                            v3910 = v3911;
                                        } else {
                                            let v3670 = if v2379 == v2432 { 1.0 } else { 0.0 };
                                            let v3809: f64;
                                            let v3922: f64;
                                            if v3670 != 0.0 {
                                                let v3923: f64;
                                                if v3671 != 0.0 {
                                                    let v3673 = (v2372 * v2370) / v70;
                                                    v3923 = v3673;
                                                } else {
                                                    let v3924: f64;
                                                    if v3674 != 0.0 {
                                                        let v3679 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3925: f64;
                                                        if v3679 != 0.0 {
                                                            let v3680 = if v3260 == v0 { 1.0 } else { 0.0 };
                                                            let v3926: f64;
                                                            if v3680 != 0.0 {
                                                                v3926 = v0;
                                                            } else {
                                                                let v3683 = (v2372 * v2367) / (v70 * v3260);
                                                                v3926 = v3683;
                                                            }
                                                            v3925 = v3926;
                                                        } else {
                                                            let v3688 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3927: f64;
                                                            if v3688 != 0.0 {
                                                                let v3689 = v2367 + v2368;
                                                                let v3690 = if v3689 == v0 { 1.0 } else { 0.0 };
                                                                if v3690 != 0.0 {
                                                                } else {
                                                                }
                                                                let v3692 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3690 != 0.0 { 1.0 } else { 0.0 };
                                                                let v3928: f64;
                                                                if v3692 != 0.0 {
                                                                    v3928 = v0;
                                                                } else {
                                                                    let v3696 = (v2372 * v70) / ((v2427 * v3260) * v3689);
                                                                    v3928 = v3696;
                                                                }
                                                                v3927 = v3928;
                                                            } else {
                                                                v3927 = v0;
                                                            }
                                                            v3925 = v3927;
                                                        }
                                                        v3924 = v3925;
                                                    } else {
                                                        let v3701 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                        let v3929: f64;
                                                        if v3701 != 0.0 {
                                                            let v3702 = if v3260 == v0 { 1.0 } else { 0.0 };
                                                            let v3930: f64;
                                                            if v3702 != 0.0 {
                                                                v3930 = v0;
                                                            } else {
                                                                let v3705 = (v2372 * v2367) / (v70 * v3260);
                                                                v3930 = v3705;
                                                            }
                                                            v3929 = v3930;
                                                        } else {
                                                            let v3710 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3931: f64;
                                                            if v3710 != 0.0 {
                                                                let v3711 = v2367 + v2368;
                                                                let v3712 = if v3711 == v0 { 1.0 } else { 0.0 };
                                                                if v3712 != 0.0 {
                                                                } else {
                                                                }
                                                                let v3714 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3712 != 0.0 { 1.0 } else { 0.0 };
                                                                let v3932: f64;
                                                                if v3714 != 0.0 {
                                                                    v3932 = v0;
                                                                } else {
                                                                    let v3718 = (v2372 * v70) / ((v2427 * v3260) * v3711);
                                                                    v3932 = v3718;
                                                                }
                                                                v3931 = v3932;
                                                            } else {
                                                                v3931 = v0;
                                                            }
                                                            v3929 = v3931;
                                                        }
                                                        v3924 = v3929;
                                                    }
                                                    v3923 = v3924;
                                                }
                                                v3809 = v3798;
                                                v3922 = v3923;
                                            } else {
                                                let v3719 = if v2379 == v2446 { 1.0 } else { 0.0 };
                                                let v3810: f64;
                                                let v3933: f64;
                                                if v3719 != 0.0 {
                                                    let v3934: f64;
                                                    if v3720 != 0.0 {
                                                        let v3721 = if v3210 == v0 { 1.0 } else { 0.0 };
                                                        let v3935: f64;
                                                        if v3721 != 0.0 {
                                                            v3935 = v0;
                                                        } else {
                                                            let v3724 = (v2372 * v2370) / (v70 * v3210);
                                                            v3935 = v3724;
                                                        }
                                                        v3934 = v3935;
                                                    } else {
                                                        let v3936: f64;
                                                        if v3725 != 0.0 {
                                                            let v3730 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3937: f64;
                                                            if v3730 != 0.0 {
                                                                let v3731 = if v3260 == v0 { 1.0 } else { 0.0 };
                                                                let v3938: f64;
                                                                if v3731 != 0.0 {
                                                                    v3938 = v0;
                                                                } else {
                                                                    let v3734 = (v2372 * v2367) / (v70 * v3260);
                                                                    v3938 = v3734;
                                                                }
                                                                v3937 = v3938;
                                                            } else {
                                                                let v3739 = if (if (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2432 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let v3939: f64;
                                                                if v3739 != 0.0 {
                                                                    let v3740 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                                    if v3740 != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let v3742 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3740 != 0.0 { 1.0 } else { 0.0 };
                                                                    let v3940: f64;
                                                                    if v3742 != 0.0 {
                                                                        v3940 = v0;
                                                                    } else {
                                                                        let v3746 = (v2372 * v70) / ((v2432 * v3260) * v2367);
                                                                        v3940 = v3746;
                                                                    }
                                                                    v3939 = v3940;
                                                                } else {
                                                                    v3939 = v0;
                                                                }
                                                                v3937 = v3939;
                                                            }
                                                            v3936 = v3937;
                                                        } else {
                                                            let v3751 = if (if (if v2375 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2446 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                            let v3941: f64;
                                                            if v3751 != 0.0 {
                                                                let v3752 = if v3260 == v0 { 1.0 } else { 0.0 };
                                                                let v3942: f64;
                                                                if v3752 != 0.0 {
                                                                    v3942 = v0;
                                                                } else {
                                                                    let v3755 = (v2372 * v2367) / (v70 * v3260);
                                                                    v3942 = v3755;
                                                                }
                                                                v3941 = v3942;
                                                            } else {
                                                                let v3760 = if (if (if v2375 == v63 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2429 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v2375 == v2456 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                                                                let v3943: f64;
                                                                if v3760 != 0.0 {
                                                                    let v3761 = if v2367 == v0 { 1.0 } else { 0.0 };
                                                                    if v3761 != 0.0 {
                                                                    } else {
                                                                    }
                                                                    let v3763 = if (if v3260 == v0 { 1.0 } else { 0.0 }) != 0.0 || v3761 != 0.0 { 1.0 } else { 0.0 };
                                                                    let v3944: f64;
                                                                    if v3763 != 0.0 {
                                                                        v3944 = v0;
                                                                    } else {
                                                                        let v3767 = (v2372 * v70) / ((v2432 * v3260) * v2367);
                                                                        v3944 = v3767;
                                                                    }
                                                                    v3943 = v3944;
                                                                } else {
                                                                    v3943 = v0;
                                                                }
                                                                v3941 = v3943;
                                                            }
                                                            v3936 = v3941;
                                                        }
                                                        v3934 = v3936;
                                                    }
                                                    v3810 = v3798;
                                                    v3933 = v3934;
                                                } else {
                                                    let v3768 = if v2379 == v2456 { 1.0 } else { 0.0 };
                                                    let v3811: f64;
                                                    let v3945: f64;
                                                    if v3768 != 0.0 {
                                                        let v3770 = (v2372 * v2370) / v70;
                                                        v3811 = v3798;
                                                        v3945 = v3770;
                                                    } else {
                                                        let v3771 = if v2379 == v2380 { 1.0 } else { 0.0 };
                                                        let v3812: f64;
                                                        let v3946: f64;
                                                        if v3771 != 0.0 {
                                                            let v3813: f64;
                                                            let v3947: f64;
                                                            if v3772 != 0.0 {
                                                                let v3775 = ((v1940 * v2372) * v2367) / v70;
                                                                let v3776 = if v24 == v63 { 1.0 } else { 0.0 };
                                                                let v3814: f64;
                                                                if v3776 != 0.0 {
                                                                    v3814 = v0;
                                                                } else {
                                                                    let v3780 = (v2372 * v2367) / (v70 * (v24 - v63));
                                                                    v3814 = v3780;
                                                                }
                                                                v3813 = v3814;
                                                                v3947 = v3775;
                                                            } else {
                                                                let v3783 = (v2372 * v2367) / (v70 * v24);
                                                                v3813 = v3783;
                                                                v3947 = v0;
                                                            }
                                                            v3812 = v3813;
                                                            v3946 = v3947;
                                                        } else {
                                                            let v3784 = if v2379 == v2995 { 1.0 } else { 0.0 };
                                                            let v3815: f64;
                                                            let v3948: f64;
                                                            if v3784 != 0.0 {
                                                                let v3816: f64;
                                                                let v3949: f64;
                                                                if v3785 != 0.0 {
                                                                    let v3788 = (v2372 * v2367) / (v70 * v24);
                                                                    v3816 = v3788;
                                                                    v3949 = v0;
                                                                } else {
                                                                    let v3791 = ((v1940 * v2372) * v2367) / v70;
                                                                    let v3792 = if v24 == v63 { 1.0 } else { 0.0 };
                                                                    let v3817: f64;
                                                                    if v3792 != 0.0 {
                                                                        v3817 = v0;
                                                                    } else {
                                                                        let v3796 = (v2372 * v2367) / (v70 * (v24 - v63));
                                                                        v3817 = v3796;
                                                                    }
                                                                    v3816 = v3817;
                                                                    v3949 = v3791;
                                                                }
                                                                v3815 = v3816;
                                                                v3948 = v3949;
                                                            } else {
                                                                v3815 = v0;
                                                                v3948 = v3950;
                                                            }
                                                            v3812 = v3815;
                                                            v3946 = v3948;
                                                        }
                                                        v3811 = v3812;
                                                        v3945 = v3946;
                                                    }
                                                    v3810 = v3811;
                                                    v3933 = v3945;
                                                }
                                                v3809 = v3810;
                                                v3922 = v3933;
                                            }
                                            v3808 = v3809;
                                            v3910 = v3922;
                                        }
                                        v3807 = v3808;
                                        v3899 = v3910;
                                    }
                                    v3806 = v3807;
                                    v3879 = v3899;
                                }
                                v3805 = v3806;
                                v3859 = v3879;
                            }
                            v3804 = v3805;
                            v3839 = v3859;
                        }
                        v3797 = v3804;
                        v3819 = v3839;
                    }
                    let v3818 = if v3797 <= v0 { 1.0 } else { 0.0 };
                    let v3956: f64;
                    if v3818 != 0.0 {
                        v3956 = v3819;
                    } else {
                        let v3952 = if v3819 <= v0 { 1.0 } else { 0.0 };
                        let v3957: f64;
                        if v3952 != 0.0 {
                            v3957 = v3797;
                        } else {
                            let v3955 = (v3797 * v3819) / (v3797 + v3819);
                            v3957 = v3955;
                        }
                        v3956 = v3957;
                    }
                    let v3958 = if v3956 == v0 { 1.0 } else { 0.0 };
                    if v3958 != 0.0 {
                    } else {
                    }
                    v3965 = v3956;
                    v4963 = v3210;
                    v4972 = v4973;
                    v4980 = v3260;
                    v4989 = v4990;
                } else {
                    v3965 = v0;
                    v4963 = v3213;
                    v4972 = v4969;
                    v4980 = v3263;
                    v4989 = v4986;
                }
                v3964 = v3965;
                v4962 = v4963;
                v4968 = v4972;
                v4979 = v4980;
                v4985 = v4989;
            }
            let v3959 = if v2287 == v0 { 1.0 } else { 0.0 };
            let v6004: f64;
            let v6012: f64;
            if v3959 != 0.0 {
                let v3963 = if v3960 < v3962 { 1.0 } else { 0.0 };
                let v6005: f64;
                if v3963 != 0.0 {
                    v6005 = v0;
                } else {
                    v6005 = v3960;
                }
                let v3966 = if v3964 < v3962 { 1.0 } else { 0.0 };
                let v6013: f64;
                if v3966 != 0.0 {
                    v6013 = v0;
                } else {
                    v6013 = v3964;
                }
                v6004 = v6005;
                v6012 = v6013;
            } else {
                let v3967 = if v3960 <= v3962 { 1.0 } else { 0.0 };
                let v6006: f64;
                if v3967 != 0.0 {
                    v6006 = v3962;
                } else {
                    v6006 = v3960;
                }
                let v3968 = if v3964 <= v3962 { 1.0 } else { 0.0 };
                let v6014: f64;
                if v3968 != 0.0 {
                    v6014 = v3962;
                } else {
                    v6014 = v3964;
                }
                v6004 = v6006;
                v6012 = v6014;
            }
            let v5995: f64;
            let v5997: f64;
            let v6735: f64;
            let v6737: f64;
            let v6763: f64;
            let v6765: f64;
            if v2288 != 0.0 {
                let v3969 = if v546 <= v0 { 1.0 } else { 0.0 };
                let v6736: f64;
                if v3969 != 0.0 {
                    v6736 = v0;
                } else {
                    v6736 = v546;
                }
                let v3970 = if v556 <= v0 { 1.0 } else { 0.0 };
                let v6764: f64;
                if v3970 != 0.0 {
                    v6764 = v0;
                } else {
                    v6764 = v556;
                }
                let v3972 = if v3971 <= v0 { 1.0 } else { 0.0 };
                let v6738: f64;
                if v3972 != 0.0 {
                    v6738 = v0;
                } else {
                    v6738 = v3971;
                }
                let v3974 = if v3973 <= v0 { 1.0 } else { 0.0 };
                let v6766: f64;
                if v3974 != 0.0 {
                    v6766 = v0;
                } else {
                    v6766 = v3973;
                }
                v5995 = v576;
                v5997 = v3976;
                v6735 = v6736;
                v6737 = v6738;
                v6763 = v6764;
                v6765 = v6766;
            } else {
                let v3975 = if v576 <= v0 { 1.0 } else { 0.0 };
                let v5996: f64;
                if v3975 != 0.0 {
                    v5996 = v0;
                } else {
                    v5996 = v576;
                }
                let v3977 = if v3976 <= v0 { 1.0 } else { 0.0 };
                let v5998: f64;
                if v3977 != 0.0 {
                    v5998 = v0;
                } else {
                    v5998 = v3976;
                }
                v5995 = v5996;
                v5997 = v5998;
                v6735 = v546;
                v6737 = v3971;
                v6763 = v556;
                v6765 = v3973;
            }
            let v3979 = if v3978 != v0 { 1.0 } else { 0.0 };
            let v10229: f64;
            let v10235: f64;
            let v10240: f64;
            let v10247: f64;
            let v10252: f64;
            if v3979 != 0.0 {
                let v3984 = (if (v65 * v3980) >= v3982 { (v65 * v3980) } else { v3982 }).ln();
                let v3987 = (if (v70 * v3980) >= v3982 { (v70 * v3980) } else { v3982 }).ln();
                let v3989 = (if v24 >= v3982 { v24 } else { v3982 }).ln();
                let v3999 = if (if v3995 == 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v3997 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v4012: f64;
                if v3999 != 0.0 {
                    v4012 = v1;
                } else {
                    let v4010 = if (if (if v4000 == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4002 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if (if v4005 == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v4007 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v4013: f64;
                    if v4010 != 0.0 {
                        v4013 = v2427;
                    } else {
                        v4013 = v2417;
                    }
                    v4012 = v4013;
                }
                let v4011 = if v3978 == v63 { 1.0 } else { 0.0 };
                let v4100: f64;
                let v4108: f64;
                let v4112: f64;
                let v4117: f64;
                let v4122: f64;
                if v4011 != 0.0 {
                    let v4014 = if v4012 == v2417 { 1.0 } else { 0.0 };
                    let v4101: f64;
                    let v4118: f64;
                    if v4014 != 0.0 {
                        let v4024 = rspice_limited_exp((((v4016 * v3984) + (v4018 * v3987)) + (v4021 * v3989)));
                        let v4025 = v4015 * v4024;
                        let v4035 = rspice_limited_exp((((v4027 * v3984) + (v4029 * v3987)) + (v4032 * v3989)));
                        let v4036 = v4026 * v4035;
                        let v4039 = (v4025 * v4036) / (v4025 + v4036);
                        let v4041 = v4040 * v4024;
                        let v4043 = v4042 * v4035;
                        let v4046 = (v4041 * v4043) / (v4041 + v4043);
                        v4101 = v4046;
                        v4118 = v4039;
                    } else {
                        v4101 = v3993;
                        v4118 = v3994;
                    }
                    let v4048 = if (if v4012 == v2427 { 1.0 } else { 0.0 }) != 0.0 || v4014 != 0.0 { 1.0 } else { 0.0 };
                    let v4113: f64;
                    let v4123: f64;
                    if v4048 != 0.0 {
                        let v4059 = v4049 * (rspice_limited_exp((((v4050 * v3984) + (v4052 * v3987)) + (v4055 * v3989))));
                        let v4070 = v4060 * (rspice_limited_exp((((v4061 * v3984) + (v4063 * v3987)) + (v4066 * v3989))));
                        v4113 = v4059;
                        v4123 = v4070;
                    } else {
                        v4113 = v3992;
                        v4123 = v3991;
                    }
                    let v4081 = v4071 * (rspice_limited_exp((((v4072 * v3984) + (v4074 * v3987)) + (v4077 * v3989))));
                    let v4092 = v4082 * (rspice_limited_exp((((v4083 * v3984) + (v4085 * v3987)) + (v4088 * v3989))));
                    let v4095 = (v4081 * v4092) / (v4081 + v4092);
                    v4100 = v4101;
                    v4108 = v4095;
                    v4112 = v4113;
                    v4117 = v4118;
                    v4122 = v4123;
                } else {
                    v4100 = v3993;
                    v4108 = v3990;
                    v4112 = v3992;
                    v4117 = v3994;
                    v4122 = v3991;
                }
                let v4099 = if (if v3978 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if v4011 != 0.0 && (if v4012 == v2417 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v10230: f64;
                let v10236: f64;
                let v10241: f64;
                let v10248: f64;
                let v10253: f64;
                if v4099 != 0.0 {
                    let v4103 = if v4100 < v4102 { 1.0 } else { 0.0 };
                    let v10249: f64;
                    if v4103 != 0.0 {
                        v10249 = v4104;
                    } else {
                        let v4107 = v4105 + (v1 / v4100);
                        v10249 = v4107;
                    }
                    let v4109 = if v4108 < v4102 { 1.0 } else { 0.0 };
                    let v10242: f64;
                    if v4109 != 0.0 {
                        v10242 = v4104;
                    } else {
                        let v4111 = v4105 + (v1 / v4108);
                        v10242 = v4111;
                    }
                    let v4114 = if v4112 < v4102 { 1.0 } else { 0.0 };
                    let v10231: f64;
                    if v4114 != 0.0 {
                        v10231 = v4104;
                    } else {
                        let v4116 = v4105 + (v1 / v4112);
                        v10231 = v4116;
                    }
                    let v4119 = if v4117 < v4102 { 1.0 } else { 0.0 };
                    let v10237: f64;
                    if v4119 != 0.0 {
                        v10237 = v4104;
                    } else {
                        let v4121 = v4105 + (v1 / v4117);
                        v10237 = v4121;
                    }
                    let v4124 = if v4122 < v4102 { 1.0 } else { 0.0 };
                    let v10254: f64;
                    if v4124 != 0.0 {
                        v10254 = v4104;
                    } else {
                        let v4126 = v4105 + (v1 / v4122);
                        v10254 = v4126;
                    }
                    v10230 = v10231;
                    v10236 = v10237;
                    v10241 = v10242;
                    v10248 = v10249;
                    v10253 = v10254;
                } else {
                    let v4128 = if v4011 != 0.0 && (if v4012 == v2427 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v10232: f64;
                    let v10238: f64;
                    let v10243: f64;
                    let v10250: f64;
                    let v10255: f64;
                    if v4128 != 0.0 {
                        let v4129 = if v4108 < v4102 { 1.0 } else { 0.0 };
                        let v10244: f64;
                        if v4129 != 0.0 {
                            v10244 = v4104;
                        } else {
                            let v4131 = v4105 + (v1 / v4108);
                            v10244 = v4131;
                        }
                        let v4132 = if v4112 < v4102 { 1.0 } else { 0.0 };
                        let v10233: f64;
                        if v4132 != 0.0 {
                            v10233 = v4104;
                        } else {
                            let v4134 = v4105 + (v1 / v4112);
                            v10233 = v4134;
                        }
                        let v4135 = if v4122 < v4102 { 1.0 } else { 0.0 };
                        let v10256: f64;
                        if v4135 != 0.0 {
                            v10256 = v4104;
                        } else {
                            let v4137 = v4105 + (v1 / v4122);
                            v10256 = v4137;
                        }
                        v10232 = v10233;
                        v10238 = v4105;
                        v10243 = v10244;
                        v10250 = v4105;
                        v10255 = v10256;
                    } else {
                        let v4139 = if v4011 != 0.0 && (if v4012 == v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v10234: f64;
                        let v10239: f64;
                        let v10245: f64;
                        let v10251: f64;
                        let v10257: f64;
                        if v4139 != 0.0 {
                            let v4140 = if v4108 < v4102 { 1.0 } else { 0.0 };
                            let v10246: f64;
                            if v4140 != 0.0 {
                                v10246 = v4104;
                            } else {
                                let v4142 = v4105 + (v1 / v4108);
                                v10246 = v4142;
                            }
                            v10234 = v4104;
                            v10239 = v4105;
                            v10245 = v10246;
                            v10251 = v4105;
                            v10257 = v4104;
                        } else {
                            v10234 = v0;
                            v10239 = v0;
                            v10245 = v0;
                            v10251 = v0;
                            v10257 = v0;
                        }
                        v10232 = v10234;
                        v10238 = v10239;
                        v10243 = v10245;
                        v10250 = v10251;
                        v10255 = v10257;
                    }
                    v10230 = v10232;
                    v10236 = v10238;
                    v10241 = v10243;
                    v10248 = v10250;
                    v10253 = v10255;
                }
                v10229 = v10230;
                v10235 = v10236;
                v10240 = v10241;
                v10247 = v10248;
                v10252 = v10253;
            } else {
                v10229 = v0;
                v10235 = v0;
                v10240 = v0;
                v10247 = v0;
                v10252 = v0;
            }
            let v4144 = if v4143 == v1 { 1.0 } else { 0.0 };
            let v8141: f64;
            let v10266: f64;
            if v4144 != 0.0 {
                let v4146 = if v4145 < v4102 { 1.0 } else { 0.0 };
                let v10267: f64;
                if v4146 != 0.0 {
                    v10267 = v4104;
                } else {
                    let v4148 = v4105 + (v1 / v4145);
                    v10267 = v4148;
                }
                let v4150 = v1 - v4149;
                v8141 = v4150;
                v10266 = v10267;
            } else {
                v8141 = v1;
                v10266 = v0;
            }
            let v4162 = (v4151 * (v4152 + ((v112 / v2427) / v4154))) / ((v4154 * v24) * (v22 - v4159));
            let v4163 = if v4162 > v0 { 1.0 } else { 0.0 };
            let v7672: f64;
            if v4163 != 0.0 {
                let v4164 = v1 / v4162;
                v7672 = v4164;
            } else {
                let v4166 = if v4165 != v0 { 1.0 } else { 0.0 };
                if v4166 != 0.0 {
                } else {
                }
                v7672 = v4104;
            }
            let v4168 = v10 * v1156;
            let v4176 = (rspice_limited_exp((v1186 * ((if (v4170 / v10) >= v3982 { (v4170 / v10) } else { v3982 }).ln())))) / (v10 * v10);
            let v4182 = (rspice_limited_exp((v1186 * ((if (v4170 / v4168) >= v3982 { (v4170 / v4168) } else { v3982 }).ln())))) / (v4168 * v4168);
            let v4185: f64;
            if v3 != 0.0 {
                v4185 = v4183;
            } else {
                v4185 = v4184;
            }
            let v4188: f64;
            if v3 != 0.0 {
                v4188 = v4186;
            } else {
                v4188 = v4187;
            }
            let v4190 = (v4185 * v70) * v4182;
            let v4192 = (-v4188) * v10;
            let v4193 = v4192 * v1156;
            let v4196 = v4185 * ((v70 * v65) * v4176);
            let v4201 = if (if v2282 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v2284 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v4203 = if v4201 != 0.0 && (if (v4197 + v70) > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v4203 != 0.0 {
            } else {
            }
            let v4207 = if v4204 <= v4206 { 1.0 } else { 0.0 };
            let v4219: f64;
            if v4207 != 0.0 {
                v4219 = v4208;
            } else {
                let v4209 = v4204 + v4205;
                v4219 = v4209;
            }
            let v4212 = v4210 + v4211;
            let v4214: f64;
            if v4203 != 0.0 {
                v4214 = v4213;
            } else {
                v4214 = v0;
            }
            let v4215 = v4214 + v4212;
            let v4217 = v4216 * v4215;
            let v4218 = v1 / v4217;
            let v4220 = v4215 / v4219;
            let v4221 = v4215 - v4219;
            let v4222 = v4216 * v4219;
            let v4230 = v4223 - (((v4224 * v4215) * v4215) / (v4215 + v4227));
            let v4235 = v4223 - (((v4224 * v4219) * v4219) / (v4219 + v4227));
            let v4246 = (v4238 * (v4220 * (v4220.sqrt()))) * (rspice_limited_exp(((v4230 / (v63 * v4222)) - (v4230 / (v63 * v4217)))));
            let v4280: f64;
            if v4203 != 0.0 {
                let v4249 = (if (v1748 / v4246) >= v3982 { (v1748 / v4246) } else { v3982 }).ln();
                let v4252 = ((v4249 * v4249) + v114).sqrt();
                v4280 = v4252;
            } else {
                let v4255 = (if (v1748 / v4246) >= v3982 { (v1748 / v4246) } else { v3982 }).ln();
                v4280 = v4255;
            }
            let v9705: f64;
            if v4203 != 0.0 {
                let v4260 = (if ((v1316 * v206) / (v4246 * v4246)) >= v3982 { ((v1316 * v206) / (v4246 * v4246)) } else { v3982 }).ln();
                let v4263 = ((v4260 * v4260) + v114).sqrt();
                v9705 = v4263;
            } else {
                let v4268 = (if ((v1316 * v206) / (v4246 * v4246)) >= v3982 { ((v1316 * v206) / (v4246 * v4246)) } else { v3982 }).ln();
                v9705 = v4268;
            }
            let v4269 = if v236 > v0 { 1.0 } else { 0.0 };
            let v6718: f64;
            if v4269 != 0.0 {
                let v4278 = (((-v4270) * v4217) * ((if (v236 / v206) >= v3982 { (v236 / v206) } else { v3982 }).ln())) + v4277;
                v6718 = v4278;
            } else {
                v6718 = v0;
            }
            let v4284 = if ((v4279 + (v4217 * v4280)) + v376) >= v4279 { ((v4279 + (v4217 * v4280)) + v376) } else { v4279 };
            let v4285 = v4284.sqrt();
            let v4286 = v63 * v7;
            let v4290 = (v4286 / (v4287 * v1748)).sqrt();
            let v4294 = (((v7 / v9) * v10) * v366).sqrt();
            let v4296 = v4220 - v1;
            let v4298 = v1 + (v4295 * v4296);
            let v4300 = if v4298 < v4299 { 1.0 } else { 0.0 };
            let v4309: f64;
            if v4300 != 0.0 {
                let v4302 = v4301 / v4298;
                v4309 = v4302;
            } else {
                let v4308 = v1940 * (v4298 + (((v4298 * v4298) + v4304).sqrt()));
                v4309 = v4308;
            }
            let v4310 = v1770 * v4309;
            let v4313 = v1 + (v4311 * v4296);
            let v4314 = v1911 * v4313;
            let v5516: f64;
            if v1578 != 0.0 {
                let v4316 = v4315 * v4313;
                v5516 = v4316;
            } else {
                v5516 = v0;
            }
            let v4317 = if v2 != v1 { 1.0 } else { 0.0 };
            let v4322: f64;
            if v4317 != 0.0 {
                let v4320 = v4318 * v4319;
                v4322 = v4320;
            } else {
                let v4321 = v1940 * v4319;
                v4322 = v4321;
            }
            let v4324 = v4220.powf(v2264);
            let v4325 = v4323 * v4324;
            let v4328 = (v1 + (v2268 * v4221)) - v114;
            let v4330 = if v4328 < v4329 { 1.0 } else { 0.0 };
            let v4340: f64;
            if v4330 != 0.0 {
                let v4332 = v4331 / v4328;
                v4340 = v4332;
            } else {
                let v4338 = v1940 * (v4328 + (((v4328 * v4328) + v4334).sqrt()));
                v4340 = v4338;
            }
            let v4341 = v4339 * v4340;
            let v4344 = (v1 + (v876 * v4221)) - v114;
            let v4346 = if v4344 < v4345 { 1.0 } else { 0.0 };
            let v4355: f64;
            if v4346 != 0.0 {
                let v4348 = v4347 / v4344;
                v4355 = v4348;
            } else {
                let v4354 = v1940 * (v4344 + (((v4344 * v4344) + v4350).sqrt()));
                v4355 = v4354;
            }
            let v4356 = v1903 * v4355;
            let v4358 = v4220.powf(v2272);
            let v4359 = v4357 * v4358;
            let v4361 = v4220.powf(v906);
            let v4362 = v4360 * v4361;
            let v4364 = v1 + (v896 * v4296);
            let v4366 = if v4364 < v4365 { 1.0 } else { 0.0 };
            let v4376: f64;
            if v4366 != 0.0 {
                let v4368 = v4367 / v4364;
                v4376 = v4368;
            } else {
                let v4374 = v1940 * (v4364 + (((v4364 * v4364) + v4370).sqrt()));
                v4376 = v4374;
            }
            let v4377 = v4375 * v4376;
            let v5543: f64;
            let v5547: f64;
            let v5551: f64;
            let v5555: f64;
            let v5559: f64;
            if v1578 != 0.0 {
                let v4383 = v4378 * v4324;
                let v4385 = if v4328 < v4384 { 1.0 } else { 0.0 };
                let v4395: f64;
                if v4385 != 0.0 {
                    let v4387 = v4386 / v4328;
                    v4395 = v4387;
                } else {
                    let v4393 = v1940 * (v4328 + (((v4328 * v4328) + v4389).sqrt()));
                    v4395 = v4393;
                }
                let v4396 = v4394 * v4395;
                let v4398 = if v4344 < v4397 { 1.0 } else { 0.0 };
                let v4408: f64;
                if v4398 != 0.0 {
                    let v4400 = v4399 / v4344;
                    v4408 = v4400;
                } else {
                    let v4406 = v1940 * (v4344 + (((v4344 * v4344) + v4402).sqrt()));
                    v4408 = v4406;
                }
                let v4409 = v4407 * v4408;
                let v4411 = v4410 * v4358;
                let v4415 = v4412 * v4361;
                v5543 = v4383;
                v5547 = v4396;
                v5551 = v4409;
                v5555 = v4411;
                v5559 = v4415;
            } else {
                v5543 = v0;
                v5547 = v0;
                v5551 = v0;
                v5555 = v0;
                v5559 = v0;
            }
            let v4416 = v4220.powf(v916);
            let v4418 = v4220.powf((-v2276));
            let v4419 = v1985 * v4418;
            let v4421 = if v4419 < v4420 { 1.0 } else { 0.0 };
            let v5308: f64;
            if v4421 != 0.0 {
                v5308 = v4420;
            } else {
                v5308 = v4419;
            }
            let v4423 = if v4422 == v1 { 1.0 } else { 0.0 };
            let v6919: f64;
            let v7020: f64;
            if v4423 != 0.0 {
                let v4425 = v4220.powf(v4424);
                let v4430 = v4426 * (v4220.powf((-v4427)));
                v6919 = v4430;
                v7020 = v4425;
            } else {
                v6919 = v1;
                v7020 = v1;
            }
            let v5533: f64;
            if v1578 != 0.0 {
                let v4432 = v4431 * v4418;
                let v4433 = if v4432 < v4420 { 1.0 } else { 0.0 };
                let v5534: f64;
                if v4433 != 0.0 {
                    v5534 = v4420;
                } else {
                    v5534 = v4432;
                }
                v5533 = v5534;
            } else {
                v5533 = v0;
            }
            let v4434 = v2197 * v4418;
            let v4435 = if v4434 < v4420 { 1.0 } else { 0.0 };
            let v9133: f64;
            if v4435 != 0.0 {
                v9133 = v4420;
            } else {
                v9133 = v4434;
            }
            let v4441 = ((v1 / v1941) * (v1 + (v4437 * v4221))) - v63;
            let v4443 = if v4441 < v4442 { 1.0 } else { 0.0 };
            let v4452: f64;
            if v4443 != 0.0 {
                let v4445 = v4444 / v4441;
                v4452 = v4445;
            } else {
                let v4451 = v1940 * (v4441 + (((v4441 * v4441) + v4447).sqrt()));
                v4452 = v4451;
            }
            let v4454 = v1 / (v4452 + v63);
            let v4457 = (v1 - (v2280 * v4221)) - v114;
            let v4459 = if v4457 < v4458 { 1.0 } else { 0.0 };
            let v4468: f64;
            if v4459 != 0.0 {
                let v4461 = v4460 / v4457;
                v4468 = v4461;
            } else {
                let v4467 = v1940 * (v4457 + (((v4457 * v4457) + v4463).sqrt()));
                v4468 = v4467;
            }
            let v4469 = v2010 * v4468;
            let v5539: f64;
            if v1578 != 0.0 {
                let v4471 = if v4457 < v4470 { 1.0 } else { 0.0 };
                let v4481: f64;
                if v4471 != 0.0 {
                    let v4473 = v4472 / v4457;
                    v4481 = v4473;
                } else {
                    let v4479 = v1940 * (v4457 + (((v4457 * v4457) + v4475).sqrt()));
                    v4481 = v4479;
                }
                let v4482 = v4480 * v4481;
                v5539 = v4482;
            } else {
                v5539 = v0;
            }
            let v4485 = (v1 + (v1236 * v4221)) - v114;
            let v4487 = if v4485 < v4486 { 1.0 } else { 0.0 };
            let v4496: f64;
            if v4487 != 0.0 {
                let v4489 = v4488 / v4485;
                v4496 = v4489;
            } else {
                let v4495 = v1940 * (v4485 + (((v4485 * v4485) + v4491).sqrt()));
                v4496 = v4495;
            }
            let v4497 = v1226 * v4496;
            let v4500 = (v1 + (v1256 * v4221)) - v114;
            let v4502 = if v4500 < v4501 { 1.0 } else { 0.0 };
            let v4511: f64;
            if v4502 != 0.0 {
                let v4504 = v4503 / v4500;
                v4511 = v4504;
            } else {
                let v4510 = v1940 * (v4500 + (((v4500 * v4500) + v4506).sqrt()));
                v4511 = v4510;
            }
            let v4512 = v1246 * v4511;
            let v4513 = v4220.powf(v946);
            let v4514 = v2040 * v4513;
            let v5567: f64;
            if v1578 != 0.0 {
                let v4516 = v4515 * v4513;
                v5567 = v4516;
            } else {
                v5567 = v0;
            }
            let v4519 = (v1 + (v956 * v4221)) - v114;
            let v4521 = if v4519 < v4520 { 1.0 } else { 0.0 };
            let v4530: f64;
            if v4521 != 0.0 {
                let v4523 = v4522 / v4519;
                v4530 = v4523;
            } else {
                let v4529 = v1940 * (v4519 + (((v4519 * v4519) + v4525).sqrt()));
                v4530 = v4529;
            }
            let v4531 = v786 * v4530;
            let v4533 = if v4519 < v4532 { 1.0 } else { 0.0 };
            let v4542: f64;
            if v4533 != 0.0 {
                let v4535 = v4534 / v4519;
                v4542 = v4535;
            } else {
                let v4541 = v1940 * (v4519 + (((v4519 * v4519) + v4537).sqrt()));
                v4542 = v4541;
            }
            let v4543 = v826 * v4542;
            let v4545 = (if v4220 >= v3982 { v4220 } else { v3982 }).ln();
            let v4547 = rspice_limited_exp((v966 * v4545));
            let v4550 = (v1 + (v1286 * v4221)) - v114;
            let v4552 = if v4550 < v4551 { 1.0 } else { 0.0 };
            let v4561: f64;
            if v4552 != 0.0 {
                let v4554 = v4553 / v4550;
                v4561 = v4554;
            } else {
                let v4560 = v1940 * (v4550 + (((v4550 * v4550) + v4556).sqrt()));
                v4561 = v4560;
            }
            let v4562 = v1266 * v4561;
            let v4565 = (v1 + (v1296 * v4221)) - v114;
            let v4567 = if v4565 < v4566 { 1.0 } else { 0.0 };
            let v4577: f64;
            if v4567 != 0.0 {
                let v4569 = v4568 / v4565;
                v4577 = v4569;
            } else {
                let v4575 = v1940 * (v4565 + (((v4565 * v4565) + v4571).sqrt()));
                v4577 = v4575;
            }
            let v4578 = v4576 * v4577;
            let v4581 = (v1 + (v1506 * v4221)) - v114;
            let v4583 = if v4581 < v4582 { 1.0 } else { 0.0 };
            let v4592: f64;
            if v4583 != 0.0 {
                let v4585 = v4584 / v4581;
                v4592 = v4585;
            } else {
                let v4591 = v1940 * (v4581 + (((v4581 * v4581) + v4587).sqrt()));
                v4592 = v4591;
            }
            let v4593 = v1496 * v4592;
            let v4596 = (v1 + (v1526 * v4221)) - v114;
            let v4598 = if v4596 < v4597 { 1.0 } else { 0.0 };
            let v4607: f64;
            if v4598 != 0.0 {
                let v4600 = v4599 / v4596;
                v4607 = v4600;
            } else {
                let v4606 = v1940 * (v4596 + (((v4596 * v4596) + v4602).sqrt()));
                v4607 = v4606;
            }
            let v4608 = v1516 * v4607;
            let v4611 = (v1 + (v1546 * v4221)) - v114;
            let v4613 = if v4611 < v4612 { 1.0 } else { 0.0 };
            let v4622: f64;
            if v4613 != 0.0 {
                let v4615 = v4614 / v4611;
                v4622 = v4615;
            } else {
                let v4621 = v1940 * (v4611 + (((v4611 * v4611) + v4617).sqrt()));
                v4622 = v4621;
            }
            let v4623 = v1536 * v4622;
            let v4627 = (v1 + (v4624 * v4221)) - v114;
            let v4629 = if v4627 < v4628 { 1.0 } else { 0.0 };
            let v4639: f64;
            if v4629 != 0.0 {
                let v4631 = v4630 / v4627;
                v4639 = v4631;
            } else {
                let v4637 = v1940 * (v4627 + (((v4627 * v4627) + v4633).sqrt()));
                v4639 = v4637;
            }
            let v4640 = v4638 * v4639;
            let v4642 = if v4627 < v4641 { 1.0 } else { 0.0 };
            let v4652: f64;
            if v4642 != 0.0 {
                let v4644 = v4643 / v4627;
                v4652 = v4644;
            } else {
                let v4650 = v1940 * (v4627 + (((v4627 * v4627) + v4646).sqrt()));
                v4652 = v4650;
            }
            let v4653 = v4651 * v4652;
            let v4657 = (v1 + (v4654 * v4221)) - v114;
            let v4659 = if v4657 < v4658 { 1.0 } else { 0.0 };
            let v4669: f64;
            if v4659 != 0.0 {
                let v4661 = v4660 / v4657;
                v4669 = v4661;
            } else {
                let v4667 = v1940 * (v4657 + (((v4657 * v4657) + v4663).sqrt()));
                v4669 = v4667;
            }
            let v4670 = v4668 * v4669;
            let v4672 = if v4657 < v4671 { 1.0 } else { 0.0 };
            let v4682: f64;
            if v4672 != 0.0 {
                let v4674 = v4673 / v4657;
                v4682 = v4674;
            } else {
                let v4680 = v1940 * (v4657 + (((v4657 * v4657) + v4676).sqrt()));
                v4682 = v4680;
            }
            let v4683 = v4681 * v4682;
            let v4687 = (v1 + (v4684 * v4221)) - v114;
            let v4689 = if v4687 < v4688 { 1.0 } else { 0.0 };
            let v4699: f64;
            if v4689 != 0.0 {
                let v4691 = v4690 / v4687;
                v4699 = v4691;
            } else {
                let v4697 = v1940 * (v4687 + (((v4687 * v4687) + v4693).sqrt()));
                v4699 = v4697;
            }
            let v4700 = v4698 * v4699;
            let v4702 = if v4687 < v4701 { 1.0 } else { 0.0 };
            let v4712: f64;
            if v4702 != 0.0 {
                let v4704 = v4703 / v4687;
                v4712 = v4704;
            } else {
                let v4710 = v1940 * (v4687 + (((v4687 * v4687) + v4706).sqrt()));
                v4712 = v4710;
            }
            let v4713 = v4711 * v4712;
            let v4716 = v4715 * v4221;
            let v4719 = (v4714 - v4716) - v4718;
            let v4721 = if v4719 < v4720 { 1.0 } else { 0.0 };
            let v4730: f64;
            if v4721 != 0.0 {
                let v4723 = v4722 / v4719;
                v4730 = v4723;
            } else {
                let v4729 = v1940 * (v4719 + (((v4719 * v4719) + v4725).sqrt()));
                v4730 = v4729;
            }
            let v4731 = v4730 + v4718;
            let v4734 = (v4732 - v4716) - v4718;
            let v4736 = if v4734 < v4735 { 1.0 } else { 0.0 };
            let v4745: f64;
            if v4736 != 0.0 {
                let v4738 = v4737 / v4734;
                v4745 = v4738;
            } else {
                let v4744 = v1940 * (v4734 + (((v4734 * v4734) + v4740).sqrt()));
                v4745 = v4744;
            }
            let v4746 = v4745 + v4718;
            let v4749 = v4748 * v4221;
            let v4751 = (v4747 - v4749) - v4718;
            let v4753 = if v4751 < v4752 { 1.0 } else { 0.0 };
            let v4762: f64;
            if v4753 != 0.0 {
                let v4755 = v4754 / v4751;
                v4762 = v4755;
            } else {
                let v4761 = v1940 * (v4751 + (((v4751 * v4751) + v4757).sqrt()));
                v4762 = v4761;
            }
            let v4763 = v4762 + v4718;
            let v4766 = (v4764 - v4749) - v4718;
            let v4768 = if v4766 < v4767 { 1.0 } else { 0.0 };
            let v4777: f64;
            if v4768 != 0.0 {
                let v4770 = v4769 / v4766;
                v4777 = v4770;
            } else {
                let v4776 = v1940 * (v4766 + (((v4766 * v4766) + v4772).sqrt()));
                v4777 = v4776;
            }
            let v4778 = v4777 + v4718;
            let v4781 = v4780 * v4221;
            let v4783 = (v4779 - v4781) - v4718;
            let v4785 = if v4783 < v4784 { 1.0 } else { 0.0 };
            let v4794: f64;
            if v4785 != 0.0 {
                let v4787 = v4786 / v4783;
                v4794 = v4787;
            } else {
                let v4793 = v1940 * (v4783 + (((v4783 * v4783) + v4789).sqrt()));
                v4794 = v4793;
            }
            let v4795 = v4794 + v4718;
            let v4798 = (v4796 - v4781) - v4718;
            let v4800 = if v4798 < v4799 { 1.0 } else { 0.0 };
            let v4809: f64;
            if v4800 != 0.0 {
                let v4802 = v4801 / v4798;
                v4809 = v4802;
            } else {
                let v4808 = v1940 * (v4798 + (((v4798 * v4798) + v4804).sqrt()));
                v4809 = v4808;
            }
            let v4810 = v4809 + v4718;
            let v4813 = (v4235 / v4222) - (v4230 / v4217);
            let v4819 = rspice_limited_exp(((v4813 + (v4814 * v4545)) / v4817));
            let v4821 = v4820 * v4819;
            let v4823 = v4822 * v4819;
            let v4825 = v4824 * v4819;
            let v4831 = rspice_limited_exp(((v4813 + (v4826 * v4545)) / v4829));
            let v4833 = v4832 * v4831;
            let v4835 = v4834 * v4831;
            let v4837 = v4836 * v4831;
            let v4844 = v4838 * (rspice_limited_exp((((v4235 * v4839) * v4296) / v4217)));
            let v4851 = v4845 * (rspice_limited_exp((((v4235 * v4846) * v4296) / v4217)));
            let v4856 = ((v4853 / v112).sqrt()) + v1;
            let v4863 = (v4852 * v4856) * (rspice_limited_exp((((v4235 * v4858) * v4296) / v4217)));
            let v4870 = v4864 * (rspice_limited_exp((((v4235 * v4865) * v4296) / v4217)));
            let v4877 = v4871 * (rspice_limited_exp((((v4235 * v4872) * v4296) / v4217)));
            let v4885 = (v4878 * v4856) * (rspice_limited_exp((((v4235 * v4880) * v4296) / v4217)));
            let v4893 = if ((v4886 * (v1 + (v4887 * v4296))) - v4718) < v4892 { 1.0 } else { 0.0 };
            if v4893 != 0.0 {
            } else {
            }
            let v4901 = if ((v4894 * (v1 + (v4895 * v4296))) - v4718) < v4900 { 1.0 } else { 0.0 };
            if v4901 != 0.0 {
            } else {
            }
            let v4909 = if ((v4902 * (v1 + (v4903 * v4296))) - v4718) < v4908 { 1.0 } else { 0.0 };
            if v4909 != 0.0 {
            } else {
            }
            let v4917 = if ((v4910 * (v1 + (v4911 * v4296))) - v4718) < v4916 { 1.0 } else { 0.0 };
            if v4917 != 0.0 {
            } else {
            }
            let v4925 = if ((v4918 * (v1 + (v4919 * v4296))) - v4718) < v4924 { 1.0 } else { 0.0 };
            if v4925 != 0.0 {
            } else {
            }
            let v4933 = if ((v4926 * (v1 + (v4927 * v4296))) - v4718) < v4932 { 1.0 } else { 0.0 };
            if v4933 != 0.0 {
            } else {
            }
            let v4934 = if v2379 < v2380 { 1.0 } else { 0.0 };
            let v4959: f64;
            let v4965: f64;
            let v4976: f64;
            let v4982: f64;
            if v4934 != 0.0 {
                let v4936 = if (v24 % v63) != v0 { 1.0 } else { 0.0 };
                let v4960: f64;
                let v4966: f64;
                let v4977: f64;
                let v4983: f64;
                if v4936 != 0.0 {
                    let v4940 = v63 * (if ((v24 - v1) / v63) >= v0 { ((v24 - v1) / v63) } else { v0 });
                    v4960 = v1;
                    v4966 = v4940;
                    v4977 = v1;
                    v4983 = v4940;
                } else {
                    let v4941 = if v2388 == v1 { 1.0 } else { 0.0 };
                    let v4961: f64;
                    let v4967: f64;
                    let v4978: f64;
                    let v4984: f64;
                    if v4941 != 0.0 {
                        let v4945 = v63 * (if ((v24 / v63) - v1) >= v0 { ((v24 / v63) - v1) } else { v0 });
                        v4961 = v0;
                        v4967 = v24;
                        v4978 = v63;
                        v4984 = v4945;
                    } else {
                        let v4949 = v63 * (if ((v24 / v63) - v1) >= v0 { ((v24 / v63) - v1) } else { v0 });
                        v4961 = v63;
                        v4967 = v4949;
                        v4978 = v0;
                        v4984 = v24;
                    }
                    v4960 = v4961;
                    v4966 = v4967;
                    v4977 = v4978;
                    v4983 = v4984;
                }
                v4959 = v4960;
                v4965 = v4966;
                v4976 = v4977;
                v4982 = v4983;
            } else {
                v4959 = v4962;
                v4965 = v4968;
                v4976 = v4979;
                v4982 = v4985;
            }
            let v4950 = v2367 + v2368;
            let v4951 = v2367 + v2367;
            let v4952 = v2370 + v2370;
            let v4954 = (v4950 + v4950) + v112;
            let v4955 = v4950 * v112;
            let v4956 = v2367 * v112;
            let v4957 = v2370 * v112;
            let v4958 = if v2379 == v0 { 1.0 } else { 0.0 };
            let v5105: f64;
            let v5122: f64;
            let v5144: f64;
            let v5164: f64;
            if v4958 != 0.0 {
                let v4975 = (v4959 * v4954) + (v4965 * v4951);
                let v4992 = (v4976 * v4954) + (v4982 * v4951);
                let v4995 = (v4959 * v4955) + (v4965 * v4956);
                let v4998 = (v4976 * v4955) + (v4982 * v4956);
                v5105 = v4995;
                v5122 = v4998;
                v5144 = v4975;
                v5164 = v4992;
            } else {
                let v4999 = if v2379 == v1 { 1.0 } else { 0.0 };
                let v5106: f64;
                let v5123: f64;
                let v5145: f64;
                let v5165: f64;
                if v4999 != 0.0 {
                    let v5002 = (v4959 * v4954) + (v4965 * v4951);
                    let v5003 = v4976 + v4982;
                    let v5004 = v5003 * v4951;
                    let v5007 = (v4959 * v4955) + (v4965 * v4956);
                    let v5008 = v5003 * v4956;
                    v5106 = v5007;
                    v5123 = v5008;
                    v5145 = v5002;
                    v5165 = v5004;
                } else {
                    let v5009 = if v2379 == v63 { 1.0 } else { 0.0 };
                    let v5107: f64;
                    let v5124: f64;
                    let v5146: f64;
                    let v5166: f64;
                    if v5009 != 0.0 {
                        let v5010 = v4959 + v4965;
                        let v5011 = v5010 * v4951;
                        let v5014 = (v4976 * v4954) + (v4982 * v4951);
                        let v5015 = v5010 * v4956;
                        let v5018 = (v4976 * v4955) + (v4982 * v4956);
                        v5107 = v5015;
                        v5124 = v5018;
                        v5146 = v5011;
                        v5166 = v5014;
                    } else {
                        let v5019 = if v2379 == v2427 { 1.0 } else { 0.0 };
                        let v5108: f64;
                        let v5125: f64;
                        let v5147: f64;
                        let v5167: f64;
                        if v5019 != 0.0 {
                            let v5020 = v4959 + v4965;
                            let v5021 = v5020 * v4951;
                            let v5022 = v4976 + v4982;
                            let v5023 = v5022 * v4951;
                            let v5024 = v5020 * v4956;
                            let v5025 = v5022 * v4956;
                            v5108 = v5024;
                            v5125 = v5025;
                            v5147 = v5021;
                            v5167 = v5023;
                        } else {
                            let v5026 = if v2379 == v2429 { 1.0 } else { 0.0 };
                            let v5109: f64;
                            let v5126: f64;
                            let v5148: f64;
                            let v5168: f64;
                            if v5026 != 0.0 {
                                let v5029 = (v4959 * v4954) + (v4965 * v4951);
                                let v5032 = (v4976 * v4952) + (v4982 * v4951);
                                let v5035 = (v4959 * v4955) + (v4965 * v4956);
                                let v5038 = (v4976 * v4957) + (v4982 * v4956);
                                v5109 = v5035;
                                v5126 = v5038;
                                v5148 = v5029;
                                v5168 = v5032;
                            } else {
                                let v5039 = if v2379 == v2417 { 1.0 } else { 0.0 };
                                let v5110: f64;
                                let v5127: f64;
                                let v5149: f64;
                                let v5169: f64;
                                if v5039 != 0.0 {
                                    let v5040 = v4959 + v4965;
                                    let v5041 = v5040 * v4951;
                                    let v5044 = (v4976 * v4952) + (v4982 * v4951);
                                    let v5045 = v5040 * v4956;
                                    let v5048 = (v4976 * v4957) + (v4982 * v4956);
                                    v5110 = v5045;
                                    v5127 = v5048;
                                    v5149 = v5041;
                                    v5169 = v5044;
                                } else {
                                    let v5049 = if v2379 == v2432 { 1.0 } else { 0.0 };
                                    let v5111: f64;
                                    let v5128: f64;
                                    let v5150: f64;
                                    let v5170: f64;
                                    if v5049 != 0.0 {
                                        let v5052 = (v4959 * v4952) + (v4965 * v4951);
                                        let v5055 = (v4976 * v4954) + (v4982 * v4951);
                                        let v5058 = (v4959 * v4957) + (v4965 * v4956);
                                        let v5061 = (v4976 * v4955) + (v4982 * v4956);
                                        v5111 = v5058;
                                        v5128 = v5061;
                                        v5150 = v5052;
                                        v5170 = v5055;
                                    } else {
                                        let v5062 = if v2379 == v2446 { 1.0 } else { 0.0 };
                                        let v5112: f64;
                                        let v5129: f64;
                                        let v5151: f64;
                                        let v5171: f64;
                                        if v5062 != 0.0 {
                                            let v5065 = (v4959 * v4952) + (v4965 * v4951);
                                            let v5066 = v4976 + v4982;
                                            let v5067 = v5066 * v4951;
                                            let v5070 = (v4959 * v4957) + (v4965 * v4956);
                                            let v5071 = v5066 * v4956;
                                            v5112 = v5070;
                                            v5129 = v5071;
                                            v5151 = v5065;
                                            v5171 = v5067;
                                        } else {
                                            let v5072 = if v2379 == v2456 { 1.0 } else { 0.0 };
                                            let v5113: f64;
                                            let v5130: f64;
                                            let v5152: f64;
                                            let v5172: f64;
                                            if v5072 != 0.0 {
                                                let v5075 = (v4959 * v4952) + (v4965 * v4951);
                                                let v5078 = (v4976 * v4952) + (v4982 * v4951);
                                                let v5081 = (v4959 * v4957) + (v4965 * v4956);
                                                let v5084 = (v4976 * v4957) + (v4982 * v4956);
                                                v5113 = v5081;
                                                v5130 = v5084;
                                                v5152 = v5075;
                                                v5172 = v5078;
                                            } else {
                                                let v5085 = if v2379 == v2380 { 1.0 } else { 0.0 };
                                                let v5114: f64;
                                                let v5131: f64;
                                                let v5153: f64;
                                                let v5173: f64;
                                                if v5085 != 0.0 {
                                                    let v5086 = v24 - v1;
                                                    let v5088 = v4954 + (v5086 * v4951);
                                                    let v5089 = v24 * v4951;
                                                    let v5091 = v4955 + (v5086 * v4956);
                                                    let v5092 = v24 * v4956;
                                                    v5114 = v5091;
                                                    v5131 = v5092;
                                                    v5153 = v5088;
                                                    v5173 = v5089;
                                                } else {
                                                    let v5093 = if v2379 == v2995 { 1.0 } else { 0.0 };
                                                    let v5115: f64;
                                                    let v5132: f64;
                                                    let v5154: f64;
                                                    let v5174: f64;
                                                    if v5093 != 0.0 {
                                                        let v5094 = v24 * v4951;
                                                        let v5095 = v24 - v1;
                                                        let v5097 = v4954 + (v5095 * v4951);
                                                        let v5098 = v24 * v4956;
                                                        let v5100 = v4955 + (v5095 * v4956);
                                                        v5115 = v5098;
                                                        v5132 = v5100;
                                                        v5154 = v5094;
                                                        v5174 = v5097;
                                                    } else {
                                                        v5115 = v0;
                                                        v5132 = v0;
                                                        v5154 = v0;
                                                        v5174 = v0;
                                                    }
                                                    v5114 = v5115;
                                                    v5131 = v5132;
                                                    v5153 = v5154;
                                                    v5173 = v5174;
                                                }
                                                v5113 = v5114;
                                                v5130 = v5131;
                                                v5152 = v5153;
                                                v5172 = v5173;
                                            }
                                            v5112 = v5113;
                                            v5129 = v5130;
                                            v5151 = v5152;
                                            v5171 = v5172;
                                        }
                                        v5111 = v5112;
                                        v5128 = v5129;
                                        v5150 = v5151;
                                        v5170 = v5171;
                                    }
                                    v5110 = v5111;
                                    v5127 = v5128;
                                    v5149 = v5150;
                                    v5169 = v5170;
                                }
                                v5109 = v5110;
                                v5126 = v5127;
                                v5148 = v5149;
                                v5168 = v5169;
                            }
                            v5108 = v5109;
                            v5125 = v5126;
                            v5147 = v5148;
                            v5167 = v5168;
                        }
                        v5107 = v5108;
                        v5124 = v5125;
                        v5146 = v5147;
                        v5166 = v5167;
                    }
                    v5106 = v5107;
                    v5123 = v5124;
                    v5145 = v5146;
                    v5165 = v5166;
                }
                v5105 = v5106;
                v5122 = v5123;
                v5144 = v5145;
                v5164 = v5165;
            }
            let v5116: f64;
            if v5101 != 0.0 {
                let v5104 = (v5102 * v19) * v16;
                v5116 = v5104;
            } else {
                v5116 = v5105;
            }
            let v5117 = if v5116 < v0 { 1.0 } else { 0.0 };
            let v5176: f64;
            if v5117 != 0.0 {
                v5176 = v0;
            } else {
                v5176 = v5116;
            }
            let v5133: f64;
            if v5118 != 0.0 {
                let v5121 = (v5119 * v19) * v16;
                v5133 = v5121;
            } else {
                v5133 = v5122;
            }
            let v5134 = if v5133 < v0 { 1.0 } else { 0.0 };
            let v5192: f64;
            if v5134 != 0.0 {
                v5192 = v0;
            } else {
                v5192 = v5133;
            }
            let v5178: f64;
            if v5135 != 0.0 {
                let v5137 = if v5136 == v0 { 1.0 } else { 0.0 };
                let v5179: f64;
                if v5137 != 0.0 {
                    let v5139 = v5138 * v19;
                    v5179 = v5139;
                } else {
                    let v5143 = if ((v5138 * v19) - (v112 * v24)) >= v0 { ((v5138 * v19) - (v112 * v24)) } else { v0 };
                    v5179 = v5143;
                }
                v5178 = v5179;
            } else {
                let v5155 = if v5144 < v0 { 1.0 } else { 0.0 };
                let v5180: f64;
                if v5155 != 0.0 {
                    v5180 = v0;
                } else {
                    v5180 = v5144;
                }
                v5178 = v5180;
            }
            let v5194: f64;
            if v5156 != 0.0 {
                let v5157 = if v5136 == v0 { 1.0 } else { 0.0 };
                let v5195: f64;
                if v5157 != 0.0 {
                    let v5159 = v5158 * v19;
                    v5195 = v5159;
                } else {
                    let v5163 = if ((v5158 * v19) - (v112 * v24)) >= v0 { ((v5158 * v19) - (v112 * v24)) } else { v0 };
                    v5195 = v5163;
                }
                v5194 = v5195;
            } else {
                let v5175 = if v5164 < v0 { 1.0 } else { 0.0 };
                let v5196: f64;
                if v5175 != 0.0 {
                    v5196 = v0;
                } else {
                    v5196 = v5164;
                }
                v5194 = v5196;
            }
            let v5183 = v112 * v24;
            let v5185 = ((v5176 * v4821) + (v5178 * v4823)) + (v5183 * v4825);
            let v5186 = if v5185 > v0 { 1.0 } else { 0.0 };
            if v5186 != 0.0 {
                let v5191 = if ((v5187 / v5185) - v2995) < v5190 { 1.0 } else { 0.0 };
                if v5191 != 0.0 {
                } else {
                }
            } else {
            }
            let v5200 = ((v5192 * v4833) + (v5194 * v4835)) + (v5183 * v4837);
            let v5201 = if v5200 > v0 { 1.0 } else { 0.0 };
            if v5201 != 0.0 {
                let v5206 = if ((v5202 / v5200) - v2995) < v5205 { 1.0 } else { 0.0 };
                if v5206 != 0.0 {
                } else {
                }
            } else {
            }
            let v5218 = if (if (if v5207 > v0 { 1.0 } else { 0.0 }) != 0.0 && (if v5209 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if (if v24 == v1 { 1.0 } else { 0.0 }) != 0.0 || (if (if v24 > v1 { 1.0 } else { 0.0 }) != 0.0 && (if v5214 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5407: f64;
            let v5409: f64;
            let v5518: f64;
            let v5536: f64;
            let v5677: f64;
            let v9622: f64;
            let v9712: f64;
            let v9715: f64;
            if v5218 != 0.0 {
                let v5220 = v22.powf(v5219);
                let v5222 = v27 + v5221;
                let v5224 = v5222.powf(v5223);
                let v5236 = v22.powf(v5235);
                let v5238 = v5222.powf(v5237);
                let v5248 = v1 + (((v5239 / v5236) + (v5241 / v5238)) + (v5244 / (v5236 * v5238)));
                let v5253 = ((v1 + (((v5225 / v5220) + (v5227 / v5224)) + (v5230 / (v5220 * v5224)))) * (v1 + (v5249 * v4296))) + v67;
                let mut v5254: f64 = 0.0;
                let mut v5266: f64 = 0.0;
                let mut v5268: f64 = 0.0;
                v5254 = v0;
                v5266 = v0;
                v5268 = v0;
                loop {
                    let v5255 = if v5254 < v24 { 1.0 } else { 0.0 };
                    if v5255 == 0.0 {
                        break;
                    }
                    let v5256 = v1 / v24;
                    let v5257 = v1940 * v17;
                    let v5260 = v5254 * (v5214 + v17);
                    let v5267 = v5266 + (v5256 / ((v5207 + v5257) + v5260));
                    let v5269 = v5268 + (v5256 / ((v5209 + v5257) + v5260));
                    let v5270 = v5254 + v1;
                    v5254 = v5270;
                    v5266 = v5267;
                    v5268 = v5269;
                }
                let v5272 = v1940 * v17;
                let v5278 = (v1 / (v5271 + v5272)) + (v1 / (v5275 + v5272));
                let v5280 = v5279 / v5253;
                let v5281 = v5280 * v5278;
                let v5282 = v5266 + v5268;
                let v5283 = v5280 * v5282;
                let v5295 = v5282 - v5278;
                let v5296 = (v5293 / v5248) * v5295;
                let v5299 = v5248.powf(v5298);
                let v5304 = v5248.powf(v5303);
                let v5307 = v4325 * ((v1 + v5283) / (v1 + v5281));
                let v5309 = v5308 * ((v1 + (v5283 * v5287)) / (v1 + (v5281 * v5287)));
                let v5310 = v2251 + ((v5297 / v5299) * v5295);
                let v5311 = v4314 + ((v5302 / v5304) * v5295);
                let v5313 = if v5312 == v1 { 1.0 } else { 0.0 };
                let v5320: f64;
                let v5322: f64;
                let v9713: f64;
                if v5313 != 0.0 {
                    let v5315 = (v1446 / v5248) * v5295;
                    let v5317 = (v1476 / v5299) * v5295;
                    let v5319 = (v1486 / v5304) * v5295;
                    v5320 = v5317;
                    v5322 = v5319;
                    v9713 = v5315;
                } else {
                    v5320 = v0;
                    v5322 = v0;
                    v9713 = v0;
                }
                let v5321 = v1436 + v5320;
                let v5323 = v1356 + v5322;
                v5407 = v5307;
                v5409 = v5310;
                v5518 = v5311;
                v5536 = v5309;
                v5677 = v5296;
                v9622 = v5323;
                v9712 = v9713;
                v9715 = v5321;
            } else {
                v5407 = v4325;
                v5409 = v2251;
                v5518 = v4314;
                v5536 = v5308;
                v5677 = v0;
                v9622 = v1356;
                v9712 = v0;
                v9715 = v1436;
            }
            let v5325 = if v5324 == v1 { 1.0 } else { 0.0 };
            let v5386: f64;
            let v5390: f64;
            let v5396: f64;
            if v5325 != 0.0 {
                let v5326 = v18 / v24;
                let v5337 = if (if (if v5330 == 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v5332 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 && (if v5335 == 0.0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v5387: f64;
                let v5391: f64;
                let v5397: f64;
                if v5337 != 0.0 {
                    let v5341 = if v5338 != 0.0 && (if v5339 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v5388: f64;
                    let v5392: f64;
                    let v5398: f64;
                    if v5341 != 0.0 {
                        let v5342 = v5339 + v5326;
                        let v5344 = v1 / v5343;
                        let v5347 = (v5343 * v5343) / (v5339 * v5342);
                        let v5350 = v4718 * v5343;
                        let v5365 = ((((v5348 * v5339) + v5350) * (rspice_limited_exp(((v5352 * v5339) * v5344)))) - (((v5348 * v5342) + v5350) * (rspice_limited_exp(((v5359 * v5342) * v5344))))) / v5326;
                        let v5369 = v5368 * v5343;
                        let v5385 = ((((v5366 * v5339) + v5369) * (rspice_limited_exp(((v5372 * v5339) * v5344)))) - (((v5366 * v5342) + v5369) * (rspice_limited_exp(((v5379 * v5342) * v5344))))) / v5326;
                        v5388 = v5347;
                        v5392 = v5365;
                        v5398 = v5385;
                    } else {
                        v5388 = v5327;
                        v5392 = v5328;
                        v5398 = v5329;
                    }
                    v5387 = v5388;
                    v5391 = v5392;
                    v5397 = v5398;
                } else {
                    v5387 = v5327;
                    v5391 = v5328;
                    v5397 = v5329;
                }
                v5386 = v5387;
                v5390 = v5391;
                v5396 = v5397;
            } else {
                v5386 = v0;
                v5390 = v0;
                v5396 = v0;
            }
            let v5400 = (v5386 + (v5389 * v5390)) + (v5395 * v5396);
            let v5401 = v746 * v5400;
            let v5403 = v1466 * v5400;
            let v5404 = v1456 * v5400;
            let v5408 = v5407 * (v1 + (v766 * v5400));
            let v5410 = v5409 + (v756 * v5400);
            let v5414 = v4270 * (v5411 - v5412);
            let v5417 = v4270 * (v5415 - v5412);
            let v5420 = v4270 * (v5418 - v5412);
            let v5421 = v5417 - v5420;
            let v5424 = v4270 * (v5422 - v5418);
            let v5427 = v4270 * (v5425 - v5415);
            let v5430 = v4270 * (v5425 - v5428);
            let v5431 = v5414 - v5417;
            let v5432 = v5414 - v5420;
            let v5435 = v4270 * (v5433 - v5415);
            let v5436 = v5433 - v5418;
            let v5438 = if v5437 != v0 { 1.0 } else { 0.0 };
            let v5441 = if v5440 == v1 { 1.0 } else { 0.0 };
            let v5443 = if (if (if v5438 != 0.0 && v2288 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v5441 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v4423 != 0.0 { 1.0 } else { 0.0 };
            let v5456: f64;
            let v7310: f64;
            let v8217: f64;
            if v5443 != 0.0 {
                let v5451 = v5417 + ((v4270 * (v1 - (v5444 / v5437))) * (v5448 - v5415));
                let v5453 = (v5427 + v5417) - v5451;
                let v5455 = (v5435 + v5417) - v5451;
                v5456 = v5451;
                v7310 = v5455;
                v8217 = v5453;
            } else {
                v5456 = v5417;
                v7310 = v5435;
                v8217 = v5427;
            }
            let v5457 = if v5421 < v0 { 1.0 } else { 0.0 };
            let v5459: f64;
            let v5460: f64;
            let v5462: f64;
            let v5463: f64;
            let v7156: f64;
            if v5457 != 0.0 {
                v5459 = v5420;
                v5460 = v5417;
                v5462 = v5420;
                v5463 = v5456;
                v7156 = v5458;
            } else {
                v5459 = v5417;
                v5460 = v5420;
                v5462 = v5456;
                v5463 = v5420;
                v7156 = v1;
            }
            let v5461 = v5459 - v5460;
            let v5464 = v5462 - v5463;
            let v5466 = v5465 * v5464;
            let v5468 = if v5466 > v5467 { 1.0 } else { 0.0 };
            let v5475: f64;
            if v5468 != 0.0 {
                v5475 = v5466;
            } else {
                let v5470 = if v5466 < v5469 { 1.0 } else { 0.0 };
                let v5476: f64;
                if v5470 != 0.0 {
                    let v5471 = v5466.exp();
                    v5476 = v5471;
                } else {
                    let v5474 = (v1 + (v5466.exp())).ln();
                    v5476 = v5474;
                }
                v5475 = v5476;
            }
            let v5477 = v63 / v5465;
            let v5486 = -(v5463 + (v1940 * (v5464 - (((v5477 * v5475) - v5464) - (v5477 * v5480)))));
            let v5487 = v5465 * v5461;
            let v5488 = if v5487 > v5467 { 1.0 } else { 0.0 };
            let v5495: f64;
            if v5488 != 0.0 {
                v5495 = v5487;
            } else {
                let v5490 = if v5487 < v5489 { 1.0 } else { 0.0 };
                let v5496: f64;
                if v5490 != 0.0 {
                    let v5491 = v5487.exp();
                    v5496 = v5491;
                } else {
                    let v5494 = (v1 + (v5487.exp())).ln();
                    v5496 = v5494;
                }
                v5495 = v5496;
            }
            let v5501 = ((v5477 * v5495) - v5461) - (v5477 * v5499);
            let v5505 = -(v5460 + (v1940 * (v5461 - v5501)));
            let v5511 = v1940 + (v1940 * (((v5506 * v5421) / v4217).tanh()));
            let v5512 = v1 - v5511;
            let v5593: f64;
            let v5618: f64;
            let v5955: f64;
            let v5957: f64;
            let v5958: f64;
            let v5963: f64;
            let v6018: f64;
            let v6038: f64;
            let v6040: f64;
            let v6043: f64;
            let v6571: f64;
            let v6606: f64;
            let v7681: f64;
            let v7683: f64;
            if v1578 != 0.0 {
                let v5515 = (v2340 * v5512) + (v1779 * v5511);
                let v5520 = (v5516 * v5512) + (v5518 * v5511);
                let v5524 = (v5521 * v5512) + (v1928 * v5511);
                let v5528 = (v5525 * v5512) + (v1960 * v5511);
                let v5532 = (v5529 * v5512) + (v1998 * v5511);
                let v5538 = (v5533 * v5512) + (v5536 * v5511);
                let v5542 = (v5539 * v5512) + (v4469 * v5511);
                let v5546 = (v5543 * v5512) + (v5408 * v5511);
                let v5550 = (v5547 * v5512) + (v4341 * v5511);
                let v5554 = (v5551 * v5512) + (v4356 * v5511);
                let v5558 = (v5555 * v5512) + (v4359 * v5511);
                let v5562 = (v5559 * v5512) + (v4362 * v5511);
                let v5566 = (v5563 * v5512) + (v2029 * v5511);
                let v5570 = (v5567 * v5512) + (v4514 * v5511);
                v5593 = v5515;
                v5618 = v5520;
                v5955 = v5562;
                v5957 = v5550;
                v5958 = v5554;
                v5963 = v5558;
                v6018 = v5532;
                v6038 = v5542;
                v6040 = v5546;
                v6043 = v5538;
                v6571 = v5524;
                v6606 = v5528;
                v7681 = v5566;
                v7683 = v5570;
            } else {
                v5593 = v1779;
                v5618 = v5518;
                v5955 = v4362;
                v5957 = v4341;
                v5958 = v4356;
                v5963 = v4359;
                v6018 = v1998;
                v6038 = v4469;
                v6040 = v5408;
                v6043 = v5536;
                v6571 = v1928;
                v6606 = v1960;
                v7681 = v2029;
                v7683 = v4514;
            }
            let v5572 = v4284 - v5505;
            let v5575 = if v5571 != 0.0 && (if v5572 < v5573 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5588: f64;
            if v5575 != 0.0 {
                let v5579 = v5576 / (v5577 * v5572);
                v5588 = v5579;
            } else {
                let v5581 = v5572 - v5366;
                let v5587 = v1940 * ((v5572 + v5366) + (((v5581 * v5581) + v5583).sqrt()));
                v5588 = v5587;
            }
            let v5589 = v5588.sqrt();
            let v5590 = v4290 * v5589;
            let v5591 = v7 / v5590;
            let v5599 = v1 + ((((v246 + v4310) + (v5593 * v5501)) - (v1790 * v5505)) / v11);
            let v5603 = if v5600 != 0.0 && (if v5599 < v5601 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5615: f64;
            if v5603 != 0.0 {
                let v5606 = v5604 / (v5577 * v5599);
                v5615 = v5606;
            } else {
                let v5608 = v5599 - v1;
                let v5614 = v1940 * ((v5599 + v1) + (((v5608 * v5608) + v5610).sqrt()));
                v5615 = v5614;
            }
            let v5616 = v5615 * v4217;
            let v5617 = v1 / v5616;
            let v5622 = (-(v5618 + (v1919 * v5505))) * v5501;
            let v5630 = (v1940 * (v5622 - (((v5622 * v5622) + v5624).sqrt()))) + v5629;
            let v5639 = ((v1196 + (v5631 / v65)) + (v1206 * v5505)) * ((v4220.powf(v5636)) - v1);
            let v5640 = if v286 > v0 { 1.0 } else { 0.0 };
            let v5660: f64;
            if v5640 != 0.0 {
                let v5642 = (-v296) * v5501;
                let v5645 = if v5642 < v5644 { 1.0 } else { 0.0 };
                let v5648: f64;
                if v5645 != 0.0 {
                    v5648 = v5646;
                } else {
                    let v5647 = rspice_limited_exp(v5642);
                    v5648 = v5647;
                }
                let v5656 = (-v5616) * ((if (v65 / (v65 + (v286 * (v1 + v5648)))) >= v3982 { (v65 / (v65 + (v286 * (v1 + v5648)))) } else { v3982 }).ln());
                v5660 = v5656;
            } else {
                v5660 = v0;
            }
            let v5666 = v2153 + v5665;
            let v5668 = v5460 * v5617;
            let v5670 = v5589 - v4285;
            let v5682 = ((v5414 * v5617) - (v5666 * v5617)) - ((((((v5630 + (v5660 - ((v336 + (v306 / (v65.powf(v316)))) * ((v326 * v5501).tanh())))) + ((v2229 * v5670) - (v5410 * v5505))) - v5639) + v5677) + v5401) * v5617);
            let v5688 = ((((v5683 * v7) * v1748) * v4218).sqrt()) / v11;
            let v5691 = (v63 * v4280) + (v5460 * v4218);
            let v5693 = if v5691 < v5692 { 1.0 } else { 0.0 };
            let v5702: f64;
            if v5693 != 0.0 {
                let v5695 = v5694 / v5691;
                v5702 = v5695;
            } else {
                let v5701 = v1940 * (v5691 + (((v5691 * v5691) + v5697).sqrt()));
                v5702 = v5701;
            }
            let v5704 = v63 * (v5702.sqrt());
            let v5706 = v1 + (v5688 / v5704);
            let v5720 = if (((v5691 + v5707) + v5709) + ((if (((v63 * v5706) / v5688) * ((v5706 / v5688) + v5704)) >= v3982 { (((v63 * v5706) / v5688) * ((v5706 / v5688) + v5704)) } else { v3982 }).ln())) < v5719 { 1.0 } else { 0.0 };
            if v5720 != 0.0 {
            } else {
            }
            let v5726 = ((((v5721 * v7) * v1748) * v5617).sqrt()) / v11;
            let v5727 = v1 / v5726;
            let v5728 = v4280 / v5615;
            let v5734 = (v1940 * v5682) - (v2427 * (v1 + (v5726 / v5730)));
            let v5739 = v5734 + (((v5734 * v5734) + (v2432 * v5682)).sqrt());
            let v5740 = if v5682 < v0 { 1.0 } else { 0.0 };
            let v5761: f64;
            if v5740 != 0.0 {
                let v5742 = (v5682 - v5739) / v5726;
                let v5748 = -((if ((v1 - v5739) + (v5742 * v5742)) >= v3982 { ((v1 - v5739) + (v5742 * v5742)) } else { v3982 }).ln());
                v5761 = v5748;
            } else {
                let v5750 = rspice_limited_exp((-v5739));
                let v5751 = v1940 * v5726;
                let v5757 = ((((v5682 - v1) + v5750) + (v5751 * v5751)).sqrt()) - v5751;
                let v5760 = ((v5757 * v5757) + v1) - v5750;
                v5761 = v5760;
            }
            let v5762 = v5761 + v1;
            let v5763 = v5761 - v1;
            let v5764 = v5763 * v5763;
            let v5770 = (v1940 * (v5762 + ((v5764 + v5765).sqrt()))).sqrt();
            let v5771 = v63 * v5770;
            let v5774 = (v1 + (v5726 / v5771)) / v5726;
            let v5776 = v5761 - (v63 * v5728);
            let v5777 = v5776 - v5668;
            let v5782 = v5777 - ((if ((v2429 * v5774) * v5770) >= v3982 { ((v2429 * v5774) * v5770) } else { v3982 }).ln());
            let v5792 = v1940 * ((v5782 - v5783) - (((v5782 * (v5782 + v5785)) + v5788).sqrt()));
            let v5794 = if v5792 <= v5793 { 1.0 } else { 0.0 };
            let v5901: f64;
            if v5794 != 0.0 {
                let v5797 = if v5792 < v5796 { 1.0 } else { 0.0 };
                let v5819: f64;
                if v5797 != 0.0 {
                    v5819 = v5798;
                } else {
                    let v5800 = if v5792 > v5799 { 1.0 } else { 0.0 };
                    let v5820: f64;
                    if v5800 != 0.0 {
                        let v5801 = rspice_limited_exp(v5792);
                        v5820 = v5801;
                    } else {
                        let v5803 = (v5792 - v5795) / v5371;
                        let v5804 = v5803 * v5803;
                        let v5818 = rspice_limited_exp((v5795 + (v5371 * ((v5805 + (v1940 * v5803)) + (v5804 * (v5809 - (v5804 * (v5810 - v5804))))))));
                        v5820 = v5818;
                    }
                    v5819 = v5820;
                }
                let v5831 = v5819 * (((v1 + v5777) - v5792) - ((if ((v63 * v5774) * (((v5819 * v63) * v5774) + v5771)) >= v3982 { ((v63 * v5774) * (((v5819 * v63) * v5774) + v5771)) } else { v3982 }).ln()));
                v5901 = v5831;
            } else {
                let v5832 = rspice_limited_exp(v5792);
                let v5834 = v63 * v5832;
                let v5835 = v5834 * v5774;
                let v5845 = v5774 + (v1 / v5770);
                let v5851 = v5832 - (((v5834 + ((if (v5835 * (v5835 + v5771)) >= v3982 { (v5835 * (v5835 + v5771)) } else { v3982 }).ln())) - v5777) / ((v63 + (v5842 / v5832)) + (v5845 / ((v5774 * v5832) + v5770))));
                let v5852 = v63 * v5851;
                let v5853 = v5852 * v5774;
                let v5859 = (v5852 + ((if (v5853 * (v5853 + v5771)) >= v3982 { (v5853 * (v5853 + v5771)) } else { v3982 }).ln())) - v5777;
                let v5864 = (v5774 * v5851) + v5770;
                let v5865 = v5845 / v5864;
                let v5866 = (v63 + (v5860 / v5851)) + v5865;
                let v5869 = v1 / v5851;
                let v5886 = v5851 - ((v5859 / v5866) * (v1 + ((v5859 * (((v5868 * (v5869 * v5869)) - (v5872 / (((v5770 * v5770) * v5770) * v5864))) - (v5865 * v5865))) / ((v63 * v5866) * v5866))));
                v5901 = v5886;
            }
            let v5890 = if v5887 != 0.0 && (if v5761 < v5888 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5899: f64;
            if v5890 != 0.0 {
                let v5893 = v5891 / (v5577 * v5761);
                v5899 = v5893;
            } else {
                let v5898 = v1940 * (v5762 + ((v5764 + v5894).sqrt()));
                v5899 = v5898;
            }
            let v5900 = v5899.sqrt();
            let v5902 = v63 * v5901;
            let v5903 = v5761 - v5902;
            let v5907 = if v5904 != 0.0 && (if v5903 < v5905 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5919: f64;
            if v5907 != 0.0 {
                let v5910 = v5908 / (v5577 * v5903);
                v5919 = v5910;
            } else {
                let v5912 = v5903 - v1;
                let v5918 = v1940 * ((v5903 + v1) + (((v5912 * v5912) + v5914).sqrt()));
                v5919 = v5918;
            }
            let v5923 = v1 + (v5726 / (v5900 + (v5919.sqrt())));
            let v5925 = v12 * v10;
            let v5926 = v5924 / v5925;
            let v5927 = v5682 - v5761;
            let v5928 = v5923 - v1;
            let v5931 = v5616 * (v5927 - (v5902 * v5928));
            let v5935 = if v5932 != 0.0 && (if v5931 < v5933 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v5948: f64;
            if v5935 != 0.0 {
                let v5938 = v5936 / (v5577 * v5931);
                v5948 = v5938;
            } else {
                let v5944 = v1940 * (v5931 + (((v5931 * v5931) + v5940).sqrt()));
                v5948 = v5944;
            }
            let v5947 = ((v63 * v5923) * v5616) * v5901;
            let v5960 = v5957 + (v5958 * v5505);
            let v5966 = v1 + ((v5960 * ((v5926 * (v5948 + (v4322 * v5947))).powf(v4377))) + (v5963 / ((v1940 * (v1 + (v5947 / v5948))).powf(v5955))));
            let v5970 = if v5967 != 0.0 && (if v5966 < v5968 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v6017: f64;
            if v5970 != 0.0 {
                let v5973 = v5971 / (v5577 * v5966);
                v6017 = v5973;
            } else {
                let v5975 = v5966 - v1;
                let v5981 = v1940 * ((v5966 + v1) + (((v5975 * v5975) + v5977).sqrt()));
                v6017 = v5981;
            }
            let v5985 = v1 / (((v70 * v3980).powf(v536)) * v24);
            let v6059: f64;
            if v2288 != 0.0 {
                v6059 = v0;
            } else {
                let v5990 = (v1 / (v1 + (v516 * v5947))) + (v2260 * v5670);
                let v5994 = v5990 + (((v5990 * v5990) + v4718).sqrt());
                let v6060: f64;
                if v3959 != 0.0 {
                    let v6003 = (((v5995 + (v5997 * v5994)) * v5985) * v24) * v4416;
                    v6060 = v6003;
                } else {
                    let v6016 = ((v6004 + (((v5995 + (v5997 * v5994)) * v5985) * v24)) + v6012) * v4416;
                    v6060 = v6016;
                }
                v6059 = v6060;
            }
            let v6019 = v1 / v6018;
            let v6020 = v6017.powf(v6019);
            let v6021 = v1216 * v5505;
            let v6025 = v1 - v6021;
            let v6030 = v1940 * (v6025 + (((v6025 * v6025) + ((v5348 + (v6021 * v6021)).sqrt())).sqrt()));
            let v6032 = v2995 * v6031;
            let v6037 = ((v6032 * v5901) * v6030) / (v6032 + (v5901 * v6030));
            let v6039 = if v6038 < v0 { 1.0 } else { 0.0 };
            let v6067: f64;
            if v6039 != 0.0 {
                let v6050 = (v63 * (((v6040 / v6020) * v5616) / (v6043 * v65))) * (v1 / (v1 - (v6038 * v6037)));
                v6067 = v6050;
            } else {
                let v6058 = (v63 * (((v6040 / v6020) * v5616) / (v6043 * v65))) * (v1 + (v6038 * v6037));
                v6067 = v6058;
            }
            let v6061 = if v6059 > v0 { 1.0 } else { 0.0 };
            let v6264: f64;
            if v6061 != 0.0 {
                let v6071 = (((((((v70 * v63) * v5923) * v11) * v5616) * v6043) * v6067) * v6059) / (v63 * v5616);
                let v6072 = v1940 * v6067;
                let v6074 = (v5901 * v5901) + v5901;
                let v6079 = (v6072 * v6074) / (v1 + (v6072 * (v1 + v5901)));
                let v6080 = v63 * v6067;
                let v6082 = v6080 * (v5901 - v6079);
                let v6083 = v6082 * v6082;
                let v6085 = (v1 + v6083).sqrt();
                let v6086 = if v6082 != v0 { 1.0 } else { 0.0 };
                let v6093: f64;
                let v6108: f64;
                if v6086 != 0.0 {
                    let v6087 = v6082.asinh();
                    let v6090 = v6085 + ((v1 / v6082) * v6087);
                    v6093 = v6090;
                    v6108 = v6087;
                } else {
                    let v6092 = v6085 + (v1 / v6085);
                    v6093 = v6092;
                    v6108 = v0;
                }
                let v6104 = ((v6079 * v6093) + ((v6071 * v6079) * ((v5901 + v6079) + v1))) - (v6067 * (v6074 - ((v6079 * v6079) + v6079)));
                let v6116: f64;
                if v6086 != 0.0 {
                    let v6111 = ((v6105 * v6067) * ((v6082 * v6085) - v6108)) / v6083;
                    v6116 = v6111;
                } else {
                    let v6115 = (v6112 * v6067) * (v6082 / v6085);
                    v6116 = v6115;
                }
                let v6119 = v63 * v6079;
                let v6128 = v6079 - (v6104 / ((((v6079 * v6116) + v6093) + (v6071 * ((v5901 + v6119) + v1))) + (v6067 * (v6119 + v1))));
                let v6130 = v6080 * (v5901 - v6128);
                let v6131 = v6130 * v6130;
                let v6133 = (v1 + v6131).sqrt();
                let v6134 = if v6130 != v0 { 1.0 } else { 0.0 };
                let v6141: f64;
                let v6156: f64;
                if v6134 != 0.0 {
                    let v6135 = v6130.asinh();
                    let v6138 = v6133 + ((v1 / v6130) * v6135);
                    v6141 = v6138;
                    v6156 = v6135;
                } else {
                    let v6140 = v6133 + (v1 / v6133);
                    v6141 = v6140;
                    v6156 = v6108;
                }
                let v6152 = ((v6128 * v6141) + ((v6071 * v6128) * ((v5901 + v6128) + v1))) - (v6067 * (v6074 - ((v6128 * v6128) + v6128)));
                let v6164: f64;
                if v6134 != 0.0 {
                    let v6159 = ((v6153 * v6067) * ((v6130 * v6133) - v6156)) / v6131;
                    v6164 = v6159;
                } else {
                    let v6163 = (v6160 * v6067) * (v6130 / v6133);
                    v6164 = v6163;
                }
                let v6167 = v63 * v6128;
                let v6176 = v6128 - (v6152 / ((((v6128 * v6164) + v6141) + (v6071 * ((v5901 + v6167) + v1))) + (v6067 * (v6167 + v1))));
                v6264 = v6176;
            } else {
                let v6177 = v1940 * v6067;
                let v6179 = (v5901 * v5901) + v5901;
                let v6184 = (v6177 * v6179) / (v1 + (v6177 * (v1 + v5901)));
                let v6185 = v63 * v6067;
                let v6187 = v6185 * (v5901 - v6184);
                let v6188 = v6187 * v6187;
                let v6190 = (v1 + v6188).sqrt();
                let v6191 = if v6187 != v0 { 1.0 } else { 0.0 };
                let v6198: f64;
                let v6208: f64;
                if v6191 != 0.0 {
                    let v6192 = v6187.asinh();
                    let v6195 = v6190 + ((v1 / v6187) * v6192);
                    v6198 = v6195;
                    v6208 = v6192;
                } else {
                    let v6197 = v6190 + (v1 / v6190);
                    v6198 = v6197;
                    v6208 = v0;
                }
                let v6204 = (v6184 * v6198) - (v6067 * (v6179 - ((v6184 * v6184) + v6184)));
                let v6216: f64;
                if v6191 != 0.0 {
                    let v6211 = ((v6205 * v6067) * ((v6187 * v6190) - v6208)) / v6188;
                    v6216 = v6211;
                } else {
                    let v6215 = (v6212 * v6067) * (v6187 / v6190);
                    v6216 = v6215;
                }
                let v6224 = v6184 - (v6204 / (((v6184 * v6216) + v6198) + (v6067 * ((v63 * v6184) + v1))));
                let v6226 = v6185 * (v5901 - v6224);
                let v6227 = v6226 * v6226;
                let v6229 = (v1 + v6227).sqrt();
                let v6230 = if v6226 != v0 { 1.0 } else { 0.0 };
                let v6237: f64;
                let v6247: f64;
                if v6230 != 0.0 {
                    let v6231 = v6226.asinh();
                    let v6234 = v6229 + ((v1 / v6226) * v6231);
                    v6237 = v6234;
                    v6247 = v6231;
                } else {
                    let v6236 = v6229 + (v1 / v6229);
                    v6237 = v6236;
                    v6247 = v6208;
                }
                let v6243 = (v6224 * v6237) - (v6067 * (v6179 - ((v6224 * v6224) + v6224)));
                let v6255: f64;
                if v6230 != 0.0 {
                    let v6250 = ((v6244 * v6067) * ((v6226 * v6229) - v6247)) / v6227;
                    v6255 = v6250;
                } else {
                    let v6254 = (v6251 * v6067) * (v6226 / v6229);
                    v6255 = v6254;
                }
                let v6263 = v6224 - (v6243 / (((v6224 * v6255) + v6237) + (v6067 * ((v63 * v6224) + v1))));
                v6264 = v6263;
            }
            let v6265 = v63 * v6264;
            let v6267 = (v6265 * v5923) * v5727;
            let v6275 = (v5776 - (v6265 + ((if (v6267 * (v6267 + (v5726 / v5928))) >= v3982 { (v6267 * (v6267 + (v5726 / v5928))) } else { v3982 }).ln()))) * v5616;
            let v6280 = if (if v6276 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v6278 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v6327: f64;
            if v6280 != 0.0 {
                v6327 = v1;
            } else {
                let v6284 = v65 / (v65 + ((v366 * v5590).sqrt()));
                let v6296 = v1 + (((v6276 * v6284) - (((v6278 * v6284) * (v5901.powf(v6287))) * v5616)) / (v1 + (v6292 * v5505)));
                let v6300 = if v6297 != 0.0 && (if v6296 < v6298 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v6328: f64;
                if v6300 != 0.0 {
                    let v6303 = v6301 / (v5577 * v6296);
                    v6328 = v6303;
                } else {
                    let v6305 = v6296 - v5348;
                    let v6311 = v1940 * ((v6296 + v5348) + (((v6305 * v6305) + v6307).sqrt()));
                    v6328 = v6311;
                }
                v6327 = v6328;
            }
            let v6313 = v6275 - v5460;
            let v6316 = if v6312 != 0.0 && (if v6313 < v6314 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v6326: f64;
            if v6316 != 0.0 {
                let v6319 = v6317 / (v5577 * v6313);
                v6326 = v6319;
            } else {
                let v6325 = v1940 * (v6313 + (((v6313 * v6313) + v6321).sqrt()));
                v6326 = v6325;
            }
            let v6329 = v6326 / v6327;
            let v6332 = v1 / v4454;
            let v6335 = -v4454;
            let v6337 = v5461 * ((v1 + (((v5461 / v6329) + v114).powf(v6332))).powf(v6335));
            let v6345 = (v1940 * (v5762 + ((v5764 + v6340).sqrt()))).sqrt();
            let v6346 = v63 * v6345;
            let v6349 = (v1 + (v5726 / v6346)) / v5726;
            let v6350 = v5776 - ((v6337 + v5460) * v5617);
            let v6355 = v6350 - ((if ((v2429 * v6349) * v6345) >= v3982 { ((v2429 * v6349) * v6345) } else { v3982 }).ln());
            let v6362 = v1940 * ((v6355 - v5783) - (((v6355 * (v6355 + v5785)) + v5788).sqrt()));
            let v6364 = if v6362 <= v6363 { 1.0 } else { 0.0 };
            let v6456: f64;
            if v6364 != 0.0 {
                let v6367 = if v6362 < v6366 { 1.0 } else { 0.0 };
                let v6387: f64;
                if v6367 != 0.0 {
                    v6387 = v6368;
                } else {
                    let v6370 = if v6362 > v6369 { 1.0 } else { 0.0 };
                    let v6388: f64;
                    if v6370 != 0.0 {
                        let v6371 = rspice_limited_exp(v6362);
                        v6388 = v6371;
                    } else {
                        let v6373 = (v6362 - v6365) / v5371;
                        let v6374 = v6373 * v6373;
                        let v6386 = rspice_limited_exp((v6365 + (v5371 * ((v6375 + (v1940 * v6373)) + (v6374 * (v6378 - (v6374 * (v5810 - v6374))))))));
                        v6388 = v6386;
                    }
                    v6387 = v6388;
                }
                let v6399 = v6387 * (((v1 + v6350) - v6362) - ((if ((v63 * v6349) * (((v6387 * v63) * v6349) + v6346)) >= v3982 { ((v63 * v6349) * (((v6387 * v63) * v6349) + v6346)) } else { v3982 }).ln()));
                v6456 = v6399;
            } else {
                let v6400 = rspice_limited_exp(v6362);
                let v6402 = v63 * v6400;
                let v6403 = v6402 * v6349;
                let v6413 = v6349 + (v1 / v6345);
                let v6419 = v6400 - (((v6402 + ((if (v6403 * (v6403 + v6346)) >= v3982 { (v6403 * (v6403 + v6346)) } else { v3982 }).ln())) - v6350) / ((v63 + (v6410 / v6400)) + (v6413 / ((v6349 * v6400) + v6345))));
                let v6420 = v63 * v6419;
                let v6421 = v6420 * v6349;
                let v6427 = (v6420 + ((if (v6421 * (v6421 + v6346)) >= v3982 { (v6421 * (v6421 + v6346)) } else { v3982 }).ln())) - v6350;
                let v6432 = (v6349 * v6419) + v6345;
                let v6433 = v6413 / v6432;
                let v6434 = (v63 + (v6428 / v6419)) + v6433;
                let v6437 = v1 / v6419;
                let v6454 = v6419 - ((v6427 / v6434) * (v1 + ((v6427 * (((v6436 * (v6437 * v6437)) - (v6440 / (((v6345 * v6345) * v6345) * v6432))) - (v6433 * v6433))) / ((v63 * v6434) * v6434))));
                v6456 = v6454;
            }
            let v6458 = ((v5761 - v5901) - v6456) - v1;
            let v6462 = if v6459 != 0.0 && (if v6458 < v6460 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v6474: f64;
            if v6462 != 0.0 {
                let v6465 = v6463 / (v5577 * v6458);
                v6474 = v6465;
            } else {
                let v6467 = v6458 - v1;
                let v6473 = v1940 * ((v6458 + v1) + (((v6467 * v6467) + v6469).sqrt()));
                v6474 = v6473;
            }
            let v6478 = v1 + (v5726 / (v6345 + (v6474.sqrt())));
            let v6479 = v5901 - v6456;
            let v6480 = v6479 * v6479;
            let v6482 = (v1 + v5901) + v6456;
            let v6483 = v1 / v6482;
            let v6484 = v6480 * v6483;
            let v6486 = v5901 + v6456;
            let v6491 = v4318 * v6478;
            let v6492 = v6484 * v6483;
            let v6503 = v6491 * ((v5902 + v6456) + ((v1940 * ((v1 + (v6494 * v5901)) + (v6497 * v6456))) * v6492));
            let v6513 = v6491 * ((v5901 + (v63 * v6456)) + ((v1940 * ((v1 + (v6497 * v5901)) + (v6494 * v6456))) * v6492));
            let v6515 = v5616 * (v5927 - ((v6478 - v1) * (v6486 + (v4318 * v6484))));
            let v6518 = if v6514 != 0.0 && (if v6515 < v6516 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v6530: f64;
            if v6518 != 0.0 {
                let v6521 = v6519 / (v5577 * v6515);
                v6530 = v6521;
            } else {
                let v6527 = v1940 * (v6515 + (((v6515 * v6515) + v6523).sqrt()));
                v6530 = v6527;
            }
            let v6529 = v5616 * (v6503 + v6513);
            let v6542 = v1 + ((v5960 * ((v5926 * (v6530 + (v4322 * v6529))).powf(v4377))) + (v5963 / ((v1940 * (v1 + (v6529 / v6530))).powf(v5955))));
            let v6546 = if v6543 != 0.0 && (if v6542 < v6544 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v6559: f64;
            if v6546 != 0.0 {
                let v6549 = v6547 / (v5577 * v6542);
                v6559 = v6549;
            } else {
                let v6551 = v6542 - v1;
                let v6557 = v1940 * ((v6542 + v1) + (((v6551 * v6551) + v6553).sqrt()));
                v6559 = v6557;
            }
            let v6558 = v63 * v6043;
            let v6562 = (v6558 / (v6040 / v6559)) * v65;
            let v6563 = if v666 > v0 { 1.0 } else { 0.0 };
            let v6594: f64;
            if v6563 != 0.0 {
                let v6566 = v1 + ((v666 * v6529) / v6562);
                v6594 = v6566;
            } else {
                let v6570 = v1 / (v1 - ((v666 * v6529) / v6562));
                v6594 = v6570;
            }
            let v6572 = v5461 - v6337;
            let v6574 = v6529 + (v63 * v5616);
            let v6575 = if v6571 > v0 { 1.0 } else { 0.0 };
            let v6642: f64;
            if v6575 != 0.0 {
                let v6577 = v6574 / (v6329 + v6574);
                let v6579 = v1 + (v606 * v5505);
                let v6581 = if v6579 < v6580 { 1.0 } else { 0.0 };
                let v6590: f64;
                if v6581 != 0.0 {
                    let v6583 = v6582 / v6579;
                    v6590 = v6583;
                } else {
                    let v6589 = v1940 * (v6579 + (((v6579 * v6579) + v6585).sqrt()));
                    v6590 = v6589;
                }
                let v6598 = v1 + (v6572 / ((((v6574 / v6571) * v6577) * v6594) * (v1 / v6590)));
                v6642 = v6598;
            } else {
                v6642 = v1;
            }
            let v6599 = if v1950 <= v0 { 1.0 } else { 0.0 };
            let v6614: f64;
            if v6599 != 0.0 {
                v6614 = v1;
            } else {
                let v6604 = v1 / (v1 + ((v1950 * (v65.sqrt())) / v6574));
                v6614 = v6604;
            }
            let v6605 = v6329 + v6562;
            let v6607 = if v6606 > v0 { 1.0 } else { 0.0 };
            let v6643: f64;
            if v6607 != 0.0 {
                let v6609 = if v6608 < v0 { 1.0 } else { 0.0 };
                let v6621: f64;
                if v6609 != 0.0 {
                    let v6615 = (v6606 / (v1 - ((v6608 * v6529) / v6562))) / v6614;
                    v6621 = v6615;
                } else {
                    let v6620 = (v6606 * (v1 + ((v6608 * v6529) / v6562))) / v6614;
                    v6621 = v6620;
                }
                let v6628 = v1 + (v6621 * ((if (v1 + ((v6572 / v6621) / v6605)) >= v3982 { (v1 + ((v6572 / v6621) / v6605)) } else { v3982 }).ln()));
                v6643 = v6628;
            } else {
                let v6629 = if v6608 < v0 { 1.0 } else { 0.0 };
                let v6640: f64;
                if v6629 != 0.0 {
                    let v6634 = (v6606 / (v1 - ((v6608 * v6529) / v6562))) / v6614;
                    v6640 = v6634;
                } else {
                    let v6639 = (v6606 * (v1 + ((v6608 * v6529) / v6562))) / v6614;
                    v6640 = v6639;
                }
                let v6641 = v1 + v6640;
                v6643 = v6641;
            }
            let v6644 = v6642 * v6643;
            let v6646 = rspice_limited_exp((v646 * v5461));
            let v6647 = if v636 > v0 { 1.0 } else { 0.0 };
            let v6656: f64;
            if v6647 != 0.0 {
                let v6654 = ((v1 + ((v1 + (v6648 * v65)) * v6646)) / v636) * v6614;
                v6656 = v6654;
            } else {
                v6656 = v6655;
            }
            let v6659 = v6644 * (v1 + (v6572 / v6656));
            let v6660 = if v626 > v0 { 1.0 } else { 0.0 };
            let v6670: f64;
            if v6660 != 0.0 {
                let v6661 = v616 * v4294;
                let v6663 = if v6572 > (v6661 / v5643) { 1.0 } else { 0.0 };
                let v6671: f64;
                if v6663 != 0.0 {
                    let v6667 = (v65 * (rspice_limited_exp((v6661 / v6572)))) / v626;
                    v6671 = v6667;
                } else {
                    let v6669 = (v6655 * v65) / v626;
                    v6671 = v6669;
                }
                v6670 = v6671;
            } else {
                v6670 = v6655;
            }
            let v6674 = v6659 * (v1 + (v6572 / v6670));
            let v6675 = v6559.powf(v6019);
            let v6680 = ((v6032 * v6529) * v6030) / (v6032 + (v6529 * v6030));
            let v6698: f64;
            if v6039 != 0.0 {
                let v6689 = (v63 * (((v6040 / v6675) * v5616) / (v6043 * v65))) * (v1 / (v1 - (v6038 * v6680)));
                v6698 = v6689;
            } else {
                let v6697 = (v63 * (((v6040 / v6675) * v5616) / (v6043 * v65))) * (v1 + (v6038 * v6680));
                v6698 = v6697;
            }
            let v6700 = (v63 * v6698) * v6479;
            let v6703 = (v1 + (v6700 * v6700)).sqrt();
            let v6704 = if v6700 != v0 { 1.0 } else { 0.0 };
            let v6713: f64;
            if v6704 != 0.0 {
                let v6709 = v1940 * (v6703 + ((v1 / v6700) * (v6700.asinh())));
                v6713 = v6709;
            } else {
                let v6712 = v1940 * (v6703 + (v1 / v6703));
                v6713 = v6712;
            }
            let v6849: f64;
            let v8742: f64;
            let v10173: f64;
            let v10194: f64;
            if v2288 != 0.0 {
                let v6716 = v4270 * (v6714 - v5412);
                let v6719 = (v5414 - v6716) - v6718;
                let v6729 = (v1 / (v1 + (v516 * (v1940 * (v6719 + (((v6719 * v6719) + v4718).sqrt())))))) + (v2260 * v6716);
                let v6743 = v4416 * (v6004 + ((v6735 + (v6737 * (v1940 * (v6729 + (((v6729 * v6729) + v4718).sqrt()))))) * v5985));
                let v6745 = v4270 * (v5448 - v5412);
                let v6747 = (v5414 - v6745) - v6718;
                let v6757 = (v1 / (v1 + (v516 * (v1940 * (v6747 + (((v6747 * v6747) + v4718).sqrt())))))) + (v2260 * v6745);
                let v6771 = v4416 * (v6012 + ((v6763 + (v6765 * (v1940 * (v6757 + (((v6757 * v6757) + v4718).sqrt()))))) * v5985));
                v6849 = v1;
                v8742 = v0;
                v10173 = v6771;
                v10194 = v6743;
            } else {
                let v6776 = (v1 / (v1 + (v516 * v6529))) + (v2260 * v5670);
                let v6783 = v5995 + (v5997 * (v1940 * (v6776 + (((v6776 * v6776) + v4718).sqrt()))));
                let v6786 = ((v4416 * v6783) * v5985) * v24;
                let v6792 = ((((v6040 / (v6713 * v6559)) * v11) * v70) / v65) * v6529;
                let v6794 = v1 + (v6792 * v6786);
                let v6795 = if v2287 == v63 { 1.0 } else { 0.0 };
                let v6850: f64;
                let v8743: f64;
                let v10174: f64;
                let v10195: f64;
                if v6795 != 0.0 {
                    let v6800 = v4416 * ((v6004 + ((v6783 * v5985) * v24)) + v6012);
                    let v6802 = v1 + (v6792 * v6800);
                    v6850 = v6802;
                    v8743 = v6800;
                    v10174 = v0;
                    v10195 = v0;
                } else {
                    v6850 = v6794;
                    v8743 = v6786;
                    v10174 = v6012;
                    v10195 = v6004;
                }
                v6849 = v6850;
                v8742 = v8743;
                v10173 = v10174;
                v10194 = v10195;
            }
            let v6804 = (v63 * v5615) * v4217;
            let v6811 = ((((v4497 + (v4512 / (v6529 + v6804))) * v6479) * v6479) + v1) - v4102;
            let v6823 = v1940 * (v1 + ((v1 + (v6812 + (v1940 * (v6811 + (((v6811 * v6811) + v6814).sqrt()))))).sqrt()));
            let v6825 = v6823 - v1;
            let v6835 = v6479 / (v6486 + v4578);
            let v6838 = v1 + ((v4562 * v6835) * v6835);
            let v6847 = rspice_limited_exp((-(v4593 / (((if v0 >= (v4608 + ((v4623 * v6479) * v6479)) { v0 } else { (v4608 + ((v4623 * v6479) * v6479)) }) * v6486) + v6804))));
            let v6852 = v6040 / ((v6559 * v6713) * v6849);
            let v6853 = v63 * v24;
            let v6868 = ((((((((((((v6853 * v6478) * v6852) * v70) / v65) * v11) * v5616) * v5616) * (v6479 * v6482)) * v6674) / ((v1940 * ((v6823 + v1) - (((v6825 * v6825) + v6827).sqrt()))) + v6832)) * v6838) * v6847) * v6867;
            let v6869 = if v2288 != 0.0 && v4423 != 0.0 { 1.0 } else { 0.0 };
            let v7688: f64;
            let v7813: f64;
            let v10180: f64;
            let v10201: f64;
            if v6869 != 0.0 {
                let v6875 = v4217 * (((v1748 * v6870) / (v4246 * v4246)).ln());
                let v6979: f64;
                if v4203 != 0.0 {
                    let v6879 = v4217 * (((v6875 * v6875) + v114).sqrt());
                    v6979 = v6879;
                } else {
                    v6979 = v6875;
                }
                let v6882 = v1 - (v6880 * v5420);
                let v6886 = if v6883 != 0.0 && (if v6882 < v6884 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v7023: f64;
                if v6886 != 0.0 {
                    let v6889 = v6887 / (v5577 * v6882);
                    v7023 = v6889;
                } else {
                    let v6895 = v1940 * (v6882 + (((v6882 * v6882) + v6891).sqrt()));
                    v7023 = v6895;
                }
                let v6897 = v5901 - v6896;
                let v6901 = if v6898 != 0.0 && (if v6897 < v6899 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v6915: f64;
                if v6901 != 0.0 {
                    let v6904 = v6902 / (v5577 * v6897);
                    v6915 = v6904;
                } else {
                    let v6906 = v6897 - v5348;
                    let v6912 = v1940 * ((v6897 + v5348) + (((v6906 * v6906) + v6908).sqrt()));
                    v6915 = v6912;
                }
                let v6914 = v2995 * v6913;
                let v6926 = ((v24 * v70) * v4287) * (v6919 * (v1 + (v6920 * ((v6914 * v6915) / (v6914 + v6915)))));
                let v7158: f64;
                let v7816: f64;
                let v10181: f64;
                if v5438 != 0.0 {
                    let v6928 = (v5448 - v5415).abs();
                    let v6930 = if v6929 == v0 { 1.0 } else { 0.0 };
                    let v6971: f64;
                    if v6930 != 0.0 {
                        v6971 = v1;
                    } else {
                        let v6933 = v6928 - v6932;
                        let v6936 = if v6931 != 0.0 && (if v6933 < v6934 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v6946: f64;
                        if v6936 != 0.0 {
                            let v6939 = v6937 / (v5577 * v6933);
                            v6946 = v6939;
                        } else {
                            let v6945 = v1940 * (v6933 + (((v6933 * v6933) + v6941).sqrt()));
                            v6946 = v6945;
                        }
                        let v6948 = v1 + (v6946 * v6929);
                        v6971 = v6948;
                    }
                    let v6952 = if (if v6949 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v3978 != v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v6952 != 0.0 {
                    } else {
                    }
                    let v6956 = if (if v6953 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v6949 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7018: f64;
                    let v7817: f64;
                    if v6956 != 0.0 {
                        let v6958 = v5412 - v6957;
                        let v6968 = ((v6958 * v6958) + (v2995.powf(((v63 * (v6960 - (v6949.ln()))) / v6964)))).sqrt();
                        let v6976 = ((v6926 * v6969) * v6971) * (v1 + (v6949 * (v6968.powf(v6964))));
                        v7018 = v6976;
                        v7817 = v6968;
                    } else {
                        let v6978 = (v6926 * v6969) * v6971;
                        v7018 = v6978;
                        v7817 = v0;
                    }
                    let v6981 = v1 + (v5420 / v6979);
                    let v6985 = if v6982 != 0.0 && (if v6981 < v6983 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v6996: f64;
                    if v6985 != 0.0 {
                        let v6988 = v6986 / (v5577 * v6981);
                        v6996 = v6988;
                    } else {
                        let v6994 = v1940 * (v6981 + (((v6981 * v6981) + v6990).sqrt()));
                        v6996 = v6994;
                    }
                    let v7003 = (v1 - (v6995 * ((v6996.sqrt()) - v1))) - (v7001 * v5420);
                    let v7007 = if v7004 != 0.0 && (if v7003 < v7005 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7017: f64;
                    if v7007 != 0.0 {
                        let v7010 = v7008 / (v5577 * v7003);
                        v7017 = v7010;
                    } else {
                        let v7016 = v1940 * (v7003 + (((v7003 * v7003) + v7012).sqrt()));
                        v7017 = v7016;
                    }
                    let v7019 = v7017 * v7018;
                    let v7024 = ((v7020 * v5437) * v5985) * v7023;
                    let v7025 = v7019 * v7024;
                    let v7027 = v2429 - v7026;
                    let v7028 = v6928.powf(v7027);
                    let v7034 = v1 / v7026;
                    let v7037 = (((v7028 / (v7028 + (v7029 * (v7025.powf(v7027))))).powf(v7034)) * v6928) / v7025;
                    let v7041 = if v7038 != 0.0 && (if v7037 < v7039 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7051: f64;
                    if v7041 != 0.0 {
                        let v7044 = v7042 / (v5577 * v7037);
                        v7051 = v7044;
                    } else {
                        let v7050 = v1940 * (v7037 + (((v7037 * v7037) + v7046).sqrt()));
                        v7051 = v7050;
                    }
                    let v7055 = v7024 * ((v1 + (v7051.powf(v7026))).powf(v7034));
                    v7158 = v7019;
                    v7816 = v7817;
                    v10181 = v7055;
                } else {
                    v7158 = v0;
                    v7816 = v0;
                    v10181 = v0;
                }
                let v7057 = if v7056 != v0 { 1.0 } else { 0.0 };
                let v7159: f64;
                let v7814: f64;
                let v10202: f64;
                if v7057 != 0.0 {
                    let v7059 = (v5418 - v6714).abs();
                    let v7063 = if (if v7060 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v3978 != v63 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v7063 != 0.0 {
                    } else {
                    }
                    let v7066 = if (if v6953 != v0 { 1.0 } else { 0.0 }) != 0.0 && (if v7060 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7121: f64;
                    let v7815: f64;
                    if v7066 != 0.0 {
                        let v7067 = v5412 - v6957;
                        let v7077 = ((v7067 * v7067) + (v2995.powf(((v63 * (v7069 - (v7060.ln()))) / v7073)))).sqrt();
                        let v7083 = (v6926 * v7078) * (v1 + (v7060 * (v7077.powf(v7073))));
                        v7121 = v7083;
                        v7815 = v7077;
                    } else {
                        let v7084 = v6926 * v7078;
                        v7121 = v7084;
                        v7815 = v7816;
                    }
                    let v7086 = v1 + (v5420 / v6979);
                    let v7090 = if v7087 != 0.0 && (if v7086 < v7088 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7100: f64;
                    if v7090 != 0.0 {
                        let v7093 = v7091 / (v5577 * v7086);
                        v7100 = v7093;
                    } else {
                        let v7099 = v1940 * (v7086 + (((v7086 * v7086) + v7095).sqrt()));
                        v7100 = v7099;
                    }
                    let v7106 = (v1 - (v6995 * ((v7100.sqrt()) - v1))) - (v7001 * v5420);
                    let v7110 = if v7107 != 0.0 && (if v7106 < v7108 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7120: f64;
                    if v7110 != 0.0 {
                        let v7113 = v7111 / (v5577 * v7106);
                        v7120 = v7113;
                    } else {
                        let v7119 = v1940 * (v7106 + (((v7106 * v7106) + v7115).sqrt()));
                        v7120 = v7119;
                    }
                    let v7122 = v7120 * v7121;
                    let v7125 = ((v7020 * v7056) * v5985) * v7023;
                    let v7126 = v7122 * v7125;
                    let v7127 = v2429 - v7026;
                    let v7128 = v7059.powf(v7127);
                    let v7133 = v1 / v7026;
                    let v7136 = (((v7128 / (v7128 + (v7029 * (v7126.powf(v7127))))).powf(v7133)) * v7059) / v7126;
                    let v7140 = if v7137 != 0.0 && (if v7136 < v7138 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7150: f64;
                    if v7140 != 0.0 {
                        let v7143 = v7141 / (v5577 * v7136);
                        v7150 = v7143;
                    } else {
                        let v7149 = v1940 * (v7136 + (((v7136 * v7136) + v7145).sqrt()));
                        v7150 = v7149;
                    }
                    let v7154 = v7125 * ((v1 + (v7150.powf(v7026))).powf(v7133));
                    v7159 = v7122;
                    v7814 = v7815;
                    v10202 = v7154;
                } else {
                    v7159 = v7160;
                    v7814 = v7816;
                    v10202 = v0;
                }
                let v7155 = if v5438 != 0.0 && v7057 != 0.0 { 1.0 } else { 0.0 };
                let v7689: f64;
                if v7155 != 0.0 {
                    let v7161 = if v7158 <= v7159 { v7158 } else { v7159 };
                    let v7162 = (v7156 * v6868) / v7161;
                    let v7164 = v7162 - v1;
                    let v7167 = v1997 * v7166;
                    let v7168 = v7167 * v7166;
                    let v7180 = ((((v1940 * ((v7162 + v1) - (((v7164 * v7164) + v7168).sqrt()))) + v7167) + (v1940 * ((v7174 + v7168).sqrt()))) - v1940) - v7167;
                    let v7185 = if v7181 != 0.0 && (if v7180 < (v7182 * v7166) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7201: f64;
                    if v7185 != 0.0 {
                        let v7189 = ((-v7166) * v7166) / (v5577 * v7180);
                        v7201 = v7189;
                    } else {
                        let v7200 = v1940 * ((v7180 + v7190) + ((((v7180 - v7192) * (v7180 - v7194)) + v7168).sqrt()));
                        v7201 = v7200;
                    }
                    let v7209 = (v7156 * v7161) * ((v7201 - (v1940 * ((v7202 + v7168).sqrt()))) + v1940);
                    v7689 = v7209;
                } else {
                    let v7258: f64;
                    if v5438 != 0.0 {
                        let v7211 = (v7156 * v6868) / v7158;
                        let v7213 = v7211 - v1;
                        let v7215 = v1997 * v7166;
                        let v7216 = v7215 * v7166;
                        let v7228 = ((((v1940 * ((v7211 + v1) - (((v7213 * v7213) + v7216).sqrt()))) + v7215) + (v1940 * ((v7222 + v7216).sqrt()))) - v1940) - v7215;
                        let v7233 = if v7229 != 0.0 && (if v7228 < (v7230 * v7166) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v7249: f64;
                        if v7233 != 0.0 {
                            let v7237 = ((-v7166) * v7166) / (v5577 * v7228);
                            v7249 = v7237;
                        } else {
                            let v7248 = v1940 * ((v7228 + v7238) + ((((v7228 - v7240) * (v7228 - v7242)) + v7216).sqrt()));
                            v7249 = v7248;
                        }
                        let v7257 = (v7156 * v7158) * ((v7249 - (v1940 * ((v7250 + v7216).sqrt()))) + v1940);
                        v7258 = v7257;
                    } else {
                        v7258 = v6868;
                    }
                    let v7690: f64;
                    if v7057 != 0.0 {
                        let v7260 = (v7156 * v7258) / v7159;
                        let v7262 = v7260 - v1;
                        let v7264 = v1997 * v7166;
                        let v7265 = v7264 * v7166;
                        let v7277 = ((((v1940 * ((v7260 + v1) - (((v7262 * v7262) + v7265).sqrt()))) + v7264) + (v1940 * ((v7271 + v7265).sqrt()))) - v1940) - v7264;
                        let v7282 = if v7278 != 0.0 && (if v7277 < (v7279 * v7166) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                        let v7298: f64;
                        if v7282 != 0.0 {
                            let v7286 = ((-v7166) * v7166) / (v5577 * v7277);
                            v7298 = v7286;
                        } else {
                            let v7297 = v1940 * ((v7277 + v7287) + ((((v7277 - v7289) * (v7277 - v7291)) + v7265).sqrt()));
                            v7298 = v7297;
                        }
                        let v7306 = (v7156 * v7159) * ((v7298 - (v1940 * ((v7299 + v7265).sqrt()))) + v1940);
                        v7690 = v7306;
                    } else {
                        v7690 = v7258;
                    }
                    v7689 = v7690;
                }
                v7688 = v7689;
                v7813 = v7814;
                v10180 = v10181;
                v10201 = v10202;
            } else {
                v7688 = v6868;
                v7813 = v0;
                v10180 = v0;
                v10201 = v0;
            }
            let v7309 = if (if v2288 != 0.0 && v5441 != 0.0 { 1.0 } else { 0.0 }) != 0.0 && v4423 != 0.0 { 1.0 } else { 0.0 };
            if v7309 != 0.0 {
                let v7314 = ((-v7310) - v7312) / v4217;
                let v7320 = ((((v7315 * v7) * v6870) * v4218).sqrt()) / v11;
                let v7323 = (if (v6870 / v4246) >= v3982 { (v6870 / v4246) } else { v3982 }).ln();
                let v7327 = v2427 * (v1 + (v7320 / v5730));
                let v7328 = (v1940 * v7314) - v7327;
                let v7333 = v7328 + (((v7328 * v7328) + (v2432 * v7314)).sqrt());
                let v7334 = if v7314 < v0 { 1.0 } else { 0.0 };
                let v7355: f64;
                if v7334 != 0.0 {
                    let v7336 = (v7314 - v7333) / v7320;
                    let v7342 = -((if ((v1 - v7333) + (v7336 * v7336)) >= v3982 { ((v1 - v7333) + (v7336 * v7336)) } else { v3982 }).ln());
                    v7355 = v7342;
                } else {
                    let v7344 = rspice_limited_exp((-v7333));
                    let v7345 = v1940 * v7320;
                    let v7351 = ((((v7314 - v1) + v7344) + (v7345 * v7345)).sqrt()) - v7345;
                    let v7354 = ((v7351 * v7351) + v1) - v7344;
                    v7355 = v7354;
                }
                let v7357 = v7355 - v1;
                let v7364 = (v1940 * ((v7355 + v1) + (((v7357 * v7357) + v7359).sqrt()))).sqrt();
                let v7365 = v63 * v7364;
                let v7368 = (v1 + (v7320 / v7365)) / v7320;
                let v7369 = v63 * v7323;
                let v7372 = (v7355 - v7369) - (v5456 / v4217);
                let v7377 = v7372 - ((if ((v2429 * v7368) * v7364) >= v3982 { ((v2429 * v7368) * v7364) } else { v3982 }).ln());
                let v7384 = v1940 * ((v7377 - v5783) - (((v7377 * (v7377 + v5785)) + v5788).sqrt()));
                let v7386 = if v7384 <= v7385 { 1.0 } else { 0.0 };
                let v7481: f64;
                if v7386 != 0.0 {
                    let v7389 = if v7384 < v7388 { 1.0 } else { 0.0 };
                    let v7409: f64;
                    if v7389 != 0.0 {
                        v7409 = v7390;
                    } else {
                        let v7392 = if v7384 > v7391 { 1.0 } else { 0.0 };
                        let v7410: f64;
                        if v7392 != 0.0 {
                            let v7393 = rspice_limited_exp(v7384);
                            v7410 = v7393;
                        } else {
                            let v7395 = (v7384 - v7387) / v5371;
                            let v7396 = v7395 * v7395;
                            let v7408 = rspice_limited_exp((v7387 + (v5371 * ((v7397 + (v1940 * v7395)) + (v7396 * (v7400 - (v7396 * (v5810 - v7396))))))));
                            v7410 = v7408;
                        }
                        v7409 = v7410;
                    }
                    let v7421 = v7409 * (((v1 + v7372) - v7384) - ((if ((v63 * v7368) * (((v7409 * v63) * v7368) + v7365)) >= v3982 { ((v63 * v7368) * (((v7409 * v63) * v7368) + v7365)) } else { v3982 }).ln()));
                    v7481 = v7421;
                } else {
                    let v7422 = rspice_limited_exp(v7384);
                    let v7424 = v63 * v7422;
                    let v7425 = v7424 * v7368;
                    let v7435 = v7368 + (v1 / v7364);
                    let v7441 = v7422 - (((v7424 + ((if (v7425 * (v7425 + v7365)) >= v3982 { (v7425 * (v7425 + v7365)) } else { v3982 }).ln())) - v7372) / ((v63 + (v7432 / v7422)) + (v7435 / ((v7368 * v7422) + v7364))));
                    let v7442 = v63 * v7441;
                    let v7443 = v7442 * v7368;
                    let v7449 = (v7442 + ((if (v7443 * (v7443 + v7365)) >= v3982 { (v7443 * (v7443 + v7365)) } else { v3982 }).ln())) - v7372;
                    let v7454 = (v7368 * v7441) + v7364;
                    let v7455 = v7435 / v7454;
                    let v7456 = (v63 + (v7450 / v7441)) + v7455;
                    let v7459 = v1 / v7441;
                    let v7476 = v7441 - ((v7449 / v7456) * (v1 + ((v7449 * (((v7458 * (v7459 * v7459)) - (v7462 / (((v7364 * v7364) * v7364) * v7454))) - (v7455 * v7455))) / ((v63 * v7456) * v7456))));
                    v7481 = v7476;
                }
                let v7480 = if v7477 != 0.0 && (if v7355 < v7478 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v7480 != 0.0 {
                } else {
                }
                let v7487 = if v7484 != 0.0 && (if (v7355 - (v63 * v7481)) < v7485 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v7487 != 0.0 {
                } else {
                }
                let v7489 = if v7488 > v0 { 1.0 } else { 0.0 };
                if v7489 != 0.0 {
                } else {
                }
                let v7491 = if v7490 == v1 { 1.0 } else { 0.0 };
                if v7491 != 0.0 {
                    let v7495 = (((-v4270) * v5436) - v7312) / v4217;
                    let v7497 = (v1940 * v7495) - v7327;
                    let v7502 = v7497 + (((v7497 * v7497) + (v2432 * v7495)).sqrt());
                    let v7503 = if v7495 < v0 { 1.0 } else { 0.0 };
                    let v7524: f64;
                    if v7503 != 0.0 {
                        let v7505 = (v7495 - v7502) / v7320;
                        let v7511 = -((if ((v1 - v7502) + (v7505 * v7505)) >= v3982 { ((v1 - v7502) + (v7505 * v7505)) } else { v3982 }).ln());
                        v7524 = v7511;
                    } else {
                        let v7513 = rspice_limited_exp((-v7502));
                        let v7514 = v1940 * v7320;
                        let v7520 = ((((v7495 - v1) + v7513) + (v7514 * v7514)).sqrt()) - v7514;
                        let v7523 = ((v7520 * v7520) + v1) - v7513;
                        v7524 = v7523;
                    }
                    let v7526 = v7524 - v1;
                    let v7533 = (v1940 * ((v7524 + v1) + (((v7526 * v7526) + v7528).sqrt()))).sqrt();
                    let v7534 = v63 * v7533;
                    let v7537 = (v1 + (v7320 / v7534)) / v7320;
                    let v7540 = (v7524 - v7369) - (v5420 / v4217);
                    let v7545 = v7540 - ((if ((v2429 * v7537) * v7533) >= v3982 { ((v2429 * v7537) * v7533) } else { v3982 }).ln());
                    let v7552 = v1940 * ((v7545 - v5783) - (((v7545 * (v7545 + v5785)) + v5788).sqrt()));
                    let v7554 = if v7552 <= v7553 { 1.0 } else { 0.0 };
                    let v7649: f64;
                    if v7554 != 0.0 {
                        let v7557 = if v7552 < v7556 { 1.0 } else { 0.0 };
                        let v7577: f64;
                        if v7557 != 0.0 {
                            v7577 = v7558;
                        } else {
                            let v7560 = if v7552 > v7559 { 1.0 } else { 0.0 };
                            let v7578: f64;
                            if v7560 != 0.0 {
                                let v7561 = rspice_limited_exp(v7552);
                                v7578 = v7561;
                            } else {
                                let v7563 = (v7552 - v7555) / v5371;
                                let v7564 = v7563 * v7563;
                                let v7576 = rspice_limited_exp((v7555 + (v5371 * ((v7565 + (v1940 * v7563)) + (v7564 * (v7568 - (v7564 * (v5810 - v7564))))))));
                                v7578 = v7576;
                            }
                            v7577 = v7578;
                        }
                        let v7589 = v7577 * (((v1 + v7540) - v7552) - ((if ((v63 * v7537) * (((v7577 * v63) * v7537) + v7534)) >= v3982 { ((v63 * v7537) * (((v7577 * v63) * v7537) + v7534)) } else { v3982 }).ln()));
                        v7649 = v7589;
                    } else {
                        let v7590 = rspice_limited_exp(v7552);
                        let v7592 = v63 * v7590;
                        let v7593 = v7592 * v7537;
                        let v7603 = v7537 + (v1 / v7533);
                        let v7609 = v7590 - (((v7592 + ((if (v7593 * (v7593 + v7534)) >= v3982 { (v7593 * (v7593 + v7534)) } else { v3982 }).ln())) - v7540) / ((v63 + (v7600 / v7590)) + (v7603 / ((v7537 * v7590) + v7533))));
                        let v7610 = v63 * v7609;
                        let v7611 = v7610 * v7537;
                        let v7617 = (v7610 + ((if (v7611 * (v7611 + v7534)) >= v3982 { (v7611 * (v7611 + v7534)) } else { v3982 }).ln())) - v7540;
                        let v7622 = (v7537 * v7609) + v7533;
                        let v7623 = v7603 / v7622;
                        let v7624 = (v63 + (v7618 / v7609)) + v7623;
                        let v7627 = v1 / v7609;
                        let v7644 = v7609 - ((v7617 / v7624) * (v1 + ((v7617 * (((v7626 * (v7627 * v7627)) - (v7630 / (((v7533 * v7533) * v7533) * v7622))) - (v7623 * v7623))) / ((v63 * v7624) * v7624))));
                        v7649 = v7644;
                    }
                    let v7648 = if v7645 != 0.0 && (if v7524 < v7646 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v7648 != 0.0 {
                    } else {
                    }
                    let v7655 = if v7652 != 0.0 && (if (v7524 - (v63 * v7649)) < v7653 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v7655 != 0.0 {
                    } else {
                    }
                    if v7489 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v7656 = if v4165 > v1 { 1.0 } else { 0.0 };
            let v10215: f64;
            let v10218: f64;
            if v7656 != 0.0 {
                let v7670 = (v7667 * v24) * ((((((v7661 * v4217) * v6852) * v70) / v65) * v11) + ((((v6852 * v70) / v65) * v11) * v6529));
                let v7671 = if v4165 == v63 { 1.0 } else { 0.0 };
                let v10216: f64;
                let v10219: f64;
                if v7671 != 0.0 {
                    let v7674 = if (v1 / v7672) < v3962 { 1.0 } else { 0.0 };
                    let v7676: f64;
                    if v7674 != 0.0 {
                        let v7675 = v1 / v3962;
                        v7676 = v7675;
                    } else {
                        v7676 = v7672;
                    }
                    let v7679 = (v7676 * v7670) / (v7676 + v7670);
                    v10216 = v7679;
                    v10219 = v7676;
                } else {
                    v10216 = v7670;
                    v10219 = v7672;
                }
                v10215 = v10216;
                v10218 = v10219;
            } else {
                v10215 = v0;
                v10218 = v7672;
            }
            let v7680 = if v4422 == v0 { 1.0 } else { 0.0 };
            let v7807: f64;
            if v7680 != 0.0 {
                let v7685 = if (if v7681 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v7683 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v7685 != 0.0 {
                } else {
                    let v7687 = if v6572 > (v7683 / v5643) { 1.0 } else { 0.0 };
                    if v7687 != 0.0 {
                    } else {
                    }
                }
                v7807 = v0;
            } else {
                let v7808: f64;
                if v4423 != 0.0 {
                    let v7700 = v5461 * ((v1 + (((v5461 / ((v1 + (v7691 * v5461)) * v6329)) + v114).powf(v6332))).powf(v6335));
                    let v7701 = v5461 - v7700;
                    let v7705 = if v7702 != 0.0 && (if v7701 < v7703 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7741: f64;
                    if v7705 != 0.0 {
                        let v7708 = v7706 / (v5577 * v7701);
                        v7741 = v7708;
                    } else {
                        let v7714 = v1940 * (v7701 + (((v7701 * v7701) + v7710).sqrt()));
                        v7741 = v7714;
                    }
                    let v7718 = (v1940 * v7683) * (v1 + (v7700.powf(v2060)));
                    let v7737 = if v7734 != 0.0 && (if ((v7681 / (v1 + (v7719 * (rspice_limited_exp((v7720 * v5501)))))) * ((v1 + (v7726 * v5505)) + ((v7729 * v5505) * v5505))) < v7735 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v7737 != 0.0 {
                    } else {
                    }
                    let v7740 = if (if v7681 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v7683 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v7740 != 0.0 {
                    } else {
                        let v7743 = if v7741 > (v7718 / v5643) { 1.0 } else { 0.0 };
                        if v7743 != 0.0 {
                        } else {
                        }
                    }
                    v7808 = v7700;
                } else {
                    v7808 = v0;
                }
                v7807 = v7808;
            }
            let v7745 = if v4423 != 0.0 && (if v6953 == v1 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v7745 != 0.0 {
                let v7747 = v5901 - v7746;
                let v7751 = if v7748 != 0.0 && (if v7747 < v7749 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v7765: f64;
                if v7751 != 0.0 {
                    let v7754 = v7752 / (v5577 * v7747);
                    v7765 = v7754;
                } else {
                    let v7756 = v7747 - v5348;
                    let v7762 = v1940 * ((v7747 + v5348) + (((v7756 * v7756) + v7758).sqrt()));
                    v7765 = v7762;
                }
                let v7764 = v2995 * v7763;
                let v7780 = (((v7773 * v7688) / (((v24 * v70) * v4287) * (v6919 * (v1 + (v7769 * ((v7764 * v7765) / (v7764 + v7765))))))) / v6969) - v1;
                let v7786 = if v7781 != 0.0 && (if v7780 < (v7782 * v7783) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v7798: f64;
                if v7786 != 0.0 {
                    let v7790 = ((-v7783) * v7783) / (v5577 * v7780);
                    v7798 = v7790;
                } else {
                    let v7797 = v1940 * (v7780 + (((v7780 * v7780) + ((v1997 * v7783) * v7783)).sqrt()));
                    v7798 = v7797;
                }
                let v7799 = v6969 * v7798;
                let v7800 = if v6949 > v0 { 1.0 } else { 0.0 };
                let v7855: f64;
                if v7800 != 0.0 {
                    let v7821 = (((v4270 * (v7802 - v7803)) - (v7806 * v7807)) - v7811) - (v6949 * (v7813.powf(v7818)));
                    let v7824 = if v7801 != 0.0 && (if v7821 < v7822 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7856: f64;
                    if v7824 != 0.0 {
                        let v7827 = v7825 / (v5577 * v7821);
                        v7856 = v7827;
                    } else {
                        let v7833 = v1940 * (v7821 + (((v7821 * v7821) + v7829).sqrt()));
                        v7856 = v7833;
                    }
                    v7855 = v7856;
                } else {
                    let v7839 = ((v4270 * (v7802 - v7803)) - (v7806 * v7807)) - v7811;
                    let v7842 = if v7834 != 0.0 && (if v7839 < v7840 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v7857: f64;
                    if v7842 != 0.0 {
                        let v7845 = v7843 / (v5577 * v7839);
                        v7857 = v7845;
                    } else {
                        let v7851 = v1940 * (v7839 + (((v7839 * v7839) + v7847).sqrt()));
                        v7857 = v7851;
                    }
                    v7855 = v7857;
                }
                let v7859 = (((v7852 / v7) * v7799) * v7855).sqrt();
                let v7880 = if v7877 != 0.0 && (if (v7873 * ((v1 + ((v7860 * v5505) + ((v7862 * v5505) * v5505))) + ((v7866 * v7855) + (v7868 * (v7855.powf(v7869)))))) < v7878 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                if v7880 != 0.0 {
                } else {
                }
                let v7883 = if v7859 > (v7881 / v5643) { 1.0 } else { 0.0 };
                if v7883 != 0.0 {
                } else {
                }
            } else {
            }
            let v7884 = if v2337 != 0.0 || v2333 != 0.0 { 1.0 } else { 0.0 };
            let v8074: f64;
            let v8076: f64;
            let v8078: f64;
            let v8080: f64;
            let v8083: f64;
            if v7884 != 0.0 {
                let v7887 = v5616 * ((v5927 + v5901) + v6456);
                let v7891 = ((v7887 * v7887) + v7889).sqrt();
                let v7894 = v1940 * ((-v7887) + v7891);
                let v7896 = v1940 * (v7887 + v7891);
                let v8079: f64;
                if v2333 != 0.0 {
                    let v7899 = -((v7887 / v1056) / v4217);
                    let v7900 = if v7899 > v5467 { 1.0 } else { 0.0 };
                    let v7908: f64;
                    if v7900 != 0.0 {
                        v7908 = v7899;
                    } else {
                        let v7902 = if v7899 < v7901 { 1.0 } else { 0.0 };
                        let v7909: f64;
                        if v7902 != 0.0 {
                            let v7903 = v7899.exp();
                            v7909 = v7903;
                        } else {
                            let v7906 = (v1 + (v7899.exp())).ln();
                            v7909 = v7906;
                        }
                        v7908 = v7909;
                    }
                    let v7921 = (v24 * v70) * v65;
                    let v7927 = (((((v7921 * v4183) * v4176) * v5414) * ((v1056 * v4217) * v7908)) * (rspice_limited_exp((((v7915 * v10) * (v1026 - (v1036 * v7894))) * (v1 + (v1046 * v7894)))))) * v4547;
                    let v7930 = ((v7887 - v1006) / v1016) / v4217;
                    let v7931 = if v7930 > v5467 { 1.0 } else { 0.0 };
                    let v7939: f64;
                    if v7931 != 0.0 {
                        v7939 = v7930;
                    } else {
                        let v7933 = if v7930 < v7932 { 1.0 } else { 0.0 };
                        let v7940: f64;
                        if v7933 != 0.0 {
                            let v7934 = v7930.exp();
                            v7940 = v7934;
                        } else {
                            let v7937 = (v1 + (v7930.exp())).ln();
                            v7940 = v7937;
                        }
                        v7939 = v7940;
                    }
                    let v7958 = v7927 + ((((((v7921 * v7951) * v4176) * v5414) * ((v1016 * v4217) * v7939)) * (rspice_limited_exp((((v7946 * v10) * (v976 - (v986 * v7896))) * (v1 + (v996 * v7896)))))) * v4547);
                    v8079 = v7958;
                } else {
                    v8079 = v0;
                }
                let v8075: f64;
                let v8077: f64;
                let v8081: f64;
                let v8084: f64;
                if v2337 != 0.0 {
                    let v7977 = (((v24 * v4196) * (((v6478 * v5616) * v6486) * (rspice_limited_exp(((v4192 * (v2081 - (v1076 * v7896))) * (v1 + (v1086 * v7896))))))) * ((v5414 + (v1940 * v5501)) - (v1940 * (v5460 + v5459)))) * v4547;
                    let v7982 = v2100 * ((((v6337 * v6337) + v4718).sqrt()) - v5348);
                    let v7984 = rspice_limited_exp((-v7982));
                    let v7987 = ((v7982 + v7984) - v1) + v7889;
                    let v7991 = (v1 - ((v7982 + v1) * v7984)) + v7889;
                    let v7994 = (v7982 * v7982) + v7993;
                    let v7995 = if v7156 > v0 { 1.0 } else { 0.0 };
                    let v8082: f64;
                    let v8085: f64;
                    if v7995 != 0.0 {
                        let v7997 = (v7977 * v7991) / v7994;
                        let v7999 = (v7977 * v7987) / v7994;
                        v8082 = v7999;
                        v8085 = v7997;
                    } else {
                        let v8001 = (v7977 * v7991) / v7994;
                        let v8003 = (v7977 * v7987) / v7994;
                        v8082 = v8001;
                        v8085 = v8003;
                    }
                    let v8004 = v5432 - v6718;
                    let v8007 = ((v8004 * v8004) + v7889).sqrt();
                    let v8009 = if v8008 == v1 { 1.0 } else { 0.0 };
                    let v8026: f64;
                    let v8030: f64;
                    if v8009 != 0.0 {
                        let v8011 = v2088 - (v1106 * v8007);
                        let v8013 = if v8011 < v8012 { 1.0 } else { 0.0 };
                        let v8022: f64;
                        if v8013 != 0.0 {
                            let v8015 = v8014 / v8011;
                            v8022 = v8015;
                        } else {
                            let v8021 = v1940 * (v8011 + (((v8011 * v8011) + v8017).sqrt()));
                            v8022 = v8021;
                        }
                        let v8023 = if v1116 < v4718 { 1.0 } else { 0.0 };
                        let v8027: f64;
                        if v8023 != 0.0 {
                            v8027 = v4718;
                        } else {
                            v8027 = v1116;
                        }
                        v8026 = v8027;
                        v8030 = v8022;
                    } else {
                        let v8025 = v2088 - (v1106 * v8007);
                        v8026 = v1116;
                        v8030 = v8025;
                    }
                    let v8035 = (v4547 * v24) * v4190;
                    let v8040 = (((v8035 * v8036) * v5432) * v8007) * (rspice_limited_exp(((v4193 * v8030) * (v1 + (v8026 * v8007)))));
                    let v8041 = v5431 - v6718;
                    let v8044 = ((v8041 * v8041) + v7889).sqrt();
                    let v8061: f64;
                    let v8065: f64;
                    if v8009 != 0.0 {
                        let v8046 = v2095 - (v1136 * v8044);
                        let v8048 = if v8046 < v8047 { 1.0 } else { 0.0 };
                        let v8057: f64;
                        if v8048 != 0.0 {
                            let v8050 = v8049 / v8046;
                            v8057 = v8050;
                        } else {
                            let v8056 = v1940 * (v8046 + (((v8046 * v8046) + v8052).sqrt()));
                            v8057 = v8056;
                        }
                        let v8058 = if v1146 < v4718 { 1.0 } else { 0.0 };
                        let v8062: f64;
                        if v8058 != 0.0 {
                            v8062 = v4718;
                        } else {
                            v8062 = v1146;
                        }
                        v8061 = v8062;
                        v8065 = v8057;
                    } else {
                        let v8060 = v2095 - (v1136 * v8044);
                        v8061 = v1146;
                        v8065 = v8060;
                    }
                    let v8073 = (((v8035 * v8069) * v5431) * v8044) * (rspice_limited_exp(((v4193 * v8065) * (v1 + (v8061 * v8044)))));
                    v8075 = v8040;
                    v8077 = v8073;
                    v8081 = v8082;
                    v8084 = v8085;
                } else {
                    v8075 = v0;
                    v8077 = v0;
                    v8081 = v0;
                    v8084 = v0;
                }
                v8074 = v8075;
                v8076 = v8077;
                v8078 = v8079;
                v8080 = v8081;
                v8083 = v8084;
            } else {
                v8074 = v0;
                v8076 = v0;
                v8078 = v0;
                v8080 = v0;
                v8083 = v0;
            }
            let v8087 = if v8086 != v0 { 1.0 } else { 0.0 };
            if v8087 != 0.0 {
                let v8091 = if (if (if v2067 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4531 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v2321 != 0.0 { 1.0 } else { 0.0 };
                if v8091 != 0.0 {
                } else {
                    let v8097 = if ((((-v5431) - v806) + v6718) / v5925) < v8096 { 1.0 } else { 0.0 };
                    if v8097 != 0.0 {
                    } else {
                    }
                    let v8098 = if v796 != v0 { 1.0 } else { 0.0 };
                    if v8098 != 0.0 {
                        let v8100 = (v5417 * v5417) * v5417;
                        let v8106 = if (v8100 / ((v796 + (v8100.abs())) + v7889)) < v8105 { 1.0 } else { 0.0 };
                        if v8106 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                }
                let v8110 = if (if (if v2074 <= v0 { 1.0 } else { 0.0 }) != 0.0 || (if v4543 <= v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || v2322 != 0.0 { 1.0 } else { 0.0 };
                if v8110 != 0.0 {
                } else {
                    let v8116 = if ((((-v5432) - v846) + v6718) / v5925) < v8115 { 1.0 } else { 0.0 };
                    if v8116 != 0.0 {
                    } else {
                    }
                    let v8117 = if v836 != v0 { 1.0 } else { 0.0 };
                    if v8117 != 0.0 {
                        let v8119 = (v5420 * v5420) * v5420;
                        let v8125 = if (v8119 / ((v836 + (v8119.abs())) + v7889)) < v8124 { 1.0 } else { 0.0 };
                        if v8125 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                }
            } else {
            }
            if v5186 != 0.0 {
            } else {
            }
            let v8126 = if v4844 > v0 { 1.0 } else { 0.0 };
            if v8126 != 0.0 {
                let v8130 = if (v8127 - v5424) < (v8127 * v4102) { 1.0 } else { 0.0 };
                if v8130 != 0.0 {
                } else {
                }
            } else {
            }
            let v8131 = if v4851 > v0 { 1.0 } else { 0.0 };
            if v8131 != 0.0 {
                let v8135 = if (v8132 - v5424) < (v8132 * v4102) { 1.0 } else { 0.0 };
                if v8135 != 0.0 {
                } else {
                }
            } else {
            }
            let v8136 = if v4863 > v0 { 1.0 } else { 0.0 };
            if v8136 != 0.0 {
                let v8140 = if (v8137 - v5424) < (v8137 * v4102) { 1.0 } else { 0.0 };
                if v8140 != 0.0 {
                } else {
                }
            } else {
            }
            if v5201 != 0.0 {
                let v8142 = if v8141 > v0 { 1.0 } else { 0.0 };
                if v8142 != 0.0 {
                } else {
                }
                let v8144 = if (if v4149 > v0 { 1.0 } else { 0.0 }) != 0.0 && v4144 != 0.0 { 1.0 } else { 0.0 };
                if v8144 != 0.0 {
                } else {
                }
            } else {
            }
            let v8145 = if v4870 > v0 { 1.0 } else { 0.0 };
            if v8145 != 0.0 {
                let v8149 = if (v8146 - v5427) < (v8146 * v4102) { 1.0 } else { 0.0 };
                if v8149 != 0.0 {
                } else {
                }
            } else {
            }
            let v8150 = if v4877 > v0 { 1.0 } else { 0.0 };
            if v8150 != 0.0 {
                let v8152 = if (if v4149 > v0 { 1.0 } else { 0.0 }) != 0.0 && v4144 != 0.0 { 1.0 } else { 0.0 };
                if v8152 != 0.0 {
                    let v8153 = if v5194 > v5183 { 1.0 } else { 0.0 };
                    if v8153 != 0.0 {
                    } else {
                    }
                } else {
                }
                let v8157 = if (v8154 - v5427) < (v8154 * v4102) { 1.0 } else { 0.0 };
                if v8157 != 0.0 {
                } else {
                }
            } else {
            }
            let v8158 = if v4885 > v0 { 1.0 } else { 0.0 };
            if v8158 != 0.0 {
                let v8162 = if (v8159 - v5427) < (v8159 * v4102) { 1.0 } else { 0.0 };
                if v8162 != 0.0 {
                } else {
                }
            } else {
            }
            let v8163 = if v4149 > v0 { 1.0 } else { 0.0 };
            if v8163 != 0.0 {
                if v8145 != 0.0 {
                    let v8166 = if (v8146 - v5430) < (v8146 * v4102) { 1.0 } else { 0.0 };
                    if v8166 != 0.0 {
                    } else {
                    }
                } else {
                }
                if v8150 != 0.0 {
                    let v8167 = if v5194 > v5183 { 1.0 } else { 0.0 };
                    if v8167 != 0.0 {
                    } else {
                    }
                    let v8170 = if (v8154 - v5430) < (v8154 * v4102) { 1.0 } else { 0.0 };
                    if v8170 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v8171 = v4640 * v5176;
            let v8172 = v4670 * v5178;
            let v8174 = (v4700 * v112) * v24;
            let v8176 = if v8175 == v1 { 1.0 } else { 0.0 };
            if v8176 != 0.0 {
            } else {
            }
            let v8178 = if v8177 == v1 { 1.0 } else { 0.0 };
            if v8178 != 0.0 {
            } else {
            }
            let v8180 = if v8179 == v1 { 1.0 } else { 0.0 };
            if v8180 != 0.0 {
            } else {
            }
            let v8181 = if v8171 > v0 { 1.0 } else { 0.0 };
            if v8181 != 0.0 {
                let v8184 = if (v5424 / v4731) < v8183 { 1.0 } else { 0.0 };
                if v8184 != 0.0 {
                    let v8185 = if v8175 != v1 { 1.0 } else { 0.0 };
                    if v8185 != 0.0 {
                        let v8186 = if v8175 == v1940 { 1.0 } else { 0.0 };
                        if v8186 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v8187 = if v8172 > v0 { 1.0 } else { 0.0 };
            if v8187 != 0.0 {
                let v8189 = if (v5424 / v4763) < v8183 { 1.0 } else { 0.0 };
                if v8189 != 0.0 {
                    let v8190 = if v8177 != v1 { 1.0 } else { 0.0 };
                    if v8190 != 0.0 {
                        let v8191 = if v8177 == v1940 { 1.0 } else { 0.0 };
                        if v8191 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v8192 = if v8174 > v0 { 1.0 } else { 0.0 };
            if v8192 != 0.0 {
                let v8194 = if (v5424 / v4795) < v8183 { 1.0 } else { 0.0 };
                if v8194 != 0.0 {
                    let v8195 = if v8179 != v1 { 1.0 } else { 0.0 };
                    if v8195 != 0.0 {
                        let v8196 = if v8179 == v1940 { 1.0 } else { 0.0 };
                        if v8196 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v8198 = (v8141 * v4653) * v5192;
            let v8199 = if v5194 > v5183 { 1.0 } else { 0.0 };
            let v8222: f64;
            if v8199 != 0.0 {
                let v8200 = if v8163 != 0.0 && v4144 != 0.0 { 1.0 } else { 0.0 };
                let v8223: f64;
                if v8200 != 0.0 {
                    let v8203 = (v8141 * v4683) * (v5194 - v5183);
                    v8223 = v8203;
                } else {
                    let v8205 = (v8141 * v4683) * v5194;
                    v8223 = v8205;
                }
                v8222 = v8223;
            } else {
                let v8207 = (v8141 * v4683) * v5194;
                v8222 = v8207;
            }
            let v8209 = (v4713 * v112) * v24;
            let v8211 = if v8210 == v1 { 1.0 } else { 0.0 };
            if v8211 != 0.0 {
            } else {
            }
            let v8213 = if v8212 == v1 { 1.0 } else { 0.0 };
            if v8213 != 0.0 {
            } else {
            }
            let v8215 = if v8214 == v1 { 1.0 } else { 0.0 };
            if v8215 != 0.0 {
            } else {
            }
            let v8216 = if v8198 > v0 { 1.0 } else { 0.0 };
            if v8216 != 0.0 {
                let v8219 = if (v8217 / v4746) < v8183 { 1.0 } else { 0.0 };
                if v8219 != 0.0 {
                    let v8220 = if v8210 != v1 { 1.0 } else { 0.0 };
                    if v8220 != 0.0 {
                        let v8221 = if v8210 == v1940 { 1.0 } else { 0.0 };
                        if v8221 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v8224 = if v8222 > v0 { 1.0 } else { 0.0 };
            if v8224 != 0.0 {
                let v8226 = if (v8217 / v4778) < v8183 { 1.0 } else { 0.0 };
                if v8226 != 0.0 {
                    let v8227 = if v8212 != v1 { 1.0 } else { 0.0 };
                    if v8227 != 0.0 {
                        let v8228 = if v8212 == v1940 { 1.0 } else { 0.0 };
                        if v8228 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v8229 = if v8209 > v0 { 1.0 } else { 0.0 };
            if v8229 != 0.0 {
                let v8231 = if (v8217 / v4810) < v8183 { 1.0 } else { 0.0 };
                if v8231 != 0.0 {
                    let v8232 = if v8214 != v1 { 1.0 } else { 0.0 };
                    if v8232 != 0.0 {
                        let v8233 = if v8214 == v1940 { 1.0 } else { 0.0 };
                        if v8233 != 0.0 {
                        } else {
                        }
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v8234 = if v8163 != 0.0 && v4144 != 0.0 { 1.0 } else { 0.0 };
            if v8234 != 0.0 {
                let v8236 = (v4149 * v4653) * v5192;
                let v8248: f64;
                if v8199 != 0.0 {
                    let v8240 = v4683 * ((v4149 * (v5194 - v5183)) + v5183);
                    v8248 = v8240;
                } else {
                    let v8242 = (v4149 * v4683) * v5194;
                    v8248 = v8242;
                }
                let v8243 = if v8236 > v0 { 1.0 } else { 0.0 };
                if v8243 != 0.0 {
                    let v8245 = if (v5430 / v4746) < v8183 { 1.0 } else { 0.0 };
                    if v8245 != 0.0 {
                        let v8246 = if v8210 != v1 { 1.0 } else { 0.0 };
                        if v8246 != 0.0 {
                            let v8247 = if v8210 == v1940 { 1.0 } else { 0.0 };
                            if v8247 != 0.0 {
                            } else {
                            }
                        } else {
                        }
                    } else {
                    }
                } else {
                }
                let v8249 = if v8248 > v0 { 1.0 } else { 0.0 };
                if v8249 != 0.0 {
                    let v8251 = if (v5430 / v4778) < v8183 { 1.0 } else { 0.0 };
                    if v8251 != 0.0 {
                        let v8252 = if v8212 != v1 { 1.0 } else { 0.0 };
                        if v8252 != 0.0 {
                            let v8253 = if v8212 == v1940 { 1.0 } else { 0.0 };
                            if v8253 != 0.0 {
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
            let v8255 = if v8254 != v0 { 1.0 } else { 0.0 };
            if v8255 != 0.0 {
            } else {
            }
            let v8257 = (v2429 * v4217) * v4287;
            let v8258 = v6558 / v6852;
            let v8260 = if v8259 <= v0 { 1.0 } else { 0.0 };
            let v8568: f64;
            if v8260 != 0.0 {
                v8568 = v0;
            } else {
                let v8266 = v4294 * ((if (((v6572 / v4294) + v8259) / v8258) >= v3982 { (((v6572 / v4294) + v8259) / v8258) } else { v3982 }).ln());
                let v8267 = if v8266 < v0 { 1.0 } else { 0.0 };
                let v8569: f64;
                if v8267 != 0.0 {
                    v8569 = v0;
                } else {
                    v8569 = v8266;
                }
                v8568 = v8569;
            }
            let v8268 = v4217 / v4287;
            let v8271 = v8268 * ((v11 + v5591) + v246);
            let v8272 = v63 * v6478;
            let v8274 = (v8272 * v11) * v4217;
            let v8278 = (((v8274 * v6456) * v6847) * v6838) / v4287;
            let v8283 = ((v8279 * v4217) * (v7688.abs())) * v6852;
            let v8284 = v4287 * v4217;
            let v8286 = (v8284 * v7688) * v7688;
            let v8294 = (v8287 + (v8288 * v8278)) + ((v8291 * v8278) * v8278);
            let v8295 = v8278 + v8271;
            let v8296 = v8295 * v8295;
            let v8298 = (v8287 * v4287) * v4217;
            let v10091: f64;
            let v10270: f64;
            let v10271: f64;
            let v10272: f64;
            let v10273: f64;
            let v10274: f64;
            let v10275: f64;
            if v2357 != 0.0 {
                let v8300 = (v5414 - v5666) / v4217;
                let v8307 = ((((v8301 * v7) * v8303) / v4217).sqrt()) / v11;
                let v8309 = (v8303 / v4246).ln();
                let v8314 = (v1940 * v8300) - (v2427 * (v1 + (v8307 / v5730)));
                let v8319 = v8314 + (((v8314 * v8314) + (v2432 * v8300)).sqrt());
                let v8320 = if v8300 < v0 { 1.0 } else { 0.0 };
                let v8341: f64;
                if v8320 != 0.0 {
                    let v8322 = (v8300 - v8319) / v8307;
                    let v8328 = -((if ((v1 - v8319) + (v8322 * v8322)) >= v3982 { ((v1 - v8319) + (v8322 * v8322)) } else { v3982 }).ln());
                    v8341 = v8328;
                } else {
                    let v8330 = rspice_limited_exp((-v8319));
                    let v8331 = v1940 * v8307;
                    let v8337 = ((((v8300 - v1) + v8330) + (v8331 * v8331)).sqrt()) - v8331;
                    let v8340 = ((v8337 * v8337) + v1) - v8330;
                    v8341 = v8340;
                }
                let v8342 = v8341 + v1;
                let v8343 = v8341 - v1;
                let v8344 = v8343 * v8343;
                let v8350 = (v1940 * (v8342 + ((v8344 + v8345).sqrt()))).sqrt();
                let v8351 = v63 * v8350;
                let v8354 = (v1 + (v8307 / v8351)) / v8307;
                let v8357 = (v8341 - (v63 * v8309)) - v5668;
                let v8362 = v8357 - ((if ((v2429 * v8354) * v8350) >= v3982 { ((v2429 * v8354) * v8350) } else { v3982 }).ln());
                let v8369 = v1940 * ((v8362 - v5783) - (((v8362 * (v8362 + v5785)) + v5788).sqrt()));
                let v8371 = if v8369 <= v8370 { 1.0 } else { 0.0 };
                let v8496: f64;
                if v8371 != 0.0 {
                    let v8374 = if v8369 < v8373 { 1.0 } else { 0.0 };
                    let v8394: f64;
                    if v8374 != 0.0 {
                        v8394 = v8375;
                    } else {
                        let v8377 = if v8369 > v8376 { 1.0 } else { 0.0 };
                        let v8395: f64;
                        if v8377 != 0.0 {
                            let v8378 = rspice_limited_exp(v8369);
                            v8395 = v8378;
                        } else {
                            let v8380 = (v8369 - v8372) / v5371;
                            let v8381 = v8380 * v8380;
                            let v8393 = rspice_limited_exp((v8372 + (v5371 * ((v8382 + (v1940 * v8380)) + (v8381 * (v8385 - (v8381 * (v5810 - v8381))))))));
                            v8395 = v8393;
                        }
                        v8394 = v8395;
                    }
                    let v8406 = v8394 * (((v1 + v8357) - v8369) - ((if ((v63 * v8354) * (((v8394 * v63) * v8354) + v8351)) >= v3982 { ((v63 * v8354) * (((v8394 * v63) * v8354) + v8351)) } else { v3982 }).ln()));
                    v8496 = v8406;
                } else {
                    let v8407 = rspice_limited_exp(v8369);
                    let v8409 = v63 * v8407;
                    let v8410 = v8409 * v8354;
                    let v8420 = v8354 + (v1 / v8350);
                    let v8426 = v8407 - (((v8409 + ((if (v8410 * (v8410 + v8351)) >= v3982 { (v8410 * (v8410 + v8351)) } else { v3982 }).ln())) - v8357) / ((v63 + (v8417 / v8407)) + (v8420 / ((v8354 * v8407) + v8350))));
                    let v8427 = v63 * v8426;
                    let v8428 = v8427 * v8354;
                    let v8434 = (v8427 + ((if (v8428 * (v8428 + v8351)) >= v3982 { (v8428 * (v8428 + v8351)) } else { v3982 }).ln())) - v8357;
                    let v8439 = (v8354 * v8426) + v8350;
                    let v8440 = v8420 / v8439;
                    let v8441 = (v63 + (v8435 / v8426)) + v8440;
                    let v8444 = v1 / v8426;
                    let v8461 = v8426 - ((v8434 / v8441) * (v1 + ((v8434 * (((v8443 * (v8444 * v8444)) - (v8447 / (((v8350 * v8350) * v8350) * v8439))) - (v8440 * v8440))) / ((v63 * v8441) * v8441))));
                    v8496 = v8461;
                }
                let v8465 = if v8462 != 0.0 && (if v8341 < v8463 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8474: f64;
                if v8465 != 0.0 {
                    let v8468 = v8466 / (v5577 * v8341);
                    v8474 = v8468;
                } else {
                    let v8473 = v1940 * (v8342 + ((v8344 + v8469).sqrt()));
                    v8474 = v8473;
                }
                let v8480 = (v6852 * v11) * v70;
                let v8487 = ((v63 * (v1 + (v8307 / (v63 * (v8474.sqrt()))))) * v8480) * v4217;
                let v8490 = v65 - v8481;
                let v8492 = v8272 * v8480;
                let v8495 = (v7688 * v8490) / ((v8492 * v5616) * v5616);
                let v8501 = v1 + (v2429 * (((v8496 * v8496) + v8496) - ((v7688 * v8481) / (v8487 * v4217))));
                let v8502 = if v8501 < v1 { 1.0 } else { 0.0 };
                let v8516: f64;
                if v8502 != 0.0 {
                    v8516 = v0;
                } else {
                    let v8506 = v8503 + (v1940 * (v8501.sqrt()));
                    v8516 = v8506;
                }
                let v8515 = v8507 + (v1940 * ((v1 + (v2429 * (((v6456 * v6456) + v6456) + v8495))).sqrt()));
                let v8524 = (v8487 * v8516) * v8490;
                let v8527 = ((((v63 * v8480) * v4217) * (v8515 - v6456)) * v8481) + (((v8492 * v4217) * v6456) * v8481);
                let v8528 = v8524 + v8527;
                let v8530 = (v1 / v8528) / v8528;
                let v8532 = (v8524 * v8524) * v8530;
                let v8534 = (v8527 * v8527) * v8530;
                let v8535 = if v65 != v8481 { 1.0 } else { 0.0 };
                let v8598: f64;
                if v8535 != 0.0 {
                    let v8537 = (v8274 * v8515) / v4287;
                    let v8543 = (v65 - (v63 * v8538)) - v8481;
                    let v8544 = v8543 * v8543;
                    let v8573 = ((v8283 / ((v8545 * v11) * v8544)) * (((v8287 * ((if ((v8537 + v8271) / v8295) >= v3982 { ((v8537 + v8271) / v8295) } else { v3982 }).ln())) + (v8288 * (v8537 - v8278))) + ((v1940 * v8291) * ((v8537 * v8537) - (v8278 * v8278))))) + ((((v8286 / (((v8545 * v8544) * v70) * v24)) * v8568) * v8294) / v8296);
                    let v8581 = ((v8298 / (((((v70 * v24) * v8543) * v8545) * v8271) * v8271)) * v7688) * v7688;
                    let v8582 = v8581 + v8573;
                    let v8583 = if v8582 > v0 { 1.0 } else { 0.0 };
                    let v8599: f64;
                    if v8583 != 0.0 {
                        let v8585 = (v8573 * v8581) / v8582;
                        v8599 = v8585;
                    } else {
                        v8599 = v0;
                    }
                    v8598 = v8599;
                } else {
                    v8598 = v0;
                }
                let v8596 = ((((v8586 * v4287) * v4217) / (((((v70 * v24) * v8481) * v8545) * v8271) * v8271)) * v7688) * v7688;
                let v8597 = if v8596 > v0 { 1.0 } else { 0.0 };
                let v8601: f64;
                if v8597 != 0.0 {
                    v8601 = v8596;
                } else {
                    v8601 = v0;
                }
                let v8606 = (v7156 * v8604) * ((v8598 * v8532) + (v8601 * v8534));
                v10091 = v8538;
                v10270 = v1;
                v10271 = v8606;
                v10272 = v8607;
                v10273 = v0;
                v10274 = v0;
                v10275 = v0;
            } else {
                let v8609 = if v2361 >= (v65 / v63) { 1.0 } else { 0.0 };
                let v8633: f64;
                if v8609 != 0.0 {
                    v8633 = v0;
                } else {
                    v8633 = v2361;
                }
                let v8610 = if v8287 > v0 { 1.0 } else { 0.0 };
                let v8614 = if (if v8610 != 0.0 || (if v8288 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 }) != 0.0 || (if v8291 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v8689: f64;
                if v8614 != 0.0 {
                    let v8616 = if (if v1547 != v0 { 1.0 } else { 0.0 }) != 0.0 && v8610 != 0.0 { 1.0 } else { 0.0 };
                    let v8643: f64;
                    if v8616 != 0.0 {
                        let v8621 = (v1556 / (v1 + ((v6529 / v1566).powf(v1576)))) / v8287;
                        let v8623 = v8621 - v1;
                        let v8632 = v8287 * (v1940 * ((v8621 + v1) + (((v8623 * v8623) + ((v1997 * v8625) * v8625)).sqrt())));
                        v8643 = v8632;
                    } else {
                        v8643 = v8287;
                    }
                    let v8635 = v65 - (v63 * v8633);
                    let v8636 = v8635 * v8635;
                    let v8642 = (((v8274 * v5901) * v6847) * v6838) / v4287;
                    let v8667 = ((v8283 / ((v8545 * v11) * v8636)) * (((v8643 * ((if ((v8642 + v8271) / v8295) >= v3982 { ((v8642 + v8271) / v8295) } else { v3982 }).ln())) + (v8288 * (v8642 - v8278))) + ((v1940 * v8291) * ((v8642 * v8642) - (v8278 * v8278))))) + ((((v8286 / (((v8545 * v8636) * v70) * v24)) * v8568) * v8294) / v8296);
                    let v8677 = ((((v8643 * v4287) * v4217) / (((((v70 * v24) * v8635) * v8545) * v8271) * v8271)) * v7688) * v7688;
                    let v8678 = v8677 + v8667;
                    let v8679 = if v8678 > v0 { 1.0 } else { 0.0 };
                    let v8690: f64;
                    if v8679 != 0.0 {
                        let v8687 = ((v8667 * v8677) / v8678) / (v1 + (v8682 * (v6479.powf(v8683))));
                        v8690 = v8687;
                    } else {
                        v8690 = v0;
                    }
                    v8689 = v8690;
                } else {
                    v8689 = v0;
                }
                let v8691 = (v7156 * v8604) * v8689;
                v10091 = v8633;
                v10270 = v0;
                v10271 = v0;
                v10272 = v0;
                v10273 = v1;
                v10274 = v8691;
                v10275 = v8607;
            }
            let v8693 = (v6529 / v8258) / v65;
            let v8694 = v8693 * v8693;
            let v8700 = v8695 * (v1 + ((v8696 * v65) * v8694));
            let v8706 = v8701 * (v1 + ((v8702 * v65) * v8694));
            let v8712 = v8707 * (v1 + ((v8708 * v65) * v8694));
            let v8718 = v8713 * (v1 + ((v8714 * v65) * v8694));
            let v8725 = ((-v65) / v8723).exp();
            let v8727 = ((((v2427 * v8700) * v8700) - v1) * v8725) + v1;
            let v8728 = v8712 * v8712;
            let v8729 = v8706 * v8706;
            let v8731 = if v8730 == v0 { 1.0 } else { 0.0 };
            let v10276: f64;
            let v10277: f64;
            let v10278: f64;
            let v10280: f64;
            let v10282: f64;
            let v10284: f64;
            let v10286: f64;
            let v10288: f64;
            if v8731 != 0.0 {
                let v8736 = ((((-v24) * v70) * v65) * v11) * v4217;
                let v8741 = v6852 * (((v8736 * v6503) + (v8736 * v6513)).abs());
                let v8751 = v7307 * (v8257 * ((v8741 / ((v8741 * v8742) + (v65 * v65))) * v8748));
                v10276 = v1;
                v10277 = v8751;
                v10278 = v0;
                v10280 = v0;
                v10282 = v0;
                v10284 = v0;
                v10286 = v0;
                v10288 = v0;
            } else {
                let v8752 = if v8730 == v1 { 1.0 } else { 0.0 };
                let v10279: f64;
                let v10281: f64;
                let v10283: f64;
                let v10285: f64;
                let v10287: f64;
                let v10289: f64;
                if v8752 != 0.0 {
                    let v8757 = (((v6852 * v6713) * v6674) * v11) * (v8272 * v5616);
                    let v8758 = v1940 * v6486;
                    let v8759 = v8758 + v1940;
                    let v8760 = v8759 * v8759;
                    let v8761 = v8760 * v8759;
                    let v8762 = v6480 * v6479;
                    let v8765 = ((v2432 * v8758) + v1940) * v6480;
                    let v8766 = v65 * v6713;
                    let v8767 = v8766 / v65;
                    let v8776 = (((v1 + ((v8728 * (v6337 / v6329)) / (v8770 + v6529))) - v1) * v8725) + v1;
                    let v8780 = if v8777 != 0.0 && (if v8776 < v8778 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v8793: f64;
                    if v8780 != 0.0 {
                        let v8783 = v8781 / (v5577 * v8776);
                        v8793 = v8783;
                    } else {
                        let v8789 = v1940 * (v8776 + (((v8776 * v8776) + v8785).sqrt()));
                        v8793 = v8789;
                    }
                    let v8797 = v8796 * v8759;
                    let v8822 = ((((((v8766 * v8767) * v8767) * (((v8758 / v8760) - (v8765 / ((v8804 * v8760) * v8760))) + ((v6480 * v6480) / ((v8810 * v8760) * v8761)))) * v5808) / v2429) * v8729) / (((v24 * v70) * v8796) * v8757);
                    let v8830 = ((v8767 * ((v6479 / v8797) - (v8762 / (v8810 * v8761)))) * v8718) / v8829;
                    let v8832 = (v8257 * ((((v8757 * v24) * v70) / v8766) * ((v8758 * v8793) + ((v6480 * v8727) / v8797)))).sqrt();
                    let v8833 = if v8822 > v0 { 1.0 } else { 0.0 };
                    let v8839: f64;
                    let v8841: f64;
                    if v8833 != 0.0 {
                        let v8835 = (v8257 / v8822).sqrt();
                        let v8836 = if v8832 > v0 { 1.0 } else { 0.0 };
                        let v8840: f64;
                        if v8836 != 0.0 {
                            let v8838 = (v8830 * v8835) / v8832;
                            v8840 = v8838;
                        } else {
                            v8840 = v0;
                        }
                        v8839 = v8840;
                        v8841 = v8835;
                    } else {
                        v8839 = v0;
                        v8841 = v0;
                    }
                    let v8843 = v1 - v8839;
                    let v8844 = (v8841 * v8841) * v8843;
                    let v8847 = ((v7307 * v8832) * v8832) * v8843;
                    v10279 = v1;
                    v10281 = v8839;
                    v10283 = v1;
                    v10285 = v8844;
                    v10287 = v1;
                    v10289 = v8847;
                } else {
                    v10279 = v0;
                    v10281 = v0;
                    v10283 = v0;
                    v10285 = v0;
                    v10287 = v0;
                    v10289 = v0;
                }
                v10276 = v0;
                v10277 = v0;
                v10278 = v10279;
                v10280 = v10281;
                v10282 = v10283;
                v10284 = v10285;
                v10286 = v10287;
                v10288 = v10289;
            }
            let v10290: f64;
            let v10291: f64;
            let v10292: f64;
            let v10293: f64;
            if v2337 != 0.0 {
                let v8849 = (v7307 * v63) * v4287;
                let v8852 = v8849 * ((v8080 + v8074).abs());
                let v8855 = v8849 * ((v8083 + v8076).abs());
                v10290 = v1;
                v10291 = v8852;
                v10292 = v1;
                v10293 = v8855;
            } else {
                v10290 = v0;
                v10291 = v0;
                v10292 = v0;
                v10293 = v0;
            }
            let v10294: f64;
            let v10295: f64;
            if v2333 != 0.0 {
                let v8859 = ((v7307 * v63) * v4287) * (v8078.abs());
                v10294 = v1;
                v10295 = v8859;
            } else {
                v10294 = v0;
                v10295 = v0;
            }
            let v8861 = if v8860 == v1 { 1.0 } else { 0.0 };
            let v9482: f64;
            let v9484: f64;
            let v9494: f64;
            let v9495: f64;
            let v9498: f64;
            let v9499: f64;
            let v9533: f64;
            let v9552: f64;
            let v9553: f64;
            let v9570: f64;
            if v8861 != 0.0 {
                let v8864 = v5463 * v4218;
                let v8866 = (v5414 * v4218) - ((v2175 + v5665) * v4218);
                let v8869 = (if (v2131 / v4246) >= v3982 { (v2131 / v4246) } else { v3982 }).ln();
                let v8875 = ((((v8870 * v7) * v2131) * v4218).sqrt()) / v11;
                let v8876 = v1 / v8875;
                let v8882 = ((v8877 * v7) * v236) / ((v11 * v11) * v4217);
                let v8884: f64;
                if v4269 != 0.0 {
                    let v8883 = v1 / v8882;
                    v8884 = v8883;
                } else {
                    v8884 = v0;
                }
                let v8886: f64;
                if v4269 != 0.0 {
                    let v8885 = v2131 / v236;
                    v8886 = v8885;
                } else {
                    v8886 = v0;
                }
                let v8887 = v1 + v8886;
                let v8888 = v8866 / v8887;
                let v8889 = v8875 / v8887;
                let v8893 = v2427 * (v1 + (v8889 / v5730));
                let v8894 = (v1940 * v8888) - v8893;
                let v8899 = v8894 + (((v8894 * v8894) + (v2432 * v8888)).sqrt());
                let v8900 = if v8888 < v0 { 1.0 } else { 0.0 };
                let v8921: f64;
                if v8900 != 0.0 {
                    let v8902 = (v8888 - v8899) / v8889;
                    let v8908 = -((if ((v1 - v8899) + (v8902 * v8902)) >= v3982 { ((v1 - v8899) + (v8902 * v8902)) } else { v3982 }).ln());
                    v8921 = v8908;
                } else {
                    let v8910 = rspice_limited_exp((-v8899));
                    let v8911 = v1940 * v8889;
                    let v8917 = ((((v8888 - v1) + v8910) + (v8911 * v8911)).sqrt()) - v8911;
                    let v8920 = ((v8917 * v8917) + v1) - v8910;
                    v8921 = v8920;
                }
                let v8922 = v8921 + v1;
                let v8923 = v8921 - v1;
                let v8924 = v8923 * v8923;
                let v8930 = (v1940 * (v8922 + ((v8924 + v8925).sqrt()))).sqrt();
                let v8931 = v63 * v8930;
                let v8934 = (v1 + (v8875 / v8931)) / v8875;
                let v8936 = v8921 - (v63 * v8869);
                let v8937 = v8936 - v8864;
                let v8944 = (v8937 / v8938) - ((if ((v2429 * v8934) * v8930) >= v3982 { ((v2429 * v8934) * v8930) } else { v3982 }).ln());
                let v8951 = v1940 * ((v8944 - v5783) - (((v8944 * (v8944 + v5785)) + v5788).sqrt()));
                let v8953 = if v8951 <= v8952 { 1.0 } else { 0.0 };
                let v9062: f64;
                if v8953 != 0.0 {
                    let v8956 = if v8951 < v8955 { 1.0 } else { 0.0 };
                    let v8976: f64;
                    if v8956 != 0.0 {
                        v8976 = v8957;
                    } else {
                        let v8959 = if v8951 > v8958 { 1.0 } else { 0.0 };
                        let v8977: f64;
                        if v8959 != 0.0 {
                            let v8960 = rspice_limited_exp(v8951);
                            v8977 = v8960;
                        } else {
                            let v8962 = (v8951 - v8954) / v5371;
                            let v8963 = v8962 * v8962;
                            let v8975 = rspice_limited_exp((v8954 + (v5371 * ((v8964 + (v1940 * v8962)) + (v8963 * (v8967 - (v8963 * (v5810 - v8963))))))));
                            v8977 = v8975;
                        }
                        v8976 = v8977;
                    }
                    let v8990 = v8976 * (((v1 + v8937) - (v8938 * v8951)) - (v8938 * ((if ((v63 * v8934) * (((v8976 * v63) * v8934) + v8931)) >= v3982 { ((v63 * v8934) * (((v8976 * v63) * v8934) + v8931)) } else { v3982 }).ln())));
                    v9062 = v8990;
                } else {
                    let v8991 = rspice_limited_exp(v8951);
                    let v8993 = v63 * v8991;
                    let v8994 = v8993 * v8934;
                    let v9004 = v8934 + (v1 / v8930);
                    let v9005 = v8938 * v9004;
                    let v9011 = v8991 - (((v8993 + (v8938 * ((if (v8994 * (v8994 + v8931)) >= v3982 { (v8994 * (v8994 + v8931)) } else { v3982 }).ln()))) - v8937) / ((v63 + (v8938 / v8991)) + (v9005 / ((v8934 * v8991) + v8930))));
                    let v9012 = v63 * v9011;
                    let v9013 = v9012 * v8934;
                    let v9020 = (v9012 + (v8938 * ((if (v9013 * (v9013 + v8931)) >= v3982 { (v9013 * (v9013 + v8931)) } else { v3982 }).ln()))) - v8937;
                    let v9024 = (v8934 * v9011) + v8930;
                    let v9026 = (v63 + (v8938 / v9011)) + (v9005 / v9024);
                    let v9027 = v9004 / v9024;
                    let v9031 = v1 / v9011;
                    let v9047 = v9011 - ((v9020 / v9026) * (v1 + ((v9020 * ((((-v8938) * (v9031 * v9031)) - (v8938 / (((v8930 * v8930) * v8930) * v9024))) - ((v8938 * v9027) * v9027))) / ((v63 * v9026) * v9026))));
                    v9062 = v9047;
                }
                let v9051 = if v9048 != 0.0 && (if v8921 < v9049 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9060: f64;
                if v9051 != 0.0 {
                    let v9054 = v9052 / (v5577 * v8921);
                    v9060 = v9054;
                } else {
                    let v9059 = v1940 * (v8922 + ((v8924 + v9055).sqrt()));
                    v9060 = v9059;
                }
                let v9061 = v9060.sqrt();
                let v9063 = v63 * v9062;
                let v9064 = v8921 - v9063;
                let v9068 = if v9065 != 0.0 && (if v9064 < v9066 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9080: f64;
                if v9068 != 0.0 {
                    let v9071 = v9069 / (v5577 * v9064);
                    v9080 = v9071;
                } else {
                    let v9073 = v9064 - v1;
                    let v9079 = v1940 * ((v9064 + v1) + (((v9073 * v9073) + v9075).sqrt()));
                    v9080 = v9079;
                }
                let v9084 = v1 + (v8875 / (v9061 + (v9080.sqrt())));
                let v9085 = v8866 - v8921;
                let v9086 = v9084 - v1;
                let v9089 = v4217 * (v9085 - (v9063 * v9086));
                let v9093 = if v9090 != 0.0 && (if v9089 < v9091 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9106: f64;
                if v9093 != 0.0 {
                    let v9096 = v9094 / (v5577 * v9089);
                    v9106 = v9096;
                } else {
                    let v9102 = v1940 * (v9089 + (((v9089 * v9089) + v9098).sqrt()));
                    v9106 = v9102;
                }
                let v9111 = v5957 + (v5958 * v5486);
                let v9114 = v1 + (v9111 * ((v5926 * (v9106 + (v4322 * (((v63 * v9084) * v4217) * v9062)))).powf(v4377)));
                let v9118 = if v9115 != 0.0 && (if v9114 < v9116 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9130: f64;
                if v9118 != 0.0 {
                    let v9121 = v9119 / (v5577 * v9114);
                    v9130 = v9121;
                } else {
                    let v9123 = v9114 - v1;
                    let v9129 = v1940 * ((v9114 + v1) + (((v9123 * v9123) + v9125).sqrt()));
                    v9130 = v9129;
                }
                let v9134 = v9133 * v94;
                let v9135 = ((v6040 / v9130) * v4217) / v9134;
                let v9143 = v63 * ((v9135 * ((v9062 * v9062) + v9062)) / (v1 + (v9135 * (v1 + v9062))));
                let v9145 = (v9143 * v9084) * v8876;
                let v9155 = ((v8936 - (v9143 + ((if (v9145 * (v9145 + (v8875 / v9086))) >= v3982 { (v9145 * (v9145 + (v8875 / v9086))) } else { v3982 }).ln()))) * v4217) - v5463;
                let v9158 = if v9154 != 0.0 && (if v9155 < v9156 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9203: f64;
                if v9158 != 0.0 {
                    let v9161 = v9159 / (v5577 * v9155);
                    v9203 = v9161;
                } else {
                    let v9167 = v1940 * (v9155 + (((v9155 * v9155) + v9163).sqrt()));
                    v9203 = v9167;
                }
                let v9172 = if (if v9168 == v0 { 1.0 } else { 0.0 }) != 0.0 && (if v9170 == v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9204: f64;
                if v9172 != 0.0 {
                    v9204 = v9173;
                } else {
                    let v9177 = v65 / (v65 + ((v366 * v5590).sqrt()));
                    let v9187 = v1 + (((v9168 * v9177) - (((v9170 * v9177) * v9062) * v5616)) / (v1 + (v9183 * v5505)));
                    let v9191 = if v9188 != 0.0 && (if v9187 < v9189 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    let v9205: f64;
                    if v9191 != 0.0 {
                        let v9194 = v9192 / (v5577 * v9187);
                        v9205 = v9194;
                    } else {
                        let v9196 = v9187 - v5348;
                        let v9202 = v1940 * ((v9187 + v5348) + (((v9196 * v9196) + v9198).sqrt()));
                        v9205 = v9202;
                    }
                    v9204 = v9205;
                }
                let v9206 = v9203 / v9204;
                let v9212 = v5464 * ((v1 + (((v5461 / v9206) + v114).powf(v6332))).powf(v6335));
                let v9220 = (v1940 * (v8922 + ((v8924 + v9215).sqrt()))).sqrt();
                let v9221 = v63 * v9220;
                let v9224 = (v1 + (v8875 / v9221)) / v8875;
                let v9225 = v8936 - ((v9212 + v5463) * v4218);
                let v9231 = (v9225 / v8938) - ((if ((v2429 * v9224) * v9220) >= v3982 { ((v2429 * v9224) * v9220) } else { v3982 }).ln());
                let v9238 = v1940 * ((v9231 - v5783) - (((v9231 * (v9231 + v5785)) + v5788).sqrt()));
                let v9240 = if v9238 <= v9239 { 1.0 } else { 0.0 };
                let v9336: f64;
                if v9240 != 0.0 {
                    let v9243 = if v9238 < v9242 { 1.0 } else { 0.0 };
                    let v9263: f64;
                    if v9243 != 0.0 {
                        v9263 = v9244;
                    } else {
                        let v9246 = if v9238 > v9245 { 1.0 } else { 0.0 };
                        let v9264: f64;
                        if v9246 != 0.0 {
                            let v9247 = rspice_limited_exp(v9238);
                            v9264 = v9247;
                        } else {
                            let v9249 = (v9238 - v9241) / v5371;
                            let v9250 = v9249 * v9249;
                            let v9262 = rspice_limited_exp((v9241 + (v5371 * ((v9251 + (v1940 * v9249)) + (v9250 * (v9254 - (v9250 * (v5810 - v9250))))))));
                            v9264 = v9262;
                        }
                        v9263 = v9264;
                    }
                    let v9277 = v9263 * (((v1 + v9225) - (v8938 * v9238)) - (v8938 * ((if ((v63 * v9224) * (((v9263 * v63) * v9224) + v9221)) >= v3982 { ((v63 * v9224) * (((v9263 * v63) * v9224) + v9221)) } else { v3982 }).ln())));
                    v9336 = v9277;
                } else {
                    let v9278 = rspice_limited_exp(v9238);
                    let v9280 = v63 * v9278;
                    let v9281 = v9280 * v9224;
                    let v9291 = v9224 + (v1 / v9220);
                    let v9292 = v8938 * v9291;
                    let v9298 = v9278 - (((v9280 + (v8938 * ((if (v9281 * (v9281 + v9221)) >= v3982 { (v9281 * (v9281 + v9221)) } else { v3982 }).ln()))) - v9225) / ((v63 + (v8938 / v9278)) + (v9292 / ((v9224 * v9278) + v9220))));
                    let v9299 = v63 * v9298;
                    let v9300 = v9299 * v9224;
                    let v9307 = (v9299 + (v8938 * ((if (v9300 * (v9300 + v9221)) >= v3982 { (v9300 * (v9300 + v9221)) } else { v3982 }).ln()))) - v9225;
                    let v9311 = (v9224 * v9298) + v9220;
                    let v9313 = (v63 + (v8938 / v9298)) + (v9292 / v9311);
                    let v9314 = v9291 / v9311;
                    let v9318 = v1 / v9298;
                    let v9334 = v9298 - ((v9307 / v9313) * (v1 + ((v9307 * ((((-v8938) * (v9318 * v9318)) - (v8938 / (((v9220 * v9220) * v9220) * v9311))) - ((v8938 * v9314) * v9314))) / ((v63 * v9313) * v9313))));
                    v9336 = v9334;
                }
                let v9338 = ((v8921 - v9062) - v9336) - v1;
                let v9342 = if v9339 != 0.0 && (if v9338 < v9340 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9354: f64;
                if v9342 != 0.0 {
                    let v9345 = v9343 / (v5577 * v9338);
                    v9354 = v9345;
                } else {
                    let v9347 = v9338 - v1;
                    let v9353 = v1940 * ((v9338 + v1) + (((v9347 * v9347) + v9349).sqrt()));
                    v9354 = v9353;
                }
                let v9355 = v9354.sqrt();
                let v9358 = v8887 + (v8875 / (v9220 + v9355));
                let v9361 = v1940 + ((v8886 * v9355) * v8876);
                let v9363 = v9062 + v9336;
                let v9369 = v9358 / (v9361 + (((v9361 * v9361) + ((v9358 * v9363) * v8884)).sqrt()));
                let v9370 = v9369 - v1;
                let v9373 = v4217 * (v9085 - (v9063 * v9370));
                let v9377 = if v9374 != 0.0 && (if v9373 < v9375 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9404: f64;
                if v9377 != 0.0 {
                    let v9380 = v9378 / (v5577 * v9373);
                    v9404 = v9380;
                } else {
                    let v9386 = v1940 * (v9373 + (((v9373 * v9373) + v9382).sqrt()));
                    v9404 = v9386;
                }
                let v9390 = v4217 * (v9085 - ((v63 * v9336) * v9370));
                let v9394 = if v9391 != 0.0 && (if v9390 < v9392 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9405: f64;
                if v9394 != 0.0 {
                    let v9397 = v9395 / (v5577 * v9390);
                    v9405 = v9397;
                } else {
                    let v9403 = v1940 * (v9390 + (((v9390 * v9390) + v9399).sqrt()));
                    v9405 = v9403;
                }
                let v9412 = v5926 * ((v1940 * (v9404 + v9405)) + (v4322 * ((v9369 * v4217) * v9363)));
                let v9416 = (v8866 + (v9413 * v4218)) / v8887;
                let v9418 = (v1940 * v9416) - v8893;
                let v9423 = v9418 + (((v9418 * v9418) + (v2432 * v9416)).sqrt());
                let v9424 = if v9416 < v0 { 1.0 } else { 0.0 };
                let v9496: f64;
                if v9424 != 0.0 {
                    let v9426 = (v9416 - v9423) / v8889;
                    let v9432 = -((if ((v1 - v9423) + (v9426 * v9426)) >= v3982 { ((v1 - v9423) + (v9426 * v9426)) } else { v3982 }).ln());
                    v9496 = v9432;
                } else {
                    let v9434 = rspice_limited_exp((-v9423));
                    let v9435 = v1940 * v8889;
                    let v9441 = ((((v9416 - v1) + v9434) + (v9435 * v9435)).sqrt()) - v9435;
                    let v9444 = ((v9441 * v9441) + v1) - v9434;
                    v9496 = v9444;
                }
                let v9447 = v1 + (v9111 * (v9412.powf(v4377)));
                let v9451 = if v9448 != 0.0 && (if v9447 < v9449 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9463: f64;
                if v9451 != 0.0 {
                    let v9454 = v9452 / (v5577 * v9447);
                    v9463 = v9454;
                } else {
                    let v9456 = v9447 - v1;
                    let v9462 = v1940 * ((v9447 + v1) + (((v9456 * v9456) + v9458).sqrt()));
                    v9463 = v9462;
                }
                let v9464 = v6040 / v9463;
                let v9469 = (((v63 * v9464) * v4217) / v9134) * (v9062 - v9336);
                let v9475 = v1940 * (v1 + ((v1 + ((v63 * v9469) * v9469)).sqrt()));
                let v9479 = v9206 + (((v63 * v9133) / v9464) * v94);
                let v9480 = v5464 - v9212;
                v9482 = v9480;
                v9484 = v9479;
                v9494 = v8866;
                v9495 = v9496;
                v9498 = v9062;
                v9499 = v9336;
                v9533 = v8884;
                v9552 = v9204;
                v9553 = v9475;
                v9570 = v9369;
            } else {
                v9482 = v6572;
                v9484 = v6605;
                v9494 = v5682;
                v9495 = v5761;
                v9498 = v5901;
                v9499 = v6456;
                v9533 = v0;
                v9552 = v1;
                v9553 = v6713;
                v9570 = v6478;
            }
            let v9481 = if v2207 != v0 { 1.0 } else { 0.0 };
            let v9491: f64;
            if v9481 != 0.0 {
                let v9490 = v1 + (v2207 * ((if (v1 + ((v9482 / v2207) / v9484)) >= v3982 { (v1 + ((v9482 / v2207) / v9484)) } else { v3982 }).ln()));
                v9491 = v9490;
            } else {
                v9491 = v1;
            }
            let v9492 = v1 / v9491;
            let v9493 = v9491 - v1;
            let v9497 = v9494 - v9495;
            let v9500 = v9498 - v9499;
            let v9501 = v9500 * v9500;
            let v9503 = v9497 + (v63 * v9498);
            let v9505 = v9497 + (v63 * v9499);
            let v9509 = if v9506 != 0.0 && (if v9503 < v9507 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v9532: f64;
            if v9509 != 0.0 {
                let v9512 = v9510 / (v5577 * v9503);
                v9532 = v9512;
            } else {
                let v9518 = v1940 * (v9503 + (((v9503 * v9503) + v9514).sqrt()));
                v9532 = v9518;
            }
            let v9522 = if v9519 != 0.0 && (if v9505 < v9520 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v9537: f64;
            if v9522 != 0.0 {
                let v9525 = v9523 / (v5577 * v9505);
                v9537 = v9525;
            } else {
                let v9531 = v1940 * (v9505 + (((v9505 * v9505) + v9527).sqrt()));
                v9537 = v9531;
            }
            let v9536 = (v1997 + (v9532 * v9533)).sqrt();
            let v9540 = (v1997 + (v9537 * v9533)).sqrt();
            let v9544 = v63 * v9540;
            let v9545 = v1 + v9544;
            let v9547 = v9536 + v9540;
            let v9548 = v9547 * v9547;
            let v9558 = ((v9552 * v9553) * v9492) / ((v1 + v9498) + v9499);
            let v9592 = if v9586 != 0.0 && (if (v4217 * ((v9492 * (((v9503 / (v1 + (v63 * v9536))) + (v9505 / v9545)) + (((v4318 * (v9501 / (v9548 * v9547))) * (((v6494 * (v9548 + (v9536 * v9540))) * v9558) + (v63 * v9533))) - (v9570 * ((v9498 + v9499) + ((v4318 * v9501) * v9558)))))) + (v9493 * ((v9497 - ((v63 * (v9570 - v1)) * v9499)) + ((v9505 * (v9544 - v1)) / v9545))))) < (v9588 * v9589) { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            if v9592 != 0.0 {
            } else {
            }
            let v9594 = if v9593 == 0.0 { 1.0 } else { 0.0 };
            if v9594 != 0.0 {
            } else {
            }
            let v9596 = if v9595 == v0 { 1.0 } else { 0.0 };
            if v9596 != 0.0 {
            } else {
            }
            let v9597 = if v5312 == v1 { 1.0 } else { 0.0 };
            let v10169: f64;
            let v10296: f64;
            let v10297: f64;
            let v10298: f64;
            if v9597 != 0.0 {
                let v9600 = (if (v1316 / v4246) >= v3982 { (v1316 / v4246) } else { v3982 }).ln();
                let v9604 = if ((v4279 + (v4217 * v9600)) + v376) >= v4279 { ((v4279 + (v4217 * v9600)) + v376) } else { v4279 };
                let v9607 = (v4286 / (v4287 * v1316)).sqrt();
                let v9609 = v1 + (v1416 * v4296);
                let v9611 = if v9609 < v9610 { 1.0 } else { 0.0 };
                let v9620: f64;
                if v9611 != 0.0 {
                    let v9613 = v9612 / v9609;
                    v9620 = v9613;
                } else {
                    let v9619 = v1940 * (v9609 + (((v9609 * v9609) + v9615).sqrt()));
                    v9620 = v9619;
                }
                let v9621 = v1306 * v9620;
                let v9625 = v9622 * (v1 + (v1426 * v4296));
                let v9627 = v9604 - v5505;
                let v9630 = if v9626 != 0.0 && (if v9627 < v9628 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9642: f64;
                if v9630 != 0.0 {
                    let v9633 = v9631 / (v5577 * v9627);
                    v9642 = v9633;
                } else {
                    let v9635 = v9627 - v5366;
                    let v9641 = v1940 * ((v9627 + v5366) + (((v9635 * v9635) + v9637).sqrt()));
                    v9642 = v9641;
                }
                let v9645 = v7 / (v9607 * (v9642.sqrt()));
                let v9652 = v1 + ((((v1326 + v9621) + (v1336 * v5501)) - (v1346 * v5505)) / v11);
                let v9656 = if v9653 != 0.0 && (if v9652 < v9654 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9668: f64;
                if v9656 != 0.0 {
                    let v9659 = v9657 / (v5577 * v9652);
                    v9668 = v9659;
                } else {
                    let v9661 = v9652 - v1;
                    let v9667 = v1940 * ((v9652 + v1) + (((v9661 * v9661) + v9663).sqrt()));
                    v9668 = v9667;
                }
                let v9669 = v9668 * v4217;
                let v9670 = v1 / v9669;
                let v9671 = v5414 * v9670;
                let v9672 = v5460 * v9670;
                let v9673 = v5666 * v9670;
                let v9677 = (-(v9625 + (v1366 * v5505))) * v5501;
                let v9684 = ((v1376 + (v1386 / v65)) + (v1396 * v5505)) * ((v4220.powf(v1406)) - v1);
                let v9688 = v4294 * (v1 + (v9685 * v5505));
                let v9689 = if v9688 > v0 { 1.0 } else { 0.0 };
                let v9703: f64;
                if v9689 != 0.0 {
                    let v9692 = (v9690 * v65) / v9688;
                    let v9694 = if v9692 < v9693 { 1.0 } else { 0.0 };
                    let v9704: f64;
                    if v9694 != 0.0 {
                        let v9699 = (v1940 * v9695) / ((v9692.cosh()) - v1);
                        v9704 = v9699;
                    } else {
                        let v9702 = v9695 * (rspice_limited_exp((-v9692)));
                        v9704 = v9702;
                    }
                    v9703 = v9704;
                } else {
                    v9703 = v0;
                }
                let v9722 = (v9671 - v9673) - (((((((v9677 - v9684) + (v9703 * (v9705 - v9604))) + v9710) + v9712) - ((v9715 + v5404) * v5505)) + v5403) * v9670);
                let v9738 = (((((v9731 * v7) * v1316) * v9670).sqrt()) / v11) * (v1 + (v9723 * (v1 + (v9724 * (v65.powf((-v9725)))))));
                let v9739 = v9600 / v9668;
                let v9744 = (v1940 * v9722) - (v2427 * (v1 + (v9738 / v5730)));
                let v9749 = v9744 + (((v9744 * v9744) + (v2432 * v9722)).sqrt());
                let v9750 = if v9722 < v0 { 1.0 } else { 0.0 };
                let v9771: f64;
                if v9750 != 0.0 {
                    let v9752 = (v9722 - v9749) / v9738;
                    let v9758 = -((if ((v1 - v9749) + (v9752 * v9752)) >= v3982 { ((v1 - v9749) + (v9752 * v9752)) } else { v3982 }).ln());
                    v9771 = v9758;
                } else {
                    let v9760 = rspice_limited_exp((-v9749));
                    let v9761 = v1940 * v9738;
                    let v9767 = ((((v9722 - v1) + v9760) + (v9761 * v9761)).sqrt()) - v9761;
                    let v9770 = ((v9767 * v9767) + v1) - v9760;
                    v9771 = v9770;
                }
                let v9772 = v9771 + v1;
                let v9773 = v9771 - v1;
                let v9774 = v9773 * v9773;
                let v9780 = (v1940 * (v9772 + ((v9774 + v9775).sqrt()))).sqrt();
                let v9781 = v63 * v9780;
                let v9784 = (v1 + (v9738 / v9781)) / v9738;
                let v9786 = v9771 - (v63 * v9739);
                let v9787 = v9786 - v9672;
                let v9792 = v9787 - ((if ((v2429 * v9784) * v9780) >= v3982 { ((v2429 * v9784) * v9780) } else { v3982 }).ln());
                let v9799 = v1940 * ((v9792 - v5783) - (((v9792 * (v9792 + v5785)) + v5788).sqrt()));
                let v9801 = if v9799 <= v9800 { 1.0 } else { 0.0 };
                let v9893: f64;
                if v9801 != 0.0 {
                    let v9804 = if v9799 < v9803 { 1.0 } else { 0.0 };
                    let v9824: f64;
                    if v9804 != 0.0 {
                        v9824 = v9805;
                    } else {
                        let v9807 = if v9799 > v9806 { 1.0 } else { 0.0 };
                        let v9825: f64;
                        if v9807 != 0.0 {
                            let v9808 = rspice_limited_exp(v9799);
                            v9825 = v9808;
                        } else {
                            let v9810 = (v9799 - v9802) / v5371;
                            let v9811 = v9810 * v9810;
                            let v9823 = rspice_limited_exp((v9802 + (v5371 * ((v9812 + (v1940 * v9810)) + (v9811 * (v9815 - (v9811 * (v5810 - v9811))))))));
                            v9825 = v9823;
                        }
                        v9824 = v9825;
                    }
                    let v9836 = v9824 * (((v1 + v9787) - v9799) - ((if ((v63 * v9784) * (((v9824 * v63) * v9784) + v9781)) >= v3982 { ((v63 * v9784) * (((v9824 * v63) * v9784) + v9781)) } else { v3982 }).ln()));
                    v9893 = v9836;
                } else {
                    let v9837 = rspice_limited_exp(v9799);
                    let v9839 = v63 * v9837;
                    let v9840 = v9839 * v9784;
                    let v9850 = v9784 + (v1 / v9780);
                    let v9856 = v9837 - (((v9839 + ((if (v9840 * (v9840 + v9781)) >= v3982 { (v9840 * (v9840 + v9781)) } else { v3982 }).ln())) - v9787) / ((v63 + (v9847 / v9837)) + (v9850 / ((v9784 * v9837) + v9780))));
                    let v9857 = v63 * v9856;
                    let v9858 = v9857 * v9784;
                    let v9864 = (v9857 + ((if (v9858 * (v9858 + v9781)) >= v3982 { (v9858 * (v9858 + v9781)) } else { v3982 }).ln())) - v9787;
                    let v9869 = (v9784 * v9856) + v9780;
                    let v9870 = v9850 / v9869;
                    let v9871 = (v63 + (v9865 / v9856)) + v9870;
                    let v9874 = v1 / v9856;
                    let v9891 = v9856 - ((v9864 / v9871) * (v1 + ((v9864 * (((v9873 * (v9874 * v9874)) - (v9877 / (((v9780 * v9780) * v9780) * v9869))) - (v9870 * v9870))) / ((v63 * v9871) * v9871))));
                    v9893 = v9891;
                }
                let v9892 = v63 * v9669;
                let v9898 = (((v9892 * v9893) + v9892) + v5460) - v5460;
                let v9901 = if v9897 != 0.0 && (if v9898 < v9899 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v9911: f64;
                if v9901 != 0.0 {
                    let v9904 = v9902 / (v5577 * v9898);
                    v9911 = v9904;
                } else {
                    let v9910 = v1940 * (v9898 + (((v9898 * v9898) + v9906).sqrt()));
                    v9911 = v9910;
                }
                let v9924 = (v1940 * (v9772 + ((v9774 + v9919).sqrt()))).sqrt();
                let v9925 = v63 * v9924;
                let v9928 = (v1 + (v9738 / v9925)) / v9738;
                let v9929 = v9786 - (((v5461 * ((v1 + ((v5461 / v9911).powf(v6332))).powf(v6335))) + v5460) * v9670);
                let v9934 = v9929 - ((if ((v2429 * v9928) * v9924) >= v3982 { ((v2429 * v9928) * v9924) } else { v3982 }).ln());
                let v9941 = v1940 * ((v9934 - v5783) - (((v9934 * (v9934 + v5785)) + v5788).sqrt()));
                let v9943 = if v9941 <= v9942 { 1.0 } else { 0.0 };
                let v10049: f64;
                if v9943 != 0.0 {
                    let v9946 = if v9941 < v9945 { 1.0 } else { 0.0 };
                    let v9966: f64;
                    if v9946 != 0.0 {
                        v9966 = v9947;
                    } else {
                        let v9949 = if v9941 > v9948 { 1.0 } else { 0.0 };
                        let v9967: f64;
                        if v9949 != 0.0 {
                            let v9950 = rspice_limited_exp(v9941);
                            v9967 = v9950;
                        } else {
                            let v9952 = (v9941 - v9944) / v5371;
                            let v9953 = v9952 * v9952;
                            let v9965 = rspice_limited_exp((v9944 + (v5371 * ((v9954 + (v1940 * v9952)) + (v9953 * (v9957 - (v9953 * (v5810 - v9953))))))));
                            v9967 = v9965;
                        }
                        v9966 = v9967;
                    }
                    let v9978 = v9966 * (((v1 + v9929) - v9941) - ((if ((v63 * v9928) * (((v9966 * v63) * v9928) + v9925)) >= v3982 { ((v63 * v9928) * (((v9966 * v63) * v9928) + v9925)) } else { v3982 }).ln()));
                    v10049 = v9978;
                } else {
                    let v9979 = rspice_limited_exp(v9941);
                    let v9981 = v63 * v9979;
                    let v9982 = v9981 * v9928;
                    let v9992 = v9928 + (v1 / v9924);
                    let v9998 = v9979 - (((v9981 + ((if (v9982 * (v9982 + v9925)) >= v3982 { (v9982 * (v9982 + v9925)) } else { v3982 }).ln())) - v9929) / ((v63 + (v9989 / v9979)) + (v9992 / ((v9928 * v9979) + v9924))));
                    let v9999 = v63 * v9998;
                    let v10000 = v9999 * v9928;
                    let v10006 = (v9999 + ((if (v10000 * (v10000 + v9925)) >= v3982 { (v10000 * (v10000 + v9925)) } else { v3982 }).ln())) - v9929;
                    let v10011 = (v9928 * v9998) + v9924;
                    let v10012 = v9992 / v10011;
                    let v10013 = (v63 + (v10007 / v9998)) + v10012;
                    let v10016 = v1 / v9998;
                    let v10033 = v9998 - ((v10006 / v10013) * (v1 + ((v10006 * (((v10015 * (v10016 * v10016)) - (v10019 / (((v9924 * v9924) * v9924) * v10011))) - (v10012 * v10012))) / ((v63 * v10013) * v10013))));
                    v10049 = v10033;
                }
                let v10037 = if v10034 != 0.0 && (if v9771 < v10035 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v10046: f64;
                if v10037 != 0.0 {
                    let v10040 = v10038 / (v5577 * v9771);
                    v10046 = v10040;
                } else {
                    let v10045 = v1940 * (v9772 + ((v9774 + v10041).sqrt()));
                    v10046 = v10045;
                }
                let v10047 = v10046.sqrt();
                let v10051 = ((v9771 - v9893) - v10049) - v1;
                let v10055 = if v10052 != 0.0 && (if v10051 < v10053 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v10067: f64;
                if v10055 != 0.0 {
                    let v10058 = v10056 / (v5577 * v10051);
                    v10067 = v10058;
                } else {
                    let v10060 = v10051 - v1;
                    let v10066 = v1940 * ((v10051 + v1) + (((v10060 * v10060) + v10062).sqrt()));
                    v10067 = v10066;
                }
                let v10071 = v1 + (v9738 / (v10047 + (v10067.sqrt())));
                let v10080 = v9893 - v10049;
                let v10085 = ((((((((v6853 * v10071) * v6852) * v10074) / v65) * v11) * v9669) * v9669) * (v10080 * ((v1 + v9893) + v10049))) * v6674;
                let v10086 = v10085 + v7688;
                let v10088 = v8287 * v10087;
                let v10089 = v8288 * v10087;
                let v10090 = v8291 * v10087;
                let v10093 = v65 - (v63 * v10091);
                let v10097 = v8268 * ((v11 + v9645) + v1326);
                let v10100 = ((v63 * v10071) * v11) * v4217;
                let v10102 = (v10100 * v10049) / v4287;
                let v10115 = v10102 + v10097;
                let v10120 = (v10100 * v9893) / v4287;
                let v10144 = (((((v10103 * v4217) * (v10085.abs())) * v6852) / v10067) * (((v10088 * ((if ((v10120 + v10097) / v10115) >= v3982 { ((v10120 + v10097) / v10115) } else { v3982 }).ln())) + (v10089 * (v10120 - v10102))) + ((v1940 * v10090) * ((v10120 * v10120) - (v10102 * v10102))))) + ((((((v8284 * v10085) * v10085) / (((v8545 * (v10093 * v10093)) * v10074) * v24)) * v8568) * ((v10088 + (v10089 * v10102)) + ((v10090 * v10102) * v10102))) / (v10115 * v10115));
                let v10152 = ((((v10088 * v4287) * v4217) / (((((v10074 * v24) * v10093) * v8545) * v10097) * v10097)) * v10085) * v10085;
                let v10153 = v10152 + v10144;
                let v10154 = if v10153 > v0 { 1.0 } else { 0.0 };
                let v10164: f64;
                if v10154 != 0.0 {
                    let v10162 = ((v10144 * v10152) / v10153) / (v1 + (v10157 * (v10080.powf(v10158))));
                    v10164 = v10162;
                } else {
                    v10164 = v0;
                }
                let v10165 = (v7156 * v8604) * v10164;
                v10169 = v10086;
                v10296 = v1;
                v10297 = v10165;
                v10298 = v8607;
            } else {
                v10169 = v7688;
                v10296 = v0;
                v10297 = v0;
                v10298 = v0;
            }
            let v10166 = if v7156 > v0 { 1.0 } else { 0.0 };
            if v10166 != 0.0 {
            } else {
            }
            let v10167 = if v4423 != 0.0 && v5441 != 0.0 { 1.0 } else { 0.0 };
            if v10167 != 0.0 {
                let v10168 = if v7490 == v1 { 1.0 } else { 0.0 };
                if v10168 != 0.0 {
                } else {
                }
            } else {
            }
            if v10166 != 0.0 {
            } else {
            }
            if v10166 != 0.0 {
            } else {
            }
            if v2333 != 0.0 {
            } else {
            }
            if v2337 != 0.0 {
            } else {
            }
            if v10166 != 0.0 {
            } else {
            }
            let v10170 = if v2287 != v63 { 1.0 } else { 0.0 };
            let v10172 = if v10170 != 0.0 && (if v6012 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v10299: f64;
            let v10300: f64;
            let v10301: f64;
            let v10303: f64;
            let v10305: f64;
            let v10307: f64;
            let v10309: f64;
            if v10172 != 0.0 {
                let v10176 = v7307 * v8257;
                let v10177 = v10176 * (v1 / v10173);
                let v10179 = if v6869 != 0.0 && (if v5437 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v10302: f64;
                let v10304: f64;
                let v10306: f64;
                let v10308: f64;
                let v10310: f64;
                if v10179 != 0.0 {
                    let v10183 = v10176 * (v1 / v10180);
                    let v10190 = ((v8604 * v10184) * v18) * ((v10169 / v18).powf(v10188));
                    v10302 = v1;
                    v10304 = v10183;
                    v10306 = v1;
                    v10308 = v10190;
                    v10310 = v10191;
                } else {
                    v10302 = v0;
                    v10304 = v0;
                    v10306 = v0;
                    v10308 = v0;
                    v10310 = v0;
                }
                v10299 = v1;
                v10300 = v10177;
                v10301 = v10302;
                v10303 = v10304;
                v10305 = v10306;
                v10307 = v10308;
                v10309 = v10310;
            } else {
                v10299 = v0;
                v10300 = v0;
                v10301 = v0;
                v10303 = v0;
                v10305 = v0;
                v10307 = v0;
                v10309 = v0;
            }
            let v10193 = if v10170 != 0.0 && (if v6004 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
            let v10311: f64;
            let v10312: f64;
            let v10313: f64;
            let v10315: f64;
            let v10317: f64;
            let v10319: f64;
            let v10321: f64;
            if v10193 != 0.0 {
                let v10197 = v7307 * v8257;
                let v10198 = v10197 * (v1 / v10194);
                let v10200 = if v6869 != 0.0 && (if v7056 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                let v10314: f64;
                let v10316: f64;
                let v10318: f64;
                let v10320: f64;
                let v10322: f64;
                if v10200 != 0.0 {
                    let v10204 = v10197 * (v1 / v10201);
                    let v10211 = ((v8604 * v10205) * v18) * ((v10169 / v18).powf(v10209));
                    v10314 = v1;
                    v10316 = v10204;
                    v10318 = v1;
                    v10320 = v10211;
                    v10322 = v10212;
                } else {
                    v10314 = v0;
                    v10316 = v0;
                    v10318 = v0;
                    v10320 = v0;
                    v10322 = v0;
                }
                v10311 = v1;
                v10312 = v10198;
                v10313 = v10314;
                v10315 = v10316;
                v10317 = v10318;
                v10319 = v10320;
                v10321 = v10322;
            } else {
                v10311 = v0;
                v10312 = v0;
                v10313 = v0;
                v10315 = v0;
                v10317 = v0;
                v10319 = v0;
                v10321 = v0;
            }
            let v10213 = if v4165 == v0 { 1.0 } else { 0.0 };
            let v10323: f64;
            let v10324: f64;
            if v10213 != 0.0 {
                v10323 = v0;
                v10324 = v0;
            } else {
                let v10214 = if v4165 == v63 { 1.0 } else { 0.0 };
                let v10222: f64;
                if v10214 != 0.0 {
                    let v10220 = (v10215 * v10215) / v10218;
                    v10222 = v10220;
                } else {
                    v10222 = v10218;
                }
                let v10223 = (v7307 * v8257) * v10222;
                v10323 = v1;
                v10324 = v10223;
            }
            let v10224 = if v4165 == v2427 { 1.0 } else { 0.0 };
            if v10224 != 0.0 {
            } else {
            }
            if v4201 != 0.0 {
                if v10172 != 0.0 {
                    let v10226 = if v6869 != 0.0 && (if v5437 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v10226 != 0.0 {
                    } else {
                    }
                } else {
                }
                if v10193 != 0.0 {
                    let v10228 = if v6869 != 0.0 && (if v7056 > v0 { 1.0 } else { 0.0 }) != 0.0 { 1.0 } else { 0.0 };
                    if v10228 != 0.0 {
                    } else {
                    }
                } else {
                }
            } else {
            }
            let v10325: f64;
            let v10326: f64;
            let v10327: f64;
            let v10328: f64;
            let v10329: f64;
            let v10330: f64;
            let v10331: f64;
            let v10332: f64;
            let v10333: f64;
            let v10334: f64;
            if v3979 != 0.0 {
                let v10258 = v8257 * v7307;
                let v10259 = v10258 * v10229;
                let v10260 = v10258 * v10235;
                let v10261 = v10258 * v10240;
                let v10262 = v10258 * v10252;
                let v10263 = v10258 * v10247;
                v10325 = v1;
                v10326 = v10259;
                v10327 = v1;
                v10328 = v10260;
                v10329 = v1;
                v10330 = v10261;
                v10331 = v1;
                v10332 = v10262;
                v10333 = v1;
                v10334 = v10263;
            } else {
                v10325 = v0;
                v10326 = v0;
                v10327 = v0;
                v10328 = v0;
                v10329 = v0;
                v10330 = v0;
                v10331 = v0;
                v10332 = v0;
                v10333 = v0;
                v10334 = v0;
            }
            if v3979 != 0.0 {
                let v10264 = if v4143 == v0 { 1.0 } else { 0.0 };
                if v10264 != 0.0 {
                } else {
                }
            } else {
            }
            let v10265 = if v3979 != 0.0 && v4144 != 0.0 { 1.0 } else { 0.0 };
            let v10335: f64;
            let v10336: f64;
            if v10265 != 0.0 {
                let v10269 = (v8257 * v7307) * v10266;
                v10335 = v1;
                v10336 = v10269;
            } else {
                v10335 = v0;
                v10336 = v0;
            }
            if v10265 != 0.0 {
            } else {
            }
        if v10270 == 0.0 {
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10271;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 0, value: psd }); }
            let exponent: Option<f64> = Some(v10272);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 0, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(0, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10273 == 0.0 {
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10274;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 1, value: psd }); }
            let exponent: Option<f64> = Some(v10275);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 1, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(1, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10276 == 0.0 {
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10277;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 2, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 2, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(2, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10278 == 0.0 {
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10280;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 3, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 3, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(3, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10282 == 0.0 {
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10284;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 4, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 4, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(4, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10286 == 0.0 {
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10288;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 5, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 5, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(5, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10290 == 0.0 {
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10291;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 6, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 6, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(6, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10292 == 0.0 {
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10293;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 7, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 7, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(7, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10294 == 0.0 {
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10295;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 8, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 8, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(8, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10296 == 0.0 {
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10297;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 9, value: psd }); }
            let exponent: Option<f64> = Some(v10298);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 9, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(9, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10299 == 0.0 {
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10300;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 10, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 10, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(10, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10301 == 0.0 {
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10303;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 11, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 11, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(11, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10305 == 0.0 {
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10307;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 12, value: psd }); }
            let exponent: Option<f64> = Some(v10309);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 12, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(12, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10311 == 0.0 {
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10312;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 13, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 13, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(13, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10313 == 0.0 {
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10315;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 14, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 14, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(14, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10317 == 0.0 {
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10319;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 15, value: psd }); }
            let exponent: Option<f64> = Some(v10321);
            if let Some(value) = exponent { if !value.is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "exponent", value }); } }
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 15, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(15, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10323 == 0.0 {
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10324;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 16, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 16, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(16, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10325 == 0.0 {
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10326;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 17, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 17, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(17, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10327 == 0.0 {
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10328;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 18, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 18, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(18, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10329 == 0.0 {
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10330;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 19, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 19, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(19, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10331 == 0.0 {
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10332;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 20, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 20, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(20, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10333 == 0.0 {
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10334;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 21, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 21, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(21, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        if v10335 == 0.0 {
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: false, psd: 0.0, exponent: None, table_operands: &[] }) { return Ok(()); }
        } else {
            let psd = v10336;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "psd", value: psd }); }
            if psd < 0.0 { return Err(GeneratedNoiseEvaluationError::NegativePower { index: 22, value: psd }); }
            let exponent: Option<f64> = None;
            let table_operands = [];
            let psd = psd * self.multiplicity;
            if !(psd).is_finite() { return Err(GeneratedNoiseEvaluationError::NonFinite { index: 22, quantity: "scaled psd", value: psd }); }
            if !visitor.visit(22, GeneratedNoiseEvaluationRef { active: true, psd, exponent, table_operands: &table_operands }) { return Ok(()); }
        }
        Ok(())
    }
}

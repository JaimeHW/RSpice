#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard36 != 0.0) {
            locals.var_cgidld_p = p.p351;
            locals.var_cgidld_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6290_e4832: f64 = (8.8541878176e-12 * p.p207);
            let assign6290_e4834: f64 = (assign6290_e4832 * locals.var_wecv);
            let assign6290_e4836: f64 = (assign6290_e4834 * locals.var_lecv);
            let assign6290_e4838: f64 = (assign6290_e4836 / p.p206);
            locals.var_cox_p = assign6290_e4838;
            locals.var_cox_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6300_e4844: f64 = (8.8541878176e-12 * p.p207);
            let assign6300_e4846: f64 = (assign6300_e4844 * locals.var_wecv);
            let assign6300_e4848: f64 = (assign6300_e4846 * p.p234);
            let assign6300_e4850: f64 = (assign6300_e4848 / p.p232);
            locals.var_cgov_p = assign6300_e4850;
            locals.var_cgov_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6310_e4856: f64 = (8.8541878176e-12 * p.p207);
            let assign6310_e4858: f64 = (assign6310_e4856 * locals.var_wecv);
            let assign6310_e4860: f64 = (assign6310_e4858 * p.p235);
            let assign6310_e4862: f64 = (assign6310_e4860 / p.p233);
            locals.var_cgovd_p = assign6310_e4862;
            locals.var_cgovd_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6320_e4870: f64 = (locals.var_ile).powf(p.p354);
            let assign6320_e4871: f64 = (p.p353 * assign6320_e4870);
            let assign6320_e4872: f64 = (p.p352 + assign6320_e4871);
            let assign6320_e4875: f64 = (p.p355 * locals.var_iwe);
            let assign6320_e4876: f64 = (assign6320_e4872 + assign6320_e4875);
            let assign6320_e4879: f64 = (p.p356 * locals.var_iae);
            let assign6320_e4880: f64 = (assign6320_e4876 + assign6320_e4879);
            locals.var_delvtac_p = assign6320_e4880;
            locals.var_delvtac_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6330_e4887: f64 = (p.p358 * locals.var_ile);
            let assign6330_e4888: f64 = (p.p357 + assign6330_e4887);
            let assign6330_e4891: f64 = (p.p359 * locals.var_iwe);
            let assign6330_e4892: f64 = (assign6330_e4888 + assign6330_e4891);
            let assign6330_e4895: f64 = (p.p360 * locals.var_iae);
            let assign6330_e4896: f64 = (assign6330_e4892 + assign6330_e4895);
            locals.var_facneffac_p = assign6330_e4896;
            locals.var_facneffac_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_thesataco_i = p.p294;
            locals.var_thesataco_i_rv = 0.0;
        }

        let assign6350_e4904: f64 = if param_given[361] { 1.0 } else { 0.0 };
        let assign6350_e4906: f64 = if assign6350_e4904 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign6350_e4906;
        locals.var_guard43_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard43 != 0.0)) {
            locals.var_thesataco_i = p.p361;
            locals.var_thesataco_i_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_thesatacl_i = p.p295;
            locals.var_thesatacl_i_rv = 0.0;
        }

        let assign6380_e4918: f64 = if param_given[362] { 1.0 } else { 0.0 };
        let assign6380_e4920: f64 = if assign6380_e4918 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign6380_e4920;
        locals.var_guard44_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard44 != 0.0)) {
            locals.var_thesatacl_i = p.p362;
            locals.var_thesatacl_i_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_thesataclexp_i = p.p296;
            locals.var_thesataclexp_i_rv = 0.0;
        }

        let assign6410_e4932: f64 = if param_given[363] { 1.0 } else { 0.0 };
        let assign6410_e4934: f64 = if assign6410_e4932 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign6410_e4934;
        locals.var_guard45_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard45 != 0.0)) {
            locals.var_thesataclexp_i = p.p363;
            locals.var_thesataclexp_i_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_thesatacw_i = p.p297;
            locals.var_thesatacw_i_rv = 0.0;
        }

        let assign6440_e4946: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6440_e4948: f64 = if assign6440_e4946 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign6440_e4948;
        locals.var_guard46_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard46 != 0.0)) {
            locals.var_thesatacw_i = p.p364;
            locals.var_thesatacw_i_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_thesataclw_i = p.p298;
            locals.var_thesataclw_i_rv = 0.0;
        }

        let assign6470_e4960: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6470_e4962: f64 = if assign6470_e4960 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign6470_e4962;
        locals.var_guard47_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard47 != 0.0)) {
            locals.var_thesataclw_i = p.p365;
            locals.var_thesataclw_i_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6490_e4973: f64 = (locals.var_thesatacl_i * locals.var_gwe);
            let assign6490_e4975: f64 = (assign6490_e4973 / locals.var_gpe);
            let assign6490_e4978: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
            let assign6490_e4979: f64 = (assign6490_e4975 * assign6490_e4978);
            let assign6490_e4980: f64 = (locals.var_thesataco_i + assign6490_e4979);
            let assign6490_e4984: f64 = (locals.var_thesatacw_i * locals.var_iwe);
            let assign6490_e4985: f64 = (1.0 + assign6490_e4984);
            let assign6490_e4986: f64 = (assign6490_e4980 * assign6490_e4985);
            let assign6490_e4990: f64 = (locals.var_thesataclw_i * locals.var_iae);
            let assign6490_e4991: f64 = (1.0 + assign6490_e4990);
            let assign6490_e4992: f64 = (assign6490_e4986 * assign6490_e4991);
            locals.var_thesatac_p = assign6490_e4992;
            locals.var_thesatac_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_axaco_i = p.p306;
            locals.var_axaco_i_rv = 0.0;
        }

        let assign6510_e5000: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6510_e5002: f64 = if assign6510_e5000 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign6510_e5002;
        locals.var_guard48_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard48 != 0.0)) {
            locals.var_axaco_i = p.p366;
            locals.var_axaco_i_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_axacl_i = p.p307;
            locals.var_axacl_i_rv = 0.0;
        }

        let assign6540_e5014: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6540_e5016: f64 = if assign6540_e5014 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign6540_e5016;
        locals.var_guard49_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard49 != 0.0)) {
            locals.var_axacl_i = p.p367;
            locals.var_axacl_i_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6560_e5028: f64 = (locals.var_axacl_i * locals.var_ile);
            let assign6560_e5029: f64 = (1.0 + assign6560_e5028);
            let assign6560_e5030: f64 = (locals.var_axaco_i / assign6560_e5029);
            locals.var_axac_p = assign6560_e5030;
            locals.var_axac_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6570_e5037: f64 = (locals.var_ile).powf(p.p369);
            let assign6570_e5038: f64 = (p.p368 * assign6570_e5037);
            let assign6570_e5042: f64 = (p.p370 * locals.var_iwe);
            let assign6570_e5043: f64 = (1.0 + assign6570_e5042);
            let assign6570_e5044: f64 = (assign6570_e5038 * assign6570_e5043);
            locals.var_alpac_p = assign6570_e5044;
            locals.var_alpac_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6580_e5050: f64 = (locals.var_ile).powf(p.p372);
            locals.var_tmpx = assign6580_e5050;
            locals.var_tmpx_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6590_e5056: f64 = (p.p371 * locals.var_tmpx);
            let assign6590_e5060: f64 = (p.p374 * locals.var_iwe);
            let assign6590_e5061: f64 = (1.0 + assign6590_e5060);
            let assign6590_e5062: f64 = (assign6590_e5056 * assign6590_e5061);
            let assign6590_e5066: f64 = (p.p373 * locals.var_ile);
            let assign6590_e5068: f64 = (assign6590_e5066 * locals.var_tmpx);
            let assign6590_e5069: f64 = (1.0 + assign6590_e5068);
            let assign6590_e5070: f64 = (assign6590_e5062 / assign6590_e5069);
            locals.var_alp1ac_p = assign6590_e5070;
            locals.var_alp1ac_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_fcgovacc_p = p.p375;
            locals.var_fcgovacc_p_rv = 0.0;
            locals.var_fcgovaccd_p = p.p376;
            locals.var_fcgovaccd_p_rv = 0.0;
            locals.var_cgovaccg_p = p.p377;
            locals.var_cgovaccg_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6630_e5088: f64 = (p.p378 * locals.var_iilcv);
            locals.var_cgbov_p = assign6630_e5088;
            locals.var_cgbov_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6640_e5094: f64 = (p.p379 * locals.var_iiwecv);
            locals.var_cinr_p = assign6640_e5094;
            locals.var_cinr_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6650_e5100: f64 = (p.p380 * locals.var_iiwecv);
            locals.var_cinrd_p = assign6650_e5100;
            locals.var_cinrd_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_dvfbinr_p = p.p381;
            locals.var_dvfbinr_p_rv = 0.0;
            locals.var_fcinrdep_p = p.p382;
            locals.var_fcinrdep_p_rv = 0.0;
            locals.var_fcinracc_p = p.p383;
            locals.var_fcinracc_p_rv = 0.0;
            locals.var_axinr_p = p.p384;
            locals.var_axinr_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6720_e5135: f64 = (2.0 * p.p393);
            let assign6720_e5137: f64 = (assign6720_e5135 / locals.var_le);
            let assign6720_e5138: f64 = (1.0 - assign6720_e5137);
            locals.var_temp0 = assign6720_e5138;
            locals.var_temp0_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_fnt_p = p.p387;
            locals.var_fnt_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6810_e5205: f64 = (2.0 * p.p395);
            let assign6810_e5208: f64 = (p.p396 * locals.var_we);
            let assign6810_e5209: f64 = (assign6810_e5205 + assign6810_e5208);
            locals.var_we_edge = assign6810_e5209;
            locals.var_we_edge_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_vfbedge_p = p.p397;
            locals.var_vfbedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6850_e5232: f64 = (p.p399 * locals.var_ile);
            let assign6850_e5233: f64 = (p.p398 + assign6850_e5232);
            let assign6850_e5236: f64 = (p.p400 * locals.var_iwe);
            let assign6850_e5237: f64 = (assign6850_e5233 + assign6850_e5236);
            let assign6850_e5240: f64 = (p.p401 * locals.var_iae);
            let assign6850_e5241: f64 = (assign6850_e5237 + assign6850_e5240);
            locals.var_stvfbedge_p = assign6850_e5241;
            locals.var_stvfbedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6860_e5249: f64 = (locals.var_ile).powf(p.p404);
            let assign6860_e5250: f64 = (p.p403 * assign6860_e5249);
            let assign6860_e5251: f64 = (p.p402 + assign6860_e5250);
            let assign6860_e5254: f64 = (p.p405 * locals.var_iwe);
            let assign6860_e5255: f64 = (assign6860_e5251 + assign6860_e5254);
            let assign6860_e5258: f64 = (p.p406 * locals.var_iae);
            let assign6860_e5259: f64 = (assign6860_e5255 + assign6860_e5258);
            locals.var_dphibedge_p = assign6860_e5259;
            locals.var_dphibedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6870_e5268: f64 = (locals.var_ile).powf(p.p409);
            let assign6870_e5269: f64 = (p.p408 * assign6870_e5268);
            let assign6870_e5270: f64 = (1.0 + assign6870_e5269);
            let assign6870_e5271: f64 = (p.p407 * assign6870_e5270);
            let assign6870_e5275: f64 = (p.p410 * locals.var_iwe);
            let assign6870_e5276: f64 = (1.0 + assign6870_e5275);
            let assign6870_e5277: f64 = (assign6870_e5271 * assign6870_e5276);
            let assign6870_e5281: f64 = (p.p411 * locals.var_iae);
            let assign6870_e5282: f64 = (1.0 + assign6870_e5281);
            let assign6870_e5283: f64 = (assign6870_e5277 * assign6870_e5282);
            locals.var_neffedge_p = assign6870_e5283;
            locals.var_neffedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6880_e5291: f64 = (locals.var_ile).powf(p.p414);
            let assign6880_e5292: f64 = (p.p413 * assign6880_e5291);
            let assign6880_e5293: f64 = (p.p412 + assign6880_e5292);
            locals.var_ctedge_p = assign6880_e5293;
            locals.var_ctedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6890_e5300: f64 = (p.p415 * p.p416);
            let assign6890_e5302: f64 = (assign6890_e5300 / locals.var_le);
            let assign6890_e5305: f64 = (-locals.var_le);
            let assign6890_e5307: f64 = (assign6890_e5305 / p.p416);
            let assign6890_e5308: f64 = (assign6890_e5307).exp();
            let assign6890_e5309: f64 = (1.0 - assign6890_e5308);
            let assign6890_e5310: f64 = (assign6890_e5302 * assign6890_e5309);
            let assign6890_e5311: f64 = (1.0 + assign6890_e5310);
            locals.var_gpe_edge = assign6890_e5311;
            locals.var_gpe_edge_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let (assign6900_e5320,) = {
    if (locals.var_gpe_edge > 1e-15) {
        (locals.var_gpe_edge,)
    } else {
        (1e-15,)
    }
};
            locals.var_gpe_edge = assign6900_e5320;
            locals.var_gpe_edge_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6910_e5326: f64 = (p.p256 * locals.var_we_edge);
            let assign6910_e5329: f64 = (locals.var_gpe_edge * locals.var_le);
            let assign6910_e5330: f64 = (assign6910_e5326 / assign6910_e5329);
            let assign6910_e5334: f64 = (p.p417 * locals.var_iwe);
            let assign6910_e5335: f64 = (1.0 + assign6910_e5334);
            let assign6910_e5336: f64 = (assign6910_e5330 * assign6910_e5335);
            locals.var_betnedge_p = assign6910_e5336;
            locals.var_betnedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6920_e5343: f64 = (p.p419 * locals.var_ile);
            let assign6920_e5344: f64 = (p.p418 + assign6920_e5343);
            let assign6920_e5347: f64 = (p.p420 * locals.var_iwe);
            let assign6920_e5348: f64 = (assign6920_e5344 + assign6920_e5347);
            let assign6920_e5351: f64 = (p.p421 * locals.var_iae);
            let assign6920_e5352: f64 = (assign6920_e5348 + assign6920_e5351);
            locals.var_stbetedge_p = assign6920_e5352;
            locals.var_stbetedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6930_e5359: f64 = (locals.var_ile).powf(p.p423);
            let assign6930_e5360: f64 = (p.p422 * assign6930_e5359);
            let assign6930_e5364: f64 = (p.p424 * locals.var_iwe);
            let assign6930_e5365: f64 = (1.0 + assign6930_e5364);
            let assign6930_e5366: f64 = (assign6930_e5360 * assign6930_e5365);
            locals.var_psceedge_p = assign6930_e5366;
            locals.var_psceedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_pscebedge_p = p.p425;
            locals.var_pscebedge_p_rv = 0.0;
            locals.var_pscededge_p = p.p426;
            locals.var_pscededge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6960_e5381: f64 = (locals.var_ile).powf(p.p428);
            let assign6960_e5382: f64 = (p.p427 * assign6960_e5381);
            let assign6960_e5386: f64 = (p.p429 * locals.var_iwe);
            let assign6960_e5387: f64 = (1.0 + assign6960_e5386);
            let assign6960_e5388: f64 = (assign6960_e5382 * assign6960_e5387);
            locals.var_cfedge_p = assign6960_e5388;
            locals.var_cfedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_cfdedge_p = p.p431;
            locals.var_cfdedge_p_rv = 0.0;
            locals.var_cfbedge_p = p.p430;
            locals.var_cfbedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign7040_e5429: f64 = (p.p808 * locals.var_ile);
            let assign7040_e5430: f64 = (p.p807 + assign7040_e5429);
            let assign7040_e5433: f64 = (p.p809 * locals.var_iwe);
            let assign7040_e5434: f64 = (assign7040_e5430 + assign7040_e5433);
            let assign7040_e5437: f64 = (p.p810 * locals.var_iae);
            let assign7040_e5438: f64 = (assign7040_e5434 + assign7040_e5437);
            locals.var_kvthowe = assign7040_e5438;
            locals.var_kvthowe_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign7050_e5445: f64 = (p.p812 * locals.var_ile);
            let assign7050_e5446: f64 = (p.p811 + assign7050_e5445);
            let assign7050_e5449: f64 = (p.p813 * locals.var_iwe);
            let assign7050_e5450: f64 = (assign7050_e5446 + assign7050_e5449);
            let assign7050_e5453: f64 = (p.p814 * locals.var_iae);
            let assign7050_e5454: f64 = (assign7050_e5450 + assign7050_e5453);
            locals.var_kuowe = assign7050_e5454;
            locals.var_kuowe_rv = 0.0;
        }

        let assign7170_e5570: f64 = if (((param_given[448] || param_given[449]) || param_given[450]) || param_given[451]) { 1.0 } else { 0.0 };
        locals.var_guard51 = assign7170_e5570;
        locals.var_guard51_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard51 != 0.0)) {
            let assign7180_e5577: f64 = (p.p449 * locals.var_ile);
            let assign7180_e5578: f64 = (p.p448 + assign7180_e5577);
            let assign7180_e5581: f64 = (p.p450 * locals.var_iwe);
            let assign7180_e5582: f64 = (assign7180_e5578 + assign7180_e5581);
            let assign7180_e5585: f64 = (p.p451 * locals.var_iae);
            let assign7180_e5586: f64 = (assign7180_e5582 + assign7180_e5585);
            locals.var_vfb_p = assign7180_e5586;
            locals.var_vfb_p_rv = 0.0;
        }

        let assign7190_e5607: f64 = if (((param_given[452] || param_given[453]) || param_given[454]) || param_given[455]) { 1.0 } else { 0.0 };
        locals.var_guard52 = assign7190_e5607;
        locals.var_guard52_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard52 != 0.0)) {
            let assign7200_e5614: f64 = (p.p453 * locals.var_ile);
            let assign7200_e5615: f64 = (p.p452 + assign7200_e5614);
            let assign7200_e5618: f64 = (p.p454 * locals.var_iwe);
            let assign7200_e5619: f64 = (assign7200_e5615 + assign7200_e5618);
            let assign7200_e5622: f64 = (p.p455 * locals.var_iae);
            let assign7200_e5623: f64 = (assign7200_e5619 + assign7200_e5622);
            locals.var_stvfb_p = assign7200_e5623;
            locals.var_stvfb_p_rv = 0.0;
        }

        let assign7210_e5644: f64 = if (((param_given[456] || param_given[457]) || param_given[458]) || param_given[459]) { 1.0 } else { 0.0 };
        locals.var_guard53 = assign7210_e5644;
        locals.var_guard53_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard53 != 0.0)) {
            let assign7220_e5651: f64 = (p.p457 * locals.var_ile);
            let assign7220_e5652: f64 = (p.p456 + assign7220_e5651);
            let assign7220_e5655: f64 = (p.p458 * locals.var_iwe);
            let assign7220_e5656: f64 = (assign7220_e5652 + assign7220_e5655);
            let assign7220_e5659: f64 = (p.p459 * locals.var_iae);
            let assign7220_e5660: f64 = (assign7220_e5656 + assign7220_e5659);
            locals.var_neff_p = assign7220_e5660;
            locals.var_neff_p_rv = 0.0;
        }

        let assign7230_e5681: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };
        locals.var_guard54 = assign7230_e5681;
        locals.var_guard54_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard54 != 0.0)) {
            let assign7240_e5688: f64 = (p.p461 * locals.var_ile);
            let assign7240_e5689: f64 = (p.p460 + assign7240_e5688);
            let assign7240_e5692: f64 = (p.p462 * locals.var_iwe);
            let assign7240_e5693: f64 = (assign7240_e5689 + assign7240_e5692);
            let assign7240_e5696: f64 = (p.p463 * locals.var_iae);
            let assign7240_e5697: f64 = (assign7240_e5693 + assign7240_e5696);
            locals.var_gfacnud_p = assign7240_e5697;
            locals.var_gfacnud_p_rv = 0.0;
        }

        let assign7250_e5718: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };
        locals.var_guard55 = assign7250_e5718;
        locals.var_guard55_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard36 != 0.0) && (locals.var_guard55 != 0.0)) {
            let assign7260_e5725: f64 = (p.p465 * locals.var_ile);
            let assign7260_e5726: f64 = (p.p464 + assign7260_e5725);
            let assign7260_e5729: f64 = (p.p466 * locals.var_iwe);
            let assign7260_e5730: f64 = (assign7260_e5726 + assign7260_e5729);
            let assign7260_e5733: f64 = (p.p467 * locals.var_iae);
            let assign7260_e5734: f64 = (assign7260_e5730 + assign7260_e5733);
            locals.var_vsbnud_p = assign7260_e5734;
            locals.var_vsbnud_p_rv = 0.0;
        }

        let assign7270_e5755: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };
        locals.var_guard56 = assign7270_e5755;
        locals.var_guard56_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard56 != 0.0)) {
            let assign7280_e5762: f64 = (p.p469 * locals.var_ile);
            let assign7280_e5763: f64 = (p.p468 + assign7280_e5762);
            let assign7280_e5766: f64 = (p.p470 * locals.var_iwe);
            let assign7280_e5767: f64 = (assign7280_e5763 + assign7280_e5766);
            let assign7280_e5770: f64 = (p.p471 * locals.var_iae);
            let assign7280_e5771: f64 = (assign7280_e5767 + assign7280_e5770);
            locals.var_dphib_p = assign7280_e5771;
            locals.var_dphib_p_rv = 0.0;
        }

        let assign7290_e5792: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };
        locals.var_guard57 = assign7290_e5792;
        locals.var_guard57_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard57 != 0.0)) {
            let assign7300_e5799: f64 = (p.p473 * locals.var_ile);
            let assign7300_e5800: f64 = (p.p472 + assign7300_e5799);
            let assign7300_e5803: f64 = (p.p474 * locals.var_iwe);
            let assign7300_e5804: f64 = (assign7300_e5800 + assign7300_e5803);
            let assign7300_e5807: f64 = (p.p475 * locals.var_iae);
            let assign7300_e5808: f64 = (assign7300_e5804 + assign7300_e5807);
            locals.var_np_p = assign7300_e5808;
            locals.var_np_p_rv = 0.0;
        }

        let assign7310_e5829: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };
        locals.var_guard58 = assign7310_e5829;
        locals.var_guard58_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard58 != 0.0)) {
            let assign7320_e5836: f64 = (p.p477 * locals.var_ile);
            let assign7320_e5837: f64 = (p.p476 + assign7320_e5836);
            let assign7320_e5840: f64 = (p.p478 * locals.var_iwe);
            let assign7320_e5841: f64 = (assign7320_e5837 + assign7320_e5840);
            let assign7320_e5844: f64 = (p.p479 * locals.var_iae);
            let assign7320_e5845: f64 = (assign7320_e5841 + assign7320_e5844);
            locals.var_nov_p = assign7320_e5845;
            locals.var_nov_p_rv = 0.0;
        }

        let assign7330_e5866: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };
        locals.var_guard59 = assign7330_e5866;
        locals.var_guard59_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard59 != 0.0)) {
            let assign7340_e5873: f64 = (p.p481 * locals.var_ile);
            let assign7340_e5874: f64 = (p.p480 + assign7340_e5873);
            let assign7340_e5877: f64 = (p.p482 * locals.var_iwe);
            let assign7340_e5878: f64 = (assign7340_e5874 + assign7340_e5877);
            let assign7340_e5881: f64 = (p.p483 * locals.var_iae);
            let assign7340_e5882: f64 = (assign7340_e5878 + assign7340_e5881);
            locals.var_novd_p = assign7340_e5882;
            locals.var_novd_p_rv = 0.0;
        }

        let assign7350_e5903: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };
        locals.var_guard60 = assign7350_e5903;
        locals.var_guard60_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard60 != 0.0)) {
            let assign7360_e5910: f64 = (p.p485 * locals.var_ile);
            let assign7360_e5911: f64 = (p.p484 + assign7360_e5910);
            let assign7360_e5914: f64 = (p.p486 * locals.var_iwe);
            let assign7360_e5915: f64 = (assign7360_e5911 + assign7360_e5914);
            let assign7360_e5918: f64 = (p.p487 * locals.var_iae);
            let assign7360_e5919: f64 = (assign7360_e5915 + assign7360_e5918);
            locals.var_ct_p = assign7360_e5919;
            locals.var_ct_p_rv = 0.0;
        }

        let assign7370_e5940: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };
        locals.var_guard61 = assign7370_e5940;
        locals.var_guard61_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard61 != 0.0)) {
            let assign7380_e5947: f64 = (p.p493 * locals.var_ile);
            let assign7380_e5948: f64 = (p.p492 + assign7380_e5947);
            let assign7380_e5951: f64 = (p.p494 * locals.var_iwe);
            let assign7380_e5952: f64 = (assign7380_e5948 + assign7380_e5951);
            let assign7380_e5955: f64 = (p.p495 * locals.var_iae);
            let assign7380_e5956: f64 = (assign7380_e5952 + assign7380_e5955);
            locals.var_ctg_p = assign7380_e5956;
            locals.var_ctg_p_rv = 0.0;
        }

        let assign7390_e5977: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };
        locals.var_guard62 = assign7390_e5977;
        locals.var_guard62_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard62 != 0.0)) {
            let assign7400_e5984: f64 = (p.p489 * locals.var_ile);
            let assign7400_e5985: f64 = (p.p488 + assign7400_e5984);
            let assign7400_e5988: f64 = (p.p490 * locals.var_iwe);
            let assign7400_e5989: f64 = (assign7400_e5985 + assign7400_e5988);
            let assign7400_e5992: f64 = (p.p491 * locals.var_iae);
            let assign7400_e5993: f64 = (assign7400_e5989 + assign7400_e5992);
            locals.var_ctb_p = assign7400_e5993;
            locals.var_ctb_p_rv = 0.0;
        }

        let assign7410_e6014: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };
        locals.var_guard63 = assign7410_e6014;
        locals.var_guard63_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard63 != 0.0)) {
            let assign7420_e6021: f64 = (p.p497 * locals.var_ile);
            let assign7420_e6022: f64 = (p.p496 + assign7420_e6021);
            let assign7420_e6025: f64 = (p.p498 * locals.var_iwe);
            let assign7420_e6026: f64 = (assign7420_e6022 + assign7420_e6025);
            let assign7420_e6029: f64 = (p.p499 * locals.var_iae);
            let assign7420_e6030: f64 = (assign7420_e6026 + assign7420_e6029);
            locals.var_stct_p = assign7420_e6030;
            locals.var_stct_p_rv = 0.0;
        }

        let assign7430_e6051: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };
        locals.var_guard64 = assign7430_e6051;
        locals.var_guard64_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard64 != 0.0)) {
            let assign7440_e6059: f64 = (p.p501 * locals.var_ile);
            let assign7440_e6060: f64 = (p.p500 + assign7440_e6059);
            let assign7440_e6063: f64 = (p.p502 * locals.var_iwe);
            let assign7440_e6064: f64 = (assign7440_e6060 + assign7440_e6063);
            let assign7440_e6067: f64 = (p.p503 * locals.var_iae);
            let assign7440_e6068: f64 = (assign7440_e6064 + assign7440_e6067);
            let assign7440_e6069: f64 = (locals.var_ile2 * assign7440_e6068);
            locals.var_cf_p = assign7440_e6069;
            locals.var_cf_p_rv = 0.0;
        }

        let assign7450_e6090: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };
        locals.var_guard65 = assign7450_e6090;
        locals.var_guard65_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard65 != 0.0)) {
            let assign7460_e6097: f64 = (p.p509 * locals.var_ile);
            let assign7460_e6098: f64 = (p.p508 + assign7460_e6097);
            let assign7460_e6101: f64 = (p.p510 * locals.var_iwe);
            let assign7460_e6102: f64 = (assign7460_e6098 + assign7460_e6101);
            let assign7460_e6105: f64 = (p.p511 * locals.var_iae);
            let assign7460_e6106: f64 = (assign7460_e6102 + assign7460_e6105);
            locals.var_cfd_p = assign7460_e6106;
            locals.var_cfd_p_rv = 0.0;
        }

        let assign7470_e6127: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };
        locals.var_guard66 = assign7470_e6127;
        locals.var_guard66_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard66 != 0.0)) {
            let assign7480_e6134: f64 = (p.p505 * locals.var_ile);
            let assign7480_e6135: f64 = (p.p504 + assign7480_e6134);
            let assign7480_e6138: f64 = (p.p506 * locals.var_iwe);
            let assign7480_e6139: f64 = (assign7480_e6135 + assign7480_e6138);
            let assign7480_e6142: f64 = (p.p507 * locals.var_iae);
            let assign7480_e6143: f64 = (assign7480_e6139 + assign7480_e6142);
            locals.var_cfb_p = assign7480_e6143;
            locals.var_cfb_p_rv = 0.0;
        }

        let assign7490_e6164: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };
        locals.var_guard67 = assign7490_e6164;
        locals.var_guard67_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard67 != 0.0)) {
            let assign7500_e6172: f64 = (p.p513 * locals.var_ile);
            let assign7500_e6173: f64 = (p.p512 + assign7500_e6172);
            let assign7500_e6176: f64 = (p.p514 * locals.var_iwe);
            let assign7500_e6177: f64 = (assign7500_e6173 + assign7500_e6176);
            let assign7500_e6180: f64 = (p.p515 * locals.var_iae);
            let assign7500_e6181: f64 = (assign7500_e6177 + assign7500_e6180);
            let assign7500_e6182: f64 = (locals.var_ile2 * assign7500_e6181);
            locals.var_psce_p = assign7500_e6182;
            locals.var_psce_p_rv = 0.0;
        }

        let assign7510_e6203: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };
        locals.var_guard68 = assign7510_e6203;
        locals.var_guard68_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard68 != 0.0)) {
            let assign7520_e6210: f64 = (p.p521 * locals.var_ile);
            let assign7520_e6211: f64 = (p.p520 + assign7520_e6210);
            let assign7520_e6214: f64 = (p.p522 * locals.var_iwe);
            let assign7520_e6215: f64 = (assign7520_e6211 + assign7520_e6214);
            let assign7520_e6218: f64 = (p.p523 * locals.var_iae);
            let assign7520_e6219: f64 = (assign7520_e6215 + assign7520_e6218);
            locals.var_psced_p = assign7520_e6219;
            locals.var_psced_p_rv = 0.0;
        }

        let assign7530_e6240: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };
        locals.var_guard69 = assign7530_e6240;
        locals.var_guard69_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard69 != 0.0)) {
            let assign7540_e6247: f64 = (p.p517 * locals.var_ile);
            let assign7540_e6248: f64 = (p.p516 + assign7540_e6247);
            let assign7540_e6251: f64 = (p.p518 * locals.var_iwe);
            let assign7540_e6252: f64 = (assign7540_e6248 + assign7540_e6251);
            let assign7540_e6255: f64 = (p.p519 * locals.var_iae);
            let assign7540_e6256: f64 = (assign7540_e6252 + assign7540_e6255);
            locals.var_psceb_p = assign7540_e6256;
            locals.var_psceb_p_rv = 0.0;
        }

        let assign7550_e6277: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };
        locals.var_guard70 = assign7550_e6277;
        locals.var_guard70_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard70 != 0.0)) {
            let assign7560_e6283: f64 = (locals.var_we / locals.var_le);
            let assign7560_e6287: f64 = (p.p525 * locals.var_ile);
            let assign7560_e6288: f64 = (p.p524 + assign7560_e6287);
            let assign7560_e6291: f64 = (p.p526 * locals.var_iwe);
            let assign7560_e6292: f64 = (assign7560_e6288 + assign7560_e6291);
            let assign7560_e6295: f64 = (p.p527 * locals.var_iae);
            let assign7560_e6296: f64 = (assign7560_e6292 + assign7560_e6295);
            let assign7560_e6297: f64 = (assign7560_e6283 * assign7560_e6296);
            locals.var_betn_p = assign7560_e6297;
            locals.var_betn_p_rv = 0.0;
        }

        let assign7570_e6318: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };
        locals.var_guard71 = assign7570_e6318;
        locals.var_guard71_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard71 != 0.0)) {
            let assign7580_e6325: f64 = (p.p529 * locals.var_ile);
            let assign7580_e6326: f64 = (p.p528 + assign7580_e6325);
            let assign7580_e6329: f64 = (p.p530 * locals.var_iwe);
            let assign7580_e6330: f64 = (assign7580_e6326 + assign7580_e6329);
            let assign7580_e6333: f64 = (p.p531 * locals.var_iae);
            let assign7580_e6334: f64 = (assign7580_e6330 + assign7580_e6333);
            locals.var_stbet_p = assign7580_e6334;
            locals.var_stbet_p_rv = 0.0;
        }

        let assign7590_e6355: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };
        locals.var_guard72 = assign7590_e6355;
        locals.var_guard72_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard72 != 0.0)) {
            let assign7600_e6362: f64 = (p.p533 * locals.var_ile);
            let assign7600_e6363: f64 = (p.p532 + assign7600_e6362);
            let assign7600_e6366: f64 = (p.p534 * locals.var_iwe);
            let assign7600_e6367: f64 = (assign7600_e6363 + assign7600_e6366);
            let assign7600_e6370: f64 = (p.p535 * locals.var_iae);
            let assign7600_e6371: f64 = (assign7600_e6367 + assign7600_e6370);
            locals.var_mue_p = assign7600_e6371;
            locals.var_mue_p_rv = 0.0;
        }

        let assign7610_e6392: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };
        locals.var_guard73 = assign7610_e6392;
        locals.var_guard73_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard73 != 0.0)) {
            let assign7620_e6399: f64 = (p.p537 * locals.var_ile);
            let assign7620_e6400: f64 = (p.p536 + assign7620_e6399);
            let assign7620_e6403: f64 = (p.p538 * locals.var_iwe);
            let assign7620_e6404: f64 = (assign7620_e6400 + assign7620_e6403);
            let assign7620_e6407: f64 = (p.p539 * locals.var_iae);
            let assign7620_e6408: f64 = (assign7620_e6404 + assign7620_e6407);
            locals.var_themu_p = assign7620_e6408;
            locals.var_themu_p_rv = 0.0;
        }

        let assign7630_e6429: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign7630_e6429;
        locals.var_guard74_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard74 != 0.0)) {
            let assign7640_e6436: f64 = (p.p541 * locals.var_ile);
            let assign7640_e6437: f64 = (p.p540 + assign7640_e6436);
            let assign7640_e6440: f64 = (p.p542 * locals.var_iwe);
            let assign7640_e6441: f64 = (assign7640_e6437 + assign7640_e6440);
            let assign7640_e6444: f64 = (p.p543 * locals.var_iae);
            let assign7640_e6445: f64 = (assign7640_e6441 + assign7640_e6444);
            locals.var_cs_p = assign7640_e6445;
            locals.var_cs_p_rv = 0.0;
        }

        let assign7650_e6466: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign7650_e6466;
        locals.var_guard75_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard75 != 0.0)) {
            let assign7660_e6473: f64 = (p.p545 * locals.var_ile);
            let assign7660_e6474: f64 = (p.p544 + assign7660_e6473);
            let assign7660_e6477: f64 = (p.p546 * locals.var_iwe);
            let assign7660_e6478: f64 = (assign7660_e6474 + assign7660_e6477);
            let assign7660_e6481: f64 = (p.p547 * locals.var_iae);
            let assign7660_e6482: f64 = (assign7660_e6478 + assign7660_e6481);
            locals.var_thecs_p = assign7660_e6482;
            locals.var_thecs_p_rv = 0.0;
        }

        let assign7670_e6503: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign7670_e6503;
        locals.var_guard76_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard76 != 0.0)) {
            let assign7680_e6510: f64 = (p.p549 * locals.var_ile);
            let assign7680_e6511: f64 = (p.p548 + assign7680_e6510);
            let assign7680_e6514: f64 = (p.p550 * locals.var_iwe);
            let assign7680_e6515: f64 = (assign7680_e6511 + assign7680_e6514);
            let assign7680_e6518: f64 = (p.p551 * locals.var_iae);
            let assign7680_e6519: f64 = (assign7680_e6515 + assign7680_e6518);
            locals.var_xcor_p = assign7680_e6519;
            locals.var_xcor_p_rv = 0.0;
        }

        let assign7690_e6540: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign7690_e6540;
        locals.var_guard77_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard77 != 0.0)) {
            let assign7700_e6548: f64 = (p.p553 * locals.var_ile);
            let assign7700_e6549: f64 = (p.p552 + assign7700_e6548);
            let assign7700_e6552: f64 = (p.p554 * locals.var_iwe);
            let assign7700_e6553: f64 = (assign7700_e6549 + assign7700_e6552);
            let assign7700_e6556: f64 = (p.p555 * locals.var_iae);
            let assign7700_e6557: f64 = (assign7700_e6553 + assign7700_e6556);
            let assign7700_e6558: f64 = (locals.var_iwe * assign7700_e6557);
            locals.var_rs_p = assign7700_e6558;
            locals.var_rs_p_rv = 0.0;
        }

        let assign7710_e6579: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign7710_e6579;
        locals.var_guard78_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard78 != 0.0)) {
            let assign7720_e6586: f64 = (p.p557 * locals.var_ile);
            let assign7720_e6587: f64 = (p.p556 + assign7720_e6586);
            let assign7720_e6590: f64 = (p.p558 * locals.var_iwe);
            let assign7720_e6591: f64 = (assign7720_e6587 + assign7720_e6590);
            let assign7720_e6594: f64 = (p.p559 * locals.var_iae);
            let assign7720_e6595: f64 = (assign7720_e6591 + assign7720_e6594);
            locals.var_strs_p = assign7720_e6595;
            locals.var_strs_p_rv = 0.0;
        }

        let assign7730_e6616: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };
        locals.var_guard79 = assign7730_e6616;
        locals.var_guard79_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard79 != 0.0)) {
            let assign7740_e6623: f64 = (p.p561 * locals.var_ile);
            let assign7740_e6624: f64 = (p.p560 + assign7740_e6623);
            let assign7740_e6627: f64 = (p.p562 * locals.var_iwe);
            let assign7740_e6628: f64 = (assign7740_e6624 + assign7740_e6627);
            let assign7740_e6631: f64 = (p.p563 * locals.var_iae);
            let assign7740_e6632: f64 = (assign7740_e6628 + assign7740_e6631);
            locals.var_rsb_p = assign7740_e6632;
            locals.var_rsb_p_rv = 0.0;
        }

        let assign7750_e6653: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };
        locals.var_guard80 = assign7750_e6653;
        locals.var_guard80_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard80 != 0.0)) {
            let assign7760_e6660: f64 = (p.p565 * locals.var_ile);
            let assign7760_e6661: f64 = (p.p564 + assign7760_e6660);
            let assign7760_e6664: f64 = (p.p566 * locals.var_iwe);
            let assign7760_e6665: f64 = (assign7760_e6661 + assign7760_e6664);
            let assign7760_e6668: f64 = (p.p567 * locals.var_iae);
            let assign7760_e6669: f64 = (assign7760_e6665 + assign7760_e6668);
            locals.var_rsg_p = assign7760_e6669;
            locals.var_rsg_p_rv = 0.0;
        }

        let assign7770_e6690: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        locals.var_guard81 = assign7770_e6690;
        locals.var_guard81_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard81 != 0.0)) {
            let assign7780_e6698: f64 = (p.p569 * locals.var_ile);
            let assign7780_e6699: f64 = (p.p568 + assign7780_e6698);
            let assign7780_e6702: f64 = (p.p570 * locals.var_iwe);
            let assign7780_e6703: f64 = (assign7780_e6699 + assign7780_e6702);
            let assign7780_e6706: f64 = (p.p571 * locals.var_iae);
            let assign7780_e6707: f64 = (assign7780_e6703 + assign7780_e6706);
            let assign7780_e6708: f64 = (locals.var_ile * assign7780_e6707);
            locals.var_thesat_p = assign7780_e6708;
            locals.var_thesat_p_rv = 0.0;
        }

        let assign7790_e6729: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };
        locals.var_guard82 = assign7790_e6729;
        locals.var_guard82_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard82 != 0.0)) {
            let assign7800_e6736: f64 = (p.p573 * locals.var_ile);
            let assign7800_e6737: f64 = (p.p572 + assign7800_e6736);
            let assign7800_e6740: f64 = (p.p574 * locals.var_iwe);
            let assign7800_e6741: f64 = (assign7800_e6737 + assign7800_e6740);
            let assign7800_e6744: f64 = (p.p575 * locals.var_iae);
            let assign7800_e6745: f64 = (assign7800_e6741 + assign7800_e6744);
            locals.var_stthesat_p = assign7800_e6745;
            locals.var_stthesat_p_rv = 0.0;
        }

        let assign7810_e6766: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };
        locals.var_guard83 = assign7810_e6766;
        locals.var_guard83_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard83 != 0.0)) {
            let assign7820_e6773: f64 = (p.p577 * locals.var_ile);
            let assign7820_e6774: f64 = (p.p576 + assign7820_e6773);
            let assign7820_e6777: f64 = (p.p578 * locals.var_iwe);
            let assign7820_e6778: f64 = (assign7820_e6774 + assign7820_e6777);
            let assign7820_e6781: f64 = (p.p579 * locals.var_iae);
            let assign7820_e6782: f64 = (assign7820_e6778 + assign7820_e6781);
            locals.var_thesatb_p = assign7820_e6782;
            locals.var_thesatb_p_rv = 0.0;
        }

        let assign7830_e6803: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        locals.var_guard84 = assign7830_e6803;
        locals.var_guard84_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard84 != 0.0)) {
            let assign7840_e6810: f64 = (p.p581 * locals.var_ile);
            let assign7840_e6811: f64 = (p.p580 + assign7840_e6810);
            let assign7840_e6814: f64 = (p.p582 * locals.var_iwe);
            let assign7840_e6815: f64 = (assign7840_e6811 + assign7840_e6814);
            let assign7840_e6818: f64 = (p.p583 * locals.var_iae);
            let assign7840_e6819: f64 = (assign7840_e6815 + assign7840_e6818);
            locals.var_thesatg_p = assign7840_e6819;
            locals.var_thesatg_p_rv = 0.0;
        }

        let assign7850_e6840: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        locals.var_guard85 = assign7850_e6840;
        locals.var_guard85_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard85 != 0.0)) {
            let assign7860_e6847: f64 = (p.p585 * locals.var_ile);
            let assign7860_e6848: f64 = (p.p584 + assign7860_e6847);
            let assign7860_e6851: f64 = (p.p586 * locals.var_iwe);
            let assign7860_e6852: f64 = (assign7860_e6848 + assign7860_e6851);
            let assign7860_e6855: f64 = (p.p587 * locals.var_iae);
            let assign7860_e6856: f64 = (assign7860_e6852 + assign7860_e6855);
            locals.var_ax_p = assign7860_e6856;
            locals.var_ax_p_rv = 0.0;
        }

        let assign7870_e6877: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };
        locals.var_guard86 = assign7870_e6877;
        locals.var_guard86_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard86 != 0.0)) {
            let assign7880_e6885: f64 = (p.p589 * locals.var_ile);
            let assign7880_e6886: f64 = (p.p588 + assign7880_e6885);
            let assign7880_e6889: f64 = (p.p590 * locals.var_iwe);
            let assign7880_e6890: f64 = (assign7880_e6886 + assign7880_e6889);
            let assign7880_e6893: f64 = (p.p591 * locals.var_iae);
            let assign7880_e6894: f64 = (assign7880_e6890 + assign7880_e6893);
            let assign7880_e6895: f64 = (locals.var_ile * assign7880_e6894);
            locals.var_alp_p = assign7880_e6895;
            locals.var_alp_p_rv = 0.0;
        }

        let assign7890_e6916: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };
        locals.var_guard87 = assign7890_e6916;
        locals.var_guard87_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard87 != 0.0)) {
            let assign7900_e6923: f64 = (p.p593 * locals.var_ile);
            let assign7900_e6924: f64 = (p.p592 + assign7900_e6923);
            let assign7900_e6927: f64 = (p.p594 * locals.var_iwe);
            let assign7900_e6928: f64 = (assign7900_e6924 + assign7900_e6927);
            let assign7900_e6931: f64 = (p.p595 * locals.var_iae);
            let assign7900_e6932: f64 = (assign7900_e6928 + assign7900_e6931);
            locals.var_alp1_p = assign7900_e6932;
            locals.var_alp1_p_rv = 0.0;
        }

        let assign7910_e6953: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7910_e6953;
        locals.var_guard88_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard36 != 0.0) && (locals.var_guard88 != 0.0)) {
            let assign7920_e6960: f64 = (p.p597 * locals.var_ile);
            let assign7920_e6961: f64 = (p.p596 + assign7920_e6960);
            let assign7920_e6964: f64 = (p.p598 * locals.var_iwe);
            let assign7920_e6965: f64 = (assign7920_e6961 + assign7920_e6964);
            let assign7920_e6968: f64 = (p.p599 * locals.var_iae);
            let assign7920_e6969: f64 = (assign7920_e6965 + assign7920_e6968);
            locals.var_alp2_p = assign7920_e6969;
            locals.var_alp2_p_rv = 0.0;
        }

        let assign7930_e6990: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7930_e6990;
        locals.var_guard89_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard89 != 0.0)) {
            let assign7940_e6997: f64 = (p.p601 * locals.var_ile);
            let assign7940_e6998: f64 = (p.p600 + assign7940_e6997);
            let assign7940_e7001: f64 = (p.p602 * locals.var_iwe);
            let assign7940_e7002: f64 = (assign7940_e6998 + assign7940_e7001);
            let assign7940_e7005: f64 = (p.p603 * locals.var_iae);
            let assign7940_e7006: f64 = (assign7940_e7002 + assign7940_e7005);
            locals.var_a1_p = assign7940_e7006;
            locals.var_a1_p_rv = 0.0;
        }

        let assign7950_e7027: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };
        locals.var_guard90 = assign7950_e7027;
        locals.var_guard90_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard90 != 0.0)) {
            let assign7960_e7034: f64 = (p.p605 * locals.var_ile);
            let assign7960_e7035: f64 = (p.p604 + assign7960_e7034);
            let assign7960_e7038: f64 = (p.p606 * locals.var_iwe);
            let assign7960_e7039: f64 = (assign7960_e7035 + assign7960_e7038);
            let assign7960_e7042: f64 = (p.p607 * locals.var_iae);
            let assign7960_e7043: f64 = (assign7960_e7039 + assign7960_e7042);
            locals.var_sta2_p = assign7960_e7043;
            locals.var_sta2_p_rv = 0.0;
        }

        let assign7970_e7064: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };
        locals.var_guard91 = assign7970_e7064;
        locals.var_guard91_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard91 != 0.0)) {
            let assign7980_e7071: f64 = (p.p609 * locals.var_ile);
            let assign7980_e7072: f64 = (p.p608 + assign7980_e7071);
            let assign7980_e7075: f64 = (p.p610 * locals.var_iwe);
            let assign7980_e7076: f64 = (assign7980_e7072 + assign7980_e7075);
            let assign7980_e7079: f64 = (p.p611 * locals.var_iae);
            let assign7980_e7080: f64 = (assign7980_e7076 + assign7980_e7079);
            locals.var_a3_p = assign7980_e7080;
            locals.var_a3_p_rv = 0.0;
        }

        let assign7990_e7101: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };
        locals.var_guard92 = assign7990_e7101;
        locals.var_guard92_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard92 != 0.0)) {
            let assign8000_e7108: f64 = (p.p613 * locals.var_ile);
            let assign8000_e7109: f64 = (p.p612 + assign8000_e7108);
            let assign8000_e7112: f64 = (p.p614 * locals.var_iwe);
            let assign8000_e7113: f64 = (assign8000_e7109 + assign8000_e7112);
            let assign8000_e7116: f64 = (p.p615 * locals.var_iae);
            let assign8000_e7117: f64 = (assign8000_e7113 + assign8000_e7116);
            locals.var_a4_p = assign8000_e7117;
            locals.var_a4_p_rv = 0.0;
        }

        let assign8010_e7138: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };
        locals.var_guard93 = assign8010_e7138;
        locals.var_guard93_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard93 != 0.0)) {
            let assign8020_e7146: f64 = (p.p617 * locals.var_ile);
            let assign8020_e7147: f64 = (p.p616 + assign8020_e7146);
            let assign8020_e7150: f64 = (p.p618 * locals.var_iwe);
            let assign8020_e7151: f64 = (assign8020_e7147 + assign8020_e7150);
            let assign8020_e7154: f64 = (p.p619 * locals.var_iae);
            let assign8020_e7155: f64 = (assign8020_e7151 + assign8020_e7154);
            let assign8020_e7156: f64 = (locals.var_iiae * assign8020_e7155);
            locals.var_iginv_p = assign8020_e7156;
            locals.var_iginv_p_rv = 0.0;
        }

        let assign8030_e7177: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };
        locals.var_guard94 = assign8030_e7177;
        locals.var_guard94_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard94 != 0.0)) {
            let assign8040_e7185: f64 = (p.p621 * locals.var_ile);
            let assign8040_e7186: f64 = (p.p620 + assign8040_e7185);
            let assign8040_e7189: f64 = (p.p622 * locals.var_iwe);
            let assign8040_e7190: f64 = (assign8040_e7186 + assign8040_e7189);
            let assign8040_e7193: f64 = (p.p623 * locals.var_iae);
            let assign8040_e7194: f64 = (assign8040_e7190 + assign8040_e7193);
            let assign8040_e7195: f64 = (locals.var_iiwe * assign8040_e7194);
            locals.var_igov_p = assign8040_e7195;
            locals.var_igov_p_rv = 0.0;
        }

        let assign8050_e7216: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };
        locals.var_guard95 = assign8050_e7216;
        locals.var_guard95_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard95 != 0.0)) {
            let assign8060_e7224: f64 = (p.p625 * locals.var_ile);
            let assign8060_e7225: f64 = (p.p624 + assign8060_e7224);
            let assign8060_e7228: f64 = (p.p626 * locals.var_iwe);
            let assign8060_e7229: f64 = (assign8060_e7225 + assign8060_e7228);
            let assign8060_e7232: f64 = (p.p627 * locals.var_iae);
            let assign8060_e7233: f64 = (assign8060_e7229 + assign8060_e7232);
            let assign8060_e7234: f64 = (locals.var_iiwe * assign8060_e7233);
            locals.var_igovd_p = assign8060_e7234;
            locals.var_igovd_p_rv = 0.0;
        }

        let assign8070_e7255: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };
        locals.var_guard96 = assign8070_e7255;
        locals.var_guard96_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard96 != 0.0)) {
            let assign8080_e7262: f64 = (p.p629 * locals.var_ile);
            let assign8080_e7263: f64 = (p.p628 + assign8080_e7262);
            let assign8080_e7266: f64 = (p.p630 * locals.var_iwe);
            let assign8080_e7267: f64 = (assign8080_e7263 + assign8080_e7266);
            let assign8080_e7270: f64 = (p.p631 * locals.var_iae);
            let assign8080_e7271: f64 = (assign8080_e7267 + assign8080_e7270);
            locals.var_stig_p = assign8080_e7271;
            locals.var_stig_p_rv = 0.0;
        }

        let assign8090_e7292: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };
        locals.var_guard97 = assign8090_e7292;
        locals.var_guard97_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard97 != 0.0)) {
            let assign8100_e7300: f64 = (p.p633 * locals.var_ile);
            let assign8100_e7301: f64 = (p.p632 + assign8100_e7300);
            let assign8100_e7304: f64 = (p.p634 * locals.var_iwe);
            let assign8100_e7305: f64 = (assign8100_e7301 + assign8100_e7304);
            let assign8100_e7308: f64 = (p.p635 * locals.var_iae);
            let assign8100_e7309: f64 = (assign8100_e7305 + assign8100_e7308);
            let assign8100_e7310: f64 = (locals.var_iiwe * assign8100_e7309);
            locals.var_agidl_p = assign8100_e7310;
            locals.var_agidl_p_rv = 0.0;
        }

        let assign8110_e7331: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };
        locals.var_guard98 = assign8110_e7331;
        locals.var_guard98_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard98 != 0.0)) {
            let assign8120_e7339: f64 = (p.p637 * locals.var_ile);
            let assign8120_e7340: f64 = (p.p636 + assign8120_e7339);
            let assign8120_e7343: f64 = (p.p638 * locals.var_iwe);
            let assign8120_e7344: f64 = (assign8120_e7340 + assign8120_e7343);
            let assign8120_e7347: f64 = (p.p639 * locals.var_iae);
            let assign8120_e7348: f64 = (assign8120_e7344 + assign8120_e7347);
            let assign8120_e7349: f64 = (locals.var_iiwe * assign8120_e7348);
            locals.var_agidld_p = assign8120_e7349;
            locals.var_agidld_p_rv = 0.0;
        }

        let assign8130_e7370: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign8130_e7370;
        locals.var_guard99_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard99 != 0.0)) {
            let assign8140_e7377: f64 = (p.p641 * locals.var_ile);
            let assign8140_e7378: f64 = (p.p640 + assign8140_e7377);
            let assign8140_e7381: f64 = (p.p642 * locals.var_iwe);
            let assign8140_e7382: f64 = (assign8140_e7378 + assign8140_e7381);
            let assign8140_e7385: f64 = (p.p643 * locals.var_iae);
            let assign8140_e7386: f64 = (assign8140_e7382 + assign8140_e7385);
            locals.var_stbgidl_p = assign8140_e7386;
            locals.var_stbgidl_p_rv = 0.0;
        }

        let assign8150_e7407: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };
        locals.var_guard100 = assign8150_e7407;
        locals.var_guard100_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard100 != 0.0)) {
            let assign8160_e7414: f64 = (p.p645 * locals.var_ile);
            let assign8160_e7415: f64 = (p.p644 + assign8160_e7414);
            let assign8160_e7418: f64 = (p.p646 * locals.var_iwe);
            let assign8160_e7419: f64 = (assign8160_e7415 + assign8160_e7418);
            let assign8160_e7422: f64 = (p.p647 * locals.var_iae);
            let assign8160_e7423: f64 = (assign8160_e7419 + assign8160_e7422);
            locals.var_stbgidld_p = assign8160_e7423;
            locals.var_stbgidld_p_rv = 0.0;
        }

        let assign8170_e7444: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };
        locals.var_guard101 = assign8170_e7444;
        locals.var_guard101_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard101 != 0.0)) {
            let assign8180_e7450: f64 = (locals.var_iiwecv * locals.var_lecv);
            let assign8180_e7452: f64 = (assign8180_e7450 / 1e-6);
            let assign8180_e7456: f64 = (p.p649 * locals.var_ile);
            let assign8180_e7457: f64 = (p.p648 + assign8180_e7456);
            let assign8180_e7460: f64 = (p.p650 * locals.var_iwe);
            let assign8180_e7461: f64 = (assign8180_e7457 + assign8180_e7460);
            let assign8180_e7464: f64 = (p.p651 * locals.var_iae);
            let assign8180_e7465: f64 = (assign8180_e7461 + assign8180_e7464);
            let assign8180_e7466: f64 = (assign8180_e7452 * assign8180_e7465);
            locals.var_cox_p = assign8180_e7466;
            locals.var_cox_p_rv = 0.0;
        }

        let assign8190_e7487: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign8190_e7487;
        locals.var_guard102_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard102 != 0.0)) {
            let assign8200_e7494: f64 = (p.p653 * locals.var_ile);
            let assign8200_e7495: f64 = (p.p652 + assign8200_e7494);
            let assign8200_e7498: f64 = (p.p654 * locals.var_iwe);
            let assign8200_e7499: f64 = (assign8200_e7495 + assign8200_e7498);
            let assign8200_e7502: f64 = (p.p655 * locals.var_iae);
            let assign8200_e7503: f64 = (assign8200_e7499 + assign8200_e7502);
            locals.var_delvtac_p = assign8200_e7503;
            locals.var_delvtac_p_rv = 0.0;
        }

        let assign8210_e7524: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign8210_e7524;
        locals.var_guard103_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard103 != 0.0)) {
            let assign8220_e7531: f64 = (p.p657 * locals.var_ile);
            let assign8220_e7532: f64 = (p.p656 + assign8220_e7531);
            let assign8220_e7535: f64 = (p.p658 * locals.var_iwe);
            let assign8220_e7536: f64 = (assign8220_e7532 + assign8220_e7535);
            let assign8220_e7539: f64 = (p.p659 * locals.var_iae);
            let assign8220_e7540: f64 = (assign8220_e7536 + assign8220_e7539);
            locals.var_facneffac_p = assign8220_e7540;
            locals.var_facneffac_p_rv = 0.0;
        }

        let assign8230_e7581: f64 = if (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign8230_e7581;
        locals.var_guard104_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
            locals.var_poparam_i = p.p568;
            locals.var_poparam_i_rv = 0.0;
        }

        let assign8250_e7589: f64 = if param_given[660] { 1.0 } else { 0.0 };
        let assign8250_e7591: f64 = if assign8250_e7589 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign8250_e7591;
        locals.var_guard105_rv = 0.0;

        if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard105 != 0.0)) {
            locals.var_poparam_i = p.p660;
            locals.var_poparam_i_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
            locals.var_plparam_i = p.p569;
            locals.var_plparam_i_rv = 0.0;
        }

        let assign8280_e7607: f64 = if param_given[661] { 1.0 } else { 0.0 };
        let assign8280_e7609: f64 = if assign8280_e7607 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign8280_e7609;
        locals.var_guard106_rv = 0.0;

        if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard106 != 0.0)) {
            locals.var_plparam_i = p.p661;
            locals.var_plparam_i_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
            locals.var_pwparam_i = p.p570;
            locals.var_pwparam_i_rv = 0.0;
        }

        let assign8310_e7625: f64 = if param_given[662] { 1.0 } else { 0.0 };
        let assign8310_e7627: f64 = if assign8310_e7625 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign8310_e7627;
        locals.var_guard107_rv = 0.0;

        if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard107 != 0.0)) {
            locals.var_pwparam_i = p.p662;
            locals.var_pwparam_i_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
            locals.var_plwparam_i = p.p571;
            locals.var_plwparam_i_rv = 0.0;
        }

        let assign8340_e7643: f64 = if param_given[663] { 1.0 } else { 0.0 };
        let assign8340_e7645: f64 = if assign8340_e7643 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign8340_e7645;
        locals.var_guard108_rv = 0.0;

        if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard108 != 0.0)) {
            locals.var_plwparam_i = p.p663;
            locals.var_plwparam_i_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
            let assign8360_e7661: f64 = (locals.var_plparam_i * locals.var_ile);
            let assign8360_e7662: f64 = (locals.var_poparam_i + assign8360_e7661);
            let assign8360_e7665: f64 = (locals.var_pwparam_i * locals.var_iwe);
            let assign8360_e7666: f64 = (assign8360_e7662 + assign8360_e7665);
            let assign8360_e7669: f64 = (locals.var_plwparam_i * locals.var_iae);
            let assign8360_e7670: f64 = (assign8360_e7666 + assign8360_e7669);
            let assign8360_e7671: f64 = (locals.var_ile * assign8360_e7670);
            locals.var_thesatac_p = assign8360_e7671;
            locals.var_thesatac_p_rv = 0.0;
        }

        let assign8370_e7712: f64 = if (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign8370_e7712;
        locals.var_guard109_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
            locals.var_poparam_i = p.p584;
            locals.var_poparam_i_rv = 0.0;
        }

        let assign8390_e7720: f64 = if param_given[664] { 1.0 } else { 0.0 };
        let assign8390_e7722: f64 = if assign8390_e7720 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign8390_e7722;
        locals.var_guard110_rv = 0.0;

        if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) {
            locals.var_poparam_i = p.p664;
            locals.var_poparam_i_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
            locals.var_plparam_i = p.p585;
            locals.var_plparam_i_rv = 0.0;
        }

        let assign8420_e7738: f64 = if param_given[665] { 1.0 } else { 0.0 };
        let assign8420_e7740: f64 = if assign8420_e7738 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign8420_e7740;
        locals.var_guard111_rv = 0.0;

        if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
            locals.var_plparam_i = p.p665;
            locals.var_plparam_i_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
            locals.var_pwparam_i = p.p586;
            locals.var_pwparam_i_rv = 0.0;
        }

        let assign8450_e7756: f64 = if param_given[666] { 1.0 } else { 0.0 };
        let assign8450_e7758: f64 = if assign8450_e7756 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign8450_e7758;
        locals.var_guard112_rv = 0.0;

        if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard112 != 0.0)) {
            locals.var_pwparam_i = p.p666;
            locals.var_pwparam_i_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
            locals.var_plwparam_i = p.p587;
            locals.var_plwparam_i_rv = 0.0;
        }

        let assign8480_e7774: f64 = if param_given[667] { 1.0 } else { 0.0 };
        let assign8480_e7776: f64 = if assign8480_e7774 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign8480_e7776;
        locals.var_guard113_rv = 0.0;

        if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard113 != 0.0)) {
            locals.var_plwparam_i = p.p667;
            locals.var_plwparam_i_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
            let assign8500_e7792: f64 = (locals.var_plparam_i * locals.var_ile);
            let assign8500_e7793: f64 = (locals.var_poparam_i + assign8500_e7792);
            let assign8500_e7796: f64 = (locals.var_pwparam_i * locals.var_iwe);
            let assign8500_e7797: f64 = (assign8500_e7793 + assign8500_e7796);
            let assign8500_e7800: f64 = (locals.var_plwparam_i * locals.var_iae);
            let assign8500_e7801: f64 = (assign8500_e7797 + assign8500_e7800);
            let assign8500_e7802: f64 = assign8500_e7801;
            locals.var_axac_p = assign8500_e7802;
            locals.var_axac_p_rv = 0.0;
        }

        let assign8510_e7823: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign8510_e7823;
        locals.var_guard114_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard114 != 0.0)) {
            let assign8520_e7831: f64 = (p.p669 * locals.var_ile);
            let assign8520_e7832: f64 = (p.p668 + assign8520_e7831);
            let assign8520_e7835: f64 = (p.p670 * locals.var_iwe);
            let assign8520_e7836: f64 = (assign8520_e7832 + assign8520_e7835);
            let assign8520_e7839: f64 = (p.p671 * locals.var_iae);
            let assign8520_e7840: f64 = (assign8520_e7836 + assign8520_e7839);
            let assign8520_e7841: f64 = (locals.var_ile * assign8520_e7840);
            locals.var_alpac_p = assign8520_e7841;
            locals.var_alpac_p_rv = 0.0;
        }

        let assign8530_e7862: f64 = if (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign8530_e7862;
        locals.var_guard115_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard115 != 0.0)) {
            let assign8540_e7870: f64 = (p.p673 * locals.var_ile);
            let assign8540_e7871: f64 = (p.p672 + assign8540_e7870);
            let assign8540_e7874: f64 = (p.p674 * locals.var_iwe);
            let assign8540_e7875: f64 = (assign8540_e7871 + assign8540_e7874);
            let assign8540_e7878: f64 = (p.p675 * locals.var_iae);
            let assign8540_e7879: f64 = (assign8540_e7875 + assign8540_e7878);
            let assign8540_e7880: f64 = (locals.var_ile * assign8540_e7879);
            locals.var_alp1ac_p = assign8540_e7880;
            locals.var_alp1ac_p_rv = 0.0;
        }

        let assign8550_e7901: f64 = if (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) { 1.0 } else { 0.0 };
        locals.var_guard116 = assign8550_e7901;
        locals.var_guard116_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard116 != 0.0)) {
            let assign8560_e7909: f64 = (p.p677 * locals.var_ile);
            let assign8560_e7910: f64 = (p.p676 + assign8560_e7909);
            let assign8560_e7913: f64 = (p.p678 * locals.var_iwe);
            let assign8560_e7914: f64 = (assign8560_e7910 + assign8560_e7913);
            let assign8560_e7917: f64 = (p.p679 * locals.var_iae);
            let assign8560_e7918: f64 = (assign8560_e7914 + assign8560_e7917);
            let assign8560_e7919: f64 = (locals.var_iiwecv * assign8560_e7918);
            locals.var_cgov_p = assign8560_e7919;
            locals.var_cgov_p_rv = 0.0;
        }

        let assign8570_e7940: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };
        locals.var_guard117 = assign8570_e7940;
        locals.var_guard117_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard117 != 0.0)) {
            let assign8580_e7948: f64 = (p.p681 * locals.var_ile);
            let assign8580_e7949: f64 = (p.p680 + assign8580_e7948);
            let assign8580_e7952: f64 = (p.p682 * locals.var_iwe);
            let assign8580_e7953: f64 = (assign8580_e7949 + assign8580_e7952);
            let assign8580_e7956: f64 = (p.p683 * locals.var_iae);
            let assign8580_e7957: f64 = (assign8580_e7953 + assign8580_e7956);
            let assign8580_e7958: f64 = (locals.var_iiwecv * assign8580_e7957);
            locals.var_cgovd_p = assign8580_e7958;
            locals.var_cgovd_p_rv = 0.0;
        }

        let assign8590_e7979: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };
        locals.var_guard118 = assign8590_e7979;
        locals.var_guard118_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard118 != 0.0)) {
            let assign8600_e7987: f64 = (p.p685 * locals.var_ile);
            let assign8600_e7988: f64 = (p.p684 + assign8600_e7987);
            let assign8600_e7991: f64 = (p.p686 * locals.var_iwe);
            let assign8600_e7992: f64 = (assign8600_e7988 + assign8600_e7991);
            let assign8600_e7995: f64 = (p.p687 * locals.var_iae);
            let assign8600_e7996: f64 = (assign8600_e7992 + assign8600_e7995);
            let assign8600_e7997: f64 = (locals.var_iilcv * assign8600_e7996);
            locals.var_cgbov_p = assign8600_e7997;
            locals.var_cgbov_p_rv = 0.0;
        }

        let assign8610_e8018: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign8610_e8018;
        locals.var_guard119_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard119 != 0.0)) {
            let assign8620_e8026: f64 = (p.p689 * locals.var_ile);
            let assign8620_e8027: f64 = (p.p688 + assign8620_e8026);
            let assign8620_e8030: f64 = (p.p690 * locals.var_iwe);
            let assign8620_e8031: f64 = (assign8620_e8027 + assign8620_e8030);
            let assign8620_e8034: f64 = (p.p691 * locals.var_iae);
            let assign8620_e8035: f64 = (assign8620_e8031 + assign8620_e8034);
            let assign8620_e8036: f64 = (locals.var_iiwecv * assign8620_e8035);
            locals.var_cinr_p = assign8620_e8036;
            locals.var_cinr_p_rv = 0.0;
        }

        let assign8630_e8057: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };
        locals.var_guard120 = assign8630_e8057;
        locals.var_guard120_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard120 != 0.0)) {
            let assign8640_e8065: f64 = (p.p693 * locals.var_ile);
            let assign8640_e8066: f64 = (p.p692 + assign8640_e8065);
            let assign8640_e8069: f64 = (p.p694 * locals.var_iwe);
            let assign8640_e8070: f64 = (assign8640_e8066 + assign8640_e8069);
            let assign8640_e8073: f64 = (p.p695 * locals.var_iae);
            let assign8640_e8074: f64 = (assign8640_e8070 + assign8640_e8073);
            let assign8640_e8075: f64 = (locals.var_iiwecv * assign8640_e8074);
            locals.var_cinrd_p = assign8640_e8075;
            locals.var_cinrd_p_rv = 0.0;
        }

        let assign8770_e8330: f64 = if (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign8770_e8330;
        locals.var_guard127_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard36 != 0.0) && (locals.var_guard127 != 0.0)) {
            let assign8780_e8337: f64 = (p.p721 * locals.var_ile);
            let assign8780_e8338: f64 = (p.p720 + assign8780_e8337);
            let assign8780_e8341: f64 = (p.p722 * locals.var_iwe);
            let assign8780_e8342: f64 = (assign8780_e8338 + assign8780_e8341);
            let assign8780_e8345: f64 = (p.p723 * locals.var_iae);
            let assign8780_e8346: f64 = (assign8780_e8342 + assign8780_e8345);
            locals.var_vfbedge_p = assign8780_e8346;
            locals.var_vfbedge_p_rv = 0.0;
        }

        let assign8790_e8367: f64 = if (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]) { 1.0 } else { 0.0 };
        locals.var_guard128 = assign8790_e8367;
        locals.var_guard128_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard128 != 0.0)) {
            let assign8800_e8374: f64 = (p.p725 * locals.var_ile);
            let assign8800_e8375: f64 = (p.p724 + assign8800_e8374);
            let assign8800_e8378: f64 = (p.p726 * locals.var_iwe);
            let assign8800_e8379: f64 = (assign8800_e8375 + assign8800_e8378);
            let assign8800_e8382: f64 = (p.p727 * locals.var_iae);
            let assign8800_e8383: f64 = (assign8800_e8379 + assign8800_e8382);
            locals.var_stvfbedge_p = assign8800_e8383;
            locals.var_stvfbedge_p_rv = 0.0;
        }

        let assign8810_e8404: f64 = if (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]) { 1.0 } else { 0.0 };
        locals.var_guard129 = assign8810_e8404;
        locals.var_guard129_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard129 != 0.0)) {
            let assign8820_e8411: f64 = (p.p729 * locals.var_ile);
            let assign8820_e8412: f64 = (p.p728 + assign8820_e8411);
            let assign8820_e8415: f64 = (p.p730 * locals.var_iwe);
            let assign8820_e8416: f64 = (assign8820_e8412 + assign8820_e8415);
            let assign8820_e8419: f64 = (p.p731 * locals.var_iae);
            let assign8820_e8420: f64 = (assign8820_e8416 + assign8820_e8419);
            locals.var_dphibedge_p = assign8820_e8420;
            locals.var_dphibedge_p_rv = 0.0;
        }

        let assign8830_e8441: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };
        locals.var_guard130 = assign8830_e8441;
        locals.var_guard130_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard130 != 0.0)) {
            let assign8840_e8448: f64 = (p.p733 * locals.var_ile);
            let assign8840_e8449: f64 = (p.p732 + assign8840_e8448);
            let assign8840_e8452: f64 = (p.p734 * locals.var_iwe);
            let assign8840_e8453: f64 = (assign8840_e8449 + assign8840_e8452);
            let assign8840_e8456: f64 = (p.p735 * locals.var_iae);
            let assign8840_e8457: f64 = (assign8840_e8453 + assign8840_e8456);
            locals.var_neffedge_p = assign8840_e8457;
            locals.var_neffedge_p_rv = 0.0;
        }

        let assign8850_e8478: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };
        locals.var_guard131 = assign8850_e8478;
        locals.var_guard131_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard131 != 0.0)) {
            let assign8860_e8485: f64 = (p.p737 * locals.var_ile);
            let assign8860_e8486: f64 = (p.p736 + assign8860_e8485);
            let assign8860_e8489: f64 = (p.p738 * locals.var_iwe);
            let assign8860_e8490: f64 = (assign8860_e8486 + assign8860_e8489);
            let assign8860_e8493: f64 = (p.p739 * locals.var_iae);
            let assign8860_e8494: f64 = (assign8860_e8490 + assign8860_e8493);
            locals.var_ctedge_p = assign8860_e8494;
            locals.var_ctedge_p_rv = 0.0;
        }

        let assign8870_e8515: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign8870_e8515;
        locals.var_guard132_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard132 != 0.0)) {
            let assign8880_e8521: f64 = (locals.var_we_edge / locals.var_le);
            let assign8880_e8525: f64 = (p.p741 * locals.var_ile);
            let assign8880_e8526: f64 = (p.p740 + assign8880_e8525);
            let assign8880_e8529: f64 = (p.p742 * locals.var_iwe);
            let assign8880_e8530: f64 = (assign8880_e8526 + assign8880_e8529);
            let assign8880_e8533: f64 = (p.p743 * locals.var_iae);
            let assign8880_e8534: f64 = (assign8880_e8530 + assign8880_e8533);
            let assign8880_e8535: f64 = (assign8880_e8521 * assign8880_e8534);
            locals.var_betnedge_p = assign8880_e8535;
            locals.var_betnedge_p_rv = 0.0;
        }

        let assign8890_e8556: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign8890_e8556;
        locals.var_guard133_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard133 != 0.0)) {
            let assign8900_e8563: f64 = (p.p745 * locals.var_ile);
            let assign8900_e8564: f64 = (p.p744 + assign8900_e8563);
            let assign8900_e8567: f64 = (p.p746 * locals.var_iwe);
            let assign8900_e8568: f64 = (assign8900_e8564 + assign8900_e8567);
            let assign8900_e8571: f64 = (p.p747 * locals.var_iae);
            let assign8900_e8572: f64 = (assign8900_e8568 + assign8900_e8571);
            locals.var_stbetedge_p = assign8900_e8572;
            locals.var_stbetedge_p_rv = 0.0;
        }

        let assign8910_e8593: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };
        locals.var_guard134 = assign8910_e8593;
        locals.var_guard134_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard134 != 0.0)) {
            let assign8920_e8601: f64 = (p.p749 * locals.var_ile);
            let assign8920_e8602: f64 = (p.p748 + assign8920_e8601);
            let assign8920_e8605: f64 = (p.p750 * locals.var_iwe);
            let assign8920_e8606: f64 = (assign8920_e8602 + assign8920_e8605);
            let assign8920_e8609: f64 = (p.p751 * locals.var_iae);
            let assign8920_e8610: f64 = (assign8920_e8606 + assign8920_e8609);
            let assign8920_e8611: f64 = (locals.var_ile2 * assign8920_e8610);
            locals.var_psceedge_p = assign8920_e8611;
            locals.var_psceedge_p_rv = 0.0;
        }

        let assign8930_e8632: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };
        locals.var_guard135 = assign8930_e8632;
        locals.var_guard135_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard135 != 0.0)) {
            let assign8940_e8639: f64 = (p.p753 * locals.var_ile);
            let assign8940_e8640: f64 = (p.p752 + assign8940_e8639);
            let assign8940_e8643: f64 = (p.p754 * locals.var_iwe);
            let assign8940_e8644: f64 = (assign8940_e8640 + assign8940_e8643);
            let assign8940_e8647: f64 = (p.p755 * locals.var_iae);
            let assign8940_e8648: f64 = (assign8940_e8644 + assign8940_e8647);
            locals.var_pscebedge_p = assign8940_e8648;
            locals.var_pscebedge_p_rv = 0.0;
        }

        let assign8950_e8669: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign8950_e8669;
        locals.var_guard136_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard136 != 0.0)) {
            let assign8960_e8676: f64 = (p.p757 * locals.var_ile);
            let assign8960_e8677: f64 = (p.p756 + assign8960_e8676);
            let assign8960_e8680: f64 = (p.p758 * locals.var_iwe);
            let assign8960_e8681: f64 = (assign8960_e8677 + assign8960_e8680);
            let assign8960_e8684: f64 = (p.p759 * locals.var_iae);
            let assign8960_e8685: f64 = (assign8960_e8681 + assign8960_e8684);
            locals.var_pscededge_p = assign8960_e8685;
            locals.var_pscededge_p_rv = 0.0;
        }

        let assign8970_e8706: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign8970_e8706;
        locals.var_guard137_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard137 != 0.0)) {
            let assign8980_e8714: f64 = (p.p761 * locals.var_ile);
            let assign8980_e8715: f64 = (p.p760 + assign8980_e8714);
            let assign8980_e8718: f64 = (p.p762 * locals.var_iwe);
            let assign8980_e8719: f64 = (assign8980_e8715 + assign8980_e8718);
            let assign8980_e8722: f64 = (p.p763 * locals.var_iae);
            let assign8980_e8723: f64 = (assign8980_e8719 + assign8980_e8722);
            let assign8980_e8724: f64 = (locals.var_ile2 * assign8980_e8723);
            locals.var_cfedge_p = assign8980_e8724;
            locals.var_cfedge_p_rv = 0.0;
        }

        let assign8990_e8745: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign8990_e8745;
        locals.var_guard138_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard138 != 0.0)) {
            let assign9000_e8752: f64 = (p.p769 * locals.var_ile);
            let assign9000_e8753: f64 = (p.p768 + assign9000_e8752);
            let assign9000_e8756: f64 = (p.p770 * locals.var_iwe);
            let assign9000_e8757: f64 = (assign9000_e8753 + assign9000_e8756);
            let assign9000_e8760: f64 = (p.p771 * locals.var_iae);
            let assign9000_e8761: f64 = (assign9000_e8757 + assign9000_e8760);
            locals.var_cfdedge_p = assign9000_e8761;
            locals.var_cfdedge_p_rv = 0.0;
        }

        let assign9010_e8782: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };
        locals.var_guard139 = assign9010_e8782;
        locals.var_guard139_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard139 != 0.0)) {
            let assign9020_e8789: f64 = (p.p765 * locals.var_ile);
            let assign9020_e8790: f64 = (p.p764 + assign9020_e8789);
            let assign9020_e8793: f64 = (p.p766 * locals.var_iwe);
            let assign9020_e8794: f64 = (assign9020_e8790 + assign9020_e8793);
            let assign9020_e8797: f64 = (p.p767 * locals.var_iae);
            let assign9020_e8798: f64 = (assign9020_e8794 + assign9020_e8797);
            locals.var_cfbedge_p = assign9020_e8798;
            locals.var_cfbedge_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_tmpa = 0.0;
            locals.var_tmpa_rv = 0.0;
            locals.var_tmpb = 0.0;
            locals.var_tmpb_rv = 0.0;
            locals.var_loop_ = 0.0;
            locals.var_loop__rv = 0.0;
            locals.var_kvsatac_i = p.p788;
            locals.var_kvsatac_i_rv = 0.0;
        }

        let assign9130_e8935: f64 = if param_given[789] { 1.0 } else { 0.0 };
        let assign9130_e8937: f64 = if assign9130_e8935 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign9130_e8937;
        locals.var_guard143_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard143 != 0.0)) {
            locals.var_kvsatac_i = p.p789;
            locals.var_kvsatac_i_rv = 0.0;
        }

        let assign9150_e8962: f64 = if (((locals.var_sa_i > 0.0) && (locals.var_sb_i > 0.0)) && ((locals.var_nf_i == 1.0) || ((locals.var_nf_i > 1.0) && (locals.var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard144 = assign9150_e8962;
        locals.var_guard144_rv = 0.0;

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e8969: f64 = (locals.var_nf_i - 0.5);
            let assign9160_cond_e8971: f64 = if (((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_loop_ < assign9160_cond_e8969)) { 1.0 } else { 0.0 };
            assign9160_cond_e8971 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
                let assign9160_body0_e8980: f64 = (0.5 * locals.var_l_i);
                let assign9160_body0_e8981: f64 = (locals.var_sa_i + assign9160_body0_e8980);
                let assign9160_body0_e8985: f64 = (locals.var_sd_i + locals.var_l_i);
                let assign9160_body0_e8986: f64 = (locals.var_loop_ * assign9160_body0_e8985);
                let assign9160_body0_e8987: f64 = (assign9160_body0_e8981 + assign9160_body0_e8986);
                let assign9160_body0_e8988: f64 = (1.0 / assign9160_body0_e8987);
                let assign9160_body0_e8989: f64 = (locals.var_tmpa + assign9160_body0_e8988);
                locals.var_tmpa = assign9160_body0_e8989;
                locals.var_tmpa_rv = 0.0;
            }
            if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
                let assign9160_body1_e9000: f64 = (0.5 * locals.var_l_i);
                let assign9160_body1_e9001: f64 = (locals.var_sb_i + assign9160_body1_e9000);
                let assign9160_body1_e9005: f64 = (locals.var_sd_i + locals.var_l_i);
                let assign9160_body1_e9006: f64 = (locals.var_loop_ * assign9160_body1_e9005);
                let assign9160_body1_e9007: f64 = (assign9160_body1_e9001 + assign9160_body1_e9006);
                let assign9160_body1_e9008: f64 = (1.0 / assign9160_body1_e9007);
                let assign9160_body1_e9009: f64 = (locals.var_tmpb + assign9160_body1_e9008);
                locals.var_tmpb = assign9160_body1_e9009;
                locals.var_tmpb_rv = 0.0;
            }
            if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
                let assign9160_body2_e9017: f64 = (locals.var_loop_ + 1.0);
                locals.var_loop_ = assign9160_body2_e9017;
                locals.var_loop__rv = 0.0;
            }
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9170_e9025: f64 = (locals.var_tmpa * locals.var_invnf);
            locals.var_invsa = assign9170_e9025;
            locals.var_invsa_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9180_e9033: f64 = (locals.var_tmpb * locals.var_invnf);
            locals.var_invsb = assign9180_e9033;
            locals.var_invsb_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9190_e9043: f64 = (0.5 * locals.var_l_i);
            let assign9190_e9044: f64 = (p.p784 + assign9190_e9043);
            let assign9190_e9045: f64 = (1.0 / assign9190_e9044);
            locals.var_invsaref = assign9190_e9045;
            locals.var_invsaref_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9200_e9055: f64 = (0.5 * locals.var_l_i);
            let assign9200_e9056: f64 = (p.p785 + assign9200_e9055);
            let assign9200_e9057: f64 = (1.0 / assign9200_e9056);
            locals.var_invsbref = assign9200_e9057;
            locals.var_invsbref_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9210_e9065: f64 = (locals.var_l_i + locals.var_dellps);
            let (assign9210_e9072,) = {
    if (assign9210_e9065 > 1e-9) {
        let assign9210_e9070: f64 = (locals.var_l_i + locals.var_dellps);
        (assign9210_e9070,)
    } else {
        (1e-9,)
    }
};
            locals.var_lx = assign9210_e9072;
            locals.var_lx_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9220_e9080: f64 = (locals.var_w_i + locals.var_delwod);
            let assign9220_e9082: f64 = (assign9220_e9080 + p.p786);
            let (assign9220_e9091,) = {
    if (assign9220_e9082 > 1e-9) {
        let assign9220_e9087: f64 = (locals.var_w_i + locals.var_delwod);
        let assign9220_e9089: f64 = (assign9220_e9087 + p.p786);
        (assign9220_e9089,)
    } else {
        (1e-9,)
    }
};
            locals.var_wx = assign9220_e9091;
            locals.var_wx_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9230_e9100: f64 = (locals.var_lx).powf(p.p794);
            let assign9230_e9101: f64 = (1.0 / assign9230_e9100);
            locals.var_templ = assign9230_e9101;
            locals.var_templ_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9240_e9110: f64 = (locals.var_wx).powf(p.p795);
            let assign9240_e9111: f64 = (1.0 / assign9240_e9110);
            locals.var_tempw = assign9240_e9111;
            locals.var_tempw_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9250_e9120: f64 = (p.p791 * locals.var_templ);
            let assign9250_e9121: f64 = (1.0 + assign9250_e9120);
            let assign9250_e9124: f64 = (p.p792 * locals.var_tempw);
            let assign9250_e9125: f64 = (assign9250_e9121 + assign9250_e9124);
            let assign9250_e9128: f64 = (p.p793 * locals.var_templ);
            let assign9250_e9130: f64 = (assign9250_e9128 * locals.var_tempw);
            let assign9250_e9131: f64 = (assign9250_e9125 + assign9250_e9130);
            let assign9250_e9136: f64 = (locals.var_rta - 1.0);
            let assign9250_e9137: f64 = (p.p790 * assign9250_e9136);
            let assign9250_e9138: f64 = (1.0 + assign9250_e9137);
            let assign9250_e9139: f64 = (assign9250_e9131 * assign9250_e9138);
            locals.var_kstressu0 = assign9250_e9139;
            locals.var_kstressu0_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9260_e9148: f64 = (locals.var_invsa + locals.var_invsb);
            let assign9260_e9149: f64 = (p.p787 * assign9260_e9148);
            let assign9260_e9151: f64 = (assign9260_e9149 / locals.var_kstressu0);
            locals.var_rhobeta = assign9260_e9151;
            locals.var_rhobeta_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9270_e9160: f64 = (locals.var_invsaref + locals.var_invsbref);
            let assign9270_e9161: f64 = (p.p787 * assign9270_e9160);
            let assign9270_e9163: f64 = (assign9270_e9161 / locals.var_kstressu0);
            locals.var_rhobetaref = assign9270_e9163;
            locals.var_rhobetaref_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9280_e9172: f64 = (locals.var_lx).powf(p.p800);
            let assign9280_e9173: f64 = (1.0 / assign9280_e9172);
            locals.var_templ = assign9280_e9173;
            locals.var_templ_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9290_e9182: f64 = (locals.var_wx).powf(p.p801);
            let assign9290_e9183: f64 = (1.0 / assign9290_e9182);
            locals.var_tempw = assign9290_e9183;
            locals.var_tempw_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9300_e9192: f64 = (p.p797 * locals.var_templ);
            let assign9300_e9193: f64 = (1.0 + assign9300_e9192);
            let assign9300_e9196: f64 = (p.p798 * locals.var_tempw);
            let assign9300_e9197: f64 = (assign9300_e9193 + assign9300_e9196);
            let assign9300_e9200: f64 = (p.p799 * locals.var_templ);
            let assign9300_e9202: f64 = (assign9300_e9200 * locals.var_tempw);
            let assign9300_e9203: f64 = (assign9300_e9197 + assign9300_e9202);
            locals.var_kstressvth0 = assign9300_e9203;
            locals.var_kstressvth0_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9310_e9211: f64 = (locals.var_invsa + locals.var_invsb);
            let assign9310_e9213: f64 = (assign9310_e9211 - locals.var_invsaref);
            let assign9310_e9215: f64 = (assign9310_e9213 - locals.var_invsbref);
            locals.var_temp0 = assign9310_e9215;
            locals.var_temp0_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9320_e9223: f64 = (1.0 + locals.var_rhobeta);
            let assign9320_e9226: f64 = (1.0 + locals.var_rhobetaref);
            let assign9320_e9227: f64 = (assign9320_e9223 / assign9320_e9226);
            locals.var_temp00 = assign9320_e9227;
            locals.var_temp00_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9330_e9235: f64 = (locals.var_betn_p * locals.var_temp00);
            locals.var_betn_p = assign9330_e9235;
            locals.var_betn_p_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9340_e9243: f64 = (locals.var_thesat_p * locals.var_temp00);
            let assign9340_e9247: f64 = (p.p788 * locals.var_rhobetaref);
            let assign9340_e9248: f64 = (1.0 + assign9340_e9247);
            let assign9340_e9249: f64 = (assign9340_e9243 * assign9340_e9248);
            let assign9340_e9253: f64 = (p.p788 * locals.var_rhobeta);
            let assign9340_e9254: f64 = (1.0 + assign9340_e9253);
            let assign9340_e9255: f64 = (assign9340_e9249 / assign9340_e9254);
            locals.var_thesat_p = assign9340_e9255;
            locals.var_thesat_p_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9350_e9263: f64 = (locals.var_thesatac_p * locals.var_temp00);
            let assign9350_e9267: f64 = (locals.var_kvsatac_i * locals.var_rhobetaref);
            let assign9350_e9268: f64 = (1.0 + assign9350_e9267);
            let assign9350_e9269: f64 = (assign9350_e9263 * assign9350_e9268);
            let assign9350_e9273: f64 = (locals.var_kvsatac_i * locals.var_rhobeta);
            let assign9350_e9274: f64 = (1.0 + assign9350_e9273);
            let assign9350_e9275: f64 = (assign9350_e9269 / assign9350_e9274);
            locals.var_thesatac_p = assign9350_e9275;
            locals.var_thesatac_p_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9360_e9283: f64 = (locals.var_betnedge_p * locals.var_temp00);
            locals.var_betnedge_p = assign9360_e9283;
            locals.var_betnedge_p_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9370_e9291: f64 = (p.p796 * locals.var_temp0);
            let assign9370_e9293: f64 = (assign9370_e9291 / locals.var_kstressvth0);
            locals.var_temp00 = assign9370_e9293;
            locals.var_temp00_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9380_e9301: f64 = (locals.var_vfb_p + locals.var_temp00);
            locals.var_vfb_p = assign9380_e9301;
            locals.var_vfb_p_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9390_e9309: f64 = (locals.var_vfbedge_p + locals.var_temp00);
            locals.var_vfbedge_p = assign9390_e9309;
            locals.var_vfbedge_p_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9400_e9317: f64 = (p.p802 * locals.var_temp0);
            let assign9400_e9320: f64 = (locals.var_kstressvth0).powf(p.p803);
            let assign9400_e9321: f64 = (assign9400_e9317 / assign9400_e9320);
            locals.var_temp00 = assign9400_e9321;
            locals.var_temp00_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9410_e9329: f64 = (locals.var_cf_p + locals.var_temp00);
            locals.var_cf_p = assign9410_e9329;
            locals.var_cf_p_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
            let assign9420_e9337: f64 = (locals.var_cfedge_p + locals.var_temp00);
            locals.var_cfedge_p = assign9420_e9337;
            locals.var_cfedge_p_rv = 0.0;
        }

        let assign9430_e9354: f64 = if ((((locals.var_sca_i > 0.0) || (locals.var_scb_i > 0.0)) || (locals.var_scc_i > 0.0)) || (locals.var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard145 = assign9430_e9354;
        locals.var_guard145_rv = 0.0;

        let assign9440_e9365: f64 = if (((locals.var_sca_i == 0.0) && (locals.var_scb_i == 0.0)) && (locals.var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard146 = assign9440_e9365;
        locals.var_guard146_rv = 0.0;

        if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
            let assign9450_e9373: f64 = (locals.var_sc_i + locals.var_w_i);
            locals.var_temp0 = assign9450_e9373;
            locals.var_temp0_rv = 0.0;
        }

        if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
            let assign9460_e9383: f64 = (1.0 / p.p804);
            locals.var_temp00 = assign9460_e9383;
            locals.var_temp00_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
            let assign9470_e9393: f64 = (p.p804 * p.p804);
            let assign9470_e9396: f64 = (locals.var_sc_i * locals.var_temp0);
            let assign9470_e9397: f64 = (assign9470_e9393 / assign9470_e9396);
            locals.var_sca_i = assign9470_e9397;
            locals.var_sca_i_rv = 0.0;
        }

        if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
            let assign9480_e9407: f64 = (0.1 * locals.var_sc_i);
            let assign9480_e9410: f64 = (0.01 * p.p804);
            let assign9480_e9411: f64 = (assign9480_e9407 + assign9480_e9410);
            let assign9480_e9413: f64 = (-10.0);
            let assign9480_e9415: f64 = (assign9480_e9413 * locals.var_sc_i);
            let assign9480_e9417: f64 = (assign9480_e9415 * locals.var_temp00);
            let assign9480_e9418: f64 = (assign9480_e9417).exp();
            let assign9480_e9419: f64 = (assign9480_e9411 * assign9480_e9418);
            let assign9480_e9422: f64 = (0.1 * locals.var_temp0);
            let assign9480_e9425: f64 = (0.01 * p.p804);
            let assign9480_e9426: f64 = (assign9480_e9422 + assign9480_e9425);
            let assign9480_e9428: f64 = (-10.0);
            let assign9480_e9430: f64 = (assign9480_e9428 * locals.var_temp0);
            let assign9480_e9432: f64 = (assign9480_e9430 * locals.var_temp00);
            let assign9480_e9433: f64 = (assign9480_e9432).exp();
            let assign9480_e9434: f64 = (assign9480_e9426 * assign9480_e9433);
            let assign9480_e9435: f64 = (assign9480_e9419 - assign9480_e9434);
            let assign9480_e9437: f64 = (assign9480_e9435 / locals.var_w_i);
            locals.var_scb_i = assign9480_e9437;
            locals.var_scb_i_rv = 0.0;
        }

        if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
            let assign9490_e9447: f64 = (0.05 * locals.var_sc_i);
            let assign9490_e9450: f64 = (0.0025 * p.p804);
            let assign9490_e9451: f64 = (assign9490_e9447 + assign9490_e9450);
            let assign9490_e9453: f64 = (-20.0);
            let assign9490_e9455: f64 = (assign9490_e9453 * locals.var_sc_i);
            let assign9490_e9457: f64 = (assign9490_e9455 * locals.var_temp00);
            let assign9490_e9458: f64 = (assign9490_e9457).exp();
            let assign9490_e9459: f64 = (assign9490_e9451 * assign9490_e9458);
            let assign9490_e9462: f64 = (0.05 * locals.var_temp0);
            let assign9490_e9465: f64 = (0.0025 * p.p804);
            let assign9490_e9466: f64 = (assign9490_e9462 + assign9490_e9465);
            let assign9490_e9468: f64 = (-20.0);
            let assign9490_e9470: f64 = (assign9490_e9468 * locals.var_temp0);
            let assign9490_e9472: f64 = (assign9490_e9470 * locals.var_temp00);
            let assign9490_e9473: f64 = (assign9490_e9472).exp();
            let assign9490_e9474: f64 = (assign9490_e9466 * assign9490_e9473);
            let assign9490_e9475: f64 = (assign9490_e9459 - assign9490_e9474);
            let assign9490_e9477: f64 = (assign9490_e9475 / locals.var_w_i);
            locals.var_scc_i = assign9490_e9477;
            locals.var_scc_i_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
            let assign9500_e9486: f64 = (p.p805 * locals.var_scb_i);
            let assign9500_e9487: f64 = (locals.var_sca_i + assign9500_e9486);
            let assign9500_e9490: f64 = (p.p806 * locals.var_scc_i);
            let assign9500_e9491: f64 = (assign9500_e9487 + assign9500_e9490);
            locals.var_temp0 = assign9500_e9491;
            locals.var_temp0_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
            let assign9510_e9500: f64 = (locals.var_kvthowe * locals.var_temp0);
            let assign9510_e9501: f64 = (locals.var_vfb_p + assign9510_e9500);
            locals.var_vfb_p = assign9510_e9501;
            locals.var_vfb_p_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
            let assign9520_e9511: f64 = (locals.var_kuowe * locals.var_temp0);
            let assign9520_e9512: f64 = (1.0 + assign9520_e9511);
            let assign9520_e9513: f64 = (locals.var_betn_p * assign9520_e9512);
            locals.var_betn_p = assign9520_e9513;
            locals.var_betn_p_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
            let assign9530_e9522: f64 = (locals.var_kvthowe * locals.var_temp0);
            let assign9530_e9523: f64 = (locals.var_vfbedge_p + assign9530_e9522);
            locals.var_vfbedge_p = assign9530_e9523;
            locals.var_vfbedge_p_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
            let assign9540_e9533: f64 = (locals.var_kuowe * locals.var_temp0);
            let assign9540_e9534: f64 = (1.0 + assign9540_e9533);
            let assign9540_e9535: f64 = (locals.var_betnedge_p * assign9540_e9534);
            locals.var_betnedge_p = assign9540_e9535;
            locals.var_betnedge_p_rv = 0.0;
        }

        locals.var_vfb_i = locals.var_vfb_p;
        locals.var_vfb_i_rv = 0.0;

        locals.var_stvfb_i = locals.var_stvfb_p;
        locals.var_stvfb_i_rv = 0.0;

        locals.var_st2vfb_i = locals.var_st2vfb_p;
        locals.var_st2vfb_i_rv = 0.0;

        locals.var_tox_i = locals.var_tox_p;
        locals.var_tox_i_rv = 0.0;

        locals.var_epsrox_i = locals.var_epsrox_p;
        locals.var_epsrox_i_rv = 0.0;

        let (assign9600_e9553,) = {
    if (locals.var_neff_p > 1e20) {
        let (assign9600_e9551,) = {
            if (locals.var_neff_p < 1e26) {
                (locals.var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9600_e9551,)
    } else {
        (1e20,)
    }
};
        locals.var_neff_i = assign9600_e9553;
        locals.var_neff_i_rv = 0.0;

        let (assign9610_e9559,) = {
    if (locals.var_gfacnud_p > 0.01) {
        (locals.var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        locals.var_gfacnud_i = assign9610_e9559;
        locals.var_gfacnud_i_rv = 0.0;

        let (assign9620_e9565,) = {
    if (locals.var_vsbnud_p > 0.0) {
        (locals.var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        locals.var_vsbnud_i = assign9620_e9565;
        locals.var_vsbnud_i_rv = 0.0;

        locals.var_dvsbnud_i = locals.var_dvsbnud_p;
        locals.var_dvsbnud_i_rv = 0.0;

        locals.var_dphib_i = locals.var_dphib_p;
        locals.var_dphib_i_rv = 0.0;

        let (assign9650_e9573,) = {
    if (locals.var_np_p > 0.0) {
        (locals.var_np_p,)
    } else {
        (0.0,)
    }
};
        locals.var_np_i = assign9650_e9573;
        locals.var_np_i_rv = 0.0;

        locals.var_toxov_i = locals.var_toxov_p;
        locals.var_toxov_i_rv = 0.0;

        locals.var_toxovd_i = locals.var_toxovd_p;
        locals.var_toxovd_i_rv = 0.0;

        let (assign9680_e9586,) = {
    if (locals.var_nov_p > 1e23) {
        let (assign9680_e9584,) = {
            if (locals.var_nov_p < 1e27) {
                (locals.var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9680_e9584,)
    } else {
        (1e23,)
    }
};
        locals.var_nov_i = assign9680_e9586;
        locals.var_nov_i_rv = 0.0;

        let (assign9690_e9597,) = {
    if (locals.var_novd_p > 1e23) {
        let (assign9690_e9595,) = {
            if (locals.var_novd_p < 1e27) {
                (locals.var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9690_e9595,)
    } else {
        (1e23,)
    }
};
        locals.var_novd_i = assign9690_e9597;
        locals.var_novd_i_rv = 0.0;

        let (assign9700_e9603,) = {
    if (locals.var_ct_p > 0.0) {
        (locals.var_ct_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ct_i = assign9700_e9603;
        locals.var_ct_i_rv = 0.0;

        let (assign9710_e9614,) = {
    if (locals.var_ctb_p > 0.0) {
        let (assign9710_e9612,) = {
            if (locals.var_ctb_p < 0.5) {
                (locals.var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9710_e9612,)
    } else {
        (0.0,)
    }
};
        locals.var_ctb_i = assign9710_e9614;
        locals.var_ctb_i_rv = 0.0;

        let (assign9720_e9625,) = {
    if (locals.var_ctg_p > 0.0) {
        let (assign9720_e9623,) = {
            if (locals.var_ctg_p < 1.0) {
                (locals.var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9720_e9623,)
    } else {
        (0.0,)
    }
};
        locals.var_ctg_i = assign9720_e9625;
        locals.var_ctg_i_rv = 0.0;

        locals.var_stct_i = locals.var_stct_p;
        locals.var_stct_i_rv = 0.0;

        let (assign9740_e9632,) = {
    if (locals.var_cf_p > 0.0) {
        (locals.var_cf_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cf_i = assign9740_e9632;
        locals.var_cf_i_rv = 0.0;

        let (assign9750_e9643,) = {
    if (locals.var_cfb_p > 0.0) {
        let (assign9750_e9641,) = {
            if (locals.var_cfb_p < 1.0) {
                (locals.var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9750_e9641,)
    } else {
        (0.0,)
    }
};
        locals.var_cfb_i = assign9750_e9643;
        locals.var_cfb_i_rv = 0.0;

        let (assign9760_e9649,) = {
    if (locals.var_cfd_p > 0.0) {
        (locals.var_cfd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfd_i = assign9760_e9649;
        locals.var_cfd_i_rv = 0.0;

        let (assign9770_e9655,) = {
    if (locals.var_psce_p > 0.0) {
        (locals.var_psce_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psce_i = assign9770_e9655;
        locals.var_psce_i_rv = 0.0;

        let (assign9780_e9666,) = {
    if (locals.var_psceb_p > 0.0) {
        let (assign9780_e9664,) = {
            if (locals.var_psceb_p < 1.0) {
                (locals.var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9780_e9664,)
    } else {
        (0.0,)
    }
};
        locals.var_psceb_i = assign9780_e9666;
        locals.var_psceb_i_rv = 0.0;

        let (assign9790_e9672,) = {
    if (locals.var_psced_p > 0.0) {
        (locals.var_psced_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psced_i = assign9790_e9672;
        locals.var_psced_i_rv = 0.0;

        let (assign9800_e9678,) = {
    if (locals.var_betn_p > 0.0) {
        (locals.var_betn_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betn_i = assign9800_e9678;
        locals.var_betn_i_rv = 0.0;

        locals.var_stbet_i = locals.var_stbet_p;
        locals.var_stbet_i_rv = 0.0;

        let (assign9820_e9685,) = {
    if (locals.var_mue_p > 0.0) {
        (locals.var_mue_p,)
    } else {
        (0.0,)
    }
};
        locals.var_mue_i = assign9820_e9685;
        locals.var_mue_i_rv = 0.0;

        locals.var_stmue_i = locals.var_stmue_p;
        locals.var_stmue_i_rv = 0.0;

        let (assign9840_e9692,) = {
    if (locals.var_themu_p > 0.0) {
        (locals.var_themu_p,)
    } else {
        (0.0,)
    }
};
        locals.var_themu_i = assign9840_e9692;
        locals.var_themu_i_rv = 0.0;

        locals.var_stthemu_i = locals.var_stthemu_p;
        locals.var_stthemu_i_rv = 0.0;

        let (assign9860_e9699,) = {
    if (locals.var_cs_p > 0.0) {
        (locals.var_cs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cs_i = assign9860_e9699;
        locals.var_cs_i_rv = 0.0;

        locals.var_stcs_i = locals.var_stcs_p;
        locals.var_stcs_i_rv = 0.0;

        let (assign9880_e9706,) = {
    if (locals.var_thecs_p > 0.0) {
        (locals.var_thecs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thecs_i = assign9880_e9706;
        locals.var_thecs_i_rv = 0.0;

        locals.var_stthecs_i = locals.var_stthecs_p;
        locals.var_stthecs_i_rv = 0.0;

        let (assign9900_e9713,) = {
    if (locals.var_xcor_p > 0.0) {
        (locals.var_xcor_p,)
    } else {
        (0.0,)
    }
};
        locals.var_xcor_i = assign9900_e9713;
        locals.var_xcor_i_rv = 0.0;

        locals.var_stxcor_i = locals.var_stxcor_p;
        locals.var_stxcor_i_rv = 0.0;

        locals.var_feta_i = locals.var_feta_p;
        locals.var_feta_i_rv = 0.0;

        let (assign9930_e9721,) = {
    if (locals.var_rs_p > 0.0) {
        (locals.var_rs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_rs_i = assign9930_e9721;
        locals.var_rs_i_rv = 0.0;

        locals.var_strs_i = locals.var_strs_p;
        locals.var_strs_i_rv = 0.0;

        let assign9950_e9725: f64 = (-0.5);
        let (assign9950_e9735,) = {
    if (locals.var_rsb_p > assign9950_e9725) {
        let (assign9950_e9732,) = {
            if (locals.var_rsb_p < 1.0) {
                (locals.var_rsb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9950_e9732,)
    } else {
        let assign9950_e9734: f64 = (-0.5);
        (assign9950_e9734,)
    }
};
        locals.var_rsb_i = assign9950_e9735;
        locals.var_rsb_i_rv = 0.0;

        let assign9960_e9738: f64 = (-0.5);
        let (assign9960_e9743,) = {
    if (locals.var_rsg_p > assign9960_e9738) {
        (locals.var_rsg_p,)
    } else {
        let assign9960_e9742: f64 = (-0.5);
        (assign9960_e9742,)
    }
};
        locals.var_rsg_i = assign9960_e9743;
        locals.var_rsg_i_rv = 0.0;

        let (assign9970_e9749,) = {
    if (locals.var_thesat_p > 0.0) {
        (locals.var_thesat_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesat_i = assign9970_e9749;
        locals.var_thesat_i_rv = 0.0;

        locals.var_stthesat_i = locals.var_stthesat_p;
        locals.var_stthesat_i_rv = 0.0;

        let assign9990_e9753: f64 = (-0.5);
        let (assign9990_e9763,) = {
    if (locals.var_thesatb_p > assign9990_e9753) {
        let (assign9990_e9760,) = {
            if (locals.var_thesatb_p < 1.0) {
                (locals.var_thesatb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9990_e9760,)
    } else {
        let assign9990_e9762: f64 = (-0.5);
        (assign9990_e9762,)
    }
};
        locals.var_thesatb_i = assign9990_e9763;
        locals.var_thesatb_i_rv = 0.0;

        let assign10000_e9766: f64 = (-0.5);
        let (assign10000_e9771,) = {
    if (locals.var_thesatg_p > assign10000_e9766) {
        (locals.var_thesatg_p,)
    } else {
        let assign10000_e9770: f64 = (-0.5);
        (assign10000_e9770,)
    }
};
        locals.var_thesatg_i = assign10000_e9771;
        locals.var_thesatg_i_rv = 0.0;

        let (assign10010_e9777,) = {
    if (locals.var_thesatt_p > 0.01) {
        (locals.var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        locals.var_thesatt_i = assign10010_e9777;
        locals.var_thesatt_i_rv = 0.0;

        let (assign10020_e9783,) = {
    if (locals.var_ax_p > 2.0) {
        (locals.var_ax_p,)
    } else {
        (2.0,)
    }
};
        locals.var_ax_i = assign10020_e9783;
        locals.var_ax_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10030_e9789,) = {
    if (locals.var_alp_p > 0.0) {
        (locals.var_alp_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp_i = assign10030_e9789;
        locals.var_alp_i_rv = 0.0;

        let (assign10040_e9795,) = {
    if (locals.var_alp1_p > 0.0) {
        (locals.var_alp1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1_i = assign10040_e9795;
        locals.var_alp1_i_rv = 0.0;

        let (assign10050_e9801,) = {
    if (locals.var_alp2_p > 0.0) {
        (locals.var_alp2_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp2_i = assign10050_e9801;
        locals.var_alp2_i_rv = 0.0;

        locals.var_vp_i = locals.var_vp_p;
        locals.var_vp_i_rv = 0.0;

        let (assign10070_e9808,) = {
    if (locals.var_a1_p > 0.0) {
        (locals.var_a1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a1_i = assign10070_e9808;
        locals.var_a1_i_rv = 0.0;

        locals.var_a2_i = locals.var_a2_p;
        locals.var_a2_i_rv = 0.0;

        locals.var_sta2_i = locals.var_sta2_p;
        locals.var_sta2_i_rv = 0.0;

        let (assign10100_e9816,) = {
    if (locals.var_a3_p > 0.0) {
        (locals.var_a3_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a3_i = assign10100_e9816;
        locals.var_a3_i_rv = 0.0;

        let (assign10110_e9822,) = {
    if (locals.var_a4_p > 0.0) {
        (locals.var_a4_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a4_i = assign10110_e9822;
        locals.var_a4_i_rv = 0.0;

        let (assign10120_e9828,) = {
    if (locals.var_imaxii_p > 1e-12) {
        (locals.var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        locals.var_imaxii_i = assign10120_e9828;
        locals.var_imaxii_i_rv = 0.0;

        locals.var_gco_i = locals.var_gco_p;
        locals.var_gco_i_rv = 0.0;

        let (assign10140_e9835,) = {
    if (locals.var_iginv_p > 0.0) {
        (locals.var_iginv_p,)
    } else {
        (0.0,)
    }
};
        locals.var_iginv_i = assign10140_e9835;
        locals.var_iginv_i_rv = 0.0;

        let (assign10150_e9841,) = {
    if (locals.var_igov_p > 0.0) {
        (locals.var_igov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igov_i = assign10150_e9841;
        locals.var_igov_i_rv = 0.0;

        let (assign10160_e9847,) = {
    if (locals.var_igovd_p > 0.0) {
        (locals.var_igovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igovd_i = assign10160_e9847;
        locals.var_igovd_i_rv = 0.0;

        locals.var_stig_i = locals.var_stig_p;
        locals.var_stig_i_rv = 0.0;

        locals.var_gc2_i = locals.var_gc2_p;
        locals.var_gc2_i_rv = 0.0;

        locals.var_gc3_i = locals.var_gc3_p;
        locals.var_gc3_i_rv = 0.0;

        locals.var_gc2ov_i = locals.var_gc2ov_p;
        locals.var_gc2ov_i_rv = 0.0;

        locals.var_gc3ov_i = locals.var_gc3ov_p;
        locals.var_gc3ov_i_rv = 0.0;

        locals.var_gc2ovd_i = locals.var_gc2ovd_p;
        locals.var_gc2ovd_i_rv = 0.0;

        locals.var_gc3ovd_i = locals.var_gc3ovd_p;
        locals.var_gc3ovd_i_rv = 0.0;

        locals.var_chib_i = locals.var_chib_p;
        locals.var_chib_i_rv = 0.0;

        let (assign10250_e9861,) = {
    if (locals.var_agidl_p > 0.0) {
        (locals.var_agidl_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidl_i = assign10250_e9861;
        locals.var_agidl_i_rv = 0.0;

        let (assign10260_e9867,) = {
    if (locals.var_agidld_p > 0.0) {
        (locals.var_agidld_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidld_i = assign10260_e9867;
        locals.var_agidld_i_rv = 0.0;

        locals.var_bgidl_i = locals.var_bgidl_p;
        locals.var_bgidl_i_rv = 0.0;

        locals.var_bgidld_i = locals.var_bgidld_p;
        locals.var_bgidld_i_rv = 0.0;

        locals.var_stbgidl_i = locals.var_stbgidl_p;
        locals.var_stbgidl_i_rv = 0.0;

        locals.var_stbgidld_i = locals.var_stbgidld_p;
        locals.var_stbgidld_i_rv = 0.0;

        locals.var_cgidl_i = locals.var_cgidl_p;
        locals.var_cgidl_i_rv = 0.0;

        locals.var_cgidld_i = locals.var_cgidld_p;
        locals.var_cgidld_i_rv = 0.0;

        let (assign10330_e9879,) = {
    if (locals.var_cox_p > 0.0) {
        (locals.var_cox_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cox_i = assign10330_e9879;
        locals.var_cox_i_rv = 0.0;

        locals.var_delvtac_i = locals.var_delvtac_p;
        locals.var_delvtac_i_rv = 0.0;

        let (assign10350_e9886,) = {
    if (locals.var_facneffac_p > 0.0) {
        (locals.var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_facneffac_i = assign10350_e9886;
        locals.var_facneffac_i_rv = 0.0;

        let (assign10360_e9892,) = {
    if (locals.var_thesatac_p > 0.0) {
        (locals.var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesatac_i = assign10360_e9892;
        locals.var_thesatac_i_rv = 0.0;

        let (assign10370_e9898,) = {
    if (locals.var_axac_p > 2.0) {
        (locals.var_axac_p,)
    } else {
        (2.0,)
    }
};
        locals.var_axac_i = assign10370_e9898;
        locals.var_axac_i_rv = 0.0;

        locals.var_alpac_i = locals.var_alpac_p;
        locals.var_alpac_i_rv = 0.0;

        let (assign10390_e9905,) = {
    if (locals.var_alp1ac_p > 0.0) {
        (locals.var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1ac_i = assign10390_e9905;
        locals.var_alp1ac_i_rv = 0.0;

        let (assign10400_e9911,) = {
    if (locals.var_cgov_p > 0.0) {
        (locals.var_cgov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgov_i = assign10400_e9911;
        locals.var_cgov_i_rv = 0.0;

        let (assign10410_e9917,) = {
    if (locals.var_cgovd_p > 0.0) {
        (locals.var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgovd_i = assign10410_e9917;
        locals.var_cgovd_i_rv = 0.0;

        locals.var_fcgovacc_i = locals.var_fcgovacc_p;
        locals.var_fcgovacc_i_rv = 0.0;

        locals.var_fcgovaccd_i = locals.var_fcgovaccd_p;
        locals.var_fcgovaccd_i_rv = 0.0;

        locals.var_cgovaccg_i = locals.var_cgovaccg_p;
        locals.var_cgovaccg_i_rv = 0.0;

        let (assign10450_e9926,) = {
    if (locals.var_cgbov_p > 0.0) {
        (locals.var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgbov_i = assign10450_e9926;
        locals.var_cgbov_i_rv = 0.0;

        let (assign10460_e9932,) = {
    if (locals.var_cinr_p > 0.0) {
        (locals.var_cinr_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinr_i = assign10460_e9932;
        locals.var_cinr_i_rv = 0.0;

        let (assign10470_e9938,) = {
    if (locals.var_cinrd_p > 0.0) {
        (locals.var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinrd_i = assign10470_e9938;
        locals.var_cinrd_i_rv = 0.0;

        locals.var_dvfbinr_i = locals.var_dvfbinr_p;
        locals.var_dvfbinr_i_rv = 0.0;

        locals.var_fcinrdep_i = locals.var_fcinrdep_p;
        locals.var_fcinrdep_i_rv = 0.0;

        locals.var_fcinracc_i = locals.var_fcinracc_p;
        locals.var_fcinracc_i_rv = 0.0;

        locals.var_axinr_i = locals.var_axinr_p;
        locals.var_axinr_i_rv = 0.0;

        locals.var_fnt_i = locals.var_fnt_p;
        locals.var_fnt_i_rv = 0.0;

        locals.var_vfbedge_i = locals.var_vfbedge_p;
        locals.var_vfbedge_i_rv = 0.0;

        locals.var_stvfbedge_i = locals.var_stvfbedge_p;
        locals.var_stvfbedge_i_rv = 0.0;

        locals.var_dphibedge_i = locals.var_dphibedge_p;
        locals.var_dphibedge_i_rv = 0.0;

        let (assign10630_e9994,) = {
    if (locals.var_neffedge_p > 1e20) {
        let (assign10630_e9992,) = {
            if (locals.var_neffedge_p < 1e26) {
                (locals.var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10630_e9992,)
    } else {
        (1e20,)
    }
};
        locals.var_neffedge_i = assign10630_e9994;
        locals.var_neffedge_i_rv = 0.0;

        let (assign10640_e10000,) = {
    if (locals.var_ctedge_p > 0.0) {
        (locals.var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ctedge_i = assign10640_e10000;
        locals.var_ctedge_i_rv = 0.0;

        let (assign10650_e10006,) = {
    if (locals.var_betnedge_p > 0.0) {
        (locals.var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betnedge_i = assign10650_e10006;
        locals.var_betnedge_i_rv = 0.0;

        locals.var_stbetedge_i = locals.var_stbetedge_p;
        locals.var_stbetedge_i_rv = 0.0;

        let (assign10670_e10013,) = {
    if (locals.var_psceedge_p > 0.0) {
        (locals.var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psceedge_i = assign10670_e10013;
        locals.var_psceedge_i_rv = 0.0;

        let (assign10680_e10024,) = {
    if (locals.var_pscebedge_p > 0.0) {
        let (assign10680_e10022,) = {
            if (locals.var_pscebedge_p < 1.0) {
                (locals.var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10680_e10022,)
    } else {
        (0.0,)
    }
};
        locals.var_pscebedge_i = assign10680_e10024;
        locals.var_pscebedge_i_rv = 0.0;

        let (assign10690_e10030,) = {
    if (locals.var_pscededge_p > 0.0) {
        (locals.var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_pscededge_i = assign10690_e10030;
        locals.var_pscededge_i_rv = 0.0;

        let (assign10700_e10036,) = {
    if (locals.var_cfedge_p > 0.0) {
        (locals.var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfedge_i = assign10700_e10036;
        locals.var_cfedge_i_rv = 0.0;

        let (assign10710_e10047,) = {
    if (locals.var_cfbedge_p > 0.0) {
        let (assign10710_e10045,) = {
            if (locals.var_cfbedge_p < 1.0) {
                (locals.var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10710_e10045,)
    } else {
        (0.0,)
    }
};
        locals.var_cfbedge_i = assign10710_e10047;
        locals.var_cfbedge_i_rv = 0.0;

        let (assign10720_e10053,) = {
    if (locals.var_cfdedge_p > 0.0) {
        (locals.var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfdedge_i = assign10720_e10053;
        locals.var_cfdedge_i_rv = 0.0;

        let assign10850_e10088: f64 = (p.p31 * locals.var_nf_i);
        let (assign10850_e10095,) = {
    if (assign10850_e10088 > 0.0) {
        let assign10850_e10093: f64 = (p.p31 * locals.var_nf_i);
        (assign10850_e10093,)
    } else {
        (0.0,)
    }
};
        locals.var_mult_inst = assign10850_e10095;
        locals.var_mult_inst_rv = 0.0;

        locals.var_factuo_i = p.p16;
        locals.var_factuo_i_rv = 0.0;

        locals.var_delvto_i = p.p15;
        locals.var_delvto_i_rv = 0.0;

        locals.var_factuoedge_i = p.p18;
        locals.var_factuoedge_i_rv = 0.0;

        locals.var_delvtoedge_i = p.p17;
        locals.var_delvtoedge_i_rv = 0.0;

        let assign10900_e10102: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign10900_e10102;
        locals.var_guard147_rv = 0.0;

        if (locals.var_guard147 != 0.0) {
            locals.var_toxovd_i = locals.var_toxov_i;
            locals.var_toxovd_i_rv = 0.0;
            locals.var_novd_i = locals.var_nov_i;
            locals.var_novd_i_rv = 0.0;
            locals.var_agidld_i = locals.var_agidl_i;
            locals.var_agidld_i_rv = 0.0;
            locals.var_bgidld_i = locals.var_bgidl_i;
            locals.var_bgidld_i_rv = 0.0;
            locals.var_stbgidld_i = locals.var_stbgidl_i;
            locals.var_stbgidld_i_rv = 0.0;
            locals.var_cgidld_i = locals.var_cgidl_i;
            locals.var_cgidld_i_rv = 0.0;
            locals.var_igovd_i = locals.var_igov_i;
            locals.var_igovd_i_rv = 0.0;
            locals.var_gc2ovd_i = locals.var_gc2ov_i;
            locals.var_gc2ovd_i_rv = 0.0;
            locals.var_gc3ovd_i = locals.var_gc3ov_i;
            locals.var_gc3ovd_i_rv = 0.0;
            locals.var_cgovd_i = locals.var_cgov_i;
            locals.var_cgovd_i_rv = 0.0;
            locals.var_fcgovaccd_i = locals.var_fcgovacc_i;
            locals.var_fcgovaccd_i_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard147 != 0.0) {
            locals.var_cinrd_i = locals.var_cinr_i;
            locals.var_cinrd_i_rv = 0.0;
        }

        let assign11040_e10157: f64 = (8.8541878176e-12 * locals.var_epsrox_i);
        locals.var_epsox = assign11040_e10157;
        locals.var_epsox_rv = 0.0;

        let assign11050_e10160: f64 = (locals.var_epsox / locals.var_tox_i);
        locals.var_coxprime = assign11050_e10160;
        locals.var_coxprime_rv = 0.0;

        let assign11060_e10163: f64 = (locals.var_tox_i * locals.var_tox_i);
        locals.var_tox_sq = assign11060_e10163;
        locals.var_tox_sq_rv = 0.0;

        let assign11070_e10166: f64 = (locals.var_coxprime / 1.6021918e-19);
        locals.var_cox_over_q = assign11070_e10166;
        locals.var_cox_over_q_rv = 0.0;

        let assign11080_e10169: f64 = (locals.var_facneffac_i * locals.var_neff_i);
        locals.var_neffac_i = assign11080_e10169;
        locals.var_neffac_i_rv = 0.0;

        let (assign11090_e10180,) = {
    if (locals.var_neffac_i > 1e20) {
        let (assign11090_e10178,) = {
            if (locals.var_neffac_i < 1e26) {
                (locals.var_neffac_i,)
            } else {
                (1e26,)
            }
        };
        (assign11090_e10178,)
    } else {
        (1e20,)
    }
};
        locals.var_neffac_i = assign11090_e10180;
        locals.var_neffac_i_rv = 0.0;

        locals.var_qq = 0.0;
        locals.var_qq_rv = 0.0;

        let assign11110_e10184: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign11110_e10184;
        locals.var_guard148_rv = 0.0;

        if (locals.var_guard148 != 0.0) {
            let assign11120_e10188: f64 = (0.4 * 5.951993);
            let assign11120_e10190: f64 = (assign11120_e10188 * p.p51);
            let assign11120_e10193: f64 = (locals.var_coxprime).powf(0.6666666666666666);
            let assign11120_e10194: f64 = (assign11120_e10190 * assign11120_e10193);
            locals.var_qq = assign11120_e10194;
            locals.var_qq_rv = 0.0;
        }

        let assign11130_e10199: f64 = (-1.0);
        let assign11130_e10200: f64 = if locals.var_chnl_type == assign11130_e10199 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign11130_e10200;
        locals.var_guard149_rv = 0.0;

        if ((locals.var_guard148 != 0.0) && (locals.var_guard149 != 0.0)) {
            let assign11140_e10206: f64 = (7.448711 / 5.951993);
            let assign11140_e10208: f64 = (assign11140_e10206 * locals.var_qq);
            locals.var_qq = assign11140_e10208;
            locals.var_qq_rv = 0.0;
        }

        let assign11150_e10213: f64 = (1e-8 * locals.var_coxprime);
        let assign11150_e10215: f64 = (assign11150_e10213 / locals.var_epssi);
        locals.var_e_eff0 = assign11150_e10215;
        locals.var_e_eff0_rv = 0.0;

        let assign11160_e10218: f64 = (0.5 * locals.var_feta_i);
        locals.var_eta_mu = assign11160_e10218;
        locals.var_eta_mu_rv = 0.0;

        locals.var_eta_mu1 = 0.5;
        locals.var_eta_mu1_rv = 0.0;

        let assign11180_e10222: f64 = (-1.0);
        let assign11180_e10223: f64 = if locals.var_chnl_type == assign11180_e10222 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign11180_e10223;
        locals.var_guard150_rv = 0.0;

        if (locals.var_guard150 != 0.0) {
            let assign11190_e10227: f64 = (0.3333333333333333 * locals.var_feta_i);
            locals.var_eta_mu = assign11190_e10227;
            locals.var_eta_mu_rv = 0.0;
        }

        if (locals.var_guard150 != 0.0) {
            locals.var_eta_mu1 = 0.3333333333333333;
            locals.var_eta_mu1_rv = 0.0;
        }

        let assign11210_e10236: f64 = (-2.0);
        let assign11210_e10238: f64 = (assign11210_e10236 / locals.var_ax_i);
        let assign11210_e10240: f64 = (assign11210_e10238 + 1.0);
        let assign11210_e10241: f64 = (2.0_f64).powf(assign11210_e10240);
        let assign11210_e10243: f64 = (assign11210_e10241 - 1.0);
        locals.var_temp = assign11210_e10243;
        locals.var_temp_rv = 0.0;

        let assign11220_e10246: f64 = (locals.var_temp - 1.0);
        let assign11220_e10249: f64 = (locals.var_temp - 1.0);
        let assign11220_e10250: f64 = (assign11220_e10246 * assign11220_e10249);
        let assign11220_e10253: f64 = (4.0 * locals.var_temp);
        let (assign11220_e10260,) = {
    if (assign11220_e10253 > 0.0001) {
        let assign11220_e10258: f64 = (4.0 * locals.var_temp);
        (assign11220_e10258,)
    } else {
        (0.0001,)
    }
};
        let assign11220_e10261: f64 = (assign11220_e10250 / assign11220_e10260);
        locals.var_ar = assign11220_e10261;
        locals.var_ar_rv = 0.0;

        let assign11230_e10264: f64 = (-2.0);
        let assign11230_e10266: f64 = (assign11230_e10264 / locals.var_axac_i);
        let assign11230_e10268: f64 = (assign11230_e10266 + 1.0);
        let assign11230_e10269: f64 = (2.0_f64).powf(assign11230_e10268);
        let assign11230_e10271: f64 = (assign11230_e10269 - 1.0);
        locals.var_temp = assign11230_e10271;
        locals.var_temp_rv = 0.0;

        let assign11240_e10274: f64 = (locals.var_temp - 1.0);
        let assign11240_e10277: f64 = (locals.var_temp - 1.0);
        let assign11240_e10278: f64 = (assign11240_e10274 * assign11240_e10277);
        let assign11240_e10281: f64 = (4.0 * locals.var_temp);
        let (assign11240_e10288,) = {
    if (assign11240_e10281 > 0.0001) {
        let assign11240_e10286: f64 = (4.0 * locals.var_temp);
        (assign11240_e10286,)
    } else {
        (0.0001,)
    }
};
        let assign11240_e10289: f64 = (assign11240_e10278 / assign11240_e10288);
        locals.var_arac = assign11240_e10289;
        locals.var_arac_rv = 0.0;

        let assign11250_e10292: f64 = (1.0 / locals.var_vp_i);
        locals.var_inv_vp = assign11250_e10292;
        locals.var_inv_vp_rv = 0.0;

        let assign11260_e10295: f64 = (locals.var_epsox / locals.var_toxov_i);
        locals.var_coxovprime = assign11260_e10295;
        locals.var_coxovprime_rv = 0.0;

        let assign11270_e10298: f64 = (locals.var_epsox / locals.var_toxovd_i);
        locals.var_coxovprime_d = assign11270_e10298;
        locals.var_coxovprime_d_rv = 0.0;

        let assign11280_e10301: f64 = (2.0 * 1.6021918e-19);
        let assign11280_e10303: f64 = (assign11280_e10301 * locals.var_nov_i);
        let assign11280_e10305: f64 = (assign11280_e10303 * locals.var_epssi);
        let assign11280_e10307: f64 = (assign11280_e10305 * locals.var_inv_phita);
        let assign11280_e10308: f64 = (assign11280_e10307).sqrt();
        let assign11280_e10310: f64 = (assign11280_e10308 / locals.var_coxovprime);
        locals.var_gov_s = assign11280_e10310;
        locals.var_gov_s_rv = 0.0;

        let assign11290_e10313: f64 = (2.0 * 1.6021918e-19);
        let assign11290_e10315: f64 = (assign11290_e10313 * locals.var_novd_i);
        let assign11290_e10317: f64 = (assign11290_e10315 * locals.var_epssi);
        let assign11290_e10319: f64 = (assign11290_e10317 * locals.var_inv_phita);
        let assign11290_e10320: f64 = (assign11290_e10319).sqrt();
        let assign11290_e10322: f64 = (assign11290_e10320 / locals.var_coxovprime_d);
        locals.var_gov_d = assign11290_e10322;
        locals.var_gov_d_rv = 0.0;

        let assign11300_e10325: f64 = (locals.var_gov_s * locals.var_gov_s);
        locals.var_gov2_s = assign11300_e10325;
        locals.var_gov2_s_rv = 0.0;

        let assign11310_e10328: f64 = (locals.var_gov_d * locals.var_gov_d);
        locals.var_gov2_d = assign11310_e10328;
        locals.var_gov2_d_rv = 0.0;

        let assign11320_e10331: f64 = (locals.var_cgovaccg_i * 0.005);
        let assign11320_e10333: f64 = (assign11320_e10331 * locals.var_inv_phita);
        let assign11320_e10334: f64 = (assign11320_e10333).exp();
        let assign11320_e10336: f64 = (assign11320_e10334 - 1.0);
        let assign11320_e10337: f64 = (assign11320_e10336).ln();
        let assign11320_e10339: f64 = (assign11320_e10337 / locals.var_cgovaccg_i);
        let assign11320_e10342: f64 = (0.005 * locals.var_inv_phita);
        let assign11320_e10343: f64 = (assign11320_e10342).exp();
        let assign11320_e10345: f64 = (assign11320_e10343 - 1.0);
        let assign11320_e10346: f64 = (assign11320_e10345).ln();
        let assign11320_e10347: f64 = (assign11320_e10339 - assign11320_e10346);
        locals.var_dxgb_ov_th = assign11320_e10347;
        locals.var_dxgb_ov_th_rv = 0.0;

        let assign11330_e10350: f64 = (0.5 * locals.var_gov_s);
        let assign11330_e10351: f64 = (assign11330_e10350).ln();
        let assign11330_e10353: f64 = (assign11330_e10351 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_s = assign11330_e10353;
        locals.var_dxgb_ov_s_rv = 0.0;

        let assign11340_e10356: f64 = (0.5 * locals.var_gov_d);
        let assign11340_e10357: f64 = (assign11340_e10356).ln();
        let assign11340_e10359: f64 = (assign11340_e10357 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_d = assign11340_e10359;
        locals.var_dxgb_ov_d_rv = 0.0;

        let assign11350_e10362: f64 = (1.0 / locals.var_gov_s);
        locals.var_inv_gov = assign11350_e10362;
        locals.var_inv_gov_rv = 0.0;

        let assign11360_e10365: f64 = (3.1 * locals.var_gov_s);
        let assign11360_e10367: f64 = (assign11360_e10365 + 8.5);
        locals.var_sp_ov_eps = assign11360_e10367;
        locals.var_sp_ov_eps_rv = 0.0;

        let assign11370_e10370: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_s = assign11370_e10370;
        locals.var_sp_ov_eps2_s_rv = 0.0;

        let assign11380_e10373: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11380_e10373;
        locals.var_sp_ov_delta_rv = 0.0;

        let assign11390_e10376: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign11390_e10376;
        locals.var_guard151_rv = 0.0;

        if (locals.var_guard151 != 0.0) {
            let assign11400_e10380: f64 = (64.0 * locals.var_inv_gov);
            locals.var_sp_ov_a_s = assign11400_e10380;
            locals.var_sp_ov_a_s_rv = 0.0;
        }

        let assign11410_e10385: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign11410_e10385;
        locals.var_guard152_rv = 0.0;

        if ((locals.var_guard151 == 0.0) && (locals.var_guard152 != 0.0)) {
            let assign11420_e10392: f64 = (22.0 * locals.var_inv_gov);
            let assign11420_e10394: f64 = (assign11420_e10392 + 3.0);
            locals.var_sp_ov_a_s = assign11420_e10394;
            locals.var_sp_ov_a_s_rv = 0.0;
        }

        let assign11430_e10399: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign11430_e10399;
        locals.var_guard153_rv = 0.0;

        if (((locals.var_guard151 == 0.0) && (locals.var_guard152 == 0.0)) && (locals.var_guard153 != 0.0)) {
            let assign11440_e10408: f64 = (-7.2);
            let assign11440_e10410: f64 = (assign11440_e10408 * locals.var_inv_gov);
            let assign11440_e10412: f64 = (assign11440_e10410 + 15.5);
            locals.var_sp_ov_a_s = assign11440_e10412;
            locals.var_sp_ov_a_s_rv = 0.0;
        }

        if (((locals.var_guard151 == 0.0) && (locals.var_guard152 == 0.0)) && (locals.var_guard153 == 0.0)) {
            locals.var_sp_ov_a_s = locals.var_gov_s;
            locals.var_sp_ov_a_s_rv = 0.0;
        }

        let assign11460_e10429: f64 = (locals.var_gov2_s * 0.5);
        let assign11460_e10430: f64 = (locals.var_sp_ov_delta + assign11460_e10429);
        let assign11460_e10435: f64 = (locals.var_gov2_s * 0.25);
        let assign11460_e10436: f64 = (locals.var_sp_ov_delta + assign11460_e10435);
        let assign11460_e10438: f64 = (assign11460_e10436 + locals.var_sp_ov_a_s);
        let assign11460_e10439: f64 = (assign11460_e10438).sqrt();
        let assign11460_e10440: f64 = (locals.var_gov_s * assign11460_e10439);
        let assign11460_e10441: f64 = (assign11460_e10430 - assign11460_e10440);
        locals.var_sp_ov_delta1_s = assign11460_e10441;
        locals.var_sp_ov_delta1_s_rv = 0.0;

        let assign11470_e10444: f64 = (1.0 / locals.var_gov_d);
        locals.var_inv_gov = assign11470_e10444;
        locals.var_inv_gov_rv = 0.0;

        let assign11480_e10447: f64 = (3.1 * locals.var_gov_d);
        let assign11480_e10449: f64 = (assign11480_e10447 + 8.5);
        locals.var_sp_ov_eps = assign11480_e10449;
        locals.var_sp_ov_eps_rv = 0.0;

        let assign11490_e10452: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_d = assign11490_e10452;
        locals.var_sp_ov_eps2_d_rv = 0.0;

        let assign11500_e10455: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11500_e10455;
        locals.var_sp_ov_delta_rv = 0.0;

        let assign11510_e10458: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign11510_e10458;
        locals.var_guard154_rv = 0.0;

        if (locals.var_guard154 != 0.0) {
            let assign11520_e10462: f64 = (64.0 * locals.var_inv_gov);
            locals.var_sp_ov_a_d = assign11520_e10462;
            locals.var_sp_ov_a_d_rv = 0.0;
        }

        let assign11530_e10467: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign11530_e10467;
        locals.var_guard155_rv = 0.0;

        if ((locals.var_guard154 == 0.0) && (locals.var_guard155 != 0.0)) {
            let assign11540_e10474: f64 = (22.0 * locals.var_inv_gov);
            let assign11540_e10476: f64 = (assign11540_e10474 + 3.0);
            locals.var_sp_ov_a_d = assign11540_e10476;
            locals.var_sp_ov_a_d_rv = 0.0;
        }

        let assign11550_e10481: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign11550_e10481;
        locals.var_guard156_rv = 0.0;

        if (((locals.var_guard154 == 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 != 0.0)) {
            let assign11560_e10490: f64 = (-7.2);
            let assign11560_e10492: f64 = (assign11560_e10490 * locals.var_inv_gov);
            let assign11560_e10494: f64 = (assign11560_e10492 + 15.5);
            locals.var_sp_ov_a_d = assign11560_e10494;
            locals.var_sp_ov_a_d_rv = 0.0;
        }

        if (((locals.var_guard154 == 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 == 0.0)) {
            locals.var_sp_ov_a_d = locals.var_gov_d;
            locals.var_sp_ov_a_d_rv = 0.0;
        }

        let assign11580_e10511: f64 = (locals.var_gov2_d * 0.5);
        let assign11580_e10512: f64 = (locals.var_sp_ov_delta + assign11580_e10511);
        let assign11580_e10517: f64 = (locals.var_gov2_d * 0.25);
        let assign11580_e10518: f64 = (locals.var_sp_ov_delta + assign11580_e10517);
        let assign11580_e10520: f64 = (assign11580_e10518 + locals.var_sp_ov_a_d);
        let assign11580_e10521: f64 = (assign11580_e10520).sqrt();
        let assign11580_e10522: f64 = (locals.var_gov_d * assign11580_e10521);
        let assign11580_e10523: f64 = (assign11580_e10512 - assign11580_e10522);
        locals.var_sp_ov_delta1_d = assign11580_e10523;
        locals.var_sp_ov_delta1_d_rv = 0.0;

        let assign11590_e10526: f64 = (locals.var_eg + locals.var_dphib_i);
        let assign11590_e10529: f64 = (2.0 * locals.var_phit);
        let assign11590_e10533: f64 = (-0.75);
        let assign11590_e10534: f64 = (locals.var_phibfac).powf(assign11590_e10533);
        let assign11590_e10535: f64 = (locals.var_neff_i * assign11590_e10534);
        let assign11590_e10537: f64 = (assign11590_e10535 * 4e-26);
        let assign11590_e10538: f64 = (assign11590_e10537).ln();
        let assign11590_e10539: f64 = (assign11590_e10529 * assign11590_e10538);
        let assign11590_e10540: f64 = (assign11590_e10526 + assign11590_e10539);
        locals.var_phib_dc = assign11590_e10540;
        locals.var_phib_dc_rv = 0.0;

        if (!(locals.var_phib_dc > 0.05)) {
            locals.var_phib_dc = 0.05;
            locals.var_phib_dc_rv = 0.0;
        }

        let assign11610_e10549: f64 = (2.0 * 1.6021918e-19);
        let assign11610_e10551: f64 = (assign11610_e10549 * locals.var_neff_i);
        let assign11610_e10553: f64 = (assign11610_e10551 * locals.var_epssi);
        let assign11610_e10555: f64 = (assign11610_e10553 * locals.var_inv_phit);
        let assign11610_e10556: f64 = (assign11610_e10555).sqrt();
        let assign11610_e10558: f64 = (assign11610_e10556 / locals.var_coxprime);
        locals.var_g_0_dc = assign11610_e10558;
        locals.var_g_0_dc_rv = 0.0;

        locals.var_kp = 0.0;
        locals.var_kp_rv = 0.0;

        locals.var_np = 0.0;
        locals.var_np_rv = 0.0;

        let assign11640_e10563: f64 = if locals.var_np_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign11640_e10563;
        locals.var_guard157_rv = 0.0;

        if (locals.var_guard157 != 0.0) {
            let assign11650_e10567: f64 = (80000000.0 / locals.var_tox_sq);
            locals.var_arg2max = assign11650_e10567;
            locals.var_arg2max_rv = 0.0;
        }

        if (locals.var_guard157 != 0.0) {
            let (assign11660_e10576,) = {
    if (locals.var_np_i > locals.var_arg2max) {
        (locals.var_np_i,)
    } else {
        (locals.var_arg2max,)
    }
};
            locals.var_np = assign11660_e10576;
            locals.var_np_rv = 0.0;
        }

        if (locals.var_guard157 != 0.0) {
            let (assign11670_e10585,) = {
    if (5e24 > locals.var_np) {
        (5e24,)
    } else {
        (locals.var_np,)
    }
};
            locals.var_np = assign11670_e10585;
            locals.var_np_rv = 0.0;
        }

        if (locals.var_guard157 != 0.0) {
            let assign11680_e10591: f64 = (2.0 * locals.var_coxprime);
            let assign11680_e10593: f64 = (assign11680_e10591 * locals.var_coxprime);
            let assign11680_e10595: f64 = (assign11680_e10593 * locals.var_phit);
            let assign11680_e10598: f64 = (1.6021918e-19 * locals.var_np);
            let assign11680_e10600: f64 = (assign11680_e10598 * locals.var_epssi);
            let assign11680_e10601: f64 = (assign11680_e10595 / assign11680_e10600);
            locals.var_kp = assign11680_e10601;
            locals.var_kp_rv = 0.0;
        }

        let assign11690_e10606: f64 = (100.0 * locals.var_phit);
        let assign11690_e10608: f64 = (assign11690_e10606 * locals.var_phit);
        locals.var_qlim2 = assign11690_e10608;
        locals.var_qlim2_rv = 0.0;

        let assign11700_e10611: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign11700_e10611;
        locals.var_guard158_rv = 0.0;

        if (locals.var_guard158 != 0.0) {
            let assign11710_e10615: f64 = (locals.var_phit * locals.var_g_0_dc);
            let assign11710_e10617: f64 = (assign11710_e10615 * locals.var_g_0_dc);
            let assign11710_e10619: f64 = (assign11710_e10617 * locals.var_phib_dc);
            let assign11710_e10620: f64 = (assign11710_e10619).sqrt();
            locals.var_qb0 = assign11710_e10620;
            locals.var_qb0_rv = 0.0;
        }

        if (locals.var_guard158 != 0.0) {
            let assign11720_e10626: f64 = (0.75 * locals.var_qq);
            let assign11720_e10629: f64 = (locals.var_qb0).powf(0.6666666666666666);
            let assign11720_e10630: f64 = (assign11720_e10626 * assign11720_e10629);
            locals.var_dphibq = assign11720_e10630;
            locals.var_dphibq_rv = 0.0;
        }

        if (locals.var_guard158 != 0.0) {
            let assign11730_e10636: f64 = (locals.var_phib_dc + locals.var_dphibq);
            locals.var_phib_dc = assign11730_e10636;
            locals.var_phib_dc_rv = 0.0;
        }

        if (locals.var_guard158 != 0.0) {
            let assign11740_e10644: f64 = (2.0 * 0.6666666666666666);
            let assign11740_e10646: f64 = (assign11740_e10644 * locals.var_dphibq);
            let assign11740_e10648: f64 = (assign11740_e10646 / locals.var_qb0);
            let assign11740_e10649: f64 = (1.0 + assign11740_e10648);
            let assign11740_e10650: f64 = (locals.var_g_0_dc * assign11740_e10649);
            locals.var_g_0_dc = assign11740_e10650;
            locals.var_g_0_dc_rv = 0.0;
        }

        let assign11750_e10654: f64 = (locals.var_phib_dc).sqrt();
        locals.var_sqrt_phib_dc = assign11750_e10654;
        locals.var_sqrt_phib_dc_rv = 0.0;

        let assign11760_e10657: f64 = (0.95 * locals.var_phib_dc);
        locals.var_phix_dc = assign11760_e10657;
        locals.var_phix_dc_rv = 0.0;

        let assign11770_e10660: f64 = (0.0025 * locals.var_phib_dc);
        let assign11770_e10662: f64 = (assign11770_e10660 * locals.var_phib_dc);
        locals.var_aphi_dc = assign11770_e10662;
        locals.var_aphi_dc_rv = 0.0;

        locals.var_bphi_dc = locals.var_aphi_dc;
        locals.var_bphi_dc_rv = 0.0;

        let assign11790_e10666: f64 = (locals.var_bphi_dc).sqrt();
        let assign11790_e10667: f64 = (0.5 * assign11790_e10666);
        locals.var_phix2 = assign11790_e10667;
        locals.var_phix2_rv = 0.0;

        let assign11800_e10671: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11800_e10673: f64 = assign11800_e10671;
        let assign11800_e10676: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11800_e10678: f64 = assign11800_e10676;
        let assign11800_e10681: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11800_e10683: f64 = assign11800_e10681;
        let assign11800_e10684: f64 = (assign11800_e10678 * assign11800_e10683);
        let assign11800_e10686: f64 = (assign11800_e10684 + locals.var_aphi_dc);
        let assign11800_e10687: f64 = (assign11800_e10686).sqrt();
        let assign11800_e10688: f64 = (assign11800_e10673 - assign11800_e10687);
        let assign11800_e10689: f64 = (0.5 * assign11800_e10688);
        locals.var_phix1_dc = assign11800_e10689;
        locals.var_phix1_dc_rv = 0.0;

        let assign11810_e10693: f64 = (locals.var_phib_dc + locals.var_eg);
        let assign11810_e10694: f64 = (0.5 * assign11810_e10693);
        locals.var_alpha_b = assign11810_e10694;
        locals.var_alpha_b_rv = 0.0;

        let assign11820_e10697: f64 = (locals.var_vsbnud_i + locals.var_phib_dc);
        let assign11820_e10698: f64 = (assign11820_e10697).sqrt();
        let assign11820_e10700: f64 = (assign11820_e10698 - locals.var_sqrt_phib_dc);
        locals.var_us1 = assign11820_e10700;
        locals.var_us1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign11830_e10703: f64 = (locals.var_vsbnud_i + locals.var_dvsbnud_i);
        let assign11830_e10705: f64 = (assign11830_e10703 + locals.var_phib_dc);
        let assign11830_e10706: f64 = (assign11830_e10705).sqrt();
        let assign11830_e10708: f64 = (assign11830_e10706 - locals.var_sqrt_phib_dc);
        let assign11830_e10710: f64 = (assign11830_e10708 - locals.var_us1);
        locals.var_us21 = assign11830_e10710;
        locals.var_us21_rv = 0.0;

        let assign11840_e10713: f64 = (locals.var_eg + locals.var_dphib_i);
        let assign11840_e10715: f64 = (assign11840_e10713 + locals.var_delvtac_i);
        let assign11840_e10718: f64 = (2.0 * locals.var_phit);
        let assign11840_e10722: f64 = (-0.75);
        let assign11840_e10723: f64 = (locals.var_phibfac).powf(assign11840_e10722);
        let assign11840_e10724: f64 = (locals.var_neffac_i * assign11840_e10723);
        let assign11840_e10726: f64 = (assign11840_e10724 * 4e-26);
        let assign11840_e10727: f64 = (assign11840_e10726).ln();
        let assign11840_e10728: f64 = (assign11840_e10718 * assign11840_e10727);
        let assign11840_e10729: f64 = (assign11840_e10715 + assign11840_e10728);
        locals.var_phib_ac = assign11840_e10729;
        locals.var_phib_ac_rv = 0.0;

        if (!(locals.var_phib_ac > 0.05)) {
            locals.var_phib_ac = 0.05;
            locals.var_phib_ac_rv = 0.0;
        }

        let assign11860_e10738: f64 = (2.0 * 1.6021918e-19);
        let assign11860_e10740: f64 = (assign11860_e10738 * locals.var_neffac_i);
        let assign11860_e10742: f64 = (assign11860_e10740 * locals.var_epssi);
        let assign11860_e10744: f64 = (assign11860_e10742 * locals.var_inv_phit);
        let assign11860_e10745: f64 = (assign11860_e10744).sqrt();
        let assign11860_e10747: f64 = (assign11860_e10745 / locals.var_coxprime);
        locals.var_g_0_ac = assign11860_e10747;
        locals.var_g_0_ac_rv = 0.0;

        let assign11870_e10750: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign11870_e10750;
        locals.var_guard159_rv = 0.0;

        if (locals.var_guard159 != 0.0) {
            let assign11880_e10754: f64 = (locals.var_phit * locals.var_g_0_ac);
            let assign11880_e10756: f64 = (assign11880_e10754 * locals.var_g_0_ac);
            let assign11880_e10758: f64 = (assign11880_e10756 * locals.var_phib_ac);
            let assign11880_e10759: f64 = (assign11880_e10758).sqrt();
            locals.var_qb0 = assign11880_e10759;
            locals.var_qb0_rv = 0.0;
        }

        if (locals.var_guard159 != 0.0) {
            let assign11890_e10765: f64 = (0.75 * locals.var_qq);
            let assign11890_e10768: f64 = (locals.var_qb0).powf(0.6666666666666666);
            let assign11890_e10769: f64 = (assign11890_e10765 * assign11890_e10768);
            locals.var_dphibq = assign11890_e10769;
            locals.var_dphibq_rv = 0.0;
        }

        if (locals.var_guard159 != 0.0) {
            let assign11900_e10775: f64 = (locals.var_phib_ac + locals.var_dphibq);
            locals.var_phib_ac = assign11900_e10775;
            locals.var_phib_ac_rv = 0.0;
        }

        if (locals.var_guard159 != 0.0) {
            let assign11910_e10783: f64 = (2.0 * 0.6666666666666666);
            let assign11910_e10785: f64 = (assign11910_e10783 * locals.var_dphibq);
            let assign11910_e10787: f64 = (assign11910_e10785 / locals.var_qb0);
            let assign11910_e10788: f64 = (1.0 + assign11910_e10787);
            let assign11910_e10789: f64 = (locals.var_g_0_ac * assign11910_e10788);
            locals.var_g_0_ac = assign11910_e10789;
            locals.var_g_0_ac_rv = 0.0;
        }

        let assign11920_e10794: f64 = (0.95 * locals.var_phib_ac);
        locals.var_phix_ac = assign11920_e10794;
        locals.var_phix_ac_rv = 0.0;

        let assign11930_e10797: f64 = (0.0025 * locals.var_phib_ac);
        let assign11930_e10799: f64 = (assign11930_e10797 * locals.var_phib_ac);
        locals.var_aphi_ac = assign11930_e10799;
        locals.var_aphi_ac_rv = 0.0;

        locals.var_bphi_ac = locals.var_aphi_ac;
        locals.var_bphi_ac_rv = 0.0;

        let assign11950_e10803: f64 = (locals.var_bphi_ac).sqrt();
        let assign11950_e10804: f64 = (0.5 * assign11950_e10803);
        locals.var_phix2 = assign11950_e10804;
        locals.var_phix2_rv = 0.0;

        let assign11960_e10808: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign11960_e10810: f64 = assign11960_e10808;
        let assign11960_e10813: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign11960_e10815: f64 = assign11960_e10813;
        let assign11960_e10818: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign11960_e10820: f64 = assign11960_e10818;
        let assign11960_e10821: f64 = (assign11960_e10815 * assign11960_e10820);
        let assign11960_e10823: f64 = (assign11960_e10821 + locals.var_aphi_ac);
        let assign11960_e10824: f64 = (assign11960_e10823).sqrt();
        let assign11960_e10825: f64 = (assign11960_e10810 - assign11960_e10824);
        let assign11960_e10826: f64 = (0.5 * assign11960_e10825);
        locals.var_phix1_ac = assign11960_e10826;
        locals.var_phix1_ac_rv = 0.0;

        let assign11970_e10830: f64 = (locals.var_stvfb_i * locals.var_delt);
        let assign11970_e10834: f64 = (locals.var_st2vfb_i * locals.var_delt);
        let assign11970_e10835: f64 = (1.0 + assign11970_e10834);
        let assign11970_e10836: f64 = (assign11970_e10830 * assign11970_e10835);
        let assign11970_e10837: f64 = (locals.var_vfb_i + assign11970_e10836);
        let assign11970_e10839: f64 = (assign11970_e10837 + locals.var_delvto_i);
        locals.var_vfb_t = assign11970_e10839;
        locals.var_vfb_t_rv = 0.0;

        let assign11980_e10842: f64 = (locals.var_stct_i * locals.var_ln_rtn);
        let assign11980_e10843: f64 = (assign11980_e10842).exp();
        locals.var_tf_ct = assign11980_e10843;
        locals.var_tf_ct_rv = 0.0;

        let assign11990_e10846: f64 = (locals.var_ct_i * locals.var_tf_ct);
        locals.var_ct_t = assign11990_e10846;
        locals.var_ct_t_rv = 0.0;

        let assign12000_e10849: f64 = (locals.var_ctg_i / locals.var_rtn);
        locals.var_ctg_t = assign12000_e10849;
        locals.var_ctg_t_rv = 0.0;

        let assign12010_e10852: f64 = (locals.var_stbet_i * locals.var_ln_rtn);
        let assign12010_e10853: f64 = (assign12010_e10852).exp();
        locals.var_tf_bet = assign12010_e10853;
        locals.var_tf_bet_rv = 0.0;

        let assign12020_e10856: f64 = (locals.var_betn_i * locals.var_tf_bet);
        locals.var_betn_t = assign12020_e10856;
        locals.var_betn_t_rv = 0.0;

        let assign12030_e10859: f64 = (locals.var_factuo_i * locals.var_betn_t);
        let assign12030_e10861: f64 = (assign12030_e10859 * locals.var_coxprime);
        locals.var_bet_i = assign12030_e10861;
        locals.var_bet_i_rv = 0.0;

        let assign12040_e10865: f64 = (locals.var_stthemu_i * locals.var_ln_rtn);
        let assign12040_e10866: f64 = (assign12040_e10865).exp();
        let assign12040_e10867: f64 = (locals.var_themu_i * assign12040_e10866);
        locals.var_themu_t = assign12040_e10867;
        locals.var_themu_t_rv = 0.0;

        let assign12050_e10870: f64 = (locals.var_stmue_i * locals.var_ln_rtn);
        let assign12050_e10871: f64 = (assign12050_e10870).exp();
        locals.var_tf_mue = assign12050_e10871;
        locals.var_tf_mue_rv = 0.0;

        let assign12060_e10874: f64 = (locals.var_mue_i * locals.var_tf_mue);
        locals.var_mue_t = assign12060_e10874;
        locals.var_mue_t_rv = 0.0;

        let assign12070_e10878: f64 = (locals.var_stthecs_i * locals.var_ln_rtn);
        let assign12070_e10879: f64 = (assign12070_e10878).exp();
        let assign12070_e10880: f64 = (locals.var_thecs_i * assign12070_e10879);
        locals.var_thecs_t = assign12070_e10880;
        locals.var_thecs_t_rv = 0.0;

        let assign12080_e10883: f64 = (locals.var_stcs_i * locals.var_ln_rtn);
        let assign12080_e10884: f64 = (assign12080_e10883).exp();
        locals.var_tf_cs = assign12080_e10884;
        locals.var_tf_cs_rv = 0.0;

        let assign12090_e10887: f64 = (locals.var_cs_i * locals.var_tf_cs);
        locals.var_cs_t = assign12090_e10887;
        locals.var_cs_t_rv = 0.0;

        let assign12100_e10890: f64 = (locals.var_stxcor_i * locals.var_ln_rtn);
        let assign12100_e10891: f64 = (assign12100_e10890).exp();
        locals.var_tf_xcor = assign12100_e10891;
        locals.var_tf_xcor_rv = 0.0;

        let assign12110_e10894: f64 = (locals.var_xcor_i * locals.var_tf_xcor);
        locals.var_xcor_t = assign12110_e10894;
        locals.var_xcor_t_rv = 0.0;

        let assign12120_e10897: f64 = (locals.var_strs_i * locals.var_ln_rtn);
        let assign12120_e10898: f64 = (assign12120_e10897).exp();
        locals.var_tf_ther = assign12120_e10898;
        locals.var_tf_ther_rv = 0.0;

        let assign12130_e10901: f64 = (locals.var_rs_i * locals.var_tf_ther);
        locals.var_rs_t = assign12130_e10901;
        locals.var_rs_t_rv = 0.0;

        let assign12140_e10904: f64 = (2.0 * locals.var_bet_i);
        let assign12140_e10906: f64 = (assign12140_e10904 * locals.var_rs_t);
        locals.var_ther_i = assign12140_e10906;
        locals.var_ther_i_rv = 0.0;

        let assign12150_e10909: f64 = (locals.var_stthesat_i * locals.var_ln_rtn);
        let assign12150_e10910: f64 = (assign12150_e10909).exp();
        locals.var_tf_thesat = assign12150_e10910;
        locals.var_tf_thesat_rv = 0.0;

        let assign12160_e10913: f64 = (locals.var_thesat_i * locals.var_tf_thesat);
        locals.var_thesat_t = assign12160_e10913;
        locals.var_thesat_t_rv = 0.0;

        let assign12170_e10916: f64 = (locals.var_thesatac_i * locals.var_tf_thesat);
        locals.var_thesatac_t = assign12170_e10916;
        locals.var_thesatac_t_rv = 0.0;

        let assign12180_e10919: f64 = (-locals.var_sta2_i);
        let assign12180_e10921: f64 = (assign12180_e10919 * locals.var_ln_rtn);
        let assign12180_e10922: f64 = (assign12180_e10921).exp();
        let assign12180_e10923: f64 = (locals.var_a2_i * assign12180_e10922);
        locals.var_a2_t = assign12180_e10923;
        locals.var_a2_t_rv = 0.0;

        let assign12190_e10926: f64 = (locals.var_fnt_i * 4.0);
        let assign12190_e10928: f64 = (assign12190_e10926 * 1.3806505e-23);
        let assign12190_e10930: f64 = (assign12190_e10928 * locals.var_tkd);
        locals.var_nt = assign12190_e10930;
        locals.var_nt_rv = 0.0;

        let assign12210_e10944: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard160 = assign12210_e10944;
        locals.var_guard160_rv = 0.0;

        if (locals.var_guard160 != 0.0) {
            let assign12220_e10949: f64 = (locals.var_stvfbedge_i * locals.var_delt);
            let assign12220_e10950: f64 = (locals.var_vfbedge_i + assign12220_e10949);
            let assign12220_e10952: f64 = (assign12220_e10950 + locals.var_delvtoedge_i);
            locals.var_vfbedge_t = assign12220_e10952;
            locals.var_vfbedge_t_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12230_e10958: f64 = (locals.var_stbetedge_i * locals.var_ln_rtn);
            let assign12230_e10959: f64 = (assign12230_e10958).exp();
            locals.var_tf_betedge = assign12230_e10959;
            locals.var_tf_betedge_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12240_e10965: f64 = (locals.var_betnedge_i * locals.var_tf_betedge);
            locals.var_betnedge_t = assign12240_e10965;
            locals.var_betnedge_t_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12250_e10971: f64 = (locals.var_factuoedge_i * locals.var_betnedge_t);
            let assign12250_e10973: f64 = (assign12250_e10971 * locals.var_coxprime);
            locals.var_betedge_i = assign12250_e10973;
            locals.var_betedge_i_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12260_e10981: f64 = (locals.var_ctedge_i * locals.var_rtn);
            let assign12260_e10982: f64 = (1.0 + assign12260_e10981);
            let assign12260_e10983: f64 = (locals.var_phit * assign12260_e10982);
            locals.var_phit0edge = assign12260_e10983;
            locals.var_phit0edge_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12270_e10989: f64 = (locals.var_eg + locals.var_dphibedge_i);
            let assign12270_e10992: f64 = (2.0 * locals.var_phit0edge);
            let assign12270_e10996: f64 = (-0.75);
            let assign12270_e10997: f64 = (locals.var_phibfac).powf(assign12270_e10996);
            let assign12270_e10998: f64 = (locals.var_neffedge_i * assign12270_e10997);
            let assign12270_e11000: f64 = (assign12270_e10998 * 4e-26);
            let assign12270_e11001: f64 = (assign12270_e11000).ln();
            let assign12270_e11002: f64 = (assign12270_e10992 * assign12270_e11001);
            let assign12270_e11003: f64 = (assign12270_e10989 + assign12270_e11002);
            locals.var_phibedge = assign12270_e11003;
            locals.var_phibedge_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let (assign12280_e11012,) = {
    if (locals.var_phibedge > 0.05) {
        (locals.var_phibedge,)
    } else {
        (0.05,)
    }
};
            locals.var_phibedge = assign12280_e11012;
            locals.var_phibedge_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12290_e11018: f64 = (2.0 * 1.6021918e-19);
            let assign12290_e11020: f64 = (assign12290_e11018 * locals.var_neffedge_i);
            let assign12290_e11022: f64 = (assign12290_e11020 * locals.var_epssi);
            let assign12290_e11024: f64 = (assign12290_e11022 * locals.var_inv_phit);
            let assign12290_e11025: f64 = (assign12290_e11024).sqrt();
            let assign12290_e11027: f64 = (assign12290_e11025 / locals.var_coxprime);
            locals.var_gfedge = assign12290_e11027;
            locals.var_gfedge_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12300_e11033: f64 = (locals.var_gfedge * locals.var_gfedge);
            locals.var_gfedge2 = assign12300_e11033;
            locals.var_gfedge2_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12310_e11038: f64 = (locals.var_gfedge2).ln();
            locals.var_lngfedge2 = assign12310_e11038;
            locals.var_lngfedge2_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12320_e11044: f64 = (0.95 * locals.var_phibedge);
            locals.var_phixedge = assign12320_e11044;
            locals.var_phixedge_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12330_e11050: f64 = (0.0025 * locals.var_phibedge);
            let assign12330_e11052: f64 = (assign12330_e11050 * locals.var_phibedge);
            locals.var_aphiedge = assign12330_e11052;
            locals.var_aphiedge_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            locals.var_bphiedge = locals.var_aphiedge;
            locals.var_bphiedge_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12350_e11062: f64 = (locals.var_bphiedge).sqrt();
            let assign12350_e11063: f64 = (0.5 * assign12350_e11062);
            locals.var_phix2edge = assign12350_e11063;
            locals.var_phix2edge_rv = 0.0;
        }

        if (locals.var_guard160 != 0.0) {
            let assign12360_e11070: f64 = (locals.var_phixedge - locals.var_phix2edge);
            let assign12360_e11072: f64 = assign12360_e11070;
            let assign12360_e11075: f64 = (locals.var_phixedge - locals.var_phix2edge);
            let assign12360_e11077: f64 = assign12360_e11075;
            let assign12360_e11080: f64 = (locals.var_phixedge - locals.var_phix2edge);
            let assign12360_e11082: f64 = assign12360_e11080;
            let assign12360_e11083: f64 = (assign12360_e11077 * assign12360_e11082);
            let assign12360_e11085: f64 = (assign12360_e11083 + locals.var_aphiedge);
            let assign12360_e11086: f64 = (assign12360_e11085).sqrt();
            let assign12360_e11087: f64 = (assign12360_e11072 - assign12360_e11086);
            let assign12360_e11088: f64 = (0.5 * assign12360_e11087);
            locals.var_phix1edge = assign12360_e11088;
            locals.var_phix1edge_rv = 0.0;
        }

        if (locals.var_guard160 == 0.0) {
            locals.var_vfbedge_t = 0.0;
            locals.var_vfbedge_t_rv = 0.0;
            locals.var_tf_betedge = 1.0;
            locals.var_tf_betedge_rv = 0.0;
            locals.var_betnedge_t = 0.0;
            locals.var_betnedge_t_rv = 0.0;
            locals.var_betedge_i = 0.0;
            locals.var_betedge_i_rv = 0.0;
            locals.var_phit0edge = locals.var_phit;
            locals.var_phit0edge_rv = 0.0;
            locals.var_phibedge = 0.0;
            locals.var_phibedge_rv = 0.0;
            locals.var_gfedge = 1.0;
            locals.var_gfedge_rv = 0.0;
            locals.var_gfedge2 = 1.0;
            locals.var_gfedge2_rv = 0.0;
            locals.var_lngfedge2 = 0.0;
            locals.var_lngfedge2_rv = 0.0;
            locals.var_phixedge = 0.0;
            locals.var_phixedge_rv = 0.0;
            locals.var_aphiedge = 0.0;
            locals.var_aphiedge_rv = 0.0;
            locals.var_bphiedge = 0.0;
            locals.var_bphiedge_rv = 0.0;
            locals.var_phix2edge = 0.0;
            locals.var_phix2edge_rv = 0.0;
            locals.var_phix1edge = 0.0;
            locals.var_phix1edge_rv = 0.0;
        }

        let assign12550_e11193: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign12550_e11193;
        locals.var_inv_chib_rv = 0.0;

        let assign12560_e11196: f64 = (4.0 * 0.3333333333333333);
        let assign12560_e11199: f64 = (2.0 * 1.6021918e-19);
        let assign12560_e11201: f64 = (assign12560_e11199 * 9.1093826e-31);
        let assign12560_e11203: f64 = (assign12560_e11201 * locals.var_chib_i);
        let assign12560_e11204: f64 = (assign12560_e11203).sqrt();
        let assign12560_e11205: f64 = (assign12560_e11196 * assign12560_e11204);
        let assign12560_e11207: f64 = (assign12560_e11205 / 1.05457168e-34);
        locals.var_b_fact = assign12560_e11207;
        locals.var_b_fact_rv = 0.0;

        let assign12570_e11210: f64 = (locals.var_b_fact * locals.var_tox_i);
        locals.var_bch = assign12570_e11210;
        locals.var_bch_rv = 0.0;

        let assign12580_e11213: f64 = (locals.var_b_fact * locals.var_toxov_i);
        locals.var_bov = assign12580_e11213;
        locals.var_bov_rv = 0.0;

        let assign12590_e11216: f64 = (locals.var_b_fact * locals.var_toxovd_i);
        locals.var_bov_d = assign12590_e11216;
        locals.var_bov_d_rv = 0.0;

        locals.var_gcq = 0.0;
        locals.var_gcq_rv = 0.0;

        let assign12610_e11220: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign12610_e11220;
        locals.var_guard161_rv = 0.0;

        if (locals.var_guard161 != 0.0) {
            let assign12620_e11223: f64 = (-0.495);
            let assign12620_e11225: f64 = (assign12620_e11223 * locals.var_gc2_i);
            let assign12620_e11227: f64 = (assign12620_e11225 / locals.var_gc3_i);
            locals.var_gcq = assign12620_e11227;
            locals.var_gcq_rv = 0.0;
        }

        locals.var_gcqov = 0.0;
        locals.var_gcqov_rv = 0.0;

        let assign12640_e11233: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign12640_e11233;
        locals.var_guard162_rv = 0.0;

        if (locals.var_guard162 != 0.0) {
            let assign12650_e11236: f64 = (-0.495);
            let assign12650_e11238: f64 = (assign12650_e11236 * locals.var_gc2ov_i);
            let assign12650_e11240: f64 = (assign12650_e11238 / locals.var_gc3ov_i);
            locals.var_gcqov = assign12650_e11240;
            locals.var_gcqov_rv = 0.0;
        }

        let assign12660_e11245: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign12660_e11245;
        locals.var_guard163_rv = 0.0;

        if (locals.var_guard163 != 0.0) {
            let assign12670_e11248: f64 = (-0.495);
            let assign12670_e11250: f64 = (assign12670_e11248 * locals.var_gc2ovd_i);
            let assign12670_e11252: f64 = (assign12670_e11250 / locals.var_gc3ovd_i);
            locals.var_gcqovd = assign12670_e11252;
            locals.var_gcqovd_rv = 0.0;
        }

        let assign12680_e11257: f64 = (locals.var_rta).powf(locals.var_stig_i);
        locals.var_tf_ig = assign12680_e11257;
        locals.var_tf_ig_rv = 0.0;

        let assign12690_e11260: f64 = (locals.var_iginv_i * locals.var_tf_ig);
        locals.var_iginv_i = assign12690_e11260;
        locals.var_iginv_i_rv = 0.0;

        let assign12700_e11263: f64 = (locals.var_igov_i * locals.var_tf_ig);
        locals.var_igov_i = assign12700_e11263;
        locals.var_igov_i_rv = 0.0;

        let assign12710_e11266: f64 = (locals.var_igovd_i * locals.var_tf_ig);
        locals.var_igovd_i = assign12710_e11266;
        locals.var_igovd_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let assign12740_e11284: f64 = (locals.var_stbgidl_i * locals.var_delta);
        let assign12740_e11285: f64 = (1.0 + assign12740_e11284);
        let (assign12740_e11294,) = {
    if (assign12740_e11285 > 0.0) {
        let assign12740_e11291: f64 = (locals.var_stbgidl_i * locals.var_delta);
        let assign12740_e11292: f64 = (1.0 + assign12740_e11291);
        (assign12740_e11292,)
    } else {
        (0.0,)
    }
};
        locals.var_b_fact = assign12740_e11294;
        locals.var_b_fact_rv = 0.0;

        let assign12750_e11297: f64 = (locals.var_bgidl_i * locals.var_b_fact);
        locals.var_bgidl_t = assign12750_e11297;
        locals.var_bgidl_t_rv = 0.0;

        let assign12760_e11300: f64 = (locals.var_bgidl_t * locals.var_toxov_i);
        let assign12760_e11302: f64 = (assign12760_e11300 * 500000000.0);
        locals.var_bgidls = assign12760_e11302;
        locals.var_bgidls_rv = 0.0;

        let assign12770_e11306: f64 = (locals.var_stbgidld_i * locals.var_delta);
        let assign12770_e11307: f64 = (1.0 + assign12770_e11306);
        let (assign12770_e11316,) = {
    if (assign12770_e11307 > 0.0) {
        let assign12770_e11313: f64 = (locals.var_stbgidld_i * locals.var_delta);
        let assign12770_e11314: f64 = (1.0 + assign12770_e11313);
        (assign12770_e11314,)
    } else {
        (0.0,)
    }
};
        locals.var_b_fact = assign12770_e11316;
        locals.var_b_fact_rv = 0.0;

        let assign12780_e11319: f64 = (locals.var_bgidld_i * locals.var_b_fact);
        locals.var_bgidld_t = assign12780_e11319;
        locals.var_bgidld_t_rv = 0.0;

        let assign12790_e11322: f64 = (locals.var_bgidld_t * locals.var_toxovd_i);
        let assign12790_e11324: f64 = (assign12790_e11322 * 500000000.0);
        locals.var_bgidlds = assign12790_e11324;
        locals.var_bgidlds_rv = 0.0;

        locals.var_vinr_max = 0.0;
        locals.var_vinr_max_rv = 0.0;

        let assign12810_e11328: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard164 = assign12810_e11328;
        locals.var_guard164_rv = 0.0;

        if (locals.var_guard164 != 0.0) {
            let assign12820_e11332: f64 = (0.75 / locals.var_fcinracc_i);
            locals.var_vinr_max = assign12820_e11332;
            locals.var_vinr_max_rv = 0.0;
        }

        let assign12830_e11337: f64 = (locals.var_axinr_i * locals.var_axinr_i);
        locals.var_ainr = assign12830_e11337;
        locals.var_ainr_rv = 0.0;

        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_temp__blk936_rv = 0.0;

        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_temp1_rv = 0.0;

        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_temp2_rv = 0.0;

        let assign40320_e53455: f64 = 1.0;
        let assign40320_e53456: f64 = if locals.var_chnl_type == assign40320_e53455 { 1.0 } else { 0.0 };
        locals.var_guard1011 = assign40320_e53456;
        locals.var_guard1011_rv = 0.0;

        if (locals.var_guard1011 != 0.0) {
            (locals.var_v_gs, locals.var_v_gs_dn5, locals.var_v_gs_dn6, locals.var_v_gs_dn7, ) = ((nv5 - nv6), 1.0, -1.0, 0.0, );
            locals.var_v_gs_rv = 0.0;
            (locals.var_v_ds, locals.var_v_ds_dn6, locals.var_v_ds_dn7, ) = ((nv7 - nv6), -1.0, 1.0, );
            locals.var_v_ds_rv = 0.0;
            (locals.var_v_sb, locals.var_v_sb_dn6, locals.var_v_sb_dn7, locals.var_v_sb_dn8, ) = ((nv6 - nv8), 1.0, 0.0, -1.0, );
            locals.var_v_sb_rv = 0.0;
        }

        if (locals.var_guard1011 == 0.0) {
            let assign40380_e53482: f64 = (-(nv5 - nv6));
            (locals.var_v_gs, locals.var_v_gs_dn5, locals.var_v_gs_dn6, locals.var_v_gs_dn7, ) = (assign40380_e53482, (-1.0), 1.0, 0.0, );
            locals.var_v_gs_rv = 0.0;
        }

        if (locals.var_guard1011 == 0.0) {
            let assign40390_e53488: f64 = (-(nv7 - nv6));
            (locals.var_v_ds, locals.var_v_ds_dn6, locals.var_v_ds_dn7, ) = (assign40390_e53488, 1.0, (-1.0), );
            locals.var_v_ds_rv = 0.0;
        }

        if (locals.var_guard1011 == 0.0) {
            let assign40400_e53494: f64 = (-(nv6 - nv8));
            (locals.var_v_sb, locals.var_v_sb_dn6, locals.var_v_sb_dn7, locals.var_v_sb_dn8, ) = (assign40400_e53494, (-1.0), 0.0, 1.0, );
            locals.var_v_sb_rv = 0.0;
        }

        let assign40430_e53509: f64 = (locals.var_v_gs + locals.var_v_sb);
        (locals.var_vgb, locals.var_vgb_dn5, locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, ) = (assign40430_e53509, locals.var_v_gs_dn5, (locals.var_v_gs_dn6 + locals.var_v_sb_dn6), (locals.var_v_gs_dn7 + locals.var_v_sb_dn7), locals.var_v_sb_dn8, );
        locals.var_vgb_rv = 0.0;

        (locals.var_vgsprime, locals.var_vgsprime_dn5, locals.var_vgsprime_dn6, locals.var_vgsprime_dn7, ) = (locals.var_v_gs, locals.var_v_gs_dn5, locals.var_v_gs_dn6, locals.var_v_gs_dn7, );
        locals.var_vgsprime_rv = 0.0;

        (locals.var_vsbprime, locals.var_vsbprime_dn6, locals.var_vsbprime_dn7, locals.var_vsbprime_dn8, ) = (locals.var_v_sb, locals.var_v_sb_dn6, locals.var_v_sb_dn7, locals.var_v_sb_dn8, );
        locals.var_vsbprime_rv = 0.0;

        let assign40460_e53514: f64 = (locals.var_v_ds + locals.var_v_sb);
        (locals.var_vdbprime, locals.var_vdbprime_dn6, locals.var_vdbprime_dn7, locals.var_vdbprime_dn8, ) = (assign40460_e53514, (locals.var_v_ds_dn6 + locals.var_v_sb_dn6), (locals.var_v_ds_dn7 + locals.var_v_sb_dn7), locals.var_v_sb_dn8, );
        locals.var_vdbprime_rv = 0.0;

        let assign40470_e53517: f64 = (locals.var_v_gs - locals.var_v_ds);
        (locals.var_vgdprime, locals.var_vgdprime_dn5, locals.var_vgdprime_dn6, locals.var_vgdprime_dn7, ) = (assign40470_e53517, locals.var_v_gs_dn5, (locals.var_v_gs_dn6 - locals.var_v_ds_dn6), (locals.var_v_gs_dn7 - locals.var_v_ds_dn7), );
        locals.var_vgdprime_rv = 0.0;

        let assign40480_e53519: f64 = (-locals.var_vgsprime);
        let assign40480_e53521: f64 = (assign40480_e53519 * locals.var_inv_phita);
        (locals.var_xgs_ov, locals.var_xgs_ov_dn5, locals.var_xgs_ov_dn6, locals.var_xgs_ov_dn7, ) = (assign40480_e53521, ((-locals.var_vgsprime_dn5) * locals.var_inv_phita), ((-locals.var_vgsprime_dn6) * locals.var_inv_phita), ((-locals.var_vgsprime_dn7) * locals.var_inv_phita), );
        locals.var_xgs_ov_rv = 0.0;

        let assign40490_e53523: f64 = (-locals.var_vgdprime);
        let assign40490_e53525: f64 = (assign40490_e53523 * locals.var_inv_phita);
        (locals.var_xgd_ov, locals.var_xgd_ov_dn5, locals.var_xgd_ov_dn6, locals.var_xgd_ov_dn7, ) = (assign40490_e53525, ((-locals.var_vgdprime_dn5) * locals.var_inv_phita), ((-locals.var_vgdprime_dn6) * locals.var_inv_phita), ((-locals.var_vgdprime_dn7) * locals.var_inv_phita), );
        locals.var_xgd_ov_rv = 0.0;

        let assign40500_e53528: f64 = (locals.var_vgb - locals.var_vfb_t);
        let assign40500_e53529: f64 = (-assign40500_e53528);
        let assign40500_e53531: f64 = (assign40500_e53529 * locals.var_inv_phita);
        (locals.var_xgb_ov, locals.var_xgb_ov_dn5, locals.var_xgb_ov_dn6, locals.var_xgb_ov_dn7, locals.var_xgb_ov_dn8, ) = (assign40500_e53531, ((-locals.var_vgb_dn5) * locals.var_inv_phita), ((-locals.var_vgb_dn6) * locals.var_inv_phita), ((-locals.var_vgb_dn7) * locals.var_inv_phita), ((-locals.var_vgb_dn8) * locals.var_inv_phita), );
        locals.var_xgb_ov_rv = 0.0;

        locals.var_sigvds = 1.0;
        locals.var_sigvds_rv = 0.0;

        let assign40520_e53535: f64 = if locals.var_v_ds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1012 = assign40520_e53535;
        locals.var_guard1012_rv = 0.0;

        if (locals.var_guard1012 != 0.0) {
            let assign40530_e53538: f64 = (-1.0);
            locals.var_sigvds = assign40530_e53538;
            locals.var_sigvds_rv = 0.0;
        }

        if (locals.var_guard1012 != 0.0) {
            let assign40540_e53544: f64 = (locals.var_v_gs - locals.var_v_ds);
            (locals.var_v_gs, locals.var_v_gs_dn5, locals.var_v_gs_dn6, locals.var_v_gs_dn7, ) = (assign40540_e53544, locals.var_v_gs_dn5, (locals.var_v_gs_dn6 - locals.var_v_ds_dn6), (locals.var_v_gs_dn7 - locals.var_v_ds_dn7), );
            locals.var_v_gs_rv = 0.0;
        }

        if (locals.var_guard1012 != 0.0) {
            let assign40550_e53550: f64 = (locals.var_v_sb + locals.var_v_ds);
            (locals.var_v_sb, locals.var_v_sb_dn6, locals.var_v_sb_dn7, locals.var_v_sb_dn8, ) = (assign40550_e53550, (locals.var_v_sb_dn6 + locals.var_v_ds_dn6), (locals.var_v_sb_dn7 + locals.var_v_ds_dn7), locals.var_v_sb_dn8, );
            locals.var_v_sb_rv = 0.0;
        }

        if (locals.var_guard1012 != 0.0) {
            let assign40560_e53555: f64 = (-locals.var_v_ds);
            (locals.var_v_ds, locals.var_v_ds_dn6, locals.var_v_ds_dn7, ) = (assign40560_e53555, (-locals.var_v_ds_dn6), (-locals.var_v_ds_dn7), );
            locals.var_v_ds_rv = 0.0;
        }

        let assign40570_e53560: f64 = (locals.var_v_ds + locals.var_v_sb);
        (locals.var_v_db, locals.var_v_db_dn6, locals.var_v_db_dn7, locals.var_v_db_dn8, ) = (assign40570_e53560, (locals.var_v_ds_dn6 + locals.var_v_sb_dn6), (locals.var_v_ds_dn7 + locals.var_v_sb_dn7), locals.var_v_sb_dn8, );
        locals.var_v_db_rv = 0.0;

        let assign40580_e53563: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40580_e53566: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40580_e53568: f64 = (assign40580_e53566 + 0.01);
        let assign40580_e53569: f64 = (assign40580_e53568).sqrt();
        let assign40580_e53571: f64 = (assign40580_e53569 + 0.1);
        let assign40580_e53572: f64 = (assign40580_e53563 / assign40580_e53571);
        (locals.var_vdsx, locals.var_vdsx_dn6, locals.var_vdsx_dn7, ) = (assign40580_e53572, (((((locals.var_v_ds_dn6 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn6)) * assign40580_e53571) - (assign40580_e53563 * (((locals.var_v_ds_dn6 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn6)) / (2.0 * assign40580_e53569)))) / (assign40580_e53571 * assign40580_e53571)), (((((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) * assign40580_e53571) - (assign40580_e53563 * (((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) / (2.0 * assign40580_e53569)))) / (assign40580_e53571 * assign40580_e53571)), );
        locals.var_vdsx_rv = 0.0;

        let assign40590_e53576: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign40590_e53579: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign40590_e53582: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign40590_e53583: f64 = (assign40590_e53579 * assign40590_e53582);
        let assign40590_e53585: f64 = (assign40590_e53583 + locals.var_bphi_dc);
        let assign40590_e53586: f64 = (assign40590_e53585).sqrt();
        let assign40590_e53587: f64 = (assign40590_e53576 - assign40590_e53586);
        let assign40590_e53588: f64 = (0.5 * assign40590_e53587);
        let assign40590_e53590: f64 = (assign40590_e53588 + locals.var_phix_dc);
        (locals.var_v_xb, locals.var_v_xb_dn6, locals.var_v_xb_dn7, locals.var_v_xb_dn8, ) = (assign40590_e53590, (0.5 * ((locals.var_v_db_dn6 + locals.var_v_sb_dn6) - ((((locals.var_v_db_dn6 - locals.var_v_sb_dn6) * assign40590_e53582) + (assign40590_e53579 * (locals.var_v_db_dn6 - locals.var_v_sb_dn6))) / (2.0 * assign40590_e53586)))), (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign40590_e53582) + (assign40590_e53579 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign40590_e53586)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign40590_e53582) + (assign40590_e53579 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign40590_e53586)))), );
        locals.var_v_xb_rv = 0.0;

        (locals.var_v_xb_dc_tmp, locals.var_v_xb_dc_tmp_dn6, locals.var_v_xb_dc_tmp_dn7, locals.var_v_xb_dc_tmp_dn8, ) = (locals.var_v_xb, locals.var_v_xb_dn6, locals.var_v_xb_dn7, locals.var_v_xb_dn8, );
        locals.var_v_xb_dc_tmp_rv = 0.0;

        let assign40610_e53596: f64 = locals.var_v_xb;
        let assign40610_e53599: f64 = locals.var_v_xb;
        let assign40610_e53602: f64 = locals.var_v_xb;
        let assign40610_e53603: f64 = (assign40610_e53599 * assign40610_e53602);
        let assign40610_e53605: f64 = (assign40610_e53603 + locals.var_aphi_dc);
        let assign40610_e53606: f64 = (assign40610_e53605).sqrt();
        let assign40610_e53607: f64 = (assign40610_e53596 - assign40610_e53606);
        let assign40610_e53608: f64 = (0.5 * assign40610_e53607);
        let assign40610_e53609: f64 = (locals.var_v_sb - assign40610_e53608);
        let assign40610_e53611: f64 = (assign40610_e53609 + locals.var_phix1_dc);
        (locals.var_vsbstar_dc, locals.var_vsbstar_dc_dn5, locals.var_vsbstar_dc_dn6, locals.var_vsbstar_dc_dn7, locals.var_vsbstar_dc_dn8, ) = (assign40610_e53611, 0.0, (locals.var_v_sb_dn6 - (0.5 * (locals.var_v_xb_dn6 - (((locals.var_v_xb_dn6 * assign40610_e53602) + (assign40610_e53599 * locals.var_v_xb_dn6)) / (2.0 * assign40610_e53606))))), (locals.var_v_sb_dn7 - (0.5 * (locals.var_v_xb_dn7 - (((locals.var_v_xb_dn7 * assign40610_e53602) + (assign40610_e53599 * locals.var_v_xb_dn7)) / (2.0 * assign40610_e53606))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_v_xb_dn8 - (((locals.var_v_xb_dn8 * assign40610_e53602) + (assign40610_e53599 * locals.var_v_xb_dn8)) / (2.0 * assign40610_e53606))))), );
        locals.var_vsbstar_dc_rv = 0.0;

        (locals.var_vsbstar_dc_tmp, locals.var_vsbstar_dc_tmp_dn5, locals.var_vsbstar_dc_tmp_dn6, locals.var_vsbstar_dc_tmp_dn7, locals.var_vsbstar_dc_tmp_dn8, ) = (locals.var_vsbstar_dc, locals.var_vsbstar_dc_dn5, locals.var_vsbstar_dc_dn6, locals.var_vsbstar_dc_dn7, locals.var_vsbstar_dc_dn8, );
        locals.var_vsbstar_dc_tmp_rv = 0.0;

        (locals.var_dvbstar_dc, locals.var_dvbstar_dc_dn5, locals.var_dvbstar_dc_dn6, locals.var_dvbstar_dc_dn7, locals.var_dvbstar_dc_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_dvbstar_dc_rv = 0.0;

        let assign40640_e53620: f64 = if ((p.p45 != 0.0) && (locals.var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign40640_e53620;
        locals.var_guard1172_rv = 0.0;

        if (locals.var_guard1172 != 0.0) {
            let assign40650_e53626: f64 = (locals.var_v_ds - locals.var_vdsx);
            let assign40650_e53627: f64 = (0.5 * assign40650_e53626);
            let assign40650_e53628: f64 = (locals.var_vsbstar_dc + assign40650_e53627);
            (locals.var_vmb, locals.var_vmb_dn5, locals.var_vmb_dn6, locals.var_vmb_dn7, locals.var_vmb_dn8, ) = (assign40650_e53628, locals.var_vsbstar_dc_dn5, (locals.var_vsbstar_dc_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vsbstar_dc_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vsbstar_dc_dn8, );
            locals.var_vmb_rv = 0.0;
        }

        if (locals.var_guard1172 != 0.0) {
            let assign40660_e53634: f64 = (locals.var_vmb + locals.var_phib_dc);
            let assign40660_e53635: f64 = (assign40660_e53634).sqrt();
            let assign40660_e53637: f64 = (assign40660_e53635 - locals.var_sqrt_phib_dc);
            (locals.var_us, locals.var_us_dn5, locals.var_us_dn6, locals.var_us_dn7, locals.var_us_dn8, ) = (assign40660_e53637, (locals.var_vmb_dn5 / (2.0 * assign40660_e53635)), (locals.var_vmb_dn6 / (2.0 * assign40660_e53635)), (locals.var_vmb_dn7 / (2.0 * assign40660_e53635)), (locals.var_vmb_dn8 / (2.0 * assign40660_e53635)), );
            locals.var_us_rv = 0.0;
        }

        if (locals.var_guard1172 != 0.0) {
            let assign40670_e53644: f64 = (locals.var_us - locals.var_us1);
            let assign40670_e53645: f64 = (2.0 * assign40670_e53644);
            let assign40670_e53647: f64 = (assign40670_e53645 / locals.var_us21);
            let assign40670_e53649: f64 = (assign40670_e53647 - 1.0);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign40670_e53649, ((2.0 * locals.var_us_dn5) / locals.var_us21), ((2.0 * locals.var_us_dn6) / locals.var_us21), ((2.0 * locals.var_us_dn7) / locals.var_us21), ((2.0 * locals.var_us_dn8) / locals.var_us21), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (locals.var_guard1172 != 0.0) {
            let assign40680_e53657: f64 = (1.0 - locals.var_gfacnud_i);
            let assign40680_e53658: f64 = (0.25 * assign40680_e53657);
            let assign40680_e53660: f64 = (assign40680_e53658 * locals.var_us21);
            let assign40680_e53664: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
            let assign40680_e53666: f64 = (assign40680_e53664 + 0.4804530139182);
            let assign40680_e53667: f64 = (assign40680_e53666).sqrt();
            let assign40680_e53668: f64 = (locals.var_temp__blk936 + assign40680_e53667);
            let assign40680_e53669: f64 = (assign40680_e53660 * assign40680_e53668);
            let assign40680_e53670: f64 = (locals.var_us - assign40680_e53669);
            (locals.var_usnew, locals.var_usnew_dn5, locals.var_usnew_dn6, locals.var_usnew_dn7, locals.var_usnew_dn8, ) = (assign40680_e53670, (locals.var_us_dn5 - (assign40680_e53660 * (locals.var_temp__blk936_dn5 + (((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) / (2.0 * assign40680_e53667))))), (locals.var_us_dn6 - (assign40680_e53660 * (locals.var_temp__blk936_dn6 + (((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) / (2.0 * assign40680_e53667))))), (locals.var_us_dn7 - (assign40680_e53660 * (locals.var_temp__blk936_dn7 + (((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) / (2.0 * assign40680_e53667))))), (locals.var_us_dn8 - (assign40680_e53660 * (locals.var_temp__blk936_dn8 + (((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) / (2.0 * assign40680_e53667))))), );
            locals.var_usnew_rv = 0.0;
        }

        if (locals.var_guard1172 != 0.0) {
            let assign40690_e53676: f64 = (locals.var_usnew * locals.var_usnew);
            let assign40690_e53679: f64 = (2.0 * locals.var_sqrt_phib_dc);
            let assign40690_e53681: f64 = (assign40690_e53679 * locals.var_usnew);
            let assign40690_e53682: f64 = (assign40690_e53676 + assign40690_e53681);
            (locals.var_vmbnew, locals.var_vmbnew_dn5, locals.var_vmbnew_dn6, locals.var_vmbnew_dn7, locals.var_vmbnew_dn8, ) = (assign40690_e53682, (((locals.var_usnew_dn5 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn5)) + (assign40690_e53679 * locals.var_usnew_dn5)), (((locals.var_usnew_dn6 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn6)) + (assign40690_e53679 * locals.var_usnew_dn6)), (((locals.var_usnew_dn7 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn7)) + (assign40690_e53679 * locals.var_usnew_dn7)), (((locals.var_usnew_dn8 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn8)) + (assign40690_e53679 * locals.var_usnew_dn8)), );
            locals.var_vmbnew_rv = 0.0;
        }

        if (locals.var_guard1172 != 0.0) {
            let assign40700_e53690: f64 = (locals.var_v_ds - locals.var_vdsx);
            let assign40700_e53691: f64 = (0.5 * assign40700_e53690);
            let assign40700_e53692: f64 = (locals.var_vmbnew - assign40700_e53691);
            (locals.var_vsbstar_dc, locals.var_vsbstar_dc_dn5, locals.var_vsbstar_dc_dn6, locals.var_vsbstar_dc_dn7, locals.var_vsbstar_dc_dn8, ) = (assign40700_e53692, locals.var_vmbnew_dn5, (locals.var_vmbnew_dn6 - (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vmbnew_dn7 - (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vmbnew_dn8, );
            locals.var_vsbstar_dc_rv = 0.0;
        }

        if (locals.var_guard1172 != 0.0) {
            let assign40710_e53698: f64 = (locals.var_vsbstar_dc_tmp - locals.var_vsbstar_dc);
            (locals.var_dvbstar_dc, locals.var_dvbstar_dc_dn5, locals.var_dvbstar_dc_dn6, locals.var_dvbstar_dc_dn7, locals.var_dvbstar_dc_dn8, ) = (assign40710_e53698, (locals.var_vsbstar_dc_tmp_dn5 - locals.var_vsbstar_dc_dn5), (locals.var_vsbstar_dc_tmp_dn6 - locals.var_vsbstar_dc_dn6), (locals.var_vsbstar_dc_tmp_dn7 - locals.var_vsbstar_dc_dn7), (locals.var_vsbstar_dc_tmp_dn8 - locals.var_vsbstar_dc_dn8), );
            locals.var_dvbstar_dc_rv = 0.0;
        }

        locals.var_phib = locals.var_phib_dc;
        locals.var_phib_rv = 0.0;

        locals.var_aphi = locals.var_aphi_dc;
        locals.var_aphi_rv = 0.0;

        locals.var_g_0 = locals.var_g_0_dc;
        locals.var_g_0_rv = 0.0;

        (locals.var_vsbstar, locals.var_vsbstar_dn5, locals.var_vsbstar_dn6, locals.var_vsbstar_dn7, locals.var_vsbstar_dn8, ) = (locals.var_vsbstar_dc, locals.var_vsbstar_dc_dn5, locals.var_vsbstar_dc_dn6, locals.var_vsbstar_dc_dn7, locals.var_vsbstar_dc_dn8, );
        locals.var_vsbstar_rv = 0.0;

        (locals.var_dvbstar, locals.var_dvbstar_dn5, locals.var_dvbstar_dn6, locals.var_dvbstar_dn7, locals.var_dvbstar_dn8, ) = (locals.var_dvbstar_dc, locals.var_dvbstar_dc_dn5, locals.var_dvbstar_dc_dn6, locals.var_dvbstar_dc_dn7, locals.var_dvbstar_dc_dn8, );
        locals.var_dvbstar_rv = 0.0;

        locals.var_thesatloc = locals.var_thesat_t;
        locals.var_thesatloc_rv = 0.0;

        locals.var_arloc = locals.var_ar;
        locals.var_arloc_rv = 0.0;

        let assign40790_e53710: f64 = (locals.var_vgb - locals.var_dvbstar);
        let assign40790_e53712: f64 = (assign40790_e53710 - locals.var_vfb_t);
        (locals.var_vgb1, locals.var_vgb1_dn5, locals.var_vgb1_dn6, locals.var_vgb1_dn7, locals.var_vgb1_dn8, ) = (assign40790_e53712, (locals.var_vgb_dn5 - locals.var_dvbstar_dn5), (locals.var_vgb_dn6 - locals.var_dvbstar_dn6), (locals.var_vgb_dn7 - locals.var_dvbstar_dn7), (locals.var_vgb_dn8 - locals.var_dvbstar_dn8), );
        locals.var_vgb1_rv = 0.0;

        let assign40800_e53717: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign40800_e53718: f64 = (0.5 * assign40800_e53717);
        let assign40800_e53719: f64 = (locals.var_vsbstar + assign40800_e53718);
        (locals.var_vsbx, locals.var_vsbx_dn5, locals.var_vsbx_dn6, locals.var_vsbx_dn7, locals.var_vsbx_dn8, ) = (assign40800_e53719, locals.var_vsbstar_dn5, (locals.var_vsbstar_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vsbstar_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vsbstar_dn8, );
        locals.var_vsbx_rv = 0.0;

        (locals.var_dctg, locals.var_dctg_dn5, locals.var_dctg_dn6, locals.var_dctg_dn7, locals.var_dctg_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_dctg_rv = 0.0;

        let assign40820_e53723: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign40820_e53723;
        locals.var_guard1173_rv = 0.0;

        if (locals.var_guard1173 != 0.0) {
            let assign40830_e53727: f64 = (locals.var_phib * locals.var_inv_phit);
            locals.var_xbct = assign40830_e53727;
            locals.var_xbct_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40840_e53733: f64 = (locals.var_vsbx * locals.var_inv_phit);
            (locals.var_xsbstar, locals.var_xsbstar_dn5, locals.var_xsbstar_dn6, locals.var_xsbstar_dn7, locals.var_xsbstar_dn8, ) = (assign40840_e53733, (locals.var_vsbx_dn5 * locals.var_inv_phit), (locals.var_vsbx_dn6 * locals.var_inv_phit), (locals.var_vsbx_dn7 * locals.var_inv_phit), (locals.var_vsbx_dn8 * locals.var_inv_phit), );
            locals.var_xsbstar_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40850_e53739: f64 = (locals.var_vgb1 * locals.var_inv_phit);
            (locals.var_xgct, locals.var_xgct_dn5, locals.var_xgct_dn6, locals.var_xgct_dn7, locals.var_xgct_dn8, ) = (assign40850_e53739, (locals.var_vgb1_dn5 * locals.var_inv_phit), (locals.var_vgb1_dn6 * locals.var_inv_phit), (locals.var_vgb1_dn7 * locals.var_inv_phit), (locals.var_vgb1_dn8 * locals.var_inv_phit), );
            locals.var_xgct_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40860_e53746: f64 = (0.5 * locals.var_g_0);
            let assign40860_e53748: f64 = (locals.var_xbct).sqrt();
            let assign40860_e53749: f64 = (assign40860_e53746 / assign40860_e53748);
            let assign40860_e53750: f64 = (1.0 + assign40860_e53749);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign40860_e53750, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40870_e53757: f64 = (locals.var_xbct).sqrt();
            let assign40870_e53758: f64 = (locals.var_g_0 * assign40870_e53757);
            let assign40870_e53759: f64 = (locals.var_xbct + assign40870_e53758);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign40870_e53759, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp2_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40880_e53765: f64 = (locals.var_xgct - locals.var_temp2);
            let assign40880_e53767: f64 = (assign40880_e53765 / locals.var_temp1);
            let assign40880_e53770: f64 = (0.5 * locals.var_xbct);
            let assign40880_e53771: f64 = (assign40880_e53767 + assign40880_e53770);
            let assign40880_e53774: f64 = (1.0 + locals.var_ctb_i);
            let assign40880_e53776: f64 = (assign40880_e53774 * locals.var_xsbstar);
            let assign40880_e53777: f64 = (assign40880_e53771 - assign40880_e53776);
            (locals.var_xwict, locals.var_xwict_dn5, locals.var_xwict_dn6, locals.var_xwict_dn7, locals.var_xwict_dn8, ) = (assign40880_e53777, (((((locals.var_xgct_dn5 - locals.var_temp2_dn5) * locals.var_temp1) - (assign40880_e53765 * locals.var_temp1_dn5)) / (locals.var_temp1 * locals.var_temp1)) - (assign40880_e53774 * locals.var_xsbstar_dn5)), (((((locals.var_xgct_dn6 - locals.var_temp2_dn6) * locals.var_temp1) - (assign40880_e53765 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) - (assign40880_e53774 * locals.var_xsbstar_dn6)), (((((locals.var_xgct_dn7 - locals.var_temp2_dn7) * locals.var_temp1) - (assign40880_e53765 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) - (assign40880_e53774 * locals.var_xsbstar_dn7)), (((((locals.var_xgct_dn8 - locals.var_temp2_dn8) * locals.var_temp1) - (assign40880_e53765 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) - (assign40880_e53774 * locals.var_xsbstar_dn8)), );
            locals.var_xwict_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40890_e53783: f64 = (0.5 * locals.var_xbct);
            let assign40890_e53785: f64 = (assign40890_e53783 + 2.0);
            locals.var_xctmax = assign40890_e53785;
            locals.var_xctmax_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40900_e53791: f64 = (locals.var_xbct + locals.var_xsbstar);
            (locals.var_xnct, locals.var_xnct_dn5, locals.var_xnct_dn6, locals.var_xnct_dn7, locals.var_xnct_dn8, ) = (assign40900_e53791, locals.var_xsbstar_dn5, locals.var_xsbstar_dn6, locals.var_xsbstar_dn7, locals.var_xsbstar_dn8, );
            locals.var_xnct_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40910_e53797: f64 = (locals.var_xgct - locals.var_xnct);
            let assign40910_e53800: f64 = (locals.var_xnct).sqrt();
            let assign40910_e53801: f64 = (locals.var_g_0 * assign40910_e53800);
            let assign40910_e53802: f64 = (assign40910_e53797 - assign40910_e53801);
            let assign40910_e53806: f64 = (locals.var_xbct / locals.var_g_0);
            let assign40910_e53808: f64 = (locals.var_xbct).sqrt();
            let assign40910_e53809: f64 = (assign40910_e53806 + assign40910_e53808);
            let assign40910_e53810: f64 = (assign40910_e53809).ln();
            let assign40910_e53811: f64 = (2.0 * assign40910_e53810);
            let assign40910_e53812: f64 = (assign40910_e53802 - assign40910_e53811);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign40910_e53812, ((locals.var_xgct_dn5 - locals.var_xnct_dn5) - (locals.var_g_0 * (locals.var_xnct_dn5 / (2.0 * assign40910_e53800)))), ((locals.var_xgct_dn6 - locals.var_xnct_dn6) - (locals.var_g_0 * (locals.var_xnct_dn6 / (2.0 * assign40910_e53800)))), ((locals.var_xgct_dn7 - locals.var_xnct_dn7) - (locals.var_g_0 * (locals.var_xnct_dn7 / (2.0 * assign40910_e53800)))), ((locals.var_xgct_dn8 - locals.var_xnct_dn8) - (locals.var_g_0 * (locals.var_xnct_dn8 / (2.0 * assign40910_e53800)))), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40920_e53818: f64 = (2.0 * locals.var_temp1);
            let assign40920_e53820: f64 = (assign40920_e53818 + locals.var_xctmax);
            (locals.var_xmict, locals.var_xmict_dn5, locals.var_xmict_dn6, locals.var_xmict_dn7, locals.var_xmict_dn8, ) = (assign40920_e53820, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), );
            locals.var_xmict_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40930_e53827: f64 = (locals.var_xwict + locals.var_xmict);
            let assign40930_e53830: f64 = (locals.var_xwict - locals.var_xmict);
            let assign40930_e53833: f64 = (locals.var_xwict - locals.var_xmict);
            let assign40930_e53834: f64 = (assign40930_e53830 * assign40930_e53833);
            let assign40930_e53836: f64 = (assign40930_e53834 + 20.0);
            let assign40930_e53837: f64 = (assign40930_e53836).sqrt();
            let assign40930_e53838: f64 = (assign40930_e53827 + assign40930_e53837);
            let assign40930_e53839: f64 = (0.5 * assign40930_e53838);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign40930_e53839, (0.5 * ((locals.var_xwict_dn5 + locals.var_xmict_dn5) + ((((locals.var_xwict_dn5 - locals.var_xmict_dn5) * assign40930_e53833) + (assign40930_e53830 * (locals.var_xwict_dn5 - locals.var_xmict_dn5))) / (2.0 * assign40930_e53837)))), (0.5 * ((locals.var_xwict_dn6 + locals.var_xmict_dn6) + ((((locals.var_xwict_dn6 - locals.var_xmict_dn6) * assign40930_e53833) + (assign40930_e53830 * (locals.var_xwict_dn6 - locals.var_xmict_dn6))) / (2.0 * assign40930_e53837)))), (0.5 * ((locals.var_xwict_dn7 + locals.var_xmict_dn7) + ((((locals.var_xwict_dn7 - locals.var_xmict_dn7) * assign40930_e53833) + (assign40930_e53830 * (locals.var_xwict_dn7 - locals.var_xmict_dn7))) / (2.0 * assign40930_e53837)))), (0.5 * ((locals.var_xwict_dn8 + locals.var_xmict_dn8) + ((((locals.var_xwict_dn8 - locals.var_xmict_dn8) * assign40930_e53833) + (assign40930_e53830 * (locals.var_xwict_dn8 - locals.var_xmict_dn8))) / (2.0 * assign40930_e53837)))), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40940_e53846: f64 = (locals.var_xgct - locals.var_xsbstar);
            let assign40940_e53847: f64 = (2.0 * assign40940_e53846);
            let assign40940_e53849: f64 = (assign40940_e53847 - locals.var_xctmax);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign40940_e53849, (2.0 * (locals.var_xgct_dn5 - locals.var_xsbstar_dn5)), (2.0 * (locals.var_xgct_dn6 - locals.var_xsbstar_dn6)), (2.0 * (locals.var_xgct_dn7 - locals.var_xsbstar_dn7)), (2.0 * (locals.var_xgct_dn8 - locals.var_xsbstar_dn8)), );
            locals.var_temp2_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40950_e53856: f64 = (locals.var_temp1 + locals.var_temp2);
            let assign40950_e53859: f64 = (locals.var_temp1 - locals.var_temp2);
            let assign40950_e53862: f64 = (locals.var_temp1 - locals.var_temp2);
            let assign40950_e53863: f64 = (assign40950_e53859 * assign40950_e53862);
            let assign40950_e53865: f64 = (assign40950_e53863 + 20.0);
            let assign40950_e53866: f64 = (assign40950_e53865).sqrt();
            let assign40950_e53867: f64 = (assign40950_e53856 - assign40950_e53866);
            let assign40950_e53868: f64 = (0.5 * assign40950_e53867);
            (locals.var_xsubct, locals.var_xsubct_dn5, locals.var_xsubct_dn6, locals.var_xsubct_dn7, locals.var_xsubct_dn8, ) = (assign40950_e53868, (0.5 * ((locals.var_temp1_dn5 + locals.var_temp2_dn5) - ((((locals.var_temp1_dn5 - locals.var_temp2_dn5) * assign40950_e53862) + (assign40950_e53859 * (locals.var_temp1_dn5 - locals.var_temp2_dn5))) / (2.0 * assign40950_e53866)))), (0.5 * ((locals.var_temp1_dn6 + locals.var_temp2_dn6) - ((((locals.var_temp1_dn6 - locals.var_temp2_dn6) * assign40950_e53862) + (assign40950_e53859 * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) / (2.0 * assign40950_e53866)))), (0.5 * ((locals.var_temp1_dn7 + locals.var_temp2_dn7) - ((((locals.var_temp1_dn7 - locals.var_temp2_dn7) * assign40950_e53862) + (assign40950_e53859 * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) / (2.0 * assign40950_e53866)))), (0.5 * ((locals.var_temp1_dn8 + locals.var_temp2_dn8) - ((((locals.var_temp1_dn8 - locals.var_temp2_dn8) * assign40950_e53862) + (assign40950_e53859 * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) / (2.0 * assign40950_e53866)))), );
            locals.var_xsubct_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40960_e53875: f64 = (locals.var_xsubct + locals.var_xctmax);
            let assign40960_e53878: f64 = (locals.var_xsubct - locals.var_xctmax);
            let assign40960_e53881: f64 = (locals.var_xsubct - locals.var_xctmax);
            let assign40960_e53882: f64 = (assign40960_e53878 * assign40960_e53881);
            let assign40960_e53884: f64 = (assign40960_e53882 + 5.0);
            let assign40960_e53885: f64 = (assign40960_e53884).sqrt();
            let assign40960_e53886: f64 = (assign40960_e53875 - assign40960_e53885);
            let assign40960_e53887: f64 = (0.5 * assign40960_e53886);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign40960_e53887, (0.5 * (locals.var_xsubct_dn5 - (((locals.var_xsubct_dn5 * assign40960_e53881) + (assign40960_e53878 * locals.var_xsubct_dn5)) / (2.0 * assign40960_e53885)))), (0.5 * (locals.var_xsubct_dn6 - (((locals.var_xsubct_dn6 * assign40960_e53881) + (assign40960_e53878 * locals.var_xsubct_dn6)) / (2.0 * assign40960_e53885)))), (0.5 * (locals.var_xsubct_dn7 - (((locals.var_xsubct_dn7 * assign40960_e53881) + (assign40960_e53878 * locals.var_xsubct_dn7)) / (2.0 * assign40960_e53885)))), (0.5 * (locals.var_xsubct_dn8 - (((locals.var_xsubct_dn8 * assign40960_e53881) + (assign40960_e53878 * locals.var_xsubct_dn8)) / (2.0 * assign40960_e53885)))), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40970_e53894: f64 = (-locals.var_xctmax);
            let assign40970_e53895: f64 = (locals.var_temp1 + assign40970_e53894);
            let assign40970_e53898: f64 = (-locals.var_xctmax);
            let assign40970_e53899: f64 = (locals.var_temp1 - assign40970_e53898);
            let assign40970_e53902: f64 = (-locals.var_xctmax);
            let assign40970_e53903: f64 = (locals.var_temp1 - assign40970_e53902);
            let assign40970_e53904: f64 = (assign40970_e53899 * assign40970_e53903);
            let assign40970_e53906: f64 = (assign40970_e53904 + 20.0);
            let assign40970_e53907: f64 = (assign40970_e53906).sqrt();
            let assign40970_e53908: f64 = (assign40970_e53895 + assign40970_e53907);
            let assign40970_e53909: f64 = (0.5 * assign40970_e53908);
            (locals.var_xct, locals.var_xct_dn5, locals.var_xct_dn6, locals.var_xct_dn7, locals.var_xct_dn8, ) = (assign40970_e53909, (0.5 * (locals.var_temp1_dn5 + (((locals.var_temp1_dn5 * assign40970_e53903) + (assign40970_e53899 * locals.var_temp1_dn5)) / (2.0 * assign40970_e53907)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign40970_e53903) + (assign40970_e53899 * locals.var_temp1_dn6)) / (2.0 * assign40970_e53907)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign40970_e53903) + (assign40970_e53899 * locals.var_temp1_dn7)) / (2.0 * assign40970_e53907)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign40970_e53903) + (assign40970_e53899 * locals.var_temp1_dn8)) / (2.0 * assign40970_e53907)))), );
            locals.var_xct_rv = 0.0;
        }

        if (locals.var_guard1173 != 0.0) {
            let assign40980_e53916: f64 = (locals.var_xct / locals.var_xctmax);
            let assign40980_e53918: f64 = (assign40980_e53916 + 1.0);
            let assign40980_e53919: f64 = (locals.var_ctg_t * assign40980_e53918);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign40980_e53919, (locals.var_ctg_t * (locals.var_xct_dn5 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn6 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn7 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn8 / locals.var_xctmax)), );
            locals.var_temp2_rv = 0.0;
        }

        let assign40990_e53924: f64 = (-230.25850929940458);
        let assign40990_e53925: f64 = if locals.var_temp2 > assign40990_e53924 { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign40990_e53925;
        locals.var_guard1174_rv = 0.0;

        if ((locals.var_guard1173 != 0.0) && (locals.var_guard1174 != 0.0)) {
            let assign41000_e53930: f64 = (locals.var_temp2).exp();
            (locals.var_dctg, locals.var_dctg_dn5, locals.var_dctg_dn6, locals.var_dctg_dn7, locals.var_dctg_dn8, ) = (assign41000_e53930, (assign41000_e53930 * locals.var_temp2_dn5), (assign41000_e53930 * locals.var_temp2_dn6), (assign41000_e53930 * locals.var_temp2_dn7), (assign41000_e53930 * locals.var_temp2_dn8), );
            locals.var_dctg_rv = 0.0;
        }

        if ((locals.var_guard1173 != 0.0) && (locals.var_guard1174 == 0.0)) {
            let assign41010_e53940: f64 = (-230.25850929940458);
            let assign41010_e53942: f64 = (assign41010_e53940 - locals.var_temp2);
            let assign41010_e53946: f64 = (-230.25850929940458);
            let assign41010_e53948: f64 = (assign41010_e53946 - locals.var_temp2);
            let assign41010_e53951: f64 = (-230.25850929940458);
            let assign41010_e53953: f64 = (assign41010_e53951 - locals.var_temp2);
            let assign41010_e53955: f64 = (assign41010_e53953 * 0.3333333333333333);
            let assign41010_e53956: f64 = (1.0 + assign41010_e53955);
            let assign41010_e53957: f64 = (assign41010_e53948 * assign41010_e53956);
            let assign41010_e53958: f64 = (0.5 * assign41010_e53957);
            let assign41010_e53959: f64 = (1.0 + assign41010_e53958);
            let assign41010_e53960: f64 = (assign41010_e53942 * assign41010_e53959);
            let assign41010_e53961: f64 = (1.0 + assign41010_e53960);
            let assign41010_e53962: f64 = (1e-100 / assign41010_e53961);
            (locals.var_dctg, locals.var_dctg_dn5, locals.var_dctg_dn6, locals.var_dctg_dn7, locals.var_dctg_dn8, ) = (assign41010_e53962, (-((1e-100 * (((-locals.var_temp2_dn5) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-locals.var_temp2_dn5) * assign41010_e53956) + (assign41010_e53948 * ((-locals.var_temp2_dn5) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-locals.var_temp2_dn6) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-locals.var_temp2_dn6) * assign41010_e53956) + (assign41010_e53948 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-locals.var_temp2_dn7) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-locals.var_temp2_dn7) * assign41010_e53956) + (assign41010_e53948 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-locals.var_temp2_dn8) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-locals.var_temp2_dn8) * assign41010_e53956) + (assign41010_e53948 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), );
            locals.var_dctg_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign41020_e53968: f64 = (locals.var_ct_t * locals.var_dctg);
        let assign41020_e53969: f64 = (1.0 + assign41020_e53968);
        (locals.var_ct_fact, locals.var_ct_fact_dn5, locals.var_ct_fact_dn6, locals.var_ct_fact_dn7, locals.var_ct_fact_dn8, ) = (assign41020_e53969, (locals.var_ct_t * locals.var_dctg_dn5), (locals.var_ct_t * locals.var_dctg_dn6), (locals.var_ct_t * locals.var_dctg_dn7), (locals.var_ct_t * locals.var_dctg_dn8), );
        locals.var_ct_fact_rv = 0.0;

        let assign41030_e53972: f64 = (locals.var_phit * locals.var_ct_fact);
        (locals.var_phitct, locals.var_phitct_dn5, locals.var_phitct_dn6, locals.var_phitct_dn7, locals.var_phitct_dn8, ) = (assign41030_e53972, (locals.var_phit * locals.var_ct_fact_dn5), (locals.var_phit * locals.var_ct_fact_dn6), (locals.var_phit * locals.var_ct_fact_dn7), (locals.var_phit * locals.var_ct_fact_dn8), );
        locals.var_phitct_rv = 0.0;

        let assign41040_e53977: f64 = (locals.var_psced_i * locals.var_vdsx);
        let assign41040_e53978: f64 = (1.0 + assign41040_e53977);
        let assign41040_e53979: f64 = (locals.var_psce_i * assign41040_e53978);
        let assign41040_e53983: f64 = (locals.var_psceb_i * locals.var_vsbx);
        let assign41040_e53984: f64 = (1.0 + assign41040_e53983);
        let assign41040_e53985: f64 = (assign41040_e53979 * assign41040_e53984);
        (locals.var_dphit1, locals.var_dphit1_dn5, locals.var_dphit1_dn6, locals.var_dphit1_dn7, locals.var_dphit1_dn8, ) = (assign41040_e53985, (assign41040_e53979 * (locals.var_psceb_i * locals.var_vsbx_dn5)), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn6)) * assign41040_e53984) + (assign41040_e53979 * (locals.var_psceb_i * locals.var_vsbx_dn6))), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn7)) * assign41040_e53984) + (assign41040_e53979 * (locals.var_psceb_i * locals.var_vsbx_dn7))), (assign41040_e53979 * (locals.var_psceb_i * locals.var_vsbx_dn8)), );
        locals.var_dphit1_rv = 0.0;

        let assign41050_e53989: f64 = (1.0 + locals.var_dphit1);
        let assign41050_e53990: f64 = (locals.var_phitct * assign41050_e53989);
        (locals.var_phit1, locals.var_phit1_dn5, locals.var_phit1_dn6, locals.var_phit1_dn7, locals.var_phit1_dn8, ) = (assign41050_e53990, ((locals.var_phitct_dn5 * assign41050_e53989) + (locals.var_phitct * locals.var_dphit1_dn5)), ((locals.var_phitct_dn6 * assign41050_e53989) + (locals.var_phitct * locals.var_dphit1_dn6)), ((locals.var_phitct_dn7 * assign41050_e53989) + (locals.var_phitct * locals.var_dphit1_dn7)), ((locals.var_phitct_dn8 * assign41050_e53989) + (locals.var_phitct * locals.var_dphit1_dn8)), );
        locals.var_phit1_rv = 0.0;

        let assign41060_e53993: f64 = (1.0 / locals.var_phit1);
        (locals.var_inv_phit1, locals.var_inv_phit1_dn5, locals.var_inv_phit1_dn6, locals.var_inv_phit1_dn7, locals.var_inv_phit1_dn8, ) = (assign41060_e53993, (-(locals.var_phit1_dn5 / (locals.var_phit1 * locals.var_phit1))), (-(locals.var_phit1_dn6 / (locals.var_phit1 * locals.var_phit1))), (-(locals.var_phit1_dn7 / (locals.var_phit1 * locals.var_phit1))), (-(locals.var_phit1_dn8 / (locals.var_phit1 * locals.var_phit1))), );
        locals.var_inv_phit1_rv = 0.0;

        let assign41070_e53997: f64 = (locals.var_phit * locals.var_inv_phit1);
        let assign41070_e53998: f64 = (assign41070_e53997).sqrt();
        let assign41070_e53999: f64 = (locals.var_g_0 * assign41070_e53998);
        (locals.var_gf, locals.var_gf_dn5, locals.var_gf_dn6, locals.var_gf_dn7, locals.var_gf_dn8, ) = (assign41070_e53999, (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn5) / (2.0 * assign41070_e53998))), (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn6) / (2.0 * assign41070_e53998))), (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn7) / (2.0 * assign41070_e53998))), (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn8) / (2.0 * assign41070_e53998))), );
        locals.var_gf_rv = 0.0;

        let assign41080_e54002: f64 = (locals.var_gf * locals.var_gf);
        (locals.var_gf2, locals.var_gf2_dn5, locals.var_gf2_dn6, locals.var_gf2_dn7, locals.var_gf2_dn8, ) = (assign41080_e54002, ((locals.var_gf_dn5 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn5)), ((locals.var_gf_dn6 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn6)), ((locals.var_gf_dn7 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn7)), ((locals.var_gf_dn8 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn8)), );
        locals.var_gf2_rv = 0.0;

        let assign41090_e54005: f64 = (1.0 / locals.var_gf2);
        (locals.var_inv_gf2, locals.var_inv_gf2_dn5, locals.var_inv_gf2_dn6, locals.var_inv_gf2_dn7, locals.var_inv_gf2_dn8, ) = (assign41090_e54005, (-(locals.var_gf2_dn5 / (locals.var_gf2 * locals.var_gf2))), (-(locals.var_gf2_dn6 / (locals.var_gf2 * locals.var_gf2))), (-(locals.var_gf2_dn7 / (locals.var_gf2 * locals.var_gf2))), (-(locals.var_gf2_dn8 / (locals.var_gf2 * locals.var_gf2))), );
        locals.var_inv_gf2_rv = 0.0;

        let assign41100_e54008: f64 = (locals.var_vsbstar * locals.var_inv_phit1);
        (locals.var_ux, locals.var_ux_dn5, locals.var_ux_dn6, locals.var_ux_dn7, locals.var_ux_dn8, ) = (assign41100_e54008, ((locals.var_vsbstar_dn5 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn5)), ((locals.var_vsbstar_dn6 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn6)), ((locals.var_vsbstar_dn7 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn7)), ((locals.var_vsbstar_dn8 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn8)), );
        locals.var_ux_rv = 0.0;

        let assign41110_e54011: f64 = (locals.var_vgb1 * locals.var_inv_phit1);
        (locals.var_xg, locals.var_xg_dn5, locals.var_xg_dn6, locals.var_xg_dn7, locals.var_xg_dn8, ) = (assign41110_e54011, ((locals.var_vgb1_dn5 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn5)), ((locals.var_vgb1_dn6 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn6)), ((locals.var_vgb1_dn7 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn7)), ((locals.var_vgb1_dn8 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn8)), );
        locals.var_xg_rv = 0.0;

        let assign41120_e54014: f64 = (2.0 * locals.var_vdsx);
        let assign41120_e54019: f64 = (locals.var_cfd_i * locals.var_vdsx);
        let assign41120_e54020: f64 = (1.0 + assign41120_e54019);
        let assign41120_e54021: f64 = (assign41120_e54020).sqrt();
        let assign41120_e54022: f64 = (1.0 + assign41120_e54021);
        let assign41120_e54023: f64 = (assign41120_e54014 / assign41120_e54022);
        (locals.var_vdsp, locals.var_vdsp_dn6, locals.var_vdsp_dn7, ) = (assign41120_e54023, ((((2.0 * locals.var_vdsx_dn6) * assign41120_e54022) - (assign41120_e54014 * ((locals.var_cfd_i * locals.var_vdsx_dn6) / (2.0 * assign41120_e54021)))) / (assign41120_e54022 * assign41120_e54022)), ((((2.0 * locals.var_vdsx_dn7) * assign41120_e54022) - (assign41120_e54014 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign41120_e54021)))) / (assign41120_e54022 * assign41120_e54022)), );
        locals.var_vdsp_rv = 0.0;

        let assign41130_e54026: f64 = (locals.var_cf_i * locals.var_vdsp);
        let assign41130_e54030: f64 = (locals.var_cfb_i * locals.var_vsbx);
        let assign41130_e54031: f64 = (1.0 + assign41130_e54030);
        let assign41130_e54032: f64 = (assign41130_e54026 * assign41130_e54031);
        (locals.var_delphib, locals.var_delphib_dn5, locals.var_delphib_dn6, locals.var_delphib_dn7, locals.var_delphib_dn8, ) = (assign41130_e54032, (assign41130_e54026 * (locals.var_cfb_i * locals.var_vsbx_dn5)), (((locals.var_cf_i * locals.var_vdsp_dn6) * assign41130_e54031) + (assign41130_e54026 * (locals.var_cfb_i * locals.var_vsbx_dn6))), (((locals.var_cf_i * locals.var_vdsp_dn7) * assign41130_e54031) + (assign41130_e54026 * (locals.var_cfb_i * locals.var_vsbx_dn7))), (assign41130_e54026 * (locals.var_cfb_i * locals.var_vsbx_dn8)), );
        locals.var_delphib_rv = 0.0;

        let assign41140_e54035: f64 = (locals.var_phib * locals.var_inv_phit1);
        (locals.var_xb, locals.var_xb_dn5, locals.var_xb_dn6, locals.var_xb_dn7, locals.var_xb_dn8, ) = (assign41140_e54035, (locals.var_phib * locals.var_inv_phit1_dn5), (locals.var_phib * locals.var_inv_phit1_dn6), (locals.var_phib * locals.var_inv_phit1_dn7), (locals.var_phib * locals.var_inv_phit1_dn8), );
        locals.var_xb_rv = 0.0;

        let assign41150_e54038: f64 = (locals.var_v_xb * locals.var_v_xb);
        let assign41150_e54040: f64 = (assign41150_e54038 + locals.var_aphi);
        let assign41150_e54041: f64 = (assign41150_e54040).sqrt();
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign41150_e54041, 0.0, (((locals.var_v_xb_dn6 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn6)) / (2.0 * assign41150_e54041)), (((locals.var_v_xb_dn7 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn7)) / (2.0 * assign41150_e54041)), (((locals.var_v_xb_dn8 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn8)) / (2.0 * assign41150_e54041)), );
        locals.var_temp1_rv = 0.0;

        let assign41160_e54044: f64 = (locals.var_v_xb - locals.var_delphib);
        let assign41160_e54047: f64 = (locals.var_v_xb - locals.var_delphib);
        let assign41160_e54048: f64 = (assign41160_e54044 * assign41160_e54047);
        let assign41160_e54050: f64 = (assign41160_e54048 + locals.var_aphi);
        let assign41160_e54051: f64 = (assign41160_e54050).sqrt();
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign41160_e54051, ((((-locals.var_delphib_dn5) * assign41160_e54047) + (assign41160_e54044 * (-locals.var_delphib_dn5))) / (2.0 * assign41160_e54051)), ((((locals.var_v_xb_dn6 - locals.var_delphib_dn6) * assign41160_e54047) + (assign41160_e54044 * (locals.var_v_xb_dn6 - locals.var_delphib_dn6))) / (2.0 * assign41160_e54051)), ((((locals.var_v_xb_dn7 - locals.var_delphib_dn7) * assign41160_e54047) + (assign41160_e54044 * (locals.var_v_xb_dn7 - locals.var_delphib_dn7))) / (2.0 * assign41160_e54051)), ((((locals.var_v_xb_dn8 - locals.var_delphib_dn8) * assign41160_e54047) + (assign41160_e54044 * (locals.var_v_xb_dn8 - locals.var_delphib_dn8))) / (2.0 * assign41160_e54051)), );
        locals.var_temp2_rv = 0.0;

        let assign41170_e54054: f64 = (0.5 * locals.var_inv_phit1);
        let assign41170_e54057: f64 = (locals.var_delphib + locals.var_temp1);
        let assign41170_e54059: f64 = (assign41170_e54057 - locals.var_temp2);
        let assign41170_e54060: f64 = (assign41170_e54054 * assign41170_e54059);
        (locals.var_delxb, locals.var_delxb_dn5, locals.var_delxb_dn6, locals.var_delxb_dn7, locals.var_delxb_dn8, ) = (assign41170_e54060, (((0.5 * locals.var_inv_phit1_dn5) * assign41170_e54059) + (assign41170_e54054 * ((locals.var_delphib_dn5 + locals.var_temp1_dn5) - locals.var_temp2_dn5))), (((0.5 * locals.var_inv_phit1_dn6) * assign41170_e54059) + (assign41170_e54054 * ((locals.var_delphib_dn6 + locals.var_temp1_dn6) - locals.var_temp2_dn6))), (((0.5 * locals.var_inv_phit1_dn7) * assign41170_e54059) + (assign41170_e54054 * ((locals.var_delphib_dn7 + locals.var_temp1_dn7) - locals.var_temp2_dn7))), (((0.5 * locals.var_inv_phit1_dn8) * assign41170_e54059) + (assign41170_e54054 * ((locals.var_delphib_dn8 + locals.var_temp1_dn8) - locals.var_temp2_dn8))), );
        locals.var_delxb_rv = 0.0;

        let assign41180_e54063: f64 = (locals.var_xb + locals.var_ux);
        (locals.var_xno_s, locals.var_xno_s_dn5, locals.var_xno_s_dn6, locals.var_xno_s_dn7, locals.var_xno_s_dn8, ) = (assign41180_e54063, (locals.var_xb_dn5 + locals.var_ux_dn5), (locals.var_xb_dn6 + locals.var_ux_dn6), (locals.var_xb_dn7 + locals.var_ux_dn7), (locals.var_xb_dn8 + locals.var_ux_dn8), );
        locals.var_xno_s_rv = 0.0;

        let assign41190_e54066: f64 = (locals.var_xno_s - locals.var_delxb);
        (locals.var_xn_s, locals.var_xn_s_dn5, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8, ) = (assign41190_e54066, (locals.var_xno_s_dn5 - locals.var_delxb_dn5), (locals.var_xno_s_dn6 - locals.var_delxb_dn6), (locals.var_xno_s_dn7 - locals.var_delxb_dn7), (locals.var_xno_s_dn8 - locals.var_delxb_dn8), );
        locals.var_xn_s_rv = 0.0;

        let assign41200_e54069: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1175 = assign41200_e54069;
        locals.var_guard1175_rv = 0.0;

        let assign41210_e54071: f64 = (locals.var_xn_s).abs();
        let assign41210_e54073: f64 = if assign41210_e54071 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1176 = assign41210_e54073;
        locals.var_guard1176_rv = 0.0;

        if ((locals.var_guard1175 != 0.0) && (locals.var_guard1176 != 0.0)) {
            let assign41220_e54082: f64 = (0.5 * locals.var_xn_s);
            let assign41220_e54086: f64 = (0.3125 * locals.var_xn_s);
            let assign41220_e54087: f64 = (1.0 - assign41220_e54086);
            let assign41220_e54088: f64 = (assign41220_e54082 * assign41220_e54087);
            let assign41220_e54089: f64 = (1.0 - assign41220_e54088);
            let assign41220_e54090: f64 = (locals.var_gf * assign41220_e54089);
            let assign41220_e54091: f64 = (1.0 + assign41220_e54090);
            (locals.var_nscr, locals.var_nscr_dn5, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, ) = (assign41220_e54091, ((locals.var_gf_dn5 * assign41220_e54089) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn5) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * locals.var_xn_s_dn5))))))), ((locals.var_gf_dn6 * assign41220_e54089) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn6) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * locals.var_xn_s_dn6))))))), ((locals.var_gf_dn7 * assign41220_e54089) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn7) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * locals.var_xn_s_dn7))))))), ((locals.var_gf_dn8 * assign41220_e54089) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn8) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * locals.var_xn_s_dn8))))))), );
            locals.var_nscr_rv = 0.0;
        }

        let assign41230_e54096: f64 = if locals.var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign41230_e54096;
        locals.var_guard1177_rv = 0.0;

        if (((locals.var_guard1175 != 0.0) && (locals.var_guard1176 == 0.0)) && (locals.var_guard1177 != 0.0)) {
            let assign41240_e54104: f64 = (-locals.var_xn_s);
            let assign41240_e54105: f64 = (assign41240_e54104).exp();
            (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, ) = (assign41240_e54105, (assign41240_e54105 * (-locals.var_xn_s_dn5)), (assign41240_e54105 * (-locals.var_xn_s_dn6)), (assign41240_e54105 * (-locals.var_xn_s_dn7)), (assign41240_e54105 * (-locals.var_xn_s_dn8)), );
            locals.var_delta_ns_rv = 0.0;
        }

        if (((locals.var_guard1175 != 0.0) && (locals.var_guard1176 == 0.0)) && (locals.var_guard1177 == 0.0)) {
            let assign41250_e54119: f64 = (locals.var_xn_s - 460.51701859880916);
            let assign41250_e54124: f64 = (locals.var_xn_s - 460.51701859880916);
            let assign41250_e54128: f64 = (locals.var_xn_s - 460.51701859880916);
            let assign41250_e54130: f64 = (assign41250_e54128 * 0.3333333333333333);
            let assign41250_e54131: f64 = (1.0 + assign41250_e54130);
            let assign41250_e54132: f64 = (assign41250_e54124 * assign41250_e54131);
            let assign41250_e54133: f64 = (0.5 * assign41250_e54132);
            let assign41250_e54134: f64 = (1.0 + assign41250_e54133);
            let assign41250_e54135: f64 = (assign41250_e54119 * assign41250_e54134);
            let assign41250_e54136: f64 = (1.0 + assign41250_e54135);
            let assign41250_e54137: f64 = (1e-200 / assign41250_e54136);
            (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, ) = (assign41250_e54137, (-((1e-200 * ((locals.var_xn_s_dn5 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((locals.var_xn_s_dn5 * assign41250_e54131) + (assign41250_e54124 * (locals.var_xn_s_dn5 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((locals.var_xn_s_dn6 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((locals.var_xn_s_dn6 * assign41250_e54131) + (assign41250_e54124 * (locals.var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((locals.var_xn_s_dn7 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((locals.var_xn_s_dn7 * assign41250_e54131) + (assign41250_e54124 * (locals.var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((locals.var_xn_s_dn8 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((locals.var_xn_s_dn8 * assign41250_e54131) + (assign41250_e54124 * (locals.var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), );
            locals.var_delta_ns_rv = 0.0;
        }

        if ((locals.var_guard1175 != 0.0) && (locals.var_guard1176 == 0.0)) {
            let (assign41260_e54150,) = {
    if (locals.var_xn_s > 0.0) {
        (1.0,)
    } else {
        let assign41260_e54149: f64 = (-1.0);
        (assign41260_e54149,)
    }
};
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign41260_e54150, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp__blk936_rv = 0.0;
        }

        if ((locals.var_guard1175 != 0.0) && (locals.var_guard1176 == 0.0)) {
            let assign41270_e54160: f64 = (locals.var_temp__blk936 * locals.var_gf);
            let assign41270_e54165: f64 = (1.0 - locals.var_xn_s);
            let assign41270_e54166: f64 = (locals.var_delta_ns * assign41270_e54165);
            let assign41270_e54167: f64 = (1.0 - assign41270_e54166);
            let assign41270_e54168: f64 = (assign41270_e54160 * assign41270_e54167);
            let assign41270_e54173: f64 = (1.0 - locals.var_delta_ns);
            let assign41270_e54174: f64 = (locals.var_xn_s * assign41270_e54173);
            let assign41270_e54175: f64 = (assign41270_e54174).sqrt();
            let assign41270_e54176: f64 = (2.0 * assign41270_e54175);
            let assign41270_e54177: f64 = (assign41270_e54168 / assign41270_e54176);
            let assign41270_e54178: f64 = (1.0 + assign41270_e54177);
            (locals.var_nscr, locals.var_nscr_dn5, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, ) = (assign41270_e54178, (((((((locals.var_temp__blk936_dn5 * locals.var_gf) + (locals.var_temp__blk936 * locals.var_gf_dn5)) * assign41270_e54167) + (assign41270_e54160 * (-((locals.var_delta_ns_dn5 * assign41270_e54165) + (locals.var_delta_ns * (-locals.var_xn_s_dn5)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((locals.var_xn_s_dn5 * assign41270_e54173) + (locals.var_xn_s * (-locals.var_delta_ns_dn5))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((locals.var_temp__blk936_dn6 * locals.var_gf) + (locals.var_temp__blk936 * locals.var_gf_dn6)) * assign41270_e54167) + (assign41270_e54160 * (-((locals.var_delta_ns_dn6 * assign41270_e54165) + (locals.var_delta_ns * (-locals.var_xn_s_dn6)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((locals.var_xn_s_dn6 * assign41270_e54173) + (locals.var_xn_s * (-locals.var_delta_ns_dn6))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((locals.var_temp__blk936_dn7 * locals.var_gf) + (locals.var_temp__blk936 * locals.var_gf_dn7)) * assign41270_e54167) + (assign41270_e54160 * (-((locals.var_delta_ns_dn7 * assign41270_e54165) + (locals.var_delta_ns * (-locals.var_xn_s_dn7)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((locals.var_xn_s_dn7 * assign41270_e54173) + (locals.var_xn_s * (-locals.var_delta_ns_dn7))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((locals.var_temp__blk936_dn8 * locals.var_gf) + (locals.var_temp__blk936 * locals.var_gf_dn8)) * assign41270_e54167) + (assign41270_e54160 * (-((locals.var_delta_ns_dn8 * assign41270_e54165) + (locals.var_delta_ns * (-locals.var_xn_s_dn8)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((locals.var_xn_s_dn8 * assign41270_e54173) + (locals.var_xn_s * (-locals.var_delta_ns_dn8))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), );
            locals.var_nscr_rv = 0.0;
        }

        if (locals.var_guard1175 == 0.0) {
            let assign41280_e54186: f64 = (0.5 * locals.var_gf);
            let assign41280_e54188: f64 = (locals.var_xn_s).sqrt();
            let assign41280_e54189: f64 = (assign41280_e54186 / assign41280_e54188);
            let assign41280_e54190: f64 = (1.0 + assign41280_e54189);
            (locals.var_nscr, locals.var_nscr_dn5, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, ) = (assign41280_e54190, ((((0.5 * locals.var_gf_dn5) * assign41280_e54188) - (assign41280_e54186 * (locals.var_xn_s_dn5 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * locals.var_gf_dn6) * assign41280_e54188) - (assign41280_e54186 * (locals.var_xn_s_dn6 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * locals.var_gf_dn7) * assign41280_e54188) - (assign41280_e54186 * (locals.var_xn_s_dn7 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * locals.var_gf_dn8) * assign41280_e54188) - (assign41280_e54186 * (locals.var_xn_s_dn8 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), );
            locals.var_nscr_rv = 0.0;
        }

        let assign41290_e54196: f64 = (locals.var_xn_s).sqrt();
        let assign41290_e54197: f64 = (locals.var_gf * assign41290_e54196);
        let assign41290_e54198: f64 = (locals.var_xn_s + assign41290_e54197);
        let assign41290_e54202: f64 = (locals.var_nscr - 1.0);
        let assign41290_e54203: f64 = (assign41290_e54202).ln();
        let assign41290_e54204: f64 = (locals.var_nscr * assign41290_e54203);
        let assign41290_e54205: f64 = (assign41290_e54198 - assign41290_e54204);
        (locals.var_xthscr, locals.var_xthscr_dn5, locals.var_xthscr_dn6, locals.var_xthscr_dn7, locals.var_xthscr_dn8, ) = (assign41290_e54205, ((locals.var_xn_s_dn5 + ((locals.var_gf_dn5 * assign41290_e54196) + (locals.var_gf * (locals.var_xn_s_dn5 / (2.0 * assign41290_e54196))))) - ((locals.var_nscr_dn5 * assign41290_e54203) + (locals.var_nscr * (locals.var_nscr_dn5 / assign41290_e54202)))), ((locals.var_xn_s_dn6 + ((locals.var_gf_dn6 * assign41290_e54196) + (locals.var_gf * (locals.var_xn_s_dn6 / (2.0 * assign41290_e54196))))) - ((locals.var_nscr_dn6 * assign41290_e54203) + (locals.var_nscr * (locals.var_nscr_dn6 / assign41290_e54202)))), ((locals.var_xn_s_dn7 + ((locals.var_gf_dn7 * assign41290_e54196) + (locals.var_gf * (locals.var_xn_s_dn7 / (2.0 * assign41290_e54196))))) - ((locals.var_nscr_dn7 * assign41290_e54203) + (locals.var_nscr * (locals.var_nscr_dn7 / assign41290_e54202)))), ((locals.var_xn_s_dn8 + ((locals.var_gf_dn8 * assign41290_e54196) + (locals.var_gf * (locals.var_xn_s_dn8 / (2.0 * assign41290_e54196))))) - ((locals.var_nscr_dn8 * assign41290_e54203) + (locals.var_nscr * (locals.var_nscr_dn8 / assign41290_e54202)))), );
        locals.var_xthscr_rv = 0.0;

        let assign41300_e54208: f64 = (locals.var_xg - locals.var_xthscr);
        let assign41300_e54210: f64 = (assign41300_e54208 / locals.var_nscr);
        (locals.var_xgtscr, locals.var_xgtscr_dn5, locals.var_xgtscr_dn6, locals.var_xgtscr_dn7, locals.var_xgtscr_dn8, ) = (assign41300_e54210, ((((locals.var_xg_dn5 - locals.var_xthscr_dn5) * locals.var_nscr) - (assign41300_e54208 * locals.var_nscr_dn5)) / (locals.var_nscr * locals.var_nscr)), ((((locals.var_xg_dn6 - locals.var_xthscr_dn6) * locals.var_nscr) - (assign41300_e54208 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr)), ((((locals.var_xg_dn7 - locals.var_xthscr_dn7) * locals.var_nscr) - (assign41300_e54208 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr)), ((((locals.var_xg_dn8 - locals.var_xthscr_dn8) * locals.var_nscr) - (assign41300_e54208 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr)), );
        locals.var_xgtscr_rv = 0.0;

        let assign41310_e54213: f64 = (0.5 * locals.var_gf2);
        let assign41310_e54217: f64 = (8.0 / locals.var_gf2);
        let assign41310_e54218: f64 = (1.0 + assign41310_e54217);
        let assign41310_e54219: f64 = (assign41310_e54218).sqrt();
        let assign41310_e54221: f64 = (assign41310_e54219 - 1.0);
        let assign41310_e54222: f64 = (assign41310_e54213 * assign41310_e54221);
        (locals.var_qbscr, locals.var_qbscr_dn5, locals.var_qbscr_dn6, locals.var_qbscr_dn7, locals.var_qbscr_dn8, ) = (assign41310_e54222, (((0.5 * locals.var_gf2_dn5) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * locals.var_gf2_dn5) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41310_e54219)))), (((0.5 * locals.var_gf2_dn6) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * locals.var_gf2_dn6) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41310_e54219)))), (((0.5 * locals.var_gf2_dn7) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * locals.var_gf2_dn7) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41310_e54219)))), (((0.5 * locals.var_gf2_dn8) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * locals.var_gf2_dn8) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41310_e54219)))), );
        locals.var_qbscr_rv = 0.0;

        (locals.var_qiscr, locals.var_qiscr_dn5, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qiscr_rv = 0.0;

        (locals.var_fscr, locals.var_fscr_dn5, locals.var_fscr_dn6, locals.var_fscr_dn7, locals.var_fscr_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_fscr_rv = 0.0;

        let assign41340_e54227: f64 = (-30.0);
        let assign41340_e54228: f64 = if locals.var_xgtscr > assign41340_e54227 { 1.0 } else { 0.0 };
        locals.var_guard1178 = assign41340_e54228;
        locals.var_guard1178_rv = 0.0;

        if (locals.var_guard1178 != 0.0) {
            let assign41350_e54232: f64 = (locals.var_nscr * locals.var_xgtscr);
            let assign41350_e54234: f64 = (assign41350_e54232 - 1.0);
            (locals.var_xgtscr0, locals.var_xgtscr0_dn5, locals.var_xgtscr0_dn6, locals.var_xgtscr0_dn7, locals.var_xgtscr0_dn8, ) = (assign41350_e54234, ((locals.var_nscr_dn5 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn5)), ((locals.var_nscr_dn6 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn6)), ((locals.var_nscr_dn7 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn7)), ((locals.var_nscr_dn8 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn8)), );
            locals.var_xgtscr0_rv = 0.0;
        }

        if (locals.var_guard1178 != 0.0) {
            let assign41360_e54242: f64 = (locals.var_xgtscr0 * locals.var_xgtscr0);
            let assign41360_e54244: f64 = (assign41360_e54242 + 10.0);
            let assign41360_e54245: f64 = (assign41360_e54244).sqrt();
            let assign41360_e54246: f64 = (locals.var_xgtscr0 + assign41360_e54245);
            let assign41360_e54247: f64 = (0.5 * assign41360_e54246);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign41360_e54247, (0.5 * (locals.var_xgtscr0_dn5 + (((locals.var_xgtscr0_dn5 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn5)) / (2.0 * assign41360_e54245)))), (0.5 * (locals.var_xgtscr0_dn6 + (((locals.var_xgtscr0_dn6 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn6)) / (2.0 * assign41360_e54245)))), (0.5 * (locals.var_xgtscr0_dn7 + (((locals.var_xgtscr0_dn7 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn7)) / (2.0 * assign41360_e54245)))), (0.5 * (locals.var_xgtscr0_dn8 + (((locals.var_xgtscr0_dn8 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn8)) / (2.0 * assign41360_e54245)))), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (locals.var_guard1178 != 0.0) {
            let assign41370_e54253: f64 = (locals.var_temp__blk936).ln();
            let assign41370_e54254: f64 = (locals.var_xgtscr - assign41370_e54253);
            (locals.var_qiscr0si, locals.var_qiscr0si_dn5, locals.var_qiscr0si_dn6, locals.var_qiscr0si_dn7, locals.var_qiscr0si_dn8, ) = (assign41370_e54254, (locals.var_xgtscr_dn5 - (locals.var_temp__blk936_dn5 / locals.var_temp__blk936)), (locals.var_xgtscr_dn6 - (locals.var_temp__blk936_dn6 / locals.var_temp__blk936)), (locals.var_xgtscr_dn7 - (locals.var_temp__blk936_dn7 / locals.var_temp__blk936)), (locals.var_xgtscr_dn8 - (locals.var_temp__blk936_dn8 / locals.var_temp__blk936)), );
            locals.var_qiscr0si_rv = 0.0;
        }

        if (locals.var_guard1178 != 0.0) {
            let assign41380_e54262: f64 = (locals.var_qiscr0si * locals.var_qiscr0si);
            let assign41380_e54264: f64 = (assign41380_e54262 + 2.0);
            let assign41380_e54265: f64 = (assign41380_e54264).sqrt();
            let assign41380_e54266: f64 = (locals.var_qiscr0si + assign41380_e54265);
            let assign41380_e54267: f64 = (0.5 * assign41380_e54266);
            (locals.var_qiscr0, locals.var_qiscr0_dn5, locals.var_qiscr0_dn6, locals.var_qiscr0_dn7, locals.var_qiscr0_dn8, ) = (assign41380_e54267, (0.5 * (locals.var_qiscr0si_dn5 + (((locals.var_qiscr0si_dn5 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn5)) / (2.0 * assign41380_e54265)))), (0.5 * (locals.var_qiscr0si_dn6 + (((locals.var_qiscr0si_dn6 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn6)) / (2.0 * assign41380_e54265)))), (0.5 * (locals.var_qiscr0si_dn7 + (((locals.var_qiscr0si_dn7 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn7)) / (2.0 * assign41380_e54265)))), (0.5 * (locals.var_qiscr0si_dn8 + (((locals.var_qiscr0si_dn8 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn8)) / (2.0 * assign41380_e54265)))), );
            locals.var_qiscr0_rv = 0.0;
        }

        let assign41390_e54272: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41390_e54274: f64 = if assign41390_e54272 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1179 = assign41390_e54274;
        locals.var_guard1179_rv = 0.0;

        if ((locals.var_guard1178 != 0.0) && (locals.var_guard1179 != 0.0)) {
            let assign41400_e54280: f64 = (locals.var_xgtscr - locals.var_qiscr0);
            let assign41400_e54281: f64 = (assign41400_e54280).exp();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign41400_e54281, (assign41400_e54281 * (locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5)), (assign41400_e54281 * (locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6)), (assign41400_e54281 * (locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7)), (assign41400_e54281 * (locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if ((locals.var_guard1178 != 0.0) && (locals.var_guard1179 == 0.0)) {
            let assign41410_e54292: f64 = (locals.var_xgtscr - locals.var_qiscr0);
            let assign41410_e54294: f64 = (assign41410_e54292 - 230.25850929940458);
            let assign41410_e54299: f64 = (locals.var_xgtscr - locals.var_qiscr0);
            let assign41410_e54301: f64 = (assign41410_e54299 - 230.25850929940458);
            let assign41410_e54305: f64 = (locals.var_xgtscr - locals.var_qiscr0);
            let assign41410_e54307: f64 = (assign41410_e54305 - 230.25850929940458);
            let assign41410_e54309: f64 = (assign41410_e54307 * 0.3333333333333333);
            let assign41410_e54310: f64 = (1.0 + assign41410_e54309);
            let assign41410_e54311: f64 = (assign41410_e54301 * assign41410_e54310);
            let assign41410_e54312: f64 = (0.5 * assign41410_e54311);
            let assign41410_e54313: f64 = (1.0 + assign41410_e54312);
            let assign41410_e54314: f64 = (assign41410_e54294 * assign41410_e54313);
            let assign41410_e54315: f64 = (1.0 + assign41410_e54314);
            let assign41410_e54316: f64 = (1e100 * assign41410_e54315);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign41410_e54316, (1e100 * (((locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5) * assign41410_e54310) + (assign41410_e54301 * ((locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * assign41410_e54310) + (assign41410_e54301 * ((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * assign41410_e54310) + (assign41410_e54301 * ((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * assign41410_e54310) + (assign41410_e54301 * ((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * 0.3333333333333333))))))), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (locals.var_guard1178 != 0.0) {
            let assign41420_e54322: f64 = (locals.var_temp__blk936 / locals.var_nscr);
            (locals.var_dscr0, locals.var_dscr0_dn5, locals.var_dscr0_dn6, locals.var_dscr0_dn7, locals.var_dscr0_dn8, ) = (assign41420_e54322, (((locals.var_temp__blk936_dn5 * locals.var_nscr) - (locals.var_temp__blk936 * locals.var_nscr_dn5)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk936_dn6 * locals.var_nscr) - (locals.var_temp__blk936 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk936_dn7 * locals.var_nscr) - (locals.var_temp__blk936 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk936_dn8 * locals.var_nscr) - (locals.var_temp__blk936 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr)), );
            locals.var_dscr0_rv = 0.0;
        }

        if (locals.var_guard1178 != 0.0) {
            let assign41430_e54329: f64 = (locals.var_qiscr0 + 1.0);
            let assign41430_e54330: f64 = (2.0 * assign41430_e54329);
            let assign41430_e54332: f64 = (assign41430_e54330 - locals.var_dscr0);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign41430_e54332, ((2.0 * locals.var_qiscr0_dn5) - locals.var_dscr0_dn5), ((2.0 * locals.var_qiscr0_dn6) - locals.var_dscr0_dn6), ((2.0 * locals.var_qiscr0_dn7) - locals.var_dscr0_dn7), ((2.0 * locals.var_qiscr0_dn8) - locals.var_dscr0_dn8), );
            locals.var_temp__blk936_rv = 0.0;
        }

        let assign41440_e54337: f64 = if locals.var_dscr0 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1180 = assign41440_e54337;
        locals.var_guard1180_rv = 0.0;

        if ((locals.var_guard1178 != 0.0) && (locals.var_guard1180 != 0.0)) {
            let assign41450_e54346: f64 = (locals.var_dscr0 * locals.var_temp__blk936);
            let assign41450_e54347: f64 = (1.0 + assign41450_e54346);
            let assign41450_e54348: f64 = (assign41450_e54347).sqrt();
            let assign41450_e54350: f64 = (assign41450_e54348 - 1.0);
            let assign41450_e54352: f64 = (assign41450_e54350 / locals.var_dscr0);
            let assign41450_e54353: f64 = (locals.var_qiscr0 - assign41450_e54352);
            let assign41450_e54355: f64 = (assign41450_e54353 + 1.0);
            let assign41450_e54356: f64 = (locals.var_nscr * assign41450_e54355);
            (locals.var_qiscr, locals.var_qiscr_dn5, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8, ) = (assign41450_e54356, ((locals.var_nscr_dn5 * assign41450_e54355) + (locals.var_nscr * (locals.var_qiscr0_dn5 - ((((((locals.var_dscr0_dn5 * locals.var_temp__blk936) + (locals.var_dscr0 * locals.var_temp__blk936_dn5)) / (2.0 * assign41450_e54348)) * locals.var_dscr0) - (assign41450_e54350 * locals.var_dscr0_dn5)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn6 * assign41450_e54355) + (locals.var_nscr * (locals.var_qiscr0_dn6 - ((((((locals.var_dscr0_dn6 * locals.var_temp__blk936) + (locals.var_dscr0 * locals.var_temp__blk936_dn6)) / (2.0 * assign41450_e54348)) * locals.var_dscr0) - (assign41450_e54350 * locals.var_dscr0_dn6)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn7 * assign41450_e54355) + (locals.var_nscr * (locals.var_qiscr0_dn7 - ((((((locals.var_dscr0_dn7 * locals.var_temp__blk936) + (locals.var_dscr0 * locals.var_temp__blk936_dn7)) / (2.0 * assign41450_e54348)) * locals.var_dscr0) - (assign41450_e54350 * locals.var_dscr0_dn7)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn8 * assign41450_e54355) + (locals.var_nscr * (locals.var_qiscr0_dn8 - ((((((locals.var_dscr0_dn8 * locals.var_temp__blk936) + (locals.var_dscr0 * locals.var_temp__blk936_dn8)) / (2.0 * assign41450_e54348)) * locals.var_dscr0) - (assign41450_e54350 * locals.var_dscr0_dn8)) / (locals.var_dscr0 * locals.var_dscr0))))), );
            locals.var_qiscr_rv = 0.0;
        }

        if ((locals.var_guard1178 != 0.0) && (locals.var_guard1180 == 0.0)) {
            let assign41460_e54365: f64 = (locals.var_nscr * 0.5);
            let assign41460_e54367: f64 = (assign41460_e54365 * locals.var_dscr0);
            let assign41460_e54371: f64 = (0.25 * locals.var_temp__blk936);
            let assign41460_e54373: f64 = (assign41460_e54371 * locals.var_temp__blk936);
            let assign41460_e54374: f64 = (1.0 + assign41460_e54373);
            let assign41460_e54375: f64 = (assign41460_e54367 * assign41460_e54374);
            (locals.var_qiscr, locals.var_qiscr_dn5, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8, ) = (assign41460_e54375, (((((locals.var_nscr_dn5 * 0.5) * locals.var_dscr0) + (assign41460_e54365 * locals.var_dscr0_dn5)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * locals.var_temp__blk936_dn5) * locals.var_temp__blk936) + (assign41460_e54371 * locals.var_temp__blk936_dn5)))), (((((locals.var_nscr_dn6 * 0.5) * locals.var_dscr0) + (assign41460_e54365 * locals.var_dscr0_dn6)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * locals.var_temp__blk936_dn6) * locals.var_temp__blk936) + (assign41460_e54371 * locals.var_temp__blk936_dn6)))), (((((locals.var_nscr_dn7 * 0.5) * locals.var_dscr0) + (assign41460_e54365 * locals.var_dscr0_dn7)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * locals.var_temp__blk936_dn7) * locals.var_temp__blk936) + (assign41460_e54371 * locals.var_temp__blk936_dn7)))), (((((locals.var_nscr_dn8 * 0.5) * locals.var_dscr0) + (assign41460_e54365 * locals.var_dscr0_dn8)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * locals.var_temp__blk936_dn8) * locals.var_temp__blk936) + (assign41460_e54371 * locals.var_temp__blk936_dn8)))), );
            locals.var_qiscr_rv = 0.0;
        }

        if (locals.var_guard1178 != 0.0) {
            let assign41470_e54382: f64 = (locals.var_xg - locals.var_qiscr);
            let assign41470_e54384: f64 = (assign41470_e54382 + 2.0);
            let assign41470_e54387: f64 = (locals.var_xg - locals.var_qiscr);
            let assign41470_e54389: f64 = (assign41470_e54387 - 2.0);
            let assign41470_e54392: f64 = (locals.var_xg - locals.var_qiscr);
            let assign41470_e54394: f64 = (assign41470_e54392 - 2.0);
            let assign41470_e54395: f64 = (assign41470_e54389 * assign41470_e54394);
            let assign41470_e54397: f64 = (assign41470_e54395 + 1.0);
            let assign41470_e54398: f64 = (assign41470_e54397).sqrt();
            let assign41470_e54399: f64 = (assign41470_e54384 + assign41470_e54398);
            let assign41470_e54400: f64 = (0.5 * assign41470_e54399);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign41470_e54400, (0.5 * ((locals.var_xg_dn5 - locals.var_qiscr_dn5) + ((((locals.var_xg_dn5 - locals.var_qiscr_dn5) * assign41470_e54394) + (assign41470_e54389 * (locals.var_xg_dn5 - locals.var_qiscr_dn5))) / (2.0 * assign41470_e54398)))), (0.5 * ((locals.var_xg_dn6 - locals.var_qiscr_dn6) + ((((locals.var_xg_dn6 - locals.var_qiscr_dn6) * assign41470_e54394) + (assign41470_e54389 * (locals.var_xg_dn6 - locals.var_qiscr_dn6))) / (2.0 * assign41470_e54398)))), (0.5 * ((locals.var_xg_dn7 - locals.var_qiscr_dn7) + ((((locals.var_xg_dn7 - locals.var_qiscr_dn7) * assign41470_e54394) + (assign41470_e54389 * (locals.var_xg_dn7 - locals.var_qiscr_dn7))) / (2.0 * assign41470_e54398)))), (0.5 * ((locals.var_xg_dn8 - locals.var_qiscr_dn8) + ((((locals.var_xg_dn8 - locals.var_qiscr_dn8) * assign41470_e54394) + (assign41470_e54389 * (locals.var_xg_dn8 - locals.var_qiscr_dn8))) / (2.0 * assign41470_e54398)))), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (locals.var_guard1178 != 0.0) {
            let assign41480_e54406: f64 = (0.5 * locals.var_gf2);
            let assign41480_e54410: f64 = (4.0 / locals.var_gf2);
            let assign41480_e54412: f64 = (assign41480_e54410 * locals.var_temp__blk936);
            let assign41480_e54413: f64 = (1.0 + assign41480_e54412);
            let assign41480_e54414: f64 = (assign41480_e54413).sqrt();
            let assign41480_e54416: f64 = (assign41480_e54414 - 1.0);
            let assign41480_e54417: f64 = (assign41480_e54406 * assign41480_e54416);
            (locals.var_qbscr, locals.var_qbscr_dn5, locals.var_qbscr_dn6, locals.var_qbscr_dn7, locals.var_qbscr_dn8, ) = (assign41480_e54417, (((0.5 * locals.var_gf2_dn5) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * locals.var_gf2_dn5) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk936) + (assign41480_e54410 * locals.var_temp__blk936_dn5)) / (2.0 * assign41480_e54414)))), (((0.5 * locals.var_gf2_dn6) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * locals.var_gf2_dn6) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk936) + (assign41480_e54410 * locals.var_temp__blk936_dn6)) / (2.0 * assign41480_e54414)))), (((0.5 * locals.var_gf2_dn7) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * locals.var_gf2_dn7) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk936) + (assign41480_e54410 * locals.var_temp__blk936_dn7)) / (2.0 * assign41480_e54414)))), (((0.5 * locals.var_gf2_dn8) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * locals.var_gf2_dn8) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk936) + (assign41480_e54410 * locals.var_temp__blk936_dn8)) / (2.0 * assign41480_e54414)))), );
            locals.var_qbscr_rv = 0.0;
        }

        if (locals.var_guard1178 != 0.0) {
            let assign41490_e54424: f64 = (locals.var_qbscr + locals.var_qiscr);
            let assign41490_e54425: f64 = (locals.var_qbscr / assign41490_e54424);
            (locals.var_fscr, locals.var_fscr_dn5, locals.var_fscr_dn6, locals.var_fscr_dn7, locals.var_fscr_dn8, ) = (assign41490_e54425, (((locals.var_qbscr_dn5 * assign41490_e54424) - (locals.var_qbscr * (locals.var_qbscr_dn5 + locals.var_qiscr_dn5))) / (assign41490_e54424 * assign41490_e54424)), (((locals.var_qbscr_dn6 * assign41490_e54424) - (locals.var_qbscr * (locals.var_qbscr_dn6 + locals.var_qiscr_dn6))) / (assign41490_e54424 * assign41490_e54424)), (((locals.var_qbscr_dn7 * assign41490_e54424) - (locals.var_qbscr * (locals.var_qbscr_dn7 + locals.var_qiscr_dn7))) / (assign41490_e54424 * assign41490_e54424)), (((locals.var_qbscr_dn8 * assign41490_e54424) - (locals.var_qbscr * (locals.var_qbscr_dn8 + locals.var_qiscr_dn8))) / (assign41490_e54424 * assign41490_e54424)), );
            locals.var_fscr_rv = 0.0;
        }

        if (locals.var_guard1178 != 0.0) {
            let assign41500_e54432: f64 = (locals.var_fscr * locals.var_delxb);
            let assign41500_e54433: f64 = (locals.var_xno_s - assign41500_e54432);
            (locals.var_xn_s, locals.var_xn_s_dn5, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8, ) = (assign41500_e54433, (locals.var_xno_s_dn5 - ((locals.var_fscr_dn5 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn5))), (locals.var_xno_s_dn6 - ((locals.var_fscr_dn6 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn6))), (locals.var_xno_s_dn7 - ((locals.var_fscr_dn7 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn7))), (locals.var_xno_s_dn8 - ((locals.var_fscr_dn8 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn8))), );
            locals.var_xn_s_rv = 0.0;
        }

        let assign41510_e54439: f64 = (locals.var_gf * 0.7071067811865475);
        let assign41510_e54440: f64 = (1.0 + assign41510_e54439);
        (locals.var_xi, locals.var_xi_dn5, locals.var_xi_dn6, locals.var_xi_dn7, locals.var_xi_dn8, ) = (assign41510_e54440, (locals.var_gf_dn5 * 0.7071067811865475), (locals.var_gf_dn6 * 0.7071067811865475), (locals.var_gf_dn7 * 0.7071067811865475), (locals.var_gf_dn8 * 0.7071067811865475), );
        locals.var_xi_rv = 0.0;

        let assign41520_e54443: f64 = (1e-5 * locals.var_xi);
        (locals.var_margin, locals.var_margin_dn5, locals.var_margin_dn6, locals.var_margin_dn7, locals.var_margin_dn8, ) = (assign41520_e54443, (1e-5 * locals.var_xi_dn5), (1e-5 * locals.var_xi_dn6), (1e-5 * locals.var_xi_dn7), (1e-5 * locals.var_xi_dn8), );
        locals.var_margin_rv = 0.0;

        let assign41530_e54446: f64 = (1.0 / locals.var_xi);
        (locals.var_inv_xi, locals.var_inv_xi_dn5, locals.var_inv_xi_dn6, locals.var_inv_xi_dn7, locals.var_inv_xi_dn8, ) = (assign41530_e54446, (-(locals.var_xi_dn5 / (locals.var_xi * locals.var_xi))), (-(locals.var_xi_dn6 / (locals.var_xi * locals.var_xi))), (-(locals.var_xi_dn7 / (locals.var_xi * locals.var_xi))), (-(locals.var_xi_dn8 / (locals.var_xi * locals.var_xi))), );
        locals.var_inv_xi_rv = 0.0;

        (locals.var_sp_s_x1, locals.var_sp_s_x1_dn5, locals.var_sp_s_x1_dn6, locals.var_sp_s_x1_dn7, locals.var_sp_s_x1_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_sp_s_x1_rv = 0.0;

        (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_x_s_rv = 0.0;

        let assign41560_e54451: f64 = if locals.var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1181 = assign41560_e54451;
        locals.var_guard1181_rv = 0.0;

        if (locals.var_guard1181 != 0.0) {
            let assign41570_e54454: f64 = (-locals.var_xn_s);
            let assign41570_e54455: f64 = (assign41570_e54454).exp();
            (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, ) = (assign41570_e54455, (assign41570_e54455 * (-locals.var_xn_s_dn5)), (assign41570_e54455 * (-locals.var_xn_s_dn6)), (assign41570_e54455 * (-locals.var_xn_s_dn7)), (assign41570_e54455 * (-locals.var_xn_s_dn8)), );
            locals.var_delta_ns_rv = 0.0;
        }

        if (locals.var_guard1181 == 0.0) {
            let assign41580_e54464: f64 = (locals.var_xn_s - 460.51701859880916);
            let assign41580_e54469: f64 = (locals.var_xn_s - 460.51701859880916);
            let assign41580_e54473: f64 = (locals.var_xn_s - 460.51701859880916);
            let assign41580_e54475: f64 = (assign41580_e54473 * 0.3333333333333333);
            let assign41580_e54476: f64 = (1.0 + assign41580_e54475);
            let assign41580_e54477: f64 = (assign41580_e54469 * assign41580_e54476);
            let assign41580_e54478: f64 = (0.5 * assign41580_e54477);
            let assign41580_e54479: f64 = (1.0 + assign41580_e54478);
            let assign41580_e54480: f64 = (assign41580_e54464 * assign41580_e54479);
            let assign41580_e54481: f64 = (1.0 + assign41580_e54480);
            let assign41580_e54482: f64 = (1e-200 / assign41580_e54481);
            (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, ) = (assign41580_e54482, (-((1e-200 * ((locals.var_xn_s_dn5 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((locals.var_xn_s_dn5 * assign41580_e54476) + (assign41580_e54469 * (locals.var_xn_s_dn5 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((locals.var_xn_s_dn6 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((locals.var_xn_s_dn6 * assign41580_e54476) + (assign41580_e54469 * (locals.var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((locals.var_xn_s_dn7 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((locals.var_xn_s_dn7 * assign41580_e54476) + (assign41580_e54469 * (locals.var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((locals.var_xn_s_dn8 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((locals.var_xn_s_dn8 * assign41580_e54476) + (assign41580_e54469 * (locals.var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), );
            locals.var_delta_ns_rv = 0.0;
        }

        let assign41590_e54486: f64 = (locals.var_xg).abs();
        let assign41590_e54488: f64 = if assign41590_e54486 <= locals.var_margin { 1.0 } else { 0.0 };
        locals.var_guard1182 = assign41590_e54488;
        locals.var_guard1182_rv = 0.0;

        if (locals.var_guard1182 != 0.0) {
            let assign41600_e54492: f64 = (locals.var_inv_xi * locals.var_inv_xi);
            let assign41600_e54494: f64 = (assign41600_e54492 * 0.16666666666666666);
            let assign41600_e54496: f64 = (assign41600_e54494 * 0.7071067811865475);
            (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, ) = (assign41600_e54496, ((((locals.var_inv_xi_dn5 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn6 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn7 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn8 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475), );
            locals.var_sp_s_temp1_rv = 0.0;
        }

        if (locals.var_guard1182 != 0.0) {
            let assign41610_e54502: f64 = (locals.var_xg * locals.var_inv_xi);
            let assign41610_e54507: f64 = (1.0 - locals.var_delta_ns);
            let assign41610_e54508: f64 = (locals.var_xg * assign41610_e54507);
            let assign41610_e54510: f64 = (assign41610_e54508 * locals.var_gf);
            let assign41610_e54512: f64 = (assign41610_e54510 * locals.var_sp_s_temp1);
            let assign41610_e54513: f64 = (1.0 + assign41610_e54512);
            let assign41610_e54514: f64 = (assign41610_e54502 * assign41610_e54513);
            (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, ) = (assign41610_e54514, ((((locals.var_xg_dn5 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn5)) * assign41610_e54513) + (assign41610_e54502 * ((((((locals.var_xg_dn5 * assign41610_e54507) + (locals.var_xg * (-locals.var_delta_ns_dn5))) * locals.var_gf) + (assign41610_e54508 * locals.var_gf_dn5)) * locals.var_sp_s_temp1) + (assign41610_e54510 * locals.var_sp_s_temp1_dn5)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign41610_e54513) + (assign41610_e54502 * ((((((locals.var_xg_dn6 * assign41610_e54507) + (locals.var_xg * (-locals.var_delta_ns_dn6))) * locals.var_gf) + (assign41610_e54508 * locals.var_gf_dn6)) * locals.var_sp_s_temp1) + (assign41610_e54510 * locals.var_sp_s_temp1_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign41610_e54513) + (assign41610_e54502 * ((((((locals.var_xg_dn7 * assign41610_e54507) + (locals.var_xg * (-locals.var_delta_ns_dn7))) * locals.var_gf) + (assign41610_e54508 * locals.var_gf_dn7)) * locals.var_sp_s_temp1) + (assign41610_e54510 * locals.var_sp_s_temp1_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign41610_e54513) + (assign41610_e54502 * ((((((locals.var_xg_dn8 * assign41610_e54507) + (locals.var_xg * (-locals.var_delta_ns_dn8))) * locals.var_gf) + (assign41610_e54508 * locals.var_gf_dn8)) * locals.var_sp_s_temp1) + (assign41610_e54510 * locals.var_sp_s_temp1_dn8)))), );
            locals.var_x_s_rv = 0.0;
        }

        let assign41620_e54519: f64 = (-locals.var_margin);
        let assign41620_e54520: f64 = if locals.var_xg < assign41620_e54519 { 1.0 } else { 0.0 };
        locals.var_guard1183 = assign41620_e54520;
        locals.var_guard1183_rv = 0.0;

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41630_e54526: f64 = (-locals.var_xg);
            (locals.var_sp_s_yg, locals.var_sp_s_yg_dn5, locals.var_sp_s_yg_dn6, locals.var_sp_s_yg_dn7, locals.var_sp_s_yg_dn8, ) = (assign41630_e54526, (-locals.var_xg_dn5), (-locals.var_xg_dn6), (-locals.var_xg_dn7), (-locals.var_xg_dn8), );
            locals.var_sp_s_yg_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41640_e54536: f64 = (locals.var_sp_s_yg * locals.var_inv_xi);
            let assign41640_e54537: f64 = (1.25 * assign41640_e54536);
            (locals.var_sp_s_ysub, locals.var_sp_s_ysub_dn5, locals.var_sp_s_ysub_dn6, locals.var_sp_s_ysub_dn7, locals.var_sp_s_ysub_dn8, ) = (assign41640_e54537, (1.25 * ((locals.var_sp_s_yg_dn5 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn5))), (1.25 * ((locals.var_sp_s_yg_dn6 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn6))), (1.25 * ((locals.var_sp_s_yg_dn7 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn7))), (1.25 * ((locals.var_sp_s_yg_dn8 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn8))), );
            locals.var_sp_s_ysub_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41650_e54547: f64 = (locals.var_sp_s_ysub + 10.0);
            let assign41650_e54550: f64 = (locals.var_sp_s_ysub - 6.0);
            let assign41650_e54553: f64 = (locals.var_sp_s_ysub - 6.0);
            let assign41650_e54554: f64 = (assign41650_e54550 * assign41650_e54553);
            let assign41650_e54556: f64 = (assign41650_e54554 + 64.0);
            let assign41650_e54557: f64 = (assign41650_e54556).sqrt();
            let assign41650_e54558: f64 = (assign41650_e54547 - assign41650_e54557);
            let assign41650_e54559: f64 = (0.5 * assign41650_e54558);
            (locals.var_sp_s_eta, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, ) = (assign41650_e54559, (0.5 * (locals.var_sp_s_ysub_dn5 - (((locals.var_sp_s_ysub_dn5 * assign41650_e54553) + (assign41650_e54550 * locals.var_sp_s_ysub_dn5)) / (2.0 * assign41650_e54557)))), (0.5 * (locals.var_sp_s_ysub_dn6 - (((locals.var_sp_s_ysub_dn6 * assign41650_e54553) + (assign41650_e54550 * locals.var_sp_s_ysub_dn6)) / (2.0 * assign41650_e54557)))), (0.5 * (locals.var_sp_s_ysub_dn7 - (((locals.var_sp_s_ysub_dn7 * assign41650_e54553) + (assign41650_e54550 * locals.var_sp_s_ysub_dn7)) / (2.0 * assign41650_e54557)))), (0.5 * (locals.var_sp_s_ysub_dn8 - (((locals.var_sp_s_ysub_dn8 * assign41650_e54553) + (assign41650_e54550 * locals.var_sp_s_ysub_dn8)) / (2.0 * assign41650_e54557)))), );
            locals.var_sp_s_eta_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41660_e54568: f64 = (locals.var_sp_s_yg - locals.var_sp_s_eta);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign41660_e54568, (locals.var_sp_s_yg_dn5 - locals.var_sp_s_eta_dn5), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_eta_dn8), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41670_e54577: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
            let assign41670_e54581: f64 = (locals.var_sp_s_eta + 1.0);
            let assign41670_e54582: f64 = (locals.var_gf2 * assign41670_e54581);
            let assign41670_e54583: f64 = (assign41670_e54577 + assign41670_e54582);
            (locals.var_sp_s_a, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, ) = (assign41670_e54583, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) + ((locals.var_gf2_dn5 * assign41670_e54581) + (locals.var_gf2 * locals.var_sp_s_eta_dn5))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) + ((locals.var_gf2_dn6 * assign41670_e54581) + (locals.var_gf2 * locals.var_sp_s_eta_dn6))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) + ((locals.var_gf2_dn7 * assign41670_e54581) + (locals.var_gf2 * locals.var_sp_s_eta_dn7))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) + ((locals.var_gf2_dn8 * assign41670_e54581) + (locals.var_gf2 * locals.var_sp_s_eta_dn8))), );
            locals.var_sp_s_a_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41680_e54592: f64 = (2.0 * locals.var_sp_s_temp);
            let assign41680_e54594: f64 = (assign41680_e54592 - locals.var_gf2);
            (locals.var_sp_s_c, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, ) = (assign41680_e54594, ((2.0 * locals.var_sp_s_temp_dn5) - locals.var_gf2_dn5), ((2.0 * locals.var_sp_s_temp_dn6) - locals.var_gf2_dn6), ((2.0 * locals.var_sp_s_temp_dn7) - locals.var_gf2_dn7), ((2.0 * locals.var_sp_s_temp_dn8) - locals.var_gf2_dn8), );
            locals.var_sp_s_c_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41690_e54602: f64 = (-locals.var_sp_s_eta);
            let assign41690_e54605: f64 = (locals.var_sp_s_a * locals.var_inv_gf2);
            let assign41690_e54606: f64 = (assign41690_e54605).ln();
            let assign41690_e54607: f64 = (assign41690_e54602 + assign41690_e54606);
            (locals.var_sp_s_tau, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, ) = (assign41690_e54607, ((-locals.var_sp_s_eta_dn5) + (((locals.var_sp_s_a_dn5 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn5)) / assign41690_e54605)), ((-locals.var_sp_s_eta_dn6) + (((locals.var_sp_s_a_dn6 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn6)) / assign41690_e54605)), ((-locals.var_sp_s_eta_dn7) + (((locals.var_sp_s_a_dn7 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn7)) / assign41690_e54605)), ((-locals.var_sp_s_eta_dn8) + (((locals.var_sp_s_a_dn8 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn8)) / assign41690_e54605)), );
            locals.var_sp_s_tau_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41700_e54616: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
            (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, ) = (assign41700_e54616, (locals.var_sp_s_a_dn5 + locals.var_sp_s_c_dn5), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), );
            locals.var_nu_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_13(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41710_e54625: f64 = (locals.var_nu * locals.var_nu);
            let assign41710_e54630: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
            let assign41710_e54631: f64 = (0.5 * assign41710_e54630);
            let assign41710_e54633: f64 = (assign41710_e54631 - locals.var_sp_s_a);
            let assign41710_e54634: f64 = (locals.var_sp_s_tau * assign41710_e54633);
            let assign41710_e54635: f64 = (assign41710_e54625 + assign41710_e54634);
            (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, ) = (assign41710_e54635, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau_dn5 * assign41710_e54633) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5))) - locals.var_sp_s_a_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign41710_e54633) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - locals.var_sp_s_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign41710_e54633) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - locals.var_sp_s_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign41710_e54633) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - locals.var_sp_s_a_dn8)))), );
            locals.var_mutau_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41720_e54645: f64 = (locals.var_sp_s_a * locals.var_nu);
            let assign41720_e54647: f64 = (assign41720_e54645 * locals.var_sp_s_tau);
            let assign41720_e54651: f64 = (locals.var_nu / locals.var_mutau);
            let assign41720_e54653: f64 = (assign41720_e54651 * locals.var_sp_s_tau);
            let assign41720_e54655: f64 = (assign41720_e54653 * locals.var_sp_s_tau);
            let assign41720_e54657: f64 = (assign41720_e54655 * locals.var_sp_s_c);
            let assign41720_e54660: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
            let assign41720_e54662: f64 = (assign41720_e54660 * 0.3333333333333333);
            let assign41720_e54664: f64 = (assign41720_e54662 - locals.var_sp_s_a);
            let assign41720_e54665: f64 = (assign41720_e54657 * assign41720_e54664);
            let assign41720_e54666: f64 = (locals.var_mutau + assign41720_e54665);
            let assign41720_e54667: f64 = (assign41720_e54647 / assign41720_e54666);
            let assign41720_e54668: f64 = (locals.var_sp_s_eta + assign41720_e54667);
            (locals.var_sp_s_y0, locals.var_sp_s_y0_dn5, locals.var_sp_s_y0_dn6, locals.var_sp_s_y0_dn7, locals.var_sp_s_y0_dn8, ) = (assign41720_e54668, (locals.var_sp_s_eta_dn5 + (((((((locals.var_sp_s_a_dn5 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn5)) * locals.var_sp_s_tau) + (assign41720_e54645 * locals.var_sp_s_tau_dn5)) * assign41720_e54666) - (assign41720_e54647 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41720_e54651 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_tau) + (assign41720_e54653 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_c) + (assign41720_e54655 * locals.var_sp_s_c_dn5)) * assign41720_e54664) + (assign41720_e54657 * ((((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5)) * 0.3333333333333333) - locals.var_sp_s_a_dn5)))))) / (assign41720_e54666 * assign41720_e54666))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign41720_e54645 * locals.var_sp_s_tau_dn6)) * assign41720_e54666) - (assign41720_e54647 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41720_e54651 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign41720_e54653 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign41720_e54655 * locals.var_sp_s_c_dn6)) * assign41720_e54664) + (assign41720_e54657 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - locals.var_sp_s_a_dn6)))))) / (assign41720_e54666 * assign41720_e54666))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign41720_e54645 * locals.var_sp_s_tau_dn7)) * assign41720_e54666) - (assign41720_e54647 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41720_e54651 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign41720_e54653 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign41720_e54655 * locals.var_sp_s_c_dn7)) * assign41720_e54664) + (assign41720_e54657 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - locals.var_sp_s_a_dn7)))))) / (assign41720_e54666 * assign41720_e54666))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign41720_e54645 * locals.var_sp_s_tau_dn8)) * assign41720_e54666) - (assign41720_e54647 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41720_e54651 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign41720_e54653 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign41720_e54655 * locals.var_sp_s_c_dn8)) * assign41720_e54664) + (assign41720_e54657 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - locals.var_sp_s_a_dn8)))))) / (assign41720_e54666 * assign41720_e54666))), );
            locals.var_sp_s_y0_rv = 0.0;
        }

        let assign41730_e54673: f64 = if locals.var_sp_s_y0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1184 = assign41730_e54673;
        locals.var_guard1184_rv = 0.0;

        if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) && (locals.var_guard1184 != 0.0)) {
            let assign41740_e54681: f64 = (locals.var_sp_s_y0).exp();
            (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, ) = (assign41740_e54681, (assign41740_e54681 * locals.var_sp_s_y0_dn5), (assign41740_e54681 * locals.var_sp_s_y0_dn6), (assign41740_e54681 * locals.var_sp_s_y0_dn7), (assign41740_e54681 * locals.var_sp_s_y0_dn8), );
            locals.var_sp_s_delta0_rv = 0.0;
        }

        if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) && (locals.var_guard1184 == 0.0)) {
            let assign41750_e54695: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
            let assign41750_e54700: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
            let assign41750_e54704: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
            let assign41750_e54706: f64 = (assign41750_e54704 * 0.3333333333333333);
            let assign41750_e54707: f64 = (1.0 + assign41750_e54706);
            let assign41750_e54708: f64 = (assign41750_e54700 * assign41750_e54707);
            let assign41750_e54709: f64 = (0.5 * assign41750_e54708);
            let assign41750_e54710: f64 = (1.0 + assign41750_e54709);
            let assign41750_e54711: f64 = (assign41750_e54695 * assign41750_e54710);
            let assign41750_e54712: f64 = (1.0 + assign41750_e54711);
            let assign41750_e54713: f64 = (1e100 * assign41750_e54712);
            (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, ) = (assign41750_e54713, (1e100 * ((locals.var_sp_s_y0_dn5 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((locals.var_sp_s_y0_dn5 * assign41750_e54707) + (assign41750_e54700 * (locals.var_sp_s_y0_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn6 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((locals.var_sp_s_y0_dn6 * assign41750_e54707) + (assign41750_e54700 * (locals.var_sp_s_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn7 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((locals.var_sp_s_y0_dn7 * assign41750_e54707) + (assign41750_e54700 * (locals.var_sp_s_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn8 * assign41750_e54710) + (assign41750_e54695 * (0.5 * ((locals.var_sp_s_y0_dn8 * assign41750_e54707) + (assign41750_e54700 * (locals.var_sp_s_y0_dn8 * 0.3333333333333333))))))), );
            locals.var_sp_s_delta0_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41760_e54722: f64 = (1.0 / locals.var_sp_s_delta0);
            (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, ) = (assign41760_e54722, (-(locals.var_sp_s_delta0_dn5 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), );
            locals.var_sp_s_delta1_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41770_e54733: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
            let assign41770_e54734: f64 = (2.0 + assign41770_e54733);
            let assign41770_e54735: f64 = (1.0 / assign41770_e54734);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign41770_e54735, (-(((locals.var_sp_s_y0_dn5 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn5)) / (assign41770_e54734 * assign41770_e54734))), (-(((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) / (assign41770_e54734 * assign41770_e54734))), (-(((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) / (assign41770_e54734 * assign41770_e54734))), (-(((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) / (assign41770_e54734 * assign41770_e54734))), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41780_e54744: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
            let assign41780_e54746: f64 = (assign41780_e54744 * locals.var_sp_s_temp);
            (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, ) = (assign41780_e54746, ((((locals.var_sp_s_y0_dn5 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn5)) * locals.var_sp_s_temp) + (assign41780_e54744 * locals.var_sp_s_temp_dn5)), ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) * locals.var_sp_s_temp) + (assign41780_e54744 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) * locals.var_sp_s_temp) + (assign41780_e54744 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) * locals.var_sp_s_temp) + (assign41780_e54744 * locals.var_sp_s_temp_dn8)), );
            locals.var_sp_s_xi0_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41790_e54756: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_temp);
            let assign41790_e54758: f64 = (assign41790_e54756 * locals.var_sp_s_temp);
            let assign41790_e54759: f64 = (4.0 * assign41790_e54758);
            (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, ) = (assign41790_e54759, (4.0 * ((((locals.var_sp_s_y0_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign41790_e54756 * locals.var_sp_s_temp_dn5))), (4.0 * ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign41790_e54756 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign41790_e54756 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign41790_e54756 * locals.var_sp_s_temp_dn8))), );
            locals.var_sp_s_xi1_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41800_e54768: f64 = (8.0 * locals.var_sp_s_temp);
            let assign41800_e54771: f64 = (12.0 * locals.var_sp_s_xi0);
            let assign41800_e54772: f64 = (assign41800_e54768 - assign41800_e54771);
            let assign41800_e54774: f64 = (assign41800_e54772 * locals.var_sp_s_temp);
            let assign41800_e54776: f64 = (assign41800_e54774 * locals.var_sp_s_temp);
            (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, ) = (assign41800_e54776, ((((((8.0 * locals.var_sp_s_temp_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp) + (assign41800_e54772 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign41800_e54774 * locals.var_sp_s_temp_dn5)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign41800_e54772 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign41800_e54774 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign41800_e54772 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign41800_e54774 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign41800_e54772 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign41800_e54774 * locals.var_sp_s_temp_dn8)), );
            locals.var_sp_s_xi2_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41810_e54785: f64 = (locals.var_sp_s_yg - locals.var_sp_s_y0);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign41810_e54785, (locals.var_sp_s_yg_dn5 - locals.var_sp_s_y0_dn5), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_y0_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_y0_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_y0_dn8), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41820_e54794: f64 = (locals.var_delta_ns * locals.var_sp_s_delta1);
            (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, ) = (assign41820_e54794, ((locals.var_delta_ns_dn5 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn5)), ((locals.var_delta_ns_dn6 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn6)), ((locals.var_delta_ns_dn7 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn7)), ((locals.var_delta_ns_dn8 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn8)), );
            locals.var_sp_s_temp1_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41830_e54803: f64 = (2.0 * locals.var_sp_s_temp);
            let assign41830_e54807: f64 = (locals.var_sp_s_delta0 - 1.0);
            let assign41830_e54809: f64 = (assign41830_e54807 - locals.var_sp_s_temp1);
            let assign41830_e54813: f64 = (1.0 - locals.var_sp_s_xi1);
            let assign41830_e54814: f64 = (locals.var_delta_ns * assign41830_e54813);
            let assign41830_e54815: f64 = (assign41830_e54809 + assign41830_e54814);
            let assign41830_e54816: f64 = (locals.var_gf2 * assign41830_e54815);
            let assign41830_e54817: f64 = (assign41830_e54803 + assign41830_e54816);
            (locals.var_sp_s_pc, locals.var_sp_s_pc_dn5, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, ) = (assign41830_e54817, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign41830_e54815) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn5 - locals.var_sp_s_temp1_dn5) + ((locals.var_delta_ns_dn5 * assign41830_e54813) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn5))))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign41830_e54815) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_temp1_dn6) + ((locals.var_delta_ns_dn6 * assign41830_e54813) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn6))))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign41830_e54815) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_temp1_dn7) + ((locals.var_delta_ns_dn7 * assign41830_e54813) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn7))))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign41830_e54815) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_temp1_dn8) + ((locals.var_delta_ns_dn8 * assign41830_e54813) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn8))))))), );
            locals.var_sp_s_pc_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41840_e54826: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
            let assign41840_e54830: f64 = (locals.var_sp_s_delta0 - locals.var_sp_s_y0);
            let assign41840_e54832: f64 = (assign41840_e54830 - 1.0);
            let assign41840_e54834: f64 = (assign41840_e54832 + locals.var_sp_s_temp1);
            let assign41840_e54838: f64 = (locals.var_sp_s_y0 - 1.0);
            let assign41840_e54840: f64 = (assign41840_e54838 - locals.var_sp_s_xi0);
            let assign41840_e54841: f64 = (locals.var_delta_ns * assign41840_e54840);
            let assign41840_e54842: f64 = (assign41840_e54834 + assign41840_e54841);
            let assign41840_e54843: f64 = (locals.var_gf2 * assign41840_e54842);
            let assign41840_e54844: f64 = (assign41840_e54826 - assign41840_e54843);
            (locals.var_sp_s_qc, locals.var_sp_s_qc_dn5, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, ) = (assign41840_e54844, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign41840_e54842) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn5 - locals.var_sp_s_y0_dn5) + locals.var_sp_s_temp1_dn5) + ((locals.var_delta_ns_dn5 * assign41840_e54840) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn5 - locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign41840_e54842) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_y0_dn6) + locals.var_sp_s_temp1_dn6) + ((locals.var_delta_ns_dn6 * assign41840_e54840) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn6 - locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign41840_e54842) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_y0_dn7) + locals.var_sp_s_temp1_dn7) + ((locals.var_delta_ns_dn7 * assign41840_e54840) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn7 - locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign41840_e54842) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_y0_dn8) + locals.var_sp_s_temp1_dn8) + ((locals.var_delta_ns_dn8 * assign41840_e54840) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn8 - locals.var_sp_s_xi0_dn8))))))), );
            locals.var_sp_s_qc_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41850_e54855: f64 = (locals.var_sp_s_delta0 + locals.var_sp_s_temp1);
            let assign41850_e54858: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
            let assign41850_e54859: f64 = (assign41850_e54855 - assign41850_e54858);
            let assign41850_e54860: f64 = (locals.var_gf2 * assign41850_e54859);
            let assign41850_e54861: f64 = (2.0 - assign41850_e54860);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign41850_e54861, (-((locals.var_gf2_dn5 * assign41850_e54859) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn5 + locals.var_sp_s_temp1_dn5) - ((locals.var_delta_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn5)))))), (-((locals.var_gf2_dn6 * assign41850_e54859) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn6 + locals.var_sp_s_temp1_dn6) - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign41850_e54859) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn7 + locals.var_sp_s_temp1_dn7) - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign41850_e54859) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn8 + locals.var_sp_s_temp1_dn8) - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8)))))), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41860_e54870: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
            let assign41860_e54874: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
            let assign41860_e54875: f64 = (2.0 * assign41860_e54874);
            let assign41860_e54876: f64 = (assign41860_e54870 - assign41860_e54875);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign41860_e54876, (((locals.var_sp_s_pc_dn5 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn5)) - (2.0 * ((locals.var_sp_s_qc_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn5)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 != 0.0)) {
            let assign41870_e54884: f64 = (-locals.var_sp_s_y0);
            let assign41870_e54889: f64 = (locals.var_sp_s_temp).sqrt();
            let assign41870_e54890: f64 = (locals.var_sp_s_pc + assign41870_e54889);
            let assign41870_e54891: f64 = (locals.var_sp_s_qc / assign41870_e54890);
            let assign41870_e54892: f64 = (2.0 * assign41870_e54891);
            let assign41870_e54893: f64 = (assign41870_e54884 - assign41870_e54892);
            (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, ) = (assign41870_e54893, ((-locals.var_sp_s_y0_dn5) - (2.0 * (((locals.var_sp_s_qc_dn5 * assign41870_e54890) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn5 + (locals.var_sp_s_temp_dn5 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-locals.var_sp_s_y0_dn6) - (2.0 * (((locals.var_sp_s_qc_dn6 * assign41870_e54890) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-locals.var_sp_s_y0_dn7) - (2.0 * (((locals.var_sp_s_qc_dn7 * assign41870_e54890) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), ((-locals.var_sp_s_y0_dn8) - (2.0 * (((locals.var_sp_s_qc_dn8 * assign41870_e54890) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign41870_e54889))))) / (assign41870_e54890 * assign41870_e54890)))), );
            locals.var_x_s_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign41880_e54905: f64 = (locals.var_gf * 0.7324648775608221);
            let assign41880_e54906: f64 = (1.25 + assign41880_e54905);
            let assign41880_e54907: f64 = (1.0 / assign41880_e54906);
            (locals.var_sp_xg1, locals.var_sp_xg1_dn5, locals.var_sp_xg1_dn6, locals.var_sp_xg1_dn7, locals.var_sp_xg1_dn8, ) = (assign41880_e54907, (-((locals.var_gf_dn5 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((locals.var_gf_dn6 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((locals.var_gf_dn7 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), (-((locals.var_gf_dn8 * 0.7324648775608221) / (assign41880_e54906 * assign41880_e54906))), );
            locals.var_sp_xg1_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign41890_e54917: f64 = (locals.var_xi * 1.25);
            let assign41890_e54919: f64 = (assign41890_e54917 * locals.var_sp_xg1);
            let assign41890_e54921: f64 = (assign41890_e54919 - 1.0);
            let assign41890_e54923: f64 = (assign41890_e54921 * locals.var_sp_xg1);
            (locals.var_sp_s_a_fac, locals.var_sp_s_a_fac_dn5, locals.var_sp_s_a_fac_dn6, locals.var_sp_s_a_fac_dn7, locals.var_sp_s_a_fac_dn8, ) = (assign41890_e54923, (((((locals.var_xi_dn5 * 1.25) * locals.var_sp_xg1) + (assign41890_e54917 * locals.var_sp_xg1_dn5)) * locals.var_sp_xg1) + (assign41890_e54921 * locals.var_sp_xg1_dn5)), (((((locals.var_xi_dn6 * 1.25) * locals.var_sp_xg1) + (assign41890_e54917 * locals.var_sp_xg1_dn6)) * locals.var_sp_xg1) + (assign41890_e54921 * locals.var_sp_xg1_dn6)), (((((locals.var_xi_dn7 * 1.25) * locals.var_sp_xg1) + (assign41890_e54917 * locals.var_sp_xg1_dn7)) * locals.var_sp_xg1) + (assign41890_e54921 * locals.var_sp_xg1_dn7)), (((((locals.var_xi_dn8 * 1.25) * locals.var_sp_xg1) + (assign41890_e54917 * locals.var_sp_xg1_dn8)) * locals.var_sp_xg1) + (assign41890_e54921 * locals.var_sp_xg1_dn8)), );
            locals.var_sp_s_a_fac_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign41900_e54933: f64 = (locals.var_xg * locals.var_inv_xi);
            let assign41900_e54937: f64 = (locals.var_sp_s_a_fac * locals.var_xg);
            let assign41900_e54938: f64 = (1.0 + assign41900_e54937);
            let assign41900_e54939: f64 = (assign41900_e54933 * assign41900_e54938);
            (locals.var_sp_s_xbar, locals.var_sp_s_xbar_dn5, locals.var_sp_s_xbar_dn6, locals.var_sp_s_xbar_dn7, locals.var_sp_s_xbar_dn8, ) = (assign41900_e54939, ((((locals.var_xg_dn5 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn5)) * assign41900_e54938) + (assign41900_e54933 * ((locals.var_sp_s_a_fac_dn5 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn5)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign41900_e54938) + (assign41900_e54933 * ((locals.var_sp_s_a_fac_dn6 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign41900_e54938) + (assign41900_e54933 * ((locals.var_sp_s_a_fac_dn7 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign41900_e54938) + (assign41900_e54933 * ((locals.var_sp_s_a_fac_dn8 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn8)))), );
            locals.var_sp_s_xbar_rv = 0.0;
        }

        let assign41910_e54943: f64 = (-locals.var_sp_s_xbar);
        let assign41910_e54945: f64 = (-230.25850929940458);
        let assign41910_e54946: f64 = if assign41910_e54943 > assign41910_e54945 { 1.0 } else { 0.0 };
        locals.var_guard1185 = assign41910_e54946;
        locals.var_guard1185_rv = 0.0;

        if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1185 != 0.0)) {
            let assign41920_e54955: f64 = (-locals.var_sp_s_xbar);
            let assign41920_e54956: f64 = (assign41920_e54955).exp();
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign41920_e54956, (assign41920_e54956 * (-locals.var_sp_s_xbar_dn5)), (assign41920_e54956 * (-locals.var_sp_s_xbar_dn6)), (assign41920_e54956 * (-locals.var_sp_s_xbar_dn7)), (assign41920_e54956 * (-locals.var_sp_s_xbar_dn8)), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1185 == 0.0)) {
            let assign41930_e54970: f64 = (-230.25850929940458);
            let assign41930_e54972: f64 = (-locals.var_sp_s_xbar);
            let assign41930_e54973: f64 = (assign41930_e54970 - assign41930_e54972);
            let assign41930_e54977: f64 = (-230.25850929940458);
            let assign41930_e54979: f64 = (-locals.var_sp_s_xbar);
            let assign41930_e54980: f64 = (assign41930_e54977 - assign41930_e54979);
            let assign41930_e54983: f64 = (-230.25850929940458);
            let assign41930_e54985: f64 = (-locals.var_sp_s_xbar);
            let assign41930_e54986: f64 = (assign41930_e54983 - assign41930_e54985);
            let assign41930_e54988: f64 = (assign41930_e54986 * 0.3333333333333333);
            let assign41930_e54989: f64 = (1.0 + assign41930_e54988);
            let assign41930_e54990: f64 = (assign41930_e54980 * assign41930_e54989);
            let assign41930_e54991: f64 = (0.5 * assign41930_e54990);
            let assign41930_e54992: f64 = (1.0 + assign41930_e54991);
            let assign41930_e54993: f64 = (assign41930_e54973 * assign41930_e54992);
            let assign41930_e54994: f64 = (1.0 + assign41930_e54993);
            let assign41930_e54995: f64 = (1e-100 / assign41930_e54994);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign41930_e54995, (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn5)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-locals.var_sp_s_xbar_dn5)) * assign41930_e54989) + (assign41930_e54980 * ((-(-locals.var_sp_s_xbar_dn5)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn6)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-locals.var_sp_s_xbar_dn6)) * assign41930_e54989) + (assign41930_e54980 * ((-(-locals.var_sp_s_xbar_dn6)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn7)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-locals.var_sp_s_xbar_dn7)) * assign41930_e54989) + (assign41930_e54980 * ((-(-locals.var_sp_s_xbar_dn7)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn8)) * assign41930_e54992) + (assign41930_e54973 * (0.5 * (((-(-locals.var_sp_s_xbar_dn8)) * assign41930_e54989) + (assign41930_e54980 * ((-(-locals.var_sp_s_xbar_dn8)) * 0.3333333333333333))))))) / (assign41930_e54994 * assign41930_e54994))), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign41940_e55005: f64 = (1.0 - locals.var_sp_s_temp);
            (locals.var_sp_s_w, locals.var_sp_s_w_dn5, locals.var_sp_s_w_dn6, locals.var_sp_s_w_dn7, locals.var_sp_s_w_dn8, ) = (assign41940_e55005, (-locals.var_sp_s_temp_dn5), (-locals.var_sp_s_temp_dn6), (-locals.var_sp_s_temp_dn7), (-locals.var_sp_s_temp_dn8), );
            locals.var_sp_s_w_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign41950_e55016: f64 = (locals.var_gf2 * 0.5);
            let assign41950_e55017: f64 = (locals.var_xg + assign41950_e55016);
            let assign41950_e55022: f64 = (locals.var_gf2 * 0.25);
            let assign41950_e55023: f64 = (locals.var_xg + assign41950_e55022);
            let assign41950_e55025: f64 = (assign41950_e55023 - locals.var_sp_s_w);
            let assign41950_e55026: f64 = (assign41950_e55025).sqrt();
            let assign41950_e55027: f64 = (locals.var_gf * assign41950_e55026);
            let assign41950_e55028: f64 = (assign41950_e55017 - assign41950_e55027);
            (locals.var_sp_s_x1, locals.var_sp_s_x1_dn5, locals.var_sp_s_x1_dn6, locals.var_sp_s_x1_dn7, locals.var_sp_s_x1_dn8, ) = (assign41950_e55028, ((locals.var_xg_dn5 + (locals.var_gf2_dn5 * 0.5)) - ((locals.var_gf_dn5 * assign41950_e55026) + (locals.var_gf * (((locals.var_xg_dn5 + (locals.var_gf2_dn5 * 0.25)) - locals.var_sp_s_w_dn5) / (2.0 * assign41950_e55026))))), ((locals.var_xg_dn6 + (locals.var_gf2_dn6 * 0.5)) - ((locals.var_gf_dn6 * assign41950_e55026) + (locals.var_gf * (((locals.var_xg_dn6 + (locals.var_gf2_dn6 * 0.25)) - locals.var_sp_s_w_dn6) / (2.0 * assign41950_e55026))))), ((locals.var_xg_dn7 + (locals.var_gf2_dn7 * 0.5)) - ((locals.var_gf_dn7 * assign41950_e55026) + (locals.var_gf * (((locals.var_xg_dn7 + (locals.var_gf2_dn7 * 0.25)) - locals.var_sp_s_w_dn7) / (2.0 * assign41950_e55026))))), ((locals.var_xg_dn8 + (locals.var_gf2_dn8 * 0.5)) - ((locals.var_gf_dn8 * assign41950_e55026) + (locals.var_gf * (((locals.var_xg_dn8 + (locals.var_gf2_dn8 * 0.25)) - locals.var_sp_s_w_dn8) / (2.0 * assign41950_e55026))))), );
            locals.var_sp_s_x1_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign41960_e55038: f64 = (locals.var_xn_s + 3.0);
            (locals.var_sp_s_bx, locals.var_sp_s_bx_dn5, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8, ) = (assign41960_e55038, locals.var_xn_s_dn5, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8, );
            locals.var_sp_s_bx_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign41970_e55049: f64 = (locals.var_sp_s_x1 + locals.var_sp_s_bx);
            let assign41970_e55052: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
            let assign41970_e55055: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
            let assign41970_e55056: f64 = (assign41970_e55052 * assign41970_e55055);
            let assign41970_e55058: f64 = (assign41970_e55056 + 5.0);
            let assign41970_e55059: f64 = (assign41970_e55058).sqrt();
            let assign41970_e55060: f64 = (assign41970_e55049 - assign41970_e55059);
            let assign41970_e55061: f64 = (0.5 * assign41970_e55060);
            let assign41970_e55066: f64 = (locals.var_sp_s_bx * locals.var_sp_s_bx);
            let assign41970_e55068: f64 = (assign41970_e55066 + 5.0);
            let assign41970_e55069: f64 = (assign41970_e55068).sqrt();
            let assign41970_e55070: f64 = (locals.var_sp_s_bx - assign41970_e55069);
            let assign41970_e55071: f64 = (0.5 * assign41970_e55070);
            let assign41970_e55072: f64 = (assign41970_e55061 - assign41970_e55071);
            (locals.var_sp_s_eta, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, ) = (assign41970_e55072, ((0.5 * ((locals.var_sp_s_x1_dn5 + locals.var_sp_s_bx_dn5) - ((((locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5) * assign41970_e55055) + (assign41970_e55052 * (locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5))) / (2.0 * assign41970_e55059)))) - (0.5 * (locals.var_sp_s_bx_dn5 - (((locals.var_sp_s_bx_dn5 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn5)) / (2.0 * assign41970_e55069))))), ((0.5 * ((locals.var_sp_s_x1_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6) * assign41970_e55055) + (assign41970_e55052 * (locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign41970_e55059)))) - (0.5 * (locals.var_sp_s_bx_dn6 - (((locals.var_sp_s_bx_dn6 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn6)) / (2.0 * assign41970_e55069))))), ((0.5 * ((locals.var_sp_s_x1_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7) * assign41970_e55055) + (assign41970_e55052 * (locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign41970_e55059)))) - (0.5 * (locals.var_sp_s_bx_dn7 - (((locals.var_sp_s_bx_dn7 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn7)) / (2.0 * assign41970_e55069))))), ((0.5 * ((locals.var_sp_s_x1_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8) * assign41970_e55055) + (assign41970_e55052 * (locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign41970_e55059)))) - (0.5 * (locals.var_sp_s_bx_dn8 - (((locals.var_sp_s_bx_dn8 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn8)) / (2.0 * assign41970_e55069))))), );
            locals.var_sp_s_eta_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign41980_e55082: f64 = (locals.var_xg - locals.var_sp_s_eta);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign41980_e55082, (locals.var_xg_dn5 - locals.var_sp_s_eta_dn5), (locals.var_xg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_xg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_xg_dn8 - locals.var_sp_s_eta_dn8), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign41990_e55091: f64 = (-locals.var_sp_s_eta);
            let assign41990_e55092: f64 = (assign41990_e55091).exp();
            (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, ) = (assign41990_e55092, (assign41990_e55092 * (-locals.var_sp_s_eta_dn5)), (assign41990_e55092 * (-locals.var_sp_s_eta_dn6)), (assign41990_e55092 * (-locals.var_sp_s_eta_dn7)), (assign41990_e55092 * (-locals.var_sp_s_eta_dn8)), );
            locals.var_sp_s_temp1_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42000_e55104: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
            let assign42000_e55105: f64 = (2.0 + assign42000_e55104);
            let assign42000_e55106: f64 = (1.0 / assign42000_e55105);
            (locals.var_sp_s_temp2, locals.var_sp_s_temp2_dn5, locals.var_sp_s_temp2_dn6, locals.var_sp_s_temp2_dn7, locals.var_sp_s_temp2_dn8, ) = (assign42000_e55106, (-(((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) / (assign42000_e55105 * assign42000_e55105))), (-(((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) / (assign42000_e55105 * assign42000_e55105))), (-(((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) / (assign42000_e55105 * assign42000_e55105))), (-(((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) / (assign42000_e55105 * assign42000_e55105))), );
            locals.var_sp_s_temp2_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42010_e55116: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
            let assign42010_e55118: f64 = (assign42010_e55116 * locals.var_sp_s_temp2);
            (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, ) = (assign42010_e55118, ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) * locals.var_sp_s_temp2) + (assign42010_e55116 * locals.var_sp_s_temp2_dn5)), ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) * locals.var_sp_s_temp2) + (assign42010_e55116 * locals.var_sp_s_temp2_dn6)), ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) * locals.var_sp_s_temp2) + (assign42010_e55116 * locals.var_sp_s_temp2_dn7)), ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) * locals.var_sp_s_temp2) + (assign42010_e55116 * locals.var_sp_s_temp2_dn8)), );
            locals.var_sp_s_xi0_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42020_e55129: f64 = (locals.var_sp_s_eta * locals.var_sp_s_temp2);
            let assign42020_e55131: f64 = (assign42020_e55129 * locals.var_sp_s_temp2);
            let assign42020_e55132: f64 = (4.0 * assign42020_e55131);
            (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, ) = (assign42020_e55132, (4.0 * ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign42020_e55129 * locals.var_sp_s_temp2_dn5))), (4.0 * ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign42020_e55129 * locals.var_sp_s_temp2_dn6))), (4.0 * ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign42020_e55129 * locals.var_sp_s_temp2_dn7))), (4.0 * ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign42020_e55129 * locals.var_sp_s_temp2_dn8))), );
            locals.var_sp_s_xi1_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42030_e55142: f64 = (8.0 * locals.var_sp_s_temp2);
            let assign42030_e55145: f64 = (12.0 * locals.var_sp_s_xi0);
            let assign42030_e55146: f64 = (assign42030_e55142 - assign42030_e55145);
            let assign42030_e55148: f64 = (assign42030_e55146 * locals.var_sp_s_temp2);
            let assign42030_e55150: f64 = (assign42030_e55148 * locals.var_sp_s_temp2);
            (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, ) = (assign42030_e55150, ((((((8.0 * locals.var_sp_s_temp2_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp2) + (assign42030_e55146 * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign42030_e55148 * locals.var_sp_s_temp2_dn5)), ((((((8.0 * locals.var_sp_s_temp2_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp2) + (assign42030_e55146 * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign42030_e55148 * locals.var_sp_s_temp2_dn6)), ((((((8.0 * locals.var_sp_s_temp2_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp2) + (assign42030_e55146 * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign42030_e55148 * locals.var_sp_s_temp2_dn7)), ((((((8.0 * locals.var_sp_s_temp2_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp2) + (assign42030_e55146 * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign42030_e55148 * locals.var_sp_s_temp2_dn8)), );
            locals.var_sp_s_xi2_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42040_e55161: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
            let assign42040_e55165: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
            let assign42040_e55167: f64 = (assign42040_e55165 - 1.0);
            let assign42040_e55171: f64 = (locals.var_sp_s_eta + 1.0);
            let assign42040_e55173: f64 = (assign42040_e55171 + locals.var_sp_s_xi0);
            let assign42040_e55174: f64 = (locals.var_delta_ns * assign42040_e55173);
            let assign42040_e55175: f64 = (assign42040_e55167 - assign42040_e55174);
            let assign42040_e55176: f64 = (locals.var_gf2 * assign42040_e55175);
            let assign42040_e55177: f64 = (assign42040_e55161 - assign42040_e55176);
            let (assign42040_e55199, assign42040_e55199_d_n5, assign42040_e55199_d_n6, assign42040_e55199_d_n7, assign42040_e55199_d_n8,) = {
    if (1e-40 > assign42040_e55177) {
        (1e-40, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign42040_e55182: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42040_e55186: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
        let assign42040_e55188: f64 = (assign42040_e55186 - 1.0);
        let assign42040_e55192: f64 = (locals.var_sp_s_eta + 1.0);
        let assign42040_e55194: f64 = (assign42040_e55192 + locals.var_sp_s_xi0);
        let assign42040_e55195: f64 = (locals.var_delta_ns * assign42040_e55194);
        let assign42040_e55196: f64 = (assign42040_e55188 - assign42040_e55195);
        let assign42040_e55197: f64 = (locals.var_gf2 * assign42040_e55196);
        let assign42040_e55198: f64 = (assign42040_e55182 - assign42040_e55197);
        (assign42040_e55198, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign42040_e55196) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn5 + locals.var_sp_s_eta_dn5) - ((locals.var_delta_ns_dn5 * assign42040_e55194) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42040_e55196) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn6 + locals.var_sp_s_eta_dn6) - ((locals.var_delta_ns_dn6 * assign42040_e55194) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42040_e55196) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn7 + locals.var_sp_s_eta_dn7) - ((locals.var_delta_ns_dn7 * assign42040_e55194) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42040_e55196) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn8 + locals.var_sp_s_eta_dn8) - ((locals.var_delta_ns_dn8 * assign42040_e55194) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn8 + locals.var_sp_s_xi0_dn8))))))),)
    }
};
            (locals.var_sp_s_a, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, ) = (assign42040_e55199, assign42040_e55199_d_n5, assign42040_e55199_d_n6, assign42040_e55199_d_n7, assign42040_e55199_d_n8, );
            locals.var_sp_s_a_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42050_e55213: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
            let assign42050_e55214: f64 = (locals.var_sp_s_temp1 - assign42050_e55213);
            let assign42050_e55215: f64 = (locals.var_gf2 * assign42050_e55214);
            let assign42050_e55216: f64 = (0.5 * assign42050_e55215);
            let assign42050_e55217: f64 = (1.0 - assign42050_e55216);
            (locals.var_sp_s_b, locals.var_sp_s_b_dn5, locals.var_sp_s_b_dn6, locals.var_sp_s_b_dn7, locals.var_sp_s_b_dn8, ) = (assign42050_e55217, (-(0.5 * ((locals.var_gf2_dn5 * assign42050_e55214) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn5 - ((locals.var_delta_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn5))))))), (-(0.5 * ((locals.var_gf2_dn6 * assign42050_e55214) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn6 - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6))))))), (-(0.5 * ((locals.var_gf2_dn7 * assign42050_e55214) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn7 - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7))))))), (-(0.5 * ((locals.var_gf2_dn8 * assign42050_e55214) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn8 - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8))))))), );
            locals.var_sp_s_b_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42060_e55227: f64 = (2.0 * locals.var_sp_s_temp);
            let assign42060_e55231: f64 = (1.0 - locals.var_sp_s_temp1);
            let assign42060_e55235: f64 = (1.0 + locals.var_sp_s_xi1);
            let assign42060_e55236: f64 = (locals.var_delta_ns * assign42060_e55235);
            let assign42060_e55237: f64 = (assign42060_e55231 - assign42060_e55236);
            let assign42060_e55238: f64 = (locals.var_gf2 * assign42060_e55237);
            let assign42060_e55239: f64 = (assign42060_e55227 + assign42060_e55238);
            (locals.var_sp_s_c, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, ) = (assign42060_e55239, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign42060_e55237) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn5) - ((locals.var_delta_ns_dn5 * assign42060_e55235) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42060_e55237) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn6) - ((locals.var_delta_ns_dn6 * assign42060_e55235) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42060_e55237) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn7) - ((locals.var_delta_ns_dn7 * assign42060_e55235) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42060_e55237) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn8) - ((locals.var_delta_ns_dn8 * assign42060_e55235) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn8)))))), );
            locals.var_sp_s_c_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42070_e55249: f64 = (locals.var_xn_s - locals.var_sp_s_eta);
            let assign42070_e55252: f64 = (locals.var_sp_s_a / locals.var_gf2);
            let assign42070_e55253: f64 = (assign42070_e55252).ln();
            let assign42070_e55254: f64 = (assign42070_e55249 + assign42070_e55253);
            (locals.var_sp_s_tau, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, ) = (assign42070_e55254, ((locals.var_xn_s_dn5 - locals.var_sp_s_eta_dn5) + ((((locals.var_sp_s_a_dn5 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn5)) / (locals.var_gf2 * locals.var_gf2)) / assign42070_e55252)), ((locals.var_xn_s_dn6 - locals.var_sp_s_eta_dn6) + ((((locals.var_sp_s_a_dn6 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn6)) / (locals.var_gf2 * locals.var_gf2)) / assign42070_e55252)), ((locals.var_xn_s_dn7 - locals.var_sp_s_eta_dn7) + ((((locals.var_sp_s_a_dn7 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn7)) / (locals.var_gf2 * locals.var_gf2)) / assign42070_e55252)), ((locals.var_xn_s_dn8 - locals.var_sp_s_eta_dn8) + ((((locals.var_sp_s_a_dn8 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn8)) / (locals.var_gf2 * locals.var_gf2)) / assign42070_e55252)), );
            locals.var_sp_s_tau_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42080_e55264: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
            (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, ) = (assign42080_e55264, (locals.var_sp_s_a_dn5 + locals.var_sp_s_c_dn5), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), );
            locals.var_nu_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42090_e55274: f64 = (locals.var_nu * locals.var_nu);
            let assign42090_e55279: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
            let assign42090_e55280: f64 = (0.5 * assign42090_e55279);
            let assign42090_e55283: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
            let assign42090_e55284: f64 = (assign42090_e55280 - assign42090_e55283);
            let assign42090_e55285: f64 = (locals.var_sp_s_tau * assign42090_e55284);
            let assign42090_e55286: f64 = (assign42090_e55274 + assign42090_e55285);
            (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, ) = (assign42090_e55286, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau_dn5 * assign42090_e55284) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5))) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign42090_e55284) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign42090_e55284) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign42090_e55284) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))), );
            locals.var_mutau_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42100_e55297: f64 = (locals.var_sp_s_a * locals.var_nu);
            let assign42100_e55299: f64 = (assign42100_e55297 * locals.var_sp_s_tau);
            let assign42100_e55303: f64 = (locals.var_nu / locals.var_mutau);
            let assign42100_e55305: f64 = (assign42100_e55303 * locals.var_sp_s_tau);
            let assign42100_e55307: f64 = (assign42100_e55305 * locals.var_sp_s_tau);
            let assign42100_e55309: f64 = (assign42100_e55307 * locals.var_sp_s_c);
            let assign42100_e55312: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
            let assign42100_e55314: f64 = (assign42100_e55312 * 0.3333333333333333);
            let assign42100_e55317: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
            let assign42100_e55318: f64 = (assign42100_e55314 - assign42100_e55317);
            let assign42100_e55319: f64 = (assign42100_e55309 * assign42100_e55318);
            let assign42100_e55320: f64 = (locals.var_mutau + assign42100_e55319);
            let assign42100_e55321: f64 = (assign42100_e55299 / assign42100_e55320);
            let assign42100_e55322: f64 = (locals.var_sp_s_eta + assign42100_e55321);
            (locals.var_sp_s_x0, locals.var_sp_s_x0_dn5, locals.var_sp_s_x0_dn6, locals.var_sp_s_x0_dn7, locals.var_sp_s_x0_dn8, ) = (assign42100_e55322, (locals.var_sp_s_eta_dn5 + (((((((locals.var_sp_s_a_dn5 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn5)) * locals.var_sp_s_tau) + (assign42100_e55297 * locals.var_sp_s_tau_dn5)) * assign42100_e55320) - (assign42100_e55299 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42100_e55303 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_tau) + (assign42100_e55305 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_c) + (assign42100_e55307 * locals.var_sp_s_c_dn5)) * assign42100_e55318) + (assign42100_e55309 * ((((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))))) / (assign42100_e55320 * assign42100_e55320))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign42100_e55297 * locals.var_sp_s_tau_dn6)) * assign42100_e55320) - (assign42100_e55299 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42100_e55303 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign42100_e55305 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign42100_e55307 * locals.var_sp_s_c_dn6)) * assign42100_e55318) + (assign42100_e55309 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))))) / (assign42100_e55320 * assign42100_e55320))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign42100_e55297 * locals.var_sp_s_tau_dn7)) * assign42100_e55320) - (assign42100_e55299 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42100_e55303 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign42100_e55305 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign42100_e55307 * locals.var_sp_s_c_dn7)) * assign42100_e55318) + (assign42100_e55309 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))))) / (assign42100_e55320 * assign42100_e55320))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign42100_e55297 * locals.var_sp_s_tau_dn8)) * assign42100_e55320) - (assign42100_e55299 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42100_e55303 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign42100_e55305 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign42100_e55307 * locals.var_sp_s_c_dn8)) * assign42100_e55318) + (assign42100_e55309 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))))) / (assign42100_e55320 * assign42100_e55320))), );
            locals.var_sp_s_x0_rv = 0.0;
        }

        let assign42110_e55327: f64 = if locals.var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1186 = assign42110_e55327;
        locals.var_guard1186_rv = 0.0;

        if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 != 0.0)) {
            let assign42120_e55336: f64 = (locals.var_sp_s_x0).exp();
            (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, ) = (assign42120_e55336, (assign42120_e55336 * locals.var_sp_s_x0_dn5), (assign42120_e55336 * locals.var_sp_s_x0_dn6), (assign42120_e55336 * locals.var_sp_s_x0_dn7), (assign42120_e55336 * locals.var_sp_s_x0_dn8), );
            locals.var_sp_s_delta0_rv = 0.0;
        }

        if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 != 0.0)) {
            let assign42130_e55348: f64 = (1.0 / locals.var_sp_s_delta0);
            (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, ) = (assign42130_e55348, (-(locals.var_sp_s_delta0_dn5 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), );
            locals.var_sp_s_delta1_rv = 0.0;
        }

        if (((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 != 0.0)) {
            let assign42140_e55360: f64 = (locals.var_delta_ns * locals.var_sp_s_delta0);
            (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, ) = (assign42140_e55360, ((locals.var_delta_ns_dn5 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn5)), ((locals.var_delta_ns_dn6 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn6)), ((locals.var_delta_ns_dn7 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn7)), ((locals.var_delta_ns_dn8 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn8)), );
            locals.var_sp_s_delta0_rv = 0.0;
        }

        let assign42150_e55366: f64 = (locals.var_xn_s - 230.25850929940458);
        let assign42150_e55367: f64 = if locals.var_sp_s_x0 > assign42150_e55366 { 1.0 } else { 0.0 };
        locals.var_guard1187 = assign42150_e55367;
        locals.var_guard1187_rv = 0.0;

        if ((((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 == 0.0)) && (locals.var_guard1187 != 0.0)) {
            let assign42160_e55380: f64 = (locals.var_sp_s_x0 - locals.var_xn_s);
            let assign42160_e55381: f64 = (assign42160_e55380).exp();
            (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, ) = (assign42160_e55381, (assign42160_e55381 * (locals.var_sp_s_x0_dn5 - locals.var_xn_s_dn5)), (assign42160_e55381 * (locals.var_sp_s_x0_dn6 - locals.var_xn_s_dn6)), (assign42160_e55381 * (locals.var_sp_s_x0_dn7 - locals.var_xn_s_dn7)), (assign42160_e55381 * (locals.var_sp_s_x0_dn8 - locals.var_xn_s_dn8)), );
            locals.var_sp_s_delta0_rv = 0.0;
        }

        if ((((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 == 0.0)) && (locals.var_guard1187 != 0.0)) {
            let assign42170_e55396: f64 = (locals.var_delta_ns / locals.var_sp_s_delta0);
            (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, ) = (assign42170_e55396, (((locals.var_delta_ns_dn5 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn5)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn6 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn6)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn7 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn7)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn8 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn8)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), );
            locals.var_sp_s_delta1_rv = 0.0;
        }

        if ((((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 == 0.0)) && (locals.var_guard1187 == 0.0)) {
            let assign42180_e55414: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
            let assign42180_e55416: f64 = (assign42180_e55414 - 230.25850929940458);
            let assign42180_e55421: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
            let assign42180_e55423: f64 = (assign42180_e55421 - 230.25850929940458);
            let assign42180_e55427: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
            let assign42180_e55429: f64 = (assign42180_e55427 - 230.25850929940458);
            let assign42180_e55431: f64 = (assign42180_e55429 * 0.3333333333333333);
            let assign42180_e55432: f64 = (1.0 + assign42180_e55431);
            let assign42180_e55433: f64 = (assign42180_e55423 * assign42180_e55432);
            let assign42180_e55434: f64 = (0.5 * assign42180_e55433);
            let assign42180_e55435: f64 = (1.0 + assign42180_e55434);
            let assign42180_e55436: f64 = (assign42180_e55416 * assign42180_e55435);
            let assign42180_e55437: f64 = (1.0 + assign42180_e55436);
            let assign42180_e55438: f64 = (1e-100 / assign42180_e55437);
            (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, ) = (assign42180_e55438, (-((1e-100 * (((locals.var_xn_s_dn5 - locals.var_sp_s_x0_dn5) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((locals.var_xn_s_dn5 - locals.var_sp_s_x0_dn5) * assign42180_e55432) + (assign42180_e55423 * ((locals.var_xn_s_dn5 - locals.var_sp_s_x0_dn5) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * assign42180_e55432) + (assign42180_e55423 * ((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * assign42180_e55432) + (assign42180_e55423 * ((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), (-((1e-100 * (((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * assign42180_e55435) + (assign42180_e55416 * (0.5 * (((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * assign42180_e55432) + (assign42180_e55423 * ((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign42180_e55437 * assign42180_e55437))), );
            locals.var_sp_s_delta0_rv = 0.0;
        }

        if ((((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1186 == 0.0)) && (locals.var_guard1187 == 0.0)) {
            let assign42190_e55456: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
            let assign42190_e55461: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
            let assign42190_e55465: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
            let assign42190_e55467: f64 = (assign42190_e55465 * 0.3333333333333333);
            let assign42190_e55468: f64 = (1.0 + assign42190_e55467);
            let assign42190_e55469: f64 = (assign42190_e55461 * assign42190_e55468);
            let assign42190_e55470: f64 = (0.5 * assign42190_e55469);
            let assign42190_e55471: f64 = (1.0 + assign42190_e55470);
            let assign42190_e55472: f64 = (assign42190_e55456 * assign42190_e55471);
            let assign42190_e55473: f64 = (1.0 + assign42190_e55472);
            let assign42190_e55474: f64 = (1e-100 / assign42190_e55473);
            (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, ) = (assign42190_e55474, (-((1e-100 * ((locals.var_sp_s_x0_dn5 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((locals.var_sp_s_x0_dn5 * assign42190_e55468) + (assign42190_e55461 * (locals.var_sp_s_x0_dn5 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((locals.var_sp_s_x0_dn6 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((locals.var_sp_s_x0_dn6 * assign42190_e55468) + (assign42190_e55461 * (locals.var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((locals.var_sp_s_x0_dn7 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((locals.var_sp_s_x0_dn7 * assign42190_e55468) + (assign42190_e55461 * (locals.var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), (-((1e-100 * ((locals.var_sp_s_x0_dn8 * assign42190_e55471) + (assign42190_e55456 * (0.5 * ((locals.var_sp_s_x0_dn8 * assign42190_e55468) + (assign42190_e55461 * (locals.var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign42190_e55473 * assign42190_e55473))), );
            locals.var_sp_s_delta1_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42200_e55486: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
            let assign42200_e55487: f64 = (2.0 + assign42200_e55486);
            let assign42200_e55488: f64 = (1.0 / assign42200_e55487);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign42200_e55488, (-(((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) / (assign42200_e55487 * assign42200_e55487))), (-(((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) / (assign42200_e55487 * assign42200_e55487))), (-(((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) / (assign42200_e55487 * assign42200_e55487))), (-(((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) / (assign42200_e55487 * assign42200_e55487))), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42210_e55498: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
            let assign42210_e55500: f64 = (assign42210_e55498 * locals.var_sp_s_temp);
            (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, ) = (assign42210_e55500, ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) * locals.var_sp_s_temp) + (assign42210_e55498 * locals.var_sp_s_temp_dn5)), ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) * locals.var_sp_s_temp) + (assign42210_e55498 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) * locals.var_sp_s_temp) + (assign42210_e55498 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) * locals.var_sp_s_temp) + (assign42210_e55498 * locals.var_sp_s_temp_dn8)), );
            locals.var_sp_s_xi0_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42220_e55511: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_temp);
            let assign42220_e55513: f64 = (assign42220_e55511 * locals.var_sp_s_temp);
            let assign42220_e55514: f64 = (4.0 * assign42220_e55513);
            (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, ) = (assign42220_e55514, (4.0 * ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign42220_e55511 * locals.var_sp_s_temp_dn5))), (4.0 * ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42220_e55511 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42220_e55511 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42220_e55511 * locals.var_sp_s_temp_dn8))), );
            locals.var_sp_s_xi1_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_14(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42230_e55524: f64 = (8.0 * locals.var_sp_s_temp);
            let assign42230_e55527: f64 = (12.0 * locals.var_sp_s_xi0);
            let assign42230_e55528: f64 = (assign42230_e55524 - assign42230_e55527);
            let assign42230_e55530: f64 = (assign42230_e55528 * locals.var_sp_s_temp);
            let assign42230_e55532: f64 = (assign42230_e55530 * locals.var_sp_s_temp);
            (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, ) = (assign42230_e55532, ((((((8.0 * locals.var_sp_s_temp_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp) + (assign42230_e55528 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign42230_e55530 * locals.var_sp_s_temp_dn5)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign42230_e55528 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42230_e55530 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign42230_e55528 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42230_e55530 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign42230_e55528 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42230_e55530 * locals.var_sp_s_temp_dn8)), );
            locals.var_sp_s_xi2_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42240_e55542: f64 = (locals.var_xg - locals.var_sp_s_x0);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign42240_e55542, (locals.var_xg_dn5 - locals.var_sp_s_x0_dn5), (locals.var_xg_dn6 - locals.var_sp_s_x0_dn6), (locals.var_xg_dn7 - locals.var_sp_s_x0_dn7), (locals.var_xg_dn8 - locals.var_sp_s_x0_dn8), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42250_e55552: f64 = (2.0 * locals.var_sp_s_temp);
            let assign42250_e55556: f64 = (1.0 - locals.var_sp_s_delta1);
            let assign42250_e55558: f64 = (assign42250_e55556 + locals.var_sp_s_delta0);
            let assign42250_e55562: f64 = (1.0 + locals.var_sp_s_xi1);
            let assign42250_e55563: f64 = (locals.var_delta_ns * assign42250_e55562);
            let assign42250_e55564: f64 = (assign42250_e55558 - assign42250_e55563);
            let assign42250_e55565: f64 = (locals.var_gf2 * assign42250_e55564);
            let assign42250_e55566: f64 = (assign42250_e55552 + assign42250_e55565);
            (locals.var_sp_s_pc, locals.var_sp_s_pc_dn5, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, ) = (assign42250_e55566, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign42250_e55564) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_ns_dn5 * assign42250_e55562) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42250_e55564) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * assign42250_e55562) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42250_e55564) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * assign42250_e55562) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42250_e55564) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * assign42250_e55562) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn8)))))), );
            locals.var_sp_s_pc_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42260_e55576: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
            let assign42260_e55580: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_x0);
            let assign42260_e55582: f64 = (assign42260_e55580 - 1.0);
            let assign42260_e55584: f64 = (assign42260_e55582 + locals.var_sp_s_delta0);
            let assign42260_e55588: f64 = (locals.var_sp_s_x0 + 1.0);
            let assign42260_e55590: f64 = (assign42260_e55588 + locals.var_sp_s_xi0);
            let assign42260_e55591: f64 = (locals.var_delta_ns * assign42260_e55590);
            let assign42260_e55592: f64 = (assign42260_e55584 - assign42260_e55591);
            let assign42260_e55593: f64 = (locals.var_gf2 * assign42260_e55592);
            let assign42260_e55594: f64 = (assign42260_e55576 - assign42260_e55593);
            (locals.var_sp_s_qc, locals.var_sp_s_qc_dn5, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, ) = (assign42260_e55594, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign42260_e55592) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_x0_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_ns_dn5 * assign42260_e55590) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42260_e55592) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_x0_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * assign42260_e55590) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42260_e55592) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_x0_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * assign42260_e55590) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42260_e55592) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_x0_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * assign42260_e55590) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn8 + locals.var_sp_s_xi0_dn8))))))), );
            locals.var_sp_s_qc_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42270_e55606: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_delta0);
            let assign42270_e55609: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
            let assign42270_e55610: f64 = (assign42270_e55606 - assign42270_e55609);
            let assign42270_e55611: f64 = (locals.var_gf2 * assign42270_e55610);
            let assign42270_e55612: f64 = (2.0 - assign42270_e55611);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign42270_e55612, (-((locals.var_gf2_dn5 * assign42270_e55610) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_ns_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn5)))))), (-((locals.var_gf2_dn6 * assign42270_e55610) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign42270_e55610) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign42270_e55610) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8)))))), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42280_e55622: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
            let assign42280_e55626: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
            let assign42280_e55627: f64 = (2.0 * assign42280_e55626);
            let assign42280_e55628: f64 = (assign42280_e55622 - assign42280_e55627);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign42280_e55628, (((locals.var_sp_s_pc_dn5 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn5)) - (2.0 * ((locals.var_sp_s_qc_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn5)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1182 == 0.0) && (locals.var_guard1183 == 0.0)) {
            let assign42290_e55641: f64 = (locals.var_sp_s_temp).sqrt();
            let assign42290_e55642: f64 = (locals.var_sp_s_pc + assign42290_e55641);
            let assign42290_e55643: f64 = (locals.var_sp_s_qc / assign42290_e55642);
            let assign42290_e55644: f64 = (2.0 * assign42290_e55643);
            let assign42290_e55645: f64 = (locals.var_sp_s_x0 + assign42290_e55644);
            (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, ) = (assign42290_e55645, (locals.var_sp_s_x0_dn5 + (2.0 * (((locals.var_sp_s_qc_dn5 * assign42290_e55642) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn5 + (locals.var_sp_s_temp_dn5 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (locals.var_sp_s_x0_dn6 + (2.0 * (((locals.var_sp_s_qc_dn6 * assign42290_e55642) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (locals.var_sp_s_x0_dn7 + (2.0 * (((locals.var_sp_s_qc_dn7 * assign42290_e55642) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), (locals.var_sp_s_x0_dn8 + (2.0 * (((locals.var_sp_s_qc_dn8 * assign42290_e55642) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign42290_e55641))))) / (assign42290_e55642 * assign42290_e55642)))), );
            locals.var_x_s_rv = 0.0;
        }

        (locals.var_xi1s, locals.var_xi1s_dn5, locals.var_xi1s_dn6, locals.var_xi1s_dn7, locals.var_xi1s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_xi1s_rv = 0.0;

        (locals.var_xi2s, locals.var_xi2s_dn5, locals.var_xi2s_dn6, locals.var_xi2s_dn7, locals.var_xi2s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_xi2s_rv = 0.0;

        (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_delta_1s_rv = 0.0;

        (locals.var_es, locals.var_es_dn5, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_es_rv = 0.0;

        (locals.var_ds, locals.var_ds_dn5, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_ds_rv = 0.0;

        (locals.var_ps, locals.var_ps_dn5, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_ps_rv = 0.0;

        (locals.var_sqs, locals.var_sqs_dn5, locals.var_sqs_dn6, locals.var_sqs_dn7, locals.var_sqs_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_sqs_rv = 0.0;

        (locals.var_alphas, locals.var_alphas_dn5, locals.var_alphas_dn6, locals.var_alphas_dn7, locals.var_alphas_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_alphas_rv = 0.0;

        (locals.var_rxcor, locals.var_rxcor_dn5, locals.var_rxcor_dn6, locals.var_rxcor_dn7, locals.var_rxcor_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_rxcor_rv = 0.0;

        let assign42390_e55659: f64 = (locals.var_xg - locals.var_x_s);
        (locals.var_xgs, locals.var_xgs_dn5, locals.var_xgs_dn6, locals.var_xgs_dn7, locals.var_xgs_dn8, ) = (assign42390_e55659, (locals.var_xg_dn5 - locals.var_x_s_dn5), (locals.var_xg_dn6 - locals.var_x_s_dn6), (locals.var_xg_dn7 - locals.var_x_s_dn7), (locals.var_xg_dn8 - locals.var_x_s_dn8), );
        locals.var_xgs_rv = 0.0;

        (locals.var_qis, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qis_rv = 0.0;

        let assign42410_e55663: f64 = (locals.var_phit1 * locals.var_xgs);
        (locals.var_qbs, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, ) = (assign42410_e55663, ((locals.var_phit1_dn5 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn5)), ((locals.var_phit1_dn6 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn6)), ((locals.var_phit1_dn7 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn7)), ((locals.var_phit1_dn8 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn8)), );
        locals.var_qbs_rv = 0.0;

        (locals.var_rhob, locals.var_rhob_dn5, locals.var_rhob_dn6, locals.var_rhob_dn7, locals.var_rhob_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_rhob_rv = 0.0;

        (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_rhog_rv = 0.0;

        (locals.var_gmobs, locals.var_gmobs_dn5, locals.var_gmobs_dn6, locals.var_gmobs_dn7, locals.var_gmobs_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_gmobs_rv = 0.0;

        (locals.var_xitsb, locals.var_xitsb_dn5, locals.var_xitsb_dn6, locals.var_xitsb_dn7, locals.var_xitsb_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_xitsb_rv = 0.0;

        (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_factheta_rv = 0.0;

        let assign42470_e55671: f64 = if locals.var_xg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1188 = assign42470_e55671;
        locals.var_guard1188_rv = 0.0;

        if (locals.var_guard1188 != 0.0) {
            let assign42480_e55677: f64 = (locals.var_x_s * locals.var_x_s);
            let assign42480_e55678: f64 = (2.0 + assign42480_e55677);
            let assign42480_e55679: f64 = (1.0 / assign42480_e55678);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign42480_e55679, (-(((locals.var_x_s_dn5 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn5)) / (assign42480_e55678 * assign42480_e55678))), (-(((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) / (assign42480_e55678 * assign42480_e55678))), (-(((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) / (assign42480_e55678 * assign42480_e55678))), (-(((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) / (assign42480_e55678 * assign42480_e55678))), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (locals.var_guard1188 != 0.0) {
            let assign42490_e55685: f64 = (locals.var_x_s * locals.var_x_s);
            let assign42490_e55687: f64 = (assign42490_e55685 * locals.var_temp__blk936);
            (locals.var_xi0s, locals.var_xi0s_dn5, locals.var_xi0s_dn6, locals.var_xi0s_dn7, locals.var_xi0s_dn8, ) = (assign42490_e55687, ((((locals.var_x_s_dn5 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn5)) * locals.var_temp__blk936) + (assign42490_e55685 * locals.var_temp__blk936_dn5)), ((((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) * locals.var_temp__blk936) + (assign42490_e55685 * locals.var_temp__blk936_dn6)), ((((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) * locals.var_temp__blk936) + (assign42490_e55685 * locals.var_temp__blk936_dn7)), ((((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) * locals.var_temp__blk936) + (assign42490_e55685 * locals.var_temp__blk936_dn8)), );
            locals.var_xi0s_rv = 0.0;
        }

        if (locals.var_guard1188 != 0.0) {
            let assign42500_e55694: f64 = (locals.var_x_s * locals.var_temp__blk936);
            let assign42500_e55696: f64 = (assign42500_e55694 * locals.var_temp__blk936);
            let assign42500_e55697: f64 = (4.0 * assign42500_e55696);
            (locals.var_xi1s, locals.var_xi1s_dn5, locals.var_xi1s_dn6, locals.var_xi1s_dn7, locals.var_xi1s_dn8, ) = (assign42500_e55697, (4.0 * ((((locals.var_x_s_dn5 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign42500_e55694 * locals.var_temp__blk936_dn5))), (4.0 * ((((locals.var_x_s_dn6 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign42500_e55694 * locals.var_temp__blk936_dn6))), (4.0 * ((((locals.var_x_s_dn7 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign42500_e55694 * locals.var_temp__blk936_dn7))), (4.0 * ((((locals.var_x_s_dn8 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign42500_e55694 * locals.var_temp__blk936_dn8))), );
            locals.var_xi1s_rv = 0.0;
        }

        if (locals.var_guard1188 != 0.0) {
            let assign42510_e55703: f64 = (8.0 * locals.var_temp__blk936);
            let assign42510_e55706: f64 = (12.0 * locals.var_xi0s);
            let assign42510_e55707: f64 = (assign42510_e55703 - assign42510_e55706);
            let assign42510_e55709: f64 = (assign42510_e55707 * locals.var_temp__blk936);
            let assign42510_e55711: f64 = (assign42510_e55709 * locals.var_temp__blk936);
            (locals.var_xi2s, locals.var_xi2s_dn5, locals.var_xi2s_dn6, locals.var_xi2s_dn7, locals.var_xi2s_dn8, ) = (assign42510_e55711, ((((((8.0 * locals.var_temp__blk936_dn5) - (12.0 * locals.var_xi0s_dn5)) * locals.var_temp__blk936) + (assign42510_e55707 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign42510_e55709 * locals.var_temp__blk936_dn5)), ((((((8.0 * locals.var_temp__blk936_dn6) - (12.0 * locals.var_xi0s_dn6)) * locals.var_temp__blk936) + (assign42510_e55707 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign42510_e55709 * locals.var_temp__blk936_dn6)), ((((((8.0 * locals.var_temp__blk936_dn7) - (12.0 * locals.var_xi0s_dn7)) * locals.var_temp__blk936) + (assign42510_e55707 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign42510_e55709 * locals.var_temp__blk936_dn7)), ((((((8.0 * locals.var_temp__blk936_dn8) - (12.0 * locals.var_xi0s_dn8)) * locals.var_temp__blk936) + (assign42510_e55707 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign42510_e55709 * locals.var_temp__blk936_dn8)), );
            locals.var_xi2s_rv = 0.0;
        }

        if (locals.var_guard1188 != 0.0) {
            (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_delta_1s_rv = 0.0;
        }

        let assign42530_e55720: f64 = if locals.var_x_s < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1189 = assign42530_e55720;
        locals.var_guard1189_rv = 0.0;

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1189 != 0.0)) {
            let assign42540_e55725: f64 = (locals.var_x_s).exp();
            (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, ) = (assign42540_e55725, (assign42540_e55725 * locals.var_x_s_dn5), (assign42540_e55725 * locals.var_x_s_dn6), (assign42540_e55725 * locals.var_x_s_dn7), (assign42540_e55725 * locals.var_x_s_dn8), );
            locals.var_delta_1s_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1189 != 0.0)) {
            let assign42550_e55733: f64 = (1.0 / locals.var_delta_1s);
            (locals.var_es, locals.var_es_dn5, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, ) = (assign42550_e55733, (-(locals.var_delta_1s_dn5 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn6 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn7 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn8 / (locals.var_delta_1s * locals.var_delta_1s))), );
            locals.var_es_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1189 != 0.0)) {
            let assign42560_e55741: f64 = (locals.var_delta_ns * locals.var_delta_1s);
            (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, ) = (assign42560_e55741, ((locals.var_delta_ns_dn5 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn5)), ((locals.var_delta_ns_dn6 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn6)), ((locals.var_delta_ns_dn7 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn7)), ((locals.var_delta_ns_dn8 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn8)), );
            locals.var_delta_1s_rv = 0.0;
        }

        let assign42570_e55747: f64 = (locals.var_xn_s - 230.25850929940458);
        let assign42570_e55748: f64 = if locals.var_x_s > assign42570_e55747 { 1.0 } else { 0.0 };
        locals.var_guard1190 = assign42570_e55748;
        locals.var_guard1190_rv = 0.0;

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1189 == 0.0)) && (locals.var_guard1190 != 0.0)) {
            let assign42580_e55757: f64 = (locals.var_x_s - locals.var_xn_s);
            let assign42580_e55758: f64 = (assign42580_e55757).exp();
            (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, ) = (assign42580_e55758, (assign42580_e55758 * (locals.var_x_s_dn5 - locals.var_xn_s_dn5)), (assign42580_e55758 * (locals.var_x_s_dn6 - locals.var_xn_s_dn6)), (assign42580_e55758 * (locals.var_x_s_dn7 - locals.var_xn_s_dn7)), (assign42580_e55758 * (locals.var_x_s_dn8 - locals.var_xn_s_dn8)), );
            locals.var_delta_1s_rv = 0.0;
        }

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1189 == 0.0)) && (locals.var_guard1190 != 0.0)) {
            let assign42590_e55769: f64 = (locals.var_delta_ns / locals.var_delta_1s);
            (locals.var_es, locals.var_es_dn5, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, ) = (assign42590_e55769, (((locals.var_delta_ns_dn5 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn5)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn6 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn6)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn7 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn7)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn8 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn8)) / (locals.var_delta_1s * locals.var_delta_1s)), );
            locals.var_es_rv = 0.0;
        }

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1189 == 0.0)) && (locals.var_guard1190 == 0.0)) {
            let assign42600_e55783: f64 = (locals.var_xn_s - locals.var_x_s);
            let assign42600_e55785: f64 = (assign42600_e55783 - 230.25850929940458);
            let assign42600_e55790: f64 = (locals.var_xn_s - locals.var_x_s);
            let assign42600_e55792: f64 = (assign42600_e55790 - 230.25850929940458);
            let assign42600_e55796: f64 = (locals.var_xn_s - locals.var_x_s);
            let assign42600_e55798: f64 = (assign42600_e55796 - 230.25850929940458);
            let assign42600_e55800: f64 = (assign42600_e55798 * 0.3333333333333333);
            let assign42600_e55801: f64 = (1.0 + assign42600_e55800);
            let assign42600_e55802: f64 = (assign42600_e55792 * assign42600_e55801);
            let assign42600_e55803: f64 = (0.5 * assign42600_e55802);
            let assign42600_e55804: f64 = (1.0 + assign42600_e55803);
            let assign42600_e55805: f64 = (assign42600_e55785 * assign42600_e55804);
            let assign42600_e55806: f64 = (1.0 + assign42600_e55805);
            let assign42600_e55807: f64 = (1e-100 / assign42600_e55806);
            (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, ) = (assign42600_e55807, (-((1e-100 * (((locals.var_xn_s_dn5 - locals.var_x_s_dn5) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((locals.var_xn_s_dn5 - locals.var_x_s_dn5) * assign42600_e55801) + (assign42600_e55792 * ((locals.var_xn_s_dn5 - locals.var_x_s_dn5) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * assign42600_e55801) + (assign42600_e55792 * ((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * assign42600_e55801) + (assign42600_e55792 * ((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), (-((1e-100 * (((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * assign42600_e55804) + (assign42600_e55785 * (0.5 * (((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * assign42600_e55801) + (assign42600_e55792 * ((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * 0.3333333333333333))))))) / (assign42600_e55806 * assign42600_e55806))), );
            locals.var_delta_1s_rv = 0.0;
        }

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1189 == 0.0)) && (locals.var_guard1190 == 0.0)) {
            let assign42610_e55821: f64 = (locals.var_x_s - 230.25850929940458);
            let assign42610_e55826: f64 = (locals.var_x_s - 230.25850929940458);
            let assign42610_e55830: f64 = (locals.var_x_s - 230.25850929940458);
            let assign42610_e55832: f64 = (assign42610_e55830 * 0.3333333333333333);
            let assign42610_e55833: f64 = (1.0 + assign42610_e55832);
            let assign42610_e55834: f64 = (assign42610_e55826 * assign42610_e55833);
            let assign42610_e55835: f64 = (0.5 * assign42610_e55834);
            let assign42610_e55836: f64 = (1.0 + assign42610_e55835);
            let assign42610_e55837: f64 = (assign42610_e55821 * assign42610_e55836);
            let assign42610_e55838: f64 = (1.0 + assign42610_e55837);
            let assign42610_e55839: f64 = (1e-100 / assign42610_e55838);
            (locals.var_es, locals.var_es_dn5, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, ) = (assign42610_e55839, (-((1e-100 * ((locals.var_x_s_dn5 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((locals.var_x_s_dn5 * assign42610_e55833) + (assign42610_e55826 * (locals.var_x_s_dn5 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((locals.var_x_s_dn6 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((locals.var_x_s_dn6 * assign42610_e55833) + (assign42610_e55826 * (locals.var_x_s_dn6 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((locals.var_x_s_dn7 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((locals.var_x_s_dn7 * assign42610_e55833) + (assign42610_e55826 * (locals.var_x_s_dn7 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), (-((1e-100 * ((locals.var_x_s_dn8 * assign42610_e55836) + (assign42610_e55821 * (0.5 * ((locals.var_x_s_dn8 * assign42610_e55833) + (assign42610_e55826 * (locals.var_x_s_dn8 * 0.3333333333333333))))))) / (assign42610_e55838 * assign42610_e55838))), );
            locals.var_es_rv = 0.0;
        }

        if (locals.var_guard1188 != 0.0) {
            let assign42620_e55847: f64 = (locals.var_x_s + 1.0);
            let assign42620_e55849: f64 = (assign42620_e55847 + locals.var_xi0s);
            let assign42620_e55850: f64 = (locals.var_delta_ns * assign42620_e55849);
            let assign42620_e55851: f64 = (locals.var_delta_1s - assign42620_e55850);
            (locals.var_ds, locals.var_ds_dn5, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, ) = (assign42620_e55851, (locals.var_delta_1s_dn5 - ((locals.var_delta_ns_dn5 * assign42620_e55849) + (locals.var_delta_ns * (locals.var_x_s_dn5 + locals.var_xi0s_dn5)))), (locals.var_delta_1s_dn6 - ((locals.var_delta_ns_dn6 * assign42620_e55849) + (locals.var_delta_ns * (locals.var_x_s_dn6 + locals.var_xi0s_dn6)))), (locals.var_delta_1s_dn7 - ((locals.var_delta_ns_dn7 * assign42620_e55849) + (locals.var_delta_ns * (locals.var_x_s_dn7 + locals.var_xi0s_dn7)))), (locals.var_delta_1s_dn8 - ((locals.var_delta_ns_dn8 * assign42620_e55849) + (locals.var_delta_ns * (locals.var_x_s_dn8 + locals.var_xi0s_dn8)))), );
            locals.var_ds_rv = 0.0;
        }

        let assign42630_e55856: f64 = if locals.var_x_s < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1191 = assign42630_e55856;
        locals.var_guard1191_rv = 0.0;

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 != 0.0)) {
            let assign42640_e55863: f64 = (locals.var_x_s * locals.var_x_s);
            let assign42640_e55870: f64 = (0.25 * locals.var_x_s);
            let assign42640_e55871: f64 = (1.0 - assign42640_e55870);
            let assign42640_e55872: f64 = (locals.var_x_s * assign42640_e55871);
            let assign42640_e55873: f64 = (0.3333333333333333 * assign42640_e55872);
            let assign42640_e55874: f64 = (1.0 - assign42640_e55873);
            let assign42640_e55875: f64 = (assign42640_e55863 * assign42640_e55874);
            let assign42640_e55876: f64 = (0.5 * assign42640_e55875);
            (locals.var_ps, locals.var_ps_dn5, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8, ) = (assign42640_e55876, (0.5 * ((((locals.var_x_s_dn5 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn5)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((locals.var_x_s_dn5 * assign42640_e55871) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn5))))))))), (0.5 * ((((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((locals.var_x_s_dn6 * assign42640_e55871) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn6))))))))), (0.5 * ((((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((locals.var_x_s_dn7 * assign42640_e55871) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn7))))))))), (0.5 * ((((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((locals.var_x_s_dn8 * assign42640_e55871) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn8))))))))), );
            locals.var_ps_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 != 0.0)) {
            let assign42650_e55885: f64 = (locals.var_delta_ns * locals.var_x_s);
            let assign42650_e55887: f64 = (assign42650_e55885 * locals.var_x_s);
            let assign42650_e55889: f64 = (assign42650_e55887 * locals.var_x_s);
            let assign42650_e55893: f64 = (1.75 * locals.var_x_s);
            let assign42650_e55894: f64 = (1.0 + assign42650_e55893);
            let assign42650_e55895: f64 = (assign42650_e55889 * assign42650_e55894);
            let assign42650_e55896: f64 = (0.16666666666666666 * assign42650_e55895);
            (locals.var_ds, locals.var_ds_dn5, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, ) = (assign42650_e55896, (0.16666666666666666 * ((((((((locals.var_delta_ns_dn5 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn5)) * locals.var_x_s) + (assign42650_e55885 * locals.var_x_s_dn5)) * locals.var_x_s) + (assign42650_e55887 * locals.var_x_s_dn5)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * locals.var_x_s_dn5)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn6 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn6)) * locals.var_x_s) + (assign42650_e55885 * locals.var_x_s_dn6)) * locals.var_x_s) + (assign42650_e55887 * locals.var_x_s_dn6)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * locals.var_x_s_dn6)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn7 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn7)) * locals.var_x_s) + (assign42650_e55885 * locals.var_x_s_dn7)) * locals.var_x_s) + (assign42650_e55887 * locals.var_x_s_dn7)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * locals.var_x_s_dn7)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn8 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn8)) * locals.var_x_s) + (assign42650_e55885 * locals.var_x_s_dn8)) * locals.var_x_s) + (assign42650_e55887 * locals.var_x_s_dn8)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * locals.var_x_s_dn8)))), );
            locals.var_ds_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 != 0.0)) {
            let assign42660_e55908: f64 = (0.25 * locals.var_x_s);
            let assign42660_e55909: f64 = (1.0 - assign42660_e55908);
            let assign42660_e55910: f64 = (locals.var_x_s * assign42660_e55909);
            let assign42660_e55911: f64 = (0.3333333333333333 * assign42660_e55910);
            let assign42660_e55912: f64 = (1.0 - assign42660_e55911);
            let assign42660_e55913: f64 = (assign42660_e55912).sqrt();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign42660_e55913, ((-(0.3333333333333333 * ((locals.var_x_s_dn5 * assign42660_e55909) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn5)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((locals.var_x_s_dn6 * assign42660_e55909) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn6)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((locals.var_x_s_dn7 * assign42660_e55909) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn7)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((locals.var_x_s_dn8 * assign42660_e55909) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn8)))))) / (2.0 * assign42660_e55913)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 != 0.0)) {
            let assign42670_e55922: f64 = (locals.var_x_s * locals.var_temp__blk936);
            let assign42670_e55923: f64 = (0.7071067811865475 * assign42670_e55922);
            (locals.var_sqs, locals.var_sqs_dn5, locals.var_sqs_dn6, locals.var_sqs_dn7, locals.var_sqs_dn8, ) = (assign42670_e55923, (0.7071067811865475 * ((locals.var_x_s_dn5 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_s_dn6 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_s_dn7 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_s_dn8 * locals.var_temp__blk936) + (locals.var_x_s * locals.var_temp__blk936_dn8))), );
            locals.var_sqs_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 != 0.0)) {
            let assign42680_e55935: f64 = (0.5 * locals.var_x_s);
            let assign42680_e55936: f64 = (1.0 - assign42680_e55935);
            let assign42680_e55940: f64 = (locals.var_x_s * locals.var_x_s);
            let assign42680_e55941: f64 = (0.16666666666666666 * assign42680_e55940);
            let assign42680_e55942: f64 = (assign42680_e55936 + assign42680_e55941);
            let assign42680_e55943: f64 = (locals.var_gf * assign42680_e55942);
            let assign42680_e55945: f64 = (assign42680_e55943 / locals.var_temp__blk936);
            let assign42680_e55946: f64 = (0.7071067811865475 * assign42680_e55945);
            let assign42680_e55947: f64 = (1.0 + assign42680_e55946);
            (locals.var_alphas, locals.var_alphas_dn5, locals.var_alphas_dn6, locals.var_alphas_dn7, locals.var_alphas_dn8, ) = (assign42680_e55947, (0.7071067811865475 * (((((locals.var_gf_dn5 * assign42680_e55942) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn5)) + (0.16666666666666666 * ((locals.var_x_s_dn5 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn5)))))) * locals.var_temp__blk936) - (assign42680_e55943 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf_dn6 * assign42680_e55942) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn6)) + (0.16666666666666666 * ((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)))))) * locals.var_temp__blk936) - (assign42680_e55943 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf_dn7 * assign42680_e55942) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn7)) + (0.16666666666666666 * ((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)))))) * locals.var_temp__blk936) - (assign42680_e55943 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf_dn8 * assign42680_e55942) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn8)) + (0.16666666666666666 * ((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)))))) * locals.var_temp__blk936) - (assign42680_e55943 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), );
            locals.var_alphas_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 == 0.0)) {
            let assign42690_e55956: f64 = (locals.var_x_s - 1.0);
            let assign42690_e55958: f64 = (assign42690_e55956 + locals.var_es);
            (locals.var_ps, locals.var_ps_dn5, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8, ) = (assign42690_e55958, (locals.var_x_s_dn5 + locals.var_es_dn5), (locals.var_x_s_dn6 + locals.var_es_dn6), (locals.var_x_s_dn7 + locals.var_es_dn7), (locals.var_x_s_dn8 + locals.var_es_dn8), );
            locals.var_ps_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 == 0.0)) {
            let assign42700_e55966: f64 = (locals.var_ps).sqrt();
            (locals.var_sqs, locals.var_sqs_dn5, locals.var_sqs_dn6, locals.var_sqs_dn7, locals.var_sqs_dn8, ) = (assign42700_e55966, (locals.var_ps_dn5 / (2.0 * assign42700_e55966)), (locals.var_ps_dn6 / (2.0 * assign42700_e55966)), (locals.var_ps_dn7 / (2.0 * assign42700_e55966)), (locals.var_ps_dn8 / (2.0 * assign42700_e55966)), );
            locals.var_sqs_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1191 == 0.0)) {
            let assign42710_e55978: f64 = (1.0 - locals.var_es);
            let assign42710_e55979: f64 = (locals.var_gf * assign42710_e55978);
            let assign42710_e55981: f64 = (assign42710_e55979 / locals.var_sqs);
            let assign42710_e55982: f64 = (0.5 * assign42710_e55981);
            let assign42710_e55983: f64 = (1.0 + assign42710_e55982);
            (locals.var_alphas, locals.var_alphas_dn5, locals.var_alphas_dn6, locals.var_alphas_dn7, locals.var_alphas_dn8, ) = (assign42710_e55983, (0.5 * (((((locals.var_gf_dn5 * assign42710_e55978) + (locals.var_gf * (-locals.var_es_dn5))) * locals.var_sqs) - (assign42710_e55979 * locals.var_sqs_dn5)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn6 * assign42710_e55978) + (locals.var_gf * (-locals.var_es_dn6))) * locals.var_sqs) - (assign42710_e55979 * locals.var_sqs_dn6)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn7 * assign42710_e55978) + (locals.var_gf * (-locals.var_es_dn7))) * locals.var_sqs) - (assign42710_e55979 * locals.var_sqs_dn7)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn8 * assign42710_e55978) + (locals.var_gf * (-locals.var_es_dn8))) * locals.var_sqs) - (assign42710_e55979 * locals.var_sqs_dn8)) / (locals.var_sqs * locals.var_sqs))), );
            locals.var_alphas_rv = 0.0;
        }

        if (locals.var_guard1188 != 0.0) {
            let assign42720_e55990: f64 = (0.2 * locals.var_xcor_t);
            let assign42720_e55992: f64 = (assign42720_e55990 * locals.var_vsbx);
            let assign42720_e55993: f64 = (1.0 + assign42720_e55992);
            let assign42720_e55997: f64 = (locals.var_xcor_t * locals.var_vsbx);
            let assign42720_e55998: f64 = (1.0 + assign42720_e55997);
            let assign42720_e55999: f64 = (assign42720_e55993 / assign42720_e55998);
            (locals.var_rxcor, locals.var_rxcor_dn5, locals.var_rxcor_dn6, locals.var_rxcor_dn7, locals.var_rxcor_dn8, ) = (assign42720_e55999, ((((assign42720_e55990 * locals.var_vsbx_dn5) * assign42720_e55998) - (assign42720_e55993 * (locals.var_xcor_t * locals.var_vsbx_dn5))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * locals.var_vsbx_dn6) * assign42720_e55998) - (assign42720_e55993 * (locals.var_xcor_t * locals.var_vsbx_dn6))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * locals.var_vsbx_dn7) * assign42720_e55998) - (assign42720_e55993 * (locals.var_xcor_t * locals.var_vsbx_dn7))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * locals.var_vsbx_dn8) * assign42720_e55998) - (assign42720_e55993 * (locals.var_xcor_t * locals.var_vsbx_dn8))) / (assign42720_e55998 * assign42720_e55998)), );
            locals.var_rxcor_rv = 0.0;
        }

        let assign42730_e56004: f64 = if locals.var_ds > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1192 = assign42730_e56004;
        locals.var_guard1192_rv = 0.0;

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign42740_e56011: f64 = (locals.var_ps + locals.var_ds);
            let assign42740_e56012: f64 = (assign42740_e56011).sqrt();
            let assign42740_e56013: f64 = (locals.var_gf * assign42740_e56012);
            (locals.var_xgs, locals.var_xgs_dn5, locals.var_xgs_dn6, locals.var_xgs_dn7, locals.var_xgs_dn8, ) = (assign42740_e56013, ((locals.var_gf_dn5 * assign42740_e56012) + (locals.var_gf * ((locals.var_ps_dn5 + locals.var_ds_dn5) / (2.0 * assign42740_e56012)))), ((locals.var_gf_dn6 * assign42740_e56012) + (locals.var_gf * ((locals.var_ps_dn6 + locals.var_ds_dn6) / (2.0 * assign42740_e56012)))), ((locals.var_gf_dn7 * assign42740_e56012) + (locals.var_gf * ((locals.var_ps_dn7 + locals.var_ds_dn7) / (2.0 * assign42740_e56012)))), ((locals.var_gf_dn8 * assign42740_e56012) + (locals.var_gf * ((locals.var_ps_dn8 + locals.var_ds_dn8) / (2.0 * assign42740_e56012)))), );
            locals.var_xgs_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign42750_e56021: f64 = (locals.var_gf2 * locals.var_ds);
            let assign42750_e56023: f64 = (assign42750_e56021 * locals.var_phit1);
            let assign42750_e56027: f64 = (locals.var_gf * locals.var_sqs);
            let assign42750_e56028: f64 = (locals.var_xgs + assign42750_e56027);
            let assign42750_e56029: f64 = (assign42750_e56023 / assign42750_e56028);
            (locals.var_qis, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, ) = (assign42750_e56029, (((((((locals.var_gf2_dn5 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn5)) * locals.var_phit1) + (assign42750_e56021 * locals.var_phit1_dn5)) * assign42750_e56028) - (assign42750_e56023 * (locals.var_xgs_dn5 + ((locals.var_gf_dn5 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn5))))) / (assign42750_e56028 * assign42750_e56028)), (((((((locals.var_gf2_dn6 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn6)) * locals.var_phit1) + (assign42750_e56021 * locals.var_phit1_dn6)) * assign42750_e56028) - (assign42750_e56023 * (locals.var_xgs_dn6 + ((locals.var_gf_dn6 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn6))))) / (assign42750_e56028 * assign42750_e56028)), (((((((locals.var_gf2_dn7 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn7)) * locals.var_phit1) + (assign42750_e56021 * locals.var_phit1_dn7)) * assign42750_e56028) - (assign42750_e56023 * (locals.var_xgs_dn7 + ((locals.var_gf_dn7 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn7))))) / (assign42750_e56028 * assign42750_e56028)), (((((((locals.var_gf2_dn8 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn8)) * locals.var_phit1) + (assign42750_e56021 * locals.var_phit1_dn8)) * assign42750_e56028) - (assign42750_e56023 * (locals.var_xgs_dn8 + ((locals.var_gf_dn8 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn8))))) / (assign42750_e56028 * assign42750_e56028)), );
            locals.var_qis_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign42760_e56037: f64 = (locals.var_sqs * locals.var_gf);
            let assign42760_e56039: f64 = (assign42760_e56037 * locals.var_phit1);
            (locals.var_qbs, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, ) = (assign42760_e56039, ((((locals.var_sqs_dn5 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn5)) * locals.var_phit1) + (assign42760_e56037 * locals.var_phit1_dn5)), ((((locals.var_sqs_dn6 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn6)) * locals.var_phit1) + (assign42760_e56037 * locals.var_phit1_dn6)), ((((locals.var_sqs_dn7 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn7)) * locals.var_phit1) + (assign42760_e56037 * locals.var_phit1_dn7)), ((((locals.var_sqs_dn8 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn8)) * locals.var_phit1) + (assign42760_e56037 * locals.var_phit1_dn8)), );
            locals.var_qbs_rv = 0.0;
        }

        let assign42770_e56044: f64 = if locals.var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1193 = assign42770_e56044;
        locals.var_guard1193_rv = 0.0;

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1193 != 0.0)) {
            let assign42780_e56054: f64 = (locals.var_rsb_i * locals.var_vsbx);
            let assign42780_e56055: f64 = (1.0 - assign42780_e56054);
            let assign42780_e56056: f64 = (1.0 / assign42780_e56055);
            (locals.var_rhob, locals.var_rhob_dn5, locals.var_rhob_dn6, locals.var_rhob_dn7, locals.var_rhob_dn8, ) = (assign42780_e56056, (-((-(locals.var_rsb_i * locals.var_vsbx_dn5)) / (assign42780_e56055 * assign42780_e56055))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn6)) / (assign42780_e56055 * assign42780_e56055))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn7)) / (assign42780_e56055 * assign42780_e56055))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn8)) / (assign42780_e56055 * assign42780_e56055))), );
            locals.var_rhob_rv = 0.0;
        }

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1193 == 0.0)) {
            let assign42790_e56068: f64 = (locals.var_rsb_i * locals.var_vsbx);
            let assign42790_e56069: f64 = (1.0 + assign42790_e56068);
            (locals.var_rhob, locals.var_rhob_dn5, locals.var_rhob_dn6, locals.var_rhob_dn7, locals.var_rhob_dn8, ) = (assign42790_e56069, (locals.var_rsb_i * locals.var_vsbx_dn5), (locals.var_rsb_i * locals.var_vsbx_dn6), (locals.var_rsb_i * locals.var_vsbx_dn7), (locals.var_rsb_i * locals.var_vsbx_dn8), );
            locals.var_rhob_rv = 0.0;
        }

        let assign42800_e56074: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1194 = assign42800_e56074;
        locals.var_guard1194_rv = 0.0;

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1194 != 0.0)) {
            let assign42810_e56083: f64 = (locals.var_rsg_i * locals.var_qis);
            let assign42810_e56084: f64 = (1.0 - assign42810_e56083);
            (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, ) = (assign42810_e56084, (-(locals.var_rsg_i * locals.var_qis_dn5)), (-(locals.var_rsg_i * locals.var_qis_dn6)), (-(locals.var_rsg_i * locals.var_qis_dn7)), (-(locals.var_rsg_i * locals.var_qis_dn8)), );
            locals.var_rhog_rv = 0.0;
        }

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1194 == 0.0)) {
            let assign42820_e56097: f64 = (locals.var_rsg_i * locals.var_qis);
            let assign42820_e56098: f64 = (1.0 + assign42820_e56097);
            let assign42820_e56099: f64 = (1.0 / assign42820_e56098);
            (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, ) = (assign42820_e56099, (-((locals.var_rsg_i * locals.var_qis_dn5) / (assign42820_e56098 * assign42820_e56098))), (-((locals.var_rsg_i * locals.var_qis_dn6) / (assign42820_e56098 * assign42820_e56098))), (-((locals.var_rsg_i * locals.var_qis_dn7) / (assign42820_e56098 * assign42820_e56098))), (-((locals.var_rsg_i * locals.var_qis_dn8) / (assign42820_e56098 * assign42820_e56098))), );
            locals.var_rhog_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign42830_e56107: f64 = (locals.var_ther_i * locals.var_rhob);
            let assign42830_e56109: f64 = (assign42830_e56107 * locals.var_rhog);
            let assign42830_e56111: f64 = (assign42830_e56109 * locals.var_qis);
            (locals.var_gr, locals.var_gr_dn5, locals.var_gr_dn6, locals.var_gr_dn7, locals.var_gr_dn8, ) = (assign42830_e56111, (((((locals.var_ther_i * locals.var_rhob_dn5) * locals.var_rhog) + (assign42830_e56107 * locals.var_rhog_dn5)) * locals.var_qis) + (assign42830_e56109 * locals.var_qis_dn5)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign42830_e56107 * locals.var_rhog_dn6)) * locals.var_qis) + (assign42830_e56109 * locals.var_qis_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign42830_e56107 * locals.var_rhog_dn7)) * locals.var_qis) + (assign42830_e56109 * locals.var_qis_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign42830_e56107 * locals.var_rhog_dn8)) * locals.var_qis) + (assign42830_e56109 * locals.var_qis_dn8)), );
            locals.var_gr_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign42840_e56121: f64 = (locals.var_eta_mu * locals.var_qis);
            let assign42840_e56122: f64 = (locals.var_qbs + assign42840_e56121);
            let assign42840_e56123: f64 = (locals.var_e_eff0 * assign42840_e56122);
            (locals.var_eeffs, locals.var_eeffs_dn5, locals.var_eeffs_dn6, locals.var_eeffs_dn7, locals.var_eeffs_dn8, ) = (assign42840_e56123, (locals.var_e_eff0 * (locals.var_qbs_dn5 + (locals.var_eta_mu * locals.var_qis_dn5))), (locals.var_e_eff0 * (locals.var_qbs_dn6 + (locals.var_eta_mu * locals.var_qis_dn6))), (locals.var_e_eff0 * (locals.var_qbs_dn7 + (locals.var_eta_mu * locals.var_qis_dn7))), (locals.var_e_eff0 * (locals.var_qbs_dn8 + (locals.var_eta_mu * locals.var_qis_dn8))), );
            locals.var_eeffs_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign42850_e56132: f64 = (locals.var_ps + locals.var_ds);
            let assign42850_e56134: f64 = (assign42850_e56132 + 1e-14);
            let assign42850_e56135: f64 = (locals.var_ps / assign42850_e56134);
            let assign42850_e56136: f64 = (assign42850_e56135).ln();
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign42850_e56136, ((((locals.var_ps_dn5 * assign42850_e56134) - (locals.var_ps * (locals.var_ps_dn5 + locals.var_ds_dn5))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((locals.var_ps_dn6 * assign42850_e56134) - (locals.var_ps * (locals.var_ps_dn6 + locals.var_ds_dn6))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((locals.var_ps_dn7 * assign42850_e56134) - (locals.var_ps * (locals.var_ps_dn7 + locals.var_ds_dn7))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((locals.var_ps_dn8 * assign42850_e56134) - (locals.var_ps * (locals.var_ps_dn8 + locals.var_ds_dn8))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign42860_e56144: f64 = (locals.var_eeffs * locals.var_mue_t);
            let assign42860_e56146: f64 = (assign42860_e56144).powf(locals.var_themu_t);
            let assign42860_e56150: f64 = (0.5 * locals.var_thecs_t);
            let assign42860_e56152: f64 = (assign42860_e56150 * locals.var_temp1);
            let assign42860_e56153: f64 = (assign42860_e56152).exp();
            let assign42860_e56154: f64 = (locals.var_cs_t * assign42860_e56153);
            let assign42860_e56155: f64 = (assign42860_e56146 + assign42860_e56154);
            (locals.var_mutmp, locals.var_mutmp_dn5, locals.var_mutmp_dn6, locals.var_mutmp_dn7, locals.var_mutmp_dn8, ) = (assign42860_e56155, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign42860_e56144).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn5 * locals.var_mue_t))) } } else { (assign42860_e56146 * (locals.var_themu_t * ((locals.var_eeffs_dn5 * locals.var_mue_t) / assign42860_e56144))) } + (locals.var_cs_t * (assign42860_e56153 * (assign42860_e56150 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign42860_e56144).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn6 * locals.var_mue_t))) } } else { (assign42860_e56146 * (locals.var_themu_t * ((locals.var_eeffs_dn6 * locals.var_mue_t) / assign42860_e56144))) } + (locals.var_cs_t * (assign42860_e56153 * (assign42860_e56150 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign42860_e56144).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn7 * locals.var_mue_t))) } } else { (assign42860_e56146 * (locals.var_themu_t * ((locals.var_eeffs_dn7 * locals.var_mue_t) / assign42860_e56144))) } + (locals.var_cs_t * (assign42860_e56153 * (assign42860_e56150 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign42860_e56144).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn8 * locals.var_mue_t))) } } else { (assign42860_e56146 * (locals.var_themu_t * ((locals.var_eeffs_dn8 * locals.var_mue_t) / assign42860_e56144))) } + (locals.var_cs_t * (assign42860_e56153 * (assign42860_e56150 * locals.var_temp1_dn8)))), );
            locals.var_mutmp_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign42870_e56163: f64 = (1.0 + locals.var_mutmp);
            let assign42870_e56165: f64 = (assign42870_e56163 + locals.var_gr);
            let assign42870_e56167: f64 = (assign42870_e56165 * locals.var_rxcor);
            (locals.var_gmobs, locals.var_gmobs_dn5, locals.var_gmobs_dn6, locals.var_gmobs_dn7, locals.var_gmobs_dn8, ) = (assign42870_e56167, (((locals.var_mutmp_dn5 + locals.var_gr_dn5) * locals.var_rxcor) + (assign42870_e56165 * locals.var_rxcor_dn5)), (((locals.var_mutmp_dn6 + locals.var_gr_dn6) * locals.var_rxcor) + (assign42870_e56165 * locals.var_rxcor_dn6)), (((locals.var_mutmp_dn7 + locals.var_gr_dn7) * locals.var_rxcor) + (assign42870_e56165 * locals.var_rxcor_dn7)), (((locals.var_mutmp_dn8 + locals.var_gr_dn8) * locals.var_rxcor) + (assign42870_e56165 * locals.var_rxcor_dn8)), );
            locals.var_gmobs_rv = 0.0;
        }

        let assign42880_e56172: f64 = if locals.var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1195 = assign42880_e56172;
        locals.var_guard1195_rv = 0.0;

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1195 != 0.0)) {
            let assign42890_e56182: f64 = (locals.var_thesatb_i * locals.var_vsbx);
            let assign42890_e56183: f64 = (1.0 - assign42890_e56182);
            let assign42890_e56184: f64 = (1.0 / assign42890_e56183);
            (locals.var_xitsb, locals.var_xitsb_dn5, locals.var_xitsb_dn6, locals.var_xitsb_dn7, locals.var_xitsb_dn8, ) = (assign42890_e56184, (-((-(locals.var_thesatb_i * locals.var_vsbx_dn5)) / (assign42890_e56183 * assign42890_e56183))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn6)) / (assign42890_e56183 * assign42890_e56183))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn7)) / (assign42890_e56183 * assign42890_e56183))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn8)) / (assign42890_e56183 * assign42890_e56183))), );
            locals.var_xitsb_rv = 0.0;
        }

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1195 == 0.0)) {
            let assign42900_e56196: f64 = (locals.var_thesatb_i * locals.var_vsbx);
            let assign42900_e56197: f64 = (1.0 + assign42900_e56196);
            (locals.var_xitsb, locals.var_xitsb_dn5, locals.var_xitsb_dn6, locals.var_xitsb_dn7, locals.var_xitsb_dn8, ) = (assign42900_e56197, (locals.var_thesatb_i * locals.var_vsbx_dn5), (locals.var_thesatb_i * locals.var_vsbx_dn6), (locals.var_thesatb_i * locals.var_vsbx_dn7), (locals.var_thesatb_i * locals.var_vsbx_dn8), );
            locals.var_xitsb_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign42910_e56205: f64 = (locals.var_qis * locals.var_xitsb);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign42910_e56205, ((locals.var_qis_dn5 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn5)), ((locals.var_qis_dn6 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn6)), ((locals.var_qis_dn7 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn7)), ((locals.var_qis_dn8 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn8)), );
            locals.var_temp2_rv = 0.0;
        }

        if ((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) {
            let assign42920_e56214: f64 = (locals.var_thesatt_i + locals.var_temp2);
            let assign42920_e56215: f64 = (locals.var_temp2 / assign42920_e56214);
            (locals.var_wsat, locals.var_wsat_dn5, locals.var_wsat_dn6, locals.var_wsat_dn7, locals.var_wsat_dn8, ) = (assign42920_e56215, (((locals.var_temp2_dn5 * assign42920_e56214) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign42920_e56214 * assign42920_e56214)), (((locals.var_temp2_dn6 * assign42920_e56214) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign42920_e56214 * assign42920_e56214)), (((locals.var_temp2_dn7 * assign42920_e56214) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign42920_e56214 * assign42920_e56214)), (((locals.var_temp2_dn8 * assign42920_e56214) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign42920_e56214 * assign42920_e56214)), );
            locals.var_wsat_rv = 0.0;
        }

        let assign42930_e56220: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1196 = assign42930_e56220;
        locals.var_guard1196_rv = 0.0;

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1196 != 0.0)) {
            let assign42940_e56230: f64 = (locals.var_thesatg_i * locals.var_wsat);
            let assign42940_e56231: f64 = (1.0 - assign42940_e56230);
            let assign42940_e56232: f64 = (1.0 / assign42940_e56231);
            (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, ) = (assign42940_e56232, (-((-(locals.var_thesatg_i * locals.var_wsat_dn5)) / (assign42940_e56231 * assign42940_e56231))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn6)) / (assign42940_e56231 * assign42940_e56231))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn7)) / (assign42940_e56231 * assign42940_e56231))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn8)) / (assign42940_e56231 * assign42940_e56231))), );
            locals.var_factheta_rv = 0.0;
        }

        if (((locals.var_guard1188 != 0.0) && (locals.var_guard1192 != 0.0)) && (locals.var_guard1196 == 0.0)) {
            let assign42950_e56244: f64 = (locals.var_thesatg_i * locals.var_wsat);
            let assign42950_e56245: f64 = (1.0 + assign42950_e56244);
            (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, ) = (assign42950_e56245, (locals.var_thesatg_i * locals.var_wsat_dn5), (locals.var_thesatg_i * locals.var_wsat_dn6), (locals.var_thesatg_i * locals.var_wsat_dn7), (locals.var_thesatg_i * locals.var_wsat_dn8), );
            locals.var_factheta_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_15(
        locals: &mut StampLocals,
    ) {
        (locals.var_vgb1_dc, locals.var_vgb1_dc_dn5, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8, ) = (locals.var_vgb1, locals.var_vgb1_dn5, locals.var_vgb1_dn6, locals.var_vgb1_dn7, locals.var_vgb1_dn8, );
        locals.var_vgb1_dc_rv = 0.0;

        (locals.var_vsbx_dc, locals.var_vsbx_dc_dn5, locals.var_vsbx_dc_dn6, locals.var_vsbx_dc_dn7, locals.var_vsbx_dc_dn8, ) = (locals.var_vsbx, locals.var_vsbx_dn5, locals.var_vsbx_dn6, locals.var_vsbx_dn7, locals.var_vsbx_dn8, );
        locals.var_vsbx_dc_rv = 0.0;

        (locals.var_phit1_dc, locals.var_phit1_dc_dn5, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8, ) = (locals.var_phit1, locals.var_phit1_dn5, locals.var_phit1_dn6, locals.var_phit1_dn7, locals.var_phit1_dn8, );
        locals.var_phit1_dc_rv = 0.0;

        (locals.var_inv_phit1_dc, locals.var_inv_phit1_dc_dn5, locals.var_inv_phit1_dc_dn6, locals.var_inv_phit1_dc_dn7, locals.var_inv_phit1_dc_dn8, ) = (locals.var_inv_phit1, locals.var_inv_phit1_dn5, locals.var_inv_phit1_dn6, locals.var_inv_phit1_dn7, locals.var_inv_phit1_dn8, );
        locals.var_inv_phit1_dc_rv = 0.0;

        (locals.var_gf_dc, locals.var_gf_dc_dn5, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8, ) = (locals.var_gf, locals.var_gf_dn5, locals.var_gf_dn6, locals.var_gf_dn7, locals.var_gf_dn8, );
        locals.var_gf_dc_rv = 0.0;

        (locals.var_gf2_dc, locals.var_gf2_dc_dn5, locals.var_gf2_dc_dn6, locals.var_gf2_dc_dn7, locals.var_gf2_dc_dn8, ) = (locals.var_gf2, locals.var_gf2_dn5, locals.var_gf2_dn6, locals.var_gf2_dn7, locals.var_gf2_dn8, );
        locals.var_gf2_dc_rv = 0.0;

        (locals.var_inv_gf2_dc, locals.var_inv_gf2_dc_dn5, locals.var_inv_gf2_dc_dn6, locals.var_inv_gf2_dc_dn7, locals.var_inv_gf2_dc_dn8, ) = (locals.var_inv_gf2, locals.var_inv_gf2_dn5, locals.var_inv_gf2_dn6, locals.var_inv_gf2_dn7, locals.var_inv_gf2_dn8, );
        locals.var_inv_gf2_dc_rv = 0.0;

        (locals.var_xg_dc, locals.var_xg_dc_dn5, locals.var_xg_dc_dn6, locals.var_xg_dc_dn7, locals.var_xg_dc_dn8, ) = (locals.var_xg, locals.var_xg_dn5, locals.var_xg_dn6, locals.var_xg_dn7, locals.var_xg_dn8, );
        locals.var_xg_dc_rv = 0.0;

        (locals.var_xno_s_dc, locals.var_xno_s_dc_dn5, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8, ) = (locals.var_xno_s, locals.var_xno_s_dn5, locals.var_xno_s_dn6, locals.var_xno_s_dn7, locals.var_xno_s_dn8, );
        locals.var_xno_s_dc_rv = 0.0;

        (locals.var_xn_s_dc, locals.var_xn_s_dc_dn5, locals.var_xn_s_dc_dn6, locals.var_xn_s_dc_dn7, locals.var_xn_s_dc_dn8, ) = (locals.var_xn_s, locals.var_xn_s_dn5, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8, );
        locals.var_xn_s_dc_rv = 0.0;

        (locals.var_xi_dc, locals.var_xi_dc_dn5, locals.var_xi_dc_dn6, locals.var_xi_dc_dn7, locals.var_xi_dc_dn8, ) = (locals.var_xi, locals.var_xi_dn5, locals.var_xi_dn6, locals.var_xi_dn7, locals.var_xi_dn8, );
        locals.var_xi_dc_rv = 0.0;

        (locals.var_margin_dc, locals.var_margin_dc_dn5, locals.var_margin_dc_dn6, locals.var_margin_dc_dn7, locals.var_margin_dc_dn8, ) = (locals.var_margin, locals.var_margin_dn5, locals.var_margin_dn6, locals.var_margin_dn7, locals.var_margin_dn8, );
        locals.var_margin_dc_rv = 0.0;

        (locals.var_inv_xi_dc, locals.var_inv_xi_dc_dn5, locals.var_inv_xi_dc_dn6, locals.var_inv_xi_dc_dn7, locals.var_inv_xi_dc_dn8, ) = (locals.var_inv_xi, locals.var_inv_xi_dn5, locals.var_inv_xi_dn6, locals.var_inv_xi_dn7, locals.var_inv_xi_dn8, );
        locals.var_inv_xi_dc_rv = 0.0;

        (locals.var_sp_s_x1_dc, locals.var_sp_s_x1_dc_dn5, locals.var_sp_s_x1_dc_dn6, locals.var_sp_s_x1_dc_dn7, locals.var_sp_s_x1_dc_dn8, ) = (locals.var_sp_s_x1, locals.var_sp_s_x1_dn5, locals.var_sp_s_x1_dn6, locals.var_sp_s_x1_dn7, locals.var_sp_s_x1_dn8, );
        locals.var_sp_s_x1_dc_rv = 0.0;

        (locals.var_delta_ns_dc, locals.var_delta_ns_dc_dn5, locals.var_delta_ns_dc_dn6, locals.var_delta_ns_dc_dn7, locals.var_delta_ns_dc_dn8, ) = (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, );
        locals.var_delta_ns_dc_rv = 0.0;

        (locals.var_x_s_dc, locals.var_x_s_dc_dn5, locals.var_x_s_dc_dn6, locals.var_x_s_dc_dn7, locals.var_x_s_dc_dn8, ) = (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, );
        locals.var_x_s_dc_rv = 0.0;

        (locals.var_xi1s_dc, locals.var_xi1s_dc_dn5, locals.var_xi1s_dc_dn6, locals.var_xi1s_dc_dn7, locals.var_xi1s_dc_dn8, ) = (locals.var_xi1s, locals.var_xi1s_dn5, locals.var_xi1s_dn6, locals.var_xi1s_dn7, locals.var_xi1s_dn8, );
        locals.var_xi1s_dc_rv = 0.0;

        (locals.var_xi2s_dc, locals.var_xi2s_dc_dn5, locals.var_xi2s_dc_dn6, locals.var_xi2s_dc_dn7, locals.var_xi2s_dc_dn8, ) = (locals.var_xi2s, locals.var_xi2s_dn5, locals.var_xi2s_dn6, locals.var_xi2s_dn7, locals.var_xi2s_dn8, );
        locals.var_xi2s_dc_rv = 0.0;

        (locals.var_delta_1s_dc, locals.var_delta_1s_dc_dn5, locals.var_delta_1s_dc_dn6, locals.var_delta_1s_dc_dn7, locals.var_delta_1s_dc_dn8, ) = (locals.var_delta_1s, locals.var_delta_1s_dn5, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, );
        locals.var_delta_1s_dc_rv = 0.0;

        (locals.var_es_dc, locals.var_es_dc_dn5, locals.var_es_dc_dn6, locals.var_es_dc_dn7, locals.var_es_dc_dn8, ) = (locals.var_es, locals.var_es_dn5, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, );
        locals.var_es_dc_rv = 0.0;

        (locals.var_ps_dc, locals.var_ps_dc_dn5, locals.var_ps_dc_dn6, locals.var_ps_dc_dn7, locals.var_ps_dc_dn8, ) = (locals.var_ps, locals.var_ps_dn5, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8, );
        locals.var_ps_dc_rv = 0.0;

        (locals.var_ds_dc, locals.var_ds_dc_dn5, locals.var_ds_dc_dn6, locals.var_ds_dc_dn7, locals.var_ds_dc_dn8, ) = (locals.var_ds, locals.var_ds_dn5, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, );
        locals.var_ds_dc_rv = 0.0;

        (locals.var_sqs_dc, locals.var_sqs_dc_dn5, locals.var_sqs_dc_dn6, locals.var_sqs_dc_dn7, locals.var_sqs_dc_dn8, ) = (locals.var_sqs, locals.var_sqs_dn5, locals.var_sqs_dn6, locals.var_sqs_dn7, locals.var_sqs_dn8, );
        locals.var_sqs_dc_rv = 0.0;

        (locals.var_alphas_dc, locals.var_alphas_dc_dn5, locals.var_alphas_dc_dn6, locals.var_alphas_dc_dn7, locals.var_alphas_dc_dn8, ) = (locals.var_alphas, locals.var_alphas_dn5, locals.var_alphas_dn6, locals.var_alphas_dn7, locals.var_alphas_dn8, );
        locals.var_alphas_dc_rv = 0.0;

        (locals.var_rxcor_dc, locals.var_rxcor_dc_dn5, locals.var_rxcor_dc_dn6, locals.var_rxcor_dc_dn7, locals.var_rxcor_dc_dn8, ) = (locals.var_rxcor, locals.var_rxcor_dn5, locals.var_rxcor_dn6, locals.var_rxcor_dn7, locals.var_rxcor_dn8, );
        locals.var_rxcor_dc_rv = 0.0;

        (locals.var_xgs_dc, locals.var_xgs_dc_dn5, locals.var_xgs_dc_dn6, locals.var_xgs_dc_dn7, locals.var_xgs_dc_dn8, ) = (locals.var_xgs, locals.var_xgs_dn5, locals.var_xgs_dn6, locals.var_xgs_dn7, locals.var_xgs_dn8, );
        locals.var_xgs_dc_rv = 0.0;

        (locals.var_qis_dc, locals.var_qis_dc_dn5, locals.var_qis_dc_dn6, locals.var_qis_dc_dn7, locals.var_qis_dc_dn8, ) = (locals.var_qis, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, );
        locals.var_qis_dc_rv = 0.0;

        (locals.var_qbs_dc, locals.var_qbs_dc_dn5, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8, ) = (locals.var_qbs, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, );
        locals.var_qbs_dc_rv = 0.0;

        (locals.var_rhob_dc, locals.var_rhob_dc_dn5, locals.var_rhob_dc_dn6, locals.var_rhob_dc_dn7, locals.var_rhob_dc_dn8, ) = (locals.var_rhob, locals.var_rhob_dn5, locals.var_rhob_dn6, locals.var_rhob_dn7, locals.var_rhob_dn8, );
        locals.var_rhob_dc_rv = 0.0;

        (locals.var_rhog_dc, locals.var_rhog_dc_dn5, locals.var_rhog_dc_dn6, locals.var_rhog_dc_dn7, locals.var_rhog_dc_dn8, ) = (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, );
        locals.var_rhog_dc_rv = 0.0;

        (locals.var_gmobs_dc, locals.var_gmobs_dc_dn5, locals.var_gmobs_dc_dn6, locals.var_gmobs_dc_dn7, locals.var_gmobs_dc_dn8, ) = (locals.var_gmobs, locals.var_gmobs_dn5, locals.var_gmobs_dn6, locals.var_gmobs_dn7, locals.var_gmobs_dn8, );
        locals.var_gmobs_dc_rv = 0.0;

        (locals.var_xitsb_dc, locals.var_xitsb_dc_dn5, locals.var_xitsb_dc_dn6, locals.var_xitsb_dc_dn7, locals.var_xitsb_dc_dn8, ) = (locals.var_xitsb, locals.var_xitsb_dn5, locals.var_xitsb_dn6, locals.var_xitsb_dn7, locals.var_xitsb_dn8, );
        locals.var_xitsb_dc_rv = 0.0;

        (locals.var_factheta_dc, locals.var_factheta_dc_dn5, locals.var_factheta_dc_dn6, locals.var_factheta_dc_dn7, locals.var_factheta_dc_dn8, ) = (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, );
        locals.var_factheta_dc_rv = 0.0;

        (locals.var_thesat1, locals.var_thesat1_dn5, locals.var_thesat1_dn6, locals.var_thesat1_dn7, locals.var_thesat1_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_thesat1_rv = 0.0;

        let assign43300_e56284: f64 = (locals.var_phit1 * 4.60517018598809);
        (locals.var_vdsat_lim, locals.var_vdsat_lim_dn5, locals.var_vdsat_lim_dn6, locals.var_vdsat_lim_dn7, locals.var_vdsat_lim_dn8, ) = (assign43300_e56284, (locals.var_phit1_dn5 * 4.60517018598809), (locals.var_phit1_dn6 * 4.60517018598809), (locals.var_phit1_dn7 * 4.60517018598809), (locals.var_phit1_dn8 * 4.60517018598809), );
        locals.var_vdsat_lim_rv = 0.0;

        (locals.var_v_dsat, locals.var_v_dsat_dn5, locals.var_v_dsat_dn6, locals.var_v_dsat_dn7, locals.var_v_dsat_dn8, ) = (locals.var_vdsat_lim, locals.var_vdsat_lim_dn5, locals.var_vdsat_lim_dn6, locals.var_vdsat_lim_dn7, locals.var_vdsat_lim_dn8, );
        locals.var_v_dsat_rv = 0.0;

        (locals.var_vdse, locals.var_vdse_dn5, locals.var_vdse_dn6, locals.var_vdse_dn7, locals.var_vdse_dn8, ) = (locals.var_v_ds, 0.0, locals.var_v_ds_dn6, locals.var_v_ds_dn7, 0.0, );
        locals.var_vdse_rv = 0.0;

        let assign43330_e56289: f64 = (locals.var_v_ds * locals.var_inv_phit1);
        (locals.var_udse, locals.var_udse_dn5, locals.var_udse_dn6, locals.var_udse_dn7, locals.var_udse_dn8, ) = (assign43330_e56289, (locals.var_v_ds * locals.var_inv_phit1_dn5), ((locals.var_v_ds_dn6 * locals.var_inv_phit1) + (locals.var_v_ds * locals.var_inv_phit1_dn6)), ((locals.var_v_ds_dn7 * locals.var_inv_phit1) + (locals.var_v_ds * locals.var_inv_phit1_dn7)), (locals.var_v_ds * locals.var_inv_phit1_dn8), );
        locals.var_udse_rv = 0.0;

        (locals.var_x_d, locals.var_x_d_dn5, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8, ) = (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, );
        locals.var_x_d_rv = 0.0;

        (locals.var_x_ds, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_x_ds_rv = 0.0;

        (locals.var_dps, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_dps_rv = 0.0;

        (locals.var_ed, locals.var_ed_dn5, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, ) = (locals.var_es, locals.var_es_dn5, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, );
        locals.var_ed_rv = 0.0;

        (locals.var_pd, locals.var_pd_dn5, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, ) = (locals.var_ps, locals.var_ps_dn5, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8, );
        locals.var_pd_rv = 0.0;

        (locals.var_dd, locals.var_dd_dn5, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, ) = (locals.var_ds, locals.var_ds_dn5, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, );
        locals.var_dd_rv = 0.0;

        (locals.var_qbd, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, ) = (locals.var_qbs, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, );
        locals.var_qbd_rv = 0.0;

        (locals.var_x_m, locals.var_x_m_dn5, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, ) = (locals.var_x_s, locals.var_x_s_dn5, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, );
        locals.var_x_m_rv = 0.0;

        (locals.var_em, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, ) = (locals.var_es, locals.var_es_dn5, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, );
        locals.var_em_rv = 0.0;

        (locals.var_dm, locals.var_dm_dn5, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, ) = (locals.var_ds, locals.var_ds_dn5, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, );
        locals.var_dm_rv = 0.0;

        (locals.var_pm, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, ) = (locals.var_ps, locals.var_ps_dn5, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8, );
        locals.var_pm_rv = 0.0;

        let assign43450_e56303: f64 = (locals.var_xg - locals.var_x_s);
        (locals.var_xgm, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, ) = (assign43450_e56303, (locals.var_xg_dn5 - locals.var_x_s_dn5), (locals.var_xg_dn6 - locals.var_x_s_dn6), (locals.var_xg_dn7 - locals.var_x_s_dn7), (locals.var_xg_dn8 - locals.var_x_s_dn8), );
        locals.var_xgm_rv = 0.0;

        (locals.var_eta_p, locals.var_eta_p_dn5, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_eta_p_rv = 0.0;

        (locals.var_alpha, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_alpha_rv = 0.0;

        (locals.var_sqm, locals.var_sqm_dn5, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_sqm_rv = 0.0;

        (locals.var_qim, locals.var_qim_dn5, locals.var_qim_dn6, locals.var_qim_dn7, locals.var_qim_dn8, ) = (locals.var_qis, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, );
        locals.var_qim_rv = 0.0;

        let assign43500_e56310: f64 = (locals.var_xgm * locals.var_phit1);
        (locals.var_qeff1, locals.var_qeff1_dn5, locals.var_qeff1_dn6, locals.var_qeff1_dn7, locals.var_qeff1_dn8, ) = (assign43500_e56310, ((locals.var_xgm_dn5 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn5)), ((locals.var_xgm_dn6 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn6)), ((locals.var_xgm_dn7 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn7)), ((locals.var_xgm_dn8 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn8)), );
        locals.var_qeff1_rv = 0.0;

        (locals.var_qim1, locals.var_qim1_dn5, locals.var_qim1_dn6, locals.var_qim1_dn7, locals.var_qim1_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qim1_rv = 0.0;

        (locals.var_qbm, locals.var_qbm_dn5, locals.var_qbm_dn6, locals.var_qbm_dn7, locals.var_qbm_dn8, ) = (locals.var_qbs, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, );
        locals.var_qbm_rv = 0.0;

        (locals.var_s1, locals.var_s1_dn5, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_s1_rv = 0.0;

        (locals.var_gmob, locals.var_gmob_dn5, locals.var_gmob_dn6, locals.var_gmob_dn7, locals.var_gmob_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_gmob_rv = 0.0;

        (locals.var_thesateff, locals.var_thesateff_dn5, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8, ) = (locals.var_thesatloc, 0.0, 0.0, 0.0, 0.0, );
        locals.var_thesateff_rv = 0.0;

        (locals.var_voxm, locals.var_voxm_dn5, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8, ) = (locals.var_qeff1, locals.var_qeff1_dn5, locals.var_qeff1_dn6, locals.var_qeff1_dn7, locals.var_qeff1_dn8, );
        locals.var_voxm_rv = 0.0;

        let assign43570_e56319: f64 = if locals.var_xg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign43570_e56319;
        locals.var_guard1197_rv = 0.0;

        let assign43580_e56322: f64 = if locals.var_ds > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign43580_e56322;
        locals.var_guard1198_rv = 0.0;

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43590_e56328: f64 = (locals.var_thesatloc * locals.var_factheta);
            (locals.var_thesateff, locals.var_thesateff_dn5, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8, ) = (assign43590_e56328, (locals.var_thesatloc * locals.var_factheta_dn5), (locals.var_thesatloc * locals.var_factheta_dn6), (locals.var_thesatloc * locals.var_factheta_dn7), (locals.var_thesatloc * locals.var_factheta_dn8), );
            locals.var_thesateff_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43600_e56336: f64 = (locals.var_thesateff / locals.var_gmobs);
            (locals.var_thesat1, locals.var_thesat1_dn5, locals.var_thesat1_dn6, locals.var_thesat1_dn7, locals.var_thesat1_dn8, ) = (assign43600_e56336, (((locals.var_thesateff_dn5 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn5)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn6 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn6)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn7 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn7)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn8 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn8)) / (locals.var_gmobs * locals.var_gmobs)), );
            locals.var_thesat1_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43610_e56345: f64 = (0.5 * locals.var_gf2);
            let assign43610_e56346: f64 = (locals.var_xgs + assign43610_e56345);
            (locals.var_asat, locals.var_asat_dn5, locals.var_asat_dn6, locals.var_asat_dn7, locals.var_asat_dn8, ) = (assign43610_e56346, (locals.var_xgs_dn5 + (0.5 * locals.var_gf2_dn5)), (locals.var_xgs_dn6 + (0.5 * locals.var_gf2_dn6)), (locals.var_xgs_dn7 + (0.5 * locals.var_gf2_dn7)), (locals.var_xgs_dn8 + (0.5 * locals.var_gf2_dn8)), );
            locals.var_asat_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43620_e56354: f64 = (locals.var_gf2 * locals.var_delta_1s);
            let __rspice_inv_cse_0: f64 = 1.0 / locals.var_asat;
            let assign43620_e56356: f64 = (assign43620_e56354 * __rspice_inv_cse_0);
            let assign43620_e56358: f64 = (assign43620_e56356 * __rspice_inv_cse_0);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign43620_e56358, ((((((((locals.var_gf2_dn5 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn5)) * locals.var_asat) - (assign43620_e56354 * locals.var_asat_dn5)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43620_e56356 * locals.var_asat_dn5)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn6 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn6)) * locals.var_asat) - (assign43620_e56354 * locals.var_asat_dn6)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43620_e56356 * locals.var_asat_dn6)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn7 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn7)) * locals.var_asat) - (assign43620_e56354 * locals.var_asat_dn7)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43620_e56356 * locals.var_asat_dn7)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn8 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn8)) * locals.var_asat) - (assign43620_e56354 * locals.var_asat_dn8)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43620_e56356 * locals.var_asat_dn8)) / (locals.var_asat * locals.var_asat)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        let assign43630_e56363: f64 = if locals.var_temp__blk936 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign43630_e56363;
        locals.var_guard1199_rv = 0.0;

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1199 != 0.0)) {
            let assign43640_e56371: f64 = (1.0 - locals.var_temp__blk936);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign43640_e56371, (-locals.var_temp__blk936_dn5), (-locals.var_temp__blk936_dn6), (-locals.var_temp__blk936_dn7), (-locals.var_temp__blk936_dn8), );
            locals.var_temp1_rv = 0.0;
        }

        let assign43650_e56376: f64 = if locals.var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign43650_e56376;
        locals.var_guard1200_rv = 0.0;

        if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1199 != 0.0)) && (locals.var_guard1200 != 0.0)) {
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp2_rv = 0.0;
        }

        if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1199 != 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign43670_e56397: f64 = (locals.var_temp1).sqrt();
            let assign43670_e56398: f64 = (1.0 - assign43670_e56397);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign43670_e56398, (-(locals.var_temp1_dn5 / (2.0 * assign43670_e56397))), (-(locals.var_temp1_dn6 / (2.0 * assign43670_e56397))), (-(locals.var_temp1_dn7 / (2.0 * assign43670_e56397))), (-(locals.var_temp1_dn8 / (2.0 * assign43670_e56397))), );
            locals.var_temp2_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1199 == 0.0)) {
            let assign43680_e56409: f64 = (0.5 * locals.var_temp__blk936);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign43680_e56409, (0.5 * locals.var_temp__blk936_dn5), (0.5 * locals.var_temp__blk936_dn6), (0.5 * locals.var_temp__blk936_dn7), (0.5 * locals.var_temp__blk936_dn8), );
            locals.var_temp2_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43690_e56417: f64 = (locals.var_temp2 * locals.var_asat);
            (locals.var_x_inf0, locals.var_x_inf0_dn5, locals.var_x_inf0_dn6, locals.var_x_inf0_dn7, locals.var_x_inf0_dn8, ) = (assign43690_e56417, ((locals.var_temp2_dn5 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn5)), ((locals.var_temp2_dn6 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn6)), ((locals.var_temp2_dn7 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn7)), ((locals.var_temp2_dn8 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn8)), );
            locals.var_x_inf0_rv = 0.0;
        }

        let assign43700_e56426: f64 = if ((locals.var_cs_t > 0.0) && (locals.var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign43700_e56426;
        locals.var_guard1201_rv = 0.0;

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43710_e56434: f64 = (0.475 * locals.var_phit1);
            let assign43710_e56436: f64 = (assign43710_e56434 * locals.var_x_inf0);
            (locals.var_midphi0, locals.var_midphi0_dn5, locals.var_midphi0_dn6, locals.var_midphi0_dn7, locals.var_midphi0_dn8, ) = (assign43710_e56436, (((0.475 * locals.var_phit1_dn5) * locals.var_x_inf0) + (assign43710_e56434 * locals.var_x_inf0_dn5)), (((0.475 * locals.var_phit1_dn6) * locals.var_x_inf0) + (assign43710_e56434 * locals.var_x_inf0_dn6)), (((0.475 * locals.var_phit1_dn7) * locals.var_x_inf0) + (assign43710_e56434 * locals.var_x_inf0_dn7)), (((0.475 * locals.var_phit1_dn8) * locals.var_x_inf0) + (assign43710_e56434 * locals.var_x_inf0_dn8)), );
            locals.var_midphi0_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43720_e56447: f64 = (locals.var_alphas * locals.var_midphi0);
            let assign43720_e56448: f64 = (locals.var_qis - assign43720_e56447);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign43720_e56448, (locals.var_qis_dn5 - ((locals.var_alphas_dn5 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn5))), (locals.var_qis_dn6 - ((locals.var_alphas_dn6 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn6))), (locals.var_qis_dn7 - ((locals.var_alphas_dn7 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn7))), (locals.var_qis_dn8 - ((locals.var_alphas_dn8 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn8))), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43730_e56460: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
            let assign43730_e56462: f64 = (assign43730_e56460 + 1e-12);
            let assign43730_e56463: f64 = (assign43730_e56462).sqrt();
            let assign43730_e56464: f64 = (locals.var_temp__blk936 + assign43730_e56463);
            let assign43730_e56465: f64 = (0.5 * assign43730_e56464);
            (locals.var_qisat, locals.var_qisat_dn5, locals.var_qisat_dn6, locals.var_qisat_dn7, locals.var_qisat_dn8, ) = (assign43730_e56465, (0.5 * (locals.var_temp__blk936_dn5 + (((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) / (2.0 * assign43730_e56463)))), (0.5 * (locals.var_temp__blk936_dn6 + (((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) / (2.0 * assign43730_e56463)))), (0.5 * (locals.var_temp__blk936_dn7 + (((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) / (2.0 * assign43730_e56463)))), (0.5 * (locals.var_temp__blk936_dn8 + (((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) / (2.0 * assign43730_e56463)))), );
            locals.var_qisat_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43740_e56475: f64 = (locals.var_phit1 * locals.var_xgs);
            let assign43740_e56477: f64 = (assign43740_e56475 - locals.var_qis);
            let assign43740_e56480: f64 = (locals.var_alphas - 1.0);
            let assign43740_e56482: f64 = (assign43740_e56480 * locals.var_midphi0);
            let assign43740_e56483: f64 = (assign43740_e56477 + assign43740_e56482);
            (locals.var_qbsat, locals.var_qbsat_dn5, locals.var_qbsat_dn6, locals.var_qbsat_dn7, locals.var_qbsat_dn8, ) = (assign43740_e56483, ((((locals.var_phit1_dn5 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn5)) - locals.var_qis_dn5) + ((locals.var_alphas_dn5 * locals.var_midphi0) + (assign43740_e56480 * locals.var_midphi0_dn5))), ((((locals.var_phit1_dn6 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn6)) - locals.var_qis_dn6) + ((locals.var_alphas_dn6 * locals.var_midphi0) + (assign43740_e56480 * locals.var_midphi0_dn6))), ((((locals.var_phit1_dn7 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn7)) - locals.var_qis_dn7) + ((locals.var_alphas_dn7 * locals.var_midphi0) + (assign43740_e56480 * locals.var_midphi0_dn7))), ((((locals.var_phit1_dn8 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn8)) - locals.var_qis_dn8) + ((locals.var_alphas_dn8 * locals.var_midphi0) + (assign43740_e56480 * locals.var_midphi0_dn8))), );
            locals.var_qbsat_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43750_e56494: f64 = (0.5 * locals.var_gf2);
            let assign43750_e56496: f64 = (assign43750_e56494 * locals.var_phit1);
            let assign43750_e56498: f64 = (assign43750_e56496 / locals.var_qbsat);
            let assign43750_e56499: f64 = (1.0 + assign43750_e56498);
            (locals.var_alphasat, locals.var_alphasat_dn5, locals.var_alphasat_dn6, locals.var_alphasat_dn7, locals.var_alphasat_dn8, ) = (assign43750_e56499, ((((((0.5 * locals.var_gf2_dn5) * locals.var_phit1) + (assign43750_e56494 * locals.var_phit1_dn5)) * locals.var_qbsat) - (assign43750_e56496 * locals.var_qbsat_dn5)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn6) * locals.var_phit1) + (assign43750_e56494 * locals.var_phit1_dn6)) * locals.var_qbsat) - (assign43750_e56496 * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn7) * locals.var_phit1) + (assign43750_e56494 * locals.var_phit1_dn7)) * locals.var_qbsat) - (assign43750_e56496 * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn8) * locals.var_phit1) + (assign43750_e56494 * locals.var_phit1_dn8)) * locals.var_qbsat) - (assign43750_e56496 * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)), );
            locals.var_alphasat_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43760_e56510: f64 = (locals.var_eta_mu * locals.var_qisat);
            let assign43760_e56511: f64 = (locals.var_qbsat + assign43760_e56510);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign43760_e56511, (locals.var_qbsat_dn5 + (locals.var_eta_mu * locals.var_qisat_dn5)), (locals.var_qbsat_dn6 + (locals.var_eta_mu * locals.var_qisat_dn6)), (locals.var_qbsat_dn7 + (locals.var_eta_mu * locals.var_qisat_dn7)), (locals.var_qbsat_dn8 + (locals.var_eta_mu * locals.var_qisat_dn8)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43770_e56521: f64 = (locals.var_e_eff0 * locals.var_temp__blk936);
            let assign43770_e56523: f64 = (assign43770_e56521 * locals.var_mue_t);
            let assign43770_e56525: f64 = (assign43770_e56523).powf(locals.var_themu_t);
            (locals.var_gmobmusat, locals.var_gmobmusat_dn5, locals.var_gmobmusat_dn6, locals.var_gmobmusat_dn7, locals.var_gmobmusat_dn8, ) = (assign43770_e56525, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43770_e56523).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn5) * locals.var_mue_t))) } } else { (assign43770_e56525 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn5) * locals.var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43770_e56523).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn6) * locals.var_mue_t))) } } else { (assign43770_e56525 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn6) * locals.var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43770_e56523).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn7) * locals.var_mue_t))) } } else { (assign43770_e56525 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn7) * locals.var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43770_e56523).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn8) * locals.var_mue_t))) } } else { (assign43770_e56525 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn8) * locals.var_mue_t) / assign43770_e56523))) }, );
            locals.var_gmobmusat_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43780_e56537: f64 = (1.0 - locals.var_eta_mu);
            let assign43780_e56538: f64 = (locals.var_alphasat * assign43780_e56537);
            let assign43780_e56540: f64 = (assign43780_e56538 - 1.0);
            let assign43780_e56541: f64 = (locals.var_themu_t * assign43780_e56540);
            let assign43780_e56543: f64 = (assign43780_e56541 / locals.var_temp__blk936);
            let assign43780_e56545: f64 = (assign43780_e56543 * locals.var_gmobmusat);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign43780_e56545, ((((((locals.var_themu_t * (locals.var_alphasat_dn5 * assign43780_e56537)) * locals.var_temp__blk936) - (assign43780_e56541 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat) + (assign43780_e56543 * locals.var_gmobmusat_dn5)), ((((((locals.var_themu_t * (locals.var_alphasat_dn6 * assign43780_e56537)) * locals.var_temp__blk936) - (assign43780_e56541 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat) + (assign43780_e56543 * locals.var_gmobmusat_dn6)), ((((((locals.var_themu_t * (locals.var_alphasat_dn7 * assign43780_e56537)) * locals.var_temp__blk936) - (assign43780_e56541 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat) + (assign43780_e56543 * locals.var_gmobmusat_dn7)), ((((((locals.var_themu_t * (locals.var_alphasat_dn8 * assign43780_e56537)) * locals.var_temp__blk936) - (assign43780_e56541 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat) + (assign43780_e56543 * locals.var_gmobmusat_dn8)), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43790_e56555: f64 = (locals.var_qisat / locals.var_qbsat);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign43790_e56555, (((locals.var_qisat_dn5 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn5)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn6 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn7 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn8 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43800_e56566: f64 = (1.0 + locals.var_temp__blk936);
            let assign43800_e56568: f64 = (-locals.var_thecs_t);
            let assign43800_e56569: f64 = (assign43800_e56566).powf(assign43800_e56568);
            let assign43800_e56570: f64 = (locals.var_cs_t * assign43800_e56569);
            (locals.var_gmobcssat, locals.var_gmobcssat_dn5, locals.var_gmobcssat_dn6, locals.var_gmobcssat_dn7, locals.var_gmobcssat_dn8, ) = (assign43800_e56570, (locals.var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * locals.var_temp__blk936_dn5)) } } else { (assign43800_e56569 * (assign43800_e56568 * (locals.var_temp__blk936_dn5 / assign43800_e56566))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * locals.var_temp__blk936_dn6)) } } else { (assign43800_e56569 * (assign43800_e56568 * (locals.var_temp__blk936_dn6 / assign43800_e56566))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * locals.var_temp__blk936_dn7)) } } else { (assign43800_e56569 * (assign43800_e56568 * (locals.var_temp__blk936_dn7 / assign43800_e56566))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * locals.var_temp__blk936_dn8)) } } else { (assign43800_e56569 * (assign43800_e56568 * (locals.var_temp__blk936_dn8 / assign43800_e56566))) }), );
            locals.var_gmobcssat_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43810_e56581: f64 = (locals.var_alphasat - 1.0);
            let assign43810_e56585: f64 = (locals.var_temp__blk936 + 1.0);
            let assign43810_e56586: f64 = (1.0 / assign43810_e56585);
            let assign43810_e56587: f64 = (assign43810_e56581 + assign43810_e56586);
            let assign43810_e56588: f64 = (locals.var_thecs_t * assign43810_e56587);
            let assign43810_e56590: f64 = (assign43810_e56588 / locals.var_qbsat);
            let assign43810_e56592: f64 = (assign43810_e56590 * locals.var_gmobcssat);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign43810_e56592, ((((((locals.var_thecs_t * (locals.var_alphasat_dn5 + (-(locals.var_temp__blk936_dn5 / (assign43810_e56585 * assign43810_e56585))))) * locals.var_qbsat) - (assign43810_e56588 * locals.var_qbsat_dn5)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43810_e56590 * locals.var_gmobcssat_dn5)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn6 + (-(locals.var_temp__blk936_dn6 / (assign43810_e56585 * assign43810_e56585))))) * locals.var_qbsat) - (assign43810_e56588 * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43810_e56590 * locals.var_gmobcssat_dn6)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn7 + (-(locals.var_temp__blk936_dn7 / (assign43810_e56585 * assign43810_e56585))))) * locals.var_qbsat) - (assign43810_e56588 * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43810_e56590 * locals.var_gmobcssat_dn7)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn8 + (-(locals.var_temp__blk936_dn8 / (assign43810_e56585 * assign43810_e56585))))) * locals.var_qbsat) - (assign43810_e56588 * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43810_e56590 * locals.var_gmobcssat_dn8)), );
            locals.var_temp2_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43820_e56602: f64 = (locals.var_ther_i * locals.var_rhob);
            let assign43820_e56604: f64 = (assign43820_e56602 * locals.var_rhog);
            let assign43820_e56606: f64 = (assign43820_e56604 * locals.var_qisat);
            (locals.var_grsat, locals.var_grsat_dn5, locals.var_grsat_dn6, locals.var_grsat_dn7, locals.var_grsat_dn8, ) = (assign43820_e56606, (((((locals.var_ther_i * locals.var_rhob_dn5) * locals.var_rhog) + (assign43820_e56602 * locals.var_rhog_dn5)) * locals.var_qisat) + (assign43820_e56604 * locals.var_qisat_dn5)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign43820_e56602 * locals.var_rhog_dn6)) * locals.var_qisat) + (assign43820_e56604 * locals.var_qisat_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign43820_e56602 * locals.var_rhog_dn7)) * locals.var_qisat) + (assign43820_e56604 * locals.var_qisat_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign43820_e56602 * locals.var_rhog_dn8)) * locals.var_qisat) + (assign43820_e56604 * locals.var_qisat_dn8)), );
            locals.var_grsat_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43830_e56618: f64 = (locals.var_ther_i * locals.var_rhob);
            let assign43830_e56620: f64 = (assign43830_e56618 * locals.var_rhog);
            let assign43830_e56622: f64 = (assign43830_e56620 * locals.var_alphasat);
            let assign43830_e56623: f64 = (locals.var_temp1 - assign43830_e56622);
            let assign43830_e56625: f64 = (assign43830_e56623 / locals.var_temp2);
            let assign43830_e56626: f64 = (1.0 + assign43830_e56625);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign43830_e56626, ((((locals.var_temp1_dn5 - (((((locals.var_ther_i * locals.var_rhob_dn5) * locals.var_rhog) + (assign43830_e56618 * locals.var_rhog_dn5)) * locals.var_alphasat) + (assign43830_e56620 * locals.var_alphasat_dn5))) * locals.var_temp2) - (assign43830_e56623 * locals.var_temp2_dn5)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn6 - (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign43830_e56618 * locals.var_rhog_dn6)) * locals.var_alphasat) + (assign43830_e56620 * locals.var_alphasat_dn6))) * locals.var_temp2) - (assign43830_e56623 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn7 - (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign43830_e56618 * locals.var_rhog_dn7)) * locals.var_alphasat) + (assign43830_e56620 * locals.var_alphasat_dn7))) * locals.var_temp2) - (assign43830_e56623 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn8 - (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign43830_e56618 * locals.var_rhog_dn8)) * locals.var_alphasat) + (assign43830_e56620 * locals.var_alphasat_dn8))) * locals.var_temp2) - (assign43830_e56623 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        let assign43840_e56631: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign43840_e56631;
        locals.var_guard1202_rv = 0.0;

        if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) && (locals.var_guard1202 != 0.0)) {
            let assign43850_e56643: f64 = (2.0 * locals.var_temp__blk936);
            let assign43850_e56644: f64 = (assign43850_e56643).exp();
            let assign43850_e56645: f64 = (1.0 + assign43850_e56644);
            let assign43850_e56646: f64 = (assign43850_e56645).ln();
            let assign43850_e56647: f64 = (0.5 * assign43850_e56646);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign43850_e56647, (0.5 * ((assign43850_e56644 * (2.0 * locals.var_temp__blk936_dn5)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * locals.var_temp__blk936_dn6)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * locals.var_temp__blk936_dn7)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * locals.var_temp__blk936_dn8)) / assign43850_e56645)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) && (locals.var_guard1202 == 0.0)) {
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43870_e56667: f64 = (-locals.var_midphi0);
            let assign43870_e56669: f64 = (assign43870_e56667 * locals.var_temp2);
            let assign43870_e56671: f64 = (assign43870_e56669 * locals.var_temp1);
            let assign43870_e56674: f64 = (1.0 + locals.var_gmobmusat);
            let assign43870_e56676: f64 = (assign43870_e56674 + locals.var_gmobcssat);
            let assign43870_e56678: f64 = (assign43870_e56676 + locals.var_grsat);
            let assign43870_e56679: f64 = (assign43870_e56671 / assign43870_e56678);
            (locals.var_delta_gmob, locals.var_delta_gmob_dn5, locals.var_delta_gmob_dn6, locals.var_delta_gmob_dn7, locals.var_delta_gmob_dn8, ) = (assign43870_e56679, ((((((((-locals.var_midphi0_dn5) * locals.var_temp2) + (assign43870_e56667 * locals.var_temp2_dn5)) * locals.var_temp1) + (assign43870_e56669 * locals.var_temp1_dn5)) * assign43870_e56678) - (assign43870_e56671 * ((locals.var_gmobmusat_dn5 + locals.var_gmobcssat_dn5) + locals.var_grsat_dn5))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-locals.var_midphi0_dn6) * locals.var_temp2) + (assign43870_e56667 * locals.var_temp2_dn6)) * locals.var_temp1) + (assign43870_e56669 * locals.var_temp1_dn6)) * assign43870_e56678) - (assign43870_e56671 * ((locals.var_gmobmusat_dn6 + locals.var_gmobcssat_dn6) + locals.var_grsat_dn6))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-locals.var_midphi0_dn7) * locals.var_temp2) + (assign43870_e56667 * locals.var_temp2_dn7)) * locals.var_temp1) + (assign43870_e56669 * locals.var_temp1_dn7)) * assign43870_e56678) - (assign43870_e56671 * ((locals.var_gmobmusat_dn7 + locals.var_gmobcssat_dn7) + locals.var_grsat_dn7))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-locals.var_midphi0_dn8) * locals.var_temp2) + (assign43870_e56667 * locals.var_temp2_dn8)) * locals.var_temp1) + (assign43870_e56669 * locals.var_temp1_dn8)) * assign43870_e56678) - (assign43870_e56671 * ((locals.var_gmobmusat_dn8 + locals.var_gmobcssat_dn8) + locals.var_grsat_dn8))) / (assign43870_e56678 * assign43870_e56678)), );
            locals.var_delta_gmob_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 != 0.0)) {
            let assign43880_e56694: f64 = (locals.var_delta_gmob * locals.var_delta_gmob);
            let assign43880_e56695: f64 = (1.0 + assign43880_e56694);
            let assign43880_e56696: f64 = (assign43880_e56695).sqrt();
            let assign43880_e56697: f64 = (1.0 + assign43880_e56696);
            let assign43880_e56698: f64 = (locals.var_delta_gmob / assign43880_e56697);
            let assign43880_e56699: f64 = (1.0 + assign43880_e56698);
            let assign43880_e56700: f64 = (locals.var_x_inf0 * assign43880_e56699);
            (locals.var_x_inf, locals.var_x_inf_dn5, locals.var_x_inf_dn6, locals.var_x_inf_dn7, locals.var_x_inf_dn8, ) = (assign43880_e56700, ((locals.var_x_inf0_dn5 * assign43880_e56699) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn5 * assign43880_e56697) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn5 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn5)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((locals.var_x_inf0_dn6 * assign43880_e56699) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn6 * assign43880_e56697) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn6 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn6)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((locals.var_x_inf0_dn7 * assign43880_e56699) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn7 * assign43880_e56697) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn7 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn7)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((locals.var_x_inf0_dn8 * assign43880_e56699) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn8 * assign43880_e56697) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn8 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn8)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), );
            locals.var_x_inf_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1201 == 0.0)) {
            (locals.var_x_inf, locals.var_x_inf_dn5, locals.var_x_inf_dn6, locals.var_x_inf_dn7, locals.var_x_inf_dn8, ) = (locals.var_x_inf0, locals.var_x_inf0_dn5, locals.var_x_inf0_dn6, locals.var_x_inf0_dn7, locals.var_x_inf0_dn8, );
            locals.var_x_inf_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43900_e56717: f64 = (locals.var_phit1 * locals.var_thesat1);
            let assign43900_e56719: f64 = (assign43900_e56717 * locals.var_x_inf);
            let assign43900_e56721: f64 = (assign43900_e56719 * 0.7071067811865475);
            (locals.var_ysat, locals.var_ysat_dn5, locals.var_ysat_dn6, locals.var_ysat_dn7, locals.var_ysat_dn8, ) = (assign43900_e56721, (((((locals.var_phit1_dn5 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn5)) * locals.var_x_inf) + (assign43900_e56717 * locals.var_x_inf_dn5)) * 0.7071067811865475), (((((locals.var_phit1_dn6 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn6)) * locals.var_x_inf) + (assign43900_e56717 * locals.var_x_inf_dn6)) * 0.7071067811865475), (((((locals.var_phit1_dn7 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn7)) * locals.var_x_inf) + (assign43900_e56717 * locals.var_x_inf_dn7)) * 0.7071067811865475), (((((locals.var_phit1_dn8 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn8)) * locals.var_x_inf) + (assign43900_e56717 * locals.var_x_inf_dn8)) * 0.7071067811865475), );
            locals.var_ysat_rv = 0.0;
        }

        let assign43910_e56726: f64 = (-1.0);
        let assign43910_e56727: f64 = if locals.var_chnl_type == assign43910_e56726 { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign43910_e56727;
        locals.var_guard1203_rv = 0.0;

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) && (locals.var_guard1203 != 0.0)) {
            let assign43920_e56736: f64 = (1.0 + locals.var_ysat);
            let assign43920_e56737: f64 = (assign43920_e56736).sqrt();
            let assign43920_e56738: f64 = (locals.var_ysat / assign43920_e56737);
            (locals.var_ysat, locals.var_ysat_dn5, locals.var_ysat_dn6, locals.var_ysat_dn7, locals.var_ysat_dn8, ) = (assign43920_e56738, (((locals.var_ysat_dn5 * assign43920_e56737) - (locals.var_ysat * (locals.var_ysat_dn5 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((locals.var_ysat_dn6 * assign43920_e56737) - (locals.var_ysat * (locals.var_ysat_dn6 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((locals.var_ysat_dn7 * assign43920_e56737) - (locals.var_ysat * (locals.var_ysat_dn7 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((locals.var_ysat_dn8 * assign43920_e56737) - (locals.var_ysat * (locals.var_ysat_dn8 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), );
            locals.var_ysat_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43930_e56749: f64 = (4.0 * locals.var_ysat);
            let assign43930_e56750: f64 = (1.0 + assign43930_e56749);
            let assign43930_e56751: f64 = (assign43930_e56750).sqrt();
            let assign43930_e56752: f64 = (1.0 + assign43930_e56751);
            let assign43930_e56753: f64 = (2.0 / assign43930_e56752);
            (locals.var_za, locals.var_za_dn5, locals.var_za_dn6, locals.var_za_dn7, locals.var_za_dn8, ) = (assign43930_e56753, (-((2.0 * ((4.0 * locals.var_ysat_dn5) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * locals.var_ysat_dn6) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * locals.var_ysat_dn7) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * locals.var_ysat_dn8) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), );
            locals.var_za_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43940_e56761: f64 = (locals.var_za * locals.var_ysat);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign43940_e56761, ((locals.var_za_dn5 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn5)), ((locals.var_za_dn6 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn6)), ((locals.var_za_dn7 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn7)), ((locals.var_za_dn8 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn8)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43950_e56769: f64 = (locals.var_x_inf * locals.var_za);
            let assign43950_e56773: f64 = (0.86 * locals.var_temp__blk936);
            let assign43950_e56777: f64 = (locals.var_temp__blk936 * locals.var_za);
            let assign43950_e56778: f64 = (1.0 - assign43950_e56777);
            let assign43950_e56779: f64 = (assign43950_e56773 * assign43950_e56778);
            let assign43950_e56783: f64 = (4.0 * locals.var_temp__blk936);
            let assign43950_e56785: f64 = (assign43950_e56783 * locals.var_temp__blk936);
            let assign43950_e56787: f64 = (assign43950_e56785 * locals.var_za);
            let assign43950_e56788: f64 = (1.0 + assign43950_e56787);
            let assign43950_e56789: f64 = (assign43950_e56779 / assign43950_e56788);
            let assign43950_e56790: f64 = (1.0 + assign43950_e56789);
            let assign43950_e56791: f64 = (assign43950_e56769 * assign43950_e56790);
            (locals.var_x_0, locals.var_x_0_dn5, locals.var_x_0_dn6, locals.var_x_0_dn7, locals.var_x_0_dn8, ) = (assign43950_e56791, ((((locals.var_x_inf_dn5 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn5)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * locals.var_temp__blk936_dn5) * assign43950_e56778) + (assign43950_e56773 * (-((locals.var_temp__blk936_dn5 * locals.var_za) + (locals.var_temp__blk936 * locals.var_za_dn5))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * locals.var_temp__blk936_dn5) * locals.var_temp__blk936) + (assign43950_e56783 * locals.var_temp__blk936_dn5)) * locals.var_za) + (assign43950_e56785 * locals.var_za_dn5)))) / (assign43950_e56788 * assign43950_e56788)))), ((((locals.var_x_inf_dn6 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn6)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * locals.var_temp__blk936_dn6) * assign43950_e56778) + (assign43950_e56773 * (-((locals.var_temp__blk936_dn6 * locals.var_za) + (locals.var_temp__blk936 * locals.var_za_dn6))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * locals.var_temp__blk936_dn6) * locals.var_temp__blk936) + (assign43950_e56783 * locals.var_temp__blk936_dn6)) * locals.var_za) + (assign43950_e56785 * locals.var_za_dn6)))) / (assign43950_e56788 * assign43950_e56788)))), ((((locals.var_x_inf_dn7 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn7)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * locals.var_temp__blk936_dn7) * assign43950_e56778) + (assign43950_e56773 * (-((locals.var_temp__blk936_dn7 * locals.var_za) + (locals.var_temp__blk936 * locals.var_za_dn7))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * locals.var_temp__blk936_dn7) * locals.var_temp__blk936) + (assign43950_e56783 * locals.var_temp__blk936_dn7)) * locals.var_za) + (assign43950_e56785 * locals.var_za_dn7)))) / (assign43950_e56788 * assign43950_e56788)))), ((((locals.var_x_inf_dn8 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn8)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * locals.var_temp__blk936_dn8) * assign43950_e56778) + (assign43950_e56773 * (-((locals.var_temp__blk936_dn8 * locals.var_za) + (locals.var_temp__blk936 * locals.var_za_dn8))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * locals.var_temp__blk936_dn8) * locals.var_temp__blk936) + (assign43950_e56783 * locals.var_temp__blk936_dn8)) * locals.var_za) + (assign43950_e56785 * locals.var_za_dn8)))) / (assign43950_e56788 * assign43950_e56788)))), );
            locals.var_x_0_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43960_e56799: f64 = (0.99 * locals.var_x_0);
            (locals.var_x_sat, locals.var_x_sat_dn5, locals.var_x_sat_dn6, locals.var_x_sat_dn7, locals.var_x_sat_dn8, ) = (assign43960_e56799, (0.99 * locals.var_x_0_dn5), (0.99 * locals.var_x_0_dn6), (0.99 * locals.var_x_0_dn7), (0.99 * locals.var_x_0_dn8), );
            locals.var_x_sat_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43970_e56809: f64 = (2.0 * locals.var_asat);
            let assign43970_e56810: f64 = (locals.var_x_sat - assign43970_e56809);
            let assign43970_e56811: f64 = (locals.var_x_sat * assign43970_e56810);
            let assign43970_e56813: f64 = (assign43970_e56811 * locals.var_inv_gf2);
            let assign43970_e56815: f64 = (assign43970_e56813 / locals.var_ds);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign43970_e56815, (((((((locals.var_x_sat_dn5 * assign43970_e56810) + (locals.var_x_sat * (locals.var_x_sat_dn5 - (2.0 * locals.var_asat_dn5)))) * locals.var_inv_gf2) + (assign43970_e56811 * locals.var_inv_gf2_dn5)) * locals.var_ds) - (assign43970_e56813 * locals.var_ds_dn5)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn6 * assign43970_e56810) + (locals.var_x_sat * (locals.var_x_sat_dn6 - (2.0 * locals.var_asat_dn6)))) * locals.var_inv_gf2) + (assign43970_e56811 * locals.var_inv_gf2_dn6)) * locals.var_ds) - (assign43970_e56813 * locals.var_ds_dn6)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn7 * assign43970_e56810) + (locals.var_x_sat * (locals.var_x_sat_dn7 - (2.0 * locals.var_asat_dn7)))) * locals.var_inv_gf2) + (assign43970_e56811 * locals.var_inv_gf2_dn7)) * locals.var_ds) - (assign43970_e56813 * locals.var_ds_dn7)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn8 * assign43970_e56810) + (locals.var_x_sat * (locals.var_x_sat_dn8 - (2.0 * locals.var_asat_dn8)))) * locals.var_inv_gf2) + (assign43970_e56811 * locals.var_inv_gf2_dn8)) * locals.var_ds) - (assign43970_e56813 * locals.var_ds_dn8)) / (locals.var_ds * locals.var_ds)), );
            locals.var_temp__blk936_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_16(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign43980_e56826: f64 = (-0.99);
            let (assign43980_e56831, assign43980_e56831_d_n5, assign43980_e56831_d_n6, assign43980_e56831_d_n7, assign43980_e56831_d_n8,) = {
    if (locals.var_temp__blk936 > assign43980_e56826) {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        let assign43980_e56830: f64 = (-0.99);
        (assign43980_e56830, 0.0, 0.0, 0.0, 0.0,)
    }
};
            let assign43980_e56832: f64 = (1.0 + assign43980_e56831);
            let assign43980_e56833: f64 = (assign43980_e56832).ln();
            let assign43980_e56834: f64 = (locals.var_x_sat - assign43980_e56833);
            let assign43980_e56835: f64 = (locals.var_phit1 * assign43980_e56834);
            (locals.var_v_dsat, locals.var_v_dsat_dn5, locals.var_v_dsat_dn6, locals.var_v_dsat_dn7, locals.var_v_dsat_dn8, ) = (assign43980_e56835, ((locals.var_phit1_dn5 * assign43980_e56834) + (locals.var_phit1 * (locals.var_x_sat_dn5 - (assign43980_e56831_d_n5 / assign43980_e56832)))), ((locals.var_phit1_dn6 * assign43980_e56834) + (locals.var_phit1 * (locals.var_x_sat_dn6 - (assign43980_e56831_d_n6 / assign43980_e56832)))), ((locals.var_phit1_dn7 * assign43980_e56834) + (locals.var_phit1 * (locals.var_x_sat_dn7 - (assign43980_e56831_d_n7 / assign43980_e56832)))), ((locals.var_phit1_dn8 * assign43980_e56834) + (locals.var_phit1 * (locals.var_x_sat_dn8 - (assign43980_e56831_d_n8 / assign43980_e56832)))), );
            locals.var_v_dsat_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1198 == 0.0)) {
            (locals.var_v_dsat, locals.var_v_dsat_dn5, locals.var_v_dsat_dn6, locals.var_v_dsat_dn7, locals.var_v_dsat_dn8, ) = (locals.var_vdsat_lim, locals.var_vdsat_lim_dn5, locals.var_vdsat_lim_dn6, locals.var_vdsat_lim_dn7, locals.var_vdsat_lim_dn8, );
            locals.var_v_dsat_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44000_e56848: f64 = (1.0 + locals.var_arloc);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign44000_e56848, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44010_e56853: f64 = (locals.var_temp__blk936).sqrt();
            let assign44010_e56855: f64 = (assign44010_e56853 * locals.var_v_ds);
            let assign44010_e56857: f64 = (assign44010_e56855 / locals.var_v_dsat);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign44010_e56857, (((((locals.var_temp__blk936_dn5 / (2.0 * assign44010_e56853)) * locals.var_v_ds) * locals.var_v_dsat) - (assign44010_e56855 * locals.var_v_dsat_dn5)) / (locals.var_v_dsat * locals.var_v_dsat)), ((((((locals.var_temp__blk936_dn6 / (2.0 * assign44010_e56853)) * locals.var_v_ds) + (assign44010_e56853 * locals.var_v_ds_dn6)) * locals.var_v_dsat) - (assign44010_e56855 * locals.var_v_dsat_dn6)) / (locals.var_v_dsat * locals.var_v_dsat)), ((((((locals.var_temp__blk936_dn7 / (2.0 * assign44010_e56853)) * locals.var_v_ds) + (assign44010_e56853 * locals.var_v_ds_dn7)) * locals.var_v_dsat) - (assign44010_e56855 * locals.var_v_dsat_dn7)) / (locals.var_v_dsat * locals.var_v_dsat)), (((((locals.var_temp__blk936_dn8 / (2.0 * assign44010_e56853)) * locals.var_v_ds) * locals.var_v_dsat) - (assign44010_e56855 * locals.var_v_dsat_dn8)) / (locals.var_v_dsat * locals.var_v_dsat)), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44020_e56863: f64 = (locals.var_temp1 * locals.var_temp1);
            let assign44020_e56865: f64 = (assign44020_e56863 + locals.var_temp__blk936);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign44020_e56865, (((locals.var_temp1_dn5 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn5)) + locals.var_temp__blk936_dn5), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk936_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk936_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk936_dn8), );
            locals.var_temp2_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44030_e56871: f64 = (2.0 * locals.var_temp1);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign44030_e56871, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44040_e56877: f64 = (locals.var_v_dsat * locals.var_temp__blk936);
            let assign44040_e56880: f64 = (locals.var_temp2 - locals.var_temp__blk936);
            let assign44040_e56881: f64 = (assign44040_e56880).sqrt();
            let assign44040_e56884: f64 = (locals.var_temp2 + locals.var_temp__blk936);
            let assign44040_e56885: f64 = (assign44040_e56884).sqrt();
            let assign44040_e56886: f64 = (assign44040_e56881 + assign44040_e56885);
            let assign44040_e56887: f64 = (assign44040_e56877 / assign44040_e56886);
            (locals.var_vdse, locals.var_vdse_dn5, locals.var_vdse_dn6, locals.var_vdse_dn7, locals.var_vdse_dn8, ) = (assign44040_e56887, (((((locals.var_v_dsat_dn5 * locals.var_temp__blk936) + (locals.var_v_dsat * locals.var_temp__blk936_dn5)) * assign44040_e56886) - (assign44040_e56877 * (((locals.var_temp2_dn5 - locals.var_temp__blk936_dn5) / (2.0 * assign44040_e56881)) + ((locals.var_temp2_dn5 + locals.var_temp__blk936_dn5) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((locals.var_v_dsat_dn6 * locals.var_temp__blk936) + (locals.var_v_dsat * locals.var_temp__blk936_dn6)) * assign44040_e56886) - (assign44040_e56877 * (((locals.var_temp2_dn6 - locals.var_temp__blk936_dn6) / (2.0 * assign44040_e56881)) + ((locals.var_temp2_dn6 + locals.var_temp__blk936_dn6) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((locals.var_v_dsat_dn7 * locals.var_temp__blk936) + (locals.var_v_dsat * locals.var_temp__blk936_dn7)) * assign44040_e56886) - (assign44040_e56877 * (((locals.var_temp2_dn7 - locals.var_temp__blk936_dn7) / (2.0 * assign44040_e56881)) + ((locals.var_temp2_dn7 + locals.var_temp__blk936_dn7) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((locals.var_v_dsat_dn8 * locals.var_temp__blk936) + (locals.var_v_dsat * locals.var_temp__blk936_dn8)) * assign44040_e56886) - (assign44040_e56877 * (((locals.var_temp2_dn8 - locals.var_temp__blk936_dn8) / (2.0 * assign44040_e56881)) + ((locals.var_temp2_dn8 + locals.var_temp__blk936_dn8) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), );
            locals.var_vdse_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44050_e56893: f64 = (locals.var_vdse * locals.var_inv_phit1);
            (locals.var_udse, locals.var_udse_dn5, locals.var_udse_dn6, locals.var_udse_dn7, locals.var_udse_dn8, ) = (assign44050_e56893, ((locals.var_vdse_dn5 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn5)), ((locals.var_vdse_dn6 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn6)), ((locals.var_vdse_dn7 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn7)), ((locals.var_vdse_dn8 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn8)), );
            locals.var_udse_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44060_e56899: f64 = (locals.var_xn_s + locals.var_udse);
            (locals.var_xn_d, locals.var_xn_d_dn5, locals.var_xn_d_dn6, locals.var_xn_d_dn7, locals.var_xn_d_dn8, ) = (assign44060_e56899, (locals.var_xn_s_dn5 + locals.var_udse_dn5), (locals.var_xn_s_dn6 + locals.var_udse_dn6), (locals.var_xn_s_dn7 + locals.var_udse_dn7), (locals.var_xn_s_dn8 + locals.var_udse_dn8), );
            locals.var_xn_d_rv = 0.0;
        }

        let assign44070_e56904: f64 = if locals.var_udse < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign44070_e56904;
        locals.var_guard1204_rv = 0.0;

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1204 != 0.0)) {
            let assign44080_e56909: f64 = (-locals.var_udse);
            let assign44080_e56910: f64 = (assign44080_e56909).exp();
            (locals.var_k_ds, locals.var_k_ds_dn5, locals.var_k_ds_dn6, locals.var_k_ds_dn7, locals.var_k_ds_dn8, ) = (assign44080_e56910, (assign44080_e56910 * (-locals.var_udse_dn5)), (assign44080_e56910 * (-locals.var_udse_dn6)), (assign44080_e56910 * (-locals.var_udse_dn7)), (assign44080_e56910 * (-locals.var_udse_dn8)), );
            locals.var_k_ds_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1204 == 0.0)) {
            let assign44090_e56921: f64 = (locals.var_udse - 460.51701859880916);
            let assign44090_e56926: f64 = (locals.var_udse - 460.51701859880916);
            let assign44090_e56930: f64 = (locals.var_udse - 460.51701859880916);
            let assign44090_e56932: f64 = (assign44090_e56930 * 0.3333333333333333);
            let assign44090_e56933: f64 = (1.0 + assign44090_e56932);
            let assign44090_e56934: f64 = (assign44090_e56926 * assign44090_e56933);
            let assign44090_e56935: f64 = (0.5 * assign44090_e56934);
            let assign44090_e56936: f64 = (1.0 + assign44090_e56935);
            let assign44090_e56937: f64 = (assign44090_e56921 * assign44090_e56936);
            let assign44090_e56938: f64 = (1.0 + assign44090_e56937);
            let assign44090_e56939: f64 = (1e-200 / assign44090_e56938);
            (locals.var_k_ds, locals.var_k_ds_dn5, locals.var_k_ds_dn6, locals.var_k_ds_dn7, locals.var_k_ds_dn8, ) = (assign44090_e56939, (-((1e-200 * ((locals.var_udse_dn5 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((locals.var_udse_dn5 * assign44090_e56933) + (assign44090_e56926 * (locals.var_udse_dn5 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((locals.var_udse_dn6 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((locals.var_udse_dn6 * assign44090_e56933) + (assign44090_e56926 * (locals.var_udse_dn6 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((locals.var_udse_dn7 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((locals.var_udse_dn7 * assign44090_e56933) + (assign44090_e56926 * (locals.var_udse_dn7 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((locals.var_udse_dn8 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((locals.var_udse_dn8 * assign44090_e56933) + (assign44090_e56926 * (locals.var_udse_dn8 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), );
            locals.var_k_ds_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44100_e56945: f64 = (locals.var_delta_ns * locals.var_k_ds);
            (locals.var_delta_nd, locals.var_delta_nd_dn5, locals.var_delta_nd_dn6, locals.var_delta_nd_dn7, locals.var_delta_nd_dn8, ) = (assign44100_e56945, ((locals.var_delta_ns_dn5 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn5)), ((locals.var_delta_ns_dn6 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn6)), ((locals.var_delta_ns_dn7 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn7)), ((locals.var_delta_ns_dn8 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn8)), );
            locals.var_delta_nd_rv = 0.0;
        }

        let assign44110_e56949: f64 = (locals.var_xg).abs();
        let assign44110_e56951: f64 = if assign44110_e56949 <= locals.var_margin { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign44110_e56951;
        locals.var_guard1205_rv = 0.0;

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 != 0.0)) {
            let assign44120_e56957: f64 = (locals.var_inv_xi * locals.var_inv_xi);
            let assign44120_e56959: f64 = (assign44120_e56957 * 0.16666666666666666);
            let assign44120_e56961: f64 = (assign44120_e56959 * 0.7071067811865475);
            (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, ) = (assign44120_e56961, ((((locals.var_inv_xi_dn5 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn6 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn7 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn8 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475), );
            locals.var_sp_s_temp1_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 != 0.0)) {
            let assign44130_e56969: f64 = (locals.var_xg * locals.var_inv_xi);
            let assign44130_e56974: f64 = (1.0 - locals.var_delta_nd);
            let assign44130_e56975: f64 = (locals.var_xg * assign44130_e56974);
            let assign44130_e56977: f64 = (assign44130_e56975 * locals.var_gf);
            let assign44130_e56979: f64 = (assign44130_e56977 * locals.var_sp_s_temp1);
            let assign44130_e56980: f64 = (1.0 + assign44130_e56979);
            let assign44130_e56981: f64 = (assign44130_e56969 * assign44130_e56980);
            (locals.var_x_d, locals.var_x_d_dn5, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8, ) = (assign44130_e56981, ((((locals.var_xg_dn5 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn5)) * assign44130_e56980) + (assign44130_e56969 * ((((((locals.var_xg_dn5 * assign44130_e56974) + (locals.var_xg * (-locals.var_delta_nd_dn5))) * locals.var_gf) + (assign44130_e56975 * locals.var_gf_dn5)) * locals.var_sp_s_temp1) + (assign44130_e56977 * locals.var_sp_s_temp1_dn5)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign44130_e56980) + (assign44130_e56969 * ((((((locals.var_xg_dn6 * assign44130_e56974) + (locals.var_xg * (-locals.var_delta_nd_dn6))) * locals.var_gf) + (assign44130_e56975 * locals.var_gf_dn6)) * locals.var_sp_s_temp1) + (assign44130_e56977 * locals.var_sp_s_temp1_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign44130_e56980) + (assign44130_e56969 * ((((((locals.var_xg_dn7 * assign44130_e56974) + (locals.var_xg * (-locals.var_delta_nd_dn7))) * locals.var_gf) + (assign44130_e56975 * locals.var_gf_dn7)) * locals.var_sp_s_temp1) + (assign44130_e56977 * locals.var_sp_s_temp1_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign44130_e56980) + (assign44130_e56969 * ((((((locals.var_xg_dn8 * assign44130_e56974) + (locals.var_xg * (-locals.var_delta_nd_dn8))) * locals.var_gf) + (assign44130_e56975 * locals.var_gf_dn8)) * locals.var_sp_s_temp1) + (assign44130_e56977 * locals.var_sp_s_temp1_dn8)))), );
            locals.var_x_d_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44140_e56990: f64 = (locals.var_xn_d + 3.0);
            (locals.var_sp_s_bx, locals.var_sp_s_bx_dn5, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8, ) = (assign44140_e56990, locals.var_xn_d_dn5, locals.var_xn_d_dn6, locals.var_xn_d_dn7, locals.var_xn_d_dn8, );
            locals.var_sp_s_bx_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44150_e57000: f64 = (locals.var_sp_s_x1 + locals.var_sp_s_bx);
            let assign44150_e57003: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
            let assign44150_e57006: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
            let assign44150_e57007: f64 = (assign44150_e57003 * assign44150_e57006);
            let assign44150_e57009: f64 = (assign44150_e57007 + 5.0);
            let assign44150_e57010: f64 = (assign44150_e57009).sqrt();
            let assign44150_e57011: f64 = (assign44150_e57000 - assign44150_e57010);
            let assign44150_e57012: f64 = (0.5 * assign44150_e57011);
            let assign44150_e57017: f64 = (locals.var_sp_s_bx * locals.var_sp_s_bx);
            let assign44150_e57019: f64 = (assign44150_e57017 + 5.0);
            let assign44150_e57020: f64 = (assign44150_e57019).sqrt();
            let assign44150_e57021: f64 = (locals.var_sp_s_bx - assign44150_e57020);
            let assign44150_e57022: f64 = (0.5 * assign44150_e57021);
            let assign44150_e57023: f64 = (assign44150_e57012 - assign44150_e57022);
            (locals.var_sp_s_eta, locals.var_sp_s_eta_dn5, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, ) = (assign44150_e57023, ((0.5 * ((locals.var_sp_s_x1_dn5 + locals.var_sp_s_bx_dn5) - ((((locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5) * assign44150_e57006) + (assign44150_e57003 * (locals.var_sp_s_x1_dn5 - locals.var_sp_s_bx_dn5))) / (2.0 * assign44150_e57010)))) - (0.5 * (locals.var_sp_s_bx_dn5 - (((locals.var_sp_s_bx_dn5 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn5)) / (2.0 * assign44150_e57020))))), ((0.5 * ((locals.var_sp_s_x1_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6) * assign44150_e57006) + (assign44150_e57003 * (locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign44150_e57010)))) - (0.5 * (locals.var_sp_s_bx_dn6 - (((locals.var_sp_s_bx_dn6 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn6)) / (2.0 * assign44150_e57020))))), ((0.5 * ((locals.var_sp_s_x1_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7) * assign44150_e57006) + (assign44150_e57003 * (locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign44150_e57010)))) - (0.5 * (locals.var_sp_s_bx_dn7 - (((locals.var_sp_s_bx_dn7 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn7)) / (2.0 * assign44150_e57020))))), ((0.5 * ((locals.var_sp_s_x1_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8) * assign44150_e57006) + (assign44150_e57003 * (locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign44150_e57010)))) - (0.5 * (locals.var_sp_s_bx_dn8 - (((locals.var_sp_s_bx_dn8 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn8)) / (2.0 * assign44150_e57020))))), );
            locals.var_sp_s_eta_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44160_e57032: f64 = (locals.var_xg - locals.var_sp_s_eta);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign44160_e57032, (locals.var_xg_dn5 - locals.var_sp_s_eta_dn5), (locals.var_xg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_xg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_xg_dn8 - locals.var_sp_s_eta_dn8), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44170_e57040: f64 = (-locals.var_sp_s_eta);
            let assign44170_e57041: f64 = (assign44170_e57040).exp();
            (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn5, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, ) = (assign44170_e57041, (assign44170_e57041 * (-locals.var_sp_s_eta_dn5)), (assign44170_e57041 * (-locals.var_sp_s_eta_dn6)), (assign44170_e57041 * (-locals.var_sp_s_eta_dn7)), (assign44170_e57041 * (-locals.var_sp_s_eta_dn8)), );
            locals.var_sp_s_temp1_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44180_e57052: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
            let assign44180_e57053: f64 = (2.0 + assign44180_e57052);
            let assign44180_e57054: f64 = (1.0 / assign44180_e57053);
            (locals.var_sp_s_temp2, locals.var_sp_s_temp2_dn5, locals.var_sp_s_temp2_dn6, locals.var_sp_s_temp2_dn7, locals.var_sp_s_temp2_dn8, ) = (assign44180_e57054, (-(((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) / (assign44180_e57053 * assign44180_e57053))), (-(((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) / (assign44180_e57053 * assign44180_e57053))), (-(((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) / (assign44180_e57053 * assign44180_e57053))), (-(((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) / (assign44180_e57053 * assign44180_e57053))), );
            locals.var_sp_s_temp2_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44190_e57063: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
            let assign44190_e57065: f64 = (assign44190_e57063 * locals.var_sp_s_temp2);
            (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, ) = (assign44190_e57065, ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn5)) * locals.var_sp_s_temp2) + (assign44190_e57063 * locals.var_sp_s_temp2_dn5)), ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) * locals.var_sp_s_temp2) + (assign44190_e57063 * locals.var_sp_s_temp2_dn6)), ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) * locals.var_sp_s_temp2) + (assign44190_e57063 * locals.var_sp_s_temp2_dn7)), ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) * locals.var_sp_s_temp2) + (assign44190_e57063 * locals.var_sp_s_temp2_dn8)), );
            locals.var_sp_s_xi0_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44200_e57075: f64 = (locals.var_sp_s_eta * locals.var_sp_s_temp2);
            let assign44200_e57077: f64 = (assign44200_e57075 * locals.var_sp_s_temp2);
            let assign44200_e57078: f64 = (4.0 * assign44200_e57077);
            (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, ) = (assign44200_e57078, (4.0 * ((((locals.var_sp_s_eta_dn5 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign44200_e57075 * locals.var_sp_s_temp2_dn5))), (4.0 * ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign44200_e57075 * locals.var_sp_s_temp2_dn6))), (4.0 * ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign44200_e57075 * locals.var_sp_s_temp2_dn7))), (4.0 * ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign44200_e57075 * locals.var_sp_s_temp2_dn8))), );
            locals.var_sp_s_xi1_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44210_e57087: f64 = (8.0 * locals.var_sp_s_temp2);
            let assign44210_e57090: f64 = (12.0 * locals.var_sp_s_xi0);
            let assign44210_e57091: f64 = (assign44210_e57087 - assign44210_e57090);
            let assign44210_e57093: f64 = (assign44210_e57091 * locals.var_sp_s_temp2);
            let assign44210_e57095: f64 = (assign44210_e57093 * locals.var_sp_s_temp2);
            (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, ) = (assign44210_e57095, ((((((8.0 * locals.var_sp_s_temp2_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp2) + (assign44210_e57091 * locals.var_sp_s_temp2_dn5)) * locals.var_sp_s_temp2) + (assign44210_e57093 * locals.var_sp_s_temp2_dn5)), ((((((8.0 * locals.var_sp_s_temp2_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp2) + (assign44210_e57091 * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign44210_e57093 * locals.var_sp_s_temp2_dn6)), ((((((8.0 * locals.var_sp_s_temp2_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp2) + (assign44210_e57091 * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign44210_e57093 * locals.var_sp_s_temp2_dn7)), ((((((8.0 * locals.var_sp_s_temp2_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp2) + (assign44210_e57091 * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign44210_e57093 * locals.var_sp_s_temp2_dn8)), );
            locals.var_sp_s_xi2_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44220_e57105: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
            let assign44220_e57109: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
            let assign44220_e57111: f64 = (assign44220_e57109 - 1.0);
            let assign44220_e57115: f64 = (locals.var_sp_s_eta + 1.0);
            let assign44220_e57117: f64 = (assign44220_e57115 + locals.var_sp_s_xi0);
            let assign44220_e57118: f64 = (locals.var_delta_nd * assign44220_e57117);
            let assign44220_e57119: f64 = (assign44220_e57111 - assign44220_e57118);
            let assign44220_e57120: f64 = (locals.var_gf2 * assign44220_e57119);
            let assign44220_e57121: f64 = (assign44220_e57105 - assign44220_e57120);
            let (assign44220_e57143, assign44220_e57143_d_n5, assign44220_e57143_d_n6, assign44220_e57143_d_n7, assign44220_e57143_d_n8,) = {
    if (1e-40 > assign44220_e57121) {
        (1e-40, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign44220_e57126: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign44220_e57130: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
        let assign44220_e57132: f64 = (assign44220_e57130 - 1.0);
        let assign44220_e57136: f64 = (locals.var_sp_s_eta + 1.0);
        let assign44220_e57138: f64 = (assign44220_e57136 + locals.var_sp_s_xi0);
        let assign44220_e57139: f64 = (locals.var_delta_nd * assign44220_e57138);
        let assign44220_e57140: f64 = (assign44220_e57132 - assign44220_e57139);
        let assign44220_e57141: f64 = (locals.var_gf2 * assign44220_e57140);
        let assign44220_e57142: f64 = (assign44220_e57126 - assign44220_e57141);
        (assign44220_e57142, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign44220_e57140) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn5 + locals.var_sp_s_eta_dn5) - ((locals.var_delta_nd_dn5 * assign44220_e57138) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign44220_e57140) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn6 + locals.var_sp_s_eta_dn6) - ((locals.var_delta_nd_dn6 * assign44220_e57138) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign44220_e57140) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn7 + locals.var_sp_s_eta_dn7) - ((locals.var_delta_nd_dn7 * assign44220_e57138) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign44220_e57140) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn8 + locals.var_sp_s_eta_dn8) - ((locals.var_delta_nd_dn8 * assign44220_e57138) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn8 + locals.var_sp_s_xi0_dn8))))))),)
    }
};
            (locals.var_sp_s_a, locals.var_sp_s_a_dn5, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, ) = (assign44220_e57143, assign44220_e57143_d_n5, assign44220_e57143_d_n6, assign44220_e57143_d_n7, assign44220_e57143_d_n8, );
            locals.var_sp_s_a_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44230_e57156: f64 = (locals.var_delta_nd * locals.var_sp_s_xi2);
            let assign44230_e57157: f64 = (locals.var_sp_s_temp1 - assign44230_e57156);
            let assign44230_e57158: f64 = (locals.var_gf2 * assign44230_e57157);
            let assign44230_e57159: f64 = (0.5 * assign44230_e57158);
            let assign44230_e57160: f64 = (1.0 - assign44230_e57159);
            (locals.var_sp_s_b, locals.var_sp_s_b_dn5, locals.var_sp_s_b_dn6, locals.var_sp_s_b_dn7, locals.var_sp_s_b_dn8, ) = (assign44230_e57160, (-(0.5 * ((locals.var_gf2_dn5 * assign44230_e57157) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn5 - ((locals.var_delta_nd_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn5))))))), (-(0.5 * ((locals.var_gf2_dn6 * assign44230_e57157) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn6 - ((locals.var_delta_nd_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn6))))))), (-(0.5 * ((locals.var_gf2_dn7 * assign44230_e57157) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn7 - ((locals.var_delta_nd_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn7))))))), (-(0.5 * ((locals.var_gf2_dn8 * assign44230_e57157) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn8 - ((locals.var_delta_nd_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn8))))))), );
            locals.var_sp_s_b_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44240_e57169: f64 = (2.0 * locals.var_sp_s_temp);
            let assign44240_e57173: f64 = (1.0 - locals.var_sp_s_temp1);
            let assign44240_e57177: f64 = (1.0 + locals.var_sp_s_xi1);
            let assign44240_e57178: f64 = (locals.var_delta_nd * assign44240_e57177);
            let assign44240_e57179: f64 = (assign44240_e57173 - assign44240_e57178);
            let assign44240_e57180: f64 = (locals.var_gf2 * assign44240_e57179);
            let assign44240_e57181: f64 = (assign44240_e57169 + assign44240_e57180);
            (locals.var_sp_s_c, locals.var_sp_s_c_dn5, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, ) = (assign44240_e57181, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign44240_e57179) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn5) - ((locals.var_delta_nd_dn5 * assign44240_e57177) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign44240_e57179) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn6) - ((locals.var_delta_nd_dn6 * assign44240_e57177) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign44240_e57179) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn7) - ((locals.var_delta_nd_dn7 * assign44240_e57177) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign44240_e57179) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn8) - ((locals.var_delta_nd_dn8 * assign44240_e57177) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn8)))))), );
            locals.var_sp_s_c_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44250_e57190: f64 = (locals.var_xn_d - locals.var_sp_s_eta);
            let assign44250_e57193: f64 = (locals.var_sp_s_a / locals.var_gf2);
            let assign44250_e57194: f64 = (assign44250_e57193).ln();
            let assign44250_e57195: f64 = (assign44250_e57190 + assign44250_e57194);
            (locals.var_sp_s_tau, locals.var_sp_s_tau_dn5, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, ) = (assign44250_e57195, ((locals.var_xn_d_dn5 - locals.var_sp_s_eta_dn5) + ((((locals.var_sp_s_a_dn5 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn5)) / (locals.var_gf2 * locals.var_gf2)) / assign44250_e57193)), ((locals.var_xn_d_dn6 - locals.var_sp_s_eta_dn6) + ((((locals.var_sp_s_a_dn6 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn6)) / (locals.var_gf2 * locals.var_gf2)) / assign44250_e57193)), ((locals.var_xn_d_dn7 - locals.var_sp_s_eta_dn7) + ((((locals.var_sp_s_a_dn7 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn7)) / (locals.var_gf2 * locals.var_gf2)) / assign44250_e57193)), ((locals.var_xn_d_dn8 - locals.var_sp_s_eta_dn8) + ((((locals.var_sp_s_a_dn8 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn8)) / (locals.var_gf2 * locals.var_gf2)) / assign44250_e57193)), );
            locals.var_sp_s_tau_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44260_e57204: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
            (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, ) = (assign44260_e57204, (locals.var_sp_s_a_dn5 + locals.var_sp_s_c_dn5), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), );
            locals.var_nu_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44270_e57213: f64 = (locals.var_nu * locals.var_nu);
            let assign44270_e57218: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
            let assign44270_e57219: f64 = (0.5 * assign44270_e57218);
            let assign44270_e57222: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
            let assign44270_e57223: f64 = (assign44270_e57219 - assign44270_e57222);
            let assign44270_e57224: f64 = (locals.var_sp_s_tau * assign44270_e57223);
            let assign44270_e57225: f64 = (assign44270_e57213 + assign44270_e57224);
            (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, ) = (assign44270_e57225, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau_dn5 * assign44270_e57223) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5))) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign44270_e57223) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign44270_e57223) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign44270_e57223) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))), );
            locals.var_mutau_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44280_e57235: f64 = (locals.var_sp_s_a * locals.var_nu);
            let assign44280_e57237: f64 = (assign44280_e57235 * locals.var_sp_s_tau);
            let assign44280_e57241: f64 = (locals.var_nu / locals.var_mutau);
            let assign44280_e57243: f64 = (assign44280_e57241 * locals.var_sp_s_tau);
            let assign44280_e57245: f64 = (assign44280_e57243 * locals.var_sp_s_tau);
            let assign44280_e57247: f64 = (assign44280_e57245 * locals.var_sp_s_c);
            let assign44280_e57250: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
            let assign44280_e57252: f64 = (assign44280_e57250 * 0.3333333333333333);
            let assign44280_e57255: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
            let assign44280_e57256: f64 = (assign44280_e57252 - assign44280_e57255);
            let assign44280_e57257: f64 = (assign44280_e57247 * assign44280_e57256);
            let assign44280_e57258: f64 = (locals.var_mutau + assign44280_e57257);
            let assign44280_e57259: f64 = (assign44280_e57237 / assign44280_e57258);
            let assign44280_e57260: f64 = (locals.var_sp_s_eta + assign44280_e57259);
            (locals.var_sp_s_x0, locals.var_sp_s_x0_dn5, locals.var_sp_s_x0_dn6, locals.var_sp_s_x0_dn7, locals.var_sp_s_x0_dn8, ) = (assign44280_e57260, (locals.var_sp_s_eta_dn5 + (((((((locals.var_sp_s_a_dn5 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn5)) * locals.var_sp_s_tau) + (assign44280_e57235 * locals.var_sp_s_tau_dn5)) * assign44280_e57258) - (assign44280_e57237 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44280_e57241 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_tau) + (assign44280_e57243 * locals.var_sp_s_tau_dn5)) * locals.var_sp_s_c) + (assign44280_e57245 * locals.var_sp_s_c_dn5)) * assign44280_e57256) + (assign44280_e57247 * ((((locals.var_sp_s_c_dn5 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn5 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn5)))))))) / (assign44280_e57258 * assign44280_e57258))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign44280_e57235 * locals.var_sp_s_tau_dn6)) * assign44280_e57258) - (assign44280_e57237 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44280_e57241 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign44280_e57243 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign44280_e57245 * locals.var_sp_s_c_dn6)) * assign44280_e57256) + (assign44280_e57247 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))))) / (assign44280_e57258 * assign44280_e57258))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign44280_e57235 * locals.var_sp_s_tau_dn7)) * assign44280_e57258) - (assign44280_e57237 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44280_e57241 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign44280_e57243 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign44280_e57245 * locals.var_sp_s_c_dn7)) * assign44280_e57256) + (assign44280_e57247 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))))) / (assign44280_e57258 * assign44280_e57258))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign44280_e57235 * locals.var_sp_s_tau_dn8)) * assign44280_e57258) - (assign44280_e57237 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44280_e57241 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign44280_e57243 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign44280_e57245 * locals.var_sp_s_c_dn8)) * assign44280_e57256) + (assign44280_e57247 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))))) / (assign44280_e57258 * assign44280_e57258))), );
            locals.var_sp_s_x0_rv = 0.0;
        }

        let assign44290_e57265: f64 = if locals.var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign44290_e57265;
        locals.var_guard1206_rv = 0.0;

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 != 0.0)) {
            let assign44300_e57273: f64 = (locals.var_sp_s_x0).exp();
            (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, ) = (assign44300_e57273, (assign44300_e57273 * locals.var_sp_s_x0_dn5), (assign44300_e57273 * locals.var_sp_s_x0_dn6), (assign44300_e57273 * locals.var_sp_s_x0_dn7), (assign44300_e57273 * locals.var_sp_s_x0_dn8), );
            locals.var_sp_s_delta0_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 != 0.0)) {
            let assign44310_e57284: f64 = (1.0 / locals.var_sp_s_delta0);
            (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, ) = (assign44310_e57284, (-(locals.var_sp_s_delta0_dn5 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), );
            locals.var_sp_s_delta1_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 != 0.0)) {
            let assign44320_e57295: f64 = (locals.var_delta_nd * locals.var_sp_s_delta0);
            (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, ) = (assign44320_e57295, ((locals.var_delta_nd_dn5 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn5)), ((locals.var_delta_nd_dn6 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn6)), ((locals.var_delta_nd_dn7 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn7)), ((locals.var_delta_nd_dn8 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn8)), );
            locals.var_sp_s_delta0_rv = 0.0;
        }

        let assign44330_e57301: f64 = (locals.var_xn_d - 230.25850929940458);
        let assign44330_e57302: f64 = if locals.var_sp_s_x0 > assign44330_e57301 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign44330_e57302;
        locals.var_guard1207_rv = 0.0;

        if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
            let assign44340_e57314: f64 = (locals.var_sp_s_x0 - locals.var_xn_d);
            let assign44340_e57315: f64 = (assign44340_e57314).exp();
            (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, ) = (assign44340_e57315, (assign44340_e57315 * (locals.var_sp_s_x0_dn5 - locals.var_xn_d_dn5)), (assign44340_e57315 * (locals.var_sp_s_x0_dn6 - locals.var_xn_d_dn6)), (assign44340_e57315 * (locals.var_sp_s_x0_dn7 - locals.var_xn_d_dn7)), (assign44340_e57315 * (locals.var_sp_s_x0_dn8 - locals.var_xn_d_dn8)), );
            locals.var_sp_s_delta0_rv = 0.0;
        }

        if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
            let assign44350_e57329: f64 = (locals.var_delta_nd / locals.var_sp_s_delta0);
            (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, ) = (assign44350_e57329, (((locals.var_delta_nd_dn5 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn5)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn6 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn6)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn7 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn7)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn8 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn8)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), );
            locals.var_sp_s_delta1_rv = 0.0;
        }

        if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) {
            let assign44360_e57346: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
            let assign44360_e57348: f64 = (assign44360_e57346 - 230.25850929940458);
            let assign44360_e57353: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
            let assign44360_e57355: f64 = (assign44360_e57353 - 230.25850929940458);
            let assign44360_e57359: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
            let assign44360_e57361: f64 = (assign44360_e57359 - 230.25850929940458);
            let assign44360_e57363: f64 = (assign44360_e57361 * 0.3333333333333333);
            let assign44360_e57364: f64 = (1.0 + assign44360_e57363);
            let assign44360_e57365: f64 = (assign44360_e57355 * assign44360_e57364);
            let assign44360_e57366: f64 = (0.5 * assign44360_e57365);
            let assign44360_e57367: f64 = (1.0 + assign44360_e57366);
            let assign44360_e57368: f64 = (assign44360_e57348 * assign44360_e57367);
            let assign44360_e57369: f64 = (1.0 + assign44360_e57368);
            let assign44360_e57370: f64 = (1e-100 / assign44360_e57369);
            (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn5, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, ) = (assign44360_e57370, (-((1e-100 * (((locals.var_xn_d_dn5 - locals.var_sp_s_x0_dn5) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((locals.var_xn_d_dn5 - locals.var_sp_s_x0_dn5) * assign44360_e57364) + (assign44360_e57355 * ((locals.var_xn_d_dn5 - locals.var_sp_s_x0_dn5) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))), (-((1e-100 * (((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * assign44360_e57364) + (assign44360_e57355 * ((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))), (-((1e-100 * (((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * assign44360_e57364) + (assign44360_e57355 * ((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))), (-((1e-100 * (((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * assign44360_e57364) + (assign44360_e57355 * ((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))), );
            locals.var_sp_s_delta0_rv = 0.0;
        }

        if ((((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) {
            let assign44370_e57387: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
            let assign44370_e57392: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
            let assign44370_e57396: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
            let assign44370_e57398: f64 = (assign44370_e57396 * 0.3333333333333333);
            let assign44370_e57399: f64 = (1.0 + assign44370_e57398);
            let assign44370_e57400: f64 = (assign44370_e57392 * assign44370_e57399);
            let assign44370_e57401: f64 = (0.5 * assign44370_e57400);
            let assign44370_e57402: f64 = (1.0 + assign44370_e57401);
            let assign44370_e57403: f64 = (assign44370_e57387 * assign44370_e57402);
            let assign44370_e57404: f64 = (1.0 + assign44370_e57403);
            let assign44370_e57405: f64 = (1e-100 / assign44370_e57404);
            (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn5, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, ) = (assign44370_e57405, (-((1e-100 * ((locals.var_sp_s_x0_dn5 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((locals.var_sp_s_x0_dn5 * assign44370_e57399) + (assign44370_e57392 * (locals.var_sp_s_x0_dn5 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))), (-((1e-100 * ((locals.var_sp_s_x0_dn6 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((locals.var_sp_s_x0_dn6 * assign44370_e57399) + (assign44370_e57392 * (locals.var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))), (-((1e-100 * ((locals.var_sp_s_x0_dn7 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((locals.var_sp_s_x0_dn7 * assign44370_e57399) + (assign44370_e57392 * (locals.var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))), (-((1e-100 * ((locals.var_sp_s_x0_dn8 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((locals.var_sp_s_x0_dn8 * assign44370_e57399) + (assign44370_e57392 * (locals.var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))), );
            locals.var_sp_s_delta1_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44380_e57416: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
            let assign44380_e57417: f64 = (2.0 + assign44380_e57416);
            let assign44380_e57418: f64 = (1.0 / assign44380_e57417);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign44380_e57418, (-(((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) / (assign44380_e57417 * assign44380_e57417))), (-(((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) / (assign44380_e57417 * assign44380_e57417))), (-(((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) / (assign44380_e57417 * assign44380_e57417))), (-(((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) / (assign44380_e57417 * assign44380_e57417))), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44390_e57427: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
            let assign44390_e57429: f64 = (assign44390_e57427 * locals.var_sp_s_temp);
            (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn5, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, ) = (assign44390_e57429, ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn5)) * locals.var_sp_s_temp) + (assign44390_e57427 * locals.var_sp_s_temp_dn5)), ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) * locals.var_sp_s_temp) + (assign44390_e57427 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) * locals.var_sp_s_temp) + (assign44390_e57427 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) * locals.var_sp_s_temp) + (assign44390_e57427 * locals.var_sp_s_temp_dn8)), );
            locals.var_sp_s_xi0_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44400_e57439: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_temp);
            let assign44400_e57441: f64 = (assign44400_e57439 * locals.var_sp_s_temp);
            let assign44400_e57442: f64 = (4.0 * assign44400_e57441);
            (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn5, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, ) = (assign44400_e57442, (4.0 * ((((locals.var_sp_s_x0_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign44400_e57439 * locals.var_sp_s_temp_dn5))), (4.0 * ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign44400_e57439 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign44400_e57439 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign44400_e57439 * locals.var_sp_s_temp_dn8))), );
            locals.var_sp_s_xi1_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44410_e57451: f64 = (8.0 * locals.var_sp_s_temp);
            let assign44410_e57454: f64 = (12.0 * locals.var_sp_s_xi0);
            let assign44410_e57455: f64 = (assign44410_e57451 - assign44410_e57454);
            let assign44410_e57457: f64 = (assign44410_e57455 * locals.var_sp_s_temp);
            let assign44410_e57459: f64 = (assign44410_e57457 * locals.var_sp_s_temp);
            (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn5, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, ) = (assign44410_e57459, ((((((8.0 * locals.var_sp_s_temp_dn5) - (12.0 * locals.var_sp_s_xi0_dn5)) * locals.var_sp_s_temp) + (assign44410_e57455 * locals.var_sp_s_temp_dn5)) * locals.var_sp_s_temp) + (assign44410_e57457 * locals.var_sp_s_temp_dn5)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign44410_e57455 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign44410_e57457 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign44410_e57455 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign44410_e57457 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign44410_e57455 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign44410_e57457 * locals.var_sp_s_temp_dn8)), );
            locals.var_sp_s_xi2_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44420_e57468: f64 = (locals.var_xg - locals.var_sp_s_x0);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign44420_e57468, (locals.var_xg_dn5 - locals.var_sp_s_x0_dn5), (locals.var_xg_dn6 - locals.var_sp_s_x0_dn6), (locals.var_xg_dn7 - locals.var_sp_s_x0_dn7), (locals.var_xg_dn8 - locals.var_sp_s_x0_dn8), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44430_e57477: f64 = (2.0 * locals.var_sp_s_temp);
            let assign44430_e57481: f64 = (1.0 - locals.var_sp_s_delta1);
            let assign44430_e57483: f64 = (assign44430_e57481 + locals.var_sp_s_delta0);
            let assign44430_e57487: f64 = (1.0 + locals.var_sp_s_xi1);
            let assign44430_e57488: f64 = (locals.var_delta_nd * assign44430_e57487);
            let assign44430_e57489: f64 = (assign44430_e57483 - assign44430_e57488);
            let assign44430_e57490: f64 = (locals.var_gf2 * assign44430_e57489);
            let assign44430_e57491: f64 = (assign44430_e57477 + assign44430_e57490);
            (locals.var_sp_s_pc, locals.var_sp_s_pc_dn5, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, ) = (assign44430_e57491, ((2.0 * locals.var_sp_s_temp_dn5) + ((locals.var_gf2_dn5 * assign44430_e57489) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_nd_dn5 * assign44430_e57487) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn5)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign44430_e57489) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * assign44430_e57487) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign44430_e57489) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * assign44430_e57487) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign44430_e57489) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * assign44430_e57487) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn8)))))), );
            locals.var_sp_s_pc_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44440_e57500: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
            let assign44440_e57504: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_x0);
            let assign44440_e57506: f64 = (assign44440_e57504 - 1.0);
            let assign44440_e57508: f64 = (assign44440_e57506 + locals.var_sp_s_delta0);
            let assign44440_e57512: f64 = (locals.var_sp_s_x0 + 1.0);
            let assign44440_e57514: f64 = (assign44440_e57512 + locals.var_sp_s_xi0);
            let assign44440_e57515: f64 = (locals.var_delta_nd * assign44440_e57514);
            let assign44440_e57516: f64 = (assign44440_e57508 - assign44440_e57515);
            let assign44440_e57517: f64 = (locals.var_gf2 * assign44440_e57516);
            let assign44440_e57518: f64 = (assign44440_e57500 - assign44440_e57517);
            (locals.var_sp_s_qc, locals.var_sp_s_qc_dn5, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, ) = (assign44440_e57518, (((locals.var_sp_s_temp_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn5)) - ((locals.var_gf2_dn5 * assign44440_e57516) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_x0_dn5) + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_nd_dn5 * assign44440_e57514) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn5 + locals.var_sp_s_xi0_dn5))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign44440_e57516) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_x0_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * assign44440_e57514) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign44440_e57516) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_x0_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * assign44440_e57514) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign44440_e57516) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_x0_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * assign44440_e57514) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn8 + locals.var_sp_s_xi0_dn8))))))), );
            locals.var_sp_s_qc_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44450_e57529: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_delta0);
            let assign44450_e57532: f64 = (locals.var_delta_nd * locals.var_sp_s_xi2);
            let assign44450_e57533: f64 = (assign44450_e57529 - assign44450_e57532);
            let assign44450_e57534: f64 = (locals.var_gf2 * assign44450_e57533);
            let assign44450_e57535: f64 = (2.0 - assign44450_e57534);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign44450_e57535, (-((locals.var_gf2_dn5 * assign44450_e57533) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn5 + locals.var_sp_s_delta0_dn5) - ((locals.var_delta_nd_dn5 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn5)))))), (-((locals.var_gf2_dn6 * assign44450_e57533) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign44450_e57533) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign44450_e57533) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn8)))))), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44460_e57544: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
            let assign44460_e57548: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
            let assign44460_e57549: f64 = (2.0 * assign44460_e57548);
            let assign44460_e57550: f64 = (assign44460_e57544 - assign44460_e57549);
            (locals.var_sp_s_temp, locals.var_sp_s_temp_dn5, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, ) = (assign44460_e57550, (((locals.var_sp_s_pc_dn5 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn5)) - (2.0 * ((locals.var_sp_s_qc_dn5 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn5)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), );
            locals.var_sp_s_temp_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1205 == 0.0)) {
            let assign44470_e57562: f64 = (locals.var_sp_s_temp).sqrt();
            let assign44470_e57563: f64 = (locals.var_sp_s_pc + assign44470_e57562);
            let assign44470_e57564: f64 = (locals.var_sp_s_qc / assign44470_e57563);
            let assign44470_e57565: f64 = (2.0 * assign44470_e57564);
            let assign44470_e57566: f64 = (locals.var_sp_s_x0 + assign44470_e57565);
            (locals.var_x_d, locals.var_x_d_dn5, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8, ) = (assign44470_e57566, (locals.var_sp_s_x0_dn5 + (2.0 * (((locals.var_sp_s_qc_dn5 * assign44470_e57563) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn5 + (locals.var_sp_s_temp_dn5 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))), (locals.var_sp_s_x0_dn6 + (2.0 * (((locals.var_sp_s_qc_dn6 * assign44470_e57563) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))), (locals.var_sp_s_x0_dn7 + (2.0 * (((locals.var_sp_s_qc_dn7 * assign44470_e57563) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))), (locals.var_sp_s_x0_dn8 + (2.0 * (((locals.var_sp_s_qc_dn8 * assign44470_e57563) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))), );
            locals.var_x_d_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44480_e57572: f64 = (locals.var_x_d - locals.var_x_s);
            (locals.var_x_ds, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, ) = (assign44480_e57572, (locals.var_x_d_dn5 - locals.var_x_s_dn5), (locals.var_x_d_dn6 - locals.var_x_s_dn6), (locals.var_x_d_dn7 - locals.var_x_s_dn7), (locals.var_x_d_dn8 - locals.var_x_s_dn8), );
            locals.var_x_ds_rv = 0.0;
        }

        let assign44490_e57577: f64 = if locals.var_x_ds < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1208 = assign44490_e57577;
        locals.var_guard1208_rv = 0.0;

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
            let assign44500_e57584: f64 = (locals.var_xg - locals.var_x_s);
            let assign44500_e57585: f64 = (2.0 * assign44500_e57584);
            let assign44500_e57589: f64 = (1.0 - locals.var_es);
            let assign44500_e57592: f64 = (locals.var_delta_1s * locals.var_k_ds);
            let assign44500_e57593: f64 = (assign44500_e57589 + assign44500_e57592);
            let assign44500_e57597: f64 = (1.0 + locals.var_xi1s);
            let assign44500_e57598: f64 = (locals.var_delta_nd * assign44500_e57597);
            let assign44500_e57599: f64 = (assign44500_e57593 - assign44500_e57598);
            let assign44500_e57600: f64 = (locals.var_gf2 * assign44500_e57599);
            let assign44500_e57601: f64 = (assign44500_e57585 + assign44500_e57600);
            (locals.var_pc, locals.var_pc_dn5, locals.var_pc_dn6, locals.var_pc_dn7, locals.var_pc_dn8, ) = (assign44500_e57601, ((2.0 * (locals.var_xg_dn5 - locals.var_x_s_dn5)) + ((locals.var_gf2_dn5 * assign44500_e57599) + (locals.var_gf2 * (((-locals.var_es_dn5) + ((locals.var_delta_1s_dn5 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn5))) - ((locals.var_delta_nd_dn5 * assign44500_e57597) + (locals.var_delta_nd * locals.var_xi1s_dn5)))))), ((2.0 * (locals.var_xg_dn6 - locals.var_x_s_dn6)) + ((locals.var_gf2_dn6 * assign44500_e57599) + (locals.var_gf2 * (((-locals.var_es_dn6) + ((locals.var_delta_1s_dn6 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn6))) - ((locals.var_delta_nd_dn6 * assign44500_e57597) + (locals.var_delta_nd * locals.var_xi1s_dn6)))))), ((2.0 * (locals.var_xg_dn7 - locals.var_x_s_dn7)) + ((locals.var_gf2_dn7 * assign44500_e57599) + (locals.var_gf2 * (((-locals.var_es_dn7) + ((locals.var_delta_1s_dn7 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn7))) - ((locals.var_delta_nd_dn7 * assign44500_e57597) + (locals.var_delta_nd * locals.var_xi1s_dn7)))))), ((2.0 * (locals.var_xg_dn8 - locals.var_x_s_dn8)) + ((locals.var_gf2_dn8 * assign44500_e57599) + (locals.var_gf2 * (((-locals.var_es_dn8) + ((locals.var_delta_1s_dn8 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn8))) - ((locals.var_delta_nd_dn8 * assign44500_e57597) + (locals.var_delta_nd * locals.var_xi1s_dn8)))))), );
            locals.var_pc_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
            let assign44510_e57610: f64 = (1.0 - locals.var_k_ds);
            let assign44510_e57611: f64 = (locals.var_gf2 * assign44510_e57610);
            let assign44510_e57613: f64 = (assign44510_e57611 * locals.var_ds);
            (locals.var_qc, locals.var_qc_dn5, locals.var_qc_dn6, locals.var_qc_dn7, locals.var_qc_dn8, ) = (assign44510_e57613, ((((locals.var_gf2_dn5 * assign44510_e57610) + (locals.var_gf2 * (-locals.var_k_ds_dn5))) * locals.var_ds) + (assign44510_e57611 * locals.var_ds_dn5)), ((((locals.var_gf2_dn6 * assign44510_e57610) + (locals.var_gf2 * (-locals.var_k_ds_dn6))) * locals.var_ds) + (assign44510_e57611 * locals.var_ds_dn6)), ((((locals.var_gf2_dn7 * assign44510_e57610) + (locals.var_gf2 * (-locals.var_k_ds_dn7))) * locals.var_ds) + (assign44510_e57611 * locals.var_ds_dn7)), ((((locals.var_gf2_dn8 * assign44510_e57610) + (locals.var_gf2 * (-locals.var_k_ds_dn8))) * locals.var_ds) + (assign44510_e57611 * locals.var_ds_dn8)), );
            locals.var_qc_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_17(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
            let assign44520_e57624: f64 = (locals.var_delta_1s * locals.var_k_ds);
            let assign44520_e57625: f64 = (locals.var_es + assign44520_e57624);
            let assign44520_e57628: f64 = (locals.var_delta_nd * locals.var_xi2s);
            let assign44520_e57629: f64 = (assign44520_e57625 - assign44520_e57628);
            let assign44520_e57630: f64 = (locals.var_gf2 * assign44520_e57629);
            let assign44520_e57631: f64 = (2.0 - assign44520_e57630);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign44520_e57631, (-((locals.var_gf2_dn5 * assign44520_e57629) + (locals.var_gf2 * ((locals.var_es_dn5 + ((locals.var_delta_1s_dn5 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn5))) - ((locals.var_delta_nd_dn5 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn5)))))), (-((locals.var_gf2_dn6 * assign44520_e57629) + (locals.var_gf2 * ((locals.var_es_dn6 + ((locals.var_delta_1s_dn6 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn6))) - ((locals.var_delta_nd_dn6 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn6)))))), (-((locals.var_gf2_dn7 * assign44520_e57629) + (locals.var_gf2 * ((locals.var_es_dn7 + ((locals.var_delta_1s_dn7 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn7))) - ((locals.var_delta_nd_dn7 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn7)))))), (-((locals.var_gf2_dn8 * assign44520_e57629) + (locals.var_gf2 * ((locals.var_es_dn8 + ((locals.var_delta_1s_dn8 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn8))) - ((locals.var_delta_nd_dn8 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn8)))))), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
            let assign44530_e57639: f64 = (locals.var_pc * locals.var_pc);
            let assign44530_e57643: f64 = (locals.var_temp__blk936 * locals.var_qc);
            let assign44530_e57644: f64 = (2.0 * assign44530_e57643);
            let assign44530_e57645: f64 = (assign44530_e57639 - assign44530_e57644);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign44530_e57645, (((locals.var_pc_dn5 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn5)) - (2.0 * ((locals.var_temp__blk936_dn5 * locals.var_qc) + (locals.var_temp__blk936 * locals.var_qc_dn5)))), (((locals.var_pc_dn6 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn6)) - (2.0 * ((locals.var_temp__blk936_dn6 * locals.var_qc) + (locals.var_temp__blk936 * locals.var_qc_dn6)))), (((locals.var_pc_dn7 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn7)) - (2.0 * ((locals.var_temp__blk936_dn7 * locals.var_qc) + (locals.var_temp__blk936 * locals.var_qc_dn7)))), (((locals.var_pc_dn8 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn8)) - (2.0 * ((locals.var_temp__blk936_dn8 * locals.var_qc) + (locals.var_temp__blk936 * locals.var_qc_dn8)))), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
            let assign44540_e57655: f64 = (locals.var_temp__blk936).sqrt();
            let assign44540_e57656: f64 = (locals.var_pc + assign44540_e57655);
            let assign44540_e57657: f64 = (locals.var_qc / assign44540_e57656);
            let assign44540_e57658: f64 = (2.0 * assign44540_e57657);
            (locals.var_x_ds, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, ) = (assign44540_e57658, (2.0 * (((locals.var_qc_dn5 * assign44540_e57656) - (locals.var_qc * (locals.var_pc_dn5 + (locals.var_temp__blk936_dn5 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))), (2.0 * (((locals.var_qc_dn6 * assign44540_e57656) - (locals.var_qc * (locals.var_pc_dn6 + (locals.var_temp__blk936_dn6 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))), (2.0 * (((locals.var_qc_dn7 * assign44540_e57656) - (locals.var_qc * (locals.var_pc_dn7 + (locals.var_temp__blk936_dn7 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))), (2.0 * (((locals.var_qc_dn8 * assign44540_e57656) - (locals.var_qc * (locals.var_pc_dn8 + (locals.var_temp__blk936_dn8 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))), );
            locals.var_x_ds_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1208 != 0.0)) {
            let assign44550_e57666: f64 = (locals.var_x_s + locals.var_x_ds);
            (locals.var_x_d, locals.var_x_d_dn5, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8, ) = (assign44550_e57666, (locals.var_x_s_dn5 + locals.var_x_ds_dn5), (locals.var_x_s_dn6 + locals.var_x_ds_dn6), (locals.var_x_s_dn7 + locals.var_x_ds_dn7), (locals.var_x_s_dn8 + locals.var_x_ds_dn8), );
            locals.var_x_d_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44560_e57672: f64 = (locals.var_x_ds * locals.var_phit1);
            (locals.var_dps, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, ) = (assign44560_e57672, ((locals.var_x_ds_dn5 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn5)), ((locals.var_x_ds_dn6 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn6)), ((locals.var_x_ds_dn7 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn7)), ((locals.var_x_ds_dn8 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn8)), );
            locals.var_dps_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44570_e57678: f64 = (locals.var_x_d * locals.var_x_d);
            let assign44570_e57682: f64 = (locals.var_x_d * locals.var_x_d);
            let assign44570_e57683: f64 = (2.0 + assign44570_e57682);
            let assign44570_e57684: f64 = (assign44570_e57678 / assign44570_e57683);
            (locals.var_xi0d, locals.var_xi0d_dn5, locals.var_xi0d_dn6, locals.var_xi0d_dn7, locals.var_xi0d_dn8, ) = (assign44570_e57684, (((((locals.var_x_d_dn5 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn5)) * assign44570_e57683) - (assign44570_e57678 * ((locals.var_x_d_dn5 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn5)))) / (assign44570_e57683 * assign44570_e57683)), (((((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)) * assign44570_e57683) - (assign44570_e57678 * ((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)))) / (assign44570_e57683 * assign44570_e57683)), (((((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)) * assign44570_e57683) - (assign44570_e57678 * ((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)))) / (assign44570_e57683 * assign44570_e57683)), (((((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)) * assign44570_e57683) - (assign44570_e57678 * ((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)))) / (assign44570_e57683 * assign44570_e57683)), );
            locals.var_xi0d_rv = 0.0;
        }

        let assign44580_e57689: f64 = if locals.var_x_d < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign44580_e57689;
        locals.var_guard1209_rv = 0.0;

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) {
            let assign44590_e57694: f64 = (-locals.var_x_d);
            let assign44590_e57695: f64 = (assign44590_e57694).exp();
            (locals.var_ed, locals.var_ed_dn5, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, ) = (assign44590_e57695, (assign44590_e57695 * (-locals.var_x_d_dn5)), (assign44590_e57695 * (-locals.var_x_d_dn6)), (assign44590_e57695 * (-locals.var_x_d_dn7)), (assign44590_e57695 * (-locals.var_x_d_dn8)), );
            locals.var_ed_rv = 0.0;
        }

        let assign44600_e57700: f64 = if locals.var_x_d < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign44600_e57700;
        locals.var_guard1210_rv = 0.0;

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
            let assign44610_e57709: f64 = (locals.var_x_d * locals.var_x_d);
            let assign44610_e57716: f64 = (0.25 * locals.var_x_d);
            let assign44610_e57717: f64 = (1.0 - assign44610_e57716);
            let assign44610_e57718: f64 = (locals.var_x_d * assign44610_e57717);
            let assign44610_e57719: f64 = (0.3333333333333333 * assign44610_e57718);
            let assign44610_e57720: f64 = (1.0 - assign44610_e57719);
            let assign44610_e57721: f64 = (assign44610_e57709 * assign44610_e57720);
            let assign44610_e57722: f64 = (0.5 * assign44610_e57721);
            (locals.var_pd, locals.var_pd_dn5, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, ) = (assign44610_e57722, (0.5 * ((((locals.var_x_d_dn5 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn5)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((locals.var_x_d_dn5 * assign44610_e57717) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn5))))))))), (0.5 * ((((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((locals.var_x_d_dn6 * assign44610_e57717) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn6))))))))), (0.5 * ((((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((locals.var_x_d_dn7 * assign44610_e57717) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn7))))))))), (0.5 * ((((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((locals.var_x_d_dn8 * assign44610_e57717) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn8))))))))), );
            locals.var_pd_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
            let assign44620_e57736: f64 = (0.25 * locals.var_x_d);
            let assign44620_e57737: f64 = (1.0 - assign44620_e57736);
            let assign44620_e57738: f64 = (locals.var_x_d * assign44620_e57737);
            let assign44620_e57739: f64 = (0.3333333333333333 * assign44620_e57738);
            let assign44620_e57740: f64 = (1.0 - assign44620_e57739);
            let assign44620_e57741: f64 = (assign44620_e57740).sqrt();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign44620_e57741, ((-(0.3333333333333333 * ((locals.var_x_d_dn5 * assign44620_e57737) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn5)))))) / (2.0 * assign44620_e57741)), ((-(0.3333333333333333 * ((locals.var_x_d_dn6 * assign44620_e57737) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn6)))))) / (2.0 * assign44620_e57741)), ((-(0.3333333333333333 * ((locals.var_x_d_dn7 * assign44620_e57737) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn7)))))) / (2.0 * assign44620_e57741)), ((-(0.3333333333333333 * ((locals.var_x_d_dn8 * assign44620_e57737) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn8)))))) / (2.0 * assign44620_e57741)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
            let assign44630_e57752: f64 = (locals.var_x_d * locals.var_temp__blk936);
            let assign44630_e57753: f64 = (0.7071067811865475 * assign44630_e57752);
            (locals.var_sqd, locals.var_sqd_dn5, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8, ) = (assign44630_e57753, (0.7071067811865475 * ((locals.var_x_d_dn5 * locals.var_temp__blk936) + (locals.var_x_d * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_d_dn6 * locals.var_temp__blk936) + (locals.var_x_d * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_d_dn7 * locals.var_temp__blk936) + (locals.var_x_d * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_d_dn8 * locals.var_temp__blk936) + (locals.var_x_d * locals.var_temp__blk936_dn8))), );
            locals.var_sqd_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
            let assign44640_e57763: f64 = (0.16666666666666666 * locals.var_delta_nd);
            let assign44640_e57765: f64 = (assign44640_e57763 * locals.var_x_d);
            let assign44640_e57767: f64 = (assign44640_e57765 * locals.var_x_d);
            let assign44640_e57769: f64 = (assign44640_e57767 * locals.var_x_d);
            let assign44640_e57773: f64 = (1.75 * locals.var_x_d);
            let assign44640_e57774: f64 = (1.0 + assign44640_e57773);
            let assign44640_e57775: f64 = (assign44640_e57769 * assign44640_e57774);
            (locals.var_dd, locals.var_dd_dn5, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, ) = (assign44640_e57775, (((((((((0.16666666666666666 * locals.var_delta_nd_dn5) * locals.var_x_d) + (assign44640_e57763 * locals.var_x_d_dn5)) * locals.var_x_d) + (assign44640_e57765 * locals.var_x_d_dn5)) * locals.var_x_d) + (assign44640_e57767 * locals.var_x_d_dn5)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * locals.var_x_d_dn5))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn6) * locals.var_x_d) + (assign44640_e57763 * locals.var_x_d_dn6)) * locals.var_x_d) + (assign44640_e57765 * locals.var_x_d_dn6)) * locals.var_x_d) + (assign44640_e57767 * locals.var_x_d_dn6)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * locals.var_x_d_dn6))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn7) * locals.var_x_d) + (assign44640_e57763 * locals.var_x_d_dn7)) * locals.var_x_d) + (assign44640_e57765 * locals.var_x_d_dn7)) * locals.var_x_d) + (assign44640_e57767 * locals.var_x_d_dn7)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * locals.var_x_d_dn7))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn8) * locals.var_x_d) + (assign44640_e57763 * locals.var_x_d_dn8)) * locals.var_x_d) + (assign44640_e57765 * locals.var_x_d_dn8)) * locals.var_x_d) + (assign44640_e57767 * locals.var_x_d_dn8)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * locals.var_x_d_dn8))), );
            locals.var_dd_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
            let assign44650_e57786: f64 = (locals.var_x_d - 1.0);
            let assign44650_e57788: f64 = (assign44650_e57786 + locals.var_ed);
            (locals.var_pd, locals.var_pd_dn5, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, ) = (assign44650_e57788, (locals.var_x_d_dn5 + locals.var_ed_dn5), (locals.var_x_d_dn6 + locals.var_ed_dn6), (locals.var_x_d_dn7 + locals.var_ed_dn7), (locals.var_x_d_dn8 + locals.var_ed_dn8), );
            locals.var_pd_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
            let assign44660_e57798: f64 = (locals.var_pd).sqrt();
            (locals.var_sqd, locals.var_sqd_dn5, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8, ) = (assign44660_e57798, (locals.var_pd_dn5 / (2.0 * assign44660_e57798)), (locals.var_pd_dn6 / (2.0 * assign44660_e57798)), (locals.var_pd_dn7 / (2.0 * assign44660_e57798)), (locals.var_pd_dn8 / (2.0 * assign44660_e57798)), );
            locals.var_sqd_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
            let assign44670_e57810: f64 = (1.0 / locals.var_ed);
            let assign44670_e57812: f64 = (assign44670_e57810 - locals.var_x_d);
            let assign44670_e57814: f64 = (assign44670_e57812 - 1.0);
            let assign44670_e57816: f64 = (assign44670_e57814 - locals.var_xi0d);
            let assign44670_e57817: f64 = (locals.var_delta_nd * assign44670_e57816);
            (locals.var_dd, locals.var_dd_dn5, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, ) = (assign44670_e57817, ((locals.var_delta_nd_dn5 * assign44670_e57816) + (locals.var_delta_nd * (((-(locals.var_ed_dn5 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn5) - locals.var_xi0d_dn5))), ((locals.var_delta_nd_dn6 * assign44670_e57816) + (locals.var_delta_nd * (((-(locals.var_ed_dn6 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn6) - locals.var_xi0d_dn6))), ((locals.var_delta_nd_dn7 * assign44670_e57816) + (locals.var_delta_nd * (((-(locals.var_ed_dn7 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn7) - locals.var_xi0d_dn7))), ((locals.var_delta_nd_dn8 * assign44670_e57816) + (locals.var_delta_nd * (((-(locals.var_ed_dn8 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn8) - locals.var_xi0d_dn8))), );
            locals.var_dd_rv = 0.0;
        }

        let assign44680_e57823: f64 = (locals.var_xn_d - 230.25850929940458);
        let assign44680_e57824: f64 = if locals.var_x_d > assign44680_e57823 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign44680_e57824;
        locals.var_guard1211_rv = 0.0;

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 != 0.0)) {
            let assign44690_e57833: f64 = (locals.var_x_d - locals.var_xn_d);
            let assign44690_e57834: f64 = (assign44690_e57833).exp();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign44690_e57834, (assign44690_e57834 * (locals.var_x_d_dn5 - locals.var_xn_d_dn5)), (assign44690_e57834 * (locals.var_x_d_dn6 - locals.var_xn_d_dn6)), (assign44690_e57834 * (locals.var_x_d_dn7 - locals.var_xn_d_dn7)), (assign44690_e57834 * (locals.var_x_d_dn8 - locals.var_xn_d_dn8)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 != 0.0)) {
            let assign44700_e57845: f64 = (locals.var_delta_nd / locals.var_temp__blk936);
            (locals.var_ed, locals.var_ed_dn5, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, ) = (assign44700_e57845, (((locals.var_delta_nd_dn5 * locals.var_temp__blk936) - (locals.var_delta_nd * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd_dn6 * locals.var_temp__blk936) - (locals.var_delta_nd * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd_dn7 * locals.var_temp__blk936) - (locals.var_delta_nd * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd_dn8 * locals.var_temp__blk936) - (locals.var_delta_nd * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), );
            locals.var_ed_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 != 0.0)) {
            let assign44710_e57858: f64 = (locals.var_x_d + 1.0);
            let assign44710_e57860: f64 = (assign44710_e57858 + locals.var_xi0d);
            let assign44710_e57861: f64 = (locals.var_delta_nd * assign44710_e57860);
            let assign44710_e57862: f64 = (locals.var_temp__blk936 - assign44710_e57861);
            (locals.var_dd, locals.var_dd_dn5, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, ) = (assign44710_e57862, (locals.var_temp__blk936_dn5 - ((locals.var_delta_nd_dn5 * assign44710_e57860) + (locals.var_delta_nd * (locals.var_x_d_dn5 + locals.var_xi0d_dn5)))), (locals.var_temp__blk936_dn6 - ((locals.var_delta_nd_dn6 * assign44710_e57860) + (locals.var_delta_nd * (locals.var_x_d_dn6 + locals.var_xi0d_dn6)))), (locals.var_temp__blk936_dn7 - ((locals.var_delta_nd_dn7 * assign44710_e57860) + (locals.var_delta_nd * (locals.var_x_d_dn7 + locals.var_xi0d_dn7)))), (locals.var_temp__blk936_dn8 - ((locals.var_delta_nd_dn8 * assign44710_e57860) + (locals.var_delta_nd * (locals.var_x_d_dn8 + locals.var_xi0d_dn8)))), );
            locals.var_dd_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 == 0.0)) {
            let assign44720_e57876: f64 = (locals.var_x_d - 230.25850929940458);
            let assign44720_e57881: f64 = (locals.var_x_d - 230.25850929940458);
            let assign44720_e57885: f64 = (locals.var_x_d - 230.25850929940458);
            let assign44720_e57887: f64 = (assign44720_e57885 * 0.3333333333333333);
            let assign44720_e57888: f64 = (1.0 + assign44720_e57887);
            let assign44720_e57889: f64 = (assign44720_e57881 * assign44720_e57888);
            let assign44720_e57890: f64 = (0.5 * assign44720_e57889);
            let assign44720_e57891: f64 = (1.0 + assign44720_e57890);
            let assign44720_e57892: f64 = (assign44720_e57876 * assign44720_e57891);
            let assign44720_e57893: f64 = (1.0 + assign44720_e57892);
            let assign44720_e57894: f64 = (1e-100 / assign44720_e57893);
            (locals.var_ed, locals.var_ed_dn5, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, ) = (assign44720_e57894, (-((1e-100 * ((locals.var_x_d_dn5 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((locals.var_x_d_dn5 * assign44720_e57888) + (assign44720_e57881 * (locals.var_x_d_dn5 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))), (-((1e-100 * ((locals.var_x_d_dn6 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((locals.var_x_d_dn6 * assign44720_e57888) + (assign44720_e57881 * (locals.var_x_d_dn6 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))), (-((1e-100 * ((locals.var_x_d_dn7 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((locals.var_x_d_dn7 * assign44720_e57888) + (assign44720_e57881 * (locals.var_x_d_dn7 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))), (-((1e-100 * ((locals.var_x_d_dn8 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((locals.var_x_d_dn8 * assign44720_e57888) + (assign44720_e57881 * (locals.var_x_d_dn8 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))), );
            locals.var_ed_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 == 0.0)) {
            let assign44730_e57908: f64 = (locals.var_xn_d - locals.var_x_d);
            let assign44730_e57910: f64 = (assign44730_e57908 - 230.25850929940458);
            let assign44730_e57915: f64 = (locals.var_xn_d - locals.var_x_d);
            let assign44730_e57917: f64 = (assign44730_e57915 - 230.25850929940458);
            let assign44730_e57921: f64 = (locals.var_xn_d - locals.var_x_d);
            let assign44730_e57923: f64 = (assign44730_e57921 - 230.25850929940458);
            let assign44730_e57925: f64 = (assign44730_e57923 * 0.3333333333333333);
            let assign44730_e57926: f64 = (1.0 + assign44730_e57925);
            let assign44730_e57927: f64 = (assign44730_e57917 * assign44730_e57926);
            let assign44730_e57928: f64 = (0.5 * assign44730_e57927);
            let assign44730_e57929: f64 = (1.0 + assign44730_e57928);
            let assign44730_e57930: f64 = (assign44730_e57910 * assign44730_e57929);
            let assign44730_e57931: f64 = (1.0 + assign44730_e57930);
            let assign44730_e57932: f64 = (1e-100 / assign44730_e57931);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign44730_e57932, (-((1e-100 * (((locals.var_xn_d_dn5 - locals.var_x_d_dn5) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((locals.var_xn_d_dn5 - locals.var_x_d_dn5) * assign44730_e57926) + (assign44730_e57917 * ((locals.var_xn_d_dn5 - locals.var_x_d_dn5) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))), (-((1e-100 * (((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * assign44730_e57926) + (assign44730_e57917 * ((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))), (-((1e-100 * (((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * assign44730_e57926) + (assign44730_e57917 * ((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))), (-((1e-100 * (((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * assign44730_e57926) + (assign44730_e57917 * ((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1211 == 0.0)) {
            let assign44740_e57946: f64 = (locals.var_x_d + 1.0);
            let assign44740_e57948: f64 = (assign44740_e57946 + locals.var_xi0d);
            let assign44740_e57949: f64 = (locals.var_delta_nd * assign44740_e57948);
            let assign44740_e57950: f64 = (locals.var_temp__blk936 - assign44740_e57949);
            (locals.var_dd, locals.var_dd_dn5, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, ) = (assign44740_e57950, (locals.var_temp__blk936_dn5 - ((locals.var_delta_nd_dn5 * assign44740_e57948) + (locals.var_delta_nd * (locals.var_x_d_dn5 + locals.var_xi0d_dn5)))), (locals.var_temp__blk936_dn6 - ((locals.var_delta_nd_dn6 * assign44740_e57948) + (locals.var_delta_nd * (locals.var_x_d_dn6 + locals.var_xi0d_dn6)))), (locals.var_temp__blk936_dn7 - ((locals.var_delta_nd_dn7 * assign44740_e57948) + (locals.var_delta_nd * (locals.var_x_d_dn7 + locals.var_xi0d_dn7)))), (locals.var_temp__blk936_dn8 - ((locals.var_delta_nd_dn8 * assign44740_e57948) + (locals.var_delta_nd * (locals.var_x_d_dn8 + locals.var_xi0d_dn8)))), );
            locals.var_dd_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) {
            let assign44750_e57959: f64 = (locals.var_x_d - 1.0);
            let assign44750_e57961: f64 = (assign44750_e57959 + locals.var_ed);
            (locals.var_pd, locals.var_pd_dn5, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, ) = (assign44750_e57961, (locals.var_x_d_dn5 + locals.var_ed_dn5), (locals.var_x_d_dn6 + locals.var_ed_dn6), (locals.var_x_d_dn7 + locals.var_ed_dn7), (locals.var_x_d_dn8 + locals.var_ed_dn8), );
            locals.var_pd_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1209 == 0.0)) {
            let assign44760_e57969: f64 = (locals.var_pd).sqrt();
            (locals.var_sqd, locals.var_sqd_dn5, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8, ) = (assign44760_e57969, (locals.var_pd_dn5 / (2.0 * assign44760_e57969)), (locals.var_pd_dn6 / (2.0 * assign44760_e57969)), (locals.var_pd_dn7 / (2.0 * assign44760_e57969)), (locals.var_pd_dn8 / (2.0 * assign44760_e57969)), );
            locals.var_sqd_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44770_e57975: f64 = (locals.var_sqd * locals.var_gf);
            let assign44770_e57977: f64 = (assign44770_e57975 * locals.var_phit1);
            (locals.var_qbd, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, ) = (assign44770_e57977, ((((locals.var_sqd_dn5 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn5)) * locals.var_phit1) + (assign44770_e57975 * locals.var_phit1_dn5)), ((((locals.var_sqd_dn6 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn6)) * locals.var_phit1) + (assign44770_e57975 * locals.var_phit1_dn6)), ((((locals.var_sqd_dn7 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn7)) * locals.var_phit1) + (assign44770_e57975 * locals.var_phit1_dn7)), ((((locals.var_sqd_dn8 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn8)) * locals.var_phit1) + (assign44770_e57975 * locals.var_phit1_dn8)), );
            locals.var_qbd_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44780_e57984: f64 = (locals.var_x_s + locals.var_x_d);
            let assign44780_e57985: f64 = (0.5 * assign44780_e57984);
            (locals.var_x_m, locals.var_x_m_dn5, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, ) = (assign44780_e57985, (0.5 * (locals.var_x_s_dn5 + locals.var_x_d_dn5)), (0.5 * (locals.var_x_s_dn6 + locals.var_x_d_dn6)), (0.5 * (locals.var_x_s_dn7 + locals.var_x_d_dn7)), (0.5 * (locals.var_x_s_dn8 + locals.var_x_d_dn8)), );
            locals.var_x_m_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            (locals.var_em, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_em_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44800_e57995: f64 = (locals.var_ed * locals.var_es);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign44800_e57995, ((locals.var_ed_dn5 * locals.var_es) + (locals.var_ed * locals.var_es_dn5)), ((locals.var_ed_dn6 * locals.var_es) + (locals.var_ed * locals.var_es_dn6)), ((locals.var_ed_dn7 * locals.var_es) + (locals.var_ed * locals.var_es_dn7)), ((locals.var_ed_dn8 * locals.var_es) + (locals.var_ed * locals.var_es_dn8)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        let assign44810_e58000: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign44810_e58000;
        locals.var_guard1212_rv = 0.0;

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1212 != 0.0)) {
            let assign44820_e58005: f64 = (locals.var_temp__blk936).sqrt();
            (locals.var_em, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, ) = (assign44820_e58005, (locals.var_temp__blk936_dn5 / (2.0 * assign44820_e58005)), (locals.var_temp__blk936_dn6 / (2.0 * assign44820_e58005)), (locals.var_temp__blk936_dn7 / (2.0 * assign44820_e58005)), (locals.var_temp__blk936_dn8 / (2.0 * assign44820_e58005)), );
            locals.var_em_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44830_e58012: f64 = (locals.var_ds + locals.var_dd);
            let assign44830_e58013: f64 = (0.5 * assign44830_e58012);
            (locals.var_d_bar, locals.var_d_bar_dn5, locals.var_d_bar_dn6, locals.var_d_bar_dn7, locals.var_d_bar_dn8, ) = (assign44830_e58013, (0.5 * (locals.var_ds_dn5 + locals.var_dd_dn5)), (0.5 * (locals.var_ds_dn6 + locals.var_dd_dn6)), (0.5 * (locals.var_ds_dn7 + locals.var_dd_dn7)), (0.5 * (locals.var_ds_dn8 + locals.var_dd_dn8)), );
            locals.var_d_bar_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign44840_e58021: f64 = (locals.var_x_ds * locals.var_x_ds);
            let assign44840_e58025: f64 = (2.0 * locals.var_inv_gf2);
            let assign44840_e58026: f64 = (locals.var_em - assign44840_e58025);
            let assign44840_e58027: f64 = (assign44840_e58021 * assign44840_e58026);
            let assign44840_e58028: f64 = (0.125 * assign44840_e58027);
            let assign44840_e58029: f64 = (locals.var_d_bar + assign44840_e58028);
            (locals.var_dm, locals.var_dm_dn5, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, ) = (assign44840_e58029, (locals.var_d_bar_dn5 + (0.125 * ((((locals.var_x_ds_dn5 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn5)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn5 - (2.0 * locals.var_inv_gf2_dn5)))))), (locals.var_d_bar_dn6 + (0.125 * ((((locals.var_x_ds_dn6 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn6)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn6 - (2.0 * locals.var_inv_gf2_dn6)))))), (locals.var_d_bar_dn7 + (0.125 * ((((locals.var_x_ds_dn7 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn7)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn7 - (2.0 * locals.var_inv_gf2_dn7)))))), (locals.var_d_bar_dn8 + (0.125 * ((((locals.var_x_ds_dn8 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn8)) * assign44840_e58026) + (assign44840_e58021 * (locals.var_em_dn8 - (2.0 * locals.var_inv_gf2_dn8)))))), );
            locals.var_dm_rv = 0.0;
        }

        let assign44850_e58034: f64 = if locals.var_x_m < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign44850_e58034;
        locals.var_guard1213_rv = 0.0;

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
            let assign44860_e58041: f64 = (locals.var_x_m * locals.var_x_m);
            let assign44860_e58048: f64 = (0.25 * locals.var_x_m);
            let assign44860_e58049: f64 = (1.0 - assign44860_e58048);
            let assign44860_e58050: f64 = (locals.var_x_m * assign44860_e58049);
            let assign44860_e58051: f64 = (0.3333333333333333 * assign44860_e58050);
            let assign44860_e58052: f64 = (1.0 - assign44860_e58051);
            let assign44860_e58053: f64 = (assign44860_e58041 * assign44860_e58052);
            let assign44860_e58054: f64 = (0.5 * assign44860_e58053);
            (locals.var_pm, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, ) = (assign44860_e58054, (0.5 * ((((locals.var_x_m_dn5 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn5)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn5 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn5))))))))), (0.5 * ((((locals.var_x_m_dn6 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn6)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn6 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn6))))))))), (0.5 * ((((locals.var_x_m_dn7 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn7)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn7 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn7))))))))), (0.5 * ((((locals.var_x_m_dn8 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn8)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((locals.var_x_m_dn8 * assign44860_e58049) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn8))))))))), );
            locals.var_pm_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
            let assign44870_e58063: f64 = (locals.var_dm + locals.var_pm);
            let assign44870_e58064: f64 = (assign44870_e58063).sqrt();
            let assign44870_e58065: f64 = (locals.var_gf * assign44870_e58064);
            (locals.var_xgm, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, ) = (assign44870_e58065, ((locals.var_gf_dn5 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign44870_e58064)))), ((locals.var_gf_dn6 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign44870_e58064)))), ((locals.var_gf_dn7 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign44870_e58064)))), ((locals.var_gf_dn8 * assign44870_e58064) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign44870_e58064)))), );
            locals.var_xgm_rv = 0.0;
        }

        let assign44880_e58070: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign44880_e58070;
        locals.var_guard1214_rv = 0.0;

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 != 0.0)) {
            let assign44890_e58080: f64 = (locals.var_kp * locals.var_xgm);
            let assign44890_e58081: f64 = (1.0 + assign44890_e58080);
            let assign44890_e58082: f64 = (assign44890_e58081).sqrt();
            let assign44890_e58083: f64 = (1.0 / assign44890_e58082);
            (locals.var_eta_p, locals.var_eta_p_dn5, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8, ) = (assign44890_e58083, (-(((locals.var_kp * locals.var_xgm_dn5) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((locals.var_kp * locals.var_xgm_dn6) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((locals.var_kp * locals.var_xgm_dn7) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((locals.var_kp * locals.var_xgm_dn8) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), );
            locals.var_eta_p_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
            let assign44900_e58095: f64 = (0.25 * locals.var_x_m);
            let assign44900_e58096: f64 = (1.0 - assign44900_e58095);
            let assign44900_e58097: f64 = (locals.var_x_m * assign44900_e58096);
            let assign44900_e58098: f64 = (0.3333333333333333 * assign44900_e58097);
            let assign44900_e58099: f64 = (1.0 - assign44900_e58098);
            let assign44900_e58100: f64 = (assign44900_e58099).sqrt();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign44900_e58100, ((-(0.3333333333333333 * ((locals.var_x_m_dn5 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn5)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((locals.var_x_m_dn6 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn6)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((locals.var_x_m_dn7 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn7)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((locals.var_x_m_dn8 * assign44900_e58096) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn8)))))) / (2.0 * assign44900_e58100)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
            let assign44910_e58109: f64 = (locals.var_x_m * locals.var_temp__blk936);
            let assign44910_e58110: f64 = (0.7071067811865475 * assign44910_e58109);
            (locals.var_sqm, locals.var_sqm_dn5, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8, ) = (assign44910_e58110, (0.7071067811865475 * ((locals.var_x_m_dn5 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_m_dn6 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_m_dn7 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_m_dn8 * locals.var_temp__blk936) + (locals.var_x_m * locals.var_temp__blk936_dn8))), );
            locals.var_sqm_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 != 0.0)) {
            let assign44920_e58122: f64 = (0.5 * locals.var_x_m);
            let assign44920_e58123: f64 = (1.0 - assign44920_e58122);
            let assign44920_e58127: f64 = (locals.var_x_m * locals.var_x_m);
            let assign44920_e58128: f64 = (0.16666666666666666 * assign44920_e58127);
            let assign44920_e58129: f64 = (assign44920_e58123 + assign44920_e58128);
            let assign44920_e58130: f64 = (locals.var_gf * assign44920_e58129);
            let assign44920_e58132: f64 = (assign44920_e58130 / locals.var_temp__blk936);
            let assign44920_e58133: f64 = (0.7071067811865475 * assign44920_e58132);
            let assign44920_e58134: f64 = (locals.var_eta_p + assign44920_e58133);
            (locals.var_alpha, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, ) = (assign44920_e58134, (locals.var_eta_p_dn5 + (0.7071067811865475 * (((((locals.var_gf_dn5 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn5)) + (0.16666666666666666 * ((locals.var_x_m_dn5 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn5)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p_dn6 + (0.7071067811865475 * (((((locals.var_gf_dn6 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn6)) + (0.16666666666666666 * ((locals.var_x_m_dn6 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn6)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p_dn7 + (0.7071067811865475 * (((((locals.var_gf_dn7 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn7)) + (0.16666666666666666 * ((locals.var_x_m_dn7 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn7)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p_dn8 + (0.7071067811865475 * (((((locals.var_gf_dn8 * assign44920_e58129) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn8)) + (0.16666666666666666 * ((locals.var_x_m_dn8 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn8)))))) * locals.var_temp__blk936) - (assign44920_e58130 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), );
            locals.var_alpha_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
            let assign44930_e58143: f64 = (locals.var_x_m - 1.0);
            let assign44930_e58145: f64 = (assign44930_e58143 + locals.var_em);
            (locals.var_pm, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, ) = (assign44930_e58145, (locals.var_x_m_dn5 + locals.var_em_dn5), (locals.var_x_m_dn6 + locals.var_em_dn6), (locals.var_x_m_dn7 + locals.var_em_dn7), (locals.var_x_m_dn8 + locals.var_em_dn8), );
            locals.var_pm_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
            let assign44940_e58155: f64 = (locals.var_dm + locals.var_pm);
            let assign44940_e58156: f64 = (assign44940_e58155).sqrt();
            let assign44940_e58157: f64 = (locals.var_gf * assign44940_e58156);
            (locals.var_xgm, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, ) = (assign44940_e58157, ((locals.var_gf_dn5 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign44940_e58156)))), ((locals.var_gf_dn6 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign44940_e58156)))), ((locals.var_gf_dn7 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign44940_e58156)))), ((locals.var_gf_dn8 * assign44940_e58156) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign44940_e58156)))), );
            locals.var_xgm_rv = 0.0;
        }

        let assign44950_e58162: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign44950_e58162;
        locals.var_guard1215_rv = 0.0;

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign44960_e58171: f64 = (1.0 - locals.var_em);
            let assign44960_e58175: f64 = (locals.var_xgm * locals.var_inv_gf2);
            let assign44960_e58176: f64 = (2.0 * assign44960_e58175);
            let assign44960_e58177: f64 = (assign44960_e58171 + assign44960_e58176);
            (locals.var_d0, locals.var_d0_dn5, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn8, ) = (assign44960_e58177, ((-locals.var_em_dn5) + (2.0 * ((locals.var_xgm_dn5 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn5)))), ((-locals.var_em_dn6) + (2.0 * ((locals.var_xgm_dn6 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((locals.var_xgm_dn7 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((locals.var_xgm_dn8 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn8)))), );
            locals.var_d0_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign44970_e58190: f64 = (locals.var_kp * locals.var_xgm);
            let assign44970_e58191: f64 = (1.0 + assign44970_e58190);
            let assign44970_e58192: f64 = (assign44970_e58191).sqrt();
            let assign44970_e58193: f64 = (1.0 / assign44970_e58192);
            (locals.var_eta_p, locals.var_eta_p_dn5, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8, ) = (assign44970_e58193, (-(((locals.var_kp * locals.var_xgm_dn5) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((locals.var_kp * locals.var_xgm_dn6) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((locals.var_kp * locals.var_xgm_dn7) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((locals.var_kp * locals.var_xgm_dn8) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), );
            locals.var_eta_p_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign44980_e58205: f64 = (locals.var_eta_p + 1.0);
            let assign44980_e58206: f64 = (locals.var_eta_p / assign44980_e58205);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign44980_e58206, (((locals.var_eta_p_dn5 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn5)) / (assign44980_e58205 * assign44980_e58205)), (((locals.var_eta_p_dn6 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn6)) / (assign44980_e58205 * assign44980_e58205)), (((locals.var_eta_p_dn7 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn7)) / (assign44980_e58205 * assign44980_e58205)), (((locals.var_eta_p_dn8 * assign44980_e58205) - (locals.var_eta_p * locals.var_eta_p_dn8)) / (assign44980_e58205 * assign44980_e58205)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign44990_e58218: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
            let assign44990_e58220: f64 = (assign44990_e58218 * locals.var_gf2);
            let assign44990_e58222: f64 = (assign44990_e58220 * locals.var_dm);
            let assign44990_e58223: f64 = (locals.var_kp * assign44990_e58222);
            (locals.var_x_pm, locals.var_x_pm_dn5, locals.var_x_pm_dn6, locals.var_x_pm_dn7, locals.var_x_pm_dn8, ) = (assign44990_e58223, (locals.var_kp * ((((((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn5)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn5))), (locals.var_kp * ((((((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn6)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn6))), (locals.var_kp * ((((((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn7)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn7))), (locals.var_kp * ((((((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) * locals.var_gf2) + (assign44990_e58218 * locals.var_gf2_dn8)) * locals.var_dm) + (assign44990_e58220 * locals.var_dm_dn8))), );
            locals.var_x_pm_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45000_e58235: f64 = (locals.var_xgm - locals.var_x_pm);
            let assign45000_e58236: f64 = (2.0 * assign45000_e58235);
            let assign45000_e58240: f64 = (1.0 - locals.var_em);
            let assign45000_e58242: f64 = (assign45000_e58240 + locals.var_dm);
            let assign45000_e58243: f64 = (locals.var_gf2 * assign45000_e58242);
            let assign45000_e58244: f64 = (assign45000_e58236 + assign45000_e58243);
            (locals.var_p_pd, locals.var_p_pd_dn5, locals.var_p_pd_dn6, locals.var_p_pd_dn7, locals.var_p_pd_dn8, ) = (assign45000_e58244, ((2.0 * (locals.var_xgm_dn5 - locals.var_x_pm_dn5)) + ((locals.var_gf2_dn5 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn5) + locals.var_dm_dn5)))), ((2.0 * (locals.var_xgm_dn6 - locals.var_x_pm_dn6)) + ((locals.var_gf2_dn6 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn6) + locals.var_dm_dn6)))), ((2.0 * (locals.var_xgm_dn7 - locals.var_x_pm_dn7)) + ((locals.var_gf2_dn7 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn7) + locals.var_dm_dn7)))), ((2.0 * (locals.var_xgm_dn8 - locals.var_x_pm_dn8)) + ((locals.var_gf2_dn8 * assign45000_e58242) + (locals.var_gf2 * ((-locals.var_em_dn8) + locals.var_dm_dn8)))), );
            locals.var_p_pd_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45010_e58257: f64 = (2.0 * locals.var_xgm);
            let assign45010_e58258: f64 = (locals.var_x_pm - assign45010_e58257);
            let assign45010_e58259: f64 = (locals.var_x_pm * assign45010_e58258);
            (locals.var_q_pd, locals.var_q_pd_dn5, locals.var_q_pd_dn6, locals.var_q_pd_dn7, locals.var_q_pd_dn8, ) = (assign45010_e58259, ((locals.var_x_pm_dn5 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn5 - (2.0 * locals.var_xgm_dn5)))), ((locals.var_x_pm_dn6 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn6 - (2.0 * locals.var_xgm_dn6)))), ((locals.var_x_pm_dn7 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn7 - (2.0 * locals.var_xgm_dn7)))), ((locals.var_x_pm_dn8 * assign45010_e58258) + (locals.var_x_pm * (locals.var_x_pm_dn8 - (2.0 * locals.var_xgm_dn8)))), );
            locals.var_q_pd_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45020_e58273: f64 = (locals.var_em + locals.var_dm);
            let assign45020_e58274: f64 = (locals.var_gf2 * assign45020_e58273);
            let assign45020_e58275: f64 = (0.5 * assign45020_e58274);
            let assign45020_e58276: f64 = (1.0 - assign45020_e58275);
            (locals.var_xi_pd, locals.var_xi_pd_dn5, locals.var_xi_pd_dn6, locals.var_xi_pd_dn7, locals.var_xi_pd_dn8, ) = (assign45020_e58276, (-(0.5 * ((locals.var_gf2_dn5 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn5 + locals.var_dm_dn5))))), (-(0.5 * ((locals.var_gf2_dn6 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn6 + locals.var_dm_dn6))))), (-(0.5 * ((locals.var_gf2_dn7 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn7 + locals.var_dm_dn7))))), (-(0.5 * ((locals.var_gf2_dn8 * assign45020_e58273) + (locals.var_gf2 * (locals.var_em_dn8 + locals.var_dm_dn8))))), );
            locals.var_xi_pd_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45030_e58287: f64 = (locals.var_q_pd * locals.var_p_pd);
            let assign45030_e58290: f64 = (locals.var_p_pd * locals.var_p_pd);
            let assign45030_e58293: f64 = (locals.var_xi_pd * locals.var_q_pd);
            let assign45030_e58294: f64 = (assign45030_e58290 - assign45030_e58293);
            let assign45030_e58295: f64 = (assign45030_e58287 / assign45030_e58294);
            (locals.var_u_pd, locals.var_u_pd_dn5, locals.var_u_pd_dn6, locals.var_u_pd_dn7, locals.var_u_pd_dn8, ) = (assign45030_e58295, (((((locals.var_q_pd_dn5 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn5)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn5 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn5)) - ((locals.var_xi_pd_dn5 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn5))))) / (assign45030_e58294 * assign45030_e58294)), (((((locals.var_q_pd_dn6 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn6)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn6 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn6)) - ((locals.var_xi_pd_dn6 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn6))))) / (assign45030_e58294 * assign45030_e58294)), (((((locals.var_q_pd_dn7 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn7)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn7 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn7)) - ((locals.var_xi_pd_dn7 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn7))))) / (assign45030_e58294 * assign45030_e58294)), (((((locals.var_q_pd_dn8 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn8)) * assign45030_e58294) - (assign45030_e58287 * (((locals.var_p_pd_dn8 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn8)) - ((locals.var_xi_pd_dn8 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn8))))) / (assign45030_e58294 * assign45030_e58294)), );
            locals.var_u_pd_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45040_e58306: f64 = (locals.var_x_m + locals.var_u_pd);
            (locals.var_x_m, locals.var_x_m_dn5, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, ) = (assign45040_e58306, (locals.var_x_m_dn5 + locals.var_u_pd_dn5), (locals.var_x_m_dn6 + locals.var_u_pd_dn6), (locals.var_x_m_dn7 + locals.var_u_pd_dn7), (locals.var_x_m_dn8 + locals.var_u_pd_dn8), );
            locals.var_x_m_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45050_e58316: f64 = (locals.var_u_pd).exp();
            (locals.var_km, locals.var_km_dn5, locals.var_km_dn6, locals.var_km_dn7, locals.var_km_dn8, ) = (assign45050_e58316, (assign45050_e58316 * locals.var_u_pd_dn5), (assign45050_e58316 * locals.var_u_pd_dn6), (assign45050_e58316 * locals.var_u_pd_dn7), (assign45050_e58316 * locals.var_u_pd_dn8), );
            locals.var_km_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45060_e58327: f64 = (locals.var_em / locals.var_km);
            (locals.var_em, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, ) = (assign45060_e58327, (((locals.var_em_dn5 * locals.var_km) - (locals.var_em * locals.var_km_dn5)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn6 * locals.var_km) - (locals.var_em * locals.var_km_dn6)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn7 * locals.var_km) - (locals.var_em * locals.var_km_dn7)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn8 * locals.var_km) - (locals.var_em * locals.var_km_dn8)) / (locals.var_km * locals.var_km)), );
            locals.var_em_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45070_e58338: f64 = (locals.var_dm * locals.var_km);
            (locals.var_dm, locals.var_dm_dn5, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, ) = (assign45070_e58338, ((locals.var_dm_dn5 * locals.var_km) + (locals.var_dm * locals.var_km_dn5)), ((locals.var_dm_dn6 * locals.var_km) + (locals.var_dm * locals.var_km_dn6)), ((locals.var_dm_dn7 * locals.var_km) + (locals.var_dm * locals.var_km_dn7)), ((locals.var_dm_dn8 * locals.var_km) + (locals.var_dm * locals.var_km_dn8)), );
            locals.var_dm_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45080_e58349: f64 = (locals.var_x_m - 1.0);
            let assign45080_e58351: f64 = (assign45080_e58349 + locals.var_em);
            (locals.var_pm, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, ) = (assign45080_e58351, (locals.var_x_m_dn5 + locals.var_em_dn5), (locals.var_x_m_dn6 + locals.var_em_dn6), (locals.var_x_m_dn7 + locals.var_em_dn7), (locals.var_x_m_dn8 + locals.var_em_dn8), );
            locals.var_pm_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45090_e58363: f64 = (locals.var_dm + locals.var_pm);
            let assign45090_e58364: f64 = (assign45090_e58363).sqrt();
            let assign45090_e58365: f64 = (locals.var_gf * assign45090_e58364);
            (locals.var_xgm, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, ) = (assign45090_e58365, ((locals.var_gf_dn5 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign45090_e58364)))), ((locals.var_gf_dn6 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign45090_e58364)))), ((locals.var_gf_dn7 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign45090_e58364)))), ((locals.var_gf_dn8 * assign45090_e58364) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign45090_e58364)))), );
            locals.var_xgm_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45100_e58376: f64 = (1.0 - locals.var_em);
            let assign45100_e58380: f64 = (locals.var_xgm * locals.var_eta_p);
            let assign45100_e58382: f64 = (assign45100_e58380 * locals.var_inv_gf2);
            let assign45100_e58383: f64 = (2.0 * assign45100_e58382);
            let assign45100_e58384: f64 = (assign45100_e58376 + assign45100_e58383);
            (locals.var_km0, locals.var_km0_dn5, locals.var_km0_dn6, locals.var_km0_dn7, locals.var_km0_dn8, ) = (assign45100_e58384, ((-locals.var_em_dn5) + (2.0 * ((((locals.var_xgm_dn5 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn5)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn5)))), ((-locals.var_em_dn6) + (2.0 * ((((locals.var_xgm_dn6 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn6)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((((locals.var_xgm_dn7 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn7)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((((locals.var_xgm_dn8 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn8)) * locals.var_inv_gf2) + (assign45100_e58380 * locals.var_inv_gf2_dn8)))), );
            locals.var_km0_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45110_e58395: f64 = (locals.var_x_ds * locals.var_km);
            let assign45110_e58398: f64 = (locals.var_d0 + locals.var_d_bar);
            let assign45110_e58399: f64 = (assign45110_e58395 * assign45110_e58398);
            let assign45110_e58403: f64 = (locals.var_km * locals.var_d_bar);
            let assign45110_e58404: f64 = (locals.var_km0 + assign45110_e58403);
            let assign45110_e58405: f64 = (assign45110_e58399 / assign45110_e58404);
            (locals.var_x_ds, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, ) = (assign45110_e58405, (((((((locals.var_x_ds_dn5 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn5)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn5 + locals.var_d_bar_dn5))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn5 + ((locals.var_km_dn5 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn5))))) / (assign45110_e58404 * assign45110_e58404)), (((((((locals.var_x_ds_dn6 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn6)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn6 + locals.var_d_bar_dn6))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn6 + ((locals.var_km_dn6 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn6))))) / (assign45110_e58404 * assign45110_e58404)), (((((((locals.var_x_ds_dn7 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn7)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn7 + locals.var_d_bar_dn7))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn7 + ((locals.var_km_dn7 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn7))))) / (assign45110_e58404 * assign45110_e58404)), (((((((locals.var_x_ds_dn8 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn8)) * assign45110_e58398) + (assign45110_e58395 * (locals.var_d0_dn8 + locals.var_d_bar_dn8))) * assign45110_e58404) - (assign45110_e58399 * (locals.var_km0_dn8 + ((locals.var_km_dn8 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn8))))) / (assign45110_e58404 * assign45110_e58404)), );
            locals.var_x_ds_rv = 0.0;
        }

        if (((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
            let assign45120_e58416: f64 = (locals.var_x_ds * locals.var_phit1);
            (locals.var_dps, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, ) = (assign45120_e58416, ((locals.var_x_ds_dn5 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn5)), ((locals.var_x_ds_dn6 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn6)), ((locals.var_x_ds_dn7 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn7)), ((locals.var_x_ds_dn8 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn8)), );
            locals.var_dps_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
            let assign45130_e58424: f64 = (locals.var_pm).sqrt();
            (locals.var_sqm, locals.var_sqm_dn5, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8, ) = (assign45130_e58424, (locals.var_pm_dn5 / (2.0 * assign45130_e58424)), (locals.var_pm_dn6 / (2.0 * assign45130_e58424)), (locals.var_pm_dn7 / (2.0 * assign45130_e58424)), (locals.var_pm_dn8 / (2.0 * assign45130_e58424)), );
            locals.var_sqm_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1213 == 0.0)) {
            let assign45140_e58436: f64 = (1.0 - locals.var_em);
            let assign45140_e58437: f64 = (locals.var_gf * assign45140_e58436);
            let assign45140_e58439: f64 = (assign45140_e58437 / locals.var_sqm);
            let assign45140_e58440: f64 = (0.5 * assign45140_e58439);
            let assign45140_e58441: f64 = (locals.var_eta_p + assign45140_e58440);
            (locals.var_alpha, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, ) = (assign45140_e58441, (locals.var_eta_p_dn5 + (0.5 * (((((locals.var_gf_dn5 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn5))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn5)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn6 + (0.5 * (((((locals.var_gf_dn6 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn6))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn6)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn7 + (0.5 * (((((locals.var_gf_dn7 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn7))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn7)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn8 + (0.5 * (((((locals.var_gf_dn8 * assign45140_e58436) + (locals.var_gf * (-locals.var_em_dn8))) * locals.var_sqm) - (assign45140_e58437 * locals.var_sqm_dn8)) / (locals.var_sqm * locals.var_sqm)))), );
            locals.var_alpha_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1197 != 0.0) {
            let assign45150_e58448: f64 = (locals.var_gf2 * locals.var_dm);
            let assign45150_e58452: f64 = (locals.var_gf * locals.var_sqm);
            let assign45150_e58453: f64 = (locals.var_xgm + assign45150_e58452);
            let assign45150_e58454: f64 = (assign45150_e58448 / assign45150_e58453);
            let assign45150_e58455: f64 = (locals.var_phit1 * assign45150_e58454);
            (locals.var_qim, locals.var_qim_dn5, locals.var_qim_dn6, locals.var_qim_dn7, locals.var_qim_dn8, ) = (assign45150_e58455, ((locals.var_phit1_dn5 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn5 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn5)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn5 + ((locals.var_gf_dn5 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn5))))) / (assign45150_e58453 * assign45150_e58453)))), ((locals.var_phit1_dn6 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn6 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn6)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn6 + ((locals.var_gf_dn6 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn6))))) / (assign45150_e58453 * assign45150_e58453)))), ((locals.var_phit1_dn7 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn7 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn7)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn7 + ((locals.var_gf_dn7 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn7))))) / (assign45150_e58453 * assign45150_e58453)))), ((locals.var_phit1_dn8 * assign45150_e58454) + (locals.var_phit1 * (((((locals.var_gf2_dn8 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn8)) * assign45150_e58453) - (assign45150_e58448 * (locals.var_xgm_dn8 + ((locals.var_gf_dn8 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn8))))) / (assign45150_e58453 * assign45150_e58453)))), );
            locals.var_qim_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45160_e58462: f64 = (locals.var_phit1 * locals.var_alpha);
            let assign45160_e58463: f64 = (locals.var_qim + assign45160_e58462);
            (locals.var_qim1, locals.var_qim1_dn5, locals.var_qim1_dn6, locals.var_qim1_dn7, locals.var_qim1_dn8, ) = (assign45160_e58463, (locals.var_qim_dn5 + ((locals.var_phit1_dn5 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn5))), (locals.var_qim_dn6 + ((locals.var_phit1_dn6 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn6))), (locals.var_qim_dn7 + ((locals.var_phit1_dn7 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn7))), (locals.var_qim_dn8 + ((locals.var_phit1_dn8 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn8))), );
            locals.var_qim1_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45170_e58469: f64 = (locals.var_sqm * locals.var_gf);
            let assign45170_e58471: f64 = (assign45170_e58469 * locals.var_phit1);
            (locals.var_qbm, locals.var_qbm_dn5, locals.var_qbm_dn6, locals.var_qbm_dn7, locals.var_qbm_dn8, ) = (assign45170_e58471, ((((locals.var_sqm_dn5 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn5)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn5)), ((((locals.var_sqm_dn6 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn6)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn6)), ((((locals.var_sqm_dn7 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn7)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn7)), ((((locals.var_sqm_dn8 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn8)) * locals.var_phit1) + (assign45170_e58469 * locals.var_phit1_dn8)), );
            locals.var_qbm_rv = 0.0;
        }

        let assign45180_e58476: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign45180_e58476;
        locals.var_guard1216_rv = 0.0;

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1216 != 0.0)) {
            let assign45190_e58483: f64 = (locals.var_rsg_i * locals.var_qim);
            let assign45190_e58484: f64 = (1.0 - assign45190_e58483);
            (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, ) = (assign45190_e58484, (-(locals.var_rsg_i * locals.var_qim_dn5)), (-(locals.var_rsg_i * locals.var_qim_dn6)), (-(locals.var_rsg_i * locals.var_qim_dn7)), (-(locals.var_rsg_i * locals.var_qim_dn8)), );
            locals.var_rhog_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1216 == 0.0)) {
            let assign45200_e58495: f64 = (locals.var_rsg_i * locals.var_qim);
            let assign45200_e58496: f64 = (1.0 + assign45200_e58495);
            let assign45200_e58497: f64 = (1.0 / assign45200_e58496);
            (locals.var_rhog, locals.var_rhog_dn5, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, ) = (assign45200_e58497, (-((locals.var_rsg_i * locals.var_qim_dn5) / (assign45200_e58496 * assign45200_e58496))), (-((locals.var_rsg_i * locals.var_qim_dn6) / (assign45200_e58496 * assign45200_e58496))), (-((locals.var_rsg_i * locals.var_qim_dn7) / (assign45200_e58496 * assign45200_e58496))), (-((locals.var_rsg_i * locals.var_qim_dn8) / (assign45200_e58496 * assign45200_e58496))), );
            locals.var_rhog_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45210_e58503: f64 = (locals.var_ther_i * locals.var_rhob);
            let assign45210_e58505: f64 = (assign45210_e58503 * locals.var_rhog);
            let assign45210_e58507: f64 = (assign45210_e58505 * locals.var_qim);
            (locals.var_gr, locals.var_gr_dn5, locals.var_gr_dn6, locals.var_gr_dn7, locals.var_gr_dn8, ) = (assign45210_e58507, (((((locals.var_ther_i * locals.var_rhob_dn5) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn5)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn5)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn6)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn7)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign45210_e58503 * locals.var_rhog_dn8)) * locals.var_qim) + (assign45210_e58505 * locals.var_qim_dn8)), );
            locals.var_gr_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45220_e58514: f64 = (locals.var_eta_mu * locals.var_qim);
            let assign45220_e58515: f64 = (locals.var_qbm + assign45220_e58514);
            (locals.var_qeff, locals.var_qeff_dn5, locals.var_qeff_dn6, locals.var_qeff_dn7, locals.var_qeff_dn8, ) = (assign45220_e58515, (locals.var_qbm_dn5 + (locals.var_eta_mu * locals.var_qim_dn5)), (locals.var_qbm_dn6 + (locals.var_eta_mu * locals.var_qim_dn6)), (locals.var_qbm_dn7 + (locals.var_eta_mu * locals.var_qim_dn7)), (locals.var_qbm_dn8 + (locals.var_eta_mu * locals.var_qim_dn8)), );
            locals.var_qeff_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45230_e58522: f64 = (locals.var_eta_mu1 * locals.var_qim);
            let assign45230_e58523: f64 = (locals.var_qbm + assign45230_e58522);
            (locals.var_qeff1, locals.var_qeff1_dn5, locals.var_qeff1_dn6, locals.var_qeff1_dn7, locals.var_qeff1_dn8, ) = (assign45230_e58523, (locals.var_qbm_dn5 + (locals.var_eta_mu1 * locals.var_qim_dn5)), (locals.var_qbm_dn6 + (locals.var_eta_mu1 * locals.var_qim_dn6)), (locals.var_qbm_dn7 + (locals.var_eta_mu1 * locals.var_qim_dn7)), (locals.var_qbm_dn8 + (locals.var_eta_mu1 * locals.var_qim_dn8)), );
            locals.var_qeff1_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45240_e58529: f64 = (locals.var_e_eff0 * locals.var_qeff);
            (locals.var_eeffm, locals.var_eeffm_dn5, locals.var_eeffm_dn6, locals.var_eeffm_dn7, locals.var_eeffm_dn8, ) = (assign45240_e58529, (locals.var_e_eff0 * locals.var_qeff_dn5), (locals.var_e_eff0 * locals.var_qeff_dn6), (locals.var_e_eff0 * locals.var_qeff_dn7), (locals.var_e_eff0 * locals.var_qeff_dn8), );
            locals.var_eeffm_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45250_e58536: f64 = (locals.var_pm + locals.var_dm);
            let assign45250_e58538: f64 = (assign45250_e58536 + 1e-14);
            let assign45250_e58539: f64 = (locals.var_pm / assign45250_e58538);
            let assign45250_e58540: f64 = (assign45250_e58539).ln();
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign45250_e58540, ((((locals.var_pm_dn5 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn5 + locals.var_dm_dn5))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((locals.var_pm_dn6 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn6 + locals.var_dm_dn6))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((locals.var_pm_dn7 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn7 + locals.var_dm_dn7))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((locals.var_pm_dn8 * assign45250_e58538) - (locals.var_pm * (locals.var_pm_dn8 + locals.var_dm_dn8))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45260_e58546: f64 = (locals.var_eeffm * locals.var_mue_t);
            let assign45260_e58548: f64 = (assign45260_e58546).powf(locals.var_themu_t);
            let assign45260_e58552: f64 = (0.5 * locals.var_thecs_t);
            let assign45260_e58554: f64 = (assign45260_e58552 * locals.var_temp1);
            let assign45260_e58555: f64 = (assign45260_e58554).exp();
            let assign45260_e58556: f64 = (locals.var_cs_t * assign45260_e58555);
            let assign45260_e58557: f64 = (assign45260_e58548 + assign45260_e58556);
            (locals.var_mutmp, locals.var_mutmp_dn5, locals.var_mutmp_dn6, locals.var_mutmp_dn7, locals.var_mutmp_dn8, ) = (assign45260_e58557, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn5 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn5 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn6 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn6 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn7 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn7 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45260_e58546).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn8 * locals.var_mue_t))) } } else { (assign45260_e58548 * (locals.var_themu_t * ((locals.var_eeffm_dn8 * locals.var_mue_t) / assign45260_e58546))) } + (locals.var_cs_t * (assign45260_e58555 * (assign45260_e58552 * locals.var_temp1_dn8)))), );
            locals.var_mutmp_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45270_e58563: f64 = (1.0 + locals.var_mutmp);
            let assign45270_e58565: f64 = (assign45270_e58563 + locals.var_gr);
            let assign45270_e58567: f64 = (assign45270_e58565 * locals.var_rxcor);
            (locals.var_gmob, locals.var_gmob_dn5, locals.var_gmob_dn6, locals.var_gmob_dn7, locals.var_gmob_dn8, ) = (assign45270_e58567, (((locals.var_mutmp_dn5 + locals.var_gr_dn5) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn5)), (((locals.var_mutmp_dn6 + locals.var_gr_dn6) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn6)), (((locals.var_mutmp_dn7 + locals.var_gr_dn7) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn7)), (((locals.var_mutmp_dn8 + locals.var_gr_dn8) * locals.var_rxcor) + (assign45270_e58565 * locals.var_rxcor_dn8)), );
            locals.var_gmob_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45280_e58574: f64 = (locals.var_v_ds - locals.var_dps);
            let assign45280_e58576: f64 = (assign45280_e58574 * locals.var_inv_vp);
            let assign45280_e58577: f64 = (1.0 + assign45280_e58576);
            let assign45280_e58581: f64 = (locals.var_vdse - locals.var_dps);
            let assign45280_e58583: f64 = (assign45280_e58581 * locals.var_inv_vp);
            let assign45280_e58584: f64 = (1.0 + assign45280_e58583);
            let assign45280_e58585: f64 = (assign45280_e58577 / assign45280_e58584);
            let assign45280_e58586: f64 = (assign45280_e58585).ln();
            (locals.var_s1, locals.var_s1_dn5, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, ) = (assign45280_e58586, ((((((-locals.var_dps_dn5) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn5 - locals.var_dps_dn5) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((locals.var_v_ds_dn6 - locals.var_dps_dn6) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn6 - locals.var_dps_dn6) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((locals.var_v_ds_dn7 - locals.var_dps_dn7) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn7 - locals.var_dps_dn7) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((-locals.var_dps_dn8) * locals.var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((locals.var_vdse_dn8 - locals.var_dps_dn8) * locals.var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), );
            locals.var_s1_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45290_e58592: f64 = (locals.var_qim * locals.var_xitsb);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign45290_e58592, ((locals.var_qim_dn5 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn5)), ((locals.var_qim_dn6 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn6)), ((locals.var_qim_dn7 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn7)), ((locals.var_qim_dn8 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn8)), );
            locals.var_temp2_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45300_e58599: f64 = (locals.var_thesatt_i + locals.var_temp2);
            let assign45300_e58600: f64 = (locals.var_temp2 / assign45300_e58599);
            (locals.var_wsat, locals.var_wsat_dn5, locals.var_wsat_dn6, locals.var_wsat_dn7, locals.var_wsat_dn8, ) = (assign45300_e58600, (((locals.var_temp2_dn5 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign45300_e58599 * assign45300_e58599)), (((locals.var_temp2_dn6 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign45300_e58599 * assign45300_e58599)), (((locals.var_temp2_dn7 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign45300_e58599 * assign45300_e58599)), (((locals.var_temp2_dn8 * assign45300_e58599) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign45300_e58599 * assign45300_e58599)), );
            locals.var_wsat_rv = 0.0;
        }

        let assign45310_e58605: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1217 = assign45310_e58605;
        locals.var_guard1217_rv = 0.0;

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1217 != 0.0)) {
            let assign45320_e58613: f64 = (locals.var_thesatg_i * locals.var_wsat);
            let assign45320_e58614: f64 = (1.0 - assign45320_e58613);
            let assign45320_e58615: f64 = (1.0 / assign45320_e58614);
            (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, ) = (assign45320_e58615, (-((-(locals.var_thesatg_i * locals.var_wsat_dn5)) / (assign45320_e58614 * assign45320_e58614))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn6)) / (assign45320_e58614 * assign45320_e58614))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn7)) / (assign45320_e58614 * assign45320_e58614))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn8)) / (assign45320_e58614 * assign45320_e58614))), );
            locals.var_factheta_rv = 0.0;
        }

        if ((locals.var_guard1197 != 0.0) && (locals.var_guard1217 == 0.0)) {
            let assign45330_e58625: f64 = (locals.var_thesatg_i * locals.var_wsat);
            let assign45330_e58626: f64 = (1.0 + assign45330_e58625);
            (locals.var_factheta, locals.var_factheta_dn5, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, ) = (assign45330_e58626, (locals.var_thesatg_i * locals.var_wsat_dn5), (locals.var_thesatg_i * locals.var_wsat_dn6), (locals.var_thesatg_i * locals.var_wsat_dn7), (locals.var_thesatg_i * locals.var_wsat_dn8), );
            locals.var_factheta_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45340_e58632: f64 = (locals.var_thesatloc * locals.var_factheta);
            (locals.var_thesateff, locals.var_thesateff_dn5, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8, ) = (assign45340_e58632, (locals.var_thesatloc * locals.var_factheta_dn5), (locals.var_thesatloc * locals.var_factheta_dn6), (locals.var_thesatloc * locals.var_factheta_dn7), (locals.var_thesatloc * locals.var_factheta_dn8), );
            locals.var_thesateff_rv = 0.0;
        }

        if (locals.var_guard1197 != 0.0) {
            let assign45350_e58638: f64 = (locals.var_xgm * locals.var_phit1);
            (locals.var_voxm, locals.var_voxm_dn5, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8, ) = (assign45350_e58638, ((locals.var_xgm_dn5 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn5)), ((locals.var_xgm_dn6 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn6)), ((locals.var_xgm_dn7 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn7)), ((locals.var_xgm_dn8 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn8)), );
            locals.var_voxm_rv = 0.0;
        }

        (locals.var_vdsat_lim_dc, locals.var_vdsat_lim_dc_dn5, locals.var_vdsat_lim_dc_dn6, locals.var_vdsat_lim_dc_dn7, locals.var_vdsat_lim_dc_dn8, ) = (locals.var_vdsat_lim, locals.var_vdsat_lim_dn5, locals.var_vdsat_lim_dn6, locals.var_vdsat_lim_dn7, locals.var_vdsat_lim_dn8, );
        locals.var_vdsat_lim_dc_rv = 0.0;

        (locals.var_vdse_dc, locals.var_vdse_dc_dn5, locals.var_vdse_dc_dn6, locals.var_vdse_dc_dn7, locals.var_vdse_dc_dn8, ) = (locals.var_vdse, locals.var_vdse_dn5, locals.var_vdse_dn6, locals.var_vdse_dn7, locals.var_vdse_dn8, );
        locals.var_vdse_dc_rv = 0.0;

        (locals.var_udse_dc, locals.var_udse_dc_dn5, locals.var_udse_dc_dn6, locals.var_udse_dc_dn7, locals.var_udse_dc_dn8, ) = (locals.var_udse, locals.var_udse_dn5, locals.var_udse_dn6, locals.var_udse_dn7, locals.var_udse_dn8, );
        locals.var_udse_dc_rv = 0.0;

        (locals.var_x_ds_dc, locals.var_x_ds_dc_dn5, locals.var_x_ds_dc_dn6, locals.var_x_ds_dc_dn7, locals.var_x_ds_dc_dn8, ) = (locals.var_x_ds, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, );
        locals.var_x_ds_dc_rv = 0.0;

        (locals.var_dps_dc, locals.var_dps_dc_dn5, locals.var_dps_dc_dn6, locals.var_dps_dc_dn7, locals.var_dps_dc_dn8, ) = (locals.var_dps, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, );
        locals.var_dps_dc_rv = 0.0;

        (locals.var_x_m_dc, locals.var_x_m_dc_dn5, locals.var_x_m_dc_dn6, locals.var_x_m_dc_dn7, locals.var_x_m_dc_dn8, ) = (locals.var_x_m, locals.var_x_m_dn5, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, );
        locals.var_x_m_dc_rv = 0.0;

        (locals.var_qbd_dc, locals.var_qbd_dc_dn5, locals.var_qbd_dc_dn6, locals.var_qbd_dc_dn7, locals.var_qbd_dc_dn8, ) = (locals.var_qbd, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, );
        locals.var_qbd_dc_rv = 0.0;

        (locals.var_eta_p_dc, locals.var_eta_p_dc_dn5, locals.var_eta_p_dc_dn6, locals.var_eta_p_dc_dn7, locals.var_eta_p_dc_dn8, ) = (locals.var_eta_p, locals.var_eta_p_dn5, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8, );
        locals.var_eta_p_dc_rv = 0.0;

        (locals.var_alpha_dc, locals.var_alpha_dc_dn5, locals.var_alpha_dc_dn6, locals.var_alpha_dc_dn7, locals.var_alpha_dc_dn8, ) = (locals.var_alpha, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, );
        locals.var_alpha_dc_rv = 0.0;

        (locals.var_qim_dc, locals.var_qim_dc_dn5, locals.var_qim_dc_dn6, locals.var_qim_dc_dn7, locals.var_qim_dc_dn8, ) = (locals.var_qim, locals.var_qim_dn5, locals.var_qim_dn6, locals.var_qim_dn7, locals.var_qim_dn8, );
        locals.var_qim_dc_rv = 0.0;

        (locals.var_qim1_dc, locals.var_qim1_dc_dn5, locals.var_qim1_dc_dn6, locals.var_qim1_dc_dn7, locals.var_qim1_dc_dn8, ) = (locals.var_qim1, locals.var_qim1_dn5, locals.var_qim1_dn6, locals.var_qim1_dn7, locals.var_qim1_dn8, );
        locals.var_qim1_dc_rv = 0.0;

        (locals.var_qbm_dc, locals.var_qbm_dc_dn5, locals.var_qbm_dc_dn6, locals.var_qbm_dc_dn7, locals.var_qbm_dc_dn8, ) = (locals.var_qbm, locals.var_qbm_dn5, locals.var_qbm_dn6, locals.var_qbm_dn7, locals.var_qbm_dn8, );
        locals.var_qbm_dc_rv = 0.0;

        (locals.var_qeff1_dc, locals.var_qeff1_dc_dn5, locals.var_qeff1_dc_dn6, locals.var_qeff1_dc_dn7, locals.var_qeff1_dc_dn8, ) = (locals.var_qeff1, locals.var_qeff1_dn5, locals.var_qeff1_dn6, locals.var_qeff1_dn7, locals.var_qeff1_dn8, );
        locals.var_qeff1_dc_rv = 0.0;

        (locals.var_gmob_dc, locals.var_gmob_dc_dn5, locals.var_gmob_dc_dn6, locals.var_gmob_dc_dn7, locals.var_gmob_dc_dn8, ) = (locals.var_gmob, locals.var_gmob_dn5, locals.var_gmob_dn6, locals.var_gmob_dn7, locals.var_gmob_dn8, );
        locals.var_gmob_dc_rv = 0.0;

        (locals.var_s1_dc, locals.var_s1_dc_dn5, locals.var_s1_dc_dn6, locals.var_s1_dc_dn7, locals.var_s1_dc_dn8, ) = (locals.var_s1, locals.var_s1_dn5, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, );
        locals.var_s1_dc_rv = 0.0;

        (locals.var_thesateff_dc, locals.var_thesateff_dc_dn5, locals.var_thesateff_dc_dn6, locals.var_thesateff_dc_dn7, locals.var_thesateff_dc_dn8, ) = (locals.var_thesateff, locals.var_thesateff_dn5, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8, );
        locals.var_thesateff_dc_rv = 0.0;

        (locals.var_voxm_dc, locals.var_voxm_dc_dn5, locals.var_voxm_dc_dn6, locals.var_voxm_dc_dn7, locals.var_voxm_dc_dn8, ) = (locals.var_voxm, locals.var_voxm_dn5, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8, );
        locals.var_voxm_dc_rv = 0.0;

        (locals.var_gdl_dc, locals.var_gdl_dc_dn5, locals.var_gdl_dc_dn6, locals.var_gdl_dc_dn7, locals.var_gdl_dc_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_gdl_dc_rv = 0.0;

        (locals.var_gmob_dl_dc, locals.var_gmob_dl_dc_dn5, locals.var_gmob_dl_dc_dn6, locals.var_gmob_dl_dc_dn7, locals.var_gmob_dl_dc_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_gmob_dl_dc_rv = 0.0;

        (locals.var_gvsatinv_dc, locals.var_gvsatinv_dc_dn5, locals.var_gvsatinv_dc_dn6, locals.var_gvsatinv_dc_dn7, locals.var_gvsatinv_dc_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_gvsatinv_dc_rv = 0.0;

        (locals.var_h_dc, locals.var_h_dc_dn5, locals.var_h_dc_dn6, locals.var_h_dc_dn7, locals.var_h_dc_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_h_dc_rv = 0.0;

        (locals.var_i_ds, locals.var_i_ds_dn5, locals.var_i_ds_dn6, locals.var_i_ds_dn7, locals.var_i_ds_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_i_ds_rv = 0.0;

        let assign45690_e58714: f64 = if locals.var_xg_dc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign45690_e58714;
        locals.var_guard1218_rv = 0.0;

        if (locals.var_guard1218 != 0.0) {
            let assign45700_e58719: f64 = (locals.var_vdsx * locals.var_inv_vp);
            let assign45700_e58720: f64 = (1.0 + assign45700_e58719);
            let assign45700_e58721: f64 = (assign45700_e58720).ln();
            (locals.var_s2, locals.var_s2_dn6, locals.var_s2_dn7, ) = (assign45700_e58721, ((locals.var_vdsx_dn6 * locals.var_inv_vp) / assign45700_e58720), ((locals.var_vdsx_dn7 * locals.var_inv_vp) / assign45700_e58720), );
            locals.var_s2_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45710_e58727: f64 = (locals.var_phit1_dc * locals.var_alpha_dc);
            let assign45710_e58729: f64 = (assign45710_e58727 / locals.var_qim1_dc);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign45710_e58729, (((((locals.var_phit1_dc_dn5 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn5)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn5)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn6 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn6)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn7 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn7)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn8 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn8)) * locals.var_qim1_dc) - (assign45710_e58727 * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45720_e58736: f64 = (locals.var_alp1_i / locals.var_qim1_dc);
            let assign45720_e58737: f64 = (locals.var_alp_i + assign45720_e58736);
            let assign45720_e58739: f64 = (assign45720_e58737 * locals.var_qim_dc);
            let assign45720_e58741: f64 = (assign45720_e58739 / locals.var_qim1_dc);
            let assign45720_e58743: f64 = (assign45720_e58741 * locals.var_s1_dc);
            let assign45720_e58746: f64 = (locals.var_alp2_i * locals.var_qbm_dc);
            let assign45720_e58748: f64 = (assign45720_e58746 * locals.var_temp__blk936);
            let assign45720_e58750: f64 = (assign45720_e58748 * locals.var_temp__blk936);
            let assign45720_e58752: f64 = (assign45720_e58750 * locals.var_s2);
            let assign45720_e58753: f64 = (assign45720_e58743 + assign45720_e58752);
            (locals.var_dl, locals.var_dl_dn5, locals.var_dl_dn6, locals.var_dl_dn7, locals.var_dl_dn8, ) = (assign45720_e58753, (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn5) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn5)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn5)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn5)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn5) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn5)) * locals.var_s2)), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn6) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn6)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn6)) + (((((((locals.var_alp2_i * locals.var_qbm_dc_dn6) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn6)) * locals.var_s2) + (assign45720_e58750 * locals.var_s2_dn6))), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn7) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn7)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn7)) + (((((((locals.var_alp2_i * locals.var_qbm_dc_dn7) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn7)) * locals.var_s2) + (assign45720_e58750 * locals.var_s2_dn7))), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn8) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45720_e58737 * locals.var_qim_dc_dn8)) * locals.var_qim1_dc) - (assign45720_e58739 * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45720_e58741 * locals.var_s1_dc_dn8)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn8) * locals.var_temp__blk936) + (assign45720_e58746 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign45720_e58748 * locals.var_temp__blk936_dn8)) * locals.var_s2)), );
            locals.var_dl_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45730_e58760: f64 = (1.0 + locals.var_dl);
            let assign45730_e58763: f64 = (locals.var_dl * locals.var_dl);
            let assign45730_e58764: f64 = (assign45730_e58760 + assign45730_e58763);
            let assign45730_e58765: f64 = (1.0 / assign45730_e58764);
            (locals.var_gdl_dc, locals.var_gdl_dc_dn5, locals.var_gdl_dc_dn6, locals.var_gdl_dc_dn7, locals.var_gdl_dc_dn8, ) = (assign45730_e58765, (-((locals.var_dl_dn5 + ((locals.var_dl_dn5 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn5))) / (assign45730_e58764 * assign45730_e58764))), (-((locals.var_dl_dn6 + ((locals.var_dl_dn6 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn6))) / (assign45730_e58764 * assign45730_e58764))), (-((locals.var_dl_dn7 + ((locals.var_dl_dn7 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn7))) / (assign45730_e58764 * assign45730_e58764))), (-((locals.var_dl_dn8 + ((locals.var_dl_dn8 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn8))) / (assign45730_e58764 * assign45730_e58764))), );
            locals.var_gdl_dc_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45740_e58771: f64 = (locals.var_gmob_dc * locals.var_gdl_dc);
            (locals.var_gmob_dl_dc, locals.var_gmob_dl_dc_dn5, locals.var_gmob_dl_dc_dn6, locals.var_gmob_dl_dc_dn7, locals.var_gmob_dl_dc_dn8, ) = (assign45740_e58771, ((locals.var_gmob_dc_dn5 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn5)), ((locals.var_gmob_dc_dn6 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn6)), ((locals.var_gmob_dc_dn7 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn7)), ((locals.var_gmob_dc_dn8 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn8)), );
            locals.var_gmob_dl_dc_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45750_e58777: f64 = (locals.var_thesateff_dc / locals.var_gmob_dl_dc);
            (locals.var_thesat1_dc, locals.var_thesat1_dc_dn5, locals.var_thesat1_dc_dn6, locals.var_thesat1_dc_dn7, locals.var_thesat1_dc_dn8, ) = (assign45750_e58777, (((locals.var_thesateff_dc_dn5 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn5)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn6 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn6)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn7 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn7)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn8 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn8)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), );
            locals.var_thesat1_dc_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45760_e58783: f64 = (locals.var_thesat1_dc * locals.var_thesat1_dc);
            let assign45760_e58785: f64 = (assign45760_e58783 * locals.var_dps_dc);
            let assign45760_e58787: f64 = (assign45760_e58785 * locals.var_dps_dc);
            (locals.var_zsat, locals.var_zsat_dn5, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8, ) = (assign45760_e58787, ((((((locals.var_thesat1_dc_dn5 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn5)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn5)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn5)), ((((((locals.var_thesat1_dc_dn6 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn6)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn6)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn6)), ((((((locals.var_thesat1_dc_dn7 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn7)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn7)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn7)), ((((((locals.var_thesat1_dc_dn8 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn8)) * locals.var_dps_dc) + (assign45760_e58783 * locals.var_dps_dc_dn8)) * locals.var_dps_dc) + (assign45760_e58785 * locals.var_dps_dc_dn8)), );
            locals.var_zsat_rv = 0.0;
        }

        let assign45770_e58792: f64 = (-1.0);
        let assign45770_e58793: f64 = if locals.var_chnl_type == assign45770_e58792 { 1.0 } else { 0.0 };
        locals.var_guard1219 = assign45770_e58793;
        locals.var_guard1219_rv = 0.0;

        if ((locals.var_guard1218 != 0.0) && (locals.var_guard1219 != 0.0)) {
            let assign45780_e58801: f64 = (locals.var_thesat1_dc * locals.var_dps_dc);
            let assign45780_e58802: f64 = (1.0 + assign45780_e58801);
            let assign45780_e58803: f64 = (locals.var_zsat / assign45780_e58802);
            (locals.var_zsat, locals.var_zsat_dn5, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8, ) = (assign45780_e58803, (((locals.var_zsat_dn5 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn5 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn5)))) / (assign45780_e58802 * assign45780_e58802)), (((locals.var_zsat_dn6 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn6 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn6)))) / (assign45780_e58802 * assign45780_e58802)), (((locals.var_zsat_dn7 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn7 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn7)))) / (assign45780_e58802 * assign45780_e58802)), (((locals.var_zsat_dn8 * assign45780_e58802) - (locals.var_zsat * ((locals.var_thesat1_dc_dn8 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn8)))) / (assign45780_e58802 * assign45780_e58802)), );
            locals.var_zsat_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45790_e58813: f64 = (2.0 * locals.var_zsat);
            let assign45790_e58814: f64 = (1.0 + assign45790_e58813);
            let assign45790_e58815: f64 = (assign45790_e58814).sqrt();
            let assign45790_e58816: f64 = (1.0 + assign45790_e58815);
            let assign45790_e58817: f64 = (locals.var_gmob_dl_dc * assign45790_e58816);
            let assign45790_e58818: f64 = (0.5 * assign45790_e58817);
            (locals.var_gvsat, locals.var_gvsat_dn5, locals.var_gvsat_dn6, locals.var_gvsat_dn7, locals.var_gvsat_dn8, ) = (assign45790_e58818, (0.5 * ((locals.var_gmob_dl_dc_dn5 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn5) / (2.0 * assign45790_e58815))))), (0.5 * ((locals.var_gmob_dl_dc_dn6 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn6) / (2.0 * assign45790_e58815))))), (0.5 * ((locals.var_gmob_dl_dc_dn7 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn7) / (2.0 * assign45790_e58815))))), (0.5 * ((locals.var_gmob_dl_dc_dn8 * assign45790_e58816) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn8) / (2.0 * assign45790_e58815))))), );
            locals.var_gvsat_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45800_e58824: f64 = (1.0 / locals.var_gvsat);
            (locals.var_gvsatinv_dc, locals.var_gvsatinv_dc_dn5, locals.var_gvsatinv_dc_dn6, locals.var_gvsatinv_dc_dn7, locals.var_gvsatinv_dc_dn8, ) = (assign45800_e58824, (-(locals.var_gvsat_dn5 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn6 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn7 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn8 / (locals.var_gvsat * locals.var_gvsat))), );
            locals.var_gvsatinv_dc_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45810_e58830: f64 = (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign45810_e58830, ((locals.var_gmob_dl_dc_dn5 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn5)), ((locals.var_gmob_dl_dc_dn6 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn6)), ((locals.var_gmob_dl_dc_dn7 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn7)), ((locals.var_gmob_dl_dc_dn8 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn8)), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45820_e58839: f64 = (locals.var_zsat * locals.var_temp__blk936);
            let assign45820_e58841: f64 = (assign45820_e58839 * locals.var_temp__blk936);
            let assign45820_e58842: f64 = (0.5 * assign45820_e58841);
            let assign45820_e58843: f64 = (1.0 + assign45820_e58842);
            let assign45820_e58844: f64 = (locals.var_alpha_dc * assign45820_e58843);
            (locals.var_alpha1, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, ) = (assign45820_e58844, ((locals.var_alpha_dc_dn5 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn5 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn5))))), ((locals.var_alpha_dc_dn6 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn6 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn6))))), ((locals.var_alpha_dc_dn7 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn7 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn7))))), ((locals.var_alpha_dc_dn8 * assign45820_e58843) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn8 * locals.var_temp__blk936) + (locals.var_zsat * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign45820_e58839 * locals.var_temp__blk936_dn8))))), );
            locals.var_alpha1_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45830_e58850: f64 = (locals.var_temp__blk936 * locals.var_qim1_dc);
            let assign45830_e58852: f64 = (assign45830_e58850 / locals.var_alpha1);
            (locals.var_h_dc, locals.var_h_dc_dn5, locals.var_h_dc_dn6, locals.var_h_dc_dn7, locals.var_h_dc_dn8, ) = (assign45830_e58852, (((((locals.var_temp__blk936_dn5 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn5)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn5)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk936_dn6 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn6)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn6)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk936_dn7 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn7)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn7)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk936_dn8 * locals.var_qim1_dc) + (locals.var_temp__blk936 * locals.var_qim1_dc_dn8)) * locals.var_alpha1) - (assign45830_e58850 * locals.var_alpha1_dn8)) / (locals.var_alpha1 * locals.var_alpha1)), );
            locals.var_h_dc_rv = 0.0;
        }

        if (locals.var_guard1218 != 0.0) {
            let assign45840_e58858: f64 = (locals.var_bet_i * locals.var_qim1_dc);
            let assign45840_e58860: f64 = (assign45840_e58858 * locals.var_dps_dc);
            let assign45840_e58862: f64 = (assign45840_e58860 * locals.var_gvsatinv_dc);
            (locals.var_i_ds, locals.var_i_ds_dn5, locals.var_i_ds_dn6, locals.var_i_ds_dn7, locals.var_i_ds_dn8, ) = (assign45840_e58862, (((((locals.var_bet_i * locals.var_qim1_dc_dn5) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn5)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn5)), (((((locals.var_bet_i * locals.var_qim1_dc_dn6) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn6)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn6)), (((((locals.var_bet_i * locals.var_qim1_dc_dn7) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn7)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn7)), (((((locals.var_bet_i * locals.var_qim1_dc_dn8) * locals.var_dps_dc) + (assign45840_e58858 * locals.var_dps_dc_dn8)) * locals.var_gvsatinv_dc) + (assign45840_e58860 * locals.var_gvsatinv_dc_dn8)), );
            locals.var_i_ds_rv = 0.0;
        }

        (locals.var_xs_ov, locals.var_xs_ov_dn5, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, ) = (0.0, 0.0, 0.0, 0.0, );
        locals.var_xs_ov_rv = 0.0;

        (locals.var_xd_ov, locals.var_xd_ov_dn5, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, ) = (0.0, 0.0, 0.0, 0.0, );
        locals.var_xd_ov_rv = 0.0;

        (locals.var_vovs, locals.var_vovs_dn5, locals.var_vovs_dn6, locals.var_vovs_dn7, ) = (0.0, 0.0, 0.0, 0.0, );
        locals.var_vovs_rv = 0.0;

        (locals.var_vovd, locals.var_vovd_dn5, locals.var_vovd_dn6, locals.var_vovd_dn7, ) = (0.0, 0.0, 0.0, 0.0, );
        locals.var_vovd_rv = 0.0;

        let assign45890_e58899: f64 = if (((((p.p40 != 0.0) && ((locals.var_igov_i > 0.0) || (locals.var_igovd_i > 0.0))) || ((p.p42 != 0.0) && ((locals.var_agidl_i > 0.0) || (locals.var_agidld_i > 0.0)))) || (locals.var_cgov_i > 0.0)) || (locals.var_cgovd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign45890_e58899;
        locals.var_guard1220_rv = 0.0;

        if (locals.var_guard1220 != 0.0) {
            let assign45900_e58905: f64 = (locals.var_xgs_ov * locals.var_xgs_ov);
            let assign45900_e58907: f64 = (assign45900_e58905 + locals.var_sp_ov_eps2_s);
            let assign45900_e58908: f64 = (assign45900_e58907).sqrt();
            let assign45900_e58909: f64 = (locals.var_xgs_ov + assign45900_e58908);
            let assign45900_e58910: f64 = (0.5 * assign45900_e58909);
            (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn5, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7, ) = (assign45900_e58910, (0.5 * (locals.var_xgs_ov_dn5 + (((locals.var_xgs_ov_dn5 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn5)) / (2.0 * assign45900_e58908)))), (0.5 * (locals.var_xgs_ov_dn6 + (((locals.var_xgs_ov_dn6 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn6)) / (2.0 * assign45900_e58908)))), (0.5 * (locals.var_xgs_ov_dn7 + (((locals.var_xgs_ov_dn7 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn7)) / (2.0 * assign45900_e58908)))), );
            locals.var_sp_ov_xg_rv = 0.0;
        }

        if (locals.var_guard1220 != 0.0) {
            let assign45910_e58915: f64 = (-locals.var_sp_ov_xg);
            let assign45910_e58918: f64 = (locals.var_gov2_s * 0.5);
            let assign45910_e58919: f64 = (assign45910_e58915 - assign45910_e58918);
            let assign45910_e58924: f64 = (locals.var_gov2_s * 0.25);
            let assign45910_e58925: f64 = (locals.var_sp_ov_xg + assign45910_e58924);
            let assign45910_e58927: f64 = (assign45910_e58925 + locals.var_sp_ov_a_s);
            let assign45910_e58928: f64 = (assign45910_e58927).sqrt();
            let assign45910_e58929: f64 = (locals.var_gov_s * assign45910_e58928);
            let assign45910_e58930: f64 = (assign45910_e58919 + assign45910_e58929);
            let assign45910_e58932: f64 = (assign45910_e58930 + locals.var_sp_ov_delta1_s);
            (locals.var_xs_ov, locals.var_xs_ov_dn5, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, ) = (assign45910_e58932, ((-locals.var_sp_ov_xg_dn5) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn5 / (2.0 * assign45910_e58928)))), ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn6 / (2.0 * assign45910_e58928)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn7 / (2.0 * assign45910_e58928)))), );
            locals.var_xs_ov_rv = 0.0;
        }

        if (locals.var_guard1220 != 0.0) {
            let assign45920_e58940: f64 = (locals.var_xgd_ov * locals.var_xgd_ov);
            let assign45920_e58942: f64 = (assign45920_e58940 + locals.var_sp_ov_eps2_d);
            let assign45920_e58943: f64 = (assign45920_e58942).sqrt();
            let assign45920_e58944: f64 = (locals.var_xgd_ov + assign45920_e58943);
            let assign45920_e58945: f64 = (0.5 * assign45920_e58944);
            (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn5, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7, ) = (assign45920_e58945, (0.5 * (locals.var_xgd_ov_dn5 + (((locals.var_xgd_ov_dn5 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn5)) / (2.0 * assign45920_e58943)))), (0.5 * (locals.var_xgd_ov_dn6 + (((locals.var_xgd_ov_dn6 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn6)) / (2.0 * assign45920_e58943)))), (0.5 * (locals.var_xgd_ov_dn7 + (((locals.var_xgd_ov_dn7 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn7)) / (2.0 * assign45920_e58943)))), );
            locals.var_sp_ov_xg_rv = 0.0;
        }

        if (locals.var_guard1220 != 0.0) {
            let assign45930_e58950: f64 = (-locals.var_sp_ov_xg);
            let assign45930_e58953: f64 = (locals.var_gov2_d * 0.5);
            let assign45930_e58954: f64 = (assign45930_e58950 - assign45930_e58953);
            let assign45930_e58959: f64 = (locals.var_gov2_d * 0.25);
            let assign45930_e58960: f64 = (locals.var_sp_ov_xg + assign45930_e58959);
            let assign45930_e58962: f64 = (assign45930_e58960 + locals.var_sp_ov_a_d);
            let assign45930_e58963: f64 = (assign45930_e58962).sqrt();
            let assign45930_e58964: f64 = (locals.var_gov_d * assign45930_e58963);
            let assign45930_e58965: f64 = (assign45930_e58954 + assign45930_e58964);
            let assign45930_e58967: f64 = (assign45930_e58965 + locals.var_sp_ov_delta1_d);
            (locals.var_xd_ov, locals.var_xd_ov_dn5, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, ) = (assign45930_e58967, ((-locals.var_sp_ov_xg_dn5) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn5 / (2.0 * assign45930_e58963)))), ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn6 / (2.0 * assign45930_e58963)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn7 / (2.0 * assign45930_e58963)))), );
            locals.var_xd_ov_rv = 0.0;
        }

        if (locals.var_guard1220 != 0.0) {
            let assign45940_e58972: f64 = (-locals.var_phita);
            let assign45940_e58975: f64 = (locals.var_xgs_ov + locals.var_xs_ov);
            let assign45940_e58976: f64 = (assign45940_e58972 * assign45940_e58975);
            (locals.var_vovs, locals.var_vovs_dn5, locals.var_vovs_dn6, locals.var_vovs_dn7, ) = (assign45940_e58976, (assign45940_e58972 * (locals.var_xgs_ov_dn5 + locals.var_xs_ov_dn5)), (assign45940_e58972 * (locals.var_xgs_ov_dn6 + locals.var_xs_ov_dn6)), (assign45940_e58972 * (locals.var_xgs_ov_dn7 + locals.var_xs_ov_dn7)), );
            locals.var_vovs_rv = 0.0;
        }

        if (locals.var_guard1220 != 0.0) {
            let assign45950_e58981: f64 = (-locals.var_phita);
            let assign45950_e58984: f64 = (locals.var_xgd_ov + locals.var_xd_ov);
            let assign45950_e58985: f64 = (assign45950_e58981 * assign45950_e58984);
            (locals.var_vovd, locals.var_vovd_dn5, locals.var_vovd_dn6, locals.var_vovd_dn7, ) = (assign45950_e58985, (assign45950_e58981 * (locals.var_xgd_ov_dn5 + locals.var_xd_ov_dn5)), (assign45950_e58981 * (locals.var_xgd_ov_dn6 + locals.var_xd_ov_dn6)), (assign45950_e58981 * (locals.var_xgd_ov_dn7 + locals.var_xd_ov_dn7)), );
            locals.var_vovd_rv = 0.0;
        }

        let assign46020_e58996: f64 = if p.p40 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign46020_e58996;
        locals.var_guard1221_rv = 0.0;

        let assign46030_e58999: f64 = if locals.var_igov_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign46030_e58999;
        locals.var_guard1222_rv = 0.0;

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46040_e59005: f64 = (locals.var_vovs * locals.var_vovs);
            let assign46040_e59007: f64 = (assign46040_e59005 + 1e-6);
            let assign46040_e59008: f64 = (assign46040_e59007).sqrt();
            let assign46040_e59010: f64 = (assign46040_e59008 * locals.var_inv_chib);
            (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, ) = (assign46040_e59010, ((((locals.var_vovs_dn5 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn5)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), 0.0, );
            locals.var_zg_rv = 0.0;
        }

        let assign46050_e59015: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign46050_e59015;
        locals.var_guard1223_rv = 0.0;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) && (locals.var_guard1223 != 0.0)) {
            let assign46060_e59024: f64 = (locals.var_zg + locals.var_gcqov);
            let assign46060_e59027: f64 = (locals.var_zg - locals.var_gcqov);
            let assign46060_e59030: f64 = (locals.var_zg - locals.var_gcqov);
            let assign46060_e59031: f64 = (assign46060_e59027 * assign46060_e59030);
            let assign46060_e59033: f64 = (assign46060_e59031 + 1e-6);
            let assign46060_e59034: f64 = (assign46060_e59033).sqrt();
            let assign46060_e59035: f64 = (assign46060_e59024 - assign46060_e59034);
            let assign46060_e59036: f64 = (0.5 * assign46060_e59035);
            (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, ) = (assign46060_e59036, (0.5 * (locals.var_zg_dn5 - (((locals.var_zg_dn5 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn5)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn6)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn7)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn8)) / (2.0 * assign46060_e59034)))), );
            locals.var_zg_rv = 0.0;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46070_e59044: f64 = (-1.5);
            let assign46070_e59049: f64 = (locals.var_gc3ov_i * locals.var_zg);
            let assign46070_e59050: f64 = (locals.var_gc2ov_i + assign46070_e59049);
            let assign46070_e59051: f64 = (locals.var_zg * assign46070_e59050);
            let assign46070_e59052: f64 = (assign46070_e59044 + assign46070_e59051);
            let assign46070_e59053: f64 = (locals.var_bov * assign46070_e59052);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46070_e59053, (locals.var_bov * ((locals.var_zg_dn5 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn5)))), (locals.var_bov * ((locals.var_zg_dn6 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn6)))), (locals.var_bov * ((locals.var_zg_dn7 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn7)))), (locals.var_bov * ((locals.var_zg_dn8 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn8)))), );
            locals.var_temp__blk936_rv = 0.0;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46130_e59139: f64 = (3.0 + locals.var_xs_ov);
            (locals.var_fs1, locals.var_fs1_dn5, locals.var_fs1_dn6, locals.var_fs1_dn7, ) = (assign46130_e59139, locals.var_xs_ov_dn5, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, );
            locals.var_fs1_rv = 0.0;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46140_e59146: f64 = (-3.0);
            let assign46140_e59148: f64 = (assign46140_e59146 - locals.var_gco_i);
            locals.var_fs2 = assign46140_e59148;
            locals.var_fs2_rv = 0.0;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46150_e59156: f64 = (30.0 * locals.var_vgsprime);
            (locals.var_fs3, locals.var_fs3_dn5, locals.var_fs3_dn6, locals.var_fs3_dn7, ) = (assign46150_e59156, (30.0 * locals.var_vgsprime_dn5), (30.0 * locals.var_vgsprime_dn6), (30.0 * locals.var_vgsprime_dn7), );
            locals.var_fs3_rv = 0.0;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46160_e59164: f64 = (4.0 - 0.9);
            locals.var_tme1 = assign46160_e59164;
            locals.var_tme1_rv = 0.0;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46170_e59172: f64 = (locals.var_fs1 + locals.var_fs3);
            (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, ) = (assign46170_e59172, (locals.var_fs1_dn5 + locals.var_fs3_dn5), (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), 0.0, );
            locals.var_tme2_rv = 0.0;
        }

    }
}

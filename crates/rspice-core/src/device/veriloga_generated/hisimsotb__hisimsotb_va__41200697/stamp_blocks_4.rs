#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard343 != 0.0)) {
            let assign19290_e24017: f64 = (locals.var_fb * locals.var_fb);
            let assign19290_e24020: f64 = (10.0 * 2.220446049250313e-16);
            let assign19290_e24021: f64 = (assign19290_e24017 + assign19290_e24020);
            (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn8, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12, ) = (assign19290_e24021, ((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)), ((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)), ((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)), ((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)), ((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)), ((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)), ((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)), ((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)), ((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)), );
            locals.var_xi0_rv = 0.0;
        }
        if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard343 != 0.0)) {
            let assign19300_e24035: f64 = (10.0 * 2.220446049250313e-16);
            let assign19300_e24036: f64 = (locals.var_fb + assign19300_e24035);
            (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn8, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12, ) = (assign19300_e24036, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn8, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, );
            locals.var_xi0p12_rv = 0.0;
        }
        if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard343 == 0.0)) {
            let assign19320_e24062: f64 = (locals.var_chi - 1.0);
            (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn8, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12, ) = (assign19320_e24062, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, );
            locals.var_xi0_rv = 0.0;
        }
        if ((((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) && (locals.var_guard343 == 0.0)) {
            let assign19330_e24075: f64 = (locals.var_xi0).sqrt();
            (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn8, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12, ) = (assign19330_e24075, (locals.var_xi0_dn0 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn2 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn4 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn5 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn6 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn8 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn10 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn11 / (2.0 * assign19330_e24075)), (locals.var_xi0_dn12 / (2.0 * assign19330_e24075)), );
            locals.var_xi0p12_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
            let assign19340_e24086: f64 = (locals.var_cnst0over * locals.var_xi0p12);
            (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn8, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, ) = (assign19340_e24086, ((locals.var_cnst0over_dn0 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_dn2 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_dn4 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_dn5 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_dn6 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_dn8 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_dn10 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_dn11 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_dn12 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn12)), );
            locals.var_qbuld_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
            let assign19350_e24098: f64 = (locals.var_fs02 + locals.var_xi0p12);
            let assign19350_e24099: f64 = (1.0 / assign19350_e24098);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign19350_e24099, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign19350_e24098 * assign19350_e24098))), (-((locals.var_fs02_dn12 + locals.var_xi0p12_dn12) / (assign19350_e24098 * assign19350_e24098))), );
            locals.var_t1_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
            let assign19360_e24110: f64 = (locals.var_cnst0over * locals.var_fs01);
            let assign19360_e24112: f64 = (assign19360_e24110 * locals.var_t1);
            (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn8, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, ) = (assign19360_e24112, ((((locals.var_cnst0over_dn0 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn0)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn0)), ((((locals.var_cnst0over_dn2 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn2)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn2)), ((((locals.var_cnst0over_dn4 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn4)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn4)), ((((locals.var_cnst0over_dn5 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn5)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn5)), ((((locals.var_cnst0over_dn6 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn6)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn6)), ((((locals.var_cnst0over_dn8 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn8)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn8)), ((((locals.var_cnst0over_dn10 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn10)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn10)), ((((locals.var_cnst0over_dn11 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn11)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn11)), ((((locals.var_cnst0over_dn12 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn12)) * locals.var_t1) + (assign19360_e24110 * locals.var_t1_dn12)), );
            locals.var_qiuld_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard329 == 0.0)) && (locals.var_guard336 != 0.0)) {
            let assign19370_e24123: f64 = (locals.var_qbuld + locals.var_qiuld);
            (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, ) = (assign19370_e24123, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), );
            locals.var_qsuld_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19380_e24129: f64 = (locals.var_qsuld - locals.var_qbuld);
            (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn8, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, ) = (assign19380_e24129, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), );
            locals.var_qiuld_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19390_e24135: f64 = (locals.var_weffcv_nf * locals.var_lov);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, ) = (assign19390_e24135, (locals.var_weffcv_nf_dn0 * locals.var_lov), (locals.var_weffcv_nf_dn2 * locals.var_lov), (locals.var_weffcv_nf_dn4 * locals.var_lov), (locals.var_weffcv_nf_dn5 * locals.var_lov), (locals.var_weffcv_nf_dn6 * locals.var_lov), (locals.var_weffcv_nf_dn8 * locals.var_lov), (locals.var_weffcv_nf_dn10 * locals.var_lov), (locals.var_weffcv_nf_dn11 * locals.var_lov), (locals.var_weffcv_nf_dn12 * locals.var_lov), );
            locals.var_t4_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overs != 0.0)) {
            let assign19400_e24143: f64 = (locals.var_t4 * locals.var_qsuld);
            (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn8, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, ) = (assign19400_e24143, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn12 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn12)), );
            locals.var_qovs_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overs != 0.0)) {
            let assign19410_e24151: f64 = (locals.var_t4 * locals.var_qbuld);
            (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn8, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, ) = (assign19410_e24151, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn12 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn12)), );
            locals.var_qbsld_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overd != 0.0)) {
            let assign19420_e24159: f64 = (locals.var_t4 * locals.var_qsuld);
            (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn8, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, ) = (assign19420_e24159, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn12 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn12)), );
            locals.var_qovd_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overd != 0.0)) {
            let assign19430_e24167: f64 = (locals.var_t4 * locals.var_qbuld);
            (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn8, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, ) = (assign19430_e24167, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn12 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn12)), );
            locals.var_qbdld_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19440_e24173: f64 = (1.0 - 1.0);
            let assign19440_e24175: f64 = (assign19440_e24173 / 2.0);
            locals.var_flg_ovloops = assign19440_e24175;
            locals.var_flg_ovloops_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19450_e24181: f64 = (1.0 + 1.0);
            let assign19450_e24183: f64 = (assign19450_e24181 / 2.0);
            locals.var_flg_ovloopd = assign19450_e24183;
            locals.var_flg_ovloopd_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19460_e24189: f64 = (locals.var_flg_ovloops * locals.var_modenml);
            let assign19460_e24192: f64 = (locals.var_flg_ovloopd * locals.var_modervs);
            let assign19460_e24193: f64 = (assign19460_e24189 + assign19460_e24192);
            locals.var_flg_overs = assign19460_e24193;
            locals.var_flg_overs_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19470_e24199: f64 = (locals.var_flg_ovloops * locals.var_modervs);
            let assign19470_e24202: f64 = (locals.var_flg_ovloopd * locals.var_modenml);
            let assign19470_e24203: f64 = (assign19470_e24199 + assign19470_e24202);
            locals.var_flg_overd = assign19470_e24203;
            locals.var_flg_overd_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_ovloops != 0.0)) {
            let assign19480_e24211: f64 = (locals.var_modenml * locals.var_vgse);
            let assign19480_e24215: f64 = (locals.var_vgse - locals.var_vdse);
            let assign19480_e24216: f64 = (locals.var_modervs * assign19480_e24215);
            let assign19480_e24217: f64 = (assign19480_e24211 + assign19480_e24216);
            (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn5, ) = (assign19480_e24217, ((locals.var_modenml * locals.var_vgse_dn0) + (locals.var_modervs * (locals.var_vgse_dn0 - locals.var_vdse_dn0))), ((locals.var_modenml * locals.var_vgse_dn2) + (locals.var_modervs * (locals.var_vgse_dn2 - locals.var_vdse_dn2))), ((locals.var_modenml * locals.var_vgse_dn5) + (locals.var_modervs * locals.var_vgse_dn5)), );
            locals.var_vgbgmt_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_ovloopd != 0.0)) {
            let assign19490_e24225: f64 = (locals.var_modervs * locals.var_vgse);
            let assign19490_e24229: f64 = (locals.var_vgse - locals.var_vdse);
            let assign19490_e24230: f64 = (locals.var_modenml * assign19490_e24229);
            let assign19490_e24231: f64 = (assign19490_e24225 + assign19490_e24230);
            (locals.var_vgbgmt, locals.var_vgbgmt_dn0, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn5, ) = (assign19490_e24231, ((locals.var_modervs * locals.var_vgse_dn0) + (locals.var_modenml * (locals.var_vgse_dn0 - locals.var_vdse_dn0))), ((locals.var_modervs * locals.var_vgse_dn2) + (locals.var_modenml * (locals.var_vgse_dn2 - locals.var_vdse_dn2))), ((locals.var_modervs * locals.var_vgse_dn5) + (locals.var_modenml * locals.var_vgse_dn5)), );
            locals.var_vgbgmt_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            locals.var_vxbgmt = 0.0;
            locals.var_vxbgmt_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19510_e24240: f64 = (-locals.var_vxbgmt);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, ) = (assign19510_e24240, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        let assign19520_e24245: f64 = if locals.var_t0 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard345 = assign19520_e24245;
        locals.var_guard345_rv = 0.0;
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19530_e24251: f64 = (locals.var_t0 - locals.var_vbs_bnd);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign19530_e24251, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19540_e24259: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign19540_e24259, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19550_e24267: f64 = (locals.var_t1 / locals.var_t2);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, ) = (assign19550_e24267, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn12 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn12)) / (locals.var_t2 * locals.var_t2)), );
            locals.var_tmf1_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19560_e24275: f64 = (locals.var_tmf1 * locals.var_tmf1);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, ) = (assign19560_e24275, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19570_e24283: f64 = (locals.var_tmf2 * locals.var_tmf1);
            (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn8, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, ) = (assign19570_e24283, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), );
            locals.var_tmf3_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19580_e24291: f64 = (locals.var_tmf2 * locals.var_tmf2);
            (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn8, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, ) = (assign19580_e24291, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), );
            locals.var_tmf4_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19590_e24300: f64 = (1.0 + locals.var_tmf1);
            let assign19590_e24302: f64 = (assign19590_e24300 + locals.var_tmf2);
            let assign19590_e24304: f64 = (assign19590_e24302 + locals.var_tmf3);
            let assign19590_e24306: f64 = (assign19590_e24304 + locals.var_tmf4);
            let assign19590_e24307: f64 = (1.0 / assign19590_e24306);
            (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, ) = (assign19590_e24307, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign19590_e24306 * assign19590_e24306))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign19590_e24306 * assign19590_e24306))), );
            locals.var_ty_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19600_e24316: f64 = (2.0 * locals.var_tmf1);
            let assign19600_e24317: f64 = (1.0 + assign19600_e24316);
            let assign19600_e24320: f64 = (3.0 * locals.var_tmf2);
            let assign19600_e24321: f64 = (assign19600_e24317 + assign19600_e24320);
            let assign19600_e24324: f64 = (4.0 * locals.var_tmf3);
            let assign19600_e24325: f64 = (assign19600_e24321 + assign19600_e24324);
            let assign19600_e24326: f64 = (-assign19600_e24325);
            let assign19600_e24328: f64 = (assign19600_e24326 * locals.var_ty);
            let assign19600_e24330: f64 = (assign19600_e24328 * locals.var_ty);
            (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn8, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12, ) = (assign19600_e24330, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn0)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn2)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn4)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn5)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn6)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn6)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn8)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn8)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn10)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn11)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn11)), (((((-(((2.0 * locals.var_tmf1_dn12) + (3.0 * locals.var_tmf2_dn12)) + (4.0 * locals.var_tmf3_dn12))) * locals.var_ty) + (assign19600_e24326 * locals.var_ty_dn12)) * locals.var_ty) + (assign19600_e24328 * locals.var_ty_dn12)), );
            locals.var_t11_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19610_e24339: f64 = (1.0 - locals.var_ty);
            let assign19610_e24340: f64 = (locals.var_t2 * assign19610_e24339);
            (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, ) = (assign19610_e24340, ((locals.var_t2_dn0 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn0))), ((locals.var_t2_dn2 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn2))), ((locals.var_t2_dn4 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn4))), ((locals.var_t2_dn5 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn5))), ((locals.var_t2_dn6 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn6))), ((locals.var_t2_dn8 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn8))), ((locals.var_t2_dn10 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn10))), ((locals.var_t2_dn11 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn11))), ((locals.var_t2_dn12 * assign19610_e24339) + (locals.var_t2 * (-locals.var_ty_dn12))), );
            locals.var_ty_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19620_e24347: f64 = (-locals.var_t11);
            (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn8, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12, ) = (assign19620_e24347, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn8), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn12), );
            locals.var_t11_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 != 0.0)) {
            let assign19630_e24355: f64 = (locals.var_vbs_bnd + locals.var_ty);
            (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, ) = (assign19630_e24355, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, );
            locals.var_t10_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard345 == 0.0)) {
            (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, ) = (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, );
            locals.var_t10_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19650_e24367: f64 = (-locals.var_t10);
            let assign19650_e24369: f64 = (assign19650_e24367 - 1e-12);
            (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12, ) = (assign19650_e24369, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn8), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn12), );
            locals.var_vxbgmtcl_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19660_e24375: f64 = (locals.var_cnst0over * locals.var_cox0_inv);
            (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn8, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn12, ) = (assign19660_e24375, (locals.var_cnst0over_dn0 * locals.var_cox0_inv), (locals.var_cnst0over_dn2 * locals.var_cox0_inv), (locals.var_cnst0over_dn4 * locals.var_cox0_inv), (locals.var_cnst0over_dn5 * locals.var_cox0_inv), (locals.var_cnst0over_dn6 * locals.var_cox0_inv), (locals.var_cnst0over_dn8 * locals.var_cox0_inv), (locals.var_cnst0over_dn10 * locals.var_cox0_inv), (locals.var_cnst0over_dn11 * locals.var_cox0_inv), (locals.var_cnst0over_dn12 * locals.var_cox0_inv), );
            locals.var_fac1_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19670_e24381: f64 = (locals.var_fac1 * locals.var_fac1);
            (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn8, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn12, ) = (assign19670_e24381, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn12 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn12)), );
            locals.var_fac1p2_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19680_e24386: f64 = (-locals.var_vgbgmt);
            let assign19680_e24388: f64 = (assign19680_e24386 + p.p39);
            (locals.var_vgpld, locals.var_vgpld_dn0, locals.var_vgpld_dn2, locals.var_vgpld_dn5, ) = (assign19680_e24388, (-locals.var_vgbgmt_dn0), (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn5), );
            locals.var_vgpld_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19690_e24394: f64 = (2.0 / locals.var_beta);
            let assign19690_e24397: f64 = (locals.var_mks_nover / locals.var_nin);
            let assign19690_e24398: f64 = (assign19690_e24397).ln();
            let assign19690_e24399: f64 = (assign19690_e24394 * assign19690_e24398);
            (locals.var_pb2over, locals.var_pb2over_dn0, locals.var_pb2over_dn2, locals.var_pb2over_dn4, locals.var_pb2over_dn5, locals.var_pb2over_dn6, locals.var_pb2over_dn8, locals.var_pb2over_dn10, locals.var_pb2over_dn11, locals.var_pb2over_dn12, ) = (assign19690_e24399, (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (((-((2.0 * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign19690_e24398) + (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign19690_e24397))), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), (assign19690_e24394 * ((-((locals.var_mks_nover * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))) / assign19690_e24397)), );
            locals.var_pb2over_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign19700_e24404: f64 = (-locals.var_vxbgmtcl);
            (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn12, ) = (assign19700_e24404, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn12), );
            locals.var_vgb_fb_ld_rv = 0.0;
        }
        let assign19710_e24409: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard346 = assign19710_e24409;
        locals.var_guard346_rv = 0.0;
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19730_e24423: f64 = (locals.var_beta * locals.var_cnst0over);
            let assign19730_e24424: f64 = (locals.var_cox0 / assign19730_e24423);
            (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, ) = (assign19730_e24424, (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn0)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn2)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * ((locals.var_beta_dn4 * locals.var_cnst0over) + (locals.var_beta * locals.var_cnst0over_dn4))) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn5)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn6)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn8)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn10)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn11)) / (assign19730_e24423 * assign19730_e24423))), (-((locals.var_cox0 * (locals.var_beta * locals.var_cnst0over_dn12)) / (assign19730_e24423 * assign19730_e24423))), );
            locals.var_ty_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19740_e24433: f64 = (3.0 * 1.414213562373095);
            let assign19740_e24435: f64 = (assign19740_e24433 * locals.var_ty);
            let assign19740_e24436: f64 = (2.0 + assign19740_e24435);
            (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn8, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12, ) = (assign19740_e24436, (assign19740_e24433 * locals.var_ty_dn0), (assign19740_e24433 * locals.var_ty_dn2), (assign19740_e24433 * locals.var_ty_dn4), (assign19740_e24433 * locals.var_ty_dn5), (assign19740_e24433 * locals.var_ty_dn6), (assign19740_e24433 * locals.var_ty_dn8), (assign19740_e24433 * locals.var_ty_dn10), (assign19740_e24433 * locals.var_ty_dn11), (assign19740_e24433 * locals.var_ty_dn12), );
            locals.var_ac41_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19750_e24444: f64 = (8.0 * locals.var_ac41);
            let assign19750_e24446: f64 = (assign19750_e24444 * locals.var_ac41);
            let assign19750_e24448: f64 = (assign19750_e24446 * locals.var_ac41);
            (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn8, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12, ) = (assign19750_e24448, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign19750_e24444 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign19750_e24446 * locals.var_ac41_dn12)), );
            locals.var_ac4_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19760_e24456: f64 = (locals.var_eg - locals.var_pb2over);
            (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn8, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12, ) = (assign19760_e24456, (locals.var_eg_dn0 - locals.var_pb2over_dn0), (locals.var_eg_dn2 - locals.var_pb2over_dn2), (locals.var_eg_dn4 - locals.var_pb2over_dn4), (locals.var_eg_dn5 - locals.var_pb2over_dn5), (locals.var_eg_dn6 - locals.var_pb2over_dn6), (locals.var_eg_dn8 - locals.var_pb2over_dn8), (locals.var_eg_dn10 - locals.var_pb2over_dn10), (locals.var_eg_dn11 - locals.var_pb2over_dn11), (locals.var_eg_dn12 - locals.var_pb2over_dn12), );
            locals.var_ps0_min_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19770_e24465: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
            let assign19770_e24466: f64 = (locals.var_beta * assign19770_e24465);
            (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, ) = (assign19770_e24466, (locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign19770_e24465) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), (locals.var_beta * (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * locals.var_vxbgmtcl_dn6), (locals.var_beta * locals.var_vxbgmtcl_dn8), (locals.var_beta * locals.var_vxbgmtcl_dn10), (locals.var_beta * locals.var_vxbgmtcl_dn11), (locals.var_beta * locals.var_vxbgmtcl_dn12), );
            locals.var_tx_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19780_e24474: f64 = (7.0 * 1.414213562373095);
            let assign19780_e24477: f64 = (9.0 * locals.var_ty);
            let assign19780_e24480: f64 = (locals.var_tx - 2.0);
            let assign19780_e24481: f64 = (assign19780_e24477 * assign19780_e24480);
            let assign19780_e24482: f64 = (assign19780_e24474 - assign19780_e24481);
            (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn8, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12, ) = (assign19780_e24482, (-(((9.0 * locals.var_ty_dn0) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn8) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn10) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn12) * assign19780_e24480) + (assign19780_e24477 * locals.var_tx_dn12))), );
            locals.var_ac31_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19790_e24490: f64 = (locals.var_ac31 * locals.var_ac31);
            (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn8, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12, ) = (assign19790_e24490, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)), );
            locals.var_ac3_rv = 0.0;
        }
        let assign19800_e24496: f64 = (locals.var_ac3 * 1e-8);
        let assign19800_e24497: f64 = if locals.var_ac4 < assign19800_e24496 { 1.0 } else { 0.0 };
        locals.var_guard347 = assign19800_e24497;
        locals.var_guard347_rv = 0.0;
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 != 0.0)) {
            let assign19810_e24504: f64 = (-7.0);
            let assign19810_e24506: f64 = (assign19810_e24504 * 1.414213562373095);
            let assign19810_e24508: f64 = (assign19810_e24506 + locals.var_ac31);
            let assign19810_e24511: f64 = (0.5 * locals.var_ac4);
            let assign19810_e24513: f64 = (assign19810_e24511 / locals.var_ac31);
            let assign19810_e24514: f64 = (assign19810_e24508 + assign19810_e24513);
            let assign19810_e24517: f64 = (9.0 * locals.var_ty);
            let assign19810_e24520: f64 = (locals.var_tx - 2.0);
            let assign19810_e24521: f64 = (assign19810_e24517 * assign19810_e24520);
            let assign19810_e24522: f64 = (assign19810_e24514 + assign19810_e24521);
            (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn8, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, ) = (assign19810_e24522, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn0) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn0))), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn2) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn2))), ((locals.var_ac31_dn4 + ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn4) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn4))), ((locals.var_ac31_dn5 + ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn5) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn5))), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn6) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn6))), ((locals.var_ac31_dn8 + ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn8) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn8))), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn10) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn10))), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn11) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn11))), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign19810_e24511 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + (((9.0 * locals.var_ty_dn12) * assign19810_e24520) + (assign19810_e24517 * locals.var_tx_dn12))), );
            locals.var_ac1_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19820_e24533: f64 = (locals.var_ac4 + locals.var_ac3);
            let assign19820_e24534: f64 = (assign19820_e24533).sqrt();
            (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn8, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12, ) = (assign19820_e24534, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign19820_e24534)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign19820_e24534)), );
            locals.var_ac2_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) && (locals.var_guard347 == 0.0)) {
            let assign19830_e24544: f64 = (-7.0);
            let assign19830_e24546: f64 = (assign19830_e24544 * 1.414213562373095);
            let assign19830_e24548: f64 = (assign19830_e24546 + locals.var_ac2);
            let assign19830_e24551: f64 = (9.0 * locals.var_ty);
            let assign19830_e24554: f64 = (locals.var_tx - 2.0);
            let assign19830_e24555: f64 = (assign19830_e24551 * assign19830_e24554);
            let assign19830_e24556: f64 = (assign19830_e24548 + assign19830_e24555);
            (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn8, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, ) = (assign19830_e24556, (locals.var_ac2_dn0 + (((9.0 * locals.var_ty_dn0) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn0))), (locals.var_ac2_dn2 + (((9.0 * locals.var_ty_dn2) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn2))), (locals.var_ac2_dn4 + (((9.0 * locals.var_ty_dn4) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn4))), (locals.var_ac2_dn5 + (((9.0 * locals.var_ty_dn5) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn5))), (locals.var_ac2_dn6 + (((9.0 * locals.var_ty_dn6) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn6))), (locals.var_ac2_dn8 + (((9.0 * locals.var_ty_dn8) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn8))), (locals.var_ac2_dn10 + (((9.0 * locals.var_ty_dn10) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn10))), (locals.var_ac2_dn11 + (((9.0 * locals.var_ty_dn11) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn11))), (locals.var_ac2_dn12 + (((9.0 * locals.var_ty_dn12) * assign19830_e24554) + (assign19830_e24551 * locals.var_tx_dn12))), );
            locals.var_ac1_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19840_e24564: f64 = (locals.var_ac1).powf(0.3333333333333333);
            (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn8, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12, ) = (assign19840_e24564, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign19840_e24564 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) }, );
            locals.var_acd_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19850_e24571: f64 = (-4.0);
            let assign19850_e24573: f64 = (assign19850_e24571 * 1.414213562373095);
            let assign19850_e24576: f64 = (12.0 * locals.var_ty);
            let assign19850_e24577: f64 = (assign19850_e24573 - assign19850_e24576);
            let assign19850_e24580: f64 = (2.0 * locals.var_acd);
            let assign19850_e24581: f64 = (assign19850_e24577 + assign19850_e24580);
            let assign19850_e24584: f64 = (1.414213562373095 * locals.var_acd);
            let assign19850_e24586: f64 = (assign19850_e24584 * locals.var_acd);
            let assign19850_e24587: f64 = (assign19850_e24581 + assign19850_e24586);
            (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn8, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12, ) = (assign19850_e24587, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign19850_e24584 * locals.var_acd_dn12))), );
            locals.var_acn_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19860_e24595: f64 = (locals.var_acn / locals.var_acd);
            (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, ) = (assign19860_e24595, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn12 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn12)) / (locals.var_acd * locals.var_acd)), );
            locals.var_chi_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19870_e24603: f64 = (locals.var_chi * locals.var_beta_inv);
            let assign19870_e24605: f64 = (assign19870_e24603 - locals.var_vxbgmtcl);
            (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn4, locals.var_psa_dn5, locals.var_psa_dn6, locals.var_psa_dn8, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12, ) = (assign19870_e24605, ((locals.var_chi_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_dn5 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_dn8 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_dn10 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), );
            locals.var_psa_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19880_e24613: f64 = (locals.var_psa + locals.var_vxbgmtcl);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign19880_e24613, (locals.var_psa_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_psa_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_psa_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_psa_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_psa_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_psa_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_psa_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_psa_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_psa_dn12 + locals.var_vxbgmtcl_dn12), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19890_e24621: f64 = (locals.var_t1 / locals.var_ps0_min);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign19890_e24621, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)), );
            locals.var_t2_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19900_e24631: f64 = (locals.var_t2 * locals.var_t2);
            let assign19900_e24632: f64 = (1.0 + assign19900_e24631);
            let assign19900_e24633: f64 = (assign19900_e24632).sqrt();
            let assign19900_e24634: f64 = (locals.var_t1 / assign19900_e24633);
            let assign19900_e24636: f64 = (assign19900_e24634 - locals.var_vxbgmtcl);
            (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn8, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, ) = (assign19900_e24636, ((((locals.var_t1_dn0 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn8 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn10 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn12 * assign19900_e24633) - (locals.var_t1 * (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign19900_e24633)))) / (assign19900_e24633 * assign19900_e24633)) - locals.var_vxbgmtcl_dn12), );
            locals.var_ps0ld_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            let assign19910_e24645: f64 = (locals.var_vgpld - locals.var_ps0ld);
            let assign19910_e24646: f64 = (locals.var_cox0 * assign19910_e24645);
            (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, ) = (assign19910_e24646, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (-locals.var_ps0ld_dn4)), (locals.var_cox0 * (locals.var_vgpld_dn5 - locals.var_ps0ld_dn5)), (locals.var_cox0 * (-locals.var_ps0ld_dn6)), (locals.var_cox0 * (-locals.var_ps0ld_dn8)), (locals.var_cox0 * (-locals.var_ps0ld_dn10)), (locals.var_cox0 * (-locals.var_ps0ld_dn11)), (locals.var_cox0 * (-locals.var_ps0ld_dn12)), );
            locals.var_qsuld_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 != 0.0)) {
            (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn8, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, ) = (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, );
            locals.var_qbuld_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, ) = (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_chi_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign19950_e24675: f64 = (locals.var_chi / locals.var_beta);
            let assign19950_e24677: f64 = (assign19950_e24675 - locals.var_vxbgmtcl);
            (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, ) = (assign19950_e24677, ((locals.var_chi_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((((locals.var_chi_dn4 * locals.var_beta) - (locals.var_chi * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_dn5 / locals.var_beta) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_dn8 / locals.var_beta) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_dn10 / locals.var_beta) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), );
            locals.var_ps0_inia_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign19960_e24689: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
            let assign19960_e24690: f64 = (locals.var_beta * assign19960_e24689);
            let assign19960_e24692: f64 = (assign19960_e24690 - 1.0);
            let assign19960_e24694: f64 = (-locals.var_chi);
            let assign19960_e24695: f64 = (assign19960_e24694).exp();
            let assign19960_e24696: f64 = (assign19960_e24692 + assign19960_e24695);
            let assign19960_e24697: f64 = (4.0 * assign19960_e24696);
            let assign19960_e24700: f64 = (locals.var_fac1p2 * locals.var_beta2);
            let assign19960_e24701: f64 = (assign19960_e24697 / assign19960_e24700);
            let assign19960_e24702: f64 = (1.0 + assign19960_e24701);
            (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, ) = (assign19960_e24702, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + (assign19960_e24695 * (-locals.var_chi_dn0)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + (assign19960_e24695 * (-locals.var_chi_dn2)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * (((locals.var_beta_dn4 * assign19960_e24689) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + (assign19960_e24695 * (-locals.var_chi_dn4)))) * assign19960_e24700) - (assign19960_e24697 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)) + (assign19960_e24695 * (-locals.var_chi_dn5)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn5 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn6) + (assign19960_e24695 * (-locals.var_chi_dn6)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn8) + (assign19960_e24695 * (-locals.var_chi_dn8)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn8 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn10) + (assign19960_e24695 * (-locals.var_chi_dn10)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn10 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn11) + (assign19960_e24695 * (-locals.var_chi_dn11)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn12) + (assign19960_e24695 * (-locals.var_chi_dn12)))) * assign19960_e24700) - (assign19960_e24697 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign19960_e24700 * assign19960_e24700)), );
            locals.var_tx_rv = 0.0;
        }
        let assign19970_e24708: f64 = (10.0 * 2.220446049250313e-16);
        let assign19970_e24709: f64 = if locals.var_tx < assign19970_e24708 { 1.0 } else { 0.0 };
        locals.var_guard348 = assign19970_e24709;
        locals.var_guard348_rv = 0.0;
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard348 != 0.0)) {
            let assign19980_e24718: f64 = (10.0 * 2.220446049250313e-16);
            (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, ) = (assign19980_e24718, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_tx_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign19990_e24728: f64 = (locals.var_fac1p2 * locals.var_beta);
            let assign19990_e24730: f64 = (assign19990_e24728 / 2.0);
            let assign19990_e24733: f64 = (locals.var_tx).sqrt();
            let assign19990_e24734: f64 = (1.0 - assign19990_e24733);
            let assign19990_e24735: f64 = (assign19990_e24730 * assign19990_e24734);
            let assign19990_e24736: f64 = (locals.var_vgpld + assign19990_e24735);
            (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, ) = (assign19990_e24736, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn0 / (2.0 * assign19990_e24733)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn2 / (2.0 * assign19990_e24733)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn4 / (2.0 * assign19990_e24733))))), (locals.var_vgpld_dn5 + ((((locals.var_fac1p2_dn5 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn5 / (2.0 * assign19990_e24733)))))), ((((locals.var_fac1p2_dn6 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn6 / (2.0 * assign19990_e24733))))), ((((locals.var_fac1p2_dn8 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn8 / (2.0 * assign19990_e24733))))), ((((locals.var_fac1p2_dn10 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn10 / (2.0 * assign19990_e24733))))), ((((locals.var_fac1p2_dn11 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn11 / (2.0 * assign19990_e24733))))), ((((locals.var_fac1p2_dn12 * locals.var_beta) / 2.0) * assign19990_e24734) + (assign19990_e24730 * (-(locals.var_tx_dn12 / (2.0 * assign19990_e24733))))), );
            locals.var_ps0_inia_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign20000_e24746: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
            let assign20000_e24747: f64 = (locals.var_beta * assign20000_e24746);
            (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, ) = (assign20000_e24747, (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign20000_e24746) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 + locals.var_vxbgmtcl_dn12)), );
            locals.var_chi_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign20010_e24759: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
            let assign20010_e24760: f64 = (locals.var_beta * assign20010_e24759);
            let assign20010_e24762: f64 = (assign20010_e24760 - 1.0);
            let assign20010_e24764: f64 = (-locals.var_chi);
            let assign20010_e24765: f64 = (assign20010_e24764).exp();
            let assign20010_e24766: f64 = (assign20010_e24762 + assign20010_e24765);
            let assign20010_e24767: f64 = (4.0 * assign20010_e24766);
            let assign20010_e24770: f64 = (locals.var_fac1p2 * locals.var_beta2);
            let assign20010_e24771: f64 = (assign20010_e24767 / assign20010_e24770);
            let assign20010_e24772: f64 = (1.0 + assign20010_e24771);
            (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, ) = (assign20010_e24772, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + (assign20010_e24765 * (-locals.var_chi_dn0)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + (assign20010_e24765 * (-locals.var_chi_dn2)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * (((locals.var_beta_dn4 * assign20010_e24759) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + (assign20010_e24765 * (-locals.var_chi_dn4)))) * assign20010_e24770) - (assign20010_e24767 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)) + (assign20010_e24765 * (-locals.var_chi_dn5)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn5 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn6) + (assign20010_e24765 * (-locals.var_chi_dn6)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn8) + (assign20010_e24765 * (-locals.var_chi_dn8)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn8 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn10) + (assign20010_e24765 * (-locals.var_chi_dn10)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn10 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn11) + (assign20010_e24765 * (-locals.var_chi_dn11)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), ((((4.0 * ((locals.var_beta * locals.var_vxbgmtcl_dn12) + (assign20010_e24765 * (-locals.var_chi_dn12)))) * assign20010_e24770) - (assign20010_e24767 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign20010_e24770 * assign20010_e24770)), );
            locals.var_tx_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign20020_e24778: f64 = (10.0 * 2.220446049250313e-16);
        let assign20020_e24779: f64 = if locals.var_tx < assign20020_e24778 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign20020_e24779;
        locals.var_guard349_rv = 0.0;
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard349 != 0.0)) {
            let assign20030_e24788: f64 = (10.0 * 2.220446049250313e-16);
            (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, ) = (assign20030_e24788, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_tx_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign20040_e24798: f64 = (locals.var_fac1p2 * locals.var_beta);
            let assign20040_e24800: f64 = (assign20040_e24798 / 2.0);
            let assign20040_e24803: f64 = (locals.var_tx).sqrt();
            let assign20040_e24804: f64 = (1.0 - assign20040_e24803);
            let assign20040_e24805: f64 = (assign20040_e24800 * assign20040_e24804);
            let assign20040_e24806: f64 = (locals.var_vgpld + assign20040_e24805);
            (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, ) = (assign20040_e24806, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn0 / (2.0 * assign20040_e24803)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn2 / (2.0 * assign20040_e24803)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn4 / (2.0 * assign20040_e24803))))), (locals.var_vgpld_dn5 + ((((locals.var_fac1p2_dn5 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn5 / (2.0 * assign20040_e24803)))))), ((((locals.var_fac1p2_dn6 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn6 / (2.0 * assign20040_e24803))))), ((((locals.var_fac1p2_dn8 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn8 / (2.0 * assign20040_e24803))))), ((((locals.var_fac1p2_dn10 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn10 / (2.0 * assign20040_e24803))))), ((((locals.var_fac1p2_dn11 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn11 / (2.0 * assign20040_e24803))))), ((((locals.var_fac1p2_dn12 * locals.var_beta) / 2.0) * assign20040_e24804) + (assign20040_e24800 * (-(locals.var_tx_dn12 / (2.0 * assign20040_e24803))))), );
            locals.var_ps0_inia_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign20050_e24816: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
            let assign20050_e24817: f64 = (locals.var_beta * assign20050_e24816);
            (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, ) = (assign20050_e24817, (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign20050_e24816) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 + locals.var_vxbgmtcl_dn12)), );
            locals.var_chi_rv = 0.0;
        }
        let assign20060_e24822: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard350 = assign20060_e24822;
        locals.var_guard350_rv = 0.0;
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20080_e24841: f64 = (9.0 * 1.414213562373095);
            let assign20080_e24842: f64 = (1.0 / assign20080_e24841);
            let assign20080_e24846: f64 = (7.0 * 0.049787068367863944);
            let assign20080_e24847: f64 = (5.0 + assign20080_e24846);
            let assign20080_e24851: f64 = (2.0 + 0.049787068367863944);
            let assign20080_e24852: f64 = (assign20080_e24851).sqrt();
            let assign20080_e24853: f64 = (54.0 * assign20080_e24852);
            let assign20080_e24854: f64 = (assign20080_e24847 / assign20080_e24853);
            let assign20080_e24855: f64 = (assign20080_e24842 - assign20080_e24854);
            locals.var_ta = assign20080_e24855;
            locals.var_ta_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20090_e24866: f64 = (1.0 + 0.049787068367863944);
            let assign20090_e24870: f64 = (2.0 + 0.049787068367863944);
            let assign20090_e24871: f64 = (assign20090_e24870).sqrt();
            let assign20090_e24872: f64 = (2.0 * assign20090_e24871);
            let assign20090_e24873: f64 = (assign20090_e24866 / assign20090_e24872);
            let assign20090_e24876: f64 = (1.414213562373095 / 3.0);
            let assign20090_e24877: f64 = (assign20090_e24873 - assign20090_e24876);
            locals.var_tb = assign20090_e24877;
            locals.var_tb_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20100_e24888: f64 = (1.0 / 1.414213562373095);
            let assign20100_e24892: f64 = (locals.var_beta * locals.var_fac1);
            let assign20100_e24893: f64 = (1.0 / assign20100_e24892);
            let assign20100_e24894: f64 = (assign20100_e24888 + assign20100_e24893);
            (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn8, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn12, ) = (assign20100_e24894, (-((locals.var_beta * locals.var_fac1_dn0) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn2) / (assign20100_e24892 * assign20100_e24892))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn5) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn6) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn8) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn10) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn11) / (assign20100_e24892 * assign20100_e24892))), (-((locals.var_beta * locals.var_fac1_dn12) / (assign20100_e24892 * assign20100_e24892))), );
            locals.var_tc_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20110_e24905: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
            let assign20110_e24906: f64 = (-assign20110_e24905);
            let assign20110_e24908: f64 = (assign20110_e24906 / locals.var_fac1);
            (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn8, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn12, ) = (assign20110_e24908, ((((-(locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5)) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn8) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn12) * locals.var_fac1) - (assign20110_e24906 * locals.var_fac1_dn12)) / (locals.var_fac1 * locals.var_fac1)), );
            locals.var_td_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20120_e24919: f64 = (locals.var_tb * locals.var_tb);
            let assign20120_e24921: f64 = (assign20120_e24919 * locals.var_tb);
            let assign20120_e24924: f64 = (27.0 * locals.var_ta);
            let assign20120_e24926: f64 = (assign20120_e24924 * locals.var_ta);
            let assign20120_e24928: f64 = (assign20120_e24926 * locals.var_ta);
            let assign20120_e24929: f64 = (assign20120_e24921 / assign20120_e24928);
            let assign20120_e24932: f64 = (locals.var_tb * locals.var_tc);
            let assign20120_e24935: f64 = (6.0 * locals.var_ta);
            let assign20120_e24937: f64 = (assign20120_e24935 * locals.var_ta);
            let assign20120_e24938: f64 = (assign20120_e24932 / assign20120_e24937);
            let assign20120_e24939: f64 = (assign20120_e24929 - assign20120_e24938);
            let assign20120_e24943: f64 = (2.0 * locals.var_ta);
            let assign20120_e24944: f64 = (locals.var_td / assign20120_e24943);
            let assign20120_e24945: f64 = (assign20120_e24939 + assign20120_e24944);
            (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn8, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn12, ) = (assign20120_e24945, ((-((locals.var_tb * locals.var_tc_dn0) / assign20120_e24937)) + (locals.var_td_dn0 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn2) / assign20120_e24937)) + (locals.var_td_dn2 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn4) / assign20120_e24937)) + (locals.var_td_dn4 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn5) / assign20120_e24937)) + (locals.var_td_dn5 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn6) / assign20120_e24937)) + (locals.var_td_dn6 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn8) / assign20120_e24937)) + (locals.var_td_dn8 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn10) / assign20120_e24937)) + (locals.var_td_dn10 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn11) / assign20120_e24937)) + (locals.var_td_dn11 / assign20120_e24943)), ((-((locals.var_tb * locals.var_tc_dn12) / assign20120_e24937)) + (locals.var_td_dn12 / assign20120_e24943)), );
            locals.var_tq_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20130_e24956: f64 = (3.0 * locals.var_ta);
            let assign20130_e24958: f64 = (assign20130_e24956 * locals.var_tc);
            let assign20130_e24961: f64 = (locals.var_tb * locals.var_tb);
            let assign20130_e24962: f64 = (assign20130_e24958 - assign20130_e24961);
            let assign20130_e24965: f64 = (9.0 * locals.var_ta);
            let assign20130_e24967: f64 = (assign20130_e24965 * locals.var_ta);
            let assign20130_e24968: f64 = (assign20130_e24962 / assign20130_e24967);
            (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn8, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn12, ) = (assign20130_e24968, ((assign20130_e24956 * locals.var_tc_dn0) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn2) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn4) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn5) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn6) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn8) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn10) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn11) / assign20130_e24967), ((assign20130_e24956 * locals.var_tc_dn12) / assign20130_e24967), );
            locals.var_tp_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20140_e24979: f64 = (locals.var_tq * locals.var_tq);
            let assign20140_e24982: f64 = (locals.var_tp * locals.var_tp);
            let assign20140_e24984: f64 = (assign20140_e24982 * locals.var_tp);
            let assign20140_e24985: f64 = (assign20140_e24979 + assign20140_e24984);
            let assign20140_e24986: f64 = (assign20140_e24985).sqrt();
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, ) = (assign20140_e24986, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn0))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn2))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn4))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn5))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn6))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn8))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn10))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn11))) / (2.0 * assign20140_e24986)), ((((locals.var_tq_dn12 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn12)) + ((((locals.var_tp_dn12 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn12)) * locals.var_tp) + (assign20140_e24982 * locals.var_tp_dn12))) / (2.0 * assign20140_e24986)), );
            locals.var_t5_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20150_e24996: f64 = (-locals.var_tq);
            let assign20150_e24998: f64 = (assign20150_e24996 + locals.var_t5);
            let assign20150_e25000: f64 = (assign20150_e24998).powf(0.3333333333333333);
            (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn8, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn12, ) = (assign20150_e25000, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign20150_e24998))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20150_e24998).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn12) + locals.var_t5_dn12))) } } else { (assign20150_e25000 * (0.3333333333333333 * (((-locals.var_tq_dn12) + locals.var_t5_dn12) / assign20150_e24998))) }, );
            locals.var_tu_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20160_e25011: f64 = (locals.var_tq + locals.var_t5);
            let assign20160_e25013: f64 = (assign20160_e25011).powf(0.3333333333333333);
            let assign20160_e25014: f64 = (-assign20160_e25013);
            (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn8, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn12, ) = (assign20160_e25014, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign20160_e25011))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign20160_e25011).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn12 + locals.var_t5_dn12))) } } else { (assign20160_e25013 * (0.3333333333333333 * ((locals.var_tq_dn12 + locals.var_t5_dn12) / assign20160_e25011))) }), );
            locals.var_tv_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20170_e25025: f64 = (locals.var_tu + locals.var_tv);
            let assign20170_e25029: f64 = (3.0 * locals.var_ta);
            let assign20170_e25030: f64 = (locals.var_tb / assign20170_e25029);
            let assign20170_e25031: f64 = (assign20170_e25025 - assign20170_e25030);
            (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, ) = (assign20170_e25031, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn12 + locals.var_tv_dn12), );
            locals.var_tx_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20180_e25042: f64 = (locals.var_tx * locals.var_beta_inv);
            let assign20180_e25044: f64 = (assign20180_e25042 - locals.var_vxbgmtcl);
            (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, ) = (assign20180_e25044, ((locals.var_tx_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), (((locals.var_tx_dn4 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), ((locals.var_tx_dn5 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn5), ((locals.var_tx_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_tx_dn8 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn8), ((locals.var_tx_dn10 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), );
            locals.var_ps0_inia_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard350 != 0.0)) {
            let assign20190_e25056: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
            let assign20190_e25057: f64 = (locals.var_beta * assign20190_e25056);
            (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, ) = (assign20190_e25057, (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign20190_e25056) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 + locals.var_vxbgmtcl_dn12)), );
            locals.var_chi_rv = 0.0;
        }
        let assign20200_e25062: f64 = if p.p30 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard351 = assign20200_e25062;
        locals.var_guard351_rv = 0.0;
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20220_e25082: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
            let assign20220_e25084: f64 = (assign20220_e25082 + 0.1);
            (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn12, ) = (assign20220_e25084, (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, (locals.var_vgpld_dn5 + locals.var_vxbgmtcl_dn5), locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn12, );
            locals.var_vgpld_shift_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20230_e25095: f64 = (-locals.var_vxbgmtcl);
            let assign20230_e25096: f64 = (locals.var_beta * assign20230_e25095);
            let assign20230_e25097: f64 = (assign20230_e25096).exp();
            let assign20230_e25099: f64 = (assign20230_e25097 + 1e-50);
            (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn12, ) = (assign20230_e25099, (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign20230_e25097 * ((locals.var_beta_dn4 * assign20230_e25095) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign20230_e25097 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), );
            locals.var_exp_bvbs_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20240_e25110: f64 = (locals.var_nin / locals.var_mks_nover);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, ) = (assign20240_e25110, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn4 / locals.var_mks_nover), (locals.var_nin_dn5 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn8 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20250_e25121: f64 = (locals.var_t0 * locals.var_t0);
            (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn8, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, ) = (assign20250_e25121, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)), );
            locals.var_cnst1over_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20260_e25132: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
            (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn8, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn12, ) = (assign20260_e25132, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn12)), );
            locals.var_gammachi_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20270_e25143: f64 = (locals.var_beta2 * locals.var_fac1p2);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, ) = (assign20270_e25143, (locals.var_beta2 * locals.var_fac1p2_dn0), (locals.var_beta2 * locals.var_fac1p2_dn2), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), (locals.var_beta2 * locals.var_fac1p2_dn5), (locals.var_beta2 * locals.var_fac1p2_dn6), (locals.var_beta2 * locals.var_fac1p2_dn8), (locals.var_beta2 * locals.var_fac1p2_dn10), (locals.var_beta2 * locals.var_fac1p2_dn11), (locals.var_beta2 * locals.var_fac1p2_dn12), );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20280_e25154: f64 = (locals.var_beta * locals.var_vgpld_shift);
            (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, ) = (assign20280_e25154, (locals.var_beta * locals.var_vgpld_shift_dn0), (locals.var_beta * locals.var_vgpld_shift_dn2), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), (locals.var_beta * locals.var_vgpld_shift_dn5), (locals.var_beta * locals.var_vgpld_shift_dn6), (locals.var_beta * locals.var_vgpld_shift_dn8), (locals.var_beta * locals.var_vgpld_shift_dn10), (locals.var_beta * locals.var_vgpld_shift_dn11), (locals.var_beta * locals.var_vgpld_shift_dn12), );
            locals.var_psi_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20290_e25165: f64 = (locals.var_gammachi * locals.var_t0);
            let assign20290_e25168: f64 = (locals.var_psi * locals.var_psi);
            let assign20290_e25169: f64 = (assign20290_e25165 + assign20290_e25168);
            let assign20290_e25170: f64 = (assign20290_e25169).ln();
            let assign20290_e25173: f64 = (locals.var_cnst1over * locals.var_t0);
            let assign20290_e25174: f64 = (assign20290_e25173).ln();
            let assign20290_e25175: f64 = (assign20290_e25170 - assign20290_e25174);
            let assign20290_e25178: f64 = (locals.var_beta * locals.var_vxbgmtcl);
            let assign20290_e25179: f64 = (assign20290_e25175 + assign20290_e25178);
            (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn8, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, ) = (assign20290_e25179, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign20290_e25169) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign20290_e25169) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign20290_e25169) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign20290_e25173)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign20290_e25169) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign20290_e25169) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign20290_e25169) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn8)), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign20290_e25169) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign20290_e25169) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign20290_e25169) - (((locals.var_cnst1over_dn12 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn12)) / assign20290_e25173)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), );
            locals.var_chi_1_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20300_e25190: f64 = (locals.var_psi - locals.var_chi_1);
            let assign20300_e25192: f64 = (assign20300_e25190 - 1.0);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, ) = (assign20300_e25192, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), );
            locals.var_tmf1_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20310_e25203: f64 = (4.0 * locals.var_psi);
            let assign20310_e25205: f64 = assign20310_e25203;
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, ) = (assign20310_e25205, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn12), );
            locals.var_tmf2_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let (assign20320_e25220, assign20320_e25220_d_n0, assign20320_e25220_d_n2, assign20320_e25220_d_n4, assign20320_e25220_d_n5, assign20320_e25220_d_n6, assign20320_e25220_d_n8, assign20320_e25220_d_n10, assign20320_e25220_d_n11, assign20320_e25220_d_n12,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    } else {
        let assign20320_e25219: f64 = (-locals.var_tmf2);
        (assign20320_e25219, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, ) = (assign20320_e25220, assign20320_e25220_d_n0, assign20320_e25220_d_n2, assign20320_e25220_d_n4, assign20320_e25220_d_n5, assign20320_e25220_d_n6, assign20320_e25220_d_n8, assign20320_e25220_d_n10, assign20320_e25220_d_n11, assign20320_e25220_d_n12, );
            locals.var_tmf2_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20330_e25231: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign20330_e25233: f64 = (assign20330_e25231 + locals.var_tmf2);
            let assign20330_e25234: f64 = (assign20330_e25233).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, ) = (assign20330_e25234, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20330_e25234)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign20330_e25234)), );
            locals.var_tmf2_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20340_e25247: f64 = (locals.var_tmf1 / locals.var_tmf2);
            let assign20340_e25248: f64 = (1.0 + assign20340_e25247);
            let assign20340_e25249: f64 = (0.5 * assign20340_e25248);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign20340_e25249, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), );
            locals.var_t1_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20350_e25263: f64 = 2.0;
            let assign20350_e25264: f64 = (locals.var_tmf1 + assign20350_e25263);
            let assign20350_e25266: f64 = (assign20350_e25264 / locals.var_tmf2);
            let assign20350_e25267: f64 = (1.0 - assign20350_e25266);
            let assign20350_e25268: f64 = (0.5 * assign20350_e25267);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign20350_e25268, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign20350_e25264 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), );
            locals.var_t2_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20360_e25281: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign20360_e25282: f64 = (0.5 * assign20360_e25281);
            let assign20360_e25283: f64 = (locals.var_psi - assign20360_e25282);
            (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn8, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, ) = (assign20360_e25283, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), );
            locals.var_chi_1_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20370_e25294: f64 = (locals.var_psi - locals.var_chi_1);
            (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, ) = (assign20370_e25294, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), );
            locals.var_psi_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20380_e25306: f64 = (locals.var_beta * 0.1);
            let assign20380_e25307: f64 = (locals.var_psi + assign20380_e25306);
            (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, ) = (assign20380_e25307, locals.var_psi_dn0, locals.var_psi_dn2, (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, );
            locals.var_psi_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20390_e25318: f64 = (locals.var_gammachi * locals.var_t0);
            let assign20390_e25321: f64 = (locals.var_psi * locals.var_psi);
            let assign20390_e25322: f64 = (assign20390_e25318 + assign20390_e25321);
            let assign20390_e25323: f64 = (assign20390_e25322).ln();
            let assign20390_e25326: f64 = (locals.var_cnst1over * locals.var_t0);
            let assign20390_e25327: f64 = (assign20390_e25326).ln();
            let assign20390_e25328: f64 = (assign20390_e25323 - assign20390_e25327);
            let assign20390_e25331: f64 = (locals.var_beta * locals.var_vxbgmtcl);
            let assign20390_e25332: f64 = (assign20390_e25328 + assign20390_e25331);
            (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn8, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn12, ) = (assign20390_e25332, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign20390_e25322) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign20390_e25322) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign20390_e25322) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign20390_e25326)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign20390_e25322) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign20390_e25322) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign20390_e25322) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn8)), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign20390_e25322) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign20390_e25322) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign20390_e25322) - (((locals.var_cnst1over_dn12 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn12)) / assign20390_e25326)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), );
            locals.var_chi_b_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20400_e25343: f64 = (locals.var_chi_b / locals.var_beta);
            let assign20400_e25345: f64 = (assign20400_e25343 - locals.var_vxbgmtcl);
            (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn4, locals.var_ps0_inib_dn5, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn8, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, ) = (assign20400_e25345, ((locals.var_chi_b_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_b_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((((locals.var_chi_b_dn4 * locals.var_beta) - (locals.var_chi_b * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_b_dn5 / locals.var_beta) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_b_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_b_dn8 / locals.var_beta) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_b_dn10 / locals.var_beta) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_b_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_b_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), );
            locals.var_ps0_inib_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn8, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn12, ) = (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, );
            locals.var_chi_a_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20420_e25365: f64 = (locals.var_chi_b - locals.var_chi_a);
            let assign20420_e25368: f64 = (0.0008 * 75.0);
            let assign20420_e25369: f64 = (assign20420_e25365 - assign20420_e25368);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, ) = (assign20420_e25369, (locals.var_chi_b_dn0 - locals.var_chi_a_dn0), (locals.var_chi_b_dn2 - locals.var_chi_a_dn2), (locals.var_chi_b_dn4 - locals.var_chi_a_dn4), (locals.var_chi_b_dn5 - locals.var_chi_a_dn5), (locals.var_chi_b_dn6 - locals.var_chi_a_dn6), (locals.var_chi_b_dn8 - locals.var_chi_a_dn8), (locals.var_chi_b_dn10 - locals.var_chi_a_dn10), (locals.var_chi_b_dn11 - locals.var_chi_a_dn11), (locals.var_chi_b_dn12 - locals.var_chi_a_dn12), );
            locals.var_tmf1_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20430_e25380: f64 = (4.0 * locals.var_chi_b);
            let assign20430_e25383: f64 = (0.0008 * 75.0);
            let assign20430_e25384: f64 = (assign20430_e25380 * assign20430_e25383);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, ) = (assign20430_e25384, ((4.0 * locals.var_chi_b_dn0) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn2) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn4) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn5) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn6) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn8) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn10) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn11) * assign20430_e25383), ((4.0 * locals.var_chi_b_dn12) * assign20430_e25383), );
            locals.var_tmf2_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let (assign20440_e25399, assign20440_e25399_d_n0, assign20440_e25399_d_n2, assign20440_e25399_d_n4, assign20440_e25399_d_n5, assign20440_e25399_d_n6, assign20440_e25399_d_n8, assign20440_e25399_d_n10, assign20440_e25399_d_n11, assign20440_e25399_d_n12,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    } else {
        let assign20440_e25398: f64 = (-locals.var_tmf2);
        (assign20440_e25398, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, ) = (assign20440_e25399, assign20440_e25399_d_n0, assign20440_e25399_d_n2, assign20440_e25399_d_n4, assign20440_e25399_d_n5, assign20440_e25399_d_n6, assign20440_e25399_d_n8, assign20440_e25399_d_n10, assign20440_e25399_d_n11, assign20440_e25399_d_n12, );
            locals.var_tmf2_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20450_e25410: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign20450_e25412: f64 = (assign20450_e25410 + locals.var_tmf2);
            let assign20450_e25413: f64 = (assign20450_e25412).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, ) = (assign20450_e25413, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20450_e25413)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign20450_e25413)), );
            locals.var_tmf2_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20460_e25426: f64 = (locals.var_tmf1 / locals.var_tmf2);
            let assign20460_e25427: f64 = (1.0 + assign20460_e25426);
            let assign20460_e25428: f64 = (0.5 * assign20460_e25427);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign20460_e25428, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), );
            locals.var_t1_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20470_e25442: f64 = (2.0 * 0.0008);
            let assign20470_e25444: f64 = (assign20470_e25442 * 75.0);
            let assign20470_e25445: f64 = (locals.var_tmf1 + assign20470_e25444);
            let assign20470_e25447: f64 = (assign20470_e25445 / locals.var_tmf2);
            let assign20470_e25448: f64 = (1.0 - assign20470_e25447);
            let assign20470_e25449: f64 = (0.5 * assign20470_e25448);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign20470_e25449, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign20470_e25445 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), );
            locals.var_t2_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard351 != 0.0)) {
            let assign20480_e25462: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign20480_e25463: f64 = (0.5 * assign20480_e25462);
            let assign20480_e25464: f64 = (locals.var_chi_b - assign20480_e25463);
            (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, ) = (assign20480_e25464, (locals.var_chi_b_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_chi_b_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_chi_b_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_chi_b_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), );
            locals.var_chi_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign20490_e25473: f64 = (locals.var_chi / locals.var_beta);
            let assign20490_e25475: f64 = (assign20490_e25473 - locals.var_vxbgmtcl);
            (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn8, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, ) = (assign20490_e25475, ((locals.var_chi_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((((locals.var_chi_dn4 * locals.var_beta) - (locals.var_chi * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn4), ((locals.var_chi_dn5 / locals.var_beta) - locals.var_vxbgmtcl_dn5), ((locals.var_chi_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi_dn8 / locals.var_beta) - locals.var_vxbgmtcl_dn8), ((locals.var_chi_dn10 / locals.var_beta) - locals.var_vxbgmtcl_dn10), ((locals.var_chi_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), );
            locals.var_ps0ld_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign20500_e25484: f64 = (locals.var_chi - 1.0);
            let assign20500_e25486: f64 = (-locals.var_chi);
            let assign20500_e25487: f64 = (assign20500_e25486).exp();
            let assign20500_e25488: f64 = (assign20500_e25484 + assign20500_e25487);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign20500_e25488, (locals.var_chi_dn0 + (assign20500_e25487 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign20500_e25487 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign20500_e25487 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign20500_e25487 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign20500_e25487 * (-locals.var_chi_dn6))), (locals.var_chi_dn8 + (assign20500_e25487 * (-locals.var_chi_dn8))), (locals.var_chi_dn10 + (assign20500_e25487 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign20500_e25487 * (-locals.var_chi_dn11))), (locals.var_chi_dn12 + (assign20500_e25487 * (-locals.var_chi_dn12))), );
            locals.var_t1_rv = 0.0;
        }
        let assign20510_e25494: f64 = (10.0 * 2.220446049250313e-16);
        let assign20510_e25495: f64 = if locals.var_t1 < assign20510_e25494 { 1.0 } else { 0.0 };
        locals.var_guard352 = assign20510_e25495;
        locals.var_guard352_rv = 0.0;
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard352 != 0.0)) {
            let assign20520_e25504: f64 = (10.0 * 2.220446049250313e-16);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign20520_e25504, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign20530_e25513: f64 = (locals.var_t1).sqrt();
            let assign20530_e25514: f64 = (locals.var_cnst0over * assign20530_e25513);
            (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn8, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, ) = (assign20530_e25514, ((locals.var_cnst0over_dn0 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn0 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn2 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn2 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn4 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn4 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn5 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn5 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn6 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn6 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn8 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn8 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn10 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn10 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn11 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn11 / (2.0 * assign20530_e25513)))), ((locals.var_cnst0over_dn12 * assign20530_e25513) + (locals.var_cnst0over * (locals.var_t1_dn12 / (2.0 * assign20530_e25513)))), );
            locals.var_qbuld_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) {
            let assign20540_e25524: f64 = (locals.var_vgpld - locals.var_ps0ld);
            let assign20540_e25525: f64 = (locals.var_cox0 * assign20540_e25524);
            (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, ) = (assign20540_e25525, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (-locals.var_ps0ld_dn4)), (locals.var_cox0 * (locals.var_vgpld_dn5 - locals.var_ps0ld_dn5)), (locals.var_cox0 * (-locals.var_ps0ld_dn6)), (locals.var_cox0 * (-locals.var_ps0ld_dn8)), (locals.var_cox0 * (-locals.var_ps0ld_dn10)), (locals.var_cox0 * (-locals.var_ps0ld_dn11)), (locals.var_cox0 * (-locals.var_ps0ld_dn12)), );
            locals.var_qsuld_rv = 0.0;
        }
        let assign20550_e25530: f64 = if p.p30 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign20550_e25530;
        locals.var_guard353_rv = 0.0;
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
            let assign20560_e25539: f64 = (-locals.var_vxbgmtcl);
            let assign20560_e25540: f64 = (locals.var_beta * assign20560_e25539);
            let assign20560_e25541: f64 = (assign20560_e25540).exp();
            (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn12, ) = (assign20560_e25541, (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign20560_e25541 * ((locals.var_beta_dn4 * assign20560_e25539) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign20560_e25541 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), );
            locals.var_exp_bvbs_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
            let assign20570_e25552: f64 = (locals.var_nin / locals.var_mks_nover);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, ) = (assign20570_e25552, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn4 / locals.var_mks_nover), (locals.var_nin_dn5 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn8 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), );
            locals.var_t0_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
            let assign20580_e25563: f64 = (locals.var_t0 * locals.var_t0);
            (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn8, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, ) = (assign20580_e25563, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)), );
            locals.var_cnst1over_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
            let assign20590_e25574: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
            (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn8, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn12, ) = (assign20590_e25574, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn12)), );
            locals.var_cfs1_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
            locals.var_flg_conv = 0.0;
            locals.var_flg_conv_rv = 0.0;
            locals.var_lp_s0 = 1.0;
            locals.var_lp_s0_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign20620_loop_guard: usize = 0;
        while {
            let assign20620_cond_e25604: f64 = (40.0 + 1.0);
            let assign20620_cond_e25606: f64 = if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_lp_s0 <= assign20620_cond_e25604)) { 1.0 } else { 0.0 };
            assign20620_cond_e25606 != 0.0
        } {
            assign20620_loop_guard += 1;
            assert!(assign20620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
                let assign20620_body0_e25616: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
                let assign20620_body0_e25617: f64 = (locals.var_beta * assign20620_body0_e25616);
                (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, ) = (assign20620_body0_e25617, (locals.var_beta * (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2)), ((locals.var_beta_dn4 * assign20620_body0_e25616) + (locals.var_beta * (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4))), (locals.var_beta * (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5)), (locals.var_beta * (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8)), (locals.var_beta * (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10)), (locals.var_beta * (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0ld_dn12 + locals.var_vxbgmtcl_dn12)), );
                locals.var_chi_rv = 0.0;
            }
            let assign20620_body1_e25622: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard354 = assign20620_body1_e25622;
            locals.var_guard354_rv = 0.0;
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
                let assign20620_body2_e25633: f64 = (locals.var_chi * locals.var_chi);
                let assign20620_body2_e25635: f64 = (assign20620_body2_e25633 * locals.var_chi);
                let assign20620_body2_e25639: f64 = (-0.07053654284009761);
                let assign20620_body2_e25642: f64 = (locals.var_chi * 0.006115288895133179);
                let assign20620_body2_e25643: f64 = (assign20620_body2_e25639 + assign20620_body2_e25642);
                let assign20620_body2_e25644: f64 = (locals.var_chi * assign20620_body2_e25643);
                let assign20620_body2_e25645: f64 = (0.29693154855771 + assign20620_body2_e25644);
                let assign20620_body2_e25646: f64 = (assign20620_body2_e25635 * assign20620_body2_e25645);
                (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn4, locals.var_fi_dn5, locals.var_fi_dn6, locals.var_fi_dn8, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn12, ) = (assign20620_body2_e25646, ((((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn0)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn0 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn0 * 0.006115288895133179))))), ((((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn2)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn2 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn2 * 0.006115288895133179))))), ((((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn4)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn4 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn4 * 0.006115288895133179))))), ((((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn5)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn5 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn5 * 0.006115288895133179))))), ((((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn6)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn6 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn6 * 0.006115288895133179))))), ((((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn8)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn8 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn8 * 0.006115288895133179))))), ((((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn10)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn10 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn10 * 0.006115288895133179))))), ((((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn11)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn11 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn11 * 0.006115288895133179))))), ((((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) * locals.var_chi) + (assign20620_body2_e25633 * locals.var_chi_dn12)) * assign20620_body2_e25645) + (assign20620_body2_e25635 * ((locals.var_chi_dn12 * assign20620_body2_e25643) + (locals.var_chi * (locals.var_chi_dn12 * 0.006115288895133179))))), );
                locals.var_fi_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
                let assign20620_body3_e25659: f64 = (locals.var_chi * locals.var_chi);
                let assign20620_body3_e25662: f64 = (3.0 * 0.29693154855771);
                let assign20620_body3_e25666: f64 = (-0.07053654284009761);
                let assign20620_body3_e25667: f64 = (4.0 * assign20620_body3_e25666);
                let assign20620_body3_e25670: f64 = (locals.var_chi * 5.0);
                let assign20620_body3_e25672: f64 = (assign20620_body3_e25670 * 0.006115288895133179);
                let assign20620_body3_e25673: f64 = (assign20620_body3_e25667 + assign20620_body3_e25672);
                let assign20620_body3_e25674: f64 = (locals.var_chi * assign20620_body3_e25673);
                let assign20620_body3_e25675: f64 = (assign20620_body3_e25662 + assign20620_body3_e25674);
                let assign20620_body3_e25676: f64 = (assign20620_body3_e25659 * assign20620_body3_e25675);
                (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn4, locals.var_fi_dchi_dn5, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn8, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn12, ) = (assign20620_body3_e25676, ((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn0 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn2 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn4 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn5 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn6 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn8 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn10 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn11 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn12 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn12)) * assign20620_body3_e25675) + (assign20620_body3_e25659 * ((locals.var_chi_dn12 * assign20620_body3_e25673) + (locals.var_chi * ((locals.var_chi_dn12 * 5.0) * 0.006115288895133179))))), );
                locals.var_fi_dchi_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
                let assign20620_body4_e25689: f64 = (locals.var_cfs1 * locals.var_fi);
                let assign20620_body4_e25691: f64 = (assign20620_body4_e25689 * locals.var_fi);
                (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn8, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, ) = (assign20620_body4_e25691, ((((locals.var_cfs1_dn0 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn0)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn2)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn4)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn5)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn6)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn6)), ((((locals.var_cfs1_dn8 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn8)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn8)), ((((locals.var_cfs1_dn10 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn10)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn10)), ((((locals.var_cfs1_dn11 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn11)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn11)), ((((locals.var_cfs1_dn12 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn12)) * locals.var_fi) + (assign20620_body4_e25689 * locals.var_fi_dn12)), );
                locals.var_fs01_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
                let assign20620_body5_e25704: f64 = (locals.var_cfs1 * locals.var_beta);
                let assign20620_body5_e25706: f64 = (assign20620_body5_e25704 * 2.0);
                let assign20620_body5_e25708: f64 = (assign20620_body5_e25706 * locals.var_fi);
                let assign20620_body5_e25710: f64 = (assign20620_body5_e25708 * locals.var_fi_dchi);
                (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, ) = (assign20620_body5_e25710, ((((((locals.var_cfs1_dn0 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn0)), ((((((locals.var_cfs1_dn2 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn2)), (((((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn4)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn4)), ((((((locals.var_cfs1_dn5 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn5)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn5)), ((((((locals.var_cfs1_dn6 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn6)), ((((((locals.var_cfs1_dn8 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn8)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn8)), ((((((locals.var_cfs1_dn10 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn10)), ((((((locals.var_cfs1_dn11 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn11)), ((((((locals.var_cfs1_dn12 * locals.var_beta) * 2.0) * locals.var_fi) + (assign20620_body5_e25706 * locals.var_fi_dn12)) * locals.var_fi_dchi) + (assign20620_body5_e25708 * locals.var_fi_dchi_dn12)), );
                locals.var_fs01_dps0_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
                let assign20620_body6_e25725: f64 = (-0.117851130197758);
                let assign20620_body6_e25730: f64 = (-0.00163730162779191);
                let assign20620_body6_e25733: f64 = (locals.var_chi * 6.36964918866352e-5);
                let assign20620_body6_e25734: f64 = (assign20620_body6_e25730 + assign20620_body6_e25733);
                let assign20620_body6_e25735: f64 = (locals.var_chi * assign20620_body6_e25734);
                let assign20620_body6_e25736: f64 = (0.0178800506338833 + assign20620_body6_e25735);
                let assign20620_body6_e25737: f64 = (locals.var_chi * assign20620_body6_e25736);
                let assign20620_body6_e25738: f64 = (assign20620_body6_e25725 + assign20620_body6_e25737);
                let assign20620_body6_e25739: f64 = (locals.var_chi * assign20620_body6_e25738);
                let assign20620_body6_e25740: f64 = (0.707106781186548 + assign20620_body6_e25739);
                let assign20620_body6_e25741: f64 = (locals.var_chi * assign20620_body6_e25740);
                (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn8, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, ) = (assign20620_body6_e25741, ((locals.var_chi_dn0 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn0 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn0 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn0 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn2 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn2 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn2 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn2 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn4 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn4 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn4 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn4 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn4 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn5 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn5 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn5 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn5 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn5 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn6 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn6 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn6 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn6 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn8 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn8 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn8 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn8 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn8 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn10 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn10 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn10 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn10 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn11 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn11 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn11 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn11 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn12 * assign20620_body6_e25740) + (locals.var_chi * ((locals.var_chi_dn12 * assign20620_body6_e25738) + (locals.var_chi * ((locals.var_chi_dn12 * assign20620_body6_e25736) + (locals.var_chi * ((locals.var_chi_dn12 * assign20620_body6_e25734) + (locals.var_chi * (locals.var_chi_dn12 * 6.36964918866352e-5))))))))), );
                locals.var_fb_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
                let assign20620_body7_e25756: f64 = (-0.117851130197758);
                let assign20620_body7_e25757: f64 = (2.0 * assign20620_body7_e25756);
                let assign20620_body7_e25761: f64 = (3.0 * 0.0178800506338833);
                let assign20620_body7_e25765: f64 = (-0.00163730162779191);
                let assign20620_body7_e25766: f64 = (4.0 * assign20620_body7_e25765);
                let assign20620_body7_e25769: f64 = (locals.var_chi * 5.0);
                let assign20620_body7_e25771: f64 = (assign20620_body7_e25769 * 6.36964918866352e-5);
                let assign20620_body7_e25772: f64 = (assign20620_body7_e25766 + assign20620_body7_e25771);
                let assign20620_body7_e25773: f64 = (locals.var_chi * assign20620_body7_e25772);
                let assign20620_body7_e25774: f64 = (assign20620_body7_e25761 + assign20620_body7_e25773);
                let assign20620_body7_e25775: f64 = (locals.var_chi * assign20620_body7_e25774);
                let assign20620_body7_e25776: f64 = (assign20620_body7_e25757 + assign20620_body7_e25775);
                let assign20620_body7_e25777: f64 = (locals.var_chi * assign20620_body7_e25776);
                let assign20620_body7_e25778: f64 = (0.707106781186548 + assign20620_body7_e25777);
                (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn4, locals.var_fb_dchi_dn5, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn8, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn12, ) = (assign20620_body7_e25778, ((locals.var_chi_dn0 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn0 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn0 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn2 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn2 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn2 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn4 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn4 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn4 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn5 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn5 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn5 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn6 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn6 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn6 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn8 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn8 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn8 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn10 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn10 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn10 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn11 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn11 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn11 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn12 * assign20620_body7_e25776) + (locals.var_chi * ((locals.var_chi_dn12 * assign20620_body7_e25774) + (locals.var_chi * ((locals.var_chi_dn12 * assign20620_body7_e25772) + (locals.var_chi * ((locals.var_chi_dn12 * 5.0) * 6.36964918866352e-5))))))), );
                locals.var_fb_dchi_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
                let assign20620_body8_e25791: f64 = (locals.var_fb * locals.var_fb);
                let assign20620_body8_e25793: f64 = (assign20620_body8_e25791 + locals.var_fs01);
                let assign20620_body8_e25795: f64 = (assign20620_body8_e25793 + 1e-50);
                let assign20620_body8_e25796: f64 = (assign20620_body8_e25795).sqrt();
                (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn8, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, ) = (assign20620_body8_e25796, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign20620_body8_e25796)), ((((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)) + locals.var_fs01_dn12) / (2.0 * assign20620_body8_e25796)), );
                locals.var_fs02_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 != 0.0)) {
                let assign20620_body9_e25809: f64 = (locals.var_beta * locals.var_fb_dchi);
                let assign20620_body9_e25811: f64 = (assign20620_body9_e25809 * 2.0);
                let assign20620_body9_e25813: f64 = (assign20620_body9_e25811 * locals.var_fb);
                let assign20620_body9_e25815: f64 = (assign20620_body9_e25813 + locals.var_fs01_dps0);
                let assign20620_body9_e25818: f64 = (locals.var_fs02 + locals.var_fs02);
                let assign20620_body9_e25819: f64 = (assign20620_body9_e25815 / assign20620_body9_e25818);
                (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, ) = (assign20620_body9_e25819, ((((((((locals.var_beta * locals.var_fb_dchi_dn0) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn0 + locals.var_fs02_dn0))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn2) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn2 + locals.var_fs02_dn2))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), (((((((((locals.var_beta_dn4 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn4)) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn4 + locals.var_fs02_dn4))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn5) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn5 + locals.var_fs02_dn5))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn6) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn6 + locals.var_fs02_dn6))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn8) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn8 + locals.var_fs02_dn8))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn10) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn10 + locals.var_fs02_dn10))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn11) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn11 + locals.var_fs02_dn11))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), ((((((((locals.var_beta * locals.var_fb_dchi_dn12) * 2.0) * locals.var_fb) + (assign20620_body9_e25811 * locals.var_fb_dn12)) + locals.var_fs01_dps0_dn12) * assign20620_body9_e25818) - (assign20620_body9_e25815 * (locals.var_fs02_dn12 + locals.var_fs02_dn12))) / (assign20620_body9_e25818 * assign20620_body9_e25818)), );
                locals.var_fs02_dps0_rv = 0.0;
            }
            let assign20620_body10_e25824: f64 = if locals.var_chi < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard355 = assign20620_body10_e25824;
            locals.var_guard355_rv = 0.0;
            if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 != 0.0)) {
                let assign20620_body11_e25837: f64 = (locals.var_chi).exp();
                (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn8, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, ) = (assign20620_body11_e25837, (assign20620_body11_e25837 * locals.var_chi_dn0), (assign20620_body11_e25837 * locals.var_chi_dn2), (assign20620_body11_e25837 * locals.var_chi_dn4), (assign20620_body11_e25837 * locals.var_chi_dn5), (assign20620_body11_e25837 * locals.var_chi_dn6), (assign20620_body11_e25837 * locals.var_chi_dn8), (assign20620_body11_e25837 * locals.var_chi_dn10), (assign20620_body11_e25837 * locals.var_chi_dn11), (assign20620_body11_e25837 * locals.var_chi_dn12), );
                locals.var_exp_chi_rv = 0.0;
            }
            if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 != 0.0)) {
                let assign20620_body12_e25854: f64 = (locals.var_exp_chi - 1.0);
                let assign20620_body12_e25855: f64 = (locals.var_cfs1 * assign20620_body12_e25854);
                (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn8, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, ) = (assign20620_body12_e25855, ((locals.var_cfs1_dn0 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn0)), ((locals.var_cfs1_dn2 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn2)), ((locals.var_cfs1_dn4 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn4)), ((locals.var_cfs1_dn5 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn5)), ((locals.var_cfs1_dn6 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn6)), ((locals.var_cfs1_dn8 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn8)), ((locals.var_cfs1_dn10 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn10)), ((locals.var_cfs1_dn11 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn11)), ((locals.var_cfs1_dn12 * assign20620_body12_e25854) + (locals.var_cfs1 * locals.var_exp_chi_dn12)), );
                locals.var_fs01_rv = 0.0;
            }
            if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 != 0.0)) {
                let assign20620_body13_e25871: f64 = (locals.var_cfs1 * locals.var_beta);
                let assign20620_body13_e25873: f64 = (assign20620_body13_e25871 * locals.var_exp_chi);
                (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, ) = (assign20620_body13_e25873, (((locals.var_cfs1_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn0)), (((locals.var_cfs1_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn4)), (((locals.var_cfs1_dn5 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn5)), (((locals.var_cfs1_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn6)), (((locals.var_cfs1_dn8 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn8)), (((locals.var_cfs1_dn10 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn10)), (((locals.var_cfs1_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn11)), (((locals.var_cfs1_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign20620_body13_e25871 * locals.var_exp_chi_dn12)), );
                locals.var_fs01_dps0_rv = 0.0;
            }
            if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 == 0.0)) {
                let assign20620_body14_e25890: f64 = (locals.var_beta * locals.var_ps0ld);
                let assign20620_body14_e25891: f64 = (assign20620_body14_e25890).exp();
                (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn12, ) = (assign20620_body14_e25891, (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn0)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn2)), (assign20620_body14_e25891 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn5)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn6)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn8)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn10)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn11)), (assign20620_body14_e25891 * (locals.var_beta * locals.var_ps0ld_dn12)), );
                locals.var_exp_bps0_rv = 0.0;
            }
            if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 == 0.0)) {
                let assign20620_body15_e25909: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
                let assign20620_body15_e25910: f64 = (locals.var_cnst1over * assign20620_body15_e25909);
                (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn8, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn12, ) = (assign20620_body15_e25910, ((locals.var_cnst1over_dn0 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((locals.var_cnst1over_dn2 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((locals.var_cnst1over_dn4 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((locals.var_cnst1over_dn5 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((locals.var_cnst1over_dn6 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((locals.var_cnst1over_dn8 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((locals.var_cnst1over_dn10 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((locals.var_cnst1over_dn11 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((locals.var_cnst1over_dn12 * assign20620_body15_e25909) + (locals.var_cnst1over * (locals.var_exp_bps0_dn12 - locals.var_exp_bvbs_dn12))), );
                locals.var_fs01_rv = 0.0;
            }
            if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) && (locals.var_guard355 == 0.0)) {
                let assign20620_body16_e25927: f64 = (locals.var_cnst1over * locals.var_beta);
                let assign20620_body16_e25929: f64 = (assign20620_body16_e25927 * locals.var_exp_bps0);
                (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn12, ) = (assign20620_body16_e25929, (((locals.var_cnst1over_dn0 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn0)), (((locals.var_cnst1over_dn2 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn2)), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn4)), (((locals.var_cnst1over_dn5 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn5)), (((locals.var_cnst1over_dn6 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn6)), (((locals.var_cnst1over_dn8 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn8)), (((locals.var_cnst1over_dn10 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn10)), (((locals.var_cnst1over_dn11 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn11)), (((locals.var_cnst1over_dn12 * locals.var_beta) * locals.var_exp_bps0) + (assign20620_body16_e25927 * locals.var_exp_bps0_dn12)), );
                locals.var_fs01_dps0_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) {
                let assign20620_body17_e25943: f64 = (locals.var_chi - 1.0);
                let assign20620_body17_e25945: f64 = (assign20620_body17_e25943 + locals.var_fs01);
                let assign20620_body17_e25946: f64 = (assign20620_body17_e25945).sqrt();
                (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn8, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn12, ) = (assign20620_body17_e25946, ((locals.var_chi_dn0 + locals.var_fs01_dn0) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn2 + locals.var_fs01_dn2) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn4 + locals.var_fs01_dn4) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn5 + locals.var_fs01_dn5) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn6 + locals.var_fs01_dn6) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn8 + locals.var_fs01_dn8) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn10 + locals.var_fs01_dn10) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn11 + locals.var_fs01_dn11) / (2.0 * assign20620_body17_e25946)), ((locals.var_chi_dn12 + locals.var_fs01_dn12) / (2.0 * assign20620_body17_e25946)), );
                locals.var_fs02_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard354 == 0.0)) {
                let assign20620_body18_e25960: f64 = (locals.var_beta + locals.var_fs01_dps0);
                let assign20620_body18_e25962: f64 = (assign20620_body18_e25960 / locals.var_fs02);
                let assign20620_body18_e25964: f64 = (assign20620_body18_e25962 * 0.5);
                (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn12, ) = (assign20620_body18_e25964, ((((locals.var_fs01_dps0_dn0 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn2 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), (((((locals.var_beta_dn4 + locals.var_fs01_dps0_dn4) * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn5 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn6 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn8 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn10 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn11 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), ((((locals.var_fs01_dps0_dn12 * locals.var_fs02) - (assign20620_body18_e25960 * locals.var_fs02_dn12)) / (locals.var_fs02 * locals.var_fs02)) * 0.5), );
                locals.var_fs02_dps0_rv = 0.0;
            }
            if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
                let assign20620_body19_e25975: f64 = (locals.var_vgpld - locals.var_ps0ld);
                let assign20620_body19_e25978: f64 = (locals.var_fac1 * locals.var_fs02);
                let assign20620_body19_e25979: f64 = (assign20620_body19_e25975 - assign20620_body19_e25978);
                (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn8, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn12, ) = (assign20620_body19_e25979, ((locals.var_vgpld_dn0 - locals.var_ps0ld_dn0) - ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), ((locals.var_vgpld_dn2 - locals.var_ps0ld_dn2) - ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), ((-locals.var_ps0ld_dn4) - ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), ((locals.var_vgpld_dn5 - locals.var_ps0ld_dn5) - ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), ((-locals.var_ps0ld_dn6) - ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), ((-locals.var_ps0ld_dn8) - ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), ((-locals.var_ps0ld_dn10) - ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), ((-locals.var_ps0ld_dn11) - ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), ((-locals.var_ps0ld_dn12) - ((locals.var_fac1_dn12 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn12))), );
                locals.var_fs0_rv = 0.0;
            }
            if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
                let assign20620_body20_e25989: f64 = (-1.0);
                let assign20620_body20_e25992: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
                let assign20620_body20_e25993: f64 = (assign20620_body20_e25989 - assign20620_body20_e25992);
                (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn12, ) = (assign20620_body20_e25993, (-((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0))), (-((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2))), (-((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4))), (-((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5))), (-((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6))), (-((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8))), (-((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10))), (-((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11))), (-((locals.var_fac1_dn12 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn12))), );
                locals.var_fs0_dps0_rv = 0.0;
            }
            let assign20620_body21_e25998: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard356 = assign20620_body21_e25998;
            locals.var_guard356_rv = 0.0;
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 != 0.0)) {
                let assign20620_body22_e26009: f64 = (40.0 + 1.0);
                locals.var_lp_s0 = assign20620_body22_e26009;
                locals.var_lp_s0_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 == 0.0)) {
                let assign20620_body23_e26022: f64 = (-locals.var_fs0);
                let assign20620_body23_e26024: f64 = (assign20620_body23_e26022 / locals.var_fs0_dps0);
                (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn8, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, ) = (assign20620_body23_e26024, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn12) * locals.var_fs0_dps0) - (assign20620_body23_e26022 * locals.var_fs0_dps0_dn12)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), );
                locals.var_dps0_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 == 0.0)) {
                let assign20620_body24_e26038: f64 = (0.5 * 0.1);
                let assign20620_body24_e26042: f64 = (locals.var_ps0ld).abs();
                let (assign20620_body24_e26047, assign20620_body24_e26047_d_n0, assign20620_body24_e26047_d_n2, assign20620_body24_e26047_d_n4, assign20620_body24_e26047_d_n5, assign20620_body24_e26047_d_n6, assign20620_body24_e26047_d_n8, assign20620_body24_e26047_d_n10, assign20620_body24_e26047_d_n11, assign20620_body24_e26047_d_n12,) = {
    if (1.0 >= assign20620_body24_e26042) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign20620_body24_e26046: f64 = (locals.var_ps0ld).abs();
        (assign20620_body24_e26046, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn12 } else { (-locals.var_ps0ld_dn12) },)
    }
};
                let assign20620_body24_e26048: f64 = (1.0 + assign20620_body24_e26047);
                let assign20620_body24_e26049: f64 = (assign20620_body24_e26038 * assign20620_body24_e26048);
                (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn8, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn12, ) = (assign20620_body24_e26049, (assign20620_body24_e26038 * assign20620_body24_e26047_d_n0), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n2), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n4), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n5), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n6), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n8), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n10), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n11), (assign20620_body24_e26038 * assign20620_body24_e26047_d_n12), );
                locals.var_dplim_rv = 0.0;
            }
            let assign20620_body25_e26053: f64 = (locals.var_dps0).abs();
            let assign20620_body25_e26055: f64 = if assign20620_body25_e26053 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard357 = assign20620_body25_e26055;
            locals.var_guard357_rv = 0.0;
            if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 == 0.0)) && (locals.var_guard357 != 0.0)) {
                let (assign20620_body26_e26074,) = {
    if (locals.var_dps0 >= 0.0) {
        (1.0,)
    } else {
        let assign20620_body26_e26073: f64 = (-1.0);
        (assign20620_body26_e26073,)
    }
};
                let assign20620_body26_e26075: f64 = (locals.var_dplim * assign20620_body26_e26074);
                (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn8, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, ) = (assign20620_body26_e26075, (locals.var_dplim_dn0 * assign20620_body26_e26074), (locals.var_dplim_dn2 * assign20620_body26_e26074), (locals.var_dplim_dn4 * assign20620_body26_e26074), (locals.var_dplim_dn5 * assign20620_body26_e26074), (locals.var_dplim_dn6 * assign20620_body26_e26074), (locals.var_dplim_dn8 * assign20620_body26_e26074), (locals.var_dplim_dn10 * assign20620_body26_e26074), (locals.var_dplim_dn11 * assign20620_body26_e26074), (locals.var_dplim_dn12 * assign20620_body26_e26074), );
                locals.var_dps0_rv = 0.0;
            }
            if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 == 0.0)) {
                let assign20620_body27_e26089: f64 = (locals.var_ps0ld + locals.var_dps0);
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn8, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, ) = (assign20620_body27_e26089, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn12 + locals.var_dps0_dn12), );
                locals.var_ps0ld_rv = 0.0;
            }
            let assign20620_body28_e26093: f64 = (locals.var_dps0).abs();
            let assign20620_body28_e26097: f64 = (locals.var_fs0).abs();
            let assign20620_body28_e26100: f64 = if ((assign20620_body28_e26093 <= 1e-12) && (assign20620_body28_e26097 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard358 = assign20620_body28_e26100;
            locals.var_guard358_rv = 0.0;
            if (((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard356 == 0.0)) && (locals.var_guard358 != 0.0)) {
                locals.var_flg_conv = 1.0;
                locals.var_flg_conv_rv = 0.0;
            }
            if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
                let assign20620_body30_e26123: f64 = (locals.var_lp_s0 + 1.0);
                locals.var_lp_s0 = assign20620_body30_e26123;
                locals.var_lp_s0_rv = 0.0;
            }
        }
        let assign20640_e26131: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard360 = assign20640_e26131;
        locals.var_guard360_rv = 0.0;
        if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard360 != 0.0)) {
            let assign20680_e26172: f64 = (locals.var_fb * locals.var_fb);
            let assign20680_e26175: f64 = (10.0 * 2.220446049250313e-16);
            let assign20680_e26176: f64 = (assign20680_e26172 + assign20680_e26175);
            (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn8, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12, ) = (assign20680_e26176, ((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)), ((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)), ((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)), ((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)), ((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)), ((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)), ((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)), ((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)), ((locals.var_fb_dn12 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn12)), );
            locals.var_xi0_rv = 0.0;
        }
        if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard360 != 0.0)) {
            let assign20690_e26190: f64 = (10.0 * 2.220446049250313e-16);
            let assign20690_e26191: f64 = (locals.var_fb + assign20690_e26190);
            (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn8, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12, ) = (assign20690_e26191, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn8, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn12, );
            locals.var_xi0p12_rv = 0.0;
        }
        if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard360 == 0.0)) {
            let assign20710_e26217: f64 = (locals.var_chi - 1.0);
            (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn8, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12, ) = (assign20710_e26217, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn8, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, );
            locals.var_xi0_rv = 0.0;
        }
        if ((((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) && (locals.var_guard360 == 0.0)) {
            let assign20720_e26230: f64 = (locals.var_xi0).sqrt();
            (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn8, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12, ) = (assign20720_e26230, (locals.var_xi0_dn0 / (2.0 * assign20720_e26230)), (locals.var_xi0_dn2 / (2.0 * assign20720_e26230)), (locals.var_xi0_dn4 / (2.0 * assign20720_e26230)), (locals.var_xi0_dn5 / (2.0 * assign20720_e26230)), (locals.var_xi0_dn6 / (2.0 * assign20720_e26230)), (locals.var_xi0_dn8 / (2.0 * assign20720_e26230)), (locals.var_xi0_dn10 / (2.0 * assign20720_e26230)), (locals.var_xi0_dn11 / (2.0 * assign20720_e26230)), (locals.var_xi0_dn12 / (2.0 * assign20720_e26230)), );
            locals.var_xi0p12_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
            let assign20730_e26241: f64 = (locals.var_cnst0over * locals.var_xi0p12);
            (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn8, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, ) = (assign20730_e26241, ((locals.var_cnst0over_dn0 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_dn2 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_dn4 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_dn5 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_dn6 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_dn8 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_dn10 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_dn11 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_dn12 * locals.var_xi0p12) + (locals.var_cnst0over * locals.var_xi0p12_dn12)), );
            locals.var_qbuld_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
            let assign20740_e26253: f64 = (locals.var_fs02 + locals.var_xi0p12);
            let assign20740_e26254: f64 = (1.0 / assign20740_e26253);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign20740_e26254, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign20740_e26253 * assign20740_e26253))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign20740_e26253 * assign20740_e26253))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign20740_e26253 * assign20740_e26253))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign20740_e26253 * assign20740_e26253))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign20740_e26253 * assign20740_e26253))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign20740_e26253 * assign20740_e26253))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign20740_e26253 * assign20740_e26253))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign20740_e26253 * assign20740_e26253))), (-((locals.var_fs02_dn12 + locals.var_xi0p12_dn12) / (assign20740_e26253 * assign20740_e26253))), );
            locals.var_t1_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
            let assign20750_e26265: f64 = (locals.var_cnst0over * locals.var_fs01);
            let assign20750_e26267: f64 = (assign20750_e26265 * locals.var_t1);
            (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn8, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, ) = (assign20750_e26267, ((((locals.var_cnst0over_dn0 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn0)) * locals.var_t1) + (assign20750_e26265 * locals.var_t1_dn0)), ((((locals.var_cnst0over_dn2 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn2)) * locals.var_t1) + (assign20750_e26265 * locals.var_t1_dn2)), ((((locals.var_cnst0over_dn4 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn4)) * locals.var_t1) + (assign20750_e26265 * locals.var_t1_dn4)), ((((locals.var_cnst0over_dn5 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn5)) * locals.var_t1) + (assign20750_e26265 * locals.var_t1_dn5)), ((((locals.var_cnst0over_dn6 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn6)) * locals.var_t1) + (assign20750_e26265 * locals.var_t1_dn6)), ((((locals.var_cnst0over_dn8 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn8)) * locals.var_t1) + (assign20750_e26265 * locals.var_t1_dn8)), ((((locals.var_cnst0over_dn10 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn10)) * locals.var_t1) + (assign20750_e26265 * locals.var_t1_dn10)), ((((locals.var_cnst0over_dn11 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn11)) * locals.var_t1) + (assign20750_e26265 * locals.var_t1_dn11)), ((((locals.var_cnst0over_dn12 * locals.var_fs01) + (locals.var_cnst0over * locals.var_fs01_dn12)) * locals.var_t1) + (assign20750_e26265 * locals.var_t1_dn12)), );
            locals.var_qiuld_rv = 0.0;
        }
        if (((locals.var_guard327 != 0.0) && (locals.var_guard346 == 0.0)) && (locals.var_guard353 != 0.0)) {
            let assign20760_e26278: f64 = (locals.var_qbuld + locals.var_qiuld);
            (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn8, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, ) = (assign20760_e26278, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), );
            locals.var_qsuld_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign20770_e26284: f64 = (locals.var_qsuld - locals.var_qbuld);
            (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn8, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, ) = (assign20770_e26284, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), );
            locals.var_qiuld_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign20780_e26290: f64 = (locals.var_weffcv_nf * locals.var_lov);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, ) = (assign20780_e26290, (locals.var_weffcv_nf_dn0 * locals.var_lov), (locals.var_weffcv_nf_dn2 * locals.var_lov), (locals.var_weffcv_nf_dn4 * locals.var_lov), (locals.var_weffcv_nf_dn5 * locals.var_lov), (locals.var_weffcv_nf_dn6 * locals.var_lov), (locals.var_weffcv_nf_dn8 * locals.var_lov), (locals.var_weffcv_nf_dn10 * locals.var_lov), (locals.var_weffcv_nf_dn11 * locals.var_lov), (locals.var_weffcv_nf_dn12 * locals.var_lov), );
            locals.var_t4_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overs != 0.0)) {
            let assign20790_e26298: f64 = (locals.var_t4 * locals.var_qsuld);
            (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn8, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, ) = (assign20790_e26298, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn12 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn12)), );
            locals.var_qovs_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overs != 0.0)) {
            let assign20800_e26306: f64 = (locals.var_t4 * locals.var_qbuld);
            (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn8, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, ) = (assign20800_e26306, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn12 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn12)), );
            locals.var_qbsld_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overd != 0.0)) {
            let assign20810_e26314: f64 = (locals.var_t4 * locals.var_qsuld);
            (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn8, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, ) = (assign20810_e26314, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn11 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn11)), ((locals.var_t4_dn12 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn12)), );
            locals.var_qovd_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overd != 0.0)) {
            let assign20820_e26322: f64 = (locals.var_t4 * locals.var_qbuld);
            (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn8, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, ) = (assign20820_e26322, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn12 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn12)), );
            locals.var_qbdld_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign20830_e26328: f64 = (locals.var_modervs * locals.var_cgso_given);
            let assign20830_e26331: f64 = (locals.var_modenml * locals.var_cgdo_given);
            let assign20830_e26332: f64 = (assign20830_e26328 + assign20830_e26331);
            locals.var_flg_overgiven = assign20830_e26332;
            locals.var_flg_overgiven_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overgiven != 0.0)) {
            let assign20840_e26340: f64 = (locals.var_modervs * p.p174);
            let assign20840_e26343: f64 = (locals.var_modenml * p.p173);
            let assign20840_e26344: f64 = (assign20840_e26340 + assign20840_e26343);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn4, locals.var_cgdoe_dn5, locals.var_cgdoe_dn6, locals.var_cgdoe_dn8, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, ) = (assign20840_e26344, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overgiven != 0.0)) {
            let assign20850_e26352: f64 = (-locals.var_weffcv_nf);
            let assign20850_e26353: f64 = (locals.var_cgdoe * assign20850_e26352);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn4, locals.var_cgdoe_dn5, locals.var_cgdoe_dn6, locals.var_cgdoe_dn8, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, ) = (assign20850_e26353, ((locals.var_cgdoe_dn0 * assign20850_e26352) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn0))), ((locals.var_cgdoe_dn2 * assign20850_e26352) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn2))), ((locals.var_cgdoe_dn4 * assign20850_e26352) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn4))), ((locals.var_cgdoe_dn5 * assign20850_e26352) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn5))), ((locals.var_cgdoe_dn6 * assign20850_e26352) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn6))), ((locals.var_cgdoe_dn8 * assign20850_e26352) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn8))), ((locals.var_cgdoe_dn10 * assign20850_e26352) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn10))), ((locals.var_cgdoe_dn11 * assign20850_e26352) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn11))), ((locals.var_cgdoe_dn12 * assign20850_e26352) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn12))), );
            locals.var_cgdoe_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overgiven != 0.0)) {
            let assign20860_e26361: f64 = (-locals.var_cgdoe);
            let assign20860_e26364: f64 = (locals.var_vgs - locals.var_vds);
            let assign20860_e26365: f64 = (assign20860_e26361 * assign20860_e26364);
            let assign20860_e26366: f64 = (locals.var_qgod + assign20860_e26365);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn8, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, ) = (assign20860_e26366, (locals.var_qgod_dn0 + (((-locals.var_cgdoe_dn0) * assign20860_e26364) + (assign20860_e26361 * (-locals.var_vds_dn0)))), (locals.var_qgod_dn2 + (((-locals.var_cgdoe_dn2) * assign20860_e26364) + (assign20860_e26361 * (-locals.var_vds_dn2)))), (locals.var_qgod_dn4 + (((-locals.var_cgdoe_dn4) * assign20860_e26364) + (assign20860_e26361 * (-locals.var_vds_dn4)))), (locals.var_qgod_dn5 + (((-locals.var_cgdoe_dn5) * assign20860_e26364) + (assign20860_e26361 * (locals.var_vgs_dn5 - locals.var_vds_dn5)))), (locals.var_qgod_dn6 + (((-locals.var_cgdoe_dn6) * assign20860_e26364) + (assign20860_e26361 * (-locals.var_vds_dn6)))), (locals.var_qgod_dn8 + (((-locals.var_cgdoe_dn8) * assign20860_e26364) + (assign20860_e26361 * (-locals.var_vds_dn8)))), (locals.var_qgod_dn10 + (((-locals.var_cgdoe_dn10) * assign20860_e26364) + (assign20860_e26361 * (-locals.var_vds_dn10)))), (locals.var_qgod_dn11 + (((-locals.var_cgdoe_dn11) * assign20860_e26364) + (assign20860_e26361 * (locals.var_vgs_dn11 - locals.var_vds_dn11)))), (locals.var_qgod_dn12 + (((-locals.var_cgdoe_dn12) * assign20860_e26364) + (assign20860_e26361 * (locals.var_vgs_dn12 - locals.var_vds_dn12)))), );
            locals.var_qgod_rv = 0.0;
        }
        if (locals.var_guard327 != 0.0) {
            let assign20870_e26372: f64 = (locals.var_modenml * locals.var_cgso_given);
            let assign20870_e26375: f64 = (locals.var_modervs * locals.var_cgdo_given);
            let assign20870_e26376: f64 = (assign20870_e26372 + assign20870_e26375);
            locals.var_flg_overgiven = assign20870_e26376;
            locals.var_flg_overgiven_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overgiven != 0.0)) {
            let assign20880_e26384: f64 = (locals.var_modenml * p.p174);
            let assign20880_e26387: f64 = (locals.var_modervs * p.p173);
            let assign20880_e26388: f64 = (assign20880_e26384 + assign20880_e26387);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn4, locals.var_cgsoe_dn5, locals.var_cgsoe_dn6, locals.var_cgsoe_dn8, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, ) = (assign20880_e26388, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overgiven != 0.0)) {
            let assign20890_e26396: f64 = (-locals.var_weffcv_nf);
            let assign20890_e26397: f64 = (locals.var_cgsoe * assign20890_e26396);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn4, locals.var_cgsoe_dn5, locals.var_cgsoe_dn6, locals.var_cgsoe_dn8, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, ) = (assign20890_e26397, ((locals.var_cgsoe_dn0 * assign20890_e26396) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn0))), ((locals.var_cgsoe_dn2 * assign20890_e26396) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn2))), ((locals.var_cgsoe_dn4 * assign20890_e26396) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn4))), ((locals.var_cgsoe_dn5 * assign20890_e26396) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn5))), ((locals.var_cgsoe_dn6 * assign20890_e26396) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn6))), ((locals.var_cgsoe_dn8 * assign20890_e26396) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn8))), ((locals.var_cgsoe_dn10 * assign20890_e26396) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn10))), ((locals.var_cgsoe_dn11 * assign20890_e26396) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn11))), ((locals.var_cgsoe_dn12 * assign20890_e26396) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn12))), );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((locals.var_guard327 != 0.0) && (locals.var_flg_overgiven != 0.0)) {
            let assign20900_e26405: f64 = (-locals.var_cgsoe);
            let assign20900_e26407: f64 = (assign20900_e26405 * locals.var_vgs);
            let assign20900_e26408: f64 = (locals.var_qgos + assign20900_e26407);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn8, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, ) = (assign20900_e26408, (locals.var_qgos_dn0 + ((-locals.var_cgsoe_dn0) * locals.var_vgs)), (locals.var_qgos_dn2 + ((-locals.var_cgsoe_dn2) * locals.var_vgs)), (locals.var_qgos_dn4 + ((-locals.var_cgsoe_dn4) * locals.var_vgs)), (locals.var_qgos_dn5 + (((-locals.var_cgsoe_dn5) * locals.var_vgs) + (assign20900_e26405 * locals.var_vgs_dn5))), (locals.var_qgos_dn6 + ((-locals.var_cgsoe_dn6) * locals.var_vgs)), (locals.var_qgos_dn8 + ((-locals.var_cgsoe_dn8) * locals.var_vgs)), (locals.var_qgos_dn10 + ((-locals.var_cgsoe_dn10) * locals.var_vgs)), (locals.var_qgos_dn11 + (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign20900_e26405 * locals.var_vgs_dn11))), (locals.var_qgos_dn12 + (((-locals.var_cgsoe_dn12) * locals.var_vgs) + (assign20900_e26405 * locals.var_vgs_dn12))), );
            locals.var_qgos_rv = 0.0;
        }
        let assign20910_e26423: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgdo_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgso_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard362 = assign20910_e26423;
        locals.var_guard362_rv = 0.0;
        let assign20920_e26426: f64 = if p.p175 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard363 = assign20920_e26426;
        locals.var_guard363_rv = 0.0;
        if (((locals.var_guard327 == 0.0) && (locals.var_guard362 != 0.0)) && (locals.var_guard363 != 0.0)) {
            let assign20930_e26434: f64 = (-locals.var_cox0);
            let assign20930_e26436: f64 = (assign20930_e26434 * p.p175);
            let assign20930_e26438: f64 = (assign20930_e26436 * locals.var_weffcv_nf);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn4, locals.var_cgdoe_dn5, locals.var_cgdoe_dn6, locals.var_cgdoe_dn8, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, ) = (assign20930_e26438, (assign20930_e26436 * locals.var_weffcv_nf_dn0), (assign20930_e26436 * locals.var_weffcv_nf_dn2), (assign20930_e26436 * locals.var_weffcv_nf_dn4), (assign20930_e26436 * locals.var_weffcv_nf_dn5), (assign20930_e26436 * locals.var_weffcv_nf_dn6), (assign20930_e26436 * locals.var_weffcv_nf_dn8), (assign20930_e26436 * locals.var_weffcv_nf_dn10), (assign20930_e26436 * locals.var_weffcv_nf_dn11), (assign20930_e26436 * locals.var_weffcv_nf_dn12), );
            locals.var_cgdoe_rv = 0.0;
        }
        if (((locals.var_guard327 == 0.0) && (locals.var_guard362 != 0.0)) && (locals.var_guard363 == 0.0)) {
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn4, locals.var_cgdoe_dn5, locals.var_cgdoe_dn6, locals.var_cgdoe_dn8, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }
        if ((locals.var_guard327 == 0.0) && (locals.var_guard362 == 0.0)) {
            let assign20950_e26458: f64 = (locals.var_modervs * p.p174);
            let assign20950_e26461: f64 = (locals.var_modenml * p.p173);
            let assign20950_e26462: f64 = (assign20950_e26458 + assign20950_e26461);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn4, locals.var_cgdoe_dn5, locals.var_cgdoe_dn6, locals.var_cgdoe_dn8, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, ) = (assign20950_e26462, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgdoe_rv = 0.0;
        }
        if ((locals.var_guard327 == 0.0) && (locals.var_guard362 == 0.0)) {
            let assign20960_e26472: f64 = (-locals.var_weffcv_nf);
            let assign20960_e26473: f64 = (locals.var_cgdoe * assign20960_e26472);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn4, locals.var_cgdoe_dn5, locals.var_cgdoe_dn6, locals.var_cgdoe_dn8, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, ) = (assign20960_e26473, ((locals.var_cgdoe_dn0 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn0))), ((locals.var_cgdoe_dn2 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn2))), ((locals.var_cgdoe_dn4 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn4))), ((locals.var_cgdoe_dn5 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn5))), ((locals.var_cgdoe_dn6 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn6))), ((locals.var_cgdoe_dn8 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn8))), ((locals.var_cgdoe_dn10 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn10))), ((locals.var_cgdoe_dn11 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn11))), ((locals.var_cgdoe_dn12 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn12))), );
            locals.var_cgdoe_rv = 0.0;
        }
        if (locals.var_guard327 == 0.0) {
            let assign20970_e26479: f64 = (-locals.var_cgdoe);
            let assign20970_e26482: f64 = (locals.var_vgs - locals.var_vds);
            let assign20970_e26483: f64 = (assign20970_e26479 * assign20970_e26482);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn8, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, ) = (assign20970_e26483, (((-locals.var_cgdoe_dn0) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn0))), (((-locals.var_cgdoe_dn2) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn2))), (((-locals.var_cgdoe_dn4) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn4))), (((-locals.var_cgdoe_dn5) * assign20970_e26482) + (assign20970_e26479 * (locals.var_vgs_dn5 - locals.var_vds_dn5))), (((-locals.var_cgdoe_dn6) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn6))), (((-locals.var_cgdoe_dn8) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn8))), (((-locals.var_cgdoe_dn10) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn10))), (((-locals.var_cgdoe_dn11) * assign20970_e26482) + (assign20970_e26479 * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (((-locals.var_cgdoe_dn12) * assign20970_e26482) + (assign20970_e26479 * (locals.var_vgs_dn12 - locals.var_vds_dn12))), );
            locals.var_qgod_rv = 0.0;
        }
        let assign20980_e26498: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgso_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard364 = assign20980_e26498;
        locals.var_guard364_rv = 0.0;
        if ((locals.var_guard327 == 0.0) && (locals.var_guard364 != 0.0)) {
            let assign20990_e26504: f64 = (-locals.var_cox0);
            let assign20990_e26506: f64 = (assign20990_e26504 * p.p175);
            let assign20990_e26508: f64 = (assign20990_e26506 * locals.var_weffcv_nf);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn4, locals.var_cgsoe_dn5, locals.var_cgsoe_dn6, locals.var_cgsoe_dn8, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, ) = (assign20990_e26508, (assign20990_e26506 * locals.var_weffcv_nf_dn0), (assign20990_e26506 * locals.var_weffcv_nf_dn2), (assign20990_e26506 * locals.var_weffcv_nf_dn4), (assign20990_e26506 * locals.var_weffcv_nf_dn5), (assign20990_e26506 * locals.var_weffcv_nf_dn6), (assign20990_e26506 * locals.var_weffcv_nf_dn8), (assign20990_e26506 * locals.var_weffcv_nf_dn10), (assign20990_e26506 * locals.var_weffcv_nf_dn11), (assign20990_e26506 * locals.var_weffcv_nf_dn12), );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((locals.var_guard327 == 0.0) && (locals.var_guard364 == 0.0)) {
            let assign21000_e26518: f64 = (locals.var_modenml * p.p174);
            let assign21000_e26521: f64 = (locals.var_modervs * p.p173);
            let assign21000_e26522: f64 = (assign21000_e26518 + assign21000_e26521);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn4, locals.var_cgsoe_dn5, locals.var_cgsoe_dn6, locals.var_cgsoe_dn8, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, ) = (assign21000_e26522, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((locals.var_guard327 == 0.0) && (locals.var_guard364 == 0.0)) {
            let assign21010_e26532: f64 = (-locals.var_weffcv_nf);
            let assign21010_e26533: f64 = (locals.var_cgsoe * assign21010_e26532);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn4, locals.var_cgsoe_dn5, locals.var_cgsoe_dn6, locals.var_cgsoe_dn8, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, ) = (assign21010_e26533, ((locals.var_cgsoe_dn0 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn0))), ((locals.var_cgsoe_dn2 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn2))), ((locals.var_cgsoe_dn4 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn4))), ((locals.var_cgsoe_dn5 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn5))), ((locals.var_cgsoe_dn6 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn6))), ((locals.var_cgsoe_dn8 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn8))), ((locals.var_cgsoe_dn10 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn10))), ((locals.var_cgsoe_dn11 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn11))), ((locals.var_cgsoe_dn12 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn12))), );
            locals.var_cgsoe_rv = 0.0;
        }
        if (locals.var_guard327 == 0.0) {
            let assign21020_e26539: f64 = (-locals.var_cgsoe);
            let assign21020_e26541: f64 = (assign21020_e26539 * locals.var_vgs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn8, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, ) = (assign21020_e26541, ((-locals.var_cgsoe_dn0) * locals.var_vgs), ((-locals.var_cgsoe_dn2) * locals.var_vgs), ((-locals.var_cgsoe_dn4) * locals.var_vgs), (((-locals.var_cgsoe_dn5) * locals.var_vgs) + (assign21020_e26539 * locals.var_vgs_dn5)), ((-locals.var_cgsoe_dn6) * locals.var_vgs), ((-locals.var_cgsoe_dn8) * locals.var_vgs), ((-locals.var_cgsoe_dn10) * locals.var_vgs), (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign21020_e26539 * locals.var_vgs_dn11)), (((-locals.var_cgsoe_dn12) * locals.var_vgs) + (assign21020_e26539 * locals.var_vgs_dn12)), );
            locals.var_qgos_rv = 0.0;
        }
        let assign21030_e26546: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign21030_e26546;
        locals.var_guard365_rv = 0.0;
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard365 != 0.0)) {
            let assign21040_e26552: f64 = (p.p223 * p.p224);
            let assign21040_e26554: f64 = (assign21040_e26552 * locals.var_lch);
            let assign21040_e26556: f64 = (assign21040_e26554 * locals.var_lch);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign21040_e26556, (((assign21040_e26552 * locals.var_lch_dn0) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn0)), (((assign21040_e26552 * locals.var_lch_dn2) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn2)), (((assign21040_e26552 * locals.var_lch_dn4) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn4)), (((assign21040_e26552 * locals.var_lch_dn5) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn5)), (((assign21040_e26552 * locals.var_lch_dn6) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn6)), (((assign21040_e26552 * locals.var_lch_dn8) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn8)), (((assign21040_e26552 * locals.var_lch_dn10) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn10)), (((assign21040_e26552 * locals.var_lch_dn11) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn11)), (((assign21040_e26552 * locals.var_lch_dn12) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn12)), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard365 != 0.0)) {
            let assign21050_e26564: f64 = (locals.var_mu * locals.var_vgvt);
            let assign21050_e26566: f64 = (assign21050_e26564 * p.p223);
            let assign21050_e26569: f64 = (p.p224 * locals.var_lch);
            let assign21050_e26571: f64 = (assign21050_e26569 * locals.var_lch);
            let assign21050_e26572: f64 = (assign21050_e26566 + assign21050_e26571);
            let assign21050_e26574: f64 = (assign21050_e26572 + 1e-50);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign21050_e26574, ((((locals.var_mu_dn0 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn0)) * p.p223) + (((p.p224 * locals.var_lch_dn0) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn0))), ((((locals.var_mu_dn2 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn2)) * p.p223) + (((p.p224 * locals.var_lch_dn2) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn2))), ((((locals.var_mu_dn4 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn4)) * p.p223) + (((p.p224 * locals.var_lch_dn4) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn4))), ((((locals.var_mu_dn5 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn5)) * p.p223) + (((p.p224 * locals.var_lch_dn5) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn5))), ((((locals.var_mu_dn6 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn6)) * p.p223) + (((p.p224 * locals.var_lch_dn6) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn6))), ((((locals.var_mu_dn8 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn8)) * p.p223) + (((p.p224 * locals.var_lch_dn8) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn8))), ((((locals.var_mu_dn10 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn10)) * p.p223) + (((p.p224 * locals.var_lch_dn10) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn10))), ((((locals.var_mu_dn11 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn11)) * p.p223) + (((p.p224 * locals.var_lch_dn11) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn11))), ((((locals.var_mu_dn12 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn12)) * p.p223) + (((p.p224 * locals.var_lch_dn12) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn12))), );
            locals.var_t2_rv = 0.0;
        }
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard365 != 0.0)) {
            let assign21060_e26582: f64 = (locals.var_t1 / locals.var_t2);
            (locals.var_tau, locals.var_tau_dn0, locals.var_tau_dn2, locals.var_tau_dn4, locals.var_tau_dn5, locals.var_tau_dn6, locals.var_tau_dn8, locals.var_tau_dn10, locals.var_tau_dn11, locals.var_tau_dn12, ) = (assign21060_e26582, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn12 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn12)) / (locals.var_t2 * locals.var_t2)), );
            locals.var_tau_rv = 0.0;
        }
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard365 == 0.0)) {
            let assign21070_e26591: f64 = (p.p223 + 1e-50);
            (locals.var_tau, locals.var_tau_dn0, locals.var_tau_dn2, locals.var_tau_dn4, locals.var_tau_dn5, locals.var_tau_dn6, locals.var_tau_dn8, locals.var_tau_dn10, locals.var_tau_dn11, locals.var_tau_dn12, ) = (assign21070_e26591, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_tau_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_flg_nqs != 0.0) {
            let assign21080_e26597: f64 = (p.p225 * locals.var_c_fox);
            let assign21080_e26599: f64 = (assign21080_e26597 / 10000.0);
            (locals.var_taub, locals.var_taub_dn0, locals.var_taub_dn2, locals.var_taub_dn4, locals.var_taub_dn5, locals.var_taub_dn6, locals.var_taub_dn8, locals.var_taub_dn10, locals.var_taub_dn11, locals.var_taub_dn12, ) = (assign21080_e26599, ((p.p225 * locals.var_c_fox_dn0) / 10000.0), ((p.p225 * locals.var_c_fox_dn2) / 10000.0), ((p.p225 * locals.var_c_fox_dn4) / 10000.0), ((p.p225 * locals.var_c_fox_dn5) / 10000.0), ((p.p225 * locals.var_c_fox_dn6) / 10000.0), ((p.p225 * locals.var_c_fox_dn8) / 10000.0), ((p.p225 * locals.var_c_fox_dn10) / 10000.0), ((p.p225 * locals.var_c_fox_dn11) / 10000.0), ((p.p225 * locals.var_c_fox_dn12) / 10000.0), );
            locals.var_taub_rv = 0.0;
        }
        let assign21090_e26607: f64 = if ((p.p21 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard366 = assign21090_e26607;
        locals.var_guard366_rv = 0.0;
        if (locals.var_guard366 != 0.0) {
            locals.var_nfalpe = locals.var_mks_nfalp;
            locals.var_nfalpe_rv = 0.0;
            locals.var_cite = locals.var_mks_cit;
            locals.var_cite_rv = 0.0;
        }
        if (locals.var_guard366 != 0.0) {
            let assign21130_e26623: f64 = (locals.var_qn0 / 1.6021918e-19);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign21130_e26623, (locals.var_qn0_dn0 / 1.6021918e-19), (locals.var_qn0_dn2 / 1.6021918e-19), (locals.var_qn0_dn4 / 1.6021918e-19), (locals.var_qn0_dn5 / 1.6021918e-19), (locals.var_qn0_dn6 / 1.6021918e-19), (locals.var_qn0_dn8 / 1.6021918e-19), (locals.var_qn0_dn10 / 1.6021918e-19), (locals.var_qn0_dn11 / 1.6021918e-19), (locals.var_qn0_dn12 / 1.6021918e-19), );
            locals.var_t1_rv = 0.0;
        }
        if (locals.var_guard366 != 0.0) {
            let assign21140_e26631: f64 = (locals.var_ps0 - locals.var_vbs);
            let assign21140_e26632: f64 = (locals.var_qn0 / assign21140_e26631);
            let assign21140_e26633: f64 = (locals.var_c_fox + assign21140_e26632);
            let assign21140_e26635: f64 = (assign21140_e26633 + locals.var_cite);
            let assign21140_e26637: f64 = (assign21140_e26635 * locals.var_beta_inv);
            let assign21140_e26639: f64 = (assign21140_e26637 / 1.6021918e-19);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign21140_e26639, (((locals.var_c_fox_dn0 + (((locals.var_qn0_dn0 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn0 - locals.var_vbs_dn0))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn2 + (((locals.var_qn0_dn2 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn2 - locals.var_vbs_dn2))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), ((((locals.var_c_fox_dn4 + (((locals.var_qn0_dn4 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn4 - locals.var_vbs_dn4))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) + (assign21140_e26635 * locals.var_beta_inv_dn4)) / 1.6021918e-19), (((locals.var_c_fox_dn5 + (((locals.var_qn0_dn5 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn5 - locals.var_vbs_dn5))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn6 + (((locals.var_qn0_dn6 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn6 - locals.var_vbs_dn6))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn8 + (((locals.var_qn0_dn8 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn8 - locals.var_vbs_dn8))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn10 + (((locals.var_qn0_dn10 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn10 - locals.var_vbs_dn10))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn11 + (((locals.var_qn0_dn11 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn11 - locals.var_vbs_dn11))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn12 + (((locals.var_qn0_dn12 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn12 - locals.var_vbs_dn12))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard366 != 0.0) {
            let assign21150_e26644: f64 = (-2.0);
            let assign21150_e26646: f64 = (assign21150_e26644 * locals.var_qi);
            let assign21150_e26648: f64 = (assign21150_e26646 / 1.6021918e-19);
            let assign21150_e26650: f64 = (assign21150_e26648 / locals.var_lch);
            let assign21150_e26652: f64 = (assign21150_e26650 / locals.var_weffcv_nf);
            let assign21150_e26654: f64 = (assign21150_e26652 - locals.var_t1);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, ) = (assign21150_e26654, (((((((((assign21150_e26644 * locals.var_qi_dn0) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn0)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn0), (((((((((assign21150_e26644 * locals.var_qi_dn2) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn2)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn2), (((((((((assign21150_e26644 * locals.var_qi_dn4) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn4)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn4), (((((((((assign21150_e26644 * locals.var_qi_dn5) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn5)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn5), (((((((((assign21150_e26644 * locals.var_qi_dn6) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn6)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn6), (((((((((assign21150_e26644 * locals.var_qi_dn8) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn8)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn8), (((((((((assign21150_e26644 * locals.var_qi_dn10) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn10)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn10), (((((((((assign21150_e26644 * locals.var_qi_dn11) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn11)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn11), (((((((((assign21150_e26644 * locals.var_qi_dn12) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn12)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn12), );
            locals.var_t3_rv = 0.0;
        }
        let assign21160_e26659: f64 = (locals.var_t3 - locals.var_t1);
        let assign21160_e26660: f64 = (assign21160_e26659).abs();
        let assign21160_e26663: f64 = (10.0 * 2.220446049250313e-16);
        let assign21160_e26664: f64 = if assign21160_e26660 > assign21160_e26663 { 1.0 } else { 0.0 };
        locals.var_guard367 = assign21160_e26664;
        locals.var_guard367_rv = 0.0;
        if ((locals.var_guard366 != 0.0) && (locals.var_guard367 != 0.0)) {
            let assign21170_e26671: f64 = (locals.var_t1 + locals.var_t2);
            let assign21170_e26672: f64 = (1.0 / assign21170_e26671);
            let assign21170_e26675: f64 = (locals.var_t3 + locals.var_t2);
            let assign21170_e26676: f64 = (assign21170_e26672 / assign21170_e26675);
            let assign21170_e26679: f64 = (2.0 * locals.var_nfalpe);
            let assign21170_e26681: f64 = (assign21170_e26679 * locals.var_ey);
            let assign21170_e26683: f64 = (assign21170_e26681 * locals.var_mu);
            let assign21170_e26686: f64 = (locals.var_t3 - locals.var_t1);
            let assign21170_e26687: f64 = (assign21170_e26683 / assign21170_e26686);
            let assign21170_e26690: f64 = (locals.var_t3 + locals.var_t2);
            let assign21170_e26693: f64 = (locals.var_t1 + locals.var_t2);
            let assign21170_e26694: f64 = (assign21170_e26690 / assign21170_e26693);
            let assign21170_e26695: f64 = (assign21170_e26694).ln();
            let assign21170_e26696: f64 = (assign21170_e26687 * assign21170_e26695);
            let assign21170_e26697: f64 = (assign21170_e26676 + assign21170_e26696);
            let assign21170_e26700: f64 = (locals.var_nfalpe * locals.var_ey);
            let assign21170_e26702: f64 = (assign21170_e26700 * locals.var_mu);
            let assign21170_e26704: f64 = (assign21170_e26702 * locals.var_nfalpe);
            let assign21170_e26706: f64 = (assign21170_e26704 * locals.var_ey);
            let assign21170_e26708: f64 = (assign21170_e26706 * locals.var_mu);
            let assign21170_e26709: f64 = (assign21170_e26697 + assign21170_e26708);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, ) = (assign21170_e26709, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn0) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn0)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn0 - locals.var_t1_dn0))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn0 + locals.var_t2_dn0) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn0)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn2) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn2)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn2 - locals.var_t1_dn2))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn2 + locals.var_t2_dn2) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn2)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn4) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn4)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn4 - locals.var_t1_dn4))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn4 + locals.var_t2_dn4) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn4)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn5) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn5)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn5 - locals.var_t1_dn5))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn5 + locals.var_t2_dn5) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn5)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn6) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn6)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn6 - locals.var_t1_dn6))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn6 + locals.var_t2_dn6) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn6)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn8) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn8)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn8 - locals.var_t1_dn8))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn8 + locals.var_t2_dn8) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn8)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn10) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn10)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn10 - locals.var_t1_dn10))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn10 + locals.var_t2_dn10) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn10)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn11) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn11)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn11 - locals.var_t1_dn11))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn11 + locals.var_t2_dn11) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn11)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn12 + locals.var_t2_dn12) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn12 + locals.var_t2_dn12))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn12) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn12)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn12 - locals.var_t1_dn12))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn12 + locals.var_t2_dn12) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn12 + locals.var_t2_dn12))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn12) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn12)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn12)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn12))), );
            locals.var_t4_rv = 0.0;
        }
        if ((locals.var_guard366 != 0.0) && (locals.var_guard367 == 0.0)) {
            let assign21180_e26719: f64 = (locals.var_t1 + locals.var_t2);
            let assign21180_e26720: f64 = (1.0 / assign21180_e26719);
            let assign21180_e26723: f64 = (locals.var_t3 + locals.var_t2);
            let assign21180_e26724: f64 = (assign21180_e26720 / assign21180_e26723);
            let assign21180_e26727: f64 = (2.0 * locals.var_nfalpe);
            let assign21180_e26729: f64 = (assign21180_e26727 * locals.var_ey);
            let assign21180_e26731: f64 = (assign21180_e26729 * locals.var_mu);
            let assign21180_e26734: f64 = (locals.var_t1 + locals.var_t2);
            let assign21180_e26735: f64 = (assign21180_e26731 / assign21180_e26734);
            let assign21180_e26736: f64 = (assign21180_e26724 + assign21180_e26735);
            let assign21180_e26739: f64 = (locals.var_nfalpe * locals.var_ey);
            let assign21180_e26741: f64 = (assign21180_e26739 * locals.var_mu);
            let assign21180_e26743: f64 = (assign21180_e26741 * locals.var_nfalpe);
            let assign21180_e26745: f64 = (assign21180_e26743 * locals.var_ey);
            let assign21180_e26747: f64 = (assign21180_e26745 * locals.var_mu);
            let assign21180_e26748: f64 = (assign21180_e26736 + assign21180_e26747);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, ) = (assign21180_e26748, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn0) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn0)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn0)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn2) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn2)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn2)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn4) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn4)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn4)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn5) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn5)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn5)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn6) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn6)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn6)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn8) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn8)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn8)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn10) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn10)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn10)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn11) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn11)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn11)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn12 + locals.var_t2_dn12) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn12 + locals.var_t2_dn12))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn12) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn12)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn12 + locals.var_t2_dn12))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn12) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn12)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn12)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn12))), );
            locals.var_t4_rv = 0.0;
        }
        let assign21210_e26777: f64 = if ((p.p23 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard368 = assign21210_e26777;
        locals.var_guard368_rv = 0.0;
        if (locals.var_guard368 != 0.0) {
            let assign21220_e26781: f64 = (locals.var_psdl - locals.var_ps0);
            let assign21220_e26783: f64 = (assign21220_e26781 / locals.var_lch);
            (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn8, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn12, ) = (assign21220_e26783, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn4 - locals.var_ps0_dn4) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn5 - locals.var_ps0_dn5) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn8 - locals.var_ps0_dn8) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn12 - locals.var_ps0_dn12) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)), );
            locals.var_eyd_rv = 0.0;
        }
        if (locals.var_guard368 != 0.0) {
            let assign21230_e26789: f64 = (locals.var_muun * locals.var_eyd);
            let assign21230_e26792: f64 = (10000000.0 * 0.01);
            let assign21230_e26793: f64 = (assign21230_e26789 / assign21230_e26792);
            (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn8, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12, ) = (assign21230_e26793, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / assign21230_e26792), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / assign21230_e26792), (((locals.var_muun_dn4 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn4)) / assign21230_e26792), (((locals.var_muun_dn5 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn5)) / assign21230_e26792), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / assign21230_e26792), (((locals.var_muun_dn8 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn8)) / assign21230_e26792), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / assign21230_e26792), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / assign21230_e26792), (((locals.var_muun_dn12 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn12)) / assign21230_e26792), );
            locals.var_t12_rv = 0.0;
        }
        let assign21240_e26799: f64 = (10.0 * 2.220446049250313e-16);
        let assign21240_e26800: f64 = (1.0 - assign21240_e26799);
        let assign21240_e26807: f64 = (10.0 * 2.220446049250313e-16);
        let assign21240_e26808: f64 = (1.0 + assign21240_e26807);
        let assign21240_e26810: f64 = if ((assign21240_e26800 <= p.p114) && (p.p114 <= assign21240_e26808)) { 1.0 } else { 0.0 };
        locals.var_guard369 = assign21240_e26810;
        locals.var_guard369_rv = 0.0;
        if ((locals.var_guard368 != 0.0) && (locals.var_guard369 != 0.0)) {
            (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t7_rv = 0.0;
        }
        let assign21260_e26820: f64 = (10.0 * 2.220446049250313e-16);
        let assign21260_e26821: f64 = (2.0 - assign21260_e26820);
        let assign21260_e26828: f64 = (10.0 * 2.220446049250313e-16);
        let assign21260_e26829: f64 = (2.0 + assign21260_e26828);
        let assign21260_e26831: f64 = if ((assign21260_e26821 <= p.p114) && (p.p114 <= assign21260_e26829)) { 1.0 } else { 0.0 };
        locals.var_guard370 = assign21260_e26831;
        locals.var_guard370_rv = 0.0;
        if (((locals.var_guard368 != 0.0) && (locals.var_guard369 == 0.0)) && (locals.var_guard370 != 0.0)) {
            (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, ) = (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn8, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12, );
            locals.var_t7_rv = 0.0;
        }
        if (((locals.var_guard368 != 0.0) && (locals.var_guard369 == 0.0)) && (locals.var_guard370 == 0.0)) {
            let assign21280_e26851: f64 = (p.p114 - 1.0);
            let assign21280_e26852: f64 = (locals.var_t12).powf(assign21280_e26851);
            (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, ) = (assign21280_e26852, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn0)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn0 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn2)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn2 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn4)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn4 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn5)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn5 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn6)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn6 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn8)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn8 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn10)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn10 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn11)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn11 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn12)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn12 / locals.var_t12))) }, );
            locals.var_t7_rv = 0.0;
        }
        if (locals.var_guard368 != 0.0) {
            let assign21290_e26859: f64 = (locals.var_t12 * locals.var_t7);
            let assign21290_e26860: f64 = (1.0 + assign21290_e26859);
            (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12, ) = (assign21290_e26860, ((locals.var_t12_dn0 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn0)), ((locals.var_t12_dn2 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn2)), ((locals.var_t12_dn4 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn4)), ((locals.var_t12_dn5 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn5)), ((locals.var_t12_dn6 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn6)), ((locals.var_t12_dn8 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn8)), ((locals.var_t12_dn10 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn10)), ((locals.var_t12_dn11 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn11)), ((locals.var_t12_dn12 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn12)), );
            locals.var_t9_rv = 0.0;
        }
        if (locals.var_guard368 != 0.0) {
            let assign21300_e26866: f64 = (-1.0);
            let assign21300_e26868: f64 = (assign21300_e26866 / p.p114);
            let assign21300_e26870: f64 = (assign21300_e26868 - 1.0);
            let assign21300_e26871: f64 = (locals.var_t9).powf(assign21300_e26870);
            (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, ) = (assign21300_e26871, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn0)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn0 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn2)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn2 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn4)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn4 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn5)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn5 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn6)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn6 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn8)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn8 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn10)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn10 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn11)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn11 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn12)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn12 / locals.var_t9))) }, );
            locals.var_t10_rv = 0.0;
        }
        if (locals.var_guard368 != 0.0) {
            let assign21310_e26877: f64 = (locals.var_muun * locals.var_t9);
            let assign21310_e26879: f64 = (assign21310_e26877 * locals.var_t10);
            (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn4, locals.var_mud_hoso_dn5, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn8, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn12, ) = (assign21310_e26879, ((((locals.var_muun_dn0 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn0)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn0)), ((((locals.var_muun_dn2 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn2)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn2)), ((((locals.var_muun_dn4 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn4)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn4)), ((((locals.var_muun_dn5 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn5)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn5)), ((((locals.var_muun_dn6 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn6)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn6)), ((((locals.var_muun_dn8 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn8)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn8)), ((((locals.var_muun_dn10 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn10)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn10)), ((((locals.var_muun_dn11 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn11)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn11)), ((((locals.var_muun_dn12 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn12)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn12)), );
            locals.var_mud_hoso_rv = 0.0;
        }
        if (locals.var_guard368 != 0.0) {
            let assign21320_e26885: f64 = (locals.var_mu + locals.var_mud_hoso);
            let assign21320_e26887: f64 = (assign21320_e26885 / 2.0);
            (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn4, locals.var_mu_ave_dn5, locals.var_mu_ave_dn6, locals.var_mu_ave_dn8, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn12, ) = (assign21320_e26887, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn4 + locals.var_mud_hoso_dn4) / 2.0), ((locals.var_mu_dn5 + locals.var_mud_hoso_dn5) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn8 + locals.var_mud_hoso_dn8) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn12 + locals.var_mud_hoso_dn12) / 2.0), );
            locals.var_mu_ave_rv = 0.0;
        }
        if (locals.var_guard368 != 0.0) {
            let assign21330_e26893: f64 = (locals.var_alpha * locals.var_alpha);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, ) = (assign21330_e26893, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn4 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn4)), ((locals.var_alpha_dn5 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn5)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn8 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn8)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn12)), );
            locals.var_t0_rv = 0.0;
        }
        if (locals.var_guard368 != 0.0) {
            let assign21340_e26899: f64 = (locals.var_weff_nf * locals.var_c_fox);
            let assign21340_e26901: f64 = (assign21340_e26899 * locals.var_vgvt);
            let assign21340_e26903: f64 = (assign21340_e26901 * locals.var_mu);
            let assign21340_e26907: f64 = (3.0 * locals.var_alpha);
            let assign21340_e26908: f64 = (1.0 + assign21340_e26907);
            let assign21340_e26911: f64 = (6.0 * locals.var_t0);
            let assign21340_e26912: f64 = (assign21340_e26908 + assign21340_e26911);
            let assign21340_e26914: f64 = (assign21340_e26912 * locals.var_mud_hoso);
            let assign21340_e26916: f64 = (assign21340_e26914 * locals.var_mud_hoso);
            let assign21340_e26920: f64 = (4.0 * locals.var_alpha);
            let assign21340_e26921: f64 = (3.0 + assign21340_e26920);
            let assign21340_e26924: f64 = (3.0 * locals.var_t0);
            let assign21340_e26925: f64 = (assign21340_e26921 + assign21340_e26924);
            let assign21340_e26927: f64 = (assign21340_e26925 * locals.var_mud_hoso);
            let assign21340_e26929: f64 = (assign21340_e26927 * locals.var_mu);
            let assign21340_e26930: f64 = (assign21340_e26916 + assign21340_e26929);
            let assign21340_e26934: f64 = (3.0 * locals.var_alpha);
            let assign21340_e26935: f64 = (6.0 + assign21340_e26934);
            let assign21340_e26937: f64 = (assign21340_e26935 + locals.var_t0);
            let assign21340_e26939: f64 = (assign21340_e26937 * locals.var_mu);
            let assign21340_e26941: f64 = (assign21340_e26939 * locals.var_mu);
            let assign21340_e26942: f64 = (assign21340_e26930 + assign21340_e26941);
            let assign21340_e26943: f64 = (assign21340_e26903 * assign21340_e26942);
            let assign21340_e26946: f64 = (15.0 * locals.var_lch);
            let assign21340_e26949: f64 = (1.0 + locals.var_alpha);
            let assign21340_e26950: f64 = (assign21340_e26946 * assign21340_e26949);
            let assign21340_e26952: f64 = (assign21340_e26950 * locals.var_mu_ave);
            let assign21340_e26954: f64 = (assign21340_e26952 * locals.var_mu_ave);
            let assign21340_e26955: f64 = (assign21340_e26943 / assign21340_e26954);
            (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn8, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, ) = (assign21340_e26955, (((((((((((locals.var_weff_nf_dn0 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn0)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn0)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0_dn0) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn0)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn0))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn0) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn0)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn2 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn2)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn2)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0_dn2) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn2)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn2))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn2) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn2)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn4 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn4)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn4)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn4)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn4) + (6.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn4)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn4)) + ((((((4.0 * locals.var_alpha_dn4) + (3.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn4)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn4))) + ((((((3.0 * locals.var_alpha_dn4) + locals.var_t0_dn4) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn4)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn4))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn4) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn4)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn4)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn4)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn5 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn5)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn5)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn5)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn5) + (6.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn5)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn5)) + ((((((4.0 * locals.var_alpha_dn5) + (3.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn5)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn5))) + ((((((3.0 * locals.var_alpha_dn5) + locals.var_t0_dn5) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn5)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn5))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn5) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn5)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn5)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn5)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn6 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn6)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn6)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0_dn6) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn6)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn6))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn6) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn6)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn8 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn8)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn8)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn8)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn8) + (6.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn8)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn8)) + ((((((4.0 * locals.var_alpha_dn8) + (3.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn8)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn8))) + ((((((3.0 * locals.var_alpha_dn8) + locals.var_t0_dn8) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn8)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn8))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn8) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn8)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn8)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn8)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn10 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn10)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn10)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0_dn10) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn10)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn10))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn10) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn10)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn11 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn11)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn11)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0_dn11) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn11)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn11))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn11) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn11)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn12 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn12)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn12)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn12)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn12) + (6.0 * locals.var_t0_dn12)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn12)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn12)) + ((((((4.0 * locals.var_alpha_dn12) + (3.0 * locals.var_t0_dn12)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn12)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn12))) + ((((((3.0 * locals.var_alpha_dn12) + locals.var_t0_dn12) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn12)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn12))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn12) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn12)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn12)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn12)))) / (assign21340_e26954 * assign21340_e26954)), );
            locals.var_nthrml_rv = 0.0;
        }
        if (locals.var_guard368 == 0.0) {
            (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn8, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_nthrml_rv = 0.0;
        }
        let assign21360_e26976: f64 = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard371 = assign21360_e26976;
        locals.var_guard371_rv = 0.0;
        if (locals.var_guard371 != 0.0) {
            let assign21370_e26979: f64 = (locals.var_kusail).sqrt();
            (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn4, locals.var_sqrtkusail_dn5, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn8, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn12, ) = (assign21370_e26979, (locals.var_kusail_dn0 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn2 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn4 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn5 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn6 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn8 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn10 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn11 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn12 / (2.0 * assign21370_e26979)), );
            locals.var_sqrtkusail_rv = 0.0;
        }
        if (locals.var_guard371 != 0.0) {
            let assign21380_e26985: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign21380_e26985, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4), (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard371 != 0.0) {
            let assign21390_e26991: f64 = (locals.var_kusai00 * locals.var_kusai00);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, ) = (assign21390_e26991, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)), ((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)), );
            locals.var_t3_rv = 0.0;
        }
        if (locals.var_guard371 != 0.0) {
            let assign21400_e26997: f64 = (locals.var_kusail * locals.var_kusail);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, ) = (assign21400_e26997, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)), ((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)), );
            locals.var_t4_rv = 0.0;
        }
        if (locals.var_guard371 != 0.0) {
            let assign21410_e27003: f64 = (42.0 * locals.var_kusai00);
            let assign21410_e27005: f64 = (assign21410_e27003 * locals.var_kusail);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, ) = (assign21410_e27005, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn4) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn4)), (((42.0 * locals.var_kusai00_dn5) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn5)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn8) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn8)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn12) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn12)), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard371 != 0.0) {
            let assign21420_e27013: f64 = (locals.var_t3 + locals.var_t4);
            let assign21420_e27014: f64 = (4.0 * assign21420_e27013);
            let assign21420_e27015: f64 = (locals.var_t5 + assign21420_e27014);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, ) = (assign21420_e27015, (locals.var_t5_dn0 + (4.0 * (locals.var_t3_dn0 + locals.var_t4_dn0))), (locals.var_t5_dn2 + (4.0 * (locals.var_t3_dn2 + locals.var_t4_dn2))), (locals.var_t5_dn4 + (4.0 * (locals.var_t3_dn4 + locals.var_t4_dn4))), (locals.var_t5_dn5 + (4.0 * (locals.var_t3_dn5 + locals.var_t4_dn5))), (locals.var_t5_dn6 + (4.0 * (locals.var_t3_dn6 + locals.var_t4_dn6))), (locals.var_t5_dn8 + (4.0 * (locals.var_t3_dn8 + locals.var_t4_dn8))), (locals.var_t5_dn10 + (4.0 * (locals.var_t3_dn10 + locals.var_t4_dn10))), (locals.var_t5_dn11 + (4.0 * (locals.var_t3_dn11 + locals.var_t4_dn11))), (locals.var_t5_dn12 + (4.0 * (locals.var_t3_dn12 + locals.var_t4_dn12))), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard371 != 0.0) {
            let assign21430_e27022: f64 = (20.0 * locals.var_sqrtkusail);
            let assign21430_e27024: f64 = (assign21430_e27022 * locals.var_vgvt);
            let assign21430_e27027: f64 = (locals.var_kusai00 + locals.var_kusail);
            let assign21430_e27028: f64 = (assign21430_e27024 * assign21430_e27027);
            let assign21430_e27029: f64 = (locals.var_t5 + assign21430_e27028);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, ) = (assign21430_e27029, (locals.var_t5_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn0)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn2)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5_dn4 + (((((20.0 * locals.var_sqrtkusail_dn4) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn4)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn4 + locals.var_kusail_dn4)))), (locals.var_t5_dn5 + (((((20.0 * locals.var_sqrtkusail_dn5) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn5)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn5 + locals.var_kusail_dn5)))), (locals.var_t5_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn6)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5_dn8 + (((((20.0 * locals.var_sqrtkusail_dn8) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn8)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn8 + locals.var_kusail_dn8)))), (locals.var_t5_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn10)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn11)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5_dn12 + (((((20.0 * locals.var_sqrtkusail_dn12) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn12)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn12 + locals.var_kusail_dn12)))), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard371 != 0.0) {
            let assign21440_e27035: f64 = (locals.var_t2 * locals.var_t2);
            (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, ) = (assign21440_e27035, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), );
            locals.var_t10_rv = 0.0;
        }
        if (locals.var_guard371 != 0.0) {
            let assign21450_e27042: f64 = (locals.var_t10 * locals.var_t10);
            let assign21450_e27044: f64 = (assign21450_e27042 * locals.var_t2);
            let assign21450_e27045: f64 = (locals.var_t5 / assign21450_e27044);
            (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn4, locals.var_kusai_ig_dn5, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn8, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn12, ) = (assign21450_e27045, (((locals.var_t5_dn0 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn0)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn2 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn2)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn4 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn4)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn5 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn5)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn6 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn6)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn8 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn8)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn10 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn10)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn11 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn11)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn12 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn12 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn12)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn12)))) / (assign21450_e27044 * assign21450_e27044)), );
            locals.var_kusai_ig_rv = 0.0;
        }
        if (locals.var_guard371 != 0.0) {
            let assign21460_e27051: f64 = (locals.var_weff_nf / locals.var_lch);
            let assign21460_e27053: f64 = (assign21460_e27051 * locals.var_mu);
            let assign21460_e27055: f64 = (assign21460_e27053 * locals.var_c_fox);
            (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn4, locals.var_gds0_ign_dn5, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn8, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn12, ) = (assign21460_e27055, (((((((locals.var_weff_nf_dn0 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn0)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn0)), (((((((locals.var_weff_nf_dn2 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn2)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn2)), (((((((locals.var_weff_nf_dn4 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn4)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn4)), (((((((locals.var_weff_nf_dn5 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn5)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn5)), (((((((locals.var_weff_nf_dn6 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn6)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn6)), (((((((locals.var_weff_nf_dn8 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn8)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn8)), (((((((locals.var_weff_nf_dn10 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn10)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn10)), (((((((locals.var_weff_nf_dn11 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn11)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn11)), (((((((locals.var_weff_nf_dn12 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn12)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn12)), );
            locals.var_gds0_ign_rv = 0.0;
        }
        if (locals.var_guard371 != 0.0) {
            let assign21490_e27074: f64 = (4.0 * locals.var_vgvt);
            let assign21490_e27076: f64 = (assign21490_e27074 * locals.var_sqrtkusail);
            let assign21490_e27077: f64 = (locals.var_kusai00 + assign21490_e27076);
            let assign21490_e27079: f64 = (assign21490_e27077 + locals.var_kusail);
            (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, ) = (assign21490_e27079, ((locals.var_kusai00_dn0 + (((4.0 * locals.var_vgvt_dn0) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0), ((locals.var_kusai00_dn2 + (((4.0 * locals.var_vgvt_dn2) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2), ((locals.var_kusai00_dn4 + (((4.0 * locals.var_vgvt_dn4) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4), ((locals.var_kusai00_dn5 + (((4.0 * locals.var_vgvt_dn5) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5), ((locals.var_kusai00_dn6 + (((4.0 * locals.var_vgvt_dn6) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6), ((locals.var_kusai00_dn8 + (((4.0 * locals.var_vgvt_dn8) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8), ((locals.var_kusai00_dn10 + (((4.0 * locals.var_vgvt_dn10) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10), ((locals.var_kusai00_dn11 + (((4.0 * locals.var_vgvt_dn11) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11), ((locals.var_kusai00_dn12 + (((4.0 * locals.var_vgvt_dn12) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12), );
            locals.var_t7_rv = 0.0;
        }
        let assign21510_e27105: f64 = (locals.var_ids + locals.var_idsibpc);
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn8, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, ) = (assign21510_e27105, (locals.var_ids_dn0 + locals.var_idsibpc_dn0), (locals.var_ids_dn2 + locals.var_idsibpc_dn2), (locals.var_ids_dn4 + locals.var_idsibpc_dn4), (locals.var_ids_dn5 + locals.var_idsibpc_dn5), (locals.var_ids_dn6 + locals.var_idsibpc_dn6), (locals.var_ids_dn8 + locals.var_idsibpc_dn8), (locals.var_ids_dn10 + locals.var_idsibpc_dn10), (locals.var_ids_dn11 + locals.var_idsibpc_dn11), (locals.var_ids_dn12 + locals.var_idsibpc_dn12), );
        locals.var_ids_rv = 0.0;
        if (locals.var_cgbo_given != 0.0) {
            let assign21520_e27108: f64 = (-p.p172);
            let assign21520_e27110: f64 = (assign21520_e27108 * locals.var_lgate);
            locals.var_cgbe = assign21520_e27110;
            locals.var_cgbe_rv = 0.0;
        }
        if (locals.var_cgbo_given != 0.0) {
            let assign21530_e27117: f64 = (locals.var_vgse - locals.var_vbse);
            let assign21530_e27118: f64 = (locals.var_cgbe * assign21530_e27117);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn5, locals.var_qgob_dn6, ) = (assign21530_e27118, (locals.var_cgbe * (locals.var_vgse_dn0 - locals.var_vbse_dn0)), (locals.var_cgbe * (locals.var_vgse_dn2 - locals.var_vbse_dn2)), (locals.var_cgbe * locals.var_vgse_dn5), (locals.var_cgbe * (-locals.var_vbse_dn6)), );
            locals.var_qgob_rv = 0.0;
        }
        if (locals.var_cgbo_given == 0.0) {
            locals.var_cgbe = 0.0;
            locals.var_cgbe_rv = 0.0;
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn5, locals.var_qgob_dn6, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qgob_rv = 0.0;
        }
        (locals.var_cf, locals.var_cf_dn0, locals.var_cf_dn2, locals.var_cf_dn4, locals.var_cf_dn5, locals.var_cf_dn6, locals.var_cf_dn8, locals.var_cf_dn10, locals.var_cf_dn11, locals.var_cf_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cf_rv = 0.0;
        let assign21570_e27143: f64 = (locals.var_vgse - locals.var_vdse);
        let assign21570_e27144: f64 = (locals.var_cf * assign21570_e27143);
        (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn4, locals.var_qfd_dn5, locals.var_qfd_dn6, locals.var_qfd_dn8, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, ) = (assign21570_e27144, ((locals.var_cf_dn0 * assign21570_e27143) + (locals.var_cf * (locals.var_vgse_dn0 - locals.var_vdse_dn0))), ((locals.var_cf_dn2 * assign21570_e27143) + (locals.var_cf * (locals.var_vgse_dn2 - locals.var_vdse_dn2))), (locals.var_cf_dn4 * assign21570_e27143), ((locals.var_cf_dn5 * assign21570_e27143) + (locals.var_cf * locals.var_vgse_dn5)), (locals.var_cf_dn6 * assign21570_e27143), (locals.var_cf_dn8 * assign21570_e27143), (locals.var_cf_dn10 * assign21570_e27143), (locals.var_cf_dn11 * assign21570_e27143), (locals.var_cf_dn12 * assign21570_e27143), );
        locals.var_qfd_rv = 0.0;
        let assign21580_e27147: f64 = (locals.var_cf * locals.var_vgse);
        (locals.var_qfs, locals.var_qfs_dn0, locals.var_qfs_dn2, locals.var_qfs_dn4, locals.var_qfs_dn5, locals.var_qfs_dn6, locals.var_qfs_dn8, locals.var_qfs_dn10, locals.var_qfs_dn11, locals.var_qfs_dn12, ) = (assign21580_e27147, ((locals.var_cf_dn0 * locals.var_vgse) + (locals.var_cf * locals.var_vgse_dn0)), ((locals.var_cf_dn2 * locals.var_vgse) + (locals.var_cf * locals.var_vgse_dn2)), (locals.var_cf_dn4 * locals.var_vgse), ((locals.var_cf_dn5 * locals.var_vgse) + (locals.var_cf * locals.var_vgse_dn5)), (locals.var_cf_dn6 * locals.var_vgse), (locals.var_cf_dn8 * locals.var_vgse), (locals.var_cf_dn10 * locals.var_vgse), (locals.var_cf_dn11 * locals.var_vgse), (locals.var_cf_dn12 * locals.var_vgse), );
        locals.var_qfs_rv = 0.0;
        let assign21590_e27150: f64 = (locals.var_qgod + locals.var_qfd);
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn8, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, ) = (assign21590_e27150, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn4 + locals.var_qfd_dn4), (locals.var_qgod_dn5 + locals.var_qfd_dn5), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn8 + locals.var_qfd_dn8), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), );
        locals.var_qgod_rv = 0.0;
        let assign21600_e27153: f64 = (locals.var_qgos + locals.var_qfs);
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn8, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, ) = (assign21600_e27153, (locals.var_qgos_dn0 + locals.var_qfs_dn0), (locals.var_qgos_dn2 + locals.var_qfs_dn2), (locals.var_qgos_dn4 + locals.var_qfs_dn4), (locals.var_qgos_dn5 + locals.var_qfs_dn5), (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn8 + locals.var_qfs_dn8), (locals.var_qgos_dn10 + locals.var_qfs_dn10), (locals.var_qgos_dn11 + locals.var_qfs_dn11), (locals.var_qgos_dn12 + locals.var_qfs_dn12), );
        locals.var_qgos_rv = 0.0;
        let assign21610_e27156: f64 = (locals.var_mfactor * locals.var_ids);
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn8, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, ) = (assign21610_e27156, (locals.var_mfactor * locals.var_ids_dn0), (locals.var_mfactor * locals.var_ids_dn2), (locals.var_mfactor * locals.var_ids_dn4), (locals.var_mfactor * locals.var_ids_dn5), (locals.var_mfactor * locals.var_ids_dn6), (locals.var_mfactor * locals.var_ids_dn8), (locals.var_mfactor * locals.var_ids_dn10), (locals.var_mfactor * locals.var_ids_dn11), (locals.var_mfactor * locals.var_ids_dn12), );
        locals.var_idse_rv = 0.0;
        let assign21620_e27158: f64 = (-locals.var_weffcv_nf);
        let assign21620_e27160: f64 = (assign21620_e27158 * locals.var_leff);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign21620_e27160, (((-locals.var_weffcv_nf_dn0) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn0)), (((-locals.var_weffcv_nf_dn2) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn2)), (((-locals.var_weffcv_nf_dn4) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn4)), (((-locals.var_weffcv_nf_dn5) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn5)), (((-locals.var_weffcv_nf_dn6) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn6)), (((-locals.var_weffcv_nf_dn8) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn8)), (((-locals.var_weffcv_nf_dn10) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn10)), (((-locals.var_weffcv_nf_dn11) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn11)), (((-locals.var_weffcv_nf_dn12) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn12)), );
        locals.var_t1_rv = 0.0;
        let assign21630_e27162: f64 = (-0.5);
        let assign21630_e27165: f64 = (locals.var_q_s0_dep + locals.var_q_sl_dep);
        let assign21630_e27166: f64 = (assign21630_e27162 * assign21630_e27165);
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign21630_e27166, (assign21630_e27162 * (locals.var_q_s0_dep_dn0 + locals.var_q_sl_dep_dn0)), (assign21630_e27162 * (locals.var_q_s0_dep_dn2 + locals.var_q_sl_dep_dn2)), (assign21630_e27162 * (locals.var_q_s0_dep_dn4 + locals.var_q_sl_dep_dn4)), (assign21630_e27162 * (locals.var_q_s0_dep_dn5 + locals.var_q_sl_dep_dn5)), (assign21630_e27162 * (locals.var_q_s0_dep_dn6 + locals.var_q_sl_dep_dn6)), (assign21630_e27162 * (locals.var_q_s0_dep_dn8 + locals.var_q_sl_dep_dn8)), (assign21630_e27162 * (locals.var_q_s0_dep_dn10 + locals.var_q_sl_dep_dn10)), (assign21630_e27162 * (locals.var_q_s0_dep_dn11 + locals.var_q_sl_dep_dn11)), (assign21630_e27162 * (locals.var_q_s0_dep_dn12 + locals.var_q_sl_dep_dn12)), );
        locals.var_t2_rv = 0.0;
        let assign21640_e27168: f64 = (-0.5);
        let assign21640_e27171: f64 = (locals.var_q_b0_dep + locals.var_q_bl_dep);
        let assign21640_e27172: f64 = (assign21640_e27168 * assign21640_e27171);
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, ) = (assign21640_e27172, (assign21640_e27168 * (locals.var_q_b0_dep_dn0 + locals.var_q_bl_dep_dn0)), (assign21640_e27168 * (locals.var_q_b0_dep_dn2 + locals.var_q_bl_dep_dn2)), (assign21640_e27168 * (locals.var_q_b0_dep_dn4 + locals.var_q_bl_dep_dn4)), (assign21640_e27168 * (locals.var_q_b0_dep_dn5 + locals.var_q_bl_dep_dn5)), (assign21640_e27168 * (locals.var_q_b0_dep_dn6 + locals.var_q_bl_dep_dn6)), (assign21640_e27168 * (locals.var_q_b0_dep_dn8 + locals.var_q_bl_dep_dn8)), (assign21640_e27168 * (locals.var_q_b0_dep_dn10 + locals.var_q_bl_dep_dn10)), (assign21640_e27168 * (locals.var_q_b0_dep_dn11 + locals.var_q_bl_dep_dn11)), (assign21640_e27168 * (locals.var_q_b0_dep_dn12 + locals.var_q_bl_dep_dn12)), );
        locals.var_t3_rv = 0.0;
        let assign21650_e27176: f64 = (0.1 * locals.var_c_box);
        let assign21650_e27177: f64 = (locals.var_t1 * assign21650_e27176);
        let assign21650_e27179: f64 = (assign21650_e27177 * locals.var_vbse);
        (locals.var_qfs_box, locals.var_qfs_box_dn0, locals.var_qfs_box_dn2, locals.var_qfs_box_dn4, locals.var_qfs_box_dn5, locals.var_qfs_box_dn6, locals.var_qfs_box_dn8, locals.var_qfs_box_dn10, locals.var_qfs_box_dn11, locals.var_qfs_box_dn12, ) = (assign21650_e27179, (((locals.var_t1_dn0 * assign21650_e27176) * locals.var_vbse) + (assign21650_e27177 * locals.var_vbse_dn0)), (((locals.var_t1_dn2 * assign21650_e27176) * locals.var_vbse) + (assign21650_e27177 * locals.var_vbse_dn2)), ((locals.var_t1_dn4 * assign21650_e27176) * locals.var_vbse), ((locals.var_t1_dn5 * assign21650_e27176) * locals.var_vbse), (((locals.var_t1_dn6 * assign21650_e27176) * locals.var_vbse) + (assign21650_e27177 * locals.var_vbse_dn6)), ((locals.var_t1_dn8 * assign21650_e27176) * locals.var_vbse), ((locals.var_t1_dn10 * assign21650_e27176) * locals.var_vbse), ((locals.var_t1_dn11 * assign21650_e27176) * locals.var_vbse), ((locals.var_t1_dn12 * assign21650_e27176) * locals.var_vbse), );
        locals.var_qfs_box_rv = 0.0;
        let assign21660_e27183: f64 = (0.1 * locals.var_c_box);
        let assign21660_e27184: f64 = (locals.var_t1 * assign21660_e27183);
        let assign21660_e27187: f64 = (locals.var_vbse - locals.var_vdse);
        let assign21660_e27188: f64 = (assign21660_e27184 * assign21660_e27187);
        (locals.var_qfd_box, locals.var_qfd_box_dn0, locals.var_qfd_box_dn2, locals.var_qfd_box_dn4, locals.var_qfd_box_dn5, locals.var_qfd_box_dn6, locals.var_qfd_box_dn8, locals.var_qfd_box_dn10, locals.var_qfd_box_dn11, locals.var_qfd_box_dn12, ) = (assign21660_e27188, (((locals.var_t1_dn0 * assign21660_e27183) * assign21660_e27187) + (assign21660_e27184 * (locals.var_vbse_dn0 - locals.var_vdse_dn0))), (((locals.var_t1_dn2 * assign21660_e27183) * assign21660_e27187) + (assign21660_e27184 * (locals.var_vbse_dn2 - locals.var_vdse_dn2))), ((locals.var_t1_dn4 * assign21660_e27183) * assign21660_e27187), ((locals.var_t1_dn5 * assign21660_e27183) * assign21660_e27187), (((locals.var_t1_dn6 * assign21660_e27183) * assign21660_e27187) + (assign21660_e27184 * locals.var_vbse_dn6)), ((locals.var_t1_dn8 * assign21660_e27183) * assign21660_e27187), ((locals.var_t1_dn10 * assign21660_e27183) * assign21660_e27187), ((locals.var_t1_dn11 * assign21660_e27183) * assign21660_e27187), ((locals.var_t1_dn12 * assign21660_e27183) * assign21660_e27187), );
        locals.var_qfd_box_rv = 0.0;
        let assign21670_e27191: f64 = (locals.var_t1 * locals.var_t2);
        (locals.var_qs_dep, locals.var_qs_dep_dn0, locals.var_qs_dep_dn2, locals.var_qs_dep_dn4, locals.var_qs_dep_dn5, locals.var_qs_dep_dn6, locals.var_qs_dep_dn8, locals.var_qs_dep_dn10, locals.var_qs_dep_dn11, locals.var_qs_dep_dn12, ) = (assign21670_e27191, ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)), ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)), ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)), ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)), ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)), ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)), ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)), ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)), ((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12)), );
        locals.var_qs_dep_rv = 0.0;
        let assign21680_e27194: f64 = (locals.var_t1 * locals.var_t3);
        (locals.var_qb_dep, locals.var_qb_dep_dn0, locals.var_qb_dep_dn2, locals.var_qb_dep_dn4, locals.var_qb_dep_dn5, locals.var_qb_dep_dn6, locals.var_qb_dep_dn8, locals.var_qb_dep_dn10, locals.var_qb_dep_dn11, locals.var_qb_dep_dn12, ) = (assign21680_e27194, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), );
        locals.var_qb_dep_rv = 0.0;
        if (p.p303 != 0.0) {
            (locals.var_qsub, locals.var_qsub_dn0, locals.var_qsub_dn2, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn8, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qsub_rv = 0.0;
            (locals.var_qidep, locals.var_qidep_dn0, locals.var_qidep_dn2, locals.var_qidep_dn4, locals.var_qidep_dn5, locals.var_qidep_dn6, locals.var_qidep_dn8, locals.var_qidep_dn10, locals.var_qidep_dn11, locals.var_qidep_dn12, ) = (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn4, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn8, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, );
            locals.var_qidep_rv = 0.0;
        }
        if (p.p303 == 0.0) {
            let assign21710_e27207: f64 = (locals.var_qi + locals.var_qs_dep);
            let assign21710_e27209: f64 = (assign21710_e27207 + locals.var_qb_dep);
            (locals.var_qidep, locals.var_qidep_dn0, locals.var_qidep_dn2, locals.var_qidep_dn4, locals.var_qidep_dn5, locals.var_qidep_dn6, locals.var_qidep_dn8, locals.var_qidep_dn10, locals.var_qidep_dn11, locals.var_qidep_dn12, ) = (assign21710_e27209, ((locals.var_qi_dn0 + locals.var_qs_dep_dn0) + locals.var_qb_dep_dn0), ((locals.var_qi_dn2 + locals.var_qs_dep_dn2) + locals.var_qb_dep_dn2), ((locals.var_qi_dn4 + locals.var_qs_dep_dn4) + locals.var_qb_dep_dn4), ((locals.var_qi_dn5 + locals.var_qs_dep_dn5) + locals.var_qb_dep_dn5), ((locals.var_qi_dn6 + locals.var_qs_dep_dn6) + locals.var_qb_dep_dn6), ((locals.var_qi_dn8 + locals.var_qs_dep_dn8) + locals.var_qb_dep_dn8), ((locals.var_qi_dn10 + locals.var_qs_dep_dn10) + locals.var_qb_dep_dn10), ((locals.var_qi_dn11 + locals.var_qs_dep_dn11) + locals.var_qb_dep_dn11), ((locals.var_qi_dn12 + locals.var_qs_dep_dn12) + locals.var_qb_dep_dn12), );
            locals.var_qidep_rv = 0.0;
        }
        let assign21720_e27214: f64 = (locals.var_qidep * locals.var_qdrat);
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn8, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, ) = (assign21720_e27214, (locals.var_qidep_dn0 * locals.var_qdrat), (locals.var_qidep_dn2 * locals.var_qdrat), (locals.var_qidep_dn4 * locals.var_qdrat), (locals.var_qidep_dn5 * locals.var_qdrat), (locals.var_qidep_dn6 * locals.var_qdrat), (locals.var_qidep_dn8 * locals.var_qdrat), (locals.var_qidep_dn10 * locals.var_qdrat), (locals.var_qidep_dn11 * locals.var_qdrat), (locals.var_qidep_dn12 * locals.var_qdrat), );
        locals.var_qd_rv = 0.0;
        if (locals.var_flg_nqs != 0.0) {
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qde_rv = 0.0;
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn8, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qge_rv = 0.0;
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn8, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbe_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign21760_e27230: f64 = (locals.var_mfactor * locals.var_qsub);
            (locals.var_qb_qs, locals.var_qb_qs_dn0, locals.var_qb_qs_dn2, locals.var_qb_qs_dn4, locals.var_qb_qs_dn5, locals.var_qb_qs_dn6, locals.var_qb_qs_dn8, locals.var_qb_qs_dn10, locals.var_qb_qs_dn11, locals.var_qb_qs_dn12, ) = (assign21760_e27230, (locals.var_mfactor * locals.var_qsub_dn0), (locals.var_mfactor * locals.var_qsub_dn2), (locals.var_mfactor * locals.var_qsub_dn4), (locals.var_mfactor * locals.var_qsub_dn5), (locals.var_mfactor * locals.var_qsub_dn6), (locals.var_mfactor * locals.var_qsub_dn8), (locals.var_mfactor * locals.var_qsub_dn10), (locals.var_mfactor * locals.var_qsub_dn11), (locals.var_mfactor * locals.var_qsub_dn12), );
            locals.var_qb_qs_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign21770_e27236: f64 = (locals.var_mfactor * locals.var_qidep);
            (locals.var_qi_qs, locals.var_qi_qs_dn0, locals.var_qi_qs_dn2, locals.var_qi_qs_dn4, locals.var_qi_qs_dn5, locals.var_qi_qs_dn6, locals.var_qi_qs_dn8, locals.var_qi_qs_dn10, locals.var_qi_qs_dn11, locals.var_qi_qs_dn12, ) = (assign21770_e27236, (locals.var_mfactor * locals.var_qidep_dn0), (locals.var_mfactor * locals.var_qidep_dn2), (locals.var_mfactor * locals.var_qidep_dn4), (locals.var_mfactor * locals.var_qidep_dn5), (locals.var_mfactor * locals.var_qidep_dn6), (locals.var_mfactor * locals.var_qidep_dn8), (locals.var_mfactor * locals.var_qidep_dn10), (locals.var_mfactor * locals.var_qidep_dn11), (locals.var_mfactor * locals.var_qidep_dn12), );
            locals.var_qi_qs_rv = 0.0;
        }
        if (locals.var_flg_nqs == 0.0) {
            let assign21780_e27243: f64 = (-locals.var_qsub);
            let assign21780_e27245: f64 = (assign21780_e27243 - locals.var_qidep);
            let assign21780_e27246: f64 = (locals.var_mfactor * assign21780_e27245);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn8, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, ) = (assign21780_e27246, (locals.var_mfactor * ((-locals.var_qsub_dn0) - locals.var_qidep_dn0)), (locals.var_mfactor * ((-locals.var_qsub_dn2) - locals.var_qidep_dn2)), (locals.var_mfactor * ((-locals.var_qsub_dn4) - locals.var_qidep_dn4)), (locals.var_mfactor * ((-locals.var_qsub_dn5) - locals.var_qidep_dn5)), (locals.var_mfactor * ((-locals.var_qsub_dn6) - locals.var_qidep_dn6)), (locals.var_mfactor * ((-locals.var_qsub_dn8) - locals.var_qidep_dn8)), (locals.var_mfactor * ((-locals.var_qsub_dn10) - locals.var_qidep_dn10)), (locals.var_mfactor * ((-locals.var_qsub_dn11) - locals.var_qidep_dn11)), (locals.var_mfactor * ((-locals.var_qsub_dn12) - locals.var_qidep_dn12)), );
            locals.var_qge_rv = 0.0;
        }
        if (locals.var_flg_nqs == 0.0) {
            let assign21790_e27254: f64 = (locals.var_qd + locals.var_qfd_box);
            let assign21790_e27255: f64 = (locals.var_mfactor * assign21790_e27254);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, ) = (assign21790_e27255, (locals.var_mfactor * (locals.var_qd_dn0 + locals.var_qfd_box_dn0)), (locals.var_mfactor * (locals.var_qd_dn2 + locals.var_qfd_box_dn2)), (locals.var_mfactor * (locals.var_qd_dn4 + locals.var_qfd_box_dn4)), (locals.var_mfactor * (locals.var_qd_dn5 + locals.var_qfd_box_dn5)), (locals.var_mfactor * (locals.var_qd_dn6 + locals.var_qfd_box_dn6)), (locals.var_mfactor * (locals.var_qd_dn8 + locals.var_qfd_box_dn8)), (locals.var_mfactor * (locals.var_qd_dn10 + locals.var_qfd_box_dn10)), (locals.var_mfactor * (locals.var_qd_dn11 + locals.var_qfd_box_dn11)), (locals.var_mfactor * (locals.var_qd_dn12 + locals.var_qfd_box_dn12)), );
            locals.var_qde_rv = 0.0;
        }
        if (locals.var_flg_nqs == 0.0) {
            let assign21800_e27263: f64 = (locals.var_qidep - locals.var_qd);
            let assign21800_e27265: f64 = (assign21800_e27263 + locals.var_qfs_box);
            let assign21800_e27266: f64 = (locals.var_mfactor * assign21800_e27265);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn8, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, ) = (assign21800_e27266, (locals.var_mfactor * ((locals.var_qidep_dn0 - locals.var_qd_dn0) + locals.var_qfs_box_dn0)), (locals.var_mfactor * ((locals.var_qidep_dn2 - locals.var_qd_dn2) + locals.var_qfs_box_dn2)), (locals.var_mfactor * ((locals.var_qidep_dn4 - locals.var_qd_dn4) + locals.var_qfs_box_dn4)), (locals.var_mfactor * ((locals.var_qidep_dn5 - locals.var_qd_dn5) + locals.var_qfs_box_dn5)), (locals.var_mfactor * ((locals.var_qidep_dn6 - locals.var_qd_dn6) + locals.var_qfs_box_dn6)), (locals.var_mfactor * ((locals.var_qidep_dn8 - locals.var_qd_dn8) + locals.var_qfs_box_dn8)), (locals.var_mfactor * ((locals.var_qidep_dn10 - locals.var_qd_dn10) + locals.var_qfs_box_dn10)), (locals.var_mfactor * ((locals.var_qidep_dn11 - locals.var_qd_dn11) + locals.var_qfs_box_dn11)), (locals.var_mfactor * ((locals.var_qidep_dn12 - locals.var_qd_dn12) + locals.var_qfs_box_dn12)), );
            locals.var_qse_rv = 0.0;
        }
        let assign21810_e27271: f64 = if p.p45 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard372 = assign21810_e27271;
        locals.var_guard372_rv = 0.0;
        if (locals.var_guard372 != 0.0) {
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn8, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qy_rv = 0.0;
        }
        if (locals.var_guard372 == 0.0) {
            let assign21830_e27280: f64 = (locals.var_ec * locals.var_leff);
            let assign21830_e27282: f64 = (assign21830_e27280 + locals.var_ps0);
            (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn8, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, ) = (assign21830_e27282, (((locals.var_ec_dn0 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn0)) + locals.var_ps0_dn0), (((locals.var_ec_dn2 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn2)) + locals.var_ps0_dn2), (((locals.var_ec_dn4 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn4)) + locals.var_ps0_dn4), (((locals.var_ec_dn5 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn5)) + locals.var_ps0_dn5), (((locals.var_ec_dn6 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn6)) + locals.var_ps0_dn6), (((locals.var_ec_dn8 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn8)) + locals.var_ps0_dn8), (((locals.var_ec_dn10 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn10)) + locals.var_ps0_dn10), (((locals.var_ec_dn11 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn11)) + locals.var_ps0_dn11), (((locals.var_ec_dn12 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn12)) + locals.var_ps0_dn12), );
            locals.var_pslk_rv = 0.0;
        }
        let assign21840_e27287: f64 = if locals.var_pslk > locals.var_psdl { 1.0 } else { 0.0 };
        locals.var_guard373 = assign21840_e27287;
        locals.var_guard373_rv = 0.0;
        if ((locals.var_guard372 == 0.0) && (locals.var_guard373 != 0.0)) {
            (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn8, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, ) = (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn8, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, );
            locals.var_pslk_rv = 0.0;
        }
        if (locals.var_guard372 == 0.0) {
            let assign21860_e27300: f64 = (locals.var_vds + locals.var_ps0);
            let assign21860_e27301: f64 = (locals.var_aclm * assign21860_e27300);
            let assign21860_e27304: f64 = (1.0 - locals.var_aclm);
            let assign21860_e27306: f64 = (assign21860_e27304 * locals.var_pslk);
            let assign21860_e27307: f64 = (assign21860_e27301 + assign21860_e27306);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign21860_e27307, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign21860_e27304 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign21860_e27304 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + (assign21860_e27304 * locals.var_pslk_dn4)), ((locals.var_aclm * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + (assign21860_e27304 * locals.var_pslk_dn5)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign21860_e27304 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + (assign21860_e27304 * locals.var_pslk_dn8)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign21860_e27304 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign21860_e27304 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + (assign21860_e27304 * locals.var_pslk_dn12)), );
            locals.var_t1_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_35(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv12 = ctx.node_voltage(nodes[12]);
        if (locals.var_guard372 == 0.0) {
            let assign21870_e27314: f64 = (2.0 * 1.034943e-10);
            let assign21870_e27316: f64 = (assign21870_e27314 / locals.var_q_nsub);
            let assign21870_e27317: f64 = (assign21870_e27316).sqrt();
            (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, ) = (assign21870_e27317, ((-((assign21870_e27314 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn4) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn5) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn8) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), );
            locals.var_t10_rv = 0.0;
        }
        if (locals.var_guard372 == 0.0) {
            let assign21880_e27324: f64 = (locals.var_t10 * 1.3);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, ) = (assign21880_e27324, (locals.var_t10_dn0 * 1.3), (locals.var_t10_dn2 * 1.3), (locals.var_t10_dn4 * 1.3), (locals.var_t10_dn5 * 1.3), (locals.var_t10_dn6 * 1.3), (locals.var_t10_dn8 * 1.3), (locals.var_t10_dn10 * 1.3), (locals.var_t10_dn11 * 1.3), (locals.var_t10_dn12 * 1.3), );
            locals.var_t3_rv = 0.0;
        }
        if (locals.var_guard372 == 0.0) {
            let assign21890_e27331: f64 = (1.034943e-10 * locals.var_weffcv_nf);
            let assign21890_e27333: f64 = (assign21890_e27331 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign21890_e27333, (((1.034943e-10 * locals.var_weffcv_nf_dn0) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn0)), (((1.034943e-10 * locals.var_weffcv_nf_dn2) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn2)), (((1.034943e-10 * locals.var_weffcv_nf_dn4) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn4)), (((1.034943e-10 * locals.var_weffcv_nf_dn5) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn5)), (((1.034943e-10 * locals.var_weffcv_nf_dn6) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn6)), (((1.034943e-10 * locals.var_weffcv_nf_dn8) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn8)), (((1.034943e-10 * locals.var_weffcv_nf_dn10) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn10)), (((1.034943e-10 * locals.var_weffcv_nf_dn11) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn11)), (((1.034943e-10 * locals.var_weffcv_nf_dn12) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn12)), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard372 == 0.0) {
            let assign21900_e27340: f64 = (locals.var_ps0 + locals.var_vds);
            let assign21900_e27342: f64 = (assign21900_e27340 - locals.var_t1);
            let assign21900_e27344: f64 = (assign21900_e27342 / p.p45);
            let assign21900_e27346: f64 = (assign21900_e27344 - locals.var_ec);
            let assign21900_e27348: f64 = (assign21900_e27346 * locals.var_t2);
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn8, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, ) = (assign21900_e27348, ((((((locals.var_ps0_dn0 + locals.var_vds_dn0) - locals.var_t1_dn0) / p.p45) - locals.var_ec_dn0) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn0)), ((((((locals.var_ps0_dn2 + locals.var_vds_dn2) - locals.var_t1_dn2) / p.p45) - locals.var_ec_dn2) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn2)), ((((((locals.var_ps0_dn4 + locals.var_vds_dn4) - locals.var_t1_dn4) / p.p45) - locals.var_ec_dn4) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn4)), ((((((locals.var_ps0_dn5 + locals.var_vds_dn5) - locals.var_t1_dn5) / p.p45) - locals.var_ec_dn5) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn5)), ((((((locals.var_ps0_dn6 + locals.var_vds_dn6) - locals.var_t1_dn6) / p.p45) - locals.var_ec_dn6) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn6)), ((((((locals.var_ps0_dn8 + locals.var_vds_dn8) - locals.var_t1_dn8) / p.p45) - locals.var_ec_dn8) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn8)), ((((((locals.var_ps0_dn10 + locals.var_vds_dn10) - locals.var_t1_dn10) / p.p45) - locals.var_ec_dn10) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn10)), ((((((locals.var_ps0_dn11 + locals.var_vds_dn11) - locals.var_t1_dn11) / p.p45) - locals.var_ec_dn11) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn11)), ((((((locals.var_ps0_dn12 + locals.var_vds_dn12) - locals.var_t1_dn12) / p.p45) - locals.var_ec_dn12) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn12)), );
            locals.var_qy_rv = 0.0;
        }
        let assign21910_e27353: f64 = if p.p46 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard374 = assign21910_e27353;
        locals.var_guard374_rv = 0.0;
        if (locals.var_guard374 != 0.0) {
            let assign21920_e27358: f64 = (locals.var_cqyb0 * locals.var_vbs);
            let assign21920_e27359: f64 = (locals.var_qy + assign21920_e27358);
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn8, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, ) = (assign21920_e27359, (locals.var_qy_dn0 + ((locals.var_cqyb0_dn0 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn0))), (locals.var_qy_dn2 + ((locals.var_cqyb0_dn2 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn2))), (locals.var_qy_dn4 + ((locals.var_cqyb0_dn4 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn4))), (locals.var_qy_dn5 + ((locals.var_cqyb0_dn5 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn5))), (locals.var_qy_dn6 + ((locals.var_cqyb0_dn6 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn6))), (locals.var_qy_dn8 + ((locals.var_cqyb0_dn8 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn8))), (locals.var_qy_dn10 + ((locals.var_cqyb0_dn10 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn10))), (locals.var_qy_dn11 + ((locals.var_cqyb0_dn11 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn11))), (locals.var_qy_dn12 + ((locals.var_cqyb0_dn12 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn12))), );
            locals.var_qy_rv = 0.0;
        }
        let assign21930_e27364: f64 = if p.p14 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard375 = assign21930_e27364;
        locals.var_guard375_rv = 0.0;
        if (locals.var_guard375 != 0.0) {
            let assign21940_e27370: f64 = (locals.var_qgod + locals.var_qgos);
            let assign21940_e27372: f64 = (assign21940_e27370 - locals.var_qgob);
            let assign21940_e27374: f64 = (assign21940_e27372 - locals.var_qy);
            let assign21940_e27376: f64 = (assign21940_e27374 - locals.var_qovs);
            let assign21940_e27378: f64 = (assign21940_e27376 - locals.var_qovd);
            let assign21940_e27379: f64 = (locals.var_mfactor * assign21940_e27378);
            let assign21940_e27380: f64 = (locals.var_qge + assign21940_e27379);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn8, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, ) = (assign21940_e27380, (locals.var_qge_dn0 + (locals.var_mfactor * (((((locals.var_qgod_dn0 + locals.var_qgos_dn0) - locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * (((((locals.var_qgod_dn2 + locals.var_qgos_dn2) - locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2))), (locals.var_qge_dn4 + (locals.var_mfactor * ((((locals.var_qgod_dn4 + locals.var_qgos_dn4) - locals.var_qy_dn4) - locals.var_qovs_dn4) - locals.var_qovd_dn4))), (locals.var_qge_dn5 + (locals.var_mfactor * (((((locals.var_qgod_dn5 + locals.var_qgos_dn5) - locals.var_qgob_dn5) - locals.var_qy_dn5) - locals.var_qovs_dn5) - locals.var_qovd_dn5))), (locals.var_qge_dn6 + (locals.var_mfactor * (((((locals.var_qgod_dn6 + locals.var_qgos_dn6) - locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6))), (locals.var_qge_dn8 + (locals.var_mfactor * ((((locals.var_qgod_dn8 + locals.var_qgos_dn8) - locals.var_qy_dn8) - locals.var_qovs_dn8) - locals.var_qovd_dn8))), (locals.var_qge_dn10 + (locals.var_mfactor * ((((locals.var_qgod_dn10 + locals.var_qgos_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((((locals.var_qgod_dn11 + locals.var_qgos_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * ((((locals.var_qgod_dn12 + locals.var_qgos_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12))), );
            locals.var_qge_rv = 0.0;
        }
        if (locals.var_guard375 != 0.0) {
            let assign21950_e27387: f64 = (-locals.var_qgod);
            let assign21950_e27389: f64 = (assign21950_e27387 + locals.var_qy);
            let assign21950_e27391: f64 = (assign21950_e27389 + locals.var_qbdld);
            let assign21950_e27392: f64 = (locals.var_mfactor * assign21950_e27391);
            let assign21950_e27393: f64 = (locals.var_qde + assign21950_e27392);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, ) = (assign21950_e27393, (locals.var_qde_dn0 + (locals.var_mfactor * (((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * (((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2))), (locals.var_qde_dn4 + (locals.var_mfactor * (((-locals.var_qgod_dn4) + locals.var_qy_dn4) + locals.var_qbdld_dn4))), (locals.var_qde_dn5 + (locals.var_mfactor * (((-locals.var_qgod_dn5) + locals.var_qy_dn5) + locals.var_qbdld_dn5))), (locals.var_qde_dn6 + (locals.var_mfactor * (((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6))), (locals.var_qde_dn8 + (locals.var_mfactor * (((-locals.var_qgod_dn8) + locals.var_qy_dn8) + locals.var_qbdld_dn8))), (locals.var_qde_dn10 + (locals.var_mfactor * (((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * (((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * (((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12))), );
            locals.var_qde_rv = 0.0;
        }
        if (locals.var_guard375 != 0.0) {
            let assign21960_e27400: f64 = (-locals.var_qgos);
            let assign21960_e27402: f64 = (assign21960_e27400 + locals.var_qbsld);
            let assign21960_e27403: f64 = (locals.var_mfactor * assign21960_e27402);
            let assign21960_e27404: f64 = (locals.var_qse + assign21960_e27403);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn8, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, ) = (assign21960_e27404, (locals.var_qse_dn0 + (locals.var_mfactor * ((-locals.var_qgos_dn0) + locals.var_qbsld_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * ((-locals.var_qgos_dn2) + locals.var_qbsld_dn2))), (locals.var_qse_dn4 + (locals.var_mfactor * ((-locals.var_qgos_dn4) + locals.var_qbsld_dn4))), (locals.var_qse_dn5 + (locals.var_mfactor * ((-locals.var_qgos_dn5) + locals.var_qbsld_dn5))), (locals.var_qse_dn6 + (locals.var_mfactor * ((-locals.var_qgos_dn6) + locals.var_qbsld_dn6))), (locals.var_qse_dn8 + (locals.var_mfactor * ((-locals.var_qgos_dn8) + locals.var_qbsld_dn8))), (locals.var_qse_dn10 + (locals.var_mfactor * ((-locals.var_qgos_dn10) + locals.var_qbsld_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * ((-locals.var_qgos_dn11) + locals.var_qbsld_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * ((-locals.var_qgos_dn12) + locals.var_qbsld_dn12))), );
            locals.var_qse_rv = 0.0;
        }
        let assign21970_e27409: f64 = (locals.var_mfactor * locals.var_isub);
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn4, locals.var_isube_dn5, locals.var_isube_dn6, locals.var_isube_dn8, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, ) = (assign21970_e27409, (locals.var_mfactor * locals.var_isub_dn0), (locals.var_mfactor * locals.var_isub_dn2), (locals.var_mfactor * locals.var_isub_dn4), (locals.var_mfactor * locals.var_isub_dn5), (locals.var_mfactor * locals.var_isub_dn6), (locals.var_mfactor * locals.var_isub_dn8), (locals.var_mfactor * locals.var_isub_dn10), (locals.var_mfactor * locals.var_isub_dn11), (locals.var_mfactor * locals.var_isub_dn12), );
        locals.var_isube_rv = 0.0;
        let assign22010_e27418: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard376 = assign22010_e27418;
        locals.var_guard376_rv = 0.0;
        if (locals.var_guard376 == 0.0) {
            let assign22030_e27433: f64 = (1.0 - locals.var_glpart1);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign22030_e27433, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        let assign22050_e27449: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign22050_e27449;
        locals.var_guard377_rv = 0.0;
        if (locals.var_guard377 != 0.0) {
            let assign22060_e27453: f64 = (1.0 - locals.var_glpart1);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign22060_e27453, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        let assign22110_e27499: f64 = (4.0 * 1.3806226e-23);
        let assign22110_e27501: f64 = (assign22110_e27499 * locals.var_ttemp);
        let assign22110_e27503: f64 = assign22110_e27501;
        (locals.var_whi_noise, locals.var_whi_noise_dn4, ) = (assign22110_e27503, (assign22110_e27499 * locals.var_ttemp_dn4), );
        locals.var_whi_noise_rv = 0.0;
        let assign22130_e27509: f64 = (locals.var_mfactor * locals.var_nthrml);
        (locals.var_noithrml, locals.var_noithrml_dn0, locals.var_noithrml_dn2, locals.var_noithrml_dn4, locals.var_noithrml_dn5, locals.var_noithrml_dn6, locals.var_noithrml_dn8, locals.var_noithrml_dn10, locals.var_noithrml_dn11, locals.var_noithrml_dn12, ) = (assign22130_e27509, (locals.var_mfactor * locals.var_nthrml_dn0), (locals.var_mfactor * locals.var_nthrml_dn2), (locals.var_mfactor * locals.var_nthrml_dn4), (locals.var_mfactor * locals.var_nthrml_dn5), (locals.var_mfactor * locals.var_nthrml_dn6), (locals.var_mfactor * locals.var_nthrml_dn8), (locals.var_mfactor * locals.var_nthrml_dn10), (locals.var_mfactor * locals.var_nthrml_dn11), (locals.var_mfactor * locals.var_nthrml_dn12), );
        locals.var_noithrml_rv = 0.0;
        let assign22140_e27512: f64 = locals.var_qge_dn11;
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn8, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, ) = (assign22140_e27512, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgdbd_rv = 0.0;
        let assign22150_e27515: f64 = (p.p33 * locals.var_cgdbd);
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn8, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, ) = (assign22150_e27515, (p.p33 * locals.var_cgdbd_dn0), (p.p33 * locals.var_cgdbd_dn2), (p.p33 * locals.var_cgdbd_dn4), (p.p33 * locals.var_cgdbd_dn5), (p.p33 * locals.var_cgdbd_dn6), (p.p33 * locals.var_cgdbd_dn8), (p.p33 * locals.var_cgdbd_dn10), (p.p33 * locals.var_cgdbd_dn11), (p.p33 * locals.var_cgdbd_dn12), );
        locals.var_cgdbd_rv = 0.0;
        let assign22160_e27518: f64 = locals.var_qge_dn12;
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn8, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, ) = (assign22160_e27518, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgsbd_rv = 0.0;
        let assign22170_e27521: f64 = (p.p33 * locals.var_cgsbd);
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn8, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, ) = (assign22170_e27521, (p.p33 * locals.var_cgsbd_dn0), (p.p33 * locals.var_cgsbd_dn2), (p.p33 * locals.var_cgsbd_dn4), (p.p33 * locals.var_cgsbd_dn5), (p.p33 * locals.var_cgsbd_dn6), (p.p33 * locals.var_cgsbd_dn8), (p.p33 * locals.var_cgsbd_dn10), (p.p33 * locals.var_cgsbd_dn11), (p.p33 * locals.var_cgsbd_dn12), );
        locals.var_cgsbd_rv = 0.0;
        let (assign22180_e27527, assign22180_e27527_d_n0, assign22180_e27527_d_n2, assign22180_e27527_d_n4, assign22180_e27527_d_n5, assign22180_e27527_d_n6, assign22180_e27527_d_n8, assign22180_e27527_d_n10, assign22180_e27527_d_n11, assign22180_e27527_d_n12,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn8, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn8, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12,)
    }
};
        (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn8, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn12, ) = (assign22180_e27527, assign22180_e27527_d_n0, assign22180_e27527_d_n2, assign22180_e27527_d_n4, assign22180_e27527_d_n5, assign22180_e27527_d_n6, assign22180_e27527_d_n8, assign22180_e27527_d_n10, assign22180_e27527_d_n11, assign22180_e27527_d_n12, );
        locals.var_cgsb_rv = 0.0;
        let assign22190_e27541: f64 = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard378 = assign22190_e27541;
        locals.var_guard378_rv = 0.0;
        if (locals.var_guard378 != 0.0) {
            let assign22200_e27545: f64 = (1e-6 * locals.var_c_fox);
            let assign22200_e27547: f64 = (assign22200_e27545 * locals.var_weffcv_nf);
            let assign22200_e27549: f64 = (assign22200_e27547 * locals.var_leff);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, ) = (assign22200_e27549, (((((1e-6 * locals.var_c_fox_dn0) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn0)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn0)), (((((1e-6 * locals.var_c_fox_dn2) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn2)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn2)), (((((1e-6 * locals.var_c_fox_dn4) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn4)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn4)), (((((1e-6 * locals.var_c_fox_dn5) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn5)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn5)), (((((1e-6 * locals.var_c_fox_dn6) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn6)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn6)), (((((1e-6 * locals.var_c_fox_dn8) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn8)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn8)), (((((1e-6 * locals.var_c_fox_dn10) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn10)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn10)), (((((1e-6 * locals.var_c_fox_dn11) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn11)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn11)), (((((1e-6 * locals.var_c_fox_dn12) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn12)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn12)), );
            locals.var_t0_rv = 0.0;
        }
        if (locals.var_guard378 != 0.0) {
            let assign22210_e27555: f64 = (locals.var_cgsb / locals.var_mfactor);
            (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, ) = (assign22210_e27555, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn4 / locals.var_mfactor), (locals.var_cgsb_dn5 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn8 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn12 / locals.var_mfactor), );
            locals.var_t10_rv = 0.0;
        }
        if (locals.var_guard378 != 0.0) {
            let assign22220_e27561: f64 = (0.1185185185185185 * 1.6021918e-19);
            let assign22220_e27563: f64 = (assign22220_e27561 * locals.var_beta_inv);
            let assign22220_e27565: f64 = (assign22220_e27563 * locals.var_t10);
            let assign22220_e27567: f64 = (assign22220_e27565 * locals.var_t10);
            let assign22220_e27569: f64 = (assign22220_e27567 / locals.var_gds0_ign);
            (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn4, locals.var_nign0_dn5, locals.var_nign0_dn6, locals.var_nign0_dn8, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn12, ) = (assign22220_e27569, ((((((assign22220_e27563 * locals.var_t10_dn0) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn0)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn2) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn2)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign22220_e27561 * locals.var_beta_inv_dn4) * locals.var_t10) + (assign22220_e27563 * locals.var_t10_dn4)) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn4)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn4)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn5) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn5)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn5)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn6) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn6)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn8) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn8)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn8)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn10) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn10)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn11) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn11)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn12) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn12)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn12)) / (locals.var_gds0_ign * locals.var_gds0_ign)), );
            locals.var_nign0_rv = 0.0;
        }
        let assign22230_e27575: f64 = (10.0 * 2.220446049250313e-16);
        let assign22230_e27580: f64 = (10.0 * 2.220446049250313e-16);
        let assign22230_e27582: f64 = if ((locals.var_kusai00l > assign22230_e27575) && (locals.var_vds > assign22230_e27580)) { 1.0 } else { 0.0 };
        locals.var_guard379 = assign22230_e27582;
        locals.var_guard379_rv = 0.0;
        if ((locals.var_guard378 != 0.0) && (locals.var_guard379 != 0.0)) {
            let assign22240_e27588: f64 = (locals.var_muun / locals.var_mu);
            (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn4, locals.var_mumoda_dn5, locals.var_mumoda_dn6, locals.var_mumoda_dn8, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn12, ) = (assign22240_e27588, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn4 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn4)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn5 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn5)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn8 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn8)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn12 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn12)) / (locals.var_mu * locals.var_mu)), );
            locals.var_mumoda_rv = 0.0;
        }
        if ((locals.var_guard378 != 0.0) && (locals.var_guard379 != 0.0)) {
            let assign22250_e27596: f64 = (locals.var_muun / locals.var_mud_hoso);
            let assign22250_e27598: f64 = (assign22250_e27596 - locals.var_mumoda);
            let assign22250_e27600: f64 = (assign22250_e27598 / locals.var_vds);
            (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn4, locals.var_mumodb_dn5, locals.var_mumodb_dn6, locals.var_mumodb_dn8, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn12, ) = (assign22250_e27600, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn4) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn4)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn5) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn5)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn8) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn8)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn12) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn12)) / (locals.var_vds * locals.var_vds)), );
            locals.var_mumodb_rv = 0.0;
        }
        if ((locals.var_guard378 != 0.0) && (locals.var_guard379 != 0.0)) {
            let assign22260_e27609: f64 = (0.6666666666666667 * locals.var_mumodb);
            let assign22260_e27613: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
            let assign22260_e27614: f64 = (locals.var_kusai00 + assign22260_e27613);
            let assign22260_e27616: f64 = (assign22260_e27614 + locals.var_kusail);
            let assign22260_e27617: f64 = (assign22260_e27609 * assign22260_e27616);
            let assign22260_e27620: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
            let assign22260_e27621: f64 = (assign22260_e27617 / assign22260_e27620);
            let assign22260_e27622: f64 = (locals.var_mumoda + assign22260_e27621);
            (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn8, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, ) = (assign22260_e27622, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn4 + ((((((0.6666666666666667 * locals.var_mumodb_dn4) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn4 + ((locals.var_vgvt_dn4 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn5 + ((((((0.6666666666666667 * locals.var_mumodb_dn5) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn5 + ((locals.var_vgvt_dn5 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn8 + ((((((0.6666666666666667 * locals.var_mumodb_dn8) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn8 + ((locals.var_vgvt_dn8 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn12 + ((((((0.6666666666666667 * locals.var_mumodb_dn12) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn12 + ((locals.var_vgvt_dn12 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12))) / (assign22260_e27620 * assign22260_e27620))), );
            locals.var_correct_w1_rv = 0.0;
        }
        if ((locals.var_guard378 != 0.0) && (locals.var_guard379 == 0.0)) {
            let assign22270_e27631: f64 = (locals.var_muun / locals.var_mud_hoso);
            (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn8, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12, ) = (assign22270_e27631, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)), );
            locals.var_correct_w1_rv = 0.0;
        }
        if (locals.var_guard378 != 0.0) {
            let assign22280_e27637: f64 = (locals.var_mfactor * locals.var_nign0);
            let assign22280_e27639: f64 = (assign22280_e27637 * locals.var_kusai_ig);
            let assign22280_e27641: f64 = (assign22280_e27639 * locals.var_correct_w1);
            (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, ) = (assign22280_e27641, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn4) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn4)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn4)), (((((locals.var_mfactor * locals.var_nign0_dn5) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn5)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn5)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn8) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn8)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn8)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn12) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn12)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn12)), );
            locals.var_noiigate_rv = 0.0;
        }
        if (locals.var_guard378 != 0.0) {
            let (assign22300_e27654, assign22300_e27654_d_n0, assign22300_e27654_d_n2, assign22300_e27654_d_n4, assign22300_e27654_d_n5, assign22300_e27654_d_n6, assign22300_e27654_d_n8, assign22300_e27654_d_n10, assign22300_e27654_d_n11, assign22300_e27654_d_n12,) = {
    if (locals.var_noiigate < 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12,)
    }
};
            (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, ) = (assign22300_e27654, assign22300_e27654_d_n0, assign22300_e27654_d_n2, assign22300_e27654_d_n4, assign22300_e27654_d_n5, assign22300_e27654_d_n6, assign22300_e27654_d_n8, assign22300_e27654_d_n10, assign22300_e27654_d_n11, assign22300_e27654_d_n12, );
            locals.var_noiigate_rv = 0.0;
        }
        if (locals.var_guard378 != 0.0) {
            let assign22310_e27659: f64 = (-locals.var_t10);
            let (assign22310_e27664, assign22310_e27664_d_n0, assign22310_e27664_d_n2, assign22310_e27664_d_n4, assign22310_e27664_d_n5, assign22310_e27664_d_n6, assign22310_e27664_d_n8, assign22310_e27664_d_n10, assign22310_e27664_d_n11, assign22310_e27664_d_n12,) = {
    if (assign22310_e27659 > locals.var_t0) {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, ) = (assign22310_e27664, assign22310_e27664_d_n0, assign22310_e27664_d_n2, assign22310_e27664_d_n4, assign22310_e27664_d_n5, assign22310_e27664_d_n6, assign22310_e27664_d_n8, assign22310_e27664_d_n10, assign22310_e27664_d_n11, assign22310_e27664_d_n12, );
            locals.var_noiigate_rv = 0.0;
        }
        if (locals.var_guard378 == 0.0) {
            (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_noiigate_rv = 0.0;
        }
        let assign22350_e27689: f64 = (locals.var_whi_noise * locals.var_noithrml);
        (locals.var_sid, locals.var_sid_dn0, locals.var_sid_dn2, locals.var_sid_dn4, locals.var_sid_dn5, locals.var_sid_dn6, locals.var_sid_dn8, locals.var_sid_dn10, locals.var_sid_dn11, locals.var_sid_dn12, ) = (assign22350_e27689, (locals.var_whi_noise * locals.var_noithrml_dn0), (locals.var_whi_noise * locals.var_noithrml_dn2), ((locals.var_whi_noise_dn4 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn4)), (locals.var_whi_noise * locals.var_noithrml_dn5), (locals.var_whi_noise * locals.var_noithrml_dn6), (locals.var_whi_noise * locals.var_noithrml_dn8), (locals.var_whi_noise * locals.var_noithrml_dn10), (locals.var_whi_noise * locals.var_noithrml_dn11), (locals.var_whi_noise * locals.var_noithrml_dn12), );
        locals.var_sid_rv = 0.0;
        let (assign22370_e27703, assign22370_e27703_d_n0, assign22370_e27703_d_n2, assign22370_e27703_d_n4, assign22370_e27703_d_n5, assign22370_e27703_d_n6, assign22370_e27703_d_n8, assign22370_e27703_d_n10, assign22370_e27703_d_n11, assign22370_e27703_d_n12,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign22370_e27700: f64 = (locals.var_noiigate / locals.var_sid);
        let assign22370_e27701: f64 = (assign22370_e27700).sqrt();
        (assign22370_e27701, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn4 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn4)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn5 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn5)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn8 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn8)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn12 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn12)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        (locals.var_sigrat, locals.var_sigrat_dn0, locals.var_sigrat_dn2, locals.var_sigrat_dn4, locals.var_sigrat_dn5, locals.var_sigrat_dn6, locals.var_sigrat_dn8, locals.var_sigrat_dn10, locals.var_sigrat_dn11, locals.var_sigrat_dn12, ) = (assign22370_e27703, assign22370_e27703_d_n0, assign22370_e27703_d_n2, assign22370_e27703_d_n4, assign22370_e27703_d_n5, assign22370_e27703_d_n6, assign22370_e27703_d_n8, assign22370_e27703_d_n10, assign22370_e27703_d_n11, assign22370_e27703_d_n12, );
        locals.var_sigrat_rv = 0.0;
        let (assign22380_e27715, assign22380_e27715_d_n0, assign22380_e27715_d_n2, assign22380_e27715_d_n4, assign22380_e27715_d_n5, assign22380_e27715_d_n6, assign22380_e27715_d_n8, assign22380_e27715_d_n10, assign22380_e27715_d_n11, assign22380_e27715_d_n12,) = {
    if (locals.var_mode > 0.0) {
        let assign22380_e27710: f64 = (1.0 - locals.var_qdrat);
        let assign22380_e27711: f64 = (locals.var_sigrat * assign22380_e27710);
        (assign22380_e27711, (locals.var_sigrat_dn0 * assign22380_e27710), (locals.var_sigrat_dn2 * assign22380_e27710), (locals.var_sigrat_dn4 * assign22380_e27710), (locals.var_sigrat_dn5 * assign22380_e27710), (locals.var_sigrat_dn6 * assign22380_e27710), (locals.var_sigrat_dn8 * assign22380_e27710), (locals.var_sigrat_dn10 * assign22380_e27710), (locals.var_sigrat_dn11 * assign22380_e27710), (locals.var_sigrat_dn12 * assign22380_e27710),)
    } else {
        let assign22380_e27714: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign22380_e27714, (locals.var_sigrat_dn0 * locals.var_qdrat), (locals.var_sigrat_dn2 * locals.var_qdrat), (locals.var_sigrat_dn4 * locals.var_qdrat), (locals.var_sigrat_dn5 * locals.var_qdrat), (locals.var_sigrat_dn6 * locals.var_qdrat), (locals.var_sigrat_dn8 * locals.var_qdrat), (locals.var_sigrat_dn10 * locals.var_qdrat), (locals.var_sigrat_dn11 * locals.var_qdrat), (locals.var_sigrat_dn12 * locals.var_qdrat),)
    }
};
        (locals.var_sigrat_s, locals.var_sigrat_s_dn0, locals.var_sigrat_s_dn2, locals.var_sigrat_s_dn4, locals.var_sigrat_s_dn5, locals.var_sigrat_s_dn6, locals.var_sigrat_s_dn8, locals.var_sigrat_s_dn10, locals.var_sigrat_s_dn11, locals.var_sigrat_s_dn12, ) = (assign22380_e27715, assign22380_e27715_d_n0, assign22380_e27715_d_n2, assign22380_e27715_d_n4, assign22380_e27715_d_n5, assign22380_e27715_d_n6, assign22380_e27715_d_n8, assign22380_e27715_d_n10, assign22380_e27715_d_n11, assign22380_e27715_d_n12, );
        locals.var_sigrat_s_rv = 0.0;
        let (assign22390_e27727, assign22390_e27727_d_n0, assign22390_e27727_d_n2, assign22390_e27727_d_n4, assign22390_e27727_d_n5, assign22390_e27727_d_n6, assign22390_e27727_d_n8, assign22390_e27727_d_n10, assign22390_e27727_d_n11, assign22390_e27727_d_n12,) = {
    if (locals.var_mode > 0.0) {
        let assign22390_e27721: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign22390_e27721, (locals.var_sigrat_dn0 * locals.var_qdrat), (locals.var_sigrat_dn2 * locals.var_qdrat), (locals.var_sigrat_dn4 * locals.var_qdrat), (locals.var_sigrat_dn5 * locals.var_qdrat), (locals.var_sigrat_dn6 * locals.var_qdrat), (locals.var_sigrat_dn8 * locals.var_qdrat), (locals.var_sigrat_dn10 * locals.var_qdrat), (locals.var_sigrat_dn11 * locals.var_qdrat), (locals.var_sigrat_dn12 * locals.var_qdrat),)
    } else {
        let assign22390_e27725: f64 = (1.0 - locals.var_qdrat);
        let assign22390_e27726: f64 = (locals.var_sigrat * assign22390_e27725);
        (assign22390_e27726, (locals.var_sigrat_dn0 * assign22390_e27725), (locals.var_sigrat_dn2 * assign22390_e27725), (locals.var_sigrat_dn4 * assign22390_e27725), (locals.var_sigrat_dn5 * assign22390_e27725), (locals.var_sigrat_dn6 * assign22390_e27725), (locals.var_sigrat_dn8 * assign22390_e27725), (locals.var_sigrat_dn10 * assign22390_e27725), (locals.var_sigrat_dn11 * assign22390_e27725), (locals.var_sigrat_dn12 * assign22390_e27725),)
    }
};
        (locals.var_sigrat_d, locals.var_sigrat_d_dn0, locals.var_sigrat_d_dn2, locals.var_sigrat_d_dn4, locals.var_sigrat_d_dn5, locals.var_sigrat_d_dn6, locals.var_sigrat_d_dn8, locals.var_sigrat_d_dn10, locals.var_sigrat_d_dn11, locals.var_sigrat_d_dn12, ) = (assign22390_e27727, assign22390_e27727_d_n0, assign22390_e27727_d_n2, assign22390_e27727_d_n4, assign22390_e27727_d_n5, assign22390_e27727_d_n6, assign22390_e27727_d_n8, assign22390_e27727_d_n10, assign22390_e27727_d_n11, assign22390_e27727_d_n12, );
        locals.var_sigrat_d_rv = 0.0;
        let assign22440_e27734: f64 = if p.p312 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign22440_e27734;
        locals.var_guard380_rv = 0.0;
        if (locals.var_guard380 != 0.0) {
            locals.var_mks_rdrmue = p.p317;
            locals.var_mks_rdrmue_rv = 0.0;
            locals.var_mks_rdrvmax = p.p319;
            locals.var_mks_rdrvmax_rv = 0.0;
            (locals.var_rrdrbb, locals.var_rrdrbb_dn4, ) = (p.p324, 0.0, );
            locals.var_rrdrbb_rv = 0.0;
            locals.var_ldrifte = p.p311;
            locals.var_ldrifte_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22510_e27771: f64 = (p.p33 * (nv12 - nv2));
            (locals.var_vrdr, locals.var_vrdr_dn2, locals.var_vrdr_dn12, ) = (assign22510_e27771, (-p.p33), p.p33, );
            locals.var_vrdr_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22540_e27794: f64 = (locals.var_mks_rdrmue / 10000.0);
            locals.var_mks_rdrmue = assign22540_e27794;
            locals.var_mks_rdrmue_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22550_e27800: f64 = (locals.var_mks_rdrvmax / 100.0);
            locals.var_mks_rdrvmax = assign22550_e27800;
            locals.var_mks_rdrvmax_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22560_e27806: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_tratio, locals.var_tratio_dn4, ) = (assign22560_e27806, (locals.var_ttemp_dn4 / locals.var_uc_tnom), );
            locals.var_tratio_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22570_e27812: f64 = (locals.var_tratio).powf(p.p320);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign22570_e27812, 0.0, 0.0, if 0.0 == 0.0 && ((p.p320) as f64).is_finite() && ((p.p320) as f64).fract() == 0.0 { if p.p320 == 0.0 { 0.0 } else { (p.p320 * ((locals.var_tratio).powf(p.p320 - 1.0) * locals.var_tratio_dn4)) } } else { (assign22570_e27812 * (p.p320 * (locals.var_tratio_dn4 / locals.var_tratio))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22580_e27818: f64 = (locals.var_mks_rdrmue / locals.var_t1);
            (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn4, locals.var_mu0_dn5, locals.var_mu0_dn6, locals.var_mu0_dn8, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, ) = (assign22580_e27818, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), );
            locals.var_mu0_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22590_e27825: f64 = (0.4 * locals.var_tratio);
            let assign22590_e27826: f64 = (1.8 + assign22590_e27825);
            let assign22590_e27829: f64 = (0.1 * locals.var_tratio);
            let assign22590_e27831: f64 = (assign22590_e27829 * locals.var_tratio);
            let assign22590_e27832: f64 = (assign22590_e27826 + assign22590_e27831);
            let assign22590_e27836: f64 = (1.0 - locals.var_tratio);
            let assign22590_e27837: f64 = (p.p321 * assign22590_e27836);
            let assign22590_e27838: f64 = (assign22590_e27832 - assign22590_e27837);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, ) = (assign22590_e27838, 0.0, 0.0, (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign22590_e27829 * locals.var_tratio_dn4))) - (p.p321 * (-locals.var_tratio_dn4))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22600_e27844: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
            (locals.var_vmaxe__blk393, locals.var_vmaxe__blk393_dn0, locals.var_vmaxe__blk393_dn2, locals.var_vmaxe__blk393_dn4, locals.var_vmaxe__blk393_dn5, locals.var_vmaxe__blk393_dn6, locals.var_vmaxe__blk393_dn8, locals.var_vmaxe__blk393_dn10, locals.var_vmaxe__blk393_dn11, locals.var_vmaxe__blk393_dn12, ) = (assign22600_e27844, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), );
            locals.var_vmaxe__blk393_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22610_e27852: f64 = (locals.var_ttemp - locals.var_uc_tnom);
            let assign22610_e27853: f64 = (p.p325 * assign22610_e27852);
            let assign22610_e27854: f64 = (locals.var_rrdrbb + assign22610_e27853);
            (locals.var_rrdrbb, locals.var_rrdrbb_dn4, ) = (assign22610_e27854, (locals.var_rrdrbb_dn4 + (p.p325 * locals.var_ttemp_dn4)), );
            locals.var_rrdrbb_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22620_e27862: f64 = (locals.var_lg).powf(p.p331);
            let assign22620_e27863: f64 = (p.p330 / assign22620_e27862);
            let assign22620_e27864: f64 = (1.0 + assign22620_e27863);
            locals.var_rdrmuele = assign22620_e27864;
            locals.var_rdrmuele_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22630_e27872: f64 = (locals.var_lg).powf(p.p329);
            let assign22630_e27873: f64 = (p.p328 / assign22630_e27872);
            let assign22630_e27874: f64 = (1.0 + assign22630_e27873);
            locals.var_rdrvmaxle = assign22630_e27874;
            locals.var_rdrvmaxle_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22640_e27882: f64 = (locals.var_wg).powf(p.p327);
            let assign22640_e27883: f64 = (p.p326 / assign22640_e27882);
            let assign22640_e27884: f64 = (1.0 + assign22640_e27883);
            locals.var_rdrvmaxwe = assign22640_e27884;
            locals.var_rdrvmaxwe_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22650_e27890: f64 = (locals.var_mu0 * locals.var_rdrmuele);
            (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn4, locals.var_mu0_dn5, locals.var_mu0_dn6, locals.var_mu0_dn8, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, ) = (assign22650_e27890, (locals.var_mu0_dn0 * locals.var_rdrmuele), (locals.var_mu0_dn2 * locals.var_rdrmuele), (locals.var_mu0_dn4 * locals.var_rdrmuele), (locals.var_mu0_dn5 * locals.var_rdrmuele), (locals.var_mu0_dn6 * locals.var_rdrmuele), (locals.var_mu0_dn8 * locals.var_rdrmuele), (locals.var_mu0_dn10 * locals.var_rdrmuele), (locals.var_mu0_dn11 * locals.var_rdrmuele), (locals.var_mu0_dn12 * locals.var_rdrmuele), );
            locals.var_mu0_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22660_e27896: f64 = (locals.var_vmaxe__blk393 * locals.var_rdrvmaxwe);
            let assign22660_e27898: f64 = (assign22660_e27896 * locals.var_rdrvmaxle);
            let assign22660_e27900: f64 = (assign22660_e27898 + 1e-50);
            (locals.var_vmaxe__blk393, locals.var_vmaxe__blk393_dn0, locals.var_vmaxe__blk393_dn2, locals.var_vmaxe__blk393_dn4, locals.var_vmaxe__blk393_dn5, locals.var_vmaxe__blk393_dn6, locals.var_vmaxe__blk393_dn8, locals.var_vmaxe__blk393_dn10, locals.var_vmaxe__blk393_dn11, locals.var_vmaxe__blk393_dn12, ) = (assign22660_e27900, ((locals.var_vmaxe__blk393_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn12 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), );
            locals.var_vmaxe__blk393_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22670_e27906: f64 = (locals.var_vrdr / locals.var_ldrifte);
            (locals.var_edri, locals.var_edri_dn2, locals.var_edri_dn12, ) = (assign22670_e27906, (locals.var_vrdr_dn2 / locals.var_ldrifte), (locals.var_vrdr_dn12 / locals.var_ldrifte), );
            locals.var_edri_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22680_e27912: f64 = (locals.var_mu0 * locals.var_edri);
            (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn4, locals.var_vdri_dn5, locals.var_vdri_dn6, locals.var_vdri_dn8, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn12, ) = (assign22680_e27912, (locals.var_mu0_dn0 * locals.var_edri), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), (locals.var_mu0_dn4 * locals.var_edri), (locals.var_mu0_dn5 * locals.var_edri), (locals.var_mu0_dn6 * locals.var_edri), (locals.var_mu0_dn8 * locals.var_edri), (locals.var_mu0_dn10 * locals.var_edri), (locals.var_mu0_dn11 * locals.var_edri), ((locals.var_mu0_dn12 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn12)), );
            locals.var_vdri_rv = 0.0;
        }
        let assign22690_e27917: f64 = if locals.var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard400 = assign22690_e27917;
        locals.var_guard400_rv = 0.0;
        if ((locals.var_guard380 != 0.0) && (locals.var_guard400 != 0.0)) {
            let assign22700_e27923: f64 = (locals.var_vdri / locals.var_vmaxe__blk393);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign22700_e27923, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn0)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn2)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn4 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn4)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn5 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn5)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn6)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn8 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn8)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn10)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn11)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn12 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn12)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard380 != 0.0) && (locals.var_guard400 == 0.0)) {
            let assign22710_e27931: f64 = (-locals.var_vdri);
            let assign22710_e27933: f64 = (assign22710_e27931 / locals.var_vmaxe__blk393);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign22710_e27933, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn0)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn2)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn4) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn4)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn5) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn5)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn6)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn8) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn8)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn10)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn11)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn12) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn12)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), );
            locals.var_t1_rv = 0.0;
        }
        let assign22720_e27939: f64 = (10.0 * 2.220446049250313e-16);
        let assign22720_e27940: f64 = (1.0 - assign22720_e27939);
        let assign22720_e27947: f64 = (10.0 * 2.220446049250313e-16);
        let assign22720_e27948: f64 = (1.0 + assign22720_e27947);
        let assign22720_e27950: f64 = if ((assign22720_e27940 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign22720_e27948)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign22720_e27950;
        locals.var_guard401_rv = 0.0;
        if ((locals.var_guard380 != 0.0) && (locals.var_guard401 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t3_rv = 0.0;
        }
        let assign22740_e27960: f64 = (10.0 * 2.220446049250313e-16);
        let assign22740_e27961: f64 = (2.0 - assign22740_e27960);
        let assign22740_e27968: f64 = (10.0 * 2.220446049250313e-16);
        let assign22740_e27969: f64 = (2.0 + assign22740_e27968);
        let assign22740_e27971: f64 = if ((assign22740_e27961 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign22740_e27969)) { 1.0 } else { 0.0 };
        locals.var_guard402 = assign22740_e27971;
        locals.var_guard402_rv = 0.0;
        if (((locals.var_guard380 != 0.0) && (locals.var_guard401 == 0.0)) && (locals.var_guard402 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, );
            locals.var_t3_rv = 0.0;
        }
        if (((locals.var_guard380 != 0.0) && (locals.var_guard401 == 0.0)) && (locals.var_guard402 == 0.0)) {
            let assign22760_e27991: f64 = (locals.var_rrdrbb - 1.0);
            let assign22760_e27992: f64 = (locals.var_t1).powf(assign22760_e27991);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, ) = (assign22760_e27992, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn0)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn2)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn2 / locals.var_t1))) }, if locals.var_rrdrbb_dn4 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn4)) } } else { (assign22760_e27992 * ((locals.var_rrdrbb_dn4 * (locals.var_t1).ln()) + (assign22760_e27991 * (locals.var_t1_dn4 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn5)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn6)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn8)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn10)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn11)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn12)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn12 / locals.var_t1))) }, );
            locals.var_t3_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22770_e27998: f64 = (locals.var_t1 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign22770_e27998, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22780_e28004: f64 = (1.0 + locals.var_t2);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, ) = (assign22780_e28004, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, );
            locals.var_t4_rv = 0.0;
        }
        let assign22790_e28010: f64 = (10.0 * 2.220446049250313e-16);
        let assign22790_e28011: f64 = (1.0 - assign22790_e28010);
        let assign22790_e28018: f64 = (10.0 * 2.220446049250313e-16);
        let assign22790_e28019: f64 = (1.0 + assign22790_e28018);
        let assign22790_e28021: f64 = if ((assign22790_e28011 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign22790_e28019)) { 1.0 } else { 0.0 };
        locals.var_guard403 = assign22790_e28021;
        locals.var_guard403_rv = 0.0;
        if ((locals.var_guard380 != 0.0) && (locals.var_guard403 != 0.0)) {
            let assign22800_e28027: f64 = (1.0 / locals.var_t4);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, ) = (assign22800_e28027, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), );
            locals.var_t5_rv = 0.0;
        }
        let assign22810_e28033: f64 = (10.0 * 2.220446049250313e-16);
        let assign22810_e28034: f64 = (2.0 - assign22810_e28033);
        let assign22810_e28041: f64 = (10.0 * 2.220446049250313e-16);
        let assign22810_e28042: f64 = (2.0 + assign22810_e28041);
        let assign22810_e28044: f64 = if ((assign22810_e28034 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign22810_e28042)) { 1.0 } else { 0.0 };
        locals.var_guard404 = assign22810_e28044;
        locals.var_guard404_rv = 0.0;
        if (((locals.var_guard380 != 0.0) && (locals.var_guard403 == 0.0)) && (locals.var_guard404 != 0.0)) {
            let assign22820_e28053: f64 = (locals.var_t4).sqrt();
            let assign22820_e28054: f64 = (1.0 / assign22820_e28053);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, ) = (assign22820_e28054, (-((locals.var_t4_dn0 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn2 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn4 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn5 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn6 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn8 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn10 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn11 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn12 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), );
            locals.var_t5_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_36(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv11 = ctx.node_voltage(nodes[11]);
        if (((locals.var_guard380 != 0.0) && (locals.var_guard403 == 0.0)) && (locals.var_guard404 == 0.0)) {
            let assign22830_e28066: f64 = (-1.0);
            let assign22830_e28068: f64 = (assign22830_e28066 / locals.var_rrdrbb);
            let assign22830_e28070: f64 = (assign22830_e28068 - 1.0);
            let assign22830_e28071: f64 = (locals.var_t4).powf(assign22830_e28070);
            (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, ) = (assign22830_e28071, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn0)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn2)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn2 / locals.var_t4))) }, if (-((assign22830_e28066 * locals.var_rrdrbb_dn4) / (locals.var_rrdrbb * locals.var_rrdrbb))) == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn4)) } } else { (assign22830_e28071 * (((-((assign22830_e28066 * locals.var_rrdrbb_dn4) / (locals.var_rrdrbb * locals.var_rrdrbb))) * (locals.var_t4).ln()) + (assign22830_e28070 * (locals.var_t4_dn4 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn5)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn6)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn8)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn10)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn11)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn12)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn12 / locals.var_t4))) }, );
            locals.var_t6_rv = 0.0;
        }
        if (((locals.var_guard380 != 0.0) && (locals.var_guard403 == 0.0)) && (locals.var_guard404 == 0.0)) {
            let assign22840_e28083: f64 = (locals.var_t4 * locals.var_t6);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, ) = (assign22840_e28083, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard380 != 0.0) {
            let assign22860_e28095: f64 = (1.6021918e-19 / locals.var_ldrifte);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign22860_e28095, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        let assign22980_e28171: f64 = if p.p313 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign22980_e28171;
        locals.var_guard407_rv = 0.0;
        if (locals.var_guard407 != 0.0) {
            locals.var_mks_rdrmue__blk411 = p.p316;
            locals.var_mks_rdrmue__blk411_rv = 0.0;
            locals.var_mks_rdrvmax__blk412 = p.p318;
            locals.var_mks_rdrvmax__blk412_rv = 0.0;
            (locals.var_rrdrbb__blk413, locals.var_rrdrbb__blk413_dn4, ) = (p.p323, 0.0, );
            locals.var_rrdrbb__blk413_rv = 0.0;
            locals.var_ldrifte__blk417 = p.p310;
            locals.var_ldrifte__blk417_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23050_e28208: f64 = (p.p33 * (nv0 - nv11));
            (locals.var_vrdr__blk415, locals.var_vrdr__blk415_dn0, locals.var_vrdr__blk415_dn11, ) = (assign23050_e28208, p.p33, (-p.p33), );
            locals.var_vrdr__blk415_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23080_e28231: f64 = (locals.var_mks_rdrmue__blk411 / 10000.0);
            locals.var_mks_rdrmue__blk411 = assign23080_e28231;
            locals.var_mks_rdrmue__blk411_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23090_e28237: f64 = (locals.var_mks_rdrvmax__blk412 / 100.0);
            locals.var_mks_rdrvmax__blk412 = assign23090_e28237;
            locals.var_mks_rdrvmax__blk412_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23100_e28243: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_tratio__blk416, locals.var_tratio__blk416_dn4, ) = (assign23100_e28243, (locals.var_ttemp_dn4 / locals.var_uc_tnom), );
            locals.var_tratio__blk416_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23110_e28249: f64 = (locals.var_tratio__blk416).powf(p.p320);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign23110_e28249, 0.0, 0.0, if 0.0 == 0.0 && ((p.p320) as f64).is_finite() && ((p.p320) as f64).fract() == 0.0 { if p.p320 == 0.0 { 0.0 } else { (p.p320 * ((locals.var_tratio__blk416).powf(p.p320 - 1.0) * locals.var_tratio__blk416_dn4)) } } else { (assign23110_e28249 * (p.p320 * (locals.var_tratio__blk416_dn4 / locals.var_tratio__blk416))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23120_e28255: f64 = (locals.var_mks_rdrmue__blk411 / locals.var_t1);
            (locals.var_mu0__blk419, locals.var_mu0__blk419_dn0, locals.var_mu0__blk419_dn2, locals.var_mu0__blk419_dn4, locals.var_mu0__blk419_dn5, locals.var_mu0__blk419_dn6, locals.var_mu0__blk419_dn8, locals.var_mu0__blk419_dn10, locals.var_mu0__blk419_dn11, locals.var_mu0__blk419_dn12, ) = (assign23120_e28255, (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), );
            locals.var_mu0__blk419_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23130_e28262: f64 = (0.4 * locals.var_tratio__blk416);
            let assign23130_e28263: f64 = (1.8 + assign23130_e28262);
            let assign23130_e28266: f64 = (0.1 * locals.var_tratio__blk416);
            let assign23130_e28268: f64 = (assign23130_e28266 * locals.var_tratio__blk416);
            let assign23130_e28269: f64 = (assign23130_e28263 + assign23130_e28268);
            let assign23130_e28273: f64 = (1.0 - locals.var_tratio__blk416);
            let assign23130_e28274: f64 = (p.p321 * assign23130_e28273);
            let assign23130_e28275: f64 = (assign23130_e28269 - assign23130_e28274);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, ) = (assign23130_e28275, 0.0, 0.0, (((0.4 * locals.var_tratio__blk416_dn4) + (((0.1 * locals.var_tratio__blk416_dn4) * locals.var_tratio__blk416) + (assign23130_e28266 * locals.var_tratio__blk416_dn4))) - (p.p321 * (-locals.var_tratio__blk416_dn4))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23140_e28281: f64 = (locals.var_mks_rdrvmax__blk412 / locals.var_t0);
            (locals.var_vmaxe__blk420, locals.var_vmaxe__blk420_dn0, locals.var_vmaxe__blk420_dn2, locals.var_vmaxe__blk420_dn4, locals.var_vmaxe__blk420_dn5, locals.var_vmaxe__blk420_dn6, locals.var_vmaxe__blk420_dn8, locals.var_vmaxe__blk420_dn10, locals.var_vmaxe__blk420_dn11, locals.var_vmaxe__blk420_dn12, ) = (assign23140_e28281, (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), );
            locals.var_vmaxe__blk420_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23150_e28289: f64 = (locals.var_ttemp - locals.var_uc_tnom);
            let assign23150_e28290: f64 = (p.p325 * assign23150_e28289);
            let assign23150_e28291: f64 = (locals.var_rrdrbb__blk413 + assign23150_e28290);
            (locals.var_rrdrbb__blk413, locals.var_rrdrbb__blk413_dn4, ) = (assign23150_e28291, (locals.var_rrdrbb__blk413_dn4 + (p.p325 * locals.var_ttemp_dn4)), );
            locals.var_rrdrbb__blk413_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23160_e28299: f64 = (locals.var_lg).powf(p.p331);
            let assign23160_e28300: f64 = (p.p330 / assign23160_e28299);
            let assign23160_e28301: f64 = (1.0 + assign23160_e28300);
            locals.var_rdrmuele__blk408 = assign23160_e28301;
            locals.var_rdrmuele__blk408_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23170_e28309: f64 = (locals.var_lg).powf(p.p329);
            let assign23170_e28310: f64 = (p.p328 / assign23170_e28309);
            let assign23170_e28311: f64 = (1.0 + assign23170_e28310);
            locals.var_rdrvmaxle__blk410 = assign23170_e28311;
            locals.var_rdrvmaxle__blk410_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23180_e28319: f64 = (locals.var_wg).powf(p.p327);
            let assign23180_e28320: f64 = (p.p326 / assign23180_e28319);
            let assign23180_e28321: f64 = (1.0 + assign23180_e28320);
            locals.var_rdrvmaxwe__blk409 = assign23180_e28321;
            locals.var_rdrvmaxwe__blk409_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23190_e28327: f64 = (locals.var_mu0__blk419 * locals.var_rdrmuele__blk408);
            (locals.var_mu0__blk419, locals.var_mu0__blk419_dn0, locals.var_mu0__blk419_dn2, locals.var_mu0__blk419_dn4, locals.var_mu0__blk419_dn5, locals.var_mu0__blk419_dn6, locals.var_mu0__blk419_dn8, locals.var_mu0__blk419_dn10, locals.var_mu0__blk419_dn11, locals.var_mu0__blk419_dn12, ) = (assign23190_e28327, (locals.var_mu0__blk419_dn0 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn2 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn4 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn5 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn6 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn8 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn10 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn11 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn12 * locals.var_rdrmuele__blk408), );
            locals.var_mu0__blk419_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23200_e28333: f64 = (locals.var_vmaxe__blk420 * locals.var_rdrvmaxwe__blk409);
            let assign23200_e28335: f64 = (assign23200_e28333 * locals.var_rdrvmaxle__blk410);
            let assign23200_e28337: f64 = (assign23200_e28335 + 1e-50);
            (locals.var_vmaxe__blk420, locals.var_vmaxe__blk420_dn0, locals.var_vmaxe__blk420_dn2, locals.var_vmaxe__blk420_dn4, locals.var_vmaxe__blk420_dn5, locals.var_vmaxe__blk420_dn6, locals.var_vmaxe__blk420_dn8, locals.var_vmaxe__blk420_dn10, locals.var_vmaxe__blk420_dn11, locals.var_vmaxe__blk420_dn12, ) = (assign23200_e28337, ((locals.var_vmaxe__blk420_dn0 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn2 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn4 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn5 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn6 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn8 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn10 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn11 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn12 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), );
            locals.var_vmaxe__blk420_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23210_e28343: f64 = (locals.var_vrdr__blk415 / locals.var_ldrifte__blk417);
            (locals.var_edri__blk421, locals.var_edri__blk421_dn0, locals.var_edri__blk421_dn11, ) = (assign23210_e28343, (locals.var_vrdr__blk415_dn0 / locals.var_ldrifte__blk417), (locals.var_vrdr__blk415_dn11 / locals.var_ldrifte__blk417), );
            locals.var_edri__blk421_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23220_e28349: f64 = (locals.var_mu0__blk419 * locals.var_edri__blk421);
            (locals.var_vdri__blk422, locals.var_vdri__blk422_dn0, locals.var_vdri__blk422_dn2, locals.var_vdri__blk422_dn4, locals.var_vdri__blk422_dn5, locals.var_vdri__blk422_dn6, locals.var_vdri__blk422_dn8, locals.var_vdri__blk422_dn10, locals.var_vdri__blk422_dn11, locals.var_vdri__blk422_dn12, ) = (assign23220_e28349, ((locals.var_mu0__blk419_dn0 * locals.var_edri__blk421) + (locals.var_mu0__blk419 * locals.var_edri__blk421_dn0)), (locals.var_mu0__blk419_dn2 * locals.var_edri__blk421), (locals.var_mu0__blk419_dn4 * locals.var_edri__blk421), (locals.var_mu0__blk419_dn5 * locals.var_edri__blk421), (locals.var_mu0__blk419_dn6 * locals.var_edri__blk421), (locals.var_mu0__blk419_dn8 * locals.var_edri__blk421), (locals.var_mu0__blk419_dn10 * locals.var_edri__blk421), ((locals.var_mu0__blk419_dn11 * locals.var_edri__blk421) + (locals.var_mu0__blk419 * locals.var_edri__blk421_dn11)), (locals.var_mu0__blk419_dn12 * locals.var_edri__blk421), );
            locals.var_vdri__blk422_rv = 0.0;
        }
        let assign23230_e28354: f64 = if locals.var_vrdr__blk415 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign23230_e28354;
        locals.var_guard427_rv = 0.0;
        if ((locals.var_guard407 != 0.0) && (locals.var_guard427 != 0.0)) {
            let assign23240_e28360: f64 = (locals.var_vdri__blk422 / locals.var_vmaxe__blk420);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign23240_e28360, (((locals.var_vdri__blk422_dn0 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn0)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn2 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn2)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn4 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn4)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn5 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn5)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn6 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn6)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn8 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn8)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn10 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn10)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn11 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn11)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn12 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn12)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard407 != 0.0) && (locals.var_guard427 == 0.0)) {
            let assign23250_e28368: f64 = (-locals.var_vdri__blk422);
            let assign23250_e28370: f64 = (assign23250_e28368 / locals.var_vmaxe__blk420);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign23250_e28370, ((((-locals.var_vdri__blk422_dn0) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn0)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn2) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn2)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn4) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn4)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn5) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn5)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn6) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn6)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn8) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn8)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn10) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn10)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn11) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn11)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn12) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn12)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), );
            locals.var_t1_rv = 0.0;
        }
        let assign23260_e28376: f64 = (10.0 * 2.220446049250313e-16);
        let assign23260_e28377: f64 = (1.0 - assign23260_e28376);
        let assign23260_e28384: f64 = (10.0 * 2.220446049250313e-16);
        let assign23260_e28385: f64 = (1.0 + assign23260_e28384);
        let assign23260_e28387: f64 = if ((assign23260_e28377 <= locals.var_rrdrbb__blk413) && (locals.var_rrdrbb__blk413 <= assign23260_e28385)) { 1.0 } else { 0.0 };
        locals.var_guard428 = assign23260_e28387;
        locals.var_guard428_rv = 0.0;
        if ((locals.var_guard407 != 0.0) && (locals.var_guard428 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t3_rv = 0.0;
        }
        let assign23280_e28397: f64 = (10.0 * 2.220446049250313e-16);
        let assign23280_e28398: f64 = (2.0 - assign23280_e28397);
        let assign23280_e28405: f64 = (10.0 * 2.220446049250313e-16);
        let assign23280_e28406: f64 = (2.0 + assign23280_e28405);
        let assign23280_e28408: f64 = if ((assign23280_e28398 <= locals.var_rrdrbb__blk413) && (locals.var_rrdrbb__blk413 <= assign23280_e28406)) { 1.0 } else { 0.0 };
        locals.var_guard429 = assign23280_e28408;
        locals.var_guard429_rv = 0.0;
        if (((locals.var_guard407 != 0.0) && (locals.var_guard428 == 0.0)) && (locals.var_guard429 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, );
            locals.var_t3_rv = 0.0;
        }
        if (((locals.var_guard407 != 0.0) && (locals.var_guard428 == 0.0)) && (locals.var_guard429 == 0.0)) {
            let assign23300_e28428: f64 = (locals.var_rrdrbb__blk413 - 1.0);
            let assign23300_e28429: f64 = (locals.var_t1).powf(assign23300_e28428);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, ) = (assign23300_e28429, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn0)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn2)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn2 / locals.var_t1))) }, if locals.var_rrdrbb__blk413_dn4 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn4)) } } else { (assign23300_e28429 * ((locals.var_rrdrbb__blk413_dn4 * (locals.var_t1).ln()) + (assign23300_e28428 * (locals.var_t1_dn4 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn5)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn6)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn8)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn10)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn11)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn12)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn12 / locals.var_t1))) }, );
            locals.var_t3_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23310_e28435: f64 = (locals.var_t1 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, ) = (assign23310_e28435, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23320_e28441: f64 = (1.0 + locals.var_t2);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, ) = (assign23320_e28441, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, );
            locals.var_t4_rv = 0.0;
        }
        let assign23330_e28447: f64 = (10.0 * 2.220446049250313e-16);
        let assign23330_e28448: f64 = (1.0 - assign23330_e28447);
        let assign23330_e28455: f64 = (10.0 * 2.220446049250313e-16);
        let assign23330_e28456: f64 = (1.0 + assign23330_e28455);
        let assign23330_e28458: f64 = if ((assign23330_e28448 <= locals.var_rrdrbb__blk413) && (locals.var_rrdrbb__blk413 <= assign23330_e28456)) { 1.0 } else { 0.0 };
        locals.var_guard430 = assign23330_e28458;
        locals.var_guard430_rv = 0.0;
        if ((locals.var_guard407 != 0.0) && (locals.var_guard430 != 0.0)) {
            let assign23340_e28464: f64 = (1.0 / locals.var_t4);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, ) = (assign23340_e28464, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), );
            locals.var_t5_rv = 0.0;
        }
        let assign23350_e28470: f64 = (10.0 * 2.220446049250313e-16);
        let assign23350_e28471: f64 = (2.0 - assign23350_e28470);
        let assign23350_e28478: f64 = (10.0 * 2.220446049250313e-16);
        let assign23350_e28479: f64 = (2.0 + assign23350_e28478);
        let assign23350_e28481: f64 = if ((assign23350_e28471 <= locals.var_rrdrbb__blk413) && (locals.var_rrdrbb__blk413 <= assign23350_e28479)) { 1.0 } else { 0.0 };
        locals.var_guard431 = assign23350_e28481;
        locals.var_guard431_rv = 0.0;
        if (((locals.var_guard407 != 0.0) && (locals.var_guard430 == 0.0)) && (locals.var_guard431 != 0.0)) {
            let assign23360_e28490: f64 = (locals.var_t4).sqrt();
            let assign23360_e28491: f64 = (1.0 / assign23360_e28490);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, ) = (assign23360_e28491, (-((locals.var_t4_dn0 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn2 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn4 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn5 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn6 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn8 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn10 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn11 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn12 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), );
            locals.var_t5_rv = 0.0;
        }
        if (((locals.var_guard407 != 0.0) && (locals.var_guard430 == 0.0)) && (locals.var_guard431 == 0.0)) {
            let assign23370_e28503: f64 = (-1.0);
            let assign23370_e28505: f64 = (assign23370_e28503 / locals.var_rrdrbb__blk413);
            let assign23370_e28507: f64 = (assign23370_e28505 - 1.0);
            let assign23370_e28508: f64 = (locals.var_t4).powf(assign23370_e28507);
            (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, ) = (assign23370_e28508, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn0)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn2)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn2 / locals.var_t4))) }, if (-((assign23370_e28503 * locals.var_rrdrbb__blk413_dn4) / (locals.var_rrdrbb__blk413 * locals.var_rrdrbb__blk413))) == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn4)) } } else { (assign23370_e28508 * (((-((assign23370_e28503 * locals.var_rrdrbb__blk413_dn4) / (locals.var_rrdrbb__blk413 * locals.var_rrdrbb__blk413))) * (locals.var_t4).ln()) + (assign23370_e28507 * (locals.var_t4_dn4 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn5)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn6)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn8)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn10)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn11)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn12)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn12 / locals.var_t4))) }, );
            locals.var_t6_rv = 0.0;
        }
        if (((locals.var_guard407 != 0.0) && (locals.var_guard430 == 0.0)) && (locals.var_guard431 == 0.0)) {
            let assign23380_e28520: f64 = (locals.var_t4 * locals.var_t6);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, ) = (assign23380_e28520, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard407 != 0.0) {
            let assign23400_e28532: f64 = (1.6021918e-19 / locals.var_ldrifte__blk417);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (assign23400_e28532, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        let assign23520_e28608: f64 = if locals.var_tau < 1e-18 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign23520_e28608;
        locals.var_guard434_rv = 0.0;
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard434 != 0.0)) {
            (locals.var_tau, locals.var_tau_dn0, locals.var_tau_dn2, locals.var_tau_dn4, locals.var_tau_dn5, locals.var_tau_dn6, locals.var_tau_dn8, locals.var_tau_dn10, locals.var_tau_dn11, locals.var_tau_dn12, ) = (1e-18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_tau_rv = 0.0;
        }
        let assign23540_e28617: f64 = if locals.var_taub < 1e-18 { 1.0 } else { 0.0 };
        locals.var_guard435 = assign23540_e28617;
        locals.var_guard435_rv = 0.0;
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard435 != 0.0)) {
            (locals.var_taub, locals.var_taub_dn0, locals.var_taub_dn2, locals.var_taub_dn4, locals.var_taub_dn5, locals.var_taub_dn6, locals.var_taub_dn8, locals.var_taub_dn10, locals.var_taub_dn11, locals.var_taub_dn12, ) = (1e-18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_taub_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign23560_e28627: f64 = (locals.var_qi_nqs - locals.var_qi_qs);
            let assign23560_e28629: f64 = (assign23560_e28627 / locals.var_tau);
            (locals.var_iqi_nqs, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn4, locals.var_iqi_nqs_dn5, locals.var_iqi_nqs_dn6, locals.var_iqi_nqs_dn8, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12, ) = (assign23560_e28629, ((((-locals.var_qi_qs_dn0) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn0)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn2) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn2)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn4) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn4)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn5) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn5)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn6) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn6)) / (locals.var_tau * locals.var_tau)), ((((locals.var_qi_nqs_dn8 - locals.var_qi_qs_dn8) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn8)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn10) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn10)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn11) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn11)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn12) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn12)) / (locals.var_tau * locals.var_tau)), );
            locals.var_iqi_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign23570_e28635: f64 = (locals.var_qb_nqs - locals.var_qb_qs);
            let assign23570_e28637: f64 = (assign23570_e28635 / locals.var_taub);
            (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn4, locals.var_iqb_nqs_dn5, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn8, locals.var_iqb_nqs_dn9, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, ) = (assign23570_e28637, ((((-locals.var_qb_qs_dn0) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn0)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn2) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn2)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn4) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn4)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn5) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn5)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn6) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn6)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn8) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn8)) / (locals.var_taub * locals.var_taub)), (locals.var_qb_nqs_dn9 / locals.var_taub), ((((-locals.var_qb_qs_dn10) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn10)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn11) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn11)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn12) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn12)) / (locals.var_taub * locals.var_taub)), );
            locals.var_iqb_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign23580_e28642: f64 = (-locals.var_qi_nqs);
            let assign23580_e28644: f64 = (assign23580_e28642 - locals.var_qb_nqs);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn8, locals.var_qg_nqs_dn9, ) = (assign23580_e28644, (-locals.var_qi_nqs_dn8), (-locals.var_qb_nqs_dn9), );
            locals.var_qg_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign23590_e28650: f64 = (locals.var_qi_nqs * locals.var_qdrat);
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, ) = (assign23590_e28650, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_qi_nqs_dn8 * locals.var_qdrat), 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs != 0.0) {
            let assign23600_e28657: f64 = (1.0 - locals.var_qdrat);
            let assign23600_e28658: f64 = (locals.var_qi_nqs * assign23600_e28657);
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, ) = (assign23600_e28658, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_qi_nqs_dn8 * assign23600_e28657), 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
        }
        if (locals.var_flg_nqs == 0.0) {
            (locals.var_iqi_nqs, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn4, locals.var_iqi_nqs_dn5, locals.var_iqi_nqs_dn6, locals.var_iqi_nqs_dn8, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_iqi_nqs_rv = 0.0;
            (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn4, locals.var_iqb_nqs_dn5, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn8, locals.var_iqb_nqs_dn9, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_iqb_nqs_rv = 0.0;
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn8, locals.var_qg_nqs_dn9, ) = (0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qb_nqs, locals.var_qb_nqs_dn9, ) = (0.0, 0.0, );
            locals.var_qb_nqs_rv = 0.0;
        }
        let assign23690_e28695: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard436 = assign23690_e28695;
        locals.var_guard436_rv = 0.0;
        if (locals.var_guard436 != 0.0) {
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn8, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, ) = (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn8, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, );
            locals.var_ids_rv = 0.0;
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn8, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, ) = (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn4, locals.var_isube_dn5, locals.var_isube_dn6, locals.var_isube_dn8, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, );
            locals.var_isub_rv = 0.0;
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn8, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, ) = (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn8, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, );
            locals.var_qg_rv = 0.0;
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn8, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, ) = (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, );
            locals.var_qd_rv = 0.0;
        }
        if (locals.var_guard436 != 0.0) {
            let assign23750_e28719: f64 = (locals.var_qge + locals.var_qde);
            let assign23750_e28721: f64 = (assign23750_e28719 + locals.var_qse);
            let assign23750_e28722: f64 = (-assign23750_e28721);
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn8, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, ) = (assign23750_e28722, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), );
            locals.var_qbe_rv = 0.0;
        }
        if (locals.var_guard436 != 0.0) {
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn8, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, ) = (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn8, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, );
            locals.var_qb_rv = 0.0;
        }
        if (locals.var_guard436 == 0.0) {
            let assign23770_e28732: f64 = (-locals.var_idse);
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn8, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, ) = (assign23770_e28732, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn4), (-locals.var_idse_dn5), (-locals.var_idse_dn6), (-locals.var_idse_dn8), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12), );
            locals.var_ids_rv = 0.0;
        }
        if (locals.var_guard436 == 0.0) {
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn8, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isub_rv = 0.0;
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn8, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, ) = (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn8, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, );
            locals.var_qg_rv = 0.0;
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn8, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, ) = (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn8, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, );
            locals.var_qd_rv = 0.0;
        }
        if (locals.var_guard436 == 0.0) {
            let assign23820_e28759: f64 = (locals.var_qge + locals.var_qde);
            let assign23820_e28761: f64 = (assign23820_e28759 + locals.var_qse);
            let assign23820_e28762: f64 = (-assign23820_e28761);
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn8, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, ) = (assign23820_e28762, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), );
            locals.var_qbe_rv = 0.0;
        }
        if (locals.var_guard436 == 0.0) {
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn8, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, ) = (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn8, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, );
            locals.var_qb_rv = 0.0;
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn8, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, ) = (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, );
            locals.var_qse_rv = 0.0;
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, ) = (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn8, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_guard436 == 0.0) && (locals.var_flg_nqs != 0.0)) {
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, ) = (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, );
            locals.var_t1_rv = 0.0;
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, ) = (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, );
            locals.var_qs_nqs_rv = 0.0;
        }
        let assign23890_e28805: f64 = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard437 = assign23890_e28805;
        locals.var_guard437_rv = 0.0;
        if (locals.var_guard437 != 0.0) {
            let assign23900_e28809: f64 = (locals.var_idse * locals.var_vds);
            (locals.var_rpower, locals.var_rpower_dn0, locals.var_rpower_dn2, locals.var_rpower_dn4, locals.var_rpower_dn5, locals.var_rpower_dn6, locals.var_rpower_dn8, locals.var_rpower_dn10, locals.var_rpower_dn11, locals.var_rpower_dn12, ) = (assign23900_e28809, ((locals.var_idse_dn0 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn0)), ((locals.var_idse_dn2 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn2)), ((locals.var_idse_dn4 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn4)), ((locals.var_idse_dn5 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn5)), ((locals.var_idse_dn6 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn6)), ((locals.var_idse_dn8 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn8)), ((locals.var_idse_dn10 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn10)), ((locals.var_idse_dn11 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn11)), ((locals.var_idse_dn12 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn12)), );
            locals.var_rpower_rv = 0.0;
        }
        if (locals.var_guard437 != 0.0) {
            (locals.var_cthe, locals.var_cthe_dn0, locals.var_cthe_dn2, locals.var_cthe_dn4, locals.var_cthe_dn5, locals.var_cthe_dn6, locals.var_cthe_dn8, locals.var_cthe_dn10, locals.var_cthe_dn11, locals.var_cthe_dn12, ) = (locals.var_cth, locals.var_cth_dn0, locals.var_cth_dn2, locals.var_cth_dn4, locals.var_cth_dn5, locals.var_cth_dn6, locals.var_cth_dn8, locals.var_cth_dn10, locals.var_cth_dn11, locals.var_cth_dn12, );
            locals.var_cthe_rv = 0.0;
        }
        if (locals.var_guard437 != 0.0) {
            let assign23920_e28819: f64 = (1.0 / locals.var_rth);
            (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn8, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn12, ) = (assign23920_e28819, (-(locals.var_rth_dn0 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn2 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn4 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn5 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn6 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn8 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn10 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn11 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn12 / (locals.var_rth * locals.var_rth))), );
            locals.var_gth_rv = 0.0;
        }
        if (locals.var_guard437 == 0.0) {
            (locals.var_rpower, locals.var_rpower_dn0, locals.var_rpower_dn2, locals.var_rpower_dn4, locals.var_rpower_dn5, locals.var_rpower_dn6, locals.var_rpower_dn8, locals.var_rpower_dn10, locals.var_rpower_dn11, locals.var_rpower_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_rpower_rv = 0.0;
            (locals.var_cthe, locals.var_cthe_dn0, locals.var_cthe_dn2, locals.var_cthe_dn4, locals.var_cthe_dn5, locals.var_cthe_dn6, locals.var_cthe_dn8, locals.var_cthe_dn10, locals.var_cthe_dn11, locals.var_cthe_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cthe_rv = 0.0;
            (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn8, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn12, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_gth_rv = 0.0;
        }
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn8, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, ) = (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn8, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, );
        locals.var_idse_rv = 0.0;
        let assign24160_e28890: f64 = locals.var_qge_dn11;
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn8, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, ) = (assign24160_e28890, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgdbd_rv = 0.0;
        let assign24170_e28893: f64 = (p.p33 * locals.var_cgdbd);
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn8, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12, ) = (assign24170_e28893, (p.p33 * locals.var_cgdbd_dn0), (p.p33 * locals.var_cgdbd_dn2), (p.p33 * locals.var_cgdbd_dn4), (p.p33 * locals.var_cgdbd_dn5), (p.p33 * locals.var_cgdbd_dn6), (p.p33 * locals.var_cgdbd_dn8), (p.p33 * locals.var_cgdbd_dn10), (p.p33 * locals.var_cgdbd_dn11), (p.p33 * locals.var_cgdbd_dn12), );
        locals.var_cgdbd_rv = 0.0;
        let assign24180_e28896: f64 = locals.var_qge_dn12;
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn8, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, ) = (assign24180_e28896, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cgsbd_rv = 0.0;
        let assign24190_e28899: f64 = (p.p33 * locals.var_cgsbd);
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn8, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12, ) = (assign24190_e28899, (p.p33 * locals.var_cgsbd_dn0), (p.p33 * locals.var_cgsbd_dn2), (p.p33 * locals.var_cgsbd_dn4), (p.p33 * locals.var_cgsbd_dn5), (p.p33 * locals.var_cgsbd_dn6), (p.p33 * locals.var_cgsbd_dn8), (p.p33 * locals.var_cgsbd_dn10), (p.p33 * locals.var_cgsbd_dn11), (p.p33 * locals.var_cgsbd_dn12), );
        locals.var_cgsbd_rv = 0.0;
        let assign24500_e28994: f64 = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard443 = assign24500_e28994;
        locals.var_guard443_rv = 0.0;
        let assign24510_e28999: f64 = if (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard444 = assign24510_e28999;
        locals.var_guard444_rv = 0.0;
    }
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq0_e342: f64 = (p.p33 * locals.var_ids);
        let eq0_e342_d_n0: f64 = (p.p33 * locals.var_ids_dn0);
        let eq0_e342_d_n2: f64 = (p.p33 * locals.var_ids_dn2);
        let eq0_e342_d_n4: f64 = (p.p33 * locals.var_ids_dn4);
        let eq0_e342_d_n5: f64 = (p.p33 * locals.var_ids_dn5);
        let eq0_e342_d_n6: f64 = (p.p33 * locals.var_ids_dn6);
        let eq0_e342_d_n8: f64 = (p.p33 * locals.var_ids_dn8);
        let eq0_e342_d_n10: f64 = (p.p33 * locals.var_ids_dn10);
        let eq0_e342_d_n11: f64 = (p.p33 * locals.var_ids_dn11);
        let eq0_e342_d_n12: f64 = (p.p33 * locals.var_ids_dn12);
        let eq0_value: f64 = eq0_e342;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq0_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq0_e342_d_n0), multiplicity * (eq0_e342_d_n2), multiplicity * (eq0_e342_d_n4), multiplicity * (eq0_e342_d_n5), multiplicity * (eq0_e342_d_n6), multiplicity * (eq0_e342_d_n8), multiplicity * (eq0_e342_d_n10), multiplicity * (eq0_e342_d_n11), multiplicity * (eq0_e342_d_n12)],
            [],
            [],
            1.0,
        );
        let eq1_e346: f64 = (locals.var_igidl + locals.var_isub);
        let eq1_e346_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq1_e346_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq1_e346_d_n4: f64 = (locals.var_igidl_dn4 + locals.var_isub_dn4);
        let eq1_e346_d_n5: f64 = (locals.var_igidl_dn5 + locals.var_isub_dn5);
        let eq1_e346_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq1_e346_d_n8: f64 = (locals.var_igidl_dn8 + locals.var_isub_dn8);
        let eq1_e346_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq1_e346_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq1_e346_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq1_e347: f64 = (p.p33 * eq1_e346);
        let eq1_e347_d_n0: f64 = (p.p33 * eq1_e346_d_n0);
        let eq1_e347_d_n2: f64 = (p.p33 * eq1_e346_d_n2);
        let eq1_e347_d_n4: f64 = (p.p33 * eq1_e346_d_n4);
        let eq1_e347_d_n5: f64 = (p.p33 * eq1_e346_d_n5);
        let eq1_e347_d_n6: f64 = (p.p33 * eq1_e346_d_n6);
        let eq1_e347_d_n8: f64 = (p.p33 * eq1_e346_d_n8);
        let eq1_e347_d_n10: f64 = (p.p33 * eq1_e346_d_n10);
        let eq1_e347_d_n11: f64 = (p.p33 * eq1_e346_d_n11);
        let eq1_e347_d_n12: f64 = (p.p33 * eq1_e346_d_n12);
        let eq1_value: f64 = eq1_e347;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq1_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq1_e347_d_n0), multiplicity * (eq1_e347_d_n2), multiplicity * (eq1_e347_d_n4), multiplicity * (eq1_e347_d_n5), multiplicity * (eq1_e347_d_n6), multiplicity * (eq1_e347_d_n8), multiplicity * (eq1_e347_d_n10), multiplicity * (eq1_e347_d_n11), multiplicity * (eq1_e347_d_n12)],
            [],
            [],
            1.0,
        );
        let eq2_e351: f64 = (locals.var_igisl + locals.var_isubs);
        let eq2_e351_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq2_e351_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq2_e351_d_n4: f64 = (locals.var_igisl_dn4 + locals.var_isubs_dn4);
        let eq2_e351_d_n5: f64 = (locals.var_igisl_dn5 + locals.var_isubs_dn5);
        let eq2_e351_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq2_e351_d_n8: f64 = (locals.var_igisl_dn8 + locals.var_isubs_dn8);
        let eq2_e351_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq2_e351_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq2_e351_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq2_e352: f64 = (p.p33 * eq2_e351);
        let eq2_e352_d_n0: f64 = (p.p33 * eq2_e351_d_n0);
        let eq2_e352_d_n2: f64 = (p.p33 * eq2_e351_d_n2);
        let eq2_e352_d_n4: f64 = (p.p33 * eq2_e351_d_n4);
        let eq2_e352_d_n5: f64 = (p.p33 * eq2_e351_d_n5);
        let eq2_e352_d_n6: f64 = (p.p33 * eq2_e351_d_n6);
        let eq2_e352_d_n8: f64 = (p.p33 * eq2_e351_d_n8);
        let eq2_e352_d_n10: f64 = (p.p33 * eq2_e351_d_n10);
        let eq2_e352_d_n11: f64 = (p.p33 * eq2_e351_d_n11);
        let eq2_e352_d_n12: f64 = (p.p33 * eq2_e351_d_n12);
        let eq2_value: f64 = eq2_e352;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(12),
            Some(11),
            multiplicity * (eq2_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq2_e352_d_n0), multiplicity * (eq2_e352_d_n2), multiplicity * (eq2_e352_d_n4), multiplicity * (eq2_e352_d_n5), multiplicity * (eq2_e352_d_n6), multiplicity * (eq2_e352_d_n8), multiplicity * (eq2_e352_d_n10), multiplicity * (eq2_e352_d_n11), multiplicity * (eq2_e352_d_n12)],
            [],
            [],
            1.0,
        );
        let eq3_e355: f64 = (p.p33 * locals.var_igs);
        let eq3_e355_d_n0: f64 = (p.p33 * locals.var_igs_dn0);
        let eq3_e355_d_n2: f64 = (p.p33 * locals.var_igs_dn2);
        let eq3_e355_d_n4: f64 = (p.p33 * locals.var_igs_dn4);
        let eq3_e355_d_n5: f64 = (p.p33 * locals.var_igs_dn5);
        let eq3_e355_d_n6: f64 = (p.p33 * locals.var_igs_dn6);
        let eq3_e355_d_n8: f64 = (p.p33 * locals.var_igs_dn8);
        let eq3_e355_d_n10: f64 = (p.p33 * locals.var_igs_dn10);
        let eq3_e355_d_n11: f64 = (p.p33 * locals.var_igs_dn11);
        let eq3_e355_d_n12: f64 = (p.p33 * locals.var_igs_dn12);
        let eq3_value: f64 = eq3_e355;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq3_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq3_e355_d_n0), multiplicity * (eq3_e355_d_n2), multiplicity * (eq3_e355_d_n4), multiplicity * (eq3_e355_d_n5), multiplicity * (eq3_e355_d_n6), multiplicity * (eq3_e355_d_n8), multiplicity * (eq3_e355_d_n10), multiplicity * (eq3_e355_d_n11), multiplicity * (eq3_e355_d_n12)],
            [],
            [],
            1.0,
        );
        let eq4_e358: f64 = (p.p33 * locals.var_igd);
        let eq4_e358_d_n0: f64 = (p.p33 * locals.var_igd_dn0);
        let eq4_e358_d_n2: f64 = (p.p33 * locals.var_igd_dn2);
        let eq4_e358_d_n4: f64 = (p.p33 * locals.var_igd_dn4);
        let eq4_e358_d_n5: f64 = (p.p33 * locals.var_igd_dn5);
        let eq4_e358_d_n6: f64 = (p.p33 * locals.var_igd_dn6);
        let eq4_e358_d_n8: f64 = (p.p33 * locals.var_igd_dn8);
        let eq4_e358_d_n10: f64 = (p.p33 * locals.var_igd_dn10);
        let eq4_e358_d_n11: f64 = (p.p33 * locals.var_igd_dn11);
        let eq4_e358_d_n12: f64 = (p.p33 * locals.var_igd_dn12);
        let eq4_value: f64 = eq4_e358;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(11),
            multiplicity * (eq4_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq4_e358_d_n0), multiplicity * (eq4_e358_d_n2), multiplicity * (eq4_e358_d_n4), multiplicity * (eq4_e358_d_n5), multiplicity * (eq4_e358_d_n6), multiplicity * (eq4_e358_d_n8), multiplicity * (eq4_e358_d_n10), multiplicity * (eq4_e358_d_n11), multiplicity * (eq4_e358_d_n12)],
            [],
            [],
            1.0,
        );
        let eq5_e361: f64 = (p.p33 * locals.var_igb);
        let eq5_e361_d_n0: f64 = (p.p33 * locals.var_igb_dn0);
        let eq5_e361_d_n2: f64 = (p.p33 * locals.var_igb_dn2);
        let eq5_e361_d_n4: f64 = (p.p33 * locals.var_igb_dn4);
        let eq5_e361_d_n5: f64 = (p.p33 * locals.var_igb_dn5);
        let eq5_e361_d_n6: f64 = (p.p33 * locals.var_igb_dn6);
        let eq5_e361_d_n8: f64 = (p.p33 * locals.var_igb_dn8);
        let eq5_e361_d_n10: f64 = (p.p33 * locals.var_igb_dn10);
        let eq5_e361_d_n11: f64 = (p.p33 * locals.var_igb_dn11);
        let eq5_e361_d_n12: f64 = (p.p33 * locals.var_igb_dn12);
        let eq5_value: f64 = eq5_e361;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq5_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq5_e361_d_n0), multiplicity * (eq5_e361_d_n2), multiplicity * (eq5_e361_d_n4), multiplicity * (eq5_e361_d_n5), multiplicity * (eq5_e361_d_n6), multiplicity * (eq5_e361_d_n8), multiplicity * (eq5_e361_d_n10), multiplicity * (eq5_e361_d_n11), multiplicity * (eq5_e361_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq6_e367, eq6_e367_d_n0, eq6_e367_d_n2, eq6_e367_d_n4, eq6_e367_d_n5, eq6_e367_d_n6, eq6_e367_d_n8, eq6_e367_d_n10, eq6_e367_d_n11, eq6_e367_d_n12,) = {
    if (p.p312 != 0.0) {
        let eq6_e365: f64 = ((nv12 - nv2) / locals.var_rsd);
        let eq6_e365_d_n0: f64 = (-(((nv12 - nv2) * locals.var_rsd_dn0) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e365_d_n2: f64 = (((-locals.var_rsd) - ((nv12 - nv2) * locals.var_rsd_dn2)) / (locals.var_rsd * locals.var_rsd));
        let eq6_e365_d_n4: f64 = (-(((nv12 - nv2) * locals.var_rsd_dn4) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e365_d_n5: f64 = (-(((nv12 - nv2) * locals.var_rsd_dn5) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e365_d_n6: f64 = (-(((nv12 - nv2) * locals.var_rsd_dn6) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e365_d_n8: f64 = (-(((nv12 - nv2) * locals.var_rsd_dn8) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e365_d_n10: f64 = (-(((nv12 - nv2) * locals.var_rsd_dn10) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e365_d_n11: f64 = (-(((nv12 - nv2) * locals.var_rsd_dn11) / (locals.var_rsd * locals.var_rsd)));
        let eq6_e365_d_n12: f64 = ((locals.var_rsd - ((nv12 - nv2) * locals.var_rsd_dn12)) / (locals.var_rsd * locals.var_rsd));
        (eq6_e365, eq6_e365_d_n0, eq6_e365_d_n2, eq6_e365_d_n4, eq6_e365_d_n5, eq6_e365_d_n6, eq6_e365_d_n8, eq6_e365_d_n10, eq6_e365_d_n11, eq6_e365_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e367;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(12),
            Some(2),
            multiplicity * (eq6_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq6_e367_d_n0), multiplicity * (eq6_e367_d_n2), multiplicity * (eq6_e367_d_n4), multiplicity * (eq6_e367_d_n5), multiplicity * (eq6_e367_d_n6), multiplicity * (eq6_e367_d_n8), multiplicity * (eq6_e367_d_n10), multiplicity * (eq6_e367_d_n11), multiplicity * (eq6_e367_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq7_e372,) = {
    if (p.p312 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e372;
        stamper.stamp_potential_const_local(
            0,
            eq7_value,
        );
        let (eq8_e378, eq8_e378_d_n0, eq8_e378_d_n2, eq8_e378_d_n4, eq8_e378_d_n5, eq8_e378_d_n6, eq8_e378_d_n8, eq8_e378_d_n10, eq8_e378_d_n11, eq8_e378_d_n12,) = {
    if (p.p313 != 0.0) {
        let eq8_e376: f64 = ((nv0 - nv11) / locals.var_rdd);
        let eq8_e376_d_n0: f64 = ((locals.var_rdd - ((nv0 - nv11) * locals.var_rdd_dn0)) / (locals.var_rdd * locals.var_rdd));
        let eq8_e376_d_n2: f64 = (-(((nv0 - nv11) * locals.var_rdd_dn2) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e376_d_n4: f64 = (-(((nv0 - nv11) * locals.var_rdd_dn4) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e376_d_n5: f64 = (-(((nv0 - nv11) * locals.var_rdd_dn5) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e376_d_n6: f64 = (-(((nv0 - nv11) * locals.var_rdd_dn6) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e376_d_n8: f64 = (-(((nv0 - nv11) * locals.var_rdd_dn8) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e376_d_n10: f64 = (-(((nv0 - nv11) * locals.var_rdd_dn10) / (locals.var_rdd * locals.var_rdd)));
        let eq8_e376_d_n11: f64 = (((-locals.var_rdd) - ((nv0 - nv11) * locals.var_rdd_dn11)) / (locals.var_rdd * locals.var_rdd));
        let eq8_e376_d_n12: f64 = (-(((nv0 - nv11) * locals.var_rdd_dn12) / (locals.var_rdd * locals.var_rdd)));
        (eq8_e376, eq8_e376_d_n0, eq8_e376_d_n2, eq8_e376_d_n4, eq8_e376_d_n5, eq8_e376_d_n6, eq8_e376_d_n8, eq8_e376_d_n10, eq8_e376_d_n11, eq8_e376_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e378;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(11),
            multiplicity * (eq8_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq8_e378_d_n0), multiplicity * (eq8_e378_d_n2), multiplicity * (eq8_e378_d_n4), multiplicity * (eq8_e378_d_n5), multiplicity * (eq8_e378_d_n6), multiplicity * (eq8_e378_d_n8), multiplicity * (eq8_e378_d_n10), multiplicity * (eq8_e378_d_n11), multiplicity * (eq8_e378_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq9_e383,) = {
    if (p.p313 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e383;
        stamper.stamp_potential_const_local(
            1,
            eq9_value,
        );
        let eq10_e387: f64 = (locals.var_qg + locals.var_qg_nqs);
        let eq10_e387_d_n8: f64 = (locals.var_qg_dn8 + locals.var_qg_nqs_dn8);
        let eq10_e388: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq10_e387);
        let eq10_e389: f64 = (p.p33 * eq10_e388);
        let eq10_e389_d_n0: f64 = (p.p33 * (locals.var_qg_dn0 * ddt_scale));
        let eq10_e389_d_n2: f64 = (p.p33 * (locals.var_qg_dn2 * ddt_scale));
        let eq10_e389_d_n4: f64 = (p.p33 * (locals.var_qg_dn4 * ddt_scale));
        let eq10_e389_d_n5: f64 = (p.p33 * (locals.var_qg_dn5 * ddt_scale));
        let eq10_e389_d_n6: f64 = (p.p33 * (locals.var_qg_dn6 * ddt_scale));
        let eq10_e389_d_n8: f64 = (p.p33 * (eq10_e387_d_n8 * ddt_scale));
        let eq10_e389_d_n9: f64 = (p.p33 * (locals.var_qg_nqs_dn9 * ddt_scale));
        let eq10_e389_d_n10: f64 = (p.p33 * (locals.var_qg_dn10 * ddt_scale));
        let eq10_e389_d_n11: f64 = (p.p33 * (locals.var_qg_dn11 * ddt_scale));
        let eq10_e389_d_n12: f64 = (p.p33 * (locals.var_qg_dn12 * ddt_scale));
        let eq10_value: f64 = eq10_e389;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq10_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq10_e389_d_n0), multiplicity * (eq10_e389_d_n2), multiplicity * (eq10_e389_d_n4), multiplicity * (eq10_e389_d_n5), multiplicity * (eq10_e389_d_n6), multiplicity * (eq10_e389_d_n8), multiplicity * (eq10_e389_d_n9), multiplicity * (eq10_e389_d_n10), multiplicity * (eq10_e389_d_n11), multiplicity * (eq10_e389_d_n12)],
            [],
            [],
            1.0,
        );
        let eq11_e393: f64 = (locals.var_qd + locals.var_qd_nqs);
        let eq11_e393_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);
        let eq11_e393_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);
        let eq11_e393_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);
        let eq11_e393_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);
        let eq11_e393_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);
        let eq11_e393_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);
        let eq11_e393_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);
        let eq11_e393_d_n11: f64 = (locals.var_qd_dn11 + locals.var_qd_nqs_dn11);
        let eq11_e393_d_n12: f64 = (locals.var_qd_dn12 + locals.var_qd_nqs_dn12);
        let eq11_e394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e393);
        let eq11_e395: f64 = (p.p33 * eq11_e394);
        let eq11_e395_d_n0: f64 = (p.p33 * (eq11_e393_d_n0 * ddt_scale));
        let eq11_e395_d_n2: f64 = (p.p33 * (eq11_e393_d_n2 * ddt_scale));
        let eq11_e395_d_n4: f64 = (p.p33 * (eq11_e393_d_n4 * ddt_scale));
        let eq11_e395_d_n5: f64 = (p.p33 * (eq11_e393_d_n5 * ddt_scale));
        let eq11_e395_d_n6: f64 = (p.p33 * (eq11_e393_d_n6 * ddt_scale));
        let eq11_e395_d_n8: f64 = (p.p33 * (eq11_e393_d_n8 * ddt_scale));
        let eq11_e395_d_n10: f64 = (p.p33 * (eq11_e393_d_n10 * ddt_scale));
        let eq11_e395_d_n11: f64 = (p.p33 * (eq11_e393_d_n11 * ddt_scale));
        let eq11_e395_d_n12: f64 = (p.p33 * (eq11_e393_d_n12 * ddt_scale));
        let eq11_value: f64 = eq11_e395;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq11_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq11_e395_d_n0), multiplicity * (eq11_e395_d_n2), multiplicity * (eq11_e395_d_n4), multiplicity * (eq11_e395_d_n5), multiplicity * (eq11_e395_d_n6), multiplicity * (eq11_e395_d_n8), multiplicity * (eq11_e395_d_n10), multiplicity * (eq11_e395_d_n11), multiplicity * (eq11_e395_d_n12)],
            [],
            [],
            1.0,
        );
        let eq12_e399: f64 = (locals.var_qb + locals.var_qb_nqs);
        let eq12_e400: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e399);
        let eq12_e401: f64 = (p.p33 * eq12_e400);
        let eq12_e401_d_n0: f64 = (p.p33 * (locals.var_qb_dn0 * ddt_scale));
        let eq12_e401_d_n2: f64 = (p.p33 * (locals.var_qb_dn2 * ddt_scale));
        let eq12_e401_d_n4: f64 = (p.p33 * (locals.var_qb_dn4 * ddt_scale));
        let eq12_e401_d_n5: f64 = (p.p33 * (locals.var_qb_dn5 * ddt_scale));
        let eq12_e401_d_n6: f64 = (p.p33 * (locals.var_qb_dn6 * ddt_scale));
        let eq12_e401_d_n8: f64 = (p.p33 * (locals.var_qb_dn8 * ddt_scale));
        let eq12_e401_d_n9: f64 = (p.p33 * (locals.var_qb_nqs_dn9 * ddt_scale));
        let eq12_e401_d_n10: f64 = (p.p33 * (locals.var_qb_dn10 * ddt_scale));
        let eq12_e401_d_n11: f64 = (p.p33 * (locals.var_qb_dn11 * ddt_scale));
        let eq12_e401_d_n12: f64 = (p.p33 * (locals.var_qb_dn12 * ddt_scale));
        let eq12_value: f64 = eq12_e401;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq12_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq12_e401_d_n0), multiplicity * (eq12_e401_d_n2), multiplicity * (eq12_e401_d_n4), multiplicity * (eq12_e401_d_n5), multiplicity * (eq12_e401_d_n6), multiplicity * (eq12_e401_d_n8), multiplicity * (eq12_e401_d_n9), multiplicity * (eq12_e401_d_n10), multiplicity * (eq12_e401_d_n11), multiplicity * (eq12_e401_d_n12)],
            [],
            [],
            1.0,
        );
        let eq14_e412: f64 = (nv7 - 0.0);
        let eq14_value: f64 = eq14_e412;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq14_value),
            7,
            multiplicity * (1.0),
        );
        let eq17_e427: f64 = (locals.var_ci * (nv7 - 0.0));
        let eq17_e427_d_n0: f64 = (locals.var_ci_dn0 * (nv7 - 0.0));
        let eq17_e427_d_n2: f64 = (locals.var_ci_dn2 * (nv7 - 0.0));
        let eq17_e427_d_n4: f64 = (locals.var_ci_dn4 * (nv7 - 0.0));
        let eq17_e427_d_n5: f64 = (locals.var_ci_dn5 * (nv7 - 0.0));
        let eq17_e427_d_n6: f64 = (locals.var_ci_dn6 * (nv7 - 0.0));
        let eq17_e427_d_n8: f64 = (locals.var_ci_dn8 * (nv7 - 0.0));
        let eq17_e427_d_n10: f64 = (locals.var_ci_dn10 * (nv7 - 0.0));
        let eq17_e427_d_n11: f64 = (locals.var_ci_dn11 * (nv7 - 0.0));
        let eq17_e427_d_n12: f64 = (locals.var_ci_dn12 * (nv7 - 0.0));
        let eq17_value: f64 = eq17_e427;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq17_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * (eq17_e427_d_n0), multiplicity * (eq17_e427_d_n2), multiplicity * (eq17_e427_d_n4), multiplicity * (eq17_e427_d_n5), multiplicity * (eq17_e427_d_n6), multiplicity * (locals.var_ci), multiplicity * (eq17_e427_d_n8), multiplicity * (eq17_e427_d_n10), multiplicity * (eq17_e427_d_n11), multiplicity * (eq17_e427_d_n12)],
            [],
            [],
            1.0,
        );
        let eq18_e430: f64 = ((nv7 - 0.0) * locals.var_sigrat_s);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn4);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn5);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn8);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e431: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e430);
        let eq18_value: f64 = eq18_e431;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq18_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * ((eq18_e430_d_n0 * ddt_scale)), multiplicity * ((eq18_e430_d_n2 * ddt_scale)), multiplicity * ((eq18_e430_d_n4 * ddt_scale)), multiplicity * ((eq18_e430_d_n5 * ddt_scale)), multiplicity * ((eq18_e430_d_n6 * ddt_scale)), multiplicity * ((locals.var_sigrat_s * ddt_scale)), multiplicity * ((eq18_e430_d_n8 * ddt_scale)), multiplicity * ((eq18_e430_d_n10 * ddt_scale)), multiplicity * ((eq18_e430_d_n11 * ddt_scale)), multiplicity * ((eq18_e430_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq19_e434: f64 = ((nv7 - 0.0) * locals.var_sigrat_d);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn4);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn5);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn8);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e435: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e434);
        let eq19_value: f64 = eq19_e435;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(11),
            multiplicity * (eq19_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * ((eq19_e434_d_n0 * ddt_scale)), multiplicity * ((eq19_e434_d_n2 * ddt_scale)), multiplicity * ((eq19_e434_d_n4 * ddt_scale)), multiplicity * ((eq19_e434_d_n5 * ddt_scale)), multiplicity * ((eq19_e434_d_n6 * ddt_scale)), multiplicity * ((locals.var_sigrat_d * ddt_scale)), multiplicity * ((eq19_e434_d_n8 * ddt_scale)), multiplicity * ((eq19_e434_d_n10 * ddt_scale)), multiplicity * ((eq19_e434_d_n11 * ddt_scale)), multiplicity * ((eq19_e434_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq25_e478, eq25_e478_d_n0, eq25_e478_d_n1, eq25_e478_d_n2, eq25_e478_d_n4, eq25_e478_d_n5, eq25_e478_d_n6, eq25_e478_d_n8, eq25_e478_d_n10, eq25_e478_d_n11, eq25_e478_d_n12,) = {
    if (p.p25 != 0.0) {
        let eq25_e476: f64 = (locals.var_grg * (nv1 - nv5));
        let eq25_e476_d_n0: f64 = (locals.var_grg_dn0 * (nv1 - nv5));
        let eq25_e476_d_n2: f64 = (locals.var_grg_dn2 * (nv1 - nv5));
        let eq25_e476_d_n4: f64 = (locals.var_grg_dn4 * (nv1 - nv5));
        let eq25_e476_d_n5: f64 = ((locals.var_grg_dn5 * (nv1 - nv5)) + (-locals.var_grg));
        let eq25_e476_d_n6: f64 = (locals.var_grg_dn6 * (nv1 - nv5));
        let eq25_e476_d_n8: f64 = (locals.var_grg_dn8 * (nv1 - nv5));
        let eq25_e476_d_n10: f64 = (locals.var_grg_dn10 * (nv1 - nv5));
        let eq25_e476_d_n11: f64 = (locals.var_grg_dn11 * (nv1 - nv5));
        let eq25_e476_d_n12: f64 = (locals.var_grg_dn12 * (nv1 - nv5));
        (eq25_e476, eq25_e476_d_n0, locals.var_grg, eq25_e476_d_n2, eq25_e476_d_n4, eq25_e476_d_n5, eq25_e476_d_n6, eq25_e476_d_n8, eq25_e476_d_n10, eq25_e476_d_n11, eq25_e476_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e478;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(5),
            multiplicity * (eq25_value),
            [0, 1, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq25_e478_d_n0), multiplicity * (eq25_e478_d_n1), multiplicity * (eq25_e478_d_n2), multiplicity * (eq25_e478_d_n4), multiplicity * (eq25_e478_d_n5), multiplicity * (eq25_e478_d_n6), multiplicity * (eq25_e478_d_n8), multiplicity * (eq25_e478_d_n10), multiplicity * (eq25_e478_d_n11), multiplicity * (eq25_e478_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq26_e483,) = {
    if (p.p25 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e483;
        stamper.stamp_potential_const_local(
            2,
            eq26_value,
        );
        let eq27_value: f64 = 0.0;
        stamper.stamp_potential_const_local(
            3,
            eq27_value,
        );
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n2, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n8, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12,) = {
    if (locals.var_guard443 != 0.0) {
        let eq28_e487: f64 = (-locals.var_rpower);
        let eq28_e490: f64 = (locals.var_cthe * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (locals.var_cthe_dn0 * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (locals.var_cthe_dn2 * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((locals.var_cthe_dn4 * (nv4 - 0.0)) + locals.var_cthe);
        let eq28_e490_d_n5: f64 = (locals.var_cthe_dn5 * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (locals.var_cthe_dn6 * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (locals.var_cthe_dn8 * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (locals.var_cthe_dn10 * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (locals.var_cthe_dn11 * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (locals.var_cthe_dn12 * (nv4 - 0.0));
        let eq28_e491: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq28_e490);
        let eq28_e492: f64 = (eq28_e487 + eq28_e491);
        let eq28_e492_d_n0: f64 = ((-locals.var_rpower_dn0) + (eq28_e490_d_n0 * ddt_scale));
        let eq28_e492_d_n2: f64 = ((-locals.var_rpower_dn2) + (eq28_e490_d_n2 * ddt_scale));
        let eq28_e492_d_n4: f64 = ((-locals.var_rpower_dn4) + (eq28_e490_d_n4 * ddt_scale));
        let eq28_e492_d_n5: f64 = ((-locals.var_rpower_dn5) + (eq28_e490_d_n5 * ddt_scale));
        let eq28_e492_d_n6: f64 = ((-locals.var_rpower_dn6) + (eq28_e490_d_n6 * ddt_scale));
        let eq28_e492_d_n8: f64 = ((-locals.var_rpower_dn8) + (eq28_e490_d_n8 * ddt_scale));
        let eq28_e492_d_n10: f64 = ((-locals.var_rpower_dn10) + (eq28_e490_d_n10 * ddt_scale));
        let eq28_e492_d_n11: f64 = ((-locals.var_rpower_dn11) + (eq28_e490_d_n11 * ddt_scale));
        let eq28_e492_d_n12: f64 = ((-locals.var_rpower_dn12) + (eq28_e490_d_n12 * ddt_scale));
        let eq28_e495: f64 = ((nv4 - 0.0) * locals.var_gth);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * locals.var_gth_dn0);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * locals.var_gth_dn2);
        let eq28_e495_d_n4: f64 = (locals.var_gth + ((nv4 - 0.0) * locals.var_gth_dn4));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * locals.var_gth_dn5);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * locals.var_gth_dn6);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * locals.var_gth_dn8);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * locals.var_gth_dn10);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * locals.var_gth_dn11);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * locals.var_gth_dn12);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n2, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n8, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e498;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            None,
            multiplicity * (eq28_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq28_e498_d_n0), multiplicity * (eq28_e498_d_n2), multiplicity * (eq28_e498_d_n4), multiplicity * (eq28_e498_d_n5), multiplicity * (eq28_e498_d_n6), multiplicity * (eq28_e498_d_n8), multiplicity * (eq28_e498_d_n10), multiplicity * (eq28_e498_d_n11), multiplicity * (eq28_e498_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq29_e503,) = {
    if (locals.var_guard443 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e503;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
    }
    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n2, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n8, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12,) = {
    if (locals.var_guard444 != 0.0) {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e509: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq30_e508);
        let eq30_e510: f64 = (locals.var_iqh_nqs + eq30_e509);
        let eq30_e510_d_n10: f64 = (locals.var_iqh_nqs_dn10 + (1e-9 * ddt_scale));
        (eq30_e510, locals.var_iqh_nqs_dn0, locals.var_iqh_nqs_dn2, locals.var_iqh_nqs_dn4, locals.var_iqh_nqs_dn5, locals.var_iqh_nqs_dn6, locals.var_iqh_nqs_dn8, eq30_e510_d_n10, locals.var_iqh_nqs_dn11, locals.var_iqh_nqs_dn12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e512;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            None,
            multiplicity * (eq30_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq30_e512_d_n0), multiplicity * (eq30_e512_d_n2), multiplicity * (eq30_e512_d_n4), multiplicity * (eq30_e512_d_n5), multiplicity * (eq30_e512_d_n6), multiplicity * (eq30_e512_d_n8), multiplicity * (eq30_e512_d_n10), multiplicity * (eq30_e512_d_n11), multiplicity * (eq30_e512_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq31_e517,) = {
    if (locals.var_guard444 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e517;
        stamper.stamp_potential_const_local(
            5,
            eq31_value,
        );
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n2, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n8, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e523: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq32_e522);
        let eq32_e524: f64 = (locals.var_iqi_nqs + eq32_e523);
        let eq32_e524_d_n8: f64 = (locals.var_iqi_nqs_dn8 + (1e-9 * ddt_scale));
        (eq32_e524, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn4, locals.var_iqi_nqs_dn5, locals.var_iqi_nqs_dn6, eq32_e524_d_n8, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e526;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            None,
            multiplicity * (eq32_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq32_e526_d_n0), multiplicity * (eq32_e526_d_n2), multiplicity * (eq32_e526_d_n4), multiplicity * (eq32_e526_d_n5), multiplicity * (eq32_e526_d_n6), multiplicity * (eq32_e526_d_n8), multiplicity * (eq32_e526_d_n10), multiplicity * (eq32_e526_d_n11), multiplicity * (eq32_e526_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n2, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e532: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq33_e531);
        let eq33_e533: f64 = (locals.var_iqb_nqs + eq33_e532);
        let eq33_e533_d_n9: f64 = (locals.var_iqb_nqs_dn9 + (1e-9 * ddt_scale));
        (eq33_e533, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn4, locals.var_iqb_nqs_dn5, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn8, eq33_e533_d_n9, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e535;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            None,
            multiplicity * (eq33_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq33_e535_d_n0), multiplicity * (eq33_e535_d_n2), multiplicity * (eq33_e535_d_n4), multiplicity * (eq33_e535_d_n5), multiplicity * (eq33_e535_d_n6), multiplicity * (eq33_e535_d_n8), multiplicity * (eq33_e535_d_n9), multiplicity * (eq33_e535_d_n10), multiplicity * (eq33_e535_d_n11), multiplicity * (eq33_e535_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq34_e540,) = {
    if (p.p24 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e540;
        stamper.stamp_potential_const_local(
            6,
            eq34_value,
        );
        let (eq35_e545,) = {
    if (p.p24 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e545;
        stamper.stamp_potential_const_local(
            7,
            eq35_value,
        );
    }
    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq10_e387: f64 = (locals.var_qg + locals.var_qg_nqs);
        let eq10_e387_d_n8: f64 = (locals.var_qg_dn8 + locals.var_qg_nqs_dn8);
        let eq10_e388_q: f64 = eq10_e387;
        let eq10_e389: f64 = (p.p33 * eq10_e387);
        let eq10_e389_d_n0: f64 = (p.p33 * locals.var_qg_dn0);
        let eq10_e389_d_n2: f64 = (p.p33 * locals.var_qg_dn2);
        let eq10_e389_d_n4: f64 = (p.p33 * locals.var_qg_dn4);
        let eq10_e389_d_n5: f64 = (p.p33 * locals.var_qg_dn5);
        let eq10_e389_d_n6: f64 = (p.p33 * locals.var_qg_dn6);
        let eq10_e389_d_n8: f64 = (p.p33 * eq10_e387_d_n8);
        let eq10_e389_d_n9: f64 = (p.p33 * locals.var_qg_nqs_dn9);
        let eq10_e389_d_n10: f64 = (p.p33 * locals.var_qg_dn10);
        let eq10_e389_d_n11: f64 = (p.p33 * locals.var_qg_dn11);
        let eq10_e389_d_n12: f64 = (p.p33 * locals.var_qg_dn12);
        let eq10_e389_q: f64 = (p.p33 * eq10_e388_q);
        let eq10_reactive_node_derivatives: [f64; 13] = [eq10_e389_d_n0, 0.0, eq10_e389_d_n2, 0.0, eq10_e389_d_n4, eq10_e389_d_n5, eq10_e389_d_n6, 0.0, eq10_e389_d_n8, eq10_e389_d_n9, eq10_e389_d_n10, eq10_e389_d_n11, eq10_e389_d_n12];
        let eq10_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e393: f64 = (locals.var_qd + locals.var_qd_nqs);
        let eq11_e393_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);
        let eq11_e393_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);
        let eq11_e393_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);
        let eq11_e393_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);
        let eq11_e393_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);
        let eq11_e393_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);
        let eq11_e393_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);
        let eq11_e393_d_n11: f64 = (locals.var_qd_dn11 + locals.var_qd_nqs_dn11);
        let eq11_e393_d_n12: f64 = (locals.var_qd_dn12 + locals.var_qd_nqs_dn12);
        let eq11_e394_q: f64 = eq11_e393;
        let eq11_e395: f64 = (p.p33 * eq11_e393);
        let eq11_e395_d_n0: f64 = (p.p33 * eq11_e393_d_n0);
        let eq11_e395_d_n2: f64 = (p.p33 * eq11_e393_d_n2);
        let eq11_e395_d_n4: f64 = (p.p33 * eq11_e393_d_n4);
        let eq11_e395_d_n5: f64 = (p.p33 * eq11_e393_d_n5);
        let eq11_e395_d_n6: f64 = (p.p33 * eq11_e393_d_n6);
        let eq11_e395_d_n8: f64 = (p.p33 * eq11_e393_d_n8);
        let eq11_e395_d_n10: f64 = (p.p33 * eq11_e393_d_n10);
        let eq11_e395_d_n11: f64 = (p.p33 * eq11_e393_d_n11);
        let eq11_e395_d_n12: f64 = (p.p33 * eq11_e393_d_n12);
        let eq11_e395_q: f64 = (p.p33 * eq11_e394_q);
        let eq11_reactive_node_derivatives: [f64; 13] = [eq11_e395_d_n0, 0.0, eq11_e395_d_n2, 0.0, eq11_e395_d_n4, eq11_e395_d_n5, eq11_e395_d_n6, 0.0, eq11_e395_d_n8, 0.0, eq11_e395_d_n10, eq11_e395_d_n11, eq11_e395_d_n12];
        let eq11_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e399: f64 = (locals.var_qb + locals.var_qb_nqs);
        let eq12_e400_q: f64 = eq12_e399;
        let eq12_e401: f64 = (p.p33 * eq12_e399);
        let eq12_e401_d_n0: f64 = (p.p33 * locals.var_qb_dn0);
        let eq12_e401_d_n2: f64 = (p.p33 * locals.var_qb_dn2);
        let eq12_e401_d_n4: f64 = (p.p33 * locals.var_qb_dn4);
        let eq12_e401_d_n5: f64 = (p.p33 * locals.var_qb_dn5);
        let eq12_e401_d_n6: f64 = (p.p33 * locals.var_qb_dn6);
        let eq12_e401_d_n8: f64 = (p.p33 * locals.var_qb_dn8);
        let eq12_e401_d_n9: f64 = (p.p33 * locals.var_qb_nqs_dn9);
        let eq12_e401_d_n10: f64 = (p.p33 * locals.var_qb_dn10);
        let eq12_e401_d_n11: f64 = (p.p33 * locals.var_qb_dn11);
        let eq12_e401_d_n12: f64 = (p.p33 * locals.var_qb_dn12);
        let eq12_e401_q: f64 = (p.p33 * eq12_e400_q);
        let eq12_reactive_node_derivatives: [f64; 13] = [eq12_e401_d_n0, 0.0, eq12_e401_d_n2, 0.0, eq12_e401_d_n4, eq12_e401_d_n5, eq12_e401_d_n6, 0.0, eq12_e401_d_n8, eq12_e401_d_n9, eq12_e401_d_n10, eq12_e401_d_n11, eq12_e401_d_n12];
        let eq12_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[12]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e430: f64 = ((nv7 - 0.0) * locals.var_sigrat_s);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn4);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn5);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn8);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e431_q: f64 = eq18_e430;
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e430_d_n0, 0.0, eq18_e430_d_n2, 0.0, eq18_e430_d_n4, eq18_e430_d_n5, eq18_e430_d_n6, locals.var_sigrat_s, eq18_e430_d_n8, 0.0, eq18_e430_d_n10, eq18_e430_d_n11, eq18_e430_d_n12];
        let eq18_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e434: f64 = ((nv7 - 0.0) * locals.var_sigrat_d);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn4);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn5);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn8);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e435_q: f64 = eq19_e434;
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e434_d_n0, 0.0, eq19_e434_d_n2, 0.0, eq19_e434_d_n4, eq19_e434_d_n5, eq19_e434_d_n6, locals.var_sigrat_d, eq19_e434_d_n8, 0.0, eq19_e434_d_n10, eq19_e434_d_n11, eq19_e434_d_n12];
        let eq19_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n2, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n8, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12, eq28_e498_q, eq28_e498_q_d_n0, eq28_e498_q_d_n2, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, eq28_e498_q_d_n8, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12,) = {
    if (locals.var_guard443 != 0.0) {
        let eq28_e487: f64 = (-locals.var_rpower);
        let eq28_e490: f64 = (locals.var_cthe * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (locals.var_cthe_dn0 * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (locals.var_cthe_dn2 * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((locals.var_cthe_dn4 * (nv4 - 0.0)) + locals.var_cthe);
        let eq28_e490_d_n5: f64 = (locals.var_cthe_dn5 * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (locals.var_cthe_dn6 * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (locals.var_cthe_dn8 * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (locals.var_cthe_dn10 * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (locals.var_cthe_dn11 * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (locals.var_cthe_dn12 * (nv4 - 0.0));
        let eq28_e491_q: f64 = eq28_e490;
        let eq28_e492: f64 = (eq28_e487 + eq28_e490);
        let eq28_e492_d_n0: f64 = ((-locals.var_rpower_dn0) + eq28_e490_d_n0);
        let eq28_e492_d_n2: f64 = ((-locals.var_rpower_dn2) + eq28_e490_d_n2);
        let eq28_e492_d_n4: f64 = ((-locals.var_rpower_dn4) + eq28_e490_d_n4);
        let eq28_e492_d_n5: f64 = ((-locals.var_rpower_dn5) + eq28_e490_d_n5);
        let eq28_e492_d_n6: f64 = ((-locals.var_rpower_dn6) + eq28_e490_d_n6);
        let eq28_e492_d_n8: f64 = ((-locals.var_rpower_dn8) + eq28_e490_d_n8);
        let eq28_e492_d_n10: f64 = ((-locals.var_rpower_dn10) + eq28_e490_d_n10);
        let eq28_e492_d_n11: f64 = ((-locals.var_rpower_dn11) + eq28_e490_d_n11);
        let eq28_e492_d_n12: f64 = ((-locals.var_rpower_dn12) + eq28_e490_d_n12);
        let eq28_e492_q: f64 = eq28_e491_q;
        let eq28_e495: f64 = ((nv4 - 0.0) * locals.var_gth);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * locals.var_gth_dn0);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * locals.var_gth_dn2);
        let eq28_e495_d_n4: f64 = (locals.var_gth + ((nv4 - 0.0) * locals.var_gth_dn4));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * locals.var_gth_dn5);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * locals.var_gth_dn6);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * locals.var_gth_dn8);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * locals.var_gth_dn10);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * locals.var_gth_dn11);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * locals.var_gth_dn12);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        let eq28_e496_q: f64 = eq28_e492_q;
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n2, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n8, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12, eq28_e496_q, eq28_e490_d_n0, eq28_e490_d_n2, eq28_e490_d_n4, eq28_e490_d_n5, eq28_e490_d_n6, eq28_e490_d_n8, eq28_e490_d_n10, eq28_e490_d_n11, eq28_e490_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e498_q_d_n0, 0.0, eq28_e498_q_d_n2, 0.0, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, 0.0, eq28_e498_q_d_n8, 0.0, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12];
        let eq28_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n2, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n8, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12, eq30_e512_q, eq30_e512_q_d_n10,) = {
    if (locals.var_guard444 != 0.0) {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e509_q: f64 = eq30_e508;
        let eq30_e510: f64 = (locals.var_iqh_nqs + eq30_e508);
        let eq30_e510_d_n10: f64 = (locals.var_iqh_nqs_dn10 + 1e-9);
        let eq30_e510_q: f64 = eq30_e509_q;
        (eq30_e510, locals.var_iqh_nqs_dn0, locals.var_iqh_nqs_dn2, locals.var_iqh_nqs_dn4, locals.var_iqh_nqs_dn5, locals.var_iqh_nqs_dn6, locals.var_iqh_nqs_dn8, eq30_e510_d_n10, locals.var_iqh_nqs_dn11, locals.var_iqh_nqs_dn12, eq30_e510_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e512_q_d_n10),
        );
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n2, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n8, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12, eq32_e526_q, eq32_e526_q_d_n8,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e523_q: f64 = eq32_e522;
        let eq32_e524: f64 = (locals.var_iqi_nqs + eq32_e522);
        let eq32_e524_d_n8: f64 = (locals.var_iqi_nqs_dn8 + 1e-9);
        let eq32_e524_q: f64 = eq32_e523_q;
        (eq32_e524, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn4, locals.var_iqi_nqs_dn5, locals.var_iqi_nqs_dn6, eq32_e524_d_n8, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12, eq32_e524_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[8]),
            None,
            nodes[8],
            multiplicity * (eq32_e526_q_d_n8),
        );
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n2, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12, eq33_e535_q, eq33_e535_q_d_n9,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e532_q: f64 = eq33_e531;
        let eq33_e533: f64 = (locals.var_iqb_nqs + eq33_e531);
        let eq33_e533_d_n9: f64 = (locals.var_iqb_nqs_dn9 + 1e-9);
        let eq33_e533_q: f64 = eq33_e532_q;
        (eq33_e533, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn4, locals.var_iqb_nqs_dn5, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn8, eq33_e533_d_n9, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, eq33_e533_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (eq33_e535_q_d_n9),
        );
    }
}

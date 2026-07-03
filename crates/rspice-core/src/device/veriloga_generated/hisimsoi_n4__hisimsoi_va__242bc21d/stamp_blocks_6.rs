#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_reactive_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1022 == 0.0)) && (locals.var_guard1024 == 0.0)) {
            let assign31800_e46766: f64 = (-locals.var_weffcv_nf);
            let assign31800_e46767: f64 = (locals.var_cgdoe * assign31800_e46766);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31800_e46767, (locals.var_cgdoe_dn0 * assign31800_e46766), (locals.var_cgdoe_dn2 * assign31800_e46766), (locals.var_cgdoe_dn6 * assign31800_e46766), (locals.var_cgdoe_dn7 * assign31800_e46766), (locals.var_cgdoe_dn10 * assign31800_e46766), (locals.var_cgdoe_dn11 * assign31800_e46766), (locals.var_cgdoe_dn12 * assign31800_e46766), (locals.var_cgdoe_dn17 * assign31800_e46766), );
            locals.var_cgdoe_rv = 0.0;
        }
        if ((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) {
            let assign31810_e46775: f64 = (-locals.var_cgdoe);
            let assign31810_e46778: f64 = (locals.var_vgs - locals.var_vds);
            let assign31810_e46779: f64 = (assign31810_e46775 * assign31810_e46778);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign31810_e46779, (((-locals.var_cgdoe_dn0) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn0))), (((-locals.var_cgdoe_dn2) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn2))), (((-locals.var_cgdoe_dn6) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((-locals.var_cgdoe_dn7) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((-locals.var_cgdoe_dn10) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn10))), (((-locals.var_cgdoe_dn11) * assign31810_e46778) + (assign31810_e46775 * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (((-locals.var_cgdoe_dn12) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn12))), (((-locals.var_cgdoe_dn17) * assign31810_e46778) + (assign31810_e46775 * (-locals.var_vds_dn17))), );
            locals.var_qgod_rv = 0.0;
        }
        let assign31820_e46794: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgso_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign31820_e46794;
        locals.var_guard1025_rv = 0.0;
        let assign31830_e46797: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign31830_e46797;
        locals.var_guard1026_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 != 0.0)) && (locals.var_guard1026 != 0.0)) {
            let assign31840_e46807: f64 = (-locals.var_cox0__blk906);
            let assign31840_e46809: f64 = (assign31840_e46807 * p.p188);
            let assign31840_e46811: f64 = (assign31840_e46809 * locals.var_w_dioscv);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31840_e46811, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 != 0.0)) && (locals.var_guard1026 == 0.0)) {
            let assign31850_e46824: f64 = (-locals.var_cox0__blk906);
            let assign31850_e46826: f64 = (assign31850_e46824 * p.p188);
            let assign31850_e46828: f64 = (assign31850_e46826 * locals.var_weffcv_nf);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31850_e46828, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        if (((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) {
            let assign31860_e46840: f64 = (locals.var_modenml * p.p170);
            let assign31860_e46843: f64 = (locals.var_modervs * p.p169);
            let assign31860_e46844: f64 = (assign31860_e46840 + assign31860_e46843);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31860_e46844, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        let assign31870_e46849: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1027 = assign31870_e46849;
        locals.var_guard1027_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 != 0.0)) {
            let assign31880_e46861: f64 = (locals.var_modenml * locals.var_w_dioscv);
            let assign31880_e46864: f64 = (locals.var_modervs * locals.var_w_diodcv);
            let assign31880_e46865: f64 = (assign31880_e46861 + assign31880_e46864);
            (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17, ) = (assign31880_e46865, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk896_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 != 0.0)) {
            let assign31890_e46879: f64 = (-locals.var_t1__blk896);
            let assign31890_e46880: f64 = (locals.var_cgsoe * assign31890_e46879);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31890_e46880, ((locals.var_cgsoe_dn0 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn0))), ((locals.var_cgsoe_dn2 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn2))), ((locals.var_cgsoe_dn6 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn6))), ((locals.var_cgsoe_dn7 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn7))), ((locals.var_cgsoe_dn10 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn10))), ((locals.var_cgsoe_dn11 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn11))), ((locals.var_cgsoe_dn12 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn12))), ((locals.var_cgsoe_dn17 * assign31890_e46879) + (locals.var_cgsoe * (-locals.var_t1__blk896_dn17))), );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1027 == 0.0)) {
            let assign31900_e46895: f64 = (-locals.var_weffcv_nf);
            let assign31900_e46896: f64 = (locals.var_cgsoe * assign31900_e46895);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31900_e46896, (locals.var_cgsoe_dn0 * assign31900_e46895), (locals.var_cgsoe_dn2 * assign31900_e46895), (locals.var_cgsoe_dn6 * assign31900_e46895), (locals.var_cgsoe_dn7 * assign31900_e46895), (locals.var_cgsoe_dn10 * assign31900_e46895), (locals.var_cgsoe_dn11 * assign31900_e46895), (locals.var_cgsoe_dn12 * assign31900_e46895), (locals.var_cgsoe_dn17 * assign31900_e46895), );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((p.p24 != 0.0) && (locals.var_guard978 == 0.0)) {
            let assign31910_e46904: f64 = (-locals.var_cgsoe);
            let assign31910_e46906: f64 = (assign31910_e46904 * locals.var_vgs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign31910_e46906, ((-locals.var_cgsoe_dn0) * locals.var_vgs), ((-locals.var_cgsoe_dn2) * locals.var_vgs), (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn6)), (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn7)), ((-locals.var_cgsoe_dn10) * locals.var_vgs), (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31910_e46904 * locals.var_vgs_dn11)), ((-locals.var_cgsoe_dn12) * locals.var_vgs), ((-locals.var_cgsoe_dn17) * locals.var_vgs), );
            locals.var_qgos_rv = 0.0;
        }
        let assign31920_e46911: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1028 = assign31920_e46911;
        locals.var_guard1028_rv = 0.0;
        if (locals.var_guard1028 != 0.0) {
            (locals.var_vbdj, locals.var_vbdj_dn6, locals.var_vbdj_dn12, ) = (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12, );
            locals.var_vbdj_rv = 0.0;
            (locals.var_vbsj, locals.var_vbsj_dn7, locals.var_vbsj_dn12, ) = (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12, );
            locals.var_vbsj_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign31950_e46924: f64 = (locals.var_egtnom * locals.var_betatnom);
            let assign31950_e46927: f64 = (locals.var_eg * locals.var_beta);
            let assign31950_e46928: f64 = (assign31950_e46924 - assign31950_e46927);
            let assign31950_e46932: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            let assign31950_e46933: f64 = (assign31950_e46932).ln();
            let assign31950_e46934: f64 = (p.p175 * assign31950_e46933);
            let assign31950_e46935: f64 = (assign31950_e46928 + assign31950_e46934);
            let assign31950_e46937: f64 = (assign31950_e46935 / p.p174);
            let assign31950_e46938: f64 = (assign31950_e46937).exp();
            let assign31950_e46939: f64 = (p.p173 * assign31950_e46938);
            (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn12, locals.var_js_dn17, ) = (assign31950_e46939, (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p175 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31950_e46932))) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31950_e46938 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))), );
            locals.var_js_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign31960_e46946: f64 = (locals.var_egtnom * locals.var_betatnom);
            let assign31960_e46949: f64 = (locals.var_eg * locals.var_beta);
            let assign31960_e46950: f64 = (assign31960_e46946 - assign31960_e46949);
            let assign31960_e46954: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            let assign31960_e46955: f64 = (assign31960_e46954).ln();
            let assign31960_e46956: f64 = (p.p176 * assign31960_e46955);
            let assign31960_e46957: f64 = (assign31960_e46950 + assign31960_e46956);
            let assign31960_e46959: f64 = (assign31960_e46957 / p.p174);
            let assign31960_e46960: f64 = (assign31960_e46959).exp();
            let assign31960_e46961: f64 = (p.p173 * assign31960_e46960);
            (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn12, locals.var_js2_dn17, ) = (assign31960_e46961, (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p176 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31960_e46954))) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31960_e46960 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))), );
            locals.var_js2_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign31970_e46967: f64 = (locals.var_w_diod * p.p237);
            let assign31970_e46969: f64 = (assign31970_e46967 * locals.var_js);
            (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17, ) = (assign31970_e46969, (assign31970_e46967 * locals.var_js_dn0), (assign31970_e46967 * locals.var_js_dn2), (assign31970_e46967 * locals.var_js_dn6), (assign31970_e46967 * locals.var_js_dn7), (assign31970_e46967 * locals.var_js_dn10), (assign31970_e46967 * locals.var_js_dn11), (assign31970_e46967 * locals.var_js_dn12), (assign31970_e46967 * locals.var_js_dn17), );
            locals.var_isbd_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign31980_e46975: f64 = (locals.var_w_diod * p.p237);
            let assign31980_e46977: f64 = (assign31980_e46975 * locals.var_js2);
            (locals.var_isbd2, locals.var_isbd2_dn0, locals.var_isbd2_dn2, locals.var_isbd2_dn6, locals.var_isbd2_dn7, locals.var_isbd2_dn10, locals.var_isbd2_dn11, locals.var_isbd2_dn12, locals.var_isbd2_dn17, ) = (assign31980_e46977, (assign31980_e46975 * locals.var_js2_dn0), (assign31980_e46975 * locals.var_js2_dn2), (assign31980_e46975 * locals.var_js2_dn6), (assign31980_e46975 * locals.var_js2_dn7), (assign31980_e46975 * locals.var_js2_dn10), (assign31980_e46975 * locals.var_js2_dn11), (assign31980_e46975 * locals.var_js2_dn12), (assign31980_e46975 * locals.var_js2_dn17), );
            locals.var_isbd2_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign31990_e46983: f64 = (locals.var_w_dios * p.p237);
            let assign31990_e46985: f64 = (assign31990_e46983 * locals.var_js);
            (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn12, locals.var_isbs_dn17, ) = (assign31990_e46985, (assign31990_e46983 * locals.var_js_dn0), (assign31990_e46983 * locals.var_js_dn2), (assign31990_e46983 * locals.var_js_dn6), (assign31990_e46983 * locals.var_js_dn7), (assign31990_e46983 * locals.var_js_dn10), (assign31990_e46983 * locals.var_js_dn11), (assign31990_e46983 * locals.var_js_dn12), (assign31990_e46983 * locals.var_js_dn17), );
            locals.var_isbs_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32000_e46991: f64 = (locals.var_w_dios * p.p237);
            let assign32000_e46993: f64 = (assign32000_e46991 * locals.var_js2);
            (locals.var_isbs2, locals.var_isbs2_dn0, locals.var_isbs2_dn2, locals.var_isbs2_dn6, locals.var_isbs2_dn7, locals.var_isbs2_dn10, locals.var_isbs2_dn11, locals.var_isbs2_dn12, locals.var_isbs2_dn17, ) = (assign32000_e46993, (assign32000_e46991 * locals.var_js2_dn0), (assign32000_e46991 * locals.var_js2_dn2), (assign32000_e46991 * locals.var_js2_dn6), (assign32000_e46991 * locals.var_js2_dn7), (assign32000_e46991 * locals.var_js2_dn10), (assign32000_e46991 * locals.var_js2_dn11), (assign32000_e46991 * locals.var_js2_dn12), (assign32000_e46991 * locals.var_js2_dn17), );
            locals.var_isbs2_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32010_e46999: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32010_e46999, 0.0, 0.0, (locals.var_ttemp_dn10 / locals.var_uc_tnom), 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32030_e47011: f64 = (locals.var_isbd + 1e-50);
            (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17, ) = (assign32030_e47011, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17, );
            locals.var_t2__blk1031_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            (locals.var_vbdt, locals.var_vbdt_dn10, ) = (0.0, 0.0, );
            locals.var_vbdt_rv = 0.0;
            (locals.var_vbst, locals.var_vbst_dn10, ) = (0.0, 0.0, );
            locals.var_vbst_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32070_e47039: f64 = (p.p174 * locals.var_beta_inv);
            (locals.var_nvtm, locals.var_nvtm_dn10, ) = (assign32070_e47039, (p.p174 * locals.var_beta_inv_dn10), );
            locals.var_nvtm_rv = 0.0;
        }
        let assign32080_e47044: f64 = if locals.var_vbdj < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard1057 = assign32080_e47044;
        locals.var_guard1057_rv = 0.0;
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 != 0.0)) {
            let assign32090_e47050: f64 = (locals.var_vbdj / locals.var_nvtm);
            let assign32090_e47051: f64 = (assign32090_e47050).exp();
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32090_e47051, (assign32090_e47051 * (locals.var_vbdj_dn6 / locals.var_nvtm)), 0.0, (assign32090_e47051 * (-((locals.var_vbdj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32090_e47051 * (locals.var_vbdj_dn12 / locals.var_nvtm)), );
            locals.var_t1__blk1030_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 != 0.0)) {
            let assign32100_e47060: f64 = (locals.var_t1__blk1030 - 1.0);
            let assign32100_e47061: f64 = (locals.var_isbd * assign32100_e47060);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32100_e47061, (locals.var_isbd_dn0 * assign32100_e47060), (locals.var_isbd_dn2 * assign32100_e47060), ((locals.var_isbd_dn6 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn6)), ((locals.var_isbd_dn7 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn7)), ((locals.var_isbd_dn10 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn10)), (locals.var_isbd_dn11 * assign32100_e47060), ((locals.var_isbd_dn12 * assign32100_e47060) + (locals.var_isbd * locals.var_t1__blk1030_dn12)), (locals.var_isbd_dn17 * assign32100_e47060), );
            locals.var_ibd_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 == 0.0)) {
            let assign32110_e47070: f64 = (locals.var_vbdt / locals.var_nvtm);
            let assign32110_e47071: f64 = (assign32110_e47070).exp();
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32110_e47071, 0.0, 0.0, (assign32110_e47071 * (((locals.var_vbdt_dn10 * locals.var_nvtm) - (locals.var_vbdt * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1057 == 0.0)) {
            let assign32120_e47081: f64 = (locals.var_t1__blk1030 - 1.0);
            let assign32120_e47082: f64 = (locals.var_isbd * assign32120_e47081);
            let assign32120_e47085: f64 = (locals.var_isbd / locals.var_nvtm);
            let assign32120_e47087: f64 = (assign32120_e47085 * locals.var_t1__blk1030);
            let assign32120_e47090: f64 = (locals.var_vbdj - locals.var_vbdt);
            let assign32120_e47091: f64 = (assign32120_e47087 * assign32120_e47090);
            let assign32120_e47092: f64 = (assign32120_e47082 + assign32120_e47091);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32120_e47092, ((locals.var_isbd_dn0 * assign32120_e47081) + (((locals.var_isbd_dn0 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), ((locals.var_isbd_dn2 * assign32120_e47081) + (((locals.var_isbd_dn2 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), (((locals.var_isbd_dn6 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn6)) + (((((locals.var_isbd_dn6 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn6)) * assign32120_e47090) + (assign32120_e47087 * locals.var_vbdj_dn6))), (((locals.var_isbd_dn7 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn7)) + ((((locals.var_isbd_dn7 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn7)) * assign32120_e47090)), (((locals.var_isbd_dn10 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn10)) + (((((((locals.var_isbd_dn10 * locals.var_nvtm) - (locals.var_isbd * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn10)) * assign32120_e47090) + (assign32120_e47087 * (-locals.var_vbdt_dn10)))), ((locals.var_isbd_dn11 * assign32120_e47081) + (((locals.var_isbd_dn11 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), (((locals.var_isbd_dn12 * assign32120_e47081) + (locals.var_isbd * locals.var_t1__blk1030_dn12)) + (((((locals.var_isbd_dn12 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32120_e47085 * locals.var_t1__blk1030_dn12)) * assign32120_e47090) + (assign32120_e47087 * locals.var_vbdj_dn12))), ((locals.var_isbd_dn17 * assign32120_e47081) + (((locals.var_isbd_dn17 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32120_e47090)), );
            locals.var_ibd_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32130_e47099: f64 = (p.p178 * locals.var_vbdj);
            let assign32130_e47101: f64 = (assign32130_e47099 * locals.var_isbd2);
            let assign32130_e47102: f64 = (locals.var_ibd + assign32130_e47101);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32130_e47102, (locals.var_ibd_dn0 + (assign32130_e47099 * locals.var_isbd2_dn0)), (locals.var_ibd_dn2 + (assign32130_e47099 * locals.var_isbd2_dn2)), (locals.var_ibd_dn6 + (((p.p178 * locals.var_vbdj_dn6) * locals.var_isbd2) + (assign32130_e47099 * locals.var_isbd2_dn6))), (locals.var_ibd_dn7 + (assign32130_e47099 * locals.var_isbd2_dn7)), (locals.var_ibd_dn10 + (assign32130_e47099 * locals.var_isbd2_dn10)), (locals.var_ibd_dn11 + (assign32130_e47099 * locals.var_isbd2_dn11)), (locals.var_ibd_dn12 + (((p.p178 * locals.var_vbdj_dn12) * locals.var_isbd2) + (assign32130_e47099 * locals.var_isbd2_dn12))), (locals.var_ibd_dn17 + (assign32130_e47099 * locals.var_isbd2_dn17)), );
            locals.var_ibd_rv = 0.0;
        }
        let assign32140_e47107: f64 = if locals.var_vbsj < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard1058 = assign32140_e47107;
        locals.var_guard1058_rv = 0.0;
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 != 0.0)) {
            let assign32150_e47113: f64 = (locals.var_vbsj / locals.var_nvtm);
            let assign32150_e47114: f64 = (assign32150_e47113).exp();
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32150_e47114, 0.0, (assign32150_e47114 * (locals.var_vbsj_dn7 / locals.var_nvtm)), (assign32150_e47114 * (-((locals.var_vbsj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32150_e47114 * (locals.var_vbsj_dn12 / locals.var_nvtm)), );
            locals.var_t1__blk1030_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 != 0.0)) {
            let assign32160_e47123: f64 = (locals.var_t1__blk1030 - 1.0);
            let assign32160_e47124: f64 = (locals.var_isbs * assign32160_e47123);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32160_e47124, (locals.var_isbs_dn0 * assign32160_e47123), (locals.var_isbs_dn2 * assign32160_e47123), ((locals.var_isbs_dn6 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn6)), ((locals.var_isbs_dn7 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn7)), ((locals.var_isbs_dn10 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn10)), (locals.var_isbs_dn11 * assign32160_e47123), ((locals.var_isbs_dn12 * assign32160_e47123) + (locals.var_isbs * locals.var_t1__blk1030_dn12)), (locals.var_isbs_dn17 * assign32160_e47123), );
            locals.var_ibs_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 == 0.0)) {
            let assign32170_e47133: f64 = (locals.var_vbst / locals.var_nvtm);
            let assign32170_e47134: f64 = (assign32170_e47133).exp();
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32170_e47134, 0.0, 0.0, (assign32170_e47134 * (((locals.var_vbst_dn10 * locals.var_nvtm) - (locals.var_vbst * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1058 == 0.0)) {
            let assign32180_e47144: f64 = (locals.var_t1__blk1030 - 1.0);
            let assign32180_e47145: f64 = (locals.var_isbs * assign32180_e47144);
            let assign32180_e47148: f64 = (locals.var_isbs / locals.var_nvtm);
            let assign32180_e47150: f64 = (assign32180_e47148 * locals.var_t1__blk1030);
            let assign32180_e47153: f64 = (locals.var_vbsj - locals.var_vbst);
            let assign32180_e47154: f64 = (assign32180_e47150 * assign32180_e47153);
            let assign32180_e47155: f64 = (assign32180_e47145 + assign32180_e47154);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32180_e47155, ((locals.var_isbs_dn0 * assign32180_e47144) + (((locals.var_isbs_dn0 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), ((locals.var_isbs_dn2 * assign32180_e47144) + (((locals.var_isbs_dn2 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), (((locals.var_isbs_dn6 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn6)) + ((((locals.var_isbs_dn6 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn6)) * assign32180_e47153)), (((locals.var_isbs_dn7 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn7)) + (((((locals.var_isbs_dn7 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn7)) * assign32180_e47153) + (assign32180_e47150 * locals.var_vbsj_dn7))), (((locals.var_isbs_dn10 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn10)) + (((((((locals.var_isbs_dn10 * locals.var_nvtm) - (locals.var_isbs * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn10)) * assign32180_e47153) + (assign32180_e47150 * (-locals.var_vbst_dn10)))), ((locals.var_isbs_dn11 * assign32180_e47144) + (((locals.var_isbs_dn11 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), (((locals.var_isbs_dn12 * assign32180_e47144) + (locals.var_isbs * locals.var_t1__blk1030_dn12)) + (((((locals.var_isbs_dn12 / locals.var_nvtm) * locals.var_t1__blk1030) + (assign32180_e47148 * locals.var_t1__blk1030_dn12)) * assign32180_e47153) + (assign32180_e47150 * locals.var_vbsj_dn12))), ((locals.var_isbs_dn17 * assign32180_e47144) + (((locals.var_isbs_dn17 / locals.var_nvtm) * locals.var_t1__blk1030) * assign32180_e47153)), );
            locals.var_ibs_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32190_e47162: f64 = (p.p178 * locals.var_vbsj);
            let assign32190_e47164: f64 = (assign32190_e47162 * locals.var_isbs2);
            let assign32190_e47165: f64 = (locals.var_ibs + assign32190_e47164);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32190_e47165, (locals.var_ibs_dn0 + (assign32190_e47162 * locals.var_isbs2_dn0)), (locals.var_ibs_dn2 + (assign32190_e47162 * locals.var_isbs2_dn2)), (locals.var_ibs_dn6 + (assign32190_e47162 * locals.var_isbs2_dn6)), (locals.var_ibs_dn7 + (((p.p178 * locals.var_vbsj_dn7) * locals.var_isbs2) + (assign32190_e47162 * locals.var_isbs2_dn7))), (locals.var_ibs_dn10 + (assign32190_e47162 * locals.var_isbs2_dn10)), (locals.var_ibs_dn11 + (assign32190_e47162 * locals.var_isbs2_dn11)), (locals.var_ibs_dn12 + (((p.p178 * locals.var_vbsj_dn12) * locals.var_isbs2) + (assign32190_e47162 * locals.var_isbs2_dn12))), (locals.var_ibs_dn17 + (assign32190_e47162 * locals.var_isbs2_dn17)), );
            locals.var_ibs_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32200_e47172: f64 = (locals.var_gjmin * locals.var_vbdj);
            let assign32200_e47173: f64 = (locals.var_ibd + assign32200_e47172);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32200_e47173, locals.var_ibd_dn0, locals.var_ibd_dn2, (locals.var_ibd_dn6 + (locals.var_gjmin * locals.var_vbdj_dn6)), locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, (locals.var_ibd_dn12 + (locals.var_gjmin * locals.var_vbdj_dn12)), locals.var_ibd_dn17, );
            locals.var_ibd_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32210_e47180: f64 = (locals.var_gjmin * locals.var_vbsj);
            let assign32210_e47181: f64 = (locals.var_ibs + assign32210_e47180);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32210_e47181, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, (locals.var_ibs_dn7 + (locals.var_gjmin * locals.var_vbsj_dn7)), locals.var_ibs_dn10, locals.var_ibs_dn11, (locals.var_ibs_dn12 + (locals.var_gjmin * locals.var_vbsj_dn12)), locals.var_ibs_dn17, );
            locals.var_ibs_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32220_e47187: f64 = (p.p179 * p.p2);
            locals.var_czbd = assign32220_e47187;
            locals.var_czbd_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32230_e47193: f64 = (p.p179 * p.p3);
            locals.var_czbs = assign32230_e47193;
            locals.var_czbs_rv = 0.0;
        }
        if (locals.var_guard1028 != 0.0) {
            let assign32240_e47199: f64 = (p.p237 - p.p238);
            locals.var_xp_max = assign32240_e47199;
            locals.var_xp_max_rv = 0.0;
        }
        let assign32250_e47204: f64 = if locals.var_xp_max <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1059 = assign32250_e47204;
        locals.var_guard1059_rv = 0.0;
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1059 != 0.0)) {
            locals.var_czbd = 0.0;
            locals.var_czbd_rv = 0.0;
            locals.var_czbs = 0.0;
            locals.var_czbs_rv = 0.0;
        }
        let assign32280_e47219: f64 = if p.p5 > locals.var_w_dioscv { 1.0 } else { 0.0 };
        locals.var_guard1060 = assign32280_e47219;
        locals.var_guard1060_rv = 0.0;
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) {
            let assign32290_e47226: f64 = (p.p5 - locals.var_w_dioscv);
            let assign32290_e47227: f64 = (p.p180 * assign32290_e47226);
            locals.var_czbssw = assign32290_e47227;
            locals.var_czbssw_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) {
            let assign32300_e47235: f64 = (p.p181 * locals.var_w_dioscv);
            locals.var_czbsswg = assign32300_e47235;
            locals.var_czbsswg_rv = 0.0;
        }
        let assign32310_e47240: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign32310_e47240;
        locals.var_guard1061_rv = 0.0;
        let assign32320_e47243: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign32320_e47243;
        locals.var_guard1062_rv = 0.0;
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
            let assign32330_e47254: f64 = (locals.var_vbsj / p.p185);
            let assign32330_e47255: f64 = (1.0 - assign32330_e47254);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32330_e47255, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)), );
            locals.var_arg__blk1055_rv = 0.0;
        }
        let assign32340_e47260: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign32340_e47260;
        locals.var_guard1063_rv = 0.0;
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) {
            let assign32350_e47272: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32350_e47273: f64 = (1.0 / assign32350_e47272);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32350_e47273, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32350_e47272)) / (assign32350_e47272 * assign32350_e47272))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
            let assign32360_e47288: f64 = (-p.p182);
            let assign32360_e47289: f64 = (locals.var_arg__blk1055).powf(assign32360_e47288);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32360_e47289, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32360_e47288) as f64).is_finite() && ((assign32360_e47288) as f64).fract() == 0.0 { if assign32360_e47288 == 0.0 { 0.0 } else { (assign32360_e47288 * ((locals.var_arg__blk1055).powf(assign32360_e47288 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32360_e47289 * (assign32360_e47288 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
            let assign32370_e47301: f64 = (p.p185 * locals.var_czbs);
            let assign32370_e47305: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32370_e47306: f64 = (1.0 - assign32370_e47305);
            let assign32370_e47307: f64 = (assign32370_e47301 * assign32370_e47306);
            let assign32370_e47310: f64 = (1.0 - p.p182);
            let assign32370_e47311: f64 = (assign32370_e47307 / assign32370_e47310);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32370_e47311, 0.0, 0.0, ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32370_e47310), ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32370_e47310), 0.0, 0.0, ((assign32370_e47301 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32370_e47310), 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 == 0.0)) {
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        let assign32390_e47327: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign32390_e47327;
        locals.var_guard1064_rv = 0.0;
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) {
            let assign32400_e47338: f64 = (locals.var_vbsj / p.p186);
            let assign32400_e47339: f64 = (1.0 - assign32400_e47338);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32400_e47339, 0.0, (-(locals.var_vbsj_dn7 / p.p186)), (-(locals.var_vbsj_dn12 / p.p186)), );
            locals.var_arg__blk1055_rv = 0.0;
        }
        let assign32410_e47344: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign32410_e47344;
        locals.var_guard1065_rv = 0.0;
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) {
            let assign32420_e47356: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32420_e47357: f64 = (1.0 / assign32420_e47356);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32420_e47357, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32420_e47356)) / (assign32420_e47356 * assign32420_e47356))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
            let assign32430_e47372: f64 = (-p.p183);
            let assign32430_e47373: f64 = (locals.var_arg__blk1055).powf(assign32430_e47372);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32430_e47373, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32430_e47372) as f64).is_finite() && ((assign32430_e47372) as f64).fract() == 0.0 { if assign32430_e47372 == 0.0 { 0.0 } else { (assign32430_e47372 * ((locals.var_arg__blk1055).powf(assign32430_e47372 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32430_e47373 * (assign32430_e47372 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1064 != 0.0)) {
            let assign32440_e47386: f64 = (p.p186 * locals.var_czbssw);
            let assign32440_e47390: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32440_e47391: f64 = (1.0 - assign32440_e47390);
            let assign32440_e47392: f64 = (assign32440_e47386 * assign32440_e47391);
            let assign32440_e47395: f64 = (1.0 - p.p183);
            let assign32440_e47396: f64 = (assign32440_e47392 / assign32440_e47395);
            let assign32440_e47397: f64 = (locals.var_qbs + assign32440_e47396);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32440_e47397, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32440_e47395)), (locals.var_qbs_dn7 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32440_e47395)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32440_e47386 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32440_e47395)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        let assign32450_e47402: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign32450_e47402;
        locals.var_guard1066_rv = 0.0;
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) {
            let assign32460_e47413: f64 = (locals.var_vbsj / p.p187);
            let assign32460_e47414: f64 = (1.0 - assign32460_e47413);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32460_e47414, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)), );
            locals.var_arg__blk1055_rv = 0.0;
        }
        let assign32470_e47419: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign32470_e47419;
        locals.var_guard1067_rv = 0.0;
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
            let assign32480_e47431: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32480_e47432: f64 = (1.0 / assign32480_e47431);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32480_e47432, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32480_e47431)) / (assign32480_e47431 * assign32480_e47431))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 == 0.0)) {
            let assign32490_e47447: f64 = (-p.p184);
            let assign32490_e47448: f64 = (locals.var_arg__blk1055).powf(assign32490_e47447);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32490_e47448, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32490_e47447) as f64).is_finite() && ((assign32490_e47447) as f64).fract() == 0.0 { if assign32490_e47447 == 0.0 { 0.0 } else { (assign32490_e47447 * ((locals.var_arg__blk1055).powf(assign32490_e47447 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32490_e47448 * (assign32490_e47447 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1066 != 0.0)) {
            let assign32500_e47461: f64 = (p.p187 * locals.var_czbsswg);
            let assign32500_e47465: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32500_e47466: f64 = (1.0 - assign32500_e47465);
            let assign32500_e47467: f64 = (assign32500_e47461 * assign32500_e47466);
            let assign32500_e47470: f64 = (1.0 - p.p184);
            let assign32500_e47471: f64 = (assign32500_e47467 / assign32500_e47470);
            let assign32500_e47472: f64 = (locals.var_qbs + assign32500_e47471);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32500_e47472, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32500_e47470)), (locals.var_qbs_dn7 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32500_e47470)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32500_e47461 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32500_e47470)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
            let assign32510_e47483: f64 = (locals.var_czbs + locals.var_czbssw);
            let assign32510_e47485: f64 = (assign32510_e47483 + locals.var_czbsswg);
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32510_e47485, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
            let assign32520_e47496: f64 = (locals.var_czbs * p.p182);
            let assign32520_e47498: f64 = (assign32520_e47496 / p.p185);
            let assign32520_e47501: f64 = (locals.var_czbssw * p.p183);
            let assign32520_e47503: f64 = (assign32520_e47501 / p.p186);
            let assign32520_e47504: f64 = (assign32520_e47498 + assign32520_e47503);
            let assign32520_e47507: f64 = (locals.var_czbsswg * p.p184);
            let assign32520_e47509: f64 = (assign32520_e47507 / p.p187);
            let assign32520_e47510: f64 = (assign32520_e47504 + assign32520_e47509);
            (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17, ) = (assign32520_e47510, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1031_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 != 0.0)) && (locals.var_guard1061 == 0.0)) {
            let assign32530_e47523: f64 = (locals.var_vbsj * 0.5);
            let assign32530_e47525: f64 = (assign32530_e47523 * locals.var_t2__blk1031);
            let assign32530_e47526: f64 = (locals.var_t1__blk1030 + assign32530_e47525);
            let assign32530_e47527: f64 = (locals.var_vbsj * assign32530_e47526);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32530_e47527, (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn0)), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn2)), (locals.var_vbsj * (locals.var_t1__blk1030_dn6 + (assign32530_e47523 * locals.var_t2__blk1031_dn6))), ((locals.var_vbsj_dn7 * assign32530_e47526) + (locals.var_vbsj * (locals.var_t1__blk1030_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1031) + (assign32530_e47523 * locals.var_t2__blk1031_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1030_dn10 + (assign32530_e47523 * locals.var_t2__blk1031_dn10))), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn11)), ((locals.var_vbsj_dn12 * assign32530_e47526) + (locals.var_vbsj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32530_e47523 * locals.var_t2__blk1031_dn12))))), (locals.var_vbsj * (assign32530_e47523 * locals.var_t2__blk1031_dn17)), );
            locals.var_qbs_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) {
            let assign32540_e47536: f64 = (p.p181 * p.p5);
            locals.var_czbsswg = assign32540_e47536;
            locals.var_czbsswg_rv = 0.0;
        }
        let assign32550_e47541: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign32550_e47541;
        locals.var_guard1068_rv = 0.0;
        let assign32560_e47544: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign32560_e47544;
        locals.var_guard1069_rv = 0.0;
    }
    pub(super) fn stamp_reactive_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
            let assign32570_e47556: f64 = (locals.var_vbsj / p.p185);
            let assign32570_e47557: f64 = (1.0 - assign32570_e47556);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32570_e47557, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)), );
            locals.var_arg__blk1055_rv = 0.0;
        }
        let assign32580_e47562: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign32580_e47562;
        locals.var_guard1070_rv = 0.0;
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 != 0.0)) {
            let assign32590_e47575: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32590_e47576: f64 = (1.0 / assign32590_e47575);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32590_e47576, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32590_e47575)) / (assign32590_e47575 * assign32590_e47575))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 == 0.0)) {
            let assign32600_e47592: f64 = (-p.p182);
            let assign32600_e47593: f64 = (locals.var_arg__blk1055).powf(assign32600_e47592);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32600_e47593, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32600_e47592) as f64).is_finite() && ((assign32600_e47592) as f64).fract() == 0.0 { if assign32600_e47592 == 0.0 { 0.0 } else { (assign32600_e47592 * ((locals.var_arg__blk1055).powf(assign32600_e47592 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32600_e47593 * (assign32600_e47592 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
            let assign32610_e47606: f64 = (p.p185 * locals.var_czbs);
            let assign32610_e47610: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32610_e47611: f64 = (1.0 - assign32610_e47610);
            let assign32610_e47612: f64 = (assign32610_e47606 * assign32610_e47611);
            let assign32610_e47615: f64 = (1.0 - p.p182);
            let assign32610_e47616: f64 = (assign32610_e47612 / assign32610_e47615);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32610_e47616, 0.0, 0.0, ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32610_e47615), ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32610_e47615), 0.0, 0.0, ((assign32610_e47606 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32610_e47615), 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 == 0.0)) {
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        let assign32630_e47633: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign32630_e47633;
        locals.var_guard1071_rv = 0.0;
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) {
            let assign32640_e47645: f64 = (locals.var_vbsj / p.p187);
            let assign32640_e47646: f64 = (1.0 - assign32640_e47645);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32640_e47646, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)), );
            locals.var_arg__blk1055_rv = 0.0;
        }
        let assign32650_e47651: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign32650_e47651;
        locals.var_guard1072_rv = 0.0;
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 != 0.0)) {
            let assign32660_e47664: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32660_e47665: f64 = (1.0 / assign32660_e47664);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32660_e47665, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32660_e47664)) / (assign32660_e47664 * assign32660_e47664))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) && (locals.var_guard1072 == 0.0)) {
            let assign32670_e47681: f64 = (-p.p184);
            let assign32670_e47682: f64 = (locals.var_arg__blk1055).powf(assign32670_e47681);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32670_e47682, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32670_e47681) as f64).is_finite() && ((assign32670_e47681) as f64).fract() == 0.0 { if assign32670_e47681 == 0.0 { 0.0 } else { (assign32670_e47681 * ((locals.var_arg__blk1055).powf(assign32670_e47681 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32670_e47682 * (assign32670_e47681 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1071 != 0.0)) {
            let assign32680_e47696: f64 = (p.p187 * locals.var_czbsswg);
            let assign32680_e47700: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32680_e47701: f64 = (1.0 - assign32680_e47700);
            let assign32680_e47702: f64 = (assign32680_e47696 * assign32680_e47701);
            let assign32680_e47705: f64 = (1.0 - p.p184);
            let assign32680_e47706: f64 = (assign32680_e47702 / assign32680_e47705);
            let assign32680_e47707: f64 = (locals.var_qbs + assign32680_e47706);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32680_e47707, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32680_e47705)), (locals.var_qbs_dn7 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32680_e47705)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32680_e47696 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32680_e47705)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
            let assign32690_e47719: f64 = (locals.var_czbs + locals.var_czbsswg);
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32690_e47719, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
            let assign32700_e47731: f64 = (locals.var_czbs * p.p182);
            let assign32700_e47733: f64 = (assign32700_e47731 / p.p185);
            let assign32700_e47736: f64 = (locals.var_czbsswg * p.p184);
            let assign32700_e47738: f64 = (assign32700_e47736 / p.p187);
            let assign32700_e47739: f64 = (assign32700_e47733 + assign32700_e47738);
            (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17, ) = (assign32700_e47739, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1031_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1060 == 0.0)) && (locals.var_guard1068 == 0.0)) {
            let assign32710_e47753: f64 = (locals.var_vbsj * 0.5);
            let assign32710_e47755: f64 = (assign32710_e47753 * locals.var_t2__blk1031);
            let assign32710_e47756: f64 = (locals.var_t1__blk1030 + assign32710_e47755);
            let assign32710_e47757: f64 = (locals.var_vbsj * assign32710_e47756);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32710_e47757, (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn0)), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn2)), (locals.var_vbsj * (locals.var_t1__blk1030_dn6 + (assign32710_e47753 * locals.var_t2__blk1031_dn6))), ((locals.var_vbsj_dn7 * assign32710_e47756) + (locals.var_vbsj * (locals.var_t1__blk1030_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1031) + (assign32710_e47753 * locals.var_t2__blk1031_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1030_dn10 + (assign32710_e47753 * locals.var_t2__blk1031_dn10))), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn11)), ((locals.var_vbsj_dn12 * assign32710_e47756) + (locals.var_vbsj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32710_e47753 * locals.var_t2__blk1031_dn12))))), (locals.var_vbsj * (assign32710_e47753 * locals.var_t2__blk1031_dn17)), );
            locals.var_qbs_rv = 0.0;
        }
        let assign32720_e47762: f64 = if p.p4 > locals.var_w_diodcv { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign32720_e47762;
        locals.var_guard1073_rv = 0.0;
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) {
            let assign32730_e47769: f64 = (p.p4 - locals.var_w_diodcv);
            let assign32730_e47770: f64 = (p.p180 * assign32730_e47769);
            locals.var_czbdsw = assign32730_e47770;
            locals.var_czbdsw_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) {
            let assign32740_e47778: f64 = (p.p181 * locals.var_w_diodcv);
            locals.var_czbdswg = assign32740_e47778;
            locals.var_czbdswg_rv = 0.0;
        }
        let assign32750_e47783: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign32750_e47783;
        locals.var_guard1074_rv = 0.0;
        let assign32760_e47786: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign32760_e47786;
        locals.var_guard1075_rv = 0.0;
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
            let assign32770_e47797: f64 = (locals.var_vbdj / p.p185);
            let assign32770_e47798: f64 = (1.0 - assign32770_e47797);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32770_e47798, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)), );
            locals.var_arg__blk1055_rv = 0.0;
        }
        let assign32780_e47803: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign32780_e47803;
        locals.var_guard1076_rv = 0.0;
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) {
            let assign32790_e47815: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32790_e47816: f64 = (1.0 / assign32790_e47815);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32790_e47816, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32790_e47815)) / (assign32790_e47815 * assign32790_e47815))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
            let assign32800_e47831: f64 = (-p.p182);
            let assign32800_e47832: f64 = (locals.var_arg__blk1055).powf(assign32800_e47831);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32800_e47832, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32800_e47831) as f64).is_finite() && ((assign32800_e47831) as f64).fract() == 0.0 { if assign32800_e47831 == 0.0 { 0.0 } else { (assign32800_e47831 * ((locals.var_arg__blk1055).powf(assign32800_e47831 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32800_e47832 * (assign32800_e47831 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
            let assign32810_e47844: f64 = (p.p185 * locals.var_czbd);
            let assign32810_e47848: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32810_e47849: f64 = (1.0 - assign32810_e47848);
            let assign32810_e47850: f64 = (assign32810_e47844 * assign32810_e47849);
            let assign32810_e47853: f64 = (1.0 - p.p182);
            let assign32810_e47854: f64 = (assign32810_e47850 / assign32810_e47853);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32810_e47854, 0.0, 0.0, ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32810_e47853), ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32810_e47853), 0.0, 0.0, ((assign32810_e47844 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32810_e47853), 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 == 0.0)) {
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        let assign32830_e47870: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign32830_e47870;
        locals.var_guard1077_rv = 0.0;
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) {
            let assign32840_e47881: f64 = (locals.var_vbdj / p.p186);
            let assign32840_e47882: f64 = (1.0 - assign32840_e47881);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32840_e47882, (-(locals.var_vbdj_dn6 / p.p186)), 0.0, (-(locals.var_vbdj_dn12 / p.p186)), );
            locals.var_arg__blk1055_rv = 0.0;
        }
        let assign32850_e47887: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign32850_e47887;
        locals.var_guard1078_rv = 0.0;
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) {
            let assign32860_e47899: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32860_e47900: f64 = (1.0 / assign32860_e47899);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32860_e47900, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32860_e47899)) / (assign32860_e47899 * assign32860_e47899))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
            let assign32870_e47915: f64 = (-p.p183);
            let assign32870_e47916: f64 = (locals.var_arg__blk1055).powf(assign32870_e47915);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32870_e47916, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32870_e47915) as f64).is_finite() && ((assign32870_e47915) as f64).fract() == 0.0 { if assign32870_e47915 == 0.0 { 0.0 } else { (assign32870_e47915 * ((locals.var_arg__blk1055).powf(assign32870_e47915 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32870_e47916 * (assign32870_e47915 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1077 != 0.0)) {
            let assign32880_e47929: f64 = (p.p186 * locals.var_czbdsw);
            let assign32880_e47933: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32880_e47934: f64 = (1.0 - assign32880_e47933);
            let assign32880_e47935: f64 = (assign32880_e47929 * assign32880_e47934);
            let assign32880_e47938: f64 = (1.0 - p.p183);
            let assign32880_e47939: f64 = (assign32880_e47935 / assign32880_e47938);
            let assign32880_e47940: f64 = (locals.var_qbd + assign32880_e47939);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32880_e47940, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32880_e47938)), (locals.var_qbd_dn7 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32880_e47938)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32880_e47929 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32880_e47938)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }
        let assign32890_e47945: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign32890_e47945;
        locals.var_guard1079_rv = 0.0;
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) {
            let assign32900_e47956: f64 = (locals.var_vbdj / p.p187);
            let assign32900_e47957: f64 = (1.0 - assign32900_e47956);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign32900_e47957, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)), );
            locals.var_arg__blk1055_rv = 0.0;
        }
        let assign32910_e47962: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign32910_e47962;
        locals.var_guard1080_rv = 0.0;
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 != 0.0)) {
            let assign32920_e47974: f64 = (locals.var_arg__blk1055).sqrt();
            let assign32920_e47975: f64 = (1.0 / assign32920_e47974);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32920_e47975, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign32920_e47974)) / (assign32920_e47974 * assign32920_e47974))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 == 0.0)) {
            let assign32930_e47990: f64 = (-p.p184);
            let assign32930_e47991: f64 = (locals.var_arg__blk1055).powf(assign32930_e47990);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32930_e47991, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign32930_e47990) as f64).is_finite() && ((assign32930_e47990) as f64).fract() == 0.0 { if assign32930_e47990 == 0.0 { 0.0 } else { (assign32930_e47990 * ((locals.var_arg__blk1055).powf(assign32930_e47990 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign32930_e47991 * (assign32930_e47990 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1079 != 0.0)) {
            let assign32940_e48004: f64 = (p.p187 * locals.var_czbdswg);
            let assign32940_e48008: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign32940_e48009: f64 = (1.0 - assign32940_e48008);
            let assign32940_e48010: f64 = (assign32940_e48004 * assign32940_e48009);
            let assign32940_e48013: f64 = (1.0 - p.p184);
            let assign32940_e48014: f64 = (assign32940_e48010 / assign32940_e48013);
            let assign32940_e48015: f64 = (locals.var_qbd + assign32940_e48014);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32940_e48015, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign32940_e48013)), (locals.var_qbd_dn7 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign32940_e48013)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32940_e48004 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign32940_e48013)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
            let assign32950_e48026: f64 = (locals.var_czbd + locals.var_czbdsw);
            let assign32950_e48028: f64 = (assign32950_e48026 + locals.var_czbdswg);
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign32950_e48028, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
            let assign32960_e48039: f64 = (locals.var_czbd * p.p182);
            let assign32960_e48041: f64 = (assign32960_e48039 / p.p185);
            let assign32960_e48044: f64 = (locals.var_czbdsw * p.p183);
            let assign32960_e48046: f64 = (assign32960_e48044 / p.p186);
            let assign32960_e48047: f64 = (assign32960_e48041 + assign32960_e48046);
            let assign32960_e48050: f64 = (locals.var_czbdswg * p.p184);
            let assign32960_e48052: f64 = (assign32960_e48050 / p.p187);
            let assign32960_e48053: f64 = (assign32960_e48047 + assign32960_e48052);
            (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17, ) = (assign32960_e48053, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1031_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
            let assign32970_e48066: f64 = (locals.var_vbdj * 0.5);
            let assign32970_e48068: f64 = (assign32970_e48066 * locals.var_t2__blk1031);
            let assign32970_e48069: f64 = (locals.var_t1__blk1030 + assign32970_e48068);
            let assign32970_e48070: f64 = (locals.var_vbdj * assign32970_e48069);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32970_e48070, (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn0)), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn2)), ((locals.var_vbdj_dn6 * assign32970_e48069) + (locals.var_vbdj * (locals.var_t1__blk1030_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1031) + (assign32970_e48066 * locals.var_t2__blk1031_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1030_dn7 + (assign32970_e48066 * locals.var_t2__blk1031_dn7))), (locals.var_vbdj * (locals.var_t1__blk1030_dn10 + (assign32970_e48066 * locals.var_t2__blk1031_dn10))), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn11)), ((locals.var_vbdj_dn12 * assign32970_e48069) + (locals.var_vbdj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign32970_e48066 * locals.var_t2__blk1031_dn12))))), (locals.var_vbdj * (assign32970_e48066 * locals.var_t2__blk1031_dn17)), );
            locals.var_qbd_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) {
            let assign32980_e48079: f64 = (p.p181 * p.p4);
            locals.var_czbdswg = assign32980_e48079;
            locals.var_czbdswg_rv = 0.0;
        }
        let assign32990_e48084: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign32990_e48084;
        locals.var_guard1081_rv = 0.0;
        let assign33000_e48087: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign33000_e48087;
        locals.var_guard1082_rv = 0.0;
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
            let assign33010_e48099: f64 = (locals.var_vbdj / p.p185);
            let assign33010_e48100: f64 = (1.0 - assign33010_e48099);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign33010_e48100, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)), );
            locals.var_arg__blk1055_rv = 0.0;
        }
        let assign33020_e48105: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign33020_e48105;
        locals.var_guard1083_rv = 0.0;
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 != 0.0)) {
            let assign33030_e48118: f64 = (locals.var_arg__blk1055).sqrt();
            let assign33030_e48119: f64 = (1.0 / assign33030_e48118);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33030_e48119, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign33030_e48118)) / (assign33030_e48118 * assign33030_e48118))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) && (locals.var_guard1083 == 0.0)) {
            let assign33040_e48135: f64 = (-p.p182);
            let assign33040_e48136: f64 = (locals.var_arg__blk1055).powf(assign33040_e48135);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33040_e48136, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33040_e48135) as f64).is_finite() && ((assign33040_e48135) as f64).fract() == 0.0 { if assign33040_e48135 == 0.0 { 0.0 } else { (assign33040_e48135 * ((locals.var_arg__blk1055).powf(assign33040_e48135 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign33040_e48136 * (assign33040_e48135 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
            let assign33050_e48149: f64 = (p.p185 * locals.var_czbd);
            let assign33050_e48153: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign33050_e48154: f64 = (1.0 - assign33050_e48153);
            let assign33050_e48155: f64 = (assign33050_e48149 * assign33050_e48154);
            let assign33050_e48158: f64 = (1.0 - p.p182);
            let assign33050_e48159: f64 = (assign33050_e48155 / assign33050_e48158);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33050_e48159, 0.0, 0.0, ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign33050_e48158), ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign33050_e48158), 0.0, 0.0, ((assign33050_e48149 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign33050_e48158), 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 == 0.0)) {
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        let assign33070_e48176: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign33070_e48176;
        locals.var_guard1084_rv = 0.0;
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) {
            let assign33080_e48188: f64 = (locals.var_vbdj / p.p187);
            let assign33080_e48189: f64 = (1.0 - assign33080_e48188);
            (locals.var_arg__blk1055, locals.var_arg__blk1055_dn6, locals.var_arg__blk1055_dn7, locals.var_arg__blk1055_dn12, ) = (assign33080_e48189, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)), );
            locals.var_arg__blk1055_rv = 0.0;
        }
        let assign33090_e48194: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1085 = assign33090_e48194;
        locals.var_guard1085_rv = 0.0;
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 != 0.0)) {
            let assign33100_e48207: f64 = (locals.var_arg__blk1055).sqrt();
            let assign33100_e48208: f64 = (1.0 / assign33100_e48207);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33100_e48208, (-((locals.var_arg__blk1055_dn6 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))), (-((locals.var_arg__blk1055_dn7 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))), (-((locals.var_arg__blk1055_dn12 / (2.0 * assign33100_e48207)) / (assign33100_e48207 * assign33100_e48207))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) && (locals.var_guard1085 == 0.0)) {
            let assign33110_e48224: f64 = (-p.p184);
            let assign33110_e48225: f64 = (locals.var_arg__blk1055).powf(assign33110_e48224);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33110_e48225, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn6)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn6 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn7)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn7 / locals.var_arg__blk1055))) }, if 0.0 == 0.0 && ((assign33110_e48224) as f64).is_finite() && ((assign33110_e48224) as f64).fract() == 0.0 { if assign33110_e48224 == 0.0 { 0.0 } else { (assign33110_e48224 * ((locals.var_arg__blk1055).powf(assign33110_e48224 - 1.0) * locals.var_arg__blk1055_dn12)) } } else { (assign33110_e48225 * (assign33110_e48224 * (locals.var_arg__blk1055_dn12 / locals.var_arg__blk1055))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1084 != 0.0)) {
            let assign33120_e48239: f64 = (p.p187 * locals.var_czbdswg);
            let assign33120_e48243: f64 = (locals.var_arg__blk1055 * locals.var_sarg);
            let assign33120_e48244: f64 = (1.0 - assign33120_e48243);
            let assign33120_e48245: f64 = (assign33120_e48239 * assign33120_e48244);
            let assign33120_e48248: f64 = (1.0 - p.p184);
            let assign33120_e48249: f64 = (assign33120_e48245 / assign33120_e48248);
            let assign33120_e48250: f64 = (locals.var_qbd + assign33120_e48249);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33120_e48250, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn6 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn6)))) / assign33120_e48248)), (locals.var_qbd_dn7 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn7 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn7)))) / assign33120_e48248)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign33120_e48239 * (-((locals.var_arg__blk1055_dn12 * locals.var_sarg) + (locals.var_arg__blk1055 * locals.var_sarg_dn12)))) / assign33120_e48248)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
            let assign33130_e48262: f64 = (locals.var_czbd + locals.var_czbdswg);
            (locals.var_t1__blk1030, locals.var_t1__blk1030_dn6, locals.var_t1__blk1030_dn7, locals.var_t1__blk1030_dn10, locals.var_t1__blk1030_dn12, ) = (assign33130_e48262, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1030_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
            let assign33140_e48274: f64 = (locals.var_czbd * p.p182);
            let assign33140_e48276: f64 = (assign33140_e48274 / p.p185);
            let assign33140_e48279: f64 = (locals.var_czbdswg * p.p184);
            let assign33140_e48281: f64 = (assign33140_e48279 / p.p187);
            let assign33140_e48282: f64 = (assign33140_e48276 + assign33140_e48281);
            (locals.var_t2__blk1031, locals.var_t2__blk1031_dn0, locals.var_t2__blk1031_dn2, locals.var_t2__blk1031_dn6, locals.var_t2__blk1031_dn7, locals.var_t2__blk1031_dn10, locals.var_t2__blk1031_dn11, locals.var_t2__blk1031_dn12, locals.var_t2__blk1031_dn17, ) = (assign33140_e48282, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1031_rv = 0.0;
        }
        if (((locals.var_guard1028 != 0.0) && (locals.var_guard1073 == 0.0)) && (locals.var_guard1081 == 0.0)) {
            let assign33150_e48296: f64 = (locals.var_vbdj * 0.5);
            let assign33150_e48298: f64 = (assign33150_e48296 * locals.var_t2__blk1031);
            let assign33150_e48299: f64 = (locals.var_t1__blk1030 + assign33150_e48298);
            let assign33150_e48300: f64 = (locals.var_vbdj * assign33150_e48299);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33150_e48300, (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn0)), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn2)), ((locals.var_vbdj_dn6 * assign33150_e48299) + (locals.var_vbdj * (locals.var_t1__blk1030_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1031) + (assign33150_e48296 * locals.var_t2__blk1031_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1030_dn7 + (assign33150_e48296 * locals.var_t2__blk1031_dn7))), (locals.var_vbdj * (locals.var_t1__blk1030_dn10 + (assign33150_e48296 * locals.var_t2__blk1031_dn10))), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn11)), ((locals.var_vbdj_dn12 * assign33150_e48299) + (locals.var_vbdj * (locals.var_t1__blk1030_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1031) + (assign33150_e48296 * locals.var_t2__blk1031_dn12))))), (locals.var_vbdj * (assign33150_e48296 * locals.var_t2__blk1031_dn17)), );
            locals.var_qbd_rv = 0.0;
        }
        let assign33160_e48305: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1086 = assign33160_e48305;
        locals.var_guard1086_rv = 0.0;
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33170_e48310: f64 = (-1.6021918e-19);
            let assign33170_e48312: f64 = (assign33170_e48310 * locals.var_uc_nsubs);
            let assign33170_e48314: f64 = (assign33170_e48312 * locals.var_xp_max);
            let assign33170_e48316: f64 = (assign33170_e48314 * p.p3);
            (locals.var_qbs_max, locals.var_qbs_max_dn0, locals.var_qbs_max_dn2, locals.var_qbs_max_dn6, locals.var_qbs_max_dn7, locals.var_qbs_max_dn10, locals.var_qbs_max_dn11, locals.var_qbs_max_dn12, locals.var_qbs_max_dn17, ) = (assign33170_e48316, (((assign33170_e48310 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p3), (((assign33170_e48310 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p3), );
            locals.var_qbs_max_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33180_e48324: f64 = (-locals.var_qbs_max);
            let assign33180_e48325: f64 = (0.001 * assign33180_e48324);
            (locals.var_dlt_qbs, locals.var_dlt_qbs_dn0, locals.var_dlt_qbs_dn2, locals.var_dlt_qbs_dn6, locals.var_dlt_qbs_dn7, locals.var_dlt_qbs_dn10, locals.var_dlt_qbs_dn11, locals.var_dlt_qbs_dn12, locals.var_dlt_qbs_dn17, ) = (assign33180_e48325, (0.001 * (-locals.var_qbs_max_dn0)), (0.001 * (-locals.var_qbs_max_dn2)), (0.001 * (-locals.var_qbs_max_dn6)), (0.001 * (-locals.var_qbs_max_dn7)), (0.001 * (-locals.var_qbs_max_dn10)), (0.001 * (-locals.var_qbs_max_dn11)), (0.001 * (-locals.var_qbs_max_dn12)), (0.001 * (-locals.var_qbs_max_dn17)), );
            locals.var_dlt_qbs_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33190_e48332: f64 = (-locals.var_qbs_max);
            let assign33190_e48334: f64 = (-locals.var_qbs);
            let assign33190_e48335: f64 = (assign33190_e48332 - assign33190_e48334);
            let assign33190_e48337: f64 = (assign33190_e48335 - locals.var_dlt_qbs);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign33190_e48337, (((-locals.var_qbs_max_dn0) - (-locals.var_qbs_dn0)) - locals.var_dlt_qbs_dn0), (((-locals.var_qbs_max_dn2) - (-locals.var_qbs_dn2)) - locals.var_dlt_qbs_dn2), (((-locals.var_qbs_max_dn6) - (-locals.var_qbs_dn6)) - locals.var_dlt_qbs_dn6), (((-locals.var_qbs_max_dn7) - (-locals.var_qbs_dn7)) - locals.var_dlt_qbs_dn7), (((-locals.var_qbs_max_dn10) - (-locals.var_qbs_dn10)) - locals.var_dlt_qbs_dn10), (((-locals.var_qbs_max_dn11) - (-locals.var_qbs_dn11)) - locals.var_dlt_qbs_dn11), (((-locals.var_qbs_max_dn12) - (-locals.var_qbs_dn12)) - locals.var_dlt_qbs_dn12), (((-locals.var_qbs_max_dn17) - (-locals.var_qbs_dn17)) - locals.var_dlt_qbs_dn17), );
            locals.var_tmf1_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33200_e48345: f64 = (-locals.var_qbs_max);
            let assign33200_e48346: f64 = (4.0 * assign33200_e48345);
            let assign33200_e48348: f64 = (assign33200_e48346 * locals.var_dlt_qbs);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33200_e48348, (((4.0 * (-locals.var_qbs_max_dn0)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn0)), (((4.0 * (-locals.var_qbs_max_dn2)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn2)), (((4.0 * (-locals.var_qbs_max_dn6)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn6)), (((4.0 * (-locals.var_qbs_max_dn7)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn7)), (((4.0 * (-locals.var_qbs_max_dn10)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn10)), (((4.0 * (-locals.var_qbs_max_dn11)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn11)), (((4.0 * (-locals.var_qbs_max_dn12)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn12)), (((4.0 * (-locals.var_qbs_max_dn17)) * locals.var_dlt_qbs) + (assign33200_e48346 * locals.var_dlt_qbs_dn17)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let (assign33210_e48360, assign33210_e48360_d_n0, assign33210_e48360_d_n2, assign33210_e48360_d_n6, assign33210_e48360_d_n7, assign33210_e48360_d_n10, assign33210_e48360_d_n11, assign33210_e48360_d_n12, assign33210_e48360_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign33210_e48359: f64 = (-locals.var_tmf2);
        (assign33210_e48359, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33210_e48360, assign33210_e48360_d_n0, assign33210_e48360_d_n2, assign33210_e48360_d_n6, assign33210_e48360_d_n7, assign33210_e48360_d_n10, assign33210_e48360_d_n11, assign33210_e48360_d_n12, assign33210_e48360_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33220_e48368: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign33220_e48370: f64 = (assign33220_e48368 + locals.var_tmf2);
            let assign33220_e48371: f64 = (assign33220_e48370).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33220_e48371, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33220_e48371)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33220_e48371)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33230_e48378: f64 = (-locals.var_qbs_max);
            let assign33230_e48382: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign33230_e48383: f64 = (0.5 * assign33230_e48382);
            let assign33230_e48384: f64 = (assign33230_e48378 - assign33230_e48383);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign33230_e48384, ((-locals.var_qbs_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbs_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbs_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbs_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbs_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbs_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbs_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbs_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_qbs_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1086 != 0.0)) {
            let assign33240_e48392: f64 = (-1.0);
            let assign33240_e48393: f64 = (locals.var_qbs * assign33240_e48392);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign33240_e48393, (locals.var_qbs_dn0 * assign33240_e48392), (locals.var_qbs_dn2 * assign33240_e48392), (locals.var_qbs_dn6 * assign33240_e48392), (locals.var_qbs_dn7 * assign33240_e48392), (locals.var_qbs_dn10 * assign33240_e48392), (locals.var_qbs_dn11 * assign33240_e48392), (locals.var_qbs_dn12 * assign33240_e48392), (locals.var_qbs_dn17 * assign33240_e48392), );
            locals.var_qbs_rv = 0.0;
        }
        let assign33250_e48398: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1087 = assign33250_e48398;
        locals.var_guard1087_rv = 0.0;
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33260_e48403: f64 = (-1.6021918e-19);
            let assign33260_e48405: f64 = (assign33260_e48403 * locals.var_uc_nsubs);
            let assign33260_e48407: f64 = (assign33260_e48405 * locals.var_xp_max);
            let assign33260_e48409: f64 = (assign33260_e48407 * p.p2);
            (locals.var_qbd_max, locals.var_qbd_max_dn0, locals.var_qbd_max_dn2, locals.var_qbd_max_dn6, locals.var_qbd_max_dn7, locals.var_qbd_max_dn10, locals.var_qbd_max_dn11, locals.var_qbd_max_dn12, locals.var_qbd_max_dn17, ) = (assign33260_e48409, (((assign33260_e48403 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p2), (((assign33260_e48403 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p2), );
            locals.var_qbd_max_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33270_e48417: f64 = (-locals.var_qbd_max);
            let assign33270_e48418: f64 = (0.001 * assign33270_e48417);
            (locals.var_dlt_qbd, locals.var_dlt_qbd_dn0, locals.var_dlt_qbd_dn2, locals.var_dlt_qbd_dn6, locals.var_dlt_qbd_dn7, locals.var_dlt_qbd_dn10, locals.var_dlt_qbd_dn11, locals.var_dlt_qbd_dn12, locals.var_dlt_qbd_dn17, ) = (assign33270_e48418, (0.001 * (-locals.var_qbd_max_dn0)), (0.001 * (-locals.var_qbd_max_dn2)), (0.001 * (-locals.var_qbd_max_dn6)), (0.001 * (-locals.var_qbd_max_dn7)), (0.001 * (-locals.var_qbd_max_dn10)), (0.001 * (-locals.var_qbd_max_dn11)), (0.001 * (-locals.var_qbd_max_dn12)), (0.001 * (-locals.var_qbd_max_dn17)), );
            locals.var_dlt_qbd_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33280_e48425: f64 = (-locals.var_qbd_max);
            let assign33280_e48427: f64 = (-locals.var_qbd);
            let assign33280_e48428: f64 = (assign33280_e48425 - assign33280_e48427);
            let assign33280_e48430: f64 = (assign33280_e48428 - locals.var_dlt_qbd);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign33280_e48430, (((-locals.var_qbd_max_dn0) - (-locals.var_qbd_dn0)) - locals.var_dlt_qbd_dn0), (((-locals.var_qbd_max_dn2) - (-locals.var_qbd_dn2)) - locals.var_dlt_qbd_dn2), (((-locals.var_qbd_max_dn6) - (-locals.var_qbd_dn6)) - locals.var_dlt_qbd_dn6), (((-locals.var_qbd_max_dn7) - (-locals.var_qbd_dn7)) - locals.var_dlt_qbd_dn7), (((-locals.var_qbd_max_dn10) - (-locals.var_qbd_dn10)) - locals.var_dlt_qbd_dn10), (((-locals.var_qbd_max_dn11) - (-locals.var_qbd_dn11)) - locals.var_dlt_qbd_dn11), (((-locals.var_qbd_max_dn12) - (-locals.var_qbd_dn12)) - locals.var_dlt_qbd_dn12), (((-locals.var_qbd_max_dn17) - (-locals.var_qbd_dn17)) - locals.var_dlt_qbd_dn17), );
            locals.var_tmf1_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33290_e48438: f64 = (-locals.var_qbd_max);
            let assign33290_e48439: f64 = (4.0 * assign33290_e48438);
            let assign33290_e48441: f64 = (assign33290_e48439 * locals.var_dlt_qbd);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33290_e48441, (((4.0 * (-locals.var_qbd_max_dn0)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn0)), (((4.0 * (-locals.var_qbd_max_dn2)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn2)), (((4.0 * (-locals.var_qbd_max_dn6)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn6)), (((4.0 * (-locals.var_qbd_max_dn7)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn7)), (((4.0 * (-locals.var_qbd_max_dn10)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn10)), (((4.0 * (-locals.var_qbd_max_dn11)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn11)), (((4.0 * (-locals.var_qbd_max_dn12)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn12)), (((4.0 * (-locals.var_qbd_max_dn17)) * locals.var_dlt_qbd) + (assign33290_e48439 * locals.var_dlt_qbd_dn17)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let (assign33300_e48453, assign33300_e48453_d_n0, assign33300_e48453_d_n2, assign33300_e48453_d_n6, assign33300_e48453_d_n7, assign33300_e48453_d_n10, assign33300_e48453_d_n11, assign33300_e48453_d_n12, assign33300_e48453_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign33300_e48452: f64 = (-locals.var_tmf2);
        (assign33300_e48452, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33300_e48453, assign33300_e48453_d_n0, assign33300_e48453_d_n2, assign33300_e48453_d_n6, assign33300_e48453_d_n7, assign33300_e48453_d_n10, assign33300_e48453_d_n11, assign33300_e48453_d_n12, assign33300_e48453_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33310_e48461: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign33310_e48463: f64 = (assign33310_e48461 + locals.var_tmf2);
            let assign33310_e48464: f64 = (assign33310_e48463).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33310_e48464, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33310_e48464)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33310_e48464)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33320_e48471: f64 = (-locals.var_qbd_max);
            let assign33320_e48475: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign33320_e48476: f64 = (0.5 * assign33320_e48475);
            let assign33320_e48477: f64 = (assign33320_e48471 - assign33320_e48476);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33320_e48477, ((-locals.var_qbd_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbd_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbd_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbd_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbd_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbd_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbd_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbd_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_qbd_rv = 0.0;
        }
        if ((locals.var_guard1028 != 0.0) && (locals.var_guard1087 != 0.0)) {
            let assign33330_e48485: f64 = (-1.0);
            let assign33330_e48486: f64 = (locals.var_qbd * assign33330_e48485);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33330_e48486, (locals.var_qbd_dn0 * assign33330_e48485), (locals.var_qbd_dn2 * assign33330_e48485), (locals.var_qbd_dn6 * assign33330_e48485), (locals.var_qbd_dn7 * assign33330_e48485), (locals.var_qbd_dn10 * assign33330_e48485), (locals.var_qbd_dn11 * assign33330_e48485), (locals.var_qbd_dn12 * assign33330_e48485), (locals.var_qbd_dn17 * assign33330_e48485), );
            locals.var_qbd_rv = 0.0;
        }
        let assign33890_e49080: f64 = (locals.var_ids + locals.var_idsibpc);
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (assign33890_e49080, (locals.var_ids_dn0 + locals.var_idsibpc_dn0), (locals.var_ids_dn2 + locals.var_idsibpc_dn2), (locals.var_ids_dn6 + locals.var_idsibpc_dn6), (locals.var_ids_dn7 + locals.var_idsibpc_dn7), (locals.var_ids_dn10 + locals.var_idsibpc_dn10), (locals.var_ids_dn11 + locals.var_idsibpc_dn11), (locals.var_ids_dn12 + locals.var_idsibpc_dn12), (locals.var_ids_dn17 + locals.var_idsibpc_dn17), );
        locals.var_ids_rv = 0.0;
        let assign33900_e49083: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1124 = assign33900_e49083;
        locals.var_guard1124_rv = 0.0;
        if (locals.var_guard1124 != 0.0) {
            let assign33910_e49087: f64 = (locals.var_cbtp + locals.var_cbtn);
            locals.var_cgbe = assign33910_e49087;
            locals.var_cgbe_rv = 0.0;
        }
        if ((locals.var_guard1124 != 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign33920_e49096: f64 = (p.p168 * locals.var_lgleff);
            let assign33920_e49097: f64 = (locals.var_cgbe - assign33920_e49096);
            locals.var_cgbe = assign33920_e49097;
            locals.var_cgbe_rv = 0.0;
        }
        if (locals.var_guard1124 != 0.0) {
            let assign33930_e49102: f64 = (-locals.var_cgbe);
            let assign33930_e49105: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign33930_e49106: f64 = (assign33930_e49102 * assign33930_e49105);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign33930_e49106, (assign33930_e49102 * (-locals.var_vbsp_dn0)), (assign33930_e49102 * (-locals.var_vbsp_dn2)), (assign33930_e49102 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33930_e49102 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33930_e49102 * (-locals.var_vbsp_dn10)), (assign33930_e49102 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33930_e49102 * (-locals.var_vbsp_dn12)), (assign33930_e49102 * (-locals.var_vbsp_dn17)), );
            locals.var_qgob_rv = 0.0;
        }
        if (locals.var_guard1124 != 0.0) {
            locals.var_cfu = 0.0;
            locals.var_cfu_rv = 0.0;
        }
        if (locals.var_guard1124 != 0.0) {
            let assign33950_e49122: f64 = (locals.var_cfu * p.p9);
            let assign33950_e49125: f64 = (locals.var_wgate + locals.var_uc_pdbcp);
            let assign33950_e49126: f64 = (assign33950_e49122 * assign33950_e49125);
            locals.var_cfd = assign33950_e49126;
            locals.var_cfd_rv = 0.0;
        }
        if (locals.var_guard1124 != 0.0) {
            let assign33960_e49132: f64 = (locals.var_cfu * p.p9);
            let assign33960_e49135: f64 = (locals.var_wgate + locals.var_uc_psbcp);
            let assign33960_e49136: f64 = (assign33960_e49132 * assign33960_e49135);
            locals.var_cfs = assign33960_e49136;
            locals.var_cfs_rv = 0.0;
        }
        if (locals.var_guard1124 != 0.0) {
            let assign33970_e49143: f64 = (locals.var_vgs - locals.var_vds);
            let assign33970_e49144: f64 = (locals.var_cfd * assign33970_e49143);
            (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17, ) = (assign33970_e49144, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)), );
            locals.var_qfd_rv = 0.0;
        }
        if (locals.var_guard1124 != 0.0) {
            let assign33980_e49150: f64 = (locals.var_cfs * locals.var_vgs);
            (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11, ) = (assign33980_e49150, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11), );
            locals.var_qfs_rv = 0.0;
        }
        if (locals.var_guard1124 != 0.0) {
            let assign33990_e49156: f64 = (locals.var_cfu * p.p19);
            let assign33990_e49158: f64 = (assign33990_e49156 * p.p9);
            let assign33990_e49161: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign33990_e49162: f64 = (assign33990_e49158 * assign33990_e49161);
            (locals.var_qfbc, locals.var_qfbc_dn0, locals.var_qfbc_dn2, locals.var_qfbc_dn6, locals.var_qfbc_dn7, locals.var_qfbc_dn10, locals.var_qfbc_dn11, locals.var_qfbc_dn12, locals.var_qfbc_dn17, ) = (assign33990_e49162, (assign33990_e49158 * (-locals.var_vbsp_dn0)), (assign33990_e49158 * (-locals.var_vbsp_dn2)), (assign33990_e49158 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33990_e49158 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33990_e49158 * (-locals.var_vbsp_dn10)), (assign33990_e49158 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33990_e49158 * (-locals.var_vbsp_dn12)), (assign33990_e49158 * (-locals.var_vbsp_dn17)), );
            locals.var_qfbc_rv = 0.0;
        }
        if (locals.var_guard1124 != 0.0) {
            let assign34000_e49168: f64 = (locals.var_qgod + locals.var_qfd);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign34000_e49168, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17), );
            locals.var_qgod_rv = 0.0;
        }
        if (locals.var_guard1124 != 0.0) {
            let assign34010_e49174: f64 = (locals.var_qgos + locals.var_qfs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign34010_e49174, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17, );
            locals.var_qgos_rv = 0.0;
        }
        if (locals.var_guard1124 != 0.0) {
            let assign34020_e49180: f64 = (locals.var_qgob + locals.var_qfbc);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign34020_e49180, (locals.var_qgob_dn0 + locals.var_qfbc_dn0), (locals.var_qgob_dn2 + locals.var_qfbc_dn2), (locals.var_qgob_dn6 + locals.var_qfbc_dn6), (locals.var_qgob_dn7 + locals.var_qfbc_dn7), (locals.var_qgob_dn10 + locals.var_qfbc_dn10), (locals.var_qgob_dn11 + locals.var_qfbc_dn11), (locals.var_qgob_dn12 + locals.var_qfbc_dn12), (locals.var_qgob_dn17 + locals.var_qfbc_dn17), );
            locals.var_qgob_rv = 0.0;
        }
        if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign34030_e49188: f64 = (-p.p168);
            let assign34030_e49190: f64 = (assign34030_e49188 * locals.var_lgleff);
            locals.var_cgbe = assign34030_e49190;
            locals.var_cgbe_rv = 0.0;
        }
        if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign34040_e49198: f64 = (-locals.var_cgbe);
            let assign34040_e49201: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign34040_e49202: f64 = (assign34040_e49198 * assign34040_e49201);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign34040_e49202, (assign34040_e49198 * (-locals.var_vbsp_dn0)), (assign34040_e49198 * (-locals.var_vbsp_dn2)), (assign34040_e49198 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34040_e49198 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34040_e49198 * (-locals.var_vbsp_dn10)), (assign34040_e49198 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34040_e49198 * (-locals.var_vbsp_dn12)), (assign34040_e49198 * (-locals.var_vbsp_dn17)), );
            locals.var_qgob_rv = 0.0;
        }
        if ((locals.var_guard1124 == 0.0) && (locals.var_cgbo_given == 0.0)) {
            locals.var_cgbe = 0.0;
            locals.var_cgbe_rv = 0.0;
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qgob_rv = 0.0;
        }
        if (locals.var_guard1124 == 0.0) {
            locals.var_cf = 0.0;
            locals.var_cf_rv = 0.0;
            locals.var_cfd = locals.var_cf;
            locals.var_cfd_rv = 0.0;
            locals.var_cfs = locals.var_cf;
            locals.var_cfs_rv = 0.0;
        }
        if (locals.var_guard1124 == 0.0) {
            let assign34100_e49251: f64 = (locals.var_vgs - locals.var_vds);
            let assign34100_e49252: f64 = (locals.var_cfd * assign34100_e49251);
            (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17, ) = (assign34100_e49252, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)), );
            locals.var_qfd_rv = 0.0;
        }
        if (locals.var_guard1124 == 0.0) {
            let assign34110_e49259: f64 = (locals.var_cfs * locals.var_vgs);
            (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11, ) = (assign34110_e49259, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11), );
            locals.var_qfs_rv = 0.0;
        }
        if (locals.var_guard1124 == 0.0) {
            let assign34120_e49266: f64 = (locals.var_qgod + locals.var_qfd);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign34120_e49266, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17), );
            locals.var_qgod_rv = 0.0;
        }
        if (locals.var_guard1124 == 0.0) {
            let assign34130_e49273: f64 = (locals.var_qgos + locals.var_qfs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign34130_e49273, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17, );
            locals.var_qgos_rv = 0.0;
        }
        let assign34140_e49278: f64 = (locals.var_mfactor * locals.var_ids);
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, ) = (assign34140_e49278, (locals.var_mfactor * locals.var_ids_dn0), (locals.var_mfactor * locals.var_ids_dn2), (locals.var_mfactor * locals.var_ids_dn6), (locals.var_mfactor * locals.var_ids_dn7), (locals.var_mfactor * locals.var_ids_dn10), (locals.var_mfactor * locals.var_ids_dn11), (locals.var_mfactor * locals.var_ids_dn12), (locals.var_mfactor * locals.var_ids_dn17), );
        locals.var_idse_rv = 0.0;
        if (locals.var_flg_nqs != 0.0) {
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qde_rv = 0.0;
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qge_rv = 0.0;
        }
        let assign34170_e49289: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1125 = assign34170_e49289;
        locals.var_guard1125_rv = 0.0;
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1125 != 0.0)) {
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qse_rv = 0.0;
            (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17, ) = (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, );
            locals.var_xd_rv = 0.0;
        }
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1125 == 0.0)) {
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbe_rv = 0.0;
        }
        let assign34260_e49360: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1126 = assign34260_e49360;
        locals.var_guard1126_rv = 0.0;
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
            let assign34270_e49367: f64 = (-locals.var_qb);
            let assign34270_e49369: f64 = (assign34270_e49367 - locals.var_qi);
            let assign34270_e49370: f64 = (locals.var_mfactor * assign34270_e49369);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34270_e49370, (locals.var_mfactor * ((-locals.var_qb_dn0) - locals.var_qi_dn0)), (locals.var_mfactor * ((-locals.var_qb_dn2) - locals.var_qi_dn2)), (locals.var_mfactor * ((-locals.var_qb_dn6) - locals.var_qi_dn6)), (locals.var_mfactor * ((-locals.var_qb_dn7) - locals.var_qi_dn7)), (locals.var_mfactor * ((-locals.var_qb_dn10) - locals.var_qi_dn10)), (locals.var_mfactor * ((-locals.var_qb_dn11) - locals.var_qi_dn11)), (locals.var_mfactor * ((-locals.var_qb_dn12) - locals.var_qi_dn12)), (locals.var_mfactor * (-locals.var_qb_dn13)), (locals.var_mfactor * (-locals.var_qb_dn15)), (locals.var_mfactor * (-locals.var_qb_dn16)), (locals.var_mfactor * ((-locals.var_qb_dn17) - locals.var_qi_dn17)), (locals.var_mfactor * (-locals.var_qb_dn18)), );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
            let assign34280_e49379: f64 = (locals.var_mfactor * locals.var_qd);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34280_e49379, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn12), (locals.var_mfactor * locals.var_qd_dn13), (locals.var_mfactor * locals.var_qd_dn15), (locals.var_mfactor * locals.var_qd_dn16), (locals.var_mfactor * locals.var_qd_dn17), (locals.var_mfactor * locals.var_qd_dn18), );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 != 0.0)) {
            let assign34290_e49389: f64 = (locals.var_qi - locals.var_qd);
            let assign34290_e49390: f64 = (locals.var_mfactor * assign34290_e49389);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34290_e49390, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn12 - locals.var_qd_dn12)), (locals.var_mfactor * (-locals.var_qd_dn13)), (locals.var_mfactor * (-locals.var_qd_dn15)), (locals.var_mfactor * (-locals.var_qd_dn16)), (locals.var_mfactor * (locals.var_qi_dn17 - locals.var_qd_dn17)), (locals.var_mfactor * (-locals.var_qd_dn18)), );
            locals.var_qse_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
            let assign34300_e49400: f64 = (-locals.var_qsub);
            let assign34300_e49402: f64 = (assign34300_e49400 - locals.var_qi);
            let assign34300_e49404: f64 = (assign34300_e49402 - locals.var_qs_fb);
            let assign34300_e49406: f64 = (assign34300_e49404 - locals.var_qd_fb);
            let assign34300_e49407: f64 = (locals.var_mfactor * assign34300_e49406);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34300_e49407, (locals.var_mfactor * ((((-locals.var_qsub_dn0) - locals.var_qi_dn0) - locals.var_qs_fb_dn0) - locals.var_qd_fb_dn0)), (locals.var_mfactor * ((((-locals.var_qsub_dn2) - locals.var_qi_dn2) - locals.var_qs_fb_dn2) - locals.var_qd_fb_dn2)), (locals.var_mfactor * ((((-locals.var_qsub_dn6) - locals.var_qi_dn6) - locals.var_qs_fb_dn6) - locals.var_qd_fb_dn6)), (locals.var_mfactor * ((((-locals.var_qsub_dn7) - locals.var_qi_dn7) - locals.var_qs_fb_dn7) - locals.var_qd_fb_dn7)), (locals.var_mfactor * ((((-locals.var_qsub_dn10) - locals.var_qi_dn10) - locals.var_qs_fb_dn10) - locals.var_qd_fb_dn10)), (locals.var_mfactor * ((((-locals.var_qsub_dn11) - locals.var_qi_dn11) - locals.var_qs_fb_dn11) - locals.var_qd_fb_dn11)), (locals.var_mfactor * ((((-locals.var_qsub_dn12) - locals.var_qi_dn12) - locals.var_qs_fb_dn12) - locals.var_qd_fb_dn12)), (locals.var_mfactor * ((-locals.var_qs_fb_dn13) - locals.var_qd_fb_dn13)), (locals.var_mfactor * ((-locals.var_qs_fb_dn15) - locals.var_qd_fb_dn15)), (locals.var_mfactor * ((-locals.var_qs_fb_dn16) - locals.var_qd_fb_dn16)), (locals.var_mfactor * ((((-locals.var_qsub_dn17) - locals.var_qi_dn17) - locals.var_qs_fb_dn17) - locals.var_qd_fb_dn17)), (locals.var_mfactor * ((-locals.var_qs_fb_dn18) - locals.var_qd_fb_dn18)), );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
            let assign34310_e49418: f64 = (locals.var_qd + locals.var_qd_fb);
            let assign34310_e49419: f64 = (locals.var_mfactor * assign34310_e49418);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34310_e49419, (locals.var_mfactor * (locals.var_qd_dn0 + locals.var_qd_fb_dn0)), (locals.var_mfactor * (locals.var_qd_dn2 + locals.var_qd_fb_dn2)), (locals.var_mfactor * (locals.var_qd_dn6 + locals.var_qd_fb_dn6)), (locals.var_mfactor * (locals.var_qd_dn7 + locals.var_qd_fb_dn7)), (locals.var_mfactor * (locals.var_qd_dn10 + locals.var_qd_fb_dn10)), (locals.var_mfactor * (locals.var_qd_dn11 + locals.var_qd_fb_dn11)), (locals.var_mfactor * (locals.var_qd_dn12 + locals.var_qd_fb_dn12)), (locals.var_mfactor * (locals.var_qd_dn13 + locals.var_qd_fb_dn13)), (locals.var_mfactor * (locals.var_qd_dn15 + locals.var_qd_fb_dn15)), (locals.var_mfactor * (locals.var_qd_dn16 + locals.var_qd_fb_dn16)), (locals.var_mfactor * (locals.var_qd_dn17 + locals.var_qd_fb_dn17)), (locals.var_mfactor * (locals.var_qd_dn18 + locals.var_qd_fb_dn18)), );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1126 == 0.0)) {
            let assign34320_e49430: f64 = (locals.var_qi - locals.var_qd);
            let assign34320_e49432: f64 = (assign34320_e49430 + locals.var_qs_fb);
            let assign34320_e49433: f64 = (locals.var_mfactor * assign34320_e49432);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34320_e49433, (locals.var_mfactor * ((locals.var_qi_dn0 - locals.var_qd_dn0) + locals.var_qs_fb_dn0)), (locals.var_mfactor * ((locals.var_qi_dn2 - locals.var_qd_dn2) + locals.var_qs_fb_dn2)), (locals.var_mfactor * ((locals.var_qi_dn6 - locals.var_qd_dn6) + locals.var_qs_fb_dn6)), (locals.var_mfactor * ((locals.var_qi_dn7 - locals.var_qd_dn7) + locals.var_qs_fb_dn7)), (locals.var_mfactor * ((locals.var_qi_dn10 - locals.var_qd_dn10) + locals.var_qs_fb_dn10)), (locals.var_mfactor * ((locals.var_qi_dn11 - locals.var_qd_dn11) + locals.var_qs_fb_dn11)), (locals.var_mfactor * ((locals.var_qi_dn12 - locals.var_qd_dn12) + locals.var_qs_fb_dn12)), (locals.var_mfactor * ((-locals.var_qd_dn13) + locals.var_qs_fb_dn13)), (locals.var_mfactor * ((-locals.var_qd_dn15) + locals.var_qs_fb_dn15)), (locals.var_mfactor * ((-locals.var_qd_dn16) + locals.var_qs_fb_dn16)), (locals.var_mfactor * ((locals.var_qi_dn17 - locals.var_qd_dn17) + locals.var_qs_fb_dn17)), (locals.var_mfactor * ((-locals.var_qd_dn18) + locals.var_qs_fb_dn18)), );
            locals.var_qse_rv = 0.0;
        }
        let assign34330_e49438: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1132 = assign34330_e49438;
        locals.var_guard1132_rv = 0.0;
        if (locals.var_guard1132 != 0.0) {
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qy_rv = 0.0;
        }
        if (locals.var_guard1132 == 0.0) {
            let assign34350_e49447: f64 = (locals.var_ec * locals.var_leff);
            let assign34350_e49449: f64 = (assign34350_e49447 + locals.var_ps0);
            (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17, ) = (assign34350_e49449, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn12 * locals.var_leff) + locals.var_ps0_dn12), ((locals.var_ec_dn17 * locals.var_leff) + locals.var_ps0_dn17), );
            locals.var_pslk_rv = 0.0;
        }
        let assign34360_e49454: f64 = if locals.var_pslk > locals.var_psdl { 1.0 } else { 0.0 };
        locals.var_guard1133 = assign34360_e49454;
        locals.var_guard1133_rv = 0.0;
        if ((locals.var_guard1132 == 0.0) && (locals.var_guard1133 != 0.0)) {
            (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17, ) = (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17, );
            locals.var_pslk_rv = 0.0;
        }
        if (locals.var_guard1132 == 0.0) {
            let assign34380_e49467: f64 = (locals.var_vds + locals.var_ps0);
            let assign34380_e49468: f64 = (locals.var_aclm * assign34380_e49467);
            let assign34380_e49471: f64 = (1.0 - locals.var_aclm);
            let assign34380_e49473: f64 = (assign34380_e49471 * locals.var_pslk);
            let assign34380_e49474: f64 = (assign34380_e49468 + assign34380_e49473);
            (locals.var_t1__blk1128, locals.var_t1__blk1128_dn0, locals.var_t1__blk1128_dn2, locals.var_t1__blk1128_dn6, locals.var_t1__blk1128_dn7, locals.var_t1__blk1128_dn10, locals.var_t1__blk1128_dn11, locals.var_t1__blk1128_dn12, locals.var_t1__blk1128_dn17, ) = (assign34380_e49474, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign34380_e49471 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign34380_e49471 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign34380_e49471 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign34380_e49471 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign34380_e49471 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign34380_e49471 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + (assign34380_e49471 * locals.var_pslk_dn12)), ((locals.var_aclm * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + (assign34380_e49471 * locals.var_pslk_dn17)), );
            locals.var_t1__blk1128_rv = 0.0;
        }
        if (locals.var_guard1132 == 0.0) {
            let assign34390_e49481: f64 = (2.0 * 1.034943e-10);
            let assign34390_e49483: f64 = (assign34390_e49481 / locals.var_q_nsub);
            let assign34390_e49484: f64 = (assign34390_e49483).sqrt();
            (locals.var_t10__blk1129, locals.var_t10__blk1129_dn0, locals.var_t10__blk1129_dn2, locals.var_t10__blk1129_dn6, locals.var_t10__blk1129_dn7, locals.var_t10__blk1129_dn10, locals.var_t10__blk1129_dn11, locals.var_t10__blk1129_dn12, locals.var_t10__blk1129_dn17, ) = (assign34390_e49484, ((-((assign34390_e49481 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), ((-((assign34390_e49481 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34390_e49484)), );
            locals.var_t10__blk1129_rv = 0.0;
        }
        if (locals.var_guard1132 == 0.0) {
            let assign34400_e49491: f64 = (locals.var_t10__blk1129 * 1.3);
            (locals.var_t3__blk1130, locals.var_t3__blk1130_dn0, locals.var_t3__blk1130_dn2, locals.var_t3__blk1130_dn6, locals.var_t3__blk1130_dn7, locals.var_t3__blk1130_dn10, locals.var_t3__blk1130_dn11, locals.var_t3__blk1130_dn12, locals.var_t3__blk1130_dn17, ) = (assign34400_e49491, (locals.var_t10__blk1129_dn0 * 1.3), (locals.var_t10__blk1129_dn2 * 1.3), (locals.var_t10__blk1129_dn6 * 1.3), (locals.var_t10__blk1129_dn7 * 1.3), (locals.var_t10__blk1129_dn10 * 1.3), (locals.var_t10__blk1129_dn11 * 1.3), (locals.var_t10__blk1129_dn12 * 1.3), (locals.var_t10__blk1129_dn17 * 1.3), );
            locals.var_t3__blk1130_rv = 0.0;
        }
        if (locals.var_guard1132 == 0.0) {
            let assign34410_e49498: f64 = (1.034943e-10 * locals.var_weffcv_nf);
            let assign34410_e49500: f64 = (assign34410_e49498 * locals.var_t3__blk1130);
            (locals.var_t2__blk1131, locals.var_t2__blk1131_dn0, locals.var_t2__blk1131_dn2, locals.var_t2__blk1131_dn6, locals.var_t2__blk1131_dn7, locals.var_t2__blk1131_dn10, locals.var_t2__blk1131_dn11, locals.var_t2__blk1131_dn12, locals.var_t2__blk1131_dn17, ) = (assign34410_e49500, (assign34410_e49498 * locals.var_t3__blk1130_dn0), (assign34410_e49498 * locals.var_t3__blk1130_dn2), (assign34410_e49498 * locals.var_t3__blk1130_dn6), (assign34410_e49498 * locals.var_t3__blk1130_dn7), (assign34410_e49498 * locals.var_t3__blk1130_dn10), (assign34410_e49498 * locals.var_t3__blk1130_dn11), (assign34410_e49498 * locals.var_t3__blk1130_dn12), (assign34410_e49498 * locals.var_t3__blk1130_dn17), );
            locals.var_t2__blk1131_rv = 0.0;
        }
        if (locals.var_guard1132 == 0.0) {
            let assign34420_e49507: f64 = (locals.var_ps0 + locals.var_vds);
            let assign34420_e49509: f64 = (assign34420_e49507 - locals.var_t1__blk1128);
            let assign34420_e49511: f64 = (assign34420_e49509 / p.p64);
            let assign34420_e49513: f64 = (assign34420_e49511 - locals.var_ec);
            let assign34420_e49515: f64 = (assign34420_e49513 * locals.var_t2__blk1131);
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (assign34420_e49515, ((((((locals.var_ps0_dn0 + locals.var_vds_dn0) - locals.var_t1__blk1128_dn0) / p.p64) - locals.var_ec_dn0) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn0)), ((((((locals.var_ps0_dn2 + locals.var_vds_dn2) - locals.var_t1__blk1128_dn2) / p.p64) - locals.var_ec_dn2) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn2)), ((((((locals.var_ps0_dn6 + locals.var_vds_dn6) - locals.var_t1__blk1128_dn6) / p.p64) - locals.var_ec_dn6) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn6)), ((((((locals.var_ps0_dn7 + locals.var_vds_dn7) - locals.var_t1__blk1128_dn7) / p.p64) - locals.var_ec_dn7) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn7)), ((((((locals.var_ps0_dn10 + locals.var_vds_dn10) - locals.var_t1__blk1128_dn10) / p.p64) - locals.var_ec_dn10) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn10)), ((((((locals.var_ps0_dn11 + locals.var_vds_dn11) - locals.var_t1__blk1128_dn11) / p.p64) - locals.var_ec_dn11) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn11)), ((((((locals.var_ps0_dn12 + locals.var_vds_dn12) - locals.var_t1__blk1128_dn12) / p.p64) - locals.var_ec_dn12) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn12)), ((((((locals.var_ps0_dn17 + locals.var_vds_dn17) - locals.var_t1__blk1128_dn17) / p.p64) - locals.var_ec_dn17) * locals.var_t2__blk1131) + (assign34420_e49513 * locals.var_t2__blk1131_dn17)), );
            locals.var_qy_rv = 0.0;
        }
        let assign34430_e49520: f64 = if p.p65 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1134 = assign34430_e49520;
        locals.var_guard1134_rv = 0.0;
        if (locals.var_guard1134 != 0.0) {
            let assign34440_e49525: f64 = (locals.var_cqyb0 * locals.var_vbsp);
            let assign34440_e49526: f64 = (locals.var_qy + assign34440_e49525);
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (assign34440_e49526, (locals.var_qy_dn0 + (locals.var_cqyb0 * locals.var_vbsp_dn0)), (locals.var_qy_dn2 + (locals.var_cqyb0 * locals.var_vbsp_dn2)), (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbsp_dn6)), (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbsp_dn7)), (locals.var_qy_dn10 + (locals.var_cqyb0 * locals.var_vbsp_dn10)), (locals.var_qy_dn11 + (locals.var_cqyb0 * locals.var_vbsp_dn11)), (locals.var_qy_dn12 + (locals.var_cqyb0 * locals.var_vbsp_dn12)), (locals.var_qy_dn17 + (locals.var_cqyb0 * locals.var_vbsp_dn17)), );
            locals.var_qy_rv = 0.0;
        }
        let assign34450_e49531: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1135 = assign34450_e49531;
        locals.var_guard1135_rv = 0.0;
        let assign34460_e49534: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1136 = assign34460_e49534;
        locals.var_guard1136_rv = 0.0;
        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34470_e49539: f64 = (-locals.var_qbody_bt_p_sus);
            let assign34470_e49541: f64 = (assign34470_e49539 - locals.var_qbody_bt_p_sud);
            let assign34470_e49543: f64 = (assign34470_e49541 - locals.var_qbody_bt_n_sus);
            let assign34470_e49545: f64 = (assign34470_e49543 - locals.var_qbody_bt_n_sud);
            (locals.var_q_bt_ge, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, locals.var_q_bt_ge_dn17, ) = (assign34470_e49545, ((((-locals.var_qbody_bt_p_sus_dn0) - locals.var_qbody_bt_p_sud_dn0) - locals.var_qbody_bt_n_sus_dn0) - locals.var_qbody_bt_n_sud_dn0), ((((-locals.var_qbody_bt_p_sus_dn2) - locals.var_qbody_bt_p_sud_dn2) - locals.var_qbody_bt_n_sus_dn2) - locals.var_qbody_bt_n_sud_dn2), ((((-locals.var_qbody_bt_p_sus_dn6) - locals.var_qbody_bt_p_sud_dn6) - locals.var_qbody_bt_n_sus_dn6) - locals.var_qbody_bt_n_sud_dn6), ((((-locals.var_qbody_bt_p_sus_dn7) - locals.var_qbody_bt_p_sud_dn7) - locals.var_qbody_bt_n_sus_dn7) - locals.var_qbody_bt_n_sud_dn7), ((((-locals.var_qbody_bt_p_sus_dn10) - locals.var_qbody_bt_p_sud_dn10) - locals.var_qbody_bt_n_sus_dn10) - locals.var_qbody_bt_n_sud_dn10), ((((-locals.var_qbody_bt_p_sus_dn11) - locals.var_qbody_bt_p_sud_dn11) - locals.var_qbody_bt_n_sus_dn11) - locals.var_qbody_bt_n_sud_dn11), ((((-locals.var_qbody_bt_p_sus_dn12) - locals.var_qbody_bt_p_sud_dn12) - locals.var_qbody_bt_n_sus_dn12) - locals.var_qbody_bt_n_sud_dn12), ((((-locals.var_qbody_bt_p_sus_dn17) - locals.var_qbody_bt_p_sud_dn17) - locals.var_qbody_bt_n_sus_dn17) - locals.var_qbody_bt_n_sud_dn17), );
            locals.var_q_bt_ge_rv = 0.0;
        }
        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34480_e49553: f64 = (locals.var_qbody_bt_p_iud + locals.var_qbody_bt_n_iud);
            (locals.var_q_bt_de, locals.var_q_bt_de_dn0, locals.var_q_bt_de_dn2, locals.var_q_bt_de_dn6, locals.var_q_bt_de_dn7, locals.var_q_bt_de_dn10, locals.var_q_bt_de_dn11, locals.var_q_bt_de_dn12, locals.var_q_bt_de_dn17, ) = (assign34480_e49553, (locals.var_qbody_bt_p_iud_dn0 + locals.var_qbody_bt_n_iud_dn0), (locals.var_qbody_bt_p_iud_dn2 + locals.var_qbody_bt_n_iud_dn2), (locals.var_qbody_bt_p_iud_dn6 + locals.var_qbody_bt_n_iud_dn6), (locals.var_qbody_bt_p_iud_dn7 + locals.var_qbody_bt_n_iud_dn7), (locals.var_qbody_bt_p_iud_dn10 + locals.var_qbody_bt_n_iud_dn10), (locals.var_qbody_bt_p_iud_dn11 + locals.var_qbody_bt_n_iud_dn11), (locals.var_qbody_bt_p_iud_dn12 + locals.var_qbody_bt_n_iud_dn12), (locals.var_qbody_bt_p_iud_dn17 + locals.var_qbody_bt_n_iud_dn17), );
            locals.var_q_bt_de_rv = 0.0;
        }
        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34490_e49561: f64 = (locals.var_qbody_bt_p_ius + locals.var_qbody_bt_n_ius);
            (locals.var_q_bt_se, locals.var_q_bt_se_dn0, locals.var_q_bt_se_dn2, locals.var_q_bt_se_dn6, locals.var_q_bt_se_dn7, locals.var_q_bt_se_dn10, locals.var_q_bt_se_dn11, locals.var_q_bt_se_dn12, locals.var_q_bt_se_dn17, ) = (assign34490_e49561, (locals.var_qbody_bt_p_ius_dn0 + locals.var_qbody_bt_n_ius_dn0), (locals.var_qbody_bt_p_ius_dn2 + locals.var_qbody_bt_n_ius_dn2), (locals.var_qbody_bt_p_ius_dn6 + locals.var_qbody_bt_n_ius_dn6), (locals.var_qbody_bt_p_ius_dn7 + locals.var_qbody_bt_n_ius_dn7), (locals.var_qbody_bt_p_ius_dn10 + locals.var_qbody_bt_n_ius_dn10), (locals.var_qbody_bt_p_ius_dn11 + locals.var_qbody_bt_n_ius_dn11), (locals.var_qbody_bt_p_ius_dn12 + locals.var_qbody_bt_n_ius_dn12), (locals.var_qbody_bt_p_ius_dn17 + locals.var_qbody_bt_n_ius_dn17), );
            locals.var_q_bt_se_rv = 0.0;
        }
        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34500_e49571: f64 = (locals.var_qgod + locals.var_qgos);
            let assign34500_e49573: f64 = (assign34500_e49571 + locals.var_qgob);
            let assign34500_e49575: f64 = (assign34500_e49573 - locals.var_qy);
            let assign34500_e49577: f64 = (assign34500_e49575 - locals.var_qovs);
            let assign34500_e49579: f64 = (assign34500_e49577 - locals.var_qovd);
            let assign34500_e49581: f64 = (assign34500_e49579 + locals.var_q_bt_ge);
            let assign34500_e49582: f64 = (locals.var_mfactor * assign34500_e49581);
            let assign34500_e49583: f64 = (locals.var_qge + assign34500_e49582);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34500_e49583, (locals.var_qge_dn0 + (locals.var_mfactor * ((((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0) + locals.var_q_bt_ge_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2) + locals.var_q_bt_ge_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * ((((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6) + locals.var_q_bt_ge_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7) + locals.var_q_bt_ge_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * ((((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10) + locals.var_q_bt_ge_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11) + locals.var_q_bt_ge_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * ((((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12) + locals.var_q_bt_ge_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * ((((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17) + locals.var_q_bt_ge_dn17))), locals.var_qge_dn18, );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34510_e49592: f64 = (-locals.var_qgod);
            let assign34510_e49594: f64 = (assign34510_e49592 + locals.var_qy);
            let assign34510_e49596: f64 = (assign34510_e49594 + locals.var_qbdld);
            let assign34510_e49598: f64 = (assign34510_e49596 + locals.var_q_bt_de);
            let assign34510_e49599: f64 = (locals.var_mfactor * assign34510_e49598);
            let assign34510_e49600: f64 = (locals.var_qde + assign34510_e49599);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34510_e49600, (locals.var_qde_dn0 + (locals.var_mfactor * ((((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0) + locals.var_q_bt_de_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2) + locals.var_q_bt_de_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * ((((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6) + locals.var_q_bt_de_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7) + locals.var_q_bt_de_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * ((((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10) + locals.var_q_bt_de_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11) + locals.var_q_bt_de_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * ((((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12) + locals.var_q_bt_de_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * ((((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17) + locals.var_q_bt_de_dn17))), locals.var_qde_dn18, );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 != 0.0)) {
            let assign34520_e49609: f64 = (-locals.var_qgos);
            let assign34520_e49611: f64 = (assign34520_e49609 + locals.var_qbsld);
            let assign34520_e49613: f64 = (assign34520_e49611 + locals.var_q_bt_se);
            let assign34520_e49614: f64 = (locals.var_mfactor * assign34520_e49613);
            let assign34520_e49615: f64 = (locals.var_qse + assign34520_e49614);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34520_e49615, (locals.var_qse_dn0 + (locals.var_mfactor * (((-locals.var_qgos_dn0) + locals.var_qbsld_dn0) + locals.var_q_bt_se_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * (((-locals.var_qgos_dn2) + locals.var_qbsld_dn2) + locals.var_q_bt_se_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * (((-locals.var_qgos_dn6) + locals.var_qbsld_dn6) + locals.var_q_bt_se_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * (((-locals.var_qgos_dn7) + locals.var_qbsld_dn7) + locals.var_q_bt_se_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * (((-locals.var_qgos_dn10) + locals.var_qbsld_dn10) + locals.var_q_bt_se_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * (((-locals.var_qgos_dn11) + locals.var_qbsld_dn11) + locals.var_q_bt_se_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * (((-locals.var_qgos_dn12) + locals.var_qbsld_dn12) + locals.var_q_bt_se_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * (((-locals.var_qgos_dn17) + locals.var_qbsld_dn17) + locals.var_q_bt_se_dn17))), locals.var_qse_dn18, );
            locals.var_qse_rv = 0.0;
        }
        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
            let assign34530_e49626: f64 = (locals.var_qgod + locals.var_qgos);
            let assign34530_e49628: f64 = (assign34530_e49626 + locals.var_qgob);
            let assign34530_e49630: f64 = (assign34530_e49628 - locals.var_qy);
            let assign34530_e49632: f64 = (assign34530_e49630 - locals.var_qovs);
            let assign34530_e49634: f64 = (assign34530_e49632 - locals.var_qovd);
            let assign34530_e49635: f64 = (locals.var_mfactor * assign34530_e49634);
            let assign34530_e49636: f64 = (locals.var_qge + assign34530_e49635);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34530_e49636, (locals.var_qge_dn0 + (locals.var_mfactor * (((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * (((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * (((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * (((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * (((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * (((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * (((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * (((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17))), locals.var_qge_dn18, );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
            let assign34540_e49646: f64 = (-locals.var_qgod);
            let assign34540_e49648: f64 = (assign34540_e49646 + locals.var_qy);
            let assign34540_e49650: f64 = (assign34540_e49648 + locals.var_qbdld);
            let assign34540_e49651: f64 = (locals.var_mfactor * assign34540_e49650);
            let assign34540_e49652: f64 = (locals.var_qde + assign34540_e49651);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34540_e49652, (locals.var_qde_dn0 + (locals.var_mfactor * (((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * (((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * (((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * (((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * (((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * (((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * (((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * (((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17))), locals.var_qde_dn18, );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_guard1135 != 0.0) && (locals.var_guard1136 == 0.0)) {
            let assign34550_e49662: f64 = (-locals.var_qgos);
            let assign34550_e49664: f64 = (assign34550_e49662 + locals.var_qbsld);
            let assign34550_e49665: f64 = (locals.var_mfactor * assign34550_e49664);
            let assign34550_e49666: f64 = (locals.var_qse + assign34550_e49665);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34550_e49666, (locals.var_qse_dn0 + (locals.var_mfactor * ((-locals.var_qgos_dn0) + locals.var_qbsld_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * ((-locals.var_qgos_dn2) + locals.var_qbsld_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * ((-locals.var_qgos_dn6) + locals.var_qbsld_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * ((-locals.var_qgos_dn7) + locals.var_qbsld_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * ((-locals.var_qgos_dn10) + locals.var_qbsld_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * ((-locals.var_qgos_dn11) + locals.var_qbsld_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * ((-locals.var_qgos_dn12) + locals.var_qbsld_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * ((-locals.var_qgos_dn17) + locals.var_qbsld_dn17))), locals.var_qse_dn18, );
            locals.var_qse_rv = 0.0;
        }
        let assign34580_e49673: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1137 = assign34580_e49673;
        locals.var_guard1137_rv = 0.0;
        if (locals.var_guard1137 != 0.0) {
            let assign34590_e49677: f64 = (locals.var_mfactor * locals.var_ibs);
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (assign34590_e49677, (locals.var_mfactor * locals.var_ibs_dn0), (locals.var_mfactor * locals.var_ibs_dn2), (locals.var_mfactor * locals.var_ibs_dn6), (locals.var_mfactor * locals.var_ibs_dn7), (locals.var_mfactor * locals.var_ibs_dn10), (locals.var_mfactor * locals.var_ibs_dn11), (locals.var_mfactor * locals.var_ibs_dn12), (locals.var_mfactor * locals.var_ibs_dn17), );
            locals.var_ibsb_rv = 0.0;
        }
        if (locals.var_guard1137 != 0.0) {
            let assign34600_e49683: f64 = (locals.var_mfactor * locals.var_ibd);
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (assign34600_e49683, (locals.var_mfactor * locals.var_ibd_dn0), (locals.var_mfactor * locals.var_ibd_dn2), (locals.var_mfactor * locals.var_ibd_dn6), (locals.var_mfactor * locals.var_ibd_dn7), (locals.var_mfactor * locals.var_ibd_dn10), (locals.var_mfactor * locals.var_ibd_dn11), (locals.var_mfactor * locals.var_ibd_dn12), (locals.var_mfactor * locals.var_ibd_dn17), );
            locals.var_ibdb_rv = 0.0;
        }
        if (locals.var_guard1137 != 0.0) {
            let assign34610_e49689: f64 = (locals.var_mfactor * locals.var_qbd);
            (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, ) = (assign34610_e49689, (locals.var_mfactor * locals.var_qbd_dn0), (locals.var_mfactor * locals.var_qbd_dn2), (locals.var_mfactor * locals.var_qbd_dn6), (locals.var_mfactor * locals.var_qbd_dn7), (locals.var_mfactor * locals.var_qbd_dn10), (locals.var_mfactor * locals.var_qbd_dn11), (locals.var_mfactor * locals.var_qbd_dn12), (locals.var_mfactor * locals.var_qbd_dn17), );
            locals.var_qbd_s0_rv = 0.0;
        }
        if (locals.var_guard1137 != 0.0) {
            let assign34620_e49695: f64 = (locals.var_mfactor * locals.var_qbs);
            (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, ) = (assign34620_e49695, (locals.var_mfactor * locals.var_qbs_dn0), (locals.var_mfactor * locals.var_qbs_dn2), (locals.var_mfactor * locals.var_qbs_dn6), (locals.var_mfactor * locals.var_qbs_dn7), (locals.var_mfactor * locals.var_qbs_dn10), (locals.var_mfactor * locals.var_qbs_dn11), (locals.var_mfactor * locals.var_qbs_dn12), (locals.var_mfactor * locals.var_qbs_dn17), );
            locals.var_qbs_s0_rv = 0.0;
        }
        if (locals.var_guard1137 == 0.0) {
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ibsb_rv = 0.0;
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ibdb_rv = 0.0;
            (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_s0_rv = 0.0;
            (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_s0_rv = 0.0;
        }
        let assign34670_e49720: f64 = if p.p25 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1138 = assign34670_e49720;
        locals.var_guard1138_rv = 0.0;
        if (locals.var_guard1138 != 0.0) {
            (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isube_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_53(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        if (locals.var_guard1138 == 0.0) {
            let assign34690_e49729: f64 = (locals.var_mfactor * locals.var_isub);
            (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17, ) = (assign34690_e49729, (locals.var_mfactor * locals.var_isub_dn0), (locals.var_mfactor * locals.var_isub_dn2), (locals.var_mfactor * locals.var_isub_dn6), (locals.var_mfactor * locals.var_isub_dn7), (locals.var_mfactor * locals.var_isub_dn10), (locals.var_mfactor * locals.var_isub_dn11), (locals.var_mfactor * locals.var_isub_dn12), (locals.var_mfactor * locals.var_isub_dn17), );
            locals.var_isube_rv = 0.0;
        }
        let assign35050_e49992: f64 = if p.p259 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1149 = assign35050_e49992;
        locals.var_guard1149_rv = 0.0;
        if (locals.var_guard1149 != 0.0) {
            locals.var_rdmod = 1.0;
            locals.var_rdmod_rv = 0.0;
        }
        let assign35070_e49999: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1169 = assign35070_e49999;
        locals.var_guard1169_rv = 0.0;
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
            locals.var_mks_rdrmue = p.p266;
            locals.var_mks_rdrmue_rv = 0.0;
            locals.var_mks_rdrvmax = p.p268;
            locals.var_mks_rdrvmax_rv = 0.0;
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (p.p273, 0.0, );
            locals.var_rrdrbb_rv = 0.0;
            locals.var_ldrifte = p.p258;
            locals.var_ldrifte_rv = 0.0;
        }
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 != 0.0)) {
            let assign35140_e50050: f64 = (p.p50 * (nv7 - nv2));
            (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7, ) = (assign35140_e50050, 0.0, (-p.p50), 0.0, p.p50, );
            locals.var_vrdr_rv = 0.0;
        }
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
            locals.var_mks_rdrmue = p.p265;
            locals.var_mks_rdrmue_rv = 0.0;
            locals.var_mks_rdrvmax = p.p267;
            locals.var_mks_rdrvmax_rv = 0.0;
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (p.p272, 0.0, );
            locals.var_rrdrbb_rv = 0.0;
            locals.var_ldrifte = p.p257;
            locals.var_ldrifte_rv = 0.0;
        }
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1169 == 0.0)) {
            let assign35210_e50110: f64 = (p.p50 * (nv0 - nv6));
            (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7, ) = (assign35210_e50110, p.p50, 0.0, (-p.p50), 0.0, );
            locals.var_vrdr_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35240_e50133: f64 = (locals.var_mks_rdrmue / 10000.0);
            locals.var_mks_rdrmue = assign35240_e50133;
            locals.var_mks_rdrmue_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35250_e50139: f64 = (locals.var_mks_rdrvmax / 100.0);
            locals.var_mks_rdrvmax = assign35250_e50139;
            locals.var_mks_rdrvmax_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35260_e50145: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_tratio, locals.var_tratio_dn10, ) = (assign35260_e50145, (locals.var_ttemp_dn10 / locals.var_uc_tnom), );
            locals.var_tratio_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35270_e50151: f64 = (locals.var_tratio).powf(p.p269);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35270_e50151, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio).powf(p.p269 - 1.0) * locals.var_tratio_dn10)) } } else { (assign35270_e50151 * (p.p269 * (locals.var_tratio_dn10 / locals.var_tratio))) }, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35280_e50157: f64 = (locals.var_mks_rdrmue / locals.var_t1);
            (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17, ) = (assign35280_e50157, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))), );
            locals.var_mu0_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35290_e50164: f64 = (0.4 * locals.var_tratio);
            let assign35290_e50165: f64 = (1.8 + assign35290_e50164);
            let assign35290_e50168: f64 = (0.1 * locals.var_tratio);
            let assign35290_e50170: f64 = (assign35290_e50168 * locals.var_tratio);
            let assign35290_e50171: f64 = (assign35290_e50165 + assign35290_e50170);
            let assign35290_e50175: f64 = (1.0 - locals.var_tratio);
            let assign35290_e50176: f64 = (p.p270 * assign35290_e50175);
            let assign35290_e50177: f64 = (assign35290_e50171 - assign35290_e50176);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17, ) = (assign35290_e50177, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign35290_e50168 * locals.var_tratio_dn10))) - (p.p270 * (-locals.var_tratio_dn10))), 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35300_e50183: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
            (locals.var_vmaxe__blk1162, locals.var_vmaxe__blk1162_dn0, locals.var_vmaxe__blk1162_dn2, locals.var_vmaxe__blk1162_dn6, locals.var_vmaxe__blk1162_dn7, locals.var_vmaxe__blk1162_dn10, locals.var_vmaxe__blk1162_dn11, locals.var_vmaxe__blk1162_dn12, locals.var_vmaxe__blk1162_dn17, ) = (assign35300_e50183, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))), );
            locals.var_vmaxe__blk1162_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35310_e50191: f64 = (locals.var_ttemp - locals.var_uc_tnom);
            let assign35310_e50192: f64 = (p.p274 * assign35310_e50191);
            let assign35310_e50193: f64 = (locals.var_rrdrbb + assign35310_e50192);
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (assign35310_e50193, (locals.var_rrdrbb_dn10 + (p.p274 * locals.var_ttemp_dn10)), );
            locals.var_rrdrbb_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35320_e50201: f64 = (locals.var_lgle).powf(p.p280);
            let assign35320_e50202: f64 = (p.p279 / assign35320_e50201);
            let assign35320_e50203: f64 = (1.0 + assign35320_e50202);
            locals.var_rdrmuele = assign35320_e50203;
            locals.var_rdrmuele_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35330_e50211: f64 = (locals.var_lgle).powf(p.p278);
            let assign35330_e50212: f64 = (p.p277 / assign35330_e50211);
            let assign35330_e50213: f64 = (1.0 + assign35330_e50212);
            locals.var_rdrvmaxle = assign35330_e50213;
            locals.var_rdrvmaxle_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35340_e50221: f64 = (locals.var_wg).powf(p.p276);
            let assign35340_e50222: f64 = (p.p275 / assign35340_e50221);
            let assign35340_e50223: f64 = (1.0 + assign35340_e50222);
            locals.var_rdrvmaxwe = assign35340_e50223;
            locals.var_rdrvmaxwe_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35350_e50229: f64 = (locals.var_mu0 * locals.var_rdrmuele);
            (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17, ) = (assign35350_e50229, (locals.var_mu0_dn0 * locals.var_rdrmuele), (locals.var_mu0_dn2 * locals.var_rdrmuele), (locals.var_mu0_dn6 * locals.var_rdrmuele), (locals.var_mu0_dn7 * locals.var_rdrmuele), (locals.var_mu0_dn10 * locals.var_rdrmuele), (locals.var_mu0_dn11 * locals.var_rdrmuele), (locals.var_mu0_dn12 * locals.var_rdrmuele), (locals.var_mu0_dn17 * locals.var_rdrmuele), );
            locals.var_mu0_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35360_e50235: f64 = (locals.var_vmaxe__blk1162 * locals.var_rdrvmaxwe);
            let assign35360_e50237: f64 = (assign35360_e50235 * locals.var_rdrvmaxle);
            let assign35360_e50239: f64 = (assign35360_e50237 + 1e-50);
            (locals.var_vmaxe__blk1162, locals.var_vmaxe__blk1162_dn0, locals.var_vmaxe__blk1162_dn2, locals.var_vmaxe__blk1162_dn6, locals.var_vmaxe__blk1162_dn7, locals.var_vmaxe__blk1162_dn10, locals.var_vmaxe__blk1162_dn11, locals.var_vmaxe__blk1162_dn12, locals.var_vmaxe__blk1162_dn17, ) = (assign35360_e50239, ((locals.var_vmaxe__blk1162_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn12 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1162_dn17 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), );
            locals.var_vmaxe__blk1162_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35370_e50245: f64 = (locals.var_vrdr / locals.var_ldrifte);
            (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn6, locals.var_edri_dn7, ) = (assign35370_e50245, (locals.var_vrdr_dn0 / locals.var_ldrifte), (locals.var_vrdr_dn2 / locals.var_ldrifte), (locals.var_vrdr_dn6 / locals.var_ldrifte), (locals.var_vrdr_dn7 / locals.var_ldrifte), );
            locals.var_edri_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35380_e50251: f64 = (locals.var_mu0 * locals.var_edri);
            (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn12, locals.var_vdri_dn17, ) = (assign35380_e50251, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), (locals.var_mu0_dn10 * locals.var_edri), (locals.var_mu0_dn11 * locals.var_edri), (locals.var_mu0_dn12 * locals.var_edri), (locals.var_mu0_dn17 * locals.var_edri), );
            locals.var_vdri_rv = 0.0;
        }
        let assign35390_e50256: f64 = if locals.var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1170 = assign35390_e50256;
        locals.var_guard1170_rv = 0.0;
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1170 != 0.0)) {
            let assign35400_e50262: f64 = (locals.var_vdri / locals.var_vmaxe__blk1162);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35400_e50262, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn0)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn2)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn6)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn7)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn10)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn11)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn12 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn12)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), (((locals.var_vdri_dn17 * locals.var_vmaxe__blk1162) - (locals.var_vdri * locals.var_vmaxe__blk1162_dn17)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1170 == 0.0)) {
            let assign35410_e50270: f64 = (-locals.var_vdri);
            let assign35410_e50272: f64 = (assign35410_e50270 / locals.var_vmaxe__blk1162);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35410_e50272, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn0)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn2)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn6)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn7)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn10)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn11)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn12) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn12)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), ((((-locals.var_vdri_dn17) * locals.var_vmaxe__blk1162) - (assign35410_e50270 * locals.var_vmaxe__blk1162_dn17)) / (locals.var_vmaxe__blk1162 * locals.var_vmaxe__blk1162)), );
            locals.var_t1_rv = 0.0;
        }
        let assign35420_e50278: f64 = (10.0 * 2.220446049250313e-16);
        let assign35420_e50279: f64 = (1.0 - assign35420_e50278);
        let assign35420_e50286: f64 = (10.0 * 2.220446049250313e-16);
        let assign35420_e50287: f64 = (1.0 + assign35420_e50286);
        let assign35420_e50289: f64 = if ((assign35420_e50279 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35420_e50287)) { 1.0 } else { 0.0 };
        locals.var_guard1171 = assign35420_e50289;
        locals.var_guard1171_rv = 0.0;
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1171 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t3_rv = 0.0;
        }
        let assign35440_e50299: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50300: f64 = (2.0 - assign35440_e50299);
        let assign35440_e50307: f64 = (10.0 * 2.220446049250313e-16);
        let assign35440_e50308: f64 = (2.0 + assign35440_e50307);
        let assign35440_e50310: f64 = if ((assign35440_e50300 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35440_e50308)) { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign35440_e50310;
        locals.var_guard1172_rv = 0.0;
        if (((locals.var_guard1149 != 0.0) && (locals.var_guard1171 == 0.0)) && (locals.var_guard1172 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, );
            locals.var_t3_rv = 0.0;
        }
        if (((locals.var_guard1149 != 0.0) && (locals.var_guard1171 == 0.0)) && (locals.var_guard1172 == 0.0)) {
            let assign35460_e50330: f64 = (locals.var_rrdrbb - 1.0);
            let assign35460_e50331: f64 = (locals.var_t1).powf(assign35460_e50330);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (assign35460_e50331, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn0)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn2)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn6)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn7)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb_dn10 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn10)) } } else { (assign35460_e50331 * ((locals.var_rrdrbb_dn10 * (locals.var_t1).ln()) + (assign35460_e50330 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn11)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn12)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35460_e50330) as f64).is_finite() && ((assign35460_e50330) as f64).fract() == 0.0 { if assign35460_e50330 == 0.0 { 0.0 } else { (assign35460_e50330 * ((locals.var_t1).powf(assign35460_e50330 - 1.0) * locals.var_t1_dn17)) } } else { (assign35460_e50331 * (assign35460_e50330 * (locals.var_t1_dn17 / locals.var_t1))) }, );
            locals.var_t3_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35470_e50337: f64 = (locals.var_t1 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, ) = (assign35470_e50337, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35480_e50343: f64 = (1.0 + locals.var_t2);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17, ) = (assign35480_e50343, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, );
            locals.var_t4_rv = 0.0;
        }
        let assign35490_e50349: f64 = (10.0 * 2.220446049250313e-16);
        let assign35490_e50350: f64 = (1.0 - assign35490_e50349);
        let assign35490_e50357: f64 = (10.0 * 2.220446049250313e-16);
        let assign35490_e50358: f64 = (1.0 + assign35490_e50357);
        let assign35490_e50360: f64 = if ((assign35490_e50350 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35490_e50358)) { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign35490_e50360;
        locals.var_guard1173_rv = 0.0;
        if ((locals.var_guard1149 != 0.0) && (locals.var_guard1173 != 0.0)) {
            let assign35500_e50366: f64 = (1.0 / locals.var_t4);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35500_e50366, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))), );
            locals.var_t5_rv = 0.0;
        }
        let assign35510_e50372: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50373: f64 = (2.0 - assign35510_e50372);
        let assign35510_e50380: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50381: f64 = (2.0 + assign35510_e50380);
        let assign35510_e50383: f64 = if ((assign35510_e50373 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35510_e50381)) { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign35510_e50383;
        locals.var_guard1174_rv = 0.0;
        if (((locals.var_guard1149 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 != 0.0)) {
            let assign35520_e50392: f64 = (locals.var_t4).sqrt();
            let assign35520_e50393: f64 = (1.0 / assign35520_e50392);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35520_e50393, (-((locals.var_t4_dn0 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn2 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn6 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn7 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn10 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn11 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn12 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), (-((locals.var_t4_dn17 / (2.0 * assign35520_e50392)) / (assign35520_e50392 * assign35520_e50392))), );
            locals.var_t5_rv = 0.0;
        }
        if (((locals.var_guard1149 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 == 0.0)) {
            let assign35530_e50405: f64 = (-1.0);
            let assign35530_e50407: f64 = (assign35530_e50405 / locals.var_rrdrbb);
            let assign35530_e50409: f64 = (assign35530_e50407 - 1.0);
            let assign35530_e50410: f64 = (locals.var_t4).powf(assign35530_e50409);
            (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17, ) = (assign35530_e50410, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn0)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn2)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn6)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn7)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign35530_e50405 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn10)) } } else { (assign35530_e50410 * (((-((assign35530_e50405 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) * (locals.var_t4).ln()) + (assign35530_e50409 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn11)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn12)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35530_e50409) as f64).is_finite() && ((assign35530_e50409) as f64).fract() == 0.0 { if assign35530_e50409 == 0.0 { 0.0 } else { (assign35530_e50409 * ((locals.var_t4).powf(assign35530_e50409 - 1.0) * locals.var_t4_dn17)) } } else { (assign35530_e50410 * (assign35530_e50409 * (locals.var_t4_dn17 / locals.var_t4))) }, );
            locals.var_t6_rv = 0.0;
        }
        if (((locals.var_guard1149 != 0.0) && (locals.var_guard1173 == 0.0)) && (locals.var_guard1174 == 0.0)) {
            let assign35540_e50422: f64 = (locals.var_t4 * locals.var_t6);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35540_e50422, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard1149 != 0.0) {
            let assign35560_e50434: f64 = (1.6021918e-19 / locals.var_ldrifte);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35560_e50434, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        let assign35680_e50510: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign35680_e50510;
        locals.var_guard1177_rv = 0.0;
        if (locals.var_guard1177 != 0.0) {
            locals.var_rdmod = 2.0;
            locals.var_rdmod_rv = 0.0;
        }
        let assign35700_e50517: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign35700_e50517;
        locals.var_guard1197_rv = 0.0;
        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 != 0.0)) {
            locals.var_mks_rdrmue__blk1181 = p.p266;
            locals.var_mks_rdrmue__blk1181_rv = 0.0;
            locals.var_mks_rdrvmax__blk1182 = p.p268;
            locals.var_mks_rdrvmax__blk1182_rv = 0.0;
            (locals.var_rrdrbb__blk1183, locals.var_rrdrbb__blk1183_dn10, ) = (p.p273, 0.0, );
            locals.var_rrdrbb__blk1183_rv = 0.0;
            locals.var_ldrifte__blk1187 = p.p258;
            locals.var_ldrifte__blk1187_rv = 0.0;
        }
        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 != 0.0)) {
            let assign35770_e50568: f64 = (p.p50 * (nv7 - nv2));
            (locals.var_vrdr__blk1185, locals.var_vrdr__blk1185_dn0, locals.var_vrdr__blk1185_dn2, locals.var_vrdr__blk1185_dn6, locals.var_vrdr__blk1185_dn7, ) = (assign35770_e50568, 0.0, (-p.p50), 0.0, p.p50, );
            locals.var_vrdr__blk1185_rv = 0.0;
        }
        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 == 0.0)) {
            locals.var_mks_rdrmue__blk1181 = p.p265;
            locals.var_mks_rdrmue__blk1181_rv = 0.0;
            locals.var_mks_rdrvmax__blk1182 = p.p267;
            locals.var_mks_rdrvmax__blk1182_rv = 0.0;
            (locals.var_rrdrbb__blk1183, locals.var_rrdrbb__blk1183_dn10, ) = (p.p272, 0.0, );
            locals.var_rrdrbb__blk1183_rv = 0.0;
            locals.var_ldrifte__blk1187 = p.p257;
            locals.var_ldrifte__blk1187_rv = 0.0;
        }
        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1197 == 0.0)) {
            let assign35840_e50628: f64 = (p.p50 * (nv0 - nv6));
            (locals.var_vrdr__blk1185, locals.var_vrdr__blk1185_dn0, locals.var_vrdr__blk1185_dn2, locals.var_vrdr__blk1185_dn6, locals.var_vrdr__blk1185_dn7, ) = (assign35840_e50628, p.p50, 0.0, (-p.p50), 0.0, );
            locals.var_vrdr__blk1185_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35870_e50651: f64 = (locals.var_mks_rdrmue__blk1181 / 10000.0);
            locals.var_mks_rdrmue__blk1181 = assign35870_e50651;
            locals.var_mks_rdrmue__blk1181_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35880_e50657: f64 = (locals.var_mks_rdrvmax__blk1182 / 100.0);
            locals.var_mks_rdrvmax__blk1182 = assign35880_e50657;
            locals.var_mks_rdrvmax__blk1182_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35890_e50663: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_tratio__blk1186, locals.var_tratio__blk1186_dn10, ) = (assign35890_e50663, (locals.var_ttemp_dn10 / locals.var_uc_tnom), );
            locals.var_tratio__blk1186_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35900_e50669: f64 = (locals.var_tratio__blk1186).powf(p.p269);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35900_e50669, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio__blk1186).powf(p.p269 - 1.0) * locals.var_tratio__blk1186_dn10)) } } else { (assign35900_e50669 * (p.p269 * (locals.var_tratio__blk1186_dn10 / locals.var_tratio__blk1186))) }, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35910_e50675: f64 = (locals.var_mks_rdrmue__blk1181 / locals.var_t1);
            (locals.var_mu0__blk1189, locals.var_mu0__blk1189_dn0, locals.var_mu0__blk1189_dn2, locals.var_mu0__blk1189_dn6, locals.var_mu0__blk1189_dn7, locals.var_mu0__blk1189_dn10, locals.var_mu0__blk1189_dn11, locals.var_mu0__blk1189_dn12, locals.var_mu0__blk1189_dn17, ) = (assign35910_e50675, (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1181 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))), );
            locals.var_mu0__blk1189_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35920_e50682: f64 = (0.4 * locals.var_tratio__blk1186);
            let assign35920_e50683: f64 = (1.8 + assign35920_e50682);
            let assign35920_e50686: f64 = (0.1 * locals.var_tratio__blk1186);
            let assign35920_e50688: f64 = (assign35920_e50686 * locals.var_tratio__blk1186);
            let assign35920_e50689: f64 = (assign35920_e50683 + assign35920_e50688);
            let assign35920_e50693: f64 = (1.0 - locals.var_tratio__blk1186);
            let assign35920_e50694: f64 = (p.p270 * assign35920_e50693);
            let assign35920_e50695: f64 = (assign35920_e50689 - assign35920_e50694);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17, ) = (assign35920_e50695, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio__blk1186_dn10) + (((0.1 * locals.var_tratio__blk1186_dn10) * locals.var_tratio__blk1186) + (assign35920_e50686 * locals.var_tratio__blk1186_dn10))) - (p.p270 * (-locals.var_tratio__blk1186_dn10))), 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35930_e50701: f64 = (locals.var_mks_rdrvmax__blk1182 / locals.var_t0);
            (locals.var_vmaxe__blk1190, locals.var_vmaxe__blk1190_dn0, locals.var_vmaxe__blk1190_dn2, locals.var_vmaxe__blk1190_dn6, locals.var_vmaxe__blk1190_dn7, locals.var_vmaxe__blk1190_dn10, locals.var_vmaxe__blk1190_dn11, locals.var_vmaxe__blk1190_dn12, locals.var_vmaxe__blk1190_dn17, ) = (assign35930_e50701, (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1182 * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))), );
            locals.var_vmaxe__blk1190_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35940_e50709: f64 = (locals.var_ttemp - locals.var_uc_tnom);
            let assign35940_e50710: f64 = (p.p274 * assign35940_e50709);
            let assign35940_e50711: f64 = (locals.var_rrdrbb__blk1183 + assign35940_e50710);
            (locals.var_rrdrbb__blk1183, locals.var_rrdrbb__blk1183_dn10, ) = (assign35940_e50711, (locals.var_rrdrbb__blk1183_dn10 + (p.p274 * locals.var_ttemp_dn10)), );
            locals.var_rrdrbb__blk1183_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35950_e50719: f64 = (locals.var_lgle).powf(p.p280);
            let assign35950_e50720: f64 = (p.p279 / assign35950_e50719);
            let assign35950_e50721: f64 = (1.0 + assign35950_e50720);
            locals.var_rdrmuele__blk1178 = assign35950_e50721;
            locals.var_rdrmuele__blk1178_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35960_e50729: f64 = (locals.var_lgle).powf(p.p278);
            let assign35960_e50730: f64 = (p.p277 / assign35960_e50729);
            let assign35960_e50731: f64 = (1.0 + assign35960_e50730);
            locals.var_rdrvmaxle__blk1180 = assign35960_e50731;
            locals.var_rdrvmaxle__blk1180_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35970_e50739: f64 = (locals.var_wg).powf(p.p276);
            let assign35970_e50740: f64 = (p.p275 / assign35970_e50739);
            let assign35970_e50741: f64 = (1.0 + assign35970_e50740);
            locals.var_rdrvmaxwe__blk1179 = assign35970_e50741;
            locals.var_rdrvmaxwe__blk1179_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35980_e50747: f64 = (locals.var_mu0__blk1189 * locals.var_rdrmuele__blk1178);
            (locals.var_mu0__blk1189, locals.var_mu0__blk1189_dn0, locals.var_mu0__blk1189_dn2, locals.var_mu0__blk1189_dn6, locals.var_mu0__blk1189_dn7, locals.var_mu0__blk1189_dn10, locals.var_mu0__blk1189_dn11, locals.var_mu0__blk1189_dn12, locals.var_mu0__blk1189_dn17, ) = (assign35980_e50747, (locals.var_mu0__blk1189_dn0 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn2 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn6 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn7 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn10 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn11 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn12 * locals.var_rdrmuele__blk1178), (locals.var_mu0__blk1189_dn17 * locals.var_rdrmuele__blk1178), );
            locals.var_mu0__blk1189_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign35990_e50753: f64 = (locals.var_vmaxe__blk1190 * locals.var_rdrvmaxwe__blk1179);
            let assign35990_e50755: f64 = (assign35990_e50753 * locals.var_rdrvmaxle__blk1180);
            let assign35990_e50757: f64 = (assign35990_e50755 + 1e-50);
            (locals.var_vmaxe__blk1190, locals.var_vmaxe__blk1190_dn0, locals.var_vmaxe__blk1190_dn2, locals.var_vmaxe__blk1190_dn6, locals.var_vmaxe__blk1190_dn7, locals.var_vmaxe__blk1190_dn10, locals.var_vmaxe__blk1190_dn11, locals.var_vmaxe__blk1190_dn12, locals.var_vmaxe__blk1190_dn17, ) = (assign35990_e50757, ((locals.var_vmaxe__blk1190_dn0 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn2 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn6 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn7 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn10 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn11 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn12 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), ((locals.var_vmaxe__blk1190_dn17 * locals.var_rdrvmaxwe__blk1179) * locals.var_rdrvmaxle__blk1180), );
            locals.var_vmaxe__blk1190_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign36000_e50763: f64 = (locals.var_vrdr__blk1185 / locals.var_ldrifte__blk1187);
            (locals.var_edri__blk1191, locals.var_edri__blk1191_dn0, locals.var_edri__blk1191_dn2, locals.var_edri__blk1191_dn6, locals.var_edri__blk1191_dn7, ) = (assign36000_e50763, (locals.var_vrdr__blk1185_dn0 / locals.var_ldrifte__blk1187), (locals.var_vrdr__blk1185_dn2 / locals.var_ldrifte__blk1187), (locals.var_vrdr__blk1185_dn6 / locals.var_ldrifte__blk1187), (locals.var_vrdr__blk1185_dn7 / locals.var_ldrifte__blk1187), );
            locals.var_edri__blk1191_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign36010_e50769: f64 = (locals.var_mu0__blk1189 * locals.var_edri__blk1191);
            (locals.var_vdri__blk1192, locals.var_vdri__blk1192_dn0, locals.var_vdri__blk1192_dn2, locals.var_vdri__blk1192_dn6, locals.var_vdri__blk1192_dn7, locals.var_vdri__blk1192_dn10, locals.var_vdri__blk1192_dn11, locals.var_vdri__blk1192_dn12, locals.var_vdri__blk1192_dn17, ) = (assign36010_e50769, ((locals.var_mu0__blk1189_dn0 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn0)), ((locals.var_mu0__blk1189_dn2 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn2)), ((locals.var_mu0__blk1189_dn6 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn6)), ((locals.var_mu0__blk1189_dn7 * locals.var_edri__blk1191) + (locals.var_mu0__blk1189 * locals.var_edri__blk1191_dn7)), (locals.var_mu0__blk1189_dn10 * locals.var_edri__blk1191), (locals.var_mu0__blk1189_dn11 * locals.var_edri__blk1191), (locals.var_mu0__blk1189_dn12 * locals.var_edri__blk1191), (locals.var_mu0__blk1189_dn17 * locals.var_edri__blk1191), );
            locals.var_vdri__blk1192_rv = 0.0;
        }
        let assign36020_e50774: f64 = if locals.var_vrdr__blk1185 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign36020_e50774;
        locals.var_guard1198_rv = 0.0;
        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1198 != 0.0)) {
            let assign36030_e50780: f64 = (locals.var_vdri__blk1192 / locals.var_vmaxe__blk1190);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36030_e50780, (((locals.var_vdri__blk1192_dn0 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn0)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn2 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn2)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn6 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn6)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn7 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn7)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn10 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn10)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn11 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn11)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn12 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn12)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), (((locals.var_vdri__blk1192_dn17 * locals.var_vmaxe__blk1190) - (locals.var_vdri__blk1192 * locals.var_vmaxe__blk1190_dn17)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1198 == 0.0)) {
            let assign36040_e50788: f64 = (-locals.var_vdri__blk1192);
            let assign36040_e50790: f64 = (assign36040_e50788 / locals.var_vmaxe__blk1190);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36040_e50790, ((((-locals.var_vdri__blk1192_dn0) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn0)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn2) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn2)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn6) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn6)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn7) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn7)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn10) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn10)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn11) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn11)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn12) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn12)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), ((((-locals.var_vdri__blk1192_dn17) * locals.var_vmaxe__blk1190) - (assign36040_e50788 * locals.var_vmaxe__blk1190_dn17)) / (locals.var_vmaxe__blk1190 * locals.var_vmaxe__blk1190)), );
            locals.var_t1_rv = 0.0;
        }
        let assign36050_e50796: f64 = (10.0 * 2.220446049250313e-16);
        let assign36050_e50797: f64 = (1.0 - assign36050_e50796);
        let assign36050_e50804: f64 = (10.0 * 2.220446049250313e-16);
        let assign36050_e50805: f64 = (1.0 + assign36050_e50804);
        let assign36050_e50807: f64 = if ((assign36050_e50797 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36050_e50805)) { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign36050_e50807;
        locals.var_guard1199_rv = 0.0;
        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1199 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t3_rv = 0.0;
        }
        let assign36070_e50817: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50818: f64 = (2.0 - assign36070_e50817);
        let assign36070_e50825: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50826: f64 = (2.0 + assign36070_e50825);
        let assign36070_e50828: f64 = if ((assign36070_e50818 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36070_e50826)) { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign36070_e50828;
        locals.var_guard1200_rv = 0.0;
        if (((locals.var_guard1177 != 0.0) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, );
            locals.var_t3_rv = 0.0;
        }
        if (((locals.var_guard1177 != 0.0) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
            let assign36090_e50848: f64 = (locals.var_rrdrbb__blk1183 - 1.0);
            let assign36090_e50849: f64 = (locals.var_t1).powf(assign36090_e50848);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (assign36090_e50849, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn0)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn2)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn6)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn7)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb__blk1183_dn10 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn10)) } } else { (assign36090_e50849 * ((locals.var_rrdrbb__blk1183_dn10 * (locals.var_t1).ln()) + (assign36090_e50848 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn11)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn12)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36090_e50848) as f64).is_finite() && ((assign36090_e50848) as f64).fract() == 0.0 { if assign36090_e50848 == 0.0 { 0.0 } else { (assign36090_e50848 * ((locals.var_t1).powf(assign36090_e50848 - 1.0) * locals.var_t1_dn17)) } } else { (assign36090_e50849 * (assign36090_e50848 * (locals.var_t1_dn17 / locals.var_t1))) }, );
            locals.var_t3_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1177 != 0.0) {
            let assign36100_e50855: f64 = (locals.var_t1 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, ) = (assign36100_e50855, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign36110_e50861: f64 = (1.0 + locals.var_t2);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17, ) = (assign36110_e50861, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, );
            locals.var_t4_rv = 0.0;
        }
        let assign36120_e50867: f64 = (10.0 * 2.220446049250313e-16);
        let assign36120_e50868: f64 = (1.0 - assign36120_e50867);
        let assign36120_e50875: f64 = (10.0 * 2.220446049250313e-16);
        let assign36120_e50876: f64 = (1.0 + assign36120_e50875);
        let assign36120_e50878: f64 = if ((assign36120_e50868 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36120_e50876)) { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign36120_e50878;
        locals.var_guard1201_rv = 0.0;
        if ((locals.var_guard1177 != 0.0) && (locals.var_guard1201 != 0.0)) {
            let assign36130_e50884: f64 = (1.0 / locals.var_t4);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36130_e50884, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))), );
            locals.var_t5_rv = 0.0;
        }
        let assign36140_e50890: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50891: f64 = (2.0 - assign36140_e50890);
        let assign36140_e50898: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50899: f64 = (2.0 + assign36140_e50898);
        let assign36140_e50901: f64 = if ((assign36140_e50891 <= locals.var_rrdrbb__blk1183) && (locals.var_rrdrbb__blk1183 <= assign36140_e50899)) { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign36140_e50901;
        locals.var_guard1202_rv = 0.0;
        if (((locals.var_guard1177 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
            let assign36150_e50910: f64 = (locals.var_t4).sqrt();
            let assign36150_e50911: f64 = (1.0 / assign36150_e50910);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36150_e50911, (-((locals.var_t4_dn0 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn2 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn6 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn7 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn10 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn11 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn12 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), (-((locals.var_t4_dn17 / (2.0 * assign36150_e50910)) / (assign36150_e50910 * assign36150_e50910))), );
            locals.var_t5_rv = 0.0;
        }
        if (((locals.var_guard1177 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
            let assign36160_e50923: f64 = (-1.0);
            let assign36160_e50925: f64 = (assign36160_e50923 / locals.var_rrdrbb__blk1183);
            let assign36160_e50927: f64 = (assign36160_e50925 - 1.0);
            let assign36160_e50928: f64 = (locals.var_t4).powf(assign36160_e50927);
            (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17, ) = (assign36160_e50928, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn0)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn2)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn6)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn7)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign36160_e50923 * locals.var_rrdrbb__blk1183_dn10) / (locals.var_rrdrbb__blk1183 * locals.var_rrdrbb__blk1183))) == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn10)) } } else { (assign36160_e50928 * (((-((assign36160_e50923 * locals.var_rrdrbb__blk1183_dn10) / (locals.var_rrdrbb__blk1183 * locals.var_rrdrbb__blk1183))) * (locals.var_t4).ln()) + (assign36160_e50927 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn11)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn12)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36160_e50927) as f64).is_finite() && ((assign36160_e50927) as f64).fract() == 0.0 { if assign36160_e50927 == 0.0 { 0.0 } else { (assign36160_e50927 * ((locals.var_t4).powf(assign36160_e50927 - 1.0) * locals.var_t4_dn17)) } } else { (assign36160_e50928 * (assign36160_e50927 * (locals.var_t4_dn17 / locals.var_t4))) }, );
            locals.var_t6_rv = 0.0;
        }
        if (((locals.var_guard1177 != 0.0) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
            let assign36170_e50940: f64 = (locals.var_t4 * locals.var_t6);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36170_e50940, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard1177 != 0.0) {
            let assign36190_e50952: f64 = (1.6021918e-19 / locals.var_ldrifte__blk1187);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36190_e50952, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        let assign36310_e51028: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign36310_e51028;
        locals.var_guard1205_rv = 0.0;
        if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let (assign36360_e51069, assign36360_e51069_d_n0, assign36360_e51069_d_n2, assign36360_e51069_d_n6, assign36360_e51069_d_n7, assign36360_e51069_d_n10, assign36360_e51069_d_n11, assign36360_e51069_d_n12, assign36360_e51069_d_n17,) = {
    if (locals.var_mode == 1.0) {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
    } else {
        let assign36360_e51068: f64 = (1.0 - locals.var_xd);
        (assign36360_e51068, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn12), (-locals.var_xd_dn17),)
    }
};
            (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, ) = (assign36360_e51069, assign36360_e51069_d_n0, assign36360_e51069_d_n2, assign36360_e51069_d_n6, assign36360_e51069_d_n7, assign36360_e51069_d_n10, assign36360_e51069_d_n11, assign36360_e51069_d_n12, assign36360_e51069_d_n17, );
            locals.var_qdrat_rv = 0.0;
        }
        if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36390_e51097: f64 = (locals.var_qi_nqs * locals.var_qdrat);
            let assign36390_e51099: f64 = (assign36390_e51097 + locals.var_q_bt_se);
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (assign36390_e51099, ((locals.var_qi_nqs * locals.var_qdrat_dn0) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * locals.var_qdrat_dn2) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * locals.var_qdrat_dn6) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * locals.var_qdrat_dn7) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * locals.var_qdrat_dn10) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * locals.var_qdrat_dn11) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * locals.var_qdrat_dn12) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * locals.var_qdrat_dn17) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * locals.var_qdrat), );
            locals.var_qd_nqs_rv = 0.0;
        }
        if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36400_e51108: f64 = (1.0 - locals.var_qdrat);
            let assign36400_e51109: f64 = (locals.var_qi_nqs * assign36400_e51108);
            let assign36400_e51111: f64 = (assign36400_e51109 + locals.var_q_bt_se);
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (assign36400_e51111, ((locals.var_qi_nqs * (-locals.var_qdrat_dn0)) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * (-locals.var_qdrat_dn2)) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * (-locals.var_qdrat_dn6)) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * (-locals.var_qdrat_dn7)) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * (-locals.var_qdrat_dn10)) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * (-locals.var_qdrat_dn11)) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * (-locals.var_qdrat_dn12)) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * (-locals.var_qdrat_dn17)) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * assign36400_e51108), );
            locals.var_qs_nqs_rv = 0.0;
        }
        if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36410_e51118: f64 = (-locals.var_qi_nqs);
            let assign36410_e51120: f64 = (assign36410_e51118 - locals.var_qb_nqs);
            let assign36410_e51122: f64 = (assign36410_e51120 + locals.var_q_bt_ge);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (assign36410_e51122, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, (-locals.var_qb_nqs_dn13), 0.0, 0.0, locals.var_q_bt_ge_dn17, (-locals.var_qi_nqs_dn18), );
            locals.var_qg_nqs_rv = 0.0;
        }
        if ((locals.var_guard1205 != 0.0) && (locals.var_flg_nqs == 0.0)) {
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qb_nqs, locals.var_qb_nqs_dn13, ) = (0.0, 0.0, );
            locals.var_qb_nqs_rv = 0.0;
        }
        if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36560_e51244: f64 = (-locals.var_qd_nqs);
            let assign36560_e51246: f64 = (assign36560_e51244 - locals.var_qs_nqs);
            let assign36560_e51248: f64 = (assign36560_e51246 - locals.var_qb_nqs);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (assign36560_e51248, ((-locals.var_qd_nqs_dn0) - locals.var_qs_nqs_dn0), ((-locals.var_qd_nqs_dn2) - locals.var_qs_nqs_dn2), ((-locals.var_qd_nqs_dn6) - locals.var_qs_nqs_dn6), ((-locals.var_qd_nqs_dn7) - locals.var_qs_nqs_dn7), ((-locals.var_qd_nqs_dn10) - locals.var_qs_nqs_dn10), ((-locals.var_qd_nqs_dn11) - locals.var_qs_nqs_dn11), ((-locals.var_qd_nqs_dn12) - locals.var_qs_nqs_dn12), (-locals.var_qb_nqs_dn13), (-locals.var_qd_nqs_dn15), (-locals.var_qs_nqs_dn16), ((-locals.var_qd_nqs_dn17) - locals.var_qs_nqs_dn17), ((-locals.var_qd_nqs_dn18) - locals.var_qs_nqs_dn18), );
            locals.var_qg_nqs_rv = 0.0;
        }
        if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs == 0.0)) {
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qb_nqs, locals.var_qb_nqs_dn13, ) = (0.0, 0.0, );
            locals.var_qb_nqs_rv = 0.0;
        }
        let assign36660_e51311: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign36660_e51311;
        locals.var_guard1210_rv = 0.0;
        if (locals.var_guard1210 != 0.0) {
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, );
            locals.var_ids_rv = 0.0;
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17, ) = (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17, );
            locals.var_isub_rv = 0.0;
        }
        if (locals.var_guard1210 != 0.0) {
            let assign36700_e51327: f64 = (locals.var_qge + locals.var_qg_nqs);
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18, ) = (assign36700_e51327, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18), );
            locals.var_qg_rv = 0.0;
        }
        if (locals.var_guard1210 != 0.0) {
            let assign36710_e51333: f64 = (locals.var_qde + locals.var_qd_nqs);
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, ) = (assign36710_e51333, (locals.var_qde_dn0 + locals.var_qd_nqs_dn0), (locals.var_qde_dn2 + locals.var_qd_nqs_dn2), (locals.var_qde_dn6 + locals.var_qd_nqs_dn6), (locals.var_qde_dn7 + locals.var_qd_nqs_dn7), (locals.var_qde_dn10 + locals.var_qd_nqs_dn10), (locals.var_qde_dn11 + locals.var_qd_nqs_dn11), (locals.var_qde_dn12 + locals.var_qd_nqs_dn12), locals.var_qde_dn13, (locals.var_qde_dn15 + locals.var_qd_nqs_dn15), locals.var_qde_dn16, (locals.var_qde_dn17 + locals.var_qd_nqs_dn17), (locals.var_qde_dn18 + locals.var_qd_nqs_dn18), );
            locals.var_qd_rv = 0.0;
        }
        if (locals.var_guard1210 != 0.0) {
            let assign36730_e51345: f64 = (locals.var_qge + locals.var_qde);
            let assign36730_e51347: f64 = (assign36730_e51345 + locals.var_qse);
            let assign36730_e51348: f64 = (-assign36730_e51347);
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (assign36730_e51348, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)), );
            locals.var_qbe_rv = 0.0;
        }
        if (locals.var_guard1210 != 0.0) {
            let assign36740_e51354: f64 = (locals.var_qbe + locals.var_qb_nqs);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18, ) = (assign36740_e51354, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, );
            locals.var_qb_rv = 0.0;
        }
        if (locals.var_guard1210 == 0.0) {
            let assign36750_e51360: f64 = (-locals.var_idse);
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (assign36750_e51360, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12), (-locals.var_idse_dn17), );
            locals.var_ids_rv = 0.0;
        }
        if (locals.var_guard1210 == 0.0) {
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isub_rv = 0.0;
        }
        if (locals.var_guard1210 == 0.0) {
            let assign36780_e51377: f64 = (locals.var_qge + locals.var_qg_nqs);
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18, ) = (assign36780_e51377, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18), );
            locals.var_qg_rv = 0.0;
        }
        if (locals.var_guard1210 == 0.0) {
            let assign36790_e51384: f64 = (locals.var_qse + locals.var_qs_nqs);
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, ) = (assign36790_e51384, (locals.var_qse_dn0 + locals.var_qs_nqs_dn0), (locals.var_qse_dn2 + locals.var_qs_nqs_dn2), (locals.var_qse_dn6 + locals.var_qs_nqs_dn6), (locals.var_qse_dn7 + locals.var_qs_nqs_dn7), (locals.var_qse_dn10 + locals.var_qs_nqs_dn10), (locals.var_qse_dn11 + locals.var_qs_nqs_dn11), (locals.var_qse_dn12 + locals.var_qs_nqs_dn12), locals.var_qse_dn13, locals.var_qse_dn15, (locals.var_qse_dn16 + locals.var_qs_nqs_dn16), (locals.var_qse_dn17 + locals.var_qs_nqs_dn17), (locals.var_qse_dn18 + locals.var_qs_nqs_dn18), );
            locals.var_qd_rv = 0.0;
        }
        if (locals.var_guard1210 == 0.0) {
            let assign36810_e51398: f64 = (locals.var_qge + locals.var_qde);
            let assign36810_e51400: f64 = (assign36810_e51398 + locals.var_qse);
            let assign36810_e51401: f64 = (-assign36810_e51400);
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (assign36810_e51401, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)), );
            locals.var_qbe_rv = 0.0;
        }
        if (locals.var_guard1210 == 0.0) {
            let assign36820_e51408: f64 = (locals.var_qbe + locals.var_qb_nqs);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18, ) = (assign36820_e51408, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, );
            locals.var_qb_rv = 0.0;
        }
        let assign36880_e51418: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign36880_e51418;
        locals.var_guard1211_rv = 0.0;
        if (locals.var_guard1211 != 0.0) {
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, );
            locals.var_ibd_rv = 0.0;
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, );
            locals.var_qbd_rv = 0.0;
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, );
            locals.var_ibs_rv = 0.0;
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        let assign36930_e51441: f64 = if ((p.p38 == 1.0) && (locals.var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign36930_e51441;
        locals.var_guard1212_rv = 0.0;
        if (locals.var_guard1212 != 0.0) {
            locals.var_cthe = locals.var_cth;
            locals.var_cthe_rv = 0.0;
        }
        if (locals.var_guard1212 == 0.0) {
            locals.var_cthe = 0.0;
            locals.var_cthe_rv = 0.0;
        }
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, ) = (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, );
        locals.var_idse_rv = 0.0;
        let assign37450_e51611: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign37450_e51611;
        locals.var_guard1214_rv = 0.0;
        if (locals.var_guard1214 != 0.0) {
            let assign37460_e51615: f64 = (p.p50 * locals.var_ibd);
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (assign37460_e51615, (p.p50 * locals.var_ibd_dn0), (p.p50 * locals.var_ibd_dn2), (p.p50 * locals.var_ibd_dn6), (p.p50 * locals.var_ibd_dn7), (p.p50 * locals.var_ibd_dn10), (p.p50 * locals.var_ibd_dn11), (p.p50 * locals.var_ibd_dn12), (p.p50 * locals.var_ibd_dn17), );
            locals.var_ibdb_rv = 0.0;
        }
        if (locals.var_guard1214 != 0.0) {
            let assign37470_e51621: f64 = (p.p50 * locals.var_ibs);
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (assign37470_e51621, (p.p50 * locals.var_ibs_dn0), (p.p50 * locals.var_ibs_dn2), (p.p50 * locals.var_ibs_dn6), (p.p50 * locals.var_ibs_dn7), (p.p50 * locals.var_ibs_dn10), (p.p50 * locals.var_ibs_dn11), (p.p50 * locals.var_ibs_dn12), (p.p50 * locals.var_ibs_dn17), );
            locals.var_ibsb_rv = 0.0;
        }
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, ) = (locals.var_qdrat_noi, locals.var_qdrat_noi_dn0, locals.var_qdrat_noi_dn2, locals.var_qdrat_noi_dn6, locals.var_qdrat_noi_dn7, locals.var_qdrat_noi_dn10, locals.var_qdrat_noi_dn11, locals.var_qdrat_noi_dn12, locals.var_qdrat_noi_dn17, );
        locals.var_qdrat_rv = 0.0;
        let assign37680_e51734: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign37680_e51734;
        locals.var_guard1222_rv = 0.0;
        let assign37700_e51741: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign37700_e51741;
        locals.var_guard1223_rv = 0.0;
        let assign37710_e51750: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign37710_e51750;
        locals.var_guard1224_rv = 0.0;
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let eq10_e356: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qg);
        let eq10_e357: f64 = (p.p50 * eq10_e356);
        let eq10_e357_d_n0: f64 = (p.p50 * (locals.var_qg_dn0 * ddt_scale));
        let eq10_e357_d_n2: f64 = (p.p50 * (locals.var_qg_dn2 * ddt_scale));
        let eq10_e357_d_n6: f64 = (p.p50 * (locals.var_qg_dn6 * ddt_scale));
        let eq10_e357_d_n7: f64 = (p.p50 * (locals.var_qg_dn7 * ddt_scale));
        let eq10_e357_d_n10: f64 = (p.p50 * (locals.var_qg_dn10 * ddt_scale));
        let eq10_e357_d_n11: f64 = (p.p50 * (locals.var_qg_dn11 * ddt_scale));
        let eq10_e357_d_n12: f64 = (p.p50 * (locals.var_qg_dn12 * ddt_scale));
        let eq10_e357_d_n13: f64 = (p.p50 * (locals.var_qg_dn13 * ddt_scale));
        let eq10_e357_d_n15: f64 = (p.p50 * (locals.var_qg_dn15 * ddt_scale));
        let eq10_e357_d_n16: f64 = (p.p50 * (locals.var_qg_dn16 * ddt_scale));
        let eq10_e357_d_n17: f64 = (p.p50 * (locals.var_qg_dn17 * ddt_scale));
        let eq10_e357_d_n18: f64 = (p.p50 * (locals.var_qg_dn18 * ddt_scale));
        let eq10_value: f64 = eq10_e357;
        let eq10_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq10_node_derivatives: [f64; 12] = [eq10_e357_d_n0, eq10_e357_d_n2, eq10_e357_d_n6, eq10_e357_d_n7, eq10_e357_d_n10, eq10_e357_d_n11, eq10_e357_d_n12, eq10_e357_d_n13, eq10_e357_d_n15, eq10_e357_d_n16, eq10_e357_d_n17, eq10_e357_d_n18];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qd);
        let eq11_e361: f64 = (p.p50 * eq11_e360);
        let eq11_e361_d_n0: f64 = (p.p50 * (locals.var_qd_dn0 * ddt_scale));
        let eq11_e361_d_n2: f64 = (p.p50 * (locals.var_qd_dn2 * ddt_scale));
        let eq11_e361_d_n6: f64 = (p.p50 * (locals.var_qd_dn6 * ddt_scale));
        let eq11_e361_d_n7: f64 = (p.p50 * (locals.var_qd_dn7 * ddt_scale));
        let eq11_e361_d_n10: f64 = (p.p50 * (locals.var_qd_dn10 * ddt_scale));
        let eq11_e361_d_n11: f64 = (p.p50 * (locals.var_qd_dn11 * ddt_scale));
        let eq11_e361_d_n12: f64 = (p.p50 * (locals.var_qd_dn12 * ddt_scale));
        let eq11_e361_d_n13: f64 = (p.p50 * (locals.var_qd_dn13 * ddt_scale));
        let eq11_e361_d_n15: f64 = (p.p50 * (locals.var_qd_dn15 * ddt_scale));
        let eq11_e361_d_n16: f64 = (p.p50 * (locals.var_qd_dn16 * ddt_scale));
        let eq11_e361_d_n17: f64 = (p.p50 * (locals.var_qd_dn17 * ddt_scale));
        let eq11_e361_d_n18: f64 = (p.p50 * (locals.var_qd_dn18 * ddt_scale));
        let eq11_value: f64 = eq11_e361;
        let eq11_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq11_node_derivatives: [f64; 12] = [eq11_e361_d_n0, eq11_e361_d_n2, eq11_e361_d_n6, eq11_e361_d_n7, eq11_e361_d_n10, eq11_e361_d_n11, eq11_e361_d_n12, eq11_e361_d_n13, eq11_e361_d_n15, eq11_e361_d_n16, eq11_e361_d_n17, eq11_e361_d_n18];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qb);
        let eq12_e365: f64 = (p.p50 * eq12_e364);
        let eq12_e365_d_n0: f64 = (p.p50 * (locals.var_qb_dn0 * ddt_scale));
        let eq12_e365_d_n2: f64 = (p.p50 * (locals.var_qb_dn2 * ddt_scale));
        let eq12_e365_d_n6: f64 = (p.p50 * (locals.var_qb_dn6 * ddt_scale));
        let eq12_e365_d_n7: f64 = (p.p50 * (locals.var_qb_dn7 * ddt_scale));
        let eq12_e365_d_n10: f64 = (p.p50 * (locals.var_qb_dn10 * ddt_scale));
        let eq12_e365_d_n11: f64 = (p.p50 * (locals.var_qb_dn11 * ddt_scale));
        let eq12_e365_d_n12: f64 = (p.p50 * (locals.var_qb_dn12 * ddt_scale));
        let eq12_e365_d_n13: f64 = (p.p50 * (locals.var_qb_dn13 * ddt_scale));
        let eq12_e365_d_n15: f64 = (p.p50 * (locals.var_qb_dn15 * ddt_scale));
        let eq12_e365_d_n16: f64 = (p.p50 * (locals.var_qb_dn16 * ddt_scale));
        let eq12_e365_d_n17: f64 = (p.p50 * (locals.var_qb_dn17 * ddt_scale));
        let eq12_e365_d_n18: f64 = (p.p50 * (locals.var_qb_dn18 * ddt_scale));
        let eq12_value: f64 = eq12_e365;
        let eq12_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq12_node_derivatives: [f64; 12] = [eq12_e365_d_n0, eq12_e365_d_n2, eq12_e365_d_n6, eq12_e365_d_n7, eq12_e365_d_n10, eq12_e365_d_n11, eq12_e365_d_n12, eq12_e365_d_n13, eq12_e365_d_n15, eq12_e365_d_n16, eq12_e365_d_n17, eq12_e365_d_n18];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let (eq30_e480, eq30_e480_d_n10,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq30_e477: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq30_e478: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq30_e477);
        (eq30_e478, (locals.var_cthe * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e480;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e480_d_n10),
        );
        let (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n2, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq34_e508: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qbs);
        let eq34_e509: f64 = (locals.var_ibs + eq34_e508);
        let eq34_e509_d_n0: f64 = (locals.var_ibs_dn0 + (locals.var_qbs_dn0 * ddt_scale));
        let eq34_e509_d_n2: f64 = (locals.var_ibs_dn2 + (locals.var_qbs_dn2 * ddt_scale));
        let eq34_e509_d_n6: f64 = (locals.var_ibs_dn6 + (locals.var_qbs_dn6 * ddt_scale));
        let eq34_e509_d_n7: f64 = (locals.var_ibs_dn7 + (locals.var_qbs_dn7 * ddt_scale));
        let eq34_e509_d_n10: f64 = (locals.var_ibs_dn10 + (locals.var_qbs_dn10 * ddt_scale));
        let eq34_e509_d_n11: f64 = (locals.var_ibs_dn11 + (locals.var_qbs_dn11 * ddt_scale));
        let eq34_e509_d_n12: f64 = (locals.var_ibs_dn12 + (locals.var_qbs_dn12 * ddt_scale));
        let eq34_e509_d_n17: f64 = (locals.var_ibs_dn17 + (locals.var_qbs_dn17 * ddt_scale));
        let eq34_e510: f64 = (p.p50 * eq34_e509);
        let eq34_e510_d_n0: f64 = (p.p50 * eq34_e509_d_n0);
        let eq34_e510_d_n2: f64 = (p.p50 * eq34_e509_d_n2);
        let eq34_e510_d_n6: f64 = (p.p50 * eq34_e509_d_n6);
        let eq34_e510_d_n7: f64 = (p.p50 * eq34_e509_d_n7);
        let eq34_e510_d_n10: f64 = (p.p50 * eq34_e509_d_n10);
        let eq34_e510_d_n11: f64 = (p.p50 * eq34_e509_d_n11);
        let eq34_e510_d_n12: f64 = (p.p50 * eq34_e509_d_n12);
        let eq34_e510_d_n17: f64 = (p.p50 * eq34_e509_d_n17);
        (eq34_e510, eq34_e510_d_n0, eq34_e510_d_n2, eq34_e510_d_n6, eq34_e510_d_n7, eq34_e510_d_n10, eq34_e510_d_n11, eq34_e510_d_n12, eq34_e510_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e512;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq34_e512_d_n0), multiplicity * (eq34_e512_d_n2), multiplicity * (eq34_e512_d_n6), multiplicity * (eq34_e512_d_n7), multiplicity * (eq34_e512_d_n10), multiplicity * (eq34_e512_d_n11), multiplicity * (eq34_e512_d_n12), multiplicity * (eq34_e512_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq35_e517: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qbd);
        let eq35_e518: f64 = (locals.var_ibd + eq35_e517);
        let eq35_e518_d_n0: f64 = (locals.var_ibd_dn0 + (locals.var_qbd_dn0 * ddt_scale));
        let eq35_e518_d_n2: f64 = (locals.var_ibd_dn2 + (locals.var_qbd_dn2 * ddt_scale));
        let eq35_e518_d_n6: f64 = (locals.var_ibd_dn6 + (locals.var_qbd_dn6 * ddt_scale));
        let eq35_e518_d_n7: f64 = (locals.var_ibd_dn7 + (locals.var_qbd_dn7 * ddt_scale));
        let eq35_e518_d_n10: f64 = (locals.var_ibd_dn10 + (locals.var_qbd_dn10 * ddt_scale));
        let eq35_e518_d_n11: f64 = (locals.var_ibd_dn11 + (locals.var_qbd_dn11 * ddt_scale));
        let eq35_e518_d_n12: f64 = (locals.var_ibd_dn12 + (locals.var_qbd_dn12 * ddt_scale));
        let eq35_e518_d_n17: f64 = (locals.var_ibd_dn17 + (locals.var_qbd_dn17 * ddt_scale));
        let eq35_e519: f64 = (p.p50 * eq35_e518);
        let eq35_e519_d_n0: f64 = (p.p50 * eq35_e518_d_n0);
        let eq35_e519_d_n2: f64 = (p.p50 * eq35_e518_d_n2);
        let eq35_e519_d_n6: f64 = (p.p50 * eq35_e518_d_n6);
        let eq35_e519_d_n7: f64 = (p.p50 * eq35_e518_d_n7);
        let eq35_e519_d_n10: f64 = (p.p50 * eq35_e518_d_n10);
        let eq35_e519_d_n11: f64 = (p.p50 * eq35_e518_d_n11);
        let eq35_e519_d_n12: f64 = (p.p50 * eq35_e518_d_n12);
        let eq35_e519_d_n17: f64 = (p.p50 * eq35_e518_d_n17);
        (eq35_e519, eq35_e519_d_n0, eq35_e519_d_n2, eq35_e519_d_n6, eq35_e519_d_n7, eq35_e519_d_n10, eq35_e519_d_n11, eq35_e519_d_n12, eq35_e519_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e521;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq35_e521_d_n0), multiplicity * (eq35_e521_d_n2), multiplicity * (eq35_e521_d_n6), multiplicity * (eq35_e521_d_n7), multiplicity * (eq35_e521_d_n10), multiplicity * (eq35_e521_d_n11), multiplicity * (eq35_e521_d_n12), multiplicity * (eq35_e521_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq46_e605, eq46_e605_d_n18,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e600: f64 = (1e-9 / 0.0001);
        let eq46_e602: f64 = (eq46_e600 * (nv18 - 0.0));
        let eq46_e603: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq46_e602);
        (eq46_e603, (eq46_e600 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e605;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq46_value),
            18,
            multiplicity * (eq46_e605_d_n18),
        );
        let (eq47_e616, eq47_e616_d_n13,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv13 - 0.0));
        let eq47_e614: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq47_e613);
        (eq47_e614, (eq47_e611 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e616;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq47_value),
            13,
            multiplicity * (eq47_e616_d_n13),
        );
        let (eq52_e655, eq52_e655_d_n17,) = {
    if ((locals.var_guard1223 != 0.0) && (locals.var_guard1224 != 0.0)) {
        let eq52_e650: f64 = (1e-9 / 0.0001);
        let eq52_e652: f64 = (eq52_e650 * (nv17 - 0.0));
        let eq52_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e652);
        (eq52_e653, (eq52_e650 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e655;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq52_value),
            17,
            multiplicity * (eq52_e655_d_n17),
        );
        let (eq59_e713, eq59_e713_d_n17,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e708: f64 = (1e-9 / 0.0001);
        let eq59_e710: f64 = (eq59_e708 * (nv17 - 0.0));
        let eq59_e711: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq59_e710);
        (eq59_e711, (eq59_e708 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e713;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq59_value),
            17,
            multiplicity * (eq59_e713_d_n17),
        );
        let (eq67_e781, eq67_e781_d_n15,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e776: f64 = (1e-9 / 0.0001);
        let eq67_e778: f64 = (eq67_e776 * (nv15 - 0.0));
        let eq67_e779: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq67_e778);
        (eq67_e779, (eq67_e776 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e781;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq67_value),
            15,
            multiplicity * (eq67_e781_d_n15),
        );
        let (eq68_e793, eq68_e793_d_n16,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e788: f64 = (1e-9 / 0.0001);
        let eq68_e790: f64 = (eq68_e788 * (nv16 - 0.0));
        let eq68_e791: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq68_e790);
        (eq68_e791, (eq68_e788 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e793;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq68_value),
            16,
            multiplicity * (eq68_e793_d_n16),
        );
        let (eq69_e805, eq69_e805_d_n13,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e800: f64 = (1e-9 / 0.0001);
        let eq69_e802: f64 = (eq69_e800 * (nv13 - 0.0));
        let eq69_e803: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq69_e802);
        (eq69_e803, (eq69_e800 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e805;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq69_value),
            13,
            multiplicity * (eq69_e805_d_n13),
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let eq10_e356_q: f64 = locals.var_qg;
        let eq10_e357: f64 = (p.p50 * locals.var_qg);
        let eq10_e357_d_n0: f64 = (p.p50 * locals.var_qg_dn0);
        let eq10_e357_d_n2: f64 = (p.p50 * locals.var_qg_dn2);
        let eq10_e357_d_n6: f64 = (p.p50 * locals.var_qg_dn6);
        let eq10_e357_d_n7: f64 = (p.p50 * locals.var_qg_dn7);
        let eq10_e357_d_n10: f64 = (p.p50 * locals.var_qg_dn10);
        let eq10_e357_d_n11: f64 = (p.p50 * locals.var_qg_dn11);
        let eq10_e357_d_n12: f64 = (p.p50 * locals.var_qg_dn12);
        let eq10_e357_d_n13: f64 = (p.p50 * locals.var_qg_dn13);
        let eq10_e357_d_n15: f64 = (p.p50 * locals.var_qg_dn15);
        let eq10_e357_d_n16: f64 = (p.p50 * locals.var_qg_dn16);
        let eq10_e357_d_n17: f64 = (p.p50 * locals.var_qg_dn17);
        let eq10_e357_d_n18: f64 = (p.p50 * locals.var_qg_dn18);
        let eq10_e357_q: f64 = (p.p50 * eq10_e356_q);
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e357_d_n0, 0.0, eq10_e357_d_n2, 0.0, 0.0, 0.0, eq10_e357_d_n6, eq10_e357_d_n7, 0.0, 0.0, eq10_e357_d_n10, eq10_e357_d_n11, eq10_e357_d_n12, eq10_e357_d_n13, 0.0, eq10_e357_d_n15, eq10_e357_d_n16, eq10_e357_d_n17, eq10_e357_d_n18];
        let eq10_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e360_q: f64 = locals.var_qd;
        let eq11_e361: f64 = (p.p50 * locals.var_qd);
        let eq11_e361_d_n0: f64 = (p.p50 * locals.var_qd_dn0);
        let eq11_e361_d_n2: f64 = (p.p50 * locals.var_qd_dn2);
        let eq11_e361_d_n6: f64 = (p.p50 * locals.var_qd_dn6);
        let eq11_e361_d_n7: f64 = (p.p50 * locals.var_qd_dn7);
        let eq11_e361_d_n10: f64 = (p.p50 * locals.var_qd_dn10);
        let eq11_e361_d_n11: f64 = (p.p50 * locals.var_qd_dn11);
        let eq11_e361_d_n12: f64 = (p.p50 * locals.var_qd_dn12);
        let eq11_e361_d_n13: f64 = (p.p50 * locals.var_qd_dn13);
        let eq11_e361_d_n15: f64 = (p.p50 * locals.var_qd_dn15);
        let eq11_e361_d_n16: f64 = (p.p50 * locals.var_qd_dn16);
        let eq11_e361_d_n17: f64 = (p.p50 * locals.var_qd_dn17);
        let eq11_e361_d_n18: f64 = (p.p50 * locals.var_qd_dn18);
        let eq11_e361_q: f64 = (p.p50 * eq11_e360_q);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e361_d_n0, 0.0, eq11_e361_d_n2, 0.0, 0.0, 0.0, eq11_e361_d_n6, eq11_e361_d_n7, 0.0, 0.0, eq11_e361_d_n10, eq11_e361_d_n11, eq11_e361_d_n12, eq11_e361_d_n13, 0.0, eq11_e361_d_n15, eq11_e361_d_n16, eq11_e361_d_n17, eq11_e361_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e364_q: f64 = locals.var_qb;
        let eq12_e365: f64 = (p.p50 * locals.var_qb);
        let eq12_e365_d_n0: f64 = (p.p50 * locals.var_qb_dn0);
        let eq12_e365_d_n2: f64 = (p.p50 * locals.var_qb_dn2);
        let eq12_e365_d_n6: f64 = (p.p50 * locals.var_qb_dn6);
        let eq12_e365_d_n7: f64 = (p.p50 * locals.var_qb_dn7);
        let eq12_e365_d_n10: f64 = (p.p50 * locals.var_qb_dn10);
        let eq12_e365_d_n11: f64 = (p.p50 * locals.var_qb_dn11);
        let eq12_e365_d_n12: f64 = (p.p50 * locals.var_qb_dn12);
        let eq12_e365_d_n13: f64 = (p.p50 * locals.var_qb_dn13);
        let eq12_e365_d_n15: f64 = (p.p50 * locals.var_qb_dn15);
        let eq12_e365_d_n16: f64 = (p.p50 * locals.var_qb_dn16);
        let eq12_e365_d_n17: f64 = (p.p50 * locals.var_qb_dn17);
        let eq12_e365_d_n18: f64 = (p.p50 * locals.var_qb_dn18);
        let eq12_e365_q: f64 = (p.p50 * eq12_e364_q);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e365_d_n0, 0.0, eq12_e365_d_n2, 0.0, 0.0, 0.0, eq12_e365_d_n6, eq12_e365_d_n7, 0.0, 0.0, eq12_e365_d_n10, eq12_e365_d_n11, eq12_e365_d_n12, eq12_e365_d_n13, 0.0, eq12_e365_d_n15, eq12_e365_d_n16, eq12_e365_d_n17, eq12_e365_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e480, eq30_e480_d_n10, eq30_e480_q,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq30_e477: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq30_e478_q: f64 = eq30_e477;
        (eq30_e477, locals.var_cthe, eq30_e478_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e480_d_n10),
        );
        let (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n2, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n17, eq34_e512_q, eq34_e512_q_d_n0, eq34_e512_q_d_n2, eq34_e512_q_d_n6, eq34_e512_q_d_n7, eq34_e512_q_d_n10, eq34_e512_q_d_n11, eq34_e512_q_d_n12, eq34_e512_q_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq34_e508_q: f64 = locals.var_qbs;
        let eq34_e509: f64 = (locals.var_ibs + locals.var_qbs);
        let eq34_e509_d_n0: f64 = (locals.var_ibs_dn0 + locals.var_qbs_dn0);
        let eq34_e509_d_n2: f64 = (locals.var_ibs_dn2 + locals.var_qbs_dn2);
        let eq34_e509_d_n6: f64 = (locals.var_ibs_dn6 + locals.var_qbs_dn6);
        let eq34_e509_d_n7: f64 = (locals.var_ibs_dn7 + locals.var_qbs_dn7);
        let eq34_e509_d_n10: f64 = (locals.var_ibs_dn10 + locals.var_qbs_dn10);
        let eq34_e509_d_n11: f64 = (locals.var_ibs_dn11 + locals.var_qbs_dn11);
        let eq34_e509_d_n12: f64 = (locals.var_ibs_dn12 + locals.var_qbs_dn12);
        let eq34_e509_d_n17: f64 = (locals.var_ibs_dn17 + locals.var_qbs_dn17);
        let eq34_e509_q: f64 = eq34_e508_q;
        let eq34_e510: f64 = (p.p50 * eq34_e509);
        let eq34_e510_d_n0: f64 = (p.p50 * eq34_e509_d_n0);
        let eq34_e510_d_n2: f64 = (p.p50 * eq34_e509_d_n2);
        let eq34_e510_d_n6: f64 = (p.p50 * eq34_e509_d_n6);
        let eq34_e510_d_n7: f64 = (p.p50 * eq34_e509_d_n7);
        let eq34_e510_d_n10: f64 = (p.p50 * eq34_e509_d_n10);
        let eq34_e510_d_n11: f64 = (p.p50 * eq34_e509_d_n11);
        let eq34_e510_d_n12: f64 = (p.p50 * eq34_e509_d_n12);
        let eq34_e510_d_n17: f64 = (p.p50 * eq34_e509_d_n17);
        let eq34_e510_q: f64 = (p.p50 * eq34_e509_q);
        let eq34_e510_q_d_n0: f64 = (p.p50 * locals.var_qbs_dn0);
        let eq34_e510_q_d_n2: f64 = (p.p50 * locals.var_qbs_dn2);
        let eq34_e510_q_d_n6: f64 = (p.p50 * locals.var_qbs_dn6);
        let eq34_e510_q_d_n7: f64 = (p.p50 * locals.var_qbs_dn7);
        let eq34_e510_q_d_n10: f64 = (p.p50 * locals.var_qbs_dn10);
        let eq34_e510_q_d_n11: f64 = (p.p50 * locals.var_qbs_dn11);
        let eq34_e510_q_d_n12: f64 = (p.p50 * locals.var_qbs_dn12);
        let eq34_e510_q_d_n17: f64 = (p.p50 * locals.var_qbs_dn17);
        (eq34_e510, eq34_e510_d_n0, eq34_e510_d_n2, eq34_e510_d_n6, eq34_e510_d_n7, eq34_e510_d_n10, eq34_e510_d_n11, eq34_e510_d_n12, eq34_e510_d_n17, eq34_e510_q, eq34_e510_q_d_n0, eq34_e510_q_d_n2, eq34_e510_q_d_n6, eq34_e510_q_d_n7, eq34_e510_q_d_n10, eq34_e510_q_d_n11, eq34_e510_q_d_n12, eq34_e510_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e512_q_d_n0, 0.0, eq34_e512_q_d_n2, 0.0, 0.0, 0.0, eq34_e512_q_d_n6, eq34_e512_q_d_n7, 0.0, 0.0, eq34_e512_q_d_n10, eq34_e512_q_d_n11, eq34_e512_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq34_e512_q_d_n17, 0.0];
        let eq34_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17, eq35_e521_q, eq35_e521_q_d_n0, eq35_e521_q_d_n2, eq35_e521_q_d_n6, eq35_e521_q_d_n7, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, eq35_e521_q_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq35_e517_q: f64 = locals.var_qbd;
        let eq35_e518: f64 = (locals.var_ibd + locals.var_qbd);
        let eq35_e518_d_n0: f64 = (locals.var_ibd_dn0 + locals.var_qbd_dn0);
        let eq35_e518_d_n2: f64 = (locals.var_ibd_dn2 + locals.var_qbd_dn2);
        let eq35_e518_d_n6: f64 = (locals.var_ibd_dn6 + locals.var_qbd_dn6);
        let eq35_e518_d_n7: f64 = (locals.var_ibd_dn7 + locals.var_qbd_dn7);
        let eq35_e518_d_n10: f64 = (locals.var_ibd_dn10 + locals.var_qbd_dn10);
        let eq35_e518_d_n11: f64 = (locals.var_ibd_dn11 + locals.var_qbd_dn11);
        let eq35_e518_d_n12: f64 = (locals.var_ibd_dn12 + locals.var_qbd_dn12);
        let eq35_e518_d_n17: f64 = (locals.var_ibd_dn17 + locals.var_qbd_dn17);
        let eq35_e518_q: f64 = eq35_e517_q;
        let eq35_e519: f64 = (p.p50 * eq35_e518);
        let eq35_e519_d_n0: f64 = (p.p50 * eq35_e518_d_n0);
        let eq35_e519_d_n2: f64 = (p.p50 * eq35_e518_d_n2);
        let eq35_e519_d_n6: f64 = (p.p50 * eq35_e518_d_n6);
        let eq35_e519_d_n7: f64 = (p.p50 * eq35_e518_d_n7);
        let eq35_e519_d_n10: f64 = (p.p50 * eq35_e518_d_n10);
        let eq35_e519_d_n11: f64 = (p.p50 * eq35_e518_d_n11);
        let eq35_e519_d_n12: f64 = (p.p50 * eq35_e518_d_n12);
        let eq35_e519_d_n17: f64 = (p.p50 * eq35_e518_d_n17);
        let eq35_e519_q: f64 = (p.p50 * eq35_e518_q);
        let eq35_e519_q_d_n0: f64 = (p.p50 * locals.var_qbd_dn0);
        let eq35_e519_q_d_n2: f64 = (p.p50 * locals.var_qbd_dn2);
        let eq35_e519_q_d_n6: f64 = (p.p50 * locals.var_qbd_dn6);
        let eq35_e519_q_d_n7: f64 = (p.p50 * locals.var_qbd_dn7);
        let eq35_e519_q_d_n10: f64 = (p.p50 * locals.var_qbd_dn10);
        let eq35_e519_q_d_n11: f64 = (p.p50 * locals.var_qbd_dn11);
        let eq35_e519_q_d_n12: f64 = (p.p50 * locals.var_qbd_dn12);
        let eq35_e519_q_d_n17: f64 = (p.p50 * locals.var_qbd_dn17);
        (eq35_e519, eq35_e519_d_n0, eq35_e519_d_n2, eq35_e519_d_n6, eq35_e519_d_n7, eq35_e519_d_n10, eq35_e519_d_n11, eq35_e519_d_n12, eq35_e519_d_n17, eq35_e519_q, eq35_e519_q_d_n0, eq35_e519_q_d_n2, eq35_e519_q_d_n6, eq35_e519_q_d_n7, eq35_e519_q_d_n10, eq35_e519_q_d_n11, eq35_e519_q_d_n12, eq35_e519_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e521_q_d_n0, 0.0, eq35_e521_q_d_n2, 0.0, 0.0, 0.0, eq35_e521_q_d_n6, eq35_e521_q_d_n7, 0.0, 0.0, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq35_e521_q_d_n17, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e605, eq46_e605_d_n18, eq46_e605_q,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e600: f64 = (1e-9 / 0.0001);
        let eq46_e602: f64 = (eq46_e600 * (nv18 - 0.0));
        let eq46_e603_q: f64 = eq46_e602;
        (eq46_e602, eq46_e600, eq46_e603_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq46_e605_d_n18),
        );
        let (eq47_e616, eq47_e616_d_n13, eq47_e616_q,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv13 - 0.0));
        let eq47_e614_q: f64 = eq47_e613;
        (eq47_e613, eq47_e611, eq47_e614_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq47_e616_d_n13),
        );
        let (eq52_e655, eq52_e655_d_n17, eq52_e655_q,) = {
    if ((locals.var_guard1223 != 0.0) && (locals.var_guard1224 != 0.0)) {
        let eq52_e650: f64 = (1e-9 / 0.0001);
        let eq52_e652: f64 = (eq52_e650 * (nv17 - 0.0));
        let eq52_e653_q: f64 = eq52_e652;
        (eq52_e652, eq52_e650, eq52_e653_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq52_e655_d_n17),
        );
        let (eq59_e713, eq59_e713_d_n17, eq59_e713_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e708: f64 = (1e-9 / 0.0001);
        let eq59_e710: f64 = (eq59_e708 * (nv17 - 0.0));
        let eq59_e711_q: f64 = eq59_e710;
        (eq59_e710, eq59_e708, eq59_e711_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq59_e713_d_n17),
        );
        let (eq67_e781, eq67_e781_d_n15, eq67_e781_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e776: f64 = (1e-9 / 0.0001);
        let eq67_e778: f64 = (eq67_e776 * (nv15 - 0.0));
        let eq67_e779_q: f64 = eq67_e778;
        (eq67_e778, eq67_e776, eq67_e779_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq67_e781_d_n15),
        );
        let (eq68_e793, eq68_e793_d_n16, eq68_e793_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e788: f64 = (1e-9 / 0.0001);
        let eq68_e790: f64 = (eq68_e788 * (nv16 - 0.0));
        let eq68_e791_q: f64 = eq68_e790;
        (eq68_e790, eq68_e788, eq68_e791_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq68_e793_d_n16),
        );
        let (eq69_e805, eq69_e805_d_n13, eq69_e805_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e800: f64 = (1e-9 / 0.0001);
        let eq69_e802: f64 = (eq69_e800 * (nv13 - 0.0));
        let eq69_e803_q: f64 = eq69_e802;
        (eq69_e802, eq69_e800, eq69_e803_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq69_e805_d_n13),
        );
    }
}

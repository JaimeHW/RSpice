#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_reactive_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1026 == 0.0)) && (locals.var_guard1028 == 0.0)) {
            let assign31840_e46791: f64 = (-locals.var_weffcv_nf);
            let assign31840_e46792: f64 = (locals.var_cgdoe * assign31840_e46791);
            (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn6, locals.var_cgdoe_dn7, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12, locals.var_cgdoe_dn17, ) = (assign31840_e46792, (locals.var_cgdoe_dn0 * assign31840_e46791), (locals.var_cgdoe_dn2 * assign31840_e46791), (locals.var_cgdoe_dn6 * assign31840_e46791), (locals.var_cgdoe_dn7 * assign31840_e46791), (locals.var_cgdoe_dn10 * assign31840_e46791), (locals.var_cgdoe_dn11 * assign31840_e46791), (locals.var_cgdoe_dn12 * assign31840_e46791), (locals.var_cgdoe_dn17 * assign31840_e46791), );
            locals.var_cgdoe_rv = 0.0;
        }
        if ((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) {
            let assign31850_e46800: f64 = (-locals.var_cgdoe);
            let assign31850_e46803: f64 = (locals.var_vgs - locals.var_vds);
            let assign31850_e46804: f64 = (assign31850_e46800 * assign31850_e46803);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign31850_e46804, (((-locals.var_cgdoe_dn0) * assign31850_e46803) + (assign31850_e46800 * (-locals.var_vds_dn0))), (((-locals.var_cgdoe_dn2) * assign31850_e46803) + (assign31850_e46800 * (-locals.var_vds_dn2))), (((-locals.var_cgdoe_dn6) * assign31850_e46803) + (assign31850_e46800 * (locals.var_vgs_dn6 - locals.var_vds_dn6))), (((-locals.var_cgdoe_dn7) * assign31850_e46803) + (assign31850_e46800 * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (((-locals.var_cgdoe_dn10) * assign31850_e46803) + (assign31850_e46800 * (-locals.var_vds_dn10))), (((-locals.var_cgdoe_dn11) * assign31850_e46803) + (assign31850_e46800 * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (((-locals.var_cgdoe_dn12) * assign31850_e46803) + (assign31850_e46800 * (-locals.var_vds_dn12))), (((-locals.var_cgdoe_dn17) * assign31850_e46803) + (assign31850_e46800 * (-locals.var_vds_dn17))), );
            locals.var_qgod_rv = 0.0;
        }
        let assign31860_e46819: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgso_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1029 = assign31860_e46819;
        locals.var_guard1029_rv = 0.0;
        let assign31870_e46822: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1030 = assign31870_e46822;
        locals.var_guard1030_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 != 0.0)) && (locals.var_guard1030 != 0.0)) {
            let assign31880_e46832: f64 = (-locals.var_cox0__blk910);
            let assign31880_e46834: f64 = (assign31880_e46832 * p.p188);
            let assign31880_e46836: f64 = (assign31880_e46834 * locals.var_w_dioscv);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31880_e46836, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 != 0.0)) && (locals.var_guard1030 == 0.0)) {
            let assign31890_e46849: f64 = (-locals.var_cox0__blk910);
            let assign31890_e46851: f64 = (assign31890_e46849 * p.p188);
            let assign31890_e46853: f64 = (assign31890_e46851 * locals.var_weffcv_nf);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31890_e46853, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        if (((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 == 0.0)) {
            let assign31900_e46865: f64 = (locals.var_modenml * p.p170);
            let assign31900_e46868: f64 = (locals.var_modervs * p.p169);
            let assign31900_e46869: f64 = (assign31900_e46865 + assign31900_e46868);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31900_e46869, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cgsoe_rv = 0.0;
        }
        let assign31910_e46874: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1031 = assign31910_e46874;
        locals.var_guard1031_rv = 0.0;
        if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 == 0.0)) && (locals.var_guard1031 != 0.0)) {
            let assign31920_e46886: f64 = (locals.var_modenml * locals.var_w_dioscv);
            let assign31920_e46889: f64 = (locals.var_modervs * locals.var_w_diodcv);
            let assign31920_e46890: f64 = (assign31920_e46886 + assign31920_e46889);
            (locals.var_t1__blk900, locals.var_t1__blk900_dn0, locals.var_t1__blk900_dn2, locals.var_t1__blk900_dn6, locals.var_t1__blk900_dn7, locals.var_t1__blk900_dn10, locals.var_t1__blk900_dn11, locals.var_t1__blk900_dn12, locals.var_t1__blk900_dn17, ) = (assign31920_e46890, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk900_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 == 0.0)) && (locals.var_guard1031 != 0.0)) {
            let assign31930_e46904: f64 = (-locals.var_t1__blk900);
            let assign31930_e46905: f64 = (locals.var_cgsoe * assign31930_e46904);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31930_e46905, ((locals.var_cgsoe_dn0 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn0))), ((locals.var_cgsoe_dn2 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn2))), ((locals.var_cgsoe_dn6 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn6))), ((locals.var_cgsoe_dn7 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn7))), ((locals.var_cgsoe_dn10 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn10))), ((locals.var_cgsoe_dn11 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn11))), ((locals.var_cgsoe_dn12 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn12))), ((locals.var_cgsoe_dn17 * assign31930_e46904) + (locals.var_cgsoe * (-locals.var_t1__blk900_dn17))), );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) && (locals.var_guard1029 == 0.0)) && (locals.var_guard1031 == 0.0)) {
            let assign31940_e46920: f64 = (-locals.var_weffcv_nf);
            let assign31940_e46921: f64 = (locals.var_cgsoe * assign31940_e46920);
            (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn6, locals.var_cgsoe_dn7, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12, locals.var_cgsoe_dn17, ) = (assign31940_e46921, (locals.var_cgsoe_dn0 * assign31940_e46920), (locals.var_cgsoe_dn2 * assign31940_e46920), (locals.var_cgsoe_dn6 * assign31940_e46920), (locals.var_cgsoe_dn7 * assign31940_e46920), (locals.var_cgsoe_dn10 * assign31940_e46920), (locals.var_cgsoe_dn11 * assign31940_e46920), (locals.var_cgsoe_dn12 * assign31940_e46920), (locals.var_cgsoe_dn17 * assign31940_e46920), );
            locals.var_cgsoe_rv = 0.0;
        }
        if ((p.p24 != 0.0) && (locals.var_guard982 == 0.0)) {
            let assign31950_e46929: f64 = (-locals.var_cgsoe);
            let assign31950_e46931: f64 = (assign31950_e46929 * locals.var_vgs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign31950_e46931, ((-locals.var_cgsoe_dn0) * locals.var_vgs), ((-locals.var_cgsoe_dn2) * locals.var_vgs), (((-locals.var_cgsoe_dn6) * locals.var_vgs) + (assign31950_e46929 * locals.var_vgs_dn6)), (((-locals.var_cgsoe_dn7) * locals.var_vgs) + (assign31950_e46929 * locals.var_vgs_dn7)), ((-locals.var_cgsoe_dn10) * locals.var_vgs), (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign31950_e46929 * locals.var_vgs_dn11)), ((-locals.var_cgsoe_dn12) * locals.var_vgs), ((-locals.var_cgsoe_dn17) * locals.var_vgs), );
            locals.var_qgos_rv = 0.0;
        }
        let assign31960_e46936: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1032 = assign31960_e46936;
        locals.var_guard1032_rv = 0.0;
        if (locals.var_guard1032 != 0.0) {
            (locals.var_vbdj, locals.var_vbdj_dn6, locals.var_vbdj_dn12, ) = (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12, );
            locals.var_vbdj_rv = 0.0;
            (locals.var_vbsj, locals.var_vbsj_dn7, locals.var_vbsj_dn12, ) = (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12, );
            locals.var_vbsj_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign31990_e46949: f64 = (locals.var_egtnom * locals.var_betatnom);
            let assign31990_e46952: f64 = (locals.var_eg * locals.var_beta);
            let assign31990_e46953: f64 = (assign31990_e46949 - assign31990_e46952);
            let assign31990_e46957: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            let assign31990_e46958: f64 = (assign31990_e46957).ln();
            let assign31990_e46959: f64 = (p.p175 * assign31990_e46958);
            let assign31990_e46960: f64 = (assign31990_e46953 + assign31990_e46959);
            let assign31990_e46962: f64 = (assign31990_e46960 / p.p174);
            let assign31990_e46963: f64 = (assign31990_e46962).exp();
            let assign31990_e46964: f64 = (p.p173 * assign31990_e46963);
            (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn12, locals.var_js_dn17, ) = (assign31990_e46964, (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p175 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign31990_e46957))) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign31990_e46963 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))), );
            locals.var_js_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32000_e46971: f64 = (locals.var_egtnom * locals.var_betatnom);
            let assign32000_e46974: f64 = (locals.var_eg * locals.var_beta);
            let assign32000_e46975: f64 = (assign32000_e46971 - assign32000_e46974);
            let assign32000_e46979: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            let assign32000_e46980: f64 = (assign32000_e46979).ln();
            let assign32000_e46981: f64 = (p.p176 * assign32000_e46980);
            let assign32000_e46982: f64 = (assign32000_e46975 + assign32000_e46981);
            let assign32000_e46984: f64 = (assign32000_e46982 / p.p174);
            let assign32000_e46985: f64 = (assign32000_e46984).exp();
            let assign32000_e46986: f64 = (p.p173 * assign32000_e46985);
            (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn12, locals.var_js2_dn17, ) = (assign32000_e46986, (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn0 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn2 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn6 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn7 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p176 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign32000_e46979))) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn11 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn12 * locals.var_beta)) / p.p174))), (p.p173 * (assign32000_e46985 * ((-(locals.var_eg_dn17 * locals.var_beta)) / p.p174))), );
            locals.var_js2_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32010_e46992: f64 = (locals.var_w_diod * p.p237);
            let assign32010_e46994: f64 = (assign32010_e46992 * locals.var_js);
            (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17, ) = (assign32010_e46994, (assign32010_e46992 * locals.var_js_dn0), (assign32010_e46992 * locals.var_js_dn2), (assign32010_e46992 * locals.var_js_dn6), (assign32010_e46992 * locals.var_js_dn7), (assign32010_e46992 * locals.var_js_dn10), (assign32010_e46992 * locals.var_js_dn11), (assign32010_e46992 * locals.var_js_dn12), (assign32010_e46992 * locals.var_js_dn17), );
            locals.var_isbd_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32020_e47000: f64 = (locals.var_w_diod * p.p237);
            let assign32020_e47002: f64 = (assign32020_e47000 * locals.var_js2);
            (locals.var_isbd2, locals.var_isbd2_dn0, locals.var_isbd2_dn2, locals.var_isbd2_dn6, locals.var_isbd2_dn7, locals.var_isbd2_dn10, locals.var_isbd2_dn11, locals.var_isbd2_dn12, locals.var_isbd2_dn17, ) = (assign32020_e47002, (assign32020_e47000 * locals.var_js2_dn0), (assign32020_e47000 * locals.var_js2_dn2), (assign32020_e47000 * locals.var_js2_dn6), (assign32020_e47000 * locals.var_js2_dn7), (assign32020_e47000 * locals.var_js2_dn10), (assign32020_e47000 * locals.var_js2_dn11), (assign32020_e47000 * locals.var_js2_dn12), (assign32020_e47000 * locals.var_js2_dn17), );
            locals.var_isbd2_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32030_e47008: f64 = (locals.var_w_dios * p.p237);
            let assign32030_e47010: f64 = (assign32030_e47008 * locals.var_js);
            (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn12, locals.var_isbs_dn17, ) = (assign32030_e47010, (assign32030_e47008 * locals.var_js_dn0), (assign32030_e47008 * locals.var_js_dn2), (assign32030_e47008 * locals.var_js_dn6), (assign32030_e47008 * locals.var_js_dn7), (assign32030_e47008 * locals.var_js_dn10), (assign32030_e47008 * locals.var_js_dn11), (assign32030_e47008 * locals.var_js_dn12), (assign32030_e47008 * locals.var_js_dn17), );
            locals.var_isbs_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32040_e47016: f64 = (locals.var_w_dios * p.p237);
            let assign32040_e47018: f64 = (assign32040_e47016 * locals.var_js2);
            (locals.var_isbs2, locals.var_isbs2_dn0, locals.var_isbs2_dn2, locals.var_isbs2_dn6, locals.var_isbs2_dn7, locals.var_isbs2_dn10, locals.var_isbs2_dn11, locals.var_isbs2_dn12, locals.var_isbs2_dn17, ) = (assign32040_e47018, (assign32040_e47016 * locals.var_js2_dn0), (assign32040_e47016 * locals.var_js2_dn2), (assign32040_e47016 * locals.var_js2_dn6), (assign32040_e47016 * locals.var_js2_dn7), (assign32040_e47016 * locals.var_js2_dn10), (assign32040_e47016 * locals.var_js2_dn11), (assign32040_e47016 * locals.var_js2_dn12), (assign32040_e47016 * locals.var_js2_dn17), );
            locals.var_isbs2_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32050_e47024: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12, ) = (assign32050_e47024, 0.0, 0.0, (locals.var_ttemp_dn10 / locals.var_uc_tnom), 0.0, );
            locals.var_t1__blk1034_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32070_e47036: f64 = (locals.var_isbd + 1e-50);
            (locals.var_t2__blk1035, locals.var_t2__blk1035_dn0, locals.var_t2__blk1035_dn2, locals.var_t2__blk1035_dn6, locals.var_t2__blk1035_dn7, locals.var_t2__blk1035_dn10, locals.var_t2__blk1035_dn11, locals.var_t2__blk1035_dn12, locals.var_t2__blk1035_dn17, ) = (assign32070_e47036, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn12, locals.var_isbd_dn17, );
            locals.var_t2__blk1035_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            (locals.var_vbdt, locals.var_vbdt_dn10, ) = (0.0, 0.0, );
            locals.var_vbdt_rv = 0.0;
            (locals.var_vbst, locals.var_vbst_dn10, ) = (0.0, 0.0, );
            locals.var_vbst_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32110_e47064: f64 = (p.p174 * locals.var_beta_inv);
            (locals.var_nvtm, locals.var_nvtm_dn10, ) = (assign32110_e47064, (p.p174 * locals.var_beta_inv_dn10), );
            locals.var_nvtm_rv = 0.0;
        }
        let assign32120_e47069: f64 = if locals.var_vbdj < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign32120_e47069;
        locals.var_guard1061_rv = 0.0;
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1061 != 0.0)) {
            let assign32130_e47075: f64 = (locals.var_vbdj / locals.var_nvtm);
            let assign32130_e47076: f64 = (assign32130_e47075).exp();
            (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12, ) = (assign32130_e47076, (assign32130_e47076 * (locals.var_vbdj_dn6 / locals.var_nvtm)), 0.0, (assign32130_e47076 * (-((locals.var_vbdj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32130_e47076 * (locals.var_vbdj_dn12 / locals.var_nvtm)), );
            locals.var_t1__blk1034_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1061 != 0.0)) {
            let assign32140_e47085: f64 = (locals.var_t1__blk1034 - 1.0);
            let assign32140_e47086: f64 = (locals.var_isbd * assign32140_e47085);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32140_e47086, (locals.var_isbd_dn0 * assign32140_e47085), (locals.var_isbd_dn2 * assign32140_e47085), ((locals.var_isbd_dn6 * assign32140_e47085) + (locals.var_isbd * locals.var_t1__blk1034_dn6)), ((locals.var_isbd_dn7 * assign32140_e47085) + (locals.var_isbd * locals.var_t1__blk1034_dn7)), ((locals.var_isbd_dn10 * assign32140_e47085) + (locals.var_isbd * locals.var_t1__blk1034_dn10)), (locals.var_isbd_dn11 * assign32140_e47085), ((locals.var_isbd_dn12 * assign32140_e47085) + (locals.var_isbd * locals.var_t1__blk1034_dn12)), (locals.var_isbd_dn17 * assign32140_e47085), );
            locals.var_ibd_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1061 == 0.0)) {
            let assign32150_e47095: f64 = (locals.var_vbdt / locals.var_nvtm);
            let assign32150_e47096: f64 = (assign32150_e47095).exp();
            (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12, ) = (assign32150_e47096, 0.0, 0.0, (assign32150_e47096 * (((locals.var_vbdt_dn10 * locals.var_nvtm) - (locals.var_vbdt * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0, );
            locals.var_t1__blk1034_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1061 == 0.0)) {
            let assign32160_e47106: f64 = (locals.var_t1__blk1034 - 1.0);
            let assign32160_e47107: f64 = (locals.var_isbd * assign32160_e47106);
            let assign32160_e47110: f64 = (locals.var_isbd / locals.var_nvtm);
            let assign32160_e47112: f64 = (assign32160_e47110 * locals.var_t1__blk1034);
            let assign32160_e47115: f64 = (locals.var_vbdj - locals.var_vbdt);
            let assign32160_e47116: f64 = (assign32160_e47112 * assign32160_e47115);
            let assign32160_e47117: f64 = (assign32160_e47107 + assign32160_e47116);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32160_e47117, ((locals.var_isbd_dn0 * assign32160_e47106) + (((locals.var_isbd_dn0 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32160_e47115)), ((locals.var_isbd_dn2 * assign32160_e47106) + (((locals.var_isbd_dn2 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32160_e47115)), (((locals.var_isbd_dn6 * assign32160_e47106) + (locals.var_isbd * locals.var_t1__blk1034_dn6)) + (((((locals.var_isbd_dn6 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32160_e47110 * locals.var_t1__blk1034_dn6)) * assign32160_e47115) + (assign32160_e47112 * locals.var_vbdj_dn6))), (((locals.var_isbd_dn7 * assign32160_e47106) + (locals.var_isbd * locals.var_t1__blk1034_dn7)) + ((((locals.var_isbd_dn7 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32160_e47110 * locals.var_t1__blk1034_dn7)) * assign32160_e47115)), (((locals.var_isbd_dn10 * assign32160_e47106) + (locals.var_isbd * locals.var_t1__blk1034_dn10)) + (((((((locals.var_isbd_dn10 * locals.var_nvtm) - (locals.var_isbd * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1034) + (assign32160_e47110 * locals.var_t1__blk1034_dn10)) * assign32160_e47115) + (assign32160_e47112 * (-locals.var_vbdt_dn10)))), ((locals.var_isbd_dn11 * assign32160_e47106) + (((locals.var_isbd_dn11 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32160_e47115)), (((locals.var_isbd_dn12 * assign32160_e47106) + (locals.var_isbd * locals.var_t1__blk1034_dn12)) + (((((locals.var_isbd_dn12 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32160_e47110 * locals.var_t1__blk1034_dn12)) * assign32160_e47115) + (assign32160_e47112 * locals.var_vbdj_dn12))), ((locals.var_isbd_dn17 * assign32160_e47106) + (((locals.var_isbd_dn17 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32160_e47115)), );
            locals.var_ibd_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32170_e47124: f64 = (p.p178 * locals.var_vbdj);
            let assign32170_e47126: f64 = (assign32170_e47124 * locals.var_isbd2);
            let assign32170_e47127: f64 = (locals.var_ibd + assign32170_e47126);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32170_e47127, (locals.var_ibd_dn0 + (assign32170_e47124 * locals.var_isbd2_dn0)), (locals.var_ibd_dn2 + (assign32170_e47124 * locals.var_isbd2_dn2)), (locals.var_ibd_dn6 + (((p.p178 * locals.var_vbdj_dn6) * locals.var_isbd2) + (assign32170_e47124 * locals.var_isbd2_dn6))), (locals.var_ibd_dn7 + (assign32170_e47124 * locals.var_isbd2_dn7)), (locals.var_ibd_dn10 + (assign32170_e47124 * locals.var_isbd2_dn10)), (locals.var_ibd_dn11 + (assign32170_e47124 * locals.var_isbd2_dn11)), (locals.var_ibd_dn12 + (((p.p178 * locals.var_vbdj_dn12) * locals.var_isbd2) + (assign32170_e47124 * locals.var_isbd2_dn12))), (locals.var_ibd_dn17 + (assign32170_e47124 * locals.var_isbd2_dn17)), );
            locals.var_ibd_rv = 0.0;
        }
        let assign32180_e47132: f64 = if locals.var_vbsj < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign32180_e47132;
        locals.var_guard1062_rv = 0.0;
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1062 != 0.0)) {
            let assign32190_e47138: f64 = (locals.var_vbsj / locals.var_nvtm);
            let assign32190_e47139: f64 = (assign32190_e47138).exp();
            (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12, ) = (assign32190_e47139, 0.0, (assign32190_e47139 * (locals.var_vbsj_dn7 / locals.var_nvtm)), (assign32190_e47139 * (-((locals.var_vbsj * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)))), (assign32190_e47139 * (locals.var_vbsj_dn12 / locals.var_nvtm)), );
            locals.var_t1__blk1034_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1062 != 0.0)) {
            let assign32200_e47148: f64 = (locals.var_t1__blk1034 - 1.0);
            let assign32200_e47149: f64 = (locals.var_isbs * assign32200_e47148);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32200_e47149, (locals.var_isbs_dn0 * assign32200_e47148), (locals.var_isbs_dn2 * assign32200_e47148), ((locals.var_isbs_dn6 * assign32200_e47148) + (locals.var_isbs * locals.var_t1__blk1034_dn6)), ((locals.var_isbs_dn7 * assign32200_e47148) + (locals.var_isbs * locals.var_t1__blk1034_dn7)), ((locals.var_isbs_dn10 * assign32200_e47148) + (locals.var_isbs * locals.var_t1__blk1034_dn10)), (locals.var_isbs_dn11 * assign32200_e47148), ((locals.var_isbs_dn12 * assign32200_e47148) + (locals.var_isbs * locals.var_t1__blk1034_dn12)), (locals.var_isbs_dn17 * assign32200_e47148), );
            locals.var_ibs_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1062 == 0.0)) {
            let assign32210_e47158: f64 = (locals.var_vbst / locals.var_nvtm);
            let assign32210_e47159: f64 = (assign32210_e47158).exp();
            (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12, ) = (assign32210_e47159, 0.0, 0.0, (assign32210_e47159 * (((locals.var_vbst_dn10 * locals.var_nvtm) - (locals.var_vbst * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm))), 0.0, );
            locals.var_t1__blk1034_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1062 == 0.0)) {
            let assign32220_e47169: f64 = (locals.var_t1__blk1034 - 1.0);
            let assign32220_e47170: f64 = (locals.var_isbs * assign32220_e47169);
            let assign32220_e47173: f64 = (locals.var_isbs / locals.var_nvtm);
            let assign32220_e47175: f64 = (assign32220_e47173 * locals.var_t1__blk1034);
            let assign32220_e47178: f64 = (locals.var_vbsj - locals.var_vbst);
            let assign32220_e47179: f64 = (assign32220_e47175 * assign32220_e47178);
            let assign32220_e47180: f64 = (assign32220_e47170 + assign32220_e47179);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32220_e47180, ((locals.var_isbs_dn0 * assign32220_e47169) + (((locals.var_isbs_dn0 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32220_e47178)), ((locals.var_isbs_dn2 * assign32220_e47169) + (((locals.var_isbs_dn2 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32220_e47178)), (((locals.var_isbs_dn6 * assign32220_e47169) + (locals.var_isbs * locals.var_t1__blk1034_dn6)) + ((((locals.var_isbs_dn6 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32220_e47173 * locals.var_t1__blk1034_dn6)) * assign32220_e47178)), (((locals.var_isbs_dn7 * assign32220_e47169) + (locals.var_isbs * locals.var_t1__blk1034_dn7)) + (((((locals.var_isbs_dn7 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32220_e47173 * locals.var_t1__blk1034_dn7)) * assign32220_e47178) + (assign32220_e47175 * locals.var_vbsj_dn7))), (((locals.var_isbs_dn10 * assign32220_e47169) + (locals.var_isbs * locals.var_t1__blk1034_dn10)) + (((((((locals.var_isbs_dn10 * locals.var_nvtm) - (locals.var_isbs * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)) * locals.var_t1__blk1034) + (assign32220_e47173 * locals.var_t1__blk1034_dn10)) * assign32220_e47178) + (assign32220_e47175 * (-locals.var_vbst_dn10)))), ((locals.var_isbs_dn11 * assign32220_e47169) + (((locals.var_isbs_dn11 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32220_e47178)), (((locals.var_isbs_dn12 * assign32220_e47169) + (locals.var_isbs * locals.var_t1__blk1034_dn12)) + (((((locals.var_isbs_dn12 / locals.var_nvtm) * locals.var_t1__blk1034) + (assign32220_e47173 * locals.var_t1__blk1034_dn12)) * assign32220_e47178) + (assign32220_e47175 * locals.var_vbsj_dn12))), ((locals.var_isbs_dn17 * assign32220_e47169) + (((locals.var_isbs_dn17 / locals.var_nvtm) * locals.var_t1__blk1034) * assign32220_e47178)), );
            locals.var_ibs_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32230_e47187: f64 = (p.p178 * locals.var_vbsj);
            let assign32230_e47189: f64 = (assign32230_e47187 * locals.var_isbs2);
            let assign32230_e47190: f64 = (locals.var_ibs + assign32230_e47189);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32230_e47190, (locals.var_ibs_dn0 + (assign32230_e47187 * locals.var_isbs2_dn0)), (locals.var_ibs_dn2 + (assign32230_e47187 * locals.var_isbs2_dn2)), (locals.var_ibs_dn6 + (assign32230_e47187 * locals.var_isbs2_dn6)), (locals.var_ibs_dn7 + (((p.p178 * locals.var_vbsj_dn7) * locals.var_isbs2) + (assign32230_e47187 * locals.var_isbs2_dn7))), (locals.var_ibs_dn10 + (assign32230_e47187 * locals.var_isbs2_dn10)), (locals.var_ibs_dn11 + (assign32230_e47187 * locals.var_isbs2_dn11)), (locals.var_ibs_dn12 + (((p.p178 * locals.var_vbsj_dn12) * locals.var_isbs2) + (assign32230_e47187 * locals.var_isbs2_dn12))), (locals.var_ibs_dn17 + (assign32230_e47187 * locals.var_isbs2_dn17)), );
            locals.var_ibs_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32240_e47197: f64 = (locals.var_gjmin * locals.var_vbdj);
            let assign32240_e47198: f64 = (locals.var_ibd + assign32240_e47197);
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (assign32240_e47198, locals.var_ibd_dn0, locals.var_ibd_dn2, (locals.var_ibd_dn6 + (locals.var_gjmin * locals.var_vbdj_dn6)), locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, (locals.var_ibd_dn12 + (locals.var_gjmin * locals.var_vbdj_dn12)), locals.var_ibd_dn17, );
            locals.var_ibd_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32250_e47205: f64 = (locals.var_gjmin * locals.var_vbsj);
            let assign32250_e47206: f64 = (locals.var_ibs + assign32250_e47205);
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (assign32250_e47206, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, (locals.var_ibs_dn7 + (locals.var_gjmin * locals.var_vbsj_dn7)), locals.var_ibs_dn10, locals.var_ibs_dn11, (locals.var_ibs_dn12 + (locals.var_gjmin * locals.var_vbsj_dn12)), locals.var_ibs_dn17, );
            locals.var_ibs_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32260_e47212: f64 = (p.p179 * p.p2);
            locals.var_czbd = assign32260_e47212;
            locals.var_czbd_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32270_e47218: f64 = (p.p179 * p.p3);
            locals.var_czbs = assign32270_e47218;
            locals.var_czbs_rv = 0.0;
        }
        if (locals.var_guard1032 != 0.0) {
            let assign32280_e47224: f64 = (p.p237 - p.p238);
            locals.var_xp_max = assign32280_e47224;
            locals.var_xp_max_rv = 0.0;
        }
        let assign32290_e47229: f64 = if locals.var_xp_max <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign32290_e47229;
        locals.var_guard1063_rv = 0.0;
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1063 != 0.0)) {
            locals.var_czbd = 0.0;
            locals.var_czbd_rv = 0.0;
            locals.var_czbs = 0.0;
            locals.var_czbs_rv = 0.0;
        }
        let assign32320_e47244: f64 = if p.p5 > locals.var_w_dioscv { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign32320_e47244;
        locals.var_guard1064_rv = 0.0;
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) {
            let assign32330_e47251: f64 = (p.p5 - locals.var_w_dioscv);
            let assign32330_e47252: f64 = (p.p180 * assign32330_e47251);
            locals.var_czbssw = assign32330_e47252;
            locals.var_czbssw_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) {
            let assign32340_e47260: f64 = (p.p181 * locals.var_w_dioscv);
            locals.var_czbsswg = assign32340_e47260;
            locals.var_czbsswg_rv = 0.0;
        }
        let assign32350_e47265: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign32350_e47265;
        locals.var_guard1065_rv = 0.0;
        let assign32360_e47268: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign32360_e47268;
        locals.var_guard1066_rv = 0.0;
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 != 0.0)) {
            let assign32370_e47279: f64 = (locals.var_vbsj / p.p185);
            let assign32370_e47280: f64 = (1.0 - assign32370_e47279);
            (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12, ) = (assign32370_e47280, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)), );
            locals.var_arg__blk1059_rv = 0.0;
        }
        let assign32380_e47285: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign32380_e47285;
        locals.var_guard1067_rv = 0.0;
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 != 0.0)) {
            let assign32390_e47297: f64 = (locals.var_arg__blk1059).sqrt();
            let assign32390_e47298: f64 = (1.0 / assign32390_e47297);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32390_e47298, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32390_e47297)) / (assign32390_e47297 * assign32390_e47297))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32390_e47297)) / (assign32390_e47297 * assign32390_e47297))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32390_e47297)) / (assign32390_e47297 * assign32390_e47297))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 != 0.0)) && (locals.var_guard1067 == 0.0)) {
            let assign32400_e47313: f64 = (-p.p182);
            let assign32400_e47314: f64 = (locals.var_arg__blk1059).powf(assign32400_e47313);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32400_e47314, if 0.0 == 0.0 && ((assign32400_e47313) as f64).is_finite() && ((assign32400_e47313) as f64).fract() == 0.0 { if assign32400_e47313 == 0.0 { 0.0 } else { (assign32400_e47313 * ((locals.var_arg__blk1059).powf(assign32400_e47313 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32400_e47314 * (assign32400_e47313 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32400_e47313) as f64).is_finite() && ((assign32400_e47313) as f64).fract() == 0.0 { if assign32400_e47313 == 0.0 { 0.0 } else { (assign32400_e47313 * ((locals.var_arg__blk1059).powf(assign32400_e47313 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32400_e47314 * (assign32400_e47313 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32400_e47313) as f64).is_finite() && ((assign32400_e47313) as f64).fract() == 0.0 { if assign32400_e47313 == 0.0 { 0.0 } else { (assign32400_e47313 * ((locals.var_arg__blk1059).powf(assign32400_e47313 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32400_e47314 * (assign32400_e47313 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 != 0.0)) {
            let assign32410_e47326: f64 = (p.p185 * locals.var_czbs);
            let assign32410_e47330: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
            let assign32410_e47331: f64 = (1.0 - assign32410_e47330);
            let assign32410_e47332: f64 = (assign32410_e47326 * assign32410_e47331);
            let assign32410_e47335: f64 = (1.0 - p.p182);
            let assign32410_e47336: f64 = (assign32410_e47332 / assign32410_e47335);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32410_e47336, 0.0, 0.0, ((assign32410_e47326 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32410_e47335), ((assign32410_e47326 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32410_e47335), 0.0, 0.0, ((assign32410_e47326 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32410_e47335), 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1066 == 0.0)) {
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        let assign32430_e47352: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign32430_e47352;
        locals.var_guard1068_rv = 0.0;
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1068 != 0.0)) {
            let assign32440_e47363: f64 = (locals.var_vbsj / p.p186);
            let assign32440_e47364: f64 = (1.0 - assign32440_e47363);
            (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12, ) = (assign32440_e47364, 0.0, (-(locals.var_vbsj_dn7 / p.p186)), (-(locals.var_vbsj_dn12 / p.p186)), );
            locals.var_arg__blk1059_rv = 0.0;
        }
        let assign32450_e47369: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign32450_e47369;
        locals.var_guard1069_rv = 0.0;
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
            let assign32460_e47381: f64 = (locals.var_arg__blk1059).sqrt();
            let assign32460_e47382: f64 = (1.0 / assign32460_e47381);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32460_e47382, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32460_e47381)) / (assign32460_e47381 * assign32460_e47381))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32460_e47381)) / (assign32460_e47381 * assign32460_e47381))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32460_e47381)) / (assign32460_e47381 * assign32460_e47381))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 == 0.0)) {
            let assign32470_e47397: f64 = (-p.p183);
            let assign32470_e47398: f64 = (locals.var_arg__blk1059).powf(assign32470_e47397);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32470_e47398, if 0.0 == 0.0 && ((assign32470_e47397) as f64).is_finite() && ((assign32470_e47397) as f64).fract() == 0.0 { if assign32470_e47397 == 0.0 { 0.0 } else { (assign32470_e47397 * ((locals.var_arg__blk1059).powf(assign32470_e47397 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32470_e47398 * (assign32470_e47397 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32470_e47397) as f64).is_finite() && ((assign32470_e47397) as f64).fract() == 0.0 { if assign32470_e47397 == 0.0 { 0.0 } else { (assign32470_e47397 * ((locals.var_arg__blk1059).powf(assign32470_e47397 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32470_e47398 * (assign32470_e47397 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32470_e47397) as f64).is_finite() && ((assign32470_e47397) as f64).fract() == 0.0 { if assign32470_e47397 == 0.0 { 0.0 } else { (assign32470_e47397 * ((locals.var_arg__blk1059).powf(assign32470_e47397 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32470_e47398 * (assign32470_e47397 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1068 != 0.0)) {
            let assign32480_e47411: f64 = (p.p186 * locals.var_czbssw);
            let assign32480_e47415: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
            let assign32480_e47416: f64 = (1.0 - assign32480_e47415);
            let assign32480_e47417: f64 = (assign32480_e47411 * assign32480_e47416);
            let assign32480_e47420: f64 = (1.0 - p.p183);
            let assign32480_e47421: f64 = (assign32480_e47417 / assign32480_e47420);
            let assign32480_e47422: f64 = (locals.var_qbs + assign32480_e47421);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32480_e47422, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32480_e47411 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32480_e47420)), (locals.var_qbs_dn7 + ((assign32480_e47411 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32480_e47420)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32480_e47411 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32480_e47420)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        let assign32490_e47427: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign32490_e47427;
        locals.var_guard1070_rv = 0.0;
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1070 != 0.0)) {
            let assign32500_e47438: f64 = (locals.var_vbsj / p.p187);
            let assign32500_e47439: f64 = (1.0 - assign32500_e47438);
            (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12, ) = (assign32500_e47439, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)), );
            locals.var_arg__blk1059_rv = 0.0;
        }
        let assign32510_e47444: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign32510_e47444;
        locals.var_guard1071_rv = 0.0;
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 != 0.0)) {
            let assign32520_e47456: f64 = (locals.var_arg__blk1059).sqrt();
            let assign32520_e47457: f64 = (1.0 / assign32520_e47456);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32520_e47457, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32520_e47456)) / (assign32520_e47456 * assign32520_e47456))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32520_e47456)) / (assign32520_e47456 * assign32520_e47456))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32520_e47456)) / (assign32520_e47456 * assign32520_e47456))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1070 != 0.0)) && (locals.var_guard1071 == 0.0)) {
            let assign32530_e47472: f64 = (-p.p184);
            let assign32530_e47473: f64 = (locals.var_arg__blk1059).powf(assign32530_e47472);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32530_e47473, if 0.0 == 0.0 && ((assign32530_e47472) as f64).is_finite() && ((assign32530_e47472) as f64).fract() == 0.0 { if assign32530_e47472 == 0.0 { 0.0 } else { (assign32530_e47472 * ((locals.var_arg__blk1059).powf(assign32530_e47472 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32530_e47473 * (assign32530_e47472 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32530_e47472) as f64).is_finite() && ((assign32530_e47472) as f64).fract() == 0.0 { if assign32530_e47472 == 0.0 { 0.0 } else { (assign32530_e47472 * ((locals.var_arg__blk1059).powf(assign32530_e47472 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32530_e47473 * (assign32530_e47472 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32530_e47472) as f64).is_finite() && ((assign32530_e47472) as f64).fract() == 0.0 { if assign32530_e47472 == 0.0 { 0.0 } else { (assign32530_e47472 * ((locals.var_arg__blk1059).powf(assign32530_e47472 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32530_e47473 * (assign32530_e47472 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 != 0.0)) && (locals.var_guard1070 != 0.0)) {
            let assign32540_e47486: f64 = (p.p187 * locals.var_czbsswg);
            let assign32540_e47490: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
            let assign32540_e47491: f64 = (1.0 - assign32540_e47490);
            let assign32540_e47492: f64 = (assign32540_e47486 * assign32540_e47491);
            let assign32540_e47495: f64 = (1.0 - p.p184);
            let assign32540_e47496: f64 = (assign32540_e47492 / assign32540_e47495);
            let assign32540_e47497: f64 = (locals.var_qbs + assign32540_e47496);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32540_e47497, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32540_e47486 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32540_e47495)), (locals.var_qbs_dn7 + ((assign32540_e47486 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32540_e47495)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32540_e47486 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32540_e47495)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
            let assign32550_e47508: f64 = (locals.var_czbs + locals.var_czbssw);
            let assign32550_e47510: f64 = (assign32550_e47508 + locals.var_czbsswg);
            (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12, ) = (assign32550_e47510, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1034_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
            let assign32560_e47521: f64 = (locals.var_czbs * p.p182);
            let assign32560_e47523: f64 = (assign32560_e47521 / p.p185);
            let assign32560_e47526: f64 = (locals.var_czbssw * p.p183);
            let assign32560_e47528: f64 = (assign32560_e47526 / p.p186);
            let assign32560_e47529: f64 = (assign32560_e47523 + assign32560_e47528);
            let assign32560_e47532: f64 = (locals.var_czbsswg * p.p184);
            let assign32560_e47534: f64 = (assign32560_e47532 / p.p187);
            let assign32560_e47535: f64 = (assign32560_e47529 + assign32560_e47534);
            (locals.var_t2__blk1035, locals.var_t2__blk1035_dn0, locals.var_t2__blk1035_dn2, locals.var_t2__blk1035_dn6, locals.var_t2__blk1035_dn7, locals.var_t2__blk1035_dn10, locals.var_t2__blk1035_dn11, locals.var_t2__blk1035_dn12, locals.var_t2__blk1035_dn17, ) = (assign32560_e47535, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1035_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 != 0.0)) && (locals.var_guard1065 == 0.0)) {
            let assign32570_e47548: f64 = (locals.var_vbsj * 0.5);
            let assign32570_e47550: f64 = (assign32570_e47548 * locals.var_t2__blk1035);
            let assign32570_e47551: f64 = (locals.var_t1__blk1034 + assign32570_e47550);
            let assign32570_e47552: f64 = (locals.var_vbsj * assign32570_e47551);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32570_e47552, (locals.var_vbsj * (assign32570_e47548 * locals.var_t2__blk1035_dn0)), (locals.var_vbsj * (assign32570_e47548 * locals.var_t2__blk1035_dn2)), (locals.var_vbsj * (locals.var_t1__blk1034_dn6 + (assign32570_e47548 * locals.var_t2__blk1035_dn6))), ((locals.var_vbsj_dn7 * assign32570_e47551) + (locals.var_vbsj * (locals.var_t1__blk1034_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1035) + (assign32570_e47548 * locals.var_t2__blk1035_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1034_dn10 + (assign32570_e47548 * locals.var_t2__blk1035_dn10))), (locals.var_vbsj * (assign32570_e47548 * locals.var_t2__blk1035_dn11)), ((locals.var_vbsj_dn12 * assign32570_e47551) + (locals.var_vbsj * (locals.var_t1__blk1034_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1035) + (assign32570_e47548 * locals.var_t2__blk1035_dn12))))), (locals.var_vbsj * (assign32570_e47548 * locals.var_t2__blk1035_dn17)), );
            locals.var_qbs_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) {
            let assign32580_e47561: f64 = (p.p181 * p.p5);
            locals.var_czbsswg = assign32580_e47561;
            locals.var_czbsswg_rv = 0.0;
        }
        let assign32590_e47566: f64 = if locals.var_vbsj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign32590_e47566;
        locals.var_guard1072_rv = 0.0;
        let assign32600_e47569: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign32600_e47569;
        locals.var_guard1073_rv = 0.0;
    }
    pub(super) fn stamp_reactive_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) {
            let assign32610_e47581: f64 = (locals.var_vbsj / p.p185);
            let assign32610_e47582: f64 = (1.0 - assign32610_e47581);
            (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12, ) = (assign32610_e47582, 0.0, (-(locals.var_vbsj_dn7 / p.p185)), (-(locals.var_vbsj_dn12 / p.p185)), );
            locals.var_arg__blk1059_rv = 0.0;
        }
        let assign32620_e47587: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign32620_e47587;
        locals.var_guard1074_rv = 0.0;
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 != 0.0)) {
            let assign32630_e47600: f64 = (locals.var_arg__blk1059).sqrt();
            let assign32630_e47601: f64 = (1.0 / assign32630_e47600);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32630_e47601, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32630_e47600)) / (assign32630_e47600 * assign32630_e47600))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32630_e47600)) / (assign32630_e47600 * assign32630_e47600))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32630_e47600)) / (assign32630_e47600 * assign32630_e47600))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) && (locals.var_guard1074 == 0.0)) {
            let assign32640_e47617: f64 = (-p.p182);
            let assign32640_e47618: f64 = (locals.var_arg__blk1059).powf(assign32640_e47617);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32640_e47618, if 0.0 == 0.0 && ((assign32640_e47617) as f64).is_finite() && ((assign32640_e47617) as f64).fract() == 0.0 { if assign32640_e47617 == 0.0 { 0.0 } else { (assign32640_e47617 * ((locals.var_arg__blk1059).powf(assign32640_e47617 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32640_e47618 * (assign32640_e47617 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32640_e47617) as f64).is_finite() && ((assign32640_e47617) as f64).fract() == 0.0 { if assign32640_e47617 == 0.0 { 0.0 } else { (assign32640_e47617 * ((locals.var_arg__blk1059).powf(assign32640_e47617 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32640_e47618 * (assign32640_e47617 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32640_e47617) as f64).is_finite() && ((assign32640_e47617) as f64).fract() == 0.0 { if assign32640_e47617 == 0.0 { 0.0 } else { (assign32640_e47617 * ((locals.var_arg__blk1059).powf(assign32640_e47617 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32640_e47618 * (assign32640_e47617 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 != 0.0)) {
            let assign32650_e47631: f64 = (p.p185 * locals.var_czbs);
            let assign32650_e47635: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
            let assign32650_e47636: f64 = (1.0 - assign32650_e47635);
            let assign32650_e47637: f64 = (assign32650_e47631 * assign32650_e47636);
            let assign32650_e47640: f64 = (1.0 - p.p182);
            let assign32650_e47641: f64 = (assign32650_e47637 / assign32650_e47640);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32650_e47641, 0.0, 0.0, ((assign32650_e47631 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32650_e47640), ((assign32650_e47631 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32650_e47640), 0.0, 0.0, ((assign32650_e47631 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32650_e47640), 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1073 == 0.0)) {
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_rv = 0.0;
        }
        let assign32670_e47658: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign32670_e47658;
        locals.var_guard1075_rv = 0.0;
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1075 != 0.0)) {
            let assign32680_e47670: f64 = (locals.var_vbsj / p.p187);
            let assign32680_e47671: f64 = (1.0 - assign32680_e47670);
            (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12, ) = (assign32680_e47671, 0.0, (-(locals.var_vbsj_dn7 / p.p187)), (-(locals.var_vbsj_dn12 / p.p187)), );
            locals.var_arg__blk1059_rv = 0.0;
        }
        let assign32690_e47676: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign32690_e47676;
        locals.var_guard1076_rv = 0.0;
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) {
            let assign32700_e47689: f64 = (locals.var_arg__blk1059).sqrt();
            let assign32700_e47690: f64 = (1.0 / assign32700_e47689);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32700_e47690, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32700_e47689)) / (assign32700_e47689 * assign32700_e47689))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32700_e47689)) / (assign32700_e47689 * assign32700_e47689))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32700_e47689)) / (assign32700_e47689 * assign32700_e47689))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) {
            let assign32710_e47706: f64 = (-p.p184);
            let assign32710_e47707: f64 = (locals.var_arg__blk1059).powf(assign32710_e47706);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32710_e47707, if 0.0 == 0.0 && ((assign32710_e47706) as f64).is_finite() && ((assign32710_e47706) as f64).fract() == 0.0 { if assign32710_e47706 == 0.0 { 0.0 } else { (assign32710_e47706 * ((locals.var_arg__blk1059).powf(assign32710_e47706 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32710_e47707 * (assign32710_e47706 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32710_e47706) as f64).is_finite() && ((assign32710_e47706) as f64).fract() == 0.0 { if assign32710_e47706 == 0.0 { 0.0 } else { (assign32710_e47706 * ((locals.var_arg__blk1059).powf(assign32710_e47706 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32710_e47707 * (assign32710_e47706 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32710_e47706) as f64).is_finite() && ((assign32710_e47706) as f64).fract() == 0.0 { if assign32710_e47706 == 0.0 { 0.0 } else { (assign32710_e47706 * ((locals.var_arg__blk1059).powf(assign32710_e47706 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32710_e47707 * (assign32710_e47706 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 != 0.0)) && (locals.var_guard1075 != 0.0)) {
            let assign32720_e47721: f64 = (p.p187 * locals.var_czbsswg);
            let assign32720_e47725: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
            let assign32720_e47726: f64 = (1.0 - assign32720_e47725);
            let assign32720_e47727: f64 = (assign32720_e47721 * assign32720_e47726);
            let assign32720_e47730: f64 = (1.0 - p.p184);
            let assign32720_e47731: f64 = (assign32720_e47727 / assign32720_e47730);
            let assign32720_e47732: f64 = (locals.var_qbs + assign32720_e47731);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32720_e47732, locals.var_qbs_dn0, locals.var_qbs_dn2, (locals.var_qbs_dn6 + ((assign32720_e47721 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32720_e47730)), (locals.var_qbs_dn7 + ((assign32720_e47721 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32720_e47730)), locals.var_qbs_dn10, locals.var_qbs_dn11, (locals.var_qbs_dn12 + ((assign32720_e47721 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32720_e47730)), locals.var_qbs_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 == 0.0)) {
            let assign32730_e47744: f64 = (locals.var_czbs + locals.var_czbsswg);
            (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12, ) = (assign32730_e47744, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1034_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 == 0.0)) {
            let assign32740_e47756: f64 = (locals.var_czbs * p.p182);
            let assign32740_e47758: f64 = (assign32740_e47756 / p.p185);
            let assign32740_e47761: f64 = (locals.var_czbsswg * p.p184);
            let assign32740_e47763: f64 = (assign32740_e47761 / p.p187);
            let assign32740_e47764: f64 = (assign32740_e47758 + assign32740_e47763);
            (locals.var_t2__blk1035, locals.var_t2__blk1035_dn0, locals.var_t2__blk1035_dn2, locals.var_t2__blk1035_dn6, locals.var_t2__blk1035_dn7, locals.var_t2__blk1035_dn10, locals.var_t2__blk1035_dn11, locals.var_t2__blk1035_dn12, locals.var_t2__blk1035_dn17, ) = (assign32740_e47764, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1035_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1072 == 0.0)) {
            let assign32750_e47778: f64 = (locals.var_vbsj * 0.5);
            let assign32750_e47780: f64 = (assign32750_e47778 * locals.var_t2__blk1035);
            let assign32750_e47781: f64 = (locals.var_t1__blk1034 + assign32750_e47780);
            let assign32750_e47782: f64 = (locals.var_vbsj * assign32750_e47781);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign32750_e47782, (locals.var_vbsj * (assign32750_e47778 * locals.var_t2__blk1035_dn0)), (locals.var_vbsj * (assign32750_e47778 * locals.var_t2__blk1035_dn2)), (locals.var_vbsj * (locals.var_t1__blk1034_dn6 + (assign32750_e47778 * locals.var_t2__blk1035_dn6))), ((locals.var_vbsj_dn7 * assign32750_e47781) + (locals.var_vbsj * (locals.var_t1__blk1034_dn7 + (((locals.var_vbsj_dn7 * 0.5) * locals.var_t2__blk1035) + (assign32750_e47778 * locals.var_t2__blk1035_dn7))))), (locals.var_vbsj * (locals.var_t1__blk1034_dn10 + (assign32750_e47778 * locals.var_t2__blk1035_dn10))), (locals.var_vbsj * (assign32750_e47778 * locals.var_t2__blk1035_dn11)), ((locals.var_vbsj_dn12 * assign32750_e47781) + (locals.var_vbsj * (locals.var_t1__blk1034_dn12 + (((locals.var_vbsj_dn12 * 0.5) * locals.var_t2__blk1035) + (assign32750_e47778 * locals.var_t2__blk1035_dn12))))), (locals.var_vbsj * (assign32750_e47778 * locals.var_t2__blk1035_dn17)), );
            locals.var_qbs_rv = 0.0;
        }
        let assign32760_e47787: f64 = if p.p4 > locals.var_w_diodcv { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign32760_e47787;
        locals.var_guard1077_rv = 0.0;
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) {
            let assign32770_e47794: f64 = (p.p4 - locals.var_w_diodcv);
            let assign32770_e47795: f64 = (p.p180 * assign32770_e47794);
            locals.var_czbdsw = assign32770_e47795;
            locals.var_czbdsw_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) {
            let assign32780_e47803: f64 = (p.p181 * locals.var_w_diodcv);
            locals.var_czbdswg = assign32780_e47803;
            locals.var_czbdswg_rv = 0.0;
        }
        let assign32790_e47808: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign32790_e47808;
        locals.var_guard1078_rv = 0.0;
        let assign32800_e47811: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign32800_e47811;
        locals.var_guard1079_rv = 0.0;
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 != 0.0)) {
            let assign32810_e47822: f64 = (locals.var_vbdj / p.p185);
            let assign32810_e47823: f64 = (1.0 - assign32810_e47822);
            (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12, ) = (assign32810_e47823, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)), );
            locals.var_arg__blk1059_rv = 0.0;
        }
        let assign32820_e47828: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1080 = assign32820_e47828;
        locals.var_guard1080_rv = 0.0;
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 != 0.0)) {
            let assign32830_e47840: f64 = (locals.var_arg__blk1059).sqrt();
            let assign32830_e47841: f64 = (1.0 / assign32830_e47840);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32830_e47841, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32830_e47840)) / (assign32830_e47840 * assign32830_e47840))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32830_e47840)) / (assign32830_e47840 * assign32830_e47840))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32830_e47840)) / (assign32830_e47840 * assign32830_e47840))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 != 0.0)) && (locals.var_guard1080 == 0.0)) {
            let assign32840_e47856: f64 = (-p.p182);
            let assign32840_e47857: f64 = (locals.var_arg__blk1059).powf(assign32840_e47856);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32840_e47857, if 0.0 == 0.0 && ((assign32840_e47856) as f64).is_finite() && ((assign32840_e47856) as f64).fract() == 0.0 { if assign32840_e47856 == 0.0 { 0.0 } else { (assign32840_e47856 * ((locals.var_arg__blk1059).powf(assign32840_e47856 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32840_e47857 * (assign32840_e47856 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32840_e47856) as f64).is_finite() && ((assign32840_e47856) as f64).fract() == 0.0 { if assign32840_e47856 == 0.0 { 0.0 } else { (assign32840_e47856 * ((locals.var_arg__blk1059).powf(assign32840_e47856 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32840_e47857 * (assign32840_e47856 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32840_e47856) as f64).is_finite() && ((assign32840_e47856) as f64).fract() == 0.0 { if assign32840_e47856 == 0.0 { 0.0 } else { (assign32840_e47856 * ((locals.var_arg__blk1059).powf(assign32840_e47856 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32840_e47857 * (assign32840_e47856 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 != 0.0)) {
            let assign32850_e47869: f64 = (p.p185 * locals.var_czbd);
            let assign32850_e47873: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
            let assign32850_e47874: f64 = (1.0 - assign32850_e47873);
            let assign32850_e47875: f64 = (assign32850_e47869 * assign32850_e47874);
            let assign32850_e47878: f64 = (1.0 - p.p182);
            let assign32850_e47879: f64 = (assign32850_e47875 / assign32850_e47878);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32850_e47879, 0.0, 0.0, ((assign32850_e47869 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32850_e47878), ((assign32850_e47869 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32850_e47878), 0.0, 0.0, ((assign32850_e47869 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32850_e47878), 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1079 == 0.0)) {
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        let assign32870_e47895: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1081 = assign32870_e47895;
        locals.var_guard1081_rv = 0.0;
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1081 != 0.0)) {
            let assign32880_e47906: f64 = (locals.var_vbdj / p.p186);
            let assign32880_e47907: f64 = (1.0 - assign32880_e47906);
            (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12, ) = (assign32880_e47907, (-(locals.var_vbdj_dn6 / p.p186)), 0.0, (-(locals.var_vbdj_dn12 / p.p186)), );
            locals.var_arg__blk1059_rv = 0.0;
        }
        let assign32890_e47912: f64 = if p.p183 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1082 = assign32890_e47912;
        locals.var_guard1082_rv = 0.0;
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 != 0.0)) {
            let assign32900_e47924: f64 = (locals.var_arg__blk1059).sqrt();
            let assign32900_e47925: f64 = (1.0 / assign32900_e47924);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32900_e47925, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32900_e47924)) / (assign32900_e47924 * assign32900_e47924))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32900_e47924)) / (assign32900_e47924 * assign32900_e47924))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32900_e47924)) / (assign32900_e47924 * assign32900_e47924))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1081 != 0.0)) && (locals.var_guard1082 == 0.0)) {
            let assign32910_e47940: f64 = (-p.p183);
            let assign32910_e47941: f64 = (locals.var_arg__blk1059).powf(assign32910_e47940);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32910_e47941, if 0.0 == 0.0 && ((assign32910_e47940) as f64).is_finite() && ((assign32910_e47940) as f64).fract() == 0.0 { if assign32910_e47940 == 0.0 { 0.0 } else { (assign32910_e47940 * ((locals.var_arg__blk1059).powf(assign32910_e47940 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32910_e47941 * (assign32910_e47940 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32910_e47940) as f64).is_finite() && ((assign32910_e47940) as f64).fract() == 0.0 { if assign32910_e47940 == 0.0 { 0.0 } else { (assign32910_e47940 * ((locals.var_arg__blk1059).powf(assign32910_e47940 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32910_e47941 * (assign32910_e47940 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32910_e47940) as f64).is_finite() && ((assign32910_e47940) as f64).fract() == 0.0 { if assign32910_e47940 == 0.0 { 0.0 } else { (assign32910_e47940 * ((locals.var_arg__blk1059).powf(assign32910_e47940 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32910_e47941 * (assign32910_e47940 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1081 != 0.0)) {
            let assign32920_e47954: f64 = (p.p186 * locals.var_czbdsw);
            let assign32920_e47958: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
            let assign32920_e47959: f64 = (1.0 - assign32920_e47958);
            let assign32920_e47960: f64 = (assign32920_e47954 * assign32920_e47959);
            let assign32920_e47963: f64 = (1.0 - p.p183);
            let assign32920_e47964: f64 = (assign32920_e47960 / assign32920_e47963);
            let assign32920_e47965: f64 = (locals.var_qbd + assign32920_e47964);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32920_e47965, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32920_e47954 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32920_e47963)), (locals.var_qbd_dn7 + ((assign32920_e47954 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32920_e47963)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32920_e47954 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32920_e47963)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }
        let assign32930_e47970: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1083 = assign32930_e47970;
        locals.var_guard1083_rv = 0.0;
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1083 != 0.0)) {
            let assign32940_e47981: f64 = (locals.var_vbdj / p.p187);
            let assign32940_e47982: f64 = (1.0 - assign32940_e47981);
            (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12, ) = (assign32940_e47982, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)), );
            locals.var_arg__blk1059_rv = 0.0;
        }
        let assign32950_e47987: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1084 = assign32950_e47987;
        locals.var_guard1084_rv = 0.0;
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 != 0.0)) {
            let assign32960_e47999: f64 = (locals.var_arg__blk1059).sqrt();
            let assign32960_e48000: f64 = (1.0 / assign32960_e47999);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32960_e48000, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign32960_e47999)) / (assign32960_e47999 * assign32960_e47999))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign32960_e47999)) / (assign32960_e47999 * assign32960_e47999))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign32960_e47999)) / (assign32960_e47999 * assign32960_e47999))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1083 != 0.0)) && (locals.var_guard1084 == 0.0)) {
            let assign32970_e48015: f64 = (-p.p184);
            let assign32970_e48016: f64 = (locals.var_arg__blk1059).powf(assign32970_e48015);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign32970_e48016, if 0.0 == 0.0 && ((assign32970_e48015) as f64).is_finite() && ((assign32970_e48015) as f64).fract() == 0.0 { if assign32970_e48015 == 0.0 { 0.0 } else { (assign32970_e48015 * ((locals.var_arg__blk1059).powf(assign32970_e48015 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign32970_e48016 * (assign32970_e48015 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32970_e48015) as f64).is_finite() && ((assign32970_e48015) as f64).fract() == 0.0 { if assign32970_e48015 == 0.0 { 0.0 } else { (assign32970_e48015 * ((locals.var_arg__blk1059).powf(assign32970_e48015 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign32970_e48016 * (assign32970_e48015 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign32970_e48015) as f64).is_finite() && ((assign32970_e48015) as f64).fract() == 0.0 { if assign32970_e48015 == 0.0 { 0.0 } else { (assign32970_e48015 * ((locals.var_arg__blk1059).powf(assign32970_e48015 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign32970_e48016 * (assign32970_e48015 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 != 0.0)) && (locals.var_guard1083 != 0.0)) {
            let assign32980_e48029: f64 = (p.p187 * locals.var_czbdswg);
            let assign32980_e48033: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
            let assign32980_e48034: f64 = (1.0 - assign32980_e48033);
            let assign32980_e48035: f64 = (assign32980_e48029 * assign32980_e48034);
            let assign32980_e48038: f64 = (1.0 - p.p184);
            let assign32980_e48039: f64 = (assign32980_e48035 / assign32980_e48038);
            let assign32980_e48040: f64 = (locals.var_qbd + assign32980_e48039);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign32980_e48040, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign32980_e48029 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign32980_e48038)), (locals.var_qbd_dn7 + ((assign32980_e48029 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign32980_e48038)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign32980_e48029 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign32980_e48038)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
            let assign32990_e48051: f64 = (locals.var_czbd + locals.var_czbdsw);
            let assign32990_e48053: f64 = (assign32990_e48051 + locals.var_czbdswg);
            (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12, ) = (assign32990_e48053, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1034_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
            let assign33000_e48064: f64 = (locals.var_czbd * p.p182);
            let assign33000_e48066: f64 = (assign33000_e48064 / p.p185);
            let assign33000_e48069: f64 = (locals.var_czbdsw * p.p183);
            let assign33000_e48071: f64 = (assign33000_e48069 / p.p186);
            let assign33000_e48072: f64 = (assign33000_e48066 + assign33000_e48071);
            let assign33000_e48075: f64 = (locals.var_czbdswg * p.p184);
            let assign33000_e48077: f64 = (assign33000_e48075 / p.p187);
            let assign33000_e48078: f64 = (assign33000_e48072 + assign33000_e48077);
            (locals.var_t2__blk1035, locals.var_t2__blk1035_dn0, locals.var_t2__blk1035_dn2, locals.var_t2__blk1035_dn6, locals.var_t2__blk1035_dn7, locals.var_t2__blk1035_dn10, locals.var_t2__blk1035_dn11, locals.var_t2__blk1035_dn12, locals.var_t2__blk1035_dn17, ) = (assign33000_e48078, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1035_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 != 0.0)) && (locals.var_guard1078 == 0.0)) {
            let assign33010_e48091: f64 = (locals.var_vbdj * 0.5);
            let assign33010_e48093: f64 = (assign33010_e48091 * locals.var_t2__blk1035);
            let assign33010_e48094: f64 = (locals.var_t1__blk1034 + assign33010_e48093);
            let assign33010_e48095: f64 = (locals.var_vbdj * assign33010_e48094);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33010_e48095, (locals.var_vbdj * (assign33010_e48091 * locals.var_t2__blk1035_dn0)), (locals.var_vbdj * (assign33010_e48091 * locals.var_t2__blk1035_dn2)), ((locals.var_vbdj_dn6 * assign33010_e48094) + (locals.var_vbdj * (locals.var_t1__blk1034_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1035) + (assign33010_e48091 * locals.var_t2__blk1035_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1034_dn7 + (assign33010_e48091 * locals.var_t2__blk1035_dn7))), (locals.var_vbdj * (locals.var_t1__blk1034_dn10 + (assign33010_e48091 * locals.var_t2__blk1035_dn10))), (locals.var_vbdj * (assign33010_e48091 * locals.var_t2__blk1035_dn11)), ((locals.var_vbdj_dn12 * assign33010_e48094) + (locals.var_vbdj * (locals.var_t1__blk1034_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1035) + (assign33010_e48091 * locals.var_t2__blk1035_dn12))))), (locals.var_vbdj * (assign33010_e48091 * locals.var_t2__blk1035_dn17)), );
            locals.var_qbd_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) {
            let assign33020_e48104: f64 = (p.p181 * p.p4);
            locals.var_czbdswg = assign33020_e48104;
            locals.var_czbdswg_rv = 0.0;
        }
        let assign33030_e48109: f64 = if locals.var_vbdj < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1085 = assign33030_e48109;
        locals.var_guard1085_rv = 0.0;
        let assign33040_e48112: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1086 = assign33040_e48112;
        locals.var_guard1086_rv = 0.0;
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1086 != 0.0)) {
            let assign33050_e48124: f64 = (locals.var_vbdj / p.p185);
            let assign33050_e48125: f64 = (1.0 - assign33050_e48124);
            (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12, ) = (assign33050_e48125, (-(locals.var_vbdj_dn6 / p.p185)), 0.0, (-(locals.var_vbdj_dn12 / p.p185)), );
            locals.var_arg__blk1059_rv = 0.0;
        }
        let assign33060_e48130: f64 = if p.p182 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1087 = assign33060_e48130;
        locals.var_guard1087_rv = 0.0;
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1086 != 0.0)) && (locals.var_guard1087 != 0.0)) {
            let assign33070_e48143: f64 = (locals.var_arg__blk1059).sqrt();
            let assign33070_e48144: f64 = (1.0 / assign33070_e48143);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33070_e48144, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign33070_e48143)) / (assign33070_e48143 * assign33070_e48143))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign33070_e48143)) / (assign33070_e48143 * assign33070_e48143))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign33070_e48143)) / (assign33070_e48143 * assign33070_e48143))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1086 != 0.0)) && (locals.var_guard1087 == 0.0)) {
            let assign33080_e48160: f64 = (-p.p182);
            let assign33080_e48161: f64 = (locals.var_arg__blk1059).powf(assign33080_e48160);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33080_e48161, if 0.0 == 0.0 && ((assign33080_e48160) as f64).is_finite() && ((assign33080_e48160) as f64).fract() == 0.0 { if assign33080_e48160 == 0.0 { 0.0 } else { (assign33080_e48160 * ((locals.var_arg__blk1059).powf(assign33080_e48160 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign33080_e48161 * (assign33080_e48160 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33080_e48160) as f64).is_finite() && ((assign33080_e48160) as f64).fract() == 0.0 { if assign33080_e48160 == 0.0 { 0.0 } else { (assign33080_e48160 * ((locals.var_arg__blk1059).powf(assign33080_e48160 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign33080_e48161 * (assign33080_e48160 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33080_e48160) as f64).is_finite() && ((assign33080_e48160) as f64).fract() == 0.0 { if assign33080_e48160 == 0.0 { 0.0 } else { (assign33080_e48160 * ((locals.var_arg__blk1059).powf(assign33080_e48160 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign33080_e48161 * (assign33080_e48160 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1086 != 0.0)) {
            let assign33090_e48174: f64 = (p.p185 * locals.var_czbd);
            let assign33090_e48178: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
            let assign33090_e48179: f64 = (1.0 - assign33090_e48178);
            let assign33090_e48180: f64 = (assign33090_e48174 * assign33090_e48179);
            let assign33090_e48183: f64 = (1.0 - p.p182);
            let assign33090_e48184: f64 = (assign33090_e48180 / assign33090_e48183);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33090_e48184, 0.0, 0.0, ((assign33090_e48174 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign33090_e48183), ((assign33090_e48174 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign33090_e48183), 0.0, 0.0, ((assign33090_e48174 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign33090_e48183), 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1086 == 0.0)) {
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_rv = 0.0;
        }
        let assign33110_e48201: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1088 = assign33110_e48201;
        locals.var_guard1088_rv = 0.0;
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1088 != 0.0)) {
            let assign33120_e48213: f64 = (locals.var_vbdj / p.p187);
            let assign33120_e48214: f64 = (1.0 - assign33120_e48213);
            (locals.var_arg__blk1059, locals.var_arg__blk1059_dn6, locals.var_arg__blk1059_dn7, locals.var_arg__blk1059_dn12, ) = (assign33120_e48214, (-(locals.var_vbdj_dn6 / p.p187)), 0.0, (-(locals.var_vbdj_dn12 / p.p187)), );
            locals.var_arg__blk1059_rv = 0.0;
        }
        let assign33130_e48219: f64 = if p.p184 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1089 = assign33130_e48219;
        locals.var_guard1089_rv = 0.0;
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1088 != 0.0)) && (locals.var_guard1089 != 0.0)) {
            let assign33140_e48232: f64 = (locals.var_arg__blk1059).sqrt();
            let assign33140_e48233: f64 = (1.0 / assign33140_e48232);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33140_e48233, (-((locals.var_arg__blk1059_dn6 / (2.0 * assign33140_e48232)) / (assign33140_e48232 * assign33140_e48232))), (-((locals.var_arg__blk1059_dn7 / (2.0 * assign33140_e48232)) / (assign33140_e48232 * assign33140_e48232))), (-((locals.var_arg__blk1059_dn12 / (2.0 * assign33140_e48232)) / (assign33140_e48232 * assign33140_e48232))), );
            locals.var_sarg_rv = 0.0;
        }
        if (((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1088 != 0.0)) && (locals.var_guard1089 == 0.0)) {
            let assign33150_e48249: f64 = (-p.p184);
            let assign33150_e48250: f64 = (locals.var_arg__blk1059).powf(assign33150_e48249);
            (locals.var_sarg, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn12, ) = (assign33150_e48250, if 0.0 == 0.0 && ((assign33150_e48249) as f64).is_finite() && ((assign33150_e48249) as f64).fract() == 0.0 { if assign33150_e48249 == 0.0 { 0.0 } else { (assign33150_e48249 * ((locals.var_arg__blk1059).powf(assign33150_e48249 - 1.0) * locals.var_arg__blk1059_dn6)) } } else { (assign33150_e48250 * (assign33150_e48249 * (locals.var_arg__blk1059_dn6 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33150_e48249) as f64).is_finite() && ((assign33150_e48249) as f64).fract() == 0.0 { if assign33150_e48249 == 0.0 { 0.0 } else { (assign33150_e48249 * ((locals.var_arg__blk1059).powf(assign33150_e48249 - 1.0) * locals.var_arg__blk1059_dn7)) } } else { (assign33150_e48250 * (assign33150_e48249 * (locals.var_arg__blk1059_dn7 / locals.var_arg__blk1059))) }, if 0.0 == 0.0 && ((assign33150_e48249) as f64).is_finite() && ((assign33150_e48249) as f64).fract() == 0.0 { if assign33150_e48249 == 0.0 { 0.0 } else { (assign33150_e48249 * ((locals.var_arg__blk1059).powf(assign33150_e48249 - 1.0) * locals.var_arg__blk1059_dn12)) } } else { (assign33150_e48250 * (assign33150_e48249 * (locals.var_arg__blk1059_dn12 / locals.var_arg__blk1059))) }, );
            locals.var_sarg_rv = 0.0;
        }
        if ((((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 != 0.0)) && (locals.var_guard1088 != 0.0)) {
            let assign33160_e48264: f64 = (p.p187 * locals.var_czbdswg);
            let assign33160_e48268: f64 = (locals.var_arg__blk1059 * locals.var_sarg);
            let assign33160_e48269: f64 = (1.0 - assign33160_e48268);
            let assign33160_e48270: f64 = (assign33160_e48264 * assign33160_e48269);
            let assign33160_e48273: f64 = (1.0 - p.p184);
            let assign33160_e48274: f64 = (assign33160_e48270 / assign33160_e48273);
            let assign33160_e48275: f64 = (locals.var_qbd + assign33160_e48274);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33160_e48275, locals.var_qbd_dn0, locals.var_qbd_dn2, (locals.var_qbd_dn6 + ((assign33160_e48264 * (-((locals.var_arg__blk1059_dn6 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn6)))) / assign33160_e48273)), (locals.var_qbd_dn7 + ((assign33160_e48264 * (-((locals.var_arg__blk1059_dn7 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn7)))) / assign33160_e48273)), locals.var_qbd_dn10, locals.var_qbd_dn11, (locals.var_qbd_dn12 + ((assign33160_e48264 * (-((locals.var_arg__blk1059_dn12 * locals.var_sarg) + (locals.var_arg__blk1059 * locals.var_sarg_dn12)))) / assign33160_e48273)), locals.var_qbd_dn17, );
            locals.var_qbd_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 == 0.0)) {
            let assign33170_e48287: f64 = (locals.var_czbd + locals.var_czbdswg);
            (locals.var_t1__blk1034, locals.var_t1__blk1034_dn6, locals.var_t1__blk1034_dn7, locals.var_t1__blk1034_dn10, locals.var_t1__blk1034_dn12, ) = (assign33170_e48287, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1__blk1034_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 == 0.0)) {
            let assign33180_e48299: f64 = (locals.var_czbd * p.p182);
            let assign33180_e48301: f64 = (assign33180_e48299 / p.p185);
            let assign33180_e48304: f64 = (locals.var_czbdswg * p.p184);
            let assign33180_e48306: f64 = (assign33180_e48304 / p.p187);
            let assign33180_e48307: f64 = (assign33180_e48301 + assign33180_e48306);
            (locals.var_t2__blk1035, locals.var_t2__blk1035_dn0, locals.var_t2__blk1035_dn2, locals.var_t2__blk1035_dn6, locals.var_t2__blk1035_dn7, locals.var_t2__blk1035_dn10, locals.var_t2__blk1035_dn11, locals.var_t2__blk1035_dn12, locals.var_t2__blk1035_dn17, ) = (assign33180_e48307, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t2__blk1035_rv = 0.0;
        }
        if (((locals.var_guard1032 != 0.0) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1085 == 0.0)) {
            let assign33190_e48321: f64 = (locals.var_vbdj * 0.5);
            let assign33190_e48323: f64 = (assign33190_e48321 * locals.var_t2__blk1035);
            let assign33190_e48324: f64 = (locals.var_t1__blk1034 + assign33190_e48323);
            let assign33190_e48325: f64 = (locals.var_vbdj * assign33190_e48324);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33190_e48325, (locals.var_vbdj * (assign33190_e48321 * locals.var_t2__blk1035_dn0)), (locals.var_vbdj * (assign33190_e48321 * locals.var_t2__blk1035_dn2)), ((locals.var_vbdj_dn6 * assign33190_e48324) + (locals.var_vbdj * (locals.var_t1__blk1034_dn6 + (((locals.var_vbdj_dn6 * 0.5) * locals.var_t2__blk1035) + (assign33190_e48321 * locals.var_t2__blk1035_dn6))))), (locals.var_vbdj * (locals.var_t1__blk1034_dn7 + (assign33190_e48321 * locals.var_t2__blk1035_dn7))), (locals.var_vbdj * (locals.var_t1__blk1034_dn10 + (assign33190_e48321 * locals.var_t2__blk1035_dn10))), (locals.var_vbdj * (assign33190_e48321 * locals.var_t2__blk1035_dn11)), ((locals.var_vbdj_dn12 * assign33190_e48324) + (locals.var_vbdj * (locals.var_t1__blk1034_dn12 + (((locals.var_vbdj_dn12 * 0.5) * locals.var_t2__blk1035) + (assign33190_e48321 * locals.var_t2__blk1035_dn12))))), (locals.var_vbdj * (assign33190_e48321 * locals.var_t2__blk1035_dn17)), );
            locals.var_qbd_rv = 0.0;
        }
        let assign33200_e48330: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1090 = assign33200_e48330;
        locals.var_guard1090_rv = 0.0;
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
            let assign33210_e48335: f64 = (-1.6021918e-19);
            let assign33210_e48337: f64 = (assign33210_e48335 * locals.var_uc_nsubs);
            let assign33210_e48339: f64 = (assign33210_e48337 * locals.var_xp_max);
            let assign33210_e48341: f64 = (assign33210_e48339 * p.p3);
            (locals.var_qbs_max, locals.var_qbs_max_dn0, locals.var_qbs_max_dn2, locals.var_qbs_max_dn6, locals.var_qbs_max_dn7, locals.var_qbs_max_dn10, locals.var_qbs_max_dn11, locals.var_qbs_max_dn12, locals.var_qbs_max_dn17, ) = (assign33210_e48341, (((assign33210_e48335 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p3), (((assign33210_e48335 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p3), );
            locals.var_qbs_max_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
            let assign33220_e48349: f64 = (-locals.var_qbs_max);
            let assign33220_e48350: f64 = (0.001 * assign33220_e48349);
            (locals.var_dlt_qbs, locals.var_dlt_qbs_dn0, locals.var_dlt_qbs_dn2, locals.var_dlt_qbs_dn6, locals.var_dlt_qbs_dn7, locals.var_dlt_qbs_dn10, locals.var_dlt_qbs_dn11, locals.var_dlt_qbs_dn12, locals.var_dlt_qbs_dn17, ) = (assign33220_e48350, (0.001 * (-locals.var_qbs_max_dn0)), (0.001 * (-locals.var_qbs_max_dn2)), (0.001 * (-locals.var_qbs_max_dn6)), (0.001 * (-locals.var_qbs_max_dn7)), (0.001 * (-locals.var_qbs_max_dn10)), (0.001 * (-locals.var_qbs_max_dn11)), (0.001 * (-locals.var_qbs_max_dn12)), (0.001 * (-locals.var_qbs_max_dn17)), );
            locals.var_dlt_qbs_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
            let assign33230_e48357: f64 = (-locals.var_qbs_max);
            let assign33230_e48359: f64 = (-locals.var_qbs);
            let assign33230_e48360: f64 = (assign33230_e48357 - assign33230_e48359);
            let assign33230_e48362: f64 = (assign33230_e48360 - locals.var_dlt_qbs);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign33230_e48362, (((-locals.var_qbs_max_dn0) - (-locals.var_qbs_dn0)) - locals.var_dlt_qbs_dn0), (((-locals.var_qbs_max_dn2) - (-locals.var_qbs_dn2)) - locals.var_dlt_qbs_dn2), (((-locals.var_qbs_max_dn6) - (-locals.var_qbs_dn6)) - locals.var_dlt_qbs_dn6), (((-locals.var_qbs_max_dn7) - (-locals.var_qbs_dn7)) - locals.var_dlt_qbs_dn7), (((-locals.var_qbs_max_dn10) - (-locals.var_qbs_dn10)) - locals.var_dlt_qbs_dn10), (((-locals.var_qbs_max_dn11) - (-locals.var_qbs_dn11)) - locals.var_dlt_qbs_dn11), (((-locals.var_qbs_max_dn12) - (-locals.var_qbs_dn12)) - locals.var_dlt_qbs_dn12), (((-locals.var_qbs_max_dn17) - (-locals.var_qbs_dn17)) - locals.var_dlt_qbs_dn17), );
            locals.var_tmf1_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
            let assign33240_e48370: f64 = (-locals.var_qbs_max);
            let assign33240_e48371: f64 = (4.0 * assign33240_e48370);
            let assign33240_e48373: f64 = (assign33240_e48371 * locals.var_dlt_qbs);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33240_e48373, (((4.0 * (-locals.var_qbs_max_dn0)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn0)), (((4.0 * (-locals.var_qbs_max_dn2)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn2)), (((4.0 * (-locals.var_qbs_max_dn6)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn6)), (((4.0 * (-locals.var_qbs_max_dn7)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn7)), (((4.0 * (-locals.var_qbs_max_dn10)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn10)), (((4.0 * (-locals.var_qbs_max_dn11)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn11)), (((4.0 * (-locals.var_qbs_max_dn12)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn12)), (((4.0 * (-locals.var_qbs_max_dn17)) * locals.var_dlt_qbs) + (assign33240_e48371 * locals.var_dlt_qbs_dn17)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
            let (assign33250_e48385, assign33250_e48385_d_n0, assign33250_e48385_d_n2, assign33250_e48385_d_n6, assign33250_e48385_d_n7, assign33250_e48385_d_n10, assign33250_e48385_d_n11, assign33250_e48385_d_n12, assign33250_e48385_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign33250_e48384: f64 = (-locals.var_tmf2);
        (assign33250_e48384, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33250_e48385, assign33250_e48385_d_n0, assign33250_e48385_d_n2, assign33250_e48385_d_n6, assign33250_e48385_d_n7, assign33250_e48385_d_n10, assign33250_e48385_d_n11, assign33250_e48385_d_n12, assign33250_e48385_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
            let assign33260_e48393: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign33260_e48395: f64 = (assign33260_e48393 + locals.var_tmf2);
            let assign33260_e48396: f64 = (assign33260_e48395).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33260_e48396, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33260_e48396)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33260_e48396)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
            let assign33270_e48403: f64 = (-locals.var_qbs_max);
            let assign33270_e48407: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign33270_e48408: f64 = (0.5 * assign33270_e48407);
            let assign33270_e48409: f64 = (assign33270_e48403 - assign33270_e48408);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign33270_e48409, ((-locals.var_qbs_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbs_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbs_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbs_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbs_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbs_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbs_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbs_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_qbs_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1090 != 0.0)) {
            let assign33280_e48417: f64 = (-1.0);
            let assign33280_e48418: f64 = (locals.var_qbs * assign33280_e48417);
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (assign33280_e48418, (locals.var_qbs_dn0 * assign33280_e48417), (locals.var_qbs_dn2 * assign33280_e48417), (locals.var_qbs_dn6 * assign33280_e48417), (locals.var_qbs_dn7 * assign33280_e48417), (locals.var_qbs_dn10 * assign33280_e48417), (locals.var_qbs_dn11 * assign33280_e48417), (locals.var_qbs_dn12 * assign33280_e48417), (locals.var_qbs_dn17 * assign33280_e48417), );
            locals.var_qbs_rv = 0.0;
        }
        let assign33290_e48423: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1091 = assign33290_e48423;
        locals.var_guard1091_rv = 0.0;
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
            let assign33300_e48428: f64 = (-1.6021918e-19);
            let assign33300_e48430: f64 = (assign33300_e48428 * locals.var_uc_nsubs);
            let assign33300_e48432: f64 = (assign33300_e48430 * locals.var_xp_max);
            let assign33300_e48434: f64 = (assign33300_e48432 * p.p2);
            (locals.var_qbd_max, locals.var_qbd_max_dn0, locals.var_qbd_max_dn2, locals.var_qbd_max_dn6, locals.var_qbd_max_dn7, locals.var_qbd_max_dn10, locals.var_qbd_max_dn11, locals.var_qbd_max_dn12, locals.var_qbd_max_dn17, ) = (assign33300_e48434, (((assign33300_e48428 * locals.var_uc_nsubs_dn0) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn2) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn6) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn7) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn10) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn11) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn12) * locals.var_xp_max) * p.p2), (((assign33300_e48428 * locals.var_uc_nsubs_dn17) * locals.var_xp_max) * p.p2), );
            locals.var_qbd_max_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
            let assign33310_e48442: f64 = (-locals.var_qbd_max);
            let assign33310_e48443: f64 = (0.001 * assign33310_e48442);
            (locals.var_dlt_qbd, locals.var_dlt_qbd_dn0, locals.var_dlt_qbd_dn2, locals.var_dlt_qbd_dn6, locals.var_dlt_qbd_dn7, locals.var_dlt_qbd_dn10, locals.var_dlt_qbd_dn11, locals.var_dlt_qbd_dn12, locals.var_dlt_qbd_dn17, ) = (assign33310_e48443, (0.001 * (-locals.var_qbd_max_dn0)), (0.001 * (-locals.var_qbd_max_dn2)), (0.001 * (-locals.var_qbd_max_dn6)), (0.001 * (-locals.var_qbd_max_dn7)), (0.001 * (-locals.var_qbd_max_dn10)), (0.001 * (-locals.var_qbd_max_dn11)), (0.001 * (-locals.var_qbd_max_dn12)), (0.001 * (-locals.var_qbd_max_dn17)), );
            locals.var_dlt_qbd_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
            let assign33320_e48450: f64 = (-locals.var_qbd_max);
            let assign33320_e48452: f64 = (-locals.var_qbd);
            let assign33320_e48453: f64 = (assign33320_e48450 - assign33320_e48452);
            let assign33320_e48455: f64 = (assign33320_e48453 - locals.var_dlt_qbd);
            (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17, ) = (assign33320_e48455, (((-locals.var_qbd_max_dn0) - (-locals.var_qbd_dn0)) - locals.var_dlt_qbd_dn0), (((-locals.var_qbd_max_dn2) - (-locals.var_qbd_dn2)) - locals.var_dlt_qbd_dn2), (((-locals.var_qbd_max_dn6) - (-locals.var_qbd_dn6)) - locals.var_dlt_qbd_dn6), (((-locals.var_qbd_max_dn7) - (-locals.var_qbd_dn7)) - locals.var_dlt_qbd_dn7), (((-locals.var_qbd_max_dn10) - (-locals.var_qbd_dn10)) - locals.var_dlt_qbd_dn10), (((-locals.var_qbd_max_dn11) - (-locals.var_qbd_dn11)) - locals.var_dlt_qbd_dn11), (((-locals.var_qbd_max_dn12) - (-locals.var_qbd_dn12)) - locals.var_dlt_qbd_dn12), (((-locals.var_qbd_max_dn17) - (-locals.var_qbd_dn17)) - locals.var_dlt_qbd_dn17), );
            locals.var_tmf1_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
            let assign33330_e48463: f64 = (-locals.var_qbd_max);
            let assign33330_e48464: f64 = (4.0 * assign33330_e48463);
            let assign33330_e48466: f64 = (assign33330_e48464 * locals.var_dlt_qbd);
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33330_e48466, (((4.0 * (-locals.var_qbd_max_dn0)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn0)), (((4.0 * (-locals.var_qbd_max_dn2)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn2)), (((4.0 * (-locals.var_qbd_max_dn6)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn6)), (((4.0 * (-locals.var_qbd_max_dn7)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn7)), (((4.0 * (-locals.var_qbd_max_dn10)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn10)), (((4.0 * (-locals.var_qbd_max_dn11)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn11)), (((4.0 * (-locals.var_qbd_max_dn12)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn12)), (((4.0 * (-locals.var_qbd_max_dn17)) * locals.var_dlt_qbd) + (assign33330_e48464 * locals.var_dlt_qbd_dn17)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
            let (assign33340_e48478, assign33340_e48478_d_n0, assign33340_e48478_d_n2, assign33340_e48478_d_n6, assign33340_e48478_d_n7, assign33340_e48478_d_n10, assign33340_e48478_d_n11, assign33340_e48478_d_n12, assign33340_e48478_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign33340_e48477: f64 = (-locals.var_tmf2);
        (assign33340_e48477, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33340_e48478, assign33340_e48478_d_n0, assign33340_e48478_d_n2, assign33340_e48478_d_n6, assign33340_e48478_d_n7, assign33340_e48478_d_n10, assign33340_e48478_d_n11, assign33340_e48478_d_n12, assign33340_e48478_d_n17, );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
            let assign33350_e48486: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign33350_e48488: f64 = (assign33350_e48486 + locals.var_tmf2);
            let assign33350_e48489: f64 = (assign33350_e48488).sqrt();
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17, ) = (assign33350_e48489, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign33350_e48489)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign33350_e48489)), );
            locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
            let assign33360_e48496: f64 = (-locals.var_qbd_max);
            let assign33360_e48500: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign33360_e48501: f64 = (0.5 * assign33360_e48500);
            let assign33360_e48502: f64 = (assign33360_e48496 - assign33360_e48501);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33360_e48502, ((-locals.var_qbd_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_qbd_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_qbd_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_qbd_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_qbd_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_qbd_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_qbd_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_qbd_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))), );
            locals.var_qbd_rv = 0.0;
        }
        if ((locals.var_guard1032 != 0.0) && (locals.var_guard1091 != 0.0)) {
            let assign33370_e48510: f64 = (-1.0);
            let assign33370_e48511: f64 = (locals.var_qbd * assign33370_e48510);
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (assign33370_e48511, (locals.var_qbd_dn0 * assign33370_e48510), (locals.var_qbd_dn2 * assign33370_e48510), (locals.var_qbd_dn6 * assign33370_e48510), (locals.var_qbd_dn7 * assign33370_e48510), (locals.var_qbd_dn10 * assign33370_e48510), (locals.var_qbd_dn11 * assign33370_e48510), (locals.var_qbd_dn12 * assign33370_e48510), (locals.var_qbd_dn17 * assign33370_e48510), );
            locals.var_qbd_rv = 0.0;
        }
        let assign33930_e49105: f64 = (locals.var_ids + locals.var_idsibpc);
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (assign33930_e49105, (locals.var_ids_dn0 + locals.var_idsibpc_dn0), (locals.var_ids_dn2 + locals.var_idsibpc_dn2), (locals.var_ids_dn6 + locals.var_idsibpc_dn6), (locals.var_ids_dn7 + locals.var_idsibpc_dn7), (locals.var_ids_dn10 + locals.var_idsibpc_dn10), (locals.var_ids_dn11 + locals.var_idsibpc_dn11), (locals.var_ids_dn12 + locals.var_idsibpc_dn12), (locals.var_ids_dn17 + locals.var_idsibpc_dn17), );
        locals.var_ids_rv = 0.0;
        let assign33940_e49108: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1128 = assign33940_e49108;
        locals.var_guard1128_rv = 0.0;
        if (locals.var_guard1128 != 0.0) {
            let assign33950_e49112: f64 = (locals.var_cbtp + locals.var_cbtn);
            locals.var_cgbe = assign33950_e49112;
            locals.var_cgbe_rv = 0.0;
        }
        if ((locals.var_guard1128 != 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign33960_e49121: f64 = (p.p168 * locals.var_lgleff);
            let assign33960_e49122: f64 = (locals.var_cgbe - assign33960_e49121);
            locals.var_cgbe = assign33960_e49122;
            locals.var_cgbe_rv = 0.0;
        }
        if (locals.var_guard1128 != 0.0) {
            let assign33970_e49127: f64 = (-locals.var_cgbe);
            let assign33970_e49130: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign33970_e49131: f64 = (assign33970_e49127 * assign33970_e49130);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign33970_e49131, (assign33970_e49127 * (-locals.var_vbsp_dn0)), (assign33970_e49127 * (-locals.var_vbsp_dn2)), (assign33970_e49127 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign33970_e49127 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign33970_e49127 * (-locals.var_vbsp_dn10)), (assign33970_e49127 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign33970_e49127 * (-locals.var_vbsp_dn12)), (assign33970_e49127 * (-locals.var_vbsp_dn17)), );
            locals.var_qgob_rv = 0.0;
        }
        if (locals.var_guard1128 != 0.0) {
            locals.var_cfu = 0.0;
            locals.var_cfu_rv = 0.0;
        }
        if (locals.var_guard1128 != 0.0) {
            let assign33990_e49147: f64 = (locals.var_cfu * p.p9);
            let assign33990_e49150: f64 = (locals.var_wgate + locals.var_uc_pdbcp);
            let assign33990_e49151: f64 = (assign33990_e49147 * assign33990_e49150);
            locals.var_cfd = assign33990_e49151;
            locals.var_cfd_rv = 0.0;
        }
        if (locals.var_guard1128 != 0.0) {
            let assign34000_e49157: f64 = (locals.var_cfu * p.p9);
            let assign34000_e49160: f64 = (locals.var_wgate + locals.var_uc_psbcp);
            let assign34000_e49161: f64 = (assign34000_e49157 * assign34000_e49160);
            locals.var_cfs = assign34000_e49161;
            locals.var_cfs_rv = 0.0;
        }
        if (locals.var_guard1128 != 0.0) {
            let assign34010_e49168: f64 = (locals.var_vgs - locals.var_vds);
            let assign34010_e49169: f64 = (locals.var_cfd * assign34010_e49168);
            (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17, ) = (assign34010_e49169, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)), );
            locals.var_qfd_rv = 0.0;
        }
        if (locals.var_guard1128 != 0.0) {
            let assign34020_e49175: f64 = (locals.var_cfs * locals.var_vgs);
            (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11, ) = (assign34020_e49175, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11), );
            locals.var_qfs_rv = 0.0;
        }
        if (locals.var_guard1128 != 0.0) {
            let assign34030_e49181: f64 = (locals.var_cfu * p.p19);
            let assign34030_e49183: f64 = (assign34030_e49181 * p.p9);
            let assign34030_e49186: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign34030_e49187: f64 = (assign34030_e49183 * assign34030_e49186);
            (locals.var_qfbc, locals.var_qfbc_dn0, locals.var_qfbc_dn2, locals.var_qfbc_dn6, locals.var_qfbc_dn7, locals.var_qfbc_dn10, locals.var_qfbc_dn11, locals.var_qfbc_dn12, locals.var_qfbc_dn17, ) = (assign34030_e49187, (assign34030_e49183 * (-locals.var_vbsp_dn0)), (assign34030_e49183 * (-locals.var_vbsp_dn2)), (assign34030_e49183 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34030_e49183 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34030_e49183 * (-locals.var_vbsp_dn10)), (assign34030_e49183 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34030_e49183 * (-locals.var_vbsp_dn12)), (assign34030_e49183 * (-locals.var_vbsp_dn17)), );
            locals.var_qfbc_rv = 0.0;
        }
        if (locals.var_guard1128 != 0.0) {
            let assign34040_e49193: f64 = (locals.var_qgod + locals.var_qfd);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign34040_e49193, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17), );
            locals.var_qgod_rv = 0.0;
        }
        if (locals.var_guard1128 != 0.0) {
            let assign34050_e49199: f64 = (locals.var_qgos + locals.var_qfs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign34050_e49199, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17, );
            locals.var_qgos_rv = 0.0;
        }
        if (locals.var_guard1128 != 0.0) {
            let assign34060_e49205: f64 = (locals.var_qgob + locals.var_qfbc);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign34060_e49205, (locals.var_qgob_dn0 + locals.var_qfbc_dn0), (locals.var_qgob_dn2 + locals.var_qfbc_dn2), (locals.var_qgob_dn6 + locals.var_qfbc_dn6), (locals.var_qgob_dn7 + locals.var_qfbc_dn7), (locals.var_qgob_dn10 + locals.var_qfbc_dn10), (locals.var_qgob_dn11 + locals.var_qfbc_dn11), (locals.var_qgob_dn12 + locals.var_qfbc_dn12), (locals.var_qgob_dn17 + locals.var_qfbc_dn17), );
            locals.var_qgob_rv = 0.0;
        }
        if ((locals.var_guard1128 == 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign34070_e49213: f64 = (-p.p168);
            let assign34070_e49215: f64 = (assign34070_e49213 * locals.var_lgleff);
            locals.var_cgbe = assign34070_e49215;
            locals.var_cgbe_rv = 0.0;
        }
        if ((locals.var_guard1128 == 0.0) && (locals.var_cgbo_given != 0.0)) {
            let assign34080_e49223: f64 = (-locals.var_cgbe);
            let assign34080_e49226: f64 = (locals.var_vgs - locals.var_vbsp);
            let assign34080_e49227: f64 = (assign34080_e49223 * assign34080_e49226);
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (assign34080_e49227, (assign34080_e49223 * (-locals.var_vbsp_dn0)), (assign34080_e49223 * (-locals.var_vbsp_dn2)), (assign34080_e49223 * (locals.var_vgs_dn6 - locals.var_vbsp_dn6)), (assign34080_e49223 * (locals.var_vgs_dn7 - locals.var_vbsp_dn7)), (assign34080_e49223 * (-locals.var_vbsp_dn10)), (assign34080_e49223 * (locals.var_vgs_dn11 - locals.var_vbsp_dn11)), (assign34080_e49223 * (-locals.var_vbsp_dn12)), (assign34080_e49223 * (-locals.var_vbsp_dn17)), );
            locals.var_qgob_rv = 0.0;
        }
        if ((locals.var_guard1128 == 0.0) && (locals.var_cgbo_given == 0.0)) {
            locals.var_cgbe = 0.0;
            locals.var_cgbe_rv = 0.0;
            (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn6, locals.var_qgob_dn7, locals.var_qgob_dn10, locals.var_qgob_dn11, locals.var_qgob_dn12, locals.var_qgob_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qgob_rv = 0.0;
        }
        if (locals.var_guard1128 == 0.0) {
            locals.var_cf = 0.0;
            locals.var_cf_rv = 0.0;
            locals.var_cfd = locals.var_cf;
            locals.var_cfd_rv = 0.0;
            locals.var_cfs = locals.var_cf;
            locals.var_cfs_rv = 0.0;
        }
        if (locals.var_guard1128 == 0.0) {
            let assign34140_e49276: f64 = (locals.var_vgs - locals.var_vds);
            let assign34140_e49277: f64 = (locals.var_cfd * assign34140_e49276);
            (locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn6, locals.var_qfd_dn7, locals.var_qfd_dn10, locals.var_qfd_dn11, locals.var_qfd_dn12, locals.var_qfd_dn17, ) = (assign34140_e49277, (locals.var_cfd * (-locals.var_vds_dn0)), (locals.var_cfd * (-locals.var_vds_dn2)), (locals.var_cfd * (locals.var_vgs_dn6 - locals.var_vds_dn6)), (locals.var_cfd * (locals.var_vgs_dn7 - locals.var_vds_dn7)), (locals.var_cfd * (-locals.var_vds_dn10)), (locals.var_cfd * (locals.var_vgs_dn11 - locals.var_vds_dn11)), (locals.var_cfd * (-locals.var_vds_dn12)), (locals.var_cfd * (-locals.var_vds_dn17)), );
            locals.var_qfd_rv = 0.0;
        }
        if (locals.var_guard1128 == 0.0) {
            let assign34150_e49284: f64 = (locals.var_cfs * locals.var_vgs);
            (locals.var_qfs, locals.var_qfs_dn6, locals.var_qfs_dn7, locals.var_qfs_dn11, ) = (assign34150_e49284, (locals.var_cfs * locals.var_vgs_dn6), (locals.var_cfs * locals.var_vgs_dn7), (locals.var_cfs * locals.var_vgs_dn11), );
            locals.var_qfs_rv = 0.0;
        }
        if (locals.var_guard1128 == 0.0) {
            let assign34160_e49291: f64 = (locals.var_qgod + locals.var_qfd);
            (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17, ) = (assign34160_e49291, (locals.var_qgod_dn0 + locals.var_qfd_dn0), (locals.var_qgod_dn2 + locals.var_qfd_dn2), (locals.var_qgod_dn6 + locals.var_qfd_dn6), (locals.var_qgod_dn7 + locals.var_qfd_dn7), (locals.var_qgod_dn10 + locals.var_qfd_dn10), (locals.var_qgod_dn11 + locals.var_qfd_dn11), (locals.var_qgod_dn12 + locals.var_qfd_dn12), (locals.var_qgod_dn17 + locals.var_qfd_dn17), );
            locals.var_qgod_rv = 0.0;
        }
        if (locals.var_guard1128 == 0.0) {
            let assign34170_e49298: f64 = (locals.var_qgos + locals.var_qfs);
            (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17, ) = (assign34170_e49298, locals.var_qgos_dn0, locals.var_qgos_dn2, (locals.var_qgos_dn6 + locals.var_qfs_dn6), (locals.var_qgos_dn7 + locals.var_qfs_dn7), locals.var_qgos_dn10, (locals.var_qgos_dn11 + locals.var_qfs_dn11), locals.var_qgos_dn12, locals.var_qgos_dn17, );
            locals.var_qgos_rv = 0.0;
        }
        let assign34180_e49303: f64 = (locals.var_mfactor * locals.var_ids);
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, ) = (assign34180_e49303, (locals.var_mfactor * locals.var_ids_dn0), (locals.var_mfactor * locals.var_ids_dn2), (locals.var_mfactor * locals.var_ids_dn6), (locals.var_mfactor * locals.var_ids_dn7), (locals.var_mfactor * locals.var_ids_dn10), (locals.var_mfactor * locals.var_ids_dn11), (locals.var_mfactor * locals.var_ids_dn12), (locals.var_mfactor * locals.var_ids_dn17), );
        locals.var_idse_rv = 0.0;
        if (locals.var_flg_nqs != 0.0) {
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qde_rv = 0.0;
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qge_rv = 0.0;
        }
        let assign34210_e49314: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1129 = assign34210_e49314;
        locals.var_guard1129_rv = 0.0;
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1129 != 0.0)) {
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qse_rv = 0.0;
            (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17, ) = (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, );
            locals.var_xd_rv = 0.0;
        }
        if ((locals.var_flg_nqs != 0.0) && (locals.var_guard1129 == 0.0)) {
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbe_rv = 0.0;
        }
        let assign34300_e49385: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1130 = assign34300_e49385;
        locals.var_guard1130_rv = 0.0;
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 != 0.0)) {
            let assign34310_e49392: f64 = (-locals.var_qb);
            let assign34310_e49394: f64 = (assign34310_e49392 - locals.var_qi);
            let assign34310_e49395: f64 = (locals.var_mfactor * assign34310_e49394);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34310_e49395, (locals.var_mfactor * ((-locals.var_qb_dn0) - locals.var_qi_dn0)), (locals.var_mfactor * ((-locals.var_qb_dn2) - locals.var_qi_dn2)), (locals.var_mfactor * ((-locals.var_qb_dn6) - locals.var_qi_dn6)), (locals.var_mfactor * ((-locals.var_qb_dn7) - locals.var_qi_dn7)), (locals.var_mfactor * ((-locals.var_qb_dn10) - locals.var_qi_dn10)), (locals.var_mfactor * ((-locals.var_qb_dn11) - locals.var_qi_dn11)), (locals.var_mfactor * ((-locals.var_qb_dn12) - locals.var_qi_dn12)), (locals.var_mfactor * (-locals.var_qb_dn13)), (locals.var_mfactor * (-locals.var_qb_dn15)), (locals.var_mfactor * (-locals.var_qb_dn16)), (locals.var_mfactor * ((-locals.var_qb_dn17) - locals.var_qi_dn17)), (locals.var_mfactor * (-locals.var_qb_dn18)), );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 != 0.0)) {
            let assign34320_e49404: f64 = (locals.var_mfactor * locals.var_qd);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34320_e49404, (locals.var_mfactor * locals.var_qd_dn0), (locals.var_mfactor * locals.var_qd_dn2), (locals.var_mfactor * locals.var_qd_dn6), (locals.var_mfactor * locals.var_qd_dn7), (locals.var_mfactor * locals.var_qd_dn10), (locals.var_mfactor * locals.var_qd_dn11), (locals.var_mfactor * locals.var_qd_dn12), (locals.var_mfactor * locals.var_qd_dn13), (locals.var_mfactor * locals.var_qd_dn15), (locals.var_mfactor * locals.var_qd_dn16), (locals.var_mfactor * locals.var_qd_dn17), (locals.var_mfactor * locals.var_qd_dn18), );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 != 0.0)) {
            let assign34330_e49414: f64 = (locals.var_qi - locals.var_qd);
            let assign34330_e49415: f64 = (locals.var_mfactor * assign34330_e49414);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34330_e49415, (locals.var_mfactor * (locals.var_qi_dn0 - locals.var_qd_dn0)), (locals.var_mfactor * (locals.var_qi_dn2 - locals.var_qd_dn2)), (locals.var_mfactor * (locals.var_qi_dn6 - locals.var_qd_dn6)), (locals.var_mfactor * (locals.var_qi_dn7 - locals.var_qd_dn7)), (locals.var_mfactor * (locals.var_qi_dn10 - locals.var_qd_dn10)), (locals.var_mfactor * (locals.var_qi_dn11 - locals.var_qd_dn11)), (locals.var_mfactor * (locals.var_qi_dn12 - locals.var_qd_dn12)), (locals.var_mfactor * (-locals.var_qd_dn13)), (locals.var_mfactor * (-locals.var_qd_dn15)), (locals.var_mfactor * (-locals.var_qd_dn16)), (locals.var_mfactor * (locals.var_qi_dn17 - locals.var_qd_dn17)), (locals.var_mfactor * (-locals.var_qd_dn18)), );
            locals.var_qse_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 == 0.0)) {
            let assign34340_e49425: f64 = (-locals.var_qsub);
            let assign34340_e49427: f64 = (assign34340_e49425 - locals.var_qi);
            let assign34340_e49429: f64 = (assign34340_e49427 - locals.var_qs_fb);
            let assign34340_e49431: f64 = (assign34340_e49429 - locals.var_qd_fb);
            let assign34340_e49432: f64 = (locals.var_mfactor * assign34340_e49431);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34340_e49432, (locals.var_mfactor * ((((-locals.var_qsub_dn0) - locals.var_qi_dn0) - locals.var_qs_fb_dn0) - locals.var_qd_fb_dn0)), (locals.var_mfactor * ((((-locals.var_qsub_dn2) - locals.var_qi_dn2) - locals.var_qs_fb_dn2) - locals.var_qd_fb_dn2)), (locals.var_mfactor * ((((-locals.var_qsub_dn6) - locals.var_qi_dn6) - locals.var_qs_fb_dn6) - locals.var_qd_fb_dn6)), (locals.var_mfactor * ((((-locals.var_qsub_dn7) - locals.var_qi_dn7) - locals.var_qs_fb_dn7) - locals.var_qd_fb_dn7)), (locals.var_mfactor * ((((-locals.var_qsub_dn10) - locals.var_qi_dn10) - locals.var_qs_fb_dn10) - locals.var_qd_fb_dn10)), (locals.var_mfactor * ((((-locals.var_qsub_dn11) - locals.var_qi_dn11) - locals.var_qs_fb_dn11) - locals.var_qd_fb_dn11)), (locals.var_mfactor * ((((-locals.var_qsub_dn12) - locals.var_qi_dn12) - locals.var_qs_fb_dn12) - locals.var_qd_fb_dn12)), (locals.var_mfactor * ((-locals.var_qs_fb_dn13) - locals.var_qd_fb_dn13)), (locals.var_mfactor * ((-locals.var_qs_fb_dn15) - locals.var_qd_fb_dn15)), (locals.var_mfactor * ((-locals.var_qs_fb_dn16) - locals.var_qd_fb_dn16)), (locals.var_mfactor * ((((-locals.var_qsub_dn17) - locals.var_qi_dn17) - locals.var_qs_fb_dn17) - locals.var_qd_fb_dn17)), (locals.var_mfactor * ((-locals.var_qs_fb_dn18) - locals.var_qd_fb_dn18)), );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 == 0.0)) {
            let assign34350_e49443: f64 = (locals.var_qd + locals.var_qd_fb);
            let assign34350_e49444: f64 = (locals.var_mfactor * assign34350_e49443);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34350_e49444, (locals.var_mfactor * (locals.var_qd_dn0 + locals.var_qd_fb_dn0)), (locals.var_mfactor * (locals.var_qd_dn2 + locals.var_qd_fb_dn2)), (locals.var_mfactor * (locals.var_qd_dn6 + locals.var_qd_fb_dn6)), (locals.var_mfactor * (locals.var_qd_dn7 + locals.var_qd_fb_dn7)), (locals.var_mfactor * (locals.var_qd_dn10 + locals.var_qd_fb_dn10)), (locals.var_mfactor * (locals.var_qd_dn11 + locals.var_qd_fb_dn11)), (locals.var_mfactor * (locals.var_qd_dn12 + locals.var_qd_fb_dn12)), (locals.var_mfactor * (locals.var_qd_dn13 + locals.var_qd_fb_dn13)), (locals.var_mfactor * (locals.var_qd_dn15 + locals.var_qd_fb_dn15)), (locals.var_mfactor * (locals.var_qd_dn16 + locals.var_qd_fb_dn16)), (locals.var_mfactor * (locals.var_qd_dn17 + locals.var_qd_fb_dn17)), (locals.var_mfactor * (locals.var_qd_dn18 + locals.var_qd_fb_dn18)), );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_flg_nqs == 0.0) && (locals.var_guard1130 == 0.0)) {
            let assign34360_e49455: f64 = (locals.var_qi - locals.var_qd);
            let assign34360_e49457: f64 = (assign34360_e49455 + locals.var_qs_fb);
            let assign34360_e49458: f64 = (locals.var_mfactor * assign34360_e49457);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34360_e49458, (locals.var_mfactor * ((locals.var_qi_dn0 - locals.var_qd_dn0) + locals.var_qs_fb_dn0)), (locals.var_mfactor * ((locals.var_qi_dn2 - locals.var_qd_dn2) + locals.var_qs_fb_dn2)), (locals.var_mfactor * ((locals.var_qi_dn6 - locals.var_qd_dn6) + locals.var_qs_fb_dn6)), (locals.var_mfactor * ((locals.var_qi_dn7 - locals.var_qd_dn7) + locals.var_qs_fb_dn7)), (locals.var_mfactor * ((locals.var_qi_dn10 - locals.var_qd_dn10) + locals.var_qs_fb_dn10)), (locals.var_mfactor * ((locals.var_qi_dn11 - locals.var_qd_dn11) + locals.var_qs_fb_dn11)), (locals.var_mfactor * ((locals.var_qi_dn12 - locals.var_qd_dn12) + locals.var_qs_fb_dn12)), (locals.var_mfactor * ((-locals.var_qd_dn13) + locals.var_qs_fb_dn13)), (locals.var_mfactor * ((-locals.var_qd_dn15) + locals.var_qs_fb_dn15)), (locals.var_mfactor * ((-locals.var_qd_dn16) + locals.var_qs_fb_dn16)), (locals.var_mfactor * ((locals.var_qi_dn17 - locals.var_qd_dn17) + locals.var_qs_fb_dn17)), (locals.var_mfactor * ((-locals.var_qd_dn18) + locals.var_qs_fb_dn18)), );
            locals.var_qse_rv = 0.0;
        }
        let assign34370_e49463: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1136 = assign34370_e49463;
        locals.var_guard1136_rv = 0.0;
        if (locals.var_guard1136 != 0.0) {
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qy_rv = 0.0;
        }
        if (locals.var_guard1136 == 0.0) {
            let assign34390_e49472: f64 = (locals.var_ec * locals.var_leff);
            let assign34390_e49474: f64 = (assign34390_e49472 + locals.var_ps0);
            (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17, ) = (assign34390_e49474, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn11 * locals.var_leff) + locals.var_ps0_dn11), ((locals.var_ec_dn12 * locals.var_leff) + locals.var_ps0_dn12), ((locals.var_ec_dn17 * locals.var_leff) + locals.var_ps0_dn17), );
            locals.var_pslk_rv = 0.0;
        }
        let assign34400_e49479: f64 = if locals.var_pslk > locals.var_psdl { 1.0 } else { 0.0 };
        locals.var_guard1137 = assign34400_e49479;
        locals.var_guard1137_rv = 0.0;
        if ((locals.var_guard1136 == 0.0) && (locals.var_guard1137 != 0.0)) {
            (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12, locals.var_pslk_dn17, ) = (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17, );
            locals.var_pslk_rv = 0.0;
        }
        if (locals.var_guard1136 == 0.0) {
            let assign34420_e49492: f64 = (locals.var_vds + locals.var_ps0);
            let assign34420_e49493: f64 = (locals.var_aclm * assign34420_e49492);
            let assign34420_e49496: f64 = (1.0 - locals.var_aclm);
            let assign34420_e49498: f64 = (assign34420_e49496 * locals.var_pslk);
            let assign34420_e49499: f64 = (assign34420_e49493 + assign34420_e49498);
            (locals.var_t1__blk1132, locals.var_t1__blk1132_dn0, locals.var_t1__blk1132_dn2, locals.var_t1__blk1132_dn6, locals.var_t1__blk1132_dn7, locals.var_t1__blk1132_dn10, locals.var_t1__blk1132_dn11, locals.var_t1__blk1132_dn12, locals.var_t1__blk1132_dn17, ) = (assign34420_e49499, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign34420_e49496 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign34420_e49496 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign34420_e49496 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + (assign34420_e49496 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign34420_e49496 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign34420_e49496 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + (assign34420_e49496 * locals.var_pslk_dn12)), ((locals.var_aclm * (locals.var_vds_dn17 + locals.var_ps0_dn17)) + (assign34420_e49496 * locals.var_pslk_dn17)), );
            locals.var_t1__blk1132_rv = 0.0;
        }
        if (locals.var_guard1136 == 0.0) {
            let assign34430_e49506: f64 = (2.0 * 1.034943e-10);
            let assign34430_e49508: f64 = (assign34430_e49506 / locals.var_q_nsub);
            let assign34430_e49509: f64 = (assign34430_e49508).sqrt();
            (locals.var_t10__blk1133, locals.var_t10__blk1133_dn0, locals.var_t10__blk1133_dn2, locals.var_t10__blk1133_dn6, locals.var_t10__blk1133_dn7, locals.var_t10__blk1133_dn10, locals.var_t10__blk1133_dn11, locals.var_t10__blk1133_dn12, locals.var_t10__blk1133_dn17, ) = (assign34430_e49509, ((-((assign34430_e49506 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), ((-((assign34430_e49506 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign34430_e49509)), );
            locals.var_t10__blk1133_rv = 0.0;
        }
        if (locals.var_guard1136 == 0.0) {
            let assign34440_e49516: f64 = (locals.var_t10__blk1133 * 1.3);
            (locals.var_t3__blk1134, locals.var_t3__blk1134_dn0, locals.var_t3__blk1134_dn2, locals.var_t3__blk1134_dn6, locals.var_t3__blk1134_dn7, locals.var_t3__blk1134_dn10, locals.var_t3__blk1134_dn11, locals.var_t3__blk1134_dn12, locals.var_t3__blk1134_dn17, ) = (assign34440_e49516, (locals.var_t10__blk1133_dn0 * 1.3), (locals.var_t10__blk1133_dn2 * 1.3), (locals.var_t10__blk1133_dn6 * 1.3), (locals.var_t10__blk1133_dn7 * 1.3), (locals.var_t10__blk1133_dn10 * 1.3), (locals.var_t10__blk1133_dn11 * 1.3), (locals.var_t10__blk1133_dn12 * 1.3), (locals.var_t10__blk1133_dn17 * 1.3), );
            locals.var_t3__blk1134_rv = 0.0;
        }
        if (locals.var_guard1136 == 0.0) {
            let assign34450_e49523: f64 = (1.034943e-10 * locals.var_weffcv_nf);
            let assign34450_e49525: f64 = (assign34450_e49523 * locals.var_t3__blk1134);
            (locals.var_t2__blk1135, locals.var_t2__blk1135_dn0, locals.var_t2__blk1135_dn2, locals.var_t2__blk1135_dn6, locals.var_t2__blk1135_dn7, locals.var_t2__blk1135_dn10, locals.var_t2__blk1135_dn11, locals.var_t2__blk1135_dn12, locals.var_t2__blk1135_dn17, ) = (assign34450_e49525, (assign34450_e49523 * locals.var_t3__blk1134_dn0), (assign34450_e49523 * locals.var_t3__blk1134_dn2), (assign34450_e49523 * locals.var_t3__blk1134_dn6), (assign34450_e49523 * locals.var_t3__blk1134_dn7), (assign34450_e49523 * locals.var_t3__blk1134_dn10), (assign34450_e49523 * locals.var_t3__blk1134_dn11), (assign34450_e49523 * locals.var_t3__blk1134_dn12), (assign34450_e49523 * locals.var_t3__blk1134_dn17), );
            locals.var_t2__blk1135_rv = 0.0;
        }
        if (locals.var_guard1136 == 0.0) {
            let assign34460_e49532: f64 = (locals.var_ps0 + locals.var_vds);
            let assign34460_e49534: f64 = (assign34460_e49532 - locals.var_t1__blk1132);
            let assign34460_e49536: f64 = (assign34460_e49534 / p.p64);
            let assign34460_e49538: f64 = (assign34460_e49536 - locals.var_ec);
            let assign34460_e49540: f64 = (assign34460_e49538 * locals.var_t2__blk1135);
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (assign34460_e49540, ((((((locals.var_ps0_dn0 + locals.var_vds_dn0) - locals.var_t1__blk1132_dn0) / p.p64) - locals.var_ec_dn0) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn0)), ((((((locals.var_ps0_dn2 + locals.var_vds_dn2) - locals.var_t1__blk1132_dn2) / p.p64) - locals.var_ec_dn2) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn2)), ((((((locals.var_ps0_dn6 + locals.var_vds_dn6) - locals.var_t1__blk1132_dn6) / p.p64) - locals.var_ec_dn6) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn6)), ((((((locals.var_ps0_dn7 + locals.var_vds_dn7) - locals.var_t1__blk1132_dn7) / p.p64) - locals.var_ec_dn7) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn7)), ((((((locals.var_ps0_dn10 + locals.var_vds_dn10) - locals.var_t1__blk1132_dn10) / p.p64) - locals.var_ec_dn10) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn10)), ((((((locals.var_ps0_dn11 + locals.var_vds_dn11) - locals.var_t1__blk1132_dn11) / p.p64) - locals.var_ec_dn11) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn11)), ((((((locals.var_ps0_dn12 + locals.var_vds_dn12) - locals.var_t1__blk1132_dn12) / p.p64) - locals.var_ec_dn12) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn12)), ((((((locals.var_ps0_dn17 + locals.var_vds_dn17) - locals.var_t1__blk1132_dn17) / p.p64) - locals.var_ec_dn17) * locals.var_t2__blk1135) + (assign34460_e49538 * locals.var_t2__blk1135_dn17)), );
            locals.var_qy_rv = 0.0;
        }
        let assign34470_e49545: f64 = if p.p65 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1138 = assign34470_e49545;
        locals.var_guard1138_rv = 0.0;
        if (locals.var_guard1138 != 0.0) {
            let assign34480_e49550: f64 = (locals.var_cqyb0 * locals.var_vbsp);
            let assign34480_e49551: f64 = (locals.var_qy + assign34480_e49550);
            (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12, locals.var_qy_dn17, ) = (assign34480_e49551, (locals.var_qy_dn0 + (locals.var_cqyb0 * locals.var_vbsp_dn0)), (locals.var_qy_dn2 + (locals.var_cqyb0 * locals.var_vbsp_dn2)), (locals.var_qy_dn6 + (locals.var_cqyb0 * locals.var_vbsp_dn6)), (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbsp_dn7)), (locals.var_qy_dn10 + (locals.var_cqyb0 * locals.var_vbsp_dn10)), (locals.var_qy_dn11 + (locals.var_cqyb0 * locals.var_vbsp_dn11)), (locals.var_qy_dn12 + (locals.var_cqyb0 * locals.var_vbsp_dn12)), (locals.var_qy_dn17 + (locals.var_cqyb0 * locals.var_vbsp_dn17)), );
            locals.var_qy_rv = 0.0;
        }
        let assign34490_e49556: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1139 = assign34490_e49556;
        locals.var_guard1139_rv = 0.0;
        let assign34500_e49559: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1140 = assign34500_e49559;
        locals.var_guard1140_rv = 0.0;
        if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
            let assign34510_e49564: f64 = (-locals.var_qbody_bt_p_sus);
            let assign34510_e49566: f64 = (assign34510_e49564 - locals.var_qbody_bt_p_sud);
            let assign34510_e49568: f64 = (assign34510_e49566 - locals.var_qbody_bt_n_sus);
            let assign34510_e49570: f64 = (assign34510_e49568 - locals.var_qbody_bt_n_sud);
            (locals.var_q_bt_ge, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, locals.var_q_bt_ge_dn17, ) = (assign34510_e49570, ((((-locals.var_qbody_bt_p_sus_dn0) - locals.var_qbody_bt_p_sud_dn0) - locals.var_qbody_bt_n_sus_dn0) - locals.var_qbody_bt_n_sud_dn0), ((((-locals.var_qbody_bt_p_sus_dn2) - locals.var_qbody_bt_p_sud_dn2) - locals.var_qbody_bt_n_sus_dn2) - locals.var_qbody_bt_n_sud_dn2), ((((-locals.var_qbody_bt_p_sus_dn6) - locals.var_qbody_bt_p_sud_dn6) - locals.var_qbody_bt_n_sus_dn6) - locals.var_qbody_bt_n_sud_dn6), ((((-locals.var_qbody_bt_p_sus_dn7) - locals.var_qbody_bt_p_sud_dn7) - locals.var_qbody_bt_n_sus_dn7) - locals.var_qbody_bt_n_sud_dn7), ((((-locals.var_qbody_bt_p_sus_dn10) - locals.var_qbody_bt_p_sud_dn10) - locals.var_qbody_bt_n_sus_dn10) - locals.var_qbody_bt_n_sud_dn10), ((((-locals.var_qbody_bt_p_sus_dn11) - locals.var_qbody_bt_p_sud_dn11) - locals.var_qbody_bt_n_sus_dn11) - locals.var_qbody_bt_n_sud_dn11), ((((-locals.var_qbody_bt_p_sus_dn12) - locals.var_qbody_bt_p_sud_dn12) - locals.var_qbody_bt_n_sus_dn12) - locals.var_qbody_bt_n_sud_dn12), ((((-locals.var_qbody_bt_p_sus_dn17) - locals.var_qbody_bt_p_sud_dn17) - locals.var_qbody_bt_n_sus_dn17) - locals.var_qbody_bt_n_sud_dn17), );
            locals.var_q_bt_ge_rv = 0.0;
        }
        if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
            let assign34520_e49578: f64 = (locals.var_qbody_bt_p_iud + locals.var_qbody_bt_n_iud);
            (locals.var_q_bt_de, locals.var_q_bt_de_dn0, locals.var_q_bt_de_dn2, locals.var_q_bt_de_dn6, locals.var_q_bt_de_dn7, locals.var_q_bt_de_dn10, locals.var_q_bt_de_dn11, locals.var_q_bt_de_dn12, locals.var_q_bt_de_dn17, ) = (assign34520_e49578, (locals.var_qbody_bt_p_iud_dn0 + locals.var_qbody_bt_n_iud_dn0), (locals.var_qbody_bt_p_iud_dn2 + locals.var_qbody_bt_n_iud_dn2), (locals.var_qbody_bt_p_iud_dn6 + locals.var_qbody_bt_n_iud_dn6), (locals.var_qbody_bt_p_iud_dn7 + locals.var_qbody_bt_n_iud_dn7), (locals.var_qbody_bt_p_iud_dn10 + locals.var_qbody_bt_n_iud_dn10), (locals.var_qbody_bt_p_iud_dn11 + locals.var_qbody_bt_n_iud_dn11), (locals.var_qbody_bt_p_iud_dn12 + locals.var_qbody_bt_n_iud_dn12), (locals.var_qbody_bt_p_iud_dn17 + locals.var_qbody_bt_n_iud_dn17), );
            locals.var_q_bt_de_rv = 0.0;
        }
        if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
            let assign34530_e49586: f64 = (locals.var_qbody_bt_p_ius + locals.var_qbody_bt_n_ius);
            (locals.var_q_bt_se, locals.var_q_bt_se_dn0, locals.var_q_bt_se_dn2, locals.var_q_bt_se_dn6, locals.var_q_bt_se_dn7, locals.var_q_bt_se_dn10, locals.var_q_bt_se_dn11, locals.var_q_bt_se_dn12, locals.var_q_bt_se_dn17, ) = (assign34530_e49586, (locals.var_qbody_bt_p_ius_dn0 + locals.var_qbody_bt_n_ius_dn0), (locals.var_qbody_bt_p_ius_dn2 + locals.var_qbody_bt_n_ius_dn2), (locals.var_qbody_bt_p_ius_dn6 + locals.var_qbody_bt_n_ius_dn6), (locals.var_qbody_bt_p_ius_dn7 + locals.var_qbody_bt_n_ius_dn7), (locals.var_qbody_bt_p_ius_dn10 + locals.var_qbody_bt_n_ius_dn10), (locals.var_qbody_bt_p_ius_dn11 + locals.var_qbody_bt_n_ius_dn11), (locals.var_qbody_bt_p_ius_dn12 + locals.var_qbody_bt_n_ius_dn12), (locals.var_qbody_bt_p_ius_dn17 + locals.var_qbody_bt_n_ius_dn17), );
            locals.var_q_bt_se_rv = 0.0;
        }
        if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
            let assign34540_e49596: f64 = (locals.var_qgod + locals.var_qgos);
            let assign34540_e49598: f64 = (assign34540_e49596 + locals.var_qgob);
            let assign34540_e49600: f64 = (assign34540_e49598 - locals.var_qy);
            let assign34540_e49602: f64 = (assign34540_e49600 - locals.var_qovs);
            let assign34540_e49604: f64 = (assign34540_e49602 - locals.var_qovd);
            let assign34540_e49606: f64 = (assign34540_e49604 + locals.var_q_bt_ge);
            let assign34540_e49607: f64 = (locals.var_mfactor * assign34540_e49606);
            let assign34540_e49608: f64 = (locals.var_qge + assign34540_e49607);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34540_e49608, (locals.var_qge_dn0 + (locals.var_mfactor * ((((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0) + locals.var_q_bt_ge_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * ((((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2) + locals.var_q_bt_ge_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * ((((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6) + locals.var_q_bt_ge_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * ((((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7) + locals.var_q_bt_ge_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * ((((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10) + locals.var_q_bt_ge_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11) + locals.var_q_bt_ge_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * ((((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12) + locals.var_q_bt_ge_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * ((((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17) + locals.var_q_bt_ge_dn17))), locals.var_qge_dn18, );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
            let assign34550_e49617: f64 = (-locals.var_qgod);
            let assign34550_e49619: f64 = (assign34550_e49617 + locals.var_qy);
            let assign34550_e49621: f64 = (assign34550_e49619 + locals.var_qbdld);
            let assign34550_e49623: f64 = (assign34550_e49621 + locals.var_q_bt_de);
            let assign34550_e49624: f64 = (locals.var_mfactor * assign34550_e49623);
            let assign34550_e49625: f64 = (locals.var_qde + assign34550_e49624);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34550_e49625, (locals.var_qde_dn0 + (locals.var_mfactor * ((((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0) + locals.var_q_bt_de_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * ((((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2) + locals.var_q_bt_de_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * ((((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6) + locals.var_q_bt_de_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * ((((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7) + locals.var_q_bt_de_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * ((((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10) + locals.var_q_bt_de_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * ((((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11) + locals.var_q_bt_de_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * ((((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12) + locals.var_q_bt_de_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * ((((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17) + locals.var_q_bt_de_dn17))), locals.var_qde_dn18, );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 != 0.0)) {
            let assign34560_e49634: f64 = (-locals.var_qgos);
            let assign34560_e49636: f64 = (assign34560_e49634 + locals.var_qbsld);
            let assign34560_e49638: f64 = (assign34560_e49636 + locals.var_q_bt_se);
            let assign34560_e49639: f64 = (locals.var_mfactor * assign34560_e49638);
            let assign34560_e49640: f64 = (locals.var_qse + assign34560_e49639);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34560_e49640, (locals.var_qse_dn0 + (locals.var_mfactor * (((-locals.var_qgos_dn0) + locals.var_qbsld_dn0) + locals.var_q_bt_se_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * (((-locals.var_qgos_dn2) + locals.var_qbsld_dn2) + locals.var_q_bt_se_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * (((-locals.var_qgos_dn6) + locals.var_qbsld_dn6) + locals.var_q_bt_se_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * (((-locals.var_qgos_dn7) + locals.var_qbsld_dn7) + locals.var_q_bt_se_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * (((-locals.var_qgos_dn10) + locals.var_qbsld_dn10) + locals.var_q_bt_se_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * (((-locals.var_qgos_dn11) + locals.var_qbsld_dn11) + locals.var_q_bt_se_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * (((-locals.var_qgos_dn12) + locals.var_qbsld_dn12) + locals.var_q_bt_se_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * (((-locals.var_qgos_dn17) + locals.var_qbsld_dn17) + locals.var_q_bt_se_dn17))), locals.var_qse_dn18, );
            locals.var_qse_rv = 0.0;
        }
        if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 == 0.0)) {
            let assign34570_e49651: f64 = (locals.var_qgod + locals.var_qgos);
            let assign34570_e49653: f64 = (assign34570_e49651 + locals.var_qgob);
            let assign34570_e49655: f64 = (assign34570_e49653 - locals.var_qy);
            let assign34570_e49657: f64 = (assign34570_e49655 - locals.var_qovs);
            let assign34570_e49659: f64 = (assign34570_e49657 - locals.var_qovd);
            let assign34570_e49660: f64 = (locals.var_mfactor * assign34570_e49659);
            let assign34570_e49661: f64 = (locals.var_qge + assign34570_e49660);
            (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12, locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, locals.var_qge_dn17, locals.var_qge_dn18, ) = (assign34570_e49661, (locals.var_qge_dn0 + (locals.var_mfactor * (((((locals.var_qgod_dn0 + locals.var_qgos_dn0) + locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * (((((locals.var_qgod_dn2 + locals.var_qgos_dn2) + locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2))), (locals.var_qge_dn6 + (locals.var_mfactor * (((((locals.var_qgod_dn6 + locals.var_qgos_dn6) + locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6))), (locals.var_qge_dn7 + (locals.var_mfactor * (((((locals.var_qgod_dn7 + locals.var_qgos_dn7) + locals.var_qgob_dn7) - locals.var_qy_dn7) - locals.var_qovs_dn7) - locals.var_qovd_dn7))), (locals.var_qge_dn10 + (locals.var_mfactor * (((((locals.var_qgod_dn10 + locals.var_qgos_dn10) + locals.var_qgob_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * (((((locals.var_qgod_dn11 + locals.var_qgos_dn11) + locals.var_qgob_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * (((((locals.var_qgod_dn12 + locals.var_qgos_dn12) + locals.var_qgob_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12))), locals.var_qge_dn13, locals.var_qge_dn15, locals.var_qge_dn16, (locals.var_qge_dn17 + (locals.var_mfactor * (((((locals.var_qgod_dn17 + locals.var_qgos_dn17) + locals.var_qgob_dn17) - locals.var_qy_dn17) - locals.var_qovs_dn17) - locals.var_qovd_dn17))), locals.var_qge_dn18, );
            locals.var_qge_rv = 0.0;
        }
        if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 == 0.0)) {
            let assign34580_e49671: f64 = (-locals.var_qgod);
            let assign34580_e49673: f64 = (assign34580_e49671 + locals.var_qy);
            let assign34580_e49675: f64 = (assign34580_e49673 + locals.var_qbdld);
            let assign34580_e49676: f64 = (locals.var_mfactor * assign34580_e49675);
            let assign34580_e49677: f64 = (locals.var_qde + assign34580_e49676);
            (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12, locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, locals.var_qde_dn17, locals.var_qde_dn18, ) = (assign34580_e49677, (locals.var_qde_dn0 + (locals.var_mfactor * (((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * (((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2))), (locals.var_qde_dn6 + (locals.var_mfactor * (((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6))), (locals.var_qde_dn7 + (locals.var_mfactor * (((-locals.var_qgod_dn7) + locals.var_qy_dn7) + locals.var_qbdld_dn7))), (locals.var_qde_dn10 + (locals.var_mfactor * (((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * (((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * (((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12))), locals.var_qde_dn13, locals.var_qde_dn15, locals.var_qde_dn16, (locals.var_qde_dn17 + (locals.var_mfactor * (((-locals.var_qgod_dn17) + locals.var_qy_dn17) + locals.var_qbdld_dn17))), locals.var_qde_dn18, );
            locals.var_qde_rv = 0.0;
        }
        if ((locals.var_guard1139 != 0.0) && (locals.var_guard1140 == 0.0)) {
            let assign34590_e49687: f64 = (-locals.var_qgos);
            let assign34590_e49689: f64 = (assign34590_e49687 + locals.var_qbsld);
            let assign34590_e49690: f64 = (locals.var_mfactor * assign34590_e49689);
            let assign34590_e49691: f64 = (locals.var_qse + assign34590_e49690);
            (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12, locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, locals.var_qse_dn17, locals.var_qse_dn18, ) = (assign34590_e49691, (locals.var_qse_dn0 + (locals.var_mfactor * ((-locals.var_qgos_dn0) + locals.var_qbsld_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * ((-locals.var_qgos_dn2) + locals.var_qbsld_dn2))), (locals.var_qse_dn6 + (locals.var_mfactor * ((-locals.var_qgos_dn6) + locals.var_qbsld_dn6))), (locals.var_qse_dn7 + (locals.var_mfactor * ((-locals.var_qgos_dn7) + locals.var_qbsld_dn7))), (locals.var_qse_dn10 + (locals.var_mfactor * ((-locals.var_qgos_dn10) + locals.var_qbsld_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * ((-locals.var_qgos_dn11) + locals.var_qbsld_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * ((-locals.var_qgos_dn12) + locals.var_qbsld_dn12))), locals.var_qse_dn13, locals.var_qse_dn15, locals.var_qse_dn16, (locals.var_qse_dn17 + (locals.var_mfactor * ((-locals.var_qgos_dn17) + locals.var_qbsld_dn17))), locals.var_qse_dn18, );
            locals.var_qse_rv = 0.0;
        }
        let assign34620_e49698: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1141 = assign34620_e49698;
        locals.var_guard1141_rv = 0.0;
        if (locals.var_guard1141 != 0.0) {
            let assign34630_e49702: f64 = (locals.var_mfactor * locals.var_ibs);
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (assign34630_e49702, (locals.var_mfactor * locals.var_ibs_dn0), (locals.var_mfactor * locals.var_ibs_dn2), (locals.var_mfactor * locals.var_ibs_dn6), (locals.var_mfactor * locals.var_ibs_dn7), (locals.var_mfactor * locals.var_ibs_dn10), (locals.var_mfactor * locals.var_ibs_dn11), (locals.var_mfactor * locals.var_ibs_dn12), (locals.var_mfactor * locals.var_ibs_dn17), );
            locals.var_ibsb_rv = 0.0;
        }
        if (locals.var_guard1141 != 0.0) {
            let assign34640_e49708: f64 = (locals.var_mfactor * locals.var_ibd);
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (assign34640_e49708, (locals.var_mfactor * locals.var_ibd_dn0), (locals.var_mfactor * locals.var_ibd_dn2), (locals.var_mfactor * locals.var_ibd_dn6), (locals.var_mfactor * locals.var_ibd_dn7), (locals.var_mfactor * locals.var_ibd_dn10), (locals.var_mfactor * locals.var_ibd_dn11), (locals.var_mfactor * locals.var_ibd_dn12), (locals.var_mfactor * locals.var_ibd_dn17), );
            locals.var_ibdb_rv = 0.0;
        }
        if (locals.var_guard1141 != 0.0) {
            let assign34650_e49714: f64 = (locals.var_mfactor * locals.var_qbd);
            (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, ) = (assign34650_e49714, (locals.var_mfactor * locals.var_qbd_dn0), (locals.var_mfactor * locals.var_qbd_dn2), (locals.var_mfactor * locals.var_qbd_dn6), (locals.var_mfactor * locals.var_qbd_dn7), (locals.var_mfactor * locals.var_qbd_dn10), (locals.var_mfactor * locals.var_qbd_dn11), (locals.var_mfactor * locals.var_qbd_dn12), (locals.var_mfactor * locals.var_qbd_dn17), );
            locals.var_qbd_s0_rv = 0.0;
        }
        if (locals.var_guard1141 != 0.0) {
            let assign34660_e49720: f64 = (locals.var_mfactor * locals.var_qbs);
            (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, ) = (assign34660_e49720, (locals.var_mfactor * locals.var_qbs_dn0), (locals.var_mfactor * locals.var_qbs_dn2), (locals.var_mfactor * locals.var_qbs_dn6), (locals.var_mfactor * locals.var_qbs_dn7), (locals.var_mfactor * locals.var_qbs_dn10), (locals.var_mfactor * locals.var_qbs_dn11), (locals.var_mfactor * locals.var_qbs_dn12), (locals.var_mfactor * locals.var_qbs_dn17), );
            locals.var_qbs_s0_rv = 0.0;
        }
        if (locals.var_guard1141 == 0.0) {
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ibsb_rv = 0.0;
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ibdb_rv = 0.0;
            (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbd_s0_rv = 0.0;
            (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qbs_s0_rv = 0.0;
        }
        let assign34710_e49745: f64 = if p.p25 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1142 = assign34710_e49745;
        locals.var_guard1142_rv = 0.0;
        if (locals.var_guard1142 != 0.0) {
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
        if (locals.var_guard1142 == 0.0) {
            let assign34730_e49754: f64 = (locals.var_mfactor * locals.var_isub);
            (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17, ) = (assign34730_e49754, (locals.var_mfactor * locals.var_isub_dn0), (locals.var_mfactor * locals.var_isub_dn2), (locals.var_mfactor * locals.var_isub_dn6), (locals.var_mfactor * locals.var_isub_dn7), (locals.var_mfactor * locals.var_isub_dn10), (locals.var_mfactor * locals.var_isub_dn11), (locals.var_mfactor * locals.var_isub_dn12), (locals.var_mfactor * locals.var_isub_dn17), );
            locals.var_isube_rv = 0.0;
        }
        let assign35090_e50017: f64 = if p.p259 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1153 = assign35090_e50017;
        locals.var_guard1153_rv = 0.0;
        if (locals.var_guard1153 != 0.0) {
            locals.var_rdmod = 1.0;
            locals.var_rdmod_rv = 0.0;
        }
        let assign35110_e50024: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign35110_e50024;
        locals.var_guard1173_rv = 0.0;
        if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 != 0.0)) {
            locals.var_mks_rdrmue = p.p266;
            locals.var_mks_rdrmue_rv = 0.0;
            locals.var_mks_rdrvmax = p.p268;
            locals.var_mks_rdrvmax_rv = 0.0;
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (p.p273, 0.0, );
            locals.var_rrdrbb_rv = 0.0;
            locals.var_ldrifte = p.p258;
            locals.var_ldrifte_rv = 0.0;
        }
        if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 != 0.0)) {
            let assign35180_e50075: f64 = (p.p50 * (nv7 - nv2));
            (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7, ) = (assign35180_e50075, 0.0, (-p.p50), 0.0, p.p50, );
            locals.var_vrdr_rv = 0.0;
        }
        if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 == 0.0)) {
            locals.var_mks_rdrmue = p.p265;
            locals.var_mks_rdrmue_rv = 0.0;
            locals.var_mks_rdrvmax = p.p267;
            locals.var_mks_rdrvmax_rv = 0.0;
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (p.p272, 0.0, );
            locals.var_rrdrbb_rv = 0.0;
            locals.var_ldrifte = p.p257;
            locals.var_ldrifte_rv = 0.0;
        }
        if ((locals.var_guard1153 != 0.0) && (locals.var_guard1173 == 0.0)) {
            let assign35250_e50135: f64 = (p.p50 * (nv0 - nv6));
            (locals.var_vrdr, locals.var_vrdr_dn0, locals.var_vrdr_dn2, locals.var_vrdr_dn6, locals.var_vrdr_dn7, ) = (assign35250_e50135, p.p50, 0.0, (-p.p50), 0.0, );
            locals.var_vrdr_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35280_e50158: f64 = (locals.var_mks_rdrmue / 10000.0);
            locals.var_mks_rdrmue = assign35280_e50158;
            locals.var_mks_rdrmue_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35290_e50164: f64 = (locals.var_mks_rdrvmax / 100.0);
            locals.var_mks_rdrvmax = assign35290_e50164;
            locals.var_mks_rdrvmax_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35300_e50170: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_tratio, locals.var_tratio_dn10, ) = (assign35300_e50170, (locals.var_ttemp_dn10 / locals.var_uc_tnom), );
            locals.var_tratio_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35310_e50176: f64 = (locals.var_tratio).powf(p.p269);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35310_e50176, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio).powf(p.p269 - 1.0) * locals.var_tratio_dn10)) } } else { (assign35310_e50176 * (p.p269 * (locals.var_tratio_dn10 / locals.var_tratio))) }, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35320_e50182: f64 = (locals.var_mks_rdrmue / locals.var_t1);
            (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17, ) = (assign35320_e50182, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))), );
            locals.var_mu0_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35330_e50189: f64 = (0.4 * locals.var_tratio);
            let assign35330_e50190: f64 = (1.8 + assign35330_e50189);
            let assign35330_e50193: f64 = (0.1 * locals.var_tratio);
            let assign35330_e50195: f64 = (assign35330_e50193 * locals.var_tratio);
            let assign35330_e50196: f64 = (assign35330_e50190 + assign35330_e50195);
            let assign35330_e50200: f64 = (1.0 - locals.var_tratio);
            let assign35330_e50201: f64 = (p.p270 * assign35330_e50200);
            let assign35330_e50202: f64 = (assign35330_e50196 - assign35330_e50201);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17, ) = (assign35330_e50202, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign35330_e50193 * locals.var_tratio_dn10))) - (p.p270 * (-locals.var_tratio_dn10))), 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35340_e50208: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
            (locals.var_vmaxe__blk1166, locals.var_vmaxe__blk1166_dn0, locals.var_vmaxe__blk1166_dn2, locals.var_vmaxe__blk1166_dn6, locals.var_vmaxe__blk1166_dn7, locals.var_vmaxe__blk1166_dn10, locals.var_vmaxe__blk1166_dn11, locals.var_vmaxe__blk1166_dn12, locals.var_vmaxe__blk1166_dn17, ) = (assign35340_e50208, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))), );
            locals.var_vmaxe__blk1166_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35350_e50216: f64 = (locals.var_ttemp - locals.var_uc_tnom);
            let assign35350_e50217: f64 = (p.p274 * assign35350_e50216);
            let assign35350_e50218: f64 = (locals.var_rrdrbb + assign35350_e50217);
            (locals.var_rrdrbb, locals.var_rrdrbb_dn10, ) = (assign35350_e50218, (locals.var_rrdrbb_dn10 + (p.p274 * locals.var_ttemp_dn10)), );
            locals.var_rrdrbb_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35360_e50226: f64 = (locals.var_lgle).powf(p.p280);
            let assign35360_e50227: f64 = (p.p279 / assign35360_e50226);
            let assign35360_e50228: f64 = (1.0 + assign35360_e50227);
            locals.var_rdrmuele = assign35360_e50228;
            locals.var_rdrmuele_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35370_e50236: f64 = (locals.var_lgle).powf(p.p278);
            let assign35370_e50237: f64 = (p.p277 / assign35370_e50236);
            let assign35370_e50238: f64 = (1.0 + assign35370_e50237);
            locals.var_rdrvmaxle = assign35370_e50238;
            locals.var_rdrvmaxle_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35380_e50246: f64 = (locals.var_wg).powf(p.p276);
            let assign35380_e50247: f64 = (p.p275 / assign35380_e50246);
            let assign35380_e50248: f64 = (1.0 + assign35380_e50247);
            locals.var_rdrvmaxwe = assign35380_e50248;
            locals.var_rdrvmaxwe_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35390_e50254: f64 = (locals.var_mu0 * locals.var_rdrmuele);
            (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn6, locals.var_mu0_dn7, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12, locals.var_mu0_dn17, ) = (assign35390_e50254, (locals.var_mu0_dn0 * locals.var_rdrmuele), (locals.var_mu0_dn2 * locals.var_rdrmuele), (locals.var_mu0_dn6 * locals.var_rdrmuele), (locals.var_mu0_dn7 * locals.var_rdrmuele), (locals.var_mu0_dn10 * locals.var_rdrmuele), (locals.var_mu0_dn11 * locals.var_rdrmuele), (locals.var_mu0_dn12 * locals.var_rdrmuele), (locals.var_mu0_dn17 * locals.var_rdrmuele), );
            locals.var_mu0_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35400_e50260: f64 = (locals.var_vmaxe__blk1166 * locals.var_rdrvmaxwe);
            let assign35400_e50262: f64 = (assign35400_e50260 * locals.var_rdrvmaxle);
            let assign35400_e50264: f64 = (assign35400_e50262 + 1e-50);
            (locals.var_vmaxe__blk1166, locals.var_vmaxe__blk1166_dn0, locals.var_vmaxe__blk1166_dn2, locals.var_vmaxe__blk1166_dn6, locals.var_vmaxe__blk1166_dn7, locals.var_vmaxe__blk1166_dn10, locals.var_vmaxe__blk1166_dn11, locals.var_vmaxe__blk1166_dn12, locals.var_vmaxe__blk1166_dn17, ) = (assign35400_e50264, ((locals.var_vmaxe__blk1166_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn7 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn12 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk1166_dn17 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), );
            locals.var_vmaxe__blk1166_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35410_e50270: f64 = (locals.var_vrdr / locals.var_ldrifte);
            (locals.var_edri, locals.var_edri_dn0, locals.var_edri_dn2, locals.var_edri_dn6, locals.var_edri_dn7, ) = (assign35410_e50270, (locals.var_vrdr_dn0 / locals.var_ldrifte), (locals.var_vrdr_dn2 / locals.var_ldrifte), (locals.var_vrdr_dn6 / locals.var_ldrifte), (locals.var_vrdr_dn7 / locals.var_ldrifte), );
            locals.var_edri_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35420_e50276: f64 = (locals.var_mu0 * locals.var_edri);
            (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn6, locals.var_vdri_dn7, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn12, locals.var_vdri_dn17, ) = (assign35420_e50276, ((locals.var_mu0_dn0 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn0)), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), ((locals.var_mu0_dn6 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn6)), ((locals.var_mu0_dn7 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn7)), (locals.var_mu0_dn10 * locals.var_edri), (locals.var_mu0_dn11 * locals.var_edri), (locals.var_mu0_dn12 * locals.var_edri), (locals.var_mu0_dn17 * locals.var_edri), );
            locals.var_vdri_rv = 0.0;
        }
        let assign35430_e50281: f64 = if locals.var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign35430_e50281;
        locals.var_guard1174_rv = 0.0;
        if ((locals.var_guard1153 != 0.0) && (locals.var_guard1174 != 0.0)) {
            let assign35440_e50287: f64 = (locals.var_vdri / locals.var_vmaxe__blk1166);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35440_e50287, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn0)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn2)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn6)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn7 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn7)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn10)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn11)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn12 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn12)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), (((locals.var_vdri_dn17 * locals.var_vmaxe__blk1166) - (locals.var_vdri * locals.var_vmaxe__blk1166_dn17)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard1153 != 0.0) && (locals.var_guard1174 == 0.0)) {
            let assign35450_e50295: f64 = (-locals.var_vdri);
            let assign35450_e50297: f64 = (assign35450_e50295 / locals.var_vmaxe__blk1166);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35450_e50297, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn0)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn2)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn6)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn7) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn7)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn10)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn11)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn12) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn12)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), ((((-locals.var_vdri_dn17) * locals.var_vmaxe__blk1166) - (assign35450_e50295 * locals.var_vmaxe__blk1166_dn17)) / (locals.var_vmaxe__blk1166 * locals.var_vmaxe__blk1166)), );
            locals.var_t1_rv = 0.0;
        }
        let assign35460_e50303: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50304: f64 = (1.0 - assign35460_e50303);
        let assign35460_e50311: f64 = (10.0 * 2.220446049250313e-16);
        let assign35460_e50312: f64 = (1.0 + assign35460_e50311);
        let assign35460_e50314: f64 = if ((assign35460_e50304 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35460_e50312)) { 1.0 } else { 0.0 };
        locals.var_guard1175 = assign35460_e50314;
        locals.var_guard1175_rv = 0.0;
        if ((locals.var_guard1153 != 0.0) && (locals.var_guard1175 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t3_rv = 0.0;
        }
        let assign35480_e50324: f64 = (10.0 * 2.220446049250313e-16);
        let assign35480_e50325: f64 = (2.0 - assign35480_e50324);
        let assign35480_e50332: f64 = (10.0 * 2.220446049250313e-16);
        let assign35480_e50333: f64 = (2.0 + assign35480_e50332);
        let assign35480_e50335: f64 = if ((assign35480_e50325 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35480_e50333)) { 1.0 } else { 0.0 };
        locals.var_guard1176 = assign35480_e50335;
        locals.var_guard1176_rv = 0.0;
        if (((locals.var_guard1153 != 0.0) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, );
            locals.var_t3_rv = 0.0;
        }
        if (((locals.var_guard1153 != 0.0) && (locals.var_guard1175 == 0.0)) && (locals.var_guard1176 == 0.0)) {
            let assign35500_e50355: f64 = (locals.var_rrdrbb - 1.0);
            let assign35500_e50356: f64 = (locals.var_t1).powf(assign35500_e50355);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (assign35500_e50356, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn0)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn2)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn6)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn7)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb_dn10 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn10)) } } else { (assign35500_e50356 * ((locals.var_rrdrbb_dn10 * (locals.var_t1).ln()) + (assign35500_e50355 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn11)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn12)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign35500_e50355) as f64).is_finite() && ((assign35500_e50355) as f64).fract() == 0.0 { if assign35500_e50355 == 0.0 { 0.0 } else { (assign35500_e50355 * ((locals.var_t1).powf(assign35500_e50355 - 1.0) * locals.var_t1_dn17)) } } else { (assign35500_e50356 * (assign35500_e50355 * (locals.var_t1_dn17 / locals.var_t1))) }, );
            locals.var_t3_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35510_e50362: f64 = (locals.var_t1 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, ) = (assign35510_e50362, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35520_e50368: f64 = (1.0 + locals.var_t2);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17, ) = (assign35520_e50368, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, );
            locals.var_t4_rv = 0.0;
        }
        let assign35530_e50374: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50375: f64 = (1.0 - assign35530_e50374);
        let assign35530_e50382: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50383: f64 = (1.0 + assign35530_e50382);
        let assign35530_e50385: f64 = if ((assign35530_e50375 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35530_e50383)) { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign35530_e50385;
        locals.var_guard1177_rv = 0.0;
        if ((locals.var_guard1153 != 0.0) && (locals.var_guard1177 != 0.0)) {
            let assign35540_e50391: f64 = (1.0 / locals.var_t4);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35540_e50391, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))), );
            locals.var_t5_rv = 0.0;
        }
        let assign35550_e50397: f64 = (10.0 * 2.220446049250313e-16);
        let assign35550_e50398: f64 = (2.0 - assign35550_e50397);
        let assign35550_e50405: f64 = (10.0 * 2.220446049250313e-16);
        let assign35550_e50406: f64 = (2.0 + assign35550_e50405);
        let assign35550_e50408: f64 = if ((assign35550_e50398 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign35550_e50406)) { 1.0 } else { 0.0 };
        locals.var_guard1178 = assign35550_e50408;
        locals.var_guard1178_rv = 0.0;
        if (((locals.var_guard1153 != 0.0) && (locals.var_guard1177 == 0.0)) && (locals.var_guard1178 != 0.0)) {
            let assign35560_e50417: f64 = (locals.var_t4).sqrt();
            let assign35560_e50418: f64 = (1.0 / assign35560_e50417);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35560_e50418, (-((locals.var_t4_dn0 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn2 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn6 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn7 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn10 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn11 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn12 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((locals.var_t4_dn17 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), );
            locals.var_t5_rv = 0.0;
        }
        if (((locals.var_guard1153 != 0.0) && (locals.var_guard1177 == 0.0)) && (locals.var_guard1178 == 0.0)) {
            let assign35570_e50430: f64 = (-1.0);
            let assign35570_e50432: f64 = (assign35570_e50430 / locals.var_rrdrbb);
            let assign35570_e50434: f64 = (assign35570_e50432 - 1.0);
            let assign35570_e50435: f64 = (locals.var_t4).powf(assign35570_e50434);
            (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17, ) = (assign35570_e50435, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn0)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn2)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn6)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn7)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign35570_e50430 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn10)) } } else { (assign35570_e50435 * (((-((assign35570_e50430 * locals.var_rrdrbb_dn10) / (locals.var_rrdrbb * locals.var_rrdrbb))) * (locals.var_t4).ln()) + (assign35570_e50434 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn11)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn12)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((locals.var_t4).powf(assign35570_e50434 - 1.0) * locals.var_t4_dn17)) } } else { (assign35570_e50435 * (assign35570_e50434 * (locals.var_t4_dn17 / locals.var_t4))) }, );
            locals.var_t6_rv = 0.0;
        }
        if (((locals.var_guard1153 != 0.0) && (locals.var_guard1177 == 0.0)) && (locals.var_guard1178 == 0.0)) {
            let assign35580_e50447: f64 = (locals.var_t4 * locals.var_t6);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign35580_e50447, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard1153 != 0.0) {
            let assign35600_e50459: f64 = (1.6021918e-19 / locals.var_ldrifte);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35600_e50459, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        let assign35720_e50535: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1181 = assign35720_e50535;
        locals.var_guard1181_rv = 0.0;
        if (locals.var_guard1181 != 0.0) {
            locals.var_rdmod = 2.0;
            locals.var_rdmod_rv = 0.0;
        }
        let assign35740_e50542: f64 = if locals.var_rdmod == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign35740_e50542;
        locals.var_guard1201_rv = 0.0;
        if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 != 0.0)) {
            locals.var_mks_rdrmue__blk1185 = p.p266;
            locals.var_mks_rdrmue__blk1185_rv = 0.0;
            locals.var_mks_rdrvmax__blk1186 = p.p268;
            locals.var_mks_rdrvmax__blk1186_rv = 0.0;
            (locals.var_rrdrbb__blk1187, locals.var_rrdrbb__blk1187_dn10, ) = (p.p273, 0.0, );
            locals.var_rrdrbb__blk1187_rv = 0.0;
            locals.var_ldrifte__blk1191 = p.p258;
            locals.var_ldrifte__blk1191_rv = 0.0;
        }
        if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 != 0.0)) {
            let assign35810_e50593: f64 = (p.p50 * (nv7 - nv2));
            (locals.var_vrdr__blk1189, locals.var_vrdr__blk1189_dn0, locals.var_vrdr__blk1189_dn2, locals.var_vrdr__blk1189_dn6, locals.var_vrdr__blk1189_dn7, ) = (assign35810_e50593, 0.0, (-p.p50), 0.0, p.p50, );
            locals.var_vrdr__blk1189_rv = 0.0;
        }
        if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 == 0.0)) {
            locals.var_mks_rdrmue__blk1185 = p.p265;
            locals.var_mks_rdrmue__blk1185_rv = 0.0;
            locals.var_mks_rdrvmax__blk1186 = p.p267;
            locals.var_mks_rdrvmax__blk1186_rv = 0.0;
            (locals.var_rrdrbb__blk1187, locals.var_rrdrbb__blk1187_dn10, ) = (p.p272, 0.0, );
            locals.var_rrdrbb__blk1187_rv = 0.0;
            locals.var_ldrifte__blk1191 = p.p257;
            locals.var_ldrifte__blk1191_rv = 0.0;
        }
        if ((locals.var_guard1181 != 0.0) && (locals.var_guard1201 == 0.0)) {
            let assign35880_e50653: f64 = (p.p50 * (nv0 - nv6));
            (locals.var_vrdr__blk1189, locals.var_vrdr__blk1189_dn0, locals.var_vrdr__blk1189_dn2, locals.var_vrdr__blk1189_dn6, locals.var_vrdr__blk1189_dn7, ) = (assign35880_e50653, p.p50, 0.0, (-p.p50), 0.0, );
            locals.var_vrdr__blk1189_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign35910_e50676: f64 = (locals.var_mks_rdrmue__blk1185 / 10000.0);
            locals.var_mks_rdrmue__blk1185 = assign35910_e50676;
            locals.var_mks_rdrmue__blk1185_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign35920_e50682: f64 = (locals.var_mks_rdrvmax__blk1186 / 100.0);
            locals.var_mks_rdrvmax__blk1186 = assign35920_e50682;
            locals.var_mks_rdrvmax__blk1186_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign35930_e50688: f64 = (locals.var_ttemp / locals.var_uc_tnom);
            (locals.var_tratio__blk1190, locals.var_tratio__blk1190_dn10, ) = (assign35930_e50688, (locals.var_ttemp_dn10 / locals.var_uc_tnom), );
            locals.var_tratio__blk1190_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign35940_e50694: f64 = (locals.var_tratio__blk1190).powf(p.p269);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign35940_e50694, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((locals.var_tratio__blk1190).powf(p.p269 - 1.0) * locals.var_tratio__blk1190_dn10)) } } else { (assign35940_e50694 * (p.p269 * (locals.var_tratio__blk1190_dn10 / locals.var_tratio__blk1190))) }, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign35950_e50700: f64 = (locals.var_mks_rdrmue__blk1185 / locals.var_t1);
            (locals.var_mu0__blk1193, locals.var_mu0__blk1193_dn0, locals.var_mu0__blk1193_dn2, locals.var_mu0__blk1193_dn6, locals.var_mu0__blk1193_dn7, locals.var_mu0__blk1193_dn10, locals.var_mu0__blk1193_dn11, locals.var_mu0__blk1193_dn12, locals.var_mu0__blk1193_dn17, ) = (assign35950_e50700, (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk1185 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))), );
            locals.var_mu0__blk1193_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign35960_e50707: f64 = (0.4 * locals.var_tratio__blk1190);
            let assign35960_e50708: f64 = (1.8 + assign35960_e50707);
            let assign35960_e50711: f64 = (0.1 * locals.var_tratio__blk1190);
            let assign35960_e50713: f64 = (assign35960_e50711 * locals.var_tratio__blk1190);
            let assign35960_e50714: f64 = (assign35960_e50708 + assign35960_e50713);
            let assign35960_e50718: f64 = (1.0 - locals.var_tratio__blk1190);
            let assign35960_e50719: f64 = (p.p270 * assign35960_e50718);
            let assign35960_e50720: f64 = (assign35960_e50714 - assign35960_e50719);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17, ) = (assign35960_e50720, 0.0, 0.0, 0.0, 0.0, (((0.4 * locals.var_tratio__blk1190_dn10) + (((0.1 * locals.var_tratio__blk1190_dn10) * locals.var_tratio__blk1190) + (assign35960_e50711 * locals.var_tratio__blk1190_dn10))) - (p.p270 * (-locals.var_tratio__blk1190_dn10))), 0.0, 0.0, 0.0, );
            locals.var_t0_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign35970_e50726: f64 = (locals.var_mks_rdrvmax__blk1186 / locals.var_t0);
            (locals.var_vmaxe__blk1194, locals.var_vmaxe__blk1194_dn0, locals.var_vmaxe__blk1194_dn2, locals.var_vmaxe__blk1194_dn6, locals.var_vmaxe__blk1194_dn7, locals.var_vmaxe__blk1194_dn10, locals.var_vmaxe__blk1194_dn11, locals.var_vmaxe__blk1194_dn12, locals.var_vmaxe__blk1194_dn17, ) = (assign35970_e50726, (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk1186 * locals.var_t0_dn17) / (locals.var_t0 * locals.var_t0))), );
            locals.var_vmaxe__blk1194_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign35980_e50734: f64 = (locals.var_ttemp - locals.var_uc_tnom);
            let assign35980_e50735: f64 = (p.p274 * assign35980_e50734);
            let assign35980_e50736: f64 = (locals.var_rrdrbb__blk1187 + assign35980_e50735);
            (locals.var_rrdrbb__blk1187, locals.var_rrdrbb__blk1187_dn10, ) = (assign35980_e50736, (locals.var_rrdrbb__blk1187_dn10 + (p.p274 * locals.var_ttemp_dn10)), );
            locals.var_rrdrbb__blk1187_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign35990_e50744: f64 = (locals.var_lgle).powf(p.p280);
            let assign35990_e50745: f64 = (p.p279 / assign35990_e50744);
            let assign35990_e50746: f64 = (1.0 + assign35990_e50745);
            locals.var_rdrmuele__blk1182 = assign35990_e50746;
            locals.var_rdrmuele__blk1182_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign36000_e50754: f64 = (locals.var_lgle).powf(p.p278);
            let assign36000_e50755: f64 = (p.p277 / assign36000_e50754);
            let assign36000_e50756: f64 = (1.0 + assign36000_e50755);
            locals.var_rdrvmaxle__blk1184 = assign36000_e50756;
            locals.var_rdrvmaxle__blk1184_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign36010_e50764: f64 = (locals.var_wg).powf(p.p276);
            let assign36010_e50765: f64 = (p.p275 / assign36010_e50764);
            let assign36010_e50766: f64 = (1.0 + assign36010_e50765);
            locals.var_rdrvmaxwe__blk1183 = assign36010_e50766;
            locals.var_rdrvmaxwe__blk1183_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign36020_e50772: f64 = (locals.var_mu0__blk1193 * locals.var_rdrmuele__blk1182);
            (locals.var_mu0__blk1193, locals.var_mu0__blk1193_dn0, locals.var_mu0__blk1193_dn2, locals.var_mu0__blk1193_dn6, locals.var_mu0__blk1193_dn7, locals.var_mu0__blk1193_dn10, locals.var_mu0__blk1193_dn11, locals.var_mu0__blk1193_dn12, locals.var_mu0__blk1193_dn17, ) = (assign36020_e50772, (locals.var_mu0__blk1193_dn0 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn2 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn6 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn7 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn10 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn11 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn12 * locals.var_rdrmuele__blk1182), (locals.var_mu0__blk1193_dn17 * locals.var_rdrmuele__blk1182), );
            locals.var_mu0__blk1193_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign36030_e50778: f64 = (locals.var_vmaxe__blk1194 * locals.var_rdrvmaxwe__blk1183);
            let assign36030_e50780: f64 = (assign36030_e50778 * locals.var_rdrvmaxle__blk1184);
            let assign36030_e50782: f64 = (assign36030_e50780 + 1e-50);
            (locals.var_vmaxe__blk1194, locals.var_vmaxe__blk1194_dn0, locals.var_vmaxe__blk1194_dn2, locals.var_vmaxe__blk1194_dn6, locals.var_vmaxe__blk1194_dn7, locals.var_vmaxe__blk1194_dn10, locals.var_vmaxe__blk1194_dn11, locals.var_vmaxe__blk1194_dn12, locals.var_vmaxe__blk1194_dn17, ) = (assign36030_e50782, ((locals.var_vmaxe__blk1194_dn0 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn2 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn6 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn7 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn10 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn11 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn12 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), ((locals.var_vmaxe__blk1194_dn17 * locals.var_rdrvmaxwe__blk1183) * locals.var_rdrvmaxle__blk1184), );
            locals.var_vmaxe__blk1194_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign36040_e50788: f64 = (locals.var_vrdr__blk1189 / locals.var_ldrifte__blk1191);
            (locals.var_edri__blk1195, locals.var_edri__blk1195_dn0, locals.var_edri__blk1195_dn2, locals.var_edri__blk1195_dn6, locals.var_edri__blk1195_dn7, ) = (assign36040_e50788, (locals.var_vrdr__blk1189_dn0 / locals.var_ldrifte__blk1191), (locals.var_vrdr__blk1189_dn2 / locals.var_ldrifte__blk1191), (locals.var_vrdr__blk1189_dn6 / locals.var_ldrifte__blk1191), (locals.var_vrdr__blk1189_dn7 / locals.var_ldrifte__blk1191), );
            locals.var_edri__blk1195_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign36050_e50794: f64 = (locals.var_mu0__blk1193 * locals.var_edri__blk1195);
            (locals.var_vdri__blk1196, locals.var_vdri__blk1196_dn0, locals.var_vdri__blk1196_dn2, locals.var_vdri__blk1196_dn6, locals.var_vdri__blk1196_dn7, locals.var_vdri__blk1196_dn10, locals.var_vdri__blk1196_dn11, locals.var_vdri__blk1196_dn12, locals.var_vdri__blk1196_dn17, ) = (assign36050_e50794, ((locals.var_mu0__blk1193_dn0 * locals.var_edri__blk1195) + (locals.var_mu0__blk1193 * locals.var_edri__blk1195_dn0)), ((locals.var_mu0__blk1193_dn2 * locals.var_edri__blk1195) + (locals.var_mu0__blk1193 * locals.var_edri__blk1195_dn2)), ((locals.var_mu0__blk1193_dn6 * locals.var_edri__blk1195) + (locals.var_mu0__blk1193 * locals.var_edri__blk1195_dn6)), ((locals.var_mu0__blk1193_dn7 * locals.var_edri__blk1195) + (locals.var_mu0__blk1193 * locals.var_edri__blk1195_dn7)), (locals.var_mu0__blk1193_dn10 * locals.var_edri__blk1195), (locals.var_mu0__blk1193_dn11 * locals.var_edri__blk1195), (locals.var_mu0__blk1193_dn12 * locals.var_edri__blk1195), (locals.var_mu0__blk1193_dn17 * locals.var_edri__blk1195), );
            locals.var_vdri__blk1196_rv = 0.0;
        }
        let assign36060_e50799: f64 = if locals.var_vrdr__blk1189 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign36060_e50799;
        locals.var_guard1202_rv = 0.0;
        if ((locals.var_guard1181 != 0.0) && (locals.var_guard1202 != 0.0)) {
            let assign36070_e50805: f64 = (locals.var_vdri__blk1196 / locals.var_vmaxe__blk1194);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36070_e50805, (((locals.var_vdri__blk1196_dn0 * locals.var_vmaxe__blk1194) - (locals.var_vdri__blk1196 * locals.var_vmaxe__blk1194_dn0)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), (((locals.var_vdri__blk1196_dn2 * locals.var_vmaxe__blk1194) - (locals.var_vdri__blk1196 * locals.var_vmaxe__blk1194_dn2)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), (((locals.var_vdri__blk1196_dn6 * locals.var_vmaxe__blk1194) - (locals.var_vdri__blk1196 * locals.var_vmaxe__blk1194_dn6)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), (((locals.var_vdri__blk1196_dn7 * locals.var_vmaxe__blk1194) - (locals.var_vdri__blk1196 * locals.var_vmaxe__blk1194_dn7)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), (((locals.var_vdri__blk1196_dn10 * locals.var_vmaxe__blk1194) - (locals.var_vdri__blk1196 * locals.var_vmaxe__blk1194_dn10)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), (((locals.var_vdri__blk1196_dn11 * locals.var_vmaxe__blk1194) - (locals.var_vdri__blk1196 * locals.var_vmaxe__blk1194_dn11)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), (((locals.var_vdri__blk1196_dn12 * locals.var_vmaxe__blk1194) - (locals.var_vdri__blk1196 * locals.var_vmaxe__blk1194_dn12)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), (((locals.var_vdri__blk1196_dn17 * locals.var_vmaxe__blk1194) - (locals.var_vdri__blk1196 * locals.var_vmaxe__blk1194_dn17)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), );
            locals.var_t1_rv = 0.0;
        }
        if ((locals.var_guard1181 != 0.0) && (locals.var_guard1202 == 0.0)) {
            let assign36080_e50813: f64 = (-locals.var_vdri__blk1196);
            let assign36080_e50815: f64 = (assign36080_e50813 / locals.var_vmaxe__blk1194);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36080_e50815, ((((-locals.var_vdri__blk1196_dn0) * locals.var_vmaxe__blk1194) - (assign36080_e50813 * locals.var_vmaxe__blk1194_dn0)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), ((((-locals.var_vdri__blk1196_dn2) * locals.var_vmaxe__blk1194) - (assign36080_e50813 * locals.var_vmaxe__blk1194_dn2)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), ((((-locals.var_vdri__blk1196_dn6) * locals.var_vmaxe__blk1194) - (assign36080_e50813 * locals.var_vmaxe__blk1194_dn6)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), ((((-locals.var_vdri__blk1196_dn7) * locals.var_vmaxe__blk1194) - (assign36080_e50813 * locals.var_vmaxe__blk1194_dn7)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), ((((-locals.var_vdri__blk1196_dn10) * locals.var_vmaxe__blk1194) - (assign36080_e50813 * locals.var_vmaxe__blk1194_dn10)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), ((((-locals.var_vdri__blk1196_dn11) * locals.var_vmaxe__blk1194) - (assign36080_e50813 * locals.var_vmaxe__blk1194_dn11)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), ((((-locals.var_vdri__blk1196_dn12) * locals.var_vmaxe__blk1194) - (assign36080_e50813 * locals.var_vmaxe__blk1194_dn12)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), ((((-locals.var_vdri__blk1196_dn17) * locals.var_vmaxe__blk1194) - (assign36080_e50813 * locals.var_vmaxe__blk1194_dn17)) / (locals.var_vmaxe__blk1194 * locals.var_vmaxe__blk1194)), );
            locals.var_t1_rv = 0.0;
        }
        let assign36090_e50821: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50822: f64 = (1.0 - assign36090_e50821);
        let assign36090_e50829: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50830: f64 = (1.0 + assign36090_e50829);
        let assign36090_e50832: f64 = if ((assign36090_e50822 <= locals.var_rrdrbb__blk1187) && (locals.var_rrdrbb__blk1187 <= assign36090_e50830)) { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign36090_e50832;
        locals.var_guard1203_rv = 0.0;
        if ((locals.var_guard1181 != 0.0) && (locals.var_guard1203 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t3_rv = 0.0;
        }
        let assign36110_e50842: f64 = (10.0 * 2.220446049250313e-16);
        let assign36110_e50843: f64 = (2.0 - assign36110_e50842);
        let assign36110_e50850: f64 = (10.0 * 2.220446049250313e-16);
        let assign36110_e50851: f64 = (2.0 + assign36110_e50850);
        let assign36110_e50853: f64 = if ((assign36110_e50843 <= locals.var_rrdrbb__blk1187) && (locals.var_rrdrbb__blk1187 <= assign36110_e50851)) { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign36110_e50853;
        locals.var_guard1204_rv = 0.0;
        if (((locals.var_guard1181 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 != 0.0)) {
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, );
            locals.var_t3_rv = 0.0;
        }
        if (((locals.var_guard1181 != 0.0) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
            let assign36130_e50873: f64 = (locals.var_rrdrbb__blk1187 - 1.0);
            let assign36130_e50874: f64 = (locals.var_t1).powf(assign36130_e50873);
            (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17, ) = (assign36130_e50874, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((locals.var_t1).powf(assign36130_e50873 - 1.0) * locals.var_t1_dn0)) } } else { (assign36130_e50874 * (assign36130_e50873 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((locals.var_t1).powf(assign36130_e50873 - 1.0) * locals.var_t1_dn2)) } } else { (assign36130_e50874 * (assign36130_e50873 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((locals.var_t1).powf(assign36130_e50873 - 1.0) * locals.var_t1_dn6)) } } else { (assign36130_e50874 * (assign36130_e50873 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((locals.var_t1).powf(assign36130_e50873 - 1.0) * locals.var_t1_dn7)) } } else { (assign36130_e50874 * (assign36130_e50873 * (locals.var_t1_dn7 / locals.var_t1))) }, if locals.var_rrdrbb__blk1187_dn10 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((locals.var_t1).powf(assign36130_e50873 - 1.0) * locals.var_t1_dn10)) } } else { (assign36130_e50874 * ((locals.var_rrdrbb__blk1187_dn10 * (locals.var_t1).ln()) + (assign36130_e50873 * (locals.var_t1_dn10 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((locals.var_t1).powf(assign36130_e50873 - 1.0) * locals.var_t1_dn11)) } } else { (assign36130_e50874 * (assign36130_e50873 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((locals.var_t1).powf(assign36130_e50873 - 1.0) * locals.var_t1_dn12)) } } else { (assign36130_e50874 * (assign36130_e50873 * (locals.var_t1_dn12 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((locals.var_t1).powf(assign36130_e50873 - 1.0) * locals.var_t1_dn17)) } } else { (assign36130_e50874 * (assign36130_e50873 * (locals.var_t1_dn17 / locals.var_t1))) }, );
            locals.var_t3_rv = 0.0;
        }
    }
    pub(super) fn stamp_reactive_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1181 != 0.0) {
            let assign36140_e50880: f64 = (locals.var_t1 * locals.var_t3);
            (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, ) = (assign36140_e50880, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)), ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)), );
            locals.var_t2_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign36150_e50886: f64 = (1.0 + locals.var_t2);
            (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17, ) = (assign36150_e50886, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17, );
            locals.var_t4_rv = 0.0;
        }
        let assign36160_e50892: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50893: f64 = (1.0 - assign36160_e50892);
        let assign36160_e50900: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50901: f64 = (1.0 + assign36160_e50900);
        let assign36160_e50903: f64 = if ((assign36160_e50893 <= locals.var_rrdrbb__blk1187) && (locals.var_rrdrbb__blk1187 <= assign36160_e50901)) { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign36160_e50903;
        locals.var_guard1205_rv = 0.0;
        if ((locals.var_guard1181 != 0.0) && (locals.var_guard1205 != 0.0)) {
            let assign36170_e50909: f64 = (1.0 / locals.var_t4);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36170_e50909, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn17 / (locals.var_t4 * locals.var_t4))), );
            locals.var_t5_rv = 0.0;
        }
        let assign36180_e50915: f64 = (10.0 * 2.220446049250313e-16);
        let assign36180_e50916: f64 = (2.0 - assign36180_e50915);
        let assign36180_e50923: f64 = (10.0 * 2.220446049250313e-16);
        let assign36180_e50924: f64 = (2.0 + assign36180_e50923);
        let assign36180_e50926: f64 = if ((assign36180_e50916 <= locals.var_rrdrbb__blk1187) && (locals.var_rrdrbb__blk1187 <= assign36180_e50924)) { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign36180_e50926;
        locals.var_guard1206_rv = 0.0;
        if (((locals.var_guard1181 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 != 0.0)) {
            let assign36190_e50935: f64 = (locals.var_t4).sqrt();
            let assign36190_e50936: f64 = (1.0 / assign36190_e50935);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36190_e50936, (-((locals.var_t4_dn0 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((locals.var_t4_dn2 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((locals.var_t4_dn6 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((locals.var_t4_dn7 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((locals.var_t4_dn10 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((locals.var_t4_dn11 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((locals.var_t4_dn12 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((locals.var_t4_dn17 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), );
            locals.var_t5_rv = 0.0;
        }
        if (((locals.var_guard1181 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 == 0.0)) {
            let assign36200_e50948: f64 = (-1.0);
            let assign36200_e50950: f64 = (assign36200_e50948 / locals.var_rrdrbb__blk1187);
            let assign36200_e50952: f64 = (assign36200_e50950 - 1.0);
            let assign36200_e50953: f64 = (locals.var_t4).powf(assign36200_e50952);
            (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17, ) = (assign36200_e50953, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((locals.var_t4).powf(assign36200_e50952 - 1.0) * locals.var_t4_dn0)) } } else { (assign36200_e50953 * (assign36200_e50952 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((locals.var_t4).powf(assign36200_e50952 - 1.0) * locals.var_t4_dn2)) } } else { (assign36200_e50953 * (assign36200_e50952 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((locals.var_t4).powf(assign36200_e50952 - 1.0) * locals.var_t4_dn6)) } } else { (assign36200_e50953 * (assign36200_e50952 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((locals.var_t4).powf(assign36200_e50952 - 1.0) * locals.var_t4_dn7)) } } else { (assign36200_e50953 * (assign36200_e50952 * (locals.var_t4_dn7 / locals.var_t4))) }, if (-((assign36200_e50948 * locals.var_rrdrbb__blk1187_dn10) / (locals.var_rrdrbb__blk1187 * locals.var_rrdrbb__blk1187))) == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((locals.var_t4).powf(assign36200_e50952 - 1.0) * locals.var_t4_dn10)) } } else { (assign36200_e50953 * (((-((assign36200_e50948 * locals.var_rrdrbb__blk1187_dn10) / (locals.var_rrdrbb__blk1187 * locals.var_rrdrbb__blk1187))) * (locals.var_t4).ln()) + (assign36200_e50952 * (locals.var_t4_dn10 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((locals.var_t4).powf(assign36200_e50952 - 1.0) * locals.var_t4_dn11)) } } else { (assign36200_e50953 * (assign36200_e50952 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((locals.var_t4).powf(assign36200_e50952 - 1.0) * locals.var_t4_dn12)) } } else { (assign36200_e50953 * (assign36200_e50952 * (locals.var_t4_dn12 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((locals.var_t4).powf(assign36200_e50952 - 1.0) * locals.var_t4_dn17)) } } else { (assign36200_e50953 * (assign36200_e50952 * (locals.var_t4_dn17 / locals.var_t4))) }, );
            locals.var_t6_rv = 0.0;
        }
        if (((locals.var_guard1181 != 0.0) && (locals.var_guard1205 == 0.0)) && (locals.var_guard1206 == 0.0)) {
            let assign36210_e50965: f64 = (locals.var_t4 * locals.var_t6);
            (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17, ) = (assign36210_e50965, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)), ((locals.var_t4_dn17 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn17)), );
            locals.var_t5_rv = 0.0;
        }
        if (locals.var_guard1181 != 0.0) {
            let assign36230_e50977: f64 = (1.6021918e-19 / locals.var_ldrifte__blk1191);
            (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17, ) = (assign36230_e50977, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_t1_rv = 0.0;
        }
        let assign36350_e51053: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign36350_e51053;
        locals.var_guard1209_rv = 0.0;
        if ((locals.var_guard1209 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let (assign36400_e51094, assign36400_e51094_d_n0, assign36400_e51094_d_n2, assign36400_e51094_d_n6, assign36400_e51094_d_n7, assign36400_e51094_d_n10, assign36400_e51094_d_n11, assign36400_e51094_d_n12, assign36400_e51094_d_n17,) = {
    if (locals.var_mode == 1.0) {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn10, locals.var_xd_dn11, locals.var_xd_dn12, locals.var_xd_dn17,)
    } else {
        let assign36400_e51093: f64 = (1.0 - locals.var_xd);
        (assign36400_e51093, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn12), (-locals.var_xd_dn17),)
    }
};
            (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, ) = (assign36400_e51094, assign36400_e51094_d_n0, assign36400_e51094_d_n2, assign36400_e51094_d_n6, assign36400_e51094_d_n7, assign36400_e51094_d_n10, assign36400_e51094_d_n11, assign36400_e51094_d_n12, assign36400_e51094_d_n17, );
            locals.var_qdrat_rv = 0.0;
        }
        if ((locals.var_guard1209 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36430_e51122: f64 = (locals.var_qi_nqs * locals.var_qdrat);
            let assign36430_e51124: f64 = (assign36430_e51122 + locals.var_q_bt_se);
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (assign36430_e51124, ((locals.var_qi_nqs * locals.var_qdrat_dn0) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * locals.var_qdrat_dn2) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * locals.var_qdrat_dn6) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * locals.var_qdrat_dn7) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * locals.var_qdrat_dn10) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * locals.var_qdrat_dn11) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * locals.var_qdrat_dn12) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * locals.var_qdrat_dn17) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * locals.var_qdrat), );
            locals.var_qd_nqs_rv = 0.0;
        }
        if ((locals.var_guard1209 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36440_e51133: f64 = (1.0 - locals.var_qdrat);
            let assign36440_e51134: f64 = (locals.var_qi_nqs * assign36440_e51133);
            let assign36440_e51136: f64 = (assign36440_e51134 + locals.var_q_bt_se);
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (assign36440_e51136, ((locals.var_qi_nqs * (-locals.var_qdrat_dn0)) + locals.var_q_bt_se_dn0), ((locals.var_qi_nqs * (-locals.var_qdrat_dn2)) + locals.var_q_bt_se_dn2), ((locals.var_qi_nqs * (-locals.var_qdrat_dn6)) + locals.var_q_bt_se_dn6), ((locals.var_qi_nqs * (-locals.var_qdrat_dn7)) + locals.var_q_bt_se_dn7), ((locals.var_qi_nqs * (-locals.var_qdrat_dn10)) + locals.var_q_bt_se_dn10), ((locals.var_qi_nqs * (-locals.var_qdrat_dn11)) + locals.var_q_bt_se_dn11), ((locals.var_qi_nqs * (-locals.var_qdrat_dn12)) + locals.var_q_bt_se_dn12), 0.0, ((locals.var_qi_nqs * (-locals.var_qdrat_dn17)) + locals.var_q_bt_se_dn17), (locals.var_qi_nqs_dn18 * assign36440_e51133), );
            locals.var_qs_nqs_rv = 0.0;
        }
        if ((locals.var_guard1209 != 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36450_e51143: f64 = (-locals.var_qi_nqs);
            let assign36450_e51145: f64 = (assign36450_e51143 - locals.var_qb_nqs);
            let assign36450_e51147: f64 = (assign36450_e51145 + locals.var_q_bt_ge);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (assign36450_e51147, locals.var_q_bt_ge_dn0, locals.var_q_bt_ge_dn2, locals.var_q_bt_ge_dn6, locals.var_q_bt_ge_dn7, locals.var_q_bt_ge_dn10, locals.var_q_bt_ge_dn11, locals.var_q_bt_ge_dn12, (-locals.var_qb_nqs_dn13), 0.0, 0.0, locals.var_q_bt_ge_dn17, (-locals.var_qi_nqs_dn18), );
            locals.var_qg_nqs_rv = 0.0;
        }
        if ((locals.var_guard1209 != 0.0) && (locals.var_flg_nqs == 0.0)) {
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qb_nqs, locals.var_qb_nqs_dn13, ) = (0.0, 0.0, );
            locals.var_qb_nqs_rv = 0.0;
        }
        if ((locals.var_guard1209 == 0.0) && (locals.var_flg_nqs != 0.0)) {
            let assign36600_e51269: f64 = (-locals.var_qd_nqs);
            let assign36600_e51271: f64 = (assign36600_e51269 - locals.var_qs_nqs);
            let assign36600_e51273: f64 = (assign36600_e51271 - locals.var_qb_nqs);
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (assign36600_e51273, ((-locals.var_qd_nqs_dn0) - locals.var_qs_nqs_dn0), ((-locals.var_qd_nqs_dn2) - locals.var_qs_nqs_dn2), ((-locals.var_qd_nqs_dn6) - locals.var_qs_nqs_dn6), ((-locals.var_qd_nqs_dn7) - locals.var_qs_nqs_dn7), ((-locals.var_qd_nqs_dn10) - locals.var_qs_nqs_dn10), ((-locals.var_qd_nqs_dn11) - locals.var_qs_nqs_dn11), ((-locals.var_qd_nqs_dn12) - locals.var_qs_nqs_dn12), (-locals.var_qb_nqs_dn13), (-locals.var_qd_nqs_dn15), (-locals.var_qs_nqs_dn16), ((-locals.var_qd_nqs_dn17) - locals.var_qs_nqs_dn17), ((-locals.var_qd_nqs_dn18) - locals.var_qs_nqs_dn18), );
            locals.var_qg_nqs_rv = 0.0;
        }
        if ((locals.var_guard1209 == 0.0) && (locals.var_flg_nqs == 0.0)) {
            (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qd_nqs_rv = 0.0;
            (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qs_nqs_rv = 0.0;
            (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qg_nqs_rv = 0.0;
            (locals.var_qb_nqs, locals.var_qb_nqs_dn13, ) = (0.0, 0.0, );
            locals.var_qb_nqs_rv = 0.0;
        }
        let assign36700_e51336: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign36700_e51336;
        locals.var_guard1214_rv = 0.0;
        if (locals.var_guard1214 != 0.0) {
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, );
            locals.var_ids_rv = 0.0;
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17, ) = (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17, );
            locals.var_isub_rv = 0.0;
        }
        if (locals.var_guard1214 != 0.0) {
            let assign36740_e51352: f64 = (locals.var_qge + locals.var_qg_nqs);
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18, ) = (assign36740_e51352, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18), );
            locals.var_qg_rv = 0.0;
        }
        if (locals.var_guard1214 != 0.0) {
            let assign36750_e51358: f64 = (locals.var_qde + locals.var_qd_nqs);
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, ) = (assign36750_e51358, (locals.var_qde_dn0 + locals.var_qd_nqs_dn0), (locals.var_qde_dn2 + locals.var_qd_nqs_dn2), (locals.var_qde_dn6 + locals.var_qd_nqs_dn6), (locals.var_qde_dn7 + locals.var_qd_nqs_dn7), (locals.var_qde_dn10 + locals.var_qd_nqs_dn10), (locals.var_qde_dn11 + locals.var_qd_nqs_dn11), (locals.var_qde_dn12 + locals.var_qd_nqs_dn12), locals.var_qde_dn13, (locals.var_qde_dn15 + locals.var_qd_nqs_dn15), locals.var_qde_dn16, (locals.var_qde_dn17 + locals.var_qd_nqs_dn17), (locals.var_qde_dn18 + locals.var_qd_nqs_dn18), );
            locals.var_qd_rv = 0.0;
        }
        if (locals.var_guard1214 != 0.0) {
            let assign36770_e51370: f64 = (locals.var_qge + locals.var_qde);
            let assign36770_e51372: f64 = (assign36770_e51370 + locals.var_qse);
            let assign36770_e51373: f64 = (-assign36770_e51372);
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (assign36770_e51373, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)), );
            locals.var_qbe_rv = 0.0;
        }
        if (locals.var_guard1214 != 0.0) {
            let assign36780_e51379: f64 = (locals.var_qbe + locals.var_qb_nqs);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18, ) = (assign36780_e51379, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, );
            locals.var_qb_rv = 0.0;
        }
        if (locals.var_guard1214 == 0.0) {
            let assign36790_e51385: f64 = (-locals.var_idse);
            (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, ) = (assign36790_e51385, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12), (-locals.var_idse_dn17), );
            locals.var_ids_rv = 0.0;
        }
        if (locals.var_guard1214 == 0.0) {
            (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_isub_rv = 0.0;
        }
        if (locals.var_guard1214 == 0.0) {
            let assign36820_e51402: f64 = (locals.var_qge + locals.var_qg_nqs);
            (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18, ) = (assign36820_e51402, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18), );
            locals.var_qg_rv = 0.0;
        }
        if (locals.var_guard1214 == 0.0) {
            let assign36830_e51409: f64 = (locals.var_qse + locals.var_qs_nqs);
            (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18, ) = (assign36830_e51409, (locals.var_qse_dn0 + locals.var_qs_nqs_dn0), (locals.var_qse_dn2 + locals.var_qs_nqs_dn2), (locals.var_qse_dn6 + locals.var_qs_nqs_dn6), (locals.var_qse_dn7 + locals.var_qs_nqs_dn7), (locals.var_qse_dn10 + locals.var_qs_nqs_dn10), (locals.var_qse_dn11 + locals.var_qs_nqs_dn11), (locals.var_qse_dn12 + locals.var_qs_nqs_dn12), locals.var_qse_dn13, locals.var_qse_dn15, (locals.var_qse_dn16 + locals.var_qs_nqs_dn16), (locals.var_qse_dn17 + locals.var_qs_nqs_dn17), (locals.var_qse_dn18 + locals.var_qs_nqs_dn18), );
            locals.var_qd_rv = 0.0;
        }
        if (locals.var_guard1214 == 0.0) {
            let assign36850_e51423: f64 = (locals.var_qge + locals.var_qde);
            let assign36850_e51425: f64 = (assign36850_e51423 + locals.var_qse);
            let assign36850_e51426: f64 = (-assign36850_e51425);
            (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, ) = (assign36850_e51426, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)), );
            locals.var_qbe_rv = 0.0;
        }
        if (locals.var_guard1214 == 0.0) {
            let assign36860_e51433: f64 = (locals.var_qbe + locals.var_qb_nqs);
            (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18, ) = (assign36860_e51433, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18, );
            locals.var_qb_rv = 0.0;
        }
        let assign36920_e51443: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign36920_e51443;
        locals.var_guard1215_rv = 0.0;
        if (locals.var_guard1215 != 0.0) {
            (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17, ) = (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, );
            locals.var_ibd_rv = 0.0;
            (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17, ) = (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17, );
            locals.var_qbd_rv = 0.0;
            (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17, ) = (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, );
            locals.var_ibs_rv = 0.0;
            (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17, ) = (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17, );
            locals.var_qbs_rv = 0.0;
        }
        let assign36970_e51466: f64 = if ((p.p38 == 1.0) && (locals.var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign36970_e51466;
        locals.var_guard1216_rv = 0.0;
        if (locals.var_guard1216 != 0.0) {
            locals.var_cthe = locals.var_cth;
            locals.var_cthe_rv = 0.0;
        }
        if (locals.var_guard1216 == 0.0) {
            locals.var_cthe = 0.0;
            locals.var_cthe_rv = 0.0;
        }
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17, ) = (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17, );
        locals.var_idse_rv = 0.0;
        let assign37490_e51636: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign37490_e51636;
        locals.var_guard1218_rv = 0.0;
        if (locals.var_guard1218 != 0.0) {
            let assign37500_e51640: f64 = (p.p50 * locals.var_ibd);
            (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17, ) = (assign37500_e51640, (p.p50 * locals.var_ibd_dn0), (p.p50 * locals.var_ibd_dn2), (p.p50 * locals.var_ibd_dn6), (p.p50 * locals.var_ibd_dn7), (p.p50 * locals.var_ibd_dn10), (p.p50 * locals.var_ibd_dn11), (p.p50 * locals.var_ibd_dn12), (p.p50 * locals.var_ibd_dn17), );
            locals.var_ibdb_rv = 0.0;
        }
        if (locals.var_guard1218 != 0.0) {
            let assign37510_e51646: f64 = (p.p50 * locals.var_ibs);
            (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17, ) = (assign37510_e51646, (p.p50 * locals.var_ibs_dn0), (p.p50 * locals.var_ibs_dn2), (p.p50 * locals.var_ibs_dn6), (p.p50 * locals.var_ibs_dn7), (p.p50 * locals.var_ibs_dn10), (p.p50 * locals.var_ibs_dn11), (p.p50 * locals.var_ibs_dn12), (p.p50 * locals.var_ibs_dn17), );
            locals.var_ibsb_rv = 0.0;
        }
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn12, locals.var_qdrat_dn17, ) = (locals.var_qdrat_noi, locals.var_qdrat_noi_dn0, locals.var_qdrat_noi_dn2, locals.var_qdrat_noi_dn6, locals.var_qdrat_noi_dn7, locals.var_qdrat_noi_dn10, locals.var_qdrat_noi_dn11, locals.var_qdrat_noi_dn12, locals.var_qdrat_noi_dn17, );
        locals.var_qdrat_rv = 0.0;
        let assign37720_e51759: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign37720_e51759;
        locals.var_guard1226_rv = 0.0;
        let assign37740_e51766: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign37740_e51766;
        locals.var_guard1227_rv = 0.0;
        let assign37750_e51775: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign37750_e51775;
        locals.var_guard1228_rv = 0.0;
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
        let eq11_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qg);
        let eq11_e368: f64 = (p.p50 * eq11_e367);
        let eq11_e368_d_n0: f64 = (p.p50 * (locals.var_qg_dn0 * ddt_scale));
        let eq11_e368_d_n2: f64 = (p.p50 * (locals.var_qg_dn2 * ddt_scale));
        let eq11_e368_d_n6: f64 = (p.p50 * (locals.var_qg_dn6 * ddt_scale));
        let eq11_e368_d_n7: f64 = (p.p50 * (locals.var_qg_dn7 * ddt_scale));
        let eq11_e368_d_n10: f64 = (p.p50 * (locals.var_qg_dn10 * ddt_scale));
        let eq11_e368_d_n11: f64 = (p.p50 * (locals.var_qg_dn11 * ddt_scale));
        let eq11_e368_d_n12: f64 = (p.p50 * (locals.var_qg_dn12 * ddt_scale));
        let eq11_e368_d_n13: f64 = (p.p50 * (locals.var_qg_dn13 * ddt_scale));
        let eq11_e368_d_n15: f64 = (p.p50 * (locals.var_qg_dn15 * ddt_scale));
        let eq11_e368_d_n16: f64 = (p.p50 * (locals.var_qg_dn16 * ddt_scale));
        let eq11_e368_d_n17: f64 = (p.p50 * (locals.var_qg_dn17 * ddt_scale));
        let eq11_e368_d_n18: f64 = (p.p50 * (locals.var_qg_dn18 * ddt_scale));
        let eq11_value: f64 = eq11_e368;
        let eq11_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq11_node_derivatives: [f64; 12] = [eq11_e368_d_n0, eq11_e368_d_n2, eq11_e368_d_n6, eq11_e368_d_n7, eq11_e368_d_n10, eq11_e368_d_n11, eq11_e368_d_n12, eq11_e368_d_n13, eq11_e368_d_n15, eq11_e368_d_n16, eq11_e368_d_n17, eq11_e368_d_n18];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e371: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qd);
        let eq12_e372: f64 = (p.p50 * eq12_e371);
        let eq12_e372_d_n0: f64 = (p.p50 * (locals.var_qd_dn0 * ddt_scale));
        let eq12_e372_d_n2: f64 = (p.p50 * (locals.var_qd_dn2 * ddt_scale));
        let eq12_e372_d_n6: f64 = (p.p50 * (locals.var_qd_dn6 * ddt_scale));
        let eq12_e372_d_n7: f64 = (p.p50 * (locals.var_qd_dn7 * ddt_scale));
        let eq12_e372_d_n10: f64 = (p.p50 * (locals.var_qd_dn10 * ddt_scale));
        let eq12_e372_d_n11: f64 = (p.p50 * (locals.var_qd_dn11 * ddt_scale));
        let eq12_e372_d_n12: f64 = (p.p50 * (locals.var_qd_dn12 * ddt_scale));
        let eq12_e372_d_n13: f64 = (p.p50 * (locals.var_qd_dn13 * ddt_scale));
        let eq12_e372_d_n15: f64 = (p.p50 * (locals.var_qd_dn15 * ddt_scale));
        let eq12_e372_d_n16: f64 = (p.p50 * (locals.var_qd_dn16 * ddt_scale));
        let eq12_e372_d_n17: f64 = (p.p50 * (locals.var_qd_dn17 * ddt_scale));
        let eq12_e372_d_n18: f64 = (p.p50 * (locals.var_qd_dn18 * ddt_scale));
        let eq12_value: f64 = eq12_e372;
        let eq12_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq12_node_derivatives: [f64; 12] = [eq12_e372_d_n0, eq12_e372_d_n2, eq12_e372_d_n6, eq12_e372_d_n7, eq12_e372_d_n10, eq12_e372_d_n11, eq12_e372_d_n12, eq12_e372_d_n13, eq12_e372_d_n15, eq12_e372_d_n16, eq12_e372_d_n17, eq12_e372_d_n18];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qb);
        let eq13_e376: f64 = (p.p50 * eq13_e375);
        let eq13_e376_d_n0: f64 = (p.p50 * (locals.var_qb_dn0 * ddt_scale));
        let eq13_e376_d_n2: f64 = (p.p50 * (locals.var_qb_dn2 * ddt_scale));
        let eq13_e376_d_n6: f64 = (p.p50 * (locals.var_qb_dn6 * ddt_scale));
        let eq13_e376_d_n7: f64 = (p.p50 * (locals.var_qb_dn7 * ddt_scale));
        let eq13_e376_d_n10: f64 = (p.p50 * (locals.var_qb_dn10 * ddt_scale));
        let eq13_e376_d_n11: f64 = (p.p50 * (locals.var_qb_dn11 * ddt_scale));
        let eq13_e376_d_n12: f64 = (p.p50 * (locals.var_qb_dn12 * ddt_scale));
        let eq13_e376_d_n13: f64 = (p.p50 * (locals.var_qb_dn13 * ddt_scale));
        let eq13_e376_d_n15: f64 = (p.p50 * (locals.var_qb_dn15 * ddt_scale));
        let eq13_e376_d_n16: f64 = (p.p50 * (locals.var_qb_dn16 * ddt_scale));
        let eq13_e376_d_n17: f64 = (p.p50 * (locals.var_qb_dn17 * ddt_scale));
        let eq13_e376_d_n18: f64 = (p.p50 * (locals.var_qb_dn18 * ddt_scale));
        let eq13_value: f64 = eq13_e376;
        let eq13_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq13_node_derivatives: [f64; 12] = [eq13_e376_d_n0, eq13_e376_d_n2, eq13_e376_d_n6, eq13_e376_d_n7, eq13_e376_d_n10, eq13_e376_d_n11, eq13_e376_d_n12, eq13_e376_d_n13, eq13_e376_d_n15, eq13_e376_d_n16, eq13_e376_d_n17, eq13_e376_d_n18];
        let eq13_branch_derivative_indices: [usize; 0] = [];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq13_value),
            &eq13_node_derivative_indices,
            &eq13_node_derivatives,
            &eq13_branch_derivative_indices,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let (eq31_e491, eq31_e491_d_n10,) = {
    if (locals.var_guard1226 != 0.0) {
        let eq31_e488: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq31_e489: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq31_e488);
        (eq31_e489, (locals.var_cthe * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e491;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq31_value),
            10,
            multiplicity * (eq31_e491_d_n10),
        );
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n2, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n17,) = {
    if (locals.var_guard1227 != 0.0) {
        let eq35_e519: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qbs);
        let eq35_e520: f64 = (locals.var_ibs + eq35_e519);
        let eq35_e520_d_n0: f64 = (locals.var_ibs_dn0 + (locals.var_qbs_dn0 * ddt_scale));
        let eq35_e520_d_n2: f64 = (locals.var_ibs_dn2 + (locals.var_qbs_dn2 * ddt_scale));
        let eq35_e520_d_n6: f64 = (locals.var_ibs_dn6 + (locals.var_qbs_dn6 * ddt_scale));
        let eq35_e520_d_n7: f64 = (locals.var_ibs_dn7 + (locals.var_qbs_dn7 * ddt_scale));
        let eq35_e520_d_n10: f64 = (locals.var_ibs_dn10 + (locals.var_qbs_dn10 * ddt_scale));
        let eq35_e520_d_n11: f64 = (locals.var_ibs_dn11 + (locals.var_qbs_dn11 * ddt_scale));
        let eq35_e520_d_n12: f64 = (locals.var_ibs_dn12 + (locals.var_qbs_dn12 * ddt_scale));
        let eq35_e520_d_n17: f64 = (locals.var_ibs_dn17 + (locals.var_qbs_dn17 * ddt_scale));
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e523;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq35_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq35_e523_d_n0), multiplicity * (eq35_e523_d_n2), multiplicity * (eq35_e523_d_n6), multiplicity * (eq35_e523_d_n7), multiplicity * (eq35_e523_d_n10), multiplicity * (eq35_e523_d_n11), multiplicity * (eq35_e523_d_n12), multiplicity * (eq35_e523_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n2, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n17,) = {
    if (locals.var_guard1227 != 0.0) {
        let eq36_e528: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qbd);
        let eq36_e529: f64 = (locals.var_ibd + eq36_e528);
        let eq36_e529_d_n0: f64 = (locals.var_ibd_dn0 + (locals.var_qbd_dn0 * ddt_scale));
        let eq36_e529_d_n2: f64 = (locals.var_ibd_dn2 + (locals.var_qbd_dn2 * ddt_scale));
        let eq36_e529_d_n6: f64 = (locals.var_ibd_dn6 + (locals.var_qbd_dn6 * ddt_scale));
        let eq36_e529_d_n7: f64 = (locals.var_ibd_dn7 + (locals.var_qbd_dn7 * ddt_scale));
        let eq36_e529_d_n10: f64 = (locals.var_ibd_dn10 + (locals.var_qbd_dn10 * ddt_scale));
        let eq36_e529_d_n11: f64 = (locals.var_ibd_dn11 + (locals.var_qbd_dn11 * ddt_scale));
        let eq36_e529_d_n12: f64 = (locals.var_ibd_dn12 + (locals.var_qbd_dn12 * ddt_scale));
        let eq36_e529_d_n17: f64 = (locals.var_ibd_dn17 + (locals.var_qbd_dn17 * ddt_scale));
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n2, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e532;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq36_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq36_e532_d_n0), multiplicity * (eq36_e532_d_n2), multiplicity * (eq36_e532_d_n6), multiplicity * (eq36_e532_d_n7), multiplicity * (eq36_e532_d_n10), multiplicity * (eq36_e532_d_n11), multiplicity * (eq36_e532_d_n12), multiplicity * (eq36_e532_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq47_e616, eq47_e616_d_n18,) = {
    if ((locals.var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq47_e613);
        (eq47_e614, (eq47_e611 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e616;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq47_value),
            18,
            multiplicity * (eq47_e616_d_n18),
        );
        let (eq48_e627, eq48_e627_d_n13,) = {
    if ((locals.var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq48_e624);
        (eq48_e625, (eq48_e622 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e627;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq48_value),
            13,
            multiplicity * (eq48_e627_d_n13),
        );
        let (eq53_e666, eq53_e666_d_n17,) = {
    if ((locals.var_guard1227 != 0.0) && (locals.var_guard1228 != 0.0)) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq53_e663);
        (eq53_e664, (eq53_e661 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e666;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq53_value),
            17,
            multiplicity * (eq53_e666_d_n17),
        );
        let (eq60_e724, eq60_e724_d_n17,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq60_e721);
        (eq60_e722, (eq60_e719 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e724;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq60_value),
            17,
            multiplicity * (eq60_e724_d_n17),
        );
        let (eq68_e792, eq68_e792_d_n15,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq68_e789);
        (eq68_e790, (eq68_e787 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e792;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq68_value),
            15,
            multiplicity * (eq68_e792_d_n15),
        );
        let (eq69_e804, eq69_e804_d_n16,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq69_e801);
        (eq69_e802, (eq69_e799 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e804;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq69_value),
            16,
            multiplicity * (eq69_e804_d_n16),
        );
        let (eq70_e816, eq70_e816_d_n13,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq70_e813);
        (eq70_e814, (eq70_e811 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e816;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq70_value),
            13,
            multiplicity * (eq70_e816_d_n13),
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
        let eq11_e367_q: f64 = locals.var_qg;
        let eq11_e368: f64 = (p.p50 * locals.var_qg);
        let eq11_e368_d_n0: f64 = (p.p50 * locals.var_qg_dn0);
        let eq11_e368_d_n2: f64 = (p.p50 * locals.var_qg_dn2);
        let eq11_e368_d_n6: f64 = (p.p50 * locals.var_qg_dn6);
        let eq11_e368_d_n7: f64 = (p.p50 * locals.var_qg_dn7);
        let eq11_e368_d_n10: f64 = (p.p50 * locals.var_qg_dn10);
        let eq11_e368_d_n11: f64 = (p.p50 * locals.var_qg_dn11);
        let eq11_e368_d_n12: f64 = (p.p50 * locals.var_qg_dn12);
        let eq11_e368_d_n13: f64 = (p.p50 * locals.var_qg_dn13);
        let eq11_e368_d_n15: f64 = (p.p50 * locals.var_qg_dn15);
        let eq11_e368_d_n16: f64 = (p.p50 * locals.var_qg_dn16);
        let eq11_e368_d_n17: f64 = (p.p50 * locals.var_qg_dn17);
        let eq11_e368_d_n18: f64 = (p.p50 * locals.var_qg_dn18);
        let eq11_e368_q: f64 = (p.p50 * eq11_e367_q);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e368_d_n0, 0.0, eq11_e368_d_n2, 0.0, 0.0, 0.0, eq11_e368_d_n6, eq11_e368_d_n7, 0.0, 0.0, eq11_e368_d_n10, eq11_e368_d_n11, eq11_e368_d_n12, eq11_e368_d_n13, 0.0, eq11_e368_d_n15, eq11_e368_d_n16, eq11_e368_d_n17, eq11_e368_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e371_q: f64 = locals.var_qd;
        let eq12_e372: f64 = (p.p50 * locals.var_qd);
        let eq12_e372_d_n0: f64 = (p.p50 * locals.var_qd_dn0);
        let eq12_e372_d_n2: f64 = (p.p50 * locals.var_qd_dn2);
        let eq12_e372_d_n6: f64 = (p.p50 * locals.var_qd_dn6);
        let eq12_e372_d_n7: f64 = (p.p50 * locals.var_qd_dn7);
        let eq12_e372_d_n10: f64 = (p.p50 * locals.var_qd_dn10);
        let eq12_e372_d_n11: f64 = (p.p50 * locals.var_qd_dn11);
        let eq12_e372_d_n12: f64 = (p.p50 * locals.var_qd_dn12);
        let eq12_e372_d_n13: f64 = (p.p50 * locals.var_qd_dn13);
        let eq12_e372_d_n15: f64 = (p.p50 * locals.var_qd_dn15);
        let eq12_e372_d_n16: f64 = (p.p50 * locals.var_qd_dn16);
        let eq12_e372_d_n17: f64 = (p.p50 * locals.var_qd_dn17);
        let eq12_e372_d_n18: f64 = (p.p50 * locals.var_qd_dn18);
        let eq12_e372_q: f64 = (p.p50 * eq12_e371_q);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e372_d_n0, 0.0, eq12_e372_d_n2, 0.0, 0.0, 0.0, eq12_e372_d_n6, eq12_e372_d_n7, 0.0, 0.0, eq12_e372_d_n10, eq12_e372_d_n11, eq12_e372_d_n12, eq12_e372_d_n13, 0.0, eq12_e372_d_n15, eq12_e372_d_n16, eq12_e372_d_n17, eq12_e372_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e375_q: f64 = locals.var_qb;
        let eq13_e376: f64 = (p.p50 * locals.var_qb);
        let eq13_e376_d_n0: f64 = (p.p50 * locals.var_qb_dn0);
        let eq13_e376_d_n2: f64 = (p.p50 * locals.var_qb_dn2);
        let eq13_e376_d_n6: f64 = (p.p50 * locals.var_qb_dn6);
        let eq13_e376_d_n7: f64 = (p.p50 * locals.var_qb_dn7);
        let eq13_e376_d_n10: f64 = (p.p50 * locals.var_qb_dn10);
        let eq13_e376_d_n11: f64 = (p.p50 * locals.var_qb_dn11);
        let eq13_e376_d_n12: f64 = (p.p50 * locals.var_qb_dn12);
        let eq13_e376_d_n13: f64 = (p.p50 * locals.var_qb_dn13);
        let eq13_e376_d_n15: f64 = (p.p50 * locals.var_qb_dn15);
        let eq13_e376_d_n16: f64 = (p.p50 * locals.var_qb_dn16);
        let eq13_e376_d_n17: f64 = (p.p50 * locals.var_qb_dn17);
        let eq13_e376_d_n18: f64 = (p.p50 * locals.var_qb_dn18);
        let eq13_e376_q: f64 = (p.p50 * eq13_e375_q);
        let eq13_reactive_node_derivatives: [f64; 19] = [eq13_e376_d_n0, 0.0, eq13_e376_d_n2, 0.0, 0.0, 0.0, eq13_e376_d_n6, eq13_e376_d_n7, 0.0, 0.0, eq13_e376_d_n10, eq13_e376_d_n11, eq13_e376_d_n12, eq13_e376_d_n13, 0.0, eq13_e376_d_n15, eq13_e376_d_n16, eq13_e376_d_n17, eq13_e376_d_n18];
        let eq13_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq31_e491, eq31_e491_d_n10, eq31_e491_q,) = {
    if (locals.var_guard1226 != 0.0) {
        let eq31_e488: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq31_e489_q: f64 = eq31_e488;
        (eq31_e488, locals.var_cthe, eq31_e489_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq31_e491_d_n10),
        );
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n2, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n17, eq35_e523_q, eq35_e523_q_d_n0, eq35_e523_q_d_n2, eq35_e523_q_d_n6, eq35_e523_q_d_n7, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, eq35_e523_q_d_n17,) = {
    if (locals.var_guard1227 != 0.0) {
        let eq35_e519_q: f64 = locals.var_qbs;
        let eq35_e520: f64 = (locals.var_ibs + locals.var_qbs);
        let eq35_e520_d_n0: f64 = (locals.var_ibs_dn0 + locals.var_qbs_dn0);
        let eq35_e520_d_n2: f64 = (locals.var_ibs_dn2 + locals.var_qbs_dn2);
        let eq35_e520_d_n6: f64 = (locals.var_ibs_dn6 + locals.var_qbs_dn6);
        let eq35_e520_d_n7: f64 = (locals.var_ibs_dn7 + locals.var_qbs_dn7);
        let eq35_e520_d_n10: f64 = (locals.var_ibs_dn10 + locals.var_qbs_dn10);
        let eq35_e520_d_n11: f64 = (locals.var_ibs_dn11 + locals.var_qbs_dn11);
        let eq35_e520_d_n12: f64 = (locals.var_ibs_dn12 + locals.var_qbs_dn12);
        let eq35_e520_d_n17: f64 = (locals.var_ibs_dn17 + locals.var_qbs_dn17);
        let eq35_e520_q: f64 = eq35_e519_q;
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        let eq35_e521_q: f64 = (p.p50 * eq35_e520_q);
        let eq35_e521_q_d_n0: f64 = (p.p50 * locals.var_qbs_dn0);
        let eq35_e521_q_d_n2: f64 = (p.p50 * locals.var_qbs_dn2);
        let eq35_e521_q_d_n6: f64 = (p.p50 * locals.var_qbs_dn6);
        let eq35_e521_q_d_n7: f64 = (p.p50 * locals.var_qbs_dn7);
        let eq35_e521_q_d_n10: f64 = (p.p50 * locals.var_qbs_dn10);
        let eq35_e521_q_d_n11: f64 = (p.p50 * locals.var_qbs_dn11);
        let eq35_e521_q_d_n12: f64 = (p.p50 * locals.var_qbs_dn12);
        let eq35_e521_q_d_n17: f64 = (p.p50 * locals.var_qbs_dn17);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17, eq35_e521_q, eq35_e521_q_d_n0, eq35_e521_q_d_n2, eq35_e521_q_d_n6, eq35_e521_q_d_n7, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, eq35_e521_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e523_q_d_n0, 0.0, eq35_e523_q_d_n2, 0.0, 0.0, 0.0, eq35_e523_q_d_n6, eq35_e523_q_d_n7, 0.0, 0.0, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq35_e523_q_d_n17, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n2, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n17, eq36_e532_q, eq36_e532_q_d_n0, eq36_e532_q_d_n2, eq36_e532_q_d_n6, eq36_e532_q_d_n7, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, eq36_e532_q_d_n17,) = {
    if (locals.var_guard1227 != 0.0) {
        let eq36_e528_q: f64 = locals.var_qbd;
        let eq36_e529: f64 = (locals.var_ibd + locals.var_qbd);
        let eq36_e529_d_n0: f64 = (locals.var_ibd_dn0 + locals.var_qbd_dn0);
        let eq36_e529_d_n2: f64 = (locals.var_ibd_dn2 + locals.var_qbd_dn2);
        let eq36_e529_d_n6: f64 = (locals.var_ibd_dn6 + locals.var_qbd_dn6);
        let eq36_e529_d_n7: f64 = (locals.var_ibd_dn7 + locals.var_qbd_dn7);
        let eq36_e529_d_n10: f64 = (locals.var_ibd_dn10 + locals.var_qbd_dn10);
        let eq36_e529_d_n11: f64 = (locals.var_ibd_dn11 + locals.var_qbd_dn11);
        let eq36_e529_d_n12: f64 = (locals.var_ibd_dn12 + locals.var_qbd_dn12);
        let eq36_e529_d_n17: f64 = (locals.var_ibd_dn17 + locals.var_qbd_dn17);
        let eq36_e529_q: f64 = eq36_e528_q;
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        let eq36_e530_q: f64 = (p.p50 * eq36_e529_q);
        let eq36_e530_q_d_n0: f64 = (p.p50 * locals.var_qbd_dn0);
        let eq36_e530_q_d_n2: f64 = (p.p50 * locals.var_qbd_dn2);
        let eq36_e530_q_d_n6: f64 = (p.p50 * locals.var_qbd_dn6);
        let eq36_e530_q_d_n7: f64 = (p.p50 * locals.var_qbd_dn7);
        let eq36_e530_q_d_n10: f64 = (p.p50 * locals.var_qbd_dn10);
        let eq36_e530_q_d_n11: f64 = (p.p50 * locals.var_qbd_dn11);
        let eq36_e530_q_d_n12: f64 = (p.p50 * locals.var_qbd_dn12);
        let eq36_e530_q_d_n17: f64 = (p.p50 * locals.var_qbd_dn17);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n2, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n17, eq36_e530_q, eq36_e530_q_d_n0, eq36_e530_q_d_n2, eq36_e530_q_d_n6, eq36_e530_q_d_n7, eq36_e530_q_d_n10, eq36_e530_q_d_n11, eq36_e530_q_d_n12, eq36_e530_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 19] = [eq36_e532_q_d_n0, 0.0, eq36_e532_q_d_n2, 0.0, 0.0, 0.0, eq36_e532_q_d_n6, eq36_e532_q_d_n7, 0.0, 0.0, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq36_e532_q_d_n17, 0.0];
        let eq36_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e616, eq47_e616_d_n18, eq47_e616_q,) = {
    if ((locals.var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614_q: f64 = eq47_e613;
        (eq47_e613, eq47_e611, eq47_e614_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq47_e616_d_n18),
        );
        let (eq48_e627, eq48_e627_d_n13, eq48_e627_q,) = {
    if ((locals.var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625_q: f64 = eq48_e624;
        (eq48_e624, eq48_e622, eq48_e625_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq48_e627_d_n13),
        );
        let (eq53_e666, eq53_e666_d_n17, eq53_e666_q,) = {
    if ((locals.var_guard1227 != 0.0) && (locals.var_guard1228 != 0.0)) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664_q: f64 = eq53_e663;
        (eq53_e663, eq53_e661, eq53_e664_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq53_e666_d_n17),
        );
        let (eq60_e724, eq60_e724_d_n17, eq60_e724_q,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722_q: f64 = eq60_e721;
        (eq60_e721, eq60_e719, eq60_e722_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq60_e724_d_n17),
        );
        let (eq68_e792, eq68_e792_d_n15, eq68_e792_q,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790_q: f64 = eq68_e789;
        (eq68_e789, eq68_e787, eq68_e790_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq68_e792_d_n15),
        );
        let (eq69_e804, eq69_e804_d_n16, eq69_e804_q,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802_q: f64 = eq69_e801;
        (eq69_e801, eq69_e799, eq69_e802_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq69_e804_d_n16),
        );
        let (eq70_e816, eq70_e816_d_n13, eq70_e816_q,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814_q: f64 = eq70_e813;
        (eq70_e813, eq70_e811, eq70_e814_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq70_e816_d_n13),
        );
    }
}

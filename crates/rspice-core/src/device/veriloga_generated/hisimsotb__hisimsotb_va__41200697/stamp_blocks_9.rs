#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_53(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12370_e17104, assign12370_e17104_d_n0, assign12370_e17104_d_n2, assign12370_e17104_d_n4, assign12370_e17104_d_n5, assign12370_e17104_d_n6, assign12370_e17104_d_n8, assign12370_e17104_d_n10, assign12370_e17104_d_n11, assign12370_e17104_d_n12,) = {
    if ((locals.var_guard232 != 0.0) && (locals.var_guard233 == 0.0)) {
        let assign12370_e17102: f64 = (locals.var_fmdvds * locals.var_t1);
        (assign12370_e17102, ((locals.var_fmdvds_dn0 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn0)), ((locals.var_fmdvds_dn2 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn2)), ((locals.var_fmdvds_dn4 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn4)), ((locals.var_fmdvds_dn5 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn5)), ((locals.var_fmdvds_dn6 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn6)), ((locals.var_fmdvds_dn8 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn8)), ((locals.var_fmdvds_dn10 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn10)), ((locals.var_fmdvds_dn11 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn11)), ((locals.var_fmdvds_dn12 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn12)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn8, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12,)
    }
};
        locals.var_lred = assign12370_e17104;
        locals.var_lred_dn0 = assign12370_e17104_d_n0;
        locals.var_lred_dn2 = assign12370_e17104_d_n2;
        locals.var_lred_dn4 = assign12370_e17104_d_n4;
        locals.var_lred_dn5 = assign12370_e17104_d_n5;
        locals.var_lred_dn6 = assign12370_e17104_d_n6;
        locals.var_lred_dn8 = assign12370_e17104_d_n8;
        locals.var_lred_dn10 = assign12370_e17104_d_n10;
        locals.var_lred_dn11 = assign12370_e17104_d_n11;
        locals.var_lred_dn12 = assign12370_e17104_d_n12;
        locals.var_lred_rv = 0.0;

        let (assign12380_e17110, assign12380_e17110_d_n0, assign12380_e17110_d_n2, assign12380_e17110_d_n4, assign12380_e17110_d_n5, assign12380_e17110_d_n6, assign12380_e17110_d_n8, assign12380_e17110_d_n10, assign12380_e17110_d_n11, assign12380_e17110_d_n12,) = {
    if (locals.var_guard232 != 0.0) {
        let assign12380_e17108: f64 = (locals.var_lred * locals.var_clmmod);
        (assign12380_e17108, (locals.var_lred_dn0 * locals.var_clmmod), (locals.var_lred_dn2 * locals.var_clmmod), (locals.var_lred_dn4 * locals.var_clmmod), (locals.var_lred_dn5 * locals.var_clmmod), (locals.var_lred_dn6 * locals.var_clmmod), (locals.var_lred_dn8 * locals.var_clmmod), (locals.var_lred_dn10 * locals.var_clmmod), (locals.var_lred_dn11 * locals.var_clmmod), (locals.var_lred_dn12 * locals.var_clmmod),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn8, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn12,)
    }
};
        locals.var_lred = assign12380_e17110;
        locals.var_lred_dn0 = assign12380_e17110_d_n0;
        locals.var_lred_dn2 = assign12380_e17110_d_n2;
        locals.var_lred_dn4 = assign12380_e17110_d_n4;
        locals.var_lred_dn5 = assign12380_e17110_d_n5;
        locals.var_lred_dn6 = assign12380_e17110_d_n6;
        locals.var_lred_dn8 = assign12380_e17110_d_n8;
        locals.var_lred_dn10 = assign12380_e17110_d_n10;
        locals.var_lred_dn11 = assign12380_e17110_d_n11;
        locals.var_lred_dn12 = assign12380_e17110_d_n12;
        locals.var_lred_rv = 0.0;

        let assign12390_e17113: f64 = (locals.var_leff - locals.var_lred);
        locals.var_lch = assign12390_e17113;
        locals.var_lch_dn0 = (locals.var_leff_dn0 - locals.var_lred_dn0);
        locals.var_lch_dn2 = (locals.var_leff_dn2 - locals.var_lred_dn2);
        locals.var_lch_dn4 = (locals.var_leff_dn4 - locals.var_lred_dn4);
        locals.var_lch_dn5 = (locals.var_leff_dn5 - locals.var_lred_dn5);
        locals.var_lch_dn6 = (locals.var_leff_dn6 - locals.var_lred_dn6);
        locals.var_lch_dn8 = (locals.var_leff_dn8 - locals.var_lred_dn8);
        locals.var_lch_dn10 = (locals.var_leff_dn10 - locals.var_lred_dn10);
        locals.var_lch_dn11 = (locals.var_leff_dn11 - locals.var_lred_dn11);
        locals.var_lch_dn12 = (locals.var_leff_dn12 - locals.var_lred_dn12);
        locals.var_lch_rv = 0.0;

        let assign12400_e17116: f64 = if locals.var_lch < 1e-9 { 1.0 } else { 0.0 };
        locals.var_guard237 = assign12400_e17116;
        locals.var_guard237_rv = 0.0;

        let (assign12410_e17120, assign12410_e17120_d_n0, assign12410_e17120_d_n2, assign12410_e17120_d_n4, assign12410_e17120_d_n5, assign12410_e17120_d_n6, assign12410_e17120_d_n8, assign12410_e17120_d_n10, assign12410_e17120_d_n11, assign12410_e17120_d_n12,) = {
    if (locals.var_guard237 != 0.0) {
        (1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn8, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12,)
    }
};
        locals.var_lch = assign12410_e17120;
        locals.var_lch_dn0 = assign12410_e17120_d_n0;
        locals.var_lch_dn2 = assign12410_e17120_d_n2;
        locals.var_lch_dn4 = assign12410_e17120_d_n4;
        locals.var_lch_dn5 = assign12410_e17120_d_n5;
        locals.var_lch_dn6 = assign12410_e17120_d_n6;
        locals.var_lch_dn8 = assign12410_e17120_d_n8;
        locals.var_lch_dn10 = assign12410_e17120_d_n10;
        locals.var_lch_dn11 = assign12410_e17120_d_n11;
        locals.var_lch_dn12 = assign12410_e17120_d_n12;
        locals.var_lch_rv = 0.0;

        let assign12420_e17122: f64 = (-locals.var_weffcv_nf);
        let assign12420_e17124: f64 = (assign12420_e17122 * locals.var_leff);
        let assign12420_e17127: f64 = (locals.var_qiu + locals.var_qibu);
        let assign12420_e17128: f64 = (assign12420_e17124 * assign12420_e17127);
        locals.var_qi = assign12420_e17128;
        locals.var_qi_dn0 = (((((-locals.var_weffcv_nf_dn0) * locals.var_leff) + (assign12420_e17122 * locals.var_leff_dn0)) * assign12420_e17127) + (assign12420_e17124 * (locals.var_qiu_dn0 + locals.var_qibu_dn0)));
        locals.var_qi_dn2 = (((((-locals.var_weffcv_nf_dn2) * locals.var_leff) + (assign12420_e17122 * locals.var_leff_dn2)) * assign12420_e17127) + (assign12420_e17124 * (locals.var_qiu_dn2 + locals.var_qibu_dn2)));
        locals.var_qi_dn4 = (((((-locals.var_weffcv_nf_dn4) * locals.var_leff) + (assign12420_e17122 * locals.var_leff_dn4)) * assign12420_e17127) + (assign12420_e17124 * (locals.var_qiu_dn4 + locals.var_qibu_dn4)));
        locals.var_qi_dn5 = (((((-locals.var_weffcv_nf_dn5) * locals.var_leff) + (assign12420_e17122 * locals.var_leff_dn5)) * assign12420_e17127) + (assign12420_e17124 * (locals.var_qiu_dn5 + locals.var_qibu_dn5)));
        locals.var_qi_dn6 = (((((-locals.var_weffcv_nf_dn6) * locals.var_leff) + (assign12420_e17122 * locals.var_leff_dn6)) * assign12420_e17127) + (assign12420_e17124 * (locals.var_qiu_dn6 + locals.var_qibu_dn6)));
        locals.var_qi_dn8 = (((((-locals.var_weffcv_nf_dn8) * locals.var_leff) + (assign12420_e17122 * locals.var_leff_dn8)) * assign12420_e17127) + (assign12420_e17124 * (locals.var_qiu_dn8 + locals.var_qibu_dn8)));
        locals.var_qi_dn10 = (((((-locals.var_weffcv_nf_dn10) * locals.var_leff) + (assign12420_e17122 * locals.var_leff_dn10)) * assign12420_e17127) + (assign12420_e17124 * (locals.var_qiu_dn10 + locals.var_qibu_dn10)));
        locals.var_qi_dn11 = (((((-locals.var_weffcv_nf_dn11) * locals.var_leff) + (assign12420_e17122 * locals.var_leff_dn11)) * assign12420_e17127) + (assign12420_e17124 * (locals.var_qiu_dn11 + locals.var_qibu_dn11)));
        locals.var_qi_dn12 = (((((-locals.var_weffcv_nf_dn12) * locals.var_leff) + (assign12420_e17122 * locals.var_leff_dn12)) * assign12420_e17127) + (assign12420_e17124 * (locals.var_qiu_dn12 + locals.var_qibu_dn12)));
        locals.var_qi_rv = 0.0;

        let assign12430_e17132: f64 = (locals.var_q_s0_bulk + locals.var_q_sl_bulk);
        let assign12430_e17133: f64 = (0.5 * assign12430_e17132);
        let assign12430_e17135: f64 = (assign12430_e17133 * locals.var_leff);
        let assign12430_e17137: f64 = (assign12430_e17135 * locals.var_weffcv_nf);
        locals.var_qsub = assign12430_e17137;
        locals.var_qsub_dn0 = (((((0.5 * (locals.var_q_s0_bulk_dn0 + locals.var_q_sl_bulk_dn0)) * locals.var_leff) + (assign12430_e17133 * locals.var_leff_dn0)) * locals.var_weffcv_nf) + (assign12430_e17135 * locals.var_weffcv_nf_dn0));
        locals.var_qsub_dn2 = (((((0.5 * (locals.var_q_s0_bulk_dn2 + locals.var_q_sl_bulk_dn2)) * locals.var_leff) + (assign12430_e17133 * locals.var_leff_dn2)) * locals.var_weffcv_nf) + (assign12430_e17135 * locals.var_weffcv_nf_dn2));
        locals.var_qsub_dn4 = (((((0.5 * (locals.var_q_s0_bulk_dn4 + locals.var_q_sl_bulk_dn4)) * locals.var_leff) + (assign12430_e17133 * locals.var_leff_dn4)) * locals.var_weffcv_nf) + (assign12430_e17135 * locals.var_weffcv_nf_dn4));
        locals.var_qsub_dn5 = (((((0.5 * (locals.var_q_s0_bulk_dn5 + locals.var_q_sl_bulk_dn5)) * locals.var_leff) + (assign12430_e17133 * locals.var_leff_dn5)) * locals.var_weffcv_nf) + (assign12430_e17135 * locals.var_weffcv_nf_dn5));
        locals.var_qsub_dn6 = (((((0.5 * (locals.var_q_s0_bulk_dn6 + locals.var_q_sl_bulk_dn6)) * locals.var_leff) + (assign12430_e17133 * locals.var_leff_dn6)) * locals.var_weffcv_nf) + (assign12430_e17135 * locals.var_weffcv_nf_dn6));
        locals.var_qsub_dn8 = (((((0.5 * (locals.var_q_s0_bulk_dn8 + locals.var_q_sl_bulk_dn8)) * locals.var_leff) + (assign12430_e17133 * locals.var_leff_dn8)) * locals.var_weffcv_nf) + (assign12430_e17135 * locals.var_weffcv_nf_dn8));
        locals.var_qsub_dn10 = (((((0.5 * (locals.var_q_s0_bulk_dn10 + locals.var_q_sl_bulk_dn10)) * locals.var_leff) + (assign12430_e17133 * locals.var_leff_dn10)) * locals.var_weffcv_nf) + (assign12430_e17135 * locals.var_weffcv_nf_dn10));
        locals.var_qsub_dn11 = (((((0.5 * (locals.var_q_s0_bulk_dn11 + locals.var_q_sl_bulk_dn11)) * locals.var_leff) + (assign12430_e17133 * locals.var_leff_dn11)) * locals.var_weffcv_nf) + (assign12430_e17135 * locals.var_weffcv_nf_dn11));
        locals.var_qsub_dn12 = (((((0.5 * (locals.var_q_s0_bulk_dn12 + locals.var_q_sl_bulk_dn12)) * locals.var_leff) + (assign12430_e17133 * locals.var_leff_dn12)) * locals.var_weffcv_nf) + (assign12430_e17135 * locals.var_weffcv_nf_dn12));
        locals.var_qsub_rv = 0.0;

        let assign12440_e17140: f64 = (locals.var_vds - locals.var_pds);
        let assign12440_e17142: f64 = (assign12440_e17140 / 2.0);
        locals.var_t1 = assign12440_e17142;
        locals.var_t1_dn0 = ((locals.var_vds_dn0 - locals.var_pds_dn0) / 2.0);
        locals.var_t1_dn2 = ((locals.var_vds_dn2 - locals.var_pds_dn2) / 2.0);
        locals.var_t1_dn4 = ((locals.var_vds_dn4 - locals.var_pds_dn4) / 2.0);
        locals.var_t1_dn5 = ((locals.var_vds_dn5 - locals.var_pds_dn5) / 2.0);
        locals.var_t1_dn6 = ((locals.var_vds_dn6 - locals.var_pds_dn6) / 2.0);
        locals.var_t1_dn8 = ((locals.var_vds_dn8 - locals.var_pds_dn8) / 2.0);
        locals.var_t1_dn10 = ((locals.var_vds_dn10 - locals.var_pds_dn10) / 2.0);
        locals.var_t1_dn11 = ((locals.var_vds_dn11 - locals.var_pds_dn11) / 2.0);
        locals.var_t1_dn12 = ((locals.var_vds_dn12 - locals.var_pds_dn12) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign12450_e17145: f64 = (2.0 * locals.var_t1);
        let assign12450_e17147: f64 = (assign12450_e17145 / p.p217);
        locals.var_tmf1 = assign12450_e17147;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p217);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p217);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p217);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p217);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p217);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p217);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p217);
        locals.var_tmf1_dn11 = ((2.0 * locals.var_t1_dn11) / p.p217);
        locals.var_tmf1_dn12 = ((2.0 * locals.var_t1_dn12) / p.p217);
        locals.var_tmf1_rv = 0.0;

        let assign12460_e17152: f64 = (1.0 / 2.0);
        let assign12460_e17156: f64 = (1.0 / 6.0);
        let assign12460_e17160: f64 = (1.0 / 24.0);
        let assign12460_e17164: f64 = (1.0 / 120.0);
        let assign12460_e17168: f64 = (1.0 / 720.0);
        let assign12460_e17172: f64 = (1.0 / 5040.0);
        let assign12460_e17173: f64 = (locals.var_tmf1 * assign12460_e17172);
        let assign12460_e17174: f64 = (assign12460_e17168 + assign12460_e17173);
        let assign12460_e17175: f64 = (locals.var_tmf1 * assign12460_e17174);
        let assign12460_e17176: f64 = (assign12460_e17164 + assign12460_e17175);
        let assign12460_e17177: f64 = (locals.var_tmf1 * assign12460_e17176);
        let assign12460_e17178: f64 = (assign12460_e17160 + assign12460_e17177);
        let assign12460_e17179: f64 = (locals.var_tmf1 * assign12460_e17178);
        let assign12460_e17180: f64 = (assign12460_e17156 + assign12460_e17179);
        let assign12460_e17181: f64 = (locals.var_tmf1 * assign12460_e17180);
        let assign12460_e17182: f64 = (assign12460_e17152 + assign12460_e17181);
        let assign12460_e17183: f64 = (locals.var_tmf1 * assign12460_e17182);
        let assign12460_e17184: f64 = (1.0 + assign12460_e17183);
        locals.var_tmf2 = assign12460_e17184;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign12460_e17182) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign12460_e17180) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign12460_e17178) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign12460_e17176) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign12460_e17174) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign12460_e17172)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign12460_e17182) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign12460_e17180) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign12460_e17178) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign12460_e17176) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign12460_e17174) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign12460_e17172)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign12460_e17182) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign12460_e17180) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign12460_e17178) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign12460_e17176) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign12460_e17174) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign12460_e17172)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign12460_e17182) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign12460_e17180) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign12460_e17178) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign12460_e17176) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign12460_e17174) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign12460_e17172)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign12460_e17182) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign12460_e17180) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign12460_e17178) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign12460_e17176) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign12460_e17174) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign12460_e17172)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign12460_e17182) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign12460_e17180) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign12460_e17178) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign12460_e17176) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign12460_e17174) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign12460_e17172)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign12460_e17182) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign12460_e17180) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign12460_e17178) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign12460_e17176) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign12460_e17174) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign12460_e17172)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign12460_e17182) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign12460_e17180) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign12460_e17178) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign12460_e17176) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign12460_e17174) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign12460_e17172)))))))))));
        locals.var_tmf2_dn12 = ((locals.var_tmf1_dn12 * assign12460_e17182) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign12460_e17180) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign12460_e17178) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign12460_e17176) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign12460_e17174) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign12460_e17172)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign12470_e17187: f64 = (1.0 / 2.0);
        let assign12470_e17191: f64 = (1.0 / 3.0);
        let assign12470_e17195: f64 = (1.0 / 8.0);
        let assign12470_e17199: f64 = (1.0 / 30.0);
        let assign12470_e17203: f64 = (1.0 / 144.0);
        let assign12470_e17207: f64 = (1.0 / 840.0);
        let assign12470_e17208: f64 = (locals.var_tmf1 * assign12470_e17207);
        let assign12470_e17209: f64 = (assign12470_e17203 + assign12470_e17208);
        let assign12470_e17210: f64 = (locals.var_tmf1 * assign12470_e17209);
        let assign12470_e17211: f64 = (assign12470_e17199 + assign12470_e17210);
        let assign12470_e17212: f64 = (locals.var_tmf1 * assign12470_e17211);
        let assign12470_e17213: f64 = (assign12470_e17195 + assign12470_e17212);
        let assign12470_e17214: f64 = (locals.var_tmf1 * assign12470_e17213);
        let assign12470_e17215: f64 = (assign12470_e17191 + assign12470_e17214);
        let assign12470_e17216: f64 = (locals.var_tmf1 * assign12470_e17215);
        let assign12470_e17217: f64 = (assign12470_e17187 + assign12470_e17216);
        locals.var_tmf3 = assign12470_e17217;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign12470_e17215) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign12470_e17213) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign12470_e17211) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign12470_e17209) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign12470_e17207)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign12470_e17215) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign12470_e17213) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign12470_e17211) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign12470_e17209) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign12470_e17207)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign12470_e17215) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign12470_e17213) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign12470_e17211) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign12470_e17209) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign12470_e17207)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign12470_e17215) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign12470_e17213) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign12470_e17211) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign12470_e17209) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign12470_e17207)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign12470_e17215) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign12470_e17213) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign12470_e17211) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign12470_e17209) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign12470_e17207)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign12470_e17215) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign12470_e17213) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign12470_e17211) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign12470_e17209) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign12470_e17207)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign12470_e17215) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign12470_e17213) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign12470_e17211) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign12470_e17209) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign12470_e17207)))))))));
        locals.var_tmf3_dn11 = ((locals.var_tmf1_dn11 * assign12470_e17215) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign12470_e17213) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign12470_e17211) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign12470_e17209) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign12470_e17207)))))))));
        locals.var_tmf3_dn12 = ((locals.var_tmf1_dn12 * assign12470_e17215) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign12470_e17213) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign12470_e17211) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign12470_e17209) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign12470_e17207)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign12480_e17220: f64 = (p.p217 / locals.var_tmf2);
        locals.var_pzadd = assign12480_e17220;
        locals.var_pzadd_dn0 = (-((p.p217 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn2 = (-((p.p217 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn4 = (-((p.p217 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn5 = (-((p.p217 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn6 = (-((p.p217 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn8 = (-((p.p217 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn10 = (-((p.p217 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn11 = (-((p.p217 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn12 = (-((p.p217 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_rv = 0.0;

        let assign12490_e17222: f64 = (-2.0);
        let assign12490_e17224: f64 = (assign12490_e17222 * locals.var_tmf3);
        let assign12490_e17227: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign12490_e17228: f64 = (assign12490_e17224 / assign12490_e17227);
        locals.var_t2 = assign12490_e17228;
        locals.var_t2_dn0 = ((((assign12490_e17222 * locals.var_tmf3_dn0) * assign12490_e17227) - (assign12490_e17224 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign12490_e17227 * assign12490_e17227));
        locals.var_t2_dn2 = ((((assign12490_e17222 * locals.var_tmf3_dn2) * assign12490_e17227) - (assign12490_e17224 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign12490_e17227 * assign12490_e17227));
        locals.var_t2_dn4 = ((((assign12490_e17222 * locals.var_tmf3_dn4) * assign12490_e17227) - (assign12490_e17224 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign12490_e17227 * assign12490_e17227));
        locals.var_t2_dn5 = ((((assign12490_e17222 * locals.var_tmf3_dn5) * assign12490_e17227) - (assign12490_e17224 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign12490_e17227 * assign12490_e17227));
        locals.var_t2_dn6 = ((((assign12490_e17222 * locals.var_tmf3_dn6) * assign12490_e17227) - (assign12490_e17224 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign12490_e17227 * assign12490_e17227));
        locals.var_t2_dn8 = ((((assign12490_e17222 * locals.var_tmf3_dn8) * assign12490_e17227) - (assign12490_e17224 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign12490_e17227 * assign12490_e17227));
        locals.var_t2_dn10 = ((((assign12490_e17222 * locals.var_tmf3_dn10) * assign12490_e17227) - (assign12490_e17224 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign12490_e17227 * assign12490_e17227));
        locals.var_t2_dn11 = ((((assign12490_e17222 * locals.var_tmf3_dn11) * assign12490_e17227) - (assign12490_e17224 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign12490_e17227 * assign12490_e17227));
        locals.var_t2_dn12 = ((((assign12490_e17222 * locals.var_tmf3_dn12) * assign12490_e17227) - (assign12490_e17224 * ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)))) / (assign12490_e17227 * assign12490_e17227));
        locals.var_t2_rv = 0.0;

        let assign12500_e17232: f64 = (10.0 * 2.220446049250313e-16);
        let assign12500_e17233: f64 = if locals.var_pzadd < assign12500_e17232 { 1.0 } else { 0.0 };
        locals.var_guard238 = assign12500_e17233;
        locals.var_guard238_rv = 0.0;

        let (assign12510_e17239, assign12510_e17239_d_n0, assign12510_e17239_d_n2, assign12510_e17239_d_n4, assign12510_e17239_d_n5, assign12510_e17239_d_n6, assign12510_e17239_d_n8, assign12510_e17239_d_n10, assign12510_e17239_d_n11, assign12510_e17239_d_n12,) = {
    if (locals.var_guard238 != 0.0) {
        let assign12510_e17237: f64 = (10.0 * 2.220446049250313e-16);
        (assign12510_e17237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn8, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn12,)
    }
};
        locals.var_pzadd = assign12510_e17239;
        locals.var_pzadd_dn0 = assign12510_e17239_d_n0;
        locals.var_pzadd_dn2 = assign12510_e17239_d_n2;
        locals.var_pzadd_dn4 = assign12510_e17239_d_n4;
        locals.var_pzadd_dn5 = assign12510_e17239_d_n5;
        locals.var_pzadd_dn6 = assign12510_e17239_d_n6;
        locals.var_pzadd_dn8 = assign12510_e17239_d_n8;
        locals.var_pzadd_dn10 = assign12510_e17239_d_n10;
        locals.var_pzadd_dn11 = assign12510_e17239_d_n11;
        locals.var_pzadd_dn12 = assign12510_e17239_d_n12;
        locals.var_pzadd_rv = 0.0;

        let assign12520_e17242: f64 = (locals.var_ps0 + locals.var_pzadd);
        locals.var_ps0z = assign12520_e17242;
        locals.var_ps0z_dn0 = (locals.var_ps0_dn0 + locals.var_pzadd_dn0);
        locals.var_ps0z_dn2 = (locals.var_ps0_dn2 + locals.var_pzadd_dn2);
        locals.var_ps0z_dn4 = (locals.var_ps0_dn4 + locals.var_pzadd_dn4);
        locals.var_ps0z_dn5 = (locals.var_ps0_dn5 + locals.var_pzadd_dn5);
        locals.var_ps0z_dn6 = (locals.var_ps0_dn6 + locals.var_pzadd_dn6);
        locals.var_ps0z_dn8 = (locals.var_ps0_dn8 + locals.var_pzadd_dn8);
        locals.var_ps0z_dn10 = (locals.var_ps0_dn10 + locals.var_pzadd_dn10);
        locals.var_ps0z_dn11 = (locals.var_ps0_dn11 + locals.var_pzadd_dn11);
        locals.var_ps0z_dn12 = (locals.var_ps0_dn12 + locals.var_pzadd_dn12);
        locals.var_ps0z_rv = 0.0;

        let assign12530_e17245: f64 = (1.034943e-10 / 100.0);
        locals.var_cgs_esi = assign12530_e17245;
        locals.var_cgs_esi_rv = 0.0;

        let assign12540_e17248: f64 = (locals.var_q_nl / 10000.0);
        locals.var_cgs_q_nl = assign12540_e17248;
        locals.var_cgs_q_nl_dn0 = (locals.var_q_nl_dn0 / 10000.0);
        locals.var_cgs_q_nl_dn2 = (locals.var_q_nl_dn2 / 10000.0);
        locals.var_cgs_q_nl_dn4 = (locals.var_q_nl_dn4 / 10000.0);
        locals.var_cgs_q_nl_dn5 = (locals.var_q_nl_dn5 / 10000.0);
        locals.var_cgs_q_nl_dn6 = (locals.var_q_nl_dn6 / 10000.0);
        locals.var_cgs_q_nl_dn8 = (locals.var_q_nl_dn8 / 10000.0);
        locals.var_cgs_q_nl_dn10 = (locals.var_q_nl_dn10 / 10000.0);
        locals.var_cgs_q_nl_dn11 = (locals.var_q_nl_dn11 / 10000.0);
        locals.var_cgs_q_nl_dn12 = (locals.var_q_nl_dn12 / 10000.0);
        locals.var_cgs_q_nl_rv = 0.0;

        let assign12550_e17251: f64 = (locals.var_q_bl / 10000.0);
        locals.var_cgs_q_bl = assign12550_e17251;
        locals.var_cgs_q_bl_dn0 = (locals.var_q_bl_dn0 / 10000.0);
        locals.var_cgs_q_bl_dn2 = (locals.var_q_bl_dn2 / 10000.0);
        locals.var_cgs_q_bl_dn4 = (locals.var_q_bl_dn4 / 10000.0);
        locals.var_cgs_q_bl_dn5 = (locals.var_q_bl_dn5 / 10000.0);
        locals.var_cgs_q_bl_dn6 = (locals.var_q_bl_dn6 / 10000.0);
        locals.var_cgs_q_bl_dn8 = (locals.var_q_bl_dn8 / 10000.0);
        locals.var_cgs_q_bl_dn10 = (locals.var_q_bl_dn10 / 10000.0);
        locals.var_cgs_q_bl_dn11 = (locals.var_q_bl_dn11 / 10000.0);
        locals.var_cgs_q_bl_dn12 = (locals.var_q_bl_dn12 / 10000.0);
        locals.var_cgs_q_bl_rv = 0.0;

        let assign12560_e17254: f64 = (locals.var_q_b0_dep / 10000.0);
        locals.var_cgs_q_b0_dep = assign12560_e17254;
        locals.var_cgs_q_b0_dep_dn0 = (locals.var_q_b0_dep_dn0 / 10000.0);
        locals.var_cgs_q_b0_dep_dn2 = (locals.var_q_b0_dep_dn2 / 10000.0);
        locals.var_cgs_q_b0_dep_dn4 = (locals.var_q_b0_dep_dn4 / 10000.0);
        locals.var_cgs_q_b0_dep_dn5 = (locals.var_q_b0_dep_dn5 / 10000.0);
        locals.var_cgs_q_b0_dep_dn6 = (locals.var_q_b0_dep_dn6 / 10000.0);
        locals.var_cgs_q_b0_dep_dn8 = (locals.var_q_b0_dep_dn8 / 10000.0);
        locals.var_cgs_q_b0_dep_dn10 = (locals.var_q_b0_dep_dn10 / 10000.0);
        locals.var_cgs_q_b0_dep_dn11 = (locals.var_q_b0_dep_dn11 / 10000.0);
        locals.var_cgs_q_b0_dep_dn12 = (locals.var_q_b0_dep_dn12 / 10000.0);
        locals.var_cgs_q_b0_dep_rv = 0.0;

        let assign12570_e17257: f64 = (locals.var_q_bl_dep / 10000.0);
        locals.var_cgs_q_bl_dep = assign12570_e17257;
        locals.var_cgs_q_bl_dep_dn0 = (locals.var_q_bl_dep_dn0 / 10000.0);
        locals.var_cgs_q_bl_dep_dn2 = (locals.var_q_bl_dep_dn2 / 10000.0);
        locals.var_cgs_q_bl_dep_dn4 = (locals.var_q_bl_dep_dn4 / 10000.0);
        locals.var_cgs_q_bl_dep_dn5 = (locals.var_q_bl_dep_dn5 / 10000.0);
        locals.var_cgs_q_bl_dep_dn6 = (locals.var_q_bl_dep_dn6 / 10000.0);
        locals.var_cgs_q_bl_dep_dn8 = (locals.var_q_bl_dep_dn8 / 10000.0);
        locals.var_cgs_q_bl_dep_dn10 = (locals.var_q_bl_dep_dn10 / 10000.0);
        locals.var_cgs_q_bl_dep_dn11 = (locals.var_q_bl_dep_dn11 / 10000.0);
        locals.var_cgs_q_bl_dep_dn12 = (locals.var_q_bl_dep_dn12 / 10000.0);
        locals.var_cgs_q_bl_dep_rv = 0.0;

        let assign12580_e17260: f64 = (locals.var_qiu / 10000.0);
        locals.var_cgs_qiu = assign12580_e17260;
        locals.var_cgs_qiu_dn0 = (locals.var_qiu_dn0 / 10000.0);
        locals.var_cgs_qiu_dn2 = (locals.var_qiu_dn2 / 10000.0);
        locals.var_cgs_qiu_dn4 = (locals.var_qiu_dn4 / 10000.0);
        locals.var_cgs_qiu_dn5 = (locals.var_qiu_dn5 / 10000.0);
        locals.var_cgs_qiu_dn6 = (locals.var_qiu_dn6 / 10000.0);
        locals.var_cgs_qiu_dn8 = (locals.var_qiu_dn8 / 10000.0);
        locals.var_cgs_qiu_dn10 = (locals.var_qiu_dn10 / 10000.0);
        locals.var_cgs_qiu_dn11 = (locals.var_qiu_dn11 / 10000.0);
        locals.var_cgs_qiu_dn12 = (locals.var_qiu_dn12 / 10000.0);
        locals.var_cgs_qiu_rv = 0.0;

        let assign12590_e17263: f64 = (locals.var_qibu / 10000.0);
        locals.var_cgs_qibu = assign12590_e17263;
        locals.var_cgs_qibu_dn0 = (locals.var_qibu_dn0 / 10000.0);
        locals.var_cgs_qibu_dn2 = (locals.var_qibu_dn2 / 10000.0);
        locals.var_cgs_qibu_dn4 = (locals.var_qibu_dn4 / 10000.0);
        locals.var_cgs_qibu_dn5 = (locals.var_qibu_dn5 / 10000.0);
        locals.var_cgs_qibu_dn6 = (locals.var_qibu_dn6 / 10000.0);
        locals.var_cgs_qibu_dn8 = (locals.var_qibu_dn8 / 10000.0);
        locals.var_cgs_qibu_dn10 = (locals.var_qibu_dn10 / 10000.0);
        locals.var_cgs_qibu_dn11 = (locals.var_qibu_dn11 / 10000.0);
        locals.var_cgs_qibu_dn12 = (locals.var_qibu_dn12 / 10000.0);
        locals.var_cgs_qibu_rv = 0.0;

        let assign12600_e17266: f64 = (locals.var_q_b0_dep / 10000.0);
        locals.var_cgs_q_b0_dep = assign12600_e17266;
        locals.var_cgs_q_b0_dep_dn0 = (locals.var_q_b0_dep_dn0 / 10000.0);
        locals.var_cgs_q_b0_dep_dn2 = (locals.var_q_b0_dep_dn2 / 10000.0);
        locals.var_cgs_q_b0_dep_dn4 = (locals.var_q_b0_dep_dn4 / 10000.0);
        locals.var_cgs_q_b0_dep_dn5 = (locals.var_q_b0_dep_dn5 / 10000.0);
        locals.var_cgs_q_b0_dep_dn6 = (locals.var_q_b0_dep_dn6 / 10000.0);
        locals.var_cgs_q_b0_dep_dn8 = (locals.var_q_b0_dep_dn8 / 10000.0);
        locals.var_cgs_q_b0_dep_dn10 = (locals.var_q_b0_dep_dn10 / 10000.0);
        locals.var_cgs_q_b0_dep_dn11 = (locals.var_q_b0_dep_dn11 / 10000.0);
        locals.var_cgs_q_b0_dep_dn12 = (locals.var_q_b0_dep_dn12 / 10000.0);
        locals.var_cgs_q_b0_dep_rv = 0.0;

        let assign12610_e17269: f64 = (locals.var_q_bl_dep / 10000.0);
        locals.var_cgs_q_bl_dep = assign12610_e17269;
        locals.var_cgs_q_bl_dep_dn0 = (locals.var_q_bl_dep_dn0 / 10000.0);
        locals.var_cgs_q_bl_dep_dn2 = (locals.var_q_bl_dep_dn2 / 10000.0);
        locals.var_cgs_q_bl_dep_dn4 = (locals.var_q_bl_dep_dn4 / 10000.0);
        locals.var_cgs_q_bl_dep_dn5 = (locals.var_q_bl_dep_dn5 / 10000.0);
        locals.var_cgs_q_bl_dep_dn6 = (locals.var_q_bl_dep_dn6 / 10000.0);
        locals.var_cgs_q_bl_dep_dn8 = (locals.var_q_bl_dep_dn8 / 10000.0);
        locals.var_cgs_q_bl_dep_dn10 = (locals.var_q_bl_dep_dn10 / 10000.0);
        locals.var_cgs_q_bl_dep_dn11 = (locals.var_q_bl_dep_dn11 / 10000.0);
        locals.var_cgs_q_bl_dep_dn12 = (locals.var_q_bl_dep_dn12 / 10000.0);
        locals.var_cgs_q_bl_dep_rv = 0.0;

        let assign12620_e17272: f64 = (locals.var_qdepu / 10000.0);
        locals.var_cgs_qdepu = assign12620_e17272;
        locals.var_cgs_qdepu_dn0 = (locals.var_qdepu_dn0 / 10000.0);
        locals.var_cgs_qdepu_dn2 = (locals.var_qdepu_dn2 / 10000.0);
        locals.var_cgs_qdepu_dn4 = (locals.var_qdepu_dn4 / 10000.0);
        locals.var_cgs_qdepu_dn5 = (locals.var_qdepu_dn5 / 10000.0);
        locals.var_cgs_qdepu_dn6 = (locals.var_qdepu_dn6 / 10000.0);
        locals.var_cgs_qdepu_dn8 = (locals.var_qdepu_dn8 / 10000.0);
        locals.var_cgs_qdepu_dn10 = (locals.var_qdepu_dn10 / 10000.0);
        locals.var_cgs_qdepu_dn11 = (locals.var_qdepu_dn11 / 10000.0);
        locals.var_cgs_qdepu_dn12 = (locals.var_qdepu_dn12 / 10000.0);
        locals.var_cgs_qdepu_rv = 0.0;

        let assign12630_e17275: f64 = (p.p229 * 100.0);
        locals.var_cgs_tbox = assign12630_e17275;
        locals.var_cgs_tbox_rv = 0.0;

        let assign12640_e17281: f64 = (locals.var_lg).powf(p.p83);
        let assign12640_e17282: f64 = (p.p82 / assign12640_e17281);
        let assign12640_e17283: f64 = (1.0 + assign12640_e17282);
        let assign12640_e17284: f64 = (p.p81 * assign12640_e17283);
        let assign12640_e17286: f64 = (assign12640_e17284 / locals.var_cgs_esi);
        locals.var_ninv = assign12640_e17286;
        locals.var_ninv_rv = 0.0;

        let assign12650_e17292: f64 = (locals.var_lg).powf(p.p80);
        let assign12650_e17293: f64 = (p.p79 / assign12650_e17292);
        let assign12650_e17294: f64 = (1.0 + assign12650_e17293);
        let assign12650_e17295: f64 = (p.p78 * assign12650_e17294);
        let assign12650_e17297: f64 = (assign12650_e17295 / locals.var_cgs_esi);
        locals.var_ndep = assign12650_e17297;
        locals.var_ndep_rv = 0.0;

        let assign12660_e17300: f64 = (locals.var_pds * locals.var_pds);
        let assign12660_e17303: f64 = (4.0 * 1e-6);
        let assign12660_e17305: f64 = (assign12660_e17303 * 1e-6);
        let assign12660_e17306: f64 = (assign12660_e17300 + assign12660_e17305);
        let assign12660_e17307: f64 = (assign12660_e17306).sqrt();
        locals.var_tmf2 = assign12660_e17307;
        locals.var_tmf2_dn0 = (((locals.var_pds_dn0 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn0)) / (2.0 * assign12660_e17307));
        locals.var_tmf2_dn2 = (((locals.var_pds_dn2 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn2)) / (2.0 * assign12660_e17307));
        locals.var_tmf2_dn4 = (((locals.var_pds_dn4 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn4)) / (2.0 * assign12660_e17307));
        locals.var_tmf2_dn5 = (((locals.var_pds_dn5 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn5)) / (2.0 * assign12660_e17307));
        locals.var_tmf2_dn6 = (((locals.var_pds_dn6 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn6)) / (2.0 * assign12660_e17307));
        locals.var_tmf2_dn8 = (((locals.var_pds_dn8 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn8)) / (2.0 * assign12660_e17307));
        locals.var_tmf2_dn10 = (((locals.var_pds_dn10 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn10)) / (2.0 * assign12660_e17307));
        locals.var_tmf2_dn11 = (((locals.var_pds_dn11 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn11)) / (2.0 * assign12660_e17307));
        locals.var_tmf2_dn12 = (((locals.var_pds_dn12 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn12)) / (2.0 * assign12660_e17307));
        locals.var_tmf2_rv = 0.0;

        let assign12670_e17312: f64 = (locals.var_pds / locals.var_tmf2);
        let assign12670_e17313: f64 = (1.0 + assign12670_e17312);
        let assign12670_e17314: f64 = (0.5 * assign12670_e17313);
        locals.var_t0 = assign12670_e17314;
        locals.var_t0_dn0 = (0.5 * (((locals.var_pds_dn0 * locals.var_tmf2) - (locals.var_pds * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_pds_dn2 * locals.var_tmf2) - (locals.var_pds * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_pds_dn4 * locals.var_tmf2) - (locals.var_pds * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_pds_dn5 * locals.var_tmf2) - (locals.var_pds * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_pds_dn6 * locals.var_tmf2) - (locals.var_pds * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_pds_dn8 * locals.var_tmf2) - (locals.var_pds * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_pds_dn10 * locals.var_tmf2) - (locals.var_pds * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn11 = (0.5 * (((locals.var_pds_dn11 * locals.var_tmf2) - (locals.var_pds * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn12 = (0.5 * (((locals.var_pds_dn12 * locals.var_tmf2) - (locals.var_pds * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_rv = 0.0;

        let assign12680_e17318: f64 = (locals.var_pds + locals.var_tmf2);
        let assign12680_e17319: f64 = (0.5 * assign12680_e17318);
        let assign12680_e17322: f64 = (1e-10 * 1e-6);
        let assign12680_e17323: f64 = (assign12680_e17319 + assign12680_e17322);
        locals.var_pdsz = assign12680_e17323;
        locals.var_pdsz_dn0 = (0.5 * (locals.var_pds_dn0 + locals.var_tmf2_dn0));
        locals.var_pdsz_dn2 = (0.5 * (locals.var_pds_dn2 + locals.var_tmf2_dn2));
        locals.var_pdsz_dn4 = (0.5 * (locals.var_pds_dn4 + locals.var_tmf2_dn4));
        locals.var_pdsz_dn5 = (0.5 * (locals.var_pds_dn5 + locals.var_tmf2_dn5));
        locals.var_pdsz_dn6 = (0.5 * (locals.var_pds_dn6 + locals.var_tmf2_dn6));
        locals.var_pdsz_dn8 = (0.5 * (locals.var_pds_dn8 + locals.var_tmf2_dn8));
        locals.var_pdsz_dn10 = (0.5 * (locals.var_pds_dn10 + locals.var_tmf2_dn10));
        locals.var_pdsz_dn11 = (0.5 * (locals.var_pds_dn11 + locals.var_tmf2_dn11));
        locals.var_pdsz_dn12 = (0.5 * (locals.var_pds_dn12 + locals.var_tmf2_dn12));
        locals.var_pdsz_rv = 0.0;

        let assign12690_e17326: f64 = if locals.var_pdsz < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard239 = assign12690_e17326;
        locals.var_guard239_rv = 0.0;

        let (assign12700_e17330, assign12700_e17330_d_n0, assign12700_e17330_d_n2, assign12700_e17330_d_n4, assign12700_e17330_d_n5, assign12700_e17330_d_n6, assign12700_e17330_d_n8, assign12700_e17330_d_n10, assign12700_e17330_d_n11, assign12700_e17330_d_n12,) = {
    if (locals.var_guard239 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdsz, locals.var_pdsz_dn0, locals.var_pdsz_dn2, locals.var_pdsz_dn4, locals.var_pdsz_dn5, locals.var_pdsz_dn6, locals.var_pdsz_dn8, locals.var_pdsz_dn10, locals.var_pdsz_dn11, locals.var_pdsz_dn12,)
    }
};
        locals.var_pdsz = assign12700_e17330;
        locals.var_pdsz_dn0 = assign12700_e17330_d_n0;
        locals.var_pdsz_dn2 = assign12700_e17330_d_n2;
        locals.var_pdsz_dn4 = assign12700_e17330_d_n4;
        locals.var_pdsz_dn5 = assign12700_e17330_d_n5;
        locals.var_pdsz_dn6 = assign12700_e17330_d_n6;
        locals.var_pdsz_dn8 = assign12700_e17330_d_n8;
        locals.var_pdsz_dn10 = assign12700_e17330_d_n10;
        locals.var_pdsz_dn11 = assign12700_e17330_d_n11;
        locals.var_pdsz_dn12 = assign12700_e17330_d_n12;
        locals.var_pdsz_rv = 0.0;

        let (assign12710_e17334, assign12710_e17334_d_n0, assign12710_e17334_d_n2, assign12710_e17334_d_n4, assign12710_e17334_d_n5, assign12710_e17334_d_n6, assign12710_e17334_d_n8, assign12710_e17334_d_n10, assign12710_e17334_d_n11, assign12710_e17334_d_n12,) = {
    if (locals.var_guard239 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign12710_e17334;
        locals.var_t0_dn0 = assign12710_e17334_d_n0;
        locals.var_t0_dn2 = assign12710_e17334_d_n2;
        locals.var_t0_dn4 = assign12710_e17334_d_n4;
        locals.var_t0_dn5 = assign12710_e17334_d_n5;
        locals.var_t0_dn6 = assign12710_e17334_d_n6;
        locals.var_t0_dn8 = assign12710_e17334_d_n8;
        locals.var_t0_dn10 = assign12710_e17334_d_n10;
        locals.var_t0_dn11 = assign12710_e17334_d_n11;
        locals.var_t0_dn12 = assign12710_e17334_d_n12;
        locals.var_t0_rv = 0.0;

        let assign12720_e17337: f64 = (locals.var_pdsz * locals.var_pdsz);
        let assign12720_e17339: f64 = (assign12720_e17337 + p.p216);
        let assign12720_e17340: f64 = (assign12720_e17339).sqrt();
        let assign12720_e17342: f64 = (p.p216).sqrt();
        let assign12720_e17343: f64 = (assign12720_e17340 - assign12720_e17342);
        locals.var_ninvdpdsz = assign12720_e17343;
        locals.var_ninvdpdsz_dn0 = (((locals.var_pdsz_dn0 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn0)) / (2.0 * assign12720_e17340));
        locals.var_ninvdpdsz_dn2 = (((locals.var_pdsz_dn2 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn2)) / (2.0 * assign12720_e17340));
        locals.var_ninvdpdsz_dn4 = (((locals.var_pdsz_dn4 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn4)) / (2.0 * assign12720_e17340));
        locals.var_ninvdpdsz_dn5 = (((locals.var_pdsz_dn5 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn5)) / (2.0 * assign12720_e17340));
        locals.var_ninvdpdsz_dn6 = (((locals.var_pdsz_dn6 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn6)) / (2.0 * assign12720_e17340));
        locals.var_ninvdpdsz_dn8 = (((locals.var_pdsz_dn8 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn8)) / (2.0 * assign12720_e17340));
        locals.var_ninvdpdsz_dn10 = (((locals.var_pdsz_dn10 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn10)) / (2.0 * assign12720_e17340));
        locals.var_ninvdpdsz_dn11 = (((locals.var_pdsz_dn11 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn11)) / (2.0 * assign12720_e17340));
        locals.var_ninvdpdsz_dn12 = (((locals.var_pdsz_dn12 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn12)) / (2.0 * assign12720_e17340));
        locals.var_ninvdpdsz_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign12730_e17346: f64 = (locals.var_ninvdpdsz).powf(p.p85);
        locals.var_ninvdpdsz = assign12730_e17346;
        locals.var_ninvdpdsz_dn0 = if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn0)) } } else { (assign12730_e17346 * (p.p85 * (locals.var_ninvdpdsz_dn0 / locals.var_ninvdpdsz))) };
        locals.var_ninvdpdsz_dn2 = if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn2)) } } else { (assign12730_e17346 * (p.p85 * (locals.var_ninvdpdsz_dn2 / locals.var_ninvdpdsz))) };
        locals.var_ninvdpdsz_dn4 = if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn4)) } } else { (assign12730_e17346 * (p.p85 * (locals.var_ninvdpdsz_dn4 / locals.var_ninvdpdsz))) };
        locals.var_ninvdpdsz_dn5 = if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn5)) } } else { (assign12730_e17346 * (p.p85 * (locals.var_ninvdpdsz_dn5 / locals.var_ninvdpdsz))) };
        locals.var_ninvdpdsz_dn6 = if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn6)) } } else { (assign12730_e17346 * (p.p85 * (locals.var_ninvdpdsz_dn6 / locals.var_ninvdpdsz))) };
        locals.var_ninvdpdsz_dn8 = if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn8)) } } else { (assign12730_e17346 * (p.p85 * (locals.var_ninvdpdsz_dn8 / locals.var_ninvdpdsz))) };
        locals.var_ninvdpdsz_dn10 = if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn10)) } } else { (assign12730_e17346 * (p.p85 * (locals.var_ninvdpdsz_dn10 / locals.var_ninvdpdsz))) };
        locals.var_ninvdpdsz_dn11 = if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn11)) } } else { (assign12730_e17346 * (p.p85 * (locals.var_ninvdpdsz_dn11 / locals.var_ninvdpdsz))) };
        locals.var_ninvdpdsz_dn12 = if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn12)) } } else { (assign12730_e17346 * (p.p85 * (locals.var_ninvdpdsz_dn12 / locals.var_ninvdpdsz))) };
        locals.var_ninvdpdsz_rv = 0.0;

        let assign12740_e17350: f64 = (locals.var_ninvdpdsz * p.p84);
        let assign12740_e17351: f64 = (1.0 + assign12740_e17350);
        locals.var_t4 = assign12740_e17351;
        locals.var_t4_dn0 = (locals.var_ninvdpdsz_dn0 * p.p84);
        locals.var_t4_dn2 = (locals.var_ninvdpdsz_dn2 * p.p84);
        locals.var_t4_dn4 = (locals.var_ninvdpdsz_dn4 * p.p84);
        locals.var_t4_dn5 = (locals.var_ninvdpdsz_dn5 * p.p84);
        locals.var_t4_dn6 = (locals.var_ninvdpdsz_dn6 * p.p84);
        locals.var_t4_dn8 = (locals.var_ninvdpdsz_dn8 * p.p84);
        locals.var_t4_dn10 = (locals.var_ninvdpdsz_dn10 * p.p84);
        locals.var_t4_dn11 = (locals.var_ninvdpdsz_dn11 * p.p84);
        locals.var_t4_dn12 = (locals.var_ninvdpdsz_dn12 * p.p84);
        locals.var_t4_rv = 0.0;

        let assign12750_e17357: f64 = (locals.var_lg).powf(p.p301);
        let assign12750_e17358: f64 = (p.p300 / assign12750_e17357);
        let assign12750_e17359: f64 = (1.0 + assign12750_e17358);
        let assign12750_e17360: f64 = (p.p299 * assign12750_e17359);
        locals.var_mueqb = assign12750_e17360;
        locals.var_mueqb_rv = 0.0;

        let assign12760_e17364: f64 = (locals.var_mueqb * locals.var_cgs_q_bl);
        let assign12760_e17365: f64 = (locals.var_cgs_qiu - assign12760_e17364);
        locals.var_t10 = assign12760_e17365;
        locals.var_t10_dn0 = (locals.var_cgs_qiu_dn0 - (locals.var_mueqb * locals.var_cgs_q_bl_dn0));
        locals.var_t10_dn2 = (locals.var_cgs_qiu_dn2 - (locals.var_mueqb * locals.var_cgs_q_bl_dn2));
        locals.var_t10_dn4 = (locals.var_cgs_qiu_dn4 - (locals.var_mueqb * locals.var_cgs_q_bl_dn4));
        locals.var_t10_dn5 = (locals.var_cgs_qiu_dn5 - (locals.var_mueqb * locals.var_cgs_q_bl_dn5));
        locals.var_t10_dn6 = (locals.var_cgs_qiu_dn6 - (locals.var_mueqb * locals.var_cgs_q_bl_dn6));
        locals.var_t10_dn8 = (locals.var_cgs_qiu_dn8 - (locals.var_mueqb * locals.var_cgs_q_bl_dn8));
        locals.var_t10_dn10 = (locals.var_cgs_qiu_dn10 - (locals.var_mueqb * locals.var_cgs_q_bl_dn10));
        locals.var_t10_dn11 = (locals.var_cgs_qiu_dn11 - (locals.var_mueqb * locals.var_cgs_q_bl_dn11));
        locals.var_t10_dn12 = (locals.var_cgs_qiu_dn12 - (locals.var_mueqb * locals.var_cgs_q_bl_dn12));
        locals.var_t10_rv = 0.0;

        let assign12770_e17368: f64 = (locals.var_ndep * locals.var_cgs_qdepu);
        let assign12770_e17371: f64 = (locals.var_ninv * locals.var_t10);
        let assign12770_e17372: f64 = (assign12770_e17368 + assign12770_e17371);
        locals.var_t5 = assign12770_e17372;
        locals.var_t5_dn0 = ((locals.var_ndep * locals.var_cgs_qdepu_dn0) + (locals.var_ninv * locals.var_t10_dn0));
        locals.var_t5_dn2 = ((locals.var_ndep * locals.var_cgs_qdepu_dn2) + (locals.var_ninv * locals.var_t10_dn2));
        locals.var_t5_dn4 = ((locals.var_ndep * locals.var_cgs_qdepu_dn4) + (locals.var_ninv * locals.var_t10_dn4));
        locals.var_t5_dn5 = ((locals.var_ndep * locals.var_cgs_qdepu_dn5) + (locals.var_ninv * locals.var_t10_dn5));
        locals.var_t5_dn6 = ((locals.var_ndep * locals.var_cgs_qdepu_dn6) + (locals.var_ninv * locals.var_t10_dn6));
        locals.var_t5_dn8 = ((locals.var_ndep * locals.var_cgs_qdepu_dn8) + (locals.var_ninv * locals.var_t10_dn8));
        locals.var_t5_dn10 = ((locals.var_ndep * locals.var_cgs_qdepu_dn10) + (locals.var_ninv * locals.var_t10_dn10));
        locals.var_t5_dn11 = ((locals.var_ndep * locals.var_cgs_qdepu_dn11) + (locals.var_ninv * locals.var_t10_dn11));
        locals.var_t5_dn12 = ((locals.var_ndep * locals.var_cgs_qdepu_dn12) + (locals.var_ninv * locals.var_t10_dn12));
        locals.var_t5_rv = 0.0;

        let assign12780_e17375: f64 = (locals.var_t5 / locals.var_t4);
        locals.var_eeff = assign12780_e17375;
        locals.var_eeff_dn0 = (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4));
        locals.var_eeff_dn2 = (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4));
        locals.var_eeff_dn4 = (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4));
        locals.var_eeff_dn5 = (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4));
        locals.var_eeff_dn6 = (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4));
        locals.var_eeff_dn8 = (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4));
        locals.var_eeff_dn10 = (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4));
        locals.var_eeff_dn11 = (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4));
        locals.var_eeff_dn12 = (((locals.var_t5_dn12 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4));
        locals.var_eeff_rv = 0.0;

        let (assign12790_e17383, assign12790_e17383_d_n0, assign12790_e17383_d_n2, assign12790_e17383_d_n4, assign12790_e17383_d_n5, assign12790_e17383_d_n6, assign12790_e17383_d_n8, assign12790_e17383_d_n10, assign12790_e17383_d_n11, assign12790_e17383_d_n12,) = {
    if (p.p32 != 0.0) {
        let assign12790_e17379: f64 = (locals.var_phi_b0_soi + locals.var_phi_bl_soi);
        let assign12790_e17381: f64 = (assign12790_e17379 * 0.5);
        (assign12790_e17381, ((locals.var_phi_b0_soi_dn0 + locals.var_phi_bl_soi_dn0) * 0.5), ((locals.var_phi_b0_soi_dn2 + locals.var_phi_bl_soi_dn2) * 0.5), ((locals.var_phi_b0_soi_dn4 + locals.var_phi_bl_soi_dn4) * 0.5), ((locals.var_phi_b0_soi_dn5 + locals.var_phi_bl_soi_dn5) * 0.5), ((locals.var_phi_b0_soi_dn6 + locals.var_phi_bl_soi_dn6) * 0.5), ((locals.var_phi_b0_soi_dn8 + locals.var_phi_bl_soi_dn8) * 0.5), ((locals.var_phi_b0_soi_dn10 + locals.var_phi_bl_soi_dn10) * 0.5), ((locals.var_phi_b0_soi_dn11 + locals.var_phi_bl_soi_dn11) * 0.5), ((locals.var_phi_b0_soi_dn12 + locals.var_phi_bl_soi_dn12) * 0.5),)
    } else {
        (locals.var_pb0ls, locals.var_pb0ls_dn0, locals.var_pb0ls_dn2, locals.var_pb0ls_dn4, locals.var_pb0ls_dn5, locals.var_pb0ls_dn6, locals.var_pb0ls_dn8, locals.var_pb0ls_dn10, locals.var_pb0ls_dn11, locals.var_pb0ls_dn12,)
    }
};
        locals.var_pb0ls = assign12790_e17383;
        locals.var_pb0ls_dn0 = assign12790_e17383_d_n0;
        locals.var_pb0ls_dn2 = assign12790_e17383_d_n2;
        locals.var_pb0ls_dn4 = assign12790_e17383_d_n4;
        locals.var_pb0ls_dn5 = assign12790_e17383_d_n5;
        locals.var_pb0ls_dn6 = assign12790_e17383_d_n6;
        locals.var_pb0ls_dn8 = assign12790_e17383_d_n8;
        locals.var_pb0ls_dn10 = assign12790_e17383_d_n10;
        locals.var_pb0ls_dn11 = assign12790_e17383_d_n11;
        locals.var_pb0ls_dn12 = assign12790_e17383_d_n12;
        locals.var_pb0ls_rv = 0.0;

        let (assign12800_e17391, assign12800_e17391_d_n0, assign12800_e17391_d_n2, assign12800_e17391_d_n4, assign12800_e17391_d_n5, assign12800_e17391_d_n6, assign12800_e17391_d_n8, assign12800_e17391_d_n10, assign12800_e17391_d_n11, assign12800_e17391_d_n12,) = {
    if (p.p32 != 0.0) {
        let assign12800_e17387: f64 = (locals.var_phi_s0_bulk + locals.var_phi_sl_bulk);
        let assign12800_e17389: f64 = (assign12800_e17387 * 0.5);
        (assign12800_e17389, ((locals.var_phi_s0_bulk_dn0 + locals.var_phi_sl_bulk_dn0) * 0.5), ((locals.var_phi_s0_bulk_dn2 + locals.var_phi_sl_bulk_dn2) * 0.5), ((locals.var_phi_s0_bulk_dn4 + locals.var_phi_sl_bulk_dn4) * 0.5), ((locals.var_phi_s0_bulk_dn5 + locals.var_phi_sl_bulk_dn5) * 0.5), ((locals.var_phi_s0_bulk_dn6 + locals.var_phi_sl_bulk_dn6) * 0.5), ((locals.var_phi_s0_bulk_dn8 + locals.var_phi_sl_bulk_dn8) * 0.5), ((locals.var_phi_s0_bulk_dn10 + locals.var_phi_sl_bulk_dn10) * 0.5), ((locals.var_phi_s0_bulk_dn11 + locals.var_phi_sl_bulk_dn11) * 0.5), ((locals.var_phi_s0_bulk_dn12 + locals.var_phi_sl_bulk_dn12) * 0.5),)
    } else {
        (locals.var_ps0lb, locals.var_ps0lb_dn0, locals.var_ps0lb_dn2, locals.var_ps0lb_dn4, locals.var_ps0lb_dn5, locals.var_ps0lb_dn6, locals.var_ps0lb_dn8, locals.var_ps0lb_dn10, locals.var_ps0lb_dn11, locals.var_ps0lb_dn12,)
    }
};
        locals.var_ps0lb = assign12800_e17391;
        locals.var_ps0lb_dn0 = assign12800_e17391_d_n0;
        locals.var_ps0lb_dn2 = assign12800_e17391_d_n2;
        locals.var_ps0lb_dn4 = assign12800_e17391_d_n4;
        locals.var_ps0lb_dn5 = assign12800_e17391_d_n5;
        locals.var_ps0lb_dn6 = assign12800_e17391_d_n6;
        locals.var_ps0lb_dn8 = assign12800_e17391_d_n8;
        locals.var_ps0lb_dn10 = assign12800_e17391_d_n10;
        locals.var_ps0lb_dn11 = assign12800_e17391_d_n11;
        locals.var_ps0lb_dn12 = assign12800_e17391_d_n12;
        locals.var_ps0lb_rv = 0.0;

        let (assign12810_e17405, assign12810_e17405_d_n0, assign12810_e17405_d_n2, assign12810_e17405_d_n4, assign12810_e17405_d_n5, assign12810_e17405_d_n6, assign12810_e17405_d_n8, assign12810_e17405_d_n10, assign12810_e17405_d_n11, assign12810_e17405_d_n12,) = {
    if (p.p32 != 0.0) {
        let assign12810_e17396: f64 = (locals.var_pb0ls - locals.var_ps0lb);
        let assign12810_e17398: f64 = (assign12810_e17396 - locals.var_vbsbiz);
        let assign12810_e17399: f64 = (3.9 * assign12810_e17398);
        let assign12810_e17402: f64 = (11.7 * locals.var_cgs_tbox);
        let assign12810_e17403: f64 = (assign12810_e17399 / assign12810_e17402);
        (assign12810_e17403, ((3.9 * ((locals.var_pb0ls_dn0 - locals.var_ps0lb_dn0) - locals.var_vbsbiz_dn0)) / assign12810_e17402), ((3.9 * ((locals.var_pb0ls_dn2 - locals.var_ps0lb_dn2) - locals.var_vbsbiz_dn2)) / assign12810_e17402), ((3.9 * ((locals.var_pb0ls_dn4 - locals.var_ps0lb_dn4) - locals.var_vbsbiz_dn4)) / assign12810_e17402), ((3.9 * ((locals.var_pb0ls_dn5 - locals.var_ps0lb_dn5) - locals.var_vbsbiz_dn5)) / assign12810_e17402), ((3.9 * ((locals.var_pb0ls_dn6 - locals.var_ps0lb_dn6) - locals.var_vbsbiz_dn6)) / assign12810_e17402), ((3.9 * ((locals.var_pb0ls_dn8 - locals.var_ps0lb_dn8) - locals.var_vbsbiz_dn8)) / assign12810_e17402), ((3.9 * ((locals.var_pb0ls_dn10 - locals.var_ps0lb_dn10) - locals.var_vbsbiz_dn10)) / assign12810_e17402), ((3.9 * ((locals.var_pb0ls_dn11 - locals.var_ps0lb_dn11) - locals.var_vbsbiz_dn11)) / assign12810_e17402), ((3.9 * ((locals.var_pb0ls_dn12 - locals.var_ps0lb_dn12) - locals.var_vbsbiz_dn12)) / assign12810_e17402),)
    } else {
        (locals.var_eeffb, locals.var_eeffb_dn0, locals.var_eeffb_dn2, locals.var_eeffb_dn4, locals.var_eeffb_dn5, locals.var_eeffb_dn6, locals.var_eeffb_dn8, locals.var_eeffb_dn10, locals.var_eeffb_dn11, locals.var_eeffb_dn12,)
    }
};
        locals.var_eeffb = assign12810_e17405;
        locals.var_eeffb_dn0 = assign12810_e17405_d_n0;
        locals.var_eeffb_dn2 = assign12810_e17405_d_n2;
        locals.var_eeffb_dn4 = assign12810_e17405_d_n4;
        locals.var_eeffb_dn5 = assign12810_e17405_d_n5;
        locals.var_eeffb_dn6 = assign12810_e17405_d_n6;
        locals.var_eeffb_dn8 = assign12810_e17405_d_n8;
        locals.var_eeffb_dn10 = assign12810_e17405_d_n10;
        locals.var_eeffb_dn11 = assign12810_e17405_d_n11;
        locals.var_eeffb_dn12 = assign12810_e17405_d_n12;
        locals.var_eeffb_rv = 0.0;

        let (assign12820_e17411, assign12820_e17411_d_n0, assign12820_e17411_d_n2, assign12820_e17411_d_n4, assign12820_e17411_d_n5, assign12820_e17411_d_n6, assign12820_e17411_d_n8, assign12820_e17411_d_n10, assign12820_e17411_d_n11, assign12820_e17411_d_n12,) = {
    if (p.p32 != 0.0) {
        let assign12820_e17409: f64 = (locals.var_eeff + locals.var_eeffb);
        (assign12820_e17409, (locals.var_eeff_dn0 + locals.var_eeffb_dn0), (locals.var_eeff_dn2 + locals.var_eeffb_dn2), (locals.var_eeff_dn4 + locals.var_eeffb_dn4), (locals.var_eeff_dn5 + locals.var_eeffb_dn5), (locals.var_eeff_dn6 + locals.var_eeffb_dn6), (locals.var_eeff_dn8 + locals.var_eeffb_dn8), (locals.var_eeff_dn10 + locals.var_eeffb_dn10), (locals.var_eeff_dn11 + locals.var_eeffb_dn11), (locals.var_eeff_dn12 + locals.var_eeffb_dn12),)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn8, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn12,)
    }
};
        locals.var_eeff = assign12820_e17411;
        locals.var_eeff_dn0 = assign12820_e17411_d_n0;
        locals.var_eeff_dn2 = assign12820_e17411_d_n2;
        locals.var_eeff_dn4 = assign12820_e17411_d_n4;
        locals.var_eeff_dn5 = assign12820_e17411_d_n5;
        locals.var_eeff_dn6 = assign12820_e17411_d_n6;
        locals.var_eeff_dn8 = assign12820_e17411_d_n8;
        locals.var_eeff_dn10 = assign12820_e17411_d_n10;
        locals.var_eeff_dn11 = assign12820_e17411_d_n11;
        locals.var_eeff_dn12 = assign12820_e17411_d_n12;
        locals.var_eeff_rv = 0.0;

        let (assign12830_e17416, assign12830_e17416_d_n0, assign12830_e17416_d_n2, assign12830_e17416_d_n4, assign12830_e17416_d_n5, assign12830_e17416_d_n6, assign12830_e17416_d_n8, assign12830_e17416_d_n10, assign12830_e17416_d_n11, assign12830_e17416_d_n12,) = {
    if (p.p32 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb0ls, locals.var_pb0ls_dn0, locals.var_pb0ls_dn2, locals.var_pb0ls_dn4, locals.var_pb0ls_dn5, locals.var_pb0ls_dn6, locals.var_pb0ls_dn8, locals.var_pb0ls_dn10, locals.var_pb0ls_dn11, locals.var_pb0ls_dn12,)
    }
};
        locals.var_pb0ls = assign12830_e17416;
        locals.var_pb0ls_dn0 = assign12830_e17416_d_n0;
        locals.var_pb0ls_dn2 = assign12830_e17416_d_n2;
        locals.var_pb0ls_dn4 = assign12830_e17416_d_n4;
        locals.var_pb0ls_dn5 = assign12830_e17416_d_n5;
        locals.var_pb0ls_dn6 = assign12830_e17416_d_n6;
        locals.var_pb0ls_dn8 = assign12830_e17416_d_n8;
        locals.var_pb0ls_dn10 = assign12830_e17416_d_n10;
        locals.var_pb0ls_dn11 = assign12830_e17416_d_n11;
        locals.var_pb0ls_dn12 = assign12830_e17416_d_n12;
        locals.var_pb0ls_rv = 0.0;

        let (assign12840_e17421, assign12840_e17421_d_n0, assign12840_e17421_d_n2, assign12840_e17421_d_n4, assign12840_e17421_d_n5, assign12840_e17421_d_n6, assign12840_e17421_d_n8, assign12840_e17421_d_n10, assign12840_e17421_d_n11, assign12840_e17421_d_n12,) = {
    if (p.p32 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0lb, locals.var_ps0lb_dn0, locals.var_ps0lb_dn2, locals.var_ps0lb_dn4, locals.var_ps0lb_dn5, locals.var_ps0lb_dn6, locals.var_ps0lb_dn8, locals.var_ps0lb_dn10, locals.var_ps0lb_dn11, locals.var_ps0lb_dn12,)
    }
};
        locals.var_ps0lb = assign12840_e17421;
        locals.var_ps0lb_dn0 = assign12840_e17421_d_n0;
        locals.var_ps0lb_dn2 = assign12840_e17421_d_n2;
        locals.var_ps0lb_dn4 = assign12840_e17421_d_n4;
        locals.var_ps0lb_dn5 = assign12840_e17421_d_n5;
        locals.var_ps0lb_dn6 = assign12840_e17421_d_n6;
        locals.var_ps0lb_dn8 = assign12840_e17421_d_n8;
        locals.var_ps0lb_dn10 = assign12840_e17421_d_n10;
        locals.var_ps0lb_dn11 = assign12840_e17421_d_n11;
        locals.var_ps0lb_dn12 = assign12840_e17421_d_n12;
        locals.var_ps0lb_rv = 0.0;

        let (assign12850_e17426, assign12850_e17426_d_n0, assign12850_e17426_d_n2, assign12850_e17426_d_n4, assign12850_e17426_d_n5, assign12850_e17426_d_n6, assign12850_e17426_d_n8, assign12850_e17426_d_n10, assign12850_e17426_d_n11, assign12850_e17426_d_n12,) = {
    if (p.p32 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eeffb, locals.var_eeffb_dn0, locals.var_eeffb_dn2, locals.var_eeffb_dn4, locals.var_eeffb_dn5, locals.var_eeffb_dn6, locals.var_eeffb_dn8, locals.var_eeffb_dn10, locals.var_eeffb_dn11, locals.var_eeffb_dn12,)
    }
};
        locals.var_eeffb = assign12850_e17426;
        locals.var_eeffb_dn0 = assign12850_e17426_d_n0;
        locals.var_eeffb_dn2 = assign12850_e17426_d_n2;
        locals.var_eeffb_dn4 = assign12850_e17426_d_n4;
        locals.var_eeffb_dn5 = assign12850_e17426_d_n5;
        locals.var_eeffb_dn6 = assign12850_e17426_d_n6;
        locals.var_eeffb_dn8 = assign12850_e17426_d_n8;
        locals.var_eeffb_dn10 = assign12850_e17426_d_n10;
        locals.var_eeffb_dn11 = assign12850_e17426_d_n11;
        locals.var_eeffb_dn12 = assign12850_e17426_d_n12;
        locals.var_eeffb_rv = 0.0;

        let assign12860_e17429: f64 = (locals.var_eeff * locals.var_eeff);
        let assign12860_e17432: f64 = (4.0 * 3000.0);
        let assign12860_e17434: f64 = (assign12860_e17432 * 3000.0);
        let assign12860_e17435: f64 = (assign12860_e17429 + assign12860_e17434);
        let assign12860_e17436: f64 = (assign12860_e17435).sqrt();
        locals.var_tmf2 = assign12860_e17436;
        locals.var_tmf2_dn0 = (((locals.var_eeff_dn0 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn0)) / (2.0 * assign12860_e17436));
        locals.var_tmf2_dn2 = (((locals.var_eeff_dn2 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn2)) / (2.0 * assign12860_e17436));
        locals.var_tmf2_dn4 = (((locals.var_eeff_dn4 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn4)) / (2.0 * assign12860_e17436));
        locals.var_tmf2_dn5 = (((locals.var_eeff_dn5 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn5)) / (2.0 * assign12860_e17436));
        locals.var_tmf2_dn6 = (((locals.var_eeff_dn6 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn6)) / (2.0 * assign12860_e17436));
        locals.var_tmf2_dn8 = (((locals.var_eeff_dn8 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn8)) / (2.0 * assign12860_e17436));
        locals.var_tmf2_dn10 = (((locals.var_eeff_dn10 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn10)) / (2.0 * assign12860_e17436));
        locals.var_tmf2_dn11 = (((locals.var_eeff_dn11 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn11)) / (2.0 * assign12860_e17436));
        locals.var_tmf2_dn12 = (((locals.var_eeff_dn12 * locals.var_eeff) + (locals.var_eeff * locals.var_eeff_dn12)) / (2.0 * assign12860_e17436));
        locals.var_tmf2_rv = 0.0;

        let assign12870_e17441: f64 = (locals.var_eeff / locals.var_tmf2);
        let assign12870_e17442: f64 = (1.0 + assign12870_e17441);
        let assign12870_e17443: f64 = (0.5 * assign12870_e17442);
        locals.var_t1 = assign12870_e17443;
        locals.var_t1_dn0 = (0.5 * (((locals.var_eeff_dn0 * locals.var_tmf2) - (locals.var_eeff * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn2 = (0.5 * (((locals.var_eeff_dn2 * locals.var_tmf2) - (locals.var_eeff * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn4 = (0.5 * (((locals.var_eeff_dn4 * locals.var_tmf2) - (locals.var_eeff * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn5 = (0.5 * (((locals.var_eeff_dn5 * locals.var_tmf2) - (locals.var_eeff * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn6 = (0.5 * (((locals.var_eeff_dn6 * locals.var_tmf2) - (locals.var_eeff * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn8 = (0.5 * (((locals.var_eeff_dn8 * locals.var_tmf2) - (locals.var_eeff * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn10 = (0.5 * (((locals.var_eeff_dn10 * locals.var_tmf2) - (locals.var_eeff * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn11 = (0.5 * (((locals.var_eeff_dn11 * locals.var_tmf2) - (locals.var_eeff * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn12 = (0.5 * (((locals.var_eeff_dn12 * locals.var_tmf2) - (locals.var_eeff * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_rv = 0.0;

        let assign12880_e17447: f64 = (locals.var_eeff + locals.var_tmf2);
        let assign12880_e17448: f64 = (0.5 * assign12880_e17447);
        let assign12880_e17451: f64 = (1e-10 * 3000.0);
        let assign12880_e17452: f64 = (assign12880_e17448 + assign12880_e17451);
        locals.var_eeff = assign12880_e17452;
        locals.var_eeff_dn0 = (0.5 * (locals.var_eeff_dn0 + locals.var_tmf2_dn0));
        locals.var_eeff_dn2 = (0.5 * (locals.var_eeff_dn2 + locals.var_tmf2_dn2));
        locals.var_eeff_dn4 = (0.5 * (locals.var_eeff_dn4 + locals.var_tmf2_dn4));
        locals.var_eeff_dn5 = (0.5 * (locals.var_eeff_dn5 + locals.var_tmf2_dn5));
        locals.var_eeff_dn6 = (0.5 * (locals.var_eeff_dn6 + locals.var_tmf2_dn6));
        locals.var_eeff_dn8 = (0.5 * (locals.var_eeff_dn8 + locals.var_tmf2_dn8));
        locals.var_eeff_dn10 = (0.5 * (locals.var_eeff_dn10 + locals.var_tmf2_dn10));
        locals.var_eeff_dn11 = (0.5 * (locals.var_eeff_dn11 + locals.var_tmf2_dn11));
        locals.var_eeff_dn12 = (0.5 * (locals.var_eeff_dn12 + locals.var_tmf2_dn12));
        locals.var_eeff_rv = 0.0;

        let assign12890_e17455: f64 = if locals.var_eeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign12890_e17455;
        locals.var_guard240_rv = 0.0;

        let (assign12900_e17459, assign12900_e17459_d_n0, assign12900_e17459_d_n2, assign12900_e17459_d_n4, assign12900_e17459_d_n5, assign12900_e17459_d_n6, assign12900_e17459_d_n8, assign12900_e17459_d_n10, assign12900_e17459_d_n11, assign12900_e17459_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn8, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn12,)
    }
};
        locals.var_eeff = assign12900_e17459;
        locals.var_eeff_dn0 = assign12900_e17459_d_n0;
        locals.var_eeff_dn2 = assign12900_e17459_d_n2;
        locals.var_eeff_dn4 = assign12900_e17459_d_n4;
        locals.var_eeff_dn5 = assign12900_e17459_d_n5;
        locals.var_eeff_dn6 = assign12900_e17459_d_n6;
        locals.var_eeff_dn8 = assign12900_e17459_d_n8;
        locals.var_eeff_dn10 = assign12900_e17459_d_n10;
        locals.var_eeff_dn11 = assign12900_e17459_d_n11;
        locals.var_eeff_dn12 = assign12900_e17459_d_n12;
        locals.var_eeff_rv = 0.0;

        let (assign12910_e17463, assign12910_e17463_d_n0, assign12910_e17463_d_n2, assign12910_e17463_d_n4, assign12910_e17463_d_n5, assign12910_e17463_d_n6, assign12910_e17463_d_n8, assign12910_e17463_d_n10, assign12910_e17463_d_n11, assign12910_e17463_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign12910_e17463;
        locals.var_t1_dn0 = assign12910_e17463_d_n0;
        locals.var_t1_dn2 = assign12910_e17463_d_n2;
        locals.var_t1_dn4 = assign12910_e17463_d_n4;
        locals.var_t1_dn5 = assign12910_e17463_d_n5;
        locals.var_t1_dn6 = assign12910_e17463_d_n6;
        locals.var_t1_dn8 = assign12910_e17463_d_n8;
        locals.var_t1_dn10 = assign12910_e17463_d_n10;
        locals.var_t1_dn11 = assign12910_e17463_d_n11;
        locals.var_t1_dn12 = assign12910_e17463_d_n12;
        locals.var_t1_rv = 0.0;

        let assign12920_e17466: f64 = (locals.var_eeff).powf(p.p94);
        locals.var_t8 = assign12920_e17466;
        locals.var_t8_dn0 = if 0.0 == 0.0 && ((p.p94) as f64).is_finite() && ((p.p94) as f64).fract() == 0.0 { if p.p94 == 0.0 { 0.0 } else { (p.p94 * ((locals.var_eeff).powf(p.p94 - 1.0) * locals.var_eeff_dn0)) } } else { (assign12920_e17466 * (p.p94 * (locals.var_eeff_dn0 / locals.var_eeff))) };
        locals.var_t8_dn2 = if 0.0 == 0.0 && ((p.p94) as f64).is_finite() && ((p.p94) as f64).fract() == 0.0 { if p.p94 == 0.0 { 0.0 } else { (p.p94 * ((locals.var_eeff).powf(p.p94 - 1.0) * locals.var_eeff_dn2)) } } else { (assign12920_e17466 * (p.p94 * (locals.var_eeff_dn2 / locals.var_eeff))) };
        locals.var_t8_dn4 = if 0.0 == 0.0 && ((p.p94) as f64).is_finite() && ((p.p94) as f64).fract() == 0.0 { if p.p94 == 0.0 { 0.0 } else { (p.p94 * ((locals.var_eeff).powf(p.p94 - 1.0) * locals.var_eeff_dn4)) } } else { (assign12920_e17466 * (p.p94 * (locals.var_eeff_dn4 / locals.var_eeff))) };
        locals.var_t8_dn5 = if 0.0 == 0.0 && ((p.p94) as f64).is_finite() && ((p.p94) as f64).fract() == 0.0 { if p.p94 == 0.0 { 0.0 } else { (p.p94 * ((locals.var_eeff).powf(p.p94 - 1.0) * locals.var_eeff_dn5)) } } else { (assign12920_e17466 * (p.p94 * (locals.var_eeff_dn5 / locals.var_eeff))) };
        locals.var_t8_dn6 = if 0.0 == 0.0 && ((p.p94) as f64).is_finite() && ((p.p94) as f64).fract() == 0.0 { if p.p94 == 0.0 { 0.0 } else { (p.p94 * ((locals.var_eeff).powf(p.p94 - 1.0) * locals.var_eeff_dn6)) } } else { (assign12920_e17466 * (p.p94 * (locals.var_eeff_dn6 / locals.var_eeff))) };
        locals.var_t8_dn8 = if 0.0 == 0.0 && ((p.p94) as f64).is_finite() && ((p.p94) as f64).fract() == 0.0 { if p.p94 == 0.0 { 0.0 } else { (p.p94 * ((locals.var_eeff).powf(p.p94 - 1.0) * locals.var_eeff_dn8)) } } else { (assign12920_e17466 * (p.p94 * (locals.var_eeff_dn8 / locals.var_eeff))) };
        locals.var_t8_dn10 = if 0.0 == 0.0 && ((p.p94) as f64).is_finite() && ((p.p94) as f64).fract() == 0.0 { if p.p94 == 0.0 { 0.0 } else { (p.p94 * ((locals.var_eeff).powf(p.p94 - 1.0) * locals.var_eeff_dn10)) } } else { (assign12920_e17466 * (p.p94 * (locals.var_eeff_dn10 / locals.var_eeff))) };
        locals.var_t8_dn11 = if 0.0 == 0.0 && ((p.p94) as f64).is_finite() && ((p.p94) as f64).fract() == 0.0 { if p.p94 == 0.0 { 0.0 } else { (p.p94 * ((locals.var_eeff).powf(p.p94 - 1.0) * locals.var_eeff_dn11)) } } else { (assign12920_e17466 * (p.p94 * (locals.var_eeff_dn11 / locals.var_eeff))) };
        locals.var_t8_dn12 = if 0.0 == 0.0 && ((p.p94) as f64).is_finite() && ((p.p94) as f64).fract() == 0.0 { if p.p94 == 0.0 { 0.0 } else { (p.p94 * ((locals.var_eeff).powf(p.p94 - 1.0) * locals.var_eeff_dn12)) } } else { (assign12920_e17466 * (p.p94 * (locals.var_eeff_dn12 / locals.var_eeff))) };
        locals.var_t8_rv = 0.0;

        let assign12930_e17469: f64 = (locals.var_eeff).powf(locals.var_muesr);
        locals.var_t6 = assign12930_e17469;
        locals.var_t6_dn0 = if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn0)) } } else { (assign12930_e17469 * (locals.var_muesr * (locals.var_eeff_dn0 / locals.var_eeff))) };
        locals.var_t6_dn2 = if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn2)) } } else { (assign12930_e17469 * (locals.var_muesr * (locals.var_eeff_dn2 / locals.var_eeff))) };
        locals.var_t6_dn4 = if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn4)) } } else { (assign12930_e17469 * (locals.var_muesr * (locals.var_eeff_dn4 / locals.var_eeff))) };
        locals.var_t6_dn5 = if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn5)) } } else { (assign12930_e17469 * (locals.var_muesr * (locals.var_eeff_dn5 / locals.var_eeff))) };
        locals.var_t6_dn6 = if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn6)) } } else { (assign12930_e17469 * (locals.var_muesr * (locals.var_eeff_dn6 / locals.var_eeff))) };
        locals.var_t6_dn8 = if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn8)) } } else { (assign12930_e17469 * (locals.var_muesr * (locals.var_eeff_dn8 / locals.var_eeff))) };
        locals.var_t6_dn10 = if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn10)) } } else { (assign12930_e17469 * (locals.var_muesr * (locals.var_eeff_dn10 / locals.var_eeff))) };
        locals.var_t6_dn11 = if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn11)) } } else { (assign12930_e17469 * (locals.var_muesr * (locals.var_eeff_dn11 / locals.var_eeff))) };
        locals.var_t6_dn12 = if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn12)) } } else { (assign12930_e17469 * (locals.var_muesr * (locals.var_eeff_dn12 / locals.var_eeff))) };
        locals.var_t6_rv = 0.0;

        let assign12940_e17472: f64 = (locals.var_cgs_qiu / 1.6021918e-19);
        locals.var_rns = assign12940_e17472;
        locals.var_rns_dn0 = (locals.var_cgs_qiu_dn0 / 1.6021918e-19);
        locals.var_rns_dn2 = (locals.var_cgs_qiu_dn2 / 1.6021918e-19);
        locals.var_rns_dn4 = (locals.var_cgs_qiu_dn4 / 1.6021918e-19);
        locals.var_rns_dn5 = (locals.var_cgs_qiu_dn5 / 1.6021918e-19);
        locals.var_rns_dn6 = (locals.var_cgs_qiu_dn6 / 1.6021918e-19);
        locals.var_rns_dn8 = (locals.var_cgs_qiu_dn8 / 1.6021918e-19);
        locals.var_rns_dn10 = (locals.var_cgs_qiu_dn10 / 1.6021918e-19);
        locals.var_rns_dn11 = (locals.var_cgs_qiu_dn11 / 1.6021918e-19);
        locals.var_rns_dn12 = (locals.var_cgs_qiu_dn12 / 1.6021918e-19);
        locals.var_rns_rv = 0.0;

        let assign12950_e17477: f64 = (locals.var_uc_muecb1 * locals.var_rns);
        let assign12950_e17479: f64 = (assign12950_e17477 / 100000000000.0);
        let assign12950_e17480: f64 = (locals.var_uc_muecb0 + assign12950_e17479);
        let assign12950_e17481: f64 = (1.0 / assign12950_e17480);
        let assign12950_e17484: f64 = (locals.var_cgs_mphn0 * locals.var_t8);
        let assign12950_e17485: f64 = (assign12950_e17481 + assign12950_e17484);
        let assign12950_e17488: f64 = (locals.var_t6 / p.p105);
        let assign12950_e17489: f64 = (assign12950_e17485 + assign12950_e17488);
        locals.var_t1 = assign12950_e17489;
        locals.var_t1_dn0 = (((-(((locals.var_uc_muecb1 * locals.var_rns_dn0) / 100000000000.0) / (assign12950_e17480 * assign12950_e17480))) + ((locals.var_cgs_mphn0_dn0 * locals.var_t8) + (locals.var_cgs_mphn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / p.p105));
        locals.var_t1_dn2 = (((-(((locals.var_uc_muecb1 * locals.var_rns_dn2) / 100000000000.0) / (assign12950_e17480 * assign12950_e17480))) + ((locals.var_cgs_mphn0_dn2 * locals.var_t8) + (locals.var_cgs_mphn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / p.p105));
        locals.var_t1_dn4 = (((-(((locals.var_uc_muecb1 * locals.var_rns_dn4) / 100000000000.0) / (assign12950_e17480 * assign12950_e17480))) + ((locals.var_cgs_mphn0_dn4 * locals.var_t8) + (locals.var_cgs_mphn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / p.p105));
        locals.var_t1_dn5 = (((-(((locals.var_uc_muecb1 * locals.var_rns_dn5) / 100000000000.0) / (assign12950_e17480 * assign12950_e17480))) + ((locals.var_cgs_mphn0_dn5 * locals.var_t8) + (locals.var_cgs_mphn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / p.p105));
        locals.var_t1_dn6 = (((-(((locals.var_uc_muecb1 * locals.var_rns_dn6) / 100000000000.0) / (assign12950_e17480 * assign12950_e17480))) + ((locals.var_cgs_mphn0_dn6 * locals.var_t8) + (locals.var_cgs_mphn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / p.p105));
        locals.var_t1_dn8 = (((-(((locals.var_uc_muecb1 * locals.var_rns_dn8) / 100000000000.0) / (assign12950_e17480 * assign12950_e17480))) + ((locals.var_cgs_mphn0_dn8 * locals.var_t8) + (locals.var_cgs_mphn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / p.p105));
        locals.var_t1_dn10 = (((-(((locals.var_uc_muecb1 * locals.var_rns_dn10) / 100000000000.0) / (assign12950_e17480 * assign12950_e17480))) + ((locals.var_cgs_mphn0_dn10 * locals.var_t8) + (locals.var_cgs_mphn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / p.p105));
        locals.var_t1_dn11 = (((-(((locals.var_uc_muecb1 * locals.var_rns_dn11) / 100000000000.0) / (assign12950_e17480 * assign12950_e17480))) + ((locals.var_cgs_mphn0_dn11 * locals.var_t8) + (locals.var_cgs_mphn0 * locals.var_t8_dn11))) + (locals.var_t6_dn11 / p.p105));
        locals.var_t1_dn12 = (((-(((locals.var_uc_muecb1 * locals.var_rns_dn12) / 100000000000.0) / (assign12950_e17480 * assign12950_e17480))) + ((locals.var_cgs_mphn0_dn12 * locals.var_t8) + (locals.var_cgs_mphn0 * locals.var_t8_dn12))) + (locals.var_t6_dn12 / p.p105));
        locals.var_t1_rv = 0.0;

        let assign12960_e17492: f64 = (1.0 / locals.var_t1);
        locals.var_muun = assign12960_e17492;
        locals.var_muun_dn0 = (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1)));
        locals.var_muun_dn2 = (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1)));
        locals.var_muun_dn4 = (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1)));
        locals.var_muun_dn5 = (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1)));
        locals.var_muun_dn6 = (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1)));
        locals.var_muun_dn8 = (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1)));
        locals.var_muun_dn10 = (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1)));
        locals.var_muun_dn11 = (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1)));
        locals.var_muun_dn12 = (-(locals.var_t1_dn12 / (locals.var_t1 * locals.var_t1)));
        locals.var_muun_rv = 0.0;

        let assign12970_e17495: f64 = (locals.var_muun * 0.0001);
        locals.var_muun = assign12970_e17495;
        locals.var_muun_dn0 = (locals.var_muun_dn0 * 0.0001);
        locals.var_muun_dn2 = (locals.var_muun_dn2 * 0.0001);
        locals.var_muun_dn4 = (locals.var_muun_dn4 * 0.0001);
        locals.var_muun_dn5 = (locals.var_muun_dn5 * 0.0001);
        locals.var_muun_dn6 = (locals.var_muun_dn6 * 0.0001);
        locals.var_muun_dn8 = (locals.var_muun_dn8 * 0.0001);
        locals.var_muun_dn10 = (locals.var_muun_dn10 * 0.0001);
        locals.var_muun_dn11 = (locals.var_muun_dn11 * 0.0001);
        locals.var_muun_dn12 = (locals.var_muun_dn12 * 0.0001);
        locals.var_muun_rv = 0.0;

        let (assign12980_e17507, assign12980_e17507_d_n0, assign12980_e17507_d_n2, assign12980_e17507_d_n4, assign12980_e17507_d_n5, assign12980_e17507_d_n6, assign12980_e17507_d_n8, assign12980_e17507_d_n10, assign12980_e17507_d_n11, assign12980_e17507_d_n12,) = {
    if (p.p32 != 0.0) {
        let assign12980_e17500: f64 = (locals.var_pb0ls - locals.var_ps0lb);
        let assign12980_e17501: f64 = (3.9 * assign12980_e17500);
        let assign12980_e17504: f64 = (11.7 * locals.var_cgs_tbox);
        let assign12980_e17505: f64 = (assign12980_e17501 / assign12980_e17504);
        (assign12980_e17505, ((3.9 * (locals.var_pb0ls_dn0 - locals.var_ps0lb_dn0)) / assign12980_e17504), ((3.9 * (locals.var_pb0ls_dn2 - locals.var_ps0lb_dn2)) / assign12980_e17504), ((3.9 * (locals.var_pb0ls_dn4 - locals.var_ps0lb_dn4)) / assign12980_e17504), ((3.9 * (locals.var_pb0ls_dn5 - locals.var_ps0lb_dn5)) / assign12980_e17504), ((3.9 * (locals.var_pb0ls_dn6 - locals.var_ps0lb_dn6)) / assign12980_e17504), ((3.9 * (locals.var_pb0ls_dn8 - locals.var_ps0lb_dn8)) / assign12980_e17504), ((3.9 * (locals.var_pb0ls_dn10 - locals.var_ps0lb_dn10)) / assign12980_e17504), ((3.9 * (locals.var_pb0ls_dn11 - locals.var_ps0lb_dn11)) / assign12980_e17504), ((3.9 * (locals.var_pb0ls_dn12 - locals.var_ps0lb_dn12)) / assign12980_e17504),)
    } else {
        (locals.var_eeffb, locals.var_eeffb_dn0, locals.var_eeffb_dn2, locals.var_eeffb_dn4, locals.var_eeffb_dn5, locals.var_eeffb_dn6, locals.var_eeffb_dn8, locals.var_eeffb_dn10, locals.var_eeffb_dn11, locals.var_eeffb_dn12,)
    }
};
        locals.var_eeffb = assign12980_e17507;
        locals.var_eeffb_dn0 = assign12980_e17507_d_n0;
        locals.var_eeffb_dn2 = assign12980_e17507_d_n2;
        locals.var_eeffb_dn4 = assign12980_e17507_d_n4;
        locals.var_eeffb_dn5 = assign12980_e17507_d_n5;
        locals.var_eeffb_dn6 = assign12980_e17507_d_n6;
        locals.var_eeffb_dn8 = assign12980_e17507_d_n8;
        locals.var_eeffb_dn10 = assign12980_e17507_d_n10;
        locals.var_eeffb_dn11 = assign12980_e17507_d_n11;
        locals.var_eeffb_dn12 = assign12980_e17507_d_n12;
        locals.var_eeffb_rv = 0.0;

        let (assign12990_e17521, assign12990_e17521_d_n0, assign12990_e17521_d_n2, assign12990_e17521_d_n4, assign12990_e17521_d_n5, assign12990_e17521_d_n6, assign12990_e17521_d_n8, assign12990_e17521_d_n10, assign12990_e17521_d_n11, assign12990_e17521_d_n12,) = {
    if (p.p32 == 0.0) {
        let assign12990_e17512: f64 = (locals.var_pbds * locals.var_pbds);
        let assign12990_e17515: f64 = (4.0 * 1e-6);
        let assign12990_e17517: f64 = (assign12990_e17515 * 1e-6);
        let assign12990_e17518: f64 = (assign12990_e17512 + assign12990_e17517);
        let assign12990_e17519: f64 = (assign12990_e17518).sqrt();
        (assign12990_e17519, (((locals.var_pbds_dn0 * locals.var_pbds) + (locals.var_pbds * locals.var_pbds_dn0)) / (2.0 * assign12990_e17519)), (((locals.var_pbds_dn2 * locals.var_pbds) + (locals.var_pbds * locals.var_pbds_dn2)) / (2.0 * assign12990_e17519)), (((locals.var_pbds_dn4 * locals.var_pbds) + (locals.var_pbds * locals.var_pbds_dn4)) / (2.0 * assign12990_e17519)), (((locals.var_pbds_dn5 * locals.var_pbds) + (locals.var_pbds * locals.var_pbds_dn5)) / (2.0 * assign12990_e17519)), (((locals.var_pbds_dn6 * locals.var_pbds) + (locals.var_pbds * locals.var_pbds_dn6)) / (2.0 * assign12990_e17519)), (((locals.var_pbds_dn8 * locals.var_pbds) + (locals.var_pbds * locals.var_pbds_dn8)) / (2.0 * assign12990_e17519)), (((locals.var_pbds_dn10 * locals.var_pbds) + (locals.var_pbds * locals.var_pbds_dn10)) / (2.0 * assign12990_e17519)), (((locals.var_pbds_dn11 * locals.var_pbds) + (locals.var_pbds * locals.var_pbds_dn11)) / (2.0 * assign12990_e17519)), (((locals.var_pbds_dn12 * locals.var_pbds) + (locals.var_pbds * locals.var_pbds_dn12)) / (2.0 * assign12990_e17519)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign12990_e17521;
        locals.var_tmf2_dn0 = assign12990_e17521_d_n0;
        locals.var_tmf2_dn2 = assign12990_e17521_d_n2;
        locals.var_tmf2_dn4 = assign12990_e17521_d_n4;
        locals.var_tmf2_dn5 = assign12990_e17521_d_n5;
        locals.var_tmf2_dn6 = assign12990_e17521_d_n6;
        locals.var_tmf2_dn8 = assign12990_e17521_d_n8;
        locals.var_tmf2_dn10 = assign12990_e17521_d_n10;
        locals.var_tmf2_dn11 = assign12990_e17521_d_n11;
        locals.var_tmf2_dn12 = assign12990_e17521_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign13000_e17532, assign13000_e17532_d_n0, assign13000_e17532_d_n2, assign13000_e17532_d_n4, assign13000_e17532_d_n5, assign13000_e17532_d_n6, assign13000_e17532_d_n8, assign13000_e17532_d_n10, assign13000_e17532_d_n11, assign13000_e17532_d_n12,) = {
    if (p.p32 == 0.0) {
        let assign13000_e17528: f64 = (locals.var_pbds / locals.var_tmf2);
        let assign13000_e17529: f64 = (1.0 + assign13000_e17528);
        let assign13000_e17530: f64 = (0.5 * assign13000_e17529);
        (assign13000_e17530, (0.5 * (((locals.var_pbds_dn0 * locals.var_tmf2) - (locals.var_pbds * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_pbds_dn2 * locals.var_tmf2) - (locals.var_pbds * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_pbds_dn4 * locals.var_tmf2) - (locals.var_pbds * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_pbds_dn5 * locals.var_tmf2) - (locals.var_pbds * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_pbds_dn6 * locals.var_tmf2) - (locals.var_pbds * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_pbds_dn8 * locals.var_tmf2) - (locals.var_pbds * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_pbds_dn10 * locals.var_tmf2) - (locals.var_pbds * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_pbds_dn11 * locals.var_tmf2) - (locals.var_pbds * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_pbds_dn12 * locals.var_tmf2) - (locals.var_pbds * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign13000_e17532;
        locals.var_t0_dn0 = assign13000_e17532_d_n0;
        locals.var_t0_dn2 = assign13000_e17532_d_n2;
        locals.var_t0_dn4 = assign13000_e17532_d_n4;
        locals.var_t0_dn5 = assign13000_e17532_d_n5;
        locals.var_t0_dn6 = assign13000_e17532_d_n6;
        locals.var_t0_dn8 = assign13000_e17532_d_n8;
        locals.var_t0_dn10 = assign13000_e17532_d_n10;
        locals.var_t0_dn11 = assign13000_e17532_d_n11;
        locals.var_t0_dn12 = assign13000_e17532_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign13010_e17545, assign13010_e17545_d_n0, assign13010_e17545_d_n2, assign13010_e17545_d_n4, assign13010_e17545_d_n5, assign13010_e17545_d_n6, assign13010_e17545_d_n8, assign13010_e17545_d_n10, assign13010_e17545_d_n11, assign13010_e17545_d_n12,) = {
    if (p.p32 == 0.0) {
        let assign13010_e17538: f64 = (locals.var_pbds + locals.var_tmf2);
        let assign13010_e17539: f64 = (0.5 * assign13010_e17538);
        let assign13010_e17542: f64 = (1e-10 * 1e-6);
        let assign13010_e17543: f64 = (assign13010_e17539 + assign13010_e17542);
        (assign13010_e17543, (0.5 * (locals.var_pbds_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_pbds_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_pbds_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_pbds_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_pbds_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_pbds_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_pbds_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_pbds_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_pbds_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_pdsz, locals.var_pdsz_dn0, locals.var_pdsz_dn2, locals.var_pdsz_dn4, locals.var_pdsz_dn5, locals.var_pdsz_dn6, locals.var_pdsz_dn8, locals.var_pdsz_dn10, locals.var_pdsz_dn11, locals.var_pdsz_dn12,)
    }
};
        locals.var_pdsz = assign13010_e17545;
        locals.var_pdsz_dn0 = assign13010_e17545_d_n0;
        locals.var_pdsz_dn2 = assign13010_e17545_d_n2;
        locals.var_pdsz_dn4 = assign13010_e17545_d_n4;
        locals.var_pdsz_dn5 = assign13010_e17545_d_n5;
        locals.var_pdsz_dn6 = assign13010_e17545_d_n6;
        locals.var_pdsz_dn8 = assign13010_e17545_d_n8;
        locals.var_pdsz_dn10 = assign13010_e17545_d_n10;
        locals.var_pdsz_dn11 = assign13010_e17545_d_n11;
        locals.var_pdsz_dn12 = assign13010_e17545_d_n12;
        locals.var_pdsz_rv = 0.0;

        let assign13020_e17548: f64 = if locals.var_pdsz < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign13020_e17548;
        locals.var_guard241_rv = 0.0;

        let (assign13030_e17555, assign13030_e17555_d_n0, assign13030_e17555_d_n2, assign13030_e17555_d_n4, assign13030_e17555_d_n5, assign13030_e17555_d_n6, assign13030_e17555_d_n8, assign13030_e17555_d_n10, assign13030_e17555_d_n11, assign13030_e17555_d_n12,) = {
    if ((p.p32 == 0.0) && (locals.var_guard241 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdsz, locals.var_pdsz_dn0, locals.var_pdsz_dn2, locals.var_pdsz_dn4, locals.var_pdsz_dn5, locals.var_pdsz_dn6, locals.var_pdsz_dn8, locals.var_pdsz_dn10, locals.var_pdsz_dn11, locals.var_pdsz_dn12,)
    }
};
        locals.var_pdsz = assign13030_e17555;
        locals.var_pdsz_dn0 = assign13030_e17555_d_n0;
        locals.var_pdsz_dn2 = assign13030_e17555_d_n2;
        locals.var_pdsz_dn4 = assign13030_e17555_d_n4;
        locals.var_pdsz_dn5 = assign13030_e17555_d_n5;
        locals.var_pdsz_dn6 = assign13030_e17555_d_n6;
        locals.var_pdsz_dn8 = assign13030_e17555_d_n8;
        locals.var_pdsz_dn10 = assign13030_e17555_d_n10;
        locals.var_pdsz_dn11 = assign13030_e17555_d_n11;
        locals.var_pdsz_dn12 = assign13030_e17555_d_n12;
        locals.var_pdsz_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13040_e17562, assign13040_e17562_d_n0, assign13040_e17562_d_n2, assign13040_e17562_d_n4, assign13040_e17562_d_n5, assign13040_e17562_d_n6, assign13040_e17562_d_n8, assign13040_e17562_d_n10, assign13040_e17562_d_n11, assign13040_e17562_d_n12,) = {
    if ((p.p32 == 0.0) && (locals.var_guard241 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign13040_e17562;
        locals.var_t0_dn0 = assign13040_e17562_d_n0;
        locals.var_t0_dn2 = assign13040_e17562_d_n2;
        locals.var_t0_dn4 = assign13040_e17562_d_n4;
        locals.var_t0_dn5 = assign13040_e17562_d_n5;
        locals.var_t0_dn6 = assign13040_e17562_d_n6;
        locals.var_t0_dn8 = assign13040_e17562_d_n8;
        locals.var_t0_dn10 = assign13040_e17562_d_n10;
        locals.var_t0_dn11 = assign13040_e17562_d_n11;
        locals.var_t0_dn12 = assign13040_e17562_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign13050_e17575, assign13050_e17575_d_n0, assign13050_e17575_d_n2, assign13050_e17575_d_n4, assign13050_e17575_d_n5, assign13050_e17575_d_n6, assign13050_e17575_d_n8, assign13050_e17575_d_n10, assign13050_e17575_d_n11, assign13050_e17575_d_n12,) = {
    if (p.p32 == 0.0) {
        let assign13050_e17567: f64 = (locals.var_pdsz * locals.var_pdsz);
        let assign13050_e17569: f64 = (assign13050_e17567 + p.p216);
        let assign13050_e17570: f64 = (assign13050_e17569).sqrt();
        let assign13050_e17572: f64 = (p.p216).sqrt();
        let assign13050_e17573: f64 = (assign13050_e17570 - assign13050_e17572);
        (assign13050_e17573, (((locals.var_pdsz_dn0 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn0)) / (2.0 * assign13050_e17570)), (((locals.var_pdsz_dn2 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn2)) / (2.0 * assign13050_e17570)), (((locals.var_pdsz_dn4 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn4)) / (2.0 * assign13050_e17570)), (((locals.var_pdsz_dn5 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn5)) / (2.0 * assign13050_e17570)), (((locals.var_pdsz_dn6 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn6)) / (2.0 * assign13050_e17570)), (((locals.var_pdsz_dn8 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn8)) / (2.0 * assign13050_e17570)), (((locals.var_pdsz_dn10 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn10)) / (2.0 * assign13050_e17570)), (((locals.var_pdsz_dn11 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn11)) / (2.0 * assign13050_e17570)), (((locals.var_pdsz_dn12 * locals.var_pdsz) + (locals.var_pdsz * locals.var_pdsz_dn12)) / (2.0 * assign13050_e17570)),)
    } else {
        (locals.var_ninvdpdsz, locals.var_ninvdpdsz_dn0, locals.var_ninvdpdsz_dn2, locals.var_ninvdpdsz_dn4, locals.var_ninvdpdsz_dn5, locals.var_ninvdpdsz_dn6, locals.var_ninvdpdsz_dn8, locals.var_ninvdpdsz_dn10, locals.var_ninvdpdsz_dn11, locals.var_ninvdpdsz_dn12,)
    }
};
        locals.var_ninvdpdsz = assign13050_e17575;
        locals.var_ninvdpdsz_dn0 = assign13050_e17575_d_n0;
        locals.var_ninvdpdsz_dn2 = assign13050_e17575_d_n2;
        locals.var_ninvdpdsz_dn4 = assign13050_e17575_d_n4;
        locals.var_ninvdpdsz_dn5 = assign13050_e17575_d_n5;
        locals.var_ninvdpdsz_dn6 = assign13050_e17575_d_n6;
        locals.var_ninvdpdsz_dn8 = assign13050_e17575_d_n8;
        locals.var_ninvdpdsz_dn10 = assign13050_e17575_d_n10;
        locals.var_ninvdpdsz_dn11 = assign13050_e17575_d_n11;
        locals.var_ninvdpdsz_dn12 = assign13050_e17575_d_n12;
        locals.var_ninvdpdsz_rv = 0.0;

        let (assign13060_e17582, assign13060_e17582_d_n0, assign13060_e17582_d_n2, assign13060_e17582_d_n4, assign13060_e17582_d_n5, assign13060_e17582_d_n6, assign13060_e17582_d_n8, assign13060_e17582_d_n10, assign13060_e17582_d_n11, assign13060_e17582_d_n12,) = {
    if (p.p32 == 0.0) {
        let assign13060_e17580: f64 = (locals.var_ninvdpdsz).powf(p.p85);
        (assign13060_e17580, if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn0)) } } else { (assign13060_e17580 * (p.p85 * (locals.var_ninvdpdsz_dn0 / locals.var_ninvdpdsz))) }, if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn2)) } } else { (assign13060_e17580 * (p.p85 * (locals.var_ninvdpdsz_dn2 / locals.var_ninvdpdsz))) }, if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn4)) } } else { (assign13060_e17580 * (p.p85 * (locals.var_ninvdpdsz_dn4 / locals.var_ninvdpdsz))) }, if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn5)) } } else { (assign13060_e17580 * (p.p85 * (locals.var_ninvdpdsz_dn5 / locals.var_ninvdpdsz))) }, if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn6)) } } else { (assign13060_e17580 * (p.p85 * (locals.var_ninvdpdsz_dn6 / locals.var_ninvdpdsz))) }, if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn8)) } } else { (assign13060_e17580 * (p.p85 * (locals.var_ninvdpdsz_dn8 / locals.var_ninvdpdsz))) }, if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn10)) } } else { (assign13060_e17580 * (p.p85 * (locals.var_ninvdpdsz_dn10 / locals.var_ninvdpdsz))) }, if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn11)) } } else { (assign13060_e17580 * (p.p85 * (locals.var_ninvdpdsz_dn11 / locals.var_ninvdpdsz))) }, if 0.0 == 0.0 && ((p.p85) as f64).is_finite() && ((p.p85) as f64).fract() == 0.0 { if p.p85 == 0.0 { 0.0 } else { (p.p85 * ((locals.var_ninvdpdsz).powf(p.p85 - 1.0) * locals.var_ninvdpdsz_dn12)) } } else { (assign13060_e17580 * (p.p85 * (locals.var_ninvdpdsz_dn12 / locals.var_ninvdpdsz))) },)
    } else {
        (locals.var_ninvdpdsz, locals.var_ninvdpdsz_dn0, locals.var_ninvdpdsz_dn2, locals.var_ninvdpdsz_dn4, locals.var_ninvdpdsz_dn5, locals.var_ninvdpdsz_dn6, locals.var_ninvdpdsz_dn8, locals.var_ninvdpdsz_dn10, locals.var_ninvdpdsz_dn11, locals.var_ninvdpdsz_dn12,)
    }
};
        locals.var_ninvdpdsz = assign13060_e17582;
        locals.var_ninvdpdsz_dn0 = assign13060_e17582_d_n0;
        locals.var_ninvdpdsz_dn2 = assign13060_e17582_d_n2;
        locals.var_ninvdpdsz_dn4 = assign13060_e17582_d_n4;
        locals.var_ninvdpdsz_dn5 = assign13060_e17582_d_n5;
        locals.var_ninvdpdsz_dn6 = assign13060_e17582_d_n6;
        locals.var_ninvdpdsz_dn8 = assign13060_e17582_d_n8;
        locals.var_ninvdpdsz_dn10 = assign13060_e17582_d_n10;
        locals.var_ninvdpdsz_dn11 = assign13060_e17582_d_n11;
        locals.var_ninvdpdsz_dn12 = assign13060_e17582_d_n12;
        locals.var_ninvdpdsz_rv = 0.0;

        let (assign13070_e17591, assign13070_e17591_d_n0, assign13070_e17591_d_n2, assign13070_e17591_d_n4, assign13070_e17591_d_n5, assign13070_e17591_d_n6, assign13070_e17591_d_n8, assign13070_e17591_d_n10, assign13070_e17591_d_n11, assign13070_e17591_d_n12,) = {
    if (p.p32 == 0.0) {
        let assign13070_e17588: f64 = (locals.var_ninvdpdsz * p.p84);
        let assign13070_e17589: f64 = (1.0 + assign13070_e17588);
        (assign13070_e17589, (locals.var_ninvdpdsz_dn0 * p.p84), (locals.var_ninvdpdsz_dn2 * p.p84), (locals.var_ninvdpdsz_dn4 * p.p84), (locals.var_ninvdpdsz_dn5 * p.p84), (locals.var_ninvdpdsz_dn6 * p.p84), (locals.var_ninvdpdsz_dn8 * p.p84), (locals.var_ninvdpdsz_dn10 * p.p84), (locals.var_ninvdpdsz_dn11 * p.p84), (locals.var_ninvdpdsz_dn12 * p.p84),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign13070_e17591;
        locals.var_t4_dn0 = assign13070_e17591_d_n0;
        locals.var_t4_dn2 = assign13070_e17591_d_n2;
        locals.var_t4_dn4 = assign13070_e17591_d_n4;
        locals.var_t4_dn5 = assign13070_e17591_d_n5;
        locals.var_t4_dn6 = assign13070_e17591_d_n6;
        locals.var_t4_dn8 = assign13070_e17591_d_n8;
        locals.var_t4_dn10 = assign13070_e17591_d_n10;
        locals.var_t4_dn11 = assign13070_e17591_d_n11;
        locals.var_t4_dn12 = assign13070_e17591_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign13080_e17604,) = {
    if (p.p32 == 0.0) {
        let assign13080_e17599: f64 = (locals.var_lg).powf(p.p301);
        let assign13080_e17600: f64 = (p.p300 / assign13080_e17599);
        let assign13080_e17601: f64 = (1.0 + assign13080_e17600);
        let assign13080_e17602: f64 = (p.p302 * assign13080_e17601);
        (assign13080_e17602,)
    } else {
        (locals.var_mueqbb,)
    }
};
        locals.var_mueqbb = assign13080_e17604;
        locals.var_mueqbb_rv = 0.0;

        let (assign13090_e17613, assign13090_e17613_d_n0, assign13090_e17613_d_n2, assign13090_e17613_d_n4, assign13090_e17613_d_n5, assign13090_e17613_d_n6, assign13090_e17613_d_n8, assign13090_e17613_d_n10, assign13090_e17613_d_n11, assign13090_e17613_d_n12,) = {
    if (p.p32 == 0.0) {
        let assign13090_e17610: f64 = (locals.var_mueqbb * locals.var_cgs_q_nl);
        let assign13090_e17611: f64 = (locals.var_cgs_qibu - assign13090_e17610);
        (assign13090_e17611, (locals.var_cgs_qibu_dn0 - (locals.var_mueqbb * locals.var_cgs_q_nl_dn0)), (locals.var_cgs_qibu_dn2 - (locals.var_mueqbb * locals.var_cgs_q_nl_dn2)), (locals.var_cgs_qibu_dn4 - (locals.var_mueqbb * locals.var_cgs_q_nl_dn4)), (locals.var_cgs_qibu_dn5 - (locals.var_mueqbb * locals.var_cgs_q_nl_dn5)), (locals.var_cgs_qibu_dn6 - (locals.var_mueqbb * locals.var_cgs_q_nl_dn6)), (locals.var_cgs_qibu_dn8 - (locals.var_mueqbb * locals.var_cgs_q_nl_dn8)), (locals.var_cgs_qibu_dn10 - (locals.var_mueqbb * locals.var_cgs_q_nl_dn10)), (locals.var_cgs_qibu_dn11 - (locals.var_mueqbb * locals.var_cgs_q_nl_dn11)), (locals.var_cgs_qibu_dn12 - (locals.var_mueqbb * locals.var_cgs_q_nl_dn12)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign13090_e17613;
        locals.var_t10_dn0 = assign13090_e17613_d_n0;
        locals.var_t10_dn2 = assign13090_e17613_d_n2;
        locals.var_t10_dn4 = assign13090_e17613_d_n4;
        locals.var_t10_dn5 = assign13090_e17613_d_n5;
        locals.var_t10_dn6 = assign13090_e17613_d_n6;
        locals.var_t10_dn8 = assign13090_e17613_d_n8;
        locals.var_t10_dn10 = assign13090_e17613_d_n10;
        locals.var_t10_dn11 = assign13090_e17613_d_n11;
        locals.var_t10_dn12 = assign13090_e17613_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign13100_e17623, assign13100_e17623_d_n0, assign13100_e17623_d_n2, assign13100_e17623_d_n4, assign13100_e17623_d_n5, assign13100_e17623_d_n6, assign13100_e17623_d_n8, assign13100_e17623_d_n10, assign13100_e17623_d_n11, assign13100_e17623_d_n12,) = {
    if (p.p32 == 0.0) {
        let assign13100_e17617: f64 = (-0.5);
        let assign13100_e17620: f64 = (locals.var_cgs_q_bl_dep + locals.var_cgs_q_b0_dep);
        let assign13100_e17621: f64 = (assign13100_e17617 * assign13100_e17620);
        (assign13100_e17621, (assign13100_e17617 * (locals.var_cgs_q_bl_dep_dn0 + locals.var_cgs_q_b0_dep_dn0)), (assign13100_e17617 * (locals.var_cgs_q_bl_dep_dn2 + locals.var_cgs_q_b0_dep_dn2)), (assign13100_e17617 * (locals.var_cgs_q_bl_dep_dn4 + locals.var_cgs_q_b0_dep_dn4)), (assign13100_e17617 * (locals.var_cgs_q_bl_dep_dn5 + locals.var_cgs_q_b0_dep_dn5)), (assign13100_e17617 * (locals.var_cgs_q_bl_dep_dn6 + locals.var_cgs_q_b0_dep_dn6)), (assign13100_e17617 * (locals.var_cgs_q_bl_dep_dn8 + locals.var_cgs_q_b0_dep_dn8)), (assign13100_e17617 * (locals.var_cgs_q_bl_dep_dn10 + locals.var_cgs_q_b0_dep_dn10)), (assign13100_e17617 * (locals.var_cgs_q_bl_dep_dn11 + locals.var_cgs_q_b0_dep_dn11)), (assign13100_e17617 * (locals.var_cgs_q_bl_dep_dn12 + locals.var_cgs_q_b0_dep_dn12)),)
    } else {
        (locals.var_cgs_qbdepu, locals.var_cgs_qbdepu_dn0, locals.var_cgs_qbdepu_dn2, locals.var_cgs_qbdepu_dn4, locals.var_cgs_qbdepu_dn5, locals.var_cgs_qbdepu_dn6, locals.var_cgs_qbdepu_dn8, locals.var_cgs_qbdepu_dn10, locals.var_cgs_qbdepu_dn11, locals.var_cgs_qbdepu_dn12,)
    }
};
        locals.var_cgs_qbdepu = assign13100_e17623;
        locals.var_cgs_qbdepu_dn0 = assign13100_e17623_d_n0;
        locals.var_cgs_qbdepu_dn2 = assign13100_e17623_d_n2;
        locals.var_cgs_qbdepu_dn4 = assign13100_e17623_d_n4;
        locals.var_cgs_qbdepu_dn5 = assign13100_e17623_d_n5;
        locals.var_cgs_qbdepu_dn6 = assign13100_e17623_d_n6;
        locals.var_cgs_qbdepu_dn8 = assign13100_e17623_d_n8;
        locals.var_cgs_qbdepu_dn10 = assign13100_e17623_d_n10;
        locals.var_cgs_qbdepu_dn11 = assign13100_e17623_d_n11;
        locals.var_cgs_qbdepu_dn12 = assign13100_e17623_d_n12;
        locals.var_cgs_qbdepu_rv = 0.0;

        let (assign13110_e17634, assign13110_e17634_d_n0, assign13110_e17634_d_n2, assign13110_e17634_d_n4, assign13110_e17634_d_n5, assign13110_e17634_d_n6, assign13110_e17634_d_n8, assign13110_e17634_d_n10, assign13110_e17634_d_n11, assign13110_e17634_d_n12,) = {
    if (p.p32 == 0.0) {
        let assign13110_e17628: f64 = (locals.var_ndep * locals.var_cgs_qbdepu);
        let assign13110_e17631: f64 = (locals.var_ninv * locals.var_t10);
        let assign13110_e17632: f64 = (assign13110_e17628 + assign13110_e17631);
        (assign13110_e17632, ((locals.var_ndep * locals.var_cgs_qbdepu_dn0) + (locals.var_ninv * locals.var_t10_dn0)), ((locals.var_ndep * locals.var_cgs_qbdepu_dn2) + (locals.var_ninv * locals.var_t10_dn2)), ((locals.var_ndep * locals.var_cgs_qbdepu_dn4) + (locals.var_ninv * locals.var_t10_dn4)), ((locals.var_ndep * locals.var_cgs_qbdepu_dn5) + (locals.var_ninv * locals.var_t10_dn5)), ((locals.var_ndep * locals.var_cgs_qbdepu_dn6) + (locals.var_ninv * locals.var_t10_dn6)), ((locals.var_ndep * locals.var_cgs_qbdepu_dn8) + (locals.var_ninv * locals.var_t10_dn8)), ((locals.var_ndep * locals.var_cgs_qbdepu_dn10) + (locals.var_ninv * locals.var_t10_dn10)), ((locals.var_ndep * locals.var_cgs_qbdepu_dn11) + (locals.var_ninv * locals.var_t10_dn11)), ((locals.var_ndep * locals.var_cgs_qbdepu_dn12) + (locals.var_ninv * locals.var_t10_dn12)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign13110_e17634;
        locals.var_t5_dn0 = assign13110_e17634_d_n0;
        locals.var_t5_dn2 = assign13110_e17634_d_n2;
        locals.var_t5_dn4 = assign13110_e17634_d_n4;
        locals.var_t5_dn5 = assign13110_e17634_d_n5;
        locals.var_t5_dn6 = assign13110_e17634_d_n6;
        locals.var_t5_dn8 = assign13110_e17634_d_n8;
        locals.var_t5_dn10 = assign13110_e17634_d_n10;
        locals.var_t5_dn11 = assign13110_e17634_d_n11;
        locals.var_t5_dn12 = assign13110_e17634_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign13120_e17641, assign13120_e17641_d_n0, assign13120_e17641_d_n2, assign13120_e17641_d_n4, assign13120_e17641_d_n5, assign13120_e17641_d_n6, assign13120_e17641_d_n8, assign13120_e17641_d_n10, assign13120_e17641_d_n11, assign13120_e17641_d_n12,) = {
    if (p.p32 == 0.0) {
        let assign13120_e17639: f64 = (locals.var_t5 / locals.var_t4);
        (assign13120_e17639, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn12 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_eeffb, locals.var_eeffb_dn0, locals.var_eeffb_dn2, locals.var_eeffb_dn4, locals.var_eeffb_dn5, locals.var_eeffb_dn6, locals.var_eeffb_dn8, locals.var_eeffb_dn10, locals.var_eeffb_dn11, locals.var_eeffb_dn12,)
    }
};
        locals.var_eeffb = assign13120_e17641;
        locals.var_eeffb_dn0 = assign13120_e17641_d_n0;
        locals.var_eeffb_dn2 = assign13120_e17641_d_n2;
        locals.var_eeffb_dn4 = assign13120_e17641_d_n4;
        locals.var_eeffb_dn5 = assign13120_e17641_d_n5;
        locals.var_eeffb_dn6 = assign13120_e17641_d_n6;
        locals.var_eeffb_dn8 = assign13120_e17641_d_n8;
        locals.var_eeffb_dn10 = assign13120_e17641_d_n10;
        locals.var_eeffb_dn11 = assign13120_e17641_d_n11;
        locals.var_eeffb_dn12 = assign13120_e17641_d_n12;
        locals.var_eeffb_rv = 0.0;

        let assign13130_e17644: f64 = (locals.var_eeffb * locals.var_eeffb);
        let assign13130_e17647: f64 = (4.0 * 30.0);
        let assign13130_e17649: f64 = (assign13130_e17647 * 30.0);
        let assign13130_e17650: f64 = (assign13130_e17644 + assign13130_e17649);
        let assign13130_e17651: f64 = (assign13130_e17650).sqrt();
        locals.var_tmf2 = assign13130_e17651;
        locals.var_tmf2_dn0 = (((locals.var_eeffb_dn0 * locals.var_eeffb) + (locals.var_eeffb * locals.var_eeffb_dn0)) / (2.0 * assign13130_e17651));
        locals.var_tmf2_dn2 = (((locals.var_eeffb_dn2 * locals.var_eeffb) + (locals.var_eeffb * locals.var_eeffb_dn2)) / (2.0 * assign13130_e17651));
        locals.var_tmf2_dn4 = (((locals.var_eeffb_dn4 * locals.var_eeffb) + (locals.var_eeffb * locals.var_eeffb_dn4)) / (2.0 * assign13130_e17651));
        locals.var_tmf2_dn5 = (((locals.var_eeffb_dn5 * locals.var_eeffb) + (locals.var_eeffb * locals.var_eeffb_dn5)) / (2.0 * assign13130_e17651));
        locals.var_tmf2_dn6 = (((locals.var_eeffb_dn6 * locals.var_eeffb) + (locals.var_eeffb * locals.var_eeffb_dn6)) / (2.0 * assign13130_e17651));
        locals.var_tmf2_dn8 = (((locals.var_eeffb_dn8 * locals.var_eeffb) + (locals.var_eeffb * locals.var_eeffb_dn8)) / (2.0 * assign13130_e17651));
        locals.var_tmf2_dn10 = (((locals.var_eeffb_dn10 * locals.var_eeffb) + (locals.var_eeffb * locals.var_eeffb_dn10)) / (2.0 * assign13130_e17651));
        locals.var_tmf2_dn11 = (((locals.var_eeffb_dn11 * locals.var_eeffb) + (locals.var_eeffb * locals.var_eeffb_dn11)) / (2.0 * assign13130_e17651));
        locals.var_tmf2_dn12 = (((locals.var_eeffb_dn12 * locals.var_eeffb) + (locals.var_eeffb * locals.var_eeffb_dn12)) / (2.0 * assign13130_e17651));
        locals.var_tmf2_rv = 0.0;

        let assign13140_e17656: f64 = (locals.var_eeffb / locals.var_tmf2);
        let assign13140_e17657: f64 = (1.0 + assign13140_e17656);
        let assign13140_e17658: f64 = (0.5 * assign13140_e17657);
        locals.var_t1 = assign13140_e17658;
        locals.var_t1_dn0 = (0.5 * (((locals.var_eeffb_dn0 * locals.var_tmf2) - (locals.var_eeffb * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn2 = (0.5 * (((locals.var_eeffb_dn2 * locals.var_tmf2) - (locals.var_eeffb * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn4 = (0.5 * (((locals.var_eeffb_dn4 * locals.var_tmf2) - (locals.var_eeffb * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn5 = (0.5 * (((locals.var_eeffb_dn5 * locals.var_tmf2) - (locals.var_eeffb * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn6 = (0.5 * (((locals.var_eeffb_dn6 * locals.var_tmf2) - (locals.var_eeffb * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn8 = (0.5 * (((locals.var_eeffb_dn8 * locals.var_tmf2) - (locals.var_eeffb * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn10 = (0.5 * (((locals.var_eeffb_dn10 * locals.var_tmf2) - (locals.var_eeffb * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn11 = (0.5 * (((locals.var_eeffb_dn11 * locals.var_tmf2) - (locals.var_eeffb * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn12 = (0.5 * (((locals.var_eeffb_dn12 * locals.var_tmf2) - (locals.var_eeffb * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_rv = 0.0;

        let assign13150_e17662: f64 = (locals.var_eeffb + locals.var_tmf2);
        let assign13150_e17663: f64 = (0.5 * assign13150_e17662);
        let assign13150_e17666: f64 = (1e-10 * 30.0);
        let assign13150_e17667: f64 = (assign13150_e17663 + assign13150_e17666);
        locals.var_eeffb = assign13150_e17667;
        locals.var_eeffb_dn0 = (0.5 * (locals.var_eeffb_dn0 + locals.var_tmf2_dn0));
        locals.var_eeffb_dn2 = (0.5 * (locals.var_eeffb_dn2 + locals.var_tmf2_dn2));
        locals.var_eeffb_dn4 = (0.5 * (locals.var_eeffb_dn4 + locals.var_tmf2_dn4));
        locals.var_eeffb_dn5 = (0.5 * (locals.var_eeffb_dn5 + locals.var_tmf2_dn5));
        locals.var_eeffb_dn6 = (0.5 * (locals.var_eeffb_dn6 + locals.var_tmf2_dn6));
        locals.var_eeffb_dn8 = (0.5 * (locals.var_eeffb_dn8 + locals.var_tmf2_dn8));
        locals.var_eeffb_dn10 = (0.5 * (locals.var_eeffb_dn10 + locals.var_tmf2_dn10));
        locals.var_eeffb_dn11 = (0.5 * (locals.var_eeffb_dn11 + locals.var_tmf2_dn11));
        locals.var_eeffb_dn12 = (0.5 * (locals.var_eeffb_dn12 + locals.var_tmf2_dn12));
        locals.var_eeffb_rv = 0.0;

        let assign13160_e17670: f64 = if locals.var_eeffb < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard242 = assign13160_e17670;
        locals.var_guard242_rv = 0.0;

        let (assign13170_e17674, assign13170_e17674_d_n0, assign13170_e17674_d_n2, assign13170_e17674_d_n4, assign13170_e17674_d_n5, assign13170_e17674_d_n6, assign13170_e17674_d_n8, assign13170_e17674_d_n10, assign13170_e17674_d_n11, assign13170_e17674_d_n12,) = {
    if (locals.var_guard242 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eeffb, locals.var_eeffb_dn0, locals.var_eeffb_dn2, locals.var_eeffb_dn4, locals.var_eeffb_dn5, locals.var_eeffb_dn6, locals.var_eeffb_dn8, locals.var_eeffb_dn10, locals.var_eeffb_dn11, locals.var_eeffb_dn12,)
    }
};
        locals.var_eeffb = assign13170_e17674;
        locals.var_eeffb_dn0 = assign13170_e17674_d_n0;
        locals.var_eeffb_dn2 = assign13170_e17674_d_n2;
        locals.var_eeffb_dn4 = assign13170_e17674_d_n4;
        locals.var_eeffb_dn5 = assign13170_e17674_d_n5;
        locals.var_eeffb_dn6 = assign13170_e17674_d_n6;
        locals.var_eeffb_dn8 = assign13170_e17674_d_n8;
        locals.var_eeffb_dn10 = assign13170_e17674_d_n10;
        locals.var_eeffb_dn11 = assign13170_e17674_d_n11;
        locals.var_eeffb_dn12 = assign13170_e17674_d_n12;
        locals.var_eeffb_rv = 0.0;

        let (assign13180_e17678, assign13180_e17678_d_n0, assign13180_e17678_d_n2, assign13180_e17678_d_n4, assign13180_e17678_d_n5, assign13180_e17678_d_n6, assign13180_e17678_d_n8, assign13180_e17678_d_n10, assign13180_e17678_d_n11, assign13180_e17678_d_n12,) = {
    if (locals.var_guard242 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign13180_e17678;
        locals.var_t1_dn0 = assign13180_e17678_d_n0;
        locals.var_t1_dn2 = assign13180_e17678_d_n2;
        locals.var_t1_dn4 = assign13180_e17678_d_n4;
        locals.var_t1_dn5 = assign13180_e17678_d_n5;
        locals.var_t1_dn6 = assign13180_e17678_d_n6;
        locals.var_t1_dn8 = assign13180_e17678_d_n8;
        locals.var_t1_dn10 = assign13180_e17678_d_n10;
        locals.var_t1_dn11 = assign13180_e17678_d_n11;
        locals.var_t1_dn12 = assign13180_e17678_d_n12;
        locals.var_t1_rv = 0.0;

        let assign13190_e17681: f64 = (locals.var_eeffb).powf(p.p275);
        locals.var_t8 = assign13190_e17681;
        locals.var_t8_dn0 = if 0.0 == 0.0 && ((p.p275) as f64).is_finite() && ((p.p275) as f64).fract() == 0.0 { if p.p275 == 0.0 { 0.0 } else { (p.p275 * ((locals.var_eeffb).powf(p.p275 - 1.0) * locals.var_eeffb_dn0)) } } else { (assign13190_e17681 * (p.p275 * (locals.var_eeffb_dn0 / locals.var_eeffb))) };
        locals.var_t8_dn2 = if 0.0 == 0.0 && ((p.p275) as f64).is_finite() && ((p.p275) as f64).fract() == 0.0 { if p.p275 == 0.0 { 0.0 } else { (p.p275 * ((locals.var_eeffb).powf(p.p275 - 1.0) * locals.var_eeffb_dn2)) } } else { (assign13190_e17681 * (p.p275 * (locals.var_eeffb_dn2 / locals.var_eeffb))) };
        locals.var_t8_dn4 = if 0.0 == 0.0 && ((p.p275) as f64).is_finite() && ((p.p275) as f64).fract() == 0.0 { if p.p275 == 0.0 { 0.0 } else { (p.p275 * ((locals.var_eeffb).powf(p.p275 - 1.0) * locals.var_eeffb_dn4)) } } else { (assign13190_e17681 * (p.p275 * (locals.var_eeffb_dn4 / locals.var_eeffb))) };
        locals.var_t8_dn5 = if 0.0 == 0.0 && ((p.p275) as f64).is_finite() && ((p.p275) as f64).fract() == 0.0 { if p.p275 == 0.0 { 0.0 } else { (p.p275 * ((locals.var_eeffb).powf(p.p275 - 1.0) * locals.var_eeffb_dn5)) } } else { (assign13190_e17681 * (p.p275 * (locals.var_eeffb_dn5 / locals.var_eeffb))) };
        locals.var_t8_dn6 = if 0.0 == 0.0 && ((p.p275) as f64).is_finite() && ((p.p275) as f64).fract() == 0.0 { if p.p275 == 0.0 { 0.0 } else { (p.p275 * ((locals.var_eeffb).powf(p.p275 - 1.0) * locals.var_eeffb_dn6)) } } else { (assign13190_e17681 * (p.p275 * (locals.var_eeffb_dn6 / locals.var_eeffb))) };
        locals.var_t8_dn8 = if 0.0 == 0.0 && ((p.p275) as f64).is_finite() && ((p.p275) as f64).fract() == 0.0 { if p.p275 == 0.0 { 0.0 } else { (p.p275 * ((locals.var_eeffb).powf(p.p275 - 1.0) * locals.var_eeffb_dn8)) } } else { (assign13190_e17681 * (p.p275 * (locals.var_eeffb_dn8 / locals.var_eeffb))) };
        locals.var_t8_dn10 = if 0.0 == 0.0 && ((p.p275) as f64).is_finite() && ((p.p275) as f64).fract() == 0.0 { if p.p275 == 0.0 { 0.0 } else { (p.p275 * ((locals.var_eeffb).powf(p.p275 - 1.0) * locals.var_eeffb_dn10)) } } else { (assign13190_e17681 * (p.p275 * (locals.var_eeffb_dn10 / locals.var_eeffb))) };
        locals.var_t8_dn11 = if 0.0 == 0.0 && ((p.p275) as f64).is_finite() && ((p.p275) as f64).fract() == 0.0 { if p.p275 == 0.0 { 0.0 } else { (p.p275 * ((locals.var_eeffb).powf(p.p275 - 1.0) * locals.var_eeffb_dn11)) } } else { (assign13190_e17681 * (p.p275 * (locals.var_eeffb_dn11 / locals.var_eeffb))) };
        locals.var_t8_dn12 = if 0.0 == 0.0 && ((p.p275) as f64).is_finite() && ((p.p275) as f64).fract() == 0.0 { if p.p275 == 0.0 { 0.0 } else { (p.p275 * ((locals.var_eeffb).powf(p.p275 - 1.0) * locals.var_eeffb_dn12)) } } else { (assign13190_e17681 * (p.p275 * (locals.var_eeffb_dn12 / locals.var_eeffb))) };
        locals.var_t8_rv = 0.0;

        let assign13200_e17684: f64 = (locals.var_eeffb).powf(locals.var_muesrb);
        locals.var_t6 = assign13200_e17684;
        locals.var_t6_dn0 = if 0.0 == 0.0 && ((locals.var_muesrb) as f64).is_finite() && ((locals.var_muesrb) as f64).fract() == 0.0 { if locals.var_muesrb == 0.0 { 0.0 } else { (locals.var_muesrb * ((locals.var_eeffb).powf(locals.var_muesrb - 1.0) * locals.var_eeffb_dn0)) } } else { (assign13200_e17684 * (locals.var_muesrb * (locals.var_eeffb_dn0 / locals.var_eeffb))) };
        locals.var_t6_dn2 = if 0.0 == 0.0 && ((locals.var_muesrb) as f64).is_finite() && ((locals.var_muesrb) as f64).fract() == 0.0 { if locals.var_muesrb == 0.0 { 0.0 } else { (locals.var_muesrb * ((locals.var_eeffb).powf(locals.var_muesrb - 1.0) * locals.var_eeffb_dn2)) } } else { (assign13200_e17684 * (locals.var_muesrb * (locals.var_eeffb_dn2 / locals.var_eeffb))) };
        locals.var_t6_dn4 = if 0.0 == 0.0 && ((locals.var_muesrb) as f64).is_finite() && ((locals.var_muesrb) as f64).fract() == 0.0 { if locals.var_muesrb == 0.0 { 0.0 } else { (locals.var_muesrb * ((locals.var_eeffb).powf(locals.var_muesrb - 1.0) * locals.var_eeffb_dn4)) } } else { (assign13200_e17684 * (locals.var_muesrb * (locals.var_eeffb_dn4 / locals.var_eeffb))) };
        locals.var_t6_dn5 = if 0.0 == 0.0 && ((locals.var_muesrb) as f64).is_finite() && ((locals.var_muesrb) as f64).fract() == 0.0 { if locals.var_muesrb == 0.0 { 0.0 } else { (locals.var_muesrb * ((locals.var_eeffb).powf(locals.var_muesrb - 1.0) * locals.var_eeffb_dn5)) } } else { (assign13200_e17684 * (locals.var_muesrb * (locals.var_eeffb_dn5 / locals.var_eeffb))) };
        locals.var_t6_dn6 = if 0.0 == 0.0 && ((locals.var_muesrb) as f64).is_finite() && ((locals.var_muesrb) as f64).fract() == 0.0 { if locals.var_muesrb == 0.0 { 0.0 } else { (locals.var_muesrb * ((locals.var_eeffb).powf(locals.var_muesrb - 1.0) * locals.var_eeffb_dn6)) } } else { (assign13200_e17684 * (locals.var_muesrb * (locals.var_eeffb_dn6 / locals.var_eeffb))) };
        locals.var_t6_dn8 = if 0.0 == 0.0 && ((locals.var_muesrb) as f64).is_finite() && ((locals.var_muesrb) as f64).fract() == 0.0 { if locals.var_muesrb == 0.0 { 0.0 } else { (locals.var_muesrb * ((locals.var_eeffb).powf(locals.var_muesrb - 1.0) * locals.var_eeffb_dn8)) } } else { (assign13200_e17684 * (locals.var_muesrb * (locals.var_eeffb_dn8 / locals.var_eeffb))) };
        locals.var_t6_dn10 = if 0.0 == 0.0 && ((locals.var_muesrb) as f64).is_finite() && ((locals.var_muesrb) as f64).fract() == 0.0 { if locals.var_muesrb == 0.0 { 0.0 } else { (locals.var_muesrb * ((locals.var_eeffb).powf(locals.var_muesrb - 1.0) * locals.var_eeffb_dn10)) } } else { (assign13200_e17684 * (locals.var_muesrb * (locals.var_eeffb_dn10 / locals.var_eeffb))) };
        locals.var_t6_dn11 = if 0.0 == 0.0 && ((locals.var_muesrb) as f64).is_finite() && ((locals.var_muesrb) as f64).fract() == 0.0 { if locals.var_muesrb == 0.0 { 0.0 } else { (locals.var_muesrb * ((locals.var_eeffb).powf(locals.var_muesrb - 1.0) * locals.var_eeffb_dn11)) } } else { (assign13200_e17684 * (locals.var_muesrb * (locals.var_eeffb_dn11 / locals.var_eeffb))) };
        locals.var_t6_dn12 = if 0.0 == 0.0 && ((locals.var_muesrb) as f64).is_finite() && ((locals.var_muesrb) as f64).fract() == 0.0 { if locals.var_muesrb == 0.0 { 0.0 } else { (locals.var_muesrb * ((locals.var_eeffb).powf(locals.var_muesrb - 1.0) * locals.var_eeffb_dn12)) } } else { (assign13200_e17684 * (locals.var_muesrb * (locals.var_eeffb_dn12 / locals.var_eeffb))) };
        locals.var_t6_rv = 0.0;

        let assign13210_e17687: f64 = (locals.var_cgs_qibu / 1.6021918e-19);
        locals.var_rns = assign13210_e17687;
        locals.var_rns_dn0 = (locals.var_cgs_qibu_dn0 / 1.6021918e-19);
        locals.var_rns_dn2 = (locals.var_cgs_qibu_dn2 / 1.6021918e-19);
        locals.var_rns_dn4 = (locals.var_cgs_qibu_dn4 / 1.6021918e-19);
        locals.var_rns_dn5 = (locals.var_cgs_qibu_dn5 / 1.6021918e-19);
        locals.var_rns_dn6 = (locals.var_cgs_qibu_dn6 / 1.6021918e-19);
        locals.var_rns_dn8 = (locals.var_cgs_qibu_dn8 / 1.6021918e-19);
        locals.var_rns_dn10 = (locals.var_cgs_qibu_dn10 / 1.6021918e-19);
        locals.var_rns_dn11 = (locals.var_cgs_qibu_dn11 / 1.6021918e-19);
        locals.var_rns_dn12 = (locals.var_cgs_qibu_dn12 / 1.6021918e-19);
        locals.var_rns_rv = 0.0;

        let assign13220_e17692: f64 = (locals.var_uc_muecb1b * locals.var_rns);
        let assign13220_e17694: f64 = (assign13220_e17692 / 100000000000.0);
        let assign13220_e17695: f64 = (locals.var_uc_muecb0b + assign13220_e17694);
        let assign13220_e17696: f64 = (1.0 / assign13220_e17695);
        let assign13220_e17699: f64 = (locals.var_cgs_mphbn0 * locals.var_t8);
        let assign13220_e17700: f64 = (assign13220_e17696 + assign13220_e17699);
        let assign13220_e17703: f64 = (locals.var_t6 / p.p284);
        let assign13220_e17704: f64 = (assign13220_e17700 + assign13220_e17703);
        locals.var_t1 = assign13220_e17704;
        locals.var_t1_dn0 = (((-(((locals.var_uc_muecb1b * locals.var_rns_dn0) / 100000000000.0) / (assign13220_e17695 * assign13220_e17695))) + ((locals.var_cgs_mphbn0_dn0 * locals.var_t8) + (locals.var_cgs_mphbn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / p.p284));
        locals.var_t1_dn2 = (((-(((locals.var_uc_muecb1b * locals.var_rns_dn2) / 100000000000.0) / (assign13220_e17695 * assign13220_e17695))) + ((locals.var_cgs_mphbn0_dn2 * locals.var_t8) + (locals.var_cgs_mphbn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / p.p284));
        locals.var_t1_dn4 = (((-(((locals.var_uc_muecb1b * locals.var_rns_dn4) / 100000000000.0) / (assign13220_e17695 * assign13220_e17695))) + ((locals.var_cgs_mphbn0_dn4 * locals.var_t8) + (locals.var_cgs_mphbn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / p.p284));
        locals.var_t1_dn5 = (((-(((locals.var_uc_muecb1b * locals.var_rns_dn5) / 100000000000.0) / (assign13220_e17695 * assign13220_e17695))) + ((locals.var_cgs_mphbn0_dn5 * locals.var_t8) + (locals.var_cgs_mphbn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / p.p284));
        locals.var_t1_dn6 = (((-(((locals.var_uc_muecb1b * locals.var_rns_dn6) / 100000000000.0) / (assign13220_e17695 * assign13220_e17695))) + ((locals.var_cgs_mphbn0_dn6 * locals.var_t8) + (locals.var_cgs_mphbn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / p.p284));
        locals.var_t1_dn8 = (((-(((locals.var_uc_muecb1b * locals.var_rns_dn8) / 100000000000.0) / (assign13220_e17695 * assign13220_e17695))) + ((locals.var_cgs_mphbn0_dn8 * locals.var_t8) + (locals.var_cgs_mphbn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / p.p284));
        locals.var_t1_dn10 = (((-(((locals.var_uc_muecb1b * locals.var_rns_dn10) / 100000000000.0) / (assign13220_e17695 * assign13220_e17695))) + ((locals.var_cgs_mphbn0_dn10 * locals.var_t8) + (locals.var_cgs_mphbn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / p.p284));
        locals.var_t1_dn11 = (((-(((locals.var_uc_muecb1b * locals.var_rns_dn11) / 100000000000.0) / (assign13220_e17695 * assign13220_e17695))) + ((locals.var_cgs_mphbn0_dn11 * locals.var_t8) + (locals.var_cgs_mphbn0 * locals.var_t8_dn11))) + (locals.var_t6_dn11 / p.p284));
        locals.var_t1_dn12 = (((-(((locals.var_uc_muecb1b * locals.var_rns_dn12) / 100000000000.0) / (assign13220_e17695 * assign13220_e17695))) + ((locals.var_cgs_mphbn0_dn12 * locals.var_t8) + (locals.var_cgs_mphbn0 * locals.var_t8_dn12))) + (locals.var_t6_dn12 / p.p284));
        locals.var_t1_rv = 0.0;

        let assign13230_e17707: f64 = (1.0 / locals.var_t1);
        locals.var_muunb = assign13230_e17707;
        locals.var_muunb_dn0 = (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1)));
        locals.var_muunb_dn2 = (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1)));
        locals.var_muunb_dn4 = (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1)));
        locals.var_muunb_dn5 = (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1)));
        locals.var_muunb_dn6 = (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1)));
        locals.var_muunb_dn8 = (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1)));
        locals.var_muunb_dn10 = (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1)));
        locals.var_muunb_dn11 = (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1)));
        locals.var_muunb_dn12 = (-(locals.var_t1_dn12 / (locals.var_t1 * locals.var_t1)));
        locals.var_muunb_rv = 0.0;

        let assign13240_e17710: f64 = (locals.var_muunb * 0.0001);
        locals.var_muunb = assign13240_e17710;
        locals.var_muunb_dn0 = (locals.var_muunb_dn0 * 0.0001);
        locals.var_muunb_dn2 = (locals.var_muunb_dn2 * 0.0001);
        locals.var_muunb_dn4 = (locals.var_muunb_dn4 * 0.0001);
        locals.var_muunb_dn5 = (locals.var_muunb_dn5 * 0.0001);
        locals.var_muunb_dn6 = (locals.var_muunb_dn6 * 0.0001);
        locals.var_muunb_dn8 = (locals.var_muunb_dn8 * 0.0001);
        locals.var_muunb_dn10 = (locals.var_muunb_dn10 * 0.0001);
        locals.var_muunb_dn11 = (locals.var_muunb_dn11 * 0.0001);
        locals.var_muunb_dn12 = (locals.var_muunb_dn12 * 0.0001);
        locals.var_muunb_rv = 0.0;

        let assign13250_e17713: f64 = (0.2 * locals.var_vmaxe);
        let assign13250_e17715: f64 = (assign13250_e17713 / locals.var_muun);
        locals.var_c_mueey0 = assign13250_e17715;
        locals.var_c_mueey0_dn0 = ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign13250_e17713 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun));
        locals.var_c_mueey0_dn2 = ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign13250_e17713 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun));
        locals.var_c_mueey0_dn4 = ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign13250_e17713 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun));
        locals.var_c_mueey0_dn5 = ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign13250_e17713 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun));
        locals.var_c_mueey0_dn6 = ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign13250_e17713 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun));
        locals.var_c_mueey0_dn8 = ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign13250_e17713 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun));
        locals.var_c_mueey0_dn10 = ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign13250_e17713 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun));
        locals.var_c_mueey0_dn11 = ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign13250_e17713 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun));
        locals.var_c_mueey0_dn12 = ((((0.2 * locals.var_vmaxe_dn12) * locals.var_muun) - (assign13250_e17713 * locals.var_muun_dn12)) / (locals.var_muun * locals.var_muun));
        locals.var_c_mueey0_rv = 0.0;

        let assign13260_e17720: f64 = (locals.var_qn0 + 1e-50);
        let assign13260_e17721: f64 = (locals.var_beta * assign13260_e17720);
        let assign13260_e17723: f64 = (assign13260_e17721 * locals.var_lch);
        let assign13260_e17724: f64 = (locals.var_idd1 / assign13260_e17723);
        locals.var_ty = assign13260_e17724;
        locals.var_ty_dn0 = (((locals.var_idd1_dn0 * assign13260_e17723) - (locals.var_idd1 * (((locals.var_beta * locals.var_qn0_dn0) * locals.var_lch) + (assign13260_e17721 * locals.var_lch_dn0)))) / (assign13260_e17723 * assign13260_e17723));
        locals.var_ty_dn2 = (((locals.var_idd1_dn2 * assign13260_e17723) - (locals.var_idd1 * (((locals.var_beta * locals.var_qn0_dn2) * locals.var_lch) + (assign13260_e17721 * locals.var_lch_dn2)))) / (assign13260_e17723 * assign13260_e17723));
        locals.var_ty_dn4 = (((locals.var_idd1_dn4 * assign13260_e17723) - (locals.var_idd1 * ((((locals.var_beta_dn4 * assign13260_e17720) + (locals.var_beta * locals.var_qn0_dn4)) * locals.var_lch) + (assign13260_e17721 * locals.var_lch_dn4)))) / (assign13260_e17723 * assign13260_e17723));
        locals.var_ty_dn5 = (((locals.var_idd1_dn5 * assign13260_e17723) - (locals.var_idd1 * (((locals.var_beta * locals.var_qn0_dn5) * locals.var_lch) + (assign13260_e17721 * locals.var_lch_dn5)))) / (assign13260_e17723 * assign13260_e17723));
        locals.var_ty_dn6 = (((locals.var_idd1_dn6 * assign13260_e17723) - (locals.var_idd1 * (((locals.var_beta * locals.var_qn0_dn6) * locals.var_lch) + (assign13260_e17721 * locals.var_lch_dn6)))) / (assign13260_e17723 * assign13260_e17723));
        locals.var_ty_dn8 = (((locals.var_idd1_dn8 * assign13260_e17723) - (locals.var_idd1 * (((locals.var_beta * locals.var_qn0_dn8) * locals.var_lch) + (assign13260_e17721 * locals.var_lch_dn8)))) / (assign13260_e17723 * assign13260_e17723));
        locals.var_ty_dn10 = (((locals.var_idd1_dn10 * assign13260_e17723) - (locals.var_idd1 * (((locals.var_beta * locals.var_qn0_dn10) * locals.var_lch) + (assign13260_e17721 * locals.var_lch_dn10)))) / (assign13260_e17723 * assign13260_e17723));
        locals.var_ty_dn11 = (((locals.var_idd1_dn11 * assign13260_e17723) - (locals.var_idd1 * (((locals.var_beta * locals.var_qn0_dn11) * locals.var_lch) + (assign13260_e17721 * locals.var_lch_dn11)))) / (assign13260_e17723 * assign13260_e17723));
        locals.var_ty_dn12 = (((locals.var_idd1_dn12 * assign13260_e17723) - (locals.var_idd1 * (((locals.var_beta * locals.var_qn0_dn12) * locals.var_lch) + (assign13260_e17721 * locals.var_lch_dn12)))) / (assign13260_e17723 * assign13260_e17723));
        locals.var_ty_rv = 0.0;

        let assign13270_e17727: f64 = (locals.var_ty * locals.var_ty);
        let assign13270_e17730: f64 = (locals.var_c_mueey0 * locals.var_c_mueey0);
        let assign13270_e17731: f64 = (assign13270_e17727 + assign13270_e17730);
        let assign13270_e17732: f64 = (assign13270_e17731).sqrt();
        locals.var_ey = assign13270_e17732;
        locals.var_ey_dn0 = ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_c_mueey0_dn0 * locals.var_c_mueey0) + (locals.var_c_mueey0 * locals.var_c_mueey0_dn0))) / (2.0 * assign13270_e17732));
        locals.var_ey_dn2 = ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_c_mueey0_dn2 * locals.var_c_mueey0) + (locals.var_c_mueey0 * locals.var_c_mueey0_dn2))) / (2.0 * assign13270_e17732));
        locals.var_ey_dn4 = ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_c_mueey0_dn4 * locals.var_c_mueey0) + (locals.var_c_mueey0 * locals.var_c_mueey0_dn4))) / (2.0 * assign13270_e17732));
        locals.var_ey_dn5 = ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_c_mueey0_dn5 * locals.var_c_mueey0) + (locals.var_c_mueey0 * locals.var_c_mueey0_dn5))) / (2.0 * assign13270_e17732));
        locals.var_ey_dn6 = ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_c_mueey0_dn6 * locals.var_c_mueey0) + (locals.var_c_mueey0 * locals.var_c_mueey0_dn6))) / (2.0 * assign13270_e17732));
        locals.var_ey_dn8 = ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_c_mueey0_dn8 * locals.var_c_mueey0) + (locals.var_c_mueey0 * locals.var_c_mueey0_dn8))) / (2.0 * assign13270_e17732));
        locals.var_ey_dn10 = ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_c_mueey0_dn10 * locals.var_c_mueey0) + (locals.var_c_mueey0 * locals.var_c_mueey0_dn10))) / (2.0 * assign13270_e17732));
        locals.var_ey_dn11 = ((((locals.var_ty_dn11 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn11)) + ((locals.var_c_mueey0_dn11 * locals.var_c_mueey0) + (locals.var_c_mueey0 * locals.var_c_mueey0_dn11))) / (2.0 * assign13270_e17732));
        locals.var_ey_dn12 = ((((locals.var_ty_dn12 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn12)) + ((locals.var_c_mueey0_dn12 * locals.var_c_mueey0) + (locals.var_c_mueey0 * locals.var_c_mueey0_dn12))) / (2.0 * assign13270_e17732));
        locals.var_ey_rv = 0.0;

        let assign13280_e17735: f64 = (locals.var_muun * locals.var_ey);
        locals.var_em = assign13280_e17735;
        locals.var_em_dn0 = ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0));
        locals.var_em_dn2 = ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2));
        locals.var_em_dn4 = ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4));
        locals.var_em_dn5 = ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5));
        locals.var_em_dn6 = ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6));
        locals.var_em_dn8 = ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8));
        locals.var_em_dn10 = ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10));
        locals.var_em_dn11 = ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11));
        locals.var_em_dn12 = ((locals.var_muun_dn12 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn12));
        locals.var_em_rv = 0.0;

        let assign13290_e17738: f64 = (locals.var_em / locals.var_vmaxe);
        locals.var_t1 = assign13290_e17738;
        locals.var_t1_dn0 = (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn2 = (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn4 = (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn5 = (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn6 = (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn8 = (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn10 = (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn11 = (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn12 = (((locals.var_em_dn12 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn12)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_rv = 0.0;

        let assign13300_e17742: f64 = (10.0 * 2.220446049250313e-16);
        let assign13300_e17743: f64 = (1.0 - assign13300_e17742);
        let assign13300_e17750: f64 = (10.0 * 2.220446049250313e-16);
        let assign13300_e17751: f64 = (1.0 + assign13300_e17750);
        let assign13300_e17753: f64 = if ((assign13300_e17743 <= p.p114) && (p.p114 <= assign13300_e17751)) { 1.0 } else { 0.0 };
        locals.var_guard243 = assign13300_e17753;
        locals.var_guard243_rv = 0.0;

        let (assign13310_e17757, assign13310_e17757_d_n0, assign13310_e17757_d_n2, assign13310_e17757_d_n4, assign13310_e17757_d_n5, assign13310_e17757_d_n6, assign13310_e17757_d_n8, assign13310_e17757_d_n10, assign13310_e17757_d_n11, assign13310_e17757_d_n12,) = {
    if (locals.var_guard243 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign13310_e17757;
        locals.var_t3_dn0 = assign13310_e17757_d_n0;
        locals.var_t3_dn2 = assign13310_e17757_d_n2;
        locals.var_t3_dn4 = assign13310_e17757_d_n4;
        locals.var_t3_dn5 = assign13310_e17757_d_n5;
        locals.var_t3_dn6 = assign13310_e17757_d_n6;
        locals.var_t3_dn8 = assign13310_e17757_d_n8;
        locals.var_t3_dn10 = assign13310_e17757_d_n10;
        locals.var_t3_dn11 = assign13310_e17757_d_n11;
        locals.var_t3_dn12 = assign13310_e17757_d_n12;
        locals.var_t3_rv = 0.0;

        let assign13320_e17761: f64 = (10.0 * 2.220446049250313e-16);
        let assign13320_e17762: f64 = (2.0 - assign13320_e17761);
        let assign13320_e17769: f64 = (10.0 * 2.220446049250313e-16);
        let assign13320_e17770: f64 = (2.0 + assign13320_e17769);
        let assign13320_e17772: f64 = if ((assign13320_e17762 <= p.p114) && (p.p114 <= assign13320_e17770)) { 1.0 } else { 0.0 };
        locals.var_guard244 = assign13320_e17772;
        locals.var_guard244_rv = 0.0;

        let (assign13330_e17779, assign13330_e17779_d_n0, assign13330_e17779_d_n2, assign13330_e17779_d_n4, assign13330_e17779_d_n5, assign13330_e17779_d_n6, assign13330_e17779_d_n8, assign13330_e17779_d_n10, assign13330_e17779_d_n11, assign13330_e17779_d_n12,) = {
    if ((locals.var_guard243 == 0.0) && (locals.var_guard244 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign13330_e17779;
        locals.var_t3_dn0 = assign13330_e17779_d_n0;
        locals.var_t3_dn2 = assign13330_e17779_d_n2;
        locals.var_t3_dn4 = assign13330_e17779_d_n4;
        locals.var_t3_dn5 = assign13330_e17779_d_n5;
        locals.var_t3_dn6 = assign13330_e17779_d_n6;
        locals.var_t3_dn8 = assign13330_e17779_d_n8;
        locals.var_t3_dn10 = assign13330_e17779_d_n10;
        locals.var_t3_dn11 = assign13330_e17779_d_n11;
        locals.var_t3_dn12 = assign13330_e17779_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign13340_e17791, assign13340_e17791_d_n0, assign13340_e17791_d_n2, assign13340_e17791_d_n4, assign13340_e17791_d_n5, assign13340_e17791_d_n6, assign13340_e17791_d_n8, assign13340_e17791_d_n10, assign13340_e17791_d_n11, assign13340_e17791_d_n12,) = {
    if ((locals.var_guard243 == 0.0) && (locals.var_guard244 == 0.0)) {
        let assign13340_e17788: f64 = (p.p114 - 1.0);
        let assign13340_e17789: f64 = (locals.var_t1).powf(assign13340_e17788);
        (assign13340_e17789, if 0.0 == 0.0 && ((assign13340_e17788) as f64).is_finite() && ((assign13340_e17788) as f64).fract() == 0.0 { if assign13340_e17788 == 0.0 { 0.0 } else { (assign13340_e17788 * ((locals.var_t1).powf(assign13340_e17788 - 1.0) * locals.var_t1_dn0)) } } else { (assign13340_e17789 * (assign13340_e17788 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13340_e17788) as f64).is_finite() && ((assign13340_e17788) as f64).fract() == 0.0 { if assign13340_e17788 == 0.0 { 0.0 } else { (assign13340_e17788 * ((locals.var_t1).powf(assign13340_e17788 - 1.0) * locals.var_t1_dn2)) } } else { (assign13340_e17789 * (assign13340_e17788 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13340_e17788) as f64).is_finite() && ((assign13340_e17788) as f64).fract() == 0.0 { if assign13340_e17788 == 0.0 { 0.0 } else { (assign13340_e17788 * ((locals.var_t1).powf(assign13340_e17788 - 1.0) * locals.var_t1_dn4)) } } else { (assign13340_e17789 * (assign13340_e17788 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13340_e17788) as f64).is_finite() && ((assign13340_e17788) as f64).fract() == 0.0 { if assign13340_e17788 == 0.0 { 0.0 } else { (assign13340_e17788 * ((locals.var_t1).powf(assign13340_e17788 - 1.0) * locals.var_t1_dn5)) } } else { (assign13340_e17789 * (assign13340_e17788 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13340_e17788) as f64).is_finite() && ((assign13340_e17788) as f64).fract() == 0.0 { if assign13340_e17788 == 0.0 { 0.0 } else { (assign13340_e17788 * ((locals.var_t1).powf(assign13340_e17788 - 1.0) * locals.var_t1_dn6)) } } else { (assign13340_e17789 * (assign13340_e17788 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13340_e17788) as f64).is_finite() && ((assign13340_e17788) as f64).fract() == 0.0 { if assign13340_e17788 == 0.0 { 0.0 } else { (assign13340_e17788 * ((locals.var_t1).powf(assign13340_e17788 - 1.0) * locals.var_t1_dn8)) } } else { (assign13340_e17789 * (assign13340_e17788 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13340_e17788) as f64).is_finite() && ((assign13340_e17788) as f64).fract() == 0.0 { if assign13340_e17788 == 0.0 { 0.0 } else { (assign13340_e17788 * ((locals.var_t1).powf(assign13340_e17788 - 1.0) * locals.var_t1_dn10)) } } else { (assign13340_e17789 * (assign13340_e17788 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13340_e17788) as f64).is_finite() && ((assign13340_e17788) as f64).fract() == 0.0 { if assign13340_e17788 == 0.0 { 0.0 } else { (assign13340_e17788 * ((locals.var_t1).powf(assign13340_e17788 - 1.0) * locals.var_t1_dn11)) } } else { (assign13340_e17789 * (assign13340_e17788 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13340_e17788) as f64).is_finite() && ((assign13340_e17788) as f64).fract() == 0.0 { if assign13340_e17788 == 0.0 { 0.0 } else { (assign13340_e17788 * ((locals.var_t1).powf(assign13340_e17788 - 1.0) * locals.var_t1_dn12)) } } else { (assign13340_e17789 * (assign13340_e17788 * (locals.var_t1_dn12 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign13340_e17791;
        locals.var_t3_dn0 = assign13340_e17791_d_n0;
        locals.var_t3_dn2 = assign13340_e17791_d_n2;
        locals.var_t3_dn4 = assign13340_e17791_d_n4;
        locals.var_t3_dn5 = assign13340_e17791_d_n5;
        locals.var_t3_dn6 = assign13340_e17791_d_n6;
        locals.var_t3_dn8 = assign13340_e17791_d_n8;
        locals.var_t3_dn10 = assign13340_e17791_d_n10;
        locals.var_t3_dn11 = assign13340_e17791_d_n11;
        locals.var_t3_dn12 = assign13340_e17791_d_n12;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13350_e17795: f64 = (locals.var_t1 * locals.var_t3);
        let assign13350_e17796: f64 = (1.0 + assign13350_e17795);
        locals.var_t4 = assign13350_e17796;
        locals.var_t4_dn0 = ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0));
        locals.var_t4_dn2 = ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2));
        locals.var_t4_dn4 = ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4));
        locals.var_t4_dn5 = ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5));
        locals.var_t4_dn6 = ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6));
        locals.var_t4_dn8 = ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8));
        locals.var_t4_dn10 = ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10));
        locals.var_t4_dn11 = ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11));
        locals.var_t4_dn12 = ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12));
        locals.var_t4_rv = 0.0;

        let assign13360_e17800: f64 = (10.0 * 2.220446049250313e-16);
        let assign13360_e17801: f64 = (1.0 - assign13360_e17800);
        let assign13360_e17808: f64 = (10.0 * 2.220446049250313e-16);
        let assign13360_e17809: f64 = (1.0 + assign13360_e17808);
        let assign13360_e17811: f64 = if ((assign13360_e17801 <= p.p114) && (p.p114 <= assign13360_e17809)) { 1.0 } else { 0.0 };
        locals.var_guard245 = assign13360_e17811;
        locals.var_guard245_rv = 0.0;

        let (assign13370_e17817, assign13370_e17817_d_n0, assign13370_e17817_d_n2, assign13370_e17817_d_n4, assign13370_e17817_d_n5, assign13370_e17817_d_n6, assign13370_e17817_d_n8, assign13370_e17817_d_n10, assign13370_e17817_d_n11, assign13370_e17817_d_n12,) = {
    if (locals.var_guard245 != 0.0) {
        let assign13370_e17815: f64 = (1.0 / locals.var_t4);
        (assign13370_e17815, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign13370_e17817;
        locals.var_t5_dn0 = assign13370_e17817_d_n0;
        locals.var_t5_dn2 = assign13370_e17817_d_n2;
        locals.var_t5_dn4 = assign13370_e17817_d_n4;
        locals.var_t5_dn5 = assign13370_e17817_d_n5;
        locals.var_t5_dn6 = assign13370_e17817_d_n6;
        locals.var_t5_dn8 = assign13370_e17817_d_n8;
        locals.var_t5_dn10 = assign13370_e17817_d_n10;
        locals.var_t5_dn11 = assign13370_e17817_d_n11;
        locals.var_t5_dn12 = assign13370_e17817_d_n12;
        locals.var_t5_rv = 0.0;

        let assign13380_e17821: f64 = (10.0 * 2.220446049250313e-16);
        let assign13380_e17822: f64 = (2.0 - assign13380_e17821);
        let assign13380_e17829: f64 = (10.0 * 2.220446049250313e-16);
        let assign13380_e17830: f64 = (2.0 + assign13380_e17829);
        let assign13380_e17832: f64 = if ((assign13380_e17822 <= p.p114) && (p.p114 <= assign13380_e17830)) { 1.0 } else { 0.0 };
        locals.var_guard246 = assign13380_e17832;
        locals.var_guard246_rv = 0.0;

        let (assign13390_e17842, assign13390_e17842_d_n0, assign13390_e17842_d_n2, assign13390_e17842_d_n4, assign13390_e17842_d_n5, assign13390_e17842_d_n6, assign13390_e17842_d_n8, assign13390_e17842_d_n10, assign13390_e17842_d_n11, assign13390_e17842_d_n12,) = {
    if ((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) {
        let assign13390_e17839: f64 = (locals.var_t4).sqrt();
        let assign13390_e17840: f64 = (1.0 / assign13390_e17839);
        (assign13390_e17840, (-((locals.var_t4_dn0 / (2.0 * assign13390_e17839)) / (assign13390_e17839 * assign13390_e17839))), (-((locals.var_t4_dn2 / (2.0 * assign13390_e17839)) / (assign13390_e17839 * assign13390_e17839))), (-((locals.var_t4_dn4 / (2.0 * assign13390_e17839)) / (assign13390_e17839 * assign13390_e17839))), (-((locals.var_t4_dn5 / (2.0 * assign13390_e17839)) / (assign13390_e17839 * assign13390_e17839))), (-((locals.var_t4_dn6 / (2.0 * assign13390_e17839)) / (assign13390_e17839 * assign13390_e17839))), (-((locals.var_t4_dn8 / (2.0 * assign13390_e17839)) / (assign13390_e17839 * assign13390_e17839))), (-((locals.var_t4_dn10 / (2.0 * assign13390_e17839)) / (assign13390_e17839 * assign13390_e17839))), (-((locals.var_t4_dn11 / (2.0 * assign13390_e17839)) / (assign13390_e17839 * assign13390_e17839))), (-((locals.var_t4_dn12 / (2.0 * assign13390_e17839)) / (assign13390_e17839 * assign13390_e17839))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign13390_e17842;
        locals.var_t5_dn0 = assign13390_e17842_d_n0;
        locals.var_t5_dn2 = assign13390_e17842_d_n2;
        locals.var_t5_dn4 = assign13390_e17842_d_n4;
        locals.var_t5_dn5 = assign13390_e17842_d_n5;
        locals.var_t5_dn6 = assign13390_e17842_d_n6;
        locals.var_t5_dn8 = assign13390_e17842_d_n8;
        locals.var_t5_dn10 = assign13390_e17842_d_n10;
        locals.var_t5_dn11 = assign13390_e17842_d_n11;
        locals.var_t5_dn12 = assign13390_e17842_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign13400_e17857, assign13400_e17857_d_n0, assign13400_e17857_d_n2, assign13400_e17857_d_n4, assign13400_e17857_d_n5, assign13400_e17857_d_n6, assign13400_e17857_d_n8, assign13400_e17857_d_n10, assign13400_e17857_d_n11, assign13400_e17857_d_n12,) = {
    if ((locals.var_guard245 == 0.0) && (locals.var_guard246 == 0.0)) {
        let assign13400_e17850: f64 = (-1.0);
        let assign13400_e17852: f64 = (assign13400_e17850 / p.p114);
        let assign13400_e17854: f64 = (assign13400_e17852 - 1.0);
        let assign13400_e17855: f64 = (locals.var_t4).powf(assign13400_e17854);
        (assign13400_e17855, if 0.0 == 0.0 && ((assign13400_e17854) as f64).is_finite() && ((assign13400_e17854) as f64).fract() == 0.0 { if assign13400_e17854 == 0.0 { 0.0 } else { (assign13400_e17854 * ((locals.var_t4).powf(assign13400_e17854 - 1.0) * locals.var_t4_dn0)) } } else { (assign13400_e17855 * (assign13400_e17854 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13400_e17854) as f64).is_finite() && ((assign13400_e17854) as f64).fract() == 0.0 { if assign13400_e17854 == 0.0 { 0.0 } else { (assign13400_e17854 * ((locals.var_t4).powf(assign13400_e17854 - 1.0) * locals.var_t4_dn2)) } } else { (assign13400_e17855 * (assign13400_e17854 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13400_e17854) as f64).is_finite() && ((assign13400_e17854) as f64).fract() == 0.0 { if assign13400_e17854 == 0.0 { 0.0 } else { (assign13400_e17854 * ((locals.var_t4).powf(assign13400_e17854 - 1.0) * locals.var_t4_dn4)) } } else { (assign13400_e17855 * (assign13400_e17854 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13400_e17854) as f64).is_finite() && ((assign13400_e17854) as f64).fract() == 0.0 { if assign13400_e17854 == 0.0 { 0.0 } else { (assign13400_e17854 * ((locals.var_t4).powf(assign13400_e17854 - 1.0) * locals.var_t4_dn5)) } } else { (assign13400_e17855 * (assign13400_e17854 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13400_e17854) as f64).is_finite() && ((assign13400_e17854) as f64).fract() == 0.0 { if assign13400_e17854 == 0.0 { 0.0 } else { (assign13400_e17854 * ((locals.var_t4).powf(assign13400_e17854 - 1.0) * locals.var_t4_dn6)) } } else { (assign13400_e17855 * (assign13400_e17854 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13400_e17854) as f64).is_finite() && ((assign13400_e17854) as f64).fract() == 0.0 { if assign13400_e17854 == 0.0 { 0.0 } else { (assign13400_e17854 * ((locals.var_t4).powf(assign13400_e17854 - 1.0) * locals.var_t4_dn8)) } } else { (assign13400_e17855 * (assign13400_e17854 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13400_e17854) as f64).is_finite() && ((assign13400_e17854) as f64).fract() == 0.0 { if assign13400_e17854 == 0.0 { 0.0 } else { (assign13400_e17854 * ((locals.var_t4).powf(assign13400_e17854 - 1.0) * locals.var_t4_dn10)) } } else { (assign13400_e17855 * (assign13400_e17854 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13400_e17854) as f64).is_finite() && ((assign13400_e17854) as f64).fract() == 0.0 { if assign13400_e17854 == 0.0 { 0.0 } else { (assign13400_e17854 * ((locals.var_t4).powf(assign13400_e17854 - 1.0) * locals.var_t4_dn11)) } } else { (assign13400_e17855 * (assign13400_e17854 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13400_e17854) as f64).is_finite() && ((assign13400_e17854) as f64).fract() == 0.0 { if assign13400_e17854 == 0.0 { 0.0 } else { (assign13400_e17854 * ((locals.var_t4).powf(assign13400_e17854 - 1.0) * locals.var_t4_dn12)) } } else { (assign13400_e17855 * (assign13400_e17854 * (locals.var_t4_dn12 / locals.var_t4))) },)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign13400_e17857;
        locals.var_t6_dn0 = assign13400_e17857_d_n0;
        locals.var_t6_dn2 = assign13400_e17857_d_n2;
        locals.var_t6_dn4 = assign13400_e17857_d_n4;
        locals.var_t6_dn5 = assign13400_e17857_d_n5;
        locals.var_t6_dn6 = assign13400_e17857_d_n6;
        locals.var_t6_dn8 = assign13400_e17857_d_n8;
        locals.var_t6_dn10 = assign13400_e17857_d_n10;
        locals.var_t6_dn11 = assign13400_e17857_d_n11;
        locals.var_t6_dn12 = assign13400_e17857_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign13410_e17867, assign13410_e17867_d_n0, assign13410_e17867_d_n2, assign13410_e17867_d_n4, assign13410_e17867_d_n5, assign13410_e17867_d_n6, assign13410_e17867_d_n8, assign13410_e17867_d_n10, assign13410_e17867_d_n11, assign13410_e17867_d_n12,) = {
    if ((locals.var_guard245 == 0.0) && (locals.var_guard246 == 0.0)) {
        let assign13410_e17865: f64 = (locals.var_t4 * locals.var_t6);
        (assign13410_e17865, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign13410_e17867;
        locals.var_t5_dn0 = assign13410_e17867_d_n0;
        locals.var_t5_dn2 = assign13410_e17867_d_n2;
        locals.var_t5_dn4 = assign13410_e17867_d_n4;
        locals.var_t5_dn5 = assign13410_e17867_d_n5;
        locals.var_t5_dn6 = assign13410_e17867_d_n6;
        locals.var_t5_dn8 = assign13410_e17867_d_n8;
        locals.var_t5_dn10 = assign13410_e17867_d_n10;
        locals.var_t5_dn11 = assign13410_e17867_d_n11;
        locals.var_t5_dn12 = assign13410_e17867_d_n12;
        locals.var_t5_rv = 0.0;

        let assign13420_e17870: f64 = (locals.var_muun * locals.var_t5);
        locals.var_mu = assign13420_e17870;
        locals.var_mu_dn0 = ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0));
        locals.var_mu_dn2 = ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2));
        locals.var_mu_dn4 = ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4));
        locals.var_mu_dn5 = ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5));
        locals.var_mu_dn6 = ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6));
        locals.var_mu_dn8 = ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8));
        locals.var_mu_dn10 = ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10));
        locals.var_mu_dn11 = ((locals.var_muun_dn11 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn11));
        locals.var_mu_dn12 = ((locals.var_muun_dn12 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn12));
        locals.var_mu_rv = 0.0;

        let assign13430_e17873: f64 = (0.2 * locals.var_vmaxe);
        let assign13430_e17875: f64 = (assign13430_e17873 / locals.var_muunb);
        locals.var_c_mueey0b = assign13430_e17875;
        locals.var_c_mueey0b_dn0 = ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muunb) - (assign13430_e17873 * locals.var_muunb_dn0)) / (locals.var_muunb * locals.var_muunb));
        locals.var_c_mueey0b_dn2 = ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muunb) - (assign13430_e17873 * locals.var_muunb_dn2)) / (locals.var_muunb * locals.var_muunb));
        locals.var_c_mueey0b_dn4 = ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muunb) - (assign13430_e17873 * locals.var_muunb_dn4)) / (locals.var_muunb * locals.var_muunb));
        locals.var_c_mueey0b_dn5 = ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muunb) - (assign13430_e17873 * locals.var_muunb_dn5)) / (locals.var_muunb * locals.var_muunb));
        locals.var_c_mueey0b_dn6 = ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muunb) - (assign13430_e17873 * locals.var_muunb_dn6)) / (locals.var_muunb * locals.var_muunb));
        locals.var_c_mueey0b_dn8 = ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muunb) - (assign13430_e17873 * locals.var_muunb_dn8)) / (locals.var_muunb * locals.var_muunb));
        locals.var_c_mueey0b_dn10 = ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muunb) - (assign13430_e17873 * locals.var_muunb_dn10)) / (locals.var_muunb * locals.var_muunb));
        locals.var_c_mueey0b_dn11 = ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muunb) - (assign13430_e17873 * locals.var_muunb_dn11)) / (locals.var_muunb * locals.var_muunb));
        locals.var_c_mueey0b_dn12 = ((((0.2 * locals.var_vmaxe_dn12) * locals.var_muunb) - (assign13430_e17873 * locals.var_muunb_dn12)) / (locals.var_muunb * locals.var_muunb));
        locals.var_c_mueey0b_rv = 0.0;

        let assign13440_e17880: f64 = (locals.var_qb0 + 1e-50);
        let assign13440_e17881: f64 = (locals.var_beta * assign13440_e17880);
        let assign13440_e17883: f64 = (assign13440_e17881 * locals.var_lch);
        let assign13440_e17884: f64 = (locals.var_idd2 / assign13440_e17883);
        locals.var_ty = assign13440_e17884;
        locals.var_ty_dn0 = (((locals.var_idd2_dn0 * assign13440_e17883) - (locals.var_idd2 * (((locals.var_beta * locals.var_qb0_dn0) * locals.var_lch) + (assign13440_e17881 * locals.var_lch_dn0)))) / (assign13440_e17883 * assign13440_e17883));
        locals.var_ty_dn2 = (((locals.var_idd2_dn2 * assign13440_e17883) - (locals.var_idd2 * (((locals.var_beta * locals.var_qb0_dn2) * locals.var_lch) + (assign13440_e17881 * locals.var_lch_dn2)))) / (assign13440_e17883 * assign13440_e17883));
        locals.var_ty_dn4 = (((locals.var_idd2_dn4 * assign13440_e17883) - (locals.var_idd2 * ((((locals.var_beta_dn4 * assign13440_e17880) + (locals.var_beta * locals.var_qb0_dn4)) * locals.var_lch) + (assign13440_e17881 * locals.var_lch_dn4)))) / (assign13440_e17883 * assign13440_e17883));
        locals.var_ty_dn5 = (((locals.var_idd2_dn5 * assign13440_e17883) - (locals.var_idd2 * (((locals.var_beta * locals.var_qb0_dn5) * locals.var_lch) + (assign13440_e17881 * locals.var_lch_dn5)))) / (assign13440_e17883 * assign13440_e17883));
        locals.var_ty_dn6 = (((locals.var_idd2_dn6 * assign13440_e17883) - (locals.var_idd2 * (((locals.var_beta * locals.var_qb0_dn6) * locals.var_lch) + (assign13440_e17881 * locals.var_lch_dn6)))) / (assign13440_e17883 * assign13440_e17883));
        locals.var_ty_dn8 = (((locals.var_idd2_dn8 * assign13440_e17883) - (locals.var_idd2 * (((locals.var_beta * locals.var_qb0_dn8) * locals.var_lch) + (assign13440_e17881 * locals.var_lch_dn8)))) / (assign13440_e17883 * assign13440_e17883));
        locals.var_ty_dn10 = (((locals.var_idd2_dn10 * assign13440_e17883) - (locals.var_idd2 * (((locals.var_beta * locals.var_qb0_dn10) * locals.var_lch) + (assign13440_e17881 * locals.var_lch_dn10)))) / (assign13440_e17883 * assign13440_e17883));
        locals.var_ty_dn11 = (((locals.var_idd2_dn11 * assign13440_e17883) - (locals.var_idd2 * (((locals.var_beta * locals.var_qb0_dn11) * locals.var_lch) + (assign13440_e17881 * locals.var_lch_dn11)))) / (assign13440_e17883 * assign13440_e17883));
        locals.var_ty_dn12 = (((locals.var_idd2_dn12 * assign13440_e17883) - (locals.var_idd2 * (((locals.var_beta * locals.var_qb0_dn12) * locals.var_lch) + (assign13440_e17881 * locals.var_lch_dn12)))) / (assign13440_e17883 * assign13440_e17883));
        locals.var_ty_rv = 0.0;

        let assign13450_e17887: f64 = (locals.var_ty * locals.var_ty);
        let assign13450_e17890: f64 = (locals.var_c_mueey0b * locals.var_c_mueey0b);
        let assign13450_e17891: f64 = (assign13450_e17887 + assign13450_e17890);
        let assign13450_e17892: f64 = (assign13450_e17891).sqrt();
        locals.var_eyb = assign13450_e17892;
        locals.var_eyb_dn0 = ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_c_mueey0b_dn0 * locals.var_c_mueey0b) + (locals.var_c_mueey0b * locals.var_c_mueey0b_dn0))) / (2.0 * assign13450_e17892));
        locals.var_eyb_dn2 = ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_c_mueey0b_dn2 * locals.var_c_mueey0b) + (locals.var_c_mueey0b * locals.var_c_mueey0b_dn2))) / (2.0 * assign13450_e17892));
        locals.var_eyb_dn4 = ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_c_mueey0b_dn4 * locals.var_c_mueey0b) + (locals.var_c_mueey0b * locals.var_c_mueey0b_dn4))) / (2.0 * assign13450_e17892));
        locals.var_eyb_dn5 = ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_c_mueey0b_dn5 * locals.var_c_mueey0b) + (locals.var_c_mueey0b * locals.var_c_mueey0b_dn5))) / (2.0 * assign13450_e17892));
        locals.var_eyb_dn6 = ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_c_mueey0b_dn6 * locals.var_c_mueey0b) + (locals.var_c_mueey0b * locals.var_c_mueey0b_dn6))) / (2.0 * assign13450_e17892));
        locals.var_eyb_dn8 = ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_c_mueey0b_dn8 * locals.var_c_mueey0b) + (locals.var_c_mueey0b * locals.var_c_mueey0b_dn8))) / (2.0 * assign13450_e17892));
        locals.var_eyb_dn10 = ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_c_mueey0b_dn10 * locals.var_c_mueey0b) + (locals.var_c_mueey0b * locals.var_c_mueey0b_dn10))) / (2.0 * assign13450_e17892));
        locals.var_eyb_dn11 = ((((locals.var_ty_dn11 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn11)) + ((locals.var_c_mueey0b_dn11 * locals.var_c_mueey0b) + (locals.var_c_mueey0b * locals.var_c_mueey0b_dn11))) / (2.0 * assign13450_e17892));
        locals.var_eyb_dn12 = ((((locals.var_ty_dn12 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn12)) + ((locals.var_c_mueey0b_dn12 * locals.var_c_mueey0b) + (locals.var_c_mueey0b * locals.var_c_mueey0b_dn12))) / (2.0 * assign13450_e17892));
        locals.var_eyb_rv = 0.0;

        let assign13460_e17895: f64 = (locals.var_muunb * locals.var_eyb);
        locals.var_em = assign13460_e17895;
        locals.var_em_dn0 = ((locals.var_muunb_dn0 * locals.var_eyb) + (locals.var_muunb * locals.var_eyb_dn0));
        locals.var_em_dn2 = ((locals.var_muunb_dn2 * locals.var_eyb) + (locals.var_muunb * locals.var_eyb_dn2));
        locals.var_em_dn4 = ((locals.var_muunb_dn4 * locals.var_eyb) + (locals.var_muunb * locals.var_eyb_dn4));
        locals.var_em_dn5 = ((locals.var_muunb_dn5 * locals.var_eyb) + (locals.var_muunb * locals.var_eyb_dn5));
        locals.var_em_dn6 = ((locals.var_muunb_dn6 * locals.var_eyb) + (locals.var_muunb * locals.var_eyb_dn6));
        locals.var_em_dn8 = ((locals.var_muunb_dn8 * locals.var_eyb) + (locals.var_muunb * locals.var_eyb_dn8));
        locals.var_em_dn10 = ((locals.var_muunb_dn10 * locals.var_eyb) + (locals.var_muunb * locals.var_eyb_dn10));
        locals.var_em_dn11 = ((locals.var_muunb_dn11 * locals.var_eyb) + (locals.var_muunb * locals.var_eyb_dn11));
        locals.var_em_dn12 = ((locals.var_muunb_dn12 * locals.var_eyb) + (locals.var_muunb * locals.var_eyb_dn12));
        locals.var_em_rv = 0.0;

        let assign13470_e17898: f64 = (locals.var_em / locals.var_vmaxe);
        locals.var_t1 = assign13470_e17898;
        locals.var_t1_dn0 = (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn2 = (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn4 = (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn5 = (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn6 = (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn8 = (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn10 = (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn11 = (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_dn12 = (((locals.var_em_dn12 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn12)) / (locals.var_vmaxe * locals.var_vmaxe));
        locals.var_t1_rv = 0.0;

        let assign13480_e17902: f64 = (10.0 * 2.220446049250313e-16);
        let assign13480_e17903: f64 = (1.0 - assign13480_e17902);
        let assign13480_e17910: f64 = (10.0 * 2.220446049250313e-16);
        let assign13480_e17911: f64 = (1.0 + assign13480_e17910);
        let assign13480_e17913: f64 = if ((assign13480_e17903 <= p.p114) && (p.p114 <= assign13480_e17911)) { 1.0 } else { 0.0 };
        locals.var_guard247 = assign13480_e17913;
        locals.var_guard247_rv = 0.0;

        let (assign13490_e17917, assign13490_e17917_d_n0, assign13490_e17917_d_n2, assign13490_e17917_d_n4, assign13490_e17917_d_n5, assign13490_e17917_d_n6, assign13490_e17917_d_n8, assign13490_e17917_d_n10, assign13490_e17917_d_n11, assign13490_e17917_d_n12,) = {
    if (locals.var_guard247 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign13490_e17917;
        locals.var_t3_dn0 = assign13490_e17917_d_n0;
        locals.var_t3_dn2 = assign13490_e17917_d_n2;
        locals.var_t3_dn4 = assign13490_e17917_d_n4;
        locals.var_t3_dn5 = assign13490_e17917_d_n5;
        locals.var_t3_dn6 = assign13490_e17917_d_n6;
        locals.var_t3_dn8 = assign13490_e17917_d_n8;
        locals.var_t3_dn10 = assign13490_e17917_d_n10;
        locals.var_t3_dn11 = assign13490_e17917_d_n11;
        locals.var_t3_dn12 = assign13490_e17917_d_n12;
        locals.var_t3_rv = 0.0;

        let assign13500_e17921: f64 = (10.0 * 2.220446049250313e-16);
        let assign13500_e17922: f64 = (2.0 - assign13500_e17921);
        let assign13500_e17929: f64 = (10.0 * 2.220446049250313e-16);
        let assign13500_e17930: f64 = (2.0 + assign13500_e17929);
        let assign13500_e17932: f64 = if ((assign13500_e17922 <= p.p114) && (p.p114 <= assign13500_e17930)) { 1.0 } else { 0.0 };
        locals.var_guard248 = assign13500_e17932;
        locals.var_guard248_rv = 0.0;

        let (assign13510_e17939, assign13510_e17939_d_n0, assign13510_e17939_d_n2, assign13510_e17939_d_n4, assign13510_e17939_d_n5, assign13510_e17939_d_n6, assign13510_e17939_d_n8, assign13510_e17939_d_n10, assign13510_e17939_d_n11, assign13510_e17939_d_n12,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign13510_e17939;
        locals.var_t3_dn0 = assign13510_e17939_d_n0;
        locals.var_t3_dn2 = assign13510_e17939_d_n2;
        locals.var_t3_dn4 = assign13510_e17939_d_n4;
        locals.var_t3_dn5 = assign13510_e17939_d_n5;
        locals.var_t3_dn6 = assign13510_e17939_d_n6;
        locals.var_t3_dn8 = assign13510_e17939_d_n8;
        locals.var_t3_dn10 = assign13510_e17939_d_n10;
        locals.var_t3_dn11 = assign13510_e17939_d_n11;
        locals.var_t3_dn12 = assign13510_e17939_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign13520_e17951, assign13520_e17951_d_n0, assign13520_e17951_d_n2, assign13520_e17951_d_n4, assign13520_e17951_d_n5, assign13520_e17951_d_n6, assign13520_e17951_d_n8, assign13520_e17951_d_n10, assign13520_e17951_d_n11, assign13520_e17951_d_n12,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 == 0.0)) {
        let assign13520_e17948: f64 = (p.p114 - 1.0);
        let assign13520_e17949: f64 = (locals.var_t1).powf(assign13520_e17948);
        (assign13520_e17949, if 0.0 == 0.0 && ((assign13520_e17948) as f64).is_finite() && ((assign13520_e17948) as f64).fract() == 0.0 { if assign13520_e17948 == 0.0 { 0.0 } else { (assign13520_e17948 * ((locals.var_t1).powf(assign13520_e17948 - 1.0) * locals.var_t1_dn0)) } } else { (assign13520_e17949 * (assign13520_e17948 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13520_e17948) as f64).is_finite() && ((assign13520_e17948) as f64).fract() == 0.0 { if assign13520_e17948 == 0.0 { 0.0 } else { (assign13520_e17948 * ((locals.var_t1).powf(assign13520_e17948 - 1.0) * locals.var_t1_dn2)) } } else { (assign13520_e17949 * (assign13520_e17948 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13520_e17948) as f64).is_finite() && ((assign13520_e17948) as f64).fract() == 0.0 { if assign13520_e17948 == 0.0 { 0.0 } else { (assign13520_e17948 * ((locals.var_t1).powf(assign13520_e17948 - 1.0) * locals.var_t1_dn4)) } } else { (assign13520_e17949 * (assign13520_e17948 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13520_e17948) as f64).is_finite() && ((assign13520_e17948) as f64).fract() == 0.0 { if assign13520_e17948 == 0.0 { 0.0 } else { (assign13520_e17948 * ((locals.var_t1).powf(assign13520_e17948 - 1.0) * locals.var_t1_dn5)) } } else { (assign13520_e17949 * (assign13520_e17948 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13520_e17948) as f64).is_finite() && ((assign13520_e17948) as f64).fract() == 0.0 { if assign13520_e17948 == 0.0 { 0.0 } else { (assign13520_e17948 * ((locals.var_t1).powf(assign13520_e17948 - 1.0) * locals.var_t1_dn6)) } } else { (assign13520_e17949 * (assign13520_e17948 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13520_e17948) as f64).is_finite() && ((assign13520_e17948) as f64).fract() == 0.0 { if assign13520_e17948 == 0.0 { 0.0 } else { (assign13520_e17948 * ((locals.var_t1).powf(assign13520_e17948 - 1.0) * locals.var_t1_dn8)) } } else { (assign13520_e17949 * (assign13520_e17948 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13520_e17948) as f64).is_finite() && ((assign13520_e17948) as f64).fract() == 0.0 { if assign13520_e17948 == 0.0 { 0.0 } else { (assign13520_e17948 * ((locals.var_t1).powf(assign13520_e17948 - 1.0) * locals.var_t1_dn10)) } } else { (assign13520_e17949 * (assign13520_e17948 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13520_e17948) as f64).is_finite() && ((assign13520_e17948) as f64).fract() == 0.0 { if assign13520_e17948 == 0.0 { 0.0 } else { (assign13520_e17948 * ((locals.var_t1).powf(assign13520_e17948 - 1.0) * locals.var_t1_dn11)) } } else { (assign13520_e17949 * (assign13520_e17948 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign13520_e17948) as f64).is_finite() && ((assign13520_e17948) as f64).fract() == 0.0 { if assign13520_e17948 == 0.0 { 0.0 } else { (assign13520_e17948 * ((locals.var_t1).powf(assign13520_e17948 - 1.0) * locals.var_t1_dn12)) } } else { (assign13520_e17949 * (assign13520_e17948 * (locals.var_t1_dn12 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign13520_e17951;
        locals.var_t3_dn0 = assign13520_e17951_d_n0;
        locals.var_t3_dn2 = assign13520_e17951_d_n2;
        locals.var_t3_dn4 = assign13520_e17951_d_n4;
        locals.var_t3_dn5 = assign13520_e17951_d_n5;
        locals.var_t3_dn6 = assign13520_e17951_d_n6;
        locals.var_t3_dn8 = assign13520_e17951_d_n8;
        locals.var_t3_dn10 = assign13520_e17951_d_n10;
        locals.var_t3_dn11 = assign13520_e17951_d_n11;
        locals.var_t3_dn12 = assign13520_e17951_d_n12;
        locals.var_t3_rv = 0.0;

        let assign13530_e17955: f64 = (locals.var_t1 * locals.var_t3);
        let assign13530_e17956: f64 = (1.0 + assign13530_e17955);
        locals.var_t4 = assign13530_e17956;
        locals.var_t4_dn0 = ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0));
        locals.var_t4_dn2 = ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2));
        locals.var_t4_dn4 = ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4));
        locals.var_t4_dn5 = ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5));
        locals.var_t4_dn6 = ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6));
        locals.var_t4_dn8 = ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8));
        locals.var_t4_dn10 = ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10));
        locals.var_t4_dn11 = ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11));
        locals.var_t4_dn12 = ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12));
        locals.var_t4_rv = 0.0;

        let assign13540_e17960: f64 = (10.0 * 2.220446049250313e-16);
        let assign13540_e17961: f64 = (1.0 - assign13540_e17960);
        let assign13540_e17968: f64 = (10.0 * 2.220446049250313e-16);
        let assign13540_e17969: f64 = (1.0 + assign13540_e17968);
        let assign13540_e17971: f64 = if ((assign13540_e17961 <= p.p114) && (p.p114 <= assign13540_e17969)) { 1.0 } else { 0.0 };
        locals.var_guard249 = assign13540_e17971;
        locals.var_guard249_rv = 0.0;

        let (assign13550_e17977, assign13550_e17977_d_n0, assign13550_e17977_d_n2, assign13550_e17977_d_n4, assign13550_e17977_d_n5, assign13550_e17977_d_n6, assign13550_e17977_d_n8, assign13550_e17977_d_n10, assign13550_e17977_d_n11, assign13550_e17977_d_n12,) = {
    if (locals.var_guard249 != 0.0) {
        let assign13550_e17975: f64 = (1.0 / locals.var_t4);
        (assign13550_e17975, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign13550_e17977;
        locals.var_t5_dn0 = assign13550_e17977_d_n0;
        locals.var_t5_dn2 = assign13550_e17977_d_n2;
        locals.var_t5_dn4 = assign13550_e17977_d_n4;
        locals.var_t5_dn5 = assign13550_e17977_d_n5;
        locals.var_t5_dn6 = assign13550_e17977_d_n6;
        locals.var_t5_dn8 = assign13550_e17977_d_n8;
        locals.var_t5_dn10 = assign13550_e17977_d_n10;
        locals.var_t5_dn11 = assign13550_e17977_d_n11;
        locals.var_t5_dn12 = assign13550_e17977_d_n12;
        locals.var_t5_rv = 0.0;

        let assign13560_e17981: f64 = (10.0 * 2.220446049250313e-16);
        let assign13560_e17982: f64 = (2.0 - assign13560_e17981);
        let assign13560_e17989: f64 = (10.0 * 2.220446049250313e-16);
        let assign13560_e17990: f64 = (2.0 + assign13560_e17989);
        let assign13560_e17992: f64 = if ((assign13560_e17982 <= p.p114) && (p.p114 <= assign13560_e17990)) { 1.0 } else { 0.0 };
        locals.var_guard250 = assign13560_e17992;
        locals.var_guard250_rv = 0.0;

        let (assign13570_e18002, assign13570_e18002_d_n0, assign13570_e18002_d_n2, assign13570_e18002_d_n4, assign13570_e18002_d_n5, assign13570_e18002_d_n6, assign13570_e18002_d_n8, assign13570_e18002_d_n10, assign13570_e18002_d_n11, assign13570_e18002_d_n12,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 != 0.0)) {
        let assign13570_e17999: f64 = (locals.var_t4).sqrt();
        let assign13570_e18000: f64 = (1.0 / assign13570_e17999);
        (assign13570_e18000, (-((locals.var_t4_dn0 / (2.0 * assign13570_e17999)) / (assign13570_e17999 * assign13570_e17999))), (-((locals.var_t4_dn2 / (2.0 * assign13570_e17999)) / (assign13570_e17999 * assign13570_e17999))), (-((locals.var_t4_dn4 / (2.0 * assign13570_e17999)) / (assign13570_e17999 * assign13570_e17999))), (-((locals.var_t4_dn5 / (2.0 * assign13570_e17999)) / (assign13570_e17999 * assign13570_e17999))), (-((locals.var_t4_dn6 / (2.0 * assign13570_e17999)) / (assign13570_e17999 * assign13570_e17999))), (-((locals.var_t4_dn8 / (2.0 * assign13570_e17999)) / (assign13570_e17999 * assign13570_e17999))), (-((locals.var_t4_dn10 / (2.0 * assign13570_e17999)) / (assign13570_e17999 * assign13570_e17999))), (-((locals.var_t4_dn11 / (2.0 * assign13570_e17999)) / (assign13570_e17999 * assign13570_e17999))), (-((locals.var_t4_dn12 / (2.0 * assign13570_e17999)) / (assign13570_e17999 * assign13570_e17999))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign13570_e18002;
        locals.var_t5_dn0 = assign13570_e18002_d_n0;
        locals.var_t5_dn2 = assign13570_e18002_d_n2;
        locals.var_t5_dn4 = assign13570_e18002_d_n4;
        locals.var_t5_dn5 = assign13570_e18002_d_n5;
        locals.var_t5_dn6 = assign13570_e18002_d_n6;
        locals.var_t5_dn8 = assign13570_e18002_d_n8;
        locals.var_t5_dn10 = assign13570_e18002_d_n10;
        locals.var_t5_dn11 = assign13570_e18002_d_n11;
        locals.var_t5_dn12 = assign13570_e18002_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign13580_e18017, assign13580_e18017_d_n0, assign13580_e18017_d_n2, assign13580_e18017_d_n4, assign13580_e18017_d_n5, assign13580_e18017_d_n6, assign13580_e18017_d_n8, assign13580_e18017_d_n10, assign13580_e18017_d_n11, assign13580_e18017_d_n12,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 == 0.0)) {
        let assign13580_e18010: f64 = (-1.0);
        let assign13580_e18012: f64 = (assign13580_e18010 / p.p114);
        let assign13580_e18014: f64 = (assign13580_e18012 - 1.0);
        let assign13580_e18015: f64 = (locals.var_t4).powf(assign13580_e18014);
        (assign13580_e18015, if 0.0 == 0.0 && ((assign13580_e18014) as f64).is_finite() && ((assign13580_e18014) as f64).fract() == 0.0 { if assign13580_e18014 == 0.0 { 0.0 } else { (assign13580_e18014 * ((locals.var_t4).powf(assign13580_e18014 - 1.0) * locals.var_t4_dn0)) } } else { (assign13580_e18015 * (assign13580_e18014 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13580_e18014) as f64).is_finite() && ((assign13580_e18014) as f64).fract() == 0.0 { if assign13580_e18014 == 0.0 { 0.0 } else { (assign13580_e18014 * ((locals.var_t4).powf(assign13580_e18014 - 1.0) * locals.var_t4_dn2)) } } else { (assign13580_e18015 * (assign13580_e18014 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13580_e18014) as f64).is_finite() && ((assign13580_e18014) as f64).fract() == 0.0 { if assign13580_e18014 == 0.0 { 0.0 } else { (assign13580_e18014 * ((locals.var_t4).powf(assign13580_e18014 - 1.0) * locals.var_t4_dn4)) } } else { (assign13580_e18015 * (assign13580_e18014 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13580_e18014) as f64).is_finite() && ((assign13580_e18014) as f64).fract() == 0.0 { if assign13580_e18014 == 0.0 { 0.0 } else { (assign13580_e18014 * ((locals.var_t4).powf(assign13580_e18014 - 1.0) * locals.var_t4_dn5)) } } else { (assign13580_e18015 * (assign13580_e18014 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13580_e18014) as f64).is_finite() && ((assign13580_e18014) as f64).fract() == 0.0 { if assign13580_e18014 == 0.0 { 0.0 } else { (assign13580_e18014 * ((locals.var_t4).powf(assign13580_e18014 - 1.0) * locals.var_t4_dn6)) } } else { (assign13580_e18015 * (assign13580_e18014 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13580_e18014) as f64).is_finite() && ((assign13580_e18014) as f64).fract() == 0.0 { if assign13580_e18014 == 0.0 { 0.0 } else { (assign13580_e18014 * ((locals.var_t4).powf(assign13580_e18014 - 1.0) * locals.var_t4_dn8)) } } else { (assign13580_e18015 * (assign13580_e18014 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13580_e18014) as f64).is_finite() && ((assign13580_e18014) as f64).fract() == 0.0 { if assign13580_e18014 == 0.0 { 0.0 } else { (assign13580_e18014 * ((locals.var_t4).powf(assign13580_e18014 - 1.0) * locals.var_t4_dn10)) } } else { (assign13580_e18015 * (assign13580_e18014 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13580_e18014) as f64).is_finite() && ((assign13580_e18014) as f64).fract() == 0.0 { if assign13580_e18014 == 0.0 { 0.0 } else { (assign13580_e18014 * ((locals.var_t4).powf(assign13580_e18014 - 1.0) * locals.var_t4_dn11)) } } else { (assign13580_e18015 * (assign13580_e18014 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign13580_e18014) as f64).is_finite() && ((assign13580_e18014) as f64).fract() == 0.0 { if assign13580_e18014 == 0.0 { 0.0 } else { (assign13580_e18014 * ((locals.var_t4).powf(assign13580_e18014 - 1.0) * locals.var_t4_dn12)) } } else { (assign13580_e18015 * (assign13580_e18014 * (locals.var_t4_dn12 / locals.var_t4))) },)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign13580_e18017;
        locals.var_t6_dn0 = assign13580_e18017_d_n0;
        locals.var_t6_dn2 = assign13580_e18017_d_n2;
        locals.var_t6_dn4 = assign13580_e18017_d_n4;
        locals.var_t6_dn5 = assign13580_e18017_d_n5;
        locals.var_t6_dn6 = assign13580_e18017_d_n6;
        locals.var_t6_dn8 = assign13580_e18017_d_n8;
        locals.var_t6_dn10 = assign13580_e18017_d_n10;
        locals.var_t6_dn11 = assign13580_e18017_d_n11;
        locals.var_t6_dn12 = assign13580_e18017_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign13590_e18027, assign13590_e18027_d_n0, assign13590_e18027_d_n2, assign13590_e18027_d_n4, assign13590_e18027_d_n5, assign13590_e18027_d_n6, assign13590_e18027_d_n8, assign13590_e18027_d_n10, assign13590_e18027_d_n11, assign13590_e18027_d_n12,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 == 0.0)) {
        let assign13590_e18025: f64 = (locals.var_t4 * locals.var_t6);
        (assign13590_e18025, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign13590_e18027;
        locals.var_t5_dn0 = assign13590_e18027_d_n0;
        locals.var_t5_dn2 = assign13590_e18027_d_n2;
        locals.var_t5_dn4 = assign13590_e18027_d_n4;
        locals.var_t5_dn5 = assign13590_e18027_d_n5;
        locals.var_t5_dn6 = assign13590_e18027_d_n6;
        locals.var_t5_dn8 = assign13590_e18027_d_n8;
        locals.var_t5_dn10 = assign13590_e18027_d_n10;
        locals.var_t5_dn11 = assign13590_e18027_d_n11;
        locals.var_t5_dn12 = assign13590_e18027_d_n12;
        locals.var_t5_rv = 0.0;

        let assign13600_e18030: f64 = (locals.var_muunb * locals.var_t5);
        locals.var_mub = assign13600_e18030;
        locals.var_mub_dn0 = ((locals.var_muunb_dn0 * locals.var_t5) + (locals.var_muunb * locals.var_t5_dn0));
        locals.var_mub_dn2 = ((locals.var_muunb_dn2 * locals.var_t5) + (locals.var_muunb * locals.var_t5_dn2));
        locals.var_mub_dn4 = ((locals.var_muunb_dn4 * locals.var_t5) + (locals.var_muunb * locals.var_t5_dn4));
        locals.var_mub_dn5 = ((locals.var_muunb_dn5 * locals.var_t5) + (locals.var_muunb * locals.var_t5_dn5));
        locals.var_mub_dn6 = ((locals.var_muunb_dn6 * locals.var_t5) + (locals.var_muunb * locals.var_t5_dn6));
        locals.var_mub_dn8 = ((locals.var_muunb_dn8 * locals.var_t5) + (locals.var_muunb * locals.var_t5_dn8));
        locals.var_mub_dn10 = ((locals.var_muunb_dn10 * locals.var_t5) + (locals.var_muunb * locals.var_t5_dn10));
        locals.var_mub_dn11 = ((locals.var_muunb_dn11 * locals.var_t5) + (locals.var_muunb * locals.var_t5_dn11));
        locals.var_mub_dn12 = ((locals.var_muunb_dn12 * locals.var_t5) + (locals.var_muunb * locals.var_t5_dn12));
        locals.var_mub_rv = 0.0;

        let assign13610_e18033: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign13610_e18036: f64 = (locals.var_leff - locals.var_lred);
        let assign13610_e18037: f64 = (assign13610_e18033 / assign13610_e18036);
        locals.var_betawl = assign13610_e18037;
        locals.var_betawl_dn0 = ((((locals.var_weff_nf_dn0 * locals.var_beta_inv) * assign13610_e18036) - (assign13610_e18033 * (locals.var_leff_dn0 - locals.var_lred_dn0))) / (assign13610_e18036 * assign13610_e18036));
        locals.var_betawl_dn2 = ((((locals.var_weff_nf_dn2 * locals.var_beta_inv) * assign13610_e18036) - (assign13610_e18033 * (locals.var_leff_dn2 - locals.var_lred_dn2))) / (assign13610_e18036 * assign13610_e18036));
        locals.var_betawl_dn4 = (((((locals.var_weff_nf_dn4 * locals.var_beta_inv) + (locals.var_weff_nf * locals.var_beta_inv_dn4)) * assign13610_e18036) - (assign13610_e18033 * (locals.var_leff_dn4 - locals.var_lred_dn4))) / (assign13610_e18036 * assign13610_e18036));
        locals.var_betawl_dn5 = ((((locals.var_weff_nf_dn5 * locals.var_beta_inv) * assign13610_e18036) - (assign13610_e18033 * (locals.var_leff_dn5 - locals.var_lred_dn5))) / (assign13610_e18036 * assign13610_e18036));
        locals.var_betawl_dn6 = ((((locals.var_weff_nf_dn6 * locals.var_beta_inv) * assign13610_e18036) - (assign13610_e18033 * (locals.var_leff_dn6 - locals.var_lred_dn6))) / (assign13610_e18036 * assign13610_e18036));
        locals.var_betawl_dn8 = ((((locals.var_weff_nf_dn8 * locals.var_beta_inv) * assign13610_e18036) - (assign13610_e18033 * (locals.var_leff_dn8 - locals.var_lred_dn8))) / (assign13610_e18036 * assign13610_e18036));
        locals.var_betawl_dn10 = ((((locals.var_weff_nf_dn10 * locals.var_beta_inv) * assign13610_e18036) - (assign13610_e18033 * (locals.var_leff_dn10 - locals.var_lred_dn10))) / (assign13610_e18036 * assign13610_e18036));
        locals.var_betawl_dn11 = ((((locals.var_weff_nf_dn11 * locals.var_beta_inv) * assign13610_e18036) - (assign13610_e18033 * (locals.var_leff_dn11 - locals.var_lred_dn11))) / (assign13610_e18036 * assign13610_e18036));
        locals.var_betawl_dn12 = ((((locals.var_weff_nf_dn12 * locals.var_beta_inv) * assign13610_e18036) - (assign13610_e18033 * (locals.var_leff_dn12 - locals.var_lred_dn12))) / (assign13610_e18036 * assign13610_e18036));
        locals.var_betawl_rv = 0.0;

        let assign13620_e18040: f64 = (locals.var_betawl * locals.var_idd1);
        let assign13620_e18042: f64 = (assign13620_e18040 * locals.var_mu);
        locals.var_ids01 = assign13620_e18042;
        locals.var_ids01_dn0 = ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign13620_e18040 * locals.var_mu_dn0));
        locals.var_ids01_dn2 = ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign13620_e18040 * locals.var_mu_dn2));
        locals.var_ids01_dn4 = ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign13620_e18040 * locals.var_mu_dn4));
        locals.var_ids01_dn5 = ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign13620_e18040 * locals.var_mu_dn5));
        locals.var_ids01_dn6 = ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign13620_e18040 * locals.var_mu_dn6));
        locals.var_ids01_dn8 = ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign13620_e18040 * locals.var_mu_dn8));
        locals.var_ids01_dn10 = ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign13620_e18040 * locals.var_mu_dn10));
        locals.var_ids01_dn11 = ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign13620_e18040 * locals.var_mu_dn11));
        locals.var_ids01_dn12 = ((((locals.var_betawl_dn12 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn12)) * locals.var_mu) + (assign13620_e18040 * locals.var_mu_dn12));
        locals.var_ids01_rv = 0.0;

        let assign13630_e18045: f64 = (locals.var_betawl * locals.var_idd2);
        let assign13630_e18047: f64 = (assign13630_e18045 * locals.var_mub);
        locals.var_ids02 = assign13630_e18047;
        locals.var_ids02_dn0 = ((((locals.var_betawl_dn0 * locals.var_idd2) + (locals.var_betawl * locals.var_idd2_dn0)) * locals.var_mub) + (assign13630_e18045 * locals.var_mub_dn0));
        locals.var_ids02_dn2 = ((((locals.var_betawl_dn2 * locals.var_idd2) + (locals.var_betawl * locals.var_idd2_dn2)) * locals.var_mub) + (assign13630_e18045 * locals.var_mub_dn2));
        locals.var_ids02_dn4 = ((((locals.var_betawl_dn4 * locals.var_idd2) + (locals.var_betawl * locals.var_idd2_dn4)) * locals.var_mub) + (assign13630_e18045 * locals.var_mub_dn4));
        locals.var_ids02_dn5 = ((((locals.var_betawl_dn5 * locals.var_idd2) + (locals.var_betawl * locals.var_idd2_dn5)) * locals.var_mub) + (assign13630_e18045 * locals.var_mub_dn5));
        locals.var_ids02_dn6 = ((((locals.var_betawl_dn6 * locals.var_idd2) + (locals.var_betawl * locals.var_idd2_dn6)) * locals.var_mub) + (assign13630_e18045 * locals.var_mub_dn6));
        locals.var_ids02_dn8 = ((((locals.var_betawl_dn8 * locals.var_idd2) + (locals.var_betawl * locals.var_idd2_dn8)) * locals.var_mub) + (assign13630_e18045 * locals.var_mub_dn8));
        locals.var_ids02_dn10 = ((((locals.var_betawl_dn10 * locals.var_idd2) + (locals.var_betawl * locals.var_idd2_dn10)) * locals.var_mub) + (assign13630_e18045 * locals.var_mub_dn10));
        locals.var_ids02_dn11 = ((((locals.var_betawl_dn11 * locals.var_idd2) + (locals.var_betawl * locals.var_idd2_dn11)) * locals.var_mub) + (assign13630_e18045 * locals.var_mub_dn11));
        locals.var_ids02_dn12 = ((((locals.var_betawl_dn12 * locals.var_idd2) + (locals.var_betawl * locals.var_idd2_dn12)) * locals.var_mub) + (assign13630_e18045 * locals.var_mub_dn12));
        locals.var_ids02_rv = 0.0;

        let assign13640_e18050: f64 = (locals.var_ids01 + locals.var_ids02);
        locals.var_ids0 = assign13640_e18050;
        locals.var_ids0_dn0 = (locals.var_ids01_dn0 + locals.var_ids02_dn0);
        locals.var_ids0_dn2 = (locals.var_ids01_dn2 + locals.var_ids02_dn2);
        locals.var_ids0_dn4 = (locals.var_ids01_dn4 + locals.var_ids02_dn4);
        locals.var_ids0_dn5 = (locals.var_ids01_dn5 + locals.var_ids02_dn5);
        locals.var_ids0_dn6 = (locals.var_ids01_dn6 + locals.var_ids02_dn6);
        locals.var_ids0_dn8 = (locals.var_ids01_dn8 + locals.var_ids02_dn8);
        locals.var_ids0_dn10 = (locals.var_ids01_dn10 + locals.var_ids02_dn10);
        locals.var_ids0_dn11 = (locals.var_ids01_dn11 + locals.var_ids02_dn11);
        locals.var_ids0_dn12 = (locals.var_ids01_dn12 + locals.var_ids02_dn12);
        locals.var_ids0_rv = 0.0;

        locals.var_idspt = 0.0;
        locals.var_idspt_dn0 = 0.0;
        locals.var_idspt_dn2 = 0.0;
        locals.var_idspt_dn4 = 0.0;
        locals.var_idspt_dn5 = 0.0;
        locals.var_idspt_dn6 = 0.0;
        locals.var_idspt_dn8 = 0.0;
        locals.var_idspt_dn10 = 0.0;
        locals.var_idspt_dn11 = 0.0;
        locals.var_idspt_dn12 = 0.0;
        locals.var_idspt_rv = 0.0;

        locals.var_ids1 = 0.0;
        locals.var_ids1_dn0 = 0.0;
        locals.var_ids1_dn2 = 0.0;
        locals.var_ids1_dn4 = 0.0;
        locals.var_ids1_dn5 = 0.0;
        locals.var_ids1_dn6 = 0.0;
        locals.var_ids1_dn8 = 0.0;
        locals.var_ids1_dn10 = 0.0;
        locals.var_ids1_dn11 = 0.0;
        locals.var_ids1_dn12 = 0.0;
        locals.var_ids1_rv = 0.0;

        locals.var_ids1_fac = 0.0;
        locals.var_ids1_fac_dn0 = 0.0;
        locals.var_ids1_fac_dn2 = 0.0;
        locals.var_ids1_fac_dn4 = 0.0;
        locals.var_ids1_fac_dn5 = 0.0;
        locals.var_ids1_fac_dn6 = 0.0;
        locals.var_ids1_fac_dn8 = 0.0;
        locals.var_ids1_fac_dn10 = 0.0;
        locals.var_ids1_fac_dn11 = 0.0;
        locals.var_ids1_fac_dn12 = 0.0;
        locals.var_ids1_fac_rv = 0.0;

        locals.var_ids2_fac = 0.0;
        locals.var_ids2_fac_dn0 = 0.0;
        locals.var_ids2_fac_dn2 = 0.0;
        locals.var_ids2_fac_dn4 = 0.0;
        locals.var_ids2_fac_dn5 = 0.0;
        locals.var_ids2_fac_dn6 = 0.0;
        locals.var_ids2_fac_dn8 = 0.0;
        locals.var_ids2_fac_dn10 = 0.0;
        locals.var_ids2_fac_dn11 = 0.0;
        locals.var_ids2_fac_dn12 = 0.0;
        locals.var_ids2_fac_rv = 0.0;

        let assign13690_e18057: f64 = if p.p239 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign13690_e18057;
        locals.var_guard251_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_57(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13700_e18065, assign13700_e18065_d_n0, assign13700_e18065_d_n2, assign13700_e18065_d_n4, assign13700_e18065_d_n5, assign13700_e18065_d_n6, assign13700_e18065_d_n8, assign13700_e18065_d_n10, assign13700_e18065_d_n11, assign13700_e18065_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13700_e18062: f64 = (locals.var_vds - locals.var_pds);
        let assign13700_e18063: f64 = (0.5 * assign13700_e18062);
        (assign13700_e18063, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn12 - locals.var_pds_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign13700_e18065;
        locals.var_t1_dn0 = assign13700_e18065_d_n0;
        locals.var_t1_dn2 = assign13700_e18065_d_n2;
        locals.var_t1_dn4 = assign13700_e18065_d_n4;
        locals.var_t1_dn5 = assign13700_e18065_d_n5;
        locals.var_t1_dn6 = assign13700_e18065_d_n6;
        locals.var_t1_dn8 = assign13700_e18065_d_n8;
        locals.var_t1_dn10 = assign13700_e18065_d_n10;
        locals.var_t1_dn11 = assign13700_e18065_d_n11;
        locals.var_t1_dn12 = assign13700_e18065_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign13710_e18073, assign13710_e18073_d_n0, assign13710_e18073_d_n2, assign13710_e18073_d_n4, assign13710_e18073_d_n5, assign13710_e18073_d_n6, assign13710_e18073_d_n8, assign13710_e18073_d_n10, assign13710_e18073_d_n11, assign13710_e18073_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13710_e18069: f64 = (2.0 * locals.var_t1);
        let assign13710_e18071: f64 = (assign13710_e18069 / 0.01);
        (assign13710_e18071, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn11) / 0.01), ((2.0 * locals.var_t1_dn12) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign13710_e18073;
        locals.var_tmf1_dn0 = assign13710_e18073_d_n0;
        locals.var_tmf1_dn2 = assign13710_e18073_d_n2;
        locals.var_tmf1_dn4 = assign13710_e18073_d_n4;
        locals.var_tmf1_dn5 = assign13710_e18073_d_n5;
        locals.var_tmf1_dn6 = assign13710_e18073_d_n6;
        locals.var_tmf1_dn8 = assign13710_e18073_d_n8;
        locals.var_tmf1_dn10 = assign13710_e18073_d_n10;
        locals.var_tmf1_dn11 = assign13710_e18073_d_n11;
        locals.var_tmf1_dn12 = assign13710_e18073_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign13720_e18113, assign13720_e18113_d_n0, assign13720_e18113_d_n2, assign13720_e18113_d_n4, assign13720_e18113_d_n5, assign13720_e18113_d_n6, assign13720_e18113_d_n8, assign13720_e18113_d_n10, assign13720_e18113_d_n11, assign13720_e18113_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13720_e18079: f64 = (1.0 / 2.0);
        let assign13720_e18083: f64 = (1.0 / 6.0);
        let assign13720_e18087: f64 = (1.0 / 24.0);
        let assign13720_e18091: f64 = (1.0 / 120.0);
        let assign13720_e18095: f64 = (1.0 / 720.0);
        let assign13720_e18099: f64 = (1.0 / 5040.0);
        let assign13720_e18100: f64 = (locals.var_tmf1 * assign13720_e18099);
        let assign13720_e18101: f64 = (assign13720_e18095 + assign13720_e18100);
        let assign13720_e18102: f64 = (locals.var_tmf1 * assign13720_e18101);
        let assign13720_e18103: f64 = (assign13720_e18091 + assign13720_e18102);
        let assign13720_e18104: f64 = (locals.var_tmf1 * assign13720_e18103);
        let assign13720_e18105: f64 = (assign13720_e18087 + assign13720_e18104);
        let assign13720_e18106: f64 = (locals.var_tmf1 * assign13720_e18105);
        let assign13720_e18107: f64 = (assign13720_e18083 + assign13720_e18106);
        let assign13720_e18108: f64 = (locals.var_tmf1 * assign13720_e18107);
        let assign13720_e18109: f64 = (assign13720_e18079 + assign13720_e18108);
        let assign13720_e18110: f64 = (locals.var_tmf1 * assign13720_e18109);
        let assign13720_e18111: f64 = (1.0 + assign13720_e18110);
        (assign13720_e18111, ((locals.var_tmf1_dn0 * assign13720_e18109) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign13720_e18107) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign13720_e18105) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign13720_e18103) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign13720_e18101) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign13720_e18099))))))))))), ((locals.var_tmf1_dn2 * assign13720_e18109) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign13720_e18107) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign13720_e18105) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign13720_e18103) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign13720_e18101) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign13720_e18099))))))))))), ((locals.var_tmf1_dn4 * assign13720_e18109) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign13720_e18107) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign13720_e18105) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign13720_e18103) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign13720_e18101) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign13720_e18099))))))))))), ((locals.var_tmf1_dn5 * assign13720_e18109) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign13720_e18107) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign13720_e18105) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign13720_e18103) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign13720_e18101) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign13720_e18099))))))))))), ((locals.var_tmf1_dn6 * assign13720_e18109) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign13720_e18107) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign13720_e18105) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign13720_e18103) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign13720_e18101) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign13720_e18099))))))))))), ((locals.var_tmf1_dn8 * assign13720_e18109) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign13720_e18107) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign13720_e18105) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign13720_e18103) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign13720_e18101) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign13720_e18099))))))))))), ((locals.var_tmf1_dn10 * assign13720_e18109) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign13720_e18107) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign13720_e18105) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign13720_e18103) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign13720_e18101) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign13720_e18099))))))))))), ((locals.var_tmf1_dn11 * assign13720_e18109) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign13720_e18107) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign13720_e18105) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign13720_e18103) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign13720_e18101) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign13720_e18099))))))))))), ((locals.var_tmf1_dn12 * assign13720_e18109) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign13720_e18107) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign13720_e18105) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign13720_e18103) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign13720_e18101) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign13720_e18099))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign13720_e18113;
        locals.var_tmf2_dn0 = assign13720_e18113_d_n0;
        locals.var_tmf2_dn2 = assign13720_e18113_d_n2;
        locals.var_tmf2_dn4 = assign13720_e18113_d_n4;
        locals.var_tmf2_dn5 = assign13720_e18113_d_n5;
        locals.var_tmf2_dn6 = assign13720_e18113_d_n6;
        locals.var_tmf2_dn8 = assign13720_e18113_d_n8;
        locals.var_tmf2_dn10 = assign13720_e18113_d_n10;
        locals.var_tmf2_dn11 = assign13720_e18113_d_n11;
        locals.var_tmf2_dn12 = assign13720_e18113_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign13730_e18149, assign13730_e18149_d_n0, assign13730_e18149_d_n2, assign13730_e18149_d_n4, assign13730_e18149_d_n5, assign13730_e18149_d_n6, assign13730_e18149_d_n8, assign13730_e18149_d_n10, assign13730_e18149_d_n11, assign13730_e18149_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13730_e18117: f64 = (1.0 / 2.0);
        let assign13730_e18121: f64 = (1.0 / 3.0);
        let assign13730_e18125: f64 = (1.0 / 8.0);
        let assign13730_e18129: f64 = (1.0 / 30.0);
        let assign13730_e18133: f64 = (1.0 / 144.0);
        let assign13730_e18137: f64 = (1.0 / 840.0);
        let assign13730_e18138: f64 = (locals.var_tmf1 * assign13730_e18137);
        let assign13730_e18139: f64 = (assign13730_e18133 + assign13730_e18138);
        let assign13730_e18140: f64 = (locals.var_tmf1 * assign13730_e18139);
        let assign13730_e18141: f64 = (assign13730_e18129 + assign13730_e18140);
        let assign13730_e18142: f64 = (locals.var_tmf1 * assign13730_e18141);
        let assign13730_e18143: f64 = (assign13730_e18125 + assign13730_e18142);
        let assign13730_e18144: f64 = (locals.var_tmf1 * assign13730_e18143);
        let assign13730_e18145: f64 = (assign13730_e18121 + assign13730_e18144);
        let assign13730_e18146: f64 = (locals.var_tmf1 * assign13730_e18145);
        let assign13730_e18147: f64 = (assign13730_e18117 + assign13730_e18146);
        (assign13730_e18147, ((locals.var_tmf1_dn0 * assign13730_e18145) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign13730_e18143) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign13730_e18141) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign13730_e18139) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign13730_e18137))))))))), ((locals.var_tmf1_dn2 * assign13730_e18145) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign13730_e18143) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign13730_e18141) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign13730_e18139) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign13730_e18137))))))))), ((locals.var_tmf1_dn4 * assign13730_e18145) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign13730_e18143) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign13730_e18141) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign13730_e18139) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign13730_e18137))))))))), ((locals.var_tmf1_dn5 * assign13730_e18145) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign13730_e18143) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign13730_e18141) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign13730_e18139) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign13730_e18137))))))))), ((locals.var_tmf1_dn6 * assign13730_e18145) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign13730_e18143) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign13730_e18141) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign13730_e18139) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign13730_e18137))))))))), ((locals.var_tmf1_dn8 * assign13730_e18145) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign13730_e18143) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign13730_e18141) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign13730_e18139) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign13730_e18137))))))))), ((locals.var_tmf1_dn10 * assign13730_e18145) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign13730_e18143) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign13730_e18141) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign13730_e18139) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign13730_e18137))))))))), ((locals.var_tmf1_dn11 * assign13730_e18145) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign13730_e18143) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign13730_e18141) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign13730_e18139) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign13730_e18137))))))))), ((locals.var_tmf1_dn12 * assign13730_e18145) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign13730_e18143) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign13730_e18141) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign13730_e18139) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign13730_e18137))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn8, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12,)
    }
};
        locals.var_tmf3 = assign13730_e18149;
        locals.var_tmf3_dn0 = assign13730_e18149_d_n0;
        locals.var_tmf3_dn2 = assign13730_e18149_d_n2;
        locals.var_tmf3_dn4 = assign13730_e18149_d_n4;
        locals.var_tmf3_dn5 = assign13730_e18149_d_n5;
        locals.var_tmf3_dn6 = assign13730_e18149_d_n6;
        locals.var_tmf3_dn8 = assign13730_e18149_d_n8;
        locals.var_tmf3_dn10 = assign13730_e18149_d_n10;
        locals.var_tmf3_dn11 = assign13730_e18149_d_n11;
        locals.var_tmf3_dn12 = assign13730_e18149_d_n12;
        locals.var_tmf3_rv = 0.0;

        let (assign13740_e18155, assign13740_e18155_d_n0, assign13740_e18155_d_n2, assign13740_e18155_d_n4, assign13740_e18155_d_n5, assign13740_e18155_d_n6, assign13740_e18155_d_n8, assign13740_e18155_d_n10, assign13740_e18155_d_n11, assign13740_e18155_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13740_e18153: f64 = (0.01 / locals.var_tmf2);
        (assign13740_e18153, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign13740_e18155;
        locals.var_t6_dn0 = assign13740_e18155_d_n0;
        locals.var_t6_dn2 = assign13740_e18155_d_n2;
        locals.var_t6_dn4 = assign13740_e18155_d_n4;
        locals.var_t6_dn5 = assign13740_e18155_d_n5;
        locals.var_t6_dn6 = assign13740_e18155_d_n6;
        locals.var_t6_dn8 = assign13740_e18155_d_n8;
        locals.var_t6_dn10 = assign13740_e18155_d_n10;
        locals.var_t6_dn11 = assign13740_e18155_d_n11;
        locals.var_t6_dn12 = assign13740_e18155_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign13750_e18166, assign13750_e18166_d_n0, assign13750_e18166_d_n2, assign13750_e18166_d_n4, assign13750_e18166_d_n5, assign13750_e18166_d_n6, assign13750_e18166_d_n8, assign13750_e18166_d_n10, assign13750_e18166_d_n11, assign13750_e18166_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13750_e18158: f64 = (-2.0);
        let assign13750_e18160: f64 = (assign13750_e18158 * locals.var_tmf3);
        let assign13750_e18163: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign13750_e18164: f64 = (assign13750_e18160 / assign13750_e18163);
        (assign13750_e18164, ((((assign13750_e18158 * locals.var_tmf3_dn0) * assign13750_e18163) - (assign13750_e18160 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign13750_e18163 * assign13750_e18163)), ((((assign13750_e18158 * locals.var_tmf3_dn2) * assign13750_e18163) - (assign13750_e18160 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign13750_e18163 * assign13750_e18163)), ((((assign13750_e18158 * locals.var_tmf3_dn4) * assign13750_e18163) - (assign13750_e18160 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign13750_e18163 * assign13750_e18163)), ((((assign13750_e18158 * locals.var_tmf3_dn5) * assign13750_e18163) - (assign13750_e18160 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign13750_e18163 * assign13750_e18163)), ((((assign13750_e18158 * locals.var_tmf3_dn6) * assign13750_e18163) - (assign13750_e18160 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign13750_e18163 * assign13750_e18163)), ((((assign13750_e18158 * locals.var_tmf3_dn8) * assign13750_e18163) - (assign13750_e18160 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign13750_e18163 * assign13750_e18163)), ((((assign13750_e18158 * locals.var_tmf3_dn10) * assign13750_e18163) - (assign13750_e18160 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign13750_e18163 * assign13750_e18163)), ((((assign13750_e18158 * locals.var_tmf3_dn11) * assign13750_e18163) - (assign13750_e18160 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign13750_e18163 * assign13750_e18163)), ((((assign13750_e18158 * locals.var_tmf3_dn12) * assign13750_e18163) - (assign13750_e18160 * ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)))) / (assign13750_e18163 * assign13750_e18163)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign13750_e18166;
        locals.var_t2_dn0 = assign13750_e18166_d_n0;
        locals.var_t2_dn2 = assign13750_e18166_d_n2;
        locals.var_t2_dn4 = assign13750_e18166_d_n4;
        locals.var_t2_dn5 = assign13750_e18166_d_n5;
        locals.var_t2_dn6 = assign13750_e18166_d_n6;
        locals.var_t2_dn8 = assign13750_e18166_d_n8;
        locals.var_t2_dn10 = assign13750_e18166_d_n10;
        locals.var_t2_dn11 = assign13750_e18166_d_n11;
        locals.var_t2_dn12 = assign13750_e18166_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign13760_e18174, assign13760_e18174_d_n0, assign13760_e18174_d_n2, assign13760_e18174_d_n4, assign13760_e18174_d_n5, assign13760_e18174_d_n6, assign13760_e18174_d_n8, assign13760_e18174_d_n10, assign13760_e18174_d_n11, assign13760_e18174_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13760_e18171: f64 = (locals.var_ps0 + locals.var_t6);
        let assign13760_e18172: f64 = (1.1 - assign13760_e18171);
        (assign13760_e18172, (-(locals.var_ps0_dn0 + locals.var_t6_dn0)), (-(locals.var_ps0_dn2 + locals.var_t6_dn2)), (-(locals.var_ps0_dn4 + locals.var_t6_dn4)), (-(locals.var_ps0_dn5 + locals.var_t6_dn5)), (-(locals.var_ps0_dn6 + locals.var_t6_dn6)), (-(locals.var_ps0_dn8 + locals.var_t6_dn8)), (-(locals.var_ps0_dn10 + locals.var_t6_dn10)), (-(locals.var_ps0_dn11 + locals.var_t6_dn11)), (-(locals.var_ps0_dn12 + locals.var_t6_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign13760_e18174;
        locals.var_t1_dn0 = assign13760_e18174_d_n0;
        locals.var_t1_dn2 = assign13760_e18174_d_n2;
        locals.var_t1_dn4 = assign13760_e18174_d_n4;
        locals.var_t1_dn5 = assign13760_e18174_d_n5;
        locals.var_t1_dn6 = assign13760_e18174_d_n6;
        locals.var_t1_dn8 = assign13760_e18174_d_n8;
        locals.var_t1_dn10 = assign13760_e18174_d_n10;
        locals.var_t1_dn11 = assign13760_e18174_d_n11;
        locals.var_t1_dn12 = assign13760_e18174_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign13770_e18187, assign13770_e18187_d_n0, assign13770_e18187_d_n2, assign13770_e18187_d_n4, assign13770_e18187_d_n5, assign13770_e18187_d_n6, assign13770_e18187_d_n8, assign13770_e18187_d_n10, assign13770_e18187_d_n11, assign13770_e18187_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13770_e18178: f64 = (locals.var_t1 * locals.var_t1);
        let assign13770_e18181: f64 = (4.0 * 0.05);
        let assign13770_e18183: f64 = (assign13770_e18181 * 0.05);
        let assign13770_e18184: f64 = (assign13770_e18178 + assign13770_e18183);
        let assign13770_e18185: f64 = (assign13770_e18184).sqrt();
        (assign13770_e18185, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign13770_e18185)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign13770_e18185)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign13770_e18185)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign13770_e18185)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign13770_e18185)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign13770_e18185)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign13770_e18185)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign13770_e18185)), (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign13770_e18185)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign13770_e18187;
        locals.var_tmf2_dn0 = assign13770_e18187_d_n0;
        locals.var_tmf2_dn2 = assign13770_e18187_d_n2;
        locals.var_tmf2_dn4 = assign13770_e18187_d_n4;
        locals.var_tmf2_dn5 = assign13770_e18187_d_n5;
        locals.var_tmf2_dn6 = assign13770_e18187_d_n6;
        locals.var_tmf2_dn8 = assign13770_e18187_d_n8;
        locals.var_tmf2_dn10 = assign13770_e18187_d_n10;
        locals.var_tmf2_dn11 = assign13770_e18187_d_n11;
        locals.var_tmf2_dn12 = assign13770_e18187_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign13780_e18197, assign13780_e18197_d_n0, assign13780_e18197_d_n2, assign13780_e18197_d_n4, assign13780_e18197_d_n5, assign13780_e18197_d_n6, assign13780_e18197_d_n8, assign13780_e18197_d_n10, assign13780_e18197_d_n11, assign13780_e18197_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13780_e18193: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign13780_e18194: f64 = (1.0 + assign13780_e18193);
        let assign13780_e18195: f64 = (0.5 * assign13780_e18194);
        (assign13780_e18195, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn12 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign13780_e18197;
        locals.var_t0_dn0 = assign13780_e18197_d_n0;
        locals.var_t0_dn2 = assign13780_e18197_d_n2;
        locals.var_t0_dn4 = assign13780_e18197_d_n4;
        locals.var_t0_dn5 = assign13780_e18197_d_n5;
        locals.var_t0_dn6 = assign13780_e18197_d_n6;
        locals.var_t0_dn8 = assign13780_e18197_d_n8;
        locals.var_t0_dn10 = assign13780_e18197_d_n10;
        locals.var_t0_dn11 = assign13780_e18197_d_n11;
        locals.var_t0_dn12 = assign13780_e18197_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign13790_e18209, assign13790_e18209_d_n0, assign13790_e18209_d_n2, assign13790_e18209_d_n4, assign13790_e18209_d_n5, assign13790_e18209_d_n6, assign13790_e18209_d_n8, assign13790_e18209_d_n10, assign13790_e18209_d_n11, assign13790_e18209_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13790_e18202: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign13790_e18203: f64 = (0.5 * assign13790_e18202);
        let assign13790_e18206: f64 = (1e-10 * 0.05);
        let assign13790_e18207: f64 = (assign13790_e18203 + assign13790_e18206);
        (assign13790_e18207, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign13790_e18209;
        locals.var_t2_dn0 = assign13790_e18209_d_n0;
        locals.var_t2_dn2 = assign13790_e18209_d_n2;
        locals.var_t2_dn4 = assign13790_e18209_d_n4;
        locals.var_t2_dn5 = assign13790_e18209_d_n5;
        locals.var_t2_dn6 = assign13790_e18209_d_n6;
        locals.var_t2_dn8 = assign13790_e18209_d_n8;
        locals.var_t2_dn10 = assign13790_e18209_d_n10;
        locals.var_t2_dn11 = assign13790_e18209_d_n11;
        locals.var_t2_dn12 = assign13790_e18209_d_n12;
        locals.var_t2_rv = 0.0;

        let assign13800_e18212: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign13800_e18212;
        locals.var_guard252_rv = 0.0;

        let (assign13810_e18218, assign13810_e18218_d_n0, assign13810_e18218_d_n2, assign13810_e18218_d_n4, assign13810_e18218_d_n5, assign13810_e18218_d_n6, assign13810_e18218_d_n8, assign13810_e18218_d_n10, assign13810_e18218_d_n11, assign13810_e18218_d_n12,) = {
    if ((locals.var_guard251 != 0.0) && (locals.var_guard252 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign13810_e18218;
        locals.var_t2_dn0 = assign13810_e18218_d_n0;
        locals.var_t2_dn2 = assign13810_e18218_d_n2;
        locals.var_t2_dn4 = assign13810_e18218_d_n4;
        locals.var_t2_dn5 = assign13810_e18218_d_n5;
        locals.var_t2_dn6 = assign13810_e18218_d_n6;
        locals.var_t2_dn8 = assign13810_e18218_d_n8;
        locals.var_t2_dn10 = assign13810_e18218_d_n10;
        locals.var_t2_dn11 = assign13810_e18218_d_n11;
        locals.var_t2_dn12 = assign13810_e18218_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign13820_e18224, assign13820_e18224_d_n0, assign13820_e18224_d_n2, assign13820_e18224_d_n4, assign13820_e18224_d_n5, assign13820_e18224_d_n6, assign13820_e18224_d_n8, assign13820_e18224_d_n10, assign13820_e18224_d_n11, assign13820_e18224_d_n12,) = {
    if ((locals.var_guard251 != 0.0) && (locals.var_guard252 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign13820_e18224;
        locals.var_t0_dn0 = assign13820_e18224_d_n0;
        locals.var_t0_dn2 = assign13820_e18224_d_n2;
        locals.var_t0_dn4 = assign13820_e18224_d_n4;
        locals.var_t0_dn5 = assign13820_e18224_d_n5;
        locals.var_t0_dn6 = assign13820_e18224_d_n6;
        locals.var_t0_dn8 = assign13820_e18224_d_n8;
        locals.var_t0_dn10 = assign13820_e18224_d_n10;
        locals.var_t0_dn11 = assign13820_e18224_d_n11;
        locals.var_t0_dn12 = assign13820_e18224_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign13830_e18236, assign13830_e18236_d_n0, assign13830_e18236_d_n2, assign13830_e18236_d_n4, assign13830_e18236_d_n5, assign13830_e18236_d_n6, assign13830_e18236_d_n8, assign13830_e18236_d_n10, assign13830_e18236_d_n11, assign13830_e18236_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13830_e18228: f64 = (locals.var_c_fox * locals.var_beta);
        let assign13830_e18230: f64 = (assign13830_e18228 * locals.var_ptl0);
        let assign13830_e18233: f64 = (locals.var_t2).powf(p.p240);
        let assign13830_e18234: f64 = (assign13830_e18230 * assign13830_e18233);
        (assign13830_e18234, ((((locals.var_c_fox_dn0 * locals.var_beta) * locals.var_ptl0) * assign13830_e18233) + (assign13830_e18230 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn0)) } } else { (assign13830_e18233 * (p.p240 * (locals.var_t2_dn0 / locals.var_t2))) })), ((((locals.var_c_fox_dn2 * locals.var_beta) * locals.var_ptl0) * assign13830_e18233) + (assign13830_e18230 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn2)) } } else { (assign13830_e18233 * (p.p240 * (locals.var_t2_dn2 / locals.var_t2))) })), (((((locals.var_c_fox_dn4 * locals.var_beta) + (locals.var_c_fox * locals.var_beta_dn4)) * locals.var_ptl0) * assign13830_e18233) + (assign13830_e18230 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn4)) } } else { (assign13830_e18233 * (p.p240 * (locals.var_t2_dn4 / locals.var_t2))) })), ((((locals.var_c_fox_dn5 * locals.var_beta) * locals.var_ptl0) * assign13830_e18233) + (assign13830_e18230 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn5)) } } else { (assign13830_e18233 * (p.p240 * (locals.var_t2_dn5 / locals.var_t2))) })), ((((locals.var_c_fox_dn6 * locals.var_beta) * locals.var_ptl0) * assign13830_e18233) + (assign13830_e18230 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn6)) } } else { (assign13830_e18233 * (p.p240 * (locals.var_t2_dn6 / locals.var_t2))) })), ((((locals.var_c_fox_dn8 * locals.var_beta) * locals.var_ptl0) * assign13830_e18233) + (assign13830_e18230 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn8)) } } else { (assign13830_e18233 * (p.p240 * (locals.var_t2_dn8 / locals.var_t2))) })), ((((locals.var_c_fox_dn10 * locals.var_beta) * locals.var_ptl0) * assign13830_e18233) + (assign13830_e18230 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn10)) } } else { (assign13830_e18233 * (p.p240 * (locals.var_t2_dn10 / locals.var_t2))) })), ((((locals.var_c_fox_dn11 * locals.var_beta) * locals.var_ptl0) * assign13830_e18233) + (assign13830_e18230 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn11)) } } else { (assign13830_e18233 * (p.p240 * (locals.var_t2_dn11 / locals.var_t2))) })), ((((locals.var_c_fox_dn12 * locals.var_beta) * locals.var_ptl0) * assign13830_e18233) + (assign13830_e18230 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn12)) } } else { (assign13830_e18233 * (p.p240 * (locals.var_t2_dn12 / locals.var_t2))) })),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign13830_e18236;
        locals.var_t9_dn0 = assign13830_e18236_d_n0;
        locals.var_t9_dn2 = assign13830_e18236_d_n2;
        locals.var_t9_dn4 = assign13830_e18236_d_n4;
        locals.var_t9_dn5 = assign13830_e18236_d_n5;
        locals.var_t9_dn6 = assign13830_e18236_d_n6;
        locals.var_t9_dn8 = assign13830_e18236_d_n8;
        locals.var_t9_dn10 = assign13830_e18236_d_n10;
        locals.var_t9_dn11 = assign13830_e18236_d_n11;
        locals.var_t9_dn12 = assign13830_e18236_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign13840_e18254, assign13840_e18254_d_n0, assign13840_e18254_d_n2, assign13840_e18254_d_n4, assign13840_e18254_d_n5, assign13840_e18254_d_n6, assign13840_e18254_d_n8, assign13840_e18254_d_n10, assign13840_e18254_d_n11, assign13840_e18254_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13840_e18241: f64 = (locals.var_vdsz * p.p241);
        let assign13840_e18242: f64 = (1.0 + assign13840_e18241);
        let assign13840_e18245: f64 = (locals.var_vdsz * locals.var_pt40);
        let assign13840_e18248: f64 = (locals.var_ps0 + locals.var_t6);
        let assign13840_e18250: f64 = (assign13840_e18248 - locals.var_vbsz);
        let assign13840_e18251: f64 = (assign13840_e18245 * assign13840_e18250);
        let assign13840_e18252: f64 = (assign13840_e18242 + assign13840_e18251);
        (assign13840_e18252, ((locals.var_vdsz_dn0 * p.p241) + (((locals.var_vdsz_dn0 * locals.var_pt40) * assign13840_e18250) + (assign13840_e18245 * ((locals.var_ps0_dn0 + locals.var_t6_dn0) - locals.var_vbsz_dn0)))), ((locals.var_vdsz_dn2 * p.p241) + (((locals.var_vdsz_dn2 * locals.var_pt40) * assign13840_e18250) + (assign13840_e18245 * ((locals.var_ps0_dn2 + locals.var_t6_dn2) - locals.var_vbsz_dn2)))), ((locals.var_vdsz_dn4 * p.p241) + (((locals.var_vdsz_dn4 * locals.var_pt40) * assign13840_e18250) + (assign13840_e18245 * ((locals.var_ps0_dn4 + locals.var_t6_dn4) - locals.var_vbsz_dn4)))), ((locals.var_vdsz_dn5 * p.p241) + (((locals.var_vdsz_dn5 * locals.var_pt40) * assign13840_e18250) + (assign13840_e18245 * ((locals.var_ps0_dn5 + locals.var_t6_dn5) - locals.var_vbsz_dn5)))), ((locals.var_vdsz_dn6 * p.p241) + (((locals.var_vdsz_dn6 * locals.var_pt40) * assign13840_e18250) + (assign13840_e18245 * ((locals.var_ps0_dn6 + locals.var_t6_dn6) - locals.var_vbsz_dn6)))), ((locals.var_vdsz_dn8 * p.p241) + (((locals.var_vdsz_dn8 * locals.var_pt40) * assign13840_e18250) + (assign13840_e18245 * ((locals.var_ps0_dn8 + locals.var_t6_dn8) - locals.var_vbsz_dn8)))), ((locals.var_vdsz_dn10 * p.p241) + (((locals.var_vdsz_dn10 * locals.var_pt40) * assign13840_e18250) + (assign13840_e18245 * ((locals.var_ps0_dn10 + locals.var_t6_dn10) - locals.var_vbsz_dn10)))), ((locals.var_vdsz_dn11 * p.p241) + (((locals.var_vdsz_dn11 * locals.var_pt40) * assign13840_e18250) + (assign13840_e18245 * ((locals.var_ps0_dn11 + locals.var_t6_dn11) - locals.var_vbsz_dn11)))), ((locals.var_vdsz_dn12 * p.p241) + (((locals.var_vdsz_dn12 * locals.var_pt40) * assign13840_e18250) + (assign13840_e18245 * ((locals.var_ps0_dn12 + locals.var_t6_dn12) - locals.var_vbsz_dn12)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign13840_e18254;
        locals.var_t4_dn0 = assign13840_e18254_d_n0;
        locals.var_t4_dn2 = assign13840_e18254_d_n2;
        locals.var_t4_dn4 = assign13840_e18254_d_n4;
        locals.var_t4_dn5 = assign13840_e18254_d_n5;
        locals.var_t4_dn6 = assign13840_e18254_d_n6;
        locals.var_t4_dn8 = assign13840_e18254_d_n8;
        locals.var_t4_dn10 = assign13840_e18254_d_n10;
        locals.var_t4_dn11 = assign13840_e18254_d_n11;
        locals.var_t4_dn12 = assign13840_e18254_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign13850_e18260, assign13850_e18260_d_n0, assign13850_e18260_d_n2, assign13850_e18260_d_n4, assign13850_e18260_d_n5, assign13850_e18260_d_n6, assign13850_e18260_d_n8, assign13850_e18260_d_n10, assign13850_e18260_d_n11, assign13850_e18260_d_n12,) = {
    if (locals.var_guard251 != 0.0) {
        let assign13850_e18258: f64 = (locals.var_t9 * locals.var_t4);
        (assign13850_e18258, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn11 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn11)), ((locals.var_t9_dn12 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn12)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign13850_e18260;
        locals.var_t9_dn0 = assign13850_e18260_d_n0;
        locals.var_t9_dn2 = assign13850_e18260_d_n2;
        locals.var_t9_dn4 = assign13850_e18260_d_n4;
        locals.var_t9_dn5 = assign13850_e18260_d_n5;
        locals.var_t9_dn6 = assign13850_e18260_d_n6;
        locals.var_t9_dn8 = assign13850_e18260_d_n8;
        locals.var_t9_dn10 = assign13850_e18260_d_n10;
        locals.var_t9_dn11 = assign13850_e18260_d_n11;
        locals.var_t9_dn12 = assign13850_e18260_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign13860_e18265, assign13860_e18265_d_n0, assign13860_e18265_d_n2, assign13860_e18265_d_n4, assign13860_e18265_d_n5, assign13860_e18265_d_n6, assign13860_e18265_d_n8, assign13860_e18265_d_n10, assign13860_e18265_d_n11, assign13860_e18265_d_n12,) = {
    if (locals.var_guard251 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign13860_e18265;
        locals.var_t9_dn0 = assign13860_e18265_d_n0;
        locals.var_t9_dn2 = assign13860_e18265_d_n2;
        locals.var_t9_dn4 = assign13860_e18265_d_n4;
        locals.var_t9_dn5 = assign13860_e18265_d_n5;
        locals.var_t9_dn6 = assign13860_e18265_d_n6;
        locals.var_t9_dn8 = assign13860_e18265_d_n8;
        locals.var_t9_dn10 = assign13860_e18265_d_n10;
        locals.var_t9_dn11 = assign13860_e18265_d_n11;
        locals.var_t9_dn12 = assign13860_e18265_d_n12;
        locals.var_t9_rv = 0.0;

        let assign13870_e18268: f64 = if p.p246 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign13870_e18268;
        locals.var_guard253_rv = 0.0;

        let (assign13880_e18278, assign13880_e18278_d_n0, assign13880_e18278_d_n2, assign13880_e18278_d_n4, assign13880_e18278_d_n5, assign13880_e18278_d_n6, assign13880_e18278_d_n8, assign13880_e18278_d_n10, assign13880_e18278_d_n11, assign13880_e18278_d_n12,) = {
    if (locals.var_guard253 != 0.0) {
        let assign13880_e18272: f64 = (locals.var_c_fox * locals.var_beta);
        let assign13880_e18274: f64 = (assign13880_e18272 * locals.var_gdl0);
        let assign13880_e18276: f64 = (assign13880_e18274 * locals.var_vdsz);
        (assign13880_e18276, ((((locals.var_c_fox_dn0 * locals.var_beta) * locals.var_gdl0) * locals.var_vdsz) + (assign13880_e18274 * locals.var_vdsz_dn0)), ((((locals.var_c_fox_dn2 * locals.var_beta) * locals.var_gdl0) * locals.var_vdsz) + (assign13880_e18274 * locals.var_vdsz_dn2)), (((((locals.var_c_fox_dn4 * locals.var_beta) + (locals.var_c_fox * locals.var_beta_dn4)) * locals.var_gdl0) * locals.var_vdsz) + (assign13880_e18274 * locals.var_vdsz_dn4)), ((((locals.var_c_fox_dn5 * locals.var_beta) * locals.var_gdl0) * locals.var_vdsz) + (assign13880_e18274 * locals.var_vdsz_dn5)), ((((locals.var_c_fox_dn6 * locals.var_beta) * locals.var_gdl0) * locals.var_vdsz) + (assign13880_e18274 * locals.var_vdsz_dn6)), ((((locals.var_c_fox_dn8 * locals.var_beta) * locals.var_gdl0) * locals.var_vdsz) + (assign13880_e18274 * locals.var_vdsz_dn8)), ((((locals.var_c_fox_dn10 * locals.var_beta) * locals.var_gdl0) * locals.var_vdsz) + (assign13880_e18274 * locals.var_vdsz_dn10)), ((((locals.var_c_fox_dn11 * locals.var_beta) * locals.var_gdl0) * locals.var_vdsz) + (assign13880_e18274 * locals.var_vdsz_dn11)), ((((locals.var_c_fox_dn12 * locals.var_beta) * locals.var_gdl0) * locals.var_vdsz) + (assign13880_e18274 * locals.var_vdsz_dn12)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn8, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12,)
    }
};
        locals.var_t8 = assign13880_e18278;
        locals.var_t8_dn0 = assign13880_e18278_d_n0;
        locals.var_t8_dn2 = assign13880_e18278_d_n2;
        locals.var_t8_dn4 = assign13880_e18278_d_n4;
        locals.var_t8_dn5 = assign13880_e18278_d_n5;
        locals.var_t8_dn6 = assign13880_e18278_d_n6;
        locals.var_t8_dn8 = assign13880_e18278_d_n8;
        locals.var_t8_dn10 = assign13880_e18278_d_n10;
        locals.var_t8_dn11 = assign13880_e18278_d_n11;
        locals.var_t8_dn12 = assign13880_e18278_d_n12;
        locals.var_t8_rv = 0.0;

        let (assign13890_e18283, assign13890_e18283_d_n0, assign13890_e18283_d_n2, assign13890_e18283_d_n4, assign13890_e18283_d_n5, assign13890_e18283_d_n6, assign13890_e18283_d_n8, assign13890_e18283_d_n10, assign13890_e18283_d_n11, assign13890_e18283_d_n12,) = {
    if (locals.var_guard253 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn8, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12,)
    }
};
        locals.var_t8 = assign13890_e18283;
        locals.var_t8_dn0 = assign13890_e18283_d_n0;
        locals.var_t8_dn2 = assign13890_e18283_d_n2;
        locals.var_t8_dn4 = assign13890_e18283_d_n4;
        locals.var_t8_dn5 = assign13890_e18283_d_n5;
        locals.var_t8_dn6 = assign13890_e18283_d_n6;
        locals.var_t8_dn8 = assign13890_e18283_d_n8;
        locals.var_t8_dn10 = assign13890_e18283_d_n10;
        locals.var_t8_dn11 = assign13890_e18283_d_n11;
        locals.var_t8_dn12 = assign13890_e18283_d_n12;
        locals.var_t8_rv = 0.0;

        let assign13900_e18286: f64 = (locals.var_t9 + locals.var_t8);
        let assign13900_e18288: f64 = if assign13900_e18286 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign13900_e18288;
        locals.var_guard254_rv = 0.0;

        let (assign13910_e18296, assign13910_e18296_d_n0, assign13910_e18296_d_n2, assign13910_e18296_d_n4, assign13910_e18296_d_n5, assign13910_e18296_d_n6, assign13910_e18296_d_n8, assign13910_e18296_d_n10, assign13910_e18296_d_n11, assign13910_e18296_d_n12,) = {
    if (locals.var_guard254 != 0.0) {
        let assign13910_e18293: f64 = (locals.var_t9 + locals.var_t8);
        let assign13910_e18294: f64 = (locals.var_pds * assign13910_e18293);
        (assign13910_e18294, ((locals.var_pds_dn0 * assign13910_e18293) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign13910_e18293) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign13910_e18293) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign13910_e18293) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign13910_e18293) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn8 * assign13910_e18293) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn10 * assign13910_e18293) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn11 * assign13910_e18293) + (locals.var_pds * (locals.var_t9_dn11 + locals.var_t8_dn11))), ((locals.var_pds_dn12 * assign13910_e18293) + (locals.var_pds * (locals.var_t9_dn12 + locals.var_t8_dn12))),)
    } else {
        (locals.var_idd0, locals.var_idd0_dn0, locals.var_idd0_dn2, locals.var_idd0_dn4, locals.var_idd0_dn5, locals.var_idd0_dn6, locals.var_idd0_dn8, locals.var_idd0_dn10, locals.var_idd0_dn11, locals.var_idd0_dn12,)
    }
};
        locals.var_idd0 = assign13910_e18296;
        locals.var_idd0_dn0 = assign13910_e18296_d_n0;
        locals.var_idd0_dn2 = assign13910_e18296_d_n2;
        locals.var_idd0_dn4 = assign13910_e18296_d_n4;
        locals.var_idd0_dn5 = assign13910_e18296_d_n5;
        locals.var_idd0_dn6 = assign13910_e18296_d_n6;
        locals.var_idd0_dn8 = assign13910_e18296_d_n8;
        locals.var_idd0_dn10 = assign13910_e18296_d_n10;
        locals.var_idd0_dn11 = assign13910_e18296_d_n11;
        locals.var_idd0_dn12 = assign13910_e18296_d_n12;
        locals.var_idd0_rv = 0.0;

        let (assign13920_e18304, assign13920_e18304_d_n0, assign13920_e18304_d_n2, assign13920_e18304_d_n4, assign13920_e18304_d_n5, assign13920_e18304_d_n6, assign13920_e18304_d_n8, assign13920_e18304_d_n10, assign13920_e18304_d_n11, assign13920_e18304_d_n12,) = {
    if (locals.var_guard254 != 0.0) {
        let assign13920_e18300: f64 = (locals.var_betawl * locals.var_idd0);
        let assign13920_e18302: f64 = (assign13920_e18300 * locals.var_mu);
        (assign13920_e18302, ((((locals.var_betawl_dn0 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn0)) * locals.var_mu) + (assign13920_e18300 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn2)) * locals.var_mu) + (assign13920_e18300 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn4)) * locals.var_mu) + (assign13920_e18300 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn5)) * locals.var_mu) + (assign13920_e18300 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn6)) * locals.var_mu) + (assign13920_e18300 * locals.var_mu_dn6)), ((((locals.var_betawl_dn8 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn8)) * locals.var_mu) + (assign13920_e18300 * locals.var_mu_dn8)), ((((locals.var_betawl_dn10 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn10)) * locals.var_mu) + (assign13920_e18300 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn11)) * locals.var_mu) + (assign13920_e18300 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn12)) * locals.var_mu) + (assign13920_e18300 * locals.var_mu_dn12)),)
    } else {
        (locals.var_idspt, locals.var_idspt_dn0, locals.var_idspt_dn2, locals.var_idspt_dn4, locals.var_idspt_dn5, locals.var_idspt_dn6, locals.var_idspt_dn8, locals.var_idspt_dn10, locals.var_idspt_dn11, locals.var_idspt_dn12,)
    }
};
        locals.var_idspt = assign13920_e18304;
        locals.var_idspt_dn0 = assign13920_e18304_d_n0;
        locals.var_idspt_dn2 = assign13920_e18304_d_n2;
        locals.var_idspt_dn4 = assign13920_e18304_d_n4;
        locals.var_idspt_dn5 = assign13920_e18304_d_n5;
        locals.var_idspt_dn6 = assign13920_e18304_d_n6;
        locals.var_idspt_dn8 = assign13920_e18304_d_n8;
        locals.var_idspt_dn10 = assign13920_e18304_d_n10;
        locals.var_idspt_dn11 = assign13920_e18304_d_n11;
        locals.var_idspt_dn12 = assign13920_e18304_d_n12;
        locals.var_idspt_rv = 0.0;

        let (assign13930_e18316, assign13930_e18316_d_n0, assign13930_e18316_d_n2, assign13930_e18316_d_n4, assign13930_e18316_d_n5, assign13930_e18316_d_n6, assign13930_e18316_d_n8, assign13930_e18316_d_n10, assign13930_e18316_d_n11, assign13930_e18316_d_n12,) = {
    if (locals.var_guard254 != 0.0) {
        let assign13930_e18309: f64 = (-p.p245);
        let assign13930_e18311: f64 = (assign13930_e18309 * locals.var_vbsbiz);
        let assign13930_e18312: f64 = (assign13930_e18311).exp();
        let assign13930_e18313: f64 = (1.0 + assign13930_e18312);
        let assign13930_e18314: f64 = (1.0 / assign13930_e18313);
        (assign13930_e18314, (-((assign13930_e18312 * (assign13930_e18309 * locals.var_vbsbiz_dn0)) / (assign13930_e18313 * assign13930_e18313))), (-((assign13930_e18312 * (assign13930_e18309 * locals.var_vbsbiz_dn2)) / (assign13930_e18313 * assign13930_e18313))), (-((assign13930_e18312 * (assign13930_e18309 * locals.var_vbsbiz_dn4)) / (assign13930_e18313 * assign13930_e18313))), (-((assign13930_e18312 * (assign13930_e18309 * locals.var_vbsbiz_dn5)) / (assign13930_e18313 * assign13930_e18313))), (-((assign13930_e18312 * (assign13930_e18309 * locals.var_vbsbiz_dn6)) / (assign13930_e18313 * assign13930_e18313))), (-((assign13930_e18312 * (assign13930_e18309 * locals.var_vbsbiz_dn8)) / (assign13930_e18313 * assign13930_e18313))), (-((assign13930_e18312 * (assign13930_e18309 * locals.var_vbsbiz_dn10)) / (assign13930_e18313 * assign13930_e18313))), (-((assign13930_e18312 * (assign13930_e18309 * locals.var_vbsbiz_dn11)) / (assign13930_e18313 * assign13930_e18313))), (-((assign13930_e18312 * (assign13930_e18309 * locals.var_vbsbiz_dn12)) / (assign13930_e18313 * assign13930_e18313))),)
    } else {
        (locals.var_ids2_fac, locals.var_ids2_fac_dn0, locals.var_ids2_fac_dn2, locals.var_ids2_fac_dn4, locals.var_ids2_fac_dn5, locals.var_ids2_fac_dn6, locals.var_ids2_fac_dn8, locals.var_ids2_fac_dn10, locals.var_ids2_fac_dn11, locals.var_ids2_fac_dn12,)
    }
};
        locals.var_ids2_fac = assign13930_e18316;
        locals.var_ids2_fac_dn0 = assign13930_e18316_d_n0;
        locals.var_ids2_fac_dn2 = assign13930_e18316_d_n2;
        locals.var_ids2_fac_dn4 = assign13930_e18316_d_n4;
        locals.var_ids2_fac_dn5 = assign13930_e18316_d_n5;
        locals.var_ids2_fac_dn6 = assign13930_e18316_d_n6;
        locals.var_ids2_fac_dn8 = assign13930_e18316_d_n8;
        locals.var_ids2_fac_dn10 = assign13930_e18316_d_n10;
        locals.var_ids2_fac_dn11 = assign13930_e18316_d_n11;
        locals.var_ids2_fac_dn12 = assign13930_e18316_d_n12;
        locals.var_ids2_fac_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13940_e18322, assign13940_e18322_d_n0, assign13940_e18322_d_n2, assign13940_e18322_d_n4, assign13940_e18322_d_n5, assign13940_e18322_d_n6, assign13940_e18322_d_n8, assign13940_e18322_d_n10, assign13940_e18322_d_n11, assign13940_e18322_d_n12,) = {
    if (locals.var_guard254 != 0.0) {
        let assign13940_e18320: f64 = (1.0 - locals.var_ids2_fac);
        (assign13940_e18320, (-locals.var_ids2_fac_dn0), (-locals.var_ids2_fac_dn2), (-locals.var_ids2_fac_dn4), (-locals.var_ids2_fac_dn5), (-locals.var_ids2_fac_dn6), (-locals.var_ids2_fac_dn8), (-locals.var_ids2_fac_dn10), (-locals.var_ids2_fac_dn11), (-locals.var_ids2_fac_dn12),)
    } else {
        (locals.var_ids1_fac, locals.var_ids1_fac_dn0, locals.var_ids1_fac_dn2, locals.var_ids1_fac_dn4, locals.var_ids1_fac_dn5, locals.var_ids1_fac_dn6, locals.var_ids1_fac_dn8, locals.var_ids1_fac_dn10, locals.var_ids1_fac_dn11, locals.var_ids1_fac_dn12,)
    }
};
        locals.var_ids1_fac = assign13940_e18322;
        locals.var_ids1_fac_dn0 = assign13940_e18322_d_n0;
        locals.var_ids1_fac_dn2 = assign13940_e18322_d_n2;
        locals.var_ids1_fac_dn4 = assign13940_e18322_d_n4;
        locals.var_ids1_fac_dn5 = assign13940_e18322_d_n5;
        locals.var_ids1_fac_dn6 = assign13940_e18322_d_n6;
        locals.var_ids1_fac_dn8 = assign13940_e18322_d_n8;
        locals.var_ids1_fac_dn10 = assign13940_e18322_d_n10;
        locals.var_ids1_fac_dn11 = assign13940_e18322_d_n11;
        locals.var_ids1_fac_dn12 = assign13940_e18322_d_n12;
        locals.var_ids1_fac_rv = 0.0;

        let (assign13950_e18328, assign13950_e18328_d_n0, assign13950_e18328_d_n2, assign13950_e18328_d_n4, assign13950_e18328_d_n5, assign13950_e18328_d_n6, assign13950_e18328_d_n8, assign13950_e18328_d_n10, assign13950_e18328_d_n11, assign13950_e18328_d_n12,) = {
    if (locals.var_guard254 != 0.0) {
        let assign13950_e18326: f64 = (locals.var_ids1_fac * locals.var_idspt);
        (assign13950_e18326, ((locals.var_ids1_fac_dn0 * locals.var_idspt) + (locals.var_ids1_fac * locals.var_idspt_dn0)), ((locals.var_ids1_fac_dn2 * locals.var_idspt) + (locals.var_ids1_fac * locals.var_idspt_dn2)), ((locals.var_ids1_fac_dn4 * locals.var_idspt) + (locals.var_ids1_fac * locals.var_idspt_dn4)), ((locals.var_ids1_fac_dn5 * locals.var_idspt) + (locals.var_ids1_fac * locals.var_idspt_dn5)), ((locals.var_ids1_fac_dn6 * locals.var_idspt) + (locals.var_ids1_fac * locals.var_idspt_dn6)), ((locals.var_ids1_fac_dn8 * locals.var_idspt) + (locals.var_ids1_fac * locals.var_idspt_dn8)), ((locals.var_ids1_fac_dn10 * locals.var_idspt) + (locals.var_ids1_fac * locals.var_idspt_dn10)), ((locals.var_ids1_fac_dn11 * locals.var_idspt) + (locals.var_ids1_fac * locals.var_idspt_dn11)), ((locals.var_ids1_fac_dn12 * locals.var_idspt) + (locals.var_ids1_fac * locals.var_idspt_dn12)),)
    } else {
        (locals.var_ids1, locals.var_ids1_dn0, locals.var_ids1_dn2, locals.var_ids1_dn4, locals.var_ids1_dn5, locals.var_ids1_dn6, locals.var_ids1_dn8, locals.var_ids1_dn10, locals.var_ids1_dn11, locals.var_ids1_dn12,)
    }
};
        locals.var_ids1 = assign13950_e18328;
        locals.var_ids1_dn0 = assign13950_e18328_d_n0;
        locals.var_ids1_dn2 = assign13950_e18328_d_n2;
        locals.var_ids1_dn4 = assign13950_e18328_d_n4;
        locals.var_ids1_dn5 = assign13950_e18328_d_n5;
        locals.var_ids1_dn6 = assign13950_e18328_d_n6;
        locals.var_ids1_dn8 = assign13950_e18328_d_n8;
        locals.var_ids1_dn10 = assign13950_e18328_d_n10;
        locals.var_ids1_dn11 = assign13950_e18328_d_n11;
        locals.var_ids1_dn12 = assign13950_e18328_d_n12;
        locals.var_ids1_rv = 0.0;

        locals.var_idsptb = 0.0;
        locals.var_idsptb_dn0 = 0.0;
        locals.var_idsptb_dn2 = 0.0;
        locals.var_idsptb_dn4 = 0.0;
        locals.var_idsptb_dn5 = 0.0;
        locals.var_idsptb_dn6 = 0.0;
        locals.var_idsptb_dn8 = 0.0;
        locals.var_idsptb_dn10 = 0.0;
        locals.var_idsptb_dn11 = 0.0;
        locals.var_idsptb_dn12 = 0.0;
        locals.var_idsptb_rv = 0.0;

        locals.var_ids2 = 0.0;
        locals.var_ids2_dn0 = 0.0;
        locals.var_ids2_dn2 = 0.0;
        locals.var_ids2_dn4 = 0.0;
        locals.var_ids2_dn5 = 0.0;
        locals.var_ids2_dn6 = 0.0;
        locals.var_ids2_dn8 = 0.0;
        locals.var_ids2_dn10 = 0.0;
        locals.var_ids2_dn11 = 0.0;
        locals.var_ids2_dn12 = 0.0;
        locals.var_ids2_rv = 0.0;

        let assign13980_e18333: f64 = if p.p239 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign13980_e18333;
        locals.var_guard255_rv = 0.0;

        let (assign13990_e18341, assign13990_e18341_d_n0, assign13990_e18341_d_n2, assign13990_e18341_d_n4, assign13990_e18341_d_n5, assign13990_e18341_d_n6, assign13990_e18341_d_n8, assign13990_e18341_d_n10, assign13990_e18341_d_n11, assign13990_e18341_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign13990_e18338: f64 = (locals.var_vds - locals.var_pbds);
        let assign13990_e18339: f64 = (0.5 * assign13990_e18338);
        (assign13990_e18339, (0.5 * (locals.var_vds_dn0 - locals.var_pbds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pbds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pbds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pbds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pbds_dn6)), (0.5 * (locals.var_vds_dn8 - locals.var_pbds_dn8)), (0.5 * (locals.var_vds_dn10 - locals.var_pbds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pbds_dn11)), (0.5 * (locals.var_vds_dn12 - locals.var_pbds_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign13990_e18341;
        locals.var_t1_dn0 = assign13990_e18341_d_n0;
        locals.var_t1_dn2 = assign13990_e18341_d_n2;
        locals.var_t1_dn4 = assign13990_e18341_d_n4;
        locals.var_t1_dn5 = assign13990_e18341_d_n5;
        locals.var_t1_dn6 = assign13990_e18341_d_n6;
        locals.var_t1_dn8 = assign13990_e18341_d_n8;
        locals.var_t1_dn10 = assign13990_e18341_d_n10;
        locals.var_t1_dn11 = assign13990_e18341_d_n11;
        locals.var_t1_dn12 = assign13990_e18341_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign14000_e18349, assign14000_e18349_d_n0, assign14000_e18349_d_n2, assign14000_e18349_d_n4, assign14000_e18349_d_n5, assign14000_e18349_d_n6, assign14000_e18349_d_n8, assign14000_e18349_d_n10, assign14000_e18349_d_n11, assign14000_e18349_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14000_e18345: f64 = (2.0 * locals.var_t1);
        let assign14000_e18347: f64 = (assign14000_e18345 / 0.01);
        (assign14000_e18347, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn11) / 0.01), ((2.0 * locals.var_t1_dn12) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign14000_e18349;
        locals.var_tmf1_dn0 = assign14000_e18349_d_n0;
        locals.var_tmf1_dn2 = assign14000_e18349_d_n2;
        locals.var_tmf1_dn4 = assign14000_e18349_d_n4;
        locals.var_tmf1_dn5 = assign14000_e18349_d_n5;
        locals.var_tmf1_dn6 = assign14000_e18349_d_n6;
        locals.var_tmf1_dn8 = assign14000_e18349_d_n8;
        locals.var_tmf1_dn10 = assign14000_e18349_d_n10;
        locals.var_tmf1_dn11 = assign14000_e18349_d_n11;
        locals.var_tmf1_dn12 = assign14000_e18349_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign14010_e18389, assign14010_e18389_d_n0, assign14010_e18389_d_n2, assign14010_e18389_d_n4, assign14010_e18389_d_n5, assign14010_e18389_d_n6, assign14010_e18389_d_n8, assign14010_e18389_d_n10, assign14010_e18389_d_n11, assign14010_e18389_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14010_e18355: f64 = (1.0 / 2.0);
        let assign14010_e18359: f64 = (1.0 / 6.0);
        let assign14010_e18363: f64 = (1.0 / 24.0);
        let assign14010_e18367: f64 = (1.0 / 120.0);
        let assign14010_e18371: f64 = (1.0 / 720.0);
        let assign14010_e18375: f64 = (1.0 / 5040.0);
        let assign14010_e18376: f64 = (locals.var_tmf1 * assign14010_e18375);
        let assign14010_e18377: f64 = (assign14010_e18371 + assign14010_e18376);
        let assign14010_e18378: f64 = (locals.var_tmf1 * assign14010_e18377);
        let assign14010_e18379: f64 = (assign14010_e18367 + assign14010_e18378);
        let assign14010_e18380: f64 = (locals.var_tmf1 * assign14010_e18379);
        let assign14010_e18381: f64 = (assign14010_e18363 + assign14010_e18380);
        let assign14010_e18382: f64 = (locals.var_tmf1 * assign14010_e18381);
        let assign14010_e18383: f64 = (assign14010_e18359 + assign14010_e18382);
        let assign14010_e18384: f64 = (locals.var_tmf1 * assign14010_e18383);
        let assign14010_e18385: f64 = (assign14010_e18355 + assign14010_e18384);
        let assign14010_e18386: f64 = (locals.var_tmf1 * assign14010_e18385);
        let assign14010_e18387: f64 = (1.0 + assign14010_e18386);
        (assign14010_e18387, ((locals.var_tmf1_dn0 * assign14010_e18385) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign14010_e18383) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign14010_e18381) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign14010_e18379) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign14010_e18377) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign14010_e18375))))))))))), ((locals.var_tmf1_dn2 * assign14010_e18385) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign14010_e18383) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign14010_e18381) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign14010_e18379) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign14010_e18377) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign14010_e18375))))))))))), ((locals.var_tmf1_dn4 * assign14010_e18385) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign14010_e18383) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign14010_e18381) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign14010_e18379) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign14010_e18377) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign14010_e18375))))))))))), ((locals.var_tmf1_dn5 * assign14010_e18385) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign14010_e18383) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign14010_e18381) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign14010_e18379) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign14010_e18377) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign14010_e18375))))))))))), ((locals.var_tmf1_dn6 * assign14010_e18385) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign14010_e18383) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign14010_e18381) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign14010_e18379) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign14010_e18377) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign14010_e18375))))))))))), ((locals.var_tmf1_dn8 * assign14010_e18385) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign14010_e18383) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign14010_e18381) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign14010_e18379) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign14010_e18377) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign14010_e18375))))))))))), ((locals.var_tmf1_dn10 * assign14010_e18385) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign14010_e18383) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign14010_e18381) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign14010_e18379) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign14010_e18377) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign14010_e18375))))))))))), ((locals.var_tmf1_dn11 * assign14010_e18385) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign14010_e18383) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign14010_e18381) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign14010_e18379) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign14010_e18377) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign14010_e18375))))))))))), ((locals.var_tmf1_dn12 * assign14010_e18385) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign14010_e18383) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign14010_e18381) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign14010_e18379) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign14010_e18377) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign14010_e18375))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign14010_e18389;
        locals.var_tmf2_dn0 = assign14010_e18389_d_n0;
        locals.var_tmf2_dn2 = assign14010_e18389_d_n2;
        locals.var_tmf2_dn4 = assign14010_e18389_d_n4;
        locals.var_tmf2_dn5 = assign14010_e18389_d_n5;
        locals.var_tmf2_dn6 = assign14010_e18389_d_n6;
        locals.var_tmf2_dn8 = assign14010_e18389_d_n8;
        locals.var_tmf2_dn10 = assign14010_e18389_d_n10;
        locals.var_tmf2_dn11 = assign14010_e18389_d_n11;
        locals.var_tmf2_dn12 = assign14010_e18389_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign14020_e18425, assign14020_e18425_d_n0, assign14020_e18425_d_n2, assign14020_e18425_d_n4, assign14020_e18425_d_n5, assign14020_e18425_d_n6, assign14020_e18425_d_n8, assign14020_e18425_d_n10, assign14020_e18425_d_n11, assign14020_e18425_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14020_e18393: f64 = (1.0 / 2.0);
        let assign14020_e18397: f64 = (1.0 / 3.0);
        let assign14020_e18401: f64 = (1.0 / 8.0);
        let assign14020_e18405: f64 = (1.0 / 30.0);
        let assign14020_e18409: f64 = (1.0 / 144.0);
        let assign14020_e18413: f64 = (1.0 / 840.0);
        let assign14020_e18414: f64 = (locals.var_tmf1 * assign14020_e18413);
        let assign14020_e18415: f64 = (assign14020_e18409 + assign14020_e18414);
        let assign14020_e18416: f64 = (locals.var_tmf1 * assign14020_e18415);
        let assign14020_e18417: f64 = (assign14020_e18405 + assign14020_e18416);
        let assign14020_e18418: f64 = (locals.var_tmf1 * assign14020_e18417);
        let assign14020_e18419: f64 = (assign14020_e18401 + assign14020_e18418);
        let assign14020_e18420: f64 = (locals.var_tmf1 * assign14020_e18419);
        let assign14020_e18421: f64 = (assign14020_e18397 + assign14020_e18420);
        let assign14020_e18422: f64 = (locals.var_tmf1 * assign14020_e18421);
        let assign14020_e18423: f64 = (assign14020_e18393 + assign14020_e18422);
        (assign14020_e18423, ((locals.var_tmf1_dn0 * assign14020_e18421) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign14020_e18419) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign14020_e18417) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign14020_e18415) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign14020_e18413))))))))), ((locals.var_tmf1_dn2 * assign14020_e18421) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign14020_e18419) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign14020_e18417) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign14020_e18415) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign14020_e18413))))))))), ((locals.var_tmf1_dn4 * assign14020_e18421) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign14020_e18419) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign14020_e18417) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign14020_e18415) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign14020_e18413))))))))), ((locals.var_tmf1_dn5 * assign14020_e18421) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign14020_e18419) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign14020_e18417) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign14020_e18415) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign14020_e18413))))))))), ((locals.var_tmf1_dn6 * assign14020_e18421) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign14020_e18419) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign14020_e18417) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign14020_e18415) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign14020_e18413))))))))), ((locals.var_tmf1_dn8 * assign14020_e18421) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign14020_e18419) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign14020_e18417) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign14020_e18415) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign14020_e18413))))))))), ((locals.var_tmf1_dn10 * assign14020_e18421) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign14020_e18419) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign14020_e18417) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign14020_e18415) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign14020_e18413))))))))), ((locals.var_tmf1_dn11 * assign14020_e18421) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign14020_e18419) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign14020_e18417) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign14020_e18415) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign14020_e18413))))))))), ((locals.var_tmf1_dn12 * assign14020_e18421) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign14020_e18419) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign14020_e18417) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign14020_e18415) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign14020_e18413))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn8, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12,)
    }
};
        locals.var_tmf3 = assign14020_e18425;
        locals.var_tmf3_dn0 = assign14020_e18425_d_n0;
        locals.var_tmf3_dn2 = assign14020_e18425_d_n2;
        locals.var_tmf3_dn4 = assign14020_e18425_d_n4;
        locals.var_tmf3_dn5 = assign14020_e18425_d_n5;
        locals.var_tmf3_dn6 = assign14020_e18425_d_n6;
        locals.var_tmf3_dn8 = assign14020_e18425_d_n8;
        locals.var_tmf3_dn10 = assign14020_e18425_d_n10;
        locals.var_tmf3_dn11 = assign14020_e18425_d_n11;
        locals.var_tmf3_dn12 = assign14020_e18425_d_n12;
        locals.var_tmf3_rv = 0.0;

        let (assign14030_e18431, assign14030_e18431_d_n0, assign14030_e18431_d_n2, assign14030_e18431_d_n4, assign14030_e18431_d_n5, assign14030_e18431_d_n6, assign14030_e18431_d_n8, assign14030_e18431_d_n10, assign14030_e18431_d_n11, assign14030_e18431_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14030_e18429: f64 = (0.01 / locals.var_tmf2);
        (assign14030_e18429, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign14030_e18431;
        locals.var_t6_dn0 = assign14030_e18431_d_n0;
        locals.var_t6_dn2 = assign14030_e18431_d_n2;
        locals.var_t6_dn4 = assign14030_e18431_d_n4;
        locals.var_t6_dn5 = assign14030_e18431_d_n5;
        locals.var_t6_dn6 = assign14030_e18431_d_n6;
        locals.var_t6_dn8 = assign14030_e18431_d_n8;
        locals.var_t6_dn10 = assign14030_e18431_d_n10;
        locals.var_t6_dn11 = assign14030_e18431_d_n11;
        locals.var_t6_dn12 = assign14030_e18431_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign14040_e18442, assign14040_e18442_d_n0, assign14040_e18442_d_n2, assign14040_e18442_d_n4, assign14040_e18442_d_n5, assign14040_e18442_d_n6, assign14040_e18442_d_n8, assign14040_e18442_d_n10, assign14040_e18442_d_n11, assign14040_e18442_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14040_e18434: f64 = (-2.0);
        let assign14040_e18436: f64 = (assign14040_e18434 * locals.var_tmf3);
        let assign14040_e18439: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign14040_e18440: f64 = (assign14040_e18436 / assign14040_e18439);
        (assign14040_e18440, ((((assign14040_e18434 * locals.var_tmf3_dn0) * assign14040_e18439) - (assign14040_e18436 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign14040_e18439 * assign14040_e18439)), ((((assign14040_e18434 * locals.var_tmf3_dn2) * assign14040_e18439) - (assign14040_e18436 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign14040_e18439 * assign14040_e18439)), ((((assign14040_e18434 * locals.var_tmf3_dn4) * assign14040_e18439) - (assign14040_e18436 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign14040_e18439 * assign14040_e18439)), ((((assign14040_e18434 * locals.var_tmf3_dn5) * assign14040_e18439) - (assign14040_e18436 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign14040_e18439 * assign14040_e18439)), ((((assign14040_e18434 * locals.var_tmf3_dn6) * assign14040_e18439) - (assign14040_e18436 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign14040_e18439 * assign14040_e18439)), ((((assign14040_e18434 * locals.var_tmf3_dn8) * assign14040_e18439) - (assign14040_e18436 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign14040_e18439 * assign14040_e18439)), ((((assign14040_e18434 * locals.var_tmf3_dn10) * assign14040_e18439) - (assign14040_e18436 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign14040_e18439 * assign14040_e18439)), ((((assign14040_e18434 * locals.var_tmf3_dn11) * assign14040_e18439) - (assign14040_e18436 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign14040_e18439 * assign14040_e18439)), ((((assign14040_e18434 * locals.var_tmf3_dn12) * assign14040_e18439) - (assign14040_e18436 * ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)))) / (assign14040_e18439 * assign14040_e18439)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign14040_e18442;
        locals.var_t2_dn0 = assign14040_e18442_d_n0;
        locals.var_t2_dn2 = assign14040_e18442_d_n2;
        locals.var_t2_dn4 = assign14040_e18442_d_n4;
        locals.var_t2_dn5 = assign14040_e18442_d_n5;
        locals.var_t2_dn6 = assign14040_e18442_d_n6;
        locals.var_t2_dn8 = assign14040_e18442_d_n8;
        locals.var_t2_dn10 = assign14040_e18442_d_n10;
        locals.var_t2_dn11 = assign14040_e18442_d_n11;
        locals.var_t2_dn12 = assign14040_e18442_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign14050_e18450, assign14050_e18450_d_n0, assign14050_e18450_d_n2, assign14050_e18450_d_n4, assign14050_e18450_d_n5, assign14050_e18450_d_n6, assign14050_e18450_d_n8, assign14050_e18450_d_n10, assign14050_e18450_d_n11, assign14050_e18450_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14050_e18447: f64 = (locals.var_pb0s + locals.var_t6);
        let assign14050_e18448: f64 = (1.1 - assign14050_e18447);
        (assign14050_e18448, (-(locals.var_pb0s_dn0 + locals.var_t6_dn0)), (-(locals.var_pb0s_dn2 + locals.var_t6_dn2)), (-(locals.var_pb0s_dn4 + locals.var_t6_dn4)), (-(locals.var_pb0s_dn5 + locals.var_t6_dn5)), (-(locals.var_pb0s_dn6 + locals.var_t6_dn6)), (-(locals.var_pb0s_dn8 + locals.var_t6_dn8)), (-(locals.var_pb0s_dn10 + locals.var_t6_dn10)), (-(locals.var_pb0s_dn11 + locals.var_t6_dn11)), (-(locals.var_pb0s_dn12 + locals.var_t6_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign14050_e18450;
        locals.var_t1_dn0 = assign14050_e18450_d_n0;
        locals.var_t1_dn2 = assign14050_e18450_d_n2;
        locals.var_t1_dn4 = assign14050_e18450_d_n4;
        locals.var_t1_dn5 = assign14050_e18450_d_n5;
        locals.var_t1_dn6 = assign14050_e18450_d_n6;
        locals.var_t1_dn8 = assign14050_e18450_d_n8;
        locals.var_t1_dn10 = assign14050_e18450_d_n10;
        locals.var_t1_dn11 = assign14050_e18450_d_n11;
        locals.var_t1_dn12 = assign14050_e18450_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign14060_e18463, assign14060_e18463_d_n0, assign14060_e18463_d_n2, assign14060_e18463_d_n4, assign14060_e18463_d_n5, assign14060_e18463_d_n6, assign14060_e18463_d_n8, assign14060_e18463_d_n10, assign14060_e18463_d_n11, assign14060_e18463_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14060_e18454: f64 = (locals.var_t1 * locals.var_t1);
        let assign14060_e18457: f64 = (4.0 * 0.05);
        let assign14060_e18459: f64 = (assign14060_e18457 * 0.05);
        let assign14060_e18460: f64 = (assign14060_e18454 + assign14060_e18459);
        let assign14060_e18461: f64 = (assign14060_e18460).sqrt();
        (assign14060_e18461, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign14060_e18461)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign14060_e18461)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign14060_e18461)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign14060_e18461)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign14060_e18461)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign14060_e18461)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign14060_e18461)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign14060_e18461)), (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign14060_e18461)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign14060_e18463;
        locals.var_tmf2_dn0 = assign14060_e18463_d_n0;
        locals.var_tmf2_dn2 = assign14060_e18463_d_n2;
        locals.var_tmf2_dn4 = assign14060_e18463_d_n4;
        locals.var_tmf2_dn5 = assign14060_e18463_d_n5;
        locals.var_tmf2_dn6 = assign14060_e18463_d_n6;
        locals.var_tmf2_dn8 = assign14060_e18463_d_n8;
        locals.var_tmf2_dn10 = assign14060_e18463_d_n10;
        locals.var_tmf2_dn11 = assign14060_e18463_d_n11;
        locals.var_tmf2_dn12 = assign14060_e18463_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign14070_e18473, assign14070_e18473_d_n0, assign14070_e18473_d_n2, assign14070_e18473_d_n4, assign14070_e18473_d_n5, assign14070_e18473_d_n6, assign14070_e18473_d_n8, assign14070_e18473_d_n10, assign14070_e18473_d_n11, assign14070_e18473_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14070_e18469: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign14070_e18470: f64 = (1.0 + assign14070_e18469);
        let assign14070_e18471: f64 = (0.5 * assign14070_e18470);
        (assign14070_e18471, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn12 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign14070_e18473;
        locals.var_t0_dn0 = assign14070_e18473_d_n0;
        locals.var_t0_dn2 = assign14070_e18473_d_n2;
        locals.var_t0_dn4 = assign14070_e18473_d_n4;
        locals.var_t0_dn5 = assign14070_e18473_d_n5;
        locals.var_t0_dn6 = assign14070_e18473_d_n6;
        locals.var_t0_dn8 = assign14070_e18473_d_n8;
        locals.var_t0_dn10 = assign14070_e18473_d_n10;
        locals.var_t0_dn11 = assign14070_e18473_d_n11;
        locals.var_t0_dn12 = assign14070_e18473_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign14080_e18485, assign14080_e18485_d_n0, assign14080_e18485_d_n2, assign14080_e18485_d_n4, assign14080_e18485_d_n5, assign14080_e18485_d_n6, assign14080_e18485_d_n8, assign14080_e18485_d_n10, assign14080_e18485_d_n11, assign14080_e18485_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14080_e18478: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign14080_e18479: f64 = (0.5 * assign14080_e18478);
        let assign14080_e18482: f64 = (1e-10 * 0.05);
        let assign14080_e18483: f64 = (assign14080_e18479 + assign14080_e18482);
        (assign14080_e18483, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign14080_e18485;
        locals.var_t2_dn0 = assign14080_e18485_d_n0;
        locals.var_t2_dn2 = assign14080_e18485_d_n2;
        locals.var_t2_dn4 = assign14080_e18485_d_n4;
        locals.var_t2_dn5 = assign14080_e18485_d_n5;
        locals.var_t2_dn6 = assign14080_e18485_d_n6;
        locals.var_t2_dn8 = assign14080_e18485_d_n8;
        locals.var_t2_dn10 = assign14080_e18485_d_n10;
        locals.var_t2_dn11 = assign14080_e18485_d_n11;
        locals.var_t2_dn12 = assign14080_e18485_d_n12;
        locals.var_t2_rv = 0.0;

        let assign14090_e18488: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign14090_e18488;
        locals.var_guard256_rv = 0.0;

        let (assign14100_e18494, assign14100_e18494_d_n0, assign14100_e18494_d_n2, assign14100_e18494_d_n4, assign14100_e18494_d_n5, assign14100_e18494_d_n6, assign14100_e18494_d_n8, assign14100_e18494_d_n10, assign14100_e18494_d_n11, assign14100_e18494_d_n12,) = {
    if ((locals.var_guard255 != 0.0) && (locals.var_guard256 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign14100_e18494;
        locals.var_t2_dn0 = assign14100_e18494_d_n0;
        locals.var_t2_dn2 = assign14100_e18494_d_n2;
        locals.var_t2_dn4 = assign14100_e18494_d_n4;
        locals.var_t2_dn5 = assign14100_e18494_d_n5;
        locals.var_t2_dn6 = assign14100_e18494_d_n6;
        locals.var_t2_dn8 = assign14100_e18494_d_n8;
        locals.var_t2_dn10 = assign14100_e18494_d_n10;
        locals.var_t2_dn11 = assign14100_e18494_d_n11;
        locals.var_t2_dn12 = assign14100_e18494_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign14110_e18500, assign14110_e18500_d_n0, assign14110_e18500_d_n2, assign14110_e18500_d_n4, assign14110_e18500_d_n5, assign14110_e18500_d_n6, assign14110_e18500_d_n8, assign14110_e18500_d_n10, assign14110_e18500_d_n11, assign14110_e18500_d_n12,) = {
    if ((locals.var_guard255 != 0.0) && (locals.var_guard256 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign14110_e18500;
        locals.var_t0_dn0 = assign14110_e18500_d_n0;
        locals.var_t0_dn2 = assign14110_e18500_d_n2;
        locals.var_t0_dn4 = assign14110_e18500_d_n4;
        locals.var_t0_dn5 = assign14110_e18500_d_n5;
        locals.var_t0_dn6 = assign14110_e18500_d_n6;
        locals.var_t0_dn8 = assign14110_e18500_d_n8;
        locals.var_t0_dn10 = assign14110_e18500_d_n10;
        locals.var_t0_dn11 = assign14110_e18500_d_n11;
        locals.var_t0_dn12 = assign14110_e18500_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign14120_e18512, assign14120_e18512_d_n0, assign14120_e18512_d_n2, assign14120_e18512_d_n4, assign14120_e18512_d_n5, assign14120_e18512_d_n6, assign14120_e18512_d_n8, assign14120_e18512_d_n10, assign14120_e18512_d_n11, assign14120_e18512_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14120_e18504: f64 = (locals.var_c_fox * locals.var_beta);
        let assign14120_e18506: f64 = (assign14120_e18504 * locals.var_ptl0);
        let assign14120_e18509: f64 = (locals.var_t2).powf(p.p240);
        let assign14120_e18510: f64 = (assign14120_e18506 * assign14120_e18509);
        (assign14120_e18510, ((((locals.var_c_fox_dn0 * locals.var_beta) * locals.var_ptl0) * assign14120_e18509) + (assign14120_e18506 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn0)) } } else { (assign14120_e18509 * (p.p240 * (locals.var_t2_dn0 / locals.var_t2))) })), ((((locals.var_c_fox_dn2 * locals.var_beta) * locals.var_ptl0) * assign14120_e18509) + (assign14120_e18506 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn2)) } } else { (assign14120_e18509 * (p.p240 * (locals.var_t2_dn2 / locals.var_t2))) })), (((((locals.var_c_fox_dn4 * locals.var_beta) + (locals.var_c_fox * locals.var_beta_dn4)) * locals.var_ptl0) * assign14120_e18509) + (assign14120_e18506 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn4)) } } else { (assign14120_e18509 * (p.p240 * (locals.var_t2_dn4 / locals.var_t2))) })), ((((locals.var_c_fox_dn5 * locals.var_beta) * locals.var_ptl0) * assign14120_e18509) + (assign14120_e18506 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn5)) } } else { (assign14120_e18509 * (p.p240 * (locals.var_t2_dn5 / locals.var_t2))) })), ((((locals.var_c_fox_dn6 * locals.var_beta) * locals.var_ptl0) * assign14120_e18509) + (assign14120_e18506 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn6)) } } else { (assign14120_e18509 * (p.p240 * (locals.var_t2_dn6 / locals.var_t2))) })), ((((locals.var_c_fox_dn8 * locals.var_beta) * locals.var_ptl0) * assign14120_e18509) + (assign14120_e18506 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn8)) } } else { (assign14120_e18509 * (p.p240 * (locals.var_t2_dn8 / locals.var_t2))) })), ((((locals.var_c_fox_dn10 * locals.var_beta) * locals.var_ptl0) * assign14120_e18509) + (assign14120_e18506 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn10)) } } else { (assign14120_e18509 * (p.p240 * (locals.var_t2_dn10 / locals.var_t2))) })), ((((locals.var_c_fox_dn11 * locals.var_beta) * locals.var_ptl0) * assign14120_e18509) + (assign14120_e18506 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn11)) } } else { (assign14120_e18509 * (p.p240 * (locals.var_t2_dn11 / locals.var_t2))) })), ((((locals.var_c_fox_dn12 * locals.var_beta) * locals.var_ptl0) * assign14120_e18509) + (assign14120_e18506 * if 0.0 == 0.0 && ((p.p240) as f64).is_finite() && ((p.p240) as f64).fract() == 0.0 { if p.p240 == 0.0 { 0.0 } else { (p.p240 * ((locals.var_t2).powf(p.p240 - 1.0) * locals.var_t2_dn12)) } } else { (assign14120_e18509 * (p.p240 * (locals.var_t2_dn12 / locals.var_t2))) })),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign14120_e18512;
        locals.var_t9_dn0 = assign14120_e18512_d_n0;
        locals.var_t9_dn2 = assign14120_e18512_d_n2;
        locals.var_t9_dn4 = assign14120_e18512_d_n4;
        locals.var_t9_dn5 = assign14120_e18512_d_n5;
        locals.var_t9_dn6 = assign14120_e18512_d_n6;
        locals.var_t9_dn8 = assign14120_e18512_d_n8;
        locals.var_t9_dn10 = assign14120_e18512_d_n10;
        locals.var_t9_dn11 = assign14120_e18512_d_n11;
        locals.var_t9_dn12 = assign14120_e18512_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign14130_e18530, assign14130_e18530_d_n0, assign14130_e18530_d_n2, assign14130_e18530_d_n4, assign14130_e18530_d_n5, assign14130_e18530_d_n6, assign14130_e18530_d_n8, assign14130_e18530_d_n10, assign14130_e18530_d_n11, assign14130_e18530_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14130_e18517: f64 = (locals.var_vdsz * p.p241);
        let assign14130_e18518: f64 = (1.0 + assign14130_e18517);
        let assign14130_e18521: f64 = (locals.var_vdsz * locals.var_pt40);
        let assign14130_e18524: f64 = (locals.var_pb0s + locals.var_t6);
        let assign14130_e18526: f64 = (assign14130_e18524 - locals.var_vbsz);
        let assign14130_e18527: f64 = (assign14130_e18521 * assign14130_e18526);
        let assign14130_e18528: f64 = (assign14130_e18518 + assign14130_e18527);
        (assign14130_e18528, ((locals.var_vdsz_dn0 * p.p241) + (((locals.var_vdsz_dn0 * locals.var_pt40) * assign14130_e18526) + (assign14130_e18521 * ((locals.var_pb0s_dn0 + locals.var_t6_dn0) - locals.var_vbsz_dn0)))), ((locals.var_vdsz_dn2 * p.p241) + (((locals.var_vdsz_dn2 * locals.var_pt40) * assign14130_e18526) + (assign14130_e18521 * ((locals.var_pb0s_dn2 + locals.var_t6_dn2) - locals.var_vbsz_dn2)))), ((locals.var_vdsz_dn4 * p.p241) + (((locals.var_vdsz_dn4 * locals.var_pt40) * assign14130_e18526) + (assign14130_e18521 * ((locals.var_pb0s_dn4 + locals.var_t6_dn4) - locals.var_vbsz_dn4)))), ((locals.var_vdsz_dn5 * p.p241) + (((locals.var_vdsz_dn5 * locals.var_pt40) * assign14130_e18526) + (assign14130_e18521 * ((locals.var_pb0s_dn5 + locals.var_t6_dn5) - locals.var_vbsz_dn5)))), ((locals.var_vdsz_dn6 * p.p241) + (((locals.var_vdsz_dn6 * locals.var_pt40) * assign14130_e18526) + (assign14130_e18521 * ((locals.var_pb0s_dn6 + locals.var_t6_dn6) - locals.var_vbsz_dn6)))), ((locals.var_vdsz_dn8 * p.p241) + (((locals.var_vdsz_dn8 * locals.var_pt40) * assign14130_e18526) + (assign14130_e18521 * ((locals.var_pb0s_dn8 + locals.var_t6_dn8) - locals.var_vbsz_dn8)))), ((locals.var_vdsz_dn10 * p.p241) + (((locals.var_vdsz_dn10 * locals.var_pt40) * assign14130_e18526) + (assign14130_e18521 * ((locals.var_pb0s_dn10 + locals.var_t6_dn10) - locals.var_vbsz_dn10)))), ((locals.var_vdsz_dn11 * p.p241) + (((locals.var_vdsz_dn11 * locals.var_pt40) * assign14130_e18526) + (assign14130_e18521 * ((locals.var_pb0s_dn11 + locals.var_t6_dn11) - locals.var_vbsz_dn11)))), ((locals.var_vdsz_dn12 * p.p241) + (((locals.var_vdsz_dn12 * locals.var_pt40) * assign14130_e18526) + (assign14130_e18521 * ((locals.var_pb0s_dn12 + locals.var_t6_dn12) - locals.var_vbsz_dn12)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign14130_e18530;
        locals.var_t4_dn0 = assign14130_e18530_d_n0;
        locals.var_t4_dn2 = assign14130_e18530_d_n2;
        locals.var_t4_dn4 = assign14130_e18530_d_n4;
        locals.var_t4_dn5 = assign14130_e18530_d_n5;
        locals.var_t4_dn6 = assign14130_e18530_d_n6;
        locals.var_t4_dn8 = assign14130_e18530_d_n8;
        locals.var_t4_dn10 = assign14130_e18530_d_n10;
        locals.var_t4_dn11 = assign14130_e18530_d_n11;
        locals.var_t4_dn12 = assign14130_e18530_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign14140_e18536, assign14140_e18536_d_n0, assign14140_e18536_d_n2, assign14140_e18536_d_n4, assign14140_e18536_d_n5, assign14140_e18536_d_n6, assign14140_e18536_d_n8, assign14140_e18536_d_n10, assign14140_e18536_d_n11, assign14140_e18536_d_n12,) = {
    if (locals.var_guard255 != 0.0) {
        let assign14140_e18534: f64 = (locals.var_t9 * locals.var_t4);
        (assign14140_e18534, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn11 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn11)), ((locals.var_t9_dn12 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn12)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign14140_e18536;
        locals.var_t9_dn0 = assign14140_e18536_d_n0;
        locals.var_t9_dn2 = assign14140_e18536_d_n2;
        locals.var_t9_dn4 = assign14140_e18536_d_n4;
        locals.var_t9_dn5 = assign14140_e18536_d_n5;
        locals.var_t9_dn6 = assign14140_e18536_d_n6;
        locals.var_t9_dn8 = assign14140_e18536_d_n8;
        locals.var_t9_dn10 = assign14140_e18536_d_n10;
        locals.var_t9_dn11 = assign14140_e18536_d_n11;
        locals.var_t9_dn12 = assign14140_e18536_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign14150_e18541, assign14150_e18541_d_n0, assign14150_e18541_d_n2, assign14150_e18541_d_n4, assign14150_e18541_d_n5, assign14150_e18541_d_n6, assign14150_e18541_d_n8, assign14150_e18541_d_n10, assign14150_e18541_d_n11, assign14150_e18541_d_n12,) = {
    if (locals.var_guard255 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign14150_e18541;
        locals.var_t9_dn0 = assign14150_e18541_d_n0;
        locals.var_t9_dn2 = assign14150_e18541_d_n2;
        locals.var_t9_dn4 = assign14150_e18541_d_n4;
        locals.var_t9_dn5 = assign14150_e18541_d_n5;
        locals.var_t9_dn6 = assign14150_e18541_d_n6;
        locals.var_t9_dn8 = assign14150_e18541_d_n8;
        locals.var_t9_dn10 = assign14150_e18541_d_n10;
        locals.var_t9_dn11 = assign14150_e18541_d_n11;
        locals.var_t9_dn12 = assign14150_e18541_d_n12;
        locals.var_t9_rv = 0.0;

        let assign14160_e18544: f64 = (locals.var_t9 + locals.var_t8);
        let assign14160_e18546: f64 = if assign14160_e18544 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign14160_e18546;
        locals.var_guard257_rv = 0.0;

        let (assign14170_e18554, assign14170_e18554_d_n0, assign14170_e18554_d_n2, assign14170_e18554_d_n4, assign14170_e18554_d_n5, assign14170_e18554_d_n6, assign14170_e18554_d_n8, assign14170_e18554_d_n10, assign14170_e18554_d_n11, assign14170_e18554_d_n12,) = {
    if (locals.var_guard257 != 0.0) {
        let assign14170_e18551: f64 = (locals.var_t9 + locals.var_t8);
        let assign14170_e18552: f64 = (locals.var_pbds * assign14170_e18551);
        (assign14170_e18552, ((locals.var_pbds_dn0 * assign14170_e18551) + (locals.var_pbds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pbds_dn2 * assign14170_e18551) + (locals.var_pbds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pbds_dn4 * assign14170_e18551) + (locals.var_pbds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pbds_dn5 * assign14170_e18551) + (locals.var_pbds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pbds_dn6 * assign14170_e18551) + (locals.var_pbds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pbds_dn8 * assign14170_e18551) + (locals.var_pbds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pbds_dn10 * assign14170_e18551) + (locals.var_pbds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pbds_dn11 * assign14170_e18551) + (locals.var_pbds * (locals.var_t9_dn11 + locals.var_t8_dn11))), ((locals.var_pbds_dn12 * assign14170_e18551) + (locals.var_pbds * (locals.var_t9_dn12 + locals.var_t8_dn12))),)
    } else {
        (locals.var_idd0, locals.var_idd0_dn0, locals.var_idd0_dn2, locals.var_idd0_dn4, locals.var_idd0_dn5, locals.var_idd0_dn6, locals.var_idd0_dn8, locals.var_idd0_dn10, locals.var_idd0_dn11, locals.var_idd0_dn12,)
    }
};
        locals.var_idd0 = assign14170_e18554;
        locals.var_idd0_dn0 = assign14170_e18554_d_n0;
        locals.var_idd0_dn2 = assign14170_e18554_d_n2;
        locals.var_idd0_dn4 = assign14170_e18554_d_n4;
        locals.var_idd0_dn5 = assign14170_e18554_d_n5;
        locals.var_idd0_dn6 = assign14170_e18554_d_n6;
        locals.var_idd0_dn8 = assign14170_e18554_d_n8;
        locals.var_idd0_dn10 = assign14170_e18554_d_n10;
        locals.var_idd0_dn11 = assign14170_e18554_d_n11;
        locals.var_idd0_dn12 = assign14170_e18554_d_n12;
        locals.var_idd0_rv = 0.0;

        let (assign14180_e18562, assign14180_e18562_d_n0, assign14180_e18562_d_n2, assign14180_e18562_d_n4, assign14180_e18562_d_n5, assign14180_e18562_d_n6, assign14180_e18562_d_n8, assign14180_e18562_d_n10, assign14180_e18562_d_n11, assign14180_e18562_d_n12,) = {
    if (locals.var_guard257 != 0.0) {
        let assign14180_e18558: f64 = (locals.var_betawl * locals.var_idd0);
        let assign14180_e18560: f64 = (assign14180_e18558 * locals.var_mub);
        (assign14180_e18560, ((((locals.var_betawl_dn0 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn0)) * locals.var_mub) + (assign14180_e18558 * locals.var_mub_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn2)) * locals.var_mub) + (assign14180_e18558 * locals.var_mub_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn4)) * locals.var_mub) + (assign14180_e18558 * locals.var_mub_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn5)) * locals.var_mub) + (assign14180_e18558 * locals.var_mub_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn6)) * locals.var_mub) + (assign14180_e18558 * locals.var_mub_dn6)), ((((locals.var_betawl_dn8 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn8)) * locals.var_mub) + (assign14180_e18558 * locals.var_mub_dn8)), ((((locals.var_betawl_dn10 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn10)) * locals.var_mub) + (assign14180_e18558 * locals.var_mub_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn11)) * locals.var_mub) + (assign14180_e18558 * locals.var_mub_dn11)), ((((locals.var_betawl_dn12 * locals.var_idd0) + (locals.var_betawl * locals.var_idd0_dn12)) * locals.var_mub) + (assign14180_e18558 * locals.var_mub_dn12)),)
    } else {
        (locals.var_idsptb, locals.var_idsptb_dn0, locals.var_idsptb_dn2, locals.var_idsptb_dn4, locals.var_idsptb_dn5, locals.var_idsptb_dn6, locals.var_idsptb_dn8, locals.var_idsptb_dn10, locals.var_idsptb_dn11, locals.var_idsptb_dn12,)
    }
};
        locals.var_idsptb = assign14180_e18562;
        locals.var_idsptb_dn0 = assign14180_e18562_d_n0;
        locals.var_idsptb_dn2 = assign14180_e18562_d_n2;
        locals.var_idsptb_dn4 = assign14180_e18562_d_n4;
        locals.var_idsptb_dn5 = assign14180_e18562_d_n5;
        locals.var_idsptb_dn6 = assign14180_e18562_d_n6;
        locals.var_idsptb_dn8 = assign14180_e18562_d_n8;
        locals.var_idsptb_dn10 = assign14180_e18562_d_n10;
        locals.var_idsptb_dn11 = assign14180_e18562_d_n11;
        locals.var_idsptb_dn12 = assign14180_e18562_d_n12;
        locals.var_idsptb_rv = 0.0;

        let assign14190_e18567: f64 = (locals.var_idspt * 0.05);
        let assign14190_e18568: f64 = (locals.var_idspt - assign14190_e18567);
        let assign14190_e18572: f64 = (locals.var_idspt * 0.05);
        let assign14190_e18575: f64 = if ((locals.var_idsptb > assign14190_e18568) && (assign14190_e18572 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard258 = assign14190_e18575;
        locals.var_guard258_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        locals: &mut StampLocals,
    ) {
        let (assign14200_e18587, assign14200_e18587_d_n0, assign14200_e18587_d_n2, assign14200_e18587_d_n4, assign14200_e18587_d_n5, assign14200_e18587_d_n6, assign14200_e18587_d_n8, assign14200_e18587_d_n10, assign14200_e18587_d_n11, assign14200_e18587_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14200_e18581: f64 = (locals.var_idsptb - locals.var_idspt);
        let assign14200_e18584: f64 = (locals.var_idspt * 0.05);
        let assign14200_e18585: f64 = (assign14200_e18581 + assign14200_e18584);
        (assign14200_e18585, ((locals.var_idsptb_dn0 - locals.var_idspt_dn0) + (locals.var_idspt_dn0 * 0.05)), ((locals.var_idsptb_dn2 - locals.var_idspt_dn2) + (locals.var_idspt_dn2 * 0.05)), ((locals.var_idsptb_dn4 - locals.var_idspt_dn4) + (locals.var_idspt_dn4 * 0.05)), ((locals.var_idsptb_dn5 - locals.var_idspt_dn5) + (locals.var_idspt_dn5 * 0.05)), ((locals.var_idsptb_dn6 - locals.var_idspt_dn6) + (locals.var_idspt_dn6 * 0.05)), ((locals.var_idsptb_dn8 - locals.var_idspt_dn8) + (locals.var_idspt_dn8 * 0.05)), ((locals.var_idsptb_dn10 - locals.var_idspt_dn10) + (locals.var_idspt_dn10 * 0.05)), ((locals.var_idsptb_dn11 - locals.var_idspt_dn11) + (locals.var_idspt_dn11 * 0.05)), ((locals.var_idsptb_dn12 - locals.var_idspt_dn12) + (locals.var_idspt_dn12 * 0.05)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign14200_e18587;
        locals.var_tmf1_dn0 = assign14200_e18587_d_n0;
        locals.var_tmf1_dn2 = assign14200_e18587_d_n2;
        locals.var_tmf1_dn4 = assign14200_e18587_d_n4;
        locals.var_tmf1_dn5 = assign14200_e18587_d_n5;
        locals.var_tmf1_dn6 = assign14200_e18587_d_n6;
        locals.var_tmf1_dn8 = assign14200_e18587_d_n8;
        locals.var_tmf1_dn10 = assign14200_e18587_d_n10;
        locals.var_tmf1_dn11 = assign14200_e18587_d_n11;
        locals.var_tmf1_dn12 = assign14200_e18587_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign14210_e18595, assign14210_e18595_d_n0, assign14210_e18595_d_n2, assign14210_e18595_d_n4, assign14210_e18595_d_n5, assign14210_e18595_d_n6, assign14210_e18595_d_n8, assign14210_e18595_d_n10, assign14210_e18595_d_n11, assign14210_e18595_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14210_e18593: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign14210_e18593, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn8, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12,)
    }
};
        locals.var_x2 = assign14210_e18595;
        locals.var_x2_dn0 = assign14210_e18595_d_n0;
        locals.var_x2_dn2 = assign14210_e18595_d_n2;
        locals.var_x2_dn4 = assign14210_e18595_d_n4;
        locals.var_x2_dn5 = assign14210_e18595_d_n5;
        locals.var_x2_dn6 = assign14210_e18595_d_n6;
        locals.var_x2_dn8 = assign14210_e18595_d_n8;
        locals.var_x2_dn10 = assign14210_e18595_d_n10;
        locals.var_x2_dn11 = assign14210_e18595_d_n11;
        locals.var_x2_dn12 = assign14210_e18595_d_n12;
        locals.var_x2_rv = 0.0;

        let (assign14220_e18607, assign14220_e18607_d_n0, assign14220_e18607_d_n2, assign14220_e18607_d_n4, assign14220_e18607_d_n5, assign14220_e18607_d_n6, assign14220_e18607_d_n8, assign14220_e18607_d_n10, assign14220_e18607_d_n11, assign14220_e18607_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14220_e18601: f64 = (locals.var_idspt * 0.05);
        let assign14220_e18604: f64 = (locals.var_idspt * 0.05);
        let assign14220_e18605: f64 = (assign14220_e18601 * assign14220_e18604);
        (assign14220_e18605, (((locals.var_idspt_dn0 * 0.05) * assign14220_e18604) + (assign14220_e18601 * (locals.var_idspt_dn0 * 0.05))), (((locals.var_idspt_dn2 * 0.05) * assign14220_e18604) + (assign14220_e18601 * (locals.var_idspt_dn2 * 0.05))), (((locals.var_idspt_dn4 * 0.05) * assign14220_e18604) + (assign14220_e18601 * (locals.var_idspt_dn4 * 0.05))), (((locals.var_idspt_dn5 * 0.05) * assign14220_e18604) + (assign14220_e18601 * (locals.var_idspt_dn5 * 0.05))), (((locals.var_idspt_dn6 * 0.05) * assign14220_e18604) + (assign14220_e18601 * (locals.var_idspt_dn6 * 0.05))), (((locals.var_idspt_dn8 * 0.05) * assign14220_e18604) + (assign14220_e18601 * (locals.var_idspt_dn8 * 0.05))), (((locals.var_idspt_dn10 * 0.05) * assign14220_e18604) + (assign14220_e18601 * (locals.var_idspt_dn10 * 0.05))), (((locals.var_idspt_dn11 * 0.05) * assign14220_e18604) + (assign14220_e18601 * (locals.var_idspt_dn11 * 0.05))), (((locals.var_idspt_dn12 * 0.05) * assign14220_e18604) + (assign14220_e18601 * (locals.var_idspt_dn12 * 0.05))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn8, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12,)
    }
};
        locals.var_xmax2 = assign14220_e18607;
        locals.var_xmax2_dn0 = assign14220_e18607_d_n0;
        locals.var_xmax2_dn2 = assign14220_e18607_d_n2;
        locals.var_xmax2_dn4 = assign14220_e18607_d_n4;
        locals.var_xmax2_dn5 = assign14220_e18607_d_n5;
        locals.var_xmax2_dn6 = assign14220_e18607_d_n6;
        locals.var_xmax2_dn8 = assign14220_e18607_d_n8;
        locals.var_xmax2_dn10 = assign14220_e18607_d_n10;
        locals.var_xmax2_dn11 = assign14220_e18607_d_n11;
        locals.var_xmax2_dn12 = assign14220_e18607_d_n12;
        locals.var_xmax2_rv = 0.0;

        let (assign14230_e18613, assign14230_e18613_d_n0, assign14230_e18613_d_n2, assign14230_e18613_d_n4, assign14230_e18613_d_n5, assign14230_e18613_d_n6, assign14230_e18613_d_n8, assign14230_e18613_d_n10, assign14230_e18613_d_n11, assign14230_e18613_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign14230_e18613;
        locals.var_xp_dn0 = assign14230_e18613_d_n0;
        locals.var_xp_dn2 = assign14230_e18613_d_n2;
        locals.var_xp_dn4 = assign14230_e18613_d_n4;
        locals.var_xp_dn5 = assign14230_e18613_d_n5;
        locals.var_xp_dn6 = assign14230_e18613_d_n6;
        locals.var_xp_dn8 = assign14230_e18613_d_n8;
        locals.var_xp_dn10 = assign14230_e18613_d_n10;
        locals.var_xp_dn11 = assign14230_e18613_d_n11;
        locals.var_xp_dn12 = assign14230_e18613_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign14240_e18619, assign14240_e18619_d_n0, assign14240_e18619_d_n2, assign14240_e18619_d_n4, assign14240_e18619_d_n5, assign14240_e18619_d_n6, assign14240_e18619_d_n8, assign14240_e18619_d_n10, assign14240_e18619_d_n11, assign14240_e18619_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign14240_e18619;
        locals.var_xmp_dn0 = assign14240_e18619_d_n0;
        locals.var_xmp_dn2 = assign14240_e18619_d_n2;
        locals.var_xmp_dn4 = assign14240_e18619_d_n4;
        locals.var_xmp_dn5 = assign14240_e18619_d_n5;
        locals.var_xmp_dn6 = assign14240_e18619_d_n6;
        locals.var_xmp_dn8 = assign14240_e18619_d_n8;
        locals.var_xmp_dn10 = assign14240_e18619_d_n10;
        locals.var_xmp_dn11 = assign14240_e18619_d_n11;
        locals.var_xmp_dn12 = assign14240_e18619_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign14250_e18625,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign14250_e18625;
        locals.var_m0_rv = 0.0;

        let (assign14260_e18631,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14260_e18631;
        locals.var_mm_rv = 0.0;

        let (assign14270_e18637, assign14270_e18637_d_n0, assign14270_e18637_d_n2, assign14270_e18637_d_n4, assign14270_e18637_d_n5, assign14270_e18637_d_n6, assign14270_e18637_d_n8, assign14270_e18637_d_n10, assign14270_e18637_d_n11, assign14270_e18637_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn8, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12,)
    }
};
        locals.var_arg = assign14270_e18637;
        locals.var_arg_dn0 = assign14270_e18637_d_n0;
        locals.var_arg_dn2 = assign14270_e18637_d_n2;
        locals.var_arg_dn4 = assign14270_e18637_d_n4;
        locals.var_arg_dn5 = assign14270_e18637_d_n5;
        locals.var_arg_dn6 = assign14270_e18637_d_n6;
        locals.var_arg_dn8 = assign14270_e18637_d_n8;
        locals.var_arg_dn10 = assign14270_e18637_d_n10;
        locals.var_arg_dn11 = assign14270_e18637_d_n11;
        locals.var_arg_dn12 = assign14270_e18637_d_n12;
        locals.var_arg_rv = 0.0;

        let (assign14280_e18643, assign14280_e18643_d_n0, assign14280_e18643_d_n2, assign14280_e18643_d_n4, assign14280_e18643_d_n5, assign14280_e18643_d_n6, assign14280_e18643_d_n8, assign14280_e18643_d_n10, assign14280_e18643_d_n11, assign14280_e18643_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign14280_e18643;
        locals.var_dnm_dn0 = assign14280_e18643_d_n0;
        locals.var_dnm_dn2 = assign14280_e18643_d_n2;
        locals.var_dnm_dn4 = assign14280_e18643_d_n4;
        locals.var_dnm_dn5 = assign14280_e18643_d_n5;
        locals.var_dnm_dn6 = assign14280_e18643_d_n6;
        locals.var_dnm_dn8 = assign14280_e18643_d_n8;
        locals.var_dnm_dn10 = assign14280_e18643_d_n10;
        locals.var_dnm_dn11 = assign14280_e18643_d_n11;
        locals.var_dnm_dn12 = assign14280_e18643_d_n12;
        locals.var_dnm_rv = 0.0;

        let (assign14290_e18651, assign14290_e18651_d_n0, assign14290_e18651_d_n2, assign14290_e18651_d_n4, assign14290_e18651_d_n5, assign14290_e18651_d_n6, assign14290_e18651_d_n8, assign14290_e18651_d_n10, assign14290_e18651_d_n11, assign14290_e18651_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14290_e18649: f64 = (locals.var_xp * locals.var_x2);
        (assign14290_e18649, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign14290_e18651;
        locals.var_xp_dn0 = assign14290_e18651_d_n0;
        locals.var_xp_dn2 = assign14290_e18651_d_n2;
        locals.var_xp_dn4 = assign14290_e18651_d_n4;
        locals.var_xp_dn5 = assign14290_e18651_d_n5;
        locals.var_xp_dn6 = assign14290_e18651_d_n6;
        locals.var_xp_dn8 = assign14290_e18651_d_n8;
        locals.var_xp_dn10 = assign14290_e18651_d_n10;
        locals.var_xp_dn11 = assign14290_e18651_d_n11;
        locals.var_xp_dn12 = assign14290_e18651_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign14300_e18659, assign14300_e18659_d_n0, assign14300_e18659_d_n2, assign14300_e18659_d_n4, assign14300_e18659_d_n5, assign14300_e18659_d_n6, assign14300_e18659_d_n8, assign14300_e18659_d_n10, assign14300_e18659_d_n11, assign14300_e18659_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14300_e18657: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign14300_e18657, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign14300_e18659;
        locals.var_xmp_dn0 = assign14300_e18659_d_n0;
        locals.var_xmp_dn2 = assign14300_e18659_d_n2;
        locals.var_xmp_dn4 = assign14300_e18659_d_n4;
        locals.var_xmp_dn5 = assign14300_e18659_d_n5;
        locals.var_xmp_dn6 = assign14300_e18659_d_n6;
        locals.var_xmp_dn8 = assign14300_e18659_d_n8;
        locals.var_xmp_dn10 = assign14300_e18659_d_n10;
        locals.var_xmp_dn11 = assign14300_e18659_d_n11;
        locals.var_xmp_dn12 = assign14300_e18659_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign14310_e18667, assign14310_e18667_d_n0, assign14310_e18667_d_n2, assign14310_e18667_d_n4, assign14310_e18667_d_n5, assign14310_e18667_d_n6, assign14310_e18667_d_n8, assign14310_e18667_d_n10, assign14310_e18667_d_n11, assign14310_e18667_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14310_e18665: f64 = (locals.var_xp * locals.var_x2);
        (assign14310_e18665, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign14310_e18667;
        locals.var_xp_dn0 = assign14310_e18667_d_n0;
        locals.var_xp_dn2 = assign14310_e18667_d_n2;
        locals.var_xp_dn4 = assign14310_e18667_d_n4;
        locals.var_xp_dn5 = assign14310_e18667_d_n5;
        locals.var_xp_dn6 = assign14310_e18667_d_n6;
        locals.var_xp_dn8 = assign14310_e18667_d_n8;
        locals.var_xp_dn10 = assign14310_e18667_d_n10;
        locals.var_xp_dn11 = assign14310_e18667_d_n11;
        locals.var_xp_dn12 = assign14310_e18667_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign14320_e18675, assign14320_e18675_d_n0, assign14320_e18675_d_n2, assign14320_e18675_d_n4, assign14320_e18675_d_n5, assign14320_e18675_d_n6, assign14320_e18675_d_n8, assign14320_e18675_d_n10, assign14320_e18675_d_n11, assign14320_e18675_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14320_e18673: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign14320_e18673, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign14320_e18675;
        locals.var_xmp_dn0 = assign14320_e18675_d_n0;
        locals.var_xmp_dn2 = assign14320_e18675_d_n2;
        locals.var_xmp_dn4 = assign14320_e18675_d_n4;
        locals.var_xmp_dn5 = assign14320_e18675_d_n5;
        locals.var_xmp_dn6 = assign14320_e18675_d_n6;
        locals.var_xmp_dn8 = assign14320_e18675_d_n8;
        locals.var_xmp_dn10 = assign14320_e18675_d_n10;
        locals.var_xmp_dn11 = assign14320_e18675_d_n11;
        locals.var_xmp_dn12 = assign14320_e18675_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign14330_e18683, assign14330_e18683_d_n0, assign14330_e18683_d_n2, assign14330_e18683_d_n4, assign14330_e18683_d_n5, assign14330_e18683_d_n6, assign14330_e18683_d_n8, assign14330_e18683_d_n10, assign14330_e18683_d_n11, assign14330_e18683_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14330_e18681: f64 = (locals.var_xp + locals.var_xmp);
        (assign14330_e18681, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn8, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12,)
    }
};
        locals.var_arg = assign14330_e18683;
        locals.var_arg_dn0 = assign14330_e18683_d_n0;
        locals.var_arg_dn2 = assign14330_e18683_d_n2;
        locals.var_arg_dn4 = assign14330_e18683_d_n4;
        locals.var_arg_dn5 = assign14330_e18683_d_n5;
        locals.var_arg_dn6 = assign14330_e18683_d_n6;
        locals.var_arg_dn8 = assign14330_e18683_d_n8;
        locals.var_arg_dn10 = assign14330_e18683_d_n10;
        locals.var_arg_dn11 = assign14330_e18683_d_n11;
        locals.var_arg_dn12 = assign14330_e18683_d_n12;
        locals.var_arg_rv = 0.0;

        let (assign14340_e18689, assign14340_e18689_d_n0, assign14340_e18689_d_n2, assign14340_e18689_d_n4, assign14340_e18689_d_n5, assign14340_e18689_d_n6, assign14340_e18689_d_n8, assign14340_e18689_d_n10, assign14340_e18689_d_n11, assign14340_e18689_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn8, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign14340_e18689;
        locals.var_dnm_dn0 = assign14340_e18689_d_n0;
        locals.var_dnm_dn2 = assign14340_e18689_d_n2;
        locals.var_dnm_dn4 = assign14340_e18689_d_n4;
        locals.var_dnm_dn5 = assign14340_e18689_d_n5;
        locals.var_dnm_dn6 = assign14340_e18689_d_n6;
        locals.var_dnm_dn8 = assign14340_e18689_d_n8;
        locals.var_dnm_dn10 = assign14340_e18689_d_n10;
        locals.var_dnm_dn11 = assign14340_e18689_d_n11;
        locals.var_dnm_dn12 = assign14340_e18689_d_n12;
        locals.var_dnm_rv = 0.0;

        let assign14350_e18704: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard259 = assign14350_e18704;
        locals.var_guard259_rv = 0.0;

        let assign14360_e18707: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign14360_e18707;
        locals.var_guard260_rv = 0.0;

        let (assign14370_e18717,) = {
    if ((((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14370_e18717;
        locals.var_mm_rv = 0.0;

        let assign14380_e18720: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign14380_e18720;
        locals.var_guard261_rv = 0.0;

        let (assign14390_e18733,) = {
    if (((((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 == 0.0)) && (locals.var_guard261 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14390_e18733;
        locals.var_mm_rv = 0.0;

        let assign14400_e18736: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign14400_e18736;
        locals.var_guard262_rv = 0.0;

        let (assign14410_e18752,) = {
    if ((((((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 == 0.0)) && (locals.var_guard261 == 0.0)) && (locals.var_guard262 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14410_e18752;
        locals.var_mm_rv = 0.0;

        let assign14420_e18755: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign14420_e18755;
        locals.var_guard263_rv = 0.0;

        let (assign14430_e18774,) = {
    if (((((((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 == 0.0)) && (locals.var_guard261 == 0.0)) && (locals.var_guard262 == 0.0)) && (locals.var_guard263 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14430_e18774;
        locals.var_mm_rv = 0.0;

        let (assign14440_e18782,) = {
    if (((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) && (locals.var_guard259 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign14440_e18782;
        locals.var_m0_rv = 0.0;

        let mut assign14450_loop_guard: usize = 0;
        while {
            let assign14450_cond_e18791: f64 = if ((((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) && (locals.var_guard259 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign14450_cond_e18791 != 0.0
        } {
            assign14450_loop_guard += 1;
            assert!(assign14450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign14450_body0_e18800, assign14450_body0_e18800_d_n0, assign14450_body0_e18800_d_n2, assign14450_body0_e18800_d_n4, assign14450_body0_e18800_d_n5, assign14450_body0_e18800_d_n6, assign14450_body0_e18800_d_n8, assign14450_body0_e18800_d_n10, assign14450_body0_e18800_d_n11, assign14450_body0_e18800_d_n12,) = {
    if (((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) && (locals.var_guard259 != 0.0)) {
        let assign14450_body0_e18798: f64 = (locals.var_dnm).sqrt();
        (assign14450_body0_e18798, (locals.var_dnm_dn0 / (2.0 * assign14450_body0_e18798)), (locals.var_dnm_dn2 / (2.0 * assign14450_body0_e18798)), (locals.var_dnm_dn4 / (2.0 * assign14450_body0_e18798)), (locals.var_dnm_dn5 / (2.0 * assign14450_body0_e18798)), (locals.var_dnm_dn6 / (2.0 * assign14450_body0_e18798)), (locals.var_dnm_dn8 / (2.0 * assign14450_body0_e18798)), (locals.var_dnm_dn10 / (2.0 * assign14450_body0_e18798)), (locals.var_dnm_dn11 / (2.0 * assign14450_body0_e18798)), (locals.var_dnm_dn12 / (2.0 * assign14450_body0_e18798)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
            locals.var_dnm = assign14450_body0_e18800;
            locals.var_dnm_dn0 = assign14450_body0_e18800_d_n0;
            locals.var_dnm_dn2 = assign14450_body0_e18800_d_n2;
            locals.var_dnm_dn4 = assign14450_body0_e18800_d_n4;
            locals.var_dnm_dn5 = assign14450_body0_e18800_d_n5;
            locals.var_dnm_dn6 = assign14450_body0_e18800_d_n6;
            locals.var_dnm_dn8 = assign14450_body0_e18800_d_n8;
            locals.var_dnm_dn10 = assign14450_body0_e18800_d_n10;
            locals.var_dnm_dn11 = assign14450_body0_e18800_d_n11;
            locals.var_dnm_dn12 = assign14450_body0_e18800_d_n12;
            locals.var_dnm_rv = 0.0;
            let (assign14450_body1_e18810,) = {
    if (((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) && (locals.var_guard259 != 0.0)) {
        let assign14450_body1_e18808: f64 = (locals.var_m0 + 1.0);
        (assign14450_body1_e18808,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign14450_body1_e18810;
            locals.var_m0_rv = 0.0;
        }

        let (assign14460_e18825, assign14460_e18825_d_n0, assign14460_e18825_d_n2, assign14460_e18825_d_n4, assign14460_e18825_d_n5, assign14460_e18825_d_n6, assign14460_e18825_d_n8, assign14460_e18825_d_n10, assign14460_e18825_d_n11, assign14460_e18825_d_n12,) = {
    if (((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) && (locals.var_guard259 == 0.0)) {
        let assign14460_e18821: f64 = (2.0 * 2.0);
        let assign14460_e18822: f64 = (1.0 / assign14460_e18821);
        let assign14460_e18823: f64 = (locals.var_dnm).powf(assign14460_e18822);
        (assign14460_e18823, if 0.0 == 0.0 && ((assign14460_e18822) as f64).is_finite() && ((assign14460_e18822) as f64).fract() == 0.0 { if assign14460_e18822 == 0.0 { 0.0 } else { (assign14460_e18822 * ((locals.var_dnm).powf(assign14460_e18822 - 1.0) * locals.var_dnm_dn0)) } } else { (assign14460_e18823 * (assign14460_e18822 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14460_e18822) as f64).is_finite() && ((assign14460_e18822) as f64).fract() == 0.0 { if assign14460_e18822 == 0.0 { 0.0 } else { (assign14460_e18822 * ((locals.var_dnm).powf(assign14460_e18822 - 1.0) * locals.var_dnm_dn2)) } } else { (assign14460_e18823 * (assign14460_e18822 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14460_e18822) as f64).is_finite() && ((assign14460_e18822) as f64).fract() == 0.0 { if assign14460_e18822 == 0.0 { 0.0 } else { (assign14460_e18822 * ((locals.var_dnm).powf(assign14460_e18822 - 1.0) * locals.var_dnm_dn4)) } } else { (assign14460_e18823 * (assign14460_e18822 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14460_e18822) as f64).is_finite() && ((assign14460_e18822) as f64).fract() == 0.0 { if assign14460_e18822 == 0.0 { 0.0 } else { (assign14460_e18822 * ((locals.var_dnm).powf(assign14460_e18822 - 1.0) * locals.var_dnm_dn5)) } } else { (assign14460_e18823 * (assign14460_e18822 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14460_e18822) as f64).is_finite() && ((assign14460_e18822) as f64).fract() == 0.0 { if assign14460_e18822 == 0.0 { 0.0 } else { (assign14460_e18822 * ((locals.var_dnm).powf(assign14460_e18822 - 1.0) * locals.var_dnm_dn6)) } } else { (assign14460_e18823 * (assign14460_e18822 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14460_e18822) as f64).is_finite() && ((assign14460_e18822) as f64).fract() == 0.0 { if assign14460_e18822 == 0.0 { 0.0 } else { (assign14460_e18822 * ((locals.var_dnm).powf(assign14460_e18822 - 1.0) * locals.var_dnm_dn8)) } } else { (assign14460_e18823 * (assign14460_e18822 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14460_e18822) as f64).is_finite() && ((assign14460_e18822) as f64).fract() == 0.0 { if assign14460_e18822 == 0.0 { 0.0 } else { (assign14460_e18822 * ((locals.var_dnm).powf(assign14460_e18822 - 1.0) * locals.var_dnm_dn10)) } } else { (assign14460_e18823 * (assign14460_e18822 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14460_e18822) as f64).is_finite() && ((assign14460_e18822) as f64).fract() == 0.0 { if assign14460_e18822 == 0.0 { 0.0 } else { (assign14460_e18822 * ((locals.var_dnm).powf(assign14460_e18822 - 1.0) * locals.var_dnm_dn11)) } } else { (assign14460_e18823 * (assign14460_e18822 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14460_e18822) as f64).is_finite() && ((assign14460_e18822) as f64).fract() == 0.0 { if assign14460_e18822 == 0.0 { 0.0 } else { (assign14460_e18822 * ((locals.var_dnm).powf(assign14460_e18822 - 1.0) * locals.var_dnm_dn12)) } } else { (assign14460_e18823 * (assign14460_e18822 * (locals.var_dnm_dn12 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign14460_e18825;
        locals.var_dnm_dn0 = assign14460_e18825_d_n0;
        locals.var_dnm_dn2 = assign14460_e18825_d_n2;
        locals.var_dnm_dn4 = assign14460_e18825_d_n4;
        locals.var_dnm_dn5 = assign14460_e18825_d_n5;
        locals.var_dnm_dn6 = assign14460_e18825_d_n6;
        locals.var_dnm_dn8 = assign14460_e18825_d_n8;
        locals.var_dnm_dn10 = assign14460_e18825_d_n10;
        locals.var_dnm_dn11 = assign14460_e18825_d_n11;
        locals.var_dnm_dn12 = assign14460_e18825_d_n12;
        locals.var_dnm_rv = 0.0;

        let (assign14470_e18835, assign14470_e18835_d_n0, assign14470_e18835_d_n2, assign14470_e18835_d_n4, assign14470_e18835_d_n5, assign14470_e18835_d_n6, assign14470_e18835_d_n8, assign14470_e18835_d_n10, assign14470_e18835_d_n11, assign14470_e18835_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14470_e18832: f64 = (locals.var_dnm + 1e-50);
        let assign14470_e18833: f64 = (1.0 / assign14470_e18832);
        (assign14470_e18833, (-(locals.var_dnm_dn0 / (assign14470_e18832 * assign14470_e18832))), (-(locals.var_dnm_dn2 / (assign14470_e18832 * assign14470_e18832))), (-(locals.var_dnm_dn4 / (assign14470_e18832 * assign14470_e18832))), (-(locals.var_dnm_dn5 / (assign14470_e18832 * assign14470_e18832))), (-(locals.var_dnm_dn6 / (assign14470_e18832 * assign14470_e18832))), (-(locals.var_dnm_dn8 / (assign14470_e18832 * assign14470_e18832))), (-(locals.var_dnm_dn10 / (assign14470_e18832 * assign14470_e18832))), (-(locals.var_dnm_dn11 / (assign14470_e18832 * assign14470_e18832))), (-(locals.var_dnm_dn12 / (assign14470_e18832 * assign14470_e18832))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign14470_e18835;
        locals.var_dnm_dn0 = assign14470_e18835_d_n0;
        locals.var_dnm_dn2 = assign14470_e18835_d_n2;
        locals.var_dnm_dn4 = assign14470_e18835_d_n4;
        locals.var_dnm_dn5 = assign14470_e18835_d_n5;
        locals.var_dnm_dn6 = assign14470_e18835_d_n6;
        locals.var_dnm_dn8 = assign14470_e18835_d_n8;
        locals.var_dnm_dn10 = assign14470_e18835_d_n10;
        locals.var_dnm_dn11 = assign14470_e18835_d_n11;
        locals.var_dnm_dn12 = assign14470_e18835_d_n12;
        locals.var_dnm_rv = 0.0;

        let (assign14480_e18847, assign14480_e18847_d_n0, assign14480_e18847_d_n2, assign14480_e18847_d_n4, assign14480_e18847_d_n5, assign14480_e18847_d_n6, assign14480_e18847_d_n8, assign14480_e18847_d_n10, assign14480_e18847_d_n11, assign14480_e18847_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14480_e18842: f64 = (locals.var_idspt * 0.05);
        let assign14480_e18843: f64 = (locals.var_tmf1 * assign14480_e18842);
        let assign14480_e18845: f64 = (assign14480_e18843 * locals.var_dnm);
        (assign14480_e18845, ((((locals.var_tmf1_dn0 * assign14480_e18842) + (locals.var_tmf1 * (locals.var_idspt_dn0 * 0.05))) * locals.var_dnm) + (assign14480_e18843 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign14480_e18842) + (locals.var_tmf1 * (locals.var_idspt_dn2 * 0.05))) * locals.var_dnm) + (assign14480_e18843 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign14480_e18842) + (locals.var_tmf1 * (locals.var_idspt_dn4 * 0.05))) * locals.var_dnm) + (assign14480_e18843 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign14480_e18842) + (locals.var_tmf1 * (locals.var_idspt_dn5 * 0.05))) * locals.var_dnm) + (assign14480_e18843 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign14480_e18842) + (locals.var_tmf1 * (locals.var_idspt_dn6 * 0.05))) * locals.var_dnm) + (assign14480_e18843 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn8 * assign14480_e18842) + (locals.var_tmf1 * (locals.var_idspt_dn8 * 0.05))) * locals.var_dnm) + (assign14480_e18843 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn10 * assign14480_e18842) + (locals.var_tmf1 * (locals.var_idspt_dn10 * 0.05))) * locals.var_dnm) + (assign14480_e18843 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign14480_e18842) + (locals.var_tmf1 * (locals.var_idspt_dn11 * 0.05))) * locals.var_dnm) + (assign14480_e18843 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign14480_e18842) + (locals.var_tmf1 * (locals.var_idspt_dn12 * 0.05))) * locals.var_dnm) + (assign14480_e18843 * locals.var_dnm_dn12)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn8, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12,)
    }
};
        locals.var_tmf0 = assign14480_e18847;
        locals.var_tmf0_dn0 = assign14480_e18847_d_n0;
        locals.var_tmf0_dn2 = assign14480_e18847_d_n2;
        locals.var_tmf0_dn4 = assign14480_e18847_d_n4;
        locals.var_tmf0_dn5 = assign14480_e18847_d_n5;
        locals.var_tmf0_dn6 = assign14480_e18847_d_n6;
        locals.var_tmf0_dn8 = assign14480_e18847_d_n8;
        locals.var_tmf0_dn10 = assign14480_e18847_d_n10;
        locals.var_tmf0_dn11 = assign14480_e18847_d_n11;
        locals.var_tmf0_dn12 = assign14480_e18847_d_n12;
        locals.var_tmf0_rv = 0.0;

        let (assign14490_e18863, assign14490_e18863_d_n0, assign14490_e18863_d_n2, assign14490_e18863_d_n4, assign14490_e18863_d_n5, assign14490_e18863_d_n6, assign14490_e18863_d_n8, assign14490_e18863_d_n10, assign14490_e18863_d_n11, assign14490_e18863_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14490_e18853: f64 = (locals.var_idspt * 0.05);
        let assign14490_e18855: f64 = (assign14490_e18853 * locals.var_xmp);
        let assign14490_e18857: f64 = (assign14490_e18855 * locals.var_dnm);
        let assign14490_e18860: f64 = (locals.var_arg + 1e-50);
        let assign14490_e18861: f64 = (assign14490_e18857 / assign14490_e18860);
        (assign14490_e18861, ((((((((locals.var_idspt_dn0 * 0.05) * locals.var_xmp) + (assign14490_e18853 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign14490_e18855 * locals.var_dnm_dn0)) * assign14490_e18860) - (assign14490_e18857 * locals.var_arg_dn0)) / (assign14490_e18860 * assign14490_e18860)), ((((((((locals.var_idspt_dn2 * 0.05) * locals.var_xmp) + (assign14490_e18853 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign14490_e18855 * locals.var_dnm_dn2)) * assign14490_e18860) - (assign14490_e18857 * locals.var_arg_dn2)) / (assign14490_e18860 * assign14490_e18860)), ((((((((locals.var_idspt_dn4 * 0.05) * locals.var_xmp) + (assign14490_e18853 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign14490_e18855 * locals.var_dnm_dn4)) * assign14490_e18860) - (assign14490_e18857 * locals.var_arg_dn4)) / (assign14490_e18860 * assign14490_e18860)), ((((((((locals.var_idspt_dn5 * 0.05) * locals.var_xmp) + (assign14490_e18853 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign14490_e18855 * locals.var_dnm_dn5)) * assign14490_e18860) - (assign14490_e18857 * locals.var_arg_dn5)) / (assign14490_e18860 * assign14490_e18860)), ((((((((locals.var_idspt_dn6 * 0.05) * locals.var_xmp) + (assign14490_e18853 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign14490_e18855 * locals.var_dnm_dn6)) * assign14490_e18860) - (assign14490_e18857 * locals.var_arg_dn6)) / (assign14490_e18860 * assign14490_e18860)), ((((((((locals.var_idspt_dn8 * 0.05) * locals.var_xmp) + (assign14490_e18853 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign14490_e18855 * locals.var_dnm_dn8)) * assign14490_e18860) - (assign14490_e18857 * locals.var_arg_dn8)) / (assign14490_e18860 * assign14490_e18860)), ((((((((locals.var_idspt_dn10 * 0.05) * locals.var_xmp) + (assign14490_e18853 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign14490_e18855 * locals.var_dnm_dn10)) * assign14490_e18860) - (assign14490_e18857 * locals.var_arg_dn10)) / (assign14490_e18860 * assign14490_e18860)), ((((((((locals.var_idspt_dn11 * 0.05) * locals.var_xmp) + (assign14490_e18853 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign14490_e18855 * locals.var_dnm_dn11)) * assign14490_e18860) - (assign14490_e18857 * locals.var_arg_dn11)) / (assign14490_e18860 * assign14490_e18860)), ((((((((locals.var_idspt_dn12 * 0.05) * locals.var_xmp) + (assign14490_e18853 * locals.var_xmp_dn12)) * locals.var_dnm) + (assign14490_e18855 * locals.var_dnm_dn12)) * assign14490_e18860) - (assign14490_e18857 * locals.var_arg_dn12)) / (assign14490_e18860 * assign14490_e18860)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign14490_e18863;
        locals.var_t0_dn0 = assign14490_e18863_d_n0;
        locals.var_t0_dn2 = assign14490_e18863_d_n2;
        locals.var_t0_dn4 = assign14490_e18863_d_n4;
        locals.var_t0_dn5 = assign14490_e18863_d_n5;
        locals.var_t0_dn6 = assign14490_e18863_d_n6;
        locals.var_t0_dn8 = assign14490_e18863_d_n8;
        locals.var_t0_dn10 = assign14490_e18863_d_n10;
        locals.var_t0_dn11 = assign14490_e18863_d_n11;
        locals.var_t0_dn12 = assign14490_e18863_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign14500_e18875, assign14500_e18875_d_n0, assign14500_e18875_d_n2, assign14500_e18875_d_n4, assign14500_e18875_d_n5, assign14500_e18875_d_n6, assign14500_e18875_d_n8, assign14500_e18875_d_n10, assign14500_e18875_d_n11, assign14500_e18875_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign14500_e18870: f64 = (locals.var_idspt * 0.05);
        let assign14500_e18871: f64 = (locals.var_idspt - assign14500_e18870);
        let assign14500_e18873: f64 = (assign14500_e18871 + locals.var_tmf0);
        (assign14500_e18873, ((locals.var_idspt_dn0 - (locals.var_idspt_dn0 * 0.05)) + locals.var_tmf0_dn0), ((locals.var_idspt_dn2 - (locals.var_idspt_dn2 * 0.05)) + locals.var_tmf0_dn2), ((locals.var_idspt_dn4 - (locals.var_idspt_dn4 * 0.05)) + locals.var_tmf0_dn4), ((locals.var_idspt_dn5 - (locals.var_idspt_dn5 * 0.05)) + locals.var_tmf0_dn5), ((locals.var_idspt_dn6 - (locals.var_idspt_dn6 * 0.05)) + locals.var_tmf0_dn6), ((locals.var_idspt_dn8 - (locals.var_idspt_dn8 * 0.05)) + locals.var_tmf0_dn8), ((locals.var_idspt_dn10 - (locals.var_idspt_dn10 * 0.05)) + locals.var_tmf0_dn10), ((locals.var_idspt_dn11 - (locals.var_idspt_dn11 * 0.05)) + locals.var_tmf0_dn11), ((locals.var_idspt_dn12 - (locals.var_idspt_dn12 * 0.05)) + locals.var_tmf0_dn12),)
    } else {
        (locals.var_idsptb, locals.var_idsptb_dn0, locals.var_idsptb_dn2, locals.var_idsptb_dn4, locals.var_idsptb_dn5, locals.var_idsptb_dn6, locals.var_idsptb_dn8, locals.var_idsptb_dn10, locals.var_idsptb_dn11, locals.var_idsptb_dn12,)
    }
};
        locals.var_idsptb = assign14500_e18875;
        locals.var_idsptb_dn0 = assign14500_e18875_d_n0;
        locals.var_idsptb_dn2 = assign14500_e18875_d_n2;
        locals.var_idsptb_dn4 = assign14500_e18875_d_n4;
        locals.var_idsptb_dn5 = assign14500_e18875_d_n5;
        locals.var_idsptb_dn6 = assign14500_e18875_d_n6;
        locals.var_idsptb_dn8 = assign14500_e18875_d_n8;
        locals.var_idsptb_dn10 = assign14500_e18875_d_n10;
        locals.var_idsptb_dn11 = assign14500_e18875_d_n11;
        locals.var_idsptb_dn12 = assign14500_e18875_d_n12;
        locals.var_idsptb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14510_e18881, assign14510_e18881_d_n0, assign14510_e18881_d_n2, assign14510_e18881_d_n4, assign14510_e18881_d_n5, assign14510_e18881_d_n6, assign14510_e18881_d_n8, assign14510_e18881_d_n10, assign14510_e18881_d_n11, assign14510_e18881_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign14510_e18881;
        locals.var_t0_dn0 = assign14510_e18881_d_n0;
        locals.var_t0_dn2 = assign14510_e18881_d_n2;
        locals.var_t0_dn4 = assign14510_e18881_d_n4;
        locals.var_t0_dn5 = assign14510_e18881_d_n5;
        locals.var_t0_dn6 = assign14510_e18881_d_n6;
        locals.var_t0_dn8 = assign14510_e18881_d_n8;
        locals.var_t0_dn10 = assign14510_e18881_d_n10;
        locals.var_t0_dn11 = assign14510_e18881_d_n11;
        locals.var_t0_dn12 = assign14510_e18881_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign14520_e18888, assign14520_e18888_d_n0, assign14520_e18888_d_n2, assign14520_e18888_d_n4, assign14520_e18888_d_n5, assign14520_e18888_d_n6, assign14520_e18888_d_n8, assign14520_e18888_d_n10, assign14520_e18888_d_n11, assign14520_e18888_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 == 0.0)) {
        (locals.var_idsptb, locals.var_idsptb_dn0, locals.var_idsptb_dn2, locals.var_idsptb_dn4, locals.var_idsptb_dn5, locals.var_idsptb_dn6, locals.var_idsptb_dn8, locals.var_idsptb_dn10, locals.var_idsptb_dn11, locals.var_idsptb_dn12,)
    } else {
        (locals.var_idsptb, locals.var_idsptb_dn0, locals.var_idsptb_dn2, locals.var_idsptb_dn4, locals.var_idsptb_dn5, locals.var_idsptb_dn6, locals.var_idsptb_dn8, locals.var_idsptb_dn10, locals.var_idsptb_dn11, locals.var_idsptb_dn12,)
    }
};
        locals.var_idsptb = assign14520_e18888;
        locals.var_idsptb_dn0 = assign14520_e18888_d_n0;
        locals.var_idsptb_dn2 = assign14520_e18888_d_n2;
        locals.var_idsptb_dn4 = assign14520_e18888_d_n4;
        locals.var_idsptb_dn5 = assign14520_e18888_d_n5;
        locals.var_idsptb_dn6 = assign14520_e18888_d_n6;
        locals.var_idsptb_dn8 = assign14520_e18888_d_n8;
        locals.var_idsptb_dn10 = assign14520_e18888_d_n10;
        locals.var_idsptb_dn11 = assign14520_e18888_d_n11;
        locals.var_idsptb_dn12 = assign14520_e18888_d_n12;
        locals.var_idsptb_rv = 0.0;

        let (assign14530_e18895, assign14530_e18895_d_n0, assign14530_e18895_d_n2, assign14530_e18895_d_n4, assign14530_e18895_d_n5, assign14530_e18895_d_n6, assign14530_e18895_d_n8, assign14530_e18895_d_n10, assign14530_e18895_d_n11, assign14530_e18895_d_n12,) = {
    if ((locals.var_guard257 != 0.0) && (locals.var_guard258 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign14530_e18895;
        locals.var_t0_dn0 = assign14530_e18895_d_n0;
        locals.var_t0_dn2 = assign14530_e18895_d_n2;
        locals.var_t0_dn4 = assign14530_e18895_d_n4;
        locals.var_t0_dn5 = assign14530_e18895_d_n5;
        locals.var_t0_dn6 = assign14530_e18895_d_n6;
        locals.var_t0_dn8 = assign14530_e18895_d_n8;
        locals.var_t0_dn10 = assign14530_e18895_d_n10;
        locals.var_t0_dn11 = assign14530_e18895_d_n11;
        locals.var_t0_dn12 = assign14530_e18895_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign14540_e18901, assign14540_e18901_d_n0, assign14540_e18901_d_n2, assign14540_e18901_d_n4, assign14540_e18901_d_n5, assign14540_e18901_d_n6, assign14540_e18901_d_n8, assign14540_e18901_d_n10, assign14540_e18901_d_n11, assign14540_e18901_d_n12,) = {
    if (locals.var_guard257 != 0.0) {
        let assign14540_e18899: f64 = (locals.var_ids2_fac * locals.var_idsptb);
        (assign14540_e18899, ((locals.var_ids2_fac_dn0 * locals.var_idsptb) + (locals.var_ids2_fac * locals.var_idsptb_dn0)), ((locals.var_ids2_fac_dn2 * locals.var_idsptb) + (locals.var_ids2_fac * locals.var_idsptb_dn2)), ((locals.var_ids2_fac_dn4 * locals.var_idsptb) + (locals.var_ids2_fac * locals.var_idsptb_dn4)), ((locals.var_ids2_fac_dn5 * locals.var_idsptb) + (locals.var_ids2_fac * locals.var_idsptb_dn5)), ((locals.var_ids2_fac_dn6 * locals.var_idsptb) + (locals.var_ids2_fac * locals.var_idsptb_dn6)), ((locals.var_ids2_fac_dn8 * locals.var_idsptb) + (locals.var_ids2_fac * locals.var_idsptb_dn8)), ((locals.var_ids2_fac_dn10 * locals.var_idsptb) + (locals.var_ids2_fac * locals.var_idsptb_dn10)), ((locals.var_ids2_fac_dn11 * locals.var_idsptb) + (locals.var_ids2_fac * locals.var_idsptb_dn11)), ((locals.var_ids2_fac_dn12 * locals.var_idsptb) + (locals.var_ids2_fac * locals.var_idsptb_dn12)),)
    } else {
        (locals.var_ids2, locals.var_ids2_dn0, locals.var_ids2_dn2, locals.var_ids2_dn4, locals.var_ids2_dn5, locals.var_ids2_dn6, locals.var_ids2_dn8, locals.var_ids2_dn10, locals.var_ids2_dn11, locals.var_ids2_dn12,)
    }
};
        locals.var_ids2 = assign14540_e18901;
        locals.var_ids2_dn0 = assign14540_e18901_d_n0;
        locals.var_ids2_dn2 = assign14540_e18901_d_n2;
        locals.var_ids2_dn4 = assign14540_e18901_d_n4;
        locals.var_ids2_dn5 = assign14540_e18901_d_n5;
        locals.var_ids2_dn6 = assign14540_e18901_d_n6;
        locals.var_ids2_dn8 = assign14540_e18901_d_n8;
        locals.var_ids2_dn10 = assign14540_e18901_d_n10;
        locals.var_ids2_dn11 = assign14540_e18901_d_n11;
        locals.var_ids2_dn12 = assign14540_e18901_d_n12;
        locals.var_ids2_rv = 0.0;

        let assign14550_e18904: f64 = (locals.var_ids1 + locals.var_ids2);
        locals.var_idspttot = assign14550_e18904;
        locals.var_idspttot_dn0 = (locals.var_ids1_dn0 + locals.var_ids2_dn0);
        locals.var_idspttot_dn2 = (locals.var_ids1_dn2 + locals.var_ids2_dn2);
        locals.var_idspttot_dn4 = (locals.var_ids1_dn4 + locals.var_ids2_dn4);
        locals.var_idspttot_dn5 = (locals.var_ids1_dn5 + locals.var_ids2_dn5);
        locals.var_idspttot_dn6 = (locals.var_ids1_dn6 + locals.var_ids2_dn6);
        locals.var_idspttot_dn8 = (locals.var_ids1_dn8 + locals.var_ids2_dn8);
        locals.var_idspttot_dn10 = (locals.var_ids1_dn10 + locals.var_ids2_dn10);
        locals.var_idspttot_dn11 = (locals.var_ids1_dn11 + locals.var_ids2_dn11);
        locals.var_idspttot_dn12 = (locals.var_ids1_dn12 + locals.var_ids2_dn12);
        locals.var_idspttot_rv = 0.0;

        let assign14560_e18907: f64 = (locals.var_ids0 + locals.var_idspttot);
        locals.var_ids = assign14560_e18907;
        locals.var_ids_dn0 = (locals.var_ids0_dn0 + locals.var_idspttot_dn0);
        locals.var_ids_dn2 = (locals.var_ids0_dn2 + locals.var_idspttot_dn2);
        locals.var_ids_dn4 = (locals.var_ids0_dn4 + locals.var_idspttot_dn4);
        locals.var_ids_dn5 = (locals.var_ids0_dn5 + locals.var_idspttot_dn5);
        locals.var_ids_dn6 = (locals.var_ids0_dn6 + locals.var_idspttot_dn6);
        locals.var_ids_dn8 = (locals.var_ids0_dn8 + locals.var_idspttot_dn8);
        locals.var_ids_dn10 = (locals.var_ids0_dn10 + locals.var_idspttot_dn10);
        locals.var_ids_dn11 = (locals.var_ids0_dn11 + locals.var_idspttot_dn11);
        locals.var_ids_dn12 = (locals.var_ids0_dn12 + locals.var_idspttot_dn12);
        locals.var_ids_rv = 0.0;

        let assign14570_e18910: f64 = if p.p22 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign14570_e18910;
        locals.var_guard264_rv = 0.0;

        let (assign14580_e18916, assign14580_e18916_d_n0, assign14580_e18916_d_n2, assign14580_e18916_d_n4, assign14580_e18916_d_n5, assign14580_e18916_d_n6, assign14580_e18916_d_n8, assign14580_e18916_d_n10, assign14580_e18916_d_n11, assign14580_e18916_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14580_e18914: f64 = (1.034943e-10 * locals.var_c_fox_inv);
        (assign14580_e18914, (1.034943e-10 * locals.var_c_fox_inv_dn0), (1.034943e-10 * locals.var_c_fox_inv_dn2), (1.034943e-10 * locals.var_c_fox_inv_dn4), (1.034943e-10 * locals.var_c_fox_inv_dn5), (1.034943e-10 * locals.var_c_fox_inv_dn6), (1.034943e-10 * locals.var_c_fox_inv_dn8), (1.034943e-10 * locals.var_c_fox_inv_dn10), (1.034943e-10 * locals.var_c_fox_inv_dn11), (1.034943e-10 * locals.var_c_fox_inv_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign14580_e18916;
        locals.var_t1_dn0 = assign14580_e18916_d_n0;
        locals.var_t1_dn2 = assign14580_e18916_d_n2;
        locals.var_t1_dn4 = assign14580_e18916_d_n4;
        locals.var_t1_dn5 = assign14580_e18916_d_n5;
        locals.var_t1_dn6 = assign14580_e18916_d_n6;
        locals.var_t1_dn8 = assign14580_e18916_d_n8;
        locals.var_t1_dn10 = assign14580_e18916_d_n10;
        locals.var_t1_dn11 = assign14580_e18916_d_n11;
        locals.var_t1_dn12 = assign14580_e18916_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign14590_e18920, assign14590_e18920_d_n0, assign14590_e18920_d_n2, assign14590_e18920_d_n4, assign14590_e18920_d_n5, assign14590_e18920_d_n6, assign14590_e18920_d_n8, assign14590_e18920_d_n10, assign14590_e18920_d_n11, assign14590_e18920_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn8, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn12,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign14590_e18920;
        locals.var_t2_dn0 = assign14590_e18920_d_n0;
        locals.var_t2_dn2 = assign14590_e18920_d_n2;
        locals.var_t2_dn4 = assign14590_e18920_d_n4;
        locals.var_t2_dn5 = assign14590_e18920_d_n5;
        locals.var_t2_dn6 = assign14590_e18920_d_n6;
        locals.var_t2_dn8 = assign14590_e18920_d_n8;
        locals.var_t2_dn10 = assign14590_e18920_d_n10;
        locals.var_t2_dn11 = assign14590_e18920_d_n11;
        locals.var_t2_dn12 = assign14590_e18920_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign14600_e18926, assign14600_e18926_d_n0, assign14600_e18926_d_n2, assign14600_e18926_d_n4, assign14600_e18926_d_n5, assign14600_e18926_d_n6, assign14600_e18926_d_n8, assign14600_e18926_d_n10, assign14600_e18926_d_n11, assign14600_e18926_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14600_e18924: f64 = (locals.var_lgatesm - p.p57);
        (assign14600_e18924, locals.var_lgatesm_dn0, locals.var_lgatesm_dn2, locals.var_lgatesm_dn4, locals.var_lgatesm_dn5, locals.var_lgatesm_dn6, locals.var_lgatesm_dn8, locals.var_lgatesm_dn10, locals.var_lgatesm_dn11, locals.var_lgatesm_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign14600_e18926;
        locals.var_t3_dn0 = assign14600_e18926_d_n0;
        locals.var_t3_dn2 = assign14600_e18926_d_n2;
        locals.var_t3_dn4 = assign14600_e18926_d_n4;
        locals.var_t3_dn5 = assign14600_e18926_d_n5;
        locals.var_t3_dn6 = assign14600_e18926_d_n6;
        locals.var_t3_dn8 = assign14600_e18926_d_n8;
        locals.var_t3_dn10 = assign14600_e18926_d_n10;
        locals.var_t3_dn11 = assign14600_e18926_d_n11;
        locals.var_t3_dn12 = assign14600_e18926_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign14610_e18934, assign14610_e18934_d_n0, assign14610_e18934_d_n2, assign14610_e18934_d_n4, assign14610_e18934_d_n5, assign14610_e18934_d_n6, assign14610_e18934_d_n8, assign14610_e18934_d_n10, assign14610_e18934_d_n11, assign14610_e18934_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14610_e18931: f64 = (locals.var_t3 * locals.var_t3);
        let assign14610_e18932: f64 = (1.0 / assign14610_e18931);
        (assign14610_e18932, (-(((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (assign14610_e18931 * assign14610_e18931))), (-(((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (assign14610_e18931 * assign14610_e18931))), (-(((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (assign14610_e18931 * assign14610_e18931))), (-(((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (assign14610_e18931 * assign14610_e18931))), (-(((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (assign14610_e18931 * assign14610_e18931))), (-(((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (assign14610_e18931 * assign14610_e18931))), (-(((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (assign14610_e18931 * assign14610_e18931))), (-(((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (assign14610_e18931 * assign14610_e18931))), (-(((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12)) / (assign14610_e18931 * assign14610_e18931))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign14610_e18934;
        locals.var_t4_dn0 = assign14610_e18934_d_n0;
        locals.var_t4_dn2 = assign14610_e18934_d_n2;
        locals.var_t4_dn4 = assign14610_e18934_d_n4;
        locals.var_t4_dn5 = assign14610_e18934_d_n5;
        locals.var_t4_dn6 = assign14610_e18934_d_n6;
        locals.var_t4_dn8 = assign14610_e18934_d_n8;
        locals.var_t4_dn10 = assign14610_e18934_d_n10;
        locals.var_t4_dn11 = assign14610_e18934_d_n11;
        locals.var_t4_dn12 = assign14610_e18934_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign14620_e18948, assign14620_e18948_d_n0, assign14620_e18948_d_n2, assign14620_e18948_d_n4, assign14620_e18948_d_n5, assign14620_e18948_d_n6, assign14620_e18948_d_n8, assign14620_e18948_d_n10, assign14620_e18948_d_n11, assign14620_e18948_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14620_e18939: f64 = (p.p55 - locals.var_pb20b);
        let assign14620_e18940: f64 = (2.0 * assign14620_e18939);
        let assign14620_e18942: f64 = (assign14620_e18940 * locals.var_t1);
        let assign14620_e18944: f64 = (assign14620_e18942 * locals.var_t2);
        let assign14620_e18946: f64 = (assign14620_e18944 * locals.var_t4);
        (assign14620_e18946, (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign14620_e18940 * locals.var_t1_dn0)) * locals.var_t2) + (assign14620_e18942 * locals.var_t2_dn0)) * locals.var_t4) + (assign14620_e18944 * locals.var_t4_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign14620_e18940 * locals.var_t1_dn2)) * locals.var_t2) + (assign14620_e18942 * locals.var_t2_dn2)) * locals.var_t4) + (assign14620_e18944 * locals.var_t4_dn2)), (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign14620_e18940 * locals.var_t1_dn4)) * locals.var_t2) + (assign14620_e18942 * locals.var_t2_dn4)) * locals.var_t4) + (assign14620_e18944 * locals.var_t4_dn4)), (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign14620_e18940 * locals.var_t1_dn5)) * locals.var_t2) + (assign14620_e18942 * locals.var_t2_dn5)) * locals.var_t4) + (assign14620_e18944 * locals.var_t4_dn5)), (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign14620_e18940 * locals.var_t1_dn6)) * locals.var_t2) + (assign14620_e18942 * locals.var_t2_dn6)) * locals.var_t4) + (assign14620_e18944 * locals.var_t4_dn6)), (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign14620_e18940 * locals.var_t1_dn8)) * locals.var_t2) + (assign14620_e18942 * locals.var_t2_dn8)) * locals.var_t4) + (assign14620_e18944 * locals.var_t4_dn8)), (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign14620_e18940 * locals.var_t1_dn10)) * locals.var_t2) + (assign14620_e18942 * locals.var_t2_dn10)) * locals.var_t4) + (assign14620_e18944 * locals.var_t4_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * locals.var_t1) + (assign14620_e18940 * locals.var_t1_dn11)) * locals.var_t2) + (assign14620_e18942 * locals.var_t2_dn11)) * locals.var_t4) + (assign14620_e18944 * locals.var_t4_dn11)), (((((((2.0 * (-locals.var_pb20b_dn12)) * locals.var_t1) + (assign14620_e18940 * locals.var_t1_dn12)) * locals.var_t2) + (assign14620_e18942 * locals.var_t2_dn12)) * locals.var_t4) + (assign14620_e18944 * locals.var_t4_dn12)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign14620_e18948;
        locals.var_t5_dn0 = assign14620_e18948_d_n0;
        locals.var_t5_dn2 = assign14620_e18948_d_n2;
        locals.var_t5_dn4 = assign14620_e18948_d_n4;
        locals.var_t5_dn5 = assign14620_e18948_d_n5;
        locals.var_t5_dn6 = assign14620_e18948_d_n6;
        locals.var_t5_dn8 = assign14620_e18948_d_n8;
        locals.var_t5_dn10 = assign14620_e18948_d_n10;
        locals.var_t5_dn11 = assign14620_e18948_d_n11;
        locals.var_t5_dn12 = assign14620_e18948_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign14630_e18954, assign14630_e18954_d_n0, assign14630_e18954_d_n2, assign14630_e18954_d_n4, assign14630_e18954_d_n5, assign14630_e18954_d_n6, assign14630_e18954_d_n8, assign14630_e18954_d_n10, assign14630_e18954_d_n11, assign14630_e18954_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14630_e18952: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        (assign14630_e18952, ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4)), ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5)), ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8)), ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5_dn12 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn12)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn4, locals.var_dvth0_dn5, locals.var_dvth0_dn6, locals.var_dvth0_dn8, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn12,)
    }
};
        locals.var_dvth0 = assign14630_e18954;
        locals.var_dvth0_dn0 = assign14630_e18954_d_n0;
        locals.var_dvth0_dn2 = assign14630_e18954_d_n2;
        locals.var_dvth0_dn4 = assign14630_e18954_d_n4;
        locals.var_dvth0_dn5 = assign14630_e18954_d_n5;
        locals.var_dvth0_dn6 = assign14630_e18954_d_n6;
        locals.var_dvth0_dn8 = assign14630_e18954_d_n8;
        locals.var_dvth0_dn10 = assign14630_e18954_d_n10;
        locals.var_dvth0_dn11 = assign14630_e18954_d_n11;
        locals.var_dvth0_dn12 = assign14630_e18954_d_n12;
        locals.var_dvth0_rv = 0.0;

        let (assign14640_e18958, assign14640_e18958_d_n0, assign14640_e18958_d_n2, assign14640_e18958_d_n4, assign14640_e18958_d_n5, assign14640_e18958_d_n6, assign14640_e18958_d_n8, assign14640_e18958_d_n10, assign14640_e18958_d_n11, assign14640_e18958_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        (p.p158, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign14640_e18958;
        locals.var_t4_dn0 = assign14640_e18958_d_n0;
        locals.var_t4_dn2 = assign14640_e18958_d_n2;
        locals.var_t4_dn4 = assign14640_e18958_d_n4;
        locals.var_t4_dn5 = assign14640_e18958_d_n5;
        locals.var_t4_dn6 = assign14640_e18958_d_n6;
        locals.var_t4_dn8 = assign14640_e18958_d_n8;
        locals.var_t4_dn10 = assign14640_e18958_d_n10;
        locals.var_t4_dn11 = assign14640_e18958_d_n11;
        locals.var_t4_dn12 = assign14640_e18958_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign14650_e18962, assign14650_e18962_d_n0, assign14650_e18962_d_n2, assign14650_e18962_d_n4, assign14650_e18962_d_n5, assign14650_e18962_d_n6, assign14650_e18962_d_n8, assign14650_e18962_d_n10, assign14650_e18962_d_n11, assign14650_e18962_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        (p.p159, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign14650_e18962;
        locals.var_t6_dn0 = assign14650_e18962_d_n0;
        locals.var_t6_dn2 = assign14650_e18962_d_n2;
        locals.var_t6_dn4 = assign14650_e18962_d_n4;
        locals.var_t6_dn5 = assign14650_e18962_d_n5;
        locals.var_t6_dn6 = assign14650_e18962_d_n6;
        locals.var_t6_dn8 = assign14650_e18962_d_n8;
        locals.var_t6_dn10 = assign14650_e18962_d_n10;
        locals.var_t6_dn11 = assign14650_e18962_d_n11;
        locals.var_t6_dn12 = assign14650_e18962_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign14660_e18970, assign14660_e18970_d_n0, assign14660_e18970_d_n2, assign14660_e18970_d_n4, assign14660_e18970_d_n5, assign14660_e18970_d_n6, assign14660_e18970_d_n8, assign14660_e18970_d_n10, assign14660_e18970_d_n11, assign14660_e18970_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14660_e18967: f64 = (locals.var_t6 * locals.var_vdsz);
        let assign14660_e18968: f64 = (locals.var_t4 + assign14660_e18967);
        (assign14660_e18968, (locals.var_t4_dn0 + ((locals.var_t6_dn0 * locals.var_vdsz) + (locals.var_t6 * locals.var_vdsz_dn0))), (locals.var_t4_dn2 + ((locals.var_t6_dn2 * locals.var_vdsz) + (locals.var_t6 * locals.var_vdsz_dn2))), (locals.var_t4_dn4 + ((locals.var_t6_dn4 * locals.var_vdsz) + (locals.var_t6 * locals.var_vdsz_dn4))), (locals.var_t4_dn5 + ((locals.var_t6_dn5 * locals.var_vdsz) + (locals.var_t6 * locals.var_vdsz_dn5))), (locals.var_t4_dn6 + ((locals.var_t6_dn6 * locals.var_vdsz) + (locals.var_t6 * locals.var_vdsz_dn6))), (locals.var_t4_dn8 + ((locals.var_t6_dn8 * locals.var_vdsz) + (locals.var_t6 * locals.var_vdsz_dn8))), (locals.var_t4_dn10 + ((locals.var_t6_dn10 * locals.var_vdsz) + (locals.var_t6 * locals.var_vdsz_dn10))), (locals.var_t4_dn11 + ((locals.var_t6_dn11 * locals.var_vdsz) + (locals.var_t6 * locals.var_vdsz_dn11))), (locals.var_t4_dn12 + ((locals.var_t6_dn12 * locals.var_vdsz) + (locals.var_t6 * locals.var_vdsz_dn12))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign14660_e18970;
        locals.var_t1_dn0 = assign14660_e18970_d_n0;
        locals.var_t1_dn2 = assign14660_e18970_d_n2;
        locals.var_t1_dn4 = assign14660_e18970_d_n4;
        locals.var_t1_dn5 = assign14660_e18970_d_n5;
        locals.var_t1_dn6 = assign14660_e18970_d_n6;
        locals.var_t1_dn8 = assign14660_e18970_d_n8;
        locals.var_t1_dn10 = assign14660_e18970_d_n10;
        locals.var_t1_dn11 = assign14660_e18970_d_n11;
        locals.var_t1_dn12 = assign14660_e18970_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign14670_e18976, assign14670_e18976_d_n0, assign14670_e18976_d_n2, assign14670_e18976_d_n4, assign14670_e18976_d_n5, assign14670_e18976_d_n6, assign14670_e18976_d_n8, assign14670_e18976_d_n10, assign14670_e18976_d_n11, assign14670_e18976_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14670_e18974: f64 = (locals.var_dvth0 * locals.var_t1);
        (assign14670_e18974, ((locals.var_dvth0_dn0 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn0)), ((locals.var_dvth0_dn2 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn2)), ((locals.var_dvth0_dn4 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn4)), ((locals.var_dvth0_dn5 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn5)), ((locals.var_dvth0_dn6 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn6)), ((locals.var_dvth0_dn8 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn8)), ((locals.var_dvth0_dn10 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn10)), ((locals.var_dvth0_dn11 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn11)), ((locals.var_dvth0_dn12 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn12)),)
    } else {
        (locals.var_dvthscsti, locals.var_dvthscsti_dn0, locals.var_dvthscsti_dn2, locals.var_dvthscsti_dn4, locals.var_dvthscsti_dn5, locals.var_dvthscsti_dn6, locals.var_dvthscsti_dn8, locals.var_dvthscsti_dn10, locals.var_dvthscsti_dn11, locals.var_dvthscsti_dn12,)
    }
};
        locals.var_dvthscsti = assign14670_e18976;
        locals.var_dvthscsti_dn0 = assign14670_e18976_d_n0;
        locals.var_dvthscsti_dn2 = assign14670_e18976_d_n2;
        locals.var_dvthscsti_dn4 = assign14670_e18976_d_n4;
        locals.var_dvthscsti_dn5 = assign14670_e18976_d_n5;
        locals.var_dvthscsti_dn6 = assign14670_e18976_d_n6;
        locals.var_dvthscsti_dn8 = assign14670_e18976_d_n8;
        locals.var_dvthscsti_dn10 = assign14670_e18976_d_n10;
        locals.var_dvthscsti_dn11 = assign14670_e18976_d_n11;
        locals.var_dvthscsti_dn12 = assign14670_e18976_d_n12;
        locals.var_dvthscsti_rv = 0.0;

        let (assign14680_e18984, assign14680_e18984_d_n0, assign14680_e18984_d_n2, assign14680_e18984_d_n4, assign14680_e18984_d_n5, assign14680_e18984_d_n6, assign14680_e18984_d_n8, assign14680_e18984_d_n10, assign14680_e18984_d_n11, assign14680_e18984_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14680_e18981: f64 = (p.p161 * locals.var_vds);
        let assign14680_e18982: f64 = (p.p160 - assign14680_e18981);
        (assign14680_e18982, (-(p.p161 * locals.var_vds_dn0)), (-(p.p161 * locals.var_vds_dn2)), (-(p.p161 * locals.var_vds_dn4)), (-(p.p161 * locals.var_vds_dn5)), (-(p.p161 * locals.var_vds_dn6)), (-(p.p161 * locals.var_vds_dn8)), (-(p.p161 * locals.var_vds_dn10)), (-(p.p161 * locals.var_vds_dn11)), (-(p.p161 * locals.var_vds_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign14680_e18984;
        locals.var_t1_dn0 = assign14680_e18984_d_n0;
        locals.var_t1_dn2 = assign14680_e18984_d_n2;
        locals.var_t1_dn4 = assign14680_e18984_d_n4;
        locals.var_t1_dn5 = assign14680_e18984_d_n5;
        locals.var_t1_dn6 = assign14680_e18984_d_n6;
        locals.var_t1_dn8 = assign14680_e18984_d_n8;
        locals.var_t1_dn10 = assign14680_e18984_d_n10;
        locals.var_t1_dn11 = assign14680_e18984_d_n11;
        locals.var_t1_dn12 = assign14680_e18984_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign14690_e18994, assign14690_e18994_d_n0, assign14690_e18994_d_n2, assign14690_e18994_d_n4, assign14690_e18994_d_n5, assign14690_e18994_d_n6, assign14690_e18994_d_n8, assign14690_e18994_d_n10, assign14690_e18994_d_n11, assign14690_e18994_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14690_e18988: f64 = (locals.var_vgsz - locals.var_vfb);
        let assign14690_e18990: f64 = (assign14690_e18988 + locals.var_t1);
        let assign14690_e18992: f64 = (assign14690_e18990 + locals.var_dvthscsti);
        (assign14690_e18992, (((locals.var_vgsz_dn0 - locals.var_vfb_dn0) + locals.var_t1_dn0) + locals.var_dvthscsti_dn0), (((locals.var_vgsz_dn2 - locals.var_vfb_dn2) + locals.var_t1_dn2) + locals.var_dvthscsti_dn2), (((locals.var_vgsz_dn4 - locals.var_vfb_dn4) + locals.var_t1_dn4) + locals.var_dvthscsti_dn4), (((locals.var_vgsz_dn5 - locals.var_vfb_dn5) + locals.var_t1_dn5) + locals.var_dvthscsti_dn5), (((locals.var_vgsz_dn6 - locals.var_vfb_dn6) + locals.var_t1_dn6) + locals.var_dvthscsti_dn6), (((locals.var_vgsz_dn8 - locals.var_vfb_dn8) + locals.var_t1_dn8) + locals.var_dvthscsti_dn8), (((locals.var_vgsz_dn10 - locals.var_vfb_dn10) + locals.var_t1_dn10) + locals.var_dvthscsti_dn10), (((locals.var_vgsz_dn11 - locals.var_vfb_dn11) + locals.var_t1_dn11) + locals.var_dvthscsti_dn11), (((locals.var_vgsz_dn12 - locals.var_vfb_dn12) + locals.var_t1_dn12) + locals.var_dvthscsti_dn12),)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn4, locals.var_vgssti_dn5, locals.var_vgssti_dn6, locals.var_vgssti_dn8, locals.var_vgssti_dn10, locals.var_vgssti_dn11, locals.var_vgssti_dn12,)
    }
};
        locals.var_vgssti = assign14690_e18994;
        locals.var_vgssti_dn0 = assign14690_e18994_d_n0;
        locals.var_vgssti_dn2 = assign14690_e18994_d_n2;
        locals.var_vgssti_dn4 = assign14690_e18994_d_n4;
        locals.var_vgssti_dn5 = assign14690_e18994_d_n5;
        locals.var_vgssti_dn6 = assign14690_e18994_d_n6;
        locals.var_vgssti_dn8 = assign14690_e18994_d_n8;
        locals.var_vgssti_dn10 = assign14690_e18994_d_n10;
        locals.var_vgssti_dn11 = assign14690_e18994_d_n11;
        locals.var_vgssti_dn12 = assign14690_e18994_d_n12;
        locals.var_vgssti_rv = 0.0;

        let (assign14700_e19002, assign14700_e19002_d_n0, assign14700_e19002_d_n2, assign14700_e19002_d_n4, assign14700_e19002_d_n5, assign14700_e19002_d_n6, assign14700_e19002_d_n8, assign14700_e19002_d_n10, assign14700_e19002_d_n11, assign14700_e19002_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14700_e18998: f64 = (locals.var_costi0_p2 * locals.var_c_fox_inv);
        let assign14700_e19000: f64 = (assign14700_e18998 * locals.var_c_fox_inv);
        (assign14700_e19000, ((((locals.var_costi0_p2_dn0 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn0)) * locals.var_c_fox_inv) + (assign14700_e18998 * locals.var_c_fox_inv_dn0)), ((((locals.var_costi0_p2_dn2 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn2)) * locals.var_c_fox_inv) + (assign14700_e18998 * locals.var_c_fox_inv_dn2)), ((((locals.var_costi0_p2_dn4 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn4)) * locals.var_c_fox_inv) + (assign14700_e18998 * locals.var_c_fox_inv_dn4)), ((((locals.var_costi0_p2_dn5 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn5)) * locals.var_c_fox_inv) + (assign14700_e18998 * locals.var_c_fox_inv_dn5)), ((((locals.var_costi0_p2_dn6 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn6)) * locals.var_c_fox_inv) + (assign14700_e18998 * locals.var_c_fox_inv_dn6)), ((((locals.var_costi0_p2_dn8 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn8)) * locals.var_c_fox_inv) + (assign14700_e18998 * locals.var_c_fox_inv_dn8)), ((((locals.var_costi0_p2_dn10 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn10)) * locals.var_c_fox_inv) + (assign14700_e18998 * locals.var_c_fox_inv_dn10)), ((((locals.var_costi0_p2_dn11 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn11)) * locals.var_c_fox_inv) + (assign14700_e18998 * locals.var_c_fox_inv_dn11)), ((((locals.var_costi0_p2_dn12 * locals.var_c_fox_inv) + (locals.var_costi0_p2 * locals.var_c_fox_inv_dn12)) * locals.var_c_fox_inv) + (assign14700_e18998 * locals.var_c_fox_inv_dn12)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn4, locals.var_costi3_dn5, locals.var_costi3_dn6, locals.var_costi3_dn8, locals.var_costi3_dn10, locals.var_costi3_dn11, locals.var_costi3_dn12,)
    }
};
        locals.var_costi3 = assign14700_e19002;
        locals.var_costi3_dn0 = assign14700_e19002_d_n0;
        locals.var_costi3_dn2 = assign14700_e19002_d_n2;
        locals.var_costi3_dn4 = assign14700_e19002_d_n4;
        locals.var_costi3_dn5 = assign14700_e19002_d_n5;
        locals.var_costi3_dn6 = assign14700_e19002_d_n6;
        locals.var_costi3_dn8 = assign14700_e19002_d_n8;
        locals.var_costi3_dn10 = assign14700_e19002_d_n10;
        locals.var_costi3_dn11 = assign14700_e19002_d_n11;
        locals.var_costi3_dn12 = assign14700_e19002_d_n12;
        locals.var_costi3_rv = 0.0;

        let (assign14710_e19010, assign14710_e19010_d_n0, assign14710_e19010_d_n2, assign14710_e19010_d_n4, assign14710_e19010_d_n5, assign14710_e19010_d_n6, assign14710_e19010_d_n8, assign14710_e19010_d_n10, assign14710_e19010_d_n11, assign14710_e19010_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14710_e19006: f64 = (locals.var_costi3 * locals.var_beta);
        let assign14710_e19008: f64 = (assign14710_e19006 * 0.5);
        (assign14710_e19008, ((locals.var_costi3_dn0 * locals.var_beta) * 0.5), ((locals.var_costi3_dn2 * locals.var_beta) * 0.5), (((locals.var_costi3_dn4 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn4)) * 0.5), ((locals.var_costi3_dn5 * locals.var_beta) * 0.5), ((locals.var_costi3_dn6 * locals.var_beta) * 0.5), ((locals.var_costi3_dn8 * locals.var_beta) * 0.5), ((locals.var_costi3_dn10 * locals.var_beta) * 0.5), ((locals.var_costi3_dn11 * locals.var_beta) * 0.5), ((locals.var_costi3_dn12 * locals.var_beta) * 0.5),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn4, locals.var_costi4_dn5, locals.var_costi4_dn6, locals.var_costi4_dn8, locals.var_costi4_dn10, locals.var_costi4_dn11, locals.var_costi4_dn12,)
    }
};
        locals.var_costi4 = assign14710_e19010;
        locals.var_costi4_dn0 = assign14710_e19010_d_n0;
        locals.var_costi4_dn2 = assign14710_e19010_d_n2;
        locals.var_costi4_dn4 = assign14710_e19010_d_n4;
        locals.var_costi4_dn5 = assign14710_e19010_d_n5;
        locals.var_costi4_dn6 = assign14710_e19010_d_n6;
        locals.var_costi4_dn8 = assign14710_e19010_d_n8;
        locals.var_costi4_dn10 = assign14710_e19010_d_n10;
        locals.var_costi4_dn11 = assign14710_e19010_d_n11;
        locals.var_costi4_dn12 = assign14710_e19010_d_n12;
        locals.var_costi4_rv = 0.0;

        let (assign14720_e19018, assign14720_e19018_d_n0, assign14720_e19018_d_n2, assign14720_e19018_d_n4, assign14720_e19018_d_n5, assign14720_e19018_d_n6, assign14720_e19018_d_n8, assign14720_e19018_d_n10, assign14720_e19018_d_n11, assign14720_e19018_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14720_e19014: f64 = (locals.var_costi4 * locals.var_beta);
        let assign14720_e19016: f64 = (assign14720_e19014 * 2.0);
        (assign14720_e19016, ((locals.var_costi4_dn0 * locals.var_beta) * 2.0), ((locals.var_costi4_dn2 * locals.var_beta) * 2.0), (((locals.var_costi4_dn4 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn4)) * 2.0), ((locals.var_costi4_dn5 * locals.var_beta) * 2.0), ((locals.var_costi4_dn6 * locals.var_beta) * 2.0), ((locals.var_costi4_dn8 * locals.var_beta) * 2.0), ((locals.var_costi4_dn10 * locals.var_beta) * 2.0), ((locals.var_costi4_dn11 * locals.var_beta) * 2.0), ((locals.var_costi4_dn12 * locals.var_beta) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn4, locals.var_costi5_dn5, locals.var_costi5_dn6, locals.var_costi5_dn8, locals.var_costi5_dn10, locals.var_costi5_dn11, locals.var_costi5_dn12,)
    }
};
        locals.var_costi5 = assign14720_e19018;
        locals.var_costi5_dn0 = assign14720_e19018_d_n0;
        locals.var_costi5_dn2 = assign14720_e19018_d_n2;
        locals.var_costi5_dn4 = assign14720_e19018_d_n4;
        locals.var_costi5_dn5 = assign14720_e19018_d_n5;
        locals.var_costi5_dn6 = assign14720_e19018_d_n6;
        locals.var_costi5_dn8 = assign14720_e19018_d_n8;
        locals.var_costi5_dn10 = assign14720_e19018_d_n10;
        locals.var_costi5_dn11 = assign14720_e19018_d_n11;
        locals.var_costi5_dn12 = assign14720_e19018_d_n12;
        locals.var_costi5_rv = 0.0;

        let (assign14730_e19024, assign14730_e19024_d_n0, assign14730_e19024_d_n2, assign14730_e19024_d_n4, assign14730_e19024_d_n5, assign14730_e19024_d_n6, assign14730_e19024_d_n8, assign14730_e19024_d_n10, assign14730_e19024_d_n11, assign14730_e19024_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14730_e19022: f64 = (locals.var_beta * 0.25);
        (assign14730_e19022, 0.0, 0.0, (locals.var_beta_dn4 * 0.25), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn8, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign14730_e19024;
        locals.var_t11_dn0 = assign14730_e19024_d_n0;
        locals.var_t11_dn2 = assign14730_e19024_d_n2;
        locals.var_t11_dn4 = assign14730_e19024_d_n4;
        locals.var_t11_dn5 = assign14730_e19024_d_n5;
        locals.var_t11_dn6 = assign14730_e19024_d_n6;
        locals.var_t11_dn8 = assign14730_e19024_d_n8;
        locals.var_t11_dn10 = assign14730_e19024_d_n10;
        locals.var_t11_dn11 = assign14730_e19024_d_n11;
        locals.var_t11_dn12 = assign14730_e19024_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign14740_e19040, assign14740_e19040_d_n0, assign14740_e19040_d_n2, assign14740_e19040_d_n4, assign14740_e19040_d_n5, assign14740_e19040_d_n6, assign14740_e19040_d_n8, assign14740_e19040_d_n10, assign14740_e19040_d_n11, assign14740_e19040_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14740_e19029: f64 = (locals.var_costi3 * locals.var_t11);
        let assign14740_e19030: f64 = (locals.var_beta_inv - assign14740_e19029);
        let assign14740_e19032: f64 = (assign14740_e19030 + locals.var_vfb);
        let assign14740_e19034: f64 = (assign14740_e19032 - p.p160);
        let assign14740_e19036: f64 = (assign14740_e19034 - locals.var_dvthscsti);
        let assign14740_e19038: f64 = (assign14740_e19036 + 1e-50);
        (assign14740_e19038, (((-((locals.var_costi3_dn0 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn0))) + locals.var_vfb_dn0) - locals.var_dvthscsti_dn0), (((-((locals.var_costi3_dn2 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn2))) + locals.var_vfb_dn2) - locals.var_dvthscsti_dn2), (((locals.var_beta_inv_dn4 - ((locals.var_costi3_dn4 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn4))) + locals.var_vfb_dn4) - locals.var_dvthscsti_dn4), (((-((locals.var_costi3_dn5 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn5))) + locals.var_vfb_dn5) - locals.var_dvthscsti_dn5), (((-((locals.var_costi3_dn6 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn6))) + locals.var_vfb_dn6) - locals.var_dvthscsti_dn6), (((-((locals.var_costi3_dn8 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn8))) + locals.var_vfb_dn8) - locals.var_dvthscsti_dn8), (((-((locals.var_costi3_dn10 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn10))) + locals.var_vfb_dn10) - locals.var_dvthscsti_dn10), (((-((locals.var_costi3_dn11 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn11))) + locals.var_vfb_dn11) - locals.var_dvthscsti_dn11), (((-((locals.var_costi3_dn12 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn12))) + locals.var_vfb_dn12) - locals.var_dvthscsti_dn12),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign14740_e19040;
        locals.var_t10_dn0 = assign14740_e19040_d_n0;
        locals.var_t10_dn2 = assign14740_e19040_d_n2;
        locals.var_t10_dn4 = assign14740_e19040_d_n4;
        locals.var_t10_dn5 = assign14740_e19040_d_n5;
        locals.var_t10_dn6 = assign14740_e19040_d_n6;
        locals.var_t10_dn8 = assign14740_e19040_d_n8;
        locals.var_t10_dn10 = assign14740_e19040_d_n10;
        locals.var_t10_dn11 = assign14740_e19040_d_n11;
        locals.var_t10_dn12 = assign14740_e19040_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign14750_e19048, assign14750_e19048_d_n0, assign14750_e19048_d_n2, assign14750_e19048_d_n4, assign14750_e19048_d_n5, assign14750_e19048_d_n6, assign14750_e19048_d_n8, assign14750_e19048_d_n10, assign14750_e19048_d_n11, assign14750_e19048_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14750_e19044: f64 = (locals.var_vgsz - locals.var_t10);
        let assign14750_e19046: f64 = (assign14750_e19044 - 0.005);
        (assign14750_e19046, (locals.var_vgsz_dn0 - locals.var_t10_dn0), (locals.var_vgsz_dn2 - locals.var_t10_dn2), (locals.var_vgsz_dn4 - locals.var_t10_dn4), (locals.var_vgsz_dn5 - locals.var_t10_dn5), (locals.var_vgsz_dn6 - locals.var_t10_dn6), (locals.var_vgsz_dn8 - locals.var_t10_dn8), (locals.var_vgsz_dn10 - locals.var_t10_dn10), (locals.var_vgsz_dn11 - locals.var_t10_dn11), (locals.var_vgsz_dn12 - locals.var_t10_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign14750_e19048;
        locals.var_t1_dn0 = assign14750_e19048_d_n0;
        locals.var_t1_dn2 = assign14750_e19048_d_n2;
        locals.var_t1_dn4 = assign14750_e19048_d_n4;
        locals.var_t1_dn5 = assign14750_e19048_d_n5;
        locals.var_t1_dn6 = assign14750_e19048_d_n6;
        locals.var_t1_dn8 = assign14750_e19048_d_n8;
        locals.var_t1_dn10 = assign14750_e19048_d_n10;
        locals.var_t1_dn11 = assign14750_e19048_d_n11;
        locals.var_t1_dn12 = assign14750_e19048_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign14760_e19058, assign14760_e19058_d_n0, assign14760_e19058_d_n2, assign14760_e19058_d_n4, assign14760_e19058_d_n5, assign14760_e19058_d_n6, assign14760_e19058_d_n8, assign14760_e19058_d_n10, assign14760_e19058_d_n11, assign14760_e19058_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let (assign14760_e19056,) = {
            if (locals.var_t10 >= 0.0) {
                (1.0,)
            } else {
                let assign14760_e19055: f64 = (-1.0);
                (assign14760_e19055,)
            }
        };
        (assign14760_e19056, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign14760_e19058;
        locals.var_t0_dn0 = assign14760_e19058_d_n0;
        locals.var_t0_dn2 = assign14760_e19058_d_n2;
        locals.var_t0_dn4 = assign14760_e19058_d_n4;
        locals.var_t0_dn5 = assign14760_e19058_d_n5;
        locals.var_t0_dn6 = assign14760_e19058_d_n6;
        locals.var_t0_dn8 = assign14760_e19058_d_n8;
        locals.var_t0_dn10 = assign14760_e19058_d_n10;
        locals.var_t0_dn11 = assign14760_e19058_d_n11;
        locals.var_t0_dn12 = assign14760_e19058_d_n12;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14770_e19073, assign14770_e19073_d_n0, assign14770_e19073_d_n2, assign14770_e19073_d_n4, assign14770_e19073_d_n5, assign14770_e19073_d_n6, assign14770_e19073_d_n8, assign14770_e19073_d_n10, assign14770_e19073_d_n11, assign14770_e19073_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14770_e19062: f64 = (locals.var_t1 * locals.var_t1);
        let assign14770_e19065: f64 = (locals.var_t0 * 4.0);
        let assign14770_e19067: f64 = (assign14770_e19065 * locals.var_t10);
        let assign14770_e19069: f64 = (assign14770_e19067 * 0.005);
        let assign14770_e19070: f64 = (assign14770_e19062 + assign14770_e19069);
        let assign14770_e19071: f64 = (assign14770_e19070).sqrt();
        (assign14770_e19071, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((locals.var_t0_dn0 * 4.0) * locals.var_t10) + (assign14770_e19065 * locals.var_t10_dn0)) * 0.005)) / (2.0 * assign14770_e19071)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((locals.var_t0_dn2 * 4.0) * locals.var_t10) + (assign14770_e19065 * locals.var_t10_dn2)) * 0.005)) / (2.0 * assign14770_e19071)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((locals.var_t0_dn4 * 4.0) * locals.var_t10) + (assign14770_e19065 * locals.var_t10_dn4)) * 0.005)) / (2.0 * assign14770_e19071)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((locals.var_t0_dn5 * 4.0) * locals.var_t10) + (assign14770_e19065 * locals.var_t10_dn5)) * 0.005)) / (2.0 * assign14770_e19071)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((locals.var_t0_dn6 * 4.0) * locals.var_t10) + (assign14770_e19065 * locals.var_t10_dn6)) * 0.005)) / (2.0 * assign14770_e19071)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((locals.var_t0_dn8 * 4.0) * locals.var_t10) + (assign14770_e19065 * locals.var_t10_dn8)) * 0.005)) / (2.0 * assign14770_e19071)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((locals.var_t0_dn10 * 4.0) * locals.var_t10) + (assign14770_e19065 * locals.var_t10_dn10)) * 0.005)) / (2.0 * assign14770_e19071)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + ((((locals.var_t0_dn11 * 4.0) * locals.var_t10) + (assign14770_e19065 * locals.var_t10_dn11)) * 0.005)) / (2.0 * assign14770_e19071)), ((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + ((((locals.var_t0_dn12 * 4.0) * locals.var_t10) + (assign14770_e19065 * locals.var_t10_dn12)) * 0.005)) / (2.0 * assign14770_e19071)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign14770_e19073;
        locals.var_t2_dn0 = assign14770_e19073_d_n0;
        locals.var_t2_dn2 = assign14770_e19073_d_n2;
        locals.var_t2_dn4 = assign14770_e19073_d_n4;
        locals.var_t2_dn5 = assign14770_e19073_d_n5;
        locals.var_t2_dn6 = assign14770_e19073_d_n6;
        locals.var_t2_dn8 = assign14770_e19073_d_n8;
        locals.var_t2_dn10 = assign14770_e19073_d_n10;
        locals.var_t2_dn11 = assign14770_e19073_d_n11;
        locals.var_t2_dn12 = assign14770_e19073_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign14780_e19091, assign14780_e19091_d_n0, assign14780_e19091_d_n2, assign14780_e19091_d_n4, assign14780_e19091_d_n5, assign14780_e19091_d_n6, assign14780_e19091_d_n8, assign14780_e19091_d_n10, assign14780_e19091_d_n11, assign14780_e19091_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14780_e19079: f64 = (locals.var_t1 + locals.var_t2);
        let assign14780_e19080: f64 = (0.5 * assign14780_e19079);
        let assign14780_e19081: f64 = (locals.var_t10 + assign14780_e19080);
        let assign14780_e19083: f64 = (assign14780_e19081 - locals.var_vfb);
        let assign14780_e19085: f64 = (assign14780_e19083 + p.p160);
        let assign14780_e19087: f64 = (assign14780_e19085 + locals.var_dvthscsti);
        let assign14780_e19089: f64 = (assign14780_e19087 - locals.var_vbsz);
        (assign14780_e19089, ((((locals.var_t10_dn0 + (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0))) - locals.var_vfb_dn0) + locals.var_dvthscsti_dn0) - locals.var_vbsz_dn0), ((((locals.var_t10_dn2 + (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2))) - locals.var_vfb_dn2) + locals.var_dvthscsti_dn2) - locals.var_vbsz_dn2), ((((locals.var_t10_dn4 + (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4))) - locals.var_vfb_dn4) + locals.var_dvthscsti_dn4) - locals.var_vbsz_dn4), ((((locals.var_t10_dn5 + (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5))) - locals.var_vfb_dn5) + locals.var_dvthscsti_dn5) - locals.var_vbsz_dn5), ((((locals.var_t10_dn6 + (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6))) - locals.var_vfb_dn6) + locals.var_dvthscsti_dn6) - locals.var_vbsz_dn6), ((((locals.var_t10_dn8 + (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8))) - locals.var_vfb_dn8) + locals.var_dvthscsti_dn8) - locals.var_vbsz_dn8), ((((locals.var_t10_dn10 + (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10))) - locals.var_vfb_dn10) + locals.var_dvthscsti_dn10) - locals.var_vbsz_dn10), ((((locals.var_t10_dn11 + (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11))) - locals.var_vfb_dn11) + locals.var_dvthscsti_dn11) - locals.var_vbsz_dn11), ((((locals.var_t10_dn12 + (0.5 * (locals.var_t1_dn12 + locals.var_t2_dn12))) - locals.var_vfb_dn12) + locals.var_dvthscsti_dn12) - locals.var_vbsz_dn12),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign14780_e19091;
        locals.var_t3_dn0 = assign14780_e19091_d_n0;
        locals.var_t3_dn2 = assign14780_e19091_d_n2;
        locals.var_t3_dn4 = assign14780_e19091_d_n4;
        locals.var_t3_dn5 = assign14780_e19091_d_n5;
        locals.var_t3_dn6 = assign14780_e19091_d_n6;
        locals.var_t3_dn8 = assign14780_e19091_d_n8;
        locals.var_t3_dn10 = assign14780_e19091_d_n10;
        locals.var_t3_dn11 = assign14780_e19091_d_n11;
        locals.var_t3_dn12 = assign14780_e19091_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign14790_e19099, assign14790_e19099_d_n0, assign14790_e19099_d_n2, assign14790_e19099_d_n4, assign14790_e19099_d_n5, assign14790_e19099_d_n6, assign14790_e19099_d_n8, assign14790_e19099_d_n10, assign14790_e19099_d_n11, assign14790_e19099_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14790_e19095: f64 = (locals.var_beta * locals.var_t3);
        let assign14790_e19097: f64 = (assign14790_e19095 - 1.0);
        (assign14790_e19097, (locals.var_beta * locals.var_t3_dn0), (locals.var_beta * locals.var_t3_dn2), ((locals.var_beta_dn4 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn4)), (locals.var_beta * locals.var_t3_dn5), (locals.var_beta * locals.var_t3_dn6), (locals.var_beta * locals.var_t3_dn8), (locals.var_beta * locals.var_t3_dn10), (locals.var_beta * locals.var_t3_dn11), (locals.var_beta * locals.var_t3_dn12),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign14790_e19099;
        locals.var_t4_dn0 = assign14790_e19099_d_n0;
        locals.var_t4_dn2 = assign14790_e19099_d_n2;
        locals.var_t4_dn4 = assign14790_e19099_d_n4;
        locals.var_t4_dn5 = assign14790_e19099_d_n5;
        locals.var_t4_dn6 = assign14790_e19099_d_n6;
        locals.var_t4_dn8 = assign14790_e19099_d_n8;
        locals.var_t4_dn10 = assign14790_e19099_d_n10;
        locals.var_t4_dn11 = assign14790_e19099_d_n11;
        locals.var_t4_dn12 = assign14790_e19099_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign14800_e19105, assign14800_e19105_d_n0, assign14800_e19105_d_n2, assign14800_e19105_d_n4, assign14800_e19105_d_n5, assign14800_e19105_d_n6, assign14800_e19105_d_n8, assign14800_e19105_d_n10, assign14800_e19105_d_n11, assign14800_e19105_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14800_e19103: f64 = (4.0 / locals.var_costi5);
        (assign14800_e19103, (-((4.0 * locals.var_costi5_dn0) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn2) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn4) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn5) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn6) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn8) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn10) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn11) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn12) / (locals.var_costi5 * locals.var_costi5))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign14800_e19105;
        locals.var_t5_dn0 = assign14800_e19105_d_n0;
        locals.var_t5_dn2 = assign14800_e19105_d_n2;
        locals.var_t5_dn4 = assign14800_e19105_d_n4;
        locals.var_t5_dn5 = assign14800_e19105_d_n5;
        locals.var_t5_dn6 = assign14800_e19105_d_n6;
        locals.var_t5_dn8 = assign14800_e19105_d_n8;
        locals.var_t5_dn10 = assign14800_e19105_d_n10;
        locals.var_t5_dn11 = assign14800_e19105_d_n11;
        locals.var_t5_dn12 = assign14800_e19105_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign14810_e19113, assign14810_e19113_d_n0, assign14810_e19113_d_n2, assign14810_e19113_d_n4, assign14810_e19113_d_n5, assign14810_e19113_d_n6, assign14810_e19113_d_n8, assign14810_e19113_d_n10, assign14810_e19113_d_n11, assign14810_e19113_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14810_e19110: f64 = (locals.var_t4 * locals.var_t5);
        let assign14810_e19111: f64 = (1.0 + assign14810_e19110);
        (assign14810_e19111, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn12 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign14810_e19113;
        locals.var_t1_dn0 = assign14810_e19113_d_n0;
        locals.var_t1_dn2 = assign14810_e19113_d_n2;
        locals.var_t1_dn4 = assign14810_e19113_d_n4;
        locals.var_t1_dn5 = assign14810_e19113_d_n5;
        locals.var_t1_dn6 = assign14810_e19113_d_n6;
        locals.var_t1_dn8 = assign14810_e19113_d_n8;
        locals.var_t1_dn10 = assign14810_e19113_d_n10;
        locals.var_t1_dn11 = assign14810_e19113_d_n11;
        locals.var_t1_dn12 = assign14810_e19113_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign14820_e19126, assign14820_e19126_d_n0, assign14820_e19126_d_n2, assign14820_e19126_d_n4, assign14820_e19126_d_n5, assign14820_e19126_d_n6, assign14820_e19126_d_n8, assign14820_e19126_d_n10, assign14820_e19126_d_n11, assign14820_e19126_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14820_e19117: f64 = (locals.var_t1 * locals.var_t1);
        let assign14820_e19120: f64 = (4.0 * 0.01);
        let assign14820_e19122: f64 = (assign14820_e19120 * 0.01);
        let assign14820_e19123: f64 = (assign14820_e19117 + assign14820_e19122);
        let assign14820_e19124: f64 = (assign14820_e19123).sqrt();
        (assign14820_e19124, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign14820_e19124)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign14820_e19124)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign14820_e19124)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign14820_e19124)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign14820_e19124)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign14820_e19124)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign14820_e19124)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign14820_e19124)), (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign14820_e19124)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign14820_e19126;
        locals.var_tmf2_dn0 = assign14820_e19126_d_n0;
        locals.var_tmf2_dn2 = assign14820_e19126_d_n2;
        locals.var_tmf2_dn4 = assign14820_e19126_d_n4;
        locals.var_tmf2_dn5 = assign14820_e19126_d_n5;
        locals.var_tmf2_dn6 = assign14820_e19126_d_n6;
        locals.var_tmf2_dn8 = assign14820_e19126_d_n8;
        locals.var_tmf2_dn10 = assign14820_e19126_d_n10;
        locals.var_tmf2_dn11 = assign14820_e19126_d_n11;
        locals.var_tmf2_dn12 = assign14820_e19126_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign14830_e19136, assign14830_e19136_d_n0, assign14830_e19136_d_n2, assign14830_e19136_d_n4, assign14830_e19136_d_n5, assign14830_e19136_d_n6, assign14830_e19136_d_n8, assign14830_e19136_d_n10, assign14830_e19136_d_n11, assign14830_e19136_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14830_e19132: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign14830_e19133: f64 = (1.0 + assign14830_e19132);
        let assign14830_e19134: f64 = (0.5 * assign14830_e19133);
        (assign14830_e19134, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn12 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign14830_e19136;
        locals.var_t2_dn0 = assign14830_e19136_d_n0;
        locals.var_t2_dn2 = assign14830_e19136_d_n2;
        locals.var_t2_dn4 = assign14830_e19136_d_n4;
        locals.var_t2_dn5 = assign14830_e19136_d_n5;
        locals.var_t2_dn6 = assign14830_e19136_d_n6;
        locals.var_t2_dn8 = assign14830_e19136_d_n8;
        locals.var_t2_dn10 = assign14830_e19136_d_n10;
        locals.var_t2_dn11 = assign14830_e19136_d_n11;
        locals.var_t2_dn12 = assign14830_e19136_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign14840_e19148, assign14840_e19148_d_n0, assign14840_e19148_d_n2, assign14840_e19148_d_n4, assign14840_e19148_d_n5, assign14840_e19148_d_n6, assign14840_e19148_d_n8, assign14840_e19148_d_n10, assign14840_e19148_d_n11, assign14840_e19148_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14840_e19141: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign14840_e19142: f64 = (0.5 * assign14840_e19141);
        let assign14840_e19145: f64 = (1e-10 * 0.01);
        let assign14840_e19146: f64 = (assign14840_e19142 + assign14840_e19145);
        (assign14840_e19146, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign14840_e19148;
        locals.var_t1_dn0 = assign14840_e19148_d_n0;
        locals.var_t1_dn2 = assign14840_e19148_d_n2;
        locals.var_t1_dn4 = assign14840_e19148_d_n4;
        locals.var_t1_dn5 = assign14840_e19148_d_n5;
        locals.var_t1_dn6 = assign14840_e19148_d_n6;
        locals.var_t1_dn8 = assign14840_e19148_d_n8;
        locals.var_t1_dn10 = assign14840_e19148_d_n10;
        locals.var_t1_dn11 = assign14840_e19148_d_n11;
        locals.var_t1_dn12 = assign14840_e19148_d_n12;
        locals.var_t1_rv = 0.0;

        let assign14850_e19151: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign14850_e19151;
        locals.var_guard265_rv = 0.0;

        let (assign14860_e19157, assign14860_e19157_d_n0, assign14860_e19157_d_n2, assign14860_e19157_d_n4, assign14860_e19157_d_n5, assign14860_e19157_d_n6, assign14860_e19157_d_n8, assign14860_e19157_d_n10, assign14860_e19157_d_n11, assign14860_e19157_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard265 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign14860_e19157;
        locals.var_t1_dn0 = assign14860_e19157_d_n0;
        locals.var_t1_dn2 = assign14860_e19157_d_n2;
        locals.var_t1_dn4 = assign14860_e19157_d_n4;
        locals.var_t1_dn5 = assign14860_e19157_d_n5;
        locals.var_t1_dn6 = assign14860_e19157_d_n6;
        locals.var_t1_dn8 = assign14860_e19157_d_n8;
        locals.var_t1_dn10 = assign14860_e19157_d_n10;
        locals.var_t1_dn11 = assign14860_e19157_d_n11;
        locals.var_t1_dn12 = assign14860_e19157_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign14870_e19163, assign14870_e19163_d_n0, assign14870_e19163_d_n2, assign14870_e19163_d_n4, assign14870_e19163_d_n5, assign14870_e19163_d_n6, assign14870_e19163_d_n8, assign14870_e19163_d_n10, assign14870_e19163_d_n11, assign14870_e19163_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard265 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign14870_e19163;
        locals.var_t2_dn0 = assign14870_e19163_d_n0;
        locals.var_t2_dn2 = assign14870_e19163_d_n2;
        locals.var_t2_dn4 = assign14870_e19163_d_n4;
        locals.var_t2_dn5 = assign14870_e19163_d_n5;
        locals.var_t2_dn6 = assign14870_e19163_d_n6;
        locals.var_t2_dn8 = assign14870_e19163_d_n8;
        locals.var_t2_dn10 = assign14870_e19163_d_n10;
        locals.var_t2_dn11 = assign14870_e19163_d_n11;
        locals.var_t2_dn12 = assign14870_e19163_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign14880_e19169, assign14880_e19169_d_n0, assign14880_e19169_d_n2, assign14880_e19169_d_n4, assign14880_e19169_d_n5, assign14880_e19169_d_n6, assign14880_e19169_d_n8, assign14880_e19169_d_n10, assign14880_e19169_d_n11, assign14880_e19169_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14880_e19167: f64 = (locals.var_t1 + 1e-50);
        (assign14880_e19167, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign14880_e19169;
        locals.var_t1_dn0 = assign14880_e19169_d_n0;
        locals.var_t1_dn2 = assign14880_e19169_d_n2;
        locals.var_t1_dn4 = assign14880_e19169_d_n4;
        locals.var_t1_dn5 = assign14880_e19169_d_n5;
        locals.var_t1_dn6 = assign14880_e19169_d_n6;
        locals.var_t1_dn8 = assign14880_e19169_d_n8;
        locals.var_t1_dn10 = assign14880_e19169_d_n10;
        locals.var_t1_dn11 = assign14880_e19169_d_n11;
        locals.var_t1_dn12 = assign14880_e19169_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign14890_e19174, assign14890_e19174_d_n0, assign14890_e19174_d_n2, assign14890_e19174_d_n4, assign14890_e19174_d_n5, assign14890_e19174_d_n6, assign14890_e19174_d_n8, assign14890_e19174_d_n10, assign14890_e19174_d_n11, assign14890_e19174_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14890_e19172: f64 = (locals.var_t1).sqrt();
        (assign14890_e19172, (locals.var_t1_dn0 / (2.0 * assign14890_e19172)), (locals.var_t1_dn2 / (2.0 * assign14890_e19172)), (locals.var_t1_dn4 / (2.0 * assign14890_e19172)), (locals.var_t1_dn5 / (2.0 * assign14890_e19172)), (locals.var_t1_dn6 / (2.0 * assign14890_e19172)), (locals.var_t1_dn8 / (2.0 * assign14890_e19172)), (locals.var_t1_dn10 / (2.0 * assign14890_e19172)), (locals.var_t1_dn11 / (2.0 * assign14890_e19172)), (locals.var_t1_dn12 / (2.0 * assign14890_e19172)),)
    } else {
        (locals.var_costi6, locals.var_costi6_dn0, locals.var_costi6_dn2, locals.var_costi6_dn4, locals.var_costi6_dn5, locals.var_costi6_dn6, locals.var_costi6_dn8, locals.var_costi6_dn10, locals.var_costi6_dn11, locals.var_costi6_dn12,)
    }
};
        locals.var_costi6 = assign14890_e19174;
        locals.var_costi6_dn0 = assign14890_e19174_d_n0;
        locals.var_costi6_dn2 = assign14890_e19174_d_n2;
        locals.var_costi6_dn4 = assign14890_e19174_d_n4;
        locals.var_costi6_dn5 = assign14890_e19174_d_n5;
        locals.var_costi6_dn6 = assign14890_e19174_d_n6;
        locals.var_costi6_dn8 = assign14890_e19174_d_n8;
        locals.var_costi6_dn10 = assign14890_e19174_d_n10;
        locals.var_costi6_dn11 = assign14890_e19174_d_n11;
        locals.var_costi6_dn12 = assign14890_e19174_d_n12;
        locals.var_costi6_rv = 0.0;

        let (assign14900_e19182, assign14900_e19182_d_n0, assign14900_e19182_d_n2, assign14900_e19182_d_n4, assign14900_e19182_d_n5, assign14900_e19182_d_n6, assign14900_e19182_d_n8, assign14900_e19182_d_n10, assign14900_e19182_d_n11, assign14900_e19182_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14900_e19179: f64 = (1.0 - locals.var_costi6);
        let assign14900_e19180: f64 = (locals.var_costi4 * assign14900_e19179);
        (assign14900_e19180, ((locals.var_costi4_dn0 * assign14900_e19179) + (locals.var_costi4 * (-locals.var_costi6_dn0))), ((locals.var_costi4_dn2 * assign14900_e19179) + (locals.var_costi4 * (-locals.var_costi6_dn2))), ((locals.var_costi4_dn4 * assign14900_e19179) + (locals.var_costi4 * (-locals.var_costi6_dn4))), ((locals.var_costi4_dn5 * assign14900_e19179) + (locals.var_costi4 * (-locals.var_costi6_dn5))), ((locals.var_costi4_dn6 * assign14900_e19179) + (locals.var_costi4 * (-locals.var_costi6_dn6))), ((locals.var_costi4_dn8 * assign14900_e19179) + (locals.var_costi4 * (-locals.var_costi6_dn8))), ((locals.var_costi4_dn10 * assign14900_e19179) + (locals.var_costi4 * (-locals.var_costi6_dn10))), ((locals.var_costi4_dn11 * assign14900_e19179) + (locals.var_costi4 * (-locals.var_costi6_dn11))), ((locals.var_costi4_dn12 * assign14900_e19179) + (locals.var_costi4 * (-locals.var_costi6_dn12))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign14900_e19182;
        locals.var_t0_dn0 = assign14900_e19182_d_n0;
        locals.var_t0_dn2 = assign14900_e19182_d_n2;
        locals.var_t0_dn4 = assign14900_e19182_d_n4;
        locals.var_t0_dn5 = assign14900_e19182_d_n5;
        locals.var_t0_dn6 = assign14900_e19182_d_n6;
        locals.var_t0_dn8 = assign14900_e19182_d_n8;
        locals.var_t0_dn10 = assign14900_e19182_d_n10;
        locals.var_t0_dn11 = assign14900_e19182_d_n11;
        locals.var_t0_dn12 = assign14900_e19182_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign14910_e19188, assign14910_e19188_d_n0, assign14910_e19188_d_n2, assign14910_e19188_d_n4, assign14910_e19188_d_n5, assign14910_e19188_d_n6, assign14910_e19188_d_n8, assign14910_e19188_d_n10, assign14910_e19188_d_n11, assign14910_e19188_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14910_e19186: f64 = (locals.var_vgssti + locals.var_t0);
        (assign14910_e19186, (locals.var_vgssti_dn0 + locals.var_t0_dn0), (locals.var_vgssti_dn2 + locals.var_t0_dn2), (locals.var_vgssti_dn4 + locals.var_t0_dn4), (locals.var_vgssti_dn5 + locals.var_t0_dn5), (locals.var_vgssti_dn6 + locals.var_t0_dn6), (locals.var_vgssti_dn8 + locals.var_t0_dn8), (locals.var_vgssti_dn10 + locals.var_t0_dn10), (locals.var_vgssti_dn11 + locals.var_t0_dn11), (locals.var_vgssti_dn12 + locals.var_t0_dn12),)
    } else {
        (locals.var_psasti, locals.var_psasti_dn0, locals.var_psasti_dn2, locals.var_psasti_dn4, locals.var_psasti_dn5, locals.var_psasti_dn6, locals.var_psasti_dn8, locals.var_psasti_dn10, locals.var_psasti_dn11, locals.var_psasti_dn12,)
    }
};
        locals.var_psasti = assign14910_e19188;
        locals.var_psasti_dn0 = assign14910_e19188_d_n0;
        locals.var_psasti_dn2 = assign14910_e19188_d_n2;
        locals.var_psasti_dn4 = assign14910_e19188_d_n4;
        locals.var_psasti_dn5 = assign14910_e19188_d_n5;
        locals.var_psasti_dn6 = assign14910_e19188_d_n6;
        locals.var_psasti_dn8 = assign14910_e19188_d_n8;
        locals.var_psasti_dn10 = assign14910_e19188_d_n10;
        locals.var_psasti_dn11 = assign14910_e19188_d_n11;
        locals.var_psasti_dn12 = assign14910_e19188_d_n12;
        locals.var_psasti_rv = 0.0;

        let (assign14920_e19200, assign14920_e19200_d_n0, assign14920_e19200_d_n2, assign14920_e19200_d_n4, assign14920_e19200_d_n5, assign14920_e19200_d_n6, assign14920_e19200_d_n8, assign14920_e19200_d_n10, assign14920_e19200_d_n11, assign14920_e19200_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14920_e19195: f64 = (locals.var_vgssti + 1e-50);
        let assign14920_e19196: f64 = (2.0 / assign14920_e19195);
        let assign14920_e19197: f64 = (locals.var_beta + assign14920_e19196);
        let assign14920_e19198: f64 = (1.0 / assign14920_e19197);
        (assign14920_e19198, (-((-((2.0 * locals.var_vgssti_dn0) / (assign14920_e19195 * assign14920_e19195))) / (assign14920_e19197 * assign14920_e19197))), (-((-((2.0 * locals.var_vgssti_dn2) / (assign14920_e19195 * assign14920_e19195))) / (assign14920_e19197 * assign14920_e19197))), (-((locals.var_beta_dn4 + (-((2.0 * locals.var_vgssti_dn4) / (assign14920_e19195 * assign14920_e19195)))) / (assign14920_e19197 * assign14920_e19197))), (-((-((2.0 * locals.var_vgssti_dn5) / (assign14920_e19195 * assign14920_e19195))) / (assign14920_e19197 * assign14920_e19197))), (-((-((2.0 * locals.var_vgssti_dn6) / (assign14920_e19195 * assign14920_e19195))) / (assign14920_e19197 * assign14920_e19197))), (-((-((2.0 * locals.var_vgssti_dn8) / (assign14920_e19195 * assign14920_e19195))) / (assign14920_e19197 * assign14920_e19197))), (-((-((2.0 * locals.var_vgssti_dn10) / (assign14920_e19195 * assign14920_e19195))) / (assign14920_e19197 * assign14920_e19197))), (-((-((2.0 * locals.var_vgssti_dn11) / (assign14920_e19195 * assign14920_e19195))) / (assign14920_e19197 * assign14920_e19197))), (-((-((2.0 * locals.var_vgssti_dn12) / (assign14920_e19195 * assign14920_e19195))) / (assign14920_e19197 * assign14920_e19197))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign14920_e19200;
        locals.var_t0_dn0 = assign14920_e19200_d_n0;
        locals.var_t0_dn2 = assign14920_e19200_d_n2;
        locals.var_t0_dn4 = assign14920_e19200_d_n4;
        locals.var_t0_dn5 = assign14920_e19200_d_n5;
        locals.var_t0_dn6 = assign14920_e19200_d_n6;
        locals.var_t0_dn8 = assign14920_e19200_d_n8;
        locals.var_t0_dn10 = assign14920_e19200_d_n10;
        locals.var_t0_dn11 = assign14920_e19200_d_n11;
        locals.var_t0_dn12 = assign14920_e19200_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign14930_e19215, assign14930_e19215_d_n0, assign14930_e19215_d_n2, assign14930_e19215_d_n4, assign14930_e19215_d_n5, assign14930_e19215_d_n6, assign14930_e19215_d_n8, assign14930_e19215_d_n10, assign14930_e19215_d_n11, assign14930_e19215_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14930_e19204: f64 = (1.0 / locals.var_costi1);
        let assign14930_e19206: f64 = (assign14930_e19204 / locals.var_costi3);
        let assign14930_e19209: f64 = (locals.var_vgssti * locals.var_vgssti);
        let assign14930_e19210: f64 = (assign14930_e19206 * assign14930_e19209);
        let assign14930_e19211: f64 = (assign14930_e19210).ln();
        let assign14930_e19213: f64 = (assign14930_e19211 * locals.var_t0);
        (assign14930_e19213, (((((((((-(locals.var_costi1_dn0 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign14930_e19204 * locals.var_costi3_dn0)) / (locals.var_costi3 * locals.var_costi3)) * assign14930_e19209) + (assign14930_e19206 * ((locals.var_vgssti_dn0 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn0)))) / assign14930_e19210) * locals.var_t0) + (assign14930_e19211 * locals.var_t0_dn0)), (((((((((-(locals.var_costi1_dn2 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign14930_e19204 * locals.var_costi3_dn2)) / (locals.var_costi3 * locals.var_costi3)) * assign14930_e19209) + (assign14930_e19206 * ((locals.var_vgssti_dn2 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn2)))) / assign14930_e19210) * locals.var_t0) + (assign14930_e19211 * locals.var_t0_dn2)), (((((((((-(locals.var_costi1_dn4 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign14930_e19204 * locals.var_costi3_dn4)) / (locals.var_costi3 * locals.var_costi3)) * assign14930_e19209) + (assign14930_e19206 * ((locals.var_vgssti_dn4 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn4)))) / assign14930_e19210) * locals.var_t0) + (assign14930_e19211 * locals.var_t0_dn4)), (((((((((-(locals.var_costi1_dn5 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign14930_e19204 * locals.var_costi3_dn5)) / (locals.var_costi3 * locals.var_costi3)) * assign14930_e19209) + (assign14930_e19206 * ((locals.var_vgssti_dn5 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn5)))) / assign14930_e19210) * locals.var_t0) + (assign14930_e19211 * locals.var_t0_dn5)), (((((((((-(locals.var_costi1_dn6 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign14930_e19204 * locals.var_costi3_dn6)) / (locals.var_costi3 * locals.var_costi3)) * assign14930_e19209) + (assign14930_e19206 * ((locals.var_vgssti_dn6 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn6)))) / assign14930_e19210) * locals.var_t0) + (assign14930_e19211 * locals.var_t0_dn6)), (((((((((-(locals.var_costi1_dn8 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign14930_e19204 * locals.var_costi3_dn8)) / (locals.var_costi3 * locals.var_costi3)) * assign14930_e19209) + (assign14930_e19206 * ((locals.var_vgssti_dn8 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn8)))) / assign14930_e19210) * locals.var_t0) + (assign14930_e19211 * locals.var_t0_dn8)), (((((((((-(locals.var_costi1_dn10 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign14930_e19204 * locals.var_costi3_dn10)) / (locals.var_costi3 * locals.var_costi3)) * assign14930_e19209) + (assign14930_e19206 * ((locals.var_vgssti_dn10 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn10)))) / assign14930_e19210) * locals.var_t0) + (assign14930_e19211 * locals.var_t0_dn10)), (((((((((-(locals.var_costi1_dn11 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign14930_e19204 * locals.var_costi3_dn11)) / (locals.var_costi3 * locals.var_costi3)) * assign14930_e19209) + (assign14930_e19206 * ((locals.var_vgssti_dn11 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn11)))) / assign14930_e19210) * locals.var_t0) + (assign14930_e19211 * locals.var_t0_dn11)), (((((((((-(locals.var_costi1_dn12 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign14930_e19204 * locals.var_costi3_dn12)) / (locals.var_costi3 * locals.var_costi3)) * assign14930_e19209) + (assign14930_e19206 * ((locals.var_vgssti_dn12 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn12)))) / assign14930_e19210) * locals.var_t0) + (assign14930_e19211 * locals.var_t0_dn12)),)
    } else {
        (locals.var_psbsti, locals.var_psbsti_dn0, locals.var_psbsti_dn2, locals.var_psbsti_dn4, locals.var_psbsti_dn5, locals.var_psbsti_dn6, locals.var_psbsti_dn8, locals.var_psbsti_dn10, locals.var_psbsti_dn11, locals.var_psbsti_dn12,)
    }
};
        locals.var_psbsti = assign14930_e19215;
        locals.var_psbsti_dn0 = assign14930_e19215_d_n0;
        locals.var_psbsti_dn2 = assign14930_e19215_d_n2;
        locals.var_psbsti_dn4 = assign14930_e19215_d_n4;
        locals.var_psbsti_dn5 = assign14930_e19215_d_n5;
        locals.var_psbsti_dn6 = assign14930_e19215_d_n6;
        locals.var_psbsti_dn8 = assign14930_e19215_d_n8;
        locals.var_psbsti_dn10 = assign14930_e19215_d_n10;
        locals.var_psbsti_dn11 = assign14930_e19215_d_n11;
        locals.var_psbsti_dn12 = assign14930_e19215_d_n12;
        locals.var_psbsti_rv = 0.0;

        let (assign14940_e19223, assign14940_e19223_d_n0, assign14940_e19223_d_n2, assign14940_e19223_d_n4, assign14940_e19223_d_n5, assign14940_e19223_d_n6, assign14940_e19223_d_n8, assign14940_e19223_d_n10, assign14940_e19223_d_n11, assign14940_e19223_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14940_e19220: f64 = (locals.var_vgssti + 1e-50);
        let assign14940_e19221: f64 = (locals.var_psbsti / assign14940_e19220);
        (assign14940_e19221, (((locals.var_psbsti_dn0 * assign14940_e19220) - (locals.var_psbsti * locals.var_vgssti_dn0)) / (assign14940_e19220 * assign14940_e19220)), (((locals.var_psbsti_dn2 * assign14940_e19220) - (locals.var_psbsti * locals.var_vgssti_dn2)) / (assign14940_e19220 * assign14940_e19220)), (((locals.var_psbsti_dn4 * assign14940_e19220) - (locals.var_psbsti * locals.var_vgssti_dn4)) / (assign14940_e19220 * assign14940_e19220)), (((locals.var_psbsti_dn5 * assign14940_e19220) - (locals.var_psbsti * locals.var_vgssti_dn5)) / (assign14940_e19220 * assign14940_e19220)), (((locals.var_psbsti_dn6 * assign14940_e19220) - (locals.var_psbsti * locals.var_vgssti_dn6)) / (assign14940_e19220 * assign14940_e19220)), (((locals.var_psbsti_dn8 * assign14940_e19220) - (locals.var_psbsti * locals.var_vgssti_dn8)) / (assign14940_e19220 * assign14940_e19220)), (((locals.var_psbsti_dn10 * assign14940_e19220) - (locals.var_psbsti * locals.var_vgssti_dn10)) / (assign14940_e19220 * assign14940_e19220)), (((locals.var_psbsti_dn11 * assign14940_e19220) - (locals.var_psbsti * locals.var_vgssti_dn11)) / (assign14940_e19220 * assign14940_e19220)), (((locals.var_psbsti_dn12 * assign14940_e19220) - (locals.var_psbsti * locals.var_vgssti_dn12)) / (assign14940_e19220 * assign14940_e19220)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign14940_e19223;
        locals.var_t3_dn0 = assign14940_e19223_d_n0;
        locals.var_t3_dn2 = assign14940_e19223_d_n2;
        locals.var_t3_dn4 = assign14940_e19223_d_n4;
        locals.var_t3_dn5 = assign14940_e19223_d_n5;
        locals.var_t3_dn6 = assign14940_e19223_d_n6;
        locals.var_t3_dn8 = assign14940_e19223_d_n8;
        locals.var_t3_dn10 = assign14940_e19223_d_n10;
        locals.var_t3_dn11 = assign14940_e19223_d_n11;
        locals.var_t3_dn12 = assign14940_e19223_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign14950_e19231, assign14950_e19231_d_n0, assign14950_e19231_d_n2, assign14950_e19231_d_n4, assign14950_e19231_d_n5, assign14950_e19231_d_n6, assign14950_e19231_d_n8, assign14950_e19231_d_n10, assign14950_e19231_d_n11, assign14950_e19231_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14950_e19227: f64 = (locals.var_psbsti - locals.var_psasti);
        let assign14950_e19229: f64 = (assign14950_e19227 - p.p136);
        (assign14950_e19229, (locals.var_psbsti_dn0 - locals.var_psasti_dn0), (locals.var_psbsti_dn2 - locals.var_psasti_dn2), (locals.var_psbsti_dn4 - locals.var_psasti_dn4), (locals.var_psbsti_dn5 - locals.var_psasti_dn5), (locals.var_psbsti_dn6 - locals.var_psasti_dn6), (locals.var_psbsti_dn8 - locals.var_psasti_dn8), (locals.var_psbsti_dn10 - locals.var_psasti_dn10), (locals.var_psbsti_dn11 - locals.var_psasti_dn11), (locals.var_psbsti_dn12 - locals.var_psasti_dn12),)
    } else {
        (locals.var_psab, locals.var_psab_dn0, locals.var_psab_dn2, locals.var_psab_dn4, locals.var_psab_dn5, locals.var_psab_dn6, locals.var_psab_dn8, locals.var_psab_dn10, locals.var_psab_dn11, locals.var_psab_dn12,)
    }
};
        locals.var_psab = assign14950_e19231;
        locals.var_psab_dn0 = assign14950_e19231_d_n0;
        locals.var_psab_dn2 = assign14950_e19231_d_n2;
        locals.var_psab_dn4 = assign14950_e19231_d_n4;
        locals.var_psab_dn5 = assign14950_e19231_d_n5;
        locals.var_psab_dn6 = assign14950_e19231_d_n6;
        locals.var_psab_dn8 = assign14950_e19231_d_n8;
        locals.var_psab_dn10 = assign14950_e19231_d_n10;
        locals.var_psab_dn11 = assign14950_e19231_d_n11;
        locals.var_psab_dn12 = assign14950_e19231_d_n12;
        locals.var_psab_rv = 0.0;

        let (assign14960_e19243, assign14960_e19243_d_n0, assign14960_e19243_d_n2, assign14960_e19243_d_n4, assign14960_e19243_d_n5, assign14960_e19243_d_n6, assign14960_e19243_d_n8, assign14960_e19243_d_n10, assign14960_e19243_d_n11, assign14960_e19243_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14960_e19235: f64 = (locals.var_psab * locals.var_psab);
        let assign14960_e19238: f64 = (4.0 * p.p136);
        let assign14960_e19240: f64 = (assign14960_e19238 * locals.var_psbsti);
        let assign14960_e19241: f64 = (assign14960_e19235 + assign14960_e19240);
        (assign14960_e19241, (((locals.var_psab_dn0 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn0)) + (assign14960_e19238 * locals.var_psbsti_dn0)), (((locals.var_psab_dn2 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn2)) + (assign14960_e19238 * locals.var_psbsti_dn2)), (((locals.var_psab_dn4 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn4)) + (assign14960_e19238 * locals.var_psbsti_dn4)), (((locals.var_psab_dn5 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn5)) + (assign14960_e19238 * locals.var_psbsti_dn5)), (((locals.var_psab_dn6 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn6)) + (assign14960_e19238 * locals.var_psbsti_dn6)), (((locals.var_psab_dn8 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn8)) + (assign14960_e19238 * locals.var_psbsti_dn8)), (((locals.var_psab_dn10 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn10)) + (assign14960_e19238 * locals.var_psbsti_dn10)), (((locals.var_psab_dn11 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn11)) + (assign14960_e19238 * locals.var_psbsti_dn11)), (((locals.var_psab_dn12 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn12)) + (assign14960_e19238 * locals.var_psbsti_dn12)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign14960_e19243;
        locals.var_t0_dn0 = assign14960_e19243_d_n0;
        locals.var_t0_dn2 = assign14960_e19243_d_n2;
        locals.var_t0_dn4 = assign14960_e19243_d_n4;
        locals.var_t0_dn5 = assign14960_e19243_d_n5;
        locals.var_t0_dn6 = assign14960_e19243_d_n6;
        locals.var_t0_dn8 = assign14960_e19243_d_n8;
        locals.var_t0_dn10 = assign14960_e19243_d_n10;
        locals.var_t0_dn11 = assign14960_e19243_d_n11;
        locals.var_t0_dn12 = assign14960_e19243_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign14970_e19256, assign14970_e19256_d_n0, assign14970_e19256_d_n2, assign14970_e19256_d_n4, assign14970_e19256_d_n5, assign14970_e19256_d_n6, assign14970_e19256_d_n8, assign14970_e19256_d_n10, assign14970_e19256_d_n11, assign14970_e19256_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14970_e19247: f64 = (locals.var_t0 * locals.var_t0);
        let assign14970_e19250: f64 = (4.0 * 1e-6);
        let assign14970_e19252: f64 = (assign14970_e19250 * 1e-6);
        let assign14970_e19253: f64 = (assign14970_e19247 + assign14970_e19252);
        let assign14970_e19254: f64 = (assign14970_e19253).sqrt();
        (assign14970_e19254, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign14970_e19254)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign14970_e19254)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign14970_e19254)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign14970_e19254)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign14970_e19254)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign14970_e19254)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign14970_e19254)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign14970_e19254)), (((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)) / (2.0 * assign14970_e19254)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign14970_e19256;
        locals.var_tmf2_dn0 = assign14970_e19256_d_n0;
        locals.var_tmf2_dn2 = assign14970_e19256_d_n2;
        locals.var_tmf2_dn4 = assign14970_e19256_d_n4;
        locals.var_tmf2_dn5 = assign14970_e19256_d_n5;
        locals.var_tmf2_dn6 = assign14970_e19256_d_n6;
        locals.var_tmf2_dn8 = assign14970_e19256_d_n8;
        locals.var_tmf2_dn10 = assign14970_e19256_d_n10;
        locals.var_tmf2_dn11 = assign14970_e19256_d_n11;
        locals.var_tmf2_dn12 = assign14970_e19256_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign14980_e19266, assign14980_e19266_d_n0, assign14980_e19266_d_n2, assign14980_e19266_d_n4, assign14980_e19266_d_n5, assign14980_e19266_d_n6, assign14980_e19266_d_n8, assign14980_e19266_d_n10, assign14980_e19266_d_n11, assign14980_e19266_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14980_e19262: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign14980_e19263: f64 = (1.0 + assign14980_e19262);
        let assign14980_e19264: f64 = (0.5 * assign14980_e19263);
        (assign14980_e19264, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn12 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign14980_e19266;
        locals.var_t2_dn0 = assign14980_e19266_d_n0;
        locals.var_t2_dn2 = assign14980_e19266_d_n2;
        locals.var_t2_dn4 = assign14980_e19266_d_n4;
        locals.var_t2_dn5 = assign14980_e19266_d_n5;
        locals.var_t2_dn6 = assign14980_e19266_d_n6;
        locals.var_t2_dn8 = assign14980_e19266_d_n8;
        locals.var_t2_dn10 = assign14980_e19266_d_n10;
        locals.var_t2_dn11 = assign14980_e19266_d_n11;
        locals.var_t2_dn12 = assign14980_e19266_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign14990_e19278, assign14990_e19278_d_n0, assign14990_e19278_d_n2, assign14990_e19278_d_n4, assign14990_e19278_d_n5, assign14990_e19278_d_n6, assign14990_e19278_d_n8, assign14990_e19278_d_n10, assign14990_e19278_d_n11, assign14990_e19278_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign14990_e19271: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign14990_e19272: f64 = (0.5 * assign14990_e19271);
        let assign14990_e19275: f64 = (1e-10 * 1e-6);
        let assign14990_e19276: f64 = (assign14990_e19272 + assign14990_e19275);
        (assign14990_e19276, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign14990_e19278;
        locals.var_t0_dn0 = assign14990_e19278_d_n0;
        locals.var_t0_dn2 = assign14990_e19278_d_n2;
        locals.var_t0_dn4 = assign14990_e19278_d_n4;
        locals.var_t0_dn5 = assign14990_e19278_d_n5;
        locals.var_t0_dn6 = assign14990_e19278_d_n6;
        locals.var_t0_dn8 = assign14990_e19278_d_n8;
        locals.var_t0_dn10 = assign14990_e19278_d_n10;
        locals.var_t0_dn11 = assign14990_e19278_d_n11;
        locals.var_t0_dn12 = assign14990_e19278_d_n12;
        locals.var_t0_rv = 0.0;

        let assign15000_e19281: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard266 = assign15000_e19281;
        locals.var_guard266_rv = 0.0;

        let (assign15010_e19287, assign15010_e19287_d_n0, assign15010_e19287_d_n2, assign15010_e19287_d_n4, assign15010_e19287_d_n5, assign15010_e19287_d_n6, assign15010_e19287_d_n8, assign15010_e19287_d_n10, assign15010_e19287_d_n11, assign15010_e19287_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard266 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15010_e19287;
        locals.var_t0_dn0 = assign15010_e19287_d_n0;
        locals.var_t0_dn2 = assign15010_e19287_d_n2;
        locals.var_t0_dn4 = assign15010_e19287_d_n4;
        locals.var_t0_dn5 = assign15010_e19287_d_n5;
        locals.var_t0_dn6 = assign15010_e19287_d_n6;
        locals.var_t0_dn8 = assign15010_e19287_d_n8;
        locals.var_t0_dn10 = assign15010_e19287_d_n10;
        locals.var_t0_dn11 = assign15010_e19287_d_n11;
        locals.var_t0_dn12 = assign15010_e19287_d_n12;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        locals: &mut StampLocals,
    ) {
        let (assign15020_e19293, assign15020_e19293_d_n0, assign15020_e19293_d_n2, assign15020_e19293_d_n4, assign15020_e19293_d_n5, assign15020_e19293_d_n6, assign15020_e19293_d_n8, assign15020_e19293_d_n10, assign15020_e19293_d_n11, assign15020_e19293_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard266 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign15020_e19293;
        locals.var_t2_dn0 = assign15020_e19293_d_n0;
        locals.var_t2_dn2 = assign15020_e19293_d_n2;
        locals.var_t2_dn4 = assign15020_e19293_d_n4;
        locals.var_t2_dn5 = assign15020_e19293_d_n5;
        locals.var_t2_dn6 = assign15020_e19293_d_n6;
        locals.var_t2_dn8 = assign15020_e19293_d_n8;
        locals.var_t2_dn10 = assign15020_e19293_d_n10;
        locals.var_t2_dn11 = assign15020_e19293_d_n11;
        locals.var_t2_dn12 = assign15020_e19293_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign15030_e19298, assign15030_e19298_d_n0, assign15030_e19298_d_n2, assign15030_e19298_d_n4, assign15030_e19298_d_n5, assign15030_e19298_d_n6, assign15030_e19298_d_n8, assign15030_e19298_d_n10, assign15030_e19298_d_n11, assign15030_e19298_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15030_e19296: f64 = (locals.var_t0).sqrt();
        (assign15030_e19296, (locals.var_t0_dn0 / (2.0 * assign15030_e19296)), (locals.var_t0_dn2 / (2.0 * assign15030_e19296)), (locals.var_t0_dn4 / (2.0 * assign15030_e19296)), (locals.var_t0_dn5 / (2.0 * assign15030_e19296)), (locals.var_t0_dn6 / (2.0 * assign15030_e19296)), (locals.var_t0_dn8 / (2.0 * assign15030_e19296)), (locals.var_t0_dn10 / (2.0 * assign15030_e19296)), (locals.var_t0_dn11 / (2.0 * assign15030_e19296)), (locals.var_t0_dn12 / (2.0 * assign15030_e19296)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15030_e19298;
        locals.var_t0_dn0 = assign15030_e19298_d_n0;
        locals.var_t0_dn2 = assign15030_e19298_d_n2;
        locals.var_t0_dn4 = assign15030_e19298_d_n4;
        locals.var_t0_dn5 = assign15030_e19298_d_n5;
        locals.var_t0_dn6 = assign15030_e19298_d_n6;
        locals.var_t0_dn8 = assign15030_e19298_d_n8;
        locals.var_t0_dn10 = assign15030_e19298_d_n10;
        locals.var_t0_dn11 = assign15030_e19298_d_n11;
        locals.var_t0_dn12 = assign15030_e19298_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign15040_e19308, assign15040_e19308_d_n0, assign15040_e19308_d_n2, assign15040_e19308_d_n4, assign15040_e19308_d_n5, assign15040_e19308_d_n6, assign15040_e19308_d_n8, assign15040_e19308_d_n10, assign15040_e19308_d_n11, assign15040_e19308_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15040_e19304: f64 = (locals.var_psab + locals.var_t0);
        let assign15040_e19305: f64 = (0.5 * assign15040_e19304);
        let assign15040_e19306: f64 = (locals.var_psbsti - assign15040_e19305);
        (assign15040_e19306, (locals.var_psbsti_dn0 - (0.5 * (locals.var_psab_dn0 + locals.var_t0_dn0))), (locals.var_psbsti_dn2 - (0.5 * (locals.var_psab_dn2 + locals.var_t0_dn2))), (locals.var_psbsti_dn4 - (0.5 * (locals.var_psab_dn4 + locals.var_t0_dn4))), (locals.var_psbsti_dn5 - (0.5 * (locals.var_psab_dn5 + locals.var_t0_dn5))), (locals.var_psbsti_dn6 - (0.5 * (locals.var_psab_dn6 + locals.var_t0_dn6))), (locals.var_psbsti_dn8 - (0.5 * (locals.var_psab_dn8 + locals.var_t0_dn8))), (locals.var_psbsti_dn10 - (0.5 * (locals.var_psab_dn10 + locals.var_t0_dn10))), (locals.var_psbsti_dn11 - (0.5 * (locals.var_psab_dn11 + locals.var_t0_dn11))), (locals.var_psbsti_dn12 - (0.5 * (locals.var_psab_dn12 + locals.var_t0_dn12))),)
    } else {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn4, locals.var_psti_dn5, locals.var_psti_dn6, locals.var_psti_dn8, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn12,)
    }
};
        locals.var_psti = assign15040_e19308;
        locals.var_psti_dn0 = assign15040_e19308_d_n0;
        locals.var_psti_dn2 = assign15040_e19308_d_n2;
        locals.var_psti_dn4 = assign15040_e19308_d_n4;
        locals.var_psti_dn5 = assign15040_e19308_d_n5;
        locals.var_psti_dn6 = assign15040_e19308_d_n6;
        locals.var_psti_dn8 = assign15040_e19308_d_n8;
        locals.var_psti_dn10 = assign15040_e19308_d_n10;
        locals.var_psti_dn11 = assign15040_e19308_d_n11;
        locals.var_psti_dn12 = assign15040_e19308_d_n12;
        locals.var_psti_rv = 0.0;

        let (assign15050_e19314, assign15050_e19314_d_n0, assign15050_e19314_d_n2, assign15050_e19314_d_n4, assign15050_e19314_d_n5, assign15050_e19314_d_n6, assign15050_e19314_d_n8, assign15050_e19314_d_n10, assign15050_e19314_d_n11, assign15050_e19314_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15050_e19312: f64 = (1.0 / locals.var_t0);
        (assign15050_e19312, (-(locals.var_t0_dn0 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn2 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn4 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn5 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn8 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))), (-(locals.var_t0_dn12 / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15050_e19314;
        locals.var_t1_dn0 = assign15050_e19314_d_n0;
        locals.var_t1_dn2 = assign15050_e19314_d_n2;
        locals.var_t1_dn4 = assign15050_e19314_d_n4;
        locals.var_t1_dn5 = assign15050_e19314_d_n5;
        locals.var_t1_dn6 = assign15050_e19314_d_n6;
        locals.var_t1_dn8 = assign15050_e19314_d_n8;
        locals.var_t1_dn10 = assign15050_e19314_d_n10;
        locals.var_t1_dn11 = assign15050_e19314_d_n11;
        locals.var_t1_dn12 = assign15050_e19314_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15060_e19323, assign15060_e19323_d_n0, assign15060_e19323_d_n2, assign15060_e19323_d_n4, assign15060_e19323_d_n5, assign15060_e19323_d_n6, assign15060_e19323_d_n8, assign15060_e19323_d_n10, assign15060_e19323_d_n11, assign15060_e19323_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15060_e19319: f64 = (locals.var_beta * locals.var_psti);
        let assign15060_e19320: f64 = (assign15060_e19319).exp();
        let assign15060_e19321: f64 = (locals.var_costi1 * assign15060_e19320);
        (assign15060_e19321, ((locals.var_costi1_dn0 * assign15060_e19320) + (locals.var_costi1 * (assign15060_e19320 * (locals.var_beta * locals.var_psti_dn0)))), ((locals.var_costi1_dn2 * assign15060_e19320) + (locals.var_costi1 * (assign15060_e19320 * (locals.var_beta * locals.var_psti_dn2)))), ((locals.var_costi1_dn4 * assign15060_e19320) + (locals.var_costi1 * (assign15060_e19320 * ((locals.var_beta_dn4 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn4))))), ((locals.var_costi1_dn5 * assign15060_e19320) + (locals.var_costi1 * (assign15060_e19320 * (locals.var_beta * locals.var_psti_dn5)))), ((locals.var_costi1_dn6 * assign15060_e19320) + (locals.var_costi1 * (assign15060_e19320 * (locals.var_beta * locals.var_psti_dn6)))), ((locals.var_costi1_dn8 * assign15060_e19320) + (locals.var_costi1 * (assign15060_e19320 * (locals.var_beta * locals.var_psti_dn8)))), ((locals.var_costi1_dn10 * assign15060_e19320) + (locals.var_costi1 * (assign15060_e19320 * (locals.var_beta * locals.var_psti_dn10)))), ((locals.var_costi1_dn11 * assign15060_e19320) + (locals.var_costi1 * (assign15060_e19320 * (locals.var_beta * locals.var_psti_dn11)))), ((locals.var_costi1_dn12 * assign15060_e19320) + (locals.var_costi1 * (assign15060_e19320 * (locals.var_beta * locals.var_psti_dn12)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15060_e19323;
        locals.var_t0_dn0 = assign15060_e19323_d_n0;
        locals.var_t0_dn2 = assign15060_e19323_d_n2;
        locals.var_t0_dn4 = assign15060_e19323_d_n4;
        locals.var_t0_dn5 = assign15060_e19323_d_n5;
        locals.var_t0_dn6 = assign15060_e19323_d_n6;
        locals.var_t0_dn8 = assign15060_e19323_d_n8;
        locals.var_t0_dn10 = assign15060_e19323_d_n10;
        locals.var_t0_dn11 = assign15060_e19323_d_n11;
        locals.var_t0_dn12 = assign15060_e19323_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign15070_e19335, assign15070_e19335_d_n0, assign15070_e19335_d_n2, assign15070_e19335_d_n4, assign15070_e19335_d_n5, assign15070_e19335_d_n6, assign15070_e19335_d_n8, assign15070_e19335_d_n10, assign15070_e19335_d_n11, assign15070_e19335_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15070_e19328: f64 = (locals.var_psti - locals.var_vbsz);
        let assign15070_e19329: f64 = (locals.var_beta * assign15070_e19328);
        let assign15070_e19331: f64 = (assign15070_e19329 - 1.0);
        let assign15070_e19333: f64 = (assign15070_e19331 + locals.var_t0);
        (assign15070_e19333, ((locals.var_beta * (locals.var_psti_dn0 - locals.var_vbsz_dn0)) + locals.var_t0_dn0), ((locals.var_beta * (locals.var_psti_dn2 - locals.var_vbsz_dn2)) + locals.var_t0_dn2), (((locals.var_beta_dn4 * assign15070_e19328) + (locals.var_beta * (locals.var_psti_dn4 - locals.var_vbsz_dn4))) + locals.var_t0_dn4), ((locals.var_beta * (locals.var_psti_dn5 - locals.var_vbsz_dn5)) + locals.var_t0_dn5), ((locals.var_beta * (locals.var_psti_dn6 - locals.var_vbsz_dn6)) + locals.var_t0_dn6), ((locals.var_beta * (locals.var_psti_dn8 - locals.var_vbsz_dn8)) + locals.var_t0_dn8), ((locals.var_beta * (locals.var_psti_dn10 - locals.var_vbsz_dn10)) + locals.var_t0_dn10), ((locals.var_beta * (locals.var_psti_dn11 - locals.var_vbsz_dn11)) + locals.var_t0_dn11), ((locals.var_beta * (locals.var_psti_dn12 - locals.var_vbsz_dn12)) + locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15070_e19335;
        locals.var_t1_dn0 = assign15070_e19335_d_n0;
        locals.var_t1_dn2 = assign15070_e19335_d_n2;
        locals.var_t1_dn4 = assign15070_e19335_d_n4;
        locals.var_t1_dn5 = assign15070_e19335_d_n5;
        locals.var_t1_dn6 = assign15070_e19335_d_n6;
        locals.var_t1_dn8 = assign15070_e19335_d_n8;
        locals.var_t1_dn10 = assign15070_e19335_d_n10;
        locals.var_t1_dn11 = assign15070_e19335_d_n11;
        locals.var_t1_dn12 = assign15070_e19335_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15080_e19348, assign15080_e19348_d_n0, assign15080_e19348_d_n2, assign15080_e19348_d_n4, assign15080_e19348_d_n5, assign15080_e19348_d_n6, assign15080_e19348_d_n8, assign15080_e19348_d_n10, assign15080_e19348_d_n11, assign15080_e19348_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15080_e19339: f64 = (locals.var_t1 * locals.var_t1);
        let assign15080_e19342: f64 = (4.0 * 0.01);
        let assign15080_e19344: f64 = (assign15080_e19342 * 0.01);
        let assign15080_e19345: f64 = (assign15080_e19339 + assign15080_e19344);
        let assign15080_e19346: f64 = (assign15080_e19345).sqrt();
        (assign15080_e19346, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign15080_e19346)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign15080_e19346)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign15080_e19346)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign15080_e19346)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign15080_e19346)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign15080_e19346)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign15080_e19346)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign15080_e19346)), (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign15080_e19346)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign15080_e19348;
        locals.var_tmf2_dn0 = assign15080_e19348_d_n0;
        locals.var_tmf2_dn2 = assign15080_e19348_d_n2;
        locals.var_tmf2_dn4 = assign15080_e19348_d_n4;
        locals.var_tmf2_dn5 = assign15080_e19348_d_n5;
        locals.var_tmf2_dn6 = assign15080_e19348_d_n6;
        locals.var_tmf2_dn8 = assign15080_e19348_d_n8;
        locals.var_tmf2_dn10 = assign15080_e19348_d_n10;
        locals.var_tmf2_dn11 = assign15080_e19348_d_n11;
        locals.var_tmf2_dn12 = assign15080_e19348_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign15090_e19358, assign15090_e19358_d_n0, assign15090_e19358_d_n2, assign15090_e19358_d_n4, assign15090_e19358_d_n5, assign15090_e19358_d_n6, assign15090_e19358_d_n8, assign15090_e19358_d_n10, assign15090_e19358_d_n11, assign15090_e19358_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15090_e19354: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign15090_e19355: f64 = (1.0 + assign15090_e19354);
        let assign15090_e19356: f64 = (0.5 * assign15090_e19355);
        (assign15090_e19356, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn12 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15090_e19358;
        locals.var_t0_dn0 = assign15090_e19358_d_n0;
        locals.var_t0_dn2 = assign15090_e19358_d_n2;
        locals.var_t0_dn4 = assign15090_e19358_d_n4;
        locals.var_t0_dn5 = assign15090_e19358_d_n5;
        locals.var_t0_dn6 = assign15090_e19358_d_n6;
        locals.var_t0_dn8 = assign15090_e19358_d_n8;
        locals.var_t0_dn10 = assign15090_e19358_d_n10;
        locals.var_t0_dn11 = assign15090_e19358_d_n11;
        locals.var_t0_dn12 = assign15090_e19358_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign15100_e19370, assign15100_e19370_d_n0, assign15100_e19370_d_n2, assign15100_e19370_d_n4, assign15100_e19370_d_n5, assign15100_e19370_d_n6, assign15100_e19370_d_n8, assign15100_e19370_d_n10, assign15100_e19370_d_n11, assign15100_e19370_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15100_e19363: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign15100_e19364: f64 = (0.5 * assign15100_e19363);
        let assign15100_e19367: f64 = (1e-10 * 0.01);
        let assign15100_e19368: f64 = (assign15100_e19364 + assign15100_e19367);
        (assign15100_e19368, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15100_e19370;
        locals.var_t1_dn0 = assign15100_e19370_d_n0;
        locals.var_t1_dn2 = assign15100_e19370_d_n2;
        locals.var_t1_dn4 = assign15100_e19370_d_n4;
        locals.var_t1_dn5 = assign15100_e19370_d_n5;
        locals.var_t1_dn6 = assign15100_e19370_d_n6;
        locals.var_t1_dn8 = assign15100_e19370_d_n8;
        locals.var_t1_dn10 = assign15100_e19370_d_n10;
        locals.var_t1_dn11 = assign15100_e19370_d_n11;
        locals.var_t1_dn12 = assign15100_e19370_d_n12;
        locals.var_t1_rv = 0.0;

        let assign15110_e19373: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard267 = assign15110_e19373;
        locals.var_guard267_rv = 0.0;

        let (assign15120_e19379, assign15120_e19379_d_n0, assign15120_e19379_d_n2, assign15120_e19379_d_n4, assign15120_e19379_d_n5, assign15120_e19379_d_n6, assign15120_e19379_d_n8, assign15120_e19379_d_n10, assign15120_e19379_d_n11, assign15120_e19379_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard267 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15120_e19379;
        locals.var_t1_dn0 = assign15120_e19379_d_n0;
        locals.var_t1_dn2 = assign15120_e19379_d_n2;
        locals.var_t1_dn4 = assign15120_e19379_d_n4;
        locals.var_t1_dn5 = assign15120_e19379_d_n5;
        locals.var_t1_dn6 = assign15120_e19379_d_n6;
        locals.var_t1_dn8 = assign15120_e19379_d_n8;
        locals.var_t1_dn10 = assign15120_e19379_d_n10;
        locals.var_t1_dn11 = assign15120_e19379_d_n11;
        locals.var_t1_dn12 = assign15120_e19379_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15130_e19385, assign15130_e19385_d_n0, assign15130_e19385_d_n2, assign15130_e19385_d_n4, assign15130_e19385_d_n5, assign15130_e19385_d_n6, assign15130_e19385_d_n8, assign15130_e19385_d_n10, assign15130_e19385_d_n11, assign15130_e19385_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard267 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15130_e19385;
        locals.var_t0_dn0 = assign15130_e19385_d_n0;
        locals.var_t0_dn2 = assign15130_e19385_d_n2;
        locals.var_t0_dn4 = assign15130_e19385_d_n4;
        locals.var_t0_dn5 = assign15130_e19385_d_n5;
        locals.var_t0_dn6 = assign15130_e19385_d_n6;
        locals.var_t0_dn8 = assign15130_e19385_d_n8;
        locals.var_t0_dn10 = assign15130_e19385_d_n10;
        locals.var_t0_dn11 = assign15130_e19385_d_n11;
        locals.var_t0_dn12 = assign15130_e19385_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign15140_e19393, assign15140_e19393_d_n0, assign15140_e19393_d_n2, assign15140_e19393_d_n4, assign15140_e19393_d_n5, assign15140_e19393_d_n6, assign15140_e19393_d_n8, assign15140_e19393_d_n10, assign15140_e19393_d_n11, assign15140_e19393_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15140_e19390: f64 = (10.0 * 2.220446049250313e-16);
        let assign15140_e19391: f64 = (locals.var_t1 + assign15140_e19390);
        (assign15140_e19391, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15140_e19393;
        locals.var_t1_dn0 = assign15140_e19393_d_n0;
        locals.var_t1_dn2 = assign15140_e19393_d_n2;
        locals.var_t1_dn4 = assign15140_e19393_d_n4;
        locals.var_t1_dn5 = assign15140_e19393_d_n5;
        locals.var_t1_dn6 = assign15140_e19393_d_n6;
        locals.var_t1_dn8 = assign15140_e19393_d_n8;
        locals.var_t1_dn10 = assign15140_e19393_d_n10;
        locals.var_t1_dn11 = assign15140_e19393_d_n11;
        locals.var_t1_dn12 = assign15140_e19393_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15150_e19398, assign15150_e19398_d_n0, assign15150_e19398_d_n2, assign15150_e19398_d_n4, assign15150_e19398_d_n5, assign15150_e19398_d_n6, assign15150_e19398_d_n8, assign15150_e19398_d_n10, assign15150_e19398_d_n11, assign15150_e19398_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15150_e19396: f64 = (locals.var_t1).sqrt();
        (assign15150_e19396, (locals.var_t1_dn0 / (2.0 * assign15150_e19396)), (locals.var_t1_dn2 / (2.0 * assign15150_e19396)), (locals.var_t1_dn4 / (2.0 * assign15150_e19396)), (locals.var_t1_dn5 / (2.0 * assign15150_e19396)), (locals.var_t1_dn6 / (2.0 * assign15150_e19396)), (locals.var_t1_dn8 / (2.0 * assign15150_e19396)), (locals.var_t1_dn10 / (2.0 * assign15150_e19396)), (locals.var_t1_dn11 / (2.0 * assign15150_e19396)), (locals.var_t1_dn12 / (2.0 * assign15150_e19396)),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn4, locals.var_sq1sti_dn5, locals.var_sq1sti_dn6, locals.var_sq1sti_dn8, locals.var_sq1sti_dn10, locals.var_sq1sti_dn11, locals.var_sq1sti_dn12,)
    }
};
        locals.var_sq1sti = assign15150_e19398;
        locals.var_sq1sti_dn0 = assign15150_e19398_d_n0;
        locals.var_sq1sti_dn2 = assign15150_e19398_d_n2;
        locals.var_sq1sti_dn4 = assign15150_e19398_d_n4;
        locals.var_sq1sti_dn5 = assign15150_e19398_d_n5;
        locals.var_sq1sti_dn6 = assign15150_e19398_d_n6;
        locals.var_sq1sti_dn8 = assign15150_e19398_d_n8;
        locals.var_sq1sti_dn10 = assign15150_e19398_d_n10;
        locals.var_sq1sti_dn11 = assign15150_e19398_d_n11;
        locals.var_sq1sti_dn12 = assign15150_e19398_d_n12;
        locals.var_sq1sti_rv = 0.0;

        let (assign15160_e19408, assign15160_e19408_d_n0, assign15160_e19408_d_n2, assign15160_e19408_d_n4, assign15160_e19408_d_n5, assign15160_e19408_d_n6, assign15160_e19408_d_n8, assign15160_e19408_d_n10, assign15160_e19408_d_n11, assign15160_e19408_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15160_e19403: f64 = (locals.var_psti - locals.var_vbsz);
        let assign15160_e19404: f64 = (locals.var_beta * assign15160_e19403);
        let assign15160_e19406: f64 = (assign15160_e19404 - 1.0);
        (assign15160_e19406, (locals.var_beta * (locals.var_psti_dn0 - locals.var_vbsz_dn0)), (locals.var_beta * (locals.var_psti_dn2 - locals.var_vbsz_dn2)), ((locals.var_beta_dn4 * assign15160_e19403) + (locals.var_beta * (locals.var_psti_dn4 - locals.var_vbsz_dn4))), (locals.var_beta * (locals.var_psti_dn5 - locals.var_vbsz_dn5)), (locals.var_beta * (locals.var_psti_dn6 - locals.var_vbsz_dn6)), (locals.var_beta * (locals.var_psti_dn8 - locals.var_vbsz_dn8)), (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbsz_dn10)), (locals.var_beta * (locals.var_psti_dn11 - locals.var_vbsz_dn11)), (locals.var_beta * (locals.var_psti_dn12 - locals.var_vbsz_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15160_e19408;
        locals.var_t1_dn0 = assign15160_e19408_d_n0;
        locals.var_t1_dn2 = assign15160_e19408_d_n2;
        locals.var_t1_dn4 = assign15160_e19408_d_n4;
        locals.var_t1_dn5 = assign15160_e19408_d_n5;
        locals.var_t1_dn6 = assign15160_e19408_d_n6;
        locals.var_t1_dn8 = assign15160_e19408_d_n8;
        locals.var_t1_dn10 = assign15160_e19408_d_n10;
        locals.var_t1_dn11 = assign15160_e19408_d_n11;
        locals.var_t1_dn12 = assign15160_e19408_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15170_e19421, assign15170_e19421_d_n0, assign15170_e19421_d_n2, assign15170_e19421_d_n4, assign15170_e19421_d_n5, assign15170_e19421_d_n6, assign15170_e19421_d_n8, assign15170_e19421_d_n10, assign15170_e19421_d_n11, assign15170_e19421_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15170_e19412: f64 = (locals.var_t1 * locals.var_t1);
        let assign15170_e19415: f64 = (4.0 * 0.01);
        let assign15170_e19417: f64 = (assign15170_e19415 * 0.01);
        let assign15170_e19418: f64 = (assign15170_e19412 + assign15170_e19417);
        let assign15170_e19419: f64 = (assign15170_e19418).sqrt();
        (assign15170_e19419, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign15170_e19419)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign15170_e19419)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign15170_e19419)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign15170_e19419)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign15170_e19419)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign15170_e19419)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign15170_e19419)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign15170_e19419)), (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign15170_e19419)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign15170_e19421;
        locals.var_tmf2_dn0 = assign15170_e19421_d_n0;
        locals.var_tmf2_dn2 = assign15170_e19421_d_n2;
        locals.var_tmf2_dn4 = assign15170_e19421_d_n4;
        locals.var_tmf2_dn5 = assign15170_e19421_d_n5;
        locals.var_tmf2_dn6 = assign15170_e19421_d_n6;
        locals.var_tmf2_dn8 = assign15170_e19421_d_n8;
        locals.var_tmf2_dn10 = assign15170_e19421_d_n10;
        locals.var_tmf2_dn11 = assign15170_e19421_d_n11;
        locals.var_tmf2_dn12 = assign15170_e19421_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign15180_e19431, assign15180_e19431_d_n0, assign15180_e19431_d_n2, assign15180_e19431_d_n4, assign15180_e19431_d_n5, assign15180_e19431_d_n6, assign15180_e19431_d_n8, assign15180_e19431_d_n10, assign15180_e19431_d_n11, assign15180_e19431_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15180_e19427: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign15180_e19428: f64 = (1.0 + assign15180_e19427);
        let assign15180_e19429: f64 = (0.5 * assign15180_e19428);
        (assign15180_e19429, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn12 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15180_e19431;
        locals.var_t0_dn0 = assign15180_e19431_d_n0;
        locals.var_t0_dn2 = assign15180_e19431_d_n2;
        locals.var_t0_dn4 = assign15180_e19431_d_n4;
        locals.var_t0_dn5 = assign15180_e19431_d_n5;
        locals.var_t0_dn6 = assign15180_e19431_d_n6;
        locals.var_t0_dn8 = assign15180_e19431_d_n8;
        locals.var_t0_dn10 = assign15180_e19431_d_n10;
        locals.var_t0_dn11 = assign15180_e19431_d_n11;
        locals.var_t0_dn12 = assign15180_e19431_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign15190_e19443, assign15190_e19443_d_n0, assign15190_e19443_d_n2, assign15190_e19443_d_n4, assign15190_e19443_d_n5, assign15190_e19443_d_n6, assign15190_e19443_d_n8, assign15190_e19443_d_n10, assign15190_e19443_d_n11, assign15190_e19443_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15190_e19436: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign15190_e19437: f64 = (0.5 * assign15190_e19436);
        let assign15190_e19440: f64 = (1e-10 * 0.01);
        let assign15190_e19441: f64 = (assign15190_e19437 + assign15190_e19440);
        (assign15190_e19441, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15190_e19443;
        locals.var_t1_dn0 = assign15190_e19443_d_n0;
        locals.var_t1_dn2 = assign15190_e19443_d_n2;
        locals.var_t1_dn4 = assign15190_e19443_d_n4;
        locals.var_t1_dn5 = assign15190_e19443_d_n5;
        locals.var_t1_dn6 = assign15190_e19443_d_n6;
        locals.var_t1_dn8 = assign15190_e19443_d_n8;
        locals.var_t1_dn10 = assign15190_e19443_d_n10;
        locals.var_t1_dn11 = assign15190_e19443_d_n11;
        locals.var_t1_dn12 = assign15190_e19443_d_n12;
        locals.var_t1_rv = 0.0;

        let assign15200_e19446: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard268 = assign15200_e19446;
        locals.var_guard268_rv = 0.0;

        let (assign15210_e19452, assign15210_e19452_d_n0, assign15210_e19452_d_n2, assign15210_e19452_d_n4, assign15210_e19452_d_n5, assign15210_e19452_d_n6, assign15210_e19452_d_n8, assign15210_e19452_d_n10, assign15210_e19452_d_n11, assign15210_e19452_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard268 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15210_e19452;
        locals.var_t1_dn0 = assign15210_e19452_d_n0;
        locals.var_t1_dn2 = assign15210_e19452_d_n2;
        locals.var_t1_dn4 = assign15210_e19452_d_n4;
        locals.var_t1_dn5 = assign15210_e19452_d_n5;
        locals.var_t1_dn6 = assign15210_e19452_d_n6;
        locals.var_t1_dn8 = assign15210_e19452_d_n8;
        locals.var_t1_dn10 = assign15210_e19452_d_n10;
        locals.var_t1_dn11 = assign15210_e19452_d_n11;
        locals.var_t1_dn12 = assign15210_e19452_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15220_e19458, assign15220_e19458_d_n0, assign15220_e19458_d_n2, assign15220_e19458_d_n4, assign15220_e19458_d_n5, assign15220_e19458_d_n6, assign15220_e19458_d_n8, assign15220_e19458_d_n10, assign15220_e19458_d_n11, assign15220_e19458_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard268 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15220_e19458;
        locals.var_t0_dn0 = assign15220_e19458_d_n0;
        locals.var_t0_dn2 = assign15220_e19458_d_n2;
        locals.var_t0_dn4 = assign15220_e19458_d_n4;
        locals.var_t0_dn5 = assign15220_e19458_d_n5;
        locals.var_t0_dn6 = assign15220_e19458_d_n6;
        locals.var_t0_dn8 = assign15220_e19458_d_n8;
        locals.var_t0_dn10 = assign15220_e19458_d_n10;
        locals.var_t0_dn11 = assign15220_e19458_d_n11;
        locals.var_t0_dn12 = assign15220_e19458_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign15230_e19466, assign15230_e19466_d_n0, assign15230_e19466_d_n2, assign15230_e19466_d_n4, assign15230_e19466_d_n5, assign15230_e19466_d_n6, assign15230_e19466_d_n8, assign15230_e19466_d_n10, assign15230_e19466_d_n11, assign15230_e19466_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15230_e19463: f64 = (10.0 * 2.220446049250313e-16);
        let assign15230_e19464: f64 = (locals.var_t1 + assign15230_e19463);
        (assign15230_e19464, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15230_e19466;
        locals.var_t1_dn0 = assign15230_e19466_d_n0;
        locals.var_t1_dn2 = assign15230_e19466_d_n2;
        locals.var_t1_dn4 = assign15230_e19466_d_n4;
        locals.var_t1_dn5 = assign15230_e19466_d_n5;
        locals.var_t1_dn6 = assign15230_e19466_d_n6;
        locals.var_t1_dn8 = assign15230_e19466_d_n8;
        locals.var_t1_dn10 = assign15230_e19466_d_n10;
        locals.var_t1_dn11 = assign15230_e19466_d_n11;
        locals.var_t1_dn12 = assign15230_e19466_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15240_e19471, assign15240_e19471_d_n0, assign15240_e19471_d_n2, assign15240_e19471_d_n4, assign15240_e19471_d_n5, assign15240_e19471_d_n6, assign15240_e19471_d_n8, assign15240_e19471_d_n10, assign15240_e19471_d_n11, assign15240_e19471_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15240_e19469: f64 = (locals.var_t1).sqrt();
        (assign15240_e19469, (locals.var_t1_dn0 / (2.0 * assign15240_e19469)), (locals.var_t1_dn2 / (2.0 * assign15240_e19469)), (locals.var_t1_dn4 / (2.0 * assign15240_e19469)), (locals.var_t1_dn5 / (2.0 * assign15240_e19469)), (locals.var_t1_dn6 / (2.0 * assign15240_e19469)), (locals.var_t1_dn8 / (2.0 * assign15240_e19469)), (locals.var_t1_dn10 / (2.0 * assign15240_e19469)), (locals.var_t1_dn11 / (2.0 * assign15240_e19469)), (locals.var_t1_dn12 / (2.0 * assign15240_e19469)),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn4, locals.var_sq2sti_dn5, locals.var_sq2sti_dn6, locals.var_sq2sti_dn8, locals.var_sq2sti_dn10, locals.var_sq2sti_dn11, locals.var_sq2sti_dn12,)
    }
};
        locals.var_sq2sti = assign15240_e19471;
        locals.var_sq2sti_dn0 = assign15240_e19471_d_n0;
        locals.var_sq2sti_dn2 = assign15240_e19471_d_n2;
        locals.var_sq2sti_dn4 = assign15240_e19471_d_n4;
        locals.var_sq2sti_dn5 = assign15240_e19471_d_n5;
        locals.var_sq2sti_dn6 = assign15240_e19471_d_n6;
        locals.var_sq2sti_dn8 = assign15240_e19471_d_n8;
        locals.var_sq2sti_dn10 = assign15240_e19471_d_n10;
        locals.var_sq2sti_dn11 = assign15240_e19471_d_n11;
        locals.var_sq2sti_dn12 = assign15240_e19471_d_n12;
        locals.var_sq2sti_rv = 0.0;

        let (assign15250_e19479, assign15250_e19479_d_n0, assign15250_e19479_d_n2, assign15250_e19479_d_n4, assign15250_e19479_d_n5, assign15250_e19479_d_n6, assign15250_e19479_d_n8, assign15250_e19479_d_n10, assign15250_e19479_d_n11, assign15250_e19479_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15250_e19476: f64 = (locals.var_sq1sti - locals.var_sq2sti);
        let assign15250_e19477: f64 = (locals.var_costi0 * assign15250_e19476);
        (assign15250_e19477, ((locals.var_costi0_dn0 * assign15250_e19476) + (locals.var_costi0 * (locals.var_sq1sti_dn0 - locals.var_sq2sti_dn0))), ((locals.var_costi0_dn2 * assign15250_e19476) + (locals.var_costi0 * (locals.var_sq1sti_dn2 - locals.var_sq2sti_dn2))), ((locals.var_costi0_dn4 * assign15250_e19476) + (locals.var_costi0 * (locals.var_sq1sti_dn4 - locals.var_sq2sti_dn4))), ((locals.var_costi0_dn5 * assign15250_e19476) + (locals.var_costi0 * (locals.var_sq1sti_dn5 - locals.var_sq2sti_dn5))), ((locals.var_costi0_dn6 * assign15250_e19476) + (locals.var_costi0 * (locals.var_sq1sti_dn6 - locals.var_sq2sti_dn6))), ((locals.var_costi0_dn8 * assign15250_e19476) + (locals.var_costi0 * (locals.var_sq1sti_dn8 - locals.var_sq2sti_dn8))), ((locals.var_costi0_dn10 * assign15250_e19476) + (locals.var_costi0 * (locals.var_sq1sti_dn10 - locals.var_sq2sti_dn10))), ((locals.var_costi0_dn11 * assign15250_e19476) + (locals.var_costi0 * (locals.var_sq1sti_dn11 - locals.var_sq2sti_dn11))), ((locals.var_costi0_dn12 * assign15250_e19476) + (locals.var_costi0 * (locals.var_sq1sti_dn12 - locals.var_sq2sti_dn12))),)
    } else {
        (locals.var_qn0sti, locals.var_qn0sti_dn0, locals.var_qn0sti_dn2, locals.var_qn0sti_dn4, locals.var_qn0sti_dn5, locals.var_qn0sti_dn6, locals.var_qn0sti_dn8, locals.var_qn0sti_dn10, locals.var_qn0sti_dn11, locals.var_qn0sti_dn12,)
    }
};
        locals.var_qn0sti = assign15250_e19479;
        locals.var_qn0sti_dn0 = assign15250_e19479_d_n0;
        locals.var_qn0sti_dn2 = assign15250_e19479_d_n2;
        locals.var_qn0sti_dn4 = assign15250_e19479_d_n4;
        locals.var_qn0sti_dn5 = assign15250_e19479_d_n5;
        locals.var_qn0sti_dn6 = assign15250_e19479_d_n6;
        locals.var_qn0sti_dn8 = assign15250_e19479_d_n8;
        locals.var_qn0sti_dn10 = assign15250_e19479_d_n10;
        locals.var_qn0sti_dn11 = assign15250_e19479_d_n11;
        locals.var_qn0sti_dn12 = assign15250_e19479_d_n12;
        locals.var_qn0sti_rv = 0.0;

        let (assign15260_e19485, assign15260_e19485_d_n0, assign15260_e19485_d_n2, assign15260_e19485_d_n4, assign15260_e19485_d_n5, assign15260_e19485_d_n6, assign15260_e19485_d_n8, assign15260_e19485_d_n10, assign15260_e19485_d_n11, assign15260_e19485_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15260_e19483: f64 = (locals.var_psasti - locals.var_psti);
        (assign15260_e19483, (locals.var_psasti_dn0 - locals.var_psti_dn0), (locals.var_psasti_dn2 - locals.var_psti_dn2), (locals.var_psasti_dn4 - locals.var_psti_dn4), (locals.var_psasti_dn5 - locals.var_psti_dn5), (locals.var_psasti_dn6 - locals.var_psti_dn6), (locals.var_psasti_dn8 - locals.var_psti_dn8), (locals.var_psasti_dn10 - locals.var_psti_dn10), (locals.var_psasti_dn11 - locals.var_psti_dn11), (locals.var_psasti_dn12 - locals.var_psti_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15260_e19485;
        locals.var_t1_dn0 = assign15260_e19485_d_n0;
        locals.var_t1_dn2 = assign15260_e19485_d_n2;
        locals.var_t1_dn4 = assign15260_e19485_d_n4;
        locals.var_t1_dn5 = assign15260_e19485_d_n5;
        locals.var_t1_dn6 = assign15260_e19485_d_n6;
        locals.var_t1_dn8 = assign15260_e19485_d_n8;
        locals.var_t1_dn10 = assign15260_e19485_d_n10;
        locals.var_t1_dn11 = assign15260_e19485_d_n11;
        locals.var_t1_dn12 = assign15260_e19485_d_n12;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        locals: &mut StampLocals,
    ) {
        let (assign15270_e19498, assign15270_e19498_d_n0, assign15270_e19498_d_n2, assign15270_e19498_d_n4, assign15270_e19498_d_n5, assign15270_e19498_d_n6, assign15270_e19498_d_n8, assign15270_e19498_d_n10, assign15270_e19498_d_n11, assign15270_e19498_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15270_e19489: f64 = (locals.var_t1 * locals.var_t1);
        let assign15270_e19492: f64 = (4.0 * 0.1);
        let assign15270_e19494: f64 = (assign15270_e19492 * 0.1);
        let assign15270_e19495: f64 = (assign15270_e19489 + assign15270_e19494);
        let assign15270_e19496: f64 = (assign15270_e19495).sqrt();
        (assign15270_e19496, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign15270_e19496)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign15270_e19496)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign15270_e19496)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign15270_e19496)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign15270_e19496)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign15270_e19496)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign15270_e19496)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign15270_e19496)), (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign15270_e19496)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign15270_e19498;
        locals.var_tmf2_dn0 = assign15270_e19498_d_n0;
        locals.var_tmf2_dn2 = assign15270_e19498_d_n2;
        locals.var_tmf2_dn4 = assign15270_e19498_d_n4;
        locals.var_tmf2_dn5 = assign15270_e19498_d_n5;
        locals.var_tmf2_dn6 = assign15270_e19498_d_n6;
        locals.var_tmf2_dn8 = assign15270_e19498_d_n8;
        locals.var_tmf2_dn10 = assign15270_e19498_d_n10;
        locals.var_tmf2_dn11 = assign15270_e19498_d_n11;
        locals.var_tmf2_dn12 = assign15270_e19498_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign15280_e19508, assign15280_e19508_d_n0, assign15280_e19508_d_n2, assign15280_e19508_d_n4, assign15280_e19508_d_n5, assign15280_e19508_d_n6, assign15280_e19508_d_n8, assign15280_e19508_d_n10, assign15280_e19508_d_n11, assign15280_e19508_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15280_e19504: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign15280_e19505: f64 = (1.0 + assign15280_e19504);
        let assign15280_e19506: f64 = (0.5 * assign15280_e19505);
        (assign15280_e19506, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn12 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign15280_e19508;
        locals.var_t2_dn0 = assign15280_e19508_d_n0;
        locals.var_t2_dn2 = assign15280_e19508_d_n2;
        locals.var_t2_dn4 = assign15280_e19508_d_n4;
        locals.var_t2_dn5 = assign15280_e19508_d_n5;
        locals.var_t2_dn6 = assign15280_e19508_d_n6;
        locals.var_t2_dn8 = assign15280_e19508_d_n8;
        locals.var_t2_dn10 = assign15280_e19508_d_n10;
        locals.var_t2_dn11 = assign15280_e19508_d_n11;
        locals.var_t2_dn12 = assign15280_e19508_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign15290_e19520, assign15290_e19520_d_n0, assign15290_e19520_d_n2, assign15290_e19520_d_n4, assign15290_e19520_d_n5, assign15290_e19520_d_n6, assign15290_e19520_d_n8, assign15290_e19520_d_n10, assign15290_e19520_d_n11, assign15290_e19520_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15290_e19513: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign15290_e19514: f64 = (0.5 * assign15290_e19513);
        let assign15290_e19517: f64 = (1e-10 * 0.1);
        let assign15290_e19518: f64 = (assign15290_e19514 + assign15290_e19517);
        (assign15290_e19518, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15290_e19520;
        locals.var_t1_dn0 = assign15290_e19520_d_n0;
        locals.var_t1_dn2 = assign15290_e19520_d_n2;
        locals.var_t1_dn4 = assign15290_e19520_d_n4;
        locals.var_t1_dn5 = assign15290_e19520_d_n5;
        locals.var_t1_dn6 = assign15290_e19520_d_n6;
        locals.var_t1_dn8 = assign15290_e19520_d_n8;
        locals.var_t1_dn10 = assign15290_e19520_d_n10;
        locals.var_t1_dn11 = assign15290_e19520_d_n11;
        locals.var_t1_dn12 = assign15290_e19520_d_n12;
        locals.var_t1_rv = 0.0;

        let assign15300_e19523: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign15300_e19523;
        locals.var_guard269_rv = 0.0;

        let (assign15310_e19529, assign15310_e19529_d_n0, assign15310_e19529_d_n2, assign15310_e19529_d_n4, assign15310_e19529_d_n5, assign15310_e19529_d_n6, assign15310_e19529_d_n8, assign15310_e19529_d_n10, assign15310_e19529_d_n11, assign15310_e19529_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard269 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15310_e19529;
        locals.var_t1_dn0 = assign15310_e19529_d_n0;
        locals.var_t1_dn2 = assign15310_e19529_d_n2;
        locals.var_t1_dn4 = assign15310_e19529_d_n4;
        locals.var_t1_dn5 = assign15310_e19529_d_n5;
        locals.var_t1_dn6 = assign15310_e19529_d_n6;
        locals.var_t1_dn8 = assign15310_e19529_d_n8;
        locals.var_t1_dn10 = assign15310_e19529_d_n10;
        locals.var_t1_dn11 = assign15310_e19529_d_n11;
        locals.var_t1_dn12 = assign15310_e19529_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15320_e19535, assign15320_e19535_d_n0, assign15320_e19535_d_n2, assign15320_e19535_d_n4, assign15320_e19535_d_n5, assign15320_e19535_d_n6, assign15320_e19535_d_n8, assign15320_e19535_d_n10, assign15320_e19535_d_n11, assign15320_e19535_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard269 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign15320_e19535;
        locals.var_t2_dn0 = assign15320_e19535_d_n0;
        locals.var_t2_dn2 = assign15320_e19535_d_n2;
        locals.var_t2_dn4 = assign15320_e19535_d_n4;
        locals.var_t2_dn5 = assign15320_e19535_d_n5;
        locals.var_t2_dn6 = assign15320_e19535_d_n6;
        locals.var_t2_dn8 = assign15320_e19535_d_n8;
        locals.var_t2_dn10 = assign15320_e19535_d_n10;
        locals.var_t2_dn11 = assign15320_e19535_d_n11;
        locals.var_t2_dn12 = assign15320_e19535_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign15330_e19543, assign15330_e19543_d_n0, assign15330_e19543_d_n2, assign15330_e19543_d_n4, assign15330_e19543_d_n5, assign15330_e19543_d_n6, assign15330_e19543_d_n8, assign15330_e19543_d_n10, assign15330_e19543_d_n11, assign15330_e19543_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15330_e19540: f64 = (10.0 * 2.220446049250313e-16);
        let assign15330_e19541: f64 = (locals.var_t1 + assign15330_e19540);
        (assign15330_e19541, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15330_e19543;
        locals.var_t1_dn0 = assign15330_e19543_d_n0;
        locals.var_t1_dn2 = assign15330_e19543_d_n2;
        locals.var_t1_dn4 = assign15330_e19543_d_n4;
        locals.var_t1_dn5 = assign15330_e19543_d_n5;
        locals.var_t1_dn6 = assign15330_e19543_d_n6;
        locals.var_t1_dn8 = assign15330_e19543_d_n8;
        locals.var_t1_dn10 = assign15330_e19543_d_n10;
        locals.var_t1_dn11 = assign15330_e19543_d_n11;
        locals.var_t1_dn12 = assign15330_e19543_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15340_e19549, assign15340_e19549_d_n0, assign15340_e19549_d_n2, assign15340_e19549_d_n4, assign15340_e19549_d_n5, assign15340_e19549_d_n6, assign15340_e19549_d_n8, assign15340_e19549_d_n10, assign15340_e19549_d_n11, assign15340_e19549_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15340_e19547: f64 = (locals.var_vds / locals.var_t1);
        (assign15340_e19547, (((locals.var_vds_dn0 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn2 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn4 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn5 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn6 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn8 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn10 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn11 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn12 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn8, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12,)
    }
};
        locals.var_tx = assign15340_e19549;
        locals.var_tx_dn0 = assign15340_e19549_d_n0;
        locals.var_tx_dn2 = assign15340_e19549_d_n2;
        locals.var_tx_dn4 = assign15340_e19549_d_n4;
        locals.var_tx_dn5 = assign15340_e19549_d_n5;
        locals.var_tx_dn6 = assign15340_e19549_d_n6;
        locals.var_tx_dn8 = assign15340_e19549_d_n8;
        locals.var_tx_dn10 = assign15340_e19549_d_n10;
        locals.var_tx_dn11 = assign15340_e19549_d_n11;
        locals.var_tx_dn12 = assign15340_e19549_d_n12;
        locals.var_tx_rv = 0.0;

        let (assign15350_e19555, assign15350_e19555_d_n0, assign15350_e19555_d_n2, assign15350_e19555_d_n4, assign15350_e19555_d_n5, assign15350_e19555_d_n6, assign15350_e19555_d_n8, assign15350_e19555_d_n10, assign15350_e19555_d_n11, assign15350_e19555_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15350_e19553: f64 = (locals.var_tx * locals.var_tx);
        (assign15350_e19553, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)), ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn12 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn12)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn8, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12,)
    }
};
        locals.var_x2 = assign15350_e19555;
        locals.var_x2_dn0 = assign15350_e19555_d_n0;
        locals.var_x2_dn2 = assign15350_e19555_d_n2;
        locals.var_x2_dn4 = assign15350_e19555_d_n4;
        locals.var_x2_dn5 = assign15350_e19555_d_n5;
        locals.var_x2_dn6 = assign15350_e19555_d_n6;
        locals.var_x2_dn8 = assign15350_e19555_d_n8;
        locals.var_x2_dn10 = assign15350_e19555_d_n10;
        locals.var_x2_dn11 = assign15350_e19555_d_n11;
        locals.var_x2_dn12 = assign15350_e19555_d_n12;
        locals.var_x2_rv = 0.0;

        let (assign15360_e19561, assign15360_e19561_d_n0, assign15360_e19561_d_n2, assign15360_e19561_d_n4, assign15360_e19561_d_n5, assign15360_e19561_d_n6, assign15360_e19561_d_n8, assign15360_e19561_d_n10, assign15360_e19561_d_n11, assign15360_e19561_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15360_e19559: f64 = 1.0;
        (assign15360_e19559, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn8, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12,)
    }
};
        locals.var_xmax2 = assign15360_e19561;
        locals.var_xmax2_dn0 = assign15360_e19561_d_n0;
        locals.var_xmax2_dn2 = assign15360_e19561_d_n2;
        locals.var_xmax2_dn4 = assign15360_e19561_d_n4;
        locals.var_xmax2_dn5 = assign15360_e19561_d_n5;
        locals.var_xmax2_dn6 = assign15360_e19561_d_n6;
        locals.var_xmax2_dn8 = assign15360_e19561_d_n8;
        locals.var_xmax2_dn10 = assign15360_e19561_d_n10;
        locals.var_xmax2_dn11 = assign15360_e19561_d_n11;
        locals.var_xmax2_dn12 = assign15360_e19561_d_n12;
        locals.var_xmax2_rv = 0.0;

        let (assign15370_e19565, assign15370_e19565_d_n0, assign15370_e19565_d_n2, assign15370_e19565_d_n4, assign15370_e19565_d_n5, assign15370_e19565_d_n6, assign15370_e19565_d_n8, assign15370_e19565_d_n10, assign15370_e19565_d_n11, assign15370_e19565_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign15370_e19565;
        locals.var_xp_dn0 = assign15370_e19565_d_n0;
        locals.var_xp_dn2 = assign15370_e19565_d_n2;
        locals.var_xp_dn4 = assign15370_e19565_d_n4;
        locals.var_xp_dn5 = assign15370_e19565_d_n5;
        locals.var_xp_dn6 = assign15370_e19565_d_n6;
        locals.var_xp_dn8 = assign15370_e19565_d_n8;
        locals.var_xp_dn10 = assign15370_e19565_d_n10;
        locals.var_xp_dn11 = assign15370_e19565_d_n11;
        locals.var_xp_dn12 = assign15370_e19565_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign15380_e19569, assign15380_e19569_d_n0, assign15380_e19569_d_n2, assign15380_e19569_d_n4, assign15380_e19569_d_n5, assign15380_e19569_d_n6, assign15380_e19569_d_n8, assign15380_e19569_d_n10, assign15380_e19569_d_n11, assign15380_e19569_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign15380_e19569;
        locals.var_xmp_dn0 = assign15380_e19569_d_n0;
        locals.var_xmp_dn2 = assign15380_e19569_d_n2;
        locals.var_xmp_dn4 = assign15380_e19569_d_n4;
        locals.var_xmp_dn5 = assign15380_e19569_d_n5;
        locals.var_xmp_dn6 = assign15380_e19569_d_n6;
        locals.var_xmp_dn8 = assign15380_e19569_d_n8;
        locals.var_xmp_dn10 = assign15380_e19569_d_n10;
        locals.var_xmp_dn11 = assign15380_e19569_d_n11;
        locals.var_xmp_dn12 = assign15380_e19569_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign15390_e19573,) = {
    if (locals.var_guard264 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign15390_e19573;
        locals.var_m0_rv = 0.0;

        let (assign15400_e19577,) = {
    if (locals.var_guard264 != 0.0) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15400_e19577;
        locals.var_mm_rv = 0.0;

        let (assign15410_e19581, assign15410_e19581_d_n0, assign15410_e19581_d_n2, assign15410_e19581_d_n4, assign15410_e19581_d_n5, assign15410_e19581_d_n6, assign15410_e19581_d_n8, assign15410_e19581_d_n10, assign15410_e19581_d_n11, assign15410_e19581_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn8, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12,)
    }
};
        locals.var_arg = assign15410_e19581;
        locals.var_arg_dn0 = assign15410_e19581_d_n0;
        locals.var_arg_dn2 = assign15410_e19581_d_n2;
        locals.var_arg_dn4 = assign15410_e19581_d_n4;
        locals.var_arg_dn5 = assign15410_e19581_d_n5;
        locals.var_arg_dn6 = assign15410_e19581_d_n6;
        locals.var_arg_dn8 = assign15410_e19581_d_n8;
        locals.var_arg_dn10 = assign15410_e19581_d_n10;
        locals.var_arg_dn11 = assign15410_e19581_d_n11;
        locals.var_arg_dn12 = assign15410_e19581_d_n12;
        locals.var_arg_rv = 0.0;

        let (assign15420_e19585, assign15420_e19585_d_n0, assign15420_e19585_d_n2, assign15420_e19585_d_n4, assign15420_e19585_d_n5, assign15420_e19585_d_n6, assign15420_e19585_d_n8, assign15420_e19585_d_n10, assign15420_e19585_d_n11, assign15420_e19585_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign15420_e19585;
        locals.var_dnm_dn0 = assign15420_e19585_d_n0;
        locals.var_dnm_dn2 = assign15420_e19585_d_n2;
        locals.var_dnm_dn4 = assign15420_e19585_d_n4;
        locals.var_dnm_dn5 = assign15420_e19585_d_n5;
        locals.var_dnm_dn6 = assign15420_e19585_d_n6;
        locals.var_dnm_dn8 = assign15420_e19585_d_n8;
        locals.var_dnm_dn10 = assign15420_e19585_d_n10;
        locals.var_dnm_dn11 = assign15420_e19585_d_n11;
        locals.var_dnm_dn12 = assign15420_e19585_d_n12;
        locals.var_dnm_rv = 0.0;

        let (assign15430_e19591, assign15430_e19591_d_n0, assign15430_e19591_d_n2, assign15430_e19591_d_n4, assign15430_e19591_d_n5, assign15430_e19591_d_n6, assign15430_e19591_d_n8, assign15430_e19591_d_n10, assign15430_e19591_d_n11, assign15430_e19591_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15430_e19589: f64 = (locals.var_xp * locals.var_x2);
        (assign15430_e19589, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign15430_e19591;
        locals.var_xp_dn0 = assign15430_e19591_d_n0;
        locals.var_xp_dn2 = assign15430_e19591_d_n2;
        locals.var_xp_dn4 = assign15430_e19591_d_n4;
        locals.var_xp_dn5 = assign15430_e19591_d_n5;
        locals.var_xp_dn6 = assign15430_e19591_d_n6;
        locals.var_xp_dn8 = assign15430_e19591_d_n8;
        locals.var_xp_dn10 = assign15430_e19591_d_n10;
        locals.var_xp_dn11 = assign15430_e19591_d_n11;
        locals.var_xp_dn12 = assign15430_e19591_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign15440_e19597, assign15440_e19597_d_n0, assign15440_e19597_d_n2, assign15440_e19597_d_n4, assign15440_e19597_d_n5, assign15440_e19597_d_n6, assign15440_e19597_d_n8, assign15440_e19597_d_n10, assign15440_e19597_d_n11, assign15440_e19597_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15440_e19595: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15440_e19595, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign15440_e19597;
        locals.var_xmp_dn0 = assign15440_e19597_d_n0;
        locals.var_xmp_dn2 = assign15440_e19597_d_n2;
        locals.var_xmp_dn4 = assign15440_e19597_d_n4;
        locals.var_xmp_dn5 = assign15440_e19597_d_n5;
        locals.var_xmp_dn6 = assign15440_e19597_d_n6;
        locals.var_xmp_dn8 = assign15440_e19597_d_n8;
        locals.var_xmp_dn10 = assign15440_e19597_d_n10;
        locals.var_xmp_dn11 = assign15440_e19597_d_n11;
        locals.var_xmp_dn12 = assign15440_e19597_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign15450_e19603, assign15450_e19603_d_n0, assign15450_e19603_d_n2, assign15450_e19603_d_n4, assign15450_e19603_d_n5, assign15450_e19603_d_n6, assign15450_e19603_d_n8, assign15450_e19603_d_n10, assign15450_e19603_d_n11, assign15450_e19603_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15450_e19601: f64 = (locals.var_xp * locals.var_x2);
        (assign15450_e19601, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign15450_e19603;
        locals.var_xp_dn0 = assign15450_e19603_d_n0;
        locals.var_xp_dn2 = assign15450_e19603_d_n2;
        locals.var_xp_dn4 = assign15450_e19603_d_n4;
        locals.var_xp_dn5 = assign15450_e19603_d_n5;
        locals.var_xp_dn6 = assign15450_e19603_d_n6;
        locals.var_xp_dn8 = assign15450_e19603_d_n8;
        locals.var_xp_dn10 = assign15450_e19603_d_n10;
        locals.var_xp_dn11 = assign15450_e19603_d_n11;
        locals.var_xp_dn12 = assign15450_e19603_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign15460_e19609, assign15460_e19609_d_n0, assign15460_e19609_d_n2, assign15460_e19609_d_n4, assign15460_e19609_d_n5, assign15460_e19609_d_n6, assign15460_e19609_d_n8, assign15460_e19609_d_n10, assign15460_e19609_d_n11, assign15460_e19609_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15460_e19607: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15460_e19607, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign15460_e19609;
        locals.var_xmp_dn0 = assign15460_e19609_d_n0;
        locals.var_xmp_dn2 = assign15460_e19609_d_n2;
        locals.var_xmp_dn4 = assign15460_e19609_d_n4;
        locals.var_xmp_dn5 = assign15460_e19609_d_n5;
        locals.var_xmp_dn6 = assign15460_e19609_d_n6;
        locals.var_xmp_dn8 = assign15460_e19609_d_n8;
        locals.var_xmp_dn10 = assign15460_e19609_d_n10;
        locals.var_xmp_dn11 = assign15460_e19609_d_n11;
        locals.var_xmp_dn12 = assign15460_e19609_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign15470_e19615, assign15470_e19615_d_n0, assign15470_e19615_d_n2, assign15470_e19615_d_n4, assign15470_e19615_d_n5, assign15470_e19615_d_n6, assign15470_e19615_d_n8, assign15470_e19615_d_n10, assign15470_e19615_d_n11, assign15470_e19615_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15470_e19613: f64 = (locals.var_xp * locals.var_x2);
        (assign15470_e19613, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign15470_e19615;
        locals.var_xp_dn0 = assign15470_e19615_d_n0;
        locals.var_xp_dn2 = assign15470_e19615_d_n2;
        locals.var_xp_dn4 = assign15470_e19615_d_n4;
        locals.var_xp_dn5 = assign15470_e19615_d_n5;
        locals.var_xp_dn6 = assign15470_e19615_d_n6;
        locals.var_xp_dn8 = assign15470_e19615_d_n8;
        locals.var_xp_dn10 = assign15470_e19615_d_n10;
        locals.var_xp_dn11 = assign15470_e19615_d_n11;
        locals.var_xp_dn12 = assign15470_e19615_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign15480_e19621, assign15480_e19621_d_n0, assign15480_e19621_d_n2, assign15480_e19621_d_n4, assign15480_e19621_d_n5, assign15480_e19621_d_n6, assign15480_e19621_d_n8, assign15480_e19621_d_n10, assign15480_e19621_d_n11, assign15480_e19621_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15480_e19619: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15480_e19619, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign15480_e19621;
        locals.var_xmp_dn0 = assign15480_e19621_d_n0;
        locals.var_xmp_dn2 = assign15480_e19621_d_n2;
        locals.var_xmp_dn4 = assign15480_e19621_d_n4;
        locals.var_xmp_dn5 = assign15480_e19621_d_n5;
        locals.var_xmp_dn6 = assign15480_e19621_d_n6;
        locals.var_xmp_dn8 = assign15480_e19621_d_n8;
        locals.var_xmp_dn10 = assign15480_e19621_d_n10;
        locals.var_xmp_dn11 = assign15480_e19621_d_n11;
        locals.var_xmp_dn12 = assign15480_e19621_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign15490_e19627, assign15490_e19627_d_n0, assign15490_e19627_d_n2, assign15490_e19627_d_n4, assign15490_e19627_d_n5, assign15490_e19627_d_n6, assign15490_e19627_d_n8, assign15490_e19627_d_n10, assign15490_e19627_d_n11, assign15490_e19627_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15490_e19625: f64 = (locals.var_xp * locals.var_x2);
        (assign15490_e19625, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign15490_e19627;
        locals.var_xp_dn0 = assign15490_e19627_d_n0;
        locals.var_xp_dn2 = assign15490_e19627_d_n2;
        locals.var_xp_dn4 = assign15490_e19627_d_n4;
        locals.var_xp_dn5 = assign15490_e19627_d_n5;
        locals.var_xp_dn6 = assign15490_e19627_d_n6;
        locals.var_xp_dn8 = assign15490_e19627_d_n8;
        locals.var_xp_dn10 = assign15490_e19627_d_n10;
        locals.var_xp_dn11 = assign15490_e19627_d_n11;
        locals.var_xp_dn12 = assign15490_e19627_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign15500_e19633, assign15500_e19633_d_n0, assign15500_e19633_d_n2, assign15500_e19633_d_n4, assign15500_e19633_d_n5, assign15500_e19633_d_n6, assign15500_e19633_d_n8, assign15500_e19633_d_n10, assign15500_e19633_d_n11, assign15500_e19633_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15500_e19631: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign15500_e19631, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign15500_e19633;
        locals.var_xmp_dn0 = assign15500_e19633_d_n0;
        locals.var_xmp_dn2 = assign15500_e19633_d_n2;
        locals.var_xmp_dn4 = assign15500_e19633_d_n4;
        locals.var_xmp_dn5 = assign15500_e19633_d_n5;
        locals.var_xmp_dn6 = assign15500_e19633_d_n6;
        locals.var_xmp_dn8 = assign15500_e19633_d_n8;
        locals.var_xmp_dn10 = assign15500_e19633_d_n10;
        locals.var_xmp_dn11 = assign15500_e19633_d_n11;
        locals.var_xmp_dn12 = assign15500_e19633_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign15510_e19639, assign15510_e19639_d_n0, assign15510_e19639_d_n2, assign15510_e19639_d_n4, assign15510_e19639_d_n5, assign15510_e19639_d_n6, assign15510_e19639_d_n8, assign15510_e19639_d_n10, assign15510_e19639_d_n11, assign15510_e19639_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15510_e19637: f64 = (locals.var_xp + locals.var_xmp);
        (assign15510_e19637, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn8, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12,)
    }
};
        locals.var_arg = assign15510_e19639;
        locals.var_arg_dn0 = assign15510_e19639_d_n0;
        locals.var_arg_dn2 = assign15510_e19639_d_n2;
        locals.var_arg_dn4 = assign15510_e19639_d_n4;
        locals.var_arg_dn5 = assign15510_e19639_d_n5;
        locals.var_arg_dn6 = assign15510_e19639_d_n6;
        locals.var_arg_dn8 = assign15510_e19639_d_n8;
        locals.var_arg_dn10 = assign15510_e19639_d_n10;
        locals.var_arg_dn11 = assign15510_e19639_d_n11;
        locals.var_arg_dn12 = assign15510_e19639_d_n12;
        locals.var_arg_rv = 0.0;

        let (assign15520_e19643, assign15520_e19643_d_n0, assign15520_e19643_d_n2, assign15520_e19643_d_n4, assign15520_e19643_d_n5, assign15520_e19643_d_n6, assign15520_e19643_d_n8, assign15520_e19643_d_n10, assign15520_e19643_d_n11, assign15520_e19643_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn8, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign15520_e19643;
        locals.var_dnm_dn0 = assign15520_e19643_d_n0;
        locals.var_dnm_dn2 = assign15520_e19643_d_n2;
        locals.var_dnm_dn4 = assign15520_e19643_d_n4;
        locals.var_dnm_dn5 = assign15520_e19643_d_n5;
        locals.var_dnm_dn6 = assign15520_e19643_d_n6;
        locals.var_dnm_dn8 = assign15520_e19643_d_n8;
        locals.var_dnm_dn10 = assign15520_e19643_d_n10;
        locals.var_dnm_dn11 = assign15520_e19643_d_n11;
        locals.var_dnm_dn12 = assign15520_e19643_d_n12;
        locals.var_dnm_rv = 0.0;

        let assign15530_e19658: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard270 = assign15530_e19658;
        locals.var_guard270_rv = 0.0;

        let assign15540_e19661: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard271 = assign15540_e19661;
        locals.var_guard271_rv = 0.0;

        let (assign15550_e19669,) = {
    if (((locals.var_guard264 != 0.0) && (locals.var_guard270 != 0.0)) && (locals.var_guard271 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15550_e19669;
        locals.var_mm_rv = 0.0;

        let assign15560_e19672: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard272 = assign15560_e19672;
        locals.var_guard272_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15570_e19683,) = {
    if ((((locals.var_guard264 != 0.0) && (locals.var_guard270 != 0.0)) && (locals.var_guard271 == 0.0)) && (locals.var_guard272 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15570_e19683;
        locals.var_mm_rv = 0.0;

        let assign15580_e19686: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign15580_e19686;
        locals.var_guard273_rv = 0.0;

        let (assign15590_e19700,) = {
    if (((((locals.var_guard264 != 0.0) && (locals.var_guard270 != 0.0)) && (locals.var_guard271 == 0.0)) && (locals.var_guard272 == 0.0)) && (locals.var_guard273 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15590_e19700;
        locals.var_mm_rv = 0.0;

        let assign15600_e19703: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign15600_e19703;
        locals.var_guard274_rv = 0.0;

        let (assign15610_e19720,) = {
    if ((((((locals.var_guard264 != 0.0) && (locals.var_guard270 != 0.0)) && (locals.var_guard271 == 0.0)) && (locals.var_guard272 == 0.0)) && (locals.var_guard273 == 0.0)) && (locals.var_guard274 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign15610_e19720;
        locals.var_mm_rv = 0.0;

        let (assign15620_e19726,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard270 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign15620_e19726;
        locals.var_m0_rv = 0.0;

        let mut assign15630_loop_guard: usize = 0;
        while {
            let assign15630_cond_e19733: f64 = if (((locals.var_guard264 != 0.0) && (locals.var_guard270 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign15630_cond_e19733 != 0.0
        } {
            assign15630_loop_guard += 1;
            assert!(assign15630_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15630_body0_e19740, assign15630_body0_e19740_d_n0, assign15630_body0_e19740_d_n2, assign15630_body0_e19740_d_n4, assign15630_body0_e19740_d_n5, assign15630_body0_e19740_d_n6, assign15630_body0_e19740_d_n8, assign15630_body0_e19740_d_n10, assign15630_body0_e19740_d_n11, assign15630_body0_e19740_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard270 != 0.0)) {
        let assign15630_body0_e19738: f64 = (locals.var_dnm).sqrt();
        (assign15630_body0_e19738, (locals.var_dnm_dn0 / (2.0 * assign15630_body0_e19738)), (locals.var_dnm_dn2 / (2.0 * assign15630_body0_e19738)), (locals.var_dnm_dn4 / (2.0 * assign15630_body0_e19738)), (locals.var_dnm_dn5 / (2.0 * assign15630_body0_e19738)), (locals.var_dnm_dn6 / (2.0 * assign15630_body0_e19738)), (locals.var_dnm_dn8 / (2.0 * assign15630_body0_e19738)), (locals.var_dnm_dn10 / (2.0 * assign15630_body0_e19738)), (locals.var_dnm_dn11 / (2.0 * assign15630_body0_e19738)), (locals.var_dnm_dn12 / (2.0 * assign15630_body0_e19738)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
            locals.var_dnm = assign15630_body0_e19740;
            locals.var_dnm_dn0 = assign15630_body0_e19740_d_n0;
            locals.var_dnm_dn2 = assign15630_body0_e19740_d_n2;
            locals.var_dnm_dn4 = assign15630_body0_e19740_d_n4;
            locals.var_dnm_dn5 = assign15630_body0_e19740_d_n5;
            locals.var_dnm_dn6 = assign15630_body0_e19740_d_n6;
            locals.var_dnm_dn8 = assign15630_body0_e19740_d_n8;
            locals.var_dnm_dn10 = assign15630_body0_e19740_d_n10;
            locals.var_dnm_dn11 = assign15630_body0_e19740_d_n11;
            locals.var_dnm_dn12 = assign15630_body0_e19740_d_n12;
            locals.var_dnm_rv = 0.0;
            let (assign15630_body1_e19748,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard270 != 0.0)) {
        let assign15630_body1_e19746: f64 = (locals.var_m0 + 1.0);
        (assign15630_body1_e19746,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign15630_body1_e19748;
            locals.var_m0_rv = 0.0;
        }

        let (assign15640_e19761, assign15640_e19761_d_n0, assign15640_e19761_d_n2, assign15640_e19761_d_n4, assign15640_e19761_d_n5, assign15640_e19761_d_n6, assign15640_e19761_d_n8, assign15640_e19761_d_n10, assign15640_e19761_d_n11, assign15640_e19761_d_n12,) = {
    if ((locals.var_guard264 != 0.0) && (locals.var_guard270 == 0.0)) {
        let assign15640_e19757: f64 = (2.0 * 4.0);
        let assign15640_e19758: f64 = (1.0 / assign15640_e19757);
        let assign15640_e19759: f64 = (locals.var_dnm).powf(assign15640_e19758);
        (assign15640_e19759, if 0.0 == 0.0 && ((assign15640_e19758) as f64).is_finite() && ((assign15640_e19758) as f64).fract() == 0.0 { if assign15640_e19758 == 0.0 { 0.0 } else { (assign15640_e19758 * ((locals.var_dnm).powf(assign15640_e19758 - 1.0) * locals.var_dnm_dn0)) } } else { (assign15640_e19759 * (assign15640_e19758 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15640_e19758) as f64).is_finite() && ((assign15640_e19758) as f64).fract() == 0.0 { if assign15640_e19758 == 0.0 { 0.0 } else { (assign15640_e19758 * ((locals.var_dnm).powf(assign15640_e19758 - 1.0) * locals.var_dnm_dn2)) } } else { (assign15640_e19759 * (assign15640_e19758 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15640_e19758) as f64).is_finite() && ((assign15640_e19758) as f64).fract() == 0.0 { if assign15640_e19758 == 0.0 { 0.0 } else { (assign15640_e19758 * ((locals.var_dnm).powf(assign15640_e19758 - 1.0) * locals.var_dnm_dn4)) } } else { (assign15640_e19759 * (assign15640_e19758 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15640_e19758) as f64).is_finite() && ((assign15640_e19758) as f64).fract() == 0.0 { if assign15640_e19758 == 0.0 { 0.0 } else { (assign15640_e19758 * ((locals.var_dnm).powf(assign15640_e19758 - 1.0) * locals.var_dnm_dn5)) } } else { (assign15640_e19759 * (assign15640_e19758 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15640_e19758) as f64).is_finite() && ((assign15640_e19758) as f64).fract() == 0.0 { if assign15640_e19758 == 0.0 { 0.0 } else { (assign15640_e19758 * ((locals.var_dnm).powf(assign15640_e19758 - 1.0) * locals.var_dnm_dn6)) } } else { (assign15640_e19759 * (assign15640_e19758 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15640_e19758) as f64).is_finite() && ((assign15640_e19758) as f64).fract() == 0.0 { if assign15640_e19758 == 0.0 { 0.0 } else { (assign15640_e19758 * ((locals.var_dnm).powf(assign15640_e19758 - 1.0) * locals.var_dnm_dn8)) } } else { (assign15640_e19759 * (assign15640_e19758 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15640_e19758) as f64).is_finite() && ((assign15640_e19758) as f64).fract() == 0.0 { if assign15640_e19758 == 0.0 { 0.0 } else { (assign15640_e19758 * ((locals.var_dnm).powf(assign15640_e19758 - 1.0) * locals.var_dnm_dn10)) } } else { (assign15640_e19759 * (assign15640_e19758 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15640_e19758) as f64).is_finite() && ((assign15640_e19758) as f64).fract() == 0.0 { if assign15640_e19758 == 0.0 { 0.0 } else { (assign15640_e19758 * ((locals.var_dnm).powf(assign15640_e19758 - 1.0) * locals.var_dnm_dn11)) } } else { (assign15640_e19759 * (assign15640_e19758 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign15640_e19758) as f64).is_finite() && ((assign15640_e19758) as f64).fract() == 0.0 { if assign15640_e19758 == 0.0 { 0.0 } else { (assign15640_e19758 * ((locals.var_dnm).powf(assign15640_e19758 - 1.0) * locals.var_dnm_dn12)) } } else { (assign15640_e19759 * (assign15640_e19758 * (locals.var_dnm_dn12 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign15640_e19761;
        locals.var_dnm_dn0 = assign15640_e19761_d_n0;
        locals.var_dnm_dn2 = assign15640_e19761_d_n2;
        locals.var_dnm_dn4 = assign15640_e19761_d_n4;
        locals.var_dnm_dn5 = assign15640_e19761_d_n5;
        locals.var_dnm_dn6 = assign15640_e19761_d_n6;
        locals.var_dnm_dn8 = assign15640_e19761_d_n8;
        locals.var_dnm_dn10 = assign15640_e19761_d_n10;
        locals.var_dnm_dn11 = assign15640_e19761_d_n11;
        locals.var_dnm_dn12 = assign15640_e19761_d_n12;
        locals.var_dnm_rv = 0.0;

        let (assign15650_e19769, assign15650_e19769_d_n0, assign15650_e19769_d_n2, assign15650_e19769_d_n4, assign15650_e19769_d_n5, assign15650_e19769_d_n6, assign15650_e19769_d_n8, assign15650_e19769_d_n10, assign15650_e19769_d_n11, assign15650_e19769_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15650_e19766: f64 = (locals.var_dnm + 1e-50);
        let assign15650_e19767: f64 = (1.0 / assign15650_e19766);
        (assign15650_e19767, (-(locals.var_dnm_dn0 / (assign15650_e19766 * assign15650_e19766))), (-(locals.var_dnm_dn2 / (assign15650_e19766 * assign15650_e19766))), (-(locals.var_dnm_dn4 / (assign15650_e19766 * assign15650_e19766))), (-(locals.var_dnm_dn5 / (assign15650_e19766 * assign15650_e19766))), (-(locals.var_dnm_dn6 / (assign15650_e19766 * assign15650_e19766))), (-(locals.var_dnm_dn8 / (assign15650_e19766 * assign15650_e19766))), (-(locals.var_dnm_dn10 / (assign15650_e19766 * assign15650_e19766))), (-(locals.var_dnm_dn11 / (assign15650_e19766 * assign15650_e19766))), (-(locals.var_dnm_dn12 / (assign15650_e19766 * assign15650_e19766))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign15650_e19769;
        locals.var_dnm_dn0 = assign15650_e19769_d_n0;
        locals.var_dnm_dn2 = assign15650_e19769_d_n2;
        locals.var_dnm_dn4 = assign15650_e19769_d_n4;
        locals.var_dnm_dn5 = assign15650_e19769_d_n5;
        locals.var_dnm_dn6 = assign15650_e19769_d_n6;
        locals.var_dnm_dn8 = assign15650_e19769_d_n8;
        locals.var_dnm_dn10 = assign15650_e19769_d_n10;
        locals.var_dnm_dn11 = assign15650_e19769_d_n11;
        locals.var_dnm_dn12 = assign15650_e19769_d_n12;
        locals.var_dnm_rv = 0.0;

        let (assign15660_e19777, assign15660_e19777_d_n0, assign15660_e19777_d_n2, assign15660_e19777_d_n4, assign15660_e19777_d_n5, assign15660_e19777_d_n6, assign15660_e19777_d_n8, assign15660_e19777_d_n10, assign15660_e19777_d_n11, assign15660_e19777_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15660_e19773: f64 = locals.var_tx;
        let assign15660_e19775: f64 = (assign15660_e19773 * locals.var_dnm);
        (assign15660_e19775, ((locals.var_tx_dn0 * locals.var_dnm) + (assign15660_e19773 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign15660_e19773 * locals.var_dnm_dn2)), ((locals.var_tx_dn4 * locals.var_dnm) + (assign15660_e19773 * locals.var_dnm_dn4)), ((locals.var_tx_dn5 * locals.var_dnm) + (assign15660_e19773 * locals.var_dnm_dn5)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign15660_e19773 * locals.var_dnm_dn6)), ((locals.var_tx_dn8 * locals.var_dnm) + (assign15660_e19773 * locals.var_dnm_dn8)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign15660_e19773 * locals.var_dnm_dn10)), ((locals.var_tx_dn11 * locals.var_dnm) + (assign15660_e19773 * locals.var_dnm_dn11)), ((locals.var_tx_dn12 * locals.var_dnm) + (assign15660_e19773 * locals.var_dnm_dn12)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn8, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12,)
    }
};
        locals.var_ty = assign15660_e19777;
        locals.var_ty_dn0 = assign15660_e19777_d_n0;
        locals.var_ty_dn2 = assign15660_e19777_d_n2;
        locals.var_ty_dn4 = assign15660_e19777_d_n4;
        locals.var_ty_dn5 = assign15660_e19777_d_n5;
        locals.var_ty_dn6 = assign15660_e19777_d_n6;
        locals.var_ty_dn8 = assign15660_e19777_d_n8;
        locals.var_ty_dn10 = assign15660_e19777_d_n10;
        locals.var_ty_dn11 = assign15660_e19777_d_n11;
        locals.var_ty_dn12 = assign15660_e19777_d_n12;
        locals.var_ty_rv = 0.0;

        let (assign15670_e19789, assign15670_e19789_d_n0, assign15670_e19789_d_n2, assign15670_e19789_d_n4, assign15670_e19789_d_n5, assign15670_e19789_d_n6, assign15670_e19789_d_n8, assign15670_e19789_d_n10, assign15670_e19789_d_n11, assign15670_e19789_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15670_e19781: f64 = locals.var_xmp;
        let assign15670_e19783: f64 = (assign15670_e19781 * locals.var_dnm);
        let assign15670_e19786: f64 = (locals.var_arg + 1e-50);
        let assign15670_e19787: f64 = (assign15670_e19783 / assign15670_e19786);
        (assign15670_e19787, (((((locals.var_xmp_dn0 * locals.var_dnm) + (assign15670_e19781 * locals.var_dnm_dn0)) * assign15670_e19786) - (assign15670_e19783 * locals.var_arg_dn0)) / (assign15670_e19786 * assign15670_e19786)), (((((locals.var_xmp_dn2 * locals.var_dnm) + (assign15670_e19781 * locals.var_dnm_dn2)) * assign15670_e19786) - (assign15670_e19783 * locals.var_arg_dn2)) / (assign15670_e19786 * assign15670_e19786)), (((((locals.var_xmp_dn4 * locals.var_dnm) + (assign15670_e19781 * locals.var_dnm_dn4)) * assign15670_e19786) - (assign15670_e19783 * locals.var_arg_dn4)) / (assign15670_e19786 * assign15670_e19786)), (((((locals.var_xmp_dn5 * locals.var_dnm) + (assign15670_e19781 * locals.var_dnm_dn5)) * assign15670_e19786) - (assign15670_e19783 * locals.var_arg_dn5)) / (assign15670_e19786 * assign15670_e19786)), (((((locals.var_xmp_dn6 * locals.var_dnm) + (assign15670_e19781 * locals.var_dnm_dn6)) * assign15670_e19786) - (assign15670_e19783 * locals.var_arg_dn6)) / (assign15670_e19786 * assign15670_e19786)), (((((locals.var_xmp_dn8 * locals.var_dnm) + (assign15670_e19781 * locals.var_dnm_dn8)) * assign15670_e19786) - (assign15670_e19783 * locals.var_arg_dn8)) / (assign15670_e19786 * assign15670_e19786)), (((((locals.var_xmp_dn10 * locals.var_dnm) + (assign15670_e19781 * locals.var_dnm_dn10)) * assign15670_e19786) - (assign15670_e19783 * locals.var_arg_dn10)) / (assign15670_e19786 * assign15670_e19786)), (((((locals.var_xmp_dn11 * locals.var_dnm) + (assign15670_e19781 * locals.var_dnm_dn11)) * assign15670_e19786) - (assign15670_e19783 * locals.var_arg_dn11)) / (assign15670_e19786 * assign15670_e19786)), (((((locals.var_xmp_dn12 * locals.var_dnm) + (assign15670_e19781 * locals.var_dnm_dn12)) * assign15670_e19786) - (assign15670_e19783 * locals.var_arg_dn12)) / (assign15670_e19786 * assign15670_e19786)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign15670_e19789;
        locals.var_t2_dn0 = assign15670_e19789_d_n0;
        locals.var_t2_dn2 = assign15670_e19789_d_n2;
        locals.var_t2_dn4 = assign15670_e19789_d_n4;
        locals.var_t2_dn5 = assign15670_e19789_d_n5;
        locals.var_t2_dn6 = assign15670_e19789_d_n6;
        locals.var_t2_dn8 = assign15670_e19789_d_n8;
        locals.var_t2_dn10 = assign15670_e19789_d_n10;
        locals.var_t2_dn11 = assign15670_e19789_d_n11;
        locals.var_t2_dn12 = assign15670_e19789_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign15680_e19799, assign15680_e19799_d_n0, assign15680_e19799_d_n2, assign15680_e19799_d_n4, assign15680_e19799_d_n5, assign15680_e19799_d_n6, assign15680_e19799_d_n8, assign15680_e19799_d_n10, assign15680_e19799_d_n11, assign15680_e19799_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15680_e19793: f64 = (2.0 * locals.var_uc_wsti);
        let assign15680_e19795: f64 = (assign15680_e19793 * p.p5);
        let assign15680_e19797: f64 = (assign15680_e19795 * locals.var_beta_inv);
        (assign15680_e19797, (((2.0 * locals.var_uc_wsti_dn0) * p.p5) * locals.var_beta_inv), (((2.0 * locals.var_uc_wsti_dn2) * p.p5) * locals.var_beta_inv), ((((2.0 * locals.var_uc_wsti_dn4) * p.p5) * locals.var_beta_inv) + (assign15680_e19795 * locals.var_beta_inv_dn4)), (((2.0 * locals.var_uc_wsti_dn5) * p.p5) * locals.var_beta_inv), (((2.0 * locals.var_uc_wsti_dn6) * p.p5) * locals.var_beta_inv), (((2.0 * locals.var_uc_wsti_dn8) * p.p5) * locals.var_beta_inv), (((2.0 * locals.var_uc_wsti_dn10) * p.p5) * locals.var_beta_inv), (((2.0 * locals.var_uc_wsti_dn11) * p.p5) * locals.var_beta_inv), (((2.0 * locals.var_uc_wsti_dn12) * p.p5) * locals.var_beta_inv),)
    } else {
        (locals.var_costi7, locals.var_costi7_dn0, locals.var_costi7_dn2, locals.var_costi7_dn4, locals.var_costi7_dn5, locals.var_costi7_dn6, locals.var_costi7_dn8, locals.var_costi7_dn10, locals.var_costi7_dn11, locals.var_costi7_dn12,)
    }
};
        locals.var_costi7 = assign15680_e19799;
        locals.var_costi7_dn0 = assign15680_e19799_d_n0;
        locals.var_costi7_dn2 = assign15680_e19799_d_n2;
        locals.var_costi7_dn4 = assign15680_e19799_d_n4;
        locals.var_costi7_dn5 = assign15680_e19799_d_n5;
        locals.var_costi7_dn6 = assign15680_e19799_d_n6;
        locals.var_costi7_dn8 = assign15680_e19799_d_n8;
        locals.var_costi7_dn10 = assign15680_e19799_d_n10;
        locals.var_costi7_dn11 = assign15680_e19799_d_n11;
        locals.var_costi7_dn12 = assign15680_e19799_d_n12;
        locals.var_costi7_rv = 0.0;

        let (assign15690_e19803, assign15690_e19803_d_n0, assign15690_e19803_d_n2, assign15690_e19803_d_n4, assign15690_e19803_d_n5, assign15690_e19803_d_n6, assign15690_e19803_d_n8, assign15690_e19803_d_n10, assign15690_e19803_d_n11, assign15690_e19803_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn8, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15690_e19803;
        locals.var_t1_dn0 = assign15690_e19803_d_n0;
        locals.var_t1_dn2 = assign15690_e19803_d_n2;
        locals.var_t1_dn4 = assign15690_e19803_d_n4;
        locals.var_t1_dn5 = assign15690_e19803_d_n5;
        locals.var_t1_dn6 = assign15690_e19803_d_n6;
        locals.var_t1_dn8 = assign15690_e19803_d_n8;
        locals.var_t1_dn10 = assign15690_e19803_d_n10;
        locals.var_t1_dn11 = assign15690_e19803_d_n11;
        locals.var_t1_dn12 = assign15690_e19803_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15700_e19815, assign15700_e19815_d_n0, assign15700_e19815_d_n2, assign15700_e19815_d_n4, assign15700_e19815_d_n5, assign15700_e19815_d_n6, assign15700_e19815_d_n8, assign15700_e19815_d_n10, assign15700_e19815_d_n11, assign15700_e19815_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15700_e19807: f64 = (locals.var_costi7 * locals.var_mu);
        let assign15700_e19809: f64 = (assign15700_e19807 * locals.var_qn0sti);
        let assign15700_e19811: f64 = (assign15700_e19809 * locals.var_ty);
        let assign15700_e19813: f64 = (assign15700_e19811 / locals.var_t1);
        (assign15700_e19813, (((((((((locals.var_costi7_dn0 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn0)) * locals.var_qn0sti) + (assign15700_e19807 * locals.var_qn0sti_dn0)) * locals.var_ty) + (assign15700_e19809 * locals.var_ty_dn0)) * locals.var_t1) - (assign15700_e19811 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn2 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn2)) * locals.var_qn0sti) + (assign15700_e19807 * locals.var_qn0sti_dn2)) * locals.var_ty) + (assign15700_e19809 * locals.var_ty_dn2)) * locals.var_t1) - (assign15700_e19811 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn4 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn4)) * locals.var_qn0sti) + (assign15700_e19807 * locals.var_qn0sti_dn4)) * locals.var_ty) + (assign15700_e19809 * locals.var_ty_dn4)) * locals.var_t1) - (assign15700_e19811 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn5 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn5)) * locals.var_qn0sti) + (assign15700_e19807 * locals.var_qn0sti_dn5)) * locals.var_ty) + (assign15700_e19809 * locals.var_ty_dn5)) * locals.var_t1) - (assign15700_e19811 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn6 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn6)) * locals.var_qn0sti) + (assign15700_e19807 * locals.var_qn0sti_dn6)) * locals.var_ty) + (assign15700_e19809 * locals.var_ty_dn6)) * locals.var_t1) - (assign15700_e19811 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn8 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn8)) * locals.var_qn0sti) + (assign15700_e19807 * locals.var_qn0sti_dn8)) * locals.var_ty) + (assign15700_e19809 * locals.var_ty_dn8)) * locals.var_t1) - (assign15700_e19811 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn10 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn10)) * locals.var_qn0sti) + (assign15700_e19807 * locals.var_qn0sti_dn10)) * locals.var_ty) + (assign15700_e19809 * locals.var_ty_dn10)) * locals.var_t1) - (assign15700_e19811 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn11 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn11)) * locals.var_qn0sti) + (assign15700_e19807 * locals.var_qn0sti_dn11)) * locals.var_ty) + (assign15700_e19809 * locals.var_ty_dn11)) * locals.var_t1) - (assign15700_e19811 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn12 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn12)) * locals.var_qn0sti) + (assign15700_e19807 * locals.var_qn0sti_dn12)) * locals.var_ty) + (assign15700_e19809 * locals.var_ty_dn12)) * locals.var_t1) - (assign15700_e19811 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn4, locals.var_idssti_dn5, locals.var_idssti_dn6, locals.var_idssti_dn8, locals.var_idssti_dn10, locals.var_idssti_dn11, locals.var_idssti_dn12,)
    }
};
        locals.var_idssti = assign15700_e19815;
        locals.var_idssti_dn0 = assign15700_e19815_d_n0;
        locals.var_idssti_dn2 = assign15700_e19815_d_n2;
        locals.var_idssti_dn4 = assign15700_e19815_d_n4;
        locals.var_idssti_dn5 = assign15700_e19815_d_n5;
        locals.var_idssti_dn6 = assign15700_e19815_d_n6;
        locals.var_idssti_dn8 = assign15700_e19815_d_n8;
        locals.var_idssti_dn10 = assign15700_e19815_d_n10;
        locals.var_idssti_dn11 = assign15700_e19815_d_n11;
        locals.var_idssti_dn12 = assign15700_e19815_d_n12;
        locals.var_idssti_rv = 0.0;

        let (assign15710_e19821, assign15710_e19821_d_n0, assign15710_e19821_d_n2, assign15710_e19821_d_n4, assign15710_e19821_d_n5, assign15710_e19821_d_n6, assign15710_e19821_d_n8, assign15710_e19821_d_n10, assign15710_e19821_d_n11, assign15710_e19821_d_n12,) = {
    if (locals.var_guard264 != 0.0) {
        let assign15710_e19819: f64 = (locals.var_ids + locals.var_idssti);
        (assign15710_e19819, (locals.var_ids_dn0 + locals.var_idssti_dn0), (locals.var_ids_dn2 + locals.var_idssti_dn2), (locals.var_ids_dn4 + locals.var_idssti_dn4), (locals.var_ids_dn5 + locals.var_idssti_dn5), (locals.var_ids_dn6 + locals.var_idssti_dn6), (locals.var_ids_dn8 + locals.var_idssti_dn8), (locals.var_ids_dn10 + locals.var_idssti_dn10), (locals.var_ids_dn11 + locals.var_idssti_dn11), (locals.var_ids_dn12 + locals.var_idssti_dn12),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn8, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12,)
    }
};
        locals.var_ids = assign15710_e19821;
        locals.var_ids_dn0 = assign15710_e19821_d_n0;
        locals.var_ids_dn2 = assign15710_e19821_d_n2;
        locals.var_ids_dn4 = assign15710_e19821_d_n4;
        locals.var_ids_dn5 = assign15710_e19821_d_n5;
        locals.var_ids_dn6 = assign15710_e19821_d_n6;
        locals.var_ids_dn8 = assign15710_e19821_d_n8;
        locals.var_ids_dn10 = assign15710_e19821_d_n10;
        locals.var_ids_dn11 = assign15710_e19821_d_n11;
        locals.var_ids_dn12 = assign15710_e19821_d_n12;
        locals.var_ids_rv = 0.0;

        let assign15720_e19828: f64 = if ((p.p20 != 0.0) && (p.p23 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard275 = assign15720_e19828;
        locals.var_guard275_rv = 0.0;

        let (assign15730_e19834, assign15730_e19834_d_n0, assign15730_e19834_d_n2, assign15730_e19834_d_n4, assign15730_e19834_d_n5, assign15730_e19834_d_n6, assign15730_e19834_d_n8, assign15730_e19834_d_n10, assign15730_e19834_d_n11, assign15730_e19834_d_n12,) = {
    if (locals.var_guard275 != 0.0) {
        let assign15730_e19832: f64 = (locals.var_vgvt * locals.var_vgvt);
        (assign15730_e19832, ((locals.var_vgvt_dn0 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn0)), ((locals.var_vgvt_dn2 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn2)), ((locals.var_vgvt_dn4 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn4)), ((locals.var_vgvt_dn5 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn5)), ((locals.var_vgvt_dn6 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn6)), ((locals.var_vgvt_dn8 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn8)), ((locals.var_vgvt_dn10 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn10)), ((locals.var_vgvt_dn11 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn11)), ((locals.var_vgvt_dn12 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn12)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn8, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12,)
    }
};
        locals.var_kusai00 = assign15730_e19834;
        locals.var_kusai00_dn0 = assign15730_e19834_d_n0;
        locals.var_kusai00_dn2 = assign15730_e19834_d_n2;
        locals.var_kusai00_dn4 = assign15730_e19834_d_n4;
        locals.var_kusai00_dn5 = assign15730_e19834_d_n5;
        locals.var_kusai00_dn6 = assign15730_e19834_d_n6;
        locals.var_kusai00_dn8 = assign15730_e19834_d_n8;
        locals.var_kusai00_dn10 = assign15730_e19834_d_n10;
        locals.var_kusai00_dn11 = assign15730_e19834_d_n11;
        locals.var_kusai00_dn12 = assign15730_e19834_d_n12;
        locals.var_kusai00_rv = 0.0;

        let (assign15740_e19844, assign15740_e19844_d_n0, assign15740_e19844_d_n2, assign15740_e19844_d_n4, assign15740_e19844_d_n5, assign15740_e19844_d_n6, assign15740_e19844_d_n8, assign15740_e19844_d_n10, assign15740_e19844_d_n11, assign15740_e19844_d_n12,) = {
    if (locals.var_guard275 != 0.0) {
        let assign15740_e19838: f64 = (2.0 * locals.var_beta_inv);
        let assign15740_e19840: f64 = (assign15740_e19838 * locals.var_c_fox_inv);
        let assign15740_e19842: f64 = (assign15740_e19840 * locals.var_idd);
        (assign15740_e19842, (((assign15740_e19838 * locals.var_c_fox_inv_dn0) * locals.var_idd) + (assign15740_e19840 * locals.var_idd_dn0)), (((assign15740_e19838 * locals.var_c_fox_inv_dn2) * locals.var_idd) + (assign15740_e19840 * locals.var_idd_dn2)), (((((2.0 * locals.var_beta_inv_dn4) * locals.var_c_fox_inv) + (assign15740_e19838 * locals.var_c_fox_inv_dn4)) * locals.var_idd) + (assign15740_e19840 * locals.var_idd_dn4)), (((assign15740_e19838 * locals.var_c_fox_inv_dn5) * locals.var_idd) + (assign15740_e19840 * locals.var_idd_dn5)), (((assign15740_e19838 * locals.var_c_fox_inv_dn6) * locals.var_idd) + (assign15740_e19840 * locals.var_idd_dn6)), (((assign15740_e19838 * locals.var_c_fox_inv_dn8) * locals.var_idd) + (assign15740_e19840 * locals.var_idd_dn8)), (((assign15740_e19838 * locals.var_c_fox_inv_dn10) * locals.var_idd) + (assign15740_e19840 * locals.var_idd_dn10)), (((assign15740_e19838 * locals.var_c_fox_inv_dn11) * locals.var_idd) + (assign15740_e19840 * locals.var_idd_dn11)), (((assign15740_e19838 * locals.var_c_fox_inv_dn12) * locals.var_idd) + (assign15740_e19840 * locals.var_idd_dn12)),)
    } else {
        (locals.var_kusaidd, locals.var_kusaidd_dn0, locals.var_kusaidd_dn2, locals.var_kusaidd_dn4, locals.var_kusaidd_dn5, locals.var_kusaidd_dn6, locals.var_kusaidd_dn8, locals.var_kusaidd_dn10, locals.var_kusaidd_dn11, locals.var_kusaidd_dn12,)
    }
};
        locals.var_kusaidd = assign15740_e19844;
        locals.var_kusaidd_dn0 = assign15740_e19844_d_n0;
        locals.var_kusaidd_dn2 = assign15740_e19844_d_n2;
        locals.var_kusaidd_dn4 = assign15740_e19844_d_n4;
        locals.var_kusaidd_dn5 = assign15740_e19844_d_n5;
        locals.var_kusaidd_dn6 = assign15740_e19844_d_n6;
        locals.var_kusaidd_dn8 = assign15740_e19844_d_n8;
        locals.var_kusaidd_dn10 = assign15740_e19844_d_n10;
        locals.var_kusaidd_dn11 = assign15740_e19844_d_n11;
        locals.var_kusaidd_dn12 = assign15740_e19844_d_n12;
        locals.var_kusaidd_rv = 0.0;

        let (assign15750_e19850, assign15750_e19850_d_n0, assign15750_e19850_d_n2, assign15750_e19850_d_n4, assign15750_e19850_d_n5, assign15750_e19850_d_n6, assign15750_e19850_d_n8, assign15750_e19850_d_n10, assign15750_e19850_d_n11, assign15750_e19850_d_n12,) = {
    if (locals.var_guard275 != 0.0) {
        let assign15750_e19848: f64 = (locals.var_kusai00 - locals.var_kusaidd);
        (assign15750_e19848, (locals.var_kusai00_dn0 - locals.var_kusaidd_dn0), (locals.var_kusai00_dn2 - locals.var_kusaidd_dn2), (locals.var_kusai00_dn4 - locals.var_kusaidd_dn4), (locals.var_kusai00_dn5 - locals.var_kusaidd_dn5), (locals.var_kusai00_dn6 - locals.var_kusaidd_dn6), (locals.var_kusai00_dn8 - locals.var_kusaidd_dn8), (locals.var_kusai00_dn10 - locals.var_kusaidd_dn10), (locals.var_kusai00_dn11 - locals.var_kusaidd_dn11), (locals.var_kusai00_dn12 - locals.var_kusaidd_dn12),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn8, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12,)
    }
};
        locals.var_kusail = assign15750_e19850;
        locals.var_kusail_dn0 = assign15750_e19850_d_n0;
        locals.var_kusail_dn2 = assign15750_e19850_d_n2;
        locals.var_kusail_dn4 = assign15750_e19850_d_n4;
        locals.var_kusail_dn5 = assign15750_e19850_d_n5;
        locals.var_kusail_dn6 = assign15750_e19850_d_n6;
        locals.var_kusail_dn8 = assign15750_e19850_d_n8;
        locals.var_kusail_dn10 = assign15750_e19850_d_n10;
        locals.var_kusail_dn11 = assign15750_e19850_d_n11;
        locals.var_kusail_dn12 = assign15750_e19850_d_n12;
        locals.var_kusail_rv = 0.0;

        let (assign15760_e19863, assign15760_e19863_d_n0, assign15760_e19863_d_n2, assign15760_e19863_d_n4, assign15760_e19863_d_n5, assign15760_e19863_d_n6, assign15760_e19863_d_n8, assign15760_e19863_d_n10, assign15760_e19863_d_n11, assign15760_e19863_d_n12,) = {
    if (locals.var_guard275 != 0.0) {
        let assign15760_e19854: f64 = (locals.var_kusai00 * locals.var_kusai00);
        let assign15760_e19857: f64 = (4.0 * 0.001);
        let assign15760_e19859: f64 = (assign15760_e19857 * 0.001);
        let assign15760_e19860: f64 = (assign15760_e19854 + assign15760_e19859);
        let assign15760_e19861: f64 = (assign15760_e19860).sqrt();
        (assign15760_e19861, (((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)) / (2.0 * assign15760_e19861)), (((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)) / (2.0 * assign15760_e19861)), (((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)) / (2.0 * assign15760_e19861)), (((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)) / (2.0 * assign15760_e19861)), (((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)) / (2.0 * assign15760_e19861)), (((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)) / (2.0 * assign15760_e19861)), (((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)) / (2.0 * assign15760_e19861)), (((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)) / (2.0 * assign15760_e19861)), (((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)) / (2.0 * assign15760_e19861)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign15760_e19863;
        locals.var_tmf2_dn0 = assign15760_e19863_d_n0;
        locals.var_tmf2_dn2 = assign15760_e19863_d_n2;
        locals.var_tmf2_dn4 = assign15760_e19863_d_n4;
        locals.var_tmf2_dn5 = assign15760_e19863_d_n5;
        locals.var_tmf2_dn6 = assign15760_e19863_d_n6;
        locals.var_tmf2_dn8 = assign15760_e19863_d_n8;
        locals.var_tmf2_dn10 = assign15760_e19863_d_n10;
        locals.var_tmf2_dn11 = assign15760_e19863_d_n11;
        locals.var_tmf2_dn12 = assign15760_e19863_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign15770_e19873, assign15770_e19873_d_n0, assign15770_e19873_d_n2, assign15770_e19873_d_n4, assign15770_e19873_d_n5, assign15770_e19873_d_n6, assign15770_e19873_d_n8, assign15770_e19873_d_n10, assign15770_e19873_d_n11, assign15770_e19873_d_n12,) = {
    if (locals.var_guard275 != 0.0) {
        let assign15770_e19869: f64 = (locals.var_kusai00 / locals.var_tmf2);
        let assign15770_e19870: f64 = (1.0 + assign15770_e19869);
        let assign15770_e19871: f64 = (0.5 * assign15770_e19870);
        (assign15770_e19871, (0.5 * (((locals.var_kusai00_dn0 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn2 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn4 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn5 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn6 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn8 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn10 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn11 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn12 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15770_e19873;
        locals.var_t0_dn0 = assign15770_e19873_d_n0;
        locals.var_t0_dn2 = assign15770_e19873_d_n2;
        locals.var_t0_dn4 = assign15770_e19873_d_n4;
        locals.var_t0_dn5 = assign15770_e19873_d_n5;
        locals.var_t0_dn6 = assign15770_e19873_d_n6;
        locals.var_t0_dn8 = assign15770_e19873_d_n8;
        locals.var_t0_dn10 = assign15770_e19873_d_n10;
        locals.var_t0_dn11 = assign15770_e19873_d_n11;
        locals.var_t0_dn12 = assign15770_e19873_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign15780_e19885, assign15780_e19885_d_n0, assign15780_e19885_d_n2, assign15780_e19885_d_n4, assign15780_e19885_d_n5, assign15780_e19885_d_n6, assign15780_e19885_d_n8, assign15780_e19885_d_n10, assign15780_e19885_d_n11, assign15780_e19885_d_n12,) = {
    if (locals.var_guard275 != 0.0) {
        let assign15780_e19878: f64 = (locals.var_kusai00 + locals.var_tmf2);
        let assign15780_e19879: f64 = (0.5 * assign15780_e19878);
        let assign15780_e19882: f64 = (1e-10 * 0.001);
        let assign15780_e19883: f64 = (assign15780_e19879 + assign15780_e19882);
        (assign15780_e19883, (0.5 * (locals.var_kusai00_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_kusai00_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_kusai00_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_kusai00_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_kusai00_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_kusai00_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_kusai00_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_kusai00_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_kusai00_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn8, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12,)
    }
};
        locals.var_kusai00 = assign15780_e19885;
        locals.var_kusai00_dn0 = assign15780_e19885_d_n0;
        locals.var_kusai00_dn2 = assign15780_e19885_d_n2;
        locals.var_kusai00_dn4 = assign15780_e19885_d_n4;
        locals.var_kusai00_dn5 = assign15780_e19885_d_n5;
        locals.var_kusai00_dn6 = assign15780_e19885_d_n6;
        locals.var_kusai00_dn8 = assign15780_e19885_d_n8;
        locals.var_kusai00_dn10 = assign15780_e19885_d_n10;
        locals.var_kusai00_dn11 = assign15780_e19885_d_n11;
        locals.var_kusai00_dn12 = assign15780_e19885_d_n12;
        locals.var_kusai00_rv = 0.0;

        let assign15790_e19888: f64 = if locals.var_kusai00 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign15790_e19888;
        locals.var_guard276_rv = 0.0;

        let (assign15800_e19894, assign15800_e19894_d_n0, assign15800_e19894_d_n2, assign15800_e19894_d_n4, assign15800_e19894_d_n5, assign15800_e19894_d_n6, assign15800_e19894_d_n8, assign15800_e19894_d_n10, assign15800_e19894_d_n11, assign15800_e19894_d_n12,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn8, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn12,)
    }
};
        locals.var_kusai00 = assign15800_e19894;
        locals.var_kusai00_dn0 = assign15800_e19894_d_n0;
        locals.var_kusai00_dn2 = assign15800_e19894_d_n2;
        locals.var_kusai00_dn4 = assign15800_e19894_d_n4;
        locals.var_kusai00_dn5 = assign15800_e19894_d_n5;
        locals.var_kusai00_dn6 = assign15800_e19894_d_n6;
        locals.var_kusai00_dn8 = assign15800_e19894_d_n8;
        locals.var_kusai00_dn10 = assign15800_e19894_d_n10;
        locals.var_kusai00_dn11 = assign15800_e19894_d_n11;
        locals.var_kusai00_dn12 = assign15800_e19894_d_n12;
        locals.var_kusai00_rv = 0.0;

        let (assign15810_e19900, assign15810_e19900_d_n0, assign15810_e19900_d_n2, assign15810_e19900_d_n4, assign15810_e19900_d_n5, assign15810_e19900_d_n6, assign15810_e19900_d_n8, assign15810_e19900_d_n10, assign15810_e19900_d_n11, assign15810_e19900_d_n12,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15810_e19900;
        locals.var_t0_dn0 = assign15810_e19900_d_n0;
        locals.var_t0_dn2 = assign15810_e19900_d_n2;
        locals.var_t0_dn4 = assign15810_e19900_d_n4;
        locals.var_t0_dn5 = assign15810_e19900_d_n5;
        locals.var_t0_dn6 = assign15810_e19900_d_n6;
        locals.var_t0_dn8 = assign15810_e19900_d_n8;
        locals.var_t0_dn10 = assign15810_e19900_d_n10;
        locals.var_t0_dn11 = assign15810_e19900_d_n11;
        locals.var_t0_dn12 = assign15810_e19900_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign15820_e19913, assign15820_e19913_d_n0, assign15820_e19913_d_n2, assign15820_e19913_d_n4, assign15820_e19913_d_n5, assign15820_e19913_d_n6, assign15820_e19913_d_n8, assign15820_e19913_d_n10, assign15820_e19913_d_n11, assign15820_e19913_d_n12,) = {
    if (locals.var_guard275 != 0.0) {
        let assign15820_e19904: f64 = (locals.var_kusail * locals.var_kusail);
        let assign15820_e19907: f64 = (4.0 * 0.001);
        let assign15820_e19909: f64 = (assign15820_e19907 * 0.001);
        let assign15820_e19910: f64 = (assign15820_e19904 + assign15820_e19909);
        let assign15820_e19911: f64 = (assign15820_e19910).sqrt();
        (assign15820_e19911, (((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)) / (2.0 * assign15820_e19911)), (((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)) / (2.0 * assign15820_e19911)), (((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)) / (2.0 * assign15820_e19911)), (((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)) / (2.0 * assign15820_e19911)), (((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)) / (2.0 * assign15820_e19911)), (((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)) / (2.0 * assign15820_e19911)), (((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)) / (2.0 * assign15820_e19911)), (((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)) / (2.0 * assign15820_e19911)), (((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)) / (2.0 * assign15820_e19911)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign15820_e19913;
        locals.var_tmf2_dn0 = assign15820_e19913_d_n0;
        locals.var_tmf2_dn2 = assign15820_e19913_d_n2;
        locals.var_tmf2_dn4 = assign15820_e19913_d_n4;
        locals.var_tmf2_dn5 = assign15820_e19913_d_n5;
        locals.var_tmf2_dn6 = assign15820_e19913_d_n6;
        locals.var_tmf2_dn8 = assign15820_e19913_d_n8;
        locals.var_tmf2_dn10 = assign15820_e19913_d_n10;
        locals.var_tmf2_dn11 = assign15820_e19913_d_n11;
        locals.var_tmf2_dn12 = assign15820_e19913_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign15830_e19923, assign15830_e19923_d_n0, assign15830_e19923_d_n2, assign15830_e19923_d_n4, assign15830_e19923_d_n5, assign15830_e19923_d_n6, assign15830_e19923_d_n8, assign15830_e19923_d_n10, assign15830_e19923_d_n11, assign15830_e19923_d_n12,) = {
    if (locals.var_guard275 != 0.0) {
        let assign15830_e19919: f64 = (locals.var_kusail / locals.var_tmf2);
        let assign15830_e19920: f64 = (1.0 + assign15830_e19919);
        let assign15830_e19921: f64 = (0.5 * assign15830_e19920);
        (assign15830_e19921, (0.5 * (((locals.var_kusail_dn0 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn2 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn4 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn5 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn6 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn8 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn10 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn11 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn12 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15830_e19923;
        locals.var_t0_dn0 = assign15830_e19923_d_n0;
        locals.var_t0_dn2 = assign15830_e19923_d_n2;
        locals.var_t0_dn4 = assign15830_e19923_d_n4;
        locals.var_t0_dn5 = assign15830_e19923_d_n5;
        locals.var_t0_dn6 = assign15830_e19923_d_n6;
        locals.var_t0_dn8 = assign15830_e19923_d_n8;
        locals.var_t0_dn10 = assign15830_e19923_d_n10;
        locals.var_t0_dn11 = assign15830_e19923_d_n11;
        locals.var_t0_dn12 = assign15830_e19923_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign15840_e19935, assign15840_e19935_d_n0, assign15840_e19935_d_n2, assign15840_e19935_d_n4, assign15840_e19935_d_n5, assign15840_e19935_d_n6, assign15840_e19935_d_n8, assign15840_e19935_d_n10, assign15840_e19935_d_n11, assign15840_e19935_d_n12,) = {
    if (locals.var_guard275 != 0.0) {
        let assign15840_e19928: f64 = (locals.var_kusail + locals.var_tmf2);
        let assign15840_e19929: f64 = (0.5 * assign15840_e19928);
        let assign15840_e19932: f64 = (1e-10 * 0.001);
        let assign15840_e19933: f64 = (assign15840_e19929 + assign15840_e19932);
        (assign15840_e19933, (0.5 * (locals.var_kusail_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_kusail_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_kusail_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_kusail_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_kusail_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_kusail_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_kusail_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_kusail_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_kusail_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn8, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12,)
    }
};
        locals.var_kusail = assign15840_e19935;
        locals.var_kusail_dn0 = assign15840_e19935_d_n0;
        locals.var_kusail_dn2 = assign15840_e19935_d_n2;
        locals.var_kusail_dn4 = assign15840_e19935_d_n4;
        locals.var_kusail_dn5 = assign15840_e19935_d_n5;
        locals.var_kusail_dn6 = assign15840_e19935_d_n6;
        locals.var_kusail_dn8 = assign15840_e19935_d_n8;
        locals.var_kusail_dn10 = assign15840_e19935_d_n10;
        locals.var_kusail_dn11 = assign15840_e19935_d_n11;
        locals.var_kusail_dn12 = assign15840_e19935_d_n12;
        locals.var_kusail_rv = 0.0;

        let assign15850_e19938: f64 = if locals.var_kusail < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign15850_e19938;
        locals.var_guard277_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15860_e19944, assign15860_e19944_d_n0, assign15860_e19944_d_n2, assign15860_e19944_d_n4, assign15860_e19944_d_n5, assign15860_e19944_d_n6, assign15860_e19944_d_n8, assign15860_e19944_d_n10, assign15860_e19944_d_n11, assign15860_e19944_d_n12,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard277 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn8, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn12,)
    }
};
        locals.var_kusail = assign15860_e19944;
        locals.var_kusail_dn0 = assign15860_e19944_d_n0;
        locals.var_kusail_dn2 = assign15860_e19944_d_n2;
        locals.var_kusail_dn4 = assign15860_e19944_d_n4;
        locals.var_kusail_dn5 = assign15860_e19944_d_n5;
        locals.var_kusail_dn6 = assign15860_e19944_d_n6;
        locals.var_kusail_dn8 = assign15860_e19944_d_n8;
        locals.var_kusail_dn10 = assign15860_e19944_d_n10;
        locals.var_kusail_dn11 = assign15860_e19944_d_n11;
        locals.var_kusail_dn12 = assign15860_e19944_d_n12;
        locals.var_kusail_rv = 0.0;

        let (assign15870_e19950, assign15870_e19950_d_n0, assign15870_e19950_d_n2, assign15870_e19950_d_n4, assign15870_e19950_d_n5, assign15870_e19950_d_n6, assign15870_e19950_d_n8, assign15870_e19950_d_n10, assign15870_e19950_d_n11, assign15870_e19950_d_n12,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard277 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign15870_e19950;
        locals.var_t0_dn0 = assign15870_e19950_d_n0;
        locals.var_t0_dn2 = assign15870_e19950_d_n2;
        locals.var_t0_dn4 = assign15870_e19950_d_n4;
        locals.var_t0_dn5 = assign15870_e19950_d_n5;
        locals.var_t0_dn6 = assign15870_e19950_d_n6;
        locals.var_t0_dn8 = assign15870_e19950_d_n8;
        locals.var_t0_dn10 = assign15870_e19950_d_n10;
        locals.var_t0_dn11 = assign15870_e19950_d_n11;
        locals.var_t0_dn12 = assign15870_e19950_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign15880_e19956, assign15880_e19956_d_n0, assign15880_e19956_d_n2, assign15880_e19956_d_n4, assign15880_e19956_d_n5, assign15880_e19956_d_n6, assign15880_e19956_d_n8, assign15880_e19956_d_n10, assign15880_e19956_d_n11, assign15880_e19956_d_n12,) = {
    if (locals.var_guard275 != 0.0) {
        let assign15880_e19954: f64 = (locals.var_kusai00 - locals.var_kusail);
        (assign15880_e19954, (locals.var_kusai00_dn0 - locals.var_kusail_dn0), (locals.var_kusai00_dn2 - locals.var_kusail_dn2), (locals.var_kusai00_dn4 - locals.var_kusail_dn4), (locals.var_kusai00_dn5 - locals.var_kusail_dn5), (locals.var_kusai00_dn6 - locals.var_kusail_dn6), (locals.var_kusai00_dn8 - locals.var_kusail_dn8), (locals.var_kusai00_dn10 - locals.var_kusail_dn10), (locals.var_kusai00_dn11 - locals.var_kusail_dn11), (locals.var_kusai00_dn12 - locals.var_kusail_dn12),)
    } else {
        (locals.var_kusai00l, locals.var_kusai00l_dn0, locals.var_kusai00l_dn2, locals.var_kusai00l_dn4, locals.var_kusai00l_dn5, locals.var_kusai00l_dn6, locals.var_kusai00l_dn8, locals.var_kusai00l_dn10, locals.var_kusai00l_dn11, locals.var_kusai00l_dn12,)
    }
};
        locals.var_kusai00l = assign15880_e19956;
        locals.var_kusai00l_dn0 = assign15880_e19956_d_n0;
        locals.var_kusai00l_dn2 = assign15880_e19956_d_n2;
        locals.var_kusai00l_dn4 = assign15880_e19956_d_n4;
        locals.var_kusai00l_dn5 = assign15880_e19956_d_n5;
        locals.var_kusai00l_dn6 = assign15880_e19956_d_n6;
        locals.var_kusai00l_dn8 = assign15880_e19956_d_n8;
        locals.var_kusai00l_dn10 = assign15880_e19956_d_n10;
        locals.var_kusai00l_dn11 = assign15880_e19956_d_n11;
        locals.var_kusai00l_dn12 = assign15880_e19956_d_n12;
        locals.var_kusai00l_rv = 0.0;

        let assign15890_e19960: f64 = (10.0 * 2.220446049250313e-16);
        let assign15890_e19965: f64 = (10.0 * 2.220446049250313e-16);
        let assign15890_e19967: f64 = if ((locals.var_qn0 < assign15890_e19960) || (locals.var_kusai00l < assign15890_e19965)) { 1.0 } else { 0.0 };
        locals.var_guard278 = assign15890_e19967;
        locals.var_guard278_rv = 0.0;

        let (assign15900_e19973,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard278 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign15900_e19973;
        locals.var_flg_ign_rv = 0.0;

        let (assign15910_e19980,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard278 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign15910_e19980;
        locals.var_flg_ign_rv = 0.0;

        let assign15920_e19983: f64 = if locals.var_isub > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign15920_e19983;
        locals.var_guard279_rv = 0.0;

        let (assign15930_e19987, assign15930_e19987_d_n0, assign15930_e19987_d_n2, assign15930_e19987_d_n4, assign15930_e19987_d_n5, assign15930_e19987_d_n6, assign15930_e19987_d_n8, assign15930_e19987_d_n10, assign15930_e19987_d_n11, assign15930_e19987_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        (locals.var_vgpsub, locals.var_vgpsub_dn0, locals.var_vgpsub_dn2, locals.var_vgpsub_dn4, locals.var_vgpsub_dn5, locals.var_vgpsub_dn6, locals.var_vgpsub_dn8, locals.var_vgpsub_dn10, locals.var_vgpsub_dn11, locals.var_vgpsub_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign15930_e19987;
        locals.var_t1_dn0 = assign15930_e19987_d_n0;
        locals.var_t1_dn2 = assign15930_e19987_d_n2;
        locals.var_t1_dn4 = assign15930_e19987_d_n4;
        locals.var_t1_dn5 = assign15930_e19987_d_n5;
        locals.var_t1_dn6 = assign15930_e19987_d_n6;
        locals.var_t1_dn8 = assign15930_e19987_d_n8;
        locals.var_t1_dn10 = assign15930_e19987_d_n10;
        locals.var_t1_dn11 = assign15930_e19987_d_n11;
        locals.var_t1_dn12 = assign15930_e19987_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign15940_e19993, assign15940_e19993_d_n0, assign15940_e19993_d_n2, assign15940_e19993_d_n4, assign15940_e19993_d_n5, assign15940_e19993_d_n6, assign15940_e19993_d_n8, assign15940_e19993_d_n10, assign15940_e19993_d_n11, assign15940_e19993_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign15940_e19991: f64 = (locals.var_c_fox * locals.var_c_fox);
        (assign15940_e19991, ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)), ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)), ((locals.var_c_fox_dn4 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn4)), ((locals.var_c_fox_dn5 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn5)), ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)), ((locals.var_c_fox_dn8 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn8)), ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)), ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)), ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign15940_e19993;
        locals.var_t7_dn0 = assign15940_e19993_d_n0;
        locals.var_t7_dn2 = assign15940_e19993_d_n2;
        locals.var_t7_dn4 = assign15940_e19993_d_n4;
        locals.var_t7_dn5 = assign15940_e19993_d_n5;
        locals.var_t7_dn6 = assign15940_e19993_d_n6;
        locals.var_t7_dn8 = assign15940_e19993_d_n8;
        locals.var_t7_dn10 = assign15940_e19993_d_n10;
        locals.var_t7_dn11 = assign15940_e19993_d_n11;
        locals.var_t7_dn12 = assign15940_e19993_d_n12;
        locals.var_t7_rv = 0.0;

        let (assign15950_e20001, assign15950_e20001_d_n0, assign15950_e20001_d_n2, assign15950_e20001_d_n4, assign15950_e20001_d_n5, assign15950_e20001_d_n6, assign15950_e20001_d_n8, assign15950_e20001_d_n10, assign15950_e20001_d_n11, assign15950_e20001_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign15950_e19997: f64 = (2.0 / locals.var_qnsub_esi);
        let assign15950_e19999: f64 = (assign15950_e19997 * locals.var_t7);
        (assign15950_e19999, (((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * locals.var_t7) + (assign15950_e19997 * locals.var_t7_dn0)), (((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * locals.var_t7) + (assign15950_e19997 * locals.var_t7_dn2)), (((-((2.0 * locals.var_qnsub_esi_dn4) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * locals.var_t7) + (assign15950_e19997 * locals.var_t7_dn4)), (((-((2.0 * locals.var_qnsub_esi_dn5) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * locals.var_t7) + (assign15950_e19997 * locals.var_t7_dn5)), (((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * locals.var_t7) + (assign15950_e19997 * locals.var_t7_dn6)), (((-((2.0 * locals.var_qnsub_esi_dn8) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * locals.var_t7) + (assign15950_e19997 * locals.var_t7_dn8)), (((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * locals.var_t7) + (assign15950_e19997 * locals.var_t7_dn10)), (((-((2.0 * locals.var_qnsub_esi_dn11) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * locals.var_t7) + (assign15950_e19997 * locals.var_t7_dn11)), (((-((2.0 * locals.var_qnsub_esi_dn12) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * locals.var_t7) + (assign15950_e19997 * locals.var_t7_dn12)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign15950_e20001;
        locals.var_t4_dn0 = assign15950_e20001_d_n0;
        locals.var_t4_dn2 = assign15950_e20001_d_n2;
        locals.var_t4_dn4 = assign15950_e20001_d_n4;
        locals.var_t4_dn5 = assign15950_e20001_d_n5;
        locals.var_t4_dn6 = assign15950_e20001_d_n6;
        locals.var_t4_dn8 = assign15950_e20001_d_n8;
        locals.var_t4_dn10 = assign15950_e20001_d_n10;
        locals.var_t4_dn11 = assign15950_e20001_d_n11;
        locals.var_t4_dn12 = assign15950_e20001_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign15960_e20011, assign15960_e20011_d_n0, assign15960_e20011_d_n2, assign15960_e20011_d_n4, assign15960_e20011_d_n5, assign15960_e20011_d_n6, assign15960_e20011_d_n8, assign15960_e20011_d_n10, assign15960_e20011_d_n11, assign15960_e20011_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign15960_e20005: f64 = (locals.var_t1 - locals.var_beta_inv);
        let assign15960_e20008: f64 = (locals.var_xvbs * locals.var_vbsz);
        let assign15960_e20009: f64 = (assign15960_e20005 - assign15960_e20008);
        (assign15960_e20009, (locals.var_t1_dn0 - (locals.var_xvbs * locals.var_vbsz_dn0)), (locals.var_t1_dn2 - (locals.var_xvbs * locals.var_vbsz_dn2)), ((locals.var_t1_dn4 - locals.var_beta_inv_dn4) - (locals.var_xvbs * locals.var_vbsz_dn4)), (locals.var_t1_dn5 - (locals.var_xvbs * locals.var_vbsz_dn5)), (locals.var_t1_dn6 - (locals.var_xvbs * locals.var_vbsz_dn6)), (locals.var_t1_dn8 - (locals.var_xvbs * locals.var_vbsz_dn8)), (locals.var_t1_dn10 - (locals.var_xvbs * locals.var_vbsz_dn10)), (locals.var_t1_dn11 - (locals.var_xvbs * locals.var_vbsz_dn11)), (locals.var_t1_dn12 - (locals.var_xvbs * locals.var_vbsz_dn12)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign15960_e20011;
        locals.var_t5_dn0 = assign15960_e20011_d_n0;
        locals.var_t5_dn2 = assign15960_e20011_d_n2;
        locals.var_t5_dn4 = assign15960_e20011_d_n4;
        locals.var_t5_dn5 = assign15960_e20011_d_n5;
        locals.var_t5_dn6 = assign15960_e20011_d_n6;
        locals.var_t5_dn8 = assign15960_e20011_d_n8;
        locals.var_t5_dn10 = assign15960_e20011_d_n10;
        locals.var_t5_dn11 = assign15960_e20011_d_n11;
        locals.var_t5_dn12 = assign15960_e20011_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign15970_e20019, assign15970_e20019_d_n0, assign15970_e20019_d_n2, assign15970_e20019_d_n4, assign15970_e20019_d_n5, assign15970_e20019_d_n6, assign15970_e20019_d_n8, assign15970_e20019_d_n10, assign15970_e20019_d_n11, assign15970_e20019_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign15970_e20016: f64 = (locals.var_t4 * locals.var_t5);
        let assign15970_e20017: f64 = (1.0 + assign15970_e20016);
        (assign15970_e20017, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn12 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn12)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign15970_e20019;
        locals.var_t6_dn0 = assign15970_e20019_d_n0;
        locals.var_t6_dn2 = assign15970_e20019_d_n2;
        locals.var_t6_dn4 = assign15970_e20019_d_n4;
        locals.var_t6_dn5 = assign15970_e20019_d_n5;
        locals.var_t6_dn6 = assign15970_e20019_d_n6;
        locals.var_t6_dn8 = assign15970_e20019_d_n8;
        locals.var_t6_dn10 = assign15970_e20019_d_n10;
        locals.var_t6_dn11 = assign15970_e20019_d_n11;
        locals.var_t6_dn12 = assign15970_e20019_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign15980_e20032, assign15980_e20032_d_n0, assign15980_e20032_d_n2, assign15980_e20032_d_n4, assign15980_e20032_d_n5, assign15980_e20032_d_n6, assign15980_e20032_d_n8, assign15980_e20032_d_n10, assign15980_e20032_d_n11, assign15980_e20032_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign15980_e20023: f64 = (locals.var_t6 * locals.var_t6);
        let assign15980_e20026: f64 = (4.0 * 0.001);
        let assign15980_e20028: f64 = (assign15980_e20026 * 0.001);
        let assign15980_e20029: f64 = (assign15980_e20023 + assign15980_e20028);
        let assign15980_e20030: f64 = (assign15980_e20029).sqrt();
        (assign15980_e20030, (((locals.var_t6_dn0 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn0)) / (2.0 * assign15980_e20030)), (((locals.var_t6_dn2 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn2)) / (2.0 * assign15980_e20030)), (((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) / (2.0 * assign15980_e20030)), (((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) / (2.0 * assign15980_e20030)), (((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / (2.0 * assign15980_e20030)), (((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) / (2.0 * assign15980_e20030)), (((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / (2.0 * assign15980_e20030)), (((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / (2.0 * assign15980_e20030)), (((locals.var_t6_dn12 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn12)) / (2.0 * assign15980_e20030)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign15980_e20032;
        locals.var_tmf2_dn0 = assign15980_e20032_d_n0;
        locals.var_tmf2_dn2 = assign15980_e20032_d_n2;
        locals.var_tmf2_dn4 = assign15980_e20032_d_n4;
        locals.var_tmf2_dn5 = assign15980_e20032_d_n5;
        locals.var_tmf2_dn6 = assign15980_e20032_d_n6;
        locals.var_tmf2_dn8 = assign15980_e20032_d_n8;
        locals.var_tmf2_dn10 = assign15980_e20032_d_n10;
        locals.var_tmf2_dn11 = assign15980_e20032_d_n11;
        locals.var_tmf2_dn12 = assign15980_e20032_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign15990_e20042, assign15990_e20042_d_n0, assign15990_e20042_d_n2, assign15990_e20042_d_n4, assign15990_e20042_d_n5, assign15990_e20042_d_n6, assign15990_e20042_d_n8, assign15990_e20042_d_n10, assign15990_e20042_d_n11, assign15990_e20042_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign15990_e20038: f64 = (locals.var_t6 / locals.var_tmf2);
        let assign15990_e20039: f64 = (1.0 + assign15990_e20038);
        let assign15990_e20040: f64 = (0.5 * assign15990_e20039);
        (assign15990_e20040, (0.5 * (((locals.var_t6_dn0 * locals.var_tmf2) - (locals.var_t6 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t6_dn2 * locals.var_tmf2) - (locals.var_t6 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t6_dn4 * locals.var_tmf2) - (locals.var_t6 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t6_dn5 * locals.var_tmf2) - (locals.var_t6 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t6_dn6 * locals.var_tmf2) - (locals.var_t6 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t6_dn8 * locals.var_tmf2) - (locals.var_t6 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t6_dn10 * locals.var_tmf2) - (locals.var_t6 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t6_dn11 * locals.var_tmf2) - (locals.var_t6 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t6_dn12 * locals.var_tmf2) - (locals.var_t6 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign15990_e20042;
        locals.var_t9_dn0 = assign15990_e20042_d_n0;
        locals.var_t9_dn2 = assign15990_e20042_d_n2;
        locals.var_t9_dn4 = assign15990_e20042_d_n4;
        locals.var_t9_dn5 = assign15990_e20042_d_n5;
        locals.var_t9_dn6 = assign15990_e20042_d_n6;
        locals.var_t9_dn8 = assign15990_e20042_d_n8;
        locals.var_t9_dn10 = assign15990_e20042_d_n10;
        locals.var_t9_dn11 = assign15990_e20042_d_n11;
        locals.var_t9_dn12 = assign15990_e20042_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign16000_e20054, assign16000_e20054_d_n0, assign16000_e20054_d_n2, assign16000_e20054_d_n4, assign16000_e20054_d_n5, assign16000_e20054_d_n6, assign16000_e20054_d_n8, assign16000_e20054_d_n10, assign16000_e20054_d_n11, assign16000_e20054_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign16000_e20047: f64 = (locals.var_t6 + locals.var_tmf2);
        let assign16000_e20048: f64 = (0.5 * assign16000_e20047);
        let assign16000_e20051: f64 = (1e-10 * 0.001);
        let assign16000_e20052: f64 = (assign16000_e20048 + assign16000_e20051);
        (assign16000_e20052, (0.5 * (locals.var_t6_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t6_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t6_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t6_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t6_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t6_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t6_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t6_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t6_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign16000_e20054;
        locals.var_t6_dn0 = assign16000_e20054_d_n0;
        locals.var_t6_dn2 = assign16000_e20054_d_n2;
        locals.var_t6_dn4 = assign16000_e20054_d_n4;
        locals.var_t6_dn5 = assign16000_e20054_d_n5;
        locals.var_t6_dn6 = assign16000_e20054_d_n6;
        locals.var_t6_dn8 = assign16000_e20054_d_n8;
        locals.var_t6_dn10 = assign16000_e20054_d_n10;
        locals.var_t6_dn11 = assign16000_e20054_d_n11;
        locals.var_t6_dn12 = assign16000_e20054_d_n12;
        locals.var_t6_rv = 0.0;

        let assign16010_e20057: f64 = if locals.var_t6 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign16010_e20057;
        locals.var_guard280_rv = 0.0;

        let (assign16020_e20063, assign16020_e20063_d_n0, assign16020_e20063_d_n2, assign16020_e20063_d_n4, assign16020_e20063_d_n5, assign16020_e20063_d_n6, assign16020_e20063_d_n8, assign16020_e20063_d_n10, assign16020_e20063_d_n11, assign16020_e20063_d_n12,) = {
    if ((locals.var_guard279 != 0.0) && (locals.var_guard280 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign16020_e20063;
        locals.var_t6_dn0 = assign16020_e20063_d_n0;
        locals.var_t6_dn2 = assign16020_e20063_d_n2;
        locals.var_t6_dn4 = assign16020_e20063_d_n4;
        locals.var_t6_dn5 = assign16020_e20063_d_n5;
        locals.var_t6_dn6 = assign16020_e20063_d_n6;
        locals.var_t6_dn8 = assign16020_e20063_d_n8;
        locals.var_t6_dn10 = assign16020_e20063_d_n10;
        locals.var_t6_dn11 = assign16020_e20063_d_n11;
        locals.var_t6_dn12 = assign16020_e20063_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign16030_e20069, assign16030_e20069_d_n0, assign16030_e20069_d_n2, assign16030_e20069_d_n4, assign16030_e20069_d_n5, assign16030_e20069_d_n6, assign16030_e20069_d_n8, assign16030_e20069_d_n10, assign16030_e20069_d_n11, assign16030_e20069_d_n12,) = {
    if ((locals.var_guard279 != 0.0) && (locals.var_guard280 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign16030_e20069;
        locals.var_t9_dn0 = assign16030_e20069_d_n0;
        locals.var_t9_dn2 = assign16030_e20069_d_n2;
        locals.var_t9_dn4 = assign16030_e20069_d_n4;
        locals.var_t9_dn5 = assign16030_e20069_d_n5;
        locals.var_t9_dn6 = assign16030_e20069_d_n6;
        locals.var_t9_dn8 = assign16030_e20069_d_n8;
        locals.var_t9_dn10 = assign16030_e20069_d_n10;
        locals.var_t9_dn11 = assign16030_e20069_d_n11;
        locals.var_t9_dn12 = assign16030_e20069_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign16040_e20075, assign16040_e20075_d_n0, assign16040_e20075_d_n2, assign16040_e20075_d_n4, assign16040_e20075_d_n5, assign16040_e20075_d_n6, assign16040_e20075_d_n8, assign16040_e20075_d_n10, assign16040_e20075_d_n11, assign16040_e20075_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign16040_e20073: f64 = (locals.var_t6 + 1e-50);
        (assign16040_e20073, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign16040_e20075;
        locals.var_t6_dn0 = assign16040_e20075_d_n0;
        locals.var_t6_dn2 = assign16040_e20075_d_n2;
        locals.var_t6_dn4 = assign16040_e20075_d_n4;
        locals.var_t6_dn5 = assign16040_e20075_d_n5;
        locals.var_t6_dn6 = assign16040_e20075_d_n6;
        locals.var_t6_dn8 = assign16040_e20075_d_n8;
        locals.var_t6_dn10 = assign16040_e20075_d_n10;
        locals.var_t6_dn11 = assign16040_e20075_d_n11;
        locals.var_t6_dn12 = assign16040_e20075_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign16050_e20090, assign16050_e20090_d_n0, assign16050_e20090_d_n2, assign16050_e20090_d_n4, assign16050_e20090_d_n5, assign16050_e20090_d_n6, assign16050_e20090_d_n8, assign16050_e20090_d_n10, assign16050_e20090_d_n11, assign16050_e20090_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign16050_e20079: f64 = (locals.var_t1 * locals.var_uc_svgs);
        let assign16050_e20082: f64 = (locals.var_qnsub_esi / locals.var_t7);
        let assign16050_e20085: f64 = (locals.var_t6).sqrt();
        let assign16050_e20086: f64 = (1.0 - assign16050_e20085);
        let assign16050_e20087: f64 = (assign16050_e20082 * assign16050_e20086);
        let assign16050_e20088: f64 = (assign16050_e20079 + assign16050_e20087);
        (assign16050_e20088, ((locals.var_t1_dn0 * locals.var_uc_svgs) + (((((locals.var_qnsub_esi_dn0 * locals.var_t7) - (locals.var_qnsub_esi * locals.var_t7_dn0)) / (locals.var_t7 * locals.var_t7)) * assign16050_e20086) + (assign16050_e20082 * (-(locals.var_t6_dn0 / (2.0 * assign16050_e20085)))))), ((locals.var_t1_dn2 * locals.var_uc_svgs) + (((((locals.var_qnsub_esi_dn2 * locals.var_t7) - (locals.var_qnsub_esi * locals.var_t7_dn2)) / (locals.var_t7 * locals.var_t7)) * assign16050_e20086) + (assign16050_e20082 * (-(locals.var_t6_dn2 / (2.0 * assign16050_e20085)))))), ((locals.var_t1_dn4 * locals.var_uc_svgs) + (((((locals.var_qnsub_esi_dn4 * locals.var_t7) - (locals.var_qnsub_esi * locals.var_t7_dn4)) / (locals.var_t7 * locals.var_t7)) * assign16050_e20086) + (assign16050_e20082 * (-(locals.var_t6_dn4 / (2.0 * assign16050_e20085)))))), ((locals.var_t1_dn5 * locals.var_uc_svgs) + (((((locals.var_qnsub_esi_dn5 * locals.var_t7) - (locals.var_qnsub_esi * locals.var_t7_dn5)) / (locals.var_t7 * locals.var_t7)) * assign16050_e20086) + (assign16050_e20082 * (-(locals.var_t6_dn5 / (2.0 * assign16050_e20085)))))), ((locals.var_t1_dn6 * locals.var_uc_svgs) + (((((locals.var_qnsub_esi_dn6 * locals.var_t7) - (locals.var_qnsub_esi * locals.var_t7_dn6)) / (locals.var_t7 * locals.var_t7)) * assign16050_e20086) + (assign16050_e20082 * (-(locals.var_t6_dn6 / (2.0 * assign16050_e20085)))))), ((locals.var_t1_dn8 * locals.var_uc_svgs) + (((((locals.var_qnsub_esi_dn8 * locals.var_t7) - (locals.var_qnsub_esi * locals.var_t7_dn8)) / (locals.var_t7 * locals.var_t7)) * assign16050_e20086) + (assign16050_e20082 * (-(locals.var_t6_dn8 / (2.0 * assign16050_e20085)))))), ((locals.var_t1_dn10 * locals.var_uc_svgs) + (((((locals.var_qnsub_esi_dn10 * locals.var_t7) - (locals.var_qnsub_esi * locals.var_t7_dn10)) / (locals.var_t7 * locals.var_t7)) * assign16050_e20086) + (assign16050_e20082 * (-(locals.var_t6_dn10 / (2.0 * assign16050_e20085)))))), ((locals.var_t1_dn11 * locals.var_uc_svgs) + (((((locals.var_qnsub_esi_dn11 * locals.var_t7) - (locals.var_qnsub_esi * locals.var_t7_dn11)) / (locals.var_t7 * locals.var_t7)) * assign16050_e20086) + (assign16050_e20082 * (-(locals.var_t6_dn11 / (2.0 * assign16050_e20085)))))), ((locals.var_t1_dn12 * locals.var_uc_svgs) + (((((locals.var_qnsub_esi_dn12 * locals.var_t7) - (locals.var_qnsub_esi * locals.var_t7_dn12)) / (locals.var_t7 * locals.var_t7)) * assign16050_e20086) + (assign16050_e20082 * (-(locals.var_t6_dn12 / (2.0 * assign16050_e20085)))))),)
    } else {
        (locals.var_psislsat, locals.var_psislsat_dn0, locals.var_psislsat_dn2, locals.var_psislsat_dn4, locals.var_psislsat_dn5, locals.var_psislsat_dn6, locals.var_psislsat_dn8, locals.var_psislsat_dn10, locals.var_psislsat_dn11, locals.var_psislsat_dn12,)
    }
};
        locals.var_psislsat = assign16050_e20090;
        locals.var_psislsat_dn0 = assign16050_e20090_d_n0;
        locals.var_psislsat_dn2 = assign16050_e20090_d_n2;
        locals.var_psislsat_dn4 = assign16050_e20090_d_n4;
        locals.var_psislsat_dn5 = assign16050_e20090_d_n5;
        locals.var_psislsat_dn6 = assign16050_e20090_d_n6;
        locals.var_psislsat_dn8 = assign16050_e20090_d_n8;
        locals.var_psislsat_dn10 = assign16050_e20090_d_n10;
        locals.var_psislsat_dn11 = assign16050_e20090_d_n11;
        locals.var_psislsat_dn12 = assign16050_e20090_d_n12;
        locals.var_psislsat_rv = 0.0;

        let (assign16060_e20104, assign16060_e20104_d_n0, assign16060_e20104_d_n2, assign16060_e20104_d_n4, assign16060_e20104_d_n5, assign16060_e20104_d_n6, assign16060_e20104_d_n8, assign16060_e20104_d_n10, assign16060_e20104_d_n11, assign16060_e20104_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign16060_e20094: f64 = (p.p123 * locals.var_vdsz);
        let assign16060_e20096: f64 = (assign16060_e20094 + locals.var_ps0_isub);
        let assign16060_e20099: f64 = (locals.var_xgate * locals.var_zvgs);
        let assign16060_e20101: f64 = (assign16060_e20099 * locals.var_psislsat);
        let assign16060_e20102: f64 = (assign16060_e20096 - assign16060_e20101);
        (assign16060_e20102, (((p.p123 * locals.var_vdsz_dn0) + locals.var_ps0_isub_dn0) - (assign16060_e20099 * locals.var_psislsat_dn0)), (((p.p123 * locals.var_vdsz_dn2) + locals.var_ps0_isub_dn2) - (assign16060_e20099 * locals.var_psislsat_dn2)), (((p.p123 * locals.var_vdsz_dn4) + locals.var_ps0_isub_dn4) - (assign16060_e20099 * locals.var_psislsat_dn4)), (((p.p123 * locals.var_vdsz_dn5) + locals.var_ps0_isub_dn5) - (assign16060_e20099 * locals.var_psislsat_dn5)), (((p.p123 * locals.var_vdsz_dn6) + locals.var_ps0_isub_dn6) - (assign16060_e20099 * locals.var_psislsat_dn6)), (((p.p123 * locals.var_vdsz_dn8) + locals.var_ps0_isub_dn8) - (assign16060_e20099 * locals.var_psislsat_dn8)), (((p.p123 * locals.var_vdsz_dn10) + locals.var_ps0_isub_dn10) - (assign16060_e20099 * locals.var_psislsat_dn10)), (((p.p123 * locals.var_vdsz_dn11) + locals.var_ps0_isub_dn11) - (assign16060_e20099 * locals.var_psislsat_dn11)), (((p.p123 * locals.var_vdsz_dn12) + locals.var_ps0_isub_dn12) - (assign16060_e20099 * locals.var_psislsat_dn12)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn8, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12,)
    }
};
        locals.var_psisubsat = assign16060_e20104;
        locals.var_psisubsat_dn0 = assign16060_e20104_d_n0;
        locals.var_psisubsat_dn2 = assign16060_e20104_d_n2;
        locals.var_psisubsat_dn4 = assign16060_e20104_d_n4;
        locals.var_psisubsat_dn5 = assign16060_e20104_d_n5;
        locals.var_psisubsat_dn6 = assign16060_e20104_d_n6;
        locals.var_psisubsat_dn8 = assign16060_e20104_d_n8;
        locals.var_psisubsat_dn10 = assign16060_e20104_d_n10;
        locals.var_psisubsat_dn11 = assign16060_e20104_d_n11;
        locals.var_psisubsat_dn12 = assign16060_e20104_d_n12;
        locals.var_psisubsat_rv = 0.0;

        let (assign16070_e20117, assign16070_e20117_d_n0, assign16070_e20117_d_n2, assign16070_e20117_d_n4, assign16070_e20117_d_n5, assign16070_e20117_d_n6, assign16070_e20117_d_n8, assign16070_e20117_d_n10, assign16070_e20117_d_n11, assign16070_e20117_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign16070_e20108: f64 = (locals.var_psisubsat * locals.var_psisubsat);
        let assign16070_e20111: f64 = (4.0 * 0.01);
        let assign16070_e20113: f64 = (assign16070_e20111 * 0.01);
        let assign16070_e20114: f64 = (assign16070_e20108 + assign16070_e20113);
        let assign16070_e20115: f64 = (assign16070_e20114).sqrt();
        (assign16070_e20115, (((locals.var_psisubsat_dn0 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn0)) / (2.0 * assign16070_e20115)), (((locals.var_psisubsat_dn2 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn2)) / (2.0 * assign16070_e20115)), (((locals.var_psisubsat_dn4 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn4)) / (2.0 * assign16070_e20115)), (((locals.var_psisubsat_dn5 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn5)) / (2.0 * assign16070_e20115)), (((locals.var_psisubsat_dn6 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn6)) / (2.0 * assign16070_e20115)), (((locals.var_psisubsat_dn8 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn8)) / (2.0 * assign16070_e20115)), (((locals.var_psisubsat_dn10 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn10)) / (2.0 * assign16070_e20115)), (((locals.var_psisubsat_dn11 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn11)) / (2.0 * assign16070_e20115)), (((locals.var_psisubsat_dn12 * locals.var_psisubsat) + (locals.var_psisubsat * locals.var_psisubsat_dn12)) / (2.0 * assign16070_e20115)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign16070_e20117;
        locals.var_tmf2_dn0 = assign16070_e20117_d_n0;
        locals.var_tmf2_dn2 = assign16070_e20117_d_n2;
        locals.var_tmf2_dn4 = assign16070_e20117_d_n4;
        locals.var_tmf2_dn5 = assign16070_e20117_d_n5;
        locals.var_tmf2_dn6 = assign16070_e20117_d_n6;
        locals.var_tmf2_dn8 = assign16070_e20117_d_n8;
        locals.var_tmf2_dn10 = assign16070_e20117_d_n10;
        locals.var_tmf2_dn11 = assign16070_e20117_d_n11;
        locals.var_tmf2_dn12 = assign16070_e20117_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign16080_e20127, assign16080_e20127_d_n0, assign16080_e20127_d_n2, assign16080_e20127_d_n4, assign16080_e20127_d_n5, assign16080_e20127_d_n6, assign16080_e20127_d_n8, assign16080_e20127_d_n10, assign16080_e20127_d_n11, assign16080_e20127_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign16080_e20123: f64 = (locals.var_psisubsat / locals.var_tmf2);
        let assign16080_e20124: f64 = (1.0 + assign16080_e20123);
        let assign16080_e20125: f64 = (0.5 * assign16080_e20124);
        (assign16080_e20125, (0.5 * (((locals.var_psisubsat_dn0 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn2 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn4 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn5 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn6 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn8 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn10 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn11 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_psisubsat_dn12 * locals.var_tmf2) - (locals.var_psisubsat * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign16080_e20127;
        locals.var_t9_dn0 = assign16080_e20127_d_n0;
        locals.var_t9_dn2 = assign16080_e20127_d_n2;
        locals.var_t9_dn4 = assign16080_e20127_d_n4;
        locals.var_t9_dn5 = assign16080_e20127_d_n5;
        locals.var_t9_dn6 = assign16080_e20127_d_n6;
        locals.var_t9_dn8 = assign16080_e20127_d_n8;
        locals.var_t9_dn10 = assign16080_e20127_d_n10;
        locals.var_t9_dn11 = assign16080_e20127_d_n11;
        locals.var_t9_dn12 = assign16080_e20127_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign16090_e20139, assign16090_e20139_d_n0, assign16090_e20139_d_n2, assign16090_e20139_d_n4, assign16090_e20139_d_n5, assign16090_e20139_d_n6, assign16090_e20139_d_n8, assign16090_e20139_d_n10, assign16090_e20139_d_n11, assign16090_e20139_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign16090_e20132: f64 = (locals.var_psisubsat + locals.var_tmf2);
        let assign16090_e20133: f64 = (0.5 * assign16090_e20132);
        let assign16090_e20136: f64 = (1e-10 * 0.01);
        let assign16090_e20137: f64 = (assign16090_e20133 + assign16090_e20136);
        (assign16090_e20137, (0.5 * (locals.var_psisubsat_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_psisubsat_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_psisubsat_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_psisubsat_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_psisubsat_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_psisubsat_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_psisubsat_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_psisubsat_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_psisubsat_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn8, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12,)
    }
};
        locals.var_psisubsat = assign16090_e20139;
        locals.var_psisubsat_dn0 = assign16090_e20139_d_n0;
        locals.var_psisubsat_dn2 = assign16090_e20139_d_n2;
        locals.var_psisubsat_dn4 = assign16090_e20139_d_n4;
        locals.var_psisubsat_dn5 = assign16090_e20139_d_n5;
        locals.var_psisubsat_dn6 = assign16090_e20139_d_n6;
        locals.var_psisubsat_dn8 = assign16090_e20139_d_n8;
        locals.var_psisubsat_dn10 = assign16090_e20139_d_n10;
        locals.var_psisubsat_dn11 = assign16090_e20139_d_n11;
        locals.var_psisubsat_dn12 = assign16090_e20139_d_n12;
        locals.var_psisubsat_rv = 0.0;

        let assign16100_e20142: f64 = if locals.var_psisubsat < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard281 = assign16100_e20142;
        locals.var_guard281_rv = 0.0;

        let (assign16110_e20148, assign16110_e20148_d_n0, assign16110_e20148_d_n2, assign16110_e20148_d_n4, assign16110_e20148_d_n5, assign16110_e20148_d_n6, assign16110_e20148_d_n8, assign16110_e20148_d_n10, assign16110_e20148_d_n11, assign16110_e20148_d_n12,) = {
    if ((locals.var_guard279 != 0.0) && (locals.var_guard281 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn8, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12,)
    }
};
        locals.var_psisubsat = assign16110_e20148;
        locals.var_psisubsat_dn0 = assign16110_e20148_d_n0;
        locals.var_psisubsat_dn2 = assign16110_e20148_d_n2;
        locals.var_psisubsat_dn4 = assign16110_e20148_d_n4;
        locals.var_psisubsat_dn5 = assign16110_e20148_d_n5;
        locals.var_psisubsat_dn6 = assign16110_e20148_d_n6;
        locals.var_psisubsat_dn8 = assign16110_e20148_d_n8;
        locals.var_psisubsat_dn10 = assign16110_e20148_d_n10;
        locals.var_psisubsat_dn11 = assign16110_e20148_d_n11;
        locals.var_psisubsat_dn12 = assign16110_e20148_d_n12;
        locals.var_psisubsat_rv = 0.0;

        let (assign16120_e20154, assign16120_e20154_d_n0, assign16120_e20154_d_n2, assign16120_e20154_d_n4, assign16120_e20154_d_n5, assign16120_e20154_d_n6, assign16120_e20154_d_n8, assign16120_e20154_d_n10, assign16120_e20154_d_n11, assign16120_e20154_d_n12,) = {
    if ((locals.var_guard279 != 0.0) && (locals.var_guard281 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign16120_e20154;
        locals.var_t9_dn0 = assign16120_e20154_d_n0;
        locals.var_t9_dn2 = assign16120_e20154_d_n2;
        locals.var_t9_dn4 = assign16120_e20154_d_n4;
        locals.var_t9_dn5 = assign16120_e20154_d_n5;
        locals.var_t9_dn6 = assign16120_e20154_d_n6;
        locals.var_t9_dn8 = assign16120_e20154_d_n8;
        locals.var_t9_dn10 = assign16120_e20154_d_n10;
        locals.var_t9_dn11 = assign16120_e20154_d_n11;
        locals.var_t9_dn12 = assign16120_e20154_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign16130_e20160, assign16130_e20160_d_n0, assign16130_e20160_d_n2, assign16130_e20160_d_n4, assign16130_e20160_d_n5, assign16130_e20160_d_n6, assign16130_e20160_d_n8, assign16130_e20160_d_n10, assign16130_e20160_d_n11, assign16130_e20160_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign16130_e20158: f64 = (locals.var_psisubsat + 1e-50);
        (assign16130_e20158, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn8, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12,)
    } else {
        (locals.var_psisubsat, locals.var_psisubsat_dn0, locals.var_psisubsat_dn2, locals.var_psisubsat_dn4, locals.var_psisubsat_dn5, locals.var_psisubsat_dn6, locals.var_psisubsat_dn8, locals.var_psisubsat_dn10, locals.var_psisubsat_dn11, locals.var_psisubsat_dn12,)
    }
};
        locals.var_psisubsat = assign16130_e20160;
        locals.var_psisubsat_dn0 = assign16130_e20160_d_n0;
        locals.var_psisubsat_dn2 = assign16130_e20160_d_n2;
        locals.var_psisubsat_dn4 = assign16130_e20160_d_n4;
        locals.var_psisubsat_dn5 = assign16130_e20160_d_n5;
        locals.var_psisubsat_dn6 = assign16130_e20160_d_n6;
        locals.var_psisubsat_dn8 = assign16130_e20160_d_n8;
        locals.var_psisubsat_dn10 = assign16130_e20160_d_n10;
        locals.var_psisubsat_dn11 = assign16130_e20160_d_n11;
        locals.var_psisubsat_dn12 = assign16130_e20160_d_n12;
        locals.var_psisubsat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16140_e20168, assign16140_e20168_d_n0, assign16140_e20168_d_n2, assign16140_e20168_d_n4, assign16140_e20168_d_n5, assign16140_e20168_d_n6, assign16140_e20168_d_n8, assign16140_e20168_d_n10, assign16140_e20168_d_n11, assign16140_e20168_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign16140_e20163: f64 = (-locals.var_xsub2);
        let assign16140_e20165: f64 = (assign16140_e20163 / locals.var_psisubsat);
        let assign16140_e20166: f64 = (assign16140_e20165).exp();
        (assign16140_e20166, (assign16140_e20166 * (-((assign16140_e20163 * locals.var_psisubsat_dn0) / (locals.var_psisubsat * locals.var_psisubsat)))), (assign16140_e20166 * (-((assign16140_e20163 * locals.var_psisubsat_dn2) / (locals.var_psisubsat * locals.var_psisubsat)))), (assign16140_e20166 * (-((assign16140_e20163 * locals.var_psisubsat_dn4) / (locals.var_psisubsat * locals.var_psisubsat)))), (assign16140_e20166 * (-((assign16140_e20163 * locals.var_psisubsat_dn5) / (locals.var_psisubsat * locals.var_psisubsat)))), (assign16140_e20166 * (-((assign16140_e20163 * locals.var_psisubsat_dn6) / (locals.var_psisubsat * locals.var_psisubsat)))), (assign16140_e20166 * (-((assign16140_e20163 * locals.var_psisubsat_dn8) / (locals.var_psisubsat * locals.var_psisubsat)))), (assign16140_e20166 * (-((assign16140_e20163 * locals.var_psisubsat_dn10) / (locals.var_psisubsat * locals.var_psisubsat)))), (assign16140_e20166 * (-((assign16140_e20163 * locals.var_psisubsat_dn11) / (locals.var_psisubsat * locals.var_psisubsat)))), (assign16140_e20166 * (-((assign16140_e20163 * locals.var_psisubsat_dn12) / (locals.var_psisubsat * locals.var_psisubsat)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign16140_e20168;
        locals.var_t2_dn0 = assign16140_e20168_d_n0;
        locals.var_t2_dn2 = assign16140_e20168_d_n2;
        locals.var_t2_dn4 = assign16140_e20168_d_n4;
        locals.var_t2_dn5 = assign16140_e20168_d_n5;
        locals.var_t2_dn6 = assign16140_e20168_d_n6;
        locals.var_t2_dn8 = assign16140_e20168_d_n8;
        locals.var_t2_dn10 = assign16140_e20168_d_n10;
        locals.var_t2_dn11 = assign16140_e20168_d_n11;
        locals.var_t2_dn12 = assign16140_e20168_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign16150_e20178, assign16150_e20178_d_n0, assign16150_e20178_d_n2, assign16150_e20178_d_n4, assign16150_e20178_d_n5, assign16150_e20178_d_n6, assign16150_e20178_d_n8, assign16150_e20178_d_n10, assign16150_e20178_d_n11, assign16150_e20178_d_n12,) = {
    if (locals.var_guard279 != 0.0) {
        let assign16150_e20172: f64 = (locals.var_xsub1 * locals.var_psisubsat);
        let assign16150_e20174: f64 = (assign16150_e20172 * locals.var_ids);
        let assign16150_e20176: f64 = (assign16150_e20174 * locals.var_t2);
        (assign16150_e20176, (((((locals.var_xsub1 * locals.var_psisubsat_dn0) * locals.var_ids) + (assign16150_e20172 * locals.var_ids_dn0)) * locals.var_t2) + (assign16150_e20174 * locals.var_t2_dn0)), (((((locals.var_xsub1 * locals.var_psisubsat_dn2) * locals.var_ids) + (assign16150_e20172 * locals.var_ids_dn2)) * locals.var_t2) + (assign16150_e20174 * locals.var_t2_dn2)), (((((locals.var_xsub1 * locals.var_psisubsat_dn4) * locals.var_ids) + (assign16150_e20172 * locals.var_ids_dn4)) * locals.var_t2) + (assign16150_e20174 * locals.var_t2_dn4)), (((((locals.var_xsub1 * locals.var_psisubsat_dn5) * locals.var_ids) + (assign16150_e20172 * locals.var_ids_dn5)) * locals.var_t2) + (assign16150_e20174 * locals.var_t2_dn5)), (((((locals.var_xsub1 * locals.var_psisubsat_dn6) * locals.var_ids) + (assign16150_e20172 * locals.var_ids_dn6)) * locals.var_t2) + (assign16150_e20174 * locals.var_t2_dn6)), (((((locals.var_xsub1 * locals.var_psisubsat_dn8) * locals.var_ids) + (assign16150_e20172 * locals.var_ids_dn8)) * locals.var_t2) + (assign16150_e20174 * locals.var_t2_dn8)), (((((locals.var_xsub1 * locals.var_psisubsat_dn10) * locals.var_ids) + (assign16150_e20172 * locals.var_ids_dn10)) * locals.var_t2) + (assign16150_e20174 * locals.var_t2_dn10)), (((((locals.var_xsub1 * locals.var_psisubsat_dn11) * locals.var_ids) + (assign16150_e20172 * locals.var_ids_dn11)) * locals.var_t2) + (assign16150_e20174 * locals.var_t2_dn11)), (((((locals.var_xsub1 * locals.var_psisubsat_dn12) * locals.var_ids) + (assign16150_e20172 * locals.var_ids_dn12)) * locals.var_t2) + (assign16150_e20174 * locals.var_t2_dn12)),)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn8, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12,)
    }
};
        locals.var_isub = assign16150_e20178;
        locals.var_isub_dn0 = assign16150_e20178_d_n0;
        locals.var_isub_dn2 = assign16150_e20178_d_n2;
        locals.var_isub_dn4 = assign16150_e20178_d_n4;
        locals.var_isub_dn5 = assign16150_e20178_d_n5;
        locals.var_isub_dn6 = assign16150_e20178_d_n6;
        locals.var_isub_dn8 = assign16150_e20178_d_n8;
        locals.var_isub_dn10 = assign16150_e20178_d_n10;
        locals.var_isub_dn11 = assign16150_e20178_d_n11;
        locals.var_isub_dn12 = assign16150_e20178_d_n12;
        locals.var_isub_rv = 0.0;

        let assign16160_e20189: f64 = if (((locals.var_flg_noqi == 0.0) && (locals.var_isub > 0.0)) && (p.p145 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard282 = assign16160_e20189;
        locals.var_guard282_rv = 0.0;

        let (assign16170_e20197, assign16170_e20197_d_n0, assign16170_e20197_d_n2, assign16170_e20197_d_n4, assign16170_e20197_d_n5, assign16170_e20197_d_n6, assign16170_e20197_d_n8, assign16170_e20197_d_n10, assign16170_e20197_d_n11, assign16170_e20197_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16170_e20194: f64 = (p.p146 * locals.var_dvth);
        let assign16170_e20195: f64 = (1.0 + assign16170_e20194);
        (assign16170_e20195, (p.p146 * locals.var_dvth_dn0), (p.p146 * locals.var_dvth_dn2), (p.p146 * locals.var_dvth_dn4), (p.p146 * locals.var_dvth_dn5), (p.p146 * locals.var_dvth_dn6), (p.p146 * locals.var_dvth_dn8), (p.p146 * locals.var_dvth_dn10), (p.p146 * locals.var_dvth_dn11), (p.p146 * locals.var_dvth_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign16170_e20197;
        locals.var_t0_dn0 = assign16170_e20197_d_n0;
        locals.var_t0_dn2 = assign16170_e20197_d_n2;
        locals.var_t0_dn4 = assign16170_e20197_d_n4;
        locals.var_t0_dn5 = assign16170_e20197_d_n5;
        locals.var_t0_dn6 = assign16170_e20197_d_n6;
        locals.var_t0_dn8 = assign16170_e20197_d_n8;
        locals.var_t0_dn10 = assign16170_e20197_d_n10;
        locals.var_t0_dn11 = assign16170_e20197_d_n11;
        locals.var_t0_dn12 = assign16170_e20197_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign16180_e20205, assign16180_e20205_d_n0, assign16180_e20205_d_n2, assign16180_e20205_d_n4, assign16180_e20205_d_n5, assign16180_e20205_d_n6, assign16180_e20205_d_n8, assign16180_e20205_d_n10, assign16180_e20205_d_n11, assign16180_e20205_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16180_e20201: f64 = (p.p145 * locals.var_t0);
        let assign16180_e20203: f64 = (assign16180_e20201 * locals.var_isub);
        (assign16180_e20203, (((p.p145 * locals.var_t0_dn0) * locals.var_isub) + (assign16180_e20201 * locals.var_isub_dn0)), (((p.p145 * locals.var_t0_dn2) * locals.var_isub) + (assign16180_e20201 * locals.var_isub_dn2)), (((p.p145 * locals.var_t0_dn4) * locals.var_isub) + (assign16180_e20201 * locals.var_isub_dn4)), (((p.p145 * locals.var_t0_dn5) * locals.var_isub) + (assign16180_e20201 * locals.var_isub_dn5)), (((p.p145 * locals.var_t0_dn6) * locals.var_isub) + (assign16180_e20201 * locals.var_isub_dn6)), (((p.p145 * locals.var_t0_dn8) * locals.var_isub) + (assign16180_e20201 * locals.var_isub_dn8)), (((p.p145 * locals.var_t0_dn10) * locals.var_isub) + (assign16180_e20201 * locals.var_isub_dn10)), (((p.p145 * locals.var_t0_dn11) * locals.var_isub) + (assign16180_e20201 * locals.var_isub_dn11)), (((p.p145 * locals.var_t0_dn12) * locals.var_isub) + (assign16180_e20201 * locals.var_isub_dn12)),)
    } else {
        (locals.var_dvbsibpc, locals.var_dvbsibpc_dn0, locals.var_dvbsibpc_dn2, locals.var_dvbsibpc_dn4, locals.var_dvbsibpc_dn5, locals.var_dvbsibpc_dn6, locals.var_dvbsibpc_dn8, locals.var_dvbsibpc_dn10, locals.var_dvbsibpc_dn11, locals.var_dvbsibpc_dn12,)
    }
};
        locals.var_dvbsibpc = assign16180_e20205;
        locals.var_dvbsibpc_dn0 = assign16180_e20205_d_n0;
        locals.var_dvbsibpc_dn2 = assign16180_e20205_d_n2;
        locals.var_dvbsibpc_dn4 = assign16180_e20205_d_n4;
        locals.var_dvbsibpc_dn5 = assign16180_e20205_d_n5;
        locals.var_dvbsibpc_dn6 = assign16180_e20205_d_n6;
        locals.var_dvbsibpc_dn8 = assign16180_e20205_d_n8;
        locals.var_dvbsibpc_dn10 = assign16180_e20205_d_n10;
        locals.var_dvbsibpc_dn11 = assign16180_e20205_d_n11;
        locals.var_dvbsibpc_dn12 = assign16180_e20205_d_n12;
        locals.var_dvbsibpc_rv = 0.0;

        let (assign16190_e20213, assign16190_e20213_d_n0, assign16190_e20213_d_n2, assign16190_e20213_d_n4, assign16190_e20213_d_n5, assign16190_e20213_d_n6, assign16190_e20213_d_n8, assign16190_e20213_d_n10, assign16190_e20213_d_n11, assign16190_e20213_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16190_e20209: f64 = (locals.var_beta * locals.var_ps0);
        let assign16190_e20211: f64 = (assign16190_e20209 - 1.0);
        (assign16190_e20211, (locals.var_beta * locals.var_ps0_dn0), (locals.var_beta * locals.var_ps0_dn2), ((locals.var_beta_dn4 * locals.var_ps0) + (locals.var_beta * locals.var_ps0_dn4)), (locals.var_beta * locals.var_ps0_dn5), (locals.var_beta * locals.var_ps0_dn6), (locals.var_beta * locals.var_ps0_dn8), (locals.var_beta * locals.var_ps0_dn10), (locals.var_beta * locals.var_ps0_dn11), (locals.var_beta * locals.var_ps0_dn12),)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn8, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12,)
    }
};
        locals.var_xi0 = assign16190_e20213;
        locals.var_xi0_dn0 = assign16190_e20213_d_n0;
        locals.var_xi0_dn2 = assign16190_e20213_d_n2;
        locals.var_xi0_dn4 = assign16190_e20213_d_n4;
        locals.var_xi0_dn5 = assign16190_e20213_d_n5;
        locals.var_xi0_dn6 = assign16190_e20213_d_n6;
        locals.var_xi0_dn8 = assign16190_e20213_d_n8;
        locals.var_xi0_dn10 = assign16190_e20213_d_n10;
        locals.var_xi0_dn11 = assign16190_e20213_d_n11;
        locals.var_xi0_dn12 = assign16190_e20213_d_n12;
        locals.var_xi0_rv = 0.0;

        let (assign16200_e20226, assign16200_e20226_d_n0, assign16200_e20226_d_n2, assign16200_e20226_d_n4, assign16200_e20226_d_n5, assign16200_e20226_d_n6, assign16200_e20226_d_n8, assign16200_e20226_d_n10, assign16200_e20226_d_n11, assign16200_e20226_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16200_e20217: f64 = (locals.var_xi0 * locals.var_xi0);
        let assign16200_e20220: f64 = (4.0 * 0.1);
        let assign16200_e20222: f64 = (assign16200_e20220 * 0.1);
        let assign16200_e20223: f64 = (assign16200_e20217 + assign16200_e20222);
        let assign16200_e20224: f64 = (assign16200_e20223).sqrt();
        (assign16200_e20224, (((locals.var_xi0_dn0 * locals.var_xi0) + (locals.var_xi0 * locals.var_xi0_dn0)) / (2.0 * assign16200_e20224)), (((locals.var_xi0_dn2 * locals.var_xi0) + (locals.var_xi0 * locals.var_xi0_dn2)) / (2.0 * assign16200_e20224)), (((locals.var_xi0_dn4 * locals.var_xi0) + (locals.var_xi0 * locals.var_xi0_dn4)) / (2.0 * assign16200_e20224)), (((locals.var_xi0_dn5 * locals.var_xi0) + (locals.var_xi0 * locals.var_xi0_dn5)) / (2.0 * assign16200_e20224)), (((locals.var_xi0_dn6 * locals.var_xi0) + (locals.var_xi0 * locals.var_xi0_dn6)) / (2.0 * assign16200_e20224)), (((locals.var_xi0_dn8 * locals.var_xi0) + (locals.var_xi0 * locals.var_xi0_dn8)) / (2.0 * assign16200_e20224)), (((locals.var_xi0_dn10 * locals.var_xi0) + (locals.var_xi0 * locals.var_xi0_dn10)) / (2.0 * assign16200_e20224)), (((locals.var_xi0_dn11 * locals.var_xi0) + (locals.var_xi0 * locals.var_xi0_dn11)) / (2.0 * assign16200_e20224)), (((locals.var_xi0_dn12 * locals.var_xi0) + (locals.var_xi0 * locals.var_xi0_dn12)) / (2.0 * assign16200_e20224)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign16200_e20226;
        locals.var_tmf2_dn0 = assign16200_e20226_d_n0;
        locals.var_tmf2_dn2 = assign16200_e20226_d_n2;
        locals.var_tmf2_dn4 = assign16200_e20226_d_n4;
        locals.var_tmf2_dn5 = assign16200_e20226_d_n5;
        locals.var_tmf2_dn6 = assign16200_e20226_d_n6;
        locals.var_tmf2_dn8 = assign16200_e20226_d_n8;
        locals.var_tmf2_dn10 = assign16200_e20226_d_n10;
        locals.var_tmf2_dn11 = assign16200_e20226_d_n11;
        locals.var_tmf2_dn12 = assign16200_e20226_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign16210_e20238, assign16210_e20238_d_n0, assign16210_e20238_d_n2, assign16210_e20238_d_n4, assign16210_e20238_d_n5, assign16210_e20238_d_n6, assign16210_e20238_d_n8, assign16210_e20238_d_n10, assign16210_e20238_d_n11, assign16210_e20238_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16210_e20231: f64 = (locals.var_xi0 + locals.var_tmf2);
        let assign16210_e20232: f64 = (0.5 * assign16210_e20231);
        let assign16210_e20235: f64 = (1e-10 * 0.1);
        let assign16210_e20236: f64 = (assign16210_e20232 + assign16210_e20235);
        (assign16210_e20236, (0.5 * (locals.var_xi0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_xi0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_xi0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_xi0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_xi0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_xi0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_xi0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_xi0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_xi0_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn8, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12,)
    }
};
        locals.var_xi0 = assign16210_e20238;
        locals.var_xi0_dn0 = assign16210_e20238_d_n0;
        locals.var_xi0_dn2 = assign16210_e20238_d_n2;
        locals.var_xi0_dn4 = assign16210_e20238_d_n4;
        locals.var_xi0_dn5 = assign16210_e20238_d_n5;
        locals.var_xi0_dn6 = assign16210_e20238_d_n6;
        locals.var_xi0_dn8 = assign16210_e20238_d_n8;
        locals.var_xi0_dn10 = assign16210_e20238_d_n10;
        locals.var_xi0_dn11 = assign16210_e20238_d_n11;
        locals.var_xi0_dn12 = assign16210_e20238_d_n12;
        locals.var_xi0_rv = 0.0;

        let assign16220_e20241: f64 = if locals.var_xi0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard283 = assign16220_e20241;
        locals.var_guard283_rv = 0.0;

        let (assign16230_e20247, assign16230_e20247_d_n0, assign16230_e20247_d_n2, assign16230_e20247_d_n4, assign16230_e20247_d_n5, assign16230_e20247_d_n6, assign16230_e20247_d_n8, assign16230_e20247_d_n10, assign16230_e20247_d_n11, assign16230_e20247_d_n12,) = {
    if ((locals.var_guard282 != 0.0) && (locals.var_guard283 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xi0, locals.var_xi0_dn0, locals.var_xi0_dn2, locals.var_xi0_dn4, locals.var_xi0_dn5, locals.var_xi0_dn6, locals.var_xi0_dn8, locals.var_xi0_dn10, locals.var_xi0_dn11, locals.var_xi0_dn12,)
    }
};
        locals.var_xi0 = assign16230_e20247;
        locals.var_xi0_dn0 = assign16230_e20247_d_n0;
        locals.var_xi0_dn2 = assign16230_e20247_d_n2;
        locals.var_xi0_dn4 = assign16230_e20247_d_n4;
        locals.var_xi0_dn5 = assign16230_e20247_d_n5;
        locals.var_xi0_dn6 = assign16230_e20247_d_n6;
        locals.var_xi0_dn8 = assign16230_e20247_d_n8;
        locals.var_xi0_dn10 = assign16230_e20247_d_n10;
        locals.var_xi0_dn11 = assign16230_e20247_d_n11;
        locals.var_xi0_dn12 = assign16230_e20247_d_n12;
        locals.var_xi0_rv = 0.0;

        let (assign16240_e20252, assign16240_e20252_d_n0, assign16240_e20252_d_n2, assign16240_e20252_d_n4, assign16240_e20252_d_n5, assign16240_e20252_d_n6, assign16240_e20252_d_n8, assign16240_e20252_d_n10, assign16240_e20252_d_n11, assign16240_e20252_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16240_e20250: f64 = (locals.var_xi0).sqrt();
        (assign16240_e20250, (locals.var_xi0_dn0 / (2.0 * assign16240_e20250)), (locals.var_xi0_dn2 / (2.0 * assign16240_e20250)), (locals.var_xi0_dn4 / (2.0 * assign16240_e20250)), (locals.var_xi0_dn5 / (2.0 * assign16240_e20250)), (locals.var_xi0_dn6 / (2.0 * assign16240_e20250)), (locals.var_xi0_dn8 / (2.0 * assign16240_e20250)), (locals.var_xi0_dn10 / (2.0 * assign16240_e20250)), (locals.var_xi0_dn11 / (2.0 * assign16240_e20250)), (locals.var_xi0_dn12 / (2.0 * assign16240_e20250)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn8, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn12,)
    }
};
        locals.var_xi0p12 = assign16240_e20252;
        locals.var_xi0p12_dn0 = assign16240_e20252_d_n0;
        locals.var_xi0p12_dn2 = assign16240_e20252_d_n2;
        locals.var_xi0p12_dn4 = assign16240_e20252_d_n4;
        locals.var_xi0p12_dn5 = assign16240_e20252_d_n5;
        locals.var_xi0p12_dn6 = assign16240_e20252_d_n6;
        locals.var_xi0p12_dn8 = assign16240_e20252_d_n8;
        locals.var_xi0p12_dn10 = assign16240_e20252_d_n10;
        locals.var_xi0p12_dn11 = assign16240_e20252_d_n11;
        locals.var_xi0p12_dn12 = assign16240_e20252_d_n12;
        locals.var_xi0p12_rv = 0.0;

        let (assign16250_e20258, assign16250_e20258_d_n0, assign16250_e20258_d_n2, assign16250_e20258_d_n4, assign16250_e20258_d_n5, assign16250_e20258_d_n6, assign16250_e20258_d_n8, assign16250_e20258_d_n10, assign16250_e20258_d_n11, assign16250_e20258_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16250_e20256: f64 = (locals.var_xi0 * locals.var_xi0p12);
        (assign16250_e20256, ((locals.var_xi0_dn0 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn0)), ((locals.var_xi0_dn2 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn2)), ((locals.var_xi0_dn4 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn4)), ((locals.var_xi0_dn5 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn5)), ((locals.var_xi0_dn6 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn6)), ((locals.var_xi0_dn8 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn8)), ((locals.var_xi0_dn10 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn10)), ((locals.var_xi0_dn11 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn11)), ((locals.var_xi0_dn12 * locals.var_xi0p12) + (locals.var_xi0 * locals.var_xi0p12_dn12)),)
    } else {
        (locals.var_xi0p32, locals.var_xi0p32_dn0, locals.var_xi0p32_dn2, locals.var_xi0p32_dn4, locals.var_xi0p32_dn5, locals.var_xi0p32_dn6, locals.var_xi0p32_dn8, locals.var_xi0p32_dn10, locals.var_xi0p32_dn11, locals.var_xi0p32_dn12,)
    }
};
        locals.var_xi0p32 = assign16250_e20258;
        locals.var_xi0p32_dn0 = assign16250_e20258_d_n0;
        locals.var_xi0p32_dn2 = assign16250_e20258_d_n2;
        locals.var_xi0p32_dn4 = assign16250_e20258_d_n4;
        locals.var_xi0p32_dn5 = assign16250_e20258_d_n5;
        locals.var_xi0p32_dn6 = assign16250_e20258_d_n6;
        locals.var_xi0p32_dn8 = assign16250_e20258_d_n8;
        locals.var_xi0p32_dn10 = assign16250_e20258_d_n10;
        locals.var_xi0p32_dn11 = assign16250_e20258_d_n11;
        locals.var_xi0p32_dn12 = assign16250_e20258_d_n12;
        locals.var_xi0p32_rv = 0.0;

        let (assign16260_e20266, assign16260_e20266_d_n0, assign16260_e20266_d_n2, assign16260_e20266_d_n4, assign16260_e20266_d_n5, assign16260_e20266_d_n6, assign16260_e20266_d_n8, assign16260_e20266_d_n10, assign16260_e20266_d_n11, assign16260_e20266_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16260_e20262: f64 = (locals.var_beta * locals.var_psl);
        let assign16260_e20264: f64 = (assign16260_e20262 - 1.0);
        (assign16260_e20264, (locals.var_beta * locals.var_psl_dn0), (locals.var_beta * locals.var_psl_dn2), ((locals.var_beta_dn4 * locals.var_psl) + (locals.var_beta * locals.var_psl_dn4)), (locals.var_beta * locals.var_psl_dn5), (locals.var_beta * locals.var_psl_dn6), (locals.var_beta * locals.var_psl_dn8), (locals.var_beta * locals.var_psl_dn10), (locals.var_beta * locals.var_psl_dn11), (locals.var_beta * locals.var_psl_dn12),)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn8, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn12,)
    }
};
        locals.var_xil = assign16260_e20266;
        locals.var_xil_dn0 = assign16260_e20266_d_n0;
        locals.var_xil_dn2 = assign16260_e20266_d_n2;
        locals.var_xil_dn4 = assign16260_e20266_d_n4;
        locals.var_xil_dn5 = assign16260_e20266_d_n5;
        locals.var_xil_dn6 = assign16260_e20266_d_n6;
        locals.var_xil_dn8 = assign16260_e20266_d_n8;
        locals.var_xil_dn10 = assign16260_e20266_d_n10;
        locals.var_xil_dn11 = assign16260_e20266_d_n11;
        locals.var_xil_dn12 = assign16260_e20266_d_n12;
        locals.var_xil_rv = 0.0;

        let (assign16270_e20279, assign16270_e20279_d_n0, assign16270_e20279_d_n2, assign16270_e20279_d_n4, assign16270_e20279_d_n5, assign16270_e20279_d_n6, assign16270_e20279_d_n8, assign16270_e20279_d_n10, assign16270_e20279_d_n11, assign16270_e20279_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16270_e20270: f64 = (locals.var_xil * locals.var_xil);
        let assign16270_e20273: f64 = (4.0 * 0.1);
        let assign16270_e20275: f64 = (assign16270_e20273 * 0.1);
        let assign16270_e20276: f64 = (assign16270_e20270 + assign16270_e20275);
        let assign16270_e20277: f64 = (assign16270_e20276).sqrt();
        (assign16270_e20277, (((locals.var_xil_dn0 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn0)) / (2.0 * assign16270_e20277)), (((locals.var_xil_dn2 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn2)) / (2.0 * assign16270_e20277)), (((locals.var_xil_dn4 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn4)) / (2.0 * assign16270_e20277)), (((locals.var_xil_dn5 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn5)) / (2.0 * assign16270_e20277)), (((locals.var_xil_dn6 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn6)) / (2.0 * assign16270_e20277)), (((locals.var_xil_dn8 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn8)) / (2.0 * assign16270_e20277)), (((locals.var_xil_dn10 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn10)) / (2.0 * assign16270_e20277)), (((locals.var_xil_dn11 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn11)) / (2.0 * assign16270_e20277)), (((locals.var_xil_dn12 * locals.var_xil) + (locals.var_xil * locals.var_xil_dn12)) / (2.0 * assign16270_e20277)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign16270_e20279;
        locals.var_tmf2_dn0 = assign16270_e20279_d_n0;
        locals.var_tmf2_dn2 = assign16270_e20279_d_n2;
        locals.var_tmf2_dn4 = assign16270_e20279_d_n4;
        locals.var_tmf2_dn5 = assign16270_e20279_d_n5;
        locals.var_tmf2_dn6 = assign16270_e20279_d_n6;
        locals.var_tmf2_dn8 = assign16270_e20279_d_n8;
        locals.var_tmf2_dn10 = assign16270_e20279_d_n10;
        locals.var_tmf2_dn11 = assign16270_e20279_d_n11;
        locals.var_tmf2_dn12 = assign16270_e20279_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign16280_e20291, assign16280_e20291_d_n0, assign16280_e20291_d_n2, assign16280_e20291_d_n4, assign16280_e20291_d_n5, assign16280_e20291_d_n6, assign16280_e20291_d_n8, assign16280_e20291_d_n10, assign16280_e20291_d_n11, assign16280_e20291_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16280_e20284: f64 = (locals.var_xil + locals.var_tmf2);
        let assign16280_e20285: f64 = (0.5 * assign16280_e20284);
        let assign16280_e20288: f64 = (1e-10 * 0.1);
        let assign16280_e20289: f64 = (assign16280_e20285 + assign16280_e20288);
        (assign16280_e20289, (0.5 * (locals.var_xil_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_xil_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_xil_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_xil_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_xil_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_xil_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_xil_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_xil_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_xil_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn8, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn12,)
    }
};
        locals.var_xil = assign16280_e20291;
        locals.var_xil_dn0 = assign16280_e20291_d_n0;
        locals.var_xil_dn2 = assign16280_e20291_d_n2;
        locals.var_xil_dn4 = assign16280_e20291_d_n4;
        locals.var_xil_dn5 = assign16280_e20291_d_n5;
        locals.var_xil_dn6 = assign16280_e20291_d_n6;
        locals.var_xil_dn8 = assign16280_e20291_d_n8;
        locals.var_xil_dn10 = assign16280_e20291_d_n10;
        locals.var_xil_dn11 = assign16280_e20291_d_n11;
        locals.var_xil_dn12 = assign16280_e20291_d_n12;
        locals.var_xil_rv = 0.0;

        let assign16290_e20294: f64 = if locals.var_xil < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign16290_e20294;
        locals.var_guard284_rv = 0.0;

        let (assign16300_e20300, assign16300_e20300_d_n0, assign16300_e20300_d_n2, assign16300_e20300_d_n4, assign16300_e20300_d_n5, assign16300_e20300_d_n6, assign16300_e20300_d_n8, assign16300_e20300_d_n10, assign16300_e20300_d_n11, assign16300_e20300_d_n12,) = {
    if ((locals.var_guard282 != 0.0) && (locals.var_guard284 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn8, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn12,)
    }
};
        locals.var_xil = assign16300_e20300;
        locals.var_xil_dn0 = assign16300_e20300_d_n0;
        locals.var_xil_dn2 = assign16300_e20300_d_n2;
        locals.var_xil_dn4 = assign16300_e20300_d_n4;
        locals.var_xil_dn5 = assign16300_e20300_d_n5;
        locals.var_xil_dn6 = assign16300_e20300_d_n6;
        locals.var_xil_dn8 = assign16300_e20300_d_n8;
        locals.var_xil_dn10 = assign16300_e20300_d_n10;
        locals.var_xil_dn11 = assign16300_e20300_d_n11;
        locals.var_xil_dn12 = assign16300_e20300_d_n12;
        locals.var_xil_rv = 0.0;

        let (assign16310_e20305, assign16310_e20305_d_n0, assign16310_e20305_d_n2, assign16310_e20305_d_n4, assign16310_e20305_d_n5, assign16310_e20305_d_n6, assign16310_e20305_d_n8, assign16310_e20305_d_n10, assign16310_e20305_d_n11, assign16310_e20305_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16310_e20303: f64 = (locals.var_xil).sqrt();
        (assign16310_e20303, (locals.var_xil_dn0 / (2.0 * assign16310_e20303)), (locals.var_xil_dn2 / (2.0 * assign16310_e20303)), (locals.var_xil_dn4 / (2.0 * assign16310_e20303)), (locals.var_xil_dn5 / (2.0 * assign16310_e20303)), (locals.var_xil_dn6 / (2.0 * assign16310_e20303)), (locals.var_xil_dn8 / (2.0 * assign16310_e20303)), (locals.var_xil_dn10 / (2.0 * assign16310_e20303)), (locals.var_xil_dn11 / (2.0 * assign16310_e20303)), (locals.var_xil_dn12 / (2.0 * assign16310_e20303)),)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn4, locals.var_xilp12_dn5, locals.var_xilp12_dn6, locals.var_xilp12_dn8, locals.var_xilp12_dn10, locals.var_xilp12_dn11, locals.var_xilp12_dn12,)
    }
};
        locals.var_xilp12 = assign16310_e20305;
        locals.var_xilp12_dn0 = assign16310_e20305_d_n0;
        locals.var_xilp12_dn2 = assign16310_e20305_d_n2;
        locals.var_xilp12_dn4 = assign16310_e20305_d_n4;
        locals.var_xilp12_dn5 = assign16310_e20305_d_n5;
        locals.var_xilp12_dn6 = assign16310_e20305_d_n6;
        locals.var_xilp12_dn8 = assign16310_e20305_d_n8;
        locals.var_xilp12_dn10 = assign16310_e20305_d_n10;
        locals.var_xilp12_dn11 = assign16310_e20305_d_n11;
        locals.var_xilp12_dn12 = assign16310_e20305_d_n12;
        locals.var_xilp12_rv = 0.0;

        let (assign16320_e20311, assign16320_e20311_d_n0, assign16320_e20311_d_n2, assign16320_e20311_d_n4, assign16320_e20311_d_n5, assign16320_e20311_d_n6, assign16320_e20311_d_n8, assign16320_e20311_d_n10, assign16320_e20311_d_n11, assign16320_e20311_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16320_e20309: f64 = (locals.var_xil * locals.var_xilp12);
        (assign16320_e20309, ((locals.var_xil_dn0 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn0)), ((locals.var_xil_dn2 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn2)), ((locals.var_xil_dn4 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn4)), ((locals.var_xil_dn5 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn5)), ((locals.var_xil_dn6 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn6)), ((locals.var_xil_dn8 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn8)), ((locals.var_xil_dn10 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn10)), ((locals.var_xil_dn11 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn11)), ((locals.var_xil_dn12 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn12)),)
    } else {
        (locals.var_xilp32, locals.var_xilp32_dn0, locals.var_xilp32_dn2, locals.var_xilp32_dn4, locals.var_xilp32_dn5, locals.var_xilp32_dn6, locals.var_xilp32_dn8, locals.var_xilp32_dn10, locals.var_xilp32_dn11, locals.var_xilp32_dn12,)
    }
};
        locals.var_xilp32 = assign16320_e20311;
        locals.var_xilp32_dn0 = assign16320_e20311_d_n0;
        locals.var_xilp32_dn2 = assign16320_e20311_d_n2;
        locals.var_xilp32_dn4 = assign16320_e20311_d_n4;
        locals.var_xilp32_dn5 = assign16320_e20311_d_n5;
        locals.var_xilp32_dn6 = assign16320_e20311_d_n6;
        locals.var_xilp32_dn8 = assign16320_e20311_d_n8;
        locals.var_xilp32_dn10 = assign16320_e20311_d_n10;
        locals.var_xilp32_dn11 = assign16320_e20311_d_n11;
        locals.var_xilp32_dn12 = assign16320_e20311_d_n12;
        locals.var_xilp32_rv = 0.0;

        let (assign16330_e20319, assign16330_e20319_d_n0, assign16330_e20319_d_n2, assign16330_e20319_d_n4, assign16330_e20319_d_n5, assign16330_e20319_d_n6, assign16330_e20319_d_n8, assign16330_e20319_d_n10, assign16330_e20319_d_n11, assign16330_e20319_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16330_e20315: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign16330_e20317: f64 = (assign16330_e20315 / locals.var_xi0);
        (assign16330_e20317, ((((locals.var_beta * locals.var_dvbsibpc_dn0) * locals.var_xi0) - (assign16330_e20315 * locals.var_xi0_dn0)) / (locals.var_xi0 * locals.var_xi0)), ((((locals.var_beta * locals.var_dvbsibpc_dn2) * locals.var_xi0) - (assign16330_e20315 * locals.var_xi0_dn2)) / (locals.var_xi0 * locals.var_xi0)), (((((locals.var_beta_dn4 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn4)) * locals.var_xi0) - (assign16330_e20315 * locals.var_xi0_dn4)) / (locals.var_xi0 * locals.var_xi0)), ((((locals.var_beta * locals.var_dvbsibpc_dn5) * locals.var_xi0) - (assign16330_e20315 * locals.var_xi0_dn5)) / (locals.var_xi0 * locals.var_xi0)), ((((locals.var_beta * locals.var_dvbsibpc_dn6) * locals.var_xi0) - (assign16330_e20315 * locals.var_xi0_dn6)) / (locals.var_xi0 * locals.var_xi0)), ((((locals.var_beta * locals.var_dvbsibpc_dn8) * locals.var_xi0) - (assign16330_e20315 * locals.var_xi0_dn8)) / (locals.var_xi0 * locals.var_xi0)), ((((locals.var_beta * locals.var_dvbsibpc_dn10) * locals.var_xi0) - (assign16330_e20315 * locals.var_xi0_dn10)) / (locals.var_xi0 * locals.var_xi0)), ((((locals.var_beta * locals.var_dvbsibpc_dn11) * locals.var_xi0) - (assign16330_e20315 * locals.var_xi0_dn11)) / (locals.var_xi0 * locals.var_xi0)), ((((locals.var_beta * locals.var_dvbsibpc_dn12) * locals.var_xi0) - (assign16330_e20315 * locals.var_xi0_dn12)) / (locals.var_xi0 * locals.var_xi0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign16330_e20319;
        locals.var_t1_dn0 = assign16330_e20319_d_n0;
        locals.var_t1_dn2 = assign16330_e20319_d_n2;
        locals.var_t1_dn4 = assign16330_e20319_d_n4;
        locals.var_t1_dn5 = assign16330_e20319_d_n5;
        locals.var_t1_dn6 = assign16330_e20319_d_n6;
        locals.var_t1_dn8 = assign16330_e20319_d_n8;
        locals.var_t1_dn10 = assign16330_e20319_d_n10;
        locals.var_t1_dn11 = assign16330_e20319_d_n11;
        locals.var_t1_dn12 = assign16330_e20319_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign16340_e20327, assign16340_e20327_d_n0, assign16340_e20327_d_n2, assign16340_e20327_d_n4, assign16340_e20327_d_n5, assign16340_e20327_d_n6, assign16340_e20327_d_n8, assign16340_e20327_d_n10, assign16340_e20327_d_n11, assign16340_e20327_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16340_e20323: f64 = (locals.var_beta * locals.var_dvbsibpc);
        let assign16340_e20325: f64 = (assign16340_e20323 / locals.var_xil);
        (assign16340_e20325, ((((locals.var_beta * locals.var_dvbsibpc_dn0) * locals.var_xil) - (assign16340_e20323 * locals.var_xil_dn0)) / (locals.var_xil * locals.var_xil)), ((((locals.var_beta * locals.var_dvbsibpc_dn2) * locals.var_xil) - (assign16340_e20323 * locals.var_xil_dn2)) / (locals.var_xil * locals.var_xil)), (((((locals.var_beta_dn4 * locals.var_dvbsibpc) + (locals.var_beta * locals.var_dvbsibpc_dn4)) * locals.var_xil) - (assign16340_e20323 * locals.var_xil_dn4)) / (locals.var_xil * locals.var_xil)), ((((locals.var_beta * locals.var_dvbsibpc_dn5) * locals.var_xil) - (assign16340_e20323 * locals.var_xil_dn5)) / (locals.var_xil * locals.var_xil)), ((((locals.var_beta * locals.var_dvbsibpc_dn6) * locals.var_xil) - (assign16340_e20323 * locals.var_xil_dn6)) / (locals.var_xil * locals.var_xil)), ((((locals.var_beta * locals.var_dvbsibpc_dn8) * locals.var_xil) - (assign16340_e20323 * locals.var_xil_dn8)) / (locals.var_xil * locals.var_xil)), ((((locals.var_beta * locals.var_dvbsibpc_dn10) * locals.var_xil) - (assign16340_e20323 * locals.var_xil_dn10)) / (locals.var_xil * locals.var_xil)), ((((locals.var_beta * locals.var_dvbsibpc_dn11) * locals.var_xil) - (assign16340_e20323 * locals.var_xil_dn11)) / (locals.var_xil * locals.var_xil)), ((((locals.var_beta * locals.var_dvbsibpc_dn12) * locals.var_xil) - (assign16340_e20323 * locals.var_xil_dn12)) / (locals.var_xil * locals.var_xil)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign16340_e20327;
        locals.var_t2_dn0 = assign16340_e20327_d_n0;
        locals.var_t2_dn2 = assign16340_e20327_d_n2;
        locals.var_t2_dn4 = assign16340_e20327_d_n4;
        locals.var_t2_dn5 = assign16340_e20327_d_n5;
        locals.var_t2_dn6 = assign16340_e20327_d_n6;
        locals.var_t2_dn8 = assign16340_e20327_d_n8;
        locals.var_t2_dn10 = assign16340_e20327_d_n10;
        locals.var_t2_dn11 = assign16340_e20327_d_n11;
        locals.var_t2_dn12 = assign16340_e20327_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign16350_e20339, assign16350_e20339_d_n0, assign16350_e20339_d_n2, assign16350_e20339_d_n4, assign16350_e20339_d_n5, assign16350_e20339_d_n6, assign16350_e20339_d_n8, assign16350_e20339_d_n10, assign16350_e20339_d_n11, assign16350_e20339_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16350_e20332: f64 = (locals.var_xilp32 * locals.var_t2);
        let assign16350_e20335: f64 = (locals.var_xi0p32 * locals.var_t1);
        let assign16350_e20336: f64 = (assign16350_e20332 - assign16350_e20335);
        let assign16350_e20337: f64 = (locals.var_cnst0soi * assign16350_e20336);
        (assign16350_e20337, ((locals.var_cnst0soi_dn0 * assign16350_e20336) + (locals.var_cnst0soi * (((locals.var_xilp32_dn0 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn0)) - ((locals.var_xi0p32_dn0 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn0))))), ((locals.var_cnst0soi_dn2 * assign16350_e20336) + (locals.var_cnst0soi * (((locals.var_xilp32_dn2 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn2)) - ((locals.var_xi0p32_dn2 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn2))))), ((locals.var_cnst0soi_dn4 * assign16350_e20336) + (locals.var_cnst0soi * (((locals.var_xilp32_dn4 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn4)) - ((locals.var_xi0p32_dn4 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn4))))), ((locals.var_cnst0soi_dn5 * assign16350_e20336) + (locals.var_cnst0soi * (((locals.var_xilp32_dn5 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn5)) - ((locals.var_xi0p32_dn5 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn5))))), ((locals.var_cnst0soi_dn6 * assign16350_e20336) + (locals.var_cnst0soi * (((locals.var_xilp32_dn6 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn6)) - ((locals.var_xi0p32_dn6 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn6))))), ((locals.var_cnst0soi_dn8 * assign16350_e20336) + (locals.var_cnst0soi * (((locals.var_xilp32_dn8 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn8)) - ((locals.var_xi0p32_dn8 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn8))))), ((locals.var_cnst0soi_dn10 * assign16350_e20336) + (locals.var_cnst0soi * (((locals.var_xilp32_dn10 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn10)) - ((locals.var_xi0p32_dn10 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn10))))), ((locals.var_cnst0soi_dn11 * assign16350_e20336) + (locals.var_cnst0soi * (((locals.var_xilp32_dn11 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn11)) - ((locals.var_xi0p32_dn11 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn11))))), ((locals.var_cnst0soi_dn12 * assign16350_e20336) + (locals.var_cnst0soi * (((locals.var_xilp32_dn12 * locals.var_t2) + (locals.var_xilp32 * locals.var_t2_dn12)) - ((locals.var_xi0p32_dn12 * locals.var_t1) + (locals.var_xi0p32 * locals.var_t1_dn12))))),)
    } else {
        (locals.var_dg3, locals.var_dg3_dn0, locals.var_dg3_dn2, locals.var_dg3_dn4, locals.var_dg3_dn5, locals.var_dg3_dn6, locals.var_dg3_dn8, locals.var_dg3_dn10, locals.var_dg3_dn11, locals.var_dg3_dn12,)
    }
};
        locals.var_dg3 = assign16350_e20339;
        locals.var_dg3_dn0 = assign16350_e20339_d_n0;
        locals.var_dg3_dn2 = assign16350_e20339_d_n2;
        locals.var_dg3_dn4 = assign16350_e20339_d_n4;
        locals.var_dg3_dn5 = assign16350_e20339_d_n5;
        locals.var_dg3_dn6 = assign16350_e20339_d_n6;
        locals.var_dg3_dn8 = assign16350_e20339_d_n8;
        locals.var_dg3_dn10 = assign16350_e20339_d_n10;
        locals.var_dg3_dn11 = assign16350_e20339_d_n11;
        locals.var_dg3_dn12 = assign16350_e20339_d_n12;
        locals.var_dg3_rv = 0.0;

        let (assign16360_e20354, assign16360_e20354_d_n0, assign16360_e20354_d_n2, assign16360_e20354_d_n4, assign16360_e20354_d_n5, assign16360_e20354_d_n6, assign16360_e20354_d_n8, assign16360_e20354_d_n10, assign16360_e20354_d_n11, assign16360_e20354_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16360_e20343: f64 = (locals.var_cnst0soi * 0.5);
        let assign16360_e20345: f64 = (-locals.var_xilp12);
        let assign16360_e20347: f64 = (assign16360_e20345 * locals.var_t2);
        let assign16360_e20350: f64 = (locals.var_xi0p12 * locals.var_t1);
        let assign16360_e20351: f64 = (assign16360_e20347 + assign16360_e20350);
        let assign16360_e20352: f64 = (assign16360_e20343 * assign16360_e20351);
        (assign16360_e20352, (((locals.var_cnst0soi_dn0 * 0.5) * assign16360_e20351) + (assign16360_e20343 * ((((-locals.var_xilp12_dn0) * locals.var_t2) + (assign16360_e20345 * locals.var_t2_dn0)) + ((locals.var_xi0p12_dn0 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn0))))), (((locals.var_cnst0soi_dn2 * 0.5) * assign16360_e20351) + (assign16360_e20343 * ((((-locals.var_xilp12_dn2) * locals.var_t2) + (assign16360_e20345 * locals.var_t2_dn2)) + ((locals.var_xi0p12_dn2 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn2))))), (((locals.var_cnst0soi_dn4 * 0.5) * assign16360_e20351) + (assign16360_e20343 * ((((-locals.var_xilp12_dn4) * locals.var_t2) + (assign16360_e20345 * locals.var_t2_dn4)) + ((locals.var_xi0p12_dn4 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn4))))), (((locals.var_cnst0soi_dn5 * 0.5) * assign16360_e20351) + (assign16360_e20343 * ((((-locals.var_xilp12_dn5) * locals.var_t2) + (assign16360_e20345 * locals.var_t2_dn5)) + ((locals.var_xi0p12_dn5 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn5))))), (((locals.var_cnst0soi_dn6 * 0.5) * assign16360_e20351) + (assign16360_e20343 * ((((-locals.var_xilp12_dn6) * locals.var_t2) + (assign16360_e20345 * locals.var_t2_dn6)) + ((locals.var_xi0p12_dn6 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn6))))), (((locals.var_cnst0soi_dn8 * 0.5) * assign16360_e20351) + (assign16360_e20343 * ((((-locals.var_xilp12_dn8) * locals.var_t2) + (assign16360_e20345 * locals.var_t2_dn8)) + ((locals.var_xi0p12_dn8 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn8))))), (((locals.var_cnst0soi_dn10 * 0.5) * assign16360_e20351) + (assign16360_e20343 * ((((-locals.var_xilp12_dn10) * locals.var_t2) + (assign16360_e20345 * locals.var_t2_dn10)) + ((locals.var_xi0p12_dn10 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn10))))), (((locals.var_cnst0soi_dn11 * 0.5) * assign16360_e20351) + (assign16360_e20343 * ((((-locals.var_xilp12_dn11) * locals.var_t2) + (assign16360_e20345 * locals.var_t2_dn11)) + ((locals.var_xi0p12_dn11 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn11))))), (((locals.var_cnst0soi_dn12 * 0.5) * assign16360_e20351) + (assign16360_e20343 * ((((-locals.var_xilp12_dn12) * locals.var_t2) + (assign16360_e20345 * locals.var_t2_dn12)) + ((locals.var_xi0p12_dn12 * locals.var_t1) + (locals.var_xi0p12 * locals.var_t1_dn12))))),)
    } else {
        (locals.var_dg4, locals.var_dg4_dn0, locals.var_dg4_dn2, locals.var_dg4_dn4, locals.var_dg4_dn5, locals.var_dg4_dn6, locals.var_dg4_dn8, locals.var_dg4_dn10, locals.var_dg4_dn11, locals.var_dg4_dn12,)
    }
};
        locals.var_dg4 = assign16360_e20354;
        locals.var_dg4_dn0 = assign16360_e20354_d_n0;
        locals.var_dg4_dn2 = assign16360_e20354_d_n2;
        locals.var_dg4_dn4 = assign16360_e20354_d_n4;
        locals.var_dg4_dn5 = assign16360_e20354_d_n5;
        locals.var_dg4_dn6 = assign16360_e20354_d_n6;
        locals.var_dg4_dn8 = assign16360_e20354_d_n8;
        locals.var_dg4_dn10 = assign16360_e20354_d_n10;
        locals.var_dg4_dn11 = assign16360_e20354_d_n11;
        locals.var_dg4_dn12 = assign16360_e20354_d_n12;
        locals.var_dg4_rv = 0.0;

        let (assign16370_e20360, assign16370_e20360_d_n0, assign16370_e20360_d_n2, assign16370_e20360_d_n4, assign16370_e20360_d_n5, assign16370_e20360_d_n6, assign16370_e20360_d_n8, assign16370_e20360_d_n10, assign16370_e20360_d_n11, assign16370_e20360_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16370_e20358: f64 = (locals.var_dg3 + locals.var_dg4);
        (assign16370_e20358, (locals.var_dg3_dn0 + locals.var_dg4_dn0), (locals.var_dg3_dn2 + locals.var_dg4_dn2), (locals.var_dg3_dn4 + locals.var_dg4_dn4), (locals.var_dg3_dn5 + locals.var_dg4_dn5), (locals.var_dg3_dn6 + locals.var_dg4_dn6), (locals.var_dg3_dn8 + locals.var_dg4_dn8), (locals.var_dg3_dn10 + locals.var_dg4_dn10), (locals.var_dg3_dn11 + locals.var_dg4_dn11), (locals.var_dg3_dn12 + locals.var_dg4_dn12),)
    } else {
        (locals.var_didd, locals.var_didd_dn0, locals.var_didd_dn2, locals.var_didd_dn4, locals.var_didd_dn5, locals.var_didd_dn6, locals.var_didd_dn8, locals.var_didd_dn10, locals.var_didd_dn11, locals.var_didd_dn12,)
    }
};
        locals.var_didd = assign16370_e20360;
        locals.var_didd_dn0 = assign16370_e20360_d_n0;
        locals.var_didd_dn2 = assign16370_e20360_d_n2;
        locals.var_didd_dn4 = assign16370_e20360_d_n4;
        locals.var_didd_dn5 = assign16370_e20360_d_n5;
        locals.var_didd_dn6 = assign16370_e20360_d_n6;
        locals.var_didd_dn8 = assign16370_e20360_d_n8;
        locals.var_didd_dn10 = assign16370_e20360_d_n10;
        locals.var_didd_dn11 = assign16370_e20360_d_n11;
        locals.var_didd_dn12 = assign16370_e20360_d_n12;
        locals.var_didd_rv = 0.0;

        let (assign16380_e20368, assign16380_e20368_d_n0, assign16380_e20368_d_n2, assign16380_e20368_d_n4, assign16380_e20368_d_n5, assign16380_e20368_d_n6, assign16380_e20368_d_n8, assign16380_e20368_d_n10, assign16380_e20368_d_n11, assign16380_e20368_d_n12,) = {
    if (locals.var_guard282 != 0.0) {
        let assign16380_e20364: f64 = (locals.var_betawl * locals.var_didd);
        let assign16380_e20366: f64 = (assign16380_e20364 * locals.var_mu);
        (assign16380_e20366, ((((locals.var_betawl_dn0 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn0)) * locals.var_mu) + (assign16380_e20364 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn2)) * locals.var_mu) + (assign16380_e20364 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn4)) * locals.var_mu) + (assign16380_e20364 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn5)) * locals.var_mu) + (assign16380_e20364 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn6)) * locals.var_mu) + (assign16380_e20364 * locals.var_mu_dn6)), ((((locals.var_betawl_dn8 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn8)) * locals.var_mu) + (assign16380_e20364 * locals.var_mu_dn8)), ((((locals.var_betawl_dn10 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn10)) * locals.var_mu) + (assign16380_e20364 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn11)) * locals.var_mu) + (assign16380_e20364 * locals.var_mu_dn11)), ((((locals.var_betawl_dn12 * locals.var_didd) + (locals.var_betawl * locals.var_didd_dn12)) * locals.var_mu) + (assign16380_e20364 * locals.var_mu_dn12)),)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn8, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn12,)
    }
};
        locals.var_idsibpc = assign16380_e20368;
        locals.var_idsibpc_dn0 = assign16380_e20368_d_n0;
        locals.var_idsibpc_dn2 = assign16380_e20368_d_n2;
        locals.var_idsibpc_dn4 = assign16380_e20368_d_n4;
        locals.var_idsibpc_dn5 = assign16380_e20368_d_n5;
        locals.var_idsibpc_dn6 = assign16380_e20368_d_n6;
        locals.var_idsibpc_dn8 = assign16380_e20368_d_n8;
        locals.var_idsibpc_dn10 = assign16380_e20368_d_n10;
        locals.var_idsibpc_dn11 = assign16380_e20368_d_n11;
        locals.var_idsibpc_dn12 = assign16380_e20368_d_n12;
        locals.var_idsibpc_rv = 0.0;

        let assign16390_e20371: f64 = (locals.var_tfox0 * 100.0);
        locals.var_cgs_tfox0__blk298 = assign16390_e20371;
        locals.var_cgs_tfox0__blk298_rv = 0.0;

        let assign16410_e20377: f64 = (locals.var_leff * 100.0);
        locals.var_cgs_leff__blk300 = assign16410_e20377;
        locals.var_cgs_leff__blk300_dn0 = (locals.var_leff_dn0 * 100.0);
        locals.var_cgs_leff__blk300_dn2 = (locals.var_leff_dn2 * 100.0);
        locals.var_cgs_leff__blk300_dn4 = (locals.var_leff_dn4 * 100.0);
        locals.var_cgs_leff__blk300_dn5 = (locals.var_leff_dn5 * 100.0);
        locals.var_cgs_leff__blk300_dn6 = (locals.var_leff_dn6 * 100.0);
        locals.var_cgs_leff__blk300_dn8 = (locals.var_leff_dn8 * 100.0);
        locals.var_cgs_leff__blk300_dn10 = (locals.var_leff_dn10 * 100.0);
        locals.var_cgs_leff__blk300_dn11 = (locals.var_leff_dn11 * 100.0);
        locals.var_cgs_leff__blk300_dn12 = (locals.var_leff_dn12 * 100.0);
        locals.var_cgs_leff__blk300_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_67(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign16420_e20380: f64 = (locals.var_weff_nf * 100.0);
        locals.var_cgs_weff_nf__blk301 = assign16420_e20380;
        locals.var_cgs_weff_nf__blk301_dn0 = (locals.var_weff_nf_dn0 * 100.0);
        locals.var_cgs_weff_nf__blk301_dn2 = (locals.var_weff_nf_dn2 * 100.0);
        locals.var_cgs_weff_nf__blk301_dn4 = (locals.var_weff_nf_dn4 * 100.0);
        locals.var_cgs_weff_nf__blk301_dn5 = (locals.var_weff_nf_dn5 * 100.0);
        locals.var_cgs_weff_nf__blk301_dn6 = (locals.var_weff_nf_dn6 * 100.0);
        locals.var_cgs_weff_nf__blk301_dn8 = (locals.var_weff_nf_dn8 * 100.0);
        locals.var_cgs_weff_nf__blk301_dn10 = (locals.var_weff_nf_dn10 * 100.0);
        locals.var_cgs_weff_nf__blk301_dn11 = (locals.var_weff_nf_dn11 * 100.0);
        locals.var_cgs_weff_nf__blk301_dn12 = (locals.var_weff_nf_dn12 * 100.0);
        locals.var_cgs_weff_nf__blk301_rv = 0.0;

        let assign16430_e20383: f64 = (locals.var_ey / 100.0);
        locals.var_cgs_ey = assign16430_e20383;
        locals.var_cgs_ey_dn0 = (locals.var_ey_dn0 / 100.0);
        locals.var_cgs_ey_dn2 = (locals.var_ey_dn2 / 100.0);
        locals.var_cgs_ey_dn4 = (locals.var_ey_dn4 / 100.0);
        locals.var_cgs_ey_dn5 = (locals.var_ey_dn5 / 100.0);
        locals.var_cgs_ey_dn6 = (locals.var_ey_dn6 / 100.0);
        locals.var_cgs_ey_dn8 = (locals.var_ey_dn8 / 100.0);
        locals.var_cgs_ey_dn10 = (locals.var_ey_dn10 / 100.0);
        locals.var_cgs_ey_dn11 = (locals.var_ey_dn11 / 100.0);
        locals.var_cgs_ey_dn12 = (locals.var_ey_dn12 / 100.0);
        locals.var_cgs_ey_rv = 0.0;

        let assign16460_e20392: f64 = if p.p17 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard305 = assign16460_e20392;
        locals.var_guard305_rv = 0.0;

        let (assign16510_e20412,) = {
    if (locals.var_guard305 != 0.0) {
        (0.0,)
    } else {
        (locals.var_glpart1,)
    }
};
        locals.var_glpart1 = assign16510_e20412;
        locals.var_glpart1_rv = 0.0;

        let assign16520_e20415: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard306 = assign16520_e20415;
        locals.var_guard306_rv = 0.0;

        let (assign16530_e20428, assign16530_e20428_d_n0, assign16530_e20428_d_n2, assign16530_e20428_d_n4, assign16530_e20428_d_n5, assign16530_e20428_d_n6, assign16530_e20428_d_n8, assign16530_e20428_d_n10, assign16530_e20428_d_n11, assign16530_e20428_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16530_e20422: f64 = (locals.var_ps0z + locals.var_vdsz);
        let assign16530_e20425: f64 = (10.0 * 2.220446049250313e-16);
        let assign16530_e20426: f64 = (assign16530_e20422 - assign16530_e20425);
        (assign16530_e20426, (locals.var_ps0z_dn0 + locals.var_vdsz_dn0), (locals.var_ps0z_dn2 + locals.var_vdsz_dn2), (locals.var_ps0z_dn4 + locals.var_vdsz_dn4), (locals.var_ps0z_dn5 + locals.var_vdsz_dn5), (locals.var_ps0z_dn6 + locals.var_vdsz_dn6), (locals.var_ps0z_dn8 + locals.var_vdsz_dn8), (locals.var_ps0z_dn10 + locals.var_vdsz_dn10), (locals.var_ps0z_dn11 + locals.var_vdsz_dn11), (locals.var_ps0z_dn12 + locals.var_vdsz_dn12),)
    } else {
        (locals.var_psdlz, locals.var_psdlz_dn0, locals.var_psdlz_dn2, locals.var_psdlz_dn4, locals.var_psdlz_dn5, locals.var_psdlz_dn6, locals.var_psdlz_dn8, locals.var_psdlz_dn10, locals.var_psdlz_dn11, locals.var_psdlz_dn12,)
    }
};
        locals.var_psdlz = assign16530_e20428;
        locals.var_psdlz_dn0 = assign16530_e20428_d_n0;
        locals.var_psdlz_dn2 = assign16530_e20428_d_n2;
        locals.var_psdlz_dn4 = assign16530_e20428_d_n4;
        locals.var_psdlz_dn5 = assign16530_e20428_d_n5;
        locals.var_psdlz_dn6 = assign16530_e20428_d_n6;
        locals.var_psdlz_dn8 = assign16530_e20428_d_n8;
        locals.var_psdlz_dn10 = assign16530_e20428_d_n10;
        locals.var_psdlz_dn11 = assign16530_e20428_d_n11;
        locals.var_psdlz_dn12 = assign16530_e20428_d_n12;
        locals.var_psdlz_rv = 0.0;

        let (assign16540_e20456, assign16540_e20456_d_n0, assign16540_e20456_d_n2, assign16540_e20456_d_n4, assign16540_e20456_d_n5, assign16540_e20456_d_n6, assign16540_e20456_d_n8, assign16540_e20456_d_n10, assign16540_e20456_d_n11, assign16540_e20456_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16540_e20436: f64 = (p.p256 * locals.var_vfb);
        let assign16540_e20437: f64 = (locals.var_vgsz - assign16540_e20436);
        let assign16540_e20439: f64 = (-p.p258);
        let assign16540_e20441: f64 = (assign16540_e20439 * locals.var_vbs);
        let assign16540_e20445: f64 = (locals.var_dvth - locals.var_dppg);
        let assign16540_e20446: f64 = (p.p206 * assign16540_e20445);
        let assign16540_e20447: f64 = (assign16540_e20441 + assign16540_e20446);
        let assign16540_e20449: f64 = (assign16540_e20447 / locals.var_cgs_leff__blk300);
        let assign16540_e20450: f64 = (assign16540_e20437 + assign16540_e20449);
        let assign16540_e20453: f64 = (locals.var_psdlz * p.p205);
        let assign16540_e20454: f64 = (assign16540_e20450 - assign16540_e20453);
        (assign16540_e20454, (((locals.var_vgsz_dn0 - (p.p256 * locals.var_vfb_dn0)) + (((((assign16540_e20439 * locals.var_vbs_dn0) + (p.p206 * (locals.var_dvth_dn0 - locals.var_dppg_dn0))) * locals.var_cgs_leff__blk300) - (assign16540_e20447 * locals.var_cgs_leff__blk300_dn0)) / (locals.var_cgs_leff__blk300 * locals.var_cgs_leff__blk300))) - (locals.var_psdlz_dn0 * p.p205)), (((locals.var_vgsz_dn2 - (p.p256 * locals.var_vfb_dn2)) + (((((assign16540_e20439 * locals.var_vbs_dn2) + (p.p206 * (locals.var_dvth_dn2 - locals.var_dppg_dn2))) * locals.var_cgs_leff__blk300) - (assign16540_e20447 * locals.var_cgs_leff__blk300_dn2)) / (locals.var_cgs_leff__blk300 * locals.var_cgs_leff__blk300))) - (locals.var_psdlz_dn2 * p.p205)), (((locals.var_vgsz_dn4 - (p.p256 * locals.var_vfb_dn4)) + (((((assign16540_e20439 * locals.var_vbs_dn4) + (p.p206 * (locals.var_dvth_dn4 - locals.var_dppg_dn4))) * locals.var_cgs_leff__blk300) - (assign16540_e20447 * locals.var_cgs_leff__blk300_dn4)) / (locals.var_cgs_leff__blk300 * locals.var_cgs_leff__blk300))) - (locals.var_psdlz_dn4 * p.p205)), (((locals.var_vgsz_dn5 - (p.p256 * locals.var_vfb_dn5)) + (((((assign16540_e20439 * locals.var_vbs_dn5) + (p.p206 * (locals.var_dvth_dn5 - locals.var_dppg_dn5))) * locals.var_cgs_leff__blk300) - (assign16540_e20447 * locals.var_cgs_leff__blk300_dn5)) / (locals.var_cgs_leff__blk300 * locals.var_cgs_leff__blk300))) - (locals.var_psdlz_dn5 * p.p205)), (((locals.var_vgsz_dn6 - (p.p256 * locals.var_vfb_dn6)) + (((((assign16540_e20439 * locals.var_vbs_dn6) + (p.p206 * (locals.var_dvth_dn6 - locals.var_dppg_dn6))) * locals.var_cgs_leff__blk300) - (assign16540_e20447 * locals.var_cgs_leff__blk300_dn6)) / (locals.var_cgs_leff__blk300 * locals.var_cgs_leff__blk300))) - (locals.var_psdlz_dn6 * p.p205)), (((locals.var_vgsz_dn8 - (p.p256 * locals.var_vfb_dn8)) + (((((assign16540_e20439 * locals.var_vbs_dn8) + (p.p206 * (locals.var_dvth_dn8 - locals.var_dppg_dn8))) * locals.var_cgs_leff__blk300) - (assign16540_e20447 * locals.var_cgs_leff__blk300_dn8)) / (locals.var_cgs_leff__blk300 * locals.var_cgs_leff__blk300))) - (locals.var_psdlz_dn8 * p.p205)), (((locals.var_vgsz_dn10 - (p.p256 * locals.var_vfb_dn10)) + (((((assign16540_e20439 * locals.var_vbs_dn10) + (p.p206 * (locals.var_dvth_dn10 - locals.var_dppg_dn10))) * locals.var_cgs_leff__blk300) - (assign16540_e20447 * locals.var_cgs_leff__blk300_dn10)) / (locals.var_cgs_leff__blk300 * locals.var_cgs_leff__blk300))) - (locals.var_psdlz_dn10 * p.p205)), (((locals.var_vgsz_dn11 - (p.p256 * locals.var_vfb_dn11)) + (((((assign16540_e20439 * locals.var_vbs_dn11) + (p.p206 * (locals.var_dvth_dn11 - locals.var_dppg_dn11))) * locals.var_cgs_leff__blk300) - (assign16540_e20447 * locals.var_cgs_leff__blk300_dn11)) / (locals.var_cgs_leff__blk300 * locals.var_cgs_leff__blk300))) - (locals.var_psdlz_dn11 * p.p205)), (((locals.var_vgsz_dn12 - (p.p256 * locals.var_vfb_dn12)) + (((((assign16540_e20439 * locals.var_vbs_dn12) + (p.p206 * (locals.var_dvth_dn12 - locals.var_dppg_dn12))) * locals.var_cgs_leff__blk300) - (assign16540_e20447 * locals.var_cgs_leff__blk300_dn12)) / (locals.var_cgs_leff__blk300 * locals.var_cgs_leff__blk300))) - (locals.var_psdlz_dn12 * p.p205)),)
    } else {
        (locals.var_t1__blk287, locals.var_t1__blk287_dn0, locals.var_t1__blk287_dn2, locals.var_t1__blk287_dn4, locals.var_t1__blk287_dn5, locals.var_t1__blk287_dn6, locals.var_t1__blk287_dn8, locals.var_t1__blk287_dn10, locals.var_t1__blk287_dn11, locals.var_t1__blk287_dn12,)
    }
};
        locals.var_t1__blk287 = assign16540_e20456;
        locals.var_t1__blk287_dn0 = assign16540_e20456_d_n0;
        locals.var_t1__blk287_dn2 = assign16540_e20456_d_n2;
        locals.var_t1__blk287_dn4 = assign16540_e20456_d_n4;
        locals.var_t1__blk287_dn5 = assign16540_e20456_d_n5;
        locals.var_t1__blk287_dn6 = assign16540_e20456_d_n6;
        locals.var_t1__blk287_dn8 = assign16540_e20456_d_n8;
        locals.var_t1__blk287_dn10 = assign16540_e20456_d_n10;
        locals.var_t1__blk287_dn11 = assign16540_e20456_d_n11;
        locals.var_t1__blk287_dn12 = assign16540_e20456_d_n12;
        locals.var_t1__blk287_rv = 0.0;

        let (assign16550_e20467, assign16550_e20467_d_n0, assign16550_e20467_d_n2, assign16550_e20467_d_n4, assign16550_e20467_d_n5, assign16550_e20467_d_n6, assign16550_e20467_d_n8, assign16550_e20467_d_n10, assign16550_e20467_d_n11, assign16550_e20467_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16550_e20464: f64 = (locals.var_cgs_ey / p.p207);
        let assign16550_e20465: f64 = (1.0 + assign16550_e20464);
        (assign16550_e20465, (locals.var_cgs_ey_dn0 / p.p207), (locals.var_cgs_ey_dn2 / p.p207), (locals.var_cgs_ey_dn4 / p.p207), (locals.var_cgs_ey_dn5 / p.p207), (locals.var_cgs_ey_dn6 / p.p207), (locals.var_cgs_ey_dn8 / p.p207), (locals.var_cgs_ey_dn10 / p.p207), (locals.var_cgs_ey_dn11 / p.p207), (locals.var_cgs_ey_dn12 / p.p207),)
    } else {
        (locals.var_t7__blk293, locals.var_t7__blk293_dn0, locals.var_t7__blk293_dn2, locals.var_t7__blk293_dn4, locals.var_t7__blk293_dn5, locals.var_t7__blk293_dn6, locals.var_t7__blk293_dn8, locals.var_t7__blk293_dn10, locals.var_t7__blk293_dn11, locals.var_t7__blk293_dn12,)
    }
};
        locals.var_t7__blk293 = assign16550_e20467;
        locals.var_t7__blk293_dn0 = assign16550_e20467_d_n0;
        locals.var_t7__blk293_dn2 = assign16550_e20467_d_n2;
        locals.var_t7__blk293_dn4 = assign16550_e20467_d_n4;
        locals.var_t7__blk293_dn5 = assign16550_e20467_d_n5;
        locals.var_t7__blk293_dn6 = assign16550_e20467_d_n6;
        locals.var_t7__blk293_dn8 = assign16550_e20467_d_n8;
        locals.var_t7__blk293_dn10 = assign16550_e20467_d_n10;
        locals.var_t7__blk293_dn11 = assign16550_e20467_d_n11;
        locals.var_t7__blk293_dn12 = assign16550_e20467_d_n12;
        locals.var_t7__blk293_rv = 0.0;

        let (assign16560_e20478, assign16560_e20478_d_n0, assign16560_e20478_d_n2, assign16560_e20478_d_n4, assign16560_e20478_d_n5, assign16560_e20478_d_n6, assign16560_e20478_d_n8, assign16560_e20478_d_n10, assign16560_e20478_d_n11, assign16560_e20478_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16560_e20474: f64 = (locals.var_t7__blk293 * locals.var_t1__blk287);
        let assign16560_e20476: f64 = (assign16560_e20474 / locals.var_cgs_tfox0__blk298);
        (assign16560_e20476, (((locals.var_t7__blk293_dn0 * locals.var_t1__blk287) + (locals.var_t7__blk293 * locals.var_t1__blk287_dn0)) / locals.var_cgs_tfox0__blk298), (((locals.var_t7__blk293_dn2 * locals.var_t1__blk287) + (locals.var_t7__blk293 * locals.var_t1__blk287_dn2)) / locals.var_cgs_tfox0__blk298), (((locals.var_t7__blk293_dn4 * locals.var_t1__blk287) + (locals.var_t7__blk293 * locals.var_t1__blk287_dn4)) / locals.var_cgs_tfox0__blk298), (((locals.var_t7__blk293_dn5 * locals.var_t1__blk287) + (locals.var_t7__blk293 * locals.var_t1__blk287_dn5)) / locals.var_cgs_tfox0__blk298), (((locals.var_t7__blk293_dn6 * locals.var_t1__blk287) + (locals.var_t7__blk293 * locals.var_t1__blk287_dn6)) / locals.var_cgs_tfox0__blk298), (((locals.var_t7__blk293_dn8 * locals.var_t1__blk287) + (locals.var_t7__blk293 * locals.var_t1__blk287_dn8)) / locals.var_cgs_tfox0__blk298), (((locals.var_t7__blk293_dn10 * locals.var_t1__blk287) + (locals.var_t7__blk293 * locals.var_t1__blk287_dn10)) / locals.var_cgs_tfox0__blk298), (((locals.var_t7__blk293_dn11 * locals.var_t1__blk287) + (locals.var_t7__blk293 * locals.var_t1__blk287_dn11)) / locals.var_cgs_tfox0__blk298), (((locals.var_t7__blk293_dn12 * locals.var_t1__blk287) + (locals.var_t7__blk293 * locals.var_t1__blk287_dn12)) / locals.var_cgs_tfox0__blk298),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign16560_e20478;
        locals.var_etun_dn0 = assign16560_e20478_d_n0;
        locals.var_etun_dn2 = assign16560_e20478_d_n2;
        locals.var_etun_dn4 = assign16560_e20478_d_n4;
        locals.var_etun_dn5 = assign16560_e20478_d_n5;
        locals.var_etun_dn6 = assign16560_e20478_d_n6;
        locals.var_etun_dn8 = assign16560_e20478_d_n8;
        locals.var_etun_dn10 = assign16560_e20478_d_n10;
        locals.var_etun_dn11 = assign16560_e20478_d_n11;
        locals.var_etun_dn12 = assign16560_e20478_d_n12;
        locals.var_etun_rv = 0.0;

        let (assign16570_e20494, assign16570_e20494_d_n0, assign16570_e20494_d_n2, assign16570_e20494_d_n4, assign16570_e20494_d_n5, assign16570_e20494_d_n6, assign16570_e20494_d_n8, assign16570_e20494_d_n10, assign16570_e20494_d_n11, assign16570_e20494_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16570_e20485: f64 = (locals.var_etun * locals.var_etun);
        let assign16570_e20488: f64 = (4.0 * 0.01);
        let assign16570_e20490: f64 = (assign16570_e20488 * 0.01);
        let assign16570_e20491: f64 = (assign16570_e20485 + assign16570_e20490);
        let assign16570_e20492: f64 = (assign16570_e20491).sqrt();
        (assign16570_e20492, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign16570_e20492)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign16570_e20492)), (((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4)) / (2.0 * assign16570_e20492)), (((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5)) / (2.0 * assign16570_e20492)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign16570_e20492)), (((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8)) / (2.0 * assign16570_e20492)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign16570_e20492)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign16570_e20492)), (((locals.var_etun_dn12 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn12)) / (2.0 * assign16570_e20492)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign16570_e20494;
        locals.var_tmf2_dn0 = assign16570_e20494_d_n0;
        locals.var_tmf2_dn2 = assign16570_e20494_d_n2;
        locals.var_tmf2_dn4 = assign16570_e20494_d_n4;
        locals.var_tmf2_dn5 = assign16570_e20494_d_n5;
        locals.var_tmf2_dn6 = assign16570_e20494_d_n6;
        locals.var_tmf2_dn8 = assign16570_e20494_d_n8;
        locals.var_tmf2_dn10 = assign16570_e20494_d_n10;
        locals.var_tmf2_dn11 = assign16570_e20494_d_n11;
        locals.var_tmf2_dn12 = assign16570_e20494_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign16580_e20507, assign16580_e20507_d_n0, assign16580_e20507_d_n2, assign16580_e20507_d_n4, assign16580_e20507_d_n5, assign16580_e20507_d_n6, assign16580_e20507_d_n8, assign16580_e20507_d_n10, assign16580_e20507_d_n11, assign16580_e20507_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16580_e20503: f64 = (locals.var_etun / locals.var_tmf2);
        let assign16580_e20504: f64 = (1.0 + assign16580_e20503);
        let assign16580_e20505: f64 = (0.5 * assign16580_e20504);
        (assign16580_e20505, (0.5 * (((locals.var_etun_dn0 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn2 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn4 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn5 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn6 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn8 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn10 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn11 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn12 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5__blk291, locals.var_t5__blk291_dn0, locals.var_t5__blk291_dn2, locals.var_t5__blk291_dn4, locals.var_t5__blk291_dn5, locals.var_t5__blk291_dn6, locals.var_t5__blk291_dn8, locals.var_t5__blk291_dn10, locals.var_t5__blk291_dn11, locals.var_t5__blk291_dn12,)
    }
};
        locals.var_t5__blk291 = assign16580_e20507;
        locals.var_t5__blk291_dn0 = assign16580_e20507_d_n0;
        locals.var_t5__blk291_dn2 = assign16580_e20507_d_n2;
        locals.var_t5__blk291_dn4 = assign16580_e20507_d_n4;
        locals.var_t5__blk291_dn5 = assign16580_e20507_d_n5;
        locals.var_t5__blk291_dn6 = assign16580_e20507_d_n6;
        locals.var_t5__blk291_dn8 = assign16580_e20507_d_n8;
        locals.var_t5__blk291_dn10 = assign16580_e20507_d_n10;
        locals.var_t5__blk291_dn11 = assign16580_e20507_d_n11;
        locals.var_t5__blk291_dn12 = assign16580_e20507_d_n12;
        locals.var_t5__blk291_rv = 0.0;

        let (assign16590_e20522, assign16590_e20522_d_n0, assign16590_e20522_d_n2, assign16590_e20522_d_n4, assign16590_e20522_d_n5, assign16590_e20522_d_n6, assign16590_e20522_d_n8, assign16590_e20522_d_n10, assign16590_e20522_d_n11, assign16590_e20522_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16590_e20515: f64 = (locals.var_etun + locals.var_tmf2);
        let assign16590_e20516: f64 = (0.5 * assign16590_e20515);
        let assign16590_e20519: f64 = (1e-10 * 0.01);
        let assign16590_e20520: f64 = (assign16590_e20516 + assign16590_e20519);
        (assign16590_e20520, (0.5 * (locals.var_etun_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_etun_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_etun_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_etun_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_etun_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign16590_e20522;
        locals.var_etun_dn0 = assign16590_e20522_d_n0;
        locals.var_etun_dn2 = assign16590_e20522_d_n2;
        locals.var_etun_dn4 = assign16590_e20522_d_n4;
        locals.var_etun_dn5 = assign16590_e20522_d_n5;
        locals.var_etun_dn6 = assign16590_e20522_d_n6;
        locals.var_etun_dn8 = assign16590_e20522_d_n8;
        locals.var_etun_dn10 = assign16590_e20522_d_n10;
        locals.var_etun_dn11 = assign16590_e20522_d_n11;
        locals.var_etun_dn12 = assign16590_e20522_d_n12;
        locals.var_etun_rv = 0.0;

        let assign16600_e20525: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign16600_e20525;
        locals.var_guard307_rv = 0.0;

        let (assign16610_e20534, assign16610_e20534_d_n0, assign16610_e20534_d_n2, assign16610_e20534_d_n4, assign16610_e20534_d_n5, assign16610_e20534_d_n6, assign16610_e20534_d_n8, assign16610_e20534_d_n10, assign16610_e20534_d_n11, assign16610_e20534_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) && (locals.var_guard307 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign16610_e20534;
        locals.var_etun_dn0 = assign16610_e20534_d_n0;
        locals.var_etun_dn2 = assign16610_e20534_d_n2;
        locals.var_etun_dn4 = assign16610_e20534_d_n4;
        locals.var_etun_dn5 = assign16610_e20534_d_n5;
        locals.var_etun_dn6 = assign16610_e20534_d_n6;
        locals.var_etun_dn8 = assign16610_e20534_d_n8;
        locals.var_etun_dn10 = assign16610_e20534_d_n10;
        locals.var_etun_dn11 = assign16610_e20534_d_n11;
        locals.var_etun_dn12 = assign16610_e20534_d_n12;
        locals.var_etun_rv = 0.0;

        let (assign16620_e20543, assign16620_e20543_d_n0, assign16620_e20543_d_n2, assign16620_e20543_d_n4, assign16620_e20543_d_n5, assign16620_e20543_d_n6, assign16620_e20543_d_n8, assign16620_e20543_d_n10, assign16620_e20543_d_n11, assign16620_e20543_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) && (locals.var_guard307 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk291, locals.var_t5__blk291_dn0, locals.var_t5__blk291_dn2, locals.var_t5__blk291_dn4, locals.var_t5__blk291_dn5, locals.var_t5__blk291_dn6, locals.var_t5__blk291_dn8, locals.var_t5__blk291_dn10, locals.var_t5__blk291_dn11, locals.var_t5__blk291_dn12,)
    }
};
        locals.var_t5__blk291 = assign16620_e20543;
        locals.var_t5__blk291_dn0 = assign16620_e20543_d_n0;
        locals.var_t5__blk291_dn2 = assign16620_e20543_d_n2;
        locals.var_t5__blk291_dn4 = assign16620_e20543_d_n4;
        locals.var_t5__blk291_dn5 = assign16620_e20543_d_n5;
        locals.var_t5__blk291_dn6 = assign16620_e20543_d_n6;
        locals.var_t5__blk291_dn8 = assign16620_e20543_d_n8;
        locals.var_t5__blk291_dn10 = assign16620_e20543_d_n10;
        locals.var_t5__blk291_dn11 = assign16620_e20543_d_n11;
        locals.var_t5__blk291_dn12 = assign16620_e20543_d_n12;
        locals.var_t5__blk291_rv = 0.0;

        let (assign16630_e20559, assign16630_e20559_d_n0, assign16630_e20559_d_n2, assign16630_e20559_d_n4, assign16630_e20559_d_n5, assign16630_e20559_d_n6, assign16630_e20559_d_n8, assign16630_e20559_d_n10, assign16630_e20559_d_n11, assign16630_e20559_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16630_e20550: f64 = (locals.var_vgsz * locals.var_vgsz);
        let assign16630_e20553: f64 = (4.0 * 0.001);
        let assign16630_e20555: f64 = (assign16630_e20553 * 0.001);
        let assign16630_e20556: f64 = (assign16630_e20550 + assign16630_e20555);
        let assign16630_e20557: f64 = (assign16630_e20556).sqrt();
        (assign16630_e20557, (((locals.var_vgsz_dn0 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn0)) / (2.0 * assign16630_e20557)), (((locals.var_vgsz_dn2 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn2)) / (2.0 * assign16630_e20557)), (((locals.var_vgsz_dn4 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn4)) / (2.0 * assign16630_e20557)), (((locals.var_vgsz_dn5 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn5)) / (2.0 * assign16630_e20557)), (((locals.var_vgsz_dn6 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn6)) / (2.0 * assign16630_e20557)), (((locals.var_vgsz_dn8 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn8)) / (2.0 * assign16630_e20557)), (((locals.var_vgsz_dn10 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn10)) / (2.0 * assign16630_e20557)), (((locals.var_vgsz_dn11 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn11)) / (2.0 * assign16630_e20557)), (((locals.var_vgsz_dn12 * locals.var_vgsz) + (locals.var_vgsz * locals.var_vgsz_dn12)) / (2.0 * assign16630_e20557)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign16630_e20559;
        locals.var_tmf2_dn0 = assign16630_e20559_d_n0;
        locals.var_tmf2_dn2 = assign16630_e20559_d_n2;
        locals.var_tmf2_dn4 = assign16630_e20559_d_n4;
        locals.var_tmf2_dn5 = assign16630_e20559_d_n5;
        locals.var_tmf2_dn6 = assign16630_e20559_d_n6;
        locals.var_tmf2_dn8 = assign16630_e20559_d_n8;
        locals.var_tmf2_dn10 = assign16630_e20559_d_n10;
        locals.var_tmf2_dn11 = assign16630_e20559_d_n11;
        locals.var_tmf2_dn12 = assign16630_e20559_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign16640_e20572, assign16640_e20572_d_n0, assign16640_e20572_d_n2, assign16640_e20572_d_n4, assign16640_e20572_d_n5, assign16640_e20572_d_n6, assign16640_e20572_d_n8, assign16640_e20572_d_n10, assign16640_e20572_d_n11, assign16640_e20572_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16640_e20568: f64 = (locals.var_vgsz / locals.var_tmf2);
        let assign16640_e20569: f64 = (1.0 + assign16640_e20568);
        let assign16640_e20570: f64 = (0.5 * assign16640_e20569);
        (assign16640_e20570, (0.5 * (((locals.var_vgsz_dn0 * locals.var_tmf2) - (locals.var_vgsz * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz_dn2 * locals.var_tmf2) - (locals.var_vgsz * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz_dn4 * locals.var_tmf2) - (locals.var_vgsz * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz_dn5 * locals.var_tmf2) - (locals.var_vgsz * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz_dn6 * locals.var_tmf2) - (locals.var_vgsz * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz_dn8 * locals.var_tmf2) - (locals.var_vgsz * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz_dn10 * locals.var_tmf2) - (locals.var_vgsz * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz_dn11 * locals.var_tmf2) - (locals.var_vgsz * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vgsz_dn12 * locals.var_tmf2) - (locals.var_vgsz * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4__blk290, locals.var_t4__blk290_dn0, locals.var_t4__blk290_dn2, locals.var_t4__blk290_dn4, locals.var_t4__blk290_dn5, locals.var_t4__blk290_dn6, locals.var_t4__blk290_dn8, locals.var_t4__blk290_dn10, locals.var_t4__blk290_dn11, locals.var_t4__blk290_dn12,)
    }
};
        locals.var_t4__blk290 = assign16640_e20572;
        locals.var_t4__blk290_dn0 = assign16640_e20572_d_n0;
        locals.var_t4__blk290_dn2 = assign16640_e20572_d_n2;
        locals.var_t4__blk290_dn4 = assign16640_e20572_d_n4;
        locals.var_t4__blk290_dn5 = assign16640_e20572_d_n5;
        locals.var_t4__blk290_dn6 = assign16640_e20572_d_n6;
        locals.var_t4__blk290_dn8 = assign16640_e20572_d_n8;
        locals.var_t4__blk290_dn10 = assign16640_e20572_d_n10;
        locals.var_t4__blk290_dn11 = assign16640_e20572_d_n11;
        locals.var_t4__blk290_dn12 = assign16640_e20572_d_n12;
        locals.var_t4__blk290_rv = 0.0;

        let (assign16650_e20587, assign16650_e20587_d_n0, assign16650_e20587_d_n2, assign16650_e20587_d_n4, assign16650_e20587_d_n5, assign16650_e20587_d_n6, assign16650_e20587_d_n8, assign16650_e20587_d_n10, assign16650_e20587_d_n11, assign16650_e20587_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16650_e20580: f64 = (locals.var_vgsz + locals.var_tmf2);
        let assign16650_e20581: f64 = (0.5 * assign16650_e20580);
        let assign16650_e20584: f64 = (1e-10 * 0.001);
        let assign16650_e20585: f64 = (assign16650_e20581 + assign16650_e20584);
        (assign16650_e20585, (0.5 * (locals.var_vgsz_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vgsz_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vgsz_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vgsz_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vgsz_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vgsz_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vgsz_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vgsz_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vgsz_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign16650_e20587;
        locals.var_t3__blk289_dn0 = assign16650_e20587_d_n0;
        locals.var_t3__blk289_dn2 = assign16650_e20587_d_n2;
        locals.var_t3__blk289_dn4 = assign16650_e20587_d_n4;
        locals.var_t3__blk289_dn5 = assign16650_e20587_d_n5;
        locals.var_t3__blk289_dn6 = assign16650_e20587_d_n6;
        locals.var_t3__blk289_dn8 = assign16650_e20587_d_n8;
        locals.var_t3__blk289_dn10 = assign16650_e20587_d_n10;
        locals.var_t3__blk289_dn11 = assign16650_e20587_d_n11;
        locals.var_t3__blk289_dn12 = assign16650_e20587_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let assign16660_e20590: f64 = if locals.var_t3__blk289 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign16660_e20590;
        locals.var_guard308_rv = 0.0;

        let (assign16670_e20599, assign16670_e20599_d_n0, assign16670_e20599_d_n2, assign16670_e20599_d_n4, assign16670_e20599_d_n5, assign16670_e20599_d_n6, assign16670_e20599_d_n8, assign16670_e20599_d_n10, assign16670_e20599_d_n11, assign16670_e20599_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) && (locals.var_guard308 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign16670_e20599;
        locals.var_t3__blk289_dn0 = assign16670_e20599_d_n0;
        locals.var_t3__blk289_dn2 = assign16670_e20599_d_n2;
        locals.var_t3__blk289_dn4 = assign16670_e20599_d_n4;
        locals.var_t3__blk289_dn5 = assign16670_e20599_d_n5;
        locals.var_t3__blk289_dn6 = assign16670_e20599_d_n6;
        locals.var_t3__blk289_dn8 = assign16670_e20599_d_n8;
        locals.var_t3__blk289_dn10 = assign16670_e20599_d_n10;
        locals.var_t3__blk289_dn11 = assign16670_e20599_d_n11;
        locals.var_t3__blk289_dn12 = assign16670_e20599_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let (assign16680_e20608, assign16680_e20608_d_n0, assign16680_e20608_d_n2, assign16680_e20608_d_n4, assign16680_e20608_d_n5, assign16680_e20608_d_n6, assign16680_e20608_d_n8, assign16680_e20608_d_n10, assign16680_e20608_d_n11, assign16680_e20608_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) && (locals.var_guard308 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk290, locals.var_t4__blk290_dn0, locals.var_t4__blk290_dn2, locals.var_t4__blk290_dn4, locals.var_t4__blk290_dn5, locals.var_t4__blk290_dn6, locals.var_t4__blk290_dn8, locals.var_t4__blk290_dn10, locals.var_t4__blk290_dn11, locals.var_t4__blk290_dn12,)
    }
};
        locals.var_t4__blk290 = assign16680_e20608;
        locals.var_t4__blk290_dn0 = assign16680_e20608_d_n0;
        locals.var_t4__blk290_dn2 = assign16680_e20608_d_n2;
        locals.var_t4__blk290_dn4 = assign16680_e20608_d_n4;
        locals.var_t4__blk290_dn5 = assign16680_e20608_d_n5;
        locals.var_t4__blk290_dn6 = assign16680_e20608_d_n6;
        locals.var_t4__blk290_dn8 = assign16680_e20608_d_n8;
        locals.var_t4__blk290_dn10 = assign16680_e20608_d_n10;
        locals.var_t4__blk290_dn11 = assign16680_e20608_d_n11;
        locals.var_t4__blk290_dn12 = assign16680_e20608_d_n12;
        locals.var_t4__blk290_rv = 0.0;

        let (assign16690_e20619, assign16690_e20619_d_n0, assign16690_e20619_d_n2, assign16690_e20619_d_n4, assign16690_e20619_d_n5, assign16690_e20619_d_n6, assign16690_e20619_d_n8, assign16690_e20619_d_n10, assign16690_e20619_d_n11, assign16690_e20619_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16690_e20615: f64 = (locals.var_t3__blk289 - p.p216);
        let assign16690_e20617: f64 = (assign16690_e20615 / 0.1);
        (assign16690_e20617, (locals.var_t3__blk289_dn0 / 0.1), (locals.var_t3__blk289_dn2 / 0.1), (locals.var_t3__blk289_dn4 / 0.1), (locals.var_t3__blk289_dn5 / 0.1), (locals.var_t3__blk289_dn6 / 0.1), (locals.var_t3__blk289_dn8 / 0.1), (locals.var_t3__blk289_dn10 / 0.1), (locals.var_t3__blk289_dn11 / 0.1), (locals.var_t3__blk289_dn12 / 0.1),)
    } else {
        (locals.var_tx__blk285, locals.var_tx__blk285_dn0, locals.var_tx__blk285_dn2, locals.var_tx__blk285_dn4, locals.var_tx__blk285_dn5, locals.var_tx__blk285_dn6, locals.var_tx__blk285_dn8, locals.var_tx__blk285_dn10, locals.var_tx__blk285_dn11, locals.var_tx__blk285_dn12,)
    }
};
        locals.var_tx__blk285 = assign16690_e20619;
        locals.var_tx__blk285_dn0 = assign16690_e20619_d_n0;
        locals.var_tx__blk285_dn2 = assign16690_e20619_d_n2;
        locals.var_tx__blk285_dn4 = assign16690_e20619_d_n4;
        locals.var_tx__blk285_dn5 = assign16690_e20619_d_n5;
        locals.var_tx__blk285_dn6 = assign16690_e20619_d_n6;
        locals.var_tx__blk285_dn8 = assign16690_e20619_d_n8;
        locals.var_tx__blk285_dn10 = assign16690_e20619_d_n10;
        locals.var_tx__blk285_dn11 = assign16690_e20619_d_n11;
        locals.var_tx__blk285_dn12 = assign16690_e20619_d_n12;
        locals.var_tx__blk285_rv = 0.0;

        let (assign16700_e20634, assign16700_e20634_d_n0, assign16700_e20634_d_n2, assign16700_e20634_d_n4, assign16700_e20634_d_n5, assign16700_e20634_d_n6, assign16700_e20634_d_n8, assign16700_e20634_d_n10, assign16700_e20634_d_n11, assign16700_e20634_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16700_e20629: f64 = (locals.var_tx__blk285 * locals.var_tx__blk285);
        let assign16700_e20630: f64 = (1.0 + assign16700_e20629);
        let assign16700_e20631: f64 = (1.0 / assign16700_e20630);
        let assign16700_e20632: f64 = (1.0 - assign16700_e20631);
        (assign16700_e20632, (-(-(((locals.var_tx__blk285_dn0 * locals.var_tx__blk285) + (locals.var_tx__blk285 * locals.var_tx__blk285_dn0)) / (assign16700_e20630 * assign16700_e20630)))), (-(-(((locals.var_tx__blk285_dn2 * locals.var_tx__blk285) + (locals.var_tx__blk285 * locals.var_tx__blk285_dn2)) / (assign16700_e20630 * assign16700_e20630)))), (-(-(((locals.var_tx__blk285_dn4 * locals.var_tx__blk285) + (locals.var_tx__blk285 * locals.var_tx__blk285_dn4)) / (assign16700_e20630 * assign16700_e20630)))), (-(-(((locals.var_tx__blk285_dn5 * locals.var_tx__blk285) + (locals.var_tx__blk285 * locals.var_tx__blk285_dn5)) / (assign16700_e20630 * assign16700_e20630)))), (-(-(((locals.var_tx__blk285_dn6 * locals.var_tx__blk285) + (locals.var_tx__blk285 * locals.var_tx__blk285_dn6)) / (assign16700_e20630 * assign16700_e20630)))), (-(-(((locals.var_tx__blk285_dn8 * locals.var_tx__blk285) + (locals.var_tx__blk285 * locals.var_tx__blk285_dn8)) / (assign16700_e20630 * assign16700_e20630)))), (-(-(((locals.var_tx__blk285_dn10 * locals.var_tx__blk285) + (locals.var_tx__blk285 * locals.var_tx__blk285_dn10)) / (assign16700_e20630 * assign16700_e20630)))), (-(-(((locals.var_tx__blk285_dn11 * locals.var_tx__blk285) + (locals.var_tx__blk285 * locals.var_tx__blk285_dn11)) / (assign16700_e20630 * assign16700_e20630)))), (-(-(((locals.var_tx__blk285_dn12 * locals.var_tx__blk285) + (locals.var_tx__blk285 * locals.var_tx__blk285_dn12)) / (assign16700_e20630 * assign16700_e20630)))),)
    } else {
        (locals.var_t1__blk287, locals.var_t1__blk287_dn0, locals.var_t1__blk287_dn2, locals.var_t1__blk287_dn4, locals.var_t1__blk287_dn5, locals.var_t1__blk287_dn6, locals.var_t1__blk287_dn8, locals.var_t1__blk287_dn10, locals.var_t1__blk287_dn11, locals.var_t1__blk287_dn12,)
    }
};
        locals.var_t1__blk287 = assign16700_e20634;
        locals.var_t1__blk287_dn0 = assign16700_e20634_d_n0;
        locals.var_t1__blk287_dn2 = assign16700_e20634_d_n2;
        locals.var_t1__blk287_dn4 = assign16700_e20634_d_n4;
        locals.var_t1__blk287_dn5 = assign16700_e20634_d_n5;
        locals.var_t1__blk287_dn6 = assign16700_e20634_d_n6;
        locals.var_t1__blk287_dn8 = assign16700_e20634_d_n8;
        locals.var_t1__blk287_dn10 = assign16700_e20634_d_n10;
        locals.var_t1__blk287_dn11 = assign16700_e20634_d_n11;
        locals.var_t1__blk287_dn12 = assign16700_e20634_d_n12;
        locals.var_t1__blk287_rv = 0.0;

        let (assign16710_e20643, assign16710_e20643_d_n0, assign16710_e20643_d_n2, assign16710_e20643_d_n4, assign16710_e20643_d_n5, assign16710_e20643_d_n6, assign16710_e20643_d_n8, assign16710_e20643_d_n10, assign16710_e20643_d_n11, assign16710_e20643_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16710_e20641: f64 = (locals.var_etun * locals.var_t1__blk287);
        (assign16710_e20641, ((locals.var_etun_dn0 * locals.var_t1__blk287) + (locals.var_etun * locals.var_t1__blk287_dn0)), ((locals.var_etun_dn2 * locals.var_t1__blk287) + (locals.var_etun * locals.var_t1__blk287_dn2)), ((locals.var_etun_dn4 * locals.var_t1__blk287) + (locals.var_etun * locals.var_t1__blk287_dn4)), ((locals.var_etun_dn5 * locals.var_t1__blk287) + (locals.var_etun * locals.var_t1__blk287_dn5)), ((locals.var_etun_dn6 * locals.var_t1__blk287) + (locals.var_etun * locals.var_t1__blk287_dn6)), ((locals.var_etun_dn8 * locals.var_t1__blk287) + (locals.var_etun * locals.var_t1__blk287_dn8)), ((locals.var_etun_dn10 * locals.var_t1__blk287) + (locals.var_etun * locals.var_t1__blk287_dn10)), ((locals.var_etun_dn11 * locals.var_t1__blk287) + (locals.var_etun * locals.var_t1__blk287_dn11)), ((locals.var_etun_dn12 * locals.var_t1__blk287) + (locals.var_etun * locals.var_t1__blk287_dn12)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign16710_e20643;
        locals.var_etun_dn0 = assign16710_e20643_d_n0;
        locals.var_etun_dn2 = assign16710_e20643_d_n2;
        locals.var_etun_dn4 = assign16710_e20643_d_n4;
        locals.var_etun_dn5 = assign16710_e20643_d_n5;
        locals.var_etun_dn6 = assign16710_e20643_d_n6;
        locals.var_etun_dn8 = assign16710_e20643_d_n8;
        locals.var_etun_dn10 = assign16710_e20643_d_n10;
        locals.var_etun_dn11 = assign16710_e20643_d_n11;
        locals.var_etun_dn12 = assign16710_e20643_d_n12;
        locals.var_etun_rv = 0.0;

        let (assign16720_e20652, assign16720_e20652_d_n0, assign16720_e20652_d_n2, assign16720_e20652_d_n4, assign16720_e20652_d_n5, assign16720_e20652_d_n6, assign16720_e20652_d_n8, assign16720_e20652_d_n10, assign16720_e20652_d_n11, assign16720_e20652_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16720_e20650: f64 = (locals.var_cgs_leff__blk300 * locals.var_cgs_weff_nf__blk301);
        (assign16720_e20650, ((locals.var_cgs_leff__blk300_dn0 * locals.var_cgs_weff_nf__blk301) + (locals.var_cgs_leff__blk300 * locals.var_cgs_weff_nf__blk301_dn0)), ((locals.var_cgs_leff__blk300_dn2 * locals.var_cgs_weff_nf__blk301) + (locals.var_cgs_leff__blk300 * locals.var_cgs_weff_nf__blk301_dn2)), ((locals.var_cgs_leff__blk300_dn4 * locals.var_cgs_weff_nf__blk301) + (locals.var_cgs_leff__blk300 * locals.var_cgs_weff_nf__blk301_dn4)), ((locals.var_cgs_leff__blk300_dn5 * locals.var_cgs_weff_nf__blk301) + (locals.var_cgs_leff__blk300 * locals.var_cgs_weff_nf__blk301_dn5)), ((locals.var_cgs_leff__blk300_dn6 * locals.var_cgs_weff_nf__blk301) + (locals.var_cgs_leff__blk300 * locals.var_cgs_weff_nf__blk301_dn6)), ((locals.var_cgs_leff__blk300_dn8 * locals.var_cgs_weff_nf__blk301) + (locals.var_cgs_leff__blk300 * locals.var_cgs_weff_nf__blk301_dn8)), ((locals.var_cgs_leff__blk300_dn10 * locals.var_cgs_weff_nf__blk301) + (locals.var_cgs_leff__blk300 * locals.var_cgs_weff_nf__blk301_dn10)), ((locals.var_cgs_leff__blk300_dn11 * locals.var_cgs_weff_nf__blk301) + (locals.var_cgs_leff__blk300 * locals.var_cgs_weff_nf__blk301_dn11)), ((locals.var_cgs_leff__blk300_dn12 * locals.var_cgs_weff_nf__blk301) + (locals.var_cgs_leff__blk300 * locals.var_cgs_weff_nf__blk301_dn12)),)
    } else {
        (locals.var_t0__blk286, locals.var_t0__blk286_dn0, locals.var_t0__blk286_dn2, locals.var_t0__blk286_dn4, locals.var_t0__blk286_dn5, locals.var_t0__blk286_dn6, locals.var_t0__blk286_dn8, locals.var_t0__blk286_dn10, locals.var_t0__blk286_dn11, locals.var_t0__blk286_dn12,)
    }
};
        locals.var_t0__blk286 = assign16720_e20652;
        locals.var_t0__blk286_dn0 = assign16720_e20652_d_n0;
        locals.var_t0__blk286_dn2 = assign16720_e20652_d_n2;
        locals.var_t0__blk286_dn4 = assign16720_e20652_d_n4;
        locals.var_t0__blk286_dn5 = assign16720_e20652_d_n5;
        locals.var_t0__blk286_dn6 = assign16720_e20652_d_n6;
        locals.var_t0__blk286_dn8 = assign16720_e20652_d_n8;
        locals.var_t0__blk286_dn10 = assign16720_e20652_d_n10;
        locals.var_t0__blk286_dn11 = assign16720_e20652_d_n11;
        locals.var_t0__blk286_dn12 = assign16720_e20652_d_n12;
        locals.var_t0__blk286_rv = 0.0;

        let (assign16730_e20663, assign16730_e20663_d_n0, assign16730_e20663_d_n2, assign16730_e20663_d_n4, assign16730_e20663_d_n5, assign16730_e20663_d_n6, assign16730_e20663_d_n8, assign16730_e20663_d_n10, assign16730_e20663_d_n11, assign16730_e20663_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16730_e20660: f64 = (p.p209 + locals.var_t0__blk286);
        let assign16730_e20661: f64 = (p.p209 / assign16730_e20660);
        (assign16730_e20661, (-((p.p209 * locals.var_t0__blk286_dn0) / (assign16730_e20660 * assign16730_e20660))), (-((p.p209 * locals.var_t0__blk286_dn2) / (assign16730_e20660 * assign16730_e20660))), (-((p.p209 * locals.var_t0__blk286_dn4) / (assign16730_e20660 * assign16730_e20660))), (-((p.p209 * locals.var_t0__blk286_dn5) / (assign16730_e20660 * assign16730_e20660))), (-((p.p209 * locals.var_t0__blk286_dn6) / (assign16730_e20660 * assign16730_e20660))), (-((p.p209 * locals.var_t0__blk286_dn8) / (assign16730_e20660 * assign16730_e20660))), (-((p.p209 * locals.var_t0__blk286_dn10) / (assign16730_e20660 * assign16730_e20660))), (-((p.p209 * locals.var_t0__blk286_dn11) / (assign16730_e20660 * assign16730_e20660))), (-((p.p209 * locals.var_t0__blk286_dn12) / (assign16730_e20660 * assign16730_e20660))),)
    } else {
        (locals.var_t7__blk293, locals.var_t7__blk293_dn0, locals.var_t7__blk293_dn2, locals.var_t7__blk293_dn4, locals.var_t7__blk293_dn5, locals.var_t7__blk293_dn6, locals.var_t7__blk293_dn8, locals.var_t7__blk293_dn10, locals.var_t7__blk293_dn11, locals.var_t7__blk293_dn12,)
    }
};
        locals.var_t7__blk293 = assign16730_e20663;
        locals.var_t7__blk293_dn0 = assign16730_e20663_d_n0;
        locals.var_t7__blk293_dn2 = assign16730_e20663_d_n2;
        locals.var_t7__blk293_dn4 = assign16730_e20663_d_n4;
        locals.var_t7__blk293_dn5 = assign16730_e20663_d_n5;
        locals.var_t7__blk293_dn6 = assign16730_e20663_d_n6;
        locals.var_t7__blk293_dn8 = assign16730_e20663_d_n8;
        locals.var_t7__blk293_dn10 = assign16730_e20663_d_n10;
        locals.var_t7__blk293_dn11 = assign16730_e20663_d_n11;
        locals.var_t7__blk293_dn12 = assign16730_e20663_d_n12;
        locals.var_t7__blk293_rv = 0.0;

        let (assign16760_e20694, assign16760_e20694_d_n0, assign16760_e20694_d_n2, assign16760_e20694_d_n4, assign16760_e20694_d_n5, assign16760_e20694_d_n6, assign16760_e20694_d_n8, assign16760_e20694_d_n10, assign16760_e20694_d_n11, assign16760_e20694_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16760_e20689: f64 = (locals.var_etun * locals.var_etun);
        let assign16760_e20691: f64 = (assign16760_e20689 + 1e-50);
        let assign16760_e20692: f64 = (1.0 / assign16760_e20691);
        (assign16760_e20692, (-(((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (assign16760_e20691 * assign16760_e20691))), (-(((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (assign16760_e20691 * assign16760_e20691))), (-(((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4)) / (assign16760_e20691 * assign16760_e20691))), (-(((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5)) / (assign16760_e20691 * assign16760_e20691))), (-(((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (assign16760_e20691 * assign16760_e20691))), (-(((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8)) / (assign16760_e20691 * assign16760_e20691))), (-(((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (assign16760_e20691 * assign16760_e20691))), (-(((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (assign16760_e20691 * assign16760_e20691))), (-(((locals.var_etun_dn12 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn12)) / (assign16760_e20691 * assign16760_e20691))),)
    } else {
        (locals.var_t4__blk290, locals.var_t4__blk290_dn0, locals.var_t4__blk290_dn2, locals.var_t4__blk290_dn4, locals.var_t4__blk290_dn5, locals.var_t4__blk290_dn6, locals.var_t4__blk290_dn8, locals.var_t4__blk290_dn10, locals.var_t4__blk290_dn11, locals.var_t4__blk290_dn12,)
    }
};
        locals.var_t4__blk290 = assign16760_e20694;
        locals.var_t4__blk290_dn0 = assign16760_e20694_d_n0;
        locals.var_t4__blk290_dn2 = assign16760_e20694_d_n2;
        locals.var_t4__blk290_dn4 = assign16760_e20694_d_n4;
        locals.var_t4__blk290_dn5 = assign16760_e20694_d_n5;
        locals.var_t4__blk290_dn6 = assign16760_e20694_d_n6;
        locals.var_t4__blk290_dn8 = assign16760_e20694_d_n8;
        locals.var_t4__blk290_dn10 = assign16760_e20694_d_n10;
        locals.var_t4__blk290_dn11 = assign16760_e20694_d_n11;
        locals.var_t4__blk290_dn12 = assign16760_e20694_d_n12;
        locals.var_t4__blk290_rv = 0.0;

        let (assign16770_e20706, assign16770_e20706_d_n0, assign16770_e20706_d_n2, assign16770_e20706_d_n4, assign16770_e20706_d_n5, assign16770_e20706_d_n6, assign16770_e20706_d_n8, assign16770_e20706_d_n10, assign16770_e20706_d_n11, assign16770_e20706_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) {
        let assign16770_e20700: f64 = (-p.p204);
        let assign16770_e20702: f64 = (assign16770_e20700 * locals.var_egp32);
        let assign16770_e20704: f64 = (assign16770_e20702 * locals.var_t4__blk290);
        (assign16770_e20704, (((assign16770_e20700 * locals.var_egp32_dn0) * locals.var_t4__blk290) + (assign16770_e20702 * locals.var_t4__blk290_dn0)), (((assign16770_e20700 * locals.var_egp32_dn2) * locals.var_t4__blk290) + (assign16770_e20702 * locals.var_t4__blk290_dn2)), (((assign16770_e20700 * locals.var_egp32_dn4) * locals.var_t4__blk290) + (assign16770_e20702 * locals.var_t4__blk290_dn4)), (((assign16770_e20700 * locals.var_egp32_dn5) * locals.var_t4__blk290) + (assign16770_e20702 * locals.var_t4__blk290_dn5)), (((assign16770_e20700 * locals.var_egp32_dn6) * locals.var_t4__blk290) + (assign16770_e20702 * locals.var_t4__blk290_dn6)), (((assign16770_e20700 * locals.var_egp32_dn8) * locals.var_t4__blk290) + (assign16770_e20702 * locals.var_t4__blk290_dn8)), (((assign16770_e20700 * locals.var_egp32_dn10) * locals.var_t4__blk290) + (assign16770_e20702 * locals.var_t4__blk290_dn10)), (((assign16770_e20700 * locals.var_egp32_dn11) * locals.var_t4__blk290) + (assign16770_e20702 * locals.var_t4__blk290_dn11)), (((assign16770_e20700 * locals.var_egp32_dn12) * locals.var_t4__blk290) + (assign16770_e20702 * locals.var_t4__blk290_dn12)),)
    } else {
        (locals.var_t1__blk287, locals.var_t1__blk287_dn0, locals.var_t1__blk287_dn2, locals.var_t1__blk287_dn4, locals.var_t1__blk287_dn5, locals.var_t1__blk287_dn6, locals.var_t1__blk287_dn8, locals.var_t1__blk287_dn10, locals.var_t1__blk287_dn11, locals.var_t1__blk287_dn12,)
    }
};
        locals.var_t1__blk287 = assign16770_e20706;
        locals.var_t1__blk287_dn0 = assign16770_e20706_d_n0;
        locals.var_t1__blk287_dn2 = assign16770_e20706_d_n2;
        locals.var_t1__blk287_dn4 = assign16770_e20706_d_n4;
        locals.var_t1__blk287_dn5 = assign16770_e20706_d_n5;
        locals.var_t1__blk287_dn6 = assign16770_e20706_d_n6;
        locals.var_t1__blk287_dn8 = assign16770_e20706_d_n8;
        locals.var_t1__blk287_dn10 = assign16770_e20706_d_n10;
        locals.var_t1__blk287_dn11 = assign16770_e20706_d_n11;
        locals.var_t1__blk287_dn12 = assign16770_e20706_d_n12;
        locals.var_t1__blk287_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign16780_e20709: f64 = (-34.0);
        let assign16780_e20710: f64 = if locals.var_t1__blk287 < assign16780_e20709 { 1.0 } else { 0.0 };
        locals.var_guard309 = assign16780_e20710;
        locals.var_guard309_rv = 0.0;

        let (assign16800_e20735, assign16800_e20735_d_n0, assign16800_e20735_d_n2, assign16800_e20735_d_n4, assign16800_e20735_d_n5, assign16800_e20735_d_n6, assign16800_e20735_d_n8, assign16800_e20735_d_n10, assign16800_e20735_d_n11, assign16800_e20735_d_n12,) = {
    if (((locals.var_guard305 == 0.0) && (locals.var_guard306 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign16800_e20729: f64 = (p.p203 / locals.var_egp12);
        let assign16800_e20731: f64 = (assign16800_e20729 * 1.6021918e-19);
        let assign16800_e20733: f64 = (assign16800_e20731 * locals.var_t0__blk286);
        (assign16800_e20733, ((((-((p.p203 * locals.var_egp12_dn0) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk286) + (assign16800_e20731 * locals.var_t0__blk286_dn0)), ((((-((p.p203 * locals.var_egp12_dn2) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk286) + (assign16800_e20731 * locals.var_t0__blk286_dn2)), ((((-((p.p203 * locals.var_egp12_dn4) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk286) + (assign16800_e20731 * locals.var_t0__blk286_dn4)), ((((-((p.p203 * locals.var_egp12_dn5) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk286) + (assign16800_e20731 * locals.var_t0__blk286_dn5)), ((((-((p.p203 * locals.var_egp12_dn6) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk286) + (assign16800_e20731 * locals.var_t0__blk286_dn6)), ((((-((p.p203 * locals.var_egp12_dn8) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk286) + (assign16800_e20731 * locals.var_t0__blk286_dn8)), ((((-((p.p203 * locals.var_egp12_dn10) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk286) + (assign16800_e20731 * locals.var_t0__blk286_dn10)), ((((-((p.p203 * locals.var_egp12_dn11) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk286) + (assign16800_e20731 * locals.var_t0__blk286_dn11)), ((((-((p.p203 * locals.var_egp12_dn12) / (locals.var_egp12 * locals.var_egp12))) * 1.6021918e-19) * locals.var_t0__blk286) + (assign16800_e20731 * locals.var_t0__blk286_dn12)),)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign16800_e20735;
        locals.var_t3__blk289_dn0 = assign16800_e20735_d_n0;
        locals.var_t3__blk289_dn2 = assign16800_e20735_d_n2;
        locals.var_t3__blk289_dn4 = assign16800_e20735_d_n4;
        locals.var_t3__blk289_dn5 = assign16800_e20735_d_n5;
        locals.var_t3__blk289_dn6 = assign16800_e20735_d_n6;
        locals.var_t3__blk289_dn8 = assign16800_e20735_d_n8;
        locals.var_t3__blk289_dn10 = assign16800_e20735_d_n10;
        locals.var_t3__blk289_dn11 = assign16800_e20735_d_n11;
        locals.var_t3__blk289_dn12 = assign16800_e20735_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let (assign16850_e20804, assign16850_e20804_d_n0, assign16850_e20804_d_n2, assign16850_e20804_d_n4, assign16850_e20804_d_n5, assign16850_e20804_d_n6, assign16850_e20804_d_n8, assign16850_e20804_d_n10, assign16850_e20804_d_n11, assign16850_e20804_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign16850_e20798: f64 = (-p.p211);
        let assign16850_e20800: f64 = (assign16850_e20798 * locals.var_vgs);
        let assign16850_e20802: f64 = (assign16850_e20800 + p.p212);
        (assign16850_e20802, 0.0, 0.0, 0.0, (assign16850_e20798 * locals.var_vgs_dn5), 0.0, 0.0, 0.0, (assign16850_e20798 * locals.var_vgs_dn11), (assign16850_e20798 * locals.var_vgs_dn12),)
    } else {
        (locals.var_t0__blk286, locals.var_t0__blk286_dn0, locals.var_t0__blk286_dn2, locals.var_t0__blk286_dn4, locals.var_t0__blk286_dn5, locals.var_t0__blk286_dn6, locals.var_t0__blk286_dn8, locals.var_t0__blk286_dn10, locals.var_t0__blk286_dn11, locals.var_t0__blk286_dn12,)
    }
};
        locals.var_t0__blk286 = assign16850_e20804;
        locals.var_t0__blk286_dn0 = assign16850_e20804_d_n0;
        locals.var_t0__blk286_dn2 = assign16850_e20804_d_n2;
        locals.var_t0__blk286_dn4 = assign16850_e20804_d_n4;
        locals.var_t0__blk286_dn5 = assign16850_e20804_d_n5;
        locals.var_t0__blk286_dn6 = assign16850_e20804_d_n6;
        locals.var_t0__blk286_dn8 = assign16850_e20804_d_n8;
        locals.var_t0__blk286_dn10 = assign16850_e20804_d_n10;
        locals.var_t0__blk286_dn11 = assign16850_e20804_d_n11;
        locals.var_t0__blk286_dn12 = assign16850_e20804_d_n12;
        locals.var_t0__blk286_rv = 0.0;

        let (assign16860_e20812, assign16860_e20812_d_n0, assign16860_e20812_d_n2, assign16860_e20812_d_n4, assign16860_e20812_d_n5, assign16860_e20812_d_n6, assign16860_e20812_d_n8, assign16860_e20812_d_n10, assign16860_e20812_d_n11, assign16860_e20812_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign16860_e20809: f64 = (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286);
        let assign16860_e20810: f64 = (assign16860_e20809).exp();
        (assign16860_e20810, (assign16860_e20810 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn0)), (assign16860_e20810 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn2)), (assign16860_e20810 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn4)), (assign16860_e20810 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn5)), (assign16860_e20810 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn6)), (assign16860_e20810 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn8)), (assign16860_e20810 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn10)), (assign16860_e20810 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn11)), (assign16860_e20810 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn12)),)
    } else {
        (locals.var_t2__blk288, locals.var_t2__blk288_dn0, locals.var_t2__blk288_dn2, locals.var_t2__blk288_dn4, locals.var_t2__blk288_dn5, locals.var_t2__blk288_dn6, locals.var_t2__blk288_dn8, locals.var_t2__blk288_dn10, locals.var_t2__blk288_dn11, locals.var_t2__blk288_dn12,)
    }
};
        locals.var_t2__blk288 = assign16860_e20812;
        locals.var_t2__blk288_dn0 = assign16860_e20812_d_n0;
        locals.var_t2__blk288_dn2 = assign16860_e20812_d_n2;
        locals.var_t2__blk288_dn4 = assign16860_e20812_d_n4;
        locals.var_t2__blk288_dn5 = assign16860_e20812_d_n5;
        locals.var_t2__blk288_dn6 = assign16860_e20812_d_n6;
        locals.var_t2__blk288_dn8 = assign16860_e20812_d_n8;
        locals.var_t2__blk288_dn10 = assign16860_e20812_d_n10;
        locals.var_t2__blk288_dn11 = assign16860_e20812_d_n11;
        locals.var_t2__blk288_dn12 = assign16860_e20812_d_n12;
        locals.var_t2__blk288_rv = 0.0;

        let (assign16870_e20819, assign16870_e20819_d_n0, assign16870_e20819_d_n2, assign16870_e20819_d_n4, assign16870_e20819_d_n5, assign16870_e20819_d_n6, assign16870_e20819_d_n8, assign16870_e20819_d_n10, assign16870_e20819_d_n11, assign16870_e20819_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign16870_e20817: f64 = (p.p260 * locals.var_vgs);
        (assign16870_e20817, 0.0, 0.0, 0.0, (p.p260 * locals.var_vgs_dn5), 0.0, 0.0, 0.0, (p.p260 * locals.var_vgs_dn11), (p.p260 * locals.var_vgs_dn12),)
    } else {
        (locals.var_t1__blk287, locals.var_t1__blk287_dn0, locals.var_t1__blk287_dn2, locals.var_t1__blk287_dn4, locals.var_t1__blk287_dn5, locals.var_t1__blk287_dn6, locals.var_t1__blk287_dn8, locals.var_t1__blk287_dn10, locals.var_t1__blk287_dn11, locals.var_t1__blk287_dn12,)
    }
};
        locals.var_t1__blk287 = assign16870_e20819;
        locals.var_t1__blk287_dn0 = assign16870_e20819_d_n0;
        locals.var_t1__blk287_dn2 = assign16870_e20819_d_n2;
        locals.var_t1__blk287_dn4 = assign16870_e20819_d_n4;
        locals.var_t1__blk287_dn5 = assign16870_e20819_d_n5;
        locals.var_t1__blk287_dn6 = assign16870_e20819_d_n6;
        locals.var_t1__blk287_dn8 = assign16870_e20819_d_n8;
        locals.var_t1__blk287_dn10 = assign16870_e20819_d_n10;
        locals.var_t1__blk287_dn11 = assign16870_e20819_d_n11;
        locals.var_t1__blk287_dn12 = assign16870_e20819_d_n12;
        locals.var_t1__blk287_rv = 0.0;

        let (assign16880_e20828, assign16880_e20828_d_n0, assign16880_e20828_d_n2, assign16880_e20828_d_n4, assign16880_e20828_d_n5, assign16880_e20828_d_n6, assign16880_e20828_d_n8, assign16880_e20828_d_n10, assign16880_e20828_d_n11, assign16880_e20828_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_cgs_tfox0__blk298;
        let assign16880_e20824: f64 = (1.0 * __rspice_inv_cse_0);
        let assign16880_e20826: f64 = (assign16880_e20824 * __rspice_inv_cse_0);
        (assign16880_e20826, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk286, locals.var_t0__blk286_dn0, locals.var_t0__blk286_dn2, locals.var_t0__blk286_dn4, locals.var_t0__blk286_dn5, locals.var_t0__blk286_dn6, locals.var_t0__blk286_dn8, locals.var_t0__blk286_dn10, locals.var_t0__blk286_dn11, locals.var_t0__blk286_dn12,)
    }
};
        locals.var_t0__blk286 = assign16880_e20828;
        locals.var_t0__blk286_dn0 = assign16880_e20828_d_n0;
        locals.var_t0__blk286_dn2 = assign16880_e20828_d_n2;
        locals.var_t0__blk286_dn4 = assign16880_e20828_d_n4;
        locals.var_t0__blk286_dn5 = assign16880_e20828_d_n5;
        locals.var_t0__blk286_dn6 = assign16880_e20828_d_n6;
        locals.var_t0__blk286_dn8 = assign16880_e20828_d_n8;
        locals.var_t0__blk286_dn10 = assign16880_e20828_d_n10;
        locals.var_t0__blk286_dn11 = assign16880_e20828_d_n11;
        locals.var_t0__blk286_dn12 = assign16880_e20828_d_n12;
        locals.var_t0__blk286_rv = 0.0;

        let (assign16890_e20837, assign16890_e20837_d_n0, assign16890_e20837_d_n2, assign16890_e20837_d_n4, assign16890_e20837_d_n5, assign16890_e20837_d_n6, assign16890_e20837_d_n8, assign16890_e20837_d_n10, assign16890_e20837_d_n11, assign16890_e20837_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign16890_e20833: f64 = (locals.var_t1__blk287 * locals.var_t1__blk287);
        let assign16890_e20835: f64 = (assign16890_e20833 * locals.var_t0__blk286);
        (assign16890_e20835, ((((locals.var_t1__blk287_dn0 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn0)) * locals.var_t0__blk286) + (assign16890_e20833 * locals.var_t0__blk286_dn0)), ((((locals.var_t1__blk287_dn2 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn2)) * locals.var_t0__blk286) + (assign16890_e20833 * locals.var_t0__blk286_dn2)), ((((locals.var_t1__blk287_dn4 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn4)) * locals.var_t0__blk286) + (assign16890_e20833 * locals.var_t0__blk286_dn4)), ((((locals.var_t1__blk287_dn5 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn5)) * locals.var_t0__blk286) + (assign16890_e20833 * locals.var_t0__blk286_dn5)), ((((locals.var_t1__blk287_dn6 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn6)) * locals.var_t0__blk286) + (assign16890_e20833 * locals.var_t0__blk286_dn6)), ((((locals.var_t1__blk287_dn8 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn8)) * locals.var_t0__blk286) + (assign16890_e20833 * locals.var_t0__blk286_dn8)), ((((locals.var_t1__blk287_dn10 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn10)) * locals.var_t0__blk286) + (assign16890_e20833 * locals.var_t0__blk286_dn10)), ((((locals.var_t1__blk287_dn11 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn11)) * locals.var_t0__blk286) + (assign16890_e20833 * locals.var_t0__blk286_dn11)), ((((locals.var_t1__blk287_dn12 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn12)) * locals.var_t0__blk286) + (assign16890_e20833 * locals.var_t0__blk286_dn12)),)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign16890_e20837;
        locals.var_t3__blk289_dn0 = assign16890_e20837_d_n0;
        locals.var_t3__blk289_dn2 = assign16890_e20837_d_n2;
        locals.var_t3__blk289_dn4 = assign16890_e20837_d_n4;
        locals.var_t3__blk289_dn5 = assign16890_e20837_d_n5;
        locals.var_t3__blk289_dn6 = assign16890_e20837_d_n6;
        locals.var_t3__blk289_dn8 = assign16890_e20837_d_n8;
        locals.var_t3__blk289_dn10 = assign16890_e20837_d_n10;
        locals.var_t3__blk289_dn11 = assign16890_e20837_d_n11;
        locals.var_t3__blk289_dn12 = assign16890_e20837_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let (assign16900_e20850, assign16900_e20850_d_n0, assign16900_e20850_d_n2, assign16900_e20850_d_n4, assign16900_e20850_d_n5, assign16900_e20850_d_n6, assign16900_e20850_d_n8, assign16900_e20850_d_n10, assign16900_e20850_d_n11, assign16900_e20850_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign16900_e20842: f64 = (p.p210 / 1000000.0);
        let assign16900_e20844: f64 = (assign16900_e20842 * locals.var_cgs_weff_nf__blk301);
        let assign16900_e20847: f64 = (locals.var_lg).powf(p.p259);
        let assign16900_e20848: f64 = (assign16900_e20844 * assign16900_e20847);
        (assign16900_e20848, ((assign16900_e20842 * locals.var_cgs_weff_nf__blk301_dn0) * assign16900_e20847), ((assign16900_e20842 * locals.var_cgs_weff_nf__blk301_dn2) * assign16900_e20847), ((assign16900_e20842 * locals.var_cgs_weff_nf__blk301_dn4) * assign16900_e20847), ((assign16900_e20842 * locals.var_cgs_weff_nf__blk301_dn5) * assign16900_e20847), ((assign16900_e20842 * locals.var_cgs_weff_nf__blk301_dn6) * assign16900_e20847), ((assign16900_e20842 * locals.var_cgs_weff_nf__blk301_dn8) * assign16900_e20847), ((assign16900_e20842 * locals.var_cgs_weff_nf__blk301_dn10) * assign16900_e20847), ((assign16900_e20842 * locals.var_cgs_weff_nf__blk301_dn11) * assign16900_e20847), ((assign16900_e20842 * locals.var_cgs_weff_nf__blk301_dn12) * assign16900_e20847),)
    } else {
        (locals.var_t4__blk290, locals.var_t4__blk290_dn0, locals.var_t4__blk290_dn2, locals.var_t4__blk290_dn4, locals.var_t4__blk290_dn5, locals.var_t4__blk290_dn6, locals.var_t4__blk290_dn8, locals.var_t4__blk290_dn10, locals.var_t4__blk290_dn11, locals.var_t4__blk290_dn12,)
    }
};
        locals.var_t4__blk290 = assign16900_e20850;
        locals.var_t4__blk290_dn0 = assign16900_e20850_d_n0;
        locals.var_t4__blk290_dn2 = assign16900_e20850_d_n2;
        locals.var_t4__blk290_dn4 = assign16900_e20850_d_n4;
        locals.var_t4__blk290_dn5 = assign16900_e20850_d_n5;
        locals.var_t4__blk290_dn6 = assign16900_e20850_d_n6;
        locals.var_t4__blk290_dn8 = assign16900_e20850_d_n8;
        locals.var_t4__blk290_dn10 = assign16900_e20850_d_n10;
        locals.var_t4__blk290_dn11 = assign16900_e20850_d_n11;
        locals.var_t4__blk290_dn12 = assign16900_e20850_d_n12;
        locals.var_t4__blk290_rv = 0.0;

        let (assign16940_e20879, assign16940_e20879_d_n0, assign16940_e20879_d_n2, assign16940_e20879_d_n4, assign16940_e20879_d_n5, assign16940_e20879_d_n6, assign16940_e20879_d_n8, assign16940_e20879_d_n10, assign16940_e20879_d_n11, assign16940_e20879_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign16940_e20877: f64 = (locals.var_vgs - locals.var_vds);
        (assign16940_e20877, (-locals.var_vds_dn0), (-locals.var_vds_dn2), (-locals.var_vds_dn4), (locals.var_vgs_dn5 - locals.var_vds_dn5), (-locals.var_vds_dn6), (-locals.var_vds_dn8), (-locals.var_vds_dn10), (locals.var_vgs_dn11 - locals.var_vds_dn11), (locals.var_vgs_dn12 - locals.var_vds_dn12),)
    } else {
        (locals.var_t5__blk291, locals.var_t5__blk291_dn0, locals.var_t5__blk291_dn2, locals.var_t5__blk291_dn4, locals.var_t5__blk291_dn5, locals.var_t5__blk291_dn6, locals.var_t5__blk291_dn8, locals.var_t5__blk291_dn10, locals.var_t5__blk291_dn11, locals.var_t5__blk291_dn12,)
    }
};
        locals.var_t5__blk291 = assign16940_e20879;
        locals.var_t5__blk291_dn0 = assign16940_e20879_d_n0;
        locals.var_t5__blk291_dn2 = assign16940_e20879_d_n2;
        locals.var_t5__blk291_dn4 = assign16940_e20879_d_n4;
        locals.var_t5__blk291_dn5 = assign16940_e20879_d_n5;
        locals.var_t5__blk291_dn6 = assign16940_e20879_d_n6;
        locals.var_t5__blk291_dn8 = assign16940_e20879_d_n8;
        locals.var_t5__blk291_dn10 = assign16940_e20879_d_n10;
        locals.var_t5__blk291_dn11 = assign16940_e20879_d_n11;
        locals.var_t5__blk291_dn12 = assign16940_e20879_d_n12;
        locals.var_t5__blk291_rv = 0.0;

        let (assign16950_e20889, assign16950_e20889_d_n0, assign16950_e20889_d_n2, assign16950_e20889_d_n4, assign16950_e20889_d_n5, assign16950_e20889_d_n6, assign16950_e20889_d_n8, assign16950_e20889_d_n10, assign16950_e20889_d_n11, assign16950_e20889_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign16950_e20883: f64 = (-p.p211);
        let assign16950_e20885: f64 = (assign16950_e20883 * locals.var_t5__blk291);
        let assign16950_e20887: f64 = (assign16950_e20885 + p.p212);
        (assign16950_e20887, (assign16950_e20883 * locals.var_t5__blk291_dn0), (assign16950_e20883 * locals.var_t5__blk291_dn2), (assign16950_e20883 * locals.var_t5__blk291_dn4), (assign16950_e20883 * locals.var_t5__blk291_dn5), (assign16950_e20883 * locals.var_t5__blk291_dn6), (assign16950_e20883 * locals.var_t5__blk291_dn8), (assign16950_e20883 * locals.var_t5__blk291_dn10), (assign16950_e20883 * locals.var_t5__blk291_dn11), (assign16950_e20883 * locals.var_t5__blk291_dn12),)
    } else {
        (locals.var_t0__blk286, locals.var_t0__blk286_dn0, locals.var_t0__blk286_dn2, locals.var_t0__blk286_dn4, locals.var_t0__blk286_dn5, locals.var_t0__blk286_dn6, locals.var_t0__blk286_dn8, locals.var_t0__blk286_dn10, locals.var_t0__blk286_dn11, locals.var_t0__blk286_dn12,)
    }
};
        locals.var_t0__blk286 = assign16950_e20889;
        locals.var_t0__blk286_dn0 = assign16950_e20889_d_n0;
        locals.var_t0__blk286_dn2 = assign16950_e20889_d_n2;
        locals.var_t0__blk286_dn4 = assign16950_e20889_d_n4;
        locals.var_t0__blk286_dn5 = assign16950_e20889_d_n5;
        locals.var_t0__blk286_dn6 = assign16950_e20889_d_n6;
        locals.var_t0__blk286_dn8 = assign16950_e20889_d_n8;
        locals.var_t0__blk286_dn10 = assign16950_e20889_d_n10;
        locals.var_t0__blk286_dn11 = assign16950_e20889_d_n11;
        locals.var_t0__blk286_dn12 = assign16950_e20889_d_n12;
        locals.var_t0__blk286_rv = 0.0;

        let (assign16960_e20897, assign16960_e20897_d_n0, assign16960_e20897_d_n2, assign16960_e20897_d_n4, assign16960_e20897_d_n5, assign16960_e20897_d_n6, assign16960_e20897_d_n8, assign16960_e20897_d_n10, assign16960_e20897_d_n11, assign16960_e20897_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign16960_e20894: f64 = (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286);
        let assign16960_e20895: f64 = (assign16960_e20894).exp();
        (assign16960_e20895, (assign16960_e20895 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn0)), (assign16960_e20895 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn2)), (assign16960_e20895 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn4)), (assign16960_e20895 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn5)), (assign16960_e20895 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn6)), (assign16960_e20895 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn8)), (assign16960_e20895 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn10)), (assign16960_e20895 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn11)), (assign16960_e20895 * (locals.var_cgs_tfox0__blk298 * locals.var_t0__blk286_dn12)),)
    } else {
        (locals.var_t2__blk288, locals.var_t2__blk288_dn0, locals.var_t2__blk288_dn2, locals.var_t2__blk288_dn4, locals.var_t2__blk288_dn5, locals.var_t2__blk288_dn6, locals.var_t2__blk288_dn8, locals.var_t2__blk288_dn10, locals.var_t2__blk288_dn11, locals.var_t2__blk288_dn12,)
    }
};
        locals.var_t2__blk288 = assign16960_e20897;
        locals.var_t2__blk288_dn0 = assign16960_e20897_d_n0;
        locals.var_t2__blk288_dn2 = assign16960_e20897_d_n2;
        locals.var_t2__blk288_dn4 = assign16960_e20897_d_n4;
        locals.var_t2__blk288_dn5 = assign16960_e20897_d_n5;
        locals.var_t2__blk288_dn6 = assign16960_e20897_d_n6;
        locals.var_t2__blk288_dn8 = assign16960_e20897_d_n8;
        locals.var_t2__blk288_dn10 = assign16960_e20897_d_n10;
        locals.var_t2__blk288_dn11 = assign16960_e20897_d_n11;
        locals.var_t2__blk288_dn12 = assign16960_e20897_d_n12;
        locals.var_t2__blk288_rv = 0.0;

        let (assign16970_e20904, assign16970_e20904_d_n0, assign16970_e20904_d_n2, assign16970_e20904_d_n4, assign16970_e20904_d_n5, assign16970_e20904_d_n6, assign16970_e20904_d_n8, assign16970_e20904_d_n10, assign16970_e20904_d_n11, assign16970_e20904_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign16970_e20902: f64 = (p.p260 * locals.var_t5__blk291);
        (assign16970_e20902, (p.p260 * locals.var_t5__blk291_dn0), (p.p260 * locals.var_t5__blk291_dn2), (p.p260 * locals.var_t5__blk291_dn4), (p.p260 * locals.var_t5__blk291_dn5), (p.p260 * locals.var_t5__blk291_dn6), (p.p260 * locals.var_t5__blk291_dn8), (p.p260 * locals.var_t5__blk291_dn10), (p.p260 * locals.var_t5__blk291_dn11), (p.p260 * locals.var_t5__blk291_dn12),)
    } else {
        (locals.var_t1__blk287, locals.var_t1__blk287_dn0, locals.var_t1__blk287_dn2, locals.var_t1__blk287_dn4, locals.var_t1__blk287_dn5, locals.var_t1__blk287_dn6, locals.var_t1__blk287_dn8, locals.var_t1__blk287_dn10, locals.var_t1__blk287_dn11, locals.var_t1__blk287_dn12,)
    }
};
        locals.var_t1__blk287 = assign16970_e20904;
        locals.var_t1__blk287_dn0 = assign16970_e20904_d_n0;
        locals.var_t1__blk287_dn2 = assign16970_e20904_d_n2;
        locals.var_t1__blk287_dn4 = assign16970_e20904_d_n4;
        locals.var_t1__blk287_dn5 = assign16970_e20904_d_n5;
        locals.var_t1__blk287_dn6 = assign16970_e20904_d_n6;
        locals.var_t1__blk287_dn8 = assign16970_e20904_d_n8;
        locals.var_t1__blk287_dn10 = assign16970_e20904_d_n10;
        locals.var_t1__blk287_dn11 = assign16970_e20904_d_n11;
        locals.var_t1__blk287_dn12 = assign16970_e20904_d_n12;
        locals.var_t1__blk287_rv = 0.0;

        let (assign16980_e20913, assign16980_e20913_d_n0, assign16980_e20913_d_n2, assign16980_e20913_d_n4, assign16980_e20913_d_n5, assign16980_e20913_d_n6, assign16980_e20913_d_n8, assign16980_e20913_d_n10, assign16980_e20913_d_n11, assign16980_e20913_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_cgs_tfox0__blk298;
        let assign16980_e20909: f64 = (1.0 * __rspice_inv_cse_1);
        let assign16980_e20911: f64 = (assign16980_e20909 * __rspice_inv_cse_1);
        (assign16980_e20911, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk286, locals.var_t0__blk286_dn0, locals.var_t0__blk286_dn2, locals.var_t0__blk286_dn4, locals.var_t0__blk286_dn5, locals.var_t0__blk286_dn6, locals.var_t0__blk286_dn8, locals.var_t0__blk286_dn10, locals.var_t0__blk286_dn11, locals.var_t0__blk286_dn12,)
    }
};
        locals.var_t0__blk286 = assign16980_e20913;
        locals.var_t0__blk286_dn0 = assign16980_e20913_d_n0;
        locals.var_t0__blk286_dn2 = assign16980_e20913_d_n2;
        locals.var_t0__blk286_dn4 = assign16980_e20913_d_n4;
        locals.var_t0__blk286_dn5 = assign16980_e20913_d_n5;
        locals.var_t0__blk286_dn6 = assign16980_e20913_d_n6;
        locals.var_t0__blk286_dn8 = assign16980_e20913_d_n8;
        locals.var_t0__blk286_dn10 = assign16980_e20913_d_n10;
        locals.var_t0__blk286_dn11 = assign16980_e20913_d_n11;
        locals.var_t0__blk286_dn12 = assign16980_e20913_d_n12;
        locals.var_t0__blk286_rv = 0.0;

        let (assign16990_e20922, assign16990_e20922_d_n0, assign16990_e20922_d_n2, assign16990_e20922_d_n4, assign16990_e20922_d_n5, assign16990_e20922_d_n6, assign16990_e20922_d_n8, assign16990_e20922_d_n10, assign16990_e20922_d_n11, assign16990_e20922_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign16990_e20918: f64 = (locals.var_t1__blk287 * locals.var_t1__blk287);
        let assign16990_e20920: f64 = (assign16990_e20918 * locals.var_t0__blk286);
        (assign16990_e20920, ((((locals.var_t1__blk287_dn0 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn0)) * locals.var_t0__blk286) + (assign16990_e20918 * locals.var_t0__blk286_dn0)), ((((locals.var_t1__blk287_dn2 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn2)) * locals.var_t0__blk286) + (assign16990_e20918 * locals.var_t0__blk286_dn2)), ((((locals.var_t1__blk287_dn4 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn4)) * locals.var_t0__blk286) + (assign16990_e20918 * locals.var_t0__blk286_dn4)), ((((locals.var_t1__blk287_dn5 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn5)) * locals.var_t0__blk286) + (assign16990_e20918 * locals.var_t0__blk286_dn5)), ((((locals.var_t1__blk287_dn6 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn6)) * locals.var_t0__blk286) + (assign16990_e20918 * locals.var_t0__blk286_dn6)), ((((locals.var_t1__blk287_dn8 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn8)) * locals.var_t0__blk286) + (assign16990_e20918 * locals.var_t0__blk286_dn8)), ((((locals.var_t1__blk287_dn10 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn10)) * locals.var_t0__blk286) + (assign16990_e20918 * locals.var_t0__blk286_dn10)), ((((locals.var_t1__blk287_dn11 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn11)) * locals.var_t0__blk286) + (assign16990_e20918 * locals.var_t0__blk286_dn11)), ((((locals.var_t1__blk287_dn12 * locals.var_t1__blk287) + (locals.var_t1__blk287 * locals.var_t1__blk287_dn12)) * locals.var_t0__blk286) + (assign16990_e20918 * locals.var_t0__blk286_dn12)),)
    } else {
        (locals.var_t3__blk289, locals.var_t3__blk289_dn0, locals.var_t3__blk289_dn2, locals.var_t3__blk289_dn4, locals.var_t3__blk289_dn5, locals.var_t3__blk289_dn6, locals.var_t3__blk289_dn8, locals.var_t3__blk289_dn10, locals.var_t3__blk289_dn11, locals.var_t3__blk289_dn12,)
    }
};
        locals.var_t3__blk289 = assign16990_e20922;
        locals.var_t3__blk289_dn0 = assign16990_e20922_d_n0;
        locals.var_t3__blk289_dn2 = assign16990_e20922_d_n2;
        locals.var_t3__blk289_dn4 = assign16990_e20922_d_n4;
        locals.var_t3__blk289_dn5 = assign16990_e20922_d_n5;
        locals.var_t3__blk289_dn6 = assign16990_e20922_d_n6;
        locals.var_t3__blk289_dn8 = assign16990_e20922_d_n8;
        locals.var_t3__blk289_dn10 = assign16990_e20922_d_n10;
        locals.var_t3__blk289_dn11 = assign16990_e20922_d_n11;
        locals.var_t3__blk289_dn12 = assign16990_e20922_d_n12;
        locals.var_t3__blk289_rv = 0.0;

        let (assign17000_e20935, assign17000_e20935_d_n0, assign17000_e20935_d_n2, assign17000_e20935_d_n4, assign17000_e20935_d_n5, assign17000_e20935_d_n6, assign17000_e20935_d_n8, assign17000_e20935_d_n10, assign17000_e20935_d_n11, assign17000_e20935_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign17000_e20927: f64 = (p.p210 / 1000000.0);
        let assign17000_e20929: f64 = (assign17000_e20927 * locals.var_cgs_weff_nf__blk301);
        let assign17000_e20932: f64 = (locals.var_lg).powf(p.p259);
        let assign17000_e20933: f64 = (assign17000_e20929 * assign17000_e20932);
        (assign17000_e20933, ((assign17000_e20927 * locals.var_cgs_weff_nf__blk301_dn0) * assign17000_e20932), ((assign17000_e20927 * locals.var_cgs_weff_nf__blk301_dn2) * assign17000_e20932), ((assign17000_e20927 * locals.var_cgs_weff_nf__blk301_dn4) * assign17000_e20932), ((assign17000_e20927 * locals.var_cgs_weff_nf__blk301_dn5) * assign17000_e20932), ((assign17000_e20927 * locals.var_cgs_weff_nf__blk301_dn6) * assign17000_e20932), ((assign17000_e20927 * locals.var_cgs_weff_nf__blk301_dn8) * assign17000_e20932), ((assign17000_e20927 * locals.var_cgs_weff_nf__blk301_dn10) * assign17000_e20932), ((assign17000_e20927 * locals.var_cgs_weff_nf__blk301_dn11) * assign17000_e20932), ((assign17000_e20927 * locals.var_cgs_weff_nf__blk301_dn12) * assign17000_e20932),)
    } else {
        (locals.var_t4__blk290, locals.var_t4__blk290_dn0, locals.var_t4__blk290_dn2, locals.var_t4__blk290_dn4, locals.var_t4__blk290_dn5, locals.var_t4__blk290_dn6, locals.var_t4__blk290_dn8, locals.var_t4__blk290_dn10, locals.var_t4__blk290_dn11, locals.var_t4__blk290_dn12,)
    }
};
        locals.var_t4__blk290 = assign17000_e20935;
        locals.var_t4__blk290_dn0 = assign17000_e20935_d_n0;
        locals.var_t4__blk290_dn2 = assign17000_e20935_d_n2;
        locals.var_t4__blk290_dn4 = assign17000_e20935_d_n4;
        locals.var_t4__blk290_dn5 = assign17000_e20935_d_n5;
        locals.var_t4__blk290_dn6 = assign17000_e20935_d_n6;
        locals.var_t4__blk290_dn8 = assign17000_e20935_d_n8;
        locals.var_t4__blk290_dn10 = assign17000_e20935_d_n10;
        locals.var_t4__blk290_dn11 = assign17000_e20935_d_n11;
        locals.var_t4__blk290_dn12 = assign17000_e20935_d_n12;
        locals.var_t4__blk290_rv = 0.0;

        let (assign17040_e20973, assign17040_e20973_d_n0, assign17040_e20973_d_n2, assign17040_e20973_d_n4, assign17040_e20973_d_n5, assign17040_e20973_d_n6, assign17040_e20973_d_n8, assign17040_e20973_d_n10, assign17040_e20973_d_n11, assign17040_e20973_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign17040_e20961: f64 = (-locals.var_vgs);
        let assign17040_e20964: f64 = (p.p261 * locals.var_vbs);
        let assign17040_e20965: f64 = (assign17040_e20961 + assign17040_e20964);
        let assign17040_e20967: f64 = (assign17040_e20965 + locals.var_vfb);
        let assign17040_e20969: f64 = (assign17040_e20967 + p.p215);
        let assign17040_e20971: f64 = (assign17040_e20969 / locals.var_cgs_tfox0__blk298);
        (assign17040_e20971, (((p.p261 * locals.var_vbs_dn0) + locals.var_vfb_dn0) / locals.var_cgs_tfox0__blk298), (((p.p261 * locals.var_vbs_dn2) + locals.var_vfb_dn2) / locals.var_cgs_tfox0__blk298), (((p.p261 * locals.var_vbs_dn4) + locals.var_vfb_dn4) / locals.var_cgs_tfox0__blk298), ((((-locals.var_vgs_dn5) + (p.p261 * locals.var_vbs_dn5)) + locals.var_vfb_dn5) / locals.var_cgs_tfox0__blk298), (((p.p261 * locals.var_vbs_dn6) + locals.var_vfb_dn6) / locals.var_cgs_tfox0__blk298), (((p.p261 * locals.var_vbs_dn8) + locals.var_vfb_dn8) / locals.var_cgs_tfox0__blk298), (((p.p261 * locals.var_vbs_dn10) + locals.var_vfb_dn10) / locals.var_cgs_tfox0__blk298), ((((-locals.var_vgs_dn11) + (p.p261 * locals.var_vbs_dn11)) + locals.var_vfb_dn11) / locals.var_cgs_tfox0__blk298), ((((-locals.var_vgs_dn12) + (p.p261 * locals.var_vbs_dn12)) + locals.var_vfb_dn12) / locals.var_cgs_tfox0__blk298),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign17040_e20973;
        locals.var_etun_dn0 = assign17040_e20973_d_n0;
        locals.var_etun_dn2 = assign17040_e20973_d_n2;
        locals.var_etun_dn4 = assign17040_e20973_d_n4;
        locals.var_etun_dn5 = assign17040_e20973_d_n5;
        locals.var_etun_dn6 = assign17040_e20973_d_n6;
        locals.var_etun_dn8 = assign17040_e20973_d_n8;
        locals.var_etun_dn10 = assign17040_e20973_d_n10;
        locals.var_etun_dn11 = assign17040_e20973_d_n11;
        locals.var_etun_dn12 = assign17040_e20973_d_n12;
        locals.var_etun_rv = 0.0;

        let (assign17050_e20987, assign17050_e20987_d_n0, assign17050_e20987_d_n2, assign17050_e20987_d_n4, assign17050_e20987_d_n5, assign17050_e20987_d_n6, assign17050_e20987_d_n8, assign17050_e20987_d_n10, assign17050_e20987_d_n11, assign17050_e20987_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign17050_e20978: f64 = (locals.var_etun * locals.var_etun);
        let assign17050_e20981: f64 = (4.0 * 0.01);
        let assign17050_e20983: f64 = (assign17050_e20981 * 0.01);
        let assign17050_e20984: f64 = (assign17050_e20978 + assign17050_e20983);
        let assign17050_e20985: f64 = (assign17050_e20984).sqrt();
        (assign17050_e20985, (((locals.var_etun_dn0 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn0)) / (2.0 * assign17050_e20985)), (((locals.var_etun_dn2 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn2)) / (2.0 * assign17050_e20985)), (((locals.var_etun_dn4 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn4)) / (2.0 * assign17050_e20985)), (((locals.var_etun_dn5 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn5)) / (2.0 * assign17050_e20985)), (((locals.var_etun_dn6 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn6)) / (2.0 * assign17050_e20985)), (((locals.var_etun_dn8 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn8)) / (2.0 * assign17050_e20985)), (((locals.var_etun_dn10 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn10)) / (2.0 * assign17050_e20985)), (((locals.var_etun_dn11 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn11)) / (2.0 * assign17050_e20985)), (((locals.var_etun_dn12 * locals.var_etun) + (locals.var_etun * locals.var_etun_dn12)) / (2.0 * assign17050_e20985)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign17050_e20987;
        locals.var_tmf2_dn0 = assign17050_e20987_d_n0;
        locals.var_tmf2_dn2 = assign17050_e20987_d_n2;
        locals.var_tmf2_dn4 = assign17050_e20987_d_n4;
        locals.var_tmf2_dn5 = assign17050_e20987_d_n5;
        locals.var_tmf2_dn6 = assign17050_e20987_d_n6;
        locals.var_tmf2_dn8 = assign17050_e20987_d_n8;
        locals.var_tmf2_dn10 = assign17050_e20987_d_n10;
        locals.var_tmf2_dn11 = assign17050_e20987_d_n11;
        locals.var_tmf2_dn12 = assign17050_e20987_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign17060_e20998, assign17060_e20998_d_n0, assign17060_e20998_d_n2, assign17060_e20998_d_n4, assign17060_e20998_d_n5, assign17060_e20998_d_n6, assign17060_e20998_d_n8, assign17060_e20998_d_n10, assign17060_e20998_d_n11, assign17060_e20998_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign17060_e20994: f64 = (locals.var_etun / locals.var_tmf2);
        let assign17060_e20995: f64 = (1.0 + assign17060_e20994);
        let assign17060_e20996: f64 = (0.5 * assign17060_e20995);
        (assign17060_e20996, (0.5 * (((locals.var_etun_dn0 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn2 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn4 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn5 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn6 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn8 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn10 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn11 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_etun_dn12 * locals.var_tmf2) - (locals.var_etun * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t5__blk291, locals.var_t5__blk291_dn0, locals.var_t5__blk291_dn2, locals.var_t5__blk291_dn4, locals.var_t5__blk291_dn5, locals.var_t5__blk291_dn6, locals.var_t5__blk291_dn8, locals.var_t5__blk291_dn10, locals.var_t5__blk291_dn11, locals.var_t5__blk291_dn12,)
    }
};
        locals.var_t5__blk291 = assign17060_e20998;
        locals.var_t5__blk291_dn0 = assign17060_e20998_d_n0;
        locals.var_t5__blk291_dn2 = assign17060_e20998_d_n2;
        locals.var_t5__blk291_dn4 = assign17060_e20998_d_n4;
        locals.var_t5__blk291_dn5 = assign17060_e20998_d_n5;
        locals.var_t5__blk291_dn6 = assign17060_e20998_d_n6;
        locals.var_t5__blk291_dn8 = assign17060_e20998_d_n8;
        locals.var_t5__blk291_dn10 = assign17060_e20998_d_n10;
        locals.var_t5__blk291_dn11 = assign17060_e20998_d_n11;
        locals.var_t5__blk291_dn12 = assign17060_e20998_d_n12;
        locals.var_t5__blk291_rv = 0.0;

        let (assign17070_e21011, assign17070_e21011_d_n0, assign17070_e21011_d_n2, assign17070_e21011_d_n4, assign17070_e21011_d_n5, assign17070_e21011_d_n6, assign17070_e21011_d_n8, assign17070_e21011_d_n10, assign17070_e21011_d_n11, assign17070_e21011_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign17070_e21004: f64 = (locals.var_etun + locals.var_tmf2);
        let assign17070_e21005: f64 = (0.5 * assign17070_e21004);
        let assign17070_e21008: f64 = (1e-10 * 0.01);
        let assign17070_e21009: f64 = (assign17070_e21005 + assign17070_e21008);
        (assign17070_e21009, (0.5 * (locals.var_etun_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_etun_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_etun_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_etun_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_etun_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_etun_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_etun_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_etun_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_etun_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign17070_e21011;
        locals.var_etun_dn0 = assign17070_e21011_d_n0;
        locals.var_etun_dn2 = assign17070_e21011_d_n2;
        locals.var_etun_dn4 = assign17070_e21011_d_n4;
        locals.var_etun_dn5 = assign17070_e21011_d_n5;
        locals.var_etun_dn6 = assign17070_e21011_d_n6;
        locals.var_etun_dn8 = assign17070_e21011_d_n8;
        locals.var_etun_dn10 = assign17070_e21011_d_n10;
        locals.var_etun_dn11 = assign17070_e21011_d_n11;
        locals.var_etun_dn12 = assign17070_e21011_d_n12;
        locals.var_etun_rv = 0.0;

        let assign17080_e21014: f64 = if locals.var_etun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign17080_e21014;
        locals.var_guard312_rv = 0.0;

        let (assign17090_e21021, assign17090_e21021_d_n0, assign17090_e21021_d_n2, assign17090_e21021_d_n4, assign17090_e21021_d_n5, assign17090_e21021_d_n6, assign17090_e21021_d_n8, assign17090_e21021_d_n10, assign17090_e21021_d_n11, assign17090_e21021_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard312 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign17090_e21021;
        locals.var_etun_dn0 = assign17090_e21021_d_n0;
        locals.var_etun_dn2 = assign17090_e21021_d_n2;
        locals.var_etun_dn4 = assign17090_e21021_d_n4;
        locals.var_etun_dn5 = assign17090_e21021_d_n5;
        locals.var_etun_dn6 = assign17090_e21021_d_n6;
        locals.var_etun_dn8 = assign17090_e21021_d_n8;
        locals.var_etun_dn10 = assign17090_e21021_d_n10;
        locals.var_etun_dn11 = assign17090_e21021_d_n11;
        locals.var_etun_dn12 = assign17090_e21021_d_n12;
        locals.var_etun_rv = 0.0;

        let (assign17100_e21028, assign17100_e21028_d_n0, assign17100_e21028_d_n2, assign17100_e21028_d_n4, assign17100_e21028_d_n5, assign17100_e21028_d_n6, assign17100_e21028_d_n8, assign17100_e21028_d_n10, assign17100_e21028_d_n11, assign17100_e21028_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard312 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk291, locals.var_t5__blk291_dn0, locals.var_t5__blk291_dn2, locals.var_t5__blk291_dn4, locals.var_t5__blk291_dn5, locals.var_t5__blk291_dn6, locals.var_t5__blk291_dn8, locals.var_t5__blk291_dn10, locals.var_t5__blk291_dn11, locals.var_t5__blk291_dn12,)
    }
};
        locals.var_t5__blk291 = assign17100_e21028;
        locals.var_t5__blk291_dn0 = assign17100_e21028_d_n0;
        locals.var_t5__blk291_dn2 = assign17100_e21028_d_n2;
        locals.var_t5__blk291_dn4 = assign17100_e21028_d_n4;
        locals.var_t5__blk291_dn5 = assign17100_e21028_d_n5;
        locals.var_t5__blk291_dn6 = assign17100_e21028_d_n6;
        locals.var_t5__blk291_dn8 = assign17100_e21028_d_n8;
        locals.var_t5__blk291_dn10 = assign17100_e21028_d_n10;
        locals.var_t5__blk291_dn11 = assign17100_e21028_d_n11;
        locals.var_t5__blk291_dn12 = assign17100_e21028_d_n12;
        locals.var_t5__blk291_rv = 0.0;

        let (assign17110_e21035, assign17110_e21035_d_n0, assign17110_e21035_d_n2, assign17110_e21035_d_n4, assign17110_e21035_d_n5, assign17110_e21035_d_n6, assign17110_e21035_d_n8, assign17110_e21035_d_n10, assign17110_e21035_d_n11, assign17110_e21035_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign17110_e21033: f64 = (locals.var_etun + 1e-50);
        (assign17110_e21033, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    } else {
        (locals.var_etun, locals.var_etun_dn0, locals.var_etun_dn2, locals.var_etun_dn4, locals.var_etun_dn5, locals.var_etun_dn6, locals.var_etun_dn8, locals.var_etun_dn10, locals.var_etun_dn11, locals.var_etun_dn12,)
    }
};
        locals.var_etun = assign17110_e21035;
        locals.var_etun_dn0 = assign17110_e21035_d_n0;
        locals.var_etun_dn2 = assign17110_e21035_d_n2;
        locals.var_etun_dn4 = assign17110_e21035_d_n4;
        locals.var_etun_dn5 = assign17110_e21035_d_n5;
        locals.var_etun_dn6 = assign17110_e21035_d_n6;
        locals.var_etun_dn8 = assign17110_e21035_d_n8;
        locals.var_etun_dn10 = assign17110_e21035_d_n10;
        locals.var_etun_dn11 = assign17110_e21035_d_n11;
        locals.var_etun_dn12 = assign17110_e21035_d_n12;
        locals.var_etun_rv = 0.0;

        let (assign17120_e21045, assign17120_e21045_d_n0, assign17120_e21045_d_n2, assign17120_e21045_d_n4, assign17120_e21045_d_n5, assign17120_e21045_d_n6, assign17120_e21045_d_n8, assign17120_e21045_d_n10, assign17120_e21045_d_n11, assign17120_e21045_d_n12,) = {
    if (locals.var_guard305 == 0.0) {
        let assign17120_e21039: f64 = (-p.p214);
        let assign17120_e21042: f64 = (locals.var_etun).powf(p.p263);
        let assign17120_e21043: f64 = (assign17120_e21039 / assign17120_e21042);
        (assign17120_e21043, (-((assign17120_e21039 * if 0.0 == 0.0 && ((p.p263) as f64).is_finite() && ((p.p263) as f64).fract() == 0.0 { if p.p263 == 0.0 { 0.0 } else { (p.p263 * ((locals.var_etun).powf(p.p263 - 1.0) * locals.var_etun_dn0)) } } else { (assign17120_e21042 * (p.p263 * (locals.var_etun_dn0 / locals.var_etun))) }) / (assign17120_e21042 * assign17120_e21042))), (-((assign17120_e21039 * if 0.0 == 0.0 && ((p.p263) as f64).is_finite() && ((p.p263) as f64).fract() == 0.0 { if p.p263 == 0.0 { 0.0 } else { (p.p263 * ((locals.var_etun).powf(p.p263 - 1.0) * locals.var_etun_dn2)) } } else { (assign17120_e21042 * (p.p263 * (locals.var_etun_dn2 / locals.var_etun))) }) / (assign17120_e21042 * assign17120_e21042))), (-((assign17120_e21039 * if 0.0 == 0.0 && ((p.p263) as f64).is_finite() && ((p.p263) as f64).fract() == 0.0 { if p.p263 == 0.0 { 0.0 } else { (p.p263 * ((locals.var_etun).powf(p.p263 - 1.0) * locals.var_etun_dn4)) } } else { (assign17120_e21042 * (p.p263 * (locals.var_etun_dn4 / locals.var_etun))) }) / (assign17120_e21042 * assign17120_e21042))), (-((assign17120_e21039 * if 0.0 == 0.0 && ((p.p263) as f64).is_finite() && ((p.p263) as f64).fract() == 0.0 { if p.p263 == 0.0 { 0.0 } else { (p.p263 * ((locals.var_etun).powf(p.p263 - 1.0) * locals.var_etun_dn5)) } } else { (assign17120_e21042 * (p.p263 * (locals.var_etun_dn5 / locals.var_etun))) }) / (assign17120_e21042 * assign17120_e21042))), (-((assign17120_e21039 * if 0.0 == 0.0 && ((p.p263) as f64).is_finite() && ((p.p263) as f64).fract() == 0.0 { if p.p263 == 0.0 { 0.0 } else { (p.p263 * ((locals.var_etun).powf(p.p263 - 1.0) * locals.var_etun_dn6)) } } else { (assign17120_e21042 * (p.p263 * (locals.var_etun_dn6 / locals.var_etun))) }) / (assign17120_e21042 * assign17120_e21042))), (-((assign17120_e21039 * if 0.0 == 0.0 && ((p.p263) as f64).is_finite() && ((p.p263) as f64).fract() == 0.0 { if p.p263 == 0.0 { 0.0 } else { (p.p263 * ((locals.var_etun).powf(p.p263 - 1.0) * locals.var_etun_dn8)) } } else { (assign17120_e21042 * (p.p263 * (locals.var_etun_dn8 / locals.var_etun))) }) / (assign17120_e21042 * assign17120_e21042))), (-((assign17120_e21039 * if 0.0 == 0.0 && ((p.p263) as f64).is_finite() && ((p.p263) as f64).fract() == 0.0 { if p.p263 == 0.0 { 0.0 } else { (p.p263 * ((locals.var_etun).powf(p.p263 - 1.0) * locals.var_etun_dn10)) } } else { (assign17120_e21042 * (p.p263 * (locals.var_etun_dn10 / locals.var_etun))) }) / (assign17120_e21042 * assign17120_e21042))), (-((assign17120_e21039 * if 0.0 == 0.0 && ((p.p263) as f64).is_finite() && ((p.p263) as f64).fract() == 0.0 { if p.p263 == 0.0 { 0.0 } else { (p.p263 * ((locals.var_etun).powf(p.p263 - 1.0) * locals.var_etun_dn11)) } } else { (assign17120_e21042 * (p.p263 * (locals.var_etun_dn11 / locals.var_etun))) }) / (assign17120_e21042 * assign17120_e21042))), (-((assign17120_e21039 * if 0.0 == 0.0 && ((p.p263) as f64).is_finite() && ((p.p263) as f64).fract() == 0.0 { if p.p263 == 0.0 { 0.0 } else { (p.p263 * ((locals.var_etun).powf(p.p263 - 1.0) * locals.var_etun_dn12)) } } else { (assign17120_e21042 * (p.p263 * (locals.var_etun_dn12 / locals.var_etun))) }) / (assign17120_e21042 * assign17120_e21042))),)
    } else {
        (locals.var_t1__blk287, locals.var_t1__blk287_dn0, locals.var_t1__blk287_dn2, locals.var_t1__blk287_dn4, locals.var_t1__blk287_dn5, locals.var_t1__blk287_dn6, locals.var_t1__blk287_dn8, locals.var_t1__blk287_dn10, locals.var_t1__blk287_dn11, locals.var_t1__blk287_dn12,)
    }
};
        locals.var_t1__blk287 = assign17120_e21045;
        locals.var_t1__blk287_dn0 = assign17120_e21045_d_n0;
        locals.var_t1__blk287_dn2 = assign17120_e21045_d_n2;
        locals.var_t1__blk287_dn4 = assign17120_e21045_d_n4;
        locals.var_t1__blk287_dn5 = assign17120_e21045_d_n5;
        locals.var_t1__blk287_dn6 = assign17120_e21045_d_n6;
        locals.var_t1__blk287_dn8 = assign17120_e21045_d_n8;
        locals.var_t1__blk287_dn10 = assign17120_e21045_d_n10;
        locals.var_t1__blk287_dn11 = assign17120_e21045_d_n11;
        locals.var_t1__blk287_dn12 = assign17120_e21045_d_n12;
        locals.var_t1__blk287_rv = 0.0;

        let assign17130_e21048: f64 = (-34.0);
        let assign17130_e21049: f64 = if locals.var_t1__blk287 < assign17130_e21048 { 1.0 } else { 0.0 };
        locals.var_guard313 = assign17130_e21049;
        locals.var_guard313_rv = 0.0;

        let (assign17150_e21065, assign17150_e21065_d_n0, assign17150_e21065_d_n2, assign17150_e21065_d_n4, assign17150_e21065_d_n5, assign17150_e21065_d_n6, assign17150_e21065_d_n8, assign17150_e21065_d_n10, assign17150_e21065_d_n11, assign17150_e21065_d_n12,) = {
    if ((locals.var_guard305 == 0.0) && (locals.var_guard313 == 0.0)) {
        let assign17150_e21063: f64 = (locals.var_t1__blk287).exp();
        (assign17150_e21063, (assign17150_e21063 * locals.var_t1__blk287_dn0), (assign17150_e21063 * locals.var_t1__blk287_dn2), (assign17150_e21063 * locals.var_t1__blk287_dn4), (assign17150_e21063 * locals.var_t1__blk287_dn5), (assign17150_e21063 * locals.var_t1__blk287_dn6), (assign17150_e21063 * locals.var_t1__blk287_dn8), (assign17150_e21063 * locals.var_t1__blk287_dn10), (assign17150_e21063 * locals.var_t1__blk287_dn11), (assign17150_e21063 * locals.var_t1__blk287_dn12),)
    } else {
        (locals.var_t2__blk288, locals.var_t2__blk288_dn0, locals.var_t2__blk288_dn2, locals.var_t2__blk288_dn4, locals.var_t2__blk288_dn5, locals.var_t2__blk288_dn6, locals.var_t2__blk288_dn8, locals.var_t2__blk288_dn10, locals.var_t2__blk288_dn11, locals.var_t2__blk288_dn12,)
    }
};
        locals.var_t2__blk288 = assign17150_e21065;
        locals.var_t2__blk288_dn0 = assign17150_e21065_d_n0;
        locals.var_t2__blk288_dn2 = assign17150_e21065_d_n2;
        locals.var_t2__blk288_dn4 = assign17150_e21065_d_n4;
        locals.var_t2__blk288_dn5 = assign17150_e21065_d_n5;
        locals.var_t2__blk288_dn6 = assign17150_e21065_d_n6;
        locals.var_t2__blk288_dn8 = assign17150_e21065_d_n8;
        locals.var_t2__blk288_dn10 = assign17150_e21065_d_n10;
        locals.var_t2__blk288_dn11 = assign17150_e21065_d_n11;
        locals.var_t2__blk288_dn12 = assign17150_e21065_d_n12;
        locals.var_t2__blk288_rv = 0.0;

    }
}
